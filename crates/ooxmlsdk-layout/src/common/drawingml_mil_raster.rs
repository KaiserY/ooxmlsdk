//! MIL-compatible nonzero path coverage for Word's static-3-D text contour.
//!
//! WPF's software renderer does not use continuous analytic area for this
//! path. It flattens curves in 28.4 fixed point, evaluates eight horizontal
//! subscanlines on a 1/8-pixel grid, and accumulates integer coverage in
//! `0..=64`. This safe implementation follows the connected pipeline in the
//! local WPF sources (`bezier.{h,cpp}`, `aarasterizer.{h,cpp}`, and
//! `aacoverage.h`) while retaining an allocation proportional to the output
//! mask rather than an 8x supersampled bitmap.

use kurbo::{BezPath, PathEl};

const FIXED_28_4_SCALE: f32 = 16.0;
const EDGE_FRACTION_BITS: u32 = 4;
const COVERAGE_SHIFT: u32 = 3;
const COVERAGE_SCALE: i64 = 1_i64 << COVERAGE_SHIFT;
const COVERAGE_COMPLETE: u8 = (COVERAGE_SCALE * COVERAGE_SCALE) as u8;
const MAX_FLATTENED_POINTS_PER_CURVE: usize = 1 << 20;

const HFD32_INITIAL_SHIFT: u32 = 10;
const HFD32_ADDITIONAL_SHIFT: u32 = 3;
const HFD32_SHIFT: u32 = HFD32_INITIAL_SHIFT + HFD32_ADDITIONAL_SHIFT;
const HFD32_ROUND: i64 = 1_i64 << (HFD32_SHIFT - 1);
const HFD32_TOLERANCE: i64 = 24;
const HFD32_INITIAL_TEST_MAGNITUDE: i64 = HFD32_TOLERANCE << HFD32_INITIAL_SHIFT;
const HFD32_TEST_MAGNITUDE: i64 = HFD32_INITIAL_TEST_MAGNITUDE << HFD32_ADDITIONAL_SHIFT;
const HFD32_MAX_ERROR: i64 = HFD32_TOLERANCE << ((2 * HFD32_INITIAL_SHIFT) / 3);
const HFD32_MAX_LOCAL_COORDINATE: i64 = 1 << 14;

const HFD64_FRACTION: u32 = 28;
const HFD64_ROUND: i128 = 1_i128 << (HFD64_FRACTION - 1);
const HFD64_ERROR_HIGH: i128 = ((6_i128 * (1_i128 << 15)) >> (32 - HFD64_FRACTION)) << 32;
const HFD64_ERROR_LOW: i128 = 3_i128 << 31;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct FixedPoint {
  x: i32,
  y: i32,
}

impl FixedPoint {
  const fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }
}

#[derive(Clone, Copy, Debug)]
struct FloatPoint {
  x: f32,
  y: f32,
}

impl FloatPoint {
  fn from_kurbo(point: kurbo::Point) -> Option<Self> {
    let x = point.x as f32;
    let y = point.y as f32;
    (x.is_finite() && y.is_finite()).then_some(Self { x, y })
  }

  fn quadratic_control_from_start(self, control: Self) -> Self {
    const TWO_THIRDS: f32 = 2.0 / 3.0;
    Self {
      x: self.x + TWO_THIRDS * (control.x - self.x),
      y: self.y + TWO_THIRDS * (control.y - self.y),
    }
  }
}

fn round_halves_up(value: f32) -> Option<i32> {
  if !value.is_finite() || value < i32::MIN as f32 || value >= i32::MAX as f32 {
    return None;
  }
  Some((value + 0.5).floor() as i32)
}

