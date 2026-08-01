use image::RgbaImage;
use skrifa::{
  FontRef, GlyphId, MetadataProvider,
  instance::{LocationRef, Size},
  outline::{DrawSettings, OutlinePen},
  raw::TableProvider,
};
use tiny_skia::{
  Color as SkColor, FillRule, FilterQuality, GradientStop as SkGradientStop, LineCap, LineJoin,
  LinearGradient, Paint, Path, PathBuilder, Pattern, Pixmap, Point as SkPoint,
  PremultipliedColorU8, SpreadMode, Stroke as SkStroke, StrokeDash, Transform as SkTransform,
};

use super::{
  Color, DisplayItem, Fill, GradientFill, ImageItem, LineItem, PathCommand, PathItem, PatternFill,
  Rect, RectItem, Stroke, TextRun,
};
use crate::text_metrics::TextMetrics;

const MAX_EFFECT_RASTER_PIXELS: f32 = 250_000.0;
const MAX_EFFECT_PIXELS_PER_POINT: f32 = 2.0;
const DRAWINGML_PATTERN_TILE_PT: f32 = 6.0;

#[derive(Debug)]
pub(crate) struct DrawingRaster {
  pub(crate) image: RgbaImage,
  pub(crate) fill_image: Option<RgbaImage>,
  pub(crate) line_image: Option<RgbaImage>,
  pub(crate) fill_line_image: Option<RgbaImage>,
  pub(crate) children_image: Option<RgbaImage>,
  pub(crate) pixels_per_point: f32,
}

pub(crate) fn rescale_drawing_raster(raster: &mut DrawingRaster, pixels_per_point: f32) {
  if !pixels_per_point.is_finite()
    || pixels_per_point <= 0.0
    || pixels_per_point >= raster.pixels_per_point
  {
    return;
  }
  let scale = pixels_per_point / raster.pixels_per_point;
  let width = ((raster.image.width() as f32 * scale).round() as u32).max(1);
  let height = ((raster.image.height() as f32 * scale).round() as u32).max(1);
  raster.image = image::imageops::resize(
    &raster.image,
    width,
    height,
    image::imageops::FilterType::Lanczos3,
  );
  let resize = |image: &RgbaImage| {
    image::imageops::resize(image, width, height, image::imageops::FilterType::Lanczos3)
  };
  raster.fill_image = raster.fill_image.as_ref().map(resize);
  raster.line_image = raster.line_image.as_ref().map(resize);
  raster.fill_line_image = raster.fill_line_image.as_ref().map(resize);
  raster.children_image = raster.children_image.as_ref().map(resize);
  raster.pixels_per_point = pixels_per_point;
}

/// Rasterizes one already-resolved 2-D Drawing shape for effects that require
/// full-color pixels.
///
/// The input contract is intentionally strict: only vector items whose paint
/// can be reproduced exactly here are accepted. Callers retain their vector
/// display list when this function returns `None`, so a gradient, image, text,
/// or group is never silently replaced by a rectangle or an alpha-only blur.
#[cfg(test)]
pub(crate) fn rasterize_vector_items(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
) -> Option<DrawingRaster> {
  rasterize_vector_items_impl(items, raster_bounds).map(|(image, pixels_per_point)| DrawingRaster {
    image,
    fill_image: None,
    line_image: None,
    fill_line_image: None,
    children_image: None,
    pixels_per_point,
  })
}

pub(crate) fn rasterize_vector_items_for_effects(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  effects: &super::drawingml_image_effects::ImageEffectContainer,
) -> Option<DrawingRaster> {
  rasterize_vector_items_for_effects_impl(items, raster_bounds, effects, false)
}

pub(crate) fn rasterize_vector_items_for_effects_at_pixels_per_point(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  pixels_per_point: f32,
) -> Option<DrawingRaster> {
  if super::drawingml_image_effects::source_requirements(effects)
    != super::drawingml_image_effects::ImageEffectSourceRequirements::default()
  {
    let mut raster = rasterize_vector_items_for_effects(items, raster_bounds, effects)?;
    rescale_drawing_raster(&mut raster, pixels_per_point);
    return Some(raster);
  }
  let (image, pixels_per_point) =
    rasterize_vector_items_impl_at_pixels_per_point(items, raster_bounds, pixels_per_point)?;
  Some(DrawingRaster {
    image,
    fill_image: None,
    line_image: None,
    fill_line_image: None,
    children_image: None,
    pixels_per_point,
  })
}

pub(crate) fn rasterize_vector_items_for_effects_at_bounded_pixels_per_point(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  max_pixels_per_point: f32,
) -> Option<DrawingRaster> {
  let pixels_per_point = effect_pixels_per_point_with_max(
    raster_bounds.size.width.0,
    raster_bounds.size.height.0,
    max_pixels_per_point,
  );
  rasterize_vector_items_for_effects_at_pixels_per_point(
    items,
    raster_bounds,
    effects,
    pixels_per_point,
  )
}

pub(crate) fn rasterize_fill_layer_at_pixels_per_point(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  pixels_per_point: f32,
) -> Option<DrawingRaster> {
  let mut fill_items = Vec::new();
  for item in items {
    collect_source_layer_item(item, SourceLayer::Fill, &mut fill_items)?;
  }
  let (image, pixels_per_point) =
    rasterize_vector_items_impl_at_pixels_per_point(&fill_items, raster_bounds, pixels_per_point)?;
  Some(DrawingRaster {
    image,
    fill_image: None,
    line_image: None,
    fill_line_image: None,
    children_image: None,
    pixels_per_point,
  })
}

pub(crate) fn rasterize_group_items_for_effects(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  effects: &super::drawingml_image_effects::ImageEffectContainer,
) -> Option<DrawingRaster> {
  rasterize_vector_items_for_effects_impl(items, raster_bounds, effects, true)
}

pub(crate) fn rasterize_group_items_for_effects_at_pixels_per_point(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  pixels_per_point: f32,
) -> Option<DrawingRaster> {
  if super::drawingml_image_effects::source_requirements(effects)
    == super::drawingml_image_effects::ImageEffectSourceRequirements::default()
  {
    let (image, pixels_per_point) =
      rasterize_vector_items_impl_at_pixels_per_point(items, raster_bounds, pixels_per_point)?;
    return Some(DrawingRaster {
      image,
      fill_image: None,
      line_image: None,
      fill_line_image: None,
      children_image: None,
      pixels_per_point,
    });
  }
  let mut raster = rasterize_group_items_for_effects(items, raster_bounds, effects)?;
  rescale_drawing_raster(&mut raster, pixels_per_point);
  Some(raster)
}

