use super::*;

impl<'a, 'doc> FormulaEvaluator<'a, 'doc> {
  pub(crate) fn evaluate_unary_value(
    &self,
    op: FormulaOperator,
    value: FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    if self.array_context
      && op != FormulaOperator::ImplicitIntersection
      && matches!(
        value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      )
    {
      return self.map_unary_values(value, |evaluator, value| {
        evaluator.evaluate_unary_value(op, value.clone())
      });
    }
    if let FormulaValue::Error(error) = value {
      return Some(FormulaValue::Error(error));
    }
    if matches!(value, FormulaValue::Reference(_) | FormulaValue::RefList(_)) {
      return self.evaluate_unary_reference_value(op, value);
    }
    match op {
      FormulaOperator::ImplicitIntersection => Some(self.implicit_intersection_scalar_value(value)),
      FormulaOperator::UnaryPlus => Some(FormulaValue::Number(self.number(&value)?)),
      FormulaOperator::UnaryMinus => match value {
        FormulaValue::String(_) => self
          .number(&value)
          .map(|number| FormulaValue::Number(-number))
          .or(Some(FormulaValue::Error(FormulaErrorValue::Value))),
        value => Some(FormulaValue::Number(-self.number(&value)?)),
      },
      FormulaOperator::Percent => Some(FormulaValue::Number(self.number(&value)? / 100.0)),
      _ => None,
    }
  }

  fn evaluate_unary_reference_value(
    &self,
    op: FormulaOperator,
    value: FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let value = match value {
      FormulaValue::Reference(reference) => match self.implicit_intersection_value(&reference) {
        Some(value) => value,
        None if matches!(op, FormulaOperator::UnaryPlus) => FormulaValue::Reference(reference),
        None => FormulaValue::Error(FormulaErrorValue::Value),
      },
      FormulaValue::RefList(ranges) if ranges.len() == 1 => self
        .implicit_intersection_value(&ranges[0])
        .unwrap_or_else(|| {
          if matches!(op, FormulaOperator::UnaryPlus) {
            FormulaValue::RefList(ranges)
          } else {
            FormulaValue::Error(FormulaErrorValue::Value)
          }
        }),
      FormulaValue::RefList(_) => FormulaValue::Error(FormulaErrorValue::Value),
      value => value,
    };
    match op {
      FormulaOperator::ImplicitIntersection => Some(value),
      FormulaOperator::UnaryPlus => match value {
        FormulaValue::Error(error) => Some(FormulaValue::Error(error)),
        FormulaValue::Reference(_) | FormulaValue::RefList(_) => Some(value),
        FormulaValue::Blank => Some(FormulaValue::Number(0.0)),
        FormulaValue::Number(_) | FormulaValue::String(_) => Some(value),
        FormulaValue::Boolean(_) => Some(FormulaValue::Number(self.number(&value)?)),
        _ => None,
      },
      FormulaOperator::UnaryMinus => match value {
        FormulaValue::Error(error) => Some(FormulaValue::Error(error)),
        FormulaValue::String(_) => self
          .number(&value)
          .map(|number| FormulaValue::Number(-number))
          .or(Some(FormulaValue::Error(FormulaErrorValue::Value))),
        value => Some(FormulaValue::Number(-self.number(&value)?)),
      },
      FormulaOperator::Percent => Some(FormulaValue::Number(self.number(&value)? / 100.0)),
      _ => None,
    }
  }

