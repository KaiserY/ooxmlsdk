use super::{Script, UnicodeScript, WordprocessingLineBreakOptions};

fn western_alphanumeric(character: char) -> bool {
  character.script() == Script::Latin || character.is_ascii_digit()
}

fn starts_western_closing_quote(text: &str) -> bool {
  let mut characters = text.chars();
  matches!(characters.next(), Some('”' | '’'))
    && characters.next().is_some_and(western_alphanumeric)
}

pub(super) fn forbids_break(
  before: &str,
  after: &str,
  options: WordprocessingLineBreakOptions,
) -> bool {
  if options.compatibility_mode >= 15 && !options.kashida {
    // B2 normally offers both sides of an em dash. With a Western closing
    // quote Word keeps that dash and quoted word attached to the prefix.
    if (before.ends_with(['-', '–', '—', '‐']) && starts_western_closing_quote(after))
      || (before.chars().next_back().is_some_and(western_alphanumeric)
        && after
          .strip_prefix('—')
          .is_some_and(starts_western_closing_quote))
    {
      return true;
    }
  }
  options.compatibility_mode < 14
    && before
      .strip_suffix('‐')
      .and_then(|text| text.chars().next_back())
      .is_some_and(western_alphanumeric)
    && after
      .trim_start_matches(['"', '\'', '“', '‘', '”', '’', '«', '»', '„'])
      .chars()
      .next()
      .is_some_and(western_alphanumeric)
}

pub(super) fn legacy_en_dash_quote_opportunity(after_dash: &str, mode: u16) -> bool {
  // Native Word2010 controls already break an en dash before a straight
  // double quote with compressPunctuation. Curly closing quotes still join.
  mode == 14
    && after_dash
      .strip_prefix('"')
      .and_then(|text| text.chars().next())
      .is_some_and(western_alphanumeric)
}

/// Word's Western dash/quote opportunities, above the generic UAX14 QU rule.
/// Native mode12/14/15, punctuation-setting and colored-run controls establish
/// the separate opening/closing roles; this does not change glyph measurements.
pub(super) fn breaks(
  text: &str,
  options: WordprocessingLineBreakOptions,
) -> impl Iterator<Item = usize> + '_ {
  let mut previous = None;
  let mut before_previous = None;
  text.char_indices().filter_map(move |(index, character)| {
    let preceding = previous;
    let before = before_previous;
    before_previous = preceding.map(|(_, ch)| ch);
    previous = Some((index, character));
    let (dash_index, dash) = preceding?;
    if !matches!(
      character,
      '"' | '\'' | '“' | '‘' | '”' | '’' | '«' | '»' | '„'
    ) || !text[index + character.len_utf8()..]
      .chars()
      .next()
      .is_some_and(western_alphanumeric)
    {
      return None;
    }
    if matches!(character, '”' | '’') {
      if options.compatibility_mode >= 15 && !options.kashida {
        // Modern closing punctuation keeps the final hyphen with its word.
        // A double/triple ASCII dash can break before that final pair; a
        // single dash does not acquire a break before itself.
        return (dash == '-' && before == Some('-')).then_some(dash_index);
      }
      if dash == '‐' {
        return None;
      }
    }
    (matches!(dash, '-' | '–' | '—') || (dash == '‐' && options.compatibility_mode >= 14))
      .then_some(index)
  })
}

pub(super) fn legacy_closing_prefix(
  previous: Option<&str>,
  text: &str,
  options: WordprocessingLineBreakOptions,
) -> bool {
  let mut characters = text.chars();
  options.compatibility_mode < 15
    && matches!(characters.next(), Some('”' | '’'))
    && characters.next().is_some_and(western_alphanumeric)
    && previous.is_some_and(|text| text.ends_with(['-', '–', '—']))
}

#[cfg(test)]
mod tests {
  use super::super::{
    InlineCursor, InlineItem, TextMetrics, TextRun, TextStyle, WordprocessingLineBreakOptions,
    cross_run_unbreakable_continuation_width_for_break_options, text_segments_for_compatibility,
  };

  #[test]
  fn word_dash_quote_cross_run_fit_uses_the_same_boundary() {
    let inlines = ["birth", "--", "\"Married, "].map(|text| {
      InlineItem::Text(TextRun {
        text: text.to_owned(),
        style: TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      })
    });
    let mut metrics = TextMetrics::new();
    for mode in [12, 14, 15] {
      for compress in [false, true] {
        let options = WordprocessingLineBreakOptions {
          compatibility_mode: mode,
          compress_punctuation: compress,
          strict_japanese: false,
          fullwidth_period_joins_han: true,
          kashida: false,
        };
        let legacy_connected = mode < 15 && compress;
        let continuation = cross_run_unbreakable_continuation_width_for_break_options(
          &inlines,
          0,
          "birth",
          options,
          None,
          &mut metrics,
        )
        .expect("the ordinary dash remains with its preceding word");
        assert_eq!(
          continuation.source_end,
          if legacy_connected {
            InlineCursor {
              inline_index: 2,
              text_offset: "\"Married, ".len(),
            }
          } else {
            InlineCursor {
              inline_index: 1,
              text_offset: 2,
            }
          }
        );
        assert_eq!(
          cross_run_unbreakable_continuation_width_for_break_options(
            &inlines,
            1,
            "--",
            options,
            None,
            &mut metrics,
          )
          .is_some(),
          legacy_connected,
          "mode{mode} compress{compress}"
        );
      }
    }
  }

