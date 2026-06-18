use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use icu_collator::options::{CollatorOptions, Strength};
use icu_collator::{Collator, CollatorBorrowed, CollatorPreferences};
use icu_locale::Locale;
use num_complex::Complex;

use crate::calc::CalcEngine;
use crate::calc::datetime::{
  civil_from_days, date_serial_with_system, days_from_civil, weekday_index_from_serial,
};
use crate::calc::financial::financial_cum;
use crate::calc::numeric::{
  CeilingFloorKind, KahanSum, NumericError, approx_add, approx_equal, approx_floor,
  ceiling as numeric_ceiling, floor as numeric_floor, floor_to_i32, formula_mod, formula_subtract,
  kahan_sum, normalize_formula_number, round_half_away_from_zero, round_to_decimal_places,
};
use crate::calc::query::{
  FindTextError, QueryOp, QuerySearchType, detect_query_search_type, find_byte_text_position,
  find_text_position, lookup_text_contains, may_be_regex, may_be_wildcard, parse_criteria_operator,
  regex_match, wildcard_match,
};
use crate::calc::regression::{
  EtsCalculation, EtsError, EtsKind, RegressionScalarState,
  scalar_state as regression_scalar_state_from_slices,
};
use crate::calc::statistics::{
  PercentileKind, kth_value, mean, mode_slice, percentile_sorted, variance_slice,
};
use crate::calc::text::{leftb, rightb};
use crate::code::{
  FormulaArgRange, FormulaCode, FormulaControlOp, FormulaOp, formula_error_from_lex,
};
use crate::dependency::ExternalReferenceId;
use crate::model::*;
use crate::{
  AddressFlags, CellAddress, CellRange, FormulaErrorValue, FormulaValue, QualifiedRange, SheetId,
  SheetName,
};

mod context;
mod engine;
mod query;
mod stack;

#[cfg(test)]
pub(crate) use context::valid_date_serial_with_system;
pub(crate) use context::{
  DatabaseFunction, DatePart, FormulaEvaluator, TimePart, column_index_to_name, datevalue,
  display_text_from_value, display_text_from_value_with_number_format, error_text,
  error_text_value, error_value, format_number_with_format_code, parse_date_input, qualified_range,
  range_intersection_value, rtl_cos, rtl_sin, rtl_tan, split_indirect_intersection, timevalue,
};
pub(crate) use engine::{
  FormulaEvaluatorEngine, evaluate_arg_direct, evaluate_code_with_context,
  evaluate_formula_text_raw, evaluate_parsed_formula_raw,
};
pub(crate) use query::{
  CriteriaPlan, DatabaseCriteriaPlan, FieldCriteriaPlan, IfsAggregate, LookupPlan,
  QueryValueSource, compare_numbers, compare_text, database_criterion_cell_present,
  database_criterion_entry_present, query_candidate_number,
};
pub(crate) use stack::{EvalArg, EvalArgs, EvalOperand};
