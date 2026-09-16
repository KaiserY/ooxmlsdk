use image::{
  RgbaImage,
  imageops::{FilterType, replace},
};
use kurbo::{Affine, BezPath, Point as KurboPoint, Shape as KurboShape};
use skrifa::{
  FontRef, GlyphId, MetadataProvider,
  instance::{LocationRef, Size},
  outline::{DrawSettings, HintingInstance, HintingOptions, OutlinePen, SmoothMode, Target},
  raw::TableProvider,
};
use tiny_skia::{
  Color as SkColor, FillRule, FilterQuality, GradientStop as SkGradientStop, LineCap, LineJoin,
  LinearGradient, Mask, Paint, Path, PathBuilder, PathSegment, PathStroker, Pattern, Pixmap,
  Point as SkPoint, PremultipliedColorU8, Rect as SkRect, SpreadMode, Stroke as SkStroke,
  StrokeDash, Transform as SkTransform,
};

use super::{
  Color, DisplayItem, Fill, GradientFill, ImageItem, LineItem, PathCommand, PathItem, PatternFill,
  Pt, Rect, RectItem, Stroke, StrokeAlignment, TextRun,
};
use crate::text_metrics::TextMetrics;

const MAX_EFFECT_RASTER_PIXELS: f32 = 250_000.0;
// Fixed-format effects are output surfaces, not low-cost previews. Keep their
// resource guard separate from the optional supersampling/preview budget.
const MAX_FIXED_OUTPUT_EFFECT_RASTER_PIXELS: f32 = 16_000_000.0;
const MAX_EFFECT_PIXELS_PER_POINT: f32 = 2.0;
const OFFICE_SHAPE_GLOW_MAX_REFERENCE_RADIUS_PX: f32 = 16.0;
const OFFICE_ANTIALIAS_8X4_HORIZONTAL_SAMPLES: u32 = 8;
const OFFICE_ANTIALIAS_8X4_VERTICAL_SAMPLES: u32 = 4;

/// Selects Office's fixed-output working density for a simple shape glow.
///
/// Office evaluates the authored radius against a 200-DPI reference surface,
/// then chooses the smallest integer divisor that keeps the radius at or
/// below 16 reference pixels. The divisor is applied to the configured output
/// density, so Print and Screen keep the same tier boundaries while producing
/// different bitmap densities.
pub(crate) fn office_simple_glow_pixels_per_point(output_dpi: f32, radius_pt: f32) -> f32 {
  let reference_radius_px = radius_pt.max(0.0) * crate::units::OFFICE_FIXED_OUTPUT_RASTER_DPI
    / crate::units::POINTS_PER_INCH;
  let tier_position = reference_radius_px / OFFICE_SHAPE_GLOW_MAX_REFERENCE_RADIUS_PX;
  let nearest_tier = tier_position.round();
  let integer_tolerance = f32::EPSILON * tier_position.abs().max(1.0) * 8.0;
  let divisor = if (tier_position - nearest_tier).abs() <= integer_tolerance {
    nearest_tier
  } else {
    tier_position.ceil()
  }
  .max(1.0);
  output_dpi / crate::units::POINTS_PER_INCH / divisor
}

/// Realizes Office's simple shape-shadow output range separately from its
/// working allocation. Word radius/phase controls and PowerPoint rectangle
/// radius/size/position controls share the printer guard and far sample inset.
/// PowerPoint picture reflections share this range as well: same-options
/// 0/.5/3/6pt blur and 78/156pt height controls retain all three density tiers.
pub(crate) fn office_shape_shadow_bitmap_sample_bounds(
  content_bounds: Rect,
  output_bounds: super::drawingml_image_effects::EffectOutputBounds,
  blur_radius_pt: f32,
  effect_pixels_per_point: f32,
) -> Rect {
  fn quantize_source_edge(value_pt: f32) -> f32 {
    let dot_position = f64::from(value_pt) * f64::from(crate::units::OFFICE_FIXED_OUTPUT_DPI)
      / f64::from(crate::units::POINTS_PER_INCH);
    let dots = dot_position.round();
    (dots * f64::from(crate::units::POINTS_PER_INCH)
      / f64::from(crate::units::OFFICE_FIXED_OUTPUT_DPI)) as f32
  }

  let printer_dot_pt = crate::units::POINTS_PER_INCH / crate::units::OFFICE_FIXED_OUTPUT_DPI;
  let radius_position = blur_radius_pt.max(0.0) / printer_dot_pt;
  let nearest_radius_dot = radius_position.round();
  let integer_tolerance = f32::EPSILON * radius_position.abs().max(1.0) * 8.0;
  let radius_dots = if (radius_position - nearest_radius_dot).abs() <= integer_tolerance {
    nearest_radius_dot
  } else {
    radius_position.ceil()
  };
  // The WPS shadow surface owns one complete 600-DPI guard dot beyond the
  // upward-quantized DrawingML blur radius. The exact-config 0.1/1/2/2.88/
  // 3/4/5/6/8/9/12pt radius matrix keeps this rule across all five balanced
  // pre-scale tiers. A zero-radius shadow is emitted as vectors by Word and
  // never enters this bitmap path.
  let display_radius_pt = (radius_dots + 1.0) * printer_dot_pt;
  let far_edge_inset_pt = 0.5 / effect_pixels_per_point.max(f32::EPSILON);

  // `output_bounds` already includes transform, alignment, rotation policy,
  // distance, and the authored blur radius. Remove only that continuous blur
  // to recover the moved source edges, quantize those edges on Office's printer
  // grid, then install the independently observed display radius. This keeps
  // local sample count separate from page/world phase.
  let moved_left = content_bounds.origin.x.0 + output_bounds.left_pt + blur_radius_pt;
  let moved_top = content_bounds.origin.y.0 + output_bounds.top_pt + blur_radius_pt;
  let moved_right = content_bounds.origin.x.0 + output_bounds.right_pt - blur_radius_pt;
  let moved_bottom = content_bounds.origin.y.0 + output_bounds.bottom_pt - blur_radius_pt;
  let left = quantize_source_edge(moved_left) - display_radius_pt;
  let top = quantize_source_edge(moved_top) - display_radius_pt;
  let right = quantize_source_edge(moved_right) + display_radius_pt - far_edge_inset_pt;
  let bottom = quantize_source_edge(moved_bottom) + display_radius_pt - far_edge_inset_pt;
  crate::model::common_rect(left, top, right - left, bottom - top)
}

#[derive(Debug)]
pub(crate) struct DrawingRaster {
  pub(crate) image: RgbaImage,
  pub(crate) fill_image: Option<RgbaImage>,
  pub(crate) line_image: Option<RgbaImage>,
  pub(crate) fill_line_image: Option<RgbaImage>,
  pub(crate) children_image: Option<RgbaImage>,
  pub(crate) pixels_per_point: f32,
}

impl DrawingRaster {
  /// Physical extent of an integer bitmap painted at uniform sample density.
  /// Callers using a custom `PageToRasterMapping` must retain that mapping's
  /// display transform instead; nominal density does not describe its axes.
  pub(crate) fn bounds_at_pixel_density(&self, origin: super::Point) -> Rect {
    Rect {
      origin,
      size: super::Size {
        width: Pt(self.image.width() as f32 / self.pixels_per_point),
        height: Pt(self.image.height() as f32 / self.pixels_per_point),
      },
    }
  }
}

/// Fixed-output straight, solid stroke source. Allocation may include a cap's
/// conservative range independently of the tight widened ink.
pub(crate) fn powerpoint_shadow_line<'a, 'data>(
  items: &'a [DisplayItem<'data>],
) -> Option<(super::Point, super::Point, &'a Stroke<'data>)> {
  let [item] = items else { return None };
  let (start, end, stroke) = match item {
    DisplayItem::Path(path) if !path.closed && matches!(path.fill, Fill::None) => {
      let [PathCommand::MoveTo(start), PathCommand::LineTo(end)] = path.commands.as_slice() else {
        return None;
      };
      (*start, *end, path.stroke.as_ref()?)
    }
    DisplayItem::Line(line) if line.kind == super::LineKind::Stroke => {
      (line.start, line.end, &line.stroke)
    }
    _ => return None,
  };
  if !stroke.width.0.is_finite()
    || stroke.width.0 <= 0.0
    || stroke.resolved_dash().is_some()
    || !matches!(stroke.compound, None | Some(super::StrokeCompound::Single))
    || stroke.head_end.is_some()
    || stroke.tail_end.is_some()
    || stroke.color.a == 0 && stroke.gradient.is_none() && stroke.pattern.is_none()
    || ![start.x.0, start.y.0, end.x.0, end.y.0]
      .iter()
      .all(|v| v.is_finite())
    || start == end
  {
    return None;
  }
  Some((start, end, stroke))
}

