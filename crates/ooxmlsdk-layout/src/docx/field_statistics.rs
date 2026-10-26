//! Character statistics for Word's cached main story, before print updates.
//!
//! MS-OI29500/ECMA NUMCHARS describe the statistic; current Word controls show
//! that stale app.xml and the temporary printed field results are not its source.
//! Count a completed cached story and preserve the cache for unsupported content.

use icu_properties::{CodePointMapData, props::GeneralCategory};
use quick_xml::{
  NsReader,
  events::{BytesStart, Event},
  name::ResolveResult,
};

use super::{RunStyleOverrides, StylesCatalog, TextStyle, field_instruction_tokens};

const WORD: &[u8] = b"http://schemas.openxmlformats.org/wordprocessingml/2006/main";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Characters {
  without_spaces: usize,
  with_spaces: usize,
}

impl Characters {
  fn add(&mut self, other: Self) {
    self.without_spaces += other.without_spaces;
    self.with_spaces += other.with_spaces;
  }

  fn text(text: &str) -> Option<Self> {
    let mut result = Self::default();
    for character in text.chars() {
      // Default-ignorable characters depend on Word's text realization (e.g.
      // Latin ZWJ versus emoji ZWJ). Do not substitute a grapheme counter:
      // native Word counts combining accents separately but surrogate pairs once.
      if super::is_unicode_variation_selector(character)
        || (character != '\u{ad}'
          && CodePointMapData::<GeneralCategory>::new().get(character) == GeneralCategory::Format)
      {
        return None;
      }
      result.with_spaces += 1;
      // Word's observed statistic differs from Unicode White_Space: U+2000,
      // U+2003 and U+202F count as characters, whereas U+2005 does not.
      if !matches!(
        character,
        ' ' | '\t' | '\n' | '\r' | '\u{b}' | '\u{a0}' | '\u{2005}' | '\u{3000}'
      ) {
        result.without_spaces += 1;
      }
    }
    Some(result)
  }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum FieldSource {
  Complex,
  Simple,
}

struct Field {
  source: FieldSource,
  instruction: String,
  in_result: bool,
  result: Characters,
}

#[derive(Default)]
struct Story {
  fields: Vec<Field>,
  characters: Characters,
  requested: bool,
  in_body: bool,
  body_complete: bool,
  in_run: bool,
  text_is_instruction: Option<bool>,
  default_hidden: bool,
  paragraph_hidden: bool,
  run_hidden: bool,
}

impl Story {
  fn text(&mut self, text: &str) -> Option<()> {
    let instruction_text = self.text_is_instruction?;
    if instruction_text && self.fields.is_empty() {
      return None;
    }
    if instruction_text
      && let Some(field) = self.fields.last_mut()
      && !field.in_result
    {
      field.instruction.push_str(text);
    } else if !self.run_hidden {
      self.add(Characters::text(text)?)?;
    }
    Some(())
  }

  fn add(&mut self, characters: Characters) -> Option<()> {
    if let Some(field) = self.fields.last_mut() {
      if !field.in_result {
        // Nested computed operands require field-code evaluation, which is
        // distinct from counting the cached visible result of a nested field.
        return None;
      }
      field.result.add(characters);
    } else {
      self.characters.add(characters);
    }
    Some(())
  }

  fn finish_field(&mut self, source: FieldSource) -> Option<()> {
    let field = self.fields.pop()?;
    if field.source != source {
      return None;
    }
    let tokens = field_instruction_tokens(&field.instruction);
    let name = tokens.first()?.to_ascii_uppercase();
    self.requested |= name == "NUMCHARS"
      || (name == "INFO"
        && tokens
          .get(1)
          .is_some_and(|token| token.eq_ignore_ascii_case("NumChars")));
    let characters = match name.as_str() {
      "GOTOBUTTON" | "MACROBUTTON" => Characters::text(button_display_tail(&field.instruction)?)?,
      // These hot display fields retain an internal display marker. Their
      // stale cached text is ignored, including when it is nonempty.
      "PRINT" | "BARCODE" => Characters {
        without_spaces: 1,
        with_spaces: 1,
      },
      // EQ's internal display text, cold fields and assignments need their
      // own representation. Keep the authored NUMCHARS cache for such stories.
      "EQ" | "TC" | "XE" | "PRIVATE" | "ASK" | "SET" => return None,
      _ => field.result,
    };
    self.add(characters)
  }

