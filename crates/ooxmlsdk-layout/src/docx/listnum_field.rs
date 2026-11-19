//! Inline LISTNUM display fields (ECMA-376 §17.16.5.33, MS-OI29500 §2.1.484).
//! Word computes these without a persisted result, including locked fields.
//! The predefined names are UI resources: English names also work in Chinese,
//! but fall back to the unnamed sequence in the verified Spanish UI controls.

use super::{
  FieldNumberFormat, InlineItem, StylesCatalog, field_instruction_tokens, format_field_number,
};

#[derive(Clone, Copy, Default)]
enum Format {
  #[default]
  Number,
  Outline,
  Legal,
}

struct Spec {
  name: Option<String>,
  level: Option<usize>,
  start: Option<u32>,
}

impl Spec {
  fn parse(instruction: &str) -> Option<Self> {
    // Switches do not require whitespace before the backslash. Keep quoted
    // list names intact instead of changing the shared field tokenizer.
    let mut separated = String::with_capacity(instruction.len());
    let mut quoted = false;
    for ch in instruction.chars() {
      if ch == '"' {
        quoted = !quoted;
      } else if ch == '\\' && !quoted {
        separated.push(' ');
      }
      separated.push(ch);
    }
    let tokens = field_instruction_tokens(&separated);
    if !tokens.first()?.eq_ignore_ascii_case("LISTNUM") {
      return None;
    }
    let mut result = Self {
      name: None,
      level: None,
      start: None,
    };
    let mut index = 1;
    if let Some(name) = tokens.get(index).filter(|token| !token.starts_with('\\')) {
      result.name = Some(name.clone());
      index += 1;
    }
    while index < tokens.len() {
      let switch = tokens[index].strip_prefix('\\')?;
      let value = tokens.get(index + 1)?;
      if switch.eq_ignore_ascii_case("l") {
        result.level = Some(value.parse::<i32>().ok()?.clamp(1, 9) as usize - 1);
      } else if switch.eq_ignore_ascii_case("s") {
        result.start = u32::try_from(value.parse::<i32>().ok()?).ok();
      } else if switch != "*"
        || !["MERGEFORMAT", "CHARFORMAT"]
          .iter()
          .any(|name| value.eq_ignore_ascii_case(name))
      {
        return None;
      }
      index += 2;
    }
    Some(result)
  }
}

#[derive(Default)]
pub(super) struct State {
  values: [Option<u32>; 9],
  level: usize,
  format: Format,
  story_format: Format,
  numbered_context: bool,
  story_numbered_context: bool,
  fields_in_paragraph: bool,
  after_content: bool,
  inline_end: usize,
}

impl State {
  pub(super) fn begin_paragraph(&mut self) {
    self.format = self.story_format;
    self.numbered_context = self.story_numbered_context;
    self.fields_in_paragraph = false;
    self.after_content = false;
    self.inline_end = 0;
  }

  pub(super) fn numbered_paragraph(&mut self) {
    // A bare/custom LISTNUM can use the paragraph's real list definition.
    // Keep its prior cache until that shared counter/format path is modeled;
    // a recognized predefined sequence explicitly starts its own context.
    self.numbered_context = true;
    self.story_numbered_context = true;
  }

