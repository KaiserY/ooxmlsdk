use super::lex::{LexToken, formula_body_start, lex_tokens};
use super::semantic::{
  ExternalReferenceSpans, SemanticSpan, SemanticToken, SemanticTokenKind, SemanticWordKind,
  semantic_token_from_lex, semantic_word_kind,
};
use super::syntax::{
  SyntaxIssue, SyntaxNode, SyntaxParse, SyntaxSpan, parse_syntax_ast, parse_syntax_ast_from_tokens,
};
use super::{LexErrorValue, LexLogicalFunction, LexOperator};
use crate::CellAddress;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FormulaBodyParse {
  pub tokens: Vec<FormulaBodyToken>,
  pub ast: Option<FormulaAst>,
  pub issues: Vec<FormulaParseIssue>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FormulaSourceParse<'a> {
  pub body_start: usize,
  pub body: &'a str,
  pub body_parse: FormulaBodyParse,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FormulaSyntaxParse {
  pub ast: Option<FormulaAst>,
  pub issues: Vec<FormulaParseIssue>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FormulaSourceSyntaxParse<'a> {
  pub body: &'a str,
  pub syntax_parse: FormulaSyntaxParse,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct FormulaBodyToken {
  pub kind: FormulaBodyTokenKind,
  pub span: SemanticSpan,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum FormulaBodyTokenKind {
  Text,
  Number(f64),
  Error(LexErrorValue),
  Operator(LexOperator),
  ArrayOpen,
  ArrayClose,
  ArgumentSeparator,
  RowSeparator,
  Function { volatile: bool },
  Boolean(bool),
  ExternalReference(ExternalReferenceSpans),
  ReferenceCandidate,
  Name,
  Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FormulaParseIssue {
  UnrecognizedCharacter(SemanticSpan),
  MissingClosingParenthesis,
  IncompleteExpression,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum FormulaAst {
  Blank,
  Text(SemanticSpan),
  Number(f64),
  Error(LexErrorValue),
  Word {
    span: SemanticSpan,
    kind: SemanticWordKind,
  },
  Unary {
    op: LexOperator,
    expr: Box<FormulaAst>,
  },
  Binary {
    op: LexOperator,
    left: Box<FormulaAst>,
    right: Box<FormulaAst>,
  },
  Function {
    name: SemanticSpan,
    args: Vec<FormulaAst>,
  },
  LogicalFunction {
    function: LexLogicalFunction,
    args: Vec<FormulaAst>,
  },
  Array(Vec<Vec<FormulaAst>>),
}

pub(crate) type FormulaNode = FormulaAst;

pub(crate) struct FormulaParser<'a> {
  source: &'a str,
}

impl<'a> FormulaParser<'a> {
  pub(crate) fn new(source: &'a str) -> Self {
    Self { source }
  }

  pub(crate) fn parse(self) -> FormulaSourceParse<'a> {
    let body_start = formula_body_start(self.source);
    self.parse_at_body(body_start)
  }

  pub(crate) fn parse_at_body(self, body_start: usize) -> FormulaSourceParse<'a> {
    let body = self.source.get(body_start..).unwrap_or(self.source);
    FormulaSourceParse {
      body_start,
      body,
      body_parse: parse_formula_body(body),
    }
  }

  pub(crate) fn parse_syntax(self) -> FormulaSourceSyntaxParse<'a> {
    let body_start = formula_body_start(self.source);
    self.parse_syntax_at_body(body_start)
  }

  pub(crate) fn parse_syntax_at_body(self, body_start: usize) -> FormulaSourceSyntaxParse<'a> {
    let body = self.source.get(body_start..).unwrap_or(self.source);
    FormulaSourceSyntaxParse {
      body,
      syntax_parse: parse_formula_syntax(body),
    }
  }
}

fn parse_formula_body(source: &str) -> FormulaBodyParse {
  let lexed = lex_tokens(source).collect::<Vec<_>>();
  parse_formula_body_from_tokens(source, &lexed)
}

