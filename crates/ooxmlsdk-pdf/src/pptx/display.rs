use std::sync::Arc;

use crate::docx::TextStyle;
use crate::layout::{self, PageItem, PdfTextSegmentation, TextItem};
use crate::units;

use super::drawingml::shape::Shape;
use super::drawingml::text_body::{TextBody, TextRunKind};
use super::import::PowerPointImport;
use super::slide::SlidePersist;

const DEFAULT_TEXT_LINE_HEIGHT_PT: f32 = 14.0;
const DEFAULT_TEXT_INSET_PT: f32 = 4.0;

pub(crate) fn lower_to_layout_document(import: &PowerPointImport) -> layout::LayoutDocument {
  let pages = import
    .draw_pages
    .iter()
    .map(|slide| (slide.size.to_page_setup(), lower_slide_items(slide)))
    .collect();
  layout::fixed_pages_with_items(pages)
}

fn lower_slide_items(slide: &SlidePersist) -> Vec<PageItem> {
  let mut items = Vec::new();
  for shape in &slide.shapes {
    lower_shape(shape, DisplayOffset::default(), &mut items);
  }
  items
}

#[derive(Clone, Copy, Debug, Default)]
struct DisplayOffset {
  x_emu: i64,
  y_emu: i64,
}

fn lower_shape(shape: &Shape, offset: DisplayOffset, items: &mut Vec<PageItem>) {
  if shape.hidden || shape.hidden_master_shape {
    return;
  }

  if let Some(text_body) = &shape.text_body {
    lower_text_body(shape, offset, text_body, items);
  }

  let child_offset = child_display_offset(shape, offset);
  for child in &shape.children {
    lower_shape(child, child_offset, items);
  }
}

fn child_display_offset(shape: &Shape, offset: DisplayOffset) -> DisplayOffset {
  DisplayOffset {
    x_emu: offset.x_emu + shape.position.x - shape.child_position.x,
    y_emu: offset.y_emu + shape.position.y - shape.child_position.y,
  }
}

fn lower_text_body(
  shape: &Shape,
  offset: DisplayOffset,
  text_body: &TextBody,
  items: &mut Vec<PageItem>,
) {
  let x_pt = units::emu_to_points(offset.x_emu + shape.position.x) + DEFAULT_TEXT_INSET_PT;
  let mut y_pt = units::emu_to_points(offset.y_emu + shape.position.y) + DEFAULT_TEXT_INSET_PT;
  let style = TextStyle {
    font_family: Some(Arc::from("Liberation Sans")),
    ..TextStyle::default()
  };

  for paragraph in &text_body.paragraphs {
    let text = paragraph_text(text_body, paragraph);
    if text.is_empty() {
      y_pt += DEFAULT_TEXT_LINE_HEIGHT_PT;
      continue;
    }

    items.push(PageItem::Text(TextItem {
      x_pt,
      y_pt,
      line_height_pt: DEFAULT_TEXT_LINE_HEIGHT_PT,
      text,
      style: style.clone(),
      hyperlink_url: None,
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      form_widget_id: None,
      paragraph_bidi: false,
      preserve_text_portion: false,
      decoration_span_start_x_pt: None,
      pdf_text_segmentation: PdfTextSegmentation::Line,
    }));
    y_pt += DEFAULT_TEXT_LINE_HEIGHT_PT;
  }
}

fn paragraph_text(
  text_body: &TextBody,
  paragraph: &super::drawingml::text_body::TextParagraph,
) -> String {
  let mut text = String::new();
  if let Some(level) = paragraph.level {
    if text_body.has_list_style {
      for _ in 0..level {
        text.push_str("  ");
      }
    }
  }

  for run in &paragraph.runs {
    match run.kind {
      TextRunKind::Run | TextRunKind::Field => text.push_str(&run.text),
      TextRunKind::Break => text.push('\n'),
      TextRunKind::Math => {}
    }
  }
  text
}