fn rasterize_vector_items_for_effects_impl(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  items_are_children_source: bool,
) -> Option<DrawingRaster> {
  let requirements = super::drawingml_image_effects::source_requirements(effects);
  // A logical children source can only be produced while retaining the host
  // group's child display list. A leaf-shape raster must fail strictly here so
  // callers keep the original vector content rather than substituting an
  // empty source.
  if requirements.children && !items_are_children_source {
    return None;
  }
  let (image, pixels_per_point) = rasterize_vector_items_impl(items, raster_bounds)?;
  let fill_image = if requirements.fill && items_are_children_source {
    Some(empty_raster(raster_bounds)?.0)
  } else if requirements.fill {
    Some(rasterize_source_layer(
      items,
      raster_bounds,
      SourceLayer::Fill,
    )?)
    .map(|layer| layer.0)
  } else {
    None
  };
  let line_image = if requirements.line && items_are_children_source {
    Some(empty_raster(raster_bounds)?.0)
  } else if requirements.line {
    Some(rasterize_source_layer(
      items,
      raster_bounds,
      SourceLayer::Line,
    )?)
    .map(|layer| layer.0)
  } else {
    None
  };
  Some(DrawingRaster {
    children_image: (requirements.children && items_are_children_source).then(|| image.clone()),
    fill_line_image: if requirements.fill_line && items_are_children_source {
      Some(empty_raster(raster_bounds)?.0)
    } else {
      None
    },
    image,
    fill_image,
    line_image,
    pixels_per_point,
  })
}

#[derive(Clone, Copy)]
enum SourceLayer {
  Fill,
  Line,
}

fn rasterize_source_layer(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  layer: SourceLayer,
) -> Option<(RgbaImage, f32)> {
  let mut layer_items = Vec::new();
  for item in items {
    collect_source_layer_item(item, layer, &mut layer_items)?;
  }
  if layer_items.is_empty() {
    let (image, pixels_per_point) = empty_raster(raster_bounds)?;
    return Some((image, pixels_per_point));
  }
  rasterize_vector_items_impl(&layer_items, raster_bounds)
}

fn collect_source_layer_item(
  item: &DisplayItem<'static>,
  layer: SourceLayer,
  output: &mut Vec<DisplayItem<'static>>,
) -> Option<()> {
  let selected = match (layer, item) {
    (SourceLayer::Fill, DisplayItem::Image(image)) => Some(DisplayItem::Image(image.clone())),
    (SourceLayer::Fill, DisplayItem::Path(path)) => {
      let mut path = path.clone();
      path.stroke = None;
      Some(DisplayItem::Path(path))
    }
    (SourceLayer::Fill, DisplayItem::Rect(rect)) => {
      let mut rect = rect.clone();
      rect.stroke = None;
      Some(DisplayItem::Rect(rect))
    }
    (SourceLayer::Line, DisplayItem::Path(path)) => {
      let mut path = path.clone();
      path.fill = Fill::None;
      path.stroke.as_ref()?;
      Some(DisplayItem::Path(path))
    }
    (SourceLayer::Line, DisplayItem::Rect(rect)) => {
      let mut rect = rect.clone();
      rect.fill = Fill::None;
      rect.stroke.as_ref()?;
      Some(DisplayItem::Rect(rect))
    }
    (SourceLayer::Line, DisplayItem::Line(line)) => Some(DisplayItem::Line(line.clone())),
    _ => None,
  };
  if let Some(selected) = selected {
    output.push(selected);
  } else if let DisplayItem::Group(group) = item
    && simple_raster_group(group)
  {
    for child in &group.items {
      collect_source_layer_item(child, layer, output)?;
    }
  }
  Some(())
}

fn empty_raster(raster_bounds: Rect) -> Option<(RgbaImage, f32)> {
  let width_pt = raster_bounds.size.width.0;
  let height_pt = raster_bounds.size.height.0;
  if width_pt <= 0.0 || height_pt <= 0.0 {
    return None;
  }
  let pixels_per_point = effect_pixels_per_point(width_pt, height_pt);
  Some((
    RgbaImage::new(
      (width_pt * pixels_per_point).ceil().max(1.0) as u32,
      (height_pt * pixels_per_point).ceil().max(1.0) as u32,
    ),
    pixels_per_point,
  ))
}

fn effect_pixels_per_point(width_pt: f32, height_pt: f32) -> f32 {
  effect_pixels_per_point_with_max(width_pt, height_pt, MAX_EFFECT_PIXELS_PER_POINT)
}

fn effect_pixels_per_point_with_max(
  width_pt: f32,
  height_pt: f32,
  max_pixels_per_point: f32,
) -> f32 {
  (MAX_EFFECT_RASTER_PIXELS / (width_pt * height_pt))
    .sqrt()
    .clamp(0.25, max_pixels_per_point.max(0.25))
}

fn rasterize_vector_items_impl(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
) -> Option<(RgbaImage, f32)> {
  let width_pt = raster_bounds.size.width.0;
  let height_pt = raster_bounds.size.height.0;
  if width_pt <= 0.0 || height_pt <= 0.0 {
    return None;
  }
  if items.iter().any(|item| !supported_raster_item(item)) {
    return None;
  }

  let pixels_per_point = effect_pixels_per_point(width_pt, height_pt);
  rasterize_vector_items_impl_at_pixels_per_point(items, raster_bounds, pixels_per_point)
}

fn rasterize_vector_items_impl_at_pixels_per_point(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  pixels_per_point: f32,
) -> Option<(RgbaImage, f32)> {
  let width_pt = raster_bounds.size.width.0;
  let height_pt = raster_bounds.size.height.0;
  if width_pt <= 0.0
    || height_pt <= 0.0
    || !pixels_per_point.is_finite()
    || pixels_per_point <= 0.0
    || items.iter().any(|item| !supported_raster_item(item))
  {
    return None;
  }
  let width_px = (width_pt * pixels_per_point).ceil().max(1.0) as u32;
  let height_px = (height_pt * pixels_per_point).ceil().max(1.0) as u32;
  let mut pixmap = Pixmap::new(width_px, height_px)?;
  let mut text_metrics = TextMetrics::new();
  let page_to_raster = SkTransform::from_row(
    pixels_per_point,
    0.0,
    0.0,
    pixels_per_point,
    -raster_bounds.origin.x.0 * pixels_per_point,
    -raster_bounds.origin.y.0 * pixels_per_point,
  );

  for item in items {
    draw_display_item(&mut pixmap, item, page_to_raster, &mut text_metrics)?;
  }

  let png = pixmap.encode_png().ok()?;
  let image = image::load_from_memory(&png).ok()?.to_rgba8();
  Some((image, pixels_per_point))
}

fn supported_raster_item(item: &DisplayItem<'static>) -> bool {
  match item {
    DisplayItem::Text(_)
    | DisplayItem::Image(_)
    | DisplayItem::Path(_)
    | DisplayItem::Rect(_)
    | DisplayItem::Line(_) => true,
    DisplayItem::Group(group) => {
      simple_raster_group(group) && group.items.iter().all(supported_raster_item)
    }
    DisplayItem::Glyphs(_)
    | DisplayItem::LinkArea(_)
    | DisplayItem::AnnotationHint(_)
    | DisplayItem::Clip(_)
    | DisplayItem::Transform(_) => false,
  }
}

