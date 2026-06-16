use crate::calc::numeric::{approx_sub, kahan_sum};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PercentileKind {
  Inc,
  Exc,
}

pub fn mean(values: &[f64]) -> Option<f64> {
  (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
}

pub fn variance_slice(values: &[f64], sample: bool) -> Option<f64> {
  if values.is_empty() || (sample && values.len() < 2) {
    return None;
  }
  let mean = kahan_sum(values.iter().copied()) / values.len() as f64;
  let sum = values
    .iter()
    .map(|value| {
      let delta = approx_sub(*value, mean);
      delta * delta
    })
    .sum::<f64>();
  Some(
    sum
      / if sample {
        values.len() - 1
      } else {
        values.len()
      } as f64,
  )
}

pub fn percentile_sorted(values: &mut [f64], k: f64, kind: PercentileKind) -> Option<f64> {
  if values.is_empty() {
    return None;
  }
  values.sort_by(f64::total_cmp);
  let count = values.len() as f64;
  let rank = match kind {
    PercentileKind::Inc => 1.0 + k * (count - 1.0),
    PercentileKind::Exc => k * (count + 1.0),
  };
  if rank < 1.0 || rank > count {
    return None;
  }
  let lower = rank.floor();
  let upper = rank.ceil();
  let lower_value = values[(lower as usize).saturating_sub(1)];
  if lower == upper {
    return Some(lower_value);
  }
  let upper_value = values[(upper as usize).saturating_sub(1)];
  Some(lower_value + (rank - lower) * (upper_value - lower_value))
}

pub fn mode_slice(values: &[f64]) -> Option<f64> {
  let mut values = values.to_vec();
  values.sort_by(f64::total_cmp);
  let mut best_value = None;
  let mut best_count = 1usize;
  let mut current_value = None;
  let mut current_count = 0usize;
  for value in values {
    if current_value == Some(value) {
      current_count += 1;
    } else {
      current_value = Some(value);
      current_count = 1;
    }
    if current_count > best_count {
      best_count = current_count;
      best_value = current_value;
    }
  }
  best_value
}

pub fn mode_ms_values(values: &[f64]) -> Option<Vec<f64>> {
  let mut counts: Vec<(f64, usize)> = Vec::new();
  for value in values {
    if let Some((_, count)) = counts
      .iter_mut()
      .find(|(candidate, _)| *candidate == *value)
    {
      *count += 1;
    } else {
      counts.push((*value, 1));
    }
  }
  let max_count = counts.iter().map(|(_, count)| *count).max().unwrap_or(0);
  if max_count <= 1 {
    return None;
  }
  let mut modes = Vec::new();
  for value in values {
    if modes.iter().any(|mode| mode == value) {
      continue;
    }
    if counts
      .iter()
      .any(|(candidate, count)| *candidate == *value && *count == max_count)
    {
      modes.push(*value);
    }
  }
  Some(modes)
}

pub fn kth_value(mut values: Vec<f64>, k: f64, descending: bool) -> Option<f64> {
  if k < 1.0 || values.is_empty() {
    return None;
  }
  values.sort_by(f64::total_cmp);
  if descending {
    values.reverse();
  }
  values.get(k as usize - 1).copied()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn variance_and_percentiles_match_spreadsheet_rules() {
    assert_eq!(variance_slice(&[1.0, 2.0, 3.0], false), Some(2.0 / 3.0));
    assert_eq!(variance_slice(&[1.0, 2.0, 3.0], true), Some(1.0));
    let mut values = [1.0, 3.0, 2.0, 4.0];
    assert_eq!(
      percentile_sorted(&mut values, 0.5, PercentileKind::Inc),
      Some(2.5)
    );
    assert_eq!(
      percentile_sorted(&mut values, 0.5, PercentileKind::Exc),
      Some(2.5)
    );
  }

  #[test]
  fn mode_and_kth_preserve_existing_order_rules() {
    assert_eq!(mode_slice(&[2.0, 1.0, 2.0, 1.0]), Some(1.0));
    assert_eq!(mode_ms_values(&[2.0, 1.0, 2.0, 1.0]), Some(vec![2.0, 1.0]));
    assert_eq!(kth_value(vec![3.0, 1.0, 2.0], 2.0, false), Some(2.0));
    assert_eq!(kth_value(vec![3.0, 1.0, 2.0], 2.0, true), Some(2.0));
  }
}
