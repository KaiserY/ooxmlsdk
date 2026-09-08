//! Device-realized physical outlines, independent of continuous paint coverage.
//!
//! The starting-tangent parabolic approximation is described by Hain et al.,
//! "Fast, precise flattening of cubic Bezier path and offset curves" (2005),
//! doi:10.1016/j.cag.2005.08.002. Exact-option Word GFX producer observations
//! confirm the forward walk, half-interval step cap and inflection intervals.
//! A symmetric chord-distance test is not equivalent: reversing a curve can
//! change its subdivision and therefore the extrusion's contour creases.

use kurbo::{CubicBez, ParamCurve, PathEl, Point, QuadBez, Vec2};
use smallvec::SmallVec;

const EPSILON: f64 = 1.0e-15;

fn tangent(curve: CubicBez) -> Vec2 {
  [curve.p1, curve.p2, curve.p3]
    .into_iter()
    .map(|p| p - curve.p0)
    .find(|v| v.hypot2() > EPSILON)
    .map_or(Vec2::ZERO, |v| v / v.hypot())
}

fn split(curve: CubicBez, t: f64) -> (CubicBez, CubicBez) {
  // Also used outside [0, 1] when locating an inflection's flat interval.
  let a = curve.p0.lerp(curve.p1, t);
  let b = curve.p1.lerp(curve.p2, t);
  let c = curve.p2.lerp(curve.p3, t);
  let d = a.lerp(b, t);
  let e = b.lerp(c, t);
  let p = d.lerp(e, t);
  (
    CubicBez::new(curve.p0, a, d, p),
    CubicBez::new(p, e, c, curve.p3),
  )
}

fn inflection_interval(curve: CubicBez, tolerance: f64, t: f64) -> Option<(f64, f64)> {
  let (_, tail) = split(curve, t);
  let distance = tangent(tail).cross(tail.p3 - tail.p0).abs();
  if distance <= EPSILON {
    return None;
  }
  let half_width = ((tolerance / distance).cbrt() * (1.0 - t)).abs();
  let start = (t - half_width).max(0.0);
  let end = (t + half_width).min(1.0);
  (start.is_finite() && end.is_finite() && start < end).then_some((start, end))
}

fn ordinary(mut curve: CubicBez, tolerance: f64, emit: &mut impl FnMut(Point)) {
  loop {
    let distance = (3.0 * tangent(curve).cross(curve.p2 - curve.p0)).abs();
    if distance <= EPSILON {
      break;
    }
    let step = 2.0 * (tolerance / distance).sqrt();
    if step >= 1.0 || !step.is_finite() {
      break;
    }
    let (_, tail) = split(curve, step.min(0.5));
    // Floating-point exhaustion must terminate without emitting the same
    // vertex forever. The caller always emits the original exact endpoint.
    if tail == curve || tail.p0 == curve.p3 {
      break;
    }
    emit(tail.p0);
    curve = tail;
  }
}

fn cubic(curve: CubicBez, tolerance: f64, emit: &mut impl FnMut(Point)) {
  let a = curve.p3 - curve.p0 + 3.0 * (curve.p1 - curve.p2);
  let b = 3.0 * ((curve.p2 - curve.p1) - (curve.p1 - curve.p0));
  let c = 3.0 * (curve.p1 - curve.p0);
  let denominator = b.cross(a);
  let mut intervals = SmallVec::<[(f64, f64); 2]>::new();
  let mut cusp = None;
  if denominator.abs() > EPSILON {
    let center = -0.5 * c.cross(a) / denominator;
    let discriminant = center * center - c.cross(b) / (3.0 * denominator);
    if discriminant >= 0.0 {
      let radius = discriminant.sqrt();
      for t in [center - radius, center + radius] {
        if let Some(interval) = inflection_interval(curve, tolerance, t) {
          intervals.push(interval);
        }
      }
      intervals.sort_by(|a, b| a.0.total_cmp(&b.0));
      if intervals.len() == 2 && intervals[0].1 > intervals[1].0 {
        // Crossing flat intervals enclose a cusp; preserve its vertex.
        // If one interval contains the other, their union alone suffices.
        if intervals[0].1 < intervals[1].1 && center > 0.0 && center < 1.0 {
          cusp = Some(center);
        }
        intervals[0].1 = intervals[0].1.max(intervals[1].1);
        intervals.pop();
      }
    }
  }
  let mut start = 0.0;
  for (lo, hi) in intervals {
    if lo > start {
      let segment = curve.subsegment(start..lo);
      ordinary(segment, tolerance, emit);
      emit(segment.p3);
    }
    if let Some(t) = cusp.filter(|t| *t > lo && *t < hi) {
      emit(curve.eval(t));
    }
    if hi < 1.0 {
      emit(curve.eval(hi));
    }
    start = hi;
  }
  if start < 1.0 {
    ordinary(curve.subsegment(start..1.0), tolerance, emit);
  }
  emit(curve.p3);
}

