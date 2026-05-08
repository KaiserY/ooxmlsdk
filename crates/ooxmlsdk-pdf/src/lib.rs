//! PDF conversion support for Open XML packages.
//!
//! The crate is intentionally split into three stages:
//!
//! 1. DOCX extraction from `ooxmlsdk` package/schema types.
//! 2. Word-like layout into an internal page/display-list model.
//! 3. PDF rendering through `krilla`.

mod docx;
mod error;
mod fonts;
mod layout;
mod options;
mod pptx;
mod render;
mod text_metrics;
mod xlsx;

use std::io::{Read, Seek};

use ooxmlsdk::parts::{
  presentation_document::PresentationDocument, spreadsheet_document::SpreadsheetDocument,
  wordprocessing_document::WordprocessingDocument,
};

pub use error::{PdfError, Result};
pub use options::{PdfOptions, PdfStandard};

/// Convert a DOCX stream into PDF bytes.
pub fn convert_docx<R>(reader: R, options: PdfOptions) -> Result<Vec<u8>>
where
  R: Read + Seek,
{
  let mut document = WordprocessingDocument::new(reader)?;
  convert_wordprocessing_document(&mut document, options)
}

/// Convert an opened Wordprocessing document into PDF bytes.
pub fn convert_wordprocessing_document(
  document: &mut WordprocessingDocument,
  options: PdfOptions,
) -> Result<Vec<u8>> {
  let doc = docx::extract(document, &options)?;
  let pages = layout::layout(&doc, &options)?;
  render::krilla::render(&pages, &options)
}

/// Convert an XLSX stream into PDF bytes.
pub fn convert_xlsx<R>(reader: R, options: PdfOptions) -> Result<Vec<u8>>
where
  R: Read + Seek,
{
  let mut document = SpreadsheetDocument::new(reader)?;
  convert_spreadsheet_document(&mut document, options)
}

/// Convert an opened spreadsheet document into PDF bytes.
pub fn convert_spreadsheet_document(
  document: &mut SpreadsheetDocument,
  options: PdfOptions,
) -> Result<Vec<u8>> {
  let pages = xlsx::layout(document, &options)?;
  render::krilla::render(&pages, &options)
}

/// Convert a PPTX stream into PDF bytes.
pub fn convert_pptx<R>(reader: R, options: PdfOptions) -> Result<Vec<u8>>
where
  R: Read + Seek,
{
  let mut document = PresentationDocument::new(reader)?;
  convert_presentation_document(&mut document, options)
}

/// Convert an opened presentation document into PDF bytes.
pub fn convert_presentation_document(
  document: &mut PresentationDocument,
  options: PdfOptions,
) -> Result<Vec<u8>> {
  let pages = pptx::layout(document, &options)?;
  render::krilla::render(&pages, &options)
}

#[cfg(test)]
mod tests {
  use std::fs::File;
  use std::path::PathBuf;

  use super::*;

  #[test]
  fn convert_docx_writes_pdf_document() {
    let path = fixture_path("test-data/document/minimal_empty.docx");
    let pdf = convert_docx(File::open(path).unwrap(), PdfOptions::default()).unwrap();

    assert!(pdf.starts_with(b"%PDF-"));
    assert!(pdf.ends_with(b"%%EOF\n") || pdf.ends_with(b"%%EOF"));
  }

  #[test]
  fn convert_xlsx_writes_pdf_document() {
    let path = fixture_path("test-data/spreadsheet/minimal_values.xlsx");
    let pdf = convert_xlsx(File::open(path).unwrap(), PdfOptions::default()).unwrap();

    assert!(pdf.starts_with(b"%PDF-"));
    assert!(pdf.ends_with(b"%%EOF\n") || pdf.ends_with(b"%%EOF"));
  }

  #[test]
  fn convert_pptx_writes_pdf_document() {
    let path = fixture_path("test-data/slideshow/minimal_text.pptx");
    let pdf = convert_pptx(File::open(path).unwrap(), PdfOptions::default()).unwrap();

    assert!(pdf.starts_with(b"%PDF-"));
    assert!(pdf.ends_with(b"%%EOF\n") || pdf.ends_with(b"%%EOF"));
  }

