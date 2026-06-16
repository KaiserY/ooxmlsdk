use statrs::distribution::{ContinuousCDF, Normal};
use statrs::function::gamma as statrs_gamma;

use crate::calc::numeric::{KahanSum, approx_floor, kahan_sum};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpecialError {
  IllegalArgument,
  Num,
  Div0,
}

pub fn gamma(value: f64) -> f64 {
  lo_gamma(value).unwrap_or_else(|_| statrs_gamma::gamma(value))
}

pub fn log_gamma(z: f64) -> f64 {
  lo_log_gamma(z)
}

const LO_MAX_GAMMA_ARGUMENT: f64 = 171.624376956302;
const LO_LANCZOS_G: f64 = 6.024_680_040_776_73;
const LO_HALF_MACH_EPS: f64 = 0.5 * f64::EPSILON;

fn lo_lanczos_sum(z: f64) -> f64 {
  const NUM: [f64; 13] = [
    23_531_376_880.410_76,
    42_919_803_642.649_1,
    35_711_959_237.355_67,
    17_921_034_426.037_21,
    6_039_542_586.352_028,
    1_439_720_407.311_721_6,
    248_874_557.862_054_17,
    31_426_415.585_400_194,
    2_876_370.628_935_372_5,
    186_056.265_395_223_48,
    8_071.672_002_365_816,
    210.824_277_751_579_36,
    2.506_628_274_631_000_2,
  ];
  const DENOM: [f64; 13] = [
    0.0,
    39916800.0,
    120543840.0,
    150917976.0,
    105258076.0,
    45995730.0,
    13339535.0,
    2637558.0,
    357423.0,
    32670.0,
    1925.0,
    66.0,
    1.0,
  ];
  if z <= 1.0 {
    let mut sum_num = NUM[12];
    let mut sum_denom = DENOM[12];
    for index in (0..12).rev() {
      sum_num = sum_num * z + NUM[index];
      sum_denom = sum_denom * z + DENOM[index];
    }
    sum_num / sum_denom
  } else {
    let z_inv = 1.0 / z;
    let mut sum_num = NUM[0];
    let mut sum_denom = DENOM[0];
    for index in 1..=12 {
      sum_num = sum_num * z_inv + NUM[index];
      sum_denom = sum_denom * z_inv + DENOM[index];
    }
    sum_num / sum_denom
  }
}

fn lo_gamma_helper(z: f64) -> f64 {
  let z_g = z + LO_LANCZOS_G - 0.5;
  let half_power = z_g.powf(z / 2.0 - 0.25);
  let mut result = lo_lanczos_sum(z);
  result *= half_power;
  result /= z_g.exp();
  result *= half_power;
  if z <= 20.0 && z == approx_floor(z) {
    result = result.round();
  }
  result
}

fn lo_log_gamma_helper(z: f64) -> f64 {
  let z_g = z + LO_LANCZOS_G - 0.5;
  lo_lanczos_sum(z).ln() + (z - 0.5) * z_g.ln() - z_g
}

fn lo_gamma(z: f64) -> std::result::Result<f64, SpecialError> {
  let log_pi = std::f64::consts::PI.ln();
  let log_dbl_max = f64::MAX.ln();
  if z > LO_MAX_GAMMA_ARGUMENT {
    return Err(SpecialError::IllegalArgument);
  }
  if z >= 1.0 {
    return Ok(lo_gamma_helper(z));
  }
  if z >= 0.5 {
    return Ok(lo_gamma_helper(z + 1.0) / z);
  }
  if z >= -0.5 {
    let log_test = lo_log_gamma_helper(z + 2.0) - z.ln_1p() - z.abs().ln();
    if log_test >= log_dbl_max {
      return Err(SpecialError::IllegalArgument);
    }
    return Ok(lo_gamma_helper(z + 2.0) / (z + 1.0) / z);
  }
  let sin_pi_z = (std::f64::consts::PI * z).sin();
  let log_divisor = lo_log_gamma_helper(1.0 - z) + sin_pi_z.abs().ln();
  if log_divisor - log_pi >= log_dbl_max {
    return Ok(0.0);
  }
  if log_divisor < 0.0 && log_pi - log_divisor > log_dbl_max {
    return Err(SpecialError::IllegalArgument);
  }
  Ok((log_pi - log_divisor).exp() * if sin_pi_z < 0.0 { -1.0 } else { 1.0 })
}