fn transform_to_28_4(point: FloatPoint, translate_x: f32, translate_y: f32) -> Option<FixedPoint> {
  // RasterizePath converts half-pixel-center device coordinates to integer
  // pixel centers before appending the 16x 28.4 scale. Keep the operations in
  // single precision because their rounding is part of MIL's output contract.
  let offset_x = (translate_x - 0.5) * FIXED_28_4_SCALE;
  let offset_y = (translate_y - 0.5) * FIXED_28_4_SCALE;
  let x = point.x * FIXED_28_4_SCALE + offset_x;
  let y = point.y * FIXED_28_4_SCALE + offset_y;
  Some(FixedPoint::new(round_halves_up(x)?, round_halves_up(y)?))
}

#[derive(Clone, Debug)]
struct HfdBasis32 {
  e0: i64,
  e1: i64,
  e2: i64,
  e3: i64,
}

impl HfdBasis32 {
  fn new(points: [i32; 4]) -> Option<Self> {
    let [p1, p2, p3, p4] = points.map(i64::from);
    let e2 = 6 * (p2 - p3 - p3 + p4);
    let e3 = 6 * (p1 - p2 - p2 + p3);
    (e2.abs().max(e3.abs()) < HFD32_MAX_ERROR).then_some(Self {
      e0: p1 << HFD32_INITIAL_SHIFT,
      e1: (p4 - p1) << HFD32_INITIAL_SHIFT,
      e2: e2 << HFD32_INITIAL_SHIFT,
      e3: e3 << HFD32_INITIAL_SHIFT,
    })
  }

  fn parent_error_divided_by_four(&self) -> i64 {
    self.e3.abs().max((self.e2 + self.e2 - self.e3).abs())
  }

  fn error(&self) -> i64 {
    self.e2.abs().max(self.e3.abs())
  }

  fn value(&self) -> Option<i32> {
    i32::try_from((self.e0 + HFD32_ROUND) >> HFD32_SHIFT).ok()
  }

  fn lazy_halve_step_size(&mut self, shift: u32) {
    self.e2 = (self.e2 + self.e3) >> 1;
    self.e1 = (self.e1 - (self.e2 >> shift)) >> 1;
  }

  fn enter_steady_state(&mut self, shift: u32) {
    self.e0 <<= HFD32_ADDITIONAL_SHIFT;
    self.e1 <<= HFD32_ADDITIONAL_SHIFT;
    if shift < HFD32_ADDITIONAL_SHIFT {
      let left = HFD32_ADDITIONAL_SHIFT - shift;
      self.e2 <<= left;
      self.e3 <<= left;
    } else {
      let right = shift - HFD32_ADDITIONAL_SHIFT;
      self.e2 >>= right;
      self.e3 >>= right;
    }
  }

  fn halve_step_size(&mut self) {
    self.e2 = (self.e2 + self.e3) >> 3;
    self.e1 = (self.e1 - self.e2) >> 1;
    self.e3 >>= 2;
  }

  fn double_step_size(&mut self) {
    self.e1 = self.e1 + (self.e1 + self.e2);
    self.e3 <<= 2;
    self.e2 = (self.e2 << 3) - self.e3;
  }

  fn take_step(&mut self) {
    self.e0 += self.e1;
    let previous_e2 = self.e2;
    self.e1 += previous_e2;
    self.e2 += previous_e2 - self.e3;
    self.e3 = previous_e2;
  }
}

struct MilBezier32 {
  x: HfdBasis32,
  y: HfdBasis32,
  steps: u32,
  offset_x: i32,
  offset_y: i32,
}

