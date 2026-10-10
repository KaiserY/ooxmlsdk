//! Word's outlined pie is a depth-tested mesh, including its pens.
//!
//! Independent native wire controls at elevations 15/90 and seven rotations
//! identify a six-sided tube. Its initial axis is -Z, its hexagon has vertices
//! at 30 + 60n degrees, and a shortest-arc rotation aligns it to the edge.
//! A screen-space stroke loses both this angular footprint and cap occlusion.
//! Linear gradient materials use the analytic source-sector box on every
//! face. Native meshes retain the same UV at both extrusion depths, separate
//! seven-bit white lighting from the texture, and project UV through clip W.

use super::{
  Chart3DView, ImageItem, PlotRect, RadialChartStyle, RadialPerspectiveProjection, RadialSlice,
  RgbColor, chart_3d_scene_image, common_rect, office_perspective_chart_diffuse_color,
  word_chart_lathe_maximum_step, word_pie_3d_pen_width,
};
use crate::common::drawingml_shape_raster::PageToRasterMapping;
use crate::common::{Fill, GradientFill, GradientStop, ShapeStyleValue};
use image::{Rgba, RgbaImage};

const HEX_APOTHEM: f64 = 0.866_025_403_784_438_6;
type Point3 = [f64; 3];

#[derive(Clone, Copy)]
struct Vertex {
  point: [f64; 2],
  depth: f64,
  color: [f64; 3],
  // u/W, v/W and 1/W; lighting remains a screen-linear vertex attribute.
  texture: [f64; 3],
}

#[derive(Clone, Copy)]
struct Triangle {
  vertices: [Vertex; 3],
  material: Option<usize>,
}

struct LinearMaterial {
  stops: Vec<GradientStop<'static>>,
  axis: [f64; 2],
  offset: f64,
}

impl LinearMaterial {
  fn new(gradient: &GradientFill<'static>, bounds: [f64; 4]) -> Self {
    let size = [bounds[2] - bounds[0], bounds[3] - bounds[1]];
    let angle = f64::from(gradient.angle_degrees.unwrap_or(0.0)).to_radians();
    let (sine, cosine) = angle.sin_cos();
    let mut direction = [cosine, sine];
    if gradient.scaled {
      direction = [cosine * size[0], sine * size[1]];
    }
    let length = direction[0].hypot(direction[1]);
    direction = direction.map(|v| v / length.max(f64::EPSILON));
    let span = direction[0].abs() * size[0] + direction[1].abs() * size[1];
    let axis = [direction[0] * size[0] / span, direction[1] * size[1] / span];
    Self {
      stops: crate::common::drawingml_gradient::resolved_stops(gradient),
      axis,
      offset: 0.5 - (axis[0] + axis[1]) * 0.5,
    }
  }

  fn color(&self, uv: [f64; 2]) -> [f64; 3] {
    let position = (uv[0] * self.axis[0] + uv[1] * self.axis[1] + self.offset) as f32;
    let c = crate::common::drawingml_gradient::sample(&self.stops, position);
    [f64::from(c.r), f64::from(c.g), f64::from(c.b)]
  }
}

#[derive(Default)]
pub(super) struct Scene {
  triangles: Vec<Triangle>,
  materials: Vec<LinearMaterial>,
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
      texture: [
        0.0,
        0.0,
        if self.parallel {
          1.0
        } else {
          denominator.recip()
        },
      ],
    }
  }
}

fn rgb(color: RgbColor) -> [f64; 3] {
  [f64::from(color.r), f64::from(color.g), f64::from(color.b)]
}

fn diffuse(color: RgbColor, normal: [f32; 3]) -> [f64; 3] {
  // Recover the native seven-bit vertex attributes before interpolation,
  // matching the existing FixedGouraud7 scene gradient path.
  vertex_rgb(office_perspective_chart_diffuse_color(color, normal))
}

fn vertex_rgb(color: RgbColor) -> [f64; 3] {
  rgb(color).map(|c| (c * 128.0 / 255.0).round() * 255.0 / 128.0)
}

