use std::cell::Cell;

use smallvec::SmallVec;
use winnow::Parser;
use winnow::combinator::{Infix, Postfix, Prefix, expression};
use winnow::error::{ContextError, ParserError};
use winnow::stream::{Needed, Offset, Stream, StreamIsPartial};

use super::{
  FormulaArgSpan, FormulaCellReference, FormulaDiagnosticKind, FormulaExprId,
  FormulaExternalNameReference, FormulaFunctionName, FormulaNameScope, FormulaNamedReference,
  FormulaNodeKind, FormulaNodeLabels, FormulaNodeMetadata, FormulaOperandClass, FormulaParamClass,
  FormulaProgramBuilder, FormulaRangeReference, FormulaReference, FormulaReferencePoint,
  FormulaSheetName, FormulaSheetRange, FormulaSheetReference, FormulaUnsupportedKind, SourceSpan,
  reference_flags, valid_address_flags,
};
use crate::source::{FormulaSource, FormulaSourcePosition};
use crate::{FormulaOperator, QualifiedRange, SheetId, parser};

type FormulaArgBuffer = SmallVec<[FormulaExprId; 8]>;
type ProgramParseResult<T> = winnow::Result<T, ContextError>;
type ProgramPrefix<'p> = Prefix<ProgramSyntaxParser<'p>, FormulaExprId, ContextError>;
type ProgramPostfix<'p> = Postfix<ProgramSyntaxParser<'p>, FormulaExprId, ContextError>;
type ProgramInfix<'p> = Infix<ProgramSyntaxParser<'p>, FormulaExprId, ContextError>;
type ProgramUnaryFold<'p> =
  fn(&mut ProgramSyntaxParser<'p>, FormulaExprId) -> ProgramParseResult<FormulaExprId>;
type ProgramBinaryFold<'p> = fn(
  &mut ProgramSyntaxParser<'p>,
  FormulaExprId,
  FormulaExprId,
) -> ProgramParseResult<FormulaExprId>;

pub(super) fn parse_program_root(
  builder: &mut FormulaProgramBuilder,
  source: FormulaSource<'_>,
  body_start: usize,
  body: &str,
) -> Option<FormulaExprId> {
  let mut parser = ProgramSyntaxParser::new(builder, source, body_start, body);
  let root = parser.parse_expression();
  if parser.missing_closing_parenthesis {
    parser
      .builder
      .diagnostic(None, FormulaDiagnosticKind::ParseError);
  }
  parser.diagnose_current_unsupported_token();
  if root.is_none() || !parser.tokens.is_end() {
    parser
      .builder
      .diagnostic(None, FormulaDiagnosticKind::ParseError);
  }
  root
}

pub(super) fn program_node_capacity_hint(body: &str) -> usize {
  (body.len() / 4).clamp(4, 64)
}

struct ProgramSyntaxParser<'p> {
  builder: &'p mut FormulaProgramBuilder,
  source: FormulaSource<'p>,
  body_start: usize,
  body: &'p str,
  tokens: ProgramSyntaxTokens<'p>,
  missing_closing_parenthesis: bool,
}

impl<'p> ProgramSyntaxParser<'p> {
  fn new(
    builder: &'p mut FormulaProgramBuilder,
    source: FormulaSource<'p>,
    body_start: usize,
    body: &'p str,
  ) -> Self {
    Self {
      builder,
      source,
      body_start,
      body,
      tokens: ProgramSyntaxTokens::new(body),
      missing_closing_parenthesis: false,
    }
  }

  fn parse_expression(&mut self) -> Option<FormulaExprId> {
    self.parse_expression_bp(0)
  }

  fn parse_expression_bp(&mut self, min_bp: u8) -> Option<FormulaExprId> {
    let mut left = self.parse_winnow_expression(min_bp)?;
    loop {
      let had_ws = self.tokens.ws_before_next();

      if had_ws && is_intersection_rhs_start(self.tokens.peek()) {
        let (left_bp, right_bp) = infix_binding_power(parser::LexOperator::Intersection)?;
        if left_bp < min_bp {
          break;
        }
        let before_rhs = self.tokens.checkpoint();
        let checkpoint = self.builder.checkpoint();
        if let Some(right) = self.parse_expression_bp(right_bp) {
          left = self
            .builder
            .binary(FormulaOperator::Intersection, left, right);
          continue;
        }
        self.builder.rollback(checkpoint);
        self.tokens.reset(&before_rhs);
      }

      break;
    }
    Some(left)
  }

