//! Legacy wireframes retain the solid's indexed edges, not the boundary of
//! its antialiased fill bitmap. Word's observed meshes submit front/back
//! perimeter segments and longitudinal edges as a Direct3D line list.

use image::{Rgba, RgbaImage};

use super::{
  DIRECT3D_STANDARD_8_SAMPLES, Static3dColor, Static3dProjection, Static3dSurface,
  Static3dTextGeometry, blend_over, map_homogeneous, plane_homography, text_surface_edge,
  text_surface_triangle_covers_sample,
};

pub(super) struct Options {
  pub projection: Static3dProjection,
  pub model_surface: Static3dSurface,
  pub pixels_per_point: f32,
  pub front_z_px: f32,
  pub back_z_px: f32,
  pub color: Static3dColor,
}

pub(super) fn composite(
  destination: &mut RgbaImage,
  geometry: &Static3dTextGeometry,
  options: Options,
) {
  let (width, height) = destination.dimensions();
  if width == 0 || height == 0 || options.color.alpha == 0 {
    return;
  }
  // One byte holds all eight sample owners. Resolve once after the complete
  // line list so coincident front/back edges and shared endpoints do not
  // repeatedly blend fractional pixel alpha. No depth storage is needed for
  // this constant-color, opaque wire material.
  let mut coverage = vec![0_u8; width as usize * height as usize];
  let surface = options.model_surface;
  let center = (
    surface.left_px + surface.width_px * 0.5,
    surface.top_px + surface.height_px * 0.5,
  );
  let matrices = [options.front_z_px, options.back_z_px].map(|z| {
    plane_homography(
      options.projection,
      z,
      surface.width_px.max(1.0),
      surface.height_px.max(1.0),
      options.pixels_per_point,
    )
  });
  let project = |point: (f32, f32)| {
    matrices.map(|matrix| {
      let point = map_homogeneous(matrix, point.0 - center.0, point.1 - center.1);
      (point.0 + center.0, point.1 + center.1)
    })
  };
  for contour in &geometry.contours {
    let Some(&first) = contour.points.first() else {
      continue;
    };
    let mut previous = project(first);
    for point in contour.points.iter().skip(1).chain(std::iter::once(&first)) {
      let current = project(*point);
      for (start, end) in [
        (previous[0], current[0]),
        (current[0], current[1]),
        (current[1], previous[1]),
      ] {
        line_coverage(&mut coverage, width, height, start, end);
      }
      previous = current;
    }
  }
  for (pixel, mask) in destination.pixels_mut().zip(coverage) {
    if mask == 0 {
      continue;
    }
    let color = options.color;
    let alpha = ((u32::from(color.alpha) * mask.count_ones() + 4) / 8) as u8;
    blend_over(
      pixel,
      Rgba([color.color.r, color.color.g, color.color.b, alpha]),
    );
  }
}

fn line_coverage(coverage: &mut [u8], width: u32, height: u32, start: (f32, f32), end: (f32, f32)) {
  let dx = end.0 - start.0;
  let dy = end.1 - start.1;
  let length = dx.hypot(dy);
  if !length.is_finite() || length <= f32::EPSILON {
    return;
  }
  // D3D11.3 §3.4.5: MultisampleEnable=true, AntialiasedLineEnable=false
  // expands a line to a 1.4-device-pixel rectangle, with butt endpoints.
  // Those exact state flags were observed on Word's line-list submissions.
  // Width belongs to the output grid, independently of camera magnification.
  let normal = (-dy * (0.7 / length), dx * (0.7 / length));
  let corners = [
    (start.0 + normal.0, start.1 + normal.1),
    (start.0 - normal.0, start.1 - normal.1),
    (end.0 - normal.0, end.1 - normal.1),
    (end.0 + normal.0, end.1 + normal.1),
  ]
  // D3D11.3 §3.4.1 snaps expanded primitives to eight fractional bits.
  .map(|(x, y)| {
    (
      (x * 256.0).round_ties_even() / 256.0,
      (y * 256.0).round_ties_even() / 256.0,
    )
  });
  for [a, b, c] in [
    [corners[0], corners[1], corners[2]],
    [corners[0], corners[2], corners[3]],
  ] {
    let area = text_surface_edge(a, b, c);
    if area.abs() <= f32::EPSILON {
      continue;
    }
    let left = a.0.min(b.0).min(c.0).floor().clamp(0.0, width as f32) as u32;
    let top = a.1.min(b.1).min(c.1).floor().clamp(0.0, height as f32) as u32;
    let right = a.0.max(b.0).max(c.0).ceil().clamp(0.0, width as f32) as u32;
    let bottom = a.1.max(b.1).max(c.1).ceil().clamp(0.0, height as f32) as u32;
    for y in top..bottom {
      for x in left..right {
        let mask = &mut coverage[y as usize * width as usize + x as usize];
        for (sample, &(sx, sy)) in DIRECT3D_STANDARD_8_SAMPLES.iter().enumerate() {
          if *mask & (1 << sample) == 0
            && text_surface_triangle_covers_sample(a, b, c, (x as f32 + sx, y as f32 + sy), area)
          {
            *mask |= 1 << sample;
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::line_coverage;

  #[test]
  fn coincident_and_reversed_lines_resolve_each_sample_once() {
    for (start, end) in [((2.3, 3.8), (26.7, 21.4)), ((3.5, 2.1), (3.5, 27.2))] {
      let mut coverage = vec![0; 32 * 32];
      line_coverage(&mut coverage, 32, 32, start, end);
      let once = coverage.clone();
      assert!(once.iter().any(|&mask| mask != 0 && mask != 255));
      line_coverage(&mut coverage, 32, 32, end, start);
      line_coverage(&mut coverage, 32, 32, start, end);
      assert_eq!(coverage, once);
    }
  }

  #[test]
  fn line_coverage_uses_device_width_and_butt_endpoints() {
    for extent in [8.0, 16.0, 28.0] {
      let mut coverage = vec![0; 32 * 8];
      line_coverage(&mut coverage, 32, 8, (2.0, 3.5), (extent, 3.5));
      for y in 0..8 {
        for x in 0..32 {
          let count = coverage[y * 32 + x].count_ones();
          let expected = if x >= 2 && (x as f32) < extent {
            match y {
              2 | 4 => 2,
              3 => 8,
              _ => 0,
            }
          } else {
            0
          };
          assert_eq!(count, expected, "extent={extent}, pixel=({x},{y})");
        }
      }
    }
  }

  #[test]
  fn collapsed_and_offscreen_lines_do_not_change_coverage() {
    let mut coverage = vec![0; 16 * 16];
    for (start, end) in [
      ((4.5, 4.5), (4.5, 4.5)),
      ((-4.0, -5.0), (-1.0, -2.0)),
      ((20.0, 30.0), (24.0, 25.0)),
    ] {
      line_coverage(&mut coverage, 16, 16, start, end);
    }
    assert!(coverage.iter().all(|&mask| mask == 0));
  }
}
