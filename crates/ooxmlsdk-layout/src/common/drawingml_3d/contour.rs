//! Continuous locations for longitudinal text-contour bands.
//!
//! The source path and the rendering polygon have separate responsibilities:
//! source curvature locates a rounded terminal; the existing polygon carries
//! its material boundary without changing silhouette, normals or depth.

use kurbo::{ParamCurve, ParamCurveArclen, ParamCurveNearest, PathSeg, Point, Vec2};

use super::{Static3dTextContour, Static3dTextGeometry};

type EdgeBands = Vec<Vec<(f32, f32)>>;

const WORD_TERMINAL_CURVATURE_RADIUS_PT: f64 = 0.732;

fn polynomial_value(coefficients: &[f64], t: f64) -> f64 {
  coefficients
    .iter()
    .rev()
    .fold(0.0, |value, &c| value * t + c)
}

/// Isolate real roots on a closed unit interval using the derivative's
/// critical points. Unlike a fixed parameter grid, this also retains narrow
/// curvature intervals and repeated roots between two samples.
fn polynomial_unit_roots(coefficients: &[f64]) -> Vec<f64> {
  let scale = coefficients.iter().map(|v| v.abs()).fold(0.0_f64, f64::max);
  if !scale.is_finite() || scale == 0.0 {
    return Vec::new();
  }
  let mut coefficients = coefficients.iter().map(|v| v / scale).collect::<Vec<_>>();
  while coefficients.len() > 1 && coefficients.last().is_some_and(|v| v.abs() <= 1.0e-15) {
    coefficients.pop();
  }
  if coefficients.len() == 1 {
    return Vec::new();
  }
  if coefficients.len() == 2 {
    let root = -coefficients[0] / coefficients[1];
    return (0.0..=1.0)
      .contains(&root)
      .then_some(root)
      .into_iter()
      .collect();
  }
  let derivative = coefficients
    .iter()
    .enumerate()
    .skip(1)
    .map(|(i, c)| i as f64 * c)
    .collect::<Vec<_>>();
  let mut divisions = polynomial_unit_roots(&derivative);
  divisions.extend([0.0, 1.0]);
  divisions.sort_by(f64::total_cmp);
  divisions.dedup_by(|a, b| (*a - *b).abs() < 1.0e-12);
  let mut roots = divisions
    .iter()
    .copied()
    .filter(|&t| polynomial_value(&coefficients, t).abs() <= 1.0e-12)
    .collect::<Vec<_>>();
  for pair in divisions.windows(2) {
    let (mut lo, mut hi) = (pair[0], pair[1]);
    let mut low_value = polynomial_value(&coefficients, lo);
    let high_value = polynomial_value(&coefficients, hi);
    if low_value == 0.0
      || high_value == 0.0
      || low_value.is_sign_positive() == high_value.is_sign_positive()
    {
      continue;
    }
    for _ in 0..48 {
      let middle = (lo + hi) * 0.5;
      let value = polynomial_value(&coefficients, middle);
      if value.is_sign_positive() == low_value.is_sign_positive() {
        lo = middle;
        low_value = value;
      } else {
        hi = middle;
      }
    }
    roots.push((lo + hi) * 0.5);
  }
  roots.sort_by(f64::total_cmp);
  roots.dedup_by(|a, b| (*a - *b).abs() < 1.0e-10);
  roots
}

fn polynomial_product(first: &[f64], second: &[f64]) -> Vec<f64> {
  let mut product = vec![0.0; first.len() + second.len() - 1];
  for (i, a) in first.iter().enumerate() {
    for (j, b) in second.iter().enumerate() {
      product[i + j] += a * b;
    }
  }
  product
}