  fn parse_winnow_expression(&mut self, min_bp: u8) -> Option<FormulaExprId> {
    expression(parse_program_operand)
      .prefix(parse_program_prefix)
      .postfix(parse_program_postfix)
      .infix(parse_program_infix)
      .current_precedence_level(i64::from(min_bp))
      .parse_next(self)
      .ok()
  }

  fn checkpoint(&self) -> ProgramParserCheckpoint {
    ProgramParserCheckpoint {
      builder: self.builder.checkpoint(),
      tokens: self.tokens.checkpoint(),
      missing_closing_parenthesis: self.missing_closing_parenthesis,
    }
  }

  fn rollback(&mut self, checkpoint: ProgramParserCheckpoint) {
    self.builder.rollback(checkpoint.builder);
    self.tokens.reset(&checkpoint.tokens);
    self.missing_closing_parenthesis = checkpoint.missing_closing_parenthesis;
  }

  fn parse_operand(&mut self) -> Option<FormulaExprId> {
    if self
      .tokens
      .consume_token_kind(parser::LexTokenKind::ParenOpen)
      .is_some()
    {
      let expr = self.parse_parenthesized_expression()?;
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ParenClose)
        .is_none()
      {
        self.missing_closing_parenthesis = true;
      }
      return Some(expr);
    }
    if let Some(token) = self.tokens.consume_token_kind(parser::LexTokenKind::Text) {
      return self.push_text_literal(token_span(token));
    }
    if self
      .tokens
      .consume_token_kind(parser::LexTokenKind::ArrayOpen)
      .is_some()
    {
      return self.parse_array();
    }
    if let Some(token) = self.tokens.peek() {
      match token.kind {
        parser::LexTokenKind::Number(value) => {
          self.tokens.advance();
          return Some(self.builder.number(value));
        }
        parser::LexTokenKind::Error(error) => {
          self.tokens.advance();
          return Some(
            self
              .builder
              .error(crate::code::formula_error_from_lex(error)),
          );
        }
        _ => {}
      }
    }
    self.parse_word_or_function()
  }

  fn parse_parenthesized_expression(&mut self) -> Option<FormulaExprId> {
    let mut left = self.parse_expression()?;
    while self
      .tokens
      .consume_token_kind(parser::LexTokenKind::ArgumentSeparator)
      .is_some()
    {
      let right = self.parse_expression_bp(infix_binding_power(parser::LexOperator::Union)?.1)?;
      left = self.builder.binary(FormulaOperator::Union, left, right);
    }
    Some(left)
  }

  fn parse_array(&mut self) -> Option<FormulaExprId> {
    let checkpoint = self.checkpoint();
    let result = self.parse_array_inner();
    if result.is_none() {
      self.rollback(checkpoint);
    }
    result
  }

  fn parse_array_inner(&mut self) -> Option<FormulaExprId> {
    let mark = self.builder.array_mark();
    let mut rows = 0usize;
    let mut cols = None::<usize>;
    let mut row_len = 0usize;
    loop {
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ArrayClose)
        .is_some()
      {
        if row_len != 0 {
          rows += 1;
          cols = finish_array_row(cols, row_len)?;
        } else if rows != 0 {
          let blank = self.builder.blank();
          self.builder.push_array_element(blank);
          rows += 1;
          cols = finish_array_row(cols, 1)?;
        }
        break;
      }
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ArgumentSeparator)
        .is_some()
      {
        let blank = self.builder.blank();
        self.builder.push_array_element(blank);
        row_len += 1;
        continue;
      }
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::RowSeparator)
        .is_some()
      {
        if row_len == 0 {
          let blank = self.builder.blank();
          self.builder.push_array_element(blank);
          row_len = 1;
        }
        rows += 1;
        cols = finish_array_row(cols, row_len)?;
        row_len = 0;
        continue;
      }

      let element = self.parse_expression()?;
      self.builder.push_array_element(element);
      row_len += 1;

      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ArgumentSeparator)
        .is_some()
      {
        continue;
      }
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::RowSeparator)
        .is_some()
      {
        rows += 1;
        cols = finish_array_row(cols, row_len)?;
        row_len = 0;
        continue;
      }
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ArrayClose)
        .is_some()
      {
        rows += 1;
        cols = finish_array_row(cols, row_len)?;
        break;
      }
      return None;
    }

    if rows > u16::MAX as usize || cols.unwrap_or_default() > u16::MAX as usize {
      return None;
    }
    let span = self
      .builder
      .finish_array_span(mark, rows as u16, cols.unwrap_or_default() as u16);
    Some(self.builder.push_value(FormulaNodeKind::Array(span)))
  }

  fn parse_word_or_function(&mut self) -> Option<FormulaExprId> {
    let checkpoint = self.checkpoint();
    let result = self.parse_word_or_function_inner();
    if result.is_none() {
      self.rollback(checkpoint);
    }
    result
  }

  fn parse_word_or_function_inner(&mut self) -> Option<FormulaExprId> {
    let token = self.tokens.peek()?;
    if token.kind != parser::LexTokenKind::Word {
      return None;
    }
    let name = token_span(token);
    self.tokens.advance();
    if self
      .tokens
      .peek()
      .is_some_and(|token| token.kind == parser::LexTokenKind::ParenOpen)
    {
      let args = self.parse_argument_list(None)?;
      let name_text = self.body.get(name.start..name.end)?;
      let name_symbol = self.builder.intern(name_text);
      let mut labels = FormulaNodeLabels::empty();
      if parser::is_volatile_function_name(name_text) {
        labels |= FormulaNodeLabels::VOLATILE;
      }
      return Some(self.builder.push(
        FormulaNodeKind::Function {
          name: FormulaFunctionName::Unknown(name_symbol),
          args,
        },
        Some(source_span(self.body_start, name)),
        FormulaNodeMetadata {
          labels,
          operand_class: FormulaOperandClass::Unknown,
          param_class: FormulaParamClass::Unknown,
        },
      ));
    }
    let word = self.body.get(name.start..name.end)?;
    self.push_word(name, parser::semantic_word_kind(word))
  }

  fn parse_argument_list(&mut self, first: Option<FormulaExprId>) -> Option<FormulaArgSpan> {
    let checkpoint = self.checkpoint();
    let result = self.parse_argument_list_inner(first);
    if result.is_none() {
      self.rollback(checkpoint);
    }
    result
  }

  fn parse_argument_list_inner(&mut self, first: Option<FormulaExprId>) -> Option<FormulaArgSpan> {
    self
      .tokens
      .consume_token_kind(parser::LexTokenKind::ParenOpen)?;
    let mut args = FormulaArgBuffer::new();
    if let Some(first) = first {
      args.push(first);
    }
    loop {
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ParenClose)
        .is_some()
      {
        break;
      }
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ArgumentSeparator)
        .is_some()
      {
        let blank = self.builder.blank();
        args.push(blank);
        if self
          .tokens
          .peek()
          .is_some_and(|token| token.kind == parser::LexTokenKind::ParenClose)
        {
          let blank = self.builder.blank();
          args.push(blank);
        }
        continue;
      }
      let arg = self.parse_expression()?;
      args.push(arg);
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ParenClose)
        .is_some()
      {
        break;
      }
      self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ArgumentSeparator)?;
      if self
        .tokens
        .peek()
        .is_some_and(|token| token.kind == parser::LexTokenKind::ParenClose)
      {
        let blank = self.builder.blank();
        args.push(blank);
      }
    }
    Some(self.builder.push_arg_span(&args))
  }

  fn push_text_literal(&mut self, span: parser::SemanticSpan) -> Option<FormulaExprId> {
    let value = match parser::formula_text_literal(self.source.text, self.body_start + span.start)?
    {
      parser::TextLiteral::Borrowed(value) => value,
      parser::TextLiteral::Owned(value) => {
        let symbol = self.builder.intern(&value);
        return Some(self.builder.push(
          FormulaNodeKind::Text(symbol),
          Some(source_span(self.body_start, span)),
          FormulaNodeMetadata::default(),
        ));
      }
    };
    let symbol = self.builder.intern(value);
    Some(self.builder.push(
      FormulaNodeKind::Text(symbol),
      Some(source_span(self.body_start, span)),
      FormulaNodeMetadata::default(),
    ))
  }

  fn push_word(
    &mut self,
    span: parser::SemanticSpan,
    kind: parser::SemanticWordKind,
  ) -> Option<FormulaExprId> {
    lower_parser_word(self.builder, self.source, self.body_start, span, kind)
  }

  fn diagnose_current_unsupported_token(&mut self) {
    let Some(token) = self.tokens.peek() else {
      return;
    };
    if token.kind == parser::LexTokenKind::Unsupported {
      self.builder.diagnostic(
        Some(source_span(self.body_start, token_span(token))),
        FormulaDiagnosticKind::Unsupported(FormulaUnsupportedKind::Token),
      );
    }
  }
}