  #[test]
  fn minimal_text_docx_flows_text_into_layout() {
    let path = fixture_path("test-data/document/minimal_text.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();

    assert_eq!(doc.blocks.len(), 1);
    assert!(layout.pages.iter().any(|page| {
      page.items.iter().any(|item| {
        matches!(
          item,
          crate::layout::PageItem::Text(text) if text.text == "Hello ooxmlsdk"
        )
      })
    }));
  }

  #[test]
  fn paragraph_layout_wraps_on_word_boundaries() {
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup {
        width_pt: 56.0,
        height_pt: 200.0,
        margin_left_pt: 10.0,
        margin_right_pt: 10.0,
        margin_top_pt: 10.0,
        margin_bottom_pt: 10.0,
        header_distance_pt: 0.0,
        footer_distance_pt: 0.0,
      },
      default_tab_stop_pt: 36.0,
      even_and_odd_headers: false,
      sections: Vec::new(),
      title_page: false,
      header_blocks: Vec::new(),
      first_header_blocks: Vec::new(),
      footer_blocks: Vec::new(),
      first_footer_blocks: Vec::new(),
      footnote_blocks: Vec::new(),
      footnotes: Default::default(),
      endnote_blocks: Vec::new(),
      endnotes: Default::default(),
      comment_blocks: Vec::new(),
      blocks: vec![crate::docx::Block::Paragraph(crate::docx::Paragraph {
        format: crate::docx::ParagraphFormat::default(),
        list_label: None,
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![crate::docx::TextRun {
          text: "One Two Six".into(),
          style: crate::docx::TextStyle::default(),
        }],
        inlines: vec![crate::docx::InlineItem::Text(crate::docx::TextRun {
          text: "One Two Six".into(),
          style: crate::docx::TextStyle::default(),
        })],
      })],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let lines = layout.pages[0]
      .items
      .iter()
      .filter_map(|item| match item {
        crate::layout::PageItem::Text(text) => Some(text.text.trim().to_string()),
        crate::layout::PageItem::Image(_)
        | crate::layout::PageItem::Fill(_)
        | crate::layout::PageItem::Line(_) => None,
      })
      .collect::<Vec<_>>();

    assert_eq!(lines, ["One", "Two", "Six"]);
  }

  #[test]
  fn paragraph_tabs_advance_to_default_tab_stops() {
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup {
        width_pt: 300.0,
        height_pt: 200.0,
        margin_left_pt: 10.0,
        margin_right_pt: 10.0,
        margin_top_pt: 10.0,
        margin_bottom_pt: 10.0,
        header_distance_pt: 0.0,
        footer_distance_pt: 0.0,
      },
      default_tab_stop_pt: 36.0,
      even_and_odd_headers: false,
      sections: Vec::new(),
      title_page: false,
      header_blocks: Vec::new(),
      first_header_blocks: Vec::new(),
      footer_blocks: Vec::new(),
      first_footer_blocks: Vec::new(),
      footnote_blocks: Vec::new(),
      footnotes: Default::default(),
      endnote_blocks: Vec::new(),
      endnotes: Default::default(),
      comment_blocks: Vec::new(),
      blocks: vec![crate::docx::Block::Paragraph(crate::docx::Paragraph {
        format: crate::docx::ParagraphFormat::default(),
        list_label: None,
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![crate::docx::TextRun {
          text: "Left\tRight".into(),
          style: crate::docx::TextStyle::default(),
        }],
        inlines: vec![crate::docx::InlineItem::Text(crate::docx::TextRun {
          text: "Left\tRight".into(),
          style: crate::docx::TextStyle::default(),
        })],
      })],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let texts = layout.pages[0]
      .items
      .iter()
      .filter_map(|item| match item {
        crate::layout::PageItem::Text(text) => Some(text),
        crate::layout::PageItem::Image(_)
        | crate::layout::PageItem::Fill(_)
        | crate::layout::PageItem::Line(_) => None,
      })
      .collect::<Vec<_>>();

    assert_eq!(texts.len(), 2);
    assert_eq!(texts[0].text, "Left");
    assert_eq!(texts[1].text, "Right");
    assert!((texts[1].x_pt - 46.0).abs() < 0.1);
  }

  #[test]
  fn paragraph_right_tabs_align_following_text_to_stop() {
    let format = crate::docx::ParagraphFormat {
      tab_stops: vec![crate::docx::TabStop {
        position_pt: 100.0,
        alignment: crate::docx::TabStopAlignment::Right,
      }],
      ..Default::default()
    };
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup {
        width_pt: 300.0,
        height_pt: 200.0,
        margin_left_pt: 10.0,
        margin_right_pt: 10.0,
        margin_top_pt: 10.0,
        margin_bottom_pt: 10.0,
        header_distance_pt: 0.0,
        footer_distance_pt: 0.0,
      },
      default_tab_stop_pt: 36.0,
      even_and_odd_headers: false,
      sections: Vec::new(),
      title_page: false,
      header_blocks: Vec::new(),
      first_header_blocks: Vec::new(),
      footer_blocks: Vec::new(),
      first_footer_blocks: Vec::new(),
      footnote_blocks: Vec::new(),
      footnotes: Default::default(),
      endnote_blocks: Vec::new(),
      endnotes: Default::default(),
      comment_blocks: Vec::new(),
      blocks: vec![crate::docx::Block::Paragraph(crate::docx::Paragraph {
        format,
        list_label: None,
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![crate::docx::TextRun {
          text: "Title\t99".into(),
          style: crate::docx::TextStyle::default(),
        }],
        inlines: vec![crate::docx::InlineItem::Text(crate::docx::TextRun {
          text: "Title\t99".into(),
          style: crate::docx::TextStyle::default(),
        })],
      })],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let texts = layout.pages[0]
      .items
      .iter()
      .filter_map(|item| match item {
        crate::layout::PageItem::Text(text) => Some(text),
        crate::layout::PageItem::Image(_)
        | crate::layout::PageItem::Fill(_)
        | crate::layout::PageItem::Line(_) => None,
      })
      .collect::<Vec<_>>();

    assert_eq!(texts.len(), 2);
    assert_eq!(texts[1].text, "99");
    assert!(texts[1].x_pt < 110.0);
    assert!(texts[1].x_pt > 90.0);
  }

  #[test]
  fn run_formatting_is_preserved_in_layout_items() {
    let path = fixture_path("test-data/wml/char_formatting.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let paragraph = paragraph_at(&doc, 0);
    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();

    let texts = layout
      .pages
      .iter()
      .flat_map(|page| &page.items)
      .filter_map(|item| match item {
        crate::layout::PageItem::Text(text) => Some(text),
        crate::layout::PageItem::Image(_) => None,
        crate::layout::PageItem::Fill(_) => None,
        crate::layout::PageItem::Line(_) => None,
      })
      .collect::<Vec<_>>();

    assert!(
      texts
        .iter()
        .any(|item| item.text.contains("Bold") && item.style.bold)
    );
    assert!(
      texts
        .iter()
        .any(|item| item.text.contains("Italic") && item.style.italic)
    );
    assert!(
      texts
        .iter()
        .any(|item| item.text.contains("Underline") && item.style.underline)
    );
    assert!(
      texts
        .iter()
        .any(|item| item.text.contains("Strike") && item.style.strikethrough)
    );
    assert!(
      texts
        .iter()
        .any(|item| item.text.contains("14pt") && (item.style.font_size_pt - 14.0).abs() < 0.1)
    );
    assert!(texts.iter().any(|item| {
      item.text.contains("Red")
        && item.style.color
          == crate::docx::RgbColor {
            r: 0xC0,
            g: 0,
            b: 0,
          }
    }));
    let highlight = paragraph
      .runs
      .iter()
      .find(|run| run.text.contains("Highlight"))
      .expect("highlight run");
    assert_eq!(
      highlight.style.highlight,
      Some(crate::docx::RgbColor {
        r: 255,
        g: 255,
        b: 0,
      })
    );
    let superscript = paragraph
      .runs
      .iter()
      .find(|run| run.text == "sup")
      .expect("superscript run");
    assert!(superscript.style.baseline_shift_pt > 0.0);
    assert!(superscript.style.font_size_pt < 11.0);

    let subscript = paragraph
      .runs
      .iter()
      .find(|run| run.text == "sub")
      .expect("subscript run");
    assert!(subscript.style.baseline_shift_pt < 0.0);
    assert!(subscript.style.font_size_pt < 11.0);

    assert!(
      paragraph
        .runs
        .iter()
        .any(|run| run.text.contains(" CAPS") && run.style.uppercase)
    );
    assert!(
      paragraph
        .runs
        .iter()
        .any(|run| run.text.contains(" SMALLCAPS") && run.style.uppercase)
    );
  }

  #[test]
  fn paragraph_spacing_and_indentation_are_extracted() {
    let path = fixture_path("test-data/wml/para_indent.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();

    let first = paragraph_at(&doc, 0);
    assert_eq!(first.format.indent_left_pt, 36.0);

    let third = paragraph_at(&doc, 2);
    assert_eq!(third.format.first_line_indent_pt, 18.0);

    let fourth = paragraph_at(&doc, 3);
    assert_eq!(fourth.format.first_line_indent_pt, -18.0);

    let path = fixture_path("test-data/wml/para_spacing.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let spaced = paragraph_at(&doc, 0);

    assert_eq!(spaced.format.spacing_before_pt, 12.0);
    assert_eq!(spaced.format.spacing_after_pt, 6.0);

    let double_spaced = paragraph_at(&doc, 3);
    assert_eq!(double_spaced.format.line_height_pt, Some(28.0));
  }

  #[test]
  fn paragraph_keep_properties_are_extracted_and_applied() {
    let path = fixture_path("test-data/wml/para_keep.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();

    assert!(paragraph_at(&doc, 0).format.keep_with_next);
    assert!(paragraph_at(&doc, 2).format.keep_lines);
    assert!(paragraph_at(&doc, 3).format.page_break_before);

    let path = fixture_path("test-data/wml/para_keep_flow.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();

    assert!(layout.pages.len() >= 2);
    assert!(page_has_text(&layout.pages[0], "Filler 1"));
    assert!(!page_has_text(&layout.pages[0], "Kept heading"));
    assert!(page_has_text(&layout.pages[1], "Kept heading"));
    assert!(page_has_text(&layout.pages[1], "Kept body"));

    let pdf = convert_docx(File::open(path).unwrap(), PdfOptions::default()).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
  }

  #[test]
  fn paragraph_styles_inherit_doc_defaults_and_based_on_chain() {
    let path = fixture_path("test-data/wml/style_inheritance.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();

    let normal = paragraph_at(&doc, 0);
    assert_eq!(normal.runs[0].style.font_size_pt, 11.0);
    assert_eq!(normal.format.spacing_after_pt, 8.0);

    let body_text = paragraph_at(&doc, 1);
    assert_eq!(body_text.runs[0].style.font_size_pt, 12.0);
    assert_eq!(body_text.format.spacing_after_pt, 8.0);

    let body_indent = paragraph_at(&doc, 2);
    assert_eq!(body_indent.runs[0].style.font_size_pt, 12.0);
    assert_eq!(body_indent.format.indent_left_pt, 36.0);
    assert_eq!(body_indent.format.spacing_after_pt, 8.0);
  }

  #[test]
  fn run_character_styles_are_resolved_from_styles_part() {
    let path = fixture_path("test-data/wml/run_fonts.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let paragraph = paragraph_at(&doc, 0);
    assert!(
      paragraph
        .runs
        .iter()
        .any(|run| run.text.contains("StrongStyle") && run.style.bold)
    );

    let path = fixture_path("test-data/wml/style_linked.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let quote = paragraph_at(&doc, 1);
    assert_eq!(
      quote.format.alignment,
      crate::docx::ParagraphAlignment::Center
    );
    assert!(quote.runs[0].style.italic);

    let inline = paragraph_at(&doc, 2);
    assert!(
      inline
        .runs
        .iter()
        .any(|run| run.text == "character-styled word" && run.style.italic)
    );
  }

  #[test]
  fn numbering_labels_and_indents_are_extracted() {
    let path = fixture_path("test-data/wml/numbering_ordered.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let labels = doc
      .blocks
      .iter()
      .filter_map(|block| match block {
        crate::docx::Block::Paragraph(paragraph) => paragraph.list_label.as_deref(),
        crate::docx::Block::Table(_) => None,
      })
      .collect::<Vec<_>>();

    assert_eq!(labels, ["1. ", "a. ", "i. ", "b. ", "2. ", "3. "]);
    assert_eq!(paragraph_at(&doc, 0).format.indent_left_pt, 36.0);
    assert_eq!(paragraph_at(&doc, 1).format.indent_left_pt, 72.0);
    assert_eq!(paragraph_at(&doc, 2).format.indent_left_pt, 108.0);

    let path = fixture_path("test-data/wml/numbering_bullets.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    assert_eq!(
      paragraph_at(&doc, 0).list_label.as_deref(),
      Some("\u{2022} ")
    );

    let path = fixture_path("test-data/wml/numbering_restart.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    assert_eq!(paragraph_at(&doc, 1).list_label.as_deref(), Some("1. "));
    assert_eq!(paragraph_at(&doc, 3).list_label.as_deref(), Some("3. "));
    assert_eq!(paragraph_at(&doc, 5).list_label.as_deref(), Some("1. "));
  }

  #[test]
  fn paragraph_alignment_is_extracted_and_applied_to_layout() {
    let path = fixture_path("test-data/wml/para_alignment.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();

    assert_eq!(
      paragraph_at(&doc, 0).format.alignment,
      crate::docx::ParagraphAlignment::Left
    );
    assert_eq!(
      paragraph_at(&doc, 1).format.alignment,
      crate::docx::ParagraphAlignment::Center
    );
    assert_eq!(
      paragraph_at(&doc, 2).format.alignment,
      crate::docx::ParagraphAlignment::Right
    );
    assert_eq!(
      paragraph_at(&doc, 3).format.alignment,
      crate::docx::ParagraphAlignment::Justify
    );
    assert_eq!(
      paragraph_at(&doc, 4).format.alignment,
      crate::docx::ParagraphAlignment::Justify
    );

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let left = text_item(&layout, "Left aligned paragraph.").x_pt;
    let center = text_item(&layout, "Center aligned paragraph.").x_pt;
    let right = text_item(&layout, "Right aligned paragraph.").x_pt;

    assert_eq!(left, doc.page.margin_left_pt);
    assert!(center > left);
    assert!(right > center);
  }

  #[test]
  fn paragraph_borders_and_shading_are_extracted_and_rendered() {
    let path = fixture_path("test-data/wml/para_borders_shading.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();

    let boxed = paragraph_at(&doc, 0);
    assert_eq!(
      boxed.format.shading,
      Some(crate::docx::RgbColor {
        r: 0xDE,
        g: 0xEA,
        b: 0xF1
      })
    );
    assert_eq!(boxed.format.borders.top.unwrap().color.b, 0xC4);
    assert_eq!(boxed.format.borders.left.unwrap().width_pt, 0.75);

    let patterned = paragraph_at(&doc, 1);
    assert_eq!(
      patterned.format.shading,
      Some(crate::docx::RgbColor {
        r: 0xFF,
        g: 0xFF,
        b: 0x00
      })
    );

    let top_bottom = paragraph_at(&doc, 2);
    assert!(top_bottom.format.borders.top.is_some());
    assert!(top_bottom.format.borders.bottom.is_some());
    assert!(top_bottom.format.borders.left.is_none());

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    assert!(layout.pages.iter().any(|page| {
      page.items.iter().any(|item| {
        matches!(
          item,
          crate::layout::PageItem::Fill(fill)
            if fill.color.r == 0xDE && fill.color.g == 0xEA && fill.color.b == 0xF1
        )
      })
    }));
    assert!(layout.pages.iter().any(|page| {
      page.items.iter().any(|item| {
        matches!(
          item,
          crate::layout::PageItem::Line(line)
            if line.color.r == 0x44 && line.color.g == 0x72 && line.color.b == 0xC4
        )
      })
    }));

    let pdf = convert_docx(File::open(path).unwrap(), PdfOptions::default()).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
  }

  #[test]
  fn default_header_and_footer_are_repeated_on_layout_pages() {
    let path = fixture_path("test-data/wml/header_footer.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();

    assert_eq!(doc.header_blocks.len(), 1);
    assert_eq!(doc.footer_blocks.len(), 1);
    assert_eq!(doc.page.header_distance_pt, 36.0);
    assert_eq!(doc.page.footer_distance_pt, 36.0);

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let header = text_item(&layout, "Page Header");
    let footer = text_item(&layout, "Page Footer");
    let body = text_item(&layout, "Document body text.");

    assert!(header.y_pt < body.y_pt);
    assert!(footer.y_pt > body.y_pt);
    assert!(header.x_pt > doc.page.margin_left_pt);
    assert!(footer.x_pt > doc.page.margin_left_pt);

    let pdf = convert_docx(File::open(path).unwrap(), PdfOptions::default()).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
  }

  #[test]
  fn first_page_header_is_used_when_title_page_is_enabled() {
    let path = fixture_path("test-data/wml/header_first_page.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();

    assert!(doc.title_page);
    assert_eq!(doc.header_blocks.len(), 1);
    assert_eq!(doc.first_header_blocks.len(), 1);
    assert_eq!(doc.footer_blocks.len(), 1);
    assert!(doc.first_footer_blocks.is_empty());

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    assert!(find_text_item(&layout, "First Page Header").is_some());
    assert!(find_text_item(&layout, "Default Header (odd pages)").is_none());
    assert!(find_text_item(&layout, "Default Footer").is_some());

    let pdf = convert_docx(File::open(path).unwrap(), PdfOptions::default()).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
  }

  #[test]
  fn section_headers_inherit_and_even_headers_are_selected() {
    let path = fixture_path("test-data/wml/header_section_inheritance.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();

    assert!(doc.even_and_odd_headers);
    assert_eq!(doc.sections.len(), 2);
    assert_eq!(doc.sections[0].header_blocks.len(), 1);
    assert_eq!(doc.sections[0].even_header_blocks.len(), 1);
    assert_eq!(doc.sections[0].footer_blocks.len(), 1);
    assert_eq!(doc.sections[1].header_blocks.len(), 1);
    assert_eq!(doc.sections[1].even_header_blocks.len(), 1);
    assert_eq!(doc.sections[1].footer_blocks.len(), 1);

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    assert!(layout.pages.len() >= 2);
    assert!(page_has_text(&layout.pages[0], "Inherited Default Header"));
    assert!(page_has_text(&layout.pages[1], "Inherited Even Header"));
    assert!(page_has_text(&layout.pages[1], "Inherited Footer"));
    assert!(!page_has_text(&layout.pages[1], "Inherited Default Header"));

    let pdf = convert_docx(File::open(path).unwrap(), PdfOptions::default()).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
  }

  #[test]
  fn later_section_first_page_header_and_footer_are_selected() {
    let path = fixture_path("test-data/wml/header_section_first_page.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();

    assert_eq!(doc.sections.len(), 2);
    assert!(!doc.sections[0].title_page);
    assert!(doc.sections[1].title_page);
    assert_eq!(doc.sections[1].header_blocks.len(), 1);
    assert_eq!(doc.sections[1].first_header_blocks.len(), 1);
    assert_eq!(doc.sections[1].footer_blocks.len(), 1);
    assert_eq!(doc.sections[1].first_footer_blocks.len(), 1);

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    assert!(layout.pages.len() >= 2);
    assert!(page_has_text(&layout.pages[0], "Opening Section Header"));
    assert!(page_has_text(
      &layout.pages[1],
      "Second Section First Header"
    ));
    assert!(page_has_text(
      &layout.pages[1],
      "Second Section First Footer"
    ));
    assert!(!page_has_text(
      &layout.pages[1],
      "Second Section Default Header"
    ));
    assert!(!page_has_text(
      &layout.pages[1],
      "Second Section Default Footer"
    ));

    let pdf = convert_docx(File::open(path).unwrap(), PdfOptions::default()).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
  }

  #[test]
  fn section_columns_flow_blocks_into_next_column() {
    let path = fixture_path("test-data/wml/section_columns_flow.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();

    assert_eq!(doc.sections.len(), 1);
    assert_eq!(doc.sections[0].columns.count, 2);
    assert_eq!(doc.sections[0].columns.gap_pt, 18.0);
    assert!(doc.sections[0].columns.separator);

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let first_column = text_item(&layout, "Item 01");
    let second_column = text_item(&layout, "Item 06");

    assert!(second_column.x_pt > first_column.x_pt + 50.0);
    assert!(
      layout.pages[0]
        .items
        .iter()
        .any(|item| matches!(item, crate::layout::PageItem::Line(_)))
    );

    let pdf = convert_docx(File::open(path).unwrap(), PdfOptions::default()).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
  }

  #[test]
  fn hard_column_break_advances_to_next_section_column() {
    let path = fixture_path("test-data/wml/section_column_break.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();

    assert_eq!(doc.sections.len(), 1);
    assert_eq!(doc.sections[0].columns.count, 2);
    assert!(doc.sections[0].blocks.iter().any(|block| {
      matches!(
        block,
        crate::docx::Block::Paragraph(paragraph)
          if paragraph
            .inlines
            .iter()
            .any(|item| matches!(item, crate::docx::InlineItem::ColumnBreak))
      )
    }));

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let before = text_item(&layout, "Before break.");
    let after = text_item(&layout, "After break.");
    let before_page = layout
      .pages
      .iter()
      .position(|page| page_has_text(page, "Before break."))
      .expect("before page");
    let after_page = layout
      .pages
      .iter()
      .position(|page| page_has_text(page, "After break."))
      .expect("after page");

    assert_eq!(before_page, after_page);
    assert!(after.x_pt > before.x_pt + 100.0);

    let pdf = convert_docx(File::open(path).unwrap(), PdfOptions::default()).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
  }

  #[test]
  fn explicit_section_column_widths_are_imported_and_used() {
    let path = fixture_path("test-data/wml/section_columns_explicit.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();

    assert_eq!(doc.sections.len(), 1);
    assert_eq!(doc.sections[0].columns.count, 2);
    assert_eq!(doc.sections[0].columns.explicit_count, 2);
    assert_eq!(
      &doc.sections[0].columns.explicit_widths_pt[..2],
      [72.0, 144.0]
    );
    assert_eq!(&doc.sections[0].columns.explicit_gaps_pt[..1], [36.0]);
    assert!(doc.sections[0].columns.separator);

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let narrow = text_item(&layout, "Narrow");
    let wide = text_item(&layout, "Wide");

    assert!(wide.x_pt > narrow.x_pt + 90.0);
    assert!(
      layout.pages[0]
        .items
        .iter()
        .any(|item| matches!(item, crate::layout::PageItem::Line(_)))
    );

    let pdf = convert_docx(File::open(path).unwrap(), PdfOptions::default()).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
  }

  #[test]
  fn explicit_page_break_splits_layout_pages() {
    let path = fixture_path("test-data/wml/breaks.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();

    assert!(doc.blocks.iter().any(|block| {
      match block {
        crate::docx::Block::Paragraph(paragraph) => paragraph
          .inlines
          .iter()
          .any(|item| matches!(item, crate::docx::InlineItem::PageBreak)),
        crate::docx::Block::Table(_) => false,
      }
    }));

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    assert!(layout.pages.len() >= 2);
    assert!(layout.pages[0].items.iter().any(|item| {
      matches!(
        item,
        crate::layout::PageItem::Text(text) if text.text.contains("Before soft return")
      )
    }));
    assert!(layout.pages[1].items.iter().any(|item| {
      matches!(
        item,
        crate::layout::PageItem::Text(text) if text.text.contains("After page break")
      )
    }));
  }

  #[test]
  fn simple_fields_and_hyperlinks_emit_cached_text() {
    let path = fixture_path("test-data/wml/fields_hyperlink.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();

    assert!(find_text_item(&layout, "May 2, 2026").is_some());
    assert_eq!(
      text_item(&layout, "Visit example.com").style.color,
      crate::docx::RgbColor {
        r: 0x05,
        g: 0x63,
        b: 0xC1,
      }
    );
    assert!(text_item(&layout, "Visit example.com").style.underline);
    assert_eq!(
      text_item(&layout, "Go to target section").style.color,
      crate::docx::RgbColor {
        r: 0x05,
        g: 0x63,
        b: 0xC1,
      }
    );
    assert!(text_item(&layout, "Go to target section").style.underline);

    let pdf = convert_docx(File::open(path).unwrap(), PdfOptions::default()).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
  }

  #[test]
  fn note_references_emit_inline_markers() {
    let path = fixture_path("test-data/wml/footnotes.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();

    assert_eq!(paragraph_at(&doc, 0).footnote_reference_ids, [1, 2]);
    assert_eq!(doc.footnotes.len(), 2);
    assert_eq!(doc.footnote_blocks.len(), 2);
    assert!(find_text_item(&layout, "1").is_some());
    assert!(find_text_item(&layout, "2").is_some());
    assert!(find_text_item(&layout, " First footnote content.").is_some());
    assert!(find_text_item(&layout, " Second footnote content.").is_some());

    let path = fixture_path("test-data/wml/endnotes.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    assert_eq!(paragraph_at(&doc, 0).endnote_reference_ids, [1]);
    assert_eq!(doc.endnotes.len(), 1);
    assert_eq!(doc.endnote_blocks.len(), 1);
    assert!(find_text_item(&layout, "1").is_some());
    assert!(find_text_item(&layout, " Endnote content.").is_some());
  }

  #[test]
  fn comments_emit_reference_markers_and_comment_blocks() {
    let path = fixture_path("test-data/wml/comments.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();

    assert_eq!(doc.comment_blocks.len(), 2);
    assert!(find_text_item(&layout, "[1]").is_some());
    assert!(find_text_item(&layout, "[2]").is_some());
    assert!(find_text_item(&layout, " First comment by Alice.").is_some());
    assert!(find_text_item(&layout, " Second comment by Bob.").is_some());
  }

  #[test]
  fn tracked_insertions_are_emitted_and_deletions_are_skipped() {
    let path = fixture_path("test-data/wml/tracked_changes.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();

    assert!(find_text_item(&layout, "inserted words").is_some());
    assert!(find_text_item(&layout, "deleted text").is_none());
    assert_eq!(
      paragraph_at(&doc, 3).format.alignment,
      crate::docx::ParagraphAlignment::Center
    );
  }

  #[test]
  fn content_controls_emit_current_sdt_content() {
    let path = fixture_path("test-data/wml/content_controls.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();

    assert!(find_text_item(&layout, "Jane Smith").is_some());
    assert!(find_text_item(&layout, "5/2/2026").is_some());
    assert!(find_text_item(&layout, "Active").is_some());
  }

  #[test]
  fn inline_images_are_extracted_laid_out_and_rendered() {
    let path = fixture_path("test-data/wml/image_inline_props.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let paragraph = paragraph_at(&doc, 0);
    let image = paragraph
      .inlines
      .iter()
      .find_map(|item| match item {
        crate::docx::InlineItem::Image(image) => Some(image),
        crate::docx::InlineItem::Text(_) => None,
        crate::docx::InlineItem::PageBreak | crate::docx::InlineItem::ColumnBreak => None,
      })
      .expect("inline image");

    assert_eq!(image.content_type.as_deref(), Some("image/png"));
    assert_eq!(image.width_pt, 72.0);
    assert_eq!(image.height_pt, 72.0);
    assert_eq!(
      image.alt_text.as_deref(),
      Some("Alt text for accessibility")
    );

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    assert!(layout.pages.iter().any(|page| {
      page.items.iter().any(|item| {
        matches!(
          item,
          crate::layout::PageItem::Image(image)
            if image.width_pt == 72.0 && image.height_pt == 72.0
        )
      })
    }));

    let pdf = convert_docx(File::open(path).unwrap(), PdfOptions::default()).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
  }

  #[test]
  fn floating_anchor_images_are_extracted_as_images() {
    let path = fixture_path("test-data/wml/image_floating.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let paragraph = paragraph_at(&doc, 0);
    let image = paragraph
      .inlines
      .iter()
      .find_map(|item| match item {
        crate::docx::InlineItem::Image(image) => Some(image),
        crate::docx::InlineItem::Text(_)
        | crate::docx::InlineItem::PageBreak
        | crate::docx::InlineItem::ColumnBreak => None,
      })
      .expect("floating image");

    assert_eq!(image.content_type.as_deref(), Some("image/png"));
    assert_eq!(image.width_pt, 72.0);
    assert_eq!(image.height_pt, 72.0);
    let crate::docx::ImagePlacement::Floating(placement) = image.placement else {
      panic!("floating placement");
    };
    assert_eq!(
      placement.horizontal_relative_to,
      crate::docx::HorizontalImageReference::Column
    );
    assert_eq!(
      placement.vertical_relative_to,
      crate::docx::VerticalImageReference::Paragraph
    );
    assert_eq!(placement.wrap, crate::docx::ImageWrapMode::Square);
    assert!(!placement.behind_text);
    assert!(
      paragraph
        .runs
        .iter()
        .any(|run| run.text == "Text beside the floating image.")
    );

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let laid_out_image = layout
      .pages
      .iter()
      .flat_map(|page| &page.items)
      .find_map(|item| match item {
        crate::layout::PageItem::Image(image) => Some(image),
        _ => None,
      })
      .expect("floating image layout");
    assert_eq!(laid_out_image.x_pt, 72.0);
    assert_eq!(laid_out_image.y_pt, 72.0);

    let pdf = convert_docx(File::open(path).unwrap(), PdfOptions::default()).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
  }

  #[test]
  fn minimal_table_docx_flows_cells_and_borders_into_layout() {
    let path = fixture_path("test-data/document/minimal_table.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();

    assert!(
      doc
        .blocks
        .iter()
        .any(|block| matches!(block, crate::docx::Block::Table(_)))
    );
    for expected in ["A1", "B1", "A2", "B2"] {
      assert!(layout.pages.iter().any(|page| {
        page.items.iter().any(|item| {
          matches!(
            item,
            crate::layout::PageItem::Text(text) if text.text.contains(expected)
          )
        })
      }));
    }
    assert!(layout.pages.iter().any(|page| {
      page
        .items
        .iter()
        .any(|item| matches!(item, crate::layout::PageItem::Line(_)))
    }));
  }

  #[test]
  fn table_borders_and_cell_shading_are_extracted_and_rendered() {
    let path = fixture_path("test-data/wml/table_borders.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let table = match &doc.blocks[0] {
      crate::docx::Block::Table(table) => table,
      crate::docx::Block::Paragraph(_) => panic!("expected table"),
    };

    assert_eq!(table.preferred_width_pt, Some(432.0));
    assert_eq!(
      table.borders.unwrap().inside_vertical.unwrap().color.b,
      0xC4
    );
    assert_eq!(table.rows[0].cells[1].borders.right.unwrap().color.r, 0xFF);
    assert_eq!(
      table.rows[0].cells[2].shading,
      Some(crate::docx::RgbColor {
        r: 0xDE,
        g: 0xEA,
        b: 0xF1
      })
    );

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    assert!(layout.pages.iter().any(|page| {
      page.items.iter().any(|item| {
        matches!(
          item,
          crate::layout::PageItem::Fill(fill)
            if fill.color.r == 0xDE && fill.color.g == 0xEA && fill.color.b == 0xF1
        )
      })
    }));
    assert!(layout.pages.iter().any(|page| {
      page.items.iter().any(|item| {
        matches!(
          item,
          crate::layout::PageItem::Line(line)
            if line.color.r == 0xFF && line.width_pt == 1.0
        )
      })
    }));

    let pdf = convert_docx(File::open(path).unwrap(), PdfOptions::default()).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
  }

  #[test]
  fn table_header_rows_repeat_after_page_breaks() {
    let path = fixture_path("test-data/wml/table_header_repeat.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let table = match &doc.blocks[0] {
      crate::docx::Block::Table(table) => table,
      crate::docx::Block::Paragraph(_) => panic!("expected table"),
    };

    assert!(table.rows[0].repeat_header);
    assert!(!table.rows[1].repeat_header);

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    assert!(layout.pages.len() >= 2);
    assert!(page_has_text(&layout.pages[0], "Repeat Header"));
    assert!(page_has_text(&layout.pages[1], "Repeat Header"));
    assert!(text_occurrence_count(&layout, "Repeat Header") >= 2);

    let pdf = convert_docx(File::open(path).unwrap(), PdfOptions::default()).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
  }

  #[test]
  fn table_spans_and_row_heights_are_extracted() {
    let path = fixture_path("test-data/wml/table_merged.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let table = match &doc.blocks[0] {
      crate::docx::Block::Table(table) => table,
      crate::docx::Block::Paragraph(_) => panic!("expected table"),
    };

    assert_eq!(table.rows[0].cells[0].grid_span, 2);
    assert!(table.rows[1].cells[2].vertical_merge_continue);

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    assert!(find_text_item(&layout, "A1+A2 (horizontal merge, gridSpan=2)").is_some());
    assert!(find_text_item(&layout, "A3 top of vertical merge").is_some());

    let path = fixture_path("test-data/wml/table_props.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let table = match &doc.blocks[0] {
      crate::docx::Block::Table(table) => table,
      crate::docx::Block::Paragraph(_) => panic!("expected table"),
    };
    assert_eq!(table.rows[0].height_pt, Some(24.0));
    assert!(table.rows[0].exact_height);
    assert_eq!(table.cell_margins.left_pt, 9.0);
    assert_eq!(table.cell_margins.top_pt, 6.0);
    assert_eq!(table.rows[0].cells[0].margins.left_pt, 18.0);
    assert_eq!(table.rows[0].cells[0].margins.top_pt, 12.0);
    assert_eq!(table.rows[0].cells[1].margins.left_pt, 9.0);
    assert_eq!(
      table.rows[0].cells[0].vertical_alignment,
      crate::docx::TableCellVerticalAlignment::Center
    );
    assert_eq!(
      table.rows[1].cells[0].vertical_alignment,
      crate::docx::TableCellVerticalAlignment::Top
    );
    assert_eq!(
      table.rows[2].cells[0].vertical_alignment,
      crate::docx::TableCellVerticalAlignment::Bottom
    );
  }

  fn fixture_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("../..")
      .join(relative)
  }

  fn paragraph_at(doc: &crate::docx::DocxDocument, index: usize) -> &crate::docx::Paragraph {
    match &doc.blocks[index] {
      crate::docx::Block::Paragraph(paragraph) => paragraph,
      crate::docx::Block::Table(_) => panic!("expected paragraph at block {index}"),
    }
  }

  fn text_item<'a>(
    layout: &'a crate::layout::LayoutDocument,
    text: &str,
  ) -> &'a crate::layout::TextItem {
    find_text_item(layout, text).unwrap_or_else(|| panic!("text item {text:?}"))
  }

  fn page_has_text(page: &crate::layout::Page, text: &str) -> bool {
    page.items.iter().any(|item| {
      matches!(
        item,
        crate::layout::PageItem::Text(item) if item.text == text
      )
    })
  }

  fn text_occurrence_count(layout: &crate::layout::LayoutDocument, text: &str) -> usize {
    layout
      .pages
      .iter()
      .flat_map(|page| &page.items)
      .filter(|item| {
        matches!(
          item,
          crate::layout::PageItem::Text(item) if item.text == text
        )
      })
      .count()
  }

  fn find_text_item<'a>(
    layout: &'a crate::layout::LayoutDocument,
    text: &str,
  ) -> Option<&'a crate::layout::TextItem> {
    layout
      .pages
      .iter()
      .flat_map(|page| &page.items)
      .find_map(|item| match item {
        crate::layout::PageItem::Text(item) if item.text == text => Some(item),
        crate::layout::PageItem::Text(_) => None,
        crate::layout::PageItem::Image(_) => None,
        crate::layout::PageItem::Fill(_) => None,
        crate::layout::PageItem::Line(_) => None,
      })
  }
}