fn simple_raster_group(group: &super::CompositingGroup<'static>) -> bool {
  group.mask.is_none()
    && group.transform.is_none()
    && group.blend_mode == super::BlendMode::Normal
    && (group.opacity - 1.0).abs() <= f32::EPSILON
}

fn draw_display_item(
  pixmap: &mut Pixmap,
  item: &DisplayItem<'static>,
  page_to_raster: SkTransform,
  text_metrics: &mut TextMetrics,
) -> Option<()> {
  match item {
    DisplayItem::Text(text) => draw_text(pixmap, text, page_to_raster, text_metrics)?,
    DisplayItem::Image(image) => draw_image(pixmap, image, page_to_raster)?,
    DisplayItem::Path(path) => draw_path(pixmap, path, page_to_raster)?,
    DisplayItem::Rect(rect) => draw_rect(pixmap, rect, page_to_raster)?,
    DisplayItem::Line(line) => draw_line(pixmap, line, page_to_raster)?,
    DisplayItem::Group(group) => {
      for child in &group.items {
        draw_display_item(pixmap, child, page_to_raster, text_metrics)?;
      }
    }
    DisplayItem::Glyphs(_)
    | DisplayItem::LinkArea(_)
    | DisplayItem::AnnotationHint(_)
    | DisplayItem::Clip(_)
    | DisplayItem::Transform(_) => {
      unreachable!("unsupported drawing item rejected before rasterization")
    }
  }
  Some(())
}

fn draw_text(
  pixmap: &mut Pixmap,
  item: &TextRun<'static>,
  page_to_raster: SkTransform,
  text_metrics: &mut TextMetrics,
) -> Option<()> {
  if item.style.semantic_only || item.style.hidden || item.text.is_empty() {
    return Some(());
  }
  let shaped = text_metrics.shape_text(item.text.as_ref(), &item.style)?;
  let baseline_offset = if item.style.use_windows_font_metrics {
    text_metrics.baseline_offset_in_line_with_windows_metrics_for_text(
      item.text.as_ref(),
      &item.style,
      item.line_height.0,
    )
  } else {
    text_metrics.baseline_offset_in_line_for_text(
      item.text.as_ref(),
      &item.style,
      item.line_height.0,
    )
  };
  let baseline_y = item.origin.y.0 + baseline_offset;
  let horizontal_scale = item.style.horizontal_scale.unwrap_or(1.0);
  let mut commands = Vec::new();
  let mut cursor_x = item.origin.x.0;
  for glyph in &shaped.glyphs {
    let face_data = shaped.font_faces.get(glyph.font_index)?;
    let face = FontRef::from_index(face_data.data.as_ref(), face_data.index).ok()?;
    let units_per_em = face
      .head()
      .map(|head| f32::from(head.units_per_em()))
      .ok()?;
    if units_per_em <= f32::EPSILON {
      return None;
    }
    let origin_x = cursor_x + glyph.x_offset_em * glyph.font_size_pt;
    let origin_y = baseline_y - glyph.y_offset_em * glyph.font_size_pt;
    let mut outline = RasterGlyphOutline {
      commands: &mut commands,
      origin_x,
      origin_y,
      scale: glyph.font_size_pt / units_per_em,
      horizontal_scale,
      synthetic_italic: face_data.synthetic_italic,
      rotation_degrees: item.style.rotation_degrees,
      rotation_center: item.rotation_center,
      current: None,
    };
    if let Some(glyph_outline) = face.outline_glyphs().get(GlyphId::new(glyph.glyph_id)) {
      let _ = glyph_outline.draw(
        DrawSettings::unhinted(Size::unscaled(), LocationRef::default()),
        &mut outline,
      );
    }
    cursor_x += glyph.x_advance_em * glyph.font_size_pt;
    if item
      .text
      .get(glyph.text_range.clone())
      .is_some_and(|cluster| cluster.contains(' '))
    {
      cursor_x += item.word_spacing_pt;
    }
  }
  if commands.is_empty() {
    return Some(());
  }
  let path = path_from_commands(&commands, &[], true)?;
  let bounds = Rect {
    origin: super::Point {
      x: item.origin.x,
      y: item.origin.y,
    },
    size: super::Size {
      width: super::Pt(shaped.width_pt.max(item.style.font_size.0)),
      height: super::Pt(item.line_height.0.max(item.style.font_size.0)),
    },
  };
  let mut fill = item
    .style
    .pdf_glyph_outline_options
    .as_ref()
    .and_then(|options| options.fill.clone())
    .unwrap_or(Fill::Solid(item.color));
  resolve_text_raster_fill(&mut fill, bounds);
  draw_fill(
    pixmap,
    &path,
    &fill,
    bounds,
    Some(&commands),
    page_to_raster,
  )?;
  let mut stroke = item
    .style
    .pdf_glyph_outline_options
    .as_ref()
    .and_then(|options| options.outline_stroke.clone())
    .or_else(|| {
      item
        .style
        .outline_color
        .filter(|_| item.style.outline_width.0 > f32::EPSILON)
        .map(|color| Stroke {
          width: item.style.outline_width,
          color,
          ..Default::default()
        })
    });
  if let Some(outline_fill) = item
    .style
    .pdf_glyph_outline_options
    .as_ref()
    .and_then(|options| options.outline_fill.clone())
    && item.style.outline_width.0 > f32::EPSILON
  {
    let mut outline_fill = outline_fill;
    resolve_text_raster_fill(&mut outline_fill, bounds);
    let resolved = stroke.get_or_insert_with(|| Stroke {
      width: item.style.outline_width,
      color: Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
      },
      ..Default::default()
    });
    match outline_fill {
      Fill::Solid(color) => resolved.color = color,
      Fill::Gradient(gradient) => resolved.gradient = Some(gradient),
      Fill::Pattern(pattern) => resolved.pattern = Some(pattern),
      Fill::None | Fill::Theme(_) | Fill::Image { .. } => {}
    }
  }
  if let Some(stroke) = &stroke {
    draw_stroke(
      pixmap,
      &path,
      stroke,
      bounds,
      Some(&commands),
      page_to_raster,
    )?;
  }
  Some(())
}

fn resolve_text_raster_fill(fill: &mut Fill<'static>, bounds: Rect) {
  let Fill::Gradient(gradient) = fill else {
    return;
  };
  let unresolved = gradient.definition_bounds.is_none();
  gradient.definition_bounds.get_or_insert(bounds);
  if let Some(path) = &mut gradient.path
    && unresolved
  {
    path.transform =
      super::drawingml_gradient::bind_path_transform_to_bounds(path.transform, bounds);
    if path.kind == super::GradientPathKind::Circle {
      path.transform = super::office_circle_gradient_transform(path.transform);
    }
  }
}

