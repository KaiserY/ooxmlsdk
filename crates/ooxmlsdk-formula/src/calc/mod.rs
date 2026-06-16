use std::cell::RefCell;

use num_complex::Complex;

pub(crate) mod fourier;

#[derive(Default)]
pub(crate) struct CalcEngine {
  fourier: RefCell<fourier::FourierEngine>,
}

impl CalcEngine {
  pub(crate) fn new() -> Self {
    Self::default()
  }

  pub(crate) fn fourier_values(
    &self,
    values: Vec<Complex<f64>>,
    real_input: bool,
    inverse: bool,
  ) -> Vec<Complex<f64>> {
    self
      .fourier
      .borrow_mut()
      .values(values, real_input, inverse)
  }
}