fn lo_log_gamma(z: f64) -> f64 {
  if z >= LO_MAX_GAMMA_ARGUMENT {
    return lo_log_gamma_helper(z);
  }
  if z >= 1.0 {
    return lo_gamma_helper(z).ln();
  }
  if z >= 0.5 {
    return (lo_gamma_helper(z + 1.0) / z).ln();
  }
  lo_log_gamma_helper(z + 2.0) - z.ln_1p() - z.ln()
}

fn lo_beta(alpha: f64, beta: f64) -> f64 {
  let (a, b) = if alpha > beta {
    (alpha, beta)
  } else {
    (beta, alpha)
  };
  if a + b < LO_MAX_GAMMA_ARGUMENT {
    return lo_gamma(a).unwrap_or(f64::INFINITY) / lo_gamma(a + b).unwrap_or(f64::INFINITY)
      * lo_gamma(b).unwrap_or(f64::INFINITY);
  }
  lo_log_beta(a, b).exp()
}

fn lo_log_beta(alpha: f64, beta: f64) -> f64 {
  let (a, b) = if alpha > beta {
    (alpha, beta)
  } else {
    (beta, alpha)
  };
  let g_minus_half = LO_LANCZOS_G - 0.5;
  let lanczos = lo_lanczos_sum(a) / lo_lanczos_sum(a + b) * lo_lanczos_sum(b);
  let ab_g = a + b + g_minus_half;
  let log_lanczos =
    lanczos.ln() + 0.5 * (ab_g.ln() - (a + g_minus_half).ln() - (b + g_minus_half).ln());
  -a * (b / (a + g_minus_half)).ln_1p() - b * (a / (b + g_minus_half)).ln_1p() - g_minus_half
    + log_lanczos
}

pub fn lo_beta_dist_pdf(x: f64, a: f64, b: f64) -> std::result::Result<f64, SpecialError> {
  if a == 1.0 {
    if b == 1.0 {
      return Ok(1.0);
    }
    if b == 2.0 {
      return Ok(-2.0 * x + 2.0);
    }
    if x == 1.0 && b < 1.0 {
      return Err(SpecialError::IllegalArgument);
    }
    return Ok(if x <= 0.01 {
      b + b * ((b - 1.0) * (-x).ln_1p()).exp_m1()
    } else {
      b * (0.5 - x + 0.5).powf(b - 1.0)
    });
  }
  if b == 1.0 {
    if a == 2.0 {
      return Ok(a * x);
    }
    if x == 0.0 && a < 1.0 {
      return Err(SpecialError::IllegalArgument);
    }
    return Ok(a * x.powf(a - 1.0));
  }
  if x <= 0.0 {
    if a < 1.0 && x == 0.0 {
      return Err(SpecialError::IllegalArgument);
    }
    return Ok(0.0);
  }
  if x >= 1.0 {
    if b < 1.0 && x == 1.0 {
      return Err(SpecialError::IllegalArgument);
    }
    return Ok(0.0);
  }

  let log_dbl_max = f64::MAX.ln();
  let log_dbl_min = f64::MIN_POSITIVE.ln();
  let log_y = if x < 0.1 {
    (-x).ln_1p()
  } else {
    (0.5 - x + 0.5).ln()
  };
  let log_x = x.ln();
  let a_log_x = (a - 1.0) * log_x;
  let b_log_y = (b - 1.0) * log_y;
  let log_beta = lo_log_beta(a, b);
  if a_log_x < log_dbl_max
    && a_log_x > log_dbl_min
    && b_log_y < log_dbl_max
    && b_log_y > log_dbl_min
    && log_beta < log_dbl_max
    && log_beta > log_dbl_min
    && a_log_x + b_log_y < log_dbl_max
    && a_log_x + b_log_y > log_dbl_min
  {
    Ok(x.powf(a - 1.0) * (0.5 - x + 0.5).powf(b - 1.0) / lo_beta(a, b))
  } else {
    Ok((a_log_x + b_log_y - log_beta).exp())
  }
}

