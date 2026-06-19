use super::lex::{LexToken, formula_body_start, lex_tokens};
use super::semantic::{
  ExternalReferenceSpans, SemanticSpan, SemanticToken, SemanticTokenKind, semantic_token_from_lex,
};
use super::{LexErrorValue, LexOperator};
use crate::CellAddress;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FormulaBodyParse {
  pub lexed: Vec<LexToken>,
  pub tokens: Vec<FormulaBodyToken>,
  pub issues: Vec<FormulaParseIssue>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FormulaSourceParse<'a> {
  pub body_start: usize,
  pub body: &'a str,
  pub body_parse: FormulaBodyParse,
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
}

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
}

fn parse_formula_body(source: &str) -> FormulaBodyParse {
  let mut lexed = Vec::new();
  let mut tokens = Vec::new();
  let mut issues = Vec::new();

  for token in lex_tokens(source) {
    let semantic = semantic_token_from_lex(source, token);
    if let Some(token) = formula_body_token(semantic, &mut issues) {
      tokens.push(token);
    }
    lexed.push(token);
  }

  FormulaBodyParse {
    lexed,
    tokens,
    issues,
  }
}

fn formula_body_token(
  token: SemanticToken,
  issues: &mut Vec<FormulaParseIssue>,
) -> Option<FormulaBodyToken> {
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
    SemanticTokenKind::ExternalReference(value) => FormulaBodyTokenKind::ExternalReference(value),
    SemanticTokenKind::ReferenceCandidate => FormulaBodyTokenKind::ReferenceCandidate,
    SemanticTokenKind::Name => FormulaBodyTokenKind::Name,
    SemanticTokenKind::Unsupported => {
      issues.push(FormulaParseIssue::UnrecognizedCharacter(span));
      FormulaBodyTokenKind::Unsupported
    }
  };
  Some(FormulaBodyToken { kind, span })
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
  if let Some(offset) = parse_r1c1_relative(reference, b'C') {
    let column = base.column.checked_add_signed(offset)?.checked_add(1)?;
    let column = one_based_column_name(column);
    return Some(format!("{column}:{column}"));
  }
  if let Some(offset) = parse_r1c1_relative(reference, b'R') {
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

fn parse_r1c1_relative(reference: &str, axis: u8) -> Option<i32> {
  let rest = strip_ascii_axis_prefix(reference, axis)?;
  if rest.is_empty() {
    return Some(0);
  }
  let offset = rest.strip_prefix('[')?.strip_suffix(']')?;
  offset.parse::<i32>().ok()
}

fn parse_r1c1_cell(reference: &str, base: CellAddress) -> Option<CellAddress> {
  let reference = normalize_excel_formula_text(reference);
  let rest = strip_ascii_axis_prefix(reference, b'R')?;
  let column_marker = rest
    .char_indices()
    .find_map(|(index, ch)| ch.eq_ignore_ascii_case(&'C').then_some(index))?;
  let (row_text, column_text) = rest.split_at(column_marker);
  let column_text = column_text.get(1..)?;
  let row = parse_r1c1_axis(row_text, base.row)?;
  let column = parse_r1c1_axis(column_text, base.column)?;
  Some(CellAddress { column, row })
}

fn strip_ascii_axis_prefix(reference: &str, axis: u8) -> Option<&str> {
  let bytes = reference.as_bytes();
  bytes
    .first()
    .is_some_and(|byte| byte.eq_ignore_ascii_case(&axis))
    .then(|| &reference[1..])
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
  let mut chars = [0u8; 8];
  let mut len = 0usize;
  while column > 0 {
    column -= 1;
    chars[len] = b'A' + (column % 26) as u8;
    len += 1;
    column /= 26;
  }
  reverse_ascii_column_name(&chars[..len])
}

fn zero_based_column_name(mut column: u32) -> String {
  let mut chars = [0u8; 8];
  let mut len = 0usize;
  loop {
    chars[len] = b'A' + (column % 26) as u8;
    len += 1;
    column /= 26;
    if column == 0 {
      break;
    }
    column -= 1;
  }
  reverse_ascii_column_name(&chars[..len])
}

fn reverse_ascii_column_name(reversed: &[u8]) -> String {
  let mut name = String::with_capacity(reversed.len());
  for byte in reversed.iter().rev() {
    name.push(*byte as char);
  }
  name
}
