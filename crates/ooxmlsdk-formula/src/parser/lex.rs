use winnow::Parser;
use winnow::Result as WinnowResult;
use winnow::combinator::{dispatch, fail, opt, peek};
use winnow::token::{any, literal};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct LexToken {
  pub kind: LexTokenKind,
  pub start: usize,
  pub end: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum LexTokenKind {
  Text,
  Number(f64),
  Error(LexErrorValue),
  Operator(LexOperator),
  ArrayOpen,
  ArrayClose,
  ArgumentSeparator,
  RowSeparator,
  ParenOpen,
  ParenClose,
  Word,
  Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LexOperator {
  Add,
  Subtract,
  Multiply,
  Divide,
  Power,
  Concat,
  Equal,
  NotEqual,
  Less,
  LessOrEqual,
  Greater,
  GreaterOrEqual,
  Range,
  Union,
  Intersection,
  Percent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LexErrorValue {
  Null,
  Div0,
  Value,
  Ref,
  Name,
  Num,
  NA,
  GettingData,
  Spill,
  Calc,
  IllegalArgument,
  Parameter,
  Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LexLogicalFunction {
  And,
  Or,
  Not,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TextLiteral<'a> {
  Borrowed(&'a str),
  Owned(String),
}

impl LexLogicalFunction {
  pub(crate) fn name(self) -> &'static str {
    match self {
      Self::And => "AND",
      Self::Or => "OR",
      Self::Not => "NOT",
    }
  }
}

struct Lexer<'a> {
  source: &'a str,
  input: &'a str,
}

impl<'a> Lexer<'a> {
  fn new(source: &'a str) -> Self {
    Self {
      source,
      input: source,
    }
  }

  fn next(&mut self) -> Option<LexToken> {
    skip_whitespace(&mut self.input);
    self.next_raw()
  }

  fn next_raw(&mut self) -> Option<LexToken> {
    next_raw_token(self.source, &mut self.input)
  }
}

pub(crate) struct LexTokens<'a> {
  lexer: Lexer<'a>,
}

impl Iterator for LexTokens<'_> {
  type Item = LexToken;

  fn next(&mut self) -> Option<Self::Item> {
    self.lexer.next()
  }
}

pub(crate) fn lex_tokens(source: &str) -> LexTokens<'_> {
  LexTokens {
    lexer: Lexer::new(source),
  }
}

pub(crate) struct FormulaCursor<'a> {
  source: &'a str,
  index: usize,
}

impl<'a> FormulaCursor<'a> {
  pub(crate) fn new(source: &'a str) -> Self {
    Self { source, index: 0 }
  }

  pub(crate) fn source(&self) -> &'a str {
    self.source
  }

  pub(crate) fn set_index(&mut self, index: usize) -> bool {
    if self.source.get(index..).is_some() {
      self.index = index;
      true
    } else {
      false
    }
  }

  fn rest(&self) -> &'a str {
    &self.source[self.index..]
  }

  pub(crate) fn skip_ws(&mut self) {
    let mut input = self.rest();
    skip_whitespace(&mut input);
    self.index = self.source.len() - input.len();
  }

  pub(crate) fn is_end(&self) -> bool {
    self.index >= self.source.len()
  }

  pub(crate) fn peek_token_raw(&self) -> Option<LexToken> {
    let mut input = self.rest();
    next_raw_token(self.source, &mut input)
  }

  pub(crate) fn consume_token_kind(&mut self, kind: LexTokenKind) -> Option<LexToken> {
    let token = self.peek_token_raw()?;
    if token.kind != kind {
      return None;
    }
    self.index = token.end;
    Some(token)
  }

  pub(crate) fn consume_operator_where(
    &mut self,
    predicate: impl FnOnce(LexOperator) -> bool,
  ) -> Option<LexOperator> {
    let token = self.peek_token_raw()?;
    let LexTokenKind::Operator(operator) = token.kind else {
      return None;
    };
    if !predicate(operator) {
      return None;
    }
    self.index = token.end;
    Some(operator)
  }

  pub(crate) fn consume_adjacent_logical_function(&mut self) -> Option<LexLogicalFunction> {
    let rest = self.rest();
    let function = if rest
      .get(..3)
      .is_some_and(|value| value.eq_ignore_ascii_case("AND"))
    {
      LexLogicalFunction::And
    } else if rest
      .get(..2)
      .is_some_and(|value| value.eq_ignore_ascii_case("OR"))
    {
      LexLogicalFunction::Or
    } else if rest
      .get(..3)
      .is_some_and(|value| value.eq_ignore_ascii_case("NOT"))
    {
      LexLogicalFunction::Not
    } else {
      return None;
    };
    let next = self.index + function.name().len();
    if next < self.source.len() {
      let next_char = self.source[next..].chars().next()?;
      if is_formula_word_char(next_char) {
        return None;
      }
    }
    let mut probe = next;
    while let Some(ch) = self.source[probe..].chars().next() {
      if !ch.is_whitespace() {
        break;
      }
      probe += ch.len_utf8();
    }
    if self.source[probe..].starts_with('(') {
      self.index = next;
      Some(function)
    } else {
      None
    }
  }
}

