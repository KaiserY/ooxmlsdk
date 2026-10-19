//! Word's solid cone rims are separate, unlit three-dimensional tubes.
//! Native PDF/XPS buffers establish six tube sectors and two cap latitudes.
//! Resolve their depth against the lathe before flattening to our 2-D scene.

use super::*;

type Vertex = [f32; 3];
type Triangle = [Vertex; 3];

struct Lathe {
  length: f32,
  radius: f32,
  taper: (f32, f32),
  to_model: [[f32; 4]; 3],
}

pub(super) fn lower_horizontal(
  items: &mut Vec<PageItem>,
  marker: Marker3DContext<'_, '_, '_>,
  bounds: HorizontalMarkerBounds,
  depth: MarkerDepth,
  taper: (f32, f32),
  style: &ClusteredColumnStyle,
) {
  let p = marker.projection;
  let length = (bounds.end_x - bounds.start_x) * p.model_width / p.input.width;
  let radius = bounds.height * 0.5 * p.model_height / p.input.height;
  let depth_radius = (depth.back - depth.front) * 0.5 * p.model_depth;
  let end = ((bounds.end_x - p.input.left) / p.input.width - 0.5) * p.model_width;
  let center =
    ((bounds.y + bounds.height * 0.5 - p.input.top) / p.input.height - 0.5) * p.model_height;
  lower(
    items,
    marker,
    style,
    Lathe {
      length: length.abs(),
      radius,
      taper: (taper.1, taper.0),
      to_model: [
        [0.0, 0.0, -length.signum(), end],
        [1.0, 0.0, 0.0, center],
        [
          0.0,
          -depth_radius / radius,
          0.0,
          ((depth.front + depth.back) * 0.5 - 0.5) * p.model_depth,
        ],
      ],
    },
  );
}

pub(super) fn lower_vertical(
  items: &mut Vec<PageItem>,
  marker: Marker3DContext<'_, '_, '_>,
  bounds: VerticalMarkerBounds,
  depth: MarkerDepth,
  taper: (f32, f32),
  style: &ClusteredColumnStyle,
) {
  let p = marker.projection;
  let length = (bounds.end_y - bounds.start_y) * p.model_height / p.input.height;
  let radius = bounds.width * 0.5 * p.model_width / p.input.width;
  let depth_radius = (depth.back - depth.front) * 0.5 * p.model_depth;
  let center =
    ((bounds.x + bounds.width * 0.5 - p.input.left) / p.input.width - 0.5) * p.model_width;
  let start = ((bounds.start_y - p.input.top) / p.input.height - 0.5) * p.model_height;
  lower(
    items,
    marker,
    style,
    Lathe {
      length: length.abs(),
      radius,
      taper,
      to_model: [
        [-1.0, 0.0, 0.0, center],
        [0.0, 0.0, length.signum(), start],
        [
          0.0,
          -depth_radius / radius,
          0.0,
          ((depth.front + depth.back) * 0.5 - 0.5) * p.model_depth,
        ],
      ],
    },
  );
}

fn ring(radius: f32, angle: f32, z: f32) -> Vertex {
  [-radius * angle.sin(), radius * angle.cos(), z]
}

fn circle_tube(radius: f32, z: f32, pen_radius: f32, segments: usize) -> Vec<Triangle> {
  let step = std::f32::consts::TAU / segments as f32;
  let half = step * 0.5;
  let section = |index: usize, outgoing: bool| {
    let angle = index as f32 * step;
    let center = ring(radius, angle, z);
    std::array::from_fn::<_, 6, _>(|sector| {
      let theta = half + sector as f32 * std::f32::consts::TAU / 6.0;
      let radial = pen_radius * theta.cos();
      // The small circular turns weld their outer miter; the inside retains
      // both edge normals. Native24/27/29-sector rims have9*n+17 vertices.
      let (direction, scale) = if radial > 0.0 && index > 0 && index < segments {
        (angle, 1.0 / half.cos())
      } else {
        (angle + if outgoing { half } else { -half }, 1.0)
      };
      [
        center[0] - direction.sin() * radial * scale,
        center[1] + direction.cos() * radial * scale,
        z + pen_radius * theta.sin(),
      ]
    })
  };
  let mut triangles = Vec::with_capacity(12 * segments + 36);
  for index in 0..segments {
    let start = section(index, true);
    let end = section(index + 1, false);
    for i in 0..6 {
      let j = (i + 1) % 6;
      triangles.push([start[i], end[i], start[j]]);
      triangles.push([start[j], end[i], end[j]]);
    }
  }
  for (index, outgoing, sign) in [(0, true, -1.0), (segments, false, 1.0)] {
    let angle = index as f32 * step;
    let edge = angle + if outgoing { half } else { -half };
    let center = ring(radius, angle, z);
    let direction = [-edge.cos() * sign, -edge.sin() * sign, 0.0];
    let base = section(index, outgoing);
    let middle: [Vertex; 6] = std::array::from_fn(|i| {
      std::array::from_fn(|axis| {
        center[axis]
          + (base[i][axis] - center[axis] + pen_radius * direction[axis])
            * std::f32::consts::FRAC_1_SQRT_2
      })
    });
    let pole = std::array::from_fn(|axis| center[axis] + pen_radius * direction[axis]);
    for i in 0..6 {
      let j = (i + 1) % 6;
      triangles.push([base[i], middle[i], base[j]]);
      triangles.push([base[j], middle[i], middle[j]]);
      triangles.push([middle[i], pole, middle[j]]);
    }
  }
  triangles
}

