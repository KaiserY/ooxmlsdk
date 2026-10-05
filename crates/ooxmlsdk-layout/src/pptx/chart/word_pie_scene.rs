//! Word's outlined pie is a depth-tested mesh, including its pens.
//!
//! Independent native wire controls at elevations 15/90 and seven rotations
//! identify a six-sided tube. Its initial axis is -Z, its hexagon has vertices
//! at 30 + 60n degrees, and a shortest-arc rotation aligns it to the edge.
//! A screen-space stroke loses both this angular footprint and cap occlusion.

use super::{
  Chart3DView, ImageItem, PlotRect, RadialChartStyle, RadialPerspectiveProjection, RadialSlice,
  RgbColor, chart_3d_scene_image, common_rect, office_perspective_chart_diffuse_color,
  word_pie_3d_pen_width, word_pie_cut_face_color,
};
use crate::common::drawingml_shape_raster::PageToRasterMapping;
use crate::common::{Fill, ShapeStyleValue};
use image::{Rgba, RgbaImage};

const HEX_APOTHEM: f64 = 0.866_025_403_784_438_6;
type Point3 = [f64; 3];

#[derive(Clone, Copy)]
struct Vertex {
  point: [f64; 2],
  depth: f64,
  color: [f64; 3],
}

#[derive(Default)]
pub(super) struct Scene {
  triangles: Vec<[Vertex; 3]>,
}

struct Projector {
  plane: RadialPerspectiveProjection,
  thickness: f64,
  sine: f64,
  cosine: f64,
  parallel: bool,
}

impl Projector {
  fn vertex(&self, point: Point3, color: [f64; 3]) -> Vertex {
    let p = self.plane;
    let k = f64::from(p.strength);
    let u = point[0] + f64::from(p.plane_offset.0);
    let v = point[1] + f64::from(p.plane_offset.1);
    let fraction = -point[2] / self.thickness;
    let extrusion = p.extrusion.expect("physical pie extrusion");
    let denominator = 1.0 - k * v + f64::from(extrusion.denominator) * fraction;
    Vertex {
      point: [
        f64::from(p.conic_center.0) + f64::from(p.radii.0) * (1.0 - k * k).sqrt() * u / denominator,
        f64::from(p.conic_center.1) - f64::from(p.radii.1) * k
          + (f64::from(p.radii.1) * (1.0 - k * k) * v + f64::from(extrusion.numerator) * fraction)
            / denominator,
      ],
      depth: if self.parallel {
        self.cosine * v + self.sine * point[2]
      } else {
        denominator.recip()
      },
      color,
    }
  }
}

fn rgb(color: RgbColor) -> [f64; 3] {
  [f64::from(color.r), f64::from(color.g), f64::from(color.b)]
}

fn diffuse(color: RgbColor, normal: [f32; 3]) -> [f64; 3] {
  // Recover the native seven-bit vertex attributes before interpolation,
  // matching the existing FixedGouraud7 scene gradient path.
  rgb(office_perspective_chart_diffuse_color(color, normal))
    .map(|c| (c * 128.0 / 255.0).round() * 255.0 / 128.0)
}