fn parse_program_operand<'p>(
  input: &mut ProgramSyntaxParser<'p>,
) -> ProgramParseResult<FormulaExprId> {
  input.parse_operand().ok_or_else(|| program_error(input))
}

fn parse_program_prefix<'p>(
  input: &mut ProgramSyntaxParser<'p>,
) -> ProgramParseResult<ProgramPrefix<'p>> {
  let Some(token) = input.tokens.peek() else {
    return program_fail(input);
  };
  let parser::LexTokenKind::Operator(operator) = token.kind else {
    return program_fail(input);
  };
  let fold: ProgramUnaryFold<'p> = match operator {
    parser::LexOperator::Add => fold_prefix_plus as ProgramUnaryFold<'p>,
    parser::LexOperator::Subtract => fold_prefix_minus as ProgramUnaryFold<'p>,
    parser::LexOperator::ImplicitIntersection => {
      fold_prefix_implicit_intersection as ProgramUnaryFold<'p>
    }
    _ => return program_fail(input),
  };
  input.tokens.advance();
  Ok(Prefix(i64::from(prefix_binding_power()), fold))
}

fn parse_program_postfix<'p>(
  input: &mut ProgramSyntaxParser<'p>,
) -> ProgramParseResult<ProgramPostfix<'p>> {
  if let Some(token) = input.tokens.peek()
    && token.kind == parser::LexTokenKind::Operator(parser::LexOperator::Percent)
  {
    input.tokens.advance();
    return Ok(Postfix(
      i64::from(percent_postfix_binding_power()),
      fold_postfix_percent,
    ));
  }

  let Some(function) = input.tokens.consume_logical_function_call() else {
    return program_fail(input);
  };
  let fold: ProgramUnaryFold<'p> = match function {
    parser::LexLogicalFunction::And => fold_postfix_logical_and as ProgramUnaryFold<'p>,
    parser::LexLogicalFunction::Or => fold_postfix_logical_or as ProgramUnaryFold<'p>,
    parser::LexLogicalFunction::Not => fold_postfix_logical_not as ProgramUnaryFold<'p>,
  };
  Ok(Postfix(i64::from(logical_binding_power()), fold))
}