struct RasterGlyphOutline<'a> {
  commands: &'a mut Vec<PathCommand>,
  origin_x: f32,
  origin_y: f32,
  scale: f32,
  horizontal_scale: f32,
  synthetic_italic: bool,
  rotation_degrees: f32,
  rotation_center: Option<super::Point>,
  current: Option<super::Point>,
}

impl RasterGlyphOutline<'_> {
  fn point(&self, x: f32, y: f32) -> super::Point {
    let x = if self.synthetic_italic {
      x + y / 3.0
    } else {
      x
    };
    let mut point = super::Point {
      x: super::Pt(self.origin_x + x * self.scale * self.horizontal_scale),
      y: super::Pt(self.origin_y - y * self.scale),
    };
    if self.rotation_degrees.abs() > f32::EPSILON {
      let center = self.rotation_center.unwrap_or(super::Point {
        x: super::Pt(self.origin_x),
        y: super::Pt(self.origin_y),
      });
      let (sin, cos) = self.rotation_degrees.to_radians().sin_cos();
      let x = point.x.0 - center.x.0;
      let y = point.y.0 - center.y.0;
      point.x.0 = center.x.0 + cos.mul_add(x, -sin * y);
      point.y.0 = center.y.0 + sin.mul_add(x, cos * y);
    }
    point
  }
}

impl OutlinePen for RasterGlyphOutline<'_> {
  fn move_to(&mut self, x: f32, y: f32) {
    let point = self.point(x, y);
    self.commands.push(PathCommand::MoveTo(point));
    self.current = Some(point);
  }

  fn line_to(&mut self, x: f32, y: f32) {
    let point = self.point(x, y);
    self.commands.push(PathCommand::LineTo(point));
    self.current = Some(point);
  }

  fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
    let control = self.point(x1, y1);
    let end = self.point(x, y);
    if let Some(start) = self.current {
      self.commands.push(PathCommand::CubicTo {
        control1: super::Point {
          x: super::Pt(start.x.0 + (control.x.0 - start.x.0) * (2.0 / 3.0)),
          y: super::Pt(start.y.0 + (control.y.0 - start.y.0) * (2.0 / 3.0)),
        },
        control2: super::Point {
          x: super::Pt(end.x.0 + (control.x.0 - end.x.0) * (2.0 / 3.0)),
          y: super::Pt(end.y.0 + (control.y.0 - end.y.0) * (2.0 / 3.0)),
        },
        end,
      });
    }
    self.current = Some(end);
  }

  fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
    let control1 = self.point(x1, y1);
    let control2 = self.point(x2, y2);
    let end = self.point(x, y);
    self.commands.push(PathCommand::CubicTo {
      control1,
      control2,
      end,
    });
    self.current = Some(end);
  }

  fn close(&mut self) {
    self.commands.push(PathCommand::Close);
    self.current = None;
  }
}

fn draw_image(
  pixmap: &mut Pixmap,
  item: &ImageItem<'static>,
  page_to_raster: SkTransform,
) -> Option<()> {
  let raster_data = crate::render::emf_wmf::decode_metafile_as_raster(
    item.bytes.as_ref(),
    Some(item.content_type.as_ref()),
  )
  .ok()
  .flatten()
  .map(|decoded| decoded.data);
  let source_data = raster_data.as_deref().unwrap_or(item.bytes.as_ref());
  let source = image::load_from_memory(source_data).ok()?.to_rgba8();
  let crop = item.crop.unwrap_or_default();
  let visible_width = 1.0 - crop.left - crop.right;
  let visible_height = 1.0 - crop.top - crop.bottom;
  let width_pt = item.bounds.size.width.0;
  let height_pt = item.bounds.size.height.0;
  if visible_width <= f32::EPSILON
    || visible_height <= f32::EPSILON
    || width_pt <= f32::EPSILON
    || height_pt <= f32::EPSILON
  {
    return None;
  }

  let mut mask = Pixmap::new(pixmap.width(), pixmap.height())?;
  if item.clip_path.is_empty() {
    mask.fill(SkColor::WHITE);
  } else {
    let clip_path = path_from_commands(&item.clip_path, &[], true)?;
    let mut mask_paint = Paint::default();
    mask_paint.set_color_rgba8(255, 255, 255, 255);
    mask_paint.anti_alias = true;
    mask.fill_path(
      &clip_path,
      &mask_paint,
      FillRule::Winding,
      page_to_raster,
      None,
    );
  }

  let pixels_per_point = page_to_raster.sx;
  let raster_origin_x = -page_to_raster.tx / pixels_per_point;
  let raster_origin_y = -page_to_raster.ty / pixels_per_point;
  let center_x = item.bounds.origin.x.0 + width_pt * 0.5;
  let center_y = item.bounds.origin.y.0 + height_pt * 0.5;
  let angle = item.rotation_degrees.to_radians();
  let (sin, cos) = angle.sin_cos();
  let source_width = source.width() as f32;
  let source_height = source.height() as f32;

  for y in 0..pixmap.height() {
    let page_y = raster_origin_y + (y as f32 + 0.5) / pixels_per_point;
    for x in 0..pixmap.width() {
      let mask_alpha = mask.pixel(x, y)?.alpha();
      if mask_alpha == 0 {
        continue;
      }
      let page_x = raster_origin_x + (x as f32 + 0.5) / pixels_per_point;
      let offset_x = page_x - center_x;
      let offset_y = page_y - center_y;
      let local_x = cos.mul_add(offset_x, sin * offset_y) + width_pt * 0.5;
      let local_y = (-sin).mul_add(offset_x, cos * offset_y) + height_pt * 0.5;
      if local_x < 0.0 || local_y < 0.0 || local_x > width_pt || local_y > height_pt {
        continue;
      }
      let mut u = local_x / width_pt;
      let mut v = local_y / height_pt;
      if item.flip_horizontal {
        u = 1.0 - u;
      }
      if item.flip_vertical {
        v = 1.0 - v;
      }
      u = crop.left + u * visible_width;
      v = crop.top + v * visible_height;
      let sample = bilinear_sample(
        &source,
        u.mul_add(source_width, -0.5),
        v.mul_add(source_height, -0.5),
      );
      composite_straight_rgba_over_pixmap(pixmap, x, y, sample.0, mask_alpha)?;
    }
  }
  Some(())
}

