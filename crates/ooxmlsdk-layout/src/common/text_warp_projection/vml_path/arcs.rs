//! Integer controls for VML's center/radius/start/sweep arc operators.
//!
//! The legacy textpath owner rounds downward before converting controls to
//! integers. In particular, a terminal partial quarter is not interchangeable
//! with a stored full quarter. Keep that distinction before scaling the path.

use super::downward::Down;

const HALF_PI: Down = Down(std::f64::consts::FRAC_PI_2);

// The compensated kernels below follow fdlibm k_sin/k_cos:
// Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
// Developed at SunPro, a Sun Microsystems, Inc. business.
// Permission to use, copy, modify, and distribute this software is freely
// granted, provided that this notice is preserved.
//
// Range reduction uses the legacy downward-rounded quadrant selection. This
// differs from a nearest-quadrant libm call, including near full-turn endpoints.
fn sin_cos(angle: Down) -> (Down, Down) {
  let a = angle.abs();
  let quadrant = (a * Down(std::f64::consts::FRAC_2_PI)).0.floor() as i32;
  let q = Down(f64::from(quadrant));
  let mut r = a - q * Down(1.5707963267341256);
  let mut w = q * Down(6.077100506506192e-11);
  let mut x = r - w;
  let exponent = |v: Down| ((v.0.to_bits() >> 52) & 0x7ff) as i32;
  if x.0 == 0.0 || exponent(a) - exponent(x.abs()) > 15 {
    let previous = r;
    w = q * Down(6.077100506303966e-11);
    r = previous - w;
    w = q * Down(2.0222662487959506e-21) - ((previous - r) - w);
    x = r - w;
  }
  let y = (r - x) - w;
  let z = x * x;
  let w = z * z;
  let c = [
    0.041666666666666664,
    -0.0013888888888887398,
    2.4801587298767044e-5,
    -2.755731727234489e-7,
    2.0876146382372144e-9,
    -1.138263981623609e-11,
  ]
  .map(Down);
  let cc = z * (c[0] + z * (c[1] + z * c[2])) + w * w * (c[3] + z * (c[4] + z * c[5]));
  let hz = Down(0.5) * z;
  let v = Down(1.0) - hz;
  let cosine = v + (((Down(1.0) - v) - hz) + (z * cc - x * y));
  let s = [
    -0.16666666666666666,
    0.00833333333333095,
    -0.00019841269836761127,
    2.7557316103728802e-6,
    -2.5051132068021698e-8,
    1.5918144304485914e-10,
  ]
  .map(Down);
  let rr = s[1] + z * (s[2] + z * s[3]) + z * w * (s[4] + z * s[5]);
  let v = z * x;
  let sine = x - ((z * (Down(0.5) * y - v * rr) - y) - v * s[0]);
  let (sine, cosine) = match quadrant.rem_euclid(4) {
    0 => (sine, cosine),
    1 => (cosine, -sine),
    2 => (-sine, -cosine),
    _ => (-cosine, sine),
  };
  (if angle.0 < 0.0 { -sine } else { sine }, cosine)
}

fn partial(start: Down, end: Down) -> [[Down; 2]; 4] {
  let (sa, ca) = sin_cos(start);
  let (sb, cb) = sin_cos(end);
  let half = (end - start) * Down(0.5);
  let (_, cosine) = sin_cos(half);
  let k = Down(half.0.tan()) * ((cosine * Down(4.0)) / ((cosine + Down(1.0)) * Down(3.0)));
  [
    [ca, sa],
    [ca - sa * k, ca * k + sa],
    [sb * k + cb, sb - cb * k],
    [cb, sb],
  ]
}