fn next_raw_token(source: &str, input: &mut &str) -> Option<LexToken> {
  if input.is_empty() {
    return None;
  }
  let start = source.len() - input.len();
  let kind = formula_token_kind.parse_next(input).unwrap_or_else(|_| {
    consume_one_char(input);
    LexTokenKind::Unsupported
  });
  let end = source.len() - input.len();
  Some(LexToken { kind, start, end })
}

pub(crate) fn formula_body_start(source: &str) -> usize {
  let mut input = source;
  formula_body_start_parser
    .parse_next(&mut input)
    .unwrap_or(0)
}

pub(crate) fn formula_text_literal(source: &str, start: usize) -> Option<TextLiteral<'_>> {
  if source.as_bytes().get(start) != Some(&b'"') {
    return None;
  }
  let body_start = start + 1;
  let mut index = body_start;
  let mut segment_start = body_start;
  let mut parsed = None::<String>;
  while index < source.len() {
    let char_start = index;
    let ch = source[index..].chars().next()?;
    index += ch.len_utf8();
    if ch != '"' {
      continue;
    }
    if source[index..].starts_with('"') {
      let output = parsed.get_or_insert_with(String::new);
      output.push_str(&source[segment_start..char_start]);
      output.push('"');
      index += 1;
      segment_start = index;
      continue;
    }
    return Some(match parsed {
      Some(mut output) => {
        output.push_str(&source[segment_start..char_start]);
        TextLiteral::Owned(output)
      }
      None => TextLiteral::Borrowed(&source[body_start..char_start]),
    });
  }
  Some(match parsed {
    Some(mut output) => {
      output.push_str(&source[segment_start..]);
      TextLiteral::Owned(output)
    }
    None => TextLiteral::Borrowed(&source[body_start..]),
  })
}

fn formula_body_start_parser(input: &mut &str) -> WinnowResult<usize> {
  let start_len = input.len();
  let _ = opt(literal("=")).parse_next(input)?;
  let _ = opt(literal("=")).parse_next(input)?;
  Ok(start_len - input.len())
}

fn formula_token_kind(input: &mut &str) -> WinnowResult<LexTokenKind> {
  dispatch! {peek(any);
    '"' => formula_text.value(LexTokenKind::Text),
    '0'..='9' => formula_number.map(LexTokenKind::Number),
    '.' => formula_dot_prefixed_number.map(LexTokenKind::Number),
    '#' | 'E' => formula_error_or_word,
    '<' => formula_comparison_operator,
    '>' => formula_comparison_operator,
    '+' => '+'.value(LexTokenKind::Operator(LexOperator::Add)),
    '-' => '-'.value(LexTokenKind::Operator(LexOperator::Subtract)),
    '*' => '*'.value(LexTokenKind::Operator(LexOperator::Multiply)),
    '/' => '/'.value(LexTokenKind::Operator(LexOperator::Divide)),
    '^' => '^'.value(LexTokenKind::Operator(LexOperator::Power)),
    '&' => '&'.value(LexTokenKind::Operator(LexOperator::Concat)),
    '=' => '='.value(LexTokenKind::Operator(LexOperator::Equal)),
    ':' => ':'.value(LexTokenKind::Operator(LexOperator::Range)),
    '~' => '~'.value(LexTokenKind::Operator(LexOperator::Union)),
    '!' => '!'.value(LexTokenKind::Operator(LexOperator::Intersection)),
    '%' => '%'.value(LexTokenKind::Operator(LexOperator::Percent)),
    '{' => '{'.value(LexTokenKind::ArrayOpen),
    '}' => '}'.value(LexTokenKind::ArrayClose),
    ',' => ','.value(LexTokenKind::ArgumentSeparator),
    ';' => ';'.value(LexTokenKind::RowSeparator),
    '|' => '|'.value(LexTokenKind::RowSeparator),
    '(' => '('.value(LexTokenKind::ParenOpen),
    ')' => ')'.value(LexTokenKind::ParenClose),
    _ => formula_word_or_unknown,
  }
  .parse_next(input)
}

