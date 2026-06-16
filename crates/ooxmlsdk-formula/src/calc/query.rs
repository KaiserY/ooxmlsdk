use std::borrow::Cow;

use regex::RegexBuilder;

use crate::FormulaSearchType;
use crate::calc::text::{rightb, text_byte_len};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryOp {
  Equal,
  NotEqual,
  Less,
  LessOrEqual,
  Greater,
  GreaterOrEqual,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySearchType {
  Normal,
  Wildcard,
  Regex,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FindTextError {
  Value,
  NotAvailable,
}

pub fn parse_criteria_operator(value: &str) -> (QueryOp, &str) {
  if let Some(rest) = value.strip_prefix("<>") {
    (QueryOp::NotEqual, rest)
  } else if let Some(rest) = value.strip_prefix("<=") {
    (QueryOp::LessOrEqual, rest)
  } else if let Some(rest) = value.strip_prefix(">=") {
    (QueryOp::GreaterOrEqual, rest)
  } else if let Some(rest) = value.strip_prefix('=') {
    (QueryOp::Equal, rest)
  } else if let Some(rest) = value.strip_prefix('<') {
    (QueryOp::Less, rest)
  } else if let Some(rest) = value.strip_prefix('>') {
    (QueryOp::Greater, rest)
  } else {
    (QueryOp::Equal, value)
  }
}

pub fn lookup_search_key(value: &str) -> String {
  value.chars().flat_map(char::to_lowercase).collect()
}

pub fn lookup_text_contains(text: &str, pattern: &str) -> bool {
  lookup_search_key(text).contains(&lookup_search_key(pattern))
}

pub fn wildcard_match(pattern: &str, text: &str) -> bool {
  let pattern = lookup_search_key(pattern).chars().collect::<Vec<_>>();
  let text = lookup_search_key(text).chars().collect::<Vec<_>>();
  let mut p = 0usize;
  let mut t = 0usize;
  let mut star = None;
  let mut star_text = 0usize;
  while t < text.len() {
    if p < pattern.len() {
      if pattern[p] == '~' && p + 1 < pattern.len() {
        if pattern[p + 1] == text[t] {
          p += 2;
          t += 1;
          continue;
        }
      } else if pattern[p] == '?' || pattern[p] == text[t] {
        p += 1;
        t += 1;
        continue;
      } else if pattern[p] == '*' {
        star = Some(p);
        p += 1;
        star_text = t;
        continue;
      }
    }
    if let Some(star_index) = star {
      p = star_index + 1;
      star_text += 1;
      t = star_text;
    } else {
      return false;
    }
  }
  while p < pattern.len() && pattern[p] == '*' {
    p += 1;
  }
  p == pattern.len()
}

pub fn wildcard_search_regex_pattern(pattern: &str) -> String {
  let mut output = String::new();
  let mut chars = pattern.chars();
  while let Some(ch) = chars.next() {
    match ch {
      '*' => output.push_str(".*"),
      '?' => output.push('.'),
      '~' => {
        if let Some(escaped) = chars.next() {
          output.push_str(&regex::escape(&escaped.to_string()));
        } else {
          output.push_str(&regex::escape("~"));
        }
      }
      ch => output.push_str(&regex::escape(&ch.to_string())),
    }
  }
  output
}

pub fn may_be_wildcard(value: &str) -> bool {
  value.chars().any(|ch| matches!(ch, '*' | '?' | '~'))
}

pub fn may_be_regex(value: &str) -> bool {
  if value.is_empty() || (value.chars().count() == 1 && value != ".") {
    return false;
  }
  value.chars().any(|ch| {
    matches!(
      ch,
      '?' | '*' | '+' | '.' | '[' | ']' | '^' | '$' | '\\' | '<' | '>' | '(' | ')' | '|'
    )
  })
}

pub fn detect_query_search_type(search_type: FormulaSearchType, value: &str) -> QuerySearchType {
  match search_type {
    FormulaSearchType::Wildcard if may_be_wildcard(value) => QuerySearchType::Wildcard,
    FormulaSearchType::Regex if may_be_regex(value) => QuerySearchType::Regex,
    _ => QuerySearchType::Normal,
  }
}

pub fn regex_match(pattern: &str, text: &str, whole_cell: bool) -> Option<bool> {
  let pattern = if whole_cell {
    Cow::Owned(format!("^(?:{pattern})$"))
  } else {
    Cow::Borrowed(pattern)
  };
  RegexBuilder::new(pattern.as_ref())
    .case_insensitive(true)
    .build()
    .ok()
    .map(|regex| regex.is_match(text))
}

pub fn find_text_position(
  needle: &str,
  haystack: &str,
  start: usize,
  case_sensitive: bool,
  search_type: FormulaSearchType,
) -> Result<usize, FindTextError> {
  if start == 0 {
    return Err(FindTextError::Value);
  }
  let skip = start - 1;
  let haystack_tail = haystack.chars().skip(skip).collect::<String>();
  if !case_sensitive {
    match detect_query_search_type(search_type, needle) {
      QuerySearchType::Regex => {
        let regex = RegexBuilder::new(needle).case_insensitive(true).build();
        return regex
          .ok()
          .and_then(|regex| regex.find(&haystack_tail))
          .map(|match_| skip + haystack_tail[..match_.start()].chars().count() + 1)
          .ok_or(FindTextError::Value);
      }
      QuerySearchType::Wildcard => {
        let pattern = wildcard_search_regex_pattern(needle);
        let regex = RegexBuilder::new(&pattern).case_insensitive(true).build();
        return regex
          .ok()
          .and_then(|regex| regex.find(&haystack_tail))
          .map(|match_| skip + haystack_tail[..match_.start()].chars().count() + 1)
          .ok_or(FindTextError::Value);
      }
      QuerySearchType::Normal => {}
    }
  }
  let (haystack_search, needle_search): (Cow<'_, str>, Cow<'_, str>) = if case_sensitive {
    (Cow::Owned(haystack_tail), Cow::Borrowed(needle))
  } else {
    (
      Cow::Owned(haystack_tail.to_lowercase()),
      Cow::Owned(needle.to_lowercase()),
    )
  };
  haystack_search
    .find(needle_search.as_ref())
    .map(|offset| skip + haystack_search[..offset].chars().count() + 1)
    .ok_or(FindTextError::Value)
}

pub fn find_byte_text_position(
  needle: &str,
  haystack: &str,
  start: usize,
  case_sensitive: bool,
  search_type: FormulaSearchType,
) -> Result<usize, FindTextError> {
  let haystack_len = text_byte_len(haystack);
  if start == 0 {
    return Err(FindTextError::Value);
  }
  if start - 1 >= haystack_len {
    return Err(FindTextError::NotAvailable);
  }
  let tail = rightb(haystack, haystack_len - start + 1);
  if case_sensitive {
    return tail
      .find(needle)
      .map(|offset| start + text_byte_len(&tail[..offset]))
      .ok_or(FindTextError::Value);
  }
  match detect_query_search_type(search_type, needle) {
    QuerySearchType::Regex => RegexBuilder::new(needle).case_insensitive(true).build(),
    QuerySearchType::Wildcard => RegexBuilder::new(&wildcard_search_regex_pattern(needle))
      .case_insensitive(true)
      .build(),
    QuerySearchType::Normal => RegexBuilder::new(&regex::escape(needle))
      .case_insensitive(true)
      .build(),
  }
  .ok()
  .and_then(|regex| regex.find(&tail))
  .map(|match_| start + text_byte_len(&tail[..match_.start()]))
  .ok_or(FindTextError::Value)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn detects_formula_search_type_like_core() {
    assert_eq!(
      detect_query_search_type(FormulaSearchType::Wildcard, "a~b"),
      QuerySearchType::Wildcard
    );
    assert_eq!(
      detect_query_search_type(FormulaSearchType::Regex, "."),
      QuerySearchType::Regex
    );
    assert_eq!(
      detect_query_search_type(FormulaSearchType::Regex, "a+b"),
      QuerySearchType::Regex
    );
  }

  #[test]
  fn matches_wildcards_with_tilde_escape() {
    assert!(wildcard_match("a~*b", "a*b"));
    assert!(wildcard_match("a?c", "abc"));
    assert!(!wildcard_match("a~?c", "abc"));
  }

  #[test]
  fn finds_text_positions_with_character_and_byte_rules() {
    assert_eq!(
      find_text_position("B", "aBc", 1, false, FormulaSearchType::Normal),
      Ok(2)
    );
    assert_eq!(
      find_text_position("b", "aBc", 1, true, FormulaSearchType::Normal),
      Err(FindTextError::Value)
    );
    assert_eq!(
      find_byte_text_position("b", "A中b", 1, false, FormulaSearchType::Normal),
      Ok(4)
    );
  }
}
