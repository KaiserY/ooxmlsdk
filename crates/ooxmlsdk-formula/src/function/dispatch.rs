use std::borrow::Cow;

use num_complex::Complex;

use super::{
  EvalContext, FormulaFunctionId, FunctionArgReader, FunctionArgs, resolve_function_name,
};
use crate::calc::numeric::{
  CeilingFloorKind, approx_floor, even_odd, kahan_sum, round_to_decimal_places, sign_number,
};
use crate::calc::regression::EtsKind;
use crate::calc::special::{BesselKind, erf, erfc, lo_gauss, lo_phi, log_gamma};
use crate::calc::statistics::{PercentileKind, percentile_sorted, variance_slice};
use crate::calc::text::{
  legacy_char_text, legacy_text_code, proper_formula_text, rot13_formula_text, text_byte_len,
};
use crate::evaluator::{
  CouponFunction, DatabaseFunction, DatePart, TimePart, datevalue, rtl_cos, rtl_sin, rtl_tan,
  timevalue,
};
use crate::{FormulaErrorValue, FormulaValue};

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
  if matches!(args, FunctionArgs::Lazy(_))
    && let Some(value) = evaluate_function_reader(evaluator, function, args.reader(evaluator))
  {
    return Some(value);
  }
  let args = args.as_ast()?;
  match function {
    FormulaFunctionId::OrgDotOpenofficeDotErrortype => evaluator.evaluate_error_type_raw(args),
    FormulaFunctionId::ComDotMicrosoftDotCeiling => evaluator.evaluate_ceiling_excel_legacy(args),
    FormulaFunctionId::ComDotMicrosoftDotFloor => evaluator.evaluate_floor_excel_legacy(args),
    FormulaFunctionId::Let => evaluator.evaluate_let(args),
    FormulaFunctionId::If => evaluator.evaluate_if(args),
    FormulaFunctionId::Iferror => evaluator.evaluate_if_error(args, false),
    FormulaFunctionId::Ifna => evaluator.evaluate_if_error(args, true),
    FormulaFunctionId::Ifs => evaluator.evaluate_ifs_function(args),
    FormulaFunctionId::Switch => evaluator.evaluate_switch(args),
    FormulaFunctionId::Sum => match evaluator.numeric_aggregate(args, true) {
      Ok(aggregate) => Some(FormulaValue::Number(kahan_sum(aggregate.values))),
      Err(error) => Some(FormulaValue::Error(error)),
    },
    FormulaFunctionId::Product => match evaluator.numeric_aggregate(args, true) {
      Ok(aggregate) => {
        if aggregate.values.is_empty() {
          Some(FormulaValue::Number(0.0))
        } else {
          Some(FormulaValue::Number(aggregate.values.iter().product()))
        }
      }
      Err(error) => Some(FormulaValue::Error(error)),
    },
    FormulaFunctionId::Average => {
      let values = match evaluator.numeric_aggregate(args, true) {
        Ok(aggregate) => aggregate.values,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
      (!values.is_empty())
        .then(|| FormulaValue::Number(kahan_sum(values.iter().copied()) / values.len() as f64))
    }
    FormulaFunctionId::Count => Some(FormulaValue::Number(evaluator.count_numbers(args) as f64)),
    FormulaFunctionId::Counta => {
      Some(FormulaValue::Number(evaluator.count_all_values(args) as f64))
    }
    FormulaFunctionId::Iserror => evaluator.evaluate_information_error(args, |_| true),
    FormulaFunctionId::Isna => {
      evaluator.evaluate_information_error(args, |error| error == FormulaErrorValue::NA)
    }
    FormulaFunctionId::Iserr => {
      evaluator.evaluate_information_error(args, |error| error != FormulaErrorValue::NA)
    }
    FormulaFunctionId::Isblank => evaluator.evaluate_isblank(args),
    FormulaFunctionId::Istext => Some(FormulaValue::Boolean(matches!(
      args
        .first()
        .and_then(|arg| evaluator.evaluate(arg))
        .and_then(|value| evaluator.information_scalar_value(value)),
      Some(FormulaValue::String(_))
    ))),
    FormulaFunctionId::Isnontext => Some(FormulaValue::Boolean(!matches!(
      args
        .first()
        .and_then(|arg| evaluator.evaluate(arg))
        .and_then(|value| evaluator.information_scalar_value(value)),
      Some(FormulaValue::String(_))
    ))),
    FormulaFunctionId::Islogical => Some(FormulaValue::Boolean(matches!(
      args
        .first()
        .and_then(|arg| evaluator.evaluate(arg))
        .and_then(|value| evaluator.information_scalar_value(value)),
      Some(FormulaValue::Boolean(_))
    ))),
    FormulaFunctionId::Isnumber => evaluator.evaluate_isnumber(args),
    FormulaFunctionId::Isref => Some(FormulaValue::Boolean(matches!(
      args.first().and_then(|arg| evaluator.evaluate(arg)),
      Some(FormulaValue::Reference(_) | FormulaValue::RefList(_))
    ))),
    FormulaFunctionId::Isformula => evaluator.evaluate_is_formula(args),
    FormulaFunctionId::ErrorDotType => evaluator.evaluate_error_type(args),
    FormulaFunctionId::Errortype => evaluator.evaluate_error_type_raw(args),
    FormulaFunctionId::Type => evaluator.evaluate_type(args),
    FormulaFunctionId::Areas => evaluator.evaluate_areas(args),
    FormulaFunctionId::Min => Some(
      evaluator
        .numeric_aggregate(args, true)
        .map(|aggregate| {
          aggregate
            .values
            .iter()
            .copied()
            .reduce(f64::min)
            .map(FormulaValue::Number)
            .unwrap_or(FormulaValue::Number(0.0))
        })
        .unwrap_or_else(FormulaValue::Error),
    ),
    FormulaFunctionId::Mina => evaluator.evaluate_mina(args),
    FormulaFunctionId::Max => Some(
      evaluator
        .numeric_aggregate(args, true)
        .map(|aggregate| {
          aggregate
            .values
            .iter()
            .copied()
            .reduce(f64::max)
            .map(FormulaValue::Number)
            .unwrap_or(FormulaValue::Number(0.0))
        })
        .unwrap_or_else(FormulaValue::Error),
    ),
    FormulaFunctionId::Maxa => evaluator.evaluate_maxa(args),
    FormulaFunctionId::And => evaluator.evaluate_and(args),
    FormulaFunctionId::Or => evaluator.evaluate_or(args),
    FormulaFunctionId::Xor => evaluator.evaluate_xor(args),
    FormulaFunctionId::Not => evaluator.evaluate_not(args),
    FormulaFunctionId::True => Some(FormulaValue::Boolean(true)),
    FormulaFunctionId::False => Some(FormulaValue::Boolean(false)),
    FormulaFunctionId::Na => Some(FormulaValue::Error(FormulaErrorValue::NA)),
    FormulaFunctionId::Style => Some(FormulaValue::Number(0.0)),
    FormulaFunctionId::Current => evaluator
      .current_value
      .clone()
      .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
    FormulaFunctionId::N => evaluator.evaluate_n(args),
    FormulaFunctionId::Countblank => Some(FormulaValue::Number(
      evaluator.count_blank(&evaluator.evaluate(args.first()?)?) as f64,
    )),
    FormulaFunctionId::Abs => evaluator.evaluate_numeric_unary(args, f64::abs),
    FormulaFunctionId::Sign => evaluator
      .number_arg(args, 0)
      .map(|value| FormulaValue::Number(sign_number(value)))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
    FormulaFunctionId::Int => evaluator.evaluate_numeric_unary(args, approx_floor),
    FormulaFunctionId::Trunc => evaluator.evaluate_trunc(args),
    FormulaFunctionId::Mod => evaluator.evaluate_mod(args),
    FormulaFunctionId::Even => Some(FormulaValue::Number(even_odd(
      match evaluator.number_arg(args, 0) {
        Some(value) => value,
        None => return Some(FormulaValue::Error(FormulaErrorValue::Unknown)),
      },
      true,
    ))),
    FormulaFunctionId::Odd => Some(FormulaValue::Number(even_odd(
      match evaluator.number_arg(args, 0) {
        Some(value) => value,
        None => return Some(FormulaValue::Error(FormulaErrorValue::Unknown)),
      },
      false,
    ))),
    FormulaFunctionId::Rawsubtract => evaluator.evaluate_raw_subtract(args),
    FormulaFunctionId::Iseven => {
      let Some(value) = evaluator.number_arg(args, 0) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      Some(FormulaValue::Boolean(
        approx_floor(value.abs()) as i64 % 2 == 0,
      ))
    }
    FormulaFunctionId::Isodd => {
      let Some(value) = evaluator.number_arg(args, 0) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      Some(FormulaValue::Boolean(
        approx_floor(value.abs()) as i64 % 2 != 0,
      ))
    }
    FormulaFunctionId::Sqrt => {
      let value = evaluator.number(&evaluator.evaluate(args.first()?)?)?;
      if value < 0.0 {
        Some(FormulaValue::Error(FormulaErrorValue::Num))
      } else {
        Some(FormulaValue::Number(value.sqrt()))
      }
    }
    FormulaFunctionId::Power => {
      let value = evaluator.number(&evaluator.evaluate(args.first()?)?)?;
      let power = evaluator.number(&evaluator.evaluate(args.get(1)?)?)?;
      let result = value.powf(power);
      if result.is_finite() {
        Some(FormulaValue::Number(result))
      } else {
        Some(FormulaValue::Error(FormulaErrorValue::Num))
      }
    }
    FormulaFunctionId::Pi => {
      if args.is_empty() {
        Some(FormulaValue::Number(std::f64::consts::PI))
      } else {
        Some(FormulaValue::Error(FormulaErrorValue::Unknown))
      }
    }
    FormulaFunctionId::Sqrtpi => {
      let Some(value) = evaluator.number_arg(args, 0) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      if value < 0.0 {
        Some(FormulaValue::Error(FormulaErrorValue::Num))
      } else {
        Some(FormulaValue::Number((value * std::f64::consts::PI).sqrt()))
      }
    }
    FormulaFunctionId::Radians => args
      .first()
      .and_then(|arg| evaluator.evaluate(arg))
      .and_then(|value| evaluator.number(&value))
      .map(|value| FormulaValue::Number(value.to_radians()))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
    FormulaFunctionId::Degrees => args
      .first()
      .and_then(|arg| evaluator.evaluate(arg))
      .and_then(|value| evaluator.number(&value))
      .map(|value| FormulaValue::Number(value.to_degrees()))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
    FormulaFunctionId::Sin => evaluator.evaluate_numeric_unary(args, rtl_sin),
    FormulaFunctionId::Csc => evaluator.evaluate_numeric_unary(args, |value| 1.0 / rtl_sin(value)),
    FormulaFunctionId::Cos => evaluator.evaluate_numeric_unary(args, rtl_cos),
    FormulaFunctionId::Sec => evaluator.evaluate_numeric_unary(args, |value| 1.0 / rtl_cos(value)),
    FormulaFunctionId::Tan => evaluator.evaluate_numeric_unary(args, rtl_tan),
    FormulaFunctionId::Cot => evaluator.evaluate_numeric_unary(args, |value| 1.0 / rtl_tan(value)),
    FormulaFunctionId::Sinh => evaluator.evaluate_numeric_unary(args, f64::sinh),
    FormulaFunctionId::Csch => evaluator.evaluate_numeric_unary(args, |value| 1.0 / value.sinh()),
    FormulaFunctionId::Cosh => evaluator.evaluate_numeric_unary(args, f64::cosh),
    FormulaFunctionId::Sech => evaluator.evaluate_numeric_unary(args, |value| 1.0 / value.cosh()),
    FormulaFunctionId::Tanh => evaluator.evaluate_numeric_unary(args, f64::tanh),
    FormulaFunctionId::Coth => evaluator.evaluate_numeric_unary(args, |value| 1.0 / value.tanh()),
    FormulaFunctionId::Asin => evaluator.evaluate_numeric_unary(args, f64::asin),
    FormulaFunctionId::Asinh => evaluator.evaluate_numeric_unary(args, f64::asinh),
    FormulaFunctionId::Acos => evaluator.evaluate_numeric_unary(args, f64::acos),
    FormulaFunctionId::Acosh => evaluator.evaluate_numeric_unary_checked_error(
      args,
      |value| (value >= 1.0).then(|| value.acosh()),
      FormulaErrorValue::IllegalArgument,
    ),
    FormulaFunctionId::Acot => {
      evaluator.evaluate_numeric_unary(args, |value| std::f64::consts::FRAC_PI_2 - value.atan())
    }
    FormulaFunctionId::Atan => evaluator.evaluate_numeric_unary(args, f64::atan),
    FormulaFunctionId::Atanh => {
      let Some(value) = evaluator.number_arg(args, 0) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      if value.abs() >= 1.0 {
        Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
      } else {
        Some(FormulaValue::Number(value.atanh()))
      }
    }
    FormulaFunctionId::Acoth => {
      let Some(value) = evaluator.number_arg(args, 0) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      if value.abs() <= 1.0 {
        Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
      } else {
        Some(FormulaValue::Number((1.0 / value).atanh()))
      }
    }
    FormulaFunctionId::Atan2 => Some(FormulaValue::Number(
      evaluator
        .number(&evaluator.evaluate(args.get(1)?)?)?
        .atan2(evaluator.number(&evaluator.evaluate(args.first()?)?)?),
    )),
    FormulaFunctionId::Exp => evaluator
      .number_arg(args, 0)
      .map(|value| FormulaValue::Number(value.exp()))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
    FormulaFunctionId::Ln => {
      let Some(value) = evaluator.number_arg(args, 0) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      if value > 0.0 {
        Some(FormulaValue::Number(value.ln()))
      } else {
        Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
      }
    }
    FormulaFunctionId::Log10 => {
      let value = evaluator.number(&evaluator.evaluate(args.first()?)?)?;
      if value > 0.0 {
        Some(FormulaValue::Number(value.log10()))
      } else {
        Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
      }
    }
    FormulaFunctionId::Log => {
      let Some(value) = evaluator.number_arg(args, 0) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      let base = args
        .get(1)
        .and_then(|arg| evaluator.evaluate(arg))
        .and_then(|value| evaluator.number(&value))
        .unwrap_or(10.0);
      if value > 0.0 && base > 0.0 && base != 1.0 {
        Some(FormulaValue::Number(value.log(base)))
      } else {
        Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
      }
    }
    FormulaFunctionId::Sumsq => match evaluator.numeric_aggregate(args, true) {
      Ok(aggregate) => Some(FormulaValue::Number(kahan_sum(
        aggregate.values.into_iter().map(|value| value * value),
      ))),
      Err(error) => Some(FormulaValue::Error(error)),
    },
    FormulaFunctionId::Sumproduct => evaluator.evaluate_sumproduct(args),
    FormulaFunctionId::Round => {
      let value = evaluator.number(&evaluator.evaluate(args.first()?)?)?;
      let digits = args
        .get(1)
        .and_then(|arg| evaluator.evaluate(arg))
        .and_then(|value| evaluator.number(&value))
        .unwrap_or(0.0) as i32;
      Some(FormulaValue::Number(round_to_decimal_places(value, digits)))
    }
    FormulaFunctionId::Roundsig => evaluator.evaluate_roundsig(args),
    FormulaFunctionId::Roundup => evaluator.evaluate_round_direction(args, true),
    FormulaFunctionId::Rounddown => evaluator.evaluate_round_direction(args, false),
    FormulaFunctionId::Date => evaluator.evaluate_date(args),
    FormulaFunctionId::Datevalue => Some(datevalue(
      &evaluator.text(&evaluator.evaluate(args.first()?)?),
      evaluator.book.date_system,
    )),
    FormulaFunctionId::Time => evaluator.evaluate_time(args),
    FormulaFunctionId::Year => evaluator.evaluate_date_part(args, DatePart::Year),
    FormulaFunctionId::Month => evaluator.evaluate_date_part(args, DatePart::Month),
    FormulaFunctionId::Day => evaluator.evaluate_date_part(args, DatePart::Day),
    FormulaFunctionId::Weekday => evaluator.evaluate_weekday(args),
    FormulaFunctionId::Hour => evaluator.evaluate_time_part(args, TimePart::Hour),
    FormulaFunctionId::Minute => evaluator.evaluate_time_part(args, TimePart::Minute),
    FormulaFunctionId::Second => evaluator.evaluate_time_part(args, TimePart::Second),
    FormulaFunctionId::Days => match (
      args
        .first()
        .and_then(|arg| evaluator.evaluate(arg))
        .and_then(|value| evaluator.date_number_from_value(&value)),
      args
        .get(1)
        .and_then(|arg| evaluator.evaluate(arg))
        .and_then(|value| evaluator.date_number_from_value(&value)),
    ) {
      (Some(end), Some(start)) => Some(FormulaValue::Number(end - start)),
      _ => Some(FormulaValue::Error(FormulaErrorValue::Unknown)),
    },
    FormulaFunctionId::Days360 => evaluator.evaluate_days360(args),
    FormulaFunctionId::Today => evaluator.evaluate_today(),
    FormulaFunctionId::Eomonth => evaluator.evaluate_eomonth(args),
    FormulaFunctionId::Edate => evaluator.evaluate_edate(args),
    FormulaFunctionId::Isleapyear => evaluator.evaluate_is_leap_year(args),
    FormulaFunctionId::Trim => evaluator.evaluate_trim(args),
    FormulaFunctionId::Upper => Some(FormulaValue::String(Cow::Owned(
      evaluator
        .text(&evaluator.evaluate(args.first()?)?)
        .to_uppercase(),
    ))),
    FormulaFunctionId::Lower => Some(FormulaValue::String(Cow::Owned(
      evaluator
        .text(&evaluator.evaluate(args.first()?)?)
        .to_lowercase(),
    ))),
    FormulaFunctionId::Rot13 => Some(FormulaValue::String(Cow::Owned(rot13_formula_text(
      &evaluator.text(&evaluator.evaluate(args.first()?)?),
    )))),
    FormulaFunctionId::Proper => {
      let Some(value) = args.first().and_then(|arg| evaluator.evaluate(arg)) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      Some(FormulaValue::String(Cow::Owned(proper_formula_text(
        &evaluator.text(&value),
      ))))
    }
    FormulaFunctionId::Len => evaluator.evaluate_len(args, false),
    FormulaFunctionId::Lenb => evaluator.evaluate_len(args, true),
    FormulaFunctionId::T => evaluator.evaluate_t(args),
    FormulaFunctionId::Value => evaluator.evaluate_value(args),
    FormulaFunctionId::Numbervalue => evaluator.evaluate_numbervalue(args),
    FormulaFunctionId::Dollar => evaluator.evaluate_dollar(args),
    FormulaFunctionId::Fixed => evaluator.evaluate_fixed(args),
    FormulaFunctionId::Base => evaluator.evaluate_base(args),
    FormulaFunctionId::Decimal => evaluator.evaluate_decimal(args),
    FormulaFunctionId::Convert => evaluator.evaluate_convert(args),
    FormulaFunctionId::Bin2dec => evaluator.evaluate_base_to_decimal(args, 2),
    FormulaFunctionId::Oct2dec => evaluator.evaluate_base_to_decimal(args, 8),
    FormulaFunctionId::Hex2dec => evaluator.evaluate_base_to_decimal(args, 16),
    FormulaFunctionId::Bin2oct => {
      evaluator.evaluate_base_to_base(args, 2, 8, -536_870_912.0, 536_870_911.0)
    }
    FormulaFunctionId::Bin2hex => {
      evaluator.evaluate_base_to_base(args, 2, 16, -549_755_813_888.0, 549_755_813_887.0)
    }
    FormulaFunctionId::Oct2bin => evaluator.evaluate_base_to_base(args, 8, 2, -512.0, 511.0),
    FormulaFunctionId::Oct2hex => {
      evaluator.evaluate_base_to_base(args, 8, 16, -549_755_813_888.0, 549_755_813_887.0)
    }
    FormulaFunctionId::Hex2bin => evaluator.evaluate_base_to_base(args, 16, 2, -512.0, 511.0),
    FormulaFunctionId::Hex2oct => {
      evaluator.evaluate_base_to_base(args, 16, 8, -536_870_912.0, 536_870_911.0)
    }
    FormulaFunctionId::Dec2bin => evaluator.evaluate_decimal_to_base(args, 2, -512.0, 511.0),
    FormulaFunctionId::Dec2oct => {
      evaluator.evaluate_decimal_to_base(args, 8, -536_870_912.0, 536_870_911.0)
    }
    FormulaFunctionId::Dec2hex => {
      evaluator.evaluate_decimal_to_base(args, 16, -549_755_813_888.0, 549_755_813_887.0)
    }
    FormulaFunctionId::Bitand => evaluator.evaluate_bit_binary(args, |left, right| left & right),
    FormulaFunctionId::Bitor => evaluator.evaluate_bit_binary(args, |left, right| left | right),
    FormulaFunctionId::Bitxor => evaluator.evaluate_bit_binary(args, |left, right| left ^ right),
    FormulaFunctionId::Bitlshift => evaluator.evaluate_bit_shift(args, true),
    FormulaFunctionId::Bitrshift => evaluator.evaluate_bit_shift(args, false),
    FormulaFunctionId::Code => evaluator
      .text(&evaluator.evaluate(args.first()?)?)
      .chars()
      .next()
      .map(legacy_text_code)
      .map(|code| FormulaValue::Number(code as f64))
      .or(Some(FormulaValue::Number(0.0))),
    FormulaFunctionId::Unicode => evaluator
      .text(&evaluator.evaluate(args.first()?)?)
      .chars()
      .next()
      .map(|ch| FormulaValue::Number(ch as u32 as f64))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value))),
    FormulaFunctionId::Char => {
      let Some(code) = evaluator.number_arg(args, 0) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      legacy_char_text(code).map_or(
        Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
        |text| Some(FormulaValue::String(Cow::Owned(text))),
      )
    }
    FormulaFunctionId::Unichar => {
      // Source: LibreOffice sc/source/core/tool/interpr1.cxx ScInterpreter::ScUnichar.
      let Some(code) = evaluator.number(&evaluator.evaluate(args.first()?)?) else {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      };
      if code < 0.0 || code > u32::MAX as f64 {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      char::from_u32(code as u32)
        .map(|ch| FormulaValue::String(Cow::Owned(ch.to_string())))
        .or(Some(FormulaValue::Error(
          FormulaErrorValue::IllegalArgument,
        )))
    }
    FormulaFunctionId::Clean => evaluator.evaluate_clean(args),
    FormulaFunctionId::Choose => evaluator.evaluate_choose(args),
    FormulaFunctionId::Concat => Some(FormulaValue::String(Cow::Owned(
      evaluator
        .values(args)
        .map(|value| evaluator.text(&value))
        .collect(),
    ))),
    FormulaFunctionId::Exact => Some(FormulaValue::Boolean(
      evaluator.text(&evaluator.evaluate(args.first()?)?)
        == evaluator.text(&evaluator.evaluate(args.get(1)?)?),
    )),
    FormulaFunctionId::Find => evaluator.evaluate_find(args, true),
    FormulaFunctionId::Findb => evaluator.evaluate_findb(args, true),
    FormulaFunctionId::Search => evaluator.evaluate_find(args, false),
    FormulaFunctionId::Searchb => evaluator.evaluate_findb(args, false),
    FormulaFunctionId::Rept => {
      let text = evaluator.text(&evaluator.evaluate(args.first()?)?);
      let count = evaluator.number(&evaluator.evaluate(args.get(1)?)?)?;
      if count < 0.0 {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      Some(FormulaValue::String(Cow::Owned(
        text.repeat(count as usize),
      )))
    }
    FormulaFunctionId::Substitute => evaluator.evaluate_substitute(args),
    FormulaFunctionId::Replace => evaluator.evaluate_replace(args),
    FormulaFunctionId::Textbefore => evaluator.evaluate_text_before_after(args, false),
    FormulaFunctionId::Textafter => evaluator.evaluate_text_before_after(args, true),
    FormulaFunctionId::Textsplit => evaluator.evaluate_textsplit(args),
    FormulaFunctionId::Textjoin => evaluator.evaluate_textjoin(args),
    FormulaFunctionId::Replaceb => evaluator.evaluate_replaceb(args),
    FormulaFunctionId::Asc => evaluator.evaluate_width_conversion(args, false),
    FormulaFunctionId::Jis => evaluator.evaluate_width_conversion(args, true),
    FormulaFunctionId::Median => {
      let mut values = evaluator.median_numeric_args(args);
      percentile_sorted(&mut values, 0.5, PercentileKind::Inc)
        .map(FormulaValue::Number)
        .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown)))
    }
    FormulaFunctionId::Large => evaluator.evaluate_large_small(args, true),
    FormulaFunctionId::Small => evaluator.evaluate_large_small(args, false),
    FormulaFunctionId::Trimmean => evaluator.evaluate_trimmean(args),
    FormulaFunctionId::Devsq => evaluator.evaluate_devsq(args),
    FormulaFunctionId::Avedev => evaluator.evaluate_avedev(args),
    FormulaFunctionId::Averagea => evaluator.evaluate_averagea(args),
    FormulaFunctionId::StdevDotP => match evaluator.st_var_numbers(args, true) {
      Ok(values) => variance_slice(&values, false).map(|value| FormulaValue::Number(value.sqrt())),
      Err(error) => Some(FormulaValue::Error(error)),
    },
    FormulaFunctionId::StdevDotS => match evaluator.st_var_numbers(args, true) {
      Ok(values) => variance_slice(&values, true)
        .map(|value| FormulaValue::Number(value.sqrt()))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
      Err(error) => Some(FormulaValue::Error(error)),
    },
    FormulaFunctionId::Stdev => match evaluator.st_var_numbers(args, true) {
      Ok(values) => variance_slice(&values, true)
        .map(|value| FormulaValue::Number(value.sqrt()))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
      Err(error) => Some(FormulaValue::Error(error)),
    },
    FormulaFunctionId::Stdeva => evaluator.evaluate_stdeva(args, true),
    FormulaFunctionId::Stdevpa => evaluator.evaluate_stdeva(args, false),
    FormulaFunctionId::VarDotP => match evaluator.st_var_numbers(args, true) {
      Ok(values) => variance_slice(&values, false)
        .map(FormulaValue::Number)
        .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
      Err(error) => Some(FormulaValue::Error(error)),
    },
    FormulaFunctionId::VarDotS => variance_slice(&evaluator.numeric_args(args), true)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
    FormulaFunctionId::Vara => evaluator.evaluate_vara(args),
    FormulaFunctionId::Varpa => evaluator.evaluate_varpa(args),
    FormulaFunctionId::Skew => evaluator.evaluate_skew(args, false),
    FormulaFunctionId::SkewDotP => evaluator.evaluate_skew(args, true),
    FormulaFunctionId::Geomean => evaluator.evaluate_geo_har_mean(args, false),
    FormulaFunctionId::Harmean => evaluator.evaluate_geo_har_mean(args, true),
    FormulaFunctionId::Gauss => Some(FormulaValue::Number(lo_gauss(
      evaluator.number(&evaluator.evaluate(args.first()?)?)?,
    ))),
    FormulaFunctionId::Text => evaluator.evaluate_text(args),
    FormulaFunctionId::Timevalue => Some(timevalue(
      &evaluator.text(&evaluator.evaluate(args.first()?)?),
    )),
    FormulaFunctionId::Basisodatetime => evaluator.evaluate_basis_o_datetime(args),
    FormulaFunctionId::Datedif => evaluator.evaluate_datedif(args),
    FormulaFunctionId::Yearfrac => evaluator.evaluate_yearfrac(args),
    FormulaFunctionId::Weeknum => evaluator.evaluate_weeknum(args),
    FormulaFunctionId::Isoweeknum => evaluator.evaluate_iso_weeknum(args),
    FormulaFunctionId::Weeks => evaluator.evaluate_weeks(args),
    FormulaFunctionId::Weeksinyear => evaluator.evaluate_weeks_in_year(args),
    FormulaFunctionId::Years => evaluator.evaluate_years_months(args, true),
    FormulaFunctionId::Months => evaluator.evaluate_years_months(args, false),
    FormulaFunctionId::Daysinmonth => evaluator.evaluate_days_in_month_year(args, false),
    FormulaFunctionId::Daysinyear => evaluator.evaluate_days_in_month_year(args, true),
    FormulaFunctionId::Indirect => evaluator.evaluate_indirect(args),
    FormulaFunctionId::Index => evaluator.evaluate_index(args),
    FormulaFunctionId::Offset => evaluator.evaluate_offset(args),
    FormulaFunctionId::Lookup => evaluator.evaluate_lookup(args),
    FormulaFunctionId::Match => evaluator.evaluate_match(args),
    FormulaFunctionId::Xmatch => evaluator.evaluate_xmatch(args),
    FormulaFunctionId::Vlookup => evaluator.evaluate_vlookup(args),
    FormulaFunctionId::Hlookup => evaluator.evaluate_hlookup(args),
    FormulaFunctionId::Xlookup => evaluator.evaluate_xlookup(args),
    FormulaFunctionId::Sheets => evaluator.evaluate_sheets(args),
    FormulaFunctionId::Sheet => evaluator.evaluate_sheet(args),
    FormulaFunctionId::Formula => evaluator.evaluate_formula_text(args),
    FormulaFunctionId::Cell => evaluator.evaluate_cell(args),
    FormulaFunctionId::Address => evaluator.evaluate_address(args),
    FormulaFunctionId::Row => evaluator.evaluate_row_column(args, false),
    FormulaFunctionId::Column => evaluator.evaluate_row_column(args, true),
    FormulaFunctionId::Rows => evaluator.evaluate_rows_columns(args, false),
    FormulaFunctionId::Columns => evaluator.evaluate_rows_columns(args, true),
    FormulaFunctionId::Mid => evaluator.evaluate_mid(args),
    FormulaFunctionId::Midb => evaluator.evaluate_midb(args),
    FormulaFunctionId::Left => evaluator.evaluate_left(args),
    FormulaFunctionId::Leftb => evaluator.evaluate_leftb(args),
    FormulaFunctionId::Right => evaluator.evaluate_right(args),
    FormulaFunctionId::Rightb => evaluator.evaluate_rightb(args),
    FormulaFunctionId::Roman => evaluator.evaluate_roman(args),
    FormulaFunctionId::Arabic => evaluator.evaluate_arabic(args),
    FormulaFunctionId::Hyperlink => evaluator.evaluate_hyperlink(args),
    FormulaFunctionId::Tocol => evaluator.evaluate_to_row_column(args, false),
    FormulaFunctionId::Torow => evaluator.evaluate_to_row_column(args, true),
    FormulaFunctionId::Chooserows => evaluator.evaluate_choose_rows(args),
    FormulaFunctionId::Choosecols => evaluator.evaluate_choose_cols(args),
    FormulaFunctionId::Expand => evaluator.evaluate_expand(args),
    FormulaFunctionId::Hstack => evaluator.evaluate_stack(args, true),
    FormulaFunctionId::Vstack => evaluator.evaluate_stack(args, false),
    FormulaFunctionId::Wrapcols => evaluator.evaluate_wrap(args, true),
    FormulaFunctionId::Wraprows => evaluator.evaluate_wrap(args, false),
    FormulaFunctionId::Filter => evaluator.evaluate_filter(args),
    FormulaFunctionId::Unique => evaluator.evaluate_unique(args),
    FormulaFunctionId::Sequence => evaluator.evaluate_sequence(args),
    FormulaFunctionId::Transpose => evaluator.evaluate_transpose(args),
    FormulaFunctionId::Drop => evaluator.evaluate_take_drop(args, false),
    FormulaFunctionId::Take => evaluator.evaluate_take_drop(args, true),
    FormulaFunctionId::Sort => evaluator.evaluate_sort(args),
    FormulaFunctionId::Sortby => evaluator.evaluate_sortby(args),
    FormulaFunctionId::Mmult => evaluator.evaluate_mmult(args),
    FormulaFunctionId::Mdeterm => evaluator.evaluate_mdeterm(args),
    FormulaFunctionId::Minverse => evaluator.evaluate_minverse(args),
    FormulaFunctionId::Munit => evaluator.evaluate_munit(args),
    FormulaFunctionId::Gcd => evaluator.evaluate_gcd_lcm(args, false),
    FormulaFunctionId::Lcm => evaluator.evaluate_gcd_lcm(args, true),
    FormulaFunctionId::Fact => evaluator.evaluate_fact(args),
    FormulaFunctionId::Factdouble => evaluator.evaluate_fact_double(args),
    FormulaFunctionId::Multinomial => evaluator.evaluate_multinomial(args),
    FormulaFunctionId::Combin => evaluator.evaluate_combin(args, false),
    FormulaFunctionId::Combina => evaluator.evaluate_combin(args, true),
    FormulaFunctionId::Permut => evaluator.evaluate_permut(args),
    FormulaFunctionId::Permutationa => evaluator.evaluate_permutationa(args),
    FormulaFunctionId::Mround => evaluator.evaluate_mround(args),
    FormulaFunctionId::Quotient => evaluator.evaluate_quotient(args),
    FormulaFunctionId::Pmt => evaluator.evaluate_pmt(args),
    FormulaFunctionId::Fv => evaluator.evaluate_fv(args),
    FormulaFunctionId::Fvschedule => evaluator.evaluate_fvschedule(args),
    FormulaFunctionId::Npv => evaluator.evaluate_npv(args),
    FormulaFunctionId::Effect => evaluator.evaluate_effect(args),
    FormulaFunctionId::Rate => evaluator.evaluate_rate(args),
    FormulaFunctionId::Ispmt => evaluator.evaluate_ispmt(args),
    FormulaFunctionId::Ipmt => evaluator.evaluate_ipmt(args),
    FormulaFunctionId::Ppmt => evaluator.evaluate_ppmt(args),
    FormulaFunctionId::Cumipmt => evaluator.evaluate_cumipmt(args),
    FormulaFunctionId::Cumprinc => evaluator.evaluate_cumprinc(args),
    FormulaFunctionId::Ddb => evaluator.evaluate_ddb(args),
    FormulaFunctionId::Vdb => evaluator.evaluate_vdb(args),
    FormulaFunctionId::Sumifs => evaluator.evaluate_sumifs(args),
    FormulaFunctionId::Countifs => evaluator.evaluate_countifs(args),
    FormulaFunctionId::Maxifs => evaluator.evaluate_minmaxifs(args, true),
    FormulaFunctionId::Minifs => evaluator.evaluate_minmaxifs(args, false),
    FormulaFunctionId::Sumif => evaluator.evaluate_sumif(args),
    FormulaFunctionId::Countif => evaluator.evaluate_countif(args),
    FormulaFunctionId::Averageif => evaluator.evaluate_averageif(args),
    FormulaFunctionId::Averageifs => evaluator.evaluate_averageifs(args),
    FormulaFunctionId::Dsum => evaluator.evaluate_database_function(args, DatabaseFunction::Sum),
    FormulaFunctionId::Dcount => {
      evaluator.evaluate_database_function(args, DatabaseFunction::Count)
    }
    FormulaFunctionId::Dcounta => {
      evaluator.evaluate_database_function(args, DatabaseFunction::CountA)
    }
    FormulaFunctionId::Daverage => {
      evaluator.evaluate_database_function(args, DatabaseFunction::Average)
    }
    FormulaFunctionId::Dget => evaluator.evaluate_database_function(args, DatabaseFunction::Get),
    FormulaFunctionId::Dmax => evaluator.evaluate_database_function(args, DatabaseFunction::Max),
    FormulaFunctionId::Dmin => evaluator.evaluate_database_function(args, DatabaseFunction::Min),
    FormulaFunctionId::Dproduct => {
      evaluator.evaluate_database_function(args, DatabaseFunction::Product)
    }
    FormulaFunctionId::Dvar => evaluator.evaluate_database_function(args, DatabaseFunction::Var),
    FormulaFunctionId::Dvarp => evaluator.evaluate_database_function(args, DatabaseFunction::VarP),
    FormulaFunctionId::Dstdev => {
      evaluator.evaluate_database_function(args, DatabaseFunction::StdDev)
    }
    FormulaFunctionId::Dstdevp => {
      evaluator.evaluate_database_function(args, DatabaseFunction::StdDevP)
    }
    FormulaFunctionId::Ceiling => evaluator.evaluate_ceiling(args, CeilingFloorKind::Odff),
    FormulaFunctionId::CeilingDotMath => evaluator.evaluate_ceiling(args, CeilingFloorKind::Math),
    FormulaFunctionId::CeilingDotPrecise => {
      evaluator.evaluate_ceiling(args, CeilingFloorKind::Precise)
    }
    FormulaFunctionId::Floor => evaluator.evaluate_floor(args, CeilingFloorKind::Odff),
    FormulaFunctionId::FloorDotMath => evaluator.evaluate_floor(args, CeilingFloorKind::Math),
    FormulaFunctionId::FloorDotPrecise => evaluator.evaluate_floor(args, CeilingFloorKind::Precise),
    FormulaFunctionId::B => evaluator.evaluate_b(args),
    FormulaFunctionId::BetaDotDist => evaluator.evaluate_beta_dist(args, false),
    FormulaFunctionId::Betadist => evaluator.evaluate_beta_dist(args, true),
    FormulaFunctionId::BetaDotInv => evaluator.evaluate_beta_inv(args),
    FormulaFunctionId::BinomDotDist => evaluator.evaluate_binom_dist(args),
    FormulaFunctionId::BinomDotDistDotRange => evaluator.evaluate_binom_dist_range(args),
    FormulaFunctionId::BinomDotInv => evaluator.evaluate_binom_inv(args),
    FormulaFunctionId::ChisqDotDist => evaluator.evaluate_chisq_dist(args, false),
    FormulaFunctionId::ChisqDotDistDotRt => evaluator.evaluate_chisq_dist(args, true),
    FormulaFunctionId::Chisqdist => evaluator.evaluate_chisq_dist(args, false),
    FormulaFunctionId::ChisqDotInv => evaluator.evaluate_chisq_inv(args, false),
    FormulaFunctionId::Chisqinv => evaluator.evaluate_chisq_inv(args, false),
    FormulaFunctionId::ChisqDotInvDotRt => evaluator.evaluate_chisq_inv(args, true),
    FormulaFunctionId::ChisqDotTest => evaluator.evaluate_chisq_test(args),
    FormulaFunctionId::ConfidenceDotNorm => evaluator.evaluate_confidence_norm(args),
    FormulaFunctionId::ConfidenceDotT => evaluator.evaluate_confidence_t(args),
    FormulaFunctionId::CovarianceDotP => evaluator.evaluate_covariance(args, false),
    FormulaFunctionId::CovarianceDotS => evaluator.evaluate_covariance(args, true),
    FormulaFunctionId::Covar => evaluator.evaluate_covariance(args, false),
    FormulaFunctionId::Correl => evaluator.evaluate_correl(args),
    FormulaFunctionId::Pearson => evaluator.evaluate_correl(args),
    FormulaFunctionId::Phi => Some(FormulaValue::Number(lo_phi(
      evaluator.number(&evaluator.evaluate(args.first()?)?)?,
    ))),
    FormulaFunctionId::ErfDotPrecise => Some(FormulaValue::Number(erf(
      evaluator.number(&evaluator.evaluate(args.first()?)?)?,
    ))),
    FormulaFunctionId::Erf => evaluator.evaluate_erf(args),
    FormulaFunctionId::ErfcDotPrecise => evaluator.evaluate_numeric_unary(args, erfc),
    FormulaFunctionId::ExponDotDist => evaluator.evaluate_expon_dist(args),
    FormulaFunctionId::FDotDist => evaluator.evaluate_f_dist(args),
    FormulaFunctionId::FDotDistDotRt => evaluator.evaluate_f_dist_rt(args),
    FormulaFunctionId::FDotInv => evaluator.evaluate_f_inv(args, false),
    FormulaFunctionId::FDotInvDotRt => evaluator.evaluate_f_inv(args, true),
    FormulaFunctionId::FDotTest => evaluator.evaluate_f_test(args),
    FormulaFunctionId::GammaDotDist => evaluator.evaluate_gamma_dist(args),
    FormulaFunctionId::GammaDotInv => evaluator.evaluate_gamma_inv(args),
    FormulaFunctionId::Gamma => evaluator.evaluate_gamma(args),
    FormulaFunctionId::GammalnDotPrecise => evaluator
      .evaluate_numeric_unary_checked(args, |value| (value > 0.0).then(|| log_gamma(value))),
    FormulaFunctionId::HypgeomDotDist => evaluator.evaluate_hypgeom_dist(args),
    FormulaFunctionId::LognormDotDist => evaluator.evaluate_lognorm_dist(args),
    FormulaFunctionId::LognormDotInv => evaluator.evaluate_lognorm_inv(args),
    FormulaFunctionId::ModeDotMult => evaluator.evaluate_mode_ms(args, false),
    FormulaFunctionId::ModeDotSngl => evaluator.evaluate_mode_ms(args, true),
    FormulaFunctionId::Mode => evaluator.evaluate_mode(args),
    FormulaFunctionId::NegbinomDotDist => evaluator.evaluate_negbinom_dist(args, true),
    FormulaFunctionId::Negbinomdist => evaluator.evaluate_negbinom_dist(args, false),
    FormulaFunctionId::NormDotDist => evaluator.evaluate_norm_dist(args),
    FormulaFunctionId::NormDotInv => evaluator.evaluate_norm_inv(args),
    FormulaFunctionId::NormDotSDotDist => evaluator.evaluate_norm_s_dist(args),
    FormulaFunctionId::NormDotSDotInv => evaluator.evaluate_norm_s_inv(args),
    FormulaFunctionId::PercentileDotExc => evaluator.evaluate_percentile(args, PercentileKind::Exc),
    FormulaFunctionId::PercentileDotInc => evaluator.evaluate_percentile(args, PercentileKind::Inc),
    FormulaFunctionId::Percentrank => evaluator.evaluate_percent_rank(args, PercentileKind::Inc),
    FormulaFunctionId::PercentrankDotExc => {
      evaluator.evaluate_percent_rank(args, PercentileKind::Exc)
    }
    FormulaFunctionId::PoissonDotDist => evaluator.evaluate_poisson_dist(args, false),
    FormulaFunctionId::Poisson => evaluator.evaluate_poisson_dist(args, true),
    FormulaFunctionId::QuartileDotExc => evaluator.evaluate_quartile(args, PercentileKind::Exc),
    FormulaFunctionId::QuartileDotInc => evaluator.evaluate_quartile(args, PercentileKind::Inc),
    FormulaFunctionId::RankDotAvg => evaluator.evaluate_rank(args, true),
    FormulaFunctionId::RankDotEq => evaluator.evaluate_rank(args, false),
    FormulaFunctionId::Kurt => evaluator.evaluate_kurt(args),
    FormulaFunctionId::TDotDist => evaluator.evaluate_t_dist(args),
    FormulaFunctionId::TDotDistDot2t => evaluator.evaluate_t_dist_tails(args, 2),
    FormulaFunctionId::TDotDistDotRt => evaluator.evaluate_t_dist_tails(args, 1),
    FormulaFunctionId::Tdist => evaluator.evaluate_t_dist_legacy(args),
    FormulaFunctionId::TDotInv => evaluator.evaluate_t_inv(args, false),
    FormulaFunctionId::TDotInvDot2t => evaluator.evaluate_t_inv(args, true),
    FormulaFunctionId::TDotTest => evaluator.evaluate_t_test(args),
    FormulaFunctionId::Fisher => evaluator.evaluate_fisher(args, false),
    FormulaFunctionId::Fisherinv => evaluator.evaluate_fisher(args, true),
    FormulaFunctionId::Besseli => evaluator.evaluate_bessel(args, BesselKind::I),
    FormulaFunctionId::Besselj => evaluator.evaluate_bessel(args, BesselKind::J),
    FormulaFunctionId::Besselk => evaluator.evaluate_bessel(args, BesselKind::K),
    FormulaFunctionId::Bessely => evaluator.evaluate_bessel(args, BesselKind::Y),
    FormulaFunctionId::Fourier => evaluator.evaluate_fourier(args),
    FormulaFunctionId::Imreal => evaluator.evaluate_complex_part(args, false),
    FormulaFunctionId::Imaginary => evaluator.evaluate_complex_part(args, true),
    FormulaFunctionId::Imabs => evaluator.evaluate_complex_abs(args),
    FormulaFunctionId::Imargument => evaluator.evaluate_complex_argument(args),
    FormulaFunctionId::Imconjugate => evaluator.evaluate_complex_unary(args, |value| value.conj()),
    FormulaFunctionId::Imexp => evaluator.evaluate_complex_unary(args, |value| value.exp()),
    FormulaFunctionId::Imln => evaluator.evaluate_complex_unary(args, |value| value.ln()),
    FormulaFunctionId::Imlog10 => evaluator.evaluate_complex_unary(args, |value| value.log10()),
    FormulaFunctionId::Imlog2 => evaluator.evaluate_complex_unary(args, |value| value.log(2.0)),
    FormulaFunctionId::Imsin => evaluator.evaluate_complex_unary(args, |value| value.sin()),
    FormulaFunctionId::Imsinh => evaluator.evaluate_complex_unary(args, |value| value.sinh()),
    FormulaFunctionId::Imcosh => evaluator.evaluate_complex_unary(args, |value| value.cosh()),
    FormulaFunctionId::Imtan => evaluator.evaluate_complex_unary(args, |value| value.tan()),
    FormulaFunctionId::Imcot => {
      evaluator.evaluate_complex_unary(args, |value| Complex::new(1.0, 0.0) / value.tan())
    }
    FormulaFunctionId::Imcsc => {
      evaluator.evaluate_complex_unary(args, |value| Complex::new(1.0, 0.0) / value.sin())
    }
    FormulaFunctionId::Imsqrt => evaluator.evaluate_complex_unary(args, |value| value.sqrt()),
    FormulaFunctionId::Imcos => evaluator.evaluate_complex_unary(args, |value| value.cos()),
    FormulaFunctionId::Imsec => {
      evaluator.evaluate_complex_unary(args, |value| Complex::new(1.0, 0.0) / value.cos())
    }
    FormulaFunctionId::Imsech => {
      evaluator.evaluate_complex_unary(args, |value| Complex::new(1.0, 0.0) / value.cosh())
    }
    FormulaFunctionId::Imcsch => {
      evaluator.evaluate_complex_unary(args, |value| Complex::new(1.0, 0.0) / value.sinh())
    }
    FormulaFunctionId::Imsub => evaluator.evaluate_complex_binary(args, |left, right| left - right),
    FormulaFunctionId::Imdiv => evaluator.evaluate_complex_div(args),
    FormulaFunctionId::Impower => evaluator.evaluate_complex_power(args),
    FormulaFunctionId::Imsum => evaluator.evaluate_complex_sum_product(args, false),
    FormulaFunctionId::Improduct => evaluator.evaluate_complex_sum_product(args, true),
    FormulaFunctionId::Complex => evaluator.evaluate_complex(args),
    FormulaFunctionId::Delta => evaluator.evaluate_delta(args),
    FormulaFunctionId::Gestep => evaluator.evaluate_gestep(args),
    FormulaFunctionId::Slope => evaluator.evaluate_slope(args),
    FormulaFunctionId::Intercept => evaluator.evaluate_intercept(args),
    FormulaFunctionId::Forecast => evaluator.evaluate_forecast(args),
    FormulaFunctionId::ForecastDotEts => evaluator.evaluate_forecast_ets(args, EtsKind::Add),
    FormulaFunctionId::ForecastDotEtsDotMult => {
      evaluator.evaluate_forecast_ets(args, EtsKind::Mult)
    }
    FormulaFunctionId::ForecastDotEtsDotStat => {
      evaluator.evaluate_forecast_ets(args, EtsKind::StatAdd)
    }
    FormulaFunctionId::ForecastDotEtsDotStatDotMult => {
      evaluator.evaluate_forecast_ets(args, EtsKind::StatMult)
    }
    FormulaFunctionId::ForecastDotEtsDotSeasonality => {
      evaluator.evaluate_forecast_ets(args, EtsKind::Season)
    }
    FormulaFunctionId::Rsq => evaluator.evaluate_rsq(args),
    FormulaFunctionId::Steyx => evaluator.evaluate_steyx(args),
    FormulaFunctionId::Linest => evaluator.evaluate_linest(args, false),
    FormulaFunctionId::Logest => evaluator.evaluate_linest(args, true),
    FormulaFunctionId::Trend => evaluator.evaluate_trend_growth(args, false),
    FormulaFunctionId::Growth => evaluator.evaluate_trend_growth(args, true),
    FormulaFunctionId::WeibullDotDist => evaluator.evaluate_weibull_dist(args),
    FormulaFunctionId::NetworkdaysDotIntl => evaluator.evaluate_networkdays(args, true),
    FormulaFunctionId::Networkdays => evaluator.evaluate_networkdays(args, false),
    FormulaFunctionId::WorkdayDotIntl => evaluator.evaluate_workday(args, true),
    FormulaFunctionId::Workday => evaluator.evaluate_workday(args, false),
    FormulaFunctionId::ZDotTest => evaluator.evaluate_z_test(args),
    FormulaFunctionId::Standardize => evaluator.evaluate_standardize(args),
    FormulaFunctionId::Subtotal => evaluator.evaluate_subtotal(args),
    FormulaFunctionId::Aggregate => evaluator.evaluate_aggregate(args),
    FormulaFunctionId::Db => evaluator.evaluate_db(args),
    FormulaFunctionId::Price => evaluator.evaluate_price(args),
    FormulaFunctionId::Yield => evaluator.evaluate_yield(args),
    FormulaFunctionId::Pricedisc => evaluator.evaluate_pricedisc(args),
    FormulaFunctionId::Pricemat => evaluator.evaluate_pricemat(args),
    FormulaFunctionId::Yielddisc => evaluator.evaluate_yielddisc(args),
    FormulaFunctionId::Accrint => evaluator.evaluate_accrint(args, false),
    FormulaFunctionId::Accrintm => evaluator.evaluate_accrint(args, true),
    FormulaFunctionId::Oddlprice => evaluator.evaluate_oddlprice(args),
    FormulaFunctionId::Oddlyield => evaluator.evaluate_oddlyield(args),
    FormulaFunctionId::Amorlinc => evaluator.evaluate_amorlinc(args, false),
    FormulaFunctionId::Amordegrc => evaluator.evaluate_amorlinc(args, true),
    FormulaFunctionId::Disc => evaluator.evaluate_disc(args),
    FormulaFunctionId::Received => evaluator.evaluate_received(args),
    FormulaFunctionId::Intrate => evaluator.evaluate_intrate(args),
    FormulaFunctionId::Mduration => evaluator.evaluate_mduration(args),
    FormulaFunctionId::Coupdays => evaluator.evaluate_coupon(args, CouponFunction::Days),
    FormulaFunctionId::Coupdaybs => evaluator.evaluate_coupon(args, CouponFunction::DayBs),
    FormulaFunctionId::Coupdaysnc => evaluator.evaluate_coupon(args, CouponFunction::DaysNc),
    FormulaFunctionId::Coupncd => evaluator.evaluate_coupon(args, CouponFunction::Ncd),
    FormulaFunctionId::Coupnum => evaluator.evaluate_coupon(args, CouponFunction::Num),
    FormulaFunctionId::Couppcd => evaluator.evaluate_coupon(args, CouponFunction::Pcd),
    FormulaFunctionId::Tbilleq => evaluator.evaluate_tbilleq(args),
    FormulaFunctionId::Tbillprice => evaluator.evaluate_tbillprice(args),
    FormulaFunctionId::Tbillyield => evaluator.evaluate_tbillyield(args),
    FormulaFunctionId::Pv => evaluator.evaluate_pv(args),
    FormulaFunctionId::Nper => evaluator.evaluate_nper(args),
    FormulaFunctionId::Rri => evaluator.evaluate_rri(args),
    FormulaFunctionId::Pduration => evaluator.evaluate_pduration(args),
    FormulaFunctionId::Seriessum => evaluator.evaluate_seriessum(args),
    FormulaFunctionId::Eastersunday => evaluator.evaluate_eastersunday(args),
    FormulaFunctionId::Xirr => evaluator.evaluate_xirr(args),
    FormulaFunctionId::Xnpv => evaluator.evaluate_xnpv(args),
    FormulaFunctionId::Irr => evaluator.evaluate_irr(args),
    FormulaFunctionId::Mirr => evaluator.evaluate_mirr(args),
    FormulaFunctionId::Euroconvert => evaluator.evaluate_euroconvert(args),
    FormulaFunctionId::Bahttext => evaluator.evaluate_bahttext(args),
    FormulaFunctionId::Nominal => evaluator.evaluate_nominal(args),
    FormulaFunctionId::Sln => evaluator.evaluate_sln(args),
    FormulaFunctionId::Syd => evaluator.evaluate_syd(args),
    FormulaFunctionId::Prob => evaluator.evaluate_prob(args),
    FormulaFunctionId::Frequency => evaluator.evaluate_frequency(args),
    FormulaFunctionId::Sumx2my2 => evaluator.evaluate_sumx2(args, false),
    FormulaFunctionId::Sumx2py2 => evaluator.evaluate_sumx2(args, true),
    FormulaFunctionId::Sumxmy2 => evaluator.evaluate_sumxmy2(args),
    FormulaFunctionId::Dollarde => evaluator.evaluate_dollar_decimal(args, true),
    FormulaFunctionId::Dollarfr => evaluator.evaluate_dollar_decimal(args, false),
    FormulaFunctionId::Info => evaluator.evaluate_info(args),
    FormulaFunctionId::Regex => evaluator.evaluate_regex(args),
    FormulaFunctionId::Encodeurl => evaluator.evaluate_encodeurl(args),
    FormulaFunctionId::OrgDotOpenofficeDotRot13 => evaluator.evaluate_rot13(args),
    FormulaFunctionId::Getpivotdata => evaluator.evaluate_getpivotdata(args),
    FormulaFunctionId::Color => evaluator.evaluate_color(args),
    FormulaFunctionId::Dhfg => Some(FormulaValue::Error(FormulaErrorValue::Name)),
    FormulaFunctionId::Testfuncbool => Some(FormulaValue::Boolean(true)),
    FormulaFunctionId::Testfunccurr => Some(FormulaValue::Number(5.5)),
    FormulaFunctionId::Testfuncdate => Some(FormulaValue::Number(5590.0)),
    FormulaFunctionId::Randarray => evaluator.evaluate_randarray(args),
    FormulaFunctionId::Testfuncint => Some(FormulaValue::Number(6.0)),
    FormulaFunctionId::Testfuncsingle => Some(FormulaValue::Number(5.5)),
  }
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
    FormulaFunctionId::Count => Some(FormulaValue::Number(args.count_numbers()? as f64)),
    FormulaFunctionId::Counta => Some(FormulaValue::Number(args.count_all_values()? as f64)),
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
    FormulaFunctionId::ErrorDotType if args.len() == 1 => {
      evaluate_error_type_value(evaluator, &args.first_value()?)
    }
    FormulaFunctionId::Type if args.len() == 1 => {
      evaluate_type_value(evaluator, &args.first_value()?)
    }
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
    FormulaFunctionId::Abs if args.len() == 1 => Some(FormulaValue::Number(
      scalar_number_arg(evaluator, args, 0)?.abs(),
    )),
    FormulaFunctionId::Sign if args.len() == 1 => Some(FormulaValue::Number(sign_number(
      scalar_number_arg(evaluator, args, 0)?,
    ))),
    FormulaFunctionId::Int if args.len() == 1 => Some(FormulaValue::Number(approx_floor(
      scalar_number_arg(evaluator, args, 0)?,
    ))),
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
    FormulaFunctionId::Even if args.len() == 1 => Some(FormulaValue::Number(even_odd(
      scalar_number_arg(evaluator, args, 0)?,
      true,
    ))),
    FormulaFunctionId::Odd if args.len() == 1 => Some(FormulaValue::Number(even_odd(
      scalar_number_arg(evaluator, args, 0)?,
      false,
    ))),
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
    FormulaFunctionId::Not if args.len() == 1 => {
      let value = args.scalar_value(0)?;
      match value {
        FormulaValue::Error(error) => Some(FormulaValue::Error(error)),
        value => Some(FormulaValue::Boolean(!evaluator.truthy(&value))),
      }
    }
    FormulaFunctionId::True if args.is_empty() => Some(FormulaValue::Boolean(true)),
    FormulaFunctionId::False if args.is_empty() => Some(FormulaValue::Boolean(false)),
    FormulaFunctionId::Na if args.is_empty() => Some(FormulaValue::Error(FormulaErrorValue::NA)),
    FormulaFunctionId::Pi if args.is_empty() => Some(FormulaValue::Number(std::f64::consts::PI)),
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
    && matches!(
      value,
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
    )
  {
    return None;
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
    return None;
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
    FormulaValue::Boolean(value) => {
      if value {
        1.0
      } else {
        0.0
      }
    }
    FormulaValue::Error(error) => return Some(FormulaValue::Error(error)),
    _ => 0.0,
  }))
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
  let value = scalar_number_arg(evaluator, args, 0)?;
  Some(match op(value) {
    Some(result) if result.is_finite() => FormulaValue::Number(result),
    Some(_) => FormulaValue::Error(FormulaErrorValue::Value),
    None => FormulaValue::Error(error),
  })
}