fn parse_formula_body_from_tokens(source: &str, lexed: &[LexToken]) -> FormulaBodyParse {
  let mut issues = Vec::new();
  let tokens = formula_body_tokens(
    lexed
      .iter()
      .copied()
      .map(|token| semantic_token_from_lex(source, token)),
    &mut issues,
  );

  let syntax = parse_formula_syntax_from_tokens(source, lexed);
  issues.extend(syntax.issues);

  FormulaBodyParse {
    tokens,
    ast: syntax.ast,
    issues,
  }
}

fn parse_formula_syntax(source: &str) -> FormulaSyntaxParse {
  formula_syntax_from_syntax(source, parse_syntax_ast(source))
}

fn parse_formula_syntax_from_tokens(source: &str, lexed: &[LexToken]) -> FormulaSyntaxParse {
  formula_syntax_from_syntax(source, parse_syntax_ast_from_tokens(source, lexed))
}

fn formula_syntax_from_syntax(source: &str, syntax: SyntaxParse) -> FormulaSyntaxParse {
  let mut issues = Vec::new();
  for issue in syntax.issues {
    match issue {
      SyntaxIssue::MissingClosingParenthesis => {
        issues.push(FormulaParseIssue::MissingClosingParenthesis);
      }
    }
  }
  if syntax.ast.is_none() || !syntax.complete {
    issues.push(FormulaParseIssue::IncompleteExpression);
  }
  let ast = syntax
    .ast
    .as_ref()
    .and_then(|node| formula_ast_from_syntax(source, node));

  FormulaSyntaxParse { ast, issues }
}

fn formula_body_tokens(
  tokens: impl IntoIterator<Item = SemanticToken>,
  issues: &mut Vec<FormulaParseIssue>,
) -> Vec<FormulaBodyToken> {
  tokens
    .into_iter()
    .filter_map(|token| {
      let span = SemanticSpan {
        start: token.start,
        end: token.end,
      };
      let kind = match token.kind {
        SemanticTokenKind::Text => FormulaBodyTokenKind::Text,
        SemanticTokenKind::Number(value) => FormulaBodyTokenKind::Number(value),
        SemanticTokenKind::Error(value) => FormulaBodyTokenKind::Error(value),
        SemanticTokenKind::Operator(value) => FormulaBodyTokenKind::Operator(value),
        SemanticTokenKind::ArrayOpen => FormulaBodyTokenKind::ArrayOpen,
        SemanticTokenKind::ArrayClose => FormulaBodyTokenKind::ArrayClose,
        SemanticTokenKind::ArgumentSeparator => FormulaBodyTokenKind::ArgumentSeparator,
        SemanticTokenKind::RowSeparator => FormulaBodyTokenKind::RowSeparator,
        SemanticTokenKind::ParenOpen | SemanticTokenKind::ParenClose => return None,
        SemanticTokenKind::Function { volatile } => FormulaBodyTokenKind::Function { volatile },
        SemanticTokenKind::Boolean(value) => FormulaBodyTokenKind::Boolean(value),
        SemanticTokenKind::ExternalReference(value) => {
          FormulaBodyTokenKind::ExternalReference(value)
        }
        SemanticTokenKind::ReferenceCandidate => FormulaBodyTokenKind::ReferenceCandidate,
        SemanticTokenKind::Name => FormulaBodyTokenKind::Name,
        SemanticTokenKind::Unsupported => {
          issues.push(FormulaParseIssue::UnrecognizedCharacter(span));
          FormulaBodyTokenKind::Unsupported
        }
      };
      Some(FormulaBodyToken { kind, span })
    })
    .collect()
}

