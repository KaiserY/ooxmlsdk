use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Cursor;
use std::ops::Range;
use std::sync::{Arc, OnceLock};

use crate::common::drawingml_geometry::{
  group_child_affine, transform_commands, transform_point, transform_rect_bounds, transform_vector,
};
use crate::common::drawingml_image_effects::{
  ImageEffect, ImageEffectColorResolver, ResolvedEffectColor,
};
use crate::common::{self, DebugProperty, DebugRecord, DebugShape, DebugValue, Point, Rect, Size};
use crate::common::{
  drawingml_custom_geometry as custom_geometry, drawingml_preset_geometry as preset_geometry,
};
use crate::field_datetime;
use crate::localization::OfficeLocaleContext;
use crate::model::{
  BorderStyle, ImageCrop, ImageItem, LineItem, LineItemKind, LinkAreaItem, PageItem, PageSetup,
  PdfTextSegmentation, RectItem, RgbColor, RgbColor as LayoutRgbColor, TextItem, TextStyle,
  common_page_setup, common_point, common_rect, common_rgb, common_stroke_from_border,
  common_text_style,
};
use crate::options::{FieldUpdateDateTime, LayoutOptions};
use crate::render::chart as shared_chart;
use crate::render::diagram as shared_diagram;
use crate::render::symbol as shared_symbol;
use crate::text_layout::{StyledTextSpan, break_text_lines};
use crate::text_metrics::TextMetrics;
use crate::units;
use icu_segmenter::GraphemeClusterSegmenter;
use image::codecs::png::PngEncoder;
use image::{ColorType, GenericImageView, ImageEncoder};
use kurbo::{Affine, Rect as KurboRect};
use ooxmlsdk::schemas::schemas_microsoft_com_office_drawing_2008_diagram as dsp;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_chart as c;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_diagram as dgm;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;
use ooxmlsdk::units as sdk_units;
use ooxmlsdk::units::DrawingmlPercentageValue;
use unicode_bidi::{BidiClass, BidiInfo, Level, bidi_class};
use unicode_script::{Script, UnicodeScript};

use super::chart::{
  CartesianChartGroupDecorationStyle, ChartFrame, ChartLayoutProfile, ClusteredColumnStyle,
  RadialChartStyle, lower_clustered_column_chart, lower_radial_chart,
};
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
use super::shadow::{
  ShadowFrame, ShadowShape, glow_image_item, inner_shadow_image_item, outer_shadow_image_item,
  reflection_mask_image_item, shadow_alignment, soft_edge_mask_image_item,
};
use super::slide::{BackgroundKind, ChartResource, ImageResource, SlidePersist};
use super::{
  PptxBulletParagraphSummary, PptxDrawShapeSummary, PptxLayoutSummary,
  PptxSmartArtTextShapeSummary, PptxTextShapeSummary,
};

const DEFAULT_TEXT_FONT_SIZE_PT: f32 = 18.0;
const MINIMUM_TEXT_FONT_SIZE_PT: f32 = 1.0;
const DEFAULT_TEXT_LINE_HEIGHT_SCALE: f32 = 1.2;
const DEFAULT_TABLE_BORDER_PT: f32 = 0.75;
const MISSING_PICTURE_BORDER_WIDTH_PT: f32 = 0.14;
const MISSING_PICTURE_BORDER_INSET_PT: f32 = 0.06;
const MISSING_PICTURE_ICON_OFFSET_PT: f32 = 0.84;
const MISSING_PICTURE_ICON_WIDTH_PT: f32 = 1.68;
const MISSING_PICTURE_ICON_HEIGHT_PT: f32 = 1.92;
// Microsoft Office fixed-output evidence:
// - smartart-missing-bullet.pptx scales the synthesized 22.5 pt indent with
//   the SmartArt font scale;
// - smartart-autofit-sync.pptx stops that spacing scale at 20% while its font
//   continues shrinking.
const POWERPOINT_SMARTART_MINIMUM_AUTOFIT_SPACING_SCALE: f32 = 0.2;
// The explicitly sized C branch in smartart-autofit-sync.pptx disables font
// autofit, but Office still emits the tx-generated 22.5 pt hanging indent at
// 40% (9 pt). This is separate from document-authored paragraph indentation.
const POWERPOINT_SMARTART_EXPLICIT_FONT_SPACING_SCALE: f32 = 0.4;

pub(crate) fn lower_to_layout_document(
  import: &PowerPointImport,
  options: &LayoutOptions,
) -> common::LayoutDocument<'static> {
  let locales = OfficeLocaleContext::new(
    options.ui_language.as_deref(),
    options.format_locale.as_deref(),
    options.default_document_language.as_deref(),
  );
  let pages = import
    .draw_pages
    .iter()
    .enumerate()
    .filter(|(_, slide)| slide.visible)
    .map(|(page_index, slide)| {
      (
        slide.size.to_page_setup(),
        lower_slide_items_with_summary(import, slide, page_index, &locales, None),
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
    PageItem::Group {
      mask,
      clip,
      transform,
      blend_mode,
      opacity,
      items,
    } => common::DisplayItem::Group(common::CompositingGroup {
      mask: mask.map(common_image_item),
      clip,
      transform,
      blend_mode,
      opacity,
      flatten_identity: false,
      inherit_text_line_owner: true,
      items: items.into_iter().map(common_display_item).collect(),
    }),
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
    paint_clip: None,
    style: common_text_style(*item.style),
    font_id: None,
    color,
    rotation_center: item.rotation_center_pt.map(|(x, y)| common_point(x, y)),
    hyperlink_url: item.hyperlink_url.map(Cow::Owned),
    dynamic_field: None,
    form_widget_id: item.form_widget_id,
    paragraph_bidi: item.paragraph_bidi,
    word_spacing_pt: 0.0,
    preserve_text_portion: item.preserve_text_portion,
    pdf_text_segmentation: match item.pdf_text_segmentation {
      PdfTextSegmentation::Line => common::PdfTextSegmentation::Line,
      PdfTextSegmentation::WordLine => common::PdfTextSegmentation::WordLine,
      PdfTextSegmentation::Portion => common::PdfTextSegmentation::Portion,
    },
    source: (!item.source_path.is_empty()).then_some(common::DisplaySource {
      engine: common::LayoutEngineKind::Pptx,
      path: item.source_path,
      relationship_id: None,
    }),
  }
}

fn common_image_item(item: ImageItem) -> common::ImageItem<'static> {
  let semantic_metafile_text = supports_semantic_metafile_text(item.content_type.as_deref());
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
    metafile_monochrome_dib_palette_override: item.metafile_monochrome_dib_palette_override,
    metafile_background_color: item.metafile_background_color,
    metafile_external_header: item.metafile_external_header,
    relationship_id: None,
    alt_text: item.alt_text.map(Cow::Owned),
    hyperlink_url: item.hyperlink_url.map(Cow::Owned),
    semantic_metafile_text,
    metafile_semantic_text_includes_raster_backdrop: item
      .metafile_semantic_text_includes_raster_backdrop,
    signature_line: None,
    metafile_native_size: false,
    floating: item.floating,
    behind_text: item.behind_text,
  }
}