fn lathe_faces(lathe: &Lathe, segments: usize) -> Vec<Triangle> {
  let step = std::f32::consts::TAU / segments as f32;
  let mut triangles = Vec::with_capacity(4 * segments);
  for index in 0..segments {
    let a = index as f32 * step;
    let b = (index + 1) as f32 * step;
    let start_a = ring(lathe.radius * lathe.taper.0, a, 0.0);
    let start_b = ring(lathe.radius * lathe.taper.0, b, 0.0);
    let end_a = ring(lathe.radius * lathe.taper.1, a, lathe.length);
    let end_b = ring(lathe.radius * lathe.taper.1, b, lathe.length);
    triangles.extend([
      [start_a, end_a, start_b],
      [start_b, end_a, end_b],
      [[0.0, 0.0, 0.0], start_a, start_b],
      [[0.0, 0.0, lathe.length], end_a, end_b],
    ]);
  }
  triangles
}

#[derive(Clone)]
struct ProjectedTriangle {
  points: [[f64; 2]; 3],
  depth: [f64; 3],
  bounds: [f64; 4],
}

impl ProjectedTriangle {
  fn new(points: [[f64; 3]; 3]) -> Option<Self> {
    let [a, b, c] = points;
    let determinant = (b[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (b[1] - a[1]);
    if determinant == 0.0 {
      return None;
    }
    let dx = ((b[2] - a[2]) * (c[1] - a[1]) - (c[2] - a[2]) * (b[1] - a[1])) / determinant;
    let dy = ((b[0] - a[0]) * (c[2] - a[2]) - (c[0] - a[0]) * (b[2] - a[2])) / determinant;
    let mut points = points.map(|p| [p[0], p[1]]);
    if determinant < 0.0 {
      points.swap(1, 2);
    }
    Some(Self {
      points,
      depth: [dx, dy, a[2] - dx * a[0] - dy * a[1]],
      bounds: [
        a[0].min(b[0]).min(c[0]),
        a[1].min(b[1]).min(c[1]),
        a[0].max(b[0]).max(c[0]),
        a[1].max(b[1]).max(c[1]),
      ],
    })
  }
}

fn clip(polygon: &[[f64; 2]], distance: impl Fn([f64; 2]) -> f64) -> Vec<[f64; 2]> {
  let Some(&last) = polygon.last() else {
    return Vec::new();
  };
  let mut previous = last;
  let mut before = distance(previous);
  let mut result = Vec::new();
  for &current in polygon {
    let after = distance(current);
    if (before >= 0.0) != (after >= 0.0) {
      let t = before / (before - after);
      result.push(std::array::from_fn(|axis| {
        previous[axis] + t * (current[axis] - previous[axis])
      }));
    }
    if after >= 0.0 {
      result.push(current);
    }
    previous = current;
    before = after;
  }
  result
}

fn visible_fragments(tube: &ProjectedTriangle, faces: &[ProjectedTriangle]) -> Vec<Vec<[f64; 2]>> {
  let mut fragments = vec![tube.points.to_vec()];
  for face in faces {
    if face.bounds[0] >= tube.bounds[2]
      || face.bounds[2] <= tube.bounds[0]
      || face.bounds[1] >= tube.bounds[3]
      || face.bounds[3] <= tube.bounds[1]
    {
      continue;
    }
    // Under parallel projection z is affine in screen x/y. Subtract only
    // the part of this face nearer than the tube, rather than hiding a rim
    // wholesale or altering its authored pen width. This is a depth test.
    let nearer = |p: [f64; 2]| {
      (tube.depth[0] - face.depth[0]) * p[0]
        + (tube.depth[1] - face.depth[1]) * p[1]
        + tube.depth[2]
        - face.depth[2]
    };
    let occluder = clip(&face.points, nearer);
    if occluder.len() < 3 {
      continue;
    }
    let mut remaining = Vec::new();
    for mut fragment in fragments {
      for i in 0..occluder.len() {
        let a = occluder[i];
        let b = occluder[(i + 1) % occluder.len()];
        let inside = |p: [f64; 2]| (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]);
        let outside = clip(&fragment, |p| -inside(p));
        if outside.len() >= 3 {
          remaining.push(outside);
        }
        fragment = clip(&fragment, inside);
        if fragment.len() < 3 {
          break;
        }
      }
    }
    fragments = remaining;
    if fragments.is_empty() {
      break;
    }
  }
  fragments
}

fn lower(
  items: &mut Vec<PageItem>,
  marker: Marker3DContext<'_, '_, '_>,
  style: &ClusteredColumnStyle,
  lathe: Lathe,
) {
  let projection = marker.projection;
  if !projection.right_angle_axes
    || projection.model_pen_scale.is_none()
    || lathe.radius <= f32::EPSILON
    || lathe.length <= f32::EPSILON
  {
    return;
  }
  let Some(crate::common::ShapeStyleValue::Paint(stroke)) =
    chart_series_stroke_style(style, marker.series_index, Some(marker.category_index))
  else {
    return;
  };
  if stroke.color.a != 255
    || stroke.dash.is_some()
    || stroke
      .preset_dash
      .is_some_and(|dash| dash != crate::common::StrokeDashPreset::Solid)
    || stroke.gradient.is_some()
    || stroke.pattern.is_some()
  {
    return;
  }
  let segments = word_chart_lathe_segment_count(
    lathe.radius * lathe.taper.0.max(lathe.taper.1) / projection.model_width,
  );
  let pen_radius =
    projection.pen_width(stroke.width.0, style.stroke_scale) / projection.scale * 0.5;
  let project = |triangle: Triangle| {
    ProjectedTriangle::new(triangle.map(|p| {
      let model = lathe
        .to_model
        .map(|row| row[0] * p[0] + row[1] * p[1] + row[2] * p[2] + row[3]);
      let (x, y) = projection.project_model(model);
      [f64::from(x), f64::from(y), f64::from(model[2])]
    }))
  };
  let faces = if matches!(
    chart_series_fill_style(style, marker.series_index, Some(marker.category_index)),
    Some(crate::common::ShapeStyleValue::NoPaint)
  ) {
    Vec::new()
  } else {
    lathe_faces(&lathe, segments)
      .into_iter()
      .filter_map(project)
      .collect::<Vec<_>>()
  };
  let color = RgbColor {
    r: stroke.color.r,
    g: stroke.color.g,
    b: stroke.color.b,
  };
  for (z, ratio) in [(0.0, lathe.taper.0), (lathe.length, lathe.taper.1)] {
    if ratio <= f32::EPSILON {
      continue;
    }
    for tube in circle_tube(lathe.radius * ratio, z, pen_radius, segments)
      .into_iter()
      .filter_map(project)
    {
      for fragment in visible_fragments(&tube, &faces) {
        let points = fragment
          .into_iter()
          .map(|p| (p[0] as f32, p[1] as f32))
          .collect::<Vec<_>>();
        // Keep each visible mesh fragment independent: a compound even-odd
        // path cancels the overlap of the tube's near and far faces.
        push_chart_polygon(items, &points, color, None);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn word_chart_rim_matches_native_vertices_and_indices() {
    let triangles = circle_tube(1.356_656_8, 100.0, 0.176_572_81, 24);
    assert_eq!(triangles.len() * 3, 972);
    // Actual native VF3/16-byte vertices, independent from our projection.
    for expected in [
      [0.175_062_22, 1.379_704_1, 100.0],
      [-0.008_819_81, 1.423_650_3, 100.163_13],
      [-0.022_850_14, 1.530_221_3, 100.023_05],
    ] {
      assert!(
        triangles.iter().flatten().any(|p| p
          .iter()
          .zip(expected)
          .all(|(a, b)| (*a - b).abs() < 0.000_01)),
        "missing native point {expected:?}"
      );
    }
    assert_eq!(
      circle_tube(2.713_313_7, 100.0, 0.176_572_81, 29).len() * 3,
      1152
    );
    assert_eq!(
      circle_tube(2.170_651, 100.0, 0.176_572_81, 27).len() * 3,
      1080
    );
  }

  #[test]
  fn word_chart_rim_depth_clips_only_the_hidden_overlap() {
    let tube = ProjectedTriangle::new([[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 2.0, 0.0]]).unwrap();
    let hidden =
      ProjectedTriangle::new([[0.0, 0.0, -1.0], [1.0, 0.0, -1.0], [0.0, 1.0, -1.0]]).unwrap();
    let fragments = visible_fragments(&tube, &[hidden]);
    let area = |p: &[[f64; 2]]| {
      p.iter()
        .zip(p.iter().cycle().skip(1))
        .map(|(a, b)| a[0] * b[1] - a[1] * b[0])
        .sum::<f64>()
        * 0.5
    };
    assert!((fragments.iter().map(|p| area(p)).sum::<f64>() - 1.5).abs() < 1e-12);
    let behind =
      ProjectedTriangle::new([[0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [0.0, 1.0, 1.0]]).unwrap();
    assert_eq!(
      visible_fragments(&tube, &[behind]),
      vec![tube.points.to_vec()]
    );
  }
}
