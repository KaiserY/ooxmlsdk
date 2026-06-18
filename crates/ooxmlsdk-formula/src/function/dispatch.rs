use std::borrow::Cow;

use num_complex::Complex;
use regex::RegexBuilder;
use statrs::distribution::{
  ContinuousCDF, Discrete, DiscreteCDF, Hypergeometric, LogNormal, Normal, StudentsT,
};

use super::{
  EvalContext, FormulaFunctionId, FunctionArgReader, FunctionArgs, resolve_function_name,
};
use crate::calc::combinatorics::{
  combination_count, gcd_number, lcm_number, permutation_count, permutation_with_repetition_count,
};
use crate::calc::complex::{
  FormulaComplex, binary_suffix, format_complex_number as format_formula_complex_number,
  parse_complex_number,
};
use crate::calc::datetime::{
  basis_o_datetime_text, date_from_serial_with_system, date_serial, date_serial_with_system,
  date_to_days, days360_with_system as date_days360, is_leap_year,
  is_valid_libreoffice_gregorian_date, iso_weeknum_from_serial_with_system, last_day_of_month,
  normalized_date_components, weekday_index_from_serial, weeks_in_year_from_serial_with_system,
  weeks_mode_one_index, yearfrac as date_yearfrac,
};
use crate::calc::financial::{
  OddPeriodArgs, finance_accrint, finance_accrintm, finance_coupdaybs, finance_coupdays,
  finance_coupdaysnc, finance_coupncd, finance_coupnum, finance_couppcd, finance_disc,
  finance_duration, finance_intrate, finance_price, finance_pricedisc, finance_pricemat,
  finance_received, finance_tbilleq, finance_tbillprice, finance_tbillyield, finance_yield,
  finance_yielddisc, financial_amordegrc, financial_amorlinc, financial_db, financial_ddb,
  financial_fv, financial_ipmt, financial_irr, financial_mirr, financial_nper, financial_oddlprice,
  financial_oddlyield, financial_pmt, financial_rate, financial_vdb, financial_xirr,
  financial_xnpv,
};
use crate::calc::matrix::{determinant, lup_decompose, lup_solve, matrix_multiply};
use crate::calc::numeric::{
  CeilingFloorKind, KahanSum, NumericError, approx_ceil, approx_floor, ceiling_excel_legacy,
  even_odd, floor_excel_legacy, floor_to_i32, floor_to_u16, floor_to_u32, floor_to_usize,
  kahan_sum, mround, quotient, round_direction, round_to_decimal_places,
  round_to_significant_digits, sign_number,
};
use crate::calc::radix::{
  base_number_text, convert_from_decimal, convert_to_decimal, decimal_text_to_number,
};
use crate::calc::regression::{EtsKind, regression_model, scalar_state as regression_scalar_state};
use crate::calc::special::{
  BesselKind, SpecialError, bessel, erf, erfc, lo_chi_dist, lo_chisq_dist_cdf, lo_chisq_dist_pdf,
  lo_f_dist_pdf, lo_f_dist_right_tail, lo_gamma_dist, lo_gamma_dist_pdf, lo_gauss, lo_integral_phi,
  lo_iterate_inverse, lo_phi, lo_poisson_dist, lo_t_dist, log_gamma, norm_s_inv,
};
use crate::calc::special::{
  gamma as lo_gamma, lo_beta_dist, lo_beta_dist_pdf, lo_binom_dist_pmf, lo_binom_dist_range,
};
use crate::calc::statistics::{
  PercentileKind, StatisticsError, correlation, covariance, deviation_sum_squares,
  frequency_counts, kurtosis, mode_ms_values, mode_slice, percent_rank, percentile_sorted,
  rank_value, skewness, trim_mean, variance_slice,
};
use crate::calc::text::{
  baht_text, clean_formula_text, full_width_like_jis, half_width_like_asc, leftb, legacy_char_text,
  legacy_text_code, proper_formula_text, rightb, roman_text_libreoffice, rot13_formula_text,
  text_byte_len, trim_formula_text,
};
use crate::calc::units::convert_unit;
use crate::code::FormulaOp;
use crate::evaluator::{
  DatabaseFunction, DatePart, IfsAggregate, TimePart, column_index_to_name, compare_text,
  datevalue, display_text_from_value, error_text_value, format_number_with_format_code, rtl_cos,
  rtl_sin, rtl_tan, timevalue,
};
use crate::model::{XLSX_MAX_COLUMN_ZERO_BASED, XLSX_MAX_ROW_ZERO_BASED};
use crate::{
  CellAddress, CellRange, DateSystem, FormulaErrorValue, FormulaGrammar, FormulaOperator,
  FormulaValue, MAX_EXPANDED_RANGE_CELLS, PivotDataRequest, PivotFieldFilter, QualifiedRange,
  SheetId,
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
      if values.is_empty() {
        Some(FormulaValue::Error(FormulaErrorValue::Div0))
      } else {
        let sum = match evaluator.grammar {
          FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1 => values.iter().sum::<f64>(),
          FormulaGrammar::CalcA1 | FormulaGrammar::OpenFormula => kahan_sum(values.iter().copied()),
        };
        Some(FormulaValue::Number(sum / values.len() as f64))
      }
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
    FormulaFunctionId::Mode => evaluate_mode_reader(evaluator, args),
    FormulaFunctionId::ModeDotSngl => evaluate_mode_ms_reader(evaluator, args, true),
    FormulaFunctionId::ModeDotMult => evaluate_mode_ms_reader(evaluator, args, false),
    FormulaFunctionId::Sumif if (2..=3).contains(&args.len()) => evaluator.evaluate_ifs_reader(
      args,
      Some(if args.len() == 3 { 2 } else { 0 }),
      args.len() == 3,
      0,
      2,
      IfsAggregate::Sum,
    ),
    FormulaFunctionId::Countif if args.len() == 2 => {
      evaluator.evaluate_ifs_reader(args, None, false, 0, 2, IfsAggregate::Count)
    }
    FormulaFunctionId::Averageif if (2..=3).contains(&args.len()) => evaluator.evaluate_ifs_reader(
      args,
      Some(if args.len() == 3 { 2 } else { 0 }),
      args.len() == 3,
      0,
      2,
      IfsAggregate::Average,
    ),
    FormulaFunctionId::Sumifs if args.len() >= 3 && !args.len().is_multiple_of(2) => {
      evaluator.evaluate_ifs_reader(args, Some(0), false, 1, args.len() - 1, IfsAggregate::Sum)
    }
    FormulaFunctionId::Countifs if args.len() >= 2 && args.len().is_multiple_of(2) => {
      evaluator.evaluate_ifs_reader(args, None, false, 0, args.len(), IfsAggregate::Count)
    }
    FormulaFunctionId::Averageifs if args.len() >= 3 && !args.len().is_multiple_of(2) => evaluator
      .evaluate_ifs_reader(
        args,
        Some(0),
        false,
        1,
        args.len() - 1,
        IfsAggregate::Average,
      ),
    FormulaFunctionId::Maxifs if args.len() >= 3 && !args.len().is_multiple_of(2) => {
      evaluator.evaluate_ifs_reader(args, Some(0), false, 1, args.len() - 1, IfsAggregate::Max)
    }
    FormulaFunctionId::Minifs if args.len() >= 3 && !args.len().is_multiple_of(2) => {
      evaluator.evaluate_ifs_reader(args, Some(0), false, 1, args.len() - 1, IfsAggregate::Min)
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
      information_logical_value(evaluator, args.first_value()?),
      Some(FormulaValue::String(_))
    ))),
    FormulaFunctionId::Isnontext if args.len() == 1 => Some(FormulaValue::Boolean(!matches!(
      information_logical_value(evaluator, args.first_value()?),
      Some(FormulaValue::String(_))
    ))),
    FormulaFunctionId::Islogical if args.len() == 1 => Some(FormulaValue::Boolean(matches!(
      information_logical_value(evaluator, args.first_value()?),
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
    FormulaFunctionId::Text if args.len() == 2 => evaluate_text_reader(evaluator, args),
    FormulaFunctionId::Concat => evaluate_concat_reader(evaluator, args),
    FormulaFunctionId::Concatenate => evaluate_concatenate_reader(evaluator, args),
    FormulaFunctionId::Textjoin => evaluate_textjoin_reader(evaluator, args),
    FormulaFunctionId::Textafter if (1..=6).contains(&args.len()) => {
      evaluate_text_before_after_reader(evaluator, args, false)
    }
    FormulaFunctionId::Textbefore if (1..=6).contains(&args.len()) => {
      evaluate_text_before_after_reader(evaluator, args, true)
    }
    FormulaFunctionId::Textsplit if (1..=6).contains(&args.len()) => {
      evaluate_textsplit_reader(evaluator, args)
    }
    FormulaFunctionId::Regex if (2..=4).contains(&args.len()) => {
      evaluate_regex_reader(evaluator, args)
    }
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
    FormulaFunctionId::Proper if args.len() == 1 => {
      let text = match strict_text_arg(evaluator, args.value(0)?) {
        Ok(text) => text,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
      Some(FormulaValue::String(Cow::Owned(proper_formula_text(&text))))
    }
    FormulaFunctionId::Rot13 | FormulaFunctionId::OrgDotOpenofficeDotRot13 if args.len() == 1 => {
      let text = match strict_text_arg(evaluator, args.value(0)?) {
        Ok(text) => text,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
      Some(FormulaValue::String(Cow::Owned(rot13_formula_text(&text))))
    }
    FormulaFunctionId::Upper if args.len() == 1 => {
      let value = args.value(0)?;
      if evaluator.array_context && is_matrix_argument(&value) {
        return map_text_unary_value(evaluator, value, |text| {
          FormulaValue::String(Cow::Owned(text.to_ascii_uppercase()))
        });
      }
      let text = match strict_text_arg(evaluator, args.value(0)?) {
        Ok(text) => text,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
      Some(FormulaValue::String(Cow::Owned(text.to_ascii_uppercase())))
    }
    FormulaFunctionId::Lower if args.len() == 1 => {
      let value = args.value(0)?;
      if evaluator.array_context && is_matrix_argument(&value) {
        return map_text_unary_value(evaluator, value, |text| {
          FormulaValue::String(Cow::Owned(text.to_ascii_lowercase()))
        });
      }
      let text = match strict_text_arg(evaluator, args.value(0)?) {
        Ok(text) => text,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
      Some(FormulaValue::String(Cow::Owned(text.to_ascii_lowercase())))
    }
    FormulaFunctionId::Code if args.len() == 1 => {
      let text = match strict_text_arg(evaluator, args.value(0)?) {
        Ok(text) => text,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
      let Some(code) = text.chars().next().map(legacy_text_code) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      };
      Some(FormulaValue::Number(code as f64))
    }
    FormulaFunctionId::Unicode if args.len() == 1 => {
      let value = args.value(0)?;
      if evaluator.array_context && is_matrix_argument(&value) {
        return map_text_unary_value(evaluator, value, |text| {
          FormulaValue::Number(text.chars().next().map(|ch| ch as u32).unwrap_or(0) as f64)
        });
      }
      let text = match strict_text_arg(evaluator, args.value(0)?) {
        Ok(text) => text,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
      Some(FormulaValue::Number(
        text.chars().next().map(|ch| ch as u32).unwrap_or(0) as f64,
      ))
    }
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
    FormulaFunctionId::Rept if args.len() == 2 => evaluate_rept_reader(evaluator, args),
    FormulaFunctionId::Asc if args.len() == 1 => {
      evaluate_text_transform_reader(evaluator, args, half_width_like_asc)
    }
    FormulaFunctionId::Jis if args.len() == 1 => {
      evaluate_text_transform_reader(evaluator, args, full_width_like_jis)
    }
    FormulaFunctionId::Bahttext if args.len() == 1 => evaluate_bahttext_reader(evaluator, args),
    FormulaFunctionId::Roman if (1..=2).contains(&args.len()) => {
      evaluate_roman_reader(evaluator, args)
    }
    FormulaFunctionId::Arabic if args.len() == 1 => evaluate_arabic_reader(evaluator, args),
    FormulaFunctionId::Abs if args.len() == 1 => {
      let value = args.value(0)?;
      if matches!(value, FormulaValue::Matrix(_))
        || (evaluator.array_context && is_matrix_argument(&value))
      {
        evaluator.map_unary_values(value, |evaluator, value| {
          evaluator
            .number(value)
            .map(|value| FormulaValue::Number(value.abs()))
        })
      } else {
        let value = if evaluator.current_cell.is_some() {
          evaluator.scalar_binary_operand(value)
        } else {
          evaluator.first_value(&value)
        };
        if let FormulaValue::Error(error) = value {
          return Some(FormulaValue::Error(error));
        }
        evaluator
          .number(&value)
          .map(|value| FormulaValue::Number(value.abs()))
          .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
      }
    }
    FormulaFunctionId::Sign if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, sign_number)
    }
    FormulaFunctionId::Int if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, approx_floor)
    }
    FormulaFunctionId::Sqrt if args.len() == 1 => {
      let value = match scalar_number_arg_or_value(evaluator, args, 0)? {
        Ok(value) => value,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
      if value < 0.0 {
        Some(FormulaValue::Error(FormulaErrorValue::Num))
      } else {
        Some(FormulaValue::Number(value.sqrt()))
      }
    }
    FormulaFunctionId::Power if args.len() == 2 => {
      let value = match scalar_number_arg_or_value(evaluator, args, 0)? {
        Ok(value) => value,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
      let power = match scalar_number_arg_or_value(evaluator, args, 1)? {
        Ok(value) => value,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
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
    FormulaFunctionId::Iseven if args.len() == 1 => {
      let value = args.value(0)?;
      if evaluator.array_context && is_matrix_argument(&value) {
        return evaluator.map_unary_values(value, |evaluator, value| {
          if let FormulaValue::Error(error) = value {
            return Some(FormulaValue::Error(*error));
          }
          let Some(value) = evaluator.number(value) else {
            return Some(FormulaValue::Error(FormulaErrorValue::Value));
          };
          Some(FormulaValue::Boolean(
            approx_floor(value.abs()) as i64 % 2 == 0,
          ))
        });
      }
      let value = match scalar_number_arg_or_value(evaluator, args, 0)? {
        Ok(value) => value,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
      Some(FormulaValue::Boolean(
        approx_floor(value.abs()) as i64 % 2 == 0,
      ))
    }
    FormulaFunctionId::Isodd if args.len() == 1 => {
      let value = args.value(0)?;
      if evaluator.array_context && is_matrix_argument(&value) {
        return evaluator.map_unary_values(value, |evaluator, value| {
          if let FormulaValue::Error(error) = value {
            return Some(FormulaValue::Error(*error));
          }
          let Some(value) = evaluator.number(value) else {
            return Some(FormulaValue::Error(FormulaErrorValue::Value));
          };
          Some(FormulaValue::Boolean(
            approx_floor(value.abs()) as i64 % 2 != 0,
          ))
        });
      }
      let value = match scalar_number_arg_or_value(evaluator, args, 0)? {
        Ok(value) => value,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
      Some(FormulaValue::Boolean(
        approx_floor(value.abs()) as i64 % 2 != 0,
      ))
    }
    FormulaFunctionId::Rawsubtract if args.len() >= 2 => {
      evaluate_rawsubtract_reader(evaluator, args)
    }
    FormulaFunctionId::Rawsubtract => Some(FormulaValue::Error(FormulaErrorValue::Value)),
    FormulaFunctionId::Sqrtpi if args.len() == 1 => {
      let Some(value) = args.scalar_number(0) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      };
      if value < 0.0 {
        Some(FormulaValue::Error(FormulaErrorValue::Num))
      } else {
        Some(FormulaValue::Number((value * std::f64::consts::PI).sqrt()))
      }
    }
    FormulaFunctionId::Sqrtpi => Some(FormulaValue::Error(FormulaErrorValue::Value)),
    FormulaFunctionId::Fact if args.len() == 1 => evaluate_factorial_reader(evaluator, args, false),
    FormulaFunctionId::Factdouble if args.len() == 1 => {
      evaluate_factorial_reader(evaluator, args, true)
    }
    FormulaFunctionId::Fact | FormulaFunctionId::Factdouble => {
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    }
    FormulaFunctionId::Multinomial => evaluate_multinomial_reader(args),
    FormulaFunctionId::Seriessum if args.len() == 4 => evaluate_seriessum_reader(evaluator, args),
    FormulaFunctionId::Seriessum => Some(FormulaValue::Error(FormulaErrorValue::Value)),
    FormulaFunctionId::Radians if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, f64::to_radians)
    }
    FormulaFunctionId::Radians => Some(FormulaValue::Error(FormulaErrorValue::Value)),
    FormulaFunctionId::Degrees if args.len() == 1 => {
      scalar_numeric_unary_arg(evaluator, args, f64::to_degrees)
    }
    FormulaFunctionId::Degrees => Some(FormulaValue::Error(FormulaErrorValue::Value)),
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
    FormulaFunctionId::Atan2 if args.len() == 2 => {
      let atan2_arg = |value: FormulaValue<'doc>| match value {
        FormulaValue::Reference(reference) => evaluator
          .implicit_intersection_value(&reference)
          .unwrap_or(FormulaValue::Error(FormulaErrorValue::Value)),
        FormulaValue::RefList(ranges) if ranges.len() == 1 => evaluator
          .implicit_intersection_value(&ranges[0])
          .unwrap_or(FormulaValue::Error(FormulaErrorValue::Value)),
        FormulaValue::RefList(_) => FormulaValue::Error(FormulaErrorValue::Value),
        FormulaValue::Matrix(rows) => {
          if rows.len() == 1 && rows.first().is_some_and(|row| row.len() == 1) {
            rows
              .into_iter()
              .next()
              .and_then(|row| row.into_iter().next())
              .unwrap_or_default()
          } else {
            FormulaValue::Error(FormulaErrorValue::Value)
          }
        }
        value => value,
      };
      let x_value = atan2_arg(args.value(0)?);
      if let FormulaValue::Error(error) = x_value {
        return Some(FormulaValue::Error(error));
      }
      let y_value = atan2_arg(args.value(1)?);
      if let FormulaValue::Error(error) = y_value {
        return Some(FormulaValue::Error(error));
      }
      let Some(x) = evaluator.number(&x_value) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      };
      let Some(y) = evaluator.number(&y_value) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      };
      if x == 0.0
        && y == 0.0
        && matches!(
          evaluator.grammar,
          FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1
        )
      {
        Some(FormulaValue::Error(FormulaErrorValue::Div0))
      } else {
        Some(FormulaValue::Number(y.atan2(x)))
      }
    }
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
      let arg_values = (0..args.len())
        .map(|index| args.value(index))
        .collect::<Option<Vec<_>>>()?;
      if evaluator.array_context && arg_values.iter().any(is_matrix_argument) {
        return if args.len() == 1 {
          map_numeric_array_values(evaluator, &arg_values, FormulaErrorValue::Value, |values| {
            evaluate_log_values(values[0], 10.0)
          })
        } else {
          map_numeric_array_values(evaluator, &arg_values, FormulaErrorValue::Value, |values| {
            evaluate_log_values(values[0], values[1])
          })
        };
      }
      let value = match scalar_number_arg_or_value(evaluator, args, 0)? {
        Ok(value) => value,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
      let base = if args.len() == 2 {
        match scalar_number_arg_or_value(evaluator, args, 1)? {
          Ok(value) => value,
          Err(error) => return Some(FormulaValue::Error(error)),
        }
      } else {
        10.0
      };
      Some(evaluate_log_values(value, base))
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
      let value = match scalar_number_arg_or_value(evaluator, args, 0)? {
        Ok(value) => value,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
      let places = match scalar_number_arg_or_value(evaluator, args, 1)? {
        Ok(value) => value,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
      Some(FormulaValue::Number(round_to_decimal_places(
        value,
        places as i32,
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
        let number = evaluator.scalar_binary_operand(number);
        if let FormulaValue::Error(error) = number {
          return Some(FormulaValue::Error(error));
        }
        let divisor = evaluator.scalar_binary_operand(divisor);
        if let FormulaValue::Error(error) = divisor {
          return Some(FormulaValue::Error(error));
        }
        Some(evaluator.mod_value(&number, &divisor))
      }
    }
    FormulaFunctionId::Quotient if args.len() == 2 => {
      let Some(numerator) = evaluator.number(&args.value(0)?) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      };
      let Some(denominator) = evaluator.number(&args.value(1)?) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      };
      Some(match quotient(numerator, denominator) {
        Ok(value) => FormulaValue::Number(value),
        Err(error) => FormulaValue::Error(numeric_error_value(error)),
      })
    }
    FormulaFunctionId::Trunc if (1..=2).contains(&args.len()) => {
      evaluate_trunc_reader(evaluator, args)
    }
    FormulaFunctionId::Mround if args.len() == 2 => evaluate_mround_reader(evaluator, args),
    FormulaFunctionId::Gcd => evaluate_gcd_lcm_reader(args, true),
    FormulaFunctionId::Lcm => evaluate_gcd_lcm_reader(args, false),
    FormulaFunctionId::Combin if args.len() == 2 => evaluate_combin_reader(evaluator, args, false),
    FormulaFunctionId::Combina if args.len() == 2 => evaluate_combin_reader(evaluator, args, true),
    FormulaFunctionId::Permut if args.len() == 2 => {
      evaluate_permutation_reader(evaluator, args, false)
    }
    FormulaFunctionId::Permutationa if args.len() == 2 => {
      evaluate_permutation_reader(evaluator, args, true)
    }
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
    FormulaFunctionId::Geomean => evaluate_geomean_harmean_reader(evaluator, args, true),
    FormulaFunctionId::Harmean => evaluate_geomean_harmean_reader(evaluator, args, false),
    FormulaFunctionId::Phi if args.len() == 1 => {
      evaluate_simple_stat_unary_reader(evaluator, args, lo_phi)
    }
    FormulaFunctionId::Gauss if args.len() == 1 => {
      evaluate_simple_stat_unary_reader(evaluator, args, lo_gauss)
    }
    FormulaFunctionId::Fisher if args.len() == 1 => evaluate_fisher_reader(evaluator, args),
    FormulaFunctionId::Fisherinv if args.len() == 1 => {
      evaluate_simple_stat_unary_reader(evaluator, args, f64::tanh)
    }
    FormulaFunctionId::Standardize if args.len() == 3 => {
      evaluate_standardize_reader(evaluator, args)
    }
    FormulaFunctionId::GammalnDotPrecise if args.len() == 1 => {
      evaluate_gammaln_reader(evaluator, args)
    }
    FormulaFunctionId::Gamma if args.len() == 1 => evaluate_gamma_reader(evaluator, args),
    FormulaFunctionId::B if (3..=4).contains(&args.len()) => {
      evaluate_b_function_reader(evaluator, args)
    }
    FormulaFunctionId::BetaDotDist if (4..=6).contains(&args.len()) => {
      evaluate_beta_dist_reader(evaluator, args, true)
    }
    FormulaFunctionId::Betadist if (3..=6).contains(&args.len()) => {
      evaluate_beta_dist_reader(evaluator, args, false)
    }
    FormulaFunctionId::BetaDotInv if (3..=5).contains(&args.len()) => {
      evaluate_beta_inv_reader(evaluator, args)
    }
    FormulaFunctionId::BinomDotDist if args.len() == 4 => {
      evaluate_binom_dist_reader(evaluator, args)
    }
    FormulaFunctionId::BinomDotDistDotRange if (3..=4).contains(&args.len()) => {
      evaluate_binom_dist_range_reader(evaluator, args)
    }
    FormulaFunctionId::BinomDotInv if args.len() == 3 => evaluate_binom_inv_reader(evaluator, args),
    FormulaFunctionId::ChisqDotDist if args.len() == 3 => {
      evaluate_chisq_dist_reader(evaluator, args, true)
    }
    FormulaFunctionId::Chisqdist if (2..=3).contains(&args.len()) => {
      evaluate_chisq_dist_reader(evaluator, args, false)
    }
    FormulaFunctionId::ChisqDotDistDotRt if args.len() == 2 => {
      evaluate_chisq_dist_rt_reader(evaluator, args)
    }
    FormulaFunctionId::ChisqDotInv | FormulaFunctionId::Chisqinv if args.len() == 2 => {
      evaluate_chisq_inv_reader(evaluator, args, false)
    }
    FormulaFunctionId::ExponDotDist if args.len() == 3 => {
      evaluate_expon_dist_reader(evaluator, args)
    }
    FormulaFunctionId::FDotDist if (3..=4).contains(&args.len()) => {
      evaluate_f_dist_reader(evaluator, args)
    }
    FormulaFunctionId::FDotDistDotRt if args.len() == 4 => evaluate_f_dist_reader(evaluator, args),
    FormulaFunctionId::FDotDistDotRt if args.len() == 3 => {
      evaluate_f_dist_rt_reader(evaluator, args)
    }
    FormulaFunctionId::FDotInv if args.len() == 3 => evaluate_f_inv_reader(evaluator, args, false),
    FormulaFunctionId::FDotInvDotRt if args.len() == 3 => {
      evaluate_f_inv_reader(evaluator, args, true)
    }
    FormulaFunctionId::GammaDotDist if args.len() == 4 => {
      evaluate_gamma_dist_reader(evaluator, args, true)
    }
    FormulaFunctionId::Gammadist if (3..=4).contains(&args.len()) => {
      evaluate_gamma_dist_reader(evaluator, args, false)
    }
    FormulaFunctionId::GammaDotInv if args.len() == 3 => evaluate_gamma_inv_reader(evaluator, args),
    FormulaFunctionId::Negbinomdist if args.len() == 3 => {
      evaluate_negbinom_dist_reader(evaluator, args, false)
    }
    FormulaFunctionId::NegbinomDotDist if args.len() == 4 => {
      evaluate_negbinom_dist_reader(evaluator, args, true)
    }
    FormulaFunctionId::TDotDist if args.len() == 3 => evaluate_t_dist_reader(evaluator, args),
    FormulaFunctionId::TDotDistDot2t if args.len() == 2 => {
      evaluate_t_dist_tail_reader(evaluator, args, 2)
    }
    FormulaFunctionId::TDotDistDotRt if args.len() == 2 => {
      evaluate_t_dist_tail_reader(evaluator, args, 1)
    }
    FormulaFunctionId::Tdist if args.len() == 3 => evaluate_tdist_reader(evaluator, args),
    FormulaFunctionId::TDotInv if args.len() == 2 => evaluate_t_inv_reader(evaluator, args, 4),
    FormulaFunctionId::TDotInvDot2t if args.len() == 2 => evaluate_t_inv_reader(evaluator, args, 2),
    FormulaFunctionId::WeibullDotDist if args.len() == 4 => {
      evaluate_weibull_dist_reader(evaluator, args)
    }
    FormulaFunctionId::B
    | FormulaFunctionId::BetaDotDist
    | FormulaFunctionId::Betadist
    | FormulaFunctionId::BetaDotInv
    | FormulaFunctionId::BinomDotDist
    | FormulaFunctionId::BinomDotDistDotRange
    | FormulaFunctionId::BinomDotInv
    | FormulaFunctionId::ChisqDotDist
    | FormulaFunctionId::ChisqDotDistDotRt
    | FormulaFunctionId::Chisqdist
    | FormulaFunctionId::ChisqDotInv
    | FormulaFunctionId::Chisqinv
    | FormulaFunctionId::ExponDotDist
    | FormulaFunctionId::FDotDist
    | FormulaFunctionId::FDotDistDotRt
    | FormulaFunctionId::FDotInv
    | FormulaFunctionId::FDotInvDotRt
    | FormulaFunctionId::Gamma
    | FormulaFunctionId::GammaDotDist
    | FormulaFunctionId::Gammadist
    | FormulaFunctionId::GammaDotInv
    | FormulaFunctionId::Negbinomdist
    | FormulaFunctionId::NegbinomDotDist
    | FormulaFunctionId::TDotDist
    | FormulaFunctionId::TDotDistDot2t
    | FormulaFunctionId::TDotDistDotRt
    | FormulaFunctionId::Tdist
    | FormulaFunctionId::TDotInv
    | FormulaFunctionId::TDotInvDot2t
    | FormulaFunctionId::WeibullDotDist => Some(FormulaValue::Error(FormulaErrorValue::Value)),
    FormulaFunctionId::Mmult if args.len() == 2 => evaluate_mmult_reader(evaluator, args),
    FormulaFunctionId::Mdeterm if args.len() == 1 => evaluate_mdeterm_reader(evaluator, args),
    FormulaFunctionId::Minverse if args.len() == 1 => evaluate_minverse_reader(evaluator, args),
    FormulaFunctionId::Munit if args.len() == 1 => evaluate_munit_reader(evaluator, args),
    FormulaFunctionId::Transpose if args.len() == 1 => evaluate_transpose_reader(evaluator, args),
    FormulaFunctionId::Filter if (2..=3).contains(&args.len()) => {
      evaluate_filter_reader(evaluator, args)
    }
    FormulaFunctionId::Expand if (2..=4).contains(&args.len()) => {
      evaluate_expand_reader(evaluator, args)
    }
    FormulaFunctionId::Hstack if !args.is_empty() => evaluate_stack_reader(evaluator, args, true),
    FormulaFunctionId::Vstack if !args.is_empty() => evaluate_stack_reader(evaluator, args, false),
    FormulaFunctionId::Tocol if (1..=3).contains(&args.len()) => {
      evaluate_torow_tocol_reader(evaluator, args, false)
    }
    FormulaFunctionId::Torow if (1..=3).contains(&args.len()) => {
      evaluate_torow_tocol_reader(evaluator, args, true)
    }
    FormulaFunctionId::Sequence if (1..=4).contains(&args.len()) => {
      evaluate_sequence_reader(evaluator, args)
    }
    FormulaFunctionId::Wrapcols if (2..=3).contains(&args.len()) => {
      evaluate_wrap_reader(evaluator, args, false)
    }
    FormulaFunctionId::Wraprows if (2..=3).contains(&args.len()) => {
      evaluate_wrap_reader(evaluator, args, true)
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
    FormulaFunctionId::Unique if (1..=3).contains(&args.len()) => {
      evaluate_unique_reader(evaluator, args)
    }
    FormulaFunctionId::Chooserows if args.len() >= 2 => {
      evaluate_choose_rows_columns_reader(evaluator, args, true)
    }
    FormulaFunctionId::Choosecols if args.len() >= 2 => {
      evaluate_choose_rows_columns_reader(evaluator, args, false)
    }
    FormulaFunctionId::Ceiling if (1..=3).contains(&args.len()) => {
      if matches!(
        evaluator.grammar,
        FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1
      ) && args.len() == 2
        && !(evaluator.array_context && ceiling_floor_has_array_argument(args))
      {
        return evaluate_ceiling_floor_legacy_reader(evaluator, args, true);
      }
      evaluate_ceiling_floor_reader(evaluator, args, true, CeilingFloorKind::Odff)
    }
    FormulaFunctionId::ComDotMicrosoftDotCeiling if args.len() == 2 => {
      evaluate_ceiling_floor_legacy_reader(evaluator, args, true)
    }
    FormulaFunctionId::ComDotMicrosoftDotCeiling => {
      Some(FormulaValue::Error(FormulaErrorValue::Unknown))
    }
    FormulaFunctionId::Floor if (1..=3).contains(&args.len()) => {
      if matches!(
        evaluator.grammar,
        FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1
      ) && args.len() == 2
        && !(evaluator.array_context && ceiling_floor_has_array_argument(args))
      {
        return evaluate_ceiling_floor_legacy_reader(evaluator, args, false);
      }
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
    FormulaFunctionId::Ceiling
    | FormulaFunctionId::Floor
    | FormulaFunctionId::CeilingDotMath
    | FormulaFunctionId::FloorDotMath
    | FormulaFunctionId::CeilingDotPrecise
    | FormulaFunctionId::FloorDotPrecise => Some(FormulaValue::Error(FormulaErrorValue::Value)),
    FormulaFunctionId::Date if args.len() == 3 => evaluate_date_reader(evaluator, args),
    FormulaFunctionId::Datevalue if args.len() == 1 => {
      let value = datevalue(
        &evaluator.text(&args.first_value()?),
        evaluator.book.date_system,
      );
      Some(match value {
        FormulaValue::Error(FormulaErrorValue::IllegalArgument) => {
          FormulaValue::Error(FormulaErrorValue::Value)
        }
        value => value,
      })
    }
    FormulaFunctionId::Timevalue if args.len() == 1 => {
      let text = evaluator.text(&args.first_value()?);
      if !text.contains(':')
        && crate::evaluator::parse_date_input(&text, evaluator.book.date_system).is_some()
      {
        Some(FormulaValue::Number(0.0))
      } else {
        Some(timevalue(&text))
      }
    }
    FormulaFunctionId::Basisodatetime if args.len() == 1 => {
      evaluate_basis_o_datetime_reader(evaluator, args)
    }
    FormulaFunctionId::Time if args.len() == 3 => evaluate_time_reader(evaluator, args),
    FormulaFunctionId::Weekday if (1..=2).contains(&args.len()) => {
      evaluate_weekday_reader(evaluator, args)
    }
    FormulaFunctionId::Weekday => Some(FormulaValue::Error(FormulaErrorValue::Value)),
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
    FormulaFunctionId::Days if args.len() == 2 => evaluate_days_reader(evaluator, args),
    FormulaFunctionId::Days => Some(FormulaValue::Error(FormulaErrorValue::Value)),
    FormulaFunctionId::Days360 if (2..=3).contains(&args.len()) => {
      evaluate_days360_reader(evaluator, args)
    }
    FormulaFunctionId::Days360 => Some(FormulaValue::Error(FormulaErrorValue::Value)),
    FormulaFunctionId::Weeks if args.len() == 3 => evaluate_weeks_reader(evaluator, args),
    FormulaFunctionId::Weeknum if args.len() == 2 => evaluate_weeknum_reader(evaluator, args),
    FormulaFunctionId::Months if args.len() == 3 => {
      evaluate_months_years_reader(evaluator, args, false)
    }
    FormulaFunctionId::Years if args.len() == 3 => {
      evaluate_months_years_reader(evaluator, args, true)
    }
    FormulaFunctionId::Weeksinyear if args.len() == 1 => {
      evaluate_weeks_in_year_reader(evaluator, args)
    }
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
    FormulaFunctionId::Eomonth | FormulaFunctionId::Edate => {
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    }
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
    FormulaFunctionId::Networkdays if (2..=4).contains(&args.len()) => {
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
    FormulaFunctionId::Workday | FormulaFunctionId::WorkdayDotIntl => {
      Some(FormulaValue::Error(FormulaErrorValue::Value))
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
    FormulaFunctionId::Bitand | FormulaFunctionId::Bitor | FormulaFunctionId::Bitxor
      if args.len() == 2 =>
    {
      evaluate_bitwise_reader(evaluator, args, function)
    }
    FormulaFunctionId::Bitlshift | FormulaFunctionId::Bitrshift if args.len() == 2 => {
      evaluate_bitshift_reader(
        evaluator,
        args,
        matches!(function, FormulaFunctionId::Bitlshift),
      )
    }
    FormulaFunctionId::Bin2dec
    | FormulaFunctionId::Oct2dec
    | FormulaFunctionId::Hex2dec
    | FormulaFunctionId::Bin2oct
    | FormulaFunctionId::Bin2hex
    | FormulaFunctionId::Oct2bin
    | FormulaFunctionId::Oct2hex
    | FormulaFunctionId::Hex2bin
    | FormulaFunctionId::Hex2oct
    | FormulaFunctionId::Dec2bin
    | FormulaFunctionId::Dec2oct
    | FormulaFunctionId::Dec2hex
    | FormulaFunctionId::Bitand
    | FormulaFunctionId::Bitor
    | FormulaFunctionId::Bitxor
    | FormulaFunctionId::Bitlshift
    | FormulaFunctionId::Bitrshift => Some(FormulaValue::Error(FormulaErrorValue::Value)),
    FormulaFunctionId::Color if (3..=4).contains(&args.len()) => {
      evaluate_color_reader(evaluator, args)
    }
    FormulaFunctionId::Color => Some(FormulaValue::Error(FormulaErrorValue::Value)),
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
    FormulaFunctionId::Replace if args.len() == 4 => evaluate_replace_reader(evaluator, args),
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
    FormulaFunctionId::Slope if args.len() == 2 => {
      evaluate_slope_intercept_reader(evaluator, args, true)
    }
    FormulaFunctionId::Intercept if args.len() == 2 => {
      evaluate_slope_intercept_reader(evaluator, args, false)
    }
    FormulaFunctionId::Rsq if args.len() == 2 => {
      evaluate_regression_scalar_reader(evaluator, args, RegressionScalarKind::Rsq)
    }
    FormulaFunctionId::Steyx if args.len() == 2 => {
      evaluate_regression_scalar_reader(evaluator, args, RegressionScalarKind::Steyx)
    }
    FormulaFunctionId::Linest if (1..=4).contains(&args.len()) => {
      evaluate_regression_array_reader(evaluator, args, RegressionArrayKind::Linest)
    }
    FormulaFunctionId::Logest if (1..=4).contains(&args.len()) => {
      evaluate_regression_array_reader(evaluator, args, RegressionArrayKind::Logest)
    }
    FormulaFunctionId::Trend if (1..=4).contains(&args.len()) => {
      evaluate_regression_prediction_reader(evaluator, args, RegressionPredictionKind::Trend)
    }
    FormulaFunctionId::Growth if (1..=4).contains(&args.len()) => {
      evaluate_regression_prediction_reader(evaluator, args, RegressionPredictionKind::Growth)
    }
    FormulaFunctionId::ForecastDotEts if (3..=6).contains(&args.len()) => {
      evaluator.evaluate_forecast_ets_reader(args, EtsKind::Add)
    }
    FormulaFunctionId::ForecastDotEtsDotMult if (3..=6).contains(&args.len()) => {
      evaluator.evaluate_forecast_ets_reader(args, EtsKind::Mult)
    }
    FormulaFunctionId::ForecastDotEtsDotPi if (3..=7).contains(&args.len()) => {
      evaluator.evaluate_forecast_ets_reader(args, EtsKind::PiAdd)
    }
    FormulaFunctionId::ForecastDotEtsDotPiDotMult if (3..=7).contains(&args.len()) => {
      evaluator.evaluate_forecast_ets_reader(args, EtsKind::PiMult)
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
    FormulaFunctionId::Prob if (3..=4).contains(&args.len()) => {
      evaluate_prob_reader(evaluator, args)
    }
    FormulaFunctionId::HypgeomDotDist if (4..=5).contains(&args.len()) => {
      evaluate_hypgeom_dist_reader(evaluator, args)
    }
    FormulaFunctionId::LognormDotInv if args.len() == 3 => {
      evaluate_lognorm_inv_reader(evaluator, args)
    }
    FormulaFunctionId::LognormDotDist if (1..=4).contains(&args.len()) => {
      evaluate_lognorm_dist_reader(evaluator, args)
    }
    FormulaFunctionId::NormDotInv if args.len() == 3 => {
      evaluate_norm_inv_reader(evaluator, args, false)
    }
    FormulaFunctionId::NormDotSDotInv if args.len() == 1 => {
      evaluate_norm_inv_reader(evaluator, args, true)
    }
    FormulaFunctionId::NormDotDist if args.len() == 4 => {
      evaluate_norm_dist_reader(evaluator, args, false)
    }
    FormulaFunctionId::NormDotSDotDist if args.len() == 2 => {
      evaluate_norm_dist_reader(evaluator, args, true)
    }
    FormulaFunctionId::NormDotSDotDist if args.len() == 1 => {
      evaluate_norm_s_dist_legacy_reader(evaluator, args)
    }
    FormulaFunctionId::Poisson | FormulaFunctionId::PoissonDotDist if args.len() == 3 => {
      evaluate_poisson_reader(evaluator, args)
    }
    FormulaFunctionId::Poisson | FormulaFunctionId::PoissonDotDist => {
      Some(FormulaValue::Error(FormulaErrorValue::Value))
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
    FormulaFunctionId::Correl | FormulaFunctionId::Pearson if args.len() == 2 => {
      evaluate_correlation_reader(evaluator, args)
    }
    FormulaFunctionId::Covar
    | FormulaFunctionId::CovarianceDotP
    | FormulaFunctionId::CovarianceDotS => Some(FormulaValue::Error(FormulaErrorValue::Unknown)),
    FormulaFunctionId::Complex if (2..=3).contains(&args.len()) => {
      evaluate_complex_reader(evaluator, args)
    }
    FormulaFunctionId::Imabs if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Abs)
    }
    FormulaFunctionId::Imaginary if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Imaginary)
    }
    FormulaFunctionId::Imargument if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Argument)
    }
    FormulaFunctionId::Imconjugate if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Conjugate)
    }
    FormulaFunctionId::Imcos if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Cos)
    }
    FormulaFunctionId::Imcosh if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Cosh)
    }
    FormulaFunctionId::Imcot if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Cot)
    }
    FormulaFunctionId::Imcsc if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Csc)
    }
    FormulaFunctionId::Imcsch if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Csch)
    }
    FormulaFunctionId::Imdiv if args.len() == 2 => evaluate_complex_div_reader(evaluator, args),
    FormulaFunctionId::Imexp if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Exp)
    }
    FormulaFunctionId::Imln if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Ln)
    }
    FormulaFunctionId::Imlog10 if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Log10)
    }
    FormulaFunctionId::Imlog2 if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Log2)
    }
    FormulaFunctionId::Impower if args.len() == 2 => evaluate_complex_power_reader(evaluator, args),
    FormulaFunctionId::Improduct if args.len() >= 1 => {
      evaluate_complex_aggregate_reader(evaluator, args, true)
    }
    FormulaFunctionId::Imreal if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Real)
    }
    FormulaFunctionId::Imsec if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Sec)
    }
    FormulaFunctionId::Imsech if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Sech)
    }
    FormulaFunctionId::Imsin if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Sin)
    }
    FormulaFunctionId::Imsinh if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Sinh)
    }
    FormulaFunctionId::Imsqrt if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Sqrt)
    }
    FormulaFunctionId::Imsub if args.len() == 2 => evaluate_complex_sub_reader(evaluator, args),
    FormulaFunctionId::Imsum if args.len() >= 1 => {
      evaluate_complex_aggregate_reader(evaluator, args, false)
    }
    FormulaFunctionId::Imtan if args.len() == 1 => {
      evaluate_complex_unary_reader(evaluator, args, ComplexUnaryKind::Tan)
    }
    FormulaFunctionId::Cumipmt if args.len() == 6 => {
      evaluator.evaluate_cum_interest_principal_reader(args, true)
    }
    FormulaFunctionId::Cumprinc if args.len() == 6 => {
      evaluator.evaluate_cum_interest_principal_reader(args, false)
    }
    FormulaFunctionId::Fvschedule if args.len() == 2 => evaluate_fvschedule_reader(evaluator, args),
    FormulaFunctionId::Besseli if args.len() == 2 => {
      evaluate_bessel_reader(evaluator, args, BesselKind::I)
    }
    FormulaFunctionId::Besselj if args.len() == 2 => {
      evaluate_bessel_reader(evaluator, args, BesselKind::J)
    }
    FormulaFunctionId::Besselk if args.len() == 2 => {
      evaluate_bessel_reader(evaluator, args, BesselKind::K)
    }
    FormulaFunctionId::Bessely if args.len() == 2 => {
      evaluate_bessel_reader(evaluator, args, BesselKind::Y)
    }
    FormulaFunctionId::Erf | FormulaFunctionId::ErfDotPrecise if (1..=2).contains(&args.len()) => {
      evaluate_erf_reader(evaluator, args)
    }
    FormulaFunctionId::ErfcDotPrecise if args.len() == 1 => evaluate_erfc_reader(evaluator, args),
    FormulaFunctionId::Gestep if (1..=2).contains(&args.len()) => {
      evaluate_gestep_reader(evaluator, args)
    }
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
    FormulaFunctionId::Fv if (3..=5).contains(&args.len()) => evaluate_fv_reader(evaluator, args),
    FormulaFunctionId::Ipmt if (4..=6).contains(&args.len()) => {
      evaluate_ipmt_ppmt_reader(evaluator, args, true)
    }
    FormulaFunctionId::Ppmt if (4..=6).contains(&args.len()) => {
      evaluate_ipmt_ppmt_reader(evaluator, args, false)
    }
    FormulaFunctionId::Nper if (3..=5).contains(&args.len()) => {
      evaluate_nper_reader(evaluator, args)
    }
    FormulaFunctionId::Rate if (3..=6).contains(&args.len()) => {
      evaluate_rate_reader(evaluator, args)
    }
    FormulaFunctionId::Irr if (1..=2).contains(&args.len()) => evaluate_irr_reader(evaluator, args),
    FormulaFunctionId::Mirr if args.len() == 3 => evaluate_mirr_reader(evaluator, args),
    FormulaFunctionId::Npv if args.len() >= 2 => evaluate_npv_reader(evaluator, args),
    FormulaFunctionId::Ispmt if args.len() == 4 => evaluate_ispmt_reader(evaluator, args),
    FormulaFunctionId::Rri if args.len() == 3 => evaluate_rri_reader(evaluator, args),
    FormulaFunctionId::Pduration if args.len() == 3 => evaluate_pduration_reader(evaluator, args),
    FormulaFunctionId::Effect if args.len() == 2 => {
      evaluate_effect_nominal_reader(evaluator, args, true)
    }
    FormulaFunctionId::Nominal if args.len() == 2 => {
      evaluate_effect_nominal_reader(evaluator, args, false)
    }
    FormulaFunctionId::Sln if args.len() == 3 => evaluate_sln_reader(evaluator, args),
    FormulaFunctionId::Syd if args.len() == 4 => evaluate_syd_reader(evaluator, args),
    FormulaFunctionId::Db if (4..=5).contains(&args.len()) => evaluate_db_reader(evaluator, args),
    FormulaFunctionId::Ddb if (4..=5).contains(&args.len()) => evaluate_ddb_reader(evaluator, args),
    FormulaFunctionId::Vdb if (5..=7).contains(&args.len()) => evaluate_vdb_reader(evaluator, args),
    FormulaFunctionId::Xnpv if args.len() == 3 => evaluate_xnpv_reader(evaluator, args),
    FormulaFunctionId::Xirr if (2..=3).contains(&args.len()) => {
      evaluate_xirr_reader(evaluator, args)
    }
    FormulaFunctionId::Coupdaybs if (4..=5).contains(&args.len()) => {
      evaluate_coupon_reader(evaluator, args, CouponKind::Daybs)
    }
    FormulaFunctionId::Coupdays if (4..=5).contains(&args.len()) => {
      evaluate_coupon_reader(evaluator, args, CouponKind::Days)
    }
    FormulaFunctionId::Coupdaysnc if (4..=5).contains(&args.len()) => {
      evaluate_coupon_reader(evaluator, args, CouponKind::Daysnc)
    }
    FormulaFunctionId::Coupncd if (4..=5).contains(&args.len()) => {
      evaluate_coupon_reader(evaluator, args, CouponKind::Ncd)
    }
    FormulaFunctionId::Coupnum if (4..=5).contains(&args.len()) => {
      evaluate_coupon_reader(evaluator, args, CouponKind::Num)
    }
    FormulaFunctionId::Couppcd if (4..=5).contains(&args.len()) => {
      evaluate_coupon_reader(evaluator, args, CouponKind::Pcd)
    }
    FormulaFunctionId::Price if (6..=7).contains(&args.len()) => {
      evaluate_price_reader(evaluator, args)
    }
    FormulaFunctionId::Pricedisc if (4..=5).contains(&args.len()) => {
      evaluate_discount_security_reader(evaluator, args, DiscountSecurityKind::Price)
    }
    FormulaFunctionId::Pricemat if (5..=6).contains(&args.len()) => {
      evaluate_pricemat_reader(evaluator, args)
    }
    FormulaFunctionId::Yielddisc if (4..=5).contains(&args.len()) => {
      evaluate_discount_security_reader(evaluator, args, DiscountSecurityKind::Yield)
    }
    FormulaFunctionId::Disc if (4..=5).contains(&args.len()) => {
      evaluate_discount_security_reader(evaluator, args, DiscountSecurityKind::Discount)
    }
    FormulaFunctionId::Received if (4..=5).contains(&args.len()) => {
      evaluate_discount_security_reader(evaluator, args, DiscountSecurityKind::Received)
    }
    FormulaFunctionId::Intrate if (4..=5).contains(&args.len()) => {
      evaluate_discount_security_reader(evaluator, args, DiscountSecurityKind::Intrate)
    }
    FormulaFunctionId::Accrint if (6..=7).contains(&args.len()) => {
      evaluate_accrint_reader(evaluator, args, false)
    }
    FormulaFunctionId::Accrintm if (4..=5).contains(&args.len()) => {
      evaluate_accrint_reader(evaluator, args, true)
    }
    FormulaFunctionId::Tbilleq if args.len() == 3 => {
      evaluate_tbill_reader(evaluator, args, TbillKind::Eq)
    }
    FormulaFunctionId::Tbillprice if args.len() == 3 => {
      evaluate_tbill_reader(evaluator, args, TbillKind::Price)
    }
    FormulaFunctionId::Tbillyield if args.len() == 3 => {
      evaluate_tbill_reader(evaluator, args, TbillKind::Yield)
    }
    FormulaFunctionId::Oddlprice if (7..=8).contains(&args.len()) => {
      evaluate_odd_period_reader(evaluator, args, true)
    }
    FormulaFunctionId::Oddlyield if (7..=8).contains(&args.len()) => {
      evaluate_odd_period_reader(evaluator, args, false)
    }
    FormulaFunctionId::Amordegrc if (6..=7).contains(&args.len()) => {
      evaluate_amor_reader(evaluator, args, true)
    }
    FormulaFunctionId::Amorlinc if (6..=7).contains(&args.len()) => {
      evaluate_amor_reader(evaluator, args, false)
    }
    FormulaFunctionId::Mduration if (5..=6).contains(&args.len()) => {
      evaluate_mduration_reader(evaluator, args)
    }
    FormulaFunctionId::Yield if (6..=7).contains(&args.len()) => {
      evaluate_yield_reader(evaluator, args)
    }
    FormulaFunctionId::Yield => Some(FormulaValue::Error(FormulaErrorValue::Value)),
    FormulaFunctionId::Randbetween if args.len() == 2 => {
      evaluate_randbetween_reader(evaluator, args)
    }
    FormulaFunctionId::Randbetween => Some(FormulaValue::Error(FormulaErrorValue::Value)),
    FormulaFunctionId::Randarray if args.len() <= 5 => evaluate_randarray_reader(evaluator, args),
    FormulaFunctionId::Subtotal if args.len() >= 2 => evaluator.evaluate_subtotal_reader(args),
    FormulaFunctionId::Subtotal => Some(FormulaValue::Error(FormulaErrorValue::Value)),
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
          Some(match logical_scalar_boolean(evaluator, value.clone()) {
            Ok(value) => FormulaValue::Boolean(!value.unwrap_or(false)),
            Err(error) => FormulaValue::Error(error),
          })
        });
      }
      Some(match logical_scalar_boolean(evaluator, value) {
        Ok(value) => FormulaValue::Boolean(!value.unwrap_or(false)),
        Err(error) => FormulaValue::Error(error),
      })
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
    FormulaFunctionId::Current if args.is_empty() => evaluator.current_value.clone(),
    FormulaFunctionId::Dhfg => Some(FormulaValue::Error(FormulaErrorValue::Name)),
    FormulaFunctionId::Testfunccurr if args.is_empty() => Some(FormulaValue::Number(5.5)),
    FormulaFunctionId::Testfuncint if args.is_empty() => Some(FormulaValue::Number(6.0)),
    FormulaFunctionId::Testfuncsingle if args.is_empty() => Some(FormulaValue::Number(5.5)),
    FormulaFunctionId::Testfuncdate if args.is_empty() => Some(FormulaValue::Number(5590.0)),
    FormulaFunctionId::Testfuncbool if args.is_empty() => Some(FormulaValue::Boolean(true)),
    _ => None,
  }
}

