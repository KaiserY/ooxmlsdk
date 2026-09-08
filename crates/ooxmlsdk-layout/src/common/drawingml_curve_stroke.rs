//! Single-precision curve-preserving path widening for text contour masks.
//!
//! Office's fixed-output text pipeline widens the original glyph curves before
//! the 24.8 cell/area raster boundary. Generic double-precision offsetters do
//! not preserve that contract: in particular, curve pieces created to keep an
//! offset stable must reconnect with a round continuation, while authored
//! vertices retain their requested join. This module implements that narrow
//! centered-stroke contract in safe Rust without an external raster/stroke
//! engine.

use std::ops::{Add, Mul, Sub};

use kurbo::{BezPath, PathEl};

const MERGE_EPSILON: f32 = 0.01;
const CURVE_TANGENT_EPSILON: f32 = 0.5;
const DEFAULT_MITER_LIMIT: f32 = 4.0;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Point {
  x: f32,
  y: f32,
}

impl Point {
  const ZERO: Self = Self { x: 0.0, y: 0.0 };

  const fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }

  fn from_kurbo(point: kurbo::Point) -> Option<Self> {
    let x = point.x as f32;
    let y = point.y as f32;
    (x.is_finite() && y.is_finite()).then_some(Self::new(x, y))
  }

  fn is_finite(self) -> bool {
    self.x.is_finite() && self.y.is_finite()
  }

  fn length(self) -> f32 {
    (self.x * self.x + self.y * self.y).sqrt()
  }

  fn dot(self, other: Self) -> f32 {
    self.x * other.x + self.y * other.y
  }

  fn normalize(self) -> Self {
    let length = self.length();
    if length == 0.0 {
      Self::ZERO
    } else {
      self * (1.0 / length)
    }
  }

  fn nearly_eq(self, other: Self) -> bool {
    self.nearly_eq_by(other, f32::EPSILON)
  }

  fn nearly_eq_by(self, other: Self, epsilon: f32) -> bool {
    (self.x - other.x).abs() < epsilon && (self.y - other.y).abs() < epsilon
  }
}

impl Add for Point {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self::new(self.x + other.x, self.y + other.y)
  }
}

impl Sub for Point {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self::new(self.x - other.x, self.y - other.y)
  }
}

impl Mul<f32> for Point {
  type Output = Self;

  fn mul(self, scalar: f32) -> Self {
    Self::new(self.x * scalar, self.y * scalar)
  }
}

