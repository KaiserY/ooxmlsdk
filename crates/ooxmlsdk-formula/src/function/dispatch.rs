use std::borrow::Cow;

use statrs::distribution::{
  ContinuousCDF, Discrete, DiscreteCDF, Hypergeometric, LogNormal, Normal, StudentsT,
};

use super::{
  EvalContext, FormulaFunctionId, FunctionArgReader, FunctionArgs, resolve_function_name,
};
use crate::calc::combinatorics::{gcd_number, lcm_number};
use crate::calc::complex::{
  FormulaComplex, binary_suffix, format_complex_number as format_formula_complex_number,
  parse_complex_number,
};
use crate::calc::datetime::{
  date_from_serial_with_system, date_serial, date_serial_with_system, is_leap_year,
  is_valid_libreoffice_gregorian_date, iso_weeknum_from_serial_with_system, last_day_of_month,
  normalized_date_components, weeks_mode_one_index, yearfrac as date_yearfrac,
};
use crate::calc::financial::{finance_duration, finance_price, financial_pmt};
use crate::calc::matrix::{determinant, matrix_multiply};
use crate::calc::numeric::{
  CeilingFloorKind, KahanSum, NumericError, approx_ceil, approx_floor, ceiling_excel_legacy,
  even_odd, floor_excel_legacy, floor_to_i32, floor_to_u32, floor_to_usize, kahan_sum, mround,
  round_direction, round_to_decimal_places, round_to_significant_digits, sign_number,
};
use crate::calc::radix::{
  base_number_text, convert_from_decimal, convert_to_decimal, decimal_text_to_number,
};
use crate::calc::regression::EtsKind;
use crate::calc::special::{
  SpecialError, lo_chi_dist, lo_chisq_dist_cdf, lo_f_dist_right_tail, lo_gauss, lo_iterate_inverse,
  norm_s_inv,
};
use crate::calc::statistics::{
  PercentileKind, StatisticsError, covariance, deviation_sum_squares, frequency_counts, kurtosis,
  mode_slice, percent_rank, percentile_sorted, rank_value, skewness, trim_mean, variance_slice,
};
use crate::calc::text::{
  clean_formula_text, leftb, legacy_char_text, legacy_text_code, proper_formula_text, rightb,
  rot13_formula_text, text_byte_len, trim_formula_text,
};
use crate::calc::units::convert_unit;
use crate::code::FormulaOp;
use crate::evaluator::{
  DatabaseFunction, DatePart, IfsAggregate, TimePart, column_index_to_name, compare_text,
  datevalue, display_text_from_value, error_text_value, rtl_cos, rtl_sin, rtl_tan, timevalue,
};
use crate::model::{XLSX_MAX_COLUMN_ZERO_BASED, XLSX_MAX_ROW_ZERO_BASED};
use crate::{
  CellAddress, CellRange, DateSystem, FormulaErrorValue, FormulaGrammar, FormulaValue,
  PivotDataRequest, PivotFieldFilter, QualifiedRange,
};

pub(crate) fn evaluate_function<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  function: Option<FormulaFunctionId>,
  name: &Cow<'doc, str>,
  args: FunctionArgs<'_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let function = function.or_else(|| resolve_function_name(name.as_ref()))?;
  evaluate_function_id(evaluator, function, args)
}

fn evaluate_function_id<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  function: FormulaFunctionId,
  args: FunctionArgs<'_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  evaluate_function_reader(evaluator, function, args.reader(evaluator))
}

