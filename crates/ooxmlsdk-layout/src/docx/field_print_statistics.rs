//! Word's ordered print transaction over a supported cached main story.
//! Native reordered fields prove that statistics observe prior replacements,
//! while later fields still expose their pre-print cache. A suffix fold keeps
//! this linear in story size instead of rescanning the document per field.

use super::{
  FieldMessage, StylesCatalog, field_datetime, field_instruction_tokens, field_localization,
  field_number_format,
  field_statistics::{CachedField, CachedPiece, Characters},
  format_field_number, refreshed_date_time_field, refreshed_doc_property_field,
  refreshed_scalar_field, refreshed_user_info_field, scalar_field_switches_supported,
};

#[derive(Clone, Copy, Debug)]
struct Words {
  count: usize,
  leading: bool,
  trailing: bool,
  empty: bool,
}

impl Default for Words {
  fn default() -> Self {
    Self {
      count: 0,
      leading: false,
      trailing: false,
      empty: true,
    }
  }
}

impl Words {
  fn text(text: &str) -> Option<Self> {
    // The native default-ignorable counterexamples apply to word boundaries
    // as well. Do not infer their realization from ordinary punctuation.
    Characters::text(text)?;
    let mut value = Self::default();
    for ch in text.chars() {
      // Native Latin boundary controls do not establish CJK, emoji or
      // default-ignorable segmentation. Preserve their NUMWORDS cache.
      if !(ch.is_ascii() || matches!(ch as u32, 0xa0..=0x24f | 0x300..=0x36f | 0x2000..=0x206f)) {
        return None;
      }
      let word = !matches!(
        ch,
        ' ' | '\t' | '\n' | '\r' | '\u{b}' | '\u{a0}' | '\u{2013}' | '\u{2014}'
      );
      if value.empty {
        value.leading = word;
      }
      value.count += usize::from(word && !value.trailing);
      value.trailing = word;
      value.empty = false;
    }
    Some(value)
  }