  fn implicit_intersection_scalar_value(&self, value: FormulaValue<'doc>) -> FormulaValue<'doc> {
    match value {
      FormulaValue::Matrix(rows) => rows
        .first()
        .and_then(|row| row.first())
        .cloned()
        .unwrap_or(FormulaValue::Blank),
      value => value,
    }
  }

  pub(crate) fn evaluate_binary_values(
    &self,
    op: FormulaOperator,
    left: FormulaValue<'doc>,
    right: FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    if let Some(error) = propagate_binary_error(&left, &right) {
      return Some(FormulaValue::Error(error));
    }
    if op == FormulaOperator::Union {
      let left_ranges = self.reference_ranges_from_value(&left);
      let right_ranges = self.reference_ranges_from_value(&right);
      if let Some(value) = self.evaluate_union_ranges(left_ranges, right_ranges) {
        return Some(value);
      }
    }
    let (left, right) = if self.array_context {
      (left, right)
    } else {
      (
        self.scalar_binary_operand(left),
        self.scalar_binary_operand(right),
      )
    };
    if let Some(error) = propagate_binary_error(&left, &right) {
      return Some(FormulaValue::Error(error));
    }
    match op {
      FormulaOperator::Add => self.numeric_binary(left, right, approx_add),
      FormulaOperator::Subtract => self.numeric_binary(left, right, formula_subtract),
      FormulaOperator::Multiply => self.numeric_binary(left, right, |a, b| a * b),
      FormulaOperator::Divide => {
        if matches!(left, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
          || matches!(right, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        {
          return self.map_binary_values(left, right, |evaluator, left, right| {
            let Some(denominator) = evaluator.number(right) else {
              return Some(FormulaValue::Error(FormulaErrorValue::Value));
            };
            if denominator == 0.0 {
              Some(FormulaValue::Error(FormulaErrorValue::Div0))
            } else if let Some(numerator) = evaluator.number(left) {
              Some(FormulaValue::Number(numerator / denominator))
            } else {
              Some(FormulaValue::Error(FormulaErrorValue::Value))
            }
          });
        }
        let Some(denominator) = self.number(&right) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        if denominator == 0.0 {
          Some(FormulaValue::Error(FormulaErrorValue::Div0))
        } else if let Some(numerator) = self.number(&left) {
          Some(FormulaValue::Number(numerator / denominator))
        } else {
          Some(FormulaValue::Error(FormulaErrorValue::Value))
        }
      }
      FormulaOperator::Power => self.numeric_binary(left, right, formula_power),
      FormulaOperator::Concat => {
        if self.array_context && (is_matrix_argument(&left) || is_matrix_argument(&right)) {
          return self.map_binary_values(left, right, |evaluator, left, right| {
            Some(FormulaValue::String(Cow::Owned(format!(
              "{}{}",
              evaluator.text(left),
              evaluator.text(right)
            ))))
          });
        }
        Some(FormulaValue::String(Cow::Owned(format!(
          "{}{}",
          self.text(&left),
          self.text(&right)
        ))))
      }
      FormulaOperator::Union => {
        let mut rows = self.matrix_values(&left);
        rows.extend(self.matrix_values(&right));
        Some(FormulaValue::Matrix(rows))
      }
      FormulaOperator::Equal
      | FormulaOperator::NotEqual
      | FormulaOperator::Less
      | FormulaOperator::LessOrEqual
      | FormulaOperator::Greater
      | FormulaOperator::GreaterOrEqual => {
        let left_is_matrix_compare = matches!(left, FormulaValue::Matrix(_))
          || matches!(
            &left,
            FormulaValue::Reference(reference) if reference.range.cell_count_hint() != 1
          )
          || matches!(left, FormulaValue::RefList(_));
        let right_is_matrix_compare = matches!(right, FormulaValue::Matrix(_))
          || matches!(
            &right,
            FormulaValue::Reference(reference) if reference.range.cell_count_hint() != 1
          )
          || matches!(right, FormulaValue::RefList(_));
        if left_is_matrix_compare || right_is_matrix_compare {
          return self.map_binary_values(left, right, |evaluator, left, right| {
            Some(FormulaValue::Boolean(evaluator.compare(left, right, op)))
          });
        }
        let left = self.scalar_value(left);
        let right = self.scalar_value(right);
        Some(FormulaValue::Boolean(self.compare(&left, &right, op)))
      }
      _ => None,
    }
  }
}

fn is_matrix_argument(value: &FormulaValue<'_>) -> bool {
  matches!(
    value,
    FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
  )
}

fn formula_power(left: f64, right: f64) -> f64 {
  let result = left.powf(right);
  if result.is_finite() { result } else { f64::NAN }
}
