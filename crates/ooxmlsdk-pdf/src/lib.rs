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
use ooxmlsdk::sdk::{
  FileFormatVersion, MarkupCompatibilityProcessMode, MarkupCompatibilityProcessSettings,
  OpenSettings,
};

pub use error::{PdfError, Result};
pub use options::{PdfOptions, PdfStandard};

/// Convert a DOCX stream into PDF bytes.
pub fn convert_docx<R>(reader: R, options: PdfOptions) -> Result<Vec<u8>>
where
  R: Read + Seek,
{
  let settings = OpenSettings {
    markup_compatibility_process_settings: MarkupCompatibilityProcessSettings {
      process_mode: MarkupCompatibilityProcessMode::ProcessLoadedPartsOnly,
      target_file_format_version: FileFormatVersion::Microsoft365,
    },
    ..Default::default()
  };
  let mut document = WordprocessingDocument::new_with_settings(reader, settings)?;
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

  use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;

  use super::*;

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
        | crate::layout::PageItem::Rect(_)
        | crate::layout::PageItem::Fill(_)
        | crate::layout::PageItem::Line(_) => None,
      })
      .collect::<Vec<_>>();

    assert_eq!(lines, ["One", "Two", "Six"]);
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
        | crate::layout::PageItem::Rect(_)
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
        | crate::layout::PageItem::Rect(_)
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
        | crate::layout::PageItem::Rect(_)
        | crate::layout::PageItem::Fill(_)
        | crate::layout::PageItem::Line(_) => None,
      })
      .expect("inline image");

    assert!(!image.floating);
    assert!((image.width_pt - 36.0).abs() < 0.1);
    assert!((image.height_pt - 24.0).abs() < 0.1);
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
        | crate::layout::PageItem::Rect(_)
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
  fn pdfexport_fixture_tdf156685_mce_processing_rewrites_run_alternate_content() {
    let path = fixture_path("test-data/ooxmlsdk-pdf-test/libreoffice/tdf156685.docx");
    let settings = OpenSettings {
      markup_compatibility_process_settings: MarkupCompatibilityProcessSettings {
        process_mode: MarkupCompatibilityProcessMode::ProcessLoadedPartsOnly,
        target_file_format_version: FileFormatVersion::Microsoft365,
      },
      ..Default::default()
    };
    let mut package =
      WordprocessingDocument::new_with_settings(File::open(path).unwrap(), settings).unwrap();
    let root = package
      .main_document_part()
      .unwrap()
      .root_element(&mut package)
      .unwrap();

    let mut drawing_count = 0usize;
    let mut pict_count = 0usize;
    let mut xml_any_count = 0usize;

    for choice in &root.body.as_ref().unwrap().body_choice {
      let w::BodyChoice::WP(paragraph) = choice else {
        continue;
      };
      for choice in &paragraph.paragraph_choice {
        let w::ParagraphChoice::WR(run) = choice else {
          continue;
        };
        for choice in &run.run_choice {
          match choice {
            w::RunChoice::WDrawing(_) => drawing_count += 1,
            w::RunChoice::WPict(_) => pict_count += 1,
            w::RunChoice::XmlAny(_) => xml_any_count += 1,
            _ => {}
          }
        }
      }
    }

    assert_eq!(xml_any_count, 0);
    assert!(drawing_count + pict_count > 0);
  }

  fn fixture_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("../..")
      .join(relative)
  }

  fn text_item<'a>(
    layout: &'a crate::layout::LayoutDocument,
    text: &str,
  ) -> &'a crate::layout::TextItem {
    find_text_item(layout, text).unwrap_or_else(|| panic!("text item {text:?}"))
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
        crate::layout::PageItem::Rect(_) => None,
        crate::layout::PageItem::Fill(_) => None,
        crate::layout::PageItem::Line(_) => None,
      })
  }
}