fn evaluate_function_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  function: FormulaFunctionId,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  match function {
    FormulaFunctionId::Sum => match args.numeric_aggregate(true)? {
      Ok(values) => Some(FormulaValue::Number(kahan_sum(values))),
      Err(error) => Some(FormulaValue::Error(error)),
    },
    FormulaFunctionId::Product => match args.numeric_aggregate(true)? {
      Ok(values) => {
        if values.is_empty() {
          Some(FormulaValue::Number(0.0))
        } else {
          Some(FormulaValue::Number(values.iter().product()))
        }
      }
      Err(error) => Some(FormulaValue::Error(error)),
    },
    FormulaFunctionId::Average => {
      let values = match args.numeric_aggregate(true)? {
        Ok(values) => values,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
      (!values.is_empty())
        .then(|| FormulaValue::Number(kahan_sum(values.iter().copied()) / values.len() as f64))
    }
    FormulaFunctionId::Min => Some(
      args
        .numeric_aggregate(true)?
        .map(|values| {
          values
            .iter()
            .copied()
            .reduce(f64::min)
            .map(FormulaValue::Number)
            .unwrap_or(FormulaValue::Number(0.0))
        })
        .unwrap_or_else(FormulaValue::Error),
    ),
    FormulaFunctionId::Max => Some(
      args
        .numeric_aggregate(true)?
        .map(|values| {
          values
            .iter()
            .copied()
            .reduce(f64::max)
            .map(FormulaValue::Number)
            .unwrap_or(FormulaValue::Number(0.0))
        })
        .unwrap_or_else(FormulaValue::Error),
    ),
    FormulaFunctionId::Mina => evaluate_mina_maxa_reader(evaluator, args, false),
    FormulaFunctionId::Maxa => evaluate_mina_maxa_reader(evaluator, args, true),
    FormulaFunctionId::Averagea => evaluate_averagea_reader(evaluator, args),
    FormulaFunctionId::Count => Some(FormulaValue::Number(args.count_numbers()? as f64)),
    FormulaFunctionId::Counta => Some(FormulaValue::Number(args.count_all_values()? as f64)),
    FormulaFunctionId::Countblank if args.len() == 1 => Some(FormulaValue::Number(
      evaluator.count_blank(&args.first_value()?) as f64,
    )),
    FormulaFunctionId::Mode => evaluate_mode_reader(args),
    FormulaFunctionId::Sumif if (2..=3).contains(&args.len()) => evaluator.evaluate_ifs_reader(
      args,
      Some(if args.len() == 3 { 2 } else { 0 }),
      0,
      2,
      IfsAggregate::Sum,
    ),
    FormulaFunctionId::Countif if args.len() == 2 => {
      evaluator.evaluate_ifs_reader(args, None, 0, 2, IfsAggregate::Count)
    }
    FormulaFunctionId::Averageif if (2..=3).contains(&args.len()) => evaluator.evaluate_ifs_reader(
      args,
      Some(if args.len() == 3 { 2 } else { 0 }),
      0,
      2,
      IfsAggregate::Average,
    ),
    FormulaFunctionId::Sumifs if args.len() >= 3 && !args.len().is_multiple_of(2) => {
      evaluator.evaluate_ifs_reader(args, Some(0), 1, args.len() - 1, IfsAggregate::Sum)
    }
    FormulaFunctionId::Countifs if args.len() >= 2 && args.len().is_multiple_of(2) => {
      evaluator.evaluate_ifs_reader(args, None, 0, args.len(), IfsAggregate::Count)
    }
    FormulaFunctionId::Averageifs if args.len() >= 3 && !args.len().is_multiple_of(2) => {
      evaluator.evaluate_ifs_reader(args, Some(0), 1, args.len() - 1, IfsAggregate::Average)
    }
    FormulaFunctionId::Maxifs if args.len() >= 3 && !args.len().is_multiple_of(2) => {
      evaluator.evaluate_ifs_reader(args, Some(0), 1, args.len() - 1, IfsAggregate::Max)
    }
    FormulaFunctionId::Minifs if args.len() >= 3 && !args.len().is_multiple_of(2) => {
      evaluator.evaluate_ifs_reader(args, Some(0), 1, args.len() - 1, IfsAggregate::Min)
    }
    FormulaFunctionId::Iserror if args.len() == 1 => {
      evaluate_information_error_value(evaluator, &args.first_value()?, |_| true)
    }
    FormulaFunctionId::Isna if args.len() == 1 => {
      evaluate_information_error_value(evaluator, &args.first_value()?, |error| {
        error == FormulaErrorValue::NA
      })
    }
    FormulaFunctionId::Iserr if args.len() == 1 => {
      evaluate_information_error_value(evaluator, &args.first_value()?, |error| {
        error != FormulaErrorValue::NA
      })
    }
    FormulaFunctionId::Isblank if args.len() == 1 => {
      evaluate_isblank_value(evaluator, &args.first_value()?)
    }
    FormulaFunctionId::Istext if args.len() == 1 => Some(FormulaValue::Boolean(matches!(
      evaluator.information_scalar_value(args.first_value()?),
      Some(FormulaValue::String(_))
    ))),
    FormulaFunctionId::Isnontext if args.len() == 1 => Some(FormulaValue::Boolean(!matches!(
      evaluator.information_scalar_value(args.first_value()?),
      Some(FormulaValue::String(_))
    ))),
    FormulaFunctionId::Islogical if args.len() == 1 => Some(FormulaValue::Boolean(matches!(
      evaluator.information_scalar_value(args.first_value()?),
      Some(FormulaValue::Boolean(_))
    ))),
    FormulaFunctionId::Isnumber if args.len() == 1 => {
      evaluate_isnumber_value(evaluator, &args.first_value()?)
    }
    FormulaFunctionId::Isref if args.len() == 1 => Some(FormulaValue::Boolean(matches!(
      args.first_value()?,
      FormulaValue::Reference(_) | FormulaValue::RefList(_)
    ))),
    FormulaFunctionId::Isformula if args.len() == 1 => evaluate_isformula_reader(evaluator, args),
    FormulaFunctionId::ErrorDotType if args.len() == 1 => {
      evaluate_error_type_value(evaluator, &args.first_value()?)
    }
    FormulaFunctionId::Errortype | FormulaFunctionId::OrgDotOpenofficeDotErrortype
      if args.len() == 1 =>
    {
      evaluate_error_type_raw_reader(evaluator, args)
    }
    FormulaFunctionId::Type if args.len() == 1 => {
      evaluate_type_value(evaluator, &args.first_value()?)
    }
    FormulaFunctionId::Areas => evaluate_areas_reader(evaluator, args),
    FormulaFunctionId::Row => evaluate_row_column_reader(evaluator, args, false),
    FormulaFunctionId::Column => evaluate_row_column_reader(evaluator, args, true),
    FormulaFunctionId::Rows => evaluate_rows_columns_reader(args, false),
    FormulaFunctionId::Columns => evaluate_rows_columns_reader(args, true),
    FormulaFunctionId::N if args.len() == 1 => evaluate_n_value(evaluator, &args.first_value()?),
    FormulaFunctionId::Len if args.len() == 1 => {
      evaluate_len_value(evaluator, &args.first_value()?, false)
    }
    FormulaFunctionId::Lenb if args.len() == 1 => {
      evaluate_len_value(evaluator, &args.first_value()?, true)
    }
    FormulaFunctionId::Info if args.len() == 1 => {
      evaluate_info_value(evaluator, &args.first_value()?)
    }
    FormulaFunctionId::T if args.len() == 1 => evaluate_t_reader(evaluator, args),
    FormulaFunctionId::Value if args.len() == 1 => evaluate_value_reader(evaluator, args),
    FormulaFunctionId::Numbervalue if (1..=3).contains(&args.len()) => {
      evaluate_numbervalue_reader(evaluator, args)
    }
    FormulaFunctionId::Convert if args.len() == 3 => evaluate_convert_reader(evaluator, args),
    FormulaFunctionId::Convert => Some(FormulaValue::Error(FormulaErrorValue::Unknown)),
    FormulaFunctionId::Encodeurl if args.len() == 1 => evaluate_encodeurl_reader(evaluator, args),
    FormulaFunctionId::Hyperlink if (1..=2).contains(&args.len()) => {
      evaluate_hyperlink_reader(evaluator, args)
    }
    FormulaFunctionId::Fixed if (1..=3).contains(&args.len()) => {
      evaluate_fixed_reader(evaluator, args)
    }
    FormulaFunctionId::Concat => evaluate_concat_reader(evaluator, args),
    FormulaFunctionId::Textjoin => evaluate_textjoin_reader(evaluator, args),
    FormulaFunctionId::Exact if args.len() == 2 => evaluate_exact_reader(evaluator, args),
    FormulaFunctionId::Find if (2..=3).contains(&args.len()) => {
      evaluate_find_reader(evaluator, args, true, false)
    }
    FormulaFunctionId::Search if (2..=3).contains(&args.len()) => {
      evaluate_find_reader(evaluator, args, false, false)
    }
    FormulaFunctionId::Findb if (2..=3).contains(&args.len()) => {
      evaluate_find_reader(evaluator, args, true, true)
    }
    FormulaFunctionId::Searchb if (2..=3).contains(&args.len()) => {
      evaluate_find_reader(evaluator, args, false, true)
    }
    FormulaFunctionId::Substitute if (3..=4).contains(&args.len()) => {
      evaluate_substitute_reader(evaluator, args)
    }
    FormulaFunctionId::Trim if args.len() == 1 => evaluate_trim_reader(evaluator, args),
    FormulaFunctionId::Proper if args.len() == 1 => Some(FormulaValue::String(Cow::Owned(
      proper_formula_text(&evaluator.text(&args.scalar_value(0)?)),
    ))),
    FormulaFunctionId::Rot13 if args.len() == 1 => Some(FormulaValue::String(Cow::Owned(
      rot13_formula_text(&evaluator.text(&args.scalar_value(0)?)),
    ))),
    FormulaFunctionId::Upper if args.len() == 1 => Some(FormulaValue::String(Cow::Owned(
      evaluator.text(&args.scalar_value(0)?).to_ascii_uppercase(),
    ))),
    FormulaFunctionId::Lower if args.len() == 1 => Some(FormulaValue::String(Cow::Owned(
      evaluator.text(&args.scalar_value(0)?).to_ascii_lowercase(),
    ))),
    FormulaFunctionId::Code if args.len() == 1 => Some(FormulaValue::Number(
      evaluator
        .text(&args.scalar_value(0)?)
        .chars()
        .next()
        .map(legacy_text_code)
        .unwrap_or(0) as f64,
    )),
    FormulaFunctionId::Unicode if args.len() == 1 => Some(FormulaValue::Number(
      evaluator
        .text(&args.scalar_value(0)?)
        .chars()
        .next()
        .map(|ch| ch as u32)
        .unwrap_or(0) as f64,
    )),
    FormulaFunctionId::Char if args.len() == 1 => {
      let code = evaluator.number(&args.scalar_value(0)?)?;
      legacy_char_text(code)
        .map(|text| FormulaValue::String(Cow::Owned(text)))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
    }
    FormulaFunctionId::Unichar if args.len() == 1 => {
      let code = evaluator.number(&args.scalar_value(0)?)?;
      let code = floor_to_u32(code)?;
      char::from_u32(code)
        .map(|ch| FormulaValue::String(Cow::Owned(ch.to_string())))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
    }
    FormulaFunctionId::Clean if args.len() == 1 => evaluate_clean_reader(evaluator, args),
    FormulaFunctionId::Abs if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, f64::abs)
    }
    FormulaFunctionId::Sign if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, sign_number)
    }
    FormulaFunctionId::Int if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, approx_floor)
    }
    FormulaFunctionId::Sqrt if args.len() == 1 => {
      let value = scalar_number_arg(evaluator, args, 0)?;
      if value < 0.0 {
        Some(FormulaValue::Error(FormulaErrorValue::Num))
      } else {
        Some(FormulaValue::Number(value.sqrt()))
      }
    }
    FormulaFunctionId::Power if args.len() == 2 => {
      let value = scalar_number_arg(evaluator, args, 0)?;
      let power = scalar_number_arg(evaluator, args, 1)?;
      let result = value.powf(power);
      if result.is_finite() {
        Some(FormulaValue::Number(result))
      } else {
        Some(FormulaValue::Error(FormulaErrorValue::Num))
      }
    }
    FormulaFunctionId::Even if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, |value| even_odd(value, true))
    }
    FormulaFunctionId::Odd if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, |value| even_odd(value, false))
    }
    FormulaFunctionId::Iseven if args.len() == 1 => Some(FormulaValue::Boolean(
      approx_floor(scalar_number_arg(evaluator, args, 0)?.abs()) as i64 % 2 == 0,
    )),
    FormulaFunctionId::Isodd if args.len() == 1 => Some(FormulaValue::Boolean(
      approx_floor(scalar_number_arg(evaluator, args, 0)?.abs()) as i64 % 2 != 0,
    )),
    FormulaFunctionId::Sqrtpi if args.len() == 1 => {
      let value = scalar_number_arg(evaluator, args, 0)?;
      if value < 0.0 {
        Some(FormulaValue::Error(FormulaErrorValue::Num))
      } else {
        Some(FormulaValue::Number((value * std::f64::consts::PI).sqrt()))
      }
    }
    FormulaFunctionId::Radians if args.len() == 1 => Some(FormulaValue::Number(
      scalar_number_arg(evaluator, args, 0)?.to_radians(),
    )),
    FormulaFunctionId::Degrees if args.len() == 1 => Some(FormulaValue::Number(
      scalar_number_arg(evaluator, args, 0)?.to_degrees(),
    )),
    FormulaFunctionId::Sin if args.len() == 1 => scalar_numeric_unary_arg(evaluator, args, rtl_sin),
    FormulaFunctionId::Csc if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, |value| 1.0 / rtl_sin(value))
    }
    FormulaFunctionId::Cos if args.len() == 1 => scalar_numeric_unary_arg(evaluator, args, rtl_cos),
    FormulaFunctionId::Sec if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, |value| 1.0 / rtl_cos(value))
    }
    FormulaFunctionId::Tan if args.len() == 1 => scalar_numeric_unary_arg(evaluator, args, rtl_tan),
    FormulaFunctionId::Cot if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, |value| 1.0 / rtl_tan(value))
    }
    FormulaFunctionId::Sinh if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, f64::sinh)
    }
    FormulaFunctionId::Csch if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, |value| 1.0 / value.sinh())
    }
    FormulaFunctionId::Cosh if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, f64::cosh)
    }
    FormulaFunctionId::Sech if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, |value| 1.0 / value.cosh())
    }
    FormulaFunctionId::Tanh if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, f64::tanh)
    }
    FormulaFunctionId::Coth if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, |value| 1.0 / value.tanh())
    }
    FormulaFunctionId::Asin if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, f64::asin)
    }
    FormulaFunctionId::Asinh if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, f64::asinh)
    }
    FormulaFunctionId::Acos if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, f64::acos)
    }
    FormulaFunctionId::Acosh if args.len() == 1 => scalar_numeric_unary_checked_arg(
      evaluator,
      args,
      |value| (value >= 1.0).then(|| value.acosh()),
      FormulaErrorValue::IllegalArgument,
    ),
    FormulaFunctionId::Acot if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, |value| {
        std::f64::consts::FRAC_PI_2 - value.atan()
      })
    }
    FormulaFunctionId::Atan if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, f64::atan)
    }
    FormulaFunctionId::Atanh if args.len() == 1 => scalar_numeric_unary_checked_arg(
      evaluator,
      args,
      |value| (value.abs() < 1.0).then(|| value.atanh()),
      FormulaErrorValue::IllegalArgument,
    ),
    FormulaFunctionId::Acoth if args.len() == 1 => scalar_numeric_unary_checked_arg(
      evaluator,
      args,
      |value| (value.abs() > 1.0).then(|| (1.0 / value).atanh()),
      FormulaErrorValue::IllegalArgument,
    ),
    FormulaFunctionId::Atan2 if args.len() == 2 => Some(FormulaValue::Number(
      scalar_number_arg(evaluator, args, 1)?.atan2(scalar_number_arg(evaluator, args, 0)?),
    )),
    FormulaFunctionId::Exp if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, f64::exp)
    }
    FormulaFunctionId::Ln if args.len() == 1 => scalar_numeric_unary_checked_arg(
      evaluator,
      args,
      |value| (value > 0.0).then(|| value.ln()),
      FormulaErrorValue::IllegalArgument,
    ),
    FormulaFunctionId::Log10 if args.len() == 1 => scalar_numeric_unary_checked_arg(
      evaluator,
      args,
      |value| (value > 0.0).then(|| value.log10()),
      FormulaErrorValue::IllegalArgument,
    ),
    FormulaFunctionId::Log if (1..=2).contains(&args.len()) => {
      let value = scalar_number_arg(evaluator, args, 0)?;
      let base = if args.len() == 2 {
        scalar_number_arg(evaluator, args, 1)?
      } else {
        10.0
      };
      if value > 0.0 && base > 0.0 && base != 1.0 {
        Some(FormulaValue::Number(value.log(base)))
      } else {
        Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
      }
    }
    FormulaFunctionId::Sumsq => evaluate_sumsq_reader(args),
    FormulaFunctionId::Sumx2my2 if args.len() == 2 => evaluate_sumx2_reader(evaluator, args, false),
    FormulaFunctionId::Sumx2py2 if args.len() == 2 => evaluate_sumx2_reader(evaluator, args, true),
    FormulaFunctionId::Sumxmy2 if args.len() == 2 => evaluate_sumxmy2_reader(evaluator, args),
    FormulaFunctionId::Devsq => evaluate_devsq_reader(args),
    FormulaFunctionId::Avedev => evaluate_avedev_reader(args),
    FormulaFunctionId::Stdev | FormulaFunctionId::StdevDotS => {
      evaluate_stdev_reader(evaluator, args, true, false)
    }
    FormulaFunctionId::StdevDotP => evaluate_stdev_reader(evaluator, args, false, false),
    FormulaFunctionId::Stdeva => evaluate_stdev_reader(evaluator, args, true, true),
    FormulaFunctionId::Stdevpa => evaluate_stdev_reader(evaluator, args, false, true),
    FormulaFunctionId::VarDotS => evaluate_variance_reader(evaluator, args, true, false),
    FormulaFunctionId::VarDotP => evaluate_variance_reader(evaluator, args, false, false),
    FormulaFunctionId::Vara => evaluate_variance_reader(evaluator, args, true, true),
    FormulaFunctionId::Varpa => evaluate_variance_reader(evaluator, args, false, true),
    FormulaFunctionId::Kurt => evaluate_kurt_reader(args),
    FormulaFunctionId::Skew => evaluate_skew_reader(args, false),
    FormulaFunctionId::SkewDotP => evaluate_skew_reader(args, true),
    FormulaFunctionId::Sumproduct if !args.is_empty() => {
      evaluate_sumproduct_reader(evaluator, args)
    }
    FormulaFunctionId::Round if args.len() == 2 => {
      Some(FormulaValue::Number(round_to_decimal_places(
        scalar_number_arg(evaluator, args, 0)?,
        scalar_number_arg(evaluator, args, 1)? as i32,
      )))
    }
    FormulaFunctionId::Roundup if (1..=2).contains(&args.len()) => {
      evaluate_round_direction_reader(evaluator, args, true)
    }
    FormulaFunctionId::Rounddown if (1..=2).contains(&args.len()) => {
      evaluate_round_direction_reader(evaluator, args, false)
    }
    FormulaFunctionId::Roundsig if args.len() == 2 => evaluate_roundsig_reader(evaluator, args),
    FormulaFunctionId::Mod if args.len() == 2 => {
      let number = args.value(0)?;
      let divisor = args.value(1)?;
      if evaluator.array_context && (is_matrix_argument(&number) || is_matrix_argument(&divisor)) {
        evaluator.map_binary_values(number, divisor, |evaluator, number, divisor| {
          Some(evaluator.mod_value(number, divisor))
        })
      } else {
        Some(evaluator.mod_value(&number, &divisor))
      }
    }
    FormulaFunctionId::Trunc if (1..=2).contains(&args.len()) => {
      evaluate_trunc_reader(evaluator, args)
    }
    FormulaFunctionId::Mround if args.len() == 2 => evaluate_mround_reader(evaluator, args),
    FormulaFunctionId::Gcd => evaluate_gcd_lcm_reader(args, true),
    FormulaFunctionId::Lcm => evaluate_gcd_lcm_reader(args, false),
    FormulaFunctionId::Median => evaluate_median_reader(evaluator, args),
    FormulaFunctionId::Large if args.len() == 2 => {
      evaluate_large_small_reader(evaluator, args, true)
    }
    FormulaFunctionId::Small if args.len() == 2 => {
      evaluate_large_small_reader(evaluator, args, false)
    }
    FormulaFunctionId::Trimmean if args.len() == 2 => evaluate_trimmean_reader(evaluator, args),
    FormulaFunctionId::PercentileDotInc if args.len() == 2 => {
      evaluate_percentile_reader(evaluator, args, PercentileKind::Inc)
    }
    FormulaFunctionId::PercentileDotExc if args.len() == 2 => {
      evaluate_percentile_reader(evaluator, args, PercentileKind::Exc)
    }
    FormulaFunctionId::QuartileDotInc if args.len() == 2 => {
      evaluate_quartile_reader(evaluator, args, PercentileKind::Inc)
    }
    FormulaFunctionId::QuartileDotExc if args.len() == 2 => {
      evaluate_quartile_reader(evaluator, args, PercentileKind::Exc)
    }
    FormulaFunctionId::Percentrank if (2..=3).contains(&args.len()) => {
      evaluate_percentrank_reader(evaluator, args, PercentileKind::Inc)
    }
    FormulaFunctionId::PercentrankDotExc if (2..=3).contains(&args.len()) => {
      evaluate_percentrank_reader(evaluator, args, PercentileKind::Exc)
    }
    FormulaFunctionId::RankDotEq if (2..=3).contains(&args.len()) => {
      evaluate_rank_reader(evaluator, args, false)
    }
    FormulaFunctionId::RankDotAvg if (2..=3).contains(&args.len()) => {
      evaluate_rank_reader(evaluator, args, true)
    }
    FormulaFunctionId::Geomean => evaluate_geomean_harmean_reader(args, true),
    FormulaFunctionId::Harmean => evaluate_geomean_harmean_reader(args, false),
    FormulaFunctionId::Mmult if args.len() == 2 => evaluate_mmult_reader(evaluator, args),
    FormulaFunctionId::Mdeterm if args.len() == 1 => evaluate_mdeterm_reader(evaluator, args),
    FormulaFunctionId::Munit if args.len() == 1 => evaluate_munit_reader(evaluator, args),
    FormulaFunctionId::Filter if (2..=3).contains(&args.len()) => {
      evaluate_filter_reader(evaluator, args)
    }
    FormulaFunctionId::Hstack if !args.is_empty() => evaluate_stack_reader(evaluator, args, true),
    FormulaFunctionId::Vstack if !args.is_empty() => evaluate_stack_reader(evaluator, args, false),
    FormulaFunctionId::Sequence if (1..=4).contains(&args.len()) => {
      evaluate_sequence_reader(evaluator, args)
    }
    FormulaFunctionId::Take if (1..=3).contains(&args.len()) => {
      evaluate_take_drop_reader(evaluator, args, true)
    }
    FormulaFunctionId::Drop if (1..=3).contains(&args.len()) => {
      evaluate_take_drop_reader(evaluator, args, false)
    }
    FormulaFunctionId::Sort if (1..=4).contains(&args.len()) => {
      evaluate_sort_reader(evaluator, args)
    }
    FormulaFunctionId::Sortby if args.len() >= 2 => evaluate_sortby_reader(evaluator, args),
    FormulaFunctionId::Chooserows if args.len() >= 2 => {
      evaluate_choose_rows_columns_reader(evaluator, args, true)
    }
    FormulaFunctionId::Choosecols if args.len() >= 2 => {
      evaluate_choose_rows_columns_reader(evaluator, args, false)
    }
    FormulaFunctionId::Ceiling if (1..=3).contains(&args.len()) => {
      evaluate_ceiling_floor_reader(evaluator, args, true, CeilingFloorKind::Odff)
    }
    FormulaFunctionId::ComDotMicrosoftDotCeiling if args.len() == 2 => {
      evaluate_ceiling_floor_legacy_reader(evaluator, args, true)
    }
    FormulaFunctionId::ComDotMicrosoftDotCeiling => {
      Some(FormulaValue::Error(FormulaErrorValue::Unknown))
    }
    FormulaFunctionId::Floor if (1..=3).contains(&args.len()) => {
      evaluate_ceiling_floor_reader(evaluator, args, false, CeilingFloorKind::Odff)
    }
    FormulaFunctionId::ComDotMicrosoftDotFloor if args.len() == 2 => {
      evaluate_ceiling_floor_legacy_reader(evaluator, args, false)
    }
    FormulaFunctionId::ComDotMicrosoftDotFloor => {
      Some(FormulaValue::Error(FormulaErrorValue::Unknown))
    }
    FormulaFunctionId::CeilingDotMath if (1..=3).contains(&args.len()) => {
      evaluate_ceiling_floor_reader(evaluator, args, true, CeilingFloorKind::Math)
    }
    FormulaFunctionId::FloorDotMath if (1..=3).contains(&args.len()) => {
      evaluate_ceiling_floor_reader(evaluator, args, false, CeilingFloorKind::Math)
    }
    FormulaFunctionId::CeilingDotPrecise if (1..=2).contains(&args.len()) => {
      evaluate_ceiling_floor_reader(evaluator, args, true, CeilingFloorKind::Precise)
    }
    FormulaFunctionId::FloorDotPrecise if (1..=2).contains(&args.len()) => {
      evaluate_ceiling_floor_reader(evaluator, args, false, CeilingFloorKind::Precise)
    }
    FormulaFunctionId::Date if args.len() == 3 => evaluate_date_reader(evaluator, args),
    FormulaFunctionId::Datevalue if args.len() == 1 => Some(datevalue(
      &evaluator.text(&args.first_value()?),
      evaluator.book.date_system,
    )),
    FormulaFunctionId::Timevalue if args.len() == 1 => {
      Some(timevalue(&evaluator.text(&args.first_value()?)))
    }
    FormulaFunctionId::Time if args.len() == 3 => evaluate_time_reader(evaluator, args),
    FormulaFunctionId::Year if args.len() == 1 => {
      evaluate_date_part_reader(evaluator, args, DatePart::Year)
    }
    FormulaFunctionId::Month if args.len() == 1 => {
      evaluate_date_part_reader(evaluator, args, DatePart::Month)
    }
    FormulaFunctionId::Day if args.len() == 1 => {
      evaluate_date_part_reader(evaluator, args, DatePart::Day)
    }
    FormulaFunctionId::Hour if args.len() == 1 => {
      evaluate_time_part_reader(evaluator, args, TimePart::Hour)
    }
    FormulaFunctionId::Minute if args.len() == 1 => {
      evaluate_time_part_reader(evaluator, args, TimePart::Minute)
    }
    FormulaFunctionId::Second if args.len() == 1 => {
      evaluate_time_part_reader(evaluator, args, TimePart::Second)
    }
    FormulaFunctionId::Days if args.len() == 2 => {
      let end = evaluator.date_number_from_value(&args.value(0)?)?;
      let start = evaluator.date_number_from_value(&args.value(1)?)?;
      Some(FormulaValue::Number(end.floor() - start.floor()))
    }
    FormulaFunctionId::Weeks if args.len() == 3 => evaluate_weeks_reader(evaluator, args),
    FormulaFunctionId::Daysinmonth if args.len() == 1 => {
      evaluate_days_in_month_year_reader(evaluator, args, false)
    }
    FormulaFunctionId::Daysinyear if args.len() == 1 => {
      evaluate_days_in_month_year_reader(evaluator, args, true)
    }
    FormulaFunctionId::Datedif if args.len() == 3 => evaluate_datedif_reader(evaluator, args),
    FormulaFunctionId::Yearfrac if (2..=3).contains(&args.len()) => {
      evaluate_yearfrac_reader(evaluator, args)
    }
    FormulaFunctionId::Eomonth if args.len() == 2 => evaluate_eomonth_reader(evaluator, args),
    FormulaFunctionId::Edate if args.len() == 2 => evaluate_edate_reader(evaluator, args),
    FormulaFunctionId::Eastersunday if args.len() == 1 => {
      evaluate_eastersunday_reader(evaluator, args)
    }
    FormulaFunctionId::Isleapyear if args.len() == 1 => {
      evaluate_is_leap_year_reader(evaluator, args)
    }
    FormulaFunctionId::Isoweeknum if args.len() == 1 => {
      evaluate_iso_weeknum_reader(evaluator, args)
    }
    FormulaFunctionId::Today if args.is_empty() => evaluator.evaluate_today(),
    FormulaFunctionId::Networkdays if (2..=3).contains(&args.len()) => {
      evaluator.evaluate_networkdays_reader(args, false)
    }
    FormulaFunctionId::NetworkdaysDotIntl if (2..=4).contains(&args.len()) => {
      evaluator.evaluate_networkdays_reader(args, true)
    }
    FormulaFunctionId::Networkdays | FormulaFunctionId::NetworkdaysDotIntl => {
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    }
    FormulaFunctionId::Workday if (2..=3).contains(&args.len()) => {
      evaluator.evaluate_workday_reader(args, false)
    }
    FormulaFunctionId::WorkdayDotIntl if (2..=4).contains(&args.len()) => {
      evaluator.evaluate_workday_reader(args, true)
    }
    FormulaFunctionId::Dollar if (1..=2).contains(&args.len()) => {
      evaluate_dollar_reader(evaluator, args)
    }
    FormulaFunctionId::Base if (2..=3).contains(&args.len()) => {
      evaluate_base_reader(evaluator, args)
    }
    FormulaFunctionId::Decimal if args.len() == 2 => evaluate_decimal_reader(evaluator, args),
    FormulaFunctionId::Bin2dec if args.len() == 1 => {
      evaluate_base_to_decimal_reader(evaluator, args, 2)
    }
    FormulaFunctionId::Oct2dec if args.len() == 1 => {
      evaluate_base_to_decimal_reader(evaluator, args, 8)
    }
    FormulaFunctionId::Hex2dec if args.len() == 1 => {
      evaluate_base_to_decimal_reader(evaluator, args, 16)
    }
    FormulaFunctionId::Bin2oct if (1..=2).contains(&args.len()) => {
      evaluate_base_to_base_reader(evaluator, args, 2, 8, -512.0, 511.0)
    }
    FormulaFunctionId::Bin2hex if (1..=2).contains(&args.len()) => {
      evaluate_base_to_base_reader(evaluator, args, 2, 16, -512.0, 511.0)
    }
    FormulaFunctionId::Oct2bin if (1..=2).contains(&args.len()) => {
      evaluate_base_to_base_reader(evaluator, args, 8, 2, -512.0, 511.0)
    }
    FormulaFunctionId::Oct2hex if (1..=2).contains(&args.len()) => {
      evaluate_base_to_base_reader(evaluator, args, 8, 16, -536_870_912.0, 536_870_911.0)
    }
    FormulaFunctionId::Hex2bin if (1..=2).contains(&args.len()) => {
      evaluate_base_to_base_reader(evaluator, args, 16, 2, -512.0, 511.0)
    }
    FormulaFunctionId::Hex2oct if (1..=2).contains(&args.len()) => {
      evaluate_base_to_base_reader(evaluator, args, 16, 8, -536_870_912.0, 536_870_911.0)
    }
    FormulaFunctionId::Dec2bin if (1..=2).contains(&args.len()) => {
      evaluate_decimal_to_base_reader(evaluator, args, 2, -512.0, 511.0)
    }
    FormulaFunctionId::Dec2oct if (1..=2).contains(&args.len()) => {
      evaluate_decimal_to_base_reader(evaluator, args, 8, -536_870_912.0, 536_870_911.0)
    }
    FormulaFunctionId::Dec2hex if (1..=2).contains(&args.len()) => {
      evaluate_decimal_to_base_reader(evaluator, args, 16, -549_755_813_888.0, 549_755_813_887.0)
    }
    FormulaFunctionId::Left if (1..=2).contains(&args.len()) => {
      evaluate_left_reader(evaluator, args, false)
    }
    FormulaFunctionId::Leftb if (1..=2).contains(&args.len()) => {
      evaluate_left_reader(evaluator, args, true)
    }
    FormulaFunctionId::Right if (1..=2).contains(&args.len()) => {
      evaluate_right_reader(evaluator, args, false)
    }
    FormulaFunctionId::Rightb if (1..=2).contains(&args.len()) => {
      evaluate_right_reader(evaluator, args, true)
    }
    FormulaFunctionId::Mid if args.len() == 3 => evaluate_mid_reader(evaluator, args, false),
    FormulaFunctionId::Midb if args.len() == 3 => evaluate_mid_reader(evaluator, args, true),
    FormulaFunctionId::Replaceb if args.len() == 4 => evaluate_replaceb_reader(evaluator, args),
    FormulaFunctionId::Cell if (1..=2).contains(&args.len()) => {
      evaluate_cell_reader(evaluator, args)
    }
    FormulaFunctionId::Address if (2..=5).contains(&args.len()) => {
      evaluate_address_reader(evaluator, args)
    }
    FormulaFunctionId::Formula if args.len() == 1 => evaluate_formula_text_reader(evaluator, args),
    FormulaFunctionId::Sheet => evaluator.evaluate_sheet_reader(args),
    FormulaFunctionId::Sheets => evaluator.evaluate_sheets_reader(args),
    FormulaFunctionId::Indirect if (1..=2).contains(&args.len()) => {
      evaluate_indirect_reader(evaluator, args)
    }
    FormulaFunctionId::Index if (1..=4).contains(&args.len()) => {
      evaluate_index_reader(evaluator, args)
    }
    FormulaFunctionId::Offset if (3..=5).contains(&args.len()) => {
      evaluate_offset_reader(evaluator, args)
    }
    FormulaFunctionId::Lookup if (2..=3).contains(&args.len()) => {
      evaluator.evaluate_lookup_reader(args)
    }
    FormulaFunctionId::Match if (2..=3).contains(&args.len()) => {
      evaluator.evaluate_match_reader(args)
    }
    FormulaFunctionId::Xmatch if (2..=4).contains(&args.len()) => {
      evaluator.evaluate_xmatch_reader(args)
    }
    FormulaFunctionId::Xlookup if (3..=6).contains(&args.len()) => {
      evaluator.evaluate_xlookup_reader(args)
    }
    FormulaFunctionId::Vlookup if (3..=4).contains(&args.len()) => {
      evaluate_vlookup_reader(evaluator, args, false)
    }
    FormulaFunctionId::Hlookup if (3..=4).contains(&args.len()) => {
      evaluate_vlookup_reader(evaluator, args, true)
    }
    FormulaFunctionId::Getpivotdata if args.len() >= 2 => {
      evaluate_getpivotdata_reader(evaluator, args)
    }
    FormulaFunctionId::ChisqDotInvDotRt if args.len() == 2 => {
      evaluate_chisq_inv_reader(evaluator, args, true)
    }
    FormulaFunctionId::Forecast if args.len() == 3 => evaluator.evaluate_forecast_reader(args),
    FormulaFunctionId::ForecastDotEts if (3..=6).contains(&args.len()) => {
      evaluator.evaluate_forecast_ets_reader(args, EtsKind::Add)
    }
    FormulaFunctionId::ForecastDotEtsDotMult if (3..=6).contains(&args.len()) => {
      evaluator.evaluate_forecast_ets_reader(args, EtsKind::Mult)
    }
    FormulaFunctionId::ForecastDotEtsDotStat if (3..=6).contains(&args.len()) => {
      evaluator.evaluate_forecast_ets_reader(args, EtsKind::StatAdd)
    }
    FormulaFunctionId::ForecastDotEtsDotStatDotMult if (3..=6).contains(&args.len()) => {
      evaluator.evaluate_forecast_ets_reader(args, EtsKind::StatMult)
    }
    FormulaFunctionId::ForecastDotEtsDotSeasonality if (2..=4).contains(&args.len()) => {
      evaluator.evaluate_forecast_ets_reader(args, EtsKind::Season)
    }
    FormulaFunctionId::ConfidenceDotNorm if args.len() == 3 => {
      evaluate_confidence_reader(evaluator, args, false)
    }
    FormulaFunctionId::ConfidenceDotT if args.len() == 3 => {
      evaluate_confidence_reader(evaluator, args, true)
    }
    FormulaFunctionId::ChisqDotTest if args.len() == 2 => {
      evaluate_chisq_test_reader(evaluator, args)
    }
    FormulaFunctionId::FDotTest if args.len() == 2 => evaluate_f_test_reader(evaluator, args),
    FormulaFunctionId::TDotTest if args.len() == 4 => evaluate_t_test_reader(evaluator, args),
    FormulaFunctionId::ZDotTest if (2..=3).contains(&args.len()) => {
      evaluate_z_test_reader(evaluator, args)
    }
    FormulaFunctionId::Frequency if args.len() == 2 => evaluate_frequency_reader(evaluator, args),
    FormulaFunctionId::HypgeomDotDist if (4..=5).contains(&args.len()) => {
      evaluate_hypgeom_dist_reader(evaluator, args)
    }
    FormulaFunctionId::LognormDotInv if args.len() == 3 => {
      evaluate_lognorm_inv_reader(evaluator, args)
    }
    FormulaFunctionId::NormDotInv if args.len() == 3 => {
      evaluate_norm_inv_reader(evaluator, args, false)
    }
    FormulaFunctionId::NormDotSDotInv if args.len() == 1 => {
      evaluate_norm_inv_reader(evaluator, args, true)
    }
    FormulaFunctionId::Fourier if (2..=5).contains(&args.len()) => {
      evaluator.evaluate_fourier_reader(args)
    }
    FormulaFunctionId::Covar | FormulaFunctionId::CovarianceDotP if args.len() == 2 => {
      evaluate_covariance_reader(evaluator, args, false)
    }
    FormulaFunctionId::CovarianceDotS if args.len() == 2 => {
      evaluate_covariance_reader(evaluator, args, true)
    }
    FormulaFunctionId::Covar
    | FormulaFunctionId::CovarianceDotP
    | FormulaFunctionId::CovarianceDotS => Some(FormulaValue::Error(FormulaErrorValue::Unknown)),
    FormulaFunctionId::Imdiv if args.len() == 2 => evaluate_complex_div_reader(evaluator, args),
    FormulaFunctionId::Cumipmt if args.len() == 6 => {
      evaluator.evaluate_cum_interest_principal_reader(args, true)
    }
    FormulaFunctionId::Cumprinc if args.len() == 6 => {
      evaluator.evaluate_cum_interest_principal_reader(args, false)
    }
    FormulaFunctionId::Fvschedule if args.len() == 2 => evaluate_fvschedule_reader(evaluator, args),
    FormulaFunctionId::Dollarde if args.len() == 2 => {
      evaluate_dollar_decimal_reader(evaluator, args, true)
    }
    FormulaFunctionId::Dollarfr if args.len() == 2 => {
      evaluate_dollar_decimal_reader(evaluator, args, false)
    }
    FormulaFunctionId::Delta if (1..=2).contains(&args.len()) => {
      evaluate_delta_reader(evaluator, args)
    }
    FormulaFunctionId::Delta => Some(FormulaValue::Error(FormulaErrorValue::Value)),
    FormulaFunctionId::Euroconvert if (3..=5).contains(&args.len()) => {
      evaluator.evaluate_euroconvert_reader(args)
    }
    FormulaFunctionId::Pmt if (3..=5).contains(&args.len()) => evaluate_pmt_reader(evaluator, args),
    FormulaFunctionId::Pv if (3..=5).contains(&args.len()) => evaluate_pv_reader(evaluator, args),
    FormulaFunctionId::Npv if args.len() >= 2 => evaluate_npv_reader(evaluator, args),
    FormulaFunctionId::Ispmt if args.len() == 4 => evaluate_ispmt_reader(evaluator, args),
    FormulaFunctionId::Rri if args.len() == 3 => evaluate_rri_reader(evaluator, args),
    FormulaFunctionId::Price if (6..=7).contains(&args.len()) => {
      evaluate_price_reader(evaluator, args)
    }
    FormulaFunctionId::Mduration if (5..=6).contains(&args.len()) => {
      evaluate_mduration_reader(evaluator, args)
    }
    FormulaFunctionId::Yield if (0..args.len()).any(|index| args.is_missing(index)) => {
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    }
    FormulaFunctionId::Randarray if args.len() <= 5 => evaluate_randarray_reader(evaluator, args),
    FormulaFunctionId::Subtotal if args.len() >= 2 => evaluator.evaluate_subtotal_reader(args),
    FormulaFunctionId::Aggregate if args.len() >= 3 => evaluator.evaluate_aggregate_reader(args),
    FormulaFunctionId::Dsum => {
      evaluator.evaluate_database_function_reader(args, DatabaseFunction::Sum)
    }
    FormulaFunctionId::Dcount => {
      evaluator.evaluate_database_function_reader(args, DatabaseFunction::Count)
    }
    FormulaFunctionId::Dcounta => {
      evaluator.evaluate_database_function_reader(args, DatabaseFunction::CountA)
    }
    FormulaFunctionId::Daverage => {
      evaluator.evaluate_database_function_reader(args, DatabaseFunction::Average)
    }
    FormulaFunctionId::Dget => {
      evaluator.evaluate_database_function_reader(args, DatabaseFunction::Get)
    }
    FormulaFunctionId::Dmax => {
      evaluator.evaluate_database_function_reader(args, DatabaseFunction::Max)
    }
    FormulaFunctionId::Dmin => {
      evaluator.evaluate_database_function_reader(args, DatabaseFunction::Min)
    }
    FormulaFunctionId::Dproduct => {
      evaluator.evaluate_database_function_reader(args, DatabaseFunction::Product)
    }
    FormulaFunctionId::Dvar => {
      evaluator.evaluate_database_function_reader(args, DatabaseFunction::Var)
    }
    FormulaFunctionId::Dvarp => {
      evaluator.evaluate_database_function_reader(args, DatabaseFunction::VarP)
    }
    FormulaFunctionId::Dstdev => {
      evaluator.evaluate_database_function_reader(args, DatabaseFunction::StdDev)
    }
    FormulaFunctionId::Dstdevp => {
      evaluator.evaluate_database_function_reader(args, DatabaseFunction::StdDevP)
    }
    FormulaFunctionId::Not if args.len() == 1 => {
      let value = args.value(0)?;
      if evaluator.array_context
        && matches!(
          value,
          FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
        )
      {
        return evaluator.map_unary_values(value, |evaluator, value| {
          Some(FormulaValue::Boolean(!evaluator.truthy(value)))
        });
      }
      let value = evaluator.scalar_value(value);
      match value {
        FormulaValue::Error(error) => Some(FormulaValue::Error(error)),
        value => Some(FormulaValue::Boolean(!evaluator.truthy(&value))),
      }
    }
    FormulaFunctionId::And if args.is_empty() => {
      Some(FormulaValue::Error(FormulaErrorValue::Parameter))
    }
    FormulaFunctionId::And => {
      evaluate_logical_aggregate_reader(evaluator, args, LogicalAggregate::And)
    }
    FormulaFunctionId::Or if args.is_empty() => {
      Some(FormulaValue::Error(FormulaErrorValue::Parameter))
    }
    FormulaFunctionId::Or => {
      evaluate_logical_aggregate_reader(evaluator, args, LogicalAggregate::Or)
    }
    FormulaFunctionId::Xor if args.is_empty() => {
      Some(FormulaValue::Error(FormulaErrorValue::Parameter))
    }
    FormulaFunctionId::Xor => {
      evaluate_logical_aggregate_reader(evaluator, args, LogicalAggregate::Xor)
    }
    FormulaFunctionId::True if args.is_empty() => Some(FormulaValue::Boolean(true)),
    FormulaFunctionId::False if args.is_empty() => Some(FormulaValue::Boolean(false)),
    FormulaFunctionId::Na if args.is_empty() => Some(FormulaValue::Error(FormulaErrorValue::NA)),
    FormulaFunctionId::Pi if args.is_empty() => Some(FormulaValue::Number(std::f64::consts::PI)),
    FormulaFunctionId::Pi => Some(FormulaValue::Error(FormulaErrorValue::Unknown)),
    FormulaFunctionId::Style if args.is_empty() => Some(FormulaValue::Number(0.0)),
    _ => None,
  }
}

