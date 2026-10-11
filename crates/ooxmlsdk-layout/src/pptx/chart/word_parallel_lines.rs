//! Native Word grid pens are three-dimensional hexagonal tubes, including
//! rounded caps and bends. Projecting a uniform two-dimensional pen loses
//! their physical cross section. Native VF7 buffers corroborate this mesh.

use super::*;

type Vertex = [f32; 3];
type Triangle = [Vertex; 3];
type Section = [Vertex; 6];

fn dot(a: Vertex, b: Vertex) -> f32 {
  a.into_iter().zip(b).map(|(a, b)| a * b).sum()
}

fn cross(a: Vertex, b: Vertex) -> Vertex {
  [
    a[1] * b[2] - a[2] * b[1],
    a[2] * b[0] - a[0] * b[2],
    a[0] * b[1] - a[1] * b[0],
  ]
}

fn rotate(v: Vertex, axis: Vertex, cosine: f32, sine: f32) -> Vertex {
  let c = cross(axis, v);
  let d = dot(axis, v) * (1.0 - cosine);
  std::array::from_fn(|i| v[i] * cosine + c[i] * sine + axis[i] * d)
}

fn join_triangles(triangles: &mut Vec<Triangle>, start: Section, end: Section) {
  for i in 0..6 {
    let j = (i + 1) % 6;
    triangles.extend([[start[i], end[i], start[j]], [start[j], end[i], end[j]]]);
  }
}

fn tube(path: &[Vertex], radius: f32) -> Vec<Triangle> {
  let path = path.iter().copied().fold(Vec::new(), |mut points, point| {
    if points.last() != Some(&point) {
      points.push(point);
    }
    points
  });
  if path.len() < 2 {
    return Vec::new();
  }
  let directions = path
    .windows(2)
    .map(|p| {
      let d = std::array::from_fn(|i| p[1][i] - p[0][i]);
      let length = dot(d, d).sqrt();
      d.map(|v| v / length)
    })
    .collect::<Vec<_>>();
  // These paths follow the three physical chart axes. Preserve the native
  // initial frame and transport it through each bend; restarting a frame
  // on every edge changes the projected hexagon.
  let normal = if directions[0][1].abs() > 0.5 {
    [0.0, 0.0, 1.0]
  } else {
    [0.0, 1.0, 0.0]
  };
  let binormal = cross(directions[0], normal);
  let sine60 = 3.0_f32.sqrt() * 0.5;
  let rays = [
    (1.0, 0.0),
    (0.5, sine60),
    (-0.5, sine60),
    (-1.0, 0.0),
    (-0.5, -sine60),
    (0.5, -sine60),
  ];
  let mut offsets: Section =
    rays.map(|(c, s)| std::array::from_fn(|i| radius * (normal[i] * c + binormal[i] * s)));
  let first: Section = offsets.map(|o| std::array::from_fn(|i| path[0][i] + o[i]));
  let mut current = first;
  let mut triangles = Vec::new();
  for index in 1..path.len() {
    let incoming: Section = offsets.map(|o| std::array::from_fn(|i| path[index][i] + o[i]));
    join_triangles(&mut triangles, current, incoming);
    if index == path.len() - 1 {
      current = incoming;
      break;
    }
    let incoming_direction = directions[index - 1];
    let outgoing_direction = directions[index];
    let axis = cross(incoming_direction, outgoing_direction);
    let sine = dot(axis, axis).sqrt();
    let cosine = dot(incoming_direction, outgoing_direction);
    if sine == 0.0 {
      current = incoming;
      continue;
    }
    let axis = axis.map(|v| v / sine);
    let next = offsets.map(|o| rotate(o, axis, cosine, sine));
    let outgoing: Section = next.map(|o| std::array::from_fn(|i| path[index][i] + o[i]));
    let half_cosine = ((1.0 + cosine) * 0.5).sqrt();
    let half_sine = sine / (2.0 * half_cosine);
    let middle: Section = offsets.map(|o| {
      let rotated = rotate(o, axis, half_cosine, half_sine);
      std::array::from_fn(|i| path[index][i] + rotated[i])
    });
    let outside = offsets.map(|o| dot(o, outgoing_direction) <= 0.0);
    for i in 0..6 {
      let j = (i + 1) % 6;
      match (outside[i], outside[j]) {
        (true, true) => triangles.extend([
          [incoming[i], middle[i], incoming[j]],
          [incoming[j], middle[i], middle[j]],
          [middle[i], outgoing[i], middle[j]],
          [middle[j], outgoing[i], outgoing[j]],
        ]),
        (true, false) => triangles.extend([
          [incoming[i], middle[i], incoming[j]],
          [middle[i], outgoing[i], incoming[j]],
        ]),
        (false, true) => triangles.extend([
          [incoming[j], middle[j], incoming[i]],
          [middle[j], outgoing[j], incoming[i]],
        ]),
        (false, false) => {}
      }
    }
    offsets = next;
    current = outgoing;
  }
  for (center, direction, base, sign) in [
    (path[0], directions[0], first, -1.0),
    (
      path[path.len() - 1],
      directions[directions.len() - 1],
      current,
      1.0,
    ),
  ] {
    let middle: Section = base.map(|p| {
      std::array::from_fn(|i| {
        center[i]
          + (p[i] - center[i] + sign * direction[i] * radius) * std::f32::consts::FRAC_1_SQRT_2
      })
    });
    let pole = std::array::from_fn(|i| center[i] + sign * direction[i] * radius);
    for i in 0..6 {
      let j = (i + 1) % 6;
      if sign < 0.0 {
        triangles.extend([
          [base[i], base[j], middle[j]],
          [middle[j], middle[i], base[i]],
        ]);
      } else {
        triangles.extend([
          [base[i], middle[i], base[j]],
          [base[j], middle[i], middle[j]],
        ]);
      }
      triangles.push([middle[i], pole, middle[j]]);
    }
  }
  triangles
}