  fn start(
    &mut self,
    element: &BytesStart<'_>,
    reader: &NsReader<&[u8]>,
    styles: &StylesCatalog,
  ) -> Option<()> {
    match element.local_name().as_ref() {
      b"body" => self.in_body = true,
      _ if !self.in_body => return Some(()),
      b"p" => self.paragraph_hidden = self.default_hidden,
      b"pStyle" => {
        let id = word_attribute(element, b"val", reader)?;
        if styles.paragraph_numbering_reference(Some(&id)).is_some() {
          return None;
        }
        self.paragraph_hidden = styles
          .run_style_with_base(
            Some(&id),
            TextStyle::default(),
            RunStyleOverrides::default(),
          )
          .hidden;
      }
      b"r" => {
        self.in_run = true;
        self.run_hidden = self.paragraph_hidden;
      }
      b"rStyle" if self.in_run => {
        let id = word_attribute(element, b"val", reader)?;
        self.run_hidden = styles
          .character_run_style(
            Some(&id),
            TextStyle {
              hidden: self.paragraph_hidden,
              ..Default::default()
            },
          )
          .hidden;
      }
      b"vanish" if self.in_run => {
        self.run_hidden = !word_attribute(element, b"val", reader)
          .is_some_and(|value| matches!(value.as_str(), "0" | "false" | "off"));
      }
      b"t" => self.text_is_instruction = Some(false),
      b"instrText" => self.text_is_instruction = Some(true),
      b"fldSimple" => self.fields.push(Field {
        source: FieldSource::Simple,
        instruction: word_attribute(element, b"instr", reader)?,
        in_result: true,
        result: Characters::default(),
      }),
      b"fldChar" => match word_attribute(element, b"fldCharType", reader)?.as_str() {
        "begin" => self.fields.push(Field {
          source: FieldSource::Complex,
          instruction: String::new(),
          in_result: false,
          result: Characters::default(),
        }),
        "separate" => {
          let field = self.fields.last_mut()?;
          if field.source != FieldSource::Complex || field.in_result {
            return None;
          }
          field.in_result = true;
        }
        "end" => self.finish_field(FieldSource::Complex)?,
        _ => return None,
      },
      b"tab" => {
        if !self.run_hidden {
          self.add(Characters::text("\t")?)?;
        }
      }
      b"br" | b"cr" => {
        if word_attribute(element, b"type", reader).is_some_and(|value| value != "textWrapping") {
          return None;
        }
        if !self.run_hidden {
          self.add(Characters::text("\u{b}")?)?;
        }
      }
      b"softHyphen" => {} // Word's optional-hyphen control is not counted.
      b"noBreakHyphen" => {
        if !self.run_hidden {
          self.add(Characters::text("-")?)?;
        }
      }
      // These structures can add or suppress story characters beyond w:t.
      b"drawing" | b"pict" | b"object" | b"sym" | b"altChunk" | b"numPr" | b"footnoteReference"
      | b"endnoteReference" | b"del" | b"ins" | b"moveFrom" | b"moveTo" => return None,
      _ => {}
    }
    Some(())
  }

