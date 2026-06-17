use super::*;

impl<'a, 'doc> FormulaEvaluator<'a, 'doc> {
  pub(crate) fn evaluate_unary(
    &self,
    op: FormulaOperator,
    expr: &FormulaAst<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let value = self.evaluate(expr)?;
    match op {
      FormulaOperator::UnaryPlus => Some(FormulaValue::Number(self.number(&value)?)),
      FormulaOperator::UnaryMinus => Some(FormulaValue::Number(-self.number(&value)?)),
      FormulaOperator::Percent => Some(FormulaValue::Number(self.number(&value)? / 100.0)),
      _ => None,
    }
  }

  pub(crate) fn evaluate_binary(
    &self,
    op: FormulaOperator,
    left: &FormulaAst<'doc>,
    right: &FormulaAst<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    if op == FormulaOperator::Intersection {
      return self.evaluate_intersection_ast(left, right);
    }
    if op == FormulaOperator::Range {
      return self.evaluate_range_ast(left, right);
    }
    if op == FormulaOperator::Union {
      let left_ranges = self.reference_ranges_from_ast(left);
      let right_ranges = self.reference_ranges_from_ast(right);
      if !left_ranges.is_empty() && !right_ranges.is_empty() {
        let mut ranges = left_ranges;
        ranges.extend(right_ranges);
        return Some(FormulaValue::RefList(ranges));
      }
    }
    let left = self.evaluate(left)?;
    let right = self.with_current_value(left.clone()).evaluate(right)?;
    if let Some(error) = propagate_binary_error(&left, &right) {
      return Some(FormulaValue::Error(error));
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
      FormulaOperator::Subtract => self.numeric_binary(left, right, approx_sub),
      FormulaOperator::Multiply => self.numeric_binary(left, right, |a, b| a * b),
      FormulaOperator::Divide => {
        if matches!(left, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
          || matches!(right, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        {
          return self.map_binary_values(left, right, |evaluator, left, right| {
            let denominator = evaluator.number(right)?;
            if denominator == 0.0 {
              Some(FormulaValue::Error(FormulaErrorValue::Div0))
            } else {
              Some(FormulaValue::Number(evaluator.number(left)? / denominator))
            }
          });
        }
        let denominator = self.number(&right)?;
        if denominator == 0.0 {
          Some(FormulaValue::Error(FormulaErrorValue::Div0))
        } else {
          Some(FormulaValue::Number(self.number(&left)? / denominator))
        }
      }
      FormulaOperator::Power => self.numeric_binary(left, right, f64::powf),
      FormulaOperator::Concat => Some(FormulaValue::String(Cow::Owned(format!(
        "{}{}",
        self.text(&left),
        self.text(&right)
      )))),
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