fn lo_beta_helper_cont_frac(x: f64, a: f64, b: f64) -> f64 {
  let mut a1 = 1.0;
  let mut b1 = 1.0;
  let mut b2 = 1.0 - (a + b) / (a + 1.0) * x;
  let mut a2;
  let mut fnorm;
  let mut cf;
  if b2 == 0.0 {
    a2 = 0.0;
    fnorm = 1.0;
    cf = 1.0;
  } else {
    a2 = 1.0;
    fnorm = 1.0 / b2;
    cf = a2 * fnorm;
  }
  let mut cf_new = 1.0;
  let mut rm = 1.0;
  let mut finished = false;
  while rm < 50000.0 && !finished {
    let apl2m = a + 2.0 * rm;
    let d2m = rm * (b - rm) * x / ((apl2m - 1.0) * apl2m);
    let d2m1 = -(a + rm) * (a + b + rm) * x / (apl2m * (apl2m + 1.0));
    a1 = (a2 + d2m * a1) * fnorm;
    b1 = (b2 + d2m * b1) * fnorm;
    a2 = a1 + d2m1 * a2 * fnorm;
    b2 = b1 + d2m1 * b2 * fnorm;
    if b2 != 0.0 {
      fnorm = 1.0 / b2;
      cf_new = a2 * fnorm;
      finished = (cf - cf_new).abs() < cf.abs() * f64::EPSILON;
    }
    cf = cf_new;
    rm += 1.0;
  }
  cf
}

pub fn lo_beta_dist(x_in: f64, alpha: f64, beta: f64) -> f64 {
  if x_in <= 0.0 {
    return 0.0;
  }
  if x_in >= 1.0 {
    return 1.0;
  }
  if beta == 1.0 {
    return x_in.powf(alpha);
  }
  if alpha == 1.0 {
    return -(beta * (-x_in).ln_1p()).exp_m1();
  }
  let mut y = 0.5 - x_in + 0.5;
  let mut ln_y = (-x_in).ln_1p();
  let mut x = x_in;
  let mut ln_x = x_in.ln();
  let mut a = alpha;
  let mut b = beta;
  let reflect = x_in > alpha / (alpha + beta);
  if reflect {
    a = beta;
    b = alpha;
    x = y;
    y = x_in;
    ln_x = ln_y;
    ln_y = x_in.ln();
  }
  let mut result = lo_beta_helper_cont_frac(x, a, b) / a;
  let p = a / (a + b);
  let q = b / (a + b);
  let temp = if a > 1.0 && b > 1.0 && p < 0.97 && q < 0.97 {
    lo_beta_dist_pdf(x, a, b).unwrap_or(f64::INFINITY) * x * y
  } else {
    (a * ln_x + b * ln_y - lo_log_beta(a, b)).exp()
  };
  result *= temp;
  if reflect {
    result = 0.5 - result + 0.5;
  }
  result.clamp(0.0, 1.0)
}

pub fn lo_f_dist_right_tail(x: f64, df1: f64, df2: f64) -> f64 {
  lo_beta_dist(df2 / (df2 + df1 * x), df2 / 2.0, df1 / 2.0)
}

pub fn lo_f_dist_pdf(x: f64, df1: f64, df2: f64) -> f64 {
  (df1 / df2).powf(df1 / 2.0) * x.powf(df1 / 2.0 - 1.0)
    / ((1.0 + x * df1 / df2).powf((df1 + df2) / 2.0) * lo_beta(df1 / 2.0, df2 / 2.0))
}

