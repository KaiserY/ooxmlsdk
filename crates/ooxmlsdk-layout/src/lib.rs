pub mod common;
pub mod docx;
pub mod error;
pub mod pptx;
pub mod xlsx;

use std::borrow::Cow;

use common::{
  DebugCell, DebugFrame, DebugPage, DebugRecord, DebugShape, DebugTextLine, DisplayDocument,
  DisplayPage, FrameId, FrameRecord, GlyphRun, LayoutDocument, LayoutEngineKind, LayoutFontRequest,
  LayoutOptions, Point, Pt, Rect, RectItem, Size, TextRun,
};
use ooxmlsdk_fonts::{FontRegistry, ShapeOptions, TextDirection, TextScript};

pub use error::{LayoutError, Result};

pub fn layout_docx_model<'doc>(
  document: &docx::DocxDocument<'doc>,
  options: LayoutOptions,
) -> LayoutDocument<'doc> {
  layout_docx_model_impl(document, options, None)
}

pub fn layout_docx_model_with_fonts<'doc>(
  document: &docx::DocxDocument<'doc>,
  options: LayoutOptions,
  fonts: &FontRegistry<'doc>,
) -> LayoutDocument<'doc> {
  layout_docx_model_impl(document, options, Some(fonts))
}

fn layout_docx_model_impl<'doc>(
  document: &docx::DocxDocument<'doc>,
  options: LayoutOptions,
  fonts: Option<&FontRegistry<'doc>>,
) -> LayoutDocument<'doc> {
  let mut layout = LayoutDocument {
    engine_kind: LayoutEngineKind::Docx,
    options,
    ..LayoutDocument::default()
  };
  for (section_index, section) in document.sections.iter().enumerate() {
    let bounds = page_bounds(section.page_desc.page_size);
    push_page(&mut layout, None, bounds);
    let body_rect = body_bounds(bounds, section.page_desc.margins);
    let frame_id = push_frame(&mut layout, None, Cow::Borrowed("docx:section"), body_rect);
    let mut block_y = body_rect.origin.y.0;
    for (block_index, block) in section.body_blocks.iter().enumerate() {
      match block {
        docx::DocxBlock::Paragraph(paragraph) => {
          let text = paragraph_text(paragraph);
          let line_height = docx_paragraph_line_height(paragraph);
          let line_bounds = Rect {
            origin: Point {
              x: Pt(body_rect.origin.x.0 + paragraph.format.margins.left.0),
              y: Pt(block_y),
            },
            size: Size {
              width: Pt(
                (body_rect.size.width.0
                  - paragraph.format.margins.left.0
                  - paragraph.format.margins.right.0)
                  .max(0.0),
              ),
              height: line_height,
            },
          };
          layout
            .debug_records
            .push(DebugRecord::TextLine(DebugTextLine {
              frame: frame_id,
              index: block_index,
              text,
              bounds: line_bounds,
            }));
          if let Some(text) = docx_paragraph_visible_text(paragraph) {
            let style = docx_paragraph_first_text_style(paragraph);
            let font_request = style.as_ref().map(|(font, _)| font.clone());
            let color = style.as_ref().map(|(_, color)| *color).unwrap_or_default();
            push_display_text(
              &mut layout.pages[section_index],
              text,
              line_bounds.origin,
              font_request,
              color,
              fonts,
            );
          }
          block_y += line_height.0;
        }
        docx::DocxBlock::Table(table) => {
          let table_bounds = Rect {
            origin: Point {
              x: body_rect.origin.x,
              y: Pt(block_y),
            },
            size: Size {
              width: body_rect.size.width,
              height: docx_table_height(table),
            },
          };
          push_frame(
            &mut layout,
            Some(frame_id),
            Cow::Borrowed("docx:table"),
            table_bounds,
          );
          block_y += table_bounds.size.height.0;
        }
        docx::DocxBlock::FloatingFrame(frame) => {
          layout.debug_records.push(DebugRecord::Shape(DebugShape {
            page_index: section_index,
            path: vec![block_index],
            kind: Cow::Borrowed("docx:floatingFrame"),
            bounds: frame.bounds,
          }));
        }
      }
    }
  }
  layout
}

