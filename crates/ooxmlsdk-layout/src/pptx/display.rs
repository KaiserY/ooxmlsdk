use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;

use crate::common::{self, DebugProperty, DebugRecord, DebugShape, DebugValue, Point, Rect, Size};
use crate::model::{
  BorderStyle, ImageCrop, ImageItem, LineItem, LineItemKind, LinkAreaItem, PageItem, PageSetup,
  PdfTextSegmentation, RectItem, RgbColor, RgbColor as LayoutRgbColor, TextItem, TextStyle,
  common_page_setup, common_point, common_rect, common_rgb, common_stroke_from_border,
  common_text_style,
};
use crate::options::LayoutOptions;
use crate::render::chart as shared_chart;
use crate::render::diagram as shared_diagram;
use crate::render::emf_wmf;
use crate::render::symbol as shared_symbol;
use crate::text_metrics::TextMetrics;
use crate::units;
use image::codecs::png::PngEncoder;
use image::{ColorType, GenericImageView, ImageEncoder};
use ooxmlsdk::schemas::schemas_microsoft_com_office_drawing_2008_diagram as dsp;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_chart as c;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_diagram as dgm;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::units as sdk_units;
use ooxmlsdk::units::DrawingmlPercentageValue;

use super::chart::{ChartFrame, ClusteredColumnStyle, lower_clustered_column_chart};
use super::custom_geometry;
use super::drawingml::color::{Color, SchemeColor};
use super::drawingml::fill::{FillKind, FillProperties};
use super::drawingml::line::{LineFill, LineProperties};
use super::drawingml::shape::{
  CustomShapeGeometry, FontStyleReference, GraphicDataKind, GraphicDataRecord, Shape, ShapeService,
};
use super::drawingml::table::{
  TableCell, TableCellBorders, TableProperties, TableStyle, TableStyleBorders, TableStylePart,
  TableStyleTextProperties, predefined_table_style,
};
use super::drawingml::text_body::{
  TextAutoFit, TextBody, TextBodyDisplayProperties, TextParagraph, TextRun, TextRunKind,
  has_noninherited_body_properties,
};
use super::drawingml::text_list_style::{
  TextListLevelParagraphProperties, TextListParagraphStyle, TextListStyle,
};
use super::import::{PowerPointImport, ThemeFragmentRecord};
use super::preset_geometry;
use super::shadow::{ShadowFrame, outer_shadow_image_item};
use super::slide::{BackgroundKind, ChartResource, ImageResource, SlidePersist};
use super::{
  PptxBulletParagraphSummary, PptxDrawShapeSummary, PptxLayoutSummary,
  PptxSmartArtTextShapeSummary, PptxTextShapeSummary,
};

const DEFAULT_TEXT_FONT_SIZE_PT: f32 = 18.0;
const DEFAULT_TEXT_LINE_HEIGHT_SCALE: f32 = 1.2;
const POWERPOINT_PRINT_DPI: f32 = 600.0;
const DEFAULT_TABLE_BORDER_PT: f32 = 0.75;

pub(crate) fn lower_to_layout_document(
  import: &PowerPointImport,
  options: &LayoutOptions,
) -> common::LayoutDocument<'static> {
  let pages = import
    .draw_pages
    .iter()
    .enumerate()
    .map(|(page_index, slide)| {
      (
        slide.size.to_page_setup(),
        lower_slide_items_with_summary(
          import,
          slide,
          page_index,
          options.ui_language.as_deref(),
          None,
        ),
      )
    })
    .collect();
  common_fixed_pages_with_items(pages, options)
}

fn common_fixed_pages_with_items(
  pages: Vec<(PageSetup, Vec<PageItem>)>,
  options: &LayoutOptions,
) -> common::LayoutDocument<'static> {
  let pages = if pages.is_empty() {
    vec![(PageSetup::default(), Vec::new())]
  } else {
    pages
  };
  common::LayoutDocument {
    engine_kind: common::LayoutEngineKind::Pptx,
    options: common::LayoutOptions {
      collect_debug: options.diagnostics.collect_debug_records,
      approximate_unsupported: false,
      preserve_source_links: options.diagnostics.preserve_source_links,
    },
    pages: pages
      .into_iter()
      .map(|(setup, items)| common_display_page(setup, items))
      .collect(),
    ..Default::default()
  }
}

fn common_display_page(setup: PageSetup, items: Vec<PageItem>) -> common::DisplayPage<'static> {
  let common_setup = common_page_setup(setup);
  common::DisplayPage {
    section_index: 0,
    section_page_index: 0,
    bounds: common_rect(0.0, 0.0, setup.width_pt, setup.height_pt),
    background: setup
      .background
      .map(|color| common::Fill::Solid(common_rgb(color, 1.0))),
    setup: common_setup,
    items: items.into_iter().map(common_display_item).collect(),
    ..Default::default()
  }
}

fn common_display_item(item: PageItem) -> common::DisplayItem<'static> {
  match item {
    PageItem::Text(item) => common::DisplayItem::Text(common_text_run(item)),
    PageItem::Image(item) => common::DisplayItem::Image(common_image_item(item)),
    PageItem::LinkArea(item) => common::DisplayItem::LinkArea(common::LinkArea {
      bounds: common_rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
      target: Cow::Owned(item.hyperlink_url),
    }),
    PageItem::Path(item) => common::DisplayItem::Path(item),
    PageItem::Rect(item) => common::DisplayItem::Rect(common_rect_item(item)),
    PageItem::Line(item) => common::DisplayItem::Line(common_line_item(item)),
  }
}

fn common_text_run(item: TextItem) -> common::TextRun<'static> {
  let color = common_rgb(item.style.color, item.style.opacity);
  common::TextRun {
    text: Cow::Owned(item.text),
    origin: common_point(item.x_pt, item.y_pt),
    line_height: common::Pt(item.line_height_pt),
    style: common_text_style(item.style),
    font_id: None,
    color,
    rotation_center: item.rotation_center_pt.map(|(x, y)| common_point(x, y)),
    hyperlink_url: item.hyperlink_url.map(Cow::Owned),
    dynamic_field: None,
    form_widget_id: item.form_widget_id,
    paragraph_bidi: item.paragraph_bidi,
    preserve_text_portion: item.preserve_text_portion,
    pdf_text_segmentation: match item.pdf_text_segmentation {
      PdfTextSegmentation::Line => common::PdfTextSegmentation::Line,
      PdfTextSegmentation::Portion => common::PdfTextSegmentation::Portion,
    },
    source: None,
  }
}

fn common_image_item(item: ImageItem) -> common::ImageItem<'static> {
  common::ImageItem {
    bounds: common_rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
    crop: Some(common::ImageCrop {
      left: item.crop.left,
      top: item.crop.top,
      right: item.crop.right,
      bottom: item.crop.bottom,
    }),
    clip_path: item.clip_path,
    rotation_degrees: item.rotation_deg,
    flip_horizontal: item.flip_horizontal,
    flip_vertical: item.flip_vertical,
    content_type: item
      .content_type
      .map(Cow::Owned)
      .unwrap_or(Cow::Borrowed("application/octet-stream")),
    bytes: item.data,
    relationship_id: None,
    alt_text: item.alt_text.map(Cow::Owned),
    hyperlink_url: item.hyperlink_url.map(Cow::Owned),
    floating: item.floating,
    behind_text: item.behind_text,
  }
}

fn common_rect_item(item: RectItem) -> common::RectItem<'static> {
  common::RectItem {
    bounds: common_rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
    fill: item
      .fill_color
      .map(|color| common::Fill::Solid(common_rgb(color, item.fill_opacity)))
      .unwrap_or(common::Fill::None),
    stroke: item
      .stroke
      .map(|stroke| common_stroke_from_border(stroke, item.stroke_opacity)),
  }
}

fn common_line_item(item: LineItem) -> common::LineItem<'static> {
  common::LineItem {
    start: common_point(item.x1_pt, item.y1_pt),
    end: common_point(item.x2_pt, item.y2_pt),
    stroke: common::Stroke {
      width: common::Pt(item.width_pt),
      color: common_rgb(item.color, 1.0),
      dash: None,
      source_style_id: None,
    },
    kind: match item.kind {
      LineItemKind::Stroke => common::LineKind::Stroke,
    },
  }
}

pub(crate) fn inspect_layout_summary(import: &PowerPointImport) -> PptxLayoutSummary {
  let mut summary = PptxLayoutSummary {
    is_endless: import.is_endless,
    is_automatic: import.is_automatic,
    first_page_name: import.first_page_name.clone(),
    custom_show_name: import.custom_show_name.clone(),
    embed_true_type_fonts: import.embed_true_type_fonts,
    save_subset_fonts: import.save_subset_fonts,
    embedded_font_typefaces: import.embedded_font_typefaces.clone(),
    notes_page_shape_counts: import
      .notes_pages
      .iter()
      .map(notes_page_shape_count)
      .collect(),
    draw_page_shape_counts: import
      .draw_pages
      .iter()
      .map(draw_page_shape_count)
      .collect(),
    ..PptxLayoutSummary::default()
  };
  collect_draw_shape_summaries(import, &mut summary);
  collect_master_text_shapes(import, &mut summary);
  for (page_index, slide) in import.draw_pages.iter().enumerate() {
    let _ = lower_slide_items_with_summary(import, slide, page_index, None, Some(&mut summary));
  }
  summary
}

pub(crate) fn debug_records(import: &PowerPointImport) -> Vec<DebugRecord<'static>> {
  let summary = inspect_layout_summary(import);
  let mut records = Vec::new();
  if let Some(first_page_name) = summary.first_page_name {
    records.push(debug_shape(
      0,
      Vec::new(),
      "pptx_first_page",
      Rect::default(),
      vec![debug_text("name", first_page_name)],
    ));
  }
  for (page_index, count) in summary.notes_page_shape_counts.into_iter().enumerate() {
    for shape_index in 0..count {
      records.push(debug_shape(
        page_index,
        vec![shape_index],
        "pptx_notes_shape",
        Rect::default(),
        Vec::new(),
      ));
    }
  }
  for shape in summary.draw_shapes {
    records.push(debug_record_from_draw_shape(shape));
  }
  for shape in summary.master_text_shapes {
    records.push(debug_shape(
      shape.master_page_index,
      vec![shape.shape_index],
      "pptx_master_text_shape",
      Rect::default(),
      vec![debug_text("text", shape.text)],
    ));
  }
  for shape in summary.smartart_text_shapes {
    records.push(debug_shape(
      shape.page_index,
      Vec::new(),
      "pptx_smartart_text_shape",
      rect_100mm(
        shape.text_anchor_left_100mm,
        shape.text_anchor_top_100mm,
        shape.text_anchor_right_100mm,
        shape.text_anchor_bottom_100mm,
      ),
      vec![
        debug_text("text", shape.text),
        debug_i32("text_left_distance_100mm", shape.text_left_distance_100mm),
        debug_i32("text_upper_distance_100mm", shape.text_upper_distance_100mm),
        debug_i32("text_anchor_left_100mm", shape.text_anchor_left_100mm),
        debug_i32("text_anchor_top_100mm", shape.text_anchor_top_100mm),
        debug_i32("text_anchor_right_100mm", shape.text_anchor_right_100mm),
        debug_i32("text_anchor_bottom_100mm", shape.text_anchor_bottom_100mm),
      ],
    ));
  }
  for paragraph in summary.bullet_paragraphs {
    let mut metadata = vec![
      debug_i32("paragraph_index", paragraph.paragraph_index as i32),
      debug_text("text", paragraph.text),
    ];
    if let Some(character) = paragraph.character {
      metadata.push(debug_text("character", character));
    }
    if let Some(font) = paragraph.font {
      metadata.push(debug_text("font", font));
    }
    if let Some(width) = paragraph.graphic_width_100mm {
      metadata.push(debug_i32("graphic_width_100mm", width));
    }
    if let Some(height) = paragraph.graphic_height_100mm {
      metadata.push(debug_i32("graphic_height_100mm", height));
    }
    records.push(debug_shape(
      paragraph.page_index,
      Vec::new(),
      "pptx_bullet_paragraph",
      Rect::default(),
      metadata,
    ));
  }
  records
}

fn debug_record_from_draw_shape(shape: PptxDrawShapeSummary) -> DebugRecord<'static> {
  let mut metadata = vec![
    debug_text("service_name", shape.service_name),
    debug_text("text", shape.text),
    debug_i32("left_100mm", shape.left_100mm),
    debug_i32("top_100mm", shape.top_100mm),
    debug_i32("right_100mm", shape.right_100mm),
    debug_i32("bottom_100mm", shape.bottom_100mm),
    debug_i32("width_100mm", shape.width_100mm),
    debug_i32("height_100mm", shape.height_100mm),
    debug_text("fill_style", shape.fill_style),
    debug_bool(
      "fill_uses_slide_background",
      shape.fill_uses_slide_background,
    ),
  ];
  if let Some(geometry) = shape.geometry {
    metadata.push(debug_text("geometry", geometry));
  }
  if let Some(gradient_style) = shape.gradient_style {
    metadata.push(debug_text("gradient_style", gradient_style));
  }
  if let Some(gradient_angle) = shape.gradient_angle {
    metadata.push(debug_i32("gradient_angle", i32::from(gradient_angle)));
  }
  if let Some(value) = shape.text_left_distance_100mm {
    metadata.push(debug_i32("text_left_distance_100mm", value));
  }
  if let Some(value) = shape.text_upper_distance_100mm {
    metadata.push(debug_i32("text_upper_distance_100mm", value));
  }
  if let Some(value) = shape.text_right_distance_100mm {
    metadata.push(debug_i32("text_right_distance_100mm", value));
  }
  if let Some(value) = shape.text_lower_distance_100mm {
    metadata.push(debug_i32("text_lower_distance_100mm", value));
  }
  debug_shape(
    shape.page_index,
    shape.shape_path,
    "pptx_draw_shape",
    rect_100mm(
      shape.left_100mm,
      shape.top_100mm,
      shape.right_100mm,
      shape.bottom_100mm,
    ),
    metadata,
  )
}

fn debug_shape(
  page_index: usize,
  path: Vec<usize>,
  kind: &'static str,
  bounds: Rect,
  metadata: Vec<DebugProperty<'static>>,
) -> DebugRecord<'static> {
  DebugRecord::Shape(DebugShape {
    page_index,
    path,
    kind: kind.into(),
    bounds,
    metadata,
  })
}

fn debug_bool(name: &'static str, value: bool) -> DebugProperty<'static> {
  DebugProperty {
    name: name.into(),
    value: DebugValue::Bool(value),
  }
}

fn debug_i32(name: &'static str, value: i32) -> DebugProperty<'static> {
  DebugProperty {
    name: name.into(),
    value: DebugValue::Integer(i64::from(value)),
  }
}

fn debug_text(name: &'static str, value: String) -> DebugProperty<'static> {
  DebugProperty {
    name: name.into(),
    value: DebugValue::Text(value.into()),
  }
}

fn rect_100mm(left: i32, top: i32, right: i32, bottom: i32) -> Rect {
  Rect {
    origin: Point {
      x: crate::common::Pt(points_from_100mm(left)),
      y: crate::common::Pt(points_from_100mm(top)),
    },
    size: Size {
      width: crate::common::Pt(points_from_100mm(right - left)),
      height: crate::common::Pt(points_from_100mm(bottom - top)),
    },
  }
}

fn points_from_100mm(value: i32) -> f32 {
  value as f32 * 72.0 / 2540.0
}

fn notes_page_shape_count(slide: &SlidePersist) -> usize {
  slide.shapes.iter().map(notes_shape_count).sum()
}

fn draw_page_shape_count(slide: &SlidePersist) -> usize {
  slide
    .shapes
    .iter()
    .filter(|shape| shape.shape_location == Some(super::slide::ShapeLocation::Slide))
    .count()
}

fn collect_draw_shape_summaries(import: &PowerPointImport, summary: &mut PptxLayoutSummary) {
  for (page_index, slide) in import.draw_pages.iter().enumerate() {
    for (shape_index, shape) in slide.shapes.iter().enumerate() {
      if shape.shape_location != Some(super::slide::ShapeLocation::Slide) {
        continue;
      }
      collect_shape_summary(
        summary,
        page_index,
        shape,
        ShapeSummaryTransform::default(),
        vec![shape_index],
      );
    }
  }
}

fn collect_shape_summary(
  summary: &mut PptxLayoutSummary,
  page_index: usize,
  shape: &Shape,
  transform: ShapeSummaryTransform,
  shape_path: Vec<usize>,
) {
  if shape.hidden
    || shape.hidden_master_shape
    || shape.referenced
    || is_uninstantiated_placeholder(shape)
  {
    return;
  }

  let x_pt =
    units::emu_to_points_f32(transform.x_emu + shape.position.x as f32 * transform.scale_x);
  let y_pt =
    units::emu_to_points_f32(transform.y_emu + shape.position.y as f32 * transform.scale_y);
  let width_pt = units::emu_to_points_f32(shape.size.cx as f32 * transform.scale_x);
  let height_pt = units::emu_to_points_f32(shape.size.cy as f32 * transform.scale_y);
  let (geo_x_pt, geo_y_pt) =
    rotated_shape_geo_top_left(x_pt, y_pt, width_pt, height_pt, shape.rotation);
  summary
    .draw_shapes
    .push(draw_shape_summary_from_parts(DrawShapeSummaryParts {
      page_index,
      shape_path: shape_path.clone(),
      service_name: format!("{:?}", shape.service_name),
      geometry: shape_geometry_name(shape.custom_shape_properties.geometry.as_ref()),
      text: shape_text(shape.text_body.as_ref()),
      frame: TextFrame {
        x_pt: geo_x_pt,
        y_pt: geo_y_pt,
        width_pt,
        height_pt,
      },
      fill: shape.actual_fill_properties.as_ref(),
      rotation_deg: shape.rotation,
      flip_h: shape.flip_h,
      flip_v: shape.flip_v,
      text_distances: shape.text_body.as_ref().map(|text_body| {
        let frame = text_body_frame(x_pt, y_pt, width_pt, height_pt, text_body);
        text_distances_from_frame(x_pt, y_pt, width_pt, height_pt, frame)
      }),
    }));

  let child_transform = transform.child(shape);
  for (child_index, child) in shape.children.iter().enumerate() {
    let mut child_path = shape_path.clone();
    child_path.push(child_index);
    collect_shape_summary(summary, page_index, child, child_transform, child_path);
  }
}

#[derive(Clone, Copy, Debug)]
struct ShapeSummaryTransform {
  x_emu: f32,
  y_emu: f32,
  scale_x: f32,
  scale_y: f32,
}

impl Default for ShapeSummaryTransform {
  fn default() -> Self {
    Self {
      x_emu: 0.0,
      y_emu: 0.0,
      scale_x: 1.0,
      scale_y: 1.0,
    }
  }
}

impl ShapeSummaryTransform {
  fn child(self, shape: &Shape) -> Self {
    let scale_x = if shape.child_size.cx != 0 {
      self.scale_x * shape.size.cx as f32 / shape.child_size.cx as f32
    } else {
      self.scale_x
    };
    let scale_y = if shape.child_size.cy != 0 {
      self.scale_y * shape.size.cy as f32 / shape.child_size.cy as f32
    } else {
      self.scale_y
    };
    let parent_x = self.x_emu + shape.position.x as f32 * self.scale_x;
    let parent_y = self.y_emu + shape.position.y as f32 * self.scale_y;
    Self {
      x_emu: parent_x - shape.child_position.x as f32 * scale_x,
      y_emu: parent_y - shape.child_position.y as f32 * scale_y,
      scale_x,
      scale_y,
    }
  }
}

fn notes_shape_count(shape: &Shape) -> usize {
  let own = usize::from(shape.shape_location == Some(super::slide::ShapeLocation::Slide));
  own + shape.children.iter().map(notes_shape_count).sum::<usize>()
}

fn collect_master_text_shapes(import: &PowerPointImport, summary: &mut PptxLayoutSummary) {
  for (master_page_index, master) in import.master_pages.iter().enumerate() {
    let mut shape_index = 0;
    collect_text_shapes_from_shape_list(
      master_page_index,
      &master.shapes,
      &mut shape_index,
      &mut summary.master_text_shapes,
    );
  }
}

fn collect_text_shapes_from_shape_list(
  master_page_index: usize,
  shapes: &[Shape],
  shape_index: &mut usize,
  summary: &mut Vec<PptxTextShapeSummary>,
) {
  for shape in shapes {
    if let Some(text_body) = &shape.text_body {
      let text = text_body_plain_text(text_body);
      if !text.trim().is_empty() {
        summary.push(PptxTextShapeSummary {
          master_page_index,
          shape_index: *shape_index,
          text,
        });
      }
    }
    *shape_index += 1;
    collect_text_shapes_from_shape_list(master_page_index, &shape.children, shape_index, summary);
  }
}

fn lower_slide_items_with_summary(
  import: &PowerPointImport,
  slide: &SlidePersist,
  page_index: usize,
  ui_language: Option<&str>,
  summary: Option<&mut PptxLayoutSummary>,
) -> Vec<PageItem> {
  let mut items = Vec::new();
  let _has_structured_comment_identity = slide.comments.iter().any(|comment| comment.has_payload())
    || slide
      .comment_authors
      .iter()
      .any(|author| author.has_payload());
  let _has_header_footer_identity = slide.header_footer.has_visible_slot();
  if let Some(fill) = resolved_slide_background_fill(import, slide) {
    lower_background(import, slide, &fill, &mut items);
  }
  lower_shapes(
    PptxLoweringContext {
      import,
      slide,
      page_index,
      ui_language,
    },
    &slide.shapes,
    &mut items,
    summary,
  );
  items
}

fn lower_background(
  import: &PowerPointImport,
  slide: &SlidePersist,
  fill_properties: &FillProperties,
  items: &mut Vec<PageItem>,
) {
  if let Some(fill_paint) = background_fill_paint(import, slide, fill_properties) {
    if is_default_white_page_background(fill_paint) {
      return;
    }
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
  } else {
    items.extend(
      blip_fill_image_items(
        import,
        slide,
        fill_properties,
        ImageFillPlacement {
          frame: TextFrame {
            x_pt: 0.0,
            y_pt: 0.0,
            width_pt: slide.size.width_pt,
            height_pt: slide.size.height_pt,
          },
          rotation_deg: 0.0,
          flip_horizontal: false,
          flip_vertical: false,
          crop_bitmap: false,
          clip_path: Vec::new(),
          alt_text: None,
          hyperlink_url: None,
        },
      )
      .into_iter()
      .map(PageItem::Image),
    );
  }
}

fn resolved_slide_background_fill(
  import: &PowerPointImport,
  slide: &SlidePersist,
) -> Option<FillProperties> {
  let master_page = slide
    .master_page_index
    .and_then(|master_page_index| import.master_pages.get(master_page_index));
  let background = slide
    .background_properties
    .as_ref()
    .or_else(|| master_page.and_then(|master_page| master_page.background_properties.as_ref()))?;
  match &background.kind {
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
  }
}

fn is_default_white_page_background(paint: DisplayPaint) -> bool {
  paint.opacity >= 1.0 && paint.color.r == 255 && paint.color.g == 255 && paint.color.b == 255
}

fn background_fill_paint(
  import: &PowerPointImport,
  slide: &SlidePersist,
  fill: &FillProperties,
) -> Option<DisplayPaint> {
  match &fill.kind {
    FillKind::Solid(color) => color.as_ref().and_then(|color| {
      display_paint_for_slide(import, slide, color, fill.placeholder_color.as_ref())
    }),
    FillKind::None
    | FillKind::SlideBackground
    | FillKind::Group
    | FillKind::Gradient(_)
    | FillKind::Blip(_)
    | FillKind::Pattern(_) => None,
  }
}

fn lower_shapes(
  context: PptxLoweringContext<'_>,
  shapes: &[Shape],
  items: &mut Vec<PageItem>,
  mut summary: Option<&mut PptxLayoutSummary>,
) {
  for shape in shapes {
    lower_shape(
      context,
      shape,
      DisplayOffset::default(),
      items,
      summary.as_deref_mut(),
    );
  }
}

#[derive(Clone, Copy, Debug, Default)]
struct DisplayOffset {
  x_emu: i64,
  y_emu: i64,
}

fn lower_shape(
  context: PptxLoweringContext<'_>,
  shape: &Shape,
  offset: DisplayOffset,
  items: &mut Vec<PageItem>,
  mut summary: Option<&mut PptxLayoutSummary>,
) {
  let PptxLoweringContext {
    import,
    slide,
    ui_language,
    ..
  } = context;
  if shape.hidden
    || shape.hidden_master_shape
    || shape.referenced
    || is_uninstantiated_placeholder(shape)
  {
    return;
  }

  lower_shape_bounds(import, slide, shape, offset, items);
  lower_picture(import, slide, shape, offset, items);
  lower_shape_hyperlink(shape, offset, items);
  let _has_structured_media_identity = shape.media.as_ref().is_some_and(|media| {
    !matches!(media.kind, super::drawingml::shape::MediaKind::Unknown)
      || media.embed_relationship_id.is_some()
      || media.link_relationship_id.is_some()
      || media
        .resource
        .as_ref()
        .is_some_and(|resource| resource.has_payload())
  });
  let _has_structured_content_part_identity =
    shape.content_part.as_ref().is_some_and(|content_part| {
      !content_part.relationship_id.is_empty()
        || content_part
          .resource
          .as_ref()
          .is_some_and(|resource| resource.has_payload())
    });
  let _has_structured_graphic_identity = shape
    .graphic_data
    .as_ref()
    .is_some_and(graphic_data_has_structured_identity);

  if let Some(table) = &shape.table_properties
    && shape.service_name == ShapeService::Table
  {
    lower_table(import, shape, offset, table, items);
  }

  if shape.service_name == ShapeService::Chart
    && let Some(record) = &shape.graphic_data
  {
    lower_chart(import, slide, shape, offset, record, ui_language, items);
  }

  if shape.frame_type == super::drawingml::shape::FrameType::Diagram
    && let Some(record) = &shape.graphic_data
  {
    lower_diagram(
      context,
      shape,
      offset,
      record,
      items,
      summary.as_deref_mut(),
    );
  }

  if let Some(text_body) = &shape.text_body {
    lower_text_body(
      context,
      shape,
      offset,
      text_body,
      items,
      summary.as_deref_mut(),
    );
  }

  let child_offset = child_display_offset(shape, offset);
  for child in &shape.children {
    lower_shape(context, child, child_offset, items, summary.as_deref_mut());
  }
}

fn is_uninstantiated_placeholder(shape: &Shape) -> bool {
  shape.sub_type.is_some()
    && shape
      .shape_location
      .is_some_and(|location| location != super::slide::ShapeLocation::Slide)
}

fn graphic_data_has_structured_identity(record: &GraphicDataRecord) -> bool {
  !record.uri.is_empty()
    || !matches!(record.kind, GraphicDataKind::Unsupported)
    || record.chart_relationship_id.is_some()
    || record
      .chart_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record
      .extended_chart_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record.has_inline_chart_space
    || record.diagram_relationship_ids.as_ref().is_some_and(|ids| {
      !ids.data_part.is_empty()
        || !ids.layout_part.is_empty()
        || !ids.style_part.is_empty()
        || !ids.color_part.is_empty()
    })
    || record
      .diagram_data_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record
      .diagram_layout_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record
      .diagram_style_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record
      .diagram_color_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record.ole_object.as_ref().is_some_and(|ole| {
      ole.relationship_id.is_some()
        || ole.name.is_some()
        || ole.prog_id.is_some()
        || ole.show_as_icon
    })
    || record
      .ole_binary_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record
      .embedded_package_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
}

fn lower_chart(
  import: &PowerPointImport,
  slide: &SlidePersist,
  shape: &Shape,
  offset: DisplayOffset,
  record: &GraphicDataRecord,
  ui_language: Option<&str>,
  items: &mut Vec<PageItem>,
) {
  if shape.size.cx <= 0 || shape.size.cy <= 0 {
    return;
  }
  let Some(chart_resource) = &record.chart_resource else {
    return;
  };
  let x = units::emu_to_points(offset.x_emu + shape.position.x);
  let y = units::emu_to_points(offset.y_emu + shape.position.y);
  let width = units::emu_to_points(shape.size.cx);
  let height = units::emu_to_points(shape.size.cy);

  if let Some(chart) = shared_chart::clustered_column_chart(&chart_resource.chart_space) {
    let series_colors = chart
      .series
      .iter()
      .enumerate()
      .filter_map(|(index, series)| {
        series
          .solid_fill
          .and_then(|fill| display_paint_for_chart(import, slide, chart_resource, fill))
          .map(|paint| paint.color)
          .or_else(|| display_color_for_chart_series(import, slide, chart_resource, index))
      })
      .collect::<Vec<_>>();
    if series_colors.len() == chart.series.len() {
      let title_properties = chart_resource
        .chart_space
        .chart
        .title
        .as_deref()
        .and_then(|title| title.text_properties.as_deref());
      let label_properties = chart
        .value_axis
        .and_then(|axis| axis.text_properties.as_deref())
        .or_else(|| {
          chart_resource
            .chart_space
            .chart
            .plot_area
            .plot_area_choice2
            .iter()
            .find_map(|choice| match choice {
              c::PlotAreaChoice2::CategoryAxis(axis) => axis.text_properties.as_deref(),
              _ => None,
            })
        });
      let gridline_color = chart
        .value_axis
        .and_then(|axis| axis.major_gridlines.as_deref())
        .and_then(|gridlines| gridlines.chart_shape_properties.as_deref())
        .and_then(shared_chart::chart_shape_solid_fill)
        .and_then(|fill| display_paint_for_chart(import, slide, chart_resource, fill))
        .map(|paint| paint.color)
        .unwrap_or(RgbColor {
          r: 217,
          g: 217,
          b: 217,
        });
      let chart_items = lower_clustered_column_chart(
        ChartFrame {
          x_pt: x,
          y_pt: y,
          width_pt: width,
          height_pt: height,
        },
        &chart,
        shared_chart::automatic_chart_title(ui_language),
        &ClusteredColumnStyle {
          title: chart_text_style(import, slide, title_properties, ui_language, 18.0),
          label: chart_text_style(import, slide, label_properties, ui_language, 12.0),
          gridline_color,
          series_colors,
        },
      );
      if !chart_items.is_empty() {
        items.extend(chart_items);
        return;
      }
    }
  }

  let paints = chart_data_point_paints(import, slide, chart_resource);
  let texts = shared_chart::visible_texts(&chart_resource.chart_space);
  if paints.is_empty() && texts.is_empty() {
    return;
  }

  let plot_x = x + width * 0.12;
  let plot_y = y + height * 0.12;
  let plot_width = width * 0.76;
  let plot_height = height * 0.76;
  if !paints.is_empty() {
    let point_count = paints.len().max(1) as f32;
    let gap = (plot_width * 0.02).min(6.0);
    let item_width = ((plot_width - gap * (point_count - 1.0)) / point_count).max(1.0);
    // Chart data point fills resolve against c:clrMapOvr, so bg1 maps through
    // the chart color map instead of the slide p:clrMapOvr.
    for (index, paint) in paints.iter().enumerate() {
      items.push(PageItem::Rect(RectItem {
        x_pt: plot_x + index as f32 * (item_width + gap),
        y_pt: plot_y,
        width_pt: item_width,
        height_pt: plot_height,
        fill_color: Some(paint.color),
        fill_opacity: paint.opacity,
        stroke: None,
        stroke_opacity: 1.0,
      }));
    }
  }

  lower_chart_texts(x, y, width, height, texts, items);
}

fn chart_text_style(
  import: &PowerPointImport,
  slide: &SlidePersist,
  properties: Option<&c::TextProperties>,
  ui_language: Option<&str>,
  fallback_size_pt: f32,
) -> TextStyle {
  let mut style = TextStyle {
    font_family: Some(Arc::from("Liberation Sans")),
    font_size_pt: fallback_size_pt,
    use_windows_font_metrics: true,
    ..TextStyle::default()
  };
  if let Some(default_run_properties) = properties
    .into_iter()
    .flat_map(|properties| &properties.paragraph)
    .filter_map(|paragraph| paragraph.paragraph_properties.as_deref())
    .find_map(|paragraph| paragraph.default_run_properties.as_deref())
  {
    apply_default_run_properties(import, Some(slide), default_run_properties, &mut style);
  }
  if let Some(typeface) = import.resolve_theme_font_for_language("+mn-ea", ui_language) {
    style.east_asia_font_family = Some(Arc::from(typeface));
  }
  style.font_size_pt =
    (style.font_size_pt * POWERPOINT_PRINT_DPI / 72.0).round() * 72.0 / POWERPOINT_PRINT_DPI;
  style
}

fn display_color_for_chart_series(
  import: &PowerPointImport,
  slide: &SlidePersist,
  chart_resource: &ChartResource,
  series_index: usize,
) -> Option<RgbColor> {
  let token = [
    a::SchemeColorValues::Accent1,
    a::SchemeColorValues::Accent2,
    a::SchemeColorValues::Accent3,
    a::SchemeColorValues::Accent4,
    a::SchemeColorValues::Accent5,
    a::SchemeColorValues::Accent6,
  ][series_index % 6];
  let theme = chart_theme(import, slide)?;
  let color_map = chart_resource.chart_space.color_map_override.as_deref();
  let mapped = shared_chart::scheme_color_token(color_map, token)?;
  let color = theme.color_scheme.get_color(mapped)?.clone();
  let mut scheme_resolver = |token| {
    let mapped = shared_chart::scheme_color_token(color_map, token)?;
    theme.color_scheme.get_color(mapped).cloned()
  };
  let resolved = color.resolve_rgb(&mut scheme_resolver, None)?;
  Some(RgbColor {
    r: resolved.r,
    g: resolved.g,
    b: resolved.b,
  })
}

fn lower_chart_texts(
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  texts: Vec<String>,
  items: &mut Vec<PageItem>,
) {
  let style = TextStyle {
    font_size_pt: 11.0,
    ..TextStyle::default()
  };
  let line_step = 13.2;
  let start_x = x + width * 0.12;
  let mut text_y = y + height * 0.12;
  let max_y = y + height * 0.9;

  for text in texts {
    if text_y > max_y {
      break;
    }
    push_text_item(
      items,
      TextItemPlacement {
        x_pt: start_x,
        y_pt: text_y,
        line_height_pt: line_height(&style, 1.0),
        rotation_center_pt: None,
      },
      text,
      style.clone(),
      None,
    );
    text_y += line_step;
  }
}

fn chart_data_point_paints(
  import: &PowerPointImport,
  slide: &SlidePersist,
  chart_resource: &ChartResource,
) -> Vec<DisplayPaint> {
  shared_chart::data_point_solid_fills(&chart_resource.chart_space)
    .into_iter()
    .filter_map(|fill| display_paint_for_chart(import, slide, chart_resource, fill.fill))
    .collect()
}

fn display_paint_for_chart(
  import: &PowerPointImport,
  slide: &SlidePersist,
  chart_resource: &ChartResource,
  fill: &a::SolidFill,
) -> Option<DisplayPaint> {
  let theme = chart_theme(import, slide)?;
  let color_map = chart_resource.chart_space.color_map_override.as_deref();
  let color = fill
    .solid_fill_choice
    .as_ref()
    .and_then(Color::from_solid_fill_choice)?;
  let mut scheme_resolver = |token| {
    let mapped = shared_chart::scheme_color_token(color_map, token)?;
    theme.color_scheme.get_color(mapped).cloned()
  };
  let color = color.resolve_rgb(&mut scheme_resolver, None)?;
  Some(DisplayPaint {
    color: RgbColor {
      r: color.r,
      g: color.g,
      b: color.b,
    },
    opacity: color_opacity(color.alpha),
  })
}

fn chart_theme<'a>(
  import: &'a PowerPointImport,
  slide: &SlidePersist,
) -> Option<&'a ThemeFragmentRecord> {
  slide
    .theme_path
    .as_deref()
    .and_then(|path| import.get_theme(path))
    .or_else(|| import.get_current_theme_ptr())
}

#[derive(Clone, Copy)]
struct PptxLoweringContext<'a> {
  import: &'a PowerPointImport,
  slide: &'a SlidePersist,
  page_index: usize,
  ui_language: Option<&'a str>,
}

