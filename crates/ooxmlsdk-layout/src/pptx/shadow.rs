use std::io::Cursor;
use std::sync::Arc;

use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use kurbo::{
  Affine, BezPath, Cap as KurboCap, Join as KurboJoin, PathEl, Shape as KurboShape,
  Stroke as KurboStroke, StrokeOpts, flatten, stroke,
};
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::common::{PathCommand, Point};
use crate::model::{ImageCrop, ImageItem, RgbColor};
use crate::units;

use super::drawingml::shape_properties::{
  EffectGlowProperties, EffectReflectionProperties, EffectShadowProperties,
  EffectSoftEdgeProperties,
};
use crate::common::drawingml_geometry::mapped_path_elements;

// LibreOffice caps the temporary bitmap used by a blurred shadow at 250,000
// pixels. Keep the same bound so large Office shapes do not turn into
// unbounded PDF resources.
const MAX_SHADOW_RASTER_PIXELS: f32 = 250_000.0;
const MAX_SHADOW_PIXELS_PER_POINT: f32 = 2.0;
const MAX_STACK_BLUR_RADIUS_PX: f32 = 254.0;
// Match Kurbo's own SVG curve-lowering tolerance, expressed here in the
// shadow mask's pixel coordinate space.
const SHADOW_CURVE_FLATTEN_TOLERANCE_PX: f64 = 0.1;

#[derive(Clone, Copy, Debug)]
pub(crate) struct ShadowFrame {
  pub(crate) x_pt: f32,
  pub(crate) y_pt: f32,
  pub(crate) width_pt: f32,
  pub(crate) height_pt: f32,
  pub(crate) stroke_width_pt: f32,
}