/// Parameter intervals with positive curvature at least 1/radius. For a
/// cubic, (cross(d, dd) * radius)^2 - dot(d, d)^3 has degree at most twelve.
/// Its roots partition the threshold changes; cross(d, dd) partitions the
/// sign changes so squaring never admits a concave curve.
fn convex_curvature_intervals(segment: PathSeg, orientation: f64, radius: f64) -> Vec<(f64, f64)> {
  let (a, b, c) = match segment {
    PathSeg::Line(_) => return Vec::new(),
    PathSeg::Quad(q) => (
      (q.p1 - q.p0) * 2.0,
      (q.p2.to_vec2() - q.p1.to_vec2() * 2.0 + q.p0.to_vec2()) * 2.0,
      Vec2::ZERO,
    ),
    PathSeg::Cubic(q) => (
      (q.p1 - q.p0) * 3.0,
      (q.p2.to_vec2() - q.p1.to_vec2() * 2.0 + q.p0.to_vec2()) * 6.0,
      (q.p3.to_vec2() - q.p2.to_vec2() * 3.0 + q.p1.to_vec2() * 3.0 - q.p0.to_vec2()) * 3.0,
    ),
  };
  let cross = [
    a.cross(b) * orientation,
    2.0 * a.cross(c) * orientation,
    b.cross(c) * orientation,
  ];
  let speed_squared = [
    a.dot(a),
    2.0 * a.dot(b),
    b.dot(b) + 2.0 * a.dot(c),
    2.0 * b.dot(c),
    c.dot(c),
  ];
  let mut threshold = polynomial_product(
    &polynomial_product(&speed_squared, &speed_squared),
    &speed_squared,
  );
  for value in &mut threshold {
    *value = -*value;
  }
  for (value, cross_squared) in threshold.iter_mut().zip(polynomial_product(&cross, &cross)) {
    *value += cross_squared * radius * radius;
  }
  let mut divisions = polynomial_unit_roots(&threshold);
  divisions.extend(polynomial_unit_roots(&cross));
  divisions.extend([0.0, 1.0]);
  divisions.sort_by(f64::total_cmp);
  divisions.dedup_by(|a, b| (*a - *b).abs() < 1.0e-10);
  divisions
    .windows(2)
    .filter_map(|pair| {
      (curvature(segment, (pair[0] + pair[1]) * 0.5, orientation) >= 1.0 / radius)
        .then_some((pair[0], pair[1]))
    })
    .collect()
}

fn segment_partial_length(segment: PathSeg, t: f64) -> f64 {
  if t <= 0.0 {
    0.0
  } else if t >= 1.0 {
    segment.arclen(1.0e-5)
  } else {
    segment.subsegment(0.0..t).arclen(1.0e-5)
  }
}

fn continuously_convex(segment: PathSeg, orientation: f64) -> bool {
  if matches!(segment, PathSeg::Line(_)) {
    return false;
  }
  // cross(d, dd) is at most quadratic even for a cubic. Check its actual
  // minimum, not a sampling grid which could skip a narrow inflection.
  let cross = |t| {
    let (d, dd) = derivatives(segment, t);
    orientation * d.cross(dd)
  };
  let start = cross(0.0);
  let middle = cross(0.5);
  let end = cross(1.0);
  if start <= 0.0 || end <= 0.0 || ![start, middle, end].into_iter().all(f64::is_finite) {
    return false;
  }
  let quadratic = 2.0 * (start + end - 2.0 * middle);
  let linear = end - start - quadratic;
  let minimum = -linear / (2.0 * quadratic);
  quadratic <= 0.0 || !(0.0..1.0).contains(&minimum) || cross(minimum) > 0.0
}

/// Classify whole convex source arcs, independently of their rendering
/// subdivisions. A chain containing a rounded shoulder is not a smooth wall.
fn smooth_convex_segments(segments: &[PathSeg], orientation: f64, radius: f64) -> Vec<bool> {
  let convex = segments
    .iter()
    .map(|&segment| continuously_convex(segment, orientation))
    .collect::<Vec<_>>();
  let mut selected = segments
    .iter()
    .zip(&convex)
    .map(|(&segment, &convex)| {
      convex
        && curvature(segment, 0.0, orientation) < 1.0 / radius
        && curvature(segment, 1.0, orientation) < 1.0 / radius
        && convex_curvature_intervals(segment, orientation, radius).is_empty()
    })
    .collect::<Vec<_>>();
  let count = segments.len();
  let Some(first_barrier) = convex.iter().position(|&value| !value) else {
    if selected.iter().any(|&value| !value) {
      selected.fill(false);
    }
    return selected;
  };
  let mut chain = Vec::new();
  for offset in 1..=count {
    let index = (first_barrier + offset) % count;
    if convex[index] {
      chain.push(index);
    } else {
      // A curve joining two tangent straight sides is their rounded corner,
      // not an unbounded smooth wall. It still owns the side contour even
      // when its radius is larger than the terminal curvature threshold.
      // Native Office e's upper return is the stopping counterexample.
      let rounded_corner = chain
        .first()
        .zip(chain.last())
        .is_some_and(|(&first, &last)| {
          let before = segments[(first + count - 1) % count];
          let after = segments[(last + 1) % count];
          let tangent_join = |first: PathSeg, second: PathSeg| {
            let a = derivatives(first, 1.0).0;
            let b = derivatives(second, 0.0).0;
            let lengths = a.hypot() * b.hypot();
            lengths > 1.0e-12 && a.dot(b) > 0.984_807_753_012_208 * lengths
          };
          matches!(before, PathSeg::Line(_))
            && matches!(after, PathSeg::Line(_))
            && tangent_join(before, segments[first])
            && tangent_join(segments[last], after)
        });
      if rounded_corner || chain.iter().any(|&index| !selected[index]) {
        for &index in &chain {
          selected[index] = false;
        }
      }
      chain.clear();
    }
  }
  selected
}