fn bilinear_sample(source: &image::RgbaImage, x: f32, y: f32) -> image::Rgba<u8> {
  let x = x.clamp(-0.5, source.width() as f32 - 0.5);
  let y = y.clamp(-0.5, source.height() as f32 - 0.5);
  let x0 = x.floor() as i64;
  let y0 = y.floor() as i64;
  let x_amount = x - x.floor();
  let y_amount = y - y.floor();
  let sample = |sample_x: i64, sample_y: i64| {
    let sample_x = sample_x.clamp(0, i64::from(source.width()) - 1) as u32;
    let sample_y = sample_y.clamp(0, i64::from(source.height()) - 1) as u32;
    source.get_pixel(sample_x, sample_y).0
  };
  let top_left = sample(x0, y0);
  let top_right = sample(x0 + 1, y0);
  let bottom_left = sample(x0, y0 + 1);
  let bottom_right = sample(x0 + 1, y0 + 1);
  let weights = [
    (1.0 - x_amount) * (1.0 - y_amount),
    x_amount * (1.0 - y_amount),
    (1.0 - x_amount) * y_amount,
    x_amount * y_amount,
  ];
  let samples = [top_left, top_right, bottom_left, bottom_right];
  let alpha = samples
    .iter()
    .zip(weights)
    .map(|(sample, weight)| f32::from(sample[3]) * weight)
    .sum::<f32>();
  if alpha <= f32::EPSILON {
    return image::Rgba([0; 4]);
  }
  let mut result = [0_u8; 4];
  result[3] = alpha.round().clamp(0.0, 255.0) as u8;
  for channel in 0..3 {
    let premultiplied = samples
      .iter()
      .zip(weights)
      .map(|(sample, weight)| f32::from(sample[channel]) * f32::from(sample[3]) / 255.0 * weight)
      .sum::<f32>();
    result[channel] = (premultiplied * 255.0 / alpha).round().clamp(0.0, 255.0) as u8;
  }
  image::Rgba(result)
}

fn composite_straight_rgba_over_pixmap(
  pixmap: &mut Pixmap,
  x: u32,
  y: u32,
  source: [u8; 4],
  mask_alpha: u8,
) -> Option<()> {
  let source_alpha = (u32::from(source[3]) * u32::from(mask_alpha) + 127) / 255;
  if source_alpha == 0 {
    return Some(());
  }
  let offset = (y as usize * pixmap.width() as usize + x as usize) * 4;
  let destination = &mut pixmap.data_mut()[offset..offset + 4];
  let inverse_source_alpha = 255 - source_alpha;
  for channel in 0..3 {
    let source_premultiplied = (u32::from(source[channel]) * source_alpha + 127) / 255;
    destination[channel] = (source_premultiplied
      + (u32::from(destination[channel]) * inverse_source_alpha + 127) / 255)
      .min(255) as u8;
  }
  destination[3] =
    (source_alpha + (u32::from(destination[3]) * inverse_source_alpha + 127) / 255).min(255) as u8;
  Some(())
}

fn draw_path(
  pixmap: &mut Pixmap,
  item: &PathItem<'static>,
  page_to_raster: SkTransform,
) -> Option<()> {
  let path = path_from_commands(&item.commands, &item.points, item.closed)?;
  draw_fill(
    pixmap,
    &path,
    &item.fill,
    item.bounds,
    Some(&item.commands),
    page_to_raster,
  )?;
  if let Some(stroke) = &item.stroke {
    let shortened_path = shortened_straight_stroke_path(item, stroke);
    draw_stroke(
      pixmap,
      shortened_path.as_ref().unwrap_or(&path),
      stroke,
      item.bounds,
      Some(&item.commands),
      page_to_raster,
    )?;
    draw_stroke_end_markers(pixmap, item, stroke, page_to_raster)?;
  }
  Some(())
}

fn draw_stroke_end_markers(
  pixmap: &mut Pixmap,
  item: &PathItem<'static>,
  stroke: &Stroke<'static>,
  page_to_raster: SkTransform,
) -> Option<()> {
  let mut paint = solid_paint(stroke.color);
  paint.anti_alias = true;
  for polygon in super::drawingml_stroke::stroke_end_marker_polygons(item, stroke) {
    let [first, rest @ ..] = polygon.as_slice() else {
      continue;
    };
    let mut builder = PathBuilder::new();
    builder.move_to(first.x.0, first.y.0);
    for point in rest {
      builder.line_to(point.x.0, point.y.0);
    }
    builder.close();
    let path = builder.finish()?;
    pixmap.fill_path(&path, &paint, FillRule::EvenOdd, page_to_raster, None);
  }
  for marker in super::drawingml_stroke::stroked_open_arrow_markers(item, stroke) {
    let [first, middle, last] = marker.points;
    let mut builder = PathBuilder::new();
    builder.move_to(first.x.0, first.y.0);
    builder.line_to(middle.x.0, middle.y.0);
    builder.line_to(last.x.0, last.y.0);
    let path = builder.finish()?;
    let sk_stroke = SkStroke {
      width: marker.width.0,
      line_cap: LineCap::Round,
      line_join: LineJoin::Miter,
      ..SkStroke::default()
    };
    pixmap.stroke_path(&path, &paint, &sk_stroke, page_to_raster, None);
  }
  Some(())
}

fn shortened_straight_stroke_path(
  item: &PathItem<'static>,
  stroke: &Stroke<'static>,
) -> Option<Path> {
  if item.closed {
    return None;
  }
  let (start, end) = if item.commands.is_empty() {
    let [start, end] = item.points.as_slice() else {
      return None;
    };
    (*start, *end)
  } else {
    let [PathCommand::MoveTo(start), PathCommand::LineTo(end)] = item.commands.as_slice() else {
      return None;
    };
    (*start, *end)
  };
  let (head_inset, tail_inset) = super::drawingml_stroke::stroke_end_shaft_insets(stroke);
  if head_inset <= 0.0 && tail_inset <= 0.0 {
    return None;
  }
  let dx = end.x.0 - start.x.0;
  let dy = end.y.0 - start.y.0;
  let length = dx.hypot(dy);
  if length <= head_inset + tail_inset || length <= f32::EPSILON {
    return None;
  }
  let direction = (dx / length, dy / length);
  let mut builder = PathBuilder::new();
  builder.move_to(
    start.x.0 + direction.0 * head_inset,
    start.y.0 + direction.1 * head_inset,
  );
  builder.line_to(
    end.x.0 - direction.0 * tail_inset,
    end.y.0 - direction.1 * tail_inset,
  );
  builder.finish()
}

fn draw_rect(
  pixmap: &mut Pixmap,
  item: &RectItem<'static>,
  page_to_raster: SkTransform,
) -> Option<()> {
  let left = item.bounds.origin.x.0;
  let top = item.bounds.origin.y.0;
  let right = left + item.bounds.size.width.0;
  let bottom = top + item.bounds.size.height.0;
  let mut builder = PathBuilder::new();
  builder.move_to(left, top);
  builder.line_to(right, top);
  builder.line_to(right, bottom);
  builder.line_to(left, bottom);
  builder.close();
  let path = builder.finish()?;
  draw_fill(pixmap, &path, &item.fill, item.bounds, None, page_to_raster)?;
  if let Some(stroke) = &item.stroke {
    draw_stroke(pixmap, &path, stroke, item.bounds, None, page_to_raster)?;
  }
  Some(())
}