pub(crate) fn outer_shadow_image_item(
  effect: &EffectShadowProperties,
  frame: ShadowFrame,
  shape_path: &[PathCommand],
  transform: crate::common::Transform,
  has_fill: bool,
  stroke_style: Option<&crate::common::Stroke<'_>>,
  color: RgbColor,
  opacity: f32,
) -> Option<ImageItem> {
  if frame.width_pt <= 0.0 || frame.height_pt <= 0.0 || opacity <= 0.0 {
    return None;
  }

  let transform_point = |point: Point| {
    (
      transform.m11 * point.x.0 + transform.m21 * point.y.0 + transform.dx.0,
      transform.m12 * point.x.0 + transform.m22 * point.y.0 + transform.dy.0,
    )
  };
  let transformed_elements = mapped_path_elements(shape_path, |point| {
    let (x, y) = transform_point(point);
    kurbo::Point::new(f64::from(x), f64::from(y))
  });
  let transformed_bounds = (!transformed_elements.is_empty())
    .then(|| BezPath::from_vec(transformed_elements.clone()).bounding_box())
    .filter(|bounds| bounds.width() > 0.0 && bounds.height() > 0.0)
    .unwrap_or_else(|| {
      let corners = [
        Point {
          x: crate::common::Pt(frame.x_pt),
          y: crate::common::Pt(frame.y_pt),
        },
        Point {
          x: crate::common::Pt(frame.x_pt + frame.width_pt),
          y: crate::common::Pt(frame.y_pt),
        },
        Point {
          x: crate::common::Pt(frame.x_pt + frame.width_pt),
          y: crate::common::Pt(frame.y_pt + frame.height_pt),
        },
        Point {
          x: crate::common::Pt(frame.x_pt),
          y: crate::common::Pt(frame.y_pt + frame.height_pt),
        },
      ];
      let transformed = corners.map(transform_point);
      let left = transformed
        .iter()
        .map(|point| point.0)
        .fold(f32::INFINITY, f32::min);
      let top = transformed
        .iter()
        .map(|point| point.1)
        .fold(f32::INFINITY, f32::min);
      let right = transformed
        .iter()
        .map(|point| point.0)
        .fold(f32::NEG_INFINITY, f32::max);
      let bottom = transformed
        .iter()
        .map(|point| point.1)
        .fold(f32::NEG_INFINITY, f32::max);
      kurbo::Rect::new(
        f64::from(left),
        f64::from(top),
        f64::from(right),
        f64::from(bottom),
      )
    });
  // The Frobenius norm is a conservative upper bound for the largest
  // singular value, so skewed strokes cannot be clipped by the raster box.
  let stroke_scale = (transform.m11 * transform.m11
    + transform.m12 * transform.m12
    + transform.m21 * transform.m21
    + transform.m22 * transform.m22)
    .sqrt();
  let stroke_outset_pt = frame.stroke_width_pt.max(0.0) * stroke_scale / 2.0;
  let blur_pt = units::emu_to_points(effect.blur_radius_emu.unwrap_or_default()).max(0.0);
  let image_x_pt = transformed_bounds.x0 as f32 - stroke_outset_pt - blur_pt;
  let image_y_pt = transformed_bounds.y0 as f32 - stroke_outset_pt - blur_pt;
  let image_width_pt = transformed_bounds.width() as f32 + (stroke_outset_pt + blur_pt) * 2.0;
  let image_height_pt = transformed_bounds.height() as f32 + (stroke_outset_pt + blur_pt) * 2.0;
  if image_width_pt <= 0.0 || image_height_pt <= 0.0 {
    return None;
  }

  let mut pixels_per_point = (MAX_SHADOW_RASTER_PIXELS / (image_width_pt * image_height_pt)).sqrt();
  pixels_per_point = pixels_per_point.min(MAX_SHADOW_PIXELS_PER_POINT);
  if blur_pt > 0.0 {
    pixels_per_point = pixels_per_point.min(MAX_STACK_BLUR_RADIUS_PX / blur_pt);
  }
  pixels_per_point = pixels_per_point.max(0.25);
  let width_px = (image_width_pt * pixels_per_point).ceil().max(1.0) as u32;
  let height_px = (image_height_pt * pixels_per_point).ceil().max(1.0) as u32;
  let mut alpha = vec![0_u8; width_px as usize * height_px as usize];

  let path_filled = has_fill
    && !shape_path.is_empty()
    && fill_path_alpha(
      &mut alpha,
      width_px as usize,
      height_px as usize,
      shape_path,
      |point| {
        let (x, y) = transform_point(point);
        (
          (x - image_x_pt) * pixels_per_point,
          (y - image_y_pt) * pixels_per_point,
        )
      },
    );
  if let Some(stroke_style) = stroke_style
    && stroke_style.width.0 > 0.0
    && !shape_path.is_empty()
  {
    let source_elements = mapped_path_elements(shape_path, |point| {
      kurbo::Point::new(f64::from(point.x.0), f64::from(point.y.0))
    });
    let outline = stroke(
      source_elements,
      &kurbo_stroke_style(stroke_style, 1.0),
      &StrokeOpts::default(),
      SHADOW_CURVE_FLATTEN_TOLERANCE_PX,
    );
    let effect_affine = Affine::new([
      f64::from(transform.m11),
      f64::from(transform.m12),
      f64::from(transform.m21),
      f64::from(transform.m22),
      f64::from(transform.dx.0),
      f64::from(transform.dy.0),
    ]);
    let raster_affine = Affine::new([
      f64::from(pixels_per_point),
      0.0,
      0.0,
      f64::from(pixels_per_point),
      -f64::from(image_x_pt * pixels_per_point),
      -f64::from(image_y_pt * pixels_per_point),
    ]);
    let outline = raster_affine * effect_affine * outline;
    fill_elements_alpha(
      &mut alpha,
      width_px as usize,
      height_px as usize,
      outline.iter(),
    );
  }
  if !path_filled && has_fill && alpha.iter().all(|value| *value == 0) {
    let left_px = ((transformed_bounds.x0 as f32 - image_x_pt) * pixels_per_point)
      .floor()
      .max(0.0) as u32;
    let top_px = ((transformed_bounds.y0 as f32 - image_y_pt) * pixels_per_point)
      .floor()
      .max(0.0) as u32;
    let right_px = ((transformed_bounds.x1 as f32 - image_x_pt) * pixels_per_point)
      .ceil()
      .clamp(0.0, width_px as f32) as u32;
    let bottom_px = ((transformed_bounds.y1 as f32 - image_y_pt) * pixels_per_point)
      .ceil()
      .clamp(0.0, height_px as f32) as u32;
    for y in top_px..bottom_px {
      let row = y as usize * width_px as usize;
      alpha[row + left_px as usize..row + right_px as usize].fill(255);
    }
  }
  if alpha.iter().all(|value| *value == 0) {
    return None;
  }

  let blur_radius_px = (blur_pt * pixels_per_point).ceil() as usize;
  if blur_radius_px > 0 {
    stack_blur_alpha(
      &mut alpha,
      width_px as usize,
      height_px as usize,
      blur_radius_px,
    );
  }

  let mut rgba = Vec::with_capacity(alpha.len() * 4);
  for mask in alpha {
    rgba.extend_from_slice(&[
      color.r,
      color.g,
      color.b,
      (f32::from(mask) * opacity.clamp(0.0, 1.0)).round() as u8,
    ]);
  }
  let mut png = Cursor::new(Vec::new());
  PngEncoder::new(&mut png)
    .write_image(&rgba, width_px, height_px, ColorType::Rgba8.into())
    .ok()?;

  Some(ImageItem {
    x_pt: image_x_pt,
    y_pt: image_y_pt,
    width_pt: image_width_pt,
    height_pt: image_height_pt,
    crop: ImageCrop::default(),
    clip_path: Vec::new(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    data: Arc::from(png.into_inner()),
    content_type: Some("image/png".to_string()),
    alt_text: None,
    hyperlink_url: None,
    floating: false,
    behind_text: true,
  })
}

/// Rasterizes an Office inner shadow as the blurred complement of a shifted
/// shape mask, clipped back to the original shape.
///
/// ECMA-376 Part 1 §20.1.8.40 requires the shadow to stay within the object
/// edges. `[MS-OI29500]` further defines `dir` as clockwise from the left,
/// which differs from the usual DrawingML right-origin angle convention.
pub(crate) fn inner_shadow_image_item(
  effect: &EffectShadowProperties,
  frame: ShadowFrame,
  shape_path: &[PathCommand],
  has_fill: bool,
  stroke_style: Option<&crate::common::Stroke<'_>>,
  color: RgbColor,
  opacity: f32,
) -> Option<ImageItem> {
  if frame.width_pt <= 0.0 || frame.height_pt <= 0.0 || opacity <= 0.0 {
    return None;
  }

  let blur_pt = units::emu_to_points(effect.blur_radius_emu.unwrap_or_default()).max(0.0);
  let (offset_x_pt, offset_y_pt) = office_inner_shadow_offset(
    effect.direction.unwrap_or_default(),
    units::emu_to_points(effect.distance_emu.unwrap_or_default()),
  );
  if blur_pt <= 0.0 && offset_x_pt.abs() <= f32::EPSILON && offset_y_pt.abs() <= f32::EPSILON {
    return None;
  }

  let mut pixels_per_point = (MAX_SHADOW_RASTER_PIXELS / (frame.width_pt * frame.height_pt)).sqrt();
  pixels_per_point = pixels_per_point.min(MAX_SHADOW_PIXELS_PER_POINT);
  if blur_pt > 0.0 {
    pixels_per_point = pixels_per_point.min(MAX_STACK_BLUR_RADIUS_PX / blur_pt);
  }
  pixels_per_point = pixels_per_point.max(0.25);
  let width_px = (frame.width_pt * pixels_per_point).ceil().max(1.0) as u32;
  let height_px = (frame.height_pt * pixels_per_point).ceil().max(1.0) as u32;
  let pixel_count = width_px as usize * height_px as usize;
  let mut clip_alpha = vec![0_u8; pixel_count];
  let mut shifted_alpha = vec![0_u8; pixel_count];

  let shape_rasterized = raster_shape_alpha(
    &mut clip_alpha,
    width_px as usize,
    height_px as usize,
    shape_path,
    has_fill,
    stroke_style,
    pixels_per_point,
    |point| {
      (
        (point.x.0 - frame.x_pt) * pixels_per_point,
        (point.y.0 - frame.y_pt) * pixels_per_point,
      )
    },
  );
  if shape_rasterized {
    raster_shape_alpha(
      &mut shifted_alpha,
      width_px as usize,
      height_px as usize,
      shape_path,
      has_fill,
      stroke_style,
      pixels_per_point,
      |point| {
        (
          (point.x.0 + offset_x_pt - frame.x_pt) * pixels_per_point,
          (point.y.0 + offset_y_pt - frame.y_pt) * pixels_per_point,
        )
      },
    );
  } else if has_fill {
    clip_alpha.fill(u8::MAX);
    fill_shifted_rectangle_alpha(
      &mut shifted_alpha,
      width_px as usize,
      height_px as usize,
      offset_x_pt * pixels_per_point,
      offset_y_pt * pixels_per_point,
    );
  } else {
    return None;
  }

  let blur_radius_px = (blur_pt * pixels_per_point).ceil() as usize;
  if blur_radius_px > 0 {
    stack_blur_alpha(
      &mut shifted_alpha,
      width_px as usize,
      height_px as usize,
      blur_radius_px,
    );
  }

  let mut rgba = Vec::with_capacity(pixel_count * 4);
  for (clip, shifted) in clip_alpha.into_iter().zip(shifted_alpha) {
    let inset = u16::from(clip) * u16::from(u8::MAX - shifted) / u16::from(u8::MAX);
    rgba.extend_from_slice(&[
      color.r,
      color.g,
      color.b,
      (f32::from(inset) * opacity.clamp(0.0, 1.0)).round() as u8,
    ]);
  }
  let mut png = Cursor::new(Vec::new());
  PngEncoder::new(&mut png)
    .write_image(&rgba, width_px, height_px, ColorType::Rgba8.into())
    .ok()?;

  Some(ImageItem {
    x_pt: frame.x_pt,
    y_pt: frame.y_pt,
    width_pt: frame.width_pt,
    height_pt: frame.height_pt,
    crop: ImageCrop::default(),
    clip_path: Vec::new(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    data: Arc::from(png.into_inner()),
    content_type: Some("image/png".to_string()),
    alt_text: None,
    hyperlink_url: None,
    floating: false,
    behind_text: false,
  })
}

/// Builds the shared alpha mask for DrawingML soft edges.
///
/// LibreOffice `SoftEdgePrimitive2D` rasterizes the complete non-text shape
/// content within a 250,000-pixel budget, erodes the alpha by `rad` with a
/// square kernel, blurs it by the same radius, then multiplies that result by
/// the original alpha. `[MS-OI29500]` confirms that this is an inset effect:
/// only the band from the shape boundary to `rad` is softened.
pub(crate) fn soft_edge_mask_image_item(
  effect: &EffectSoftEdgeProperties,
  frame: ShadowFrame,
  shape_path: &[PathCommand],
  has_fill: bool,
  stroke_style: Option<&crate::common::Stroke<'_>>,
) -> Option<ImageItem> {
  let radius_pt = units::emu_to_points(effect.radius_emu).max(0.0);
  if radius_pt <= 0.0 || frame.width_pt <= 0.0 || frame.height_pt <= 0.0 {
    return None;
  }

  let mut pixels_per_point = (MAX_SHADOW_RASTER_PIXELS / (frame.width_pt * frame.height_pt)).sqrt();
  pixels_per_point = pixels_per_point.min(MAX_SHADOW_PIXELS_PER_POINT);
  pixels_per_point = pixels_per_point.min(MAX_STACK_BLUR_RADIUS_PX / radius_pt);
  pixels_per_point = pixels_per_point.max(0.25);
  let width_px = (frame.width_pt * pixels_per_point).ceil().max(1.0) as u32;
  let height_px = (frame.height_pt * pixels_per_point).ceil().max(1.0) as u32;
  let mut original_alpha = vec![0_u8; width_px as usize * height_px as usize];
  let path_elements = mapped_path_elements(shape_path, |point| {
    kurbo::Point::new(
      f64::from((point.x.0 - frame.x_pt) * pixels_per_point),
      f64::from((point.y.0 - frame.y_pt) * pixels_per_point),
    )
  });

  if has_fill {
    if path_elements.is_empty() {
      original_alpha.fill(u8::MAX);
    } else {
      fill_elements_alpha(
        &mut original_alpha,
        width_px as usize,
        height_px as usize,
        path_elements.iter().copied(),
      );
    }
  }
  if let Some(stroke_style) = stroke_style
    && stroke_style.width.0 > 0.0
    && !path_elements.is_empty()
  {
    let outline = stroke(
      path_elements.iter().copied(),
      &kurbo_stroke_style(stroke_style, pixels_per_point),
      &StrokeOpts::default(),
      SHADOW_CURVE_FLATTEN_TOLERANCE_PX,
    );
    let mut stroke_alpha = vec![0_u8; original_alpha.len()];
    fill_elements_alpha(
      &mut stroke_alpha,
      width_px as usize,
      height_px as usize,
      outline.iter(),
    );
    for (original, stroke) in original_alpha.iter_mut().zip(stroke_alpha) {
      *original = (*original).max(stroke);
    }
  }
  if original_alpha.iter().all(|alpha| *alpha == 0) {
    return None;
  }

  let radius_px = (radius_pt * pixels_per_point).ceil().max(1.0) as usize;
  let mut softened_alpha = erode_square_alpha(
    &original_alpha,
    width_px as usize,
    height_px as usize,
    radius_px,
  );
  stack_blur_alpha(
    &mut softened_alpha,
    width_px as usize,
    height_px as usize,
    radius_px,
  );
  for (softened, original) in softened_alpha.iter_mut().zip(original_alpha) {
    *softened = (u16::from(*softened) * u16::from(original) / u16::from(u8::MAX)) as u8;
  }

  let mut rgba = Vec::with_capacity(softened_alpha.len() * 4);
  for alpha in softened_alpha {
    rgba.extend_from_slice(&[u8::MAX, u8::MAX, u8::MAX, alpha]);
  }
  let mut png = Cursor::new(Vec::new());
  PngEncoder::new(&mut png)
    .write_image(&rgba, width_px, height_px, ColorType::Rgba8.into())
    .ok()?;

  Some(ImageItem {
    x_pt: frame.x_pt,
    y_pt: frame.y_pt,
    width_pt: frame.width_pt,
    height_pt: frame.height_pt,
    crop: ImageCrop::default(),
    clip_path: Vec::new(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    data: Arc::from(png.into_inner()),
    content_type: Some("image/png".to_string()),
    alt_text: None,
    hyperlink_url: None,
    floating: false,
    behind_text: false,
  })
}

/// Builds the linear alpha ramp used by a zero-blur DrawingML reflection.
///
/// ECMA-376 Part 1 §20.1.8.50 defines the two alpha/position pairs and
/// `[MS-OI29500]` corrects the exchanged prose for `fadeDir` and `dir`:
/// `fadeDir` controls this ramp, while `dir` controls the reflected copy's
/// offset. The remaining reflection transform stays vector-native.
///
/// A non-zero reflection blur affects every color channel and therefore
/// cannot be represented by merely blurring this alpha mask. Such a
/// reflection is retained in the parsed effect model until the bounded
/// full-group raster stage is available.
pub(crate) fn reflection_mask_image_item(
  effect: &EffectReflectionProperties,
  frame: ShadowFrame,
  rotation_deg: f32,
  flip_horizontal: bool,
  flip_vertical: bool,
) -> Option<ImageItem> {
  if effect.blur_radius_emu.unwrap_or_default() != 0
    || frame.width_pt <= 0.0
    || frame.height_pt <= 0.0
  {
    return None;
  }

  // Defaults are confirmed by LibreOffice
  // `oox/source/drawingml/effectpropertiescontext.cxx`.
  let start_opacity = effect.start_opacity.unwrap_or(0.0).clamp(0.0, 1.0);
  let start_position = effect.start_position.unwrap_or(1.0).clamp(0.0, 1.0);
  let end_opacity = effect.end_opacity.unwrap_or(1.0).clamp(0.0, 1.0);
  let end_position = effect.end_position.unwrap_or(0.0).clamp(0.0, 1.0);
  if start_opacity <= 0.0 && end_opacity <= 0.0 {
    return None;
  }

  let pixels_per_point = (MAX_SHADOW_RASTER_PIXELS / (frame.width_pt * frame.height_pt))
    .sqrt()
    .min(MAX_SHADOW_PIXELS_PER_POINT)
    .max(0.25);
  let width_px = (frame.width_pt * pixels_per_point).ceil().max(1.0) as u32;
  let height_px = (frame.height_pt * pixels_per_point).ceil().max(1.0) as u32;
  let fade_radians =
    effect.fade_direction.unwrap_or(5_400_000) as f32 / 60_000.0 * 1.0_f32.to_radians();
  let direction_x = fade_radians.cos();
  let direction_y = fade_radians.sin();
  let projections = [
    0.0,
    direction_x * frame.width_pt,
    direction_y * frame.height_pt,
    direction_x * frame.width_pt + direction_y * frame.height_pt,
  ];
  let projection_min = projections.iter().copied().fold(f32::INFINITY, f32::min);
  let projection_max = projections
    .iter()
    .copied()
    .fold(f32::NEG_INFINITY, f32::max);
  let projection_span = (projection_max - projection_min).max(f32::EPSILON);
  let mut rgba = Vec::with_capacity(width_px as usize * height_px as usize * 4);
  for y in 0..height_px {
    let y_pt = (y as f32 + 0.5) * frame.height_pt / height_px as f32;
    for x in 0..width_px {
      let x_pt = (x as f32 + 0.5) * frame.width_pt / width_px as f32;
      let position = (direction_x * x_pt + direction_y * y_pt - projection_min) / projection_span;
      let opacity = reflection_opacity(
        position,
        (start_position, start_opacity),
        (end_position, end_opacity),
      );
      rgba.extend_from_slice(&[
        u8::MAX,
        u8::MAX,
        u8::MAX,
        (opacity * f32::from(u8::MAX)).round() as u8,
      ]);
    }
  }

  let mut png = Cursor::new(Vec::new());
  PngEncoder::new(&mut png)
    .write_image(&rgba, width_px, height_px, ColorType::Rgba8.into())
    .ok()?;
  Some(ImageItem {
    x_pt: frame.x_pt,
    y_pt: frame.y_pt,
    width_pt: frame.width_pt,
    height_pt: frame.height_pt,
    crop: ImageCrop::default(),
    clip_path: Vec::new(),
    rotation_deg,
    flip_horizontal,
    flip_vertical,
    data: Arc::from(png.into_inner()),
    content_type: Some("image/png".to_string()),
    alt_text: None,
    hyperlink_url: None,
    floating: false,
    behind_text: true,
  })
}

fn raster_shape_alpha(
  alpha: &mut [u8],
  width: usize,
  height: usize,
  shape_path: &[PathCommand],
  has_fill: bool,
  stroke_style: Option<&crate::common::Stroke<'_>>,
  coordinate_scale: f32,
  map: impl Fn(Point) -> (f32, f32),
) -> bool {
  if shape_path.is_empty() {
    return false;
  }
  let path_elements = mapped_path_elements(shape_path, |point| {
    let (x, y) = map(point);
    kurbo::Point::new(f64::from(x), f64::from(y))
  });
  if has_fill {
    fill_elements_alpha(alpha, width, height, path_elements.iter().copied());
  }
  if let Some(stroke_style) = stroke_style
    && stroke_style.width.0 > 0.0
  {
    let outline = stroke(
      path_elements,
      &kurbo_stroke_style(stroke_style, coordinate_scale),
      &StrokeOpts::default(),
      SHADOW_CURVE_FLATTEN_TOLERANCE_PX,
    );
    fill_elements_alpha(alpha, width, height, outline.iter());
  }
  alpha.iter().any(|value| *value != 0)
}

fn kurbo_stroke_style(
  stroke_style: &crate::common::Stroke<'_>,
  coordinate_scale: f32,
) -> KurboStroke {
  let mut style = KurboStroke::new(f64::from(stroke_style.width.0 * coordinate_scale));
  style.join = match stroke_style.join {
    Some(crate::common::StrokeJoin::Round) => KurboJoin::Round,
    Some(crate::common::StrokeJoin::Bevel) => KurboJoin::Bevel,
    Some(crate::common::StrokeJoin::Miter { .. }) | None => KurboJoin::Miter,
  };
  style.miter_limit = match stroke_style.join {
    Some(crate::common::StrokeJoin::Miter { limit: Some(limit) }) => f64::from(limit),
    _ => KurboStroke::default().miter_limit,
  };
  let cap = match stroke_style.cap {
    Some(crate::common::StrokeCap::Round) => KurboCap::Round,
    Some(crate::common::StrokeCap::Square) => KurboCap::Square,
    Some(crate::common::StrokeCap::Flat) | None => KurboCap::Butt,
  };
  style.start_cap = cap;
  style.end_cap = cap;
  if let Some(dash) = stroke_style.resolved_dash() {
    style = style.with_dashes(
      f64::from(stroke_style.dash_offset.0 * coordinate_scale),
      dash
        .iter()
        .map(|length| f64::from(length.0 * coordinate_scale)),
    );
  }
  style
}

fn reflection_opacity(position: f32, first: (f32, f32), second: (f32, f32)) -> f32 {
  let (lower, upper) = if first.0 <= second.0 {
    (first, second)
  } else {
    (second, first)
  };
  if position <= lower.0 {
    return lower.1;
  }
  if position >= upper.0 {
    return upper.1;
  }
  let span = upper.0 - lower.0;
  if span <= f32::EPSILON {
    return upper.1;
  }
  lower.1 + (upper.1 - lower.1) * ((position - lower.0) / span)
}

fn erode_square_alpha(alpha: &[u8], width: usize, height: usize, radius: usize) -> Vec<u8> {
  if radius == 0 {
    return alpha.to_vec();
  }
  let integral_width = width + 1;
  let mut integral = vec![0_u64; integral_width * (height + 1)];
  for y in 0..height {
    let mut row_sum = 0_u64;
    for x in 0..width {
      row_sum += u64::from(alpha[y * width + x]);
      integral[(y + 1) * integral_width + x + 1] = integral[y * integral_width + x + 1] + row_sum;
    }
  }

  let mut eroded = vec![0_u8; alpha.len()];
  for y in radius..height.saturating_sub(radius) {
    let top = y - radius;
    let bottom = y + radius + 1;
    for x in radius..width.saturating_sub(radius) {
      let left = x - radius;
      let right = x + radius + 1;
      let sum = integral[bottom * integral_width + right] + integral[top * integral_width + left]
        - integral[top * integral_width + right]
        - integral[bottom * integral_width + left];
      let area = (right - left) as u64 * (bottom - top) as u64;
      if sum == area * u64::from(u8::MAX) {
        eroded[y * width + x] = u8::MAX;
      }
    }
  }
  eroded
}

fn office_inner_shadow_offset(direction: i32, distance_pt: f32) -> (f32, f32) {
  let angle = direction as f32 / 60_000.0 * 1.0_f32.to_radians();
  (-angle.cos() * distance_pt, -angle.sin() * distance_pt)
}

fn fill_shifted_rectangle_alpha(
  alpha: &mut [u8],
  width: usize,
  height: usize,
  offset_x: f32,
  offset_y: f32,
) {
  let left = offset_x.floor().clamp(0.0, width as f32) as usize;
  let top = offset_y.floor().clamp(0.0, height as f32) as usize;
  let right = (width as f32 + offset_x).ceil().clamp(0.0, width as f32) as usize;
  let bottom = (height as f32 + offset_y).ceil().clamp(0.0, height as f32) as usize;
  for y in top..bottom {
    alpha[y * width + left..y * width + right].fill(u8::MAX);
  }
}

/// Rasterizes the DrawingML glow halo while retaining the authored shape as
/// independent vector content above it.
///
/// LibreOffice `GlowPrimitive2D` grows the content range by `rad` and blurs a
/// sharp alpha boundary with half that radius because the blur spreads to
/// both sides. It also uses the same 250,000-pixel safety budget as blurred
/// shadows. Keeping the halo in a separate image preserves the documented
/// effect order: glow behind the original shape.
pub(crate) fn glow_image_item(
  effect: &EffectGlowProperties,
  frame: ShadowFrame,
  shape_path: &[PathCommand],
  has_fill: bool,
  stroke_style: Option<&crate::common::Stroke<'_>>,
  color: RgbColor,
  opacity: f32,
) -> Option<ImageItem> {
  let radius_pt = units::emu_to_points(effect.radius_emu?).max(0.0);
  if radius_pt <= 0.0 || frame.width_pt <= 0.0 || frame.height_pt <= 0.0 || opacity <= 0.0 {
    return None;
  }

  let stroke_outset_pt = frame.stroke_width_pt.max(0.0) / 2.0;
  let image_x_pt = frame.x_pt - stroke_outset_pt - radius_pt;
  let image_y_pt = frame.y_pt - stroke_outset_pt - radius_pt;
  let image_width_pt = frame.width_pt + (stroke_outset_pt + radius_pt) * 2.0;
  let image_height_pt = frame.height_pt + (stroke_outset_pt + radius_pt) * 2.0;
  let mut pixels_per_point = (MAX_SHADOW_RASTER_PIXELS / (image_width_pt * image_height_pt)).sqrt();
  pixels_per_point = pixels_per_point.min(MAX_SHADOW_PIXELS_PER_POINT);
  let blur_radius_pt = radius_pt / 2.0;
  if blur_radius_pt > 0.0 {
    pixels_per_point = pixels_per_point.min(MAX_STACK_BLUR_RADIUS_PX / blur_radius_pt);
  }
  pixels_per_point = pixels_per_point.max(0.25);
  let width_px = (image_width_pt * pixels_per_point).ceil().max(1.0) as u32;
  let height_px = (image_height_pt * pixels_per_point).ceil().max(1.0) as u32;
  let mut alpha = vec![0_u8; width_px as usize * height_px as usize];

  let shape_rasterized = raster_shape_alpha(
    &mut alpha,
    width_px as usize,
    height_px as usize,
    shape_path,
    has_fill,
    stroke_style,
    pixels_per_point,
    |point| {
      (
        (point.x.0 - image_x_pt) * pixels_per_point,
        (point.y.0 - image_y_pt) * pixels_per_point,
      )
    },
  );
  if !shape_rasterized && has_fill {
    let left_px = ((frame.x_pt - stroke_outset_pt - image_x_pt) * pixels_per_point)
      .floor()
      .max(0.0) as usize;
    let top_px = ((frame.y_pt - stroke_outset_pt - image_y_pt) * pixels_per_point)
      .floor()
      .max(0.0) as usize;
    let right_px = ((frame.x_pt + frame.width_pt + stroke_outset_pt - image_x_pt)
      * pixels_per_point)
      .ceil()
      .clamp(0.0, width_px as f32) as usize;
    let bottom_px = ((frame.y_pt + frame.height_pt + stroke_outset_pt - image_y_pt)
      * pixels_per_point)
      .ceil()
      .clamp(0.0, height_px as f32) as usize;
    for y in top_px..bottom_px {
      let row = y * width_px as usize;
      alpha[row + left_px..row + right_px].fill(u8::MAX);
    }
  } else if !shape_rasterized {
    return None;
  }

  let blur_radius_px = (blur_radius_pt * pixels_per_point).ceil() as usize;
  if blur_radius_px > 0 {
    stack_blur_alpha(
      &mut alpha,
      width_px as usize,
      height_px as usize,
      blur_radius_px,
    );
  }

  let mut rgba = Vec::with_capacity(alpha.len() * 4);
  for mask in alpha {
    rgba.extend_from_slice(&[
      color.r,
      color.g,
      color.b,
      (f32::from(mask) * opacity.clamp(0.0, 1.0)).round() as u8,
    ]);
  }
  let mut png = Cursor::new(Vec::new());
  PngEncoder::new(&mut png)
    .write_image(&rgba, width_px, height_px, ColorType::Rgba8.into())
    .ok()?;

  Some(ImageItem {
    x_pt: image_x_pt,
    y_pt: image_y_pt,
    width_pt: image_width_pt,
    height_pt: image_height_pt,
    crop: ImageCrop::default(),
    clip_path: Vec::new(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    data: Arc::from(png.into_inner()),
    content_type: Some("image/png".to_string()),
    alt_text: None,
    hyperlink_url: None,
    floating: false,
    behind_text: true,
  })
}

fn fill_path_alpha(
  alpha: &mut [u8],
  width: usize,
  height: usize,
  commands: &[PathCommand],
  map: impl Fn(Point) -> (f32, f32),
) -> bool {
  fill_elements_alpha(
    alpha,
    width,
    height,
    mapped_path_elements(commands, |point| {
      let (x, y) = map(point);
      kurbo::Point::new(f64::from(x), f64::from(y))
    }),
  )
}

fn fill_elements_alpha(
  alpha: &mut [u8],
  width: usize,
  height: usize,
  elements: impl IntoIterator<Item = PathEl>,
) -> bool {
  let polygons = flattened_element_subpaths(elements);
  if polygons.is_empty() {
    return false;
  }
  let mut intersections = Vec::new();
  for y in 0..height {
    intersections.clear();
    let scan_y = y as f32 + 0.5;
    for polygon in &polygons {
      for edge in polygon.windows(2) {
        let (x1, y1) = edge[0];
        let (x2, y2) = edge[1];
        if (y1 <= scan_y && scan_y < y2) || (y2 <= scan_y && scan_y < y1) {
          let x = x1 + (scan_y - y1) * (x2 - x1) / (y2 - y1);
          intersections.push((x, if y2 > y1 { 1_i32 } else { -1_i32 }));
        }
      }
    }
    intersections.sort_by(|left, right| left.0.total_cmp(&right.0));
    let mut winding = 0_i32;
    let mut span_start = None;
    let mut index = 0;
    while index < intersections.len() {
      let x = intersections[index].0;
      let was_inside = winding != 0;
      while index < intersections.len() && (intersections[index].0 - x).abs() <= f32::EPSILON {
        winding += intersections[index].1;
        index += 1;
      }
      let is_inside = winding != 0;
      if !was_inside && is_inside {
        span_start = Some(x);
      } else if was_inside
        && !is_inside
        && let Some(start) = span_start.take()
      {
        fill_scanline_span(alpha, width, y, start, x);
      }
    }
  }
  true
}

fn flattened_element_subpaths(elements: impl IntoIterator<Item = PathEl>) -> Vec<Vec<(f32, f32)>> {
  let mut polygons = Vec::new();
  let mut polygon = Vec::new();
  flatten(
    elements,
    SHADOW_CURVE_FLATTEN_TOLERANCE_PX,
    |element| match element {
      PathEl::MoveTo(point) => {
        finish_subpath(&mut polygons, &mut polygon);
        polygon.push((point.x as f32, point.y as f32));
      }
      PathEl::LineTo(point) => {
        polygon.push((point.x as f32, point.y as f32));
      }
      PathEl::ClosePath => {
        finish_subpath(&mut polygons, &mut polygon);
      }
      PathEl::QuadTo(_, _) | PathEl::CurveTo(_, _, _) => {
        unreachable!("kurbo::flatten only emits line path elements")
      }
    },
  );
  finish_subpath(&mut polygons, &mut polygon);
  polygons
}

fn finish_subpath(polygons: &mut Vec<Vec<(f32, f32)>>, polygon: &mut Vec<(f32, f32)>) {
  if polygon.len() >= 3 {
    if polygon.first() != polygon.last() {
      polygon.push(polygon[0]);
    }
    polygons.push(std::mem::take(polygon));
  } else {
    polygon.clear();
  }
}

fn fill_scanline_span(alpha: &mut [u8], width: usize, y: usize, start: f32, end: f32) {
  let left = start.min(end);
  let right = start.max(end);
  let start_index = (left - 0.5).ceil().clamp(0.0, width as f32) as usize;
  let end_index = (right - 0.5).ceil().clamp(0.0, width as f32) as usize;
  if start_index < end_index {
    alpha[y * width + start_index..y * width + end_index].fill(255);
  }
}

pub(crate) fn shadow_alignment(alignment: Option<a::RectangleAlignmentValues>) -> (f32, f32) {
  match alignment.unwrap_or(a::RectangleAlignmentValues::Bottom) {
    a::RectangleAlignmentValues::TopLeft => (0.0, 0.0),
    a::RectangleAlignmentValues::Top => (0.5, 0.0),
    a::RectangleAlignmentValues::TopRight => (1.0, 0.0),
    a::RectangleAlignmentValues::Left => (0.0, 0.5),
    a::RectangleAlignmentValues::Center => (0.5, 0.5),
    a::RectangleAlignmentValues::Right => (1.0, 0.5),
    a::RectangleAlignmentValues::BottomLeft => (0.0, 1.0),
    a::RectangleAlignmentValues::Bottom => (0.5, 1.0),
    a::RectangleAlignmentValues::BottomRight => (1.0, 1.0),
  }
}

fn stack_blur_alpha(alpha: &mut [u8], width: usize, height: usize, radius: usize) {
  let radius = radius.min(MAX_STACK_BLUR_RADIUS_PX as usize);
  if radius == 0 || width == 0 || height == 0 {
    return;
  }
  let mut horizontal = vec![0_u8; alpha.len()];
  for y in 0..height {
    triangular_blur_line(
      &alpha[y * width..(y + 1) * width],
      &mut horizontal[y * width..(y + 1) * width],
      radius,
    );
  }
  let mut column = vec![0_u8; height];
  let mut blurred_column = vec![0_u8; height];
  for x in 0..width {
    for y in 0..height {
      column[y] = horizontal[y * width + x];
    }
    triangular_blur_line(&column, &mut blurred_column, radius);
    for y in 0..height {
      alpha[y * width + x] = blurred_column[y];
    }
  }
}

/// Applies the triangular kernel used by stack blur in linear time.
///
/// The expanded shadow bitmap has transparent edge pixels, so samples beyond
/// the bitmap are transparent too. The recurrence moves the weighted window
/// by subtracting the left half and adding the right half.
fn triangular_blur_line(input: &[u8], output: &mut [u8], radius: usize) {
  debug_assert_eq!(input.len(), output.len());
  if input.is_empty() {
    return;
  }
  let divisor = ((radius + 1) * (radius + 1)) as i64;
  let mut prefix = Vec::with_capacity(input.len() + 1);
  prefix.push(0_i64);
  for value in input {
    prefix.push(prefix.last().copied().unwrap_or_default() + i64::from(*value));
  }
  let range_sum = |start: usize, end: usize| prefix[end] - prefix[start];

  let mut weighted_sum = 0_i64;
  for (index, value) in input.iter().take(radius + 1).enumerate() {
    weighted_sum += i64::from(*value) * (radius + 1 - index) as i64;
  }
  for (center, output_value) in output.iter_mut().enumerate() {
    *output_value = (weighted_sum / divisor).clamp(0, 255) as u8;
    if center + 1 == input.len() {
      break;
    }
    let left_start = center.saturating_sub(radius);
    let left_end = center + 1;
    let right_start = center + 1;
    let right_end = (center + radius + 2).min(input.len());
    weighted_sum -= range_sum(left_start, left_end);
    weighted_sum += range_sum(right_start, right_end);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn shadow_alignment_uses_the_declared_scale_origin() {
    assert_eq!(
      shadow_alignment(Some(a::RectangleAlignmentValues::TopLeft)),
      (0.0, 0.0)
    );
    assert_eq!(
      shadow_alignment(Some(a::RectangleAlignmentValues::Center)),
      (0.5, 0.5)
    );
    assert_eq!(
      shadow_alignment(Some(a::RectangleAlignmentValues::BottomRight)),
      (1.0, 1.0)
    );
  }

  #[test]
  fn shadow_alpha_uses_the_shape_path_instead_of_its_bounds() {
    let commands = [
      PathCommand::MoveTo(Point {
        x: crate::common::Pt(1.0),
        y: crate::common::Pt(1.0),
      }),
      PathCommand::LineTo(Point {
        x: crate::common::Pt(9.0),
        y: crate::common::Pt(1.0),
      }),
      PathCommand::LineTo(Point {
        x: crate::common::Pt(1.0),
        y: crate::common::Pt(9.0),
      }),
      PathCommand::Close,
    ];
    let mut alpha = vec![0; 10 * 10];
    assert!(fill_path_alpha(&mut alpha, 10, 10, &commands, |point| (
      point.x.0, point.y.0
    ),));
    assert_eq!(alpha[2 * 10 + 2], 255);
    assert_eq!(alpha[8 * 10 + 8], 0);
  }

  #[test]
  fn stack_blur_preserves_symmetry_and_spreads_alpha() {
    let mut alpha = vec![0; 7 * 7];
    alpha[3 * 7 + 3] = 255;
    stack_blur_alpha(&mut alpha, 7, 7, 2);
    assert!(alpha[3 * 7 + 3] > alpha[3 * 7 + 2]);
    assert_eq!(alpha[3 * 7 + 2], alpha[3 * 7 + 4]);
    assert_eq!(alpha[2 * 7 + 3], alpha[4 * 7 + 3]);
    assert!(alpha[3 * 7 + 2] > 0);
  }

  #[test]
  fn linear_stack_blur_matches_the_triangular_kernel() {
    let input = [0, 0, 255, 0, 0];
    let mut output = [0; 5];
    triangular_blur_line(&input, &mut output, 2);
    assert_eq!(output, [28, 56, 85, 56, 28]);
  }

  #[test]
  fn soft_edge_erosion_treats_pixels_outside_the_shape_as_transparent() {
    let alpha = vec![255; 5 * 5];
    let eroded = erode_square_alpha(&alpha, 5, 5, 1);
    for y in 0..5 {
      for x in 0..5 {
        let expected = if (1..4).contains(&x) && (1..4).contains(&y) {
          255
        } else {
          0
        };
        assert_eq!(eroded[y * 5 + x], expected);
      }
    }
  }

  #[test]
  fn reflection_alpha_ramp_follows_authored_positions_in_either_order() {
    assert_eq!(reflection_opacity(0.0, (1.0, 0.0), (0.0, 1.0)), 1.0);
    assert!((reflection_opacity(0.5, (1.0, 0.0), (0.0, 1.0)) - 0.5).abs() < f32::EPSILON);
    assert_eq!(reflection_opacity(1.0, (1.0, 0.0), (0.0, 1.0)), 0.0);
  }

  #[test]
  fn office_inner_shadow_direction_starts_at_the_left() {
    let (x, y) = office_inner_shadow_offset(0, 10.0);
    assert!((x + 10.0).abs() <= f32::EPSILON);
    assert!(y.abs() <= f32::EPSILON);

    let (x, y) = office_inner_shadow_offset(5_400_000, 10.0);
    assert!(x.abs() < 0.001);
    assert!((y + 10.0).abs() < 0.001);
  }
}