fn normal(start: Point, end: Point) -> Point {
  Point::new(end.y - start.y, -(end.x - start.x)).normalize()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Cubic {
  a: Point,
  b: Point,
  c: Point,
  d: Point,
}

impl Cubic {
  const fn new(a: Point, b: Point, c: Point, d: Point) -> Self {
    Self { a, b, c, d }
  }

  fn from_quadratic(a: Point, b: Point, c: Point) -> Self {
    Self {
      a,
      b: Point::new(a.x + 2.0 / 3.0 * (b.x - a.x), a.y + 2.0 / 3.0 * (b.y - a.y)),
      c: Point::new(c.x + 2.0 / 3.0 * (b.x - c.x), c.y + 2.0 / 3.0 * (b.y - c.y)),
      d: c,
    }
  }

  fn reverse(self) -> Self {
    Self::new(self.d, self.c, self.b, self.a)
  }

  fn is_line(self, tolerance: f32) -> bool {
    let ab = self.a.nearly_eq_by(self.b, tolerance);
    let bc = self.b.nearly_eq_by(self.c, tolerance);
    let cd = self.c.nearly_eq_by(self.d, tolerance);
    u8::from(ab) + u8::from(bc) + u8::from(cd) >= 2
  }

  fn slice(self, start: f32, end: f32) -> Self {
    let t0 = start;
    let t1 = end;
    let u0 = 1.0 - t0;
    let u1 = 1.0 - t1;
    let v0 = self.a;
    let v1 = self.b;
    let v2 = self.c;
    let v3 = self.d;
    Self::new(
      (v0 * (u0 * u0 * u0))
        + (v1 * (t0 * u0 * u0 + u0 * t0 * u0 + u0 * u0 * t0))
        + (v2 * (t0 * t0 * u0 + u0 * t0 * t0 + t0 * u0 * t0))
        + (v3 * (t0 * t0 * t0)),
      (v0 * (u0 * u0 * u1))
        + (v1 * (t0 * u0 * u1 + u0 * t0 * u1 + u0 * u0 * t1))
        + (v2 * (t0 * t0 * u1 + u0 * t0 * t1 + t0 * u0 * t1))
        + (v3 * (t0 * t0 * t1)),
      (v0 * (u0 * u1 * u1))
        + (v1 * (t0 * u1 * u1 + u0 * t1 * u1 + u0 * u1 * t1))
        + (v2 * (t0 * t1 * u1 + u0 * t1 * t1 + t0 * u1 * t1))
        + (v3 * (t0 * t1 * t1)),
      (v0 * (u1 * u1 * u1))
        + (v1 * (t1 * u1 * u1 + u1 * t1 * u1 + u1 * u1 * t1))
        + (v2 * (t1 * t1 * u1 + u1 * t1 * t1 + t1 * u1 * t1))
        + (v3 * (t1 * t1 * t1)),
    )
  }

  fn split(self, time: f32) -> (Self, Self) {
    (self.slice(0.0, time), self.slice(time, 1.0))
  }

  fn split_at_max_curvature(self, output: &mut [Self; 4]) -> usize {
    let mut roots = [0.0_f32; 3];
    let root_count = self.max_curvature(&mut roots);
    let mut times = [0.0_f32; 4];
    let mut count = 0;
    for &time in &roots[..root_count] {
      if time > 0.0 && time < 1.0 {
        times[count] = time;
        count += 1;
      }
    }
    if count == 0 {
      output[0] = self;
    } else {
      let mut output_index = 0;
      let mut last_time = 0.0;
      for &time in &times[..count] {
        output[output_index] = self.slice(last_time, time);
        output_index += 1;
        last_time = time;
      }
      output[output_index] = self.slice(last_time, 1.0);
    }
    count + 1
  }

  fn max_curvature(self, roots: &mut [f32; 3]) -> usize {
    fn coefficients(values: [f32; 4]) -> [f32; 4] {
      let a = values[1] - values[0];
      let b = values[2] - 2.0 * values[1] + values[0];
      let c = values[3] + 3.0 * (values[1] - values[2]) - values[0];
      [c * c, 3.0 * b * c, 2.0 * b * b + c * a, a * b]
    }

    let mut combined = coefficients([self.a.x, self.b.x, self.c.x, self.d.x]);
    let y = coefficients([self.a.y, self.b.y, self.c.y, self.d.y]);
    for index in 0..combined.len() {
      combined[index] += y[index];
    }
    solve_cubic(combined, roots)
  }

  fn needs_stable_offset_split(self) -> bool {
    if self.b.nearly_eq_by(self.c, MERGE_EPSILON) {
      return true;
    }
    const FLAT_ENOUGH: f32 = std::f32::consts::SQRT_2 / 2.0 + 0.1;
    let normal_ab = normal(self.a, self.b);
    let normal_bc = normal(self.b, self.c);
    normal_ab.dot(normal_bc) <= FLAT_ENOUGH || normal_bc.dot(normal(self.c, self.d)) <= FLAT_ENOUGH
  }
}

fn solve_cubic(coefficients: [f32; 4], roots: &mut [f32; 3]) -> usize {
  let inverse = 1.0 / coefficients[0];
  let a = coefficients[1] * inverse;
  let b = coefficients[2] * inverse;
  let c = coefficients[3] * inverse;
  let q = (a * a - b * 3.0) / 9.0;
  let r = (2.0 * a * a * a - 9.0 * a * b + 27.0 * c) / 54.0;
  let q_cubed = q * q * q;
  let discriminant = r * r - q_cubed;
  let a_third = a / 3.0;
  if discriminant < 0.0 {
    let theta = clamp_unit(r / q_cubed.sqrt()).acos();
    let negative_two_root_q = -2.0 * q.sqrt();
    roots[0] = clamp_unit(negative_two_root_q * (theta / 3.0).cos() - a_third);
    roots[1] = clamp_unit(
      negative_two_root_q * ((theta + 2.0 * std::f32::consts::PI) / 3.0).cos() - a_third,
    );
    roots[2] = clamp_unit(
      negative_two_root_q * ((theta - 2.0 * std::f32::consts::PI) / 3.0).cos() - a_third,
    );
    roots
      .sort_unstable_by(|left, right| left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Less));
    let mut count = 3;
    if roots[0] == roots[1] {
      roots[1] = roots[2];
      count -= 1;
    }
    if roots[1] == roots[2] {
      count -= 1;
    }
    count
  } else {
    let mut root = r.abs() + discriminant.sqrt();
    root = root.powf(0.333_333_3);
    if r > 0.0 {
      root = -root;
    }
    if root != 0.0 {
      root += q / root;
    }
    roots[0] = clamp_unit(root - a_third);
    1
  }
}

