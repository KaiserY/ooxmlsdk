//! Word's separator/header/footer subdocument has one numbering stream.
//!
//! [MS-DOC] 2.3.3 describes the physical story order and the extra, invisible
//! paragraph terminating each nonempty story. Native DOCX controls confirm
//! that these guards inherit Normal numbering, independently of the preceding
//! paragraph, and that the main document and ordinary notes remain separate.

use super::{
  Block, InlineItem, NumberingCatalog, NumberingCounterState, NumberingFormatMergeContext,
  NumberingReference, ParagraphFormat, StylesCatalog, TextStyle, layout, properties, w,
};
use std::sync::Arc;

pub(super) struct HeaderDocumentNumbering {
  pub catalog: NumberingCatalog,
  guard_reference: Option<NumberingReference>,
  guard_style_id: Option<Arc<str>>,
}

pub(super) struct RepeatingStoryImport<'a> {
  pub styles: &'a StylesCatalog,
  pub numbering: &'a mut HeaderDocumentNumbering,
  pub own_later_section: bool,
}

impl HeaderDocumentNumbering {
  pub fn new(styles: &StylesCatalog) -> Self {
    // Word retains the built-in Normal on these internal marks even when a
    // producer assigns w:default to a second, custom paragraph style. Prefer
    // the built-in name, not the arbitrary XML styleId.
    let guard_style_id = styles
      .styles
      .iter()
      .filter(|(_, entry)| {
        !entry.custom_style
          && entry
            .style_type
            .is_none_or(|kind| kind == w::StyleValues::Paragraph)
          && entry
            .name
            .as_deref()
            .is_some_and(|name| name.eq_ignore_ascii_case("Normal"))
      })
      .map(|(id, _)| id.as_str())
      .min()
      .or(styles.default_paragraph_style_id.as_deref());
    Self {
      catalog: styles.numbering_for_story(),
      guard_reference: styles
        .paragraph_numbering_reference(guard_style_id)
        .filter(|reference| reference.num_id() != 0),
      guard_style_id: guard_style_id.map(Arc::from),
    }
  }

  pub fn advance_guard(&mut self, styles: &StylesCatalog) {
    let Some(reference) = self.guard_reference else {
      return;
    };
    // No paragraph, line layout or font measurement is created for this mark.
    // Reuse the counter's level/restart rules; discard its unpainted label.
    self.catalog.next_label(
      reference,
      &mut ParagraphFormat {
        style_id: self.guard_style_id.clone(),
        ..Default::default()
      },
      styles,
      TextStyle::default(),
      None,
      NumberingFormatMergeContext {
        style_numbering: true,
        ..Default::default()
      },
    );
  }

  pub fn recovered_note_separator(&mut self, styles: &StylesCatalog) {
    // Missing/unreferenced separators recover a Normal paragraph containing
    // the separator character, followed by its separate guard paragraph.
    self.advance_guard(styles);
    self.advance_guard(styles);
  }

  pub fn recovered_note_separators(&mut self, styles: &StylesCatalog) {
    self.recovered_note_separator(styles);
    self.recovered_note_separator(styles);
  }

