//! Word transports cubic controls along the interpolated boundary tangent.
//! Mapping controls as ordinary points does not preserve that direction.

use kurbo::{
  BezPath, CubicBez, ParamCurve, ParamCurveArclen, ParamCurveDeriv, PathEl, PathSeg, Point, Vec2,
};

use super::{TextWarpBoundary, checked_point, split_outline_at_warp_seams};
use crate::common::{PathCommand, TextWarp};

pub fn append_word_envelope_outline(
  path: &BezPath,
  glyph_to_page: impl Fn(Point) -> Point,
  warp: &TextWarp,
  boundaries: &[TextWarpBoundary],
  commands: &mut Vec<PathCommand>,
) -> Result<(), &'static str> {
  let [upper, lower] = boundaries else {
    return Err("Word envelope requires two boundaries");
  };
  let source = warp.source_bounds;
  let normalize = |point| {
    let point = glyph_to_page(point);
    Point::new(
      f64::from(((point.x - f64::from(source.origin.x.0)) / f64::from(source.size.width.0)) as f32),
      f64::from(
        ((point.y - f64::from(source.origin.y.0)) / f64::from(source.size.height.0)) as f32,
      ),
    )
  };
  let mut normalized = BezPath::new();
  let mut current = Point::ZERO;
  let mut start = current;
  for &element in path.elements() {
    match element {
      PathEl::MoveTo(p) => {
        normalized.move_to(normalize(p));
        current = p;
        start = p;
      }
      PathEl::LineTo(p) => {
        normalized.line_to(normalize(p));
        current = p;
      }
      PathEl::QuadTo(control, end) => {
        // GGO's quadratic outlines become cubics before the source-plane
        // transform and FLOAT normalization. Keep that order here too.
        let c1 = Point::new(
          (current.x + 2.0 * control.x) / 3.0,
          (current.y + 2.0 * control.y) / 3.0,
        );
        let c2 = Point::new(
          (end.x + 2.0 * control.x) / 3.0,
          (end.y + 2.0 * control.y) / 3.0,
        );
        normalized.curve_to(normalize(c1), normalize(c2), normalize(end));
        current = end;
      }
      PathEl::CurveTo(a, b, end) => {
        normalized.curve_to(normalize(a), normalize(b), normalize(end));
        current = end;
      }
      PathEl::ClosePath => {
        if current != start {
          normalized.line_to(normalize(start));
        }
        normalized.close_path();
        current = start;
      }
    }
  }
  let mut seams = Vec::new();
  for boundary in boundaries {
    if boundary.total > f64::EPSILON {
      seams.extend(
        boundary.ends[..boundary.ends.len() - 1]
          .iter()
          .map(|end| end / boundary.total),
      );
    }
  }
  seams.sort_by(f64::total_cmp);
  seams.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
  let split = split_outline_at_warp_seams(&normalized, |p| p, &seams);
  append_normalized(
    split.as_ref().unwrap_or(&normalized),
    upper,
    lower,
    commands,
  )
}

