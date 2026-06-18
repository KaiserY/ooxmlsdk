use super::datetime::{date_from_serial, date_serial, is_leap_year, last_day_of_month, yearfrac};
use super::numeric::{approx_ceil, approx_equal, approx_floor};

pub fn financial_pmt(rate: f64, nper: f64, pv: f64, fv: f64, pay_in_advance: bool) -> f64 {
  let payment = if rate == 0.0 {
    (pv + fv) / nper
  } else {
    let log_rate = rate.ln_1p();
    if pay_in_advance {
      (fv + pv * (nper * log_rate).exp()) * rate / (((nper + 1.0) * log_rate).exp_m1() - rate)
    } else {
      (fv + pv * (nper * log_rate).exp()) * rate / (nper * log_rate).exp_m1()
    }
  };
  -payment
}

pub fn financial_pv(rate: f64, nper: f64, pmt: f64, fv: f64, pay_in_advance: bool) -> f64 {
  let pv = if rate == 0.0 {
    fv + pmt * nper
  } else if pay_in_advance {
    fv * (1.0 + rate).powf(-nper) + pmt * (1.0 - (1.0 + rate).powf(-nper + 1.0)) / rate + pmt
  } else {
    fv * (1.0 + rate).powf(-nper) + pmt * (1.0 - (1.0 + rate).powf(-nper)) / rate
  };
  -pv
}

pub fn financial_nper(rate: f64, pmt: f64, pv: f64, fv: f64, pay_in_advance: bool) -> Option<f64> {
  if pv + fv == 0.0 {
    return Some(0.0);
  }
  if rate == 0.0 {
    return Some(-(pv + fv) / pmt);
  }
  let numerator = if pay_in_advance {
    -(rate * fv - pmt * (1.0 + rate)) / (rate * pv + pmt * (1.0 + rate))
  } else {
    -(rate * fv - pmt) / (rate * pv + pmt)
  };
  (numerator > 0.0).then(|| numerator.ln() / rate.ln_1p())
}

pub fn financial_fv(rate: f64, nper: f64, pmt: f64, pv: f64, pay_in_advance: bool) -> f64 {
  let fv = if rate == 0.0 {
    pv + pmt * nper
  } else {
    let factor = (1.0 + rate).powf(nper);
    pv * factor + pmt * (1.0 + if pay_in_advance { rate } else { 0.0 }) * (factor - 1.0) / rate
  };
  -fv
}

pub fn financial_ipmt(
  rate: f64,
  period: f64,
  nper: f64,
  pv: f64,
  fv: f64,
  pay_in_advance: bool,
) -> (f64, f64) {
  let pmt = financial_pmt(rate, nper, pv, fv, pay_in_advance);
  let ipmt = if period == 1.0 {
    if pay_in_advance { 0.0 } else { -pv }
  } else if pay_in_advance {
    financial_fv(rate, period - 2.0, pmt, pv, true) - pmt
  } else {
    financial_fv(rate, period - 1.0, pmt, pv, false)
  };
  (ipmt * rate, pmt)
}

pub fn financial_cum(
  rate: f64,
  nper: f64,
  pv: f64,
  start: u64,
  end: u64,
  pay_in_advance: bool,
  interest: bool,
) -> f64 {
  let pmt = financial_pmt(rate, nper, pv, 0.0, pay_in_advance);
  let mut total = 0.0;
  let mut current = start;
  if current == 1 {
    if interest {
      if !pay_in_advance {
        total = -pv;
      }
    } else {
      total = if pay_in_advance { pmt } else { pmt + pv * rate };
    }
    current += 1;
  }
  while current <= end {
    if interest {
      total += if pay_in_advance {
        financial_fv(rate, current as f64 - 2.0, pmt, pv, true) - pmt
      } else {
        financial_fv(rate, current as f64 - 1.0, pmt, pv, false)
      };
    } else {
      total += if pay_in_advance {
        pmt - (financial_fv(rate, current as f64 - 2.0, pmt, pv, true) - pmt) * rate
      } else {
        pmt - financial_fv(rate, current as f64 - 1.0, pmt, pv, false) * rate
      };
    }
    current += 1;
  }
  if interest { total * rate } else { total }
}