fn scalar_number_arg<'doc>(
  _evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
) -> Option<f64> {
  args.scalar_number(index)
}

fn evaluate_information_error_value<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
  matches_error: impl Fn(FormulaErrorValue) -> bool + Copy,
) -> Option<FormulaValue<'doc>> {
  if evaluator.array_context
    && matches!(
      value,
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
    )
  {
    return evaluator.map_unary_values(value.clone(), |_, value| {
      Some(FormulaValue::Boolean(matches!(
        value,
        FormulaValue::Error(error) if matches_error(*error)
      )))
    });
  }
  Some(FormulaValue::Boolean(matches!(
    evaluator.first_value(value),
    FormulaValue::Error(error) if matches_error(error)
  )))
}

fn evaluate_isblank_value<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
) -> Option<FormulaValue<'doc>> {
  if evaluator.array_context
    && let FormulaValue::Reference(reference) = value
  {
    if reference.range.cell_count_hint() > crate::model::MAX_EXPANDED_RANGE_CELLS {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let sheet = evaluator.range_sheet(reference);
    let start_row = reference.range.start.row.min(reference.range.end.row);
    let end_row = reference.range.start.row.max(reference.range.end.row);
    let start_column = reference.range.start.column.min(reference.range.end.column);
    let end_column = reference.range.start.column.max(reference.range.end.column);
    let mut rows = Vec::new();
    for row in start_row..=end_row {
      let mut result_row = Vec::new();
      for column in start_column..=end_column {
        let address = CellAddress { column, row };
        result_row.push(FormulaValue::Boolean(
          !evaluator.book.formulas.contains_key(&(sheet, address))
            && matches!(
              evaluator.book.cell_value(sheet, address),
              FormulaValue::Blank
            ),
        ));
      }
      rows.push(result_row);
    }
    return Some(FormulaValue::Matrix(rows));
  }
  if evaluator.array_context
    && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
  {
    return Some(FormulaValue::Matrix(
      evaluator
        .matrix_values(value)
        .into_iter()
        .map(|row| {
          row
            .into_iter()
            .map(|value| FormulaValue::Boolean(matches!(value, FormulaValue::Blank)))
            .collect()
        })
        .collect(),
    ));
  }
  if let FormulaValue::Reference(reference) = value
    && reference.range.cell_count_hint() == 1
  {
    let sheet = evaluator.range_sheet(reference);
    if evaluator
      .book
      .formulas
      .contains_key(&(sheet, reference.range.start))
    {
      return Some(FormulaValue::Boolean(false));
    }
  }
  Some(FormulaValue::Boolean(matches!(
    evaluator.first_value(value),
    FormulaValue::Blank
  )))
}

fn evaluate_isformula_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let ranges = args.reference_ranges(0)?;
  let Some(reference) = ranges.first() else {
    return Some(FormulaValue::Boolean(false));
  };
  if ranges.len() != 1 || (!evaluator.array_context && reference.range.cell_count_hint() != 1) {
    return Some(FormulaValue::Boolean(false));
  }
  let sheet = evaluator.range_sheet(reference);
  Some(FormulaValue::Boolean(
    evaluator
      .book
      .formulas
      .contains_key(&(sheet, reference.range.start)),
  ))
}

fn evaluate_isnumber_value<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
) -> Option<FormulaValue<'doc>> {
  if evaluator.array_context
    && matches!(
      value,
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
    )
  {
    return evaluator.map_unary_values(value.clone(), |_, value| {
      Some(FormulaValue::Boolean(matches!(
        value,
        FormulaValue::Number(_) | FormulaValue::Boolean(_)
      )))
    });
  }
  Some(FormulaValue::Boolean(matches!(
    evaluator.information_scalar_value(value.clone()),
    Some(FormulaValue::Number(_) | FormulaValue::Boolean(_))
  )))
}

fn evaluate_error_type_value<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
) -> Option<FormulaValue<'doc>> {
  let Some(FormulaValue::Error(error)) = evaluator.first_error_value(value) else {
    return Some(FormulaValue::Error(FormulaErrorValue::NA));
  };
  let code = match error {
    FormulaErrorValue::Null => 1.0,
    FormulaErrorValue::Div0 => 2.0,
    FormulaErrorValue::Value => 3.0,
    FormulaErrorValue::Ref => 4.0,
    FormulaErrorValue::Name => 5.0,
    FormulaErrorValue::Num => 6.0,
    FormulaErrorValue::NA => 7.0,
    FormulaErrorValue::GettingData
    | FormulaErrorValue::Spill
    | FormulaErrorValue::Calc
    | FormulaErrorValue::IllegalArgument
    | FormulaErrorValue::Parameter => return Some(FormulaValue::Error(FormulaErrorValue::NA)),
    FormulaErrorValue::Unknown => return Some(FormulaValue::Error(FormulaErrorValue::Unknown)),
  };
  Some(FormulaValue::Number(code))
}

fn evaluate_error_type_raw_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let direct_unknown_error = args.raw_arg(0).is_some_and(|arg| {
    arg.range.end == arg.range.start + 1
      && matches!(
        arg.ops.get(arg.range.start),
        Some(FormulaOp::PushError(FormulaErrorValue::Unknown))
      )
  });
  let value = args.first_value()?;
  if let FormulaValue::Reference(reference) = &value
    && reference.range.cell_count_hint() == 1
    && evaluator.first_error_value(&value).is_none()
  {
    return Some(FormulaValue::Error(FormulaErrorValue::NA));
  }
  if matches!(
    value,
    FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
  ) && evaluator.first_error_value(&value).is_none()
  {
    return Some(FormulaValue::Number(519.0));
  }
  let Some(FormulaValue::Error(error)) = evaluator.first_error_value(&value) else {
    return Some(FormulaValue::Error(FormulaErrorValue::NA));
  };
  if error == FormulaErrorValue::Unknown && direct_unknown_error {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  }
  Some(FormulaValue::Number(match error {
    FormulaErrorValue::Null => 521.0,
    FormulaErrorValue::Div0 => 532.0,
    FormulaErrorValue::Value => 519.0,
    FormulaErrorValue::Ref => 524.0,
    FormulaErrorValue::Name => 525.0,
    FormulaErrorValue::Num => 503.0,
    FormulaErrorValue::NA => 32767.0,
    FormulaErrorValue::Spill => 541.0,
    FormulaErrorValue::IllegalArgument => 502.0,
    FormulaErrorValue::Parameter => 511.0,
    FormulaErrorValue::Unknown => 508.0,
    FormulaErrorValue::GettingData | FormulaErrorValue::Calc => 515.0,
  }))
}

fn evaluate_encodeurl_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let text = evaluator.text(&args.first_value()?);
  if text.is_empty() {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let mut output = String::with_capacity(text.len());
  for byte in text.bytes() {
    if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_') {
      output.push(byte as char);
    } else {
      output.push_str(&format!("%{byte:02X}"));
    }
  }
  Some(FormulaValue::String(Cow::Owned(output)))
}

fn evaluate_hyperlink_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let url = match evaluator.scalar_value(args.first_value()?) {
    FormulaValue::Error(error) => {
      return Some(FormulaValue::Matrix(vec![
        vec![FormulaValue::Error(error)],
        vec![FormulaValue::Error(error)],
      ]));
    }
    value => evaluator.text(&value),
  };
  let display = if args.len() == 2 {
    match evaluator.scalar_value(args.value(1)?) {
      FormulaValue::Error(error) => FormulaValue::Error(error),
      FormulaValue::Number(value) => FormulaValue::Number(value),
      FormulaValue::String(value) => FormulaValue::String(value),
      FormulaValue::Boolean(value) => FormulaValue::Number(if value { 1.0 } else { 0.0 }),
      FormulaValue::Blank => FormulaValue::Number(0.0),
      value => FormulaValue::String(Cow::Owned(evaluator.text(&value))),
    }
  } else {
    FormulaValue::String(Cow::Owned(url.clone()))
  };
  if let FormulaValue::Error(error) = display {
    return Some(FormulaValue::Matrix(vec![
      vec![FormulaValue::Error(error)],
      vec![FormulaValue::String(Cow::Owned(url))],
    ]));
  }
  Some(FormulaValue::Matrix(vec![
    vec![display],
    vec![FormulaValue::String(Cow::Owned(url))],
  ]))
}

fn evaluate_fixed_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value_arg = args.first_value()?;
  if !evaluator.array_context && is_multicell_scalar_argument_value(&value_arg) {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let Some(value) = evaluator
    .number(&value_arg)
    .or_else(|| crate::parser::grouped_formula_number(&evaluator.text(&value_arg)))
  else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  let digits = match args.value(1).and_then(|value| evaluator.number(&value)) {
    Some(digits) => match floor_to_i32(approx_floor(digits)) {
      Some(digits) => digits,
      None => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    },
    None => 2,
  };
  if !(-15..=15).contains(&digits) {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let no_commas = args.value(2).is_some_and(|value| evaluator.truthy(&value));
  let rounded = round_to_decimal_places(value, digits);
  let text = if digits >= 0 {
    format!("{rounded:.digits$}", digits = digits as usize)
  } else {
    format!("{rounded:.0}")
  };
  Some(FormulaValue::String(Cow::Owned(if no_commas {
    text
  } else {
    add_group_separators(&text)
  })))
}

fn is_multicell_scalar_argument_value(value: &FormulaValue<'_>) -> bool {
  match value {
    FormulaValue::Reference(reference) => reference.range.cell_count_hint() != 1,
    FormulaValue::Matrix(rows) => rows.len() != 1 || rows.first().is_some_and(|row| row.len() != 1),
    FormulaValue::RefList(ranges) => ranges.len() != 1 || ranges[0].range.cell_count_hint() != 1,
    _ => false,
  }
}

fn add_group_separators(value: &str) -> String {
  let (sign, body) = value
    .strip_prefix('-')
    .map_or(("", value), |body| ("-", body));
  let (integer, fraction) = body.split_once('.').unwrap_or((body, ""));
  let mut grouped = String::new();
  for (index, ch) in integer.chars().rev().enumerate() {
    if index != 0 && index.is_multiple_of(3) {
      grouped.push(',');
    }
    grouped.push(ch);
  }
  let integer = grouped.chars().rev().collect::<String>();
  if fraction.is_empty() {
    format!("{sign}{integer}")
  } else {
    format!("{sign}{integer}.{fraction}")
  }
}

fn evaluate_type_value<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
) -> Option<FormulaValue<'doc>> {
  if matches!(value, FormulaValue::Matrix(_) | FormulaValue::RefList(_)) {
    return Some(FormulaValue::Number(64.0));
  }
  Some(FormulaValue::Number(match evaluator.first_value(value) {
    FormulaValue::Number(_) => 1.0,
    FormulaValue::String(_) => 2.0,
    FormulaValue::Boolean(_) => 1.0,
    FormulaValue::Error(_) => 16.0,
    FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => 64.0,
    FormulaValue::Blank => 1.0,
  }))
}

fn evaluate_n_value<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
) -> Option<FormulaValue<'doc>> {
  if evaluator.array_context
    && matches!(
      value,
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
    )
  {
    return evaluator.map_unary_values(value.clone(), |_, value| match value {
      FormulaValue::Number(value) => Some(FormulaValue::Number(*value)),
      FormulaValue::Boolean(value) => Some(FormulaValue::Number(if *value { 1.0 } else { 0.0 })),
      FormulaValue::Error(error) => Some(FormulaValue::Error(*error)),
      _ => Some(FormulaValue::Number(0.0)),
    });
  }
  match value {
    FormulaValue::Reference(reference) if reference.range.cell_count_hint() != 1 => {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    FormulaValue::RefList(_) => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    _ => {}
  }
  Some(FormulaValue::Number(match evaluator.first_value(value) {
    FormulaValue::Number(value) => value,
    FormulaValue::Boolean(true) => 1.0,
    FormulaValue::Boolean(false) => 0.0,
    FormulaValue::Error(error) => return Some(FormulaValue::Error(error)),
    _ => 0.0,
  }))
}

fn evaluate_mode_reader<'doc>(args: FunctionArgReader<'_, '_, 'doc>) -> Option<FormulaValue<'doc>> {
  let mut values = Vec::new();
  for index in 0..args.len() {
    values.extend(args.value_numbers(index)?);
  }
  mode_slice(&values)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
}

fn evaluate_areas_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  if args.is_empty() {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  }
  let mut ranges = Vec::new();
  for index in 0..args.len() {
    ranges.extend(args.reference_ranges(index)?);
  }
  if !ranges.is_empty() {
    return Some(FormulaValue::Number(ranges.len() as f64));
  }
  Some(match args.first_value()? {
    FormulaValue::Matrix(_) => FormulaValue::Number(1.0),
    value if !evaluator.reference_ranges_from_value(&value).is_empty() => {
      FormulaValue::Number(evaluator.reference_ranges_from_value(&value).len() as f64)
    }
    _ => FormulaValue::Error(FormulaErrorValue::Value),
  })
}

fn evaluate_row_column_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  column: bool,
) -> Option<FormulaValue<'doc>> {
  let reference = if args.is_empty() {
    None
  } else {
    let Some(reference) = single_raw_reference(args, 0).or_else(|| {
      let value = args.first_value()?;
      evaluator.as_reference(&value)
    }) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    Some(reference)
  };
  let address = reference
    .as_ref()
    .map(|reference| reference.range.start)
    .unwrap_or_else(|| evaluator.current_cell.unwrap_or_default());
  if let Some(reference) = reference {
    let range = reference.range;
    let start_column = range.start.column.min(range.end.column);
    let end_column = range.start.column.max(range.end.column);
    let start_row = range.start.row.min(range.end.row);
    let end_row = range.start.row.max(range.end.row);
    if column && end_column > start_column {
      return Some(FormulaValue::Matrix(vec![
        (start_column..=end_column)
          .map(|column| FormulaValue::Number(column as f64 + 1.0))
          .collect(),
      ]));
    }
    if !column && end_row > start_row {
      return Some(FormulaValue::Matrix(
        (start_row..=end_row)
          .map(|row| vec![FormulaValue::Number(row as f64 + 1.0)])
          .collect(),
      ));
    }
  }
  Some(FormulaValue::Number(if column {
    address.column as f64 + 1.0
  } else {
    address.row as f64 + 1.0
  }))
}

fn evaluate_rows_columns_reader<'doc>(
  args: FunctionArgReader<'_, '_, 'doc>,
  columns: bool,
) -> Option<FormulaValue<'doc>> {
  if args.is_empty() {
    return Some(FormulaValue::Number(0.0));
  }
  Some(match args.first_value()? {
    FormulaValue::Reference(reference) => {
      let range = reference.range;
      FormulaValue::Number(if columns {
        range.start.column.abs_diff(range.end.column) as f64 + 1.0
      } else {
        range.start.row.abs_diff(range.end.row) as f64 + 1.0
      })
    }
    FormulaValue::Matrix(rows) => FormulaValue::Number(if columns {
      rows.first().map_or(0, Vec::len) as f64
    } else {
      rows.len() as f64
    }),
    _ => FormulaValue::Number(1.0),
  })
}

fn evaluate_len_value<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
  bytes: bool,
) -> Option<FormulaValue<'doc>> {
  if evaluator.array_context
    && matches!(
      value,
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
    )
  {
    return None;
  }
  let text = evaluator.text(value);
  Some(FormulaValue::Number(if bytes {
    text_byte_len(&text) as f64
  } else {
    text.chars().count() as f64
  }))
}

fn evaluate_info_value<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
) -> Option<FormulaValue<'doc>> {
  let key = evaluator.text(value).to_ascii_uppercase();
  match key.as_str() {
    "SYSTEM" => Some(FormulaValue::String(Cow::Borrowed("LINUX"))),
    "OSVERSION" | "RELEASE" => Some(FormulaValue::String(Cow::Borrowed(""))),
    "NUMFILE" => Some(FormulaValue::Number(1.0)),
    "RECALC" => Some(FormulaValue::String(Cow::Borrowed("Automatic"))),
    "DIRECTORY" | "MEMAVAIL" | "MEMUSED" | "ORIGIN" | "TOTMEM" => {
      Some(FormulaValue::Error(FormulaErrorValue::NA))
    }
    _ => Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
  }
}

fn evaluate_t_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = args.first_value()?;
  let value = match value {
    FormulaValue::Reference(reference) => evaluator.scalar_reference_value(&reference),
    FormulaValue::Matrix(rows) => rows
      .into_iter()
      .next()
      .and_then(|row| row.into_iter().next())
      .unwrap_or_default(),
    value => value,
  };
  match value {
    FormulaValue::String(text) => Some(FormulaValue::String(text)),
    FormulaValue::Error(error) => Some(FormulaValue::Error(error)),
    _ => Some(FormulaValue::String(Cow::Borrowed(""))),
  }
}

fn evaluate_value_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = args.first_value()?;
  if evaluator.array_context
    && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
  {
    return Some(FormulaValue::Matrix(
      evaluator
        .matrix_values(&value)
        .into_iter()
        .map(|row| {
          row
            .into_iter()
            .map(|value| evaluator.value_from_formatter_text(&value))
            .collect()
        })
        .collect(),
    ));
  }
  let value = match value {
    FormulaValue::Reference(reference) => {
      if reference.range.cell_count_hint() != 1 {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      evaluator.scalar_reference_value(&reference)
    }
    value => value,
  };
  Some(evaluator.value_from_formatter_text(&value))
}

