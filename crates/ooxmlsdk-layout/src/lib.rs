pub mod common;
pub mod docx;
pub mod error;
pub mod pptx;
pub mod xlsx;

use std::borrow::Cow;

use common::{
  DebugCell, DebugFrame, DebugPage, DebugRecord, DebugShape, DebugTextLine, DisplayDocument,
  DisplayPage, FrameId, FrameRecord, LayoutDocument, LayoutEngineKind, LayoutOptions, Point, Pt,
  Rect, Size, TextRun,
};

pub use error::{LayoutError, Result};

pub fn layout_docx_model<'doc>(
  document: &docx::DocxDocument<'doc>,
  options: LayoutOptions,
) -> LayoutDocument<'doc> {
  let mut layout = LayoutDocument {
    engine_kind: LayoutEngineKind::Docx,
    options,
    ..LayoutDocument::default()
  };
  for (section_index, section) in document.sections.iter().enumerate() {
    let bounds = page_bounds(section.page_desc.page_size);
    push_page(&mut layout, None, bounds);
    let frame_id = push_frame(
      &mut layout,
      None,
      Cow::Borrowed("docx:section"),
      body_bounds(bounds, section.page_desc.margins),
    );
    for (block_index, block) in section.body_blocks.iter().enumerate() {
      match block {
        docx::DocxBlock::Paragraph(paragraph) => {
          let text = paragraph_text(paragraph);
          layout
            .debug_records
            .push(DebugRecord::TextLine(DebugTextLine {
              frame: frame_id,
              index: block_index,
              text,
              bounds: Rect::default(),
            }));
        }
        docx::DocxBlock::Table(_) => layout.debug_records.push(DebugRecord::Frame(DebugFrame {
          id: FrameId(layout.frames.len() as u32),
          parent: Some(frame_id),
          kind: Cow::Borrowed("docx:table"),
          bounds: Rect::default(),
          print_bounds: Rect::default(),
        })),
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
  let mut layout = LayoutDocument {
    engine_kind: LayoutEngineKind::Xlsx,
    options,
    ..LayoutDocument::default()
  };
  for sheet in workbook
    .sheets
    .iter()
    .filter(|sheet| sheet.state == xlsx::SheetState::Visible)
  {
    let bounds = Rect::default();
    push_page(&mut layout, Some(sheet.name.clone()), bounds);
  }
  let print_plan = if workbook.print_plan.sheet_pages.is_empty() {
    workbook.build_print_plan()
  } else {
    workbook.print_plan.clone()
  };
  for printed_sheet in &print_plan.sheet_pages {
    for page in &printed_sheet.pages {
      for cell in &page.cells {
        layout.debug_records.push(DebugRecord::Cell(DebugCell {
          sheet: printed_sheet.sheet_name.clone(),
          reference: Cow::Owned(format_xlsx_address(cell.address)),
          bounds: cell.bounds,
        }));
      }
    }
  }
  layout
}

pub fn layout_pptx_model<'doc>(
  presentation: &pptx::PptxPresentation<'doc>,
  options: LayoutOptions,
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
      push_pptx_shape_debug(&mut layout, page_index, vec![shape_index], shape);
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

fn push_pptx_shape_debug<'doc>(
  layout: &mut LayoutDocument<'doc>,
  page_index: usize,
  path: Vec<usize>,
  shape: &pptx::PptxShape<'doc>,
) {
  layout.debug_records.push(DebugRecord::Shape(DebugShape {
    page_index,
    path: path.clone(),
    kind: pptx_shape_kind(shape),
    bounds: shape.transform_bounds(),
  }));
  if let Some(text_body) = &shape.text_body {
    let text = text_body
      .paragraphs
      .iter()
      .flat_map(|paragraph| paragraph.runs.iter())
      .map(|run| run.text.as_ref())
      .collect::<Vec<_>>()
      .join("");
    layout.pages[page_index]
      .items
      .push(common::DisplayItem::Text(TextRun {
        text: Cow::Owned(text),
        origin: Point {
          x: shape.transform.dx,
          y: shape.transform.dy,
        },
        font_id: None,
        color: common::Color::default(),
        source: None,
      }));
  }
  for (index, child) in shape.children.iter().enumerate() {
    let mut child_path = path.clone();
    child_path.push(index);
    push_pptx_shape_debug(layout, page_index, child_path, child);
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

  #[test]
  fn docx_layout_emits_page_frame_and_text_debug_records() {
    let document = docx::DocxDocument {
      sections: vec![docx::DocxSection {
        body_blocks: vec![docx::DocxBlock::Paragraph(docx::DocxParagraph {
          inlines: vec![docx::InlineItem::Text(docx::DocxTextRun {
            text: Cow::Borrowed("Hello"),
            ..docx::DocxTextRun::default()
          })],
          ..docx::DocxParagraph::default()
        })],
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
  }
}