impl MilBezier32 {
  fn new(points: [FixedPoint; 4]) -> Option<Self> {
    let offset_x = points.iter().map(|point| point.x).min()?.checked_sub(16)?;
    let offset_y = points.iter().map(|point| point.y).min()?.checked_sub(16)?;
    let mut local_x = [0_i32; 4];
    let mut local_y = [0_i32; 4];
    let mut local_or = 0_i64;
    for (index, point) in points.iter().enumerate() {
      local_x[index] = point.x.checked_sub(offset_x)?;
      local_y[index] = point.y.checked_sub(offset_y)?;
      local_or |= i64::from(local_x[index]) | i64::from(local_y[index]);
    }
    if !(0..HFD32_MAX_LOCAL_COORDINATE).contains(&local_or) {
      return None;
    }

    let mut x = HfdBasis32::new(local_x)?;
    let mut y = HfdBasis32::new(local_y)?;
    let mut steps = 1_u32;
    let mut shift = 0_u32;
    loop {
      let test = HFD32_INITIAL_TEST_MAGNITUDE.checked_shl(shift)?;
      if x.error() <= test && y.error() <= test {
        break;
      }
      shift = shift.checked_add(2)?;
      x.lazy_halve_step_size(shift);
      y.lazy_halve_step_size(shift);
      steps = steps.checked_mul(2)?;
    }
    x.enter_steady_state(shift);
    y.enter_steady_state(shift);
    x.take_step();
    y.take_step();
    steps -= 1;
    Some(Self {
      x,
      y,
      steps,
      offset_x,
      offset_y,
    })
  }

  fn flatten(mut self) -> Option<Vec<FixedPoint>> {
    let mut result = Vec::new();
    loop {
      result.push(FixedPoint::new(
        self.x.value()?.checked_add(self.offset_x)?,
        self.y.value()?.checked_add(self.offset_y)?,
      ));
      if result.len() > MAX_FLATTENED_POINTS_PER_CURVE {
        return None;
      }
      if self.steps == 0 {
        return Some(result);
      }
      if self.x.error().max(self.y.error()) > HFD32_TEST_MAGNITUDE {
        self.x.halve_step_size();
        self.y.halve_step_size();
        self.steps = self.steps.checked_mul(2)?;
      }
      while self.steps & 1 == 0
        && self.x.parent_error_divided_by_four() <= (HFD32_TEST_MAGNITUDE >> 2)
        && self.y.parent_error_divided_by_four() <= (HFD32_TEST_MAGNITUDE >> 2)
      {
        self.x.double_step_size();
        self.y.double_step_size();
        self.steps >>= 1;
      }
      self.steps -= 1;
      self.x.take_step();
      self.y.take_step();
    }
  }
}

#[derive(Clone, Debug)]
struct HfdBasis64 {
  e0: i128,
  e1: i128,
  e2: i128,
  e3: i128,
}

impl HfdBasis64 {
  fn new(points: [i32; 4]) -> Self {
    let [p1, p2, p3, p4] = points.map(i128::from);
    Self {
      e0: p1 << HFD64_FRACTION,
      e1: (p4 - p1) << HFD64_FRACTION,
      e2: (6 * (p2 - p3 - p3 + p4)) << HFD64_FRACTION,
      e3: (6 * (p1 - p2 - p2 + p3)) << HFD64_FRACTION,
    }
  }

  fn parent_error(&self) -> i128 {
    (self.e3 << 2)
      .abs()
      .max(((self.e2 << 3) - (self.e3 << 2)).abs())
  }

  fn error(&self) -> i128 {
    self.e2.abs().max(self.e3.abs())
  }

  fn value(&self) -> Option<i32> {
    i32::try_from((self.e0 + HFD64_ROUND) >> HFD64_FRACTION).ok()
  }

  fn control_points(&self) -> Option<[i32; 4]> {
    let p0 = self.e0;
    let mut p2 = self.e1 * 3;
    let mut p1 = p2 * 2;
    p1 -= self.e2;
    p2 = p1 * 2;
    p2 -= self.e3;
    p1 -= self.e3 * 2;
    p1 = p1 / 18 + self.e0;
    p2 = p2 / 18 + self.e0;
    let p3 = self.e0 + self.e1;
    [p0, p1, p2, p3]
      .map(|value| i32::try_from((value + HFD64_ROUND) >> HFD64_FRACTION).ok())
      .into_iter()
      .collect::<Option<Vec<_>>>()?
      .try_into()
      .ok()
  }

