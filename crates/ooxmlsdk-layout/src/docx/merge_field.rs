//! Unmerged field refresh and the independently retained result-format units.

use std::{borrow::Cow, ops::Range};

use super::{
  InlineItem, TextStyle, field_instruction_tokens, push_resolved_field_text, run_display_text,
};

pub(super) fn refresh(
  instruction: &str,
  cache: &[InlineItem],
  code_style: &TextStyle,
  hyperlink_url: Option<&str>,
) -> Option<Vec<InlineItem>> {
  let tokens = field_instruction_tokens(instruction);
  let [kind, name, switches @ ..] = tokens.as_slice() else {
    return None;
  };
  if !kind.eq_ignore_ascii_case("MERGEFIELD") || name.is_empty() || name.starts_with('\\') {
    return None;
  }
  let mut name = display_name(name).into_owned();
  let mut merge_format = false;
  let (pairs, remainder) = switches.as_chunks::<2>();
  if !remainder.is_empty() {
    return None;
  }
  for [switch, argument] in pairs {
    if switch != r"\*" {
      return None;
    }
    match argument.to_ascii_lowercase().as_str() {
      "upper" => name = name.to_uppercase(),
      "lower" => name = name.to_lowercase(),
      "mergeformat" => merge_format = true,
      "charformat" if !merge_format => {}
      _ => return None,
    }
  }
  let value = format!("«{name}»");
  let mut result = Vec::new();
  if !merge_format {
    push_piece(&mut result, &value, code_style, hyperlink_url);
    return Some(result);
  }

  let mut cached_text = String::new();
  let mut runs = Vec::new();
  for item in cache {
    let InlineItem::Text(run) = item else {
      return None;
    };
    if run.dynamic_field.is_some() {
      return None;
    }
    if !run.text.is_empty() {
      let start = cached_text.len();
      cached_text.push_str(&run.text);
      runs.push((start..cached_text.len(), &run.style));
    }
  }
  // ECMA-376 §17.16.4.3.3 specifies retention of result run structure.
  // Actual Word placeholder controls distinguish its word/space/punctuation
  // units from absolute character offsets: VALUE maps only to the opening
  // guillemet; «OLD» maps all three units. A missing unit uses code format.
  // Within a corresponding unit, retain old run lengths and extend its last
  // run when the replacement is longer. This is independent of XML run splits.
  let old_units = format_units(&cached_text)?;
  // With no further old units, all text after the opening guillemet uses
  // code format, regardless of the replacement's script/word boundaries.
  let new_units = if old_units.len() <= 1 {
    vec![0..'«'.len_utf8(), '«'.len_utf8()..value.len()]
  } else {
    format_units(&value)?
  };
  let mut first_run = 0;
  for (index, new_unit) in new_units.into_iter().enumerate() {
    let Some(old_unit) = old_units.get(index) else {
      push_piece(&mut result, &value[new_unit], code_style, hyperlink_url);
      continue;
    };
    while runs
      .get(first_run)
      .is_some_and(|(range, _)| range.end <= old_unit.start)
    {
      first_run += 1;
    }
    let mut remaining = &value[new_unit];
    for (range, style) in &runs[first_run..] {
      if range.start >= old_unit.end || remaining.is_empty() {
        break;
      }
      let end = if range.end >= old_unit.end {
        remaining.len()
      } else {
        let count = cached_text[range.start.max(old_unit.start)..range.end]
          .chars()
          .count();
        remaining
          .char_indices()
          .nth(count)
          .map_or(remaining.len(), |(offset, _)| offset)
      };
      push_piece(&mut result, &remaining[..end], style, hyperlink_url);
      remaining = &remaining[end..];
    }
  }
  Some(result)
}

pub(super) fn display_name(name: &str) -> Cow<'_, str> {
  // Actual Word12/15, simple/complex and ordinary/CHARFORMAT/MERGEFORMAT
  // controls retain40UTF16 code units in the unmerged display. The field
  // instruction itself stays intact. A cut surrogate appears as U+FFFD.
  let mut units = 0;
  for (offset, ch) in name.char_indices() {
    let next = units + ch.len_utf16();
    if next > 40 {
      return if units == 40 {
        Cow::Borrowed(&name[..offset])
      } else {
        let mut prefix = name[..offset].to_string();
        prefix.push('\u{fffd}');
        Cow::Owned(prefix)
      };
    }
    units = next;
  }
  Cow::Borrowed(name)
}

