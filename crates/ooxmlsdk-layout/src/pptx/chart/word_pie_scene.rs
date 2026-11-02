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
  word_chart_lathe_maximum_step,
};
use crate::common::drawingml_shape_raster::PageToRasterMapping;
use crate::common::{Fill, GradientFill, GradientStop, ShapeStyleValue};
use image::{Rgba, RgbaImage};

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

  fn position(&self, uv: [f64; 2]) -> f64 {
    uv[0] * self.axis[0] + uv[1] * self.axis[1] + self.offset
  }

  fn color(&self, uv: [f64; 2]) -> [f64; 3] {
    let position = self.position(uv) as f32;
    let c = crate::common::drawingml_gradient::sample(&self.stops, position);
    [f64::from(c.r), f64::from(c.g), f64::from(c.b)]
  }
}

/// Parallel faces are affine: transform the texture gradient's normal, not
/// its endpoints. All faces retain the same source UV and independent light.
pub(super) fn projected_linear_gradient(
  gradient: &GradientFill<'static>,
  size: [f64; 2],
  points: [[f32; 2]; 3],
  uv: [[f64; 2]; 3],
  diffuse: f32,
) -> Fill<'static> {
  let material = LinearMaterial::new(gradient, [0.0, 0.0, size[0], size[1]]);
  let positions = uv.map(|uv| material.position(uv));
  let light = |v: u8| (f32::from(v) * diffuse).round().clamp(0.0, 255.0) as u8;
  let solid = || {
    let color = crate::common::drawingml_gradient::sample(&material.stops, positions[0] as f32);
    Fill::Solid(crate::common::Color {
      r: light(color.r),
      g: light(color.g),
      b: light(color.b),
      a: color.a,
    })
  };
  let a = [
    f64::from(points[1][0] - points[0][0]),
    f64::from(points[1][1] - points[0][1]),
  ];
  let b = [
    f64::from(points[2][0] - points[0][0]),
    f64::from(points[2][1] - points[0][1]),
  ];
  let determinant = a[0] * b[1] - a[1] * b[0];
  if determinant.abs() <= f64::EPSILON {
    return solid();
  }
  let ta = positions[1] - positions[0];
  let tb = positions[2] - positions[0];
  let normal = [
    (ta * b[1] - tb * a[1]) / determinant,
    (tb * a[0] - ta * b[0]) / determinant,
  ];
  let length_squared = normal[0] * normal[0] + normal[1] * normal[1];
  if length_squared <= f64::EPSILON {
    return solid();
  }
  let endpoint = |position: f64| {
    super::common_point(
      (f64::from(points[0][0]) + normal[0] * (position - positions[0]) / length_squared) as f32,
      (f64::from(points[0][1]) + normal[1] * (position - positions[0]) / length_squared) as f32,
    )
  };
  let stops = material
    .stops
    .iter()
    .map(|stop| GradientStop {
      position: stop.position,
      color: crate::common::Color {
        r: light(stop.color.r),
        g: light(stop.color.g),
        b: light(stop.color.b),
        a: stop.color.a,
      },
      scheme: None,
    })
    .collect();
  Fill::Gradient(GradientFill {
    stops,
    line: Some((endpoint(0.0), endpoint(1.0))),
    ..Default::default()
  })
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