fn append_normalized(
  path: &BezPath,
  upper: &TextWarpBoundary,
  lower: &TextWarpBoundary,
  commands: &mut Vec<PathCommand>,
) -> Result<(), &'static str> {
  let project = |p: Point| {
    let a = upper.sample(p.x);
    let b = lower.sample(p.x);
    Point::new((1.0 - p.y) * a.x + p.y * b.x, (1.0 - p.y) * a.y + p.y * b.y)
  };
  let correct = |p: Point, c: Point, mapped_p: Point, mapped_c: Point| {
    let delta = c - p;
    let a = upper.sample(p.x);
    let b = lower.sample(p.x);
    let upper_angle = tangent(upper, p.x, delta.x).atan2();
    let lower_angle = tangent(lower, p.x, delta.x).atan2();
    let mut angle_delta = lower_angle - upper_angle;
    if angle_delta > std::f64::consts::PI {
      angle_delta -= std::f64::consts::TAU;
    }
    if angle_delta < -std::f64::consts::PI {
      angle_delta += std::f64::consts::TAU;
    }
    // Native GFX interpolates angles and lengths separately, rather than
    // interpolating tangent vectors (the latter is a different Jacobian).
    let angle = upper_angle + angle_delta * p.y;
    let length = upper.total + (lower.total - upper.total) * p.y;
    let direction = Vec2::from_angle(angle) * length * delta.x + (b - a) * delta.y;
    let square = direction.hypot2();
    let projection = (mapped_c - mapped_p).dot(direction);
    if square > f64::from(f32::EPSILON) && projection > 0.0 {
      mapped_p + direction * (projection / square)
    } else {
      mapped_p
    }
  };
  let mut current = Point::ZERO;
  let mut first = current;
  for &element in path.elements() {
    let cubic = match element {
      PathEl::MoveTo(p) => {
        commands.push(PathCommand::MoveTo(checked_point(project(p))?));
        current = p;
        first = p;
        continue;
      }
      PathEl::LineTo(end) => CubicBez::new(
        current,
        current.lerp(end, 1.0 / 3.0),
        current.lerp(end, 2.0 / 3.0),
        end,
      ),
      PathEl::CurveTo(a, b, end) => CubicBez::new(current, a, b, end),
      PathEl::ClosePath => {
        commands.push(PathCommand::Close);
        current = first;
        continue;
      }
      PathEl::QuadTo(..) => return Err("Word envelope source must be cubic"),
    };
    let [a, b, c, d] = [cubic.p0, cubic.p1, cubic.p2, cubic.p3].map(project);
    commands.push(PathCommand::CubicTo {
      control1: checked_point(correct(cubic.p0, cubic.p1, a, b))?,
      control2: checked_point(correct(cubic.p3, cubic.p2, d, c))?,
      end: checked_point(d)?,
    });
    current = cubic.p3;
  }
  Ok(())
}