fn draw_line(
  pixmap: &mut Pixmap,
  item: &LineItem<'static>,
  page_to_raster: SkTransform,
) -> Option<()> {
  let mut builder = PathBuilder::new();
  builder.move_to(item.start.x.0, item.start.y.0);
  builder.line_to(item.end.x.0, item.end.y.0);
  let path = builder.finish()?;
  let bounds = Rect {
    origin: super::Point {
      x: super::Pt(item.start.x.0.min(item.end.x.0)),
      y: super::Pt(item.start.y.0.min(item.end.y.0)),
    },
    size: super::Size {
      width: super::Pt((item.end.x.0 - item.start.x.0).abs()),
      height: super::Pt((item.end.y.0 - item.start.y.0).abs()),
    },
  };
  draw_stroke(pixmap, &path, &item.stroke, bounds, None, page_to_raster)
}

fn path_from_commands(
  commands: &[PathCommand],
  points: &[super::Point],
  closed: bool,
) -> Option<Path> {
  let mut builder = PathBuilder::new();
  if commands.is_empty() {
    let first = points.first()?;
    builder.move_to(first.x.0, first.y.0);
    for point in &points[1..] {
      builder.line_to(point.x.0, point.y.0);
    }
    if closed {
      builder.close();
    }
  } else {
    for command in commands {
      match command {
        PathCommand::MoveTo(point) => builder.move_to(point.x.0, point.y.0),
        PathCommand::LineTo(point) => builder.line_to(point.x.0, point.y.0),
        PathCommand::CubicTo {
          control1,
          control2,
          end,
        } => builder.cubic_to(
          control1.x.0,
          control1.y.0,
          control2.x.0,
          control2.y.0,
          end.x.0,
          end.y.0,
        ),
        PathCommand::Close => builder.close(),
      }
    }
  }
  builder.finish()
}

fn draw_fill(
  pixmap: &mut Pixmap,
  path: &Path,
  fill: &Fill<'static>,
  bounds: Rect,
  commands: Option<&[PathCommand]>,
  page_to_raster: SkTransform,
) -> Option<()> {
  match fill {
    Fill::None => Some(()),
    Fill::Solid(color) => {
      let mut paint = solid_paint(*color);
      paint.anti_alias = true;
      pixmap.fill_path(path, &paint, FillRule::EvenOdd, page_to_raster, None);
      Some(())
    }
    Fill::Gradient(gradient) if gradient.path.is_none() => {
      let mut paint = linear_gradient_paint(gradient, bounds)?;
      paint.anti_alias = true;
      pixmap.fill_path(path, &paint, FillRule::EvenOdd, page_to_raster, None);
      Some(())
    }
    Fill::Gradient(gradient) => {
      draw_path_gradient(pixmap, path, gradient, commands, page_to_raster)
    }
    Fill::Pattern(pattern) => {
      let tile = pattern_tile(*pattern, page_to_raster.sx)?;
      let paint = Paint {
        anti_alias: true,
        shader: Pattern::new(
          tile.as_ref(),
          SpreadMode::Repeat,
          FilterQuality::Nearest,
          1.0,
          SkTransform::from_translate(
            drawingml_pattern_origin(bounds.origin.x.0),
            drawingml_pattern_origin(bounds.origin.y.0),
          ),
        ),
        ..Paint::default()
      };
      pixmap.fill_path(path, &paint, FillRule::EvenOdd, page_to_raster, None);
      Some(())
    }
    Fill::Theme(_) | Fill::Image { .. } => None,
  }
}

fn draw_path_gradient(
  pixmap: &mut Pixmap,
  clip_path: &Path,
  gradient: &GradientFill<'static>,
  commands: Option<&[PathCommand]>,
  page_to_raster: SkTransform,
) -> Option<()> {
  let mut mask = Pixmap::new(pixmap.width(), pixmap.height())?;
  let mut paint = Paint::default();
  paint.set_color_rgba8(255, 255, 255, 255);
  paint.anti_alias = true;
  mask.fill_path(clip_path, &paint, FillRule::EvenOdd, page_to_raster, None);
  composite_path_gradient_mask(pixmap, &mask, gradient, commands, page_to_raster)
}

fn composite_path_gradient_mask(
  pixmap: &mut Pixmap,
  mask: &Pixmap,
  gradient: &GradientFill<'static>,
  commands: Option<&[PathCommand]>,
  page_to_raster: SkTransform,
) -> Option<()> {
  let gradient_path = gradient.path?;
  let focus_width = 1.0 - gradient_path.fill_to.left - gradient_path.fill_to.right;
  let focus_height = 1.0 - gradient_path.fill_to.top - gradient_path.fill_to.bottom;
  if focus_width < 0.0 || focus_height < 0.0 || gradient.stops.is_empty() {
    return None;
  }
  let default_shape = vec![vec![
    kurbo::Point::new(0.0, 0.0),
    kurbo::Point::new(1.0, 0.0),
    kurbo::Point::new(1.0, 1.0),
    kurbo::Point::new(0.0, 1.0),
    kurbo::Point::new(0.0, 0.0),
  ]];
  let shape = if gradient_path.kind == super::GradientPathKind::Shape {
    commands
      .filter(|commands| !commands.is_empty())
      .and_then(|commands| {
        super::drawingml_gradient::shape_polygons(commands, gradient_path.transform)
      })
      .unwrap_or(default_shape)
  } else {
    Vec::new()
  };
  let stops = super::drawingml_gradient::resolved_stops(gradient);
  let pixels_per_point = page_to_raster.sx;
  let raster_origin_x = -page_to_raster.tx / pixels_per_point;
  let raster_origin_y = -page_to_raster.ty / pixels_per_point;
  for y in 0..pixmap.height() {
    let page_y = raster_origin_y + (y as f32 + 0.5) / pixels_per_point;
    for x in 0..pixmap.width() {
      let mask_alpha = mask.pixel(x, y)?.alpha();
      if mask_alpha == 0 {
        continue;
      }
      let page_x = raster_origin_x + (x as f32 + 0.5) / pixels_per_point;
      let point = super::drawingml_gradient::inverse_point(
        gradient_path.transform,
        f64::from(page_x),
        f64::from(page_y),
      )?;
      let position = super::drawingml_gradient::position(
        gradient_path,
        point,
        (!shape.is_empty()).then_some(shape.as_slice()),
      )?;
      let color = super::drawingml_gradient::sample(&stops, position);
      composite_straight_rgba_over_pixmap(
        pixmap,
        x,
        y,
        [color.r, color.g, color.b, color.a],
        mask_alpha,
      )?;
    }
  }
  Some(())
}