fn push_piece(
  result: &mut Vec<InlineItem>,
  text: &str,
  style: &TextStyle,
  hyperlink_url: Option<&str>,
) {
  let mut style = style.clone();
  style.wordprocessingml_mail_merge_placeholder = true;
  style.wordprocessingml_field_group = true;
  let text = run_display_text(text.to_string(), style.clone());
  if let Some(InlineItem::Text(previous)) = result.last_mut()
    && previous.style == style
    && previous.hyperlink_url.as_deref() == hyperlink_url
  {
    previous.text.push_str(&text);
  } else {
    push_resolved_field_text(result, text, style, hyperlink_url);
  }
}

fn format_units(text: &str) -> Option<Vec<Range<usize>>> {
  // Native controls establish this Latin format-transfer grammar, including
  // separate underscore/slash/quote units and joined internal hyphens/decimal
  // points. Do not substitute NUMWORDS or a Unicode grapheme counter: those
  // have different owners. Complex scripts and structural whitespace retain
  // their cache until their result-format boundaries are established.
  if text.contains("  ")
    || text.chars().any(|ch| {
      !(ch.is_ascii() && !ch.is_ascii_control() || matches!(ch, '«' | '»') || latin_letter(ch))
    })
  {
    return None;
  }
  let mut result: Vec<Range<usize>> = Vec::new();
  let mut previous_word = false;
  let mut chars = text.char_indices().peekable();
  while let Some((start, ch)) = chars.next() {
    let word = word_character(ch)
      || ((ch == '-' || ch == '.')
        && previous_word
        && chars.peek().is_some_and(|(_, next)| {
          if ch == '.' {
            ch_before(text, start).is_some_and(|ch| ch.is_ascii_digit()) && next.is_ascii_digit()
          } else {
            word_character(*next)
          }
        }));
    let end = start + ch.len_utf8();
    if word && previous_word {
      result.last_mut()?.end = end;
    } else {
      result.push(start..end);
    }
    previous_word = word;
  }
  Some(result)
}

fn ch_before(text: &str, offset: usize) -> Option<char> {
  text[..offset].chars().next_back()
}

fn latin_letter(ch: char) -> bool {
  matches!(ch as u32, 0xc0..=0x24f) && ch.is_alphabetic()
}

