//! GDI's synthesized TrueType outline, before printer scaling.
//!
//! Native GetGlyphOutlineW controls distinguish this from either a centered
//! stroke or FreeType's unbounded bisector expansion. Offset adjacent contour
//! edges, intersect them, clamp each displacement component, then translate.
//! The 26.6 arithmetic also preserves component placement and implicit points.

use kurbo::{BezPath, Point};
use skrifa::{
  GlyphId,
  raw::{
    FontRef, TableProvider,
    tables::{
      glyf::{Anchor, CompositeGlyphFlags, Glyph},
      loca::Loca,
    },
  },
};

const DESIGN_EM: i64 = 2048;
const MAX_POINTS: usize = 65_536;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct GridPoint {
  x: i64,
  y: i64,
}

impl GridPoint {
  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }

  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

#[derive(Default)]
struct Contours {
  points: Vec<GridPoint>,
  on_curve: Vec<bool>,
  ends: Vec<usize>,
}

/// Return font-unit coordinates so the consumer retains its existing em,
/// advance and placement owners. Only static TrueType outlines are admitted.
pub(super) fn synthetic_bold(font: &FontRef<'_>, glyph: GlyphId) -> Option<BezPath> {
  if font.fvar().is_ok() {
    return None;
  }
  let units = i64::from(font.head().ok()?.units_per_em());
  if units < 16 {
    return None;
  }
  let glyf = font.glyf().ok()?;
  let loca = font.loca(None).ok()?;
  // Native font-driver transformation: remove the vertical bold increment
  // from the requested em, and expand by (increment + 1, increment). The
  // path branch keeps fractional half-strengths, unlike its bitmap branch.
  let strength = (DESIGN_EM * 20 - 10) / 1000;
  let mut contours = load(&loca, &glyf, glyph, DESIGN_EM - strength, units, 0)?;
  let half = GridPoint {
    x: (strength + 1) * 32,
    y: strength * 32,
  };
  let mut start = 0;
  for &end in &contours.ends {
    embolden(&mut contours.points[start..=end], half);
    start = end + 1;
  }
  Some(to_path(&contours, units as f64 / (DESIGN_EM * 64) as f64))
}

fn load(
  loca: &Loca<'_>,
  glyf: &skrifa::raw::tables::glyf::Glyf<'_>,
  glyph: GlyphId,
  em: i64,
  units: i64,
  depth: usize,
) -> Option<Contours> {
  if depth >= 32 {
    return None;
  }
  let mut result = Contours::default();
  let Some(glyph) = loca.get_glyf(glyph, glyf).ok()? else {
    return Some(result);
  };
  match glyph {
    Glyph::Simple(glyph) => {
      let count = glyph.num_points();
      if count > MAX_POINTS {
        return None;
      }
      result.points.reserve(count);
      result.on_curve.reserve(count);
      for point in glyph.points() {
        let scale = |value: i16| (i64::from(value) * em * 64 + units / 2).div_euclid(units);
        result.points.push(GridPoint {
          x: scale(point.x),
          y: scale(point.y),
        });
        result.on_curve.push(point.on_curve);
      }
      if result.points.len() != count {
        return None;
      }
      result.ends = glyph
        .end_pts_of_contours()
        .iter()
        .map(|value| usize::from(value.get()))
        .collect();
      if result.ends.windows(2).any(|pair| pair[0] >= pair[1]) {
        return None;
      }
    }
    Glyph::Composite(glyph) => {
      for component in glyph.components() {
        let mut child = load(loca, glyf, component.glyph.into(), em, units, depth + 1)?;
        if result.points.len().checked_add(child.points.len())? > MAX_POINTS {
          return None;
        }
        let matrix = component.transform;
        let transform = |point: GridPoint| GridPoint {
          x: (point.x * i64::from(matrix.xx.to_bits())
            + point.y * i64::from(matrix.xy.to_bits())
            + 8192)
            >> 14,
          y: (point.x * i64::from(matrix.yx.to_bits())
            + point.y * i64::from(matrix.yy.to_bits())
            + 8192)
            >> 14,
        };
        for point in &mut child.points {
          *point = transform(*point);
        }
        let offset = match component.anchor {
          Anchor::Offset { x, y } => {
            let scale = |value: i16| (i64::from(value) * em * 64 + units / 2).div_euclid(units);
            let mut offset = GridPoint {
              x: scale(x),
              y: scale(y),
            };
            if component
              .flags
              .contains(CompositeGlyphFlags::SCALED_COMPONENT_OFFSET)
            {
              offset = transform(offset);
            }
            if component
              .flags
              .contains(CompositeGlyphFlags::ROUND_XY_TO_GRID)
            {
              offset.x = (offset.x + 32).div_euclid(64) * 64;
              offset.y = (offset.y + 32).div_euclid(64) * 64;
            }
            offset
          }
          Anchor::Point { base, component } => result
            .points
            .get(usize::from(base))?
            .sub(*child.points.get(usize::from(component))?),
        };
        let base = result.points.len();
        for point in &mut child.points {
          *point = point.add(offset);
          // Bound arithmetic for malformed deeply transformed components.
          if point.x.abs().max(point.y.abs()) > i64::from(i32::MAX) / 16 {
            return None;
          }
        }
        result.points.extend(child.points);
        result.on_curve.extend(child.on_curve);
        result
          .ends
          .extend(child.ends.into_iter().map(|end| base + end));
      }
    }
  }
  Some(result)
}