/// Input and tolerance use the same coordinates. Normalize them to physical
/// inches so numerical degeneracy has the same meaning at every source DPI.
pub(super) fn flatten(
  elements: impl IntoIterator<Item = PathEl>,
  tolerance_px: f64,
  pixels_per_inch: f64,
  mut emit: impl FnMut(PathEl),
) {
  assert!(tolerance_px.is_finite() && tolerance_px > 0.0);
  assert!(pixels_per_inch.is_finite() && pixels_per_inch > 0.0);
  let tolerance = tolerance_px / pixels_per_inch;
  let normalize = |p: Point| Point::new(p.x / pixels_per_inch, p.y / pixels_per_inch);
  let mut current = None;
  for element in elements {
    match element {
      PathEl::MoveTo(p) | PathEl::LineTo(p) => {
        current = Some(normalize(p));
        emit(element);
      }
      PathEl::QuadTo(control, end) => {
        if let Some(start) = current {
          let curve = QuadBez::new(start, normalize(control), normalize(end)).raise();
          cubic(curve, tolerance, &mut |p| {
            emit(PathEl::LineTo(Point::new(
              p.x * pixels_per_inch,
              p.y * pixels_per_inch,
            )))
          });
        }
        current = Some(normalize(end));
      }
      PathEl::CurveTo(a, b, end) => {
        if let Some(start) = current {
          let curve = CubicBez::new(start, normalize(a), normalize(b), normalize(end));
          cubic(curve, tolerance, &mut |p| {
            emit(PathEl::LineTo(Point::new(
              p.x * pixels_per_inch,
              p.y * pixels_per_inch,
            )))
          });
        }
        current = Some(normalize(end));
      }
      PathEl::ClosePath => {
        emit(element);
        current = None;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn points(curve: CubicBez, tolerance: f64) -> Vec<Point> {
    let mut result = Vec::new();
    cubic(curve, tolerance, &mut |p| result.push(p));
    result
  }

  #[test]
  fn forward_tangent_is_not_a_symmetric_chord_test() {
    let q = QuadBez::new((0.0, 0.0), (2.0, 0.0), (2.0, 1.0));
    assert_eq!(points(q.raise(), 0.3).len(), 1);
    let reversed = QuadBez::new(q.p2, q.p1, q.p0);
    let points = points(reversed.raise(), 0.3);
    assert_eq!(points.len(), 2);
    assert!(points[0].distance(reversed.eval(0.5)) < 1e-12);
  }

  #[test]
  fn density_changes_segmentation_without_moving_source_endpoints() {
    let curve = QuadBez::new((0.0, 0.0), (2.0, 0.0), (2.0, 1.0)).raise();
    let coarse = points(curve, 0.1);
    let fine = points(curve, 0.01);
    assert!(fine.len() > coarse.len());
    assert_eq!(coarse.last(), Some(&curve.p3));
    assert_eq!(fine.last(), Some(&curve.p3));
  }

  #[test]
  fn degenerate_controls_terminate_and_retain_the_endpoint() {
    for curve in [
      CubicBez::new((0., 0.), (0., 0.), (0., 0.), (0., 0.)),
      CubicBez::new((0., 0.), (0., 0.), (0., 0.), (1., 1.)),
      CubicBez::new((0., 0.), (0., 0.), (1., 0.), (1., 1.)),
    ] {
      let result = points(curve, 0.01);
      assert!(!result.is_empty() && result.len() < 100);
      assert_eq!(result.last(), Some(&curve.p3));
      assert!(result.iter().all(|p| p.is_finite()));
    }
  }

  #[test]
  fn inflection_and_loop_intervals_keep_both_sides_of_the_curve() {
    for curve in [
      CubicBez::new((0., 0.), (1., 2.), (2., -2.), (3., 0.)),
      CubicBez::new((0., 0.), (2., 3.), (-2., 3.), (0., 0.)),
    ] {
      let result = points(curve, 0.001);
      assert!(result.len() > 4 && result.len() < 1000);
      assert!(result.iter().any(|p| p.y > 0.1));
      assert_eq!(result.last(), Some(&curve.p3));
    }
  }
}