pub fn financial_ddb(cost: f64, salvage: f64, life: f64, period: f64, factor: f64) -> f64 {
  let mut rate = factor / life;
  let old_value = if rate >= 1.0 {
    rate = 1.0;
    if period == 1.0 { cost } else { 0.0 }
  } else {
    cost * (1.0 - rate).powf(period - 1.0)
  };
  let new_value = cost * (1.0 - rate).powf(period);
  let ddb = if new_value < salvage {
    old_value - salvage
  } else {
    old_value - new_value
  };
  ddb.max(0.0)
}

pub fn financial_db(cost: f64, salvage: f64, life: f64, period: f64, months: f64) -> f64 {
  let off_rate = approx_floor(((1.0 - (salvage / cost).powf(1.0 / life)) * 1000.0) + 0.5) / 1000.0;
  let first_off_rate = cost * off_rate * months / 12.0;
  if approx_floor(period) == 1.0 {
    return first_off_rate;
  }
  let mut db = 0.0;
  let mut sum_off_rate = first_off_rate;
  let max = life.min(period).floor() as u16;
  for _ in 2..=max {
    db = -(sum_off_rate - cost) * off_rate;
    sum_off_rate += db;
  }
  if period > life {
    db = -(sum_off_rate - cost) * off_rate * (12.0 - months) / 12.0;
  }
  db
}

pub fn finance_price(
  settle: i32,
  maturity: i32,
  rate: f64,
  yield_value: f64,
  redemption: f64,
  frequency: i32,
  basis: i32,
) -> Option<f64> {
  let frequency_value = f64::from(frequency);
  let coupon_days = finance_coupdays(settle, maturity, frequency, basis)?;
  let dsc_e = finance_coupdaysnc(settle, maturity, frequency, basis)? / coupon_days;
  let coupon_count = finance_coupnum(settle, maturity, frequency, basis)?;
  let accrued_days = finance_coupdaybs(settle, maturity, frequency, basis)?;

  let discount = 1.0 + yield_value / frequency_value;
  let mut price = redemption / discount.powf(coupon_count - 1.0 + dsc_e);
  price -= 100.0 * rate / frequency_value * accrued_days / coupon_days;

  let coupon = 100.0 * rate / frequency_value;
  let mut k = 0.0;
  while k < coupon_count {
    price += coupon / discount.powf(k + dsc_e);
    k += 1.0;
  }
  Some(price)
}

#[derive(Clone, Copy)]
pub struct OddPeriodArgs {
  pub settle: i32,
  pub maturity: i32,
  pub last_coupon: i32,
  pub rate: f64,
  pub value: f64,
  pub redemption: f64,
  pub frequency: i32,
  pub basis: i32,
}

pub fn financial_oddlprice(args: OddPeriodArgs) -> Option<f64> {
  let frequency = f64::from(args.frequency);
  let dci = yearfrac(args.last_coupon, args.maturity, args.basis)? * frequency;
  let dsci = yearfrac(args.settle, args.maturity, args.basis)? * frequency;
  let ai = yearfrac(args.last_coupon, args.settle, args.basis)? * frequency;
  let mut price = args.redemption + dci * 100.0 * args.rate / frequency;
  price /= dsci * args.value / frequency + 1.0;
  price -= ai * 100.0 * args.rate / frequency;
  Some(price)
}

pub fn finance_yield(
  settle: i32,
  maturity: i32,
  coupon: f64,
  price: f64,
  redemption: f64,
  frequency: i32,
  basis: i32,
) -> Option<f64> {
  let rate = coupon;
  let mut price_n = 0.0;
  let mut yield_1 = 0.0;
  let mut yield_2 = 1.0;
  let mut price_1 = finance_price(
    settle, maturity, rate, yield_1, redemption, frequency, basis,
  )?;
  let mut price_2 = finance_price(
    settle, maturity, rate, yield_2, redemption, frequency, basis,
  )?;
  let mut yield_n = (yield_2 - yield_1) * 0.5;

  for _ in 0..100 {
    if approx_equal(price_n, price) {
      break;
    }
    price_n = finance_price(
      settle, maturity, rate, yield_n, redemption, frequency, basis,
    )?;
    if approx_equal(price, price_1) {
      return Some(yield_1);
    } else if approx_equal(price, price_2) {
      return Some(yield_2);
    } else if approx_equal(price, price_n) {
      return Some(yield_n);
    } else if price < price_2 {
      yield_2 *= 2.0;
      price_2 = finance_price(
        settle, maturity, rate, yield_2, redemption, frequency, basis,
      )?;
      yield_n = (yield_2 - yield_1) * 0.5;
    } else {
      if price < price_n {
        yield_1 = yield_n;
        price_1 = price_n;
      } else {
        yield_2 = yield_n;
        price_2 = price_n;
      }
      yield_n = yield_2 - (yield_2 - yield_1) * ((price - price_2) / (price_1 - price_2));
    }
  }

  if (price - price_n).abs() > price / 100.0 {
    None
  } else {
    Some(yield_n)
  }
}