fn evaluate_numbervalue_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let mut text = evaluator.text(&args.first_value()?);
  let decimal = if args.len() >= 2 {
    match args.value(1)? {
      FormulaValue::Blank => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
      value => {
        let text = evaluator.text(&value);
        if text.chars().count() != 1 {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        }
        text
      }
    }
  } else {
    String::new()
  };
  let group = if args.len() >= 3 {
    evaluator.text(&args.value(2)?)
  } else {
    String::new()
  };
  if !decimal.is_empty() && group.contains(&decimal) {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let percent_count = text.chars().rev().take_while(|ch| *ch == '%').count();
  text.truncate(text.len() - percent_count);
  text = text
    .chars()
    .filter(|ch| !ch.is_whitespace() && !group.contains(*ch))
    .collect();
  if text.is_empty() {
    return Some(FormulaValue::Number(0.0));
  }
  if decimal.is_empty() && text.contains('.') {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  if decimal != "." && !decimal.is_empty() {
    text = text.replace(&decimal, ".");
  }
  text
    .trim()
    .parse::<f64>()
    .map(|value| value / 100_f64.powi(percent_count as i32))
    .map(FormulaValue::Number)
    .ok()
    .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
}

fn evaluate_convert_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = evaluator.number(&args.value(0)?)?;
  let from = evaluator.text(&args.value(1)?);
  let to = evaluator.text(&args.value(2)?);
  Some(match convert_unit(value, &from, &to) {
    Ok(value) => FormulaValue::Number(value),
    Err(_) => FormulaValue::Error(FormulaErrorValue::IllegalArgument),
  })
}

fn evaluate_trim_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = args.first_value()?;
  if evaluator.array_context
    && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
  {
    return Some(FormulaValue::Matrix(
      evaluator
        .matrix_values(&value)
        .into_iter()
        .map(|row| {
          row
            .into_iter()
            .map(|value| {
              FormulaValue::String(Cow::Owned(trim_formula_text(&display_text_from_value(
                &value,
              ))))
            })
            .collect()
        })
        .collect(),
    ));
  }
  if is_multicell_scalar_argument(&value) {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  Some(FormulaValue::String(Cow::Owned(trim_formula_text(
    &evaluator.text(&value),
  ))))
}

fn evaluate_clean_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = args.first_value()?;
  if evaluator.array_context
    && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
  {
    return Some(FormulaValue::Matrix(
      evaluator
        .matrix_values(&value)
        .into_iter()
        .map(|row| {
          row
            .into_iter()
            .map(|value| {
              FormulaValue::String(Cow::Owned(clean_formula_text(&display_text_from_value(
                &value,
              ))))
            })
            .collect()
        })
        .collect(),
    ));
  }
  if is_multicell_scalar_argument(&value) {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  Some(FormulaValue::String(Cow::Owned(clean_formula_text(
    &evaluator.text(&value),
  ))))
}

fn evaluate_concat_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let mut output = String::new();
  for index in 0..args.len() {
    if let Err(error) = concat_value_text(evaluator, &args.value(index)?, &mut output) {
      return Some(FormulaValue::Error(error));
    }
  }
  Some(FormulaValue::String(Cow::Owned(output)))
}

fn evaluate_textjoin_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  if args.len() < 3 {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let delimiters = match textjoin_delimiters(evaluator, &args.value(0)?) {
    Ok(delimiters) => delimiters,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let ignore_empty_value = evaluator.scalar_value(args.value(1)?);
  if let FormulaValue::Error(error) = ignore_empty_value {
    return Some(FormulaValue::Error(error));
  }
  let ignore_empty = evaluator.truthy(&ignore_empty_value);
  let mut output = String::new();
  let mut value_count = 0usize;
  for index in 2..args.len() {
    let value = args.value(index)?;
    if let Err(error) = textjoin_value_text(
      evaluator,
      &value,
      &delimiters,
      ignore_empty,
      &mut output,
      &mut value_count,
    ) {
      return Some(FormulaValue::Error(error));
    }
  }
  Some(FormulaValue::String(Cow::Owned(output)))
}

fn textjoin_delimiters<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
) -> Result<Vec<String>, FormulaErrorValue> {
  let mut delimiters = Vec::new();
  for value in evaluator.matrix_values(value).into_iter().flatten() {
    match value {
      FormulaValue::Error(error) => return Err(error),
      value => delimiters.push(evaluator.text(&value)),
    }
  }
  if delimiters.is_empty() {
    delimiters.push(String::new());
  }
  Ok(delimiters)
}

fn textjoin_value_text<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
  delimiters: &[String],
  ignore_empty: bool,
  output: &mut String,
  value_count: &mut usize,
) -> Result<(), FormulaErrorValue> {
  for value in evaluator.matrix_values(value).into_iter().flatten() {
    match value {
      FormulaValue::Error(error) => return Err(error),
      FormulaValue::Blank => {
        if ignore_empty {
          continue;
        }
        textjoin_push_value("", delimiters, output, value_count);
      }
      FormulaValue::String(text) if text.is_empty() && ignore_empty => {}
      value => {
        let text = evaluator.text(&value);
        textjoin_push_value(&text, delimiters, output, value_count);
      }
    }
  }
  Ok(())
}

fn textjoin_push_value(
  text: &str,
  delimiters: &[String],
  output: &mut String,
  value_count: &mut usize,
) {
  if *value_count > 0 {
    output.push_str(&delimiters[(*value_count - 1) % delimiters.len()]);
  }
  output.push_str(text);
  *value_count += 1;
}

fn concat_value_text<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
  output: &mut String,
) -> Result<(), FormulaErrorValue> {
  match value {
    FormulaValue::Reference(reference) => {
      for value in evaluator.range_values(reference) {
        concat_value_text(evaluator, &value, output)?;
      }
    }
    FormulaValue::RefList(ranges) => {
      for range in ranges {
        for value in evaluator.range_values(range) {
          concat_value_text(evaluator, &value, output)?;
        }
      }
    }
    FormulaValue::Matrix(rows) => {
      for value in rows.iter().flatten() {
        concat_value_text(evaluator, value, output)?;
      }
    }
    FormulaValue::Blank => {}
    FormulaValue::Error(error) => return Err(*error),
    value => output.push_str(&evaluator.text(value)),
  }
  Ok(())
}

fn evaluate_exact_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let left = args.value(0)?;
  let right = args.value(1)?;
  if evaluator.array_context && (is_matrix_argument(&left) || is_matrix_argument(&right)) {
    return evaluator.map_binary_values(left, right, |evaluator, left, right| {
      Some(FormulaValue::Boolean(exact_text_match(
        &evaluator.text(left),
        &evaluator.text(right),
      )))
    });
  }
  Some(FormulaValue::Boolean(exact_text_match(
    &evaluator.text(&left),
    &evaluator.text(&right),
  )))
}

fn exact_text_match(left: &str, right: &str) -> bool {
  left == right
    || ((!left.is_ascii() || !right.is_ascii()) && left.to_lowercase() == right.to_lowercase())
}

fn evaluate_find_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  case_sensitive: bool,
  bytes: bool,
) -> Option<FormulaValue<'doc>> {
  let needle_value = args.array_value(0)?;
  let haystack_value = args.array_value(1)?;
  let start = args
    .raw_arg(2)
    .and_then(|_| args.value(2))
    .and_then(|value| evaluator.number(&value))
    .unwrap_or(1.0) as usize;
  evaluator.map_find_values(needle_value, haystack_value, start, case_sensitive, bytes)
}

fn evaluate_substitute_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let text = evaluator.text(&args.value(0)?);
  let old = evaluator.text(&args.value(1)?);
  let new = evaluator.text(&args.value(2)?);
  if old.is_empty() {
    return Some(FormulaValue::String(Cow::Owned(text)));
  }
  if let Some(instance) = args
    .raw_arg(3)
    .and_then(|_| args.value(3))
    .and_then(|value| evaluator.number(&value))
    .map(|value| value as usize)
  {
    if instance == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut result = String::new();
    let mut rest = text.as_str();
    let mut count = 0usize;
    while let Some(position) = rest.find(&old) {
      result.push_str(&rest[..position]);
      count += 1;
      if count == instance {
        result.push_str(&new);
      } else {
        result.push_str(&old);
      }
      rest = &rest[position + old.len()..];
    }
    result.push_str(rest);
    Some(FormulaValue::String(Cow::Owned(result)))
  } else {
    Some(FormulaValue::String(Cow::Owned(text.replace(&old, &new))))
  }
}

fn evaluate_sumsq_reader<'doc>(
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  match args.numeric_aggregate(true)? {
    Ok(values) => Some(FormulaValue::Number(
      values.into_iter().map(|value| value * value).sum(),
    )),
    Err(error) => Some(FormulaValue::Error(error)),
  }
}

fn evaluate_mina_maxa_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  max: bool,
) -> Option<FormulaValue<'doc>> {
  let mut values = Vec::new();
  for index in 0..args.len() {
    collect_a_numbers(evaluator, args.value(index)?, &mut values)?;
  }
  if values.is_empty() {
    return Some(FormulaValue::Number(0.0));
  }
  Some(FormulaValue::Number(if max {
    values.into_iter().reduce(f64::max).unwrap_or(0.0)
  } else {
    values.into_iter().reduce(f64::min).unwrap_or(0.0)
  }))
}

fn evaluate_averagea_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let mut values = Vec::new();
  for index in 0..args.len() {
    collect_a_numbers(evaluator, args.value(index)?, &mut values)?;
  }
  if values.is_empty() {
    return Some(FormulaValue::Error(FormulaErrorValue::Div0));
  }
  Some(FormulaValue::Number(
    values.iter().sum::<f64>() / values.len() as f64,
  ))
}

fn collect_a_numbers<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: FormulaValue<'doc>,
  values: &mut Vec<f64>,
) -> Option<()> {
  match value {
    FormulaValue::Error(_) => return None,
    FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_) => {
      for value in evaluator.matrix_values(&value).into_iter().flatten() {
        match value {
          FormulaValue::Number(value) => values.push(value),
          FormulaValue::Boolean(value) => values.push(if value { 1.0 } else { 0.0 }),
          FormulaValue::String(_) => values.push(0.0),
          FormulaValue::Blank => {}
          FormulaValue::Error(_) => return None,
          FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_) => {}
        }
      }
    }
    FormulaValue::Number(value) => values.push(value),
    FormulaValue::Boolean(value) => values.push(if value { 1.0 } else { 0.0 }),
    FormulaValue::String(_) | FormulaValue::Blank => values.push(0.0),
  }
  Some(())
}

fn evaluate_sumx2_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  plus: bool,
) -> Option<FormulaValue<'doc>> {
  evaluate_xy_matrix_reader(evaluator, args, |sum, left, right| {
    sum.add(left * left);
    sum.add(if plus {
      right * right
    } else {
      -(right * right)
    });
  })
}

fn evaluate_sumxmy2_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  evaluate_xy_matrix_reader(evaluator, args, |sum, left, right| {
    let difference = left - right;
    sum.add(difference * difference);
  })
}

fn evaluate_xy_matrix_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  mut accumulate: impl FnMut(&mut KahanSum, f64, f64),
) -> Option<FormulaValue<'doc>> {
  let left_value = args.array_value(0)?;
  let right_value = args.array_value(1)?;
  let left = evaluator.matrix_values(&left_value);
  let right = evaluator.matrix_values(&right_value);
  if left.len() != right.len()
    || left
      .iter()
      .zip(&right)
      .any(|(left_row, right_row)| left_row.len() != right_row.len())
  {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let mut sum = KahanSum::default();
  for (left_row, right_row) in left.iter().zip(&right) {
    for (left, right) in left_row.iter().zip(right_row) {
      match (left, right) {
        (FormulaValue::Error(error), _) | (_, FormulaValue::Error(error)) => {
          return Some(FormulaValue::Error(*error));
        }
        (FormulaValue::Blank | FormulaValue::String(_), _)
        | (_, FormulaValue::Blank | FormulaValue::String(_)) => {}
        _ => {
          let Some(left) = evaluator.number(left) else {
            return Some(FormulaValue::Error(FormulaErrorValue::Value));
          };
          let Some(right) = evaluator.number(right) else {
            return Some(FormulaValue::Error(FormulaErrorValue::Value));
          };
          accumulate(&mut sum, left, right);
        }
      }
    }
  }
  Some(FormulaValue::Number(sum.finish()))
}

fn evaluate_devsq_reader<'doc>(
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  match args.numeric_aggregate(true)? {
    Ok(values) => deviation_sum_squares(&values).map(FormulaValue::Number),
    Err(error) => Some(FormulaValue::Error(error)),
  }
}

fn evaluate_avedev_reader<'doc>(
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let values = match args.numeric_aggregate(true)? {
    Ok(values) => values,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  if values.is_empty() {
    return Some(FormulaValue::Error(FormulaErrorValue::Div0));
  }
  let mean = mean_number_slice(&values);
  Some(FormulaValue::Number(
    kahan_sum(values.iter().map(|value| (value - mean).abs())) / values.len() as f64,
  ))
}

fn evaluate_stdev_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  sample: bool,
  text_as_zero: bool,
) -> Option<FormulaValue<'doc>> {
  let values = if text_as_zero {
    let mut values = Vec::new();
    for index in 0..args.len() {
      collect_a_numbers(evaluator, args.value(index)?, &mut values)?;
    }
    values
  } else {
    match args.numeric_aggregate(true)? {
      Ok(values) => values,
      Err(error) => return Some(FormulaValue::Error(error)),
    }
  };
  variance_slice(&values, sample)
    .map(|value| FormulaValue::Number(value.sqrt()))
    .or(Some(FormulaValue::Error(FormulaErrorValue::Div0)))
}

fn evaluate_variance_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  sample: bool,
  text_as_zero: bool,
) -> Option<FormulaValue<'doc>> {
  let values = if text_as_zero {
    let mut values = Vec::new();
    for index in 0..args.len() {
      collect_a_numbers(evaluator, args.value(index)?, &mut values)?;
    }
    values
  } else {
    match args.numeric_aggregate(true)? {
      Ok(values) => values,
      Err(error) => return Some(FormulaValue::Error(error)),
    }
  };
  variance_slice(&values, sample)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::Div0)))
}

fn evaluate_kurt_reader<'doc>(args: FunctionArgReader<'_, '_, 'doc>) -> Option<FormulaValue<'doc>> {
  let values = match args.numeric_aggregate(true)? {
    Ok(values) => values,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  Some(
    kurtosis(&values)
      .map(FormulaValue::Number)
      .unwrap_or_else(|error| FormulaValue::Error(statistics_error_value(error))),
  )
}

fn evaluate_skew_reader<'doc>(
  args: FunctionArgReader<'_, '_, 'doc>,
  population: bool,
) -> Option<FormulaValue<'doc>> {
  let values = match args.numeric_aggregate(true)? {
    Ok(values) => values,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  Some(
    skewness(&values, population)
      .map(FormulaValue::Number)
      .unwrap_or_else(|error| FormulaValue::Error(statistics_error_value(error))),
  )
}

fn evaluate_large_small_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  large: bool,
) -> Option<FormulaValue<'doc>> {
  let mut values = evaluator.value_numbers(&args.array_value(0)?);
  if values.is_empty() {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  values.sort_by(f64::total_cmp);
  let ranks = evaluator.matrix_values(&args.array_value(1)?);
  let evaluate_rank = |rank: &FormulaValue<'doc>, values: &[f64]| -> FormulaValue<'doc> {
    let Some(rank) = evaluator.number(rank) else {
      return FormulaValue::Error(FormulaErrorValue::Value);
    };
    let rank = if large { rank.ceil() } else { rank.floor() };
    if rank < 1.0 {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    let index = rank as usize;
    if index == 0 || index > values.len() {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    let index = if large {
      values.len() - index
    } else {
      index - 1
    };
    FormulaValue::Number(values[index])
  };
  if ranks.len() == 1 && ranks.first().is_some_and(|row| row.len() == 1) {
    return ranks
      .first()
      .and_then(|row| row.first())
      .map(|rank| evaluate_rank(rank, &values));
  }
  Some(FormulaValue::Matrix(
    ranks
      .iter()
      .map(|row| {
        row
          .iter()
          .map(|rank| evaluate_rank(rank, &values))
          .collect()
      })
      .collect(),
  ))
}

fn evaluate_trimmean_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let values = evaluator.value_numbers(&args.value(0)?);
  let alpha = evaluator.number(&args.value(1)?)?;
  trim_mean(values, alpha)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
}

fn evaluate_percentile_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  kind: PercentileKind,
) -> Option<FormulaValue<'doc>> {
  let mut values = evaluator.value_numbers(&args.value(0)?);
  let k = evaluator.number(&args.value(1)?)?;
  percentile_sorted(&mut values, k, kind)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
}

fn evaluate_quartile_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  kind: PercentileKind,
) -> Option<FormulaValue<'doc>> {
  let mut values = evaluator.value_numbers(&args.value(0)?);
  let k = evaluator.number(&args.value(1)?)?.floor();
  percentile_sorted(&mut values, k / 4.0, kind)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
}

fn evaluate_percentrank_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  kind: PercentileKind,
) -> Option<FormulaValue<'doc>> {
  let values = evaluator.value_numbers(&args.value(0)?);
  let x = evaluator.number(&args.value(1)?)?;
  let significance = args
    .raw_arg(2)
    .and_then(|_| args.value(2))
    .and_then(|value| evaluator.number(&value))
    .map(approx_floor)
    .unwrap_or(3.0);
  if significance < 1.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  percent_rank(values, x, significance, kind)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
}

fn evaluate_rank_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  average: bool,
) -> Option<FormulaValue<'doc>> {
  let value = evaluator.number(&args.value(0)?)?;
  let values = evaluator.value_numbers(&args.value(1)?);
  let ascending = args
    .raw_arg(2)
    .and_then(|_| args.value(2))
    .and_then(|value| evaluator.number(&value))
    .is_some_and(|value| value != 0.0);
  rank_value(values, value, ascending, average)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
}

fn evaluate_sumproduct_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let matrices = (0..args.len())
    .map(|index| {
      let value = args.array_value(index)?;
      Some(evaluator.matrix_values(&value))
    })
    .collect::<Option<Vec<_>>>()?;
  let first = matrices.first()?;
  let rows = first.len();
  let columns = first.first().map_or(0, Vec::len);
  if rows == 0 || columns == 0 {
    return Some(FormulaValue::Number(0.0));
  }
  if matrices
    .iter()
    .any(|matrix| matrix.len() != rows || matrix.iter().any(|row| row.len() != columns))
  {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let mut total = 0.0;
  let mut compensation = 0.0;
  for row in 0..rows {
    for column in 0..columns {
      let mut product = Some(1.0);
      for matrix in &matrices {
        match &matrix[row][column] {
          FormulaValue::Number(value) => product = product.map(|current| current * value),
          FormulaValue::Boolean(value) => {
            product = product.map(|current| current * if *value { 1.0 } else { 0.0 });
          }
          FormulaValue::Error(error) => return Some(FormulaValue::Error(*error)),
          FormulaValue::Blank | FormulaValue::String(_) => product = None,
          FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => {}
        }
      }
      if let Some(value) = product {
        let y = value - compensation;
        let t = total + y;
        compensation = (t - total) - y;
        total = t;
      }
    }
  }
  Some(FormulaValue::Number(total))
}

fn evaluate_geomean_harmean_reader<'doc>(
  args: FunctionArgReader<'_, '_, 'doc>,
  geometric: bool,
) -> Option<FormulaValue<'doc>> {
  let values = match args.numeric_aggregate(true)? {
    Ok(values) => values,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  if values.is_empty() || values.iter().any(|value| *value <= 0.0) {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
  if geometric {
    Some(FormulaValue::Number(
      (values.iter().map(|value| value.ln()).sum::<f64>() / values.len() as f64).exp(),
    ))
  } else {
    Some(FormulaValue::Number(
      values.len() as f64 / values.iter().map(|value| 1.0 / value).sum::<f64>(),
    ))
  }
}

fn evaluate_filter_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let data = evaluator.matrix_values(&args.array_value(0)?);
  let include = evaluator.matrix_values(&args.array_value(1)?);
  if data.is_empty() || data.first().is_none_or(Vec::is_empty) {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let include_rows = include.len();
  let include_columns = include.first().map_or(0, Vec::len);
  if include_rows == 0 || include_columns == 0 || (include_rows > 1 && include_columns > 1) {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let include_values = include.into_iter().flatten().collect::<Vec<_>>();
  let mut result = Vec::new();
  if include_values.len() == data.len() {
    for (row, include) in data.into_iter().zip(include_values) {
      if evaluator.truthy(&include) {
        result.push(row);
      }
    }
  } else if include_values.len() == data.first()?.len() {
    let columns = include_values
      .iter()
      .enumerate()
      .filter_map(|(index, value)| evaluator.truthy(value).then_some(index))
      .collect::<Vec<_>>();
    for row in data {
      result.push(columns.iter().map(|column| row[*column].clone()).collect());
    }
  } else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  if result.is_empty() || result.first().is_some_and(Vec::is_empty) {
    return Some(
      args
        .raw_arg(2)
        .and_then(|_| args.value(2))
        .unwrap_or(FormulaValue::Error(FormulaErrorValue::Calc)),
    );
  }
  Some(FormulaValue::Matrix(result))
}

fn evaluate_stack_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  horizontal: bool,
) -> Option<FormulaValue<'doc>> {
  if (0..args.len()).any(|index| args.is_missing(index)) {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let matrices = (0..args.len())
    .map(|index| {
      args
        .array_value(index)
        .map(|value| evaluator.matrix_values(&value))
    })
    .collect::<Option<Vec<_>>>()?;
  if matrices
    .iter()
    .any(|matrix| matrix.is_empty() || matrix.first().is_none_or(Vec::is_empty))
  {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let rows = if horizontal {
    matrices.iter().map(Vec::len).max().unwrap_or(0)
  } else {
    matrices.iter().map(Vec::len).sum()
  };
  let columns = if horizontal {
    matrices
      .iter()
      .map(|matrix| matrix.first().map_or(0, Vec::len))
      .sum()
  } else {
    matrices
      .iter()
      .map(|matrix| matrix.first().map_or(0, Vec::len))
      .max()
      .unwrap_or(0)
  };
  let pad = FormulaValue::Error(FormulaErrorValue::NA);
  let mut result = Vec::with_capacity(rows);
  if horizontal {
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for matrix in &matrices {
        let width = matrix.first().map_or(0, Vec::len);
        for column in 0..width {
          result_row.push(
            matrix
              .get(row)
              .and_then(|source_row| source_row.get(column))
              .cloned()
              .unwrap_or_else(|| pad.clone()),
          );
        }
      }
      result.push(result_row);
    }
  } else {
    for matrix in &matrices {
      for source_row in matrix {
        let mut row = source_row.clone();
        row.resize(columns, pad.clone());
        result.push(row);
      }
    }
  }
  Some(FormulaValue::Matrix(result))
}

fn evaluate_sequence_reader<'doc>(
  _evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let optional_number = |index: usize, default: f64| {
    if args.raw_arg(index).is_none() || args.is_missing(index) {
      Some(default)
    } else {
      args.scalar_number(index)
    }
  };
  let rows = optional_number(0, 1.0)?.floor() as usize;
  let columns = optional_number(1, 1.0)?.floor() as usize;
  let start = optional_number(2, 1.0)?;
  let step = optional_number(3, 1.0)?;
  if rows == 0 || columns == 0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let mut next = start;
  let mut output = Vec::with_capacity(rows);
  for _ in 0..rows {
    let mut row = Vec::with_capacity(columns);
    for _ in 0..columns {
      row.push(FormulaValue::Number(next));
      next += step;
    }
    output.push(row);
  }
  Some(FormulaValue::Matrix(output))
}

