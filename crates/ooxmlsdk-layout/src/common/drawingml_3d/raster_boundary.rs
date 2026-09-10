//! Geometric side contacts beside, rather than reconstructed from, a source A8 mask.

use std::collections::BTreeSet;

use kurbo::{
  BezPath, Line, ParamCurve, ParamCurveDeriv, ParamCurveNearest, PathEl, PathSeg, Point, Rect,
  Shape,
};
use tiny_skia::FillRule;

use super::raster_extrusion::SourceEdge;

#[derive(Default)]
pub(crate) struct RasterSourceBoundary {
  regions: Vec<(BezPath, FillRule)>,
  segments: Vec<(PathSeg, Rect)>,
}

/// Keep the side's position and normal on the same exposed vector segment.
/// Source-mask coverage still owns opacity, not the position of this surface.
#[cfg(test)]
struct BoundaryContact {
  pub(super) normal: [f32; 2],
  segment: PathSeg,
}

#[cfg(test)]
impl BoundaryContact {
  pub(super) fn project(&self, point: (f32, f32)) -> (f32, f32) {
    let point = Point::new(f64::from(point.0), f64::from(point.1));
    let nearest = self.segment.nearest(point, 1.0e-5);
    let projected = self.segment.eval(nearest.t);
    (projected.x as f32, projected.y as f32)
  }
}

impl RasterSourceBoundary {
  pub(crate) fn push(&mut self, path: BezPath, rule: FillRule) {
    // The source fill rasterizer implicitly closes every subpath. Kurbo's
    // segment iterator does not; retain the same closing edges for normals
    // and winding, without changing the independently widened open stroke.
    let mut closed = BezPath::new();
    let mut open = false;
    for element in path {
      match element {
        PathEl::MoveTo(_) => {
          if open {
            closed.close_path();
          }
          open = true;
        }
        PathEl::ClosePath => open = false,
        _ => {}
      }
      closed.push(element);
    }
    if open {
      closed.close_path();
    }
    let path = closed;
    self.segments.extend(
      path
        .segments()
        .map(|segment| (segment, segment.bounding_box())),
    );
    self.regions.push((path, rule));
  }

  pub(crate) fn is_empty(&self) -> bool {
    self.segments.is_empty()
  }

  fn contains(&self, point: Point) -> bool {
    self.regions.iter().any(|(path, rule)| {
      let winding = path.winding(point);
      match rule {
        FillRule::Winding => winding != 0,
        FillRule::EvenOdd => winding.rem_euclid(2) != 0,
      }
    })
  }