pub fn financial_oddlyield(args: OddPeriodArgs) -> Option<f64> {
  let frequency = f64::from(args.frequency);
  let dci = yearfrac(args.last_coupon, args.maturity, args.basis)? * frequency;
  let dsci = yearfrac(args.settle, args.maturity, args.basis)? * frequency;
  let ai = yearfrac(args.last_coupon, args.settle, args.basis)? * frequency;
  let mut yield_value = args.redemption + dci * 100.0 * args.rate / frequency;
  yield_value /= args.value + ai * 100.0 * args.rate / frequency;
  yield_value -= 1.0;
  Some(yield_value * frequency / dsci)
}

pub fn financial_amordegrc(
  cost: f64,
  date: i32,
  first_period: i32,
  residual: f64,
  period: f64,
  rate: f64,
  basis: i32,
) -> Option<f64> {
  let period = period as u32;
  let use_period = 1.0 / rate;
  let coefficient = if use_period < 3.0 {
    1.0
  } else if use_period < 5.0 {
    1.5
  } else if use_period <= 6.0 {
    2.0
  } else {
    2.5
  };
  let rate = rate * coefficient;
  let mut depreciation = (yearfrac(date, first_period, basis)? * rate * cost).round();
  let mut cost = cost - depreciation;
  let mut rest = cost - residual;
  for index in 0..period {
    depreciation = (rate * cost).round();
    rest -= depreciation;
    if rest < 0.0 {
      return Some(match period - index {
        0 | 1 => (cost * 0.5).round(),
        _ => 0.0,
      });
    }
    cost -= depreciation;
  }
  Some(depreciation)
}

pub fn financial_amorlinc(
  cost: f64,
  date: i32,
  first_period: i32,
  residual: f64,
  period: f64,
  rate: f64,
  basis: i32,
) -> Option<f64> {
  let period = period as u32;
  let one_rate = cost * rate;
  let cost_delta = cost - residual;
  let first_rate = yearfrac(date, first_period, basis)? * rate * cost;
  let full_periods = ((cost - residual - first_rate) / one_rate) as u32;
  let result = if period == 0 {
    first_rate
  } else if period <= full_periods {
    one_rate
  } else if period == full_periods + 1 {
    cost_delta - one_rate * f64::from(full_periods) - first_rate
  } else {
    0.0
  };
  Some(result.max(0.0))
}

pub fn finance_duration(
  settle: i32,
  maturity: i32,
  coupon: f64,
  yield_value: f64,
  frequency: i32,
  basis: i32,
) -> Option<f64> {
  let yearfrac = yearfrac(settle, maturity, basis)?;
  let coupon_count = finance_coupnum(settle, maturity, frequency, basis)?;
  let coupon_cash = coupon * 100.0 / f64::from(frequency);
  let discount = 1.0 + yield_value / f64::from(frequency);
  let diff = yearfrac * f64::from(frequency) - coupon_count;

  let mut duration = 0.0;
  let mut period = 1.0;
  while period < coupon_count {
    duration += (period + diff) * coupon_cash / discount.powf(period + diff);
    period += 1.0;
  }
  duration += (coupon_count + diff) * (coupon_cash + 100.0) / discount.powf(coupon_count + diff);

  let mut price = 0.0;
  let mut period = 1.0;
  while period < coupon_count {
    price += coupon_cash / discount.powf(period + diff);
    period += 1.0;
  }
  price += (coupon_cash + 100.0) / discount.powf(coupon_count + diff);
  Some(duration / price / f64::from(frequency))
}

