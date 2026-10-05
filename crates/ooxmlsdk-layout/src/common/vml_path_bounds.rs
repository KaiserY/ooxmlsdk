//! The legacy Office path-measurement enclosure, independent of painted joins.
//!
//! Native WordArt observations (MSO's path bounds collector) include cubic
//! control points and axis-projected miter bounds before the final pen outset.
//! A round painted join therefore does not imply a round layout enclosure.

use kurbo::{BezPath, PathEl, Point, Rect};

/// Measure in the caller's device coordinates. Office uses FLOAT path points
/// and DOUBLE normalized tangents; preserving that distinction also avoids
/// unstable slopes at a nearly horizontal or vertical join.
pub(super) fn stroked_bounds(path: &BezPath, width: f32, miter_limit: f32) -> Option<Rect> {
  let mut walker = BoundsWalker {
    bounds: [
      f32::INFINITY,
      f32::INFINITY,
      f32::NEG_INFINITY,
      f32::NEG_INFINITY,
    ],
    half: width / 2.0,
    miter_extent: width * miter_limit,
    start: [0.0; 2],
    last: [0.0; 2],
    incoming: [0.0; 2],
    outgoing: None,
  };
  let point = |p: Point| [p.x as f32, p.y as f32];
  for element in path.elements() {
    match *element {
      PathEl::MoveTo(p) => {
        let p = point(p);
        walker.start = p;
        walker.last = p;
        walker.incoming = p;
        walker.outgoing = None;
        walker.include(p);
      }
      PathEl::LineTo(p) => walker.line_to(point(p)),
      PathEl::QuadTo(control, end) => {
        let start = Point::new(f64::from(walker.last[0]), f64::from(walker.last[1]));
        walker.curve_to(
          point(start.lerp(control, 2.0 / 3.0)),
          point(end.lerp(control, 2.0 / 3.0)),
          point(end),
        );
      }
      PathEl::CurveTo(a, b, end) => walker.curve_to(point(a), point(b), point(end)),
      PathEl::ClosePath => {
        walker.line_to(walker.start);
        if let Some(outgoing) = walker.outgoing {
          walker.join(walker.incoming, walker.start, outgoing);
        }
      }
    }
  }
  let [left, top, right, bottom] = walker.bounds;
  [left, top, right, bottom]
    .into_iter()
    .all(f32::is_finite)
    .then(|| {
      Rect::new(
        f64::from(left - walker.half),
        f64::from(top - walker.half),
        f64::from(right + walker.half),
        f64::from(bottom + walker.half),
      )
    })
}

struct BoundsWalker {
  bounds: [f32; 4],
  half: f32,
  miter_extent: f32,
  start: [f32; 2],
  last: [f32; 2],
  incoming: [f32; 2],
  outgoing: Option<[f32; 2]>,
}

impl BoundsWalker {
  fn include(&mut self, p: [f32; 2]) {
    for (axis, value) in p.into_iter().enumerate() {
      self.bounds[axis] = self.bounds[axis].min(value);
      self.bounds[axis + 2] = self.bounds[axis + 2].max(value);
    }
  }

  fn line_to(&mut self, p: [f32; 2]) {
    if p == self.last {
      return;
    }
    self.include(p);
    if self.outgoing.is_some() {
      self.join(self.incoming, self.last, p);
      self.incoming = self.last;
    } else {
      self.outgoing = Some(p);
    }
    self.last = p;
  }

  fn curve_to(&mut self, a: [f32; 2], b: [f32; 2], end: [f32; 2]) {
    self.line_to(a);
    self.include(b);
    self.include(end);
    self.incoming = if b == end { a } else { b };
    self.last = end;
    if self.outgoing.is_none() {
      self.outgoing = Some(if self.incoming == self.start {
        end
      } else {
        self.incoming
      });
    }
  }

  fn join(&mut self, previous: [f32; 2], p: [f32; 2], next: [f32; 2]) {
    let mut u = [f64::from(p[0] - previous[0]), f64::from(p[1] - previous[1])];
    let mut v = [f64::from(p[0] - next[0]), f64::from(p[1] - next[1])];
    if u[0] * v[1] == v[0] * u[1] {
      return;
    }
    let ul = u[0].hypot(u[1]);
    let vl = v[0].hypot(v[1]);
    u = [u[0] / ul, u[1] / ul];
    v = [v[0] / vl, v[1] / vl];
    let half = f64::from(self.half);
    let numerator = half * (v[0] * u[1] + v[1] * u[0]);
    for axis in 0..2 {
      if u[axis] * v[axis] <= 0.0 {
        continue;
      }
      let denominator = if axis == 0 { u[0] - v[0] } else { v[1] - u[1] };
      let alternate = if axis == 0 { v[1] } else { u[0] };
      let extension = if denominator != 0.0 {
        (numerator / denominator).abs()
      } else if alternate != 0.0 {
        (half / alternate).abs()
      } else {
        0.0
      };
      if extension > f64::from(self.miter_extent) {
        continue;
      }
      // These are the native collector's axis projections, not the extrema
      // of a stroked outline. Both axes extend in the incoming direction.
      let positive = u[axis] > 0.0;
      let value = (f64::from(p[axis]) + if positive { extension } else { -extension }) as f32;
      if positive {
        self.bounds[axis + 2] = self.bounds[axis + 2].max(value);
      } else {
        self.bounds[axis] = self.bounds[axis].min(value);
      }
    }
  }
}