fn scalar_number_arg_or_value<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
) -> Option<std::result::Result<f64, FormulaErrorValue>> {
  let value = evaluator.scalar_binary_operand(args.value(index)?);
  Some(match value {
    FormulaValue::Error(error) => Err(error),
    value => match evaluator.number(&value) {
      Some(value) if value.is_finite() => Ok(value),
      Some(_) => Err(FormulaErrorValue::Num),
      None => Err(FormulaErrorValue::Value),
    },
  })
}

fn evaluate_rawsubtract_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let mut result = match scalar_number_arg_or_value(evaluator, args, 0)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  for index in 1..args.len() {
    let value = match scalar_number_arg_or_value(evaluator, args, index)? {
      Ok(value) => value,
      Err(error) => return Some(FormulaValue::Error(error)),
    };
    result -= value;
  }
  Some(FormulaValue::Number(result))
}

fn strict_text_arg<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: FormulaValue<'doc>,
) -> std::result::Result<String, FormulaErrorValue> {
  let value = evaluator.scalar_binary_operand(value);
  if let FormulaValue::Error(error) = value {
    return Err(error);
  }
  Ok(evaluator.text(&value))
}

fn map_text_unary_value<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: FormulaValue<'doc>,
  op: impl Fn(&str) -> FormulaValue<'doc> + Copy,
) -> Option<FormulaValue<'doc>> {
  evaluator.map_unary_values(value, |evaluator, value| {
    if let FormulaValue::Error(error) = value {
      return Some(FormulaValue::Error(*error));
    }
    Some(op(&evaluator.text(value)))
  })
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
  if let Some(value) = information_logical_value(evaluator, value.clone())
    && matches!(value, FormulaValue::Error(_))
  {
    return Some(FormulaValue::Boolean(matches!(
      value,
      FormulaValue::Error(error) if matches_error(error)
    )));
  }
  Some(FormulaValue::Boolean(matches!(
    evaluator.first_value(value),
    FormulaValue::Error(error) if matches_error(error)
  )))
}

