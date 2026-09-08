//! Per-path coverage on the Direct3D standard four-sample lattice.
//!
//! D3D11.3 sections 3.2.4, 3.4.1 and 3.5.6 specify fixed-point vertices,
//! top-left ownership and Direct2D's resolve-before-composite semantics:
//! https://microsoft.github.io/DirectX-Specs/d3d/archive/D3D11_3_FunctionalSpec.htm
//! This is an explicit coverage profile, not a claim that every Direct2D
//! render target uses four samples. Curve flattening remains a separate
//! approximation; the lattice is applied to the expanded path, not its pen
//! centerline. A single output-sized byte mask replaces a supersampled scene.

use kurbo::{BezPath, PathEl, Point};
use tiny_skia::{FillRule, Mask};

const SCALE: i64 = 256;
const SAMPLES: [(i64, i64); 4] = [(96, 32), (224, 96), (32, 160), (160, 224)];
const MAX_EDGES: usize = 1 << 20;

#[derive(Clone, Copy)]
struct FixedPoint {
  x: i64,
  y: i64,
}

impl FixedPoint {
  fn from_point(point: Point) -> Option<Self> {
    let fixed = |value: f64| {
      let value = (value * SCALE as f64).round_ties_even();
      (value.is_finite() && value >= i32::MIN as f64 && value <= i32::MAX as f64)
        .then_some(value as i64)
    };
    Some(Self {
      x: fixed(point.x)?,
      y: fixed(point.y)?,
    })
  }
}

struct Edge {
  top: FixedPoint,
  bottom: FixedPoint,
  winding: i32,
}

#[derive(Clone, Copy)]
struct Crossing {
  numerator: i128,
  denominator: i128,
  winding: i32,
}

impl Crossing {
  fn compare(&self, other: &Self) -> std::cmp::Ordering {
    (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator))
  }

  // First pixel whose sample lies on or to the right of this intersection.
  fn pixel_ceiling(self, sample_x: i64, width: u32) -> usize {
    let numerator = self.numerator - i128::from(sample_x) * self.denominator;
    let denominator = i128::from(SCALE) * self.denominator;
    let ceil =
      numerator.div_euclid(denominator) + i128::from(numerator.rem_euclid(denominator) != 0);
    ceil.clamp(0, i128::from(width)) as usize
  }
}

