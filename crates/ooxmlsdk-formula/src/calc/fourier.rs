use std::sync::Arc;

use num_complex::Complex;
use rustfft::{Fft, FftPlanner};

pub(crate) struct FourierEngine {
  planner: FftPlanner<f64>,
}

impl Default for FourierEngine {
  fn default() -> Self {
    Self {
      planner: FftPlanner::new(),
    }
  }
}

impl FourierEngine {
  pub(crate) fn values(
    &mut self,
    mut values: Vec<Complex<f64>>,
    real_input: bool,
    inverse: bool,
  ) -> Vec<Complex<f64>> {
    if real_input && values.len() > 1 && values.len() % 2 == 0 {
      return self.even_real_values(&values, inverse);
    }
    self.planned_fft(values.len(), inverse).process(&mut values);
    values
  }

  fn planned_fft(&mut self, len: usize, inverse: bool) -> Arc<dyn Fft<f64>> {
    if inverse {
      self.planner.plan_fft_inverse(len)
    } else {
      self.planner.plan_fft_forward(len)
    }
  }

  fn even_real_values(&mut self, input: &[Complex<f64>], inverse: bool) -> Vec<Complex<f64>> {
    let point_count = input.len();
    let half_count = point_count / 2;
    let mut work = Vec::with_capacity(half_count);
    for index in 0..half_count {
      work.push(Complex::new(input[index * 2].re, input[index * 2 + 1].re));
    }

    self.planned_fft(half_count, inverse).process(&mut work);

    let mut output = vec![Complex::new(0.0, 0.0); point_count];
    let twiddle_sign = if inverse { 2.0 } else { -2.0 };
    for index in 0..half_count {
      let reverse_index = if index == 0 { 0 } else { half_count - index };
      let y1 = work[index];
      let y2 = work[reverse_index];
      let angle = twiddle_sign * std::f64::consts::PI * index as f64 / point_count as f64;
      let twiddle_real = angle.cos();
      let twiddle_imaginary = angle.sin();
      let result_real = 0.5
        * (y1.re + y2.re + twiddle_real * (y1.im + y2.im) + twiddle_imaginary * (y1.re - y2.re));
      let result_imaginary = 0.5
        * (y1.im - y2.im + twiddle_imaginary * (y1.im + y2.im) - twiddle_real * (y1.re - y2.re));
      output[index] = Complex::new(result_real, result_imaginary);
      if index == 0 {
        output[half_count] = Complex::new(y1.re - y1.im, 0.0);
      } else {
        output[half_count + reverse_index] = Complex::new(result_real, -result_imaginary);
      }
    }
    output
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn fourier_even_real_reconstructs_full_spectrum() {
    let values = [1.0, 2.0, 3.0, 4.0]
      .into_iter()
      .map(|value| Complex::new(value, 0.0))
      .collect();
    let mut engine = FourierEngine::default();
    let actual = engine.values(values, true, false);
    assert_complex_close(&actual[0], 10.0, 0.0);
    assert_complex_close(&actual[1], -2.0, 2.0);
    assert_complex_close(&actual[2], -2.0, 0.0);
    assert_complex_close(&actual[3], -2.0, -2.0);
  }

  fn assert_complex_close(actual: &Complex<f64>, expected_re: f64, expected_im: f64) {
    assert!(
      (actual.re - expected_re).abs() <= 1.0e-9,
      "expected real {expected_re}, got {}",
      actual.re
    );
    assert!(
      (actual.im - expected_im).abs() <= 1.0e-9,
      "expected imaginary {expected_im}, got {}",
      actual.im
    );
  }
}