fn information_logical_value<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: FormulaValue<'doc>,
) -> Option<FormulaValue<'doc>> {
  match value {
    FormulaValue::Reference(reference) => Some(
      evaluator
        .implicit_intersection_value(&reference)
        .map(|value| evaluator.first_value(&value))
        .unwrap_or(FormulaValue::Error(FormulaErrorValue::Value)),
    ),
    FormulaValue::RefList(ranges) => {
      if ranges.len() == 1 {
        Some(
          evaluator
            .implicit_intersection_value(&ranges[0])
            .map(|value| evaluator.first_value(&value))
            .unwrap_or(FormulaValue::Error(FormulaErrorValue::Value)),
        )
      } else {
        Some(FormulaValue::Error(FormulaErrorValue::Value))
      }
    }
    FormulaValue::Matrix(rows) => rows
      .into_iter()
      .next()
      .and_then(|row| row.into_iter().next()),
    value => Some(value),
  }
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
  if let FormulaValue::Reference(reference) = value {
    let sheet = evaluator.range_sheet(reference);
    if reference.range.cell_count_hint() == 1 {
      if evaluator
        .book
        .formulas
        .contains_key(&(sheet, reference.range.start))
      {
        return Some(FormulaValue::Boolean(false));
      }
    } else if let Some(value) = evaluator.implicit_intersection_value(reference) {
      if let FormulaValue::Reference(intersection) = &value {
        if evaluator
          .book
          .formulas
          .contains_key(&(sheet, intersection.range.start))
        {
          return Some(FormulaValue::Boolean(false));
        }
      }
      return Some(FormulaValue::Boolean(matches!(
        evaluator.first_value(&value),
        FormulaValue::Blank
      )));
    } else {
      return Some(FormulaValue::Boolean(false));
    }
  }
  if matches!(value, FormulaValue::RefList(_))
    || matches!(value, FormulaValue::Reference(reference) if reference.range.cell_count_hint() != 1)
  {
    return Some(FormulaValue::Boolean(false));
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
    return evaluator.map_unary_values(value.clone(), |evaluator, value| {
      Some(FormulaValue::Boolean(
        matches!(value, FormulaValue::Number(_))
          || matches!(
            (evaluator.grammar, value),
            (
              FormulaGrammar::CalcA1 | FormulaGrammar::OpenFormula,
              FormulaValue::Boolean(_)
            )
          ),
      ))
    });
  }
  let value = information_logical_value(evaluator, value.clone());
  Some(FormulaValue::Boolean(
    matches!(value, Some(FormulaValue::Number(_)))
      || matches!(
        (evaluator.grammar, value),
        (
          FormulaGrammar::CalcA1 | FormulaGrammar::OpenFormula,
          Some(FormulaValue::Boolean(_))
        )
      ),
  ))
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
    FormulaValue::Error(error) => return Some(FormulaValue::Error(error)),
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
  Some(display)
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

fn evaluate_mode_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let mut values = Vec::new();
  for index in 0..args.len() {
    if let Err(error) = collect_mode_numbers(evaluator, args.value(index)?, &mut values) {
      return Some(FormulaValue::Error(error));
    }
  }
  mode_slice(&values)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
}

fn evaluate_mode_ms_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  single: bool,
) -> Option<FormulaValue<'doc>> {
  let mut values = Vec::new();
  for index in 0..args.len() {
    if let Err(error) = collect_mode_ms_numbers(evaluator, args.value(index)?, &mut values) {
      return Some(FormulaValue::Error(error));
    }
  }
  let modes = match mode_ms_values(&values) {
    Some(modes) => modes,
    None => return Some(FormulaValue::Error(FormulaErrorValue::NA)),
  };
  if single || !evaluator.array_context {
    return Some(FormulaValue::Number(modes[0]));
  }
  Some(FormulaValue::Matrix(
    modes
      .into_iter()
      .map(|value| vec![FormulaValue::Number(value)])
      .collect(),
  ))
}

fn collect_mode_ms_numbers<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: FormulaValue<'doc>,
  values: &mut Vec<f64>,
) -> std::result::Result<(), FormulaErrorValue> {
  match value {
    FormulaValue::Reference(reference) if reference.range.cell_count_hint() == 1 => {
      match evaluator.first_value(&FormulaValue::Reference(reference)) {
        FormulaValue::Number(value) => values.push(value),
        FormulaValue::Error(error) => return Err(error),
        _ => return Err(FormulaErrorValue::Value),
      }
    }
    FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_) => {
      let matrix = evaluator.matrix_values(&value);
      let columns = matrix.first().map_or(0, Vec::len);
      for column in 0..columns {
        for row in &matrix {
          match row.get(column) {
            Some(FormulaValue::Number(value)) => values.push(*value),
            Some(FormulaValue::Error(error)) => return Err(*error),
            Some(_) | None => {}
          }
        }
      }
    }
    FormulaValue::Number(value) => values.push(value),
    FormulaValue::Error(error) => return Err(error),
    FormulaValue::Blank | FormulaValue::Boolean(_) | FormulaValue::String(_) => {
      return Err(FormulaErrorValue::Value);
    }
  }
  Ok(())
}

fn collect_mode_numbers<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: FormulaValue<'doc>,
  values: &mut Vec<f64>,
) -> std::result::Result<(), FormulaErrorValue> {
  match value {
    FormulaValue::Reference(reference) if reference.range.cell_count_hint() == 1 => {
      match evaluator.first_value(&FormulaValue::Reference(reference)) {
        FormulaValue::Number(value) => values.push(value),
        FormulaValue::Error(error) => return Err(error),
        _ => return Err(FormulaErrorValue::Value),
      }
    }
    FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_) => {
      for value in evaluator.matrix_values(&value).into_iter().flatten() {
        match value {
          FormulaValue::Number(value) => values.push(value),
          FormulaValue::Error(error) => return Err(error),
          _ => {}
        }
      }
    }
    FormulaValue::Number(value) => values.push(value),
    FormulaValue::Error(error) => return Err(error),
    FormulaValue::Blank | FormulaValue::Boolean(_) | FormulaValue::String(_) => {
      return Err(FormulaErrorValue::Value);
    }
  }
  Ok(())
}

fn evaluate_areas_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  if args.len() != 1 {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  }
  if let Some(count) = raw_reference_area_count(args, 0) {
    return Some(FormulaValue::Number(count as f64));
  }
  let ranges = args.reference_ranges(0)?;
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
      let value = FormulaValue::Matrix(vec![
        (start_column..=end_column)
          .map(|column| FormulaValue::Number(column as f64 + 1.0))
          .collect(),
      ]);
      return Some(if evaluator.array_context {
        value
      } else {
        evaluator.scalar_value(value)
      });
    }
    if !column && end_row > start_row {
      let value = FormulaValue::Matrix(
        (start_row..=end_row)
          .map(|row| vec![FormulaValue::Number(row as f64 + 1.0)])
          .collect(),
      );
      return Some(if evaluator.array_context {
        value
      } else {
        evaluator.scalar_value(value)
      });
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
    return evaluator.map_unary_values(value.clone(), |evaluator, value| {
      if let FormulaValue::Error(error) = value {
        return Some(FormulaValue::Error(*error));
      }
      let text = evaluator.text(value);
      Some(FormulaValue::Number(if bytes {
        text_byte_len(&text) as f64
      } else {
        text.chars().count() as f64
      }))
    });
  }
  let value = evaluator.scalar_binary_operand(value.clone());
  if let FormulaValue::Error(error) = value {
    return Some(FormulaValue::Error(error));
  }
  let text = evaluator.text(&value);
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
  let value = evaluator.first_value(&value);
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
    match evaluator.grammar {
      FormulaGrammar::OpenFormula | FormulaGrammar::CalcA1 => String::new(),
      FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1 => ".".to_string(),
    }
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

fn evaluate_text_transform_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  transform: fn(&str) -> String,
) -> Option<FormulaValue<'doc>> {
  let value = args.value(0)?;
  if evaluator.array_context && is_matrix_argument(&value) {
    return map_text_unary_value(evaluator, value, |text| {
      FormulaValue::String(Cow::Owned(transform(text)))
    });
  }
  let text = match strict_text_arg(evaluator, value) {
    Ok(text) => text,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  Some(FormulaValue::String(Cow::Owned(transform(&text))))
}

fn evaluate_rept_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let text = match strict_text_arg(evaluator, args.value(0)?) {
    Ok(text) => text,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let count = match scalar_number_arg_or_value(evaluator, args, 1)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  if count < 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let count = floor_to_usize(count)?;
  if count == 0 {
    return Some(FormulaValue::String(Cow::Borrowed("")));
  }
  if text.chars().count().saturating_mul(count) > 32_767 {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  Some(FormulaValue::String(Cow::Owned(text.repeat(count))))
}

fn evaluate_bahttext_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let arg = args.value(0)?;
  if evaluator.array_context && is_matrix_argument(&arg) {
    return evaluator.map_unary_values(arg, |evaluator, value| {
      evaluator
        .number(value)
        .filter(|value| value.is_finite())
        .map(|value| FormulaValue::String(Cow::Owned(baht_text(value))))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Num)))
    });
  }
  let value = match scalar_number_arg_or_value(evaluator, args, 0)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  if !value.is_finite() {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
  Some(FormulaValue::String(Cow::Owned(baht_text(value))))
}

fn evaluate_roman_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = match scalar_number_arg_or_value(evaluator, args, 0)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let mode = if args.len() == 2 && !args.is_missing(1) {
    match scalar_number_arg_or_value(evaluator, args, 1)? {
      Ok(value) => value,
      Err(error) => return Some(FormulaValue::Error(error)),
    }
  } else {
    0.0
  };
  if !(0.0..4000.0).contains(&value) || !(0.0..5.0).contains(&mode) {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let Some(value) = floor_to_u16(value) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let Some(mode) = floor_to_u16(mode) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  Some(FormulaValue::String(Cow::Owned(roman_text_libreoffice(
    value, mode,
  ))))
}

fn evaluate_arabic_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let text = match strict_text_arg(evaluator, args.value(0)?) {
    Ok(text) => text,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  arabic_from_roman(&text.to_ascii_uppercase())
    .map(|value| FormulaValue::Number(value as f64))
    .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
}

fn arabic_from_roman(text: &str) -> Option<u16> {
  let chars = text.as_bytes();
  let mut value = 0u16;
  let mut valid_rest = 3999u16;
  let mut index = 0usize;
  while index < chars.len() {
    let (digit1, is_decimal1) = roman_digit_value(chars[index])?;
    let digit2 = chars
      .get(index + 1)
      .and_then(|value| roman_digit_value(*value).map(|(digit, _)| digit))
      .unwrap_or(0);
    if digit1 >= digit2 {
      value = value.checked_add(digit1)?;
      valid_rest %= digit1 * if is_decimal1 { 5 } else { 2 };
      if valid_rest < digit1 {
        return None;
      }
      valid_rest -= digit1;
      index += 1;
    } else if digit1 * 2 != digit2 {
      let difference = digit2 - digit1;
      value = value.checked_add(difference)?;
      if valid_rest < difference {
        return None;
      }
      valid_rest = digit1 - 1;
      index += 2;
    } else {
      return None;
    }
  }
  Some(value)
}

fn roman_digit_value(value: u8) -> Option<(u16, bool)> {
  match value {
    b'M' => Some((1000, true)),
    b'D' => Some((500, false)),
    b'C' => Some((100, true)),
    b'L' => Some((50, false)),
    b'X' => Some((10, true)),
    b'V' => Some((5, false)),
    b'I' => Some((1, true)),
    _ => None,
  }
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

fn evaluate_concatenate_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let mut output = String::new();
  for index in 0..args.len() {
    let value = evaluator.scalar_binary_operand(args.value(index)?);
    if let FormulaValue::Error(error) = value {
      return Some(FormulaValue::Error(error));
    }
    output.push_str(&evaluator.text(&value));
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
  let delimiters = match textjoin_delimiters(evaluator, &args.array_value(0)?) {
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
    let value = args.array_value(index)?;
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

fn evaluate_text_before_after_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  before: bool,
) -> Option<FormulaValue<'doc>> {
  let if_not_found = args
    .raw_arg(5)
    .filter(|_| !args.is_missing(5))
    .and_then(|_| args.value(5));
  let match_end = args
    .raw_arg(4)
    .filter(|_| !args.is_missing(4))
    .and_then(|_| args.value(4))
    .is_some_and(|value| evaluator.truthy(&value));
  let match_mode = args
    .raw_arg(3)
    .filter(|_| !args.is_missing(3))
    .and_then(|_| args.value(3))
    .is_some_and(|value| evaluator.truthy(&value));
  let instance = match optional_i32_arg(evaluator, args, 2, 1)? {
    0 => return Some(FormulaValue::Error(FormulaErrorValue::NA)),
    value => value,
  };
  let delimiters = if args.raw_arg(1).is_some() {
    match delimiter_texts(evaluator, &args.array_value(1)?) {
      Ok(delimiters) => delimiters,
      Err(error) => return Some(FormulaValue::Error(error)),
    }
  } else {
    Vec::new()
  };
  let text = evaluator.text(&args.value(0)?);
  if text.is_empty() {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }

  let mut positions = text_delimiter_positions(&text, &delimiters, match_mode, before);
  if match_end {
    if before {
      if instance < 0 {
        positions.insert(0, 0);
      } else {
        positions.push(text.len());
      }
    } else if instance < 0 {
      positions.push(text.len());
    } else {
      positions.insert(0, 0);
    }
  }
  let count = positions.len() as i32;
  if count == 0 || instance.abs() > count {
    return Some(if_not_found.unwrap_or(FormulaValue::Error(FormulaErrorValue::NA)));
  }
  let index = if instance < 0 {
    (count + instance) as usize
  } else {
    instance as usize - 1
  };
  let position = positions[index].min(text.len());
  Some(FormulaValue::String(Cow::Owned(if before {
    text[..position].to_string()
  } else {
    text[position..].to_string()
  })))
}

fn evaluate_textsplit_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let pad = args
    .raw_arg(5)
    .filter(|_| !args.is_missing(5))
    .and_then(|_| args.value(5))
    .unwrap_or(FormulaValue::Error(FormulaErrorValue::NA));
  let match_mode = args
    .raw_arg(4)
    .filter(|_| !args.is_missing(4))
    .and_then(|_| args.value(4))
    .is_some_and(|value| evaluator.truthy(&value));
  let ignore_empty = args
    .raw_arg(3)
    .filter(|_| !args.is_missing(3))
    .and_then(|_| args.value(3))
    .is_some_and(|value| evaluator.truthy(&value));
  let row_delimiters = if args.raw_arg(2).is_some() {
    match delimiter_texts(evaluator, &args.array_value(2)?) {
      Ok(delimiters) => delimiters,
      Err(error) => return Some(FormulaValue::Error(error)),
    }
  } else {
    Vec::new()
  };
  let col_delimiters = if args.raw_arg(1).is_some() {
    match delimiter_texts(evaluator, &args.array_value(1)?) {
      Ok(delimiters) => delimiters,
      Err(error) => return Some(FormulaValue::Error(error)),
    }
  } else {
    Vec::new()
  };
  let text = evaluator.text(&args.value(0)?);
  if text.is_empty() {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let rows = split_text_by_delimiters(&text, &row_delimiters, ignore_empty, match_mode);
  let mut result = rows
    .into_iter()
    .map(|row| {
      split_text_by_delimiters(&row, &col_delimiters, ignore_empty, match_mode)
        .into_iter()
        .map(|value| FormulaValue::String(Cow::Owned(value)))
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();
  let columns = result.iter().map(Vec::len).max().unwrap_or(1).max(1);
  for row in &mut result {
    row.resize(columns, pad.clone());
  }
  Some(FormulaValue::Matrix(result))
}

fn evaluate_regex_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let text_value = args.array_value(0)?;
  let pattern = evaluator.text(&args.value(1)?);
  let replacement = args
    .raw_arg(2)
    .filter(|_| !args.is_missing(2))
    .and_then(|_| args.value(2))
    .map(|value| evaluator.text(&value));
  let (occurrence, global) = match regex_occurrence_flags(evaluator, args)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let regex = match RegexBuilder::new(&pattern).build() {
    Ok(regex) => regex,
    Err(_) => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
  };
  evaluator.map_unary_values(text_value, |evaluator, value| {
    if let FormulaValue::Error(error) = value {
      return Some(FormulaValue::Error(*error));
    }
    Some(regex_apply(
      &regex,
      &evaluator.text(value),
      replacement.as_deref(),
      occurrence,
      global,
    ))
  })
}

fn regex_occurrence_flags<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<std::result::Result<(usize, bool), FormulaErrorValue>> {
  let Some(_) = args.raw_arg(3).filter(|_| !args.is_missing(3)) else {
    return Some(Ok((1, false)));
  };
  let value = args.value(3)?;
  if let FormulaValue::Error(error) = value {
    return Some(Err(error));
  }
  match value {
    FormulaValue::String(flags) => match flags.as_ref() {
      "" => Some(Ok((1, false))),
      "g" => Some(Ok((1, true))),
      _ => Some(Err(FormulaErrorValue::IllegalArgument)),
    },
    value => {
      let Some(number) = evaluator.number(&value) else {
        return Some(Err(FormulaErrorValue::Value));
      };
      if number < 0.0 {
        return Some(Err(FormulaErrorValue::IllegalArgument));
      }
      Some(Ok((number.floor() as usize, false)))
    }
  }
}

fn regex_apply<'doc>(
  regex: &regex::Regex,
  text: &str,
  replacement: Option<&str>,
  occurrence: usize,
  global: bool,
) -> FormulaValue<'doc> {
  if occurrence == 0 {
    return FormulaValue::String(Cow::Owned(text.to_string()));
  }
  let Some(replacement) = replacement else {
    return regex
      .find_iter(text)
      .nth(occurrence - 1)
      .map(|matched| FormulaValue::String(Cow::Owned(matched.as_str().to_string())))
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::NA));
  };
  if global {
    return FormulaValue::String(Cow::Owned(
      regex.replace_all(text, replacement).into_owned(),
    ));
  }
  if occurrence == 1 {
    return FormulaValue::String(Cow::Owned(regex.replace(text, replacement).into_owned()));
  }
  let mut result = String::new();
  let mut last = 0usize;
  for (index, captures) in regex.captures_iter(text).enumerate() {
    if index + 1 == occurrence {
      let matched = captures.get(0).expect("regex captures include full match");
      result.push_str(&text[last..matched.start()]);
      captures.expand(replacement, &mut result);
      last = matched.end();
      result.push_str(&text[last..]);
      return FormulaValue::String(Cow::Owned(result));
    }
  }
  FormulaValue::String(Cow::Owned(text.to_string()))
}

fn delimiter_texts<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
) -> std::result::Result<Vec<String>, FormulaErrorValue> {
  let mut delimiters = Vec::new();
  for value in evaluator.matrix_values(value).into_iter().flatten() {
    if let FormulaValue::Error(error) = value {
      return Err(error);
    }
    delimiters.push(evaluator.text(&value));
  }
  Ok(delimiters)
}

fn split_text_by_delimiters(
  text: &str,
  delimiters: &[String],
  ignore_empty: bool,
  case_insensitive: bool,
) -> Vec<String> {
  if delimiters.is_empty() || text.is_empty() {
    return vec![text.to_string()];
  }
  let mut result = Vec::new();
  let mut start = 0usize;
  while start < text.len() {
    let Some((index, length)) = next_text_delimiter(text, delimiters, start, case_insensitive)
    else {
      break;
    };
    let value = text[start..index].to_string();
    if !ignore_empty || !value.is_empty() {
      result.push(value);
    }
    start = index + length;
  }
  let value = text[start..].to_string();
  if !ignore_empty || !value.is_empty() {
    result.push(value);
  }
  result
}

fn text_delimiter_positions(
  text: &str,
  delimiters: &[String],
  case_insensitive: bool,
  before: bool,
) -> Vec<usize> {
  let mut positions = Vec::new();
  let mut start = 0usize;
  while start < text.len() {
    let Some((index, length)) = next_text_delimiter(text, delimiters, start, case_insensitive)
    else {
      break;
    };
    positions.push(if before { index } else { index + length });
    start = index + length;
  }
  positions
}

fn next_text_delimiter(
  text: &str,
  delimiters: &[String],
  start: usize,
  case_insensitive: bool,
) -> Option<(usize, usize)> {
  let mut best: Option<(usize, usize)> = None;
  for index in text
    .char_indices()
    .map(|(index, _)| index)
    .filter(|index| *index >= start)
  {
    for delimiter in delimiters {
      if delimiter.is_empty() {
        continue;
      }
      let Some(length) = text_delimiter_match_len(&text[index..], delimiter, case_insensitive)
      else {
        continue;
      };
      if best.is_none_or(|(best_index, _)| index < best_index) {
        best = Some((index, length));
      }
    }
  }
  best
}

fn text_delimiter_match_len(text: &str, delimiter: &str, case_insensitive: bool) -> Option<usize> {
  if !case_insensitive {
    return text.starts_with(delimiter).then_some(delimiter.len());
  }
  let mut end = 0usize;
  let mut text_chars = text.chars();
  for _ in delimiter.chars() {
    let ch = text_chars.next()?;
    end += ch.len_utf8();
  }
  (text[..end].to_lowercase() == delimiter.to_lowercase()).then_some(end)
}

fn optional_i32_arg<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
  default: i32,
) -> Option<i32> {
  if args.raw_arg(index).is_none() || args.is_missing(index) {
    return Some(default);
  }
  evaluator
    .number(&args.value(index)?)
    .map(|value| value.floor() as i32)
}

fn evaluate_text_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = evaluator.scalar_value(args.value(0)?);
  let format_value = args.value(1)?;
  match format_value {
    FormulaValue::Boolean(_) | FormulaValue::Blank => {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    FormulaValue::Error(error) => return Some(FormulaValue::Error(error)),
    _ => {}
  }
  let format = evaluator.text(&format_value);
  match value {
    FormulaValue::Error(error) => Some(FormulaValue::Error(error)),
    FormulaValue::Boolean(_) => Some(FormulaValue::String(Cow::Owned(display_text_from_value(
      &value,
    )))),
    FormulaValue::String(ref text) if text.is_empty() => {
      Some(FormulaValue::String(Cow::Borrowed("")))
    }
    FormulaValue::String(ref text) => {
      if let Some(number) = evaluator.date_number_from_scalar(&value)
        && let Some(text) = format_text_date(number, &format, evaluator.book.date_system)
      {
        return Some(FormulaValue::String(Cow::Owned(text)));
      }
      Some(FormulaValue::String(Cow::Owned(text.to_string())))
    }
    FormulaValue::Blank => format_text_number(0.0, &format, evaluator.book.date_system),
    value => {
      let Some(number) = evaluator.number(&value) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      };
      format_text_number(number, &format, evaluator.book.date_system)
    }
  }
}

fn format_text_number<'doc>(
  number: f64,
  format: &str,
  date_system: DateSystem,
) -> Option<FormulaValue<'doc>> {
  if let Some(text) = format_text_date(number, format, date_system) {
    return Some(FormulaValue::String(Cow::Owned(text)));
  }
  Some(FormulaValue::String(Cow::Owned(
    format_number_with_format_code(number, format)
      .unwrap_or_else(|| display_text_from_value(&FormulaValue::Number(number))),
  )))
}

fn format_text_date(number: f64, format: &str, date_system: DateSystem) -> Option<String> {
  let lower = format.to_ascii_lowercase();
  if !(lower.contains('y')
    || lower.contains('d')
    || lower.contains('h')
    || lower.contains('s')
    || lower.contains('m'))
  {
    return None;
  }
  let day = number.floor();
  if !day.is_finite() {
    return None;
  }
  let mut milliseconds = ((number - day) * 86_400_000.0).round() as i64;
  let day_adjust = milliseconds.div_euclid(86_400_000);
  milliseconds = milliseconds.rem_euclid(86_400_000);
  let (year, month, date) =
    date_from_serial_with_system(day as i32 + day_adjust as i32, date_system)?;
  let hour = milliseconds / 3_600_000;
  let minute = (milliseconds % 3_600_000) / 60_000;
  let second = (milliseconds % 60_000) / 1_000;
  let millisecond = milliseconds % 1_000;
  let mut output = String::new();
  let chars = format.chars().collect::<Vec<_>>();
  let mut index = 0usize;
  while index < chars.len() {
    let ch = chars[index];
    if ch == '"' {
      let quote_width = if chars.get(index + 1).is_some_and(|next| *next == '"') {
        2
      } else {
        1
      };
      index += quote_width;
      while index < chars.len() {
        if chars[index] == '"'
          && (quote_width == 1 || chars.get(index + 1).is_some_and(|next| *next == '"'))
        {
          index += quote_width;
          break;
        }
        output.push(chars[index]);
        index += 1;
      }
      continue;
    }
    if ch == '.'
      && chars
        .get(index + 1..index + 4)
        .is_some_and(|tail| tail == ['0', '0', '0'])
    {
      output.push_str(&format!(".{millisecond:03}"));
      index += 4;
      continue;
    }
    let upper = ch.to_ascii_uppercase();
    if matches!(upper, 'Y' | 'M' | 'D' | 'H' | 'S') {
      let start = index;
      while index < chars.len() && chars[index].to_ascii_uppercase() == upper {
        index += 1;
      }
      let len = index - start;
      match upper {
        'Y' => output.push_str(&format!("{year:0len$}")),
        'M' if len >= 3 => output.push_str(month_abbrev(month)),
        'M' if month_token_is_minute(&chars, start, index) => {
          output.push_str(&format!("{minute:0len$}"))
        }
        'M' => output.push_str(&format!("{month:0len$}")),
        'D' => output.push_str(&format!("{date:0len$}")),
        'H' => output.push_str(&format!("{hour:0len$}")),
        'S' => output.push_str(&format!("{second:0len$}")),
        _ => {}
      }
      continue;
    }
    output.push(ch);
    index += 1;
  }
  Some(output)
}

fn month_token_is_minute(chars: &[char], start: usize, end: usize) -> bool {
  if chars
    .get(start.wrapping_sub(1))
    .is_some_and(|ch| *ch == ':')
    || chars.get(end).is_some_and(|ch| *ch == ':')
  {
    return true;
  }
  chars[..start]
    .iter()
    .rev()
    .find_map(|ch| match ch.to_ascii_uppercase() {
      'H' | 'S' => Some(true),
      'Y' | 'D' => Some(false),
      _ => None,
    })
    .unwrap_or(false)
}

fn month_abbrev(month: u32) -> &'static str {
  match month {
    1 => "Jan",
    2 => "Feb",
    3 => "Mar",
    4 => "Apr",
    5 => "May",
    6 => "Jun",
    7 => "Jul",
    8 => "Aug",
    9 => "Sep",
    10 => "Oct",
    11 => "Nov",
    12 => "Dec",
    _ => "",
  }
}