fn parse_program_infix<'p>(
  input: &mut ProgramSyntaxParser<'p>,
) -> ProgramParseResult<ProgramInfix<'p>> {
  let Some(token) = input.tokens.peek() else {
    return program_fail(input);
  };
  let parser::LexTokenKind::Operator(operator) = token.kind else {
    return program_fail(input);
  };
  let (left_bp, right_associative, fold): (u8, bool, ProgramBinaryFold<'p>) = match operator {
    parser::LexOperator::Equal => (2, false, fold_infix_equal),
    parser::LexOperator::NotEqual => (2, false, fold_infix_not_equal),
    parser::LexOperator::Less => (2, false, fold_infix_less),
    parser::LexOperator::LessOrEqual => (2, false, fold_infix_less_or_equal),
    parser::LexOperator::Greater => (2, false, fold_infix_greater),
    parser::LexOperator::GreaterOrEqual => (2, false, fold_infix_greater_or_equal),
    parser::LexOperator::Union => (4, false, fold_infix_union),
    parser::LexOperator::Intersection => (6, false, fold_infix_intersection),
    parser::LexOperator::Range => (8, false, fold_infix_range),
    parser::LexOperator::Concat => (10, false, fold_infix_concat),
    parser::LexOperator::Add => (12, false, fold_infix_add),
    parser::LexOperator::Subtract => (12, false, fold_infix_subtract),
    parser::LexOperator::Multiply => (14, false, fold_infix_multiply),
    parser::LexOperator::Divide => (14, false, fold_infix_divide),
    parser::LexOperator::Power => (16, true, fold_infix_power),
    parser::LexOperator::ImplicitIntersection | parser::LexOperator::Percent => {
      return program_fail(input);
    }
  };
  input.tokens.advance();
  if right_associative {
    Ok(Infix::Right(i64::from(left_bp), fold))
  } else {
    Ok(Infix::Left(i64::from(left_bp), fold))
  }
}

fn program_fail<T>(input: &ProgramSyntaxParser<'_>) -> ProgramParseResult<T> {
  Err(program_error(input))
}

fn program_error(input: &ProgramSyntaxParser<'_>) -> ContextError {
  ContextError::from_input(input)
}

macro_rules! fold_unary_operator {
  ($name:ident, $operator:expr) => {
    fn $name(
      input: &mut ProgramSyntaxParser<'_>,
      expr: FormulaExprId,
    ) -> ProgramParseResult<FormulaExprId> {
      Ok(input.builder.unary($operator, expr))
    }
  };
}

macro_rules! fold_binary_operator {
  ($name:ident, $operator:expr) => {
    fn $name(
      input: &mut ProgramSyntaxParser<'_>,
      left: FormulaExprId,
      right: FormulaExprId,
    ) -> ProgramParseResult<FormulaExprId> {
      Ok(input.builder.binary($operator, left, right))
    }
  };
}