fn lower_diagram(
  context: PptxLoweringContext<'_>,
  shape: &Shape,
  offset: DisplayOffset,
  record: &GraphicDataRecord,
  items: &mut Vec<PageItem>,
  mut summary: Option<&mut PptxLayoutSummary>,
) {
  let Some(data_resource) = record.diagram_data_resource.as_ref() else {
    return;
  };
  let x_pt = units::emu_to_points(offset.x_emu + shape.position.x);
  let y_pt = units::emu_to_points(offset.y_emu + shape.position.y);
  let width_pt = units::emu_to_points(shape.size.cx);
  let height_pt = units::emu_to_points(shape.size.cy);
  if let Some(background_fill) =
    diagram_background_fill(context.import, context.slide, &data_resource.model)
  {
    items.push(PageItem::Rect(RectItem {
      x_pt,
      y_pt,
      width_pt,
      height_pt,
      fill_color: Some(background_fill),
      fill_opacity: 1.0,
      stroke: None,
      stroke_opacity: 1.0,
    }));
  }
  if let Some(drawing) = diagram_drawing_resource(context.slide, &data_resource.model)
    && lower_diagram_drawing(
      context,
      drawing,
      &data_resource.model,
      TextFrame {
        x_pt,
        y_pt,
        width_pt,
        height_pt,
      },
      items,
      summary.as_deref_mut(),
    )
  {
    return;
  }
  let fill = layout_rgb_color(diagram_accent_fill(context.import, context.slide));
  let styles = diagram_styles(record);
  let colors = diagram_style_colors(context.import, context.slide, record);
  let shapes = shared_diagram::layout_shapes(
    &data_resource.model,
    record
      .diagram_layout_resource
      .as_ref()
      .map(|resource| &resource.layout),
    styles.as_ref(),
    colors.as_ref(),
    shared_diagram::DiagramBounds {
      x: x_pt,
      y: y_pt,
      width: width_pt,
      height: height_pt,
    },
    fill,
  );
  let mut drawing_items = Vec::new();
  let mut text_items = Vec::new();
  let mut pending_text_items = Vec::new();
  let mut font_sync_scales: HashMap<String, (f32, f32)> = HashMap::new();
  for diagram_shape in shapes {
    let fill_images = diagram_shape
      .shape_properties
      .as_deref()
      .map(|properties| {
        diagram_model_shape_blip_fill_image_items(
          context.import,
          context.slide,
          data_resource,
          properties,
          shared_diagram::DiagramBounds {
            x: diagram_shape.x,
            y: diagram_shape.y,
            width: diagram_shape.width,
            height: diagram_shape.height,
          },
        )
      })
      .unwrap_or_default();
    if fill_images.is_empty()
      && diagram_shape.is_blip_placeholder
      && let Some(item) = diagram_blip_placeholder_image_item(shared_diagram::DiagramBounds {
        x: diagram_shape.x,
        y: diagram_shape.y,
        width: diagram_shape.width,
        height: diagram_shape.height,
      })
    {
      drawing_items.push(PageItem::Image(item));
    }
    drawing_items.extend(fill_images.into_iter().map(PageItem::Image));
    let fill_color = diagram_shape
      .shape_properties
      .as_deref()
      .and_then(|properties| {
        diagram_model_shape_fill_color(context.import, context.slide, properties)
      })
      .unwrap_or_else(|| pdf_rgb_color(diagram_shape.fill));
    let suppress_fill = diagram_shape
      .shape_properties
      .as_deref()
      .is_some_and(diagram_model_shape_suppresses_fill);
    if diagram_shape.is_connector {
      drawing_items.push(PageItem::Line(diagram_connector_line_item(&diagram_shape)));
    } else {
      drawing_items.push(PageItem::Rect(RectItem {
        x_pt: diagram_shape.x,
        y_pt: diagram_shape.y,
        width_pt: diagram_shape.width,
        height_pt: diagram_shape.height,
        fill_color: diagram_shape
          .shape_properties
          .as_deref()
          .is_none_or(|properties| !diagram_model_shape_has_blip_fill(properties))
          .then_some(fill_color)
          .filter(|_| !suppress_fill),
        fill_opacity: 1.0,
        stroke: diagram_shape
          .shape_properties
          .as_deref()
          .and_then(|properties| {
            diagram_model_shape_outline(context.import, context.slide, properties)
          })
          .or_else(|| Some(BorderStyle::default()).filter(|_| !suppress_fill)),
        stroke_opacity: 1.0,
      }));
    }
    if !diagram_shape.text_body.is_empty() {
      let mut text_body = diagram_text_body(&diagram_shape.text_body);
      if diagram_shape.text_rotation_deg != 0.0 {
        let rotation = (diagram_shape.text_rotation_deg * 60_000.0).round() as i32;
        text_body.display_properties.text_area_rotation = Some(
          text_body
            .display_properties
            .text_area_rotation
            .unwrap_or_default()
            + rotation,
        );
      }
      let text_frame = text_body_frame(
        diagram_shape.x,
        diagram_shape.y,
        diagram_shape.width,
        diagram_shape.height,
        &text_body,
      );
      record_smartart_text_shape(
        summary.as_deref_mut(),
        context.page_index,
        &text_body,
        diagram_shape.x,
        diagram_shape.y,
        text_frame,
      );
      let font_reference = diagram_shape.style.as_deref().map(|style| {
        diagram_font_style_reference(
          &style.font_reference,
          diagram_shape.text_fill.map(pdf_rgb_color),
        )
      });
      let options = TextLoweringOptions::from_text_body(&text_body);
      let (font_scale, line_scale) = text_auto_fit_scales(&options);
      let sync_auto_fit = text_body.display_properties.auto_fit == TextAutoFit::Shape;
      if sync_auto_fit && let Some(group) = diagram_shape.font_sync_group.as_deref() {
        font_sync_scales
          .entry(group.to_string())
          .and_modify(|scale| {
            if font_scale < scale.0 || (font_scale == scale.0 && line_scale < scale.1) {
              *scale = (font_scale, line_scale);
            }
          })
          .or_insert((font_scale, line_scale));
      }
      pending_text_items.push(PendingDiagramTextItem {
        order: diagram_shape.text_order,
        frame: text_frame,
        text_body,
        font_reference,
        base_font_size_pt: diagram_shape.font_size_pt,
        font_sync_group: diagram_shape.font_sync_group,
        sync_auto_fit,
        font_scale,
        line_scale,
      });
    }
  }
  for pending in pending_text_items {
    let (font_scale, line_scale) = pending
      .font_sync_group
      .as_deref()
      .filter(|_| pending.sync_auto_fit)
      .and_then(|group| font_sync_scales.get(group).copied())
      .unwrap_or((pending.font_scale, pending.line_scale));
    lower_diagram_text_body_at_with_style_and_scale(
      context.import,
      pending.frame,
      &pending.text_body,
      DiagramTextLoweringStyle {
        font_reference: pending.font_reference.as_ref(),
        table_text_style: None,
        shape_hyperlink_url: None,
        base_font_size_pt: pending.base_font_size_pt,
        font_scale,
        line_scale,
        shape_order: pending.order,
      },
      &mut text_items,
    );
  }
  text_items.sort_by_key(|text_item| (text_item.paragraph_order, text_item.order));
  items.extend(drawing_items);
  items.extend(text_items.into_iter().map(|text_item| text_item.item));
}

fn diagram_drawing_resource<'a>(
  slide: &'a SlidePersist,
  data: &dgm::DataModelRoot,
) -> Option<&'a super::slide::DiagramDrawingResource> {
  let extensions = data.data_model_extension_list.as_ref()?;
  for extension in &extensions.data_model_extension {
    if let Some(a::DataModelExtensionChoice::DataModelExtensionBlock(block)) =
      extension.data_model_extension_choice.as_ref()
      && let Some(rel_id) = block.rel_id.as_deref()
      && let Some(resource) = slide.diagram_drawing_resources.get(rel_id)
    {
      return Some(resource);
    }
  }
  None
}

fn lower_diagram_drawing(
  context: PptxLoweringContext<'_>,
  drawing_resource: &super::slide::DiagramDrawingResource,
  data: &dgm::DataModelRoot,
  frame: TextFrame,
  items: &mut Vec<PageItem>,
  mut summary: Option<&mut PptxLayoutSummary>,
) -> bool {
  // imports persisted diagramDrawing extDrawing before falling back to layout
  // atom shape generation.
  let transform = DiagramDrawingTransform::root(
    frame.x_pt,
    frame.y_pt,
    frame.width_pt,
    frame.height_pt,
    drawing_resource
      .drawing
      .shape_tree
      .group_shape_properties
      .transform_group
      .as_deref(),
  );
  let text_orders = shared_diagram::presentation_point_list_orders(data);
  let drawing_context = DiagramDrawingLoweringContext {
    import: context.import,
    slide: context.slide,
    drawing_resource,
    text_orders: &text_orders,
    page_index: context.page_index,
  };
  let mut drawing_items = Vec::new();
  let mut text_items = Vec::new();
  for choice in &drawing_resource.drawing.shape_tree.shape_tree_choice {
    match choice {
      dsp::ShapeTreeChoice::Shape(shape) => lower_diagram_drawing_shape(
        drawing_context,
        shape,
        transform,
        &mut drawing_items,
        &mut text_items,
        summary.as_deref_mut(),
      ),
      dsp::ShapeTreeChoice::GroupShape(group) => lower_diagram_drawing_group(
        drawing_context,
        group,
        transform,
        &mut drawing_items,
        &mut text_items,
        summary.as_deref_mut(),
      ),
    }
  }
  if drawing_items.is_empty() && text_items.is_empty() {
    return false;
  }
  text_items.sort_by_key(|text_item| text_item.order);
  items.extend(drawing_items);
  items.extend(text_items.into_iter().map(|text_item| text_item.item));
  true
}

#[derive(Clone, Copy)]
struct DiagramDrawingLoweringContext<'a> {
  import: &'a PowerPointImport,
  slide: &'a SlidePersist,
  drawing_resource: &'a super::slide::DiagramDrawingResource,
  text_orders: &'a HashMap<String, usize>,
  page_index: usize,
}

fn lower_diagram_drawing_group(
  context: DiagramDrawingLoweringContext<'_>,
  group: &dsp::GroupShape,
  parent_transform: DiagramDrawingTransform,
  items: &mut Vec<PageItem>,
  text_items: &mut Vec<DiagramDrawingTextItem>,
  mut summary: Option<&mut PptxLayoutSummary>,
) {
  let transform =
    parent_transform.for_group(group.group_shape_properties.transform_group.as_deref());
  for choice in &group.group_shape_choice {
    match choice {
      dsp::GroupShapeChoice::Shape(shape) => lower_diagram_drawing_shape(
        context,
        shape,
        transform,
        items,
        text_items,
        summary.as_deref_mut(),
      ),
      dsp::GroupShapeChoice::GroupShape(group) => lower_diagram_drawing_group(
        context,
        group,
        transform,
        items,
        text_items,
        summary.as_deref_mut(),
      ),
    }
  }
}

fn lower_diagram_drawing_shape(
  context: DiagramDrawingLoweringContext<'_>,
  shape: &dsp::Shape,
  transform: DiagramDrawingTransform,
  items: &mut Vec<PageItem>,
  text_items: &mut Vec<DiagramDrawingTextItem>,
  mut summary: Option<&mut PptxLayoutSummary>,
) {
  let Some(bounds) = diagram_shape_bounds(&shape.shape_properties, transform) else {
    return;
  };
  if let Some(summary) = summary.as_deref_mut() {
    record_diagram_draw_shape_summary(summary, context.page_index, shape, bounds, transform);
  }
  let fill_color = diagram_shape_fill_color(context.import, context.slide, &shape.shape_properties)
    .unwrap_or(RgbColor {
      r: 255,
      g: 255,
      b: 255,
    });
  let fill_images = diagram_shape_blip_fill_image_items(
    context.import,
    context.slide,
    context.drawing_resource,
    &shape.shape_properties,
    bounds,
  );
  items.extend(fill_images.into_iter().map(PageItem::Image));
  items.push(PageItem::Rect(RectItem {
    x_pt: bounds.x,
    y_pt: bounds.y,
    width_pt: bounds.width,
    height_pt: bounds.height,
    fill_color: (!diagram_shape_has_blip_fill(&shape.shape_properties)).then_some(fill_color),
    fill_opacity: 1.0,
    stroke: Some(BorderStyle::default()),
    stroke_opacity: 1.0,
  }));
  let Some(text_body) = shape.text_body.as_deref() else {
    return;
  };
  let mut text_body = TextBody::from_diagram_drawing(text_body);
  if text_body
    .paragraphs
    .iter()
    .flat_map(|paragraph| &paragraph.runs)
    .all(|run| run.text.trim().is_empty())
  {
    return;
  }
  let text_frame = diagram_drawing_text_frame(shape, bounds, transform, &text_body);
  record_smartart_text_shape(
    summary,
    context.page_index,
    &text_body,
    text_frame.text_area_x_pt,
    text_frame.text_area_y_pt,
    text_frame.frame,
  );
  let shape_rotation = shape
    .shape_properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.rotation)
    .unwrap_or_default();
  let text_rotation = shape
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.rotation)
    .unwrap_or_default();
  let total_rotation = shape_rotation + text_rotation;
  if total_rotation != 0 {
    text_body.display_properties.text_area_rotation = Some(
      text_body
        .display_properties
        .text_area_rotation
        .unwrap_or_default()
        - total_rotation,
    );
  }
  let font_reference = shape
    .shape_style
    .as_deref()
    .map(|style| diagram_font_style_reference(&style.font_reference, None));
  let mut lowered_text_items = Vec::new();
  lower_text_body_at_with_style(
    context.import,
    text_frame.frame,
    &text_body,
    TextStyleLoweringInputs {
      font_reference: font_reference.as_ref(),
      rotation_center_pt: text_frame.rotation_center_pt,
      ..TextStyleLoweringInputs::default()
    },
    TextLoweringRuntime {
      slide: Some(context.slide),
      ..TextLoweringRuntime::default()
    },
    None,
    &mut lowered_text_items,
  );
  for item in &mut lowered_text_items {
    if let PageItem::Text(text) = item {
      text.preserve_text_portion = true;
    }
  }
  let order = context
    .text_orders
    .get(shape.model_id.as_str())
    .copied()
    .unwrap_or(usize::MAX);
  text_items.extend(
    lowered_text_items
      .into_iter()
      .map(|item| DiagramDrawingTextItem {
        order,
        paragraph_order: 0,
        item,
      }),
  );
}

struct DiagramDrawingTextItem {
  order: usize,
  paragraph_order: usize,
  item: PageItem,
}

struct DiagramTextLoweringStyle<'a> {
  font_reference: Option<&'a FontStyleReference>,
  table_text_style: Option<&'a TableStyleTextProperties>,
  shape_hyperlink_url: Option<&'a str>,
  base_font_size_pt: Option<f32>,
  font_scale: f32,
  line_scale: f32,
  shape_order: usize,
}

fn lower_diagram_text_body_at_with_style_and_scale(
  import: &PowerPointImport,
  frame: TextFrame,
  text_body: &TextBody,
  style_inputs: DiagramTextLoweringStyle<'_>,
  items: &mut Vec<DiagramDrawingTextItem>,
) {
  let mut options = TextLoweringOptions::from_text_body(text_body);
  options.font_scale = style_inputs.font_scale;
  options.line_scale = style_inputs.line_scale;
  options.rotation_center_pt = rotated_text_area_center(frame, options.rotation_deg);
  let base_style = text_base_style(
    import,
    None,
    text_body,
    style_inputs.table_text_style,
    style_inputs.base_font_size_pt,
  );
  let mut text_metrics = TextMetrics::new();
  let estimated_height = estimate_wrapped_text_body_height(
    TextBodyHeightContext {
      import,
      slide: None,
      frame,
      base_style: &base_style,
      font_reference: style_inputs.font_reference,
      options: &options,
      slide_number: 1,
    },
    text_body,
    &mut text_metrics,
  );
  let y_pt = match text_body.display_properties.anchor {
    a::TextAnchoringTypeValues::Center => frame.y_pt + (frame.height_pt - estimated_height) / 2.0,
    a::TextAnchoringTypeValues::Bottom => frame.y_pt + frame.height_pt - estimated_height,
    a::TextAnchoringTypeValues::Top
    | a::TextAnchoringTypeValues::Justified
    | a::TextAnchoringTypeValues::Distributed => frame.y_pt,
  };

  let mut cursor = TextCursor {
    x_pt: frame.x_pt,
    y_pt,
    column_index: 0,
  };
  let mut auto_numbering = AutoNumberingState::default();
  for (paragraph_index, paragraph) in text_body.paragraphs.iter().enumerate() {
    let mut paragraph_items = Vec::new();
    lower_paragraph(
      ParagraphLoweringContext {
        import,
        slide: None,
        base_style: &base_style,
        font_reference: style_inputs.font_reference,
        options: &options,
        frame,
        shape_hyperlink_url: style_inputs.shape_hyperlink_url,
        image_resources: None,
        page_index: 0,
        slide_number: 1,
        paragraph_count: text_body.paragraphs.len(),
      },
      paragraph,
      paragraph_index,
      ParagraphLoweringOutput {
        summary: None,
        cursor: &mut cursor,
        items: &mut paragraph_items,
        text_metrics: &mut text_metrics,
        auto_numbering: &mut auto_numbering,
      },
    );
    let order = paragraph
      .diagram_source_order
      .unwrap_or(style_inputs.shape_order);
    items.extend(paragraph_items.into_iter().map(|mut item| {
      if let PageItem::Text(text_item) = &mut item {
        text_item.preserve_text_portion = true;
      }
      DiagramDrawingTextItem {
        order: style_inputs.shape_order,
        paragraph_order: order,
        item,
      }
    }));
  }
}

struct PendingDiagramTextItem {
  order: usize,
  frame: TextFrame,
  text_body: TextBody,
  font_reference: Option<FontStyleReference>,
  base_font_size_pt: Option<f32>,
  font_sync_group: Option<String>,
  sync_auto_fit: bool,
  font_scale: f32,
  line_scale: f32,
}

fn diagram_connector_line_item(diagram_shape: &shared_diagram::DiagramShape) -> LineItem {
  let center_x = diagram_shape.x + diagram_shape.width / 2.0;
  let center_y = diagram_shape.y + diagram_shape.height / 2.0;
  let length = diagram_shape.width.max(diagram_shape.height).max(1.0);
  let radians = diagram_shape.connector_angle_deg.to_radians();
  let dx = radians.cos() * length / 2.0;
  let dy = radians.sin() * length / 2.0;
  LineItem {
    x1_pt: center_x - dx,
    y1_pt: center_y - dy,
    x2_pt: center_x + dx,
    y2_pt: center_y + dy,
    width_pt: 1.0,
    color: RgbColor { r: 0, g: 0, b: 0 },
    kind: LineItemKind::Stroke,
  }
}

fn diagram_blip_placeholder_image_item(bounds: shared_diagram::DiagramBounds) -> Option<ImageItem> {
  if bounds.width <= f32::EPSILON || bounds.height <= f32::EPSILON {
    return None;
  }
  Some(ImageItem {
    x_pt: bounds.x,
    y_pt: bounds.y,
    width_pt: bounds.width,
    height_pt: bounds.height,
    crop: ImageCrop::default(),
    clip_path: Vec::new(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    data: transparent_png_1x1()?,
    content_type: Some("image/png".to_string()),
    alt_text: None,
    hyperlink_url: None,
    floating: false,
    behind_text: false,
  })
}

fn diagram_text_body(source: &shared_diagram::DiagramTextBody) -> TextBody {
  let mut display_properties = source
    .body_properties
    .as_deref()
    .map(TextBodyDisplayProperties::from_body_properties)
    .unwrap_or_default();
  // creates SmartArt text shapes from diagram layout constraints first; text
  // size is then synchronized/autofit inside that fixed layout. Do not apply
  // the generic DrawingML shape word-wrap default here, or persisted SmartArt
  // text areas wrap instead of shrinking/syncing in the LO order.
  display_properties.word_wrap = false;
  if source.auto_fit {
    display_properties.auto_fit = TextAutoFit::Shape;
  }
  TextBody {
    has_body_properties: source.body_properties.is_some(),
    has_noninherited_body_properties: source
      .body_properties
      .as_deref()
      .is_some_and(has_noninherited_body_properties),
    body_properties: source.body_properties.clone(),
    display_properties,
    has_list_style: source.list_style.is_some(),
    list_style: source
      .list_style
      .as_deref()
      .map(TextListStyle::from_dml_list_style),
    paragraphs: source
      .paragraphs
      .iter()
      .map(|paragraph| TextParagraph {
        diagram_source_order: paragraph.source_order,
        level: paragraph.level,
        paragraph_properties: paragraph.paragraph_properties.clone(),
        end_paragraph_run_properties: paragraph.end_paragraph_run_properties.clone(),
        master_paragraph_style: None,
        text_paragraph_style: None,
        runs: paragraph
          .runs
          .iter()
          .map(|run| TextRun {
            text: run.text.clone(),
            kind: match run.kind {
              shared_diagram::DiagramTextRunKind::Run => TextRunKind::Run,
              shared_diagram::DiagramTextRunKind::Break => TextRunKind::Break,
              shared_diagram::DiagramTextRunKind::Field => TextRunKind::Field,
              shared_diagram::DiagramTextRunKind::Math => TextRunKind::Math,
            },
            hyperlink_url: None,
            field_type: run.field_type.clone(),
            run_properties: run.run_properties.clone(),
            field_paragraph_properties: run.field_paragraph_properties.clone(),
          })
          .collect(),
      })
      .collect(),
  }
}

fn diagram_model_shape_fill_color(
  import: &PowerPointImport,
  slide: &SlidePersist,
  properties: &dgm::ShapeProperties,
) -> Option<RgbColor> {
  let fill = match properties.shape_properties_choice2.as_ref()? {
    dgm::ShapePropertiesChoice2::SolidFill(fill) => fill,
    _ => return None,
  };
  let color = Color::from_solid_fill_choice(fill.solid_fill_choice.as_ref()?)?;
  let resolved = import.resolve_color_for_slide(slide, &color, None)?;
  Some(RgbColor {
    r: resolved.r,
    g: resolved.g,
    b: resolved.b,
  })
}

fn diagram_model_shape_has_blip_fill(properties: &dgm::ShapeProperties) -> bool {
  matches!(
    properties.shape_properties_choice2.as_ref(),
    Some(dgm::ShapePropertiesChoice2::BlipFill(_))
  )
}

fn diagram_model_shape_suppresses_fill(properties: &dgm::ShapeProperties) -> bool {
  matches!(
    properties.shape_properties_choice2.as_ref(),
    Some(dgm::ShapePropertiesChoice2::NoFill(_) | dgm::ShapePropertiesChoice2::BlipFill(_))
  )
}

fn diagram_model_shape_outline(
  import: &PowerPointImport,
  slide: &SlidePersist,
  properties: &dgm::ShapeProperties,
) -> Option<BorderStyle> {
  let outline = properties.outline.as_deref()?;
  let color = match outline.outline_choice1.as_ref()? {
    a::OutlineChoice::NoFill(_) => return None,
    a::OutlineChoice::SolidFill(fill) => {
      let color = Color::from_solid_fill_choice(fill.solid_fill_choice.as_ref()?)?;
      import.resolve_color_for_slide(slide, &color, None)?
    }
    _ => return None,
  };
  Some(BorderStyle {
    width_pt: outline
      .width
      .map(|width| units::emu_to_points(i64::from(width)))
      .unwrap_or(0.5),
    spacing_pt: 0.0,
    color: RgbColor {
      r: color.r,
      g: color.g,
      b: color.b,
    },
    compound: false,
  })
}

fn diagram_model_shape_blip_fill_image_items(
  import: &PowerPointImport,
  slide: &SlidePersist,
  data_resource: &super::slide::DiagramDataResource,
  properties: &dgm::ShapeProperties,
  bounds: shared_diagram::DiagramBounds,
) -> Vec<ImageItem> {
  let Some(dgm::ShapePropertiesChoice2::BlipFill(blip_fill)) =
    properties.shape_properties_choice2.as_ref()
  else {
    return Vec::new();
  };
  let Some(blip) = blip_fill.blip.as_ref() else {
    return Vec::new();
  };
  let Some(relationship_id) = blip.embed.as_deref() else {
    return Vec::new();
  };
  let Some(resource) = data_resource.image_resources.get(relationship_id) else {
    return Vec::new();
  };
  let rotation_deg = properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.rotation)
    .map(|rotation| rotation as f32 / 60000.0)
    .unwrap_or_default();
  let flip_horizontal = properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.horizontal_flip)
    .map(bool::from)
    .unwrap_or(false);
  let flip_vertical = properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.vertical_flip)
    .map(bool::from)
    .unwrap_or(false);
  blip_fill_image_items_from_resource(
    import,
    slide,
    blip_fill,
    blip,
    resource,
    ImageFillPlacement {
      frame: TextFrame {
        x_pt: bounds.x,
        y_pt: bounds.y,
        width_pt: bounds.width,
        height_pt: bounds.height,
      },
      rotation_deg,
      flip_horizontal,
      flip_vertical,
      crop_bitmap: false,
      clip_path: Vec::new(),
      alt_text: None,
      hyperlink_url: None,
    },
  )
}

#[derive(Clone, Copy, Debug)]
struct DiagramDrawingTransform {
  xx: f32,
  xy: f32,
  yx: f32,
  yy: f32,
  tx_pt: f32,
  ty_pt: f32,
}

impl DiagramDrawingTransform {
  fn root(
    x_pt: f32,
    y_pt: f32,
    width_pt: f32,
    height_pt: f32,
    transform: Option<&a::TransformGroup>,
  ) -> Self {
    let mut root = Self {
      xx: 1.0,
      xy: 0.0,
      yx: 0.0,
      yy: 1.0,
      tx_pt: x_pt,
      ty_pt: y_pt,
    };
    root = root.for_group_transform(transform, width_pt, height_pt);
    root
  }

  fn for_group(self, transform: Option<&a::TransformGroup>) -> Self {
    self.for_group_transform(transform, None, None)
  }

  fn for_group_transform(
    self,
    transform: Option<&a::TransformGroup>,
    width_pt: impl Into<Option<f32>>,
    height_pt: impl Into<Option<f32>>,
  ) -> Self {
    let Some(transform) = transform else {
      return self;
    };
    let off_x = transform
      .offset
      .as_ref()
      .map(|offset| units::emu_to_points(offset.x.to_emu()))
      .unwrap_or_default();
    let off_y = transform
      .offset
      .as_ref()
      .map(|offset| units::emu_to_points(offset.y.to_emu()))
      .unwrap_or_default();
    let ext_width = transform
      .extents
      .as_ref()
      .map(|extents| units::emu_to_points(extents.cx.to_emu()))
      .or_else(|| width_pt.into())
      .unwrap_or_default();
    let ext_height = transform
      .extents
      .as_ref()
      .map(|extents| units::emu_to_points(extents.cy.to_emu()))
      .or_else(|| height_pt.into())
      .unwrap_or_default();
    let child_x = transform
      .child_offset
      .as_ref()
      .map(|offset| units::emu_to_points(offset.x.to_emu()))
      .unwrap_or_default();
    let child_y = transform
      .child_offset
      .as_ref()
      .map(|offset| units::emu_to_points(offset.y.to_emu()))
      .unwrap_or_default();
    let child_width = transform
      .child_extents
      .as_ref()
      .map(|extents| units::emu_to_points(extents.cx.to_emu()))
      .unwrap_or(ext_width);
    let child_height = transform
      .child_extents
      .as_ref()
      .map(|extents| units::emu_to_points(extents.cy.to_emu()))
      .unwrap_or(ext_height);
    let scale_x = if child_width != 0.0 {
      ext_width / child_width
    } else {
      1.0
    };
    let scale_y = if child_height != 0.0 {
      ext_height / child_height
    } else {
      1.0
    };
    let group = Self {
      xx: scale_x,
      xy: 0.0,
      yx: 0.0,
      yy: scale_y,
      tx_pt: off_x - child_x * scale_x,
      ty_pt: off_y - child_y * scale_y,
    };
    self.concat(group)
  }

  fn concat(self, child: Self) -> Self {
    Self {
      xx: self.xx * child.xx + self.xy * child.yx,
      xy: self.xx * child.xy + self.xy * child.yy,
      yx: self.yx * child.xx + self.yy * child.yx,
      yy: self.yx * child.xy + self.yy * child.yy,
      tx_pt: self.xx * child.tx_pt + self.xy * child.ty_pt + self.tx_pt,
      ty_pt: self.yx * child.tx_pt + self.yy * child.ty_pt + self.ty_pt,
    }
  }

