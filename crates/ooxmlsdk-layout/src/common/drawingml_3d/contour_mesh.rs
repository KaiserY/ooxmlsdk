//! Solid contour tubes, before projection or material realization.
//!
//! Retained exact-option Word mesh captures at 0.5, 1 and 2 pt contain six
//! circular section vertices, a constant section phase along each figure,
//! and two-ring round caps. The figure starts at its topmost/leftmost point.
//! Horizontal joins depend on the realized segment angle, not on whether a
//! vertex was authored or introduced by curve flattening.

use super::Static3dTextContour;

type Point = (f32, f32);
type Vertex = [f32; 3];
type Triangle = [Vertex; 3];
const SECTION_COUNT: usize = 6;
const LONGITUDINAL_CREASE_COSINE: f32 = 0.965_925_8; // cos(15 degrees)

/// Office's contour producer divides a quarter turn into two arc segments,
/// subtracting five degrees before rounding the count up. A bend requiring
/// at most one segment is mitered instead. This is distinct from longitudinal
/// crease selection and from the source curve's tangent continuity.
///
/// The exact-option 0.5/1/2 pt indexed meshes agree at vertices, edge midpoints
/// and face interiors. A read-only GFX join-worker capture independently
/// confirms the count and branch (including flattened and authored vertices).
/// Preserve the producer's single-precision angular constants at boundaries;
/// using only a 50-degree threshold misses subsequent arc-count boundaries.
fn horizontal_join_steps(dot: f32) -> usize {
  let bias = f32::from_bits(0x3db2_b8c2); // five degrees in radians
  let quarter_turn = f32::from_bits(0x3fc9_0fda);
  (((dot.clamp(-1.0, 1.0).acos() - bias) * 2.0) / quarter_turn).ceil() as usize
}

fn direction(first: Point, second: Point) -> Option<Point> {
  let delta = (second.0 - first.0, second.1 - first.1);
  let length = delta.0.hypot(delta.1);
  (length.is_finite() && length > f32::EPSILON).then_some((delta.0 / length, delta.1 / length))
}

fn section(radius: f32, tangent: Point) -> [(f32, f32); SECTION_COUNT] {
  let phase = -tangent.1.atan2(tangent.0);
  std::array::from_fn(|i| {
    let angle = phase + std::f32::consts::TAU * i as f32 / SECTION_COUNT as f32;
    let (sin, cos) = angle.sin_cos();
    (radius * cos, radius * sin)
  })
}

fn ring(
  point: Point,
  normal: Point,
  z: f32,
  profile: &[(f32, f32); SECTION_COUNT],
) -> [Vertex; SECTION_COUNT] {
  profile.map(|(radial, height)| {
    [
      point.0 + normal.0 * radial,
      point.1 + normal.1 * radial,
      z + height,
    ]
  })
}

fn connect(
  triangles: &mut Vec<Triangle>,
  first: &[Vertex; SECTION_COUNT],
  second: &[Vertex; SECTION_COUNT],
) {
  for i in 0..SECTION_COUNT {
    let next = (i + 1) % SECTION_COUNT;
    triangles.push([first[i], second[i], second[next]]);
    triangles.push([second[next], first[next], first[i]]);
  }
}

/// Round only the outside of a bend. The inner rails of the two segments
/// overlap; sweeping them would introduce an extra surface inside the join.
/// The cross-profile faces fan to the incoming inner rail, as in Office's
/// indexed contour meshes, rather than rotating a second, inner semicircle.
fn connect_outer_join(
  triangles: &mut Vec<Triangle>,
  first: &[Vertex; SECTION_COUNT],
  second: &[Vertex; SECTION_COUNT],
  outer: &[bool; SECTION_COUNT],
) {
  for i in 0..SECTION_COUNT {
    let next = (i + 1) % SECTION_COUNT;
    if outer[i] {
      triangles.push([first[i], second[i], second[next]]);
    }
    if outer[next] {
      triangles.push([second[next], first[next], first[i]]);
    }
  }
}