fn supports_semantic_metafile_text(content_type: Option<&str>) -> bool {
  content_type.is_some_and(|content_type| {
    matches!(
      content_type.to_ascii_lowercase().as_str(),
      "image/emf"
        | "image/x-emf"
        | "application/emf"
        | "application/x-emf"
        | "image/wmf"
        | "image/x-wmf"
        | "application/wmf"
        | "application/x-wmf"
    )
  })
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
      ..Default::default()
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
  let locales = OfficeLocaleContext::default();
  for (page_index, slide) in import.draw_pages.iter().enumerate() {
    let _ = lower_slide_items_with_summary(import, slide, page_index, &locales, Some(&mut summary));
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
    metadata.push(debug_i32("left_margin_100mm", paragraph.left_margin_100mm));
    metadata.push(debug_i32("indent_100mm", paragraph.indent_100mm));
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

  let bounds = transform.shape_bounds(shape);
  let x_pt = units::emu_to_points_f32(bounds.x0 as f32);
  let y_pt = units::emu_to_points_f32(bounds.y0 as f32);
  let width_pt = units::emu_to_points_f32(bounds.width() as f32);
  let height_pt = units::emu_to_points_f32(bounds.height() as f32);
  let visible_rotation = shape_visual_rotation_degrees(shape);
  let (geo_x_pt, geo_y_pt) =
    rotated_shape_geo_top_left(x_pt, y_pt, width_pt, height_pt, visible_rotation);
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
      rotation_deg: visible_rotation,
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
struct ShapeSummaryTransform(Affine);

impl Default for ShapeSummaryTransform {
  fn default() -> Self {
    Self(Affine::IDENTITY)
  }
}

impl ShapeSummaryTransform {
  fn shape_bounds(self, shape: &Shape) -> KurboRect {
    transform_rect_bounds(
      KurboRect::new(
        shape.position.x as f64,
        shape.position.y as f64,
        shape.position.x as f64 + shape.size.cx as f64,
        shape.position.y as f64 + shape.size.cy as f64,
      ),
      self.0,
    )
  }

  fn child(self, shape: &Shape) -> Self {
    let child_width = if shape.child_size.cx != 0 {
      shape.child_size.cx
    } else {
      shape.size.cx
    };
    let child_height = if shape.child_size.cy != 0 {
      shape.child_size.cy
    } else {
      shape.size.cy
    };
    Self(
      self.0
        * group_child_affine(
          kurbo::Point::new(shape.position.x as f64, shape.position.y as f64),
          kurbo::Vec2::new(shape.size.cx as f64, shape.size.cy as f64),
          kurbo::Point::new(shape.child_position.x as f64, shape.child_position.y as f64),
          kurbo::Vec2::new(child_width as f64, child_height as f64),
        ),
    )
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
  locales: &OfficeLocaleContext,
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
      locales,
      inherited_scene3d: None,
    },
    &slide.shapes,
    &mut items,
    summary,
  );
  materialize_drawingml_text_effects(&mut items, &mut TextMetrics::new());
  lift_pptx_semantic_text_overlays(&mut items);
  items
}

fn lower_background(
  import: &PowerPointImport,
  slide: &SlidePersist,
  fill_properties: &FillProperties,
  items: &mut Vec<PageItem>,
) {
  let common_fill = match &fill_properties.kind {
    FillKind::Gradient(gradient) => gradient_fill_for_optional_slide(
      import,
      Some(slide),
      gradient,
      shared_diagram::DiagramBounds {
        x: 0.0,
        y: 0.0,
        width: slide.size.width_pt,
        height: slide.size.height_pt,
      },
    ),
    _ => common_fill_for_optional_slide(import, Some(slide), fill_properties),
  };
  if let Some(fill) = common_fill {
    match fill {
      common::Fill::Solid(color)
        if is_default_white_page_background(DisplayPaint {
          color: RgbColor {
            r: color.r,
            g: color.g,
            b: color.b,
          },
          opacity: opacity_from_common_color(color),
        }) => {}
      common::Fill::Solid(color) => items.push(PageItem::Rect(RectItem {
        x_pt: 0.0,
        y_pt: 0.0,
        width_pt: slide.size.width_pt,
        height_pt: slide.size.height_pt,
        fill_color: Some(RgbColor {
          r: color.r,
          g: color.g,
          b: color.b,
        }),
        fill_opacity: opacity_from_common_color(color),
        stroke: None,
        stroke_opacity: 1.0,
      })),
      common::Fill::Pattern(_) | common::Fill::Gradient(_) => push_pattern_rect(
        items,
        0.0,
        0.0,
        slide.size.width_pt,
        slide.size.height_pt,
        fill,
      ),
      _ => {}
    }
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

fn lower_shapes(
  context: PptxLoweringContext<'_>,
  shapes: &[Shape],
  items: &mut Vec<PageItem>,
  mut summary: Option<&mut PptxLayoutSummary>,
) {
  for (shape_index, shape) in shapes.iter().enumerate() {
    lower_shape(
      context,
      shape,
      DisplayOffset::default(),
      &[shape_index],
      items,
      summary.as_deref_mut(),
    );
  }
}

#[derive(Clone, Copy, Debug)]
struct DisplayOffset(Affine);

impl Default for DisplayOffset {
  fn default() -> Self {
    Self(Affine::IDENTITY)
  }
}

impl DisplayOffset {
  fn x_pt(self, x_emu: i64) -> f32 {
    let point = self.0 * kurbo::Point::new(x_emu as f64, 0.0);
    units::emu_to_points_f32(point.x as f32)
  }

  fn y_pt(self, y_emu: i64) -> f32 {
    let point = self.0 * kurbo::Point::new(0.0, y_emu as f64);
    units::emu_to_points_f32(point.y as f32)
  }

  fn width_pt(self, width_emu: i64) -> f32 {
    let vector = transform_vector(kurbo::Vec2::new(width_emu as f64, 0.0), self.0);
    units::emu_to_points_f32(vector.x as f32)
  }

  fn height_pt(self, height_emu: i64) -> f32 {
    let vector = transform_vector(kurbo::Vec2::new(0.0, height_emu as f64), self.0);
    units::emu_to_points_f32(vector.y as f32)
  }

  fn scale_y(self) -> f32 {
    self.0.as_coeffs()[3] as f32
  }
}

fn lower_shape(
  context: PptxLoweringContext<'_>,
  source_shape: &Shape,
  offset: DisplayOffset,
  source_path: &[usize],
  items: &mut Vec<PageItem>,
  mut summary: Option<&mut PptxLayoutSummary>,
) {
  // A group scene supplies the camera/light rig for descendant sp3d records.
  // Clone only the rare descendant that actually needs this inheritance; the
  // ordinary 2-D traversal remains allocation-free.
  let inherited_shape =
    (source_shape.scene3d.is_none() && source_shape.shape3d.is_some()).then(|| {
      let mut shape = source_shape.clone();
      shape.scene3d = context.inherited_scene3d.cloned();
      shape
    });
  let shape = inherited_shape.as_ref().unwrap_or(source_shape);
  let PptxLoweringContext {
    import,
    slide,
    locales,
    ..
  } = context;
  let enabled_slide_number_field = inherited_slide_number_field_is_enabled(slide, shape);
  if shape.hidden
    || shape.hidden_master_shape
    || (!enabled_slide_number_field && shape.referenced)
    || is_uninstantiated_placeholder(shape)
    || is_disabled_header_footer_placeholder(slide, shape)
  {
    return;
  }

  let own_item_start = items.len();
  lower_legacy_vml_fill_image(shape, offset, items);
  let shape_visual_start = items.len();
  if shape.picture.is_some() {
    // ECMA-376's picture double-fill example places the p:spPr fill behind
    // the default p:blipFill, with the outline in front. Stage the bitmap
    // here; lower_shape_bounds reorders the complete surface before applying
    // effects and static 3-D.
    lower_picture(import, slide, shape, offset, items);
    lower_shape_bounds(
      import,
      slide,
      shape,
      offset,
      Some(shape_visual_start),
      items,
    );
  } else {
    lower_shape_bounds(import, slide, shape, offset, None, items);
  }
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
    lower_chart(import, slide, shape, offset, record, locales, items);
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
      source_path,
      text_body,
      items,
      summary.as_deref_mut(),
    );
  }
  for item in &mut items[own_item_start..] {
    if let PageItem::Text(text) = item
      && text.source_path.is_empty()
    {
      text.source_path = source_path.to_vec();
    }
  }

  let child_offset = child_display_offset(shape, offset);
  let child_context = PptxLoweringContext {
    inherited_scene3d: shape.scene3d.as_ref().or(context.inherited_scene3d),
    ..context
  };
  for (child_index, child) in shape.children.iter().enumerate() {
    let mut child_source_path = source_path.to_vec();
    child_source_path.push(child_index);
    lower_shape(
      child_context,
      child,
      child_offset,
      &child_source_path,
      items,
      summary.as_deref_mut(),
    );
  }
  if shape.service_name == ShapeService::Group
    && shape.size.cx > 0
    && shape.size.cy > 0
    && let Some(properties) = shape.actual_effect_properties.as_ref()
    && let Some(source) = properties
      .effect_dag
      .as_ref()
      .map(ShapeEffectSource::Dag)
      .or_else(|| properties.effect_list.as_ref().map(ShapeEffectSource::List))
  {
    let frame = TextFrame {
      x_pt: offset.x_pt(shape.position.x),
      y_pt: offset.y_pt(shape.position.y),
      width_pt: offset.width_pt(shape.size.cx),
      height_pt: offset.height_pt(shape.size.cy),
    };
    let bounds = transformed_shape_bounds(frame, shape);
    finish_shape_effect_raster(
      items,
      own_item_start,
      ShapeEffectRasterContext {
        import,
        slide,
        source: Some(source),
        scene3d: shape.scene3d.as_ref(),
        shape3d: shape.shape3d.as_ref(),
        bounds,
        rotation_degrees: shape_visual_rotation_degrees(shape),
        camera_shape_rotation_degrees: shape.rotation,
        children_source: true,
      },
    );
  }
}

fn is_uninstantiated_placeholder(shape: &Shape) -> bool {
  shape.sub_type.is_some()
    && shape
      .shape_location
      .is_some_and(|location| location != super::slide::ShapeLocation::Slide)
}

fn is_disabled_header_footer_placeholder(slide: &SlidePersist, shape: &Shape) -> bool {
  if shape.shape_location != Some(super::slide::ShapeLocation::Slide) {
    return false;
  }
  match shape.sub_type {
    Some(p::PlaceholderValues::SlideNumber) => !slide.header_footer.slide_number,
    Some(p::PlaceholderValues::Header) => !slide.header_footer.header,
    Some(p::PlaceholderValues::Footer) => !slide.header_footer.footer,
    Some(p::PlaceholderValues::DateAndTime) => !slide.header_footer.date_time,
    _ => false,
  }
}

fn inherited_slide_number_field_is_enabled(slide: &SlidePersist, shape: &Shape) -> bool {
  slide.header_footer.slide_number
    // ECMA-376 Part 1 §19.3.1.33 distinguishes a user-drawn object from a
    // presentation placeholder. A master placeholder still requires a
    // matching layout/slide instance; an explicitly user-drawn master text
    // box remains ordinary inherited slide content.
    && shape.user_drawn
    && shape.sub_type.is_none()
    && shape
      .shape_location
      .is_some_and(|location| location != super::slide::ShapeLocation::Slide)
    && shape.text_body.as_ref().is_some_and(|text_body| {
      text_body.paragraphs.iter().any(|paragraph| {
        paragraph.runs.iter().any(|run| {
          run.kind == TextRunKind::Field
            && run
              .field_type
              .as_deref()
              .is_some_and(|field_type| field_type.eq_ignore_ascii_case("slidenum"))
        })
      })
    })
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
      ole.shape_id.is_some()
        || ole.relationship_id.is_some()
        || ole.name.is_some()
        || ole.prog_id.is_some()
        || ole.image_width.is_some()
        || ole.image_height.is_some()
        || ole.show_as_icon
        || ole.ole_object_choice.is_some()
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
  locales: &OfficeLocaleContext,
  items: &mut Vec<PageItem>,
) {
  let ui_language = locales.ui_language();
  let default_document_language = locales.default_document_language();
  if shape.size.cx <= 0 || shape.size.cy <= 0 {
    return;
  }
  if let Some(chart_resource) = &record.extended_chart_resource {
    lower_extended_chart(import, slide, chart_resource, shape, offset, locales, items);
    return;
  }
  let Some(chart_resource) = &record.chart_resource else {
    return;
  };
  let x = offset.x_pt(shape.position.x);
  let y = offset.y_pt(shape.position.y);
  let width = offset.width_pt(shape.size.cx);
  let height = offset.height_pt(shape.size.cy);

  if let Some(mut chart) = shared_chart::pie_chart_model(&chart_resource.chart_space) {
    if chart.title.is_none()
      && shared_chart::has_powerpoint_generic_title_placeholder(&chart_resource.chart_space.chart)
    {
      chart.title = Some(shared_chart::ChartTitleText::Automatic);
    }
    let chart_text_properties = chart_resource.chart_space.text_properties.as_deref();
    let title = chart_resource.chart_space.chart.title.as_deref();
    let title_properties = title.and_then(|title| title.text_properties.as_deref());
    let legend_properties = chart_resource
      .chart_space
      .chart
      .legend
      .as_deref()
      .and_then(|legend| legend.text_properties.as_deref());
    let data_label_properties = chart.data_label_text_properties;
    let title_text_style_context = ChartTextStyleContext {
      import,
      slide,
      default_properties: chart_text_properties,
      theme_language: if matches!(
        chart.title.as_ref(),
        Some(shared_chart::ChartTitleText::Automatic)
      ) {
        ui_language
      } else {
        default_document_language
      },
    };
    let label_text_style_context = ChartTextStyleContext {
      theme_language: default_document_language,
      ..title_text_style_context
    };
    let point_colors = (0..chart.values.len())
      .map(|index| {
        chart
          .data_point_fills
          .iter()
          .find(|fill| fill.index as usize == index)
          .and_then(|fill| display_paint_for_chart(import, slide, chart_resource, fill.fill))
          .map(|paint| paint.color)
          .or_else(|| {
            chart
              .series_solid_fill
              .and_then(|fill| display_paint_for_chart(import, slide, chart_resource, fill))
              .map(|paint| paint.color)
          })
          .or_else(|| {
            let (formatting_index, maximum_formatting_index) = if chart.vary_colors {
              (index, chart.values.len().saturating_sub(1))
            } else {
              (
                chart.series_formatting_index,
                chart.maximum_series_formatting_index,
              )
            };
            display_color_for_chart_series(
              import,
              slide,
              chart_resource,
              shared_chart::chart_style_id(&chart_resource.chart_space).unwrap_or(2),
              formatting_index,
              maximum_formatting_index,
            )
          })
          .unwrap_or_default()
      })
      .collect();
    let inherited_point_style =
      pptx_chart_shape_style(import, slide, chart_resource, chart.series_shape_properties);
    let point_styles = (0..chart.values.len())
      .map(|index| {
        let point_style = chart
          .data_points
          .iter()
          .find(|point| usize::try_from(point.index.val).ok() == Some(index))
          .map(|point| {
            pptx_chart_shape_style(
              import,
              slide,
              chart_resource,
              point.chart_shape_properties.as_deref(),
            )
          })
          .unwrap_or_default();
        common::ShapeStyle {
          fill: point_style
            .fill
            .resolve_over(&inherited_point_style.fill)
            .clone(),
          stroke: point_style
            .stroke
            .resolve_over(&inherited_point_style.stroke)
            .clone(),
        }
      })
      .collect();
    let data_label_style = chart_text_style(
      label_text_style_context,
      data_label_properties,
      POWERPOINT_CHART_LABEL_FALLBACK,
      None,
    );
    let (data_label_styles, data_label_rich_text_styles) = chart_data_label_host_styles(
      label_text_style_context,
      &chart.data_labels,
      &data_label_style,
    );
    let mut chart_items = lower_radial_chart(
      ChartFrame {
        x_pt: x,
        y_pt: y,
        width_pt: width,
        height_pt: height,
      },
      &chart,
      shared_chart::automatic_chart_title(ui_language),
      &RadialChartStyle {
        layout_profile: ChartLayoutProfile::PowerPoint,
        title: chart_text_style(
          title_text_style_context,
          title_properties,
          POWERPOINT_CHART_TITLE_FALLBACK,
          title,
        ),
        legend: chart_text_style(
          label_text_style_context,
          legend_properties,
          POWERPOINT_CHART_LABEL_FALLBACK,
          None,
        ),
        data_label: data_label_style,
        data_label_styles,
        data_label_rich_text_styles,
        point_colors,
        point_styles,
        data_label_fill_colors: chart
          .data_labels
          .iter()
          .map(|label| {
            label
              .shape_properties
              .and_then(shared_chart::chart_shape_solid_fill)
              .and_then(|fill| {
                display_paint_for_chart(import, slide, chart_resource, fill)
                  .map(|paint| paint.color)
              })
          })
          .collect(),
        leader_line_style: pptx_chart_shape_style(
          import,
          slide,
          chart_resource,
          chart.leader_line_shape_properties,
        ),
        legend_frame_style: pptx_chart_shape_style(
          import,
          slide,
          chart_resource,
          chart_resource
            .chart_space
            .chart
            .legend
            .as_deref()
            .and_then(|legend| legend.chart_shape_properties.as_deref()),
        ),
        chart_area_style: pptx_chart_area_shape_style(
          import,
          slide,
          chart_resource,
          chart_resource.chart_space.shape_properties.as_deref(),
        ),
        plot_area_style: pptx_chart_area_shape_style(
          import,
          slide,
          chart_resource,
          chart_resource
            .chart_space
            .chart
            .plot_area
            .shape_properties
            .as_deref(),
        ),
      },
    );
    if !chart_items.is_empty() {
      if let (Some(fill), Some(shared_chart::ChartTitleText::Explicit(title))) = (
        chart_resource
          .chart_space
          .chart
          .title
          .as_deref()
          .and_then(|title| title.chart_shape_properties.as_deref())
          .and_then(
            |properties| match properties.chart_shape_properties_choice2.as_ref() {
              Some(c::ChartShapePropertiesChoice2::BlipFill(fill)) => Some(fill.as_ref()),
              _ => None,
            },
          ),
        chart.title.as_ref(),
      ) {
        insert_chart_title_blip_fill(import, slide, chart_resource, fill, title, &mut chart_items);
      }
      items.extend(chart_items);
      return;
    }
  }

  if let Some(mut chart) = shared_chart::cartesian_chart_for_locales(
    &chart_resource.chart_space,
    ui_language,
    locales.format_locale(),
  ) {
    if chart.title.is_none()
      && shared_chart::has_powerpoint_automatic_title_placeholder(&chart_resource.chart_space.chart)
    {
      chart.title = chart
        .series
        .first()
        .filter(|series| chart.series.len() == 1 && series.has_nonempty_explicit_name)
        .map(|series| shared_chart::ChartTitleText::Explicit(series.name.clone()))
        .or(Some(shared_chart::ChartTitleText::Automatic));
    }
    let chart_style_id = shared_chart::chart_style_id(&chart_resource.chart_space).unwrap_or(2);
    let maximum_series_formatting_index = chart
      .series
      .iter()
      .map(|series| series.formatting_index)
      .max()
      .unwrap_or(0);
    let series_colors = chart
      .series
      .iter()
      .map(|series| {
        series
          .solid_fill
          .and_then(|fill| display_paint_for_chart(import, slide, chart_resource, fill))
          .map(|paint| paint.color)
          .or_else(|| {
            display_color_for_chart_series(
              import,
              slide,
              chart_resource,
              chart_style_id,
              series.formatting_index,
              maximum_series_formatting_index,
            )
          })
          .unwrap_or_default()
      })
      .collect::<Vec<_>>();
    let series_styles = chart
      .series
      .iter()
      .map(|series| pptx_chart_shape_style(import, slide, chart_resource, series.shape_properties))
      .collect::<Vec<_>>();
    let series_point_styles = chart
      .series
      .iter()
      .map(|series| {
        (0..series.values.len())
          .map(|point_index| {
            series
              .data_points
              .iter()
              .find(|point| usize::try_from(point.index.val).ok() == Some(point_index))
              .map(|point| {
                pptx_chart_shape_style(
                  import,
                  slide,
                  chart_resource,
                  point.chart_shape_properties.as_deref(),
                )
              })
          })
          .collect()
      })
      .collect::<Vec<_>>();
    let series_point_colors = chart
      .series
      .iter()
      .enumerate()
      .map(|(series_index, series)| {
        if !chart.vary_colors_by_point
          || chart.series.len() != 1
          || series_index != 0
          || series
            .shape_properties
            .and_then(shared_chart::chart_shape_solid_fill)
            .is_some()
        {
          return vec![None; series.values.len()];
        }
        let maximum_point_index = series.values.len().saturating_sub(1);
        (0..series.values.len())
          .map(|point_index| {
            display_color_for_chart_series(
              import,
              slide,
              chart_resource,
              chart_style_id,
              point_index,
              maximum_point_index,
            )
          })
          .collect()
      })
      .collect::<Vec<_>>();
    let trendline_styles = chart
      .series
      .iter()
      .map(|series| {
        series
          .trendlines
          .iter()
          .map(|trendline| {
            pptx_chart_shape_style(
              import,
              slide,
              chart_resource,
              trendline.chart_shape_properties.as_deref(),
            )
          })
          .collect()
      })
      .collect::<Vec<_>>();
    let error_bar_styles = chart
      .series
      .iter()
      .map(|series| {
        series
          .error_bars
          .iter()
          .map(|error_bars| {
            pptx_chart_shape_style(import, slide, chart_resource, error_bars.shape_properties)
          })
          .collect()
      })
      .collect::<Vec<_>>();
    let group_decoration_styles = chart
      .group_decorations
      .iter()
      .map(|group| CartesianChartGroupDecorationStyle {
        drop_lines: pptx_chart_shape_style(
          import,
          slide,
          chart_resource,
          group
            .drop_lines
            .and_then(|lines| lines.chart_shape_properties.as_deref()),
        ),
        high_low_lines: pptx_chart_shape_style(
          import,
          slide,
          chart_resource,
          group
            .high_low_lines
            .and_then(|lines| lines.chart_shape_properties.as_deref()),
        ),
        up_bars: pptx_chart_shape_style(
          import,
          slide,
          chart_resource,
          group
            .up_down_bars
            .and_then(|bars| bars.up_bars.as_deref())
            .and_then(|bars| bars.chart_shape_properties.as_deref()),
        ),
        down_bars: pptx_chart_shape_style(
          import,
          slide,
          chart_resource,
          group
            .up_down_bars
            .and_then(|bars| bars.down_bars.as_deref())
            .and_then(|bars| bars.chart_shape_properties.as_deref()),
        ),
      })
      .collect::<Vec<_>>();
    let surface_band_colors = chart
      .surface_groups
      .iter()
      .map(|group| {
        group
          .band_fills
          .iter()
          .filter_map(|fill| {
            display_paint_for_chart(import, slide, chart_resource, fill.fill)
              .map(|paint| (fill.index, paint.color))
          })
          .collect()
      })
      .collect();
    if series_colors.len() == chart.series.len() {
      let title_properties = chart_resource
        .chart_space
        .chart
        .title
        .as_deref()
        .and_then(|title| title.text_properties.as_deref());
      let title = chart_resource.chart_space.chart.title.as_deref();
      let title_fill_color = chart_resource
        .chart_space
        .chart
        .title
        .as_deref()
        .and_then(|title| title.chart_shape_properties.as_deref())
        .and_then(shared_chart::chart_shape_solid_fill)
        .and_then(|fill| display_paint_for_chart(import, slide, chart_resource, fill))
        .map(|paint| paint.color);
      let title_blip_fill = chart_resource
        .chart_space
        .chart
        .title
        .as_deref()
        .and_then(|title| title.chart_shape_properties.as_deref())
        .and_then(
          |properties| match properties.chart_shape_properties_choice2.as_ref() {
            Some(c::ChartShapePropertiesChoice2::BlipFill(fill)) => Some(fill.as_ref()),
            _ => None,
          },
        );
      let value_label_properties = chart
        .value_axis
        .and_then(|axis| axis.text_properties.as_deref());
      let category_label_properties = chart
        .category_axis
        .and_then(|axis| axis.text_properties.as_deref())
        .or_else(|| {
          chart
            .date_axis
            .and_then(|axis| axis.text_properties.as_deref())
        })
        .or_else(|| {
          chart
            .horizontal_value_axis
            .and_then(|axis| axis.text_properties.as_deref())
        });
      let label_properties = value_label_properties.or(category_label_properties);
      let legend_properties = chart_resource
        .chart_space
        .chart
        .legend
        .as_deref()
        .and_then(|legend| legend.text_properties.as_deref());
      let data_label_properties = chart.data_label_text_properties.or(label_properties);
      let series_label_properties = chart
        .axis_sets
        .iter()
        .find_map(|axes| axes.series_axis?.text_properties.as_deref())
        .or(label_properties);
      // ECMA-376 Part 1 §21.2.2.216: c:chartSpace/c:txPr supplies
      // the defaults, while title/axis txPr overlays those defaults.
      let chart_text_properties = chart_resource.chart_space.text_properties.as_deref();
      let title_text_style_context = ChartTextStyleContext {
        import,
        slide,
        default_properties: chart_text_properties,
        theme_language: if matches!(
          chart.title.as_ref(),
          Some(shared_chart::ChartTitleText::Automatic)
        ) {
          ui_language
        } else {
          default_document_language
        },
      };
      let label_text_style_context = ChartTextStyleContext {
        theme_language: default_document_language,
        ..title_text_style_context
      };
      let category_axis_title_style = powerpoint_chart_axis_title_style(
        label_text_style_context,
        shared_chart::category_axis_title_source(&chart),
      );
      let value_axis_title_style = powerpoint_chart_axis_title_style(
        label_text_style_context,
        shared_chart::value_axis_title_source(&chart),
      );
      let additional_axis_title_styles = chart
        .additional_axis_titles
        .iter()
        .map(|title| {
          powerpoint_chart_axis_title_style(
            label_text_style_context,
            Some((title.source, title.automatic_rotation_deg)),
          )
        })
        .collect();
      let data_label_style = chart_text_style(
        label_text_style_context,
        data_label_properties,
        POWERPOINT_CHART_LABEL_FALLBACK,
        None,
      );
      let (data_label_styles, data_label_rich_text_styles): (Vec<_>, Vec<_>) = chart
        .series
        .iter()
        .map(|series| {
          chart_data_label_host_styles(
            label_text_style_context,
            &series.data_labels,
            &data_label_style,
          )
        })
        .unzip();
      let gridline_color = chart
        .value_axis
        .and_then(|axis| axis.major_gridlines.as_deref())
        .and_then(|gridlines| gridlines.chart_shape_properties.as_deref())
        .and_then(shared_chart::chart_shape_outline_solid_fill)
        .and_then(|fill| display_paint_for_chart(import, slide, chart_resource, fill))
        .map(|paint| paint.color)
        // With no explicit c:spPr, Office's automatic chart style emits a
        // 0.525 neutral-gray, 0.75pt grid stroke (Chart_2D.pptx fixed-format
        // content stream). Keep that source color; raster antialiasing is
        // applied later by the PDF consumer.
        .unwrap_or(RgbColor {
          r: 134,
          g: 134,
          b: 134,
        });
      let mut chart_items = lower_clustered_column_chart(
        ChartFrame {
          x_pt: x,
          y_pt: y,
          width_pt: width,
          height_pt: height,
        },
        &chart,
        shared_chart::automatic_chart_title(ui_language),
        &ClusteredColumnStyle {
          layout_profile: ChartLayoutProfile::PowerPoint,
          chart_style_id: shared_chart::chart_style_id(&chart_resource.chart_space).unwrap_or(2),
          modern_excel_profile: false,
          stroke_scale: 1.0,
          automatic_line_width_pt: powerpoint_chart_automatic_line_width_pt(
            import,
            &chart_resource.chart_space,
          ),
          has_explicit_title: chart_resource
            .chart_space
            .chart
            .title
            .as_deref()
            .is_some_and(|title| title.chart_text.is_some()),
          title_top_adjustment_ratio: 0.0,
          title: chart_text_style(
            title_text_style_context,
            title_properties,
            POWERPOINT_CHART_TITLE_FALLBACK,
            title,
          ),
          title_fill_color,
          label: chart_text_style(
            label_text_style_context,
            label_properties,
            POWERPOINT_CHART_LABEL_FALLBACK,
            None,
          ),
          legend: chart_text_style(
            label_text_style_context,
            legend_properties,
            POWERPOINT_CHART_LABEL_FALLBACK,
            None,
          ),
          category_axis_title: category_axis_title_style,
          value_axis_title: value_axis_title_style,
          additional_axis_titles: additional_axis_title_styles,
          category_label: chart_text_style(
            label_text_style_context,
            category_label_properties,
            POWERPOINT_CHART_LABEL_FALLBACK,
            None,
          ),
          value_label: chart_text_style(
            label_text_style_context,
            value_label_properties,
            POWERPOINT_CHART_LABEL_FALLBACK,
            None,
          ),
          series_label: chart_text_style(
            label_text_style_context,
            series_label_properties,
            POWERPOINT_CHART_LABEL_FALLBACK,
            None,
          ),
          data_label: data_label_style,
          data_label_styles,
          data_label_rich_text_styles,
          gridline_color,
          value_gridline_width_pt: None,
          axis_line_width_pt: None,
          category_major_gridline: None,
          category_minor_gridline: None,
          series_colors,
          series_point_colors,
          series_styles,
          trendline_styles,
          error_bar_styles,
          group_decoration_styles,
          series_point_styles,
          surface_band_colors,
          data_label_fill_colors: chart
            .series
            .iter()
            .map(|series| {
              series
                .data_labels
                .iter()
                .map(|label| {
                  label
                    .shape_properties
                    .and_then(shared_chart::chart_shape_solid_fill)
                    .and_then(|fill| {
                      display_paint_for_chart(import, slide, chart_resource, fill)
                        .map(|paint| paint.color)
                    })
                })
                .collect()
            })
            .collect(),
          legend_frame_style: pptx_chart_shape_style(
            import,
            slide,
            chart_resource,
            chart_resource
              .chart_space
              .chart
              .legend
              .as_deref()
              .and_then(|legend| legend.chart_shape_properties.as_deref()),
          ),
          chart_area_style: pptx_chart_area_shape_style(
            import,
            slide,
            chart_resource,
            chart_resource.chart_space.shape_properties.as_deref(),
          ),
          plot_area_style: pptx_chart_area_shape_style(
            import,
            slide,
            chart_resource,
            chart_resource
              .chart_space
              .chart
              .plot_area
              .shape_properties
              .as_deref(),
          ),
          floor_style: pptx_chart_area_shape_style(
            import,
            slide,
            chart_resource,
            chart_resource
              .chart_space
              .chart
              .floor
              .as_deref()
              .and_then(|floor| floor.shape_properties.as_deref()),
          ),
          side_wall_style: pptx_chart_area_shape_style(
            import,
            slide,
            chart_resource,
            chart_resource
              .chart_space
              .chart
              .side_wall
              .as_deref()
              .and_then(|wall| wall.shape_properties.as_deref()),
          ),
          back_wall_style: pptx_chart_area_shape_style(
            import,
            slide,
            chart_resource,
            chart_resource
              .chart_space
              .chart
              .back_wall
              .as_deref()
              .and_then(|wall| wall.shape_properties.as_deref()),
          ),
        },
      );
      if !chart_items.is_empty() {
        if let (Some(fill), Some(shared_chart::ChartTitleText::Explicit(title))) =
          (title_blip_fill, chart.title.as_ref())
        {
          insert_chart_title_blip_fill(
            import,
            slide,
            chart_resource,
            fill,
            title,
            &mut chart_items,
          );
        }
        items.extend(chart_items);
        return;
      }
    }
  }

  let paints = chart_data_point_paints(import, slide, chart_resource);
  let texts =
    shared_chart::fixed_output_texts_for_ui_language(&chart_resource.chart_space, ui_language);
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

fn lower_extended_chart(
  import: &PowerPointImport,
  slide: &SlidePersist,
  resource: &super::slide::ExtendedChartResource,
  shape: &Shape,
  offset: DisplayOffset,
  locales: &OfficeLocaleContext,
  items: &mut Vec<PageItem>,
) {
  let ui_language = locales.ui_language();
  let default_document_language = locales.default_document_language();
  let frame = ChartFrame {
    x_pt: offset.x_pt(shape.position.x),
    y_pt: offset.y_pt(shape.position.y),
    width_pt: offset.width_pt(shape.size.cx),
    height_pt: offset.height_pt(shape.size.cy),
  };
  // A missing ChartEx title is an application-generated UI resource. Its
  // fallback face follows that resource language; cached labels and values
  // remain document content and use the authoring-language theme fallback.
  let theme = chart_theme(import, slide);
  let resolve_font = |placeholder, language| {
    theme
      .and_then(|theme| {
        theme
          .font_scheme
          .resolve_font_for_language(placeholder, language)
      })
      .or_else(|| import.resolve_theme_font_for_language(placeholder, language))
  };
  let title_language = ui_language.or(default_document_language);
  let title_font = resolve_font("+mn-lt", title_language).unwrap_or("Liberation Sans");
  let label_font = resolve_font("+mn-lt", default_document_language).unwrap_or("Liberation Sans");
  let title_east_asia_font = resolve_font("+mn-ea", title_language);
  let label_east_asia_font = resolve_font("+mn-ea", default_document_language);
  let title_style = TextStyle {
    font_family: Some(Arc::from(title_font)),
    east_asia_font_family: title_east_asia_font.map(Arc::from),
    font_size_pt: 14.0,
    bold: true,
    ..TextStyle::default()
  };
  let label_style = TextStyle {
    font_family: Some(Arc::from(label_font)),
    east_asia_font_family: label_east_asia_font.map(Arc::from),
    font_size_pt: 9.0,
    ..TextStyle::default()
  };
  let chart_styles = resource
    .style_resources
    .iter()
    .map(|resource| resource.style.clone())
    .collect::<Vec<_>>();
  let color_styles = resource
    .color_style_resources
    .iter()
    .map(|resource| resource.colors.clone())
    .collect::<Vec<_>>();
  items.extend(crate::render::chartex::lower_extended_chart(
    &resource.chart_space,
    crate::render::chartex::ChartExRenderOptions {
      host: crate::render::chartex::ChartExHost::PowerPoint,
      frame,
      title_style,
      label_style,
      ui_language,
      theme: pptx_chartex_theme(import, slide),
      resources: crate::render::chartex::ChartExStyleResources {
        chart_styles: &chart_styles,
        color_styles: &color_styles,
      },
    },
    None,
  ));
}

fn insert_chart_title_blip_fill(
  import: &PowerPointImport,
  slide: &SlidePersist,
  chart_resource: &ChartResource,
  fill: &a::BlipFill,
  title: &str,
  items: &mut Vec<PageItem>,
) {
  let Some(blip) = fill.blip.as_deref() else {
    return;
  };
  let Some(relationship_id) = blip.embed.as_deref() else {
    return;
  };
  let Some(resource) = chart_resource.image_resources.get(relationship_id) else {
    return;
  };
  let Some((title_index, text)) = items
    .iter()
    .enumerate()
    .find_map(|(index, item)| match item {
      PageItem::Text(text) if text.text == title => Some((index, text)),
      _ => None,
    })
  else {
    return;
  };
  let mut metrics = TextMetrics::new();
  let text_width = metrics.measure_text(&text.text, &text.style);
  let horizontal_padding = text.style.font_size_pt * 0.162;
  let vertical_padding = text.style.font_size_pt * 0.092;
  let frame = TextFrame {
    x_pt: text.x_pt - horizontal_padding,
    y_pt: text.y_pt - vertical_padding,
    width_pt: text_width + horizontal_padding * 2.0,
    height_pt: text.line_height_pt + vertical_padding * 2.0,
  };
  let images = blip_fill_image_items_from_resource(
    import,
    slide,
    fill,
    blip,
    resource,
    ImageFillPlacement {
      frame,
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
  .map(PageItem::Image);
  items.splice(title_index..title_index, images);
}

#[derive(Clone, Copy)]
struct ChartTextStyleContext<'a> {
  import: &'a PowerPointImport,
  slide: &'a SlidePersist,
  default_properties: Option<&'a c::TextProperties>,
  theme_language: Option<&'a str>,
}

#[derive(Clone, Copy)]
struct ChartTextFallback {
  size_pt: f32,
  bold: bool,
  automatic_title_scale: bool,
}

const POWERPOINT_CHART_TITLE_FALLBACK: ChartTextFallback = ChartTextFallback {
  size_pt: 18.0,
  bold: true,
  automatic_title_scale: true,
};
const POWERPOINT_CHART_LABEL_FALLBACK: ChartTextFallback = ChartTextFallback {
  size_pt: 12.0,
  bold: false,
  automatic_title_scale: false,
};
const POWERPOINT_CHART_AXIS_TITLE_FALLBACK: ChartTextFallback = ChartTextFallback {
  size_pt: 12.0,
  bold: true,
  automatic_title_scale: false,
};
const POWERPOINT_AUTOMATIC_CHART_TITLE_SCALE: f32 = 1.2;

fn chart_text_style(
  context: ChartTextStyleContext<'_>,
  properties: Option<&c::TextProperties>,
  fallback: ChartTextFallback,
  rich_title: Option<&c::Title>,
) -> TextStyle {
  let title_has_explicit_size = properties.is_some_and(|properties| {
    properties
      .paragraph
      .iter()
      .filter_map(|paragraph| paragraph.paragraph_properties.as_deref())
      .filter_map(|paragraph| paragraph.default_run_properties.as_deref())
      .any(|properties| properties.font_size.is_some())
  }) || rich_title
    .and_then(|title| title.chart_text.as_deref())
    .and_then(|text| text.chart_text_choice.as_ref())
    .and_then(|choice| match choice {
      c::ChartTextChoice::RichText(rich) => rich.paragraph.first(),
      _ => None,
    })
    .is_some_and(|paragraph| {
      paragraph
        .paragraph_properties
        .as_deref()
        .and_then(|properties| properties.default_run_properties.as_deref())
        .is_some_and(|properties| properties.font_size.is_some())
        || paragraph
          .paragraph_choice
          .iter()
          .any(|choice| match choice {
            a::ParagraphChoice::Run(run) => run
              .run_properties
              .as_deref()
              .is_some_and(|properties| properties.font_size.is_some()),
            a::ParagraphChoice::Field(field) => field
              .run_properties
              .as_deref()
              .is_some_and(|properties| properties.font_size.is_some()),
            _ => false,
          })
    });
  let mut style = TextStyle {
    font_family: Some(Arc::from(
      context
        .import
        .resolve_theme_font_for_language("+mn-lt", context.theme_language)
        .unwrap_or("Liberation Sans"),
    )),
    font_size_pt: fallback.size_pt,
    bold: fallback.bold,
    use_windows_font_metrics: true,
    ..TextStyle::default()
  };
  for text_properties in [context.default_properties, properties]
    .into_iter()
    .flatten()
  {
    if let Some(default_run_properties) = text_properties
      .paragraph
      .iter()
      .filter_map(|paragraph| paragraph.paragraph_properties.as_deref())
      .find_map(|paragraph| paragraph.default_run_properties.as_deref())
    {
      apply_default_run_properties(
        context.import,
        Some(context.slide),
        default_run_properties,
        &mut style,
      );
    }
  }
  if let Some(c::ChartTextChoice::RichText(rich)) = rich_title
    .and_then(|title| title.chart_text.as_deref())
    .and_then(|text| text.chart_text_choice.as_ref())
    && let Some(paragraph) = rich.paragraph.first()
  {
    if let Some(properties) = paragraph
      .paragraph_properties
      .as_deref()
      .and_then(|properties| properties.default_run_properties.as_deref())
    {
      apply_default_run_properties(context.import, Some(context.slide), properties, &mut style);
    }
    if let Some(properties) = paragraph
      .paragraph_choice
      .iter()
      .find_map(|choice| match choice {
        a::ParagraphChoice::Run(run) => run.run_properties.as_deref(),
        a::ParagraphChoice::Field(field) => field.run_properties.as_deref(),
        a::ParagraphChoice::Break(_)
        | a::ParagraphChoice::TextMath(_)
        | a::ParagraphChoice::AlternateContent(_) => None,
      })
    {
      apply_run_common(
        context.import,
        RunCommon {
          language: properties.language.as_deref(),
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
        &mut style,
      );
      if let Some(fill) = properties.run_properties_choice1.as_ref() {
        apply_text_fill(context.import, Some(context.slide), fill, &mut style);
      }
      if let Some(effect) = properties.run_properties_choice2.as_ref() {
        style.drawingml_text_effects = Some(drawingml_run_effects(
          context.import,
          Some(context.slide),
          effect,
        ));
      }
    }
  }
  if let Some(typeface) = context
    .import
    .resolve_theme_font_for_language("+mn-ea", context.theme_language)
  {
    style.east_asia_font_family = Some(Arc::from(typeface));
  }
  if fallback.automatic_title_scale && !title_has_explicit_size {
    // PowerPoint's automatic chart-title style promotes the chart-space text
    // default by 120%. A direct title txPr or rich run size replaces that
    // automatic style size rather than being scaled again.
    style.font_size_pt *= POWERPOINT_AUTOMATIC_CHART_TITLE_SCALE;
  }
  style.font_size_pt = units::quantize_points_to_office_print_grid(style.font_size_pt);
  style
}

fn powerpoint_chart_axis_title_style(
  context: ChartTextStyleContext<'_>,
  source: Option<(&c::Title, f32)>,
) -> TextStyle {
  let Some((title, automatic_rotation_deg)) = source else {
    return chart_text_style(context, None, POWERPOINT_CHART_AXIS_TITLE_FALLBACK, None);
  };
  let mut style = chart_text_style(
    context,
    title.text_properties.as_deref(),
    POWERPOINT_CHART_AXIS_TITLE_FALLBACK,
    Some(title),
  );
  style.rotation_deg =
    shared_chart::title_rotation_degrees(title).unwrap_or(automatic_rotation_deg);
  style
}

fn chart_data_label_host_styles(
  context: ChartTextStyleContext<'_>,
  labels: &[shared_chart::ClusteredColumnDataLabel<'_>],
  base_style: &TextStyle,
) -> (Vec<Option<TextStyle>>, Vec<Vec<TextStyle>>) {
  let mut label_styles = Vec::with_capacity(labels.len());
  let mut rich_text_styles = Vec::with_capacity(labels.len());
  for label in labels {
    let label_style = label.text_properties.map_or_else(
      || base_style.clone(),
      |properties| {
        chart_text_style(
          context,
          Some(properties),
          POWERPOINT_CHART_LABEL_FALLBACK,
          None,
        )
      },
    );
    label_styles.push(label.text_properties.is_some().then(|| label_style.clone()));
    rich_text_styles.push(
      label
        .rich_text_runs
        .iter()
        .map(|run| {
          let mut style = label_style.clone();
          if let Some(properties) = run.paragraph_default_run_properties {
            apply_default_run_properties(
              context.import,
              Some(context.slide),
              properties,
              &mut style,
            );
          }
          if let Some(properties) = run.run_properties {
            apply_drawingml_run_properties(
              context.import,
              Some(context.slide),
              properties,
              &mut style,
            );
          }
          style.font_size_pt = units::quantize_points_to_office_print_grid(style.font_size_pt);
          style
        })
        .collect(),
    );
  }
  (label_styles, rich_text_styles)
}

fn display_color_for_chart_series(
  import: &PowerPointImport,
  slide: &SlidePersist,
  chart_resource: &ChartResource,
  chart_style_id: u8,
  formatting_index: usize,
  maximum_formatting_index: usize,
) -> Option<RgbColor> {
  let theme = chart_theme(import, slide);
  let color_map = chart_resource.chart_space.color_map_override.as_deref();
  shared_chart::automatic_chart_series_color(
    chart_style_id,
    formatting_index,
    maximum_formatting_index,
    |token| {
      let mapped = shared_chart::scheme_color_token(color_map, token)?;
      theme
        .and_then(|theme| {
          let color = theme.color_scheme.get_color(mapped)?.clone();
          let mut scheme_resolver = |nested| {
            let mapped = shared_chart::scheme_color_token(color_map, nested)?;
            theme.color_scheme.get_color(mapped).cloned()
          };
          let resolved = color.resolve_rgb(&mut scheme_resolver, None)?;
          Some(RgbColor {
            r: resolved.r,
            g: resolved.g,
            b: resolved.b,
          })
        })
        .or_else(|| powerpoint_default_chart_theme_color(mapped))
    },
  )
}

fn powerpoint_default_chart_theme_color(token: a::ColorSchemeIndexValues) -> Option<RgbColor> {
  match token {
    a::ColorSchemeIndexValues::Dark1 => Some(RgbColor { r: 0, g: 0, b: 0 }),
    a::ColorSchemeIndexValues::Light1 => Some(RgbColor {
      r: 0xff,
      g: 0xff,
      b: 0xff,
    }),
    a::ColorSchemeIndexValues::Dark2 => Some(RgbColor {
      r: 0x44,
      g: 0x54,
      b: 0x6a,
    }),
    a::ColorSchemeIndexValues::Light2 => Some(RgbColor {
      r: 0xe7,
      g: 0xe6,
      b: 0xe6,
    }),
    a::ColorSchemeIndexValues::Accent1 => Some(RgbColor {
      r: 0x44,
      g: 0x72,
      b: 0xc4,
    }),
    a::ColorSchemeIndexValues::Accent2 => Some(RgbColor {
      r: 0xed,
      g: 0x7d,
      b: 0x31,
    }),
    a::ColorSchemeIndexValues::Accent3 => Some(RgbColor {
      r: 0xa5,
      g: 0xa5,
      b: 0xa5,
    }),
    a::ColorSchemeIndexValues::Accent4 => Some(RgbColor {
      r: 0xff,
      g: 0xc0,
      b: 0x00,
    }),
    a::ColorSchemeIndexValues::Accent5 => Some(RgbColor {
      r: 0x5b,
      g: 0x9b,
      b: 0xd5,
    }),
    a::ColorSchemeIndexValues::Accent6 => Some(RgbColor {
      r: 0x70,
      g: 0xad,
      b: 0x47,
    }),
    a::ColorSchemeIndexValues::Hyperlink => Some(RgbColor {
      r: 0x05,
      g: 0x63,
      b: 0xc1,
    }),
    a::ColorSchemeIndexValues::FollowedHyperlink => Some(RgbColor {
      r: 0x95,
      g: 0x4f,
      b: 0x72,
    }),
  }
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
        paragraph_bidi: false,
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
  let color = fill
    .solid_fill_choice
    .as_ref()
    .and_then(Color::from_solid_fill_choice)?;
  display_paint_for_chart_color(import, slide, chart_resource, &color)
}

fn display_paint_for_chart_color(
  import: &PowerPointImport,
  slide: &SlidePersist,
  chart_resource: &ChartResource,
  color: &Color,
) -> Option<DisplayPaint> {
  let theme = chart_theme(import, slide)?;
  let color_map = chart_resource.chart_space.color_map_override.as_deref();
  let mut scheme_resolver = |token| {
    let mapped = shared_chart::scheme_color_token(color_map, token)?;
    theme.color_scheme.get_color(mapped).cloned()
  };
  let color = color.clone().resolve_rgb(&mut scheme_resolver, None)?;
  Some(DisplayPaint {
    color: RgbColor {
      r: color.r,
      g: color.g,
      b: color.b,
    },
    opacity: color_opacity(color.alpha),
  })
}

fn pptx_chart_shape_style(
  import: &PowerPointImport,
  slide: &SlidePersist,
  chart_resource: &ChartResource,
  properties: Option<&c::ChartShapeProperties>,
) -> common::ShapeStyle<'static> {
  let Some(properties) = properties else {
    return common::ShapeStyle::default();
  };
  let fill = match properties.chart_shape_properties_choice2.as_ref() {
    None => common::ShapeStyleValue::Unspecified,
    Some(c::ChartShapePropertiesChoice2::NoFill(_)) => common::ShapeStyleValue::NoPaint,
    Some(c::ChartShapePropertiesChoice2::SolidFill(fill)) => display_paint_for_chart(
      import,
      slide,
      chart_resource,
      fill,
    )
    .map_or(common::ShapeStyleValue::Unspecified, |paint| {
      common::ShapeStyleValue::Paint(common::Fill::Solid(common_rgb(paint.color, paint.opacity)))
    }),
    Some(c::ChartShapePropertiesChoice2::GradientFill(fill)) => {
      pptx_chart_gradient_fill(import, slide, chart_resource, fill).map_or(
        common::ShapeStyleValue::Unspecified,
        common::ShapeStyleValue::Paint,
      )
    }
    Some(c::ChartShapePropertiesChoice2::PatternFill(fill)) => {
      pptx_chart_pattern_fill(import, slide, chart_resource, fill)
        .map_or(common::ShapeStyleValue::Unspecified, |fill| {
          common::ShapeStyleValue::Paint(common::Fill::Pattern(fill))
        })
    }
    Some(c::ChartShapePropertiesChoice2::BlipFill(fill)) => {
      common::ShapeStyleValue::Paint(pptx_chart_blip_fill(fill))
    }
  };
  common::ShapeStyle {
    fill,
    stroke: pptx_chart_outline_style(import, slide, chart_resource, properties.outline.as_deref()),
  }
}

fn pptx_chart_area_shape_style(
  import: &PowerPointImport,
  slide: &SlidePersist,
  chart_resource: &ChartResource,
  properties: Option<&c::ShapeProperties>,
) -> common::ShapeStyle<'static> {
  let Some(properties) = properties else {
    return common::ShapeStyle::default();
  };
  let fill = match properties.shape_properties_choice2.as_ref() {
    None | Some(c::ShapePropertiesChoice2::GroupFill) => common::ShapeStyleValue::Unspecified,
    Some(c::ShapePropertiesChoice2::NoFill(_)) => common::ShapeStyleValue::NoPaint,
    Some(c::ShapePropertiesChoice2::SolidFill(fill)) => display_paint_for_chart(
      import,
      slide,
      chart_resource,
      fill,
    )
    .map_or(common::ShapeStyleValue::Unspecified, |paint| {
      common::ShapeStyleValue::Paint(common::Fill::Solid(common_rgb(paint.color, paint.opacity)))
    }),
    Some(c::ShapePropertiesChoice2::GradientFill(fill)) => {
      pptx_chart_gradient_fill(import, slide, chart_resource, fill).map_or(
        common::ShapeStyleValue::Unspecified,
        common::ShapeStyleValue::Paint,
      )
    }
    Some(c::ShapePropertiesChoice2::PatternFill(fill)) => {
      pptx_chart_pattern_fill(import, slide, chart_resource, fill)
        .map_or(common::ShapeStyleValue::Unspecified, |fill| {
          common::ShapeStyleValue::Paint(common::Fill::Pattern(fill))
        })
    }
    Some(c::ShapePropertiesChoice2::BlipFill(fill)) => {
      common::ShapeStyleValue::Paint(pptx_chart_blip_fill(fill))
    }
  };
  common::ShapeStyle {
    fill,
    stroke: pptx_chart_outline_style(import, slide, chart_resource, properties.outline.as_deref()),
  }
}

fn powerpoint_chart_automatic_line_width_pt(
  import: &PowerPointImport,
  chart_space: &c::ChartSpace,
) -> f32 {
  let subtle_theme_width = import
    .get_theme_line_style(1)
    .and_then(|line| line.width_emu)
    .map(units::emu_to_points)
    .unwrap_or(0.75);
  subtle_theme_width * shared_chart::automatic_linear_series_line_width_scale(chart_space)
}

fn pptx_chart_outline_style(
  import: &PowerPointImport,
  slide: &SlidePersist,
  chart_resource: &ChartResource,
  outline: Option<&a::Outline>,
) -> common::ShapeStyleValue<common::Stroke<'static>> {
  let Some(outline) = outline else {
    return common::ShapeStyleValue::Unspecified;
  };
  let (color, pattern, gradient) = match outline.outline_choice1.as_ref() {
    Some(a::OutlineChoice::NoFill(_)) => return common::ShapeStyleValue::NoPaint,
    Some(a::OutlineChoice::SolidFill(fill)) => {
      let Some(paint) = display_paint_for_chart(import, slide, chart_resource, fill) else {
        return common::ShapeStyleValue::Unspecified;
      };
      (common_rgb(paint.color, paint.opacity), None, None)
    }
    Some(a::OutlineChoice::PatternFill(fill)) => {
      let Some(pattern) = pptx_chart_pattern_fill(import, slide, chart_resource, fill) else {
        return common::ShapeStyleValue::Unspecified;
      };
      (pattern.foreground, Some(pattern), None)
    }
    Some(a::OutlineChoice::GradientFill(fill)) => {
      let Some(common::Fill::Gradient(gradient)) =
        pptx_chart_gradient_fill(import, slide, chart_resource, fill)
      else {
        return common::ShapeStyleValue::Unspecified;
      };
      let color = gradient
        .stops
        .first()
        .map(|stop| stop.color)
        .unwrap_or_default();
      (color, None, Some(gradient))
    }
    None => return common::ShapeStyleValue::Unspecified,
  };
  let mut stroke = common::Stroke {
    width: common::Pt(
      outline
        .width
        .map(|width| units::emu_to_points(i64::from(width)))
        .unwrap_or(0.75),
    ),
    color,
    pattern,
    gradient,
    ..Default::default()
  };
  common::drawingml_stroke::apply_outline_style(&mut stroke, outline);
  common::ShapeStyleValue::Paint(stroke)
}

fn pptx_chart_gradient_fill(
  import: &PowerPointImport,
  slide: &SlidePersist,
  chart_resource: &ChartResource,
  fill: &a::GradientFill,
) -> Option<common::Fill<'static>> {
  let mut stops = fill
    .gradient_stop_list
    .as_ref()?
    .gradient_stop
    .iter()
    .filter_map(|stop| {
      let color = Color::from_gradient_stop_choice(stop.gradient_stop_choice.as_ref()?)?;
      let paint = display_paint_for_chart_color(import, slide, chart_resource, &color)?;
      Some(common::GradientStop {
        position: stop.position.as_ratio() as f32,
        color: common_rgb(paint.color, paint.opacity),
        scheme: None,
      })
    })
    .collect::<Vec<_>>();
  super::gradient::normalize_powerpoint_gradient_stops(&mut stops);
  if stops.is_empty() {
    return None;
  }
  let (angle_degrees, scaled, path) = match fill.gradient_fill_choice.as_ref()? {
    a::GradientFillChoice::LinearGradientFill(linear) => (
      Some(linear.angle.unwrap_or_default() as f32 / 60_000.0),
      linear.scaled.as_ref().is_some_and(|value| value.as_bool()),
      None,
    ),
    a::GradientFillChoice::PathGradientFill(path) => (
      None,
      false,
      Some(common::drawingml_gradient::resolve_path_gradient(
        fill,
        path,
        common::Transform::default(),
      )),
    ),
  };
  Some(common::Fill::Gradient(common::GradientFill {
    stops,
    angle_degrees,
    definition_bounds: None,
    line: None,
    interpolation: common::GradientInterpolation::LinearSrgb,
    scaled,
    rotate_with_shape: fill.rotate_with_shape.as_ref().map(|value| value.as_bool()),
    path,
  }))
}

fn pptx_chart_pattern_fill(
  import: &PowerPointImport,
  slide: &SlidePersist,
  chart_resource: &ChartResource,
  fill: &a::PatternFill,
) -> Option<common::PatternFill> {
  let foreground = fill
    .foreground_color
    .as_ref()
    .and_then(|color| color.foreground_color_choice.as_ref())
    .and_then(Color::from_foreground_color_choice)
    .and_then(|color| display_paint_for_chart_color(import, slide, chart_resource, &color))
    .map(|paint| common_rgb(paint.color, paint.opacity))
    .unwrap_or(common::Color {
      r: 0,
      g: 0,
      b: 0,
      a: u8::MAX,
    });
  let background = fill
    .background_color
    .as_ref()
    .and_then(|color| color.background_color_choice.as_ref())
    .and_then(Color::from_background_color_choice)
    .and_then(|color| display_paint_for_chart_color(import, slide, chart_resource, &color))
    .map(|paint| common_rgb(paint.color, paint.opacity))
    .unwrap_or(common::Color {
      r: u8::MAX,
      g: u8::MAX,
      b: u8::MAX,
      a: u8::MAX,
    });
  Some(common::PatternFill::drawingml(
    common::drawingml_pattern::hatch_style(fill.preset),
    foreground,
    background,
  ))
}

fn pptx_chart_blip_fill(fill: &a::BlipFill) -> common::Fill<'static> {
  let relationship_id = fill.blip.as_deref().and_then(|blip| {
    blip
      .embed
      .as_deref()
      .or(blip.link.as_deref())
      .map(|id| Cow::Owned(id.to_string()))
  });
  common::Fill::Image {
    relationship_id,
    tile: matches!(fill.blip_fill_choice, Some(a::BlipFillChoice::Tile(_))),
  }
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

fn pptx_chartex_theme(
  import: &PowerPointImport,
  slide: &SlidePersist,
) -> crate::render::chartex::ChartExTheme {
  let defaults = crate::render::chartex::ChartExTheme::default();
  let Some(theme) = chart_theme(import, slide) else {
    return defaults;
  };
  let resolve = |token: a::SchemeColorValues| {
    let mapped = shared_chart::scheme_color_token(None, token)?;
    let color = theme.color_scheme.get_color(mapped)?.clone();
    let mut resolver = |nested| {
      let mapped = shared_chart::scheme_color_token(None, nested)?;
      theme.color_scheme.get_color(mapped).cloned()
    };
    let color = color.resolve_rgb(&mut resolver, None)?;
    Some(RgbColor {
      r: color.r,
      g: color.g,
      b: color.b,
    })
  };
  crate::render::chartex::ChartExTheme {
    dark1: resolve(a::SchemeColorValues::Dark1).unwrap_or(defaults.dark1),
    light1: resolve(a::SchemeColorValues::Light1).unwrap_or(defaults.light1),
    dark2: resolve(a::SchemeColorValues::Dark2).unwrap_or(defaults.dark2),
    light2: resolve(a::SchemeColorValues::Light2).unwrap_or(defaults.light2),
    accents: [
      resolve(a::SchemeColorValues::Accent1).unwrap_or(defaults.accents[0]),
      resolve(a::SchemeColorValues::Accent2).unwrap_or(defaults.accents[1]),
      resolve(a::SchemeColorValues::Accent3).unwrap_or(defaults.accents[2]),
      resolve(a::SchemeColorValues::Accent4).unwrap_or(defaults.accents[3]),
      resolve(a::SchemeColorValues::Accent5).unwrap_or(defaults.accents[4]),
      resolve(a::SchemeColorValues::Accent6).unwrap_or(defaults.accents[5]),
    ],
    hyperlink: resolve(a::SchemeColorValues::Hyperlink).unwrap_or(defaults.hyperlink),
    followed_hyperlink: resolve(a::SchemeColorValues::FollowedHyperlink)
      .unwrap_or(defaults.followed_hyperlink),
  }
}

#[derive(Clone, Copy)]
struct PptxLoweringContext<'a> {
  import: &'a PowerPointImport,
  slide: &'a SlidePersist,
  page_index: usize,
  locales: &'a OfficeLocaleContext,
  inherited_scene3d: Option<&'a a::Scene3DType>,
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
  let x_pt = offset.x_pt(shape.position.x);
  let y_pt = offset.y_pt(shape.position.y);
  let width_pt = offset.width_pt(shape.size.cx);
  let height_pt = offset.height_pt(shape.size.cy);
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
  let colors = diagram_style_colors(context.import, context.slide, record);
  if let Some(drawing) = diagram_drawing_resource(context.slide, &data_resource.model)
    && lower_diagram_drawing(
      context,
      drawing,
      &data_resource.model,
      colors.as_ref(),
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
    if diagram_shape.draw_geometry {
      let shape_content_start = drawing_items.len();
      let shape_bounds = shared_diagram::DiagramBounds {
        x: diagram_shape.x,
        y: diagram_shape.y,
        width: diagram_shape.width,
        height: diagram_shape.height,
      };
      let fill_images = diagram_shape
        .shape_properties
        .as_deref()
        .map(|properties| {
          diagram_model_shape_blip_fill_image_items(
            context.import,
            context.slide,
            data_resource,
            properties,
            shape_bounds,
          )
        })
        .unwrap_or_default();
      if fill_images.is_empty()
        && diagram_shape.is_blip_placeholder
        && let Some(item) = diagram_blip_placeholder_image_item(shape_bounds)
      {
        drawing_items.push(PageItem::Image(item));
      }
      drawing_items.extend(fill_images.into_iter().map(PageItem::Image));
      let explicit_fill = diagram_shape
        .shape_properties
        .as_deref()
        .and_then(|properties| {
          diagram_model_shape_common_fill(context.import, context.slide, properties, shape_bounds)
        });
      let default_fill = common::Fill::Solid(common_rgb(pdf_rgb_color(diagram_shape.fill), 1.0));
      let suppress_fill = diagram_shape
        .shape_properties
        .as_deref()
        .is_some_and(diagram_model_shape_suppresses_fill);
      if diagram_shape.is_connector
        && diagram_shape.connector_dimension == dgm::ConnectorDimensionValues::OneDimension
      {
        let mut stroke = diagram_shape
          .shape_properties
          .as_deref()
          .and_then(|properties| {
            diagram_model_shape_outline(context.import, context.slide, properties, shape_bounds)
          })
          .or_else(|| {
            diagram_style_outline(
              context.import,
              context.slide,
              diagram_shape.style.as_deref(),
              diagram_shape.line_fill.map(pdf_rgb_color),
            )
            .map(|stroke| common_stroke_from_border(stroke, 1.0))
          })
          .unwrap_or_else(|| common_stroke_from_border(BorderStyle::default(), 1.0));
        diagram_shape.apply_connector_ends(&mut stroke);
        drawing_items.push(PageItem::Path(common::PathItem {
          bounds: common_rect(
            diagram_shape.x,
            diagram_shape.y,
            diagram_shape.width,
            diagram_shape.height,
          ),
          points: Vec::new(),
          commands: diagram_shape.connector_commands(),
          closed: false,
          fill: common::Fill::None,
          stroke: Some(stroke),
        }));
      } else {
        let fill = diagram_shape
          .shape_properties
          .as_deref()
          .is_none_or(|properties| !diagram_model_shape_has_blip_fill(properties))
          .then(|| explicit_fill.unwrap_or(default_fill))
          .filter(|_| !suppress_fill)
          .unwrap_or(common::Fill::None);
        let stroke = diagram_shape
          .shape_properties
          .as_deref()
          .and_then(|properties| {
            diagram_model_shape_outline(context.import, context.slide, properties, shape_bounds)
          })
          .or_else(|| {
            diagram_style_outline(
              context.import,
              context.slide,
              diagram_shape.style.as_deref(),
              diagram_shape.line_fill.map(pdf_rgb_color),
            )
            .map(|stroke| common_stroke_from_border(stroke, 1.0))
          })
          .or_else(|| {
            (!suppress_fill).then(|| common_stroke_from_border(BorderStyle::default(), 1.0))
          });
        if let Some(paths) =
          diagram_model_shape_path_items(&diagram_shape, fill.clone(), stroke.clone())
        {
          drawing_items.extend(paths.into_iter().map(PageItem::Path));
        } else {
          drawing_items.push(PageItem::Path(diagram_fallback_rectangle_path(
            shape_bounds,
            fill,
            stroke,
          )));
        }
      }
      finish_diagram_model_shape_effects(
        context.import,
        context.slide,
        diagram_shape.shape_properties.as_deref(),
        shape_bounds,
        diagram_shape.shape_rotation_deg,
        &mut drawing_items,
        shape_content_start,
      );
    }
    if !diagram_shape.text_body.is_empty() {
      let text_body = diagram_text_body(&diagram_shape.text_body);
      let text_bounds = diagram_model_shape_text_rectangle(&diagram_shape).unwrap_or(
        shared_diagram::DiagramBounds {
          x: diagram_shape.x,
          y: diagram_shape.y,
          width: diagram_shape.width,
          height: diagram_shape.height,
        },
      );
      let text_frame = text_body_frame(
        text_bounds.x,
        text_bounds.y,
        text_bounds.width,
        text_bounds.height,
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
      let sync_auto_fit = text_body.display_properties.auto_fit == TextAutoFit::Shape;
      let (font_scale, line_scale) = if sync_auto_fit {
        diagram_text_auto_fit_scales(
          DiagramTextMeasurement {
            import: context.import,
            page_index: context.page_index,
            frame: text_frame,
            text_body: &text_body,
            font_reference: font_reference.as_ref(),
            base_font_size_pt: diagram_shape.font_size_pt,
          },
          diagram_shape.minimum_font_size_pt,
          &options,
        )
      } else {
        text_auto_fit_scales(&options)
      };
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
        shape_rotation_deg: diagram_shape.text_rotation_deg,
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
      context.page_index,
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
        shape_rotation_deg: pending.shape_rotation_deg,
      },
      summary.as_deref_mut(),
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
  colors: Option<&shared_diagram::DiagramStyleColors>,
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
  let text_fills = shared_diagram::presentation_point_text_fills(data, colors);
  let drawing_context = DiagramDrawingLoweringContext {
    import: context.import,
    slide: context.slide,
    drawing_resource,
    text_orders: &text_orders,
    text_fills: &text_fills,
    page_index: context.page_index,
  };
  let drawing_bounds = shared_diagram::DiagramBounds {
    x: frame.x_pt,
    y: frame.y_pt,
    width: frame.width_pt,
    height: frame.height_pt,
  };
  let root_group_fill = diagram_group_common_fill(
    context.import,
    context.slide,
    &drawing_resource.drawing.shape_tree.group_shape_properties,
    drawing_bounds,
    None,
  );
  let mut drawing_items = Vec::new();
  let mut text_items = Vec::new();
  for choice in &drawing_resource.drawing.shape_tree.shape_tree_choice {
    match choice {
      dsp::ShapeTreeChoice::Shape(shape) => lower_diagram_drawing_shape(
        drawing_context,
        shape,
        transform,
        root_group_fill.as_ref(),
        &mut drawing_items,
        &mut text_items,
        summary.as_deref_mut(),
      ),
      dsp::ShapeTreeChoice::GroupShape(group) => lower_diagram_drawing_group(
        drawing_context,
        group,
        transform,
        root_group_fill.as_ref(),
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
  text_fills: &'a HashMap<String, RgbColor>,
  page_index: usize,
}

fn lower_diagram_drawing_group(
  context: DiagramDrawingLoweringContext<'_>,
  group: &dsp::GroupShape,
  parent_transform: DiagramDrawingTransform,
  inherited_group_fill: Option<&common::Fill<'static>>,
  items: &mut Vec<PageItem>,
  text_items: &mut Vec<DiagramDrawingTextItem>,
  mut summary: Option<&mut PptxLayoutSummary>,
) {
  let group_bounds = diagram_group_bounds(&group.group_shape_properties, parent_transform);
  let group_fill = group_bounds
    .and_then(|bounds| {
      diagram_group_common_fill(
        context.import,
        context.slide,
        &group.group_shape_properties,
        bounds,
        inherited_group_fill,
      )
    })
    .or_else(|| {
      matches!(
        group
          .group_shape_properties
          .group_shape_properties_choice1
          .as_ref(),
        Some(dsp::GroupShapePropertiesChoice::GroupFill)
      )
      .then(|| inherited_group_fill.cloned())
      .flatten()
    });
  let transform =
    parent_transform.for_group(group.group_shape_properties.transform_group.as_deref());
  for choice in &group.group_shape_choice {
    match choice {
      dsp::GroupShapeChoice::Shape(shape) => lower_diagram_drawing_shape(
        context,
        shape,
        transform,
        group_fill.as_ref(),
        items,
        text_items,
        summary.as_deref_mut(),
      ),
      dsp::GroupShapeChoice::GroupShape(group) => lower_diagram_drawing_group(
        context,
        group,
        transform,
        group_fill.as_ref(),
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
  group_fill: Option<&common::Fill<'static>>,
  items: &mut Vec<PageItem>,
  text_items: &mut Vec<DiagramDrawingTextItem>,
  mut summary: Option<&mut PptxLayoutSummary>,
) {
  let Some(bounds) = diagram_shape_bounds(&shape.shape_properties, transform) else {
    return;
  };
  let shape_content_start = items.len();
  if let Some(summary) = summary.as_deref_mut() {
    record_diagram_draw_shape_summary(summary, context.page_index, shape, bounds, transform);
  }
  let explicit_fill = diagram_shape_common_fill(
    context.import,
    context.slide,
    &shape.shape_properties,
    bounds,
    group_fill,
  );
  let fill_images = diagram_shape_blip_fill_image_items(
    context.import,
    context.slide,
    context.drawing_resource,
    &shape.shape_properties,
    bounds,
  );
  items.extend(fill_images.into_iter().map(PageItem::Image));
  let fill = if !diagram_shape_has_blip_fill(&shape.shape_properties)
    && !diagram_shape_suppresses_fill(&shape.shape_properties)
  {
    {
      explicit_fill.unwrap_or(common::Fill::Solid(common_rgb(
        RgbColor {
          r: 255,
          g: 255,
          b: 255,
        },
        1.0,
      )))
    }
  } else {
    common::Fill::None
  };
  let stroke = shape
    .shape_properties
    .outline
    .as_deref()
    .and_then(|outline| diagram_outline(context.import, context.slide, outline, bounds));
  if let Some(paths) = diagram_drawing_shape_path_items(
    &shape.shape_properties,
    bounds,
    fill.clone(),
    stroke.clone(),
  ) {
    items.extend(paths.into_iter().map(PageItem::Path));
  } else {
    items.push(PageItem::Path(diagram_drawing_fallback_rectangle_path(
      &shape.shape_properties,
      bounds,
      fill,
      stroke,
    )));
  }
  finish_diagram_drawing_shape_effects(
    context.import,
    context.slide,
    &shape.shape_properties,
    bounds,
    items,
    shape_content_start,
  );
  let Some(text_body) = shape.text_body.as_deref() else {
    return;
  };
  let mut text_body = TextBody::from_diagram_drawing(text_body);
  if diagram_text_body_is_symbol_only(&text_body)
    && let Some(scene3_d) = shape.shape_properties.scene3_d_type.as_deref()
  {
    text_body
      .body_properties
      .get_or_insert_with(|| Box::new(a::BodyProperties::default()))
      .scene3_d_type = Some(Box::new(scene3_d.clone()));
    text_body.has_body_properties = true;
  }
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
  let font_reference = shape.shape_style.as_deref().map(|style| {
    diagram_font_style_reference(
      &style.font_reference,
      context.text_fills.get(shape.model_id.as_str()).copied(),
    )
  });
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
      if text_frame.rotation_deg.abs() > f32::EPSILON {
        text.style.rotation_deg += text_frame.rotation_deg;
        text.rotation_center_pt = text_frame.rotation_center_pt;
      }
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
  shape_rotation_deg: f32,
}

fn lower_diagram_text_body_at_with_style_and_scale(
  import: &PowerPointImport,
  page_index: usize,
  frame: TextFrame,
  text_body: &TextBody,
  style_inputs: DiagramTextLoweringStyle<'_>,
  mut summary: Option<&mut PptxLayoutSummary>,
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
    a::TextAnchoringTypeValues::Center
    | a::TextAnchoringTypeValues::Justified
    | a::TextAnchoringTypeValues::Distributed => {
      frame.y_pt + (frame.height_pt - estimated_height) / 2.0
    }
    a::TextAnchoringTypeValues::Bottom => frame.y_pt + frame.height_pt - estimated_height,
    a::TextAnchoringTypeValues::Top => frame.y_pt,
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
        page_index,
        slide_number: 1,
        paragraph_count: text_body.paragraphs.len(),
      },
      paragraph,
      paragraph_index,
      ParagraphLoweringOutput {
        summary: summary.as_deref_mut(),
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
        if style_inputs.shape_rotation_deg.abs() > f32::EPSILON {
          text_item.style.rotation_deg += style_inputs.shape_rotation_deg;
          text_item.rotation_center_pt = Some((
            frame.x_pt + frame.width_pt / 2.0,
            frame.y_pt + frame.height_pt / 2.0,
          ));
        }
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
  shape_rotation_deg: f32,
}

#[derive(Clone, Copy)]
struct DiagramTextMeasurement<'a> {
  import: &'a PowerPointImport,
  page_index: usize,
  frame: TextFrame,
  text_body: &'a TextBody,
  font_reference: Option<&'a FontStyleReference>,
  base_font_size_pt: Option<f32>,
}

fn diagram_text_auto_fit_scales(
  measurement: DiagramTextMeasurement<'_>,
  minimum_font_size_pt: Option<f32>,
  options: &TextLoweringOptions,
) -> (f32, f32) {
  let (initial_font_scale, line_scale) = text_auto_fit_scales(options);
  if measurement.frame.width_pt <= f32::EPSILON || measurement.frame.height_pt <= f32::EPSILON {
    return (initial_font_scale, line_scale);
  }

  // ECMA-376 Part 1 §21.4.7.1 requires the diagram tx algorithm to size text
  // to fit its shape. LibreOffice's DiagramLayoutAtom enables AUTOFIT for
  // default SmartArt text, then SdrTextObj::setupAutoFitText() measures the
  // fixed text box and rounds the resulting font size to whole points.
  let Some((required_width, required_height)) =
    diagram_text_required_size(measurement, initial_font_scale, line_scale)
  else {
    return (initial_font_scale, line_scale);
  };
  let fit = (measurement.frame.width_pt / required_width)
    .min(measurement.frame.height_pt / required_height)
    .min(1.0);
  let minimum_scale = minimum_font_size_pt
    .zip(measurement.base_font_size_pt)
    .filter(|(_, base)| *base > f32::EPSILON)
    .map(|(minimum, base)| minimum.max(MINIMUM_TEXT_FONT_SIZE_PT) / base)
    .or_else(|| {
      measurement
        .base_font_size_pt
        .filter(|base| *base > f32::EPSILON)
        .map(|base| MINIMUM_TEXT_FONT_SIZE_PT / base)
    })
    .unwrap_or(0.0);
  let mut fitted_scale = measurement
    .base_font_size_pt
    .filter(|base| *base > f32::EPSILON)
    .map(|base| (base * initial_font_scale * fit).round() / base)
    .unwrap_or(initial_font_scale * fit)
    .max(minimum_scale);

  // Whole-point rounding is applied independently to SmartArt's primary and
  // secondary font sizes. Re-measure the rounded result and step the primary
  // size down until the actual line boxes fit, matching EditEngine's
  // format-and-retry autofit loop instead of trusting a continuous ratio.
  if let Some(base_font_size_pt) = measurement
    .base_font_size_pt
    .filter(|base| *base > f32::EPSILON)
  {
    loop {
      let fits = diagram_text_required_size(
        DiagramTextMeasurement {
          base_font_size_pt: Some(base_font_size_pt),
          ..measurement
        },
        fitted_scale,
        line_scale,
      )
      .is_none_or(|(width, height)| {
        width <= measurement.frame.width_pt && height <= measurement.frame.height_pt
      });
      if fits || fitted_scale <= minimum_scale + f32::EPSILON {
        break;
      }
      let current_font_size_pt = (base_font_size_pt * fitted_scale).round();
      let next_font_size_pt = (current_font_size_pt - 1.0).max(
        minimum_font_size_pt
          .unwrap_or(MINIMUM_TEXT_FONT_SIZE_PT)
          .max(MINIMUM_TEXT_FONT_SIZE_PT),
      );
      if next_font_size_pt >= current_font_size_pt {
        break;
      }
      fitted_scale = next_font_size_pt / base_font_size_pt;
    }
  }
  (fitted_scale, line_scale)
}

fn diagram_text_required_size(
  measurement: DiagramTextMeasurement<'_>,
  font_scale: f32,
  line_scale: f32,
) -> Option<(f32, f32)> {
  let mut probe_items = Vec::new();
  lower_diagram_text_body_at_with_style_and_scale(
    measurement.import,
    measurement.page_index,
    measurement.frame,
    measurement.text_body,
    DiagramTextLoweringStyle {
      font_reference: measurement.font_reference,
      table_text_style: None,
      shape_hyperlink_url: None,
      base_font_size_pt: measurement.base_font_size_pt,
      font_scale,
      line_scale,
      shape_order: 0,
      shape_rotation_deg: 0.0,
    },
    None,
    &mut probe_items,
  );
  let mut text_metrics = TextMetrics::new();
  let mut left = f32::INFINITY;
  let mut top = f32::INFINITY;
  let mut right = f32::NEG_INFINITY;
  let mut bottom = f32::NEG_INFINITY;
  for probe in &probe_items {
    let PageItem::Text(text) = &probe.item else {
      continue;
    };
    let width = text_metrics.measure_text(text.text.as_str(), &text.style);
    left = left.min(text.x_pt);
    top = top.min(text.y_pt);
    right = right.max(text.x_pt + width);
    bottom = bottom.max(text.y_pt + text.line_height_pt);
  }
  if !left.is_finite() || !top.is_finite() || !right.is_finite() || !bottom.is_finite() {
    return None;
  }
  Some((
    (right - left).max(f32::EPSILON),
    (bottom - top).max(f32::EPSILON),
  ))
}

fn diagram_model_shape_path_items(
  shape: &shared_diagram::DiagramShape,
  fill: common::Fill<'static>,
  stroke: Option<common::Stroke<'static>>,
) -> Option<Vec<common::PathItem<'static>>> {
  let bounds = shared_diagram::DiagramBounds {
    x: shape.x,
    y: shape.y,
    width: shape.width,
    height: shape.height,
  };
  Some(
    shape
      .drawing_paths()?
      .into_iter()
      .map(|path| {
        let closed = path
          .commands
          .iter()
          .any(|command| matches!(command, common::PathCommand::Close));
        common::PathItem {
          bounds: common_rect(bounds.x, bounds.y, bounds.width, bounds.height),
          points: Vec::new(),
          commands: path.commands,
          closed,
          fill: path.fill_mode.apply_to_fill(fill.clone()),
          stroke: if path.stroke { stroke.clone() } else { None },
        }
      })
      .collect(),
  )
}

fn diagram_model_shape_text_rectangle(
  shape: &shared_diagram::DiagramShape,
) -> Option<shared_diagram::DiagramBounds> {
  let shape_bounds = shared_diagram::DiagramBounds {
    x: shape.x,
    y: shape.y,
    width: shape.width,
    height: shape.height,
  };
  let preset = match shape
    .shape_properties
    .as_deref()
    .and_then(|properties| properties.shape_properties_choice1.as_ref())
  {
    Some(dgm::ShapePropertiesChoice::PresetGeometry(preset)) => preset.as_ref(),
    Some(dgm::ShapePropertiesChoice::CustomGeometry(_)) => return None,
    None => shape.preset_geometry.as_deref()?,
  };
  let text_bounds = preset_text_rectangle(preset, shape_bounds)?;
  if shape.shape_rotation_deg.rem_euclid(360.0).abs() <= f32::EPSILON {
    return Some(text_bounds);
  }
  let center_x = shape_bounds.x + shape_bounds.width / 2.0;
  let center_y = shape_bounds.y + shape_bounds.height / 2.0;
  let bounds = transform_rect_bounds(
    KurboRect::new(
      f64::from(text_bounds.x),
      f64::from(text_bounds.y),
      f64::from(text_bounds.x + text_bounds.width),
      f64::from(text_bounds.y + text_bounds.height),
    ),
    Affine::rotate_about(
      f64::from(shape.shape_rotation_deg.to_radians()),
      (f64::from(center_x), f64::from(center_y)),
    ),
  );
  Some(shared_diagram::DiagramBounds {
    x: bounds.x0 as f32,
    y: bounds.y0 as f32,
    width: bounds.width() as f32,
    height: bounds.height() as f32,
  })
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
    metafile_monochrome_dib_palette_override: None,
    metafile_background_color: None,
    metafile_external_header: None,
    metafile_semantic_text_includes_raster_backdrop: false,
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
        diagram_synthesized_bullet_left_margin: paragraph.synthesized_bullet_left_margin,
        diagram_synthesized_bullet_indent: paragraph.synthesized_bullet_indent,
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

fn diagram_text_body_is_symbol_only(text_body: &TextBody) -> bool {
  let mut saw_symbol = false;
  for character in text_body
    .paragraphs
    .iter()
    .flat_map(|paragraph| paragraph.runs.iter())
    .flat_map(|run| run.text.chars())
  {
    if character.is_whitespace() {
      continue;
    }
    if character.is_alphanumeric() || (character as u32) < 0x2000 {
      return false;
    }
    saw_symbol = true;
  }
  saw_symbol
}

fn diagram_model_shape_common_fill(
  import: &PowerPointImport,
  slide: &SlidePersist,
  properties: &dgm::ShapeProperties,
  bounds: shared_diagram::DiagramBounds,
) -> Option<common::Fill<'static>> {
  match properties.shape_properties_choice2.as_ref()? {
    dgm::ShapePropertiesChoice2::SolidFill(fill) => {
      diagram_solid_fill(import, slide, fill.as_ref())
    }
    dgm::ShapePropertiesChoice2::GradientFill(fill) => {
      diagram_gradient_fill(import, slide, fill.as_ref(), bounds)
    }
    dgm::ShapePropertiesChoice2::PatternFill(fill) => Some(common::Fill::Pattern(
      pattern_fill_for_optional_slide(import, Some(slide), fill.as_ref(), None),
    )),
    dgm::ShapePropertiesChoice2::NoFill(_)
    | dgm::ShapePropertiesChoice2::BlipFill(_)
    | dgm::ShapePropertiesChoice2::GroupFill => None,
  }
}

fn diagram_solid_fill(
  import: &PowerPointImport,
  slide: &SlidePersist,
  fill: &a::SolidFill,
) -> Option<common::Fill<'static>> {
  let color = Color::from_solid_fill_choice(fill.solid_fill_choice.as_ref()?)?;
  let paint = display_paint_for_slide(import, slide, &color, None)?;
  Some(common::Fill::Solid(common_rgb(paint.color, paint.opacity)))
}

fn diagram_gradient_fill(
  import: &PowerPointImport,
  slide: &SlidePersist,
  fill: &a::GradientFill,
  bounds: shared_diagram::DiagramBounds,
) -> Option<common::Fill<'static>> {
  gradient_fill_for_optional_slide(import, Some(slide), fill, bounds)
}

fn gradient_fill_for_optional_slide(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  fill: &a::GradientFill,
  bounds: shared_diagram::DiagramBounds,
) -> Option<common::Fill<'static>> {
  let mut stops = fill
    .gradient_stop_list
    .as_ref()?
    .gradient_stop
    .iter()
    .filter_map(|stop| {
      let color = stop
        .gradient_stop_choice
        .as_ref()
        .and_then(Color::from_gradient_stop_choice)?;
      let paint = display_paint_for_optional_slide(import, slide, &color, None)?;
      Some(common::GradientStop {
        position: stop.position.as_ratio() as f32,
        color: common_rgb(paint.color, paint.opacity),
        scheme: None,
      })
    })
    .collect::<Vec<_>>();
  super::gradient::normalize_powerpoint_gradient_stops(&mut stops);
  if stops.is_empty() {
    return None;
  }
  let definition_bounds = common_rect(bounds.x, bounds.y, bounds.width, bounds.height);
  let (angle_degrees, scaled, path) = match fill.gradient_fill_choice.as_ref()? {
    a::GradientFillChoice::LinearGradientFill(linear) => (
      Some(linear.angle.unwrap_or_default() as f32 / 60_000.0),
      linear.scaled.as_ref().is_some_and(|value| value.as_bool()),
      None,
    ),
    a::GradientFillChoice::PathGradientFill(path) => {
      let mut path = common::drawingml_gradient::resolve_path_gradient(
        fill,
        path,
        common::Transform {
          m11: bounds.width,
          m12: 0.0,
          m21: 0.0,
          m22: bounds.height,
          dx: common::Pt(bounds.x),
          dy: common::Pt(bounds.y),
        },
      );
      if path.kind == common::GradientPathKind::Circle {
        path.transform = common::office_circle_gradient_transform(path.transform);
      }
      (None, false, Some(path))
    }
  };
  Some(common::Fill::Gradient(common::GradientFill {
    stops,
    angle_degrees,
    definition_bounds: Some(definition_bounds),
    line: None,
    interpolation: common::GradientInterpolation::LinearSrgb,
    scaled,
    rotate_with_shape: None,
    path,
  }))
}

fn diagram_fallback_rectangle_path(
  bounds: shared_diagram::DiagramBounds,
  fill: common::Fill<'static>,
  stroke: Option<common::Stroke<'static>>,
) -> common::PathItem<'static> {
  common::PathItem {
    bounds: common_rect(bounds.x, bounds.y, bounds.width, bounds.height),
    points: Vec::new(),
    commands: vec![
      common::PathCommand::MoveTo(common_point(bounds.x, bounds.y)),
      common::PathCommand::LineTo(common_point(bounds.x + bounds.width, bounds.y)),
      common::PathCommand::LineTo(common_point(
        bounds.x + bounds.width,
        bounds.y + bounds.height,
      )),
      common::PathCommand::LineTo(common_point(bounds.x, bounds.y + bounds.height)),
      common::PathCommand::Close,
    ],
    closed: true,
    fill,
    stroke,
  }
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
    Some(
      dgm::ShapePropertiesChoice2::NoFill(_)
        | dgm::ShapePropertiesChoice2::BlipFill(_)
        | dgm::ShapePropertiesChoice2::GroupFill
    )
  )
}

fn finish_diagram_model_shape_effects(
  import: &PowerPointImport,
  slide: &SlidePersist,
  properties: Option<&dgm::ShapeProperties>,
  bounds: shared_diagram::DiagramBounds,
  rotation_degrees: f32,
  items: &mut Vec<PageItem>,
  content_start: usize,
) {
  let Some(properties) = properties else {
    return;
  };
  let source = match properties.shape_properties_choice3.as_ref() {
    Some(dgm::ShapePropertiesChoice3::EffectList(list)) => {
      Some(ShapeEffectSource::List(list.as_ref()))
    }
    Some(dgm::ShapePropertiesChoice3::EffectDag(dag)) => Some(ShapeEffectSource::Dag(dag.as_ref())),
    None => None,
  };
  if source.is_none() && (properties.scene3_d_type.is_none() || properties.shape3_d_type.is_none())
  {
    return;
  }
  finish_shape_effect_raster(
    items,
    content_start,
    ShapeEffectRasterContext {
      import,
      slide,
      source,
      scene3d: properties.scene3_d_type.as_deref(),
      shape3d: properties.shape3_d_type.as_deref(),
      bounds: common_rect(bounds.x, bounds.y, bounds.width, bounds.height),
      rotation_degrees,
      camera_shape_rotation_degrees: rotation_degrees,
      children_source: false,
    },
  );
}

fn diagram_model_shape_outline(
  import: &PowerPointImport,
  slide: &SlidePersist,
  properties: &dgm::ShapeProperties,
  bounds: shared_diagram::DiagramBounds,
) -> Option<common::Stroke<'static>> {
  let outline = properties.outline.as_deref()?;
  diagram_outline(import, slide, outline, bounds)
}