  fn apply_point(self, x: f32, y: f32) -> (f32, f32) {
    (
      self.xx * x + self.xy * y + self.tx_pt,
      self.yx * x + self.yy * y + self.ty_pt,
    )
  }

  fn apply_bounds(self, x: f32, y: f32, width: f32, height: f32) -> shared_diagram::DiagramBounds {
    let points = [
      self.apply_point(x, y),
      self.apply_point(x + width, y),
      self.apply_point(x, y + height),
      self.apply_point(x + width, y + height),
    ];
    let left = points.iter().map(|(x, _)| *x).fold(f32::INFINITY, f32::min);
    let top = points.iter().map(|(_, y)| *y).fold(f32::INFINITY, f32::min);
    let right = points
      .iter()
      .map(|(x, _)| *x)
      .fold(f32::NEG_INFINITY, f32::max);
    let bottom = points
      .iter()
      .map(|(_, y)| *y)
      .fold(f32::NEG_INFINITY, f32::max);
    shared_diagram::DiagramBounds {
      x: left,
      y: top,
      width: right - left,
      height: bottom - top,
    }
  }
}

fn diagram_shape_bounds(
  properties: &dsp::ShapeProperties,
  parent_transform: DiagramDrawingTransform,
) -> Option<shared_diagram::DiagramBounds> {
  let shape_transform = properties.transform2_d.as_deref()?;
  let offset = shape_transform.offset.as_ref()?;
  let extents = shape_transform.extents.as_ref()?;
  Some(parent_transform.apply_bounds(
    units::emu_to_points(offset.x.to_emu()),
    units::emu_to_points(offset.y.to_emu()),
    units::emu_to_points(extents.cx.to_emu()),
    units::emu_to_points(extents.cy.to_emu()),
  ))
}

fn diagram_text_transform_bounds(
  transform: &dsp::Transform2D,
  parent_transform: DiagramDrawingTransform,
) -> Option<shared_diagram::DiagramBounds> {
  let offset = transform.offset.as_ref()?;
  let extents = transform.extents.as_ref()?;
  Some(parent_transform.apply_bounds(
    units::emu_to_points(offset.x.to_emu()),
    units::emu_to_points(offset.y.to_emu()),
    units::emu_to_points(extents.cx.to_emu()),
    units::emu_to_points(extents.cy.to_emu()),
  ))
}

fn diagram_drawing_text_frame(
  shape: &dsp::Shape,
  shape_bounds: shared_diagram::DiagramBounds,
  parent_transform: DiagramDrawingTransform,
  text_body: &TextBody,
) -> DiagramDrawingTextFrame {
  let Some(text_transform) = shape.transform2_d.as_deref() else {
    return DiagramDrawingTextFrame::new(
      text_body_frame(
        shape_bounds.x,
        shape_bounds.y,
        shape_bounds.width,
        shape_bounds.height,
        text_body,
      ),
      shape_bounds.x,
      shape_bounds.y,
    );
  };
  let Some(mut text_bounds) = diagram_text_transform_bounds(text_transform, parent_transform)
  else {
    return DiagramDrawingTextFrame::new(
      text_body_frame(
        shape_bounds.x,
        shape_bounds.y,
        shape_bounds.width,
        shape_bounds.height,
        text_body,
      ),
      shape_bounds.x,
      shape_bounds.y,
    );
  };
  let Some(preset_bounds) = diagram_preset_text_rectangle(shape, shape_bounds) else {
    return DiagramDrawingTextFrame::new(
      text_body_frame(
        shape_bounds.x,
        shape_bounds.y,
        shape_bounds.width,
        shape_bounds.height,
        text_body,
      ),
      shape_bounds.x,
      shape_bounds.y,
    );
  };

  let shape_rotation = shape
    .shape_properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.rotation)
    .unwrap_or_default();
  let text_rotation = text_transform.rotation.unwrap_or_default();
  let angle_diff = (shape_rotation + text_rotation).rem_euclid(21_600_000);
  if angle_diff != 0 {
    let angle = -(angle_diff as f32 / 60_000.0).to_radians();
    let preset_center_x = preset_bounds.x + preset_bounds.width / 2.0;
    let preset_center_y = preset_bounds.y + preset_bounds.height / 2.0;
    let text_center_x = text_bounds.x + text_bounds.width / 2.0;
    let text_center_y = text_bounds.y + text_bounds.height / 2.0;
    let dx = text_center_x - preset_center_x;
    let dy = text_center_y - preset_center_y;
    let rotated_center_x = preset_center_x + dx * angle.cos() - dy * angle.sin();
    let rotated_center_y = preset_center_y + dx * angle.sin() + dy * angle.cos();
    text_bounds.x += rotated_center_x - text_center_x;
    text_bounds.y += rotated_center_y - text_center_y;
  }

  let offsets = TextDistances {
    left: text_bounds.x - preset_bounds.x,
    top: text_bounds.y - preset_bounds.y,
    right: preset_bounds.width - text_bounds.width - (text_bounds.x - preset_bounds.x),
    bottom: preset_bounds.height - text_bounds.height - (text_bounds.y - preset_bounds.y),
  };
  let frame = text_body_frame_with_distances(
    preset_bounds.x,
    preset_bounds.y,
    preset_bounds.width,
    preset_bounds.height,
    text_body,
    offsets,
    0,
  );
  let text_distances_100mm = Some(text_distances_from_frame(
    preset_bounds.x,
    preset_bounds.y,
    preset_bounds.width,
    preset_bounds.height,
    frame,
  ));
  let mut frame = frame;
  let shape_rotation = shape_rotation.rem_euclid(21_600_000);
  if shape_rotation != 0 {
    let angle = (shape_rotation as f32 / 60_000.0).to_radians();
    let (x, y) = rotate_point(
      frame.x_pt,
      frame.y_pt,
      shape_bounds.x + shape_bounds.width / 2.0,
      shape_bounds.y + shape_bounds.height / 2.0,
      angle,
    );
    frame.x_pt = x;
    frame.y_pt = y;
  }
  DiagramDrawingTextFrame {
    frame,
    text_area_x_pt: preset_bounds.x,
    text_area_y_pt: preset_bounds.y,
    rotation_center_pt: (angle_diff != 0).then_some((frame.x_pt, frame.y_pt)),
    text_distances_100mm,
  }
}

#[derive(Clone, Copy, Debug)]
struct DiagramDrawingTextFrame {
  frame: TextFrame,
  text_area_x_pt: f32,
  text_area_y_pt: f32,
  rotation_center_pt: Option<(f32, f32)>,
  text_distances_100mm: Option<ShapeTextDistances100mm>,
}

impl DiagramDrawingTextFrame {
  fn new(frame: TextFrame, text_area_x_pt: f32, text_area_y_pt: f32) -> Self {
    Self {
      frame,
      text_area_x_pt,
      text_area_y_pt,
      rotation_center_pt: None,
      text_distances_100mm: Some(text_distances_from_frame(
        text_area_x_pt,
        text_area_y_pt,
        frame.width_pt,
        frame.height_pt,
        frame,
      )),
    }
  }
}

fn diagram_preset_text_rectangle(
  shape: &dsp::Shape,
  bounds: shared_diagram::DiagramBounds,
) -> Option<shared_diagram::DiagramBounds> {
  let preset = match shape.shape_properties.shape_properties_choice1.as_ref()? {
    dsp::ShapePropertiesChoice::PresetGeometry(preset) => preset.as_ref(),
    dsp::ShapePropertiesChoice::CustomGeometry(_) => return None,
  };
  let guide = |index: usize, default: f32| {
    preset
      .adjust_value_list
      .as_ref()
      .and_then(|list| list.shape_guide.get(index))
      .and_then(|guide| guide_value(guide.formula.as_str()))
      .unwrap_or(default)
  };
  match preset.preset {
    a::ShapeTypeValues::Ellipse => {
      let factor = (1.0 - std::f32::consts::FRAC_1_SQRT_2) / 2.0;
      Some(shared_diagram::DiagramBounds {
        x: bounds.x + bounds.width * factor,
        y: bounds.y + bounds.height * factor,
        width: bounds.width * std::f32::consts::FRAC_1_SQRT_2,
        height: bounds.height * std::f32::consts::FRAC_1_SQRT_2,
      })
    }
    a::ShapeTypeValues::RoundRectangle | a::ShapeTypeValues::Round2SameRectangle => {
      let min_size = bounds.width.min(bounds.height);
      if min_size <= 0.0 {
        return None;
      }
      let max_adj = 50_000.0 * bounds.width / min_size;
      let adj = guide(0, 16_667.0).clamp(0.0, max_adj);
      let text_left = min_size * adj / 100_000.0 * 0.29289;
      let height_factor = if preset.preset == a::ShapeTypeValues::RoundRectangle {
        2.0
      } else {
        1.0
      };
      Some(shared_diagram::DiagramBounds {
        x: bounds.x + text_left,
        y: bounds.y + text_left,
        width: bounds.width - 2.0 * text_left,
        height: bounds.height - height_factor * text_left,
      })
    }
    a::ShapeTypeValues::Trapezoid => {
      let min_size = bounds.width.min(bounds.height);
      if min_size <= 0.0 {
        return None;
      }
      let max_adj = 50_000.0 * bounds.width / min_size;
      let adj = guide(0, 25_000.0).clamp(0.0, max_adj);
      let text_left = bounds.width / 3.0 * adj / max_adj;
      let text_top = bounds.height / 3.0 * adj / max_adj;
      Some(shared_diagram::DiagramBounds {
        x: bounds.x + text_left,
        y: bounds.y + text_top,
        width: bounds.width - 2.0 * text_left,
        height: bounds.height - 2.0 * text_top,
      })
    }
    a::ShapeTypeValues::FlowChartManualOperation => {
      let text_left = bounds.width / 5.0;
      Some(shared_diagram::DiagramBounds {
        x: bounds.x + text_left,
        y: bounds.y,
        width: bounds.width - 2.0 * text_left,
        height: bounds.height,
      })
    }
    a::ShapeTypeValues::Pie
    | a::ShapeTypeValues::Rectangle
    | a::ShapeTypeValues::WedgeRectangleCallout => Some(bounds),
    a::ShapeTypeValues::UpArrowCallout | a::ShapeTypeValues::DownArrowCallout => {
      let min_size = bounds.width.min(bounds.height);
      if min_size <= 0.0 || bounds.height <= 0.0 {
        return None;
      }
      let adj3 = guide(2, 25_000.0).clamp(0.0, 100_000.0 * bounds.height / min_size);
      let q2 = adj3 * min_size / bounds.height;
      let adj4 = guide(3, 64_977.0).clamp(0.0, 100_000.0 - q2);
      Some(shared_diagram::DiagramBounds {
        x: bounds.x,
        y: bounds.y,
        width: bounds.width,
        height: bounds.height * adj4 / 100_000.0,
      })
    }
    a::ShapeTypeValues::Hexagon => {
      let min_size = bounds.width.min(bounds.height);
      if min_size <= 0.0 {
        return None;
      }
      let max_adj = 50_000.0 * bounds.width / min_size;
      let adj = guide(0, 25_000.0).clamp(0.0, max_adj);
      let factor = adj / max_adj / 6.0 + 1.0 / 12.0;
      let text_left = bounds.width * factor;
      let text_top = bounds.height * factor;
      Some(shared_diagram::DiagramBounds {
        x: bounds.x + text_left,
        y: bounds.y + text_top,
        width: bounds.width - 2.0 * text_left,
        height: bounds.height - 2.0 * text_top,
      })
    }
    a::ShapeTypeValues::Gear6 => {
      if bounds.width <= 0.0 || bounds.height <= 0.0 {
        return None;
      }
      let mut a1 = guide(0, 15_000.0);
      let mut a2 = guide(1, 3_526.0);
      if preset
        .adjust_value_list
        .as_ref()
        .is_some_and(|list| list.shape_guide.len() == 2)
      {
        a1 = a1.clamp(0.0, 20_000.0);
        a2 = a2.clamp(0.0, 5_358.0);
      }
      let min_size = bounds.width.min(bounds.height);
      let tooth_height = min_size * a1 / 100_000.0;
      let half_land = min_size * a2 / 100_000.0 / 2.0;
      let diagonal = tooth_height / 2.0 + half_land;
      let radius_height = bounds.height / 2.0 - tooth_height;
      let radius_width = bounds.width / 2.0 - tooth_height;
      let max_radius = radius_width.min(radius_height);
      let ha = diagonal.atan2(max_radius);
      let angle = 330.0_f32.to_radians() - ha;
      let ta11 = radius_width * angle.cos();
      let ta12 = radius_height * angle.sin();
      let b_angle = ta12.atan2(ta11);
      let cta1 = radius_height * b_angle.cos();
      let sta1 = radius_width * b_angle.sin();
      let ma1 = cta1.hypot(sta1);
      if ma1 == 0.0 {
        return None;
      }
      let na1 = radius_width * radius_height / ma1;
      let dxa1 = na1 * b_angle.cos();
      let dya1 = na1 * b_angle.sin();
      let right = bounds.width / 2.0 + dxa1;
      let top = bounds.height / 2.0 + dya1;
      let bottom = bounds.height - top;
      let left = bounds.width - right;
      Some(shared_diagram::DiagramBounds {
        x: bounds.x + left,
        y: bounds.y + top,
        width: right - left,
        height: bottom - top,
      })
    }
    a::ShapeTypeValues::Round1Rectangle => {
      let min_size = bounds.width.min(bounds.height);
      if min_size <= 0.0 {
        return None;
      }
      let adj = guide(0, 16_667.0).clamp(0.0, 50_000.0);
      let dx = min_size * adj / 100_000.0 * 0.29289;
      Some(shared_diagram::DiagramBounds {
        x: bounds.x,
        y: bounds.y,
        width: bounds.width - dx,
        height: bounds.height,
      })
    }
    a::ShapeTypeValues::RightArrow => {
      let min_size = bounds.width.min(bounds.height);
      if min_size <= 0.0 || bounds.height <= 0.0 {
        return None;
      }
      let a1 = guide(0, 50_000.0).clamp(0.0, 100_000.0);
      let a2 = guide(1, 50_000.0).clamp(0.0, 100_000.0 * bounds.width / min_size);
      let dx1 = min_size * a2 / 100_000.0;
      let x1 = bounds.width - dx1;
      let dy1 = bounds.height * a1 / 200_000.0;
      let y1 = bounds.height / 2.0 - dy1;
      let y2 = bounds.height / 2.0 + dy1;
      let dx2 = y1 * dx1 / (bounds.height / 2.0);
      Some(shared_diagram::DiagramBounds {
        x: bounds.x,
        y: bounds.y + y1,
        width: x1 + dx2,
        height: y2 - y1,
      })
    }
    _ => None,
  }
}

fn guide_value(formula: &str) -> Option<f32> {
  formula
    .strip_prefix("val ")
    .unwrap_or(formula)
    .parse::<f32>()
    .ok()
}

fn diagram_shape_fill_color(
  import: &PowerPointImport,
  slide: &SlidePersist,
  properties: &dsp::ShapeProperties,
) -> Option<RgbColor> {
  let fill = match properties.shape_properties_choice2.as_ref()? {
    dsp::ShapePropertiesChoice2::SolidFill(fill) => fill,
    _ => return None,
  };
  let color = Color::from_solid_fill_choice(fill.solid_fill_choice.as_ref()?)?;
  let resolved = import.resolve_color_for_slide(slide, &color, None)?;
  Some(RgbColor {
    r: resolved.r,
    g: resolved.g,
    b: resolved.b,
  })
}

fn diagram_shape_has_blip_fill(properties: &dsp::ShapeProperties) -> bool {
  matches!(
    properties.shape_properties_choice2.as_ref(),
    Some(dsp::ShapePropertiesChoice2::BlipFill(_))
  )
}

fn diagram_shape_blip_fill_image_items(
  import: &PowerPointImport,
  slide: &SlidePersist,
  drawing_resource: &super::slide::DiagramDrawingResource,
  properties: &dsp::ShapeProperties,
  bounds: shared_diagram::DiagramBounds,
) -> Vec<ImageItem> {
  let Some(dsp::ShapePropertiesChoice2::BlipFill(blip_fill)) =
    properties.shape_properties_choice2.as_ref()
  else {
    return Vec::new();
  };
  let Some(blip) = blip_fill.blip.as_ref() else {
    return Vec::new();
  };
  let Some(relationship_id) = blip.embed.as_deref() else {
    return Vec::new();
  };
  let Some(resource) = drawing_resource
    .image_resources
    .get(relationship_id)
    .or_else(|| slide.image_resources.get(relationship_id))
  else {
    return Vec::new();
  };
  let rotation_deg = properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.rotation)
    .map(|rotation| rotation as f32 / 60000.0)
    .unwrap_or_default();
  let flip_horizontal = properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.horizontal_flip)
    .map(bool::from)
    .unwrap_or(false);
  let flip_vertical = properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.vertical_flip)
    .map(bool::from)
    .unwrap_or(false);
  blip_fill_image_items_from_resource(
    import,
    slide,
    blip_fill,
    blip,
    resource,
    ImageFillPlacement {
      frame: TextFrame {
        x_pt: bounds.x,
        y_pt: bounds.y,
        width_pt: bounds.width,
        height_pt: bounds.height,
      },
      rotation_deg,
      flip_horizontal,
      flip_vertical,
      crop_bitmap: false,
      clip_path: Vec::new(),
      alt_text: None,
      hyperlink_url: None,
    },
  )
}

fn diagram_background_fill(
  import: &PowerPointImport,
  slide: &SlidePersist,
  data: &dgm::DataModelRoot,
) -> Option<RgbColor> {
  let fill = match data.background.as_deref()?.background_choice1.as_ref()? {
    dgm::BackgroundChoice::SolidFill(fill) => fill,
    _ => return None,
  };
  let color = Color::from_solid_fill_choice(fill.solid_fill_choice.as_ref()?)?;
  let resolved = import.resolve_color_for_slide(slide, &color, None)?;
  Some(RgbColor {
    r: resolved.r,
    g: resolved.g,
    b: resolved.b,
  })
}

fn diagram_styles(record: &GraphicDataRecord) -> Option<shared_diagram::DiagramStyles> {
  let style_resource = record.diagram_style_resource.as_ref()?;
  let style_by_label: HashMap<String, Box<dgm::Style>> = style_resource
    .style
    .style_label
    .iter()
    .filter_map(|label| Some((label.name.clone(), label.style.clone()?)))
    .collect();
  (!style_by_label.is_empty()).then_some(shared_diagram::DiagramStyles { style_by_label })
}

fn diagram_style_colors(
  import: &PowerPointImport,
  slide: &SlidePersist,
  record: &GraphicDataRecord,
) -> Option<shared_diagram::DiagramStyleColors> {
  let color_resource = record.diagram_color_resource.as_ref()?;
  let mut fill_by_label = HashMap::new();
  let mut text_fill_by_label = HashMap::new();
  for label in &color_resource.colors.color_transform_style_label {
    if let Some(fill_list) = label.fill_color_list.as_ref() {
      let fills: Vec<LayoutRgbColor> = fill_list
        .fill_color_list_choice
        .iter()
        .filter_map(Color::from_diagram_fill_color_choice)
        .filter_map(|color| import.resolve_color_for_slide(slide, &color, None))
        .map(|color| LayoutRgbColor {
          r: color.r,
          g: color.g,
          b: color.b,
        })
        .collect();
      if !fills.is_empty() {
        fill_by_label.insert(label.name.clone(), fills);
      }
    }
    if let Some(text_fill_list) = label.text_fill_color_list.as_ref() {
      let fills: Vec<LayoutRgbColor> = text_fill_list
        .text_fill_color_list_choice
        .iter()
        .filter_map(Color::from_diagram_text_fill_color_choice)
        .filter_map(|color| import.resolve_color_for_slide(slide, &color, None))
        .map(|color| LayoutRgbColor {
          r: color.r,
          g: color.g,
          b: color.b,
        })
        .collect();
      if !fills.is_empty() {
        text_fill_by_label.insert(label.name.clone(), fills);
      }
    }
  }
  (!fill_by_label.is_empty() || !text_fill_by_label.is_empty()).then_some(
    shared_diagram::DiagramStyleColors {
      fill_by_label,
      text_fill_by_label,
    },
  )
}

fn diagram_font_style_reference(
  reference: &a::FontReference,
  text_fill: Option<RgbColor>,
) -> FontStyleReference {
  FontStyleReference {
    index: reference.index,
    placeholder_color: text_fill
      .map(|color| {
        Color::RgbHex(super::drawingml::color::RgbHexColor {
          value: format!("{:02X}{:02X}{:02X}", color.r, color.g, color.b),
          transformations: Vec::new(),
        })
      })
      .or_else(|| {
        reference
          .font_reference_choice
          .as_ref()
          .and_then(Color::from_font_reference_choice)
      }),
  }
}

fn layout_rgb_color(color: RgbColor) -> LayoutRgbColor {
  LayoutRgbColor {
    r: color.r,
    g: color.g,
    b: color.b,
  }
}

fn pdf_rgb_color(color: LayoutRgbColor) -> RgbColor {
  RgbColor {
    r: color.r,
    g: color.g,
    b: color.b,
  }
}

fn diagram_accent_fill(import: &PowerPointImport, slide: &SlidePersist) -> RgbColor {
  chart_theme(import, slide)
    .and_then(|theme| {
      theme
        .color_scheme
        .get_color(a::ColorSchemeIndexValues::Accent1)
    })
    .and_then(|color| import.resolve_color_for_slide(slide, color, None))
    .map(|color| RgbColor {
      r: color.r,
      g: color.g,
      b: color.b,
    })
    .unwrap_or(RgbColor {
      r: 0x4f,
      g: 0x81,
      b: 0xbd,
    })
}

fn lower_picture(
  import: &PowerPointImport,
  slide: &SlidePersist,
  shape: &Shape,
  offset: DisplayOffset,
  items: &mut Vec<PageItem>,
) {
  let Some(picture) = &shape.picture else {
    return;
  };
  if shape.size.cx <= 0 || shape.size.cy <= 0 {
    return;
  }
  let _embed_relationship_id = picture.embed_relationship_id.as_deref();
  let _link_relationship_id = picture.link_relationship_id.as_deref();
  let Some(resource) = picture.image_resource.as_ref() else {
    return;
  };
  let image_data = image_data_with_blip_effects(
    import,
    slide,
    &resource.data,
    resource.content_type.as_deref(),
    &picture.blip_choices,
  );
  let custom_geometry = shape.custom_shape_properties.geometry.is_some();
  let (data, content_type, crop, flip_horizontal, flip_vertical) =
    if custom_geometry && (picture.crop != ImageCrop::default() || shape.flip_h || shape.flip_v) {
      transform_image_data_to_png(&image_data.data, picture.crop, shape.flip_h, shape.flip_v)
        .map(|data| {
          (
            data.into(),
            Some("image/png".into()),
            ImageCrop::default(),
            false,
            false,
          )
        })
        .unwrap_or_else(|| {
          (
            image_data.data,
            image_data
              .content_type
              .or_else(|| resource.content_type.clone()),
            picture.crop,
            shape.flip_h,
            shape.flip_v,
          )
        })
    } else {
      (
        image_data.data,
        image_data
          .content_type
          .or_else(|| resource.content_type.clone()),
        picture.crop,
        shape.flip_h,
        shape.flip_v,
      )
    };
  items.push(PageItem::Image(ImageItem {
    x_pt: units::emu_to_points(offset.x_emu + shape.position.x),
    y_pt: units::emu_to_points(offset.y_emu + shape.position.y),
    width_pt: units::emu_to_points(shape.size.cx),
    height_pt: units::emu_to_points(shape.size.cy),
    crop,
    clip_path: Vec::new(),
    rotation_deg: shape.rotation,
    flip_horizontal,
    flip_vertical,
    data,
    content_type,
    alt_text: shape
      .description
      .clone()
      .or_else(|| shape.title.clone())
      .or_else(|| shape.name.clone()),
    hyperlink_url: shape.hyperlink_url.clone(),
    floating: false,
    behind_text: false,
  }));
}

fn lower_shape_hyperlink(shape: &Shape, offset: DisplayOffset, items: &mut Vec<PageItem>) {
  let Some(hyperlink_url) = &shape.hyperlink_url else {
    return;
  };
  if shape.service_name == ShapeService::Group || shape.size.cx <= 0 || shape.size.cy <= 0 {
    return;
  }
  items.push(PageItem::LinkArea(LinkAreaItem {
    x_pt: units::emu_to_points(offset.x_emu + shape.position.x),
    y_pt: units::emu_to_points(offset.y_emu + shape.position.y),
    width_pt: units::emu_to_points(shape.size.cx),
    height_pt: units::emu_to_points(shape.size.cy),
    hyperlink_url: hyperlink_url.clone(),
  }));
}

fn lower_table(
  import: &PowerPointImport,
  shape: &Shape,
  offset: DisplayOffset,
  table: &TableProperties,
  items: &mut Vec<PageItem>,
) {
  // table grid and row heights as the visible TableShape size.
  let x0 = units::emu_to_points(offset.x_emu + shape.position.x);
  let y0 = units::emu_to_points(offset.y_emu + shape.position.y);
  let table_width = units::emu_to_points(table.grid.iter().copied().sum::<i64>());
  let row_height_sum = table.rows.iter().map(|row| row.height).sum::<i64>();
  let table_height = units::emu_to_points(row_height_sum.max(shape.size.cy));
  if table_width <= 0.0 || table_height <= 0.0 {
    return;
  }

  let package_table_style = table
    .inline_style
    .as_ref()
    .or_else(|| import.get_table_style(table.style_id.as_deref()));
  let predefined_table_style = if package_table_style.is_none() {
    predefined_table_style(table.style_id.as_deref())
  } else {
    None
  };
  let table_style = package_table_style.or(predefined_table_style.as_ref());
  let table_background = table_style.and_then(|style| {
    table_style_part_fill(import, &style.table_background)
      .and_then(|fill| fill_paint(import, &fill))
  });
  let border_color = RgbColor { r: 0, g: 0, b: 0 };
  let draw_fallback_grid = table_style.is_none() && !table_has_visible_direct_borders(table);
  if draw_fallback_grid {
    push_table_line(items, x0, y0, x0 + table_width, y0, border_color);
    push_table_line(items, x0, y0, x0, y0 + table_height, border_color);
  } else if let Some(style) = table_style
    && !table_has_visible_direct_borders(table)
  {
    lower_table_style_outer_borders(import, style, x0, y0, table_width, table_height, items);
  }

  let mut y = y0;
  let max_row = table.rows.len().saturating_sub(1);
  let max_column = table.grid.len().saturating_sub(1);
  for (row_index, row) in table.rows.iter().enumerate() {
    let row_height = table_row_display_height(row.height, row_height_sum, shape.size.cy);
    let mut x = x0;
    let mut grid_index = 0usize;
    for cell in &row.cells {
      let span = table_cell_grid_span(cell);
      let width_emu = if grid_index < table.grid.len() {
        table.grid[grid_index..table.grid.len().min(grid_index + span)]
          .iter()
          .copied()
          .sum::<i64>()
      } else {
        0
      };
      let cell_width = units::emu_to_points(width_emu);
      if !cell.horizontal_merge && !cell.vertical_merge {
        let style_part = table_style.map(|style| {
          table_cell_style_part(
            import, table, style, grid_index, max_column, row_index, max_row,
          )
        });
        lower_table_cell(
          import,
          cell,
          style_part.as_ref(),
          table_background,
          TextFrame {
            x_pt: x,
            y_pt: y,
            width_pt: cell_width,
            height_pt: row_height,
          },
          items,
        );
      }
      x += cell_width;
      grid_index = grid_index.saturating_add(span);
    }
    y += row_height;
    if draw_fallback_grid {
      push_table_line(items, x0, y, x0 + table_width, y, border_color);
    }
  }

  if draw_fallback_grid {
    let mut x = x0;
    for width in &table.grid {
      x += units::emu_to_points(*width);
      push_table_line(items, x, y0, x, y0 + table_height, border_color);
    }
  }
}

fn lower_table_style_outer_borders(
  import: &PowerPointImport,
  style: &TableStyle,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  items: &mut Vec<PageItem>,
) {
  let borders = &style.whole_table.borders;
  push_table_border_line(
    import,
    &table_style_border_line(import, &borders.top, &borders.top_reference),
    x,
    y,
    x + width,
    y,
    items,
  );
  push_table_border_line(
    import,
    &table_style_border_line(import, &borders.bottom, &borders.bottom_reference),
    x,
    y + height,
    x + width,
    y + height,
    items,
  );
  push_table_border_line(
    import,
    &table_style_border_line(import, &borders.left, &borders.left_reference),
    x,
    y,
    x,
    y + height,
    items,
  );
  push_table_border_line(
    import,
    &table_style_border_line(import, &borders.right, &borders.right_reference),
    x + width,
    y,
    x + width,
    y + height,
    items,
  );
}

fn table_row_display_height(row_height: i64, row_height_sum: i64, shape_height: i64) -> f32 {
  let row_height = units::emu_to_points(row_height);
  if row_height_sum <= 0 || shape_height <= row_height_sum {
    return row_height;
  }
  row_height * shape_height as f32 / row_height_sum as f32
}

fn table_cell_style_part(
  import: &PowerPointImport,
  table: &TableProperties,
  style: &TableStyle,
  column: usize,
  max_column: usize,
  row: usize,
  max_row: usize,
) -> TableStylePart {
  // order: whole table, first/last row/column, horizontal banding, corners,
  // then vertical banding. Direct tcPr is merged afterwards by the caller.
  let mut result = TableStylePart::default();
  let cell_position = TableStyleCellPosition {
    column,
    max_column,
    row,
    max_row,
    whole_table: false,
  };
  merge_style_part(
    import,
    &mut result,
    &style.whole_table,
    TableStyleCellPosition {
      whole_table: true,
      ..cell_position
    },
  );
  if table.first_row && row == 0 {
    merge_style_part(import, &mut result, &style.first_row, cell_position);
  }
  if table.last_row && row == max_row {
    merge_style_part(import, &mut result, &style.last_row, cell_position);
  }
  if table.first_column && column == 0 {
    merge_style_part(import, &mut result, &style.first_column, cell_position);
  }
  if table.last_column && column == max_column {
    merge_style_part(import, &mut result, &style.last_column, cell_position);
  }
  if table.band_row
    && (!table.first_row || row != 0)
    && (!table.last_row || row != max_row)
    && (!table.first_column || column != 0 || !table_style_part_has_fill(&style.first_column))
    && (!table.last_column
      || column != max_column
      || !table_style_part_has_fill(&style.last_column))
  {
    let band = row + usize::from(table.first_row);
    let part = if band & 1 == 1 {
      &style.band2_horizontal
    } else {
      &style.band1_horizontal
    };
    merge_style_part(import, &mut result, part, cell_position);
  }
  if row == 0 && column == 0 {
    merge_style_part(import, &mut result, &style.northwest_cell, cell_position);
  }
  if row == max_row && column == 0 {
    merge_style_part(import, &mut result, &style.southwest_cell, cell_position);
  }
  if row == 0 && column == max_column {
    merge_style_part(import, &mut result, &style.northeast_cell, cell_position);
  }
  if row == max_row && column == max_column {
    merge_style_part(import, &mut result, &style.southeast_cell, cell_position);
  }
  if table.band_column
    && (!table.first_row || row != 0)
    && (!table.last_row || row != max_row)
    && (!table.first_column || column != 0)
    && (!table.last_column || column != max_column)
  {
    let band = column + usize::from(table.first_column);
    let part = if band & 1 == 1 {
      &style.band2_vertical
    } else {
      &style.band1_vertical
    };
    merge_style_part(import, &mut result, part, cell_position);
  }
  result
}

#[derive(Clone, Copy)]
struct TableStyleCellPosition {
  column: usize,
  max_column: usize,
  row: usize,
  max_row: usize,
  whole_table: bool,
}

fn merge_style_part(
  import: &PowerPointImport,
  target: &mut TableStylePart,
  source: &TableStylePart,
  cell_position: TableStyleCellPosition,
) {
  if let Some(fill) = table_style_part_fill(import, source) {
    target.fill_properties = Some(fill);
  }
  let mut borders = TableCellBorders::default();
  merge_style_borders(import, &mut borders, &source.borders, cell_position);
  merge_cell_borders_from_style(&mut target.borders, &borders);
  target.text.merge_from(&source.text);
}