/// Remove anonymous polygon ridges only on wholly smooth source arcs.
///
/// This supplements the finite-facet fallback, not the continuous terminal
/// and paired-return models. The retained empirical Office terminal radius
/// is unchanged. Exact-config Example contour-color responses at .5, .75,
/// 1, 1.25, 1.5 and 2pt expose excess contour on the smooth m shoulder.
/// Real source joins and visibility boundaries remain authoritative.
pub(super) fn smooth_arc_joins(
  geometry: &Static3dTextGeometry,
  contour: &Static3dTextContour,
  edges: &[(f32, bool)],
  joins: &[bool],
  pixels_per_point: f32,
) -> Option<Vec<bool>> {
  let source = geometry
    .source_coverage_path
    .elements()
    .get(contour.source_path_range.clone())?;
  let mut segments = kurbo::segments(source.iter().copied()).collect::<Vec<_>>();
  let first = segments.first()?.start();
  let last = segments.last()?.end();
  if first.distance(last) > 1.0e-4 {
    segments.push(PathSeg::Line(kurbo::Line::new(last, first)));
  }
  let orientation = if geometry.solid_on_right { 1.0 } else { -1.0 };
  let eligible = smooth_convex_segments(
    &segments,
    orientation,
    WORD_TERMINAL_CURVATURE_RADIUS_PT * f64::from(pixels_per_point),
  );
  if !eligible.iter().any(|&value| value) {
    return None;
  }
  let count = contour.points.len();
  let mut selected = joins.to_vec();
  let mut changed = false;
  for (index, join) in selected.iter_mut().enumerate() {
    if !*join
      || contour.longitudinal_contour_joins[index]
      || !contour.incoming_curve_edges[index]
      || !contour.incoming_curve_edges[(index + 1) % count]
      || edges[index].1 != edges[(index + count - 1) % count].1
    {
      continue;
    }
    let point = Point::from(contour.points[index]);
    let distances = segments
      .iter()
      .map(|segment| segment.nearest(point, 1.0e-6).distance_sq.sqrt())
      .collect::<Vec<_>>();
    let minimum = distances.iter().copied().fold(f64::INFINITY, f64::min);
    // At a shared endpoint both source segments own the location. Do not
    // choose a smooth donor across an adjacent non-smooth boundary.
    if minimum.is_finite()
      && distances
        .iter()
        .zip(&eligible)
        .all(|(&distance, &eligible)| distance > minimum + 1.0e-4 || eligible)
    {
      *join = false;
      changed = true;
    }
  }
  changed.then_some(selected)
}