fn word_character(ch: char) -> bool {
  ch.is_ascii_alphanumeric() || latin_letter(ch)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::model::RgbColor;

  fn style(red: u8) -> TextStyle {
    TextStyle {
      color: RgbColor { r: red, g: 0, b: 0 },
      ..TextStyle::default()
    }
  }

  fn cache(parts: &[(&str, u8)]) -> Vec<InlineItem> {
    let mut result = Vec::new();
    for (text, red) in parts {
      push_resolved_field_text(&mut result, text.to_string(), style(*red), None);
    }
    result
  }

  fn observed(name: &str, parts: &[(&str, u8)]) -> Vec<(String, u8)> {
    refresh(
      &format!(r#"MERGEFIELD "{name}" \* MERGEFORMAT"#),
      &cache(parts),
      &style(0),
      None,
    )
    .unwrap()
    .into_iter()
    .map(|item| {
      let InlineItem::Text(run) = item else {
        panic!("text result")
      };
      assert!(run.style.wordprocessingml_mail_merge_placeholder);
      (run.text, run.style.color.r)
    })
    .collect()
  }

  #[test]
  fn native_format_units_include_spaces_quotes_and_punctuation() {
    for (text, expected) in [
      ("«Alpha Beta»", vec!["«", "Alpha", " ", "Beta", "»"]),
      ("2025-10-01", vec!["2025-10-01"]),
      ("A-B", vec!["A-B"]),
      ("1.2", vec!["1.2"]),
      ("A_B", vec!["A", "_", "B"]),
      ("12/34", vec!["12", "/", "34"]),
    ] {
      assert_eq!(
        format_units(text)
          .unwrap()
          .into_iter()
          .map(|range| &text[range])
          .collect::<Vec<_>>(),
        expected
      );
    }
    for unsupported in ["汉字", "one\ttwo", "one\ntwo", "one  two"] {
      assert!(format_units(unsupported).is_none());
    }
  }

  #[test]
  fn native_unmerged_name_capacity_is_utf16_independent_of_format_owner() {
    for length in [38, 39, 40, 41, 42, 45, 55] {
      let name = format!("{}z", "A".repeat(length - 1));
      let expected = name.chars().take(40).collect::<String>();
      assert_eq!(display_name(&name), expected);
    }
    for (name, expected) in [
      ("é".repeat(41), "é".repeat(40)),
      ("汉".repeat(41), "汉".repeat(40)),
      (
        format!("{}😀XYZ", "X".repeat(37)),
        format!("{}😀X", "X".repeat(37)),
      ),
      (
        format!("{}😀YZ", "X".repeat(38)),
        format!("{}😀", "X".repeat(38)),
      ),
      (
        format!("{}😀YZ", "X".repeat(39)),
        format!("{}\u{fffd}", "X".repeat(39)),
      ),
    ] {
      assert_eq!(display_name(&name), expected);
      for suffix in ["", r"\* CHARFORMAT", r"\* MERGEFORMAT"] {
        let result = refresh(
          &format!(r#"MERGEFIELD "{name}" {suffix}"#),
          &cache(&[("VALUE", 1)]),
          &style(0),
          None,
        )
        .unwrap();
        let text = result
          .into_iter()
          .map(|item| {
            let InlineItem::Text(run) = item else {
              panic!("text result")
            };
            run.text
          })
          .collect::<String>();
        assert_eq!(text, format!("«{expected}»"));
      }
    }
  }

  #[test]
  fn native_mergeformat_replays_units_instead_of_character_offsets() {
    for (name, parts, expected) in [
      ("Alpha", vec![("VALUE", 1)], vec![("«", 1), ("Alpha»", 0)]),
      ("Alpha", vec![("«OLD»", 1)], vec![("«Alpha»", 1)]),
      (
        "Alpha",
        vec![("«", 1), ("OLD", 2), ("»", 3)],
        vec![("«", 1), ("Alpha", 2), ("»", 3)],
      ),
      ("Alpha", vec![("VALUE ", 1)], vec![("«Alpha", 1), ("»", 0)]),
      (
        "X Y",
        vec![("AAA ", 1), ("BBB", 2)],
        vec![("«X", 1), (" ", 2), ("Y»", 0)],
      ),
      (
        "X Y",
        vec![("AAA ", 1), ("BBB ", 2), ("CCC", 3)],
        vec![("«X", 1), (" Y", 2), ("»", 3)],
      ),
      (
        "X",
        vec![("long", 1), (" value", 2), (" tail", 3)],
        vec![("«", 1), ("X»", 2)],
      ),
      ("Alpha", vec![("12/34", 1)], vec![("«Alpha»", 1)]),
      ("Alpha", vec![("A_B", 1)], vec![("«Alpha»", 1)]),
      (
        "Alpha",
        vec![("va", 1), ("lu", 2), ("e", 3)],
        vec![("«", 1), ("Alpha»", 0)],
      ),
      ("Alpha", vec![], vec![("«Alpha»", 0)]),
    ] {
      let expected = expected
        .into_iter()
        .map(|(text, red)| (text.to_string(), red))
        .collect::<Vec<_>>();
      assert_eq!(observed(name, &parts), expected, "{name} {parts:?}");
    }
  }

  #[test]
  fn ordinary_case_switches_and_unsupported_switches_keep_distinct_paths() {
    for (suffix, expected) in [
      (r"\* Upper", "«ALPHA»"),
      (r"\* Lower", "«alpha»"),
      (r"\* Upper \* MERGEFORMAT", "«ALPHA»"),
      (r"\* MERGEFORMAT \* Upper", "«ALPHA»"),
    ] {
      let result = refresh(&format!("MERGEFIELD Alpha {suffix}"), &[], &style(0), None).unwrap();
      let InlineItem::Text(run) = &result[0] else {
        panic!("text result")
      };
      assert_eq!(run.text, expected);
    }
    for instruction in [
      r"MERGEFIELD Alpha \b Before",
      r"MERGEFIELD Alpha \# 0.00",
      r"MERGEFIELD Alpha \* Unsupported",
    ] {
      assert!(refresh(instruction, &[], &style(0), None).is_none());
    }
  }
}
