//! Legacy Office textpath distance lookup in its authored integer coordinates.
//!
//! Unlike GFX's DrawingML path, MSO integrates the unscaled derivatives for
//! the total length and scaled derivatives for each curve's lookup. Its
//! downward-rounded arithmetic can leave 49 samples instead of 50. Preserve
//! that endpoint decision without changing the caller's floating-point mode.

use std::sync::Arc;

use kurbo::{Affine, Point, Vec2};

mod arcs;
mod downward;
pub use arcs::vml_arc_cubics;
use downward::Down;

const CAPACITY: usize = 50;

/// Authored controls survive until the host supplies its measurement or print
/// frame. Building the distance table from a resized preset loses integer
/// controls and can select a different terminal sample.
#[derive(Clone, Debug, PartialEq)]
pub struct VmlPathSource {
  pub points: Vec<[i32; 2]>,
  pub origin: [i32; 2],
  pub size: [i32; 2],
}

#[derive(Clone, Debug, PartialEq)]
pub struct VmlPathProjection {
  sampler: Arc<VmlPathSampler>,
  scale: f64,
  origin: Point,
}

impl VmlPathSource {
  pub fn project(&self, origin: Point, size_twips: [i32; 2]) -> Option<VmlPathProjection> {
    if self.size.iter().chain(&size_twips).any(|v| *v <= 0) {
      return None;
    }
    let [width, height] = size_twips.map(|v| Down(f64::from(v)));
    let [cx, cy] = self.size.map(|v| Down(f64::from(v)));
    let sx = Down(65536.0) / cx;
    let sy = ((height / width) * Down(65536.0)) / cy;
    let sampler = VmlPathSampler::from_cubics(&self.points, [sx.0, sy.0])?;
    let scale = (width / Down(20.0)) / Down(65536.0);
    Some(VmlPathProjection {
      sampler: Arc::new(sampler),
      scale: scale.0,
      origin: Point::new(
        origin.x - (Down(f64::from(self.origin[0])) * sx * scale).0,
        origin.y - (Down(f64::from(self.origin[1])) * sy * scale).0,
      ),
    })
  }
}

impl VmlPathProjection {
  pub fn translate(&mut self, dx: f32, dy: f32) {
    self.origin += Vec2::new(f64::from(dx), f64::from(dy));
  }

  /// Map page-space glyph coordinates once per glyph. Its inclusive ink
  /// center determines the tangent; the font's baseline anchor supplies the
  /// normal offset. Bounds and PDF painting use the same immutable lookup.
  pub fn glyph_transform(
    &self,
    start_x: f64,
    advance_width: f64,
    ink_center_x: f64,
    centerline_y: f64,
  ) -> Option<Affine> {
    if advance_width <= f64::EPSILON {
      return None;
    }
    let position = (Down(ink_center_x) - Down(start_x)) / Down(advance_width);
    let (center, tangent) = self.sampler.sample(position.0)?;
    let scale = (self.sampler.length * Down(self.scale)) / Down(advance_width);
    let x = (Down(tangent.x) * scale).0;
    let y = (Down(tangent.y) * scale).0;
    Some(Affine::new([
      x,
      y,
      -y,
      x,
      self.origin.x + center.x * self.scale - x * ink_center_x + y * centerline_y,
      self.origin.y + center.y * self.scale - y * ink_center_x - x * centerline_y,
    ]))
  }
}

#[derive(Clone, Debug, PartialEq)]
struct Curve {
  coefficients: [[Down; 4]; 2],
  derivative: [[Down; 3]; 2],
}

fn speed(derivative: &[[Down; 3]; 2], t: Down, scale: [Down; 2]) -> Down {
  let [x, y] = std::array::from_fn(|axis| {
    let [a, b, c] = derivative[axis];
    ((t * c + b) * t + a) * scale[axis]
  });
  (x * x + y * y).sqrt()
}

fn length(derivative: &[[Down; 3]; 2], scale: [Down; 2]) -> Down {
  let mut sum = Down(0.0);
  for index in 0..10 {
    sum = sum
      + speed(
        derivative,
        Down(index as f64) * Down(0.1) + Down(0.05),
        scale,
      );
  }
  sum * Down(0.1)
}

