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