/// Returns resolved A8 coverage for one device-space filled outline.
/// Stroke widening, transforms, paint and compositing belong to the caller.
pub(super) fn standard_four_sample_mask(
  path: &BezPath,
  width: u32,
  height: u32,
  fill_rule: FillRule,
) -> Option<Mask> {
  // Validate control points before flattening, including points on curves
  // whose endpoint alone would hide an invalid or excessive coordinate.
  for element in path.elements() {
    match *element {
      PathEl::MoveTo(p) | PathEl::LineTo(p) => {
        FixedPoint::from_point(p)?;
      }
      PathEl::QuadTo(p, q) => {
        FixedPoint::from_point(p)?;
        FixedPoint::from_point(q)?;
      }
      PathEl::CurveTo(p, q, r) => {
        FixedPoint::from_point(p)?;
        FixedPoint::from_point(q)?;
        FixedPoint::from_point(r)?;
      }
      PathEl::ClosePath => {}
    }
  }
  let mut mask = Mask::new(width, height)?;
  let mut edges = Vec::new();
  let mut first = None;
  let mut current = None;
  let mut valid = true;
  let mut add_edge = |from: Option<FixedPoint>, to: Option<FixedPoint>| {
    let (Some(from), Some(to)) = (from, to) else {
      return;
    };
    if from.y == to.y {
      return;
    }
    if edges.len() == MAX_EDGES {
      valid = false;
      return;
    }
    let (top, bottom, winding) = if from.y < to.y {
      (from, to, 1)
    } else {
      (to, from, -1)
    };
    edges.push(Edge {
      top,
      bottom,
      winding,
    });
  };
  kurbo::flatten(path.iter(), 1.0 / SCALE as f64, |element| match element {
    PathEl::MoveTo(point) => {
      add_edge(current, first);
      first = FixedPoint::from_point(point);
      current = first;
    }
    PathEl::LineTo(point) => {
      let next = FixedPoint::from_point(point);
      add_edge(current, next);
      current = next;
    }
    PathEl::ClosePath => {
      add_edge(current, first);
      current = first;
    }
    PathEl::QuadTo(..) | PathEl::CurveTo(..) => unreachable!("flatten emits line segments"),
  });
  // Fill implicitly closes open figures, but never joins separate contours.
  add_edge(current, first);
  if !valid {
    return None;
  }
  edges.sort_unstable_by_key(|edge| edge.top.y);
  let mut next_edge = 0;
  let mut active = Vec::<usize>::new();
  let mut crossings = Vec::<Crossing>::new();
  for y in 0..height {
    let row_start = y as usize * width as usize;
    let row = &mut mask.data_mut()[row_start..row_start + width as usize];
    for (sample_x, sample_y) in SAMPLES {
      let scan_y = i64::from(y) * SCALE + sample_y;
      active.retain(|&index| edges[index].bottom.y > scan_y);
      while let Some(edge) = edges.get(next_edge).filter(|edge| edge.top.y <= scan_y) {
        if edge.bottom.y > scan_y {
          active.push(next_edge);
        }
        next_edge += 1;
      }
      crossings.clear();
      for &index in &active {
        let edge = &edges[index];
        let dy = i128::from(edge.bottom.y - edge.top.y);
        let dx = i128::from(edge.bottom.x - edge.top.x);
        crossings.push(Crossing {
          numerator: i128::from(edge.top.x) * dy + i128::from(scan_y - edge.top.y) * dx,
          denominator: dy,
          winding: edge.winding,
        });
      }
      crossings.sort_unstable_by(Crossing::compare);
      let mut winding = 0_i32;
      let mut left = None;
      for crossing in &crossings {
        let inside = match fill_rule {
          FillRule::Winding => winding != 0,
          FillRule::EvenOdd => winding.rem_euclid(2) != 0,
        };
        if inside && let Some(left) = left {
          let start = Crossing::pixel_ceiling(left, sample_x, width);
          let end = crossing.pixel_ceiling(sample_x, width);
          for coverage in &mut row[start..end] {
            *coverage += 1;
          }
        }
        winding += crossing.winding;
        left = Some(*crossing);
      }
    }
    for coverage in row {
      *coverage = ((u16::from(*coverage) * 255 + 2) / 4) as u8;
    }
  }
  Some(mask)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn polygon(points: &[(f64, f64)]) -> BezPath {
    let mut path = BezPath::new();
    path.move_to(points[0]);
    for &point in &points[1..] {
      path.line_to(point);
    }
    path.close_path();
    path
  }

  #[test]
  fn four_sample_diagonal_matches_captured_office_interior() {
    // Captured six-device-pixel flat stroke; no fitted width or origin.
    let n = 3.0 / 2.0_f64.sqrt();
    let path = polygon(&[
      (10.0 + n, -n),
      (50.0 + n, 40.0 - n),
      (50.0 - n, 40.0 + n),
      (10.0 - n, n),
    ]);
    let mask = standard_four_sample_mask(&path, 64, 40, FillRule::Winding).unwrap();
    let row = &mask.data()[20 * 64..21 * 64];
    assert_eq!(&row[26..35], &[128, 255, 255, 255, 255, 255, 255, 255, 128]);
    assert_eq!(row.iter().map(|&x| u32::from(x)).sum::<u32>(), 2041);
  }

  #[test]
  fn four_sample_top_left_negative_clip_and_winding() {
    let left = polygon(&[(-3.0, -1.0), (0.375, -1.0), (0.375, 2.0), (-3.0, 2.0)]);
    let right = polygon(&[(0.375, -1.0), (3.0, -1.0), (3.0, 2.0), (0.375, 2.0)]);
    assert_eq!(
      standard_four_sample_mask(&left, 1, 1, FillRule::Winding)
        .unwrap()
        .data(),
      &[64]
    );
    assert_eq!(
      standard_four_sample_mask(&right, 1, 1, FillRule::Winding)
        .unwrap()
        .data(),
      &[191]
    );
    let outer = polygon(&[(0.0, 0.0), (3.0, 0.0), (3.0, 3.0), (0.0, 3.0)]);
    let inner = polygon(&[(1.0, 1.0), (2.0, 1.0), (2.0, 2.0), (1.0, 2.0)]);
    let mut same = outer.clone();
    same.extend(inner.iter());
    assert_eq!(
      standard_four_sample_mask(&same, 3, 3, FillRule::Winding)
        .unwrap()
        .data()[4],
      255
    );
    assert_eq!(
      standard_four_sample_mask(&same, 3, 3, FillRule::EvenOdd)
        .unwrap()
        .data()[4],
      0
    );
    let mut opposite = outer;
    opposite.extend(inner.reverse_subpaths().iter());
    assert_eq!(
      standard_four_sample_mask(&opposite, 3, 3, FillRule::Winding)
        .unwrap()
        .data()[4],
      0
    );
  }

  #[test]
  fn four_sample_implicitly_closes_separate_figures_and_rejects_nonfinite() {
    let mut path = BezPath::new();
    path.move_to((0.0, 0.0));
    path.line_to((1.0, 0.0));
    path.line_to((1.0, 1.0));
    path.line_to((0.0, 1.0));
    path.move_to((2.0, 0.0));
    path.line_to((3.0, 0.0));
    path.line_to((3.0, 1.0));
    path.line_to((2.0, 1.0));
    assert_eq!(
      standard_four_sample_mask(&path, 3, 1, FillRule::EvenOdd)
        .unwrap()
        .data(),
      &[255, 0, 255]
    );
    path.line_to((f64::NAN, 0.0));
    assert!(standard_four_sample_mask(&path, 3, 1, FillRule::EvenOdd).is_none());
  }
}
