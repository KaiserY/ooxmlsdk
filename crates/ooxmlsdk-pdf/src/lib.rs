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
        ..Default::default()
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
        list_label_hyperlink_url: None,
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![crate::docx::TextRun {
          text: "One Two Six".into(),
          style: crate::docx::TextStyle::default(),
          hyperlink_url: None,
          dynamic_field: None,
        }],
        inlines: vec![crate::docx::InlineItem::Text(crate::docx::TextRun {
          text: "One Two Six".into(),
          style: crate::docx::TextStyle::default(),
          hyperlink_url: None,
          dynamic_field: None,
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
  fn justified_wrapped_line_expands_word_spacing() {
    let run = crate::docx::TextRun {
      text: "One Two Three Four".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup {
        width_pt: 76.0,
        height_pt: 200.0,
        margin_left_pt: 10.0,
        margin_right_pt: 10.0,
        margin_top_pt: 10.0,
        margin_bottom_pt: 10.0,
        ..Default::default()
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
        inlines: vec![crate::docx::InlineItem::Text(run.clone())],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![run],
        format: crate::docx::ParagraphFormat {
          alignment: crate::docx::ParagraphAlignment::Justify,
          ..Default::default()
        },
        list_label: None,
        list_label_hyperlink_url: None,
      })],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let one = text_item(&layout, "One");
    let two = text_item(&layout, "Two");

    assert_eq!(one.y_pt, two.y_pt);
    assert!(two.x_pt - one.x_pt > 30.0);
  }

  #[test]
  fn paragraph_explicit_line_height_applies_to_each_line() {
    let run = crate::docx::TextRun {
      text: "Line A\nLine B\nLine C".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup {
        width_pt: 160.0,
        height_pt: 200.0,
        margin_left_pt: 10.0,
        margin_right_pt: 10.0,
        margin_top_pt: 10.0,
        margin_bottom_pt: 10.0,
        ..Default::default()
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
        inlines: vec![crate::docx::InlineItem::Text(run.clone())],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![run],
        format: crate::docx::ParagraphFormat {
          line_height_pt: Some(24.0),
          ..Default::default()
        },
        list_label: None,
        list_label_hyperlink_url: None,
      })],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let line_a = text_item(&layout, "Line A");
    let line_b = text_item(&layout, "Line B");
    let line_c = text_item(&layout, "Line C");

    assert!((line_b.y_pt - line_a.y_pt - 24.0).abs() < 0.1);
    assert!((line_c.y_pt - line_b.y_pt - 24.0).abs() < 0.1);
  }

  #[test]
  fn baseline_shift_contributes_to_line_advance() {
    let base = crate::docx::TextRun {
      text: "Base".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let shifted = crate::docx::TextRun {
      text: "2\n".into(),
      style: crate::docx::TextStyle {
        font_size_pt: 8.0,
        baseline_shift_pt: 12.0,
        ..Default::default()
      },
      hyperlink_url: None,
      dynamic_field: None,
    };
    let next = crate::docx::TextRun {
      text: "Next".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup {
        width_pt: 160.0,
        height_pt: 200.0,
        margin_left_pt: 10.0,
        margin_right_pt: 10.0,
        margin_top_pt: 10.0,
        margin_bottom_pt: 10.0,
        ..Default::default()
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
        inlines: vec![
          crate::docx::InlineItem::Text(base.clone()),
          crate::docx::InlineItem::Text(shifted.clone()),
          crate::docx::InlineItem::Text(next.clone()),
        ],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![base, shifted, next],
        format: crate::docx::ParagraphFormat::default(),
        list_label: None,
        list_label_hyperlink_url: None,
      })],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let base = text_item(&layout, "Base");
    let next = text_item(&layout, "Next");

    assert!(next.y_pt - base.y_pt >= 22.0);
  }

  #[test]
  fn exact_line_height_keeps_fixed_advance_but_at_least_expands() {
    fn line_gap(rule: crate::docx::LineHeightRule) -> f32 {
      let tall = crate::docx::TextRun {
        text: "Tall\n".into(),
        style: crate::docx::TextStyle {
          font_size_pt: 30.0,
          ..Default::default()
        },
        hyperlink_url: None,
        dynamic_field: None,
      };
      let next = crate::docx::TextRun {
        text: "Next".into(),
        style: crate::docx::TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
      };
      let doc = crate::docx::DocxDocument {
        page: crate::docx::PageSetup {
          width_pt: 160.0,
          height_pt: 200.0,
          margin_left_pt: 10.0,
          margin_right_pt: 10.0,
          margin_top_pt: 10.0,
          margin_bottom_pt: 10.0,
          ..Default::default()
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
          inlines: vec![
            crate::docx::InlineItem::Text(tall.clone()),
            crate::docx::InlineItem::Text(next.clone()),
          ],
          footnote_reference_ids: Vec::new(),
          endnote_reference_ids: Vec::new(),
          runs: vec![tall, next],
          format: crate::docx::ParagraphFormat {
            line_height_pt: Some(10.0),
            line_height_rule: rule,
            ..Default::default()
          },
          list_label: None,
          list_label_hyperlink_url: None,
        })],
      };

      let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
      let tall = text_item(&layout, "Tall");
      let next = text_item(&layout, "Next");
      next.y_pt - tall.y_pt
    }

    assert!((line_gap(crate::docx::LineHeightRule::Exact) - 10.0).abs() < 0.1);
    assert!(line_gap(crate::docx::LineHeightRule::AtLeast) > 35.0);
  }

  #[test]
  fn contextual_spacing_suppresses_adjacent_matching_paragraph_spacing() {
    fn paragraph(text: &str, contextual_spacing: bool) -> crate::docx::Paragraph {
      let run = crate::docx::TextRun {
        text: text.into(),
        style: crate::docx::TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
      };
      crate::docx::Paragraph {
        inlines: vec![crate::docx::InlineItem::Text(run.clone())],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![run],
        format: crate::docx::ParagraphFormat {
          spacing_before_pt: 12.0,
          spacing_after_pt: 24.0,
          contextual_spacing,
          ..Default::default()
        },
        list_label: None,
        list_label_hyperlink_url: None,
      }
    }

    fn paragraph_gap(contextual_spacing: bool) -> f32 {
      let doc = crate::docx::DocxDocument {
        page: crate::docx::PageSetup {
          width_pt: 160.0,
          height_pt: 200.0,
          margin_left_pt: 10.0,
          margin_right_pt: 10.0,
          margin_top_pt: 10.0,
          margin_bottom_pt: 10.0,
          ..Default::default()
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
        blocks: vec![
          crate::docx::Block::Paragraph(paragraph("First", contextual_spacing)),
          crate::docx::Block::Paragraph(paragraph("Second", contextual_spacing)),
        ],
      };

      let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
      let first = text_item(&layout, "First");
      let second = text_item(&layout, "Second");
      second.y_pt - first.y_pt
    }

    assert!(paragraph_gap(false) > 45.0);
    assert!((paragraph_gap(true) - 14.0).abs() < 0.1);
  }

  #[test]
  fn long_paragraph_continues_after_page_split() {
    let text = (1..=8)
      .map(|index| format!("Line {index:02}"))
      .collect::<Vec<_>>()
      .join("\n");
    let run = crate::docx::TextRun {
      text,
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup {
        width_pt: 160.0,
        height_pt: 60.0,
        margin_left_pt: 10.0,
        margin_right_pt: 10.0,
        margin_top_pt: 10.0,
        margin_bottom_pt: 10.0,
        ..Default::default()
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
        inlines: vec![crate::docx::InlineItem::Text(run.clone())],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![run],
        format: crate::docx::ParagraphFormat::default(),
        list_label: None,
        list_label_hyperlink_url: None,
      })],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();

    assert!(layout.pages.len() > 1);
    assert!(page_has_text(&layout.pages[0], "Line 01"));
    assert!(
      layout.pages[1..]
        .iter()
        .any(|page| page_has_text(page, "Line 06"))
    );
  }

  #[test]
  fn keep_lines_paragraph_reflows_to_next_page_when_split_rejected() {
    let intro = crate::docx::TextRun {
      text: "Intro 01\nIntro 02\nIntro 03".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let kept = crate::docx::TextRun {
      text: "Keep 01\nKeep 02\nKeep 03".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let keep_format = crate::docx::ParagraphFormat {
      keep_lines: true,
      ..Default::default()
    };
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup {
        width_pt: 160.0,
        height_pt: 80.0,
        margin_left_pt: 10.0,
        margin_right_pt: 10.0,
        margin_top_pt: 10.0,
        margin_bottom_pt: 10.0,
        ..Default::default()
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
      blocks: vec![
        crate::docx::Block::Paragraph(crate::docx::Paragraph {
          inlines: vec![crate::docx::InlineItem::Text(intro.clone())],
          footnote_reference_ids: Vec::new(),
          endnote_reference_ids: Vec::new(),
          runs: vec![intro],
          format: crate::docx::ParagraphFormat::default(),
          list_label: None,
          list_label_hyperlink_url: None,
        }),
        crate::docx::Block::Paragraph(crate::docx::Paragraph {
          inlines: vec![crate::docx::InlineItem::Text(kept.clone())],
          footnote_reference_ids: Vec::new(),
          endnote_reference_ids: Vec::new(),
          runs: vec![kept],
          format: keep_format,
          list_label: None,
          list_label_hyperlink_url: None,
        }),
      ],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();

    assert_eq!(layout.pages.len(), 2);
    assert!(page_has_text(&layout.pages[0], "Intro 03"));
    assert!(!page_has_text(&layout.pages[0], "Keep 01"));
    assert!(page_has_text(&layout.pages[1], "Keep 01"));
    assert!(page_has_text(&layout.pages[1], "Keep 03"));
  }

  #[test]
  fn paragraph_follow_flow_keeps_following_blocks_in_destination_column() {
    let page = crate::docx::PageSetup {
      width_pt: 300.0,
      height_pt: 100.0,
      margin_left_pt: 20.0,
      margin_right_pt: 20.0,
      margin_top_pt: 10.0,
      margin_bottom_pt: 10.0,
      ..Default::default()
    };
    let columns = crate::docx::SectionColumns {
      count: 2,
      gap_pt: 20.0,
      ..Default::default()
    };
    let long = crate::docx::TextRun {
      text: "Flow 01\nFlow 02\nFlow 03\nFlow 04\nFlow 05\nFlow 06\nFlow 07".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let after = crate::docx::TextRun {
      text: "After follow".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let long_block = crate::docx::Block::Paragraph(crate::docx::Paragraph {
      inlines: vec![crate::docx::InlineItem::Text(long.clone())],
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      runs: vec![long],
      format: crate::docx::ParagraphFormat::default(),
      list_label: None,
      list_label_hyperlink_url: None,
    });
    let after_block = crate::docx::Block::Paragraph(crate::docx::Paragraph {
      inlines: vec![crate::docx::InlineItem::Text(after.clone())],
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      runs: vec![after],
      format: crate::docx::ParagraphFormat::default(),
      list_label: None,
      list_label_hyperlink_url: None,
    });
    let doc = crate::docx::DocxDocument {
      page,
      default_tab_stop_pt: 36.0,
      even_and_odd_headers: false,
      sections: vec![crate::docx::ImportedSection {
        break_kind: crate::docx::SectionBreakKind::Continuous,
        section_properties: None,
        page,
        columns,
        title_page: false,
        header_blocks: Vec::new(),
        footer_blocks: Vec::new(),
        first_header_blocks: Vec::new(),
        first_footer_blocks: Vec::new(),
        even_header_blocks: Vec::new(),
        even_footer_blocks: Vec::new(),
        blocks: vec![long_block.clone(), after_block.clone()],
      }],
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
      blocks: vec![long_block, after_block],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let flowed = text_item(&layout, "Flow 07");
    let after = text_item(&layout, "After follow");

    assert_eq!(layout.pages.len(), 1);
    assert!(flowed.x_pt > page.margin_left_pt + 100.0);
    assert!((after.x_pt - flowed.x_pt).abs() < 1.0);
    assert!(after.y_pt > flowed.y_pt);
  }

  #[test]
  fn table_follow_flow_keeps_following_blocks_in_destination_column() {
    fn paragraph(text: &str) -> crate::docx::Paragraph {
      let run = crate::docx::TextRun {
        text: text.into(),
        style: crate::docx::TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
      };
      crate::docx::Paragraph {
        inlines: vec![crate::docx::InlineItem::Text(run.clone())],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![run],
        format: crate::docx::ParagraphFormat::default(),
        list_label: None,
        list_label_hyperlink_url: None,
      }
    }

    fn row(text: &str) -> crate::docx::TableRow {
      crate::docx::TableRow {
        cells: vec![crate::docx::TableCell {
          blocks: vec![crate::docx::Block::Paragraph(paragraph(text))],
          shading: None,
          borders: crate::docx::CellBordersModel::default(),
          margins: crate::docx::CellMargins::default(),
          preferred_width_pt: None,
          preferred_width_pct: None,
          grid_span: 1,
          vertical_merge_continue: false,
          vertical_alignment: crate::docx::TableCellVerticalAlignment::Top,
        }],
        height_pt: Some(30.0),
        exact_height: true,
        repeat_header: false,
        cant_split: false,
        cell_spacing_pt: None,
        grid_before: 0,
        grid_after: 0,
      }
    }

    let page = crate::docx::PageSetup {
      width_pt: 300.0,
      height_pt: 100.0,
      margin_left_pt: 20.0,
      margin_right_pt: 20.0,
      margin_top_pt: 10.0,
      margin_bottom_pt: 10.0,
      ..Default::default()
    };
    let columns = crate::docx::SectionColumns {
      count: 2,
      gap_pt: 20.0,
      ..Default::default()
    };
    let table_block = crate::docx::Block::Table(crate::docx::Table {
      column_widths_pt: vec![80.0],
      preferred_width_pt: None,
      preferred_width_pct: None,
      indent_left_pt: 0.0,
      alignment: crate::docx::TableAlignment::Left,
      borders: None,
      cell_spacing_pt: 0.0,
      rows: vec![row("R1"), row("R2"), row("R3")],
    });
    let after_block = crate::docx::Block::Paragraph(paragraph("After table"));
    let doc = crate::docx::DocxDocument {
      page,
      default_tab_stop_pt: 36.0,
      even_and_odd_headers: false,
      sections: vec![crate::docx::ImportedSection {
        break_kind: crate::docx::SectionBreakKind::Continuous,
        section_properties: None,
        page,
        columns,
        title_page: false,
        header_blocks: Vec::new(),
        footer_blocks: Vec::new(),
        first_header_blocks: Vec::new(),
        first_footer_blocks: Vec::new(),
        even_header_blocks: Vec::new(),
        even_footer_blocks: Vec::new(),
        blocks: vec![table_block.clone(), after_block.clone()],
      }],
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
      blocks: vec![table_block, after_block],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let followed_row = text_item(&layout, "R3");
    let after = text_item(&layout, "After table");

    assert_eq!(layout.pages.len(), 1);
    assert!(followed_row.x_pt > page.margin_left_pt + 100.0);
    assert!(after.x_pt > page.margin_left_pt + 100.0);
    assert!((after.x_pt - followed_row.x_pt).abs() <= crate::docx::CellMargins::default().left_pt);
    assert!(after.y_pt > followed_row.y_pt);
  }

  #[test]
  fn table_without_grid_uses_cell_preferred_widths() {
    fn paragraph(text: &str) -> crate::docx::Paragraph {
      let run = crate::docx::TextRun {
        text: text.into(),
        style: crate::docx::TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
      };
      crate::docx::Paragraph {
        inlines: vec![crate::docx::InlineItem::Text(run.clone())],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![run],
        format: crate::docx::ParagraphFormat::default(),
        list_label: None,
        list_label_hyperlink_url: None,
      }
    }

    fn cell(text: &str, width: f32) -> crate::docx::TableCell {
      crate::docx::TableCell {
        blocks: vec![crate::docx::Block::Paragraph(paragraph(text))],
        shading: None,
        borders: crate::docx::CellBordersModel::default(),
        margins: crate::docx::CellMargins {
          top_pt: 0.0,
          right_pt: 0.0,
          bottom_pt: 0.0,
          left_pt: 0.0,
        },
        preferred_width_pt: Some(width),
        preferred_width_pct: None,
        grid_span: 1,
        vertical_merge_continue: false,
        vertical_alignment: crate::docx::TableCellVerticalAlignment::Top,
      }
    }

    let table = crate::docx::Table {
      column_widths_pt: Vec::new(),
      preferred_width_pt: None,
      preferred_width_pct: None,
      indent_left_pt: 0.0,
      alignment: crate::docx::TableAlignment::Left,
      borders: None,
      cell_spacing_pt: 0.0,
      rows: vec![crate::docx::TableRow {
        cells: vec![cell("Narrow", 50.0), cell("Wide", 100.0)],
        height_pt: Some(24.0),
        exact_height: true,
        repeat_header: false,
        cant_split: false,
        cell_spacing_pt: None,
        grid_before: 0,
        grid_after: 0,
      }],
    };
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup::default(),
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
      blocks: vec![crate::docx::Block::Table(table)],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let narrow = text_item(&layout, "Narrow");
    let wide = text_item(&layout, "Wide");

    assert!((wide.x_pt - narrow.x_pt - 50.0).abs() < 0.1);
  }

  #[test]
  fn table_without_grid_distributes_spanned_cell_preferred_widths() {
    fn paragraph(text: &str) -> crate::docx::Paragraph {
      let run = crate::docx::TextRun {
        text: text.into(),
        style: crate::docx::TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
      };
      crate::docx::Paragraph {
        inlines: vec![crate::docx::InlineItem::Text(run.clone())],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![run],
        format: crate::docx::ParagraphFormat::default(),
        list_label: None,
        list_label_hyperlink_url: None,
      }
    }

    fn cell(text: &str, width: Option<f32>, span: usize) -> crate::docx::TableCell {
      crate::docx::TableCell {
        blocks: vec![crate::docx::Block::Paragraph(paragraph(text))],
        shading: None,
        borders: crate::docx::CellBordersModel::default(),
        margins: crate::docx::CellMargins {
          top_pt: 0.0,
          right_pt: 0.0,
          bottom_pt: 0.0,
          left_pt: 0.0,
        },
        preferred_width_pt: width,
        preferred_width_pct: None,
        grid_span: span,
        vertical_merge_continue: false,
        vertical_alignment: crate::docx::TableCellVerticalAlignment::Top,
      }
    }

    let table = crate::docx::Table {
      column_widths_pt: Vec::new(),
      preferred_width_pt: None,
      preferred_width_pct: None,
      indent_left_pt: 0.0,
      alignment: crate::docx::TableAlignment::Left,
      borders: None,
      cell_spacing_pt: 0.0,
      rows: vec![
        crate::docx::TableRow {
          cells: vec![
            cell("Span two", Some(180.0), 2),
            cell("Third", Some(60.0), 1),
          ],
          height_pt: Some(24.0),
          exact_height: true,
          repeat_header: false,
          cant_split: false,
          cell_spacing_pt: None,
          grid_before: 0,
          grid_after: 0,
        },
        crate::docx::TableRow {
          cells: vec![
            cell("Col 1", None, 1),
            cell("Col 2", None, 1),
            cell("Col 3", None, 1),
          ],
          height_pt: Some(24.0),
          exact_height: true,
          repeat_header: false,
          cant_split: false,
          cell_spacing_pt: None,
          grid_before: 0,
          grid_after: 0,
        },
      ],
    };
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup::default(),
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
      blocks: vec![crate::docx::Block::Table(table)],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let col_1 = text_item(&layout, "Col 1");
    let col_2 = text_item(&layout, "Col 2");
    let col_3 = text_item(&layout, "Col 3");

    assert!((col_2.x_pt - col_1.x_pt - 90.0).abs() < 0.1);
    assert!((col_3.x_pt - col_2.x_pt - 90.0).abs() < 0.1);
  }

  #[test]
  fn table_internal_borders_prefer_stronger_adjacent_border() {
    fn paragraph(text: &str) -> crate::docx::Paragraph {
      let run = crate::docx::TextRun {
        text: text.into(),
        style: crate::docx::TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
      };
      crate::docx::Paragraph {
        inlines: vec![crate::docx::InlineItem::Text(run.clone())],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![run],
        format: crate::docx::ParagraphFormat::default(),
        list_label: None,
        list_label_hyperlink_url: None,
      }
    }

    fn border(width_pt: f32) -> crate::docx::BorderStyle {
      crate::docx::BorderStyle {
        width_pt,
        spacing_pt: 0.0,
        color: crate::docx::RgbColor { r: 0, g: 0, b: 0 },
        compound: false,
      }
    }

    fn cell(text: &str, borders: crate::docx::CellBordersModel) -> crate::docx::TableCell {
      crate::docx::TableCell {
        blocks: vec![crate::docx::Block::Paragraph(paragraph(text))],
        shading: None,
        borders,
        margins: crate::docx::CellMargins::default(),
        preferred_width_pt: None,
        preferred_width_pct: None,
        grid_span: 1,
        vertical_merge_continue: false,
        vertical_alignment: crate::docx::TableCellVerticalAlignment::Top,
      }
    }

    let table = crate::docx::Table {
      column_widths_pt: vec![60.0, 60.0],
      preferred_width_pt: None,
      preferred_width_pct: None,
      indent_left_pt: 0.0,
      alignment: crate::docx::TableAlignment::Left,
      borders: None,
      cell_spacing_pt: 0.0,
      rows: vec![
        crate::docx::TableRow {
          cells: vec![
            cell(
              "A",
              crate::docx::CellBordersModel {
                right: Some(border(0.5)),
                bottom: Some(border(0.5)),
                ..Default::default()
              },
            ),
            cell(
              "B",
              crate::docx::CellBordersModel {
                left: Some(border(3.0)),
                bottom: Some(border(0.5)),
                ..Default::default()
              },
            ),
          ],
          height_pt: Some(24.0),
          exact_height: true,
          repeat_header: false,
          cant_split: false,
          cell_spacing_pt: None,
          grid_before: 0,
          grid_after: 0,
        },
        crate::docx::TableRow {
          cells: vec![
            cell(
              "C",
              crate::docx::CellBordersModel {
                top: Some(border(2.5)),
                ..Default::default()
              },
            ),
            cell("D", crate::docx::CellBordersModel::default()),
          ],
          height_pt: Some(24.0),
          exact_height: true,
          repeat_header: false,
          cant_split: false,
          cell_spacing_pt: None,
          grid_before: 0,
          grid_after: 0,
        },
      ],
    };
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup::default(),
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
      blocks: vec![crate::docx::Block::Table(table)],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let line_widths = layout.pages[0]
      .items
      .iter()
      .filter_map(|item| match item {
        crate::layout::PageItem::Line(line) => Some(line.width_pt),
        _ => None,
      })
      .collect::<Vec<_>>();

    assert!(line_widths.iter().any(|width| (*width - 3.0).abs() < 0.001));
    assert!(line_widths.iter().any(|width| (*width - 2.5).abs() < 0.001));
  }

  #[test]
  fn table_cell_spacing_separates_cell_frames() {
    fn paragraph(text: &str) -> crate::docx::Paragraph {
      let run = crate::docx::TextRun {
        text: text.into(),
        style: crate::docx::TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
      };
      crate::docx::Paragraph {
        inlines: vec![crate::docx::InlineItem::Text(run.clone())],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![run],
        format: crate::docx::ParagraphFormat::default(),
        list_label: None,
        list_label_hyperlink_url: None,
      }
    }

    fn cell(text: &str) -> crate::docx::TableCell {
      crate::docx::TableCell {
        blocks: vec![crate::docx::Block::Paragraph(paragraph(text))],
        shading: None,
        borders: crate::docx::CellBordersModel::default(),
        margins: crate::docx::CellMargins::default(),
        preferred_width_pt: None,
        preferred_width_pct: None,
        grid_span: 1,
        vertical_merge_continue: false,
        vertical_alignment: crate::docx::TableCellVerticalAlignment::Top,
      }
    }

    let table = crate::docx::Table {
      column_widths_pt: vec![40.0, 40.0],
      preferred_width_pt: None,
      preferred_width_pct: None,
      indent_left_pt: 0.0,
      alignment: crate::docx::TableAlignment::Left,
      borders: Some(crate::docx::TableBordersModel {
        inside_vertical: Some(crate::docx::BorderStyle::default()),
        ..crate::docx::TableBordersModel::default()
      }),
      cell_spacing_pt: 12.0,
      rows: vec![crate::docx::TableRow {
        cells: vec![cell("A"), cell("B")],
        height_pt: Some(24.0),
        exact_height: true,
        repeat_header: false,
        cant_split: false,
        cell_spacing_pt: None,
        grid_before: 0,
        grid_after: 0,
      }],
    };
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup::default(),
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
      blocks: vec![crate::docx::Block::Table(table)],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let a = text_item(&layout, "A");
    let b = text_item(&layout, "B");
    assert!((b.x_pt - a.x_pt - 52.0).abs() < 0.001);
    assert!(layout.pages[0].items.iter().any(|item| {
      matches!(
        item,
        crate::layout::PageItem::Line(line)
          if (line.x1_pt - (doc.page.margin_left_pt + 40.0)).abs() < 0.001
      )
    }));
  }

  #[test]
  fn table_row_grid_before_offsets_visible_cells() {
    fn paragraph(text: &str) -> crate::docx::Paragraph {
      let run = crate::docx::TextRun {
        text: text.into(),
        style: crate::docx::TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
      };
      crate::docx::Paragraph {
        inlines: vec![crate::docx::InlineItem::Text(run.clone())],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![run],
        format: crate::docx::ParagraphFormat::default(),
        list_label: None,
        list_label_hyperlink_url: None,
      }
    }

    let table = crate::docx::Table {
      column_widths_pt: vec![40.0, 40.0, 40.0],
      preferred_width_pt: None,
      preferred_width_pct: None,
      indent_left_pt: 0.0,
      alignment: crate::docx::TableAlignment::Left,
      borders: None,
      cell_spacing_pt: 0.0,
      rows: vec![crate::docx::TableRow {
        cells: vec![crate::docx::TableCell {
          blocks: vec![crate::docx::Block::Paragraph(paragraph("B"))],
          shading: None,
          borders: crate::docx::CellBordersModel::default(),
          margins: crate::docx::CellMargins::default(),
          preferred_width_pt: None,
          preferred_width_pct: None,
          grid_span: 1,
          vertical_merge_continue: false,
          vertical_alignment: crate::docx::TableCellVerticalAlignment::Top,
        }],
        height_pt: Some(24.0),
        exact_height: true,
        repeat_header: false,
        cant_split: false,
        cell_spacing_pt: None,
        grid_before: 1,
        grid_after: 1,
      }],
    };
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup::default(),
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
      blocks: vec![crate::docx::Block::Table(table)],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let item = text_item(&layout, "B");
    assert!((item.x_pt - (doc.page.margin_left_pt + 40.0 + 5.4)).abs() < 0.001);
  }

  #[test]
  fn vertical_merge_continuation_keeps_origin_cell_shading() {
    fn paragraph(text: &str) -> crate::docx::Paragraph {
      let run = crate::docx::TextRun {
        text: text.into(),
        style: crate::docx::TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
      };
      crate::docx::Paragraph {
        inlines: vec![crate::docx::InlineItem::Text(run.clone())],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![run],
        format: crate::docx::ParagraphFormat::default(),
        list_label: None,
        list_label_hyperlink_url: None,
      }
    }

    fn cell(
      text: &str,
      shading: Option<crate::docx::RgbColor>,
      merge: bool,
    ) -> crate::docx::TableCell {
      crate::docx::TableCell {
        blocks: vec![crate::docx::Block::Paragraph(paragraph(text))],
        shading,
        borders: crate::docx::CellBordersModel::default(),
        margins: crate::docx::CellMargins::default(),
        preferred_width_pt: None,
        preferred_width_pct: None,
        grid_span: 1,
        vertical_merge_continue: merge,
        vertical_alignment: crate::docx::TableCellVerticalAlignment::Top,
      }
    }

    let shade = crate::docx::RgbColor {
      r: 0xDE,
      g: 0xEA,
      b: 0xF1,
    };
    let table = crate::docx::Table {
      column_widths_pt: vec![80.0, 80.0],
      preferred_width_pt: None,
      preferred_width_pct: None,
      indent_left_pt: 0.0,
      alignment: crate::docx::TableAlignment::Left,
      borders: None,
      cell_spacing_pt: 0.0,
      rows: vec![
        crate::docx::TableRow {
          cells: vec![
            cell("Merge origin", Some(shade), false),
            cell("A", None, false),
          ],
          height_pt: Some(24.0),
          exact_height: true,
          repeat_header: false,
          cant_split: false,
          cell_spacing_pt: None,
          grid_before: 0,
          grid_after: 0,
        },
        crate::docx::TableRow {
          cells: vec![cell("", None, true), cell("B", None, false)],
          height_pt: Some(24.0),
          exact_height: true,
          repeat_header: false,
          cant_split: false,
          cell_spacing_pt: None,
          grid_before: 0,
          grid_after: 0,
        },
      ],
    };
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup::default(),
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
      blocks: vec![crate::docx::Block::Table(table)],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let shaded_fragments = layout.pages[0]
      .items
      .iter()
      .filter(|item| {
        matches!(
          item,
          crate::layout::PageItem::Fill(fill)
            if fill.color.r == shade.r && fill.color.g == shade.g && fill.color.b == shade.b
        )
      })
      .count();

    assert_eq!(shaded_fragments, 2);
    assert!(find_text_item(&layout, "Merge origin").is_some());
  }

  #[test]
  fn cant_split_table_row_moves_once_to_follow_column() {
    fn paragraph(text: &str) -> crate::docx::Paragraph {
      let run = crate::docx::TextRun {
        text: text.into(),
        style: crate::docx::TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
      };
      crate::docx::Paragraph {
        inlines: vec![crate::docx::InlineItem::Text(run.clone())],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![run],
        format: crate::docx::ParagraphFormat::default(),
        list_label: None,
        list_label_hyperlink_url: None,
      }
    }

    fn row(text: &str, height_pt: f32, cant_split: bool) -> crate::docx::TableRow {
      crate::docx::TableRow {
        cells: vec![crate::docx::TableCell {
          blocks: vec![crate::docx::Block::Paragraph(paragraph(text))],
          shading: None,
          borders: crate::docx::CellBordersModel::default(),
          margins: crate::docx::CellMargins::default(),
          preferred_width_pt: None,
          preferred_width_pct: None,
          grid_span: 1,
          vertical_merge_continue: false,
          vertical_alignment: crate::docx::TableCellVerticalAlignment::Top,
        }],
        height_pt: Some(height_pt),
        exact_height: true,
        repeat_header: false,
        cant_split,
        cell_spacing_pt: None,
        grid_before: 0,
        grid_after: 0,
      }
    }

    let page = crate::docx::PageSetup {
      width_pt: 300.0,
      height_pt: 100.0,
      margin_left_pt: 20.0,
      margin_right_pt: 20.0,
      margin_top_pt: 10.0,
      margin_bottom_pt: 10.0,
      ..Default::default()
    };
    let columns = crate::docx::SectionColumns {
      count: 2,
      gap_pt: 20.0,
      ..Default::default()
    };
    let table_block = crate::docx::Block::Table(crate::docx::Table {
      column_widths_pt: vec![80.0],
      preferred_width_pt: None,
      preferred_width_pct: None,
      indent_left_pt: 0.0,
      alignment: crate::docx::TableAlignment::Left,
      borders: None,
      cell_spacing_pt: 0.0,
      rows: vec![row("Before row", 50.0, false), row("Keep row", 50.0, true)],
    });
    let doc = crate::docx::DocxDocument {
      page,
      default_tab_stop_pt: 36.0,
      even_and_odd_headers: false,
      sections: vec![crate::docx::ImportedSection {
        break_kind: crate::docx::SectionBreakKind::Continuous,
        section_properties: None,
        page,
        columns,
        title_page: false,
        header_blocks: Vec::new(),
        footer_blocks: Vec::new(),
        first_header_blocks: Vec::new(),
        first_footer_blocks: Vec::new(),
        even_header_blocks: Vec::new(),
        even_footer_blocks: Vec::new(),
        blocks: vec![table_block.clone()],
      }],
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
      blocks: vec![table_block],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let before = text_item(&layout, "Before row");
    let kept = text_item(&layout, "Keep row");

    assert_eq!(layout.pages.len(), 1);
    assert!(layout.follows.iter().any(|follow| {
      follow.kind == crate::layout::FollowFrameKind::Table
        && follow.reason == crate::layout::FollowReason::Overflow
        && follow.from_column_index == 0
        && follow.to_column_index == 1
    }));
    assert!(before.x_pt < page.margin_left_pt + 100.0);
    assert!(kept.x_pt > page.margin_left_pt + 100.0);
    assert!(kept.y_pt <= page.margin_top_pt + crate::docx::CellMargins::default().top_pt + 12.0);
  }

  #[test]
  fn splittable_table_row_continues_cell_content_on_follow_page() {
    fn paragraph(text: &str) -> crate::docx::Paragraph {
      let run = crate::docx::TextRun {
        text: text.into(),
        style: crate::docx::TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
      };
      crate::docx::Paragraph {
        inlines: vec![crate::docx::InlineItem::Text(run.clone())],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![run],
        format: crate::docx::ParagraphFormat::default(),
        list_label: None,
        list_label_hyperlink_url: None,
      }
    }

    let line_blocks = (1..=6)
      .map(|index| crate::docx::Block::Paragraph(paragraph(&format!("Line {index:02}"))))
      .collect::<Vec<_>>();
    let table = crate::docx::Table {
      column_widths_pt: vec![120.0],
      preferred_width_pt: None,
      preferred_width_pct: None,
      indent_left_pt: 0.0,
      alignment: crate::docx::TableAlignment::Left,
      borders: None,
      cell_spacing_pt: 0.0,
      rows: vec![crate::docx::TableRow {
        cells: vec![crate::docx::TableCell {
          blocks: line_blocks,
          shading: None,
          borders: crate::docx::CellBordersModel::default(),
          margins: crate::docx::CellMargins::default(),
          preferred_width_pt: None,
          preferred_width_pct: None,
          grid_span: 1,
          vertical_merge_continue: false,
          vertical_alignment: crate::docx::TableCellVerticalAlignment::Top,
        }],
        height_pt: Some(140.0),
        exact_height: true,
        repeat_header: false,
        cant_split: false,
        cell_spacing_pt: None,
        grid_before: 0,
        grid_after: 0,
      }],
    };
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup {
        width_pt: 180.0,
        height_pt: 90.0,
        margin_left_pt: 10.0,
        margin_right_pt: 10.0,
        margin_top_pt: 10.0,
        margin_bottom_pt: 10.0,
        ..Default::default()
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
      blocks: vec![
        crate::docx::Block::Paragraph(paragraph("Before table")),
        crate::docx::Block::Table(table),
      ],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let line_01_page = text_page_index(&layout, "Line 01").expect("first row fragment");
    let line_04_page = text_page_index(&layout, "Line 04").expect("follow row fragment");

    assert_eq!(line_01_page, 0);
    assert!(line_04_page > line_01_page);
    assert!(layout.follows.iter().any(|follow| {
      follow.kind == crate::layout::FollowFrameKind::Table
        && follow.reason == crate::layout::FollowReason::Overflow
        && follow.to_page_index > follow.from_page_index
    }));
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
        ..Default::default()
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
        list_label_hyperlink_url: None,
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![crate::docx::TextRun {
          text: "Left\tRight".into(),
          style: crate::docx::TextStyle::default(),
          hyperlink_url: None,
          dynamic_field: None,
        }],
        inlines: vec![crate::docx::InlineItem::Text(crate::docx::TextRun {
          text: "Left\tRight".into(),
          style: crate::docx::TextStyle::default(),
          hyperlink_url: None,
          dynamic_field: None,
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
        ..Default::default()
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
        list_label_hyperlink_url: None,
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![crate::docx::TextRun {
          text: "Title\t99".into(),
          style: crate::docx::TextStyle::default(),
          hyperlink_url: None,
          dynamic_field: None,
        }],
        inlines: vec![crate::docx::InlineItem::Text(crate::docx::TextRun {
          text: "Title\t99".into(),
          style: crate::docx::TextStyle::default(),
          hyperlink_url: None,
          dynamic_field: None,
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
  }

  #[test]
  fn paragraph_styles_inherit_doc_defaults_and_based_on_chain() {
    let path = fixture_path("test-data/wml/style_inheritance.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();

    let normal = paragraph_at(&doc, 0);
    assert_eq!(normal.runs[0].style.font_size_pt, 11.0);
    assert_eq!(normal.runs[0].style.font_family.as_deref(), Some("Calibri"));
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
    assert_eq!(
      paragraph.runs[0].style.font_family.as_deref(),
      Some("Arial")
    );
    assert_eq!(
      paragraph.runs[1].style.font_family.as_deref(),
      Some("SimSun")
    );
    assert_eq!(
      paragraph.runs[2].style.font_family.as_deref(),
      Some("Times New Roman")
    );
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
  }

  #[test]
  fn header_area_reduces_body_text_region_when_margins_overlap() {
    fn paragraph(text: &str) -> crate::docx::Paragraph {
      let run = crate::docx::TextRun {
        text: text.into(),
        style: crate::docx::TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
      };
      crate::docx::Paragraph {
        inlines: vec![crate::docx::InlineItem::Text(run.clone())],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![run],
        format: crate::docx::ParagraphFormat::default(),
        list_label: None,
        list_label_hyperlink_url: None,
      }
    }

    let page = crate::docx::PageSetup {
      margin_top_pt: 20.0,
      header_distance_pt: 30.0,
      ..Default::default()
    };
    let doc = crate::docx::DocxDocument {
      page,
      default_tab_stop_pt: 36.0,
      even_and_odd_headers: false,
      sections: Vec::new(),
      title_page: false,
      header_blocks: vec![crate::docx::Block::Paragraph(paragraph("Tall header"))],
      first_header_blocks: Vec::new(),
      footer_blocks: Vec::new(),
      first_footer_blocks: Vec::new(),
      footnote_blocks: Vec::new(),
      footnotes: Default::default(),
      endnote_blocks: Vec::new(),
      endnotes: Default::default(),
      comment_blocks: Vec::new(),
      blocks: vec![crate::docx::Block::Paragraph(paragraph(
        "Body below header",
      ))],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let body = text_item(&layout, "Body below header");

    assert!(body.y_pt >= page.header_distance_pt + 72.0 / 25.4);
  }

  #[test]
  fn body_region_uses_page_specific_repeating_header_slots() {
    fn paragraph(text: &str, page_break_before: bool) -> crate::docx::Paragraph {
      let run = crate::docx::TextRun {
        text: text.into(),
        style: crate::docx::TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
      };
      crate::docx::Paragraph {
        inlines: vec![crate::docx::InlineItem::Text(run.clone())],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![run],
        format: crate::docx::ParagraphFormat {
          page_break_before,
          ..Default::default()
        },
        list_label: None,
        list_label_hyperlink_url: None,
      }
    }

    let page = crate::docx::PageSetup {
      width_pt: 200.0,
      height_pt: 120.0,
      margin_top_pt: 20.0,
      margin_bottom_pt: 20.0,
      header_distance_pt: 30.0,
      ..Default::default()
    };
    let doc = crate::docx::DocxDocument {
      page,
      default_tab_stop_pt: 36.0,
      even_and_odd_headers: false,
      sections: Vec::new(),
      title_page: true,
      header_blocks: Vec::new(),
      first_header_blocks: vec![crate::docx::Block::Paragraph(paragraph(
        "First only header",
        false,
      ))],
      footer_blocks: Vec::new(),
      first_footer_blocks: Vec::new(),
      footnote_blocks: Vec::new(),
      footnotes: Default::default(),
      endnote_blocks: Vec::new(),
      endnotes: Default::default(),
      comment_blocks: Vec::new(),
      blocks: vec![
        crate::docx::Block::Paragraph(paragraph("F1", false)),
        crate::docx::Block::Paragraph(paragraph("S2", true)),
      ],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let first = text_item(&layout, "F1");
    let second = text_item(&layout, "S2");

    assert!(first.y_pt >= page.header_distance_pt + 72.0 / 25.4);
    assert!((second.y_pt - page.margin_top_pt).abs() < 0.1);
  }

  #[test]
  fn title_page_header_does_not_repeat_after_hard_page_break() {
    fn paragraph(text: &str, page_break_before: bool) -> crate::docx::Paragraph {
      let run = crate::docx::TextRun {
        text: text.into(),
        style: crate::docx::TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
      };
      crate::docx::Paragraph {
        inlines: vec![crate::docx::InlineItem::Text(run.clone())],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![run],
        format: crate::docx::ParagraphFormat {
          page_break_before,
          ..Default::default()
        },
        list_label: None,
        list_label_hyperlink_url: None,
      }
    }

    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup {
        width_pt: 240.0,
        height_pt: 240.0,
        margin_top_pt: 48.0,
        margin_bottom_pt: 36.0,
        header_distance_pt: 24.0,
        ..Default::default()
      },
      default_tab_stop_pt: 36.0,
      even_and_odd_headers: false,
      sections: Vec::new(),
      title_page: true,
      header_blocks: vec![crate::docx::Block::Paragraph(paragraph(
        "Default Header",
        false,
      ))],
      first_header_blocks: vec![crate::docx::Block::Paragraph(paragraph(
        "First Header",
        false,
      ))],
      footer_blocks: Vec::new(),
      first_footer_blocks: Vec::new(),
      footnote_blocks: Vec::new(),
      footnotes: Default::default(),
      endnote_blocks: Vec::new(),
      endnotes: Default::default(),
      comment_blocks: Vec::new(),
      blocks: vec![
        crate::docx::Block::Paragraph(paragraph("Page One", false)),
        crate::docx::Block::Paragraph(paragraph("Page Two", true)),
      ],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    assert!(layout.pages.len() >= 2);
    assert!(page_has_text(&layout.pages[0], "First Header"));
    assert!(!page_has_text(&layout.pages[0], "Default Header"));
    assert!(page_has_text(&layout.pages[1], "Default Header"));
    assert!(!page_has_text(&layout.pages[1], "First Header"));
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
  }

  #[test]
  fn inline_column_break_continues_same_paragraph_in_next_column() {
    let page = crate::docx::PageSetup {
      width_pt: 300.0,
      height_pt: 400.0,
      margin_left_pt: 20.0,
      margin_right_pt: 20.0,
      margin_top_pt: 20.0,
      margin_bottom_pt: 20.0,
      ..Default::default()
    };
    let mut columns = crate::docx::SectionColumns {
      count: 2,
      gap_pt: 20.0,
      ..Default::default()
    };
    columns.explicit_count = 0;
    let before = crate::docx::TextRun {
      text: "Inline before".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let after = crate::docx::TextRun {
      text: "Inline after".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let paragraph = crate::docx::Paragraph {
      inlines: vec![
        crate::docx::InlineItem::Text(before.clone()),
        crate::docx::InlineItem::ColumnBreak,
        crate::docx::InlineItem::Text(after.clone()),
      ],
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      runs: vec![before, after],
      format: crate::docx::ParagraphFormat::default(),
      list_label: None,
      list_label_hyperlink_url: None,
    };
    let block = crate::docx::Block::Paragraph(paragraph);
    let doc = crate::docx::DocxDocument {
      page,
      default_tab_stop_pt: 36.0,
      even_and_odd_headers: false,
      sections: vec![crate::docx::ImportedSection {
        break_kind: crate::docx::SectionBreakKind::Continuous,
        section_properties: None,
        page,
        columns,
        title_page: false,
        header_blocks: Vec::new(),
        footer_blocks: Vec::new(),
        first_header_blocks: Vec::new(),
        first_footer_blocks: Vec::new(),
        even_header_blocks: Vec::new(),
        even_footer_blocks: Vec::new(),
        blocks: vec![block.clone()],
      }],
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
      blocks: vec![block],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let before = text_item(&layout, "Inline before");
    let after = text_item(&layout, "Inline after");

    assert_eq!(layout.pages.len(), 1);
    assert!(after.x_pt > before.x_pt + 100.0);
    assert!(after.y_pt <= page.margin_top_pt + 1.0);
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
      text_item(&layout, "Visit example.com")
        .hyperlink_url
        .as_deref(),
      Some("https://example.com")
    );
    assert_eq!(
      text_item(&layout, "Go to target section").style.color,
      crate::docx::RgbColor {
        r: 0x05,
        g: 0x63,
        b: 0xC1,
      }
    );
    assert!(text_item(&layout, "Go to target section").style.underline);
    assert_eq!(
      text_item(&layout, "Go to target section").hyperlink_url,
      None
    );
  }

  #[test]
  fn complex_page_fields_are_resolved_from_layout_pages() {
    let path = fixture_path("test-data/wml/fields_complex.docx");
    let mut package = WordprocessingDocument::new(File::open(&path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();

    assert!(text_item(&layout, "1").dynamic_field.is_some());
    assert!(page_has_text(&layout.pages[0], "Page "));
    assert!(page_has_text(&layout.pages[0], " of "));
    assert_eq!(
      layout.pages[0]
        .items
        .iter()
        .filter(|item| matches!(item, crate::layout::PageItem::Text(text) if text.text == "1"))
        .count(),
      2
    );
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
    let first_footnote = match &doc.footnotes[&1][0] {
      crate::docx::Block::Paragraph(paragraph) => paragraph,
      crate::docx::Block::Table(_) => panic!("expected footnote paragraph"),
    };
    assert!(first_footnote.list_label.is_none());
    assert!(matches!(
      first_footnote.inlines.first(),
      Some(crate::docx::InlineItem::Text(run))
        if run.text == "1 " && run.style.baseline_shift_pt > 0.0
    ));
    assert!(find_text_item(&layout, "1").is_some());
    assert!(find_text_item(&layout, "2").is_some());
    assert!(find_text_item(&layout, " First footnote content.").is_some());
    assert!(find_text_item(&layout, " Second footnote content.").is_some());
    assert!(
      text_item(&layout, " First footnote content.").y_pt
        > doc.page.height_pt - doc.page.margin_bottom_pt - 120.0
    );

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

    assert!(layout_contains_text(&layout, "inserted words"));
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

    assert!(layout_contains_text(&layout, "Jane Smith"));
    assert!(layout_contains_text(&layout, "5/2/2026"));
    assert!(layout_contains_text(&layout, "Active"));
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
  }

  #[test]
  fn inline_image_effect_extent_does_not_expand_visible_bounds() {
    let image = crate::docx::InlineImage {
      data: vec![0; 8],
      content_type: Some("image/png".into()),
      width_pt: 36.0,
      height_pt: 24.0,
      effect_left_pt: 5.0,
      effect_top_pt: 10.0,
      effect_right_pt: 15.0,
      effect_bottom_pt: 20.0,
      crop: crate::docx::ImageCrop::default(),
      rotation_deg: 0.0,
      flip_horizontal: false,
      flip_vertical: false,
      alt_text: Some("inline effect".into()),
      hyperlink_url: Some("http://example.com".into()),
      placement: crate::docx::ImagePlacement::Inline,
    };
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup::default(),
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
        inlines: vec![crate::docx::InlineItem::Image(image)],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: Vec::new(),
        format: crate::docx::ParagraphFormat::default(),
        list_label: None,
        list_label_hyperlink_url: None,
      })],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let image = layout.pages[0]
      .items
      .iter()
      .find_map(|item| match item {
        crate::layout::PageItem::Image(image) => Some(image),
        crate::layout::PageItem::Text(_)
        | crate::layout::PageItem::Fill(_)
        | crate::layout::PageItem::Line(_) => None,
      })
      .expect("inline image");

    assert!(!image.floating);
    assert!((image.width_pt - 36.0).abs() < 0.1);
    assert!((image.height_pt - 24.0).abs() < 0.1);
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
    let wrapped_text = text_item(&layout, "Text beside the floating image.");
    assert!(wrapped_text.x_pt >= laid_out_image.x_pt + laid_out_image.width_pt);
  }

  #[test]
  fn square_floating_image_wrap_influences_following_paragraphs_on_page() {
    let intro = crate::docx::TextRun {
      text: "Anchor ".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let follow = crate::docx::TextRun {
      text: "Following paragraph beside float".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let image = crate::docx::InlineImage {
      data: vec![0; 8],
      content_type: Some("image/png".into()),
      width_pt: 72.0,
      height_pt: 60.0,
      effect_left_pt: 0.0,
      effect_top_pt: 0.0,
      effect_right_pt: 0.0,
      effect_bottom_pt: 0.0,
      crop: crate::docx::ImageCrop::default(),
      rotation_deg: 0.0,
      flip_horizontal: false,
      flip_vertical: false,
      alt_text: Some("page wrap".into()),
      hyperlink_url: None,
      placement: crate::docx::ImagePlacement::Floating(crate::docx::FloatingImagePlacement {
        horizontal_relative_to: crate::docx::HorizontalImageReference::Column,
        vertical_relative_to: crate::docx::VerticalImageReference::Paragraph,
        horizontal_alignment: None,
        vertical_alignment: None,
        horizontal_offset_pt: 0.0,
        vertical_offset_pt: 0.0,
        wrap: crate::docx::ImageWrapMode::Square,
        wrap_side: crate::docx::ImageWrapSide::BothSides,
        behind_text: false,
        margin_top_pt: 0.0,
        margin_right_pt: 6.0,
        margin_bottom_pt: 0.0,
        margin_left_pt: 0.0,
      }),
    };
    let page = crate::docx::PageSetup {
      width_pt: 240.0,
      height_pt: 240.0,
      margin_left_pt: 20.0,
      margin_right_pt: 20.0,
      margin_top_pt: 20.0,
      margin_bottom_pt: 20.0,
      ..Default::default()
    };
    let anchor = crate::docx::Paragraph {
      inlines: vec![
        crate::docx::InlineItem::Text(intro.clone()),
        crate::docx::InlineItem::Image(image),
      ],
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      runs: vec![intro],
      format: crate::docx::ParagraphFormat::default(),
      list_label: None,
      list_label_hyperlink_url: None,
    };
    let following = crate::docx::Paragraph {
      inlines: vec![crate::docx::InlineItem::Text(follow.clone())],
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      runs: vec![follow],
      format: crate::docx::ParagraphFormat::default(),
      list_label: None,
      list_label_hyperlink_url: None,
    };
    let doc = crate::docx::DocxDocument {
      page,
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
      blocks: vec![
        crate::docx::Block::Paragraph(anchor),
        crate::docx::Block::Paragraph(following),
      ],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let following = layout
      .pages
      .iter()
      .flat_map(|page| &page.items)
      .find_map(|item| match item {
        crate::layout::PageItem::Text(text) if text.text.starts_with("Following") => Some(text),
        _ => None,
      })
      .expect("following paragraph text");

    assert!(
      following.x_pt >= page.margin_left_pt + 72.0,
      "following paragraph x={}, y={}",
      following.x_pt,
      following.y_pt
    );
  }

  #[test]
  fn behind_text_floating_images_are_ordered_before_body_text() {
    let before = crate::docx::TextRun {
      text: "Before image ".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let after = crate::docx::TextRun {
      text: "After image".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let image = crate::docx::InlineImage {
      data: vec![0; 8],
      content_type: Some("image/png".into()),
      width_pt: 36.0,
      height_pt: 36.0,
      effect_left_pt: 0.0,
      effect_top_pt: 0.0,
      effect_right_pt: 0.0,
      effect_bottom_pt: 0.0,
      crop: crate::docx::ImageCrop::default(),
      rotation_deg: 0.0,
      flip_horizontal: false,
      flip_vertical: false,
      alt_text: Some("behind".into()),
      hyperlink_url: None,
      placement: crate::docx::ImagePlacement::Floating(crate::docx::FloatingImagePlacement {
        horizontal_relative_to: crate::docx::HorizontalImageReference::Column,
        vertical_relative_to: crate::docx::VerticalImageReference::Paragraph,
        horizontal_alignment: None,
        vertical_alignment: None,
        horizontal_offset_pt: 0.0,
        vertical_offset_pt: 0.0,
        wrap: crate::docx::ImageWrapMode::Through,
        wrap_side: crate::docx::ImageWrapSide::BothSides,
        behind_text: true,
        margin_top_pt: 0.0,
        margin_right_pt: 0.0,
        margin_bottom_pt: 0.0,
        margin_left_pt: 0.0,
      }),
    };
    let paragraph = crate::docx::Paragraph {
      inlines: vec![
        crate::docx::InlineItem::Text(before.clone()),
        crate::docx::InlineItem::Image(image),
        crate::docx::InlineItem::Text(after.clone()),
      ],
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      runs: vec![before, after],
      format: crate::docx::ParagraphFormat::default(),
      list_label: None,
      list_label_hyperlink_url: None,
    };
    let block = crate::docx::Block::Paragraph(paragraph);
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup::default(),
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
      blocks: vec![block],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let page = &layout.pages[0];
    let image_index = page
      .items
      .iter()
      .position(|item| matches!(item, crate::layout::PageItem::Image(image) if image.behind_text))
      .expect("behind image");
    let text_index = page
      .items
      .iter()
      .position(
        |item| matches!(item, crate::layout::PageItem::Text(text) if text.text == "Before image "),
      )
      .expect("body text");

    assert!(image_index < text_index);
  }

  #[test]
  fn foreground_floating_images_are_ordered_after_body_text() {
    let before = crate::docx::TextRun {
      text: "Before foreground ".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let after = crate::docx::TextRun {
      text: "After foreground".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let image = crate::docx::InlineImage {
      data: vec![0; 8],
      content_type: Some("image/png".into()),
      width_pt: 36.0,
      height_pt: 36.0,
      effect_left_pt: 0.0,
      effect_top_pt: 0.0,
      effect_right_pt: 0.0,
      effect_bottom_pt: 0.0,
      crop: crate::docx::ImageCrop::default(),
      rotation_deg: 0.0,
      flip_horizontal: false,
      flip_vertical: false,
      alt_text: Some("foreground".into()),
      hyperlink_url: None,
      placement: crate::docx::ImagePlacement::Floating(crate::docx::FloatingImagePlacement {
        horizontal_relative_to: crate::docx::HorizontalImageReference::Column,
        vertical_relative_to: crate::docx::VerticalImageReference::Paragraph,
        horizontal_alignment: None,
        vertical_alignment: None,
        horizontal_offset_pt: 0.0,
        vertical_offset_pt: 0.0,
        wrap: crate::docx::ImageWrapMode::Through,
        wrap_side: crate::docx::ImageWrapSide::BothSides,
        behind_text: false,
        margin_top_pt: 0.0,
        margin_right_pt: 0.0,
        margin_bottom_pt: 0.0,
        margin_left_pt: 0.0,
      }),
    };
    let paragraph = crate::docx::Paragraph {
      inlines: vec![
        crate::docx::InlineItem::Text(before.clone()),
        crate::docx::InlineItem::Image(image),
        crate::docx::InlineItem::Text(after.clone()),
      ],
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      runs: vec![before, after],
      format: crate::docx::ParagraphFormat::default(),
      list_label: None,
      list_label_hyperlink_url: None,
    };
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup::default(),
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
      blocks: vec![crate::docx::Block::Paragraph(paragraph)],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let page = &layout.pages[0];
    let image_index = page
      .items
      .iter()
      .position(|item| matches!(item, crate::layout::PageItem::Image(image) if image.floating))
      .expect("foreground image");
    let text_index = page
      .items
      .iter()
      .position(
        |item| matches!(item, crate::layout::PageItem::Text(text) if text.text == "After foreground"),
      )
      .expect("body text");

    assert!(image_index > text_index);
  }

  #[test]
  fn floating_image_effect_extent_aligns_inside_relative_margin_area() {
    let run = crate::docx::TextRun {
      text: "Aligned float".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let image = crate::docx::InlineImage {
      data: vec![0; 8],
      content_type: Some("image/png".into()),
      width_pt: 40.0,
      height_pt: 50.0,
      effect_left_pt: 5.0,
      effect_top_pt: 10.0,
      effect_right_pt: 15.0,
      effect_bottom_pt: 20.0,
      crop: crate::docx::ImageCrop::default(),
      rotation_deg: 0.0,
      flip_horizontal: false,
      flip_vertical: false,
      alt_text: Some("aligned".into()),
      hyperlink_url: None,
      placement: crate::docx::ImagePlacement::Floating(crate::docx::FloatingImagePlacement {
        horizontal_relative_to: crate::docx::HorizontalImageReference::Margin,
        vertical_relative_to: crate::docx::VerticalImageReference::Margin,
        horizontal_alignment: Some(crate::docx::HorizontalImageAlignment::Center),
        vertical_alignment: Some(crate::docx::VerticalImageAlignment::Bottom),
        horizontal_offset_pt: 0.0,
        vertical_offset_pt: 0.0,
        wrap: crate::docx::ImageWrapMode::Through,
        wrap_side: crate::docx::ImageWrapSide::BothSides,
        behind_text: false,
        margin_top_pt: 0.0,
        margin_right_pt: 0.0,
        margin_bottom_pt: 0.0,
        margin_left_pt: 0.0,
      }),
    };
    let doc = crate::docx::DocxDocument {
      page: crate::docx::PageSetup {
        width_pt: 200.0,
        height_pt: 200.0,
        margin_left_pt: 20.0,
        margin_right_pt: 20.0,
        margin_top_pt: 20.0,
        margin_bottom_pt: 20.0,
        ..Default::default()
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
        inlines: vec![
          crate::docx::InlineItem::Image(image),
          crate::docx::InlineItem::Text(run.clone()),
        ],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        runs: vec![run],
        format: crate::docx::ParagraphFormat::default(),
        list_label: None,
        list_label_hyperlink_url: None,
      })],
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let image = layout.pages[0]
      .items
      .iter()
      .find_map(|item| match item {
        crate::layout::PageItem::Image(image) => Some(image),
        crate::layout::PageItem::Text(_)
        | crate::layout::PageItem::Fill(_)
        | crate::layout::PageItem::Line(_) => None,
      })
      .expect("aligned image");

    assert!((image.x_pt - 70.0).abs() < 0.1);
    assert!((image.y_pt - 100.0).abs() < 0.1);
    assert!((image.width_pt - 60.0).abs() < 0.1);
    assert!((image.height_pt - 80.0).abs() < 0.1);
  }

  #[test]
  fn top_bottom_floating_image_advances_following_text_to_next_column() {
    let before = crate::docx::TextRun {
      text: "Before float ".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let after = crate::docx::TextRun {
      text: "After float".into(),
      style: crate::docx::TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
    };
    let image = crate::docx::InlineImage {
      data: vec![0; 8],
      content_type: Some("image/png".into()),
      width_pt: 36.0,
      height_pt: 72.0,
      effect_left_pt: 0.0,
      effect_top_pt: 0.0,
      effect_right_pt: 0.0,
      effect_bottom_pt: 0.0,
      crop: crate::docx::ImageCrop::default(),
      rotation_deg: 0.0,
      flip_horizontal: false,
      flip_vertical: false,
      alt_text: Some("top bottom".into()),
      hyperlink_url: None,
      placement: crate::docx::ImagePlacement::Floating(crate::docx::FloatingImagePlacement {
        horizontal_relative_to: crate::docx::HorizontalImageReference::Column,
        vertical_relative_to: crate::docx::VerticalImageReference::Paragraph,
        horizontal_alignment: None,
        vertical_alignment: None,
        horizontal_offset_pt: 0.0,
        vertical_offset_pt: 0.0,
        wrap: crate::docx::ImageWrapMode::TopBottom,
        wrap_side: crate::docx::ImageWrapSide::BothSides,
        behind_text: false,
        margin_top_pt: 0.0,
        margin_right_pt: 0.0,
        margin_bottom_pt: 0.0,
        margin_left_pt: 0.0,
      }),
    };
    let paragraph = crate::docx::Paragraph {
      inlines: vec![
        crate::docx::InlineItem::Text(before.clone()),
        crate::docx::InlineItem::Image(image),
        crate::docx::InlineItem::Text(after.clone()),
      ],
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      runs: vec![before, after],
      format: crate::docx::ParagraphFormat::default(),
      list_label: None,
      list_label_hyperlink_url: None,
    };
    let page = crate::docx::PageSetup {
      width_pt: 300.0,
      height_pt: 100.0,
      margin_left_pt: 20.0,
      margin_right_pt: 20.0,
      margin_top_pt: 10.0,
      margin_bottom_pt: 10.0,
      ..Default::default()
    };
    let columns = crate::docx::SectionColumns {
      count: 2,
      gap_pt: 20.0,
      ..Default::default()
    };
    let doc = crate::docx::DocxDocument {
      page,
      default_tab_stop_pt: 36.0,
      even_and_odd_headers: false,
      sections: vec![crate::docx::ImportedSection {
        break_kind: crate::docx::SectionBreakKind::Continuous,
        section_properties: None,
        page,
        columns,
        title_page: false,
        header_blocks: Vec::new(),
        footer_blocks: Vec::new(),
        first_header_blocks: Vec::new(),
        first_footer_blocks: Vec::new(),
        even_header_blocks: Vec::new(),
        even_footer_blocks: Vec::new(),
        blocks: vec![crate::docx::Block::Paragraph(paragraph)],
      }],
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
      blocks: Vec::new(),
    };

    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let before = text_item(&layout, "Before float ");
    let after = text_item(&layout, "After float");

    assert_eq!(layout.pages.len(), 1);
    assert!(before.x_pt < page.margin_left_pt + 100.0);
    assert!(after.x_pt > page.margin_left_pt + 100.0);
    assert!(after.y_pt <= page.margin_top_pt + 0.1);
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
    assert!(!layout.pages.iter().any(|page| {
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
    assert!(layout_contains_text(&layout, "A3 top of vertical merge"));

    let path = fixture_path("test-data/wml/table_props.docx");
    let mut package = WordprocessingDocument::new(File::open(path).unwrap()).unwrap();
    let doc = crate::docx::extract(&mut package, &PdfOptions::default()).unwrap();
    let table = match &doc.blocks[0] {
      crate::docx::Block::Table(table) => table,
      crate::docx::Block::Paragraph(_) => panic!("expected table"),
    };
    assert_eq!(table.rows[0].height_pt, Some(24.0));
    assert!(table.rows[0].exact_height);
    assert_eq!(table.preferred_width_pct, Some(0.8));
    assert_eq!(table.indent_left_pt, 12.0);
    assert_eq!(table.alignment, crate::docx::TableAlignment::Center);
    assert_eq!(table.rows[0].cells[0].margins.left_pt, 18.0);
    assert_eq!(table.rows[0].cells[0].margins.top_pt, 12.0);
    assert_eq!(table.rows[0].cells[1].margins.left_pt, 9.0);
    assert_eq!(table.rows[0].cells[1].margins.top_pt, 6.0);
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
    let layout = crate::layout::layout(&doc, &PdfOptions::default()).unwrap();
    let top_cell = text_item(&layout, "Top-aligned cell.");
    assert!(top_cell.x_pt > doc.page.margin_left_pt);
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

  fn text_page_index(layout: &crate::layout::LayoutDocument, text: &str) -> Option<usize> {
    layout
      .pages
      .iter()
      .position(|page| page_has_text(page, text))
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

  fn layout_contains_text(layout: &crate::layout::LayoutDocument, text: &str) -> bool {
    layout_texts(layout).contains(text)
  }

  fn layout_texts(layout: &crate::layout::LayoutDocument) -> String {
    let mut flattened = String::new();
    for item in layout.pages.iter().flat_map(|page| &page.items) {
      if let crate::layout::PageItem::Text(item) = item {
        flattened.push_str(&item.text);
      }
    }
    flattened
  }
}