fn draw_stroke(
  pixmap: &mut Pixmap,
  path: &Path,
  stroke: &Stroke<'static>,
  bounds: Rect,
  commands: Option<&[PathCommand]>,
  page_to_raster: SkTransform,
) -> Option<()> {
  if stroke.width.0 <= 0.0
    || stroke.color.a == 0 && stroke.pattern.is_none() && stroke.gradient.is_none()
  {
    return Some(());
  }
  let dash = stroke.resolved_dash().and_then(|values| {
    StrokeDash::new(
      values.into_iter().map(|value| value.0).collect(),
      stroke.dash_offset.0,
    )
  });
  let sk_stroke = SkStroke {
    width: stroke.width.0,
    miter_limit: match stroke.join {
      Some(super::StrokeJoin::Miter { limit: Some(limit) }) => limit,
      _ => SkStroke::default().miter_limit,
    },
    line_cap: match stroke.cap {
      Some(super::StrokeCap::Round) => LineCap::Round,
      Some(super::StrokeCap::Square) => LineCap::Square,
      Some(super::StrokeCap::Flat) | None => LineCap::Butt,
    },
    line_join: match stroke.join {
      Some(super::StrokeJoin::Round) => LineJoin::Round,
      Some(super::StrokeJoin::Bevel) => LineJoin::Bevel,
      Some(super::StrokeJoin::Miter { .. }) | None => LineJoin::Miter,
    },
    dash,
  };
  if let Some(gradient) = stroke.gradient.as_ref() {
    if gradient.path.is_some() {
      let mut mask = Pixmap::new(pixmap.width(), pixmap.height())?;
      let mut paint = Paint::default();
      paint.set_color_rgba8(255, 255, 255, 255);
      paint.anti_alias = true;
      mask.stroke_path(path, &paint, &sk_stroke, page_to_raster, None);
      composite_path_gradient_mask(pixmap, &mask, gradient, commands, page_to_raster)?;
    } else {
      let mut paint = linear_gradient_paint(gradient, bounds)?;
      paint.anti_alias = true;
      pixmap.stroke_path(path, &paint, &sk_stroke, page_to_raster, None);
    }
  } else if let Some(pattern) = stroke.pattern {
    let tile = pattern_tile(pattern, page_to_raster.sx)?;
    let paint = Paint {
      anti_alias: true,
      shader: Pattern::new(
        tile.as_ref(),
        SpreadMode::Repeat,
        FilterQuality::Nearest,
        1.0,
        SkTransform::from_translate(
          drawingml_pattern_origin(bounds.origin.x.0),
          drawingml_pattern_origin(bounds.origin.y.0),
        ),
      ),
      ..Paint::default()
    };
    pixmap.stroke_path(path, &paint, &sk_stroke, page_to_raster, None);
  } else {
    let mut paint = solid_paint(stroke.color);
    paint.anti_alias = true;
    pixmap.stroke_path(path, &paint, &sk_stroke, page_to_raster, None);
  }
  Some(())
}

fn solid_paint(color: Color) -> Paint<'static> {
  let mut paint = Paint::default();
  paint.set_color_rgba8(color.r, color.g, color.b, color.a);
  paint
}

fn linear_gradient_paint<'a>(
  gradient: &'a GradientFill<'static>,
  painted_bounds: Rect,
) -> Option<Paint<'a>> {
  if gradient.stops.is_empty() {
    return None;
  }
  let bounds = gradient.definition_bounds.unwrap_or(painted_bounds);
  let (start, end) = gradient
    .line
    .unwrap_or_else(|| linear_gradient_line(bounds, gradient.angle_degrees, gradient.scaled));
  let resolved_stops = super::drawingml_gradient::resolved_stops(gradient);
  let stops = resolved_stops
    .iter()
    .map(|stop| {
      SkGradientStop::new(
        stop.position.clamp(0.0, 1.0),
        SkColor::from_rgba8(stop.color.r, stop.color.g, stop.color.b, stop.color.a),
      )
    })
    .collect();
  Some(Paint {
    shader: LinearGradient::new(
      SkPoint::from_xy(start.x.0, start.y.0),
      SkPoint::from_xy(end.x.0, end.y.0),
      stops,
      SpreadMode::Pad,
      SkTransform::identity(),
    )?,
    ..Paint::default()
  })
}

fn linear_gradient_line(
  bounds: Rect,
  angle_degrees: Option<f32>,
  scaled: bool,
) -> (super::Point, super::Point) {
  let angle = angle_degrees.unwrap_or(0.0).to_radians();
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
    super::Point {
      x: super::Pt(center_x - direction_x * half_span),
      y: super::Pt(center_y - direction_y * half_span),
    },
    super::Point {
      x: super::Pt(center_x + direction_x * half_span),
      y: super::Pt(center_y + direction_y * half_span),
    },
  )
}

fn pattern_tile(pattern: PatternFill, pixels_per_point: f32) -> Option<Pixmap> {
  let tile_px = (DRAWINGML_PATTERN_TILE_PT * pixels_per_point)
    .round()
    .max(8.0) as u32;
  let mut pixmap = Pixmap::new(tile_px, tile_px)?;
  for y in 0..tile_px {
    for x in 0..tile_px {
      let hatch_x = (u64::from(x) * 8 / u64::from(tile_px)) as i32;
      let hatch_y = (u64::from(y) * 8 / u64::from(tile_px)) as i32;
      let color = if pattern.hatch_style.is_foreground(hatch_x, hatch_y) {
        pattern.foreground
      } else {
        pattern.background
      };
      let alpha = u16::from(color.a);
      pixmap.pixels_mut()[y as usize * tile_px as usize + x as usize] =
        PremultipliedColorU8::from_rgba(
          ((u16::from(color.r) * alpha + 127) / 255) as u8,
          ((u16::from(color.g) * alpha + 127) / 255) as u8,
          ((u16::from(color.b) * alpha + 127) / 255) as u8,
          color.a,
        )?;
    }
  }
  Some(pixmap)
}

fn drawingml_pattern_origin(value: f32) -> f32 {
  (value / DRAWINGML_PATTERN_TILE_PT).floor() * DRAWINGML_PATTERN_TILE_PT
}

#[cfg(test)]
mod tests {
  use super::{
    MAX_EFFECT_RASTER_PIXELS, effect_pixels_per_point_with_max, rasterize_group_items_for_effects,
    rasterize_vector_items, rasterize_vector_items_for_effects,
  };
  use image::codecs::png::PngEncoder;
  use image::{ColorType, ImageEncoder, Rgba, RgbaImage};
  use std::borrow::Cow;
  use std::sync::Arc;

  use crate::common::drawingml_image_effects::{
    ImageEffect, ImageEffectContainer, ImageEffectContainerKind, ImageEffectSourceReference,
  };
  use crate::common::{
    Color, DisplayItem, Fill, GradientFill, GradientPath, GradientPathKind, GradientStop,
    ImageCrop, ImageItem, PathCommand, PathItem, Point, Pt, Rect, RectItem, RelativeRect, Size,
    Stroke, Transform,
  };

  fn rect(x: f32, y: f32, width: f32, height: f32) -> Rect {
    Rect {
      origin: Point { x: Pt(x), y: Pt(y) },
      size: Size {
        width: Pt(width),
        height: Pt(height),
      },
    }
  }