  pub(super) fn next(
    &mut self,
    instruction: &str,
    styles: &StylesCatalog,
    inlines: &[InlineItem],
  ) -> Option<String> {
    let spec = Spec::parse(instruction)?;
    let language = styles
      .locales
      .ui_language()
      .unwrap_or("en-US")
      .split('-')
      .next()?;
    if !["en", "zh", "es"]
      .iter()
      .any(|supported| language.eq_ignore_ascii_case(supported))
    {
      return None;
    }
    let content = inlines[self.inline_end.min(inlines.len())..]
      .iter()
      .any(|item| match item {
        InlineItem::Text(run) => !run.text.trim_matches([' ', '\t']).is_empty(),
        InlineItem::BookmarkStart(_)
        | InlineItem::FormWidgetStart(_)
        | InlineItem::FormWidgetEnd(_)
        | InlineItem::LastRenderedPageBreak => false,
        _ => true,
      });
    let leading = !self.after_content && !content;
    let predefined = spec.name.as_deref().and_then(|name| {
      if language.eq_ignore_ascii_case("es") {
        return None;
      }
      match name {
        "NumberDefault" => Some(Format::Number),
        "OutlineDefault" => Some(Format::Outline),
        "LegalDefault" => Some(Format::Legal),
        _ => None,
      }
    });
    if let Some(format) = predefined {
      self.format = format;
      self.numbered_context = false;
    } else {
      if spec.name.as_deref().is_some_and(|name| {
        styles.numbering_template.as_ref().is_some_and(|numbering| {
          numbering
            .abstract_nums
            .values()
            .any(|definition| definition.name.as_deref() == Some(name))
        })
      }) {
        self.numbered_context = true;
        self.story_numbered_context |= leading;
      }
      if self.numbered_context {
        return None;
      }
      if spec.name.is_some() {
        self.format = Format::Number;
      }
    }
    if leading {
      // A leading list field publishes the paragraph's list definition to
      // the story. An inline preset changes only this paragraph's subsequent
      // fields. Word native controls isolate this from level/start switches,
      // field count, whitespace and the paragraph boundary.
      self.story_format = self.format;
    }
    let level = spec.level.unwrap_or_else(|| {
      if self.fields_in_paragraph {
        if self.after_content {
          self.level
        } else {
          (self.level + 1).min(8)
        }
      } else if spec.name.is_some() {
        0
      } else {
        self.level
      }
    });
    self.after_content |= content;
    self.fields_in_paragraph = true;
    self.level = level;
    // Entering a child implicitly realizes its missing ancestors at1. They
    // are real counter values: a later parent field increments to2 even if
    // the first child used a non-Legal format that did not print the parent.
    for ancestor in &mut self.values[..level] {
      ancestor.get_or_insert(1);
    }
    let value = spec
      .start
      .unwrap_or_else(|| self.values[level].unwrap_or(0).saturating_add(1));
    self.values[level] = Some(value);
    self.values[level + 1..].fill(None);
    Some(self.format_value(level, value))
  }

  pub(super) fn commit(&mut self, inline_end: usize) {
    self.inline_end = inline_end;
  }

  fn format_value(&self, level: usize, value: u32) -> String {
    use FieldNumberFormat::*;
    if matches!(self.format, Format::Legal) {
      let mut text = String::new();
      for &ancestor in &self.values[..level] {
        text.push_str(&ancestor.unwrap_or(1).to_string());
        text.push('.');
      }
      text.push_str(&value.to_string());
      text.push('.');
      return text;
    }
    let (format, prefix, suffix) = match self.format {
      Format::Number => (
        [Decimal, LowerLetter, LowerRoman][level % 3],
        if (3..6).contains(&level) { "(" } else { "" },
        if level < 6 { ")" } else { "." },
      ),
      Format::Outline => match level {
        0 => (UpperRoman, "", "."),
        1 => (UpperLetter, "", "."),
        2 => (Decimal, "", "."),
        3 => (LowerLetter, "", ")"),
        4 => (Decimal, "(", ")"),
        5 | 7 => (LowerLetter, "(", ")"),
        _ => (LowerRoman, "(", ")"),
      },
      Format::Legal => unreachable!(),
    };
    // Word's inline list portions have no Roman/alphabetic digit for zero;
    // paragraph numbering's existing formatter deliberately prints "0".
    let number = if value == 0 && !matches!(format, Decimal) {
      String::new()
    } else {
      format_field_number(value as usize, format)
    };
    format!("{prefix}{number}{suffix}")
  }
}

#[cfg(test)]
mod tests {
  use super::super::*;