  pub fn finish_repeating_story(
    &mut self,
    blocks: &mut [Block],
    before: NumberingCounterState,
    styles: &StylesCatalog,
    own_later_section: bool,
  ) {
    let active = match blocks {
      [] => false,
      [block @ Block::Paragraph(_)]
        if !layout::block_has_visible_body_content(block)
          && !matches!(&*block, Block::Paragraph(p) if p.inlines.iter().any(|inline| matches!(inline, InlineItem::Text(run) if !run.text.is_empty()))) =>
      {
        let Block::Paragraph(paragraph) = block else {
          unreachable!()
        };
        let style_id = paragraph.format.style_id.as_deref();
        let inherited_reference = styles
          .paragraph_numbering_reference(style_id)
          .filter(|reference| reference.num_id() != 0);
        let different_numbering =
          paragraph.format.numbering_id != inherited_reference.map(NumberingReference::num_id);
        let different_layout = if let Some(reference) = inherited_reference {
          // A matching direct indent is not a change when the style acquires
          // that indent from its list level. Compare the resolved layouts,
          // retaining the actual stream state around this format-only query.
          let after = self.catalog.counter_state();
          let mut inherited =
            styles.paragraph_format_with_base(style_id, ParagraphFormat::default());
          inherited.style_id = paragraph.format.style_id.clone();
          self.catalog.next_label(
            reference,
            &mut inherited,
            styles,
            TextStyle::default(),
            None,
            NumberingFormatMergeContext {
              style_numbering: true,
              ..styles.numbering_matched_style_indent_context(style_id)
            },
          );
          self.catalog.restore_counter_state(after);
          properties::header_layout(&paragraph.format, styles)
            != properties::header_layout(&inherited, styles)
        } else {
          paragraph.format.header_layout_differs_from_style
        };
        paragraph.format.header_layout_differs_from_style = different_numbering || different_layout;
        own_later_section || paragraph.format.header_layout_differs_from_style
      }
      _ => true,
    };
    if active {
      self.advance_guard(styles);
    } else {
      // A style-owned empty placeholder is not stored as a physical story.
      // It consumes neither its apparent paragraph number nor a guard.
      self.catalog.restore_counter_state(before);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::super::{
    AbstractNumbering, CustomXmlBindings, FormWidgetIdAllocator, HyperlinkCatalog, ImageCatalog,
    ImportSettings, NumberingInstance, StyleEntry, numbering_level_model, text,
  };
  use super::*;
  use ooxmlsdk::sdk::SdkType;
  use std::collections::HashMap;

  fn styles(normal_number: i32) -> StylesCatalog {
    let level = w::Level::from_bytes(br#"<w:lvl xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="lowerRoman"/><w:pStyle w:val="Normal"/><w:suff w:val="space"/><w:lvlText w:val="%1"/><w:pPr><w:ind w:left="227" w:hanging="227"/></w:pPr></w:lvl>"#).unwrap();
    let numbering = NumberingCatalog {
      abstract_nums: HashMap::from([(
        4,
        AbstractNumbering {
          levels: HashMap::from([(0, numbering_level_model(&level, ImportSettings::default()))]),
          ..Default::default()
        },
      )]),
      nums: HashMap::from([(
        4,
        NumberingInstance {
          abstract_num_id: 4,
          overrides: HashMap::new(),
        },
      )]),
      ..Default::default()
    };
    let normal = w::NumberingProperties::from_bytes(format!(r#"<w:numPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:numId w:val="{normal_number}"/></w:numPr>"#).as_bytes()).unwrap();
    StylesCatalog {
      default_paragraph_style_id: Some("Normal".into()),
      numbering_template: Some(numbering),
      styles: HashMap::from([
        (
          "Normal".into(),
          StyleEntry {
            name: Some("Normal".into()),
            paragraph_numbering: Some(Box::new(normal)),
            ..Default::default()
          },
        ),
        (
          "Header".into(),
          StyleEntry {
            based_on: Some("Normal".into()),
            ..Default::default()
          },
        ),
      ]),
      ..Default::default()
    }
  }

  fn paragraph(
    catalog: &mut NumberingCatalog,
    styles: &StylesCatalog,
    properties: &str,
    content: &str,
  ) -> super::super::Paragraph {
    let source = w::Paragraph::from_bytes(format!(r#"<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:pPr><w:pStyle w:val="Header"/>{properties}</w:pPr>{content}</w:p>"#).as_bytes()).unwrap();
    text::paragraph_model(
      &source,
      styles,
      catalog,
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
      &CustomXmlBindings::default(),
      &mut FormWidgetIdAllocator::default(),
    )
  }

  #[test]
  fn separator_guards_share_header_numbers_but_not_body_or_note_numbers() {
    for (normal_number, cancel_separators, expected) in [
      (4, false, "ix "),
      (4, true, "v "),
      (0, false, "v "),
      (0, true, "i "),
    ] {
      let styles = styles(normal_number);
      let mut stream = HeaderDocumentNumbering::new(&styles);
      let direct = r#"<w:numPr><w:numId w:val="4"/></w:numPr>"#;
      for _ in 0..4 {
        if !cancel_separators {
          paragraph(&mut stream.catalog, &styles, direct, "");
        }
        stream.advance_guard(&styles);
      }
      assert_eq!(
        paragraph(&mut stream.catalog, &styles, direct, "")
          .list_label
          .as_deref(),
        Some(expected)
      );
      let mut independent = styles.numbering_for_story();
      assert_eq!(
        paragraph(&mut independent, &styles, direct, "")
          .list_label
          .as_deref(),
        Some("i ")
      );
    }
  }

  #[test]
  fn empty_repeating_story_uses_resolved_list_layout_and_own_section() {
    let styles = styles(4);
    for (properties, content, later, expected) in [
      ("", "", false, "ix "),
      (r#"<w:ind w:left="227" w:hanging="227"/>"#, "", false, "ix "),
      (r#"<w:ind w:left="240" w:hanging="227"/>"#, "", false, "xi "),
      (
        r#"<w:numPr><w:numId w:val="0"/></w:numPr>"#,
        "",
        false,
        "x ",
      ),
      (
        "",
        r#"<w:r><w:t xml:space="preserve"> </w:t></w:r>"#,
        false,
        "xi ",
      ),
      ("", "", true, "xi "),
    ] {
      let mut stream = HeaderDocumentNumbering::new(&styles);
      stream.recovered_note_separators(&styles);
      stream.recovered_note_separators(&styles);
      let before = stream.catalog.counter_state();
      let mut blocks = vec![Block::paragraph(paragraph(
        &mut stream.catalog,
        &styles,
        properties,
        content,
      ))];
      stream.finish_repeating_story(&mut blocks, before, &styles, later);
      assert_eq!(
        paragraph(&mut stream.catalog, &styles, "", "")
          .list_label
          .as_deref(),
        Some(expected),
        "{properties} {content} {later}"
      );
    }
  }
}