  #[test]
  fn specialized_raster_cap_keeps_the_shared_pixel_budget() {
    let small = effect_pixels_per_point_with_max(70.0, 72.0, 200.0 / 72.0);
    assert!((small - 200.0 / 72.0).abs() < 0.001);

    let large = effect_pixels_per_point_with_max(500.0, 500.0, 200.0 / 72.0);
    assert!(large < 200.0 / 72.0);
    assert!(500.0 * 500.0 * large * large <= MAX_EFFECT_RASTER_PIXELS + 1.0);
  }

  #[test]
  fn solid_vector_shape_raster_preserves_fill_and_stroke() {
    let bounds = rect(10.0, 20.0, 12.0, 8.0);
    let item = DisplayItem::Rect(RectItem {
      bounds,
      fill: Fill::Solid(Color {
        r: 220,
        g: 20,
        b: 30,
        a: 255,
      }),
      stroke: Some(Stroke {
        width: Pt(1.0),
        color: Color {
          r: 10,
          g: 20,
          b: 200,
          a: 255,
        },
        ..Stroke::default()
      }),
    });
    let raster = rasterize_vector_items(&[item], bounds).unwrap();
    let center = raster
      .image
      .get_pixel(raster.image.width() / 2, raster.image.height() / 2)
      .0;
    let edge = raster.image.get_pixel(0, raster.image.height() / 2).0;
    assert_eq!(center, [220, 20, 30, 255]);
    assert!(edge[2] > edge[0]);
  }

  #[test]
  fn gradient_outline_is_preserved_in_effect_raster() {
    let bounds = rect(0.0, 0.0, 20.0, 10.0);
    let item = DisplayItem::Rect(RectItem {
      bounds,
      fill: Fill::None,
      stroke: Some(Stroke {
        width: Pt(2.0),
        color: Color {
          r: 255,
          g: 0,
          b: 0,
          a: 255,
        },
        gradient: Some(GradientFill {
          stops: vec![
            GradientStop {
              position: 0.0,
              color: Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
              },
              scheme: None,
            },
            GradientStop {
              position: 1.0,
              color: Color {
                r: 0,
                g: 0,
                b: 255,
                a: 255,
              },
              scheme: None,
            },
          ],
          angle_degrees: Some(0.0),
          definition_bounds: Some(bounds),
          ..GradientFill::default()
        }),
        ..Stroke::default()
      }),
    });

    let raster = rasterize_vector_items(&[item], bounds).unwrap();
    let left = raster.image.get_pixel(0, 0).0;
    let right = raster.image.get_pixel(raster.image.width() - 1, 0).0;
    assert!(left[0] > left[2], "{left:?}");
    assert!(right[2] > right[0], "{right:?}");
  }

  #[test]
  fn group_raster_exposes_children_without_reusing_them_as_group_fill() {
    let bounds = Rect {
      origin: Point {
        x: Pt(0.0),
        y: Pt(0.0),
      },
      size: Size {
        width: Pt(10.0),
        height: Pt(10.0),
      },
    };
    let item = DisplayItem::Rect(RectItem {
      bounds,
      fill: Fill::Solid(Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
      }),
      stroke: None,
    });
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: vec![
        ImageEffect::SourceReference(ImageEffectSourceReference::Fill),
        ImageEffect::SourceReference(ImageEffectSourceReference::Children),
      ],
    };

    assert!(
      rasterize_vector_items_for_effects(std::slice::from_ref(&item), bounds, &effects).is_none()
    );
    let raster = rasterize_group_items_for_effects(&[item], bounds, &effects).unwrap();
    assert_eq!(raster.fill_image.unwrap().get_pixel(5, 5).0, [0, 0, 0, 0]);
    assert_eq!(
      raster.children_image.unwrap().get_pixel(5, 5).0,
      [255, 0, 0, 255]
    );
  }

  #[test]
  fn image_raster_honors_crop_and_flip() {
    let source = RgbaImage::from_fn(2, 1, |x, _| {
      if x == 0 {
        Rgba([240, 10, 20, 255])
      } else {
        Rgba([20, 30, 240, 255])
      }
    });
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
      .write_image(source.as_raw(), 2, 1, ColorType::Rgba8.into())
      .unwrap();
    let bounds = rect(0.0, 0.0, 10.0, 10.0);
    let item = DisplayItem::Image(ImageItem {
      bounds,
      crop: Some(ImageCrop {
        left: 0.5,
        ..ImageCrop::default()
      }),
      clip_path: Vec::new(),
      rotation_degrees: 0.0,
      flip_horizontal: false,
      flip_vertical: false,
      content_type: Cow::Borrowed("image/png"),
      bytes: Arc::from(png),
      metafile_monochrome_dib_palette_override: None,
      metafile_background_color: None,
      relationship_id: None,
      alt_text: None,
      hyperlink_url: None,
      semantic_metafile_text: false,
      metafile_native_size: false,
      floating: false,
      behind_text: false,
    });
    let raster = rasterize_vector_items(&[item], bounds).unwrap();
    let center = raster
      .image
      .get_pixel(raster.image.width() / 2, raster.image.height() / 2)
      .0;
    assert!(center[2] > center[0]);
  }

  #[test]
  fn path_gradient_raster_uses_focus_path_direction() {
    let bounds = rect(0.0, 0.0, 10.0, 10.0);
    let commands = vec![
      PathCommand::MoveTo(Point {
        x: Pt(0.0),
        y: Pt(0.0),
      }),
      PathCommand::LineTo(Point {
        x: Pt(10.0),
        y: Pt(0.0),
      }),
      PathCommand::LineTo(Point {
        x: Pt(10.0),
        y: Pt(10.0),
      }),
      PathCommand::LineTo(Point {
        x: Pt(0.0),
        y: Pt(10.0),
      }),
      PathCommand::Close,
    ];
    let item = DisplayItem::Path(PathItem {
      bounds,
      points: Vec::new(),
      commands,
      closed: true,
      fill: Fill::Gradient(GradientFill {
        stops: vec![
          GradientStop {
            position: 0.0,
            color: Color {
              r: 240,
              g: 10,
              b: 20,
              a: 255,
            },
            scheme: None,
          },
          GradientStop {
            position: 1.0,
            color: Color {
              r: 20,
              g: 30,
              b: 240,
              a: 255,
            },
            scheme: None,
          },
        ],
        path: Some(GradientPath {
          kind: GradientPathKind::Rectangle,
          fill_to: RelativeRect {
            left: 0.4,
            top: 0.4,
            right: 0.4,
            bottom: 0.4,
          },
          transform: Transform {
            m11: 10.0,
            m22: 10.0,
            ..Transform::default()
          },
          mirror_tile: false,
        }),
        ..GradientFill::default()
      }),
      stroke: None,
    });
    let raster = rasterize_vector_items(&[item], bounds).unwrap();
    let edge = raster.image.get_pixel(0, raster.image.height() / 2).0;
    let center = raster
      .image
      .get_pixel(raster.image.width() / 2, raster.image.height() / 2)
      .0;
    assert!(edge[2] > edge[0]);
    assert!(center[0] > center[2]);
  }
}