fold_unary_operator!(fold_prefix_plus, FormulaOperator::UnaryPlus);
fold_unary_operator!(fold_prefix_minus, FormulaOperator::UnaryMinus);
fold_unary_operator!(
  fold_prefix_implicit_intersection,
  FormulaOperator::ImplicitIntersection
);
fold_unary_operator!(fold_postfix_percent, FormulaOperator::Percent);

fn fold_postfix_logical_and(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  fold_postfix_logical(input, left, parser::LexLogicalFunction::And)
}

fn fold_postfix_logical_or(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  fold_postfix_logical(input, left, parser::LexLogicalFunction::Or)
}

fn fold_postfix_logical_not(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  fold_postfix_logical(input, left, parser::LexLogicalFunction::Not)
}

fn fold_postfix_logical(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
  function: parser::LexLogicalFunction,
) -> ProgramParseResult<FormulaExprId> {
  let Some(args) = input.parse_argument_list(Some(left)) else {
    return program_fail(input);
  };
  let name = input.builder.intern(function.name());
  Ok(input.builder.push_value(FormulaNodeKind::Function {
    name: FormulaFunctionName::Unknown(name),
    args,
  }))
}

fold_binary_operator!(fold_infix_equal, FormulaOperator::Equal);
fold_binary_operator!(fold_infix_not_equal, FormulaOperator::NotEqual);
fold_binary_operator!(fold_infix_less, FormulaOperator::Less);
fold_binary_operator!(fold_infix_less_or_equal, FormulaOperator::LessOrEqual);
fold_binary_operator!(fold_infix_greater, FormulaOperator::Greater);
fold_binary_operator!(fold_infix_greater_or_equal, FormulaOperator::GreaterOrEqual);
fold_binary_operator!(fold_infix_union, FormulaOperator::Union);
fold_binary_operator!(fold_infix_intersection, FormulaOperator::Intersection);
fold_binary_operator!(fold_infix_range, FormulaOperator::Range);
fold_binary_operator!(fold_infix_concat, FormulaOperator::Concat);
fold_binary_operator!(fold_infix_add, FormulaOperator::Add);
fold_binary_operator!(fold_infix_subtract, FormulaOperator::Subtract);
fold_binary_operator!(fold_infix_multiply, FormulaOperator::Multiply);
fold_binary_operator!(fold_infix_divide, FormulaOperator::Divide);
fold_binary_operator!(fold_infix_power, FormulaOperator::Power);

#[derive(Clone, Debug)]
struct ProgramSyntaxCheckpoint {
  offset: usize,
  previous_end: usize,
  peeked: Option<parser::LexToken>,
}

#[derive(Clone, Debug)]
struct ProgramParserCheckpoint {
  builder: super::FormulaProgramCheckpoint,
  tokens: ProgramSyntaxCheckpoint,
  missing_closing_parenthesis: bool,
}

impl Offset for ProgramSyntaxCheckpoint {
  fn offset_from(&self, start: &Self) -> usize {
    self.offset.saturating_sub(start.offset)
  }
}

impl Offset<ProgramSyntaxCheckpoint> for ProgramSyntaxParser<'_> {
  fn offset_from(&self, start: &ProgramSyntaxCheckpoint) -> usize {
    self.tokens.position().saturating_sub(start.offset)
  }
}

impl std::fmt::Debug for ProgramSyntaxParser<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ProgramSyntaxParser")
      .field("index", &self.tokens.position())
      .field("remaining", &self.tokens.eof_offset())
      .finish()
  }
}

impl<'p> Stream for ProgramSyntaxParser<'p> {
  type Token = parser::LexToken;
  type Slice = &'p str;
  type IterOffsets = ProgramSyntaxTokenOffsets<'p>;
  type Checkpoint = ProgramSyntaxCheckpoint;

  fn iter_offsets(&self) -> Self::IterOffsets {
    self.tokens.iter_offsets()
  }

  fn eof_offset(&self) -> usize {
    self.tokens.eof_offset()
  }

  fn next_token(&mut self) -> Option<Self::Token> {
    self.tokens.advance()
  }

  fn peek_token(&self) -> Option<Self::Token> {
    self.tokens.peek()
  }

  fn offset_for<P>(&self, predicate: P) -> Option<usize>
  where
    P: Fn(Self::Token) -> bool,
  {
    self
      .tokens
      .iter_offsets()
      .find_map(|(offset, token)| predicate(token).then_some(offset))
  }

  fn offset_at(&self, tokens: usize) -> Result<usize, Needed> {
    self
      .tokens
      .byte_offset_after_tokens(tokens)
      .map(|offset| offset.saturating_sub(self.tokens.position()))
      .ok_or_else(|| Needed::new(tokens))
  }