fn formula_comparison_operator(input: &mut &str) -> WinnowResult<LexTokenKind> {
  if input.starts_with("<>") {
    *input = &input[2..];
    return Ok(LexTokenKind::Operator(LexOperator::NotEqual));
  }
  if input.starts_with("<=") {
    *input = &input[2..];
    return Ok(LexTokenKind::Operator(LexOperator::LessOrEqual));
  }
  if input.starts_with(">=") {
    *input = &input[2..];
    return Ok(LexTokenKind::Operator(LexOperator::GreaterOrEqual));
  }
  match input.chars().next() {
    Some('<') => {
      consume_one_char(input);
      Ok(LexTokenKind::Operator(LexOperator::Less))
    }
    Some('>') => {
      consume_one_char(input);
      Ok(LexTokenKind::Operator(LexOperator::Greater))
    }
    _ => fail.parse_next(input),
  }
}

fn formula_text(input: &mut &str) -> WinnowResult<()> {
  let bytes = input.as_bytes();
  if bytes.first() != Some(&b'"') {
    return fail.parse_next(input);
  }
  let mut index = 1usize;
  while index < bytes.len() {
    if bytes[index] == b'"' {
      if bytes.get(index + 1) == Some(&b'"') {
        index += 2;
      } else {
        *input = &input[index + 1..];
        return Ok(());
      }
    } else {
      index += 1;
    }
  }
  *input = "";
  Ok(())
}

fn formula_number(input: &mut &str) -> WinnowResult<f64> {
  let original = *input;
  let start_len = input.len();
  scan_formula_number(input);
  if input.len() == start_len {
    fail.parse_next(input)
  } else {
    let consumed = start_len - input.len();
    Ok(original[..consumed].parse::<f64>().unwrap_or_default())
  }
}

fn formula_dot_prefixed_number(input: &mut &str) -> WinnowResult<f64> {
  let bytes = input.as_bytes();
  if bytes.first() != Some(&b'.') || !bytes.get(1).is_some_and(u8::is_ascii_digit) {
    return fail.parse_next(input);
  }
  formula_number(input)
}

fn formula_error(input: &mut &str) -> WinnowResult<LexErrorValue> {
  for (literal_value, error) in formula_error_literals() {
    if input.starts_with(literal_value) {
      *input = &input[literal_value.len()..];
      return Ok(*error);
    }
  }
  fail.parse_next(input)
}

fn formula_error_or_word(input: &mut &str) -> WinnowResult<LexTokenKind> {
  let original = *input;
  if let Ok(error) = formula_error.parse_next(input) {
    return Ok(LexTokenKind::Error(error));
  }
  *input = original;
  formula_word_or_unknown(input)
}

fn formula_word_or_unknown(input: &mut &str) -> WinnowResult<LexTokenKind> {
  let start_len = input.len();
  scan_formula_word(input);
  if input.len() != start_len {
    return Ok(LexTokenKind::Word);
  }
  any.value(LexTokenKind::Unsupported).parse_next(input)
}

fn scan_formula_number(input: &mut &str) {
  let bytes = input.as_bytes();
  let mut index = 0usize;
  let mut previous_was_exponent = false;
  while index < bytes.len() {
    let byte = bytes[index];
    if byte.is_ascii_digit() || byte == b'.' || matches!(byte, b'E' | b'e') {
      previous_was_exponent = matches!(byte, b'E' | b'e');
      index += 1;
    } else if matches!(byte, b'+' | b'-') && previous_was_exponent {
      previous_was_exponent = false;
      index += 1;
    } else {
      break;
    }
  }
  *input = &input[index..];
}

