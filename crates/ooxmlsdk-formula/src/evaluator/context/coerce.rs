use super::*;

impl<'a, 'doc> FormulaEvaluator<'a, 'doc> {
  pub(crate) fn first_value(&self, value: &FormulaValue<'doc>) -> FormulaValue<'doc> {
    match value {
      FormulaValue::Reference(range) => self
        .range_values(range)
        .into_iter()
        .next()
        .unwrap_or_default(),
      FormulaValue::RefList(ranges) => ranges
        .first()
        .and_then(|range| self.range_values(range).into_iter().next())
        .unwrap_or_default(),
      FormulaValue::Matrix(rows) => rows
        .first()
        .and_then(|row| row.first())
        .cloned()
        .unwrap_or_default(),
      value => value.clone(),
    }
  }

  pub(crate) fn scalar_value(&self, value: FormulaValue<'doc>) -> FormulaValue<'doc> {
    match &value {
      FormulaValue::Reference(reference) => self.scalar_reference_value(reference),
      FormulaValue::RefList(ranges) => ranges
        .first()
        .map(|range| self.scalar_reference_value(range))
        .unwrap_or_default(),
      _ => self.first_value(&value),
    }
  }

  pub(crate) fn scalar_reference_value(
    &self,
    reference: &QualifiedRange<'doc>,
  ) -> FormulaValue<'doc> {
    let sheet = self.range_sheet(reference);
    let start_row = reference.range.start.row.min(reference.range.end.row);
    let end_row = reference.range.start.row.max(reference.range.end.row);
    let start_column = reference.range.start.column.min(reference.range.end.column);
    let end_column = reference.range.start.column.max(reference.range.end.column);
    if let Some(current) = self.current_cell {
      if start_column == end_column && (start_row..=end_row).contains(&current.row) {
        return self.book.cell_value(
          sheet,
          CellAddress {
            column: start_column,
            row: current.row,
          },
        );
      }
      if start_row == end_row && (start_column..=end_column).contains(&current.column) {
        return self.book.cell_value(
          sheet,
          CellAddress {
            column: current.column,
            row: start_row,
          },
        );
      }
    }
    self
      .range_values(reference)
      .into_iter()
      .next()
      .unwrap_or_default()
  }

  pub(crate) fn information_scalar_value(
    &self,
    value: FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    match value {
      FormulaValue::Reference(reference) if reference.range.cell_count_hint() == 1 => {
        self.range_values(&reference).into_iter().next()
      }
      FormulaValue::Reference(_) | FormulaValue::RefList(_) => None,
      FormulaValue::Matrix(rows) => rows
        .into_iter()
        .next()
        .and_then(|row| row.into_iter().next()),
      value => Some(value),
    }
  }

  pub(crate) fn evaluate_information_error(
    &self,
    args: &[FormulaAst<'doc>],
    matches_error: impl Fn(FormulaErrorValue) -> bool + Copy,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    if self.array_context
      && matches!(
        value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      )
    {
      return self.map_unary_values(value, |_, value| {
        Some(FormulaValue::Boolean(matches!(
          value,
          FormulaValue::Error(error) if matches_error(*error)
        )))
      });
    }
    Some(FormulaValue::Boolean(matches!(
      self.first_value(&value),
      FormulaValue::Error(error) if matches_error(error)
    )))
  }

  pub(crate) fn scalar_binary_operand(&self, value: FormulaValue<'doc>) -> FormulaValue<'doc> {
    match value {
      FormulaValue::Reference(reference) => self
        .implicit_intersection_value(&reference)
        .map(|value| self.first_value(&value))
        .unwrap_or(FormulaValue::Error(FormulaErrorValue::Value)),
      FormulaValue::RefList(ranges) => {
        if ranges.len() == 1 {
          self
            .implicit_intersection_value(&ranges[0])
            .map(|value| self.first_value(&value))
            .unwrap_or(FormulaValue::Error(FormulaErrorValue::Value))
        } else {
          FormulaValue::Error(FormulaErrorValue::Value)
        }
      }
      FormulaValue::Matrix(_) => value,
      value => value,
    }
  }

  pub(crate) fn implicit_intersection_value(
    &self,
    reference: &QualifiedRange<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    if reference.range.cell_count_hint() == 1 {
      return self.range_values(reference).into_iter().next();
    }
    let address = self.current_cell?;
    let start_row = reference.range.start.row.min(reference.range.end.row);
    let end_row = reference.range.start.row.max(reference.range.end.row);
    let start_column = reference.range.start.column.min(reference.range.end.column);
    let end_column = reference.range.start.column.max(reference.range.end.column);
    let sheet = self.range_sheet(reference);
    if start_column == end_column && (start_row..=end_row).contains(&address.row) {
      return Some(self.book.cell_value(
        sheet,
        CellAddress {
          column: start_column,
          row: address.row,
        },
      ));
    }
    if start_row == end_row && (start_column..=end_column).contains(&address.column) {
      return Some(self.book.cell_value(
        sheet,
        CellAddress {
          column: address.column,
          row: start_row,
        },
      ));
    }
    None
  }

  pub(crate) fn number(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    match self.first_value(value) {
      FormulaValue::Number(value) => Some(value),
      FormulaValue::Boolean(value) => Some(if value { 1.0 } else { 0.0 }),
      FormulaValue::String(value) => value.trim().parse::<f64>().ok(),
      FormulaValue::Blank => Some(0.0),
      FormulaValue::Error(_) => None,
      FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => None,
    }
  }

  pub(crate) fn number_arg(&self, args: &[FormulaAst<'doc>], index: usize) -> Option<f64> {
    args
      .get(index)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
  }

  pub(crate) fn optional_number_value(&self, arg: &FormulaAst<'doc>) -> Option<f64> {
    self
      .evaluate(arg)
      .filter(|value| !matches!(value, FormulaValue::Blank))
      .and_then(|value| self.number(&value))
  }

  pub(crate) fn optional_number_array_values(&self, arg: &FormulaAst<'doc>) -> Option<Vec<f64>> {
    let value = self.evaluate(arg)?;
    if matches!(value, FormulaValue::Blank) {
      return None;
    }
    let matrix = self.matrix_values(&value);
    let mut values = Vec::new();
    for value in matrix.into_iter().flatten() {
      values.push(self.number(&value)?);
    }
    Some(values)
  }

  pub(crate) fn optional_number_arg(
    &self,
    args: &[FormulaAst<'doc>],
    index: usize,
    default: f64,
  ) -> f64 {
    args
      .get(index)
      .and_then(|arg| self.optional_number_value(arg))
      .unwrap_or(default)
  }

  pub(crate) fn date_arg(&self, args: &[FormulaAst<'doc>], index: usize) -> Option<i32> {
    args
      .get(index)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.date_number_from_value(&value))
      .map(|value| value.floor() as i32)
  }

  pub(crate) fn optional_basis(&self, args: &[FormulaAst<'doc>], index: usize) -> i32 {
    args
      .get(index)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0)
      .floor() as i32
  }

  pub(crate) fn evaluate_numeric_unary(
    &self,
    args: &[FormulaAst<'doc>],
    op: impl Fn(f64) -> f64 + Copy,
  ) -> Option<FormulaValue<'doc>> {
    self.evaluate_numeric_unary_checked(args, |value| Some(op(value)))
  }

  pub(crate) fn evaluate_len(
    &self,
    args: &[FormulaAst<'doc>],
    bytes: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    let len = |evaluator: &Self, value: &FormulaValue<'doc>| {
      let text = evaluator.text(value);
      if bytes {
        text_byte_len(&text)
      } else {
        text.chars().count()
      }
    };
    if self.array_context
      && matches!(
        value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      )
    {
      return self.map_unary_values(value, |evaluator, value| {
        Some(FormulaValue::Number(len(evaluator, value) as f64))
      });
    }
    Some(FormulaValue::Number(len(self, &value) as f64))
  }

  pub(crate) fn evaluate_numeric_unary_checked(
    &self,
    args: &[FormulaAst<'doc>],
    op: impl Fn(f64) -> Option<f64> + Copy,
  ) -> Option<FormulaValue<'doc>> {
    self.evaluate_numeric_unary_checked_error(args, op, FormulaErrorValue::Unknown)
  }

  pub(crate) fn evaluate_numeric_unary_checked_error(
    &self,
    args: &[FormulaAst<'doc>],
    op: impl Fn(f64) -> Option<f64> + Copy,
    error: FormulaErrorValue,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    if (self.array_context
      && matches!(
        value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      ))
      || matches!(value, FormulaValue::Matrix(_))
    {
      return self.map_unary_values(value, |evaluator, value| {
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
    let value = self.scalar_binary_operand(value);
    if let FormulaValue::Error(error) = value {
      return Some(FormulaValue::Error(error));
    }
    self.number(&value).map(|value| match op(value) {
      Some(result) if result.is_finite() => FormulaValue::Number(result),
      Some(_) => FormulaValue::Error(FormulaErrorValue::Value),
      None => FormulaValue::Error(error),
    })
  }

  pub(crate) fn date_number_from_value(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    self.date_number_from_scalar(&self.first_value(value))
  }

  pub(crate) fn date_number_from_scalar(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    match value {
      FormulaValue::String(text) => match datevalue(text, self.book.date_system) {
        FormulaValue::Number(value) => Some(value),
        _ => None,
      },
      value => self.number(value),
    }
  }

  pub(crate) fn text(&self, value: &FormulaValue<'doc>) -> String {
    display_text_from_value(&self.first_value(value))
  }

  pub(crate) fn truthy(&self, value: &FormulaValue<'doc>) -> bool {
    match self.first_value(value) {
      FormulaValue::Boolean(value) => value,
      FormulaValue::Number(value) => value != 0.0,
      FormulaValue::String(value) => !value.is_empty(),
      FormulaValue::Blank | FormulaValue::Error(_) => false,
      FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => false,
    }
  }

  pub(crate) fn compare(
    &self,
    left: &FormulaValue<'doc>,
    right: &FormulaValue<'doc>,
    op: FormulaOperator,
  ) -> bool {
    let ordering = if let Some((left, right)) = self.number(left).zip(self.number(right)) {
      match compare_numbers(left, right) {
        -1 => Some(std::cmp::Ordering::Less),
        0 => Some(std::cmp::Ordering::Equal),
        1 => Some(std::cmp::Ordering::Greater),
        _ => None,
      }
    } else {
      Some(self.text(left).cmp(&self.text(right)))
    };
    match op {
      FormulaOperator::Equal => ordering == Some(std::cmp::Ordering::Equal),
      FormulaOperator::NotEqual => ordering != Some(std::cmp::Ordering::Equal),
      FormulaOperator::Less => ordering == Some(std::cmp::Ordering::Less),
      FormulaOperator::LessOrEqual => matches!(
        ordering,
        Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
      ),
      FormulaOperator::Greater => ordering == Some(std::cmp::Ordering::Greater),
      FormulaOperator::GreaterOrEqual => matches!(
        ordering,
        Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
      ),
      _ => false,
    }
  }
}
