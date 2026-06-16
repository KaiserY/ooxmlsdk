pub use crate::calc::combinatorics::{gcd_number, lcm_number};

use num_traits::ToPrimitive;

pub fn round_direction(value: f64, digits: i32, away_from_zero: bool) -> f64 {
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

pub fn round_to_decimal_places(value: f64, decimal_places: i32) -> f64 {
  if !value.is_finite() || value == 0.0 {
    return value;
  }
  let original = value;
  let sign = value.is_sign_negative();
  let mut value = value.abs();
  if decimal_places >= 0 && (value >= 2_f64.powi(52) || is_representable_integer(value)) {
    return original;
  }
  let mut places = decimal_places;
  let mut factor = 0.0;
  if places != 0 {
    if places > 0 {
      let exponent = ((value.to_bits() >> 52) & 0x7ff) as i32 - 1023;
      let decimals = 52 - exponent;
      if decimals <= 0 {
        return original;
      }
      if decimals < places {
        places = decimals;
      }
    }
    factor = 10_f64.powi(places.abs());
    if factor == 0.0 || (places < 0 && !factor.is_finite()) {
      return 0.0;
    }
    if !factor.is_finite() {
      return original;
    }
    if places < 0 {
      value /= factor;
    } else {
      value *= factor;
    }
    if !value.is_finite() {
      return original;
    }
  }
  if value < 2_f64.powi(52) {
    value = approx_floor(value + 0.5);
  }
  if places != 0 {
    if places < 0 {
      value *= factor;
    } else {
      value /= factor;
    }
  }
  if !value.is_finite() {
    return original;
  }
  if sign { -value } else { value }
}

pub fn round_to_significant_digits(value: f64, digits: f64) -> f64 {
  let scale = value.abs().log10().floor() + 1.0 - digits;
  let mut input = value;
  let factor = 10.0_f64.powf(scale.abs());
  if scale < 0.0 {
    input *= factor;
  } else {
    input /= factor;
  }
  let mut result = round_to_decimal_places(input, 0);
  if scale < 0.0 {
    result /= factor;
  } else {
    result *= factor;
  }
  result
}

pub fn round_half_away_from_zero(value: f64, digits: i32) -> f64 {
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
  let rounded = if scaled.is_sign_negative() {
    (scaled - 0.5).ceil()
  } else {
    (scaled + 0.5).floor()
  };
  if digits < 0 {
    rounded * factor
  } else {
    rounded / factor
  }
}

#[inline]
pub fn floor_to_u16(value: f64) -> Option<u16> {
  value.is_finite().then(|| value.floor())?.to_u16()
}

#[inline]
pub fn floor_to_u32(value: f64) -> Option<u32> {
  value.is_finite().then(|| value.floor())?.to_u32()
}

#[inline]
pub fn floor_to_u64(value: f64) -> Option<u64> {
  value.is_finite().then(|| value.floor())?.to_u64()
}

#[inline]
pub fn floor_to_usize(value: f64) -> Option<usize> {
  value.is_finite().then(|| value.floor())?.to_usize()
}

#[inline]
pub fn floor_to_i32(value: f64) -> Option<i32> {
  value.is_finite().then(|| value.floor())?.to_i32()
}

#[inline]
pub fn trunc_to_i32(value: f64) -> Option<i32> {
  value.is_finite().then(|| value.trunc())?.to_i32()
}

#[inline]
pub fn trunc_to_u32(value: f64) -> Option<u32> {
  value.is_finite().then(|| value.trunc())?.to_u32()
}

#[inline]
pub fn trunc_to_u64(value: f64) -> Option<u64> {
  value.is_finite().then(|| value.trunc())?.to_u64()
}

#[inline]
pub fn trunc_to_usize(value: f64) -> Option<usize> {
  value.is_finite().then(|| value.trunc())?.to_usize()
}

pub fn even_odd(value: f64, even: bool) -> f64 {
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

#[derive(Clone, Copy, Debug, Default)]
pub struct KahanSum {
  sum: f64,
  error: f64,
  memory: f64,
}

impl KahanSum {
  pub fn add(&mut self, value: f64) {
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

  pub fn finish(mut self) -> f64 {
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

pub fn kahan_sum(values: impl IntoIterator<Item = f64>) -> f64 {
  let mut sum = KahanSum::default();
  for value in values {
    sum.add(value);
  }
  sum.finish()
}

#[inline]
pub fn approx_floor(value: f64) -> f64 {
  approx_value(value).floor()
}

#[inline]
pub fn approx_ceil(value: f64) -> f64 {
  approx_value(value).ceil()
}

pub fn approx_equal(left: f64, right: f64) -> bool {
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

pub fn approx_sub(left: f64, right: f64) -> f64 {
  if ((left < 0.0 && right < 0.0) || (left > 0.0 && right > 0.0)) && approx_equal(left, right) {
    return 0.0;
  }
  normalize_duration_difference(left, right, left - right)
}

pub fn approx_add(left: f64, right: f64) -> f64 {
  if ((left < 0.0 && right > 0.0) || (right < 0.0 && left > 0.0)) && approx_equal(left, -right) {
    return 0.0;
  }
  left + right
}

pub fn normalize_formula_number(value: f64) -> f64 {
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

pub fn approx_value(value: f64) -> f64 {
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

pub fn is_representable_integer(value: f64) -> bool {
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

  #[test]
  fn decimal_rounding_helpers_preserve_spreadsheet_edges() {
    assert_eq!(round_to_decimal_places(2.5, 0), 3.0);
    assert_eq!(round_to_decimal_places(-2.5, 0), -3.0);
    assert_eq!(round_to_decimal_places(1234.5, -2), 1200.0);
    assert_eq!(round_to_significant_digits(12345.0, 3.0), 12300.0);
    assert_eq!(round_half_away_from_zero(-1.25, 1), -1.3);
  }

  #[test]
  fn numeric_conversions_reject_non_finite_and_out_of_range_values() {
    assert_eq!(floor_to_u16(42.9), Some(42));
    assert_eq!(floor_to_i32(-1.2), Some(-2));
    assert_eq!(floor_to_usize(7.8), Some(7));
    assert_eq!(trunc_to_i32(-1.9), Some(-1));
    assert_eq!(trunc_to_u32(-1.0), None);
    assert_eq!(trunc_to_usize(3.9), Some(3));
    assert_eq!(floor_to_u64(f64::INFINITY), None);
  }
}
