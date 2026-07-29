use std::borrow::Cow;

use super::TextStyle;

pub(super) const SOFT_HYPHEN: char = '\u{00ad}';

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CandidateKind {
  Explicit,
  Automatic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Candidate {
  /// Source byte offset immediately before the selected break.
  pub break_before: usize,
  /// Source byte offset where text resumes on the following line.
  ///
  /// These offsets differ for an explicit soft hyphen, which is consumed by
  /// the break. Keeping both values also leaves room for richer dictionary
  /// providers whose replacement/cut semantics are not a simple insertion.
  pub resume_at: usize,
  pub kind: CandidateKind,
}

pub(super) fn visible_text(text: &str) -> Cow<'_, str> {
  if text.contains(SOFT_HYPHEN) {
    Cow::Owned(
      text
        .chars()
        .filter(|character| *character != SOFT_HYPHEN)
        .collect(),
    )
  } else {
    Cow::Borrowed(text)
  }
}

pub(super) fn candidates(
  text: &str,
  style: &TextStyle,
  automatic: bool,
  do_not_hyphenate_caps: bool,
) -> Vec<Candidate> {
  let mut output = explicit_candidates(text);
  // An authored soft hyphen is the authoritative discretionary opportunity
  // for its source word. UAX #14 says it overrides automatic hyphenation for
  // that word, not unrelated words in the same text run.
  if automatic {
    output.extend(automatic_candidates(text, style, do_not_hyphenate_caps));
  }
  output.sort_unstable_by_key(|candidate| (candidate.break_before, candidate.resume_at));
  output.dedup();
  output
}

pub(super) fn is_automatic_break_at_text_boundary(
  left: &str,
  right: &str,
  style: &TextStyle,
  do_not_hyphenate_caps: bool,
) -> bool {
  let mut left_word_start = left.len();
  for (index, character) in left.char_indices().rev() {
    if !character.is_alphabetic() {
      break;
    }
    left_word_start = index;
  }
  let right_word_end = right
    .char_indices()
    .find_map(|(index, character)| (!character.is_alphabetic()).then_some(index))
    .unwrap_or(right.len());
  let left_word = &left[left_word_start..];
  let right_word = &right[..right_word_end];
  if left_word.is_empty() || right_word.is_empty() {
    return false;
  }

  let mut word = String::with_capacity(left_word.len() + right_word.len());
  word.push_str(left_word);
  word.push_str(right_word);
  automatic_candidates(&word, style, do_not_hyphenate_caps)
    .into_iter()
    .any(|candidate| candidate.break_before == left_word.len())
}

fn explicit_candidates(text: &str) -> Vec<Candidate> {
  text
    .char_indices()
    .filter_map(|(index, character)| {
      (character == SOFT_HYPHEN).then_some(Candidate {
        break_before: index,
        resume_at: index + character.len_utf8(),
        kind: CandidateKind::Explicit,
      })
    })
    .collect()
}

fn automatic_candidates(
  text: &str,
  style: &TextStyle,
  do_not_hyphenate_caps: bool,
) -> Vec<Candidate> {
  let language = if style.complex_script == Some(true) || style.right_to_left == Some(true) {
    style.bidi_language.as_deref().or(style.language.as_deref())
  } else {
    style
      .language
      .as_deref()
      .or(style.east_asia_language.as_deref())
  };
  let Some(language) = language.and_then(hypher_language) else {
    return Vec::new();
  };
  let mut output = Vec::new();
  for (start, end) in hyphenatable_word_ranges(text) {
    let word = &text[start..end];
    if word.contains(SOFT_HYPHEN) {
      continue;
    }
    if do_not_hyphenate_caps && is_all_capitals(word) {
      continue;
    }
    for offset in HypherProvider::candidate_offsets(word, language) {
      output.push(Candidate {
        break_before: start + offset,
        resume_at: start + offset,
        kind: CandidateKind::Automatic,
      });
    }
  }
  output
}

fn hyphenatable_word_ranges(text: &str) -> Vec<(usize, usize)> {
  let mut ranges = Vec::new();
  let mut start = None;
  for (index, character) in text.char_indices() {
    if character.is_alphabetic() || character == SOFT_HYPHEN {
      start.get_or_insert(index);
    } else if let Some(start) = start.take() {
      ranges.push((start, index));
    }
  }
  if let Some(start) = start {
    ranges.push((start, text.len()));
  }
  ranges
}

fn is_all_capitals(word: &str) -> bool {
  let mut saw_cased_character = false;
  for character in word.chars() {
    if character.is_lowercase() {
      return false;
    }
    saw_cased_character |= character.is_uppercase();
  }
  saw_cased_character
}