/// A curved return side can have two rounded shoulders separated by a
/// gently curving wall. Flattened shoulder vertices are not independent
/// contour ridges extending into that wall. Keep this two-sided state
/// separate from a smooth line terminal and from a single rounded corner.
///
/// The radius is the retained empirical Office terminal radius, not a PDF
/// or OOXML constant. Native Office E upper/middle/lower controls from
/// .01 through 1.25pt distinguish this paired state from m/l caps and the
/// asymmetric e terminal. No line-entry phase is applied inside the curve.
pub(super) fn curved_return_bands(
  geometry: &Static3dTextGeometry,
  contour: &Static3dTextContour,
  edges: &[(f32, bool)],
  joins: &[bool],
  width: f32,
  pixels_per_point: f32,
) -> Option<EdgeBands> {
  let elements = geometry
    .source_coverage_path
    .elements()
    .get(contour.source_path_range.clone())?;
  let mut segments = kurbo::segments(elements.iter().copied()).collect::<Vec<_>>();
  let first = segments.first()?.start();
  let last = segments.last()?.end();
  if first.distance(last) > 1.0e-4 {
    segments.push(PathSeg::Line(kurbo::Line::new(last, first)));
  }
  let orientation = if geometry.solid_on_right { 1.0 } else { -1.0 };
  let radius = WORD_TERMINAL_CURVATURE_RADIUS_PT * f64::from(pixels_per_point);
  let mut domains: Vec<(f64, f64)> = Vec::new();
  let mut barriers = Vec::new();
  let mut exact_offsets = vec![0.0];
  for &segment in &segments {
    let offset = *exact_offsets.last()?;
    let length = segment.arclen(1.0e-5);
    if !length.is_finite() {
      return None;
    }
    // A line, concavity, or inflection interrupts the convex return wall.
    let convex = continuously_convex(segment, orientation);
    if !convex {
      barriers.push((offset, offset + length));
    }
    for (start, end) in convex_curvature_intervals(segment, orientation, radius) {
      let start = offset + segment_partial_length(segment, start);
      let end = offset + segment_partial_length(segment, end);
      if let Some(previous) = domains.last_mut().filter(|last| start <= last.1 + 1.0e-8) {
        previous.1 = previous.1.max(end);
      } else {
        domains.push((start, end));
      }
    }
    exact_offsets.push(offset + length);
  }
  if domains.len() < 2 {
    return None;
  }
  let exact_perimeter = *exact_offsets.last()?;
  let mut cumulative = vec![0.0];
  for &(length, _) in edges {
    cumulative.push(cumulative.last()? + f64::from(length));
  }
  let perimeter = *cumulative.last()?;
  if perimeter <= 0.0 || !perimeter.is_finite() || exact_perimeter <= 0.0 {
    return None;
  }
  let mapped_arc = |distance: f64| {
    let distance = distance.rem_euclid(exact_perimeter);
    let index = exact_offsets
      .partition_point(|&offset| offset <= distance)
      .saturating_sub(1)
      .min(segments.len() - 1);
    let segment = segments[index];
    let t = segment.inv_arclen(distance - exact_offsets[index], 1.0e-5);
    path_distance(segment.eval(t), &contour.points, &cumulative)
  };
  let mut selected = joins.to_vec();
  let mut new_seeds = Vec::new();
  let count = edges.len();
  for (index, &(_, start)) in domains.iter().enumerate() {
    let end = domains[(index + 1) % domains.len()].0
      + if index + 1 == domains.len() {
        exact_perimeter
      } else {
        0.0
      };
    if end - start <= 1.0e-6
      || barriers.iter().any(|&(a, b)| {
        [0.0, exact_perimeter]
          .into_iter()
          .any(|shift| (b + shift).min(end) > (a + shift).max(start) + 1.0e-9)
      })
    {
      continue;
    }
    let first = mapped_arc(start)?;
    let last = mapped_arc(end)?;
    let gap_length = (last - first).rem_euclid(perimeter);
    let inside = |position: f64| {
      let distance = (position - first).rem_euclid(perimeter);
      distance > 1.0e-6 && distance < gap_length - 1.0e-6
    };
    if (0..count).any(|i| {
      inside(cumulative[i])
        && (contour.longitudinal_contour_joins[i] || edges[i].1 != edges[(i + count - 1) % count].1)
    }) {
      continue;
    }
    let removed = (0..count)
      .map(|i| selected[i] && inside(cumulative[i]))
      .collect::<Vec<_>>();
    let runs = (0..count)
      .filter(|&i| removed[i] && !removed[(i + count - 1) % count])
      .count();
    if runs != 2 {
      continue;
    }
    for (selected, removed) in selected.iter_mut().zip(removed) {
      *selected &= !removed;
    }
    new_seeds.extend([first, last]);
  }
  if new_seeds.is_empty() {
    return None;
  }
  new_seeds.extend((0..count).filter_map(|i| {
    (selected[i] || edges[i].1 != edges[(i + count - 1) % count].1).then_some(cumulative[i])
  }));
  Some(bands_from_seeds(&cumulative, &new_seeds, f64::from(width)))
}

fn derivatives(segment: PathSeg, t: f64) -> (Vec2, Vec2) {
  match segment {
    PathSeg::Line(line) => (line.p1 - line.p0, Vec2::ZERO),
    PathSeg::Quad(q) => {
      let a = q.p1 - q.p0;
      let b = q.p2 - q.p1;
      ((a * (1.0 - t) + b * t) * 2.0, (b - a) * 2.0)
    }
    PathSeg::Cubic(c) => {
      let a = c.p1 - c.p0;
      let b = c.p2 - c.p1;
      let d = c.p3 - c.p2;
      (
        (a * (1.0 - t).powi(2) + b * (2.0 * t * (1.0 - t)) + d * t * t) * 3.0,
        ((b - a) * (1.0 - t) + (d - b) * t) * 6.0,
      )
    }
  }
}