  fn next_slice(&mut self, offset: usize) -> Self::Slice {
    self.tokens.next_slice(offset)
  }

  fn peek_slice(&self, offset: usize) -> Self::Slice {
    self.tokens.peek_slice(offset)
  }

  fn checkpoint(&self) -> Self::Checkpoint {
    self.tokens.checkpoint()
  }

  fn reset(&mut self, checkpoint: &Self::Checkpoint) {
    self.tokens.reset(checkpoint);
  }

  fn trace(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "ProgramSyntaxParser(index={}, remaining={})",
      self.tokens.position(),
      self.tokens.eof_offset()
    )
  }
}

impl StreamIsPartial for ProgramSyntaxParser<'_> {
  type PartialState = ();

  fn complete(&mut self) -> Self::PartialState {}

  fn restore_partial(&mut self, _state: Self::PartialState) {}

  fn is_partial_supported() -> bool {
    false
  }
}

struct ProgramSyntaxTokens<'p> {
  source: &'p str,
  offset: usize,
  previous_end: usize,
  peeked: Cell<Option<parser::LexToken>>,
}

impl<'p> ProgramSyntaxTokens<'p> {
  fn new(source: &'p str) -> Self {
    Self {
      source,
      offset: 0,
      previous_end: 0,
      peeked: Cell::new(None),
    }
  }

  fn is_end(&self) -> bool {
    self.peek().is_none()
  }

  fn eof_offset(&self) -> usize {
    self.source.len().saturating_sub(self.offset)
  }

  fn position(&self) -> usize {
    self.offset
  }

  fn peek(&self) -> Option<parser::LexToken> {
    if let Some(token) = self.peeked.get() {
      return Some(token);
    }
    let token = parser::lex_token_at(self.source, self.offset);
    self.peeked.set(token);
    token
  }

  fn advance(&mut self) -> Option<parser::LexToken> {
    let token = self.peek()?;
    self.offset = token.end;
    self.previous_end = token.end;
    self.peeked.set(None);
    Some(token)
  }

  fn ws_before_next(&self) -> bool {
    let token = match self.peek() {
      Some(token) => token,
      None => return false,
    };
    self
      .source
      .get(self.previous_end..token.start)
      .is_some_and(|text| !text.is_empty())
  }

  fn consume_token_kind(&mut self, kind: parser::LexTokenKind) -> Option<parser::LexToken> {
    let token = self.peek()?;
    if token.kind != kind {
      return None;
    }
    self.advance()
  }

  fn consume_logical_function_call(&mut self) -> Option<parser::LexLogicalFunction> {
    let token = self.peek()?;
    if token.kind != parser::LexTokenKind::Word {
      return None;
    }
    let word = self.source.get(token.start..token.end)?;
    let function = logical_function_name(word)?;
    let next = self.token_after(token.end)?;
    if next.kind != parser::LexTokenKind::ParenOpen {
      return None;
    }
    self.advance();
    Some(function)
  }

  fn token_after(&self, offset: usize) -> Option<parser::LexToken> {
    parser::lex_token_at(self.source, offset)
  }

  fn checkpoint(&self) -> ProgramSyntaxCheckpoint {
    ProgramSyntaxCheckpoint {
      offset: self.offset,
      previous_end: self.previous_end,
      peeked: self.peeked.get(),
    }
  }

  fn reset(&mut self, checkpoint: &ProgramSyntaxCheckpoint) {
    self.offset = checkpoint.offset.min(self.source.len());
    self.previous_end = checkpoint.previous_end.min(self.source.len());
    self.peeked.set(checkpoint.peeked);
  }

  fn iter_offsets(&self) -> ProgramSyntaxTokenOffsets<'p> {
    ProgramSyntaxTokenOffsets {
      source: self.source,
      base_offset: self.offset,
      offset: self.offset,
    }
  }

  fn byte_offset_after_tokens(&self, tokens: usize) -> Option<usize> {
    let mut offset = self.offset;
    for _ in 0..tokens {
      offset = parser::lex_token_at(self.source, offset)?.end;
    }
    Some(offset)
  }

  fn next_slice(&mut self, offset: usize) -> &'p str {
    let start = self.offset;
    let end = self.offset.saturating_add(offset).min(self.source.len());
    self.offset = end;
    self.previous_end = end;
    self.peeked.set(None);
    self.source.get(start..end).unwrap_or_default()
  }

  fn peek_slice(&self, offset: usize) -> &'p str {
    let start = self.offset;
    let end = self.offset.saturating_add(offset).min(self.source.len());
    self.source.get(start..end).unwrap_or_default()
  }
}

struct ProgramSyntaxTokenOffsets<'p> {
  source: &'p str,
  base_offset: usize,
  offset: usize,
}