fn evaluate_take_drop_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  take: bool,
) -> Option<FormulaValue<'doc>> {
  let matrix = evaluator.matrix_values(&args.array_value(0)?);
  let row_count = matrix.len();
  let col_count = matrix.first().map_or(0, Vec::len);
  if row_count == 0 || col_count == 0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let optional_index = |index: usize| {
    if args.raw_arg(index).is_none() || args.is_missing(index) {
      Some(None)
    } else {
      args.scalar_number(index).map(|value| Some(value as isize))
    }
  };
  let rows_arg = optional_index(1)?;
  let cols_arg = optional_index(2)?;
  let (row_start, row_end) = take_drop_bounds(row_count, rows_arg, take);
  let (col_start, col_end) = take_drop_bounds(col_count, cols_arg, take);
  if row_start >= row_end || col_start >= col_end {
    return Some(FormulaValue::Error(FormulaErrorValue::NA));
  }
  let result = matrix[row_start..row_end]
    .iter()
    .map(|row| row[col_start..col_end].to_vec())
    .collect::<Vec<_>>();
  Some(FormulaValue::Matrix(result))
}

fn evaluate_sort_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let matrix = evaluator.matrix_values(&args.array_value(0)?);
  let (rows, columns) = rectangular_shape(&matrix)?;
  let sort_indexes = optional_number_values(evaluator, args, 1, &[1.0])?;
  let sort_orders = optional_number_values(evaluator, args, 2, &[1.0])?;
  if sort_orders.len() != 1 && sort_orders.len() != sort_indexes.len() {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let by_column = args
    .raw_arg(3)
    .filter(|_| !args.is_missing(3))
    .and_then(|_| args.value(3))
    .is_some_and(|value| evaluator.truthy(&value));
  let key_limit = if by_column { rows } else { columns };
  let mut keys = Vec::with_capacity(sort_indexes.len());
  for (index, sort_index) in sort_indexes.iter().enumerate() {
    let sort_index = approx_floor(*sort_index);
    if sort_index < 1.0 || sort_index > key_limit as f64 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let order = sort_orders
      .get(index)
      .or_else(|| sort_orders.first())
      .copied()
      .unwrap_or(1.0);
    if order != 1.0 && order != -1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    keys.push((sort_index as usize - 1, order == 1.0));
  }
  Some(sort_matrix(evaluator, matrix, keys, by_column))
}

fn evaluate_sortby_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let matrix = evaluator.matrix_values(&args.array_value(0)?);
  let (rows, columns) = rectangular_shape(&matrix)?;
  let mut by_column = None;
  let mut keys = Vec::new();
  let mut index = 1;
  while index < args.len() {
    let key_matrix = evaluator.matrix_values(&args.array_value(index)?);
    let (key_rows, key_columns) = rectangular_shape(&key_matrix)?;
    let orientation = if key_columns == 1 && key_rows == rows {
      false
    } else if key_rows == 1 && key_columns == columns {
      true
    } else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let values = key_matrix.into_iter().flatten().collect::<Vec<_>>();
    if by_column.is_some_and(|current| current != orientation) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    by_column = Some(orientation);
    let order = args
      .raw_arg(index + 1)
      .filter(|_| !args.is_missing(index + 1))
      .and_then(|_| args.value(index + 1))
      .and_then(|value| evaluator.number(&value))
      .unwrap_or(1.0);
    if order != 1.0 && order != -1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    keys.push((values, order == 1.0));
    index += 2;
  }
  let by_column = by_column.unwrap_or(false);
  let mut order = if by_column {
    (0..columns).collect::<Vec<_>>()
  } else {
    (0..rows).collect::<Vec<_>>()
  };
  order.sort_by(|left, right| sort_multi_key_order(evaluator, &keys, *left, *right));
  Some(if by_column {
    FormulaValue::Matrix(reorder_columns(&matrix, &order))
  } else {
    FormulaValue::Matrix(order.into_iter().map(|row| matrix[row].clone()).collect())
  })
}

fn take_drop_bounds(len: usize, arg: Option<isize>, take: bool) -> (usize, usize) {
  let Some(arg) = arg else {
    return (0, len);
  };
  let abs = arg.unsigned_abs();
  if abs >= len {
    return (0, len);
  }
  if take {
    if arg < 0 { (len - abs, len) } else { (0, abs) }
  } else if arg < 0 {
    (0, len - abs)
  } else {
    (abs, len)
  }
}

fn rectangular_shape(matrix: &[Vec<FormulaValue<'_>>]) -> Option<(usize, usize)> {
  let rows = matrix.len();
  let columns = matrix.first().map_or(0, Vec::len);
  if rows == 0 || columns == 0 || matrix.iter().any(|row| row.len() != columns) {
    None
  } else {
    Some((rows, columns))
  }
}

fn optional_number_values<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
  default: &[f64],
) -> Option<Vec<f64>> {
  if args.raw_arg(index).is_none() || args.is_missing(index) {
    return Some(default.to_vec());
  }
  evaluator
    .matrix_values(&args.array_value(index)?)
    .into_iter()
    .flatten()
    .map(|value| evaluator.number(&value))
    .collect()
}

fn sort_matrix<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  matrix: Vec<Vec<FormulaValue<'doc>>>,
  keys: Vec<(usize, bool)>,
  by_column: bool,
) -> FormulaValue<'doc> {
  let (rows, columns) = rectangular_shape(&matrix).unwrap_or((0, 0));
  if by_column {
    let mut order = (0..columns).collect::<Vec<_>>();
    order.sort_by(|left, right| {
      for (row, ascending) in &keys {
        let ordering = sort_value_order(evaluator, &matrix[*row][*left], &matrix[*row][*right]);
        if ordering != std::cmp::Ordering::Equal {
          return if *ascending {
            ordering
          } else {
            ordering.reverse()
          };
        }
      }
      left.cmp(right)
    });
    FormulaValue::Matrix(reorder_columns(&matrix, &order))
  } else {
    let mut order = (0..rows).collect::<Vec<_>>();
    order.sort_by(|left, right| {
      for (column, ascending) in &keys {
        let ordering =
          sort_value_order(evaluator, &matrix[*left][*column], &matrix[*right][*column]);
        if ordering != std::cmp::Ordering::Equal {
          return if *ascending {
            ordering
          } else {
            ordering.reverse()
          };
        }
      }
      left.cmp(right)
    });
    FormulaValue::Matrix(order.into_iter().map(|row| matrix[row].clone()).collect())
  }
}

fn reorder_columns<'doc>(
  matrix: &[Vec<FormulaValue<'doc>>],
  order: &[usize],
) -> Vec<Vec<FormulaValue<'doc>>> {
  matrix
    .iter()
    .map(|row| {
      order
        .iter()
        .filter_map(|column| row.get(*column).cloned())
        .collect()
    })
    .collect()
}

fn sort_multi_key_order<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  keys: &[(Vec<FormulaValue<'doc>>, bool)],
  left: usize,
  right: usize,
) -> std::cmp::Ordering {
  for (values, ascending) in keys {
    let ordering = sort_value_order(evaluator, &values[left], &values[right]);
    if ordering != std::cmp::Ordering::Equal {
      return if *ascending {
        ordering
      } else {
        ordering.reverse()
      };
    }
  }
  left.cmp(&right)
}

fn sort_value_order(
  evaluator: &EvalContext<'_, '_>,
  left: &FormulaValue<'_>,
  right: &FormulaValue<'_>,
) -> std::cmp::Ordering {
  match (left, right) {
    (FormulaValue::Number(left), FormulaValue::Number(right)) => left.total_cmp(right),
    (FormulaValue::String(left), FormulaValue::String(right)) => {
      match compare_text(evaluator, left.as_ref(), right.as_ref(), false) {
        value if value < 0 => std::cmp::Ordering::Less,
        value if value > 0 => std::cmp::Ordering::Greater,
        _ => std::cmp::Ordering::Equal,
      }
    }
    (FormulaValue::Boolean(left), FormulaValue::Boolean(right)) => left.cmp(right),
    (FormulaValue::Blank, FormulaValue::Blank) => std::cmp::Ordering::Equal,
    (FormulaValue::Error(left), FormulaValue::Error(right)) => {
      error_text_value(*left).cmp(error_text_value(*right))
    }
    (left, right) => sort_value_rank(left).cmp(&sort_value_rank(right)),
  }
}

fn sort_value_rank(value: &FormulaValue<'_>) -> u8 {
  match value {
    FormulaValue::Number(_) => 0,
    FormulaValue::String(_) => 1,
    FormulaValue::Boolean(_) => 2,
    FormulaValue::Error(_) => 3,
    FormulaValue::Blank => 4,
    FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => 5,
  }
}

fn evaluate_round_direction_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  away_from_zero: bool,
) -> Option<FormulaValue<'doc>> {
  let value = evaluator.number(&args.value(0)?)?;
  let digits = if args.len() == 2 {
    evaluator.number(&args.value(1)?)?
  } else {
    0.0
  } as i32;
  Some(FormulaValue::Number(round_direction(
    value,
    digits,
    away_from_zero,
  )))
}

fn evaluate_roundsig_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = evaluator.number(&args.value(0)?)?;
  let digits = approx_floor(evaluator.number(&args.value(1)?)?);
  if digits < 1.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  if value == 0.0 {
    return Some(FormulaValue::Number(0.0));
  }
  Some(FormulaValue::Number(round_to_significant_digits(
    value, digits,
  )))
}

fn evaluate_trunc_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  if args.is_missing(0) {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  }
  let digits = if args.len() == 2 && !args.is_missing(1) {
    let value = evaluator.number(&args.value(1)?)?;
    if value < 0.0 {
      approx_ceil(value)
    } else {
      approx_floor(value)
    }
  } else {
    0.0
  };
  let value = evaluator.number(&args.value(0)?)?;
  Some(FormulaValue::Number(round_direction(
    value,
    digits.clamp(f64::from(i16::MIN), f64::from(i16::MAX)) as i32,
    false,
  )))
}

fn evaluate_date_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  if args.is_missing(0) {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let mut year = evaluator.number(&args.value(0)?)? as i32;
  let month = evaluator.number(&args.value(1)?)? as i32;
  let day = evaluator.number(&args.value(2)?)? as i32;
  if year < 0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  if evaluator.book.date_system == DateSystem::LibreOffice {
    if year < 100 {
      year = expand_two_digit_year(year);
    }
    let (normalized_year, normalized_month, normalized_day) =
      normalized_date_components(year, month, day)?;
    if !is_valid_libreoffice_gregorian_date(normalized_year, normalized_month, normalized_day) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    return date_serial_with_system(year, month, day, evaluator.book.date_system)
      .map(FormulaValue::Number);
  }
  excel_date_function_serial(year, month, day, evaluator.book.date_system).map(FormulaValue::Number)
}

fn excel_date_function_serial(
  mut year: i32,
  month: i32,
  day: i32,
  date_system: DateSystem,
) -> Option<f64> {
  if year < 1900 {
    year += 1900;
  }
  let month_index = month - 1;
  let normalized_year = year + month_index.div_euclid(12);
  let normalized_month = month_index.rem_euclid(12) + 1;
  date_serial_with_system(normalized_year, normalized_month, 1, date_system)
    .map(|serial| serial + f64::from(day - 1))
}

fn evaluate_time_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let hour = evaluator.number(&args.value(0)?)?;
  let minute = evaluator.number(&args.value(1)?)?;
  let second = evaluator.number(&args.value(2)?)?;
  let value = ((hour * 3600.0 + minute * 60.0 + second) % 86_400.0) / 86_400.0;
  if value < 0.0 {
    Some(FormulaValue::Error(FormulaErrorValue::Value))
  } else {
    Some(FormulaValue::Number(value))
  }
}

fn evaluate_date_part_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  part: DatePart,
) -> Option<FormulaValue<'doc>> {
  let value = args.first_value()?;
  if evaluator.array_context
    && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
  {
    return Some(FormulaValue::Matrix(
      evaluator
        .matrix_values(&value)
        .into_iter()
        .map(|row| {
          row
            .into_iter()
            .map(|value| date_part_value(evaluator, &value, part))
            .collect()
        })
        .collect(),
    ));
  }
  Some(date_part_value(evaluator, &value, part))
}

fn date_part_value<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
  part: DatePart,
) -> FormulaValue<'doc> {
  let Some(serial) = evaluator
    .date_number_from_scalar(value)
    .map(|value| value.floor() as i32)
  else {
    return FormulaValue::Error(FormulaErrorValue::Value);
  };
  let Some((year, month, day)) = date_from_serial_with_system(serial, evaluator.book.date_system)
  else {
    return FormulaValue::Error(FormulaErrorValue::Value);
  };
  FormulaValue::Number(match part {
    DatePart::Year => year as f64,
    DatePart::Month => month as f64,
    DatePart::Day => day as f64,
  })
}

fn evaluate_weeks_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  if args.is_missing(2) {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let start = evaluator.date_number_from_value(&args.value(0)?)?.floor() as i32;
  let end = evaluator.date_number_from_value(&args.value(1)?)?.floor() as i32;
  let mode = evaluator.number(&args.value(2)?)?.floor() as i32;
  match mode {
    0 => Some(FormulaValue::Number(((end - start) / 7) as f64)),
    1 => Some(FormulaValue::Number(
      (weeks_mode_one_index(end, evaluator.book.date_system)?
        - weeks_mode_one_index(start, evaluator.book.date_system)?) as f64,
    )),
    _ => Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
  }
}

fn evaluate_days_in_month_year_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  year: bool,
) -> Option<FormulaValue<'doc>> {
  let serial = evaluator.date_number_from_value(&args.value(0)?)?.floor() as i32;
  let Some((year_value, month, _)) =
    date_from_serial_with_system(serial, evaluator.book.date_system)
  else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  if year {
    Some(FormulaValue::Number(if is_leap_year(year_value) {
      366.0
    } else {
      365.0
    }))
  } else {
    Some(FormulaValue::Number(
      last_day_of_month(year_value, month) as f64
    ))
  }
}

fn evaluate_datedif_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let start = evaluator.date_number_from_value(&args.value(0)?)?.floor() as i32;
  let end = evaluator.date_number_from_value(&args.value(1)?)?.floor() as i32;
  if start > end {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let interval = evaluator.text(&args.value(2)?);
  let day_difference = end - start;
  if day_difference == 0 || interval.eq_ignore_ascii_case("d") {
    return Some(FormulaValue::Number(day_difference as f64));
  }
  let Some((start_year, start_month, start_day)) =
    date_from_serial_with_system(start, evaluator.book.date_system)
  else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let Some((end_year, end_month, end_day)) =
    date_from_serial_with_system(end, evaluator.book.date_system)
  else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let months = datedif_months(
    start_year,
    start_month,
    start_day,
    end_year,
    end_month,
    end_day,
  );
  if interval.eq_ignore_ascii_case("m") {
    return Some(FormulaValue::Number(months as f64));
  }
  if interval.eq_ignore_ascii_case("y") {
    return Some(FormulaValue::Number(datedif_years(
      start_year,
      start_month,
      start_day,
      end_year,
      end_month,
      end_day,
    ) as f64));
  }
  if interval.eq_ignore_ascii_case("ym") {
    return Some(FormulaValue::Number((months % 12) as f64));
  }
  if interval.eq_ignore_ascii_case("md") {
    let days = if start_day <= end_day {
      end_day as i32 - start_day as i32
    } else {
      let (anchor_year, anchor_month) = if end_month == 1 {
        (end_year - 1, 12)
      } else {
        (end_year, end_month as i32 - 1)
      };
      let Some(anchor) = date_serial_with_system(
        anchor_year,
        anchor_month,
        start_day as i32,
        evaluator.book.date_system,
      ) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      };
      end - anchor.floor() as i32
    };
    return Some(FormulaValue::Number(days as f64));
  }
  if interval.eq_ignore_ascii_case("yd") {
    let anchor_year =
      if end_month > start_month || (end_month == start_month && end_day >= start_day) {
        end_year
      } else {
        end_year - 1
      };
    let Some(anchor) = date_serial_with_system(
      anchor_year,
      start_month as i32,
      start_day as i32,
      evaluator.book.date_system,
    ) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    return Some(FormulaValue::Number((end - anchor.floor() as i32) as f64));
  }
  Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
}

fn evaluate_yearfrac_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let start = evaluator.date_number_from_value(&args.value(0)?)?.floor() as i32;
  let end = evaluator.date_number_from_value(&args.value(1)?)?.floor() as i32;
  let basis = args
    .raw_arg(2)
    .and_then(|_| args.value(2))
    .and_then(|value| evaluator.number(&value))
    .map(|value| value.floor() as i32)
    .unwrap_or(0);
  date_yearfrac(start, end, basis)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(
      FormulaErrorValue::IllegalArgument,
    )))
}

fn datedif_months(
  start_year: i32,
  start_month: u32,
  start_day: u32,
  end_year: i32,
  end_month: u32,
  end_day: u32,
) -> i32 {
  let mut months = end_month as i32 - start_month as i32 + 12 * (end_year - start_year);
  if start_day > end_day {
    months -= 1;
  }
  months
}

fn datedif_years(
  start_year: i32,
  start_month: u32,
  start_day: u32,
  end_year: i32,
  end_month: u32,
  end_day: u32,
) -> i32 {
  if end_year <= start_year {
    return 0;
  }
  if end_month > start_month || (end_month == start_month && end_day >= start_day) {
    end_year - start_year
  } else {
    end_year - start_year - 1
  }
}

fn evaluate_time_part_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  part: TimePart,
) -> Option<FormulaValue<'doc>> {
  let value = args.first_value()?;
  let value = evaluator.time_number_from_value(&value).unwrap_or_default();
  let total_seconds = (value.fract() * 86_400.0).floor() as i64;
  Some(FormulaValue::Number(match part {
    TimePart::Hour => total_seconds.rem_euclid(86_400) / 3600,
    TimePart::Minute => total_seconds.rem_euclid(3_600) / 60,
    TimePart::Second => ((value.fract() * 86_400.0).round() as i64).rem_euclid(60),
  } as f64))
}

fn evaluate_eomonth_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let Some(start) = evaluator.date_number_from_value(&args.value(0)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  let Some(months) = evaluator.number(&args.value(1)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  let start = start.floor() as i32;
  let months = months.floor() as i32;
  let (year, month, _) = date_from_serial_with_system(start, evaluator.book.date_system)?;
  date_serial_with_system(
    year,
    month as i32 + months + 1,
    0,
    evaluator.book.date_system,
  )
  .map(FormulaValue::Number)
  .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
}

fn evaluate_edate_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let start = evaluator.date_number_from_value(&args.value(0)?)?.floor() as i32;
  let months = evaluator.number(&args.value(1)?)?.floor() as i32;
  let (year, month, day) = date_from_serial_with_system(start, evaluator.book.date_system)?;
  let (target_year, target_month, _) = normalized_date_components(year, month as i32 + months, 1)?;
  let target_day = day.min(last_day_of_month(target_year, target_month));
  date_serial_with_system(
    target_year,
    target_month as i32,
    target_day as i32,
    evaluator.book.date_system,
  )
  .map(FormulaValue::Number)
  .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
}

fn evaluate_eastersunday_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = args.value(0)?;
  if is_multicell_scalar_argument(&value) {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let Some(year_number) = evaluator.number(&value) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  let mut year = year_number.trunc() as i32;
  if (0..100).contains(&year) {
    year = expand_two_digit_year(year);
  }
  if !(1583..=9956).contains(&year) {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let n = year % 19;
  let b = year / 100;
  let c = year % 100;
  let d = b / 4;
  let e = b % 4;
  let f = (b + 8) / 25;
  let g = (b - f + 1) / 3;
  let h = (19 * n + b - d - g + 15) % 30;
  let i = c / 4;
  let k = c % 4;
  let l = (32 + 2 * e + 2 * i - h - k) % 7;
  let m = (n + 11 * h + 22 * l) / 451;
  let o = h + l - 7 * m + 114;
  let day = o % 31 + 1;
  let month = o / 31;
  date_serial(year, month, day)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(
      FormulaErrorValue::IllegalArgument,
    )))
}