fn lower(
  items: &mut Vec<PageItem>,
  projection: Chart3DProjection,
  path: &[Vertex],
  width: f32,
  minimum: f32,
  color: crate::common::Color,
) {
  let radius = projection.pen_width(width, minimum) / projection.scale * 0.5;
  let material = projection.unlit_pen_color(RgbColor {
    r: color.r,
    g: color.g,
    b: color.b,
  });
  for triangle in tube(path, radius) {
    let projected = triangle.map(|p| {
      projection.project_model([
        p[0] - projection.model_width * 0.5,
        p[1] - projection.model_height * 0.5,
        projection.model_depth * 0.5 - p[2],
      ])
    });
    // Native indexed triangles own independent coverage. PathItem uses
    // even-odd fill, so combining both sides of a closed tube would cancel
    // its interior. The scene resolves these opaque faces together on the
    // same MSAA samples, without independently antialiasing their seams.
    push_chart_polygon(items, &projected, material, None);
  }
}

pub(super) fn lower_grid(
  items: &mut Vec<PageItem>,
  projection: Chart3DProjection,
  position: f32,
  horizontal_value: bool,
  width: f32,
  minimum: f32,
  color: crate::common::Color,
) -> bool {
  if !projection.right_angle_axes || projection.model_pen_scale.is_none() || color.a != 255 {
    return false;
  }
  // Native paths extend across their walls by twice the existing model
  // plane guard (.014 in a100-unit scene), before generating rounded caps.
  let guard = 0.00014 * projection.model_width;
  let [w, h, d] = [
    projection.model_width,
    projection.model_height,
    projection.model_depth,
  ];
  let path = if horizontal_value {
    let x = (position - projection.input.left) / projection.input.width * w;
    [
      [x, h + guard, d],
      [x, h, d],
      [x, h, 0.0],
      [x, 0.0, 0.0],
      [x, 0.0, -guard],
    ]
  } else {
    let y = (position - projection.input.top) / projection.input.height * h;
    [
      [-guard, y, d],
      [0.0, y, d],
      [0.0, y, 0.0],
      [w, y, 0.0],
      [w, y, -guard],
    ]
  };
  lower(items, projection, &path, width, minimum, color);
  true
}