fn scan_formula_word(input: &mut &str) {
  let original = *input;
  let mut quoted = false;
  let mut table_ref_depth = 0i32;
  loop {
    let Some(ch) = input.chars().next() else {
      break;
    };
    if table_ref_depth > 0 {
      match ch {
        '[' => table_ref_depth += 1,
        ']' => table_ref_depth -= 1,
        _ => {}
      }
      consume_one_char(input);
      continue;
    }
    if ch == '\'' {
      quoted = !quoted;
      consume_one_char(input);
      continue;
    }
    if !quoted && ch == '[' {
      table_ref_depth += 1;
      consume_one_char(input);
      continue;
    }
    if !quoted && ch == ':' && should_stop_formula_word_at_range_operator(original, *input) {
      break;
    }
    if !quoted && !is_formula_word_char(ch) {
      break;
    }
    consume_one_char(input);
  }
}

fn should_stop_formula_word_at_range_operator(original: &str, input: &str) -> bool {
  let consumed = original.len() - input.len();
  let mut next = consumed + ':'.len_utf8();
  while next < original.len() {
    let Some(ch) = original[next..].chars().next() else {
      break;
    };
    if !ch.is_whitespace() {
      break;
    }
    next += ch.len_utf8();
  }
  if original[next..].starts_with('(') {
    return true;
  }
  let Some(ch) = original[next..].chars().next() else {
    return false;
  };
  if !(ch.is_ascii_alphabetic() || ch == '_' || ch == '.') {
    return false;
  }
  let start = next;
  next += ch.len_utf8();
  while next < original.len() {
    let Some(ch) = original[next..].chars().next() else {
      break;
    };
    if !(ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_')) {
      break;
    }
    next += ch.len_utf8();
  }
  if original[start..next]
    .chars()
    .any(|ch| ch.is_ascii_digit() || ch == '.')
  {
    return false;
  }
  while next < original.len() {
    let Some(ch) = original[next..].chars().next() else {
      break;
    };
    if !ch.is_whitespace() {
      break;
    }
    next += ch.len_utf8();
  }
  original[next..].starts_with('(')
}

fn is_formula_word_char(ch: char) -> bool {
  ch.is_alphanumeric() || matches!(ch, '$' | ':' | '!' | '\'' | '[' | ']' | '.' | '_' | '#')
}

fn skip_whitespace(input: &mut &str) {
  let bytes = input.as_bytes();
  let mut index = 0usize;
  while index < bytes.len() && bytes[index].is_ascii_whitespace() {
    index += 1;
  }
  *input = &input[index..];
  while input.chars().next().is_some_and(char::is_whitespace) {
    consume_one_char(input);
  }
}

fn consume_one_char(input: &mut &str) {
  if let Some(ch) = input.chars().next() {
    *input = &input[ch.len_utf8()..];
  }
}

fn formula_error_literals() -> &'static [(&'static str, LexErrorValue)] {
  &[
    ("#GETTING_DATA", LexErrorValue::GettingData),
    ("#getting_data", LexErrorValue::Unknown),
    ("#DIV/0!", LexErrorValue::Div0),
    ("#VALUE!", LexErrorValue::Value),
    ("#NULL!", LexErrorValue::Null),
    ("#NULL", LexErrorValue::Null),
    ("#REF!", LexErrorValue::Ref),
    ("#NAME?", LexErrorValue::Name),
    ("#NUM!", LexErrorValue::Num),
    ("#N/A", LexErrorValue::NA),
    ("#SPILL!", LexErrorValue::Spill),
    ("#CALC!", LexErrorValue::Calc),
    ("Err:502", LexErrorValue::IllegalArgument),
    ("Err:504", LexErrorValue::Unknown),
    ("Err:508", LexErrorValue::Unknown),
    ("Err:511", LexErrorValue::Parameter),
    ("#ERR502!", LexErrorValue::Unknown),
    ("#ERR508!", LexErrorValue::Unknown),
    ("#ERR504!", LexErrorValue::Unknown),
    ("#ERR511!", LexErrorValue::Parameter),
  ]
}