pub fn lo_t_dist(t: f64, df: f64, kind: i32) -> f64 {
  let x = df / (df + t * t);
  match kind {
    1 => 0.5 * lo_beta_dist(x, df / 2.0, 0.5),
    2 => lo_beta_dist(x, df / 2.0, 0.5),
    3 => (1.0 + t * t / df).powf(-(df + 1.0) / 2.0) / (df.sqrt() * lo_beta(0.5, df / 2.0)),
    4 => {
      let result = 0.5 * lo_beta_dist(x, df / 2.0, 0.5);
      if t < 0.0 { result } else { 1.0 - result }
    }
    _ => f64::NAN,
  }
}

pub fn lo_poisson_dist(x: f64, lambda: f64, cumulative: bool) -> f64 {
  if !cumulative {
    if lambda > 712.0 {
      return (x * lambda.ln() - lambda - lo_log_gamma(x + 1.0)).exp();
    }
    let mut value = 1.0;
    for i in 0..(x as u32) {
      value *= lambda / f64::from(i + 1);
    }
    return value * (-lambda).exp();
  }
  if lambda > 712.0 {
    return lo_up_reg_igamma(x + 1.0, lambda).unwrap_or(f64::NAN);
  }
  if x >= 936.0 {
    return 1.0;
  }
  let mut summand = (-lambda).exp();
  let mut sum = KahanSum::default();
  sum.add(summand);
  for i in 1..=(x as u32) {
    summand = summand * lambda / f64::from(i);
    sum.add(summand);
  }
  sum.finish()
}

pub fn lo_binom_dist_pmf(x: f64, n: f64, p: f64) -> f64 {
  let q = (0.5 - p) + 0.5;
  let mut factor = q.powf(n);
  if factor <= f64::MIN_POSITIVE {
    factor = p.powf(n);
    if factor <= f64::MIN_POSITIVE {
      lo_beta_dist_pdf(p, x + 1.0, n - x + 1.0).unwrap_or(f64::INFINITY) / (n + 1.0)
    } else {
      for i in 0..((n - x) as u32) {
        factor *= (n - f64::from(i)) / f64::from(i + 1) * q / p;
      }
      factor
    }
  } else {
    for i in 0..(x as u32) {
      factor *= (n - f64::from(i)) / f64::from(i + 1) * p / q;
    }
    factor
  }
}

pub fn lo_binom_dist_range(n: f64, xs: f64, xe: f64, mut factor: f64, p: f64, q: f64) -> f64 {
  let xs = xs as u32;
  for i in 1..=xs {
    if factor <= 0.0 {
      break;
    }
    factor *= (n - f64::from(i) + 1.0) / f64::from(i) * p / q;
  }
  let mut values = vec![factor];
  for i in xs + 1..=xe as u32 {
    if factor <= 0.0 {
      break;
    }
    factor *= (n - f64::from(i) + 1.0) / f64::from(i) * p / q;
    values.push(factor);
  }
  kahan_sum(values).min(1.0)
}

fn lo_gamma_cont_fraction(a: f64, x: f64) -> std::result::Result<f64, SpecialError> {
  let big_inv = f64::EPSILON;
  let big = 1.0 / big_inv;
  let mut count = 0.0;
  let mut y = 1.0 - a;
  let mut denom = x + 2.0 - a;
  let mut pkm1 = x + 1.0;
  let mut pkm2 = 1.0;
  let mut qkm1 = denom * x;
  let mut qkm2 = x;
  let mut approx = pkm1 / qkm1;
  let mut finished = false;
  while !finished && count < 10000.0 {
    count += 1.0;
    y += 1.0;
    let num = y * count;
    denom += 2.0;
    let pk = pkm1 * denom - pkm2 * num;
    let qk = qkm1 * denom - qkm2 * num;
    if qk != 0.0 {
      let r = pk / qk;
      finished = ((approx - r) / r).abs() <= LO_HALF_MACH_EPS;
      approx = r;
    }
    pkm2 = pkm1;
    pkm1 = pk;
    qkm2 = qkm1;
    qkm1 = qk;
    if pk.abs() > big {
      pkm2 *= big_inv;
      pkm1 *= big_inv;
      qkm2 *= big_inv;
      qkm1 *= big_inv;
    }
  }
  finished.then_some(approx).ok_or(SpecialError::Num)
}

