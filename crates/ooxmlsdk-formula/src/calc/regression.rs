use crate::calc::matrix::{apply_householder, qr_decompose, solve_upper};
use crate::calc::numeric::{approx_sub, kahan_sum};

#[derive(Clone, Debug, PartialEq)]
pub struct RegressionModel {
  pub slopes: Vec<f64>,
  pub intercept: f64,
}

impl RegressionModel {
  pub fn predict(&self, row: &[f64]) -> f64 {
    self.intercept
      + row
        .iter()
        .zip(&self.slopes)
        .map(|(x, slope)| x * slope)
        .sum::<f64>()
  }
}

#[derive(Clone, Debug)]
pub struct RegressionState {
  pub centered_x: Vec<Vec<f64>>,
  pub centered_y: Vec<f64>,
  pub means: Vec<f64>,
  pub r_diagonal: Vec<f64>,
  pub model: RegressionModel,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RegressionScalarState {
  pub count: usize,
  pub x_mean: f64,
  pub y_mean: f64,
  pub xy_sum: f64,
  pub x_sum_sq: f64,
  pub y_sum_sq: f64,
}

impl RegressionScalarState {
  pub fn slope(self) -> Option<f64> {
    (self.x_sum_sq != 0.0).then_some(self.xy_sum / self.x_sum_sq)
  }

  pub fn intercept(self) -> Option<f64> {
    self.slope().map(|slope| self.y_mean - slope * self.x_mean)
  }

  pub fn forecast(self, x: f64) -> Option<f64> {
    self
      .slope()
      .map(|slope| self.y_mean + slope * (x - self.x_mean))
  }

  pub fn r_squared(self) -> Option<f64> {
    if self.x_sum_sq == 0.0 || self.y_sum_sq == 0.0 {
      return None;
    }
    let pearson = self.xy_sum / (self.x_sum_sq * self.y_sum_sq).sqrt();
    Some(pearson * pearson)
  }