  fn halve_step_size(&mut self) {
    self.e2 = (self.e2 + self.e3) >> 3;
    self.e1 = (self.e1 - self.e2) >> 1;
    self.e3 >>= 2;
  }

  fn double_step_size(&mut self) {
    self.e1 = (self.e1 << 1) + self.e2;
    self.e3 <<= 2;
    self.e2 = (self.e2 << 3) - self.e3;
  }

  fn take_step(&mut self) {
    self.e0 += self.e1;
    let previous_e2 = self.e2;
    self.e1 += previous_e2;
    self.e2 += previous_e2 - self.e3;
    self.e3 = previous_e2;
  }
}

struct MilBezier64 {
  x_high: HfdBasis64,
  y_high: HfdBasis64,
  x_low: Option<HfdBasis64>,
  y_low: Option<HfdBasis64>,
  high_steps: u32,
  low_steps: u32,
}

impl MilBezier64 {
  fn new(points: [FixedPoint; 4]) -> Option<Self> {
    let mut x_high = HfdBasis64::new(points.map(|point| point.x));
    let mut y_high = HfdBasis64::new(points.map(|point| point.y));
    let mut high_steps = 1_u32;
    while x_high.error() > HFD64_ERROR_HIGH || y_high.error() > HFD64_ERROR_HIGH {
      high_steps = high_steps.checked_mul(2)?;
      x_high.halve_step_size();
      y_high.halve_step_size();
    }
    Some(Self {
      x_high,
      y_high,
      x_low: None,
      y_low: None,
      high_steps,
      low_steps: 0,
    })
  }

  fn flatten(mut self) -> Option<Vec<FixedPoint>> {
    let mut result = Vec::new();
    loop {
      if self.low_steps == 0 {
        let mut x_low = HfdBasis64::new(self.x_high.control_points()?);
        let mut y_low = HfdBasis64::new(self.y_high.control_points()?);
        self.low_steps = 1;
        while x_low.error() > HFD64_ERROR_LOW || y_low.error() > HFD64_ERROR_LOW {
          self.low_steps = self.low_steps.checked_mul(2)?;
          x_low.halve_step_size();
          y_low.halve_step_size();
        }
        self.high_steps = self.high_steps.checked_sub(1)?;
        if self.high_steps != 0 {
          self.x_high.take_step();
          self.y_high.take_step();
          if self.x_high.error() > HFD64_ERROR_HIGH || self.y_high.error() > HFD64_ERROR_HIGH {
            self.high_steps = self.high_steps.checked_mul(2)?;
            self.x_high.halve_step_size();
            self.y_high.halve_step_size();
          }
          while self.high_steps & 1 == 0
            && self.x_high.parent_error() <= HFD64_ERROR_HIGH
            && self.y_high.parent_error() <= HFD64_ERROR_HIGH
          {
            self.x_high.double_step_size();
            self.y_high.double_step_size();
            self.high_steps >>= 1;
          }
        }
        self.x_low = Some(x_low);
        self.y_low = Some(y_low);
      }

      let x_low = self.x_low.as_mut()?;
      let y_low = self.y_low.as_mut()?;
      x_low.take_step();
      y_low.take_step();
      result.push(FixedPoint::new(x_low.value()?, y_low.value()?));
      if result.len() > MAX_FLATTENED_POINTS_PER_CURVE {
        return None;
      }
      self.low_steps = self.low_steps.checked_sub(1)?;
      if self.low_steps == 0 && self.high_steps == 0 {
        return Some(result);
      }
      if x_low.error() > HFD64_ERROR_LOW || y_low.error() > HFD64_ERROR_LOW {
        self.low_steps = self.low_steps.checked_mul(2)?;
        x_low.halve_step_size();
        y_low.halve_step_size();
      }
      while self.low_steps & 1 == 0
        && x_low.parent_error() <= HFD64_ERROR_LOW
        && y_low.parent_error() <= HFD64_ERROR_LOW
      {
        x_low.double_step_size();
        y_low.double_step_size();
        self.low_steps >>= 1;
      }
    }
  }
}