fn pie_texture_bounds(start: f32, sweep: f32) -> [f64; 4] {
  let (start, end) = (f64::from(start), f64::from(start + sweep));
  let point = |angle: f64| [angle.sin(), -angle.cos()];
  let mut bounds = [0.0_f64; 4];
  let quadrant = std::f64::consts::FRAC_PI_2;
  for angle in [start, end].into_iter().chain(
    ((start / quadrant).ceil() as i32..=(end / quadrant).floor() as i32)
      .map(|q| f64::from(q) * quadrant),
  ) {
    let p = point(angle);
    bounds[0] = bounds[0].min(p[0]);
    bounds[1] = bounds[1].min(p[1]);
    bounds[2] = bounds[2].max(p[0]);
    bounds[3] = bounds[3].max(p[1]);
  }
  bounds
}

fn pie_ring(start: f32, sweep: f32, segments: usize) -> Vec<(f32, Point3)> {
  (0..=segments)
    .map(|i| {
      let angle = start + sweep * i as f32 / segments as f32;
      (
        angle,
        [f64::from(angle.sin()), -f64::from(angle.cos()), 0.0],
      )
    })
    .collect()
}

fn cut_light(color: RgbColor, angle: f32, elevation: f32, direction: f32) -> [f64; 3] {
  let (s, c) = elevation.to_radians().sin_cos();
  let normal = [direction * angle.cos(), direction * angle.sin()];
  diffuse(color, [normal[0], s * normal[1], -c * normal[1]])
}