fn curvature(segment: PathSeg, t: f64, orientation: f64) -> f64 {
  let (d, dd) = derivatives(segment, t);
  let speed = d.hypot();
  if speed <= 1.0e-12 {
    return 0.0;
  }
  orientation * d.cross(dd) / speed.powi(3)
}

fn terminal_point(chain: &[PathSeg], orientation: f64, radius: f64, phase: f64) -> Option<Point> {
  let accuracy = 1.0e-5;
  let mut first = None;
  let mut offset = 0.0;
  let mut lengths = Vec::with_capacity(chain.len());
  for &segment in chain {
    let length = segment.arclen(accuracy);
    if !length.is_finite() {
      return None;
    }
    lengths.push(length);
    if first.is_none() {
      if curvature(segment, 0.0, orientation) <= 0.0 {
        return None;
      }
      let threshold = 1.0 / radius;
      for step in 0..=128 {
        let mut high = f64::from(step) / 128.0;
        if curvature(segment, high, orientation) < threshold {
          continue;
        }
        let mut low = f64::from((step - 1).max(0)) / 128.0;
        for _ in 0..36 {
          let middle = (low + high) * 0.5;
          if curvature(segment, middle, orientation) >= threshold {
            high = middle;
          } else {
            low = middle;
          }
        }
        let parameter = (low + high) * 0.5;
        // A threshold reached at the segment start has exactly zero arc
        // length; avoid evaluating a degenerate zero-range quadratic.
        let before = if parameter == 0.0 {
          0.0
        } else {
          segment.subsegment(0.0..parameter).arclen(accuracy)
        };
        first = Some(offset + before);
        break;
      }
    }
    offset += length;
  }
  let mut distance = first? + phase;
  if distance >= offset {
    return None;
  }
  for (&segment, &length) in chain.iter().zip(&lengths) {
    if distance <= length {
      return Some(segment.eval(segment.inv_arclen(distance, accuracy)));
    }
    distance -= length;
  }
  None
}

fn path_distance(point: Point, points: &[(f32, f32)], cumulative: &[f64]) -> Option<f64> {
  let mut nearest = (f64::INFINITY, 0.0);
  for index in 0..points.len() {
    let a = Point::from(points[index]);
    let b = Point::from(points[(index + 1) % points.len()]);
    let edge = b - a;
    if edge.hypot2() <= 1.0e-12 {
      continue;
    }
    let t = ((point - a).dot(edge) / edge.hypot2()).clamp(0.0, 1.0);
    let error = point.distance(a + edge * t);
    if error < nearest.0 {
      nearest = (
        error,
        cumulative[index] + t * (cumulative[index + 1] - cumulative[index]),
      );
    }
  }
  // Match the existing curve-flatness contract, including f32 path storage.
  (nearest.0 <= super::TEXT_3D_CURVE_FLATTENING_TOLERANCE_PX * 1.1 + 1.0e-3).then_some(nearest.1)
}