pub(super) fn lower(
  slices: &[RadialSlice],
  style: &RadialChartStyle,
  projection: RadialPerspectiveProjection,
  view: Chart3DView,
  plot: PlotRect,
) -> Option<ImageItem> {
  // Complex paints keep their existing realization until the mesh has the
  // corresponding material support. Unoutlined scenes retain that path too.
  let mut outlined = false;
  for slice in slices {
    if style
      .point_image_effects
      .get(slice.index)
      .is_some_and(Option::is_some)
    {
      return None;
    }
    if let Some(paint) = style.point_styles.get(slice.index) {
      match &paint.fill {
        ShapeStyleValue::Paint(Fill::Solid(c)) if c.a == 255 => {}
        ShapeStyleValue::Paint(Fill::None)
        | ShapeStyleValue::NoPaint
        | ShapeStyleValue::Unspecified => {}
        _ => return None,
      }
      if let ShapeStyleValue::Paint(stroke) = &paint.stroke {
        if stroke.color.a != 255
          || stroke.dash.is_some()
          || stroke.pattern.is_some()
          || stroke.gradient.is_some()
          || stroke.compound.is_some()
        {
          return None;
        }
        outlined = true;
      }
    }
  }
  if !outlined {
    return None;
  }
  let thickness = f64::from(0.24 * (view.height_percent / 100.0).clamp(0.05, 5.0));
  let (s, c) = view.rotate_x_deg.clamp(0.0, 90.0).to_radians().sin_cos();
  let mut scene = Scene::default();
  for slice in slices {
    let projector = Projector {
      plane: projection.with_plane_offset(slice.perspective_plane_offset),
      thickness,
      sine: f64::from(s),
      cosine: f64::from(c),
      parallel: view.right_angle_axes,
    };
    let paint = style.point_styles.get(slice.index);
    let fill = match paint.map(|p| &p.fill) {
      Some(ShapeStyleValue::NoPaint | ShapeStyleValue::Paint(Fill::None)) => None,
      Some(ShapeStyleValue::Paint(Fill::Solid(color))) => Some(RgbColor {
        r: color.r,
        g: color.g,
        b: color.b,
      }),
      _ => Some(slice.color),
    };
    let segments = ((slice.sweep.to_degrees().abs() / 2.0).ceil() as usize).max(2);
    let ring: Vec<_> = (0..=segments)
      .map(|i| {
        let angle = slice.start_angle + slice.sweep * i as f32 / segments as f32;
        (
          angle,
          [f64::from(angle.sin()), -f64::from(angle.cos()), 0.0],
        )
      })
      .collect();
    let bottom = |mut p: Point3| {
      p[2] = -thickness;
      p
    };
    let hub = [0.0; 3];
    if let Some(color) = fill {
      let cap = diffuse(color, [0.0, -c, -s]);
      let base = diffuse(color, [0.0, c, s]);
      for pair in ring.windows(2) {
        let [(a, first), (b, second)] = pair else {
          unreachable!()
        };
        scene.triangle(&projector, [hub, *first, *second], [cap; 3]);
        scene.triangle(
          &projector,
          [bottom(hub), bottom(*second), bottom(*first)],
          [base; 3],
        );
        let light = |angle: f32| diffuse(color, [angle.sin(), -s * angle.cos(), c * angle.cos()]);
        let (ca, cb) = (light(*a), light(*b));
        scene.triangle(&projector, [*first, bottom(*first), *second], [ca, ca, cb]);
        scene.triangle(
          &projector,
          [*second, bottom(*first), bottom(*second)],
          [cb, ca, cb],
        );
      }
      for &(angle, outer) in [ring[0], ring[segments]].iter() {
        let color = rgb(word_pie_cut_face_color(color, angle, view.rotate_x_deg));
        scene.triangle(&projector, [hub, outer, bottom(outer)], [color; 3]);
        scene.triangle(&projector, [hub, bottom(outer), bottom(hub)], [color; 3]);
      }
    }
    if let Some(ShapeStyleValue::Paint(stroke)) = paint.map(|p| &p.stroke) {
      let radius =
        f64::from(word_pie_3d_pen_width(stroke.width.0, projection) / projection.radii.0)
          / (2.0 * HEX_APOTHEM);
      let color = rgb(RgbColor {
        r: stroke.color.r,
        g: stroke.color.g,
        b: stroke.color.b,
      });
      for pair in ring.windows(2) {
        scene.tube(&projector, pair[0].1, pair[1].1, radius, color);
        scene.tube(
          &projector,
          bottom(pair[0].1),
          bottom(pair[1].1),
          radius,
          color,
        );
      }
      for outer in [ring[0].1, ring[segments].1] {
        // Native rotation controls distinguish outward radial edges from
        // reversing one of them to follow the cap polygon's winding.
        scene.tube(&projector, hub, outer, radius, color);
        scene.tube(&projector, bottom(hub), bottom(outer), radius, color);
        scene.tube(&projector, outer, bottom(outer), radius, color);
      }
      scene.tube(&projector, hub, bottom(hub), radius, color);
    }
  }
  chart_3d_scene_image(
    &[],
    Some(common_rect(plot.left, plot.top, plot.width, plot.height)),
    style.fixed_output_raster_dpi,
    Some(&scene),
  )
}

fn cross(a: Point3, b: Point3) -> Point3 {
  [
    a[1] * b[2] - a[2] * b[1],
    a[2] * b[0] - a[0] * b[2],
    a[0] * b[1] - a[1] * b[0],
  ]
}

fn tube_ring(direction: Point3, radius: f64) -> [Point3; 6] {
  // Rodrigues' shortest-arc rotation from (0,0,-1) to the segment.
  let axis = [direction[1], -direction[0], 0.0];
  std::array::from_fn(|i| {
    let angle = (30.0 + 60.0 * i as f64).to_radians();
    let p = [radius * angle.cos(), radius * angle.sin(), 0.0];
    if direction[2] >= 1.0 - 1e-12 {
      return [p[0], -p[1], -p[2]];
    }
    let first = cross(axis, p);
    let second = cross(axis, first);
    std::array::from_fn(|j| p[j] + first[j] + second[j] / (1.0 - direction[2]))
  })
}