  /// Split the actual union boundary at region intersections and source-cell
  /// boundaries. A neighboring antialiased pixel can contain paint without
  /// covering this geometric edge; zero-alpha neighbor tests cannot supply it.
  pub(super) fn source_edges(&self, width: u32, height: u32) -> Vec<SourceEdge> {
    const FLATNESS: f64 = 0.001;
    let flattened = self
      .segments
      .iter()
      .map(|&(segment, _)| {
        let end = match segment {
          PathSeg::Line(line) => PathEl::LineTo(line.p1),
          PathSeg::Quad(quad) => PathEl::QuadTo(quad.p1, quad.p2),
          PathSeg::Cubic(cubic) => PathEl::CurveTo(cubic.p1, cubic.p2, cubic.p3),
        };
        let mut path = BezPath::new();
        kurbo::flatten([PathEl::MoveTo(segment.start()), end], FLATNESS, |el| {
          path.push(el)
        });
        path
          .segments()
          .filter_map(|segment| match segment {
            PathSeg::Line(line) => Some(line),
            _ => None,
          })
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
    let mut result = Vec::new();
    let mut emitted = BTreeSet::new();
    for (index, &(segment, bounds)) in self.segments.iter().enumerate() {
      if bounds.x1 < 0.0
        || bounds.y1 < 0.0
        || bounds.x0 > f64::from(width)
        || bounds.y0 > f64::from(height)
      {
        continue;
      }
      let mut parameters = vec![0.0, 1.0];
      for (other_index, &(_, other_bounds)) in self.segments.iter().enumerate() {
        if index == other_index
          || bounds.x1 < other_bounds.x0
          || bounds.x0 > other_bounds.x1
          || bounds.y1 < other_bounds.y0
          || bounds.y0 > other_bounds.y1
        {
          continue;
        }
        for &line in &flattened[other_index] {
          parameters.extend(
            segment
              .intersect_line(line)
              .into_iter()
              .map(|hit| hit.segment_t.clamp(0.0, 1.0)),
          );
          // Coincident edges have no isolated line intersection. Their finite
          // endpoints still partition ownership (e.g. overlapping rectangles).
          for endpoint in [line.p0, line.p1] {
            let nearest = segment.nearest(endpoint, 1.0e-5);
            if nearest.distance_sq < 1.0e-12 {
              parameters.push(nearest.t);
            }
          }
        }
      }
      for x in (bounds.x0.ceil().max(0.0) as u32)
        ..=(bounds.x1.floor().min(f64::from(width)).max(0.0) as u32)
      {
        let line = Line::new(
          (f64::from(x), bounds.y0 - 1.0),
          (f64::from(x), bounds.y1 + 1.0),
        );
        parameters.extend(
          segment
            .intersect_line(line)
            .into_iter()
            .map(|hit| hit.segment_t.clamp(0.0, 1.0)),
        );
      }
      for y in (bounds.y0.ceil().max(0.0) as u32)
        ..=(bounds.y1.floor().min(f64::from(height)).max(0.0) as u32)
      {
        let line = Line::new(
          (bounds.x0 - 1.0, f64::from(y)),
          (bounds.x1 + 1.0, f64::from(y)),
        );
        parameters.extend(
          segment
            .intersect_line(line)
            .into_iter()
            .map(|hit| hit.segment_t.clamp(0.0, 1.0)),
        );
      }
      // Preserve curved geometry inside a cell too; the raster grid is not a
      // curve-flattening tolerance. All submitted points remain on the curve.
      parameters.extend(
        flattened[index]
          .iter()
          .map(|line| segment.nearest(line.p1, 1.0e-5).t),
      );
      parameters.sort_unstable_by(f64::total_cmp);
      parameters.dedup_by(|a, b| (*a - *b).abs() < 1.0e-10);
      for pair in parameters.windows(2) {
        let middle = (pair[0] + pair[1]) * 0.5;
        let Some(normal) = self.outward_normal(segment, middle) else {
          continue;
        };
        let point = segment.eval(middle) - normal * 1.0e-5;
        let (x, y) = (point.x.floor(), point.y.floor());
        if x < 0.0 || y < 0.0 || x >= f64::from(width) || y >= f64::from(height) {
          continue;
        }
        let first = segment.eval(pair[0]);
        let second = segment.eval(pair[1]);
        if first.distance_squared(second) < 1.0e-12 {
          continue;
        }
        let key = |p: Point| ((p.x * 1.0e6).round() as i64, (p.y * 1.0e6).round() as i64);
        let (a, b) = (key(first), key(second));
        if !emitted.insert(if a < b { (a, b) } else { (b, a) }) {
          continue;
        }
        result.push(SourceEdge {
          first: (first.x as f32, first.y as f32),
          second: (second.x as f32, second.y as f32),
          normal: [normal.x as f32, normal.y as f32],
          facing_hint: [normal.x as f32, normal.y as f32],
          source_pixel: [x as u32, y as u32],
        });
      }
    }
    result
  }

  fn outward_normal(&self, segment: PathSeg, t: f64) -> Option<kurbo::Vec2> {
    let tangent = match segment {
      PathSeg::Line(line) => line.p1 - line.p0,
      PathSeg::Quad(quad) => quad.deriv().eval(t).to_vec2(),
      PathSeg::Cubic(cubic) => cubic.deriv().eval(t).to_vec2(),
    };
    let length = tangent.hypot();
    if !length.is_finite() || length <= f64::EPSILON {
      return None;
    }
    let tangent = tangent / length;
    let normal = kurbo::Vec2::new(tangent.y, -tangent.x);
    let mut probe = segment.eval(t);
    const EPSILON: f64 = 1.0e-5;
    if t <= 1.0e-8 {
      probe += tangent * (4.0 * EPSILON);
    } else if t >= 1.0 - 1.0e-8 {
      probe -= tangent * (4.0 * EPSILON);
    }
    match (
      self.contains(probe + normal * EPSILON),
      self.contains(probe - normal * EPSILON),
    ) {
      (false, true) => Some(normal),
      (true, false) => Some(-normal),
      _ => None,
    }
  }

  #[cfg(test)]
  fn contact_at(&self, point: (f32, f32), edge_hint: [f32; 2]) -> Option<BoundaryContact> {
    let point = Point::new(f64::from(point.0), f64::from(point.1));
    // A vector edge contributing to this source pixel must intersect its
    // unit square. More distant alpha edges belong to bitmap/paint content,
    // not this vector boundary, and keep the raster fallback.
    let mut best_distance = 0.5 + 1.0e-6;
    let mut best_alignment = f64::NEG_INFINITY;
    let mut result = None;
    for &(segment, bounds) in &self.segments {
      let closest_box = Point::new(
        point.x.clamp(bounds.x0, bounds.x1),
        point.y.clamp(bounds.y0, bounds.y1),
      );
      if point.distance_squared(closest_box) > best_distance + 1.0e-8 {
        continue;
      }
      let nearest = segment.nearest(point, 1.0e-5);
      if nearest.distance_sq > best_distance + 1.0e-8 {
        continue;
      }
      let Some(normal) = self.outward_normal(segment, nearest.t) else {
        continue;
      };
      let alignment = normal.x * f64::from(edge_hint[0]) + normal.y * f64::from(edge_hint[1]);
      // A corner pixel can expose more than one side. Its closest vector
      // segment need not own the side being submitted: assigning the top
      // edge to an exposed left edge can incorrectly cull that left face.
      // Require the segment to face this exposure before comparing distance.
      // A diagonal/curved boundary can still own multiple cardinal exposures;
      // retain its actual normal instead of snapping it to the raster grid.
      if alignment <= 0.0 {
        continue;
      }
      if nearest.distance_sq < best_distance - 1.0e-8 || alignment > best_alignment {
        best_distance = nearest.distance_sq;
        best_alignment = alignment;
        result = Some(BoundaryContact {
          normal: [normal.x as f32, normal.y as f32],
          segment,
        });
      }
    }
    result
  }

  #[cfg(test)]
  pub(crate) fn normal_at(&self, point: (f32, f32), edge_hint: [f32; 2]) -> Option<[f32; 2]> {
    self
      .contact_at(point, edge_hint)
      .map(|contact| contact.normal)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn union_boundary_keeps_the_terminal_fragment_inside_a_shared_paint_cell() {
    let vertical = Rect::new(5.25, 0.0, 10.0, 10.0).to_path(0.001);
    let mut boundary = RasterSourceBoundary::default();
    boundary.push(vertical.clone(), FillRule::Winding);
    boundary.push(
      Rect::new(0.0, 9.3, 10.0, 14.0).to_path(0.001),
      FillRule::Winding,
    );
    // A duplicate primitive must not acquire another coincident side surface.
    boundary.push(vertical, FillRule::Winding);
    let edges = boundary.source_edges(20, 20);
    let terminal = edges
      .iter()
      .find(|edge| edge.source_pixel == [5, 9] && edge.normal == [-1.0, 0.0])
      .unwrap();
    assert!((terminal.first.1.max(terminal.second.1) - 9.3).abs() < 1.0e-5);
    assert_eq!(terminal.first.1.min(terminal.second.1), 9.0);
    let perimeter: f32 = edges
      .iter()
      .map(|edge| (edge.second.0 - edge.first.0).hypot(edge.second.1 - edge.first.1))
      .sum();
    assert!((perimeter - 48.0).abs() < 1.0e-4, "{perimeter}");
    assert!(edges.iter().all(|edge| !(edge.first.0 == 5.25
      && edge.second.0 == 5.25
      && edge.first.1.min(edge.second.1) >= 9.3)));
  }

  #[test]
  fn union_boundary_retains_counters_and_removes_covered_counter_edges() {
    let mut path = Rect::new(0.0, 0.0, 20.0, 20.0).to_path(0.001);
    path.extend(Rect::new(5.0, 5.0, 15.0, 15.0).to_path(0.001));
    let mut boundary = RasterSourceBoundary::default();
    boundary.push(path, FillRule::EvenOdd);
    let edges = boundary.source_edges(24, 24);
    assert!(
      edges
        .iter()
        .any(|edge| edge.first.0 == 5.0 && edge.second.0 == 5.0 && edge.normal == [1.0, 0.0])
    );
    boundary.push(
      Rect::new(4.0, 4.0, 16.0, 16.0).to_path(0.001),
      FillRule::Winding,
    );
    let edges = boundary.source_edges(24, 24);
    assert_eq!(edges.len(), 80);
    assert!(edges.iter().all(|edge| edge.first.0 == 0.0
      || edge.first.0 == 20.0
      || edge.first.1 == 0.0
      || edge.first.1 == 20.0));
  }

  #[test]
  fn curved_union_boundary_keeps_closed_geometry_and_outward_normals() {
    let mut boundary = RasterSourceBoundary::default();
    boundary.push(
      kurbo::Circle::new((10.0, 10.0), 5.0).to_path(0.001),
      FillRule::Winding,
    );
    let edges = boundary.source_edges(24, 24);
    let perimeter: f32 = edges
      .iter()
      .map(|edge| (edge.second.0 - edge.first.0).hypot(edge.second.1 - edge.first.1))
      .sum();
    assert!(
      (perimeter - 10.0 * std::f32::consts::PI).abs() < 0.01,
      "{perimeter}"
    );
    for edge in &edges {
      let middle = (
        (edge.first.0 + edge.second.0) / 2.0 - 10.0,
        (edge.first.1 + edge.second.1) / 2.0 - 10.0,
      );
      assert!(middle.0 * edge.normal[0] + middle.1 * edge.normal[1] > 4.99);
      assert!(
        edges
          .iter()
          .any(|next| (next.first.0 - edge.second.0).hypot(next.first.1 - edge.second.1) < 1.0e-4)
      );
    }
  }

  #[test]
  fn intersecting_curves_partition_the_external_union() {
    let mut boundary = RasterSourceBoundary::default();
    for x in [10.0, 14.0] {
      boundary.push(
        kurbo::Circle::new((x, 10.0), 5.0).to_path(0.001),
        FillRule::Winding,
      );
    }
    let edges = boundary.source_edges(24, 24);
    let perimeter: f32 = edges
      .iter()
      .map(|edge| (edge.second.0 - edge.first.0).hypot(edge.second.1 - edge.first.1))
      .sum();
    let expected = 20.0 * (std::f32::consts::PI - 0.4_f32.acos());
    assert!(
      (perimeter - expected).abs() < 0.02,
      "{perimeter} != {expected}"
    );
    for edge in &edges {
      // Intersection against the other curve is approximated at 0.001 px;
      // demand closure at that geometric accuracy, not at image tolerance.
      assert!(
        edges
          .iter()
          .any(|next| (next.first.0 - edge.second.0).hypot(next.first.1 - edge.second.1) < 0.002)
      );
    }
  }

  #[test]
  fn vector_contact_keeps_fractional_position_and_finite_endpoints() {
    let mut boundary = RasterSourceBoundary::default();
    boundary.push(
      Rect::new(0.25, 0.3, 10.75, 9.7).to_path(0.001),
      FillRule::Winding,
    );
    let upper = boundary.contact_at((0.5, 0.5), [-1.0, 0.0]).unwrap();
    assert_eq!(upper.normal, [-1.0, 0.0]);
    assert_eq!(upper.project((0.0, 0.0)), (0.25, 0.3));
    assert_eq!(upper.project((0.0, 1.0)), (0.25, 1.0));
    let next = boundary.contact_at((0.5, 1.5), [-1.0, 0.0]).unwrap();
    assert_eq!(next.project((0.0, 1.0)), upper.project((0.0, 1.0)));
    let lower = boundary.contact_at((0.5, 9.5), [-1.0, 0.0]).unwrap();
    assert_eq!(lower.project((0.0, 10.0)), (0.25, 9.7));
    // The adjacent horizontal exposure meets the same finite vector corner.
    let top = boundary.contact_at((0.5, 0.5), [0.0, -1.0]).unwrap();
    assert_eq!(top.project((0.0, 0.0)), upper.project((0.0, 0.0)));
  }

  #[test]
  fn curved_contact_projects_shared_vertices_consistently() {
    let mut boundary = RasterSourceBoundary::default();
    boundary.push(
      kurbo::Circle::new((10.0, 10.0), 5.0).to_path(0.001),
      FillRule::Winding,
    );
    let upper = boundary.contact_at((14.5, 9.5), [1.0, 0.0]).unwrap();
    let lower = boundary.contact_at((14.5, 10.5), [1.0, 0.0]).unwrap();
    assert_eq!(upper.project((15.0, 10.0)), lower.project((15.0, 10.0)));
    for contact in [upper, lower] {
      let point = contact.project((15.0, 9.5));
      assert!(((point.0 - 10.0).hypot(point.1 - 10.0) - 5.0).abs() < 0.002);
    }
  }

  #[test]
  fn source_boundary_closes_each_filled_subpath() {
    let mut path = BezPath::new();
    for x in [0.0, 20.0] {
      path.move_to((x, 0.0));
      path.line_to((x + 10.0, 0.0));
      path.line_to((x + 10.0, 10.0));
      path.line_to((x, 10.0));
    }
    let mut boundary = RasterSourceBoundary::default();
    boundary.push(path, FillRule::EvenOdd);
    for x in [0.0, 20.0] {
      assert_eq!(
        boundary.normal_at((x - 0.2, 5.0), [-1.0, 0.0]),
        Some([-1.0, 0.0])
      );
    }
  }

  #[test]
  fn source_boundary_retains_straight_normals_at_sharp_corners() {
    let mut boundary = RasterSourceBoundary::default();
    boundary.push(
      Rect::new(0.0, 0.0, 10.0, 10.0).to_path(0.001),
      FillRule::EvenOdd,
    );
    for y in [-0.2, 0.1, 0.5, 1.5, 8.5, 9.9, 10.2] {
      assert_eq!(
        boundary.normal_at((-0.2, y), [-1.0, 0.0]),
        Some([-1.0, 0.0])
      );
    }
    assert_eq!(
      boundary.normal_at((-0.2, -0.2), [0.0, -1.0]),
      Some([0.0, -1.0])
    );
    assert_eq!(boundary.normal_at((5.0, 5.0), [-1.0, 0.0]), None);
  }

  #[test]
  fn corner_cell_keeps_each_exposed_edge_owner() {
    let mut boundary = RasterSourceBoundary::default();
    boundary.push(
      Rect::new(0.0, 0.0, 10.0, 10.0).to_path(0.001),
      FillRule::Winding,
    );
    // Both edges contribute to a corner cell. Being nearer the horizontal
    // edge must not assign its normal to the independently exposed left face.
    for (point, horizontal, vertical) in [
      ((0.5, 0.4), [-1.0, 0.0], [0.0, -1.0]),
      ((0.4, 0.5), [-1.0, 0.0], [0.0, -1.0]),
      ((9.5, 9.6), [1.0, 0.0], [0.0, 1.0]),
      ((9.6, 9.5), [1.0, 0.0], [0.0, 1.0]),
    ] {
      assert_eq!(boundary.normal_at(point, horizontal), Some(horizontal));
      assert_eq!(boundary.normal_at(point, vertical), Some(vertical));
    }
  }

  #[test]
  fn diagonal_boundary_owns_both_cardinal_exposures() {
    let mut triangle = BezPath::new();
    triangle.move_to((0.0, 0.0));
    triangle.line_to((10.0, 0.0));
    triangle.line_to((0.0, 10.0));
    triangle.close_path();
    let mut boundary = RasterSourceBoundary::default();
    boundary.push(triangle, FillRule::Winding);
    for exposure in [[1.0, 0.0], [0.0, 1.0]] {
      let normal = boundary.normal_at((5.1, 5.1), exposure).unwrap();
      assert!((normal[0] - std::f32::consts::FRAC_1_SQRT_2).abs() < 1.0e-6);
      assert!((normal[1] - std::f32::consts::FRAC_1_SQRT_2).abs() < 1.0e-6);
    }
  }

  #[test]
  fn source_boundary_respects_curves_counters_and_covered_edges() {
    let mut circle = RasterSourceBoundary::default();
    circle.push(
      kurbo::Circle::new((10.0, 10.0), 5.0).to_path(0.001),
      FillRule::Winding,
    );
    let normal = circle.normal_at((15.2, 10.0), [1.0, 0.0]).unwrap();
    assert!((normal[0] - 1.0).abs() < 0.001 && normal[1].abs() < 0.001);
    let mut path = Rect::new(0.0, 0.0, 20.0, 20.0).to_path(0.001);
    path.extend(Rect::new(5.0, 5.0, 15.0, 15.0).to_path(0.001));
    let mut boundary = RasterSourceBoundary::default();
    boundary.push(path, FillRule::EvenOdd);
    assert_eq!(
      boundary.normal_at((4.8, 10.0), [1.0, 0.0]),
      Some([1.0, 0.0])
    );
    boundary.push(
      Rect::new(4.0, 4.0, 16.0, 16.0).to_path(0.001),
      FillRule::EvenOdd,
    );
    assert_eq!(boundary.normal_at((4.8, 10.0), [1.0, 0.0]), None);
  }
}