fn flatten_cubic(points: [FixedPoint; 4]) -> Option<Vec<FixedPoint>> {
  if let Some(bezier) = MilBezier32::new(points) {
    bezier.flatten()
  } else {
    MilBezier64::new(points)?.flatten()
  }
}

/// Flatten a device-space path on the shared Windows 28.4 curve lattice.
/// This exposes geometry only; the caller still owns sample positions, fill
/// rules, coverage resolution and compositing.
pub(super) fn flatten_device_path_28_4(path: &BezPath) -> Option<BezPath> {
  let fixed = |p| transform_to_28_4(FloatPoint::from_kurbo(p)?, 0.5, 0.5);
  let point = |p: FixedPoint| kurbo::Point::new(f64::from(p.x) / 16.0, f64::from(p.y) / 16.0);
  let mut result = BezPath::new();
  let mut current = None;
  let mut first = None;
  for element in path.elements() {
    match *element {
      PathEl::MoveTo(p) => {
        let p = fixed(p)?;
        result.move_to(point(p));
        current = Some(p);
        first = Some(p);
      }
      PathEl::LineTo(p) => {
        let p = fixed(p)?;
        result.line_to(point(p));
        current = Some(p);
      }
      PathEl::QuadTo(control, end) => {
        let start = FloatPoint::from_kurbo(point(current?))?;
        let control = FloatPoint::from_kurbo(control)?;
        let end_float = FloatPoint::from_kurbo(end)?;
        let a = start.quadratic_control_from_start(control);
        let b = end_float.quadratic_control_from_start(control);
        let end = fixed(end)?;
        for p in flatten_cubic([
          current?,
          transform_to_28_4(a, 0.5, 0.5)?,
          transform_to_28_4(b, 0.5, 0.5)?,
          end,
        ])? {
          result.line_to(point(p));
        }
        current = Some(end);
      }
      PathEl::CurveTo(a, b, end) => {
        let end = fixed(end)?;
        for p in flatten_cubic([current?, fixed(a)?, fixed(b)?, end])? {
          result.line_to(point(p));
        }
        current = Some(end);
      }
      PathEl::ClosePath => {
        result.close_path();
        current = first;
      }
    }
  }
  Some(result)
}

#[derive(Clone, Debug)]
struct Edge {
  x: i64,
  dx: i64,
  error: i64,
  error_up: i64,
  error_down: i64,
  start_y: i64,
  end_y: i64,
  winding: i32,
}

impl Edge {
  fn advance(&mut self) {
    self.x += self.dx;
    self.error += self.error_up;
    if self.error >= 0 {
      self.error -= self.error_down;
      self.x += 1;
    }
  }
}