  pub fn steyx(self) -> Option<f64> {
    if self.count < 3 || self.x_sum_sq == 0.0 {
      return None;
    }
    Some(
      ((self.y_sum_sq - self.xy_sum * self.xy_sum / self.x_sum_sq) / (self.count as f64 - 2.0))
        .sqrt(),
    )
  }
}

pub fn scalar_state(y_values: &[f64], x_values: &[f64]) -> RegressionScalarState {
  let count = y_values.len().min(x_values.len());
  if count < 2 {
    return RegressionScalarState {
      count,
      x_mean: 0.0,
      y_mean: 0.0,
      xy_sum: 0.0,
      x_sum_sq: 0.0,
      y_sum_sq: 0.0,
    };
  }
  let x_mean = kahan_sum(x_values.iter().take(count).copied()) / count as f64;
  let y_mean = kahan_sum(y_values.iter().take(count).copied()) / count as f64;
  let xy_sum = kahan_sum((0..count).map(|index| {
    let x_delta = x_values[index] - x_mean;
    let y_delta = y_values[index] - y_mean;
    x_delta * y_delta
  }));
  let x_sum_sq = kahan_sum((0..count).map(|index| {
    let x_delta = x_values[index] - x_mean;
    x_delta * x_delta
  }));
  let y_sum_sq = kahan_sum((0..count).map(|index| {
    let y_delta = y_values[index] - y_mean;
    y_delta * y_delta
  }));
  RegressionScalarState {
    count,
    x_mean,
    y_mean,
    xy_sum,
    x_sum_sq,
    y_sum_sq,
  }
}

pub fn regression_model(y: &[f64], design: &[Vec<f64>], constant: bool) -> Option<RegressionModel> {
  if design.first().map_or(0, Vec::len) == 1 {
    return simple_regression_model(y, design, constant);
  }
  regression_state(y, design, constant).map(|state| state.model)
}

pub fn simple_regression_model(
  y: &[f64],
  design: &[Vec<f64>],
  constant: bool,
) -> Option<RegressionModel> {
  let n = y.len();
  if (constant && n < 2) || n < 1 {
    return None;
  }
  let mut x = design
    .iter()
    .map(|row| row.first().copied())
    .collect::<Option<Vec<_>>>()?;
  let mut y = y.to_vec();
  let mut mean_y = 0.0;
  let mut mean_x = 0.0;
  if constant {
    mean_y = kahan_sum(y.iter().copied()) / n as f64;
    for value in &mut y {
      *value = approx_sub(*value, mean_y);
    }
    mean_x = kahan_sum(x.iter().copied()) / n as f64;
    for value in &mut x {
      *value = approx_sub(*value, mean_x);
    }
  }
  let sum_xy = kahan_sum(x.iter().zip(&y).map(|(x, y)| x * y));
  let sum_x2 = kahan_sum(x.iter().map(|value| value * value));
  if sum_x2 == 0.0 {
    return None;
  }
  let slope = sum_xy / sum_x2;
  let intercept = if constant {
    mean_y - slope * mean_x
  } else {
    0.0
  };
  Some(RegressionModel {
    slopes: vec![slope],
    intercept,
  })
}

pub fn regression_state(y: &[f64], design: &[Vec<f64>], constant: bool) -> Option<RegressionState> {
  let n = y.len();
  let k = design.first().map_or(0, Vec::len);
  if (constant && n < k + 1) || (!constant && n < k) || n < 1 || k < 1 {
    return None;
  }
  if design.len() != n || design.iter().any(|row| row.len() != k) {
    return None;
  }
  let mut centered_x = design.to_vec();
  let mut centered_y = y.to_vec();
  let mut means = vec![0.0; k];
  let mut mean_y = 0.0;
  if constant {
    mean_y = kahan_sum(centered_y.iter().copied()) / n as f64;
    for value in &mut centered_y {
      *value = approx_sub(*value, mean_y);
    }
    for column in 0..k {
      means[column] = kahan_sum(centered_x.iter().map(|row| row[column])) / n as f64;
      for row in &mut centered_x {
        row[column] = approx_sub(row[column], means[column]);
      }
    }
  }
  let mut qr = centered_x.clone();
  let r_diagonal = qr_decompose(&mut qr, k, n)?;
  if r_diagonal.contains(&0.0) {
    return None;
  }
  let mut z = centered_y.clone();
  for column in 0..k {
    apply_householder(&qr, column, &mut z, n);
  }
  let mut slopes = z.iter().take(k).copied().collect::<Vec<_>>();
  solve_upper(&qr, &r_diagonal, &mut slopes, k);
  let intercept = if constant {
    mean_y
      - means
        .iter()
        .zip(&slopes)
        .map(|(mean, slope)| mean * slope)
        .sum::<f64>()
  } else {
    0.0
  };
  Some(RegressionState {
    centered_x: qr,
    centered_y,
    means,
    r_diagonal,
    model: RegressionModel { slopes, intercept },
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn scalar_regression_matches_known_line() {
    let state = scalar_state(&[2.0, 4.0, 6.0], &[1.0, 2.0, 3.0]);
    assert_eq!(state.slope(), Some(2.0));
    assert_eq!(state.intercept(), Some(0.0));
    assert_eq!(state.forecast(4.0), Some(8.0));
    assert_eq!(state.r_squared(), Some(1.0));
    assert_eq!(state.steyx(), Some(0.0));
  }

  #[test]
  fn regression_model_handles_multiple_predictors() {
    let y = [3.0, 4.0, 6.0, 8.0];
    let design = vec![
      vec![1.0, 0.0],
      vec![0.0, 1.0],
      vec![1.0, 1.0],
      vec![2.0, 1.0],
    ];
    let model = regression_model(&y, &design, true).unwrap();
    assert!((model.predict(&[3.0, 2.0]) - 13.0).abs() < 1.0e-12);
  }
}