fn formula_ast_from_syntax(source: &str, node: &SyntaxNode) -> Option<FormulaAst> {
  match node {
    SyntaxNode::Blank => Some(FormulaAst::Blank),
    SyntaxNode::Text(span) => Some(FormulaAst::Text((*span).into())),
    SyntaxNode::Number(value) => Some(FormulaAst::Number(*value)),
    SyntaxNode::Error(value) => Some(FormulaAst::Error(*value)),
    SyntaxNode::Word(span) => {
      let span = SemanticSpan::from(*span);
      let word = source.get(span.start..span.end)?;
      Some(FormulaAst::Word {
        span,
        kind: semantic_word_kind(word),
      })
    }
    SyntaxNode::Unary { op, expr } => Some(FormulaAst::Unary {
      op: *op,
      expr: Box::new(formula_ast_from_syntax(source, expr)?),
    }),
    SyntaxNode::Binary { op, left, right } => Some(FormulaAst::Binary {
      op: *op,
      left: Box::new(formula_ast_from_syntax(source, left)?),
      right: Box::new(formula_ast_from_syntax(source, right)?),
    }),
    SyntaxNode::Function { name, args } => Some(FormulaAst::Function {
      name: (*name).into(),
      args: formula_asts_from_syntax(source, args)?,
    }),
    SyntaxNode::LogicalFunction { function, args } => Some(FormulaAst::LogicalFunction {
      function: *function,
      args: formula_asts_from_syntax(source, args)?,
    }),
    SyntaxNode::Array(rows) => Some(FormulaAst::Array(
      rows
        .iter()
        .map(|row| formula_asts_from_syntax(source, row))
        .collect::<Option<Vec<_>>>()?,
    )),
  }
}

fn formula_asts_from_syntax(source: &str, nodes: &[SyntaxNode]) -> Option<Vec<FormulaAst>> {
  nodes
    .iter()
    .map(|node| formula_ast_from_syntax(source, node))
    .collect()
}

pub(crate) fn normalize_excel_formula_text(formula: &str) -> &str {
  let formula = formula.trim();
  formula
    .get(formula_body_start(formula)..)
    .unwrap_or(formula)
}

pub(crate) fn normalize_r1c1_formula_text(formula: &str, base: CellAddress) -> String {
  let formula = normalize_excel_formula_text(formula);
  if let Some(reference) = r1c1_whole_axis_reference_to_a1(formula, base) {
    reference
  } else {
    formula.to_string()
  }
}

pub(crate) fn r1c1_whole_axis_reference_to_a1(
  reference: &str,
  base: CellAddress,
) -> Option<String> {
  let reference = normalize_excel_formula_text(reference);
  if let Some(offset) = parse_r1c1_relative(reference, 'C') {
    let column = base.column.checked_add_signed(offset)?.checked_add(1)?;
    let column = one_based_column_name(column);
    return Some(format!("{column}:{column}"));
  }
  if let Some(offset) = parse_r1c1_relative(reference, 'R') {
    let row = base.row.checked_add_signed(offset)?.checked_add(1)?;
    return Some(format!("{row}:{row}"));
  }
  None
}

pub(crate) fn normalize_open_formula_text(formula: &str) -> String {
  let formula = normalize_excel_formula_text(formula);
  let text = normalize_open_formula_decimal_commas(formula);
  let text = normalize_formula_separators(&text);
  normalize_open_formula_references(&text)
}

pub(crate) fn normalize_calc_formula_text(formula: &str) -> String {
  normalize_formula_separators(normalize_excel_formula_text(formula))
}

pub(crate) fn r1c1_reference_to_a1(reference: &str, base: CellAddress) -> Option<String> {
  let reference = normalize_excel_formula_text(reference);
  let (start, end) = reference.split_once(':').unwrap_or((reference, reference));
  let start = parse_r1c1_cell(start.trim(), base)?;
  let end = parse_r1c1_cell(end.trim(), base)?;
  let start = format!("{}{}", zero_based_column_name(start.column), start.row + 1);
  let end = format!("{}{}", zero_based_column_name(end.column), end.row + 1);
  if start == end {
    Some(start)
  } else {
    Some(format!("{start}:{end}"))
  }
}

fn normalize_formula_separators(formula: &str) -> String {
  let mut output = String::with_capacity(formula.len());
  let mut quoted = false;
  let mut chars = formula.chars().peekable();
  while let Some(ch) = chars.next() {
    match ch {
      '"' => {
        output.push(ch);
        if quoted && chars.peek() == Some(&'"') {
          output.push('"');
          chars.next();
        } else {
          quoted = !quoted;
        }
      }
      ';' if !quoted => output.push(','),
      _ => output.push(ch),
    }
  }
  output
}