pub fn layout_xlsx_model<'doc>(
  workbook: &xlsx::XlsxWorkbook<'doc>,
  options: LayoutOptions,
) -> LayoutDocument<'doc> {
  layout_xlsx_model_impl(workbook, options, None)
}

pub fn layout_xlsx_model_with_fonts<'doc>(
  workbook: &xlsx::XlsxWorkbook<'doc>,
  options: LayoutOptions,
  fonts: &FontRegistry<'doc>,
) -> LayoutDocument<'doc> {
  layout_xlsx_model_impl(workbook, options, Some(fonts))
}

fn layout_xlsx_model_impl<'doc>(
  workbook: &xlsx::XlsxWorkbook<'doc>,
  options: LayoutOptions,
  fonts: Option<&FontRegistry<'doc>>,
) -> LayoutDocument<'doc> {
  let mut layout = LayoutDocument {
    engine_kind: LayoutEngineKind::Xlsx,
    options,
    ..LayoutDocument::default()
  };
  let print_plan = if workbook.print_plan.sheet_pages.is_empty() {
    workbook.build_print_plan()
  } else {
    workbook.print_plan.clone()
  };
  for printed_sheet in &print_plan.sheet_pages {
    for page in &printed_sheet.pages {
      let page_index = layout.pages.len();
      push_page(
        &mut layout,
        Some(printed_sheet.sheet_name.clone()),
        page.paper_bounds,
      );
      for cell in &page.cells {
        layout.debug_records.push(DebugRecord::Cell(DebugCell {
          sheet: printed_sheet.sheet_name.clone(),
          reference: Cow::Owned(format_xlsx_address(cell.address)),
          bounds: cell.bounds,
        }));
        if !cell.text.is_empty() {
          let style = xlsx_cell_text_style(workbook, cell);
          push_display_text(
            &mut layout.pages[page_index],
            cell.text.clone(),
            cell.bounds.origin,
            style.as_ref().map(xlsx::XlsxTextStyle::layout_font_request),
            style.and_then(|style| style.color).unwrap_or_default(),
            fonts,
          );
        }
      }
    }
  }
  layout
}

fn xlsx_cell_text_style<'doc>(
  workbook: &xlsx::XlsxWorkbook<'doc>,
  cell: &xlsx::XlsxCellFragment<'doc>,
) -> Option<xlsx::XlsxTextStyle<'doc>> {
  let index = cell.style_index? as usize;
  workbook
    .styles
    .cell_formats
    .get(index)
    .map(|format| format.text_style.clone())
}

pub fn layout_pptx_model<'doc>(
  presentation: &pptx::PptxPresentation<'doc>,
  options: LayoutOptions,
) -> LayoutDocument<'doc> {
  layout_pptx_model_impl(presentation, options, None)
}

pub fn layout_pptx_model_with_fonts<'doc>(
  presentation: &pptx::PptxPresentation<'doc>,
  options: LayoutOptions,
  fonts: &FontRegistry<'doc>,
) -> LayoutDocument<'doc> {
  layout_pptx_model_impl(presentation, options, Some(fonts))
}

fn layout_pptx_model_impl<'doc>(
  presentation: &pptx::PptxPresentation<'doc>,
  options: LayoutOptions,
  fonts: Option<&FontRegistry<'doc>>,
) -> LayoutDocument<'doc> {
  let mut layout = LayoutDocument {
    engine_kind: LayoutEngineKind::Pptx,
    options,
    ..LayoutDocument::default()
  };
  for (page_index, slide) in presentation.slides.iter().enumerate() {
    let bounds = page_bounds(presentation.page_size);
    push_page(&mut layout, slide.name.clone(), bounds);
    for (shape_index, shape) in slide.shapes.iter().enumerate() {
      push_pptx_shape_debug(&mut layout, page_index, vec![shape_index], shape, fonts);
    }
  }
  layout
}

pub fn display_document<'doc>(layout: &LayoutDocument<'doc>) -> DisplayDocument<'doc> {
  DisplayDocument {
    pages: layout.pages.clone(),
    ..DisplayDocument::default()
  }
}