fn hypher_language(tag: &str) -> Option<hypher::Lang> {
  let mut letters = tag.bytes().take_while(u8::is_ascii_alphabetic);
  let first = letters.next()?.to_ascii_lowercase();
  let second = letters.next()?.to_ascii_lowercase();
  if letters.next().is_some() {
    return None;
  }
  let language = hypher::Lang::from_iso([first, second])?;
  // The current layout candidate represents an inserted visible hyphen-minus.
  // Do not invent that glyph for scripts whose dictionary explicitly says a
  // hyphenation break has no conventional visible marker.
  language
    .hyphenation_character()
    .is_some()
    .then_some(language)
}

/// Internal provider boundary for language-specific discretionary points.
///
/// Hypher supplies compact Liang-pattern dictionaries. Its candidates are
/// simple insertion points; callers intentionally consume the richer
/// `Candidate` representation so a future libhyphen-compatible provider can
/// express replacement and cut rules without changing line layout.
trait AutomaticCandidateProvider {
  fn candidate_offsets(word: &str, language: hypher::Lang) -> Vec<usize>;
}

struct HypherProvider;

impl AutomaticCandidateProvider for HypherProvider {
  fn candidate_offsets(word: &str, language: hypher::Lang) -> Vec<usize> {
    let pieces = hypher::hyphenate(word, language).collect::<Vec<_>>();
    let mut offsets = Vec::with_capacity(pieces.len().saturating_sub(1));
    let mut offset = 0;
    for piece in pieces.iter().take(pieces.len().saturating_sub(1)) {
      offset += piece.len();
      offsets.push(offset);
    }
    offsets
  }
}

#[cfg(test)]
mod tests {
  use super::{
    Candidate, CandidateKind, candidates, hypher_language, is_automatic_break_at_text_boundary,
    visible_text,
  };
  use crate::model::TextStyle;
  use std::sync::Arc;

  fn english_style() -> TextStyle {
    TextStyle {
      language: Some(Arc::<str>::from("en-US")),
      ..TextStyle::default()
    }
  }

  #[test]
  fn provider_selects_language_from_the_effective_bcp47_tag() {
    assert_eq!(hypher_language("en-US"), Some(hypher::Lang::English));
    assert_eq!(hypher_language("HU_hu"), Some(hypher::Lang::Hungarian));
    assert_eq!(hypher_language("hi-IN"), None);
    assert_eq!(hypher_language("und"), None);
    assert_eq!(hypher_language("x-private"), None);
  }

  #[test]
  fn english_dictionary_candidates_remain_discretionary() {
    assert_eq!(
      candidates("extensive", &english_style(), true, false),
      vec![
        Candidate {
          break_before: 2,
          resume_at: 2,
          kind: CandidateKind::Automatic,
        },
        Candidate {
          break_before: 5,
          resume_at: 5,
          kind: CandidateKind::Automatic,
        },
      ]
    );
    assert_eq!(visible_text("extensive"), "extensive");
  }

  #[test]
  fn explicit_soft_hyphen_is_zero_width_until_selected() {
    assert_eq!(visible_text("at\u{00ad}mosphere"), "atmosphere");
    assert_eq!(
      candidates("at\u{00ad}mosphere", &english_style(), false, false),
      vec![Candidate {
        break_before: 2,
        resume_at: 4,
        kind: CandidateKind::Explicit,
      }]
    );
  }

  #[test]
  fn explicit_soft_hyphen_overrides_only_its_own_word() {
    assert_eq!(
      candidates(
        "at\u{00ad}mosphere, extensive",
        &english_style(),
        true,
        false
      ),
      vec![
        Candidate {
          break_before: 2,
          resume_at: 4,
          kind: CandidateKind::Explicit,
        },
        Candidate {
          break_before: 16,
          resume_at: 16,
          kind: CandidateKind::Automatic,
        },
        Candidate {
          break_before: 19,
          resume_at: 19,
          kind: CandidateKind::Automatic,
        },
      ]
    );
  }

  #[test]
  fn all_caps_setting_suppresses_only_automatic_candidates() {
    assert!(candidates("EXTENSIVE", &english_style(), true, true).is_empty());
    assert_eq!(
      candidates("EX\u{00ad}TENSIVE", &english_style(), true, true),
      vec![Candidate {
        break_before: 2,
        resume_at: 4,
        kind: CandidateKind::Explicit,
      }]
    );
  }

  #[test]
  fn complex_script_run_uses_its_bidi_language() {
    let style = TextStyle {
      language: None,
      bidi_language: Some(Arc::<str>::from("en-US")),
      complex_script: Some(true),
      ..TextStyle::default()
    };
    assert!(!candidates("extensive", &style, true, false).is_empty());
  }

  #[test]
  fn dictionary_candidate_can_cross_a_run_boundary() {
    assert!(is_automatic_break_at_text_boundary(
      "except that it has an at",
      "mosphere. ",
      &english_style(),
      false,
    ));
    assert!(!is_automatic_break_at_text_boundary(
      "except that it has an ",
      "atmosphere. ",
      &english_style(),
      false,
    ));
  }
}