fn evaluate_is_leap_year_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = args.value(0)?;
  if is_multicell_scalar_argument(&value) {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let Some(serial) = evaluator.date_number_from_value(&value) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  let (year, _, _) = date_from_serial_with_system(serial as i32, evaluator.book.date_system)?;
  Some(FormulaValue::Boolean(is_leap_year(year)))
}

fn evaluate_iso_weeknum_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = args.value(0)?;
  if is_multicell_scalar_argument(&value) {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let Some(serial) = evaluator.date_number_from_value(&value) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  iso_weeknum_from_serial_with_system(serial as i32, evaluator.book.date_system)
    .map(|value| FormulaValue::Number(value as f64))
    .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
}

fn evaluate_dollar_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = args.value(0)?;
  let digits = if args.len() == 2 {
    args.value(1)?
  } else {
    FormulaValue::Number(2.0)
  };
  if evaluator.array_context && (is_matrix_argument(&value) || is_matrix_argument(&digits)) {
    return evaluator.map_binary_values(value, digits, |evaluator, value, digits| {
      Some(evaluator.dollar_value(value, digits))
    });
  }
  Some(evaluator.dollar_value(&value, &digits))
}

fn evaluate_left_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  bytes: bool,
) -> Option<FormulaValue<'doc>> {
  let value = args.value(0)?;
  let len = if args.len() == 2 {
    args.value(1)?
  } else {
    FormulaValue::Number(1.0)
  };
  if evaluator.array_context && (is_matrix_argument(&value) || is_matrix_argument(&len)) {
    return evaluator.map_binary_values(value, len, |evaluator, value, len| {
      if bytes {
        evaluator.leftb_value(value, len)
      } else {
        evaluator.left_value(value, len)
      }
    });
  }
  if bytes {
    evaluator.leftb_value(&value, &len)
  } else {
    evaluator.left_value(&value, &len)
  }
}

fn evaluate_right_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  bytes: bool,
) -> Option<FormulaValue<'doc>> {
  let value = args.value(0)?;
  let len = if args.len() == 2 {
    args.value(1)?
  } else {
    FormulaValue::Number(1.0)
  };
  if evaluator.array_context && (is_matrix_argument(&value) || is_matrix_argument(&len)) {
    return evaluator.map_binary_values(value, len, |evaluator, value, len| {
      if bytes {
        evaluator.rightb_value(value, len)
      } else {
        evaluator.right_value(value, len)
      }
    });
  }
  if bytes {
    evaluator.rightb_value(&value, &len)
  } else {
    evaluator.right_value(&value, &len)
  }
}

fn evaluate_mid_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  bytes: bool,
) -> Option<FormulaValue<'doc>> {
  let text = evaluator.text(&args.value(0)?);
  let start = evaluator.number(&args.value(1)?)?;
  let len = evaluator.number(&args.value(2)?)?;
  if start < 1.0 || len < 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  if bytes {
    let prefix = leftb(&text, start as usize - 1);
    let suffix = text
      .chars()
      .skip(prefix.chars().count())
      .collect::<String>();
    return Some(FormulaValue::String(Cow::Owned(leftb(
      &suffix,
      len as usize,
    ))));
  }
  Some(FormulaValue::String(Cow::Owned(
    text
      .chars()
      .skip((start as usize).saturating_sub(1))
      .take(len as usize)
      .collect(),
  )))
}

fn evaluate_cell_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let info_type = evaluator.text(&args.value(0)?).to_ascii_uppercase();
  let reference = if args.len() == 2 {
    args
      .value(1)
      .and_then(|value| evaluator.as_reference(&value))
  } else {
    None
  };
  let sheet = reference
    .as_ref()
    .map(|reference| evaluator.range_sheet(reference))
    .unwrap_or(evaluator.current_sheet);
  let address = reference
    .as_ref()
    .map(|reference| reference.range.start)
    .or(evaluator.current_cell)
    .unwrap_or_default();
  match info_type.as_str() {
    "COL" => Some(FormulaValue::Number(address.column as f64 + 1.0)),
    "ROW" => Some(FormulaValue::Number(address.row as f64 + 1.0)),
    "SHEET" => evaluator
      .book
      .sheet_names
      .iter()
      .position(|binding| binding.id == sheet)
      .map(|index| FormulaValue::Number(index as f64 + 1.0)),
    "ADDRESS" => Some(FormulaValue::String(Cow::Owned(format!(
      "${}${}",
      column_index_to_name(address.column),
      address.row + 1
    )))),
    "FILENAME" => {
      let file = evaluator
        .book
        .source_file_name
        .as_deref()
        .unwrap_or("workbook.xlsx");
      let sheet_name = evaluator
        .book
        .sheet_names
        .iter()
        .find(|binding| binding.id == sheet)
        .map(|binding| binding.name.as_ref())
        .unwrap_or("");
      Some(FormulaValue::String(Cow::Owned(format!(
        "[{file}]{sheet_name}"
      ))))
    }
    "CONTENTS" => Some(evaluator.book.cell_value(sheet, address)),
    "TYPE" => Some(FormulaValue::String(Cow::Borrowed(
      match evaluator.book.cell_value(sheet, address) {
        FormulaValue::Blank => "b",
        FormulaValue::String(_) => "l",
        _ => "v",
      },
    ))),
    "WIDTH" => Some(FormulaValue::Number(0.0)),
    "PREFIX" => Some(FormulaValue::String(Cow::Borrowed(""))),
    "PROTECT" | "COLOR" | "PARENTHESES" => Some(FormulaValue::Number(0.0)),
    "FORMAT" => Some(FormulaValue::String(Cow::Borrowed("G"))),
    "COORD" => Some(FormulaValue::String(Cow::Owned(format!(
      "${}:${}${}",
      column_index_to_name(sheet.0.saturating_sub(1)),
      column_index_to_name(address.column),
      address.row + 1
    )))),
    _ => Some(FormulaValue::Error(FormulaErrorValue::Value)),
  }
}

fn evaluate_address_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let row = evaluator.number(&args.value(0)?)? as i32;
  let column = evaluator.number(&args.value(1)?)? as i32;
  if row <= 0
    || column <= 0
    || row as u32 > XLSX_MAX_ROW_ZERO_BASED + 1
    || column as u32 > XLSX_MAX_COLUMN_ZERO_BASED + 1
  {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let abs_num = if args.len() >= 3 {
    let value = args.value(2)?;
    if matches!(value, FormulaValue::Blank) {
      1.0
    } else {
      evaluator.number(&value)?
    }
  } else {
    1.0
  } as i32;
  let a1 = if args.len() >= 4 {
    let value = args.value(3)?;
    if matches!(value, FormulaValue::Blank) {
      true
    } else {
      evaluator.truthy(&value)
    }
  } else {
    true
  };
  let sheet = if args.len() >= 5 {
    let sheet = evaluator.text(&args.value(4)?);
    Some(if sheet.is_empty() {
      String::new()
    } else if a1 {
      let separator = match evaluator.grammar {
        FormulaGrammar::OpenFormula | FormulaGrammar::CalcA1 => ".",
        FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1 => "!",
      };
      format!("{}{}", quote_sheet_name_for_reference(&sheet), separator)
    } else {
      format!("{}!", quote_sheet_name_for_reference(&sheet))
    })
  } else {
    None
  };
  let reference = if a1 {
    let (abs_col, abs_row) = match abs_num {
      1 | 5 => (true, true),
      2 | 6 => (false, true),
      3 | 7 => (true, false),
      4 | 8 => (false, false),
      _ => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    };
    format!(
      "{}{}{}{}",
      if abs_col { "$" } else { "" },
      column_index_to_name((column - 1) as u32),
      if abs_row { "$" } else { "" },
      row
    )
  } else {
    match abs_num {
      1 | 5 => format!("R{row}C{column}"),
      2 | 6 => format!("R{row}C[{column}]"),
      3 | 7 => format!("R[{row}]C{column}"),
      4 | 8 => format!("R[{row}]C[{column}]"),
      _ => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    }
  };
  Some(FormulaValue::String(Cow::Owned(format!(
    "{}{reference}",
    sheet.unwrap_or_default()
  ))))
}

fn evaluate_indirect_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let text = evaluator.text(&args.value(0)?);
  let use_a1 = if args.len() == 2 {
    evaluator.truthy(&args.value(1)?)
  } else {
    true
  };
  let reference_text;
  let text = if use_a1 {
    text.as_str()
  } else {
    let Some(converted) =
      indirect_r1c1_reference_to_a1(&text, evaluator.current_cell.unwrap_or_default())
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Ref));
    };
    reference_text = converted;
    reference_text.as_str()
  };
  if use_a1 && !indirect_reference_matches_grammar(text, evaluator.grammar) {
    return Some(FormulaValue::Error(FormulaErrorValue::Ref));
  }
  if use_a1 && evaluator.calc_a1_indirect_bang_reference {
    return Some(FormulaValue::Error(FormulaErrorValue::Ref));
  }
  let normalized_text;
  let text = if use_a1 && matches!(evaluator.grammar, FormulaGrammar::ExcelR1C1) {
    let Some(converted) =
      indirect_r1c1_reference_to_a1(text, evaluator.current_cell.unwrap_or_default())
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Ref));
    };
    normalized_text = converted;
    normalized_text.as_str()
  } else if use_a1
    && matches!(
      evaluator.grammar,
      FormulaGrammar::CalcA1 | FormulaGrammar::OpenFormula
    )
  {
    if let Some(converted) = calc_a1_indirect_reference_to_excel_a1(text) {
      normalized_text = converted;
      normalized_text.as_str()
    } else {
      text
    }
  } else {
    text
  };
  evaluator
    .resolve_reference(text)
    .map(FormulaValue::Reference)
    .or_else(|| evaluator.evaluate_defined_name(&Cow::Owned(text.to_string())))
    .or(Some(FormulaValue::Error(FormulaErrorValue::Ref)))
}

fn indirect_reference_matches_grammar(reference: &str, grammar: FormulaGrammar) -> bool {
  let reference = reference.trim();
  match grammar {
    FormulaGrammar::CalcA1 => !reference.contains('!'),
    FormulaGrammar::OpenFormula => true,
    FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1 => {
      reference.contains('!') || !looks_like_calc_sheet_reference(reference)
    }
  }
}

fn looks_like_calc_sheet_reference(reference: &str) -> bool {
  let Some((sheet, address)) = reference.rsplit_once('.') else {
    return false;
  };
  !sheet.is_empty()
    && CellAddress::parse_a1(address.trim_start_matches('$')).is_ok()
    && !sheet.chars().all(|ch| ch.is_ascii_digit())
}

fn calc_a1_indirect_reference_to_excel_a1(reference: &str) -> Option<String> {
  let (sheet, address) = reference.rsplit_once('.')?;
  if sheet.is_empty() || CellAddress::parse_a1(address.trim_start_matches('$')).is_err() {
    return None;
  }
  Some(format!("{sheet}!{address}"))
}

fn indirect_r1c1_reference_to_a1(reference: &str, base: CellAddress) -> Option<String> {
  let (sheet, address) = reference
    .split_once('!')
    .map(|(sheet, address)| (Some(sheet), address))
    .unwrap_or((None, reference));
  let converted = crate::parser::r1c1_reference_to_a1(address, base)?;
  Some(match sheet {
    Some(sheet) => format!("{sheet}!{converted}"),
    None => converted,
  })
}

fn evaluate_index_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let row = args
    .value(1)
    .and_then(|value| evaluator.number(&value))
    .unwrap_or(0.0);
  let column = args
    .value(2)
    .and_then(|value| evaluator.number(&value))
    .unwrap_or(0.0);
  let area = args
    .value(3)
    .and_then(|value| evaluator.number(&value))
    .unwrap_or(1.0);
  if row < 0.0 || column < 0.0 || area < 1.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let row = row as u32;
  let column = column as u32;
  let area = area as usize;
  let ranges = args.reference_ranges(0)?;
  if !ranges.is_empty() {
    return Some(evaluator.index_reference_area(&ranges, row, column, area, args.len()));
  }
  let value = args
    .value(0)
    .unwrap_or(FormulaValue::Error(FormulaErrorValue::Value));
  if let FormulaValue::Matrix(rows) = value {
    return Some(index_matrix(rows, row, column, args.len()));
  }
  let Some(reference) = evaluator.as_reference(&value) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  Some(evaluator.index_reference_area(&[reference], row, column, area, args.len()))
}

fn evaluate_offset_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let reference = single_raw_reference(args, 0).or_else(|| {
    let value = args.value(0)?;
    evaluator.as_reference(&value)
  })?;
  let row_offset_value = raw_row_column_function_value(args, 1, false).or_else(|| args.value(1))?;
  let column_offset_value =
    raw_row_column_function_value(args, 2, true).or_else(|| args.value(2))?;
  let height = args
    .value(3)
    .and_then(|value| evaluator.number(&value))
    .map(|value| value as i64)
    .unwrap_or_else(|| i64::from(reference.range.end.row - reference.range.start.row + 1));
  let width = args
    .value(4)
    .and_then(|value| evaluator.number(&value))
    .map(|value| value as i64)
    .unwrap_or_else(|| i64::from(reference.range.end.column - reference.range.start.column + 1));
  if is_matrix_argument(&row_offset_value) || is_matrix_argument(&column_offset_value) {
    return evaluate_offset_matrix(
      evaluator,
      &reference,
      row_offset_value,
      column_offset_value,
      height,
      width,
    );
  }
  let row_offset = evaluator.number(&row_offset_value)? as i64;
  let column_offset = evaluator.number(&column_offset_value)? as i64;
  offset_reference(&reference, row_offset, column_offset, height, width)
}

fn evaluate_offset_matrix<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  reference: &QualifiedRange<'doc>,
  row_offset_value: FormulaValue<'doc>,
  column_offset_value: FormulaValue<'doc>,
  height: i64,
  width: i64,
) -> Option<FormulaValue<'doc>> {
  let row_offsets = evaluator.matrix_values(&row_offset_value);
  let column_offsets = evaluator.matrix_values(&column_offset_value);
  let row_count = row_offsets.len();
  let row_columns = row_offsets.first().map_or(0, Vec::len);
  let column_count = column_offsets.len();
  let column_columns = column_offsets.first().map_or(0, Vec::len);
  if row_count == 0 || row_columns == 0 || column_count == 0 || column_columns == 0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let rows = row_count.max(column_count);
  let columns = row_columns.max(column_columns);
  if !matrix_can_broadcast_local(row_count, row_columns, rows, columns)
    || !matrix_can_broadcast_local(column_count, column_columns, rows, columns)
  {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let mut ranges = Vec::with_capacity(rows * columns);
  for row in 0..rows {
    for column in 0..columns {
      let row_offset =
        evaluator.number(&row_offsets[row.min(row_count - 1)][column.min(row_columns - 1)])? as i64;
      let column_offset = evaluator
        .number(&column_offsets[row.min(column_count - 1)][column.min(column_columns - 1)])?
        as i64;
      match offset_reference(reference, row_offset, column_offset, height, width)? {
        FormulaValue::Reference(range) => ranges.push(range),
        value => return Some(value),
      }
    }
  }
  Some(FormulaValue::RefList(ranges))
}

fn offset_reference<'doc>(
  reference: &QualifiedRange<'doc>,
  row_offset: i64,
  column_offset: i64,
  height: i64,
  width: i64,
) -> Option<FormulaValue<'doc>> {
  if width <= 0 || height <= 0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let start_column = i64::from(reference.range.start.column) + column_offset;
  let start_row = i64::from(reference.range.start.row) + row_offset;
  let end_column = start_column + width - 1;
  let end_row = start_row + height - 1;
  if start_column < 0
    || start_row < 0
    || end_column > i64::from(XLSX_MAX_COLUMN_ZERO_BASED)
    || end_row > i64::from(XLSX_MAX_ROW_ZERO_BASED)
  {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  Some(FormulaValue::Reference(QualifiedRange {
    sheet: reference.sheet,
    sheet_name: reference.sheet_name.clone(),
    end_sheet_name: reference.end_sheet_name.clone(),
    range: CellRange::new(
      CellAddress {
        column: start_column as u32,
        row: start_row as u32,
      },
      CellAddress {
        column: end_column as u32,
        row: end_row as u32,
      },
    ),
    start_flags: reference.start_flags,
    end_flags: reference.end_flags,
  }))
}

fn matrix_can_broadcast_local(
  rows: usize,
  columns: usize,
  target_rows: usize,
  target_columns: usize,
) -> bool {
  (rows == target_rows || rows == 1) && (columns == target_columns || columns == 1)
}

fn evaluate_vlookup_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  horizontal: bool,
) -> Option<FormulaValue<'doc>> {
  let lookup = args.value(0)?;
  let table = args.value(1)?;
  let index = evaluator.number(&args.value(2)?)?.floor();
  if index < 1.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let sorted = if args.len() == 4 {
    evaluator.truthy(&args.value(3)?)
  } else {
    true
  };
  if horizontal {
    Some(evaluator.evaluate_hlookup_value(
      evaluator.scalar_value(lookup),
      &table,
      index as u32,
      sorted,
    ))
  } else if evaluator.array_context
    && matches!(
      lookup,
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
    )
  {
    evaluator.map_unary_values(lookup, |evaluator, lookup| {
      Some(evaluator.evaluate_vlookup_value(
        evaluator.scalar_value(lookup.clone()),
        &table,
        index as u32,
        sorted,
      ))
    })
  } else {
    Some(evaluator.evaluate_vlookup_value(
      evaluator.scalar_value(lookup),
      &table,
      index as u32,
      sorted,
    ))
  }
}

fn evaluate_getpivotdata_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  if !args.len().is_multiple_of(2) {
    return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
  }
  if let Some(block) = evaluator.as_reference(&args.value(0)?) {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let filters = match parse_getpivotdata_compact_filters(&evaluator.text(&args.value(1)?)) {
      Some(filters) => filters,
      None => return Some(FormulaValue::Error(FormulaErrorValue::Ref)),
    };
    let request = PivotDataRequest {
      current_sheet: evaluator.current_sheet,
      block,
      data_field_name: None,
      filters,
    };
    return Some(
      evaluator
        .pivot_data(&request)
        .unwrap_or_else(FormulaValue::Error),
    );
  }
  let data_field_name = evaluator.text(&args.value(0)?);
  let Some(block) = evaluator.as_reference(&args.value(1)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Ref));
  };
  let mut filters = Vec::new();
  let mut index = 2;
  while index + 1 < args.len() {
    filters.push(PivotFieldFilter {
      field_name: Cow::Owned(evaluator.text(&args.value(index)?)),
      match_value: Cow::Owned(evaluator.text(&args.value(index + 1)?)),
    });
    index += 2;
  }
  let request = PivotDataRequest {
    current_sheet: evaluator.current_sheet,
    block,
    data_field_name: Some(Cow::Owned(data_field_name)),
    filters,
  };
  Some(
    evaluator
      .pivot_data(&request)
      .unwrap_or_else(FormulaValue::Error),
  )
}

fn parse_getpivotdata_compact_filters<'doc>(text: &str) -> Option<Vec<PivotFieldFilter<'doc>>> {
  let mut filters = Vec::new();
  let mut rest = text.trim();
  while !rest.is_empty() {
    rest = rest.trim_start();
    let (field, after_field) = rest.split_once('[')?;
    let field = field.trim();
    if field.is_empty() {
      return None;
    }
    let (item, after_item) = parse_getpivotdata_bracket_item(after_field)?;
    filters.push(PivotFieldFilter {
      field_name: Cow::Owned(field.to_string()),
      match_value: Cow::Owned(item),
    });
    rest = after_item;
  }
  Some(filters)
}

fn parse_getpivotdata_bracket_item(text: &str) -> Option<(String, &str)> {
  let mut item = String::new();
  let mut chars = text.char_indices();
  let mut quoted = false;
  if let Some((_, '\'')) = chars.clone().next() {
    quoted = true;
    chars.next();
  }
  while let Some((index, ch)) = chars.next() {
    if quoted {
      if ch == '\'' {
        let next_index = index + ch.len_utf8();
        return text[next_index..]
          .strip_prefix(']')
          .map(|rest| (item, rest));
      }
      item.push(ch);
    } else if ch == ']' {
      return Some((item.trim().to_string(), &text[index + ch.len_utf8()..]));
    } else {
      item.push(ch);
    }
  }
  None
}

fn evaluate_mround_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  if args.is_missing(0) || args.is_missing(1) {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  }
  let value = evaluator.number(&args.value(0)?)?;
  let multiple = evaluator.number(&args.value(1)?)?;
  if value * multiple < 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
  Some(FormulaValue::Number(mround(value, multiple)))
}

fn evaluate_decimal_to_base_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  base: u32,
  min: f64,
  max: f64,
) -> Option<FormulaValue<'doc>> {
  let value = evaluator.number(&args.value(0)?)?;
  let places = args
    .raw_arg(1)
    .filter(|_| !args.is_missing(1))
    .and_then(|_| args.value(1))
    .and_then(|value| evaluator.number(&value))
    .map(|value| value as i32);
  convert_from_decimal(value, min, max, base, places, 10)
    .map(|value| FormulaValue::String(Cow::Owned(value)))
    .or(Some(FormulaValue::Error(
      FormulaErrorValue::IllegalArgument,
    )))
}