  #[test]
  fn word_dash_quote_opening_roles_keep_legacy_compression_separate() {
    for (dash, quote) in [
      ("-", '"'),
      ("--", '"'),
      ("---", '"'),
      ("–", '"'),
      ("—", '"'),
      ("--", '\''),
      ("--", '“'),
      ("--", '‘'),
      ("--", '«'),
      ("-", '«'),
      ("--", '»'),
      ("-", '»'),
      ("-", '„'),
    ] {
      for mode in [12, 14, 15] {
        for compress in [false, true] {
          let source = format!("Alpha birth{dash}{quote}Married, Omega");
          let segments = text_segments_for_compatibility(&source, mode, compress);
          assert_eq!(segments.concat(), source);
          let legacy_connected =
            mode < 15 && compress && !(mode == 14 && dash == "–" && quote == '"');
          let expected = if legacy_connected {
            vec![
              "Alpha ".to_string(),
              format!("birth{dash}{quote}Married, "),
              "Omega".into(),
            ]
          } else {
            vec![
              "Alpha ".to_string(),
              format!("birth{dash}"),
              format!("{quote}Married, "),
              "Omega".into(),
            ]
          };
          assert_eq!(segments, expected, "mode{mode} compress{compress} {source}");
        }
      }
    }
  }

  #[test]
  fn word_dash_quote_modern_closing_roles_keep_the_last_dash_with_the_quote() {
    for quote in ['”', '’'] {
      for (dash, prefix, suffix) in [
        ("-", "birth-", ""),
        ("--", "birth-", "-"),
        ("---", "birth--", "-"),
      ] {
        for mode in [12, 14, 15] {
          for compress in [false, true] {
            let source = format!("Alpha birth{dash}{quote}Married, Omega");
            let segments = text_segments_for_compatibility(&source, mode, compress);
            assert_eq!(segments.concat(), source);
            let connected = (mode < 15 && compress) || (mode >= 15 && dash == "-");
            let expected = if connected {
              vec![
                "Alpha ".to_string(),
                format!("birth{dash}{quote}Married, "),
                "Omega".into(),
              ]
            } else if mode >= 15 {
              vec![
                "Alpha ".to_string(),
                prefix.to_owned(),
                format!("{suffix}{quote}Married, "),
                "Omega".into(),
              ]
            } else {
              vec![
                "Alpha ".to_string(),
                format!("birth{dash}"),
                format!("{quote}Married, "),
                "Omega".into(),
              ]
            };
            assert_eq!(segments, expected, "mode{mode} compress{compress} {source}");
          }
        }
      }
    }
  }

  #[test]
  fn word_dash_quote_unicode_hyphen_has_its_own_versioned_boundary() {
    for mode in [12, 14, 15] {
      for compress in [false, true] {
        for quote in ["", "\""] {
          let source = format!("Alpha birth‐{quote}Married, Omega");
          let actual = text_segments_for_compatibility(&source, mode, compress);
          let expected = if mode < 14 {
            vec![
              "Alpha ".into(),
              format!("birth‐{quote}Married, "),
              "Omega".into(),
            ]
          } else {
            vec![
              "Alpha ".into(),
              "birth‐".into(),
              format!("{quote}Married, "),
              "Omega".into(),
            ]
          };
          assert_eq!(actual, expected, "mode{mode} compress{compress} {source}");
        }
        for token in [
          "birth‐”Married,",
          "birth‐’Married,",
          "birth‑\"Married,",
          "birth−\"Married,",
        ] {
          assert_eq!(
            text_segments_for_compatibility(&format!("Alpha {token} Omega"), mode, compress),
            ["Alpha ".to_string(), format!("{token} "), "Omega".into()]
          );
        }
      }
    }
  }

  #[test]
  fn word_dash_quote_modern_em_dash_keeps_its_closing_word_connected() {
    for dash in ['–', '—'] {
      for quote in ['”', '’'] {
        for mode in [12, 14, 15] {
          for compress in [false, true] {
            let source = format!("Alpha birth{dash}{quote}Married, Omega");
            let expected = if mode >= 15 || compress {
              vec![
                "Alpha ".to_string(),
                format!("birth{dash}{quote}Married, "),
                "Omega".into(),
              ]
            } else {
              vec![
                "Alpha ".to_string(),
                format!("birth{dash}"),
                format!("{quote}Married, "),
                "Omega".into(),
              ]
            };
            assert_eq!(
              text_segments_for_compatibility(&source, mode, compress),
              expected,
              "mode{mode} compress{compress} {source}"
            );
          }
        }
      }
    }
  }

  #[test]
  fn word_dash_quote_followed_by_a_space_keeps_its_closing_role() {
    for mode in [12, 14, 15] {
      for compress in [false, true] {
        assert_eq!(
          text_segments_for_compatibility("Alpha birth--\" Married, Omega", mode, compress),
          ["Alpha ", "birth--\" ", "Married, ", "Omega"]
        );
      }
    }
  }
}