/// VML angles are signed 16.16 degrees, positive counterclockwise. Controls
/// remain in the authored integer coordinate space, before host-size scaling.
pub fn vml_arc_cubics([cx, cy, rx, ry, start, sweep]: [i32; 6]) -> Option<Vec<[i32; 2]>> {
  if rx <= 0 || ry <= 0 {
    return None;
  }
  let radians = |angle: i32| -Down(f64::from(angle)) * Down(2.663161090079238e-7);
  let start = radians(start);
  let sweep = radians(sweep);
  let end = start + sweep;
  let mut quadrant = (start / HALF_PI).0.floor() as i32;
  let mut boundary = Down(f64::from(quadrant)) * HALF_PI;
  if start > boundary {
    if sweep.0 >= 0.0 {
      boundary = boundary + HALF_PI;
    }
  } else if boundary > start {
    quadrant -= 1;
    if sweep.0 < 0.0 {
      boundary = boundary - HALF_PI;
    }
  } else if sweep.0 >= 0.0 {
    quadrant -= 1;
  }
  let mut at = if sweep.0 > 0.0 {
    end.min(boundary)
  } else if sweep.0 < 0.0 {
    Down(end.0.max(boundary.0))
  } else {
    start
  };
  let integer = |point: [Down; 2]| -> Option<[i32; 2]> {
    let point: [f64; 2] = std::array::from_fn(|axis| {
      (point[axis] * Down(f64::from([rx, ry][axis])) + Down(f64::from([cx, cy][axis])))
        .0
        .floor()
    });
    point
      .iter()
      .all(|v| v.is_finite() && *v >= f64::from(i32::MIN) && *v <= f64::from(i32::MAX))
      .then(|| point.map(|v| v as i32))
  };
  let first = partial(start, at);
  let mut points = vec![integer(first[0])?];
  if (start - at).abs().0 > 1e-8 {
    for point in &first[1..] {
      points.push(integer(*point)?);
    }
  }
  let k = 0.5522847498307935;
  let positive = [
    [[1.0, k], [k, 1.0], [0.0, 1.0]],
    [[-k, 1.0], [-1.0, k], [-1.0, 0.0]],
    [[-1.0, -k], [-k, -1.0], [0.0, -1.0]],
    [[k, -1.0], [1.0, -k], [1.0, 0.0]],
  ];
  let reverse = [
    [[k, 1.0], [1.0, k], [1.0, 0.0]],
    [[-1.0, k], [-k, 1.0], [0.0, 1.0]],
    [[-k, -1.0], [-1.0, -k], [-1.0, 0.0]],
    [[1.0, -k], [k, -1.0], [0.0, -1.0]],
  ];
  let forward = sweep.0 > 0.0;
  while if forward {
    end > at + HALF_PI
  } else {
    at - HALF_PI > end
  } {
    quadrant += if forward { 1 } else { -1 };
    at = if forward { at + HALF_PI } else { at - HALF_PI };
    let table = if forward { &positive } else { &reverse };
    for point in table[quadrant.rem_euclid(4) as usize] {
      points.push(integer(point.map(Down))?);
    }
  }
  if (at - end).abs().0 > 1e-8 {
    for point in &partial(at, end)[1..] {
      points.push(integer(*point)?);
    }
  }
  Some(points)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn native_vml_arc_integer_controls_preserve_partial_quarter_endpoints() {
    // Actual Office constructor inputs from nine authored-angle controls.
    // Adjacent -181/-179 degree cases exercise downward range reduction at
    // the full-turn endpoint, independently of PDF pixels or glyph layout.
    let cases: &[(i32, i32, &[[i32; 2]])] = &[
      (
        -180,
        -180,
        &[
          [0, 10800],
          [0, 4835],
          [4835, 0],
          [10800, 0],
          [16764, 0],
          [21599, 4835],
          [21599, 10799],
        ],
      ),
      (-180, -90, &[[0, 10800], [0, 4835], [4835, 0], [10799, 0]]),
      (
        -135,
        -135,
        &[
          [3163, 18436],
          [1137, 16411],
          [0, 13664],
          [0, 10800],
          [0, 4835],
          [4835, 0],
          [10799, 0],
        ],
      ),
      (
        -90,
        -180,
        &[
          [10800, 21599],
          [4835, 21600],
          [0, 16764],
          [0, 10800],
          [0, 4835],
          [4835, 0],
          [10799, 0],
        ],
      ),
      (
        45,
        120,
        &[
          [18436, 3163],
          [16411, 1137],
          [13664, 0],
          [10800, 0],
          [5911, 0],
          [1633, 3283],
          [368, 8004],
        ],
      ),
      (90, 90, &[[10799, 0], [4835, 0], [0, 4835], [0, 10799]]),
      (
        -181,
        -179,
        &[
          [1, 10611],
          [104, 4721],
          [4908, 0],
          [10799, 0],
          [16764, 0],
          [21600, 4835],
          [21600, 10800],
        ],
      ),
      (
        -179,
        -181,
        &[
          [1, 10988],
          [0, 10925],
          [0, 10862],
          [0, 10800],
          [0, 4835],
          [4835, 0],
          [10800, 0],
          [16764, 0],
          [21600, 4835],
          [21600, 10800],
        ],
      ),
      (
        0,
        -270,
        &[
          [21600, 10800],
          [21600, 16764],
          [16764, 21600],
          [10800, 21600],
          [4835, 21600],
          [0, 16764],
          [0, 10800],
          [0, 4835],
          [4835, 0],
          [10799, 0],
        ],
      ),
    ];
    for &(start, sweep, expected) in cases {
      assert_eq!(
        vml_arc_cubics([10800, 10800, 10800, 10800, start * 65536, sweep * 65536]).as_deref(),
        Some(expected),
        "start {start}, sweep {sweep}",
      );
    }
  }
}