fn table_style_part_has_fill(part: &TableStylePart) -> bool {
  part.fill_properties.is_some() || part.fill_reference.is_some()
}

fn table_style_part_fill(
  import: &PowerPointImport,
  part: &TableStylePart,
) -> Option<FillProperties> {
  part.fill_properties.clone().or_else(|| {
    part.fill_reference.as_ref().and_then(|reference| {
      import
        .get_theme_fill_style(reference.index)
        .map(|fill| fill.with_placeholder_color(reference.placeholder_color.clone()))
    })
  })
}

fn merge_style_borders(
  import: &PowerPointImport,
  target: &mut TableCellBorders,
  source: &TableStyleBorders,
  cell_position: TableStyleCellPosition,
) {
  if (!cell_position.whole_table || cell_position.column == 0)
    && let Some(line) = table_style_border_line(import, &source.left, &source.left_reference)
  {
    target.left = Some(line);
  }
  if (!cell_position.whole_table || cell_position.column >= cell_position.max_column)
    && let Some(line) = table_style_border_line(import, &source.right, &source.right_reference)
  {
    target.right = Some(line);
  }
  if (!cell_position.whole_table || cell_position.row == 0)
    && let Some(line) = table_style_border_line(import, &source.top, &source.top_reference)
  {
    target.top = Some(line);
  }
  if (!cell_position.whole_table || cell_position.row >= cell_position.max_row)
    && let Some(line) = table_style_border_line(import, &source.bottom, &source.bottom_reference)
  {
    target.bottom = Some(line);
  }
  if let Some(line) = table_style_border_line(
    import,
    &source.inside_horizontal,
    &source.inside_horizontal_reference,
  ) {
    if cell_position.row != 0 {
      target.top = Some(line.clone());
    }
    if cell_position.row != cell_position.max_row {
      target.bottom = Some(line);
    }
  }
  if let Some(line) = table_style_border_line(
    import,
    &source.inside_vertical,
    &source.inside_vertical_reference,
  ) {
    if cell_position.column != 0 {
      target.left = Some(line.clone());
    }
    if cell_position.column != cell_position.max_column {
      target.right = Some(line);
    }
  }
  if let Some(line) = table_style_border_line(
    import,
    &source.top_left_to_bottom_right,
    &source.top_left_to_bottom_right_reference,
  ) {
    target.top_left_to_bottom_right = Some(line);
  }
  if let Some(line) = table_style_border_line(
    import,
    &source.bottom_left_to_top_right,
    &source.bottom_left_to_top_right_reference,
  ) {
    target.bottom_left_to_top_right = Some(line);
  }
}

fn table_style_border_line(
  import: &PowerPointImport,
  direct: &Option<LineProperties>,
  reference: &Option<super::drawingml::shape::ShapeStyleReference>,
) -> Option<LineProperties> {
  direct.clone().or_else(|| {
    reference.as_ref().and_then(|reference| {
      import
        .get_theme_line_style(reference.index)
        .map(|line| line.with_placeholder_color(reference.placeholder_color.clone()))
    })
  })
}

fn merge_cell_borders_from_style(target: &mut TableStyleBorders, source: &TableCellBorders) {
  if source.left.is_some() {
    target.left = source.left.clone();
  }
  if source.right.is_some() {
    target.right = source.right.clone();
  }
  if source.top.is_some() {
    target.top = source.top.clone();
  }
  if source.bottom.is_some() {
    target.bottom = source.bottom.clone();
  }
  if source.top_left_to_bottom_right.is_some() {
    target.top_left_to_bottom_right = source.top_left_to_bottom_right.clone();
  }
  if source.bottom_left_to_top_right.is_some() {
    target.bottom_left_to_top_right = source.bottom_left_to_top_right.clone();
  }
}

fn table_has_visible_direct_borders(table: &TableProperties) -> bool {
  table.rows.iter().any(|row| {
    row.cells.iter().any(|cell| {
      table_border_line_is_visible(&cell.borders.left)
        || table_border_line_is_visible(&cell.borders.right)
        || table_border_line_is_visible(&cell.borders.top)
        || table_border_line_is_visible(&cell.borders.bottom)
        || table_border_line_is_visible(&cell.borders.top_left_to_bottom_right)
        || table_border_line_is_visible(&cell.borders.bottom_left_to_top_right)
    })
  })
}

fn table_border_line_is_visible(line: &Option<LineProperties>) -> bool {
  matches!(
    line.as_ref().map(|line| &line.fill),
    Some(LineFill::Solid(_) | LineFill::Gradient(_) | LineFill::Pattern(_))
  )
}

fn lower_table_cell(
  import: &PowerPointImport,
  cell: &TableCell,
  style_part: Option<&TableStylePart>,
  table_background: Option<DisplayPaint>,
  frame: TextFrame,
  items: &mut Vec<PageItem>,
) {
  if frame.width_pt <= 0.0 || frame.height_pt <= 0.0 {
    return;
  }
  let fill = table_cell_fill_paint(import, cell, style_part, table_background);
  if let Some(fill) = fill {
    items.push(PageItem::Rect(RectItem {
      x_pt: frame.x_pt,
      y_pt: frame.y_pt,
      width_pt: frame.width_pt,
      height_pt: frame.height_pt,
      fill_color: Some(fill.color),
      fill_opacity: fill.opacity,
      stroke: None,
      stroke_opacity: 1.0,
    }));
  }
  let borders = table_cell_effective_borders(cell, style_part);
  lower_table_cell_borders(
    import,
    &borders,
    frame.x_pt,
    frame.y_pt,
    frame.width_pt,
    frame.height_pt,
    items,
  );

  if let Some(text_body) = &cell.text_body {
    let mut text_body = text_body.clone();
    text_body.display_properties.vertical = cell.vertical;
    text_body.display_properties.anchor = cell.anchor;
    text_body.display_properties.anchor_center = cell.anchor_center;
    text_body.display_properties.horizontal_overflow = Some(cell.horizontal_overflow);
    let x = frame.x_pt + units::emu_to_points(i64::from(cell.margins.left));
    let y = frame.y_pt + units::emu_to_points(i64::from(cell.margins.top));
    lower_text_body_at_with_table_style(
      import,
      TextFrame {
        x_pt: x,
        y_pt: y,
        width_pt: (frame.width_pt
          - units::emu_to_points(i64::from(cell.margins.left + cell.margins.right)))
        .max(0.0),
        height_pt: (frame.height_pt
          - units::emu_to_points(i64::from(cell.margins.top + cell.margins.bottom)))
        .max(0.0),
      },
      &text_body,
      style_part.map(|style| &style.text),
      items,
    );
  }
}

fn table_cell_fill_paint(
  import: &PowerPointImport,
  cell: &TableCell,
  style_part: Option<&TableStylePart>,
  table_background: Option<DisplayPaint>,
) -> Option<DisplayPaint> {
  let cell_fill = cell
    .fill_properties
    .as_ref()
    .map(|fill| fill_paint(import, fill))
    .unwrap_or_else(|| {
      style_part
        .and_then(|style| style.fill_properties.as_ref())
        .and_then(|fill| fill_paint(import, fill))
    });
  match (table_background, cell_fill) {
    (Some(background), Some(cell)) => Some(blend_table_cell_fill(background, cell)),
    (Some(background), None) => Some(background),
    (None, Some(cell)) => Some(cell),
    (None, None) => None,
  }
}

fn blend_table_cell_fill(background: DisplayPaint, cell: DisplayPaint) -> DisplayPaint {
  // through basegfx::interpolate(bg, cell, 1 - cellTransparency).
  let cell_weight = cell.opacity.clamp(0.0, 1.0);
  let background_weight = 1.0 - cell_weight;
  DisplayPaint {
    color: RgbColor {
      r: blend_channel(
        background.color.r,
        cell.color.r,
        background_weight,
        cell_weight,
      ),
      g: blend_channel(
        background.color.g,
        cell.color.g,
        background_weight,
        cell_weight,
      ),
      b: blend_channel(
        background.color.b,
        cell.color.b,
        background_weight,
        cell_weight,
      ),
    },
    opacity: background.opacity.max(cell.opacity).clamp(0.0, 1.0),
  }
}

fn blend_channel(background: u8, cell: u8, background_weight: f32, cell_weight: f32) -> u8 {
  (f32::from(background) * background_weight + f32::from(cell) * cell_weight)
    .round()
    .clamp(0.0, 255.0) as u8
}

fn lower_table_cell_borders(
  import: &PowerPointImport,
  borders: &TableCellBorders,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  items: &mut Vec<PageItem>,
) {
  push_table_border_line(import, &borders.top, x, y, x + width, y, items);
  push_table_border_line(
    import,
    &borders.bottom,
    x,
    y + height,
    x + width,
    y + height,
    items,
  );
  push_table_border_line(import, &borders.left, x, y, x, y + height, items);
  push_table_border_line(
    import,
    &borders.right,
    x + width,
    y,
    x + width,
    y + height,
    items,
  );
  push_table_border_line(
    import,
    &borders.top_left_to_bottom_right,
    x,
    y,
    x + width,
    y + height,
    items,
  );
  push_table_border_line(
    import,
    &borders.bottom_left_to_top_right,
    x,
    y + height,
    x + width,
    y,
    items,
  );
}

fn table_cell_effective_borders(
  cell: &TableCell,
  style_part: Option<&TableStylePart>,
) -> TableCellBorders {
  let mut borders = TableCellBorders::default();
  if let Some(style_part) = style_part {
    borders.left = style_part.borders.left.clone();
    borders.right = style_part.borders.right.clone();
    borders.top = style_part.borders.top.clone();
    borders.bottom = style_part.borders.bottom.clone();
    borders.top_left_to_bottom_right = style_part.borders.top_left_to_bottom_right.clone();
    borders.bottom_left_to_top_right = style_part.borders.bottom_left_to_top_right.clone();
  }
  if cell.borders.left.is_some() {
    borders.left = cell.borders.left.clone();
  }
  if cell.borders.right.is_some() {
    borders.right = cell.borders.right.clone();
  }
  if cell.borders.top.is_some() {
    borders.top = cell.borders.top.clone();
  }
  if cell.borders.bottom.is_some() {
    borders.bottom = cell.borders.bottom.clone();
  }
  if cell.borders.top_left_to_bottom_right.is_some() {
    borders.top_left_to_bottom_right = cell.borders.top_left_to_bottom_right.clone();
  }
  if cell.borders.bottom_left_to_top_right.is_some() {
    borders.bottom_left_to_top_right = cell.borders.bottom_left_to_top_right.clone();
  }
  borders
}

fn push_table_border_line(
  import: &PowerPointImport,
  line: &Option<LineProperties>,
  x1_pt: f32,
  y1_pt: f32,
  x2_pt: f32,
  y2_pt: f32,
  items: &mut Vec<PageItem>,
) {
  let Some(stroke) = line
    .as_ref()
    .and_then(|line| line_stroke(import, None, line))
  else {
    return;
  };
  items.push(PageItem::Line(LineItem {
    x1_pt,
    y1_pt,
    x2_pt,
    y2_pt,
    width_pt: stroke.style.width_pt,
    color: stroke.style.color,
    kind: LineItemKind::Stroke,
  }));
}

fn table_cell_grid_span(cell: &TableCell) -> usize {
  cell
    .grid_span
    .and_then(|span| usize::try_from(span).ok())
    .filter(|span| *span > 0)
    .unwrap_or(1)
}

fn push_table_line(
  items: &mut Vec<PageItem>,
  x1_pt: f32,
  y1_pt: f32,
  x2_pt: f32,
  y2_pt: f32,
  color: RgbColor,
) {
  items.push(PageItem::Line(LineItem {
    x1_pt,
    y1_pt,
    x2_pt,
    y2_pt,
    width_pt: DEFAULT_TABLE_BORDER_PT,
    color,
    kind: LineItemKind::Stroke,
  }));
}

fn lower_shape_bounds(
  import: &PowerPointImport,
  slide: &SlidePersist,
  shape: &Shape,
  offset: DisplayOffset,
  items: &mut Vec<PageItem>,
) {
  if shape.service_name == ShapeService::Group || shape.size.cx <= 0 || shape.size.cy <= 0 {
    return;
  }

  let fill_paint = shape
    .actual_fill_properties
    .as_ref()
    .and_then(|fill| shape_fill_paint(import, slide, fill));
  let x_pt = units::emu_to_points(offset.x_emu + shape.position.x);
  let y_pt = units::emu_to_points(offset.y_emu + shape.position.y);
  let width_pt = units::emu_to_points(shape.size.cx);
  let height_pt = units::emu_to_points(shape.size.cy);
  let frame = TextFrame {
    x_pt,
    y_pt,
    width_pt,
    height_pt,
  };
  let shape_path = shape_path_commands(shape, frame);
  let fill_images = shape
    .actual_fill_properties
    .as_ref()
    .map(|fill| {
      blip_fill_image_items(
        import,
        slide,
        fill,
        ImageFillPlacement {
          frame,
          rotation_deg: shape.rotation,
          flip_horizontal: shape.flip_h,
          flip_vertical: shape.flip_v,
          crop_bitmap: shape.custom_shape_properties.geometry.is_some(),
          clip_path: shape_path.clone(),
          alt_text: shape
            .description
            .clone()
            .or_else(|| shape.title.clone())
            .or_else(|| shape.name.clone()),
          hyperlink_url: shape.hyperlink_url.clone(),
        },
      )
    })
    .unwrap_or_default();
  let line = shape
    .actual_line_properties
    .as_ref()
    .and_then(|line| line_stroke(import, Some(slide), line));
  let gradient_path = shape
    .actual_fill_properties
    .as_ref()
    .and_then(|fill| shape_gradient_path(import, slide, shape, fill, frame, line));
  let has_fill_image = !fill_images.is_empty();
  if fill_paint.is_none() && !has_fill_image && line.is_none() && gradient_path.is_none() {
    return;
  }
  let (stroke, stroke_opacity) = line
    .map(|line| (Some(line.style), line.opacity))
    .unwrap_or((None, 1.0));

  if let Some(shadow) = shape
    .actual_effect_properties
    .as_ref()
    .and_then(|effects| effects.shadow.as_ref())
    && let Some(paint) = shadow
      .color
      .as_ref()
      .and_then(|color| display_paint_for_slide(import, slide, color, None))
    && let Some(image) = outer_shadow_image_item(
      shadow,
      ShadowFrame {
        x_pt,
        y_pt,
        width_pt,
        height_pt,
        stroke_width_pt: stroke.map_or(0.0, |stroke| stroke.width_pt),
      },
      &shape_path,
      paint.color,
      paint.opacity,
    )
  {
    items.push(PageItem::Image(image));
  }

  items.extend(fill_images.into_iter().map(PageItem::Image));
  if let Some(path) = gradient_path {
    items.push(PageItem::Path(path));
    return;
  }
  if fill_paint.is_none() && stroke.is_none() {
    return;
  }

  items.push(PageItem::Rect(RectItem {
    x_pt,
    y_pt,
    width_pt,
    height_pt,
    fill_color: fill_paint.map(|fill| fill.color),
    fill_opacity: fill_paint.map(|fill| fill.opacity).unwrap_or(1.0),
    stroke,
    stroke_opacity,
  }));
}

fn shape_gradient_path(
  import: &PowerPointImport,
  slide: &SlidePersist,
  shape: &Shape,
  fill: &FillProperties,
  frame: TextFrame,
  line: Option<DisplayStroke>,
) -> Option<common::PathItem<'static>> {
  let resolved_background;
  let (effective_fill, definition_bounds) = if matches!(fill.kind, FillKind::SlideBackground) {
    resolved_background = resolved_slide_background_fill(import, slide)?;
    (
      &resolved_background,
      common_rect(0.0, 0.0, slide.size.width_pt, slide.size.height_pt),
    )
  } else {
    (
      fill,
      common_rect(frame.x_pt, frame.y_pt, frame.width_pt, frame.height_pt),
    )
  };
  let FillKind::Gradient(gradient) = &effective_fill.kind else {
    return None;
  };
  let linear = match gradient.gradient_fill_choice.as_ref()? {
    a::GradientFillChoice::LinearGradientFill(linear) => linear,
    a::GradientFillChoice::PathGradientFill(_) => return None,
  };
  let stops = gradient
    .gradient_stop_list
    .as_ref()?
    .gradient_stop
    .iter()
    .filter_map(|stop| {
      let color = stop
        .gradient_stop_choice
        .as_ref()
        .and_then(Color::from_gradient_stop_choice)?;
      let paint = display_paint_for_slide(
        import,
        slide,
        &color,
        effective_fill.placeholder_color.as_ref(),
      )?;
      Some(common::GradientStop {
        position: stop.position.as_ratio() as f32,
        color: common_rgb(paint.color, paint.opacity),
        scheme: None,
      })
    })
    .collect::<Vec<_>>();
  let mut stops = stops;
  super::gradient::normalize_powerpoint_gradient_stops(&mut stops);
  if stops.is_empty() {
    return None;
  }
  let rotate_with_shape = gradient
    .rotate_with_shape
    .as_ref()
    .is_none_or(|value| value.as_bool());
  let scaled = linear.scaled.as_ref().is_some_and(|value| value.as_bool());
  let local_angle_degrees = linear.angle.unwrap_or_default() as f32 / 60_000.0;
  let has_shape_transform = shape.rotation.abs() > f32::EPSILON || shape.flip_h || shape.flip_v;
  let follows_shape_transform =
    rotate_with_shape && has_shape_transform && !matches!(fill.kind, FillKind::SlideBackground);
  let gradient_line = follows_shape_transform.then(|| {
    let (start, end) = linear_gradient_line(definition_bounds, local_angle_degrees, scaled);
    (
      transform_shape_point(start, frame, shape),
      transform_shape_point(end, frame, shape),
    )
  });
  let angle_degrees = if follows_shape_transform {
    transformed_gradient_angle(local_angle_degrees, shape)
  } else {
    local_angle_degrees
  };
  Some(common::PathItem {
    bounds: transformed_shape_bounds(frame, shape),
    points: Vec::new(),
    commands: shape_path_commands(shape, frame),
    closed: true,
    fill: common::Fill::Gradient(common::GradientFill {
      stops,
      angle_degrees: Some(angle_degrees),
      definition_bounds: Some(definition_bounds),
      line: gradient_line,
      interpolation: if follows_shape_transform {
        common::GradientInterpolation::PowerPointGammaSigma
      } else {
        common::GradientInterpolation::LinearSrgb
      },
      scaled,
      path: None,
    }),
    stroke: line.map(|line| common::Stroke {
      width: common::Pt(line.style.width_pt),
      color: common_rgb(line.style.color, line.opacity),
      dash: None,
      source_style_id: None,
    }),
  })
}

fn shape_path_commands(shape: &Shape, frame: TextFrame) -> Vec<common::PathCommand> {
  let commands = match shape.custom_shape_properties.geometry.as_ref() {
    Some(CustomShapeGeometry::Custom(geometry)) => custom_geometry::path_commands(
      geometry,
      frame.x_pt,
      frame.y_pt,
      frame.width_pt,
      frame.height_pt,
    ),
    Some(CustomShapeGeometry::Preset(preset)) => Some(preset_geometry::path_commands(
      Some(preset),
      frame.x_pt,
      frame.y_pt,
      frame.width_pt,
      frame.height_pt,
    )),
    None => None,
  };
  let commands = commands.unwrap_or_else(|| {
    preset_geometry::path_commands(
      None,
      frame.x_pt,
      frame.y_pt,
      frame.width_pt,
      frame.height_pt,
    )
  });
  commands
    .into_iter()
    .map(|command| transform_shape_path_command(command, frame, shape))
    .collect()
}

fn transform_shape_path_command(
  command: common::PathCommand,
  frame: TextFrame,
  shape: &Shape,
) -> common::PathCommand {
  match command {
    common::PathCommand::MoveTo(point) => {
      common::PathCommand::MoveTo(transform_shape_point(point, frame, shape))
    }
    common::PathCommand::LineTo(point) => {
      common::PathCommand::LineTo(transform_shape_point(point, frame, shape))
    }
    common::PathCommand::CubicTo {
      control1,
      control2,
      end,
    } => common::PathCommand::CubicTo {
      control1: transform_shape_point(control1, frame, shape),
      control2: transform_shape_point(control2, frame, shape),
      end: transform_shape_point(end, frame, shape),
    },
    common::PathCommand::Close => common::PathCommand::Close,
  }
}

fn transform_shape_point(point: common::Point, frame: TextFrame, shape: &Shape) -> common::Point {
  let center_x = frame.x_pt + frame.width_pt / 2.0;
  let center_y = frame.y_pt + frame.height_pt / 2.0;
  let x = if shape.flip_h {
    2.0 * center_x - point.x.0
  } else {
    point.x.0
  };
  let y = if shape.flip_v {
    2.0 * center_y - point.y.0
  } else {
    point.y.0
  };
  let (x, y) = rotate_point(x, y, center_x, center_y, shape.rotation.to_radians());
  common_point(x, y)
}

fn transformed_shape_bounds(frame: TextFrame, shape: &Shape) -> common::Rect {
  let corners = [
    common_point(frame.x_pt, frame.y_pt),
    common_point(frame.x_pt + frame.width_pt, frame.y_pt),
    common_point(frame.x_pt + frame.width_pt, frame.y_pt + frame.height_pt),
    common_point(frame.x_pt, frame.y_pt + frame.height_pt),
  ]
  .map(|point| transform_shape_point(point, frame, shape));
  let left = corners
    .iter()
    .map(|point| point.x.0)
    .fold(f32::INFINITY, f32::min);
  let top = corners
    .iter()
    .map(|point| point.y.0)
    .fold(f32::INFINITY, f32::min);
  let right = corners
    .iter()
    .map(|point| point.x.0)
    .fold(f32::NEG_INFINITY, f32::max);
  let bottom = corners
    .iter()
    .map(|point| point.y.0)
    .fold(f32::NEG_INFINITY, f32::max);
  common_rect(left, top, right - left, bottom - top)
}

fn linear_gradient_line(
  bounds: common::Rect,
  angle_degrees: f32,
  scaled: bool,
) -> (common::Point, common::Point) {
  let angle = angle_degrees.to_radians();
  let mut direction_x = angle.cos();
  let mut direction_y = angle.sin();
  if scaled {
    direction_x *= bounds.size.width.0;
    direction_y *= bounds.size.height.0;
  }
  let length = direction_x.hypot(direction_y).max(f32::EPSILON);
  direction_x /= length;
  direction_y /= length;
  let half_span =
    (direction_x.abs() * bounds.size.width.0 + direction_y.abs() * bounds.size.height.0) / 2.0;
  let center_x = bounds.origin.x.0 + bounds.size.width.0 / 2.0;
  let center_y = bounds.origin.y.0 + bounds.size.height.0 / 2.0;
  (
    common_point(
      center_x - direction_x * half_span,
      center_y - direction_y * half_span,
    ),
    common_point(
      center_x + direction_x * half_span,
      center_y + direction_y * half_span,
    ),
  )
}

fn transformed_gradient_angle(local_angle_degrees: f32, shape: &Shape) -> f32 {
  let mut angle = local_angle_degrees;
  if shape.flip_h {
    angle = 180.0 - angle;
  }
  if shape.flip_v {
    angle = -angle;
  }
  angle + shape.rotation
}

fn child_display_offset(shape: &Shape, offset: DisplayOffset) -> DisplayOffset {
  DisplayOffset {
    x_emu: offset.x_emu + shape.position.x - shape.child_position.x,
    y_emu: offset.y_emu + shape.position.y - shape.child_position.y,
  }
}

#[derive(Clone, Debug)]
struct ImageFillPlacement {
  frame: TextFrame,
  rotation_deg: f32,
  flip_horizontal: bool,
  flip_vertical: bool,
  crop_bitmap: bool,
  clip_path: Vec<common::PathCommand>,
  alt_text: Option<String>,
  hyperlink_url: Option<String>,
}

fn blip_fill_image_items(
  import: &PowerPointImport,
  slide: &SlidePersist,
  fill: &FillProperties,
  placement: ImageFillPlacement,
) -> Vec<ImageItem> {
  let FillKind::Blip(blip_fill) = &fill.kind else {
    return Vec::new();
  };
  let Some(blip) = blip_fill.blip.as_ref() else {
    return Vec::new();
  };
  let Some(relationship_id) = blip.embed.as_deref() else {
    return Vec::new();
  };
  let Some(resource) = slide.image_resources.get(relationship_id) else {
    return Vec::new();
  };
  blip_fill_image_items_from_resource(import, slide, blip_fill, blip, resource, placement)
}

fn blip_fill_image_items_from_resource(
  import: &PowerPointImport,
  slide: &SlidePersist,
  blip_fill: &a::BlipFill,
  blip: &a::Blip,
  resource: &ImageResource,
  placement: ImageFillPlacement,
) -> Vec<ImageItem> {
  let image_data = image_data_with_blip_effects(
    import,
    slide,
    &resource.data,
    resource.content_type.as_deref(),
    &blip.blip_choice,
  );
  let content_type = image_data
    .content_type
    .clone()
    .or_else(|| resource.content_type.clone());

  let crop = blip_fill_image_crop(blip_fill);
  if let Some(a::BlipFillChoice::Tile(tile)) = blip_fill.blip_fill_choice.as_ref() {
    return tiled_blip_fill_image_items(image_data.data, content_type, tile, placement);
  }
  // lclGetBitmapMode() defaults missing bitmap mode to XML_tile for MSO.
  if blip_fill.blip_fill_choice.is_none() {
    return tiled_blip_fill_image_items(
      image_data.data,
      content_type,
      &a::Tile::default(),
      placement,
    );
  }

  let (data, content_type, crop, flip_horizontal, flip_vertical) = if placement.crop_bitmap
    && ((blip_fill.source_rectangle.is_some() && crop != ImageCrop::default())
      || placement.flip_horizontal
      || placement.flip_vertical)
  {
    transform_image_data_to_png(
      &image_data.data,
      crop,
      placement.flip_horizontal,
      placement.flip_vertical,
    )
    .map(|data| {
      (
        data.into(),
        Some("image/png".into()),
        ImageCrop::default(),
        false,
        false,
      )
    })
    .unwrap_or((
      image_data.data,
      content_type,
      crop,
      placement.flip_horizontal,
      placement.flip_vertical,
    ))
  } else {
    (
      image_data.data,
      content_type,
      crop,
      placement.flip_horizontal,
      placement.flip_vertical,
    )
  };

  vec![ImageItem {
    x_pt: placement.frame.x_pt,
    y_pt: placement.frame.y_pt,
    width_pt: placement.frame.width_pt,
    height_pt: placement.frame.height_pt,
    crop,
    clip_path: placement.clip_path,
    rotation_deg: placement.rotation_deg,
    flip_horizontal,
    flip_vertical,
    data,
    content_type,
    alt_text: placement.alt_text,
    hyperlink_url: placement.hyperlink_url,
    floating: false,
    behind_text: false,
  }]
}

fn tiled_blip_fill_image_items(
  data: Arc<[u8]>,
  content_type: Option<String>,
  tile: &a::Tile,
  placement: ImageFillPlacement,
) -> Vec<ImageItem> {
  let (mut tile_width_pt, mut tile_height_pt) =
    image_tile_size_pt(&data).unwrap_or((placement.frame.width_pt, placement.frame.height_pt));
  let scale_x = tile
    .horizontal_ratio
    .as_ref()
    .map(|value| value.as_ratio() as f32)
    .unwrap_or(1.0);
  let scale_y = tile
    .vertical_ratio
    .as_ref()
    .map(|value| value.as_ratio() as f32)
    .unwrap_or(1.0);
  tile_width_pt = (tile_width_pt * scale_x)
    .max(1.0)
    .min(placement.frame.width_pt.max(1.0));
  tile_height_pt = (tile_height_pt * scale_y)
    .max(1.0)
    .min(placement.frame.height_pt.max(1.0));
  let offset_x_pt = tile
    .horizontal_offset
    .map(|value| units::emu_to_points(value.to_emu()))
    .unwrap_or(0.0);
  let offset_y_pt = tile
    .vertical_offset
    .map(|value| units::emu_to_points(value.to_emu()))
    .unwrap_or(0.0);

  let mut start_x = placement.frame.x_pt + offset_x_pt % tile_width_pt;
  while start_x > placement.frame.x_pt {
    start_x -= tile_width_pt;
  }
  let mut start_y = placement.frame.y_pt + offset_y_pt % tile_height_pt;
  while start_y > placement.frame.y_pt {
    start_y -= tile_height_pt;
  }

  let mut images = Vec::new();
  let mut y = start_y;
  let mut row = 0usize;
  while y < placement.frame.y_pt + placement.frame.height_pt && images.len() < 256 {
    let mut x = start_x;
    let mut column = 0usize;
    while x < placement.frame.x_pt + placement.frame.width_pt && images.len() < 256 {
      let item_x = x.max(placement.frame.x_pt);
      let item_y = y.max(placement.frame.y_pt);
      let item_right = (x + tile_width_pt).min(placement.frame.x_pt + placement.frame.width_pt);
      let item_bottom = (y + tile_height_pt).min(placement.frame.y_pt + placement.frame.height_pt);
      if item_right > item_x && item_bottom > item_y {
        let crop = ImageCrop {
          left: ((item_x - x) / tile_width_pt).max(0.0),
          top: ((item_y - y) / tile_height_pt).max(0.0),
          right: ((x + tile_width_pt - item_right) / tile_width_pt).max(0.0),
          bottom: ((y + tile_height_pt - item_bottom) / tile_height_pt).max(0.0),
        };
        let (tile_flip_h, tile_flip_v) = tile_flip(tile.flip, row, column);
        images.push(ImageItem {
          x_pt: item_x,
          y_pt: item_y,
          width_pt: item_right - item_x,
          height_pt: item_bottom - item_y,
          crop,
          clip_path: placement.clip_path.clone(),
          rotation_deg: placement.rotation_deg,
          flip_horizontal: placement.flip_horizontal ^ tile_flip_h,
          flip_vertical: placement.flip_vertical ^ tile_flip_v,
          data: Arc::clone(&data),
          content_type: content_type.clone(),
          alt_text: placement.alt_text.clone(),
          hyperlink_url: placement.hyperlink_url.clone(),
          floating: false,
          behind_text: false,
        });
      }
      x += tile_width_pt;
      column += 1;
    }
    y += tile_height_pt;
    row += 1;
  }
  images
}

fn image_tile_size_pt(data: &[u8]) -> Option<(f32, f32)> {
  let image = image::load_from_memory(data).ok()?;
  Some((
    image.width() as f32 * units::POINTS_PER_CSS_PIXEL,
    image.height() as f32 * units::POINTS_PER_CSS_PIXEL,
  ))
}

fn tile_flip(flip: Option<a::TileFlipValues>, row: usize, column: usize) -> (bool, bool) {
  match flip.unwrap_or_default() {
    a::TileFlipValues::None => (false, false),
    a::TileFlipValues::Horizontal => (column % 2 == 1, false),
    a::TileFlipValues::Vertical => (false, row % 2 == 1),
    a::TileFlipValues::HorizontalAndVertical => (column % 2 == 1, row % 2 == 1),
  }
}