/// Extrusion contours are physical tubes, not paint bands on the side faces.
/// Exact-option Office mesh captures at 0.5, 1 and 2 pt select both crease
/// edges and camera silhouettes. Neither authored curve boundaries alone nor
/// a crease-angle test alone selects the silhouette of a rounded terminal.
///
/// Visibility belongs to the caller's actual camera, including perspective;
/// selecting coordinate extrema instead moves the tubes on rounded glyphs.
/// The six-point XY section has a fixed 30-degree phase, independently of the
/// heading-dependent sections used by the front/back contour sweep.
pub(super) fn longitudinal_triangles(
  points: &[Point],
  visible_edges: &[(f32, bool)],
  radius: f32,
  front_z: f32,
  back_z: f32,
) -> Vec<Triangle> {
  if points.len() < 3
    || points.len() != visible_edges.len()
    || !radius.is_finite()
    || radius <= f32::EPSILON
    || !front_z.is_finite()
    || !back_z.is_finite()
    || front_z == back_z
    || points.iter().any(|p| !p.0.is_finite() || !p.1.is_finite())
  {
    return Vec::new();
  }
  let edges = (0..points.len())
    .filter(|&i| visible_edges[i].0 > 1.0e-4)
    .filter_map(|i| {
      direction(points[i], points[(i + 1) % points.len()])
        .map(|tangent| (i, tangent, visible_edges[i].1))
    })
    .collect::<Vec<_>>();
  if edges.len() < 3 {
    return Vec::new();
  }
  let section: [Point; SECTION_COUNT] = std::array::from_fn(|i| {
    let angle =
      std::f32::consts::FRAC_PI_6 + std::f32::consts::TAU * i as f32 / SECTION_COUNT as f32;
    let (sin, cos) = angle.sin_cos();
    (radius * cos, radius * sin)
  });
  let mut triangles = Vec::new();
  for (&(_, incoming, incoming_visible), &(index, outgoing, outgoing_visible)) in
    edges.iter().cycle().skip(edges.len() - 1).zip(&edges)
  {
    let dot = incoming.0 * outgoing.0 + incoming.1 * outgoing.1;
    if dot > LONGITUDINAL_CREASE_COSINE && incoming_visible == outgoing_visible {
      continue;
    }
    let point = points[index];
    let ring = |z| section.map(|offset| [point.0 + offset.0, point.1 + offset.1, z]);
    // The captured longitudinal primitives have two rings and no end caps.
    // They meet the separately constructed front and back contour tubes.
    connect(&mut triangles, &ring(front_z), &ring(back_z));
  }
  triangles
}

fn cap(
  triangles: &mut Vec<Triangle>,
  point: Point,
  tangent: Point,
  sign: f32,
  radius: f32,
  z: f32,
  profile: &[(f32, f32); SECTION_COUNT],
) {
  let normal = (-tangent.1, tangent.0);
  let scale = std::f32::consts::FRAC_1_SQRT_2;
  let small_center = (
    point.0 + sign * radius * scale * tangent.0,
    point.1 + sign * radius * scale * tangent.1,
  );
  let small_profile = profile.map(|(radial, height)| (radial * scale, height * scale));
  let full = ring(point, normal, z, profile);
  let small = ring(small_center, normal, z, &small_profile);
  connect(triangles, &full, &small);
  let pole = [
    point.0 + sign * radius * tangent.0,
    point.1 + sign * radius * tangent.1,
    z,
  ];
  for i in 0..SECTION_COUNT {
    triangles.push([small[i], pole, small[(i + 1) % SECTION_COUNT]]);
  }
}