fn lo_gamma_series(a: f64, x: f64) -> std::result::Result<f64, SpecialError> {
  let mut denom_factor = a;
  let mut summand = 1.0 / a;
  let mut sum = summand;
  let mut count = 1;
  while summand / sum > LO_HALF_MACH_EPS && count <= 10000 {
    denom_factor += 1.0;
    summand = summand * x / denom_factor;
    sum += summand;
    count += 1;
  }
  (count <= 10000).then_some(sum).ok_or(SpecialError::Num)
}

pub fn lo_low_reg_igamma(a: f64, x: f64) -> std::result::Result<f64, SpecialError> {
  let factor = (a * x.ln() - x - lo_log_gamma(a)).exp();
  if x > a + 1.0 {
    Ok(1.0 - factor * lo_gamma_cont_fraction(a, x)?)
  } else {
    Ok(factor * lo_gamma_series(a, x)?)
  }
}

pub fn lo_up_reg_igamma(a: f64, x: f64) -> std::result::Result<f64, SpecialError> {
  let factor = (a * x.ln() - x - lo_log_gamma(a)).exp();
  if x > a + 1.0 {
    Ok(factor * lo_gamma_cont_fraction(a, x)?)
  } else {
    Ok(1.0 - factor * lo_gamma_series(a, x)?)
  }
}

pub fn lo_gamma_dist_pdf(
  x: f64,
  alpha: f64,
  lambda: f64,
) -> std::result::Result<f64, SpecialError> {
  if x < 0.0 {
    return Ok(0.0);
  }
  if x == 0.0 {
    if alpha < 1.0 {
      return Err(SpecialError::Div0);
    }
    if alpha == 1.0 {
      return Ok(1.0 / lambda);
    }
    return Ok(0.0);
  }
  let xr = x / lambda;
  if xr > 1.0 {
    let log_dbl_max = f64::MAX.ln();
    if xr.ln() * (alpha - 1.0) < log_dbl_max && alpha < LO_MAX_GAMMA_ARGUMENT {
      Ok(xr.powf(alpha - 1.0) * (-xr).exp() / lambda / lo_gamma(alpha)?)
    } else {
      Ok(((alpha - 1.0) * xr.ln() - xr - lambda.ln() - lo_log_gamma(alpha)).exp())
    }
  } else if alpha < LO_MAX_GAMMA_ARGUMENT {
    Ok(xr.powf(alpha - 1.0) * (-xr).exp() / lambda / lo_gamma(alpha)?)
  } else {
    Ok(xr.powf(alpha - 1.0) * (-xr).exp() / lambda / lo_log_gamma(alpha).exp())
  }
}

pub fn lo_gamma_dist(x: f64, alpha: f64, lambda: f64) -> f64 {
  if x <= 0.0 {
    0.0
  } else {
    lo_low_reg_igamma(alpha, x / lambda).unwrap_or(f64::NAN)
  }
}

pub fn lo_chi_dist(x: f64, df: f64) -> f64 {
  if x <= 0.0 {
    1.0
  } else {
    lo_up_reg_igamma(df / 2.0, x / 2.0).unwrap_or(f64::NAN)
  }
}

pub fn lo_chisq_dist_cdf(x: f64, df: f64) -> f64 {
  if x <= 0.0 {
    0.0
  } else {
    lo_low_reg_igamma(df / 2.0, x / 2.0).unwrap_or(f64::NAN)
  }
}