struct ImportedImageData {
  data: Arc<[u8]>,
  content_type: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct ImageEffects {
  color_change: Option<ColorChangeEffect>,
  grayscale: bool,
  watermark: bool,
  brightness: Option<i32>,
  contrast: Option<i32>,
  bilevel_threshold: Option<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ColorChangeEffect {
  from: RgbColor,
  to: RgbColor,
  alpha: u8,
  tolerance: u8,
}

fn image_data_with_blip_effects(
  import: &PowerPointImport,
  slide: &SlidePersist,
  data: &Arc<[u8]>,
  content_type: Option<&str>,
  blip_choices: &[a::BlipChoice],
) -> ImportedImageData {
  let data_ref = data.as_ref();
  let effects = image_effects_from_blip(import, slide, blip_choices, content_type);
  if effects == ImageEffects::default() {
    return ImportedImageData {
      data: Arc::clone(data),
      content_type: content_type.map(str::to_string),
    };
  }
  let Some(data) = apply_image_effects(data_ref, content_type, effects) else {
    return ImportedImageData {
      data: Arc::clone(data),
      content_type: content_type.map(str::to_string),
    };
  };
  ImportedImageData {
    data: data.into(),
    content_type: Some("image/png".into()),
  }
}

fn image_effects_from_blip(
  import: &PowerPointImport,
  slide: &SlidePersist,
  blip_choices: &[a::BlipChoice],
  content_type: Option<&str>,
) -> ImageEffects {
  let mut effects = ImageEffects::default();
  for choice in blip_choices {
    match choice {
      a::BlipChoice::ColorChange(change) => {
        let Some(from) = change
          .color_from
          .color_from_choice
          .as_ref()
          .and_then(|choice| resolve_color_from_choice(import, slide, choice))
        else {
          continue;
        };
        let Some(to) = change
          .color_to
          .color_to_choice
          .as_ref()
          .and_then(|choice| resolve_color_to_choice(import, slide, choice))
        else {
          continue;
        };
        if from.r != to.r || from.g != to.g || from.b != to.b || to.alpha < 100_000 {
          effects.color_change = Some(ColorChangeEffect {
            from: RgbColor {
              r: from.r,
              g: from.g,
              b: from.b,
            },
            to: RgbColor {
              r: to.r,
              g: to.g,
              b: to.b,
            },
            alpha: ((to.alpha.clamp(0, 100_000) as u32 * 255 + 50_000) / 100_000) as u8,
            tolerance: color_change_tolerance(content_type),
          });
        }
      }
      a::BlipChoice::Grayscale => effects.grayscale = true,
      a::BlipChoice::LuminanceEffect(luminance) => {
        let brightness = luminance
          .brightness
          .as_ref()
          .map(|value| (value.as_ratio() * 100.0).round() as i32);
        let contrast = luminance
          .contrast
          .as_ref()
          .map(|value| (value.as_ratio() * 100.0).round() as i32);
        // maps MSO washout (bright=70%, contrast=-70%) to
        // GraphicDrawMode::Watermark and clears AdjustLuminance/Contrast.
        if brightness == Some(70) && contrast == Some(-70) {
          effects.watermark = true;
          effects.brightness = None;
          effects.contrast = None;
        } else {
          effects.brightness = brightness;
          effects.contrast = contrast;
        }
      }
      a::BlipChoice::BiLevel(bilevel) => {
        // lclApplyBlackWhiteEffect maps DrawingML's 0..100000 threshold to
        // BitmapMonochromeFilter's 0..255 luminance threshold.
        effects.bilevel_threshold = Some(
          (bilevel.threshold.as_ratio() * 255.0)
            .round()
            .clamp(0.0, 255.0) as u8,
        );
      }
      _ => {}
    }
  }
  effects
}

fn resolve_color_from_choice(
  import: &PowerPointImport,
  slide: &SlidePersist,
  choice: &a::ColorFromChoice,
) -> Option<super::drawingml::color::ResolvedColor> {
  let color = Color::from_color_from_choice(choice)?;
  import.resolve_color_for_slide(slide, &color, None)
}

fn resolve_color_to_choice(
  import: &PowerPointImport,
  slide: &SlidePersist,
  choice: &a::ColorToChoice,
) -> Option<super::drawingml::color::ResolvedColor> {
  let color = Color::from_color_to_choice(choice)?;
  import.resolve_color_for_slide(slide, &color, None)
}

fn color_change_tolerance(content_type: Option<&str>) -> u8 {
  // lclCheckAndApplyChangeColorTransform uses 9 by default, native JPEG 15,
  // native PNG/TIFF 1, and native BMP 0.
  match content_type {
    Some("image/jpeg" | "image/jpg") => 15,
    Some("image/png" | "image/tiff" | "image/tif") => 1,
    Some("image/bmp" | "image/x-bmp") => 0,
    _ => 9,
  }
}

fn apply_image_effects(
  data: &[u8],
  content_type: Option<&str>,
  effects: ImageEffects,
) -> Option<Vec<u8>> {
  let raster_data = emf_wmf::decode_metafile_as_raster(data, content_type)
    .ok()
    .flatten()
    .map(|raster| raster.data);
  let image_data = raster_data.as_deref().unwrap_or(data);
  let mut image = image::load_from_memory(image_data).ok()?.to_rgba8();
  for pixel in image.pixels_mut() {
    let [mut r, mut g, mut b, mut a] = pixel.0;
    if let Some(effect) = effects.color_change
      && channel_within_tolerance(r, effect.from.r, effect.tolerance)
      && channel_within_tolerance(g, effect.from.g, effect.tolerance)
      && channel_within_tolerance(b, effect.from.b, effect.tolerance)
    {
      r = effect.to.r;
      g = effect.to.g;
      b = effect.to.b;
      a = effect.alpha;
    }
    if effects.grayscale {
      let luminance = image_luminance(r, g, b);
      r = luminance;
      g = luminance;
      b = luminance;
    }
    if effects.watermark {
      r = libreoffice_luminance_contrast_component(r, 0.5, -0.7);
      g = libreoffice_luminance_contrast_component(g, 0.5, -0.7);
      b = libreoffice_luminance_contrast_component(b, 0.5, -0.7);
    }
    if effects.brightness.is_some() || effects.contrast.is_some() {
      let brightness = effects.brightness.unwrap_or(0);
      let contrast = effects.contrast.unwrap_or(0);
      r = mso_brightness_contrast_component(r, brightness, contrast);
      g = mso_brightness_contrast_component(g, brightness, contrast);
      b = mso_brightness_contrast_component(b, brightness, contrast);
    }
    if let Some(threshold) = effects.bilevel_threshold {
      let value = if image_luminance(r, g, b) >= threshold {
        255
      } else {
        0
      };
      r = value;
      g = value;
      b = value;
    }
    pixel.0 = [r, g, b, a];
  }

  let mut output = Vec::new();
  let encoder = PngEncoder::new(Cursor::new(&mut output));
  encoder
    .write_image(
      image.as_raw(),
      image.width(),
      image.height(),
      ColorType::Rgba8.into(),
    )
    .ok()?;
  Some(output)
}

fn transform_image_data_to_png(
  data: &[u8],
  crop: ImageCrop,
  flip_horizontal: bool,
  flip_vertical: bool,
) -> Option<Vec<u8>> {
  let mut image = image::load_from_memory(data).ok()?.to_rgba8();
  let width = image.width();
  let height = image.height();
  if width == 0 || height == 0 {
    return None;
  }
  // lclCropGraphic rounds the srcRect-derived quotients against bitmap pixels
  // and creates a cropped bitmap before assigning it as custom-shape fill.
  let left = ((width as f32) * crop.left)
    .round()
    .clamp(0.0, width as f32) as u32;
  let top = ((height as f32) * crop.top)
    .round()
    .clamp(0.0, height as f32) as u32;
  let right = ((width as f32) * crop.right)
    .round()
    .clamp(0.0, width as f32) as u32;
  let bottom = ((height as f32) * crop.bottom)
    .round()
    .clamp(0.0, height as f32) as u32;
  if left + right >= width || top + bottom >= height {
    return None;
  }

  image = image::imageops::crop_imm(
    &image,
    left,
    top,
    width - left - right,
    height - top - bottom,
  )
  .to_image();
  // lclMirrorGraphic mirrors custom-shape fill bitmaps directly instead of
  // relying on a shape-level bitmap flip.
  if flip_horizontal {
    image = image::imageops::flip_horizontal(&image);
  }
  if flip_vertical {
    image = image::imageops::flip_vertical(&image);
  }
  let mut output = Vec::new();
  let encoder = PngEncoder::new(Cursor::new(&mut output));
  encoder
    .write_image(
      image.as_raw(),
      image.width(),
      image.height(),
      ColorType::Rgba8.into(),
    )
    .ok()?;
  Some(output)
}

fn channel_within_tolerance(actual: u8, expected: u8, tolerance: u8) -> bool {
  actual.abs_diff(expected) <= tolerance
}

fn image_luminance(r: u8, g: u8, b: u8) -> u8 {
  // DrawingML gray uses the relative intensities of the sRGB primaries.
  // Keep the IEC 61966-2-1 / Rec. 709 coefficients in integer form so image
  // effects are deterministic across platforms.
  ((u32::from(r) * 2_126 + u32::from(g) * 7_152 + u32::from(b) * 722 + 5_000) / 10_000).min(255)
    as u8
}

fn mso_brightness_contrast_component(value: u8, brightness: i32, contrast: i32) -> u8 {
  let contrast = contrast.clamp(-100, 100) as f32;
  let slope = if contrast >= 0.0 {
    128.0 / (128.0 - 1.27 * contrast)
  } else {
    (128.0 + 1.27 * contrast) / 128.0
  };
  let offset = brightness.clamp(-100, 100) as f32 * 2.55;
  ((f32::from(value) + offset / 2.0 - 128.0) * slope + 128.0 + offset / 2.0)
    .round()
    .clamp(0.0, 255.0) as u8
}

fn libreoffice_luminance_contrast_component(value: u8, luminance: f32, contrast: f32) -> u8 {
  // basegfx/source/color/bcolormodifier.cxx
  // BColorModifier_RGBLuminanceContrast applies contrast as a 0..1 slope and
  // combines luminance with the prepared contrast offset.
  let luminance = luminance.clamp(-1.0, 1.0);
  let contrast = contrast.clamp(-1.0, 1.0);
  let contrast_offset = if contrast >= 0.0 {
    128.0 / (128.0 - contrast * 127.0)
  } else {
    (128.0 + contrast * 127.0) / 128.0
  };
  let prepared_contrast_offset = (128.0 - contrast_offset * 128.0) / 255.0;
  ((f32::from(value) / 255.0) * contrast_offset + luminance + prepared_contrast_offset)
    .clamp(0.0, 1.0)
    .mul_add(255.0, 0.0)
    .round() as u8
}

fn blip_fill_image_crop(blip_fill: &a::BlipFill) -> ImageCrop {
  blip_fill
    .blip_fill_choice
    .as_ref()
    .and_then(|choice| match choice {
      a::BlipFillChoice::Stretch(stretch) => stretch.fill_rectangle.as_ref(),
      a::BlipFillChoice::Tile(_) => None,
    })
    .map(image_crop_from_fill_rectangle)
    .or_else(|| {
      blip_fill
        .source_rectangle
        .as_ref()
        .map(image_crop_from_source_rectangle)
    })
    .unwrap_or_default()
}

fn image_crop_from_source_rectangle(rect: &a::SourceRectangle) -> ImageCrop {
  // CropQuotientsFromSrcRect clamps negative srcRect edges to zero before
  // deriving crop quotients.
  let left = drawingml_percent_ratio(rect.left.as_ref()).max(0.0);
  let top = drawingml_percent_ratio(rect.top.as_ref()).max(0.0);
  let right = drawingml_percent_ratio(rect.right.as_ref()).max(0.0);
  let bottom = drawingml_percent_ratio(rect.bottom.as_ref()).max(0.0);
  if left + right >= 1.0 || top + bottom >= 1.0 {
    return ImageCrop::default();
  }
  ImageCrop {
    left,
    top,
    right,
    bottom,
  }
}

fn image_crop_from_fill_rectangle(rect: &a::FillRectangle) -> ImageCrop {
  // CropQuotientsFromFillRect ignores positive fillRect edges and computes
  // crop quotients from the negative growth denominator.
  let left = drawingml_percent_ratio(rect.left.as_ref()).min(0.0);
  let top = drawingml_percent_ratio(rect.top.as_ref()).min(0.0);
  let right = drawingml_percent_ratio(rect.right.as_ref()).min(0.0);
  let bottom = drawingml_percent_ratio(rect.bottom.as_ref()).min(0.0);
  let horizontal_divisor = -1.0 + left + right;
  let vertical_divisor = -1.0 + top + bottom;
  ImageCrop {
    left: left / horizontal_divisor,
    top: top / vertical_divisor,
    right: right / horizontal_divisor,
    bottom: bottom / vertical_divisor,
  }
}

fn drawingml_percent_ratio(value: Option<&DrawingmlPercentageValue>) -> f32 {
  value.map(|value| value.as_ratio() as f32).unwrap_or(0.0)
}

fn lower_text_body(
  context: PptxLoweringContext<'_>,
  shape: &Shape,
  offset: DisplayOffset,
  text_body: &TextBody,
  items: &mut Vec<PageItem>,
  summary: Option<&mut PptxLayoutSummary>,
) {
  let adjusted_text_body;
  let text_body = if shape.rotation.abs() > f32::EPSILON
    && text_body
      .display_properties
      .text_camera_z_rotation
      .is_some()
  {
    adjusted_text_body = {
      let mut text_body = text_body.clone();
      let shape_rotation = (shape.rotation * 60_000.0).round() as i32;
      text_body.display_properties.text_area_rotation = Some(
        text_body
          .display_properties
          .text_area_rotation
          .unwrap_or_default()
          + shape_rotation,
      );
      text_body
    };
    &adjusted_text_body
  } else {
    text_body
  };
  let text_box = text_box_metrics(shape, offset, text_body);
  lower_text_body_at_with_font_ref(
    TextBodyLoweringContext {
      import: context.import,
      slide: Some(context.slide),
      image_resources: Some(&context.slide.image_resources),
      page_index: context.page_index,
    },
    text_box,
    text_body,
    shape
      .shape_style_refs
      .as_ref()
      .map(|style| &style.font_reference),
    shape.hyperlink_url.as_deref(),
    summary,
    items,
  );
}

fn lower_text_body_at_with_table_style(
  import: &PowerPointImport,
  frame: TextFrame,
  text_body: &TextBody,
  table_text_style: Option<&TableStyleTextProperties>,
  items: &mut Vec<PageItem>,
) {
  lower_text_body_at_with_style(
    import,
    frame,
    text_body,
    TextStyleLoweringInputs {
      table_text_style,
      ..TextStyleLoweringInputs::default()
    },
    TextLoweringRuntime::default(),
    None,
    items,
  );
}

fn lower_text_body_at_with_font_ref(
  context: TextBodyLoweringContext<'_>,
  frame: TextFrame,
  text_body: &TextBody,
  font_reference: Option<&FontStyleReference>,
  shape_hyperlink_url: Option<&str>,
  summary: Option<&mut PptxLayoutSummary>,
  items: &mut Vec<PageItem>,
) {
  lower_text_body_at_with_style(
    context.import,
    frame,
    text_body,
    TextStyleLoweringInputs {
      font_reference,
      shape_hyperlink_url,
      ..TextStyleLoweringInputs::default()
    },
    TextLoweringRuntime {
      image_resources: context.image_resources,
      page_index: context.page_index,
      slide: context.slide,
    },
    summary,
    items,
  );
}

#[derive(Clone, Copy)]
struct TextBodyLoweringContext<'a> {
  import: &'a PowerPointImport,
  slide: Option<&'a SlidePersist>,
  image_resources: Option<&'a HashMap<String, ImageResource>>,
  page_index: usize,
}

#[derive(Clone, Copy, Default)]
struct TextStyleLoweringInputs<'a> {
  font_reference: Option<&'a FontStyleReference>,
  table_text_style: Option<&'a TableStyleTextProperties>,
  shape_hyperlink_url: Option<&'a str>,
  base_font_size_pt: Option<f32>,
  auto_fit_font_scale: Option<f32>,
  rotation_center_pt: Option<(f32, f32)>,
}

#[derive(Clone, Copy, Default)]
struct TextLoweringRuntime<'a> {
  image_resources: Option<&'a HashMap<String, ImageResource>>,
  page_index: usize,
  slide: Option<&'a SlidePersist>,
}

fn lower_text_body_at_with_style(
  import: &PowerPointImport,
  frame: TextFrame,
  text_body: &TextBody,
  style_inputs: TextStyleLoweringInputs<'_>,
  runtime: TextLoweringRuntime<'_>,
  summary: Option<&mut PptxLayoutSummary>,
  items: &mut Vec<PageItem>,
) {
  lower_text_body_at_with_style_and_scale(
    import,
    frame,
    text_body,
    style_inputs,
    runtime,
    summary,
    items,
  );
}

fn lower_text_body_at_with_style_and_scale(
  import: &PowerPointImport,
  frame: TextFrame,
  text_body: &TextBody,
  style_inputs: TextStyleLoweringInputs<'_>,
  runtime: TextLoweringRuntime<'_>,
  mut summary: Option<&mut PptxLayoutSummary>,
  items: &mut Vec<PageItem>,
) {
  let mut options = TextLoweringOptions::from_text_body(text_body);
  options.rotation_center_pt = style_inputs
    .rotation_center_pt
    .or_else(|| rotated_text_area_center(frame, options.rotation_deg));
  let base_style = text_base_style(
    import,
    runtime.slide,
    text_body,
    style_inputs.table_text_style,
    style_inputs.base_font_size_pt,
  );
  let mut text_metrics = TextMetrics::new();
  let (font_scale, line_scale) = style_inputs.auto_fit_font_scale.map_or_else(
    || text_auto_fit_scales(&options),
    |font_scale| (font_scale, options.line_scale),
  );
  options.font_scale = font_scale;
  options.line_scale = line_scale;

  let estimated_height = estimate_wrapped_text_body_height(
    TextBodyHeightContext {
      import,
      slide: runtime.slide,
      frame,
      base_style: &base_style,
      font_reference: style_inputs.font_reference,
      options: &options,
      slide_number: presentation_slide_number(import, runtime.page_index),
    },
    text_body,
    &mut text_metrics,
  );
  let y_pt = match text_body.display_properties.anchor {
    a::TextAnchoringTypeValues::Center => frame.y_pt + (frame.height_pt - estimated_height) / 2.0,
    a::TextAnchoringTypeValues::Bottom => frame.y_pt + frame.height_pt - estimated_height,
    a::TextAnchoringTypeValues::Top
    | a::TextAnchoringTypeValues::Justified
    | a::TextAnchoringTypeValues::Distributed => frame.y_pt,
  };

  let mut cursor = TextCursor {
    x_pt: frame.x_pt,
    y_pt,
    column_index: 0,
  };
  let item_start = items.len();
  let mut auto_numbering = AutoNumberingState::default();
  for (paragraph_index, paragraph) in text_body.paragraphs.iter().enumerate() {
    lower_paragraph(
      ParagraphLoweringContext {
        import,
        slide: runtime.slide,
        base_style: &base_style,
        font_reference: style_inputs.font_reference,
        options: &options,
        frame,
        shape_hyperlink_url: style_inputs.shape_hyperlink_url,
        image_resources: runtime.image_resources,
        page_index: runtime.page_index,
        slide_number: presentation_slide_number(import, runtime.page_index),
        paragraph_count: text_body.paragraphs.len(),
      },
      paragraph,
      paragraph_index,
      ParagraphLoweringOutput {
        summary: summary.as_deref_mut(),
        cursor: &mut cursor,
        items,
        text_metrics: &mut text_metrics,
        auto_numbering: &mut auto_numbering,
      },
    );
  }
  apply_text_camera_z_rotation(
    &mut items[item_start..],
    text_body.display_properties.camera_z_rotation_degrees(),
    &mut text_metrics,
  );
}

fn apply_text_camera_z_rotation(
  items: &mut [PageItem],
  rotation_deg: f32,
  text_metrics: &mut TextMetrics,
) {
  if rotation_deg.abs() <= f32::EPSILON {
    return;
  }
  let Some((left, top, right, bottom)) = text_items_bounds(items, text_metrics) else {
    return;
  };
  let center_x = (left + right) / 2.0;
  let center_y = (top + bottom) / 2.0;
  for item in items {
    let PageItem::Text(text) = item else {
      continue;
    };
    text.style.rotation_deg += rotation_deg;
    text.rotation_center_pt = Some((center_x, center_y));
  }
}

fn text_items_bounds(
  items: &[PageItem],
  text_metrics: &mut TextMetrics,
) -> Option<(f32, f32, f32, f32)> {
  let mut bounds: Option<(f32, f32, f32, f32)> = None;
  for item in items {
    let PageItem::Text(text) = item else {
      continue;
    };
    let width_pt = text_metrics.measure_text(&text.text, &text.style);
    let left = text.x_pt;
    let top = text.y_pt - text.line_height_pt;
    let right = text.x_pt + width_pt;
    let bottom = text.y_pt;
    bounds = Some(match bounds {
      Some((old_left, old_top, old_right, old_bottom)) => (
        old_left.min(left),
        old_top.min(top),
        old_right.max(right),
        old_bottom.max(bottom),
      ),
      None => (left, top, right, bottom),
    });
  }
  bounds
}

fn text_base_style(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  text_body: &TextBody,
  table_text_style: Option<&TableStyleTextProperties>,
  base_font_size_pt: Option<f32>,
) -> TextStyle {
  let options = TextLoweringOptions::from_text_body(text_body);
  // DrawingML shape creation seeds all three script families from the
  // current theme's minor font collection before paragraph/run formatting is
  // applied. LibreOffice does this in oox/source/drawingml/shape.cxx; direct
  // a:latin/a:ea/a:cs values below still override these inherited defaults.
  let theme_latin = import
    .resolve_theme_font_for_slide(slide, "+mn-lt")
    .unwrap_or("Liberation Sans");
  let mut base_style = TextStyle {
    font_family: Some(Arc::from(theme_latin)),
    font_size_pt: base_font_size_pt.unwrap_or(DEFAULT_TEXT_FONT_SIZE_PT),
    use_windows_font_metrics: true,
    rotation_deg: options.rotation_deg,
    ..TextStyle::default()
  };
  base_style.east_asia_font_family = import
    .resolve_theme_font_for_slide(slide, "+mn-ea")
    .map(Arc::from);
  base_style.complex_font_family = import
    .resolve_theme_font_for_slide(slide, "+mn-cs")
    .map(Arc::from);
  if let Some(table_text_style) = table_text_style {
    apply_table_text_style(import, slide, table_text_style, &mut base_style);
  }
  base_style
}

fn text_auto_fit_scales(options: &TextLoweringOptions) -> (f32, f32) {
  // a:spAutoFit grows the containing shape; it does not shrink the text or
  // reduce its line spacing. a:normAutofit is the text-scaling mode and its
  // explicit fontScale/lnSpcReduction values are already carried by options.
  (options.font_scale, options.line_scale)
}

fn apply_font_reference_text_style(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  reference: &FontStyleReference,
  style: &mut TextStyle,
) {
  if let Some(typeface) = import.get_theme_latin_font(reference.index) {
    style.font_family = Some(Arc::from(typeface));
  }
  if let Some(paint) = reference
    .placeholder_color
    .as_ref()
    .and_then(|color| display_paint_for_optional_slide(import, slide, color, None))
  {
    style.color = paint.color;
    style.opacity = paint.opacity;
  }
}

fn apply_table_text_style(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  properties: &TableStyleTextProperties,
  style: &mut TextStyle,
) {
  if let Some(font_reference) = &properties.font_reference {
    apply_font_reference_text_style(import, slide, font_reference, style);
  }
  if let Some(typeface) = properties.fonts.latin.as_deref() {
    style.font_family = Some(Arc::from(typeface));
  }
  if let Some(bold) = properties.bold.and_then(boolean_style_value) {
    style.bold = bold;
  }
  if let Some(italic) = properties.italic.and_then(boolean_style_value) {
    style.italic = italic;
  }
  if let Some(paint) = properties
    .color
    .as_ref()
    .and_then(|color| display_paint_for_optional_slide(import, slide, color, None))
  {
    style.color = paint.color;
    style.opacity = paint.opacity;
  }
}

fn boolean_style_value(value: a::BooleanStyleValues) -> Option<bool> {
  match value {
    a::BooleanStyleValues::On => Some(true),
    a::BooleanStyleValues::Off => Some(false),
    a::BooleanStyleValues::Default => None,
  }
}

#[derive(Clone, Copy, Debug)]
struct TextFrame {
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
}

#[derive(Clone, Copy, Debug)]
struct ShapeTextDistances100mm {
  left: i32,
  top: i32,
  right: i32,
  bottom: i32,
}

struct DrawShapeSummaryParts<'a> {
  page_index: usize,
  shape_path: Vec<usize>,
  service_name: String,
  geometry: Option<String>,
  text: String,
  frame: TextFrame,
  fill: Option<&'a FillProperties>,
  rotation_deg: f32,
  flip_h: bool,
  flip_v: bool,
  text_distances: Option<ShapeTextDistances100mm>,
}

fn draw_shape_summary_from_parts(parts: DrawShapeSummaryParts<'_>) -> PptxDrawShapeSummary {
  let (fill_style, gradient_style, gradient_angle) =
    fill_summary(parts.fill, parts.rotation_deg, parts.flip_h, parts.flip_v);
  PptxDrawShapeSummary {
    page_index: parts.page_index,
    shape_path: parts.shape_path,
    service_name: parts.service_name,
    geometry: parts.geometry,
    text: parts.text,
    left_100mm: points_to_100mm(parts.frame.x_pt),
    top_100mm: points_to_100mm(parts.frame.y_pt),
    right_100mm: points_to_100mm(parts.frame.x_pt + parts.frame.width_pt),
    bottom_100mm: points_to_100mm(parts.frame.y_pt + parts.frame.height_pt),
    width_100mm: points_to_100mm(parts.frame.width_pt),
    height_100mm: points_to_100mm(parts.frame.height_pt),
    fill_style,
    fill_uses_slide_background: false,
    gradient_style,
    gradient_angle,
    text_left_distance_100mm: parts.text_distances.map(|distances| distances.left),
    text_upper_distance_100mm: parts.text_distances.map(|distances| distances.top),
    text_right_distance_100mm: parts.text_distances.map(|distances| distances.right),
    text_lower_distance_100mm: parts.text_distances.map(|distances| distances.bottom),
  }
}

fn text_distances_from_frame(
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  frame: TextFrame,
) -> ShapeTextDistances100mm {
  ShapeTextDistances100mm {
    left: points_to_100mm(frame.x_pt - x_pt),
    top: points_to_100mm(frame.y_pt - y_pt),
    right: points_to_100mm(x_pt + width_pt - frame.x_pt - frame.width_pt),
    bottom: points_to_100mm(y_pt + height_pt - frame.y_pt - frame.height_pt),
  }
}

fn shape_geometry_name(geometry: Option<&CustomShapeGeometry>) -> Option<String> {
  match geometry {
    Some(CustomShapeGeometry::Preset(preset)) => Some(format!("ooxml-{:?}", preset.preset)),
    Some(CustomShapeGeometry::Custom(_)) => Some("custom".to_string()),
    None => None,
  }
}

fn diagram_geometry_name(properties: &dsp::ShapeProperties) -> Option<String> {
  match properties.shape_properties_choice1.as_ref() {
    Some(dsp::ShapePropertiesChoice::PresetGeometry(preset)) => {
      Some(format!("ooxml-{:?}", preset.preset))
    }
    Some(dsp::ShapePropertiesChoice::CustomGeometry(_)) => Some("custom".to_string()),
    None => None,
  }
}

fn shape_text(text_body: Option<&TextBody>) -> String {
  text_body.map(text_body_plain_text).unwrap_or_default()
}

fn rotated_shape_geo_top_left(
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  rotation_deg: f32,
) -> (f32, f32) {
  if rotation_deg.abs() <= f32::EPSILON {
    return (x_pt, y_pt);
  }
  let center_x = x_pt + width_pt / 2.0;
  let center_y = y_pt + height_pt / 2.0;
  rotate_point(x_pt, y_pt, center_x, center_y, rotation_deg.to_radians())
}

fn fill_summary(
  fill: Option<&FillProperties>,
  rotation_deg: f32,
  flip_h: bool,
  flip_v: bool,
) -> (String, Option<String>, Option<i16>) {
  match fill.map(|fill| &fill.kind) {
    Some(FillKind::None) => ("None".to_string(), None, None),
    Some(FillKind::SlideBackground) => ("SlideBackground".to_string(), None, None),
    Some(FillKind::Solid(_)) => ("Solid".to_string(), None, None),
    Some(FillKind::Group) => ("Group".to_string(), None, None),
    Some(FillKind::Blip(_)) => ("Bitmap".to_string(), None, None),
    Some(FillKind::Pattern(_)) => ("Pattern".to_string(), None, None),
    Some(FillKind::Gradient(gradient)) => (
      "Gradient".to_string(),
      gradient_style(gradient),
      gradient_angle(gradient, rotation_deg, flip_h, flip_v),
    ),
    None => ("Default".to_string(), None, None),
  }
}

fn gradient_style(gradient: &a::GradientFill) -> Option<String> {
  match gradient.gradient_fill_choice.as_ref()? {
    a::GradientFillChoice::LinearGradientFill(_) => Some("Linear".to_string()),
    a::GradientFillChoice::PathGradientFill(path) => Some(match path.path {
      Some(a::PathShadeValues::Circle) => "Radial".to_string(),
      Some(a::PathShadeValues::Rectangle | a::PathShadeValues::Shape) | None => {
        "Rectangle".to_string()
      }
    }),
  }
}

fn gradient_angle(
  gradient: &a::GradientFill,
  rotation_deg: f32,
  flip_h: bool,
  flip_v: bool,
) -> Option<i16> {
  let a::GradientFillChoice::LinearGradientFill(linear) = gradient.gradient_fill_choice.as_ref()?
  else {
    return None;
  };
  let mut shade_angle = linear.angle.unwrap_or_default();
  if flip_h {
    shade_angle = 180 * 60_000 - shade_angle;
  }
  if flip_v {
    shade_angle = -shade_angle;
  }
  let shape_rotation = (rotation_deg * 60_000.0).round() as i32;
  let dml_angle = shade_angle + shape_rotation;
  Some((8100 - dml_angle / 6_000).rem_euclid(3600) as i16)
}

fn record_diagram_draw_shape_summary(
  summary: &mut PptxLayoutSummary,
  page_index: usize,
  shape: &dsp::Shape,
  bounds: shared_diagram::DiagramBounds,
  transform: DiagramDrawingTransform,
) {
  let fill = diagram_fill_properties(&shape.shape_properties);
  let text_body = shape
    .text_body
    .as_deref()
    .map(TextBody::from_diagram_drawing);
  let text_distances = text_body.as_ref().map(|text_body| {
    let frame = diagram_drawing_text_frame(shape, bounds, transform, text_body);
    frame.text_distances_100mm.unwrap_or_else(|| {
      text_distances_from_frame(bounds.x, bounds.y, bounds.width, bounds.height, frame.frame)
    })
  });
  let rotation_deg = shape
    .shape_properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.rotation)
    .map(|rotation| rotation as f32 / 60_000.0)
    .unwrap_or_default();
  summary
    .draw_shapes
    .push(draw_shape_summary_from_parts(DrawShapeSummaryParts {
      page_index,
      shape_path: Vec::new(),
      service_name: "DiagramShape".to_string(),
      geometry: diagram_geometry_name(&shape.shape_properties),
      text: text_body
        .as_ref()
        .map(text_body_plain_text)
        .unwrap_or_default(),
      frame: TextFrame {
        x_pt: bounds.x,
        y_pt: bounds.y,
        width_pt: bounds.width,
        height_pt: bounds.height,
      },
      fill: fill.as_ref(),
      rotation_deg,
      flip_h: shape
        .shape_properties
        .transform2_d
        .as_deref()
        .and_then(|transform| transform.horizontal_flip)
        .is_some_and(|value| value.as_bool()),
      flip_v: shape
        .shape_properties
        .transform2_d
        .as_deref()
        .and_then(|transform| transform.vertical_flip)
        .is_some_and(|value| value.as_bool()),
      text_distances,
    }));
}

fn diagram_fill_properties(properties: &dsp::ShapeProperties) -> Option<FillProperties> {
  Some(match properties.shape_properties_choice2.as_ref()? {
    dsp::ShapePropertiesChoice2::NoFill(_) => FillProperties {
      kind: FillKind::None,
      placeholder_color: None,
    },
    dsp::ShapePropertiesChoice2::SolidFill(fill) => FillProperties {
      kind: FillKind::Solid(
        fill
          .solid_fill_choice
          .as_ref()
          .and_then(Color::from_solid_fill_choice),
      ),
      placeholder_color: None,
    },
    dsp::ShapePropertiesChoice2::GradientFill(fill) => FillProperties {
      kind: FillKind::Gradient(fill.clone()),
      placeholder_color: None,
    },
    dsp::ShapePropertiesChoice2::BlipFill(fill) => FillProperties {
      kind: FillKind::Blip(fill.clone()),
      placeholder_color: None,
    },
    dsp::ShapePropertiesChoice2::PatternFill(fill) => FillProperties {
      kind: FillKind::Pattern(fill.clone()),
      placeholder_color: None,
    },
    dsp::ShapePropertiesChoice2::GroupFill => FillProperties {
      kind: FillKind::Group,
      placeholder_color: None,
    },
  })
}