/// Sweeps the entire closed figure as the captured capped polyline. The two
/// caps meet at the canonical seam; rotating the stored contour must not
/// rotate the section or relocate that seam. Each segment keeps its inner
/// rail: adjacent inner faces may overlap and are resolved by the common
/// depth buffer. Smooth outer rails meet at their tangent intersection.
pub(super) fn triangles(contour: &Static3dTextContour, radius: f32, z: f32) -> Vec<Triangle> {
  if !radius.is_finite() || radius <= f32::EPSILON || !z.is_finite() {
    return Vec::new();
  }
  let points = &contour.points;
  if points.len() < 3 || points.iter().any(|p| !p.0.is_finite() || !p.1.is_finite()) {
    return Vec::new();
  }
  let count = points.len();
  let start = (0..count)
    .min_by(|&a, &b| {
      points[a]
        .1
        .total_cmp(&points[b].1)
        .then(points[a].0.total_cmp(&points[b].0))
    })
    .expect("nonempty contour");
  // Ignore zero-length edges without inventing a direction. Join selection
  // uses the adjacent realized segments, independently of source metadata.
  let edges = (0..count)
    .filter_map(|offset| {
      let index = (start + offset) % count;
      direction(points[index], points[(index + 1) % count]).map(|tangent| (index, tangent))
    })
    .collect::<Vec<_>>();
  if edges.len() < 2 {
    return Vec::new();
  }
  let profile = section(radius, edges[0].1);
  let mut starts = edges
    .iter()
    .map(|&(i, t)| ring(points[i], (-t.1, t.0), z, &profile))
    .collect::<Vec<_>>();
  let mut ends = edges
    .iter()
    .map(|&(i, t)| ring(points[(i + 1) % count], (-t.1, t.0), z, &profile))
    .collect::<Vec<_>>();
  let mut triangles = Vec::with_capacity(edges.len() * SECTION_COUNT * 4);
  for next in 1..edges.len() {
    let previous = next - 1;
    let (index, outgoing) = edges[next];
    let incoming = edges[previous].1;
    let cross = incoming.0 * outgoing.1 - incoming.1 * outgoing.0;
    let dot = incoming.0 * outgoing.0 + incoming.1 * outgoing.1;
    if cross.abs() <= f32::EPSILON && dot > 0.0 {
      continue;
    }
    let first_normal = (-incoming.1, incoming.0);
    let second_normal = (-outgoing.1, outgoing.0);
    let denominator = 1.0 + dot;
    let steps = horizontal_join_steps(dot);
    if steps <= 1 && denominator > f32::EPSILON {
      let miter = (
        (first_normal.0 + second_normal.0) / denominator,
        (first_normal.1 + second_normal.1) / denominator,
      );
      for (i, &(radial, height)) in profile.iter().enumerate() {
        if radial * cross < 0.0 {
          let vertex = [
            points[index].0 + miter.0 * radial,
            points[index].1 + miter.1 * radial,
            z + height,
          ];
          ends[previous][i] = vertex;
          starts[next][i] = vertex;
        }
      }
    } else {
      let turn = cross.atan2(dot);
      // atan2 also supplies a turn direction for an exact reversal, whose
      // cross product is zero. Such a join still needs its outer half-circle.
      let outer = profile.map(|(radial, _)| radial * turn < 0.0);
      let inner = ends[previous];
      let mut prior = ends[previous];
      for step in 1..=steps {
        let angle = turn * step as f32 / steps as f32;
        let (sin, cos) = angle.sin_cos();
        let normal = (
          first_normal.0 * cos - first_normal.1 * sin,
          first_normal.0 * sin + first_normal.1 * cos,
        );
        let rotated = if step == steps {
          starts[next]
        } else {
          ring(points[index], normal, z, &profile)
        };
        let next_ring = std::array::from_fn(|i| if outer[i] { rotated[i] } else { inner[i] });
        connect_outer_join(&mut triangles, &prior, &next_ring, &outer);
        prior = next_ring;
      }
    }
  }
  for (first, second) in starts.iter().zip(&ends) {
    connect(&mut triangles, first, second);
  }
  let first = edges[0];
  let last = edges[edges.len() - 1];
  cap(
    &mut triangles,
    points[first.0],
    first.1,
    -1.0,
    radius,
    z,
    &profile,
  );
  cap(
    &mut triangles,
    points[(last.0 + 1) % count],
    last.1,
    1.0,
    radius,
    z,
    &profile,
  );
  triangles
}

#[cfg(test)]
mod tests {
  use super::*;

  fn contour(points: Vec<Point>) -> Static3dTextContour {
    let count = points.len();
    Static3dTextContour {
      points,
      source_path_range: 0..0,
      incoming_curve_edges: vec![false; count],
      longitudinal_contour_joins: vec![true; count],
      bounds: (0.0, 0.0, 10.0, 10.0),
    }
  }

  fn regular_polygon(count: usize) -> Vec<Point> {
    (0..count)
      .map(|i| {
        let angle = std::f32::consts::TAU * i as f32 / count as f32;
        let (sin, cos) = angle.sin_cos();
        (10.0 * cos, 10.0 * sin)
      })
      .collect()
  }