pub fn lo_chisq_dist_pdf(x: f64, df: f64) -> f64 {
  if x <= 0.0 {
    return 0.0;
  }
  if df * x > 1_391_000.0 {
    return (((0.5 * df - 1.0) * (x * 0.5).ln()) - 0.5 * x - 2.0_f64.ln() - lo_log_gamma(0.5 * df))
      .exp();
  }
  let mut value;
  let mut count;
  if df % 2.0 < 0.5 {
    value = 0.5;
    count = 2.0;
  } else {
    value = 1.0 / (x * 2.0 * std::f64::consts::PI).sqrt();
    count = 1.0;
  }
  while count < df {
    value *= x / count;
    count += 2.0;
  }
  if x >= 1425.0 {
    (value.ln() - x / 2.0).exp()
  } else {
    value * (-x / 2.0).exp()
  }
}

fn lo_has_change_of_sign(left: f64, right: f64) -> bool {
  (left < 0.0 && right > 0.0) || (left > 0.0 && right < 0.0)
}

pub fn lo_iterate_inverse<F>(
  function: F,
  mut ax: f64,
  mut bx: f64,
) -> std::result::Result<f64, SpecialError>
where
  F: Fn(f64) -> f64,
{
  let y_eps = 1.0e-307;
  let x_eps = f64::EPSILON;
  let mut ay = function(ax);
  let mut by = function(bx);
  for _ in 0..1000 {
    if lo_has_change_of_sign(ay, by) {
      break;
    }
    if ay.abs() <= by.abs() {
      let previous = ax;
      ax += (ax - bx) * 2.0;
      if ax < 0.0 {
        ax = 0.0;
      }
      bx = previous;
      by = ay;
      ay = function(ax);
    } else {
      let previous = bx;
      bx += (bx - ax) * 2.0;
      ax = previous;
      ay = by;
      by = function(bx);
    }
  }
  if ay == 0.0 {
    return Ok(ax);
  }
  if by == 0.0 {
    return Ok(bx);
  }
  if !lo_has_change_of_sign(ay, by) {
    return Err(SpecialError::Num);
  }

  let mut px = ax;
  let mut py = ay;
  let mut qx = bx;
  let mut qy = by;
  let mut rx = ax;
  let mut ry = ay;
  let mut sx = 0.5 * (ax + bx);
  let mut interpolate = true;
  let mut count = 0;
  while count < 500 && ry.abs() > y_eps && (bx - ax) > ax.abs().max(bx.abs()) * x_eps {
    if interpolate {
      if py != qy && qy != ry && ry != py {
        sx = px * ry * qy / (ry - py) / (qy - py)
          + rx * qy * py / (qy - ry) / (py - ry)
          + qx * py * ry / (py - qy) / (ry - qy);
        interpolate = ax < sx && sx < bx;
      } else {
        interpolate = false;
      }
    }
    if !interpolate {
      sx = 0.5 * (ax + bx);
      qx = bx;
      qy = by;
      interpolate = true;
    }
    px = qx;
    qx = rx;
    rx = sx;
    py = qy;
    qy = ry;
    ry = function(sx);
    if lo_has_change_of_sign(ay, ry) {
      bx = rx;
      by = ry;
    } else {
      ax = rx;
      ay = ry;
    }
    interpolate = interpolate && ry.abs() * 2.0 <= qy.abs();
    count += 1;
  }
  Ok(rx)
}

pub fn norm_s_dist(x: f64) -> f64 {
  lo_integral_phi(x)
}

pub fn norm_s_inv(p: f64) -> f64 {
  if p.is_nan() {
    return f64::NAN;
  }
  if p <= 0.0 {
    return f64::NEG_INFINITY;
  }
  if p >= 1.0 {
    return f64::INFINITY;
  }
  Normal::standard().inverse_cdf(p)
}

pub fn lo_phi(x: f64) -> f64 {
  0.39894228040143268 * (-(x * x) / 2.0).exp()
}