fn evaluate_base_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = approx_floor(evaluator.number(&args.value(0)?)?);
  let radix = approx_floor(evaluator.number(&args.value(1)?)?);
  let min_len_value = args
    .raw_arg(2)
    .filter(|_| !args.is_missing(2))
    .and_then(|_| args.value(2))
    .and_then(|value| evaluator.number(&value))
    .map(approx_floor)
    .unwrap_or(1.0);
  let min_len = if (1.0..u16::MAX as f64).contains(&min_len_value) {
    floor_to_usize(min_len_value)?
  } else if min_len_value == 0.0 {
    1
  } else {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  };
  if value < 0.0 || !(2.0..=36.0).contains(&radix) {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  base_number_text(value, floor_to_u32(radix)?, min_len)
    .map(|value| FormulaValue::String(Cow::Owned(value)))
    .or(Some(FormulaValue::Error(
      FormulaErrorValue::IllegalArgument,
    )))
}

fn evaluate_decimal_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let text = evaluator.text(&args.value(0)?);
  let base = approx_floor(evaluator.number(&args.value(1)?)?);
  if !(2.0..=36.0).contains(&base) {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  decimal_text_to_number(&text, floor_to_u32(base)?)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(
      FormulaErrorValue::IllegalArgument,
    )))
}

fn evaluate_base_to_decimal_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  base: u32,
) -> Option<FormulaValue<'doc>> {
  convert_to_decimal(&base_digits_text(evaluator, &args.value(0)?), base, 10)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(
      FormulaErrorValue::IllegalArgument,
    )))
}

fn evaluate_base_to_base_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  from_base: u32,
  to_base: u32,
  min: f64,
  max: f64,
) -> Option<FormulaValue<'doc>> {
  let value = convert_to_decimal(&base_digits_text(evaluator, &args.value(0)?), from_base, 10)?;
  let places = args
    .raw_arg(1)
    .filter(|_| !args.is_missing(1))
    .and_then(|_| args.value(1))
    .and_then(|value| evaluator.number(&value))
    .map(|value| value as i32);
  convert_from_decimal(value, min, max, to_base, places, 10)
    .map(|value| FormulaValue::String(Cow::Owned(value)))
    .or(Some(FormulaValue::Error(
      FormulaErrorValue::IllegalArgument,
    )))
}

fn base_digits_text<'doc>(evaluator: &EvalContext<'_, 'doc>, value: &FormulaValue<'doc>) -> String {
  match evaluator.scalar_value(value.clone()) {
    FormulaValue::Boolean(value) => {
      if value {
        "1".to_string()
      } else {
        "0".to_string()
      }
    }
    FormulaValue::Number(value) if value.is_finite() => {
      display_text_from_value(&FormulaValue::Number(approx_floor(value)))
    }
    value => evaluator.text(&value),
  }
}

fn evaluate_gcd_lcm_reader<'doc>(
  args: FunctionArgReader<'_, '_, 'doc>,
  gcd: bool,
) -> Option<FormulaValue<'doc>> {
  if args.is_empty() {
    return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
  }
  let mut result = if gcd { 0.0 } else { 1.0 };
  for index in 0..args.len() {
    for value in args.value_numbers(index)? {
      if value < 0.0 {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      result = if gcd {
        gcd_number(result, approx_floor(value))
      } else {
        lcm_number(result, approx_floor(value))
      };
    }
  }
  Some(FormulaValue::Number(result))
}

fn evaluate_median_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let mut values = Vec::new();
  for index in 0..args.len() {
    values.extend(evaluator.median_numbers_from_value(&args.value(index)?));
  }
  if values.is_empty() {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
  values.sort_by(f64::total_cmp);
  let mid = values.len() / 2;
  Some(FormulaValue::Number(if values.len().is_multiple_of(2) {
    (values[mid - 1] + values[mid]) / 2.0
  } else {
    values[mid]
  }))
}

fn evaluate_mmult_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let left = matrix_number_values(evaluator, &args.array_value(0)?)?;
  let right = matrix_number_values(evaluator, &args.array_value(1)?)?;
  let Some(result) = matrix_multiply(&left, &right) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  if result.len() == 1 && result.first().is_some_and(|row| row.len() == 1) {
    return result
      .first()
      .and_then(|row| row.first())
      .copied()
      .map(FormulaValue::Number);
  }
  Some(FormulaValue::Matrix(
    result
      .into_iter()
      .map(|row| row.into_iter().map(FormulaValue::Number).collect())
      .collect(),
  ))
}

fn evaluate_mdeterm_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let matrix = matrix_number_values_strict(evaluator, &args.array_value(0)?)?;
  let size = matrix.len();
  if size == 0 || matrix.iter().any(|row| row.len() != size) {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  Some(FormulaValue::Number(determinant(matrix)))
}

fn evaluate_munit_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let Some(size) = evaluator.number(&args.value(0)?).map(approx_floor) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  if size < 1.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let size = size as usize;
  let mut matrix = Vec::with_capacity(size);
  for row in 0..size {
    let mut values = Vec::with_capacity(size);
    for column in 0..size {
      values.push(FormulaValue::Number(if row == column { 1.0 } else { 0.0 }));
    }
    matrix.push(values);
  }
  Some(FormulaValue::Matrix(matrix))
}

fn evaluate_choose_rows_columns_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  rows: bool,
) -> Option<FormulaValue<'doc>> {
  let matrix = evaluator.matrix_values(&args.array_value(0)?);
  let row_count = matrix.len();
  let column_count = matrix.first().map_or(0, Vec::len);
  if row_count == 0 || column_count == 0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  if rows {
    let mut output = Vec::new();
    for arg_index in 1..args.len() {
      for index_value in evaluator
        .matrix_values(&args.array_value(arg_index)?)
        .into_iter()
        .flatten()
      {
        let Some(index) = evaluator
          .number(&index_value)
          .map(|value| value.trunc() as i64)
        else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        let Some(row) = choose_row_column_index(index, row_count) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        output.push(matrix.get(row)?.clone());
      }
    }
    Some(FormulaValue::Matrix(output))
  } else {
    let mut indexes = Vec::new();
    for arg_index in 1..args.len() {
      for index_value in evaluator
        .matrix_values(&args.array_value(arg_index)?)
        .into_iter()
        .flatten()
      {
        let Some(index) = evaluator
          .number(&index_value)
          .map(|value| value.trunc() as i64)
        else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        let Some(index) = choose_row_column_index(index, column_count) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        indexes.push(index);
      }
    }
    Some(FormulaValue::Matrix(
      matrix
        .into_iter()
        .map(|row| {
          indexes
            .iter()
            .filter_map(|index| row.get(*index).cloned())
            .collect()
        })
        .collect(),
    ))
  }
}

fn evaluate_ceiling_floor_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  ceiling: bool,
  kind: CeilingFloorKind,
) -> Option<FormulaValue<'doc>> {
  let value = args.value(0)?;
  let significance = if args.len() >= 2 && !args.is_missing(1) {
    Some(args.value(1)?)
  } else {
    None
  };
  let abs_mode = args.len() >= 3 && args.value(2).is_some_and(|value| evaluator.truthy(&value));
  if evaluator.array_context
    && (is_matrix_argument(&value) || significance.as_ref().is_some_and(is_matrix_argument))
  {
    return match significance {
      Some(significance) => {
        evaluator.map_binary_values(value, significance, |evaluator, value, significance| {
          Some(if ceiling {
            evaluator.ceiling_value(value, Some(significance), abs_mode, kind)
          } else {
            evaluator.floor_value(value, Some(significance), abs_mode, kind)
          })
        })
      }
      None => evaluator.map_unary_values(value, |evaluator, value| {
        Some(if ceiling {
          evaluator.ceiling_value(value, None, abs_mode, kind)
        } else {
          evaluator.floor_value(value, None, abs_mode, kind)
        })
      }),
    };
  }
  Some(if ceiling {
    evaluator.ceiling_value(&value, significance.as_ref(), abs_mode, kind)
  } else {
    evaluator.floor_value(&value, significance.as_ref(), abs_mode, kind)
  })
}

fn evaluate_ceiling_floor_legacy_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  ceiling: bool,
) -> Option<FormulaValue<'doc>> {
  let value = evaluator.number(&args.value(0)?)?;
  let significance = evaluator.number(&args.value(1)?)?;
  let result = if ceiling {
    ceiling_excel_legacy(value, significance)
  } else {
    floor_excel_legacy(value, significance)
  };
  match result {
    Ok(value) => Some(FormulaValue::Number(value)),
    Err(error) => Some(FormulaValue::Error(numeric_error_value(error))),
  }
}

fn evaluate_formula_text_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = args.value(0)?;
  let Some(reference) = evaluator.as_reference(&value) else {
    return Some(FormulaValue::Error(FormulaErrorValue::NA));
  };
  let sheet = evaluator.range_sheet(&reference);
  evaluator
    .book
    .formula_text(sheet, reference.range.start)
    .map(|text| FormulaValue::String(Cow::Owned(text)))
    .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
}

fn evaluate_chisq_inv_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  right_tail: bool,
) -> Option<FormulaValue<'doc>> {
  let p_value = args.value(0)?;
  let df_value = args.value(1)?;
  if evaluator.array_context && (is_matrix_argument(&p_value) || is_matrix_argument(&df_value)) {
    let p_matrix = evaluator.matrix_values(&p_value);
    let df_matrix = evaluator.matrix_values(&df_value);
    let rows = p_matrix.len().max(df_matrix.len());
    let columns = p_matrix
      .first()
      .map(Vec::len)
      .unwrap_or(1)
      .max(df_matrix.first().map(Vec::len).unwrap_or(1));
    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let p = evaluator.number(matrix_item(&p_matrix, row, column)?)?;
        let df = approx_floor(evaluator.number(matrix_item(&df_matrix, row, column)?)?);
        result_row.push(chisq_inv_value(p, df, right_tail));
      }
      result.push(result_row);
    }
    return Some(FormulaValue::Matrix(result));
  }
  let p = evaluator.number(&p_value)?;
  let df = approx_floor(evaluator.number(&df_value)?);
  Some(chisq_inv_value(p, df, right_tail))
}

fn evaluate_chisq_test_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let actual = evaluator.matrix_values(&args.value(0)?);
  let expected = evaluator.matrix_values(&args.value(1)?);
  if actual.is_empty()
    || expected.is_empty()
    || actual.len() != expected.len()
    || actual.first()?.len() != expected.first()?.len()
  {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
  let rows = actual.len();
  let columns = actual.first()?.len();
  let mut chi = KahanSum::default();
  let mut has_value = false;
  for column in 0..columns {
    for row in 0..rows {
      match (&actual[row][column], &expected[row][column]) {
        (FormulaValue::Blank, _) | (_, FormulaValue::Blank) => {}
        (left, right) => {
          let Some(observed) = evaluator.number(left) else {
            return Some(FormulaValue::Error(FormulaErrorValue::Value));
          };
          let Some(expect) = evaluator.number(right) else {
            return Some(FormulaValue::Error(FormulaErrorValue::Value));
          };
          if expect == 0.0 {
            return Some(FormulaValue::Error(FormulaErrorValue::Div0));
          }
          has_value = true;
          let delta = observed - expect;
          let term = delta * delta / expect;
          if term.is_infinite() {
            return Some(FormulaValue::Error(FormulaErrorValue::Num));
          }
          chi.add(term);
        }
      }
    }
  }
  if !has_value {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
  let df = if rows == 1 || columns == 1 {
    (rows * columns).saturating_sub(1) as f64
  } else {
    ((rows - 1) * (columns - 1)) as f64
  };
  if df == 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  Some(FormulaValue::Number(lo_chi_dist(chi.finish(), df)))
}

fn evaluate_f_test_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let left = evaluator.value_numbers(&args.value(0)?);
  let right = evaluator.value_numbers(&args.value(1)?);
  if left.len() < 2 || right.len() < 2 {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let var_left = variance_slice(&left, true)?;
  let var_right = variance_slice(&right, true)?;
  if var_left == 0.0 || var_right == 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let (f, df1, df2) = if var_left > var_right {
    (
      var_left / var_right,
      left.len() as f64 - 1.0,
      right.len() as f64 - 1.0,
    )
  } else {
    (
      var_right / var_left,
      right.len() as f64 - 1.0,
      left.len() as f64 - 1.0,
    )
  };
  let right_tail = lo_f_dist_right_tail(f, df1, df2);
  Some(FormulaValue::Number(2.0 * right_tail.min(1.0 - right_tail)))
}

fn evaluate_t_test_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let tails = approx_floor(evaluator.number(&args.value(2)?)?);
  let kind = approx_floor(evaluator.number(&args.value(3)?)?);
  if tails != 1.0 && tails != 2.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let result = if kind == 1.0 {
    t_test_paired(evaluator, args)
  } else if kind == 2.0 || kind == 3.0 {
    t_test_two_sample(evaluator, args, kind == 3.0)
  } else {
    Err(FormulaErrorValue::IllegalArgument)
  };
  match result {
    Ok((t, df)) => student_t_probability(t, df, tails as i32)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Num))),
    Err(error) => Some(FormulaValue::Error(error)),
  }
}

fn t_test_paired<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Result<(f64, f64), FormulaErrorValue> {
  let left_value = args.array_value(0).ok_or(FormulaErrorValue::Value)?;
  let right_value = args.array_value(1).ok_or(FormulaErrorValue::Value)?;
  let left = evaluator.matrix_values(&left_value);
  let right = evaluator.matrix_values(&right_value);
  if left.len() != right.len()
    || left
      .iter()
      .zip(&right)
      .any(|(left_row, right_row)| left_row.len() != right_row.len())
  {
    return Err(FormulaErrorValue::IllegalArgument);
  }
  let mut count = 0.0;
  let mut sum_left = KahanSum::default();
  let mut sum_right = KahanSum::default();
  let mut sum_square_difference = KahanSum::default();
  for (left_row, right_row) in left.iter().zip(&right) {
    for (left, right) in left_row.iter().zip(right_row) {
      match (left, right) {
        (FormulaValue::Error(error), _) | (_, FormulaValue::Error(error)) => return Err(*error),
        (FormulaValue::Blank | FormulaValue::String(_), _)
        | (_, FormulaValue::Blank | FormulaValue::String(_)) => {}
        _ => {
          let left = evaluator.number(left).ok_or(FormulaErrorValue::Value)?;
          let right = evaluator.number(right).ok_or(FormulaErrorValue::Value)?;
          sum_left.add(left);
          sum_right.add(right);
          let difference = left - right;
          sum_square_difference.add(difference * difference);
          count += 1.0;
        }
      }
    }
  }
  if count < 1.0 {
    return Err(FormulaErrorValue::Value);
  }
  let sum_difference = sum_left.finish() - sum_right.finish();
  let divider = sum_square_difference.finish() * count - sum_difference * sum_difference;
  if divider == 0.0 {
    return Err(FormulaErrorValue::Div0);
  }
  Ok((
    sum_difference.abs() * ((count - 1.0) / divider).sqrt(),
    count - 1.0,
  ))
}

fn t_test_two_sample<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  unequal_variance: bool,
) -> Result<(f64, f64), FormulaErrorValue> {
  let left = evaluator.value_numbers(&args.value(0).ok_or(FormulaErrorValue::Value)?);
  let right = evaluator.value_numbers(&args.value(1).ok_or(FormulaErrorValue::Value)?);
  if left.len() < 2 || right.len() < 2 {
    return Err(FormulaErrorValue::Value);
  }
  let left_count = left.len() as f64;
  let right_count = right.len() as f64;
  let left_mean = mean_number_slice(&left);
  let right_mean = mean_number_slice(&right);
  let left_variance = variance_slice(&left, true).ok_or(FormulaErrorValue::Value)?;
  let right_variance = variance_slice(&right, true).ok_or(FormulaErrorValue::Value)?;
  let (standard_error, degrees_freedom) = if unequal_variance {
    let left_error = left_variance / left_count;
    let right_error = right_variance / right_count;
    let denominator = left_error * left_error / (left_count - 1.0)
      + right_error * right_error / (right_count - 1.0);
    if denominator == 0.0 {
      return Err(FormulaErrorValue::Div0);
    }
    (
      (left_error + right_error).sqrt(),
      (left_error + right_error) * (left_error + right_error) / denominator,
    )
  } else {
    let pooled = ((left_count - 1.0) * left_variance + (right_count - 1.0) * right_variance)
      / (left_count + right_count - 2.0);
    (
      (pooled * (1.0 / left_count + 1.0 / right_count)).sqrt(),
      left_count + right_count - 2.0,
    )
  };
  if standard_error == 0.0 {
    return Err(FormulaErrorValue::Div0);
  }
  Ok((
    (left_mean - right_mean).abs() / standard_error,
    degrees_freedom,
  ))
}

fn mean_number_slice(values: &[f64]) -> f64 {
  kahan_sum(values.iter().copied()) / values.len() as f64
}

fn student_t_probability(t: f64, degrees_freedom: f64, tails: i32) -> Option<f64> {
  if degrees_freedom <= 0.0 {
    return None;
  }
  let distribution = StudentsT::new(0.0, 1.0, degrees_freedom).ok()?;
  let right_tail = 1.0 - distribution.cdf(t.abs());
  Some(if tails == 2 {
    2.0 * right_tail
  } else {
    right_tail
  })
}

fn evaluate_z_test_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let values = evaluator.value_numbers(&args.value(0)?);
  if values.len() <= 1 {
    return Some(FormulaValue::Error(FormulaErrorValue::Div0));
  }
  let x = evaluator.number(&args.value(1)?)?;
  let sigma = if let Some(value) = args.raw_arg(2).and_then(|_| args.value(2)) {
    let sigma = evaluator.number(&value)?;
    if sigma <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    sigma
  } else {
    let variance = variance_slice(&values, true).unwrap_or(0.0);
    if variance == 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    variance.sqrt()
  };
  if sigma <= 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let mean = mean_number_slice(&values);
  let z = (mean - x) / (sigma / (values.len() as f64).sqrt());
  Some(FormulaValue::Number(0.5 - lo_gauss(z)))
}

fn evaluate_frequency_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let data = evaluator.value_numbers(&args.array_value(0)?);
  let bins = evaluator.value_numbers(&args.array_value(1)?);
  frequency_counts(data, &bins)
    .map(|counts| {
      FormulaValue::Matrix(
        counts
          .into_iter()
          .map(|count| vec![FormulaValue::Number(count as f64)])
          .collect(),
      )
    })
    .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
}

fn evaluate_hypgeom_dist_reader<'doc>(
  _evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let Some(sample_success) = args.scalar_number(0) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  let Some(sample_size) = args.scalar_number(1) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  let Some(population_success) = args.scalar_number(2) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  let Some(population_size) = args.scalar_number(3) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  let cumulative = args
    .raw_arg(4)
    .and_then(|_| args.value(4))
    .map(|value| match value {
      FormulaValue::Boolean(value) => Some(value),
      FormulaValue::Number(value) if value == 0.0 || value == 1.0 => Some(value != 0.0),
      _ => None,
    })
    .unwrap_or(Some(false));
  let Some(cumulative) = cumulative else {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  };
  let sample_success = sample_success.floor();
  let sample_size = sample_size.floor();
  let population_success = population_success.floor();
  let population_size = population_size.floor();
  if sample_success < 0.0
    || sample_size < 0.0
    || population_success < 0.0
    || population_size < 0.0
    || sample_success > sample_size
    || sample_size > population_size
    || population_success > population_size
  {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let dist = Hypergeometric::new(
    population_size as u64,
    population_success as u64,
    sample_size as u64,
  )
  .ok()?;
  Some(FormulaValue::Number(if cumulative {
    dist.cdf(sample_success as u64)
  } else {
    dist.pmf(sample_success as u64)
  }))
}

fn evaluate_lognorm_inv_reader<'doc>(
  _evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let Some(p) = args.scalar_number(0) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  let Some(mean) = args.scalar_number(1) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  let Some(sigma) = args.scalar_number(2) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  if p <= 0.0 || p >= 1.0 || sigma <= 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  Some(FormulaValue::Number(
    LogNormal::new(mean, sigma).ok()?.inverse_cdf(p),
  ))
}

fn evaluate_norm_inv_reader<'doc>(
  _evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  standard: bool,
) -> Option<FormulaValue<'doc>> {
  let Some(p) = args.scalar_number(0) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  let (mean, sigma) = if standard {
    (0.0, 1.0)
  } else {
    let Some(mean) = args.scalar_number(1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(sigma) = args.scalar_number(2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    (mean, sigma)
  };
  if sigma <= 0.0 || !(0.0..=1.0).contains(&p) {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  if p == 0.0 || p == 1.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  if standard {
    Some(FormulaValue::Number(norm_s_inv(p)))
  } else {
    Some(FormulaValue::Number(
      Normal::new(mean, sigma).ok()?.inverse_cdf(p),
    ))
  }
}

fn evaluate_confidence_reader<'doc>(
  _evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  t_dist: bool,
) -> Option<FormulaValue<'doc>> {
  let Some(alpha) = args.scalar_number(0) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  let Some(sigma) = args.scalar_number(1) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  let Some(size) = args.scalar_number(2) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  if !alpha.is_finite()
    || !sigma.is_finite()
    || !size.is_finite()
    || alpha <= 0.0
    || alpha >= 1.0
    || sigma <= 0.0
    || size.floor() < 1.0
  {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
  let size = size.floor();
  if t_dist {
    if size == 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    let dist = StudentsT::new(0.0, 1.0, size - 1.0).ok()?;
    Some(FormulaValue::Number(
      dist.inverse_cdf(1.0 - alpha / 2.0).abs() * sigma / size.sqrt(),
    ))
  } else {
    Some(FormulaValue::Number(
      norm_s_inv(1.0 - alpha / 2.0).abs() * sigma / size.sqrt(),
    ))
  }
}

fn evaluate_fvschedule_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  if args.is_missing(0) || args.is_missing(1) {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  }
  let mut value = evaluator.number(&args.value(0)?)?;
  let schedule = args.value(1)?;
  if matches!(schedule, FormulaValue::Blank) {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  }
  for rate in evaluator.value_numbers(&schedule) {
    value *= 1.0 + rate;
  }
  Some(FormulaValue::Number(value))
}

