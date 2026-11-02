//! Word Line Services balances document advances against hinted printer widths.
//! Native complete-line input/output observations establish the signed carry,
//! neighbouring-error redistribution and 32-bit accumulator restart.

const SCALE: i64 = 4267; // round(600 * 2^21 / (72 * 4096))
const HALF: i64 = 1 << 20;

/// Largest nonnegative 1/4096-point advance before the native multiply resets.
pub const MAX_IDEAL_ADVANCE: i64 = (i32::MAX as i64 - HALF) / SCALE;

/// Balance nonnegative (ideal 1/4096-point, natural printer-dot) advances.
/// Keep one batch across all face/style runs belonging to the same line.
pub fn balance_advances(advances: &[(i32, i32)]) -> Vec<i32> {
  let mut output: Vec<i32> = Vec::with_capacity(advances.len());
  let mut ideal_sum = 0i64;
  let mut previous_rounded = 0;
  let mut previous_error = 0;
  let mut previous = false;
  for &(ideal, natural) in advances {
    if ideal_sum + i64::from(ideal) > MAX_IDEAL_ADVANCE {
      // Native restarts before its signed 32-bit multiply would overflow.
      ideal_sum = 0;
      previous_rounded = 0;
      previous = false;
    }
    ideal_sum += i64::from(ideal);
    let rounded = ((ideal_sum * SCALE + HALF) >> 21) as i32;
    let mut value = rounded - previous_rounded;
    let error = natural - value;
    if previous {
      let difference = error - previous_error;
      let floor = difference >> 1;
      let ceil = floor + (difference & 1);
      let prior = output.last_mut().expect("previous advance");
      let delta = if difference > 0 {
        (*prior).min(if previous_error >= -error {
          floor
        } else {
          ceil
        })
      } else if difference < 0 {
        (-value).max(if error >= -previous_error {
          floor
        } else {
          ceil
        })
      } else {
        0
      };
      *prior -= delta;
      value += delta;
    }
    output.push(value);
    previous_error = natural - value;
    previous_rounded = rounded;
    previous = true;
  }
  output
}