impl Curve {
  fn speed(&self, t: Down) -> Down {
    speed(&self.derivative, t, [Down(1.0); 2])
  }
}

/// A VML single-path lookup. Shared by ink bounds and final outlined glyphs.
#[derive(Clone, Debug, PartialEq)]
pub struct VmlPathSampler {
  curves: Vec<Curve>,
  lookup: Vec<(usize, Down)>,
  length: Down,
}

impl VmlPathSampler {
  /// Adjacent cubics share their endpoint: 1 + 3n integer points. Scaling is
  /// applied after integer source differences, as in the legacy path owner.
  pub fn from_cubics(points: &[[i32; 2]], scale: [f64; 2]) -> Option<Self> {
    if points.len() < 4
      || !(points.len() - 1).is_multiple_of(3)
      || scale.iter().any(|v| !v.is_finite() || *v <= 0.0)
    {
      return None;
    }
    let scale = scale.map(Down);
    let mut curves = Vec::with_capacity((points.len() - 1) / 3);
    let mut total = Down(0.0);
    for start in (0..points.len() - 1).step_by(3) {
      let mut raw = [[Down(0.0); 3]; 2];
      let mut derivative = raw;
      let coefficients = std::array::from_fn(|axis| {
        let [p0, p1, p2, p3] = std::array::from_fn(|i| Down(f64::from(points[start + i][axis])));
        let a = p1 - p0;
        let b = p2 - p1;
        let c = p3 - p2;
        raw[axis] = [
          a * Down(3.0),
          (b - a) * Down(6.0),
          (c - b * Down(2.0) + a) * Down(3.0),
        ];
        let [a, b, c] = [a, b, c].map(|value| value * scale[axis]);
        derivative[axis] = [
          a * Down(3.0),
          (b - a) * Down(6.0),
          (c - b * Down(2.0) + a) * Down(3.0),
        ];
        [
          p0 * scale[axis],
          a * Down(3.0),
          (b - a) * Down(3.0),
          c - b * Down(2.0) + a,
        ]
      });
      total = total + length(&raw, scale);
      curves.push(Curve {
        coefficients,
        derivative,
      });
    }
    if !total.0.is_finite() || total.0 <= f64::EPSILON {
      return None;
    }
    let step = total * Down(1.0 / 49.0);
    let mut before = Down(0.0);
    let mut target = Down(0.0);
    let mut lookup = Vec::with_capacity(CAPACITY);
    'curves: for (index, curve) in curves.iter().enumerate() {
      let length = length(&curve.derivative, [Down(1.0); 2]);
      if !length.0.is_finite() || length.0 <= f64::EPSILON {
        return None;
      }
      let remaining = target - before;
      if remaining > length {
        before = before + length;
        continue;
      }
      let inverse = Down(1.0) / length;
      let speed = curve.speed(remaining * Down(0.5) * inverse);
      let mut t = if remaining.0 == 0.0 {
        Down(0.0)
      } else if speed.0 == 0.0 {
        Down(1.0)
      } else {
        remaining / speed
      };
      let half_step = step * Down(0.5) * inverse;
      loop {
        t = t.min(Down(1.0));
        lookup.push((index, t));
        if lookup.len() == CAPACITY {
          break 'curves;
        }
        target = Down(lookup.len() as f64) * step;
        let mut previous = length;
        let mut speed = curve.speed((t + half_step).min(Down(1.0)));
        for _ in 1..32 {
          if speed.abs() >= (previous - speed).abs() * Down(64.0) {
            break;
          }
          let middle = if speed.0 == 0.0 {
            Down(1.0)
          } else {
            (t + step / (previous + speed)).min(Down(1.0))
          };
          previous = speed;
          speed = curve.speed(middle);
        }
        t = if speed.0 == 0.0 {
          Down(1.0)
        } else {
          t + step / speed
        };
        if target - before > length {
          break;
        }
      }
      before = before + length;
    }
    (lookup.len() >= 2).then_some(Self {
      curves,
      lookup,
      length: total,
    })
  }

  pub fn length(&self) -> f64 {
    self.length.0
  }

  pub fn sample(&self, position: f64) -> Option<(Point, Vec2)> {
    if !position.is_finite() {
      return None;
    }
    let u = Down(position.clamp(0.0, 1.0)) * Down((self.lookup.len() - 1) as f64);
    let index = (u.0 as usize).min(self.lookup.len() - 2);
    let fraction = u - Down(index as f64);
    let (first, a) = self.lookup[index];
    let (last, b) = self.lookup[index + 1];
    let mut t =
      (Down(1.0) - fraction) * a + fraction * if first == last { b } else { b + Down(1.0) };
    let segment = if first != last && t.0 > 1.0 {
      t = t - Down(1.0);
      last
    } else {
      first
    };
    let curve = &self.curves[segment];
    let point: [f64; 2] = curve
      .coefficients
      .map(|[a, b, c, d]| (((d * t + c) * t + b) * t + a).0);
    // The consumer differentiates the stored position polynomial. Its
    // arithmetic differs from the source derivative used to build the table.
    let [dx, dy] = curve
      .coefficients
      .map(|[_, b, c, d]| (d * Down(3.0) * t + c * Down(2.0)) * t + b);
    let norm = (dy * dy + dx * dx).sqrt();
    if norm.0 <= f64::EPSILON || !norm.0.is_finite() {
      return None;
    }
    let inverse = Down(1.0) / norm;
    Some((
      Point::new(point[0], point[1]),
      Vec2::new((dx * inverse).0, (dy * inverse).0),
    ))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn native_vml_glyph_affine_uses_inclusive_ink_center_and_font_anchor() {
    let source = VmlPathSource {
      points: vml_arc_cubics([10800, 10800, 10800, 10800, -11796480, -11796480]).unwrap(),
      origin: [0, 0],
      size: [21600, 21600],
    };
    let path = source.project(Point::ZERO, [5728, 2695]).unwrap();
    let em_scale = 36.0 / 2048.0;
    let transform = path
      .glyph_transform(0.0, 215.05078125, 1025.0 * em_scale, -390.0 * em_scale)
      .unwrap();
    // Actual final-frame Arial Black W transform, in the native 65536-wide
    // normalized space. This separately tests fitting, ink-center positioning,
    // analytic tangent and baseline displacement, without rendering pixels.
    let native = Affine::new([
      4.183729903424167,
      -4.826770112662623,
      4.826770112662623,
      4.183729903424167,
      7.950886141166393,
      16177.51807991566,
    ]);
    for point in [Point::ZERO, Point::new(1000.0, -1466.0)] {
      let actual = transform * Point::new(point.x * em_scale, point.y * em_scale);
      let expected = (native * point).to_vec2() * (286.4 / 65536.0);
      assert!(
        (actual.to_vec2() - expected).hypot() < 1e-9,
        "{actual:?} vs {expected:?}"
      );
    }
  }

  #[test]
  fn native_vml_sample_endpoint_depends_on_scaled_integer_derivatives() {
    // Observation-only Office controls, authored dimensions 286.45x134.8,
    // final inline 286.4x134.75, and 180x84.7. Native total and table entries
    // distinguish rounding and operation order; no visual tolerance involved.
    let points = [
      [0, 10800],
      [0, 4835],
      [4835, 0],
      [10800, 0],
      [16764, 0],
      [21599, 4835],
      [21599, 10799],
    ];
    for (sy, total, count, middle, last) in [
      (
        1.4277995642701524,
        78149.50274066177,
        50,
        0.5943551625960711,
        0.9991774428697245,
      ),
      (
        1.4275191392509827,
        78145.77569341491,
        49,
        0.594374975328796,
        0.9363053112533424,
      ),
      (
        1.4277004115226337,
        78148.18489712024,
        50,
        0.5943621680272039,
        0.9991771778261787,
      ),
    ] {
      let sampler = VmlPathSampler::from_cubics(&points, [3.034074074074074, sy]).unwrap();
      assert_eq!(sampler.length(), total);
      assert_eq!(sampler.lookup.len(), count);
      assert_eq!(sampler.lookup[12], (0, Down(middle)));
      assert_eq!(sampler.lookup[count - 1], (1, Down(last)));
      assert!(sampler.sample(0.5).is_some());
    }
  }
}
