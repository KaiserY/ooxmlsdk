use std::cell::RefCell;

use num_complex::Complex;
use regex::{Regex, RegexBuilder};
use rustc_hash::FxHashMap;

pub mod combinatorics;
pub mod complex;
pub mod datetime;
pub mod financial;
pub(crate) mod fourier;
pub mod matrix;
pub mod numeric;
pub mod query;
pub mod radix;
pub mod regression;
pub mod special;
pub mod statistics;
pub mod text;
pub mod units;

#[derive(Default)]
pub(crate) struct CalcEngine {
  fourier: RefCell<fourier::FourierEngine>,
  regex: RefCell<RegexCache>,
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

  pub(crate) fn regex(&self, pattern: &str, case_insensitive: bool) -> Result<Regex, ()> {
    self.regex.borrow_mut().regex(pattern, case_insensitive)
  }
}

#[derive(Default)]
struct RegexCache {
  case_sensitive: FxHashMap<String, Result<Regex, ()>>,
  case_insensitive: FxHashMap<String, Result<Regex, ()>>,
}

impl RegexCache {
  fn regex(&mut self, pattern: &str, case_insensitive: bool) -> Result<Regex, ()> {
    let entries = if case_insensitive {
      &mut self.case_insensitive
    } else {
      &mut self.case_sensitive
    };
    if let Some(cached) = entries.get(pattern) {
      return cached.clone();
    }
    if entries.len() >= 64 {
      entries.clear();
    }
    let compiled = RegexBuilder::new(pattern)
      .case_insensitive(case_insensitive)
      .build()
      .map_err(|_| ());
    entries.insert(pattern.to_string(), compiled.clone());
    compiled
  }
}