  fn field(code: &str, cache: Option<&str>, locked: bool, closed: bool) -> String {
    let result = cache
      .map(|text| {
        format!(r#"<w:r><w:fldChar w:fldCharType="separate"/></w:r><w:r><w:t>{text}</w:t></w:r>"#)
      })
      .unwrap_or_default();
    format!(
      r#"<w:r><w:fldChar w:fldCharType="begin" w:fldLock="{locked}"/></w:r><w:r><w:instrText xml:space="preserve"> {code} </w:instrText></w:r>{result}{}"#,
      if closed {
        r#"<w:r><w:fldChar w:fldCharType="end"/></w:r>"#
      } else {
        ""
      }
    )
  }

  fn story(paragraphs: &[String], language: &str) -> Vec<String> {
    let xml = format!(
      r#"<w:body xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">{}</w:body>"#,
      paragraphs
        .iter()
        .map(|p| format!("<w:p>{p}</w:p>"))
        .collect::<String>()
    );
    let body = w::Body::from_bytes(xml.as_bytes()).unwrap();
    let styles = StylesCatalog {
      locales: OfficeLocaleContext::new(Some(language), None, None),
      ..Default::default()
    };
    let sections = body_sections(
      &body,
      BodySectionEnv {
        styles: &styles,
        numbering: &mut NumberingCatalog::default(),
        images: &ImageCatalog::default(),
        alt_chunks: &AltChunkCatalog::default(),
        hyperlinks: &HyperlinkCatalog::default(),
        custom_xml_bindings: &CustomXmlBindings::default(),
        form_widget_ids: &mut FormWidgetIdAllocator::default(),
        no_column_balance: false,
        fixed_html_paragraph_auto_spacing: false,
      },
    );
    sections
      .iter()
      .flat_map(|section| &section.blocks)
      .filter_map(|block| {
        if let Block::Paragraph(p) = block {
          Some(
            p.inlines
              .iter()
              .filter_map(|i| {
                if let InlineItem::Text(run) = i {
                  Some(run.text.as_str())
                } else {
                  None
                }
              })
              .collect(),
          )
        } else {
          None
        }
      })
      .collect()
  }

  #[test]
  fn listnum_missing_cache_and_locked_cache_share_the_story_counter() {
    let codes = [
      (r"LISTNUM OutlineDefault\l 3", None, false),
      (r"LISTNUM OutlineDefault \l 3", Some("STALE"), false),
      (r"LISTNUM OutlineDefault \l 3", Some("LOCKED"), true),
      (r"LISTNUM OutlineDefault \l 3", None, false),
    ];
    let paragraphs = codes
      .iter()
      .map(|&(code, cache, locked)| field(code, cache, locked, true))
      .collect::<Vec<_>>();
    assert_eq!(story(&paragraphs, "es-MX"), ["i)", "ii)", "iii)", "iv)"]);
    assert_eq!(story(&paragraphs, "en-US"), ["1.", "2.", "3.", "4."]);
    // A second imported Word story has independent numbering.
    assert_eq!(story(&paragraphs[..1], "es-MX"), ["i)"]);
  }

  #[test]
  fn listnum_native_nine_level_predefined_formats_and_ui_fallback() {
    let number = ["1)", "a)", "i)", "(1)", "(a)", "(i)", "1.", "a.", "i."];
    let outline = ["I.", "A.", "1.", "a)", "(1)", "(a)", "(i)", "(a)", "(i)"];
    for (name, expected) in [("NumberDefault", number), ("OutlineDefault", outline)] {
      for joined in [false, true] {
        let paragraphs = (1..=9)
          .map(|level| {
            field(
              &format!(
                "LISTNUM {name}{}\\l {level} \\s 1",
                if joined { "" } else { " " }
              ),
              None,
              false,
              true,
            )
          })
          .collect::<Vec<_>>();
        for language in ["en-US", "zh-CN"] {
          assert_eq!(story(&paragraphs, language), expected);
        }
        assert_eq!(story(&paragraphs, "es-MX"), number);
      }
    }
  }

  #[test]
  fn listnum_predefined_formats_share_counters_and_restart_descendants() {
    let codes = [
      r"LISTNUM NumberDefault \l 1 \s 2",
      r"LISTNUM OutlineDefault \l 1",
      r"LISTNUM LegalDefault \l 3",
      r"LISTNUM LegalDefault \l 3",
      r"LISTNUM NumberDefault \l 1",
      r"LISTNUM NumberDefault \l 3",
      r"LISTNUM NumberDefault \l 3 \s 0",
      r"LISTNUM NumberDefault \l 3",
    ];
    let paragraphs = codes.map(|code| field(code, None, false, true));
    assert_eq!(
      story(&paragraphs, "en-US"),
      ["2)", "III.", "3.1.1.", "3.1.2.", "4)", "i)", ")", "i)"]
    );
  }

  #[test]
  fn listnum_implicit_levels_distinguish_leading_fields_and_body_text() {
    let bare = field("LISTNUM", None, false, true);
    let paragraphs = [
      bare.clone(),
      bare.clone(),
      bare.repeat(3),
      format!(
        r#"<w:r><w:t xml:space="preserve">Text </w:t></w:r>{bare}<w:r><w:t xml:space="preserve"> text </w:t></w:r>{bare}"#
      ),
      field(r"LISTNUM NumberDefault \l 3 \s 8", None, false, true),
      bare,
      field("LISTNUM NumberDefault", None, false, true),
    ];
    assert_eq!(
      story(&paragraphs, "es-MX"),
      [
        "1)",
        "2)",
        "3)a)i)",
        "Text ii) text iii)",
        "viii)",
        "ix)",
        "4)"
      ]
    );
  }

  #[test]
  fn listnum_zero_is_distinct_from_an_uninitialized_ancestor() {
    let codes = [
      r"LISTNUM NumberDefault \l 3 \s 4",
      r"LISTNUM NumberDefault \l 3 \s -1",
      r"LISTNUM NumberDefault \l 3",
      r"LISTNUM NumberDefault \l 1 \s 0",
      r"LISTNUM LegalDefault \l 2 \s 1",
      r"LISTNUM LegalDefault \l 2",
      r"LISTNUM NumberDefault \l 2 \s 0",
      r"LISTNUM NumberDefault \l 2",
    ];
    let paragraphs = codes.map(|code| field(code, None, false, true));
    assert_eq!(
      story(&paragraphs, "en-US"),
      ["iv)", "v)", "vi)", "0)", "0.1.", "0.2.", ")", "a)"]
    );
  }

  #[test]
  fn listnum_child_fields_initialize_real_parent_counters_in_all_formats() {
    let codes = [
      r"LISTNUM OutlineDefault \l 3 \s 5",
      r"LISTNUM \l 1",
      r"LISTNUM OutlineDefault \l 3 \s 5",
      r"LISTNUM \l 1",
      r"LISTNUM OutlineDefault \l 3 \s 5",
      r"LISTNUM \l 2",
      r"LISTNUM OutlineDefault \l 3 \s 5",
      r"LISTNUM \l 2",
    ];
    let paragraphs = codes.map(|code| field(code, None, false, true));
    assert_eq!(
      story(&paragraphs, "en-US"),
      ["5.", "II.", "5.", "III.", "5.", "B.", "5.", "C."]
    );
  }

  #[test]
  fn listnum_only_leading_presets_publish_the_next_paragraph_definition() {
    for (name, seed, continuation) in [
      ("OutlineDefault", "1.", "2."),
      ("LegalDefault", "1.1.1.", "1.1.2."),
    ] {
      for prefix in ["", " ", "Pre "] {
        let paragraphs = [
          field(r"LISTNUM NumberDefault \l 1 \s 1", None, false, true),
          format!(
            r#"<w:r><w:t xml:space="preserve">{prefix}</w:t></w:r>{}"#,
            field(&format!(r"LISTNUM {name} \l 3 \s 1"), None, false, true)
          ),
          field(r"LISTNUM \l 3", None, false, true),
        ];
        assert_eq!(
          story(&paragraphs, "en-US"),
          [
            "1)".to_owned(),
            format!("{prefix}{seed}"),
            if prefix == "Pre " {
              "ii)"
            } else {
              continuation
            }
            .to_owned()
          ]
        );
      }
    }
  }

  #[test]
  fn listnum_adjacent_leading_fields_and_inline_fields_have_separate_contexts() {
    for (separator, next) in [("", "1.1.3."), (" text ", "3.")] {
      let paragraphs = [
        field(r"LISTNUM NumberDefault \l 1 \s 1", None, false, true),
        format!(
          r#"{}<w:r><w:t xml:space="preserve">{separator}</w:t></w:r>{}"#,
          field(r"LISTNUM OutlineDefault \l 3 \s 1", None, false, true),
          field(r"LISTNUM LegalDefault \l 3", None, false, true)
        ),
        field(r"LISTNUM \l 3", None, false, true),
      ];
      assert_eq!(
        story(&paragraphs, "en-US"),
        [
          "1)".to_owned(),
          format!("1.{separator}1.1.2."),
          next.to_owned()
        ]
      );
    }
  }

  #[test]
  fn listnum_unknown_authored_name_selects_the_default_format() {
    let paragraphs = [
      field(r"LISTNUM LegalDefault \l 3 \s 1", None, false, true),
      field(r"LISTNUM UnknownList \l 3", None, false, true),
      field(r"LISTNUM \l 3", None, false, true),
    ];
    assert_eq!(story(&paragraphs, "en-US"), ["1.1.1.", "ii)", "iii)"]);
  }

  #[test]
  fn listnum_nested_fields_do_not_display_or_advance_the_counter() {
    let mut nested = field("QUOTE", Some("before"), false, false);
    nested.push_str(&field(
      r"LISTNUM NumberDefault \l 3",
      Some("NESTED"),
      false,
      true,
    ));
    nested.push_str(r#"<w:r><w:t>after</w:t><w:fldChar w:fldCharType="end"/></w:r>"#);
    let paragraphs = [
      field(r"LISTNUM NumberDefault \l 3", None, false, true),
      nested,
      field(r"LISTNUM NumberDefault \l 3", None, false, true),
    ];
    assert_eq!(story(&paragraphs, "en-US"), ["i)", "beforeafter", "ii)"]);
  }

  #[test]
  fn listnum_unclosed_and_unsupported_field_grammar_keep_the_cache() {
    for code in [
      r"LISTNUM \l broken",
      r"LISTNUM \l 3 \unknown 1",
      r"LISTNUM \l 3 \* ROMAN",
    ] {
      assert_eq!(
        story(&[field(code, Some("CACHE"), false, true)], "en-US"),
        ["CACHE"]
      );
    }
    assert_eq!(
      story(
        &[field(r"LISTNUM \l 3", Some("CACHE"), false, false)],
        "en-US"
      ),
      ["CACHE"]
    );
  }

  #[test]
  fn listnum_custom_and_real_paragraph_lists_keep_their_existing_counter_owner() {
    let mut styles = StylesCatalog::default();
    let mut numbering = NumberingCatalog::default();
    numbering.abstract_nums.insert(
      1,
      AbstractNumbering {
        name: Some("Custom".to_owned()),
        ..Default::default()
      },
    );
    styles.numbering_template = Some(numbering);
    let mut state = super::State::default();
    assert!(state.next(r"LISTNUM Custom \l 3", &styles, &[]).is_none());
    assert!(state.next(r"LISTNUM \l 3", &styles, &[]).is_none());
    assert_eq!(
      state
        .next(r"LISTNUM NumberDefault \l 3", &styles, &[])
        .as_deref(),
      Some("i)")
    );
    state.numbered_paragraph();
    assert!(state.next(r"LISTNUM \l 3", &styles, &[]).is_none());
    assert_eq!(
      state
        .next(r"LISTNUM NumberDefault \l 3", &styles, &[])
        .as_deref(),
      Some("ii)")
    );
  }
}
