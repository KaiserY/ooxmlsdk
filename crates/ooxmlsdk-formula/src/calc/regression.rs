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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EtsKind {
  Add,
  Mult,
  Season,
  StatAdd,
  StatMult,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EtsError {
  IllegalArgument,
  Num,
  Value,
  Div0,
}

#[derive(Clone, Copy, Debug)]
struct EtsDataPoint {
  x: f64,
  y: f64,
}

#[derive(Clone, Debug)]
pub struct EtsCalculation {
  data: Vec<EtsDataPoint>,
  base: Vec<f64>,
  trend: Vec<f64>,
  period_index: Vec<f64>,
  forecast_values: Vec<f64>,
  pub samples_in_period: usize,
  step_size: f64,
  alpha: f64,
  beta: f64,
  gamma: f64,
  mae: f64,
  mase: f64,
  mse: f64,
  rmse: f64,
  smape: f64,
  additive: bool,
  double_smoothing: bool,
  initialized: bool,
}

impl EtsCalculation {
  const MIN_RESOLUTION: f64 = 0.001;

  pub fn new(
    timeline: &[f64],
    values: &[f64],
    samples_in_period: usize,
    data_completion: bool,
    aggregation: i32,
    target: Option<f64>,
    kind: EtsKind,
  ) -> Result<Self, EtsError> {
    let mut data = timeline
      .iter()
      .zip(values)
      .map(|(x, y)| EtsDataPoint { x: *x, y: *y })
      .collect::<Vec<_>>();
    data.sort_by(|left, right| left.x.total_cmp(&right.x));
    if let Some(target) = target
      && target < data.first().map_or(0.0, |point| point.x)
    {
      return Err(EtsError::Num);
    }

    let mut index = 1usize;
    let mut step_size = f64::MAX;
    while index < data.len() {
      let mut step = data[index].x - data[index - 1].x;
      if step == 0.0 {
        aggregate_duplicate_ets_points(&mut data, index, aggregation)?;
        if index < data.len() {
          step = data[index].x - data[index - 1].x;
        } else {
          step = step_size;
        }
      }
      if step > 0.0 && step < step_size {
        step_size = step;
      }
      index += 1;
    }
    if data.len() < 2 || !step_size.is_finite() || step_size == f64::MAX {
      return Err(EtsError::Value);
    }
    let mut has_gap = false;
    for index in 1..data.len() {
      let step = data[index].x - data[index - 1].x;
      if step != step_size {
        if (step % step_size).abs() >= f64::EPSILON {
          return Err(EtsError::Value);
        }
        has_gap = true;
      }
    }
    if has_gap {
      fill_ets_gaps(&mut data, step_size, data_completion)?;
    }

    let mut samples_in_period = if samples_in_period != 1 {
      samples_in_period
    } else {
      ets_period_len(&data)
    };
    let double_smoothing = samples_in_period == 0 || samples_in_period == 1;
    if double_smoothing {
      samples_in_period = 0;
    }
    let additive = matches!(kind, EtsKind::Add | EtsKind::Season | EtsKind::StatAdd);
    let mut calculation = Self {
      base: vec![0.0; data.len()],
      trend: vec![0.0; data.len()],
      period_index: vec![0.0; data.len()],
      forecast_values: vec![0.0; data.len()],
      samples_in_period,
      step_size,
      alpha: 0.0,
      beta: 0.0,
      gamma: 0.0,
      mae: 0.0,
      mase: 0.0,
      mse: 0.0,
      rmse: 0.0,
      smape: 0.0,
      additive,
      double_smoothing,
      initialized: false,
      data,
    };
    calculation.init_data()?;
    Ok(calculation)
  }

  fn init_data(&mut self) -> Result<(), EtsError> {
    self.forecast_values[0] = self.data[0].y;
    self.prefill_trend()?;
    self.prefill_period_index()?;
    self.base[0] = if self.double_smoothing {
      self.data[0].y
    } else {
      self.data[0].y / self.period_index[0]
    };
    Ok(())
  }

  fn prefill_trend(&mut self) -> Result<(), EtsError> {
    if self.double_smoothing {
      self.trend[0] = (self.data.last().unwrap().y - self.data[0].y) / (self.data.len() - 1) as f64;
      return Ok(());
    }
    if self.data.len() < 2 * self.samples_in_period {
      return Err(EtsError::Value);
    }
    let sum = kahan_sum(
      (0..self.samples_in_period)
        .map(|index| self.data[index + self.samples_in_period].y - self.data[index].y),
    );
    self.trend[0] = sum / (self.samples_in_period * self.samples_in_period) as f64;
    Ok(())
  }

  fn prefill_period_index(&mut self) -> Result<(), EtsError> {
    if self.double_smoothing {
      return Ok(());
    }
    let periods = self.data.len() / self.samples_in_period;
    let mut period_average = vec![0.0; periods];
    for (period, average) in period_average.iter_mut().enumerate() {
      let start = period * self.samples_in_period;
      *average = kahan_sum(
        self.data[start..start + self.samples_in_period]
          .iter()
          .map(|point| point.y),
      ) / self.samples_in_period as f64;
      if *average == 0.0 {
        return Err(EtsError::Div0);
      }
    }
    for sample in 0..self.samples_in_period {
      let mut total = 0.0;
      for (period, average) in period_average.iter().enumerate() {
        let trend_adjust =
          (sample as f64 - 0.5 * (self.samples_in_period - 1) as f64) * self.trend[0];
        let y = self.data[period * self.samples_in_period + sample].y;
        total += if self.additive {
          y - (*average + trend_adjust)
        } else {
          y / (*average + trend_adjust)
        };
      }
      self.period_index[sample] = total / periods as f64;
    }
    if self.samples_in_period < self.data.len() {
      self.period_index[self.samples_in_period] = 0.0;
    }
    Ok(())
  }

  fn initialize(&mut self) {
    if self.initialized {
      return;
    }
    self.calc_alpha_beta_gamma();
    self.initialized = true;
    self.calc_accuracy();
  }

  fn calc_alpha_beta_gamma(&mut self) {
    self.alpha = 0.0;
    if self.double_smoothing {
      self.beta = 0.0;
      self.calc_gamma();
    } else {
      self.calc_beta_gamma();
    }
    self.refill();
    let mut low_error = self.mse;
    self.alpha = 1.0;
    if self.double_smoothing {
      self.calc_gamma();
    } else {
      self.calc_beta_gamma();
    }
    self.refill();
    let mut high_error = self.mse;
    self.alpha = 0.5;
    if self.double_smoothing {
      self.calc_gamma();
    } else {
      self.calc_beta_gamma();
    }
    self.refill();
    if low_error == self.mse && self.mse == high_error {
      self.alpha = 0.0;
      if self.double_smoothing {
        self.calc_gamma();
      } else {
        self.calc_beta_gamma();
      }
      self.refill();
      return;
    }
    let mut low = 0.0;
    let mut mid = 0.5;
    let mut high = 1.0;
    while high - mid > Self::MIN_RESOLUTION {
      if high_error > low_error {
        high = mid;
        high_error = self.mse;
        mid = (low + mid) / 2.0;
      } else {
        low = mid;
        low_error = self.mse;
        mid = (mid + high) / 2.0;
      }
      self.alpha = mid;
      if self.double_smoothing {
        self.calc_gamma();
      } else {
        self.calc_beta_gamma();
      }
      self.refill();
    }
    if high_error > low_error {
      if low_error < self.mse {
        self.alpha = low;
        if self.double_smoothing {
          self.calc_gamma();
        } else {
          self.calc_beta_gamma();
        }
        self.refill();
      }
    } else if high_error < self.mse {
      self.alpha = high;
      if self.double_smoothing {
        self.calc_gamma();
      } else {
        self.calc_beta_gamma();
      }
      self.refill();
    }
    self.calc_accuracy();
  }

  fn calc_beta_gamma(&mut self) {
    self.beta = 0.0;
    self.calc_gamma();
    self.refill();
    let mut low_error = self.mse;
    self.beta = 1.0;
    self.calc_gamma();
    self.refill();
    let mut high_error = self.mse;
    self.beta = 0.5;
    self.calc_gamma();
    self.refill();
    if low_error == self.mse && self.mse == high_error {
      self.beta = 0.0;
      self.calc_gamma();
      self.refill();
      return;
    }
    let mut low = 0.0;
    let mut mid = 0.5;
    let mut high = 1.0;
    while high - mid > Self::MIN_RESOLUTION {
      if high_error > low_error {
        high = mid;
        high_error = self.mse;
        mid = (low + mid) / 2.0;
      } else {
        low = mid;
        low_error = self.mse;
        mid = (mid + high) / 2.0;
      }
      self.beta = mid;
      self.calc_gamma();
      self.refill();
    }
    if high_error > low_error {
      if low_error < self.mse {
        self.beta = low;
        self.calc_gamma();
        self.refill();
      }
    } else if high_error < self.mse {
      self.beta = high;
      self.calc_gamma();
      self.refill();
    }
  }

  fn calc_gamma(&mut self) {
    self.gamma = 0.0;
    self.refill();
    let mut low_error = self.mse;
    self.gamma = 1.0;
    self.refill();
    let mut high_error = self.mse;
    self.gamma = 0.5;
    self.refill();
    if low_error == self.mse && self.mse == high_error {
      self.gamma = 0.0;
      self.refill();
      return;
    }
    let mut low = 0.0;
    let mut mid = 0.5;
    let mut high = 1.0;
    while high - mid > Self::MIN_RESOLUTION {
      if high_error > low_error {
        high = mid;
        high_error = self.mse;
        mid = (low + mid) / 2.0;
      } else {
        low = mid;
        low_error = self.mse;
        mid = (mid + high) / 2.0;
      }
      self.gamma = mid;
      self.refill();
    }
    if high_error > low_error {
      if low_error < self.mse {
        self.gamma = low;
        self.refill();
      }
    } else if high_error < self.mse {
      self.gamma = high;
      self.refill();
    }
  }

  fn refill(&mut self) {
    for index in 1..self.data.len() {
      if self.double_smoothing {
        self.base[index] = self.alpha * self.data[index].y
          + (1.0 - self.alpha) * (self.base[index - 1] + self.trend[index - 1]);
        self.trend[index] = self.gamma * (self.base[index] - self.base[index - 1])
          + (1.0 - self.gamma) * self.trend[index - 1];
        self.forecast_values[index] = self.base[index - 1] + self.trend[index - 1];
      } else {
        let period_index = if self.additive {
          if index > self.samples_in_period {
            index - self.samples_in_period
          } else {
            index
          }
        } else if index >= self.samples_in_period {
          index - self.samples_in_period
        } else {
          index
        };
        if self.additive {
          self.base[index] = self.alpha * (self.data[index].y - self.period_index[period_index])
            + (1.0 - self.alpha) * (self.base[index - 1] + self.trend[index - 1]);
          self.period_index[index] = self.beta * (self.data[index].y - self.base[index])
            + (1.0 - self.beta) * self.period_index[period_index];
        } else {
          self.base[index] = self.alpha * (self.data[index].y / self.period_index[period_index])
            + (1.0 - self.alpha) * (self.base[index - 1] + self.trend[index - 1]);
          self.period_index[index] = self.beta * (self.data[index].y / self.base[index])
            + (1.0 - self.beta) * self.period_index[period_index];
        }
        self.trend[index] = self.gamma * (self.base[index] - self.base[index - 1])
          + (1.0 - self.gamma) * self.trend[index - 1];
        self.forecast_values[index] = if self.additive {
          self.base[index - 1] + self.trend[index - 1] + self.period_index[period_index]
        } else {
          (self.base[index - 1] + self.trend[index - 1]) * self.period_index[period_index]
        };
      }
    }
    self.calc_accuracy();
  }

  fn calc_accuracy(&mut self) {
    let mut sum_abs_error = 0.0;
    let mut sum_divisor = 0.0;
    let mut sum_error_sq = 0.0;
    let mut sum_abs_percent_error = 0.0;
    for index in 1..self.data.len() {
      let error = self.forecast_values[index] - self.data[index].y;
      sum_abs_error += error.abs();
      sum_error_sq += error * error;
      sum_abs_percent_error +=
        error.abs() / (self.forecast_values[index].abs() + self.data[index].y.abs());
    }
    for index in 2..self.data.len() {
      sum_divisor += (self.data[index].y - self.data[index - 1].y).abs();
    }
    let count = (self.data.len() - 1) as f64;
    self.mae = sum_abs_error / count;
    self.mase = if sum_divisor == 0.0 {
      0.0
    } else {
      sum_abs_error / (count * sum_divisor / (count - 1.0))
    };
    self.mse = sum_error_sq / count;
    self.rmse = self.mse.sqrt();
    self.smape = sum_abs_percent_error * 2.0 / count;
  }

  pub fn forecast(&mut self, target: f64) -> f64 {
    self.initialize();
    let last = self.data.len() - 1;
    if target <= self.data[last].x {
      let index = ((target - self.data[0].x) / self.step_size) as usize;
      let interpolate = (target - self.data[0].x) % self.step_size;
      let mut result = self.data[index].y;
      if interpolate >= Self::MIN_RESOLUTION && index + 1 < self.forecast_values.len() {
        let factor = interpolate / self.step_size;
        result += factor * (self.forecast_values[index + 1] - result);
      }
      return result;
    }
    let steps = ((target - self.data[last].x) / self.step_size) as usize;
    let interpolate = (target - self.data[last].x) % self.step_size;
    let mut result = self.forecast_future(steps);
    if interpolate >= Self::MIN_RESOLUTION {
      let next = self.forecast_future(steps + 1);
      result += interpolate / self.step_size * (next - result);
    }
    result
  }

  fn forecast_future(&self, steps: usize) -> f64 {
    let last = self.data.len() - 1;
    if self.double_smoothing {
      self.base[last] + steps as f64 * self.trend[last]
    } else {
      let index = last - self.samples_in_period + (steps % self.samples_in_period);
      if self.additive {
        self.base[last] + steps as f64 * self.trend[last] + self.period_index[index]
      } else {
        (self.base[last] + steps as f64 * self.trend[last]) * self.period_index[index]
      }
    }
  }

  pub fn statistic(&mut self, index: i32) -> f64 {
    self.initialize();
    match index {
      1 => self.alpha,
      2 => self.gamma,
      3 => self.beta,
      4 => self.mase,
      5 => self.smape,
      6 => self.mae,
      7 => self.rmse,
      8 => self.step_size,
      9 => self.samples_in_period as f64,
      _ => f64::NAN,
    }
  }
}

fn aggregate_duplicate_ets_points(
  data: &mut Vec<EtsDataPoint>,
  index: usize,
  aggregation: i32,
) -> Result<(), EtsError> {
  let x = data[index - 1].x;
  let mut values = vec![data[index - 1].y];
  while index < data.len() && data[index].x == x {
    values.push(data.remove(index).y);
  }
  data[index - 1].y = match aggregation {
    1 => values[0],
    2 | 3 => values.len() as f64,
    4 => values.into_iter().reduce(f64::max).ok_or(EtsError::Value)?,
    5 => {
      values.sort_by(f64::total_cmp);
      if values.len() % 2 == 1 {
        values[values.len() / 2]
      } else {
        (values[values.len() / 2] + values[values.len() / 2 - 1]) / 2.0
      }
    }
    6 => values.into_iter().reduce(f64::min).ok_or(EtsError::Value)?,
    7 => kahan_sum(values.iter().copied()),
    _ => return Err(EtsError::IllegalArgument),
  };
  Ok(())
}

fn fill_ets_gaps(
  data: &mut Vec<EtsDataPoint>,
  step_size: f64,
  data_completion: bool,
) -> Result<(), EtsError> {
  let original_count = data.len() as f64;
  let mut missing_count = 0usize;
  let mut index = 1usize;
  while index < data.len() {
    let distance = data[index].x - data[index - 1].x;
    if distance > step_size {
      let y = if data_completion {
        (data[index].y + data[index - 1].y) / 2.0
      } else {
        0.0
      };
      let mut x = data[index - 1].x + step_size;
      while x < data[index].x {
        data.insert(index, EtsDataPoint { x, y });
        missing_count += 1;
        if missing_count as f64 / original_count > 0.3 {
          return Err(EtsError::Value);
        }
        index += 1;
        x += step_size;
      }
    }
    index += 1;
  }
  Ok(())
}

fn ets_period_len(data: &[EtsDataPoint]) -> usize {
  let mut best = data.len();
  let mut best_error = f64::MAX;
  for period_len in (1..=data.len() / 2).rev() {
    let periods = data.len() / period_len;
    let start = data.len() - (periods * period_len) + 1;
    let mut mean_error = 0.0;
    for index in start..(data.len() - period_len) {
      mean_error += ((data[index].y - data[index - 1].y)
        - (data[period_len + index].y - data[period_len + index - 1].y))
        .abs();
    }
    mean_error /= ((periods - 1) * period_len - 1) as f64;
    if mean_error <= best_error || mean_error == 0.0 {
      best = period_len;
      best_error = mean_error;
    }
  }
  best
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

  #[test]
  fn ets_calculation_forecasts_additive_series() {
    let timeline = [1.0, 2.0, 3.0, 4.0, 5.0];
    let values = [2.0, 4.0, 7.0, 8.0, 11.0];
    let mut calculation =
      EtsCalculation::new(&timeline, &values, 1, true, 1, Some(6.0), EtsKind::Add).unwrap();

    assert!((calculation.forecast(6.0) - 12.5).abs() < 1.0e-12);
    assert_eq!(calculation.statistic(8), 1.0);
  }
}