fn initialize_edge(first: FixedPoint, second: FixedPoint) -> Option<Edge> {
  let x0 = (i64::from(first.x) + 8) << COVERAGE_SHIFT;
  let y0 = (i64::from(first.y) + 8) << COVERAGE_SHIFT;
  let x1 = (i64::from(second.x) + 8) << COVERAGE_SHIFT;
  let y1 = (i64::from(second.y) + 8) << COVERAGE_SHIFT;
  let mut delta_x = x1 - x0;
  let mut delta_y = y1 - y0;
  let (mut x_start, y_start, start_y, end_y, winding) = if delta_y >= 0 {
    (
      x0,
      y0,
      (y0 + 15) >> EDGE_FRACTION_BITS,
      (y1 + 15) >> EDGE_FRACTION_BITS,
      1,
    )
  } else {
    delta_y = -delta_y;
    delta_x = -delta_x;
    (
      x1,
      y1,
      (y1 + 15) >> EDGE_FRACTION_BITS,
      (y0 + 15) >> EDGE_FRACTION_BITS,
      -1,
    )
  };
  if end_y <= start_y {
    return None;
  }

  let (dx, error_up) = if delta_x < 0 {
    let magnitude = -delta_x;
    if magnitude < delta_y {
      (-1, delta_y - magnitude)
    } else {
      let quotient = magnitude / delta_y;
      let remainder = magnitude % delta_y;
      if remainder > 0 {
        (-quotient - 1, delta_y - remainder)
      } else {
        (-quotient, 0)
      }
    }
  } else if delta_x < delta_y {
    (0, delta_x)
  } else {
    (delta_x / delta_y, delta_x % delta_y)
  };

  let mut error = -1_i64;
  if y_start & 15 != 0 {
    for _ in 0..16 - (y_start & 15) {
      x_start += dx;
      error += error_up;
      if error >= 0 {
        error -= delta_y;
        x_start += 1;
      }
    }
  }
  if x_start & 15 != 0 {
    error -= delta_y * (16 - (x_start & 15));
    x_start += 15;
  }
  Some(Edge {
    x: x_start >> EDGE_FRACTION_BITS,
    dx,
    error: error >> EDGE_FRACTION_BITS,
    error_up,
    error_down: delta_y,
    start_y,
    end_y,
    winding,
  })
}

struct EdgeBuilder {
  edges: Vec<Edge>,
  first: Option<FixedPoint>,
  current: Option<FixedPoint>,
  current_float: Option<FloatPoint>,
  translate_x: f32,
  translate_y: f32,
}

impl EdgeBuilder {
  fn new(translate_x: f32, translate_y: f32) -> Self {
    Self {
      edges: Vec::new(),
      first: None,
      current: None,
      current_float: None,
      translate_x,
      translate_y,
    }
  }

  fn transformed(&self, point: FloatPoint) -> Option<FixedPoint> {
    transform_to_28_4(point, self.translate_x, self.translate_y)
  }

  fn push_edge(&mut self, point: FixedPoint) {
    if let Some(current) = self.current
      && let Some(edge) = initialize_edge(current, point)
    {
      self.edges.push(edge);
    }
    self.current = Some(point);
  }

  fn close(&mut self) {
    if let Some(first) = self.first {
      self.push_edge(first);
    }
    self.first = None;
    self.current = None;
    self.current_float = None;
  }

  fn move_to(&mut self, point: FloatPoint) -> Option<()> {
    self.close();
    let fixed = self.transformed(point)?;
    self.first = Some(fixed);
    self.current = Some(fixed);
    self.current_float = Some(point);
    Some(())
  }

  fn line_to(&mut self, point: FloatPoint) -> Option<()> {
    self.current?;
    self.push_edge(self.transformed(point)?);
    self.current_float = Some(point);
    Some(())
  }

  fn cubic_to(
    &mut self,
    control1: FloatPoint,
    control2: FloatPoint,
    end: FloatPoint,
  ) -> Option<()> {
    let start = self.current?;
    for point in flatten_cubic([
      start,
      self.transformed(control1)?,
      self.transformed(control2)?,
      self.transformed(end)?,
    ])? {
      self.push_edge(point);
    }
    self.current_float = Some(end);
    Some(())
  }

  fn quad_to(&mut self, control: FloatPoint, end: FloatPoint) -> Option<()> {
    let start = self.current_float?;
    let control1 = start.quadratic_control_from_start(control);
    let control2 = end.quadratic_control_from_start(control);
    self.cubic_to(control1, control2, end)
  }

  fn finish(mut self) -> Vec<Edge> {
    self.close();
    self.edges
  }
}

