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
    lower_background(import, slide, background, &mut items);
  }
  if let Some(master_page) = master_page {
    lower_shapes(import, &master_page.shapes, &mut items);
  }
  lower_shapes(import, &slide.shapes, &mut items);
  items
}

fn lower_background(
  import: &PowerPointImport,
  slide: &SlidePersist,
  background: &BackgroundProperties,
  items: &mut Vec<PageItem>,
) {
  let fill_properties = match &background.kind {
    BackgroundKind::Properties(fill_properties) => Some(
      fill_properties
        .clone()
        .with_placeholder_color(slide.background_color.clone()),
    ),
    BackgroundKind::StyleReference {
      style_index,
      placeholder_color,
    } => import.get_theme_fill_style(*style_index).map(|fill| {
      fill.with_placeholder_color(
        placeholder_color
          .clone()
          .or_else(|| slide.background_color.clone()),
      )
    }),
  };
  let Some(fill_properties) = fill_properties else {
    return;
  };
  let Some(fill_paint) = fill_paint(import, &fill_properties) else {
    return;
  };
  items.push(PageItem::Rect(RectItem {
    x_pt: 0.0,
    y_pt: 0.0,
    width_pt: slide.size.width_pt,
    height_pt: slide.size.height_pt,
    fill_color: Some(fill_paint.color),
    fill_opacity: fill_paint.opacity,
    stroke: None,
    stroke_opacity: 1.0,
  }));
}

fn lower_shapes(import: &PowerPointImport, shapes: &[Shape], items: &mut Vec<PageItem>) {
  for shape in shapes {
    lower_shape(import, shape, DisplayOffset::default(), items);
  }
}

#[derive(Clone, Copy, Debug, Default)]
struct DisplayOffset {
  x_emu: i64,
  y_emu: i64,
}

fn lower_shape(
  import: &PowerPointImport,
  shape: &Shape,
  offset: DisplayOffset,
  items: &mut Vec<PageItem>,
) {
  if shape.hidden || shape.hidden_master_shape {
    return;
  }

  lower_shape_bounds(import, shape, offset, items);

  if let Some(text_body) = &shape.text_body {
    lower_text_body(shape, offset, text_body, items);
  }

  let child_offset = child_display_offset(shape, offset);
  for child in &shape.children {
    lower_shape(import, child, child_offset, items);
  }
}

fn lower_shape_bounds(
  import: &PowerPointImport,
  shape: &Shape,
  offset: DisplayOffset,
  items: &mut Vec<PageItem>,
) {
  if shape.service_name == ShapeService::GroupShape || shape.size.cx <= 0 || shape.size.cy <= 0 {
    return;
  }

  let fill_paint = shape
    .actual_fill_properties
    .as_ref()
    .and_then(|fill| fill_paint(import, fill));
  let line = shape
    .actual_line_properties
    .as_ref()
    .and_then(|line| line_stroke(import, line));
  if fill_paint.is_none() && line.is_none() {
    return;
  }
  let (stroke, stroke_opacity) = line
    .map(|line| (Some(line.style), line.opacity))
    .unwrap_or((None, 1.0));

  items.push(PageItem::Rect(RectItem {
    x_pt: units::emu_to_points(offset.x_emu + shape.position.x),
    y_pt: units::emu_to_points(offset.y_emu + shape.position.y),
    width_pt: units::emu_to_points(shape.size.cx),
    height_pt: units::emu_to_points(shape.size.cy),
    fill_color: fill_paint.map(|fill| fill.color),
    fill_opacity: fill_paint.map(|fill| fill.opacity).unwrap_or(1.0),
    stroke,
    stroke_opacity,
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

#[derive(Clone, Copy, Debug)]
struct DisplayPaint {
  color: RgbColor,
  opacity: f32,
}

#[derive(Clone, Copy, Debug)]
struct DisplayStroke {
  style: BorderStyle,
  opacity: f32,
}

fn fill_paint(import: &PowerPointImport, fill: &FillProperties) -> Option<DisplayPaint> {
  match &fill.kind {
    FillKind::Solid(color) => color
      .as_ref()
      .and_then(|color| display_paint(import, color, fill.placeholder_color.as_ref())),
    FillKind::None
    | FillKind::Group
    | FillKind::Gradient(_)
    | FillKind::Blip(_)
    | FillKind::Pattern(_) => None,
  }
}

fn line_stroke(import: &PowerPointImport, line: &LineProperties) -> Option<DisplayStroke> {
  let LineFill::Solid(color) = &line.fill else {
    return None;
  };
  let paint = color
    .as_ref()
    .and_then(|color| display_paint(import, color, line.placeholder_color.as_ref()))?;
  Some(DisplayStroke {
    style: BorderStyle {
      width_pt: line.width_emu.map(units::emu_to_points).unwrap_or(0.75),
      spacing_pt: 0.0,
      color: paint.color,
      compound: false,
    },
    opacity: paint.opacity,
  })
}

fn display_paint(
  import: &PowerPointImport,
  color: &Color,
  placeholder_color: Option<&Color>,
) -> Option<DisplayPaint> {
  let color = import.resolve_color(color, placeholder_color)?;
  Some(DisplayPaint {
    color: RgbColor {
      r: color.r,
      g: color.g,
      b: color.b,
    },
    opacity: color_opacity(color.alpha),
  })
}

fn color_opacity(alpha: i32) -> f32 {
  alpha.clamp(0, 100_000) as f32 / 100_000.0
}