// Native CompDiv rounds a signed product divided by a signed denominator.
fn divide(numerator: i64, denominator: i64) -> i64 {
  let value = (numerator.abs() + denominator.abs() / 2) / denominator.abs();
  if (numerator < 0) != (denominator < 0) {
    -value
  } else {
    value
  }
}

fn intersection(a: GridPoint, b: GridPoint, c: GridPoint, d: GridPoint) -> GridPoint {
  let u = b.sub(a);
  let v = d.sub(c);
  let (numerator, denominator) = if u.y == 0 {
    if v.x == 0 {
      return GridPoint { x: c.x, y: a.y };
    }
    (c.y - a.y, -v.y)
  } else if u.x == 0 {
    if v.y == 0 {
      return GridPoint { x: a.x, y: c.y };
    }
    (c.x - a.x, -v.x)
  } else if u.x.abs() < u.y.abs() {
    (
      divide((c.y - a.y) * u.x, u.y) - c.x + a.x,
      v.x - divide(v.y * u.x, u.y),
    )
  } else {
    (
      c.y - a.y - divide((c.x - a.x) * u.y, u.x),
      divide(v.x * u.y, u.x) - v.y,
    )
  };
  // The driver uses a quarter-pixel parallel-edge threshold in 26.6 space.
  if denominator.abs() <= 16 {
    return GridPoint {
      x: (b.x + c.x) >> 1,
      y: (b.y + c.y) >> 1,
    };
  }
  GridPoint {
    x: c.x + divide(v.x * numerator, denominator),
    y: c.y + divide(v.y * numerator, denominator),
  }
}

fn normal_offset(edge: GridPoint, half: GridPoint) -> GridPoint {
  let length = (edge.x as f64).hypot(edge.y as f64);
  let normal = |value: i64| ((value as f64 * 16384.0 / length + 0.5).floor() as i64) >> 8;
  GridPoint {
    x: (normal(-edge.y) * half.x + 32) >> 6,
    y: (normal(edge.x) * half.y + 32) >> 6,
  }
}

fn embolden(points: &mut [GridPoint], half: GridPoint) {
  if points.len() < 3 {
    return;
  }
  let original = points.to_vec();
  // Collapse adjacent duplicates for edge directions, retaining every
  // authored flag/point for reconstruction. This keeps degenerate input O(n).
  let mut distinct = Vec::with_capacity(points.len());
  for (index, point) in original.iter().enumerate() {
    if distinct.last().is_none_or(|&last| original[last] != *point) {
      distinct.push(index);
    }
  }
  let closing_duplicate = (distinct.len() > 1
    && original[*distinct.last().unwrap()] == original[0])
    .then(|| distinct.pop().unwrap());
  if distinct.len() < 3 {
    return;
  }
  for (index, &at) in distinct.iter().enumerate() {
    let before = original[distinct[(index + distinct.len() - 1) % distinct.len()]];
    let point = original[at];
    let after = original[distinct[(index + 1) % distinct.len()]];
    let first = normal_offset(point.sub(before), half);
    let second = normal_offset(after.sub(point), half);
    let shifted = if first == second {
      point.add(first)
    } else {
      intersection(
        before.add(first),
        point.add(first),
        point.add(second),
        after.add(second),
      )
    };
    let delta = shifted.sub(point);
    let output = point.add(GridPoint {
      x: delta.x.clamp(-half.x, half.x) + half.x,
      y: delta.y.clamp(-half.y, half.y) + half.y,
    });
    let end = distinct
      .get(index + 1)
      .copied()
      .unwrap_or(closing_duplicate.unwrap_or(points.len()));
    points[at..end].fill(output);
    if index == 0
      && let Some(closing) = closing_duplicate
    {
      points[closing..].fill(output);
    }
  }
}

