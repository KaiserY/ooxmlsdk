use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use icu_collator::options::{CollatorOptions, Strength};
use icu_collator::{Collator, CollatorBorrowed, CollatorPreferences};
use icu_locale::Locale;
use num_complex::Complex;
use rustc_hash::FxHashMap;

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
  EtsCalculation, EtsCalculationOptions, EtsError, EtsKind, RegressionScalarState,
  scalar_state as regression_scalar_state_from_slices,
};
use crate::calc::statistics::{
  PercentileKind, kth_value, mean, mode_slice, percentile_sorted, variance_slice,
};
use crate::calc::text::{leftb, rightb};
use crate::code::FormulaControlOp;
use crate::dependency::ExternalReferenceId;
use crate::model::*;
use crate::program::{
  FormulaExprId, FormulaNodeKind, FormulaNumberLiteral, FormulaProgram, FormulaReference,
  FormulaSheetReference,
};
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
  FormulaEvaluatorEngine, evaluate_arg_direct, evaluate_formula_text_raw,
  evaluate_parsed_formula_raw, evaluate_program_with_context,
};
pub(crate) use query::{
  CriteriaPlan, DatabaseCriteriaPlan, FieldCriteriaPlan, IfsAggregate, LookupPlan,
  QueryValueSource, compare_numbers, compare_text, database_criterion_cell_present,
  database_criterion_entry_present, query_candidate_number,
};
pub(crate) use stack::{EvalArg, EvalArgs, EvalOperand};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct VlookupCacheKey {
  sheet: SheetId,
  range: CellRange,
  sorted: bool,
  lookup: VlookupCacheValue,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum VlookupCacheValue {
  Number(u64),
  String(String),
  Boolean(bool),
  Error(FormulaErrorValue),
  Blank,
}

#[derive(Default)]
struct VlookupCache {
  depth: usize,
  rows: FxHashMap<VlookupCacheKey, Option<u32>>,
  indexes: FxHashMap<VlookupRangeKey, FxHashMap<VlookupIndexValue, u32>>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct VlookupRangeKey {
  sheet: SheetId,
  start: CellAddress,
  end: CellAddress,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) enum VlookupIndexValue {
  Number(u64),
  AsciiText(String),
}

thread_local! {
  static VLOOKUP_CACHE: RefCell<VlookupCache> = RefCell::new(VlookupCache::default());
}

struct VlookupCacheScope;

impl Drop for VlookupCacheScope {
  fn drop(&mut self) {
    VLOOKUP_CACHE.with(|cache| {
      let mut cache = cache.borrow_mut();
      cache.depth = cache.depth.saturating_sub(1);
      if cache.depth == 0 {
        cache.rows.clear();
        cache.indexes.clear();
      }
    });
  }
}

pub(crate) fn with_lookup_cache<R>(f: impl FnOnce() -> R) -> R {
  VLOOKUP_CACHE.with(|cache| {
    let mut cache = cache.borrow_mut();
    if cache.depth == 0 {
      cache.rows.clear();
      cache.indexes.clear();
    }
    cache.depth += 1;
  });
  let _scope = VlookupCacheScope;
  f()
}

fn vlookup_cache_key(
  sheet: SheetId,
  range: CellRange,
  sorted: bool,
  lookup: &FormulaValue<'_>,
) -> Option<VlookupCacheKey> {
  let lookup = match lookup {
    FormulaValue::Number(value) if value.is_finite() => {
      VlookupCacheValue::Number(if *value == 0.0 { 0 } else { value.to_bits() })
    }
    FormulaValue::String(value) => VlookupCacheValue::String(value.to_string()),
    FormulaValue::Boolean(value) => VlookupCacheValue::Boolean(*value),
    FormulaValue::Error(value) => VlookupCacheValue::Error(*value),
    FormulaValue::Blank => VlookupCacheValue::Blank,
    FormulaValue::Number(_)
    | FormulaValue::Matrix(_)
    | FormulaValue::Reference(_)
    | FormulaValue::RefList(_) => return None,
  };
  Some(VlookupCacheKey {
    sheet,
    range,
    sorted,
    lookup,
  })
}

fn cached_vlookup_row(key: &VlookupCacheKey) -> Option<Option<u32>> {
  VLOOKUP_CACHE.with(|cache| {
    let cache = cache.borrow();
    (cache.depth > 0)
      .then(|| cache.rows.get(key).copied())
      .flatten()
  })
}

fn cache_vlookup_row(key: VlookupCacheKey, row: Option<u32>) {
  VLOOKUP_CACHE.with(|cache| {
    let mut cache = cache.borrow_mut();
    if cache.depth > 0 {
      cache.rows.insert(key, row);
    }
  });
}

fn indexed_vlookup_row(
  book: &FormulaEvaluationBook<'_>,
  sheet: SheetId,
  range: CellRange,
  lookup: VlookupIndexValue,
) -> Option<Option<u32>> {
  VLOOKUP_CACHE.with(|cache| {
    let mut cache = cache.borrow_mut();
    if cache.depth == 0 {
      return None;
    }
    let start_row = range.start.row.min(range.end.row);
    let end_row = range.start.row.max(range.end.row);
    let column = range.start.column.min(range.end.column);
    let key = VlookupRangeKey {
      sheet,
      start: CellAddress {
        column,
        row: start_row,
      },
      end: CellAddress {
        column,
        row: end_row,
      },
    };
    let index = cache.indexes.entry(key).or_insert_with(|| {
      let mut index = FxHashMap::default();
      for ((_, address), value) in book.cells.range((sheet, key.start)..=(sheet, key.end)) {
        if let Some(value) = vlookup_candidate_index_value(value) {
          index.entry(value).or_insert(address.row);
        }
      }
      index
    });
    Some(index.get(&lookup).copied())
  })
}

fn vlookup_candidate_index_value(value: &FormulaValue<'_>) -> Option<VlookupIndexValue> {
  match value {
    FormulaValue::Number(value) if value.is_finite() => {
      Some(VlookupIndexValue::Number(if *value == 0.0 {
        0
      } else {
        value.to_bits()
      }))
    }
    FormulaValue::Boolean(value) => Some(VlookupIndexValue::Number(if *value {
      1.0f64.to_bits()
    } else {
      0
    })),
    FormulaValue::String(value) if value.is_ascii() => {
      Some(VlookupIndexValue::AsciiText(value.to_ascii_lowercase()))
    }
    FormulaValue::Number(_)
    | FormulaValue::String(_)
    | FormulaValue::Error(_)
    | FormulaValue::Blank
    | FormulaValue::Matrix(_)
    | FormulaValue::Reference(_)
    | FormulaValue::RefList(_) => None,
  }
}