fn pie_model_pen_radius(width: f32, plot_width: f32) -> f64 {
  // Native .5/2/4/8pt and independent viewport/elevation controls normalize
  // the authored pen to the nominal plot width, before fitting the camera.
  f64::from(width.max(1.0) / (2.0 * plot_width))
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
    let pen_radius = match paint.map(|p| &p.stroke) {
      Some(ShapeStyleValue::Paint(stroke)) => pie_model_pen_radius(stroke.width.0, plot.width),
      _ => 0.0,
    };
    // Native gradient UV includes the outline's model-space support. The
    // same source sector with noFill on its line has the unexpanded bounds.
    let mut bounds = pie_texture_bounds(slice.start_angle, slice.sweep);
    bounds[0] -= pen_radius;
    bounds[1] -= pen_radius;
    bounds[2] += pen_radius;
    bounds[3] += pen_radius;
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
      // The independent arc pens share the body's source chord budget.
      // Their six-sided section is transported from the last endpoint;
      // flat-capped tubes per chord lose the native joins and end caps.
      let radius = pen_radius;
      let color = rgb(RgbColor {
        r: stroke.color.r,
        g: stroke.color.g,
        b: stroke.color.b,
      });
      for z in [0.0, -thickness] {
        for face in pie_arc_tube(
          f64::from(slice.start_angle),
          f64::from(slice.sweep),
          z,
          radius,
          segments,
        ) {
          scene.triangle(&projector, face, [color; 3]);
        }
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

// The same rounded-tube construction as word_lathe_outline, with the pie
// producer's reversed arc traversal and transported initial section. Native
// 7/8/17/42-segment arcs verify every indexed face, including cap diagonals.
fn pie_arc_tube(start: f64, sweep: f64, z: f64, radius: f64, segments: usize) -> Vec<[Point3; 3]> {
  let start = start + sweep;
  let step = -sweep / segments as f64;
  let half = step * 0.5;
  let phase = -start - half;
  let center = |angle: f64| [angle.sin(), -angle.cos(), z];
  let section = |index: usize, outgoing: bool| {
    let angle = start + index as f64 * step;
    let center = center(angle);
    std::array::from_fn::<_, 6, _>(|sector| {
      let theta = phase - sector as f64 * std::f64::consts::TAU / 6.0;
      let radial = radius * theta.cos();
      let (direction, scale) = if radial > 0.0 && index > 0 && index < segments {
        (angle, 1.0 / half.cos())
      } else {
        (angle + if outgoing { half } else { -half }, 1.0)
      };
      [
        center[0] + direction.sin() * radial * scale,
        center[1] - direction.cos() * radial * scale,
        z + radius * theta.sin(),
      ]
    })
  };
  let mut triangles = Vec::with_capacity(12 * segments + 36);
  for index in 0..segments {
    let first = section(index, true);
    let last = section(index + 1, false);
    for i in 0..6 {
      let j = (i + 1) % 6;
      triangles.push([first[i], last[i], first[j]]);
      triangles.push([first[j], last[i], last[j]]);
    }
  }
  for (index, outgoing, sign) in [(0, true, -1.0), (segments, false, 1.0)] {
    let angle = start + index as f64 * step;
    let edge = angle + if outgoing { half } else { -half };
    let center = center(angle);
    let direction = [-edge.cos() * sign, -edge.sin() * sign, 0.0];
    let base = section(index, outgoing);
    let middle: [Point3; 6] = std::array::from_fn(|i| {
      std::array::from_fn(|axis| {
        center[axis]
          + (base[i][axis] - center[axis] + radius * direction[axis])
            * std::f64::consts::FRAC_1_SQRT_2
      })
    });
    let pole = std::array::from_fn(|axis| center[axis] + radius * direction[axis]);
    for i in 0..6 {
      let j = (i + 1) % 6;
      if outgoing {
        triangles.push([base[i], middle[i], middle[j]]);
        triangles.push([base[i], middle[j], base[j]]);
      } else {
        triangles.push([base[i], middle[i], base[j]]);
        triangles.push([base[j], middle[i], middle[j]]);
      }
      triangles.push([middle[i], pole, middle[j]]);
    }
  }
  triangles
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
  pub(super) fn linear_material(
    &mut self,
    gradient: &GradientFill<'static>,
    bounds: [f64; 4],
  ) -> usize {
    let index = self.materials.len();
    self.materials.push(LinearMaterial::new(gradient, bounds));
    index
  }

  pub(super) fn projected_textured_triangle(
    &mut self,
    points: [[f64; 2]; 3],
    depth: [f64; 3],
    uv: [[f64; 2]; 3],
    material: usize,
    diffuse: f32,
  ) {
    self.triangles.push(Triangle {
      vertices: std::array::from_fn(|i| Vertex {
        point: points[i],
        depth: depth[i],
        color: [255.0 * f64::from(diffuse); 3],
        texture: [uv[i][0] * depth[i], uv[i][1] * depth[i], depth[i]],
      }),
      material: Some(material),
    });
  }

  /// Reuse the depth-tested mesh for Cartesian surface triangles. Projected
  /// depth is reciprocal clip-W; materials are resolved before submission.
  pub(super) fn projected_triangle(
    &mut self,
    points: [[f64; 2]; 3],
    depth: [f64; 3],
    color: RgbColor,
  ) {
    self.triangles.push(Triangle {
      vertices: std::array::from_fn(|i| Vertex {
        point: points[i],
        depth: depth[i],
        color: rgb(color),
        texture: [0.0, 0.0, 1.0],
      }),
      material: None,
    });
  }

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

  #[test]
  fn pie_arc_tubes_match_native_end_caps_and_transported_sections() {
    // Captured unlit VF3 buffers. Independent sector angles distinguish
    // transported six-sided sections from a fixed global ring phase.
    for (start, sweep, segments, expected) in [
      (
        12.8,
        1.2,
        7,
        [
          -0.5252124786376953,
          -1.0128148651123048,
          -0.011352968215942384,
          0.012824327945709228,
          -0.8461388397216797,
          0.011352970600128173,
        ],
      ),
      (
        11.4,
        1.4,
        8,
        [
          -0.9307310485839844,
          -0.8701849365234375,
          -0.012814946174621582,
          -0.5008771896362305,
          -0.38135581970214844,
          0.012814947366714478,
        ],
      ),
      (
        8.2,
        3.2,
        17,
        [
          -1.0125043487548828,
          -0.4050287628173828,
          -0.012003519535064698,
          -0.5003983688354492,
          0.8709156036376953,
          0.012003518342971801,
        ],
      ),
      (
        0.0,
        8.2,
        42,
        [
          -0.5246782684326172,
          -1.0112840270996093,
          -0.012828474044799804,
          1.0112849426269532,
          1.0112226104736328,
          0.012828474044799804,
        ],
      ),
    ] {
      let faces = super::pie_arc_tube(
        start / 14.0 * std::f64::consts::TAU,
        sweep / 14.0 * std::f64::consts::TAU,
        0.0,
        0.012_833_812_236_785_89,
        segments,
      );
      assert_eq!(faces.len(), segments * 12 + 36);
      let mut bounds = [f64::INFINITY; 6];
      bounds[3..].fill(f64::NEG_INFINITY);
      for point in faces.into_iter().flatten() {
        for axis in 0..3 {
          bounds[axis] = bounds[axis].min(point[axis]);
          bounds[axis + 3] = bounds[axis + 3].max(point[axis]);
        }
      }
      for (actual, expected) in bounds.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 0.000_003);
      }
    }
  }
  #[test]
  fn pie_pen_radius_matches_native_viewport_and_width_controls() {
    // Actual first endpoint poles from eight independent native GFX uploads.
    for (width, plot_width, native_radius) in [
      (2.0, 155.83835, 0.6416906118392944),
      (8.0, 155.83835, 2.5667624473571777),
      (4.0, 243.78835, 0.8203837275505067),
      (4.0, 373.78836, 0.5350621342658997),
      (4.0, 155.83835, 1.2833812236785889), // taller viewport
      (4.0, 155.83835, 1.2833812236785889), // elevation60
      (4.0, 155.83835, 1.2833812236785889), // perspective0
      (0.5, 155.83835, 0.3208453059196472),
    ] {
      let actual = super::pie_model_pen_radius(width, plot_width) * 100.0;
      assert!((actual - native_radius).abs() < 0.000001);
    }
  }

  use super::*;

  #[test]
  fn projected_surface_depth_resolves_crossing_faces_independently_of_order() {
    let points = [[0.0, 0.0], [12.0, 0.0], [0.0, 12.0]];
    let red = RgbColor { r: 255, g: 0, b: 0 };
    let blue = RgbColor { r: 0, g: 0, b: 255 };
    let render = |reverse| {
      let mut scene = Scene::default();
      let triangles = [(red, [1.0, 3.0, 1.0]), (blue, [2.0; 3])];
      for i in if reverse { [1, 0] } else { [0, 1] } {
        scene.projected_triangle(points, triangles[i].1, triangles[i].0);
      }
      scene
        .rasterize(PageToRasterMapping {
          width_px: 12,
          height_px: 12,
          scale_x: 1.0,
          scale_y: 1.0,
          translate_x: 0.0,
          translate_y: 0.0,
          text_hinting: None,
        })
        .unwrap()
    };
    let image = render(false);
    assert_eq!(image, render(true));
    assert_eq!(image.get_pixel(1, 1).0, [0, 0, 255, 255]);
    assert_eq!(image.get_pixel(8, 1).0, [255, 0, 0, 255]);
  }

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
        let ring = tube_ring(
          direction,
          4.0 * radius_px / (500.0 * std::f64::consts::FRAC_PI_6.cos()),
        );
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