fn path_edges(path: &BezPath, translate_x: f32, translate_y: f32) -> Option<Vec<Edge>> {
  let mut builder = EdgeBuilder::new(translate_x, translate_y);
  for element in path.iter() {
    match element {
      PathEl::MoveTo(point) => builder.move_to(FloatPoint::from_kurbo(point)?)?,
      PathEl::LineTo(point) => builder.line_to(FloatPoint::from_kurbo(point)?)?,
      PathEl::QuadTo(control, end) => builder.quad_to(
        FloatPoint::from_kurbo(control)?,
        FloatPoint::from_kurbo(end)?,
      )?,
      PathEl::CurveTo(control1, control2, end) => builder.cubic_to(
        FloatPoint::from_kurbo(control1)?,
        FloatPoint::from_kurbo(control2)?,
        FloatPoint::from_kurbo(end)?,
      )?,
      PathEl::ClosePath => builder.close(),
    }
  }
  Some(builder.finish())
}

fn add_interval(coverage: &mut [u8], width: usize, pixel_y: usize, left: i64, right: i64) {
  let subpixel_width = width as i64 * COVERAGE_SCALE;
  let left = left.clamp(0, subpixel_width);
  let right = right.clamp(0, subpixel_width);
  if left >= right {
    return;
  }
  let first_pixel = left >> COVERAGE_SHIFT;
  let last_pixel = (right - 1) >> COVERAGE_SHIFT;
  for pixel_x in first_pixel..=last_pixel {
    let overlap = right.min((pixel_x + 1) << COVERAGE_SHIFT) - left.max(pixel_x << COVERAGE_SHIFT);
    let index = pixel_y * width + pixel_x as usize;
    coverage[index] = coverage[index]
      .checked_add(overlap as u8)
      .expect("nonzero winding intervals do not overlap");
    debug_assert!(coverage[index] <= COVERAGE_COMPLETE);
  }
}

fn coverage_alpha(coverage: u8) -> u8 {
  if coverage == COVERAGE_COMPLETE {
    return 255;
  }
  ((u16::from(coverage) * 255 * 4 + 128) >> 8) as u8
}

/// Rasterize `path` with WPF/MIL's 8x8 nonzero-winding coverage contract.
pub(super) fn rasterize_nonzero_wpf_8x8_path(
  path: &BezPath,
  width: u32,
  height: u32,
  translate_x: f64,
  translate_y: f64,
) -> Option<Vec<u8>> {
  let width = usize::try_from(width).ok()?;
  let height = usize::try_from(height).ok()?;
  let pixel_count = width.checked_mul(height)?;
  let translate_x = translate_x as f32;
  let translate_y = translate_y as f32;
  if width == 0 || height == 0 || !translate_x.is_finite() || !translate_y.is_finite() {
    return None;
  }

  let mut edges = path_edges(path, translate_x, translate_y)?;
  let subpixel_height = i64::try_from(height).ok()?.checked_mul(COVERAGE_SCALE)?;
  for edge in &mut edges {
    while edge.start_y < 0 && edge.start_y < edge.end_y {
      edge.advance();
      edge.start_y += 1;
    }
    edge.end_y = edge.end_y.min(subpixel_height);
  }
  edges.retain(|edge| edge.start_y < edge.end_y && edge.start_y < subpixel_height);
  if edges.is_empty() {
    return Some(vec![0; pixel_count]);
  }

  let mut inactive = (0..edges.len()).collect::<Vec<_>>();
  inactive.sort_by_key(|&index| (edges[index].start_y, edges[index].x, index));
  let scan_start = edges.iter().map(|edge| edge.start_y).min()?.max(0);
  let scan_end = edges
    .iter()
    .map(|edge| edge.end_y)
    .max()?
    .min(subpixel_height);
  let mut next_inactive = 0_usize;
  let mut active = Vec::<usize>::new();
  let mut coverage = vec![0_u8; pixel_count];

  for subpixel_y in scan_start..scan_end {
    active.retain(|&index| edges[index].end_y > subpixel_y);
    while next_inactive < inactive.len() && edges[inactive[next_inactive]].start_y == subpixel_y {
      active.push(inactive[next_inactive]);
      next_inactive += 1;
    }
    active.sort_unstable_by_key(|&index| (edges[index].x, index));

    let pixel_y = (subpixel_y >> COVERAGE_SHIFT) as usize;
    let mut active_index = 0_usize;
    let mut winding = 0_i32;
    let mut interval_start = None;
    while active_index < active.len() {
      let x = edges[active[active_index]].x;
      let previous_winding = winding;
      while active_index < active.len() && edges[active[active_index]].x == x {
        winding += edges[active[active_index]].winding;
        active_index += 1;
      }
      if previous_winding == 0 && winding != 0 {
        interval_start = Some(x);
      } else if previous_winding != 0 && winding == 0 {
        add_interval(&mut coverage, width, pixel_y, interval_start.take()?, x);
      }
    }
    if winding != 0 || interval_start.is_some() {
      return None;
    }
    for &index in &active {
      edges[index].advance();
    }
  }

  Some(coverage.into_iter().map(coverage_alpha).collect())
}