fn push_page<'doc>(layout: &mut LayoutDocument<'doc>, name: Option<Cow<'doc, str>>, bounds: Rect) {
  let index = layout.pages.len();
  layout.pages.push(DisplayPage {
    name: name.clone(),
    bounds,
    ..DisplayPage::default()
  });
  layout.debug_records.push(DebugRecord::Page(DebugPage {
    index,
    name,
    bounds,
  }));
}

fn push_frame<'doc>(
  layout: &mut LayoutDocument<'doc>,
  parent: Option<FrameId>,
  kind: Cow<'doc, str>,
  bounds: Rect,
) -> FrameId {
  let id = FrameId(layout.frames.len() as u32);
  let frame = FrameRecord {
    id,
    parent,
    kind: kind.clone(),
    bounds,
    print_bounds: bounds,
  };
  layout.frames.push(frame);
  layout.debug_records.push(DebugRecord::Frame(DebugFrame {
    id,
    parent,
    kind,
    bounds,
    print_bounds: bounds,
  }));
  id
}

fn paragraph_text<'doc>(paragraph: &docx::DocxParagraph<'doc>) -> Cow<'doc, str> {
  let mut text = String::new();
  for inline in &paragraph.inlines {
    match inline {
      docx::InlineItem::Text(run) => text.push_str(&run.text),
      docx::InlineItem::Field(field) => text.push_str(&field.display_text),
      docx::InlineItem::PageBreak => text.push('\u{000c}'),
      docx::InlineItem::ColumnBreak => text.push('\u{000b}'),
      _ => {}
    }
  }
  Cow::Owned(text)
}

fn docx_paragraph_visible_text<'doc>(
  paragraph: &docx::DocxParagraph<'doc>,
) -> Option<Cow<'doc, str>> {
  let text = paragraph_text(paragraph);
  (!text.is_empty()).then_some(text)
}

fn docx_paragraph_line_height(paragraph: &docx::DocxParagraph<'_>) -> Pt {
  if let Some(value) = paragraph.format.line_height.value {
    return value;
  }
  paragraph
    .inlines
    .iter()
    .filter_map(|inline| match inline {
      docx::InlineItem::Text(run) if run.style.font.size_pt.0 > 0.0 => {
        Some(run.style.font.size_pt.0)
      }
      _ => None,
    })
    .max_by(f32::total_cmp)
    .map(Pt)
    .unwrap_or_default()
}

fn docx_paragraph_first_text_style<'doc>(
  paragraph: &docx::DocxParagraph<'doc>,
) -> Option<(LayoutFontRequest<'doc>, common::Color)> {
  paragraph.inlines.iter().find_map(|inline| match inline {
    docx::InlineItem::Text(run) if !run.text.is_empty() => {
      Some((run.style.layout_font_request(), run.style.color))
    }
    _ => None,
  })
}

fn docx_table_height(table: &docx::DocxTable<'_>) -> Pt {
  let height = table
    .rows
    .iter()
    .filter_map(|row| row.height)
    .map(|height| height.0)
    .sum::<f32>();
  Pt(height)
}

fn push_pptx_shape_debug<'doc>(
  layout: &mut LayoutDocument<'doc>,
  page_index: usize,
  path: Vec<usize>,
  shape: &pptx::PptxShape<'doc>,
  fonts: Option<&FontRegistry<'doc>>,
) {
  layout.debug_records.push(DebugRecord::Shape(DebugShape {
    page_index,
    path: path.clone(),
    kind: pptx_shape_kind(shape),
    bounds: shape.transform_bounds(),
  }));
  if matches!(shape.kind, pptx::PptxShapeKind::Shape) {
    layout.pages[page_index]
      .items
      .push(common::DisplayItem::Rect(RectItem {
        bounds: shape.transform_bounds(),
        fill: shape.fill.clone(),
        stroke: shape.line.clone(),
      }));
  }
  if let Some(text_body) = &shape.text_body {
    let text = text_body
      .paragraphs
      .iter()
      .flat_map(|paragraph| paragraph.runs.iter())
      .map(|run| run.text.as_ref())
      .collect::<Vec<_>>()
      .join("");
    let style = text_body
      .paragraphs
      .iter()
      .flat_map(|paragraph| paragraph.runs.iter())
      .find(|run| !run.text.is_empty())
      .map(|run| &run.style);
    push_display_text(
      &mut layout.pages[page_index],
      Cow::Owned(text),
      Point {
        x: shape.transform.dx,
        y: shape.transform.dy,
      },
      style.map(|style| LayoutFontRequest::from_font_request(style.font.clone())),
      style.and_then(|style| style.color).unwrap_or_default(),
      fonts,
    );
  }
  for (index, child) in shape.children.iter().enumerate() {
    let mut child_path = path.clone();
    child_path.push(index);
    push_pptx_shape_debug(layout, page_index, child_path, child, fonts);
  }
}