impl Iterator for ProgramSyntaxTokenOffsets<'_> {
  type Item = (usize, parser::LexToken);

  fn next(&mut self) -> Option<Self::Item> {
    let token = parser::lex_token_at(self.source, self.offset)?;
    let offset = token.start.saturating_sub(self.base_offset);
    self.offset = token.end;
    Some((offset, token))
  }
}

fn logical_function_name(value: &str) -> Option<parser::LexLogicalFunction> {
  if value.eq_ignore_ascii_case("AND") {
    Some(parser::LexLogicalFunction::And)
  } else if value.eq_ignore_ascii_case("OR") {
    Some(parser::LexLogicalFunction::Or)
  } else if value.eq_ignore_ascii_case("NOT") {
    Some(parser::LexLogicalFunction::Not)
  } else {
    None
  }
}

fn logical_binding_power() -> u8 {
  1
}

fn infix_binding_power(operator: parser::LexOperator) -> Option<(u8, u8)> {
  match operator {
    parser::LexOperator::Equal
    | parser::LexOperator::NotEqual
    | parser::LexOperator::Less
    | parser::LexOperator::LessOrEqual
    | parser::LexOperator::Greater
    | parser::LexOperator::GreaterOrEqual => Some((2, 3)),
    parser::LexOperator::Union => Some((4, 5)),
    parser::LexOperator::Intersection => Some((6, 7)),
    parser::LexOperator::Range => Some((8, 9)),
    parser::LexOperator::Concat => Some((10, 11)),
    parser::LexOperator::Add | parser::LexOperator::Subtract => Some((12, 13)),
    parser::LexOperator::Multiply | parser::LexOperator::Divide => Some((14, 15)),
    parser::LexOperator::Power => Some((16, 16)),
    parser::LexOperator::ImplicitIntersection | parser::LexOperator::Percent => None,
  }
}

fn prefix_binding_power() -> u8 {
  17
}

fn percent_postfix_binding_power() -> u8 {
  18
}

fn token_span(token: parser::LexToken) -> parser::SemanticSpan {
  parser::SemanticSpan {
    start: token.start,
    end: token.end,
  }
}

fn is_intersection_rhs_start(token: Option<parser::LexToken>) -> bool {
  token.is_some_and(|token| {
    matches!(
      token.kind,
      parser::LexTokenKind::Text
        | parser::LexTokenKind::Number(_)
        | parser::LexTokenKind::Error(_)
        | parser::LexTokenKind::ArrayOpen
        | parser::LexTokenKind::ParenOpen
        | parser::LexTokenKind::Word
    )
  })
}

fn finish_array_row(cols: Option<usize>, row_len: usize) -> Option<Option<usize>> {
  match cols {
    Some(cols) if cols != row_len => None,
    Some(cols) => Some(Some(cols)),
    None => Some(Some(row_len)),
  }
}

fn lower_parser_word(
  builder: &mut FormulaProgramBuilder,
  source: FormulaSource<'_>,
  body_start: usize,
  span: parser::SemanticSpan,
  kind: parser::SemanticWordKind,
) -> Option<FormulaExprId> {
  let word = source
    .text
    .get(body_start + span.start..body_start + span.end)?;
  match kind {
    parser::SemanticWordKind::Boolean(value) => Some(builder.boolean(value)),
    parser::SemanticWordKind::ReferenceCandidate => {
      let sheet = match source.context.position {
        FormulaSourcePosition::Cell(cell) => cell.sheet,
        FormulaSourcePosition::Sheet(sheet) => sheet,
      };
      if let Some(range) = parser::parse_formula_range(sheet, word) {
        let reference = reference_from_qualified_range(builder, range);
        return Some(builder.push_reference(reference));
      }
      Some(named_reference_with_span(
        builder,
        word,
        Some(source_span(body_start, span)),
      ))
    }
    parser::SemanticWordKind::Name => Some(named_reference_with_span(
      builder,
      word,
      Some(source_span(body_start, span)),
    )),
    parser::SemanticWordKind::ExternalReference(reference) => {
      let reference = external_reference_from_spans(builder, word, reference)?;
      Some(builder.push_reference(reference))
    }
  }
}

fn named_reference_with_span(
  builder: &mut FormulaProgramBuilder,
  name: &str,
  span: Option<SourceSpan>,
) -> FormulaExprId {
  let name = builder.intern(name);
  builder.push(
    FormulaNodeKind::Reference(FormulaReference::Named(FormulaNamedReference {
      name,
      scope: FormulaNameScope::Workbook,
    })),
    span,
    FormulaNodeMetadata {
      labels: FormulaNodeLabels::RETURNS_REFERENCE
        | FormulaNodeLabels::CONTAINS_REFERENCE
        | FormulaNodeLabels::CONTAINS_NAME,
      operand_class: FormulaOperandClass::Reference,
      param_class: FormulaParamClass::Unknown,
    },
  )
}