pub fn finance_year_diff(start: i32, end: i32, basis: i32) -> Option<f64> {
  let (start, end, negative) = if start > end {
    (end, start, true)
  } else {
    (start, end, false)
  };
  let (year1, month1, day1) = date_from_serial(start)?;
  let (year2, month2, day2) = date_from_serial(end)?;
  let (day_count, year_days) = match basis {
    0 | 4 => {
      let months = month2 as i32 - month1 as i32 + (year2 - year1) * 12;
      let mut days = day2 as i32 - day1 as i32;
      let mut total = months * 30 + days;
      if basis == 0 && month1 == 2 && month2 != 2 && year1 == year2 {
        days = if is_leap_year(year1) { 1 } else { 2 };
        total -= days;
      }
      (total, 360)
    }
    1 => {
      let year_days = if is_leap_year(year1) { 366 } else { 365 };
      (end - start, year_days)
    }
    2 => (end - start, 360),
    3 => (end - start, 365),
    _ => return None,
  };
  let result = f64::from(day_count) / f64::from(year_days);
  Some(if negative { -result } else { result })
}

pub fn financial_xnpv(rate: f64, values: &[f64], dates: &[f64]) -> Option<f64> {
  if values.len() != dates.len() || values.len() < 2 {
    return None;
  }
  let base = dates[0];
  let rate = rate + 1.0;
  let result = values
    .iter()
    .zip(dates)
    .map(|(value, date)| value / rate.powf((date - base) / 365.0))
    .sum::<f64>();
  result.is_finite().then_some(result)
}

pub fn financial_xirr(values: &[f64], dates: &[f64], guess: f64) -> Option<f64> {
  if values.len() != dates.len() || values.len() < 2 || guess <= -1.0 {
    return None;
  }
  const MAX_EPS: f64 = 1e-10;
  const MAX_ITER: usize = 50;
  let mut scan = 0usize;
  let mut rate = guess;
  loop {
    if scan >= 1 {
      rate = -0.99 + (scan - 1) as f64 * 0.01;
    }
    let mut iter = 0usize;
    let mut cont;
    let mut value;
    loop {
      value = financial_xirr_result(values, dates, rate);
      let derivative = financial_xirr_derivative(values, dates, rate);
      let new_rate = if derivative == 0.0 {
        f64::NAN
      } else {
        rate - value / derivative
      };
      let rate_eps = (new_rate - rate).abs();
      rate = new_rate;
      cont = rate_eps > MAX_EPS && value.abs() > MAX_EPS;
      if !cont || iter >= MAX_ITER {
        break;
      }
      iter += 1;
    }
    if !rate.is_finite() || !value.is_finite() {
      cont = true;
    }
    scan += 1;
    if !cont {
      return rate.is_finite().then_some(rate);
    }
    if scan >= 200 {
      return None;
    }
  }
}

fn financial_xirr_result(values: &[f64], dates: &[f64], rate: f64) -> f64 {
  let base = dates[0];
  let r = rate + 1.0;
  let mut result = values[0];
  for index in 1..values.len() {
    result += values[index] / r.powf((dates[index] - base) / 365.0);
  }
  result
}

fn financial_xirr_derivative(values: &[f64], dates: &[f64], rate: f64) -> f64 {
  let base = dates[0];
  let r = rate + 1.0;
  let mut result = 0.0;
  for index in 1..values.len() {
    let exponent = (dates[index] - base) / 365.0;
    result -= exponent * values[index] / r.powf(exponent + 1.0);
  }
  result
}

pub fn financial_irr(values: &[f64], guess: f64) -> Option<f64> {
  if values.is_empty() {
    return None;
  }
  const EPSILON: f64 = 1.0e-7;
  const MAX_ITER: usize = 20;
  let mut x = if guess == -1.0 { 0.1 } else { guess };
  let mut epsilon = 1.0;
  let mut iterations = 0usize;
  while epsilon > EPSILON && iterations < MAX_ITER {
    let mut numerator = 0.0;
    let mut denominator = 0.0;
    for (index, value) in values.iter().enumerate() {
      let count = index as f64;
      numerator += value / (1.0 + x).powf(count);
      denominator -= count * value / (1.0 + x).powf(count + 1.0);
    }
    let x_new = if denominator == 0.0 {
      f64::NAN
    } else {
      x - numerator / denominator
    };
    iterations += 1;
    epsilon = (x_new - x).abs();
    x = x_new;
    if !x.is_finite() {
      return None;
    }
  }
  if guess == 0.0 && x.abs() < EPSILON {
    x = 0.0;
  }
  (epsilon < EPSILON).then_some(x)
}