fn push_display_text<'doc>(
  page: &mut DisplayPage<'doc>,
  text: Cow<'doc, str>,
  origin: Point,
  font: Option<LayoutFontRequest<'doc>>,
  color: common::Color,
  fonts: Option<&FontRegistry<'doc>>,
) {
  let Some(fonts) = fonts else {
    page.items.push(common::DisplayItem::Text(TextRun {
      text,
      origin,
      font_id: None,
      color,
      source: None,
    }));
    return;
  };
  let request = font.unwrap_or_default();
  let Ok(shaped_runs) = shape_layout_text_runs(fonts, &request, text.as_ref()) else {
    page.items.push(common::DisplayItem::Text(TextRun {
      text,
      origin,
      font_id: None,
      color,
      source: None,
    }));
    return;
  };
  let mut x = origin.x.0;
  for shaped in shaped_runs {
    let advance = shaped.advance_pt;
    page.items.push(common::DisplayItem::Glyphs(GlyphRun {
      glyphs: shaped.glyphs.clone(),
      shaped,
      origin: Point {
        x: Pt(x),
        y: origin.y,
      },
      color,
      source: None,
    }));
    x += advance;
  }
}

fn shape_layout_text_runs<'doc>(
  fonts: &FontRegistry<'doc>,
  request: &LayoutFontRequest<'doc>,
  text: &str,
) -> ooxmlsdk_fonts::Result<Vec<ooxmlsdk_fonts::ShapedRun<'doc>>> {
  let mut runs = Vec::new();
  for (range, script) in script_ranges(text) {
    let font_request = request.for_script(script);
    let segment = Cow::Owned(text[range.clone()].to_owned());
    let mut options = ShapeOptions::from_request(&font_request, TextDirection::LeftToRight);
    options.small_caps = request.small_caps;
    options.character_spacing_pt = request.character_spacing.0;
    for mut run in fonts.shape_text_runs_with_options(&font_request, segment, &options)? {
      run.offset_text_range(range.start);
      runs.push(run);
    }
  }
  Ok(runs)
}

fn script_ranges(text: &str) -> Vec<(std::ops::Range<usize>, TextScript)> {
  let mut ranges = Vec::new();
  let mut start = 0usize;
  let mut active = None::<TextScript>;
  for (index, ch) in text.char_indices() {
    let script = match (text_script(ch), active) {
      (TextScript::Common, Some(active)) => active,
      (script, _) => script,
    };
    if active.is_some_and(|active| active != script) {
      ranges.push((start..index, active.unwrap_or(TextScript::Common)));
      start = index;
    }
    active = Some(script);
  }
  if start < text.len() || text.is_empty() {
    ranges.push((start..text.len(), active.unwrap_or(TextScript::Common)));
  }
  ranges
}

fn text_script(ch: char) -> TextScript {
  match u32::from(ch) {
    0x0041..=0x007a | 0x00c0..=0x024f => TextScript::Latin,
    0x0370..=0x03ff => TextScript::Greek,
    0x0400..=0x052f => TextScript::Cyrillic,
    0x0590..=0x05ff => TextScript::Hebrew,
    0x0600..=0x06ff | 0x0750..=0x077f | 0x08a0..=0x08ff => TextScript::Arabic,
    0x0900..=0x097f => TextScript::Devanagari,
    0x0e00..=0x0e7f => TextScript::Thai,
    0x3040..=0x309f => TextScript::Hiragana,
    0x30a0..=0x30ff => TextScript::Katakana,
    0x3400..=0x9fff | 0xf900..=0xfaff => TextScript::Han,
    0xac00..=0xd7af => TextScript::Hangul,
    _ => TextScript::Common,
  }
}