fn clamp_unit(value: f32) -> f32 {
  value.clamp(0.0, 1.0)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SegmentGeometry {
  Line { a: Point, b: Point },
  Curve(Cubic),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Segment {
  id: u8,
  geometry: SegmentGeometry,
}

impl Segment {
  const fn line(id: u8, a: Point, b: Point) -> Self {
    Self {
      id,
      geometry: SegmentGeometry::Line { a, b },
    }
  }

  const fn curve(id: u8, curve: Cubic) -> Self {
    Self {
      id,
      geometry: SegmentGeometry::Curve(curve),
    }
  }

  fn reverse(self) -> Self {
    match self.geometry {
      SegmentGeometry::Line { a, b } => Self::line(self.id, b, a),
      SegmentGeometry::Curve(curve) => Self::curve(self.id, curve.reverse()),
    }
  }

  fn offset(self, radius: f32) -> OffsetSegment {
    match self.geometry {
      SegmentGeometry::Line { a, b } => {
        let direction = normal(a, b);
        let offset = direction * radius;
        let start = a + offset;
        let end = b + offset;
        OffsetSegment {
          segment: Self::line(self.id, start, end),
          id: self.id,
          start,
          end,
          start_normal: direction,
          end_normal: direction,
          end_pivot: b,
        }
      }
      SegmentGeometry::Curve(curve) => {
        let normal_ab = if curve.a.nearly_eq_by(curve.b, CURVE_TANGENT_EPSILON) {
          if curve.a.nearly_eq_by(curve.c, CURVE_TANGENT_EPSILON) {
            normal(curve.a, curve.d)
          } else {
            normal(curve.a, curve.c)
          }
        } else {
          normal(curve.a, curve.b)
        };
        let normal_bc = if curve.b.nearly_eq_by(curve.c, CURVE_TANGENT_EPSILON) {
          if curve.b.nearly_eq_by(curve.d, CURVE_TANGENT_EPSILON) {
            normal(curve.a, curve.d)
          } else {
            normal(curve.b, curve.d)
          }
        } else {
          normal(curve.b, curve.c)
        };
        let normal_cd = if curve.c.nearly_eq_by(curve.d, CURVE_TANGENT_EPSILON) {
          if curve.b.nearly_eq_by(curve.d, CURVE_TANGENT_EPSILON) {
            normal(curve.a, curve.d)
          } else {
            normal(curve.b, curve.d)
          }
        } else {
          normal(curve.c, curve.d)
        };
        let normal_b = (normal_ab + normal_bc).normalize()
          * (radius / ((1.0 + normal_ab.dot(normal_bc)) * 0.5).sqrt());
        let normal_c = (normal_cd + normal_bc).normalize()
          * (radius / ((1.0 + normal_cd.dot(normal_bc)) * 0.5).sqrt());
        let start = curve.a + normal_ab * radius;
        let end = curve.d + normal_cd * radius;
        OffsetSegment {
          segment: Self::curve(
            self.id,
            Cubic::new(start, curve.b + normal_b, curve.c + normal_c, end),
          ),
          id: self.id,
          start,
          end,
          start_normal: normal_ab,
          end_normal: normal_cd,
          end_pivot: curve.d,
        }
      }
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct OffsetSegment {
  segment: Segment,
  id: u8,
  start: Point,
  end: Point,
  start_normal: Point,
  end_normal: Point,
  end_pivot: Point,
}

#[derive(Debug)]
struct Subpath {
  segments: Vec<Segment>,
  closed: bool,
}

fn simplified_subpaths(path: &BezPath) -> Option<Vec<Subpath>> {
  let mut subpaths = Vec::new();
  let mut segments = Vec::new();
  let mut start = Point::ZERO;
  let mut current = Point::ZERO;
  let mut has_current = false;
  let mut id = 0_u8;

  let finish_open = |subpaths: &mut Vec<Subpath>, segments: &mut Vec<Segment>| {
    if !segments.is_empty() {
      subpaths.push(Subpath {
        segments: std::mem::take(segments),
        closed: false,
      });
    }
  };

  for element in path.iter() {
    id = if id == 254 { 0 } else { id + 1 };
    match element {
      PathEl::MoveTo(point) => {
        finish_open(&mut subpaths, &mut segments);
        current = Point::from_kurbo(point)?;
        start = current;
        has_current = true;
      }
      PathEl::LineTo(point) => {
        let point = Point::from_kurbo(point)?;
        if has_current && !current.nearly_eq_by(point, MERGE_EPSILON) {
          segments.push(Segment::line(id, current, point));
          current = point;
        }
      }
      PathEl::QuadTo(control, point) => {
        let control = Point::from_kurbo(control)?;
        let point = Point::from_kurbo(point)?;
        if has_current {
          push_simplified_curve(
            &mut segments,
            id,
            Cubic::from_quadratic(current, control, point),
          );
          current = point;
        }
      }
      PathEl::CurveTo(control1, control2, point) => {
        let control1 = Point::from_kurbo(control1)?;
        let control2 = Point::from_kurbo(control2)?;
        let point = Point::from_kurbo(point)?;
        if has_current {
          push_simplified_curve(
            &mut segments,
            id,
            Cubic::new(current, control1, control2, point),
          );
          current = point;
        }
      }
      PathEl::ClosePath => {
        if has_current {
          if segments.is_empty() || !current.nearly_eq_by(start, MERGE_EPSILON) {
            segments.push(Segment::line(id, current, start));
          }
          if !segments.is_empty() {
            subpaths.push(Subpath {
              segments: std::mem::take(&mut segments),
              closed: true,
            });
          }
          current = start;
        }
      }
    }
  }
  finish_open(&mut subpaths, &mut segments);
  Some(subpaths)
}

fn push_simplified_curve(segments: &mut Vec<Segment>, id: u8, curve: Cubic) {
  if curve.is_line(MERGE_EPSILON) {
    if !curve.a.nearly_eq_by(curve.d, MERGE_EPSILON) {
      segments.push(Segment::line(id, curve.a, curve.d));
    }
    return;
  }

  let mut curvature_splits = [Cubic::new(Point::ZERO, Point::ZERO, Point::ZERO, Point::ZERO); 4];
  let curvature_count = curve.split_at_max_curvature(&mut curvature_splits);
  let mut stable_splits = Vec::with_capacity(16);
  for &piece in &curvature_splits[..curvature_count] {
    if piece.needs_stable_offset_split() {
      let (first, second) = piece.split(0.5);
      for half in [first, second] {
        if half.needs_stable_offset_split() {
          let (first_quarter, second_quarter) = half.split(0.5);
          stable_splits.push(first_quarter);
          stable_splits.push(second_quarter);
        } else {
          stable_splits.push(half);
        }
      }
    } else {
      stable_splits.push(piece);
    }
  }
  for piece in stable_splits {
    if piece.is_line(MERGE_EPSILON) {
      if !piece.a.nearly_eq_by(piece.d, MERGE_EPSILON) {
        segments.push(Segment::line(id, piece.a, piece.d));
      }
    } else {
      segments.push(Segment::curve(id, piece));
    }
  }
}

struct StrokeSink {
  path: BezPath,
  valid: bool,
}

impl StrokeSink {
  fn new() -> Self {
    Self {
      path: BezPath::new(),
      valid: true,
    }
  }

  fn move_to(&mut self, point: Point) {
    if point.is_finite() {
      self.path.move_to((f64::from(point.x), f64::from(point.y)));
    } else {
      self.valid = false;
    }
  }

  fn line_to(&mut self, point: Point) {
    if point.is_finite() {
      self.path.line_to((f64::from(point.x), f64::from(point.y)));
    } else {
      self.valid = false;
    }
  }

  fn curve_to(&mut self, first: Point, second: Point, end: Point) {
    if first.is_finite() && second.is_finite() && end.is_finite() {
      self.path.curve_to(
        (f64::from(first.x), f64::from(first.y)),
        (f64::from(second.x), f64::from(second.y)),
        (f64::from(end.x), f64::from(end.y)),
      );
    } else {
      self.valid = false;
    }
  }

  fn close(&mut self) {
    self.path.close_path();
  }
}

/// Widens a path into a centered miter-joined outline with butt end caps.
///
/// The computation intentionally takes place in `f32`: that is the numeric
/// boundary used by Office's glyph-mask input and by the subsequent 24.8 area
/// rasterizer. Authored vertices use a miter limit of four. Artificial pieces
/// of one cubic reconnect through a circular continuation so subdivision does
/// not introduce a seam into the widened curve.
pub(super) fn widen_centered_miter_butt(path: &BezPath, width: f32) -> Option<BezPath> {
  if path.is_empty() || !width.is_finite() || width <= 0.0 {
    return None;
  }
  let radius = width.max(0.01) * 0.5;
  let mut sink = StrokeSink::new();
  for subpath in simplified_subpaths(path)? {
    stroke_subpath(&subpath.segments, subpath.closed, radius, &mut sink);
  }
  (sink.valid && !sink.path.is_empty()).then_some(sink.path)
}

fn stroke_subpath(segments: &[Segment], closed: bool, radius: f32, sink: &mut StrokeSink) {
  let Some(last_source) = segments.last() else {
    return;
  };
  let mut last_normal = Point::ZERO;
  let mut first_point = Point::ZERO;
  let mut last_point = Point::ZERO;
  let mut pivot = Point::ZERO;
  let mut last_id = u8::MAX;

  if closed {
    let last = last_source.offset(radius);
    last_normal = last.end_normal;
    pivot = last.end_pivot;
    last_point = last.end;
    first_point = last.end;
    sink.move_to(last_point);
  }

  let mut first = !closed;
  for &source in segments {
    let segment = source.offset(radius);
    if first {
      sink.move_to(segment.start);
      first_point = segment.start;
      first = false;
    } else {
      add_join(
        sink,
        last_point,
        segment.start,
        pivot,
        last_normal,
        segment.start_normal,
        radius,
        false,
      );
    }
    last_id = segment.id;
    last_normal = segment.end_normal;
    pivot = segment.end_pivot;
    last_point = emit_segment(sink, segment.segment);
  }

  first = true;
  for &source in segments.iter().rev() {
    let segment = source.reverse().offset(radius);
    if first {
      if closed {
        let initial = segments[0].reverse().offset(radius);
        last_point = initial.end;
        last_normal = initial.end_normal;
        pivot = initial.end_pivot;
        sink.line_to(initial.end);
        add_join(
          sink,
          last_point,
          segment.start,
          pivot,
          last_normal,
          segment.start_normal,
          radius,
          false,
        );
      } else {
        sink.line_to(segment.start);
      }
      first = false;
    } else {
      add_join(
        sink,
        last_point,
        segment.start,
        pivot,
        last_normal,
        segment.start_normal,
        radius,
        segment.id == last_id,
      );
    }
    last_id = segment.id;
    last_normal = segment.end_normal;
    pivot = segment.end_pivot;
    last_point = emit_segment(sink, segment.segment);
  }

  if !closed {
    sink.line_to(first_point);
  }
  sink.close();
}

fn emit_segment(sink: &mut StrokeSink, segment: Segment) -> Point {
  match segment.geometry {
    SegmentGeometry::Line { b, .. } => {
      sink.line_to(b);
      b
    }
    SegmentGeometry::Curve(curve) => {
      sink.curve_to(curve.b, curve.c, curve.d);
      curve.d
    }
  }
}

#[allow(clippy::too_many_arguments)]
fn add_join(
  sink: &mut StrokeSink,
  from: Point,
  to: Point,
  pivot: Point,
  from_normal: Point,
  to_normal: Point,
  radius: f32,
  split_continuation: bool,
) {
  if from.nearly_eq(to) {
    return;
  }
  if !is_clockwise(from_normal, to_normal) {
    sink.line_to(pivot);
    sink.line_to(to);
    return;
  }
  if split_continuation {
    add_positive_small_circular_arc(sink, from, radius.abs(), to);
    return;
  }

  let dot = from_normal.dot(to_normal);
  let sine_half = ((1.0 + dot) * 0.5).sqrt();
  if dot < 0.0 || sine_half < 1.0 / DEFAULT_MITER_LIMIT {
    sink.line_to(to);
  } else {
    let middle = (from_normal + to_normal).normalize() * (radius / sine_half);
    sink.line_to(pivot + middle);
    sink.line_to(to);
  }
}

fn is_clockwise(first: Point, second: Point) -> bool {
  first.x * second.y > first.y * second.x
}

fn add_positive_small_circular_arc(sink: &mut StrokeSink, from: Point, radius: f32, to: Point) {
  let px_prime = (from.x - to.x) * 0.5;
  let py_prime = (from.y - to.y) * 0.5;
  if px_prime == 0.0 && py_prime == 0.0 {
    return;
  }

  let mut radius = radius.abs();
  let lambda = px_prime.powi(2) / radius.powi(2) + py_prime.powi(2) / radius.powi(2);
  if lambda > 1.0 {
    radius *= lambda.sqrt();
  }
  let radius_squared = radius * radius;
  let px_squared = px_prime * px_prime;
  let py_squared = py_prime * py_prime;
  let mut radicand =
    radius_squared * radius_squared - radius_squared * py_squared - radius_squared * px_squared;
  if radicand < 0.0 {
    radicand = 0.0;
  }
  radicand /= radius_squared * py_squared + radius_squared * px_squared;
  let factor = radicand.sqrt();
  let center_prime_x = factor * py_prime;
  let center_prime_y = factor * -px_prime;
  let center = Point::new(
    center_prime_x + (from.x + to.x) * 0.5,
    center_prime_y + (from.y + to.y) * 0.5,
  );
  let first_vector = Point::new(
    (px_prime - center_prime_x) / radius,
    (py_prime - center_prime_y) / radius,
  );
  let second_vector = Point::new(
    (-px_prime - center_prime_x) / radius,
    (-py_prime - center_prime_y) / radius,
  );
  let mut angle = vector_angle(first_vector, second_vector);
  const TAU: f32 = std::f32::consts::PI * 2.0;
  if angle < 0.0 {
    angle += TAU;
  }
  let mut ratio = angle.abs() / (TAU / 4.0);
  if (1.0 - ratio).abs() < 0.000_000_1 {
    ratio = 1.0;
  }
  let segment_count = ratio.ceil().max(1.0) as usize;
  angle /= segment_count as f32;
  let cubic_factor = if angle == std::f32::consts::FRAC_PI_2 {
    0.551_915_05
  } else if angle == -std::f32::consts::FRAC_PI_2 {
    -0.551_915_05
  } else {
    4.0 / 3.0 * (angle / 4.0).tan()
  };
  let mut first_angle = vector_angle(Point::new(1.0, 0.0), first_vector);
  for _ in 0..segment_count {
    let (y1, x1) = first_angle.sin_cos();
    let (y2, x2) = (first_angle + angle).sin_cos();
    let first = Point::new(
      center.x + (x1 - y1 * cubic_factor) * radius,
      center.y + (y1 + x1 * cubic_factor) * radius,
    );
    let second = Point::new(
      center.x + (x2 + y2 * cubic_factor) * radius,
      center.y + (y2 - x2 * cubic_factor) * radius,
    );
    let end = Point::new(center.x + x2 * radius, center.y + y2 * radius);
    sink.curve_to(first, second, end);
    first_angle += angle;
  }
}

fn vector_angle(first: Point, second: Point) -> f32 {
  let sign = if first.x * second.y - first.y * second.x < 0.0 {
    -1.0
  } else {
    1.0
  };
  sign * first.dot(second).clamp(-1.0, 1.0).acos()
}

#[cfg(test)]
mod tests {
  use super::{MERGE_EPSILON, Point, simplified_subpaths, widen_centered_miter_butt};
  use kurbo::BezPath;

  #[test]
  fn curve_splits_keep_one_authored_segment_id() {
    let mut path = BezPath::new();
    path.move_to((0.0, 0.0));
    path.curve_to((0.0, 100.0), (100.0, -100.0), (100.0, 0.0));
    path.close_path();
    let subpaths = simplified_subpaths(&path).expect("valid path");
    let curve_pieces: Vec<_> = subpaths[0]
      .segments
      .iter()
      .take_while(|segment| segment.id == subpaths[0].segments[0].id)
      .collect();
    assert!(curve_pieces.len() > 1);
    assert!(curve_pieces.windows(2).all(|pair| pair[0].id == pair[1].id));
  }

  #[test]
  fn closed_square_is_widened_on_both_sides() {
    let mut path = BezPath::new();
    path.move_to((10.0, 10.0));
    path.line_to((30.0, 10.0));
    path.line_to((30.0, 30.0));
    path.line_to((10.0, 30.0));
    path.close_path();
    let outline = widen_centered_miter_butt(&path, 4.0).expect("outline");
    let bounds = kurbo::Shape::bounding_box(&outline);
    assert!((bounds.x0 - 8.0).abs() < f64::from(MERGE_EPSILON));
    assert!((bounds.y0 - 8.0).abs() < f64::from(MERGE_EPSILON));
    assert!((bounds.x1 - 32.0).abs() < f64::from(MERGE_EPSILON));
    assert!((bounds.y1 - 32.0).abs() < f64::from(MERGE_EPSILON));
  }

  #[test]
  fn invalid_or_zero_width_has_no_outline() {
    let mut path = BezPath::new();
    path.move_to((0.0, 0.0));
    path.line_to((1.0, 0.0));
    assert!(widen_centered_miter_butt(&path, 0.0).is_none());
    assert!(widen_centered_miter_butt(&path, f32::NAN).is_none());
    assert!(Point::new(1.0, 2.0).is_finite());
  }
}