  fn then(self, next: Self) -> Self {
    if self.empty {
      return next;
    }
    if next.empty {
      return self;
    }
    Self {
      count: self.count + next.count - usize::from(self.trailing && next.leading),
      leading: self.leading,
      trailing: next.trailing,
      empty: false,
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct Summary {
  characters: Option<Characters>,
  words: Option<Words>,
}

impl Default for Summary {
  fn default() -> Self {
    Self {
      characters: Some(Characters::default()),
      words: Some(Words::default()),
    }
  }
}

impl Summary {
  fn text(text: &str) -> Self {
    Self {
      characters: Characters::text(text),
      words: Words::text(text),
    }
  }

  fn then(self, next: Self) -> Self {
    Self {
      characters: self.characters.zip(next.characters).map(|(mut a, b)| {
        a.add(b);
        a
      }),
      words: self.words.zip(next.words).map(|(a, b)| a.then(b)),
    }
  }

  fn paragraph() -> Self {
    Self {
      words: Words::text("\r"),
      ..Self::default()
    }
  }
}

#[derive(Clone, Debug)]
struct Value {
  import_id: u64,
  instruction: String,
  number: Option<usize>,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum StatisticValue {
  Updated(String),
  Cached,
}

#[derive(Clone, Debug, Default)]
pub(super) struct PrintStatistics {
  values: Vec<Value>,
}

impl PrintStatistics {
  pub(super) fn field_value(&self, import_id: u64, instruction: &str) -> Option<StatisticValue> {
    let index = self
      .values
      .binary_search_by_key(&import_id, |value| value.import_id)
      .ok()?;
    let value = self
      .values
      .get(index)
      .filter(|value| value.instruction == instruction)?;
    let tokens = field_instruction_tokens(instruction);
    let switches = if tokens[0].eq_ignore_ascii_case("INFO") {
      &tokens[2..]
    } else {
      &tokens[1..]
    };
    Some(
      match value
        .number
        .filter(|_| scalar_field_switches_supported(switches, true))
      {
        Some(number) => StatisticValue::Updated(format_field_number(
          number,
          field_number_format(switches).unwrap_or_default(),
        )),
        None => StatisticValue::Cached,
      },
    )
  }
}

pub(super) fn collect(xml: &str, styles: &StylesCatalog) -> Option<PrintStatistics> {
  let pieces = super::field_statistics::cached_pieces(xml, styles)?;
  if !pieces.iter().any(|piece| matches!(piece, CachedPiece::Field(field) if statistic_kind(&field.instruction).is_some())) {
    return None;
  }
  let initial = pieces
    .iter()
    .map(|piece| match piece {
      CachedPiece::Text(text) => Summary::text(text),
      CachedPiece::Paragraph => Summary::paragraph(),
      CachedPiece::Field(field) => preprint_result(field, styles).map_or(
        Summary {
          characters: None,
          words: None,
        },
        |text| Summary::text(&text),
      ),
    })
    .collect::<Vec<_>>();
  let mut suffix = vec![Summary::default(); initial.len() + 1];
  for i in (0..initial.len()).rev() {
    suffix[i] = initial[i].then(suffix[i + 1]);
  }
  let mut prefix = Summary::default();
  let mut values = Vec::new();
  for (i, piece) in pieces.iter().enumerate() {
    let CachedPiece::Field(field) = piece else {
      prefix = prefix.then(initial[i]);
      continue;
    };
    let current = prefix.then(suffix[i]);
    let kind = statistic_kind(&field.instruction);
    let number = kind
      .and_then(|words| {
        if words {
          current.words.map(|v| v.count)
        } else {
          current.characters.map(|v| v.without_spaces)
        }
      })
      .filter(|_| {
        let tokens = field_instruction_tokens(&field.instruction);
        let switches = if tokens[0].eq_ignore_ascii_case("INFO") {
          &tokens[2..]
        } else {
          &tokens[1..]
        };
        scalar_field_switches_supported(switches, true)
      });
    if !field.locked
      && styles
        .import_settings
        .refresh_field_at_print(&field.instruction)
      && kind.is_some()
      && let Some(import_id) = field.import_id
    {
      values.push(Value {
        import_id,
        instruction: field.instruction.clone(),
        number,
      });
    }
    let replacement = if field.locked
      || !styles
        .import_settings
        .refresh_field_at_print(&field.instruction)
    {
      Some(initial[i])
    } else if kind.is_some() {
      number.map(|number| {
        Summary::text(&format_field_number(
          number,
          field_number_format(&field_instruction_tokens(&field.instruction)).unwrap_or_default(),
        ))
      })
    } else {
      printed_summary(field, styles)
    };
    prefix = prefix.then(replacement.unwrap_or(Summary {
      characters: None,
      words: None,
    }));
  }
  (!values.is_empty()).then_some(PrintStatistics { values })
}

fn statistic_kind(instruction: &str) -> Option<bool> {
  let tokens = field_instruction_tokens(instruction);
  let name = tokens.first()?;
  let name = if name.eq_ignore_ascii_case("INFO") {
    tokens.get(1)?
  } else {
    name
  };
  if name.eq_ignore_ascii_case("NUMCHARS") {
    Some(false)
  } else if name.eq_ignore_ascii_case("NUMWORDS") {
    Some(true)
  } else {
    None
  }
}

fn preprint_result(field: &CachedField, styles: &StylesCatalog) -> Option<String> {
  let tokens = field_instruction_tokens(&field.instruction);
  let name = tokens.first()?.to_ascii_uppercase();
  if field.locked {
    return Some(field.result.clone());
  }
  match name.as_str() {
    "DATE" | "TIME" | "CREATEDATE" => {
      refreshed_date_time_field(&field.instruction, &field.style, styles)
    }
    "SAVEDATE" => {
      if let Some(date) = styles.import_settings.document_saved_datetime {
        field_datetime::format_date_time_field(&tokens, field.style.language.as_deref(), date)
      } else if styles.has_saved_datetime_property {
        None
      } else {
        missing_property_picture(&tokens)
      }
    }
    "PRINTDATE" if styles.has_printed_datetime_property => None,
    "PRINTDATE" => missing_property_picture(&tokens),
    _ => Some(field.result.clone()),
  }
}

fn printed_summary(field: &CachedField, styles: &StylesCatalog) -> Option<Summary> {
  let tokens = field_instruction_tokens(&field.instruction);
  if tokens
    .first()
    .is_some_and(|name| name.eq_ignore_ascii_case("NUMPAGES"))
    && scalar_field_switches_supported(&tokens[1..], true)
    && field_number_format(&tokens[1..]).unwrap_or_default() == super::FieldNumberFormat::Decimal
  {
    // Pagination can change the number of digits, so its character summary
    // is unknown here. A bare decimal page total remains exactly one word,
    // including zero; do not discard that independently established fact.
    return Some(Summary {
      characters: None,
      words: Words::text("0"),
    });
  }
  printed_result(field, styles).map(|text| Summary::text(&text))
}

fn printed_result(field: &CachedField, styles: &StylesCatalog) -> Option<String> {
  let tokens = field_instruction_tokens(&field.instruction);
  let name = tokens.first()?.to_ascii_uppercase();
  if matches!(name.as_str(), "REF" | "NOTEREF")
    && (tokens.get(1).is_some_and(String::is_empty)
      || (name == "NOTEREF" && scalar_field_switches_supported(&tokens[1..], false)))
  {
    return Some(field_localization::localized_field_message(
      FieldMessage::BookmarkNameNotSpecified,
      styles.locales.ui_language(),
    ));
  }
  if let Some(value) = refreshed_scalar_field(&field.instruction, styles) {
    let (text, _) = value.into_text_and_style(field.style.clone(), styles.locales.ui_language());
    return Some(text);
  }
  if let Some(text) = refreshed_date_time_field(&field.instruction, &field.style, styles)
    .or_else(|| refreshed_doc_property_field(&field.instruction, styles))
    .or_else(|| refreshed_user_info_field(&field.instruction, styles))
  {
    return Some(text);
  }
  match name.as_str() {
    // These display representations are unchanged by the print update.
    "PRIVATE" | "RD" | "TA" | "TC" | "XE" | "SET" | "SKIPIF" | "PRINT" | "GOTOBUTTON"
    | "MACROBUTTON" => Some(field.result.clone()),
    // Pagination, bookmark values, assignments and rich updates need their
    // own snapshot. Do not let an unmodeled preceding update fabricate a count.
    _ => None,
  }
}

fn missing_property_picture(tokens: &[String]) -> Option<String> {
  let picture = tokens
    .windows(2)
    .find(|pair| pair[0].eq_ignore_ascii_case(r"\@"))?
    .get(1)?;
  let mut chars = picture.chars().peekable();
  let mut out = String::new();
  while let Some(ch) = chars.next() {
    if ch == '\'' {
      loop {
        let ch = chars.next()?;
        if ch == '\'' {
          break;
        }
        out.push(ch);
      }
      continue;
    }
    let mut count = 1;
    while chars.next_if_eq(&ch).is_some() {
      count += 1;
    }
    match (ch, count) {
      ('d' | 'D' | 'M', 3..=4) => out.push_str("XXX"),
      ('d' | 'D' | 'M', 1..=2) | ('y' | 'Y', 2 | 4) => out.extend(std::iter::repeat_n('0', count)),
      _ if ch.is_ascii_alphabetic() => return None,
      _ => out.extend(std::iter::repeat_n(ch, count)),
    }
  }
  Some(out)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn native_latin_word_boundaries_compose_across_run_fragments() {
    for (text, expected) in [
      ("ABC", 1),
      ("  A  B  ", 2),
      ("", 0),
      ("   ", 0),
      ("one-two", 1),
      ("one—two", 2),
      ("one–two", 2),
      ("one/two", 1),
      ("one.two", 1),
      ("one & two", 3),
      ("...", 1),
      ("a\u{a0}b", 2),
      ("a\u{2003}b", 1),
      ("a\u{202f}b", 1),
      ("can't", 1),
      ("e\u{301}", 1),
    ] {
      let expected = Words::text(text).filter(|v| v.count == expected).unwrap();
      for boundary in text
        .char_indices()
        .map(|(i, _)| i)
        .chain(std::iter::once(text.len()))
      {
        let actual = Words::text(&text[..boundary])
          .unwrap()
          .then(Words::text(&text[boundary..]).unwrap());
        assert_eq!(actual.count, expected.count, "{text:?} boundary{boundary}");
      }
    }
    assert!(Words::text("A中文B").is_none());
    assert!(Words::text("A\u{200d}B").is_none());
  }

  #[test]
  fn missing_property_clock_masks_match_native_preprint_cache() {
    for (picture, expected) in [
      (r"dddd\ dd\ MMMM\ yyyy", "XXX 00 XXX 0000"),
      (r"MMMM\ d', 'yyyy", "XXX 0, 0000"),
    ] {
      assert_eq!(
        missing_property_picture(&field_instruction_tokens(&format!(
          r#"SAVEDATE \@ "{picture}""#
        )))
        .as_deref(),
        Some(expected)
      );
    }
  }

  #[test]
  fn repeated_statistics_keep_their_own_transaction_position() {
    let field = |instruction: &str, cache: &str| {
      format!(
        r#"<w:p><w:r><w:fldChar w:fldCharType="begin"/></w:r><w:r><w:instrText>{instruction}</w:instrText></w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r><w:r><w:t>{cache}</w:t></w:r><w:r><w:fldChar w:fldCharType="end"/></w:r></w:p>"#
      )
    };
    let xml = format!(
      r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body>{}{}{}<w:p><w:r><w:t>AB C</w:t></w:r></w:p></w:body></w:document>"#,
      field("NUMCHARS", "000"),
      field("MERGESEQ", "X"),
      field("NUMCHARS", "000")
    );
    let styles = StylesCatalog {
      import_settings: super::super::ImportSettings {
        update_fields_at_print: Some(true),
        ..Default::default()
      },
      ..Default::default()
    };
    let values = collect(&xml, &styles).unwrap();
    assert_eq!(
      values.field_value(0, "NUMCHARS"),
      Some(StatisticValue::Updated("10".into()))
    );
    assert_eq!(
      values.field_value(2, "NUMCHARS"),
      Some(StatisticValue::Updated("24".into()))
    );
    assert_eq!(values.field_value(2, "NUMWORDS"), None);
    assert_eq!(values.field_value(1, "NUMCHARS"), None);
  }

  #[test]
  fn an_unmodeled_preceding_update_keeps_the_statistics_cache() {
    let field = |instruction: &str| {
      format!(
        r#"<w:p><w:r><w:fldChar w:fldCharType="begin"/></w:r><w:r><w:instrText>{instruction}</w:instrText></w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r><w:r><w:t>CACHED</w:t></w:r><w:r><w:fldChar w:fldCharType="end"/></w:r></w:p>"#
      )
    };
    let xml = format!(
      r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body>{}{}</w:body></w:document>"#,
      field("INCLUDETEXT Missing"),
      field("NUMCHARS")
    );
    let mut styles = StylesCatalog {
      cached_character_count: Some(999),
      import_settings: super::super::ImportSettings {
        update_fields_at_print: Some(true),
        ..Default::default()
      },
      ..Default::default()
    };
    let values = collect(&xml, &styles).unwrap();
    assert_eq!(
      values.field_value(1, "NUMCHARS"),
      Some(StatisticValue::Cached)
    );
    styles.print_field_statistics = Some(std::sync::Arc::new(values));
    assert!(super::super::refreshed_complex_scalar_field("NUMCHARS", &styles, 1, true).is_none());
  }
}