fn pptx_shape_kind<'doc>(shape: &pptx::PptxShape<'doc>) -> Cow<'doc, str> {
  match &shape.kind {
    pptx::PptxShapeKind::Shape => Cow::Borrowed("pptx:shape"),
    pptx::PptxShapeKind::Group => Cow::Borrowed("pptx:group"),
    pptx::PptxShapeKind::Picture { .. } => Cow::Borrowed("pptx:picture"),
    pptx::PptxShapeKind::Media { .. } => Cow::Borrowed("pptx:media"),
    pptx::PptxShapeKind::GraphicFrame { .. } => Cow::Borrowed("pptx:graphicFrame"),
    pptx::PptxShapeKind::Table(_) => Cow::Borrowed("pptx:table"),
    pptx::PptxShapeKind::Chart { .. } => Cow::Borrowed("pptx:chart"),
    pptx::PptxShapeKind::Diagram { .. } => Cow::Borrowed("pptx:diagram"),
    pptx::PptxShapeKind::OleObject { .. } => Cow::Borrowed("pptx:oleObject"),
    pptx::PptxShapeKind::Connector => Cow::Borrowed("pptx:connector"),
  }
}

fn page_bounds(size: Size) -> Rect {
  Rect {
    origin: Point {
      x: Pt(0.0),
      y: Pt(0.0),
    },
    size,
  }
}

fn body_bounds(bounds: Rect, margins: common::Insets) -> Rect {
  Rect {
    origin: Point {
      x: Pt(bounds.origin.x.0 + margins.left.0),
      y: Pt(bounds.origin.y.0 + margins.top.0),
    },
    size: Size {
      width: Pt((bounds.size.width.0 - margins.left.0 - margins.right.0).max(0.0)),
      height: Pt((bounds.size.height.0 - margins.top.0 - margins.bottom.0).max(0.0)),
    },
  }
}

fn format_xlsx_address(address: xlsx::CellAddress) -> String {
  let mut column = address.column + 1;
  let mut letters = Vec::new();
  while column > 0 {
    column -= 1;
    letters.push((b'A' + (column % 26) as u8) as char);
    column /= 26;
  }
  letters.iter().rev().collect::<String>() + &(address.row + 1).to_string()
}

trait PptxShapeBounds {
  fn transform_bounds(&self) -> Rect;
}