fn textjoin_delimiters<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
) -> Result<Vec<String>, FormulaErrorValue> {
  let mut delimiters = Vec::new();
  let values = match value {
    FormulaValue::Reference(reference)
      if matches!(
        evaluator.grammar,
        FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1
      ) =>
    {
      textjoin_delimiter_reference_values(evaluator, reference)
    }
    FormulaValue::RefList(ranges)
      if matches!(
        evaluator.grammar,
        FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1
      ) =>
    {
      ranges
        .iter()
        .flat_map(|range| textjoin_delimiter_reference_values(evaluator, range))
        .collect()
    }
    value => evaluator.matrix_values(value),
  };
  for value in values.into_iter().flatten() {
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

fn textjoin_delimiter_reference_values<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  reference: &QualifiedRange<'doc>,
) -> Vec<Vec<FormulaValue<'doc>>> {
  let sheet = evaluator.range_sheet(reference);
  let row = reference.range.start.row.max(reference.range.end.row);
  let start_column = reference.range.start.column.min(reference.range.end.column);
  let end_column = reference.range.start.column.max(reference.range.end.column);
  vec![
    (start_column..=end_column)
      .map(|column| textjoin_cell_value(evaluator, sheet, CellAddress { column, row }))
      .collect(),
  ]
}

fn textjoin_value_text<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
  delimiters: &[String],
  ignore_empty: bool,
  output: &mut String,
  value_count: &mut usize,
) -> Result<(), FormulaErrorValue> {
  for value in textjoin_matrix_values(evaluator, value, ignore_empty)
    .into_iter()
    .flatten()
  {
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

fn textjoin_matrix_values<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
  ignore_empty: bool,
) -> Vec<Vec<FormulaValue<'doc>>> {
  match value {
    FormulaValue::Reference(reference) => {
      textjoin_reference_values(evaluator, reference, ignore_empty)
    }
    FormulaValue::RefList(ranges) => ranges
      .iter()
      .flat_map(|range| {
        textjoin_matrix_values(
          evaluator,
          &FormulaValue::Reference(range.clone()),
          ignore_empty,
        )
      })
      .collect(),
    value => evaluator.matrix_values(value),
  }
}

fn textjoin_reference_values<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  reference: &QualifiedRange<'doc>,
  ignore_empty: bool,
) -> Vec<Vec<FormulaValue<'doc>>> {
  let sheet = evaluator.range_sheet(reference);
  let range = if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
    let Some(range) = evaluator.book.data_area_subrange(sheet, reference.range) else {
      return Vec::new();
    };
    if ignore_empty && range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
      return sparse_textjoin_reference_values(evaluator, sheet, range);
    }
    range
  } else {
    reference.range
  };
  let start_row = range.start.row.min(range.end.row);
  let end_row = range.start.row.max(range.end.row);
  let start_column = range.start.column.min(range.end.column);
  let end_column = range.start.column.max(range.end.column);
  (start_row..=end_row)
    .map(|row| {
      (start_column..=end_column)
        .map(|column| textjoin_cell_value(evaluator, sheet, CellAddress { column, row }))
        .collect()
    })
    .collect()
}

fn textjoin_cell_value<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  sheet: SheetId,
  address: CellAddress,
) -> FormulaValue<'doc> {
  if evaluator.book.is_query_empty_cell(sheet, address) {
    FormulaValue::Blank
  } else {
    evaluator.book.cell_value(sheet, address)
  }
}

fn sparse_textjoin_reference_values<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  sheet: SheetId,
  range: CellRange,
) -> Vec<Vec<FormulaValue<'doc>>> {
  let start_row = range.start.row.min(range.end.row);
  let end_row = range.start.row.max(range.end.row);
  let start_column = range.start.column.min(range.end.column);
  let end_column = range.start.column.max(range.end.column);
  let mut cells = evaluator
    .book
    .cells
    .iter()
    .filter_map(|((cell_sheet, address), value)| {
      (*cell_sheet == sheet
        && address.row >= start_row
        && address.row <= end_row
        && address.column >= start_column
        && address.column <= end_column)
        .then(|| {
          (
            *address,
            if evaluator.book.is_query_empty_cell(sheet, *address) {
              FormulaValue::Blank
            } else {
              value.clone()
            },
          )
        })
    })
    .collect::<Vec<_>>();
  cells.sort_by_key(|(address, _)| (address.row, address.column));
  cells.into_iter().map(|(_, value)| vec![value]).collect()
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
  let left = evaluator.scalar_binary_operand(left);
  let right = evaluator.scalar_binary_operand(right);
  if let FormulaValue::Error(error) = left {
    return Some(FormulaValue::Error(error));
  }
  if let FormulaValue::Error(error) = right {
    return Some(FormulaValue::Error(error));
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
  if let FormulaValue::Error(error) = needle_value {
    return Some(FormulaValue::Error(error));
  }
  let haystack_value = args.array_value(1)?;
  if let FormulaValue::Error(error) = haystack_value {
    return Some(FormulaValue::Error(error));
  }
  let start = if args.raw_arg(2).is_some() {
    let value = args.value(2)?;
    if let FormulaValue::Error(error) = value {
      return Some(FormulaValue::Error(error));
    }
    let Some(start) = evaluator.number(&value) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    start
  } else {
    1.0
  } as usize;
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

fn evaluate_replace_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let old = match strict_text_arg(evaluator, args.value(0)?) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let start = match offset_integer_value(evaluator, args.value(1)?) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let count = match offset_integer_value(evaluator, args.value(2)?) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let new = match strict_text_arg(evaluator, args.value(3)?) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  if start < 1 || count < 0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }

  let mut chars = old.chars().collect::<Vec<_>>();
  if start > chars.len() as i64 {
    chars.extend(new.chars());
  } else {
    let start_index = (start - 1) as usize;
    let end_index = start_index.saturating_add(count as usize).min(chars.len());
    chars.splice(start_index..end_index, new.chars());
  }
  Some(FormulaValue::String(Cow::Owned(
    chars.into_iter().collect(),
  )))
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
    FormulaValue::String(text) => {
      values.push(evaluator.number(&FormulaValue::String(text)).unwrap_or(0.0))
    }
    FormulaValue::Blank => values.push(0.0),
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
  let left = evaluator
    .matrix_values(&left_value)
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
  let right = evaluator
    .matrix_values(&right_value)
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
  if left.is_empty() || left.len() != right.len() {
    return Some(FormulaValue::Error(FormulaErrorValue::NA));
  }
  let mut sum = KahanSum::default();
  let mut accumulated = false;
  let mut first_left_error = None;
  let mut first_right_error = None;
  for (left, right) in left.iter().zip(&right) {
    match (left, right) {
      (FormulaValue::Error(error), _) => {
        first_left_error.get_or_insert(*error);
      }
      (_, FormulaValue::Error(error)) => {
        first_right_error.get_or_insert(*error);
      }
      (FormulaValue::Blank | FormulaValue::String(_), _)
      | (_, FormulaValue::Blank | FormulaValue::String(_)) => {}
      (FormulaValue::Number(left), FormulaValue::Number(right)) => {
        accumulate(&mut sum, *left, *right);
        accumulated = true;
      }
      _ => {}
    }
  }
  if let Some(error) = first_left_error {
    return Some(FormulaValue::Error(error));
  }
  if let Some(error) = first_right_error {
    return Some(FormulaValue::Error(error));
  }
  if !accumulated {
    return Some(FormulaValue::Error(FormulaErrorValue::Div0));
  }
  Some(FormulaValue::Number(sum.finish()))
}

fn evaluate_devsq_reader<'doc>(
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  match args.numeric_aggregate(true)? {
    Ok(values) => deviation_sum_squares(&values)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Num))),
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
  let mut values = large_small_values(evaluator, &args.array_value(0)?);
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

fn large_small_values<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
) -> Vec<f64> {
  match value {
    FormulaValue::Reference(reference) => evaluator
      .range_values(reference)
      .into_iter()
      .filter_map(|value| match value {
        FormulaValue::Number(value) => Some(value),
        _ => None,
      })
      .collect(),
    FormulaValue::RefList(ranges) => ranges
      .iter()
      .flat_map(|range| evaluator.range_values(range))
      .filter_map(|value| match value {
        FormulaValue::Number(value) => Some(value),
        _ => None,
      })
      .collect(),
    FormulaValue::Matrix(rows) => rows
      .iter()
      .flatten()
      .filter_map(|value| match value {
        FormulaValue::Number(value) => Some(*value),
        _ => None,
      })
      .collect(),
    value => evaluator.number(value).into_iter().collect(),
  }
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
  let mut values = match percentile_numbers(evaluator, &args.value(0)?) {
    Ok(values) => values,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let k = evaluator.number(&args.value(1)?)?;
  if !(0.0..=1.0).contains(&k) {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
  percentile_sorted(&mut values, k, kind)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
}

fn evaluate_quartile_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  kind: PercentileKind,
) -> Option<FormulaValue<'doc>> {
  let mut values = match percentile_numbers(evaluator, &args.value(0)?) {
    Ok(values) => values,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let k = evaluator.number(&args.value(1)?)?.floor();
  percentile_sorted(&mut values, k / 4.0, kind)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
}

fn percentile_numbers<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
) -> std::result::Result<Vec<f64>, FormulaErrorValue> {
  match value {
    FormulaValue::Reference(reference) => {
      percentile_values_from_iter(evaluator.range_values(reference))
    }
    FormulaValue::RefList(ranges) => {
      let mut values = Vec::new();
      for range in ranges {
        values.extend(percentile_values_from_iter(evaluator.range_values(range))?);
      }
      Ok(values)
    }
    FormulaValue::Matrix(rows) => percentile_values_from_iter(rows.iter().flatten().cloned()),
    value => Ok(evaluator.number(value).into_iter().collect()),
  }
}

fn percentile_values_from_iter<'doc>(
  values: impl IntoIterator<Item = FormulaValue<'doc>>,
) -> std::result::Result<Vec<f64>, FormulaErrorValue> {
  let mut numbers = Vec::new();
  for value in values {
    match value {
      FormulaValue::Error(error) => return Err(error),
      value => {
        if let Some(number) = percentile_value_number(&value) {
          numbers.push(number);
        }
      }
    }
  }
  Ok(numbers)
}

fn percentile_value_number(value: &FormulaValue<'_>) -> Option<f64> {
  match value {
    FormulaValue::Number(value) => Some(*value),
    FormulaValue::String(text) => text.trim().parse::<f64>().ok(),
    _ => None,
  }
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
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
  if values.is_empty() {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
  let min = values.iter().copied().reduce(f64::min)?;
  let max = values.iter().copied().reduce(f64::max)?;
  if x < min || x > max {
    return Some(FormulaValue::Error(FormulaErrorValue::NA));
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
      let value = args.array_value(index).unwrap_or(FormulaValue::Number(0.0));
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
  let scalar_product = rows == 1 && columns == 1;
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
          FormulaValue::String(value) if scalar_product && !value.is_empty() => {
            return Some(FormulaValue::Error(FormulaErrorValue::Value));
          }
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
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  geometric: bool,
) -> Option<FormulaValue<'doc>> {
  let mut values = match args.numeric_aggregate(true)? {
    Ok(values) => values,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  values.extend(
    (0..args.len())
      .filter(|index| args.is_missing(*index))
      .map(|_| 0.0),
  );
  let calc_grammar = matches!(
    evaluator.grammar,
    FormulaGrammar::CalcA1 | FormulaGrammar::OpenFormula
  );
  if values.is_empty()
    || values.iter().any(|value| *value < 0.0)
    || (!geometric || !calc_grammar) && values.iter().any(|value| *value == 0.0)
  {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
  if geometric {
    if values.iter().any(|value| *value == 0.0) {
      return Some(FormulaValue::Number(0.0));
    }
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

fn evaluate_expand_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let matrix = evaluator.matrix_values(&args.array_value(0)?);
  let (source_rows, source_columns) = match rectangular_shape(&matrix) {
    Some(shape) => shape,
    None => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
  };
  let target_rows = optional_usize_arg(evaluator, args, 1, source_rows)?;
  let target_columns = optional_usize_arg(evaluator, args, 2, source_columns)?;
  if target_rows < source_rows || target_columns < source_columns {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let pad = args
    .raw_arg(3)
    .filter(|_| !args.is_missing(3))
    .and_then(|_| args.value(3))
    .unwrap_or(FormulaValue::Error(FormulaErrorValue::NA));
  let mut result = Vec::with_capacity(target_rows);
  for row in 0..target_rows {
    let mut result_row = Vec::with_capacity(target_columns);
    for column in 0..target_columns {
      result_row.push(
        matrix
          .get(row)
          .and_then(|source_row| source_row.get(column))
          .cloned()
          .unwrap_or_else(|| pad.clone()),
      );
    }
    result.push(result_row);
  }
  Some(FormulaValue::Matrix(result))
}

fn evaluate_torow_tocol_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  row_output: bool,
) -> Option<FormulaValue<'doc>> {
  let matrix = evaluator.matrix_values(&args.array_value(0)?);
  let (rows, columns) = match rectangular_shape(&matrix) {
    Some(shape) => shape,
    None => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
  };
  let ignore = optional_usize_arg(evaluator, args, 1, 0)?;
  if ignore > 3 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let scan_by_column = args
    .raw_arg(2)
    .filter(|_| !args.is_missing(2))
    .and_then(|_| args.value(2))
    .is_some_and(|value| evaluator.truthy(&value));
  let mut values = Vec::new();
  let outer = if scan_by_column { columns } else { rows };
  let inner = if scan_by_column { rows } else { columns };
  for i in 0..outer {
    for j in 0..inner {
      let row = if scan_by_column { j } else { i };
      let column = if scan_by_column { i } else { j };
      let value = matrix[row][column].clone();
      if (ignore == 1 || ignore == 3) && matches!(value, FormulaValue::Blank) {
        continue;
      }
      if (ignore == 2 || ignore == 3) && matches!(value, FormulaValue::Error(_)) {
        continue;
      }
      values.push(value);
    }
  }
  if values.is_empty() {
    return Some(FormulaValue::Error(FormulaErrorValue::NA));
  }
  Some(FormulaValue::Matrix(if row_output {
    vec![values]
  } else {
    values.into_iter().map(|value| vec![value]).collect()
  }))
}

fn evaluate_wrap_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  rows_mode: bool,
) -> Option<FormulaValue<'doc>> {
  let matrix = evaluator.matrix_values(&args.array_value(0)?);
  let (rows, columns) = match rectangular_shape(&matrix) {
    Some(shape) => shape,
    None => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
  };
  if rows > 1 && columns > 1 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let wrap = optional_usize_arg(evaluator, args, 1, 0)?;
  if wrap == 0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let pad = args
    .raw_arg(2)
    .filter(|_| !args.is_missing(2))
    .and_then(|_| args.value(2))
    .unwrap_or(FormulaValue::Error(FormulaErrorValue::NA));
  let mut values = Vec::with_capacity(rows * columns);
  for column in 0..columns {
    for row in 0..rows {
      values.push(matrix[row][column].clone());
    }
  }
  if values.is_empty() {
    return Some(FormulaValue::Error(FormulaErrorValue::NA));
  }
  let block_count = values.len().div_ceil(wrap);
  if rows_mode {
    let mut result = Vec::with_capacity(block_count);
    for row_index in 0..block_count {
      let mut row = Vec::with_capacity(wrap);
      for column in 0..wrap {
        row.push(
          values
            .get(row_index * wrap + column)
            .cloned()
            .unwrap_or_else(|| pad.clone()),
        );
      }
      result.push(row);
    }
    Some(FormulaValue::Matrix(result))
  } else {
    let mut result = vec![vec![pad.clone(); block_count]; wrap];
    for (index, value) in values.into_iter().enumerate() {
      result[index % wrap][index / wrap] = value;
    }
    Some(FormulaValue::Matrix(result))
  }
}

fn evaluate_unique_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let matrix = evaluator.matrix_values(&args.array_value(0)?);
  let (rows, columns) = match rectangular_shape(&matrix) {
    Some(shape) => shape,
    None => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
  };
  let by_column = args
    .raw_arg(1)
    .filter(|_| !args.is_missing(1))
    .and_then(|_| args.value(1))
    .is_some_and(|value| evaluator.truthy(&value));
  let exactly_once = args
    .raw_arg(2)
    .filter(|_| !args.is_missing(2))
    .and_then(|_| args.value(2))
    .is_some_and(|value| evaluator.truthy(&value));
  let unit_count = if by_column { columns } else { rows };
  let mut units = Vec::with_capacity(unit_count);
  for index in 0..unit_count {
    let key = if by_column {
      (0..rows)
        .map(|row| evaluator.text(&matrix[row][index]).to_lowercase())
        .collect::<Vec<_>>()
        .join("\u{1}")
    } else {
      (0..columns)
        .map(|column| evaluator.text(&matrix[index][column]).to_lowercase())
        .collect::<Vec<_>>()
        .join("\u{1}")
    };
    let count = units
      .iter()
      .filter(|(unit_key, _)| unit_key == &key)
      .count()
      + 1;
    units.push((key, count));
  }
  let mut keep = Vec::new();
  for (index, (key, count)) in units.iter().enumerate() {
    if *count != 1 {
      continue;
    }
    let total = units.iter().filter(|(unit_key, _)| unit_key == key).count();
    if !exactly_once || total == 1 {
      keep.push(index);
    }
  }
  if keep.is_empty() {
    return Some(FormulaValue::Error(FormulaErrorValue::NA));
  }
  Some(if by_column {
    FormulaValue::Matrix(
      (0..rows)
        .map(|row| {
          keep
            .iter()
            .map(|column| matrix[row][*column].clone())
            .collect()
        })
        .collect(),
    )
  } else {
    FormulaValue::Matrix(keep.into_iter().map(|row| matrix[row].clone()).collect())
  })
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

fn optional_usize_arg<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
  default: usize,
) -> Option<usize> {
  if args.raw_arg(index).is_none() || args.is_missing(index) {
    return Some(default);
  }
  let value = evaluator.number(&args.value(index)?)?.floor();
  if value < 0.0 || value > usize::MAX as f64 {
    return None;
  }
  Some(value as usize)
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
  let value = match scalar_number_arg_or_value(evaluator, args, 0)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let digits = if args.len() == 2 {
    match scalar_number_arg_or_value(evaluator, args, 1)? {
      Ok(value) => value,
      Err(error) => return Some(FormulaValue::Error(error)),
    }
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
    let Some(value) = evaluator.number(&args.value(1)?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    if value < 0.0 {
      approx_ceil(value)
    } else {
      approx_floor(value)
    }
  } else {
    0.0
  };
  let Some(value) = evaluator.number(&args.value(0)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
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
  let date_arg = |index| {
    let value = evaluator.scalar_binary_operand(args.value(index)?);
    if let FormulaValue::Error(error) = value {
      return Some(Err(error));
    }
    Some(
      evaluator
        .number(&value)
        .map(|value| value as i32)
        .ok_or(FormulaErrorValue::Value),
    )
  };
  let mut year = match date_arg(0)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let month = match date_arg(1)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let day = match date_arg(2)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
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

fn evaluate_basis_o_datetime_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let serial = match scalar_number_arg_or_value(evaluator, args, 0)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  basis_o_datetime_text(serial)
    .map(|value| FormulaValue::String(Cow::Owned(value)))
    .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
}

fn evaluate_weekday_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let Some(serial) = evaluator.date_number_from_scalar(&args.value(0)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  if serial < 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let return_type = if args.len() == 1 {
    1
  } else if args.is_missing(1) {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  } else {
    let value = args.value(1)?;
    match value {
      FormulaValue::Reference(reference) if reference.range.cell_count_hint() != 1 => {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      FormulaValue::RefList(_) => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
      FormulaValue::Matrix(ref rows)
        if rows.len() != 1 || rows.first().is_none_or(|row| row.len() != 1) =>
      {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      FormulaValue::Blank => return Some(FormulaValue::Error(FormulaErrorValue::Num)),
      _ => evaluator.number(&value)? as i32,
    }
  };
  let index = weekday_index_from_serial(serial.floor() as i64) as i32;
  let value = match return_type {
    1 => ((index + 1) % 7) + 1,
    2 | 11 => index + 1,
    3 => index,
    12..=17 => (index - (return_type - 11)).rem_euclid(7) + 1,
    _ => return Some(FormulaValue::Error(FormulaErrorValue::Num)),
  };
  Some(FormulaValue::Number(value as f64))
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
  if serial < 0 && evaluator.book.date_system != DateSystem::LibreOffice {
    return FormulaValue::Error(FormulaErrorValue::Num);
  }
  if serial == 0 && evaluator.book.date_system == DateSystem::Date1900 {
    return FormulaValue::Number(match part {
      DatePart::Year => 1900.0,
      DatePart::Month => 1.0,
      DatePart::Day => 0.0,
    });
  }
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

fn evaluate_months_years_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  years: bool,
) -> Option<FormulaValue<'doc>> {
  if args.is_missing(2) {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let Some(start) = evaluator.date_number_from_value(&args.value(0)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let Some(end) = evaluator.date_number_from_value(&args.value(1)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let mode = match scalar_number_arg_or_value(evaluator, args, 2)? {
    Ok(value) => approx_floor(value) as i32,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  if mode != 0 && mode != 1 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let start = start.floor() as i32;
  let end = end.floor() as i32;
  let Some(months) = date_diff_months_with_system(start, end, mode, evaluator.book.date_system)
  else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  Some(FormulaValue::Number(if years {
    if mode == 1 {
      let (start_year, _, _) = date_from_serial_with_system(start, evaluator.book.date_system)?;
      let (end_year, _, _) = date_from_serial_with_system(end, evaluator.book.date_system)?;
      (end_year - start_year) as f64
    } else {
      (months / 12) as f64
    }
  } else {
    months as f64
  }))
}

fn date_diff_months_with_system(
  start: i32,
  end: i32,
  mode: i32,
  date_system: DateSystem,
) -> Option<i32> {
  let (start_year, start_month, start_day) = date_from_serial_with_system(start, date_system)?;
  let (end_year, end_month, end_day) = date_from_serial_with_system(end, date_system)?;
  let mut result = end_month as i32 - start_month as i32 + (end_year - start_year) * 12;
  if mode == 1 || start == end {
    return Some(result);
  }
  if start < end {
    if start_day > end_day {
      result -= 1;
    }
  } else if start_day < end_day {
    result += 1;
  }
  Some(result)
}

fn evaluate_weeks_in_year_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let Some(serial) = evaluator.date_number_from_value(&args.value(0)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  weeks_in_year_from_serial_with_system(serial.floor() as i32, evaluator.book.date_system)
    .map(|value| FormulaValue::Number(value as f64))
    .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
}

fn evaluate_weeknum_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let Some(serial) = evaluator.date_number_from_value(&args.value(0)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let mode = if args.is_missing(1) {
    1
  } else {
    match scalar_number_arg_or_value(evaluator, args, 1)? {
      Ok(value) => approx_floor(value) as i32,
      Err(error) => return Some(FormulaValue::Error(error)),
    }
  };
  if mode == 21 || mode == 150 {
    return iso_weeknum_from_serial_with_system(serial.floor() as i32, evaluator.book.date_system)
      .map(|value| FormulaValue::Number(value as f64))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)));
  }
  let first_day = match mode {
    1 => 6,
    2 => 0,
    11..=17 => mode - 11,
    _ => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
  };
  let (year, _, _) =
    date_from_serial_with_system(serial.floor() as i32, evaluator.book.date_system)?;
  let year_start_serial = date_serial_with_system(year, 1, 1, evaluator.book.date_system)? as i32;
  let year_start_days = date_to_days(year, 1, 1)?;
  let first_day_in_year = (year_start_days - 1).rem_euclid(7) as i32;
  let offset = (first_day_in_year - first_day).rem_euclid(7);
  Some(FormulaValue::Number(
    ((serial.floor() as i32 - year_start_serial + offset) / 7 + 1) as f64,
  ))
}

fn evaluate_days_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let Some(end) = evaluator.date_number_from_value(&args.value(0)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let Some(start) = evaluator.date_number_from_value(&args.value(1)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  Some(FormulaValue::Number(end.floor() - start.floor()))
}

fn evaluate_days360_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let Some(start) = evaluator.date_number_from_value(&args.value(0)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let Some(end) = evaluator.date_number_from_value(&args.value(1)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let european = match args.raw_arg(2).and_then(|_| args.value(2)) {
    Some(value) => match boolean_method_argument(evaluator, &value) {
      Some(value) => value,
      None => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    },
    None => false,
  };
  date_days360(
    start.floor() as i32,
    end.floor() as i32,
    european,
    evaluator.book.date_system,
  )
  .map(|value| FormulaValue::Number(value as f64))
  .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
}

fn boolean_method_argument<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
) -> Option<bool> {
  match evaluator.scalar_value(value.clone()) {
    FormulaValue::Boolean(value) => Some(value),
    FormulaValue::Number(value) => Some(value != 0.0),
    FormulaValue::String(value) if value.eq_ignore_ascii_case("TRUE") => Some(true),
    FormulaValue::String(value) if value.eq_ignore_ascii_case("FALSE") => Some(false),
    FormulaValue::Blank => Some(false),
    _ => None,
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

fn evaluate_factorial_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  double_factorial: bool,
) -> Option<FormulaValue<'doc>> {
  let value = evaluator.scalar_binary_operand(args.value(0)?);
  if let FormulaValue::Error(error) = value {
    return Some(FormulaValue::Error(error));
  }
  let Some(number) = evaluator.number(&value) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let invalid_error = match evaluator.grammar {
    FormulaGrammar::CalcA1 | FormulaGrammar::OpenFormula => FormulaErrorValue::IllegalArgument,
    FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1 => FormulaErrorValue::Num,
  };
  if !number.is_finite() {
    return Some(FormulaValue::Error(invalid_error));
  }
  let n = number.trunc();
  if n < 0.0 {
    return Some(FormulaValue::Error(invalid_error));
  }
  let limit = if double_factorial { 300.0 } else { 170.0 };
  if n > limit {
    return Some(FormulaValue::Error(invalid_error));
  }
  let n = n as u32;
  let value = if double_factorial {
    double_factorial_value(n)
  } else {
    factorial_value(n)
  };
  if value.is_finite() {
    Some(FormulaValue::Number(value))
  } else {
    Some(FormulaValue::Error(invalid_error))
  }
}

fn factorial_value(n: u32) -> f64 {
  (1..=n).fold(1.0, |acc, item| acc * f64::from(item))
}

fn double_factorial_value(n: u32) -> f64 {
  if n <= 1 {
    return 1.0;
  }
  (1..=n)
    .rev()
    .step_by(2)
    .fold(1.0, |acc, item| acc * f64::from(item))
}

fn libreoffice_binomial_coefficient(mut n: f64, mut k: f64) -> f64 {
  k = approx_floor(k);
  if n < k {
    0.0
  } else if k == 0.0 {
    1.0
  } else {
    let mut value = n / k;
    n -= 1.0;
    k -= 1.0;
    while k > 0.0 {
      value *= n / k;
      n -= 1.0;
      k -= 1.0;
    }
    value
  }
}

fn evaluate_multinomial_reader<'doc>(
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let mut sum = 0.0;
  let mut result = 1.0;
  let mut seen = false;
  for index in 0..args.len() {
    for value in args.value_numbers(index)? {
      seen = true;
      let n = if value >= 0.0 {
        approx_floor(value)
      } else {
        approx_ceil(value)
      };
      if n < 0.0 {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      if n > 0.0 {
        sum += n;
        let coefficient = libreoffice_binomial_coefficient(sum, n);
        result *= coefficient;
      }
    }
  }
  Some(FormulaValue::Number(if seen { result } else { 0.0 }))
}

fn evaluate_seriessum_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let x = match scalar_number_arg_or_value(evaluator, args, 0)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let mut power = match scalar_number_arg_or_value(evaluator, args, 1)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let step = match scalar_number_arg_or_value(evaluator, args, 2)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  if x == 0.0 && power == 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
  let mut result = 0.0;
  if x != 0.0 {
    for coefficient in args.value_numbers(3)? {
      result += coefficient * x.powf(power);
      power += step;
    }
  }
  Some(if result.is_finite() {
    FormulaValue::Number(result)
  } else {
    FormulaValue::Error(FormulaErrorValue::Num)
  })
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
    .or(Some(FormulaValue::Error(FormulaErrorValue::Num)))
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
  if let FormulaValue::Error(error) = value {
    return Some(FormulaValue::Error(error));
  }
  let Some(value) = evaluator.time_number_from_value(&value) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  if value < 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
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
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let Some(months) = evaluator.number(&args.value(1)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let start = if (0.0..1.0).contains(&start) {
    1
  } else {
    start.floor() as i32
  };
  let months = months.floor() as i32;
  let Some((year, month, _)) = date_from_serial_with_system(start, evaluator.book.date_system)
  else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let Some(serial) = date_serial_with_system(
    year,
    month as i32 + months + 1,
    0,
    evaluator.book.date_system,
  ) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  Some(FormulaValue::Number(if serial < 0.0 {
    -1.0
  } else {
    serial
  }))
}

fn evaluate_edate_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let Some(start) = evaluator.date_number_from_value(&args.value(0)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let Some(months) = evaluator.number(&args.value(1)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let start = start.floor() as i32;
  let months = months.floor() as i32;
  let Some((year, month, day)) = date_from_serial_with_system(start, evaluator.book.date_system)
  else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let (target_year, target_month, _) = normalized_date_components(year, month as i32 + months, 1)?;
  let target_day = day.min(last_day_of_month(target_year, target_month));
  let Some(serial) = date_serial_with_system(
    target_year,
    target_month as i32,
    target_day as i32,
    evaluator.book.date_system,
  ) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  Some(FormulaValue::Number(if serial < 0.0 {
    -1.0
  } else {
    serial
  }))
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
  let value = evaluator.scalar_binary_operand(value);
  if let FormulaValue::Error(error) = value {
    return Some(FormulaValue::Error(error));
  }
  let len = evaluator.scalar_binary_operand(len);
  if let FormulaValue::Error(error) = len {
    return Some(FormulaValue::Error(error));
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
  let value = evaluator.scalar_binary_operand(value);
  if let FormulaValue::Error(error) = value {
    return Some(FormulaValue::Error(error));
  }
  let len = evaluator.scalar_binary_operand(len);
  if let FormulaValue::Error(error) = len {
    return Some(FormulaValue::Error(error));
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
  let text_value = evaluator.scalar_binary_operand(args.value(0)?);
  if let FormulaValue::Error(error) = text_value {
    return Some(FormulaValue::Error(error));
  }
  let start_value = evaluator.scalar_binary_operand(args.value(1)?);
  if let FormulaValue::Error(error) = start_value {
    return Some(FormulaValue::Error(error));
  }
  let len_value = evaluator.scalar_binary_operand(args.value(2)?);
  if let FormulaValue::Error(error) = len_value {
    return Some(FormulaValue::Error(error));
  }
  let text = evaluator.text(&text_value);
  let start = evaluator.number(&start_value)?;
  let len = evaluator.number(&len_value)?;
  if start < 1.0 || len < 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
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
  let value = args.value(0)?;
  if let FormulaValue::Error(error) = value {
    return Some(FormulaValue::Error(error));
  }
  let text = evaluator.text(&value);
  let use_a1 = if args.len() == 2 {
    let value = args.value(1)?;
    if let FormulaValue::Error(error) = value {
      return Some(FormulaValue::Error(error));
    }
    let Some(use_a1) = indirect_a1_flag(&value) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    use_a1
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
  if use_a1 && indirect_has_invalid_sheet_spacing(text) {
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
  if let Some(reference) = evaluator.resolve_reference(text) {
    if reference
      .sheet_name
      .as_ref()
      .is_some_and(|name| evaluator.book.sheet_id_by_name(name.0.as_ref()).is_none())
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Ref));
    }
    if !indirect_is_bare_whole_axis_reference(text, &reference) {
      return Some(FormulaValue::Reference(reference));
    }
  }
  evaluator
    .evaluate_defined_name(&Cow::Owned(text.to_string()))
    .or(Some(FormulaValue::Error(FormulaErrorValue::Ref)))
}

fn indirect_a1_flag(value: &FormulaValue<'_>) -> Option<bool> {
  match value {
    FormulaValue::Boolean(value) => Some(*value),
    FormulaValue::Number(value) => Some(*value != 0.0),
    FormulaValue::String(value) if value.eq_ignore_ascii_case("TRUE") => Some(true),
    FormulaValue::String(value) if value.eq_ignore_ascii_case("FALSE") => Some(false),
    _ => None,
  }
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

fn indirect_is_bare_whole_axis_reference(reference: &str, range: &QualifiedRange<'_>) -> bool {
  (range.start_flags.whole_column
    || range.start_flags.whole_row
    || range.end_flags.whole_column
    || range.end_flags.whole_row)
    && !contains_unquoted_char(reference, ':')
}

fn contains_unquoted_char(value: &str, target: char) -> bool {
  let mut quoted = false;
  let mut chars = value.chars().peekable();
  while let Some(ch) = chars.next() {
    match ch {
      '\'' => {
        if quoted && chars.peek().is_some_and(|next| *next == '\'') {
          chars.next();
        } else {
          quoted = !quoted;
        }
      }
      ch if !quoted && ch == target => return true,
      _ => {}
    }
  }
  false
}

fn indirect_has_invalid_sheet_spacing(reference: &str) -> bool {
  let mut quoted = false;
  let mut chars = reference.char_indices().peekable();
  while let Some((index, ch)) = chars.next() {
    match ch {
      '\'' => {
        if quoted && chars.peek().is_some_and(|(_, next)| *next == '\'') {
          chars.next();
        } else {
          quoted = !quoted;
        }
      }
      '!' if !quoted => {
        let sheet = &reference[..index];
        if sheet.trim() != sheet {
          return true;
        }
        if sheet.starts_with('\'') && sheet.ends_with('\'') && sheet.len() >= 2 {
          let inner = &sheet[1..sheet.len() - 1];
          return inner.trim() != inner;
        }
        return false;
      }
      _ => {}
    }
  }
  false
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
    let value = index_matrix(rows, row, column, args.len());
    return Some(if evaluator.array_context {
      value
    } else {
      evaluator.scalar_value(value)
    });
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
  let row_offset_value =
    raw_row_column_function_value(args, 1, false).or_else(|| offset_argument_value(args, 1))?;
  let column_offset_value =
    raw_row_column_function_value(args, 2, true).or_else(|| offset_argument_value(args, 2))?;
  let row_offset_value = offset_scalar_argument(evaluator, row_offset_value);
  let column_offset_value = offset_scalar_argument(evaluator, column_offset_value);
  let height = match offset_optional_integer_argument(
    evaluator,
    args,
    3,
    i64::from(reference.range.end.row - reference.range.start.row + 1),
  ) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let width = match offset_optional_integer_argument(
    evaluator,
    args,
    4,
    i64::from(reference.range.end.column - reference.range.start.column + 1),
  ) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
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
  let row_offset = match offset_integer_value(evaluator, row_offset_value) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let column_offset = match offset_integer_value(evaluator, column_offset_value) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
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

fn offset_argument_value<'doc>(
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
) -> Option<FormulaValue<'doc>> {
  if args.is_missing(index) {
    Some(FormulaValue::Number(0.0))
  } else {
    args.value(index)
  }
}

fn offset_scalar_argument<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: FormulaValue<'doc>,
) -> FormulaValue<'doc> {
  match value {
    FormulaValue::Reference(_) | FormulaValue::RefList(_) => evaluator.scalar_binary_operand(value),
    value => value,
  }
}

fn offset_optional_integer_argument<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
  default: i64,
) -> Result<i64, FormulaErrorValue> {
  match args.raw_arg(index).filter(|_| !args.is_missing(index)) {
    Some(_) => offset_integer_value(
      evaluator,
      args.value(index).ok_or(FormulaErrorValue::Value)?,
    ),
    None => Ok(default),
  }
}

fn offset_integer_value<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: FormulaValue<'doc>,
) -> Result<i64, FormulaErrorValue> {
  let value = evaluator.scalar_binary_operand(value);
  match value {
    FormulaValue::Error(error) => Err(error),
    value => evaluator
      .number(&value)
      .filter(|value| value.is_finite())
      .map(|value| value.floor() as i64)
      .ok_or(FormulaErrorValue::Value),
  }
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
  if matches!(
    evaluator.grammar,
    FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1
  ) && value * multiple < 0.0
  {
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
  let value_arg = args.value(0)?;
  if let FormulaValue::Error(error) = value_arg {
    return Some(FormulaValue::Error(error));
  }
  let Some(value) = evaluator.number(&value_arg) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let places = args
    .raw_arg(1)
    .filter(|_| !args.is_missing(1))
    .and_then(|_| args.value(1))
    .map(|value| {
      if let FormulaValue::Error(error) = value {
        return Err(error);
      }
      evaluator
        .number(&value)
        .map(|value| value as i32)
        .ok_or(FormulaErrorValue::Value)
    })
    .transpose();
  let places = match places {
    Ok(places) => places,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  if value.fract() != 0.0
    && let Some(places) = places
    && let Some(output) = convert_from_decimal(value, min, max, base, None, 10)
  {
    if places <= 0 || output.len() > places as usize {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    return Some(FormulaValue::String(Cow::Owned(output)));
  }
  convert_from_decimal(value, min, max, base, places, 10)
    .map(|value| FormulaValue::String(Cow::Owned(value)))
    .or(Some(FormulaValue::Error(FormulaErrorValue::Num)))
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

const BIT_FUNCTION_LIMIT: f64 = 281_474_976_710_656.0;

fn evaluate_bitwise_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  function: FormulaFunctionId,
) -> Option<FormulaValue<'doc>> {
  let left = match bit_function_arg(evaluator, args, 0)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let right = match bit_function_arg(evaluator, args, 1)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  Some(FormulaValue::Number(match function {
    FormulaFunctionId::Bitand => (left & right) as f64,
    FormulaFunctionId::Bitor => (left | right) as f64,
    FormulaFunctionId::Bitxor => (left ^ right) as f64,
    _ => unreachable!(),
  }))
}

fn evaluate_bitshift_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  left_shift: bool,
) -> Option<FormulaValue<'doc>> {
  let num = match bit_function_arg(evaluator, args, 0)? {
    Ok(value) => value as f64,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let shift = match scalar_number_arg_or_value(evaluator, args, 1)? {
    Ok(value) => approx_floor(value),
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let result = if shift < 0.0 {
    if left_shift {
      approx_floor(num / 2.0_f64.powf(-shift))
    } else {
      num * 2.0_f64.powf(-shift)
    }
  } else if shift == 0.0 {
    num
  } else if left_shift {
    num * 2.0_f64.powf(shift)
  } else {
    approx_floor(num / 2.0_f64.powf(shift))
  };
  Some(FormulaValue::Number(result))
}

fn bit_function_arg<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
) -> Option<std::result::Result<u64, FormulaErrorValue>> {
  let value = match scalar_number_arg_or_value(evaluator, args, index)? {
    Ok(value) => approx_floor(value),
    Err(error) => return Some(Err(error)),
  };
  if !(0.0..BIT_FUNCTION_LIMIT).contains(&value) {
    return Some(Err(FormulaErrorValue::IllegalArgument));
  }
  Some(Ok(value as u64))
}

fn evaluate_color_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let red = match color_component_arg(evaluator, args, 0)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let green = match color_component_arg(evaluator, args, 1)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let blue = match color_component_arg(evaluator, args, 2)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let alpha = if args.len() == 4 {
    match color_component_arg(evaluator, args, 3)? {
      Ok(value) => value,
      Err(error) => return Some(FormulaValue::Error(error)),
    }
  } else {
    0.0
  };
  Some(FormulaValue::Number(
    256.0 * 256.0 * 256.0 * alpha + 256.0 * 256.0 * red + 256.0 * green + blue,
  ))
}

fn color_component_arg<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
) -> Option<std::result::Result<f64, FormulaErrorValue>> {
  let value = match scalar_number_arg_or_value(evaluator, args, index)? {
    Ok(value) => approx_floor(value),
    Err(error) => return Some(Err(error)),
  };
  if !(0.0..=255.0).contains(&value) {
    return Some(Err(FormulaErrorValue::IllegalArgument));
  }
  Some(Ok(value))
}

fn evaluate_base_to_decimal_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  base: u32,
) -> Option<FormulaValue<'doc>> {
  convert_to_decimal(&base_digits_text(evaluator, &args.value(0)?), base, 10)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::Num)))
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

