use std::sync::Arc;

use kurbo::{PathEl, Point as KurboPoint, flatten};
use ooxmlsdk_layout::common;

use super::direct_gradient::ColorStop;
use super::image::{
  DirectRasterColorSpace, DirectRasterEncoding, DirectRasterImage, PdfRasterPixels,
  PreparedRasterImage, office_fixed_output_raster_dimensions,
};
use crate::error::{PdfError, Result};

const MAX_PATH_GRADIENT_RASTER_PIXELS: f32 = 250_000.0;
const MIN_PATH_GRADIENT_PIXELS_PER_POINT: f32 = 0.25;
const PATH_GRADIENT_BINARY_STEPS: usize = 10;
const GEOMETRY_TOLERANCE: f32 = 1.0e-5;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct PathGradientProfile {
  pub(super) office_fixed_output: bool,
  pub(super) raster_sampling: PathGradientRasterSampling,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum PathGradientRasterSampling {
  OfficeFixedOutputDpi(u32),
  BoundedPixelsPerPoint(f32),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum PathGradientResolution {
  Radial([f32; 6]),
  Raster,
}

pub(super) fn resolve(
  path: common::GradientPath,
  profile: PathGradientProfile,
) -> Result<PathGradientResolution> {
  validate_path(path)?;
  if path.kind != common::GradientPathKind::Circle || path.mirror_tile {
    return Ok(PathGradientResolution::Raster);
  }

  let transform = path.transform;
  let x_scale = transform.m11.hypot(transform.m12);
  let y_scale = transform.m21.hypot(transform.m22);
  let scale = x_scale.max(y_scale).max(1.0);
  let dot = transform.m11 * transform.m21 + transform.m12 * transform.m22;
  if (x_scale - y_scale).abs() > GEOMETRY_TOLERANCE * scale
    || dot.abs() > GEOMETRY_TOLERANCE * x_scale.max(1.0) * y_scale.max(1.0)
  {
    return Ok(PathGradientResolution::Raster);
  }

  let (focus_x, focus_y, focus_radius) = if profile.office_fixed_output {
    // PowerPoint 16.0.20326 emits the same centred, zero-radius Type 3
    // shading for an 11-case fillToRect position/size matrix. Independent
    // Word, Excel, print, and screen goldens use the same fixed-output shape.
    // Keep the DrawingML focus rectangle for the requested/generic profile.
    (0.5, 0.5, 0.0)
  } else {
    let focus = normalized_focus(path.fill_to);
    let width = 1.0 - focus.left - focus.right;
    let height = 1.0 - focus.top - focus.bottom;
    if (width - height).abs() > GEOMETRY_TOLERANCE {
      return Ok(PathGradientResolution::Raster);
    }
    (
      (focus.left + 1.0 - focus.right) * 0.5,
      (focus.top + 1.0 - focus.bottom) * 0.5,
      width * 0.5,
    )
  };

  let focus = transform_point(transform, focus_x, focus_y);
  let outer = transform_point(transform, 0.5, 0.5);
  let unit_scale = (x_scale + y_scale) * 0.5;
  let focus_radius = focus_radius * unit_scale;
  let outer_radius = 0.5 * unit_scale;
  let coords = [
    focus.x,
    focus.y,
    focus_radius,
    outer.x,
    outer.y,
    outer_radius,
  ];
  if !coords.into_iter().all(f32::is_finite) || focus_radius < 0.0 || outer_radius <= f32::EPSILON {
    return Err(PdfError::Writer(
      "path gradient produced invalid radial coordinates".to_string(),
    ));
  }

  // A Type 3 shading does not cover every ray when the focal circle extends
  // outside the outer circle. The bounded sampler remains exact for that
  // legal DrawingML geometry.
  if !profile.office_fixed_output
    && (focus.x - outer.x).hypot(focus.y - outer.y) + focus_radius
      >= outer_radius - GEOMETRY_TOLERANCE * outer_radius.max(1.0)
  {
    return Ok(PathGradientResolution::Raster);
  }
  Ok(PathGradientResolution::Radial(coords))
}

pub(super) fn rasterize(
  path: common::GradientPath,
  stops: &[ColorStop],
  bounds: common::Rect,
  commands: Option<&[common::PathCommand]>,
  profile: PathGradientProfile,
) -> Result<PreparedRasterImage> {
  let (width, height) = validate_raster(path, stops, bounds, profile)?;
  let pixel_count = usize::try_from(width)
    .ok()
    .and_then(|width| {
      usize::try_from(height)
        .ok()
        .and_then(|height| width.checked_mul(height))
    })
    .ok_or_else(|| PdfError::Writer("path-gradient raster size overflows usize".to_string()))?;

  let shape = if path.kind == common::GradientPathKind::Shape {
    Some(shape_polygons(commands, path.transform)?)
  } else {
    None
  };
  let mut rgb =
    Vec::with_capacity(pixel_count.checked_mul(3).ok_or_else(|| {
      PdfError::Writer("path-gradient RGB plane size overflows usize".to_string())
    })?);
  let has_alpha = stops.iter().any(|stop| stop.alpha != u8::MAX);
  let mut alpha = has_alpha.then(|| Vec::with_capacity(pixel_count));
  let width_pt = bounds.size.width.0;
  let height_pt = bounds.size.height.0;
  let pixel_width = f64::from(width_pt) / f64::from(width);
  let pixel_height = f64::from(height_pt) / f64::from(height);
  for y in 0..height {
    let page_y = f64::from(bounds.origin.y.0) + (f64::from(y) + 0.5) * pixel_height;
    for x in 0..width {
      let page_x = f64::from(bounds.origin.x.0) + (f64::from(x) + 0.5) * pixel_width;
      let point = inverse_point(path.transform, page_x, page_y)?;
      let position = position(path, point, shape.as_deref())?;
      let color = sample(stops, position);
      rgb.extend_from_slice(&color.rgb);
      if let Some(alpha) = &mut alpha {
        alpha.push(color.alpha);
      }
    }
  }

  Ok(PreparedRasterImage::new(DirectRasterImage {
    width,
    height,
    color_space: DirectRasterColorSpace::Rgb,
    bits_per_component: 8,
    encoding: DirectRasterEncoding::Sampled {
      pixels: Arc::new(PdfRasterPixels {
        width,
        height,
        rgb,
        alpha,
        icc_profile: None,
      }),
    },
    interpolate: false,
    soft_mask_interpolate: false,
    matte: None,
  }))
}

pub(super) fn validate_raster(
  path: common::GradientPath,
  stops: &[ColorStop],
  bounds: common::Rect,
  profile: PathGradientProfile,
) -> Result<(u32, u32)> {
  validate_path(path)?;
  validate_bounds(bounds)?;
  if stops.is_empty() {
    return Err(PdfError::Writer(
      "path gradient must contain at least one color stop".to_string(),
    ));
  }
  raster_dimensions(bounds, profile)
}

fn raster_dimensions(bounds: common::Rect, profile: PathGradientProfile) -> Result<(u32, u32)> {
  let width_pt = bounds.size.width.0;
  let height_pt = bounds.size.height.0;
  match profile.raster_sampling {
    PathGradientRasterSampling::OfficeFixedOutputDpi(dpi) => {
      if dpi == 0 {
        return Err(PdfError::Writer(
          "path-gradient raster DPI must be positive".to_string(),
        ));
      }
      office_fixed_output_raster_dimensions(width_pt, height_pt, dpi).ok_or_else(|| {
        PdfError::Writer(
          "Office path-gradient raster size is outside the supported range".to_string(),
        )
      })
    }
    PathGradientRasterSampling::BoundedPixelsPerPoint(pixels_per_point) => {
      if !pixels_per_point.is_finite() || pixels_per_point <= 0.0 {
        return Err(PdfError::Writer(
          "path-gradient sampling rate must be finite and positive".to_string(),
        ));
      }
      let budget_pixels_per_point =
        (MAX_PATH_GRADIENT_RASTER_PIXELS / (width_pt * height_pt)).sqrt();
      let pixels_per_point = pixels_per_point
        .min(budget_pixels_per_point)
        .max(MIN_PATH_GRADIENT_PIXELS_PER_POINT);
      Ok((
        checked_dimension((width_pt * pixels_per_point).ceil(), "width")?,
        checked_dimension((height_pt * pixels_per_point).ceil(), "height")?,
      ))
    }
  }
}

fn checked_dimension(value: f32, axis: &str) -> Result<u32> {
  if !value.is_finite() || value < 1.0 || value > u32::MAX as f32 {
    return Err(PdfError::Writer(format!(
      "path-gradient raster {axis} is outside the supported range"
    )));
  }
  Ok(value as u32)
}

fn validate_bounds(bounds: common::Rect) -> Result<()> {
  let values = [
    bounds.origin.x.0,
    bounds.origin.y.0,
    bounds.size.width.0,
    bounds.size.height.0,
  ];
  if !values.into_iter().all(f32::is_finite)
    || bounds.size.width.0 <= f32::EPSILON
    || bounds.size.height.0 <= f32::EPSILON
  {
    return Err(PdfError::Writer(
      "path-gradient paint bounds must be finite and positive".to_string(),
    ));
  }
  Ok(())
}

fn validate_path(path: common::GradientPath) -> Result<()> {
  let focus = path.fill_to;
  let transform = path.transform;
  if ![
    focus.left,
    focus.top,
    focus.right,
    focus.bottom,
    transform.m11,
    transform.m12,
    transform.m21,
    transform.m22,
    transform.dx.0,
    transform.dy.0,
  ]
  .into_iter()
  .all(f32::is_finite)
  {
    return Err(PdfError::Writer(
      "path-gradient geometry contains a non-finite value".to_string(),
    ));
  }
  let determinant = transform.m11 * transform.m22 - transform.m12 * transform.m21;
  if !determinant.is_finite() || determinant.abs() <= f32::EPSILON {
    return Err(PdfError::Writer(
      "path-gradient transform must be invertible".to_string(),
    ));
  }
  Ok(())
}

fn transform_point(transform: common::Transform, x: f32, y: f32) -> RasterPoint {
  RasterPoint {
    x: transform
      .m11
      .mul_add(x, transform.m21.mul_add(y, transform.dx.0)),
    y: transform
      .m12
      .mul_add(x, transform.m22.mul_add(y, transform.dy.0)),
  }
}

fn inverse_point(transform: common::Transform, page_x: f64, page_y: f64) -> Result<KurboPoint> {
  let m11 = f64::from(transform.m11);
  let m12 = f64::from(transform.m12);
  let m21 = f64::from(transform.m21);
  let m22 = f64::from(transform.m22);
  let determinant = m11 * m22 - m12 * m21;
  if !determinant.is_finite() || determinant.abs() <= f64::from(f32::EPSILON) {
    return Err(PdfError::Writer(
      "path-gradient transform must be invertible".to_string(),
    ));
  }
  let x = page_x - f64::from(transform.dx.0);
  let y = page_y - f64::from(transform.dy.0);
  Ok(KurboPoint::new(
    (m22 * x - m21 * y) / determinant,
    (-m12 * x + m11 * y) / determinant,
  ))
}

fn position(
  path: common::GradientPath,
  mut point: KurboPoint,
  shape: Option<&[Vec<KurboPoint>]>,
) -> Result<f32> {
  if path.mirror_tile {
    point.x = mirrored_tile_coordinate(point.x);
    point.y = mirrored_tile_coordinate(point.y);
  }
  if !contains(path, point, 1.0, shape)? {
    return Ok(1.0);
  }
  if contains(path, point, 0.0, shape)? {
    return Ok(0.0);
  }
  let mut outside = 0.0;
  let mut inside = 1.0;
  for _ in 0..PATH_GRADIENT_BINARY_STEPS {
    let middle = (outside + inside) * 0.5;
    if contains(path, point, middle, shape)? {
      inside = middle;
    } else {
      outside = middle;
    }
  }
  Ok(inside as f32)
}

fn contains(
  path: common::GradientPath,
  point: KurboPoint,
  outer_ratio: f64,
  shape: Option<&[Vec<KurboPoint>]>,
) -> Result<bool> {
  let focus = normalized_focus(path.fill_to);
  let focus_width = 1.0 - f64::from(focus.left) - f64::from(focus.right);
  let focus_height = 1.0 - f64::from(focus.top) - f64::from(focus.bottom);
  let scale_x = focus_width + (1.0 - focus_width) * outer_ratio;
  let scale_y = focus_height + (1.0 - focus_height) * outer_ratio;
  let offset_x = f64::from(focus.left) * (1.0 - outer_ratio);
  let offset_y = f64::from(focus.top) * (1.0 - outer_ratio);
  if scale_x.abs() <= f64::EPSILON || scale_y.abs() <= f64::EPSILON {
    return Ok(
      (point.x - offset_x).abs() <= f64::EPSILON && (point.y - offset_y).abs() <= f64::EPSILON,
    );
  }
  let base = KurboPoint::new(
    (point.x - offset_x) / scale_x,
    (point.y - offset_y) / scale_y,
  );
  Ok(match path.kind {
    common::GradientPathKind::Circle => {
      let x = (base.x - 0.5) * 2.0;
      let y = (base.y - 0.5) * 2.0;
      x.mul_add(x, y * y) <= 1.0
    }
    common::GradientPathKind::Rectangle => {
      (0.0..=1.0).contains(&base.x) && (0.0..=1.0).contains(&base.y)
    }
    common::GradientPathKind::Shape => point_in_polygons(
      base,
      shape
        .ok_or_else(|| PdfError::Writer("shape path-gradient geometry is missing".to_string()))?,
    ),
  })
}

fn normalized_focus(rect: common::RelativeRect) -> common::RelativeRect {
  let right = (1.0 - rect.right).max(rect.left);
  let bottom = (1.0 - rect.bottom).max(rect.top);
  common::RelativeRect {
    left: rect.left,
    top: rect.top,
    right: 1.0 - right,
    bottom: 1.0 - bottom,
  }
}

fn mirrored_tile_coordinate(value: f64) -> f64 {
  let tile = value.floor();
  let fraction = value - tile;
  if tile.rem_euclid(2.0) < 1.0 {
    fraction
  } else {
    1.0 - fraction
  }
}

fn shape_polygons(
  commands: Option<&[common::PathCommand]>,
  transform: common::Transform,
) -> Result<Vec<Vec<KurboPoint>>> {
  let Some(commands) = commands.filter(|commands| !commands.is_empty()) else {
    return Ok(vec![vec![
      KurboPoint::new(0.0, 0.0),
      KurboPoint::new(1.0, 0.0),
      KurboPoint::new(1.0, 1.0),
      KurboPoint::new(0.0, 1.0),
      KurboPoint::new(0.0, 0.0),
    ]]);
  };
  let mut elements = Vec::with_capacity(commands.len());
  for command in commands {
    match *command {
      common::PathCommand::MoveTo(point) => elements.push(PathEl::MoveTo(inverse_point(
        transform,
        f64::from(point.x.0),
        f64::from(point.y.0),
      )?)),
      common::PathCommand::LineTo(point) => elements.push(PathEl::LineTo(inverse_point(
        transform,
        f64::from(point.x.0),
        f64::from(point.y.0),
      )?)),
      common::PathCommand::CubicTo {
        control1,
        control2,
        end,
      } => elements.push(PathEl::CurveTo(
        inverse_point(transform, f64::from(control1.x.0), f64::from(control1.y.0))?,
        inverse_point(transform, f64::from(control2.x.0), f64::from(control2.y.0))?,
        inverse_point(transform, f64::from(end.x.0), f64::from(end.y.0))?,
      )),
      common::PathCommand::Close => elements.push(PathEl::ClosePath),
    }
  }
  let mut polygons = Vec::new();
  let mut polygon = Vec::new();
  flatten(elements, 0.0005, |element| match element {
    PathEl::MoveTo(point) => {
      finish_polygon(&mut polygons, &mut polygon);
      polygon.push(point);
    }
    PathEl::LineTo(point) => polygon.push(point),
    PathEl::ClosePath => finish_polygon(&mut polygons, &mut polygon),
    PathEl::QuadTo(_, _) | PathEl::CurveTo(_, _, _) => {
      unreachable!("kurbo::flatten emits only line elements")
    }
  });
  finish_polygon(&mut polygons, &mut polygon);
  if polygons.is_empty() {
    return Err(PdfError::Writer(
      "shape path-gradient has no closed paint geometry".to_string(),
    ));
  }
  Ok(polygons)
}

fn finish_polygon(polygons: &mut Vec<Vec<KurboPoint>>, polygon: &mut Vec<KurboPoint>) {
  if polygon.len() >= 3 {
    if polygon.first() != polygon.last() {
      polygon.push(polygon[0]);
    }
    polygons.push(std::mem::take(polygon));
  } else {
    polygon.clear();
  }
}

fn point_in_polygons(point: KurboPoint, polygons: &[Vec<KurboPoint>]) -> bool {
  let mut inside = false;
  for polygon in polygons {
    for edge in polygon.windows(2) {
      let (x1, y1) = (edge[0].x, edge[0].y);
      let (x2, y2) = (edge[1].x, edge[1].y);
      if (y1 > point.y) != (y2 > point.y) && point.x < (x2 - x1) * (point.y - y1) / (y2 - y1) + x1 {
        inside = !inside;
      }
    }
  }
  inside
}

fn sample(stops: &[ColorStop], position: f32) -> RasterColor {
  let position = position.clamp(0.0, 1.0);
  let pair = stops
    .windows(2)
    .find(|pair| {
      pair[0].position < pair[1].position
        && (position < pair[1].position || pair[1].position == 1.0)
    })
    .unwrap_or_else(|| &stops[stops.len() - 2..]);
  let span = pair[1].position - pair[0].position;
  let ratio = if span <= f32::EPSILON {
    1.0
  } else {
    ((position - pair[0].position) / span).clamp(0.0, 1.0)
  };
  let channel = |index: usize| {
    ((pair[1].rgb[index] - pair[0].rgb[index]).mul_add(ratio, pair[0].rgb[index]) * 255.0)
      .round()
      .clamp(0.0, 255.0) as u8
  };
  RasterColor {
    rgb: [channel(0), channel(1), channel(2)],
    alpha: (f32::from(pair[0].alpha)
      + (f32::from(pair[1].alpha) - f32::from(pair[0].alpha)) * ratio)
      .round()
      .clamp(0.0, 255.0) as u8,
  }
}

#[derive(Clone, Copy)]
struct RasterPoint {
  x: f32,
  y: f32,
}

#[derive(Clone, Copy)]
struct RasterColor {
  rgb: [u8; 3],
  alpha: u8,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hard_edge_sampling_uses_nonempty_intervals_at_boundaries() {
    for position in [0.0, 0.5, 1.0] {
      let red = ColorStop {
        position: 0.0,
        rgb: [1.0, 0.0, 0.0],
        alpha: 64,
      };
      let blue = ColorStop {
        position: 1.0,
        rgb: [0.0, 0.0, 1.0],
        alpha: 255,
      };
      let stops = [
        red.clone(),
        ColorStop { position, ..red },
        ColorStop { position, ..blue },
        blue,
      ];
      for probe in [0.0, 0.499, 0.5, 0.501, 1.0] {
        let sampled = sample(&stops, probe);
        let use_blue = position == 0.0 || (position == 0.5 && probe >= 0.5);
        assert_eq!(
          sampled.rgb,
          if use_blue { [0, 0, 255] } else { [255, 0, 0] }
        );
        assert_eq!(sampled.alpha, if use_blue { 255 } else { 64 });
      }
    }
  }
}