fn bands_from_seeds(cumulative: &[f64], seeds: &[f64], width: f64) -> EdgeBands {
  let perimeter = *cumulative.last().unwrap_or(&0.0);
  let mut intervals = Vec::with_capacity(seeds.len() * 3);
  for &seed in seeds {
    for shift in [-perimeter, 0.0, perimeter] {
      let start = (seed + shift - width * 0.5).max(0.0);
      let end = (seed + shift + width * 0.5).min(perimeter);
      if start < end {
        intervals.push((start, end));
      }
    }
  }
  intervals.sort_by(|a, b| a.0.total_cmp(&b.0));
  let mut merged: Vec<(f64, f64)> = Vec::with_capacity(intervals.len());
  for (start, end) in intervals {
    if let Some(last) = merged.last_mut().filter(|last| start <= last.1) {
      last.1 = last.1.max(end);
    } else {
      merged.push((start, end));
    }
  }
  let mut next = 0;
  cumulative
    .windows(2)
    .map(|edge| {
      let (start, end) = (edge[0], edge[1]);
      while next < merged.len() && merged[next].1 <= start {
        next += 1;
      }
      let mut bands = Vec::new();
      if end - start > f64::from(f32::EPSILON) {
        for &(lo, hi) in &merged[next..] {
          if lo >= end {
            break;
          }
          bands.push((
            ((lo.max(start) - start) / (end - start)) as f32,
            ((hi.min(end) - start) / (end - start)) as f32,
          ));
        }
      }
      bands
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn smooth_arc_classification_survives_subdivision_elevation_and_scale() {
    let q = kurbo::QuadBez::new((0.0, 0.0), (20.0, 0.0), (20.0, 20.0));
    for count in [1, 2, 3, 8, 32] {
      for scale in [1.0, 3.0, 8.0] {
        let scaled = kurbo::Affine::scale(scale) * q;
        let pieces = (0..count)
          .map(|i| {
            PathSeg::Quad(scaled).subsegment(i as f64 / count as f64..(i + 1) as f64 / count as f64)
          })
          .collect::<Vec<_>>();
        assert_eq!(
          smooth_convex_segments(&pieces, 1.0, scale),
          vec![true; count]
        );
        let elevated = pieces
          .iter()
          .map(|s| PathSeg::Cubic(s.to_cubic()))
          .collect::<Vec<_>>();
        assert_eq!(
          smooth_convex_segments(&elevated, 1.0, scale),
          vec![true; count]
        );
        let reversed = pieces
          .iter()
          .rev()
          .map(PathSeg::reverse)
          .collect::<Vec<_>>();
        assert_eq!(
          smooth_convex_segments(&reversed, -1.0, scale),
          vec![true; count]
        );
        assert_eq!(
          smooth_convex_segments(&pieces, -1.0, scale),
          vec![false; count]
        );
      }
    }
    // One high-curvature part disqualifies the entire convex source chain,
    // even when the contour's storage seam splits that chain in two.
    let shoulder = PathSeg::Quad(kurbo::QuadBez::new(
      (20.0, 20.0),
      (20.0, 20.1),
      (19.9, 20.1),
    ));
    let wall = PathSeg::Quad(q);
    let line = PathSeg::Line(kurbo::Line::new((19.9, 20.1), (0.0, 0.0)));
    assert_eq!(
      smooth_convex_segments(&[wall, shoulder, line], 1.0, 1.0),
      vec![false; 3]
    );
    assert_eq!(
      smooth_convex_segments(&[shoulder, line, wall], 1.0, 1.0),
      vec![false; 3]
    );
  }

  #[test]
  fn smooth_arc_filter_preserves_source_joins_and_visibility_boundaries() {
    use crate::common::{PathCommand, Point as PagePoint, Pt};
    let page = |point: Point| PagePoint {
      x: Pt(point.x as f32),
      y: Pt(point.y as f32),
    };
    let cubic = kurbo::QuadBez::new((0.0, 0.0), (20.0, 0.0), (20.0, 20.0)).raise();
    let commands = [
      PathCommand::MoveTo(page(cubic.p0)),
      PathCommand::CubicTo {
        control1: page(cubic.p1),
        control2: page(cubic.p2),
        end: page(cubic.p3),
      },
      PathCommand::Close,
    ];
    let mut geometry =
      Static3dTextGeometry::from_page_path_with_mapping(&commands, 1.0, 1.0, 0.0, 0.0).unwrap();
    geometry.solid_on_right = true;
    let contour = &geometry.contours[0];
    let edges = contour
      .points
      .iter()
      .zip(contour.points.iter().cycle().skip(1))
      .map(|(a, b)| ((b.0 - a.0).hypot(b.1 - a.1), true))
      .collect::<Vec<_>>();
    let joins = vec![true; edges.len()];
    let selected = smooth_arc_joins(&geometry, contour, &edges, &joins, 1.0).unwrap();
    let removed = selected.iter().position(|&value| !value).unwrap();
    for (i, &source_join) in contour.longitudinal_contour_joins.iter().enumerate() {
      if source_join {
        assert!(selected[i]);
      }
    }
    let mut visibility = edges.clone();
    visibility[removed].1 = false;
    let guarded = smooth_arc_joins(&geometry, contour, &visibility, &joins, 1.0).unwrap();
    assert!(guarded[removed]);
    assert!(guarded[(removed + 1) % guarded.len()]);
    geometry.contours[0].longitudinal_contour_joins[removed] = true;
    let guarded = smooth_arc_joins(&geometry, &geometry.contours[0], &edges, &joins, 1.0).unwrap();
    assert!(guarded[removed]);
  }

  #[test]
  fn smooth_arc_filter_keeps_rounded_straight_side_corners_at_every_scale() {
    for scale in [1.0, 3.0, 8.0] {
      for count in [1, 2, 4, 16] {
        let q =
          kurbo::Affine::scale(scale) * kurbo::QuadBez::new((0.0, 0.0), (20.0, 0.0), (20.0, 20.0));
        let mut pieces = vec![PathSeg::Line(kurbo::Line::new((-scale, 0.0), q.p0))];
        pieces.extend((0..count).map(|i| {
          PathSeg::Quad(q).subsegment(i as f64 / count as f64..(i + 1) as f64 / count as f64)
        }));
        pieces.push(PathSeg::Line(kurbo::Line::new(
          q.p2,
          (20.0 * scale, 21.0 * scale),
        )));
        let length = pieces.len();
        for _ in 0..length {
          assert_eq!(
            smooth_convex_segments(&pieces, 1.0, scale),
            vec![false; length]
          );
          pieces.rotate_left(1);
        }
      }
    }
  }

  #[test]
  fn curvature_root_isolation_keeps_repeated_and_narrow_roots() {
    let roots = polynomial_unit_roots(&[-0.028, 0.32, -1.1, 1.0]);
    assert_eq!(roots.len(), 2, "{roots:?}");
    assert!((roots[0] - 0.2).abs() < 1.0e-7);
    assert!((roots[1] - 0.7).abs() < 1.0e-7);
    let close = polynomial_product(&[-0.501, 1.0], &[-0.502, 1.0]);
    let roots = polynomial_unit_roots(&close);
    assert_eq!(roots.len(), 2, "{roots:?}");
    assert!((roots[0] - 0.501).abs() < 1.0e-7);
    assert!((roots[1] - 0.502).abs() < 1.0e-7);
  }

  #[test]
  fn curvature_domains_agree_for_quadratic_cubic_and_reversal() {
    let q = kurbo::QuadBez::new((0.0, 0.0), (20.0, 0.0), (20.0, 5.0));
    let expected = convex_curvature_intervals(PathSeg::Quad(q), 1.0, 6.1);
    assert_eq!(expected.len(), 1);
    let elevated = convex_curvature_intervals(PathSeg::Cubic(q.raise()), 1.0, 6.1);
    let reversed = convex_curvature_intervals(PathSeg::Quad(q).reverse(), -1.0, 6.1);
    assert_eq!(elevated.len(), expected.len());
    assert_eq!(reversed.len(), expected.len());
    assert!((elevated[0].0 - expected[0].0).abs() < 1.0e-7);
    assert!((elevated[0].1 - expected[0].1).abs() < 1.0e-7);
    assert!((reversed[0].0 - (1.0 - expected[0].1)).abs() < 1.0e-7);
    assert!((reversed[0].1 - (1.0 - expected[0].0)).abs() < 1.0e-7);
    assert!(convex_curvature_intervals(PathSeg::Quad(q), -1.0, 6.1).is_empty());
    assert!(continuously_convex(PathSeg::Quad(q), 1.0));
    assert!(!continuously_convex(PathSeg::Quad(q), -1.0));
  }

  #[test]
  fn curved_return_requires_two_shoulders_not_a_straight_side() {
    use crate::common::{PathCommand, Point as PagePoint, Pt};
    let page = |point: Point| PagePoint {
      x: Pt(point.x as f32),
      y: Pt(point.y as f32),
    };
    let mut commands = vec![
      PathCommand::MoveTo(page(Point::new(0.0, 0.0))),
      PathCommand::LineTo(page(Point::new(50.0, 0.0))),
    ];
    for q in [
      kurbo::QuadBez::new((50.0, 0.0), (53.0, 0.0), (55.0, 3.0)),
      kurbo::QuadBez::new((55.0, 3.0), (57.0, 6.0), (57.0, 12.0)),
      kurbo::QuadBez::new((57.0, 12.0), (57.0, 18.0), (55.0, 21.0)),
      kurbo::QuadBez::new((55.0, 21.0), (53.0, 24.0), (50.0, 24.0)),
    ] {
      let cubic = q.raise();
      commands.push(PathCommand::CubicTo {
        control1: page(cubic.p1),
        control2: page(cubic.p2),
        end: page(cubic.p3),
      });
    }
    commands.extend([
      PathCommand::LineTo(page(Point::new(0.0, 24.0))),
      PathCommand::Close,
    ]);
    let geometry =
      Static3dTextGeometry::from_page_path_with_mapping(&commands, 1.0, 1.0, 0.0, 0.0).unwrap();
    let contour = &geometry.contours[0];
    let edges = contour
      .points
      .iter()
      .zip(contour.points.iter().cycle().skip(1))
      .map(|(a, b)| ((b.0 - a.0).hypot(b.1 - a.1), true))
      .collect::<Vec<_>>();
    let joins = super::super::text_extrusion_surface_contour_joins(
      &contour.points,
      &contour.incoming_curve_edges,
      &contour.longitudinal_contour_joins,
      geometry.solid_on_right,
    );
    assert!(curved_return_bands(&geometry, contour, &edges, &joins, 2.0, 600.0 / 72.0).is_some());
    // The same topology with no synthetic ridge witnesses must not invent
    // a pair or replace authored source corners.
    assert!(
      curved_return_bands(
        &geometry,
        contour,
        &edges,
        &contour.longitudinal_contour_joins,
        2.0,
        600.0 / 72.0
      )
      .is_none()
    );
  }

  #[test]
  fn original_path_ranges_keep_closed_open_and_degenerate_contours_separate() {
    use crate::common::{PathCommand, Point as PagePoint, Pt};

    let point = |x, y| PagePoint { x: Pt(x), y: Pt(y) };
    let commands = [
      PathCommand::MoveTo(point(0.0, 0.0)),
      PathCommand::LineTo(point(10.0, 0.0)),
      PathCommand::LineTo(point(0.0, 10.0)),
      PathCommand::Close,
      // This degenerate contour must not take ownership of the next range.
      PathCommand::MoveTo(point(20.0, 20.0)),
      PathCommand::LineTo(point(21.0, 20.0)),
      PathCommand::Close,
      // A MoveTo implicitly finishes the previous accepted open contour.
      PathCommand::MoveTo(point(30.0, 0.0)),
      PathCommand::LineTo(point(40.0, 0.0)),
      PathCommand::LineTo(point(30.0, 10.0)),
      PathCommand::MoveTo(point(50.0, 0.0)),
      PathCommand::CubicTo {
        control1: point(60.0, 0.0),
        control2: point(60.0, 10.0),
        end: point(50.0, 10.0),
      },
    ];
    let geometry =
      Static3dTextGeometry::from_page_path_with_mapping(&commands, 2.0, 2.0, 3.0, 4.0).unwrap();
    assert_eq!(geometry.contours.len(), 3);
    let elements = geometry.source_coverage_path.elements();
    for (contour, expected) in geometry.contours.iter().zip([0..4, 7..10, 10..12]) {
      assert_eq!(contour.source_path_range, expected);
      let segments = kurbo::segments(elements[expected].iter().copied()).collect::<Vec<_>>();
      assert!(!segments.is_empty());
      assert_eq!(segments[0].start(), Point::from(contour.points[0]));
    }
    let source = &elements[geometry.contours[2].source_path_range.clone()];
    assert_eq!(source.len(), 2);
    assert!(matches!(source[1], kurbo::PathEl::CurveTo(..)));
    assert!(geometry.contours[2].points.len() > source.len());
  }

  #[test]
  fn continuous_bands_preserve_interior_gaps_and_periodic_seams() {
    let bands = bands_from_seeds(&[0.0, 10.0, 20.0], &[0.0, 3.0, 7.0], 2.0);
    assert_eq!(bands[0], vec![(0.0, 0.1), (0.2, 0.4), (0.6, 0.8)]);
    assert_eq!(bands[1], vec![(0.9, 1.0)]);
    assert_eq!(
      bands_from_seeds(&[0.0, 10.0, 20.0], &[0.0], 50.0),
      vec![vec![(0.0, 1.0)]; 2]
    );
  }

  #[test]
  fn source_terminal_is_independent_of_exact_curve_subdivision() {
    let q = PathSeg::Quad(kurbo::QuadBez::new((0.0, 0.0), (20.0, 0.0), (20.0, 5.0)));
    let target = terminal_point(&[q], 1.0, 6.1, 0.85).unwrap();
    for count in [2, 3, 4, 8] {
      let pieces = (0..count)
        .map(|i| q.subsegment(f64::from(i) / f64::from(count)..f64::from(i + 1) / f64::from(count)))
        .collect::<Vec<_>>();
      let actual = terminal_point(&pieces, 1.0, 6.1, 0.85).unwrap();
      assert!(
        actual.distance(target) < 1.0e-4,
        "{count}: {actual:?} vs {target:?}"
      );
    }
    let reverse = q.reverse();
    assert!(terminal_point(&[reverse], -1.0, 6.1, 0.85).is_some());
  }
}
