pub(crate) fn round_direction(value: f64, digits: i32, away_from_zero: bool) -> f64 {
  if value == 0.0 || !value.is_finite() {
    return value;
  }
  const ROUND_SIGNIFICANT_DIGITS: i32 = 12;
  if digits < ROUND_SIGNIFICANT_DIGITS && value.fract() != 0.0 {
    let temp = value.abs().log10().floor() as i32 + 1 - ROUND_SIGNIFICANT_DIGITS;
    let scale = 10_f64.powi(temp.abs());
    let mut scaled_to_significant = if temp < 0 {
      value * scale
    } else {
      value / scale
    };
    if scaled_to_significant.is_finite() {
      if away_from_zero {
        scaled_to_significant = approx_floor(scaled_to_significant);
      }
      let rounded = round_direction_basic(scaled_to_significant, digits + temp, away_from_zero);
      return if temp < 0 {
        rounded / scale
      } else {
        rounded * scale
      };
    }
  }
  round_direction_basic(value, digits, away_from_zero)
}

fn round_direction_basic(value: f64, digits: i32, away_from_zero: bool) -> f64 {
  if value == 0.0 || !value.is_finite() {
    return value;
  }
  let factor = 10_f64.powi(digits.abs());
  if factor == 0.0 || !factor.is_finite() {
    return value;
  }
  let scaled = if digits < 0 {
    value / factor
  } else {
    value * factor
  };
  let rounded = if away_from_zero {
    if scaled.is_sign_negative() {
      approx_floor(scaled)
    } else {
      approx_ceil(scaled)
    }
  } else if scaled.is_sign_negative() {
    approx_ceil(scaled)
  } else {
    approx_floor(scaled)
  };
  if digits < 0 {
    rounded * factor
  } else {
    rounded / factor
  }
}

pub(crate) fn even_odd(value: f64, even: bool) -> f64 {
  if value == 0.0 {
    return if even { 0.0 } else { 1.0 };
  }
  let sign = if value.is_sign_negative() { -1.0 } else { 1.0 };
  let mut magnitude = value.abs().ceil();
  let is_even = (magnitude as i64) % 2 == 0;
  if is_even != even {
    magnitude += 1.0;
  }
  sign * magnitude
}

pub(crate) fn gcd_number(mut left: f64, mut right: f64) -> f64 {
  left = left.abs();
  right = right.abs();
  while right != 0.0 {
    let remainder = left % right;
    left = right;
    right = remainder;
  }
  left
}