pub fn financial_mirr(values: &[f64], finance_rate: f64, reinvest_rate: f64) -> Option<f64> {
  if values.len() < 2 {
    return None;
  }
  let reinvest = reinvest_rate + 1.0;
  let invest = finance_rate + 1.0;
  let mut npv_reinvest = 0.0;
  let mut pow_reinvest = 1.0;
  let mut npv_invest = 0.0;
  let mut pow_invest = 1.0;
  let mut has_positive = false;
  let mut has_negative = false;
  let mut count = 0usize;
  for value in values {
    if *value > 0.0 {
      has_positive = true;
      npv_reinvest += value * pow_reinvest;
    } else if *value < 0.0 {
      has_negative = true;
      npv_invest += value * pow_invest;
    }
    pow_reinvest /= reinvest;
    pow_invest /= invest;
    count += 1;
  }
  if !(has_positive && has_negative) || npv_invest == 0.0 || count < 2 {
    return None;
  }
  let mut result = -(npv_reinvest / npv_invest);
  result *= reinvest.powf((count - 1) as f64);
  Some(result.powf(1.0 / (count - 1) as f64) - 1.0)
}

pub fn is_coupon_frequency(value: i32) -> bool {
  matches!(value, 1 | 2 | 4)
}

pub fn finance_couppcd(settle: i32, maturity: i32, frequency: i32, basis: i32) -> Option<i32> {
  if settle >= maturity || !is_coupon_frequency(frequency) {
    return None;
  }
  let settle = FinanceDate::from_serial(settle, basis)?;
  let maturity = FinanceDate::from_serial(maturity, basis)?;
  let mut date = maturity.clone();
  date.set_year(settle.year)?;
  if date < settle {
    date.add_years(1)?;
  }
  while date > settle {
    date.add_months(-12 / frequency)?;
  }
  date.serial()
}

pub fn finance_coupncd(settle: i32, maturity: i32, frequency: i32, basis: i32) -> Option<i32> {
  if settle >= maturity || !is_coupon_frequency(frequency) {
    return None;
  }
  let settle = FinanceDate::from_serial(settle, basis)?;
  let maturity = FinanceDate::from_serial(maturity, basis)?;
  let mut date = maturity.clone();
  date.set_year(settle.year)?;
  if date > settle {
    date.add_years(-1)?;
  }
  while date <= settle {
    date.add_months(12 / frequency)?;
  }
  date.serial()
}

pub fn finance_coupdaybs(settle: i32, maturity: i32, frequency: i32, basis: i32) -> Option<f64> {
  if settle >= maturity || !is_coupon_frequency(frequency) {
    return None;
  }
  let settle_date = FinanceDate::from_serial(settle, basis)?;
  let previous =
    FinanceDate::from_serial(finance_couppcd(settle, maturity, frequency, basis)?, basis)?;
  Some(f64::from(FinanceDate::diff(&previous, &settle_date)))
}

pub fn finance_coupdaysnc(settle: i32, maturity: i32, frequency: i32, basis: i32) -> Option<f64> {
  if settle >= maturity || !is_coupon_frequency(frequency) {
    return None;
  }
  if basis != 0 && basis != 4 {
    let settle_date = FinanceDate::from_serial(settle, basis)?;
    let next =
      FinanceDate::from_serial(finance_coupncd(settle, maturity, frequency, basis)?, basis)?;
    return Some(f64::from(FinanceDate::diff(&settle_date, &next)));
  }
  Some(
    finance_coupdays(settle, maturity, frequency, basis)?
      - finance_coupdaybs(settle, maturity, frequency, basis)?,
  )
}

pub fn finance_coupdays(settle: i32, maturity: i32, frequency: i32, basis: i32) -> Option<f64> {
  if settle >= maturity || !is_coupon_frequency(frequency) {
    return None;
  }
  if basis == 1 {
    let previous =
      FinanceDate::from_serial(finance_couppcd(settle, maturity, frequency, basis)?, basis)?;
    let mut next = previous.clone();
    next.add_months(12 / frequency)?;
    return Some(f64::from(FinanceDate::diff(&previous, &next)));
  }
  Some(f64::from(finance_days_in_year(basis)?) / f64::from(frequency))
}

