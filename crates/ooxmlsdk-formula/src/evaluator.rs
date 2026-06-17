use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use icu_collator::options::{CollatorOptions, Strength};
use icu_collator::{Collator, CollatorBorrowed, CollatorPreferences};
use icu_locale::Locale;
use num_complex::Complex;
use regex::RegexBuilder;
use statrs::distribution::{
  Binomial, Continuous, ContinuousCDF, Discrete, DiscreteCDF, Exp, Hypergeometric, LogNormal,
  Normal, StudentsT, Weibull,
};

use crate::calc::CalcEngine;
use crate::calc::combinatorics::{
  combination_count, gcd_number, lcm_number, permutation_count, permutation_with_repetition_count,
};
use crate::calc::complex::{
  FormulaComplex, binary_suffix, format_complex_number as format_formula_complex_number,
  parse_complex_number,
};
use crate::calc::datetime::{
  basis_o_datetime_text, civil_from_days, date_diff_months, date_diff_years, date_from_serial,
  date_from_serial_with_system, date_serial, date_serial_with_system, days_from_civil, days360,
  is_leap_year, is_valid_libreoffice_gregorian_date, iso_weeknum_from_serial_with_system,
  last_day_of_month, normalized_date_components, weekday_index_from_serial,
  weeks_in_year_from_serial_with_system, weeks_mode_one_index, yearfrac,
};
use crate::calc::financial::{
  OddPeriodArgs, finance_coupdaybs, finance_coupdays, finance_coupdaysnc, finance_coupncd,
  finance_coupnum, finance_couppcd, finance_duration, finance_price, finance_year_diff,
  finance_yield, financial_amordegrc, financial_amorlinc, financial_cum, financial_db,
  financial_ddb, financial_fv, financial_ipmt, financial_irr, financial_mirr, financial_nper,
  financial_oddlprice, financial_oddlyield, financial_pmt, financial_pv, financial_rate,
  financial_vdb, financial_xirr, financial_xnpv, is_coupon_frequency,
};
use crate::calc::matrix::{
  apply_householder, determinant, lup_decompose, lup_solve, matrix_multiply, solve_lower,
  solve_upper,
};
use crate::calc::numeric::{
  CeilingFloorKind, KahanSum, NumericError, approx_add, approx_ceil, approx_equal, approx_floor,
  approx_sub, ceiling as numeric_ceiling, ceiling_excel_legacy, floor as numeric_floor,
  floor_excel_legacy, floor_to_i32, floor_to_u32, floor_to_u64, floor_to_usize, formula_mod,
  kahan_sum, mround, normalize_formula_number, quotient, round_direction,
  round_half_away_from_zero, round_to_decimal_places, round_to_significant_digits, trunc_to_u32,
};
use crate::calc::query::{
  FindTextError, QueryOp, QuerySearchType, detect_query_search_type, find_byte_text_position,
  find_text_position, lookup_text_contains, may_be_regex, may_be_wildcard, parse_criteria_operator,
  regex_match, wildcard_match,
};
use crate::calc::radix::{
  base_number_text, convert_from_decimal, convert_to_decimal, decimal_text_to_number,
};
use crate::calc::regression::{
  EtsCalculation, EtsError, EtsKind, RegressionModel, RegressionScalarState, RegressionState,
  regression_model as calc_regression_model, regression_state as calc_regression_state,
  scalar_state as regression_scalar_state_from_slices,
};
use crate::calc::special::{
  BesselKind, SpecialError, bessel, erf, gamma, lo_beta_dist, lo_beta_dist_pdf, lo_binom_dist_pmf,
  lo_binom_dist_range, lo_chi_dist, lo_chisq_dist_cdf, lo_chisq_dist_pdf, lo_f_dist_pdf,
  lo_f_dist_right_tail, lo_gamma_dist, lo_gamma_dist_pdf, lo_integral_phi, lo_iterate_inverse,
  lo_phi, lo_poisson_dist, lo_t_dist, log_gamma, norm_s_dist, norm_s_inv,
};
use crate::calc::statistics::{
  PercentileKind, StatisticsError, correlation, covariance, deviation_sum_squares,
  frequency_counts, kth_value, kurtosis, mean, mode_ms_values, mode_slice, percent_rank,
  percentile_sorted, rank_value, skewness, sum_x2 as stats_sum_x2, sum_xmy2 as stats_sum_xmy2,
  trim_mean, variance_slice,
};
use crate::calc::text::{
  baht_text, clean_formula_text, full_width_like_jis, half_width_like_asc, leftb, rightb,
  roman_text_libreoffice, text_byte_len, trim_formula_text,
};
use crate::calc::units::convert_unit;
use crate::code::{
  FormulaArgRange, FormulaCode, FormulaControlOp, FormulaOp, formula_error_from_lex,
};
use crate::dependency::ExternalReferenceId;
use crate::function::{FormulaFunctionId, canonical_function_name};
use crate::model::*;
use crate::{
  AddressFlags, CellAddress, CellRange, FormulaErrorValue, FormulaValue, QualifiedAddress,
  QualifiedRange, SheetId, SheetName,
};

mod ast;
mod context;
mod engine;
mod stack;

pub(crate) use ast::{FormulaAst, has_missing_required_argument, is_missing_argument};
pub(crate) use context::{
  CouponFunction, DatabaseFunction, DatePart, FormulaEvaluator, IfsAggregate, QueryGrid, TimePart,
  column_index_to_name, datevalue, display_text_from_value,
  display_text_from_value_with_number_format, error_text, error_text_value, error_value,
  qualified_range, range_intersection_value, rtl_cos, rtl_sin, rtl_tan,
  split_indirect_intersection, timevalue,
};
#[cfg(test)]
pub(crate) use context::{format_complex_result, valid_date_serial_with_system};
pub(crate) use engine::{
  FormulaEvaluatorEngine, evaluate_arg_direct, evaluate_code_with_context,
  evaluate_formula_text_raw, evaluate_parsed_formula_raw,
};
pub(crate) use stack::{EvalArg, EvalArgs, EvalOperand};
