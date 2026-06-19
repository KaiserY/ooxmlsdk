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
  ImplicitIntersection,
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
  Error,
  NotImplemented,
  CircularReference,
  IllegalChar,
  IllegalArgument,
  IllegalParameter,
  Pair,
  PairExpected,
  OperatorExpected,
  VariableExpected,
  Parameter,
  CodeOverflow,
  StringOverflow,
  StackOverflow,
  InvalidVariable,
  InvalidOpcode,
  InvalidStackValue,
  InvalidToken,
  NoConvergence,
  NoAddin,
  NoMacro,
  NestedArray,
  MatrixSize,
  BadArrayContent,
  LinkFormulaNeedingCheck,
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

pub(crate) fn lex_token_at(source: &str, offset: usize) -> Option<LexToken> {
  let mut input = source.get(offset..)?;
  skip_whitespace(&mut input);
  next_raw_token(source, &mut input)
}

pub(crate) struct FormulaCursor<'a> {
  source: &'a str,
  index: usize,
}

impl<'a> FormulaCursor<'a> {
  pub(crate) fn new(source: &'a str) -> Self {
    Self { source, index: 0 }
  }

  pub(crate) fn index(&self) -> usize {
    self.index
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

pub(crate) fn formula_error_value(source: &str) -> Option<LexErrorValue> {
  let mut input = source;
  let value = formula_error.parse_next(&mut input).ok()?;
  input.is_empty().then_some(value)
}

pub(crate) fn grouped_formula_number(value: &str) -> Option<f64> {
  let trimmed = value.trim();
  if trimmed.is_empty() {
    return None;
  }
  let (mantissa, _) = trimmed
    .find(['e', 'E'])
    .map(|index| trimmed.split_at(index))
    .unwrap_or((trimmed, ""));
  let mantissa = mantissa
    .strip_prefix('+')
    .or_else(|| mantissa.strip_prefix('-'))
    .unwrap_or(mantissa);
  let (integer, decimal) = mantissa
    .split_once('.')
    .map(|(integer, decimal)| (integer, Some(decimal)))
    .unwrap_or((mantissa, None));
  if !integer.contains(',') {
    return trimmed.parse::<f64>().ok();
  }
  let mut groups = integer.split(',');
  let first = groups.next()?;
  if first.is_empty() || first.len() > 3 || !first.bytes().all(|byte| byte.is_ascii_digit()) {
    return None;
  }
  if !groups.all(|group| group.len() == 3 && group.bytes().all(|byte| byte.is_ascii_digit())) {
    return None;
  }
  if decimal.is_some_and(|decimal| !decimal.bytes().all(|byte| byte.is_ascii_digit())) {
    return None;
  }
  trimmed.replace(',', "").parse::<f64>().ok()
}

fn formula_body_start_parser(input: &mut &str) -> WinnowResult<usize> {
  let start_len = input.len();
  if starts_with_ignore_ascii_case(input, "of:=") {
    *input = &input["of:=".len()..];
    return Ok(start_len - input.len());
  }
  if starts_with_ignore_ascii_case(input, "of:") {
    *input = &input["of:".len()..];
    return Ok(start_len - input.len());
  }
  let _ = opt(literal("=")).parse_next(input)?;
  let _ = opt(literal("=")).parse_next(input)?;
  Ok(start_len - input.len())
}

fn formula_token_kind(input: &mut &str) -> WinnowResult<LexTokenKind> {
  dispatch! {peek(any);
    '"' => formula_text.value(LexTokenKind::Text),
    '0'..='9' => formula_number.map(LexTokenKind::Number),
    '.' => formula_dot_prefixed_number.map(LexTokenKind::Number),
    '#' | 'E' | 'e' => formula_error_or_word,
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
    '@' => '@'.value(LexTokenKind::Operator(LexOperator::ImplicitIntersection)),
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
  let Some(consumed) = scan_formula_number_len(original) else {
    return fail.parse_next(input);
  };
  match original[..consumed].parse::<f64>() {
    Ok(value) => {
      *input = &input[consumed..];
      Ok(value)
    }
    Err(_) => fail.parse_next(input),
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
  if input.starts_with("#getting_data") {
    *input = &input["#getting_data".len()..];
    return Ok(LexErrorValue::GettingData);
  }
  if input
    .get(..4)
    .is_some_and(|prefix| prefix.eq_ignore_ascii_case("Err:"))
    && let Some((value, consumed)) = formula_err_code_value(&input[4..], false)
  {
    *input = &input[4 + consumed..];
    return Ok(value);
  }
  if input.starts_with("#ERR")
    && let Some((value, consumed)) = formula_err_code_value(&input[4..], true)
  {
    *input = &input[4 + consumed..];
    return Ok(value);
  }
  if let Some((literal_value, error)) = formula_hash_error_literal(input) {
    *input = &input[literal_value.len()..];
    return Ok(error);
  }
  fail.parse_next(input)
}

fn formula_hash_error_literal(input: &str) -> Option<(&'static str, LexErrorValue)> {
  if input.starts_with("#GETTING_DATA") {
    Some(("#GETTING_DATA", LexErrorValue::GettingData))
  } else if input.starts_with("#DIV/0!") {
    Some(("#DIV/0!", LexErrorValue::Div0))
  } else if input.starts_with("#VALUE!") {
    Some(("#VALUE!", LexErrorValue::Value))
  } else if input.starts_with("#NULL!") {
    Some(("#NULL!", LexErrorValue::Null))
  } else if input.starts_with("#NULL") {
    Some(("#NULL", LexErrorValue::Null))
  } else if input.starts_with("#REF!") {
    Some(("#REF!", LexErrorValue::Ref))
  } else if input.starts_with("#NAME?") {
    Some(("#NAME?", LexErrorValue::Name))
  } else if input.starts_with("#NUM!") {
    Some(("#NUM!", LexErrorValue::Num))
  } else if input.starts_with("#N/A") {
    Some(("#N/A", LexErrorValue::NA))
  } else if input.starts_with("#SPILL!") {
    Some(("#SPILL!", LexErrorValue::Spill))
  } else if input.starts_with("#CALC!") {
    Some(("#CALC!", LexErrorValue::Calc))
  } else if input.starts_with("#ERROR!") {
    Some(("#ERROR!", LexErrorValue::Error))
  } else if input.starts_with("#N/IMPL!") {
    Some(("#N/IMPL!", LexErrorValue::NotImplemented))
  } else if input.starts_with("#CIRC!") {
    Some(("#CIRC!", LexErrorValue::CircularReference))
  } else {
    None
  }
}

fn formula_err_code_value(input: &str, requires_bang: bool) -> Option<(LexErrorValue, usize)> {
  let bytes = input.as_bytes();
  let index = if requires_bang {
    let mut index = 0usize;
    while bytes.get(index).is_some_and(u8::is_ascii_digit) {
      index += 1;
    }
    index
  } else {
    3
  };
  if index == 0 || input.len() < index || requires_bang && bytes.get(index) != Some(&b'!') {
    return None;
  }
  if !input[..index].bytes().all(|byte| byte.is_ascii_digit()) {
    return None;
  }
  let code = input[..index].parse::<u16>().ok()?;
  let consumed = if requires_bang { index + 1 } else { index };
  Some((formula_error_code_value(code)?, consumed))
}

fn formula_error_code_value(code: u16) -> Option<LexErrorValue> {
  Some(match code {
    501 => LexErrorValue::IllegalChar,
    502 => LexErrorValue::IllegalArgument,
    503 => LexErrorValue::Num,
    504 => LexErrorValue::IllegalParameter,
    507 => LexErrorValue::Pair,
    508 => LexErrorValue::PairExpected,
    509 => LexErrorValue::OperatorExpected,
    510 => LexErrorValue::VariableExpected,
    511 => LexErrorValue::Parameter,
    512 => LexErrorValue::CodeOverflow,
    513 => LexErrorValue::StringOverflow,
    514 => LexErrorValue::StackOverflow,
    515 => LexErrorValue::Error,
    516 => LexErrorValue::InvalidVariable,
    517 => LexErrorValue::InvalidOpcode,
    518 => LexErrorValue::InvalidStackValue,
    519 => LexErrorValue::Value,
    520 => LexErrorValue::InvalidToken,
    521 => LexErrorValue::Null,
    522 => LexErrorValue::CircularReference,
    523 => LexErrorValue::NoConvergence,
    524 => LexErrorValue::Ref,
    525 => LexErrorValue::Name,
    530 => LexErrorValue::NoAddin,
    531 => LexErrorValue::NoMacro,
    532 => LexErrorValue::Div0,
    533 => LexErrorValue::NestedArray,
    538 => LexErrorValue::MatrixSize,
    539 => LexErrorValue::BadArrayContent,
    540 => LexErrorValue::LinkFormulaNeedingCheck,
    541 => LexErrorValue::Spill,
    _ => return None,
  })
}

fn formula_error_or_word(input: &mut &str) -> WinnowResult<LexTokenKind> {
  let original = *input;
  if could_start_formula_error(original)
    && let Ok(error) = formula_error.parse_next(input)
  {
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

fn scan_formula_number_len(input: &str) -> Option<usize> {
  let bytes = input.as_bytes();
  let mut index = 0usize;
  let integer_start = index;
  while bytes.get(index).is_some_and(u8::is_ascii_digit) {
    index += 1;
  }
  let integer_digits = index - integer_start;

  let mut decimal_digits = 0usize;
  if bytes.get(index) == Some(&b'.') {
    index += 1;
    let decimal_start = index;
    while bytes.get(index).is_some_and(u8::is_ascii_digit) {
      index += 1;
    }
    decimal_digits = index - decimal_start;
  }

  if integer_digits == 0 && decimal_digits == 0 {
    return None;
  }

  if matches!(bytes.get(index), Some(b'E' | b'e')) {
    index += 1;
    if matches!(bytes.get(index), Some(b'+' | b'-')) {
      index += 1;
    }
    let digits_start = index;
    while bytes.get(index).is_some_and(u8::is_ascii_digit) {
      index += 1;
    }
    if index == digits_start {
      return None;
    }
    return Some(index);
  }

  Some(index)
}

fn could_start_formula_error(input: &str) -> bool {
  input.starts_with('#')
    || input
      .get(..4)
      .is_some_and(|prefix| prefix.eq_ignore_ascii_case("Err:"))
}

fn scan_formula_word(input: &mut &str) {
  let original = *input;
  let mut quoted = false;
  let mut table_ref_depth = 0i32;
  while let Some(ch) = input.chars().next() {
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
    if !quoted && ch == ':' && should_stop_formula_word_at_range_operator(original, input) {
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
  ch.is_alphanumeric()
    || matches!(
      ch,
      '$' | ':' | '!' | '\'' | '[' | ']' | '.' | '_' | '#' | '@'
    )
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

fn starts_with_ignore_ascii_case(source: &str, prefix: &str) -> bool {
  source
    .get(..prefix.len())
    .is_some_and(|value| value.eq_ignore_ascii_case(prefix))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn formula_body_start_skips_excel_and_open_formula_prefixes() {
    assert_eq!(formula_body_start("A1"), 0);
    assert_eq!(formula_body_start("=A1"), 1);
    assert_eq!(formula_body_start("==A1"), 2);
    assert_eq!(formula_body_start("of:=A1"), 4);
    assert_eq!(formula_body_start("OF:A1"), 3);
  }

  #[test]
  fn formula_error_values_cover_calc_and_excel_forms() {
    assert_eq!(formula_error_value("#DIV/0!"), Some(LexErrorValue::Div0));
    assert_eq!(
      formula_error_value("#GETTING_DATA"),
      Some(LexErrorValue::GettingData)
    );
    assert_eq!(
      formula_error_value("#getting_data"),
      Some(LexErrorValue::GettingData)
    );
    assert_eq!(formula_error_value("Err:541"), Some(LexErrorValue::Spill));
    assert_eq!(formula_error_value("eRr:524"), Some(LexErrorValue::Ref));
    assert_eq!(formula_error_value("#ERR541!"), Some(LexErrorValue::Spill));
  }
}