pub(super) fn lower_styled_grid(
  items: &mut Vec<PageItem>,
  projection: Chart3DProjection,
  position: f32,
  horizontal_value: bool,
  stroke: &crate::common::Stroke<'_>,
  minimum: f32,
) -> bool {
  if stroke.dash.is_some()
    || stroke
      .preset_dash
      .is_some_and(|d| d != crate::common::StrokeDashPreset::Solid)
    || stroke.pattern.is_some()
    || stroke.gradient.is_some()
  {
    return false;
  }
  lower_grid(
    items,
    projection,
    position,
    horizontal_value,
    stroke.width.0,
    minimum,
    stroke.color,
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn word_chart_grid_tube_keeps_overlapping_faces_opaque() {
    let mut projection = cartesian_3d_projection(
      Chart3DView {
        rotate_x_deg: 0.0,
        rotate_y_deg: 0.0,
        right_angle_axes: true,
        height_percent: 50.0,
        height_percent_is_explicit: true,
        ..Default::default()
      },
      PlotRect {
        left: 0.0,
        top: 0.0,
        width: 40.0,
        height: 20.0,
      },
      ChartLayoutProfile::Word,
      0.1,
      false,
    );
    projection.model_pen_scale = Some(1.0);
    let mut items = Vec::new();
    lower(
      &mut items,
      projection,
      &[[0.1, 0.25, 0.0], [0.9, 0.25, 0.0]],
      4.0,
      1.0,
      common_rgb(RgbColor { r: 0, g: 0, b: 0 }, 1.0),
    );
    let display = items
      .into_iter()
      .map(|item| match item {
        PageItem::Path(path) => crate::common::DisplayItem::Path(path),
        _ => panic!("expected physical tube faces"),
      })
      .collect::<Vec<_>>();
    let image = crate::common::drawingml_shape_raster::rasterize_vector_scene_at_mapping(
      &display,
      crate::common::drawingml_shape_raster::PageToRasterMapping {
        width_px: 40,
        height_px: 20,
        scale_x: 1.0,
        scale_y: 1.0,
        translate_x: 0.0,
        translate_y: 0.0,
        text_hinting: None,
      },
    )
    .unwrap();
    for x in 5..35 {
      assert_eq!(image.get_pixel(x, 10).0, [0, 0, 0, 255]);
    }
  }

  #[test]
  fn word_chart_grid_tubes_match_native_axis_and_bend_meshes() {
    let triangles = tube(
      &[
        [100.0, 65.133_53, 6.783_284],
        [100.0, 65.119_53, 6.783_284],
        [100.0, 65.119_53, 0.0],
        [100.0, 0.0, 0.0],
        [100.0, 0.0, -0.014],
      ],
      0.176_572_81,
    );
    assert_eq!(triangles.len() * 3, 360);
    for expected in [
      [100.152_916, 65.133_53, 6.871_571],
      [100.0, 65.296_104, 0.0],
      [100.152_916, 65.181_96, -0.062_428],
      [100.0, -0.176_573, -0.014],
    ] {
      assert!(
        triangles.iter().flatten().any(|p| p
          .iter()
          .zip(expected)
          .all(|(a, b)| (*a - b).abs() < 0.000_01)),
        "missing native vertex {expected:?}"
      );
    }
    assert_eq!(
      tube(
        &[
          [-0.014, 0.0, 25.0],
          [0.0, 0.0, 25.0],
          [0.0, 0.0, 0.0],
          [100.0, 0.0, 0.0],
          [100.0, 0.0, -0.014]
        ],
        0.146_828_01
      )
      .len()
        * 3,
      396
    );
    assert_eq!(
      tube(
        &[[0.0, 63.398_426, 25.0], [100.0, 63.398_426, 25.0]],
        0.146_828_01
      )
      .len()
        * 3,
      144
    );
  }
}