fn record_smartart_text_shape(
  summary: Option<&mut PptxLayoutSummary>,
  page_index: usize,
  text_body: &TextBody,
  text_area_x_pt: f32,
  text_area_y_pt: f32,
  frame: TextFrame,
) {
  let Some(summary) = summary else {
    return;
  };
  let text = text_body_plain_text(text_body);
  if text.trim().is_empty() {
    return;
  }
  summary
    .smartart_text_shapes
    .push(PptxSmartArtTextShapeSummary {
      page_index,
      text,
      text_left_distance_100mm: points_to_100mm(frame.x_pt - text_area_x_pt),
      text_upper_distance_100mm: points_to_100mm(frame.y_pt - text_area_y_pt),
      text_anchor_left_100mm: points_to_100mm(frame.x_pt),
      text_anchor_top_100mm: points_to_100mm(frame.y_pt),
      text_anchor_right_100mm: points_to_100mm(frame.x_pt + frame.width_pt),
      text_anchor_bottom_100mm: points_to_100mm(frame.y_pt + frame.height_pt),
    });
}

fn text_body_plain_text(text_body: &TextBody) -> String {
  text_body
    .paragraphs
    .iter()
    .map(|paragraph| {
      paragraph
        .runs
        .iter()
        .map(|run| run.text.as_str())
        .collect::<String>()
    })
    .collect::<Vec<_>>()
    .join("\n")
}

fn points_to_100mm(value: f32) -> i32 {
  (value * 2540.0 / 72.0).round() as i32
}

fn rotate_point(x: f32, y: f32, center_x: f32, center_y: f32, angle: f32) -> (f32, f32) {
  let dx = x - center_x;
  let dy = y - center_y;
  (
    center_x + dx * angle.cos() - dy * angle.sin(),
    center_y + dx * angle.sin() + dy * angle.cos(),
  )
}

#[derive(Clone, Copy, Debug, Default)]
struct TextDistances {
  left: f32,
  top: f32,
  right: f32,
  bottom: f32,
}

#[derive(Clone, Copy, Debug)]
struct TextCursor {
  x_pt: f32,
  y_pt: f32,
  column_index: usize,
}

struct TextLineRun<'a> {
  run: &'a TextRun,
  text: String,
  width_pt: f32,
  style: TextStyle,
}

#[derive(Default)]
struct TextLine<'a> {
  runs: Vec<TextLineRun<'a>>,
  width_pt: f32,
}

#[derive(Clone, Copy, Debug)]
struct TextLoweringOptions {
  font_scale: f32,
  line_scale: f32,
  use_first_last_paragraph_spacing: bool,
  round_font_size_to_pt: bool,
  rotation_deg: f32,
  rotation_center_pt: Option<(f32, f32)>,
  column_count: usize,
  column_spacing_pt: f32,
  word_wrap: bool,
  clip_vertical_overflow: bool,
  anchor_center: bool,
}

impl TextLoweringOptions {
  fn from_text_body(text_body: &TextBody) -> Self {
    Self {
      font_scale: text_body.display_properties.font_scale(),
      line_scale: text_body.display_properties.line_height_scale(),
      use_first_last_paragraph_spacing: text_body
        .display_properties
        .use_first_last_paragraph_spacing,
      round_font_size_to_pt: text_body.display_properties.auto_fit == TextAutoFit::Shape,
      rotation_deg: text_body.display_properties.rotation_degrees(),
      rotation_center_pt: None,
      column_count: text_body.display_properties.column_count.max(1),
      column_spacing_pt: units::emu_to_points(text_body.display_properties.column_spacing_emu),
      word_wrap: text_body.display_properties.word_wrap,
      clip_vertical_overflow: text_body.display_properties.clip_vertical_overflow
        && text_body.display_properties.auto_fit != TextAutoFit::Shape,
      anchor_center: text_body.display_properties.anchor_center,
    }
  }

  fn column_width(self, frame: TextFrame) -> f32 {
    if self.column_count <= 1 {
      frame.width_pt
    } else {
      let total_spacing = self.column_spacing_pt * (self.column_count - 1) as f32;
      ((frame.width_pt - total_spacing) / self.column_count as f32).max(0.0)
    }
  }
}

fn rotated_text_area_center(frame: TextFrame, rotation_deg: f32) -> Option<(f32, f32)> {
  (rotation_deg.abs() > f32::EPSILON).then_some((
    frame.x_pt + frame.width_pt / 2.0,
    frame.y_pt + frame.height_pt / 2.0,
  ))
}

fn text_box_metrics(shape: &Shape, offset: DisplayOffset, text_body: &TextBody) -> TextFrame {
  text_body_frame(
    units::emu_to_points(offset.x_emu + shape.position.x),
    units::emu_to_points(offset.y_emu + shape.position.y),
    units::emu_to_points(shape.size.cx),
    units::emu_to_points(shape.size.cy),
    text_body,
  )
}

fn text_body_frame(
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  text_body: &TextBody,
) -> TextFrame {
  text_body_frame_with_distances(
    x_pt,
    y_pt,
    width_pt,
    height_pt,
    text_body,
    TextDistances::default(),
    0,
  )
}

fn text_body_frame_with_distances(
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  text_body: &TextBody,
  offsets: TextDistances,
  text_pre_rotation: i32,
) -> TextFrame {
  // ECMA-376, Part 1, 20.1.7.1 defines the DrawingML text-body inset
  // defaults as 0.1 in horizontally and 0.05 in vertically. LibreOffice
  // seeds the same values in Shape::setDefaults before applying bodyPr.
  const DEFAULT_HORIZONTAL_INSET_EMU: i64 = 91_440;
  const DEFAULT_VERTICAL_INSET_EMU: i64 = 45_720;
  let body_properties = text_body.body_properties.as_deref();
  let insets = [
    body_properties
      .and_then(|properties| properties.left_inset)
      .map(|value| units::emu_to_points(value.to_emu()))
      .unwrap_or_else(|| units::emu_to_points(DEFAULT_HORIZONTAL_INSET_EMU)),
    body_properties
      .and_then(|properties| properties.top_inset)
      .map(|value| units::emu_to_points(value.to_emu()))
      .unwrap_or_else(|| units::emu_to_points(DEFAULT_VERTICAL_INSET_EMU)),
    body_properties
      .and_then(|properties| properties.right_inset)
      .map(|value| units::emu_to_points(value.to_emu()))
      .unwrap_or_else(|| units::emu_to_points(DEFAULT_HORIZONTAL_INSET_EMU)),
    body_properties
      .and_then(|properties| properties.bottom_inset)
      .map(|value| units::emu_to_points(value.to_emu()))
      .unwrap_or_else(|| units::emu_to_points(DEFAULT_VERTICAL_INSET_EMU)),
  ];
  let offset_values = [offsets.left, offsets.top, offsets.right, offsets.bottom];
  let mut distances = [0.0; 4];
  let mut offset_index = match text_pre_rotation.rem_euclid(21_600_000) {
    5_400_000 => 3,
    10_800_000 => 2,
    16_200_000 => 1,
    _ => 0,
  };
  match text_body.display_properties.vertical {
    Some(a::TextVerticalValues::EastAsianVetical | a::TextVerticalValues::Vertical) => {
      offset_index = (offset_index + 3) % 4;
    }
    Some(a::TextVerticalValues::Vertical270) => {
      offset_index = (offset_index + 1) % 4;
    }
    _ => {}
  }
  for inset in insets {
    distances[offset_index] = offset_values[offset_index] + inset;
    offset_index = (offset_index + 1) % 4;
  }
  if width_pt > 0.0 && distances[0] + distances[2] >= width_pt {
    let diff = (distances[0] + distances[2] - width_pt) / 2.0;
    distances[0] -= diff;
    distances[2] -= diff;
  }
  if height_pt > 0.0 && distances[1] + distances[3] >= height_pt {
    let diff = (distances[1] + distances[3] - height_pt) / 2.0;
    distances[1] -= diff;
    distances[3] -= diff;
  }

  TextFrame {
    x_pt: x_pt + distances[0],
    y_pt: y_pt + distances[1],
    width_pt: (width_pt - distances[0] - distances[2]).max(0.0),
    height_pt: (height_pt - distances[1] - distances[3]).max(0.0),
  }
}

#[derive(Clone, Copy)]
struct ParagraphLoweringContext<'a> {
  import: &'a PowerPointImport,
  slide: Option<&'a SlidePersist>,
  base_style: &'a TextStyle,
  font_reference: Option<&'a FontStyleReference>,
  options: &'a TextLoweringOptions,
  frame: TextFrame,
  shape_hyperlink_url: Option<&'a str>,
  image_resources: Option<&'a HashMap<String, ImageResource>>,
  page_index: usize,
  slide_number: i32,
  paragraph_count: usize,
}

struct ParagraphLoweringOutput<'a> {
  summary: Option<&'a mut PptxLayoutSummary>,
  cursor: &'a mut TextCursor,
  items: &'a mut Vec<PageItem>,
  text_metrics: &'a mut TextMetrics,
  auto_numbering: &'a mut AutoNumberingState,
}

fn lower_paragraph(
  context: ParagraphLoweringContext<'_>,
  paragraph: &TextParagraph,
  paragraph_index: usize,
  output: ParagraphLoweringOutput<'_>,
) {
  let ParagraphLoweringOutput {
    summary,
    cursor,
    items,
    text_metrics,
    auto_numbering,
  } = output;
  let paragraph_style = ParagraphDisplayStyle::from_paragraph(paragraph);
  let mut paragraph_base_style = context.base_style.clone();
  paragraph_style.apply_master_default_run_style(
    context.import,
    context.slide,
    &mut paragraph_base_style,
  );
  if let Some(font_reference) = context.font_reference {
    apply_font_reference_text_style(
      context.import,
      context.slide,
      font_reference,
      &mut paragraph_base_style,
    );
  }
  paragraph_style.apply_local_default_run_style(
    context.import,
    context.slide,
    &mut paragraph_base_style,
  );
  apply_text_scale(&mut paragraph_base_style, context.options);
  if paragraph_index > 0 || context.options.use_first_last_paragraph_spacing {
    cursor.y_pt += paragraph_style
      .space_before
      .points(paragraph_base_style.font_size_pt);
    advance_text_column_if_needed(cursor, context.frame, *context.options);
  }
  let column_width = context.options.column_width(context.frame);
  let column_x = context.frame.x_pt
    + cursor.column_index as f32 * (column_width + context.options.column_spacing_pt);
  cursor.x_pt = column_x;
  if context.options.clip_vertical_overflow
    && cursor.y_pt > context.frame.y_pt + context.frame.height_pt
  {
    return;
  }
  let mut bullet = paragraph_style.bullet(paragraph);
  auto_numbering.resolve(paragraph, &mut bullet);
  if let Some((width, height)) = paragraph_graphic_bullet_size_100mm(
    paragraph,
    &paragraph_style,
    &bullet,
    context.base_style,
    context.options,
    context.image_resources,
  ) {
    bullet.graphic_width_100mm = Some(width);
    bullet.graphic_height_100mm = Some(height);
  }
  record_bullet_paragraph(
    summary,
    context.page_index,
    paragraph_index,
    paragraph,
    &bullet,
  );
  let paragraph_x = cursor.x_pt
    + paragraph_style.left_margin_pt
    + if bullet.label.is_some() {
      0.0
    } else {
      paragraph_style.indent_pt
    };
  let mut segment_start = 0usize;
  let mut is_first_segment = true;

  loop {
    let segment_end = paragraph.runs[segment_start..]
      .iter()
      .position(|run| run.kind == TextRunKind::Break)
      .map(|offset| segment_start + offset)
      .unwrap_or(paragraph.runs.len());
    let text_lines = layout_text_lines(
      TextLineLayoutContext {
        import: context.import,
        slide: context.slide,
        base_style: &paragraph_base_style,
        options: context.options,
        column_width,
        slide_number: context.slide_number,
      },
      &paragraph.runs[segment_start..segment_end],
      text_metrics,
    );
    let alignment = if context.options.anchor_center {
      // maps horizontal text with anchorCtr=1 to TextHorizontalAdjust_CENTER,
      // so the shape-level adjustment overrides paragraph alignment.
      a::TextAlignmentTypeValues::Center
    } else {
      paragraph_style.alignment
    };

    for (line_index, text_line) in text_lines.iter().enumerate() {
      let mut run_x = aligned_paragraph_x(paragraph_x, column_width, text_line.width_pt, alignment);
      let base_line_style = paragraph_base_style.clone();
      let mut max_line_height = paragraph_style.line_height(&base_line_style, context.options);
      for line_run in &text_line.runs {
        max_line_height =
          max_line_height.max(paragraph_style.line_height(&line_run.style, context.options));
      }

      if is_first_segment
        && line_index == 0
        && let Some(label) = bullet.label.as_deref()
      {
        let label = shared_symbol::font_symbol_transport_text(bullet.font.as_deref(), label);
        // DrawingML's follow-text bullet properties use the first character
        // in the paragraph. `line_run.style` already includes inheritance and
        // auto-fit scaling, so it is also the correct base for explicit bullet
        // font/color/size overrides.
        let mut bullet_style = text_line
          .runs
          .first()
          .map(|run| run.style.clone())
          .unwrap_or_else(|| paragraph_base_style.clone());
        if let Some(font) = bullet.font.as_deref() {
          bullet_style.font_family = Some(Arc::from(resolve_theme_font(context.import, font)));
        }
        if let Some(paint) = bullet.color.as_ref().and_then(|color| {
          display_paint_for_optional_slide(context.import, context.slide, color, None)
        }) {
          bullet_style.color = paint.color;
          bullet_style.opacity = paint.opacity;
        }
        apply_character_bullet_size(&mut bullet_style, bullet.size);
        let bullet_line_height = paragraph_style.line_height(&bullet_style, context.options);
        max_line_height = max_line_height.max(bullet_line_height);
        let text_baseline_offset = if base_line_style.use_windows_font_metrics {
          text_metrics
            .baseline_offset_in_line_with_windows_metrics(&base_line_style, max_line_height)
        } else {
          text_metrics.baseline_offset_in_line(&base_line_style, max_line_height)
        };
        let bullet_baseline_offset = if bullet_style.use_windows_font_metrics {
          text_metrics
            .baseline_offset_in_line_with_windows_metrics(&bullet_style, bullet_line_height)
        } else {
          text_metrics.baseline_offset_in_line(&bullet_style, bullet_line_height)
        };
        let bullet_y_pt = cursor.y_pt + text_baseline_offset - bullet_baseline_offset;
        if let Some(graphic) = bullet_graphic_item(
          &bullet,
          context.image_resources,
          run_x + paragraph_style.indent_pt,
          bullet_y_pt,
          &bullet_style,
          context.shape_hyperlink_url,
        ) {
          items.push(PageItem::Image(graphic));
        } else {
          push_text_item(
            items,
            TextItemPlacement {
              x_pt: run_x + paragraph_style.indent_pt,
              y_pt: bullet_y_pt,
              line_height_pt: bullet_line_height,
              rotation_center_pt: context.options.rotation_center_pt,
            },
            label.into_owned(),
            bullet_style,
            context.shape_hyperlink_url.map(ToString::to_string),
          );
        }
      }

      for line_run in &text_line.runs {
        let style = &line_run.style;
        let run_line_height = paragraph_style.line_height(style, context.options);
        max_line_height = max_line_height.max(run_line_height);
        if line_run.run.kind == TextRunKind::Math {
          push_math_ole_preview_item(
            items,
            run_x,
            cursor.y_pt,
            line_run.width_pt,
            run_line_height,
          );
        }
        let hyperlink_url = line_run
          .run
          .hyperlink_url
          .clone()
          .or_else(|| context.shape_hyperlink_url.map(ToString::to_string));
        push_symbol_split_text_items(
          items,
          TextItemPlacement {
            x_pt: run_x,
            y_pt: cursor.y_pt,
            line_height_pt: run_line_height,
            rotation_center_pt: context.options.rotation_center_pt,
          },
          &line_run.text,
          style,
          hyperlink_url,
          text_metrics,
        );
        run_x += line_run.width_pt;
      }

      cursor.y_pt += max_line_height;
      advance_text_column_if_needed(cursor, context.frame, *context.options);
    }

    if segment_end == paragraph.runs.len() {
      break;
    }
    segment_start = segment_end + 1;
    is_first_segment = false;
  }
  if paragraph_index + 1 < context.paragraph_count
    || context.options.use_first_last_paragraph_spacing
  {
    cursor.y_pt += paragraph_style
      .space_after
      .points(paragraph_base_style.font_size_pt);
    advance_text_column_if_needed(cursor, context.frame, *context.options);
  }
}

#[derive(Clone, Copy)]
struct TextLineLayoutContext<'a> {
  import: &'a PowerPointImport,
  slide: Option<&'a SlidePersist>,
  base_style: &'a TextStyle,
  options: &'a TextLoweringOptions,
  column_width: f32,
  slide_number: i32,
}

fn layout_text_lines<'a>(
  context: TextLineLayoutContext<'_>,
  runs: &'a [TextRun],
  text_metrics: &mut TextMetrics,
) -> Vec<TextLine<'a>> {
  let visible_runs = runs
    .iter()
    .filter(|run| {
      matches!(
        run.kind,
        TextRunKind::Run | TextRunKind::Field | TextRunKind::Math
      ) && !run.text.is_empty()
    })
    .collect::<Vec<_>>();
  if visible_runs.is_empty() {
    return vec![TextLine::default()];
  }

  let mut lines = Vec::new();
  let mut current = TextLine::default();
  for run in visible_runs {
    let style = styled_text_run(
      context.import,
      context.slide,
      context.base_style,
      context.options,
      run,
    );
    let field_text = presentation_field_text(run, context.slide_number);
    let uppercase_text;
    let text = if style.uppercase {
      uppercase_text = field_text.to_uppercase();
      uppercase_text.as_str()
    } else {
      field_text.as_ref()
    };
    for hard_line in text.split_inclusive('\n') {
      let (line_text, has_hard_break) = hard_line
        .strip_suffix('\n')
        .map_or((hard_line, false), |text| (text, true));
      for token in text_wrap_tokens(line_text) {
        let width_pt = text_metrics.measure_text(token, &style);
        if context.options.word_wrap
          && current.width_pt > f32::EPSILON
          && current.width_pt + width_pt > context.column_width
        {
          trim_text_line_end(&mut current, text_metrics);
          lines.push(current);
          current = TextLine::default();
        }
        push_text_line_token(&mut current, run, token, width_pt, &style);
      }
      if has_hard_break {
        trim_text_line_end(&mut current, text_metrics);
        lines.push(current);
        current = TextLine::default();
      }
    }
  }
  trim_text_line_end(&mut current, text_metrics);
  lines.push(current);
  lines
}

fn push_text_line_token<'a>(
  line: &mut TextLine<'a>,
  run: &'a TextRun,
  token: &str,
  width_pt: f32,
  style: &TextStyle,
) {
  line.width_pt += width_pt;
  if let Some(last) = line.runs.last_mut()
    && std::ptr::eq(last.run, run)
  {
    last.text.push_str(token);
    last.width_pt += width_pt;
    return;
  }
  line.runs.push(TextLineRun {
    run,
    text: token.to_string(),
    width_pt,
    style: style.clone(),
  });
}

fn text_wrap_tokens(text: &str) -> Vec<&str> {
  if text.is_empty() {
    return Vec::new();
  }
  let mut tokens = Vec::new();
  let mut start = 0usize;
  for (index, ch) in text.char_indices() {
    if ch.is_whitespace() {
      let end = index + ch.len_utf8();
      tokens.push(&text[start..end]);
      start = end;
    }
  }
  if start < text.len() {
    tokens.push(&text[start..]);
  }
  tokens
}

fn trim_text_line_end(line: &mut TextLine<'_>, text_metrics: &mut TextMetrics) {
  while let Some(run) = line.runs.last_mut() {
    let trimmed_len = run.text.trim_end().len();
    if trimmed_len == run.text.len() {
      break;
    }
    let trimmed = run.text[..trimmed_len].to_string();
    let removed_width = run.width_pt - text_metrics.measure_text(&trimmed, &run.style);
    run.text = trimmed;
    run.width_pt -= removed_width;
    line.width_pt -= removed_width;
    if !run.text.is_empty() {
      break;
    }
    line.runs.pop();
  }
}

fn styled_text_run(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  base_style: &TextStyle,
  options: &TextLoweringOptions,
  run: &TextRun,
) -> TextStyle {
  let mut style = base_style.clone();
  apply_run_properties(import, slide, run, &mut style);
  apply_text_scale(&mut style, options);
  style
}

fn apply_text_scale(style: &mut TextStyle, options: &TextLoweringOptions) {
  style.font_size_pt = if options.round_font_size_to_pt {
    // setRoundFontSizeToPt(true) for AUTOFIT; editeng then rounds the
    // unscaled font size and the scaled font size to the nearest point.
    (style.font_size_pt.round() * options.font_scale).round()
  } else {
    style.font_size_pt * options.font_scale
  };
  // PowerPoint's PDF path lays out type on its 600 dpi print grid. Preserve
  // that device-space quantization before shaping: e.g. 40 pt becomes
  // 333/600 in and 20 pt becomes 167/600 in, matching the emitted Office PDF
  // text matrices without case-specific offsets.
  style.font_size_pt =
    (style.font_size_pt * POWERPOINT_PRINT_DPI / 72.0).round() * 72.0 / POWERPOINT_PRINT_DPI;
  style.character_spacing_pt *= options.font_scale;
  style.baseline_shift_pt *= options.font_scale;
}

fn aligned_paragraph_x(
  paragraph_x: f32,
  column_width: f32,
  line_width: f32,
  alignment: a::TextAlignmentTypeValues,
) -> f32 {
  match alignment {
    a::TextAlignmentTypeValues::Center => {
      paragraph_x + ((column_width - line_width) / 2.0).max(0.0)
    }
    a::TextAlignmentTypeValues::Right => paragraph_x + (column_width - line_width).max(0.0),
    a::TextAlignmentTypeValues::Left
    | a::TextAlignmentTypeValues::Justified
    | a::TextAlignmentTypeValues::JustifiedLow
    | a::TextAlignmentTypeValues::Distributed
    | a::TextAlignmentTypeValues::ThaiDistributed => paragraph_x,
  }
}

#[derive(Clone, Copy)]
struct TextItemPlacement {
  x_pt: f32,
  y_pt: f32,
  line_height_pt: f32,
  rotation_center_pt: Option<(f32, f32)>,
}

fn push_text_item(
  items: &mut Vec<PageItem>,
  placement: TextItemPlacement,
  text: String,
  style: TextStyle,
  hyperlink_url: Option<String>,
) {
  // A hyperlink is one semantic PDF link span even when its DrawingML text is
  // split across several same-style runs. Let the PDF adapter coalesce those
  // runs; preserving each whitespace-only hyperlink run independently makes
  // extraction and link text ordering diverge from the visible line.
  let preserve_text_portion = hyperlink_url.is_none();
  items.push(PageItem::Text(TextItem {
    x_pt: placement.x_pt,
    y_pt: placement.y_pt,
    line_height_pt: placement.line_height_pt,
    text,
    style,
    rotation_center_pt: placement.rotation_center_pt,
    hyperlink_url,
    form_widget_id: None,
    paragraph_bidi: false,
    // DrawingML run boundaries are layout boundaries in PowerPoint's PDF
    // output, even when adjacent runs share formatting. Preserve them so the
    // PDF adapter does not reshape across rPr/field boundaries and introduce
    // cross-run kerning or cumulative positioning drift.
    preserve_text_portion,
    pdf_text_segmentation: PdfTextSegmentation::Line,
  }));
}

fn push_symbol_split_text_items(
  items: &mut Vec<PageItem>,
  mut placement: TextItemPlacement,
  text: &str,
  style: &TextStyle,
  hyperlink_url: Option<String>,
  text_metrics: &mut TextMetrics,
) {
  let Some(symbol_font) = style.symbol_font_family.as_deref() else {
    push_text_item(
      items,
      placement,
      text.to_string(),
      style.clone(),
      hyperlink_url,
    );
    return;
  };

  let mut current = String::new();
  let mut current_symbol = None;
  for ch in text.chars() {
    let is_symbol = is_drawingml_symbol_char(ch);
    if current_symbol == Some(is_symbol) || current_symbol.is_none() {
      current_symbol = Some(is_symbol);
      current.push(ch);
      continue;
    }
    placement.x_pt = push_text_segment(
      items,
      placement,
      &current,
      TextSegmentStyle {
        style,
        use_symbol_font: current_symbol == Some(true),
        symbol_font,
      },
      hyperlink_url.clone(),
      text_metrics,
    );
    current.clear();
    current_symbol = Some(is_symbol);
    current.push(ch);
  }
  if !current.is_empty() {
    push_text_segment(
      items,
      placement,
      &current,
      TextSegmentStyle {
        style,
        use_symbol_font: current_symbol == Some(true),
        symbol_font,
      },
      hyperlink_url,
      text_metrics,
    );
  }
}

#[derive(Clone, Copy)]
struct TextSegmentStyle<'a> {
  style: &'a TextStyle,
  use_symbol_font: bool,
  symbol_font: &'a str,
}

fn push_text_segment(
  items: &mut Vec<PageItem>,
  placement: TextItemPlacement,
  text: &str,
  segment: TextSegmentStyle<'_>,
  hyperlink_url: Option<String>,
  text_metrics: &mut TextMetrics,
) -> f32 {
  let mut segment_style = segment.style.clone();
  if segment.use_symbol_font {
    segment_style.font_family = Some(Arc::from(segment.symbol_font));
  }
  push_text_item(
    items,
    placement,
    text.to_string(),
    segment_style.clone(),
    hyperlink_url,
  );
  placement.x_pt + text_metrics.measure_text(text, &segment_style)
}

fn is_drawingml_symbol_char(ch: char) -> bool {
  // formatting only to text portions whose UTF-16 high byte is 0xf0.
  let code = ch as u32;
  (code & 0xff00) == 0xf000
}

fn bullet_graphic_item(
  bullet: &BulletDisplay,
  image_resources: Option<&HashMap<String, ImageResource>>,
  x_pt: f32,
  y_pt: f32,
  style: &TextStyle,
  shape_hyperlink_url: Option<&str>,
) -> Option<ImageItem> {
  let relationship_id = bullet.picture_relationship_id.as_deref()?;
  let resource = image_resources?.get(relationship_id)?;
  let width_pt =
    sdk_units::mm100_to_points100(i64::from(bullet.graphic_width_100mm?)) as f32 / 100.0;
  let height_pt =
    sdk_units::mm100_to_points100(i64::from(bullet.graphic_height_100mm?)) as f32 / 100.0;
  Some(ImageItem {
    x_pt,
    y_pt: y_pt + (line_height(style, 1.0) - height_pt) / 2.0,
    width_pt,
    height_pt,
    crop: ImageCrop::default(),
    clip_path: Vec::new(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    data: resource.data.clone(),
    content_type: resource.content_type.clone(),
    alt_text: None,
    hyperlink_url: shape_hyperlink_url.map(ToString::to_string),
    floating: false,
    behind_text: false,
  })
}

fn line_height(style: &TextStyle, line_scale: f32) -> f32 {
  style.font_size_pt * DEFAULT_TEXT_LINE_HEIGHT_SCALE * line_scale
}

#[derive(Clone, Copy)]
struct TextBodyHeightContext<'a> {
  import: &'a PowerPointImport,
  slide: Option<&'a SlidePersist>,
  frame: TextFrame,
  base_style: &'a TextStyle,
  font_reference: Option<&'a FontStyleReference>,
  options: &'a TextLoweringOptions,
  slide_number: i32,
}

fn estimate_wrapped_text_body_height(
  context: TextBodyHeightContext<'_>,
  text_body: &TextBody,
  text_metrics: &mut TextMetrics,
) -> f32 {
  let column_width = context.options.column_width(context.frame).max(1.0);
  let mut height = 0.0;
  for (paragraph_index, paragraph) in text_body.paragraphs.iter().enumerate() {
    let paragraph_style = ParagraphDisplayStyle::from_paragraph(paragraph);
    let mut paragraph_base_style = context.base_style.clone();
    paragraph_style.apply_master_default_run_style(
      context.import,
      context.slide,
      &mut paragraph_base_style,
    );
    if let Some(font_reference) = context.font_reference {
      apply_font_reference_text_style(
        context.import,
        context.slide,
        font_reference,
        &mut paragraph_base_style,
      );
    }
    paragraph_style.apply_local_default_run_style(
      context.import,
      context.slide,
      &mut paragraph_base_style,
    );
    apply_text_scale(&mut paragraph_base_style, context.options);
    if paragraph_index > 0 || context.options.use_first_last_paragraph_spacing {
      height += paragraph_style
        .space_before
        .points(paragraph_base_style.font_size_pt);
    }
    for runs in paragraph.runs.split(|run| run.kind == TextRunKind::Break) {
      let lines = layout_text_lines(
        TextLineLayoutContext {
          import: context.import,
          slide: context.slide,
          base_style: &paragraph_base_style,
          options: context.options,
          column_width,
          slide_number: context.slide_number,
        },
        runs,
        text_metrics,
      );
      for line in lines {
        let line_height = line.runs.iter().fold(
          paragraph_style.line_height(&paragraph_base_style, context.options),
          |height, run| height.max(paragraph_style.line_height(&run.style, context.options)),
        );
        height += line_height;
      }
    }
    if paragraph_index + 1 < text_body.paragraphs.len()
      || context.options.use_first_last_paragraph_spacing
    {
      height += paragraph_style
        .space_after
        .points(paragraph_base_style.font_size_pt);
    }
  }
  height
}

fn presentation_slide_number(import: &PowerPointImport, page_index: usize) -> i32 {
  import
    .first_slide_number
    .saturating_add(i32::try_from(page_index).unwrap_or(i32::MAX))
}

fn presentation_field_text(run: &TextRun, slide_number: i32) -> Cow<'_, str> {
  if run.kind == TextRunKind::Field && run.field_type.as_deref() == Some("slidenum") {
    Cow::Owned(slide_number.to_string())
  } else {
    Cow::Borrowed(&run.text)
  }
}

fn advance_text_column_if_needed(
  cursor: &mut TextCursor,
  frame: TextFrame,
  options: TextLoweringOptions,
) {
  if options.column_count <= 1 || cursor.y_pt <= frame.y_pt + frame.height_pt {
    return;
  }
  if cursor.column_index + 1 >= options.column_count {
    return;
  }
  cursor.column_index += 1;
  cursor.y_pt = frame.y_pt;
}