  #[test]
  fn longitudinal_contour_separates_creases_from_camera_silhouettes() {
    let points = regular_polygon(36);
    let mut visibility = vec![(1.0, true); points.len()];
    assert!(longitudinal_triangles(&points, &visibility, 0.5, 5.0, 0.0).is_empty());
    visibility[18..].fill((1.0, false));
    let mesh = longitudinal_triangles(&points, &visibility, 0.5, 5.0, 0.0);
    assert_eq!(mesh.len(), 2 * SECTION_COUNT * 2);
    for triangle in &mesh {
      for &[x, y, z] in triangle {
        assert!(z == 0.0 || z == 5.0);
        assert!(
          [points[0], points[18]]
            .iter()
            .any(|p| ((x - p.0).hypot(y - p.1) - 0.5).abs() < 1.0e-5)
        );
      }
    }
    // Opposite facing labels preserve the silhouette locations.
    for (_, visible) in &mut visibility {
      *visible = !*visible;
    }
    assert_eq!(
      longitudinal_triangles(&points, &visibility, 0.5, 5.0, 0.0),
      mesh
    );
  }

  #[test]
  fn longitudinal_contour_width_changes_section_not_edge_selection() {
    for count in [23, 25] {
      let points = regular_polygon(count);
      let visibility = vec![(1.0, true); count];
      for radius in [0.25, 0.5, 1.0, 4.0] {
        let mesh = longitudinal_triangles(&points, &visibility, radius, 5.0, 0.0);
        assert_eq!(mesh.len(), if count == 23 { count * 12 } else { 0 });
        for triangle in &mesh {
          for &[x, y, _] in triangle {
            assert!(
              points
                .iter()
                .any(|p| ((x - p.0).hypot(y - p.1) - radius).abs() < 1.0e-5)
            );
          }
        }
      }
    }
  }

  #[test]
  fn longitudinal_contour_rejects_degenerate_or_nonfinite_inputs() {
    let points = regular_polygon(4);
    let visibility = vec![(1.0, true); points.len()];
    for radius in [0.0, -1.0, f32::INFINITY, f32::NAN] {
      assert!(longitudinal_triangles(&points, &visibility, radius, 5.0, 0.0).is_empty());
    }
    assert!(longitudinal_triangles(&points, &visibility, 0.5, 0.0, 0.0).is_empty());
    assert!(longitudinal_triangles(&points, &[], 0.5, 5.0, 0.0).is_empty());
    assert!(longitudinal_triangles(&[(1.0, 1.0); 4], &visibility, 0.5, 5.0, 0.0).is_empty());
  }

  #[test]
  fn contour_mesh_section_is_circular_and_preserves_both_depth_halves() {
    for radius in [0.25, 0.5, 1.0, 4.0] {
      for heading in [-2.7_f32, -0.1, 0.0, 0.049_140_55, 1.8, 3.0] {
        let (sin, cos) = heading.sin_cos();
        let profile = section(radius, (cos, sin));
        for (radial, z) in profile {
          assert!((radial.hypot(z) - radius).abs() < 1.0e-6);
        }
        assert!(profile.iter().any(|p| p.1 > 0.0));
        assert!(profile.iter().any(|p| p.1 < 0.0));
      }
    }
  }

  #[test]
  fn horizontal_join_count_covers_miter_and_all_rounding_boundaries() {
    for (degrees, expected) in [
      (0.0_f32, 0),
      (4.999, 0),
      (5.001, 1),
      (44.922, 1),
      (49.999, 1),
      (50.001, 2),
      (51.665, 2),
      (90.0, 2),
      (94.999, 2),
      (95.001, 3),
      (139.999, 3),
      (140.001, 4),
      (180.0, 4),
    ] {
      for sign in [-1.0, 1.0] {
        assert_eq!(
          horizontal_join_steps((sign * degrees).to_radians().cos()),
          expected,
          "{sign} * {degrees} degrees"
        );
      }
    }
    assert_eq!(horizontal_join_steps(1.0 + f32::EPSILON), 0);
    assert_eq!(horizontal_join_steps(-1.0 - f32::EPSILON), 4);
  }

