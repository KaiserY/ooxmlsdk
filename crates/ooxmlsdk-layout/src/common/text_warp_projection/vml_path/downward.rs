//! Local IEEE-754 downward rounding for bounded legacy geometry arithmetic.
//! TwoSum and FMA residuals determine whether the nearest result lies above
//! the exact value. No process/thread rounding state, locks, or unsafe code.

use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub(super) struct Down(pub f64);

impl Neg for Down {
  type Output = Self;
  fn neg(self) -> Self {
    Self(-self.0)
  }
}

impl Down {
  pub fn min(self, other: Self) -> Self {
    Self(self.0.min(other.0))
  }
  pub fn abs(self) -> Self {
    Self(self.0.abs())
  }
  pub fn sqrt(self) -> Self {
    let result = self.0.sqrt();
    Self(if (-result).mul_add(result, self.0) < 0.0 {
      result.next_down()
    } else {
      result
    })
  }
}

impl Add for Down {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    let sum = self.0 + rhs.0;
    let b = sum - self.0;
    let error = (self.0 - (sum - b)) + (rhs.0 - b);
    Self(if error < 0.0 { sum.next_down() } else { sum })
  }
}

impl Sub for Down {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    self + Self(-rhs.0)
  }
}

impl Mul for Down {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self {
    let product = self.0 * rhs.0;
    Self(if self.0.mul_add(rhs.0, -product) < 0.0 {
      product.next_down()
    } else {
      product
    })
  }
}

impl Div for Down {
  type Output = Self;
  fn div(self, rhs: Self) -> Self {
    let quotient = self.0 / rhs.0;
    let remainder = (-quotient).mul_add(rhs.0, self.0);
    Self(
      if (rhs.0 > 0.0 && remainder < 0.0) || (rhs.0 < 0.0 && remainder > 0.0) {
        quotient.next_down()
      } else {
        quotient
      },
    )
  }
}
