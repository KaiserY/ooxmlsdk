use crate::calc::numeric::{KahanSum, approx_floor, approx_sub, kahan_sum};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PercentileKind {
  Inc,
  Exc,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatisticsError {
  Div0,
  IllegalArgument,
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

pub fn sum_x2(left: &[f64], right: &[f64], plus: bool) -> Option<f64> {
  if left.len() != right.len() {
    return None;
  }
  let mut total = KahanSum::default();
  for (left, right) in left.iter().zip(right) {
    total.add(left * left);
    if plus {
      total.add(right * right);
    } else {
      total.add(-(right * right));
    }
  }
  Some(total.finish())
}

pub fn sum_xmy2(left: &[f64], right: &[f64]) -> Option<f64> {
  if left.len() != right.len() {
    return None;
  }
  let mut total = KahanSum::default();
  for (left, right) in left.iter().zip(right) {
    let difference = approx_sub(*left, *right);
    total.add(difference * difference);
  }
  Some(total.finish())
}

pub fn frequency_counts(mut data: Vec<f64>, bins: &[f64]) -> Option<Vec<usize>> {
  if data.is_empty() {
    return None;
  }
  data.sort_by(f64::total_cmp);
  let mut ordered_bins = bins
    .iter()
    .copied()
    .enumerate()
    .map(|(index, value)| (value, index))
    .collect::<Vec<_>>();
  ordered_bins.sort_by(|left, right| left.0.total_cmp(&right.0));
  let mut counts = vec![0usize; bins.len() + 1];
  let mut data_index = 0;
  for (bin_value, original_index) in ordered_bins {
    let mut count = 0;
    while data_index < data.len() && data[data_index] <= bin_value {
      count += 1;
      data_index += 1;
    }
    counts[original_index] = count;
  }
  counts[bins.len()] = data.len() - data_index;
  Some(counts)
}

pub fn trim_mean(mut values: Vec<f64>, alpha: f64) -> Option<f64> {
  if values.is_empty() || !(0.0..1.0).contains(&alpha) {
    return None;
  }
  values.sort_by(f64::total_cmp);
  let mut trim = approx_floor(alpha * values.len() as f64) as usize;
  if !trim.is_multiple_of(2) {
    trim -= 1;
  }
  trim /= 2;
  if trim * 2 >= values.len() {
    return None;
  }
  let kept = &values[trim..values.len() - trim];
  Some(kahan_sum(kept.iter().copied()) / kept.len() as f64)
}

pub fn rank_value(mut values: Vec<f64>, value: f64, ascending: bool, average: bool) -> Option<f64> {
  if values.is_empty() {
    return None;
  }
  values.sort_by(f64::total_cmp);
  let before = if ascending {
    values
      .iter()
      .filter(|candidate| **candidate < value)
      .count()
  } else {
    values
      .iter()
      .filter(|candidate| **candidate > value)
      .count()
  };
  let matches = values
    .iter()
    .filter(|candidate| **candidate == value)
    .count();
  if matches == 0 {
    return None;
  }
  let first = before as f64 + 1.0;
  Some(if average {
    first + (matches as f64 - 1.0) / 2.0
  } else {
    first
  })
}

pub fn deviation_sum_squares(values: &[f64]) -> Option<f64> {
  if values.is_empty() {
    return None;
  }
  let mean = kahan_sum(values.iter().copied()) / values.len() as f64;
  Some(kahan_sum(values.iter().map(|value| {
    let delta = approx_sub(*value, mean);
    delta * delta
  })))
}

pub fn skewness(values: &[f64], population: bool) -> Result<f64, StatisticsError> {
  let count = values.len();
  if count < 3 {
    return Err(StatisticsError::Div0);
  }
  let count_f64 = count as f64;
  let mean = kahan_sum(values.iter().copied()) / count_f64;
  let variance_sum = kahan_sum(values.iter().map(|value| {
    let diff = approx_sub(*value, mean);
    diff * diff
  }));
  let std_dev = (variance_sum
    / if population {
      count_f64
    } else {
      count_f64 - 1.0
    })
  .sqrt();
  if std_dev == 0.0 {
    return Err(StatisticsError::IllegalArgument);
  }
  let cube_sum = kahan_sum(values.iter().map(|value| {
    let normalized = approx_sub(*value, mean) / std_dev;
    normalized * normalized * normalized
  }));
  Ok(if population {
    cube_sum / count_f64
  } else {
    cube_sum * count_f64 / (count_f64 - 1.0) / (count_f64 - 2.0)
  })
}

pub fn kurtosis(values: &[f64]) -> Result<f64, StatisticsError> {
  let count = values.len();
  if count < 4 {
    return Err(StatisticsError::Div0);
  }
  let mean = kahan_sum(values.iter().copied()) / count as f64;
  let mut second = KahanSum::default();
  let mut fourth = KahanSum::default();
  for value in values {
    let diff = approx_sub(*value, mean);
    let square = diff * diff;
    second.add(square);
    fourth.add(square * square);
  }
  let second = second.finish();
  if second == 0.0 {
    return Err(StatisticsError::Div0);
  }
  let count_f64 = count as f64;
  let sample_variance = second / (count_f64 - 1.0);
  let sample_fourth = fourth.finish() / (sample_variance * sample_variance);
  Ok(
    count_f64 * (count_f64 + 1.0) * sample_fourth
      / ((count_f64 - 1.0) * (count_f64 - 2.0) * (count_f64 - 3.0))
      - 3.0 * (count_f64 - 1.0) * (count_f64 - 1.0) / ((count_f64 - 2.0) * (count_f64 - 3.0)),
  )
}

pub fn covariance(pairs: &[(f64, f64)], sample: bool) -> Option<f64> {
  let count = pairs.len();
  if count == 0 || (sample && count < 2) {
    return None;
  }
  let left_mean = kahan_sum(pairs.iter().map(|(left, _)| *left)) / count as f64;
  let right_mean = kahan_sum(pairs.iter().map(|(_, right)| *right)) / count as f64;
  let sum = kahan_sum(
    pairs
      .iter()
      .map(|(left, right)| approx_sub(*left, left_mean) * approx_sub(*right, right_mean)),
  );
  Some(sum / if sample { count - 1 } else { count } as f64)
}

pub fn correlation(left: &[f64], right: &[f64]) -> Option<f64> {
  let count = left.len().min(right.len());
  if count < 2 {
    return None;
  }
  let left_mean = kahan_sum(left.iter().take(count).copied()) / count as f64;
  let right_mean = kahan_sum(right.iter().take(count).copied()) / count as f64;
  let mut numerator = KahanSum::default();
  let mut left_sum = KahanSum::default();
  let mut right_sum = KahanSum::default();
  for index in 0..count {
    let left_delta = approx_sub(left[index], left_mean);
    let right_delta = approx_sub(right[index], right_mean);
    numerator.add(left_delta * right_delta);
    left_sum.add(left_delta * left_delta);
    right_sum.add(right_delta * right_delta);
  }
  let left_sum = left_sum.finish();
  let right_sum = right_sum.finish();
  if left_sum == 0.0 || right_sum == 0.0 {
    return None;
  }
  Some(numerator.finish() / (left_sum * right_sum).sqrt())
}

pub fn percent_rank(
  mut values: Vec<f64>,
  x: f64,
  significance: f64,
  kind: PercentileKind,
) -> Option<f64> {
  values.sort_by(f64::total_cmp);
  if values.is_empty() || x < *values.first()? || x > *values.last()? {
    return None;
  }
  if values.len() == 1 {
    return Some(1.0);
  }
  let round_result = |value: f64| {
    if value == 0.0 {
      value
    } else {
      let exp = value.abs().log10().floor() + 1.0 - significance;
      (value * 10f64.powf(-exp)).round() / 10f64.powf(-exp)
    }
  };

  let size = values.len();
  let result = if x == values[0] {
    match kind {
      PercentileKind::Inc => 0.0,
      PercentileKind::Exc => 1.0 / (size + 1) as f64,
    }
  } else {
    let mut old_count = 0usize;
    let mut old_value = values[0];
    let mut index = 1usize;
    while index < size && values[index] < x {
      if values[index] != old_value {
        old_count = index;
        old_value = values[index];
      }
      index += 1;
    }
    if values[index] != old_value {
      old_count = index;
    }
    if x == values[index] {
      match kind {
        PercentileKind::Inc => old_count as f64 / (size - 1) as f64,
        PercentileKind::Exc => (index + 1) as f64 / (size + 1) as f64,
      }
    } else if old_count == 0 {
      0.0
    } else {
      let fraction = (x - values[old_count - 1]) / (values[old_count] - values[old_count - 1]);
      match kind {
        PercentileKind::Inc => (old_count as f64 - 1.0 + fraction) / (size - 1) as f64,
        PercentileKind::Exc => (old_count as f64 + fraction) / (size + 1) as f64,
      }
    }
  };
  Some(round_result(result))
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

  #[test]
  fn vector_stats_preserve_spreadsheet_ordering_rules() {
    assert_eq!(sum_x2(&[1.0, 2.0], &[3.0, 4.0], true), Some(30.0));
    assert_eq!(sum_x2(&[1.0, 2.0], &[3.0, 4.0], false), Some(-20.0));
    assert_eq!(sum_xmy2(&[1.0, 4.0], &[3.0, 1.0]), Some(13.0));
    assert_eq!(
      frequency_counts(vec![1.0, 2.0, 3.0, 4.0], &[2.0, 3.0]),
      Some(vec![2, 1, 1])
    );
    assert_eq!(trim_mean(vec![1.0, 2.0, 100.0, 3.0], 0.5), Some(2.5));
    assert_eq!(
      rank_value(vec![3.0, 1.0, 3.0, 2.0], 3.0, false, true),
      Some(1.5)
    );
    assert_eq!(deviation_sum_squares(&[1.0, 2.0, 3.0]), Some(2.0));
    assert!((skewness(&[1.0, 2.0, 4.0], false).unwrap() - 0.9352195295828235).abs() < 1.0e-15);
    assert!((kurtosis(&[1.0, 2.0, 3.0, 4.0]).unwrap() + 1.2).abs() < 1.0e-12);
    assert_eq!(
      covariance(&[(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)], false),
      Some(4.0 / 3.0)
    );
    assert_eq!(correlation(&[1.0, 2.0, 3.0], &[2.0, 4.0, 6.0]), Some(1.0));
    assert_eq!(
      percent_rank(vec![1.0, 2.0, 3.0, 4.0], 3.0, 3.0, PercentileKind::Inc),
      Some(0.667)
    );
  }
}