fn evaluate_bessel_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  kind: BesselKind,
) -> Option<FormulaValue<'doc>> {
  let Some(x) = evaluator.number(&args.value(0)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let Some(order) = evaluator.number(&args.value(1)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let order = approx_floor(order);
  if order < 0.0 || order > i32::MAX as f64 {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
  let value = bessel(kind, x, order as i32);
  Some(if value.is_finite() {
    FormulaValue::Number(value)
  } else {
    FormulaValue::Error(FormulaErrorValue::Num)
  })
}

fn evaluate_erf_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let lower = match scalar_number_arg_or_value(evaluator, args, 0)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let value = if args.len() == 2 {
    let upper = match scalar_number_arg_or_value(evaluator, args, 1)? {
      Ok(value) => value,
      Err(error) => return Some(FormulaValue::Error(error)),
    };
    erf(upper) - erf(lower)
  } else {
    erf(lower)
  };
  Some(if value.is_finite() {
    FormulaValue::Number(value)
  } else {
    FormulaValue::Error(FormulaErrorValue::Num)
  })
}

fn evaluate_erfc_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = match scalar_number_arg_or_value(evaluator, args, 0)? {
    Ok(value) => erfc(value),
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  Some(if value.is_finite() {
    FormulaValue::Number(value)
  } else {
    FormulaValue::Error(FormulaErrorValue::Num)
  })
}

fn evaluate_gestep_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let number = match scalar_number_arg_or_value(evaluator, args, 0)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let step = if args.len() == 2 {
    match scalar_number_arg_or_value(evaluator, args, 1)? {
      Ok(value) => value,
      Err(error) => return Some(FormulaValue::Error(error)),
    }
  } else {
    0.0
  };
  Some(FormulaValue::Number(if number >= step { 1.0 } else { 0.0 }))
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

fn evaluate_combin_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  repetition: bool,
) -> Option<FormulaValue<'doc>> {
  let count_arg = evaluator.scalar_binary_operand(args.value(0)?);
  if let FormulaValue::Error(error) = count_arg {
    return Some(FormulaValue::Error(error));
  }
  let chosen_arg = evaluator.scalar_binary_operand(args.value(1)?);
  if let FormulaValue::Error(error) = chosen_arg {
    return Some(FormulaValue::Error(error));
  }
  let Some(count) = evaluator.number(&count_arg).map(approx_floor) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let Some(chosen) = evaluator.number(&chosen_arg).map(approx_floor) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let invalid_error = match evaluator.grammar {
    FormulaGrammar::CalcA1 | FormulaGrammar::OpenFormula => FormulaErrorValue::IllegalArgument,
    FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1 => FormulaErrorValue::Num,
  };
  combination_count(count, chosen, repetition, log_gamma)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(invalid_error)))
}

fn evaluate_permutation_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  repetition: bool,
) -> Option<FormulaValue<'doc>> {
  let count_arg = args.value(0)?;
  let chosen_arg = args.value(1)?;
  if evaluator.array_context && (is_matrix_argument(&count_arg) || is_matrix_argument(&chosen_arg))
  {
    return evaluator.map_binary_values(count_arg, chosen_arg, |evaluator, count, chosen| {
      Some(evaluate_permutation_value(
        evaluator,
        count.clone(),
        chosen.clone(),
        repetition,
      ))
    });
  }
  Some(evaluate_permutation_value(
    evaluator, count_arg, chosen_arg, repetition,
  ))
}

fn evaluate_permutation_value<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  count_arg: FormulaValue<'doc>,
  chosen_arg: FormulaValue<'doc>,
  repetition: bool,
) -> FormulaValue<'doc> {
  let count_arg = evaluator.scalar_binary_operand(count_arg);
  if let FormulaValue::Error(error) = count_arg {
    return FormulaValue::Error(error);
  }
  let chosen_arg = evaluator.scalar_binary_operand(chosen_arg);
  if let FormulaValue::Error(error) = chosen_arg {
    return FormulaValue::Error(error);
  }
  let Some(count) = evaluator.number(&count_arg).map(approx_floor) else {
    return FormulaValue::Error(FormulaErrorValue::Value);
  };
  let Some(chosen) = evaluator.number(&chosen_arg).map(approx_floor) else {
    return FormulaValue::Error(FormulaErrorValue::Value);
  };
  let value = if repetition {
    permutation_with_repetition_count(count, chosen)
  } else {
    permutation_count(count, chosen, log_gamma)
  };
  value
    .map(FormulaValue::Number)
    .unwrap_or(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
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
  let Some(left) = matrix_number_values_strict(evaluator, &args.array_value(0)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let Some(right) = matrix_number_values_strict(evaluator, &args.array_value(1)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
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
  let Some(matrix) = matrix_number_values_strict(evaluator, &args.array_value(0)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let size = matrix.len();
  if size == 0 || matrix.iter().any(|row| row.len() != size) {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  Some(FormulaValue::Number(determinant(matrix)))
}

fn evaluate_minverse_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let Some(mut matrix) = matrix_number_values_strict(evaluator, &args.array_value(0)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let size = matrix.len();
  if size == 0 || matrix.iter().any(|row| row.len() != size) {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let Some(decomposition) = lup_decompose(&mut matrix) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  };
  let mut inverse = vec![vec![FormulaValue::Blank; size]; size];
  for column in 0..size {
    let mut rhs = vec![0.0; size];
    rhs[column] = 1.0;
    let Some(solution) = lup_solve(&matrix, &decomposition, &rhs) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    };
    for row in 0..size {
      inverse[row][column] = FormulaValue::Number(solution[row]);
    }
  }
  Some(FormulaValue::Matrix(inverse))
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

fn evaluate_transpose_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let matrix = evaluator.matrix_values(&args.array_value(0)?);
  let rows = matrix.len();
  let columns = matrix.first().map_or(0, Vec::len);
  if rows == 0 || columns == 0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let mut result = vec![Vec::with_capacity(rows); columns];
  for column in 0..columns {
    for row in &matrix {
      let value = row
        .get(column)
        .cloned()
        .unwrap_or(FormulaValue::Error(FormulaErrorValue::NA));
      result[column].push(if matches!(value, FormulaValue::Blank) {
        FormulaValue::Number(0.0)
      } else {
        value
      });
    }
  }
  Some(FormulaValue::Matrix(result))
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

fn ceiling_floor_has_array_argument<'doc>(args: FunctionArgReader<'_, '_, 'doc>) -> bool {
  (0..2).any(|index| {
    args
      .value(index)
      .is_some_and(|value| is_matrix_argument(&value))
  })
}

fn evaluate_ceiling_floor_legacy_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  ceiling: bool,
) -> Option<FormulaValue<'doc>> {
  let value_arg = evaluator.scalar_binary_operand(args.value(0)?);
  if let FormulaValue::Error(error) = value_arg {
    return Some(FormulaValue::Error(error));
  }
  let Some(value) = evaluator.number(&value_arg) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let significance_arg = evaluator.scalar_binary_operand(args.value(1)?);
  if let FormulaValue::Error(error) = significance_arg {
    return Some(FormulaValue::Error(error));
  }
  let Some(significance) = evaluator.number(&significance_arg) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let result = if ceiling {
    ceiling_excel_legacy(value, significance)
  } else {
    floor_excel_legacy(value, significance)
  };
  match result {
    Ok(value) => Some(FormulaValue::Number(value)),
    Err(NumericError::IllegalArgument) => Some(FormulaValue::Error(FormulaErrorValue::Num)),
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

fn evaluate_simple_stat_unary_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  op: impl Fn(f64) -> f64 + Copy,
) -> Option<FormulaValue<'doc>> {
  let value = args.value(0)?;
  if evaluator.array_context && is_matrix_argument(&value) {
    return map_numeric_array_values(evaluator, &[value], FormulaErrorValue::Value, |values| {
      let result = op(values[0]);
      if result.is_finite() {
        FormulaValue::Number(result)
      } else {
        FormulaValue::Error(FormulaErrorValue::Num)
      }
    });
  }
  match scalar_number_arg_or_value(evaluator, args, 0)? {
    Ok(value) => {
      let result = op(value);
      Some(if result.is_finite() {
        FormulaValue::Number(result)
      } else {
        FormulaValue::Error(FormulaErrorValue::Num)
      })
    }
    Err(error) => Some(FormulaValue::Error(error)),
  }
}

fn evaluate_fisher_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = args.value(0)?;
  if evaluator.array_context && is_matrix_argument(&value) {
    return map_numeric_array_values(evaluator, &[value], FormulaErrorValue::Value, |values| {
      fisher_value(values[0])
    });
  }
  let value = match scalar_number_arg_or_value(evaluator, args, 0)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  Some(fisher_value(value))
}

fn fisher_value<'doc>(value: f64) -> FormulaValue<'doc> {
  if value.abs() >= 1.0 {
    FormulaValue::Error(FormulaErrorValue::IllegalArgument)
  } else {
    FormulaValue::Number(value.atanh())
  }
}

fn evaluate_standardize_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let arg_values = (0..3)
    .map(|index| args.value(index))
    .collect::<Option<Vec<_>>>()?;
  if evaluator.array_context && arg_values.iter().any(is_matrix_argument) {
    return map_numeric_array_values(evaluator, &arg_values, FormulaErrorValue::Value, |values| {
      standardize_value(values[0], values[1], values[2])
    });
  }
  let x = match scalar_number_arg_or_value(evaluator, args, 0)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let mean = match scalar_number_arg_or_value(evaluator, args, 1)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let sigma = match scalar_number_arg_or_value(evaluator, args, 2)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  Some(standardize_value(x, mean, sigma))
}

fn standardize_value<'doc>(x: f64, mean: f64, sigma: f64) -> FormulaValue<'doc> {
  if sigma < 0.0 {
    FormulaValue::Error(FormulaErrorValue::IllegalArgument)
  } else if sigma == 0.0 {
    FormulaValue::Error(FormulaErrorValue::Div0)
  } else {
    FormulaValue::Number((x - mean) / sigma)
  }
}

fn evaluate_gammaln_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = args.value(0)?;
  if evaluator.array_context && is_matrix_argument(&value) {
    return map_numeric_array_values(evaluator, &[value], FormulaErrorValue::Value, |values| {
      gammaln_value(values[0])
    });
  }
  let value = match scalar_number_arg_or_value(evaluator, args, 0)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  Some(gammaln_value(value))
}

fn gammaln_value<'doc>(value: f64) -> FormulaValue<'doc> {
  if value > 0.0 {
    FormulaValue::Number(log_gamma(value))
  } else {
    FormulaValue::Error(FormulaErrorValue::IllegalArgument)
  }
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

fn evaluate_prob_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let x_values = evaluator.matrix_values(&args.array_value(0)?);
  let probabilities = evaluator.matrix_values(&args.array_value(1)?);
  let x_rows = x_values.len();
  let x_columns = x_values.first().map_or(0, Vec::len);
  let p_rows = probabilities.len();
  let p_columns = probabilities.first().map_or(0, Vec::len);
  if x_rows == 0
    || x_columns == 0
    || x_rows != p_rows
    || x_columns != p_columns
    || x_values.iter().any(|row| row.len() != x_columns)
    || probabilities.iter().any(|row| row.len() != x_columns)
  {
    return Some(FormulaValue::Error(FormulaErrorValue::NA));
  }

  let upper_index = if args.len() == 4 { 3 } else { 2 };
  let upper = match scalar_number_arg_or_value(evaluator, args, upper_index)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let lower = if args.len() == 4 {
    match scalar_number_arg_or_value(evaluator, args, 2)? {
      Ok(value) => value,
      Err(error) => return Some(FormulaValue::Error(error)),
    }
  } else {
    upper
  };
  let (lower, upper) = if lower > upper {
    (upper, lower)
  } else {
    (lower, upper)
  };

  let mut probability_sum = KahanSum::default();
  let mut result = KahanSum::default();
  for (x_row, probability_row) in x_values.iter().zip(&probabilities) {
    for (x_value, probability) in x_row.iter().zip(probability_row) {
      let x_value = match x_value {
        FormulaValue::Number(value) => *value,
        FormulaValue::Error(error) => return Some(FormulaValue::Error(*error)),
        _ => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
      };
      let probability = match probability {
        FormulaValue::Number(value) => *value,
        FormulaValue::Error(error) => return Some(FormulaValue::Error(*error)),
        _ => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
      };
      if !(0.0..=1.0).contains(&probability) {
        return Some(FormulaValue::Error(FormulaErrorValue::NA));
      }
      probability_sum.add(probability);
      if x_value >= lower && x_value <= upper {
        result.add(probability);
      }
    }
  }
  if (probability_sum.finish() - 1.0).abs() > 1.0e-7 {
    return Some(FormulaValue::Error(FormulaErrorValue::NA));
  }
  Some(FormulaValue::Number(result.finish()))
}

