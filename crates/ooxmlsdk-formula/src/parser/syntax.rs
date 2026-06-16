use super::{FormulaCursor, LexLogicalFunction, LexOperator, LexToken, LexTokenKind};

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
  parser.cursor.skip_ws();
  SyntaxParse {
    ast,
    complete: parser.cursor.is_end(),
    issues: parser.issues,
  }
}

struct SyntaxParser<'a> {
  cursor: FormulaCursor<'a>,
  issues: Vec<SyntaxIssue>,
}

impl<'a> SyntaxParser<'a> {
  fn new(source: &'a str) -> Self {
    Self {
      cursor: FormulaCursor::new(source),
      issues: Vec::new(),
    }
  }

  fn parse_expression(&mut self) -> Option<SyntaxNode> {
    self.parse_logical()
  }

  fn parse_logical(&mut self) -> Option<SyntaxNode> {
    let mut left = self.parse_comparison()?;
    loop {
      self.cursor.skip_ws();
      let Some(function) = self.cursor.consume_adjacent_logical_function() else {
        break;
      };
      let mut args = self.parse_argument_list()?;
      args.insert(0, left);
      left = SyntaxNode::LogicalFunction { function, args };
    }
    Some(left)
  }

  fn parse_comparison(&mut self) -> Option<SyntaxNode> {
    let mut left = self.parse_union()?;
    loop {
      self.cursor.skip_ws();
      let Some(op) = self.cursor.consume_operator_where(|operator| {
        matches!(
          operator,
          LexOperator::Equal
            | LexOperator::NotEqual
            | LexOperator::Less
            | LexOperator::LessOrEqual
            | LexOperator::Greater
            | LexOperator::GreaterOrEqual
        )
      }) else {
        break;
      };
      let right = self.parse_union()?;
      left = SyntaxNode::Binary {
        op,
        left: Box::new(left),
        right: Box::new(right),
      };
    }
    Some(left)
  }

  fn parse_union(&mut self) -> Option<SyntaxNode> {
    self.parse_left_associative(Self::parse_intersection, |operator| {
      operator == LexOperator::Union
    })
  }

  fn parse_intersection(&mut self) -> Option<SyntaxNode> {
    self.parse_left_associative(Self::parse_range, |operator| {
      operator == LexOperator::Intersection
    })
  }

  fn parse_range(&mut self) -> Option<SyntaxNode> {
    self.parse_left_associative(Self::parse_concat, |operator| {
      operator == LexOperator::Range
    })
  }

  fn parse_concat(&mut self) -> Option<SyntaxNode> {
    self.parse_left_associative(Self::parse_add_sub, |operator| {
      operator == LexOperator::Concat
    })
  }

  fn parse_add_sub(&mut self) -> Option<SyntaxNode> {
    self.parse_left_associative(Self::parse_mul_div, |operator| {
      matches!(operator, LexOperator::Add | LexOperator::Subtract)
    })
  }

  fn parse_mul_div(&mut self) -> Option<SyntaxNode> {
    self.parse_left_associative(Self::parse_power, |operator| {
      matches!(operator, LexOperator::Multiply | LexOperator::Divide)
    })
  }

  fn parse_left_associative(
    &mut self,
    mut operand: impl FnMut(&mut Self) -> Option<SyntaxNode>,
    predicate: impl Fn(LexOperator) -> bool,
  ) -> Option<SyntaxNode> {
    let mut left = operand(self)?;
    loop {
      self.cursor.skip_ws();
      let Some(op) = self.cursor.consume_operator_where(&predicate) else {
        break;
      };
      let right = operand(self)?;
      left = SyntaxNode::Binary {
        op,
        left: Box::new(left),
        right: Box::new(right),
      };
    }
    Some(left)
  }

  fn parse_power(&mut self) -> Option<SyntaxNode> {
    let left = self.parse_unary()?;
    self.cursor.skip_ws();
    if let Some(op) = self
      .cursor
      .consume_operator_where(|operator| operator == LexOperator::Power)
    {
      let right = self.parse_power()?;
      Some(SyntaxNode::Binary {
        op,
        left: Box::new(left),
        right: Box::new(right),
      })
    } else {
      Some(left)
    }
  }