/// Complex-script lines retain a small cumulative printer error inside words.
/// Spaces settle the error; larger discrepancies can move width between the
/// preceding and current glyph. Native complete Low/Medium/High Kashida lines
/// establish the asymmetric three/five-dot limits and contraction budget.
/// The input contains positive base-glyph advances in logical order, after
/// justification and before inserting the font's ink extenders.
pub fn balance_complex_advances(advances: &[(i32, i32, bool)], maximum_shrink: i32) -> Vec<i32> {
  let mut output: Vec<_> = advances.iter().map(|&(_, natural, _)| natural).collect();
  let mut previous_ideal: Vec<_> = advances.iter().map(|&(ideal, _, _)| ideal).collect();
  let mut ideal_sum = 0i64;
  let mut device_sum = 0i64;
  let mut removed = 0i64;
  let mut previous_delta = 0;
  for (index, &(ideal, _, space)) in advances.iter().enumerate() {
    ideal_sum += i64::from(ideal);
    let rounded = if (0..=MAX_IDEAL_ADVANCE).contains(&ideal_sum) {
      (ideal_sum * SCALE + HALF) >> 21
    } else {
      let product = ideal_sum * 600;
      (product + product.signum() * (294912 / 2)) / 294912
    };
    let desired = (rounded - device_sum) as i32;
    let error = output[index] - desired;
    let mut delta = 0;
    if space {
      if error < 0 {
        delta = error;
        output[index] = desired;
        removed += i64::from(delta);
      } else if error > 0
        && removed < i64::from(maximum_shrink)
        && !(index != 0 && previous_ideal[index - 1] <= previous_delta && error < 3)
      {
        delta = error.min((output[index] - 1) / 2);
        if error - delta > 3 && delta <= output[index] {
          delta = (error - 3).min(output[index]);
        }
        output[index] -= delta;
        previous_ideal[index] = 0;
        if index != 0 {
          previous_ideal[index - 1] = 0;
        }
        removed += i64::from(delta);
      }
    } else if error > 3 && removed < i64::from(maximum_shrink) {
      delta = error - 3;
      if index != 0 && output[index - 1] > 0 && delta > previous_delta + 1 {
        let preceding = ((delta - previous_delta) / 2).min(output[index - 1]);
        output[index - 1] -= preceding;
        device_sum -= i64::from(preceding);
        removed += i64::from(preceding);
        delta -= preceding;
        previous_ideal[index - 1] = 0;
      }
      if delta > 0 {
        delta = delta.min(output[index]);
        output[index] -= delta;
        removed += i64::from(delta);
        previous_ideal[index] = 0;
      }
    } else if error < -5 {
      delta = error + 5;
      if index != 0 {
        output[index - 1] += 1;
        device_sum += 1;
        delta += 1;
      }
      if delta < 0 {
        output[index] -= delta;
        removed += i64::from(delta);
      }
    }
    device_sum += i64::from(output[index]);
    previous_delta = delta;
  }
  output
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn complex_script_line_balance_matches_native_kashida_lines() {
    // Complete native printer input/output for Low and High Kashida. These
    // cover repeated Yeh, finite contraction, spaces and the long-line
    // rational fallback beyond the signed fixed-point multiply range.
    let input = [
      (17754, 36, false),
      (11000, 22, false),
      (23694, 48, false),
      (12518, 26, true),
      (12056, 25, false),
      (10340, 21, false),
      (22022, 45, false),
      (24338, 50, false),
      (26510, 54, false),
      (12518, 26, true),
      (11000, 22, false),
      (10340, 21, false),
      (12518, 26, true),
      (17754, 36, false),
      (9328, 19, false),
      (17754, 36, false),
      (10340, 21, false),
      (32142, 66, false),
      (12518, 26, true),
      (17754, 36, false),
      (19470, 40, false),
      (11000, 22, false),
      (10340, 21, false),
      (12716, 26, false),
      (12518, 26, false),
      (12518, 26, true),
      (17754, 36, false),
      (11000, 22, false),
      (23694, 48, false),
      (12518, 26, true),
      (12056, 25, false),
      (10340, 21, false),
      (22022, 45, false),
      (24338, 50, false),
      (26510, 54, false),
      (12518, 26, true),
      (11000, 22, false),
      (10340, 21, false),
      (12518, 26, true),
      (17754, 36, false),
      (9328, 19, false),
      (17754, 36, false),
      (10340, 21, false),
      (32142, 66, false),
      (12518, 26, true),
      (23870, 49, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (11000, 22, false),
      (9328, 19, false),
      (26510, 54, false),
      (12518, 26, true),
      (26202, 54, false),
      (19470, 40, false),
      (24508, 50, false),
      (11000, 22, false),
      (26510, 54, false),
      (12518, 26, false),
      (12518, 26, true),
      (17754, 36, false),
      (11000, 22, false),
      (23694, 48, false),
      (12518, 26, true),
      (12056, 25, false),
      (10340, 21, false),
      (22022, 45, false),
      (23914, 49, false),
      (26510, 54, false),
      (12518, 26, true),
      (11000, 22, false),
      (10340, 21, false),
      (12518, 26, true),
      (23870, 49, false),
      (17754, 36, false),
      (27060, 55, false),
      (32142, 66, false),
      (12518, 26, true),
    ];
    let expected = [
      36, 22, 48, 26, 25, 21, 45, 50, 54, 24, 22, 21, 26, 36, 19, 36, 21, 66, 26, 36, 40, 22, 21,
      26, 26, 24, 36, 22, 48, 27, 25, 21, 45, 50, 54, 24, 22, 21, 26, 36, 19, 36, 21, 66, 25, 49,
      22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 22, 23, 22, 22, 23, 22, 22, 23, 22, 23,
      22, 22, 23, 22, 22, 23, 22, 23, 22, 22, 23, 22, 22, 23, 22, 23, 22, 22, 23, 22, 23, 22, 22,
      23, 22, 22, 23, 22, 23, 22, 22, 23, 22, 22, 19, 54, 30, 54, 40, 50, 22, 54, 26, 24, 36, 22,
      48, 27, 25, 21, 45, 49, 54, 24, 22, 21, 26, 49, 36, 55, 66, 25,
    ];
    assert_eq!(balance_complex_advances(&input, 2147483647), expected);
    let input = [
      (20148, 41, false),
      (13765, 28, false),
      (23694, 48, false),
      (18777, 39, true),
      (30833, 63, false),
      (10340, 21, false),
      (22022, 45, false),
      (63863, 130, false),
      (26510, 54, false),
      (18777, 39, true),
      (29777, 60, false),
      (10340, 21, false),
      (18777, 39, true),
      (20149, 41, false),
      (12093, 25, false),
      (36531, 74, false),
      (10340, 21, false),
      (32142, 66, false),
      (18777, 39, true),
      (20149, 41, false),
      (19470, 40, false),
      (29777, 60, false),
      (10340, 21, false),
      (12716, 26, false),
      (12518, 26, false),
      (18777, 39, true),
      (20149, 41, false),
      (13765, 28, false),
      (23694, 48, false),
      (18777, 39, true),
      (30833, 63, false),
      (10340, 21, false),
      (22022, 45, false),
      (63863, 130, false),
      (26510, 54, false),
      (18777, 39, true),
      (29777, 60, false),
      (10340, 21, false),
      (18777, 39, true),
      (20149, 41, false),
      (12093, 25, false),
      (36531, 74, false),
      (10340, 21, false),
      (32142, 66, false),
      (18777, 39, true),
      (26265, 54, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13764, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13764, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13764, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13764, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13764, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13764, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (13765, 28, false),
      (12093, 25, false),
      (26510, 54, false),
      (12518, 26, true),
    ];
    let expected = [
      41, 28, 48, 38, 63, 21, 45, 130, 54, 38, 60, 21, 39, 41, 25, 74, 21, 66, 37, 41, 40, 60, 21,
      26, 26, 38, 41, 28, 48, 39, 63, 21, 45, 130, 54, 37, 60, 21, 39, 41, 25, 74, 21, 66, 38, 54,
      28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
      28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
      28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 25, 54, 24,
    ];
    assert_eq!(balance_complex_advances(&input, 9), expected);
  }
}