fn normalize_open_formula_decimal_commas(formula: &str) -> String {
  let mut output = String::with_capacity(formula.len());
  let mut quoted = false;
  let mut chars = formula.chars().peekable();
  while let Some(ch) = chars.next() {
    match ch {
      '"' => {
        output.push(ch);
        if quoted && chars.peek() == Some(&'"') {
          output.push('"');
          chars.next();
        } else {
          quoted = !quoted;
        }
      }
      ',' if !quoted => {
        let previous = output.chars().next_back();
        let next = chars.peek().copied();
        if previous.is_some_and(|value| value.is_ascii_digit())
          && next.is_some_and(|value| value.is_ascii_digit())
        {
          output.push('.');
        } else {
          output.push(ch);
        }
      }
      _ => output.push(ch),
    }
  }
  output
}

fn normalize_open_formula_references(formula: &str) -> String {
  let mut output = String::with_capacity(formula.len());
  let mut chars = formula.chars().peekable();
  let mut quoted = false;
  while let Some(ch) = chars.next() {
    match ch {
      '"' => {
        output.push(ch);
        if quoted && chars.peek() == Some(&'"') {
          output.push('"');
          chars.next();
        } else {
          quoted = !quoted;
        }
      }
      '[' if !quoted => {
        let mut reference = String::new();
        let mut closed = false;
        for next in chars.by_ref() {
          if next == ']' {
            closed = true;
            break;
          }
          reference.push(next);
        }
        if closed {
          output.push_str(&normalize_open_formula_reference(&reference));
        } else {
          output.push('[');
          output.push_str(&reference);
        }
      }
      _ => output.push(ch),
    }
  }
  output
}

fn normalize_open_formula_reference(reference: &str) -> String {
  let mut reference = reference.trim_start_matches('.').replace(":.", ":");
  let first_part_end = reference.find(':').unwrap_or(reference.len());
  if let Some(dot) = reference[..first_part_end].rfind('.')
    && !reference[..first_part_end].contains('!')
  {
    reference.replace_range(dot..=dot, "!");
    if reference.starts_with('$') {
      reference.remove(0);
    }
  }
  reference
}

fn parse_r1c1_relative(reference: &str, axis: char) -> Option<i32> {
  let rest = reference.strip_prefix(axis)?;
  if rest.is_empty() {
    return Some(0);
  }
  let offset = rest.strip_prefix('[')?.strip_suffix(']')?;
  offset.parse::<i32>().ok()
}

fn parse_r1c1_cell(reference: &str, base: CellAddress) -> Option<CellAddress> {
  let reference = normalize_excel_formula_text(reference);
  let upper = reference.to_ascii_uppercase();
  let rest = upper.strip_prefix('R')?;
  let column_marker = rest.find('C')?;
  let (row_text, column_text) = rest.split_at(column_marker);
  let column_text = column_text.strip_prefix('C')?;
  let row = parse_r1c1_axis(row_text, base.row)?;
  let column = parse_r1c1_axis(column_text, base.column)?;
  Some(CellAddress { column, row })
}

fn parse_r1c1_axis(text: &str, base: u32) -> Option<u32> {
  if text.is_empty() {
    return Some(base);
  }
  if let Some(relative) = text
    .strip_prefix('[')
    .and_then(|text| text.strip_suffix(']'))
  {
    return base.checked_add_signed(relative.parse::<i32>().ok()?);
  }
  text.parse::<u32>().ok()?.checked_sub(1)
}

fn one_based_column_name(mut column: u32) -> String {
  let mut chars = Vec::new();
  while column > 0 {
    column -= 1;
    chars.push(char::from_u32('A' as u32 + column % 26).unwrap_or('A'));
    column /= 26;
  }
  chars.into_iter().rev().collect()
}

fn zero_based_column_name(mut column: u32) -> String {
  let mut name = Vec::new();
  loop {
    name.push((b'A' + (column % 26) as u8) as char);
    column /= 26;
    if column == 0 {
      break;
    }
    column -= 1;
  }
  name.into_iter().rev().collect()
}

impl From<SyntaxSpan> for SemanticSpan {
  fn from(span: SyntaxSpan) -> Self {
    Self {
      start: span.start,
      end: span.end,
    }
  }
}