pub fn lo_integral_phi(x: f64) -> f64 {
  0.5 * erfc(-x * std::f64::consts::FRAC_1_SQRT_2)
}

fn lo_taylor(polynomial: &[f64], x: f64) -> f64 {
  let Some((&last, terms)) = polynomial.split_last() else {
    return 0.0;
  };
  let mut value = last;
  for coefficient in terms.iter().rev() {
    value = value * x + *coefficient;
  }
  value
}

pub fn lo_gauss(x: f64) -> f64 {
  let x_abs = x.abs();
  let x_short = approx_floor(x_abs) as u16;
  let value = if x_short == 0 {
    const T0: [f64; 12] = [
      0.39894228040143268,
      -0.06649038006690545,
      0.00997355701003582,
      -0.00118732821548045,
      0.00011543468761616,
      -0.00000944465625950,
      0.00000066596935163,
      -0.00000004122667415,
      0.00000000227352982,
      0.00000000011301172,
      0.00000000000511243,
      -0.00000000000021218,
    ];
    lo_taylor(&T0, x_abs * x_abs) * x_abs
  } else if x_short <= 2 {
    const T2: [f64; 24] = [
      0.47724986805182079,
      0.05399096651318805,
      -0.05399096651318805,
      0.02699548325659403,
      -0.00449924720943234,
      -0.00224962360471617,
      0.00134977416282970,
      -0.00011783742691370,
      -0.00011515930357476,
      0.00003704737285544,
      0.00000282690796889,
      -0.00000354513195524,
      0.00000037669563126,
      0.00000019202407921,
      -0.00000005226908590,
      -0.00000000491799345,
      0.00000000366377919,
      -0.00000000015981997,
      -0.00000000017381238,
      0.00000000002624031,
      0.00000000000560919,
      -0.00000000000172127,
      -0.00000000000008634,
      0.00000000000007894,
    ];
    lo_taylor(&T2, x_abs - 2.0)
  } else if x_short <= 4 {
    const T4: [f64; 21] = [
      0.49996832875816688,
      0.00013383022576489,
      -0.00026766045152977,
      0.00033457556441221,
      -0.00028996548915725,
      0.00018178605666397,
      -0.00008252863922168,
      0.00002551802519049,
      -0.00000391665839292,
      -0.00000074018205222,
      0.00000064422023359,
      -0.00000017370155340,
      0.00000000909595465,
      0.00000000944943118,
      -0.00000000329957075,
      0.00000000029492075,
      0.00000000011874477,
      -0.00000000004420396,
      0.00000000000361422,
      0.00000000000143638,
      -0.00000000000045848,
    ];
    lo_taylor(&T4, x_abs - 4.0)
  } else {
    const ASYMPT: [f64; 5] = [-1.0, 1.0, -3.0, 15.0, -105.0];
    0.5 + lo_phi(x_abs) * lo_taylor(&ASYMPT, 1.0 / (x_abs * x_abs)) / x_abs
  };
  if x < 0.0 { -value } else { value }
}

pub fn erf(x: f64) -> f64 {
  libm::erf(x)
}

pub fn erfc(x: f64) -> f64 {
  libm::erfc(x)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn gamma_and_distribution_helpers_match_known_values() {
    assert_eq!(gamma(5.0), 24.0);
    assert_eq!(log_gamma(5.0), 24.0_f64.ln());
    assert!((lo_beta_dist(0.5, 2.0, 2.0) - 0.5).abs() < 1.0e-12);
    assert!((lo_f_dist_right_tail(1.0, 5.0, 5.0) - 0.5).abs() < 1.0e-12);
    assert!((lo_poisson_dist(2.0, 1.0, false) - 0.18393972058572117).abs() < 1.0e-15);
  }

  #[test]
  fn inverse_iteration_reports_unbracketed_roots() {
    assert_eq!(
      lo_iterate_inverse(|_| 1.0, 0.0, 1.0),
      Err(SpecialError::Num)
    );
  }
}