/// Orthogonal solid lines are realized on the actual balanced-blur surface,
/// not a nominal-density image subsequently resized. The one-pixel pen is a
/// device-space cosmetic stroke; wider pens retain their local geometric width.
/// This owner is distinct from the sharp-shadow and image-reflection sources.
pub(crate) fn rasterize_powerpoint_blurred_line_source(
  items: &[DisplayItem<'static>],
  output_bounds: Rect,
  offset_pt: (f32, f32),
  pixels_per_point: f32,
) -> Option<(
  DrawingRaster,
  super::drawingml_image_effects::EffectRasterScale,
)> {
  use super::drawingml_device_stroke::{DeviceStrokeTransform, TransformPrecision};
  let (start, end, stroke) = powerpoint_shadow_line(items)?;
  if start.x != end.x && start.y != end.y
    || stroke.gradient.is_some()
    || stroke.pattern.is_some()
    || !pixels_per_point.is_finite()
    || pixels_per_point <= 0.0
    || ![
      output_bounds.origin.x.0,
      output_bounds.origin.y.0,
      output_bounds.size.width.0,
      output_bounds.size.height.0,
      offset_pt.0,
      offset_pt.1,
    ]
    .iter()
    .all(|value| value.is_finite())
    || output_bounds.size.width.0 <= 0.0
    || output_bounds.size.height.0 <= 0.0
  {
    return None;
  }
  let width = raster_pixel_extent(output_bounds.size.width.0, pixels_per_point);
  let height = raster_pixel_extent(output_bounds.size.height.0, pixels_per_point);
  if u64::from(width) * u64::from(height) > MAX_FIXED_OUTPUT_EFFECT_RASTER_PIXELS as u64 {
    return None;
  }
  // Recover the independently quantized printer edges before the f32 page
  // representation loses their exact phase. The terminal half sample belongs
  // to source coverage, not the smaller PDF placement rectangle.
  let printer_edge = |value: f64| (value / 0.12).round() * 0.12;
  let left = printer_edge(f64::from(output_bounds.origin.x.0));
  let top = printer_edge(f64::from(output_bounds.origin.y.0));
  let source_width =
    printer_edge(f64::from(output_bounds.size.width.0) + 0.5 / f64::from(pixels_per_point));
  let source_height =
    printer_edge(f64::from(output_bounds.size.height.0) + 0.5 / f64::from(pixels_per_point));
  if source_width <= 0.0 || source_height <= 0.0 {
    return None;
  }
  let scale_x = f64::from(width) / source_width;
  let scale_y = f64::from(height) / source_height;
  // Native width realization rounds the completed EMU-to-device transform,
  // not a nominal-DPI scale rounded before its unit conversion.
  let authored_emu = (f64::from(stroke.width.0) * 12_700.0).round();
  let pen_transform = DeviceStrokeTransform::new(
    [scale_x / 12_700.0, 0.0, 0.0, scale_y / 12_700.0],
    TransformPrecision::Paint,
  )?;
  let local_width = pen_transform.realize_width(authored_emu, true, 1.0, false)? / 12_700.0;
  let cosmetic = (local_width * scale_x).round() == 1.0;
  let vertical = start.x == end.x;
  let normal_scale = if vertical { scale_x } else { scale_y };
  let normal_width = if cosmetic {
    1.0
  } else {
    (local_width * normal_scale) as f32
  };
  let (cap_scale_x, cap_scale_y) = if cosmetic {
    (1.0, 1.0)
  } else {
    (
      (scale_x / normal_scale) as f32,
      (scale_y / normal_scale) as f32,
    )
  };
  let mut device_stroke = stroke.clone();
  device_stroke.width = Pt(normal_width);
  let point = |p: super::Point| super::Point {
    x: Pt(((f64::from(p.x.0) + f64::from(offset_pt.0) - left) * scale_x) as f32 / cap_scale_x),
    y: Pt(((f64::from(p.y.0) + f64::from(offset_pt.1) - top) * scale_y) as f32 / cap_scale_y),
  };
  let line = DisplayItem::Line(super::LineItem {
    start: point(start),
    end: point(end),
    stroke: device_stroke,
    kind: super::LineKind::Stroke,
  });
  let mut image = RgbaImage::new(width, height);
  let band_height = (262_144 / width.max(1)).clamp(1, 64);
  for band_top in (0..height).step_by(band_height as usize) {
    let band = rasterize_vector_items_at_mapping(
      std::slice::from_ref(&line),
      PageToRasterMapping {
        width_px: width,
        height_px: band_height.min(height - band_top),
        scale_x: cap_scale_x,
        scale_y: cap_scale_y,
        translate_x: -0.5,
        translate_y: -0.5 - band_top as f32,
        text_hinting: None,
      },
      RasterPrimitiveAntialiasing::OfficeAntiAlias8x4,
    )?;
    replace(&mut image, &band, 0, i64::from(band_top));
  }
  Some((
    DrawingRaster {
      image,
      fill_image: None,
      line_image: None,
      fill_line_image: None,
      children_image: None,
      pixels_per_point,
    },
    super::drawingml_image_effects::EffectRasterScale {
      x: (scale_x / f64::from(pixels_per_point)) as f32,
      y: (scale_y / f64::from(pixels_per_point)) as f32,
    },
  ))
}

/// PowerPoint's sharp shape-shadow surface has an inclusive terminal sample,
/// independently of the smaller rectangle used to place its PDF image. Paint
/// into that source range, not an isotropic preview followed by image shifting.
pub(crate) fn rasterize_powerpoint_sharp_shadow_source(
  items: &[DisplayItem<'static>],
  output_bounds: Rect,
  offset_pt: (f32, f32),
  pixels_per_point: f32,
  snap_pen: bool,
) -> Option<DrawingRaster> {
  if !pixels_per_point.is_finite() || pixels_per_point <= 0.0 {
    return None;
  }
  let width = raster_pixel_extent(output_bounds.size.width.0, pixels_per_point);
  let height = raster_pixel_extent(output_bounds.size.height.0, pixels_per_point);
  if u64::from(width) * u64::from(height) > MAX_FIXED_OUTPUT_EFFECT_RASTER_PIXELS as u64 {
    return None;
  }
  let scale_x = width as f32 / (output_bounds.size.width.0 + 0.5 / pixels_per_point);
  let scale_y = height as f32 / (output_bounds.size.height.0 + 0.5 / pixels_per_point);
  let mut realized_items = items.to_vec();
  fn realize(items: &mut [DisplayItem<'static>], ppp: f32, snap: bool) -> Option<()> {
    use super::drawingml_device_stroke::{DeviceStrokeTransform, TransformPrecision};
    let transform = DeviceStrokeTransform::new(
      [f64::from(ppp), 0.0, 0.0, f64::from(ppp)],
      TransformPrecision::Paint,
    )?;
    for item in items {
      let stroke = match item {
        DisplayItem::Path(path) => path.stroke.as_mut(),
        DisplayItem::Rect(rect) => rect.stroke.as_mut(),
        DisplayItem::Line(line) => Some(&mut line.stroke),
        DisplayItem::Group(group) => {
          realize(&mut group.items, ppp, snap)?;
          None
        }
        _ => None,
      };
      if let Some(stroke) = stroke {
        stroke.width.0 =
          transform.realize_width(f64::from(stroke.width.0), snap, 1.0, false)? as f32;
      }
    }
    Some(())
  }
  realize(&mut realized_items, pixels_per_point, snap_pen)?;
  let mut image = RgbaImage::new(width, height);
  let band_height = (262_144 / width.max(1)).clamp(1, 64);
  for top in (0..height).step_by(band_height as usize) {
    let band = rasterize_vector_items_at_mapping(
      &realized_items,
      PageToRasterMapping {
        width_px: width,
        height_px: band_height.min(height - top),
        scale_x,
        scale_y,
        // PixelOffsetModeHalf uses [0..7]/8 and [0..3]/4. Compensate
        // the shared GDI+ integer-centered sample storage, as for text.
        translate_x: -(output_bounds.origin.x.0 - offset_pt.0) * scale_x - 0.5,
        translate_y: -(output_bounds.origin.y.0 - offset_pt.1) * scale_y - 0.5 - top as f32,
        text_hinting: None,
      },
      RasterPrimitiveAntialiasing::OfficeAntiAlias8x4,
    )?;
    replace(&mut image, &band, 0, i64::from(top));
  }
  Some(DrawingRaster {
    image,
    fill_image: None,
    line_image: None,
    fill_line_image: None,
    children_image: None,
    pixels_per_point,
  })
}

/// Explicit page-to-device mapping for a fixed-output raster surface.
///
/// Most DrawingML effect surfaces use one isotropic page-space density and a
/// page-space bounding rectangle. Word's on-screen static-3-D surface and its
/// legacy locked canvas are exceptions: the former is tied to a separately
/// quantized PDF rectangle, while the latter fits an independently measured
/// source range to a fixed bitmap extent and can therefore have unequal axis
/// scales. Keep that mapping explicit instead of changing the authored
/// page-space display list.
#[derive(Clone, Copy, Debug)]
pub(crate) struct PageToRasterMapping {
  pub(crate) width_px: u32,
  pub(crate) height_px: u32,
  pub(crate) scale_x: f32,
  pub(crate) scale_y: f32,
  pub(crate) translate_x: f32,
  pub(crate) translate_y: f32,
  pub(crate) text_hinting: Option<(RasterTextHinting, f32)>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RasterTextHinting {
  RoundedDevicePpem,
  ExactDevicePpem,
  RoundedDevicePpemPreserveLinear,
  ExactDevicePpemPreserveLinear,
  ExactDevicePpemAsymmetric,
  ExactDevicePpemLight,
  ExactDevicePpemLcd,
  ExactDevicePpemMono,
  GdiDeviceAdvances,
  GdiDeviceAdvancesHinted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RasterResolveFilter {
  Native,
  Nearest,
  Triangle,
  CatmullRom,
  Lanczos3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RasterSourceExtent {
  Outward,
  InclusiveFarEdge,
  Floor,
  Round,
}

/// Selects how non-text vector primitives contribute coverage to an effect
/// source bitmap.
///
/// Direct2D keeps primitive and text antialiasing as independent state.  The
/// aliased mode therefore applies to paths, rectangles, lines, markers, and
/// image clip geometry, while glyph outlines retain their text-specific
/// rasterization policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RasterPrimitiveAntialiasing {
  PerPrimitive,
  Aliased,
  /// Resolve the standard four-sample lattice independently for each path.
  /// Text retains its separate rasterization policy.
  Direct2dStandard4,
  /// GDI+ `SmoothingModeAntiAlias8x4`: sample an aliased 8x4 source grid and
  /// average its 32 samples with a premultiplied box resolve.
  OfficeAntiAlias8x4,
}

/// Host policy for realizing a standalone WordprocessingShape before a
/// fixed-output effect is evaluated.
///
/// Word's glow controls explicitly pin primitive coverage to pixel centers,
/// while an outer shadow consumes Direct2D's default per-primitive coverage.
/// Keep those independently observed source graphs separate even though both
/// later pass through the balanced-blur surface tiers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WordShapeEffectSourceProfile {
  Glow,
  OuterShadow,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct RasterSourceSurface {
  pub(crate) bounds: Rect,
  pub(crate) dpi: f64,
  pub(crate) extent: RasterSourceExtent,
  pub(crate) text_hinting: Option<RasterTextHinting>,
  pub(crate) primitive_antialiasing: RasterPrimitiveAntialiasing,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct RasterTargetSurface {
  pub(crate) width_px: u32,
  pub(crate) height_px: u32,
  pub(crate) filter: RasterResolveFilter,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct WordShapeEffectSurface {
  pub(crate) profile: WordShapeEffectSourceProfile,
  pub(crate) base_bounds: Rect,
  pub(crate) content_bounds: Rect,
  pub(crate) base_pixels_per_point: f32,
  pub(crate) target_width_px: u32,
  pub(crate) target_height_px: u32,
  pub(crate) target_pixels_per_point: f32,
}

impl RasterPrimitiveAntialiasing {
  fn enabled(self) -> bool {
    matches!(self, Self::PerPrimitive | Self::Direct2dStandard4)
  }

  fn sample_factors(self) -> (u32, u32) {
    match self {
      Self::OfficeAntiAlias8x4 => (
        OFFICE_ANTIALIAS_8X4_HORIZONTAL_SAMPLES,
        OFFICE_ANTIALIAS_8X4_VERTICAL_SAMPLES,
      ),
      Self::PerPrimitive | Self::Aliased | Self::Direct2dStandard4 => (1, 1),
    }
  }
}

impl RasterResolveFilter {
  fn image_filter(self) -> Option<FilterType> {
    Some(match self {
      Self::Native => return None,
      Self::Nearest => FilterType::Nearest,
      Self::Triangle => FilterType::Triangle,
      Self::CatmullRom => FilterType::CatmullRom,
      Self::Lanczos3 => FilterType::Lanczos3,
    })
  }
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
  rasterize_vector_items_for_effects_at_pixels_per_point_with_extent(
    items,
    raster_bounds,
    effects,
    pixels_per_point,
    RasterSourceExtent::Outward,
  )
}

pub(crate) fn rasterize_vector_items_for_effects_at_pixels_per_point_with_extent(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  pixels_per_point: f32,
  extent: RasterSourceExtent,
) -> Option<DrawingRaster> {
  rasterize_vector_items_for_effects_at_pixels_per_point_with_extent_and_antialiasing(
    items,
    raster_bounds,
    effects,
    pixels_per_point,
    extent,
    RasterPrimitiveAntialiasing::PerPrimitive,
  )
}

pub(crate) fn rasterize_vector_items_for_effects_at_pixels_per_point_with_extent_and_antialiasing(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  pixels_per_point: f32,
  extent: RasterSourceExtent,
  primitive_antialiasing: RasterPrimitiveAntialiasing,
) -> Option<DrawingRaster> {
  rasterize_vector_items_for_effects_impl_at_pixels_per_point_with_antialiasing(
    items,
    raster_bounds,
    effects,
    false,
    pixels_per_point,
    extent,
    primitive_antialiasing,
  )
}

pub(crate) fn supports_powerpoint_blurred_vector_source(items: &[DisplayItem<'static>]) -> bool {
  !items.is_empty()
    && items.iter().all(|item| {
      matches!(
        item,
        DisplayItem::Path(_) | DisplayItem::Rect(_) | DisplayItem::Line(_)
      )
    })
}

/// A blurred vector shape is sampled on its actual effect-device grid. Unlike
/// the orthogonal line owner, wider geometric pens retain their authored width.
pub(crate) fn rasterize_powerpoint_blurred_vector_source(
  items: &[DisplayItem<'static>],
  output_bounds: Rect,
  offset_pt: (f32, f32),
  pixels_per_point: f32,
) -> Option<(
  DrawingRaster,
  super::drawingml_image_effects::EffectRasterScale,
)> {
  if !supports_powerpoint_blurred_vector_source(items)
    || !pixels_per_point.is_finite()
    || pixels_per_point <= 0.0
    || !output_bounds.size.width.0.is_finite()
    || !output_bounds.size.height.0.is_finite()
    || output_bounds.size.width.0 <= 0.0
    || output_bounds.size.height.0 <= 0.0
    || ![
      output_bounds.origin.x.0,
      output_bounds.origin.y.0,
      offset_pt.0,
      offset_pt.1,
    ]
    .iter()
    .all(|value| value.is_finite())
  {
    return None;
  }
  let width = raster_pixel_extent(output_bounds.size.width.0, pixels_per_point);
  let height = raster_pixel_extent(output_bounds.size.height.0, pixels_per_point);
  if u64::from(width) * u64::from(height) > MAX_FIXED_OUTPUT_EFFECT_RASTER_PIXELS as u64 {
    return None;
  }
  let scale_x = width as f32 / (output_bounds.size.width.0 + 0.5 / pixels_per_point);
  let scale_y = height as f32 / (output_bounds.size.height.0 + 0.5 / pixels_per_point);
  let mut realized = items.to_vec();
  fn realize_minimum_width(items: &mut [DisplayItem<'static>], width: f32) {
    for item in items {
      let stroke = match item {
        DisplayItem::Path(path) => path.stroke.as_mut(),
        DisplayItem::Rect(rect) => rect.stroke.as_mut(),
        DisplayItem::Line(line) => Some(&mut line.stroke),
        DisplayItem::Group(group) => {
          realize_minimum_width(&mut group.items, width);
          None
        }
        _ => None,
      };
      if let Some(stroke) = stroke
        && stroke.width.0.is_finite()
        && stroke.width.0 >= 0.0
        && stroke.width.0 < width
      {
        stroke.width.0 = width;
      }
    }
  }
  realize_minimum_width(&mut realized, 1.0 / scale_x);
  let device = SkTransform::from_row(
    scale_x,
    0.0,
    0.0,
    scale_y,
    -(output_bounds.origin.x.0 - offset_pt.0) * scale_x,
    -(output_bounds.origin.y.0 - offset_pt.1) * scale_y,
  );
  let inverse = device.invert()?;
  let mut paint_calls = Vec::new();
  for item in realized {
    match item {
      DisplayItem::Path(mut path) => {
        let transformed =
          path_from_commands(&path.commands, &path.points, path.closed)?.transform(device)?;
        let flattened =
          super::drawingml_mil_raster::flatten_device_path_28_4(&tiny_path_as_kurbo(&transformed))?;
        path.commands = flattened
          .elements()
          .iter()
          .map(|element| {
            let point = |point: kurbo::Point| {
              let mut p = SkPoint::from_xy(point.x as f32, point.y as f32);
              inverse.map_point(&mut p);
              super::Point {
                x: Pt(p.x),
                y: Pt(p.y),
              }
            };
            match *element {
              kurbo::PathEl::MoveTo(p) => PathCommand::MoveTo(point(p)),
              kurbo::PathEl::LineTo(p) => PathCommand::LineTo(point(p)),
              kurbo::PathEl::ClosePath => PathCommand::Close,
              _ => unreachable!("flattened paths have no curves"),
            }
          })
          .collect();
        let stroke = path.stroke.take();
        if !matches!(path.fill, Fill::None) {
          paint_calls.push(DisplayItem::Path(path.clone()));
        }
        if let Some(stroke) = stroke {
          path.fill = Fill::None;
          path.stroke = Some(stroke);
          paint_calls.push(DisplayItem::Path(path));
        }
      }
      DisplayItem::Rect(mut rect) => {
        let stroke = rect.stroke.take();
        if !matches!(rect.fill, Fill::None) {
          paint_calls.push(DisplayItem::Rect(rect.clone()));
        }
        if let Some(stroke) = stroke {
          rect.fill = Fill::None;
          rect.stroke = Some(stroke);
          paint_calls.push(DisplayItem::Rect(rect));
        }
      }
      DisplayItem::Line(line) => paint_calls.push(DisplayItem::Line(line)),
      _ => return None,
    }
  }
  let mut image = RgbaImage::new(width, height);
  let band_height = (262_144 / width.max(1)).clamp(1, 64);
  for top in (0..height).step_by(band_height as usize) {
    let mapping = PageToRasterMapping {
      width_px: width,
      height_px: band_height.min(height - top),
      scale_x,
      scale_y,
      translate_x: -(output_bounds.origin.x.0 - offset_pt.0) * scale_x - 0.5,
      translate_y: -(output_bounds.origin.y.0 - offset_pt.1) * scale_y - 0.5 - top as f32,
      text_hinting: None,
    };
    let mut band = RgbaImage::new(width, mapping.height_px);
    for call in &paint_calls {
      let paint = rasterize_vector_items_at_mapping(
        std::slice::from_ref(call),
        mapping,
        RasterPrimitiveAntialiasing::OfficeAntiAlias8x4,
      )?;
      image::imageops::overlay(&mut band, &paint, 0, 0);
    }
    replace(&mut image, &band, 0, i64::from(top));
  }
  Some((
    DrawingRaster {
      image,
      fill_image: None,
      line_image: None,
      fill_line_image: None,
      children_image: None,
      pixels_per_point,
    },
    super::drawingml_image_effects::EffectRasterScale {
      x: scale_x / pixels_per_point,
      y: scale_y / pixels_per_point,
    },
  ))
}

/// Realize a transformed backdrop directly in its output coordinate system.
/// Keep native filtering to one source lookup rather than transforming an
/// already sampled canvas. Blur-tier resolution follows the configured source;
/// the allocated output dimensions own the physical mapping on each axis.
pub(crate) fn rasterize_powerpoint_transformed_backdrop_source(
  items: &[DisplayItem<'static>],
  output_bounds: Rect,
  transform: super::Transform,
  source_pixels_per_point: f32,
  target_pixels_per_point: f32,
) -> Option<(
  DrawingRaster,
  super::drawingml_image_effects::EffectRasterScale,
)> {
  if items.iter().any(|item| !supported_raster_item(item))
    || !source_pixels_per_point.is_finite()
    || !target_pixels_per_point.is_finite()
    || source_pixels_per_point <= 0.0
    || target_pixels_per_point <= 0.0
    || target_pixels_per_point > source_pixels_per_point
    || !output_bounds.size.width.0.is_finite()
    || !output_bounds.size.height.0.is_finite()
    || output_bounds.size.width.0 <= 0.0
    || output_bounds.size.height.0 <= 0.0
  {
    return None;
  }
  let target_width = raster_pixel_extent(output_bounds.size.width.0, target_pixels_per_point);
  let target_height = raster_pixel_extent(output_bounds.size.height.0, target_pixels_per_point);
  let divisor = source_pixels_per_point / target_pixels_per_point;
  let width = (target_width as f32 * divisor).ceil() as u32;
  let height = (target_height as f32 * divisor).ceil() as u32;
  if u64::from(width) * u64::from(height) > MAX_FIXED_OUTPUT_EFFECT_RASTER_PIXELS as u64 {
    return None;
  }
  let mut pixmap = Pixmap::new(width, height)?;
  // As with sharp shadows, the physical source window retains the terminal
  // half sample removed from the PDF placement rectangle. The allocated
  // image dimensions determine each axis scale independently.
  let source_width_pt = output_bounds.size.width.0 + 0.5 / target_pixels_per_point;
  let source_height_pt = output_bounds.size.height.0 + 0.5 / target_pixels_per_point;
  let scale_x = target_width as f32 * divisor / source_width_pt;
  let scale_y = target_height as f32 * divisor / source_height_pt;
  let mapping = SkTransform::from_row(
    transform.m11 * scale_x,
    transform.m12 * scale_y,
    transform.m21 * scale_x,
    transform.m22 * scale_y,
    (transform.dx.0 - output_bounds.origin.x.0) * scale_x,
    (transform.dy.0 - output_bounds.origin.y.0) * scale_y,
  );
  let mut metrics = TextMetrics::new();
  for item in items {
    draw_display_item(
      &mut pixmap,
      item,
      mapping,
      None,
      RasterPrimitiveAntialiasing::PerPrimitive,
      &mut metrics,
    )?;
  }
  let source = pixmap_into_rgba(pixmap)?;
  let image = if source_pixels_per_point == target_pixels_per_point {
    source
  } else {
    super::drawingml_image_effects::dpi_compensate_linear_hard(
      &source,
      source_pixels_per_point,
      target_pixels_per_point,
      target_width,
      target_height,
    )?
  };
  Some((
    DrawingRaster {
      image,
      fill_image: None,
      line_image: None,
      fill_line_image: None,
      children_image: None,
      pixels_per_point: target_pixels_per_point,
    },
    super::drawingml_image_effects::EffectRasterScale {
      x: scale_x / source_pixels_per_point,
      y: scale_y / source_pixels_per_point,
    },
  ))
}

/// Realizes a vector effect input at one fixed-output density and resolves it
/// onto the lower-density effect surface through Direct2D-style DPI
/// compensation.
///
/// This is distinct from rasterizing the display list directly at the target
/// density: fractional vector coverage is first quantized on the source
/// surface, then the associated-alpha pixels are linearly sampled at physical
/// pixel centers. Every separately addressable effect source is resolved by
/// the same mapping so fill/line references cannot drift from the root input.
pub(crate) fn rasterize_vector_items_for_effects_via_dpi_compensated_source_surface(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  source_pixels_per_point: f32,
  target_pixels_per_point: f32,
  extent: RasterSourceExtent,
  primitive_antialiasing: RasterPrimitiveAntialiasing,
) -> Option<DrawingRaster> {
  if !target_pixels_per_point.is_finite() || target_pixels_per_point <= 0.0 {
    return None;
  }
  let target_width_px =
    raster_source_extent(raster_bounds.size.width.0, target_pixels_per_point, extent);
  let target_height_px =
    raster_source_extent(raster_bounds.size.height.0, target_pixels_per_point, extent);
  let raster = rasterize_vector_items_for_effects_at_pixels_per_point_with_extent_and_antialiasing(
    items,
    raster_bounds,
    effects,
    source_pixels_per_point,
    extent,
    primitive_antialiasing,
  )?;
  let resolve = |image: RgbaImage| {
    if (source_pixels_per_point - target_pixels_per_point).abs() <= f32::EPSILON
      && image.dimensions() == (target_width_px, target_height_px)
    {
      Some(image)
    } else {
      super::drawingml_image_effects::dpi_compensate_linear_hard(
        &image,
        source_pixels_per_point,
        target_pixels_per_point,
        target_width_px,
        target_height_px,
      )
    }
  };
  let resolve_optional = |image: Option<RgbaImage>| match image {
    Some(image) => Some(Some(resolve(image)?)),
    None => Some(None),
  };

  Some(DrawingRaster {
    image: resolve(raster.image)?,
    fill_image: resolve_optional(raster.fill_image)?,
    line_image: resolve_optional(raster.line_image)?,
    fill_line_image: resolve_optional(raster.fill_line_image)?,
    children_image: resolve_optional(raster.children_image)?,
    pixels_per_point: target_pixels_per_point,
  })
}

fn rasterize_vector_items_for_effects_impl_at_pixels_per_point_with_antialiasing(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  items_are_children_source: bool,
  pixels_per_point: f32,
  extent: RasterSourceExtent,
  primitive_antialiasing: RasterPrimitiveAntialiasing,
) -> Option<DrawingRaster> {
  let requirements = super::drawingml_image_effects::source_requirements(effects);
  // A logical children source can only be produced while retaining the host
  // group's child display list. A leaf-shape raster must fail strictly here so
  // callers keep the original vector content rather than substituting an
  // empty source.
  if requirements.children && !items_are_children_source {
    return None;
  }
  let (image, pixels_per_point) =
    rasterize_vector_items_impl_at_pixels_per_point_with_extent_and_antialiasing(
      items,
      raster_bounds,
      pixels_per_point,
      extent,
      primitive_antialiasing,
    )?;
  let fill_image = if requirements.fill && items_are_children_source {
    Some(empty_raster_at_pixels_per_point(raster_bounds, pixels_per_point, extent)?.0)
  } else if requirements.fill {
    Some(rasterize_source_layer_at_pixels_per_point(
      items,
      raster_bounds,
      SourceLayer::Fill,
      pixels_per_point,
      extent,
      primitive_antialiasing,
    )?)
    .map(|layer| layer.0)
  } else {
    None
  };
  let line_image = if requirements.line && items_are_children_source {
    Some(empty_raster_at_pixels_per_point(raster_bounds, pixels_per_point, extent)?.0)
  } else if requirements.line {
    Some(rasterize_source_layer_at_pixels_per_point(
      items,
      raster_bounds,
      SourceLayer::Line,
      pixels_per_point,
      extent,
      primitive_antialiasing,
    )?)
    .map(|layer| layer.0)
  } else {
    None
  };
  Some(DrawingRaster {
    children_image: (requirements.children && items_are_children_source).then(|| image.clone()),
    fill_line_image: if requirements.fill_line && items_are_children_source {
      Some(empty_raster_at_pixels_per_point(raster_bounds, pixels_per_point, extent)?.0)
    } else {
      None
    },
    image,
    fill_image,
    line_image,
    pixels_per_point,
  })
}

pub(crate) fn rasterize_vector_items_for_effects_with_mapping(
  items: &[DisplayItem<'static>],
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  pixels_per_point: f32,
  mapping: PageToRasterMapping,
) -> Option<DrawingRaster> {
  let requirements = super::drawingml_image_effects::source_requirements(effects);
  if requirements.children {
    return None;
  }
  let image = rasterize_vector_items_impl_with_mapping(items, mapping)?;
  let layer = |source_layer| {
    let mut layer_items = Vec::new();
    for item in items {
      collect_source_layer_item(item, source_layer, &mut layer_items)?;
    }
    if layer_items.is_empty() {
      Some(RgbaImage::new(mapping.width_px, mapping.height_px))
    } else {
      rasterize_vector_items_impl_with_mapping(&layer_items, mapping)
    }
  };
  Some(DrawingRaster {
    image,
    fill_image: requirements
      .fill
      .then(|| layer(SourceLayer::Fill))
      .flatten(),
    line_image: requirements
      .line
      .then(|| layer(SourceLayer::Line))
      .flatten(),
    fill_line_image: None,
    children_image: None,
    pixels_per_point,
  })
}

pub(crate) fn rasterize_vector_items_for_effects_via_source_surface(
  items: &[DisplayItem<'static>],
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  source_surface: RasterSourceSurface,
  target_surface: RasterTargetSurface,
) -> Option<DrawingRaster> {
  let RasterSourceSurface {
    bounds: source_bounds,
    dpi: source_dpi,
    extent,
    text_hinting,
    primitive_antialiasing,
  } = source_surface;
  let source_pixels_per_point = (source_dpi / 72.0) as f32;
  let RasterTargetSurface {
    width_px: target_width_px,
    height_px: target_height_px,
    filter,
  } = target_surface;
  let requirements = super::drawingml_image_effects::source_requirements(effects);
  if requirements.fill || requirements.line || requirements.children {
    return None;
  }
  if source_bounds.size.width.0 <= 0.0
    || source_bounds.size.height.0 <= 0.0
    || !source_pixels_per_point.is_finite()
    || source_pixels_per_point <= 0.0
    || target_width_px == 0
    || target_height_px == 0
    || items.iter().any(|item| !supported_raster_item(item))
  {
    return None;
  }

  let source_extent = |length_pt: f32| {
    let length_px = length_pt * source_pixels_per_point;
    match extent {
      RasterSourceExtent::Outward => raster_pixel_extent(length_pt, source_pixels_per_point),
      RasterSourceExtent::InclusiveFarEdge => {
        inclusive_far_edge_raster_pixel_extent(length_pt, source_pixels_per_point)
      }
      RasterSourceExtent::Floor => length_px.floor().max(1.0) as u32,
      RasterSourceExtent::Round => length_px.round().max(1.0) as u32,
    }
  };
  let source_width_px = source_extent(source_bounds.size.width.0);
  let source_height_px = source_extent(source_bounds.size.height.0);
  // Reject an unbounded source before allocating. Malformed/zero group extents
  // can leave an unfitted coordinate range here; the fixed-output owner can
  // still render it with its bounded viewport mapping. Never silently lower
  // the requested source density or label an upscaled image as native output.
  const MAX_NATIVE_SOURCE_PIXELS: u64 = 16_000_000;
  if u64::from(source_width_px) * u64::from(source_height_px) > MAX_NATIVE_SOURCE_PIXELS {
    return None;
  }
  let mut realized_items;
  let items = if primitive_antialiasing == RasterPrimitiveAntialiasing::Direct2dStandard4 {
    realized_items = items.to_vec();
    realize_source_device_strokes(&mut realized_items, source_dpi)?;
    realized_items.as_slice()
  } else {
    items
  };
  let mut source = rasterize_vector_items_at_mapping(
    items,
    PageToRasterMapping {
      width_px: source_width_px,
      height_px: source_height_px,
      scale_x: source_pixels_per_point,
      scale_y: source_pixels_per_point,
      translate_x: -source_bounds.origin.x.0 * source_pixels_per_point,
      translate_y: -source_bounds.origin.y.0 * source_pixels_per_point,
      text_hinting: text_hinting.map(|mode| (mode, source_pixels_per_point)),
    },
    primitive_antialiasing,
  )?;
  if std::env::var("OOXMLSDK_LOCKED_CANVAS_SOURCE_ALPHA_PROBE").as_deref() == Ok("wpf-gamma") {
    apply_wpf_grayscale_alpha_correction(&mut source);
  }
  if let Ok(path) = std::env::var("OOXMLSDK_LOCKED_CANVAS_SOURCE_DUMP_PROBE") {
    let _ = source.save(path);
  }
  Some(DrawingRaster {
    image: if let Some(filter) = filter.image_filter() {
      resize_premultiplied_rgba(&source, target_width_px, target_height_px, filter)
    } else {
      source
    },
    fill_image: None,
    line_image: None,
    fill_line_image: None,
    children_image: None,
    pixels_per_point: source_pixels_per_point,
  })
}

fn realize_source_device_strokes(items: &mut [DisplayItem<'static>], dpi: f64) -> Option<()> {
  use super::drawingml_device_stroke::TransformPrecision;
  for item in items {
    let stroke = match item {
      DisplayItem::Path(path) => path.stroke.as_mut(),
      DisplayItem::Rect(rect) => rect.stroke.as_mut(),
      DisplayItem::Line(line) => Some(&mut line.stroke),
      DisplayItem::Group(group) => {
        realize_source_device_strokes(&mut group.items, dpi)?;
        None
      }
      _ => None,
    };
    if let Some(source) = stroke.and_then(|stroke| stroke.drawingml_device.as_mut()) {
      source.realize(dpi, TransformPrecision::Paint)?;
    }
  }
  Some(())
}

/// A Word shape shadow realizes an existing pen at no less than one pixel of
/// the effect's output device, even when its source is painted at a higher
/// density. Same-options Office controls at 96/48/24 DPI distinguish this
/// minimum from both the base-surface pixel and an absent (`noFill`) pen.
/// This does not implement the separate rounding of wider device pens.
fn realize_word_shadow_minimum_strokes(items: &mut [DisplayItem<'static>], pixels_per_point: f32) {
  let minimum_width = pixels_per_point.recip();
  for item in items {
    let stroke = match item {
      DisplayItem::Path(path) => path.stroke.as_mut(),
      DisplayItem::Rect(rect) => rect.stroke.as_mut(),
      DisplayItem::Line(line) => Some(&mut line.stroke),
      DisplayItem::Group(group) => {
        // This rasterizer accepts only simple groups, with geometry already
        // in page coordinates; transformed groups are rejected before drawing.
        realize_word_shadow_minimum_strokes(&mut group.items, pixels_per_point);
        None
      }
      _ => None,
    };
    if let Some(stroke) = stroke
      && stroke.width.0.is_finite()
      && stroke.width.0 >= 0.0
    {
      stroke.width.0 = stroke.width.0.max(minimum_width);
    }
  }
}

/// Rasterizes a standalone WordprocessingShape backdrop from its
/// base-resolution source surface before resolving to the balanced-blur
/// surface tier.
///
/// Word first realizes the shape at the fixed-output base density. Its
/// balanced Gaussian implementation then pre-scales that source when the
/// selected effect tier is lower. Keeping these two surfaces distinct is
/// observable at every fractional source edge and at the 200-to-100-DPI tier
/// boundary.
pub(crate) fn rasterize_word_shape_effect_source_via_base_surface(
  items: &[DisplayItem<'static>],
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  surface: WordShapeEffectSurface,
) -> Option<DrawingRaster> {
  let WordShapeEffectSurface {
    profile,
    base_bounds: base_surface_bounds,
    content_bounds,
    base_pixels_per_point,
    target_width_px,
    target_height_px,
    target_pixels_per_point,
  } = surface;
  let requirements = super::drawingml_image_effects::source_requirements(effects);
  if requirements != super::drawingml_image_effects::ImageEffectSourceRequirements::default()
    || target_width_px == 0
    || target_height_px == 0
    || !target_pixels_per_point.is_finite()
    || target_pixels_per_point <= 0.0
  {
    return None;
  }

  let primitive_antialiasing = match profile {
    WordShapeEffectSourceProfile::Glow => RasterPrimitiveAntialiasing::Aliased,
    WordShapeEffectSourceProfile::OuterShadow => RasterPrimitiveAntialiasing::PerPrimitive,
  };
  let mut source_items = std::borrow::Cow::Borrowed(items);
  if profile == WordShapeEffectSourceProfile::OuterShadow {
    realize_word_shadow_minimum_strokes(source_items.to_mut(), target_pixels_per_point);
  }
  let (mut source, _) =
    rasterize_vector_items_impl_at_pixels_per_point_with_extent_and_antialiasing(
      &source_items,
      base_surface_bounds,
      base_pixels_per_point,
      RasterSourceExtent::InclusiveFarEdge,
      primitive_antialiasing,
    )?;
  if profile == WordShapeEffectSourceProfile::Glow {
    normalize_word_shape_uniform_rect_source(
      &mut source,
      base_surface_bounds,
      content_bounds,
      base_pixels_per_point,
    );
  }
  let image = if source.dimensions() == (target_width_px, target_height_px) {
    source
  } else {
    // Direct2D's official balanced Gaussian contract uses trilinear filtering
    // for its internal pre-scale. `image::FilterType::Triangle` is not that
    // filter: on minification it widens its reconstruction support by the
    // scale ratio. Use the four neighboring texels at mapped pixel centers,
    // matching Direct2D's linear sampling stage while preserving associated
    // alpha through the interpolation. The source was allocated with an
    // inclusive far edge, so its terminal sample owns half a pixel rather than
    // a complete interval. The independent WPS shadow phase controls select
    // this same `extent - 0.5` mapping on both axes.
    resize_inclusive_far_edge_premultiplied_linear(&source, target_width_px, target_height_px)
  };
  Some(DrawingRaster {
    image,
    fill_image: None,
    line_image: None,
    fill_line_image: None,
    children_image: None,
    pixels_per_point: target_pixels_per_point,
  })
}

fn resize_inclusive_far_edge_premultiplied_linear(
  source: &RgbaImage,
  width: u32,
  height: u32,
) -> RgbaImage {
  let scale_x = (source.width() as f32 - 0.5).max(0.0) / width as f32;
  let scale_y = (source.height() as f32 - 0.5).max(0.0) / height as f32;
  RgbaImage::from_fn(width, height, |x, y| {
    let source_x = (x as f32 + 0.5).mul_add(scale_x, -0.5);
    let source_y = (y as f32 + 0.5).mul_add(scale_y, -0.5);
    bilinear_sample(source, source_x, source_y)
  })
}

fn normalize_word_shape_uniform_rect_source(
  source: &mut RgbaImage,
  source_bounds: Rect,
  content_bounds: Rect,
  pixels_per_point: f32,
) {
  let Some(color) = uniform_nontransparent_rectangle_color(source) else {
    return;
  };
  let near_sample = |value: f32| {
    let boundary = f64::from(value) - 0.5;
    let nearest = boundary.round();
    let tolerance = f64::from(f32::EPSILON) * boundary.abs().max(1.0) * 16.0;
    if (boundary - nearest).abs() <= tolerance {
      nearest as i64
    } else {
      boundary.ceil() as i64
    }
  };
  let left = near_sample((content_bounds.origin.x.0 - source_bounds.origin.x.0) * pixels_per_point);
  let top = near_sample((content_bounds.origin.y.0 - source_bounds.origin.y.0) * pixels_per_point);
  let width = (content_bounds.size.width.0 * pixels_per_point)
    .round()
    .max(1.0) as i64;
  let height = (content_bounds.size.height.0 * pixels_per_point)
    .round()
    .max(1.0) as i64;
  let right = left.saturating_add(width);
  let bottom = top.saturating_add(height);
  for (x, y, pixel) in source.enumerate_pixels_mut() {
    *pixel = if i64::from(x) >= left
      && i64::from(x) < right
      && i64::from(y) >= top
      && i64::from(y) < bottom
    {
      color
    } else {
      image::Rgba([0; 4])
    };
  }
}

fn uniform_nontransparent_rectangle_color(source: &RgbaImage) -> Option<image::Rgba<u8>> {
  let mut left = source.width();
  let mut top = source.height();
  let mut right = 0;
  let mut bottom = 0;
  let mut color = None;
  for (x, y, pixel) in source.enumerate_pixels() {
    if pixel[3] == 0 {
      continue;
    }
    if color.is_some_and(|color| color != *pixel) {
      return None;
    }
    color = Some(*pixel);
    left = left.min(x);
    top = top.min(y);
    right = right.max(x);
    bottom = bottom.max(y);
  }
  let color = color?;
  for (x, y, pixel) in source.enumerate_pixels() {
    let inside = x >= left && x <= right && y >= top && y <= bottom;
    if inside != (*pixel == color) {
      return None;
    }
  }
  Some(color)
}

fn apply_wpf_grayscale_alpha_correction(image: &mut RgbaImage) {
  // WPF Gamma.cpp uses its hard-coded gamma 2.2 polynomial table for
  // software grayscale glyph painting. Keep this temporary whole-surface
  // probe beside the source-surface experiment; the retained implementation
  // will apply the table only to independently rendered glyph primitives.
  const G1: f32 = 0.2031;
  const G2: f32 = -1.3864;
  const G3: f32 = 1.9851;
  const G4: f32 = -0.3501;
  for pixel in image.pixels_mut() {
    let alpha = pixel[3];
    if matches!(alpha, 0 | 255) {
      continue;
    }
    let a = f32::from(alpha) / 255.0;
    let f1 = a + a * (1.0 - a) * (G2 * a + G4);
    let f2 = a * (1.0 - a) * (G1 * a + G3);
    let table_f1 = (f1 * 255.0).round().clamp(0.0, 255.0) as u16;
    let table_f2 = (f2 * 255.0).round().clamp(0.0, 255.0) as u16;
    let luminance = (u16::from(pixel[0]) + u16::from(pixel[1]) * 2 + u16::from(pixel[2])) >> 2;
    pixel[3] = (table_f1 + ((table_f2 * luminance) >> 8)).min(255) as u8;
  }
}

fn resize_premultiplied_rgba(
  source: &RgbaImage,
  width: u32,
  height: u32,
  filter: FilterType,
) -> RgbaImage {
  let associated = RgbaImage::from_fn(source.width(), source.height(), |x, y| {
    let pixel = source.get_pixel(x, y);
    let alpha = u16::from(pixel[3]);
    image::Rgba([
      ((u16::from(pixel[0]) * alpha + 127) / 255) as u8,
      ((u16::from(pixel[1]) * alpha + 127) / 255) as u8,
      ((u16::from(pixel[2]) * alpha + 127) / 255) as u8,
      pixel[3],
    ])
  });
  let mut resized = image::imageops::resize(&associated, width, height, filter);
  for pixel in resized.pixels_mut() {
    let alpha = u16::from(pixel[3]);
    if alpha == 0 {
      pixel.0[..3].fill(0);
      continue;
    }
    for channel in &mut pixel.0[..3] {
      *channel = ((u16::from(*channel) * 255 + alpha / 2) / alpha).min(255) as u8;
    }
  }
  resized
}

pub(crate) fn rasterize_vector_items_for_effects_at_bounded_pixels_per_point(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  max_pixels_per_point: f32,
) -> Option<DrawingRaster> {
  rasterize_vector_items_for_effects_at_bounded_pixels_per_point_with_antialiasing(
    items,
    raster_bounds,
    effects,
    max_pixels_per_point,
    RasterPrimitiveAntialiasing::PerPrimitive,
  )
}

/// Fixed-output density is independent from the interactive preview budget.
/// Keep the source origin/extent policy unchanged, with a separate allocation
/// ceiling for unusually large authored canvases.
pub(crate) fn rasterize_vector_items_for_effects_at_fixed_output_pixels_per_point(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  requested_pixels_per_point: f32,
) -> Option<DrawingRaster> {
  if !requested_pixels_per_point.is_finite()
    || requested_pixels_per_point <= 0.0
    || !raster_bounds.size.width.0.is_finite()
    || !raster_bounds.size.height.0.is_finite()
    || raster_bounds.size.width.0 <= 0.0
    || raster_bounds.size.height.0 <= 0.0
  {
    return None;
  }
  let pixels_per_point = effect_pixels_per_point_with_budget(
    raster_bounds.size.width.0,
    raster_bounds.size.height.0,
    requested_pixels_per_point,
    MAX_FIXED_OUTPUT_EFFECT_RASTER_PIXELS,
  )
  .min(requested_pixels_per_point);
  let width = raster_pixel_extent(raster_bounds.size.width.0, pixels_per_point);
  let height = raster_pixel_extent(raster_bounds.size.height.0, pixels_per_point);
  if u64::from(width) * u64::from(height) > MAX_FIXED_OUTPUT_EFFECT_RASTER_PIXELS as u64 {
    return None;
  }
  rasterize_vector_items_for_effects_at_pixels_per_point(
    items,
    raster_bounds,
    effects,
    pixels_per_point,
  )
}

pub(crate) fn rasterize_vector_items_for_effects_at_bounded_pixels_per_point_with_antialiasing(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  max_pixels_per_point: f32,
  primitive_antialiasing: RasterPrimitiveAntialiasing,
) -> Option<DrawingRaster> {
  let pixels_per_point = effect_pixels_per_point_with_max(
    raster_bounds.size.width.0,
    raster_bounds.size.height.0,
    max_pixels_per_point,
  );
  rasterize_vector_items_for_effects_at_pixels_per_point_with_extent_and_antialiasing(
    items,
    raster_bounds,
    effects,
    pixels_per_point,
    RasterSourceExtent::Outward,
    primitive_antialiasing,
  )
}

/// Rasterizes an effect source in its own stable coordinate system.
///
/// Direct2D effect inputs are images with local bounds.  Their pixels must not
/// inherit the origin of a later glow, shadow, or reflection output surface:
/// changing an effect radius would otherwise move the vector sample lattice
/// before the effect is evaluated.  The one-device-pixel guard reproduces the
/// source allocation used by Word's near-zero spatial-effect branch; the
/// caller-provided display bounds then select the local bitmap transported into
/// the independently allocated effect output.
pub(crate) fn rasterize_vector_items_for_effects_as_local_source_at_pixels_per_point_with_antialiasing(
  items: &[DisplayItem<'static>],
  source_bounds: Rect,
  source_display_bounds: Rect,
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  pixels_per_point: f32,
  primitive_antialiasing: RasterPrimitiveAntialiasing,
) -> Option<DrawingRaster> {
  if !pixels_per_point.is_finite()
    || pixels_per_point <= 0.0
    || source_bounds.size.width.0 <= 0.0
    || source_bounds.size.height.0 <= 0.0
    || source_display_bounds.size.width.0 <= 0.0
    || source_display_bounds.size.height.0 <= 0.0
  {
    return None;
  }

  // Allocate the guard in device pixels. Subtracting 1/scale in page-space
  // f32 and scaling that rounded origin back loses a fraction of a sample:
  // merely translating the same source can then change its edge coverage.
  let mapping = PageToRasterMapping {
    width_px: raster_source_extent(
      source_bounds.size.width.0,
      pixels_per_point,
      RasterSourceExtent::Outward,
    )
    .checked_add(2)?,
    height_px: raster_source_extent(
      source_bounds.size.height.0,
      pixels_per_point,
      RasterSourceExtent::Outward,
    )
    .checked_add(2)?,
    scale_x: pixels_per_point,
    scale_y: pixels_per_point,
    translate_x: 1.0 - source_bounds.origin.x.0 * pixels_per_point,
    translate_y: 1.0 - source_bounds.origin.y.0 * pixels_per_point,
    text_hinting: None,
  };
  let requirements = super::drawingml_image_effects::source_requirements(effects);
  if requirements.children {
    return None;
  }
  let render = |items: &[DisplayItem<'static>]| {
    rasterize_vector_items_at_mapping(items, mapping, primitive_antialiasing)
  };
  let layer = |kind| {
    let mut layer_items = Vec::new();
    for item in items {
      collect_source_layer_item(item, kind, &mut layer_items)?;
    }
    render(&layer_items)
  };
  let raster = DrawingRaster {
    image: render(items)?,
    fill_image: if requirements.fill {
      Some(layer(SourceLayer::Fill)?)
    } else {
      None
    },
    line_image: if requirements.line {
      Some(layer(SourceLayer::Line)?)
    } else {
      None
    },
    fill_line_image: None,
    children_image: None,
    pixels_per_point,
  };

  let rounded_nonnegative = |value: f32| {
    value
      .is_finite()
      .then(|| value.round())
      .filter(|value| *value >= 0.0)
      .map(|value| value as u32)
  };
  let crop_left = rounded_nonnegative(
    (source_display_bounds.origin.x.0 - source_bounds.origin.x.0) * pixels_per_point + 1.0,
  )?;
  let crop_top = rounded_nonnegative(
    (source_display_bounds.origin.y.0 - source_bounds.origin.y.0) * pixels_per_point + 1.0,
  )?;
  let crop_width =
    rounded_nonnegative(source_display_bounds.size.width.0 * pixels_per_point)?.max(1);
  let crop_height =
    rounded_nonnegative(source_display_bounds.size.height.0 * pixels_per_point)?.max(1);

  crop_drawing_raster(raster, crop_left, crop_top, crop_width, crop_height)
}

fn crop_drawing_raster(
  mut raster: DrawingRaster,
  left: u32,
  top: u32,
  width: u32,
  height: u32,
) -> Option<DrawingRaster> {
  let crop = |image: RgbaImage| {
    let right = left.checked_add(width)?;
    let bottom = top.checked_add(height)?;
    if right > image.width() || bottom > image.height() {
      return None;
    }
    Some(image::imageops::crop_imm(&image, left, top, width, height).to_image())
  };
  let crop_optional = |image: Option<RgbaImage>| match image {
    Some(image) => Some(Some(crop(image)?)),
    None => Some(None),
  };
  raster.image = crop(raster.image)?;
  raster.fill_image = crop_optional(raster.fill_image)?;
  raster.line_image = crop_optional(raster.line_image)?;
  raster.fill_line_image = crop_optional(raster.fill_line_image)?;
  raster.children_image = crop_optional(raster.children_image)?;
  Some(raster)
}

/// Places every logical source plane at the same coordinate on a transparent
/// effect working surface.  `replace` is intentional: the destination is an
/// allocation boundary, not another compositing operation.
pub(crate) fn place_drawing_raster_on_transparent_surface(
  mut raster: DrawingRaster,
  width: u32,
  height: u32,
  left: u32,
  top: u32,
) -> Option<DrawingRaster> {
  let place = |image: RgbaImage| {
    let right = left.checked_add(image.width())?;
    let bottom = top.checked_add(image.height())?;
    if width == 0 || height == 0 || right > width || bottom > height {
      return None;
    }
    let mut output = RgbaImage::new(width, height);
    replace(&mut output, &image, i64::from(left), i64::from(top));
    Some(output)
  };
  let place_optional = |image: Option<RgbaImage>| match image {
    Some(image) => Some(Some(place(image)?)),
    None => Some(None),
  };
  raster.image = place(raster.image)?;
  raster.fill_image = place_optional(raster.fill_image)?;
  raster.line_image = place_optional(raster.line_image)?;
  raster.fill_line_image = place_optional(raster.fill_line_image)?;
  raster.children_image = place_optional(raster.children_image)?;
  Some(raster)
}

/// Paint directly on the independently allocated 3-D material bitmap. This
/// does not resize a completed effect source or change the geometry grid.
pub(crate) fn rasterize_text_surface_material_texture(
  item: &TextRun<'static>,
  plan: super::drawingml_3d::TextMaterialTexturePlan,
) -> Option<super::drawingml_3d::TextMaterialTexture> {
  let mut pixmap = Pixmap::new(plan.width, plan.height)?;
  let mut text_metrics = TextMetrics::new();
  let outline = text_outline(item, None, &mut text_metrics)?;
  let clip_path = path_from_commands(&outline.commands, &[], true)?;
  let fill = text_fill_material_item(item);
  let line = text_surface_outline_item(item);
  // GDI+ resolves coverage for FillPath and DrawPath independently, then
  // composites them. Resolving after both draws instead correlates their
  // samples and gives the wrong alpha along their shared boundary.
  // Bands bound temporary memory without reducing the requested density.
  let sample_width = plan.width.checked_mul(8)?;
  for top in (0..plan.height).step_by(64) {
    let rows = (plan.height - top).min(64);
    let mut samples = Pixmap::new(sample_width, rows * 4)?;
    let t = plan.page_to_texture;
    // This GDI+ target explicitly selects PixelOffsetModeHalf. The shared
    // 8x4 helpers store PixelOffsetModeNone's integer-centered samples, so
    // compensate their half-pixel storage origin here, not in glyph/page UV.
    // Other effect sources retain their independently observed pixel mode.
    let transform = SkTransform::from_row(
      t.sx * 8.0,
      t.ky * 4.0,
      t.kx * 8.0,
      t.sy * 4.0,
      t.tx * 8.0 - 4.0,
      t.ty * 4.0 - (top * 4) as f32 - 2.0,
    );
    draw_text(
      &mut samples,
      &fill,
      transform,
      None,
      RasterPrimitiveAntialiasing::OfficeAntiAlias8x4,
      &mut text_metrics,
    )?;
    composite_material_sample_band(&mut pixmap, &samples, top);
    samples.fill(SkColor::TRANSPARENT);
    draw_text(
      &mut samples,
      &line,
      transform,
      None,
      RasterPrimitiveAntialiasing::OfficeAntiAlias8x4,
      &mut text_metrics,
    )?;
    let mask = office_8x4_sample_mask(&samples, &clip_path, FillRule::EvenOdd, transform)?;
    samples.apply_mask(&mask);
    composite_material_sample_band(&mut pixmap, &samples, top);
  }
  plan.finish(pixmap)
}

fn text_surface_outline_item(item: &TextRun<'static>) -> TextRun<'static> {
  let mut line = item.clone();
  let options = line
    .style
    .pdf_glyph_outline_options
    .get_or_insert_with(|| std::sync::Arc::new(Default::default()));
  let options = std::sync::Arc::make_mut(options);
  options.fill = Some(Fill::None);
  if matches!(options.outline_fill, Some(Fill::None)) {
    options.outline_stroke = None;
    line.style.outline_color = None;
    line.style.outline_width = Pt(0.0);
    return line;
  }
  // Word expands the positive source-paint pen BEFORE constructing the GDI+
  // pen and applying its inner-half compound band. Exact-option controls span
  // Example/E/HIl, 18..48pt fonts, .5..6pt widths, centered/inset alignment,
  // and zero/noFill stopping cases. Both the physical material texture and
  // the independent reflection source use this paint; ordinary glyph strokes
  // and solid 3-D contour geometry must retain their authored widths.
  // This is a source-space paint allowance, not a final-raster pixel or a
  // texture-size adjustment. The actual pen normalizer leaves it unchanged.
  const SOURCE_PAINT_EXPANSION_PT: f32 = 0.12;
  if line.style.outline_width.0.is_finite() && line.style.outline_width.0 > 0.0 {
    line.style.outline_width.0 += SOURCE_PAINT_EXPANSION_PT;
  }
  // Office's material pen uses an inner-half compound band on the original
  // closed glyph path, despite PenAlignmentCenter. A centered full stroke
  // incorrectly introduces translucent material outside the glyph. An
  // explicitly inset authored outline keeps its complete width inside.
  if let Some(stroke) = options.outline_stroke.as_mut() {
    if stroke.width.0.is_finite() && stroke.width.0 > 0.0 {
      stroke.width.0 += SOURCE_PAINT_EXPANSION_PT;
    }
    if stroke.alignment == Some(StrokeAlignment::Inside) {
      stroke.width.0 *= 2.0;
    }
  }
  line
}

fn composite_material_sample_band(pixmap: &mut Pixmap, samples: &Pixmap, top: u32) {
  composite_raster_sample_band(pixmap, samples, top, 8, 4);
}

fn composite_raster_sample_band(
  pixmap: &mut Pixmap,
  samples: &Pixmap,
  top: u32,
  horizontal_samples: u32,
  vertical_samples: u32,
) {
  let sample_width = samples.width();
  let rows = samples.height() / vertical_samples;
  let sample_count = horizontal_samples * vertical_samples;
  for y in 0..rows {
    for x in 0..pixmap.width() {
      let mut sum = [0_u32; 4];
      for dy in 0..vertical_samples {
        for dx in 0..horizontal_samples {
          let offset = (((y * vertical_samples + dy) * sample_width + x * horizontal_samples + dx)
            * 4) as usize;
          for (total, byte) in sum.iter_mut().zip(&samples.data()[offset..offset + 4]) {
            *total += u32::from(*byte);
          }
        }
      }
      let source = sum.map(|value| ((value + sample_count / 2) / sample_count) as u8);
      let inverse_alpha = 255 - u32::from(source[3]);
      let offset = (((top + y) * pixmap.width() + x) * 4) as usize;
      for (byte, source) in pixmap.data_mut()[offset..offset + 4].iter_mut().zip(source) {
        *byte = (u32::from(source) + (u32::from(*byte) * inverse_alpha + 127) / 255).min(255) as u8;
      }
    }
  }
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

/// Paints the glyph interior with the authored character-outline material.
///
/// Word's effective W14 static-3-D path has one deliberately narrow material
/// exception: `textFill/noFill` plus a fully opaque positive-width outline
/// colors the complete raw-glyph face. This helper changes paint ownership
/// only; the caller still supplies the original glyph geometry to the 3-D
/// tessellator, so the outline never widens the physical solid.
pub(crate) fn rasterize_text_outline_material_layer_at_pixels_per_point(
  item: &TextRun<'static>,
  raster_bounds: Rect,
  pixels_per_point: f32,
) -> Option<DrawingRaster> {
  let material = text_outline_material_item(item)?;
  let (image, pixels_per_point) = rasterize_vector_items_impl_at_pixels_per_point(
    &[DisplayItem::Text(material)],
    raster_bounds,
    pixels_per_point,
  )?;
  Some(DrawingRaster {
    image,
    fill_image: None,
    line_image: None,
    fill_line_image: None,
    children_image: None,
    pixels_per_point,
  })
}

/// Rasterizes only the character-outline geometry with opaque white paint.
///
/// The alpha channel is a paint-independent coverage attribute for W14's
/// text material mesh. Keeping authored opacity in the separate outline
/// material layer avoids multiplying transparency twice when the line is
/// composited over `textFill` on a bevel vertex.
pub(crate) fn rasterize_text_outline_coverage_layer_at_pixels_per_point(
  item: &TextRun<'static>,
  raster_bounds: Rect,
  pixels_per_point: f32,
) -> Option<DrawingRaster> {
  let material = text_outline_coverage_item(item)?;
  if !pixels_per_point.is_finite()
    || pixels_per_point <= 0.0
    || raster_bounds.size.width.0 <= 0.0
    || raster_bounds.size.height.0 <= 0.0
  {
    return None;
  }
  // This is a material coverage attribute, subsequently interpolated by the
  // static-3-D renderer. Preserve subpixel edge differences with the same
  // bounded high-resolution realization as other explicitly mapped surfaces;
  // the ordinary 4x4 scanner can quantize distinct edges to the same alpha.
  // Keep its conventional pixel-centre mapping (not GDI+ sample storage).
  let image = rasterize_vector_items_impl_with_mapping(
    &[DisplayItem::Text(material)],
    PageToRasterMapping {
      width_px: raster_pixel_extent(raster_bounds.size.width.0, pixels_per_point),
      height_px: raster_pixel_extent(raster_bounds.size.height.0, pixels_per_point),
      scale_x: pixels_per_point,
      scale_y: pixels_per_point,
      translate_x: -raster_bounds.origin.x.0 * pixels_per_point,
      translate_y: -raster_bounds.origin.y.0 * pixels_per_point,
      text_hinting: None,
    },
  )?;
  Some(DrawingRaster {
    image,
    fill_image: None,
    line_image: None,
    fill_line_image: None,
    children_image: None,
    pixels_per_point,
  })
}

/// Paints the raw glyph interior with `textFill` while omitting the
/// independently authored character outline.
///
/// Word maps fill and outline to separate fixed-width regions of a W14 text
/// bevel. Keeping this layer independent prevents the outline antialias fringe
/// from becoming an implicit, bevel-width-dependent fill texture.
pub(crate) fn rasterize_text_fill_material_layer_at_pixels_per_point(
  item: &TextRun<'static>,
  raster_bounds: Rect,
  pixels_per_point: f32,
) -> Option<DrawingRaster> {
  let material = text_fill_material_item(item);
  let (image, pixels_per_point) = rasterize_vector_items_impl_at_pixels_per_point(
    &[DisplayItem::Text(material)],
    raster_bounds,
    pixels_per_point,
  )?;
  Some(DrawingRaster {
    image,
    fill_image: None,
    line_image: None,
    fill_line_image: None,
    children_image: None,
    pixels_per_point,
  })
}

fn text_fill_material_item(item: &TextRun<'static>) -> TextRun<'static> {
  let mut material = item.clone();
  material.style.outline_color = None;
  material.style.outline_width = Pt(0.0);
  if let Some(options) = material.style.pdf_glyph_outline_options.as_mut() {
    let options = std::sync::Arc::make_mut(options);
    options.outline_fill = None;
    options.outline_stroke = None;
    options.outline_has_authored_transparency = false;
  }
  material
}

/// The unlit reflection source includes its own 3-D contour paint. It is not
/// the shaded/extruded foreground, nor the white coverage-only shadow caster.
/// The raw-glyph interior owns fill and the inward part of the character
/// outline; the exterior owns the outward part of the 3-D contour. Exact
/// zero-blur Office reflection controls distinguish these regions from a
/// centered contour painted over an ordinary, outward-growing text outline.
pub(crate) fn rasterize_static_3d_text_reflection_source(
  item: &TextRun<'static>,
  raster_bounds: Rect,
  pixels_per_point: f32,
  contour_width: Pt,
  contour_color: Color,
  antialiasing: RasterPrimitiveAntialiasing,
) -> Option<RgbaImage> {
  if !pixels_per_point.is_finite()
    || pixels_per_point <= 0.0
    || raster_bounds.size.width.0 <= 0.0
    || raster_bounds.size.height.0 <= 0.0
  {
    return None;
  }
  let width = raster_source_extent(
    raster_bounds.size.width.0,
    pixels_per_point,
    RasterSourceExtent::Outward,
  );
  let height = raster_source_extent(
    raster_bounds.size.height.0,
    pixels_per_point,
    RasterSourceExtent::Outward,
  );
  rasterize_static_3d_text_reflection_source_with_mapping(
    item,
    PageToRasterMapping {
      width_px: width,
      height_px: height,
      scale_x: pixels_per_point,
      scale_y: pixels_per_point,
      translate_x: -raster_bounds.origin.x.0 * pixels_per_point,
      translate_y: -raster_bounds.origin.y.0 * pixels_per_point,
      text_hinting: None,
    },
    contour_width,
    contour_color,
    antialiasing,
  )
}

/// Realizes reflection paint directly on its source texture lattice. Each
/// axis, the page origin, and the sample-coverage policy are independent of
/// the final scene bitmap. Do not implement this by resizing an isotropic
/// glyph bitmap: material/contour intersection precedes coverage resolve.
pub(crate) fn rasterize_static_3d_text_reflection_source_with_mapping(
  item: &TextRun<'static>,
  mapping: PageToRasterMapping,
  contour_width: Pt,
  contour_color: Color,
  antialiasing: RasterPrimitiveAntialiasing,
) -> Option<RgbaImage> {
  if !valid_raster_mapping(mapping) || mapping.text_hinting.is_some() {
    // This source partitions continuous, unhinted raw-glyph geometry.
    // A hinted font bitmap cannot substitute for that geometry contract.
    return None;
  }
  let (width, height) = (mapping.width_px, mapping.height_px);
  let mut output = Pixmap::new(width, height)?;
  let mut text_metrics = TextMetrics::new();
  let outline = text_outline(item, None, &mut text_metrics)?;
  let path = path_from_commands(&outline.commands, &[], true)?;
  let fill = text_fill_material_item(item);
  let line = text_surface_outline_item(item);
  let contour = static_3d_text_reflection_contour_item(item, contour_width, contour_color);
  let (sx, sy, sample_mode) = match antialiasing {
    // Preserve the ordinary scanner's 4x4 pixel-centred coverage contract,
    // but intersect geometry at sample resolution, not two resolved alphas.
    RasterPrimitiveAntialiasing::PerPrimitive | RasterPrimitiveAntialiasing::Direct2dStandard4 => {
      (4, 4, RasterPrimitiveAntialiasing::Aliased)
    }
    RasterPrimitiveAntialiasing::Aliased => (1, 1, RasterPrimitiveAntialiasing::Aliased),
    RasterPrimitiveAntialiasing::OfficeAntiAlias8x4 => (8, 4, antialiasing),
  };
  for top in (0..height).step_by(64) {
    let rows = (height - top).min(64);
    let mut samples = Pixmap::new(width.checked_mul(sx)?, rows * sy)?;
    let transform = SkTransform::from_row(
      mapping.scale_x * sx as f32,
      0.0,
      0.0,
      mapping.scale_y * sy as f32,
      mapping.translate_x * sx as f32,
      mapping.translate_y * sy as f32 - (top * sy) as f32,
    );
    let inside = if sample_mode == RasterPrimitiveAntialiasing::OfficeAntiAlias8x4 {
      office_8x4_sample_mask(&samples, &path, FillRule::EvenOdd, transform)?
    } else {
      let mut mask = Mask::new(samples.width(), samples.height())?;
      mask.fill_path(&path, FillRule::EvenOdd, false, transform);
      mask
    };
    let mut outside = inside.clone();
    for value in outside.data_mut() {
      *value = 255 - *value;
    }
    for (paint, clip) in [
      (Some(&fill), None),
      (Some(&line), Some(&inside)),
      (contour.as_ref(), Some(&outside)),
    ] {
      let Some(paint) = paint else {
        continue;
      };
      samples.fill(SkColor::TRANSPARENT);
      draw_text(
        &mut samples,
        paint,
        transform,
        None,
        sample_mode,
        &mut text_metrics,
      )?;
      if let Some(clip) = clip {
        samples.apply_mask(clip);
      }
      composite_raster_sample_band(&mut output, &samples, top, sx, sy);
    }
  }
  pixmap_into_rgba(output)
}

pub(crate) fn static_3d_text_reflection_contour_item(
  item: &TextRun<'static>,
  width: Pt,
  color: Color,
) -> Option<TextRun<'static>> {
  if !width.0.is_finite() || width.0 <= 0.0 || color.a == 0 {
    return None;
  }
  let mut contour = item.clone();
  contour.style.outline_color = Some(color);
  contour.style.outline_width = width;
  let options = contour
    .style
    .pdf_glyph_outline_options
    .get_or_insert_with(Default::default);
  let options = std::sync::Arc::make_mut(options);
  options.fill = Some(Fill::None);
  options.outline_fill = Some(Fill::Solid(color));
  options.outline_stroke = Some(Stroke {
    width,
    color,
    alignment: Some(StrokeAlignment::Center),
    join: Some(super::StrokeJoin::Round),
    ..Stroke::default()
  });
  options.outline_has_authored_transparency = color.a < 255;
  Some(contour)
}

/// Boundary paint for the existing root effect source. Its full centered
/// coverage must not be replaced by the reflected material's exterior clip.
pub(crate) fn rasterize_static_3d_text_contour_effect_source(
  item: &TextRun<'static>,
  raster_bounds: Rect,
  pixels_per_point: f32,
  width: Pt,
  color: Color,
  antialiasing: RasterPrimitiveAntialiasing,
) -> Option<RgbaImage> {
  let contour = static_3d_text_reflection_contour_item(item, width, color)?;
  rasterize_vector_items_impl_at_pixels_per_point_with_extent_and_antialiasing(
    &[DisplayItem::Text(contour)],
    raster_bounds,
    pixels_per_point,
    RasterSourceExtent::Outward,
    antialiasing,
  )
  .map(|(image, _)| image)
}

/// Realizes the opaque front-plane caster of effective W14 3-D text.
///
/// Character outline paint is an interior material of the solid, whereas
/// `contourW` contributes a separate solid boundary (MS-OI29500 20.1.5.6).
/// Native Word shadow-colour pairs over two texts, four contour widths and
/// three character-outline opacities distinguish these two sources: changing
/// contour width grows the caster; changing character-outline opacity does
/// not introduce a translucent ring around the opaque face. Extrusion-depth
/// controls keep that front-plane silhouette, apart from foreground occlusion.
///
/// This contract is established for opaque fills. Retain the existing source
/// for nonuniform/translucent or missing fills until their material ownership
/// is established independently; do not infer opacity from coverage pixels.
pub(crate) fn rasterize_opaque_static_3d_text_shadow_source(
  item: &TextRun<'static>,
  raster_bounds: Rect,
  pixels_per_point: f32,
  contour_width: Pt,
  contour_alpha: u8,
) -> Option<DrawingRaster> {
  let material = opaque_static_3d_text_effect_item(item, contour_width, contour_alpha)?;
  rasterize_static_3d_text_effect_mask(&material, raster_bounds, pixels_per_point)
}

/// Realizes a prepared glyph/contour mask without invoking 3-D material paint.
pub(crate) fn rasterize_static_3d_text_effect_mask(
  material: &TextRun<'static>,
  raster_bounds: Rect,
  pixels_per_point: f32,
) -> Option<DrawingRaster> {
  // Coverage filters read alpha only; retain the authored fill so gradient
  // realization, font selection and source-grid policy stay unchanged.
  let (image, pixels_per_point) = rasterize_vector_items_impl_at_pixels_per_point(
    &[DisplayItem::Text(material.clone())],
    raster_bounds,
    pixels_per_point,
  )?;
  Some(DrawingRaster {
    image,
    fill_image: None,
    line_image: None,
    fill_line_image: None,
    children_image: None,
    pixels_per_point,
  })
}

pub(crate) fn opaque_static_3d_text_effect_item(
  item: &TextRun<'static>,
  contour_width: Pt,
  contour_alpha: u8,
) -> Option<TextRun<'static>> {
  let fill_opacity = item
    .style
    .pdf_glyph_outline_options
    .as_deref()
    .and_then(|options| options.fill.as_ref())
    .map_or(Some(f32::from(item.color.a) / 255.0), uniform_fill_opacity);
  if fill_opacity != Some(1.0) || !contour_width.0.is_finite() {
    return None;
  }
  let mut material = text_fill_material_item(item);
  if contour_width.0 > 0.0 && contour_alpha != 0 {
    let color = Color {
      r: 255,
      g: 255,
      b: 255,
      a: contour_alpha,
    };
    material.style.outline_color = Some(color);
    material.style.outline_width = contour_width;
    let options = material
      .style
      .pdf_glyph_outline_options
      .get_or_insert_with(Default::default);
    let options = std::sync::Arc::make_mut(options);
    options.outline_fill = Some(Fill::Solid(color));
    options.outline_stroke = Some(Stroke {
      width: contour_width,
      color,
      alignment: Some(StrokeAlignment::Center),
      join: Some(super::StrokeJoin::Round),
      ..Stroke::default()
    });
    options.outline_has_authored_transparency = contour_alpha < 255;
  }
  Some(material)
}

fn text_outline_coverage_item(item: &TextRun<'static>) -> Option<TextRun<'static>> {
  let mut material = item.clone();
  let mut options = material.style.pdf_glyph_outline_options.as_deref()?.clone();
  let mut stroke = options.outline_stroke.clone().or_else(|| {
    material
      .style
      .outline_color
      .filter(|_| material.style.outline_width.0 > f32::EPSILON)
      .map(|color| Stroke {
        width: material.style.outline_width,
        color,
        ..Stroke::default()
      })
  })?;
  if stroke.width.0 <= f32::EPSILON {
    return None;
  }
  let authored_fill = options.outline_fill.as_ref().cloned().unwrap_or_else(|| {
    if let Some(gradient) = stroke.gradient.clone() {
      Fill::Gradient(gradient)
    } else if let Some(pattern) = stroke.pattern {
      Fill::Pattern(pattern)
    } else {
      Fill::Solid(stroke.color)
    }
  });
  if !fill_has_visible_alpha(&authored_fill) {
    return None;
  }

  let opaque = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
  };
  stroke.color = opaque;
  stroke.gradient = None;
  stroke.pattern = None;
  material.color = opaque;
  material.style.outline_color = Some(opaque);
  material.style.outline_width = stroke.width;
  options.fill = Some(Fill::None);
  options.fill_has_authored_transparency = false;
  options.outline_fill = Some(Fill::Solid(opaque));
  options.outline_stroke = Some(stroke);
  options.outline_has_authored_transparency = false;
  material.style.pdf_glyph_outline_options = Some(std::sync::Arc::new(options));
  Some(material)
}

fn text_outline_material_item(item: &TextRun<'static>) -> Option<TextRun<'static>> {
  let mut material = item.clone();
  let mut options = material.style.pdf_glyph_outline_options.as_deref()?.clone();
  let stroke = options.outline_stroke.clone().or_else(|| {
    material
      .style
      .outline_color
      .filter(|_| material.style.outline_width.0 > f32::EPSILON)
      .map(|color| Stroke {
        width: material.style.outline_width,
        color,
        ..Stroke::default()
      })
  })?;
  if stroke.width.0 <= f32::EPSILON {
    return None;
  }
  let fill = options.outline_fill.clone().unwrap_or_else(|| {
    if let Some(gradient) = stroke.gradient.clone() {
      Fill::Gradient(gradient)
    } else if let Some(pattern) = stroke.pattern {
      Fill::Pattern(pattern)
    } else {
      Fill::Solid(stroke.color)
    }
  });
  if matches!(fill, Fill::None) {
    return None;
  }

  material.color = stroke.color;
  material.style.outline_color = None;
  material.style.outline_width = Pt(0.0);
  options.fill = Some(fill);
  options.fill_has_authored_transparency = options.outline_has_authored_transparency;
  options.outline_fill = None;
  options.outline_stroke = None;
  options.outline_has_authored_transparency = false;
  material.style.pdf_glyph_outline_options = Some(std::sync::Arc::new(options));
  Some(material)
}

pub(crate) fn static_3d_text_geometry(
  item: &TextRun<'static>,
  raster_bounds: Rect,
  pixels_per_point: f32,
) -> Option<super::drawingml_3d::Static3dTextGeometryPaths> {
  let mut text_metrics = TextMetrics::new();
  let outline = text_outline(item, None, &mut text_metrics)?;
  let geometry = super::drawingml_3d::Static3dTextGeometryPaths::from_page_path_for_direct3d9(
    &outline.commands,
    raster_bounds,
    pixels_per_point,
  )?;
  let outline_material_inset_px = static_3d_text_outline_material_inset_px(item, pixels_per_point);
  let outline_has_authored_transparency = item
    .style
    .pdf_glyph_outline_options
    .as_deref()
    .is_some_and(|options| options.outline_has_authored_transparency);
  let fill_has_authored_transparency = item
    .style
    .pdf_glyph_outline_options
    .as_deref()
    .is_some_and(|options| options.fill_has_authored_transparency);
  let (fill_uniform_paint_opacity, outline_uniform_paint_opacity) =
    static_3d_text_material_opacities(item);
  Some(
    geometry
      .with_uniform_paint_opacity(uniform_static_3d_text_paint_opacity(item))
      .with_front_material_opacities(fill_uniform_paint_opacity, outline_uniform_paint_opacity)
      .with_front_outline_material_inset_px(outline_material_inset_px)
      .with_front_fill_authored_transparency(fill_has_authored_transparency)
      .with_front_outline_authored_transparency(outline_has_authored_transparency),
  )
}

/// Resolves the inward material region contributed by a visible W14 text
/// outline on Word's raw-glyph static-3-D solid.
///
/// [MS-DOCX] 2.6.3.36 makes a missing `algn` centered by default, while
/// 2.6.4.11 defines `ctr` around the source path and `in` wholly inside it.
/// Word clips the centered line's outward half to the raw glyph solid, so its
/// bevel material boundary is one half of the authored width. Exact-config
/// Office controls independently hold the physical glyph support fixed while
/// moving this material boundary with outline width. An entirely transparent
/// outline is the negative control and must not split the fill material.
fn static_3d_text_outline_material_inset_px(
  item: &TextRun<'static>,
  pixels_per_point: f32,
) -> Option<f32> {
  if !pixels_per_point.is_finite() || pixels_per_point <= f32::EPSILON {
    return None;
  }
  let options = item.style.pdf_glyph_outline_options.as_deref();
  let stroke = options
    .and_then(|value| value.outline_stroke.as_ref())
    .cloned()
    .or_else(|| {
      item
        .style
        .outline_color
        .filter(|_| item.style.outline_width.0 > f32::EPSILON)
        .map(|color| Stroke {
          width: item.style.outline_width,
          color,
          ..Stroke::default()
        })
    })?;
  if !stroke.width.0.is_finite() || stroke.width.0 <= f32::EPSILON {
    return None;
  }

  let outline_fill = options
    .and_then(|value| value.outline_fill.as_ref())
    .cloned()
    .unwrap_or_else(|| {
      if let Some(gradient) = stroke.gradient.clone() {
        Fill::Gradient(gradient)
      } else if let Some(pattern) = stroke.pattern {
        Fill::Pattern(pattern)
      } else {
        Fill::Solid(stroke.color)
      }
    });
  if !fill_has_visible_alpha(&outline_fill) {
    return None;
  }

  let inward_fraction = match stroke.alignment {
    Some(StrokeAlignment::Inside) => 1.0,
    Some(StrokeAlignment::Center) | None => 0.5,
  };
  Some(stroke.width.0 * pixels_per_point * inward_fraction)
}

fn fill_has_visible_alpha(fill: &Fill<'_>) -> bool {
  match fill {
    Fill::None => false,
    Fill::Solid(color) => color.a != 0,
    Fill::Gradient(gradient) => gradient.stops.iter().any(|stop| stop.color.a != 0),
    Fill::Pattern(pattern) => pattern.foreground.a != 0 || pattern.background.a != 0,
    // Theme and image fills cannot occur in CT_TextOutlineEffect, but retain
    // their conservative visible-paint semantics for legacy callers.
    Fill::Theme(_) | Fill::Image { .. } => true,
  }
}

/// Returns a paint's authored opacity when it is constant over the complete
/// material. Coverage is deliberately absent from this value: the WPF glyph
/// painter applies coverage after selecting BGR/PBGRA from the authored paint
/// state, so recovering opacity from an antialiased material bitmap would
/// multiply coverage from two independently rasterized paths.
fn uniform_fill_opacity(fill: &Fill<'_>) -> Option<f32> {
  let alpha = match fill {
    Fill::None => 0,
    Fill::Solid(color) => color.a,
    Fill::Gradient(gradient) => {
      let alpha = gradient.stops.first()?.color.a;
      gradient
        .stops
        .iter()
        .all(|stop| stop.color.a == alpha)
        .then_some(alpha)?
    }
    Fill::Pattern(pattern) => {
      (pattern.foreground.a == pattern.background.a).then_some(pattern.foreground.a)?
    }
    Fill::Theme(_) | Fill::Image { .. } => return None,
  };
  Some(f32::from(alpha) / 255.0)
}

/// Carries independently authored fill and outline opacity into the W14 text
/// material mesh whenever each paint is position-independent in alpha. The
/// paint selection mirrors `draw_text` and `text_outline_material_item`.
fn static_3d_text_material_opacities(item: &TextRun<'static>) -> (Option<f32>, Option<f32>) {
  let options = item.style.pdf_glyph_outline_options.as_deref();
  let fill_opacity = options
    .and_then(|value| value.fill.as_ref())
    .map_or(Some(f32::from(item.color.a) / 255.0), uniform_fill_opacity);

  let stroke = options
    .and_then(|value| value.outline_stroke.clone())
    .or_else(|| {
      item
        .style
        .outline_color
        .filter(|_| item.style.outline_width.0 > f32::EPSILON)
        .map(|color| Stroke {
          width: item.style.outline_width,
          color,
          ..Stroke::default()
        })
    });
  let outline_opacity = stroke
    .filter(|stroke| stroke.width.0 > f32::EPSILON)
    .and_then(|stroke| {
      if let Some(fill) = options.and_then(|value| value.outline_fill.as_ref()) {
        uniform_fill_opacity(fill)
      } else if let Some(gradient) = stroke.gradient.as_ref() {
        uniform_fill_opacity(&Fill::Gradient(gradient.clone()))
      } else if let Some(pattern) = stroke.pattern {
        uniform_fill_opacity(&Fill::Pattern(pattern))
      } else {
        Some(f32::from(stroke.color.a) / 255.0)
      }
    });

  (fill_opacity, outline_opacity)
}

/// Returns authored opacity only when it is independent of position and of
/// a second text paint. `draw_text` uses this same fill-selection rule. More
/// complex sources retain the raster recovery path until their opacity map is
/// produced by the same renderer as their RGBA material bitmap.
fn uniform_static_3d_text_paint_opacity(item: &TextRun<'static>) -> Option<f32> {
  let options = item.style.pdf_glyph_outline_options.as_deref();
  let has_outline = options
    .and_then(|value| value.outline_stroke.as_ref())
    .is_some_and(|stroke| stroke.width.0 > f32::EPSILON)
    || (item.style.outline_width.0 > f32::EPSILON && item.style.outline_color.is_some());
  if has_outline {
    return None;
  }
  let color = match options.and_then(|value| value.fill.as_ref()) {
    Some(Fill::Solid(color)) => *color,
    Some(Fill::None) => return Some(0.0),
    Some(Fill::Theme(_) | Fill::Gradient(_) | Fill::Image { .. } | Fill::Pattern(_)) => {
      return None;
    }
    None => item.color,
  };
  Some(f32::from(color.a) / 255.0)
}

/// Retains normals of the actual painted source, including widened/dashed
/// strokes. Coverage and projection still use the source raster. Sources with
/// position-dependent opacity or compositing need their alpha boundary, not
/// the union of opaque vector paths.
pub(crate) fn static_3d_raster_source_boundary(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  pixels_per_point: f32,
) -> Option<super::drawingml_3d::RasterSourceBoundary> {
  let mapping = SkTransform::from_row(
    pixels_per_point,
    0.0,
    0.0,
    pixels_per_point,
    -raster_bounds.origin.x.0 * pixels_per_point,
    -raster_bounds.origin.y.0 * pixels_per_point,
  );
  let mut boundary = super::drawingml_3d::RasterSourceBoundary::default();
  for item in items {
    let (path, fill, stroke) = match item {
      DisplayItem::Path(item) => {
        // Markers shorten the shaft and contribute separate painted shapes.
        // Do not silently retain the unshortened centerline as their boundary.
        if item.stroke.as_ref().is_some_and(|stroke| {
          shortened_straight_stroke_path(item, stroke).is_some()
            || !super::drawingml_stroke::stroke_end_marker_polygons(item, stroke).is_empty()
            || !super::drawingml_stroke::stroked_open_arrow_markers(item, stroke).is_empty()
        }) {
          return None;
        }
        (
          path_from_commands(&item.commands, &item.points, item.closed)?,
          &item.fill,
          item.stroke.as_ref(),
        )
      }
      DisplayItem::Rect(item) => (
        PathBuilder::from_rect(SkRect::from_xywh(
          item.bounds.origin.x.0,
          item.bounds.origin.y.0,
          item.bounds.size.width.0,
          item.bounds.size.height.0,
        )?),
        &item.fill,
        item.stroke.as_ref(),
      ),
      DisplayItem::Line(item) => {
        let mut builder = PathBuilder::new();
        builder.move_to(item.start.x.0, item.start.y.0);
        builder.line_to(item.end.x.0, item.end.y.0);
        (builder.finish()?, &Fill::None, Some(&item.stroke))
      }
      DisplayItem::LinkArea(_) | DisplayItem::AnnotationHint(_) => continue,
      _ => return None,
    };
    match fill {
      Fill::None => {}
      Fill::Solid(color) if color.a == 0 => {}
      Fill::Solid(color) if color.a == 255 => boundary.push(
        tiny_path_as_kurbo(&path.clone().transform(mapping)?),
        FillRule::EvenOdd,
      ),
      _ => return None,
    }
    if let Some(stroke) = stroke {
      if stroke.gradient.is_some() || stroke.pattern.is_some() {
        return None;
      }
      if stroke.width.0 <= 0.0 || stroke.color.a == 0 {
        continue;
      }
      if stroke.color.a != 255 {
        return None;
      }
      let expanded = expanded_stroke_path(&path, &resolved_sk_stroke(stroke), mapping)?;
      boundary.push(
        tiny_path_as_kurbo(&expanded.transform(mapping)?),
        FillRule::Winding,
      );
    }
  }
  (!boundary.is_empty()).then_some(boundary)
}

/// A picture's physical frame is independent of transparent texels and crop.
/// An authored page-space clip, when present, supplies the geometry instead.
pub(crate) fn static_3d_picture_frame(bounds: Rect, rotation_degrees: f32) -> Vec<PathCommand> {
  let center_x = bounds.origin.x.0 + bounds.size.width.0 * 0.5;
  let center_y = bounds.origin.y.0 + bounds.size.height.0 * 0.5;
  let (sin, cos) = rotation_degrees.to_radians().sin_cos();
  let mut commands = Vec::with_capacity(5);
  for (index, (u, v)) in [(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5), (-0.5, 0.5)]
    .into_iter()
    .enumerate()
  {
    let x = u * bounds.size.width.0;
    let y = v * bounds.size.height.0;
    let point = super::Point {
      x: Pt(cos.mul_add(x, -sin * y) + center_x),
      y: Pt(sin.mul_add(x, cos * y) + center_y),
    };
    commands.push(if index == 0 {
      PathCommand::MoveTo(point)
    } else {
      PathCommand::LineTo(point)
    });
  }
  commands.push(PathCommand::Close);
  commands
}

pub(crate) fn static_3d_shape_geometry(
  commands: &[PathCommand],
  raster_bounds: Rect,
  pixels_per_point: f32,
) -> Option<super::drawingml_3d::Static3dTextGeometry> {
  super::drawingml_3d::Static3dTextGeometry::from_page_path(
    commands,
    raster_bounds,
    pixels_per_point,
  )
}

pub(crate) fn static_3d_shape_geometry_with_mapping(
  commands: &[PathCommand],
  mapping: PageToRasterMapping,
) -> Option<super::drawingml_3d::Static3dTextGeometry> {
  super::drawingml_3d::Static3dTextGeometry::from_page_path_with_mapping(
    commands,
    mapping.scale_x,
    mapping.scale_y,
    mapping.translate_x,
    mapping.translate_y,
  )
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
  rasterize_group_items_for_effects_at_pixels_per_point_with_extent(
    items,
    raster_bounds,
    effects,
    pixels_per_point,
    RasterSourceExtent::Outward,
  )
}

pub(crate) fn rasterize_group_items_for_effects_at_pixels_per_point_with_extent(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  pixels_per_point: f32,
  extent: RasterSourceExtent,
) -> Option<DrawingRaster> {
  rasterize_vector_items_for_effects_impl_at_pixels_per_point(
    items,
    raster_bounds,
    effects,
    true,
    pixels_per_point,
    extent,
  )
}

fn rasterize_vector_items_for_effects_impl(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  items_are_children_source: bool,
) -> Option<DrawingRaster> {
  let pixels_per_point =
    effect_pixels_per_point(raster_bounds.size.width.0, raster_bounds.size.height.0);
  rasterize_vector_items_for_effects_impl_at_pixels_per_point(
    items,
    raster_bounds,
    effects,
    items_are_children_source,
    pixels_per_point,
    RasterSourceExtent::Outward,
  )
}

fn rasterize_vector_items_for_effects_impl_at_pixels_per_point(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  effects: &super::drawingml_image_effects::ImageEffectContainer,
  items_are_children_source: bool,
  pixels_per_point: f32,
  extent: RasterSourceExtent,
) -> Option<DrawingRaster> {
  rasterize_vector_items_for_effects_impl_at_pixels_per_point_with_antialiasing(
    items,
    raster_bounds,
    effects,
    items_are_children_source,
    pixels_per_point,
    extent,
    RasterPrimitiveAntialiasing::PerPrimitive,
  )
}

#[derive(Clone, Copy)]
enum SourceLayer {
  Fill,
  Line,
}

fn rasterize_source_layer_at_pixels_per_point(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  layer: SourceLayer,
  pixels_per_point: f32,
  extent: RasterSourceExtent,
  primitive_antialiasing: RasterPrimitiveAntialiasing,
) -> Option<(RgbaImage, f32)> {
  let mut layer_items = Vec::new();
  for item in items {
    collect_source_layer_item(item, layer, &mut layer_items)?;
  }
  if layer_items.is_empty() {
    let (image, pixels_per_point) =
      empty_raster_at_pixels_per_point(raster_bounds, pixels_per_point, extent)?;
    return Some((image, pixels_per_point));
  }
  rasterize_vector_items_impl_at_pixels_per_point_with_extent_and_antialiasing(
    &layer_items,
    raster_bounds,
    pixels_per_point,
    extent,
    primitive_antialiasing,
  )
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
    (SourceLayer::Fill, DisplayItem::Text(text)) => {
      Some(DisplayItem::Text(text_fill_material_item(text)))
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
    (SourceLayer::Line, DisplayItem::Text(text)) => {
      let mut text = text.clone();
      let options = text
        .style
        .pdf_glyph_outline_options
        .get_or_insert_with(|| std::sync::Arc::new(Default::default()));
      std::sync::Arc::make_mut(options).fill = Some(Fill::None);
      Some(DisplayItem::Text(text))
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

fn empty_raster_at_pixels_per_point(
  raster_bounds: Rect,
  pixels_per_point: f32,
  extent: RasterSourceExtent,
) -> Option<(RgbaImage, f32)> {
  let width_pt = raster_bounds.size.width.0;
  let height_pt = raster_bounds.size.height.0;
  if width_pt <= 0.0 || height_pt <= 0.0 || !pixels_per_point.is_finite() || pixels_per_point <= 0.0
  {
    return None;
  }
  Some((
    RgbaImage::new(
      raster_source_extent(width_pt, pixels_per_point, extent),
      raster_source_extent(height_pt, pixels_per_point, extent),
    ),
    pixels_per_point,
  ))
}

pub(crate) fn raster_pixel_extent(length_pt: f32, pixels_per_point: f32) -> u32 {
  let extent_px = length_pt * pixels_per_point;
  let nearest_px = extent_px.round();
  // Pixel-aligned bounds are divided by the density in
  // `align_rect_to_pixel_grid` and multiplied back here.  The f32 round trip
  // can leave an integer extent a few ULPs above its source value (for
  // example 116.00002), and a second unconditional ceil would allocate a
  // spurious row or column.  Snap only representationally-near integers;
  // genuine fractional coverage still rounds outward.
  let rounding_tolerance = f32::EPSILON * extent_px.abs().max(1.0) * 4.0;
  let outward_px = if (extent_px - nearest_px).abs() <= rounding_tolerance {
    nearest_px
  } else {
    extent_px.ceil()
  };
  outward_px.max(1.0) as u32
}

pub(crate) fn inclusive_far_edge_raster_pixel_extent(length_pt: f32, pixels_per_point: f32) -> u32 {
  // Word's standalone-WPS effect surface includes the sample on an exactly
  // aligned terminal boundary. The radius-6 width/height sweeps distinguish
  // this from both ordinary outward ceil and round: 294.48pt at 100 DPI is
  // exactly 409 pixels logically but allocates 410, and 54pt allocates 76.
  let extent_px = length_pt * pixels_per_point;
  let nearest_px = extent_px.round();
  let rounding_tolerance = f32::EPSILON * extent_px.abs().max(1.0) * 4.0;
  let stable_extent_px = if (extent_px - nearest_px).abs() <= rounding_tolerance {
    nearest_px
  } else {
    extent_px
  };
  (stable_extent_px.floor() + 1.0).max(1.0) as u32
}

fn raster_source_extent(length_pt: f32, pixels_per_point: f32, extent: RasterSourceExtent) -> u32 {
  let length_px = length_pt * pixels_per_point;
  match extent {
    RasterSourceExtent::Outward => raster_pixel_extent(length_pt, pixels_per_point),
    RasterSourceExtent::InclusiveFarEdge => {
      inclusive_far_edge_raster_pixel_extent(length_pt, pixels_per_point)
    }
    RasterSourceExtent::Floor => length_px.floor().max(1.0) as u32,
    RasterSourceExtent::Round => length_px.round().max(1.0) as u32,
  }
}

fn effect_pixels_per_point(width_pt: f32, height_pt: f32) -> f32 {
  effect_pixels_per_point_with_max(width_pt, height_pt, MAX_EFFECT_PIXELS_PER_POINT)
}

pub(crate) fn effect_pixels_per_point_with_max(
  width_pt: f32,
  height_pt: f32,
  max_pixels_per_point: f32,
) -> f32 {
  effect_pixels_per_point_with_budget(
    width_pt,
    height_pt,
    max_pixels_per_point,
    MAX_EFFECT_RASTER_PIXELS,
  )
}

fn effect_pixels_per_point_with_budget(
  width_pt: f32,
  height_pt: f32,
  max_pixels_per_point: f32,
  max_pixels: f32,
) -> f32 {
  (max_pixels / (width_pt * height_pt))
    .sqrt()
    .clamp(0.25, max_pixels_per_point.max(0.25))
}

/// Selects the bounded effect density and encloses a logical output rectangle
/// on that device-pixel grid.
///
/// Microsoft's Direct2D geometry-realization sample applies `floor` to the
/// left/top widened bounds and `ceil` to right/bottom before allocating an
/// opacity bitmap.  Doing this after density selection preserves the logical
/// effect range while ensuring fractional strokes and antialiasing are not
/// clipped at the source edge.  The density is rechecked after alignment so
/// the shared 250,000-pixel budget remains an actual upper bound.
pub(crate) fn bounded_effect_raster_grid(bounds: Rect, max_pixels_per_point: f32) -> (Rect, f32) {
  effect_raster_grid_with_budget(bounds, max_pixels_per_point, MAX_EFFECT_RASTER_PIXELS)
}

/// Word's two-dimensional fixed-output effects retain their requested device
/// density across the preview budget boundary. Twelve independently exported
/// picture/shadow size controls retain 200 DPI above one million pixels. The
/// larger cap here is our allocation safeguard, not an inferred Office limit.
pub(crate) fn fixed_output_effect_raster_grid(
  bounds: Rect,
  max_pixels_per_point: f32,
) -> (Rect, f32) {
  effect_raster_grid_with_budget(
    bounds,
    max_pixels_per_point,
    MAX_FIXED_OUTPUT_EFFECT_RASTER_PIXELS,
  )
}

fn effect_raster_grid_with_budget(
  bounds: Rect,
  max_pixels_per_point: f32,
  max_pixels: f32,
) -> (Rect, f32) {
  let mut pixels_per_point = effect_pixels_per_point_with_budget(
    bounds.size.width.0,
    bounds.size.height.0,
    max_pixels_per_point,
    max_pixels,
  );
  for _ in 0..3 {
    let aligned = align_rect_to_pixel_grid(bounds, pixels_per_point);
    let bounded = effect_pixels_per_point_with_budget(
      aligned.size.width.0,
      aligned.size.height.0,
      max_pixels_per_point,
      max_pixels,
    )
    .min(pixels_per_point);
    if (bounded - pixels_per_point).abs() <= f32::EPSILON {
      return (aligned, pixels_per_point);
    }
    pixels_per_point = bounded;
  }
  (
    align_rect_to_pixel_grid(bounds, pixels_per_point),
    pixels_per_point,
  )
}

fn align_rect_to_pixel_grid(bounds: Rect, pixels_per_point: f32) -> Rect {
  let left = (bounds.origin.x.0 * pixels_per_point).floor() / pixels_per_point;
  let top = (bounds.origin.y.0 * pixels_per_point).floor() / pixels_per_point;
  let right =
    ((bounds.origin.x.0 + bounds.size.width.0) * pixels_per_point).ceil() / pixels_per_point;
  let bottom =
    ((bounds.origin.y.0 + bounds.size.height.0) * pixels_per_point).ceil() / pixels_per_point;
  Rect {
    origin: super::Point {
      x: Pt(left),
      y: Pt(top),
    },
    size: super::Size {
      width: Pt(right - left),
      height: Pt(bottom - top),
    },
  }
}

#[cfg(test)]
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
  rasterize_vector_items_impl_at_pixels_per_point_with_extent(
    items,
    raster_bounds,
    pixels_per_point,
    RasterSourceExtent::Outward,
  )
}

fn rasterize_vector_items_impl_at_pixels_per_point_with_extent(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  pixels_per_point: f32,
  extent: RasterSourceExtent,
) -> Option<(RgbaImage, f32)> {
  rasterize_vector_items_impl_at_pixels_per_point_with_extent_and_antialiasing(
    items,
    raster_bounds,
    pixels_per_point,
    extent,
    RasterPrimitiveAntialiasing::PerPrimitive,
  )
}

fn rasterize_vector_items_impl_at_pixels_per_point_with_extent_and_antialiasing(
  items: &[DisplayItem<'static>],
  raster_bounds: Rect,
  pixels_per_point: f32,
  extent: RasterSourceExtent,
  primitive_antialiasing: RasterPrimitiveAntialiasing,
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
  let width_px = raster_source_extent(width_pt, pixels_per_point, extent);
  let height_px = raster_source_extent(height_pt, pixels_per_point, extent);
  let image = rasterize_vector_items_at_mapping(
    items,
    PageToRasterMapping {
      width_px,
      height_px,
      scale_x: pixels_per_point,
      scale_y: pixels_per_point,
      translate_x: -raster_bounds.origin.x.0 * pixels_per_point,
      translate_y: -raster_bounds.origin.y.0 * pixels_per_point,
      text_hinting: None,
    },
    primitive_antialiasing,
  )?;
  Some((image, pixels_per_point))
}

fn valid_raster_mapping(mapping: PageToRasterMapping) -> bool {
  mapping.width_px > 0
    && mapping.height_px > 0
    && mapping.scale_x.is_finite()
    && mapping.scale_x > 0.0
    && mapping.scale_y.is_finite()
    && mapping.scale_y > 0.0
    && mapping.translate_x.is_finite()
    && mapping.translate_y.is_finite()
}

/// Resolve a complete, painter-ordered vector scene once, rather than resolving
/// coverage independently for its adjoining faces. Independent source-over AA
/// exposes the background at an otherwise completely covered internal edge.
pub(crate) fn rasterize_vector_scene_at_mapping(
  items: &[DisplayItem<'static>],
  mapping: PageToRasterMapping,
) -> Option<RgbaImage> {
  if !valid_raster_mapping(mapping) || items.iter().any(|item| !supported_raster_item(item)) {
    return None;
  }
  let mut pixmap = Pixmap::new(mapping.width_px, mapping.height_px)?;
  let mut sums = vec![[0_u16; 4]; pixmap.pixels().len()];
  let mut text_metrics = TextMetrics::new();
  for (sample_x, sample_y) in super::drawingml_3d::DIRECT3D_STANDARD_8_SAMPLES {
    pixmap.fill(SkColor::TRANSPARENT);
    let transform = SkTransform::from_row(
      mapping.scale_x,
      0.0,
      0.0,
      mapping.scale_y,
      mapping.translate_x + 0.5 - sample_x,
      mapping.translate_y + 0.5 - sample_y,
    );
    for item in items {
      draw_display_item(
        &mut pixmap,
        item,
        transform,
        mapping.text_hinting,
        RasterPrimitiveAntialiasing::Aliased,
        &mut text_metrics,
      )?;
    }
    for (sum, pixel) in sums.iter_mut().zip(pixmap.data().as_chunks::<4>().0.iter()) {
      for (total, value) in sum.iter_mut().zip(pixel) {
        *total += u16::from(*value);
      }
    }
  }
  let mut output = RgbaImage::new(mapping.width_px, mapping.height_px);
  for (pixel, sum) in output.pixels_mut().zip(sums) {
    let alpha = u32::from(sum[3]);
    if alpha == 0 {
      continue;
    }
    for channel in 0..3 {
      pixel[channel] = ((u32::from(sum[channel]) * 255 + alpha / 2) / alpha).min(255) as u8;
    }
    pixel[3] = ((alpha + 4) / 8) as u8;
  }
  Some(output)
}

/// Paints vectors on exactly the supplied lattice and primitive sample grid.
/// Unlike the high-density convenience entry point, this does not choose an
/// extra supersampling factor or resize a previously realized bitmap. Surface
/// allocation/budget policy belongs to the caller that selected this mapping.
pub(crate) fn rasterize_vector_items_at_mapping(
  items: &[DisplayItem<'static>],
  mapping: PageToRasterMapping,
  primitive_antialiasing: RasterPrimitiveAntialiasing,
) -> Option<RgbaImage> {
  if !valid_raster_mapping(mapping) || items.iter().any(|item| !supported_raster_item(item)) {
    return None;
  }
  let (width_px, height_px) = (mapping.width_px, mapping.height_px);
  let (horizontal_sample_factor, vertical_sample_factor) = primitive_antialiasing.sample_factors();
  let source_width_px = width_px.checked_mul(horizontal_sample_factor)?;
  let source_height_px = height_px.checked_mul(vertical_sample_factor)?;
  let horizontal_scale = horizontal_sample_factor as f32;
  let vertical_scale = vertical_sample_factor as f32;
  let mut pixmap = Pixmap::new(source_width_px, source_height_px)?;
  let mut text_metrics = TextMetrics::new();
  let page_to_raster = SkTransform::from_row(
    mapping.scale_x * horizontal_scale,
    0.0,
    0.0,
    mapping.scale_y * vertical_scale,
    mapping.translate_x * horizontal_scale,
    mapping.translate_y * vertical_scale,
  );

  for item in items {
    draw_display_item(
      &mut pixmap,
      item,
      page_to_raster,
      mapping.text_hinting,
      primitive_antialiasing,
      &mut text_metrics,
    )?;
  }

  let image = pixmap_into_rgba(pixmap)?;
  let image = if horizontal_sample_factor == 1 && vertical_sample_factor == 1 {
    image
  } else {
    resolve_box_filtered_rgba(
      &image,
      width_px,
      height_px,
      horizontal_sample_factor,
      vertical_sample_factor,
    )
  };
  Some(image)
}

fn rasterize_vector_items_impl_with_mapping(
  items: &[DisplayItem<'static>],
  mapping: PageToRasterMapping,
) -> Option<RgbaImage> {
  if mapping.width_px == 0
    || mapping.height_px == 0
    || !mapping.scale_x.is_finite()
    || !mapping.scale_y.is_finite()
    || !mapping.translate_x.is_finite()
    || !mapping.translate_y.is_finite()
    || mapping.scale_x <= 0.0
    || mapping.scale_y <= 0.0
    || items.iter().any(|item| !supported_raster_item(item))
  {
    return None;
  }

  // tiny-skia's path scanner uses a 4x4 coverage grid. Word's fixed-output
  // static-3-D surface exposes finer AntiAlias8x coverage (the controlled
  // tdf97371 target has 112/105 alpha edges, neither representable by the
  // native 4x4 grid). Realize this explicitly mapped, typically tiny surface
  // at a bounded higher resolution and resolve premultiplied coverage. The
  // shared effect-pixel budget keeps large shapes from multiplying memory.
  let final_pixels = u64::from(mapping.width_px) * u64::from(mapping.height_px);
  let budget_factor = ((f64::from(MAX_EFFECT_RASTER_PIXELS) / final_pixels as f64)
    .sqrt()
    .floor() as u32)
    .clamp(1, 8);
  if budget_factor > 1 {
    let scale = budget_factor as f32;
    let supersampled = rasterize_vector_items_impl_with_mapping_at_resolution(
      items,
      PageToRasterMapping {
        width_px: mapping.width_px.checked_mul(budget_factor)?,
        height_px: mapping.height_px.checked_mul(budget_factor)?,
        scale_x: mapping.scale_x * scale,
        scale_y: mapping.scale_y * scale,
        translate_x: mapping.translate_x * scale,
        translate_y: mapping.translate_y * scale,
        text_hinting: mapping.text_hinting,
      },
    )?;
    return Some(resolve_box_filtered_rgba(
      &supersampled,
      mapping.width_px,
      mapping.height_px,
      budget_factor,
      budget_factor,
    ));
  }

  rasterize_vector_items_impl_with_mapping_at_resolution(items, mapping)
}

fn rasterize_vector_items_impl_with_mapping_at_resolution(
  items: &[DisplayItem<'static>],
  mapping: PageToRasterMapping,
) -> Option<RgbaImage> {
  let mut pixmap = Pixmap::new(mapping.width_px, mapping.height_px)?;
  let mut text_metrics = TextMetrics::new();
  let page_to_raster = SkTransform::from_row(
    mapping.scale_x,
    0.0,
    0.0,
    mapping.scale_y,
    mapping.translate_x,
    mapping.translate_y,
  );
  for item in items {
    draw_display_item(
      &mut pixmap,
      item,
      page_to_raster,
      mapping.text_hinting,
      RasterPrimitiveAntialiasing::PerPrimitive,
      &mut text_metrics,
    )?;
  }

  pixmap_into_rgba(pixmap)
}

fn pixmap_into_rgba(pixmap: Pixmap) -> Option<RgbaImage> {
  let (width, height) = (pixmap.width(), pixmap.height());
  // tiny-skia's PNG encoder uses this exact demultiplication before encoding.
  // Transfer its buffer directly: an in-memory effect source needs neither
  // PNG compression nor a second allocation and decode of the same pixels.
  RgbaImage::from_raw(width, height, pixmap.take_demultiplied())
}

fn resolve_box_filtered_rgba(
  source: &RgbaImage,
  width: u32,
  height: u32,
  horizontal_factor: u32,
  vertical_factor: u32,
) -> RgbaImage {
  debug_assert!(horizontal_factor > 0);
  debug_assert!(vertical_factor > 0);
  debug_assert_eq!(source.width(), width * horizontal_factor);
  debug_assert_eq!(source.height(), height * vertical_factor);
  let sample_count = u64::from(horizontal_factor) * u64::from(vertical_factor);
  RgbaImage::from_fn(width, height, |x, y| {
    let mut alpha_sum = 0_u64;
    let mut premultiplied_sum = [0_u64; 3];
    for sample_y in y * vertical_factor..(y + 1) * vertical_factor {
      for sample_x in x * horizontal_factor..(x + 1) * horizontal_factor {
        let pixel = source.get_pixel(sample_x, sample_y);
        let alpha = u64::from(pixel[3]);
        alpha_sum += alpha;
        for channel in 0..3 {
          premultiplied_sum[channel] += u64::from(pixel[channel]) * alpha;
        }
      }
    }
    if alpha_sum == 0 {
      return image::Rgba([0; 4]);
    }
    let alpha = ((alpha_sum + sample_count / 2) / sample_count).min(255) as u8;
    let color = premultiplied_sum.map(|sum| ((sum + alpha_sum / 2) / alpha_sum).min(255) as u8);
    image::Rgba([color[0], color[1], color[2], alpha])
  })
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
  text_hinting: Option<(RasterTextHinting, f32)>,
  primitive_antialiasing: RasterPrimitiveAntialiasing,
  text_metrics: &mut TextMetrics,
) -> Option<()> {
  match item {
    DisplayItem::Text(text) => draw_text(
      pixmap,
      text,
      page_to_raster,
      text_hinting,
      primitive_antialiasing,
      text_metrics,
    )?,
    DisplayItem::Image(image) => draw_image(pixmap, image, page_to_raster, primitive_antialiasing)?,
    DisplayItem::Path(path) => draw_path(pixmap, path, page_to_raster, primitive_antialiasing)?,
    DisplayItem::Rect(rect) => draw_rect(pixmap, rect, page_to_raster, primitive_antialiasing)?,
    DisplayItem::Line(line) => draw_line(pixmap, line, page_to_raster, primitive_antialiasing)?,
    DisplayItem::Group(group) => {
      for child in &group.items {
        draw_display_item(
          pixmap,
          child,
          page_to_raster,
          text_hinting,
          primitive_antialiasing,
          text_metrics,
        )?;
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
  text_hinting: Option<(RasterTextHinting, f32)>,
  primitive_antialiasing: RasterPrimitiveAntialiasing,
  text_metrics: &mut TextMetrics,
) -> Option<()> {
  let primitive_antialiasing = match primitive_antialiasing {
    RasterPrimitiveAntialiasing::Direct2dStandard4 => RasterPrimitiveAntialiasing::PerPrimitive,
    other => other,
  };
  let outline = text_outline(item, text_hinting, text_metrics)?;
  let commands = outline.commands;
  if commands.is_empty() {
    return Some(());
  }
  let path = path_from_commands(&commands, &[], true)?;
  let definition_width = item.style.pdf_glyph_outline_options.as_ref().map_or(
    super::Pt(outline.width_pt.max(item.style.font_size.0)),
    |options| {
      options.unresolved_definition_width(super::Pt(outline.width_pt), item.style.font_size)
    },
  );
  let bounds = Rect {
    origin: super::Point {
      x: item.origin.x,
      y: item.origin.y,
    },
    size: super::Size {
      width: definition_width,
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
  if primitive_antialiasing == RasterPrimitiveAntialiasing::OfficeAntiAlias8x4 {
    draw_office_8x4_fill(
      pixmap,
      &path,
      &fill,
      bounds,
      Some(&commands),
      page_to_raster,
    )?;
  } else {
    draw_fill(
      pixmap,
      &path,
      &fill,
      bounds,
      Some(&commands),
      page_to_raster,
      primitive_antialiasing,
    )?;
  }
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
    if primitive_antialiasing == RasterPrimitiveAntialiasing::OfficeAntiAlias8x4 {
      draw_office_8x4_stroke(
        pixmap,
        &path,
        stroke,
        bounds,
        Some(&commands),
        page_to_raster,
      )?;
    } else {
      draw_stroke(
        pixmap,
        &path,
        stroke,
        bounds,
        Some(&commands),
        page_to_raster,
        primitive_antialiasing,
      )?;
    }
  }
  Some(())
}

struct RasterTextOutline {
  commands: Vec<PathCommand>,
  width_pt: f32,
}

fn text_outline(
  item: &TextRun<'static>,
  text_hinting: Option<(RasterTextHinting, f32)>,
  text_metrics: &mut TextMetrics,
) -> Option<RasterTextOutline> {
  if item.style.semantic_only || item.style.hidden || item.text.is_empty() {
    return Some(RasterTextOutline {
      commands: Vec::new(),
      width_pt: 0.0,
    });
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
  let gdi_device_advances = text_hinting
    .filter(|(mode, _)| {
      !item.paragraph_bidi
        && matches!(
          mode,
          RasterTextHinting::GdiDeviceAdvances | RasterTextHinting::GdiDeviceAdvancesHinted
        )
    })
    .and_then(|(_, device_pixels_per_point)| {
      text_metrics.gdi_device_character_advances_pt(
        item.text.as_ref(),
        &item.style,
        device_pixels_per_point * crate::units::POINTS_PER_INCH,
      )
    });
  let mut commands = Vec::new();
  let mut cursor_x = item.origin.x.0;
  for (glyph_index, glyph) in shaped.glyphs.iter().enumerate() {
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
    let hinted_ppem = text_hinting.and_then(|(mode, device_pixels_per_point)| {
      let ppem = glyph.font_size_pt * device_pixels_per_point;
      Some(match mode {
        RasterTextHinting::RoundedDevicePpem
        | RasterTextHinting::RoundedDevicePpemPreserveLinear => ppem.round(),
        RasterTextHinting::ExactDevicePpem
        | RasterTextHinting::ExactDevicePpemPreserveLinear
        | RasterTextHinting::ExactDevicePpemAsymmetric
        | RasterTextHinting::ExactDevicePpemLight
        | RasterTextHinting::ExactDevicePpemLcd
        | RasterTextHinting::ExactDevicePpemMono
        | RasterTextHinting::GdiDeviceAdvancesHinted => ppem,
        RasterTextHinting::GdiDeviceAdvances => return None,
      })
    });
    let mut outline = RasterGlyphOutline {
      commands: &mut commands,
      origin_x,
      origin_y,
      scale: hinted_ppem.map_or(glyph.font_size_pt / units_per_em, |ppem| {
        glyph.font_size_pt / ppem
      }),
      horizontal_scale,
      synthetic_italic: face_data.synthetic_italic,
      rotation_degrees: item.style.rotation_degrees,
      rotation_center: item.rotation_center,
      current: None,
    };
    if let Some(glyph_outline) = face.outline_glyphs().get(GlyphId::new(glyph.glyph_id)) {
      if let Some(ppem) = hinted_ppem {
        let outlines = face.outline_glyphs();
        let preserve_linear_metrics = text_hinting.is_some_and(|(mode, _)| {
          matches!(
            mode,
            RasterTextHinting::RoundedDevicePpemPreserveLinear
              | RasterTextHinting::ExactDevicePpemPreserveLinear
          )
        });
        let target = match text_hinting.map(|(mode, _)| mode) {
          Some(RasterTextHinting::ExactDevicePpemAsymmetric) => Target::Smooth {
            mode: SmoothMode::Normal,
            symmetric_rendering: false,
            preserve_linear_metrics: false,
          },
          Some(RasterTextHinting::ExactDevicePpemLight) => Target::Smooth {
            mode: SmoothMode::Light,
            symmetric_rendering: true,
            preserve_linear_metrics: false,
          },
          Some(RasterTextHinting::ExactDevicePpemLcd) => Target::Smooth {
            mode: SmoothMode::Lcd,
            symmetric_rendering: true,
            preserve_linear_metrics: false,
          },
          Some(RasterTextHinting::ExactDevicePpemMono) => Target::Mono,
          _ => Target::Smooth {
            mode: SmoothMode::Normal,
            symmetric_rendering: true,
            preserve_linear_metrics,
          },
        };
        let instance = HintingInstance::new(
          &outlines,
          Size::new(ppem),
          LocationRef::default(),
          HintingOptions {
            target,
            ..HintingOptions::default()
          },
        )
        .ok()?;
        let _ = glyph_outline.draw(DrawSettings::hinted(&instance, false), &mut outline);
      } else {
        let _ = glyph_outline.draw(
          DrawSettings::unhinted(Size::unscaled(), LocationRef::default()),
          &mut outline,
        );
      }
    }
    cursor_x += gdi_device_advances
      .as_deref()
      .and_then(|advances| advances.get(glyph_index))
      .copied()
      .unwrap_or(glyph.x_advance_em * glyph.font_size_pt);
    if item
      .text
      .get(glyph.text_range.clone())
      .is_some_and(|cluster| cluster.contains(' '))
    {
      cursor_x += item.word_spacing_pt;
    }
  }
  Some(RasterTextOutline {
    commands,
    width_pt: shaped.width_pt,
  })
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
      *path = super::office_circle_gradient_path(*path);
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
  primitive_antialiasing: RasterPrimitiveAntialiasing,
) -> Option<()> {
  let raster_data = crate::render::emf_wmf::decode_metafile_as_raster(
    item.bytes.as_ref(),
    Some(item.content_type.as_ref()),
  )
  .ok()
  .flatten()
  .map(|decoded| decoded.data);
  let source_data = raster_data.as_deref().unwrap_or(item.bytes.as_ref());
  // Fixed-output image effects must consume the same decoded JPEG samples
  // as direct PDF pictures. The generic decoder uses a different IDCT;
  // reflecting or blurring its RGB cannot recover the Office source plane.
  let source = if let Some(rgb) = crate::render::jpeg_islow::decode_rgb(source_data) {
    image::DynamicImage::ImageRgb8(rgb).to_rgba8()
  } else if let Some(gray) = crate::render::jpeg_islow::decode_gray(source_data) {
    image::DynamicImage::ImageLuma8(gray).to_rgba8()
  } else {
    image::load_from_memory(source_data).ok()?.to_rgba8()
  };
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
    mask_paint.anti_alias = primitive_antialiasing.enabled();
    if primitive_antialiasing == RasterPrimitiveAntialiasing::Direct2dStandard4 {
      let coverage =
        direct2d_four_sample_mask(pixmap, &clip_path, FillRule::EvenOdd, page_to_raster)?;
      paint_sample_mask(&mut mask, &coverage, mask_paint, page_to_raster)?;
    } else {
      mask.fill_path(
        &clip_path,
        &mask_paint,
        FillRule::EvenOdd,
        page_to_raster,
        None,
      );
    }
  }

  let raster_to_page = page_to_raster.invert()?;
  let center_x = item.bounds.origin.x.0 + width_pt * 0.5;
  let center_y = item.bounds.origin.y.0 + height_pt * 0.5;
  let angle = item.rotation_degrees.to_radians();
  let (sin, cos) = angle.sin_cos();
  let source_width = source.width() as f32;
  let source_height = source.height() as f32;

  for y in 0..pixmap.height() {
    for x in 0..pixmap.width() {
      let mask_alpha = mask.pixel(x, y)?.alpha();
      if mask_alpha == 0 {
        continue;
      }
      let page = raster_pixel_center_in_page(raster_to_page, x, y);
      let page_x = page.x;
      let page_y = page.y;
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

#[inline]
fn raster_pixel_center_in_page(raster_to_page: SkTransform, x: u32, y: u32) -> SkPoint {
  let raster_x = x as f32 + 0.5;
  let raster_y = y as f32 + 0.5;
  SkPoint::from_xy(
    raster_to_page.sx.mul_add(
      raster_x,
      raster_to_page.kx.mul_add(raster_y, raster_to_page.tx),
    ),
    raster_to_page.ky.mul_add(
      raster_x,
      raster_to_page.sy.mul_add(raster_y, raster_to_page.ty),
    ),
  )
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
  primitive_antialiasing: RasterPrimitiveAntialiasing,
) -> Option<()> {
  let path = path_from_commands(&item.commands, &item.points, item.closed)?;
  draw_fill(
    pixmap,
    &path,
    &item.fill,
    item.bounds,
    Some(&item.commands),
    page_to_raster,
    primitive_antialiasing,
  )?;
  if let Some(stroke) = &item.stroke {
    if primitive_antialiasing == RasterPrimitiveAntialiasing::Direct2dStandard4
      && stroke
        .drawingml_device
        .as_ref()
        .is_some_and(|source| source.realized_width_emu.is_some())
    {
      return draw_device_stroke(
        pixmap,
        &path,
        stroke,
        item.bounds,
        Some(&item.commands),
        Some(item),
        page_to_raster,
      );
    }
    let shortened_path = shortened_straight_stroke_path(item, stroke);
    draw_stroke(
      pixmap,
      shortened_path.as_ref().unwrap_or(&path),
      stroke,
      item.bounds,
      Some(&item.commands),
      page_to_raster,
      primitive_antialiasing,
    )?;
    draw_stroke_end_markers(pixmap, item, stroke, page_to_raster, primitive_antialiasing)?;
  }
  Some(())
}

fn draw_stroke_end_markers(
  pixmap: &mut Pixmap,
  item: &PathItem<'static>,
  stroke: &Stroke<'static>,
  page_to_raster: SkTransform,
  antialiasing: RasterPrimitiveAntialiasing,
) -> Option<()> {
  let mut paint = solid_paint(stroke.color);
  paint.anti_alias = antialiasing.enabled();
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
    if antialiasing == RasterPrimitiveAntialiasing::Direct2dStandard4 {
      let mask = direct2d_four_sample_mask(pixmap, &path, FillRule::EvenOdd, page_to_raster)?;
      paint_sample_mask(pixmap, &mask, paint.clone(), page_to_raster)?;
    } else {
      pixmap.fill_path(&path, &paint, FillRule::EvenOdd, page_to_raster, None);
    }
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
    if antialiasing == RasterPrimitiveAntialiasing::Direct2dStandard4 {
      let expanded = expanded_stroke_path(&path, &sk_stroke, page_to_raster)?;
      let mask = direct2d_four_sample_mask(pixmap, &expanded, FillRule::Winding, page_to_raster)?;
      paint_sample_mask(pixmap, &mask, paint.clone(), page_to_raster)?;
    } else {
      pixmap.stroke_path(&path, &paint, &sk_stroke, page_to_raster, None);
    }
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
  primitive_antialiasing: RasterPrimitiveAntialiasing,
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
  draw_fill(
    pixmap,
    &path,
    &item.fill,
    item.bounds,
    None,
    page_to_raster,
    primitive_antialiasing,
  )?;
  if let Some(stroke) = &item.stroke {
    draw_stroke(
      pixmap,
      &path,
      stroke,
      item.bounds,
      None,
      page_to_raster,
      primitive_antialiasing,
    )?;
  }
  Some(())
}

fn draw_line(
  pixmap: &mut Pixmap,
  item: &LineItem<'static>,
  page_to_raster: SkTransform,
  primitive_antialiasing: RasterPrimitiveAntialiasing,
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
  draw_stroke(
    pixmap,
    &path,
    &item.stroke,
    bounds,
    None,
    page_to_raster,
    primitive_antialiasing,
  )
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
  antialiasing: RasterPrimitiveAntialiasing,
) -> Option<()> {
  if antialiasing == RasterPrimitiveAntialiasing::OfficeAntiAlias8x4 {
    return draw_office_8x4_fill(pixmap, path, fill, bounds, commands, page_to_raster);
  }
  if antialiasing == RasterPrimitiveAntialiasing::Direct2dStandard4 {
    if matches!(fill, Fill::None) {
      return Some(());
    }
    let mask = direct2d_four_sample_mask(pixmap, path, FillRule::EvenOdd, page_to_raster)?;
    return paint_fill_mask(pixmap, &mask, fill, bounds, commands, page_to_raster);
  }
  let anti_alias = antialiasing.enabled();
  match fill {
    Fill::None => Some(()),
    Fill::Solid(color) => {
      let mut paint = solid_paint(*color);
      paint.anti_alias = anti_alias;
      pixmap.fill_path(path, &paint, FillRule::EvenOdd, page_to_raster, None);
      Some(())
    }
    Fill::Gradient(gradient) if gradient.path.is_none() => {
      let mut paint = linear_gradient_paint(gradient, bounds)?;
      paint.anti_alias = anti_alias;
      pixmap.fill_path(path, &paint, FillRule::EvenOdd, page_to_raster, None);
      Some(())
    }
    Fill::Gradient(gradient) => {
      draw_path_gradient(pixmap, path, gradient, commands, page_to_raster, anti_alias)
    }
    Fill::Pattern(pattern) => {
      let tile = pattern_tile(*pattern, page_to_raster.sx)?;
      let paint = Paint {
        anti_alias,
        shader: Pattern::new(
          tile.as_ref(),
          SpreadMode::Repeat,
          FilterQuality::Nearest,
          1.0,
          SkTransform::from_translate(
            pattern_origin(bounds.origin.x.0, pattern.tile_size_points()),
            pattern_origin(bounds.origin.y.0, pattern.tile_size_points()),
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

/// Rasterizes Word's fixed-output text-effect path samples on the GDI+
/// `SmoothingModeAntiAlias8x4` grid.
///
/// GDI+ locates `PixelOffsetModeNone` samples at device-grid coordinates,
/// while tiny-skia's aliased scanner classifies pixel centers.  Use the fast
/// tiny-skia mask to identify the narrow boundary band, then replace only that
/// band with exact Bézier winding tests at integer device coordinates.  The
/// surrounding 8x4 surface remains binary and is resolved by the shared
/// premultiplied box filter.
fn office_8x4_sample_mask(
  pixmap: &Pixmap,
  path: &Path,
  fill_rule: FillRule,
  page_to_raster: SkTransform,
) -> Option<Mask> {
  let transformed = path
    .clone()
    .transform(office_8x4_storage_transform(page_to_raster))?;
  let mut mask = Mask::new(pixmap.width(), pixmap.height())?;
  mask.fill_path(&transformed, fill_rule, false, SkTransform::identity());
  let preliminary = mask.data().to_vec();
  let exact_path = tiny_path_as_kurbo(&transformed);
  let width = pixmap.width() as usize;
  let height = pixmap.height() as usize;

  for y in 0..height {
    for x in 0..width {
      let index = y * width + x;
      let preliminary_alpha = preliminary[index];
      let mut boundary = false;
      for offset_y in -1_i32..=1 {
        for offset_x in -1_i32..=1 {
          if offset_x == 0 && offset_y == 0 {
            continue;
          }
          let neighbor_x = x as i32 + offset_x;
          let neighbor_y = y as i32 + offset_y;
          let neighbor_alpha = if neighbor_x < 0
            || neighbor_y < 0
            || neighbor_x >= width as i32
            || neighbor_y >= height as i32
          {
            0
          } else {
            preliminary[neighbor_y as usize * width + neighbor_x as usize]
          };
          if neighbor_alpha != preliminary_alpha {
            boundary = true;
            break;
          }
        }
        if boundary {
          break;
        }
      }
      if !boundary {
        continue;
      }

      let winding = exact_path.winding(KurboPoint::new(x as f64, y as f64));
      let inside = match fill_rule {
        FillRule::Winding => winding != 0,
        FillRule::EvenOdd => winding.rem_euclid(2) != 0,
      };
      mask.data_mut()[index] = if inside { 255 } else { 0 };
    }
  }
  Some(mask)
}

fn tiny_path_as_kurbo(path: &Path) -> BezPath {
  let mut output = BezPath::new();
  for segment in path.segments() {
    match segment {
      PathSegment::MoveTo(point) => {
        output.move_to(KurboPoint::new(f64::from(point.x), f64::from(point.y)));
      }
      PathSegment::LineTo(point) => {
        output.line_to(KurboPoint::new(f64::from(point.x), f64::from(point.y)));
      }
      PathSegment::QuadTo(control, end) => {
        output.quad_to(
          KurboPoint::new(f64::from(control.x), f64::from(control.y)),
          KurboPoint::new(f64::from(end.x), f64::from(end.y)),
        );
      }
      PathSegment::CubicTo(control1, control2, end) => {
        output.curve_to(
          KurboPoint::new(f64::from(control1.x), f64::from(control1.y)),
          KurboPoint::new(f64::from(control2.x), f64::from(control2.y)),
          KurboPoint::new(f64::from(end.x), f64::from(end.y)),
        );
      }
      PathSegment::Close => output.close_path(),
    }
  }
  output
}

fn office_8x4_storage_transform(mut page_to_raster: SkTransform) -> SkTransform {
  // A GDI+ device pixel is centered on its integer coordinate.  Its 8x4 box
  // therefore spans samples [-4..3] x [-2..1] in the high-resolution storage
  // grid, while a conventional array block spans [0..7] x [0..3].
  page_to_raster.tx += OFFICE_ANTIALIAS_8X4_HORIZONTAL_SAMPLES as f32 * 0.5;
  page_to_raster.ty += OFFICE_ANTIALIAS_8X4_VERTICAL_SAMPLES as f32 * 0.5;
  page_to_raster
}

fn office_8x4_sample_transform(mut page_to_raster: SkTransform) -> SkTransform {
  page_to_raster = office_8x4_storage_transform(page_to_raster);
  // tiny-skia shades storage pixels at their centers; the extra half sample
  // evaluates paint at the integer-coordinate GDI+ sample represented there.
  page_to_raster.tx += 0.5;
  page_to_raster.ty += 0.5;
  page_to_raster
}

fn paint_sample_mask(
  pixmap: &mut Pixmap,
  mask: &Mask,
  mut paint: Paint<'_>,
  page_to_raster: SkTransform,
) -> Option<()> {
  paint.anti_alias = false;
  paint.shader.transform(page_to_raster);
  let rect = SkRect::from_xywh(0.0, 0.0, pixmap.width() as f32, pixmap.height() as f32)?;
  pixmap.fill_rect(rect, &paint, SkTransform::identity(), Some(mask));
  Some(())
}

fn draw_office_8x4_fill(
  pixmap: &mut Pixmap,
  path: &Path,
  fill: &Fill<'static>,
  bounds: Rect,
  commands: Option<&[PathCommand]>,
  page_to_raster: SkTransform,
) -> Option<()> {
  if matches!(fill, Fill::None) {
    return Some(());
  }
  let mask = office_8x4_sample_mask(pixmap, path, FillRule::EvenOdd, page_to_raster)?;
  paint_fill_mask(
    pixmap,
    &mask,
    fill,
    bounds,
    commands,
    office_8x4_sample_transform(page_to_raster),
  )
}

fn direct2d_four_sample_mask(
  pixmap: &Pixmap,
  path: &Path,
  fill_rule: FillRule,
  page_to_raster: SkTransform,
) -> Option<Mask> {
  let transformed = path.clone().transform(page_to_raster)?;
  super::drawingml_direct2d_raster::standard_four_sample_mask(
    &tiny_path_as_kurbo(&transformed),
    pixmap.width(),
    pixmap.height(),
    fill_rule,
  )
}

fn paint_fill_mask(
  pixmap: &mut Pixmap,
  mask: &Mask,
  fill: &Fill<'static>,
  bounds: Rect,
  commands: Option<&[PathCommand]>,
  page_to_raster: SkTransform,
) -> Option<()> {
  match fill {
    Fill::None => Some(()),
    Fill::Solid(color) => paint_sample_mask(pixmap, mask, solid_paint(*color), page_to_raster),
    Fill::Gradient(gradient) if gradient.path.is_none() => paint_sample_mask(
      pixmap,
      mask,
      linear_gradient_paint(gradient, bounds)?,
      page_to_raster,
    ),
    Fill::Gradient(gradient) => {
      composite_path_gradient_byte_mask(pixmap, mask, gradient, commands, page_to_raster)
    }
    Fill::Pattern(pattern) => {
      let tile = pattern_tile(*pattern, page_to_raster.sx)?;
      let paint = Paint {
        shader: Pattern::new(
          tile.as_ref(),
          SpreadMode::Repeat,
          FilterQuality::Nearest,
          1.0,
          SkTransform::from_translate(
            pattern_origin(bounds.origin.x.0, pattern.tile_size_points()),
            pattern_origin(bounds.origin.y.0, pattern.tile_size_points()),
          ),
        ),
        ..Paint::default()
      };
      paint_sample_mask(pixmap, mask, paint, page_to_raster)
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
  anti_alias: bool,
) -> Option<()> {
  let mut mask = Pixmap::new(pixmap.width(), pixmap.height())?;
  let mut paint = Paint::default();
  paint.set_color_rgba8(255, 255, 255, 255);
  paint.anti_alias = anti_alias;
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
  composite_path_gradient_with_alpha(pixmap, gradient, commands, page_to_raster, |x, y| {
    mask.pixel(x, y).map(|pixel| pixel.alpha())
  })
}

fn composite_path_gradient_byte_mask(
  pixmap: &mut Pixmap,
  mask: &Mask,
  gradient: &GradientFill<'static>,
  commands: Option<&[PathCommand]>,
  page_to_raster: SkTransform,
) -> Option<()> {
  let width = mask.width() as usize;
  composite_path_gradient_with_alpha(pixmap, gradient, commands, page_to_raster, |x, y| {
    mask.data().get(y as usize * width + x as usize).copied()
  })
}

fn composite_path_gradient_with_alpha(
  pixmap: &mut Pixmap,
  gradient: &GradientFill<'static>,
  commands: Option<&[PathCommand]>,
  page_to_raster: SkTransform,
  mut mask_alpha_at: impl FnMut(u32, u32) -> Option<u8>,
) -> Option<()> {
  let gradient_path = gradient.path?;
  if gradient.stops.is_empty() {
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
  let raster_to_page = page_to_raster.invert()?;
  for y in 0..pixmap.height() {
    for x in 0..pixmap.width() {
      let mask_alpha = mask_alpha_at(x, y)?;
      if mask_alpha == 0 {
        continue;
      }
      let page = raster_pixel_center_in_page(raster_to_page, x, y);
      let point = super::drawingml_gradient::inverse_point(
        gradient_path.transform,
        f64::from(page.x),
        f64::from(page.y),
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
  antialiasing: RasterPrimitiveAntialiasing,
) -> Option<()> {
  if antialiasing == RasterPrimitiveAntialiasing::OfficeAntiAlias8x4 {
    return draw_office_8x4_stroke(pixmap, path, stroke, bounds, commands, page_to_raster);
  }
  if antialiasing == RasterPrimitiveAntialiasing::Direct2dStandard4
    && stroke
      .drawingml_device
      .as_ref()
      .is_some_and(|source| source.realized_width_emu.is_some())
  {
    return draw_device_stroke(pixmap, path, stroke, bounds, commands, None, page_to_raster);
  }
  if stroke.width.0 <= 0.0
    || stroke.color.a == 0 && stroke.pattern.is_none() && stroke.gradient.is_none()
  {
    return Some(());
  }
  let sk_stroke = resolved_sk_stroke(stroke);
  if antialiasing == RasterPrimitiveAntialiasing::Direct2dStandard4 {
    let expanded = expanded_stroke_path(path, &sk_stroke, page_to_raster)?;
    let mask = direct2d_four_sample_mask(pixmap, &expanded, FillRule::Winding, page_to_raster)?;
    return paint_stroke_mask(pixmap, &mask, stroke, bounds, commands, page_to_raster);
  }
  let anti_alias = antialiasing.enabled();
  if let Some(gradient) = stroke.gradient.as_ref() {
    if gradient.path.is_some() {
      let mut mask = Pixmap::new(pixmap.width(), pixmap.height())?;
      let mut paint = Paint::default();
      paint.set_color_rgba8(255, 255, 255, 255);
      paint.anti_alias = anti_alias;
      mask.stroke_path(path, &paint, &sk_stroke, page_to_raster, None);
      composite_path_gradient_mask(pixmap, &mask, gradient, commands, page_to_raster)?;
    } else {
      let mut paint = linear_gradient_paint(gradient, bounds)?;
      paint.anti_alias = anti_alias;
      pixmap.stroke_path(path, &paint, &sk_stroke, page_to_raster, None);
    }
  } else if let Some(pattern) = stroke.pattern {
    let tile = pattern_tile(pattern, page_to_raster.sx)?;
    let paint = Paint {
      anti_alias,
      shader: Pattern::new(
        tile.as_ref(),
        SpreadMode::Repeat,
        FilterQuality::Nearest,
        1.0,
        SkTransform::from_translate(
          pattern_origin(bounds.origin.x.0, pattern.tile_size_points()),
          pattern_origin(bounds.origin.y.0, pattern.tile_size_points()),
        ),
      ),
      ..Paint::default()
    };
    pixmap.stroke_path(path, &paint, &sk_stroke, page_to_raster, None);
  } else {
    let mut paint = solid_paint(stroke.color);
    paint.anti_alias = anti_alias;
    pixmap.stroke_path(path, &paint, &sk_stroke, page_to_raster, None);
  }
  Some(())
}

/// Widen in the original pen frame, resolve its mask on the device lattice,
/// then paint in the original page frame. This retains anisotropic pens
/// without moving gradients/patterns or inferring a shape axis from its path.
fn draw_device_stroke(
  pixmap: &mut Pixmap,
  path: &Path,
  stroke: &Stroke<'static>,
  bounds: Rect,
  commands: Option<&[PathCommand]>,
  item: Option<&PathItem<'static>>,
  page_to_raster: SkTransform,
) -> Option<()> {
  if stroke.color.a == 0 && stroke.pattern.is_none() && stroke.gradient.is_none() {
    return Some(());
  }
  let source = stroke.drawingml_device.as_ref()?;
  let [a, b, c, d] = source.emu_to_points.map(|value| value * 12_700.0);
  let local_to_page = Affine::new([a, b, c, d, 0.0, 0.0]);
  if local_to_page.determinant() == 0.0 || !local_to_page.is_finite() {
    return None;
  }
  let page_to_local = local_to_page.inverse();
  let local_to_raster = page_to_raster.pre_concat(SkTransform::from_row(
    a as f32, b as f32, c as f32, d as f32, 0.0, 0.0,
  ));
  let mut realized = stroke.clone();
  // The native paint object stores the helper's result in f32 EMUs before
  // widening. Preserve that boundary before converting to local point units.
  realized.width = super::Pt(source.realized_width_emu? as f32 / 12_700.0);
  realized.drawingml_device = None;
  let local_commands = super::drawingml_geometry::path_elements_to_commands(
    (page_to_local * tiny_path_as_kurbo(path)).iter(),
  );
  let local_path = path_from_commands(&local_commands, &[], false)?;
  let local_item = item.map(|item| {
    let mut local = item.clone();
    local.commands =
      super::drawingml_geometry::transform_commands(item.commands.clone(), page_to_local);
    local.points = item
      .points
      .iter()
      .map(|point| super::drawingml_geometry::transform_point(*point, page_to_local))
      .collect();
    local.stroke = Some(realized.clone());
    local
  });
  let shortened = local_item
    .as_ref()
    .and_then(|item| shortened_straight_stroke_path(item, &realized));
  let expanded = expanded_stroke_path(
    shortened.as_ref().unwrap_or(&local_path),
    &resolved_sk_stroke(&realized),
    local_to_raster,
  )?;
  let mask = direct2d_four_sample_mask(pixmap, &expanded, FillRule::Winding, local_to_raster)?;
  paint_stroke_mask(pixmap, &mask, stroke, bounds, commands, page_to_raster)?;
  if let Some(item) = local_item.as_ref() {
    draw_stroke_end_markers(
      pixmap,
      item,
      &realized,
      local_to_raster,
      RasterPrimitiveAntialiasing::Direct2dStandard4,
    )?;
  }
  Some(())
}

fn resolved_sk_stroke(stroke: &Stroke<'static>) -> SkStroke {
  let dash = stroke.resolved_dash().and_then(|values| {
    StrokeDash::new(
      values.into_iter().map(|value| value.0).collect(),
      stroke.dash_offset.0,
    )
  });
  SkStroke {
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
  }
}

fn expanded_stroke_path(
  path: &Path,
  stroke: &SkStroke,
  page_to_raster: SkTransform,
) -> Option<Path> {
  let resolution_scale = PathStroker::compute_resolution_scale(&page_to_raster);
  let dashed;
  let centerline = if let Some(dash) = stroke.dash.as_ref() {
    dashed = path.dash(dash, resolution_scale)?;
    &dashed
  } else {
    path
  };
  PathStroker::new().stroke(centerline, stroke, resolution_scale)
}

fn draw_office_8x4_stroke(
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
  let sk_stroke = resolved_sk_stroke(stroke);
  let stroked_path = expanded_stroke_path(path, &sk_stroke, page_to_raster)?;
  let mask = office_8x4_sample_mask(pixmap, &stroked_path, FillRule::Winding, page_to_raster)?;
  paint_stroke_mask(
    pixmap,
    &mask,
    stroke,
    bounds,
    commands,
    office_8x4_sample_transform(page_to_raster),
  )
}

fn paint_stroke_mask(
  pixmap: &mut Pixmap,
  mask: &Mask,
  stroke: &Stroke<'static>,
  bounds: Rect,
  commands: Option<&[PathCommand]>,
  page_to_raster: SkTransform,
) -> Option<()> {
  if let Some(gradient) = stroke.gradient.as_ref() {
    if gradient.path.is_some() {
      composite_path_gradient_byte_mask(pixmap, mask, gradient, commands, page_to_raster)?;
    } else {
      paint_sample_mask(
        pixmap,
        mask,
        linear_gradient_paint(gradient, bounds)?,
        page_to_raster,
      )?;
    }
  } else if let Some(pattern) = stroke.pattern {
    let tile = pattern_tile(pattern, page_to_raster.sx)?;
    let paint = Paint {
      shader: Pattern::new(
        tile.as_ref(),
        SpreadMode::Repeat,
        FilterQuality::Nearest,
        1.0,
        SkTransform::from_translate(
          pattern_origin(bounds.origin.x.0, pattern.tile_size_points()),
          pattern_origin(bounds.origin.y.0, pattern.tile_size_points()),
        ),
      ),
      ..Paint::default()
    };
    paint_sample_mask(pixmap, mask, paint, page_to_raster)?;
  } else {
    paint_sample_mask(pixmap, mask, solid_paint(stroke.color), page_to_raster)?;
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
  let minimum_tile_px = match pattern.mask {
    super::PatternMask::EmfPlusHatch(_) => 8.0,
    super::PatternMask::Bitmap8(_) => 1.0,
  };
  let tile_px = (pattern.tile_size_points() * pixels_per_point)
    .round()
    .max(minimum_tile_px) as u32;
  let mut pixmap = Pixmap::new(tile_px, tile_px)?;
  for y in 0..tile_px {
    for x in 0..tile_px {
      let hatch_x = (u64::from(x) * 8 / u64::from(tile_px)) as i32;
      let hatch_y = (u64::from(y) * 8 / u64::from(tile_px)) as i32;
      let color = if pattern.is_foreground(hatch_x, hatch_y) {
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

fn pattern_origin(value: f32, tile_size_pt: f32) -> f32 {
  (value / tile_size_pt).floor() * tile_size_pt
}

#[cfg(test)]
mod tests {
  #[test]
  fn vector_scene_resolves_shared_face_coverage_without_background_seams() {
    let rect = |x, width, color| {
      super::DisplayItem::Rect(super::RectItem {
        bounds: crate::model::common_rect(x, 0.0, width, 4.0),
        fill: super::Fill::Solid(color),
        stroke: None,
      })
    };
    let red = crate::model::common_rgb(crate::model::RgbColor { r: 255, g: 0, b: 0 }, 1.0);
    let blue = crate::model::common_rgb(crate::model::RgbColor { r: 0, g: 0, b: 255 }, 1.0);
    let items = [rect(0.0, 3.4, red), rect(3.4, 4.6, blue)];
    let mapping = super::PageToRasterMapping {
      width_px: 8,
      height_px: 4,
      scale_x: 1.0,
      scale_y: 1.0,
      translate_x: 0.0,
      translate_y: 0.0,
      text_hinting: None,
    };
    let image = super::rasterize_vector_scene_at_mapping(&items, mapping).unwrap();
    assert!(image.pixels().all(|pixel| pixel[3] == 255));
    assert!(image.get_pixel(3, 1)[0] > 0 && image.get_pixel(3, 1)[2] > 0);
    let independent = super::rasterize_vector_items_at_mapping(
      &items,
      mapping,
      super::RasterPrimitiveAntialiasing::PerPrimitive,
    )
    .unwrap();
    assert!(independent.get_pixel(3, 1)[3] < 255);
  }

  #[test]
  fn material_texture_resolves_each_paint_before_source_over() {
    let mut result = tiny_skia::Pixmap::new(1, 2).unwrap();
    let mut samples = tiny_skia::Pixmap::new(8, 4).unwrap();
    // Half-covered opaque fill, then the same half-covered 60%-opaque
    // outline. Correlated sample composition would incorrectly give 128.
    for pixel in samples.data_mut()[..16 * 4]
      .as_chunks_mut::<4>()
      .0
      .iter_mut()
    {
      pixel.copy_from_slice(&[128, 64, 32, 255]);
    }
    super::composite_material_sample_band(&mut result, &samples, 1);
    assert_eq!(result.pixel(0, 1).unwrap().alpha(), 128);
    for pixel in samples.data_mut()[..16 * 4]
      .as_chunks_mut::<4>()
      .0
      .iter_mut()
    {
      pixel.copy_from_slice(&[120, 60, 12, 153]);
    }
    super::composite_material_sample_band(&mut result, &samples, 1);
    assert_eq!(result.pixel(0, 1).unwrap().alpha(), 166);
    assert_eq!(result.pixel(0, 0).unwrap().alpha(), 0);
  }

  use super::{
    MAX_EFFECT_RASTER_PIXELS, PageToRasterMapping, RasterPrimitiveAntialiasing, RasterSourceExtent,
    SourceLayer, WordShapeEffectSourceProfile, bounded_effect_raster_grid,
    collect_source_layer_item, effect_pixels_per_point_with_max, office_8x4_sample_mask,
    office_simple_glow_pixels_per_point, place_drawing_raster_on_transparent_surface,
    raster_pixel_extent, rasterize_group_items_for_effects,
    rasterize_group_items_for_effects_at_pixels_per_point,
    rasterize_group_items_for_effects_at_pixels_per_point_with_extent, rasterize_vector_items,
    rasterize_vector_items_for_effects,
    rasterize_vector_items_for_effects_as_local_source_at_pixels_per_point_with_antialiasing,
    rasterize_vector_items_for_effects_at_pixels_per_point_with_extent_and_antialiasing,
    rasterize_vector_items_for_effects_via_dpi_compensated_source_surface,
    rasterize_vector_items_for_effects_with_mapping,
    rasterize_word_shape_effect_source_via_base_surface,
    resize_inclusive_far_edge_premultiplied_linear, resolve_box_filtered_rgba,
    resolve_text_raster_fill, static_3d_text_outline_material_inset_px, text_outline_material_item,
  };
  use bytes::Bytes;
  use image::codecs::png::PngEncoder;
  use image::{ColorType, ImageEncoder, Rgba, RgbaImage};
  use std::borrow::Cow;
  use std::sync::Arc;
  use tiny_skia::{FillRule as SkFillRule, PathBuilder as SkPathBuilder, Pixmap as SkPixmap};

  #[test]
  fn native_source_rejects_unbounded_group_coordinates_before_allocation() {
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Tree,
      effects: vec![ImageEffect::Identity],
    };
    for (width, height) in [(2_143_141.5, 1_428_761.4), (f32::INFINITY, 100.0)] {
      assert!(
        super::rasterize_vector_items_for_effects_via_source_surface(
          &[],
          &effects,
          super::RasterSourceSurface {
            bounds: rect(0.0, 0.0, width, height),
            dpi: 600.0,
            extent: RasterSourceExtent::Outward,
            text_hinting: None,
            primitive_antialiasing: RasterPrimitiveAntialiasing::Direct2dStandard4,
          },
          super::RasterTargetSurface {
            width_px: 468,
            height_px: 312,
            filter: super::RasterResolveFilter::Native,
          },
        )
        .is_none()
      );
    }
  }

  #[test]
  fn pixmap_transfer_matches_png_roundtrip_for_every_alpha_and_channel_value() {
    let mut pixmap = SkPixmap::new(256, 256).unwrap();
    for alpha in 0..=255u8 {
      for channel in 0..=255u8 {
        pixmap.pixels_mut()[usize::from(alpha) * 256 + usize::from(channel)] =
          tiny_skia::PremultipliedColorU8::from_rgba(
            channel.min(alpha),
            (255 - channel).min(alpha),
            channel.rotate_left(3).min(alpha),
            alpha,
          )
          .unwrap();
      }
    }
    let png = pixmap.encode_png().unwrap();
    let expected = image::load_from_memory(&png).unwrap().to_rgba8();
    let original_buffer = pixmap.data().as_ptr();
    let actual = super::pixmap_into_rgba(pixmap).unwrap();
    assert_eq!(actual.as_raw().as_ptr(), original_buffer);
    assert_eq!(actual, expected);
  }

  use crate::common::drawingml_image_effects::{
    ImageEffect, ImageEffectContainer, ImageEffectContainerKind, ImageEffectSourceReference,
  };
  use crate::common::{
    Color, DisplayItem, Fill, GradientFill, GradientPath, GradientPathContext, GradientPathKind,
    GradientStop, ImageCrop, ImageItem, PathCommand, PathItem, PdfGlyphOutlineOptions, Point, Pt,
    Rect, RectItem, RelativeRect, Size, Stroke, StrokeAlignment, TextRun, TextStyle, Transform,
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
  fn powerpoint_blurred_line_source_realizes_actual_grid_and_cosmetic_pen() {
    use crate::common::drawingml_image_effects::EffectOutputBounds;
    for (pen, expected) in [
      (2.0, vec![0, 0, 64, 191, 0, 0]),
      (2.25, vec![0, 0, 64, 191, 0, 0]),
      (2.28, vec![0, 0, 255, 255, 0, 0]),
      (3.75, vec![0, 0, 191, 255, 255, 0, 0]),
      (6.0, vec![0, 0, 191, 255, 255, 255, 0, 0]),
    ] {
      for additional_distance in [0.0, 3.0, 15.0] {
        let frame = rect(96.0, 252.0, 120.0, 0.0);
        let item = DisplayItem::Line(crate::common::LineItem {
          start: frame.origin,
          end: Point {
            x: Pt(216.0),
            y: Pt(252.0),
          },
          kind: crate::common::LineKind::Stroke,
          stroke: Stroke {
            width: Pt(pen),
            color: Color {
              a: 255,
              ..Default::default()
            },
            cap: Some(crate::common::StrokeCap::Flat),
            ..Default::default()
          },
        });
        let radius = 40000.0 / 12700.0;
        let offset = 20000.0 / 12700.0 + additional_distance;
        let ppp = 48.0 / 72.0;
        let sample = super::office_shape_shadow_bitmap_sample_bounds(
          frame,
          EffectOutputBounds {
            left_pt: -radius,
            top_pt: offset - pen * 0.5 - radius,
            right_pt: 120.0 + pen * 0.5 + radius,
            bottom_pt: offset + pen * 0.5 + radius,
          },
          radius,
          ppp,
        );
        let (raster, scale) =
          super::rasterize_powerpoint_blurred_line_source(&[item], sample, (0.0, offset), ppp)
            .unwrap();
        let column = (0..raster.image.height())
          .map(|y| raster.image.get_pixel(raster.image.width() / 2, y)[3])
          .collect::<Vec<_>>();
        assert_eq!(column, expected, "pen={pen} offset={offset}");
        assert!(scale.x.is_finite() && scale.y.is_finite());
      }
    }
  }

  #[test]
  fn powerpoint_sharp_shadow_source_keeps_inclusive_range_and_device_pen() {
    use crate::common::drawingml_image_effects::EffectOutputBounds;
    let ppp = 200.0 / 72.0;
    for height in [30.0, 30.12] {
      for width in [0.0, 0.72, 0.75] {
        for offset in [0.0, 11.0 / 2.0_f32.sqrt()] {
          let frame = rect(96.0, 120.0, 120.0, height);
          let item = DisplayItem::Rect(RectItem {
            bounds: frame,
            fill: Fill::Solid(Color {
              a: 255,
              ..Default::default()
            }),
            stroke: (width > 0.0).then_some(Stroke {
              width: Pt(width),
              color: Color {
                a: 255,
                ..Default::default()
              },
              ..Default::default()
            }),
          });
          let output = super::office_shape_shadow_bitmap_sample_bounds(
            frame,
            EffectOutputBounds {
              left_pt: offset - width / 2.0,
              top_pt: offset - width / 2.0,
              right_pt: 120.0 + offset + width / 2.0,
              bottom_pt: height + offset + width / 2.0,
            },
            0.0,
            ppp,
          );
          let raster = super::rasterize_powerpoint_sharp_shadow_source(
            &[item],
            output,
            (offset, offset),
            ppp,
            true,
          )
          .unwrap();
          let image = raster.image;
          let (w, h) = image.dimensions();
          assert_eq!((w, h), if width == 0.0 { (334, 84) } else { (336, 86) });
          assert_eq!(image.get_pixel(w / 2, 0)[3], 128);
          assert_eq!(image.get_pixel(w / 2, h - 1)[3], 191);
          assert!((1..h - 1).all(|y| image.get_pixel(w / 2, y)[3] == 255));
          assert_eq!(image.get_pixel(0, h / 2)[3], 159);
          assert_eq!(
            image.get_pixel(w - 1, h / 2)[3],
            if offset == 0.0 { 191 } else { 159 }
          );
        }
      }
    }
  }

  #[test]
  fn powerpoint_reflection_source_retains_terminal_half_sample_on_each_axis() {
    for (width, height, divisor, dimensions) in [
      (84.345, 77.865, 1.0, (113, 104)),
      (85.545, 79.065, 1.0, (115, 106)),
      (89.970, 83.490, 2.0, (60, 56)),
      (95.595, 89.115, 3.0, (43, 40)),
    ] {
      let nominal = 96.0 / 72.0 / divisor;
      let (raster, scale) = super::rasterize_powerpoint_transformed_backdrop_source(
        &[],
        rect(0.0, 0.0, width, height),
        crate::common::Transform::default(),
        96.0 / 72.0,
        nominal,
      )
      .unwrap();
      assert_eq!(raster.image.dimensions(), dimensions);
      assert!((scale.x * nominal * (width + 0.5 / nominal) - dimensions.0 as f32).abs() < 0.0001);
      assert!((scale.y * nominal * (height + 0.5 / nominal) - dimensions.1 as f32).abs() < 0.0001);
    }
    assert!(
      super::rasterize_powerpoint_transformed_backdrop_source(
        &[],
        rect(0.0, 0.0, 100000.0, 100000.0),
        crate::common::Transform::default(),
        96.0 / 72.0,
        96.0 / 72.0,
      )
      .is_none()
    );
  }

  #[test]
  fn powerpoint_sharp_shadow_rejects_unbounded_source_allocation() {
    assert!(
      super::rasterize_powerpoint_sharp_shadow_source(
        &[],
        rect(0.0, 0.0, 1_000_000.0, 1_000_000.0),
        (0.0, 0.0),
        200.0 / 72.0,
        true,
      )
      .is_none()
    );
  }

  #[test]
  fn text_raster_fill_uses_word_focus_and_farthest_container_corner() {
    let bounds = rect(20.0, 30.0, 100.0, 50.0);
    let mut fill = Fill::Gradient(GradientFill {
      path: Some(GradientPath {
        kind: GradientPathKind::Circle,
        context: GradientPathContext::WordprocessingText,
        fill_to: RelativeRect {
          left: 0.5,
          top: 1.3,
          right: 0.5,
          bottom: -0.3,
        },
        transform: Transform::default(),
        mirror_tile: false,
      }),
      ..Default::default()
    });

    resolve_text_raster_fill(&mut fill, bounds);

    let Fill::Gradient(gradient) = fill else {
      unreachable!();
    };
    let path = gradient.path.unwrap();
    let radius = 65.0_f32.hypot(50.0);
    assert_eq!(gradient.definition_bounds, Some(bounds));
    assert_eq!(path.fill_to.left, 0.5);
    assert_eq!(path.fill_to.top, 0.5);
    assert!((path.transform.m11 - radius * 2.0).abs() < 1.0e-4);
    assert!((path.transform.dx.0 + radius - 70.0).abs() < 1.0e-4);
    assert!((path.transform.dy.0 + radius - 95.0).abs() < 1.0e-4);
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
  fn fixed_output_effect_grid_does_not_inherit_the_preview_budget() {
    let density = 200.0 / 72.0;
    for scale in [0.25, 0.4, 0.5, 0.75, 1.0, 1.25] {
      for shadow_scale in [1.0, 2.0] {
        let bounds = rect(
          70.85,
          70.85,
          246.75 * scale * shadow_scale,
          139.5 * scale * shadow_scale,
        );
        let (aligned, actual) = super::fixed_output_effect_raster_grid(bounds, density);
        assert_eq!(actual, density);
        assert_eq!(aligned, super::align_rect_to_pixel_grid(bounds, density));
      }
    }
    // The caller's actual blur/output tier still owns the upper density.
    let (_, blurred) = super::fixed_output_effect_raster_grid(rect(0.0, 0.0, 500.0, 500.0), 0.5);
    assert_eq!(blurred, 0.5);
    // Allocation protection is independent of that tier; it is not removed.
    let (large, capped) =
      super::fixed_output_effect_raster_grid(rect(0.0, 0.0, 4000.0, 4000.0), density);
    assert!(capped < density);
    assert!(
      large.size.width.0 * large.size.height.0 * capped * capped
        <= super::MAX_FIXED_OUTPUT_EFFECT_RASTER_PIXELS + 1.0
    );
  }

  #[test]
  fn fixed_output_source_density_survives_preview_budget_boundary() {
    let effects = super::super::drawingml_image_effects::ImageEffectContainer {
      kind: super::super::drawingml_image_effects::ImageEffectContainerKind::Sibling,
      effects: Vec::new(),
    };
    for density in [96.0 / 72.0, 200.0 / 72.0] {
      for size in [100.0, 400.0] {
        let raster = super::rasterize_vector_items_for_effects_at_fixed_output_pixels_per_point(
          &[],
          rect(3.25, 7.5, size, size),
          &effects,
          density,
        )
        .unwrap();
        assert_eq!(raster.pixels_per_point, density);
        let extent = super::raster_pixel_extent(size, density);
        assert_eq!(raster.image.dimensions(), (extent, extent));
        let requested = rect(3.25, 7.5, size, size);
        let displayed = raster.bounds_at_pixel_density(requested.origin);
        assert_eq!(displayed.origin, requested.origin);
        assert!((displayed.size.width.0 * density - extent as f32).abs() < 0.001);
        assert!((displayed.size.height.0 * density - extent as f32).abs() < 0.001);
        assert!(displayed.size.width.0 + 0.0001 >= size);
        assert!(displayed.size.width.0 < size + 1.0 / density + 0.0001);
      }
    }
    for (size, density) in [
      (1.0e9, 200.0 / 72.0),
      (10.0, f32::NAN),
      (10.0, 0.0),
      (f32::INFINITY, 1.0),
    ] {
      assert!(
        super::rasterize_vector_items_for_effects_at_fixed_output_pixels_per_point(
          &[],
          rect(0.0, 0.0, size, size),
          &effects,
          density,
        )
        .is_none()
      );
    }
  }

  #[test]
  fn uniform_raster_display_does_not_compress_fractional_requested_extents() {
    for density in [96.0 / 72.0, 200.0 / 72.0] {
      for width in [29.999, 30.0, 30.001, 248.44318] {
        let requested = rect(-7.25, 155.90796, width, 143.23581);
        let raster = super::rasterize_vector_items_for_effects_at_fixed_output_pixels_per_point(
          &[],
          requested,
          &super::super::drawingml_image_effects::ImageEffectContainer {
            kind: super::super::drawingml_image_effects::ImageEffectContainerKind::Sibling,
            effects: Vec::new(),
          },
          density,
        )
        .unwrap();
        let displayed = raster.bounds_at_pixel_density(requested.origin);
        assert_eq!(displayed.origin, requested.origin);
        for (physical, pixels) in [
          (displayed.size.width.0, raster.image.width()),
          (displayed.size.height.0, raster.image.height()),
        ] {
          assert!((physical / pixels as f32 - 1.0 / density).abs() < 0.000001);
        }
      }
    }
  }

  #[test]
  fn office_simple_glow_density_keeps_exact_reference_tier_edges() {
    let print = crate::units::OFFICE_FIXED_OUTPUT_RASTER_DPI;
    let samples = [
      (5.75, 200.0),
      (5.76, 200.0),
      (5.77, 100.0),
      (11.51, 100.0),
      (11.52, 100.0),
      (11.53, 200.0 / 3.0),
      (17.27, 200.0 / 3.0),
      (17.28, 200.0 / 3.0),
      (17.29, 50.0),
      (34.55, 200.0 / 6.0),
      (34.56, 200.0 / 6.0),
      (34.57, 200.0 / 7.0),
      (36.0, 200.0 / 7.0),
      (48.0, 200.0 / 9.0),
    ];
    for (radius_pt, expected_dpi) in samples {
      let actual_dpi =
        office_simple_glow_pixels_per_point(print, radius_pt) * crate::units::POINTS_PER_INCH;
      assert!(
        (actual_dpi - expected_dpi).abs() < 0.0001,
        "radius={radius_pt}, actual_dpi={actual_dpi}"
      );
    }
  }

  #[test]
  fn effect_grid_encloses_fractional_and_negative_logical_bounds() {
    let (aligned, pixels_per_point) = bounded_effect_raster_grid(rect(10.3, -2.1, 5.2, 3.0), 2.0);

    assert!((pixels_per_point - 2.0).abs() < f32::EPSILON);
    assert_eq!(aligned, rect(10.0, -2.5, 5.5, 3.5));
    assert!(aligned.origin.x.0 <= 10.3);
    assert!(aligned.origin.y.0 <= -2.1);
    assert!(aligned.origin.x.0 + aligned.size.width.0 >= 15.5);
    assert!(aligned.origin.y.0 + aligned.size.height.0 >= 0.9);
  }

  #[test]
  fn pixel_extent_does_not_ceil_an_aligned_integer_twice() {
    let pixels_per_point = 200.0 / 72.0;

    // The values are the 116-pixel Word static-3D target after an absolute
    // page-space grid alignment.  Their f32 product lands just above 116.
    assert_eq!(raster_pixel_extent(41.76001, pixels_per_point), 116);
    assert_eq!(raster_pixel_extent(41.77, pixels_per_point), 117);
  }

  #[test]
  fn device_stroke_source_consumer_matches_office_width_plateaus() {
    for (width_emu, row_mass) in [
      (0, 383),
      (1270, 383),
      (3810, 637),
      (9525, 2041),
      (19050, 4207),
    ] {
      let mut items = vec![DisplayItem::Path(PathItem {
        bounds: rect(1.2, 0.0, 4.8, 4.8),
        points: vec![
          super::super::Point {
            x: Pt(1.2),
            y: Pt(0.0),
          },
          super::super::Point {
            x: Pt(6.0),
            y: Pt(4.8),
          },
        ],
        stroke: Some(Stroke {
          width: Pt(width_emu as f32 / 12_700.0),
          color: Color {
            a: 255,
            ..Default::default()
          },
          drawingml_device: Some(Box::new(super::super::DrawingMlDeviceStroke {
            width_emu,
            emu_to_points: [0.0, 1.0 / 12_700.0, 1.0 / 12_700.0, 0.0],
            snap: true,
            realized_width_emu: None,
          })),
          ..Default::default()
        }),
        ..Default::default()
      })];
      super::realize_source_device_strokes(&mut items, 600.0).unwrap();
      let image = super::rasterize_vector_items_at_mapping(
        &items,
        PageToRasterMapping {
          width_px: 64,
          height_px: 40,
          scale_x: 600.0 / 72.0,
          scale_y: 600.0 / 72.0,
          translate_x: 0.0,
          translate_y: 0.0,
          text_hinting: None,
        },
        RasterPrimitiveAntialiasing::Direct2dStandard4,
      )
      .unwrap();
      assert_eq!(
        (0..64)
          .map(|x| u32::from(image.get_pixel(x, 20)[3]))
          .sum::<u32>(),
        row_mass,
        "width {width_emu} EMU"
      );
    }
  }

  #[test]
  fn direct2d_four_sample_resolves_each_path_before_compositing() {
    let item = DisplayItem::Rect(RectItem {
      bounds: rect(0.0, 0.0, 0.5, 1.0),
      fill: Fill::Solid(Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
      }),
      stroke: None,
    });
    let mapping = PageToRasterMapping {
      width_px: 1,
      height_px: 1,
      scale_x: 1.0,
      scale_y: 1.0,
      translate_x: 0.0,
      translate_y: 0.0,
      text_hinting: None,
    };
    let single = super::rasterize_vector_items_at_mapping(
      std::slice::from_ref(&item),
      mapping,
      RasterPrimitiveAntialiasing::Direct2dStandard4,
    )
    .unwrap();
    let repeated = super::rasterize_vector_items_at_mapping(
      &[item.clone(), item],
      mapping,
      RasterPrimitiveAntialiasing::Direct2dStandard4,
    )
    .unwrap();
    assert_eq!(single.get_pixel(0, 0)[3], 128);
    // Once-per-scene sample resolve would incorrectly remain 128.
    assert_eq!(repeated.get_pixel(0, 0)[3], 192);
  }

  #[test]
  fn exact_mapping_raster_preserves_axis_scales_origin_and_sample_policy() {
    let item = DisplayItem::Rect(RectItem {
      bounds: rect(2.0, 3.0, 4.0, 5.0),
      fill: Fill::Solid(Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
      }),
      stroke: None,
    });
    for (scale_x, scale_y) in [(2.0, 3.0), (3.0, 2.0)] {
      for antialiasing in [
        RasterPrimitiveAntialiasing::Aliased,
        RasterPrimitiveAntialiasing::PerPrimitive,
        RasterPrimitiveAntialiasing::Direct2dStandard4,
        RasterPrimitiveAntialiasing::OfficeAntiAlias8x4,
      ] {
        let mapping = PageToRasterMapping {
          width_px: 32,
          height_px: 32,
          scale_x,
          scale_y,
          translate_x: 1.0,
          translate_y: 2.0,
          text_hinting: None,
        };
        let image = super::rasterize_vector_items_at_mapping(
          std::slice::from_ref(&item),
          mapping,
          antialiasing,
        )
        .unwrap();
        for (x, y, pixel) in image.enumerate_pixels() {
          let left = 1.0 + 2.0 * scale_x;
          let right = 1.0 + 6.0 * scale_x;
          let top = 2.0 + 3.0 * scale_y;
          let bottom = 2.0 + 8.0 * scale_y;
          let expected = if antialiasing == RasterPrimitiveAntialiasing::OfficeAntiAlias8x4 {
            // GDI+ samples around integer device centers: an integer edge
            // bisects a pixel, including the far edge. A corner has 1/4 coverage.
            let axis = |value: f32, near, far| {
              if value == near || value == far {
                1_u32
              } else if value > near && value < far {
                2
              } else {
                0
              }
            };
            ((axis(x as f32, left, right) * axis(y as f32, top, bottom) * 255 + 2) / 4) as u8
          } else if (left..right).contains(&(x as f32)) && (top..bottom).contains(&(y as f32)) {
            255
          } else {
            0
          };
          assert_eq!(pixel[3], expected, "{antialiasing:?}: {x},{y}");
        }
      }
    }
    let valid = PageToRasterMapping {
      width_px: 32,
      height_px: 32,
      scale_x: 2.0,
      scale_y: 3.0,
      translate_x: 1.0,
      translate_y: 2.0,
      text_hinting: None,
    };
    for invalid in [
      PageToRasterMapping {
        width_px: 0,
        ..valid
      },
      PageToRasterMapping {
        height_px: 0,
        ..valid
      },
      PageToRasterMapping {
        scale_x: 0.0,
        ..valid
      },
      PageToRasterMapping {
        scale_y: -1.0,
        ..valid
      },
      PageToRasterMapping {
        scale_x: f32::NAN,
        ..valid
      },
      PageToRasterMapping {
        scale_y: f32::INFINITY,
        ..valid
      },
      PageToRasterMapping {
        translate_x: f32::NAN,
        ..valid
      },
      PageToRasterMapping {
        translate_y: f32::INFINITY,
        ..valid
      },
    ] {
      assert!(
        super::rasterize_vector_items_at_mapping(
          std::slice::from_ref(&item),
          invalid,
          RasterPrimitiveAntialiasing::Aliased,
        )
        .is_none()
      );
    }
  }

  #[test]
  fn aliased_effect_source_uses_pixel_center_and_excludes_the_far_edge() {
    let surface = rect(188.4, 116.16, 292.98, 52.02);
    let item = DisplayItem::Rect(RectItem {
      bounds: rect(193.5, 121.3, 282.95, 41.85),
      fill: Fill::Solid(Color {
        r: 255,
        g: 255,
        b: 255,
        a: 51,
      }),
      stroke: None,
    });
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: Vec::new(),
    };

    let raster =
      rasterize_vector_items_for_effects_at_pixels_per_point_with_extent_and_antialiasing(
        &[item],
        surface,
        &effects,
        200.0 / 72.0,
        RasterSourceExtent::InclusiveFarEdge,
        RasterPrimitiveAntialiasing::Aliased,
      )
      .unwrap();

    assert_eq!((raster.image.width(), raster.image.height()), (814, 145));
    assert_eq!(
      [
        raster.image.get_pixel(13, 72)[3],
        raster.image.get_pixel(14, 72)[3],
        raster.image.get_pixel(799, 72)[3],
        raster.image.get_pixel(800, 72)[3],
      ],
      [0, 51, 51, 0]
    );
  }

  #[test]
  fn local_effect_source_is_independent_of_page_translation() {
    let pixels_per_point = 200.0 / 72.0;
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: vec![ImageEffect::SourceReference(
        ImageEffectSourceReference::Fill,
      )],
    };
    let raster = |dx: f32, dy: f32| {
      let source = rect(10.13 + dx, 20.17 + dy, 4.2, 3.3);
      let display = rect(10.01 + dx, 20.05 + dy, 4.44, 3.54);
      let item = DisplayItem::Rect(RectItem {
        bounds: source,
        fill: Fill::Solid(Color {
          r: 240,
          g: 80,
          b: 20,
          a: 255,
        }),
        stroke: None,
      });
      let raster =
        rasterize_vector_items_for_effects_as_local_source_at_pixels_per_point_with_antialiasing(
          &[item],
          source,
          display,
          &effects,
          pixels_per_point,
          RasterPrimitiveAntialiasing::OfficeAntiAlias8x4,
        )
        .unwrap();
      assert_eq!(raster.fill_image.as_ref(), Some(&raster.image));
      assert!(raster.line_image.is_none());
      raster.image
    };

    let original = raster(0.0, 0.0);
    assert_eq!(original.get_pixel(0, 0)[3], 64);
    for (dx, dy) in [(0.12, 0.24), (-0.12, -0.24), (64.0, 32.0), (-10.13, -20.17)] {
      assert_eq!(original, raster(dx, dy), "translation {dx},{dy}");
    }
  }

  #[test]
  fn local_effect_source_padding_leaves_odd_terminal_pixel_on_far_edge() {
    let source = RgbaImage::from_pixel(2, 2, Rgba([10, 20, 30, 40]));
    let raster = super::DrawingRaster {
      image: source.clone(),
      fill_image: Some(source),
      line_image: None,
      fill_line_image: None,
      children_image: None,
      pixels_per_point: 1.0,
    };

    let padded = place_drawing_raster_on_transparent_surface(raster, 5, 4, 1, 1).unwrap();

    assert_eq!(padded.image.get_pixel(0, 1), &Rgba([0, 0, 0, 0]));
    assert_eq!(padded.image.get_pixel(1, 1), &Rgba([10, 20, 30, 40]));
    assert_eq!(padded.image.get_pixel(2, 2), &Rgba([10, 20, 30, 40]));
    assert_eq!(padded.image.get_pixel(3, 2), &Rgba([0, 0, 0, 0]));
    assert_eq!(padded.fill_image.as_ref().unwrap(), &padded.image);
  }

  #[test]
  fn word_shadow_minimum_pen_uses_output_density_without_changing_foreground() {
    for dpi in [96.0, 48.0, 24.0] {
      let minimum = 72.0 / dpi;
      for width in [0.0, 0.25, 0.75, 1.5, 3.0, 5.0] {
        let original = DisplayItem::Rect(RectItem {
          bounds: rect(2.0, 2.0, 8.0, 6.0),
          fill: Fill::None,
          stroke: Some(Stroke {
            width: Pt(width),
            color: Color {
              a: 255,
              ..Color::default()
            },
            ..Stroke::default()
          }),
        });
        let mut source = vec![original.clone()];
        super::realize_word_shadow_minimum_strokes(&mut source, dpi / 72.0);
        let DisplayItem::Rect(actual) = &source[0] else {
          unreachable!()
        };
        assert_eq!(actual.stroke.as_ref().unwrap().width.0, width.max(minimum));
        let DisplayItem::Rect(original) = original else {
          unreachable!()
        };
        assert_eq!(original.stroke.unwrap().width.0, width);
      }
    }
    let mut no_line = [DisplayItem::Rect(RectItem {
      bounds: rect(2.0, 2.0, 8.0, 6.0),
      fill: Fill::None,
      stroke: None,
    })];
    super::realize_word_shadow_minimum_strokes(&mut no_line, 48.0 / 72.0);
    let DisplayItem::Rect(actual) = &no_line[0] else {
      unreachable!()
    };
    assert!(actual.stroke.is_none());
  }

  #[test]
  fn word_shadow_source_keeps_zero_pen_distinct_from_no_pen() {
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: Vec::new(),
    };
    let surface = rect(0.0, 0.0, 18.0, 16.0);
    let content = rect(4.0, 4.0, 8.0, 6.0);
    let raster = |profile, width: Option<f32>, target_pixels_per_point| {
      let item = DisplayItem::Rect(RectItem {
        bounds: content,
        fill: Fill::None,
        stroke: width.map(|width| Stroke {
          width: Pt(width),
          color: Color {
            a: 255,
            ..Color::default()
          },
          ..Stroke::default()
        }),
      });
      rasterize_word_shape_effect_source_via_base_surface(
        &[item],
        &effects,
        super::WordShapeEffectSurface {
          profile,
          base_bounds: surface,
          content_bounds: content,
          base_pixels_per_point: 96.0 / 72.0,
          target_width_px: super::inclusive_far_edge_raster_pixel_extent(
            surface.size.width.0,
            target_pixels_per_point,
          ),
          target_height_px: super::inclusive_far_edge_raster_pixel_extent(
            surface.size.height.0,
            target_pixels_per_point,
          ),
          target_pixels_per_point,
        },
      )
      .unwrap()
      .image
    };
    for dpi in [96.0, 48.0, 24.0] {
      let scale = dpi / 72.0;
      let zero = raster(WordShapeEffectSourceProfile::OuterShadow, Some(0.0), scale);
      let hairline = raster(
        WordShapeEffectSourceProfile::OuterShadow,
        Some(72.0 / dpi),
        scale,
      );
      let absent = raster(WordShapeEffectSourceProfile::OuterShadow, None, scale);
      assert_eq!(zero, hairline);
      assert!(zero.pixels().any(|pixel| pixel[3] != 0));
      assert!(absent.pixels().all(|pixel| pixel[3] == 0));
      assert!(
        raster(WordShapeEffectSourceProfile::Glow, Some(0.0), scale)
          .pixels()
          .all(|pixel| pixel[3] == 0)
      );
    }
  }

  #[test]
  fn powerpoint_blurred_vector_source_keeps_pen_presence_and_device_minimum() {
    let surface = rect(0.0, 0.0, 8.25, 8.25);
    let sample = |width: Option<f32>| {
      DisplayItem::Rect(RectItem {
        bounds: rect(2.25, 2.25, 3.0, 3.0),
        fill: Fill::Solid(Color {
          r: 0,
          g: 0,
          b: 0,
          a: 255,
        }),
        stroke: width.map(|width| Stroke {
          width: Pt(width),
          color: Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
          },
          ..Default::default()
        }),
      })
    };
    let render = |item: &DisplayItem<'static>, bounds, offset, density| {
      super::rasterize_powerpoint_blurred_vector_source(
        std::slice::from_ref(item),
        bounds,
        offset,
        density,
      )
    };
    let zero = sample(Some(0.0));
    let thin = sample(Some(0.25));
    let zero_image = render(&zero, surface, (0.0, 0.0), 1.0).unwrap().0.image;
    let thin_image = render(&thin, surface, (0.0, 0.0), 1.0).unwrap().0.image;
    assert_eq!(zero_image, thin_image);
    assert_ne!(
      thin_image,
      render(&sample(None), surface, (0.0, 0.0), 1.0)
        .unwrap()
        .0
        .image
    );
    assert_ne!(
      thin_image,
      render(&sample(Some(2.0)), surface, (0.0, 0.0), 1.0)
        .unwrap()
        .0
        .image
    );
    assert_eq!(
      thin_image,
      render(&thin, rect(1.0, 2.0, 8.25, 8.25), (1.0, 2.0), 1.0)
        .unwrap()
        .0
        .image
    );
    let DisplayItem::Rect(original) = thin else {
      unreachable!()
    };
    assert_eq!(original.stroke.unwrap().width, Pt(0.25));
    for invalid in [0.0, -1.0, f32::NAN, f32::INFINITY] {
      assert!(render(&zero, surface, (0.0, 0.0), invalid).is_none());
    }
    assert!(render(&zero, surface, (f32::NAN, 0.0), 1.0).is_none());
    assert!(render(&zero, rect(0.0, 0.0, 100_000.0, 100_000.0), (0.0, 0.0), 1.0).is_none());
  }

  #[test]
  fn word_shape_glow_and_shadow_keep_independent_primitive_coverage() {
    let surface = rect(0.0, 0.0, 4.0, 3.0);
    let content = rect(0.2, 0.2, 2.5, 1.5);
    let item = DisplayItem::Rect(RectItem {
      bounds: content,
      fill: Fill::Solid(Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
      }),
      stroke: None,
    });
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: Vec::new(),
    };
    let pixels_per_point = 4.0;
    let width =
      super::inclusive_far_edge_raster_pixel_extent(surface.size.width.0, pixels_per_point);
    let height =
      super::inclusive_far_edge_raster_pixel_extent(surface.size.height.0, pixels_per_point);
    let raster = |profile| {
      rasterize_word_shape_effect_source_via_base_surface(
        std::slice::from_ref(&item),
        &effects,
        super::WordShapeEffectSurface {
          profile,
          base_bounds: surface,
          content_bounds: content,
          base_pixels_per_point: pixels_per_point,
          target_width_px: width,
          target_height_px: height,
          target_pixels_per_point: pixels_per_point,
        },
      )
      .unwrap()
      .image
    };
    let glow = raster(WordShapeEffectSourceProfile::Glow);
    let shadow = raster(WordShapeEffectSourceProfile::OuterShadow);

    assert!(glow.pixels().all(|pixel| matches!(pixel[3], 0 | 255)));
    assert!(shadow.pixels().any(|pixel| (1..=254).contains(&pixel[3])));
    assert_ne!(glow, shadow);
  }

  #[test]
  fn inclusive_far_edge_linear_resize_uses_half_terminal_sample() {
    let source = RgbaImage::from_fn(5, 1, |x, _| {
      let alpha = [0, 64, 128, 192, 255][x as usize];
      Rgba([255, 255, 255, alpha])
    });

    let resized = resize_inclusive_far_edge_premultiplied_linear(&source, 2, 1);

    assert_eq!(resized.get_pixel(0, 0)[3], 40);
    assert_eq!(resized.get_pixel(1, 0)[3], 184);
  }

  #[test]
  fn dpi_compensated_effect_source_preserves_physical_pixel_center_mapping() {
    let bounds = rect(0.0, 0.0, 2.0, 1.0);
    let item = DisplayItem::Rect(RectItem {
      bounds: rect(0.25, 0.0, 1.0, 1.0),
      fill: Fill::Solid(Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
      }),
      stroke: None,
    });
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: Vec::new(),
    };

    let source =
      rasterize_vector_items_for_effects_at_pixels_per_point_with_extent_and_antialiasing(
        std::slice::from_ref(&item),
        bounds,
        &effects,
        2.0,
        RasterSourceExtent::Outward,
        RasterPrimitiveAntialiasing::OfficeAntiAlias8x4,
      )
      .unwrap();
    let expected = crate::common::drawingml_image_effects::dpi_compensate_linear_hard(
      &source.image,
      2.0,
      1.0,
      2,
      1,
    )
    .unwrap();
    let resolved = rasterize_vector_items_for_effects_via_dpi_compensated_source_surface(
      &[item],
      bounds,
      &effects,
      2.0,
      1.0,
      RasterSourceExtent::Outward,
      RasterPrimitiveAntialiasing::OfficeAntiAlias8x4,
    )
    .unwrap();

    assert_eq!(resolved.image, expected);
    assert_eq!(resolved.image.dimensions(), (2, 1));
    assert!((resolved.pixels_per_point - 1.0).abs() < f32::EPSILON);
  }

  #[test]
  fn horizontal_box_resolve_averages_in_premultiplied_color_space() {
    let source = RgbaImage::from_raw(
      4,
      1,
      vec![255, 0, 0, 255, 0, 255, 0, 0, 255, 0, 0, 64, 0, 0, 255, 192],
    )
    .unwrap();

    let resolved = resolve_box_filtered_rgba(&source, 2, 1, 2, 1);

    assert_eq!(resolved.get_pixel(0, 0), &Rgba([255, 0, 0, 128]));
    assert_eq!(resolved.get_pixel(1, 0), &Rgba([64, 0, 191, 128]));
  }

  #[test]
  fn office_8x4_samples_include_near_grid_edges_and_exclude_far_edges() {
    let mut builder = SkPathBuilder::new();
    builder.move_to(-4.0, -2.0);
    builder.line_to(4.0, -2.0);
    builder.line_to(4.0, 2.0);
    builder.line_to(-4.0, 2.0);
    builder.close();
    let path = builder.finish().unwrap();
    let pixmap = SkPixmap::new(10, 6).unwrap();

    let mask = office_8x4_sample_mask(
      &pixmap,
      &path,
      SkFillRule::EvenOdd,
      tiny_skia::Transform::identity(),
    )
    .unwrap();

    assert_eq!(mask.data()[0], 255);
    assert_eq!(mask.data()[3 * 10 + 7], 255);
    assert_eq!(mask.data()[8], 0);
    assert_eq!(mask.data()[4 * 10], 0);
  }

  #[test]
  fn screen_static_3d_mapping_matches_office_foreground_edge_coverage() {
    let content = rect(0.0, 0.0, 34.5, 18.75);
    let width_px = 46.0;
    let height_px = 24.84375;
    let mapping = PageToRasterMapping {
      width_px: 55,
      height_px: 34,
      scale_x: width_px / content.size.width.0,
      scale_y: height_px / content.size.height.0,
      translate_x: 5.0,
      translate_y: 2.5625,
      text_hinting: None,
    };
    let items = [DisplayItem::Rect(RectItem {
      bounds: content,
      fill: Fill::Solid(Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
      }),
      stroke: None,
    })];
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: Vec::new(),
    };

    let raster =
      rasterize_vector_items_for_effects_with_mapping(&items, &effects, 96.0 / 72.0, mapping)
        .unwrap();

    assert_eq!(
      [
        raster.image.get_pixel(4, 15)[3],
        raster.image.get_pixel(5, 15)[3],
        raster.image.get_pixel(50, 15)[3],
        raster.image.get_pixel(51, 15)[3],
        raster.image.get_pixel(27, 2)[3],
        raster.image.get_pixel(27, 3)[3],
        raster.image.get_pixel(27, 26)[3],
        raster.image.get_pixel(27, 27)[3],
      ],
      // Office's final bottom-edge byte is 105; 104 is the nearest value on
      // the bounded 32-sample coverage grid used here.
      [0, 255, 255, 0, 112, 255, 255, 104]
    );
  }

  #[test]
  fn static_3d_picture_frame_retains_rectangular_surface_and_rotation() {
    for (angle, expected) in [(0.0, (10.0, 20.0)), (90.0, (25.0, 15.0))] {
      let path = super::static_3d_picture_frame(rect(10.0, 20.0, 20.0, 10.0), angle);
      assert_eq!(path.len(), 5);
      assert_eq!(path[4], PathCommand::Close);
      let PathCommand::MoveTo(first) = path[0] else {
        panic!("closed frame starts with MoveTo")
      };
      assert!((first.x.0 - expected.0).abs() < 0.0001);
      assert!((first.y.0 - expected.1).abs() < 0.0001);
    }
  }

  #[test]
  fn anisotropic_mapping_uses_both_axes_when_sampling_images() {
    let source = RgbaImage::from_pixel(1, 1, Rgba([240, 80, 20, 255]));
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
      .write_image(source.as_raw(), 1, 1, ColorType::Rgba8.into())
      .unwrap();
    let item = DisplayItem::Image(ImageItem {
      bounds: rect(0.0, 0.0, 10.0, 10.0),
      crop: Some(ImageCrop::default()),
      clip_path: Vec::new(),
      rotation_degrees: 0.0,
      flip_horizontal: false,
      flip_vertical: false,
      content_type: Cow::Borrowed("image/png"),
      bytes: Bytes::from(png),
      blip_compression_state: crate::common::BlipCompressionState::Unspecified,
      metafile_monochrome_dib_palette_override: None,
      metafile_background_color: None,
      metafile_external_header: None,
      metafile_fixed_output_profile: crate::common::MetafileFixedOutputProfile::Default,
      relationship_id: None,
      alt_text: None,
      hyperlink_url: None,
      semantic_metafile_text: false,
      metafile_semantic_text_includes_raster_backdrop: false,
      signature_line: None,
      metafile_native_size: false,
      floating: false,
      behind_text: false,
    });
    let effects = ImageEffectContainer {
      kind: ImageEffectContainerKind::Sibling,
      effects: Vec::new(),
    };
    let raster = rasterize_vector_items_for_effects_with_mapping(
      &[item],
      &effects,
      1.0,
      PageToRasterMapping {
        width_px: 20,
        height_px: 30,
        scale_x: 2.0,
        scale_y: 3.0,
        translate_x: 0.0,
        translate_y: 0.0,
        text_hinting: None,
      },
    )
    .unwrap();

    assert_eq!((raster.image.width(), raster.image.height()), (20, 30));
    assert!(raster.image.pixels().all(|pixel| pixel[3] == 255));
  }

  #[test]
  fn text_source_layers_keep_fill_and_outline_independent() {
    let fill_color = Color {
      r: 220,
      g: 80,
      b: 20,
      a: 255,
    };
    let outline_color = Color {
      r: 20,
      g: 80,
      b: 220,
      a: 180,
    };
    let style = TextStyle {
      font_family: Some("Liberation Serif".into()),
      font_size: Pt(12.0),
      outline_color: Some(outline_color),
      outline_width: Pt(2.0),
      pdf_glyph_outline_options: Some(Arc::new(PdfGlyphOutlineOptions {
        fill: Some(Fill::Solid(fill_color)),
        outline_fill: Some(Fill::Solid(outline_color)),
        outline_stroke: Some(Stroke {
          width: Pt(2.0),
          color: outline_color,
          ..Stroke::default()
        }),
        ..PdfGlyphOutlineOptions::default()
      })),
      ..TextStyle::default()
    };
    let item = DisplayItem::Text(TextRun {
      text: Cow::Borrowed("Example"),
      origin: Point {
        x: Pt(0.0),
        y: Pt(0.0),
      },
      line_height: Pt(12.0),
      line_metrics_participant: true,
      paint_clip: None,
      page_culling_bounds: None,
      style,
      font_id: None,
      color: fill_color,
      rotation_center: None,
      hyperlink_url: None,
      dynamic_field: None,
      form_widget_id: None,
      paragraph_bidi: false,
      word_spacing_pt: 0.0,
      preserve_text_portion: false,
      pdf_text_segmentation: Default::default(),
      source: None,
    });

    let mut fill_layer = Vec::new();
    collect_source_layer_item(&item, SourceLayer::Fill, &mut fill_layer).unwrap();
    let DisplayItem::Text(fill_text) = &fill_layer[0] else {
      panic!("text fill source must remain a text item");
    };
    assert_eq!(fill_text.style.outline_color, None);
    assert_eq!(fill_text.style.outline_width, Pt(0.0));
    let fill_options = fill_text.style.pdf_glyph_outline_options.as_ref().unwrap();
    assert_eq!(fill_options.fill, Some(Fill::Solid(fill_color)));
    assert_eq!(fill_options.outline_fill, None);
    assert_eq!(fill_options.outline_stroke, None);

    let DisplayItem::Text(authored) = &item else {
      unreachable!()
    };
    for width in [0.0, -1.0, f32::NAN, f32::INFINITY] {
      assert!(
        super::static_3d_text_reflection_contour_item(authored, Pt(width), outline_color).is_none()
      );
    }
    for width in [0.5, 1.0, 2.0] {
      for alpha in [0, 64, 255] {
        let color = Color {
          a: alpha,
          ..outline_color
        };
        let contour = super::static_3d_text_reflection_contour_item(authored, Pt(width), color);
        if alpha == 0 {
          assert!(contour.is_none());
          continue;
        }
        let contour = contour.unwrap();
        assert_eq!(contour.origin, authored.origin);
        assert_eq!(contour.text, authored.text);
        let options = contour.style.pdf_glyph_outline_options.as_ref().unwrap();
        assert_eq!(options.fill, Some(Fill::None));
        assert_eq!(options.outline_fill, Some(Fill::Solid(color)));
        let stroke = options.outline_stroke.as_ref().unwrap();
        assert_eq!(stroke.width, Pt(width));
        assert_eq!(stroke.color, color);
        assert_eq!(
          stroke.alignment,
          Some(super::super::StrokeAlignment::Center)
        );
        assert_eq!(stroke.join, Some(super::super::StrokeJoin::Round));
      }
    }
    for width in [0.0, 0.5, 1.0, 2.0] {
      let caster = super::opaque_static_3d_text_effect_item(authored, Pt(width), 255).unwrap();
      assert_eq!(caster.style.outline_width, Pt(width));
      assert_eq!(caster.origin, authored.origin);
      assert_eq!(caster.text, authored.text);
      let options = caster.style.pdf_glyph_outline_options.as_ref().unwrap();
      assert_eq!(options.fill, Some(Fill::Solid(fill_color)));
      if width == 0.0 {
        assert!(options.outline_stroke.is_none());
      } else {
        let stroke = options.outline_stroke.as_ref().unwrap();
        assert_eq!(stroke.color.a, 255);
        assert_eq!(stroke.join, Some(super::super::StrokeJoin::Round));
        assert_eq!(stroke.width, Pt(width));
      }
    }
    let mut translucent = authored.clone();
    for fill in [Fill::None, Fill::Solid(outline_color)] {
      let options = Arc::make_mut(
        translucent
          .style
          .pdf_glyph_outline_options
          .as_mut()
          .unwrap(),
      );
      options.fill = Some(fill);
      // Non-opaque ownership is not inferred from the opaque caster rule.
      assert!(super::opaque_static_3d_text_effect_item(&translucent, Pt(1.0), 255).is_none());
    }
    assert!(super::opaque_static_3d_text_effect_item(authored, Pt(f32::NAN), 255).is_none());
    // Independent construction must not modify the original character paint.
    assert_eq!(authored.style.outline_width, Pt(2.0));
    assert_eq!(authored.style.outline_color, Some(outline_color));

    // The reflected raw-glyph interior must never acquire contour colour,
    // including through a translucent character outline. Check the partition
    // on aliased samples so coverage interpolation cannot hide an overlap.
    let reflection_bounds = rect(-3.0, -3.0, 84.0, 24.0);
    for (scale_x, scale_y) in [(4.0, 4.0), (2.0, 5.0), (5.0, 2.0)] {
      let mapping = PageToRasterMapping {
        width_px: (84.0 * scale_x) as u32,
        height_px: (24.0 * scale_y) as u32,
        scale_x,
        scale_y,
        translate_x: -reflection_bounds.origin.x.0 * scale_x,
        translate_y: -reflection_bounds.origin.y.0 * scale_y,
        text_hinting: None,
      };
      // Use the source's same clipping bands for this ownership oracle.
      // A whole-canvas scan is not a pixel-exact oracle for a clipped scan:
      // the scanner can split curves differently at the band boundary.
      // This checks material/contour ownership, not band-size invariance.
      let mut glyph = RgbaImage::new(mapping.width_px, mapping.height_px);
      for top in (0..mapping.height_px).step_by(64) {
        let band = super::rasterize_vector_items_at_mapping(
          &[DisplayItem::Text(super::text_fill_material_item(authored))],
          PageToRasterMapping {
            height_px: (mapping.height_px - top).min(64),
            translate_y: mapping.translate_y - top as f32,
            ..mapping
          },
          RasterPrimitiveAntialiasing::Aliased,
        )
        .unwrap();
        image::imageops::replace(&mut glyph, &band, 0, i64::from(top));
      }
      for width in [0.0, 0.5, 1.0, 2.0] {
        let make = |value| {
          super::rasterize_static_3d_text_reflection_source_with_mapping(
            authored,
            mapping,
            Pt(width),
            Color {
              r: value,
              g: value,
              b: value,
              a: 255,
            },
            super::RasterPrimitiveAntialiasing::Aliased,
          )
          .unwrap()
        };
        let black = make(0);
        let white = make(255);
        let mut exterior_response = 0;
        for (index, ((b, w), g)) in black
          .pixels()
          .zip(white.pixels())
          .zip(glyph.pixels())
          .enumerate()
        {
          assert_eq!(b.0[3], w.0[3]);
          if g.0[3] != 0 {
            assert_eq!(
              b,
              w,
              "contour must not recolor the raw glyph interior: scale={scale_x},{scale_y}, width={width}, pixel={},{}",
              index as u32 % glyph.width(),
              index as u32 / glyph.width()
            );
          } else if width == 0.0 {
            assert_eq!(
              b.0[3], 0,
              "character material cannot grow outside its solid"
            );
          } else if b != w {
            exterior_response += 1;
          }
        }
        if width == 0.0 {
          assert_eq!(black, white);
        } else {
          assert!(exterior_response > 0);
        }
      }
    }

    let mut line_layer = Vec::new();
    collect_source_layer_item(&item, SourceLayer::Line, &mut line_layer).unwrap();
    let DisplayItem::Text(line_text) = &line_layer[0] else {
      panic!("text line source must remain a text item");
    };
    assert_eq!(line_text.style.outline_color, Some(outline_color));
    assert_eq!(line_text.style.outline_width, Pt(2.0));
    let line_options = line_text.style.pdf_glyph_outline_options.as_ref().unwrap();
    assert_eq!(line_options.fill, Some(Fill::None));
    assert_eq!(line_options.outline_fill, Some(Fill::Solid(outline_color)));
    assert!(line_options.outline_stroke.is_some());

    let material = text_outline_material_item(match &item {
      DisplayItem::Text(text) => text,
      _ => unreachable!(),
    })
    .expect("positive text outline material");
    assert_eq!(material.style.outline_color, None);
    assert_eq!(material.style.outline_width, Pt(0.0));
    let material_options = material.style.pdf_glyph_outline_options.as_ref().unwrap();
    assert_eq!(material_options.fill, Some(Fill::Solid(outline_color)));
    assert_eq!(material_options.outline_fill, None);
    assert_eq!(material_options.outline_stroke, None);

    let DisplayItem::Text(text) = &item else {
      unreachable!();
    };
    let bounds = rect(-2.0, -2.0, 80.0, 20.0);
    let coverage =
      super::rasterize_text_outline_coverage_layer_at_pixels_per_point(text, bounds, 2.0).unwrap();
    // Coverage is a geometric attribute: authored RGB and partial opacity
    // must not change it, while a wholly invisible outline has no layer.
    for alpha in [1, 127, 255] {
      let mut recolored = text.clone();
      let options = Arc::make_mut(recolored.style.pdf_glyph_outline_options.as_mut().unwrap());
      options.outline_fill = Some(Fill::Solid(Color {
        r: 240,
        g: 20,
        b: 90,
        a: alpha,
      }));
      let actual =
        super::rasterize_text_outline_coverage_layer_at_pixels_per_point(&recolored, bounds, 2.0)
          .unwrap();
      assert_eq!(coverage.image, actual.image);
    }
    let mut invisible = text.clone();
    Arc::make_mut(invisible.style.pdf_glyph_outline_options.as_mut().unwrap()).outline_fill =
      Some(Fill::None);
    assert!(
      super::rasterize_text_outline_coverage_layer_at_pixels_per_point(&invisible, bounds, 2.0,)
        .is_none()
    );
    assert!(
      coverage.image.pixels().any(|pixel| {
        let alpha = f32::from(pixel[3]);
        let grid_alpha = (alpha * 16.0 / 255.0).round() * 255.0 / 16.0;
        (alpha - grid_alpha).abs() > 2.0
      }),
      "material coverage must retain more than 4x4 quantized edge levels"
    );
  }

  #[test]
  fn static_3d_text_outline_material_keeps_geometry_and_source_paint_separate() {
    let outline_color = Color {
      r: 20,
      g: 80,
      b: 220,
      a: 153,
    };
    let mut item = TextRun {
      text: Cow::Borrowed("III"),
      origin: Point::default(),
      line_height: Pt(12.0),
      line_metrics_participant: true,
      paint_clip: None,
      page_culling_bounds: None,
      style: TextStyle {
        outline_color: Some(outline_color),
        outline_width: Pt(2.0),
        pdf_glyph_outline_options: Some(Arc::new(PdfGlyphOutlineOptions {
          outline_fill: Some(Fill::Solid(outline_color)),
          outline_stroke: Some(Stroke {
            width: Pt(2.0),
            color: outline_color,
            ..Stroke::default()
          }),
          ..PdfGlyphOutlineOptions::default()
        })),
        ..TextStyle::default()
      },
      font_id: None,
      color: Color::default(),
      rotation_center: None,
      hyperlink_url: None,
      dynamic_field: None,
      form_widget_id: None,
      paragraph_bidi: false,
      word_spacing_pt: 0.0,
      preserve_text_portion: false,
      pdf_text_segmentation: Default::default(),
      source: None,
    };
    let pixels_per_point = 200.0 / 72.0;

    // Paint realization must not alter the input used by physical geometry.
    // The inset operation follows expansion, not the other way around.
    for size in [18.0, 24.0, 36.0, 48.0] {
      for width in [0.0, 0.5, 1.0, 2.0, 4.0, 4.4, 4.6, 5.0, 6.0] {
        for alignment in [
          None,
          Some(StrokeAlignment::Center),
          Some(StrokeAlignment::Inside),
        ] {
          let mut input = item.clone();
          input.style.font_size = Pt(size);
          input.style.outline_width = Pt(width);
          let options = Arc::make_mut(input.style.pdf_glyph_outline_options.as_mut().unwrap());
          let stroke = options.outline_stroke.as_mut().unwrap();
          stroke.width = Pt(width);
          stroke.alignment = alignment;
          let result = super::text_surface_outline_item(&input);
          let options = result.style.pdf_glyph_outline_options.as_ref().unwrap();
          let expanded = if width == 0.0 { 0.0 } else { width + 0.12 };
          let expected = if alignment == Some(StrokeAlignment::Inside) {
            2.0 * expanded
          } else {
            expanded
          };
          assert_eq!(options.fill, Some(Fill::None));
          assert_eq!(options.outline_stroke.as_ref().unwrap().width, Pt(expected));
          assert_eq!(result.style.outline_width, Pt(expanded));
          assert_eq!(input.style.outline_width, Pt(width));
          assert_eq!(
            input
              .style
              .pdf_glyph_outline_options
              .as_ref()
              .unwrap()
              .outline_stroke
              .as_ref()
              .unwrap()
              .width,
            Pt(width)
          );
        }
      }
    }
    let mut no_fill = item.clone();
    Arc::make_mut(no_fill.style.pdf_glyph_outline_options.as_mut().unwrap()).outline_fill =
      Some(Fill::None);
    let suppressed = super::text_surface_outline_item(&no_fill);
    assert!(suppressed.style.outline_color.is_none());
    assert_eq!(suppressed.style.outline_width, Pt(0.0));
    assert!(
      suppressed
        .style
        .pdf_glyph_outline_options
        .as_ref()
        .unwrap()
        .outline_stroke
        .is_none()
    );
    let mut fallback = item.clone();
    fallback.style.pdf_glyph_outline_options = None;
    let expanded_fallback = super::text_surface_outline_item(&fallback);
    assert_eq!(expanded_fallback.style.outline_width, Pt(2.12));
    assert_eq!(fallback.style.outline_width, Pt(2.0));

    assert_eq!(
      super::static_3d_text_material_opacities(&item),
      (Some(f32::from(item.color.a) / 255.0), Some(153.0 / 255.0))
    );
    assert_eq!(super::uniform_static_3d_text_paint_opacity(&item), None);

    // [MS-DOCX] makes an omitted alignment centered, so only half of the
    // authored two-point line lies inside the physical glyph face.
    let centered = static_3d_text_outline_material_inset_px(&item, pixels_per_point).unwrap();
    assert!((centered - pixels_per_point).abs() < 0.000_1);

    let options = Arc::make_mut(item.style.pdf_glyph_outline_options.as_mut().unwrap());
    options.outline_stroke.as_mut().unwrap().alignment = Some(StrokeAlignment::Inside);
    let inside = static_3d_text_outline_material_inset_px(&item, pixels_per_point).unwrap();
    assert!((inside - 2.0 * pixels_per_point).abs() < 0.000_1);

    let options = Arc::make_mut(item.style.pdf_glyph_outline_options.as_mut().unwrap());
    options.outline_fill = Some(Fill::Solid(Color {
      a: 0,
      ..outline_color
    }));
    assert_eq!(
      static_3d_text_outline_material_inset_px(&item, pixels_per_point),
      None
    );
    assert_eq!(super::static_3d_text_material_opacities(&item).1, Some(0.0));
  }

  #[test]
  fn uniform_material_opacity_is_independent_of_gradient_rgb_and_coverage() {
    for alpha in [0, 1, 102, 153, 254, 255] {
      let mut gradient = GradientFill {
        stops: vec![
          GradientStop {
            position: 0.0,
            color: Color {
              r: 0,
              g: 80,
              b: 255,
              a: alpha,
            },
            scheme: None,
          },
          GradientStop {
            position: 1.0,
            color: Color {
              r: 255,
              g: 20,
              b: 0,
              a: alpha,
            },
            scheme: None,
          },
        ],
        ..GradientFill::default()
      };
      assert_eq!(
        super::uniform_fill_opacity(&Fill::Gradient(gradient.clone())),
        Some(f32::from(alpha) / 255.0)
      );
      gradient.stops[1].color.a = alpha.wrapping_add(1);
      assert_eq!(super::uniform_fill_opacity(&Fill::Gradient(gradient)), None);
    }
    assert_eq!(super::uniform_fill_opacity(&Fill::None), Some(0.0));
    assert_eq!(
      super::uniform_fill_opacity(&Fill::Gradient(GradientFill::default())),
      None
    );
  }

  #[test]
  fn source_boundary_uses_widened_strokes_and_source_coordinates() {
    for density in [1.0, 96.0 / 72.0, 200.0 / 72.0] {
      for fill in [
        Fill::None,
        Fill::Solid(Color {
          a: 255,
          ..Color::default()
        }),
      ] {
        let item = DisplayItem::Rect(RectItem {
          bounds: rect(10.0, 20.0, 12.0, 8.0),
          fill: fill.clone(),
          stroke: Some(Stroke {
            width: Pt(2.0),
            color: Color {
              a: 255,
              ..Color::default()
            },
            ..Stroke::default()
          }),
        });
        let boundary =
          super::static_3d_raster_source_boundary(&[item], rect(8.0, 18.0, 16.0, 12.0), density)
            .unwrap();
        assert_eq!(
          boundary.normal_at((density - 0.2, 5.0 * density), [-1.0, 0.0]),
          Some([-1.0, 0.0])
        );
        let inside = boundary.normal_at((3.0 * density + 0.2, 5.0 * density), [1.0, 0.0]);
        assert_eq!(
          inside,
          if matches!(fill, Fill::None) {
            Some([1.0, 0.0])
          } else {
            None
          }
        );
      }
    }
    let translucent = DisplayItem::Rect(RectItem {
      bounds: rect(0.0, 0.0, 10.0, 10.0),
      fill: Fill::Solid(Color {
        a: 128,
        ..Color::default()
      }),
      stroke: None,
    });
    assert!(
      super::static_3d_raster_source_boundary(&[translucent], rect(0.0, 0.0, 10.0, 10.0), 1.0)
        .is_none()
    );
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
    let raster =
      rasterize_group_items_for_effects(std::slice::from_ref(&item), bounds, &effects).unwrap();
    assert_eq!(raster.fill_image.unwrap().get_pixel(5, 5).0, [0, 0, 0, 0]);
    assert_eq!(
      raster.children_image.unwrap().get_pixel(5, 5).0,
      [255, 0, 0, 255]
    );

    let raster = rasterize_group_items_for_effects_at_pixels_per_point(
      std::slice::from_ref(&item),
      bounds,
      &effects,
      3.0,
    )
    .unwrap();
    assert_eq!((raster.image.width(), raster.image.height()), (30, 30));
    assert_eq!(
      raster
        .fill_image
        .as_ref()
        .map(|image| (image.width(), image.height())),
      Some((30, 30))
    );
    assert_eq!(
      raster
        .children_image
        .as_ref()
        .map(|image| (image.width(), image.height())),
      Some((30, 30))
    );

    let office_glow_bounds = Rect {
      origin: Point::default(),
      size: Size {
        width: Pt(164.000_02),
        height: Pt(138.5),
      },
    };
    let office_glow_density = 200.0 / 72.0 / 7.0;
    let outward = rasterize_group_items_for_effects_at_pixels_per_point(
      std::slice::from_ref(&item),
      office_glow_bounds,
      &effects,
      office_glow_density,
    )
    .unwrap();
    assert_eq!((outward.image.width(), outward.image.height()), (66, 55));
    let rounded = rasterize_group_items_for_effects_at_pixels_per_point_with_extent(
      std::slice::from_ref(&item),
      office_glow_bounds,
      &effects,
      office_glow_density,
      RasterSourceExtent::Round,
    )
    .unwrap();
    assert_eq!((rounded.image.width(), rounded.image.height()), (65, 55));
    assert_eq!(
      rounded
        .children_image
        .as_ref()
        .map(|image| (image.width(), image.height())),
      Some((65, 55))
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
      bytes: Bytes::from(png),
      blip_compression_state: crate::common::BlipCompressionState::Unspecified,
      metafile_monochrome_dib_palette_override: None,
      metafile_background_color: None,
      metafile_external_header: None,
      metafile_fixed_output_profile: crate::common::MetafileFixedOutputProfile::Default,
      relationship_id: None,
      alt_text: None,
      hyperlink_url: None,
      semantic_metafile_text: false,
      metafile_semantic_text_includes_raster_backdrop: false,
      signature_line: None,
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
          context: GradientPathContext::DrawingObject,
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
