use super::lex::{LexLogicalFunction, LexOperator, LexToken, LexTokenKind, lex_tokens};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SyntaxSpan {
  pub start: usize,
  pub end: usize,
}

impl From<LexToken> for SyntaxSpan {
  fn from(token: LexToken) -> Self {
    Self {
      start: token.start,
      end: token.end,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SyntaxNode {
  Blank,
  Text(SyntaxSpan),
  Number(f64),
  Error(super::LexErrorValue),
  Word(SyntaxSpan),
  Unary {
    op: LexOperator,
    expr: Box<SyntaxNode>,
  },
  Binary {
    op: LexOperator,
    left: Box<SyntaxNode>,
    right: Box<SyntaxNode>,
  },
  Function {
    name: SyntaxSpan,
    args: Vec<SyntaxNode>,
  },
  LogicalFunction {
    function: LexLogicalFunction,
    args: Vec<SyntaxNode>,
  },
  Array(Vec<Vec<SyntaxNode>>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SyntaxIssue {
  MissingClosingParenthesis,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SyntaxParse {
  pub ast: Option<SyntaxNode>,
  pub complete: bool,
  pub issues: Vec<SyntaxIssue>,
}

pub(crate) fn parse_syntax_ast(source: &str) -> SyntaxParse {
  let mut parser = SyntaxParser::new(source);
  let ast = parser.parse_expression();
  SyntaxParse {
    ast,
    complete: parser.tokens.is_end(),
    issues: parser.issues,
  }
}

pub(super) fn parse_syntax_ast_from_tokens(source: &str, tokens: &[LexToken]) -> SyntaxParse {
  let mut parser = SyntaxParser::from_tokens(source, tokens);
  let ast = parser.parse_expression();
  SyntaxParse {
    ast,
    complete: parser.tokens.is_end(),
    issues: parser.issues,
  }
}

struct SyntaxParser<'a> {
  source: &'a str,
  tokens: SyntaxTokens,
  issues: Vec<SyntaxIssue>,
}

impl<'a> SyntaxParser<'a> {
  fn new(source: &'a str) -> Self {
    Self {
      source,
      tokens: SyntaxTokens::new(source),
      issues: Vec::new(),
    }
  }

  fn from_tokens(source: &'a str, tokens: &[LexToken]) -> Self {
    Self {
      source,
      tokens: SyntaxTokens::from_tokens(source, tokens),
      issues: Vec::new(),
    }
  }

  fn parse_expression(&mut self) -> Option<SyntaxNode> {
    self.parse_expression_bp(0)
  }

  fn parse_expression_bp(&mut self, min_bp: u8) -> Option<SyntaxNode> {
    let mut left = self.parse_prefix()?;
    loop {
      let had_ws = self.tokens.ws_before_next();

      let start = self.tokens.position();
      if let Some(function) = self.tokens.consume_logical_function_call(self.source) {
        let left_bp = logical_binding_power();
        if left_bp < min_bp {
          self.tokens.set_position(start);
          break;
        }
        let mut args = self.parse_argument_list()?;
        args.insert(0, left);
        left = SyntaxNode::LogicalFunction { function, args };
        continue;
      }

      if let Some(token) = self.tokens.peek()
        && let LexTokenKind::Operator(op) = token.kind
      {
        if let Some(left_bp) = postfix_binding_power(op) {
          if left_bp < min_bp {
            break;
          }
          self.tokens.advance();
          left = SyntaxNode::Unary {
            op,
            expr: Box::new(left),
          };
          continue;
        }

        if let Some((left_bp, right_bp)) = infix_binding_power(op) {
          if left_bp < min_bp {
            break;
          }
          self.tokens.advance();
          let right = self.parse_expression_bp(right_bp)?;
          left = SyntaxNode::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
          };
          continue;
        }
      }

      if had_ws && is_intersection_rhs_start(self.tokens.peek()) {
        let (left_bp, right_bp) = infix_binding_power(LexOperator::Intersection)?;
        if left_bp < min_bp {
          break;
        }
        let before_rhs = self.tokens.position();
        if let Some(right) = self.parse_expression_bp(right_bp) {
          left = SyntaxNode::Binary {
            op: LexOperator::Intersection,
            left: Box::new(left),
            right: Box::new(right),
          };
          continue;
        }
        self.tokens.set_position(before_rhs);
      }

      break;
    }
    Some(left)
  }

  fn parse_prefix(&mut self) -> Option<SyntaxNode> {
    if let Some(op) = self.tokens.consume_operator_where(prefix_operator) {
      return Some(SyntaxNode::Unary {
        op,
        expr: Box::new(self.parse_expression_bp(prefix_binding_power())?),
      });
    }
    if self
      .tokens
      .consume_token_kind(LexTokenKind::ParenOpen)
      .is_some()
    {
      let expr = self.parse_expression()?;
      if self
        .tokens
        .consume_token_kind(LexTokenKind::ParenClose)
        .is_none()
      {
        self.issues.push(SyntaxIssue::MissingClosingParenthesis);
      }
      return Some(expr);
    }
    if let Some(token) = self.tokens.consume_token_kind(LexTokenKind::Text) {
      return Some(SyntaxNode::Text(token.into()));
    }
    if self
      .tokens
      .consume_token_kind(LexTokenKind::ArrayOpen)
      .is_some()
    {
      return self.parse_array();
    }
    if let Some(token) = self.tokens.peek() {
      match token.kind {
        LexTokenKind::Number(value) => {
          self.tokens.advance();
          return Some(SyntaxNode::Number(value));
        }
        LexTokenKind::Error(error) => {
          self.tokens.advance();
          return Some(SyntaxNode::Error(error));
        }
        _ => {}
      }
    }
    self.parse_word_or_function()
  }

  fn parse_array(&mut self) -> Option<SyntaxNode> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    loop {
      if self
        .tokens
        .consume_token_kind(LexTokenKind::ArrayClose)
        .is_some()
      {
        if !row.is_empty() {
          rows.push(row);
        } else if !rows.is_empty() {
          rows.push(vec![SyntaxNode::Blank]);
        }
        break;
      }
      if self
        .tokens
        .consume_token_kind(LexTokenKind::ArgumentSeparator)
        .is_some()
      {
        row.push(SyntaxNode::Blank);
        continue;
      }
      if self
        .tokens
        .consume_token_kind(LexTokenKind::RowSeparator)
        .is_some()
      {
        if row.is_empty() {
          row.push(SyntaxNode::Blank);
        }
        rows.push(row);
        row = Vec::new();
        continue;
      }
      row.push(self.parse_expression()?);
      if self
        .tokens
        .consume_token_kind(LexTokenKind::ArgumentSeparator)
        .is_some()
      {
        continue;
      }
      if self
        .tokens
        .consume_token_kind(LexTokenKind::RowSeparator)
        .is_some()
      {
        rows.push(row);
        row = Vec::new();
        continue;
      }
      if self
        .tokens
        .consume_token_kind(LexTokenKind::ArrayClose)
        .is_some()
      {
        rows.push(row);
        break;
      }
      return None;
    }
    Some(SyntaxNode::Array(rows))
  }

  fn parse_word_or_function(&mut self) -> Option<SyntaxNode> {
    let token = self.tokens.peek()?;
    if token.kind != LexTokenKind::Word {
      return None;
    }
    if let Some(split) = split_word_before_intersection(self.source, token) {
      self.tokens.advance_split_word(self.source, split.end);
      return Some(SyntaxNode::Word(split));
    }
    let name = SyntaxSpan::from(token);
    self.tokens.advance();
    if self
      .tokens
      .peek()
      .is_some_and(|token| token.kind == LexTokenKind::ParenOpen)
    {
      return Some(SyntaxNode::Function {
        name,
        args: self.parse_argument_list()?,
      });
    }
    Some(SyntaxNode::Word(name))
  }

  fn parse_argument_list(&mut self) -> Option<Vec<SyntaxNode>> {
    if self
      .tokens
      .consume_token_kind(LexTokenKind::ParenOpen)
      .is_none()
    {
      return None;
    }
    let mut args = Vec::new();
    loop {
      if self
        .tokens
        .consume_token_kind(LexTokenKind::ParenClose)
        .is_some()
      {
        break;
      }
      if self
        .tokens
        .consume_token_kind(LexTokenKind::ArgumentSeparator)
        .is_some()
      {
        args.push(SyntaxNode::Blank);
        continue;
      }
      args.push(self.parse_expression()?);
      if self
        .tokens
        .consume_token_kind(LexTokenKind::ParenClose)
        .is_some()
      {
        break;
      }
      if self
        .tokens
        .consume_token_kind(LexTokenKind::ArgumentSeparator)
        .is_none()
      {
        return None;
      }
      if self
        .tokens
        .peek()
        .is_some_and(|token| token.kind == LexTokenKind::ParenClose)
      {
        args.push(SyntaxNode::Blank);
      }
    }
    Some(args)
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SyntaxToken {
  token: LexToken,
  preceded_by_ws: bool,
}

struct SyntaxTokens {
  tokens: Vec<SyntaxToken>,
  index: usize,
}

impl SyntaxTokens {
  fn new(source: &str) -> Self {
    Self {
      tokens: Self::tokens_from(source, 0),
      index: 0,
    }
  }

  fn from_tokens(source: &str, tokens: &[LexToken]) -> Self {
    Self {
      tokens: Self::tokens_from_absolute(source, 0, tokens.iter().copied()),
      index: 0,
    }
  }

  fn tokens_from(source: &str, offset: usize) -> Vec<SyntaxToken> {
    let tokens = lex_tokens(source.get(offset..).unwrap_or_default()).map(|token| LexToken {
      kind: token.kind,
      start: token.start + offset,
      end: token.end + offset,
    });
    Self::tokens_from_absolute(source, offset, tokens)
  }

  fn tokens_from_absolute(
    source: &str,
    mut previous_end: usize,
    tokens: impl IntoIterator<Item = LexToken>,
  ) -> Vec<SyntaxToken> {
    tokens
      .into_iter()
      .map(|token| {
        let token = LexToken {
          kind: token.kind,
          start: token.start,
          end: token.end,
        };
        let preceded_by_ws = source
          .get(previous_end..token.start)
          .is_some_and(|text| !text.is_empty());
        previous_end = token.end;
        SyntaxToken {
          token,
          preceded_by_ws,
        }
      })
      .collect()
  }

  fn is_end(&self) -> bool {
    self.index >= self.tokens.len()
  }

  fn position(&self) -> usize {
    self.index
  }

  fn set_position(&mut self, index: usize) {
    self.index = index.min(self.tokens.len());
  }

  fn peek(&self) -> Option<LexToken> {
    self.tokens.get(self.index).map(|entry| entry.token)
  }

  fn advance(&mut self) -> Option<LexToken> {
    let token = self.peek()?;
    self.index += 1;
    Some(token)
  }

  fn ws_before_next(&self) -> bool {
    self
      .tokens
      .get(self.index)
      .map(|entry| entry.preceded_by_ws)
      .unwrap_or(false)
  }

  fn consume_token_kind(&mut self, kind: LexTokenKind) -> Option<LexToken> {
    let token = self.peek()?;
    if token.kind != kind {
      return None;
    }
    self.advance()
  }

  fn consume_operator_where(
    &mut self,
    predicate: impl FnOnce(LexOperator) -> bool,
  ) -> Option<LexOperator> {
    let token = self.peek()?;
    let LexTokenKind::Operator(operator) = token.kind else {
      return None;
    };
    if !predicate(operator) {
      return None;
    }
    self.advance();
    Some(operator)
  }

  fn consume_logical_function_call(&mut self, source: &str) -> Option<LexLogicalFunction> {
    let token = self.peek()?;
    if token.kind != LexTokenKind::Word {
      return None;
    }
    let word = source.get(token.start..token.end)?;
    let function = logical_function_name(word)?;
    let next = self.tokens.get(self.index + 1)?;
    if next.token.kind != LexTokenKind::ParenOpen {
      return None;
    }
    self.advance();
    Some(function)
  }

  fn advance_split_word(&mut self, source: &str, end: usize) {
    if self.tokens.get(self.index).is_none() {
      return;
    }
    if let Some(entry) = self.tokens.get_mut(self.index) {
      entry.token.end = end;
    }
    self.advance();
    let tail = Self::tokens_from(source, end);
    self.tokens.splice(self.index..self.tokens.len(), tail);
  }
}

fn logical_function_name(value: &str) -> Option<LexLogicalFunction> {
  if value.eq_ignore_ascii_case("AND") {
    Some(LexLogicalFunction::And)
  } else if value.eq_ignore_ascii_case("OR") {
    Some(LexLogicalFunction::Or)
  } else if value.eq_ignore_ascii_case("NOT") {
    Some(LexLogicalFunction::Not)
  } else {
    None
  }
}

fn logical_binding_power() -> u8 {
  1
}

fn infix_binding_power(operator: LexOperator) -> Option<(u8, u8)> {
  match operator {
    LexOperator::Equal
    | LexOperator::NotEqual
    | LexOperator::Less
    | LexOperator::LessOrEqual
    | LexOperator::Greater
    | LexOperator::GreaterOrEqual => Some((2, 3)),
    LexOperator::Union => Some((4, 5)),
    LexOperator::Intersection => Some((6, 7)),
    LexOperator::Range => Some((8, 9)),
    LexOperator::Concat => Some((10, 11)),
    LexOperator::Add | LexOperator::Subtract => Some((12, 13)),
    LexOperator::Multiply | LexOperator::Divide => Some((14, 15)),
    LexOperator::Power => Some((16, 16)),
    LexOperator::Percent => None,
  }
}

fn postfix_binding_power(operator: LexOperator) -> Option<u8> {
  (operator == LexOperator::Percent).then_some(18)
}

fn prefix_binding_power() -> u8 {
  17
}

fn prefix_operator(operator: LexOperator) -> bool {
  matches!(operator, LexOperator::Add | LexOperator::Subtract)
}

fn split_word_before_intersection(source: &str, token: LexToken) -> Option<SyntaxSpan> {
  let word = &source[token.start..token.end];
  let mut quoted = false;
  let mut chars = word.char_indices().peekable();
  while let Some((index, ch)) = chars.next() {
    match ch {
      '\'' => {
        if quoted && chars.peek().is_some_and(|(_, next)| *next == '\'') {
          chars.next();
        } else {
          quoted = !quoted;
        }
      }
      '!' if !quoted && word[..index].contains(':') => {
        return Some(SyntaxSpan {
          start: token.start,
          end: token.start + index,
        });
      }
      _ => {}
    }
  }
  None
}

fn is_intersection_rhs_start(token: Option<LexToken>) -> bool {
  token.is_some_and(|token| {
    matches!(
      token.kind,
      LexTokenKind::Text
        | LexTokenKind::Number(_)
        | LexTokenKind::Error(_)
        | LexTokenKind::ArrayOpen
        | LexTokenKind::ParenOpen
        | LexTokenKind::Word
    )
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_significant_whitespace_as_intersection_operator() {
    let parse = parse_syntax_ast("B2:D2 C1:C3");

    assert!(parse.complete);
    assert!(parse.issues.is_empty());
    assert!(matches!(
      parse.ast,
      Some(SyntaxNode::Binary {
        op: LexOperator::Intersection,
        ..
      })
    ));
  }

  #[test]
  fn ignores_insignificant_whitespace_before_regular_operators() {
    let parse = parse_syntax_ast("1 + 2");

    assert!(parse.complete);
    assert!(parse.issues.is_empty());
    assert!(matches!(
      parse.ast,
      Some(SyntaxNode::Binary {
        op: LexOperator::Add,
        ..
      })
    ));
  }
}