fn evaluate_npv_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let rate = evaluator.number(&args.value(0)?)?;
  if rate == -1.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Div0));
  }
  let mut total = KahanSum::default();
  let mut period = 1.0;
  for index in 1..args.len() {
    for value in evaluator.value_numbers(&args.value(index)?) {
      total.add(value / (1.0 + rate).powf(period));
      period += 1.0;
    }
  }
  Some(FormulaValue::Number(total.finish()))
}

fn evaluate_randarray_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let rows = optional_positive_integer(evaluator, args, 0, 1)?;
  let columns = optional_positive_integer(evaluator, args, 1, 1)?;
  let min = optional_number(evaluator, args, 2, 0.0)?;
  let max = optional_number(evaluator, args, 3, 1.0)?;
  let integer = args
    .raw_arg(4)
    .filter(|_| !args.is_missing(4))
    .and_then(|_| args.value(4))
    .is_some_and(|value| evaluator.truthy(&value));
  if min > max {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let value = if integer { min.ceil() } else { min };
  if integer && value > max {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let fill = FormulaValue::Number(value);
  Some(FormulaValue::Matrix(
    (0..rows)
      .map(|_| (0..columns).map(|_| fill.clone()).collect())
      .collect(),
  ))
}

fn optional_positive_integer<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
  default: usize,
) -> Option<usize> {
  if args.raw_arg(index).is_none() {
    return Some(default);
  }
  if args.is_missing(index) {
    return Some(default);
  }
  let value = evaluator.number(&args.value(index)?)?.floor();
  if value < 1.0 || value > usize::MAX as f64 {
    return None;
  }
  Some(value as usize)
}

fn optional_number<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
  default: f64,
) -> Option<f64> {
  if args.raw_arg(index).is_none() || args.is_missing(index) {
    return Some(default);
  }
  evaluator.number(&args.value(index)?)
}

fn evaluate_price_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  if (0..6).any(|index| args.is_missing(index)) {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let settle = evaluator.date_number_from_value(&args.value(0)?)?.floor() as i32;
  let maturity = evaluator.date_number_from_value(&args.value(1)?)?.floor() as i32;
  let rate = evaluator.number(&args.value(2)?)?;
  let yield_value = evaluator.number(&args.value(3)?)?;
  let redemption = evaluator.number(&args.value(4)?)?;
  let frequency = evaluator.number(&args.value(5)?)?.floor() as i32;
  let basis = args
    .raw_arg(6)
    .filter(|_| !args.is_missing(6))
    .and_then(|_| args.value(6))
    .and_then(|value| evaluator.number(&value))
    .map(|value| value.floor() as i32)
    .unwrap_or(0);
  if settle >= maturity
    || rate < 0.0
    || yield_value < 0.0
    || redemption <= 0.0
    || !matches!(frequency, 1 | 2 | 4)
    || !(0..=4).contains(&basis)
  {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  finance_price(
    settle,
    maturity,
    rate,
    yield_value,
    redemption,
    frequency,
    basis,
  )
  .map(FormulaValue::Number)
  .or(Some(FormulaValue::Error(
    FormulaErrorValue::IllegalArgument,
  )))
}

fn evaluate_mduration_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  if (0..5).any(|index| args.is_missing(index)) {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let settle = evaluator.date_number_from_value(&args.value(0)?)?.floor() as i32;
  let maturity = evaluator.date_number_from_value(&args.value(1)?)?.floor() as i32;
  let coupon = evaluator.number(&args.value(2)?)?;
  let yield_value = evaluator.number(&args.value(3)?)?;
  let frequency = evaluator.number(&args.value(4)?)?.floor() as i32;
  let basis = args
    .raw_arg(5)
    .filter(|_| !args.is_missing(5))
    .and_then(|_| args.value(5))
    .and_then(|value| evaluator.number(&value))
    .map(|value| value.floor() as i32)
    .unwrap_or(0);
  if settle >= maturity
    || coupon < 0.0
    || yield_value < 0.0
    || !matches!(frequency, 1 | 2 | 4)
    || !(0..=4).contains(&basis)
  {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let duration = finance_duration(settle, maturity, coupon, yield_value, frequency, basis)?;
  Some(FormulaValue::Number(
    duration / (1.0 + yield_value / f64::from(frequency)),
  ))
}

fn evaluate_replaceb_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let old = evaluator.text(&args.value(0)?);
  let start = evaluator.number(&args.value(1)?)?.floor() as i32;
  let count = evaluator.number(&args.value(2)?)?.floor() as i32;
  let new = evaluator.text(&args.value(3)?);
  let len = text_byte_len(&old) as i32;
  if start < 1 || start > len || count < 0 || start + count - 1 > len {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let prefix = leftb(&old, start as usize - 1);
  let suffix = rightb(&old, (len - start - count + 1) as usize);
  Some(FormulaValue::String(Cow::Owned(format!(
    "{prefix}{new}{suffix}"
  ))))
}

fn evaluate_pmt_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let rate = evaluator.number(&args.value(0)?)?;
  let nper = evaluator.number(&args.value(1)?)?;
  let pv = evaluator.number(&args.value(2)?)?;
  let fv = args
    .raw_arg(3)
    .and_then(|_| args.value(3))
    .and_then(|value| evaluator.number(&value))
    .unwrap_or(0.0);
  let pay_in_advance = args
    .raw_arg(4)
    .and_then(|_| args.value(4))
    .is_some_and(|value| evaluator.truthy(&value));
  let result = financial_pmt(rate, nper, pv, fv, pay_in_advance);
  if result.is_finite() {
    Some(FormulaValue::Number(result))
  } else {
    Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
  }
}

fn evaluate_pv_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let rate = evaluator.number(&args.value(0)?)?;
  let nper = evaluator.number(&args.value(1)?)?;
  let pmt = evaluator.number(&args.value(2)?)?;
  let fv = args
    .raw_arg(3)
    .and_then(|_| args.value(3))
    .and_then(|value| evaluator.number(&value))
    .unwrap_or(0.0);
  let pay_in_advance = args
    .raw_arg(4)
    .and_then(|_| args.value(4))
    .is_some_and(|value| evaluator.truthy(&value));
  let result = if rate == 0.0 {
    -((nper * pmt) + fv)
  } else {
    let rate1 = rate + 1.0;
    (((1.0 - rate1.powf(nper)) / rate) * if pay_in_advance { rate1 } else { 1.0 } * pmt - fv)
      / rate1.powf(nper)
  };
  if result.is_finite() {
    Some(FormulaValue::Number(result))
  } else {
    Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
  }
}

fn evaluate_ispmt_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let rate = evaluator.number(&args.value(0)?)?;
  let period = evaluator.number(&args.value(1)?)?;
  let nper = evaluator.number(&args.value(2)?)?;
  let pv = evaluator.number(&args.value(3)?)?;
  if nper == 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  Some(FormulaValue::Number(pv * rate * (period / nper - 1.0)))
}

fn evaluate_rri_reader<'doc>(
  _evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let periods = args.scalar_number(0)?;
  let present = args.scalar_number(1)?;
  let future = args.scalar_number(2)?;
  if periods <= 0.0 || present == 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let result = (future / present).powf(1.0 / periods) - 1.0;
  if result.is_finite() {
    Some(FormulaValue::Number(result))
  } else {
    Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
  }
}

fn evaluate_dollar_decimal_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  fractional_to_decimal: bool,
) -> Option<FormulaValue<'doc>> {
  let value = evaluator.number(&args.value(0)?)?;
  let fraction = evaluator.number(&args.value(1)?)?.trunc();
  if fraction < 1.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let integer = value.trunc();
  let decimal = value - integer;
  let result = if fractional_to_decimal {
    integer + decimal / fraction * 10.0_f64.powf(fraction.log10().ceil())
  } else {
    integer + decimal * fraction * 10.0_f64.powf(-fraction.log10().ceil())
  };
  Some(FormulaValue::Number(result))
}

fn evaluate_delta_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let left = evaluator.number(&args.value(0)?)?;
  let right = if args.len() == 2 {
    evaluator.number(&args.value(1)?)?
  } else {
    0.0
  };
  Some(FormulaValue::Number(if left == right { 1.0 } else { 0.0 }))
}

fn evaluate_covariance_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  sample: bool,
) -> Option<FormulaValue<'doc>> {
  let left = evaluator.matrix_values(&args.value(0)?);
  let right = evaluator.matrix_values(&args.value(1)?);
  let Some(pairs) = covariance_pairs(&left, &right) else {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  };
  covariance(&pairs, sample)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
}

fn evaluate_complex_div_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let left = parse_complex_number(&evaluator.text(&args.value(0)?))?;
  let right = parse_complex_number(&evaluator.text(&args.value(1)?))?;
  if right.value().norm_sqr() == 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Div0));
  }
  let result = FormulaComplex::from_value(left.value() / right.value(), binary_suffix(left, right));
  Some(FormulaValue::String(Cow::Owned(format_complex_result(
    result,
  ))))
}

fn matrix_number_values<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
) -> Option<Vec<Vec<f64>>> {
  evaluator
    .matrix_values(value)
    .into_iter()
    .map(|row| {
      row
        .into_iter()
        .map(|value| evaluator.number(&value).ok_or(()))
        .collect::<Result<Vec<_>, _>>()
        .ok()
    })
    .collect()
}

fn matrix_number_values_strict<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
) -> Option<Vec<Vec<f64>>> {
  evaluator
    .matrix_values(value)
    .into_iter()
    .map(|row| {
      row
        .into_iter()
        .map(|value| match value {
          FormulaValue::Number(value) => Ok(value),
          FormulaValue::Boolean(value) => Ok(if value { 1.0 } else { 0.0 }),
          _ => Err(()),
        })
        .collect::<Result<Vec<_>, _>>()
        .ok()
    })
    .collect()
}

fn matrix_item<'doc>(
  matrix: &'doc [Vec<FormulaValue<'doc>>],
  row: usize,
  column: usize,
) -> Option<&'doc FormulaValue<'doc>> {
  if matrix.len() == 1 && matrix.first()?.len() == 1 {
    return matrix.first()?.first();
  }
  if matrix.len() == 1 {
    return matrix.first()?.get(column);
  }
  if matrix.first()?.len() == 1 {
    return matrix.get(row)?.first();
  }
  matrix.get(row)?.get(column)
}

fn chisq_inv_value<'doc>(p: f64, df: f64, right_tail: bool) -> FormulaValue<'doc> {
  if df < 1.0 || (right_tail && (p <= 0.0 || p > 1.0)) || (!right_tail && !(0.0..1.0).contains(&p))
  {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  let result = if right_tail {
    lo_iterate_inverse(|x| p - lo_chi_dist(x, df), df * 0.5, df)
  } else {
    lo_iterate_inverse(|x| p - lo_chisq_dist_cdf(x, df), df * 0.5, df)
  };
  match result {
    Ok(value) => FormulaValue::Number(value),
    Err(error) => FormulaValue::Error(special_error_value(error)),
  }
}

fn covariance_pairs(
  left: &[Vec<FormulaValue<'_>>],
  right: &[Vec<FormulaValue<'_>>],
) -> Option<Vec<(f64, f64)>> {
  if left.len() != right.len()
    || left
      .iter()
      .zip(right)
      .any(|(left, right)| left.len() != right.len())
  {
    return None;
  }
  let pairs = left
    .iter()
    .flatten()
    .zip(right.iter().flatten())
    .filter_map(|(left, right)| {
      let left = number_only(left)?;
      let right = number_only(right)?;
      Some((left, right))
    })
    .collect::<Vec<_>>();
  Some(pairs)
}

fn number_only(value: &FormulaValue<'_>) -> Option<f64> {
  match value {
    FormulaValue::Number(value) => Some(*value),
    FormulaValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
    _ => None,
  }
}

pub(crate) fn format_complex_result(value: FormulaComplex) -> String {
  format_formula_complex_number(value, format_complex_component)
}

fn format_complex_component(value: f64, leading_sign: bool) -> String {
  if !value.is_finite() {
    return "#VALUE!".to_string();
  }
  let mut output = format_general_significant(value, 15);
  if leading_sign && value > 0.0 && !output.starts_with('+') {
    output.insert(0, '+');
  }
  output
}

fn format_general_significant(value: f64, precision: i32) -> String {
  if value == 0.0 {
    return "0".to_string();
  }
  let exponent = value.abs().log10().floor() as i32;
  if exponent >= -4 && exponent < precision {
    let decimals = (precision - exponent - 1).max(0) as usize;
    return trim_float_text(&format!("{value:.decimals$}"));
  }
  let decimals = (precision - 1).max(0) as usize;
  let output = format!("{value:.decimals$e}");
  let Some((mantissa, exponent)) = output.split_once('e') else {
    return trim_float_text(&output);
  };
  let mantissa = trim_float_text(mantissa);
  let exponent_value = exponent.parse::<i32>().unwrap_or(0);
  format!("{mantissa}e{exponent_value:+}")
}

fn trim_float_text(value: &str) -> String {
  let trimmed = value.trim_end_matches('0').trim_end_matches('.');
  if trimmed == "-0" {
    "0".to_string()
  } else {
    trimmed.to_string()
  }
}

fn choose_row_column_index(index: i64, len: usize) -> Option<usize> {
  if index == 0 {
    return None;
  }
  if index > 0 {
    (index as usize).checked_sub(1).filter(|index| *index < len)
  } else {
    len.checked_sub(index.unsigned_abs() as usize)
  }
}

#[derive(Clone, Copy)]
enum LogicalAggregate {
  And,
  Or,
  Xor,
}

fn evaluate_logical_aggregate_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  aggregate: LogicalAggregate,
) -> Option<FormulaValue<'doc>> {
  let mut any = false;
  let mut value = matches!(aggregate, LogicalAggregate::And);
  let mut true_count = 0usize;
  for index in 0..args.len() {
    let arg = args.value(index)?;
    let values = match arg {
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_) => evaluator
        .matrix_values(&arg)
        .into_iter()
        .flatten()
        .collect::<Vec<_>>(),
      value => vec![value],
    };
    for item in values {
      match item {
        FormulaValue::Error(error) => return Some(FormulaValue::Error(error)),
        FormulaValue::Boolean(item) => {
          any = true;
          match aggregate {
            LogicalAggregate::And => value &= item,
            LogicalAggregate::Or => value |= item,
            LogicalAggregate::Xor => true_count += usize::from(item),
          }
        }
        FormulaValue::Number(item) => {
          any = true;
          let item = item != 0.0;
          match aggregate {
            LogicalAggregate::And => value &= item,
            LogicalAggregate::Or => value |= item,
            LogicalAggregate::Xor => true_count += usize::from(item),
          }
        }
        FormulaValue::Blank | FormulaValue::String(_) => {}
        FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => {}
      }
    }
  }
  if !any {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  Some(FormulaValue::Boolean(match aggregate {
    LogicalAggregate::And | LogicalAggregate::Or => value,
    LogicalAggregate::Xor => true_count % 2 == 1,
  }))
}

fn numeric_error_value(error: NumericError) -> FormulaErrorValue {
  match error {
    NumericError::Div0 => FormulaErrorValue::Div0,
    NumericError::Value => FormulaErrorValue::Value,
    NumericError::IllegalArgument => FormulaErrorValue::IllegalArgument,
  }
}

fn statistics_error_value(error: StatisticsError) -> FormulaErrorValue {
  match error {
    StatisticsError::Div0 => FormulaErrorValue::Div0,
    StatisticsError::IllegalArgument => FormulaErrorValue::IllegalArgument,
  }
}

fn special_error_value(error: SpecialError) -> FormulaErrorValue {
  match error {
    SpecialError::IllegalArgument => FormulaErrorValue::IllegalArgument,
    SpecialError::Num => FormulaErrorValue::Num,
    SpecialError::Div0 => FormulaErrorValue::Div0,
  }
}

fn scalar_numeric_unary_arg<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  op: impl Fn(f64) -> f64,
) -> Option<FormulaValue<'doc>> {
  scalar_numeric_unary_checked_arg(
    evaluator,
    args,
    |value| Some(op(value)),
    FormulaErrorValue::Unknown,
  )
}

fn scalar_numeric_unary_checked_arg<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  op: impl Fn(f64) -> Option<f64>,
  error: FormulaErrorValue,
) -> Option<FormulaValue<'doc>> {
  let value = args.value(0)?;
  if (evaluator.array_context
    && matches!(
      value,
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
    ))
    || matches!(value, FormulaValue::Matrix(_))
  {
    return evaluator.map_unary_values(value, |evaluator, value| {
      if let FormulaValue::Error(error) = value {
        return Some(FormulaValue::Error(*error));
      }
      evaluator.number(value).map(|value| match op(value) {
        Some(result) if result.is_finite() => FormulaValue::Number(result),
        Some(_) => FormulaValue::Error(FormulaErrorValue::Value),
        None => FormulaValue::Error(error),
      })
    });
  }
  let value = evaluator.scalar_binary_operand(value);
  if let FormulaValue::Error(error) = value {
    return Some(FormulaValue::Error(error));
  }
  let value = evaluator.number(&value)?;
  Some(match op(value) {
    Some(result) if result.is_finite() => FormulaValue::Number(result),
    Some(_) => FormulaValue::Error(FormulaErrorValue::Value),
    None => FormulaValue::Error(error),
  })
}

fn is_matrix_argument(value: &FormulaValue<'_>) -> bool {
  matches!(
    value,
    FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
  )
}

fn single_raw_reference<'doc>(
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
) -> Option<QualifiedRange<'doc>> {
  let arg = args.raw_arg(index)?;
  let ops = &arg.ops[arg.range.start..arg.range.end];
  match ops {
    [FormulaOp::PushReference(reference)] => Some(reference.clone()),
    _ => None,
  }
}

fn raw_row_column_function_value<'doc>(
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
  column: bool,
) -> Option<FormulaValue<'doc>> {
  let arg = args.raw_arg(index)?;
  let ops = &arg.ops[arg.range.start..arg.range.end];
  let [
    FormulaOp::PushReference(reference),
    FormulaOp::Call { function, argc, .. },
  ] = ops
  else {
    return None;
  };
  let expected = if column {
    FormulaFunctionId::Column
  } else {
    FormulaFunctionId::Row
  };
  if *argc != 1 || *function != Some(expected) {
    return None;
  }
  let range = reference.range;
  let start_column = range.start.column.min(range.end.column);
  let end_column = range.start.column.max(range.end.column);
  let start_row = range.start.row.min(range.end.row);
  let end_row = range.start.row.max(range.end.row);
  if column && end_column > start_column {
    return Some(FormulaValue::Matrix(vec![
      (start_column..=end_column)
        .map(|column| FormulaValue::Number(column as f64 + 1.0))
        .collect(),
    ]));
  }
  if !column && end_row > start_row {
    return Some(FormulaValue::Matrix(
      (start_row..=end_row)
        .map(|row| vec![FormulaValue::Number(row as f64 + 1.0)])
        .collect(),
    ));
  }
  Some(FormulaValue::Number(if column {
    start_column as f64 + 1.0
  } else {
    start_row as f64 + 1.0
  }))
}

fn is_multicell_scalar_argument(value: &FormulaValue<'_>) -> bool {
  match value {
    FormulaValue::Reference(reference) => reference.range.cell_count_hint() != 1,
    FormulaValue::RefList(_) => true,
    FormulaValue::Matrix(rows) => rows.len() != 1 || rows.first().is_none_or(|row| row.len() != 1),
    _ => false,
  }
}

fn expand_two_digit_year(year: i32) -> i32 {
  if year >= 30 { 1900 + year } else { 2000 + year }
}

fn quote_sheet_name_for_reference(sheet: &str) -> String {
  if sheet
    .chars()
    .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
  {
    return sheet.to_string();
  }
  format!("'{}'", sheet.replace('\'', "''"))
}

fn index_matrix<'doc>(
  rows: Vec<Vec<FormulaValue<'doc>>>,
  row: u32,
  column: u32,
  arg_count: usize,
) -> FormulaValue<'doc> {
  let height = rows.len();
  let width = rows.iter().map(Vec::len).max().unwrap_or(0);
  let row_vector_special = arg_count == 2 || column == 0;
  let row_vector_element = height == 1 && (column != 0 || (row_vector_special && row != 0));
  let vector_element = row_vector_element || (width == 1 && row != 0);
  if height == 0
    || width == 0
    || (!vector_element && (column as usize > width || row as usize > height))
  {
    return FormulaValue::Error(FormulaErrorValue::Ref);
  }
  if row == 0 && column == 0 {
    return FormulaValue::Matrix(rows);
  }
  if vector_element {
    let (element, other_dimension) = if row_vector_element && !row_vector_special {
      (column as usize, row as usize)
    } else {
      (row as usize, column as usize)
    };
    if element == 0 || element > width * height || other_dimension > 1 {
      return FormulaValue::Error(FormulaErrorValue::Ref);
    }
    let index = element - 1;
    let row_index = index / width;
    let column_index = index % width;
    return rows
      .get(row_index)
      .and_then(|row| row.get(column_index))
      .cloned()
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::Ref));
  }
  if column == 0 {
    let row_index = row as usize - 1;
    return rows
      .get(row_index)
      .map(|row| FormulaValue::Matrix(vec![row.clone()]))
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::Ref));
  }
  if row == 0 {
    let column_index = column as usize - 1;
    let values = rows
      .into_iter()
      .map(|row| {
        vec![
          row
            .get(column_index)
            .cloned()
            .unwrap_or(FormulaValue::Error(FormulaErrorValue::Ref)),
        ]
      })
      .collect();
    return FormulaValue::Matrix(values);
  }
  rows
    .get(row as usize - 1)
    .and_then(|row_values| row_values.get(column as usize - 1))
    .cloned()
    .unwrap_or(FormulaValue::Error(FormulaErrorValue::Ref))
}
