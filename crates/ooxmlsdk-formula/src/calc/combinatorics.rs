const MAX_EXACT_F64_INTEGER: u64 = 1_u64 << 53;

pub fn gcd_number(left: f64, right: f64) -> f64 {
  let left = left.abs();
  let right = right.abs();
  if let (Some(left), Some(right)) = (exact_u64(left), exact_u64(right)) {
    return num_integer::gcd(left, right) as f64;
  }
  gcd_number_f64(left, right)
}

pub fn lcm_number(left: f64, right: f64) -> f64 {
  let left = left.abs();
  let right = right.abs();
  if left == 0.0 || right == 0.0 {
    return 0.0;
  }
  if let (Some(left), Some(right)) = (exact_u64(left), exact_u64(right)) {
    let gcd = num_integer::gcd(left, right) as u128;
    return ((left as u128 / gcd) * right as u128) as f64;
  }
  (left / gcd_number_f64(left, right) * right).abs()
}

pub fn combination_count(
  mut count: f64,
  chosen: f64,
  repetition: bool,
  log_gamma: impl Fn(f64) -> f64,
) -> Option<f64> {
  if count < 0.0 || chosen < 0.0 || chosen > count {
    return None;
  }
  if repetition {
    if count == 0.0 && chosen == 0.0 {
      return Some(0.0);
    }
    count += chosen - 1.0;
  }
  Some(
    small_exact_binomial(count, chosen)
      .unwrap_or_else(|| log_gamma_combination(count, chosen, log_gamma)),
  )
}

pub fn permutation_count(count: f64, chosen: f64, log_gamma: impl Fn(f64) -> f64) -> Option<f64> {
  if count < 0.0 || chosen < 0.0 || chosen > count {
    return None;
  }
  Some(small_exact_permutation(count, chosen).unwrap_or_else(|| {
    (log_gamma(count + 1.0) - log_gamma(count - chosen + 1.0))
      .exp()
      .round()
  }))
}

pub fn permutation_with_repetition_count(count: f64, chosen: f64) -> Option<f64> {
  if count < 0.0 || chosen < 0.0 {
    None
  } else {
    Some(count.powf(chosen))
  }
}

fn gcd_number_f64(mut left: f64, mut right: f64) -> f64 {
  while right != 0.0 {
    let remainder = left % right;
    left = right;
    right = remainder;
  }
  left
}

fn exact_u64(value: f64) -> Option<u64> {
  if value.is_finite()
    && (0.0..=MAX_EXACT_F64_INTEGER as f64).contains(&value)
    && value.fract() == 0.0
  {
    Some(value as u64)
  } else {
    None
  }
}

fn small_exact_binomial(count: f64, chosen: f64) -> Option<f64> {
  let count = exact_u64(count)?;
  let chosen = exact_u64(chosen)?;
  if count > 67 {
    return None;
  }
  let value = num_integer::binomial(count, chosen);
  (value <= MAX_EXACT_F64_INTEGER).then_some(value as f64)
}

fn small_exact_permutation(count: f64, chosen: f64) -> Option<f64> {
  let count = exact_u64(count)?;
  let chosen = exact_u64(chosen)?;
  let mut value = 1_u64;
  for factor in (count - chosen + 1)..=count {
    value = value.checked_mul(factor)?;
    if value > MAX_EXACT_F64_INTEGER {
      return None;
    }
  }
  Some(value as f64)
}

fn log_gamma_combination(count: f64, chosen: f64, log_gamma: impl Fn(f64) -> f64) -> f64 {
  (log_gamma(count + 1.0) - log_gamma(chosen + 1.0) - log_gamma(count - chosen + 1.0))
    .exp()
    .round()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn lgamma(value: f64) -> f64 {
    statrs::function::gamma::ln_gamma(value)
  }

  #[test]
  fn gcd_lcm_use_integer_results_for_exact_values() {
    assert_eq!(gcd_number(54.0, 24.0), 6.0);
    assert_eq!(lcm_number(21.0, 6.0), 42.0);
    assert_eq!(gcd_number(0.0, 12.0), 12.0);
    assert_eq!(lcm_number(0.0, 12.0), 0.0);
  }

  #[test]
  fn combination_and_permutation_match_spreadsheet_counts() {
    assert_eq!(combination_count(5.0, 2.0, false, lgamma), Some(10.0));
    assert_eq!(combination_count(5.0, 2.0, true, lgamma), Some(15.0));
    assert_eq!(combination_count(0.0, 0.0, true, lgamma), Some(0.0));
    assert_eq!(permutation_count(5.0, 2.0, lgamma), Some(20.0));
    assert_eq!(permutation_with_repetition_count(5.0, 2.0), Some(25.0));
  }
}