impl PptxShapeBounds for pptx::PptxShape<'_> {
  fn transform_bounds(&self) -> Rect {
    Rect {
      origin: Point {
        x: self.transform.dx,
        y: self.transform.dy,
      },
      size: Size {
        width: Pt(0.0),
        height: Pt(0.0),
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::Arc;

  use ooxmlsdk_fonts::{FontFaceInfo, FontId, FontSource};

  #[test]
  fn docx_layout_emits_page_frame_and_text_debug_records() {
    let document = docx::DocxDocument {
      sections: vec![docx::DocxSection {
        body_blocks: vec![
          docx::DocxBlock::Paragraph(docx::DocxParagraph {
            inlines: vec![docx::InlineItem::Text(docx::DocxTextRun {
              text: Cow::Borrowed("Hello"),
              ..docx::DocxTextRun::default()
            })],
            ..docx::DocxParagraph::default()
          }),
          docx::DocxBlock::Table(docx::DocxTable {
            rows: vec![docx::DocxTableRow::default()],
            ..docx::DocxTable::default()
          }),
        ],
        ..docx::DocxSection::default()
      }],
      ..docx::DocxDocument::default()
    };

    let layout = layout_docx_model(&document, LayoutOptions::default());

    assert_eq!(layout.engine_kind, LayoutEngineKind::Docx);
    assert_eq!(layout.pages.len(), 1);
    assert!(
      layout
        .debug_records
        .iter()
        .any(|record| matches!(record, DebugRecord::TextLine(line) if line.text == "Hello"))
    );
    assert!(layout.pages.first().is_some_and(|page| {
      page
        .items
        .iter()
        .any(|item| matches!(item, common::DisplayItem::Text(text) if text.text == "Hello"))
    }));
    assert!(layout.frames.iter().any(|frame| frame.kind == "docx:table"));
  }

  #[test]
  fn docx_layout_uses_script_specific_font_families_for_glyph_runs() {
    let mut registry = FontRegistry::new();
    let mut latin = FontFaceInfo::synthetic("latin", "Latin Face");
    latin.coverage.unicode_ranges = std::iter::once(u32::from('A')..u32::from('A') + 1).collect();
    registry.register_face(FontSource::System, latin);
    let mut east_asian = FontFaceInfo::synthetic("east-asian", "East Asian Face");
    east_asian.coverage.unicode_ranges =
      std::iter::once(u32::from('中')..u32::from('中') + 1).collect();
    registry.register_face(FontSource::System, east_asian);

    let document = docx::DocxDocument {
      sections: vec![docx::DocxSection {
        body_blocks: vec![docx::DocxBlock::Paragraph(docx::DocxParagraph {
          inlines: vec![docx::InlineItem::Text(docx::DocxTextRun {
            text: Cow::Borrowed("A中"),
            style: docx::TextStyle {
              font_families: Box::new(common::ScriptFontFamilies {
                latin: Some(Cow::Borrowed("Latin Face")),
                east_asian: Some(Cow::Borrowed("East Asian Face")),
                ..common::ScriptFontFamilies::default()
              }),
              ..docx::TextStyle::default()
            },
          })],
          ..docx::DocxParagraph::default()
        })],
        ..docx::DocxSection::default()
      }],
      ..docx::DocxDocument::default()
    };

    let layout = layout_docx_model_with_fonts(&document, LayoutOptions::default(), &registry);
    let fonts = layout.pages[0]
      .items
      .iter()
      .filter_map(|item| match item {
        common::DisplayItem::Glyphs(run) => Some(run.shaped.font_id.clone()),
        _ => None,
      })
      .collect::<Vec<_>>();

    assert_eq!(
      fonts,
      vec![FontId(Arc::from("latin")), FontId(Arc::from("east-asian"))]
    );
  }

  #[test]
  fn xlsx_layout_emits_cell_debug_records_for_visible_sheets() {
    let workbook = xlsx::XlsxWorkbook {
      sheets: vec![xlsx::XlsxSheet {
        name: Cow::Borrowed("Sheet1"),
        rows: vec![xlsx::XlsxRow {
          cells: vec![xlsx::XlsxCell {
            address: Some(xlsx::CellAddress { column: 1, row: 2 }),
            display_text: Cow::Borrowed("42"),
            ..xlsx::XlsxCell::default()
          }],
          ..xlsx::XlsxRow::default()
        }],
        ..xlsx::XlsxSheet::default()
      }],
      ..xlsx::XlsxWorkbook::default()
    };

    let layout = layout_xlsx_model(&workbook, LayoutOptions::default());

    assert_eq!(layout.engine_kind, LayoutEngineKind::Xlsx);
    assert!(
      layout
        .debug_records
        .iter()
        .any(|record| matches!(record, DebugRecord::Cell(cell) if cell.reference == "B3"))
    );
  }

  #[test]
  fn pptx_layout_emits_shape_debug_records() {
    let presentation = pptx::PptxPresentation {
      slides: vec![pptx::PptxSlide {
        name: Some(Cow::Borrowed("Slide 1")),
        shapes: vec![pptx::PptxShape {
          id: Some(1),
          name: Some(Cow::Borrowed("Title")),
          ..pptx::PptxShape::default()
        }],
        ..pptx::PptxSlide::default()
      }],
      ..pptx::PptxPresentation::default()
    };

    let layout = layout_pptx_model(&presentation, LayoutOptions::default());

    assert_eq!(layout.engine_kind, LayoutEngineKind::Pptx);
    assert!(
      layout
        .debug_records
        .iter()
        .any(|record| matches!(record, DebugRecord::Shape(shape) if shape.kind == "pptx:shape"))
    );
    assert!(layout.pages.first().is_some_and(|page| {
      page
        .items
        .iter()
        .any(|item| matches!(item, common::DisplayItem::Rect(_)))
    }));
  }
}