impl Scene {
  fn triangle(&mut self, projector: &Projector, points: [Point3; 3], colors: [[f64; 3]; 3]) {
    self.triangles.push(std::array::from_fn(|i| {
      projector.vertex(points[i], colors[i])
    }));
  }

  fn tube(
    &mut self,
    projector: &Projector,
    first: Point3,
    last: Point3,
    radius: f64,
    color: [f64; 3],
  ) {
    let delta: Point3 = std::array::from_fn(|i| last[i] - first[i]);
    let length = delta.iter().map(|v| v * v).sum::<f64>().sqrt();
    if length <= 1e-12 {
      return;
    }
    let ring = tube_ring(delta.map(|v| v / length), radius);
    let start: [Point3; 6] = ring.map(|v| std::array::from_fn(|j| first[j] + v[j]));
    let end: [Point3; 6] = ring.map(|v| std::array::from_fn(|j| last[j] + v[j]));
    for i in 0..6 {
      let next = (i + 1) % 6;
      for face in [
        [start[i], end[i], start[next]],
        [start[next], end[i], end[next]],
        [first, start[next], start[i]],
        [last, end[i], end[next]],
      ] {
        self.triangle(projector, face, [color; 3]);
      }
    }
  }

  pub(super) fn rasterize(&self, mapping: PageToRasterMapping) -> Option<RgbaImage> {
    let (width, height) = (mapping.width_px as usize, mapping.height_px as usize);
    let count = width.checked_mul(height)?;
    if count == 0 || count > 16_000_000 {
      return None;
    }
    let triangles: Vec<_> = self
      .triangles
      .iter()
      .map(|triangle| {
        triangle.map(|mut v| {
          v.point = [
            v.point[0] * f64::from(mapping.scale_x) + f64::from(mapping.translate_x),
            v.point[1] * f64::from(mapping.scale_y) + f64::from(mapping.translate_y),
          ];
          v
        })
      })
      .collect();
    let mut depth = vec![f64::NEG_INFINITY; count];
    let mut colors = vec![[0_u8; 3]; count];
    let mut sums = vec![[0_u16; 4]; count];
    for (sx, sy) in crate::common::drawingml_3d::DIRECT3D_STANDARD_8_SAMPLES {
      depth.fill(f64::NEG_INFINITY);
      for triangle in &triangles {
        raster_triangle(
          triangle,
          width,
          height,
          [f64::from(sx), f64::from(sy)],
          &mut depth,
          &mut colors,
        );
      }
      for ((z, color), sum) in depth.iter().zip(&colors).zip(&mut sums) {
        if z.is_finite() {
          for i in 0..3 {
            sum[i] += u16::from(color[i]);
          }
          sum[3] += 1;
        }
      }
    }
    let mut image = RgbaImage::new(mapping.width_px, mapping.height_px);
    for (pixel, sum) in image.pixels_mut().zip(sums) {
      let n = sum[3];
      if let Some(red) = (sum[0] + n / 2).checked_div(n) {
        *pixel = Rgba([
          red as u8,
          ((sum[1] + n / 2) / n) as u8,
          ((sum[2] + n / 2) / n) as u8,
          ((255 * n + 4) / 8) as u8,
        ]);
      }
    }
    Some(image)
  }
}

fn edge(a: [f64; 2], b: [f64; 2], p: [f64; 2]) -> f64 {
  (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0])
}