fn tangent(boundary: &TextWarpBoundary, position: f64, side: f64) -> Vec2 {
  let target = position.clamp(0.0, 1.0) * boundary.total;
  let index = boundary
    .ends
    .partition_point(|&end| {
      if side > 0.0 {
        end <= target
      } else {
        end < target
      }
    })
    .min(boundary.segments.len() - 1);
  let start = if index == 0 {
    0.0
  } else {
    boundary.ends[index - 1]
  };
  let segment = boundary.segments[index];
  let t = segment.inv_arclen(
    (target - start).max(0.0),
    super::TEXT_WARP_ARCLEN_ACCURACY_PT,
  );
  match segment {
    PathSeg::Line(line) => line.p1 - line.p0,
    PathSeg::Quad(quad) => quad.deriv().eval(t).to_vec2(),
    PathSeg::Cubic(cubic) => cubic.deriv().eval(t).to_vec2(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::common::{Point as LayoutPoint, Pt};
  fn boundary(points: &[(f32, f32)]) -> TextWarpBoundary {
    let commands: Vec<_> = points
      .iter()
      .enumerate()
      .map(|(i, &(x, y))| {
        let p = LayoutPoint { x: Pt(x), y: Pt(y) };
        if i == 0 {
          PathCommand::MoveTo(p)
        } else {
          PathCommand::LineTo(p)
        }
      })
      .collect();
    TextWarpBoundary::new(&commands).unwrap()
  }

  #[test]
  fn word_envelope_transports_controls_along_native_boundary_angles() {
    // Native GFX observed source/target, two independently realized font
    // sizes. The first horizontal line becomes a cubic because angle
    // interpolation differs from the straight mapped chord.
    let upper = boundary(&[(63.0, 642.5), (1525.0, 33.0), (2987.0, 642.5)]);
    let lower = boundary(&[(63.0, 1252.0), (2987.0, 1252.0)]);
    for (bounds, a, b, expected) in [
      (
        [
          63.87892282009125,
          80.046875,
          996.9999389648438,
          208.5869140625,
        ],
        [66.73538208007812, 87.224609375],
        [133.53225708007812, 87.224609375],
        [
          [141.77810668945312, 645.6925659179688],
          [211.43641662597656, 617.94189453125],
        ],
      ),
      (
        [
          64.75784170627594,
          117.09375,
          1932.0000000000002,
          374.173828125,
        ],
        [70.47073364257812, 131.44921875],
        [204.06448364257812, 131.44921875],
        [
          [141.73587036132812, 645.7091064453125],
          [211.3568572998047, 617.9732666015625],
        ],
      ),
    ] {
      let normalized = |p: [f64; 2]| {
        Point::new(
          f64::from(((p[0] - bounds[0]) / (bounds[2] - bounds[0])) as f32),
          f64::from(((p[1] - bounds[1]) / (bounds[3] - bounds[1])) as f32),
        )
      };
      let mut path = BezPath::new();
      path.move_to(normalized(a));
      path.line_to(normalized(b));
      let mut commands = Vec::new();
      append_normalized(&path, &upper, &lower, &mut commands).unwrap();
      let PathCommand::CubicTo {
        control1, control2, ..
      } = commands[1]
      else {
        panic!("line must become a cubic")
      };
      for (actual, expected) in [control1, control2].into_iter().zip(expected) {
        assert!((f64::from(actual.x.0) - expected[0]).abs() < 1e-5);
        assert!((f64::from(actual.y.0) - expected[1]).abs() < 1e-5);
      }
    }
  }

  #[test]
  fn word_envelope_keeps_affine_cubic_geometry() {
    let upper = boundary(&[(10.0, 20.0), (110.0, 20.0)]);
    let lower = boundary(&[(10.0, 70.0), (110.0, 70.0)]);
    let mut path = BezPath::new();
    path.move_to((0.1, 0.2));
    path.curve_to((0.3, 0.9), (0.7, 0.1), (0.9, 0.8));
    let mut commands = Vec::new();
    append_normalized(&path, &upper, &lower, &mut commands).unwrap();
    let PathCommand::CubicTo {
      control1,
      control2,
      end,
    } = commands[1]
    else {
      panic!("missing cubic")
    };
    for (point, expected) in
      [control1, control2, end]
        .into_iter()
        .zip([(40.0, 65.0), (80.0, 25.0), (100.0, 60.0)])
    {
      assert!((point.x.0 - expected.0).abs() < 1e-5);
      assert!((point.y.0 - expected.1).abs() < 1e-5);
    }
  }

  #[test]
  fn word_envelope_splits_and_transports_the_closing_edge() {
    let upper = boundary(&[(0.0, 50.0), (50.0, 0.0), (100.0, 50.0)]);
    let lower = boundary(&[(0.0, 100.0), (100.0, 100.0)]);
    let source_bounds = crate::common::Rect {
      origin: LayoutPoint {
        x: Pt(0.0),
        y: Pt(0.0),
      },
      size: crate::common::Size {
        width: Pt(1.0),
        height: Pt(1.0),
      },
    };
    let warp = TextWarp {
      source_bounds,
      paint_bounds: source_bounds,
      boundaries: Vec::new(),
      word_envelope: true,
      follow_path: None,
      vml_fit_path: None,
      vml_trim_band: None,
    };
    let mut path = BezPath::new();
    path.move_to((0.25, 0.0));
    path.line_to((0.75, 0.0));
    path.close_path();
    let mut commands = Vec::new();
    append_word_envelope_outline(&path, |p| p, &warp, &[upper, lower], &mut commands).unwrap();
    assert_eq!(commands.len(), 6);
    assert!(matches!(commands[5], PathCommand::Close));
    for index in [1, 3] {
      let PathCommand::CubicTo { end, .. } = commands[index] else {
        panic!("missing seam")
      };
      assert_eq!((end.x.0, end.y.0), (50.0, 0.0));
    }
  }
}