fn map_numeric_array_values<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  values: &[FormulaValue<'doc>],
  invalid_error: FormulaErrorValue,
  op: impl Fn(&[f64]) -> FormulaValue<'doc> + Copy,
) -> Option<FormulaValue<'doc>> {
  let matrices = values
    .iter()
    .map(|value| evaluator.matrix_values(value))
    .collect::<Vec<_>>();
  let mut rows = 1usize;
  let mut columns = 1usize;
  let mut dimensions = Vec::with_capacity(matrices.len());
  for matrix in &matrices {
    let row_count = matrix.len();
    let column_count = matrix.first().map_or(0, Vec::len);
    if row_count == 0 || column_count == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    rows = rows.max(row_count);
    columns = columns.max(column_count);
    dimensions.push((row_count, column_count));
  }
  if dimensions.iter().any(|(row_count, column_count)| {
    !matrix_can_broadcast_local(*row_count, *column_count, rows, columns)
  }) {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }

  let mut result = Vec::with_capacity(rows);
  for row in 0..rows {
    let mut result_row = Vec::with_capacity(columns);
    for column in 0..columns {
      let mut args = Vec::with_capacity(matrices.len());
      let mut error = None;
      for (matrix, (row_count, column_count)) in matrices.iter().zip(dimensions.iter().copied()) {
        let value = &matrix[row.min(row_count - 1)][column.min(column_count - 1)];
        match value {
          FormulaValue::Error(value_error) => {
            error = Some(*value_error);
            break;
          }
          _ => match evaluator.number(value) {
            Some(number) => args.push(number),
            None => {
              error = Some(invalid_error);
              break;
            }
          },
        }
      }
      result_row.push(error.map_or_else(|| op(&args), FormulaValue::Error));
    }
    result.push(result_row);
  }
  if rows == 1 && columns == 1 {
    return result.into_iter().next()?.into_iter().next();
  }
  Some(FormulaValue::Matrix(result))
}

fn evaluate_log_values<'doc>(value: f64, base: f64) -> FormulaValue<'doc> {
  if value > 0.0 && base > 0.0 && base != 1.0 {
    FormulaValue::Number(value.log(base))
  } else {
    FormulaValue::Error(FormulaErrorValue::IllegalArgument)
  }
}

fn evaluate_hypgeom_dist_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let arg_values = (0..args.len())
    .map(|index| args.value(index))
    .collect::<Option<Vec<_>>>()?;
  if evaluator.array_context && arg_values.iter().any(is_matrix_argument) {
    return map_numeric_array_values(
      evaluator,
      &arg_values,
      FormulaErrorValue::Unknown,
      |values| {
        evaluate_hypgeom_dist_values(
          values[0],
          values[1],
          values[2],
          values[3],
          values.get(4).copied().unwrap_or(0.0) != 0.0,
        )
      },
    );
  }
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
      FormulaValue::Number(value) => Some(value != 0.0),
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
  Some(evaluate_hypgeom_dist_values(
    sample_success,
    sample_size,
    population_success,
    population_size,
    cumulative,
  ))
}

fn evaluate_hypgeom_dist_values<'doc>(
  sample_success: f64,
  sample_size: f64,
  population_success: f64,
  population_size: f64,
  cumulative: bool,
) -> FormulaValue<'doc> {
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
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  let Some(dist) = Hypergeometric::new(
    population_size as u64,
    population_success as u64,
    sample_size as u64,
  )
  .ok() else {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  };
  FormulaValue::Number(if cumulative {
    dist.cdf(sample_success as u64)
  } else {
    dist.pmf(sample_success as u64)
  })
}

fn evaluate_lognorm_inv_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let arg_values = (0..args.len())
    .map(|index| args.value(index))
    .collect::<Option<Vec<_>>>()?;
  if evaluator.array_context && arg_values.iter().any(is_matrix_argument) {
    return map_numeric_array_values(
      evaluator,
      &arg_values,
      FormulaErrorValue::Unknown,
      |values| evaluate_lognorm_inv_values(values[0], values[1], values[2]),
    );
  }
  let Some(p) = args.scalar_number(0) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  let Some(mean) = args.scalar_number(1) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  let Some(sigma) = args.scalar_number(2) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  };
  Some(evaluate_lognorm_inv_values(p, mean, sigma))
}

fn evaluate_lognorm_inv_values<'doc>(p: f64, mean: f64, sigma: f64) -> FormulaValue<'doc> {
  if p <= 0.0 || p >= 1.0 || sigma <= 0.0 {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  let Some(dist) = LogNormal::new(mean, sigma).ok() else {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  };
  FormulaValue::Number(dist.inverse_cdf(p))
}

fn evaluate_lognorm_dist_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let arg_values = (0..args.len())
    .map(|index| args.value(index))
    .collect::<Option<Vec<_>>>()?;
  if evaluator.array_context && arg_values.iter().any(is_matrix_argument) {
    return map_numeric_array_values(evaluator, &arg_values, FormulaErrorValue::Value, |values| {
      evaluate_lognorm_dist_values(
        values[0],
        values.get(1).copied().unwrap_or(0.0),
        values.get(2).copied().unwrap_or(1.0),
        values.get(3).copied().unwrap_or(1.0) != 0.0,
      )
    });
  }
  let x = match scalar_number_arg_or_value(evaluator, args, 0)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let mean = if args.len() >= 2 {
    match scalar_number_arg_or_value(evaluator, args, 1)? {
      Ok(value) => value,
      Err(error) => return Some(FormulaValue::Error(error)),
    }
  } else {
    0.0
  };
  let sigma = if args.len() >= 3 {
    match scalar_number_arg_or_value(evaluator, args, 2)? {
      Ok(value) => value,
      Err(error) => return Some(FormulaValue::Error(error)),
    }
  } else {
    1.0
  };
  let cumulative = if args.len() == 4 {
    evaluator.truthy(&args.value(3)?)
  } else {
    true
  };
  Some(evaluate_lognorm_dist_values(x, mean, sigma, cumulative))
}

fn evaluate_lognorm_dist_values<'doc>(
  x: f64,
  mean: f64,
  sigma: f64,
  cumulative: bool,
) -> FormulaValue<'doc> {
  if sigma <= 0.0 {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  if x <= 0.0 {
    return if cumulative {
      FormulaValue::Number(0.0)
    } else {
      FormulaValue::Error(FormulaErrorValue::IllegalArgument)
    };
  }
  let z = (x.ln() - mean) / sigma;
  FormulaValue::Number(if cumulative {
    lo_integral_phi(z)
  } else {
    lo_phi(z) / sigma / x
  })
}

fn evaluate_numeric_args_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  invalid_error: FormulaErrorValue,
  op: impl Fn(&[f64]) -> FormulaValue<'doc> + Copy,
) -> Option<FormulaValue<'doc>> {
  let arg_values = (0..args.len())
    .map(|index| args.value(index))
    .collect::<Option<Vec<_>>>()?;
  if evaluator.array_context && arg_values.iter().any(is_matrix_argument) {
    return map_numeric_array_values(evaluator, &arg_values, invalid_error, op);
  }
  let mut values = Vec::with_capacity(args.len());
  for index in 0..args.len() {
    match scalar_number_arg_or_value(evaluator, args, index)? {
      Ok(value) => values.push(value),
      Err(error) => return Some(FormulaValue::Error(error)),
    }
  }
  Some(op(&values))
}

fn finite_number_or_num<'doc>(value: f64) -> FormulaValue<'doc> {
  if value.is_finite() {
    FormulaValue::Number(value)
  } else {
    FormulaValue::Error(FormulaErrorValue::Num)
  }
}

fn evaluate_gamma_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, |values| {
    let x = values[0];
    if x <= 0.0 && x == approx_floor(x) {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    finite_number_or_num(lo_gamma(x))
  })
}

fn evaluate_b_function_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, |values| {
    if values.len() == 3 {
      binom_dist_value(values[2], values[0], values[1], false)
    } else {
      binom_dist_range_value(values[0], values[1], values[2], Some(values[3]))
    }
  })
}

fn evaluate_beta_dist_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  microsoft: bool,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, move |values| {
    beta_dist_value(values, microsoft)
  })
}

fn beta_dist_value<'doc>(values: &[f64], microsoft: bool) -> FormulaValue<'doc> {
  let x = values[0];
  let alpha = values[1];
  let beta = values[2];
  let (lower, upper, cumulative) = if microsoft {
    (
      values.get(4).copied().unwrap_or(0.0),
      values.get(5).copied().unwrap_or(1.0),
      values[3] != 0.0,
    )
  } else {
    (
      values.get(3).copied().unwrap_or(0.0),
      values.get(4).copied().unwrap_or(1.0),
      values.get(5).copied().unwrap_or(1.0) != 0.0,
    )
  };
  let scale = upper - lower;
  if alpha <= 0.0 || beta <= 0.0 || scale <= 0.0 {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  if microsoft && (x < lower || x > upper) {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  if cumulative {
    if x < lower {
      return FormulaValue::Number(0.0);
    }
    if x > upper {
      return FormulaValue::Number(1.0);
    }
    FormulaValue::Number(lo_beta_dist((x - lower) / scale, alpha, beta))
  } else {
    if x < lower || x > upper {
      return FormulaValue::Number(0.0);
    }
    let mut standard_x = (x - lower) / scale;
    if !microsoft && alpha != 1.0 && standard_x == 1.0 && beta < 1.0 {
      standard_x = 1.0 - f64::EPSILON / 2.0;
    }
    match lo_beta_dist_pdf(standard_x, alpha, beta) {
      Ok(value) => finite_number_or_num(value / scale),
      Err(error) => FormulaValue::Error(special_error_value(error)),
    }
  }
}

fn evaluate_beta_inv_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, |values| {
    let p = values[0];
    let alpha = values[1];
    let beta = values[2];
    let lower = values.get(3).copied().unwrap_or(0.0);
    let upper = values.get(4).copied().unwrap_or(1.0);
    if !(0.0..=1.0).contains(&p) || lower >= upper || alpha <= 0.0 || beta <= 0.0 {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    match lo_iterate_inverse(|x| p - lo_beta_dist(x, alpha, beta), 0.0, 1.0) {
      Ok(value) => FormulaValue::Number(lower + value * (upper - lower)),
      Err(error) => FormulaValue::Error(special_error_value(error)),
    }
  })
}

fn evaluate_binom_dist_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, |values| {
    binom_dist_value(values[0], values[1], values[2], values[3] != 0.0)
  })
}

fn evaluate_binom_dist_range_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, |values| {
    binom_dist_range_value(values[0], values[1], values[2], values.get(3).copied())
  })
}

fn binom_dist_value<'doc>(x: f64, n: f64, p: f64, cumulative: bool) -> FormulaValue<'doc> {
  let x = approx_floor(x);
  let n = approx_floor(n);
  if n < 0.0 || x < 0.0 || x > n || !(0.0..=1.0).contains(&p) {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  if p == 0.0 {
    return FormulaValue::Number(if x == 0.0 || cumulative { 1.0 } else { 0.0 });
  }
  if p == 1.0 {
    return FormulaValue::Number(if x == n { 1.0 } else { 0.0 });
  }
  if !cumulative {
    return finite_number_or_num(lo_binom_dist_pmf(x, n, p));
  }
  if x == n {
    return FormulaValue::Number(1.0);
  }
  let q = (0.5 - p) + 0.5;
  let mut factor = q.powf(n);
  if x == 0.0 {
    return finite_number_or_num(factor);
  }
  if factor <= f64::MIN_POSITIVE {
    factor = p.powf(n);
    if factor <= f64::MIN_POSITIVE {
      finite_number_or_num(lo_beta_dist(q, n - x, x + 1.0))
    } else if factor > f64::EPSILON {
      let mut sum = 1.0 - factor;
      for i in 0..((n - x) as u32).saturating_sub(1) {
        factor *= (n - f64::from(i)) / f64::from(i + 1) * q / p;
        sum -= factor;
      }
      FormulaValue::Number(sum.max(0.0))
    } else {
      finite_number_or_num(lo_binom_dist_range(n, n - x, n, factor, q, p))
    }
  } else {
    finite_number_or_num(lo_binom_dist_range(n, 0.0, x, factor, p, q))
  }
}

fn binom_dist_range_value<'doc>(n: f64, p: f64, xs: f64, xe: Option<f64>) -> FormulaValue<'doc> {
  let n = approx_floor(n);
  let xs = approx_floor(xs);
  let xe = approx_floor(xe.unwrap_or(xs));
  let valid_x = 0.0 <= xs && xs <= xe && xe <= n;
  if !valid_x {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  if !(0.0..=1.0).contains(&p) {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  if p == 0.0 {
    return FormulaValue::Number(if xs == 0.0 { 1.0 } else { 0.0 });
  }
  if p == 1.0 {
    return FormulaValue::Number(if xe == n { 1.0 } else { 0.0 });
  }
  let q = (0.5 - p) + 0.5;
  if xs == xe {
    return finite_number_or_num(lo_binom_dist_pmf(xs, n, p));
  }
  let mut factor = q.powf(n);
  if factor > f64::MIN_POSITIVE {
    finite_number_or_num(lo_binom_dist_range(n, xs, xe, factor, p, q))
  } else {
    factor = p.powf(n);
    if factor > f64::MIN_POSITIVE {
      finite_number_or_num(lo_binom_dist_range(n, n - xe, n - xs, factor, q, p))
    } else {
      finite_number_or_num(lo_beta_dist(q, n - xe, xe + 1.0) - lo_beta_dist(q, n - xs + 1.0, xs))
    }
  }
}

fn evaluate_binom_inv_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, |values| {
    binom_inv_value(values[0], values[1], values[2])
  })
}

fn binom_inv_value<'doc>(n: f64, p: f64, alpha: f64) -> FormulaValue<'doc> {
  let n = approx_floor(n);
  if n < 0.0 || !(0.0..=1.0).contains(&alpha) || !(0.0..=1.0).contains(&p) {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  if alpha == 0.0 {
    return FormulaValue::Number(0.0);
  }
  if alpha == 1.0 {
    return FormulaValue::Number(if p == 0.0 { 0.0 } else { n });
  }
  let q = (0.5 - p) + 0.5;
  if q > p {
    let mut factor = q.powf(n);
    let mut sum = factor;
    for i in 0..n as u32 {
      if sum >= alpha {
        return FormulaValue::Number(f64::from(i));
      }
      factor *= (n - f64::from(i)) / f64::from(i + 1) * p / q;
      sum += factor;
    }
    FormulaValue::Number(n)
  } else {
    let mut factor = p.powf(n);
    let mut sum = 1.0 - factor;
    for i in 0..n as u32 {
      if sum < alpha {
        return FormulaValue::Number(n - f64::from(i));
      }
      factor *= (n - f64::from(i)) / f64::from(i + 1) * q / p;
      sum -= factor;
    }
    FormulaValue::Number(0.0)
  }
}

fn evaluate_chisq_dist_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  microsoft: bool,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, move |values| {
    let x = values[0];
    let df = approx_floor(values[1]);
    let cumulative = values.get(2).copied().unwrap_or(1.0) != 0.0;
    if df < 1.0 || (microsoft && (x < 0.0 || df > 1.0e10)) {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    FormulaValue::Number(if cumulative {
      lo_chisq_dist_cdf(x, df)
    } else {
      lo_chisq_dist_pdf(x, df)
    })
  })
}

fn evaluate_chisq_dist_rt_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, |values| {
    let x = values[0];
    let df = approx_floor(values[1]);
    if x < 0.0 || df < 1.0 {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    FormulaValue::Number(lo_chi_dist(x, df))
  })
}

fn evaluate_expon_dist_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, |values| {
    let x = values[0];
    let lambda = values[1];
    if lambda <= 0.0 {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    FormulaValue::Number(if values[2] == 0.0 {
      if x >= 0.0 {
        lambda * (-lambda * x).exp()
      } else {
        0.0
      }
    } else if x > 0.0 {
      1.0 - (-lambda * x).exp()
    } else {
      0.0
    })
  })
}

fn evaluate_f_dist_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, |values| {
    let x = values[0];
    let df1 = approx_floor(values[1]);
    let df2 = approx_floor(values[2]);
    if x < 0.0 || df1 < 1.0 || df2 < 1.0 || df1 >= 1.0e10 || df2 >= 1.0e10 {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    if values.get(3).copied().unwrap_or(1.0) != 0.0 {
      FormulaValue::Number(1.0 - lo_f_dist_right_tail(x, df1, df2))
    } else {
      finite_number_or_num(lo_f_dist_pdf(x, df1, df2))
    }
  })
}

fn evaluate_f_dist_rt_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, |values| {
    let x = values[0];
    let df1 = approx_floor(values[1]);
    let df2 = approx_floor(values[2]);
    if x < 0.0 || df1 < 1.0 || df2 < 1.0 || df1 >= 1.0e10 || df2 >= 1.0e10 {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    FormulaValue::Number(lo_f_dist_right_tail(x, df1, df2))
  })
}

fn evaluate_f_inv_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  right_tail: bool,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, move |values| {
    let p = values[0];
    let df1 = approx_floor(values[1]);
    let df2 = approx_floor(values[2]);
    if p <= 0.0 || p > 1.0 || df1 < 1.0 || df2 < 1.0 || df1 >= 1.0e10 || df2 >= 1.0e10 {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    let target = if right_tail { p } else { 1.0 - p };
    match lo_iterate_inverse(
      |x| target - lo_f_dist_right_tail(x, df1, df2),
      df1 * 0.5,
      df1,
    ) {
      Ok(value) => FormulaValue::Number(value),
      Err(error) => FormulaValue::Error(special_error_value(error)),
    }
  })
}

fn evaluate_gamma_dist_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  microsoft: bool,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, move |values| {
    let x = values[0];
    let alpha = values[1];
    let beta = values[2];
    if (microsoft && x < 0.0) || alpha <= 0.0 || beta <= 0.0 {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    if values.get(3).copied().unwrap_or(1.0) != 0.0 {
      FormulaValue::Number(lo_gamma_dist(x, alpha, beta))
    } else {
      match lo_gamma_dist_pdf(x, alpha, beta) {
        Ok(value) => finite_number_or_num(value),
        Err(error) => FormulaValue::Error(special_error_value(error)),
      }
    }
  })
}

fn evaluate_gamma_inv_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, |values| {
    let p = values[0];
    let alpha = values[1];
    let beta = values[2];
    if alpha <= 0.0 || beta <= 0.0 || p < 0.0 || p >= 1.0 {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    if p == 0.0 {
      return FormulaValue::Number(0.0);
    }
    let start = alpha * beta;
    match lo_iterate_inverse(|x| p - lo_gamma_dist(x, alpha, beta), start * 0.5, start) {
      Ok(value) => FormulaValue::Number(value),
      Err(error) => FormulaValue::Error(special_error_value(error)),
    }
  })
}

fn evaluate_negbinom_dist_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  microsoft: bool,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, move |values| {
    let failures = approx_floor(values[0]);
    let successes = approx_floor(values[1]);
    let p = values[2];
    if if microsoft {
      successes < 1.0 || failures < 0.0 || !(0.0..=1.0).contains(&p)
    } else {
      failures + successes <= 1.0 || !(0.0..=1.0).contains(&p)
    } {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    let q = 1.0 - p;
    if microsoft && values[3] != 0.0 {
      return FormulaValue::Number(1.0 - lo_beta_dist(q, failures + 1.0, successes));
    }
    let mut factor = p.powf(successes);
    for i in 0..failures as u32 {
      factor *= (f64::from(i) + successes) / (f64::from(i) + 1.0) * q;
    }
    finite_number_or_num(factor)
  })
}

fn evaluate_t_dist_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, |values| {
    let df = approx_floor(values[1]);
    if df < 1.0 {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    FormulaValue::Number(lo_t_dist(
      values[0],
      df,
      if values[2] != 0.0 { 4 } else { 3 },
    ))
  })
}

fn evaluate_t_dist_tail_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  tails: i32,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, move |values| {
    let t = values[0];
    let df = approx_floor(values[1]);
    if df < 1.0 || (tails == 2 && t < 0.0) {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    let result = lo_t_dist(t, df, tails);
    FormulaValue::Number(if tails == 1 && t < 0.0 {
      1.0 - result
    } else {
      result
    })
  })
}

fn evaluate_tdist_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, |values| {
    let t = values[0];
    let df = approx_floor(values[1]);
    let tails = approx_floor(values[2]) as i32;
    if df < 1.0 || t < 0.0 || !(tails == 1 || tails == 2) {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    FormulaValue::Number(lo_t_dist(t, df, tails))
  })
}

fn evaluate_t_inv_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  kind: i32,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, move |values| {
    let p = values[0];
    let df = approx_floor(values[1]);
    if df < 1.0 || p <= 0.0 || p > 1.0 || (kind == 4 && p == 1.0) {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    if kind == 4 && p < 0.5 {
      return match t_inv_value(1.0 - p, df, kind) {
        FormulaValue::Number(value) => FormulaValue::Number(-value),
        value => value,
      };
    }
    t_inv_value(p, df, kind)
  })
}

fn t_inv_value<'doc>(p: f64, df: f64, kind: i32) -> FormulaValue<'doc> {
  match lo_iterate_inverse(|x| p - lo_t_dist(x, df, kind), df * 0.5, df) {
    Ok(value) => FormulaValue::Number(value),
    Err(error) => FormulaValue::Error(special_error_value(error)),
  }
}

fn evaluate_weibull_dist_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  evaluate_numeric_args_reader(evaluator, args, FormulaErrorValue::Value, |values| {
    let x = values[0];
    let alpha = values[1];
    let beta = values[2];
    if alpha <= 0.0 || beta <= 0.0 || x < 0.0 {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    finite_number_or_num(if values[3] == 0.0 {
      alpha / beta.powf(alpha) * x.powf(alpha - 1.0) * (-(x / beta).powf(alpha)).exp()
    } else {
      1.0 - (-(x / beta).powf(alpha)).exp()
    })
  })
}

fn evaluate_norm_inv_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  standard: bool,
) -> Option<FormulaValue<'doc>> {
  let arg_values = (0..args.len())
    .map(|index| args.value(index))
    .collect::<Option<Vec<_>>>()?;
  if evaluator.array_context && arg_values.iter().any(is_matrix_argument) {
    return map_numeric_array_values(evaluator, &arg_values, FormulaErrorValue::Value, |values| {
      let (mean, sigma) = if standard {
        (0.0, 1.0)
      } else {
        (values[1], values[2])
      };
      evaluate_norm_inv_values(values[0], mean, sigma, standard)
    });
  }
  let Some(p) = args.scalar_number(0) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let (mean, sigma) = if standard {
    (0.0, 1.0)
  } else {
    let Some(mean) = args.scalar_number(1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let Some(sigma) = args.scalar_number(2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    (mean, sigma)
  };
  Some(evaluate_norm_inv_values(p, mean, sigma, standard))
}

fn evaluate_norm_inv_values<'doc>(
  p: f64,
  mean: f64,
  sigma: f64,
  standard: bool,
) -> FormulaValue<'doc> {
  if sigma <= 0.0 || !(0.0..=1.0).contains(&p) {
    return FormulaValue::Error(FormulaErrorValue::Num);
  }
  if p == 0.0 || p == 1.0 {
    return FormulaValue::Error(FormulaErrorValue::Num);
  }
  if standard {
    FormulaValue::Number(norm_s_inv(p))
  } else {
    let Some(dist) = Normal::new(mean, sigma).ok() else {
      return FormulaValue::Error(FormulaErrorValue::Num);
    };
    FormulaValue::Number(dist.inverse_cdf(p))
  }
}

fn evaluate_norm_dist_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  standard: bool,
) -> Option<FormulaValue<'doc>> {
  let arg_values = (0..args.len())
    .map(|index| args.value(index))
    .collect::<Option<Vec<_>>>()?;
  if evaluator.array_context && arg_values.iter().any(is_matrix_argument) {
    return map_numeric_array_values(evaluator, &arg_values, FormulaErrorValue::Value, |values| {
      let (mean, sigma, cumulative) = if standard {
        (0.0, 1.0, values[1] != 0.0)
      } else {
        (values[1], values[2], values[3] != 0.0)
      };
      evaluate_norm_dist_values(values[0], mean, sigma, cumulative)
    });
  }
  let Some(x) = args.scalar_number(0) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let (mean, sigma, cumulative_index) = if standard {
    (0.0, 1.0, 1)
  } else {
    let Some(mean) = args.scalar_number(1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let Some(sigma) = args.scalar_number(2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    (mean, sigma, 3)
  };
  if sigma <= 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
  let cumulative = evaluator.truthy(&args.value(cumulative_index)?);
  Some(evaluate_norm_dist_values(x, mean, sigma, cumulative))
}

fn evaluate_norm_dist_values<'doc>(
  x: f64,
  mean: f64,
  sigma: f64,
  cumulative: bool,
) -> FormulaValue<'doc> {
  if sigma <= 0.0 {
    return FormulaValue::Error(FormulaErrorValue::Num);
  }
  let z = (x - mean) / sigma;
  FormulaValue::Number(if cumulative {
    lo_integral_phi(z)
  } else {
    lo_phi(z) / sigma
  })
}

fn evaluate_norm_s_dist_legacy_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let value = args.value(0)?;
  if evaluator.array_context && is_matrix_argument(&value) {
    return map_numeric_array_values(evaluator, &[value], FormulaErrorValue::Value, |values| {
      FormulaValue::Number(lo_integral_phi(values[0]))
    });
  }
  if let FormulaValue::Error(error) = value {
    return Some(FormulaValue::Error(error));
  }
  let Some(x) = args.scalar_number(0) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  Some(FormulaValue::Number(lo_integral_phi(x)))
}

fn evaluate_poisson_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let arg_values = (0..args.len())
    .map(|index| args.value(index))
    .collect::<Option<Vec<_>>>()?;
  if evaluator.array_context && arg_values.iter().any(is_matrix_argument) {
    return map_numeric_array_values(evaluator, &arg_values, FormulaErrorValue::Value, |values| {
      evaluate_poisson_values(values[0], values[1], values[2] != 0.0, evaluator.grammar)
    });
  }
  let Some(x) = args.scalar_number(0) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let Some(lambda) = args.scalar_number(1) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let cumulative = evaluator.truthy(&args.value(2)?);
  Some(evaluate_poisson_values(
    x,
    lambda,
    cumulative,
    evaluator.grammar,
  ))
}

fn evaluate_poisson_values<'doc>(
  x: f64,
  lambda: f64,
  cumulative: bool,
  grammar: FormulaGrammar,
) -> FormulaValue<'doc> {
  if matches!(
    grammar,
    FormulaGrammar::OpenFormula | FormulaGrammar::CalcA1
  ) && (x < 0.0 || lambda <= 0.0)
  {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  if x == 0.0 && lambda == 0.0 {
    return FormulaValue::Number(1.0);
  }
  if x < 0.0 || lambda <= 0.0 {
    return FormulaValue::Error(FormulaErrorValue::Num);
  }
  FormulaValue::Number(lo_poisson_dist(x.floor(), lambda, cumulative))
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

fn evaluate_randbetween_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let Some(min) = evaluator.number(&args.value(0)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let Some(max) = evaluator.number(&args.value(1)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let min = min.ceil();
  let max = max.ceil();
  if min > max {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
  Some(FormulaValue::Number(min))
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

fn evaluate_ipmt_ppmt_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  interest: bool,
) -> Option<FormulaValue<'doc>> {
  let rate = finance_number_arg(evaluator, args, 0)?;
  let period = finance_number_arg(evaluator, args, 1)?;
  let nper = finance_number_arg(evaluator, args, 2)?;
  let pv = finance_number_arg(evaluator, args, 3)?;
  let fv = optional_number(evaluator, args, 4, 0.0)?;
  let pay_in_advance = optional_number(evaluator, args, 5, 0.0)? != 0.0;
  if period < 1.0 || period > nper || nper <= 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let (ipmt, pmt) = financial_ipmt(rate, period.floor(), nper, pv, fv, pay_in_advance);
  Some(FormulaValue::Number(if interest {
    ipmt
  } else {
    pmt - ipmt
  }))
}

fn evaluate_effect_nominal_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  effect: bool,
) -> Option<FormulaValue<'doc>> {
  let rate_arg = args.value(0)?;
  let periods_arg = args.value(1)?;
  if evaluator.array_context && (is_matrix_argument(&rate_arg) || is_matrix_argument(&periods_arg))
  {
    return evaluator.map_binary_values(rate_arg, periods_arg, |evaluator, rate, periods| {
      let Some(rate) = evaluator.number(rate) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      };
      let Some(periods) = evaluator.number(periods) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      };
      Some(effect_nominal_value(rate, periods, effect))
    });
  }
  let rate = finance_number_arg(evaluator, args, 0)?;
  let periods = finance_number_arg(evaluator, args, 1)?.floor();
  Some(effect_nominal_value(rate, periods, effect))
}