fn source_span(body_start: usize, span: parser::SemanticSpan) -> SourceSpan {
  SourceSpan {
    start: body_start + span.start,
    end: body_start + span.end,
  }
}

fn external_reference_from_spans(
  builder: &mut FormulaProgramBuilder,
  source: &str,
  reference: parser::ExternalReferenceSpans,
) -> Option<FormulaReference> {
  let book = builder.intern(span_text(source, reference.book));
  let sheet = reference
    .sheet
    .map(|sheet| FormulaSheetName::Name(intern_external_sheet_text(builder, source, sheet)));
  let name = span_text(source, reference.name?);

  if let Some(range) = parser::parse_formula_range(SheetId::default(), name) {
    let sheet = FormulaSheetReference::External {
      book,
      sheet: sheet.map(FormulaSheetRange::Sheet),
    };
    let start_flags = valid_address_flags(range.start_flags, true);
    let end_flags = valid_address_flags(range.end_flags, true);
    let reference_flags = reference_flags(range.start_flags, range.end_flags);
    if range.range.start == range.range.end
      && !range.start_flags.whole_column
      && !range.start_flags.whole_row
      && !range.end_flags.whole_column
      && !range.end_flags.whole_row
    {
      return Some(FormulaReference::Cell(FormulaCellReference {
        target: FormulaReferencePoint {
          sheet,
          address: range.range.start,
          flags: start_flags,
        },
        flags: reference_flags,
      }));
    }
    return Some(FormulaReference::Range(FormulaRangeReference {
      start: FormulaReferencePoint {
        sheet,
        address: range.range.start,
        flags: start_flags,
      },
      end: FormulaReferencePoint {
        sheet,
        address: range.range.end,
        flags: end_flags,
      },
      flags: reference_flags,
    }));
  }

  Some(FormulaReference::ExternalName(
    FormulaExternalNameReference {
      book,
      sheet,
      name: builder.intern(name),
    },
  ))
}

fn span_text(source: &str, span: parser::SemanticSpan) -> &str {
  source.get(span.start..span.end).unwrap_or_default()
}

fn intern_external_sheet_text(
  builder: &mut FormulaProgramBuilder,
  source: &str,
  span: parser::SemanticSpan,
) -> super::FormulaSymbolId {
  let text = span_text(source, span);
  if text.contains("''") {
    builder.intern(&text.replace("''", "'"))
  } else {
    builder.intern(text)
  }
}

fn reference_from_qualified_range(
  builder: &mut FormulaProgramBuilder,
  range: QualifiedRange<'_>,
) -> FormulaReference {
  let sheet = sheet_reference_from_qualified_range(builder, &range);
  let start_flags = valid_address_flags(range.start_flags, sheet != FormulaSheetReference::Current);
  let end_flags = valid_address_flags(range.end_flags, sheet != FormulaSheetReference::Current);
  let reference_flags = reference_flags(range.start_flags, range.end_flags);
  if range.range.start == range.range.end
    && !range.start_flags.whole_column
    && !range.start_flags.whole_row
    && !range.end_flags.whole_column
    && !range.end_flags.whole_row
  {
    FormulaReference::Cell(FormulaCellReference {
      target: FormulaReferencePoint {
        sheet,
        address: range.range.start,
        flags: start_flags,
      },
      flags: reference_flags,
    })
  } else {
    FormulaReference::Range(FormulaRangeReference {
      start: FormulaReferencePoint {
        sheet,
        address: range.range.start,
        flags: start_flags,
      },
      end: FormulaReferencePoint {
        sheet,
        address: range.range.end,
        flags: end_flags,
      },
      flags: reference_flags,
    })
  }
}

fn sheet_reference_from_qualified_range(
  builder: &mut FormulaProgramBuilder,
  range: &QualifiedRange<'_>,
) -> FormulaSheetReference {
  let Some(start) = range.sheet_name.as_ref() else {
    return FormulaSheetReference::Current;
  };
  let start = FormulaSheetName::Name(builder.intern(start.0.as_ref()));
  let sheet = if let Some(end) = range.end_sheet_name.as_ref() {
    FormulaSheetRange::Range {
      start,
      end: FormulaSheetName::Name(builder.intern(end.0.as_ref())),
    }
  } else {
    FormulaSheetRange::Sheet(start)
  };
  FormulaSheetReference::Local(sheet)
}