  fn end(&mut self, name: &[u8]) -> Option<()> {
    if !self.in_body {
      return Some(());
    }
    match name {
      b"fldSimple" => self.finish_field(FieldSource::Simple)?,
      b"t" | b"instrText" => self.text_is_instruction = None,
      b"r" => self.in_run = false,
      b"body" => {
        self.in_body = false;
        self.body_complete = true;
      }
      _ => {}
    }
    Some(())
  }
}

fn button_display_tail(instruction: &str) -> Option<&str> {
  let mut remaining = instruction.trim_start();
  for _ in 0..2 {
    let mut quoted = false;
    let end = remaining.char_indices().find_map(|(index, character)| {
      if character == '"' {
        quoted = !quoted;
      }
      (character.is_whitespace() && !quoted).then_some(index)
    })?;
    remaining = remaining[end..].trim_start();
  }
  // Trailing display whitespace participates in Word statistics even though
  // the ordinary painted button label trims it.
  Some(remaining)
}

fn word_attribute(
  element: &BytesStart<'_>,
  name: &[u8],
  reader: &NsReader<&[u8]>,
) -> Option<String> {
  element.attributes().find_map(|attribute| {
    let attribute = attribute.ok()?;
    let (namespace, local) = reader.resolver().resolve_attribute(attribute.key);
    (local.as_ref() == name
      && matches!(namespace, ResolveResult::Bound(uri) if uri.as_ref() == WORD))
    .then(|| {
      attribute
        .decoded_and_normalized_value(quick_xml::XmlVersion::Implicit1_0, reader.decoder())
        .ok()
        .map(|value| value.into_owned())
    })
    .flatten()
  })
}

fn statistics(xml: &str, styles: &StylesCatalog) -> Option<(Characters, bool)> {
  let mut reader = NsReader::from_str(xml);
  reader.config_mut().expand_empty_elements = true;
  let default_hidden = styles
    .run_style_with_base(None, TextStyle::default(), RunStyleOverrides::default())
    .hidden;
  if styles.paragraph_numbering_reference(None).is_some() {
    return None;
  }
  let mut story = Story {
    default_hidden,
    ..Default::default()
  };
  loop {
    match reader.read_resolved_event().ok()? {
      (namespace, Event::Start(element) | Event::Empty(element)) => {
        let is_word = matches!(namespace, ResolveResult::Bound(uri) if uri.as_ref() == WORD);
        if !is_word {
          if story.in_body {
            return None;
          }
          continue;
        }
        // NsReader expansion supplies matching End events for empty elements.
        story.start(&element, &reader, styles)?;
      }
      (namespace, Event::End(element)) => {
        if matches!(namespace, ResolveResult::Bound(uri) if uri.as_ref() == WORD) {
          story.end(element.local_name().as_ref())?;
        }
      }
      (_, Event::Text(text)) if story.text_is_instruction.is_some() => {
        story.text(&text.xml10_content().ok()?)?
      }
      (_, Event::CData(text)) if story.text_is_instruction.is_some() => {
        story.text(&text.xml10_content().ok()?)?
      }
      (_, Event::GeneralRef(reference)) if story.text_is_instruction.is_some() => {
        let reference = format!("&{};", reference.decode().ok()?);
        story.text(&quick_xml::escape::unescape(&reference).ok()?)?;
      }
      (_, Event::Eof) => break,
      _ => {}
    }
  }
  (story.body_complete && story.fields.is_empty()).then_some((story.characters, story.requested))
}

pub(super) fn character_count(xml: &str, styles: &StylesCatalog) -> Option<usize> {
  // Most documents have no fields. This negative check is independent of
  // prefix spelling and field-code fragmentation; it does not guess keywords.
  if !xml.contains("instrText") && !xml.contains("fldSimple") {
    return None;
  }
  let (characters, requested) = statistics(xml, styles)?;
  requested.then_some(characters.without_spaces)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn xml(body: &str) -> String {
    format!(
      r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body>{body}</w:body></w:document>"#
    )
  }

  #[test]
  fn cached_character_statistics_match_native_unicode_and_space_controls() {
    for (text, without, with) in [
      ("ABC", 3, 3),
      ("  A  B  ", 2, 8),
      ("", 0, 0),
      ("   ", 0, 3),
      ("A\u{a0}B", 2, 3),
      ("A\u{202f}B", 3, 3),
      ("A\u{1680}B", 3, 3),
      ("A\u{3000}B", 2, 3),
      ("A\u{ad}B", 3, 3),
      ("A\u{2028}B", 3, 3),
      ("A\u{2029}B", 3, 3),
      ("A😀B", 3, 3),
      ("Ae\u{301}B", 4, 4),
      ("AبَB", 4, 4),
      ("A中文B", 4, 4),
    ] {
      let (actual, requested) = statistics(
        &xml(&format!("<w:p><w:r><w:t>{text}</w:t></w:r></w:p>")),
        &StylesCatalog::default(),
      )
      .unwrap();
      assert_eq!(
        actual,
        Characters {
          without_spaces: without,
          with_spaces: with
        },
        "{text:?}"
      );
      assert!(!requested);
    }
    for code in 0x2000..=0x200a {
      let text = format!("A{}B", char::from_u32(code).unwrap());
      assert_eq!(
        Characters::text(&text),
        Some(Characters {
          without_spaces: if code == 0x2005 { 2 } else { 3 },
          with_spaces: 3
        })
      );
    }
    // These are observed context-sensitive results, not grapheme-count rules.
    for text in [
      "A\u{200b}B",
      "A\u{200c}B",
      "A\u{200d}B",
      "A\u{feff}B",
      "A❤️B",
      "A👩‍💻B",
    ] {
      assert_eq!(Characters::text(text), None);
    }
  }

  #[test]
  fn cached_character_statistics_use_display_fields_instead_of_stale_results() {
    for complex in [false, true] {
      for cache in ["", "CACHE"] {
        for (instruction, without, with) in [
          ("GOTOBUTTON target Line 5 ", 5, 7),
          ("MACROBUTTON NoMacro contacts  ssss ", 12, 15),
          ("PRINT ", 1, 1),
          ("BARCODE ", 1, 1),
          ("REF Missing ", cache.len(), cache.len()),
          ("AUTOTEXT Missing ", cache.len(), cache.len()),
          ("REVNUM ", cache.len(), cache.len()),
          ("PAGE ", cache.len(), cache.len()),
        ] {
          let field = if complex {
            format!(
              r#"<w:r><w:fldChar w:fldCharType="begin"/></w:r><w:r><w:instrText>{instruction}</w:instrText></w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r><w:r><w:t>{cache}</w:t></w:r><w:r><w:fldChar w:fldCharType="end"/></w:r>"#
            )
          } else {
            format!(
              r#"<w:fldSimple w:instr="{instruction}"><w:r><w:t>{cache}</w:t></w:r></w:fldSimple>"#
            )
          };
          let text = xml(&format!("<w:p>{field}</w:p>"));
          assert_eq!(
            statistics(&text, &StylesCatalog::default()).unwrap().0,
            Characters {
              without_spaces: without,
              with_spaces: with
            },
            "{instruction} complex={complex} cache={cache:?}"
          );
        }
      }
    }
  }

  #[test]
  fn cached_character_statistics_handle_controls_hidden_runs_and_entities() {
    for (content, without, with) in [
      ("<w:r><w:t>A</w:t><w:tab/><w:t>B</w:t></w:r>", 2, 3),
      ("<w:r><w:t>A</w:t><w:br/><w:t>B</w:t></w:r>", 2, 3),
      ("<w:r><w:t>A</w:t><w:cr/><w:t>B</w:t></w:r>", 2, 3),
      ("<w:r><w:t>A</w:t><w:softHyphen/><w:t>B</w:t></w:r>", 2, 2),
      (
        "<w:r><w:t>A</w:t><w:noBreakHyphen/><w:t>B</w:t></w:r>",
        3,
        3,
      ),
      (
        "<w:r><w:t>A</w:t></w:r><w:r><w:rPr><w:vanish/></w:rPr><w:t>HIDDEN</w:t></w:r><w:r><w:t>B</w:t></w:r>",
        2,
        2,
      ),
      (
        "<w:r><w:t>A</w:t></w:r><w:r><w:rPr><w:webHidden/></w:rPr><w:t>HIDDEN</w:t></w:r><w:r><w:t>B</w:t></w:r>",
        8,
        8,
      ),
      ("<w:r><w:t>A&amp;&#x1F600;</w:t></w:r>", 3, 3),
    ] {
      assert_eq!(
        statistics(
          &xml(&format!("<w:p>{content}</w:p>")),
          &StylesCatalog::default()
        )
        .unwrap()
        .0,
        Characters {
          without_spaces: without,
          with_spaces: with
        }
      );
    }
  }

  #[test]
  fn cached_character_statistics_require_a_complete_supported_story() {
    let counter = r#"<w:fldSimple w:instr="INFO NumChars"><w:r><w:t>737</w:t></w:r></w:fldSimple>"#;
    let nested = r#"<w:fldSimple w:instr="AUTOTEXT Missing"><w:r><w:t>Page | </w:t></w:r><w:fldSimple w:instr="PAGE"><w:r><w:t>1</w:t></w:r></w:fldSimple></w:fldSimple>"#;
    let content = xml(&format!("<w:p>{counter}</w:p><w:p>{nested}</w:p>"));
    assert_eq!(
      character_count(&content, &StylesCatalog::default()),
      Some(9)
    );
    for unsupported in [
      r#"<w:fldSimple w:instr="EQ \o(A,B)"/>"#,
      r#"<w:fldSimple w:instr="TC Entry"/>"#,
      "<w:r><w:fldChar w:fldCharType=\"begin\"/></w:r>",
      "<w:r><w:drawing/></w:r>",
      "<w:r><w:footnoteReference w:id=\"1\"/></w:r>",
      "<w:r><w:t>A👩‍💻B</w:t></w:r>",
    ] {
      assert_eq!(
        character_count(
          &xml(&format!("<w:p>{counter}{unsupported}</w:p>")),
          &StylesCatalog::default()
        ),
        None
      );
    }
    let renamed = content
      .replace("w:", "other:")
      .replace("xmlns:w=", "xmlns:other=");
    assert_eq!(
      character_count(&renamed, &StylesCatalog::default()),
      Some(9)
    );
    let wrong_namespace = content.replace(
      "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
      "urn:unrelated",
    );
    assert_eq!(
      character_count(&wrong_namespace, &StylesCatalog::default()),
      None
    );
  }
}