pub fn finance_coupnum(settle: i32, maturity: i32, frequency: i32, basis: i32) -> Option<f64> {
  if settle >= maturity || !is_coupon_frequency(frequency) {
    return None;
  }
  let maturity_date = FinanceDate::from_serial(maturity, basis)?;
  let previous =
    FinanceDate::from_serial(finance_couppcd(settle, maturity, frequency, basis)?, basis)?;
  let months = (maturity_date.year - previous.year) * 12 + maturity_date.month - previous.month;
  Some(f64::from(months * frequency / 12))
}

fn finance_days_in_year(basis: i32) -> Option<i32> {
  match basis {
    0 | 2 | 4 => Some(360),
    1 => Some(365),
    3 => Some(365),
    _ => None,
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FinanceDate {
  orig_day: i32,
  day: i32,
  month: i32,
  year: i32,
  last_day_mode: bool,
  last_day: bool,
  days_30: bool,
  us_mode: bool,
}

impl FinanceDate {
  fn from_serial(serial: i32, basis: i32) -> Option<Self> {
    let (year, month, day) = date_from_serial(serial)?;
    let mut value = Self {
      orig_day: day as i32,
      day: day as i32,
      month: month as i32,
      year,
      last_day_mode: basis != 5,
      last_day: day as i32 >= last_day_of_month(year, month) as i32,
      days_30: matches!(basis, 0 | 4),
      us_mode: basis == 0,
    };
    value.set_day();
    Some(value)
  }

  fn set_day(&mut self) {
    if self.days_30 {
      self.day = self.orig_day.min(30);
      if self.last_day || self.day >= self.days_in_month_for(self.month) {
        self.day = 30;
      }
    } else {
      let last = self.days_in_month_for(self.month);
      self.day = if self.last_day {
        last
      } else {
        self.orig_day.min(last)
      };
    }
  }

  fn days_in_month_for(&self, month: i32) -> i32 {
    if self.days_30 {
      30
    } else {
      last_day_of_month(self.year, month as u32) as i32
    }
  }

  fn days_in_month_range(&self, from: i32, to: i32) -> i32 {
    if from > to {
      return 0;
    }
    if self.days_30 {
      (to - from + 1) * 30
    } else {
      (from..=to).map(|month| self.days_in_month_for(month)).sum()
    }
  }

  fn days_in_year_range(&self, from: i32, to: i32) -> i32 {
    if from > to {
      return 0;
    }
    if self.days_30 {
      (to - from + 1) * 360
    } else {
      (from..=to)
        .map(|year| if is_leap_year(year) { 366 } else { 365 })
        .sum()
    }
  }

  fn set_year(&mut self, year: i32) -> Option<()> {
    if !(0..=0x7fff).contains(&year) {
      return None;
    }
    self.year = year;
    self.set_day();
    Some(())
  }

  fn add_years(&mut self, years: i32) -> Option<()> {
    self.set_year(self.year + years)
  }

  fn add_months(&mut self, months: i32) -> Option<()> {
    let mut new_month = self.month + months;
    if new_month > 12 {
      new_month -= 1;
      self.set_year(self.year + new_month / 12)?;
      self.month = new_month % 12 + 1;
    } else if new_month < 1 {
      self.set_year(self.year + new_month / 12 - 1)?;
      self.month = new_month % 12 + 12;
    } else {
      self.month = new_month;
    }
    self.set_day();
    Some(())
  }

  fn serial(&self) -> Option<i32> {
    let last = last_day_of_month(self.year, self.month as u32) as i32;
    let real_day = if self.last_day_mode && self.last_day {
      last
    } else {
      last.min(self.orig_day)
    };
    date_serial(self.year, self.month, real_day).map(|value| value as i32)
  }

  fn diff(from: &Self, to: &Self) -> i32 {
    if from > to {
      return Self::diff(to, from);
    }
    let mut diff = 0;
    let mut from = from.clone();
    let mut to = to.clone();

    if to.days_30 {
      if to.us_mode {
        if ((from.month == 2) || (from.day < 30)) && to.orig_day == 31 {
          to.day = 31;
        } else if to.month == 2 && to.last_day {
          to.day = last_day_of_month(to.year, 2) as i32;
        }
      } else {
        if from.month == 2 && from.day == 30 {
          from.day = last_day_of_month(from.year, 2) as i32;
        }
        if to.month == 2 && to.day == 30 {
          to.day = last_day_of_month(to.year, 2) as i32;
        }
      }
    }

    if from.year < to.year || (from.year == to.year && from.month < to.month) {
      diff = from.days_in_month_current() - from.day + 1;
      from.orig_day = 1;
      from.day = 1;
      from.last_day = false;
      let _ = from.add_months(1);

      if from.year < to.year {
        diff += from.days_in_month_range(from.month, 12);
        let add = 13 - from.month;
        let _ = from.add_months(add);
        diff += from.days_in_year_range(from.year, to.year - 1);
        let _ = from.add_years(to.year - from.year);
      }

      diff += from.days_in_month_range(from.month, to.month - 1);
      let _ = from.add_months(to.month - from.month);
    }
    diff += to.day - from.day;
    diff.max(0)
  }

  fn days_in_month_current(&self) -> i32 {
    self.days_in_month_for(self.month)
  }
}

impl PartialOrd for FinanceDate {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for FinanceDate {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self
      .year
      .cmp(&other.year)
      .then_with(|| self.month.cmp(&other.month))
      .then_with(|| self.day.cmp(&other.day))
      .then_with(|| match (self.last_day, other.last_day) {
        (left, right) if left != right => left.cmp(&right),
        _ => self.orig_day.cmp(&other.orig_day),
      })
  }
}

fn financial_inter_vdb(
  cost: f64,
  salvage: f64,
  life: f64,
  life1: f64,
  period: f64,
  factor: f64,
) -> f64 {
  let int_end = approx_ceil(period);
  let loop_end = int_end as u64;
  let mut vdb = 0.0;
  let mut sln = 0.0;
  let mut salvage_value = cost - salvage;
  let mut now_sln = false;
  for i in 1..=loop_end {
    let mut term;
    if !now_sln {
      let ddb = financial_ddb(cost, salvage, life, i as f64, factor);
      sln = salvage_value / (life1 - (i - 1) as f64);
      if sln > ddb {
        term = sln;
        now_sln = true;
      } else {
        term = ddb;
        salvage_value -= ddb;
      }
    } else {
      term = sln;
    }
    if i == loop_end {
      term *= period + 1.0 - int_end;
    }
    vdb += term;
  }
  vdb
}

pub fn financial_vdb(
  cost: f64,
  salvage: f64,
  life: f64,
  start: f64,
  end: f64,
  factor: f64,
  no_switch: bool,
) -> f64 {
  let int_start = approx_floor(start);
  let int_end = approx_ceil(end);
  let loop_start = int_start as u64;
  let loop_end = int_end as u64;
  let mut vdb = 0.0;
  if no_switch {
    for i in loop_start + 1..=loop_end {
      let mut term = financial_ddb(cost, salvage, life, i as f64, factor);
      if i == loop_start + 1 {
        term *= end.min(int_start + 1.0) - start;
      } else if i == loop_end {
        term *= end + 1.0 - int_end;
      }
      vdb += term;
    }
  } else {
    let mut part = 0.0;
    if !approx_equal(start, int_start) || !approx_equal(end, int_end) {
      if !approx_equal(start, int_start) {
        let temp_int_end = int_start + 1.0;
        let temp_value = cost - financial_inter_vdb(cost, salvage, life, life, int_start, factor);
        part += (start - int_start)
          * financial_inter_vdb(
            temp_value,
            salvage,
            life,
            life - int_start,
            temp_int_end - int_start,
            factor,
          );
      }
      if !approx_equal(end, int_end) {
        let temp_int_start = int_end - 1.0;
        let temp_value =
          cost - financial_inter_vdb(cost, salvage, life, life, temp_int_start, factor);
        part += (int_end - end)
          * financial_inter_vdb(
            temp_value,
            salvage,
            life,
            life - temp_int_start,
            int_end - temp_int_start,
            factor,
          );
      }
    }
    let adjusted_cost = cost - financial_inter_vdb(cost, salvage, life, life, int_start, factor);
    vdb = financial_inter_vdb(
      adjusted_cost,
      salvage,
      life,
      life - int_start,
      int_end - int_start,
      factor,
    ) - part;
  }
  vdb
}

pub fn financial_rate(
  nper: f64,
  payment: f64,
  pv: f64,
  fv: f64,
  pay_type: bool,
  guess: f64,
  default_guess: bool,
) -> Option<f64> {
  let original_guess = guess;
  if let Some(value) = financial_rate_iteration(nper, payment, pv, fv, pay_type, guess) {
    return Some(value);
  }
  if default_guess {
    for step in 2..=10 {
      if let Some(value) = financial_rate_iteration(
        nper,
        payment,
        pv,
        fv,
        pay_type,
        original_guess * step as f64,
      ) {
        return Some(value);
      }
      if let Some(value) = financial_rate_iteration(
        nper,
        payment,
        pv,
        fv,
        pay_type,
        original_guess / step as f64,
      ) {
        return Some(value);
      }
    }
  }
  None
}

fn financial_rate_iteration(
  nper: f64,
  payment: f64,
  mut pv: f64,
  mut fv: f64,
  pay_type: bool,
  guess: f64,
) -> Option<f64> {
  const ITERATIONS_MAX: usize = 150;
  const EPSILON_SMALL: f64 = 1.0E-14;
  const SCD_EPSILON: f64 = 1.0E-7;
  if pay_type {
    fv -= payment;
    pv += payment;
  }
  let integer_nper = nper == round_to_integer(nper);
  let mut x = if integer_nper { guess } else { guess.max(-1.0) };
  let mut found = false;
  for _ in 0..ITERATIONS_MAX {
    let (geo_series, geo_derivation, pow_n, pow_n_minus_1) = if x == 0.0 {
      (nper, nper * (nper - 1.0) / 2.0, 1.0, 1.0)
    } else if integer_nper {
      let pow_n_minus_1 = (1.0 + x).powf(nper - 1.0);
      let pow_n = pow_n_minus_1 * (1.0 + x);
      (
        (pow_n - 1.0) / x,
        nper * pow_n_minus_1 / x - ((pow_n - 1.0) / x) / x,
        pow_n,
        pow_n_minus_1,
      )
    } else {
      let pow_n = (1.0 + x).powf(nper);
      let pow_n_minus_1 = (1.0 + x).powf(nper - 1.0);
      (
        (pow_n - 1.0) / x,
        nper * pow_n_minus_1 / x - ((pow_n - 1.0) / x) / x,
        pow_n,
        pow_n_minus_1,
      )
    };
    let term = fv + pv * pow_n + payment * geo_series;
    let term_derivation = pv * nper * pow_n_minus_1 + payment * geo_derivation;
    if term.abs() < EPSILON_SMALL {
      found = true;
      break;
    }
    let x_new = if term_derivation == 0.0 {
      x + 1.1 * SCD_EPSILON
    } else {
      x - term / term_derivation
    };
    found = (x_new - x).abs() < SCD_EPSILON;
    x = x_new;
    if found {
      break;
    }
    if !integer_nper && x < -1.0 {
      return None;
    }
  }
  if found && x > -1.0 { Some(x) } else { None }
}

fn round_to_integer(value: f64) -> f64 {
  if !value.is_finite() || value == 0.0 {
    return value;
  }
  let sign = value.is_sign_negative();
  let mut rounded = approx_floor(value.abs() + 0.5);
  if sign {
    rounded = -rounded;
  }
  rounded
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn loan_functions_match_known_spreadsheet_values() {
    assert!(
      (financial_pmt(0.08 / 12.0, 10.0, 10_000.0, 0.0, false) + 1037.0320893591567).abs() < 1.0e-10
    );
    assert!(
      (financial_fv(0.06 / 12.0, 10.0, -200.0, -500.0, false) - 2571.175347651979).abs() < 1.0e-10
    );
    assert!(
      (financial_pv(0.08 / 12.0, 10.0, -1000.0, 0.0, false) - 9642.903148907895).abs() < 1.0e-10
    );
  }

  #[test]
  fn coupon_helpers_preserve_serial_date_boundaries() {
    let settle = date_serial(2008, 2, 15).unwrap() as i32;
    let maturity = date_serial(2010, 11, 15).unwrap() as i32;
    assert_eq!(
      finance_couppcd(settle, maturity, 2, 0),
      date_serial(2007, 11, 15).map(|value| value as i32)
    );
    assert_eq!(
      finance_coupncd(settle, maturity, 2, 0),
      date_serial(2008, 5, 15).map(|value| value as i32)
    );
    assert_eq!(finance_coupnum(settle, maturity, 2, 0), Some(6.0));
  }

  #[test]
  fn irr_and_xnpv_validate_cashflow_shapes() {
    assert!(financial_irr(&[-100.0, 39.0, 59.0, 55.0, 20.0], 0.1).is_some_and(f64::is_finite));
    assert_eq!(financial_xnpv(0.1, &[1.0], &[1.0]), None);
  }
}