fn effect_nominal_value<'doc>(rate: f64, periods: f64, effect: bool) -> FormulaValue<'doc> {
  let periods = periods.floor();
  if rate <= 0.0 || periods < 1.0 {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  FormulaValue::Number(if effect {
    (1.0 + rate / periods).powf(periods) - 1.0
  } else {
    ((1.0 + rate).powf(1.0 / periods) - 1.0) * periods
  })
}

fn evaluate_sln_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let cost = finance_number_arg(evaluator, args, 0)?;
  let salvage = finance_number_arg(evaluator, args, 1)?;
  let life = finance_number_arg(evaluator, args, 2)?;
  if life == 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Div0));
  }
  Some(FormulaValue::Number((cost - salvage) / life))
}

fn evaluate_syd_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let cost = finance_number_arg(evaluator, args, 0)?;
  let salvage = finance_number_arg(evaluator, args, 1)?;
  let life = finance_number_arg(evaluator, args, 2)?;
  let period = finance_number_arg(evaluator, args, 3)?;
  if life <= 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  Some(FormulaValue::Number(
    (cost - salvage) * (life - period + 1.0) * 2.0 / (life * (life + 1.0)),
  ))
}

fn evaluate_ddb_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let cost = finance_number_arg(evaluator, args, 0)?;
  let salvage = finance_number_arg(evaluator, args, 1)?;
  let life = finance_number_arg(evaluator, args, 2)?;
  let period = finance_number_arg(evaluator, args, 3)?;
  if args.raw_arg(4).is_some() && args.is_missing(4) {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let factor = optional_number(evaluator, args, 4, 2.0)?;
  if cost <= salvage
    || cost < 0.0
    || salvage < 0.0
    || life <= 0.0
    || period <= 0.0
    || period > life
    || factor <= 0.0
  {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  Some(FormulaValue::Number(financial_ddb(
    cost, salvage, life, period, factor,
  )))
}

fn evaluate_db_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let cost = finance_number_arg(evaluator, args, 0)?;
  let salvage = finance_number_arg(evaluator, args, 1)?;
  let life = finance_number_arg(evaluator, args, 2)?;
  let period = finance_number_arg(evaluator, args, 3)?;
  let months = optional_number(evaluator, args, 4, 12.0)?;
  if cost <= 0.0 || salvage < 0.0 || life <= 0.0 || period <= 0.0 || months <= 0.0 || months > 12.0
  {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  Some(FormulaValue::Number(financial_db(
    cost, salvage, life, period, months,
  )))
}

fn evaluate_vdb_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  if args.is_missing(0) {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  }
  let cost = finance_number_arg(evaluator, args, 0)?;
  let salvage = finance_number_arg(evaluator, args, 1)?;
  let life = finance_number_arg(evaluator, args, 2)?;
  let start = finance_number_arg(evaluator, args, 3)?;
  let end = finance_number_arg(evaluator, args, 4)?;
  if args.raw_arg(5).is_some() && args.is_missing(5) {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let factor = optional_number(evaluator, args, 5, 2.0)?;
  let no_switch = args
    .raw_arg(6)
    .filter(|_| !args.is_missing(6))
    .and_then(|_| args.value(6))
    .is_some_and(|value| evaluator.truthy(&value));
  if cost < 0.0
    || salvage < 0.0
    || cost <= salvage
    || life <= 0.0
    || start < 0.0
    || end < start
    || end > life
    || factor <= 0.0
  {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  Some(FormulaValue::Number(financial_vdb(
    cost, salvage, life, start, end, factor, no_switch,
  )))
}

fn evaluate_xnpv_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let rate = finance_number_arg(evaluator, args, 0)?;
  let values = evaluator.value_numbers(&args.value(1)?);
  let dates = evaluator.value_numbers(&args.value(2)?);
  if values.len() < 2 || values.len() != dates.len() {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  financial_xnpv(rate, &values, &dates)
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(
      FormulaErrorValue::IllegalArgument,
    )))
}

fn evaluate_xirr_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let values = evaluator.value_numbers(&args.value(0)?);
  let dates = evaluator.value_numbers(&args.value(1)?);
  let guess = optional_number(evaluator, args, 2, 0.1)?;
  if values.len() < 2 || values.len() != dates.len() || guess <= -1.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  financial_xirr(&values, &dates, guess)
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(
      FormulaErrorValue::IllegalArgument,
    )))
}

#[derive(Clone, Copy)]
enum CouponKind {
  Daybs,
  Days,
  Daysnc,
  Ncd,
  Num,
  Pcd,
}

fn evaluate_coupon_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  kind: CouponKind,
) -> Option<FormulaValue<'doc>> {
  let settle = finance_date_arg(evaluator, args, 0)?;
  let maturity = finance_date_arg(evaluator, args, 1)?;
  let frequency = finance_i32_arg(evaluator, args, 2)?;
  let basis = optional_number(evaluator, args, 3, 0.0)?.floor() as i32;
  if settle >= maturity || !matches!(frequency, 1 | 2 | 4) || !(0..=4).contains(&basis) {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let value = match kind {
    CouponKind::Daybs => finance_coupdaybs(settle, maturity, frequency, basis)?,
    CouponKind::Days => finance_coupdays(settle, maturity, frequency, basis)?,
    CouponKind::Daysnc => finance_coupdaysnc(settle, maturity, frequency, basis)?,
    CouponKind::Ncd => f64::from(finance_coupncd(settle, maturity, frequency, basis)?),
    CouponKind::Num => finance_coupnum(settle, maturity, frequency, basis)?,
    CouponKind::Pcd => f64::from(finance_couppcd(settle, maturity, frequency, basis)?),
  };
  Some(FormulaValue::Number(value))
}

fn finance_number_arg<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
) -> Option<f64> {
  match scalar_number_arg_or_value(evaluator, args, index)? {
    Ok(value) => Some(value),
    Err(_) => None,
  }
}

fn finance_i32_arg<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
) -> Option<i32> {
  floor_to_i32(finance_number_arg(evaluator, args, index)?)
}

fn finance_date_arg<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
) -> Option<i32> {
  floor_to_i32(evaluator.date_number_from_value(&args.value(index)?)?)
}

fn financial_invalid_error(evaluator: &EvalContext<'_, '_>) -> FormulaErrorValue {
  match evaluator.grammar {
    FormulaGrammar::CalcA1 | FormulaGrammar::OpenFormula => FormulaErrorValue::IllegalArgument,
    FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1 => FormulaErrorValue::Num,
  }
}

fn financial_number_arg_strict<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
  missing_error: FormulaErrorValue,
) -> Result<f64, FormulaErrorValue> {
  if args.is_missing(index) {
    return Err(missing_error);
  }
  financial_number_arg(evaluator, args, index).and_then(|value| {
    if value.is_finite() {
      Ok(value)
    } else {
      Err(missing_error)
    }
  })
}

fn financial_i32_arg_strict<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
  missing_error: FormulaErrorValue,
) -> Result<i32, FormulaErrorValue> {
  let value = financial_number_arg_strict(evaluator, args, index, missing_error)?;
  floor_to_i32(value).ok_or(missing_error)
}

fn optional_financial_i32_arg_strict<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
  default: i32,
  missing_error: FormulaErrorValue,
) -> Result<i32, FormulaErrorValue> {
  match args.raw_arg(index).filter(|_| !args.is_missing(index)) {
    Some(_) => financial_i32_arg_strict(evaluator, args, index, missing_error),
    None => Ok(default),
  }
}

fn financial_date_arg_strict<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
  missing_error: FormulaErrorValue,
) -> Result<i32, FormulaErrorValue> {
  if args.is_missing(index) {
    return Err(missing_error);
  }
  let value = args.value(index).ok_or(missing_error)?;
  if let FormulaValue::Error(error) = value {
    return Err(error);
  }
  evaluator
    .date_number_from_value(&value)
    .and_then(floor_to_i32)
    .ok_or(FormulaErrorValue::Value)
}

#[derive(Clone, Copy)]
enum FinancialArrayArgKind {
  Date,
  Integer,
  Number,
}

fn financial_array_arg<'doc>(
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
  default: Option<FormulaValue<'doc>>,
) -> Option<FormulaValue<'doc>> {
  if args.raw_arg(index).is_none() || args.is_missing(index) {
    return default;
  }
  args.array_value(index)
}

fn map_financial_array_values<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  values: &[FormulaValue<'doc>],
  kinds: &[FinancialArrayArgKind],
  invalid_error: FormulaErrorValue,
  op: impl Fn(&[f64]) -> Option<f64> + Copy,
) -> Option<FormulaValue<'doc>> {
  let matrices = values
    .iter()
    .map(|value| evaluator.matrix_values(value))
    .collect::<Vec<_>>();
  let mut rows = 1usize;
  let mut columns = 1usize;
  let mut dimensions = Vec::with_capacity(matrices.len());
  for matrix in &matrices {
    let row_count = matrix.len();
    let column_count = matrix.first().map_or(0, Vec::len);
    if row_count == 0 || column_count == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    rows = rows.max(row_count);
    columns = columns.max(column_count);
    dimensions.push((row_count, column_count));
  }
  if dimensions.iter().any(|(row_count, column_count)| {
    !matrix_can_broadcast_local(*row_count, *column_count, rows, columns)
  }) {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }

  let mut result = Vec::with_capacity(rows);
  for row in 0..rows {
    let mut result_row = Vec::with_capacity(columns);
    for column in 0..columns {
      let mut converted = Vec::with_capacity(values.len());
      let mut error = None;
      for ((matrix, (row_count, column_count)), kind) in
        matrices.iter().zip(dimensions.iter().copied()).zip(kinds)
      {
        let value = &matrix[row.min(row_count - 1)][column.min(column_count - 1)];
        let number = match value {
          FormulaValue::Error(value_error) => {
            error = Some(*value_error);
            break;
          }
          value => match kind {
            FinancialArrayArgKind::Date => evaluator
              .date_number_from_value(value)
              .and_then(floor_to_i32)
              .map(f64::from),
            FinancialArrayArgKind::Integer => evaluator.number(value).map(|value| value.floor()),
            FinancialArrayArgKind::Number => evaluator.number(value),
          },
        };
        match number {
          Some(number) if number.is_finite() => converted.push(number),
          _ => {
            error = Some(invalid_error);
            break;
          }
        }
      }
      result_row.push(error.map_or_else(
        || match op(&converted) {
          Some(value) if value.is_finite() => FormulaValue::Number(value),
          _ => FormulaValue::Error(invalid_error),
        },
        FormulaValue::Error,
      ));
    }
    result.push(result_row);
  }
  if rows == 1 && columns == 1 {
    return result.into_iter().next()?.into_iter().next();
  }
  Some(FormulaValue::Matrix(result))
}

#[derive(Clone, Copy)]
enum DiscountSecurityKind {
  Discount,
  Intrate,
  Price,
  Received,
  Yield,
}

fn evaluate_discount_security_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  kind: DiscountSecurityKind,
) -> Option<FormulaValue<'doc>> {
  let invalid_error = financial_invalid_error(evaluator);
  if evaluator.array_context {
    let values = [
      financial_array_arg(args, 0, None)?,
      financial_array_arg(args, 1, None)?,
      financial_array_arg(args, 2, None)?,
      financial_array_arg(args, 3, None)?,
      financial_array_arg(args, 4, Some(FormulaValue::Number(0.0)))?,
    ];
    if values.iter().any(is_matrix_argument) {
      return map_financial_array_values(
        evaluator,
        &values,
        &[
          FinancialArrayArgKind::Date,
          FinancialArrayArgKind::Date,
          FinancialArrayArgKind::Number,
          FinancialArrayArgKind::Number,
          FinancialArrayArgKind::Integer,
        ],
        invalid_error,
        |values| {
          let settle = values[0] as i32;
          let maturity = values[1] as i32;
          let basis = values[4] as i32;
          if !(0..=4).contains(&basis) {
            return None;
          }
          match kind {
            DiscountSecurityKind::Discount => {
              finance_disc(settle, maturity, values[2], values[3], basis)
            }
            DiscountSecurityKind::Intrate => {
              finance_intrate(settle, maturity, values[2], values[3], basis)
            }
            DiscountSecurityKind::Price => {
              finance_pricedisc(settle, maturity, values[2], values[3], basis)
            }
            DiscountSecurityKind::Received => {
              finance_received(settle, maturity, values[2], values[3], basis)
            }
            DiscountSecurityKind::Yield => {
              finance_yielddisc(settle, maturity, values[2], values[3], basis)
            }
          }
        },
      );
    }
  }
  let settle = match financial_date_arg_strict(evaluator, args, 0, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let maturity = match financial_date_arg_strict(evaluator, args, 1, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let first = match financial_number_arg_strict(evaluator, args, 2, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let second = match financial_number_arg_strict(evaluator, args, 3, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let basis = match optional_financial_i32_arg_strict(evaluator, args, 4, 0, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  if !(0..=4).contains(&basis) {
    return Some(FormulaValue::Error(invalid_error));
  }
  let result = match kind {
    DiscountSecurityKind::Discount => finance_disc(settle, maturity, first, second, basis),
    DiscountSecurityKind::Intrate => finance_intrate(settle, maturity, first, second, basis),
    DiscountSecurityKind::Price => finance_pricedisc(settle, maturity, first, second, basis),
    DiscountSecurityKind::Received => finance_received(settle, maturity, first, second, basis),
    DiscountSecurityKind::Yield => finance_yielddisc(settle, maturity, first, second, basis),
  };
  result
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(invalid_error)))
}

fn evaluate_pricemat_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let invalid_error = financial_invalid_error(evaluator);
  if evaluator.array_context {
    let values = [
      financial_array_arg(args, 0, None)?,
      financial_array_arg(args, 1, None)?,
      financial_array_arg(args, 2, None)?,
      financial_array_arg(args, 3, None)?,
      financial_array_arg(args, 4, None)?,
      financial_array_arg(args, 5, Some(FormulaValue::Number(0.0)))?,
    ];
    if values.iter().any(is_matrix_argument) {
      return map_financial_array_values(
        evaluator,
        &values,
        &[
          FinancialArrayArgKind::Date,
          FinancialArrayArgKind::Date,
          FinancialArrayArgKind::Date,
          FinancialArrayArgKind::Number,
          FinancialArrayArgKind::Number,
          FinancialArrayArgKind::Integer,
        ],
        invalid_error,
        |values| {
          let basis = values[5] as i32;
          if !(0..=4).contains(&basis) {
            return None;
          }
          finance_pricemat(
            values[0] as i32,
            values[1] as i32,
            values[2] as i32,
            values[3],
            values[4],
            basis,
          )
        },
      );
    }
  }
  let settle = match financial_date_arg_strict(evaluator, args, 0, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let maturity = match financial_date_arg_strict(evaluator, args, 1, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let issue = match financial_date_arg_strict(evaluator, args, 2, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let rate = match financial_number_arg_strict(evaluator, args, 3, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let yield_value = match financial_number_arg_strict(evaluator, args, 4, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let basis = match optional_financial_i32_arg_strict(evaluator, args, 5, 0, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  if !(0..=4).contains(&basis) {
    return Some(FormulaValue::Error(invalid_error));
  }
  finance_pricemat(settle, maturity, issue, rate, yield_value, basis)
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(invalid_error)))
}

fn evaluate_accrint_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  maturity_only: bool,
) -> Option<FormulaValue<'doc>> {
  let invalid_error = financial_invalid_error(evaluator);
  if evaluator.array_context {
    let values = if maturity_only {
      [
        financial_array_arg(args, 0, None)?,
        financial_array_arg(args, 1, None)?,
        financial_array_arg(args, 2, None)?,
        financial_array_arg(args, 3, Some(FormulaValue::Number(1000.0)))?,
        financial_array_arg(args, 4, Some(FormulaValue::Number(0.0)))?,
      ]
      .to_vec()
    } else {
      [
        financial_array_arg(args, 0, None)?,
        financial_array_arg(args, 1, None)?,
        financial_array_arg(args, 2, None)?,
        financial_array_arg(args, 3, None)?,
        financial_array_arg(args, 4, Some(FormulaValue::Number(1000.0)))?,
        financial_array_arg(args, 5, None)?,
        financial_array_arg(args, 6, Some(FormulaValue::Number(0.0)))?,
      ]
      .to_vec()
    };
    if values.iter().any(is_matrix_argument) {
      let kinds = if maturity_only {
        vec![
          FinancialArrayArgKind::Date,
          FinancialArrayArgKind::Date,
          FinancialArrayArgKind::Number,
          FinancialArrayArgKind::Number,
          FinancialArrayArgKind::Integer,
        ]
      } else {
        vec![
          FinancialArrayArgKind::Date,
          FinancialArrayArgKind::Date,
          FinancialArrayArgKind::Date,
          FinancialArrayArgKind::Number,
          FinancialArrayArgKind::Number,
          FinancialArrayArgKind::Integer,
          FinancialArrayArgKind::Integer,
        ]
      };
      return map_financial_array_values(evaluator, &values, &kinds, invalid_error, |values| {
        if maturity_only {
          let basis = values[4] as i32;
          if !(0..=4).contains(&basis) {
            return None;
          }
          finance_accrintm(
            values[0] as i32,
            values[1] as i32,
            values[2],
            values[3],
            basis,
          )
        } else {
          let frequency = values[5] as i32;
          let basis = values[6] as i32;
          if !(0..=4).contains(&basis) {
            return None;
          }
          finance_accrint(
            values[0] as i32,
            values[2] as i32,
            values[3],
            values[4],
            frequency,
            basis,
          )
        }
      });
    }
  }
  let issue = match financial_date_arg_strict(evaluator, args, 0, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let settle_index = if maturity_only { 1 } else { 2 };
  if !maturity_only && let Err(error) = financial_date_arg_strict(evaluator, args, 1, invalid_error)
  {
    return Some(FormulaValue::Error(error));
  }
  let settle = match financial_date_arg_strict(evaluator, args, settle_index, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let rate = match financial_number_arg_strict(evaluator, args, settle_index + 1, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let par = match optional_financial_number_strict(evaluator, args, settle_index + 2, 1000.0) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let basis_index = if maturity_only { 4 } else { 6 };
  let basis =
    match optional_financial_i32_arg_strict(evaluator, args, basis_index, 0, invalid_error) {
      Ok(value) => value,
      Err(error) => return Some(FormulaValue::Error(error)),
    };
  if !(0..=4).contains(&basis) {
    return Some(FormulaValue::Error(invalid_error));
  }
  let result = if maturity_only {
    finance_accrintm(issue, settle, rate, par, basis)
  } else {
    let frequency = match financial_i32_arg_strict(evaluator, args, 5, invalid_error) {
      Ok(value) => value,
      Err(error) => return Some(FormulaValue::Error(error)),
    };
    finance_accrint(issue, settle, rate, par, frequency, basis)
  };
  result
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(invalid_error)))
}

#[derive(Clone, Copy)]
enum TbillKind {
  Eq,
  Price,
  Yield,
}

fn evaluate_tbill_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  kind: TbillKind,
) -> Option<FormulaValue<'doc>> {
  let invalid_error = financial_invalid_error(evaluator);
  if evaluator.array_context {
    let values = [
      financial_array_arg(args, 0, None)?,
      financial_array_arg(args, 1, None)?,
      financial_array_arg(args, 2, None)?,
    ];
    if values.iter().any(is_matrix_argument) {
      return map_financial_array_values(
        evaluator,
        &values,
        &[
          FinancialArrayArgKind::Date,
          FinancialArrayArgKind::Date,
          FinancialArrayArgKind::Number,
        ],
        invalid_error,
        |values| match kind {
          TbillKind::Eq => finance_tbilleq(values[0] as i32, values[1] as i32, values[2]),
          TbillKind::Price => finance_tbillprice(values[0] as i32, values[1] as i32, values[2]),
          TbillKind::Yield => finance_tbillyield(values[0] as i32, values[1] as i32, values[2]),
        },
      );
    }
  }
  let settle = match financial_date_arg_strict(evaluator, args, 0, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let maturity = match financial_date_arg_strict(evaluator, args, 1, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let value = match financial_number_arg_strict(evaluator, args, 2, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let result = match kind {
    TbillKind::Eq => finance_tbilleq(settle, maturity, value),
    TbillKind::Price => finance_tbillprice(settle, maturity, value),
    TbillKind::Yield => finance_tbillyield(settle, maturity, value),
  };
  result
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(invalid_error)))
}

fn evaluate_yield_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  if (0..6).any(|index| args.is_missing(index)) {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  let settle = evaluator.date_number_from_value(&args.value(0)?)?.floor() as i32;
  let maturity = evaluator.date_number_from_value(&args.value(1)?)?.floor() as i32;
  let coupon = evaluator.number(&args.value(2)?)?;
  let price = evaluator.number(&args.value(3)?)?;
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
    || coupon < 0.0
    || price <= 0.0
    || redemption <= 0.0
    || !matches!(frequency, 1 | 2 | 4)
    || !(0..=4).contains(&basis)
  {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  finance_yield(
    settle, maturity, coupon, price, redemption, frequency, basis,
  )
  .map(FormulaValue::Number)
  .or(Some(FormulaValue::Error(
    FormulaErrorValue::IllegalArgument,
  )))
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

fn evaluate_odd_period_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  price: bool,
) -> Option<FormulaValue<'doc>> {
  let invalid_error = financial_invalid_error(evaluator);
  if (0..7).any(|index| args.is_missing(index)) {
    return Some(FormulaValue::Error(invalid_error));
  }
  let settle = match financial_date_arg_strict(evaluator, args, 0, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let maturity = match financial_date_arg_strict(evaluator, args, 1, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let last_coupon = match financial_date_arg_strict(evaluator, args, 2, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let rate = match financial_number_arg_strict(evaluator, args, 3, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let value = match financial_number_arg_strict(evaluator, args, 4, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let redemption = match financial_number_arg_strict(evaluator, args, 5, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let frequency = match financial_i32_arg_strict(evaluator, args, 6, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let basis = match optional_financial_i32_arg_strict(evaluator, args, 7, 0, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  if rate <= 0.0
    || value < 0.0
    || (!price && value <= 0.0)
    || redemption <= 0.0
    || !matches!(frequency, 1 | 2 | 4)
    || maturity <= settle
    || settle <= last_coupon
    || !(0..=4).contains(&basis)
  {
    return Some(FormulaValue::Error(invalid_error));
  }
  let args = OddPeriodArgs {
    settle,
    maturity,
    last_coupon,
    rate,
    value,
    redemption,
    frequency,
    basis,
  };
  let result = if price {
    financial_oddlprice(args)
  } else {
    financial_oddlyield(args)
  };
  result
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(invalid_error)))
}

fn evaluate_amor_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  degressive: bool,
) -> Option<FormulaValue<'doc>> {
  let invalid_error = financial_invalid_error(evaluator);
  if (0..6).any(|index| args.is_missing(index)) {
    return Some(FormulaValue::Error(invalid_error));
  }
  let cost = match financial_number_arg_strict(evaluator, args, 0, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let date = match financial_date_arg_strict(evaluator, args, 1, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let first_period = match financial_date_arg_strict(evaluator, args, 2, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let residual = match financial_number_arg_strict(evaluator, args, 3, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let period = match financial_number_arg_strict(evaluator, args, 4, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let rate = match financial_number_arg_strict(evaluator, args, 5, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let basis = match optional_financial_i32_arg_strict(evaluator, args, 6, 0, invalid_error) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  if matches!(
    evaluator.grammar,
    FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1
  ) && basis == 2
  {
    return Some(FormulaValue::Error(invalid_error));
  }
  if date > first_period
    || rate <= 0.0
    || residual > cost
    || cost <= 0.0
    || residual < 0.0
    || period < 0.0
    || !(0..=4).contains(&basis)
  {
    return Some(FormulaValue::Error(invalid_error));
  }
  let result = if degressive {
    financial_amordegrc(cost, date, first_period, residual, period, rate, basis)
  } else {
    financial_amorlinc(cost, date, first_period, residual, period, rate, basis)
  };
  result
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(invalid_error)))
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
  let rate = match financial_number_arg(evaluator, args, 0) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let nper = match financial_number_arg(evaluator, args, 1) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let pv = match financial_number_arg(evaluator, args, 2) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let fv = match optional_financial_number_strict(evaluator, args, 3, 0.0) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let pay_in_advance = match optional_financial_type_strict(evaluator, args, 4) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
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
  let rate = match financial_number_arg(evaluator, args, 0) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let nper = match financial_number_arg(evaluator, args, 1) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let pmt = match financial_number_arg(evaluator, args, 2) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let fv = match optional_financial_number_strict(evaluator, args, 3, 0.0) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let pay_in_advance = match optional_financial_type_strict(evaluator, args, 4) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
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

fn evaluate_fv_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let rate = match financial_number_arg(evaluator, args, 0) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let nper = match financial_number_arg(evaluator, args, 1) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let pmt = match financial_number_arg(evaluator, args, 2) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let pv = match optional_financial_number_strict(evaluator, args, 3, 0.0) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let pay_in_advance = match optional_financial_type_strict(evaluator, args, 4) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let result = financial_fv(rate, nper, pmt, pv, pay_in_advance);
  if result.is_finite() {
    Some(FormulaValue::Number(result))
  } else {
    Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
  }
}

fn evaluate_nper_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let rate = match financial_number_arg(evaluator, args, 0) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let pmt = match financial_number_arg(evaluator, args, 1) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let pv = match financial_number_arg(evaluator, args, 2) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let fv = match optional_financial_number_strict(evaluator, args, 3, 0.0) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let pay_in_advance = match optional_financial_type_strict(evaluator, args, 4) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  financial_nper(rate, pmt, pv, fv, pay_in_advance)
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::Num)))
}

fn evaluate_rate_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let nper = match financial_number_arg(evaluator, args, 0) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  if nper <= 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
  let pmt = match financial_number_arg(evaluator, args, 1) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let pv = match financial_number_arg(evaluator, args, 2) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let fv = match optional_financial_number_strict(evaluator, args, 3, 0.0) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let pay_in_advance = match optional_financial_type_strict(evaluator, args, 4) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let (guess, default_guess) = match args.raw_arg(5).filter(|_| !args.is_missing(5)) {
    Some(_) => match financial_number_arg(evaluator, args, 5) {
      Ok(value) => (value, false),
      Err(error) => return Some(FormulaValue::Error(error)),
    },
    None => (0.1, true),
  };
  financial_rate(nper, pmt, pv, fv, pay_in_advance, guess, default_guess)
    .filter(|value| value.is_finite() && value.abs() <= 1.0e10)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::Num)))
}

fn evaluate_irr_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let values = evaluator.value_numbers(&args.value(0)?);
  let guess = optional_financial_number(evaluator, args, 1, 0.1)?;
  financial_irr(&values, guess)
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::Num)))
}