fn push_math_ole_preview_item(
  items: &mut Vec<PageItem>,
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
) {
  if width_pt <= f32::EPSILON || height_pt <= f32::EPSILON {
    return;
  }
  let Some(data) = transparent_png_1x1() else {
    return;
  };
  items.push(PageItem::Image(ImageItem {
    x_pt,
    y_pt,
    width_pt,
    height_pt,
    crop: ImageCrop::default(),
    clip_path: Vec::new(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    data,
    content_type: Some("image/png".to_string()),
    alt_text: None,
    hyperlink_url: None,
    floating: false,
    behind_text: false,
  }));
}

fn transparent_png_1x1() -> Option<Arc<[u8]>> {
  let mut output = Vec::new();
  let encoder = PngEncoder::new(Cursor::new(&mut output));
  encoder
    .write_image(&[0, 0, 0, 0], 1, 1, ColorType::Rgba8.into())
    .ok()?;
  Some(output.into())
}

#[derive(Clone, Debug)]
struct ParagraphDisplayStyle {
  left_margin_pt: f32,
  indent_pt: f32,
  alignment: a::TextAlignmentTypeValues,
  line_spacing: ParagraphLineSpacing,
  space_before: ParagraphSpacing,
  space_after: ParagraphSpacing,
  bullet: BulletDisplay,
  master_default_run_properties: Option<Box<a::DefaultRunProperties>>,
  text_default_run_properties: Option<Box<a::DefaultRunProperties>>,
  direct_default_run_properties: Option<Box<a::DefaultRunProperties>>,
}

#[derive(Clone, Debug, Default)]
struct BulletDisplay {
  label: Option<String>,
  auto_number: Option<AutoNumberBullet>,
  font: Option<String>,
  color: Option<Color>,
  picture_relationship_id: Option<String>,
  size: BulletSize,
  graphic_width_100mm: Option<i32>,
  graphic_height_100mm: Option<i32>,
  disabled: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AutoNumberBullet {
  scheme: a::TextAutoNumberSchemeValues,
  start_at: Option<i32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AutoNumberCounter {
  scheme: a::TextAutoNumberSchemeValues,
  declared_start: Option<i32>,
  value: i32,
}

#[derive(Default)]
struct AutoNumberingState {
  levels: [Option<AutoNumberCounter>; 9],
}

impl AutoNumberingState {
  fn resolve(&mut self, paragraph: &TextParagraph, bullet: &mut BulletDisplay) {
    if !paragraph_has_printable_run(paragraph) {
      return;
    }

    let level = usize::from(paragraph.level.unwrap_or(0).min(8));
    let Some(auto_number) = bullet.auto_number else {
      self.levels[level] = None;
      return;
    };

    // ECMA-376 Part 1, 21.1.2.4.1: automatic numbering is based on
    // buAutoNum attributes and paragraph level. Its level 0/1/0 example
    // advances as 1, 1, 2, so each level owns an independent sequence.
    let previous = self.levels[level];
    let continues_sequence = previous.is_some_and(|counter| {
      counter.scheme == auto_number.scheme
        && auto_number
          .start_at
          .is_none_or(|start| counter.declared_start == Some(start))
    });
    let value = previous.map_or_else(
      || auto_number.start_at.unwrap_or(1),
      |counter| {
        if continues_sequence {
          counter.value.saturating_add(1)
        } else {
          auto_number.start_at.unwrap_or(1)
        }
      },
    );
    self.levels[level] = Some(AutoNumberCounter {
      scheme: auto_number.scheme,
      declared_start: if continues_sequence {
        previous.and_then(|counter| counter.declared_start)
      } else {
        auto_number.start_at
      },
      value,
    });
    self.levels[level + 1..].fill(None);
    bullet.label = Some(format_auto_number(auto_number.scheme, value));
  }
}

fn paragraph_has_printable_run(paragraph: &TextParagraph) -> bool {
  paragraph.runs.iter().any(|run| {
    !run.text.is_empty() && !matches!(run.kind, TextRunKind::Break | TextRunKind::Placeholder)
  })
}

fn format_auto_number(scheme: a::TextAutoNumberSchemeValues, value: i32) -> String {
  use a::TextAutoNumberSchemeValues as Scheme;

  let value = value.max(1);
  match scheme {
    Scheme::AlphaLowerCharacterParenBoth => format!("({})", alpha_number(value, false)),
    Scheme::AlphaUpperCharacterParenBoth => format!("({})", alpha_number(value, true)),
    Scheme::AlphaLowerCharacterParenR => format!("{})", alpha_number(value, false)),
    Scheme::AlphaUpperCharacterParenR => format!("{})", alpha_number(value, true)),
    Scheme::AlphaLowerCharacterPeriod => format!("{}.", alpha_number(value, false)),
    Scheme::AlphaUpperCharacterPeriod => format!("{}.", alpha_number(value, true)),
    Scheme::ArabicParenBoth => format!("({value})"),
    Scheme::ArabicParenR => format!("{value})"),
    Scheme::ArabicPeriod => format!("{value}."),
    Scheme::ArabicPlain => value.to_string(),
    Scheme::RomanLowerCharacterParenBoth => format!("({})", roman_number(value, false)),
    Scheme::RomanUpperCharacterParenBoth => format!("({})", roman_number(value, true)),
    Scheme::RomanLowerCharacterParenR => format!("{})", roman_number(value, false)),
    Scheme::RomanUpperCharacterParenR => format!("{})", roman_number(value, true)),
    Scheme::RomanLowerCharacterPeriod => format!("{}.", roman_number(value, false)),
    Scheme::RomanUpperCharacterPeriod => format!("{}.", roman_number(value, true)),
    Scheme::EastAsianJapaneseKoreanPeriod => format!("{}.", east_asian_number(value)),
    Scheme::EastAsianJapaneseKoreanPlain => east_asian_number(value),
    // The remaining schemes require locale-specific numeral systems or
    // symbol-font mappings. Keep the prior visible fallback until each has
    // equivalent source evidence and corpus coverage.
    _ => format!("{value}."),
  }
}

fn alpha_number(value: i32, uppercase: bool) -> String {
  // ECMA-376 Part 1, 21.1.2.4.1 explicitly maps 27 to "aa" and 53 to
  // "aaa": PowerPoint repeats a letter rather than using spreadsheet-style
  // base-26 lettering.
  let zero_based = value.max(1) as usize - 1;
  let character = if uppercase { b'A' } else { b'a' } + (zero_based % 26) as u8;
  std::iter::repeat_n(char::from(character), zero_based / 26 + 1).collect()
}

fn roman_number(value: i32, uppercase: bool) -> String {
  const TOKENS: &[(i32, &str)] = &[
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
  ];
  let mut remainder = value.max(1);
  let mut result = String::new();
  for &(unit, token) in TOKENS {
    while remainder >= unit {
      result.push_str(token);
      remainder -= unit;
    }
  }
  if uppercase {
    result
  } else {
    result.to_lowercase()
  }
}

fn east_asian_number(value: i32) -> String {
  const DIGITS: [&str; 10] = ["零", "一", "二", "三", "四", "五", "六", "七", "八", "九"];
  const UNITS: [&str; 4] = ["", "十", "百", "千"];

  let value = value.max(1);
  if value >= 10_000 {
    return value.to_string();
  }
  let mut result = String::new();
  let mut emitted = false;
  let mut pending_zero = false;
  for position in (0..4).rev() {
    let divisor = 10_i32.pow(position as u32);
    let digit = (value / divisor) % 10;
    if digit == 0 {
      pending_zero = emitted;
      continue;
    }
    if pending_zero {
      result.push_str(DIGITS[0]);
      pending_zero = false;
    }
    if !(digit == 1 && position == 1 && !emitted) {
      result.push_str(DIGITS[digit as usize]);
    }
    result.push_str(UNITS[position]);
    emitted = true;
  }
  result
}

#[derive(Clone, Copy, Debug, Default)]
enum BulletSize {
  #[default]
  FollowText,
  Percent(f32),
  Points100(i32),
}

fn apply_character_bullet_size(style: &mut TextStyle, size: BulletSize) {
  match size {
    BulletSize::FollowText => {}
    BulletSize::Percent(percent) => style.font_size_pt *= percent / 100.0,
    BulletSize::Points100(points100) => {
      style.font_size_pt = sdk_units::points100_to_points(points100) as f32;
    }
  }
}

#[derive(Clone, Copy, Debug)]
enum ParagraphLineSpacing {
  Default,
  Percent(f32),
  Points(f32),
}

#[derive(Clone, Copy, Debug, Default)]
enum ParagraphSpacing {
  #[default]
  Zero,
  Percent(f32),
  Points(f32),
}

impl ParagraphSpacing {
  fn points(self, font_size_pt: f32) -> f32 {
    match self {
      Self::Zero => 0.0,
      Self::Percent(ratio) => font_size_pt * ratio,
      Self::Points(points) => points,
    }
  }
}

impl Default for ParagraphDisplayStyle {
  fn default() -> Self {
    Self {
      left_margin_pt: 0.0,
      indent_pt: 0.0,
      alignment: a::TextAlignmentTypeValues::Left,
      line_spacing: ParagraphLineSpacing::Default,
      space_before: ParagraphSpacing::Zero,
      space_after: ParagraphSpacing::Zero,
      bullet: BulletDisplay::default(),
      master_default_run_properties: None,
      text_default_run_properties: None,
      direct_default_run_properties: None,
    }
  }
}

fn text_list_default_run_properties(
  style: &TextListParagraphStyle,
) -> Option<Box<a::DefaultRunProperties>> {
  match style {
    TextListParagraphStyle::Default(properties) => properties.default_run_properties.clone(),
    TextListParagraphStyle::Level(level) => match &level.paragraph_properties {
      TextListLevelParagraphProperties::Level1(properties) => {
        properties.default_run_properties.clone()
      }
      TextListLevelParagraphProperties::Level2(properties) => {
        properties.default_run_properties.clone()
      }
      TextListLevelParagraphProperties::Level3(properties) => {
        properties.default_run_properties.clone()
      }
      TextListLevelParagraphProperties::Level4(properties) => {
        properties.default_run_properties.clone()
      }
      TextListLevelParagraphProperties::Level5(properties) => {
        properties.default_run_properties.clone()
      }
      TextListLevelParagraphProperties::Level6(properties) => {
        properties.default_run_properties.clone()
      }
      TextListLevelParagraphProperties::Level7(properties) => {
        properties.default_run_properties.clone()
      }
      TextListLevelParagraphProperties::Level8(properties) => {
        properties.default_run_properties.clone()
      }
      TextListLevelParagraphProperties::Level9(properties) => {
        properties.default_run_properties.clone()
      }
    },
  }
}

impl ParagraphDisplayStyle {
  fn from_paragraph(paragraph: &TextParagraph) -> Self {
    let mut style = Self::default();
    if let Some(master_style) = &paragraph.master_paragraph_style {
      style.apply_text_list_style(master_style);
      style.master_default_run_properties = text_list_default_run_properties(master_style);
    }
    if let Some(text_style) = &paragraph.text_paragraph_style {
      style.apply_text_list_style(text_style);
      style.text_default_run_properties = text_list_default_run_properties(text_style);
    }
    if let Some(properties) = paragraph.paragraph_properties.as_deref() {
      if let Some(left_margin) = properties.left_margin {
        style.left_margin_pt = units::emu_to_points(i64::from(left_margin));
      }
      if let Some(indent) = properties.indent {
        style.indent_pt = units::emu_to_points(i64::from(indent));
      }
      if let Some(default_run_properties) = &properties.default_run_properties {
        style.direct_default_run_properties = Some(default_run_properties.clone());
      }
      if let Some(alignment) = properties.alignment {
        style.alignment = alignment;
      }
      if let Some(line_spacing) = properties.line_spacing.as_deref() {
        style.line_spacing = paragraph_line_spacing(line_spacing);
      }
      if let Some(space_before) = properties.space_before.as_deref() {
        style.space_before = paragraph_space_before(space_before);
      }
      if let Some(space_after) = properties.space_after.as_deref() {
        style.space_after = paragraph_space_after(space_after);
      }
      style.apply_bullet_size(&properties.paragraph_properties_choice2);
      style.bullet.apply_color(paragraph_properties_bullet_color(
        &properties.paragraph_properties_choice1,
      ));
      style.bullet.apply_font(paragraph_properties_bullet_font(
        &properties.paragraph_properties_choice3,
      ));
      style.bullet.apply_kind(paragraph_properties_bullet(
        &properties.paragraph_properties_choice4,
      ));
    }
    style
  }

  fn apply_text_list_style(&mut self, style: &TextListParagraphStyle) {
    match style {
      TextListParagraphStyle::Default(properties) => {
        self.left_margin_pt = properties
          .left_margin
          .map(|value| units::emu_to_points(i64::from(value)))
          .unwrap_or(self.left_margin_pt);
        self.indent_pt = properties
          .indent
          .map(|value| units::emu_to_points(i64::from(value)))
          .unwrap_or(self.indent_pt);
        self.alignment = properties.alignment.unwrap_or(self.alignment);
        if let Some(line_spacing) = properties.line_spacing.as_deref() {
          self.line_spacing = paragraph_line_spacing(line_spacing);
        }
        if let Some(space_before) = properties.space_before.as_deref() {
          self.space_before = paragraph_space_before(space_before);
        }
        if let Some(space_after) = properties.space_after.as_deref() {
          self.space_after = paragraph_space_after(space_after);
        }
        self.apply_default_bullet_size(&properties.default_paragraph_properties_choice2);
        self
          .bullet
          .apply_color(default_paragraph_properties_bullet_color(
            &properties.default_paragraph_properties_choice1,
          ));
        self
          .bullet
          .apply_font(default_paragraph_properties_bullet_font(
            &properties.default_paragraph_properties_choice3,
          ));
        self.bullet.apply_kind(default_paragraph_properties_bullet(
          &properties.default_paragraph_properties_choice4,
        ));
      }
      TextListParagraphStyle::Level(level) => {
        self.apply_level_bullet_size(&level.paragraph_properties);
        self
          .bullet
          .apply_color(level_paragraph_properties_bullet_color(
            &level.paragraph_properties,
          ));
        self
          .bullet
          .apply_font(level_paragraph_properties_bullet_font(
            &level.paragraph_properties,
          ));
        self.apply_level_style(&level.paragraph_properties);
      }
    }
  }

  fn apply_level_style(&mut self, properties: &TextListLevelParagraphProperties) {
    macro_rules! apply_level {
      ($properties:expr, $bullet_fn:ident, $choice:ident) => {{
        self.left_margin_pt = $properties
          .left_margin
          .map(|value| units::emu_to_points(i64::from(value)))
          .unwrap_or(self.left_margin_pt);
        self.indent_pt = $properties
          .indent
          .map(|value| units::emu_to_points(i64::from(value)))
          .unwrap_or(self.indent_pt);
        self.alignment = $properties.alignment.unwrap_or(self.alignment);
        if let Some(line_spacing) = $properties.line_spacing.as_deref() {
          self.line_spacing = paragraph_line_spacing(line_spacing);
        }
        if let Some(space_before) = $properties.space_before.as_deref() {
          self.space_before = paragraph_space_before(space_before);
        }
        if let Some(space_after) = $properties.space_after.as_deref() {
          self.space_after = paragraph_space_after(space_after);
        }
        self.bullet.apply_kind($bullet_fn(&$properties.$choice));
      }};
    }

    match properties {
      TextListLevelParagraphProperties::Level1(properties) => {
        apply_level!(
          properties,
          level1_paragraph_properties_bullet,
          level1_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level2(properties) => {
        apply_level!(
          properties,
          level2_paragraph_properties_bullet,
          level2_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level3(properties) => {
        apply_level!(
          properties,
          level3_paragraph_properties_bullet,
          level3_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level4(properties) => {
        apply_level!(
          properties,
          level4_paragraph_properties_bullet,
          level4_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level5(properties) => {
        apply_level!(
          properties,
          level5_paragraph_properties_bullet,
          level5_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level6(properties) => {
        apply_level!(
          properties,
          level6_paragraph_properties_bullet,
          level6_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level7(properties) => {
        apply_level!(
          properties,
          level7_paragraph_properties_bullet,
          level7_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level8(properties) => {
        apply_level!(
          properties,
          level8_paragraph_properties_bullet,
          level8_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level9(properties) => {
        apply_level!(
          properties,
          level9_paragraph_properties_bullet,
          level9_paragraph_properties_choice4
        )
      }
    }
  }

  fn bullet(&self, paragraph: &TextParagraph) -> BulletDisplay {
    // Empty text bodies and placeholder prompts do not produce a bullet in
    // printed/slideshow output. LibreOffice handles both through its empty
    // text-body path; placeholder prompts additionally remain edit-view-only.
    let has_printable_run = paragraph.runs.iter().any(|run| {
      !run.text.is_empty() && !matches!(run.kind, TextRunKind::Break | TextRunKind::Placeholder)
    });
    if !has_printable_run {
      return BulletDisplay::default();
    }
    let mut bullet = self.bullet.clone();
    if bullet.disabled {
      return BulletDisplay::default();
    }
    if bullet.label.is_none() && bullet.auto_number.is_none() {
      bullet.label = paragraph
        .level
        .filter(|level| *level > 0)
        .map(|_| "\u{2022}".to_string());
    }
    bullet
  }

  fn apply_master_default_run_style(
    &self,
    import: &PowerPointImport,
    slide: Option<&SlidePersist>,
    style: &mut TextStyle,
  ) {
    if let Some(properties) = &self.master_default_run_properties {
      apply_default_run_properties(import, slide, properties, style);
    }
  }

  fn apply_local_default_run_style(
    &self,
    import: &PowerPointImport,
    slide: Option<&SlidePersist>,
    style: &mut TextStyle,
  ) {
    for properties in [
      self.text_default_run_properties.as_deref(),
      self.direct_default_run_properties.as_deref(),
    ]
    .into_iter()
    .flatten()
    {
      apply_default_run_properties(import, slide, properties, style);
    }
  }

  fn line_height(&self, style: &TextStyle, options: &TextLoweringOptions) -> f32 {
    self.line_height_with_scale(style, options.line_scale)
  }

  fn line_height_with_scale(&self, style: &TextStyle, line_scale: f32) -> f32 {
    let natural_height = line_height(style, 1.0);
    match self.line_spacing {
      ParagraphLineSpacing::Default => line_height(style, line_scale),
      ParagraphLineSpacing::Percent(ratio) => natural_height * ratio * line_scale,
      ParagraphLineSpacing::Points(points) => points * line_scale,
    }
  }

  fn apply_bullet_size(&mut self, choice: &Option<a::ParagraphPropertiesChoice2>) {
    if let Some(size) = paragraph_properties_bullet_size(choice) {
      self.bullet.size = size;
    }
  }

  fn apply_default_bullet_size(&mut self, choice: &Option<a::DefaultParagraphPropertiesChoice2>) {
    if let Some(size) = default_paragraph_properties_bullet_size(choice) {
      self.bullet.size = size;
    }
  }

  fn apply_level_bullet_size(&mut self, properties: &TextListLevelParagraphProperties) {
    if let Some(size) = level_paragraph_properties_bullet_size(properties) {
      self.bullet.size = size;
    }
  }
}

fn paragraph_line_spacing(line_spacing: &a::LineSpacing) -> ParagraphLineSpacing {
  match line_spacing.line_spacing_choice.as_ref() {
    Some(a::LineSpacingChoice::SpacingPercent(spacing)) => {
      ParagraphLineSpacing::Percent(spacing.val.as_ratio() as f32)
    }
    Some(a::LineSpacingChoice::SpacingPoints(spacing)) => {
      ParagraphLineSpacing::Points(sdk_units::points100_to_points(spacing.val) as f32)
    }
    None => ParagraphLineSpacing::Default,
  }
}

fn paragraph_space_before(space: &a::SpaceBefore) -> ParagraphSpacing {
  match space.space_before_choice.as_ref() {
    Some(a::SpaceBeforeChoice::SpacingPercent(spacing)) => {
      ParagraphSpacing::Percent(spacing.val.as_ratio() as f32)
    }
    Some(a::SpaceBeforeChoice::SpacingPoints(spacing)) => {
      ParagraphSpacing::Points(sdk_units::points100_to_points(spacing.val) as f32)
    }
    None => ParagraphSpacing::Zero,
  }
}

fn paragraph_space_after(space: &a::SpaceAfter) -> ParagraphSpacing {
  match space.space_after_choice.as_ref() {
    Some(a::SpaceAfterChoice::SpacingPercent(spacing)) => {
      ParagraphSpacing::Percent(spacing.val.as_ratio() as f32)
    }
    Some(a::SpaceAfterChoice::SpacingPoints(spacing)) => {
      ParagraphSpacing::Points(sdk_units::points100_to_points(spacing.val) as f32)
    }
    None => ParagraphSpacing::Zero,
  }
}

fn paragraph_properties_bullet_size(
  choice: &Option<a::ParagraphPropertiesChoice2>,
) -> Option<BulletSize> {
  match choice {
    Some(a::ParagraphPropertiesChoice2::BulletSizeText) => Some(BulletSize::FollowText),
    Some(a::ParagraphPropertiesChoice2::BulletSizePercentage(size)) => Some(BulletSize::Percent(
      size.val.as_drawingml_percent() as f32 / 1000.0,
    )),
    Some(a::ParagraphPropertiesChoice2::BulletSizePoints(size)) => {
      Some(BulletSize::Points100(size.val))
    }
    None => None,
  }
}

#[derive(Clone, Debug)]
enum BulletColorOverride {
  Unspecified,
  FollowText,
  Color(Color),
}

fn bullet_color(color: &a::BulletColor) -> BulletColorOverride {
  color
    .bullet_color_choice
    .as_ref()
    .and_then(Color::from_bullet_color_choice)
    .map(BulletColorOverride::Color)
    .unwrap_or(BulletColorOverride::Unspecified)
}

fn paragraph_properties_bullet_color(
  choice: &Option<a::ParagraphPropertiesChoice>,
) -> BulletColorOverride {
  match choice {
    Some(a::ParagraphPropertiesChoice::BulletColorText) => BulletColorOverride::FollowText,
    Some(a::ParagraphPropertiesChoice::BulletColor(color)) => bullet_color(color),
    None => BulletColorOverride::Unspecified,
  }
}

fn default_paragraph_properties_bullet_color(
  choice: &Option<a::DefaultParagraphPropertiesChoice>,
) -> BulletColorOverride {
  match choice {
    Some(a::DefaultParagraphPropertiesChoice::BulletColorText) => BulletColorOverride::FollowText,
    Some(a::DefaultParagraphPropertiesChoice::BulletColor(color)) => bullet_color(color),
    None => BulletColorOverride::Unspecified,
  }
}

fn default_paragraph_properties_bullet_size(
  choice: &Option<a::DefaultParagraphPropertiesChoice2>,
) -> Option<BulletSize> {
  match choice {
    Some(a::DefaultParagraphPropertiesChoice2::BulletSizeText) => Some(BulletSize::FollowText),
    Some(a::DefaultParagraphPropertiesChoice2::BulletSizePercentage(size)) => Some(
      BulletSize::Percent(size.val.as_drawingml_percent() as f32 / 1000.0),
    ),
    Some(a::DefaultParagraphPropertiesChoice2::BulletSizePoints(size)) => {
      Some(BulletSize::Points100(size.val))
    }
    None => None,
  }
}

fn paragraph_properties_bullet_font(
  choice: &Option<a::ParagraphPropertiesChoice3>,
) -> BulletFontOverride {
  match choice {
    Some(a::ParagraphPropertiesChoice3::BulletFont(font)) => font
      .typeface
      .clone()
      .map(BulletFontOverride::Font)
      .unwrap_or(BulletFontOverride::Unspecified),
    Some(a::ParagraphPropertiesChoice3::BulletFontText) => BulletFontOverride::FollowText,
    None => BulletFontOverride::Unspecified,
  }
}

fn default_paragraph_properties_bullet_font(
  choice: &Option<a::DefaultParagraphPropertiesChoice3>,
) -> BulletFontOverride {
  match choice {
    Some(a::DefaultParagraphPropertiesChoice3::BulletFont(font)) => font
      .typeface
      .clone()
      .map(BulletFontOverride::Font)
      .unwrap_or(BulletFontOverride::Unspecified),
    Some(a::DefaultParagraphPropertiesChoice3::BulletFontText) => BulletFontOverride::FollowText,
    None => BulletFontOverride::Unspecified,
  }
}

#[derive(Clone, Debug)]
enum BulletFontOverride {
  Unspecified,
  FollowText,
  Font(String),
}

trait LevelBulletSizeChoice {
  fn bullet_size(&self) -> Option<BulletSize>;
}

trait LevelBulletFontChoice {
  fn bullet_font(&self) -> BulletFontOverride;
}

trait LevelBulletColorChoice {
  fn bullet_color(&self) -> BulletColorOverride;
}

macro_rules! impl_level_bullet_size_choice {
  ($ty:ty) => {
    impl LevelBulletSizeChoice for $ty {
      fn bullet_size(&self) -> Option<BulletSize> {
        match self {
          Self::BulletSizeText => Some(BulletSize::FollowText),
          Self::BulletSizePercentage(size) => Some(BulletSize::Percent(
            size.val.as_drawingml_percent() as f32 / 1000.0,
          )),
          Self::BulletSizePoints(size) => Some(BulletSize::Points100(size.val)),
        }
      }
    }
  };
}

macro_rules! impl_level_bullet_font_choice {
  ($ty:ty) => {
    impl LevelBulletFontChoice for $ty {
      fn bullet_font(&self) -> BulletFontOverride {
        match self {
          Self::BulletFont(font) => font
            .typeface
            .clone()
            .map(BulletFontOverride::Font)
            .unwrap_or(BulletFontOverride::Unspecified),
          Self::BulletFontText => BulletFontOverride::FollowText,
        }
      }
    }
  };
}

macro_rules! impl_level_bullet_color_choice {
  ($ty:ty) => {
    impl LevelBulletColorChoice for $ty {
      fn bullet_color(&self) -> BulletColorOverride {
        match self {
          Self::BulletColorText => BulletColorOverride::FollowText,
          Self::BulletColor(color) => bullet_color(color),
        }
      }
    }
  };
}

impl_level_bullet_size_choice!(a::Level1ParagraphPropertiesChoice2);
impl_level_bullet_size_choice!(a::Level2ParagraphPropertiesChoice2);
impl_level_bullet_size_choice!(a::Level3ParagraphPropertiesChoice2);
impl_level_bullet_size_choice!(a::Level4ParagraphPropertiesChoice2);
impl_level_bullet_size_choice!(a::Level5ParagraphPropertiesChoice2);
impl_level_bullet_size_choice!(a::Level6ParagraphPropertiesChoice2);
impl_level_bullet_size_choice!(a::Level7ParagraphPropertiesChoice2);
impl_level_bullet_size_choice!(a::Level8ParagraphPropertiesChoice2);
impl_level_bullet_size_choice!(a::Level9ParagraphPropertiesChoice2);

impl_level_bullet_font_choice!(a::Level1ParagraphPropertiesChoice3);
impl_level_bullet_font_choice!(a::Level2ParagraphPropertiesChoice3);
impl_level_bullet_font_choice!(a::Level3ParagraphPropertiesChoice3);
impl_level_bullet_font_choice!(a::Level4ParagraphPropertiesChoice3);
impl_level_bullet_font_choice!(a::Level5ParagraphPropertiesChoice3);
impl_level_bullet_font_choice!(a::Level6ParagraphPropertiesChoice3);
impl_level_bullet_font_choice!(a::Level7ParagraphPropertiesChoice3);
impl_level_bullet_font_choice!(a::Level8ParagraphPropertiesChoice3);
impl_level_bullet_font_choice!(a::Level9ParagraphPropertiesChoice3);

impl_level_bullet_color_choice!(a::Level1ParagraphPropertiesChoice);
impl_level_bullet_color_choice!(a::Level2ParagraphPropertiesChoice);
impl_level_bullet_color_choice!(a::Level3ParagraphPropertiesChoice);
impl_level_bullet_color_choice!(a::Level4ParagraphPropertiesChoice);
impl_level_bullet_color_choice!(a::Level5ParagraphPropertiesChoice);
impl_level_bullet_color_choice!(a::Level6ParagraphPropertiesChoice);
impl_level_bullet_color_choice!(a::Level7ParagraphPropertiesChoice);
impl_level_bullet_color_choice!(a::Level8ParagraphPropertiesChoice);
impl_level_bullet_color_choice!(a::Level9ParagraphPropertiesChoice);

fn level_paragraph_properties_bullet_color(
  properties: &TextListLevelParagraphProperties,
) -> BulletColorOverride {
  match properties {
    TextListLevelParagraphProperties::Level1(properties) => properties
      .level1_paragraph_properties_choice1
      .as_ref()
      .map(LevelBulletColorChoice::bullet_color),
    TextListLevelParagraphProperties::Level2(properties) => properties
      .level2_paragraph_properties_choice1
      .as_ref()
      .map(LevelBulletColorChoice::bullet_color),
    TextListLevelParagraphProperties::Level3(properties) => properties
      .level3_paragraph_properties_choice1
      .as_ref()
      .map(LevelBulletColorChoice::bullet_color),
    TextListLevelParagraphProperties::Level4(properties) => properties
      .level4_paragraph_properties_choice1
      .as_ref()
      .map(LevelBulletColorChoice::bullet_color),
    TextListLevelParagraphProperties::Level5(properties) => properties
      .level5_paragraph_properties_choice1
      .as_ref()
      .map(LevelBulletColorChoice::bullet_color),
    TextListLevelParagraphProperties::Level6(properties) => properties
      .level6_paragraph_properties_choice1
      .as_ref()
      .map(LevelBulletColorChoice::bullet_color),
    TextListLevelParagraphProperties::Level7(properties) => properties
      .level7_paragraph_properties_choice1
      .as_ref()
      .map(LevelBulletColorChoice::bullet_color),
    TextListLevelParagraphProperties::Level8(properties) => properties
      .level8_paragraph_properties_choice1
      .as_ref()
      .map(LevelBulletColorChoice::bullet_color),
    TextListLevelParagraphProperties::Level9(properties) => properties
      .level9_paragraph_properties_choice1
      .as_ref()
      .map(LevelBulletColorChoice::bullet_color),
  }
  .unwrap_or(BulletColorOverride::Unspecified)
}

fn level_paragraph_properties_bullet_size(
  properties: &TextListLevelParagraphProperties,
) -> Option<BulletSize> {
  match properties {
    TextListLevelParagraphProperties::Level1(properties) => properties
      .level1_paragraph_properties_choice2
      .as_ref()
      .and_then(LevelBulletSizeChoice::bullet_size),
    TextListLevelParagraphProperties::Level2(properties) => properties
      .level2_paragraph_properties_choice2
      .as_ref()
      .and_then(LevelBulletSizeChoice::bullet_size),
    TextListLevelParagraphProperties::Level3(properties) => properties
      .level3_paragraph_properties_choice2
      .as_ref()
      .and_then(LevelBulletSizeChoice::bullet_size),
    TextListLevelParagraphProperties::Level4(properties) => properties
      .level4_paragraph_properties_choice2
      .as_ref()
      .and_then(LevelBulletSizeChoice::bullet_size),
    TextListLevelParagraphProperties::Level5(properties) => properties
      .level5_paragraph_properties_choice2
      .as_ref()
      .and_then(LevelBulletSizeChoice::bullet_size),
    TextListLevelParagraphProperties::Level6(properties) => properties
      .level6_paragraph_properties_choice2
      .as_ref()
      .and_then(LevelBulletSizeChoice::bullet_size),
    TextListLevelParagraphProperties::Level7(properties) => properties
      .level7_paragraph_properties_choice2
      .as_ref()
      .and_then(LevelBulletSizeChoice::bullet_size),
    TextListLevelParagraphProperties::Level8(properties) => properties
      .level8_paragraph_properties_choice2
      .as_ref()
      .and_then(LevelBulletSizeChoice::bullet_size),
    TextListLevelParagraphProperties::Level9(properties) => properties
      .level9_paragraph_properties_choice2
      .as_ref()
      .and_then(LevelBulletSizeChoice::bullet_size),
  }
}

fn level_paragraph_properties_bullet_font(
  properties: &TextListLevelParagraphProperties,
) -> BulletFontOverride {
  match properties {
    TextListLevelParagraphProperties::Level1(properties) => properties
      .level1_paragraph_properties_choice3
      .as_ref()
      .map(LevelBulletFontChoice::bullet_font),
    TextListLevelParagraphProperties::Level2(properties) => properties
      .level2_paragraph_properties_choice3
      .as_ref()
      .map(LevelBulletFontChoice::bullet_font),
    TextListLevelParagraphProperties::Level3(properties) => properties
      .level3_paragraph_properties_choice3
      .as_ref()
      .map(LevelBulletFontChoice::bullet_font),
    TextListLevelParagraphProperties::Level4(properties) => properties
      .level4_paragraph_properties_choice3
      .as_ref()
      .map(LevelBulletFontChoice::bullet_font),
    TextListLevelParagraphProperties::Level5(properties) => properties
      .level5_paragraph_properties_choice3
      .as_ref()
      .map(LevelBulletFontChoice::bullet_font),
    TextListLevelParagraphProperties::Level6(properties) => properties
      .level6_paragraph_properties_choice3
      .as_ref()
      .map(LevelBulletFontChoice::bullet_font),
    TextListLevelParagraphProperties::Level7(properties) => properties
      .level7_paragraph_properties_choice3
      .as_ref()
      .map(LevelBulletFontChoice::bullet_font),
    TextListLevelParagraphProperties::Level8(properties) => properties
      .level8_paragraph_properties_choice3
      .as_ref()
      .map(LevelBulletFontChoice::bullet_font),
    TextListLevelParagraphProperties::Level9(properties) => properties
      .level9_paragraph_properties_choice3
      .as_ref()
      .map(LevelBulletFontChoice::bullet_font),
  }
  .unwrap_or(BulletFontOverride::Unspecified)
}

impl BulletDisplay {
  fn apply_font(&mut self, font: BulletFontOverride) {
    match font {
      BulletFontOverride::Unspecified => {}
      BulletFontOverride::FollowText => self.font = None,
      BulletFontOverride::Font(font) => self.font = Some(font),
    }
  }

  fn apply_color(&mut self, color: BulletColorOverride) {
    match color {
      BulletColorOverride::Unspecified => {}
      BulletColorOverride::FollowText => self.color = None,
      BulletColorOverride::Color(color) => self.color = Some(color),
    }
  }

  fn apply_kind(&mut self, kind: BulletOverride) {
    match kind {
      BulletOverride::Unspecified => {}
      BulletOverride::Disabled => {
        self.label = None;
        self.auto_number = None;
        self.picture_relationship_id = None;
        self.disabled = true;
      }
      BulletOverride::Kind(kind) => {
        self.label = kind.label;
        self.auto_number = kind.auto_number;
        self.picture_relationship_id = kind.picture_relationship_id;
        self.disabled = false;
      }
    }
  }
}

#[derive(Clone, Debug)]
enum BulletOverride {
  Unspecified,
  Disabled,
  Kind(BulletKind),
}

#[derive(Clone, Debug)]
struct BulletKind {
  label: Option<String>,
  auto_number: Option<AutoNumberBullet>,
  picture_relationship_id: Option<String>,
}

fn auto_number_bullet(bullet: &a::AutoNumberedBullet) -> AutoNumberBullet {
  AutoNumberBullet {
    scheme: bullet.r#type,
    start_at: bullet.start_at,
  }
}

fn character_bullet_label(value: &str) -> Option<String> {
  // DrawingML models this as a character bullet. LibreOffice's UNO bridge
  // stores only the first Unicode code point in SvxNumberFormat::BulletChar,
  // which also matches PowerPoint fixed-output behavior for malformed
  // multi-character values such as "••" in legacy SmartArt drawings.
  value.chars().next().map(|character| character.to_string())
}

fn paragraph_properties_bullet(choice: &Option<a::ParagraphPropertiesChoice4>) -> BulletOverride {
  match choice {
    Some(a::ParagraphPropertiesChoice4::NoBullet) => BulletOverride::Disabled,
    Some(a::ParagraphPropertiesChoice4::CharacterBullet(bullet)) => {
      BulletOverride::Kind(BulletKind {
        label: character_bullet_label(&bullet.char),
        auto_number: None,
        picture_relationship_id: None,
      })
    }
    Some(a::ParagraphPropertiesChoice4::AutoNumberedBullet(bullet)) => {
      BulletOverride::Kind(BulletKind {
        label: None,
        auto_number: Some(auto_number_bullet(bullet)),
        picture_relationship_id: None,
      })
    }
    Some(a::ParagraphPropertiesChoice4::PictureBullet(bullet)) => {
      BulletOverride::Kind(BulletKind {
        label: Some("\u{2022}".to_string()),
        auto_number: None,
        picture_relationship_id: bullet.blip.embed.clone(),
      })
    }
    None => BulletOverride::Unspecified,
  }
}

fn paragraph_graphic_bullet_size_100mm(
  paragraph: &TextParagraph,
  paragraph_style: &ParagraphDisplayStyle,
  bullet: &BulletDisplay,
  base_style: &TextStyle,
  options: &TextLoweringOptions,
  image_resources: Option<&HashMap<String, ImageResource>>,
) -> Option<(i32, i32)> {
  let relationship_id = bullet.picture_relationship_id.as_deref()?;
  let first_char_height =
    paragraph_first_char_font_size_points100(paragraph, paragraph_style, base_style, options);
  let mut height_100mm = drawingml_text_size_to_mm100(first_char_height);
  match bullet.size {
    BulletSize::FollowText => {
      height_100mm = (height_100mm as f32 * 0.7).round() as i32;
    }
    BulletSize::Percent(percent) => {
      height_100mm = (height_100mm as f32 * percent / 100.0 * 0.7).round() as i32;
    }
    BulletSize::Points100(points100) => {
      height_100mm = drawingml_text_size_to_mm100(points100);
    }
  }
  let mut width_100mm = height_100mm;
  if let Some(aspect_ratio) = image_resources
    .and_then(|resources| resources.get(relationship_id))
    .and_then(|resource| image_aspect_ratio(&resource.data))
    && (aspect_ratio - 1.0).abs() > f32::EPSILON
  {
    width_100mm = (height_100mm as f32 * aspect_ratio).round() as i32;
  }
  Some((width_100mm, height_100mm))
}

fn image_aspect_ratio(data: &[u8]) -> Option<f32> {
  let image = image::load_from_memory(data).ok()?;
  let (width, height) = image.dimensions();
  (height > 0).then_some(width as f32 / height as f32)
}

fn drawingml_text_size_to_mm100(points100: i32) -> i32 {
  sdk_units::points100_to_mm100(points100) as i32
}

fn paragraph_first_char_font_size_points100(
  paragraph: &TextParagraph,
  paragraph_style: &ParagraphDisplayStyle,
  base_style: &TextStyle,
  options: &TextLoweringOptions,
) -> i32 {
  if let Some(run_font_size) = paragraph
    .runs
    .iter()
    .filter(|run| {
      matches!(
        run.kind,
        TextRunKind::Run | TextRunKind::Field | TextRunKind::Math
      )
    })
    .find_map(|run| {
      run
        .run_properties
        .as_deref()
        .and_then(|properties| properties.font_size)
    })
  {
    return (run_font_size as f32 * options.font_scale).round() as i32;
  }
  [
    paragraph_style.direct_default_run_properties.as_deref(),
    paragraph_style.text_default_run_properties.as_deref(),
    paragraph_style.master_default_run_properties.as_deref(),
  ]
  .into_iter()
  .flatten()
  .find_map(|properties| properties.font_size)
  .map(|font_size| (font_size as f32 * options.font_scale).round() as i32)
  .unwrap_or_else(|| (base_style.font_size_pt * 100.0 * options.font_scale).round() as i32)
}

fn record_bullet_paragraph(
  summary: Option<&mut PptxLayoutSummary>,
  page_index: usize,
  paragraph_index: usize,
  paragraph: &TextParagraph,
  bullet: &BulletDisplay,
) {
  let Some(summary) = summary else {
    return;
  };
  if bullet.label.is_none() && bullet.picture_relationship_id.is_none() {
    return;
  }
  summary.bullet_paragraphs.push(PptxBulletParagraphSummary {
    page_index,
    paragraph_index,
    text: paragraph
      .runs
      .iter()
      .filter(|run| {
        matches!(
          run.kind,
          TextRunKind::Run | TextRunKind::Field | TextRunKind::Math
        )
      })
      .map(|run| run.text.as_str())
      .collect(),
    character: bullet.label.clone(),
    font: bullet.font.clone(),
    graphic_width_100mm: bullet.graphic_width_100mm,
    graphic_height_100mm: bullet.graphic_height_100mm,
  });
}

macro_rules! bullet_fn {
  ($name:ident, $choice_ty:ty) => {
    fn $name(choice: &Option<$choice_ty>) -> BulletOverride {
      match choice {
        Some(choice) => level_bullet_label(choice),
        None => BulletOverride::Unspecified,
      }
    }
  };
}

fn default_paragraph_properties_bullet(
  choice: &Option<a::DefaultParagraphPropertiesChoice4>,
) -> BulletOverride {
  match choice {
    Some(a::DefaultParagraphPropertiesChoice4::NoBullet) => BulletOverride::Disabled,
    Some(a::DefaultParagraphPropertiesChoice4::CharacterBullet(bullet)) => {
      BulletOverride::Kind(BulletKind {
        label: character_bullet_label(&bullet.char),
        auto_number: None,
        picture_relationship_id: None,
      })
    }
    Some(a::DefaultParagraphPropertiesChoice4::AutoNumberedBullet(bullet)) => {
      BulletOverride::Kind(BulletKind {
        label: None,
        auto_number: Some(auto_number_bullet(bullet)),
        picture_relationship_id: None,
      })
    }
    Some(a::DefaultParagraphPropertiesChoice4::PictureBullet(bullet)) => {
      BulletOverride::Kind(BulletKind {
        label: Some("\u{2022}".to_string()),
        auto_number: None,
        picture_relationship_id: bullet.blip.embed.clone(),
      })
    }
    None => BulletOverride::Unspecified,
  }
}

bullet_fn!(
  level1_paragraph_properties_bullet,
  a::Level1ParagraphPropertiesChoice4
);
bullet_fn!(
  level2_paragraph_properties_bullet,
  a::Level2ParagraphPropertiesChoice4
);
bullet_fn!(
  level3_paragraph_properties_bullet,
  a::Level3ParagraphPropertiesChoice4
);
bullet_fn!(
  level4_paragraph_properties_bullet,
  a::Level4ParagraphPropertiesChoice4
);
bullet_fn!(
  level5_paragraph_properties_bullet,
  a::Level5ParagraphPropertiesChoice4
);
bullet_fn!(
  level6_paragraph_properties_bullet,
  a::Level6ParagraphPropertiesChoice4
);
bullet_fn!(
  level7_paragraph_properties_bullet,
  a::Level7ParagraphPropertiesChoice4
);
bullet_fn!(
  level8_paragraph_properties_bullet,
  a::Level8ParagraphPropertiesChoice4
);
bullet_fn!(
  level9_paragraph_properties_bullet,
  a::Level9ParagraphPropertiesChoice4
);

trait BulletChoice {
  fn no_bullet(&self) -> bool;
  fn character(&self) -> Option<String>;
  fn auto_number(&self) -> Option<AutoNumberBullet>;
  fn picture_relationship_id(&self) -> Option<String>;
}

macro_rules! impl_bullet_choice {
  ($ty:ty) => {
    impl BulletChoice for $ty {
      fn no_bullet(&self) -> bool {
        matches!(self, Self::NoBullet)
      }

      fn character(&self) -> Option<String> {
        match self {
          Self::CharacterBullet(bullet) => character_bullet_label(&bullet.char),
          _ => None,
        }
      }

      fn auto_number(&self) -> Option<AutoNumberBullet> {
        match self {
          Self::AutoNumberedBullet(bullet) => Some(auto_number_bullet(bullet)),
          _ => None,
        }
      }

      fn picture_relationship_id(&self) -> Option<String> {
        match self {
          Self::PictureBullet(bullet) => bullet.blip.embed.clone(),
          _ => None,
        }
      }
    }
  };
}

impl_bullet_choice!(a::Level1ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level2ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level3ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level4ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level5ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level6ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level7ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level8ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level9ParagraphPropertiesChoice4);

fn level_bullet_label(choice: &impl BulletChoice) -> BulletOverride {
  if choice.no_bullet() {
    BulletOverride::Disabled
  } else if let Some(character) = choice.character() {
    BulletOverride::Kind(BulletKind {
      label: Some(character),
      auto_number: None,
      picture_relationship_id: None,
    })
  } else if let Some(auto_number) = choice.auto_number() {
    BulletOverride::Kind(BulletKind {
      label: None,
      auto_number: Some(auto_number),
      picture_relationship_id: None,
    })
  } else {
    choice
      .picture_relationship_id()
      .map(|relationship_id| BulletKind {
        label: Some("\u{2022}".to_string()),
        auto_number: None,
        picture_relationship_id: Some(relationship_id),
      })
      .map(BulletOverride::Kind)
      .unwrap_or(BulletOverride::Unspecified)
  }
}

fn apply_run_properties(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  run: &TextRun,
  style: &mut TextStyle,
) {
  if run.kind == TextRunKind::Math {
    // Math OLE object. Use the Office math face for text extraction/rendering
    // of the flattened math text instead of inheriting the surrounding
    // DrawingML paragraph font.
    style.font_family = Some(Arc::from("Cambria Math"));
  }
  let Some(properties) = run.run_properties.as_deref() else {
    return;
  };
  apply_run_common(
    import,
    RunCommon {
      font_size: properties.font_size,
      bold: properties.bold.as_ref().map(|value| value.as_bool()),
      italic: properties.italic.as_ref().map(|value| value.as_bool()),
      underline: properties.underline,
      strike: properties.strike,
      capital: properties.capital,
      spacing: properties.spacing,
      baseline: properties.baseline,
      latin_font: properties.latin_font.as_ref(),
      east_asian_font: properties.east_asian_font.as_ref(),
      complex_script_font: properties.complex_script_font.as_ref(),
      symbol_font: properties.symbol_font.as_ref(),
    },
    style,
  );
  if let Some(fill) = properties.run_properties_choice1.as_ref() {
    apply_text_fill(import, slide, fill, style);
  }
  if let Some(fill) = properties.run_properties_choice4.as_ref() {
    apply_run_underline_fill(import, slide, fill, style);
  }
  if (properties.hyperlink_on_click.is_some() || properties.hyperlink_on_mouse_over.is_some())
    && properties.run_properties_choice1.is_none()
  {
    apply_hyperlink_text_fill(import, slide, style);
  }
  if (properties.hyperlink_on_click.is_some() || properties.hyperlink_on_mouse_over.is_some())
    && properties.underline.is_none()
  {
    style.underline = true;
  }
  if let Some(highlight) = properties.highlight.as_deref() {
    apply_text_highlight(import, slide, highlight, style);
  }
}

fn apply_default_run_properties(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  properties: &a::DefaultRunProperties,
  style: &mut TextStyle,
) {
  apply_run_common(
    import,
    RunCommon {
      font_size: properties.font_size,
      bold: properties.bold.as_ref().map(|value| value.as_bool()),
      italic: properties.italic.as_ref().map(|value| value.as_bool()),
      underline: properties.underline,
      strike: properties.strike,
      capital: properties.capital,
      spacing: properties.spacing,
      baseline: properties.baseline,
      latin_font: properties.latin_font.as_ref(),
      east_asian_font: properties.east_asian_font.as_ref(),
      complex_script_font: properties.complex_script_font.as_ref(),
      symbol_font: properties.symbol_font.as_ref(),
    },
    style,
  );
  if let Some(fill) = properties.default_run_properties_choice1.as_ref() {
    apply_default_text_fill(import, slide, fill, style);
  }
  if let Some(fill) = properties.default_run_properties_choice4.as_ref() {
    apply_default_run_underline_fill(import, slide, fill, style);
  }
  if let Some(highlight) = properties.highlight.as_deref() {
    apply_text_highlight(import, slide, highlight, style);
  }
}

struct RunCommon<'a> {
  font_size: Option<i32>,
  bold: Option<bool>,
  italic: Option<bool>,
  underline: Option<a::TextUnderlineValues>,
  strike: Option<a::TextStrikeValues>,
  capital: Option<a::TextCapsValues>,
  spacing: Option<ooxmlsdk::simple_type::TextPointValue>,
  baseline: Option<ooxmlsdk::simple_type::DrawingmlPercentageValue>,
  latin_font: Option<&'a a::LatinFont>,
  east_asian_font: Option<&'a a::EastAsianFont>,
  complex_script_font: Option<&'a a::ComplexScriptFont>,
  symbol_font: Option<&'a a::SymbolFont>,
}

fn apply_run_common(import: &PowerPointImport, properties: RunCommon<'_>, style: &mut TextStyle) {
  if let Some(font_size) = properties.font_size {
    style.font_size_pt = ooxmlsdk::units::drawingml_text_size_to_points(font_size) as f32;
  }
  if let Some(bold) = properties.bold {
    style.bold = bold;
  }
  if let Some(italic) = properties.italic {
    style.italic = italic;
  }
  if let Some(underline) = properties.underline {
    style.underline = underline != a::TextUnderlineValues::None;
  }
  if let Some(strike) = properties.strike {
    style.strikethrough = strike != a::TextStrikeValues::NoStrike;
  }
  if let Some(capital) = properties.capital {
    style.uppercase = capital == a::TextCapsValues::All;
    style.small_caps = capital == a::TextCapsValues::Small;
  }
  if let Some(spacing) = properties.spacing {
    style.character_spacing_pt = spacing.to_points() as f32;
  }
  if let Some(baseline) = properties.baseline {
    style.baseline_shift_pt =
      style.font_size_pt * baseline.as_drawingml_percent() as f32 / 100_000.0;
  }
  if let Some(typeface) = properties
    .latin_font
    .and_then(|font| font.typeface.as_ref())
    .filter(|typeface| !typeface.is_empty())
  {
    style.font_family = Some(Arc::from(resolve_theme_font(import, typeface)));
  }
  if let Some(typeface) = properties
    .east_asian_font
    .and_then(|font| font.typeface.as_ref())
    .filter(|typeface| !typeface.is_empty())
  {
    style.east_asia_font_family = Some(Arc::from(resolve_theme_font(import, typeface)));
  }
  if let Some(typeface) = properties
    .complex_script_font
    .and_then(|font| font.typeface.as_ref())
    .filter(|typeface| !typeface.is_empty())
  {
    style.complex_font_family = Some(Arc::from(resolve_theme_font(import, typeface)));
  }
  if let Some(typeface) = properties
    .symbol_font
    .and_then(|font| font.typeface.as_ref())
    .filter(|typeface| !typeface.is_empty())
  {
    style.symbol_font_family = Some(Arc::from(resolve_theme_font(import, typeface)));
  }
}

fn resolve_theme_font<'a>(import: &'a PowerPointImport, typeface: &'a str) -> &'a str {
  import.resolve_theme_font(typeface).unwrap_or(typeface)
}

fn apply_text_fill(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  fill: &a::RunPropertiesChoice,
  style: &mut TextStyle,
) {
  match fill {
    a::RunPropertiesChoice::NoFill(_) => {
      // ECMA-376 Part 1, 20.1.8.44: noFill applies no fill to its parent.
      // Keep the run for layout; the PDF renderer can still paint an
      // independently specified outline.
      style.color = RgbColor { r: 0, g: 0, b: 0 };
      style.opacity = 0.0;
    }
    a::RunPropertiesChoice::SolidFill(fill) => {
      if let Some(color) = fill
        .solid_fill_choice
        .as_ref()
        .and_then(Color::from_solid_fill_choice)
        .and_then(|color| display_paint_for_optional_slide(import, slide, &color, None))
      {
        style.color = color.color;
        style.opacity = color.opacity;
      }
    }
    a::RunPropertiesChoice::GradientFill(fill) => {
      apply_best_solid_text_fill(import, slide, Color::best_solid_gradient_color(fill), style);
    }
    a::RunPropertiesChoice::PatternFill(fill) => {
      apply_best_solid_text_fill(import, slide, Color::best_solid_pattern_color(fill), style);
    }
    a::RunPropertiesChoice::BlipFill(_) | a::RunPropertiesChoice::GroupFill => {}
  }
}

fn apply_default_text_fill(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  fill: &a::DefaultRunPropertiesChoice,
  style: &mut TextStyle,
) {
  match fill {
    a::DefaultRunPropertiesChoice::NoFill(_) => {
      // ECMA-376 Part 1, 20.1.8.44: noFill applies no fill to its parent.
      style.color = RgbColor { r: 0, g: 0, b: 0 };
      style.opacity = 0.0;
    }
    a::DefaultRunPropertiesChoice::SolidFill(fill) => {
      if let Some(color) = fill
        .solid_fill_choice
        .as_ref()
        .and_then(Color::from_solid_fill_choice)
        .and_then(|color| display_paint_for_optional_slide(import, slide, &color, None))
      {
        style.color = color.color;
        style.opacity = color.opacity;
      }
    }
    a::DefaultRunPropertiesChoice::GradientFill(fill) => {
      apply_best_solid_text_fill(import, slide, Color::best_solid_gradient_color(fill), style);
    }
    a::DefaultRunPropertiesChoice::PatternFill(fill) => {
      apply_best_solid_text_fill(import, slide, Color::best_solid_pattern_color(fill), style);
    }
    a::DefaultRunPropertiesChoice::BlipFill(_) | a::DefaultRunPropertiesChoice::GroupFill => {}
  }
}

fn apply_best_solid_text_fill(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  color: Option<Color>,
  style: &mut TextStyle,
) {
  // maps DrawingML character fill to CharColor via getBestSolidColor().
  if let Some(color) =
    color.and_then(|color| display_paint_for_optional_slide(import, slide, &color, None))
  {
    style.color = color.color;
    style.opacity = color.opacity;
  }
}

fn apply_text_highlight(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  highlight: &a::Highlight,
  style: &mut TextStyle,
) {
  // imports a:highlight (CT_Color) through ColorContext into CharBackColor.
  if let Some(color) = highlight
    .highlight_choice
    .as_ref()
    .and_then(Color::from_highlight_choice)
    .and_then(|color| display_paint_for_optional_slide(import, slide, &color, None))
  {
    style.highlight = Some(color.color);
  }
}

fn apply_run_underline_fill(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  fill: &a::RunPropertiesChoice4,
  style: &mut TextStyle,
) {
  match fill {
    a::RunPropertiesChoice4::UnderlineFillText => style.underline_color = None,
    a::RunPropertiesChoice4::UnderlineFill(fill) => {
      apply_underline_fill(import, slide, fill, style)
    }
  }
}

fn apply_default_run_underline_fill(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  fill: &a::DefaultRunPropertiesChoice4,
  style: &mut TextStyle,
) {
  match fill {
    a::DefaultRunPropertiesChoice4::UnderlineFillText => style.underline_color = None,
    a::DefaultRunPropertiesChoice4::UnderlineFill(fill) => {
      apply_underline_fill(import, slide, fill, style)
    }
  }
}

fn apply_underline_fill(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  fill: &a::UnderlineFill,
  style: &mut TextStyle,
) {
  // parses a:uFill through SimpleFillPropertiesContext into maUnderlineColor.
  if let Some(color) = fill
    .underline_fill_choice
    .as_ref()
    .and_then(Color::from_underline_fill_choice)
    .and_then(|color| display_paint_for_optional_slide(import, slide, &color, None))
  {
    style.underline_color = Some(color.color);
  }
}

fn apply_hyperlink_text_fill(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  style: &mut TextStyle,
) {
  // color hlink when a hyperlink field has no explicit CharColor.
  let color = Color::Scheme(SchemeColor {
    value: a::SchemeColorValues::Hyperlink,
    transformations: Vec::new(),
  });
  if let Some(color) = display_paint_for_optional_slide(import, slide, &color, None) {
    style.color = color.color;
    style.opacity = color.opacity;
  }
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
    | FillKind::SlideBackground
    | FillKind::Group
    | FillKind::Gradient(_)
    | FillKind::Blip(_)
    | FillKind::Pattern(_) => None,
  }
}

fn fill_paint_for_slide(
  import: &PowerPointImport,
  slide: &SlidePersist,
  fill: &FillProperties,
) -> Option<DisplayPaint> {
  match &fill.kind {
    FillKind::Solid(color) => color.as_ref().and_then(|color| {
      display_paint_for_slide(import, slide, color, fill.placeholder_color.as_ref())
    }),
    FillKind::None
    | FillKind::SlideBackground
    | FillKind::Group
    | FillKind::Gradient(_)
    | FillKind::Blip(_)
    | FillKind::Pattern(_) => None,
  }
}

fn shape_fill_paint(
  import: &PowerPointImport,
  slide: &SlidePersist,
  fill: &FillProperties,
) -> Option<DisplayPaint> {
  if !matches!(fill.kind, FillKind::SlideBackground) {
    return fill_paint_for_slide(import, slide, fill);
  }

  let Some(background) = resolved_slide_background_fill(import, slide) else {
    return Some(default_page_background_paint());
  };
  match background.kind {
    // A no-fill page still exposes the PDF page's default white surface.
    FillKind::None => Some(default_page_background_paint()),
    _ => background_fill_paint(import, slide, &background),
  }
}

fn default_page_background_paint() -> DisplayPaint {
  DisplayPaint {
    color: RgbColor {
      r: 255,
      g: 255,
      b: 255,
    },
    opacity: 1.0,
  }
}

fn line_stroke(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  line: &LineProperties,
) -> Option<DisplayStroke> {
  let LineFill::Solid(color) = &line.fill else {
    return None;
  };
  let paint = color.as_ref().and_then(|color| {
    display_paint_for_optional_slide(import, slide, color, line.placeholder_color.as_ref())
  })?;
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

fn display_paint_for_slide(
  import: &PowerPointImport,
  slide: &SlidePersist,
  color: &Color,
  placeholder_color: Option<&Color>,
) -> Option<DisplayPaint> {
  let color = import.resolve_color_for_slide(slide, color, placeholder_color)?;
  Some(DisplayPaint {
    color: RgbColor {
      r: color.r,
      g: color.g,
      b: color.b,
    },
    opacity: color_opacity(color.alpha),
  })
}

fn display_paint_for_optional_slide(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  color: &Color,
  placeholder_color: Option<&Color>,
) -> Option<DisplayPaint> {
  match slide {
    Some(slide) => display_paint_for_slide(import, slide, color, placeholder_color),
    None => display_paint(import, color, placeholder_color),
  }
}

fn color_opacity(alpha: i32) -> f32 {
  alpha.clamp(0, 100_000) as f32 / 100_000.0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn slide_number_field_uses_the_current_presentation_number() {
    let run = TextRun {
      text: "‹#›".to_string(),
      kind: TextRunKind::Field,
      hyperlink_url: None,
      field_type: Some("slidenum".to_string()),
      run_properties: None,
      field_paragraph_properties: None,
    };
    assert_eq!(presentation_field_text(&run, 7), "7");
  }

  #[test]
  fn unspecified_bullet_font_preserves_inherited_font() {
    let mut bullet = BulletDisplay {
      font: Some("Arial".to_string()),
      ..BulletDisplay::default()
    };
    bullet.apply_font(BulletFontOverride::Unspecified);
    assert_eq!(bullet.font.as_deref(), Some("Arial"));
    bullet.apply_font(BulletFontOverride::FollowText);
    assert!(bullet.font.is_none());
  }

  #[test]
  fn character_bullet_uses_one_unicode_code_point() {
    assert_eq!(character_bullet_label("••").as_deref(), Some("•"));
    assert_eq!(character_bullet_label(""), None);
  }

  fn numbered_test_paragraph(level: u8) -> TextParagraph {
    TextParagraph {
      level: Some(level),
      runs: vec![TextRun {
        text: "item".to_string(),
        kind: TextRunKind::Run,
        hyperlink_url: None,
        field_type: None,
        run_properties: None,
        field_paragraph_properties: None,
      }],
      ..TextParagraph::default()
    }
  }

  #[test]
  fn drawingml_auto_number_sequences_are_independent_by_paragraph_level() {
    let mut state = AutoNumberingState::default();
    let base_bullet = BulletDisplay {
      auto_number: Some(AutoNumberBullet {
        scheme: a::TextAutoNumberSchemeValues::ArabicPeriod,
        start_at: None,
      }),
      ..BulletDisplay::default()
    };

    let mut level_zero_first = base_bullet.clone();
    state.resolve(&numbered_test_paragraph(0), &mut level_zero_first);
    let mut level_one_first = base_bullet.clone();
    state.resolve(&numbered_test_paragraph(1), &mut level_one_first);
    let mut level_zero_second = base_bullet;
    state.resolve(&numbered_test_paragraph(0), &mut level_zero_second);

    assert_eq!(level_zero_first.label.as_deref(), Some("1."));
    assert_eq!(level_one_first.label.as_deref(), Some("1."));
    assert_eq!(level_zero_second.label.as_deref(), Some("2."));
  }

  #[test]
  fn drawingml_auto_number_labels_follow_ecma_schemes() {
    use a::TextAutoNumberSchemeValues as Scheme;

    assert_eq!(
      format_auto_number(Scheme::AlphaLowerCharacterPeriod, 27),
      "aa."
    );
    assert_eq!(
      format_auto_number(Scheme::AlphaLowerCharacterPeriod, 53),
      "aaa."
    );
    assert_eq!(
      format_auto_number(Scheme::RomanUpperCharacterParenBoth, 3),
      "(III)"
    );
    assert_eq!(
      format_auto_number(Scheme::EastAsianJapaneseKoreanPeriod, 1),
      "一."
    );
  }

  #[test]
  fn repeated_start_at_describes_one_continuing_sequence() {
    let mut state = AutoNumberingState::default();
    let base_bullet = BulletDisplay {
      auto_number: Some(AutoNumberBullet {
        scheme: a::TextAutoNumberSchemeValues::ArabicPeriod,
        start_at: Some(3),
      }),
      ..BulletDisplay::default()
    };
    let mut first = base_bullet.clone();
    state.resolve(&numbered_test_paragraph(0), &mut first);
    let mut second = base_bullet;
    state.resolve(&numbered_test_paragraph(0), &mut second);

    assert_eq!(first.label.as_deref(), Some("3."));
    assert_eq!(second.label.as_deref(), Some("4."));
  }

  #[test]
  fn paragraph_keeps_master_shape_and_direct_run_style_precedence_layers() {
    let run_properties = |font_size| {
      Some(Box::new(a::DefaultRunProperties {
        font_size: Some(font_size),
        ..a::DefaultRunProperties::default()
      }))
    };
    let paragraph = TextParagraph {
      master_paragraph_style: Some(TextListParagraphStyle::Default(Box::new(
        a::DefaultParagraphProperties {
          default_run_properties: run_properties(1_000),
          ..a::DefaultParagraphProperties::default()
        },
      ))),
      text_paragraph_style: Some(TextListParagraphStyle::Default(Box::new(
        a::DefaultParagraphProperties {
          default_run_properties: run_properties(2_000),
          ..a::DefaultParagraphProperties::default()
        },
      ))),
      paragraph_properties: Some(Box::new(a::ParagraphProperties {
        default_run_properties: run_properties(3_000),
        ..a::ParagraphProperties::default()
      })),
      ..TextParagraph::default()
    };

    let style = ParagraphDisplayStyle::from_paragraph(&paragraph);

    assert_eq!(
      style
        .master_default_run_properties
        .as_deref()
        .and_then(|properties| properties.font_size),
      Some(1_000)
    );
    assert_eq!(
      style
        .text_default_run_properties
        .as_deref()
        .and_then(|properties| properties.font_size),
      Some(2_000)
    );
    assert_eq!(
      style
        .direct_default_run_properties
        .as_deref()
        .and_then(|properties| properties.font_size),
      Some(3_000)
    );
  }

  #[test]
  fn editor_only_placeholder_does_not_render_inherited_bullet() {
    let style = ParagraphDisplayStyle {
      bullet: BulletDisplay {
        label: Some("\u{2022}".to_string()),
        ..BulletDisplay::default()
      },
      ..ParagraphDisplayStyle::default()
    };
    let paragraph = TextParagraph {
      runs: vec![TextRun {
        text: "Click to add text".to_string(),
        kind: TextRunKind::Placeholder,
        hyperlink_url: None,
        field_type: None,
        run_properties: None,
        field_paragraph_properties: None,
      }],
      ..TextParagraph::default()
    };

    let bullet = style.bullet(&paragraph);

    assert!(bullet.label.is_none());
    assert!(bullet.picture_relationship_id.is_none());
  }

  #[test]
  fn empty_paragraph_does_not_render_inherited_bullet() {
    let style = ParagraphDisplayStyle {
      bullet: BulletDisplay {
        label: Some("\u{2022}".to_string()),
        ..BulletDisplay::default()
      },
      ..ParagraphDisplayStyle::default()
    };

    let bullet = style.bullet(&TextParagraph::default());

    assert!(bullet.label.is_none());
    assert!(bullet.picture_relationship_id.is_none());
  }

  #[test]
  fn explicit_level_no_bullet_clears_inherited_bullet() {
    let mut style = ParagraphDisplayStyle {
      bullet: BulletDisplay {
        label: Some("\u{2022}".to_string()),
        ..BulletDisplay::default()
      },
      ..ParagraphDisplayStyle::default()
    };
    style
      .bullet
      .apply_kind(level2_paragraph_properties_bullet(&Some(
        a::Level2ParagraphPropertiesChoice4::NoBullet,
      )));
    let paragraph = TextParagraph {
      level: Some(1),
      runs: vec![TextRun {
        text: "Visible text".to_string(),
        kind: TextRunKind::Run,
        hyperlink_url: None,
        field_type: None,
        run_properties: None,
        field_paragraph_properties: None,
      }],
      ..TextParagraph::default()
    };

    let bullet = style.bullet(&paragraph);

    assert!(bullet.label.is_none());
    assert!(bullet.picture_relationship_id.is_none());
  }
}