fn raster_triangle(
  triangle: &[Vertex; 3],
  width: usize,
  height: usize,
  sample: [f64; 2],
  depth: &mut [f64],
  colors: &mut [[u8; 3]],
) {
  let mut t = *triangle;
  let mut area = edge(t[0].point, t[1].point, t[2].point);
  if area.abs() <= 1e-12 {
    return;
  }
  if area < 0.0 {
    t.swap(1, 2);
    area = -area;
  }
  let min = |axis: usize| {
    t.iter()
      .map(|v| v.point[axis])
      .fold(f64::INFINITY, f64::min)
      .floor()
      .max(0.0) as usize
  };
  let max = |axis: usize, limit: usize| {
    t.iter()
      .map(|v| v.point[axis])
      .fold(f64::NEG_INFINITY, f64::max)
      .ceil()
      .max(0.0)
      .min(limit as f64) as usize
  };
  let (left, right, top, bottom) = (min(0), max(0, width), min(1), max(1, height));
  let edges = [
    (t[1].point, t[2].point),
    (t[2].point, t[0].point),
    (t[0].point, t[1].point),
  ];
  let inclusive = edges.map(|(a, b)| b[1] < a[1] || (b[1] == a[1] && b[0] > a[0]));
  let flat = t[0].color == t[1].color && t[0].color == t[2].color;
  for y in top..bottom {
    for x in left..right {
      let point = [x as f64 + sample[0], y as f64 + sample[1]];
      let e = edges.map(|(a, b)| edge(a, b, point));
      if (0..3).any(|i| e[i] < 0.0 || (e[i] == 0.0 && !inclusive[i])) {
        continue;
      }
      let z = (0..3).map(|i| e[i] * t[i].depth).sum::<f64>() / area;
      let index = y * width + x;
      if z < depth[index] {
        continue;
      }
      depth[index] = z;
      // As in D3D MSAA and the vector scene, color is evaluated once at
      // pixel center; only visibility/coverage is evaluated at each sample.
      let color = if flat {
        t[0].color
      } else {
        let center = [x as f64 + 0.5, y as f64 + 0.5];
        let weights = edges.map(|(a, b)| edge(a, b, center) / area);
        std::array::from_fn(|channel| {
          let value = (0..3)
            .map(|i| weights[i] * t[i].color[channel])
            .sum::<f64>();
          let low = t
            .iter()
            .map(|v| v.color[channel])
            .fold(f64::INFINITY, f64::min);
          let high = t
            .iter()
            .map(|v| v.color[channel])
            .fold(f64::NEG_INFINITY, f64::max);
          value.clamp(low, high)
        })
      };
      colors[index] = color.map(|v| v.round().clamp(0.0, 255.0) as u8);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn intersecting_surfaces_use_sample_depth_independently_of_submission_order() {
    let plane = |red: bool| {
      let vertex = |x, y| Vertex {
        point: [x, y],
        depth: if red { x } else { 4.0 - x },
        color: if red {
          [255.0, 0.0, 0.0]
        } else {
          [0.0, 0.0, 255.0]
        },
      };
      let corners = [
        vertex(0.0, 0.0),
        vertex(4.0, 0.0),
        vertex(4.0, 2.0),
        vertex(0.0, 2.0),
      ];
      [
        [corners[0], corners[1], corners[2]],
        [corners[0], corners[2], corners[3]],
      ]
    };
    let mapping = PageToRasterMapping {
      width_px: 4,
      height_px: 2,
      scale_x: 1.0,
      scale_y: 1.0,
      translate_x: 0.0,
      translate_y: 0.0,
      text_hinting: None,
    };
    let mut scene = Scene {
      triangles: plane(true).into_iter().chain(plane(false)).collect(),
    };
    let forward = scene.rasterize(mapping).unwrap();
    scene.triangles.reverse();
    assert_eq!(forward, scene.rasterize(mapping).unwrap());
    for y in 0..2 {
      assert_eq!(forward.get_pixel(0, y).0, [0, 0, 255, 255]);
      assert_eq!(forward.get_pixel(3, y).0, [255, 0, 0, 255]);
    }
    assert!(forward.pixels().all(|p| p[3] == 255));
  }

  #[test]
  fn hexagonal_pen_matches_independent_native_rotations_and_elevations() {
    // Widths measured on isolated native wire quarter-pies at 200 DPI.
    for (elevation, radius_px, native) in [
      (
        15.0_f64,
        292.315,
        [4.640, 4.773, 5.416, 4.953, 5.188, 5.439, 5.245],
      ),
      (
        90.0,
        113.270,
        [1.877, 2.013, 2.100, 2.006, 1.826, 2.022, 1.978],
      ),
    ] {
      let (s, c) = elevation.to_radians().sin_cos();
      for (i, expected) in native.into_iter().enumerate() {
        let angle = (15.0 * i as f64).to_radians();
        let direction = [angle.sin(), -angle.cos(), 0.0];
        let normal = [s * angle.cos(), angle.sin()];
        let norm = normal[0].hypot(normal[1]);
        let ring = tube_ring(direction, 4.0 * radius_px / (500.0 * HEX_APOTHEM));
        let extent = ring.map(|p| (p[0] * normal[0] + (s * p[1] - c * p[2]) * normal[1]) / norm);
        let width = extent.into_iter().fold(f64::NEG_INFINITY, f64::max)
          - extent.into_iter().fold(f64::INFINITY, f64::min);
        assert!(
          (width - expected).abs() < 0.13,
          "{elevation}/{i}: {width} != {expected}"
        );
      }
    }
  }
}
