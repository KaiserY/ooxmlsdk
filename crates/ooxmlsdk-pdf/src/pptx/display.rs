use std::sync::Arc;

use crate::docx::{BorderStyle, RgbColor, TextStyle};
use crate::layout::{self, PageItem, PdfTextSegmentation, RectItem, TextItem};
use crate::units;

use super::drawingml::color::Color;
use super::drawingml::fill::{FillKind, FillProperties};
use super::drawingml::line::{LineFill, LineProperties};
use super::drawingml::shape::{Shape, ShapeService};
use super::drawingml::text_body::{TextBody, TextRunKind};
use super::import::PowerPointImport;
use super::slide::{BackgroundKind, BackgroundProperties, SlidePersist};

const DEFAULT_TEXT_LINE_HEIGHT_PT: f32 = 14.0;
const DEFAULT_TEXT_INSET_PT: f32 = 4.0;

pub(crate) fn lower_to_layout_document(import: &PowerPointImport) -> layout::LayoutDocument {
  let pages = import
    .draw_pages
    .iter()
    .map(|slide| (slide.size.to_page_setup(), lower_slide_items(import, slide)))
    .collect();
  layout::fixed_pages_with_items(pages)
}

fn lower_slide_items(import: &PowerPointImport, slide: &SlidePersist) -> Vec<PageItem> {
  let mut items = Vec::new();
  let master_page = slide
    .master_page_index
    .and_then(|master_page_index| import.master_pages.get(master_page_index));
  if let Some(background) = slide
    .background_properties
    .as_ref()
    .or_else(|| master_page.and_then(|master_page| master_page.background_properties.as_ref()))
  {
    lower_background(slide, background, &mut items);
  }
  if let Some(master_page) = master_page {
    lower_shapes(&master_page.shapes, &mut items);
  }
  lower_shapes(&slide.shapes, &mut items);
  items
}

fn lower_background(
  slide: &SlidePersist,
  background: &BackgroundProperties,
  items: &mut Vec<PageItem>,
) {
  let BackgroundKind::Properties(fill_properties) = &background.kind else {
    return;
  };
  let Some(fill_color) = fill_color(fill_properties) else {
    return;
  };
  items.push(PageItem::Rect(RectItem {
    x_pt: 0.0,
    y_pt: 0.0,
    width_pt: slide.size.width_pt,
    height_pt: slide.size.height_pt,
    fill_color: Some(fill_color),
    fill_opacity: 1.0,
    stroke: None,
    stroke_opacity: 1.0,
  }));
}

fn lower_shapes(shapes: &[Shape], items: &mut Vec<PageItem>) {
  for shape in shapes {
    lower_shape(shape, DisplayOffset::default(), items);
  }
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

  lower_shape_bounds(shape, offset, items);

  if let Some(text_body) = &shape.text_body {
    lower_text_body(shape, offset, text_body, items);
  }

  let child_offset = child_display_offset(shape, offset);
  for child in &shape.children {
    lower_shape(child, child_offset, items);
  }
}

fn lower_shape_bounds(shape: &Shape, offset: DisplayOffset, items: &mut Vec<PageItem>) {
  if shape.service_name == ShapeService::GroupShape || shape.size.cx <= 0 || shape.size.cy <= 0 {
    return;
  }

  let fill_color = shape.actual_fill_properties.as_ref().and_then(fill_color);
  let stroke = shape.actual_line_properties.as_ref().and_then(line_stroke);
  if fill_color.is_none() && stroke.is_none() {
    return;
  }

  items.push(PageItem::Rect(RectItem {
    x_pt: units::emu_to_points(offset.x_emu + shape.position.x),
    y_pt: units::emu_to_points(offset.y_emu + shape.position.y),
    width_pt: units::emu_to_points(shape.size.cx),
    height_pt: units::emu_to_points(shape.size.cy),
    fill_color,
    fill_opacity: 1.0,
    stroke,
    stroke_opacity: 1.0,
  }));
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

fn fill_color(fill: &FillProperties) -> Option<RgbColor> {
  match &fill.kind {
    FillKind::Solid(color) => color.as_ref().and_then(display_rgb_color),
    FillKind::None
    | FillKind::Group
    | FillKind::Gradient(_)
    | FillKind::Blip(_)
    | FillKind::Pattern(_) => None,
  }
}

fn line_stroke(line: &LineProperties) -> Option<BorderStyle> {
  let LineFill::Solid(color) = &line.fill else {
    return None;
  };
  Some(BorderStyle {
    width_pt: line.width_emu.map(units::emu_to_points).unwrap_or(0.75),
    spacing_pt: 0.0,
    color: color.as_ref().and_then(display_rgb_color)?,
    compound: false,
  })
}

fn display_rgb_color(color: &Color) -> Option<RgbColor> {
  match color {
    Color::RgbHex(hex) => parse_rgb_hex(hex),
    Color::System(system) => system.last_color.as_deref().and_then(parse_rgb_hex),
    Color::Scheme(_) | Color::Preset(_) => None,
  }
}

fn parse_rgb_hex(hex: &str) -> Option<RgbColor> {
  if hex.len() != 6 {
    return None;
  }

  Some(RgbColor {
    r: u8::from_str_radix(&hex[0..2], 16).ok()?,
    g: u8::from_str_radix(&hex[2..4], 16).ok()?,
    b: u8::from_str_radix(&hex[4..6], 16).ok()?,
  })
}