pub(crate) fn lcm_number(left: f64, right: f64) -> f64 {
  if left == 0.0 || right == 0.0 {
    return 0.0;
  }
  (left / gcd_number(left, right) * right).abs()
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct KahanSum {
  sum: f64,
  error: f64,
  memory: f64,
}

impl KahanSum {
  pub(crate) fn add(&mut self, value: f64) {
    if value == 0.0 {
      return;
    }
    if self.memory == 0.0 {
      self.memory = value;
      return;
    }
    Self::sum_neumaier(&mut self.sum, &mut self.error, self.memory);
    self.memory = value;
  }

  pub(crate) fn finish(mut self) -> f64 {
    let total = self.sum + self.error;
    if self.memory == 0.0 {
      return total;
    }
    if ((self.memory < 0.0 && total > 0.0) || (total < 0.0 && self.memory > 0.0))
      && approx_equal(self.memory, -total)
    {
      return 0.0;
    }
    Self::sum_neumaier(&mut self.sum, &mut self.error, self.memory);
    self.memory = 0.0;
    self.sum + self.error
  }

  fn sum_neumaier(sum: &mut f64, error: &mut f64, value: f64) {
    let next = *sum + value;
    if sum.abs() >= value.abs() {
      *error += (*sum - next) + value;
    } else {
      *error += (value - next) + *sum;
    }
    *sum = next;
  }
}

pub(crate) fn kahan_sum(values: impl IntoIterator<Item = f64>) -> f64 {
  let mut sum = KahanSum::default();
  for value in values {
    sum.add(value);
  }
  sum.finish()
}

#[inline]
pub(crate) fn approx_floor(value: f64) -> f64 {
  approx_value(value).floor()
}

#[inline]
pub(crate) fn approx_ceil(value: f64) -> f64 {
  approx_value(value).ceil()
}

pub(crate) fn approx_equal(left: f64, right: f64) -> bool {
  if left == right {
    return true;
  }
  if left == 0.0 || right == 0.0 || left.is_sign_negative() != right.is_sign_negative() {
    return false;
  }
  let difference = (left - right).abs();
  if !difference.is_finite() {
    return false;
  }
  let left = left.abs();
  let right = right.abs();
  let min_value = left.min(right);
  let threshold1 = min_value * 2_f64.powi(-48);
  let threshold2 = pow10_exp(min_value.log10().floor() as i32) * 5.0e-15;
  if difference >= threshold1.max(threshold2) {
    return false;
  }
  !(is_representable_integer(left) && is_representable_integer(right))
}

pub(crate) fn approx_sub(left: f64, right: f64) -> f64 {
  if ((left < 0.0 && right < 0.0) || (left > 0.0 && right > 0.0)) && approx_equal(left, right) {
    return 0.0;
  }
  normalize_duration_difference(left, right, left - right)
}

pub(crate) fn approx_add(left: f64, right: f64) -> f64 {
  if ((left < 0.0 && right > 0.0) || (right < 0.0 && left > 0.0)) && approx_equal(left, -right) {
    return 0.0;
  }
  left + right
}

pub(crate) fn normalize_formula_number(value: f64) -> f64 {
  if approx_equal(value, 0.0) { 0.0 } else { value }
}

fn normalize_duration_difference(left: f64, right: f64, value: f64) -> f64 {
  if value != 0.0
    && value.abs() < 1.0
    && (left.abs() >= 1.0 || right.abs() >= 1.0)
    && (left - right).abs() <= i32::MAX as f64
  {
    let micros_per_day = 86_400_000_000.0;
    return (value * micros_per_day).round() / micros_per_day;
  }
  value
}

pub(crate) fn approx_value(value: f64) -> f64 {
  const TWO_POW_41: f64 = 2_199_023_255_552.0;
  if value == 0.0 || !value.is_finite() || value.abs() > TWO_POW_41 {
    return value;
  }
  let sign = value.is_sign_negative();
  let positive = value.abs();
  if positive.fract() == 0.0 || fraction_bit_count(positive) <= 11 {
    return value;
  }
  let exp = 14 - positive.log10().floor() as i32;
  let scale = 10_f64.powi(exp.abs());
  let rounded = if exp < 0 {
    (positive / scale).round() * scale
  } else {
    (positive * scale).round() / scale
  };
  if !rounded.is_finite() {
    return value;
  }
  if sign { -rounded } else { rounded }
}

fn pow10_exp(exp: i32) -> f64 {
  10_f64.powi(exp)
}

pub(crate) fn is_representable_integer(value: f64) -> bool {
  value.is_finite() && value >= 0.0 && value < 2_f64.powi(53) && value.fract() == 0.0
}

fn fraction_bit_count(value: f64) -> u32 {
  if value <= 0.0 || !value.is_finite() {
    return 0;
  }
  let bits = value.to_bits();
  let exponent = ((bits >> 52) & 0x7ff) as i32 - 1023;
  if exponent >= 52 {
    return 0;
  }
  let fraction = bits & ((1_u64 << 52) - 1);
  let least_significant = if fraction == 0 {
    53
  } else {
    fraction.trailing_zeros() + 1
  };
  let fraction_significant = 53 - least_significant as i32;
  (fraction_significant - exponent).max(0) as u32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn kahan_sum_cancels_approximate_zero() {
    let mut sum = KahanSum::default();
    sum.add(0.1 + 0.2);
    sum.add(-0.3);
    assert_eq!(sum.finish(), 0.0);
  }

  #[test]
  fn approx_rounding_helpers_normalize_binary_noise() {
    assert_eq!(approx_floor(3.000000000000001), 3.0);
    assert_eq!(approx_ceil(2.999999999999999), 3.0);
    assert!(approx_equal(0.1 + 0.2, 0.3));
  }
}