fn evaluate_mirr_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let values = evaluator.value_numbers(&args.value(0)?);
  let finance_rate = evaluator.number(&args.value(1)?)?;
  let reinvest_rate = evaluator.number(&args.value(2)?)?;
  financial_mirr(&values, finance_rate, reinvest_rate)
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::Div0)))
}

fn optional_financial_number<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
  default: f64,
) -> Option<f64> {
  match args.raw_arg(index).filter(|_| !args.is_missing(index)) {
    Some(_) => evaluator.number(&args.value(index)?),
    None => Some(default),
  }
}

fn financial_number_arg<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
) -> Result<f64, FormulaErrorValue> {
  let value = args.value(index).ok_or(FormulaErrorValue::Value)?;
  financial_number_operand(evaluator, value)
}

fn optional_financial_number_strict<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
  default: f64,
) -> Result<f64, FormulaErrorValue> {
  match args.raw_arg(index).filter(|_| !args.is_missing(index)) {
    Some(_) => financial_number_arg(evaluator, args, index),
    None => Ok(default),
  }
}

fn optional_financial_type_strict<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
) -> Result<bool, FormulaErrorValue> {
  match args.raw_arg(index).filter(|_| !args.is_missing(index)) {
    Some(_) => financial_number_arg(evaluator, args, index).map(|value| value != 0.0),
    None => Ok(false),
  }
}

fn financial_number_operand<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: FormulaValue<'doc>,
) -> Result<f64, FormulaErrorValue> {
  let value = evaluator.scalar_binary_operand(value);
  match value {
    FormulaValue::Error(error) => Err(error),
    value => match evaluator.number(&value) {
      Some(value) if value.is_finite() => Ok(value),
      Some(_) => Err(FormulaErrorValue::Num),
      None => Err(FormulaErrorValue::Value),
    },
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

fn evaluate_pduration_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let arg_values = (0..args.len())
    .map(|index| args.array_value(index))
    .collect::<Option<Vec<_>>>()?;
  if evaluator.array_context && arg_values.iter().any(is_matrix_argument) {
    return map_numeric_array_values(evaluator, &arg_values, FormulaErrorValue::Value, |values| {
      pduration_value(values[0], values[1], values[2])
    });
  }
  let rate = match scalar_number_arg_or_value(evaluator, args, 0)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let present = match scalar_number_arg_or_value(evaluator, args, 1)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let future = match scalar_number_arg_or_value(evaluator, args, 2)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  if rate <= 0.0 || present <= 0.0 || future <= 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
  }
  Some(pduration_value(rate, present, future))
}

fn pduration_value<'doc>(rate: f64, present: f64, future: f64) -> FormulaValue<'doc> {
  if rate <= 0.0 || present <= 0.0 || future <= 0.0 {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  FormulaValue::Number((future / present).ln() / rate.ln_1p())
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
  let left_value = args.value(0)?;
  if let FormulaValue::Error(error) = left_value {
    return Some(FormulaValue::Error(error));
  }
  let Some(left) = evaluator.number(&left_value) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let right = if args.len() == 2 {
    let right_value = args.value(1)?;
    if let FormulaValue::Error(error) = right_value {
      return Some(FormulaValue::Error(error));
    }
    let Some(right) = evaluator.number(&right_value) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    right
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
    return Some(FormulaValue::Error(FormulaErrorValue::NA));
  };
  covariance(&pairs, sample)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
}

fn evaluate_correlation_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let left_values = evaluator.matrix_values(&args.value(0)?);
  let right_values = evaluator.matrix_values(&args.value(1)?);
  let Some(pairs) = covariance_pairs(&left_values, &right_values) else {
    return Some(FormulaValue::Error(FormulaErrorValue::NA));
  };
  let (left, right): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
  correlation(&left, &right)
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::Div0)))
}

fn evaluate_slope_intercept_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  slope: bool,
) -> Option<FormulaValue<'doc>> {
  let y_values = evaluator.matrix_values(&args.value(0)?);
  let x_values = evaluator.matrix_values(&args.value(1)?);
  let Some(pairs) = covariance_pairs(&y_values, &x_values) else {
    return Some(FormulaValue::Error(FormulaErrorValue::NA));
  };
  let (y, x): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
  let state = regression_scalar_state(&y, &x);
  let value = if slope {
    state.slope()
  } else {
    state.intercept()
  };
  value
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::Div0)))
}

#[derive(Clone, Copy)]
enum RegressionScalarKind {
  Rsq,
  Steyx,
}

fn evaluate_regression_scalar_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  kind: RegressionScalarKind,
) -> Option<FormulaValue<'doc>> {
  let y_values = evaluator.matrix_values(&args.value(0)?);
  let x_values = evaluator.matrix_values(&args.value(1)?);
  let Some(pairs) = covariance_pairs(&y_values, &x_values) else {
    return Some(FormulaValue::Error(FormulaErrorValue::NA));
  };
  let (y, x): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
  let state = regression_scalar_state(&y, &x);
  let value = match kind {
    RegressionScalarKind::Rsq => state.r_squared(),
    RegressionScalarKind::Steyx => state.steyx(),
  };
  value
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(FormulaErrorValue::Div0)))
}

#[derive(Clone, Copy)]
enum RegressionArrayKind {
  Linest,
  Logest,
}

fn evaluate_regression_array_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  kind: RegressionArrayKind,
) -> Option<FormulaValue<'doc>> {
  let log_y = matches!(kind, RegressionArrayKind::Logest);
  let constant = optional_regression_bool(evaluator, args, 2, true)?;
  let stats = optional_regression_bool(evaluator, args, 3, false)?;
  let (y, design, _) = regression_known_data(evaluator, args, log_y)?;
  let model = regression_model(&y, &design, constant)?;
  let mut row = model.slopes.iter().rev().copied().collect::<Vec<_>>();
  row.push(model.intercept);
  let stats_rows = if stats {
    Some(regression_stats_rows(&y, &design, constant, &model)?)
  } else {
    None
  };
  if log_y {
    for value in &mut row {
      *value = value.exp();
    }
  }
  let mut result = vec![row.into_iter().map(FormulaValue::Number).collect()];
  if let Some(stats_rows) = stats_rows {
    result.extend(
      stats_rows
        .into_iter()
        .map(|row| row.into_iter().map(FormulaValue::Number).collect()),
    );
  }
  Some(FormulaValue::Matrix(result))
}

fn regression_stats_rows(
  y: &[f64],
  design: &[Vec<f64>],
  constant: bool,
  model: &crate::calc::regression::RegressionModel,
) -> Option<Vec<Vec<f64>>> {
  let feature_count = design.first().map_or(0, Vec::len);
  if feature_count != 1 || y.len() != design.len() {
    return None;
  }
  let x = design
    .iter()
    .map(|row| row.first().copied())
    .collect::<Option<Vec<_>>>()?;
  let count = y.len();
  let parameter_count = feature_count + usize::from(constant);
  if count <= parameter_count {
    return None;
  }
  let x_mean = kahan_sum(x.iter().copied()) / count as f64;
  let y_mean = kahan_sum(y.iter().copied()) / count as f64;
  let sxx = if constant {
    kahan_sum(x.iter().map(|value| {
      let delta = value - x_mean;
      delta * delta
    }))
  } else {
    kahan_sum(x.iter().map(|value| value * value))
  };
  if sxx == 0.0 {
    return None;
  }
  let residuals = y
    .iter()
    .zip(&x)
    .map(|(y, x)| y - model.predict(&[*x]))
    .collect::<Vec<_>>();
  let ss_res = kahan_sum(residuals.iter().map(|value| value * value));
  let df = (count - parameter_count) as f64;
  let mse = ss_res / df;
  let se_slope = (mse / sxx).sqrt();
  let se_intercept = if constant {
    (mse * (1.0 / count as f64 + x_mean * x_mean / sxx)).sqrt()
  } else {
    0.0
  };
  let ss_total = if constant {
    kahan_sum(y.iter().map(|value| {
      let delta = value - y_mean;
      delta * delta
    }))
  } else {
    kahan_sum(y.iter().map(|value| value * value))
  };
  let ss_reg = ss_total - ss_res;
  let r_squared = if ss_total == 0.0 {
    1.0
  } else {
    1.0 - ss_res / ss_total
  };
  let f_stat = if ss_res == 0.0 {
    f64::INFINITY
  } else {
    (ss_reg / feature_count as f64) / mse
  };
  Some(vec![
    vec![se_slope, se_intercept],
    vec![r_squared, mse.sqrt()],
    vec![f_stat, df],
    vec![ss_reg, ss_res],
  ])
}

#[derive(Clone, Copy)]
enum RegressionPredictionKind {
  Growth,
  Trend,
}

fn evaluate_regression_prediction_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  kind: RegressionPredictionKind,
) -> Option<FormulaValue<'doc>> {
  let growth = matches!(kind, RegressionPredictionKind::Growth);
  let constant = optional_regression_bool(evaluator, args, 3, true)?;
  let (y, design, known_shape) = regression_known_data(evaluator, args, growth)?;
  let (new_design, result_shape) = regression_new_design(evaluator, args, &design, known_shape)?;
  let model = regression_model(&y, &design, constant)?;
  let values = new_design
    .iter()
    .map(|row| {
      let value = model.predict(row);
      FormulaValue::Number(if growth { value.exp() } else { value })
    })
    .collect::<Vec<_>>();
  Some(regression_values_to_matrix(values, result_shape))
}

#[derive(Clone, Copy)]
enum RegressionKnownShape {
  Column,
  Row,
  Matrix,
}

fn regression_known_data<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  log_y: bool,
) -> Option<(Vec<f64>, Vec<Vec<f64>>, RegressionKnownShape)> {
  let y_matrix = numeric_matrix(evaluator, &args.value(0)?)?;
  let y_rows = y_matrix.len();
  let y_cols = y_matrix.first().map_or(0, Vec::len);
  if y_rows == 0 || y_cols == 0 {
    return None;
  }
  let shape = if y_rows == 1 {
    RegressionKnownShape::Row
  } else if y_cols == 1 {
    RegressionKnownShape::Column
  } else {
    RegressionKnownShape::Matrix
  };
  let mut y = y_matrix.iter().flatten().copied().collect::<Vec<_>>();
  if log_y {
    if y.iter().any(|value| *value <= 0.0) {
      return None;
    }
    for value in &mut y {
      *value = value.ln();
    }
  }
  let x_matrix = if args.raw_arg(1).is_some() && !args.is_missing(1) {
    Some(numeric_matrix(evaluator, &args.value(1)?)?)
  } else {
    None
  };
  let design = match x_matrix {
    None => (1..=y.len()).map(|index| vec![index as f64]).collect(),
    Some(x) => regression_design_from_x(&y_matrix, &x, shape)?,
  };
  (design.len() == y.len()).then_some((y, design, shape))
}

fn regression_design_from_x(
  y: &[Vec<f64>],
  x: &[Vec<f64>],
  shape: RegressionKnownShape,
) -> Option<Vec<Vec<f64>>> {
  let y_rows = y.len();
  let y_cols = y.first().map_or(0, Vec::len);
  let x_rows = x.len();
  let x_cols = x.first().map_or(0, Vec::len);
  if x_rows == 0 || x_cols == 0 || x.iter().any(|row| row.len() != x_cols) {
    return None;
  }
  if x_rows == y_rows && x_cols == y_cols {
    return Some(x.iter().flatten().map(|value| vec![*value]).collect());
  }
  match shape {
    RegressionKnownShape::Column if x_rows == y_rows => Some(x.to_vec()),
    RegressionKnownShape::Row if x_cols == y_cols => Some(
      (0..y_cols)
        .map(|column| x.iter().map(|row| row[column]).collect())
        .collect(),
    ),
    _ => None,
  }
}

fn regression_new_design<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  known_design: &[Vec<f64>],
  known_shape: RegressionKnownShape,
) -> Option<(Vec<Vec<f64>>, (usize, usize))> {
  if args.raw_arg(2).is_none() || args.is_missing(2) {
    let shape = match known_shape {
      RegressionKnownShape::Row => (1, known_design.len()),
      RegressionKnownShape::Column | RegressionKnownShape::Matrix => (known_design.len(), 1),
    };
    return Some((known_design.to_vec(), shape));
  }
  let matrix = numeric_matrix(evaluator, &args.value(2)?)?;
  let rows = matrix.len();
  let columns = matrix.first().map_or(0, Vec::len);
  let feature_count = known_design.first().map_or(0, Vec::len);
  let design = if feature_count == 1 {
    matrix
      .iter()
      .flatten()
      .map(|value| vec![*value])
      .collect::<Vec<_>>()
  } else if columns == feature_count {
    matrix.clone()
  } else if rows == feature_count {
    (0..columns)
      .map(|column| matrix.iter().map(|row| row[column]).collect())
      .collect::<Vec<_>>()
  } else {
    return None;
  };
  Some((design, (rows, columns)))
}

fn regression_values_to_matrix<'doc>(
  values: Vec<FormulaValue<'doc>>,
  shape: (usize, usize),
) -> FormulaValue<'doc> {
  let (rows, columns) = shape;
  if rows == 1 && columns == 1 {
    return values.into_iter().next().unwrap_or(FormulaValue::Blank);
  }
  let mut iter = values.into_iter();
  FormulaValue::Matrix(
    (0..rows)
      .map(|_| {
        (0..columns)
          .map(|_| {
            iter
              .next()
              .unwrap_or(FormulaValue::Error(FormulaErrorValue::NA))
          })
          .collect()
      })
      .collect(),
  )
}

fn optional_regression_bool<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  index: usize,
  default: bool,
) -> Option<bool> {
  match args.raw_arg(index).filter(|_| !args.is_missing(index)) {
    Some(_) => Some(evaluator.truthy(&args.value(index)?)),
    None => Some(default),
  }
}

fn numeric_matrix<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
) -> Option<Vec<Vec<f64>>> {
  let matrix = evaluator.matrix_values(value);
  if matrix.is_empty() {
    return None;
  }
  matrix
    .iter()
    .map(|row| row.iter().map(number_only).collect::<Option<Vec<_>>>())
    .collect()
}

fn evaluate_complex_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let Some(real) = evaluator.number(&args.value(0)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let Some(imaginary) = evaluator.number(&args.value(1)?) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let suffix = if args.len() == 3 && !args.is_missing(2) {
    match evaluator.text(&args.value(2)?).as_str() {
      "" | "i" => 'i',
      "j" => 'j',
      _ => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    }
  } else {
    'i'
  };
  Some(FormulaValue::String(Cow::Owned(format_complex_result(
    FormulaComplex::new(real, imaginary, suffix),
  ))))
}

#[derive(Clone, Copy)]
enum ComplexUnaryKind {
  Abs,
  Argument,
  Conjugate,
  Cos,
  Cosh,
  Cot,
  Csc,
  Csch,
  Exp,
  Imaginary,
  Ln,
  Log10,
  Log2,
  Real,
  Sec,
  Sech,
  Sin,
  Sinh,
  Sqrt,
  Tan,
}

fn evaluate_complex_unary_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  kind: ComplexUnaryKind,
) -> Option<FormulaValue<'doc>> {
  let value = args.value(0)?;
  if evaluator.array_context && is_matrix_argument(&value) {
    return evaluator.map_unary_values(value, |evaluator, value| {
      Some(evaluate_complex_unary_value(evaluator, value, kind))
    });
  }
  Some(evaluate_complex_unary_value(evaluator, &value, kind))
}

fn evaluate_complex_unary_value<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: &FormulaValue<'doc>,
  kind: ComplexUnaryKind,
) -> FormulaValue<'doc> {
  if let FormulaValue::Error(error) = value {
    return FormulaValue::Error(*error);
  }
  let Some(complex) = parse_complex_number(&evaluator.text(value)) else {
    return FormulaValue::Error(FormulaErrorValue::Value);
  };
  let value = complex.value();
  match kind {
    ComplexUnaryKind::Abs => finite_complex_number(value.norm()),
    ComplexUnaryKind::Argument => {
      if value.re == 0.0 && value.im == 0.0 {
        FormulaValue::Error(FormulaErrorValue::Div0)
      } else {
        finite_complex_number(value.arg())
      }
    }
    ComplexUnaryKind::Imaginary => finite_complex_number(value.im),
    ComplexUnaryKind::Real => finite_complex_number(value.re),
    ComplexUnaryKind::Conjugate => {
      complex_result_value(FormulaComplex::from_value(value.conj(), complex.suffix()))
    }
    ComplexUnaryKind::Cos => {
      complex_result_value(FormulaComplex::from_value(value.cos(), complex.suffix()))
    }
    ComplexUnaryKind::Cosh => {
      complex_result_value(FormulaComplex::from_value(value.cosh(), complex.suffix()))
    }
    ComplexUnaryKind::Cot => complex_inverse_result(value.tan(), complex.suffix()),
    ComplexUnaryKind::Csc => complex_inverse_result(value.sin(), complex.suffix()),
    ComplexUnaryKind::Csch => complex_inverse_result(value.sinh(), complex.suffix()),
    ComplexUnaryKind::Exp => {
      complex_result_value(FormulaComplex::from_value(value.exp(), complex.suffix()))
    }
    ComplexUnaryKind::Ln => {
      complex_result_value(FormulaComplex::from_value(value.ln(), complex.suffix()))
    }
    ComplexUnaryKind::Log10 => complex_result_value(FormulaComplex::from_value(
      value.ln() / 10.0_f64.ln(),
      complex.suffix(),
    )),
    ComplexUnaryKind::Log2 => complex_result_value(FormulaComplex::from_value(
      value.ln() / 2.0_f64.ln(),
      complex.suffix(),
    )),
    ComplexUnaryKind::Sec => complex_inverse_result(value.cos(), complex.suffix()),
    ComplexUnaryKind::Sech => complex_inverse_result(value.cosh(), complex.suffix()),
    ComplexUnaryKind::Sin => {
      complex_result_value(FormulaComplex::from_value(value.sin(), complex.suffix()))
    }
    ComplexUnaryKind::Sinh => {
      complex_result_value(FormulaComplex::from_value(value.sinh(), complex.suffix()))
    }
    ComplexUnaryKind::Sqrt => {
      complex_result_value(FormulaComplex::from_value(value.sqrt(), complex.suffix()))
    }
    ComplexUnaryKind::Tan => {
      complex_result_value(FormulaComplex::from_value(value.tan(), complex.suffix()))
    }
  }
}

fn evaluate_complex_div_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let left = match strict_complex_arg(evaluator, args.value(0)?) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let right = match strict_complex_arg(evaluator, args.value(1)?) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  if right.value().norm_sqr() == 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Div0));
  }
  let result = FormulaComplex::from_value(left.value() / right.value(), binary_suffix(left, right));
  Some(complex_result_value(result))
}

fn evaluate_complex_sub_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let left = match strict_complex_arg(evaluator, args.value(0)?) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let right = match strict_complex_arg(evaluator, args.value(1)?) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  Some(complex_result_value(FormulaComplex::from_value(
    left.value() - right.value(),
    binary_suffix(left, right),
  )))
}

fn evaluate_complex_power_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let complex_arg = args.value(0)?;
  let power_arg = args.value(1)?;
  if evaluator.array_context && (is_matrix_argument(&complex_arg) || is_matrix_argument(&power_arg))
  {
    return evaluator.map_binary_values(complex_arg, power_arg, |evaluator, complex, power| {
      Some(evaluate_complex_power_values(evaluator, complex, power))
    });
  }
  let complex = match strict_complex_arg(evaluator, args.value(0)?) {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  let power = match scalar_number_arg_or_value(evaluator, args, 1)? {
    Ok(value) => value,
    Err(error) => return Some(FormulaValue::Error(error)),
  };
  if complex.value().norm_sqr() == 0.0 && power <= 0.0 {
    return Some(FormulaValue::Error(FormulaErrorValue::Num));
  }
  Some(complex_power_result(complex, power))
}

fn evaluate_complex_power_values<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  complex: &FormulaValue<'doc>,
  power: &FormulaValue<'doc>,
) -> FormulaValue<'doc> {
  let complex = match strict_complex_arg(evaluator, complex.clone()) {
    Ok(value) => value,
    Err(error) => return FormulaValue::Error(error),
  };
  let Some(power) = evaluator.number(power) else {
    return FormulaValue::Error(FormulaErrorValue::Value);
  };
  if complex.value().norm_sqr() == 0.0 && power <= 0.0 {
    return FormulaValue::Error(FormulaErrorValue::Num);
  }
  complex_power_result(complex, power)
}

fn complex_power_result<'doc>(complex: FormulaComplex, power: f64) -> FormulaValue<'doc> {
  let value =
    if power.fract() == 0.0 && power >= f64::from(i32::MIN) && power <= f64::from(i32::MAX) {
      complex.value().powi(power as i32)
    } else {
      complex.value().powf(power)
    };
  complex_result_value(FormulaComplex::from_value(value, complex.suffix()))
}

fn evaluate_complex_aggregate_reader<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  args: FunctionArgReader<'_, '_, 'doc>,
  product: bool,
) -> Option<FormulaValue<'doc>> {
  let mut values = Vec::new();
  for index in 0..args.len() {
    if args.is_missing(index) {
      continue;
    }
    if let Err(error) = collect_complex_values(evaluator, args.value(index)?, &mut values) {
      return Some(FormulaValue::Error(error));
    }
  }
  if values.is_empty() {
    return Some(complex_result_value(FormulaComplex::new(0.0, 0.0, 'i')));
  }
  let mut suffix = values[0].suffix();
  let mut result = if product {
    Complex::new(1.0, 0.0)
  } else {
    Complex::new(0.0, 0.0)
  };
  for value in values {
    if suffix != 'j' && value.suffix() == 'j' {
      suffix = 'j';
    }
    result = if product {
      result * value.value()
    } else {
      result + value.value()
    };
  }
  Some(complex_result_value(FormulaComplex::from_value(
    result, suffix,
  )))
}

fn collect_complex_values<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: FormulaValue<'doc>,
  values: &mut Vec<FormulaComplex>,
) -> std::result::Result<(), FormulaErrorValue> {
  match value {
    FormulaValue::Error(error) => Err(error),
    FormulaValue::Blank => Ok(()),
    FormulaValue::String(ref text) if text.is_empty() => Ok(()),
    FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => {
      for value in evaluator.matrix_values(&value).into_iter().flatten() {
        collect_complex_values(evaluator, value, values)?;
      }
      Ok(())
    }
    value => {
      values.push(strict_complex_arg(evaluator, value)?);
      Ok(())
    }
  }
}

fn strict_complex_arg<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: FormulaValue<'doc>,
) -> std::result::Result<FormulaComplex, FormulaErrorValue> {
  let text = strict_text_arg(evaluator, value)?;
  parse_complex_number(&text).ok_or(FormulaErrorValue::Value)
}

fn complex_inverse_result<'doc>(value: Complex<f64>, suffix: char) -> FormulaValue<'doc> {
  if value.norm_sqr() == 0.0 {
    return FormulaValue::Error(FormulaErrorValue::Div0);
  }
  complex_result_value(FormulaComplex::from_value(
    Complex::new(1.0, 0.0) / value,
    suffix,
  ))
}

fn finite_complex_number<'doc>(value: f64) -> FormulaValue<'doc> {
  if value.is_finite() {
    FormulaValue::Number(value)
  } else {
    FormulaValue::Error(FormulaErrorValue::Num)
  }
}

fn complex_result_value<'doc>(value: FormulaComplex) -> FormulaValue<'doc> {
  let complex = value.value();
  if !complex.re.is_finite() || !complex.im.is_finite() {
    return FormulaValue::Error(FormulaErrorValue::Num);
  }
  let normalized = FormulaComplex::new(
    if complex.re.abs() <= 1.0e-14 {
      0.0
    } else {
      complex.re
    },
    if complex.im.abs() <= 1.0e-14 {
      0.0
    } else {
      complex.im
    },
    value.suffix(),
  );
  FormulaValue::String(Cow::Owned(format_complex_result(normalized)))
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
  let value = if value > 0.9 && value < 1.0 {
    value + 5.0e-16
  } else {
    value
  };
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
  if exponent_value < 0 {
    format!("{mantissa}e-{abs:02}", abs = exponent_value.abs())
  } else {
    format!("{mantissa}e+{exponent_value:02}")
  }
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
    let direct_scalar = !matches!(
      arg,
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
    );
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
        FormulaValue::String(_) if direct_scalar => {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
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

fn logical_scalar_boolean<'doc>(
  evaluator: &EvalContext<'_, 'doc>,
  value: FormulaValue<'doc>,
) -> Result<Option<bool>, FormulaErrorValue> {
  let value = evaluator.scalar_binary_operand(value);
  let value = match value {
    FormulaValue::Matrix(_) => evaluator.first_value(&value),
    value => value,
  };
  match value {
    FormulaValue::Blank => Ok(None),
    FormulaValue::Boolean(value) => Ok(Some(value)),
    FormulaValue::Number(value) if value.is_nan() => Err(FormulaErrorValue::Value),
    FormulaValue::Number(value) => Ok(Some(value != 0.0)),
    FormulaValue::String(value) if value.eq_ignore_ascii_case("TRUE") => Ok(Some(true)),
    FormulaValue::String(value) if value.eq_ignore_ascii_case("FALSE") => Ok(Some(false)),
    FormulaValue::String(_) => Err(FormulaErrorValue::Value),
    FormulaValue::Error(error) => Err(error),
    FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => {
      Err(FormulaErrorValue::Value)
    }
  }
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
      let Some(value) = evaluator.number(value) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      };
      Some(match op(value) {
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
  let Some(value) = evaluator.number(&value) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
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

fn raw_reference_area_count(args: FunctionArgReader<'_, '_, '_>, index: usize) -> Option<usize> {
  let arg = args.raw_arg(index)?;
  let ops = &arg.ops[arg.range.start..arg.range.end];
  let mut stack = Vec::new();
  for op in ops {
    match op {
      FormulaOp::PushReference(_) => stack.push(1usize),
      FormulaOp::Binary(FormulaOperator::Range | FormulaOperator::Intersection) => {
        stack.pop()?;
        stack.pop()?;
        stack.push(1);
      }
      FormulaOp::Binary(FormulaOperator::Union) => {
        let right = stack.pop()?;
        let left = stack.pop()?;
        stack.push(left + right);
      }
      _ => return None,
    }
  }
  match stack.as_slice() {
    [count] => Some(*count),
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