#[cfg(test)]
mod tests {
  use kurbo::{BezPath, Point};

  use super::{
    FixedPoint, MilBezier32, MilBezier64, coverage_alpha, rasterize_nonzero_wpf_8x8_path,
  };

  #[test]
  fn mil_hfd32_matches_the_fixed_point_curve_control() {
    let points = [
      FixedPoint::new(917, 538),
      FixedPoint::new(917, 610),
      FixedPoint::new(904, 679),
      FixedPoint::new(880, 737),
    ];
    assert_eq!(
      MilBezier32::new(points).unwrap().flatten().unwrap(),
      [FixedPoint::new(908, 643), FixedPoint::new(880, 737)]
    );
  }

  #[test]
  fn mil_hfd64_handles_curves_outside_the_hfd32_window() {
    let points = [
      FixedPoint::new(0, 0),
      FixedPoint::new(20_000, 30_000),
      FixedPoint::new(40_000, -30_000),
      FixedPoint::new(60_000, 0),
    ];
    assert!(MilBezier32::new(points).is_none());
    let flattened = MilBezier64::new(points).unwrap().flatten().unwrap();
    assert_eq!(flattened.last(), Some(&points[3]));
    assert!(flattened.len() > 1);
  }

  #[test]
  fn coverage_scaling_preserves_mils_64_integer_levels() {
    assert_eq!(coverage_alpha(0), 0);
    assert_eq!(coverage_alpha(1), 4);
    assert_eq!(coverage_alpha(32), 128);
    assert_eq!(coverage_alpha(33), 131);
    assert_eq!(coverage_alpha(63), 251);
    assert_eq!(coverage_alpha(64), 255);
  }

  #[test]
  fn rasterizer_accumulates_eight_subscanlines_without_a_supersampled_bitmap() {
    let mut path = BezPath::new();
    path.move_to(Point::new(0.125, 0.0));
    path.line_to(Point::new(1.0, 0.0));
    path.line_to(Point::new(1.0, 1.0));
    path.line_to(Point::new(0.125, 1.0));
    path.close_path();
    assert_eq!(
      rasterize_nonzero_wpf_8x8_path(&path, 2, 1, 0.0, 0.0).unwrap(),
      [223, 0]
    );
  }

  #[test]
  fn rasterizer_uses_nonzero_winding_for_opposite_inner_contours() {
    let mut path = BezPath::new();
    path.move_to(Point::new(0.0, 0.0));
    path.line_to(Point::new(3.0, 0.0));
    path.line_to(Point::new(3.0, 1.0));
    path.line_to(Point::new(0.0, 1.0));
    path.close_path();
    path.move_to(Point::new(1.0, 0.0));
    path.line_to(Point::new(1.0, 1.0));
    path.line_to(Point::new(2.0, 1.0));
    path.line_to(Point::new(2.0, 0.0));
    path.close_path();
    assert_eq!(
      rasterize_nonzero_wpf_8x8_path(&path, 3, 1, 0.0, 0.0).unwrap(),
      [255, 0, 255]
    );
  }
}