fn to_path(contours: &Contours, scale: f64) -> BezPath {
  let mut path = BezPath::new();
  let mut first = 0;
  for &end in &contours.ends {
    let points = &contours.points[first..=end];
    let flags = &contours.on_curve[first..=end];
    first = end + 1;
    let point = |index: usize| {
      let p = points[index];
      Point::new(p.x as f64 * scale, p.y as f64 * scale)
    };
    let last = points.len() - 1;
    let (anchor, range) = if flags[0] {
      (point(0), 1..points.len())
    } else if flags[last] {
      (point(last), 0..last)
    } else {
      (point(last).midpoint(point(0)), 0..points.len())
    };
    path.move_to(anchor);
    let mut pending: Option<Point> = None;
    for index in range {
      let next = point(index);
      if flags[index] {
        if let Some(control) = pending.take() {
          path.quad_to(control, next);
        } else {
          path.line_to(next);
        }
      } else {
        if let Some(control) = pending.replace(next) {
          path.quad_to(control, control.midpoint(next));
        }
      }
    }
    if let Some(control) = pending {
      path.quad_to(control, anchor);
    }
    path.close_path();
  }
  path
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{model::TextStyle, text_metrics::TextMetrics};
  use kurbo::PathEl;
  use std::sync::Arc;

  fn native_path_fingerprint(path: &BezPath, units_per_em: u16) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    let mut current = Point::ZERO;
    for element in path.elements() {
      let (tag, points): (u8, Vec<Point>) = match *element {
        PathEl::MoveTo(point) => (1, vec![point]),
        PathEl::LineTo(point) => (2, vec![point]),
        PathEl::QuadTo(control, end) => (
          3,
          vec![
            current.lerp(control, 2.0 / 3.0),
            end.lerp(control, 2.0 / 3.0),
            end,
          ],
        ),
        PathEl::CurveTo(a, b, end) => (3, vec![a, b, end]),
        PathEl::ClosePath => (4, Vec::new()),
      };
      if let Some(&point) = points.last() {
        current = point;
      }
      for byte in std::iter::once(tag).chain(points.iter().flat_map(|p| {
        [p.x, p.y].into_iter().flat_map(|v| {
          ((v * DESIGN_EM as f64 / f64::from(units_per_em) * 384.0).round() as i64).to_le_bytes()
        })
      })) {
        hash = (hash ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3);
      }
    }
    hash
  }

  #[test]
  fn gdi_synthetic_bold_matches_native_simple_and_composite_contours() {
    // Actual GetGlyphOutlineW(GGO_NATIVE | GGO_UNHINTED), 2048em,
    // weight700. Whole-contour fingerprints include off-curve starts,
    // composite ROUND_XY_TO_GRID placement, holes and clamped sharp joins.
    // These are native observations, not fingerprints of this implementation.
    let cases = [
      ("Monotype Corsiva", 'I', 0x473c_b431_4f5f_8d26),
      ("Monotype Corsiva", 'Ю', 0xa419_3374_c884_4a8c),
      ("Monotype Corsiva", 'Ы', 0x9c28_7f50_fc9e_3a39),
      ("Monotype Corsiva", 'Ш', 0x87d6_a98d_c692_4d15),
      ("Gabriola", 'I', 0xf035_0647_4d24_8d52),
      ("Gabriola", 'Ю', 0xc584_df0f_cd7f_154e),
      ("Gabriola", 'Ы', 0x4728_5ee1_8dc4_fe2c),
      ("Gabriola", 'Ш', 0x33b5_ccec_f1d4_3fa4),
      ("Ink Free", 'I', 0x1ae1_504a_2200_5298),
      ("Ink Free", 'Ю', 0x6b3e_4708_d4e0_e456),
      ("Ink Free", 'Ы', 0xf348_b3d2_418d_b430),
      ("Ink Free", 'Ш', 0x0604_2e55_a278_e069),
    ];
    let mut metrics = TextMetrics::new();
    for (family, character, expected) in cases {
      for size in [12.0, 36.0] {
        let style = TextStyle {
          font_family: Some(Arc::from(family)),
          font_size_pt: size,
          bold: true,
          ..Default::default()
        };
        let shaped = metrics.shape_text(&character.to_string(), &style).unwrap();
        let glyph = &shaped.glyphs[0];
        let face = &shaped.font_faces[glyph.font_index];
        assert!(face.synthetic_bold, "{family}");
        let units_per_em = FontRef::from_index(face.data.as_ref(), face.index)
          .unwrap()
          .head()
          .unwrap()
          .units_per_em();
        let path = metrics
          .gdi_synthetic_bold_outline(face, glyph.glyph_id)
          .unwrap();
        assert_eq!(
          native_path_fingerprint(path, units_per_em),
          expected,
          "{family}/{character}"
        );
      }
    }
    assert_eq!(metrics.gdi_bold_outlines.len(), cases.len());
  }

  #[test]
  fn gdi_synthetic_bold_keeps_duplicate_contours_linear_and_bounded() {
    let half = GridPoint { x: 1312, y: 1280 };
    let mut degenerate = vec![GridPoint::default(); MAX_POINTS];
    embolden(&mut degenerate, half);
    assert!(
      degenerate
        .iter()
        .all(|point| *point == GridPoint::default())
    );
    let original = [
      GridPoint { x: 0, y: 0 },
      GridPoint { x: 0, y: 6400 },
      GridPoint { x: 6400, y: 0 },
      GridPoint { x: 0, y: 0 },
    ];
    let mut points = original;
    embolden(&mut points, half);
    assert_eq!(points[0], points[3]);
    for (before, after) in original.into_iter().zip(points) {
      assert!((0..=half.x * 2).contains(&(after.x - before.x)));
      assert!((0..=half.y * 2).contains(&(after.y - before.y)));
    }
  }
}
