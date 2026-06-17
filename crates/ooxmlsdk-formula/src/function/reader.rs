use super::FunctionArgs;
use crate::code::FormulaOp;
use crate::evaluator::{EvalArg, FormulaEvaluator, QueryValueSource, evaluate_arg_direct};
use crate::{FormulaErrorValue, FormulaValue, QualifiedRange};

#[derive(Clone, Copy)]
pub(crate) struct FunctionArgReader<'args, 'eval, 'doc> {
  args: FunctionArgs<'args, 'doc>,
  evaluator: &'eval FormulaEvaluator<'eval, 'doc>,
}

impl<'args, 'eval, 'doc> FunctionArgReader<'args, 'eval, 'doc> {
  pub(crate) fn new(
    args: FunctionArgs<'args, 'doc>,
    evaluator: &'eval FormulaEvaluator<'eval, 'doc>,
  ) -> Self {
    Self { args, evaluator }
  }

  pub(crate) fn len(self) -> usize {
    self.args.len()
  }

  pub(crate) fn is_empty(self) -> bool {
    self.len() == 0
  }

  pub(crate) fn raw_arg(self, index: usize) -> Option<EvalArg<'args, 'doc>> {
    self.args.get(index)
  }

  pub(crate) fn is_missing(self, index: usize) -> bool {
    let Some(arg) = self.raw_arg(index) else {
      return false;
    };
    let range = arg.range;
    range.end == range.start + 1 && matches!(arg.ops.get(range.start), Some(FormulaOp::PushBlank))
  }

  pub(crate) fn is_array_literal(self, index: usize) -> bool {
    let Some(arg) = self.raw_arg(index) else {
      return false;
    };
    arg
      .range
      .end
      .checked_sub(1)
      .and_then(|index| arg.ops.get(index))
      .is_some_and(|op| matches!(op, FormulaOp::Array { .. }))
  }

  pub(crate) fn value(self, index: usize) -> Option<FormulaValue<'doc>> {
    evaluate_arg_direct(self.args.get(index)?, self.evaluator)
  }

  pub(crate) fn array_value(self, index: usize) -> Option<FormulaValue<'doc>> {
    let evaluator = self.evaluator.with_array_context();
    evaluate_arg_direct(self.args.get(index)?, &evaluator)
  }

  pub(crate) fn first_value(self) -> Option<FormulaValue<'doc>> {
    self.value(0)
  }

  pub(crate) fn reference_ranges(self, index: usize) -> Option<Vec<QualifiedRange<'doc>>> {
    let value = self.value(index)?;
    Some(self.evaluator.reference_ranges_from_value(&value))
  }

  pub(crate) fn query_source(self, index: usize) -> Option<QueryValueSource<'doc>> {
    self.evaluator.query_source_from_value(self.value(index)?)
  }

  pub(crate) fn value_numbers(self, index: usize) -> Option<Vec<f64>> {
    let value = self.value(index)?;
    Some(self.evaluator.value_numbers(&value))
  }

  pub(crate) fn scalar_value(self, index: usize) -> Option<FormulaValue<'doc>> {
    let value = self.value(index)?;
    if should_fallback_scalar_value(self.evaluator, &value) {
      return None;
    }
    Some(self.evaluator.scalar_value(value))
  }

  pub(crate) fn scalar_number(self, index: usize) -> Option<f64> {
    let value = self.value(index)?;
    if should_fallback_scalar_value(self.evaluator, &value) {
      return None;
    }
    self.evaluator.number(&value)
  }

  pub(crate) fn numeric_aggregate(
    self,
    text_error: bool,
  ) -> Option<std::result::Result<Vec<f64>, FormulaErrorValue>> {
    let mut values = Vec::new();
    for index in 0..self.len() {
      let arg = self.value(index)?;
      match arg {
        FormulaValue::Reference(ref reference) => {
          if let Err(error) = self
            .evaluator
            .push_range_numeric_aggregate_values(reference, &mut values)
          {
            return Some(Err(error));
          }
        }
        FormulaValue::RefList(ref ranges) => {
          for range in ranges {
            if let Err(error) = self
              .evaluator
              .push_range_numeric_aggregate_values(range, &mut values)
            {
              return Some(Err(error));
            }
          }
        }
        FormulaValue::Matrix(ref rows) => {
          for value in rows.iter().flatten() {
            match value {
              FormulaValue::Blank | FormulaValue::String(_) => {}
              value => {
                if let Err(error) = self.evaluator.push_direct_numeric_aggregate_value(
                  value.clone(),
                  text_error,
                  &mut values,
                ) {
                  return Some(Err(error));
                }
              }
            }
          }
        }
        ref value => {
          if let Err(error) = self.evaluator.push_direct_numeric_aggregate_value(
            value.clone(),
            text_error,
            &mut values,
          ) {
            return Some(Err(error));
          }
        }
      }
    }
    Some(Ok(values))
  }

  pub(crate) fn count_numbers(self) -> Option<usize> {
    let mut count = 0usize;
    for index in 0..self.len() {
      let arg = self.value(index)?;
      match arg {
        FormulaValue::Reference(ref reference) => {
          count += self.evaluator.count_numbers_in_range(reference);
        }
        FormulaValue::RefList(ref ranges) => {
          for range in ranges {
            count += self.evaluator.count_numbers_in_range(range);
          }
        }
        FormulaValue::Matrix(ref rows) => {
          count += rows
            .iter()
            .flatten()
            .filter(|value| matches!(value, FormulaValue::Number(_) | FormulaValue::Boolean(_)))
            .count();
        }
        FormulaValue::Number(_) | FormulaValue::Boolean(_) => count += 1,
        FormulaValue::String(ref value) if value.trim().parse::<f64>().is_ok() => count += 1,
        FormulaValue::String(_) | FormulaValue::Blank | FormulaValue::Error(_) => {}
      }
    }
    Some(count)
  }

  pub(crate) fn count_all_values(self) -> Option<usize> {
    let mut count = 0usize;
    for index in 0..self.len() {
      let arg = self.value(index)?;
      match arg {
        FormulaValue::Reference(ref reference) => {
          count += self.evaluator.count_all_values_in_range(reference);
        }
        FormulaValue::RefList(ref ranges) => {
          for range in ranges {
            count += self.evaluator.count_all_values_in_range(range);
          }
        }
        FormulaValue::Matrix(ref rows) => {
          count += rows
            .iter()
            .flatten()
            .filter(|value| !matches!(value, FormulaValue::Blank))
            .count();
        }
        FormulaValue::Blank => count += 1,
        _ => count += 1,
      }
    }
    Some(count)
  }
}

fn should_fallback_scalar_value<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  value: &FormulaValue<'doc>,
) -> bool {
  matches!(value, FormulaValue::Matrix(_) | FormulaValue::RefList(_))
    || matches!(value, FormulaValue::Reference(reference) if reference.range.cell_count_hint() != 1)
    || (evaluator.array_context && matches!(value, FormulaValue::Reference(_)))
}
