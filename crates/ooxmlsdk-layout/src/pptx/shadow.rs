use std::io::Cursor;
use std::sync::Arc;

use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use kurbo::{PathEl, flatten};
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::common::{PathCommand, Point};
use crate::model::{ImageCrop, ImageItem, RgbColor};
use crate::units;

use super::drawingml::shape_properties::{EffectShadowKind, EffectShadowProperties};
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
  color: RgbColor,
  opacity: f32,
) -> Option<ImageItem> {
  if effect.kind != EffectShadowKind::Outer
    || frame.width_pt <= 0.0
    || frame.height_pt <= 0.0
    || opacity <= 0.0
  {
    return None;
  }

  let scale_x = effect.scale_x.unwrap_or(100_000) as f32 / 100_000.0;
  let scale_y = effect.scale_y.unwrap_or(100_000) as f32 / 100_000.0;
  let shadow_width_pt = frame.width_pt * scale_x.abs();
  let shadow_height_pt = frame.height_pt * scale_y.abs();
  let (anchor_x, anchor_y) = shadow_alignment(effect.alignment);
  let distance_pt = units::emu_to_points(effect.distance_emu.unwrap_or_default());
  let direction_rad = effect.direction.unwrap_or_default() as f32 / 60_000.0 * 1.0_f32.to_radians();
  let offset_x_pt = direction_rad.cos() * distance_pt;
  let offset_y_pt = direction_rad.sin() * distance_pt;
  let stroke_outset_pt = frame.stroke_width_pt.max(0.0) / 2.0;
  let shadow_x_pt =
    frame.x_pt + (frame.width_pt - shadow_width_pt) * anchor_x + offset_x_pt - stroke_outset_pt;
  let shadow_y_pt =
    frame.y_pt + (frame.height_pt - shadow_height_pt) * anchor_y + offset_y_pt - stroke_outset_pt;
  let shadow_width_pt = shadow_width_pt + stroke_outset_pt * 2.0;
  let shadow_height_pt = shadow_height_pt + stroke_outset_pt * 2.0;
  let blur_pt = units::emu_to_points(effect.blur_radius_emu.unwrap_or_default()).max(0.0);
  let image_x_pt = shadow_x_pt - blur_pt;
  let image_y_pt = shadow_y_pt - blur_pt;
  let image_width_pt = shadow_width_pt + blur_pt * 2.0;
  let image_height_pt = shadow_height_pt + blur_pt * 2.0;
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

  let path_filled = !shape_path.is_empty()
    && fill_path_alpha(
      &mut alpha,
      width_px as usize,
      height_px as usize,
      shape_path,
      |point| {
        let x = frame.x_pt
          + (point.x.0 - frame.x_pt) * scale_x
          + (frame.width_pt - shadow_width_pt) * anchor_x
          + offset_x_pt;
        let y = frame.y_pt
          + (point.y.0 - frame.y_pt) * scale_y
          + (frame.height_pt - shadow_height_pt) * anchor_y
          + offset_y_pt;
        (
          (x - image_x_pt) * pixels_per_point,
          (y - image_y_pt) * pixels_per_point,
        )
      },
    );
  if !path_filled {
    let left_px = ((shadow_x_pt - image_x_pt) * pixels_per_point)
      .floor()
      .max(0.0) as u32;
    let top_px = ((shadow_y_pt - image_y_pt) * pixels_per_point)
      .floor()
      .max(0.0) as u32;
    let right_px = ((shadow_x_pt + shadow_width_pt - image_x_pt) * pixels_per_point)
      .ceil()
      .clamp(0.0, width_px as f32) as u32;
    let bottom_px = ((shadow_y_pt + shadow_height_pt - image_y_pt) * pixels_per_point)
      .ceil()
      .clamp(0.0, height_px as f32) as u32;
    for y in top_px..bottom_px {
      let row = y as usize * width_px as usize;
      alpha[row + left_px as usize..row + right_px as usize].fill(255);
    }
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

fn fill_path_alpha(
  alpha: &mut [u8],
  width: usize,
  height: usize,
  commands: &[PathCommand],
  map: impl Fn(Point) -> (f32, f32),
) -> bool {
  let polygons = flattened_subpaths(commands, map);
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

fn flattened_subpaths(
  commands: &[PathCommand],
  map: impl Fn(Point) -> (f32, f32),
) -> Vec<Vec<(f32, f32)>> {
  let elements = mapped_path_elements(commands, |point| {
    let (x, y) = map(point);
    kurbo::Point::new(f64::from(x), f64::from(y))
  });
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

fn shadow_alignment(alignment: Option<a::RectangleAlignmentValues>) -> (f32, f32) {
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
}
