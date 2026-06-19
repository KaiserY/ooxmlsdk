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
      FormulaValue::String(value) => formula_number_from_text(&value, self.book.date_system),
      FormulaValue::Blank => Some(0.0),
      FormulaValue::Error(_) => None,
      FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => None,
    }
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
    display_text_from_value(&self.scalar_value(value.clone()))
  }

  pub(crate) fn text_cow(&self, value: &FormulaValue<'doc>) -> Cow<'doc, str> {
    match self.scalar_value(value.clone()) {
      FormulaValue::String(value) => value,
      value => Cow::Owned(display_text_from_value(&value)),
    }
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
    let left = self.first_value(left);
    let right = self.first_value(right);
    let ordering = match (&left, &right) {
      (FormulaValue::Number(_), FormulaValue::Number(_))
      | (FormulaValue::Number(_), FormulaValue::Blank)
      | (FormulaValue::Blank, FormulaValue::Number(_))
      | (FormulaValue::Blank, FormulaValue::Blank)
      | (FormulaValue::Boolean(_), FormulaValue::Boolean(_))
      | (FormulaValue::Boolean(_), FormulaValue::Blank)
      | (FormulaValue::Blank, FormulaValue::Boolean(_)) => self
        .number(&left)
        .zip(self.number(&right))
        .and_then(|(left, right)| match compare_numbers(left, right) {
          -1 => Some(std::cmp::Ordering::Less),
          0 => Some(std::cmp::Ordering::Equal),
          1 => Some(std::cmp::Ordering::Greater),
          _ => None,
        }),
      (FormulaValue::Blank, FormulaValue::String(text))
      | (FormulaValue::String(text), FormulaValue::Blank)
        if text.is_empty() =>
      {
        Some(std::cmp::Ordering::Equal)
      }
      (FormulaValue::Boolean(_), FormulaValue::Number(_))
      | (FormulaValue::Number(_), FormulaValue::Boolean(_))
        if matches!(
          self.grammar,
          FormulaGrammar::CalcA1 | FormulaGrammar::OpenFormula
        ) =>
      {
        self
          .number(&left)
          .zip(self.number(&right))
          .and_then(|(left, right)| match compare_numbers(left, right) {
            -1 => Some(std::cmp::Ordering::Less),
            0 => Some(std::cmp::Ordering::Equal),
            1 => Some(std::cmp::Ordering::Greater),
            _ => None,
          })
      }
      (FormulaValue::Boolean(_), FormulaValue::Number(_)) => Some(std::cmp::Ordering::Greater),
      (FormulaValue::Number(_), FormulaValue::Boolean(_)) => Some(std::cmp::Ordering::Less),
      (FormulaValue::Boolean(_), FormulaValue::String(_)) => Some(std::cmp::Ordering::Greater),
      (FormulaValue::String(_), FormulaValue::Boolean(_)) => Some(std::cmp::Ordering::Less),
      (FormulaValue::String(_), FormulaValue::Number(_)) => Some(std::cmp::Ordering::Greater),
      (FormulaValue::Number(_), FormulaValue::String(_)) => Some(std::cmp::Ordering::Less),
      (FormulaValue::String(left), FormulaValue::String(right)) => {
        Some(left.to_ascii_lowercase().cmp(&right.to_ascii_lowercase()))
      }
      _ => match self.text(&left).cmp(&self.text(&right)) {
        std::cmp::Ordering::Equal => Some(std::cmp::Ordering::Greater),
        ordering => Some(ordering),
      },
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