pub(super) fn lower(
  slices: &[RadialSlice],
  style: &RadialChartStyle,
  projection: RadialPerspectiveProjection,
  view: Chart3DView,
  plot: PlotRect,
) -> Option<ImageItem> {
  // Retain the prior realization for materials the mesh cannot represent.
  let mut outlined = false;
  let mut textured = false;
  for slice in slices {
    if style
      .point_image_effects
      .get(slice.index)
      .and_then(Option::as_ref)
      .is_some_and(|effects| !effects.effects.is_empty())
    {
      return None;
    }
    if let Some(paint) = style.point_styles.get(slice.index) {
      match &paint.fill {
        ShapeStyleValue::Paint(Fill::Solid(c)) if c.a == 255 => {}
        ShapeStyleValue::Paint(Fill::Gradient(gradient))
          if gradient.path.is_none()
            && gradient.line.is_none()
            && gradient.definition_bounds.is_none()
            && gradient.rotate_with_shape != Some(false)
            && !gradient.stops.is_empty()
            && gradient.stops.iter().all(|stop| stop.color.a == 255) =>
        {
          textured = true;
        }
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
  if !outlined && !textured {
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
    let bounds = pie_texture_bounds(slice.start_angle, slice.sweep);
    let material = match paint.map(|p| &p.fill) {
      Some(ShapeStyleValue::Paint(Fill::Gradient(gradient))) => {
        let index = scene.materials.len();
        scene.materials.push(LinearMaterial::new(gradient, bounds));
        Some(index)
      }
      _ => None,
    };
    let fill = match paint.map(|p| &p.fill) {
      Some(ShapeStyleValue::NoPaint | ShapeStyleValue::Paint(Fill::None)) => None,
      Some(ShapeStyleValue::Paint(Fill::Solid(color))) => Some(RgbColor {
        r: color.r,
        g: color.g,
        b: color.b,
      }),
      Some(ShapeStyleValue::Paint(Fill::Gradient(_))) => Some(RgbColor {
        r: 255,
        g: 255,
        b: 255,
      }),
      _ => Some(slice.color),
    };
    // GFX's source-radius-100 chord error also governs partial pie arcs.
    // Independent native sweeps have 5/13/6/19/30 segments. Derive the
    // partial count before rounding the full-circle count.
    let segments = (slice.sweep.abs() / word_chart_lathe_maximum_step(1.0)).ceil() as usize;
    let segments = segments.max(1);
    let ring = pie_ring(slice.start_angle, slice.sweep, segments);
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
        scene.face(
          &projector,
          [hub, *first, *second],
          [cap; 3],
          material,
          bounds,
        );
        scene.face(
          &projector,
          [bottom(hub), bottom(*second), bottom(*first)],
          [base; 3],
          material,
          bounds,
        );
        let light = |angle: f32| diffuse(color, [angle.sin(), -s * angle.cos(), c * angle.cos()]);
        let (ca, cb) = (light(*a), light(*b));
        scene.face(
          &projector,
          [*first, bottom(*first), *second],
          [ca, ca, cb],
          material,
          bounds,
        );
        scene.face(
          &projector,
          [*second, bottom(*first), bottom(*second)],
          [cb, ca, cb],
          material,
          bounds,
        );
      }
      for (direction, (angle, outer)) in [(1.0, ring[0]), (-1.0, ring[segments])] {
        // Native uploaded cut normals belong to the start/end plane, even
        // when it is back-facing. An angle-sign shortcut only works after
        // visibility rejection and reverses a cut at a complete-turn seam.
        let color = cut_light(color, angle, view.rotate_x_deg, direction);
        scene.face(
          &projector,
          [hub, outer, bottom(outer)],
          [color; 3],
          material,
          bounds,
        );
        scene.face(
          &projector,
          [hub, bottom(outer), bottom(hub)],
          [color; 3],
          material,
          bounds,
        );
      }
    }
    if let Some(ShapeStyleValue::Paint(stroke)) = paint.map(|p| &p.stroke) {
      // Preserve the independently verified wire tessellation. Material
      // boundaries and six-sided pen tubes are separate native primitives.
      let segments = ((slice.sweep.to_degrees().abs() / 2.0).ceil() as usize).max(2);
      let ring = pie_ring(slice.start_angle, slice.sweep, segments);
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
    self.triangles.push(Triangle {
      vertices: std::array::from_fn(|i| projector.vertex(points[i], colors[i])),
      material: None,
    });
  }

  fn face(
    &mut self,
    projector: &Projector,
    points: [Point3; 3],
    colors: [[f64; 3]; 3],
    material: Option<usize>,
    bounds: [f64; 4],
  ) {
    let vertices = std::array::from_fn(|i| {
      let mut vertex = projector.vertex(points[i], colors[i]);
      let q = vertex.texture[2];
      vertex.texture[0] = (points[i][0] - bounds[0]) / (bounds[2] - bounds[0]) * q;
      vertex.texture[1] = (points[i][1] - bounds[1]) / (bounds[3] - bounds[1]) * q;
      vertex
    });
    self.triangles.push(Triangle { vertices, material });
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
        let vertices = triangle.vertices.map(|mut v| {
          v.point = [
            v.point[0] * f64::from(mapping.scale_x) + f64::from(mapping.translate_x),
            v.point[1] * f64::from(mapping.scale_y) + f64::from(mapping.translate_y),
          ];
          v
        });
        Triangle {
          vertices,
          material: triangle.material,
        }
      })
      .collect();
    let mut depth = vec![f64::NEG_INFINITY; count];
    let mut colors = vec![[0_u8; 3]; count];
    let mut sums = vec![[0_u16; 4]; count];
    for (sx, sy) in crate::common::drawingml_3d::DIRECT3D_STANDARD_8_SAMPLES {
      depth.fill(f64::NEG_INFINITY);
      for triangle in &triangles {
        raster_triangle(
          &triangle.vertices,
          width,
          height,
          [f64::from(sx), f64::from(sy)],
          &mut depth,
          &mut colors,
          triangle.material.map(|index| &self.materials[index]),
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
  material: Option<&LinearMaterial>,
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
      let center = [x as f64 + 0.5, y as f64 + 0.5];
      let weights = edges.map(|(a, b)| edge(a, b, center) / area);
      let mut color = if flat {
        t[0].color
      } else {
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
      if let Some(material) = material {
        let uvw: [f64; 3] =
          std::array::from_fn(|channel| (0..3).map(|i| weights[i] * t[i].texture[channel]).sum());
        let texture = material.color([uvw[0] / uvw[2], uvw[1] / uvw[2]]);
        color = std::array::from_fn(|i| color[i] * texture[i] / 255.0);
      }
      colors[index] = color.map(|v| v.round().clamp(0.0, 255.0) as u8);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn gradient_sector_geometry_matches_native_mesh_counts_and_source_uv() {
    // Native GFX buffers for five independent sweeps, unchanged when
    // switching the material angle or elevation. Radius is 100 model units.
    for (sweep, expected) in [
      (22.41398_f32, 5),
      (66.45549, 13),
      (26.34626, 6),
      (92.60513, 19),
      (152.17914, 30),
    ] {
      assert_eq!(
        (sweep.to_radians() / word_chart_lathe_maximum_step(1.0)).ceil() as usize,
        expected
      );
    }
    // The sampled ring misses the analytic x extremum; the texture still
    // uses that extremum, not the tessellated or projected shape's box.
    let start = (360.0_f32 - 152.17914).to_radians();
    let bounds = pie_texture_bounds(start, 152.17914_f32.to_radians());
    let hub_u = -bounds[0] / (bounds[2] - bounds[0]);
    let hub_v = -bounds[1] / (bounds[3] - bounds[1]);
    assert!((bounds[0] + 1.0).abs() < 1.0e-6);
    assert!((hub_u - 1.0).abs() < 1.0e-6);
    assert!((hub_v - 0.5306698084).abs() < 1.0e-6);
    let white = RgbColor {
      r: 255,
      g: 255,
      b: 255,
    };
    let (sine, cosine) = 30.0_f32.to_radians().sin_cos();
    assert_eq!(
      diffuse(white, [0.0, -cosine, -sine]),
      [255.0 * 126.0 / 128.0; 3]
    );
  }

  #[test]
  fn pie_cut_lighting_matches_native_start_and_end_attributes() {
    let white = RgbColor {
      r: 255,
      g: 255,
      b: 255,
    };
    // Native white material attributes, including the back-facing planes.
    for (angle, direction, channel) in [
      (0.0_f32, 1.0, 51),
      (22.41398, -1.0, 94),
      (22.41398, 1.0, 51),
      (88.86947, -1.0, 51),
      (88.86947, 1.0, 70),
      (115.21573, -1.0, 51),
      (115.21573, 1.0, 93),
      (207.82086, -1.0, 51),
      (207.82086, 1.0, 90),
      (360.0, -1.0, 106),
    ] {
      assert_eq!(
        cut_light(white, angle.to_radians(), 30.0, direction),
        [255.0 * f64::from(channel) / 128.0; 3]
      );
    }
  }

  #[test]
  fn gradient_texture_uses_clip_w_independently_of_vertex_lighting() {
    let gradient = GradientFill {
      stops: vec![
        GradientStop {
          position: 0.0,
          color: crate::common::Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
          },
          scheme: None,
        },
        GradientStop {
          position: 1.0,
          color: crate::common::Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
          },
          scheme: None,
        },
      ],
      angle_degrees: Some(0.0),
      ..GradientFill::default()
    };
    let vertex = |point, texture| Vertex {
      point,
      texture,
      color: [255.0; 3],
      depth: 1.0,
    };
    let scene = Scene {
      triangles: vec![Triangle {
        vertices: [
          vertex([0.0, 0.0], [0.0, 0.0, 1.0]),
          vertex([2.0, 0.0], [0.5, 0.0, 0.5]),
          vertex([0.0, 2.0], [0.0, 1.0, 1.0]),
        ],
        material: Some(0),
      }],
      materials: vec![LinearMaterial::new(&gradient, [0.0, 0.0, 1.0, 1.0])],
    };
    let image = scene
      .rasterize(PageToRasterMapping {
        width_px: 2,
        height_px: 2,
        scale_x: 1.0,
        scale_y: 1.0,
        translate_x: 0.0,
        translate_y: 0.0,
        text_hinting: None,
      })
      .unwrap();
    // At pixel center u=(.25*.5)/(.5+.25*.5+.25)=1/7.
    // An affine interpolation would instead produce gray 191.
    assert_eq!(image.get_pixel(0, 0).0, [219, 219, 219, 255]);
  }

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
        texture: [0.0, 0.0, 1.0],
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
      triangles: plane(true)
        .into_iter()
        .chain(plane(false))
        .map(|vertices| Triangle {
          vertices,
          material: None,
        })
        .collect(),
      materials: Vec::new(),
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