  #[test]
  fn horizontal_mesh_uses_segment_angles_not_source_crease_flags_or_width() {
    // A seven-sided figure rounds its ~51-degree turns even if every vertex
    // came from flattening. Eight-sided and shallower authored turns miter.
    for count in [3, 4, 7, 8, 12, 36] {
      // An oblique heading keeps section vertices off the bend's zero-radius
      // dividing plane, so each round subdivision has exactly six faces.
      let (sin, cos) = 0.123_f32.sin_cos();
      let mut c = contour(
        regular_polygon(count)
          .into_iter()
          .map(|(x, y)| (x * cos - y * sin, x * sin + y * cos))
          .collect(),
      );
      for radius in [0.25, 0.5, 1.0, 4.0] {
        c.longitudinal_contour_joins.fill(true);
        let expected = triangles(&c, radius, 5.0);
        let steps = horizontal_join_steps((std::f32::consts::TAU / count as f32).cos());
        let join_triangles = if steps <= 1 { 0 } else { steps * SECTION_COUNT };
        assert_eq!(
          expected.len(),
          count * SECTION_COUNT * 2 + (count - 1) * join_triangles + 36
        );
        c.longitudinal_contour_joins.fill(false);
        assert_eq!(triangles(&c, radius, 5.0), expected);
        for (index, crease) in c.longitudinal_contour_joins.iter_mut().enumerate() {
          *crease = index % 2 == 0;
        }
        assert_eq!(triangles(&c, radius, 5.0), expected);
      }
    }
  }

  #[test]
  fn round_join_rotates_only_outer_rails_and_fans_to_fixed_inner_rails() {
    for radius in [0.25, 0.5, 1.0, 4.0] {
      for heading in [-2.7_f32, -0.1, 0.0, 0.049_140_55, 1.8, 3.0] {
        for turn in [-1.9_f32, -std::f32::consts::FRAC_PI_2, -0.3, 0.3, 1.9] {
          let profile = section(radius, (heading.cos(), heading.sin()));
          let normal = (-heading.sin(), heading.cos());
          let first = ring((0.0, 0.0), normal, 5.0, &profile);
          let rotated = ring(
            (0.0, 0.0),
            (-(heading + turn).sin(), (heading + turn).cos()),
            5.0,
            &profile,
          );
          let outer = profile.map(|(radial, _)| radial * turn < 0.0);
          let second = std::array::from_fn(|i| if outer[i] { rotated[i] } else { first[i] });
          let mut mesh = Vec::new();
          connect_outer_join(&mut mesh, &first, &second, &outer);
          assert_eq!(mesh.len(), SECTION_COUNT);
          for i in 0..SECTION_COUNT {
            if outer[i] {
              assert!(mesh.iter().flatten().any(|v| *v == rotated[i]));
            } else {
              assert!(!mesh.iter().flatten().any(|v| *v == rotated[i]));
            }
          }
          // Every emitted face has actual area; collapsed inner quads are
          // absent, not left behind for the rasterizer to discard.
          for [a, b, c] in mesh {
            let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
            let cross = [
              u[1] * v[2] - u[2] * v[1],
              u[2] * v[0] - u[0] * v[2],
              u[0] * v[1] - u[1] * v[0],
            ];
            assert!(cross.iter().any(|x| x.abs() > 1.0e-6));
          }
        }
      }
    }
  }

  #[test]
  fn contour_mesh_cyclic_storage_does_not_move_seam_or_section() {
    let mut c = contour(vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)]);
    let expected = triangles(&c, 0.5, 5.0);
    for _ in 0..c.points.len() {
      c.points.rotate_left(1);
      assert_eq!(triangles(&c, 0.5, 5.0), expected);
    }
    assert!(
      expected
        .iter()
        .flatten()
        .all(|v| v.iter().all(|x| x.is_finite()))
    );
    assert!(
      expected
        .iter()
        .flatten()
        .any(|v| v[0] == -0.5 && v[1] == 0.0 && v[2] == 5.0)
    );
  }

  #[test]
  fn contour_mesh_reversal_keeps_its_outer_half_circle() {
    let c = contour(vec![(0.0, 0.0), (10.0, 0.0), (0.0, 0.0), (0.0, 10.0)]);
    let mesh = triangles(&c, 0.5, 5.0);
    assert!(mesh.iter().flatten().any(|v| {
      (v[0] - 10.5).abs() < 1.0e-6 && v[1].abs() < 1.0e-6 && (v[2] - 5.0).abs() < 1.0e-6
    }));
    assert!(
      mesh
        .iter()
        .flatten()
        .all(|v| v.iter().all(|x| x.is_finite()))
    );
  }

  #[test]
  fn contour_mesh_degenerate_inputs_do_not_invent_faces() {
    let c = contour(vec![(1.0, 1.0); 4]);
    assert!(triangles(&c, 1.0, 0.0).is_empty());
    let c = contour(vec![(0.0, 0.0), (10.0, 0.0), (0.0, 10.0)]);
    for radius in [0.0, -1.0, f32::NAN, f32::INFINITY] {
      assert!(triangles(&c, radius, 0.0).is_empty());
    }
  }
}