fn diagram_outline(
  import: &PowerPointImport,
  slide: &SlidePersist,
  outline: &a::Outline,
  bounds: shared_diagram::DiagramBounds,
) -> Option<common::Stroke<'static>> {
  let (color, pattern, gradient) = match outline.outline_choice1.as_ref()? {
    a::OutlineChoice::NoFill(_) => return None,
    a::OutlineChoice::SolidFill(fill) => {
      let color = Color::from_solid_fill_choice(fill.solid_fill_choice.as_ref()?)?;
      let paint = display_paint_for_slide(import, slide, &color, None)?;
      (common_rgb(paint.color, paint.opacity), None, None)
    }
    a::OutlineChoice::PatternFill(fill) => {
      let pattern = pattern_fill_for_optional_slide(import, Some(slide), fill.as_ref(), None);
      (pattern.foreground, Some(pattern), None)
    }
    a::OutlineChoice::GradientFill(fill) => {
      let common::Fill::Gradient(gradient) =
        diagram_gradient_fill(import, slide, fill.as_ref(), bounds)?
      else {
        return None;
      };
      let color = gradient
        .stops
        .first()
        .map(|stop| stop.color)
        .unwrap_or_default();
      (color, None, Some(gradient))
    }
  };
  let mut stroke = common::Stroke {
    width: common::Pt(
      outline
        .width
        .map(|width| units::emu_to_points(i64::from(width)))
        .unwrap_or(0.5),
    ),
    color,
    pattern,
    gradient,
    ..Default::default()
  };
  common::drawingml_stroke::apply_outline_style(&mut stroke, outline);
  Some(stroke)
}

fn diagram_drawing_shape_path_items(
  properties: &dsp::ShapeProperties,
  bounds: shared_diagram::DiagramBounds,
  fill: common::Fill<'static>,
  stroke: Option<common::Stroke<'static>>,
) -> Option<Vec<common::PathItem<'static>>> {
  Some(
    shared_diagram::drawing_shape_paths(properties, bounds)?
      .into_iter()
      .map(|path| {
        let closed = path
          .commands
          .iter()
          .any(|command| matches!(command, common::PathCommand::Close));
        common::PathItem {
          bounds: common_rect(bounds.x, bounds.y, bounds.width, bounds.height),
          points: Vec::new(),
          commands: path.commands,
          closed,
          fill: path.fill_mode.apply_to_fill(fill.clone()),
          stroke: if path.stroke { stroke.clone() } else { None },
        }
      })
      .collect(),
  )
}

fn diagram_drawing_fallback_rectangle_path(
  properties: &dsp::ShapeProperties,
  bounds: shared_diagram::DiagramBounds,
  fill: common::Fill<'static>,
  stroke: Option<common::Stroke<'static>>,
) -> common::PathItem<'static> {
  let mut path = diagram_fallback_rectangle_path(bounds, fill, stroke);
  let rotation = properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.rotation)
    .unwrap_or_default() as f64
    / 60_000.0;
  if rotation.abs() <= f64::EPSILON {
    return path;
  }
  let transform = Affine::rotate_about(
    rotation.to_radians(),
    (
      f64::from(bounds.x + bounds.width / 2.0),
      f64::from(bounds.y + bounds.height / 2.0),
    ),
  );
  path.commands = transform_commands(path.commands, transform);
  let visible_bounds = transform_rect_bounds(
    KurboRect::new(
      f64::from(bounds.x),
      f64::from(bounds.y),
      f64::from(bounds.x + bounds.width),
      f64::from(bounds.y + bounds.height),
    ),
    transform,
  );
  path.bounds = common_rect(
    visible_bounds.x0 as f32,
    visible_bounds.y0 as f32,
    visible_bounds.width() as f32,
    visible_bounds.height() as f32,
  );
  path
}