  fn parse_unary(&mut self) -> Option<SyntaxNode> {
    self.cursor.skip_ws();
    if let Some(op) = self.cursor.consume_operator_where(|operator| {
      matches!(operator, LexOperator::Add | LexOperator::Subtract)
    }) {
      return Some(SyntaxNode::Unary {
        op,
        expr: Box::new(self.parse_unary()?),
      });
    }
    self.parse_percent()
  }

  fn parse_percent(&mut self) -> Option<SyntaxNode> {
    let mut expr = self.parse_primary()?;
    loop {
      self.cursor.skip_ws();
      let Some(op) = self
        .cursor
        .consume_operator_where(|operator| operator == LexOperator::Percent)
      else {
        break;
      };
      expr = SyntaxNode::Unary {
        op,
        expr: Box::new(expr),
      };
    }
    Some(expr)
  }

  fn parse_primary(&mut self) -> Option<SyntaxNode> {
    self.cursor.skip_ws();
    if self
      .cursor
      .consume_token_kind(LexTokenKind::ParenOpen)
      .is_some()
    {
      let expr = self.parse_expression()?;
      self.cursor.skip_ws();
      if self
        .cursor
        .consume_token_kind(LexTokenKind::ParenClose)
        .is_none()
      {
        self.issues.push(SyntaxIssue::MissingClosingParenthesis);
      }
      return Some(expr);
    }
    if let Some(token) = self.cursor.consume_token_kind(LexTokenKind::Text) {
      return Some(SyntaxNode::Text(token.into()));
    }
    if self
      .cursor
      .consume_token_kind(LexTokenKind::ArrayOpen)
      .is_some()
    {
      return self.parse_array();
    }
    if let Some(token) = self.cursor.peek_token_raw() {
      match token.kind {
        LexTokenKind::Number(value) => {
          self.cursor.set_index(token.end);
          return Some(SyntaxNode::Number(value));
        }
        LexTokenKind::Error(error) => {
          self.cursor.set_index(token.end);
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
      self.cursor.skip_ws();
      if self
        .cursor
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
        .cursor
        .consume_token_kind(LexTokenKind::ArgumentSeparator)
        .is_some()
      {
        row.push(SyntaxNode::Blank);
        continue;
      }
      if self
        .cursor
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
      self.cursor.skip_ws();
      if self
        .cursor
        .consume_token_kind(LexTokenKind::ArgumentSeparator)
        .is_some()
      {
        continue;
      }
      if self
        .cursor
        .consume_token_kind(LexTokenKind::RowSeparator)
        .is_some()
      {
        rows.push(row);
        row = Vec::new();
        continue;
      }
      if self
        .cursor
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
    let token = self.cursor.peek_token_raw()?;
    if token.kind != LexTokenKind::Word {
      return None;
    }
    if let Some(split) = split_word_before_intersection(self.cursor.source(), token) {
      self.cursor.set_index(split.end);
      return Some(SyntaxNode::Word(split));
    }
    let name = SyntaxSpan::from(token);
    self.cursor.set_index(token.end);
    self.cursor.skip_ws();
    if self
      .cursor
      .peek_token_raw()
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
      .cursor
      .consume_token_kind(LexTokenKind::ParenOpen)
      .is_none()
    {
      return None;
    }
    let mut args = Vec::new();
    loop {
      self.cursor.skip_ws();
      if self
        .cursor
        .consume_token_kind(LexTokenKind::ParenClose)
        .is_some()
      {
        break;
      }
      if self
        .cursor
        .consume_token_kind(LexTokenKind::ArgumentSeparator)
        .is_some()
      {
        args.push(SyntaxNode::Blank);
        continue;
      }
      args.push(self.parse_expression()?);
      self.cursor.skip_ws();
      if self
        .cursor
        .consume_token_kind(LexTokenKind::ParenClose)
        .is_some()
      {
        break;
      }
      if self
        .cursor
        .consume_token_kind(LexTokenKind::ArgumentSeparator)
        .is_none()
      {
        return None;
      }
      self.cursor.skip_ws();
      if self
        .cursor
        .peek_token_raw()
        .is_some_and(|token| token.kind == LexTokenKind::ParenClose)
      {
        args.push(SyntaxNode::Blank);
      }
    }
    Some(args)
  }
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