fn diagram_style_outline(
  import: &PowerPointImport,
  slide: &SlidePersist,
  style: Option<&dgm::Style>,
  line_fill: Option<RgbColor>,
) -> Option<BorderStyle> {
  let reference = &style?.line_reference;
  let placeholder_color = line_fill.map(diagram_rgb_color).or_else(|| {
    reference
      .line_reference_choice
      .as_ref()
      .and_then(Color::from_line_reference_choice)
  });
  let line = import
    .get_theme_line_style(reference.index)
    .map(|line| line.with_placeholder_color(placeholder_color))?;
  line_stroke(import, Some(slide), &line).map(|stroke| stroke.style)
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
struct DiagramDrawingTransform(Affine);

impl DiagramDrawingTransform {
  fn root(
    x_pt: f32,
    y_pt: f32,
    width_pt: f32,
    height_pt: f32,
    transform: Option<&a::TransformGroup>,
  ) -> Self {
    let mut root = Self(Affine::translate((f64::from(x_pt), f64::from(y_pt))));
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
    let group = Self(group_child_affine(
      kurbo::Point::new(f64::from(off_x), f64::from(off_y)),
      kurbo::Vec2::new(f64::from(ext_width), f64::from(ext_height)),
      kurbo::Point::new(f64::from(child_x), f64::from(child_y)),
      kurbo::Vec2::new(f64::from(child_width), f64::from(child_height)),
    ));
    self.concat(group)
  }

  fn concat(self, child: Self) -> Self {
    Self(self.0 * child.0)
  }

  fn apply_bounds(self, x: f32, y: f32, width: f32, height: f32) -> shared_diagram::DiagramBounds {
    let bounds = transform_rect_bounds(
      KurboRect::new(
        f64::from(x),
        f64::from(y),
        f64::from(x + width),
        f64::from(y + height),
      ),
      self.0,
    );
    shared_diagram::DiagramBounds {
      x: bounds.x0 as f32,
      y: bounds.y0 as f32,
      width: bounds.width() as f32,
      height: bounds.height() as f32,
    }
  }
}

fn diagram_group_bounds(
  properties: &dsp::GroupShapeProperties,
  parent_transform: DiagramDrawingTransform,
) -> Option<shared_diagram::DiagramBounds> {
  let transform = properties.transform_group.as_deref()?;
  let offset = transform.offset.as_ref()?;
  let extents = transform.extents.as_ref()?;
  Some(parent_transform.apply_bounds(
    units::emu_to_points(offset.x.to_emu()),
    units::emu_to_points(offset.y.to_emu()),
    units::emu_to_points(extents.cx.to_emu()),
    units::emu_to_points(extents.cy.to_emu()),
  ))
}

fn diagram_group_common_fill(
  import: &PowerPointImport,
  slide: &SlidePersist,
  properties: &dsp::GroupShapeProperties,
  bounds: shared_diagram::DiagramBounds,
  inherited: Option<&common::Fill<'static>>,
) -> Option<common::Fill<'static>> {
  match properties.group_shape_properties_choice1.as_ref()? {
    dsp::GroupShapePropertiesChoice::SolidFill(fill) => {
      diagram_solid_fill(import, slide, fill.as_ref())
    }
    dsp::GroupShapePropertiesChoice::GradientFill(fill) => {
      diagram_gradient_fill(import, slide, fill.as_ref(), bounds)
    }
    dsp::GroupShapePropertiesChoice::PatternFill(fill) => Some(common::Fill::Pattern(
      pattern_fill_for_optional_slide(import, Some(slide), fill.as_ref(), None),
    )),
    dsp::GroupShapePropertiesChoice::GroupFill => inherited.cloned(),
    dsp::GroupShapePropertiesChoice::NoFill(_) | dsp::GroupShapePropertiesChoice::BlipFill(_) => {
      None
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
  shape: &dsp::Shape,
  transform: &dsp::Transform2D,
  parent_transform: DiagramDrawingTransform,
) -> Option<shared_diagram::DiagramBounds> {
  let (shape_bounds, _preset_bounds, mut text_bounds) =
    diagram_unrotated_text_transform_bounds(shape, transform)?;
  let shape_rotation = shape
    .shape_properties
    .transform2_d
    .as_deref()?
    .rotation
    .unwrap_or_default() as f32
    / 60_000.0;
  let shape_center = (
    shape_bounds.x + shape_bounds.width / 2.0,
    shape_bounds.y + shape_bounds.height / 2.0,
  );
  (text_bounds.x, text_bounds.y) =
    rotate_diagram_point((text_bounds.x, text_bounds.y), shape_center, shape_rotation);
  Some(parent_transform.apply_bounds(
    text_bounds.x,
    text_bounds.y,
    text_bounds.width,
    text_bounds.height,
  ))
}

fn diagram_unrotated_text_transform_bounds(
  shape: &dsp::Shape,
  transform: &dsp::Transform2D,
) -> Option<(
  shared_diagram::DiagramBounds,
  shared_diagram::DiagramBounds,
  shared_diagram::DiagramBounds,
)> {
  let offset = transform.offset.as_ref()?;
  let extents = transform.extents.as_ref()?;
  let shape_transform = shape.shape_properties.transform2_d.as_deref()?;
  let shape_offset = shape_transform.offset.as_ref()?;
  let shape_extents = shape_transform.extents.as_ref()?;
  let shape_bounds = shared_diagram::DiagramBounds {
    x: units::emu_to_points(shape_offset.x.to_emu()),
    y: units::emu_to_points(shape_offset.y.to_emu()),
    width: units::emu_to_points(shape_extents.cx.to_emu()),
    height: units::emu_to_points(shape_extents.cy.to_emu()),
  };
  let preset_bounds = diagram_preset_text_rectangle(shape, shape_bounds)?;
  let mut text_bounds = shared_diagram::DiagramBounds {
    x: units::emu_to_points(offset.x.to_emu()),
    y: units::emu_to_points(offset.y.to_emu()),
    width: units::emu_to_points(extents.cx.to_emu()),
    height: units::emu_to_points(extents.cy.to_emu()),
  };

  // LibreOffice Transform2DContext::onCreateContext() first compensates a
  // txXfrm rotation that is not cancelled by the owning shape rotation. The
  // resulting text rectangle is then positioned in the rotated shape's
  // coordinate system. Keep its width/height unrotated: those are the text
  // frame dimensions, not the axis-aligned bounds of a rotated rectangle.
  let shape_rotation = shape_transform.rotation.unwrap_or_default() as f32 / 60_000.0;
  let text_rotation = transform.rotation.unwrap_or_default() as f32 / 60_000.0;
  let angle_difference = (shape_rotation + text_rotation).rem_euclid(360.0);
  if angle_difference.abs() > f32::EPSILON {
    let preset_center = (
      preset_bounds.x + preset_bounds.width / 2.0,
      preset_bounds.y + preset_bounds.height / 2.0,
    );
    let text_center = (
      text_bounds.x + text_bounds.width / 2.0,
      text_bounds.y + text_bounds.height / 2.0,
    );
    let rotated = rotate_diagram_point(text_center, preset_center, -angle_difference);
    text_bounds.x += rotated.0 - text_center.0;
    text_bounds.y += rotated.1 - text_center.1;
  }
  Some((shape_bounds, preset_bounds, text_bounds))
}

fn rotate_diagram_point(point: (f32, f32), center: (f32, f32), angle_degrees: f32) -> (f32, f32) {
  let angle = angle_degrees.to_radians();
  let (sin, cos) = angle.sin_cos();
  let x = point.0 - center.0;
  let y = point.1 - center.1;
  (center.0 + x * cos - y * sin, center.1 + x * sin + y * cos)
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
  let Some(text_bounds) = diagram_text_transform_bounds(shape, text_transform, parent_transform)
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
  let text_distances_100mm = diagram_unrotated_text_transform_bounds(shape, text_transform).map(
    |(_shape_bounds, unrotated_preset_bounds, unrotated_text_bounds)| {
      let unrotated_offsets = TextDistances {
        left: unrotated_text_bounds.x - unrotated_preset_bounds.x,
        top: unrotated_text_bounds.y - unrotated_preset_bounds.y,
        right: unrotated_preset_bounds.width
          - unrotated_text_bounds.width
          - (unrotated_text_bounds.x - unrotated_preset_bounds.x),
        bottom: unrotated_preset_bounds.height
          - unrotated_text_bounds.height
          - (unrotated_text_bounds.y - unrotated_preset_bounds.y),
      };
      let unrotated_frame = text_body_frame_with_distances(
        unrotated_preset_bounds.x,
        unrotated_preset_bounds.y,
        unrotated_preset_bounds.width,
        unrotated_preset_bounds.height,
        text_body,
        unrotated_offsets,
        0,
      );
      text_distances_from_frame(
        unrotated_preset_bounds.x,
        unrotated_preset_bounds.y,
        unrotated_preset_bounds.width,
        unrotated_preset_bounds.height,
        unrotated_frame,
      )
    },
  );
  DiagramDrawingTextFrame {
    frame,
    text_area_x_pt: preset_bounds.x,
    text_area_y_pt: preset_bounds.y,
    rotation_center_pt: text_transform.rotation.map(|_| {
      (
        text_bounds.x + text_bounds.width / 2.0,
        text_bounds.y + text_bounds.height / 2.0,
      )
    }),
    rotation_deg: text_transform
      .rotation
      .map(|rotation| rotation as f32 / 60_000.0)
      .unwrap_or_default(),
    text_distances_100mm,
  }
}

#[derive(Clone, Copy, Debug)]
struct DiagramDrawingTextFrame {
  frame: TextFrame,
  text_area_x_pt: f32,
  text_area_y_pt: f32,
  rotation_center_pt: Option<(f32, f32)>,
  rotation_deg: f32,
  text_distances_100mm: Option<ShapeTextDistances100mm>,
}

impl DiagramDrawingTextFrame {
  fn new(frame: TextFrame, text_area_x_pt: f32, text_area_y_pt: f32) -> Self {
    Self {
      frame,
      text_area_x_pt,
      text_area_y_pt,
      rotation_center_pt: None,
      rotation_deg: 0.0,
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
  preset_text_rectangle(preset, bounds)
}

fn preset_text_rectangle(
  preset: &a::PresetGeometry,
  bounds: shared_diagram::DiagramBounds,
) -> Option<shared_diagram::DiagramBounds> {
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

fn diagram_shape_common_fill(
  import: &PowerPointImport,
  slide: &SlidePersist,
  properties: &dsp::ShapeProperties,
  bounds: shared_diagram::DiagramBounds,
  _group_fill: Option<&common::Fill<'static>>,
) -> Option<common::Fill<'static>> {
  match properties.shape_properties_choice2.as_ref()? {
    dsp::ShapePropertiesChoice2::SolidFill(fill) => {
      diagram_solid_fill(import, slide, fill.as_ref())
    }
    dsp::ShapePropertiesChoice2::GradientFill(fill) => {
      diagram_gradient_fill(import, slide, fill.as_ref(), bounds)
    }
    dsp::ShapePropertiesChoice2::PatternFill(fill) => Some(common::Fill::Pattern(
      pattern_fill_for_optional_slide(import, Some(slide), fill.as_ref(), None),
    )),
    dsp::ShapePropertiesChoice2::NoFill(_)
    | dsp::ShapePropertiesChoice2::BlipFill(_)
    | dsp::ShapePropertiesChoice2::GroupFill => None,
  }
}

fn diagram_shape_has_blip_fill(properties: &dsp::ShapeProperties) -> bool {
  matches!(
    properties.shape_properties_choice2.as_ref(),
    Some(dsp::ShapePropertiesChoice2::BlipFill(_))
  )
}

fn diagram_shape_suppresses_fill(properties: &dsp::ShapeProperties) -> bool {
  matches!(
    properties.shape_properties_choice2.as_ref(),
    Some(
      dsp::ShapePropertiesChoice2::NoFill(_)
        | dsp::ShapePropertiesChoice2::BlipFill(_)
        | dsp::ShapePropertiesChoice2::GroupFill
    )
  )
}

fn finish_diagram_drawing_shape_effects(
  import: &PowerPointImport,
  slide: &SlidePersist,
  properties: &dsp::ShapeProperties,
  bounds: shared_diagram::DiagramBounds,
  items: &mut Vec<PageItem>,
  content_start: usize,
) {
  let source = match properties.shape_properties_choice3.as_ref() {
    Some(dsp::ShapePropertiesChoice3::EffectList(list)) => {
      Some(ShapeEffectSource::List(list.as_ref()))
    }
    Some(dsp::ShapePropertiesChoice3::EffectDag(dag)) => Some(ShapeEffectSource::Dag(dag.as_ref())),
    None => None,
  };
  if source.is_none() && (properties.scene3_d_type.is_none() || properties.shape3_d_type.is_none())
  {
    return;
  }
  let rotation_degrees = properties
    .transform2_d
    .as_deref()
    .and_then(|transform| transform.rotation)
    .map(|rotation| rotation as f32 / 60_000.0)
    .unwrap_or_default();
  finish_shape_effect_raster(
    items,
    content_start,
    ShapeEffectRasterContext {
      import,
      slide,
      source,
      scene3d: properties.scene3_d_type.as_deref(),
      shape3d: properties.shape3_d_type.as_deref(),
      bounds: common_rect(bounds.x, bounds.y, bounds.width, bounds.height),
      rotation_degrees,
      camera_shape_rotation_degrees: rotation_degrees,
      children_source: false,
    },
  );
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
  let mut line_by_label = HashMap::new();
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
    if let Some(line_list) = label.line_color_list.as_ref() {
      let lines: Vec<LayoutRgbColor> = line_list
        .line_color_list_choice
        .iter()
        .filter_map(Color::from_diagram_line_color_choice)
        .filter_map(|color| import.resolve_color_for_slide(slide, &color, None))
        .map(|color| LayoutRgbColor {
          r: color.r,
          g: color.g,
          b: color.b,
        })
        .collect();
      if !lines.is_empty() {
        line_by_label.insert(label.name.clone(), lines);
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
  (!fill_by_label.is_empty() || !line_by_label.is_empty() || !text_fill_by_label.is_empty())
    .then_some(shared_diagram::DiagramStyleColors {
      fill_by_label,
      line_by_label,
      text_fill_by_label,
    })
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

fn diagram_rgb_color(color: RgbColor) -> Color {
  Color::RgbHex(super::drawingml::color::RgbHexColor {
    value: format!("{:02X}{:02X}{:02X}", color.r, color.g, color.b),
    transformations: Vec::new(),
  })
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

fn lower_legacy_vml_fill_image(shape: &Shape, offset: DisplayOffset, items: &mut Vec<PageItem>) {
  let Some(fill) = shape.legacy_vml_fill_image.as_ref() else {
    return;
  };
  if shape.size.cx <= 0 || shape.size.cy <= 0 {
    return;
  }
  let frame = TextFrame {
    x_pt: offset.x_pt(shape.position.x),
    y_pt: offset.y_pt(shape.position.y),
    width_pt: offset.width_pt(shape.size.cx),
    height_pt: offset.height_pt(shape.size.cy),
  };
  let clip_path = shape_path_commands(shape, frame);
  let image_dimensions = image::load_from_memory(&fill.resource.data)
    .ok()
    .map(|image| image.dimensions());
  let make_item = |frame: TextFrame, crop: ImageCrop| {
    PageItem::Image(ImageItem {
      x_pt: frame.x_pt,
      y_pt: frame.y_pt,
      width_pt: frame.width_pt,
      height_pt: frame.height_pt,
      crop,
      clip_path: clip_path.clone(),
      rotation_deg: if fill.rotate_with_shape {
        shape_visual_rotation_degrees(shape)
      } else {
        0.0
      },
      flip_horizontal: fill.rotate_with_shape && shape.flip_h,
      flip_vertical: fill.rotate_with_shape && shape.flip_v,
      data: fill.resource.data.clone(),
      content_type: fill.resource.content_type.clone(),
      metafile_monochrome_dib_palette_override: fill.resource.monochrome_dib_palette_override,
      metafile_background_color: None,
      metafile_external_header: None,
      metafile_semantic_text_includes_raster_backdrop: false,
      alt_text: shape
        .description
        .clone()
        .or_else(|| shape.title.clone())
        .or_else(|| shape.name.clone()),
      hyperlink_url: shape.hyperlink_url.clone(),
      floating: false,
      behind_text: false,
    })
  };
  if matches!(
    fill.fill_type,
    Some(
      ooxmlsdk::schemas::schemas_microsoft_com_vml::FillTypeValues::Tile
        | ooxmlsdk::schemas::schemas_microsoft_com_vml::FillTypeValues::Pattern
    )
  ) {
    let (natural_width, natural_height) = image_dimensions
      .map(|(width, height)| (width as f32 * 0.75, height as f32 * 0.75))
      .unwrap_or((frame.width_pt, frame.height_pt));
    let (tile_width, tile_height) = fill
      .size
      .as_deref()
      .and_then(|value| vml_fill_size_points(value, frame))
      .unwrap_or((natural_width, natural_height));
    if tile_width <= f32::EPSILON || tile_height <= f32::EPSILON {
      return;
    }
    let (phase_x, phase_y) = crate::xlsx::vml_tile_phase(
      fill.origin.as_deref(),
      fill.position.as_deref(),
      frame.width_pt,
      frame.height_pt,
      tile_width,
      tile_height,
    );
    let start_x = frame.x_pt + phase_x.rem_euclid(tile_width) - tile_width;
    let start_y = frame.y_pt + phase_y.rem_euclid(tile_height) - tile_height;
    let columns = ((frame.x_pt + frame.width_pt - start_x) / tile_width)
      .ceil()
      .max(1.0) as usize;
    let rows = ((frame.y_pt + frame.height_pt - start_y) / tile_height)
      .ceil()
      .max(1.0) as usize;
    let mut emitted = 0usize;
    for row in 0..rows {
      for column in 0..columns {
        if emitted == 1024 {
          return;
        }
        items.push(make_item(
          TextFrame {
            x_pt: start_x + column as f32 * tile_width,
            y_pt: start_y + row as f32 * tile_height,
            width_pt: tile_width,
            height_pt: tile_height,
          },
          ImageCrop::default(),
        ));
        emitted += 1;
      }
    }
    return;
  }
  let Some((pixel_width, pixel_height)) = image_dimensions else {
    items.push(make_item(frame, ImageCrop::default()));
    return;
  };
  let image_aspect = pixel_width as f32 / pixel_height.max(1) as f32;
  let frame_aspect = frame.width_pt / frame.height_pt.max(f32::EPSILON);
  use ooxmlsdk::schemas::schemas_microsoft_com_vml::ImageAspectValues as Aspect;
  match fill.aspect.unwrap_or_default() {
    Aspect::Ignore => items.push(make_item(frame, ImageCrop::default())),
    Aspect::AtMost => {
      let (width_pt, height_pt) = if image_aspect > frame_aspect {
        (frame.width_pt, frame.width_pt / image_aspect)
      } else {
        (frame.height_pt * image_aspect, frame.height_pt)
      };
      items.push(make_item(
        TextFrame {
          x_pt: frame.x_pt + (frame.width_pt - width_pt) / 2.0,
          y_pt: frame.y_pt + (frame.height_pt - height_pt) / 2.0,
          width_pt,
          height_pt,
        },
        ImageCrop::default(),
      ));
    }
    Aspect::AtLeast => {
      let crop = if image_aspect > frame_aspect {
        let visible = frame_aspect / image_aspect;
        ImageCrop {
          left: (1.0 - visible) / 2.0,
          right: (1.0 - visible) / 2.0,
          ..ImageCrop::default()
        }
      } else {
        let visible = image_aspect / frame_aspect;
        ImageCrop {
          top: (1.0 - visible) / 2.0,
          bottom: (1.0 - visible) / 2.0,
          ..ImageCrop::default()
        }
      };
      items.push(make_item(frame, crop));
    }
  }
}

fn vml_fill_size_points(value: &str, frame: TextFrame) -> Option<(f32, f32)> {
  let mut values = value.split(',').map(str::trim);
  let parse = |value: &str, reference: f32| {
    value
      .strip_suffix('%')
      .and_then(|value| value.trim().parse::<f32>().ok())
      .map(|value| reference * value / 100.0)
      .or_else(|| super::slide::vml_measure_to_points(value))
  };
  Some((
    parse(values.next()?, frame.width_pt)?,
    parse(values.next()?, frame.height_pt)?,
  ))
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
  let unresolved_external_picture = picture.embed_relationship_id.is_none()
    && picture.link_relationship_id.is_some()
    && picture.image_resource.is_none();
  if picture.empty_blip_fill || unresolved_external_picture {
    lower_empty_blip_fill_placeholder(shape, offset, items);
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
  let frame = TextFrame {
    x_pt: offset.x_pt(shape.position.x),
    y_pt: offset.y_pt(shape.position.y),
    width_pt: offset.width_pt(shape.size.cx),
    height_pt: offset.height_pt(shape.size.cy),
  };
  items.push(PageItem::Image(ImageItem {
    x_pt: frame.x_pt,
    y_pt: frame.y_pt,
    width_pt: frame.width_pt,
    height_pt: frame.height_pt,
    crop,
    clip_path: if custom_geometry {
      shape_path_commands(shape, frame)
    } else {
      Default::default()
    },
    rotation_deg: shape_visual_rotation_degrees(shape),
    flip_horizontal,
    flip_vertical,
    data,
    content_type,
    metafile_monochrome_dib_palette_override: resource.monochrome_dib_palette_override,
    metafile_background_color: None,
    metafile_external_header: resource.metafile_external_header,
    metafile_semantic_text_includes_raster_backdrop: resource
      .metafile_semantic_text_includes_raster_backdrop,
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

fn lower_empty_blip_fill_placeholder(
  shape: &Shape,
  offset: DisplayOffset,
  items: &mut Vec<PageItem>,
) {
  let frame = TextFrame {
    x_pt: offset.x_pt(shape.position.x),
    y_pt: offset.y_pt(shape.position.y),
    width_pt: offset.width_pt(shape.size.cx),
    height_pt: offset.height_pt(shape.size.cy),
  };
  let inset = MISSING_PICTURE_BORDER_INSET_PT
    .min(frame.width_pt / 2.0)
    .min(frame.height_pt / 2.0);
  let border_frame = TextFrame {
    x_pt: frame.x_pt + inset,
    y_pt: frame.y_pt + inset,
    width_pt: (frame.width_pt - 2.0 * inset).max(0.0),
    height_pt: (frame.height_pt - 2.0 * inset).max(0.0),
  };
  if border_frame.width_pt > 0.0 && border_frame.height_pt > 0.0 {
    items.push(PageItem::Path(common::PathItem {
      bounds: transformed_shape_bounds(border_frame, shape),
      points: Vec::new(),
      commands: shape_path_commands(shape, border_frame),
      closed: true,
      fill: common::Fill::None,
      stroke: Some(common::Stroke {
        width: common::Pt(MISSING_PICTURE_BORDER_WIDTH_PT),
        color: common_rgb(RgbColor { r: 0, g: 0, b: 0 }, 1.0),
        ..Default::default()
      }),
    }));
  }

  let mut icon_center_x =
    frame.x_pt + MISSING_PICTURE_ICON_OFFSET_PT + MISSING_PICTURE_ICON_WIDTH_PT / 2.0;
  let mut icon_center_y =
    frame.y_pt + MISSING_PICTURE_ICON_OFFSET_PT + MISSING_PICTURE_ICON_HEIGHT_PT / 2.0;
  let frame_center_x = frame.x_pt + frame.width_pt / 2.0;
  let frame_center_y = frame.y_pt + frame.height_pt / 2.0;
  if shape.flip_h {
    icon_center_x = 2.0 * frame_center_x - icon_center_x;
  }
  if shape.flip_v {
    icon_center_y = 2.0 * frame_center_y - icon_center_y;
  }
  let rotation_degrees = shape_visual_rotation_degrees(shape);
  let rotation_radians = rotation_degrees.to_radians();
  let cos = rotation_radians.cos();
  let sin = rotation_radians.sin();
  let relative_x = icon_center_x - frame_center_x;
  let relative_y = icon_center_y - frame_center_y;
  icon_center_x = frame_center_x + relative_x * cos - relative_y * sin;
  icon_center_y = frame_center_y + relative_x * sin + relative_y * cos;

  items.push(PageItem::Image(ImageItem {
    x_pt: icon_center_x - MISSING_PICTURE_ICON_WIDTH_PT / 2.0,
    y_pt: icon_center_y - MISSING_PICTURE_ICON_HEIGHT_PT / 2.0,
    width_pt: MISSING_PICTURE_ICON_WIDTH_PT,
    height_pt: MISSING_PICTURE_ICON_HEIGHT_PT,
    crop: ImageCrop::default(),
    clip_path: Vec::new(),
    rotation_deg: rotation_degrees,
    flip_horizontal: shape.flip_h,
    flip_vertical: shape.flip_v,
    data: missing_picture_icon_png(),
    content_type: Some("image/png".to_string()),
    metafile_monochrome_dib_palette_override: None,
    metafile_background_color: None,
    metafile_external_header: None,
    metafile_semantic_text_includes_raster_backdrop: false,
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

fn missing_picture_icon_png() -> Arc<[u8]> {
  static PNG: OnceLock<Arc<[u8]>> = OnceLock::new();
  PNG
    .get_or_init(|| {
      #[rustfmt::skip]
      const RGB: [u8; 4 * 5 * 3] = [
        128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
        128, 128, 128, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        128, 128, 128, 255, 255, 255, 255,   0,   0, 255, 255, 255,
        128, 128, 128, 255, 204, 204, 255, 255, 255, 255, 255, 255,
        128, 128, 128, 255, 255, 255, 255, 255, 255, 255, 255, 255,
      ];
      let mut png = Cursor::new(Vec::new());
      PngEncoder::new(&mut png)
        .write_image(&RGB, 4, 5, ColorType::Rgb8.into())
        .expect("encoding the fixed missing-picture icon cannot fail");
      Arc::from(png.into_inner())
    })
    .clone()
}

fn lower_shape_hyperlink(shape: &Shape, offset: DisplayOffset, items: &mut Vec<PageItem>) {
  let Some(hyperlink_url) = &shape.hyperlink_url else {
    return;
  };
  if shape.service_name == ShapeService::Group || shape.size.cx <= 0 || shape.size.cy <= 0 {
    return;
  }
  items.push(PageItem::LinkArea(LinkAreaItem {
    x_pt: offset.x_pt(shape.position.x),
    y_pt: offset.y_pt(shape.position.y),
    width_pt: offset.width_pt(shape.size.cx),
    height_pt: offset.height_pt(shape.size.cy),
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
  let x0 = offset.x_pt(shape.position.x);
  let y0 = offset.y_pt(shape.position.y);
  let table_width = offset.width_pt(table.grid.iter().copied().sum::<i64>());
  let row_height_sum = table.rows.iter().map(|row| row.height).sum::<i64>();
  let table_height = offset.height_pt(row_height_sum.max(shape.size.cy));
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
  let table_style = package_table_style
    .or(predefined_table_style.as_ref())
    .or_else(|| {
      import
        .table_style_list
        .as_ref()
        .and_then(|styles| styles.default_style())
    });
  let table_background = table_style.and_then(|style| {
    let fill = table_style_part_fill(import, &style.table_background)?;
    match &fill.kind {
      FillKind::Gradient(gradient) => gradient_fill_for_optional_slide(
        import,
        None,
        gradient,
        shared_diagram::DiagramBounds {
          x: x0,
          y: y0,
          width: table_width,
          height: table_height,
        },
      ),
      _ => common_fill_for_optional_slide(import, None, &fill),
    }
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
  let row_heights = table
    .rows
    .iter()
    .map(|row| {
      table_row_display_height(row.height, row_height_sum, shape.size.cy, offset.scale_y())
    })
    .collect::<Vec<_>>();
  for (row_index, row) in table.rows.iter().enumerate() {
    let row_height = row_heights[row_index];
    let mut x = x0;
    let mut grid_index = 0usize;
    for cell in &row.cells {
      let span = table_cell_grid_advance(cell);
      // PowerPoint serializes a horizontal merge as one gridSpan origin plus
      // one hMerge continuation for every covered grid cell. The origin has
      // already consumed those columns, so a continuation owns neither
      // another grid column nor another physical width.
      let width_emu = if cell.horizontal_merge {
        0
      } else if grid_index < table.grid.len() {
        table.grid[grid_index..table.grid.len().min(grid_index + span)]
          .iter()
          .copied()
          .sum::<i64>()
      } else {
        0
      };
      let cell_width = offset.width_pt(width_emu);
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
          table_background.clone(),
          TextFrame {
            x_pt: x,
            y_pt: y,
            width_pt: cell_width,
            height_pt: table_cell_display_height(cell, row_index, &row_heights),
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
      x += offset.width_pt(*width);
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

fn table_row_display_height(
  row_height: i64,
  row_height_sum: i64,
  shape_height: i64,
  scale_y: f32,
) -> f32 {
  let row_height = units::emu_to_points_f32(row_height as f32 * scale_y);
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
  table_background: Option<common::Fill<'static>>,
  frame: TextFrame,
  items: &mut Vec<PageItem>,
) {
  if frame.width_pt <= 0.0 || frame.height_pt <= 0.0 {
    return;
  }
  let cell_fill = table_cell_fill(import, cell, style_part, frame);
  if cell_fill
    .as_ref()
    .is_none_or(|fill| !common_fill_is_opaque(fill))
    && let Some(background) = table_background
  {
    push_table_cell_fill(items, frame, background);
  }
  if let Some(fill) = cell_fill {
    push_table_cell_fill(items, frame, fill);
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

fn table_cell_fill(
  import: &PowerPointImport,
  cell: &TableCell,
  style_part: Option<&TableStylePart>,
  frame: TextFrame,
) -> Option<common::Fill<'static>> {
  let fill = cell
    .fill_properties
    .as_ref()
    .or_else(|| style_part.and_then(|style| style.fill_properties.as_ref()))?;
  match &fill.kind {
    FillKind::Gradient(gradient) => gradient_fill_for_optional_slide(
      import,
      None,
      gradient,
      shared_diagram::DiagramBounds {
        x: frame.x_pt,
        y: frame.y_pt,
        width: frame.width_pt,
        height: frame.height_pt,
      },
    ),
    _ => common_fill_for_optional_slide(import, None, fill),
  }
}

fn push_table_cell_fill(items: &mut Vec<PageItem>, frame: TextFrame, fill: common::Fill<'static>) {
  match fill {
    common::Fill::Solid(color) => items.push(PageItem::Rect(RectItem {
      x_pt: frame.x_pt,
      y_pt: frame.y_pt,
      width_pt: frame.width_pt,
      height_pt: frame.height_pt,
      fill_color: Some(RgbColor {
        r: color.r,
        g: color.g,
        b: color.b,
      }),
      fill_opacity: opacity_from_common_color(color),
      stroke: None,
      stroke_opacity: 1.0,
    })),
    common::Fill::Pattern(_) | common::Fill::Gradient(_) => push_pattern_rect(
      items,
      frame.x_pt,
      frame.y_pt,
      frame.width_pt,
      frame.height_pt,
      fill,
    ),
    _ => {}
  }
}

fn common_fill_is_opaque(fill: &common::Fill<'_>) -> bool {
  match fill {
    common::Fill::Solid(color) => color.a == u8::MAX,
    common::Fill::Pattern(pattern) => {
      pattern.foreground.a == u8::MAX && pattern.background.a == u8::MAX
    }
    common::Fill::Gradient(gradient) => {
      !gradient.stops.is_empty() && gradient.stops.iter().all(|stop| stop.color.a == u8::MAX)
    }
    _ => false,
  }
}

fn push_pattern_rect(
  items: &mut Vec<PageItem>,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  fill: common::Fill<'static>,
) {
  items.push(PageItem::Path(common::PathItem {
    bounds: common_rect(x, y, width, height),
    points: Vec::new(),
    commands: vec![
      common::PathCommand::MoveTo(common_point(x, y)),
      common::PathCommand::LineTo(common_point(x + width, y)),
      common::PathCommand::LineTo(common_point(x + width, y + height)),
      common::PathCommand::LineTo(common_point(x, y + height)),
      common::PathCommand::Close,
    ],
    closed: true,
    fill,
    stroke: None,
  }));
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
  let Some(mut stroke) = line
    .as_ref()
    .and_then(|line| line_stroke(import, None, line))
  else {
    return;
  };
  if let Some(LineFill::Gradient(gradient)) = line.as_ref().map(|line| &line.fill) {
    stroke.common.gradient = gradient_fill_for_optional_slide(
      import,
      None,
      gradient,
      shared_diagram::DiagramBounds {
        x: x1_pt.min(x2_pt),
        y: y1_pt.min(y2_pt),
        width: (x2_pt - x1_pt).abs(),
        height: (y2_pt - y1_pt).abs(),
      },
    )
    .and_then(|fill| match fill {
      common::Fill::Gradient(gradient) => Some(gradient),
      _ => None,
    });
  }
  if stroke.common.pattern.is_some() || stroke.common.gradient.is_some() {
    items.push(PageItem::Path(common::PathItem {
      bounds: common_rect(
        x1_pt.min(x2_pt),
        y1_pt.min(y2_pt),
        (x2_pt - x1_pt).abs(),
        (y2_pt - y1_pt).abs(),
      ),
      points: Vec::new(),
      commands: vec![
        common::PathCommand::MoveTo(common_point(x1_pt, y1_pt)),
        common::PathCommand::LineTo(common_point(x2_pt, y2_pt)),
      ],
      closed: false,
      fill: common::Fill::None,
      stroke: Some(stroke.common),
    }));
    return;
  }
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

fn table_cell_row_span(cell: &TableCell) -> usize {
  cell
    .row_span
    .and_then(|span| usize::try_from(span).ok())
    .filter(|span| *span > 0)
    .unwrap_or(1)
}

fn table_cell_display_height(cell: &TableCell, row: usize, row_heights: &[f32]) -> f32 {
  let end = row_heights
    .len()
    .min(row.saturating_add(table_cell_row_span(cell)));
  row_heights.get(row..end).unwrap_or_default().iter().sum()
}

fn table_cell_grid_advance(cell: &TableCell) -> usize {
  table_grid_advance(cell.horizontal_merge, table_cell_grid_span(cell))
}

fn table_grid_advance(horizontal_merge: bool, span: usize) -> usize {
  if horizontal_merge {
    // DrawingML producers may emit both gridSpan on the merge origin and an
    // explicit hMerge continuation cell. The origin has already consumed
    // those grid columns; advancing again would push the following real cell
    // outside the table grid (tdf#119015).
    0
  } else {
    span
  }
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
  content_start_override: Option<usize>,
  items: &mut Vec<PageItem>,
) {
  if shape.service_name == ShapeService::Group
    || shape.size.cx < 0
    || shape.size.cy < 0
    || shape.service_name != ShapeService::Connector && (shape.size.cx == 0 || shape.size.cy == 0)
    || shape.service_name == ShapeService::Connector && shape.size.cx == 0 && shape.size.cy == 0
  {
    return;
  }

  let mut shape_fill = shape.legacy_vml_fill.clone().or_else(|| {
    shape
      .actual_fill_properties
      .as_ref()
      .and_then(|fill| shape_common_fill(import, slide, fill))
  });
  let x_pt = offset.x_pt(shape.position.x);
  let y_pt = offset.y_pt(shape.position.y);
  let width_pt = offset.width_pt(shape.size.cx);
  let height_pt = offset.height_pt(shape.size.cy);
  let frame = TextFrame {
    x_pt,
    y_pt,
    width_pt,
    height_pt,
  };
  let mut picture_foreground = content_start_override
    .map(|start| items.drain(start..).collect::<Vec<_>>())
    .unwrap_or_default();
  let has_picture_foreground = !picture_foreground.is_empty();
  if let Some(common::Fill::Gradient(gradient)) = shape_fill.as_mut()
    && let Some(path) = gradient.path.as_mut()
  {
    path.transform = crate::xlsx::common_transform_from_affine(
      shape_path_transform(frame, shape)
        * Affine::scale_non_uniform(f64::from(width_pt), f64::from(height_pt))
          .then_translate((f64::from(x_pt), f64::from(y_pt)).into()),
    );
  }
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
          rotation_deg: shape_visual_rotation_degrees(shape),
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
    .and_then(|line| shape_line_stroke(import, slide, shape, line, frame));
  let common_stroke = shape
    .legacy_vml_stroke
    .clone()
    .or_else(|| line.as_ref().map(|line| line.common.clone()));
  let gradient_path = shape.actual_fill_properties.as_ref().and_then(|fill| {
    shape_gradient_path(
      import,
      slide,
      shape,
      fill,
      frame,
      (!has_picture_foreground).then_some(line.as_ref()).flatten(),
    )
  });
  let has_fill_image = !fill_images.is_empty();
  let has_fill_overlay = shape
    .actual_effect_properties
    .as_ref()
    .and_then(|effects| effects.fill_overlay.as_ref())
    .and_then(|effect| effect.fill.as_ref())
    .is_some();
  if shape_fill.is_none()
    && !has_fill_image
    && common_stroke.is_none()
    && gradient_path.is_none()
    && !has_fill_overlay
    && content_start_override.is_none()
  {
    return;
  }
  let stroke = line.as_ref().map(|line| line.style);
  let effect_dag = shape
    .actual_effect_properties
    .as_ref()
    .and_then(|effects| effects.effect_dag.as_ref());
  let effect_list = shape
    .actual_effect_properties
    .as_ref()
    .and_then(|effects| effects.effect_list.as_ref());
  let fill_overlay = shape
    .actual_effect_properties
    .as_ref()
    .and_then(|effects| effects.fill_overlay.as_ref())
    .and_then(|effect| {
      let fill = effect.fill.as_ref()?;
      let overlay_items = shape_effect_fill_items(import, slide, shape, fill, frame, &shape_path);
      (!overlay_items.is_empty())
        .then_some((drawingml_blend_mode(effect.blend_mode), overlay_items))
    });
  let reflection_frame = ShadowFrame {
    x_pt,
    y_pt,
    width_pt,
    height_pt,
    stroke_width_pt: stroke.map_or(0.0, |stroke| stroke.width_pt),
  };
  let effect_bounds = transformed_shape_bounds(frame, shape);
  let effect_frame = ShadowFrame {
    x_pt: effect_bounds.origin.x.0,
    y_pt: effect_bounds.origin.y.0,
    width_pt: effect_bounds.size.width.0,
    height_pt: effect_bounds.size.height.0,
    stroke_width_pt: reflection_frame.stroke_width_pt,
  };
  let soft_edge_mask = shape
    .scene3d
    .is_none()
    .then_some(shape.actual_effect_properties.as_ref())
    .flatten()
    .filter(|_| shape.shape3d.is_none())
    .and_then(|effects| effects.soft_edge.as_ref())
    .and_then(|effect| {
      soft_edge_mask_image_item(
        effect,
        effect_frame,
        &shape_path,
        shape_fill.is_some() || has_fill_image || gradient_path.is_some() || fill_overlay.is_some(),
        common_stroke.as_ref(),
      )
    });
  let reflection = shape
    .actual_effect_properties
    .as_ref()
    .and_then(|effects| effects.reflection.as_ref())
    .and_then(|effect| {
      let mask = reflection_mask_image_item(
        effect,
        reflection_frame,
        shape_visual_rotation_degrees(shape),
        shape.flip_h,
        shape.flip_v,
      )?;
      Some((mask, reflection_transform(effect, frame, shape)))
    });
  let outer_shadow = shape
    .actual_effect_properties
    .as_ref()
    .and_then(|effects| effects.outer_shadow.as_ref())
    .and_then(|shadow| {
      let paint = shadow
        .color
        .as_ref()
        .and_then(|color| display_paint_for_slide(import, slide, color, None))?;
      outer_shadow_image_item(
        shadow,
        effect_frame,
        ShadowShape {
          path: &shape_path,
          has_fill: shape_fill.is_some()
            || has_fill_image
            || gradient_path.is_some()
            || fill_overlay.is_some(),
          stroke_style: common_stroke.as_ref(),
        },
        outer_shadow_transform(shadow, frame, shape),
        paint.color,
        paint.opacity,
      )
    });

  let inner_shadow = shape
    .actual_effect_properties
    .as_ref()
    .and_then(|effects| effects.inner_shadow.as_ref())
    .and_then(|shadow| {
      let paint = shadow
        .color
        .as_ref()
        .and_then(|color| display_paint_for_slide(import, slide, color, None))?;
      inner_shadow_image_item(
        shadow,
        effect_frame,
        &shape_path,
        shape_fill.is_some() || has_fill_image || gradient_path.is_some() || fill_overlay.is_some(),
        common_stroke.as_ref(),
        paint.color,
        paint.opacity,
      )
    });

  let mut legacy_behind_effects = Vec::new();
  if effect_dag.is_none() {
    if let Some(glow) = shape
      .actual_effect_properties
      .as_ref()
      .and_then(|effects| effects.glow.as_ref())
      && let Some(paint) = glow
        .color
        .as_ref()
        .and_then(|color| display_paint_for_slide(import, slide, color, None))
      && let Some(image) = glow_image_item(
        glow,
        effect_frame,
        &shape_path,
        shape_fill.is_some() || has_fill_image || gradient_path.is_some() || fill_overlay.is_some(),
        common_stroke.as_ref(),
        paint.color,
        paint.opacity,
      )
    {
      legacy_behind_effects.push(PageItem::Image(image));
    }
    // CT_EffectList fixes glow before outerShdw. Keeping that order here is
    // observable where their translucent rasters overlap.
    if let Some(image) = outer_shadow {
      legacy_behind_effects.push(PageItem::Image(image));
    }
  }

  let shape_content_start = content_start_override.unwrap_or(items.len());
  let effect_source = effect_dag
    .map(ShapeEffectSource::Dag)
    .or_else(|| effect_list.map(ShapeEffectSource::List));
  let effect_context = (effect_source.is_some()
    || shape.scene3d.is_some() && shape.shape3d.is_some())
  .then_some(ShapeEffectRasterContext {
    import,
    slide,
    source: effect_source,
    scene3d: shape.scene3d.as_ref(),
    shape3d: shape.shape3d.as_ref(),
    bounds: effect_bounds,
    rotation_degrees: shape_visual_rotation_degrees(shape),
    camera_shape_rotation_degrees: shape.rotation,
    children_source: false,
  });
  items.extend(fill_images.into_iter().map(PageItem::Image));
  if let Some(paths) = gradient_path {
    items.extend(paths.into_iter().map(PageItem::Path));
    append_picture_foreground(
      items,
      &mut picture_foreground,
      shape,
      frame,
      common_stroke.as_ref(),
    );
    finish_shape_effects(
      items,
      shape_content_start,
      (fill_overlay, inner_shadow, soft_edge_mask),
      reflection,
      effect_context,
      legacy_behind_effects,
    );
    return;
  }
  if shape_fill.is_none() && common_stroke.is_none() {
    append_picture_foreground(
      items,
      &mut picture_foreground,
      shape,
      frame,
      common_stroke.as_ref(),
    );
    finish_shape_effects(
      items,
      shape_content_start,
      (fill_overlay, inner_shadow, soft_edge_mask),
      reflection,
      effect_context,
      legacy_behind_effects,
    );
    return;
  }

  if shape.service_name == ShapeService::Connector {
    items.push(PageItem::Path(common::PathItem {
      bounds: transformed_shape_bounds(frame, shape),
      points: Vec::new(),
      commands: shape_stroke_path_commands(shape, frame),
      closed: false,
      fill: common::Fill::None,
      stroke: common_stroke.clone(),
    }));
    append_picture_foreground(
      items,
      &mut picture_foreground,
      shape,
      frame,
      common_stroke.as_ref(),
    );
    finish_shape_effects(
      items,
      shape_content_start,
      (fill_overlay, inner_shadow, soft_edge_mask),
      reflection,
      effect_context,
      legacy_behind_effects,
    );
    return;
  }

  if let Some(paths) = shape_drawing_paths(shape, frame) {
    for path in paths {
      let closed = path
        .commands
        .iter()
        .any(|command| matches!(command, common::PathCommand::Close));
      items.push(PageItem::Path(common::PathItem {
        bounds: transformed_shape_bounds(frame, shape),
        points: Vec::new(),
        commands: path.commands,
        closed,
        fill: path
          .fill_mode
          .apply_to_fill(shape_fill.clone().unwrap_or(common::Fill::None)),
        stroke: if path.stroke && !has_picture_foreground {
          common_stroke.clone()
        } else {
          None
        },
      }));
    }
    append_picture_foreground(
      items,
      &mut picture_foreground,
      shape,
      frame,
      common_stroke.as_ref(),
    );
    finish_shape_effects(
      items,
      shape_content_start,
      (fill_overlay, inner_shadow, soft_edge_mask),
      reflection,
      effect_context,
      legacy_behind_effects,
    );
    return;
  }

  append_picture_foreground(
    items,
    &mut picture_foreground,
    shape,
    frame,
    common_stroke.as_ref(),
  );
  finish_shape_effects(
    items,
    shape_content_start,
    (fill_overlay, inner_shadow, soft_edge_mask),
    reflection,
    effect_context,
    legacy_behind_effects,
  );
}

fn append_picture_foreground(
  items: &mut Vec<PageItem>,
  picture_foreground: &mut Vec<PageItem>,
  shape: &Shape,
  frame: TextFrame,
  stroke: Option<&common::Stroke<'static>>,
) {
  if picture_foreground.is_empty() {
    return;
  }
  items.append(picture_foreground);
  let Some(stroke) = stroke else {
    return;
  };
  let Some(paths) = shape_drawing_paths(shape, frame) else {
    return;
  };
  for path in paths.into_iter().filter(|path| path.stroke) {
    let closed = path
      .commands
      .iter()
      .any(|command| matches!(command, common::PathCommand::Close));
    items.push(PageItem::Path(common::PathItem {
      bounds: transformed_shape_bounds(frame, shape),
      points: Vec::new(),
      commands: path.commands,
      closed,
      fill: common::Fill::None,
      stroke: Some(stroke.clone()),
    }));
  }
}

#[derive(Clone, Copy)]
enum ShapeEffectSource<'a> {
  List(&'a a::EffectList),
  Dag(&'a a::EffectDag),
}

#[derive(Clone, Copy)]
struct ShapeEffectRasterContext<'a> {
  import: &'a PowerPointImport,
  slide: &'a SlidePersist,
  source: Option<ShapeEffectSource<'a>>,
  scene3d: Option<&'a a::Scene3DType>,
  shape3d: Option<&'a a::Shape3DType>,
  bounds: common::Rect,
  rotation_degrees: f32,
  camera_shape_rotation_degrees: f32,
  children_source: bool,
}

type ShapeEffectOverlays = (
  Option<(common::BlendMode, Vec<PageItem>)>,
  Option<ImageItem>,
  Option<ImageItem>,
);

fn finish_shape_effects(
  items: &mut Vec<PageItem>,
  content_start: usize,
  overlays: ShapeEffectOverlays,
  reflection: Option<(ImageItem, common::Transform)>,
  effect_context: Option<ShapeEffectRasterContext<'_>>,
  legacy_behind_effects: Vec<PageItem>,
) {
  let (fill_overlay, inner_shadow, soft_edge_mask) = overlays;
  if let Some(context) = effect_context
    && finish_shape_effect_raster(items, content_start, context)
  {
    return;
  }
  let behind_count = legacy_behind_effects.len();
  items.splice(content_start..content_start, legacy_behind_effects);
  let content_start = content_start + behind_count;
  let has_fill_overlay = fill_overlay.is_some();
  if let Some((blend_mode, overlay_items)) = fill_overlay {
    items.push(PageItem::Group {
      mask: None,
      clip: None,
      transform: None,
      blend_mode,
      opacity: 1.0,
      items: overlay_items,
    });
  }
  if let Some(inner_shadow) = inner_shadow {
    items.push(PageItem::Image(inner_shadow));
  }
  if items.len() <= content_start {
    return;
  }
  let mut content = items.drain(content_start..).collect::<Vec<_>>();
  if soft_edge_mask.is_some() || has_fill_overlay {
    content = vec![PageItem::Group {
      mask: soft_edge_mask,
      clip: None,
      transform: None,
      blend_mode: common::BlendMode::Normal,
      opacity: 1.0,
      items: content,
    }];
  }
  if let Some((mask, transform)) = reflection {
    let reflected_items = effect_copy_items(&content);
    if !reflected_items.is_empty() {
      items.push(PageItem::Group {
        mask: Some(mask),
        clip: None,
        transform: Some(transform),
        blend_mode: common::BlendMode::Normal,
        opacity: 1.0,
        items: reflected_items,
      });
    }
  }
  items.extend(content);
}

fn finish_shape_effect_raster(
  items: &mut Vec<PageItem>,
  content_start: usize,
  context: ShapeEffectRasterContext<'_>,
) -> bool {
  if items.len() <= content_start {
    return false;
  }
  let resolver = PptxImageEffectColorResolver {
    import: context.import,
    slide: Some(context.slide),
  };
  let fixed_effect_list = matches!(context.source, Some(ShapeEffectSource::List(_)));
  let mut effects = match context.source {
    Some(ShapeEffectSource::List(list)) => {
      common::drawingml_image_effects::from_effect_list(list, None, &resolver)
    }
    Some(ShapeEffectSource::Dag(dag)) => {
      common::drawingml_image_effects::from_effect_dag(dag, None, &resolver)
    }
    None => common::drawingml_image_effects::ImageEffectContainer {
      kind: common::drawingml_image_effects::ImageEffectContainerKind::Sibling,
      effects: Vec::new(),
    },
  };
  if context.scene3d.is_some() || context.shape3d.is_some() {
    common::drawingml_image_effects::suppress_soft_edge(&mut effects);
  }
  if effects.effects.is_empty() && (context.scene3d.is_none() || context.shape3d.is_none()) {
    return false;
  }
  common::drawingml_image_effects::rotate_container_with_shape(
    &mut effects,
    context.rotation_degrees,
  );
  let simple_glow =
    simple_shape_glow(&effects).filter(|_| context.scene3d.is_none() && context.shape3d.is_none());
  let behind_effects = fixed_effect_list
    .then(|| shape_behind_effects(&effects))
    .flatten()
    .filter(|_| context.scene3d.is_none() && context.shape3d.is_none());
  let preserve_vector_source = behind_effects.is_some();
  let output_bounds = if effects.effects.is_empty() {
    // Static 3-D is independent from a:effectLst/a:effectDag. Most authored
    // scene3d shapes have no effect container at all, so the synthetic empty
    // sibling container must act as an identity for bounds and painting
    // rather than suppressing the 3-D raster and falling back to the flat
    // source vector.
    common::drawingml_image_effects::EffectOutputBounds {
      left_pt: 0.0,
      top_pt: 0.0,
      right_pt: context.bounds.size.width.0,
      bottom_pt: context.bounds.size.height.0,
    }
  } else {
    let Some(output_bounds) = common::drawingml_image_effects::container_output_bounds(
      &effects,
      context.bounds.size.width.0,
      context.bounds.size.height.0,
    ) else {
      return false;
    };
    output_bounds
  };
  let static_padding = context
    .scene3d
    .zip(context.shape3d)
    .map(|(scene, shape)| {
      common::drawingml_3d::output_padding(
        common::drawingml_3d::camera_projection(scene, context.camera_shape_rotation_degrees),
        shape,
        context.bounds.size.width.0,
        context.bounds.size.height.0,
      )
    })
    .unwrap_or_default();
  let relative_left = output_bounds.left_pt.min(0.0) - static_padding.left_pt;
  let relative_top = output_bounds.top_pt.min(0.0) - static_padding.top_pt;
  let relative_right =
    output_bounds.right_pt.max(context.bounds.size.width.0) + static_padding.right_pt;
  let relative_bottom =
    output_bounds.bottom_pt.max(context.bounds.size.height.0) + static_padding.bottom_pt;
  let raster_bounds = common::Rect {
    origin: common::Point {
      x: common::Pt(context.bounds.origin.x.0 + relative_left),
      y: common::Pt(context.bounds.origin.y.0 + relative_top),
    },
    size: common::Size {
      width: common::Pt(relative_right - relative_left),
      height: common::Pt(relative_bottom - relative_top),
    },
  };
  let mut semantic_overlays = Vec::new();
  collect_pptx_semantic_text_overlays(
    &items[content_start..],
    &mut semantic_overlays,
    &mut TextMetrics::new(),
  );
  let display_items = items[content_start..]
    .iter()
    .cloned()
    .map(common_display_item)
    .collect::<Vec<_>>();
  let automatic_extrusion_color =
    common::drawingml_3d::automatic_extrusion_color_from_items(&display_items);
  let raster = if simple_glow.is_some() && !context.children_source {
    common::drawingml_shape_raster::rasterize_fill_layer_at_pixels_per_point(
      &display_items,
      raster_bounds,
      138.0 / 297.6,
    )
  } else if context.scene3d.is_some() && context.shape3d.is_some() && !context.children_source {
    // PowerPoint fixed output consistently stores static picture-3D surfaces
    // at about 200 DPI (tdf170095, Scene3d_pureImage, and
    // Scene3d_cropped_image). Retain the shared 250,000-pixel budget so large
    // objects do not become more expensive; only the small-shape sampling cap
    // differs from ordinary DrawingML image effects.
    common::drawingml_shape_raster::rasterize_vector_items_for_effects_at_bounded_pixels_per_point(
      &display_items,
      raster_bounds,
      &effects,
      200.0 / 72.0,
    )
  } else if context.children_source {
    common::drawingml_shape_raster::rasterize_group_items_for_effects(
      &display_items,
      raster_bounds,
      &effects,
    )
  } else {
    common::drawingml_shape_raster::rasterize_vector_items_for_effects(
      &display_items,
      raster_bounds,
      &effects,
    )
  };
  let Some(mut raster) = raster else {
    return false;
  };
  if let Some(glow) = simple_glow {
    // PowerPoint fixed output keeps a simple shape glow in a low-resolution
    // image behind the original vector shape. The Office reference for
    // shape-text-glow-effect.pptx stores a 138 px mask over a 297.6 pt box.
    let mut glow = glow;
    if let common::drawingml_image_effects::ImageEffect::Glow {
      spread_ratio,
      spread_kernel,
      ..
    } = &mut glow
    {
      *spread_ratio = 0.5;
      *spread_kernel = common::drawingml_image_effects::GlowSpreadKernel::Diamond;
    }
    effects = common::drawingml_image_effects::ImageEffectContainer {
      kind: common::drawingml_image_effects::ImageEffectContainerKind::Sibling,
      effects: vec![glow],
    };
  } else if let Some(behind_effects) = behind_effects {
    effects = behind_effects;
  }
  if let Some((scene, shape)) = context.scene3d.zip(context.shape3d) {
    let extrusion_color = shape
      .extrusion_color
      .as_deref()
      .and_then(|color| color.extrusion_color_choice.as_ref())
      .and_then(Color::from_extrusion_color_choice)
      .and_then(|color| resolver.resolve(Some(color)))
      .map(|color| common::drawingml_3d::Static3dColor {
        color: color.color,
        alpha: color.alpha,
      });
    let contour_color = shape
      .contour_color
      .as_deref()
      .and_then(|color| color.contour_color_choice.as_ref())
      .and_then(Color::from_contour_color_choice)
      .and_then(|color| resolver.resolve(Some(color)))
      .map(|color| common::drawingml_3d::Static3dColor {
        color: color.color,
        alpha: color.alpha,
      });
    common::drawingml_3d::apply_static_3d(
      &mut raster.image,
      scene,
      common::drawingml_3d::camera_projection(scene, context.camera_shape_rotation_degrees),
      shape,
      common::drawingml_3d::Static3dRenderOptions {
        extrusion_color: extrusion_color.or(automatic_extrusion_color),
        contour_color,
        pixels_per_point: raster.pixels_per_point,
        model_surface: Some(common::drawingml_3d::Static3dSurface {
          left_px: (context.bounds.origin.x.0 - raster_bounds.origin.x.0) * raster.pixels_per_point,
          top_px: (context.bounds.origin.y.0 - raster_bounds.origin.y.0) * raster.pixels_per_point,
          width_px: context.bounds.size.width.0 * raster.pixels_per_point,
          height_px: context.bounds.size.height.0 * raster.pixels_per_point,
        }),
      },
    );
  }
  if !effects.effects.is_empty() {
    common::drawingml_image_effects::scale_container_pixel_lengths(
      &mut effects,
      raster.pixels_per_point / (96.0 / 72.0),
    );
    common::drawingml_image_effects::apply_container_to_padded_image_with_sources(
      &mut raster.image,
      &effects,
      -relative_left * raster.pixels_per_point,
      -relative_top * raster.pixels_per_point,
      context.bounds.size.width.0 * raster.pixels_per_point,
      context.bounds.size.height.0 * raster.pixels_per_point,
      common::drawingml_image_effects::ImageEffectSourceImages {
        fill: raster.fill_image.as_ref(),
        line: raster.line_image.as_ref(),
        fill_line: raster.fill_line_image.as_ref(),
        children: raster.children_image.as_ref(),
      },
    );
  }
  let mut png = Cursor::new(Vec::new());
  if PngEncoder::new(&mut png)
    .write_image(
      raster.image.as_raw(),
      raster.image.width(),
      raster.image.height(),
      ColorType::Rgba8.into(),
    )
    .is_err()
  {
    return false;
  }
  let effect_image = PageItem::Image(ImageItem {
    x_pt: raster_bounds.origin.x.0,
    y_pt: raster_bounds.origin.y.0,
    width_pt: raster_bounds.size.width.0,
    height_pt: raster_bounds.size.height.0,
    crop: ImageCrop::default(),
    clip_path: Vec::new(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    data: Arc::from(png.into_inner()),
    content_type: Some("image/png".to_string()),
    metafile_monochrome_dib_palette_override: None,
    metafile_background_color: None,
    metafile_external_header: None,
    metafile_semantic_text_includes_raster_backdrop: false,
    alt_text: None,
    hyperlink_url: None,
    floating: false,
    behind_text: false,
  });
  if preserve_vector_source {
    items.insert(content_start, effect_image);
    return true;
  }
  items.truncate(content_start);
  items.push(effect_image);
  items.extend(semantic_overlays);
  true
}

fn shape_behind_effects(
  effects: &common::drawingml_image_effects::ImageEffectContainer,
) -> Option<common::drawingml_image_effects::ImageEffectContainer> {
  use common::drawingml_image_effects::{
    ImageEffect, ImageEffectContainer, ImageEffectContainerKind,
  };
  if effects.kind != ImageEffectContainerKind::Sibling {
    return None;
  }
  let mut behind = Vec::new();
  let mut has_unchanged_source = false;
  for effect in &effects.effects {
    match effect {
      ImageEffect::Glow { .. } | ImageEffect::OuterShadow { .. } | ImageEffect::Reflection(_) => {
        behind.push(effect.clone())
      }
      ImageEffect::Container(main)
        if main.kind == ImageEffectContainerKind::Tree
          && main.effects.as_slice() == [ImageEffect::Identity] =>
      {
        has_unchanged_source = true;
      }
      _ => return None,
    }
  }
  if !has_unchanged_source || behind.is_empty() {
    return None;
  }
  Some(ImageEffectContainer {
    kind: ImageEffectContainerKind::Sibling,
    effects: behind,
  })
}

fn simple_shape_glow(
  effects: &common::drawingml_image_effects::ImageEffectContainer,
) -> Option<common::drawingml_image_effects::ImageEffect> {
  use common::drawingml_image_effects::{ImageEffect, ImageEffectContainerKind};
  if effects.kind != ImageEffectContainerKind::Sibling || effects.effects.len() != 2 {
    return None;
  }
  let glow = effects.effects.first()?;
  let ImageEffect::Container(main) = effects.effects.get(1)? else {
    return None;
  };
  if main.kind != ImageEffectContainerKind::Tree
    || main.effects.as_slice() != [ImageEffect::Identity]
    || !matches!(glow, ImageEffect::Glow { .. })
  {
    return None;
  }
  Some(glow.clone())
}

fn effect_copy_items(items: &[PageItem]) -> Vec<PageItem> {
  items.iter().filter_map(effect_copy_item).collect()
}

fn effect_copy_item(item: &PageItem) -> Option<PageItem> {
  Some(match item {
    PageItem::Text(text) => {
      let mut text = text.clone();
      text.hyperlink_url = None;
      PageItem::Text(text)
    }
    PageItem::Image(image) => {
      let mut image = image.clone();
      image.alt_text = None;
      image.hyperlink_url = None;
      PageItem::Image(image)
    }
    PageItem::Group {
      mask,
      clip,
      transform,
      blend_mode,
      opacity,
      items,
    } => PageItem::Group {
      mask: mask.clone(),
      clip: *clip,
      transform: *transform,
      blend_mode: *blend_mode,
      opacity: *opacity,
      items: effect_copy_items(items),
    },
    PageItem::LinkArea(_) => return None,
    PageItem::Path(path) => PageItem::Path(path.clone()),
    PageItem::Rect(rect) => PageItem::Rect(*rect),
    PageItem::Line(line) => PageItem::Line(*line),
  })
}

fn reflection_transform(
  effect: &super::drawingml::shape_properties::EffectReflectionProperties,
  frame: TextFrame,
  shape: &Shape,
) -> common::Transform {
  drawingml_effect_transform(
    frame,
    shape,
    EffectTransformParameters {
      scale_x: effect.scale_x,
      scale_y: effect.scale_y,
      horizontal_skew: effect.horizontal_skew,
      vertical_skew: effect.vertical_skew,
      alignment: effect.alignment,
      distance_emu: effect.distance_emu,
      direction: effect.direction,
      rotate_with_shape: effect.rotate_with_shape.unwrap_or(true),
    },
  )
}

fn outer_shadow_transform(
  effect: &super::drawingml::shape_properties::EffectShadowProperties,
  frame: TextFrame,
  shape: &Shape,
) -> common::Transform {
  drawingml_effect_transform(
    frame,
    shape,
    EffectTransformParameters {
      scale_x: effect.scale_x,
      scale_y: effect.scale_y,
      horizontal_skew: effect.horizontal_skew,
      vertical_skew: effect.vertical_skew,
      alignment: effect.alignment,
      distance_emu: effect.distance_emu,
      direction: effect.direction,
      rotate_with_shape: effect.rotate_with_shape.unwrap_or(true),
    },
  )
}

#[derive(Clone, Copy)]
struct EffectTransformParameters {
  scale_x: Option<i32>,
  scale_y: Option<i32>,
  horizontal_skew: Option<i32>,
  vertical_skew: Option<i32>,
  alignment: Option<a::RectangleAlignmentValues>,
  distance_emu: Option<i64>,
  direction: Option<i32>,
  rotate_with_shape: bool,
}

fn drawingml_effect_transform(
  frame: TextFrame,
  shape: &Shape,
  parameters: EffectTransformParameters,
) -> common::Transform {
  let scale_x = parameters.scale_x.unwrap_or(100_000) as f64 / 100_000.0;
  let scale_y = parameters.scale_y.unwrap_or(100_000) as f64 / 100_000.0;
  let skew_x = (parameters.horizontal_skew.unwrap_or_default() as f64 / 60_000.0)
    .to_radians()
    .tan();
  let skew_y = (parameters.vertical_skew.unwrap_or_default() as f64 / 60_000.0)
    .to_radians()
    .tan();
  let (alignment_x, alignment_y) = shadow_alignment(parameters.alignment);
  let anchor_x = f64::from(frame.x_pt + alignment_x * frame.width_pt);
  let anchor_y = f64::from(frame.y_pt + alignment_y * frame.height_pt);
  let distance = f64::from(units::emu_to_points(
    parameters.distance_emu.unwrap_or_default(),
  ));
  let direction = (parameters.direction.unwrap_or_default() as f64 / 60_000.0).to_radians();
  let offset_x = direction.cos() * distance;
  let offset_y = direction.sin() * distance;

  // ECMA-376 Part 1 §20.1.8.61 gives the authored matrix directly as
  // [sx tan(kx) tx; tan(ky) sy ty]. Reflection alignment establishes the
  // origin for that scale/skew/offset matrix.
  let effect_transform = Affine::new([
    scale_x,
    skew_y,
    skew_x,
    scale_y,
    anchor_x + offset_x - scale_x * anchor_x - skew_x * anchor_y,
    anchor_y + offset_y - skew_y * anchor_x - scale_y * anchor_y,
  ]);
  let effect_transform = if parameters.rotate_with_shape {
    let shape_transform = shape_path_transform(frame, shape);
    shape_transform * effect_transform * shape_transform.inverse()
  } else {
    effect_transform
  };
  let [m11, m12, m21, m22, dx, dy] = effect_transform.as_coeffs();
  common::Transform {
    m11: m11 as f32,
    m12: m12 as f32,
    m21: m21 as f32,
    m22: m22 as f32,
    dx: common::Pt(dx as f32),
    dy: common::Pt(dy as f32),
  }
}

fn shape_effect_fill_items(
  import: &PowerPointImport,
  slide: &SlidePersist,
  shape: &Shape,
  fill: &FillProperties,
  frame: TextFrame,
  shape_path: &[common::PathCommand],
) -> Vec<PageItem> {
  let mut items = blip_fill_image_items(
    import,
    slide,
    fill,
    ImageFillPlacement {
      frame,
      rotation_deg: shape_visual_rotation_degrees(shape),
      flip_horizontal: shape.flip_h,
      flip_vertical: shape.flip_v,
      crop_bitmap: shape.custom_shape_properties.geometry.is_some(),
      clip_path: shape_path.to_vec(),
      alt_text: None,
      hyperlink_url: None,
    },
  )
  .into_iter()
  .map(PageItem::Image)
  .collect::<Vec<_>>();
  if let Some(paths) = shape_gradient_path(import, slide, shape, fill, frame, None) {
    items.extend(paths.into_iter().map(PageItem::Path));
    return items;
  }
  let Some(fill) = shape_common_fill(import, slide, fill) else {
    return items;
  };
  let Some(paths) = shape_drawing_paths(shape, frame) else {
    return items;
  };
  for path in paths {
    let closed = path
      .commands
      .iter()
      .any(|command| matches!(command, common::PathCommand::Close));
    items.push(PageItem::Path(common::PathItem {
      bounds: transformed_shape_bounds(frame, shape),
      points: Vec::new(),
      commands: path.commands,
      closed,
      fill: path.fill_mode.apply_to_fill(fill.clone()),
      stroke: None,
    }));
  }
  items
}

fn drawingml_blend_mode(mode: a::BlendModeValues) -> common::BlendMode {
  match mode {
    // DrawingML token `over` is source-over compositing, not the PDF
    // artistic "Overlay" blend mode.
    a::BlendModeValues::Overlay => common::BlendMode::Normal,
    a::BlendModeValues::Multiply => common::BlendMode::Multiply,
    a::BlendModeValues::Screen => common::BlendMode::Screen,
    a::BlendModeValues::Darken => common::BlendMode::Darken,
    a::BlendModeValues::Lighten => common::BlendMode::Lighten,
  }
}

fn shape_line_stroke(
  import: &PowerPointImport,
  slide: &SlidePersist,
  shape: &Shape,
  line: &LineProperties,
  frame: TextFrame,
) -> Option<DisplayStroke> {
  let mut stroke = line_stroke(import, Some(slide), line)?;
  let LineFill::Gradient(gradient) = &line.fill else {
    return Some(stroke);
  };
  let fill = FillProperties {
    kind: FillKind::Gradient(gradient.clone()),
    placeholder_color: line.placeholder_color.clone(),
  };
  stroke.common.gradient =
    shape_gradient_path(import, slide, shape, &fill, frame, None).and_then(|paths| {
      paths.into_iter().find_map(|path| match path.fill {
        common::Fill::Gradient(gradient) => Some(gradient),
        _ => None,
      })
    });
  Some(stroke)
}

fn shape_gradient_path(
  import: &PowerPointImport,
  slide: &SlidePersist,
  shape: &Shape,
  fill: &FillProperties,
  frame: TextFrame,
  line: Option<&DisplayStroke>,
) -> Option<Vec<common::PathItem<'static>>> {
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
  let has_shape_transform =
    shape_visual_rotation_degrees(shape).abs() > f32::EPSILON || shape.flip_h || shape.flip_v;
  // `rotWithShape` governs the ordinary 2-D xfrm. A scene camera instead
  // rotates the complete painted face, so its fill texture follows that
  // camera even when the authored 2-D gradient is page-fixed. Otherwise a
  // 90-degree orthographic camera samples only the middle stripe of a wide
  // horizontal gradient after the geometry becomes vertical.
  let follows_shape_transform = (rotate_with_shape || shape.scene3d.is_some())
    && has_shape_transform
    && !matches!(fill.kind, FillKind::SlideBackground);
  let (angle_degrees, gradient_line, scaled, path, interpolation) =
    match gradient.gradient_fill_choice.as_ref()? {
      a::GradientFillChoice::LinearGradientFill(linear) => {
        let scaled = linear.scaled.as_ref().is_some_and(|value| value.as_bool());
        let local_angle_degrees = linear.angle.unwrap_or_default() as f32 / 60_000.0;
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
        (
          Some(angle_degrees),
          gradient_line,
          scaled,
          None,
          if follows_shape_transform {
            common::GradientInterpolation::PowerPointGammaSigma
          } else {
            common::GradientInterpolation::LinearSrgb
          },
        )
      }
      a::GradientFillChoice::PathGradientFill(path) => {
        let mut path = common::drawingml_gradient::resolve_path_gradient(
          gradient,
          path,
          path_gradient_transform(frame, shape, follows_shape_transform),
        );
        if path.kind == common::GradientPathKind::Circle {
          path.transform = common::office_circle_gradient_transform(path.transform);
        }
        (
          None,
          None,
          false,
          Some(path),
          common::GradientInterpolation::LinearSrgb,
        )
      }
    };
  let fill = common::Fill::Gradient(common::GradientFill {
    stops,
    angle_degrees,
    definition_bounds: Some(definition_bounds),
    line: gradient_line,
    interpolation,
    scaled,
    rotate_with_shape: None,
    path,
  });
  Some(
    shape_drawing_paths(shape, frame)?
      .into_iter()
      .map(|path| {
        let closed = path
          .commands
          .iter()
          .any(|command| matches!(command, common::PathCommand::Close));
        common::PathItem {
          bounds: transformed_shape_bounds(frame, shape),
          points: Vec::new(),
          commands: path.commands,
          closed,
          fill: path.fill_mode.apply_to_fill(fill.clone()),
          stroke: path
            .stroke
            .then(|| line.map(|line| line.common.clone()))
            .flatten(),
        }
      })
      .collect(),
  )
}

fn path_gradient_transform(
  frame: TextFrame,
  shape: &Shape,
  follows_shape_transform: bool,
) -> common::Transform {
  if !follows_shape_transform {
    return common::Transform {
      m11: frame.width_pt,
      m12: 0.0,
      m21: 0.0,
      m22: frame.height_pt,
      dx: common::Pt(frame.x_pt),
      dy: common::Pt(frame.y_pt),
    };
  }
  let top_left = transform_shape_point(common_point(frame.x_pt, frame.y_pt), frame, shape);
  let top_right = transform_shape_point(
    common_point(frame.x_pt + frame.width_pt, frame.y_pt),
    frame,
    shape,
  );
  let bottom_left = transform_shape_point(
    common_point(frame.x_pt, frame.y_pt + frame.height_pt),
    frame,
    shape,
  );
  common::Transform {
    m11: top_right.x.0 - top_left.x.0,
    m12: top_right.y.0 - top_left.y.0,
    m21: bottom_left.x.0 - top_left.x.0,
    m22: bottom_left.y.0 - top_left.y.0,
    dx: top_left.x,
    dy: top_left.y,
  }
}

fn shape_path_commands(shape: &Shape, frame: TextFrame) -> Vec<common::PathCommand> {
  shape_drawing_paths(shape, frame)
    .unwrap_or_default()
    .into_iter()
    .filter(|path| path.fill_mode != common::DrawingPathFillMode::None)
    .flat_map(|path| path.commands)
    .collect()
}

fn shape_stroke_path_commands(shape: &Shape, frame: TextFrame) -> Vec<common::PathCommand> {
  shape_drawing_paths(shape, frame)
    .unwrap_or_default()
    .into_iter()
    .filter(|path| path.stroke)
    .flat_map(|path| path.commands)
    .collect()
}

fn shape_drawing_paths(shape: &Shape, frame: TextFrame) -> Option<Vec<common::DrawingPath>> {
  let mut paths = if let Some(paths) = shape.legacy_vml_paths.as_ref() {
    let translation = Affine::translate((f64::from(frame.x_pt), f64::from(frame.y_pt)));
    paths
      .iter()
      .cloned()
      .map(|mut path| {
        path.commands = transform_commands(path.commands, translation);
        path
      })
      .collect()
  } else {
    match shape.custom_shape_properties.geometry.as_ref() {
      Some(CustomShapeGeometry::Custom(geometry)) => custom_geometry::paths(
        geometry,
        frame.x_pt,
        frame.y_pt,
        frame.width_pt,
        frame.height_pt,
      )?,
      Some(CustomShapeGeometry::Preset(preset)) => preset_geometry::paths(
        Some(preset),
        frame.x_pt,
        frame.y_pt,
        frame.width_pt,
        frame.height_pt,
      )?,
      None => preset_geometry::paths(
        None,
        frame.x_pt,
        frame.y_pt,
        frame.width_pt,
        frame.height_pt,
      )?,
    }
  };
  let transform = shape_path_transform(frame, shape);
  for path in &mut paths {
    path.commands = transform_commands(std::mem::take(&mut path.commands), transform);
  }
  Some(paths)
}

fn transform_shape_point(point: common::Point, frame: TextFrame, shape: &Shape) -> common::Point {
  transform_point(point, shape_path_transform(frame, shape))
}

fn shape_visual_rotation_degrees(shape: &Shape) -> f32 {
  match (shape.scene3d.as_ref(), shape.shape3d.as_ref()) {
    // A scene without sp3d still rotates the painted 2-D face. There is no
    // static-3D raster stage to consume that camera revolution, so retain its
    // resolved face rotation in the ordinary display transform.
    (Some(scene), None) => {
      -common::drawingml_3d::camera_projection(scene, shape.rotation).face_rotation_degrees
    }
    // A complete static-3D style owns the camera and authored shape rotation;
    // keep its raster source in unrotated model coordinates.
    (Some(_), Some(_)) => 0.0,
    (None, _) => shape.rotation,
  }
}

fn shape_path_transform(frame: TextFrame, shape: &Shape) -> Affine {
  let center_x = frame.x_pt + frame.width_pt / 2.0;
  let center_y = frame.y_pt + frame.height_pt / 2.0;
  Affine::translate((-f64::from(center_x), -f64::from(center_y)))
    .then_scale_non_uniform(
      if shape.flip_h { -1.0 } else { 1.0 },
      if shape.flip_v { -1.0 } else { 1.0 },
    )
    .then_rotate(f64::from(shape_visual_rotation_degrees(shape).to_radians()))
    .then_translate((f64::from(center_x), f64::from(center_y)).into())
}

fn transformed_shape_bounds(frame: TextFrame, shape: &Shape) -> common::Rect {
  let bounds = transform_rect_bounds(
    KurboRect::new(
      f64::from(frame.x_pt),
      f64::from(frame.y_pt),
      f64::from(frame.x_pt + frame.width_pt),
      f64::from(frame.y_pt + frame.height_pt),
    ),
    shape_path_transform(frame, shape),
  );
  common_rect(
    bounds.x0 as f32,
    bounds.y0 as f32,
    bounds.width() as f32,
    bounds.height() as f32,
  )
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
  angle + shape_visual_rotation_degrees(shape)
}

fn child_display_offset(shape: &Shape, offset: DisplayOffset) -> DisplayOffset {
  let child_width = if shape.child_size.cx != 0 {
    shape.child_size.cx
  } else {
    shape.size.cx
  };
  let child_height = if shape.child_size.cy != 0 {
    shape.child_size.cy
  } else {
    shape.size.cy
  };
  DisplayOffset(
    offset.0
      * group_child_affine(
        kurbo::Point::new(shape.position.x as f64, shape.position.y as f64),
        kurbo::Vec2::new(shape.size.cx as f64, shape.size.cy as f64),
        kurbo::Point::new(shape.child_position.x as f64, shape.child_position.y as f64),
        kurbo::Vec2::new(child_width as f64, child_height as f64),
      ),
  )
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
  mut placement: ImageFillPlacement,
) -> Vec<ImageItem> {
  if !blip_fill
    .rotate_with_shape
    .as_ref()
    .is_some_and(|value| value.as_bool())
  {
    placement.rotation_deg = 0.0;
    placement.flip_horizontal = false;
    placement.flip_vertical = false;
  }
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
    return tiled_blip_fill_image_items(image_data.data, content_type, tile, crop, placement);
  }
  // lclGetBitmapMode() defaults missing bitmap mode to XML_tile for MSO.
  if blip_fill.blip_fill_choice.is_none() {
    return tiled_blip_fill_image_items(
      image_data.data,
      content_type,
      &a::Tile::default(),
      crop,
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
    metafile_monochrome_dib_palette_override: None,
    metafile_background_color: None,
    metafile_external_header: None,
    metafile_semantic_text_includes_raster_backdrop: false,
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
  source_crop: ImageCrop,
  placement: ImageFillPlacement,
) -> Vec<ImageItem> {
  let natural_size =
    image_tile_size_pt(&data).unwrap_or((placement.frame.width_pt, placement.frame.height_pt));
  common::drawingml_image_tile::placements(
    (
      placement.frame.x_pt,
      placement.frame.y_pt,
      placement.frame.width_pt,
      placement.frame.height_pt,
    ),
    natural_size,
    tile,
    source_crop,
    1024,
  )
  .into_iter()
  .map(|tile| {
    let tile = common::drawingml_image_tile::rotate_placement_about_frame(
      tile,
      (
        placement.frame.x_pt,
        placement.frame.y_pt,
        placement.frame.width_pt,
        placement.frame.height_pt,
      ),
      placement.rotation_deg,
    );
    ImageItem {
      x_pt: tile.x_pt,
      y_pt: tile.y_pt,
      width_pt: tile.width_pt,
      height_pt: tile.height_pt,
      crop: tile.crop,
      clip_path: placement.clip_path.clone(),
      rotation_deg: placement.rotation_deg,
      flip_horizontal: placement.flip_horizontal ^ tile.flip_horizontal,
      flip_vertical: placement.flip_vertical ^ tile.flip_vertical,
      data: Arc::clone(&data),
      content_type: content_type.clone(),
      metafile_monochrome_dib_palette_override: None,
      metafile_background_color: None,
      metafile_external_header: None,
      metafile_semantic_text_includes_raster_backdrop: false,
      alt_text: placement.alt_text.clone(),
      hyperlink_url: placement.hyperlink_url.clone(),
      floating: false,
      behind_text: false,
    }
  })
  .collect()
}

fn image_tile_size_pt(data: &[u8]) -> Option<(f32, f32)> {
  let image = image::load_from_memory(data).ok()?;
  if let Some((horizontal_dpi, vertical_dpi)) = jpeg_density_dpi(data) {
    return Some((
      image.width() as f32 * units::POINTS_PER_INCH / horizontal_dpi,
      image.height() as f32 * units::POINTS_PER_INCH / vertical_dpi,
    ));
  }
  Some((
    image.width() as f32 * units::POINTS_PER_CSS_PIXEL,
    image.height() as f32 * units::POINTS_PER_CSS_PIXEL,
  ))
}

fn jpeg_density_dpi(data: &[u8]) -> Option<(f32, f32)> {
  if !data.starts_with(&[0xff, 0xd8]) {
    return None;
  }
  let mut offset = 2usize;
  while offset + 4 <= data.len() {
    while offset < data.len() && data[offset] == 0xff {
      offset += 1;
    }
    let marker = *data.get(offset)?;
    offset += 1;
    if marker == 0xd9 || marker == 0xda {
      break;
    }
    let length = usize::from(u16::from_be_bytes([
      *data.get(offset)?,
      *data.get(offset + 1)?,
    ]));
    if length < 2 || offset + length > data.len() {
      return None;
    }
    let payload = &data[offset + 2..offset + length];
    if marker == 0xe0 && payload.len() >= 12 && payload.starts_with(b"JFIF\0") {
      let unit = payload[7];
      let horizontal = f32::from(u16::from_be_bytes([payload[8], payload[9]]));
      let vertical = f32::from(u16::from_be_bytes([payload[10], payload[11]]));
      if horizontal <= 0.0 || vertical <= 0.0 {
        return None;
      }
      return match unit {
        1 => Some((horizontal, vertical)),
        2 => Some((horizontal * 2.54, vertical * 2.54)),
        _ => None,
      };
    }
    offset += length;
  }
  None
}

struct ImportedImageData {
  data: Arc<[u8]>,
  content_type: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct ImageEffects(Vec<ImageEffect>);

struct PptxImageEffectColorResolver<'a> {
  import: &'a PowerPointImport,
  slide: Option<&'a SlidePersist>,
}

impl PptxImageEffectColorResolver<'_> {
  fn resolve(&self, color: Option<Color>) -> Option<ResolvedEffectColor> {
    let paint = display_paint_for_optional_slide(self.import, self.slide, &color?, None)?;
    Some(ResolvedEffectColor {
      color: paint.color,
      alpha: (paint.opacity.clamp(0.0, 1.0) * 255.0).round() as u8,
    })
  }
}

impl ImageEffectColorResolver for PptxImageEffectColorResolver<'_> {
  fn alpha_inverse(&self, choice: &a::AlphaInverseChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_alpha_inverse_choice(choice))
  }

  fn color_from(&self, choice: &a::ColorFromChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_color_from_choice(choice))
  }

  fn color_to(&self, choice: &a::ColorToChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_color_to_choice(choice))
  }

  fn color_replacement(&self, choice: &a::ColorReplacementChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_color_replacement_choice(choice))
  }

  fn duotone(&self, choice: &a::DuotoneChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_duotone_choice(choice))
  }

  fn solid_fill(&self, choice: &a::SolidFillChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_solid_fill_choice(choice))
  }

  fn gradient_stop(&self, choice: &a::GradientStopChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_gradient_stop_choice(choice))
  }

  fn foreground(&self, choice: &a::ForegroundColorChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_foreground_color_choice(choice))
  }

  fn background(&self, choice: &a::BackgroundColorChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_background_color_choice(choice))
  }

  fn glow(&self, choice: &a::GlowChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_glow_choice(choice))
  }

  fn inner_shadow(&self, choice: &a::InnerShadowChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_inner_shadow_choice(choice))
  }

  fn outer_shadow(&self, choice: &a::OuterShadowChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_outer_shadow_choice(choice))
  }

  fn preset_shadow(&self, choice: &a::PresetShadowChoice) -> Option<ResolvedEffectColor> {
    self.resolve(Color::from_preset_shadow_choice(choice))
  }

  fn blip_fill(
    &self,
    fill: &a::BlipFill,
  ) -> Option<common::drawingml_image_effects::ImageEffectFill> {
    let slide = self.slide?;
    let blip = fill.blip.as_ref()?;
    let resource = slide.image_resources.get(blip.embed.as_deref()?)?;
    let effects = common::drawingml_image_effects::from_blip_choices(
      &blip.blip_choice,
      resource.content_type.as_deref(),
      self,
    );
    common::drawingml_image_effects::raster_fill_image(
      &resource.data,
      resource.content_type.as_deref(),
      &effects,
    )
  }
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
  if effects.0.is_empty() {
    return ImportedImageData {
      data: Arc::clone(data),
      content_type: content_type.map(str::to_string),
    };
  }
  let Some(data) =
    common::drawingml_image_effects::apply(data_ref, content_type, effects.0.as_slice())
  else {
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
  ImageEffects(common::drawingml_image_effects::from_blip_choices(
    blip_choices,
    content_type,
    &PptxImageEffectColorResolver {
      import,
      slide: Some(slide),
    },
  ))
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

#[cfg(test)]
fn office_alpha_modulate_amount(value: DrawingmlPercentageValue) -> f32 {
  common::drawingml_image_effects::office_alpha_modulate_amount(value)
}

#[cfg(test)]
fn duotone_component(luminance: u8, first: u8, second: u8) -> u8 {
  common::drawingml_image_effects::duotone_component(luminance, first, second)
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
  source_path: &[usize],
  text_body: &TextBody,
  items: &mut Vec<PageItem>,
  summary: Option<&mut PptxLayoutSummary>,
) {
  let item_start = items.len();
  let adjusted_text_body;
  let visible_shape_rotation = shape_visual_rotation_degrees(shape);
  let text_body = if visible_shape_rotation.abs() > f32::EPSILON
    && text_body
      .display_properties
      .text_camera_z_rotation
      .is_some()
  {
    adjusted_text_body = {
      let mut text_body = text_body.clone();
      let shape_rotation = (visible_shape_rotation * 60_000.0).round() as i32;
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
  // A preset text warp is shape geometry, so its paths use the outer shape
  // coordinate space. The bodyPr insets still constrain ordinary paragraph
  // layout before ECMA-376 Part 1 §20.1.9.19 computes the tight source
  // rectangle, but they must not inset the destination paths. LibreOffice's
  // FontWork renderer likewise fits outlines to the custom-shape geometry and
  // explicitly excludes text-frame margins from FontWork geometry.
  let word_art_target_frame = TextFrame {
    x_pt: offset.x_pt(shape.position.x),
    y_pt: offset.y_pt(shape.position.y),
    width_pt: offset.width_pt(shape.size.cx),
    height_pt: offset.height_pt(shape.size.cy),
  };
  lower_text_body_at_with_font_ref(
    TextBodyLoweringContext {
      import: context.import,
      slide: Some(context.slide),
      image_resources: Some(&context.slide.image_resources),
      page_index: context.page_index,
    },
    text_box,
    word_art_target_frame,
    text_body,
    (
      shape
        .shape_style_refs
        .as_ref()
        .map(|style| &style.font_reference),
      shape.hyperlink_url.as_deref(),
    ),
    summary,
    items,
  );
  for item in &mut items[item_start..] {
    if let PageItem::Text(text) = item {
      text.source_path = source_path.to_vec();
    }
  }
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
      table_cell: true,
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
  word_art_target_frame: TextFrame,
  text_body: &TextBody,
  style_references: (Option<&FontStyleReference>, Option<&str>),
  summary: Option<&mut PptxLayoutSummary>,
  items: &mut Vec<PageItem>,
) {
  let (font_reference, shape_hyperlink_url) = style_references;
  lower_text_body_at_with_style(
    context.import,
    frame,
    text_body,
    TextStyleLoweringInputs {
      font_reference,
      shape_hyperlink_url,
      word_art_target_frame: Some(word_art_target_frame),
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
  table_cell: bool,
  shape_hyperlink_url: Option<&'a str>,
  base_font_size_pt: Option<f32>,
  auto_fit_font_scale: Option<f32>,
  rotation_center_pt: Option<(f32, f32)>,
  word_art_target_frame: Option<TextFrame>,
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
  // DrawingML shapes and table cells expose the same anchor tokens, but
  // LibreOffice intentionally imports `just` and `dist` differently: regular
  // shapes use centered vertical adjustment while table cells keep top.
  let y_pt = match text_body.display_properties.anchor {
    a::TextAnchoringTypeValues::Center => frame.y_pt + (frame.height_pt - estimated_height) / 2.0,
    a::TextAnchoringTypeValues::Justified | a::TextAnchoringTypeValues::Distributed
      if !style_inputs.table_cell =>
    {
      frame.y_pt + (frame.height_pt - estimated_height) / 2.0
    }
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
  apply_word_art_transform(
    &mut items[item_start..],
    style_inputs.word_art_target_frame.unwrap_or(frame),
    frame,
    text_body,
    &mut text_metrics,
  );
  apply_text_camera_z_rotation(
    &mut items[item_start..],
    text_body.display_properties.camera_z_rotation_degrees(),
    &mut text_metrics,
  );
  // Materialize character effects before the owning shape's effect graph is
  // evaluated. This preserves DrawingML's inner-text-then-outer-shape
  // composition order; the slide-wide pass also catches chart text produced
  // outside this text-body lowerer.
  materialize_drawingml_text_effects(&mut items[item_start..], &mut text_metrics);
}

fn apply_word_art_transform(
  items: &mut [PageItem],
  target_frame: TextFrame,
  paint_frame: TextFrame,
  text_body: &TextBody,
  text_metrics: &mut TextMetrics,
) {
  let Some(preset) = text_body
    .display_properties
    .preset_text_warp_geometry
    .as_deref()
    .filter(|preset| preset.preset != a::TextShapeValues::TextNoShape)
  else {
    return;
  };
  crate::common::drawingml_text_warp::apply_to_text_items(
    items,
    preset,
    common::Rect {
      origin: common::Point {
        x: common::Pt(target_frame.x_pt),
        y: common::Pt(target_frame.y_pt),
      },
      size: common::Size {
        width: common::Pt(target_frame.width_pt),
        height: common::Pt(target_frame.height_pt),
      },
    },
    common::Rect {
      origin: common::Point {
        x: common::Pt(paint_frame.x_pt),
        y: common::Pt(paint_frame.y_pt),
      },
      size: common::Size {
        width: common::Pt(paint_frame.width_pt),
        height: common::Pt(paint_frame.height_pt),
      },
    },
    text_metrics,
    None,
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
  // LibreOffice's SdrTest::test3DRotatedText and
  // CustomshapesTest::testTdf126060_3D_Z_Rotation rotate scene3d text around
  // the tight, laid-out text bounds rather than the owning shape or its line
  // box. Use the same glyph bounds already consumed by character effects;
  // TextItem::y_pt is the line-box top, not a baseline coordinate.
  let Some((left, top, right, bottom)) = text_items_ink_bounds(items, text_metrics) else {
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

fn materialize_drawingml_text_effects(items: &mut [PageItem], text_metrics: &mut TextMetrics) {
  for item in items {
    if let PageItem::Group { items, .. } = item {
      materialize_drawingml_text_effects(items, text_metrics);
      continue;
    }
    let PageItem::Text(text) = item else {
      continue;
    };
    let Some((ink_left, ink_top, ink_right, ink_bottom)) =
      pptx_text_item_ink_bounds(text, text_metrics)
    else {
      continue;
    };
    let ink_width = (ink_right - ink_left).max(f32::EPSILON);
    let ink_height = (ink_bottom - ink_top).max(f32::EPSILON);
    let mut effects = text.style.drawingml_text_effects.clone().unwrap_or(
      common::drawingml_image_effects::ImageEffectContainer {
        kind: common::drawingml_image_effects::ImageEffectContainerKind::Sibling,
        effects: Vec::new(),
      },
    );
    let static3d = text.style.drawingml_text_static3d.clone();
    if effects.effects.is_empty() && static3d.is_none() {
      continue;
    }
    // The scaled-shadow restriction belongs to ordinary `spPr/effectLst`.
    // PowerPoint applies the authored scale/skew on character `rPr/effectLst`
    // shadows (for example Text_withShadow_100chars.pptx).
    common::drawingml_image_effects::rotate_container_with_shape(
      &mut effects,
      text.style.rotation_deg,
    );
    let simple_text_glow = simple_shape_glow(&effects).filter(|_| static3d.is_none());
    let preserve_visible_text = simple_text_glow.is_some()
      && !text.style.pdf_glyph_outlines
      && text.style.outline_width_pt <= f32::EPSILON;
    let transparent_semantic_style = simple_text_glow.is_some() && !preserve_visible_text;
    let Some(output_bounds) =
      common::drawingml_image_effects::container_output_bounds(&effects, ink_width, ink_height)
    else {
      continue;
    };
    let static_padding = static3d
      .as_ref()
      .map(|style| {
        common::drawingml_3d::output_padding(
          common::drawingml_3d::camera_projection(&style.scene, text.style.rotation_deg),
          &style.shape,
          ink_width,
          ink_height,
        )
      })
      .unwrap_or_default();
    let relative_left = output_bounds.left_pt.min(0.0) - static_padding.left_pt;
    let relative_top = output_bounds.top_pt.min(0.0) - static_padding.top_pt;
    let relative_right = output_bounds.right_pt.max(ink_width) + static_padding.right_pt;
    let relative_bottom = output_bounds.bottom_pt.max(ink_height) + static_padding.bottom_pt;
    let raster_bounds = common::Rect {
      origin: common::Point {
        x: common::Pt(ink_left + relative_left),
        y: common::Pt(ink_top + relative_top),
      },
      size: common::Size {
        width: common::Pt(relative_right - relative_left),
        height: common::Pt(relative_bottom - relative_top),
      },
    };
    let source_text = text.clone();
    let source_item = common::DisplayItem::Text(common_text_run(source_text.clone()));
    let automatic_extrusion_color = common::drawingml_3d::automatic_extrusion_color_from_items(
      std::slice::from_ref(&source_item),
    );
    let Some(mut raster) =
      common::drawingml_shape_raster::rasterize_vector_items_for_effects_at_pixels_per_point(
        std::slice::from_ref(&source_item),
        raster_bounds,
        &effects,
        96.0 / 72.0,
      )
    else {
      continue;
    };
    // Office fixed output emits character-effect images at approximately its
    // 96-DPI drawing baseline (the reference fixture stores 252 px over
    // 181.08 pt), independently of larger shape-effect raster caps.
    if let Some(style) = static3d.as_ref() {
      common::drawingml_3d::apply_static_3d(
        &mut raster.image,
        &style.scene,
        common::drawingml_3d::camera_projection(&style.scene, text.style.rotation_deg),
        &style.shape,
        common::drawingml_3d::Static3dRenderOptions {
          extrusion_color: style.extrusion_color.or(automatic_extrusion_color),
          contour_color: style.contour_color,
          pixels_per_point: raster.pixels_per_point,
          model_surface: Some(common::drawingml_3d::Static3dSurface {
            left_px: (ink_left - raster_bounds.origin.x.0) * raster.pixels_per_point,
            top_px: (ink_top - raster_bounds.origin.y.0) * raster.pixels_per_point,
            width_px: ink_width * raster.pixels_per_point,
            height_px: ink_height * raster.pixels_per_point,
          }),
        },
      );
    }
    if preserve_visible_text && let Some(glow) = simple_text_glow {
      effects = common::drawingml_image_effects::ImageEffectContainer {
        kind: common::drawingml_image_effects::ImageEffectContainerKind::Sibling,
        effects: vec![glow],
      };
    }
    // PowerPoint reserves the full authored character-glow radius in the
    // effect bitmap but paints only the inner third as the glow filter. This
    // leaves transparent padding visible in its fixed-output image box.
    common::drawingml_image_effects::scale_glow_filter_radius(&mut effects, 1.0 / 3.0);
    common::drawingml_image_effects::scale_container_pixel_lengths(
      &mut effects,
      raster.pixels_per_point / (96.0 / 72.0),
    );
    common::drawingml_image_effects::apply_container_to_padded_image_with_sources(
      &mut raster.image,
      &effects,
      -relative_left * raster.pixels_per_point,
      -relative_top * raster.pixels_per_point,
      ink_width * raster.pixels_per_point,
      ink_height * raster.pixels_per_point,
      common::drawingml_image_effects::ImageEffectSourceImages {
        fill: raster.fill_image.as_ref(),
        line: raster.line_image.as_ref(),
        fill_line: raster.fill_line_image.as_ref(),
        children: raster.children_image.as_ref(),
      },
    );
    let mut png = Cursor::new(Vec::new());
    if PngEncoder::new(&mut png)
      .write_image(
        raster.image.as_raw(),
        raster.image.width(),
        raster.image.height(),
        ColorType::Rgba8.into(),
      )
      .is_err()
    {
      continue;
    }
    let mut group_items = vec![PageItem::Image(ImageItem {
      x_pt: raster_bounds.origin.x.0,
      y_pt: raster_bounds.origin.y.0,
      width_pt: raster_bounds.size.width.0,
      height_pt: raster_bounds.size.height.0,
      crop: ImageCrop::default(),
      clip_path: Vec::new(),
      rotation_deg: 0.0,
      flip_horizontal: false,
      flip_vertical: false,
      data: Arc::from(png.into_inner()),
      content_type: Some("image/png".to_string()),
      metafile_monochrome_dib_palette_override: None,
      metafile_background_color: None,
      metafile_external_header: None,
      metafile_semantic_text_includes_raster_backdrop: false,
      alt_text: None,
      hyperlink_url: text.hyperlink_url.clone(),
      floating: false,
      behind_text: false,
    })];
    if preserve_visible_text {
      let mut visible_text = source_text;
      visible_text.style.drawingml_text_effects = None;
      visible_text.style.drawingml_text_static3d = None;
      group_items.push(PageItem::Text(visible_text));
    } else {
      let preserve_semantic_overlay = pptx_text_preserves_semantic_overlay(&source_text);
      let mut semantic_text = source_text;
      semantic_text.y_pt += pptx_text_baseline_offset(&semantic_text, text_metrics);
      semantic_text.style.semantic_only = true;
      semantic_text.style.drawingml_text_effects = None;
      semantic_text.style.drawingml_text_static3d = None;
      semantic_text.style.color = RgbColor { r: 0, g: 0, b: 0 };
      if transparent_semantic_style {
        semantic_text.style.opacity = 0.0;
      }
      semantic_text.style.outline_color = None;
      semantic_text.style.outline_width_pt = 0.0;
      semantic_text.style.pdf_glyph_outlines = false;
      semantic_text.style.pdf_glyph_outline_options = None;
      if preserve_semantic_overlay {
        group_items.push(PageItem::Text(semantic_text));
      }
    }
    *item = PageItem::Group {
      mask: None,
      clip: None,
      transform: None,
      blend_mode: common::BlendMode::Normal,
      opacity: 1.0,
      items: group_items,
    };
  }
}

fn pptx_text_baseline_offset(text: &TextItem, text_metrics: &mut TextMetrics) -> f32 {
  if text.style.use_windows_font_metrics {
    text_metrics.baseline_offset_in_line_with_windows_metrics_for_text(
      &text.text,
      &text.style,
      text.line_height_pt,
    )
  } else {
    text_metrics.baseline_offset_in_line_for_text(&text.text, &text.style, text.line_height_pt)
  }
}

fn collect_pptx_semantic_text_overlays(
  items: &[PageItem],
  output: &mut Vec<PageItem>,
  text_metrics: &mut TextMetrics,
) {
  for item in items {
    match item {
      PageItem::Text(text) => {
        if !pptx_text_preserves_semantic_overlay(text) {
          continue;
        }
        let mut text = text.clone();
        if !text.style.semantic_only {
          text.y_pt += pptx_text_baseline_offset(&text, text_metrics);
          text.style.semantic_only = true;
        }
        text.style.drawingml_text_effects = None;
        text.style.drawingml_text_static3d = None;
        output.push(PageItem::Text(text));
      }
      PageItem::Group { items, .. } => {
        collect_pptx_semantic_text_overlays(items, output, text_metrics);
      }
      PageItem::Image(_)
      | PageItem::LinkArea(_)
      | PageItem::Path(_)
      | PageItem::Rect(_)
      | PageItem::Line(_) => {}
    }
  }
}

fn pptx_text_preserves_semantic_overlay(text: &TextItem) -> bool {
  text
    .style
    .pdf_glyph_outline_options
    .as_ref()
    .is_none_or(|options| options.semantic_text_overlay)
}

fn lift_pptx_semantic_text_overlays(items: &mut Vec<PageItem>) {
  let source = std::mem::take(items);
  for mut item in source {
    let lift = if let PageItem::Group {
      mask,
      clip,
      transform,
      blend_mode,
      opacity,
      items: children,
    } = &mut item
    {
      lift_pptx_semantic_text_overlays(children);
      mask.is_none()
        && clip.is_none()
        && transform.is_none()
        && *blend_mode == common::BlendMode::Normal
        && (*opacity - 1.0).abs() <= f32::EPSILON
        && children.len() == 2
        && matches!(children.first(), Some(PageItem::Image(_)))
        && matches!(children.get(1), Some(PageItem::Text(_)))
    } else {
      false
    };
    if lift
      && let PageItem::Group {
        items: children, ..
      } = item
    {
      items.extend(children);
    } else {
      items.push(item);
    }
  }
}

fn pptx_text_item_ink_bounds(
  text: &TextItem,
  text_metrics: &mut TextMetrics,
) -> Option<(f32, f32, f32, f32)> {
  let baseline_offset = if text.style.use_windows_font_metrics {
    text_metrics.baseline_offset_in_line_with_windows_metrics_for_text(
      &text.text,
      &text.style,
      text.line_height_pt,
    )
  } else {
    text_metrics.baseline_offset_in_line_for_text(&text.text, &text.style, text.line_height_pt)
  };
  let baseline = text.y_pt + baseline_offset;
  let mut glyph_x = text.x_pt;
  let mut bounds: Option<(f32, f32, f32, f32)> = None;
  let shaped = text_metrics.shape_text(&text.text, &text.style)?;
  for glyph in shaped.glyphs {
    let font_size = glyph.font_size_pt;
    if let Some(glyph_bounds) = glyph.bounds_em {
      let glyph_left = glyph_x + (glyph.x_offset_em + glyph_bounds.x_min_em) * font_size;
      let glyph_right = glyph_x + (glyph.x_offset_em + glyph_bounds.x_max_em) * font_size;
      let glyph_top = baseline - (glyph.y_offset_em + glyph_bounds.y_max_em) * font_size;
      let glyph_bottom = baseline - (glyph.y_offset_em + glyph_bounds.y_min_em) * font_size;
      bounds = Some(match bounds {
        Some((old_left, old_top, old_right, old_bottom)) => (
          old_left.min(glyph_left),
          old_top.min(glyph_top),
          old_right.max(glyph_right),
          old_bottom.max(glyph_bottom),
        ),
        None => (glyph_left, glyph_top, glyph_right, glyph_bottom),
      });
    }
    glyph_x += glyph.x_advance_em * font_size;
  }
  bounds
}

fn text_items_ink_bounds(
  items: &[PageItem],
  text_metrics: &mut TextMetrics,
) -> Option<(f32, f32, f32, f32)> {
  let mut bounds: Option<(f32, f32, f32, f32)> = None;
  for item in items {
    let PageItem::Text(text) = item else {
      continue;
    };
    let Some((left, top, right, bottom)) = pptx_text_item_ink_bounds(text, text_metrics) else {
      continue;
    };
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
  let vectorize_without_semantic_overlay = text_body
    .display_properties
    .text_area_rotation
    .is_some_and(|rotation| rotation != 0)
    || text_body
      .body_properties
      .as_deref()
      .is_some_and(|properties| properties.scene3_d_type.is_some());
  let pdf_glyph_outlines =
    vectorize_without_semantic_overlay || text_body.display_properties.from_word_art;
  // DrawingML shape creation seeds all three script families from the
  // current theme's minor font collection before paragraph/run formatting is
  // applied. LibreOffice does this in oox/source/drawingml/shape.cxx; direct
  // a:latin/a:ea/a:cs values below still override these inherited defaults.
  let theme_latin = import
    .resolve_theme_font_for_slide(slide, "+mn-lt")
    .unwrap_or("Liberation Sans");
  let mut base_style = TextStyle {
    font_family: Some(Arc::from(theme_latin)),
    fallback_font_family: Some(Arc::from(theme_latin)),
    font_size_pt: base_font_size_pt.unwrap_or(DEFAULT_TEXT_FONT_SIZE_PT),
    use_windows_font_metrics: true,
    rotation_deg: options.rotation_deg,
    // PowerPoint's fixed-format writer emits bodyPr text-area rotation and
    // scene3d text as vector glyph outlines, while ordinary shape rotation
    // remains searchable PDF text. Keep that distinction before both are
    // lowered into the same final rotation angle.
    pdf_glyph_outlines,
    pdf_glyph_outline_options: pdf_glyph_outlines.then(|| {
      Arc::new(common::PdfGlyphOutlineOptions {
        semantic_text_overlay: !vectorize_without_semantic_overlay,
        fill: None,
        outline_fill: None,
        outline_stroke: None,
        transform: None,
        text_warp: None,
      })
    }),
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
  base_style.drawingml_text_static3d =
    drawingml_text_static3d(import, slide, text_body.body_properties.as_deref());
  base_style
}

fn drawingml_text_static3d(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  properties: Option<&a::BodyProperties>,
) -> Option<common::drawingml_3d::Static3dStyle> {
  let properties = properties?;
  let scene = properties.scene3_d_type.as_ref()?;
  let shape = match properties.body_properties_choice2.as_ref()? {
    a::BodyPropertiesChoice2::Shape3DType(shape) => shape,
    a::BodyPropertiesChoice2::FlatText(_) => return None,
  };
  let resolver = PptxImageEffectColorResolver { import, slide };
  let extrusion_color = shape
    .extrusion_color
    .as_deref()
    .and_then(|color| color.extrusion_color_choice.as_ref())
    .and_then(Color::from_extrusion_color_choice)
    .and_then(|color| resolver.resolve(Some(color)))
    .map(|color| common::drawingml_3d::Static3dColor {
      color: color.color,
      alpha: color.alpha,
    });
  let contour_color = shape
    .contour_color
    .as_deref()
    .and_then(|color| color.contour_color_choice.as_ref())
    .and_then(Color::from_contour_color_choice)
    .and_then(|color| resolver.resolve(Some(color)))
    .map(|color| common::drawingml_3d::Static3dColor {
      color: color.color,
      alpha: color.alpha,
    });
  Some(common::drawingml_3d::Static3dStyle {
    scene: scene.clone(),
    shape: shape.clone(),
    extrusion_color,
    contour_color,
  })
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
  let point = Affine::rotate_about(f64::from(angle), (f64::from(center_x), f64::from(center_y)))
    * kurbo::Point::new(f64::from(x), f64::from(y));
  (point.x as f32, point.y as f32)
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

#[derive(Clone)]
struct TextLineRun<'a> {
  run: &'a TextRun,
  text: String,
  width_pt: f32,
  style: TextStyle,
  kind: TextLineRunKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TextLineRunKind {
  Text,
  Tab,
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
  right_to_left_columns: bool,
  word_wrap: bool,
  clip_vertical_overflow: bool,
  clip_bottom_extension_pt: f32,
  anchor_center: bool,
}

impl TextLoweringOptions {
  fn from_text_body(text_body: &TextBody) -> Self {
    const DEFAULT_VERTICAL_INSET_EMU: i64 = 45_720;
    let bottom_inset_pt = text_body
      .body_properties
      .as_deref()
      .and_then(|properties| properties.bottom_inset)
      .map(|value| units::emu_to_points(value.to_emu()))
      .unwrap_or_else(|| units::emu_to_points(DEFAULT_VERTICAL_INSET_EMU));
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
      right_to_left_columns: text_body.display_properties.right_to_left_columns,
      word_wrap: text_body.display_properties.word_wrap,
      // PowerPoint's fixed-output path clips a no-autofit text frame at the
      // shape edge even when bodyPr leaves vertOverflow at its schema default.
      // Shape autofit is the exception because it grows the shape instead.
      clip_vertical_overflow: text_body.display_properties.auto_fit == TextAutoFit::None
        || (text_body.display_properties.clip_vertical_overflow
          && text_body.display_properties.auto_fit != TextAutoFit::Shape),
      // TextBodyFrame excludes the bottom inset, while PowerPoint clips fixed
      // output at the outer shape edge. Add that inset back to the clip bound.
      clip_bottom_extension_pt: bottom_inset_pt,
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
    offset.x_pt(shape.position.x),
    offset.y_pt(shape.position.y),
    offset.width_pt(shape.size.cx),
    offset.height_pt(shape.size.cy),
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
  let mut paragraph_style = ParagraphDisplayStyle::from_paragraph(paragraph);
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
  let logical_column_index = cursor
    .column_index
    .min(context.options.column_count.saturating_sub(1));
  let visual_column_index = if context.options.right_to_left_columns {
    context.options.column_count - logical_column_index - 1
  } else {
    logical_column_index
  };
  let column_x = context.frame.x_pt
    + visual_column_index as f32 * (column_width + context.options.column_spacing_pt);
  cursor.x_pt = column_x;
  if context.options.clip_vertical_overflow
    && cursor.y_pt
      > context.frame.y_pt + context.frame.height_pt + context.options.clip_bottom_extension_pt
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
    &paragraph_style,
    &bullet,
  );
  paragraph_style.apply_diagram_autofit_spacing_scale(paragraph, context.options);
  let paragraph_leading_offset = paragraph_style.left_offset(bullet.label.is_some());
  let paragraph_x = cursor.x_pt
    + if paragraph_style.right_to_left {
      paragraph_style.right_margin_pt
    } else {
      paragraph_leading_offset
    };
  // ECMA-376 Part 1 §21.1.2.2.7 defines marL and marR in addition to the
  // text-body insets. They therefore reduce the paragraph's line box as well
  // as moving its origin; using the full column width lets indented text run
  // past the right edge before wrapping.
  let paragraph_width = paragraph_style.available_width(column_width, paragraph_leading_offset);
  let mut segment_start = 0usize;
  let mut is_first_segment = true;

  loop {
    let segment_end = paragraph.runs[segment_start..]
      .iter()
      .position(|run| run.kind == TextRunKind::Break)
      .map(|offset| segment_start + offset)
      .unwrap_or(paragraph.runs.len());
    let mut text_lines = layout_text_lines(
      TextLineLayoutContext {
        import: context.import,
        slide: context.slide,
        base_style: &paragraph_base_style,
        options: context.options,
        column_width: paragraph_width,
        slide_number: context.slide_number,
        east_asian_line_break: paragraph_style.east_asian_line_break,
        latin_line_break: paragraph_style.latin_line_break,
        default_tab_size_pt: paragraph_style.default_tab_size_pt,
        tab_stops: &paragraph_style.tab_stops,
        hanging_punctuation: paragraph_style.hanging_punctuation,
      },
      &paragraph.runs[segment_start..segment_end],
      text_metrics,
    );
    for text_line in &mut text_lines {
      reorder_text_line_bidi(text_line, paragraph_style.right_to_left, text_metrics);
    }
    let is_soft_break_empty_line =
      segment_start == segment_end && (segment_start > 0 || segment_end < paragraph.runs.len());
    let alignment = if context.options.anchor_center {
      // maps horizontal text with anchorCtr=1 to TextHorizontalAdjust_CENTER,
      // so the shape-level adjustment overrides paragraph alignment.
      a::TextAlignmentTypeValues::Center
    } else {
      paragraph_style.alignment
    };

    for (line_index, text_line) in text_lines.iter().enumerate() {
      let line_adjustment = paragraph_line_adjustment(
        alignment,
        line_index + 1 == text_lines.len(),
        text_line,
        paragraph_width,
      );
      let mut remaining_distributed_graphemes = match line_adjustment {
        ParagraphLineAdjustment::Grapheme { count, .. } => count,
        ParagraphLineAdjustment::None | ParagraphLineAdjustment::Word { .. } => 0,
      };
      let mut run_x =
        aligned_paragraph_x(paragraph_x, paragraph_width, text_line.width_pt, alignment);
      let base_line_style = paragraph_base_style.clone();
      // ECMA-376 Part 1 §21.1.2.2.5: without explicit lnSpc, spacing is
      // determined by the largest piece of text in the line. The inherited
      // paragraph default is only a fallback for an empty line; treating it
      // as a minimum moves explicitly smaller text upward in bottom-anchored
      // placeholders.
      let mut max_line_height = if text_line.runs.is_empty() {
        if is_soft_break_empty_line {
          paragraph_style.soft_break_empty_line_height(&base_line_style, context.options)
        } else {
          paragraph_style.line_height(&base_line_style, context.options)
        }
      } else {
        0.0
      };
      for line_run in &text_line.runs {
        max_line_height =
          max_line_height.max(paragraph_style.line_height(&line_run.style, context.options));
      }
      let common_baseline_offset = if matches!(
        paragraph_style.font_alignment,
        a::TextFontAlignmentValues::Automatic | a::TextFontAlignmentValues::Baseline
      ) && !text_line.runs.is_empty()
      {
        Some(text_line.runs.iter().fold(0.0_f32, |offset, line_run| {
          let run_line_height = paragraph_style.line_height(&line_run.style, context.options);
          // A run-level baseline shift positions only that run relative to
          // the line baseline (ECMA-376 §21.1.2.3.9). It must not select a
          // different common baseline for every other run in the line.
          let mut unshifted_style = line_run.style.clone();
          unshifted_style.baseline_shift_pt = 0.0;
          offset.max(paragraph_style.baseline_offset(
            &unshifted_style,
            run_line_height,
            text_metrics,
          ))
        }))
      } else {
        None
      };

      if is_first_segment
        && line_index == 0
        && let Some(label) = bullet.label.as_deref()
      {
        let label = shared_symbol::font_symbol_transport_text(bullet.font.as_deref(), label);
        // DrawingML's follow-text bullet properties use the first character
        // in the paragraph. `line_run.style` already includes inheritance and
        // auto-fit scaling, so it is also the correct base for explicit bullet
        // font/color/size overrides.
        let mut bullet_style = paragraph.runs[segment_start..segment_end]
          .iter()
          .find(|run| {
            matches!(
              run.kind,
              TextRunKind::Run | TextRunKind::Field | TextRunKind::Math
            ) && (!run.text.is_empty() || presentation_field_may_generate_text(context.import, run))
          })
          .map(|run| {
            styled_text_run(
              context.import,
              context.slide,
              &paragraph_base_style,
              context.options,
              run,
            )
          })
          .unwrap_or_else(|| paragraph_base_style.clone());
        if bullet.auto_number.is_none() && !bullet.font_follows_text {
          bullet_style.bold = false;
          bullet_style.italic = false;
          bullet_style.underline = false;
          bullet_style.strikethrough = false;
        }
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
        let bullet_text_height = text_metrics.vertical_metrics(&bullet_style).ink_height_pt();
        // The bullet is positioned relative to the paragraph baseline, but
        // its buSzPct/buSzPts size does not participate in the line's text
        // height. PowerPoint keeps the line spacing based on the paragraph
        // runs even when the bullet is 120% or 130% of the text size.
        let text_baseline_offset = common_baseline_offset.unwrap_or_else(|| {
          paragraph_style.baseline_offset(&base_line_style, max_line_height, text_metrics)
        });
        let bullet_baseline_offset =
          raw_baseline_offset(&bullet_style, bullet_line_height, text_metrics);
        let bullet_y_pt = cursor.y_pt + text_baseline_offset - bullet_baseline_offset;
        if let Some(mut graphic) = bullet_graphic_item(
          &bullet,
          context.image_resources,
          paragraph_bullet_x(
            run_x,
            text_line.width_pt,
            paragraph_style.indent_pt,
            sdk_units::mm100_to_points100(i64::from(bullet.graphic_width_100mm.unwrap_or_default()))
              as f32
              / 100.0,
            paragraph_style.right_to_left,
          ),
          bullet_y_pt,
          bullet_line_height,
          bullet_text_height,
          context.shape_hyperlink_url,
        ) {
          graphic.x_pt = picture_bullet_x_pt(
            graphic.x_pt,
            graphic.width_pt,
            text_line.width_pt,
            alignment,
          );
          items.push(PageItem::Image(graphic));
        } else {
          let bullet_width_pt = text_metrics.measure_text(label.as_ref(), &bullet_style);
          if paragraph_style.right_to_left {
            bullet_style.resolved_bidi_level = Some(1);
          }
          push_text_item(
            items,
            TextItemPlacement {
              x_pt: paragraph_bullet_x(
                run_x,
                text_line.width_pt,
                paragraph_style.indent_pt,
                bullet_width_pt,
                paragraph_style.right_to_left,
              ),
              y_pt: bullet_y_pt,
              line_height_pt: bullet_line_height,
              rotation_center_pt: context.options.rotation_center_pt,
              paragraph_bidi: paragraph_style.right_to_left,
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
        let raw_baseline_offset = raw_baseline_offset(style, run_line_height, text_metrics);
        let baseline_offset = common_baseline_offset
          .unwrap_or_else(|| paragraph_style.baseline_offset(style, run_line_height, text_metrics));
        let run_y_pt = cursor.y_pt + baseline_offset - raw_baseline_offset;
        if line_run.kind == TextLineRunKind::Text
          && !line_run.text.is_empty()
          && line_run.run.kind == TextRunKind::Math
        {
          push_math_ole_preview_item(items, run_x, run_y_pt, line_run.width_pt, run_line_height);
        }
        let adjusted_run_right = if line_run.kind == TextLineRunKind::Text
          && !line_run.text.is_empty()
        {
          let hyperlink_url = line_run
            .run
            .hyperlink_url
            .clone()
            .or_else(|| context.shape_hyperlink_url.map(ToString::to_string));
          let placement = TextItemPlacement {
            x_pt: run_x,
            y_pt: run_y_pt,
            line_height_pt: run_line_height,
            rotation_center_pt: context.options.rotation_center_pt,
            paragraph_bidi: paragraph_style.right_to_left,
          };
          match line_adjustment {
            ParagraphLineAdjustment::None => {
              push_symbol_split_text_items(
                items,
                placement,
                &line_run.text,
                style,
                hyperlink_url,
                text_metrics,
              );
              None
            }
            ParagraphLineAdjustment::Word { extra_per_space } => Some(push_word_spaced_text_items(
              items,
              placement,
              &line_run.text,
              style,
              hyperlink_url,
              extra_per_space,
              text_metrics,
            )),
            ParagraphLineAdjustment::Grapheme { extra_per_gap, .. } => {
              Some(push_grapheme_distributed_text_items(
                items,
                placement,
                &line_run.text,
                style,
                hyperlink_url,
                GraphemeDistribution {
                  extra_per_gap,
                  remaining: &mut remaining_distributed_graphemes,
                },
                text_metrics,
              ))
            }
          }
        } else {
          None
        };
        run_x = adjusted_run_right.unwrap_or(run_x + line_run.width_pt);
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
  east_asian_line_break: bool,
  latin_line_break: bool,
  default_tab_size_pt: f32,
  tab_stops: &'a [ParagraphTabStop],
  hanging_punctuation: bool,
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
      ) && (!run.text.is_empty() || presentation_field_may_generate_text(context.import, run))
    })
    .collect::<Vec<_>>();
  if visible_runs.is_empty() {
    return vec![TextLine::default()];
  }

  let legacy_lines = layout_text_lines_legacy(context, runs, text_metrics);
  if !context.options.word_wrap {
    return legacy_lines;
  }
  // DrawingML tabs are positional controls, not glyph advances. Parley's
  // Unicode line breaker does not carry paragraph tab-stop geometry, so keep
  // tabbed paragraphs on the Office-aware path below.
  if visible_runs.iter().any(|run| run.text.contains('\t')) {
    return legacy_lines;
  }

  let mut prepared_runs = visible_runs
    .into_iter()
    .map(|run| {
      let style = styled_text_run(
        context.import,
        context.slide,
        context.base_style,
        context.options,
        run,
      );
      let field_text = presentation_field_text(
        run,
        context.slide_number,
        context.import.field_update_datetime,
        context.import.field_format_locale.as_deref(),
        style.language.as_deref(),
      );
      let text = if style.uppercase {
        field_text.to_uppercase()
      } else {
        field_text.into_owned()
      };
      PreparedTextRun {
        run,
        text,
        style,
        range: 0..0,
      }
    })
    .collect::<Vec<_>>();
  let mut text = String::new();
  for run in &mut prepared_runs {
    let start = text.len();
    text.push_str(&run.text);
    run.range = start..text.len();
  }
  let joins_word_across_run = prepared_runs
    .windows(2)
    .any(|runs| text_spans_join_without_break(runs[0].text.as_str(), runs[1].text.as_str()));
  let legacy_overflows = legacy_lines
    .iter()
    .any(|line| line.width_pt > context.column_width + 0.01);
  let has_dictionary_or_east_asian_text = text.chars().any(parley_line_break_script);
  let has_east_asian_text = text.chars().any(east_asian_line_break_script);
  // eaLnBrk=true permits the East Asian word to wrap without inserting a
  // hyphen; false keeps that word intact. latinLnBrk has the corresponding
  // emergency-break meaning for Latin words (ECMA-376 Part 1, 21.1.2.2.7).
  // Do not invert the East Asian flag here: Parley's normal Unicode break
  // opportunities are the enabled behavior, while the legacy whole-token
  // path preserves a disabled East Asian word.
  let needs_emergency_breaking = context.latin_line_break;
  let needs_span_aware_breaking = (joins_word_across_run
    && (context.east_asian_line_break || !has_east_asian_text))
    || (has_dictionary_or_east_asian_text
      && (context.east_asian_line_break || !has_east_asian_text)
      && legacy_overflows)
    || (needs_emergency_breaking && legacy_overflows);
  if !needs_span_aware_breaking {
    return legacy_lines;
  }
  let spans = prepared_runs
    .iter()
    .map(|run| StyledTextSpan {
      range: run.range.clone(),
      style: &run.style,
    })
    .collect::<Vec<_>>();
  let max_advance = context
    .options
    .word_wrap
    .then_some(context.column_width.max(0.0));
  let Some(mut line_ranges) = break_text_lines(&text, &spans, max_advance, text_metrics) else {
    return legacy_lines;
  };
  if line_ranges.is_empty() {
    return vec![TextLine::default()];
  }
  if needs_emergency_breaking {
    line_ranges = emergency_break_text_ranges(
      &text,
      line_ranges,
      &prepared_runs,
      context.column_width,
      text_metrics,
    );
  }
  if context.hanging_punctuation {
    apply_hanging_punctuation_to_ranges(&text, &mut line_ranges);
  }

  let parley_lines = line_ranges
    .into_iter()
    .map(|range| text_line_from_range(&text, range, &prepared_runs, text_metrics))
    .collect::<Vec<_>>();
  if parley_lines.iter().any(|line| {
    text_line_fit_width(line, context.hanging_punctuation, text_metrics)
      > context.column_width + 0.01
  }) {
    legacy_lines
  } else {
    parley_lines
  }
}

fn apply_hanging_punctuation_to_ranges(text: &str, ranges: &mut Vec<Range<usize>>) {
  let mut index = 0usize;
  while index + 1 < ranges.len() {
    if ranges[index].end != ranges[index + 1].start {
      index += 1;
      continue;
    }
    let next_start = ranges[index + 1].start;
    let Some(next_text) = text.get(ranges[index + 1].clone()) else {
      index += 1;
      continue;
    };
    let Some(character) = next_text.chars().next() else {
      ranges.remove(index + 1);
      continue;
    };
    if !is_hanging_punctuation(character) {
      index += 1;
      continue;
    }
    let boundary = next_start + character.len_utf8();
    ranges[index].end = boundary;
    ranges[index + 1].start = boundary;
    if ranges[index + 1].is_empty() {
      ranges.remove(index + 1);
    }
  }
}

fn emergency_break_text_ranges(
  text: &str,
  ranges: Vec<Range<usize>>,
  runs: &[PreparedTextRun<'_>],
  max_width_pt: f32,
  text_metrics: &mut TextMetrics,
) -> Vec<Range<usize>> {
  let mut output = Vec::new();
  for range in ranges {
    let measured = text_line_from_range(text, range.clone(), runs, text_metrics);
    if measured.width_pt <= max_width_pt + 0.01 || range.is_empty() {
      output.push(range);
      continue;
    }
    let Some(line_text) = text.get(range.clone()) else {
      output.push(range);
      continue;
    };
    let mut line_start = range.start;
    let mut previous_end = range.start;
    for relative_end in GraphemeClusterSegmenter::new()
      .segment_str(line_text)
      .skip(1)
    {
      let end = range.start + relative_end;
      let candidate = text_line_from_range(text, line_start..end, runs, text_metrics);
      if candidate.width_pt > max_width_pt + 0.01 && previous_end > line_start {
        output.push(line_start..previous_end);
        line_start = previous_end;
      }
      previous_end = end;
    }
    if previous_end > line_start {
      output.push(line_start..previous_end);
    }
  }
  output
}

fn parley_line_break_script(ch: char) -> bool {
  east_asian_line_break_script(ch)
    || matches!(
      ch.script(),
      Script::Thai | Script::Lao | Script::Khmer | Script::Myanmar
    )
}

fn east_asian_line_break_script(ch: char) -> bool {
  matches!(
    ch.script(),
    Script::Han
      | Script::Hiragana
      | Script::Katakana
      | Script::Hangul
      | Script::Bopomofo
      | Script::Yi
  )
}

fn text_spans_join_without_break(left: &str, right: &str) -> bool {
  left
    .chars()
    .next_back()
    .is_some_and(|character| !character.is_whitespace())
    && right
      .chars()
      .next()
      .is_some_and(|character| !character.is_whitespace())
}

struct PreparedTextRun<'a> {
  run: &'a TextRun,
  text: String,
  style: TextStyle,
  range: Range<usize>,
}

fn text_line_from_range<'a>(
  text: &str,
  range: Range<usize>,
  runs: &[PreparedTextRun<'a>],
  text_metrics: &mut TextMetrics,
) -> TextLine<'a> {
  let mut line = TextLine::default();
  for run in runs {
    let start = range.start.max(run.range.start);
    let end = range.end.min(run.range.end);
    if start >= end {
      continue;
    }
    let Some(run_text) = text.get(start..end) else {
      continue;
    };
    let width_pt = text_metrics.measure_text(run_text, &run.style);
    push_text_line_token(&mut line, run.run, run_text, width_pt, &run.style);
  }
  trim_text_line_end(&mut line, text_metrics);
  line
}

fn reorder_text_line_bidi(
  line: &mut TextLine<'_>,
  paragraph_right_to_left: bool,
  text_metrics: &mut TextMetrics,
) {
  let mut logical_text = String::new();
  let mut run_starts = Vec::with_capacity(line.runs.len());
  let mut logical_run_by_byte = Vec::new();
  for (run_index, run) in line.runs.iter().enumerate() {
    run_starts.push(logical_text.len());
    logical_text.push_str(&run.text);
    logical_run_by_byte.extend(std::iter::repeat_n(run_index, run.text.len()));
  }
  if logical_text.is_empty() {
    return;
  }

  // UAX #9 applies L1/L2 visual reordering after line breaking.
  // `layout_text_lines` has already selected this physical line; resolve it
  // with the paragraph's explicit base level so the resulting levels drive
  // both run order and shaping direction on each uniform portion.
  let base_level = if paragraph_right_to_left {
    Level::rtl()
  } else {
    Level::ltr()
  };
  // DrawingML a:rtl is also a directional override for weak and neutral
  // characters, not merely a complex-font selector (ECMA-376
  // §21.1.2.2.8). Resolve that Office layer on an equal-character analysis
  // string, then project UAX #9 levels back to the authored UTF-8 text.
  let logical_chars = logical_text.char_indices().collect::<Vec<_>>();
  let mut analysis_text = String::new();
  let mut analysis_byte_starts = Vec::with_capacity(logical_chars.len());
  for (char_index, &(logical_byte_start, _)) in logical_chars.iter().enumerate() {
    let run = &line.runs[logical_run_by_byte[logical_byte_start]];
    analysis_byte_starts.push(analysis_text.len());
    analysis_text.push(drawingml_bidi_analysis_char(
      &logical_chars,
      char_index,
      &run.style,
    ));
  }
  let bidi = BidiInfo::new(&analysis_text, Some(base_level));
  let mut logical_levels = vec![base_level; logical_text.len()];
  for (char_index, &(logical_byte_start, character)) in logical_chars.iter().enumerate() {
    let level = bidi.levels[analysis_byte_starts[char_index]];
    logical_levels[logical_byte_start..logical_byte_start + character.len_utf8()].fill(level);
  }
  let mut logical_segments = Vec::new();
  let mut segment_levels = Vec::new();
  let mut empty_runs = Vec::new();

  for (run_index, run) in line.runs.iter().enumerate() {
    if run.text.is_empty() {
      empty_runs.push(run.clone());
      continue;
    }
    let global_start = run_starts[run_index];
    let mut segment_start = 0usize;
    let mut segment_level = logical_levels[global_start];
    for (offset, _) in run.text.char_indices().skip(1) {
      let level = logical_levels[global_start + offset];
      if level == segment_level {
        continue;
      }
      push_bidi_text_line_segment(
        &mut logical_segments,
        &mut segment_levels,
        run,
        segment_start..offset,
        segment_level,
        text_metrics,
      );
      segment_start = offset;
      segment_level = level;
    }
    push_bidi_text_line_segment(
      &mut logical_segments,
      &mut segment_levels,
      run,
      segment_start..run.text.len(),
      segment_level,
      text_metrics,
    );
  }
  if logical_segments.is_empty() {
    return;
  }

  let visual_order = BidiInfo::reorder_visual(&segment_levels);
  line.runs = visual_order
    .into_iter()
    .map(|index| logical_segments[index].clone())
    .chain(empty_runs)
    .collect();
  line.width_pt = line.runs.iter().map(|run| run.width_pt).sum();
}

fn drawingml_bidi_analysis_char(
  logical_chars: &[(usize, char)],
  index: usize,
  style: &TextStyle,
) -> char {
  let character = logical_chars[index].1;
  let Some(right_to_left) = style.right_to_left else {
    return character;
  };
  let class = bidi_class(character);
  if drawingml_bidi_numeric_component(logical_chars, index, style, class) {
    return character;
  }
  if !matches!(
    class,
    BidiClass::B
      | BidiClass::BN
      | BidiClass::CS
      | BidiClass::ES
      | BidiClass::ET
      | BidiClass::NSM
      | BidiClass::ON
      | BidiClass::S
      | BidiClass::WS
  ) {
    return character;
  }
  if right_to_left { '\u{05D0}' } else { 'A' }
}

fn drawingml_bidi_numeric_component(
  logical_chars: &[(usize, char)],
  index: usize,
  style: &TextStyle,
  class: BidiClass,
) -> bool {
  // ECMA-376 Part 1 §21.1.2.2.8 excludes these weak classes from the
  // run-level directional override. ET and CS stay excluded even when the
  // surrounding characters are not numeric; UAX #9 still decides their
  // resolved level from the authored context.
  if matches!(
    class,
    BidiClass::EN | BidiClass::AN | BidiClass::ET | BidiClass::CS
  ) {
    return true;
  }
  // Hebrew text additionally keeps an European Number Separator when it is
  // part of an European-number sequence. Arabic and other RTL runs do not
  // receive that exception.
  if class != BidiClass::ES
    || !style
      .language
      .as_deref()
      .is_some_and(drawingml_is_hebrew_language_tag)
  {
    return false;
  }
  let previous = index
    .checked_sub(1)
    .map(|previous| bidi_class(logical_chars[previous].1));
  let next = logical_chars
    .get(index + 1)
    .map(|&(_, next)| bidi_class(next));
  previous == Some(BidiClass::EN) && next == Some(BidiClass::EN)
}

fn drawingml_is_hebrew_language_tag(value: &str) -> bool {
  value.split(['-', '_']).next().is_some_and(|language| {
    language.eq_ignore_ascii_case("he") || language.eq_ignore_ascii_case("iw")
  })
}

fn push_bidi_text_line_segment<'a>(
  segments: &mut Vec<TextLineRun<'a>>,
  levels: &mut Vec<Level>,
  run: &TextLineRun<'a>,
  range: Range<usize>,
  level: Level,
  text_metrics: &mut TextMetrics,
) {
  let Some(text) = run.text.get(range) else {
    return;
  };
  let mut style = run.style.clone();
  style.resolved_bidi_level = Some(level.number());
  let width_pt = if run.kind == TextLineRunKind::Tab {
    run.width_pt
  } else {
    text_metrics.measure_text(text, &style)
  };
  segments.push(TextLineRun {
    run: run.run,
    text: text.to_string(),
    width_pt,
    style,
    kind: run.kind,
  });
  levels.push(level);
}

fn layout_text_lines_legacy<'a>(
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
      ) && (!run.text.is_empty() || presentation_field_may_generate_text(context.import, run))
    })
    .collect::<Vec<_>>();

  let mut tokens = Vec::new();
  for run in visible_runs {
    let style = styled_text_run(
      context.import,
      context.slide,
      context.base_style,
      context.options,
      run,
    );
    let field_text = presentation_field_text(
      run,
      context.slide_number,
      context.import.field_update_datetime,
      context.import.field_format_locale.as_deref(),
      style.language.as_deref(),
    );
    let uppercase_text;
    let text = if style.uppercase {
      uppercase_text = field_text.to_uppercase();
      uppercase_text.as_str()
    } else {
      field_text.as_ref()
    };
    for (line_text, has_hard_break) in drawingml_hard_lines(text) {
      for token in text_wrap_tokens(line_text) {
        tokens.push(LegacyTextToken {
          run,
          text: token.to_string(),
          style: style.clone(),
          kind: if token == "\t" {
            LegacyTextTokenKind::Tab
          } else {
            LegacyTextTokenKind::Text
          },
        });
      }
      if has_hard_break {
        tokens.push(LegacyTextToken {
          run,
          text: String::new(),
          style: style.clone(),
          kind: LegacyTextTokenKind::HardBreak,
        });
      }
    }
  }

  let mut lines = Vec::new();
  let mut current = TextLine::default();
  for (index, token) in tokens.iter().enumerate() {
    match token.kind {
      LegacyTextTokenKind::HardBreak => {
        trim_text_line_end(&mut current, text_metrics);
        lines.push(current);
        current = TextLine::default();
      }
      LegacyTextTokenKind::Tab => {
        let mut tab_stop = paragraph_tab_stop(current.width_pt, context);
        let mut aligned_width =
          tab_aligned_text_width(&tokens[index + 1..], tab_stop.alignment, text_metrics);
        let mut advance_pt = (tab_stop.position_pt - current.width_pt - aligned_width).max(0.0);
        if context.options.word_wrap
          && current.width_pt > f32::EPSILON
          && current.width_pt + advance_pt > context.column_width
        {
          trim_text_line_end(&mut current, text_metrics);
          lines.push(current);
          current = TextLine::default();
          tab_stop = paragraph_tab_stop(0.0, context);
          aligned_width =
            tab_aligned_text_width(&tokens[index + 1..], tab_stop.alignment, text_metrics);
          advance_pt = (tab_stop.position_pt - aligned_width).max(0.0);
        }
        push_text_line_tab(&mut current, token.run, advance_pt, &token.style);
      }
      LegacyTextTokenKind::Text => {
        let width_pt = text_metrics.measure_text(&token.text, &token.style);
        // The token's trailing whitespace is discarded if this becomes the
        // end of the line, so it must not force an otherwise fitting word to
        // the next line. Whitespace already accumulated after the previous
        // token remains part of current.width_pt and still separates words.
        let fit_width_pt = hanging_punctuation_fit_width(
          token.text.trim_end(),
          &token.style,
          context.hanging_punctuation,
          text_metrics,
        );
        if context.options.word_wrap
          && current.width_pt > f32::EPSILON
          && current.width_pt + fit_width_pt > context.column_width
        {
          trim_text_line_end(&mut current, text_metrics);
          lines.push(current);
          current = TextLine::default();
        }
        push_text_line_token(&mut current, token.run, &token.text, width_pt, &token.style);
      }
    }
  }
  trim_text_line_end(&mut current, text_metrics);
  lines.push(current);
  lines
}

fn drawingml_hard_lines(text: &str) -> Vec<(&str, bool)> {
  let mut lines = Vec::new();
  let mut start = 0usize;
  let mut characters = text.char_indices().peekable();
  while let Some((index, character)) = characters.next() {
    if !matches!(character, '\n' | '\r' | '\u{2028}' | '\u{2029}') {
      continue;
    }
    let mut end = index + character.len_utf8();
    if character == '\r'
      && let Some(&(next_index, '\n')) = characters.peek()
    {
      characters.next();
      end = next_index + '\n'.len_utf8();
    }
    lines.push((&text[start..index], true));
    start = end;
  }
  if start < text.len() || lines.is_empty() {
    lines.push((&text[start..], false));
  }
  lines
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LegacyTextTokenKind {
  Text,
  Tab,
  HardBreak,
}

struct LegacyTextToken<'a> {
  run: &'a TextRun,
  text: String,
  style: TextStyle,
  kind: LegacyTextTokenKind,
}

fn paragraph_tab_stop(
  current_width_pt: f32,
  context: TextLineLayoutContext<'_>,
) -> ParagraphTabStop {
  if let Some(stop) = context
    .tab_stops
    .iter()
    .find(|stop| stop.position_pt > current_width_pt + 0.01)
  {
    return *stop;
  }
  let interval = context.default_tab_size_pt.max(f32::EPSILON);
  ParagraphTabStop {
    position_pt: ((current_width_pt / interval).floor() + 1.0) * interval,
    alignment: a::TextTabAlignmentValues::Left,
  }
}

fn tab_aligned_text_width(
  tokens: &[LegacyTextToken<'_>],
  alignment: a::TextTabAlignmentValues,
  text_metrics: &mut TextMetrics,
) -> f32 {
  if alignment == a::TextTabAlignmentValues::Left {
    return 0.0;
  }
  let mut width_pt = 0.0;
  for token in tokens {
    if token.kind != LegacyTextTokenKind::Text {
      break;
    }
    let text = if alignment == a::TextTabAlignmentValues::Decimal {
      token
        .text
        .char_indices()
        .find_map(|(index, character)| matches!(character, '.' | ',').then_some(index))
        .map_or(token.text.as_str(), |index| &token.text[..index])
    } else {
      token.text.as_str()
    };
    width_pt += text_metrics.measure_text(text, &token.style);
    if alignment == a::TextTabAlignmentValues::Decimal && text.len() != token.text.len() {
      break;
    }
  }
  match alignment {
    a::TextTabAlignmentValues::Center => width_pt / 2.0,
    a::TextTabAlignmentValues::Right | a::TextTabAlignmentValues::Decimal => width_pt,
    a::TextTabAlignmentValues::Left => 0.0,
  }
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
    && last.kind == TextLineRunKind::Text
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
    kind: TextLineRunKind::Text,
  });
}

fn push_text_line_tab<'a>(
  line: &mut TextLine<'a>,
  run: &'a TextRun,
  width_pt: f32,
  style: &TextStyle,
) {
  line.width_pt += width_pt;
  line.runs.push(TextLineRun {
    run,
    text: "\t".to_string(),
    width_pt,
    style: style.clone(),
    kind: TextLineRunKind::Tab,
  });
}

fn text_wrap_tokens(text: &str) -> Vec<&str> {
  if text.is_empty() {
    return Vec::new();
  }
  let mut tokens = Vec::new();
  let mut start = 0usize;
  for (index, ch) in text.char_indices() {
    if ch == '\t' {
      if start < index {
        tokens.push(&text[start..index]);
      }
      let end = index + ch.len_utf8();
      tokens.push(&text[index..end]);
      start = end;
    } else if ch == '\u{200B}' || is_breakable_wrap_space(ch) {
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

fn is_breakable_wrap_space(character: char) -> bool {
  character.is_whitespace() && !matches!(character, '\u{00A0}' | '\u{202F}')
}

fn hanging_punctuation_fit_width(
  text: &str,
  style: &TextStyle,
  hanging_punctuation: bool,
  text_metrics: &mut TextMetrics,
) -> f32 {
  let width_pt = text_metrics.measure_text(text, style);
  if !hanging_punctuation {
    return width_pt;
  }
  let Some((index, character)) = text.char_indices().next_back() else {
    return width_pt;
  };
  if !is_hanging_punctuation(character) {
    return width_pt;
  }
  text_metrics.measure_text(&text[..index], style)
}

fn is_hanging_punctuation(character: char) -> bool {
  matches!(
    character,
    ','
      | '.'
      | '!'
      | '?'
      | ':'
      | ';'
      | '\u{3001}'
      | '\u{3002}'
      | '\u{FF0C}'
      | '\u{FF0E}'
      | '\u{FF01}'
      | '\u{FF1F}'
      | '\u{FF1A}'
      | '\u{FF1B}'
      | '\u{3009}'
      | '\u{300B}'
      | '\u{300D}'
      | '\u{300F}'
      | '\u{3011}'
      | '\u{3015}'
      | '\u{3017}'
      | '\u{3019}'
      | '\u{301B}'
      | '\u{2019}'
      | '\u{201D}'
      | ')'
      | ']'
      | '}'
  )
}

fn text_line_fit_width(
  line: &TextLine<'_>,
  hanging_punctuation: bool,
  text_metrics: &mut TextMetrics,
) -> f32 {
  if !hanging_punctuation {
    return line.width_pt;
  }
  let Some(run) = line.runs.iter().rev().find(|run| !run.text.is_empty()) else {
    return line.width_pt;
  };
  if run.kind != TextLineRunKind::Text {
    return line.width_pt;
  }
  let Some((index, character)) = run.text.char_indices().next_back() else {
    return line.width_pt;
  };
  if !is_hanging_punctuation(character) {
    return line.width_pt;
  }
  let prefix_width = text_metrics.measure_text(&run.text[..index], &run.style);
  line.width_pt - (run.width_pt - prefix_width)
}

fn trim_text_line_end(line: &mut TextLine<'_>, text_metrics: &mut TextMetrics) {
  // PowerPoint discards trailing-space advance but keeps the authored run
  // formatting in the line metrics. That distinction is visible when a
  // picture bullet is followed only by a formatted space: fdo90607's 32 pt
  // runs produce 38.4 pt line spacing in Office fixed output. Retaining the
  // now-empty run also lets mixed-size trailing spaces select the line height
  // without emitting a PDF text item.
  let mut run_index = line.runs.len();
  while run_index > 0 {
    run_index -= 1;
    let run = &mut line.runs[run_index];
    if run.kind == TextLineRunKind::Tab {
      break;
    }
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
  style.font_size_pt = scaled_text_font_size_pt(
    style.font_size_pt,
    options.font_scale,
    options.round_font_size_to_pt,
  );
  style.character_spacing_pt *= options.font_scale;
  style.baseline_shift_pt *= options.font_scale;
}

fn scaled_text_font_size_pt(font_size_pt: f32, font_scale: f32, round_to_pt: bool) -> f32 {
  let scaled = if round_to_pt {
    // setRoundFontSizeToPt(true) for AUTOFIT; editeng then rounds the
    // unscaled font size and the scaled font size to the nearest point.
    (font_size_pt.round() * font_scale).round()
  } else {
    font_size_pt * font_scale
  };
  // PowerPoint's PDF path lays out type on its 600 dpi print grid. Preserve
  // that device-space quantization before shaping: e.g. 40 pt becomes
  // 333/600 in and 20 pt becomes 167/600 in, matching the emitted Office PDF
  // text matrices without case-specific offsets.
  units::quantize_points_to_office_print_grid(scaled).max(MINIMUM_TEXT_FONT_SIZE_PT)
}

#[derive(Clone, Copy, Debug)]
enum ParagraphLineAdjustment {
  None,
  Word { extra_per_space: f32 },
  Grapheme { extra_per_gap: f32, count: usize },
}

fn paragraph_line_adjustment(
  alignment: a::TextAlignmentTypeValues,
  is_last_line: bool,
  line: &TextLine<'_>,
  available_width_pt: f32,
) -> ParagraphLineAdjustment {
  let distributed = matches!(
    alignment,
    a::TextAlignmentTypeValues::Distributed | a::TextAlignmentTypeValues::ThaiDistributed
  );
  let justified = matches!(
    alignment,
    a::TextAlignmentTypeValues::Justified | a::TextAlignmentTypeValues::JustifiedLow
  );
  if (!distributed && !justified)
    // PowerPoint keeps the final wrapped line of Thai-distributed text at
    // its natural width. Ordinary `dist` remains distinct: unlike
    // justification it can distribute a one-line paragraph.
    || (is_last_line
      && (justified || alignment == a::TextAlignmentTypeValues::ThaiDistributed))
    || line.runs.iter().any(|run| {
      run.kind == TextLineRunKind::Tab
        || run
          .style
          .resolved_bidi_level
          .is_some_and(|level| level % 2 == 1)
    })
  {
    return ParagraphLineAdjustment::None;
  }
  let extra_pt = available_width_pt - line.width_pt;
  if extra_pt <= 0.01 {
    return ParagraphLineAdjustment::None;
  }
  let space_count = line
    .runs
    .iter()
    .filter(|run| run.kind == TextLineRunKind::Text)
    .map(|run| run.text.matches(' ').count())
    .sum::<usize>();
  if space_count > 0 {
    return ParagraphLineAdjustment::Word {
      extra_per_space: extra_pt / space_count as f32,
    };
  }
  if !distributed {
    return ParagraphLineAdjustment::None;
  }
  let grapheme_count = line
    .runs
    .iter()
    .filter(|run| run.kind == TextLineRunKind::Text)
    .map(|run| {
      GraphemeClusterSegmenter::new()
        .segment_str(&run.text)
        .skip(1)
        .count()
    })
    .sum::<usize>();
  if grapheme_count < 2 {
    ParagraphLineAdjustment::None
  } else {
    ParagraphLineAdjustment::Grapheme {
      extra_per_gap: extra_pt / (grapheme_count - 1) as f32,
      count: grapheme_count,
    }
  }
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
  paragraph_bidi: bool,
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
    paint_clip: None,
    discard_if_horizontally_clipped: false,
    text,
    style: Box::new(style),
    rotation_center_pt: placement.rotation_center_pt,
    hyperlink_url,
    form_widget_id: None,
    paragraph_bidi: placement.paragraph_bidi,
    // DrawingML run boundaries are layout boundaries in PowerPoint's PDF
    // output, even when adjacent runs share formatting. Preserve them so the
    // PDF adapter does not reshape across rPr/field boundaries and introduce
    // cross-run kerning or cumulative positioning drift.
    preserve_text_portion,
    pdf_text_segmentation: PdfTextSegmentation::Line,
    source_path: Vec::new(),
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

fn push_word_spaced_text_items(
  items: &mut Vec<PageItem>,
  placement: TextItemPlacement,
  text: &str,
  style: &TextStyle,
  hyperlink_url: Option<String>,
  extra_per_space: f32,
  text_metrics: &mut TextMetrics,
) -> f32 {
  let mut x_pt = placement.x_pt;
  let mut start = 0usize;
  for (space, _) in text.match_indices(' ') {
    let end = space + 1;
    let segment = &text[start..end];
    push_symbol_split_text_items(
      items,
      TextItemPlacement { x_pt, ..placement },
      segment,
      style,
      hyperlink_url.clone(),
      text_metrics,
    );
    x_pt += text_metrics.measure_text(segment, style) + extra_per_space;
    start = end;
  }
  if start < text.len() {
    let segment = &text[start..];
    push_symbol_split_text_items(
      items,
      TextItemPlacement { x_pt, ..placement },
      segment,
      style,
      hyperlink_url,
      text_metrics,
    );
    x_pt += text_metrics.measure_text(segment, style);
  }
  x_pt
}

struct GraphemeDistribution<'a> {
  extra_per_gap: f32,
  remaining: &'a mut usize,
}

fn push_grapheme_distributed_text_items(
  items: &mut Vec<PageItem>,
  placement: TextItemPlacement,
  text: &str,
  style: &TextStyle,
  hyperlink_url: Option<String>,
  distribution: GraphemeDistribution<'_>,
  text_metrics: &mut TextMetrics,
) -> f32 {
  let mut x_pt = placement.x_pt;
  let mut start = 0usize;
  for end in GraphemeClusterSegmenter::new().segment_str(text).skip(1) {
    let segment = &text[start..end];
    push_symbol_split_text_items(
      items,
      TextItemPlacement { x_pt, ..placement },
      segment,
      style,
      hyperlink_url.clone(),
      text_metrics,
    );
    x_pt += text_metrics.measure_text(segment, style);
    *distribution.remaining = (*distribution.remaining).saturating_sub(1);
    if *distribution.remaining > 0 {
      x_pt += distribution.extra_per_gap;
    }
    start = end;
  }
  x_pt
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
  segment_style.explicit_symbol_character = segment.use_symbol_font;
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
  line_height_pt: f32,
  natural_text_height_pt: f32,
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
    // LibreOffice exposes PowerPoint's two-height rule in
    // Outliner::ImpCalcBulletArea: GetHeight() is the resolved paragraph line
    // height while GetTxtHeight() retains the physical font height. Preserve
    // that distinction with the resolved paragraph advance and the selected
    // face's ascent/descent; a proportional line spacing such as
    // tdf114913's inherited 90% changes only the former.
    y_pt: picture_bullet_y_pt(y_pt, line_height_pt, natural_text_height_pt, height_pt),
    width_pt,
    height_pt,
    crop: ImageCrop::default(),
    clip_path: Vec::new(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    data: resource.data.clone(),
    content_type: resource.content_type.clone(),
    metafile_monochrome_dib_palette_override: resource.monochrome_dib_palette_override,
    metafile_background_color: None,
    metafile_external_header: None,
    metafile_semantic_text_includes_raster_backdrop: false,
    alt_text: None,
    hyperlink_url: shape_hyperlink_url.map(ToString::to_string),
    floating: false,
    behind_text: false,
  })
}

fn picture_bullet_y_pt(
  line_box_top_pt: f32,
  line_height_pt: f32,
  natural_text_height_pt: f32,
  image_height_pt: f32,
) -> f32 {
  line_box_top_pt + line_height_pt - (natural_text_height_pt + image_height_pt) / 2.0
}

fn picture_bullet_x_pt(
  bullet_x_pt: f32,
  image_width_pt: f32,
  visible_line_width_pt: f32,
  alignment: a::TextAlignmentTypeValues,
) -> f32 {
  // When a centered paragraph contains only non-printing text, the picture
  // bullet is the line's sole visible content. PowerPoint therefore centers
  // the image itself instead of placing its left edge at the centered text
  // origin. The Office PDF for fdo90607 exposes this directly: the picture
  // transform is centered on the slide while each authored 32 pt space has no
  // printed advance.
  if alignment == a::TextAlignmentTypeValues::Center && visible_line_width_pt <= f32::EPSILON {
    bullet_x_pt - image_width_pt / 2.0
  } else {
    bullet_x_pt
  }
}

fn paragraph_bullet_x(
  text_x_pt: f32,
  text_width_pt: f32,
  indent_pt: f32,
  bullet_width_pt: f32,
  right_to_left: bool,
) -> f32 {
  if right_to_left {
    // DrawingML marL/indent form a logical leading margin in an RTL
    // paragraph. A -36 pt hanging indent therefore reserves 36 pt to the
    // right of the text; place the bullet against that outer edge.
    text_x_pt + text_width_pt - indent_pt - bullet_width_pt
  } else {
    text_x_pt + indent_pt
  }
}

fn line_height(style: &TextStyle, line_scale: f32) -> f32 {
  style.font_size_pt * DEFAULT_TEXT_LINE_HEIGHT_SCALE * line_scale
}

fn automatic_soft_break_empty_line_height(style: &TextStyle, line_scale: f32) -> f32 {
  style.font_size_pt * line_scale
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
          east_asian_line_break: paragraph_style.east_asian_line_break,
          latin_line_break: paragraph_style.latin_line_break,
          default_tab_size_pt: paragraph_style.default_tab_size_pt,
          tab_stops: &paragraph_style.tab_stops,
          hanging_punctuation: paragraph_style.hanging_punctuation,
        },
        runs,
        text_metrics,
      );
      for line in lines {
        let line_height = if line.runs.is_empty() {
          paragraph_style.line_height(&paragraph_base_style, context.options)
        } else {
          line.runs.iter().fold(0.0_f32, |height, run| {
            height.max(paragraph_style.line_height(&run.style, context.options))
          })
        };
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

fn presentation_field_text<'a>(
  run: &'a TextRun,
  slide_number: i32,
  field_update_datetime: Option<FieldUpdateDateTime>,
  field_format_locale: Option<&str>,
  language: Option<&str>,
) -> Cow<'a, str> {
  if run.kind != TextRunKind::Field {
    return Cow::Borrowed(&run.text);
  }
  let Some(field_type) = run.field_type.as_deref() else {
    return Cow::Borrowed(&run.text);
  };
  if field_type.eq_ignore_ascii_case("slidenum") {
    return Cow::Owned(slide_number.to_string());
  }
  let Some(value) = field_update_datetime else {
    return Cow::Borrowed(&run.text);
  };
  let language = language.or(field_format_locale);
  format_presentation_date_time_field(field_type, language, value)
    .map(Cow::<str>::Owned)
    .unwrap_or_else(|| Cow::Borrowed(run.text.as_str()))
}

fn presentation_field_may_generate_text(import: &PowerPointImport, run: &TextRun) -> bool {
  if run.kind != TextRunKind::Field {
    return false;
  }
  run.field_type.as_deref().is_some_and(|field_type| {
    field_type.eq_ignore_ascii_case("slidenum")
      || (import.field_update_datetime.is_some()
        && presentation_date_time_field_is_reserved(field_type))
  })
}

fn presentation_date_time_field_is_reserved(field_type: &str) -> bool {
  matches!(
    field_type.to_ascii_lowercase().as_str(),
    "datetime"
      | "datetime1"
      | "datetime2"
      | "datetime3"
      | "datetime4"
      | "datetime5"
      | "datetime6"
      | "datetime7"
      | "datetime8"
      | "datetime9"
      | "datetime10"
      | "datetime11"
      | "datetime12"
      | "datetime13"
  )
}

fn format_presentation_date_time_field(
  field_type: &str,
  language: Option<&str>,
  value: FieldUpdateDateTime,
) -> Option<String> {
  // ISO/IEC 29500-1 §20.1.5.2 reserves datetime and datetime1..13 for
  // periodically refreshed DrawingML fields. PowerPoint takes the formatting
  // locale from a:fld/a:rPr/@lang (numfmt.pptx demonstrates en-US and en-IN in
  // one package). The reserved type selects an Office semantic shape; ICU owns
  // locale order, names, digits, and day periods. LibreOffice maps datetime3
  // and datetime4 to the same locale-long date shape, which also matches the
  // en-IN cached counterexample where datetime4 is day-month-year.
  match field_type.to_ascii_lowercase().as_str() {
    "datetime" | "datetime1" => field_datetime::format_office_short_date(language, value),
    "datetime2" => field_datetime::format_office_long_date(language, value, true),
    "datetime3" | "datetime4" => field_datetime::format_office_long_date(language, value, false),
    "datetime5" => field_datetime::format_date_time_picture("dd-MMM-yy", language, value),
    "datetime6" => field_datetime::format_date_time_picture("MMMM yy", language, value),
    "datetime7" => field_datetime::format_date_time_picture("MMM-yy", language, value),
    "datetime8" | "datetime9" => {
      let time_picture = if field_type.eq_ignore_ascii_case("datetime8") {
        "h:mm am/pm"
      } else {
        "h:mm:ss am/pm"
      };
      Some(format!(
        "{} {}",
        field_datetime::format_office_short_date(language, value)?,
        field_datetime::format_date_time_picture(time_picture, language, value)?
      ))
    }
    "datetime10" => field_datetime::format_date_time_picture("H:mm", language, value),
    "datetime11" => field_datetime::format_date_time_picture("H:mm:ss", language, value),
    "datetime12" => field_datetime::format_date_time_picture("h:mm am/pm", language, value),
    "datetime13" => field_datetime::format_date_time_picture("h:mm:ss am/pm", language, value),
    _ => None,
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
    metafile_monochrome_dib_palette_override: None,
    metafile_background_color: None,
    metafile_external_header: None,
    metafile_semantic_text_includes_raster_backdrop: false,
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
  right_margin_pt: f32,
  indent_pt: f32,
  alignment: a::TextAlignmentTypeValues,
  right_to_left: bool,
  east_asian_line_break: bool,
  latin_line_break: bool,
  default_tab_size_pt: f32,
  tab_stops: Vec<ParagraphTabStop>,
  hanging_punctuation: bool,
  font_alignment: a::TextFontAlignmentValues,
  line_spacing: ParagraphLineSpacing,
  space_before: ParagraphSpacing,
  space_after: ParagraphSpacing,
  bullet: BulletDisplay,
  master_default_run_properties: Option<Box<a::DefaultRunProperties>>,
  text_default_run_properties: Option<Box<a::DefaultRunProperties>>,
  direct_default_run_properties: Option<Box<a::DefaultRunProperties>>,
}

#[derive(Clone, Copy, Debug)]
struct ParagraphTabStop {
  position_pt: f32,
  alignment: a::TextTabAlignmentValues,
}

#[derive(Clone, Debug, Default)]
struct BulletDisplay {
  label: Option<String>,
  auto_number: Option<AutoNumberBullet>,
  font: Option<String>,
  font_follows_text: bool,
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
      right_margin_pt: 0.0,
      indent_pt: 0.0,
      alignment: a::TextAlignmentTypeValues::Left,
      right_to_left: false,
      // MS-OI29500 §21.1.2.4.13 records the actual Office defaults: East
      // Asian kinsoku is enabled and emergency Latin word breaking is not.
      east_asian_line_break: true,
      latin_line_break: false,
      // MS-OI29500 §21.1.2.4.13: Office uses 914400 EMU (72 pt) when
      // defTabSz is omitted.
      default_tab_size_pt: 72.0,
      tab_stops: Vec::new(),
      hanging_punctuation: true,
      // ECMA-376 Part 1 §21.1.2.2.7: omitted fontAlgn implies base.
      font_alignment: a::TextFontAlignmentValues::Baseline,
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

fn paragraph_tab_stops(source: &a::TabStopList) -> Vec<ParagraphTabStop> {
  let mut stops = source
    .tab_stop
    .iter()
    .filter_map(|stop| {
      stop.position.map(|position| ParagraphTabStop {
        position_pt: units::emu_to_points(position.to_emu()),
        alignment: stop.alignment.unwrap_or(a::TextTabAlignmentValues::Left),
      })
    })
    .collect::<Vec<_>>();
  stops.sort_by(|left, right| left.position_pt.total_cmp(&right.position_pt));
  stops
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
      if let Some(right_margin) = properties.right_margin {
        style.right_margin_pt = units::emu_to_points(i64::from(right_margin));
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
      if let Some(right_to_left) = properties.right_to_left.as_ref() {
        style.right_to_left = right_to_left.as_bool();
      }
      if let Some(east_asian_line_break) = properties.east_asian_line_break.as_ref() {
        style.east_asian_line_break = east_asian_line_break.as_bool();
      }
      if let Some(latin_line_break) = properties.latin_line_break.as_ref() {
        style.latin_line_break = latin_line_break.as_bool();
      }
      if let Some(default_tab_size) = properties.default_tab_size {
        style.default_tab_size_pt =
          units::emu_to_points(default_tab_size.to_emu()).max(f32::EPSILON);
      }
      if let Some(tab_stop_list) = properties.tab_stop_list.as_ref() {
        style.tab_stops = paragraph_tab_stops(tab_stop_list);
      }
      if let Some(hanging_punctuation) = properties.height.as_ref() {
        style.hanging_punctuation = hanging_punctuation.as_bool();
      }
      if let Some(font_alignment) = properties.font_alignment {
        style.font_alignment = font_alignment;
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
        self.right_margin_pt = properties
          .right_margin
          .map(|value| units::emu_to_points(i64::from(value)))
          .unwrap_or(self.right_margin_pt);
        self.indent_pt = properties
          .indent
          .map(|value| units::emu_to_points(i64::from(value)))
          .unwrap_or(self.indent_pt);
        self.alignment = properties.alignment.unwrap_or(self.alignment);
        if let Some(right_to_left) = properties.right_to_left.as_ref() {
          self.right_to_left = right_to_left.as_bool();
        }
        if let Some(east_asian_line_break) = properties.east_asian_line_break.as_ref() {
          self.east_asian_line_break = east_asian_line_break.as_bool();
        }
        if let Some(latin_line_break) = properties.latin_line_break.as_ref() {
          self.latin_line_break = latin_line_break.as_bool();
        }
        if let Some(default_tab_size) = properties.default_tab_size {
          self.default_tab_size_pt =
            units::emu_to_points(default_tab_size.to_emu()).max(f32::EPSILON);
        }
        if let Some(tab_stop_list) = properties.tab_stop_list.as_ref() {
          self.tab_stops = paragraph_tab_stops(tab_stop_list);
        }
        if let Some(hanging_punctuation) = properties.height.as_ref() {
          self.hanging_punctuation = hanging_punctuation.as_bool();
        }
        self.font_alignment = properties.font_alignment.unwrap_or(self.font_alignment);
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
        self.right_margin_pt = $properties
          .right_margin
          .map(|value| units::emu_to_points(i64::from(value)))
          .unwrap_or(self.right_margin_pt);
        self.indent_pt = $properties
          .indent
          .map(|value| units::emu_to_points(i64::from(value)))
          .unwrap_or(self.indent_pt);
        self.alignment = $properties.alignment.unwrap_or(self.alignment);
        if let Some(right_to_left) = $properties.right_to_left.as_ref() {
          self.right_to_left = right_to_left.as_bool();
        }
        if let Some(east_asian_line_break) = $properties.east_asian_line_break.as_ref() {
          self.east_asian_line_break = east_asian_line_break.as_bool();
        }
        if let Some(latin_line_break) = $properties.latin_line_break.as_ref() {
          self.latin_line_break = latin_line_break.as_bool();
        }
        if let Some(default_tab_size) = $properties.default_tab_size {
          self.default_tab_size_pt =
            units::emu_to_points(default_tab_size.to_emu()).max(f32::EPSILON);
        }
        if let Some(tab_stop_list) = $properties.tab_stop_list.as_ref() {
          self.tab_stops = paragraph_tab_stops(tab_stop_list);
        }
        if let Some(hanging_punctuation) = $properties.height.as_ref() {
          self.hanging_punctuation = hanging_punctuation.as_bool();
        }
        self.font_alignment = $properties.font_alignment.unwrap_or(self.font_alignment);
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

  fn left_offset(&self, has_bullet: bool) -> f32 {
    self.left_margin_pt + if has_bullet { 0.0 } else { self.indent_pt }
  }

  fn apply_diagram_autofit_spacing_scale(
    &mut self,
    paragraph: &TextParagraph,
    options: &TextLoweringOptions,
  ) {
    if paragraph.diagram_synthesized_bullet_left_margin
      || paragraph.diagram_synthesized_bullet_indent
    {
      let scale = if options.round_font_size_to_pt {
        options
          .font_scale
          .max(POWERPOINT_SMARTART_MINIMUM_AUTOFIT_SPACING_SCALE)
      } else {
        POWERPOINT_SMARTART_EXPLICIT_FONT_SPACING_SCALE
      };
      if paragraph.diagram_synthesized_bullet_left_margin {
        self.left_margin_pt *= scale;
      }
      if paragraph.diagram_synthesized_bullet_indent {
        self.indent_pt *= scale;
      }
    }
  }

  fn available_width(&self, column_width: f32, left_offset: f32) -> f32 {
    (column_width - left_offset - self.right_margin_pt).max(0.0)
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

  fn soft_break_empty_line_height(&self, style: &TextStyle, options: &TextLoweringOptions) -> f32 {
    match self.line_spacing {
      // PowerPoint's fixed-output path advances an empty line introduced by
      // a:br by one em under automatic spacing. The br/rPr is insertion-point
      // formatting (ECMA-376 Part 1, 21.1.2.2.1), not the current empty line's
      // layout style. Explicit paragraph line spacing remains authoritative.
      ParagraphLineSpacing::Default => {
        automatic_soft_break_empty_line_height(style, options.line_scale)
      }
      _ => self.line_height(style, options),
    }
  }

  fn line_height_with_scale(&self, style: &TextStyle, line_scale: f32) -> f32 {
    let natural_height = line_height(style, 1.0);
    match self.line_spacing {
      ParagraphLineSpacing::Default => line_height(style, line_scale),
      ParagraphLineSpacing::Percent(ratio) => natural_height * ratio * line_scale,
      ParagraphLineSpacing::Points(points) => points * line_scale,
    }
  }

  fn baseline_offset(
    &self,
    style: &TextStyle,
    line_height_pt: f32,
    text_metrics: &mut TextMetrics,
  ) -> f32 {
    let offset = raw_baseline_offset(style, line_height_pt, text_metrics);
    match self.line_spacing {
      // LibreOffice's EditEngine mirrors the Windows/PPT proportional-line
      // path by capping ascent at 80% of the reduced text height. Without
      // this cap an OS/2 usWinAscent can exceed a sub-100% DrawingML line
      // box and place the glyph baseline outside that box.
      // See editeng/source/editeng/impedit3.cxx (ImpBreakLine formatting).
      ParagraphLineSpacing::Percent(ratio) if ratio < 1.0 => offset.min(line_height_pt * 0.8),
      ParagraphLineSpacing::Default
      | ParagraphLineSpacing::Percent(_)
      | ParagraphLineSpacing::Points(_) => offset,
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

fn raw_baseline_offset(
  style: &TextStyle,
  line_height_pt: f32,
  text_metrics: &mut TextMetrics,
) -> f32 {
  if style.use_windows_font_metrics {
    text_metrics.baseline_offset_in_line_with_windows_metrics(style, line_height_pt)
  } else {
    text_metrics.baseline_offset_in_line(style, line_height_pt)
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
      BulletFontOverride::FollowText => {
        self.font = None;
        self.font_follows_text = true;
      }
      BulletFontOverride::Font(font) => {
        self.font = Some(font);
        self.font_follows_text = false;
      }
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
  paragraph_style: &ParagraphDisplayStyle,
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
    left_margin_100mm: points_to_100mm(paragraph_style.left_margin_pt),
    indent_100mm: points_to_100mm(paragraph_style.indent_pt),
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
  apply_drawingml_run_properties(import, slide, properties, style);
}

fn apply_drawingml_run_properties(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  properties: &a::RunProperties,
  style: &mut TextStyle,
) {
  apply_run_common(
    import,
    RunCommon {
      language: properties.language.as_deref(),
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
  if let Some(right_to_left) = properties.right_to_left.as_ref() {
    style.right_to_left = Some(
      right_to_left
        .val
        .as_ref()
        .is_none_or(|value| value.as_bool()),
    );
  }
  if let Some(fill) = properties.run_properties_choice1.as_ref() {
    apply_text_fill(import, slide, fill, style);
  }
  if let Some(outline) = properties.outline.as_deref() {
    apply_text_outline(import, slide, outline, style);
    if properties.run_properties_choice2.is_none() && style.outline_width_pt > f32::EPSILON {
      ensure_searchable_glyph_outlines(style);
    }
  }
  if let Some(effect) = properties.run_properties_choice2.as_ref() {
    style.drawingml_text_effects = Some(drawingml_run_effects(import, slide, effect));
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
      language: properties.language.as_deref(),
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
  if let Some(right_to_left) = properties.right_to_left.as_ref() {
    style.right_to_left = Some(
      right_to_left
        .val
        .as_ref()
        .is_none_or(|value| value.as_bool()),
    );
  }
  if let Some(fill) = properties.default_run_properties_choice1.as_ref() {
    apply_default_text_fill(import, slide, fill, style);
  }
  if let Some(outline) = properties.outline.as_deref() {
    apply_text_outline(import, slide, outline, style);
    if properties.default_run_properties_choice2.is_none() && style.outline_width_pt > f32::EPSILON
    {
      ensure_searchable_glyph_outlines(style);
    }
  }
  if let Some(effect) = properties.default_run_properties_choice2.as_ref() {
    style.drawingml_text_effects = Some(drawingml_default_run_effects(import, slide, effect));
  }
  if let Some(fill) = properties.default_run_properties_choice4.as_ref() {
    apply_default_run_underline_fill(import, slide, fill, style);
  }
  if let Some(highlight) = properties.highlight.as_deref() {
    apply_text_highlight(import, slide, highlight, style);
  }
}

fn drawingml_run_effects(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  effect: &a::RunPropertiesChoice2,
) -> common::drawingml_image_effects::ImageEffectContainer {
  let resolver = PptxImageEffectColorResolver { import, slide };
  match effect {
    a::RunPropertiesChoice2::EffectList(list) => {
      common::drawingml_image_effects::from_effect_list(list, None, &resolver)
    }
    a::RunPropertiesChoice2::EffectDag(dag) => {
      common::drawingml_image_effects::from_effect_dag(dag, None, &resolver)
    }
  }
}

fn drawingml_default_run_effects(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  effect: &a::DefaultRunPropertiesChoice2,
) -> common::drawingml_image_effects::ImageEffectContainer {
  let resolver = PptxImageEffectColorResolver { import, slide };
  match effect {
    a::DefaultRunPropertiesChoice2::EffectList(list) => {
      common::drawingml_image_effects::from_effect_list(list, None, &resolver)
    }
    a::DefaultRunPropertiesChoice2::EffectDag(dag) => {
      common::drawingml_image_effects::from_effect_dag(dag, None, &resolver)
    }
  }
}

struct RunCommon<'a> {
  language: Option<&'a str>,
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
  if let Some(language) = properties.language {
    style.language = Some(Arc::from(language));
  }
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
  // A direct character fill replaces the inherited fill as one property.
  // In particular, a solid run fill must discard a gradient captured from
  // the master style instead of leaving that gradient attached to the glyph
  // outline renderer.
  clear_searchable_glyph_fill(style);
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
      if let Some(fill) = drawingml_text_gradient_fill(import, slide, fill) {
        set_searchable_glyph_fill(style, fill);
      }
    }
    a::RunPropertiesChoice::PatternFill(fill) => {
      set_searchable_glyph_fill(
        style,
        common::Fill::Pattern(pattern_fill_for_optional_slide(import, slide, fill, None)),
      );
    }
    a::RunPropertiesChoice::BlipFill(_) => {}
    // MS-OI29500 §20.1.8.35: Office treats grpFill in rPr as noFill.
    a::RunPropertiesChoice::GroupFill => {
      style.color = RgbColor { r: 0, g: 0, b: 0 };
      style.opacity = 0.0;
    }
  }
}

fn apply_default_text_fill(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  fill: &a::DefaultRunPropertiesChoice,
  style: &mut TextStyle,
) {
  clear_searchable_glyph_fill(style);
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
      if let Some(fill) = drawingml_text_gradient_fill(import, slide, fill) {
        set_searchable_glyph_fill(style, fill);
      }
    }
    a::DefaultRunPropertiesChoice::PatternFill(fill) => {
      set_searchable_glyph_fill(
        style,
        common::Fill::Pattern(pattern_fill_for_optional_slide(import, slide, fill, None)),
      );
    }
    a::DefaultRunPropertiesChoice::BlipFill(_) => {}
    // MS-OI29500 §20.1.8.35: Office treats grpFill in defRPr as noFill.
    a::DefaultRunPropertiesChoice::GroupFill => {
      style.color = RgbColor { r: 0, g: 0, b: 0 };
      style.opacity = 0.0;
    }
  }
}

fn drawingml_text_gradient_fill(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  source: &a::GradientFill,
) -> Option<common::Fill<'static>> {
  let mut fill = gradient_fill_for_optional_slide(
    import,
    slide,
    source,
    shared_diagram::DiagramBounds {
      x: 0.0,
      y: 0.0,
      width: 1.0,
      height: 1.0,
    },
  )?;
  if let common::Fill::Gradient(gradient) = &mut fill {
    if let Some(path) = &mut gradient.path
      && path.kind == common::GradientPathKind::Circle
      && let Some(a::GradientFillChoice::PathGradientFill(source_path)) =
        source.gradient_fill_choice.as_ref()
    {
      *path = common::drawingml_gradient::resolve_path_gradient(
        source,
        source_path,
        common::Transform::default(),
      );
    }
    let constant_path_color = gradient.path.is_some()
      && gradient.stops.first().is_some_and(|first| {
        gradient.stops.iter().all(|stop| {
          (stop.color.r, stop.color.g, stop.color.b)
            == (first.color.r, first.color.g, first.color.b)
        })
      });
    if constant_path_color {
      // Office's fixed-output writer collapses a same-RGB DrawingML text path
      // gradient to its final stop, including that stop's alpha. The
      // tdf139618 Office PDF stores both WordArt lines as constant #4472c4
      // RGB images with no soft mask. This is intentionally text/path scoped:
      // Office preserves the alpha ramp for an equivalent linear shape fill
      // (gradient-multistep-transparency.pptx), and for multicolor text path
      // gradients.
      return gradient
        .stops
        .last()
        .map(|stop| common::Fill::Solid(stop.color));
    }
    if gradient.path.is_none() {
      // PowerPoint's fixed-output writer applies the GDI+ linear-gradient
      // sigma blend to DrawingML character paint. The sampled function in the
      // tdf139618 Office PDF matches SetSigmaBellShape(1, 1), including
      // gamma-correct color interpolation, rather than linear sRGB.
      gradient.interpolation = common::GradientInterpolation::PowerPointGammaSigma;
    }
    // Character paint is sized only after line breaking. Leave the brush in
    // normalized coordinates; the PDF text renderer binds it to each
    // resulting text portion.
    gradient.definition_bounds = None;
  }
  Some(fill)
}

fn ensure_searchable_glyph_outlines(style: &mut TextStyle) {
  style.pdf_glyph_outlines = true;
  let preserve_semantic_overlay = style
    .pdf_glyph_outline_options
    .as_ref()
    .is_none_or(|options| options.semantic_text_overlay);
  let mut options = style
    .pdf_glyph_outline_options
    .as_deref()
    .cloned()
    .unwrap_or_default();
  options.semantic_text_overlay = preserve_semantic_overlay;
  style.pdf_glyph_outline_options = Some(Arc::new(options));
}

fn clear_searchable_glyph_fill(style: &mut TextStyle) {
  let Some(current) = style.pdf_glyph_outline_options.as_deref() else {
    return;
  };
  let mut options = current.clone();
  if options.fill.take().is_none() {
    return;
  }
  let fill_was_only_outline_reason = style.pdf_glyph_outlines
    && options.semantic_text_overlay
    && options.outline_fill.is_none()
    && options.outline_stroke.is_none()
    && options.transform.is_none()
    && options.text_warp.is_none()
    && style.outline_width_pt <= f32::EPSILON;
  if fill_was_only_outline_reason {
    style.pdf_glyph_outlines = false;
    style.pdf_glyph_outline_options = None;
  } else {
    style.pdf_glyph_outline_options = Some(Arc::new(options));
  }
}

fn set_searchable_glyph_fill(style: &mut TextStyle, fill: common::Fill<'static>) {
  ensure_searchable_glyph_outlines(style);
  style.opacity = 1.0;
  let mut options = style
    .pdf_glyph_outline_options
    .as_deref()
    .cloned()
    .unwrap_or_default();
  options.fill = Some(fill);
  style.pdf_glyph_outline_options = Some(Arc::new(options));
}

fn apply_text_outline(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  outline: &a::Outline,
  style: &mut TextStyle,
) {
  // ECMA-376 Part 1, 20.1.2.2.24 defines a:ln as the outline style for both
  // shapes and text. Character outlines participate in the same run-property
  // inheritance as the adjacent fill: an omitted a:ln keeps the inherited
  // value, while an explicit noFill removes it.
  let Some(line) = LineProperties::from_dml_outline(outline) else {
    return;
  };
  if matches!(line.fill, LineFill::None) {
    style.outline_color = None;
    style.outline_opacity = 1.0;
    style.outline_width_pt = 0.0;
    if let Some(current) = style.pdf_glyph_outline_options.as_deref() {
      let mut options = current.clone();
      options.outline_fill = None;
      options.outline_stroke = None;
      style.pdf_glyph_outline_options = Some(Arc::new(options));
    }
    return;
  }
  // Unlike a shape outline, the DrawingML line definition specifies zero as
  // the omitted text-outline width. Do not inherit the shape renderer's
  // 0.75pt convenience default here.
  style.outline_width_pt = outline
    .width
    .map(|width| units::emu_to_points(i64::from(width)))
    .unwrap_or(0.0);
  let vector_outline_fill = match &line.fill {
    LineFill::Gradient(fill) => drawingml_text_gradient_fill(import, slide, fill),
    LineFill::Pattern(fill) => Some(common::Fill::Pattern(pattern_fill_for_optional_slide(
      import, slide, fill, None,
    ))),
    LineFill::Unspecified | LineFill::None | LineFill::Solid(_) => None,
  };
  // A width-only line changes the inherited outline width without replacing
  // its inherited paint. A solid fill replaces both color and opacity.
  if let Some(stroke) = line_stroke(import, slide, &line) {
    style.outline_color = Some(stroke.style.color);
    style.outline_opacity = stroke.opacity;
    ensure_searchable_glyph_outlines(style);
    let mut options = style
      .pdf_glyph_outline_options
      .as_deref()
      .cloned()
      .unwrap_or_default();
    let mut common_stroke = stroke.common;
    if let Some(fill) = vector_outline_fill {
      options.outline_fill = Some(fill.clone());
      match fill {
        common::Fill::Gradient(gradient) => common_stroke.gradient = Some(gradient),
        common::Fill::Pattern(pattern) => common_stroke.pattern = Some(pattern),
        common::Fill::Solid(color) => common_stroke.color = color,
        common::Fill::None | common::Fill::Theme(_) | common::Fill::Image { .. } => {}
      }
    } else {
      options.outline_fill = None;
    }
    options.outline_stroke = Some(common_stroke);
    style.pdf_glyph_outline_options = Some(Arc::new(options));
  } else if let Some(current) = style.pdf_glyph_outline_options.as_deref()
    && let Some(mut inherited_stroke) = current.outline_stroke.clone()
  {
    inherited_stroke.width = common::Pt(style.outline_width_pt);
    let mut options = current.clone();
    options.outline_stroke = Some(inherited_stroke);
    style.pdf_glyph_outline_options = Some(Arc::new(options));
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

#[derive(Clone, Debug)]
struct DisplayStroke {
  style: BorderStyle,
  opacity: f32,
  common: common::Stroke<'static>,
}

fn shape_common_fill(
  import: &PowerPointImport,
  slide: &SlidePersist,
  fill: &FillProperties,
) -> Option<common::Fill<'static>> {
  if !matches!(fill.kind, FillKind::SlideBackground) {
    return common_fill_for_optional_slide(import, Some(slide), fill);
  }

  let Some(background) = resolved_slide_background_fill(import, slide) else {
    let paint = default_page_background_paint();
    return Some(common::Fill::Solid(common_rgb(paint.color, paint.opacity)));
  };
  match background.kind {
    FillKind::None => {
      let paint = default_page_background_paint();
      Some(common::Fill::Solid(common_rgb(paint.color, paint.opacity)))
    }
    _ => common_fill_for_optional_slide(import, Some(slide), &background),
  }
}

fn common_fill_for_optional_slide(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  fill: &FillProperties,
) -> Option<common::Fill<'static>> {
  match &fill.kind {
    FillKind::Solid(color) => color
      .as_ref()
      .and_then(|color| {
        display_paint_for_optional_slide(import, slide, color, fill.placeholder_color.as_ref())
      })
      .map(|paint| common::Fill::Solid(common_rgb(paint.color, paint.opacity))),
    FillKind::Pattern(pattern) => Some(common::Fill::Pattern(pattern_fill_for_optional_slide(
      import,
      slide,
      pattern,
      fill.placeholder_color.as_ref(),
    ))),
    FillKind::None
    | FillKind::SlideBackground
    | FillKind::Group
    | FillKind::Gradient(_)
    | FillKind::Blip(_) => None,
  }
}

fn pattern_fill_for_optional_slide(
  import: &PowerPointImport,
  slide: Option<&SlidePersist>,
  fill: &a::PatternFill,
  placeholder_color: Option<&Color>,
) -> common::PatternFill {
  let foreground = fill
    .foreground_color
    .as_ref()
    .and_then(|color| color.foreground_color_choice.as_ref())
    .and_then(Color::from_foreground_color_choice)
    .and_then(|color| display_paint_for_optional_slide(import, slide, &color, placeholder_color))
    .map(|paint| common_rgb(paint.color, paint.opacity))
    .unwrap_or(common::Color {
      r: 0,
      g: 0,
      b: 0,
      a: u8::MAX,
    });
  let background = fill
    .background_color
    .as_ref()
    .and_then(|color| color.background_color_choice.as_ref())
    .and_then(Color::from_background_color_choice)
    .and_then(|color| display_paint_for_optional_slide(import, slide, &color, placeholder_color))
    .map(|paint| common_rgb(paint.color, paint.opacity))
    .unwrap_or(common::Color {
      r: u8::MAX,
      g: u8::MAX,
      b: u8::MAX,
      a: u8::MAX,
    });
  common::PatternFill::drawingml(
    common::drawingml_pattern::hatch_style(fill.preset),
    foreground,
    background,
  )
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
  let (paint, pattern) = match &line.fill {
    LineFill::Solid(color) => (
      color.as_ref().and_then(|color| {
        display_paint_for_optional_slide(import, slide, color, line.placeholder_color.as_ref())
      })?,
      None,
    ),
    LineFill::Pattern(fill) => {
      let pattern =
        pattern_fill_for_optional_slide(import, slide, fill, line.placeholder_color.as_ref());
      (
        DisplayPaint {
          color: RgbColor {
            r: pattern.foreground.r,
            g: pattern.foreground.g,
            b: pattern.foreground.b,
          },
          opacity: opacity_from_common_color(pattern.foreground),
        },
        Some(pattern),
      )
    }
    LineFill::Gradient(fill) => {
      let paint = fill
        .gradient_stop_list
        .as_ref()
        .and_then(|list| list.gradient_stop.first())
        .and_then(|stop| stop.gradient_stop_choice.as_ref())
        .and_then(Color::from_gradient_stop_choice)
        .and_then(|color| {
          display_paint_for_optional_slide(import, slide, &color, line.placeholder_color.as_ref())
        })?;
      (paint, None)
    }
    LineFill::Unspecified | LineFill::None => return None,
  };
  let width_pt = line.width_emu.map(units::emu_to_points).unwrap_or(0.75);
  let mut common = common::Stroke {
    width: common::Pt(width_pt),
    color: common_rgb(paint.color, paint.opacity),
    pattern,
    ..Default::default()
  };
  if let Some(outline) = line.source_outline.as_deref() {
    common::drawingml_stroke::apply_outline_style(&mut common, outline);
  }
  Some(DisplayStroke {
    style: BorderStyle {
      width_pt,
      spacing_pt: 0.0,
      color: paint.color,
      compound: false,
      dash_pattern: crate::model::BorderDashPattern::Solid,
      shadow: false,
    },
    opacity: paint.opacity,
    common,
  })
}

fn opacity_from_common_color(color: common::Color) -> f32 {
  f32::from(color.a) / f32::from(u8::MAX)
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
  fn metafile_images_enable_a_searchable_text_overlay() {
    assert!(supports_semantic_metafile_text(Some("image/x-emf")));
    assert!(supports_semantic_metafile_text(Some("IMAGE/WMF")));
    assert!(!supports_semantic_metafile_text(Some("image/png")));
    assert!(!supports_semantic_metafile_text(None));
  }

  #[test]
  fn group_child_coordinates_apply_child_extents_scaling() {
    let shape = Shape {
      position: crate::pptx::drawingml::shape::Point { x: 1_000, y: 2_000 },
      size: crate::pptx::drawingml::shape::Size {
        cx: 4_000,
        cy: 6_000,
      },
      child_position: crate::pptx::drawingml::shape::Point { x: 100, y: 200 },
      child_size: crate::pptx::drawingml::shape::Size {
        cx: 2_000,
        cy: 3_000,
      },
      ..Shape::new(ShapeService::Group)
    };

    let child = child_display_offset(&shape, DisplayOffset::default());

    assert_eq!(child.0.as_coeffs()[0], 2.0);
    assert_eq!(child.scale_y(), 2.0);
    assert_eq!(child.0.as_coeffs()[4], 800.0);
    assert_eq!(child.0.as_coeffs()[5], 1_600.0);
    assert_eq!(child.x_pt(100), units::emu_to_points_f32(1_000.0));
    assert_eq!(child.y_pt(200), units::emu_to_points_f32(2_000.0));
  }

  #[test]
  fn horizontal_merge_continuation_does_not_consume_a_second_grid_column() {
    assert_eq!(table_grid_advance(false, 2), 2);
    assert_eq!(table_grid_advance(true, 1), 0);
  }

  #[test]
  fn jfif_density_reports_physical_image_resolution() {
    let jpeg = [
      0xff, 0xd8, 0xff, 0xe0, 0x00, 0x10, b'J', b'F', b'I', b'F', 0x00, 0x01, 0x01, 0x01, 0x00,
      0x4b, 0x00, 0x4b, 0x00, 0x00, 0xff, 0xd9,
    ];

    assert_eq!(jpeg_density_dpi(&jpeg), Some((75.0, 75.0)));
  }

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
    assert_eq!(presentation_field_text(&run, 7, None, None, None), "7");
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
  fn automatic_soft_break_empty_line_advances_one_em() {
    let style = TextStyle {
      font_size_pt: 24.0,
      ..TextStyle::default()
    };

    assert_eq!(automatic_soft_break_empty_line_height(&style, 1.0), 24.0);
    assert_eq!(automatic_soft_break_empty_line_height(&style, 0.8), 19.2);
  }

  #[test]
  fn rounded_autofit_never_produces_a_zero_point_font() {
    assert_eq!(scaled_text_font_size_pt(2.0, 0.1, true), 1.0);
    assert_eq!(scaled_text_font_size_pt(12.0, 0.5, true), 6.0);
  }

  #[test]
  fn character_bullet_uses_one_unicode_code_point() {
    assert_eq!(character_bullet_label("••").as_deref(), Some("•"));
    assert_eq!(character_bullet_label(""), None);
  }

  #[test]
  fn picture_bullet_uses_resolved_and_natural_line_heights() {
    assert!((picture_bullet_y_pt(10.0, 38.4, 38.4, 20.0) - 19.2).abs() < 0.001);
    assert!((picture_bullet_y_pt(10.0, 30.24, 33.6, 20.0) - 13.44).abs() < 0.001);
  }

  #[test]
  fn picture_bullet_is_the_centered_content_of_an_empty_visible_line() {
    assert_eq!(
      picture_bullet_x_pt(100.0, 20.0, 0.0, a::TextAlignmentTypeValues::Center),
      90.0
    );
    assert_eq!(
      picture_bullet_x_pt(100.0, 20.0, 30.0, a::TextAlignmentTypeValues::Center),
      100.0
    );
  }

  #[test]
  fn trailing_space_keeps_its_style_for_line_height() {
    let source_run = TextRun {
      text: " ".to_string(),
      kind: TextRunKind::Run,
      hyperlink_url: None,
      field_type: None,
      run_properties: None,
      field_paragraph_properties: None,
    };
    let mut line = TextLine {
      width_pt: 8.0,
      runs: vec![TextLineRun {
        run: &source_run,
        text: " ".to_string(),
        width_pt: 8.0,
        style: TextStyle {
          font_size_pt: 32.0,
          ..TextStyle::default()
        },
        kind: TextLineRunKind::Text,
      }],
    };

    trim_text_line_end(&mut line, &mut TextMetrics::new());

    assert!(line.width_pt.abs() < f32::EPSILON);
    assert_eq!(line.runs.len(), 1);
    assert!(line.runs[0].text.is_empty());
    assert_eq!(line.runs[0].style.font_size_pt, 32.0);
  }

  #[test]
  fn formatting_run_boundaries_do_not_create_line_break_opportunities() {
    assert!(text_spans_join_without_break("keywords (", "endpararpr"));
    assert!(text_spans_join_without_break("endpararpr", ")."));
    assert!(!text_spans_join_without_break("keywords ", "next"));
    assert!(!text_spans_join_without_break("word", " next"));
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
  fn paragraph_margins_follow_style_precedence_and_reduce_the_line_box() {
    let paragraph = TextParagraph {
      master_paragraph_style: Some(TextListParagraphStyle::Default(Box::new(
        a::DefaultParagraphProperties {
          left_margin: Some(457_200),
          right_margin: Some(91_440),
          ..a::DefaultParagraphProperties::default()
        },
      ))),
      text_paragraph_style: Some(TextListParagraphStyle::Default(Box::new(
        a::DefaultParagraphProperties {
          left_margin: Some(742_950),
          ..a::DefaultParagraphProperties::default()
        },
      ))),
      paragraph_properties: Some(Box::new(a::ParagraphProperties {
        right_margin: Some(182_880),
        ..a::ParagraphProperties::default()
      })),
      ..TextParagraph::default()
    };

    let style = ParagraphDisplayStyle::from_paragraph(&paragraph);

    assert!((style.left_margin_pt - 58.5).abs() < 0.001);
    assert!((style.right_margin_pt - 14.4).abs() < 0.001);
    let bullet_left = style.left_offset(true);
    assert!((style.available_width(633.6, bullet_left) - 560.7).abs() < 0.001);
    assert_eq!(style.available_width(50.0, bullet_left), 0.0);

    let hanging = ParagraphDisplayStyle {
      left_margin_pt: 8.781_496,
      indent_pt: -8.781_496,
      ..ParagraphDisplayStyle::default()
    };
    let unbulleted_left = hanging.left_offset(false);
    assert!(unbulleted_left.abs() < 0.001);
    assert!((hanging.available_width(74.192_28, unbulleted_left) - 74.192_28).abs() < 0.001);
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

  #[test]
  fn office_alpha_modulation_wraps_values_beyond_one_hundred_percent() {
    assert_eq!(
      office_alpha_modulate_amount(DrawingmlPercentageValue::Decimal(100_000)),
      1.0
    );
    assert_eq!(
      office_alpha_modulate_amount(DrawingmlPercentageValue::Decimal(150_000)),
      0.5
    );
    assert_eq!(
      office_alpha_modulate_amount(DrawingmlPercentageValue::Decimal(200_000)),
      1.0
    );
  }

  #[test]
  fn libreoffice_duotone_interpolation_uses_pixel_luminance() {
    assert_eq!(duotone_component(0, 20, 220), 20);
    assert_eq!(duotone_component(255, 20, 220), 220);
    assert_eq!(duotone_component(128, 0, 255), 128);
  }

  #[test]
  fn office_suppresses_group_fill_on_both_diagram_shape_models() {
    let model = dgm::ShapeProperties {
      shape_properties_choice2: Some(dgm::ShapePropertiesChoice2::GroupFill),
      ..Default::default()
    };
    let persisted = dsp::ShapeProperties {
      shape_properties_choice2: Some(dsp::ShapePropertiesChoice2::GroupFill),
      ..Default::default()
    };

    assert!(diagram_model_shape_suppresses_fill(&model));
    assert!(diagram_shape_suppresses_fill(&persisted));
  }

  #[test]
  fn office_circle_gradient_circumscribes_the_transformed_rectangle() {
    let transform = common::office_circle_gradient_transform(common::Transform {
      m11: 3.0,
      m22: 4.0,
      ..common::Transform::default()
    });

    assert!((transform.m11 - 5.0).abs() < f32::EPSILON);
    assert!((transform.m22 - 5.0).abs() < f32::EPSILON);
    assert!((transform.dx.0 + 1.0).abs() < f32::EPSILON);
    assert!((transform.dy.0 + 0.5).abs() < f32::EPSILON);
  }

  #[test]
  fn office_path_gradient_focus_is_converted_from_shape_to_tile_space() {
    let gradient = a::GradientFill {
      tile_rectangle: Some(a::TileRectangle {
        top: Some(DrawingmlPercentageValue::Decimal(-100_000)),
        right: Some(DrawingmlPercentageValue::Decimal(-100_000)),
        ..Default::default()
      }),
      ..Default::default()
    };
    let path = a::PathGradientFill {
      fill_to_rectangle: Some(a::FillToRectangle {
        left: Some(DrawingmlPercentageValue::Decimal(100_000)),
        bottom: Some(DrawingmlPercentageValue::Decimal(100_000)),
        ..Default::default()
      }),
      ..Default::default()
    };
    let resolved = common::drawingml_gradient::resolve_path_gradient(
      &gradient,
      &path,
      common::Transform::default(),
    );

    assert_eq!(
      resolved.fill_to,
      common::RelativeRect {
        left: 0.5,
        top: 0.5,
        right: 0.5,
        bottom: 0.5,
      }
    );
    assert_eq!(resolved.transform.m11, 2.0);
    assert_eq!(resolved.transform.m22, 2.0);
    assert_eq!(resolved.transform.dx.0, 0.0);
    assert_eq!(resolved.transform.dy.0, -1.0);
    assert!(!resolved.mirror_tile);
  }

  #[test]
  fn missing_picture_icon_matches_office_fixed_output_pixels() {
    let image = image::load_from_memory(&missing_picture_icon_png()).unwrap();
    assert_eq!(image.dimensions(), (4, 5));
    let rgb = image.to_rgb8();
    assert_eq!(rgb.get_pixel(0, 0).0, [128, 128, 128]);
    assert_eq!(rgb.get_pixel(2, 2).0, [255, 0, 0]);
    assert_eq!(rgb.get_pixel(1, 3).0, [255, 204, 204]);
    assert_eq!(rgb.get_pixel(3, 4).0, [255, 255, 255]);
  }
}
