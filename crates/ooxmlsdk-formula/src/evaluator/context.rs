use super::*;

mod coerce;
mod display;
mod operator;
mod reference;

pub(crate) use display::{
  display_text_from_value, display_text_from_value_with_number_format, error_text,
  error_text_value, error_value, format_text,
};

pub(crate) struct FormulaEvaluator<'a, 'doc> {
  pub(crate) book: &'a FormulaEvaluationBook<'doc>,
  pub(crate) engine: &'a CalcEngine,
  pub(crate) current_sheet: SheetId,
  pub(crate) current_cell: Option<CellAddress>,
  pub(crate) grammar: FormulaGrammar,
  pub(crate) locals: BTreeMap<String, FormulaValue<'doc>>,
  pub(crate) array_context: bool,
  pub(crate) current_value: Option<FormulaValue<'doc>>,
}

pub(crate) struct NumericAggregate {
  pub(crate) values: Vec<f64>,
}

impl<'a, 'doc> FormulaEvaluator<'a, 'doc> {
  pub(crate) fn with_array_context(&self) -> Self {
    Self {
      book: self.book,
      engine: self.engine,
      current_sheet: self.current_sheet,
      current_cell: self.current_cell,
      grammar: self.grammar,
      locals: self.locals.clone(),
      array_context: true,
      current_value: self.current_value.clone(),
    }
  }

  pub(crate) fn with_current_value(&self, current_value: FormulaValue<'doc>) -> Self {
    Self {
      book: self.book,
      engine: self.engine,
      current_sheet: self.current_sheet,
      current_cell: self.current_cell,
      grammar: self.grammar,
      locals: self.locals.clone(),
      array_context: self.array_context,
      current_value: Some(current_value),
    }
  }

  pub(crate) fn evaluate(&self, ast: &FormulaAst<'doc>) -> Option<FormulaValue<'doc>> {
    match ast {
      FormulaAst::Literal(value) => Some(value.clone()),
      FormulaAst::Reference(range) => Some(FormulaValue::Reference(range.clone())),
      FormulaAst::ExternalReference(reference) => self.evaluate_external_reference(reference),
      FormulaAst::Name(name) => self.evaluate_name(name),
      FormulaAst::Unary { op, expr } => self.evaluate_unary(*op, expr),
      FormulaAst::Binary { op, left, right } => self.evaluate_binary(*op, left, right),
      FormulaAst::Function {
        name,
        function,
        args,
      } => self.evaluate_function(*function, name, args),
      FormulaAst::Array(rows) => rows
        .iter()
        .map(|row| {
          row
            .iter()
            .map(|item| self.evaluate(item))
            .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>()
        .map(FormulaValue::Matrix),
    }
  }

  pub(crate) fn evaluate_function(
    &self,
    function: Option<FormulaFunctionId>,
    name: &Cow<'doc, str>,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    crate::function::evaluate_function(
      self,
      function,
      name,
      crate::function::FunctionArgs::new(args),
    )
  }

  pub(crate) fn evaluate_color(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=4).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let red = self.number_arg(args, 0)?;
    let green = self.number_arg(args, 1)?;
    let blue = self.number_arg(args, 2)?;
    let alpha = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    if [red, green, blue, alpha]
      .iter()
      .any(|value| !(0.0..=255.0).contains(value))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let red = red.floor() as u32;
    let green = green.floor() as u32;
    let blue = blue.floor() as u32;
    let alpha = alpha.floor() as u32;
    Some(FormulaValue::Number(
      ((alpha << 24) | (red << 16) | (green << 8) | blue) as f64,
    ))
  }

  pub(crate) fn evaluate_clean(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.evaluate(args.first()?)?;
    if self.array_context && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
      return Some(FormulaValue::Matrix(
        self
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
    Some(FormulaValue::String(Cow::Owned(clean_formula_text(
      &self.text(&value),
    ))))
  }

  pub(crate) fn evaluate_trim(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.evaluate(args.first()?)?;
    if self.array_context && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
      return Some(FormulaValue::Matrix(
        self
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
      &self.text(&value),
    ))))
  }

  pub(crate) fn evaluate_t(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    // Source: LibreOffice sc/source/core/tool/interpr1.cxx ScInterpreter::ScT.
    let value = self.evaluate(args.first()?)?;
    let value = match value {
      FormulaValue::Reference(reference) => self.scalar_reference_value(&reference),
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

  pub(crate) fn evaluate_value(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    // Source: LibreOffice sc/source/core/tool/interpr1.cxx ScInterpreter::ScValue.
    let value = self.evaluate(args.first()?)?;
    if self.array_context && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
      return Some(FormulaValue::Matrix(
        self
          .matrix_values(&value)
          .into_iter()
          .map(|row| {
            row
              .into_iter()
              .map(|value| self.value_from_formatter_text(&value))
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
        self.scalar_reference_value(&reference)
      }
      value => value,
    };
    Some(self.value_from_formatter_text(&value))
  }

  pub(crate) fn value_from_formatter_text(&self, value: &FormulaValue<'doc>) -> FormulaValue<'doc> {
    match value {
      FormulaValue::Blank => return FormulaValue::Number(0.0),
      FormulaValue::Number(number) => return FormulaValue::Number(*number),
      FormulaValue::Error(error) => return FormulaValue::Error(*error),
      _ => {}
    }
    let text = self.text(value);
    let text = text.trim();
    if text.is_empty() {
      return FormulaValue::Number(0.0);
    }
    let parsed = text
      .parse::<f64>()
      .ok()
      .or_else(|| crate::parser::grouped_formula_number(text))
      .or_else(|| parse_date_input(text, self.book.date_system).map(f64::floor))
      .or_else(|| match timevalue(text) {
        FormulaValue::Number(number) => Some(number),
        _ => None,
      })
      .or_else(|| {
        text
          .strip_prefix('$')
          .and_then(|value| crate::parser::grouped_formula_number(value.trim()))
      });
    parsed
      .map(FormulaValue::Number)
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::Value))
  }

  pub(crate) fn evaluate_getpivotdata(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 || args.len() % 2 == 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Ref));
    }
    let old_syntax = args.len() == 2
      && matches!(
        self.evaluate(args.first()?)?,
        FormulaValue::Reference(_) | FormulaValue::Matrix(_)
      );
    let (block, data_field_name, filters) = if old_syntax {
      let block = self.as_reference(&self.evaluate(args.first()?)?)?;
      let filter_value = self.evaluate(args.get(1)?)?;
      let filter_text = self.text(&self.first_value(&filter_value));
      let (data_field_name, filters) = parse_getpivotdata_filter_text(&filter_text);
      (block, data_field_name, filters)
    } else {
      let data_field_name = Cow::Owned(self.pivot_argument_text(args.first()?)?);
      let block = self.as_reference(&self.evaluate(args.get(1)?)?)?;
      let mut filters = Vec::new();
      for pair in args[2..].chunks(2) {
        filters.push(PivotFieldFilter {
          field_name: Cow::Owned(self.pivot_argument_text(&pair[0])?),
          match_value: Cow::Owned(self.pivot_argument_text(&pair[1])?),
        });
      }
      (block, Some(data_field_name), filters)
    };
    let request = PivotDataRequest {
      current_sheet: self.current_sheet,
      block,
      data_field_name,
      filters,
    };
    Some(match self.pivot_data(&request) {
      Ok(value) => value,
      Err(error) => FormulaValue::Error(error),
    })
  }

  pub(crate) fn pivot_argument_text(&self, arg: &FormulaAst<'doc>) -> Option<String> {
    let value = self.evaluate(arg)?;
    Some(self.text(&self.first_value(&value)))
  }

  pub(crate) fn evaluate_if(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let condition = self.evaluate(args.first()?)?;
    let if_value = |arg: Option<&FormulaAst<'doc>>, default: FormulaValue<'doc>| {
      Some(match arg.and_then(|arg| self.evaluate(arg)) {
        Some(FormulaValue::Blank) => FormulaValue::Number(0.0),
        Some(value) => value,
        None => default,
      })
    };
    if self.array_context
      && matches!(
        condition,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      )
    {
      let true_value = if_value(args.get(1), FormulaValue::Boolean(true))?;
      let false_value = if_value(args.get(2), FormulaValue::Boolean(false))?;
      return self.map_if_values(condition, true_value, false_value);
    }
    if let FormulaValue::Error(error) = self.first_value(&condition) {
      return Some(FormulaValue::Error(error));
    }
    if self.truthy(&condition) {
      if_value(args.get(1), FormulaValue::Boolean(true))
    } else {
      if_value(args.get(2), FormulaValue::Boolean(false))
    }
  }

  pub(crate) fn map_if_values(
    &self,
    condition: FormulaValue<'doc>,
    true_value: FormulaValue<'doc>,
    false_value: FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let conditions = self.matrix_values(&condition);
    let true_values = self.matrix_values(&true_value);
    let false_values = self.matrix_values(&false_value);
    let (condition_rows, condition_columns) = matrix_dimensions(&conditions);
    let (true_rows, true_columns) = matrix_dimensions(&true_values);
    let (false_rows, false_columns) = matrix_dimensions(&false_values);
    if condition_rows == 0
      || condition_columns == 0
      || true_rows == 0
      || true_columns == 0
      || false_rows == 0
      || false_columns == 0
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let rows = condition_rows.max(true_rows).max(false_rows);
    let columns = condition_columns.max(true_columns).max(false_columns);
    if !matrix_can_broadcast(condition_rows, condition_columns, rows, columns)
      || !matrix_can_broadcast(true_rows, true_columns, rows, columns)
      || !matrix_can_broadcast(false_rows, false_columns, rows, columns)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }

    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let condition = &conditions[row.min(condition_rows - 1)][column.min(condition_columns - 1)];
        result_row.push(match condition {
          FormulaValue::Error(error) => FormulaValue::Error(*error),
          condition if self.truthy(condition) => {
            true_values[row.min(true_rows - 1)][column.min(true_columns - 1)].clone()
          }
          _ => false_values[row.min(false_rows - 1)][column.min(false_columns - 1)].clone(),
        });
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_if_error(
    &self,
    args: &[FormulaAst<'doc>],
    na_only: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    if args.first().is_some_and(is_missing_argument) {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let value = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::Unknown));
    if self.array_context
      && matches!(
        value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      )
    {
      let fallback = self.evaluate(args.get(1)?)?;
      return self.map_if_error_values(value, fallback, na_only);
    }
    let value = self.scalar_value(value);
    if formula_error_matches(&value, na_only) {
      self.evaluate(args.get(1)?)
    } else {
      Some(value)
    }
  }

  pub(crate) fn map_if_error_values(
    &self,
    value: FormulaValue<'doc>,
    fallback: FormulaValue<'doc>,
    na_only: bool,
  ) -> Option<FormulaValue<'doc>> {
    let values = self.matrix_values(&value);
    let fallbacks = self.matrix_values(&fallback);
    let (value_rows, value_columns) = matrix_dimensions(&values);
    let (fallback_rows, fallback_columns) = matrix_dimensions(&fallbacks);
    if value_rows == 0 || value_columns == 0 || fallback_rows == 0 || fallback_columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let rows = value_rows.max(fallback_rows);
    let columns = value_columns.max(fallback_columns);
    if !matrix_can_broadcast(value_rows, value_columns, rows, columns)
      || !matrix_can_broadcast(fallback_rows, fallback_columns, rows, columns)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }

    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let value = &values[row.min(value_rows - 1)][column.min(value_columns - 1)];
        if formula_error_matches(value, na_only) {
          result_row
            .push(fallbacks[row.min(fallback_rows - 1)][column.min(fallback_columns - 1)].clone());
        } else {
          result_row.push(value.clone());
        }
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_text(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let array_evaluator = self.with_array_context();
    let value = array_evaluator.evaluate(args.first()?)?;
    let format = array_evaluator.evaluate(args.get(1)?)?;
    let values = array_evaluator.matrix_values(&value);
    let formats = array_evaluator.matrix_values(&format);
    let value_rows = values.len();
    let value_columns = values.first().map_or(0, Vec::len);
    let format_rows = formats.len();
    let format_columns = formats.first().map_or(0, Vec::len);
    if value_rows == 0 || value_columns == 0 || format_rows == 0 || format_columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let rows = value_rows.max(format_rows);
    let columns = value_columns.max(format_columns);
    if !matrix_can_broadcast(value_rows, value_columns, rows, columns)
      || !matrix_can_broadcast(format_rows, format_columns, rows, columns)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }

    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let value = &values[row.min(value_rows - 1)][column.min(value_columns - 1)];
        let format = &formats[row.min(format_rows - 1)][column.min(format_columns - 1)];
        result_row.push(if let Some(error) = propagate_binary_error(value, format) {
          FormulaValue::Error(error)
        } else {
          FormulaValue::String(Cow::Owned(format_text(value, format, self)))
        });
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_numbervalue(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.is_empty() || args.len() > 3 {
      return None;
    }
    let mut text = self.text(&self.evaluate(args.first()?)?);
    let decimal = if let Some(arg) = args.get(1) {
      match self.evaluate(arg)? {
        FormulaValue::Blank => {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        }
        value => {
          let text = self.text(&value);
          if text.chars().count() != 1 {
            return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
          }
          text
        }
      }
    } else {
      String::new()
    };
    let group = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.text(&value))
      .unwrap_or_default();
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

  pub(crate) fn evaluate_isblank(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return None;
    }
    let value = self.evaluate(args.first()?)?;
    if let FormulaValue::Reference(reference) = &value
      && reference.range.cell_count_hint() == 1
    {
      let sheet = self.range_sheet(reference);
      if self
        .book
        .formulas
        .contains_key(&(sheet, reference.range.start))
      {
        return Some(FormulaValue::Boolean(false));
      }
    }
    if self.array_context
      && let FormulaValue::Reference(reference) = &value
    {
      if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      let sheet = self.range_sheet(reference);
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
            !self.book.formulas.contains_key(&(sheet, address))
              && matches!(self.book.cell_value(sheet, address), FormulaValue::Blank),
          ));
        }
        rows.push(result_row);
      }
      return Some(FormulaValue::Matrix(rows));
    }
    if self.array_context && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
      let matrix = self.matrix_values(&value);
      return Some(FormulaValue::Matrix(
        matrix
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
    Some(FormulaValue::Boolean(matches!(
      self.first_value(&value),
      FormulaValue::Blank
    )))
  }

  pub(crate) fn evaluate_isnumber(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return None;
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
          FormulaValue::Number(_) | FormulaValue::Boolean(_)
        )))
      });
    }
    Some(FormulaValue::Boolean(matches!(
      self.information_scalar_value(value),
      Some(FormulaValue::Number(_) | FormulaValue::Boolean(_))
    )))
  }

  pub(crate) fn evaluate_not(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    match &value {
      FormulaValue::Reference(reference) if reference.range.cell_count_hint() == 1 => {}
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_) => {
        return self.map_unary_values(value, |evaluator, value| match value {
          FormulaValue::Error(error) => Some(FormulaValue::Error(*error)),
          value => Some(FormulaValue::Boolean(!evaluator.truthy(value))),
        });
      }
      _ => {}
    }
    let value = self.scalar_value(value);
    match value {
      FormulaValue::Error(error) => Some(FormulaValue::Error(error)),
      value => Some(FormulaValue::Boolean(!self.truthy(&value))),
    }
  }

  pub(crate) fn evaluate_n(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
      return self.map_unary_values(value, |_, value| match value {
        FormulaValue::Number(value) => Some(FormulaValue::Number(*value)),
        FormulaValue::Boolean(value) => Some(FormulaValue::Number(if *value { 1.0 } else { 0.0 })),
        FormulaValue::Error(error) => Some(FormulaValue::Error(*error)),
        _ => Some(FormulaValue::Number(0.0)),
      });
    }
    match &value {
      FormulaValue::Reference(reference) if reference.range.cell_count_hint() != 1 => {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      FormulaValue::RefList(_) => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
      _ => {}
    }
    Some(FormulaValue::Number(match self.first_value(&value) {
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

  pub(crate) fn evaluate_dollar(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.is_empty() || args.len() > 2 {
      return None;
    }
    let value = self.evaluate(args.first()?)?;
    let digits = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .unwrap_or(FormulaValue::Number(2.0));
    if self.array_context
      && (matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        || matches!(digits, FormulaValue::Reference(_) | FormulaValue::Matrix(_)))
    {
      return self.map_binary_values(value, digits, |evaluator, value, digits| {
        Some(evaluator.dollar_value(value, digits))
      });
    }
    Some(self.dollar_value(&value, &digits))
  }

  pub(crate) fn dollar_value(
    &self,
    value: &FormulaValue<'doc>,
    digits: &FormulaValue<'doc>,
  ) -> FormulaValue<'doc> {
    let Some(value) = self.number(value) else {
      return FormulaValue::Error(FormulaErrorValue::Value);
    };
    let Some(digits) = self.number(digits) else {
      return FormulaValue::Error(FormulaErrorValue::Value);
    };
    let Some(digits) = floor_to_i32(approx_floor(digits)) else {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    };
    if !(-15..=15).contains(&digits) {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    FormulaValue::String(Cow::Owned(format_dollar_value(
      round_to_decimal_places(value, digits),
      digits.max(0) as usize,
    )))
  }

  pub(crate) fn evaluate_dollar_decimal(
    &self,
    args: &[FormulaAst<'doc>],
    fractional_to_decimal: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let fraction = self.number(&self.evaluate(args.get(1)?)?)?.trunc();
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

  pub(crate) fn evaluate_fixed(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.is_empty() || args.len() > 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value_arg = self.evaluate(args.first()?)?;
    if !self.array_context && is_multicell_scalar_argument(&value_arg) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let Some(value) = self
      .number(&value_arg)
      .or_else(|| crate::parser::grouped_formula_number(&self.text(&value_arg)))
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let digits = match args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
    {
      Some(digits) => match floor_to_i32(approx_floor(digits)) {
        Some(digits) => digits,
        None => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
      },
      None => 2,
    };
    if !(-15..=15).contains(&digits) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let no_commas = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
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

  pub(crate) fn evaluate_decimal(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let text = self.text(&self.evaluate(args.first()?)?);
    let radix = self.number(&self.evaluate(args.get(1)?)?)?;
    let Some(radix) = trunc_to_u32(radix) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    if !(2..=36).contains(&radix) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    decimal_text_to_number(&text, radix)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_bit_binary(
    &self,
    args: &[FormulaAst<'doc>],
    op: impl FnOnce(u64, u64) -> u64,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(left) = self.number_arg(args, 0).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(right) = self.number_arg(args, 1).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if !(0.0..281_474_976_710_656.0).contains(&left)
      || !(0.0..281_474_976_710_656.0).contains(&right)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let Some(left) = floor_to_u64(left) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    };
    let Some(right) = floor_to_u64(right) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    };
    Some(FormulaValue::Number(op(left, right) as f64))
  }

  pub(crate) fn evaluate_bit_shift(
    &self,
    args: &[FormulaAst<'doc>],
    left_shift: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(value) = self.number_arg(args, 0).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(shift) = self.number_arg(args, 1).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(shift) = floor_to_i32(shift) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    };
    if !(0.0..281_474_976_710_656.0).contains(&value) || shift.abs() > 53 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let Some(value) = floor_to_u64(value) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    };
    let result = if left_shift == (shift >= 0) {
      value.checked_shl(shift.unsigned_abs()).unwrap_or(0)
    } else {
      value.checked_shr(shift.unsigned_abs()).unwrap_or(0)
    };
    Some(FormulaValue::Number(result as f64))
  }

  pub(crate) fn evaluate_convert(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let from = self.text(&self.evaluate(args.get(1)?)?);
    let to = self.text(&self.evaluate(args.get(2)?)?);
    Some(match convert_unit(value, &from, &to) {
      Ok(value) => FormulaValue::Number(value),
      Err(_) => FormulaValue::Error(FormulaErrorValue::IllegalArgument),
    })
  }

  pub(crate) fn evaluate_base(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let value = self.evaluate(args.first()?)?;
    let radix = self.evaluate(args.get(1)?)?;
    let min_len = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .unwrap_or(FormulaValue::Number(1.0));
    if self.array_context
      && (matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        || matches!(radix, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        || matches!(
          min_len,
          FormulaValue::Reference(_) | FormulaValue::Matrix(_)
        ))
    {
      return self.map_ternary_values(value, radix, min_len, |evaluator, value, radix, min_len| {
        evaluator.base_value(value, radix, min_len)
      });
    }
    self.base_value(&value, &radix, &min_len)
  }

  pub(crate) fn base_value(
    &self,
    value: &FormulaValue<'doc>,
    radix: &FormulaValue<'doc>,
    min_len: &FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let value = approx_floor(self.number(value)?);
    let radix = approx_floor(self.number(radix)?);
    let min_len_value = approx_floor(self.number(min_len)?);
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
    let text = base_number_text(value, floor_to_u32(radix)?, min_len)?;
    Some(FormulaValue::String(Cow::Owned(text)))
  }

  pub(crate) fn evaluate_base_to_decimal(
    &self,
    args: &[FormulaAst<'doc>],
    base: u32,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(arg) = args.first() else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let value = self.evaluate(arg)?;
    if self.array_context && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
      return Some(FormulaValue::Matrix(
        self
          .matrix_values(&value)
          .into_iter()
          .map(|row| {
            row
              .into_iter()
              .map(|value| {
                let text = self.base_digits_text(&value);
                convert_to_decimal(&text, base, 10)
                  .map(FormulaValue::Number)
                  .unwrap_or(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
              })
              .collect()
          })
          .collect(),
      ));
    }
    let text = self.base_digits_text(&value);
    convert_to_decimal(&text, base, 10)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_base_to_base(
    &self,
    args: &[FormulaAst<'doc>],
    from_base: u32,
    to_base: u32,
    min: f64,
    max: f64,
  ) -> Option<FormulaValue<'doc>> {
    if !(1..=2).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(arg) = args.first() else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let value = self.evaluate(arg)?;
    let places = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(|value| approx_floor(value) as i32);

    if self.array_context && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
      return Some(FormulaValue::Matrix(
        self
          .matrix_values(&value)
          .into_iter()
          .map(|row| {
            row
              .into_iter()
              .map(|value| {
                self
                  .base_to_base_value(&value, from_base, to_base, min, max, places)
                  .unwrap_or(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
              })
              .collect()
          })
          .collect(),
      ));
    }

    self.base_to_base_value(&value, from_base, to_base, min, max, places)
  }

  pub(crate) fn base_to_base_value(
    &self,
    value: &FormulaValue<'doc>,
    from_base: u32,
    to_base: u32,
    min: f64,
    max: f64,
    places: Option<i32>,
  ) -> Option<FormulaValue<'doc>> {
    let text = self.base_digits_text(value);
    let value = convert_to_decimal(&text, from_base, 10)?;
    convert_from_decimal(value, min, max, to_base, places, 10)
      .map(|value| FormulaValue::String(Cow::Owned(value)))
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_decimal_to_base(
    &self,
    args: &[FormulaAst<'doc>],
    base: u32,
    min: f64,
    max: f64,
  ) -> Option<FormulaValue<'doc>> {
    if !(1..=2).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(arg) = args.first() else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let value = self.number(&self.evaluate(arg)?)?;
    let places = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(|value| approx_floor(value) as i32);
    convert_from_decimal(value, min, max, base, places, 10)
      .map(|value| FormulaValue::String(Cow::Owned(value)))
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn base_digits_text(&self, value: &FormulaValue<'doc>) -> String {
    match self.scalar_value(value.clone()) {
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
      value => self.text(&value),
    }
  }

  pub(crate) fn evaluate_let(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 || args.len().is_multiple_of(2) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut evaluator = FormulaEvaluator {
      book: self.book,
      engine: self.engine,
      current_sheet: self.current_sheet,
      current_cell: self.current_cell,
      grammar: self.grammar,
      locals: self.locals.clone(),
      array_context: self.array_context,
      current_value: self.current_value.clone(),
    };
    let mut local_names = BTreeMap::new();
    let mut index = 0;
    while index + 2 < args.len() {
      let name = let_binding_name(&args[index])?;
      if name.is_empty() || local_names.insert(name.clone(), ()).is_some() {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      let value = evaluator.evaluate(&args[index + 1])?.into_owned();
      evaluator.locals.insert(name, value);
      index += 2;
    }
    evaluator.evaluate(args.last()?)
  }

  pub(crate) fn evaluate_round_direction(
    &self,
    args: &[FormulaAst<'doc>],
    away_from_zero: bool,
  ) -> Option<FormulaValue<'doc>> {
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let digits = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0) as i32;
    Some(FormulaValue::Number(round_direction(
      value,
      digits,
      away_from_zero,
    )))
  }

  pub(crate) fn evaluate_mod(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let number = self.evaluate(args.first()?)?;
    let divisor = self.evaluate(args.get(1)?)?;
    if self.array_context
      && (matches!(
        number,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      ) || matches!(
        divisor,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      ))
    {
      return self.map_binary_values(number, divisor, |evaluator, number, divisor| {
        Some(evaluator.mod_value(number, divisor))
      });
    }
    Some(self.mod_value(&number, &divisor))
  }

  pub(crate) fn mod_value(
    &self,
    number: &FormulaValue<'doc>,
    divisor: &FormulaValue<'doc>,
  ) -> FormulaValue<'doc> {
    let Some(number) = self.number(number) else {
      return FormulaValue::Error(FormulaErrorValue::Value);
    };
    let Some(divisor) = self.number(divisor) else {
      return FormulaValue::Error(FormulaErrorValue::Value);
    };
    match formula_mod(number, divisor) {
      Ok(result) => FormulaValue::Number(result),
      Err(error) => FormulaValue::Error(numeric_error_value(error)),
    }
  }

  pub(crate) fn evaluate_trunc(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=2).contains(&args.len()) || is_missing_argument(args.first()?) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let digits = match args.get(1) {
      Some(arg) if is_missing_argument(arg) => 0.0,
      Some(arg) => {
        let value = self.number(&self.evaluate(arg)?)?;
        if value < 0.0 {
          approx_ceil(value)
        } else {
          approx_floor(value)
        }
      }
      None => 0.0,
    };
    let value = self.number(&self.evaluate(args.first()?)?)?;
    Some(FormulaValue::Number(round_direction(
      value,
      digits.clamp(f64::from(i16::MIN), f64::from(i16::MAX)) as i32,
      false,
    )))
  }

  pub(crate) fn evaluate_roundsig(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let digits = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
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

  pub(crate) fn evaluate_raw_subtract(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut result = self.number(&self.evaluate(args.first()?)?)?;
    for arg in &args[1..] {
      result -= self.number(&self.evaluate(arg)?)?;
    }
    Some(FormulaValue::Number(result))
  }

  pub(crate) fn evaluate_sumproduct(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let array_evaluator = self.with_array_context();
    let matrices = args
      .iter()
      .map(|arg| {
        array_evaluator
          .evaluate(arg)
          .map(|value| array_evaluator.matrix_values(&value))
          .or(Some(vec![]))
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
    let mut total = KahanSum::default();
    for row in 0..rows {
      for column in 0..columns {
        let mut product = SumProductScalar::Number(1.0);
        for matrix in &matrices {
          product = sumproduct_merge_scalar(product, &matrix[row][column]);
        }
        match product {
          SumProductScalar::Number(value) => total.add(value),
          SumProductScalar::Error(error) => return Some(FormulaValue::Error(error)),
          SumProductScalar::NaN => {}
        }
      }
    }
    Some(FormulaValue::Number(total.finish()))
  }

  pub(crate) fn evaluate_choose(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let index = self.number(&self.evaluate(args.first()?)?)?.floor() as usize;
    if index == 0 || index >= args.len() {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    self.evaluate(args.get(index)?)
  }

  pub(crate) fn evaluate_find(
    &self,
    args: &[FormulaAst<'doc>],
    case_sensitive: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(needle_value) = self.evaluate(&args[0]) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(haystack_value) = self.evaluate(&args[1]) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let needle = self.text(&needle_value);
    let start = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as usize;
    if self.array_context
      && (matches!(
        needle_value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      ) || matches!(
        haystack_value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      ))
    {
      return self.map_find_values(needle_value, haystack_value, start, case_sensitive, false);
    }
    Some(self.find_text_value(&needle, &haystack_value, start, case_sensitive))
  }

  pub(crate) fn find_text_value(
    &self,
    needle: &str,
    haystack_value: &FormulaValue<'doc>,
    start: usize,
    case_sensitive: bool,
  ) -> FormulaValue<'doc> {
    let haystack = self.text(haystack_value);
    match find_text_position(
      needle,
      &haystack,
      start,
      case_sensitive,
      self.book.formula_search_type,
    ) {
      Ok(position) => FormulaValue::Number(position as f64),
      Err(_) => FormulaValue::Error(FormulaErrorValue::Value),
    }
  }

  pub(crate) fn evaluate_findb(
    &self,
    args: &[FormulaAst<'doc>],
    case_sensitive: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(needle_value) = self.evaluate(&args[0]) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(haystack_value) = self.evaluate(&args[1]) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let needle = self.text(&needle_value);
    let haystack = self.text(&haystack_value);
    let start = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as usize;
    if self.array_context
      && (matches!(
        needle_value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      ) || matches!(
        haystack_value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      ))
    {
      return self.map_find_values(needle_value, haystack_value, start, case_sensitive, true);
    }
    Some(self.findb_text_value(
      &needle,
      &FormulaValue::String(Cow::Owned(haystack)),
      start,
      case_sensitive,
    ))
  }

  pub(crate) fn findb_text_value(
    &self,
    needle: &str,
    haystack_value: &FormulaValue<'doc>,
    start: usize,
    case_sensitive: bool,
  ) -> FormulaValue<'doc> {
    let haystack = self.text(haystack_value);
    match find_byte_text_position(
      needle,
      &haystack,
      start,
      case_sensitive,
      self.book.formula_search_type,
    ) {
      Ok(position) => FormulaValue::Number(position as f64),
      Err(FindTextError::NotAvailable) => FormulaValue::Error(FormulaErrorValue::NA),
      Err(FindTextError::Value) => FormulaValue::Error(FormulaErrorValue::Value),
    }
  }

  pub(crate) fn map_find_values(
    &self,
    needle_value: FormulaValue<'doc>,
    haystack_value: FormulaValue<'doc>,
    start: usize,
    case_sensitive: bool,
    bytes: bool,
  ) -> Option<FormulaValue<'doc>> {
    let needles = self.matrix_values(&needle_value);
    let haystacks = self.matrix_values(&haystack_value);
    let (needle_rows, needle_columns) = matrix_dimensions(&needles);
    let (haystack_rows, haystack_columns) = matrix_dimensions(&haystacks);
    if needle_rows == 0 || needle_columns == 0 || haystack_rows == 0 || haystack_columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let rows = needle_rows.max(haystack_rows);
    let columns = needle_columns.max(haystack_columns);
    if !matrix_can_broadcast(needle_rows, needle_columns, rows, columns)
      || !matrix_can_broadcast(haystack_rows, haystack_columns, rows, columns)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let needle_row = if needle_rows == 1 { 0 } else { row };
        let needle_column = if needle_columns == 1 { 0 } else { column };
        let haystack_row = if haystack_rows == 1 { 0 } else { row };
        let haystack_column = if haystack_columns == 1 { 0 } else { column };
        let needle = needles.get(needle_row)?.get(needle_column)?;
        let haystack = haystacks.get(haystack_row)?.get(haystack_column)?;
        let needle_text = self.text(needle);
        result_row.push(if bytes {
          self.findb_text_value(&needle_text, haystack, start, case_sensitive)
        } else {
          self.find_text_value(&needle_text, haystack, start, case_sensitive)
        });
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_substitute(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(text_value) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(old_value) = args.get(1).and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(new_value) = args.get(2).and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let text = self.text(&text_value);
    let old = self.text(&old_value);
    let new = self.text(&new_value);
    if old.is_empty() {
      return Some(FormulaValue::String(Cow::Owned(text)));
    }
    if let Some(instance) = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
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

  pub(crate) fn evaluate_replace(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 4 {
      return None;
    }
    let text = self.text(&self.evaluate(args.first()?)?);
    let start = self.number(&self.evaluate(args.get(1)?)?)? as usize;
    let count = self.number(&self.evaluate(args.get(2)?)?)? as usize;
    let new_text = self.text(&self.evaluate(args.get(3)?)?);
    if start == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut chars = text.chars().collect::<Vec<_>>();
    let index = (start - 1).min(chars.len());
    let end = (index + count).min(chars.len());
    chars.splice(index..end, new_text.chars());
    Some(FormulaValue::String(Cow::Owned(
      chars.into_iter().collect(),
    )))
  }

  pub(crate) fn evaluate_text_before_after(
    &self,
    args: &[FormulaAst<'doc>],
    after: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(text_value) = self.evaluate(&args[0]) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(delimiter_value) = self.evaluate(&args[1]) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let text = self.text(&text_value);
    if text.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let delimiters = self.textsplit_delimiters(&delimiter_value);
    let mut instance = self.optional_number_arg(args, 2, 1.0) as i32;
    if instance == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    }
    let match_mode = self.optional_number_arg(args, 3, 0.0) != 0.0;
    let match_end = self.optional_number_arg(args, 4, 0.0) != 0.0;
    let if_not_found = args
      .get(5)
      .and_then(|arg| self.evaluate(arg))
      .filter(|value| !matches!(value, FormulaValue::Blank))
      .map(|value| self.text(&value));
    let mut positions = Vec::new();
    if match_end && after {
      positions.push(0usize);
    }
    let search_text = if match_mode {
      text.to_lowercase()
    } else {
      text.clone()
    };
    let search_delimiters = delimiters
      .iter()
      .map(|delimiter| {
        if match_mode {
          delimiter.to_lowercase()
        } else {
          delimiter.clone()
        }
      })
      .collect::<Vec<_>>();
    let mut start = 0usize;
    while start < search_text.len() {
      let mut found = None::<(usize, usize)>;
      for delimiter in &search_delimiters {
        if delimiter.is_empty() {
          continue;
        }
        if let Some(offset) = search_text[start..].find(delimiter) {
          let index = start + offset;
          if found.is_none_or(|(best, _)| index < best) {
            found = Some((index, delimiter.len()));
          }
        }
      }
      let Some((index, delimiter_len)) = found else {
        break;
      };
      positions.push(if after { index + delimiter_len } else { index });
      start = index + delimiter_len;
    }
    if match_end && !after {
      positions.push(text.len());
    }
    if positions.is_empty() || instance.unsigned_abs() as usize > positions.len() {
      return Some(if let Some(value) = if_not_found {
        FormulaValue::String(Cow::Owned(value))
      } else {
        FormulaValue::Error(FormulaErrorValue::NA)
      });
    }
    if instance < 0 {
      instance = positions.len() as i32 + instance + 1;
    };
    let position = positions[instance as usize - 1];
    Some(FormulaValue::String(Cow::Owned(if after {
      text[position..].to_string()
    } else {
      text[..position].to_string()
    })))
  }

  pub(crate) fn evaluate_textsplit(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.is_empty() || args.len() > 6 {
      return None;
    }
    let text = self.text(&self.evaluate(args.first()?)?);
    if text.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let column_delimiters = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.textsplit_delimiters(&value))
      .unwrap_or_default();
    let row_delimiters = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.textsplit_delimiters(&value))
      .unwrap_or_default();
    let ignore_empty = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    let match_mode = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    let pad_with = args.get(5).and_then(|arg| self.evaluate(arg));
    let pad_with = pad_with.as_ref().map(|value| self.text(value));
    let row_texts = split_text_multi(&text, &row_delimiters, ignore_empty, match_mode);
    let mut result_rows = Vec::with_capacity(row_texts.len());
    let mut columns = 1usize;
    for row in row_texts {
      let values = split_text_multi(&row, &column_delimiters, ignore_empty, match_mode);
      columns = columns.max(values.len());
      result_rows.push(values);
    }
    Some(FormulaValue::Matrix(
      result_rows
        .into_iter()
        .map(|row| {
          (0..columns)
            .map(|column| {
              row
                .get(column)
                .map(|value| FormulaValue::String(Cow::Owned(value.clone())))
                .unwrap_or_else(|| {
                  pad_with
                    .as_ref()
                    .map(|value| FormulaValue::String(Cow::Owned(value.clone())))
                    .unwrap_or(FormulaValue::Error(FormulaErrorValue::NA))
                })
            })
            .collect()
        })
        .collect(),
    ))
  }

  pub(crate) fn textsplit_delimiters(&self, value: &FormulaValue<'doc>) -> Vec<String> {
    self
      .matrix_values(value)
      .into_iter()
      .flatten()
      .map(|value| self.text(&value))
      .collect()
  }

  pub(crate) fn evaluate_textjoin(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 {
      return None;
    }
    let mut delimiters = self
      .values(&args[..1])
      .map(|value| self.text(&value))
      .collect::<Vec<_>>();
    if delimiters.is_empty() {
      delimiters.push(String::new());
    }
    let ignore_empty = self.truthy(&self.evaluate(args.get(1)?)?);
    let mut output = String::new();
    let mut count = 0usize;
    for value in self.values(&args[2..]) {
      if ignore_empty && matches!(self.first_value(&value), FormulaValue::Blank) {
        continue;
      }
      let text = self.text(&value);
      if ignore_empty && text.is_empty() {
        continue;
      }
      if count > 0 {
        output.push_str(&delimiters[(count - 1) % delimiters.len()]);
      }
      output.push_str(&text);
      count += 1;
    }
    Some(FormulaValue::String(Cow::Owned(output)))
  }

  pub(crate) fn evaluate_width_conversion(
    &self,
    args: &[FormulaAst<'doc>],
    full_width: bool,
  ) -> Option<FormulaValue<'doc>> {
    let text = self.text(&self.evaluate(args.first()?)?);
    Some(FormulaValue::String(Cow::Owned(if full_width {
      full_width_like_jis(&text)
    } else {
      half_width_like_asc(&text)
    })))
  }

  pub(crate) fn evaluate_row_column(
    &self,
    args: &[FormulaAst<'doc>],
    column: bool,
  ) -> Option<FormulaValue<'doc>> {
    let reference = if let Some(arg) = args.first() {
      let Some(value) = self.evaluate(arg) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      let Some(reference) = self.as_reference(&value) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      Some(reference)
    } else {
      None
    };
    let address = reference
      .as_ref()
      .map(|reference| reference.range.start)
      .unwrap_or_else(|| self.current_cell.unwrap_or_default());
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

  pub(crate) fn evaluate_rows_columns(
    &self,
    args: &[FormulaAst<'doc>],
    columns: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.is_empty() {
      return Some(FormulaValue::Number(0.0));
    }
    match self.evaluate(args.first()?)? {
      FormulaValue::Reference(reference) => {
        let range = reference.range;
        Some(FormulaValue::Number(if columns {
          range.start.column.abs_diff(range.end.column) as f64 + 1.0
        } else {
          range.start.row.abs_diff(range.end.row) as f64 + 1.0
        }))
      }
      FormulaValue::Matrix(rows) => Some(FormulaValue::Number(if columns {
        rows.first().map_or(0, Vec::len) as f64
      } else {
        rows.len() as f64
      })),
      _ => Some(FormulaValue::Number(1.0)),
    }
  }

  pub(crate) fn evaluate_is_formula(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let reference = self.as_reference(&self.evaluate(args.first()?)?)?;
    if !self.array_context && reference.range.cell_count_hint() != 1 {
      return Some(FormulaValue::Boolean(false));
    }
    let sheet = self.range_sheet(&reference);
    Some(FormulaValue::Boolean(
      self
        .book
        .formulas
        .contains_key(&(sheet, reference.range.start)),
    ))
  }

  pub(crate) fn evaluate_error_type(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(FormulaValue::Error(error)) = self.first_error_value(&value) else {
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

  pub(crate) fn evaluate_error_type_raw(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let direct_unknown_error = matches!(
      args.first(),
      Some(FormulaAst::Literal(FormulaValue::Error(
        FormulaErrorValue::Unknown
      )))
    );
    let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if let FormulaValue::Reference(reference) = &value
      && reference.range.cell_count_hint() == 1
      && self.first_error_value(&value).is_none()
    {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    }
    if matches!(
      value,
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
    ) && self.first_error_value(&value).is_none()
    {
      return Some(FormulaValue::Number(519.0));
    }
    let Some(FormulaValue::Error(error)) = self.first_error_value(&value) else {
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

  pub(crate) fn evaluate_info(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let key = self
      .text(&self.evaluate(args.first()?)?)
      .to_ascii_uppercase();
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

  pub(crate) fn evaluate_type(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if matches!(value, FormulaValue::Matrix(_) | FormulaValue::RefList(_)) {
      return Some(FormulaValue::Number(64.0));
    }
    Some(FormulaValue::Number(match self.first_value(&value) {
      FormulaValue::Number(_) => 1.0,
      FormulaValue::String(_) => 2.0,
      FormulaValue::Boolean(_) => 1.0,
      FormulaValue::Error(_) => 16.0,
      FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => 64.0,
      FormulaValue::Blank => 1.0,
    }))
  }

  pub(crate) fn evaluate_areas(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(value) = self.evaluate(args.first()?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let ranges = self.reference_ranges_from_value(&value);
    if !ranges.is_empty() {
      return Some(FormulaValue::Number(ranges.len() as f64));
    }
    Some(match value {
      FormulaValue::Matrix(_) => FormulaValue::Number(1.0),
      _ => FormulaValue::Error(FormulaErrorValue::Value),
    })
  }

  pub(crate) fn evaluate_ifs_function(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 || !args.len().is_multiple_of(2) {
      return None;
    }
    for pair in args.chunks_exact(2) {
      let condition = self.evaluate(&pair[0])?;
      if let FormulaValue::Error(error) = condition {
        return Some(FormulaValue::Error(error));
      }
      if self.truthy(&condition) {
        return self.evaluate(&pair[1]);
      }
    }
    Some(FormulaValue::Error(FormulaErrorValue::NA))
  }

  pub(crate) fn evaluate_switch(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 {
      return None;
    }
    let selector = self.scalar_value(self.evaluate(args.first()?)?);
    if let FormulaValue::Error(error) = &selector {
      return Some(FormulaValue::Error(*error));
    }
    let pairs_len = if args.len().is_multiple_of(2) {
      args.len() - 2
    } else {
      args.len() - 1
    };
    for pair in args[1..=pairs_len].chunks_exact(2) {
      let candidate = self.scalar_value(self.evaluate(&pair[0])?);
      if let FormulaValue::Error(error) = &candidate {
        return Some(FormulaValue::Error(*error));
      }
      let matches = match (&selector, &candidate) {
        (FormulaValue::String(left), FormulaValue::String(right)) => {
          left.eq_ignore_ascii_case(right)
        }
        _ => self.compare(&selector, &candidate, FormulaOperator::Equal),
      };
      if matches {
        return Some(self.scalar_value(self.evaluate(&pair[1])?));
      }
    }
    if args.len().is_multiple_of(2) {
      Some(self.scalar_value(self.evaluate(args.last()?)?))
    } else {
      Some(FormulaValue::Error(FormulaErrorValue::NA))
    }
  }

  pub(crate) fn evaluate_date(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 || args.first().is_some_and(is_missing_argument) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut year = self.number(&self.evaluate(args.first()?)?)? as i32;
    let month = self.number(&self.evaluate(args.get(1)?)?)? as i32;
    let day = self.number(&self.evaluate(args.get(2)?)?)? as i32;
    if year < 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    if year < 100 {
      year = expand_two_digit_year(year);
    }
    let (normalized_year, normalized_month, normalized_day) =
      normalized_date_components(year, month, day)?;
    if !is_valid_libreoffice_gregorian_date(normalized_year, normalized_month, normalized_day) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    date_serial_with_system(year, month, day, self.book.date_system).map(FormulaValue::Number)
  }

  pub(crate) fn evaluate_address(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=5).contains(&args.len()) {
      return None;
    }
    let row = self.number(&self.evaluate(args.first()?)?)? as i32;
    let column = self.number(&self.evaluate(args.get(1)?)?)? as i32;
    if row <= 0
      || column <= 0
      || row as u32 > XLSX_MAX_ROW_ZERO_BASED + 1
      || column as u32 > XLSX_MAX_COLUMN_ZERO_BASED + 1
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let abs_num = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| {
        if matches!(value, FormulaValue::Blank) {
          Some(1.0)
        } else {
          self.number(&value)
        }
      })
      .unwrap_or(1.0) as i32;
    let a1 = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| {
        if matches!(value, FormulaValue::Blank) {
          true
        } else {
          self.truthy(&value)
        }
      })
      .unwrap_or(true);
    let sheet = args.get(4).and_then(|arg| self.evaluate(arg)).map(|value| {
      let sheet = self.text(&value);
      if sheet.is_empty() {
        String::new()
      } else if a1 {
        let separator = match self.grammar {
          FormulaGrammar::OpenFormula | FormulaGrammar::CalcA1 => ".",
          FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1 => "!",
        };
        format!(
          "{}{}",
          quote_sheet_name_for_reference(sheet.as_ref()),
          separator
        )
      } else {
        format!("{}!", quote_sheet_name_for_reference(sheet.as_ref()))
      }
    });
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

  pub(crate) fn evaluate_time(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let hour = self.number(&self.evaluate(args.first()?)?)?;
    let minute = self.number(&self.evaluate(args.get(1)?)?)?;
    let second = self.number(&self.evaluate(args.get(2)?)?)?;
    let value = ((hour * 3600.0 + minute * 60.0 + second) % 86_400.0) / 86_400.0;
    if value < 0.0 {
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    } else {
      Some(FormulaValue::Number(value))
    }
  }

  pub(crate) fn evaluate_date_part(
    &self,
    args: &[FormulaAst<'doc>],
    part: DatePart,
  ) -> Option<FormulaValue<'doc>> {
    if self.array_context {
      let value = self.evaluate(args.first()?)?;
      if matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
        return Some(FormulaValue::Matrix(
          self
            .matrix_values(&value)
            .into_iter()
            .map(|row| {
              row
                .into_iter()
                .map(|value| match self.date_number_from_scalar(&value) {
                  Some(serial) => {
                    let serial = serial.floor() as i32;
                    match date_from_serial_with_system(serial, self.book.date_system) {
                      Some((year, month, day)) => FormulaValue::Number(match part {
                        DatePart::Year => year as f64,
                        DatePart::Month => month as f64,
                        DatePart::Day => day as f64,
                      }),
                      None => FormulaValue::Error(FormulaErrorValue::Value),
                    }
                  }
                  None => FormulaValue::Error(FormulaErrorValue::Value),
                })
                .collect()
            })
            .collect(),
        ));
      }
    }
    let Some(serial) = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.date_number_from_value(&value))
      .map(|value| value.floor() as i32)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let (year, month, day) = date_from_serial_with_system(serial, self.book.date_system)?;
    Some(FormulaValue::Number(match part {
      DatePart::Year => year as f64,
      DatePart::Month => month as f64,
      DatePart::Day => day as f64,
    }))
  }

  pub(crate) fn evaluate_today(&self) -> Option<FormulaValue<'doc>> {
    if let Some(value) = self.book.today_serial {
      return Some(FormulaValue::Number(value.floor()));
    }
    let unix_days = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .ok()
      .map(|duration| duration.as_secs() / 86_400)?;
    Some(FormulaValue::Number(
      date_serial_with_system(1970, 1, 1, self.book.date_system)? + unix_days as f64,
    ))
  }

  pub(crate) fn evaluate_days360(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let mut start = self
      .date_number_from_value(&self.evaluate(args.first()?)?)?
      .floor() as i32;
    let mut end = self
      .date_number_from_value(&self.evaluate(args.get(1)?)?)?
      .floor() as i32;
    let european = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .is_some_and(|value| value != 0.0);
    let mut sign = 1;
    if european && end < start {
      std::mem::swap(&mut start, &mut end);
      sign = -1;
    }
    days360(start, end, european)
      .map(|value| FormulaValue::Number(f64::from(sign * value)))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_eomonth(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(start) = self.date_number_from_value(&self.evaluate(args.first()?)?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(months) = self.number(&self.evaluate(args.get(1)?)?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let start = start.floor() as i32;
    let months = months.floor() as i32;
    let (year, month, _) = date_from_serial_with_system(start, self.book.date_system)?;
    date_serial_with_system(year, month as i32 + months + 1, 0, self.book.date_system)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_edate(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let start = self
      .date_number_from_value(&self.evaluate(args.first()?)?)?
      .floor() as i32;
    let months = self.number(&self.evaluate(args.get(1)?)?)?.floor() as i32;
    let (year, month, day) = date_from_serial_with_system(start, self.book.date_system)?;
    let (target_year, target_month, _) =
      normalized_date_components(year, month as i32 + months, 1)?;
    let target_day = day.min(last_day_of_month(target_year, target_month));
    let target = date_serial_with_system(
      target_year,
      target_month as i32,
      target_day as i32,
      self.book.date_system,
    );
    target
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_is_leap_year(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    if is_multicell_scalar_argument(&value) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let Some(serial) = self.date_number_from_value(&value) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let serial = serial as i32;
    let (year, _, _) = date_from_serial_with_system(serial, self.book.date_system)?;
    Some(FormulaValue::Boolean(is_leap_year(year)))
  }

  pub(crate) fn evaluate_basis_o_datetime(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return None;
    }
    basis_o_datetime_text(self.number(&self.evaluate(args.first()?)?)?)
      .map(|value| FormulaValue::String(Cow::Owned(value)))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_datedif(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let start = self.number(&self.evaluate(args.first()?)?)?.floor() as i32;
    let end = self.number(&self.evaluate(args.get(1)?)?)?.floor() as i32;
    let interval = self
      .text(&self.evaluate(args.get(2)?)?)
      .to_ascii_lowercase();
    if start > end {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let days = f64::from(end - start);
    if days == 0.0 || interval == "d" {
      return Some(FormulaValue::Number(days));
    }
    let (start_year, start_month, start_day) =
      date_from_serial_with_system(start, self.book.date_system)?;
    let (end_year, end_month, end_day) = date_from_serial_with_system(end, self.book.date_system)?;
    let month_diff = end_month as i32 - start_month as i32 + 12 * (end_year - start_year)
      - i32::from(start_day > end_day);
    match interval.as_str() {
      "m" => Some(FormulaValue::Number(month_diff as f64)),
      "y" => Some(FormulaValue::Number(if end_year > start_year
        && (end_month > start_month || (end_month == start_month && end_day >= start_day))
      {
        end_year - start_year
      } else if end_year > start_year {
        end_year - start_year - 1
      } else {
        0
      } as f64)),
      "md" => {
        if start_day <= end_day {
          return Some(FormulaValue::Number((end_day - start_day) as f64));
        }
        let (roll_year, roll_month) = if end_month == 1 {
          (end_year - 1, 12)
        } else {
          (end_year, end_month as i32 - 1)
        };
        let roll = date_serial_with_system(
          roll_year,
          roll_month,
          start_day as i32,
          self.book.date_system,
        )?;
        Some(FormulaValue::Number(f64::from(end) - roll))
      }
      "ym" => Some(FormulaValue::Number(month_diff.rem_euclid(12) as f64)),
      "yd" => {
        let same_year_start =
          if end_month > start_month || (end_month == start_month && end_day >= start_day) {
            date_serial_with_system(
              end_year,
              start_month as i32,
              start_day as i32,
              self.book.date_system,
            )?
          } else {
            date_serial_with_system(
              end_year - 1,
              start_month as i32,
              start_day as i32,
              self.book.date_system,
            )?
          };
        Some(FormulaValue::Number(f64::from(end) - same_year_start))
      }
      _ => Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    }
  }

  pub(crate) fn evaluate_yearfrac(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let start = self.date_number_from_value(&self.evaluate(args.first()?)?)? as i32;
    let end = self.date_number_from_value(&self.evaluate(args.get(1)?)?)? as i32;
    let basis = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0) as i32;
    yearfrac(start, end, basis)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_weeknum(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=2).contains(&args.len()) {
      return None;
    }
    let serial = self.date_number_from_value(&self.evaluate(args.first()?)?)? as i64;
    let mode = match args.get(1).and_then(|arg| self.evaluate(arg)) {
      Some(FormulaValue::Reference(_) | FormulaValue::Matrix(_) | FormulaValue::RefList(_)) => {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      Some(FormulaValue::Blank) | None => 1.0,
      Some(value) => self.number(&value)?,
    } as i32;
    let weekday = weekday_index_from_serial(serial) as i32;
    let year = date_from_serial_with_system(serial as i32, self.book.date_system)?.0;
    let jan1 = date_serial_with_system(year, 1, 1, self.book.date_system)? as i64;
    let jan1_weekday = weekday_index_from_serial(jan1) as i32;
    if matches!(mode, 21 | 150) {
      return iso_weeknum_from_serial_with_system(serial as i32, self.book.date_system)
        .map(|value| FormulaValue::Number(value as f64))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Value)));
    }
    let week_start = match mode {
      1 | 17 => 6,
      2 | 11 => 0,
      12 => 1,
      13 => 2,
      14 => 3,
      15 => 4,
      16 => 5,
      _ => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    };
    let offset = (jan1_weekday - week_start).rem_euclid(7);
    let week = ((serial - jan1 + offset as i64) / 7) + 1;
    let _ = weekday;
    Some(FormulaValue::Number(week as f64))
  }

  pub(crate) fn evaluate_iso_weeknum(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let value = self.evaluate(args.first()?)?;
    if is_multicell_scalar_argument(&value) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let serial = match self.date_number_from_value(&value) {
      Some(value) => value as i32,
      None => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    };
    iso_weeknum_from_serial_with_system(serial, self.book.date_system)
      .map(|value| FormulaValue::Number(value as f64))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_weeks(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    // Source: LibreOffice scaddins/source/datefunc/datefunc.cxx
    // ScaDateAddIn::getDiffWeeks.
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let start = match self.date_number_from_value(&self.evaluate(args.first()?)?) {
      Some(value) => value as i32,
      None => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    };
    let end = match self.date_number_from_value(&self.evaluate(args.get(1)?)?) {
      Some(value) => value as i32,
      None => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    };
    let mode_arg = self.evaluate(args.get(2)?)?;
    if matches!(mode_arg, FormulaValue::Blank) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mode = match self.number(&mode_arg) {
      Some(value) => value as i32,
      None => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    };
    match mode {
      0 => Some(FormulaValue::Number(((end - start) / 7) as f64)),
      1 => {
        let Some(start_week) = weeks_mode_one_index(start, self.book.date_system) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        let Some(end_week) = weeks_mode_one_index(end, self.book.date_system) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        Some(FormulaValue::Number((end_week - start_week) as f64))
      }
      _ => Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    }
  }

  pub(crate) fn evaluate_weeks_in_year(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let serial = match self.date_number_from_value(&self.evaluate(args.first()?)?) {
      Some(value) => value as i32,
      None => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    };
    weeks_in_year_from_serial_with_system(serial, self.book.date_system)
      .map(|value| FormulaValue::Number(value as f64))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_years_months(
    &self,
    args: &[FormulaAst<'doc>],
    years: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let start = self.date_number_from_value(&self.evaluate(args.first()?)?)? as i32;
    let end = self.date_number_from_value(&self.evaluate(args.get(1)?)?)? as i32;
    let mode_arg = self.evaluate(args.get(2)?)?;
    if matches!(mode_arg, FormulaValue::Blank) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mode = self.number(&mode_arg)? as i32;
    if !matches!(mode, 0 | 1) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let result = if years {
      date_diff_years(start, end, mode)?
    } else {
      date_diff_months(start, end, mode)?
    };
    Some(FormulaValue::Number(result as f64))
  }

  pub(crate) fn evaluate_days_in_month_year(
    &self,
    args: &[FormulaAst<'doc>],
    year: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let Some(serial) = self.date_number_from_value(&self.evaluate(args.first()?)?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let serial = serial as i32;
    let (year_value, month, _) = date_from_serial(serial)?;
    Some(FormulaValue::Number(if year {
      if is_leap_year(year_value) {
        366.0
      } else {
        365.0
      }
    } else {
      last_day_of_month(year_value, month) as f64
    }))
  }

  pub(crate) fn evaluate_weekday(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let serial = self
      .date_number_from_value(&self.evaluate(args.first()?)?)?
      .floor() as i64;
    let flag = match args.get(1).and_then(|arg| self.evaluate(arg)) {
      Some(FormulaValue::Reference(_) | FormulaValue::Matrix(_) | FormulaValue::RefList(_)) => {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      Some(value) => self.number(&value).unwrap_or(1.0),
      None => 1.0,
    } as i32;
    let monday_zero = weekday_index_from_serial(serial) as i32;
    Some(FormulaValue::Number(match flag {
      1 => {
        if monday_zero == 6 {
          1.0
        } else {
          monday_zero as f64 + 2.0
        }
      }
      2 => monday_zero as f64 + 1.0,
      3 => monday_zero as f64,
      11..=17 => {
        let start = flag - 11;
        if monday_zero < start {
          (monday_zero + 8 - start) as f64
        } else {
          (monday_zero - start + 1) as f64
        }
      }
      _ => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    }))
  }

  pub(crate) fn evaluate_time_part(
    &self,
    args: &[FormulaAst<'doc>],
    part: TimePart,
  ) -> Option<FormulaValue<'doc>> {
    let value = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.time_number_from_value(&value))
      .unwrap_or_default();
    let total_seconds = (value.fract() * 86_400.0).floor() as i64;
    Some(FormulaValue::Number(match part {
      TimePart::Hour => total_seconds.rem_euclid(86_400) / 3600,
      TimePart::Minute => total_seconds.rem_euclid(3_600) / 60,
      TimePart::Second => ((value.fract() * 86_400.0).round() as i64).rem_euclid(60),
    } as f64))
  }

  pub(crate) fn time_number_from_value(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    match self.first_value(value) {
      FormulaValue::String(text) => match timevalue(&text) {
        FormulaValue::Number(value) => Some(value),
        _ => None,
      },
      value => self.number(&value),
    }
  }

  pub(crate) fn evaluate_indirect(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(value) = self.evaluate(args.first()?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let text = self.text(&value);
    let use_a1 = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    let reference_text;
    let text = if use_a1 {
      text.as_str()
    } else {
      let Some(converted) =
        crate::parser::r1c1_reference_to_a1(&text, self.current_cell.unwrap_or_default())
      else {
        return Some(FormulaValue::Error(FormulaErrorValue::Ref));
      };
      reference_text = converted;
      reference_text.as_str()
    };
    self
      .resolve_reference(text)
      .map(FormulaValue::Reference)
      .or_else(|| self.evaluate_defined_name(&Cow::Owned(text.to_string())))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Ref)))
  }

  pub(crate) fn evaluate_index(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=4).contains(&args.len()) {
      return None;
    }
    let row = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let column = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let area = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0);
    if row < 0.0 || column < 0.0 || area < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let row = row as u32;
    let column = column as u32;
    let area = area as usize;
    let ranges = self.reference_ranges_from_ast(args.first()?);
    if !ranges.is_empty() {
      return Some(self.index_reference_area(&ranges, row, column, area, args.len()));
    }
    let Some(value) = self.evaluate(args.first()?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    if let FormulaValue::Matrix(rows) = value {
      return Some(index_matrix(rows, row, column, args.len()));
    }
    let Some(reference) = self.as_reference(&value) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    Some(self.index_reference_area(&[reference], row, column, area, args.len()))
  }

  pub(crate) fn index_reference_area(
    &self,
    ranges: &[QualifiedRange<'doc>],
    row: u32,
    column: u32,
    area: usize,
    arg_count: usize,
  ) -> FormulaValue<'doc> {
    let Some(reference) = ranges.get(area - 1) else {
      return FormulaValue::Error(FormulaErrorValue::Ref);
    };
    let start_column = reference.range.start.column.min(reference.range.end.column);
    let end_column = reference.range.start.column.max(reference.range.end.column);
    let start_row = reference.range.start.row.min(reference.range.end.row);
    let end_row = reference.range.start.row.max(reference.range.end.row);
    let b_row_array = arg_count == 2 && start_row == end_row;
    if (column > 0 && start_column + column - 1 > end_column)
      || (row > 0 && start_row + row - 1 > end_row && !b_row_array)
      || (b_row_array && row > end_column - start_column + 1)
    {
      return FormulaValue::Error(FormulaErrorValue::Ref);
    }
    if row == 0 && column == 0 {
      return FormulaValue::Reference(reference.clone());
    }
    let range = if row == 0 {
      let selected_column = start_column + column - 1;
      CellRange::new(
        CellAddress {
          column: selected_column,
          row: start_row,
        },
        CellAddress {
          column: selected_column,
          row: end_row,
        },
      )
    } else if column == 0 {
      if b_row_array {
        let selected_column = start_column + row - 1;
        CellRange::new(
          CellAddress {
            column: selected_column,
            row: start_row,
          },
          CellAddress {
            column: selected_column,
            row: start_row,
          },
        )
      } else {
        let selected_row = start_row + row - 1;
        CellRange::new(
          CellAddress {
            column: start_column,
            row: selected_row,
          },
          CellAddress {
            column: end_column,
            row: selected_row,
          },
        )
      }
    } else {
      CellRange::new(
        CellAddress {
          column: start_column + column - 1,
          row: start_row + row - 1,
        },
        CellAddress {
          column: start_column + column - 1,
          row: start_row + row - 1,
        },
      )
    };
    FormulaValue::Reference(QualifiedRange {
      sheet: reference.sheet,
      sheet_name: reference.sheet_name.clone(),
      range,
      start_flags: reference.start_flags,
      end_flags: reference.end_flags,
    })
  }

  pub(crate) fn evaluate_offset(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=5).contains(&args.len()) {
      return None;
    }
    let reference = self.as_reference(&self.evaluate(args.first()?)?)?;
    let row_offset = self.number(&self.evaluate(args.get(1)?)?)? as i64;
    let column_offset = self.number(&self.evaluate(args.get(2)?)?)? as i64;
    let height = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(|value| value as i64)
      .unwrap_or_else(|| i64::from(reference.range.end.row - reference.range.start.row + 1));
    let width = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(|value| value as i64)
      .unwrap_or_else(|| i64::from(reference.range.end.column - reference.range.start.column + 1));
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
      sheet_name: reference.sheet_name,
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

  pub(crate) fn evaluate_lookup(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let lookup = self.scalar_binary_operand(self.evaluate(args.first()?)?);
    if matches!(lookup, FormulaValue::Blank) {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    }
    let array_evaluator = self.with_array_context();
    let data = array_evaluator.evaluate(args.get(1)?)?;
    let data_matrix = self.matrix_values(&data);
    let result = if let Some(arg) = args.get(2) {
      Some(array_evaluator.evaluate(arg)?)
    } else {
      None
    };
    let result_matrix = result.as_ref().map(|value| self.matrix_values(value));
    let (data_vector, data_vertical) = lookup_vector(&data_matrix)?;
    let query = QueryEntry {
      op: QueryOp::LessOrEqual,
      field: 0,
      item: QueryItem {
        kind: query_value_kind(&lookup),
        value: lookup.clone(),
        source_text: None,
        match_empty: false,
        empty_matches_text: false,
      },
    };
    let param = QueryParam::single(query, QuerySearchType::Normal, true).with_range_lookup(true);
    let query = param.entries.first()?;
    let (search_vector, index_map) = lookup_search_vector_omitting_errors(&data_vector);
    let search_slice = search_vector.as_deref().unwrap_or(&data_vector);
    let Some(search_index) =
      lookup_binary_search(self, search_slice, query, &param, true, false, false)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    };
    let index = index_map
      .as_ref()
      .and_then(|indices| indices.get(search_index).copied())
      .unwrap_or(search_index);

    if let Some(FormulaValue::Reference(reference)) = result.as_ref() {
      let start_row = reference.range.start.row.min(reference.range.end.row);
      let end_row = reference.range.start.row.max(reference.range.end.row);
      let start_column = reference.range.start.column.min(reference.range.end.column);
      let end_column = reference.range.start.column.max(reference.range.end.column);
      if start_row != end_row && start_column != end_column {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      }
      let address = if start_row != end_row {
        let row = start_row.saturating_add(index as u32);
        if row > XLSX_MAX_ROW_ZERO_BASED {
          return Some(FormulaValue::Number(0.0));
        }
        CellAddress {
          column: start_column,
          row,
        }
      } else {
        let column = start_column.saturating_add(index as u32);
        if column > XLSX_MAX_COLUMN_ZERO_BASED {
          return Some(FormulaValue::Number(0.0));
        }
        CellAddress {
          column,
          row: start_row,
        }
      };
      return Some(self.book.cell_value(self.range_sheet(reference), address));
    }

    if let Some(result_matrix) = result_matrix {
      let rows = result_matrix.len();
      let columns = result_matrix.first().map_or(0, Vec::len);
      if rows == 1 && columns == 1 {
        if args
          .get(2)
          .is_some_and(|arg| matches!(arg, FormulaAst::Array(_)))
          && index != 0
        {
          return Some(FormulaValue::Error(FormulaErrorValue::NA));
        }
        return result_matrix
          .first()
          .and_then(|row| row.first())
          .cloned()
          .or(Some(FormulaValue::Error(FormulaErrorValue::NA)));
      }
      if rows > 1 && columns > 1 {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      }
      let result_vertical = result_matrix.len() >= result_matrix.first().map_or(0, Vec::len);
      let result_vector = lookup_vector_with_orientation(&result_matrix, result_vertical)?;
      return result_vector
        .get(index)
        .cloned()
        .or(Some(FormulaValue::Error(FormulaErrorValue::NA)));
    }

    if data_vertical {
      data_matrix
        .get(index)
        .and_then(|row| row.last())
        .cloned()
        .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
    } else {
      data_matrix
        .last()
        .and_then(|row| row.get(index))
        .cloned()
        .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
    }
  }

  pub(crate) fn evaluate_match(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let lookup = self.first_value(&self.evaluate(args.first()?)?);
    let data = self.evaluate(args.get(1)?)?;
    let (vector, index_map, data_vertical) = match &data {
      FormulaValue::Reference(reference) => self.lookup_reference_vector(reference)?,
      _ => {
        let matrix = self.matrix_values(&data);
        let (vector, data_vertical) = lookup_vector(&matrix)?;
        (vector, None, data_vertical)
      }
    };
    let mode = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as i32;
    let index = if index_map.is_some() && !data_vertical && matches!(mode, 1 | -1) {
      match_range_linear_index(self, &lookup, &vector, mode)
    } else {
      match mode {
        0 => search_vector(
          self,
          &lookup,
          &vector,
          QueryOp::Equal,
          LookupSearchMode::Forward,
          true,
        ),
        1 => search_vector(
          self,
          &lookup,
          &vector,
          QueryOp::LessOrEqual,
          LookupSearchMode::BinaryAscending,
          false,
        ),
        -1 => search_vector(
          self,
          &lookup,
          &vector,
          QueryOp::GreaterOrEqual,
          LookupSearchMode::BinaryDescending,
          false,
        ),
        _ => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
      }
    };
    let index = index.map(|index| {
      index_map
        .as_ref()
        .and_then(|indices| indices.get(index).copied())
        .unwrap_or(index)
    });
    index
      .map(|index| FormulaValue::Number(index as f64 + 1.0))
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  pub(crate) fn evaluate_xmatch(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=4).contains(&args.len()) {
      return None;
    }
    let lookup = self.scalar_value(self.evaluate(args.first()?)?);
    let data = self.with_array_context().evaluate(args.get(1)?)?;
    let matrix = self.matrix_values(&data);
    let (vector, _) = lookup_vector(&matrix)?;
    let match_mode = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0) as i32;
    let search_mode = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as i32;
    let search = LookupSearchMode::from_excel(search_mode)?;
    if matches!(match_mode, 2 | 3)
      && matches!(
        search,
        LookupSearchMode::BinaryAscending | LookupSearchMode::BinaryDescending
      )
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let index = match match_mode {
      0 => search_vector(self, &lookup, &vector, QueryOp::Equal, search, true),
      -1 => search_vector_with_type(
        self,
        &lookup,
        &vector,
        QueryOp::LessOrEqual,
        search,
        QuerySearchType::Normal,
        SearchVectorFlags::new(false, true),
      ),
      1 => search_vector_with_type(
        self,
        &lookup,
        &vector,
        QueryOp::GreaterOrEqual,
        search,
        QuerySearchType::Normal,
        SearchVectorFlags::new(false, true),
      ),
      2 | 3 => search_vector_with_type(
        self,
        &lookup,
        &vector,
        QueryOp::Equal,
        search,
        if !matches!(lookup, FormulaValue::String(_)) {
          QuerySearchType::Normal
        } else if match_mode == 2 && may_be_wildcard(self.text(&lookup).as_ref()) {
          QuerySearchType::Wildcard
        } else if match_mode == 3 && may_be_regex(self.text(&lookup).as_ref()) {
          QuerySearchType::Regex
        } else {
          QuerySearchType::Normal
        },
        SearchVectorFlags::new(true, false),
      ),
      _ => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    };
    index
      .map(|index| FormulaValue::Number(index as f64 + 1.0))
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  pub(crate) fn evaluate_vlookup(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=4).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let lookup = self.evaluate(args.first()?)?;
    let Some(result_column) = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let result_column = result_column.floor();
    if result_column < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let result_column = result_column as u32;
    let Some(table) = self.evaluate(args.get(1)?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let sorted = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    if self.array_context
      && matches!(
        lookup,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      )
    {
      return self.map_unary_values(lookup, |evaluator, lookup| {
        Some(evaluator.evaluate_vlookup_value(
          evaluator.scalar_value(lookup.clone()),
          &table,
          result_column,
          sorted,
        ))
      });
    }
    Some(self.evaluate_vlookup_value(self.scalar_value(lookup), &table, result_column, sorted))
  }

  pub(crate) fn evaluate_vlookup_value(
    &self,
    lookup: FormulaValue<'doc>,
    table: &FormulaValue<'doc>,
    result_column: u32,
    sorted: bool,
  ) -> FormulaValue<'doc> {
    if let FormulaValue::Matrix(rows) = table {
      let Some(result_index) = result_column.checked_sub(1).map(|value| value as usize) else {
        return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
      };
      if rows.first().is_none_or(|row| result_index >= row.len()) {
        return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
      }
      let index = vhlookup_matrix_index(self, &lookup, rows, false, sorted);
      let Some(index) = index else {
        return FormulaValue::Error(FormulaErrorValue::NA);
      };
      return rows
        .get(index)
        .and_then(|row| row.get(result_index))
        .cloned()
        .unwrap_or(FormulaValue::Error(FormulaErrorValue::Ref));
    }

    let Some(reference) = self.as_reference(table) else {
      return FormulaValue::Error(FormulaErrorValue::Value);
    };
    if reference.range.start.column + result_column - 1 > reference.range.end.column {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    let index = self.vlookup_reference_row_index(&lookup, &reference, sorted);
    index
      .map(|row| {
        let sheet = self.range_sheet(&reference);
        self.book.cell_value(
          sheet,
          CellAddress {
            column: reference.range.start.column + result_column - 1,
            row,
          },
        )
      })
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::NA))
  }

  pub(crate) fn vlookup_reference_row_index(
    &self,
    lookup: &FormulaValue<'doc>,
    reference: &QualifiedRange<'doc>,
    sorted: bool,
  ) -> Option<u32> {
    let sheet = self.range_sheet(reference);
    let start_row = reference.range.start.row.min(reference.range.end.row);
    let end_row = reference.range.start.row.max(reference.range.end.row);
    let search_column = reference.range.start.column.min(reference.range.end.column);
    let (mut query, search_type) = QueryEntry::from_value(self, lookup, 0);
    if sorted {
      query.op = QueryOp::LessOrEqual;
    }
    let param = QueryParam::single(query, search_type, self.book.formula_match_whole_cell)
      .with_range_lookup(sorted);
    let query = param.entries.first()?;
    let mut found = None;
    for row in start_row..=end_row {
      let address = CellAddress {
        column: search_column,
        row,
      };
      let value = self
        .book
        .query_cell_value(sheet, address, self.book.cell_value(sheet, address));
      let query_empty = self.book.is_query_empty_cell(sheet, address);
      if !query_matches(self, &param, query, &value, query_empty) {
        if sorted
          && found.is_some()
          && lookup_candidate_type_matches(query, &value)
          && lookup_compare_candidate_to_query(self, &value, query, &param, true) == Some(1)
        {
          break;
        }
        continue;
      }
      if sorted {
        if lookup_candidate_type_matches(query, &value)
          && found.is_none_or(|found_row| {
            let found_address = CellAddress {
              column: search_column,
              row: found_row,
            };
            let found_value = self.book.query_cell_value(
              sheet,
              found_address,
              self.book.cell_value(sheet, found_address),
            );
            lookup_compare_cells(self, &value, &found_value) >= 0
          })
        {
          found = Some(row);
        }
      } else {
        return Some(row);
      }
    }
    found
  }

  pub(crate) fn evaluate_hlookup(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=4).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let lookup = self.scalar_value(self.evaluate(args.first()?)?);
    let row_number = self.number(&self.evaluate(args.get(2)?)?)?.floor();
    if row_number < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let row_index = row_number as usize - 1;
    let sorted = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    let matrix = self.matrix_values(&self.evaluate(args.get(1)?)?);
    if row_index >= matrix.len() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let index = vhlookup_matrix_index(self, &lookup, &matrix, true, sorted);
    let Some(index) = index else {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    };
    matrix
      .get(row_index)
      .and_then(|row| row.get(index))
      .cloned()
      .or(Some(FormulaValue::Error(FormulaErrorValue::Ref)))
  }

  pub(crate) fn evaluate_xlookup(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 {
      return None;
    }
    let lookup = self.scalar_value(self.evaluate(args.first()?)?);
    let array_evaluator = self.with_array_context();
    let lookup_matrix = self.matrix_values(&array_evaluator.evaluate(args.get(1)?)?);
    let return_matrix = self.matrix_values(&array_evaluator.evaluate(args.get(2)?)?);
    let lookup_rows = lookup_matrix.len();
    let lookup_columns = lookup_matrix.first().map_or(0, Vec::len);
    if lookup_rows > 1 && lookup_columns > 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let (lookup_vector, lookup_vertical) = lookup_vector(&lookup_matrix)?;
    let return_rows = return_matrix.len();
    let return_columns = return_matrix.first().map_or(0, Vec::len);
    if return_rows == 0
      || return_columns == 0
      || return_matrix.iter().any(|row| row.len() != return_columns)
      || (lookup_vertical && return_rows != lookup_vector.len())
      || (!lookup_vertical && return_columns != lookup_vector.len())
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let not_found = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .filter(|value| !matches!(value, FormulaValue::Blank));
    let match_mode = args
      .get(4)
      .and_then(|arg| self.optional_number_value(arg))
      .unwrap_or(0.0) as i32;
    let search_mode = args
      .get(5)
      .and_then(|arg| self.optional_number_value(arg))
      .unwrap_or(1.0) as i32;
    let Some(search) = LookupSearchMode::from_excel(search_mode) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    if matches!(match_mode, 2 | 3)
      && matches!(
        search,
        LookupSearchMode::BinaryAscending | LookupSearchMode::BinaryDescending
      )
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let index = match match_mode {
      0 => search_vector_with_type(
        self,
        &lookup,
        &lookup_vector,
        QueryOp::Equal,
        search,
        QuerySearchType::Normal,
        SearchVectorFlags::new(true, false).with_first_exact(),
      ),
      -1 => search_vector_with_type(
        self,
        &lookup,
        &lookup_vector,
        QueryOp::Equal,
        search,
        QuerySearchType::Normal,
        SearchVectorFlags::new(true, false).with_first_exact(),
      )
      .or_else(|| {
        search_vector_with_type(
          self,
          &lookup,
          &lookup_vector,
          QueryOp::LessOrEqual,
          search,
          QuerySearchType::Normal,
          SearchVectorFlags::new(false, true),
        )
      }),
      1 => search_vector_with_type(
        self,
        &lookup,
        &lookup_vector,
        QueryOp::Equal,
        search,
        QuerySearchType::Normal,
        SearchVectorFlags::new(true, false).with_first_exact(),
      )
      .or_else(|| {
        search_vector_with_type(
          self,
          &lookup,
          &lookup_vector,
          QueryOp::GreaterOrEqual,
          search,
          QuerySearchType::Normal,
          SearchVectorFlags::new(false, true),
        )
      }),
      2 | 3 => search_vector_with_type(
        self,
        &lookup,
        &lookup_vector,
        QueryOp::Equal,
        search,
        if !matches!(lookup, FormulaValue::String(_)) {
          QuerySearchType::Normal
        } else if match_mode == 2 && may_be_wildcard(self.text(&lookup).as_ref()) {
          QuerySearchType::Wildcard
        } else if match_mode == 3 && may_be_regex(self.text(&lookup).as_ref()) {
          QuerySearchType::Regex
        } else {
          QuerySearchType::Normal
        },
        SearchVectorFlags::new(true, false),
      ),
      _ => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    };
    let Some(index) = index else {
      return not_found.or(Some(FormulaValue::Error(FormulaErrorValue::NA)));
    };
    if lookup_vertical {
      return_matrix
        .get(index)
        .and_then(|row| {
          if row.len() == 1 {
            row.first().cloned()
          } else {
            Some(FormulaValue::Matrix(vec![row.clone()]))
          }
        })
        .or(Some(FormulaValue::Error(FormulaErrorValue::Ref)))
    } else if return_matrix.len() == 1 {
      return_matrix
        .first()
        .and_then(|row| row.get(index))
        .cloned()
        .or(Some(FormulaValue::Error(FormulaErrorValue::Ref)))
    } else {
      let column = return_matrix
        .iter()
        .filter_map(|row| row.get(index).cloned())
        .map(|value| vec![value])
        .collect::<Vec<_>>();
      Some(FormulaValue::Matrix(column))
    }
  }

  pub(crate) fn evaluate_sheets(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if let Some(arg) = args.first() {
      if let Some(count) = self.sheets_count_from_ast(arg) {
        return Some(FormulaValue::Number(count as f64));
      }
      let Some(value) = self.evaluate(arg) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      return match value {
        FormulaValue::Reference(_) => Some(FormulaValue::Number(1.0)),
        FormulaValue::Matrix(_) => Some(FormulaValue::Error(FormulaErrorValue::Unknown)),
        _ => Some(FormulaValue::Error(FormulaErrorValue::Value)),
      };
    }
    Some(FormulaValue::Number(
      self.book.sheet_names.len().max(1) as f64
    ))
  }

  pub(crate) fn evaluate_sheet(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let sheet = if let Some(arg) = args.first() {
      let Some(value) = self.evaluate(arg) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      match value {
        FormulaValue::Reference(reference) => self.range_sheet(&reference),
        FormulaValue::String(name) => {
          let Some(sheet) = self.book.sheet_id_by_name(name.as_ref()) else {
            return Some(FormulaValue::Error(FormulaErrorValue::Value));
          };
          sheet
        }
        _ => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
      }
    } else {
      self.current_sheet
    };
    let index = self
      .book
      .sheet_names
      .iter()
      .position(|binding| binding.id == sheet)
      .map(|index| index + 1)
      .unwrap_or(sheet.0 as usize + 1);
    Some(FormulaValue::Number(index as f64))
  }

  pub(crate) fn sheets_count_from_ast(&self, arg: &FormulaAst<'doc>) -> Option<u32> {
    // Source: LibreOffice sc/source/core/tool/interpr1.cxx ScInterpreter::ScSheets.
    match arg {
      FormulaAst::Reference(reference) => Some(self.reference_sheet_count(reference)),
      FormulaAst::Name(name) => self.three_d_reference_sheet_count(name.as_ref()),
      _ => None,
    }
  }

  pub(crate) fn reference_sheet_count(&self, reference: &QualifiedRange<'doc>) -> u32 {
    let start = self.range_sheet(reference).0;
    let end = reference
      .sheet_name
      .as_ref()
      .and_then(|name| self.book.sheet_id_by_name(name.0.as_ref()))
      .map(|sheet| sheet.0)
      .unwrap_or(start);
    start.abs_diff(end) + 1
  }

  pub(crate) fn three_d_reference_sheet_count(&self, text: &str) -> Option<u32> {
    let (left, right) = text.split_once(':')?;
    let left = QualifiedAddress::parse_a1(self.current_sheet, left).ok()?;
    let right = QualifiedAddress::parse_a1(self.current_sheet, right).ok()?;
    let left_sheet = left
      .sheet_name
      .as_ref()
      .and_then(|name| self.book.sheet_id_by_name(name.0.as_ref()))?;
    let right_sheet = right
      .sheet_name
      .as_ref()
      .and_then(|name| self.book.sheet_id_by_name(name.0.as_ref()))?;
    Some(left_sheet.0.abs_diff(right_sheet.0) + 1)
  }

  pub(crate) fn evaluate_formula_text(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let reference = self.as_reference(&self.evaluate(args.first()?)?)?;
    let sheet = self.range_sheet(&reference);
    self
      .book
      .formula_text(sheet, reference.range.start)
      .map(|text| FormulaValue::String(Cow::Owned(text)))
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  pub(crate) fn evaluate_and(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let mut has_value = false;
    let mut result = true;
    for value in self.values(args) {
      if let FormulaValue::Error(error) = value {
        return Some(FormulaValue::Error(error));
      }
      if let Some(logical) = logical_value(&value) {
        has_value = true;
        result &= logical;
      }
    }
    Some(if has_value {
      FormulaValue::Boolean(result)
    } else {
      FormulaValue::Error(FormulaErrorValue::Value)
    })
  }

  pub(crate) fn evaluate_or(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let mut has_value = false;
    let mut result = false;
    for value in self.values(args) {
      if let FormulaValue::Error(error) = value {
        return Some(FormulaValue::Error(error));
      }
      if let Some(logical) = logical_value(&value) {
        has_value = true;
        result |= logical;
      }
    }
    Some(if has_value {
      FormulaValue::Boolean(result)
    } else {
      FormulaValue::Error(FormulaErrorValue::Value)
    })
  }

  pub(crate) fn evaluate_xor(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let mut has_value = false;
    let mut result = false;
    for value in self.values(args) {
      if let FormulaValue::Error(error) = value {
        return Some(FormulaValue::Error(error));
      }
      if let Some(logical) = logical_value(&value) {
        has_value = true;
        result ^= logical;
      }
    }
    Some(if has_value {
      FormulaValue::Boolean(result)
    } else {
      FormulaValue::Error(FormulaErrorValue::Value)
    })
  }

  pub(crate) fn evaluate_cell(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let info_type = self
      .text(&self.evaluate(args.first()?)?)
      .to_ascii_uppercase();
    let reference = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.as_reference(&value));
    let sheet = reference
      .as_ref()
      .map(|reference| self.range_sheet(reference))
      .unwrap_or(self.current_sheet);
    let address = reference
      .as_ref()
      .map(|reference| reference.range.start)
      .or(self.current_cell)
      .unwrap_or_default();
    match info_type.as_str() {
      "COL" => Some(FormulaValue::Number(address.column as f64 + 1.0)),
      "ROW" => Some(FormulaValue::Number(address.row as f64 + 1.0)),
      "SHEET" => self
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
        let file = self
          .book
          .source_file_name
          .as_deref()
          .unwrap_or("workbook.xlsx");
        let sheet_name = self
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
      "CONTENTS" => Some(self.book.cell_value(sheet, address)),
      "TYPE" => Some(FormulaValue::String(Cow::Borrowed(
        match self.book.cell_value(sheet, address) {
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

  pub(crate) fn evaluate_mid(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let text = self.text(&self.evaluate(args.first()?)?);
    let start = self.number(&self.evaluate(args.get(1)?)?)?;
    let len = self.number(&self.evaluate(args.get(2)?)?)?;
    if start < 1.0 || len < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let start = start as usize;
    let len = len as usize;
    Some(FormulaValue::String(Cow::Owned(
      text
        .chars()
        .skip(start.saturating_sub(1))
        .take(len)
        .collect(),
    )))
  }

  pub(crate) fn evaluate_midb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let text = self.text(&self.evaluate(args.first()?)?);
    let start = self.number(&self.evaluate(args.get(1)?)?)? as usize;
    let len = self.number(&self.evaluate(args.get(2)?)?)? as usize;
    if start == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let prefix = leftb(&text, start.saturating_sub(1));
    let suffix = text
      .chars()
      .skip(prefix.chars().count())
      .collect::<String>();
    Some(FormulaValue::String(Cow::Owned(leftb(&suffix, len))))
  }

  pub(crate) fn evaluate_left(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.evaluate(args.first()?)?;
    let len_value = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .unwrap_or(FormulaValue::Number(1.0));
    if self.array_context
      && (matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        || matches!(
          len_value,
          FormulaValue::Reference(_) | FormulaValue::Matrix(_)
        ))
    {
      return self.map_binary_values(value, len_value, |evaluator, value, len| {
        evaluator.left_value(value, len)
      });
    }
    self.left_value(&value, &len_value)
  }

  pub(crate) fn left_value(
    &self,
    value: &FormulaValue<'doc>,
    len: &FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let len = self.number(len)?;
    if len < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let len = len.floor() as usize;
    let text = self.text(&value);
    Some(FormulaValue::String(Cow::Owned(
      text.chars().take(len).collect(),
    )))
  }

  pub(crate) fn evaluate_leftb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.evaluate(args.first()?)?;
    let len_value = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .unwrap_or(FormulaValue::Number(1.0));
    if self.array_context
      && (matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        || matches!(
          len_value,
          FormulaValue::Reference(_) | FormulaValue::Matrix(_)
        ))
    {
      return self.map_binary_values(value, len_value, |evaluator, value, len| {
        evaluator.leftb_value(value, len)
      });
    }
    self.leftb_value(&value, &len_value)
  }

  pub(crate) fn leftb_value(
    &self,
    value: &FormulaValue<'doc>,
    len: &FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let len = self.number(len)?;
    if len < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let text = self.text(value);
    Some(FormulaValue::String(Cow::Owned(leftb(
      &text,
      len.floor() as usize,
    ))))
  }

  pub(crate) fn evaluate_right(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.evaluate(args.first()?)?;
    let len_value = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .unwrap_or(FormulaValue::Number(1.0));
    if self.array_context
      && (matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        || matches!(
          len_value,
          FormulaValue::Reference(_) | FormulaValue::Matrix(_)
        ))
    {
      return self.map_binary_values(value, len_value, |evaluator, value, len| {
        evaluator.right_value(value, len)
      });
    }
    self.right_value(&value, &len_value)
  }

  pub(crate) fn right_value(
    &self,
    value: &FormulaValue<'doc>,
    len: &FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let len = self.number(len)?;
    if len < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let len = len.floor() as usize;
    let text = self.text(value);
    Some(FormulaValue::String(Cow::Owned(
      text
        .chars()
        .rev()
        .take(len)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect(),
    )))
  }

  pub(crate) fn evaluate_rightb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.evaluate(args.first()?)?;
    let len_value = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .unwrap_or(FormulaValue::Number(1.0));
    if self.array_context
      && (matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        || matches!(
          len_value,
          FormulaValue::Reference(_) | FormulaValue::Matrix(_)
        ))
    {
      return self.map_binary_values(value, len_value, |evaluator, value, len| {
        evaluator.rightb_value(value, len)
      });
    }
    self.rightb_value(&value, &len_value)
  }

  pub(crate) fn rightb_value(
    &self,
    value: &FormulaValue<'doc>,
    len: &FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let len = self.number(len)?;
    if len < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let text = self.text(value);
    Some(FormulaValue::String(Cow::Owned(rightb(
      &text,
      len.floor() as usize,
    ))))
  }

  pub(crate) fn evaluate_roman(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=2).contains(&args.len()) {
      return None;
    }
    let value = self.number(&self.evaluate(args.first()?)?)?.floor() as i32;
    if !(0..=3999).contains(&value) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mode = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(approx_floor)
      .unwrap_or(0.0);
    if !(0.0..5.0).contains(&mode) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let output = roman_text_libreoffice(value as u16, mode as u16);
    Some(FormulaValue::String(Cow::Owned(output)))
  }

  pub(crate) fn evaluate_arabic(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return None;
    }
    let text = self
      .text(&self.evaluate(args.first()?)?)
      .to_ascii_uppercase();
    let mut previous = 0;
    let mut total = 0;
    for ch in text.chars().rev() {
      let value = match ch {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
      };
      if value < previous {
        total -= value;
      } else {
        total += value;
        previous = value;
      }
    }
    Some(FormulaValue::Number(total as f64))
  }

  pub(crate) fn evaluate_replaceb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 4 {
      return None;
    }
    let old_text = self.text(&self.evaluate(args.first()?)?);
    let start = self.number(&self.evaluate(args.get(1)?)?)? as i32;
    let count = self.number(&self.evaluate(args.get(2)?)?)? as i32;
    let new_text = self.text(&self.evaluate(args.get(3)?)?);
    let len = text_byte_len(&old_text) as i32;
    if start < 1 || start > len || count < 0 || start + count - 1 > len {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let left = leftb(&old_text, (start - 1) as usize);
    let right = rightb(&old_text, (len - start - count + 1).max(0) as usize);
    Some(FormulaValue::String(Cow::Owned(format!(
      "{left}{new_text}{right}"
    ))))
  }

  pub(crate) fn evaluate_hyperlink(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=2).contains(&args.len()) {
      return None;
    }
    let url_value = self.evaluate(args.first()?)?;
    let url = match self.scalar_value(url_value) {
      FormulaValue::Error(error) => {
        return Some(FormulaValue::Matrix(vec![
          vec![FormulaValue::Error(error)],
          vec![FormulaValue::Error(error)],
        ]));
      }
      value => self.text(&value),
    };
    let display = if let Some(display_arg) = args.get(1) {
      match self.scalar_value(self.evaluate(display_arg)?) {
        FormulaValue::Error(error) => FormulaValue::Error(error),
        FormulaValue::Number(value) => FormulaValue::Number(value),
        FormulaValue::String(value) => FormulaValue::String(value),
        FormulaValue::Boolean(value) => FormulaValue::Number(if value { 1.0 } else { 0.0 }),
        FormulaValue::Blank => FormulaValue::Number(0.0),
        value => FormulaValue::String(Cow::Owned(self.text(&value))),
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

  pub(crate) fn evaluate_to_row_column(
    &self,
    args: &[FormulaAst<'doc>],
    row_result: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.is_empty() || args.len() > 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let source = self.evaluate(args.first()?)?;
    let matrix = self.matrix_values(&source);
    let ignore = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0) as i32;
    let scan_by_column = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    let rows = matrix.len();
    let columns = matrix.iter().map(Vec::len).max().unwrap_or(0);
    if rows == 0 || columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }

    let mut values = Vec::with_capacity(rows * columns);
    if scan_by_column {
      for column in 0..columns {
        for row in &matrix {
          if let Some(value) = row.get(column)
            && !should_ignore_to_row_column_value(value, ignore)
          {
            values.push(value.clone());
          }
        }
      }
    } else {
      for row in &matrix {
        for value in row {
          if !should_ignore_to_row_column_value(value, ignore) {
            values.push(value.clone());
          }
        }
      }
    }

    if row_result {
      Some(FormulaValue::Matrix(vec![values]))
    } else {
      Some(FormulaValue::Matrix(
        values.into_iter().map(|value| vec![value]).collect(),
      ))
    }
  }

  pub(crate) fn evaluate_choose_rows(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(source) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let matrix = self.matrix_values(&source);
    let row_count = matrix.len();
    if row_count == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut rows = Vec::new();
    for arg in args.iter().skip(1) {
      let Some(value) = self.evaluate(arg) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      for index_value in self.matrix_values(&value).into_iter().flatten() {
        let Some(index) = self.number(&index_value).map(|value| value.trunc() as i64) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        let Some(row) = choose_row_column_index(index, row_count) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        rows.push(matrix.get(row)?.clone());
      }
    }
    Some(FormulaValue::Matrix(rows))
  }

  pub(crate) fn evaluate_choose_cols(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(source) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let matrix = self.matrix_values(&source);
    let column_count = matrix.first().map_or(0, Vec::len);
    if matrix.is_empty() || column_count == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut indexes = Vec::new();
    for arg in args.iter().skip(1) {
      let value = self.evaluate(arg)?;
      for index_value in self.matrix_values(&value).into_iter().flatten() {
        let Some(index) = self.number(&index_value).map(|value| value.trunc() as i64) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        let Some(index) = choose_row_column_index(index, column_count) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        indexes.push(index);
      }
    }
    let mut values = Vec::new();
    for row in &matrix {
      let mut out = Vec::new();
      for index in &indexes {
        out.push(row.get(*index).cloned().unwrap_or_default());
      }
      values.push(out);
    }
    Some(FormulaValue::Matrix(values))
  }

  pub(crate) fn evaluate_expand(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=4).contains(&args.len()) {
      return None;
    }
    let source = self.evaluate(args.first()?)?;
    let matrix = self.matrix_values(&source);
    let source_rows = matrix.len();
    let source_cols = matrix.first().map_or(0, Vec::len);
    if source_rows == 0 || source_cols == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let rows = match self.evaluate(args.get(1)?)? {
      FormulaValue::Blank => source_rows,
      value => self.number(&value)?.abs() as usize,
    };
    let cols = match args.get(2) {
      Some(arg) => match self.evaluate(arg)? {
        FormulaValue::Blank => source_cols,
        value => self.number(&value)?.abs() as usize,
      },
      None => source_cols,
    };
    if rows < source_rows || cols < source_cols {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let pad = match args.get(3) {
      Some(arg) => self.evaluate(arg)?,
      None => FormulaValue::Error(FormulaErrorValue::NA),
    };
    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut output_row = Vec::with_capacity(cols);
      for col in 0..cols {
        output_row.push(
          matrix
            .get(row)
            .and_then(|source_row| source_row.get(col))
            .cloned()
            .unwrap_or_else(|| pad.clone()),
        );
      }
      result.push(output_row);
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_stack(
    &self,
    args: &[FormulaAst<'doc>],
    horizontal: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    if args.iter().any(is_missing_argument) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let matrices = args
      .iter()
      .map(|arg| self.evaluate(arg).map(|value| self.matrix_values(&value)))
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

  pub(crate) fn evaluate_wrap(
    &self,
    args: &[FormulaAst<'doc>],
    by_columns: bool,
  ) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let matrix = self.matrix_values(&self.evaluate(args.first()?)?);
    let rows = matrix.len();
    let columns = matrix.first().map_or(0, Vec::len);
    if rows == 0 || columns == 0 || (rows > 1 && columns > 1) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let wrap = self.number_arg(args, 1)?.floor() as usize;
    if wrap == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let pad = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::NA));
    let values = matrix.into_iter().flatten().collect::<Vec<_>>();
    let outer = values.len().div_ceil(wrap);
    let result_rows = if by_columns { wrap } else { outer };
    let result_columns = if by_columns { outer } else { wrap };
    let mut result = vec![vec![pad; result_columns]; result_rows];
    for (index, value) in values.into_iter().enumerate() {
      let row = if by_columns {
        index % wrap
      } else {
        index / wrap
      };
      let column = if by_columns {
        index / wrap
      } else {
        index % wrap
      };
      result[row][column] = value;
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_filter(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let data = self.matrix_values(&self.evaluate(args.first()?)?);
    let include = self.matrix_values(&self.with_array_context().evaluate(args.get(1)?)?);
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
        if self.truthy(&include) {
          result.push(row);
        }
      }
    } else if include_values.len() == data.first()?.len() {
      let columns = include_values
        .iter()
        .enumerate()
        .filter_map(|(index, value)| self.truthy(value).then_some(index))
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
          .get(2)
          .and_then(|arg| self.evaluate(arg))
          .unwrap_or(FormulaValue::Error(FormulaErrorValue::Calc)),
      );
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_unique(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=3).contains(&args.len()) {
      return None;
    }
    let data = self.matrix_values(&self.evaluate(args.first()?)?);
    let by_col = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    let exactly_once = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    if data.is_empty() || data.first().is_none_or(Vec::is_empty) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut keys = Vec::<Vec<String>>::new();
    let mut counts = Vec::<usize>::new();
    let mut positions = Vec::<usize>::new();
    let outer = if by_col { data[0].len() } else { data.len() };
    for index in 0..outer {
      let key = if by_col {
        data
          .iter()
          .map(|row| {
            row
              .get(index)
              .map(display_text_from_value)
              .map(|text| text.to_lowercase())
              .unwrap_or_default()
          })
          .collect::<Vec<_>>()
      } else {
        data[index]
          .iter()
          .map(display_text_from_value)
          .map(|text| text.to_lowercase())
          .collect::<Vec<_>>()
      };
      if let Some(existing) = keys.iter().position(|item| *item == key) {
        counts[existing] += 1;
      } else {
        keys.push(key);
        counts.push(1);
        positions.push(index);
      }
    }
    let selected = positions
      .into_iter()
      .zip(counts)
      .filter_map(|(position, count)| (!exactly_once || count == 1).then_some(position))
      .collect::<Vec<_>>();
    if selected.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    }
    if by_col {
      Some(FormulaValue::Matrix(
        data
          .iter()
          .map(|row| selected.iter().map(|column| row[*column].clone()).collect())
          .collect(),
      ))
    } else {
      Some(FormulaValue::Matrix(
        selected.into_iter().map(|row| data[row].clone()).collect(),
      ))
    }
  }

  pub(crate) fn evaluate_transpose(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let matrix = self.matrix_values(&self.evaluate(args.first()?)?);
    let rows = matrix.len();
    let columns = matrix.iter().map(Vec::len).max().unwrap_or(0);
    if rows == 0 || columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut result = Vec::with_capacity(columns);
    for column in 0..columns {
      let mut row = Vec::with_capacity(rows);
      for source_row in &matrix {
        row.push(source_row.get(column).cloned().unwrap_or_default());
      }
      result.push(row);
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_sequence(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.is_empty() || args.len() > 4 {
      return None;
    }
    let rows = args
      .first()
      .and_then(|arg| self.optional_number_value(arg))
      .unwrap_or(1.0)
      .floor() as usize;
    let columns = args
      .get(1)
      .and_then(|arg| self.optional_number_value(arg))
      .unwrap_or(1.0)
      .floor() as usize;
    let start = args
      .get(2)
      .and_then(|arg| self.optional_number_value(arg))
      .unwrap_or(1.0);
    let step = args
      .get(3)
      .and_then(|arg| self.optional_number_value(arg))
      .unwrap_or(1.0);
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

  pub(crate) fn evaluate_randarray(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() > 5 {
      return None;
    }
    let rows = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0)
      .floor() as usize;
    let columns = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0)
      .floor() as usize;
    let mut min = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let mut max = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0);
    let whole = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    if whole {
      min = min.ceil();
      max = max.ceil();
    }
    if rows == 0 || columns == 0 || min > max {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut rng = XorShift64::new(time_seed());
    let mut result = Vec::with_capacity(rows);
    for _ in 0..rows {
      let mut row = Vec::with_capacity(columns);
      for _ in 0..columns {
        let mut value = min + rng.next_unit() * (max - min);
        if whole {
          value = value.floor();
        }
        row.push(FormulaValue::Number(value));
      }
      result.push(row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_take_drop(
    &self,
    args: &[FormulaAst<'doc>],
    take: bool,
  ) -> Option<FormulaValue<'doc>> {
    if !(1..=3).contains(&args.len()) {
      return None;
    }
    let matrix = self.matrix_values(&self.evaluate(args.first()?)?);
    let row_count = matrix.len();
    let col_count = matrix.first().map_or(0, Vec::len);
    if row_count == 0 || col_count == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let rows_arg = args
      .get(1)
      .and_then(|arg| self.optional_number_value(arg))
      .map(|value| value as isize);
    let cols_arg = args
      .get(2)
      .and_then(|arg| self.optional_number_value(arg))
      .map(|value| value as isize);
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

  pub(crate) fn evaluate_sort(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=4).contains(&args.len()) {
      return None;
    }
    let data = self.matrix_values(&self.evaluate(args.first()?)?);
    if data.is_empty() || data.first().is_none_or(Vec::is_empty) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut sort_indices = args
      .get(1)
      .and_then(|arg| self.optional_number_array_values(arg))
      .unwrap_or_else(|| vec![1.0]);
    let sort_orders = args
      .get(2)
      .and_then(|arg| self.optional_number_array_values(arg))
      .unwrap_or_else(|| vec![1.0]);
    let by_col = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .filter(|value| !matches!(value, FormulaValue::Blank))
      .map(|value| self.truthy(&value))
      .unwrap_or(false);
    if sort_indices.is_empty()
      || sort_orders.is_empty()
      || (sort_indices.len() != sort_orders.len() && sort_orders.len() > 1)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if sort_indices.iter().any(|value| *value < 1.0)
      || sort_orders
        .iter()
        .any(|value| !matches!(*value, 1.0 | -1.0))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    for sort_index in &mut sort_indices {
      *sort_index = sort_index.floor() - 1.0;
    }
    let sort_keys = sort_indices
      .iter()
      .enumerate()
      .map(|(index, sort_index)| {
        (
          *sort_index as usize,
          sort_orders
            .get(index)
            .or_else(|| sort_orders.first())
            .copied()
            .unwrap_or(1.0)
            == 1.0,
        )
      })
      .collect::<Vec<_>>();
    if by_col {
      if sort_keys.iter().any(|(key, _)| *key >= data.len()) {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      let keys = sort_keys
        .iter()
        .map(|(key, ascending)| (data[*key].clone(), *ascending))
        .collect::<Vec<_>>();
      let mut order = (0..data[0].len()).collect::<Vec<_>>();
      order.sort_by(|left, right| sort_multi_key_order(self, &keys, *left, *right));
      Some(FormulaValue::Matrix(reorder_columns(&data, &order)))
    } else {
      if sort_keys.iter().any(|(key, _)| *key >= data[0].len()) {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      let keys = sort_keys
        .iter()
        .map(|(key, ascending)| {
          (
            data
              .iter()
              .map(|row| row.get(*key).cloned().unwrap_or_default())
              .collect::<Vec<_>>(),
            *ascending,
          )
        })
        .collect::<Vec<_>>();
      let mut order = (0..data.len()).collect::<Vec<_>>();
      order.sort_by(|left, right| sort_multi_key_order(self, &keys, *left, *right));
      Some(FormulaValue::Matrix(
        order.into_iter().map(|row| data[row].clone()).collect(),
      ))
    }
  }

  pub(crate) fn evaluate_sortby(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 {
      return None;
    }
    let data = self.matrix_values(&self.evaluate(args.first()?)?);
    if data.is_empty() || data.first().is_none_or(Vec::is_empty) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut keys = Vec::new();
    let mut by_rows = None;
    let mut index = 1;
    while index < args.len() {
      let matrix = self.matrix_values(&self.evaluate(&args[index])?);
      let rows = matrix.len();
      let cols = matrix.first().map_or(0, Vec::len);
      if rows == 0 || cols == 0 {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      let current_by_rows = if cols == 1 && rows > 1 {
        true
      } else if rows == 1 && cols > 1 {
        false
      } else if rows == 1 && cols == 1 {
        return Some(FormulaValue::Matrix(data));
      } else {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      };
      if let Some(expected) = by_rows {
        if expected != current_by_rows {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        }
      } else {
        by_rows = Some(current_by_rows);
      }
      let ascending = if let Some(order_arg) = args.get(index + 1) {
        let order = self.number(&self.evaluate(order_arg)?)?;
        if !matches!(order, 1.0 | -1.0) {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        }
        order == 1.0
      } else {
        true
      };
      let values = if current_by_rows {
        matrix
          .into_iter()
          .map(|row| row.into_iter().next().unwrap_or_default())
          .collect::<Vec<_>>()
      } else {
        matrix.into_iter().next().unwrap_or_default()
      };
      keys.push((values, ascending));
      index += 2;
    }
    let by_rows = by_rows.unwrap_or(true);
    if by_rows {
      if keys.iter().any(|(values, _)| values.len() != data.len()) {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      let mut order = (0..data.len()).collect::<Vec<_>>();
      order.sort_by(|left, right| sort_multi_key_order(self, &keys, *left, *right));
      Some(FormulaValue::Matrix(
        order.into_iter().map(|row| data[row].clone()).collect(),
      ))
    } else {
      if keys.iter().any(|(values, _)| values.len() != data[0].len()) {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      let mut order = (0..data[0].len()).collect::<Vec<_>>();
      order.sort_by(|left, right| sort_multi_key_order(self, &keys, *left, *right));
      Some(FormulaValue::Matrix(reorder_columns(&data, &order)))
    }
  }

  pub(crate) fn evaluate_mmult(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let left = self.matrix_values(&self.evaluate(args.first()?)?);
    let right = self.matrix_values(&self.evaluate(args.get(1)?)?);
    let rows = left.len();
    let shared = left.first().map_or(0, Vec::len);
    let right_rows = right.len();
    let columns = right.first().map_or(0, Vec::len);
    if rows == 0
      || shared == 0
      || right_rows == 0
      || columns == 0
      || left.iter().any(|row| row.len() != shared)
      || right.iter().any(|row| row.len() != columns)
      || shared != right_rows
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let left_numbers = left
      .iter()
      .map(|row| {
        row
          .iter()
          .map(|value| self.number(value).unwrap_or(0.0))
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
    let right_numbers = right
      .iter()
      .map(|row| {
        row
          .iter()
          .map(|value| self.number(value).unwrap_or(0.0))
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
    let Some(result_numbers) = matrix_multiply(&left_numbers, &right_numbers) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let result = result_numbers
      .into_iter()
      .map(|row| {
        row
          .into_iter()
          .map(FormulaValue::Number)
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_sumx2(
    &self,
    args: &[FormulaAst<'doc>],
    plus: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let left = self.matrix_values(&self.evaluate(args.first()?)?);
    let right = self.matrix_values(&self.evaluate(args.get(1)?)?);
    if matrix_dimensions(&left) != matrix_dimensions(&right) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();
    for (left_row, right_row) in left.iter().zip(&right) {
      for (left_value, right_value) in left_row.iter().zip(right_row) {
        let Some(left_number) = matrix_stat_number(left_value) else {
          continue;
        };
        let Some(right_number) = matrix_stat_number(right_value) else {
          continue;
        };
        left_numbers.push(left_number);
        right_numbers.push(right_number);
      }
    }
    stats_sum_x2(&left_numbers, &right_numbers, plus).map(FormulaValue::Number)
  }

  pub(crate) fn evaluate_sumxmy2(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let left = self.matrix_values(&self.evaluate(args.first()?)?);
    let right = self.matrix_values(&self.evaluate(args.get(1)?)?);
    if matrix_dimensions(&left) != matrix_dimensions(&right) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();
    for (left_row, right_row) in left.iter().zip(&right) {
      for (left_value, right_value) in left_row.iter().zip(right_row) {
        let Some(left_number) = matrix_stat_number(left_value) else {
          continue;
        };
        let Some(right_number) = matrix_stat_number(right_value) else {
          continue;
        };
        left_numbers.push(left_number);
        right_numbers.push(right_number);
      }
    }
    stats_sum_xmy2(&left_numbers, &right_numbers).map(FormulaValue::Number)
  }

  pub(crate) fn evaluate_frequency(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let array_evaluator = self.with_array_context();
    let data = self.value_numbers(&array_evaluator.evaluate(args.first()?)?);
    let bins = self.value_numbers(&array_evaluator.evaluate(args.get(1)?)?);
    let Some(counts) = frequency_counts(data, &bins) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    Some(FormulaValue::Matrix(
      counts
        .into_iter()
        .map(|count| vec![FormulaValue::Number(count as f64)])
        .collect(),
    ))
  }

  pub(crate) fn evaluate_prob(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=4).contains(&args.len()) {
      return None;
    }
    let weights = self.matrix_values(&self.evaluate(args.first()?)?);
    let probabilities = self.matrix_values(&self.evaluate(args.get(1)?)?);
    let mut lower = self.number(&self.evaluate(args.get(2)?)?)?;
    let mut upper = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(lower);
    if lower > upper {
      std::mem::swap(&mut lower, &mut upper);
    }
    let shape = matrix_dimensions(&weights);
    if shape != matrix_dimensions(&probabilities) || shape.0 == 0 || shape.1 == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    }
    let mut sum = 0.0;
    let mut result = 0.0;
    for (weight_row, probability_row) in weights.iter().zip(&probabilities) {
      for (weight, probability) in weight_row.iter().zip(probability_row) {
        let Some(weight) = matrix_stat_number(weight) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        let Some(probability) = matrix_stat_number(probability) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        if !(0.0..=1.0).contains(&probability) {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        }
        sum += probability;
        if weight >= lower && weight <= upper {
          result += probability;
        }
      }
    }
    if (sum - 1.0).abs() > 1.0e-7 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    Some(FormulaValue::Number(result))
  }

  pub(crate) fn evaluate_mdeterm(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let matrix = self.matrix_values(&self.evaluate(args.first()?)?);
    let rows = matrix.len();
    let columns = matrix.first().map_or(0, Vec::len);
    if rows == 0 || rows != columns || matrix.iter().any(|row| row.len() != columns) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut values = Vec::with_capacity(rows);
    for row in matrix {
      let mut out = Vec::with_capacity(columns);
      for value in row {
        let Some(number) = matrix_stat_number(&value) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        out.push(number);
      }
      values.push(out);
    }
    Some(FormulaValue::Number(determinant(values)))
  }

  pub(crate) fn evaluate_minverse(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let matrix = self.matrix_values(&self.evaluate(args.first()?)?);
    let rows = matrix.len();
    let columns = matrix.first().map_or(0, Vec::len);
    if rows == 0 || rows != columns || matrix.iter().any(|row| row.len() != columns) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut values = vec![vec![0.0; columns]; rows];
    for row in 0..rows {
      for column in 0..columns {
        let Some(number) = matrix_stat_number(&matrix[row][column]) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        values[row][column] = number;
      }
    }
    let Some(decomposition) = lup_decompose(&mut values) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let mut inverse = vec![vec![0.0; columns]; rows];
    for column in 0..columns {
      let mut rhs = vec![0.0; rows];
      rhs[column] = 1.0;
      let Some(solution) = lup_solve(&values, &decomposition, &rhs) else {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      };
      for row in 0..rows {
        inverse[row][column] = solution[row];
      }
    }
    Some(FormulaValue::Matrix(
      inverse
        .into_iter()
        .map(|row| row.into_iter().map(FormulaValue::Number).collect())
        .collect(),
    ))
  }

  pub(crate) fn evaluate_munit(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(size) = self.number_arg(args, 0).map(approx_floor) else {
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

  pub(crate) fn evaluate_varpa(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = match self.stvar_text_as_zero_values(args) {
      Ok(values) => values,
      Err(error) => return Some(FormulaValue::Error(error)),
    };
    variance_slice(&values, false)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown)))
  }

  pub(crate) fn evaluate_vara(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = match self.stvar_text_as_zero_values(args) {
      Ok(values) => values,
      Err(error) => return Some(FormulaValue::Error(error)),
    };
    variance_slice(&values, true)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown)))
  }

  pub(crate) fn evaluate_mina(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let array_evaluator = self.with_array_context();
    array_evaluator
      .values(args)
      .map(|value| array_evaluator.number(&value).unwrap_or(0.0))
      .reduce(f64::min)
      .map(FormulaValue::Number)
  }

  pub(crate) fn evaluate_maxa(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let array_evaluator = self.with_array_context();
    array_evaluator
      .values(args)
      .map(|value| array_evaluator.number(&value).unwrap_or(0.0))
      .reduce(f64::max)
      .map(FormulaValue::Number)
  }

  pub(crate) fn evaluate_trimmean(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let values = self.value_numbers(&self.evaluate(args.first()?)?);
    let alpha = self.number(&self.evaluate(args.get(1)?)?)?;
    if !(0.0..1.0).contains(&alpha) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if values.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    trim_mean(values, alpha)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_large_small(
    &self,
    args: &[FormulaAst<'doc>],
    large: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let mut values = self.value_numbers(&value);
    if values.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let Some(rank_value) = args.get(1).and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    values.sort_by(f64::total_cmp);
    let rank_matrix = self.matrix_values(&rank_value);
    let rows = rank_matrix.len();
    let columns = rank_matrix.first().map_or(0, Vec::len);
    if rows == 0 || columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut result = Vec::with_capacity(rows);
    for row in rank_matrix {
      let mut result_row = Vec::with_capacity(columns);
      for rank in row {
        let value = match rank {
          FormulaValue::Error(error) => FormulaValue::Error(error),
          value => {
            let Some(k) = self.number(&value).map(|value| value.floor() as usize) else {
              return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
            };
            if k == 0 || k > values.len() {
              FormulaValue::Error(FormulaErrorValue::IllegalArgument)
            } else {
              let index = if large { values.len() - k } else { k - 1 };
              FormulaValue::Number(values[index])
            }
          }
        };
        result_row.push(value);
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_stdeva(
    &self,
    args: &[FormulaAst<'doc>],
    sample: bool,
  ) -> Option<FormulaValue<'doc>> {
    let values = match self.stvar_text_as_zero_values(args) {
      Ok(values) => values,
      Err(error) => return Some(FormulaValue::Error(error)),
    };
    variance_slice(&values, sample)
      .map(|value| FormulaValue::Number(value.sqrt()))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Div0)))
  }

  pub(crate) fn stvar_text_as_zero_values(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> std::result::Result<Vec<f64>, FormulaErrorValue> {
    let mut values = Vec::new();
    for arg in args {
      let ranges = self.reference_ranges_from_ast(arg);
      if !ranges.is_empty() {
        for range in ranges {
          self.push_stvar_range_values(&range, &mut values)?;
        }
        continue;
      }
      match self.evaluate(arg).ok_or(FormulaErrorValue::Unknown)? {
        FormulaValue::Reference(reference) => {
          self.push_stvar_range_values(&reference, &mut values)?
        }
        FormulaValue::RefList(ranges) => {
          for range in ranges {
            self.push_stvar_range_values(&range, &mut values)?;
          }
        }
        FormulaValue::Matrix(rows) => {
          for value in rows.into_iter().flatten() {
            self.push_stvar_matrix_value(value, &mut values)?;
          }
        }
        value => self.push_stvar_direct_value(value, &mut values)?,
      }
    }
    Ok(values)
  }

  pub(crate) fn push_stvar_range_values(
    &self,
    reference: &QualifiedRange<'doc>,
    values: &mut Vec<f64>,
  ) -> std::result::Result<(), FormulaErrorValue> {
    for value in self.range_values(reference) {
      match value {
        FormulaValue::Number(value) => values.push(value),
        FormulaValue::Boolean(value) => values.push(if value { 1.0 } else { 0.0 }),
        FormulaValue::String(_) => values.push(0.0),
        FormulaValue::Error(error) => return Err(error),
        FormulaValue::Blank
        | FormulaValue::Matrix(_)
        | FormulaValue::Reference(_)
        | FormulaValue::RefList(_) => {}
      }
    }
    Ok(())
  }

  pub(crate) fn push_stvar_matrix_value(
    &self,
    value: FormulaValue<'doc>,
    values: &mut Vec<f64>,
  ) -> std::result::Result<(), FormulaErrorValue> {
    match value {
      FormulaValue::Number(value) => values.push(value),
      FormulaValue::Boolean(value) => values.push(if value { 1.0 } else { 0.0 }),
      FormulaValue::String(_) | FormulaValue::Blank => values.push(0.0),
      FormulaValue::Error(error) => return Err(error),
      FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => {}
    }
    Ok(())
  }

  pub(crate) fn push_stvar_direct_value(
    &self,
    value: FormulaValue<'doc>,
    values: &mut Vec<f64>,
  ) -> std::result::Result<(), FormulaErrorValue> {
    match value {
      FormulaValue::Number(value) => values.push(value),
      FormulaValue::Boolean(value) => values.push(if value { 1.0 } else { 0.0 }),
      FormulaValue::String(_) => values.push(0.0),
      FormulaValue::Blank => values.push(0.0),
      FormulaValue::Error(error) => return Err(error),
      FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => {}
    }
    Ok(())
  }

  pub(crate) fn evaluate_devsq(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = match self.numeric_aggregate(args, false) {
      Ok(aggregate) => aggregate.values,
      Err(error) => return Some(FormulaValue::Error(error)),
    };
    if values.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    deviation_sum_squares(&values).map(FormulaValue::Number)
  }

  pub(crate) fn evaluate_avedev(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = self.numeric_args(args);
    if values.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    Some(FormulaValue::Number(
      values.iter().map(|value| (value - mean).abs()).sum::<f64>() / values.len() as f64,
    ))
  }

  pub(crate) fn evaluate_averagea(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = self
      .values(args)
      .map(|value| self.number(&value).unwrap_or(0.0))
      .collect::<Vec<_>>();
    if values.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    Some(FormulaValue::Number(
      values.iter().sum::<f64>() / values.len() as f64,
    ))
  }

  pub(crate) fn evaluate_gcd_lcm(
    &self,
    args: &[FormulaAst<'doc>],
    lcm: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let mut result = if lcm { 1.0 } else { 0.0 };
    let values = self.numeric_values(args).collect::<Vec<_>>();
    if values.iter().any(|value| *value < 0.0) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    for value in values {
      let value = approx_floor(value);
      result = if lcm {
        lcm_number(result, value)
      } else {
        gcd_number(result, value)
      };
    }
    Some(FormulaValue::Number(result))
  }

  pub(crate) fn evaluate_fact(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(value) = self.number_arg(args, 0).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if value < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number((log_gamma(value + 1.0)).exp().round()))
  }

  pub(crate) fn evaluate_fact_double(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(value) = self.number_arg(args, 0).map(f64::trunc) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if !(0.0..=300.0).contains(&value) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut result = 1.0;
    let mut current = value as u64;
    while current > 1 {
      result *= current as f64;
      current = current.saturating_sub(2);
    }
    Some(FormulaValue::Number(result))
  }

  pub(crate) fn evaluate_multinomial(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let values = self.numeric_values(args).collect::<Vec<_>>();
    if values.iter().any(|value| *value < 0.0) {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let sum = values.iter().map(|value| approx_floor(*value)).sum::<f64>();
    let denominator = values
      .iter()
      .map(|value| log_gamma(approx_floor(*value) + 1.0))
      .sum::<f64>();
    Some(FormulaValue::Number(
      (log_gamma(sum + 1.0) - denominator).exp(),
    ))
  }

  pub(crate) fn evaluate_combin(
    &self,
    args: &[FormulaAst<'doc>],
    repetition: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(count) = self.number_arg(args, 0).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(chosen) = self.number_arg(args, 1).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    match combination_count(count, chosen, repetition, log_gamma) {
      Some(value) => Some(FormulaValue::Number(value)),
      None => Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    }
  }

  pub(crate) fn evaluate_permut(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let count = self.number(&self.evaluate(args.first()?)?)?.floor();
    let chosen = self.number(&self.evaluate(args.get(1)?)?)?.floor();
    match permutation_count(count, chosen, log_gamma) {
      Some(value) => Some(FormulaValue::Number(value)),
      None => Some(FormulaValue::Error(FormulaErrorValue::Num)),
    }
  }

  pub(crate) fn evaluate_permutationa(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let left = self.evaluate(args.first()?)?;
    let right = self.evaluate(args.get(1)?)?;
    if matches!(left, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
      || matches!(right, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
    {
      return self.map_binary_values(left, right, |evaluator, left, right| {
        Some(permutationa_value(
          evaluator.number(left)?,
          evaluator.number(right)?,
        ))
      });
    }
    Some(permutationa_value(
      self.number(&left)?,
      self.number(&right)?,
    ))
  }

  pub(crate) fn evaluate_mround(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 || args.iter().any(is_missing_argument) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(number) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(multiple) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    Some(FormulaValue::Number(mround(number, multiple)))
  }

  pub(crate) fn evaluate_quotient(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let numerator = self.number(&self.evaluate(args.first()?)?)?;
    let denominator = self.number(&self.evaluate(args.get(1)?)?)?;
    match quotient(numerator, denominator) {
      Ok(result) => Some(FormulaValue::Number(result)),
      Err(error) => Some(FormulaValue::Error(numeric_error_value(error))),
    }
  }

  pub(crate) fn evaluate_pmt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=5).contains(&args.len()) {
      return None;
    }
    let rate = self.number(&self.evaluate(args.first()?)?)?;
    let nper = self.number(&self.evaluate(args.get(1)?)?)?;
    let pv = self.number(&self.evaluate(args.get(2)?)?)?;
    let fv = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let pay_in_advance = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    let result = financial_pmt(rate, nper, pv, fv, pay_in_advance);
    if result.is_finite() {
      Some(FormulaValue::Number(result))
    } else {
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    }
  }

  pub(crate) fn evaluate_fv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=5).contains(&args.len()) {
      return None;
    }
    let rate = self.number(&self.evaluate(args.first()?)?)?;
    let nper = self.number(&self.evaluate(args.get(1)?)?)?;
    let payment = self.number(&self.evaluate(args.get(2)?)?)?;
    let pv = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let pay_in_advance = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    Some(FormulaValue::Number(financial_fv(
      rate,
      nper,
      payment,
      pv,
      pay_in_advance,
    )))
  }

  pub(crate) fn evaluate_npv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(rate) = self.npv_scalar_number(&self.evaluate(args.first()?)?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let mut period = 1i32;
    let mut result = 0.0;
    for arg in &args[1..] {
      let values = match self.npv_values_from_ast(arg) {
        Ok(values) => values,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
      for number in values {
        result += number / (1.0 + rate).powi(period);
        period += 1;
      }
    }
    Some(FormulaValue::Number(result))
  }

  pub(crate) fn npv_values_from_ast(
    &self,
    ast: &FormulaAst<'doc>,
  ) -> std::result::Result<Vec<f64>, FormulaErrorValue> {
    let ranges = self.reference_ranges_from_ast(ast);
    if !ranges.is_empty() {
      let mut values = Vec::new();
      for range in ranges {
        values.extend(self.horizontal_range_numbers(&range)?);
      }
      return Ok(values);
    }
    match self.evaluate(ast).ok_or(FormulaErrorValue::Unknown)? {
      FormulaValue::Reference(reference) => self.horizontal_range_numbers(&reference),
      FormulaValue::RefList(ranges) => {
        let mut values = Vec::new();
        for range in ranges {
          values.extend(self.horizontal_range_numbers(&range)?);
        }
        Ok(values)
      }
      FormulaValue::Matrix(rows) => self.npv_matrix_numbers(&rows),
      value => self
        .npv_scalar_number(&value)
        .map(|value| vec![value])
        .ok_or(FormulaErrorValue::IllegalArgument),
    }
  }

  pub(crate) fn horizontal_range_numbers(
    &self,
    reference: &QualifiedRange<'doc>,
  ) -> std::result::Result<Vec<f64>, FormulaErrorValue> {
    let mut values = Vec::new();
    for (_address, value) in self.range_cells(reference) {
      match value {
        FormulaValue::Number(value) => values.push(value),
        FormulaValue::Boolean(value) => values.push(if value { 1.0 } else { 0.0 }),
        FormulaValue::Error(error) => return Err(error),
        FormulaValue::String(_)
        | FormulaValue::Blank
        | FormulaValue::Matrix(_)
        | FormulaValue::Reference(_)
        | FormulaValue::RefList(_) => {}
      }
    }
    Ok(values)
  }

  pub(crate) fn npv_matrix_numbers(
    &self,
    rows: &[Vec<FormulaValue<'doc>>],
  ) -> std::result::Result<Vec<f64>, FormulaErrorValue> {
    let columns = rows.iter().map(Vec::len).max().unwrap_or(0);
    if rows.is_empty() || columns == 0 {
      return Err(FormulaErrorValue::IllegalArgument);
    }
    let mut values = Vec::new();
    for column in 0..columns {
      for row in rows {
        let Some(value) = row.get(column) else {
          return Err(FormulaErrorValue::IllegalArgument);
        };
        let Some(number) = self.npv_scalar_number(value) else {
          return Err(match value {
            FormulaValue::Error(error) => *error,
            _ => FormulaErrorValue::IllegalArgument,
          });
        };
        values.push(number);
      }
    }
    Ok(values)
  }

  pub(crate) fn npv_scalar_number(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    match self.first_value(value) {
      FormulaValue::Number(value) => Some(value),
      FormulaValue::Boolean(value) => Some(if value { 1.0 } else { 0.0 }),
      _ => None,
    }
  }

  pub(crate) fn evaluate_fvschedule(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    if args.iter().any(is_missing_argument) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let mut value = self.number(&self.evaluate(args.first()?)?)?;
    let schedule = self.evaluate(args.get(1)?)?;
    if matches!(schedule, FormulaValue::Blank) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    for rate in self.value_numbers(&schedule) {
      value *= 1.0 + rate;
    }
    Some(FormulaValue::Number(value))
  }

  pub(crate) fn evaluate_effect(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let nominal = self.number(&self.evaluate(args.first()?)?)?;
    let periods = self.number(&self.evaluate(args.get(1)?)?)?.floor();
    if nominal <= 0.0 || periods < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(
      (1.0 + nominal / periods).powf(periods) - 1.0,
    ))
  }

  pub(crate) fn evaluate_rate(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=6).contains(&args.len()) {
      return None;
    }
    let nper = self.number(&self.evaluate(args.first()?)?)?;
    let payment = self.number(&self.evaluate(args.get(1)?)?)?;
    let pv = self.number(&self.evaluate(args.get(2)?)?)?;
    let fv = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let pay_type = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    let guess = args
      .get(5)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.1);
    if nper <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    match financial_rate(nper, payment, pv, fv, pay_type, guess, args.len() != 6) {
      Some(value) => Some(FormulaValue::Number(value)),
      None => Some(FormulaValue::Error(FormulaErrorValue::Num)),
    }
  }

  pub(crate) fn evaluate_ispmt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 4 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let rate = self.number(&self.evaluate(args.first()?)?)?;
    let period = self.number(&self.evaluate(args.get(1)?)?)?;
    let total = self.number(&self.evaluate(args.get(2)?)?)?;
    let investment = self.number(&self.evaluate(args.get(3)?)?)?;
    let result = investment * rate * (period / total - 1.0);
    if result.is_finite() {
      Some(FormulaValue::Number(result))
    } else {
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    }
  }

  pub(crate) fn evaluate_ipmt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=6).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(rate) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(period) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(nper) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(pv) = self.number_arg(args, 3) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let fv = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let pay_in_advance = args
      .get(5)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    if period < 1.0 || period > nper {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let (interest, _) = financial_ipmt(rate, period, nper, pv, fv, pay_in_advance);
    Some(FormulaValue::Number(interest))
  }

  pub(crate) fn evaluate_ppmt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=6).contains(&args.len()) {
      return None;
    }
    let rate = self.number(&self.evaluate(args.first()?)?)?;
    let period = self.number(&self.evaluate(args.get(1)?)?)?;
    let nper = self.number(&self.evaluate(args.get(2)?)?)?;
    let pv = self.number(&self.evaluate(args.get(3)?)?)?;
    let fv = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let pay_in_advance = args
      .get(5)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    if period < 1.0 || period > nper {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let (interest, payment) = financial_ipmt(rate, period, nper, pv, fv, pay_in_advance);
    Some(FormulaValue::Number(payment - interest))
  }

  pub(crate) fn evaluate_cumipmt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    self.evaluate_cum_interest_principal(args, true)
  }

  pub(crate) fn evaluate_cumprinc(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    self.evaluate_cum_interest_principal(args, false)
  }

  pub(crate) fn evaluate_cum_interest_principal(
    &self,
    args: &[FormulaAst<'doc>],
    interest: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 6 {
      return None;
    }
    if args.iter().any(is_missing_argument) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let rate = self.number(&self.evaluate(args.first()?)?)?;
    let nper = self.number(&self.evaluate(args.get(1)?)?)?;
    let pv = self.number(&self.evaluate(args.get(2)?)?)?;
    let start = approx_floor(self.number(&self.evaluate(args.get(3)?)?)?);
    let end = approx_floor(self.number(&self.evaluate(args.get(4)?)?)?);
    let flag = self.number(&self.evaluate(args.get(5)?)?)?;
    if start < 1.0
      || end < start
      || rate <= 0.0
      || end > nper
      || nper <= 0.0
      || pv <= 0.0
      || (flag != 0.0 && flag != 1.0)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(financial_cum(
      rate,
      nper,
      pv,
      start as u64,
      end as u64,
      flag != 0.0,
      interest,
    )))
  }

  pub(crate) fn evaluate_xnpv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let rate = self.number(&self.evaluate(args.first()?)?)?;
    let values = self.value_numbers(&self.evaluate(args.get(1)?)?);
    let dates = self.value_numbers(&self.evaluate(args.get(2)?)?);
    financial_xnpv(rate, &values, &dates)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_xirr(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let values = self.value_numbers(&self.evaluate(args.first()?)?);
    let dates = self.value_numbers(&self.evaluate(args.get(1)?)?);
    let guess = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.1);
    financial_xirr(&values, &dates, guess)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_irr(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=2).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let values = self.value_numbers_from_ast(args.first()?);
    let guess = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.1);
    financial_irr(&values, guess)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Num)))
  }

  pub(crate) fn evaluate_mirr(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let values = self.value_numbers(&self.evaluate(args.first()?)?);
    let finance_rate = self.number(&self.evaluate(args.get(1)?)?)?;
    let reinvest_rate = self.number(&self.evaluate(args.get(2)?)?)?;
    financial_mirr(&values, finance_rate, reinvest_rate)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_euroconvert(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if !(3..=5).contains(&args.len()) {
      return None;
    }
    if args.get(4).is_some_and(is_missing_argument)
      || (args.len() == 4 && args.get(3).is_some_and(is_missing_argument))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let from = self.text(&self.evaluate(args.get(1)?)?);
    let to = self.text(&self.evaluate(args.get(2)?)?);
    let full_precision = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    let precision = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(approx_floor)
      .unwrap_or(0.0);
    if precision != 0.0 && precision < 3.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    euro_convert(value, &from, &to, full_precision, precision)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_bahttext(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    if self.array_context && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
      return self.map_unary_values(value, |evaluator, value| {
        evaluator
          .number(value)
          .map(|value| FormulaValue::String(Cow::Owned(baht_text(value))))
          .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown)))
      });
    }
    let Some(value) = self.number(&value) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    Some(FormulaValue::String(Cow::Owned(baht_text(value))))
  }

  pub(crate) fn evaluate_regex(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=4).contains(&args.len()) {
      return None;
    }
    let text = self.text(&self.evaluate(args.first()?)?);
    let pattern = self.text(&self.evaluate(args.get(1)?)?);
    let replacement = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.text(&value));
    let mut global = false;
    let mut occurrence = 1usize;
    if let Some(arg) = args.get(3) {
      let value = self.evaluate(arg)?;
      match value {
        FormulaValue::String(flags) => {
          if flags.as_ref() == "g" {
            global = true;
          } else if !flags.is_empty() {
            return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
          }
        }
        value => {
          let number = self.number(&value)?;
          if number < 0.0 {
            return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
          }
          occurrence = number.floor() as usize;
        }
      }
    }
    if occurrence == 0 {
      return Some(FormulaValue::String(Cow::Owned(text)));
    }
    let regex = match RegexBuilder::new(&pattern).build() {
      Ok(regex) => regex,
      Err(_) => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    };
    if let Some(replacement) = replacement {
      if global {
        return Some(FormulaValue::String(Cow::Owned(
          regex.replace_all(&text, replacement.as_str()).into_owned(),
        )));
      }
      if occurrence == 1 {
        return Some(FormulaValue::String(Cow::Owned(
          regex.replace(&text, replacement.as_str()).into_owned(),
        )));
      }
      let mut count = 0usize;
      let result = regex
        .replace_all(&text, |captures: &regex::Captures<'_>| {
          count += 1;
          if count == occurrence {
            replacement.clone()
          } else {
            captures
              .get(0)
              .map(|mat| mat.as_str().to_string())
              .unwrap_or_default()
          }
        })
        .into_owned();
      return Some(FormulaValue::String(Cow::Owned(result)));
    }
    regex
      .find_iter(&text)
      .nth(occurrence - 1)
      .map(|mat| FormulaValue::String(Cow::Owned(mat.as_str().to_string())))
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  pub(crate) fn evaluate_encodeurl(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let text = self.text(&self.evaluate(args.first()?)?);
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

  pub(crate) fn evaluate_rot13(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let text = self.text(&self.evaluate(args.first()?)?);
    let result = text
      .chars()
      .map(|ch| match ch {
        'a'..='z' => ((((ch as u8 - b'a') + 13) % 26) + b'a') as char,
        'A'..='Z' => ((((ch as u8 - b'A') + 13) % 26) + b'A') as char,
        _ => ch,
      })
      .collect();
    Some(FormulaValue::String(Cow::Owned(result)))
  }

  pub(crate) fn evaluate_nominal(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let effective = self.number(&self.evaluate(args.first()?)?)?;
    let periods = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    if periods < 1.0 || effective <= 0.0 {
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    } else {
      Some(FormulaValue::Number(
        ((effective + 1.0).powf(1.0 / periods) - 1.0) * periods,
      ))
    }
  }

  pub(crate) fn evaluate_sln(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let cost = self.number(&self.evaluate(args.first()?)?)?;
    let salvage = self.number(&self.evaluate(args.get(1)?)?)?;
    let life = self.number(&self.evaluate(args.get(2)?)?)?;
    if life == 0.0 {
      Some(FormulaValue::Error(FormulaErrorValue::Div0))
    } else {
      Some(FormulaValue::Number((cost - salvage) / life))
    }
  }

  pub(crate) fn evaluate_syd(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 4 {
      return None;
    }
    let cost = self.number(&self.evaluate(args.first()?)?)?;
    let salvage = self.number(&self.evaluate(args.get(1)?)?)?;
    let life = self.number(&self.evaluate(args.get(2)?)?)?;
    let period = self.number(&self.evaluate(args.get(3)?)?)?;
    if life <= 0.0 || period <= 0.0 || period > life + 1.0 {
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    } else {
      Some(FormulaValue::Number(
        (cost - salvage) * (life - period + 1.0) * 2.0 / (life * (life + 1.0)),
      ))
    }
  }

  pub(crate) fn evaluate_ddb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=5).contains(&args.len()) {
      return None;
    }
    let cost = self.number(&self.evaluate(args.first()?)?)?;
    let salvage = self.number(&self.evaluate(args.get(1)?)?)?;
    let life = self.number(&self.evaluate(args.get(2)?)?)?;
    let period = self.number(&self.evaluate(args.get(3)?)?)?;
    let factor = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(2.0);
    if cost < 0.0
      || salvage < 0.0
      || factor <= 0.0
      || salvage > cost
      || period < 1.0
      || period > life
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(financial_ddb(
      cost, salvage, life, period, factor,
    )))
  }

  pub(crate) fn evaluate_vdb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(5..=7).contains(&args.len()) {
      return None;
    }
    let cost = self.number(&self.evaluate(args.first()?)?)?;
    let salvage = self.number(&self.evaluate(args.get(1)?)?)?;
    let life = self.number(&self.evaluate(args.get(2)?)?)?;
    let start = self.number(&self.evaluate(args.get(3)?)?)?;
    let end = self.number(&self.evaluate(args.get(4)?)?)?;
    let factor = args
      .get(5)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(2.0);
    let no_switch = args
      .get(6)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    if start < 0.0 || end < start || end > life || cost < 0.0 || salvage > cost || factor <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(financial_vdb(
      cost, salvage, life, start, end, factor, no_switch,
    )))
  }

  pub(crate) fn evaluate_skew(
    &self,
    args: &[FormulaAst<'doc>],
    population: bool,
  ) -> Option<FormulaValue<'doc>> {
    let values = self.numeric_args(args);
    match skewness(&values, population) {
      Ok(value) => Some(FormulaValue::Number(value)),
      Err(error) => Some(FormulaValue::Error(statistics_error_value(error))),
    }
  }

  pub(crate) fn evaluate_geo_har_mean(
    &self,
    args: &[FormulaAst<'doc>],
    harmonic: bool,
  ) -> Option<FormulaValue<'doc>> {
    let mut count = 0.0;
    let mut total = 0.0;
    for value in self.numeric_values(args) {
      if value < 0.0 || (harmonic && value == 0.0) {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      if value == 0.0 {
        return Some(FormulaValue::Number(0.0));
      }
      count += 1.0;
      total += if harmonic { value.recip() } else { value.ln() };
    }
    if count == 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    if harmonic {
      Some(FormulaValue::Number(count / total))
    } else {
      Some(FormulaValue::Number((total / count).exp()))
    }
  }

  pub(crate) fn evaluate_sumif(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    if let Some(sum_range) = args.get(2) {
      self.evaluate_ifs(Some(sum_range), &args[..2], IfsAggregate::Sum)
    } else {
      self.evaluate_ifs(Some(&args[0]), &args[..2], IfsAggregate::Sum)
    }
  }

  pub(crate) fn evaluate_countif(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    if let Some(value) = self.evaluate_countif_ref_list(args) {
      return Some(value);
    }
    self.evaluate_ifs(None, args, IfsAggregate::Count)
  }

  pub(crate) fn evaluate_countif_ref_list(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let ranges = self.reference_ranges_from_ast(args.first()?);
    if ranges.is_empty() {
      return None;
    }
    let criterion = QueryParam::from_criterion(self, &self.evaluate(args.get(1)?)?, 0);
    let mut count = 0.0;
    for range in ranges {
      let sheet = self.range_sheet(&range);
      let range = if criterion
        .entries
        .first()
        .is_some_and(|entry| !entry.item.match_empty)
      {
        self
          .book
          .data_area_subrange(sheet, range.range)
          .map(|data_area| QualifiedRange {
            range: data_area,
            ..range.clone()
          })
          .unwrap_or(range)
      } else {
        range
      };
      for (address, value) in self.range_cells(&range) {
        let value = self.book.query_cell_value(sheet, address, value);
        if criterion.matches_value(self, &value, self.book.is_query_empty_cell(sheet, address)) {
          count += 1.0;
        }
      }
    }
    Some(FormulaValue::Number(count))
  }

  pub(crate) fn evaluate_averageif(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    if let Some(average_range) = args.get(2) {
      self.evaluate_ifs(Some(average_range), &args[..2], IfsAggregate::Average)
    } else {
      self.evaluate_ifs(Some(&args[0]), &args[..2], IfsAggregate::Average)
    }
  }

  pub(crate) fn evaluate_sumifs(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 || args.len().is_multiple_of(2) {
      return None;
    }
    self.evaluate_ifs(Some(&args[0]), &args[1..], IfsAggregate::Sum)
  }

  pub(crate) fn evaluate_countifs(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 || !args.len().is_multiple_of(2) {
      return None;
    }
    self.evaluate_ifs(None, args, IfsAggregate::Count)
  }

  pub(crate) fn evaluate_averageifs(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 || args.len().is_multiple_of(2) {
      return None;
    }
    self.evaluate_ifs(Some(&args[0]), &args[1..], IfsAggregate::Average)
  }

  pub(crate) fn evaluate_minmaxifs(
    &self,
    args: &[FormulaAst<'doc>],
    max: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 || args.len().is_multiple_of(2) {
      return None;
    }
    self.evaluate_ifs(
      Some(&args[0]),
      &args[1..],
      if max {
        IfsAggregate::Max
      } else {
        IfsAggregate::Min
      },
    )
  }

  pub(crate) fn evaluate_database_function(
    &self,
    args: &[FormulaAst<'doc>],
    function: DatabaseFunction,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let Some(database) = self.query_grid_from_ast(args.first()?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(criteria) = self.query_grid_from_ast(args.get(2)?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    if database.values.len() < 2
      || database.values.first().is_none_or(Vec::is_empty)
      || criteria.values.len() < 2
      || criteria.values.first().is_none_or(Vec::is_empty)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let field = match self.database_field_index(args.get(1)?, &database.values[0], function) {
      Some(field) => field,
      None => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    };
    let rows = self.database_matching_rows(&database, &criteria);
    if field.is_none() && matches!(function, DatabaseFunction::Count | DatabaseFunction::CountA) {
      return Some(FormulaValue::Number(rows.len() as f64));
    }
    let Some(field) = field else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    if field >= database.values[0].len() {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }

    let mut values = Vec::new();
    let mut text_values = Vec::new();
    for row in rows {
      let value = row.get(field).cloned().unwrap_or_default();
      match function {
        DatabaseFunction::Count => {
          if formula_cell_numeric_value(&value).is_some() {
            values.push(1.0);
          }
        }
        DatabaseFunction::CountA => {
          if !matches!(value, FormulaValue::Blank) {
            values.push(1.0);
          }
        }
        DatabaseFunction::Get => {
          if !matches!(value, FormulaValue::Blank) {
            text_values.push(value);
          }
        }
        _ => {
          if let Some(number) = formula_cell_numeric_value(&value) {
            values.push(number);
          }
        }
      }
    }

    match function {
      DatabaseFunction::Count | DatabaseFunction::CountA => {
        Some(FormulaValue::Number(values.len() as f64))
      }
      DatabaseFunction::Sum => Some(FormulaValue::Number(kahan_sum(values.iter().copied()))),
      DatabaseFunction::Average if values.is_empty() => {
        Some(FormulaValue::Error(FormulaErrorValue::Div0))
      }
      DatabaseFunction::Average => Some(FormulaValue::Number(
        kahan_sum(values.iter().copied()) / values.len() as f64,
      )),
      DatabaseFunction::Get => match text_values.as_slice() {
        [value] => Some(value.clone()),
        [] => Some(FormulaValue::Error(FormulaErrorValue::Value)),
        _ => Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
      },
      DatabaseFunction::Max => Some(FormulaValue::Number(
        values.into_iter().reduce(f64::max).unwrap_or(0.0),
      )),
      DatabaseFunction::Min => Some(FormulaValue::Number(
        values.into_iter().reduce(f64::min).unwrap_or(0.0),
      )),
      DatabaseFunction::Product => Some(FormulaValue::Number(if values.is_empty() {
        0.0
      } else {
        values.into_iter().product()
      })),
      DatabaseFunction::Var => variance_slice(&values, true)
        .map(FormulaValue::Number)
        .or(Some(FormulaValue::Error(FormulaErrorValue::Div0))),
      DatabaseFunction::VarP => variance_slice(&values, false)
        .map(FormulaValue::Number)
        .or(Some(FormulaValue::Error(FormulaErrorValue::Div0))),
      DatabaseFunction::StdDev => variance_slice(&values, true)
        .map(|value| FormulaValue::Number(value.sqrt()))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Div0))),
      DatabaseFunction::StdDevP => variance_slice(&values, false)
        .map(|value| FormulaValue::Number(value.sqrt()))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Div0))),
    }
  }

  pub(crate) fn database_field_index(
    &self,
    field_arg: &FormulaAst<'doc>,
    headers: &[FormulaValue<'doc>],
    function: DatabaseFunction,
  ) -> Option<Option<usize>> {
    let value = self.evaluate(field_arg)?;
    let allow_missing = matches!(function, DatabaseFunction::Count | DatabaseFunction::CountA);
    match self.first_value(&value) {
      FormulaValue::Blank if allow_missing => Some(None),
      FormulaValue::Number(value) if allow_missing && value.floor() == 0.0 => Some(None),
      FormulaValue::Number(value) => {
        let index = value.floor() as i64 - 1;
        (index >= 0).then_some(Some(index as usize))
      }
      FormulaValue::String(name) => headers
        .iter()
        .position(|header| self.text(header).eq_ignore_ascii_case(name.trim()))
        .map(Some)
        .or(Some(Some(usize::MAX))),
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_) => {
        if allow_missing {
          Some(None)
        } else {
          None
        }
      }
      FormulaValue::Boolean(_) | FormulaValue::Error(_) | FormulaValue::Blank => None,
    }
  }

  pub(crate) fn database_matching_rows<'b>(
    &self,
    database: &'b QueryGrid<'doc>,
    criteria: &QueryGrid<'doc>,
  ) -> Vec<&'b [FormulaValue<'doc>]> {
    let params = self.database_query_params(&database.values[0], criteria);
    database
      .values
      .iter()
      .zip(database.query_empty.iter())
      .skip(1)
      .filter(|(row, query_empty)| {
        params
          .iter()
          .any(|param| param.matches_row_with_empty(self, row, query_empty))
      })
      .map(|(row, _)| row.as_slice())
      .collect()
  }

  pub(crate) fn database_query_params(
    &self,
    headers: &[FormulaValue<'doc>],
    criteria: &QueryGrid<'doc>,
  ) -> Vec<QueryParam<'doc>> {
    if let Some(params) = self.database_star_query_params(headers, criteria)
      && !params.is_empty()
    {
      return params;
    }
    let Some(criteria_headers) = criteria.values.first() else {
      return Vec::new();
    };
    criteria
      .values
      .iter()
      .zip(criteria.query_empty.iter())
      .skip(1)
      .filter_map(|(criteria_row, criteria_empty)| {
        let mut entries = Vec::new();
        let mut search_type = QuerySearchType::Normal;
        let mut invalid = false;
        let row_has_present_cell = criteria_row
          .iter()
          .zip(criteria_empty.iter())
          .any(|(value, query_empty)| database_criterion_cell_present(value, *query_empty));
        for (criteria_column, criterion_value) in criteria_row.iter().enumerate() {
          if !database_criterion_entry_present(
            criterion_value,
            criteria_empty
              .get(criteria_column)
              .copied()
              .unwrap_or(false),
          ) {
            continue;
          }
          let Some(header) = criteria_headers.get(criteria_column) else {
            continue;
          };
          let header = self.text(header);
          if header.is_empty() {
            continue;
          }
          let Some(field) = headers
            .iter()
            .position(|database_header| self.text(database_header).eq_ignore_ascii_case(&header))
          else {
            invalid = true;
            break;
          };
          let (entry, entry_search_type) =
            QueryEntry::from_database_value(self, criterion_value, field);
          if search_type == QuerySearchType::Normal {
            search_type = entry_search_type;
          }
          entries.push(entry);
        }
        if invalid {
          return None;
        }
        if entries.is_empty() && !row_has_present_cell {
          return None;
        }
        Some(QueryParam {
          entries,
          search_type,
          range_lookup: false,
          match_whole_cell: self.book.formula_match_whole_cell,
          case_sensitive: false,
        })
      })
      .collect()
  }

  pub(crate) fn database_star_query_params(
    &self,
    headers: &[FormulaValue<'doc>],
    criteria: &QueryGrid<'doc>,
  ) -> Option<Vec<QueryParam<'doc>>> {
    if criteria.values.first().map_or(0, Vec::len) < 4 {
      return None;
    }
    if !criteria.values.iter().any(|row| {
      let connector = self.text(row.first().unwrap_or(&FormulaValue::Blank));
      connector.eq_ignore_ascii_case("AND") || connector.eq_ignore_ascii_case("OR")
    }) {
      return None;
    }
    let mut params = Vec::new();
    let mut current = QueryParam {
      entries: Vec::new(),
      search_type: QuerySearchType::Normal,
      range_lookup: false,
      match_whole_cell: self.book.formula_match_whole_cell,
      case_sensitive: false,
    };
    for (row_index, (row, _row_empty)) in criteria
      .values
      .iter()
      .zip(criteria.query_empty.iter())
      .enumerate()
    {
      let connector = self.text(row.first().unwrap_or(&FormulaValue::Blank));
      if row_index > 0 && connector.eq_ignore_ascii_case("OR") {
        if !current.entries.is_empty() {
          params.push(current);
        }
        current = QueryParam {
          entries: Vec::new(),
          search_type: QuerySearchType::Normal,
          range_lookup: false,
          match_whole_cell: self.book.formula_match_whole_cell,
          case_sensitive: false,
        };
      } else if row_index > 0 && !connector.is_empty() && !connector.eq_ignore_ascii_case("AND") {
        return None;
      }
      let field_name = self.text(row.get(1).unwrap_or(&FormulaValue::Blank));
      if field_name.is_empty() {
        return None;
      }
      let field = headers
        .iter()
        .position(|header| self.text(header).eq_ignore_ascii_case(&field_name))?;
      let op_text = self.text(row.get(2).unwrap_or(&FormulaValue::Blank));
      let op = match op_text.trim() {
        "" | "=" => QueryOp::Equal,
        "<>" => QueryOp::NotEqual,
        "<" => QueryOp::Less,
        "<=" => QueryOp::LessOrEqual,
        ">" => QueryOp::Greater,
        ">=" => QueryOp::GreaterOrEqual,
        _ => return None,
      };
      let criterion = row.get(3).cloned().unwrap_or_default();
      let (mut entry, search_type) = QueryEntry::from_database_value(self, &criterion, field);
      entry.op = op;
      if current.search_type == QuerySearchType::Normal {
        current.search_type = search_type;
      }
      current.entries.push(entry);
    }
    if !current.entries.is_empty() {
      params.push(current);
    }
    Some(params)
  }

  pub(crate) fn evaluate_ifs(
    &self,
    main_range: Option<&FormulaAst<'doc>>,
    criteria_args: &[FormulaAst<'doc>],
    aggregate: IfsAggregate,
  ) -> Option<FormulaValue<'doc>> {
    let mut criteria_ranges = Vec::with_capacity(criteria_args.len() / 2);
    let mut criteria_sets = Vec::with_capacity(criteria_args.len() / 2);
    let mut result_shape = (1usize, 1usize);
    let mut result_len = 1usize;
    for pair in criteria_args.chunks_exact(2) {
      let range = self.query_grid_from_ast(&pair[0])?;
      let rows = range.values.len();
      let columns = range.values.first().map_or(0, Vec::len);
      if rows == 0
        || columns == 0
        || range.values.iter().any(|row| row.len() != columns)
        || range.query_empty.iter().any(|row| row.len() != columns)
      {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      let criteria_matrix = self.matrix_values(&self.evaluate(&pair[1])?);
      let criteria_rows = criteria_matrix.len();
      let criteria_columns = criteria_matrix.first().map_or(0, Vec::len);
      if criteria_rows == 0 || criteria_columns == 0 {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      if criteria_rows * criteria_columns > 1 {
        if result_len == 1 {
          result_shape = (criteria_rows, criteria_columns);
          result_len = criteria_rows * criteria_columns;
        } else if result_shape != (criteria_rows, criteria_columns) {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        }
      }
      criteria_ranges.push(range);
      criteria_sets.push(
        criteria_matrix
          .into_iter()
          .flatten()
          .map(|value| QueryParam::from_criterion(self, &value, 0))
          .collect::<Vec<_>>(),
      );
    }

    if criteria_ranges
      .windows(2)
      .any(|window| window[0].dimensions() != window[1].dimensions())
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let dimensions = criteria_ranges.first()?.dimensions();
    let main_values = if let Some(main_range) = main_range {
      let values = self.query_grid_from_ast(main_range)?;
      if values.dimensions() != dimensions {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      Some(values)
    } else {
      None
    };

    let mut outputs = Vec::with_capacity(result_len);
    for criteria_index in 0..result_len {
      let mut count = 0.0;
      let mut sum = KahanSum::default();
      let mut minmax = None::<f64>;
      for row in 0..dimensions.0 {
        for column in 0..dimensions.1 {
          let matches_all =
            criteria_ranges
              .iter()
              .zip(criteria_sets.iter())
              .all(|(range, criteria)| {
                let criteria = if criteria.len() == 1 {
                  &criteria[0]
                } else {
                  &criteria[criteria_index]
                };
                criteria.matches_value(
                  self,
                  &range.values[row][column],
                  range.query_empty[row][column],
                )
              });
          if !matches_all {
            continue;
          }
          match aggregate {
            IfsAggregate::Count => count += 1.0,
            IfsAggregate::Sum | IfsAggregate::Average | IfsAggregate::Min | IfsAggregate::Max => {
              if let Some(number) = main_values
                .as_ref()
                .and_then(|values| formula_cell_numeric_value(&values.values[row][column]))
              {
                count += 1.0;
                sum.add(number);
                minmax = Some(match (aggregate, minmax) {
                  (IfsAggregate::Min, Some(value)) => value.min(number),
                  (IfsAggregate::Max, Some(value)) => value.max(number),
                  _ => number,
                });
              }
            }
          }
        }
      }
      outputs.push(match aggregate {
        IfsAggregate::Count => FormulaValue::Number(count),
        IfsAggregate::Sum => FormulaValue::Number(sum.finish()),
        IfsAggregate::Average if count == 0.0 => FormulaValue::Error(FormulaErrorValue::Div0),
        IfsAggregate::Average => FormulaValue::Number(sum.finish() / count),
        IfsAggregate::Min | IfsAggregate::Max => FormulaValue::Number(minmax.unwrap_or(0.0)),
      });
    }

    if result_len == 1 {
      return outputs.into_iter().next();
    }
    let mut rows = Vec::with_capacity(result_shape.0);
    let mut iter = outputs.into_iter();
    for _ in 0..result_shape.0 {
      rows.push(iter.by_ref().take(result_shape.1).collect());
    }
    Some(FormulaValue::Matrix(rows))
  }

  pub(crate) fn evaluate_ifs_reader(
    &self,
    args: crate::function::FunctionArgReader<'_, '_, 'doc>,
    main_range: Option<usize>,
    criteria_start: usize,
    criteria_len: usize,
    aggregate: IfsAggregate,
  ) -> Option<FormulaValue<'doc>> {
    let mut criteria_ranges = Vec::with_capacity(criteria_len / 2);
    let mut criteria_sets = Vec::with_capacity(criteria_len / 2);
    let mut result_shape = (1usize, 1usize);
    let mut result_len = 1usize;
    let criteria_end = criteria_start.checked_add(criteria_len)?;
    for range_index in (criteria_start..criteria_end).step_by(2) {
      let range = args.query_grid(range_index)?;
      let rows = range.values.len();
      let columns = range.values.first().map_or(0, Vec::len);
      if rows == 0
        || columns == 0
        || range.values.iter().any(|row| row.len() != columns)
        || range.query_empty.iter().any(|row| row.len() != columns)
      {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      let criteria_matrix = self.matrix_values(&args.value(range_index + 1)?);
      let criteria_rows = criteria_matrix.len();
      let criteria_columns = criteria_matrix.first().map_or(0, Vec::len);
      if criteria_rows == 0 || criteria_columns == 0 {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      if criteria_rows * criteria_columns > 1 {
        if result_len == 1 {
          result_shape = (criteria_rows, criteria_columns);
          result_len = criteria_rows * criteria_columns;
        } else if result_shape != (criteria_rows, criteria_columns) {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        }
      }
      criteria_ranges.push(range);
      criteria_sets.push(
        criteria_matrix
          .into_iter()
          .flatten()
          .map(|value| QueryParam::from_criterion(self, &value, 0))
          .collect::<Vec<_>>(),
      );
    }

    if criteria_ranges
      .windows(2)
      .any(|window| window[0].dimensions() != window[1].dimensions())
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let dimensions = criteria_ranges.first()?.dimensions();
    let main_values = if let Some(main_range) = main_range {
      let values = args.query_grid(main_range)?;
      if values.dimensions() != dimensions {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      Some(values)
    } else {
      None
    };

    let mut outputs = Vec::with_capacity(result_len);
    for criteria_index in 0..result_len {
      let mut count = 0.0;
      let mut sum = KahanSum::default();
      let mut minmax = None::<f64>;
      for row in 0..dimensions.0 {
        for column in 0..dimensions.1 {
          let matches_all =
            criteria_ranges
              .iter()
              .zip(criteria_sets.iter())
              .all(|(range, criteria)| {
                let criteria = if criteria.len() == 1 {
                  &criteria[0]
                } else {
                  &criteria[criteria_index]
                };
                criteria.matches_value(
                  self,
                  &range.values[row][column],
                  range.query_empty[row][column],
                )
              });
          if !matches_all {
            continue;
          }
          match aggregate {
            IfsAggregate::Count => count += 1.0,
            IfsAggregate::Sum | IfsAggregate::Average | IfsAggregate::Min | IfsAggregate::Max => {
              if let Some(number) = main_values
                .as_ref()
                .and_then(|values| formula_cell_numeric_value(&values.values[row][column]))
              {
                count += 1.0;
                sum.add(number);
                minmax = Some(match (aggregate, minmax) {
                  (IfsAggregate::Min, Some(value)) => value.min(number),
                  (IfsAggregate::Max, Some(value)) => value.max(number),
                  _ => number,
                });
              }
            }
          }
        }
      }
      outputs.push(match aggregate {
        IfsAggregate::Count => FormulaValue::Number(count),
        IfsAggregate::Sum => FormulaValue::Number(sum.finish()),
        IfsAggregate::Average if count == 0.0 => FormulaValue::Error(FormulaErrorValue::Div0),
        IfsAggregate::Average => FormulaValue::Number(sum.finish() / count),
        IfsAggregate::Min | IfsAggregate::Max => FormulaValue::Number(minmax.unwrap_or(0.0)),
      });
    }

    if result_len == 1 {
      return outputs.into_iter().next();
    }
    let mut rows = Vec::with_capacity(result_shape.0);
    let mut iter = outputs.into_iter();
    for _ in 0..result_shape.0 {
      rows.push(iter.by_ref().take(result_shape.1).collect());
    }
    Some(FormulaValue::Matrix(rows))
  }

  pub(crate) fn evaluate_ceiling(
    &self,
    args: &[FormulaAst<'doc>],
    kind: CeilingFloorKind,
  ) -> Option<FormulaValue<'doc>> {
    if (kind == CeilingFloorKind::Precise && !(1..=2).contains(&args.len()))
      || (kind != CeilingFloorKind::Precise && !(1..=3).contains(&args.len()))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    let significance = args
      .get(1)
      .filter(|arg| !is_missing_argument(arg))
      .and_then(|arg| self.evaluate(arg));
    let abs_mode = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    if self.array_context
      && (is_matrix_argument(&value) || significance.as_ref().is_some_and(is_matrix_argument))
    {
      return match significance {
        Some(significance) => {
          self.map_binary_values(value, significance, |evaluator, value, significance| {
            Some(evaluator.ceiling_value(value, Some(significance), abs_mode, kind))
          })
        }
        None => self.map_unary_values(value, |evaluator, value| {
          Some(evaluator.ceiling_value(value, None, abs_mode, kind))
        }),
      };
    }
    Some(self.ceiling_value(&value, significance.as_ref(), abs_mode, kind))
  }

  pub(crate) fn ceiling_value(
    &self,
    value: &FormulaValue<'doc>,
    significance: Option<&FormulaValue<'doc>>,
    abs_mode: bool,
    kind: CeilingFloorKind,
  ) -> FormulaValue<'doc> {
    let Some(value) = self.number(value) else {
      return FormulaValue::Error(FormulaErrorValue::Value);
    };
    let significance = significance.and_then(|value| self.number(value));
    match numeric_ceiling(value, significance, abs_mode, kind) {
      Ok(result) => FormulaValue::Number(result),
      Err(error) => FormulaValue::Error(numeric_error_value(error)),
    }
  }

  pub(crate) fn evaluate_floor(
    &self,
    args: &[FormulaAst<'doc>],
    kind: CeilingFloorKind,
  ) -> Option<FormulaValue<'doc>> {
    if (kind == CeilingFloorKind::Precise && !(1..=2).contains(&args.len()))
      || (kind != CeilingFloorKind::Precise && !(1..=3).contains(&args.len()))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    let significance = args
      .get(1)
      .filter(|arg| !is_missing_argument(arg))
      .and_then(|arg| self.evaluate(arg));
    let abs_mode = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    if self.array_context
      && (is_matrix_argument(&value) || significance.as_ref().is_some_and(is_matrix_argument))
    {
      return match significance {
        Some(significance) => {
          self.map_binary_values(value, significance, |evaluator, value, significance| {
            Some(evaluator.floor_value(value, Some(significance), abs_mode, kind))
          })
        }
        None => self.map_unary_values(value, |evaluator, value| {
          Some(evaluator.floor_value(value, None, abs_mode, kind))
        }),
      };
    }
    Some(self.floor_value(&value, significance.as_ref(), abs_mode, kind))
  }

  pub(crate) fn floor_value(
    &self,
    value: &FormulaValue<'doc>,
    significance: Option<&FormulaValue<'doc>>,
    abs_mode: bool,
    kind: CeilingFloorKind,
  ) -> FormulaValue<'doc> {
    let Some(value) = self.number(value) else {
      return FormulaValue::Error(FormulaErrorValue::Value);
    };
    let significance = significance.and_then(|value| self.number(value));
    match numeric_floor(value, significance, abs_mode, kind) {
      Ok(result) => FormulaValue::Number(result),
      Err(error) => FormulaValue::Error(numeric_error_value(error)),
    }
  }

  pub(crate) fn evaluate_ceiling_excel_legacy(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let significance = self.number(&self.evaluate(args.get(1)?)?)?;
    match ceiling_excel_legacy(value, significance) {
      Ok(result) => Some(FormulaValue::Number(result)),
      Err(error) => Some(FormulaValue::Error(numeric_error_value(error))),
    }
  }

  pub(crate) fn evaluate_floor_excel_legacy(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let significance = self.number(&self.evaluate(args.get(1)?)?)?;
    match floor_excel_legacy(value, significance) {
      Ok(result) => Some(FormulaValue::Number(result)),
      Err(error) => Some(FormulaValue::Error(numeric_error_value(error))),
    }
  }

  pub(crate) fn evaluate_percentile(
    &self,
    args: &[FormulaAst<'doc>],
    kind: PercentileKind,
  ) -> Option<FormulaValue<'doc>> {
    let mut values = self.value_numbers(&self.evaluate(args.first()?)?);
    let k = self.number(&self.evaluate(args.get(1)?)?)?;
    percentile_sorted(&mut values, k, kind)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Num)))
  }

  pub(crate) fn evaluate_quartile(
    &self,
    args: &[FormulaAst<'doc>],
    kind: PercentileKind,
  ) -> Option<FormulaValue<'doc>> {
    let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let mut values = self.value_numbers(&value);
    let Some(quartile) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let quartile = approx_floor(quartile);
    percentile_sorted(&mut values, quartile / 4.0, kind)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_mode(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = self.numeric_args(args);
    mode_slice(&values)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  pub(crate) fn evaluate_mode_ms(
    &self,
    args: &[FormulaAst<'doc>],
    single: bool,
  ) -> Option<FormulaValue<'doc>> {
    let values = self.mode_ms_numeric_args(args);
    let modes = mode_ms_values(&values)?;
    if single {
      return modes
        .first()
        .copied()
        .map(FormulaValue::Number)
        .or(Some(FormulaValue::Error(FormulaErrorValue::NA)));
    }
    Some(FormulaValue::Matrix(
      modes
        .into_iter()
        .map(|value| vec![FormulaValue::Number(value)])
        .collect(),
    ))
  }

  pub(crate) fn evaluate_rank(
    &self,
    args: &[FormulaAst<'doc>],
    average: bool,
  ) -> Option<FormulaValue<'doc>> {
    let Some(value) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(range_value) = args.get(1).and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let values = self.value_numbers(&range_value);
    let ascending = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    rank_value(values, value, ascending, average)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  pub(crate) fn evaluate_kurt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = self.numeric_args(args);
    match kurtosis(&values) {
      Ok(value) => Some(FormulaValue::Number(value)),
      Err(error) => Some(FormulaValue::Error(statistics_error_value(error))),
    }
  }

  pub(crate) fn evaluate_beta_dist(
    &self,
    args: &[FormulaAst<'doc>],
    legacy: bool,
  ) -> Option<FormulaValue<'doc>> {
    if (legacy && !(3..=6).contains(&args.len())) || (!legacy && !(4..=6).contains(&args.len())) {
      return None;
    }
    let x_matrix = self.matrix_values(&self.evaluate(args.first()?)?);
    let alpha_matrix = self.matrix_values(&self.evaluate(args.get(1)?)?);
    let beta_matrix = self.matrix_values(&self.evaluate(args.get(2)?)?);
    let (cumulative_matrix, lower_matrix, upper_matrix) = if legacy {
      (
        args
          .get(5)
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.matrix_values(&value))
          .unwrap_or_else(|| vec![vec![FormulaValue::Boolean(true)]]),
        args
          .get(3)
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.matrix_values(&value))
          .unwrap_or_else(|| vec![vec![FormulaValue::Number(0.0)]]),
        args
          .get(4)
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.matrix_values(&value))
          .unwrap_or_else(|| vec![vec![FormulaValue::Number(1.0)]]),
      )
    } else {
      (
        self.matrix_values(&self.evaluate(args.get(3)?)?),
        args
          .get(4)
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.matrix_values(&value))
          .unwrap_or_else(|| vec![vec![FormulaValue::Number(0.0)]]),
        args
          .get(5)
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.matrix_values(&value))
          .unwrap_or_else(|| vec![vec![FormulaValue::Number(1.0)]]),
      )
    };
    let rows = x_matrix
      .len()
      .max(alpha_matrix.len())
      .max(beta_matrix.len())
      .max(lower_matrix.len())
      .max(upper_matrix.len())
      .max(cumulative_matrix.len());
    let columns = x_matrix
      .first()
      .map(Vec::len)
      .unwrap_or(1)
      .max(alpha_matrix.first().map(Vec::len).unwrap_or(1))
      .max(beta_matrix.first().map(Vec::len).unwrap_or(1))
      .max(lower_matrix.first().map(Vec::len).unwrap_or(1))
      .max(upper_matrix.first().map(Vec::len).unwrap_or(1))
      .max(cumulative_matrix.first().map(Vec::len).unwrap_or(1));
    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let x = self.number(matrix_item(&x_matrix, row, column)?)?;
        let alpha = self.number(matrix_item(&alpha_matrix, row, column)?)?;
        let beta = self.number(matrix_item(&beta_matrix, row, column)?)?;
        let lower = self.number(matrix_item(&lower_matrix, row, column)?)?;
        let upper = self.number(matrix_item(&upper_matrix, row, column)?)?;
        let cumulative = self.truthy(matrix_item(&cumulative_matrix, row, column)?);
        let scale = upper - lower;
        if alpha <= 0.0 || beta <= 0.0 || (legacy && scale <= 0.0) {
          result_row.push(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
          continue;
        }
        if !legacy && scale == 0.0 {
          result_row.push(FormulaValue::Error(FormulaErrorValue::Num));
          continue;
        }
        if !legacy && (x < lower || x > upper) {
          result_row.push(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
          continue;
        }
        if x < lower {
          result_row.push(FormulaValue::Number(0.0));
          continue;
        }
        if x > upper {
          result_row.push(if cumulative {
            FormulaValue::Number(1.0)
          } else {
            FormulaValue::Number(0.0)
          });
          continue;
        }
        let scaled = (x - lower) / scale;
        result_row.push(if cumulative {
          FormulaValue::Number(lo_beta_dist(scaled, alpha, beta))
        } else {
          match lo_beta_dist_pdf(scaled, alpha, beta) {
            Ok(value) => FormulaValue::Number(value / scale),
            Err(error) => FormulaValue::Error(special_error_value(error)),
          }
        });
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      result.into_iter().next()?.into_iter().next()
    } else {
      Some(FormulaValue::Matrix(result))
    }
  }

  pub(crate) fn evaluate_erf(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(lower) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let value = if let Some(upper) = args.get(1) {
      let Some(upper) = self.evaluate(upper).and_then(|value| self.number(&value)) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      erf(upper) - erf(lower)
    } else {
      erf(lower)
    };
    Some(FormulaValue::Number(value))
  }

  pub(crate) fn evaluate_delta(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(left) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let right = match args.get(1) {
      Some(arg) => {
        let value = self.evaluate(arg)?;
        let Some(number) = self.number(&value) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        number
      }
      None => 0.0,
    };
    Some(FormulaValue::Number((left == right) as u8 as f64))
  }

  pub(crate) fn evaluate_gestep(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(value) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let step = match args.get(1) {
      Some(arg) => {
        let value = self.evaluate(arg)?;
        let Some(number) = self.number(&value) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        number
      }
      None => 0.0,
    };
    Some(FormulaValue::Number((value >= step) as u8 as f64))
  }

  pub(crate) fn evaluate_beta_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let p_matrix = self.matrix_values(&self.evaluate(args.first()?)?);
    let alpha_matrix = self.matrix_values(&self.evaluate(args.get(1)?)?);
    let beta_matrix = self.matrix_values(&self.evaluate(args.get(2)?)?);
    let lower_matrix = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.matrix_values(&value))
      .unwrap_or_else(|| vec![vec![FormulaValue::Number(0.0)]]);
    let upper_matrix = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.matrix_values(&value))
      .unwrap_or_else(|| vec![vec![FormulaValue::Number(1.0)]]);
    let rows = p_matrix
      .len()
      .max(alpha_matrix.len())
      .max(beta_matrix.len())
      .max(lower_matrix.len())
      .max(upper_matrix.len());
    let columns = p_matrix
      .first()
      .map(Vec::len)
      .unwrap_or(1)
      .max(alpha_matrix.first().map(Vec::len).unwrap_or(1))
      .max(beta_matrix.first().map(Vec::len).unwrap_or(1))
      .max(lower_matrix.first().map(Vec::len).unwrap_or(1))
      .max(upper_matrix.first().map(Vec::len).unwrap_or(1));
    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let p = self.number(matrix_item(&p_matrix, row, column)?)?;
        let alpha = self.number(matrix_item(&alpha_matrix, row, column)?)?;
        let beta = self.number(matrix_item(&beta_matrix, row, column)?)?;
        let lower = self.number(matrix_item(&lower_matrix, row, column)?)?;
        let upper = self.number(matrix_item(&upper_matrix, row, column)?)?;
        if !(0.0..=1.0).contains(&p) || alpha <= 0.0 || beta <= 0.0 || lower >= upper {
          result_row.push(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
          continue;
        }
        if p == 0.0 {
          result_row.push(FormulaValue::Number(lower));
          continue;
        }
        if p == 1.0 {
          result_row.push(FormulaValue::Number(upper));
          continue;
        }
        result_row.push(
          match lo_iterate_inverse(|x| p - lo_beta_dist(x, alpha, beta), 0.0, 1.0) {
            Ok(value) => FormulaValue::Number(lower + value * (upper - lower)),
            Err(error) => FormulaValue::Error(special_error_value(error)),
          },
        );
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      result.into_iter().next()?.into_iter().next()
    } else {
      Some(FormulaValue::Matrix(result))
    }
  }

  pub(crate) fn evaluate_binom_dist(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?.floor();
    let n = self.number(&self.evaluate(args.get(1)?)?)?.floor();
    let p = self.number(&self.evaluate(args.get(2)?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(3)?)?);
    if x < 0.0 || n < 0.0 || x > n || !(0.0..=1.0).contains(&p) {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let dist = Binomial::new(p, n as u64).ok()?;
    Some(FormulaValue::Number(if cumulative {
      dist.cdf(x as u64)
    } else {
      dist.pmf(x as u64)
    }))
  }

  pub(crate) fn evaluate_b(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=4).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let n = approx_floor(self.number(&self.evaluate(args.first()?)?)?);
    let p = self.number(&self.evaluate(args.get(1)?)?)?;
    if args.len() == 3 {
      let x = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?);
      if n < 0.0 || x < 0.0 || x > n || !(0.0..=1.0).contains(&p) {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      if p == 0.0 {
        return Some(FormulaValue::Number((x == 0.0) as u8 as f64));
      }
      if p == 1.0 {
        return Some(FormulaValue::Number((x == n) as u8 as f64));
      }
      return Some(FormulaValue::Number(lo_binom_dist_pmf(x, n, p)));
    }

    let xs = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?);
    let xe = approx_floor(self.number(&self.evaluate(args.get(3)?)?)?);
    let valid_x = 0.0 <= xs && xs <= xe && xe <= n;
    if valid_x && 0.0 < p && p < 1.0 {
      if xs == xe {
        return Some(FormulaValue::Number(lo_binom_dist_pmf(xs, n, p)));
      }
      let q = (0.5 - p) + 0.5;
      let mut factor = q.powf(n);
      if factor > f64::MIN_POSITIVE {
        return Some(FormulaValue::Number(lo_binom_dist_range(
          n, xs, xe, factor, p, q,
        )));
      }
      factor = p.powf(n);
      if factor > f64::MIN_POSITIVE {
        return Some(FormulaValue::Number(lo_binom_dist_range(
          n,
          n - xe,
          n - xs,
          factor,
          q,
          p,
        )));
      }
      return Some(FormulaValue::Number(
        lo_beta_dist(q, n - xe, xe + 1.0) - lo_beta_dist(q, n - xs + 1.0, xs),
      ));
    }
    if valid_x {
      if p == 0.0 {
        Some(FormulaValue::Number((xs == 0.0) as u8 as f64))
      } else if p == 1.0 {
        Some(FormulaValue::Number((xe == n) as u8 as f64))
      } else {
        Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
      }
    } else {
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    }
  }

  pub(crate) fn evaluate_binom_dist_range(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    self.evaluate_b(args)
  }

  pub(crate) fn evaluate_binom_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let n = approx_floor(self.number(&self.evaluate(args.first()?)?)?);
    let p = self.number(&self.evaluate(args.get(1)?)?)?;
    let alpha = self.number(&self.evaluate(args.get(2)?)?)?;
    if n < 0.0 || !(0.0..=1.0).contains(&p) || !(0.0..=1.0).contains(&alpha) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if alpha == 0.0 {
      return Some(FormulaValue::Number(0.0));
    }
    if alpha == 1.0 {
      return Some(FormulaValue::Number(if p == 0.0 { 0.0 } else { n }));
    }

    let q = (0.5 - p) + 0.5;
    if q > p {
      let mut factor = q.powf(n);
      if factor > f64::MIN_POSITIVE {
        let mut sum = KahanSum::default();
        sum.add(factor);
        let mut i = 0_u32;
        let max = n as u32;
        while i < max && sum.finish() < alpha {
          factor *= (n - f64::from(i)) / f64::from(i + 1) * p / q;
          sum.add(factor);
          i += 1;
        }
        Some(FormulaValue::Number(f64::from(i)))
      } else {
        let mut sum = KahanSum::default();
        let mut i = 0_u32;
        let max = n as u32;
        while i < max && sum.finish() < alpha {
          let x = lo_beta_dist_pdf(p, f64::from(i + 1), n - f64::from(i) + 1.0).ok()? / (n + 1.0);
          sum.add(x);
          i += 1;
        }
        Some(FormulaValue::Number(f64::from(i.saturating_sub(1))))
      }
    } else {
      let mut factor = p.powf(n);
      if factor > f64::MIN_POSITIVE {
        let mut sum = KahanSum::default();
        sum.add(1.0);
        sum.add(-factor);
        let mut i = 0_u32;
        let max = n as u32;
        while i < max && sum.finish() >= alpha {
          factor *= (n - f64::from(i)) / f64::from(i + 1) * q / p;
          sum.add(-factor);
          i += 1;
        }
        Some(FormulaValue::Number(n - f64::from(i)))
      } else {
        let mut sum = KahanSum::default();
        let tail_alpha = 1.0 - alpha;
        let mut i = 0_u32;
        let max = n as u32;
        while i < max && sum.finish() < tail_alpha {
          let x = lo_beta_dist_pdf(q, f64::from(i + 1), n - f64::from(i) + 1.0).ok()? / (n + 1.0);
          sum.add(x);
          i += 1;
        }
        Some(FormulaValue::Number(n - f64::from(i) + 1.0))
      }
    }
  }

  pub(crate) fn evaluate_chisq_dist(
    &self,
    args: &[FormulaAst<'doc>],
    right_tail: bool,
  ) -> Option<FormulaValue<'doc>> {
    let x_value = self.evaluate(args.first()?)?;
    let df_value = self.evaluate(args.get(1)?)?;
    let cumulative_value = args.get(2).and_then(|arg| self.evaluate(arg));
    if self.array_context
      && (matches!(
        x_value,
        FormulaValue::Reference(_) | FormulaValue::Matrix(_)
      ) || matches!(
        df_value,
        FormulaValue::Reference(_) | FormulaValue::Matrix(_)
      ) || cumulative_value
        .as_ref()
        .is_some_and(|value| matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))))
    {
      let x_matrix = self.matrix_values(&x_value);
      let df_matrix = self.matrix_values(&df_value);
      let cumulative_matrix = cumulative_value
        .as_ref()
        .map(|value| self.matrix_values(value))
        .unwrap_or_default();
      let rows = x_matrix
        .len()
        .max(df_matrix.len())
        .max(cumulative_matrix.len());
      let columns = x_matrix
        .first()
        .map(Vec::len)
        .unwrap_or(1)
        .max(df_matrix.first().map(Vec::len).unwrap_or(1))
        .max(cumulative_matrix.first().map(Vec::len).unwrap_or(1));
      let mut result = Vec::with_capacity(rows);
      for row in 0..rows {
        let mut result_row = Vec::with_capacity(columns);
        for column in 0..columns {
          let x = self.number(matrix_item(&x_matrix, row, column)?)?;
          let df = approx_floor(self.number(matrix_item(&df_matrix, row, column)?)?);
          let cumulative = cumulative_value.as_ref().map(|_| {
            matrix_item(&cumulative_matrix, row, column)
              .map(|value| self.truthy(value))
              .unwrap_or(false)
          });
          result_row.push(chisq_dist_value(x, df, right_tail, cumulative));
        }
        result.push(result_row);
      }
      return Some(FormulaValue::Matrix(result));
    }
    let x = self.number(&x_value)?;
    let df = approx_floor(self.number(&df_value)?);
    let cumulative = cumulative_value.as_ref().map(|value| self.truthy(value));
    Some(chisq_dist_value(x, df, right_tail, cumulative))
  }

  pub(crate) fn evaluate_chisq_inv(
    &self,
    args: &[FormulaAst<'doc>],
    right_tail: bool,
  ) -> Option<FormulaValue<'doc>> {
    let p_value = self.evaluate(args.first()?)?;
    let df_value = self.evaluate(args.get(1)?)?;
    if self.array_context
      && (matches!(
        p_value,
        FormulaValue::Reference(_) | FormulaValue::Matrix(_)
      ) || matches!(
        df_value,
        FormulaValue::Reference(_) | FormulaValue::Matrix(_)
      ))
    {
      let p_matrix = self.matrix_values(&p_value);
      let df_matrix = self.matrix_values(&df_value);
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
          let p = self.number(matrix_item(&p_matrix, row, column)?)?;
          let df = approx_floor(self.number(matrix_item(&df_matrix, row, column)?)?);
          result_row.push(chisq_inv_value(p, df, right_tail));
        }
        result.push(result_row);
      }
      return Some(FormulaValue::Matrix(result));
    }
    let p = self.number(&p_value)?;
    let df = approx_floor(self.number(&df_value)?);
    Some(chisq_inv_value(p, df, right_tail))
  }

  pub(crate) fn evaluate_chisq_test(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let actual = self.matrix_values(&self.evaluate(args.first()?)?);
    let expected = self.matrix_values(&self.evaluate(args.get(1)?)?);
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
            let Some(observed) = self.number(left) else {
              return Some(FormulaValue::Error(FormulaErrorValue::Value));
            };
            let Some(expect) = self.number(right) else {
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

  pub(crate) fn evaluate_confidence_norm(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(alpha) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(sigma) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(size) = self.number_arg(args, 2) else {
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
    Some(FormulaValue::Number(
      norm_s_inv(1.0 - alpha / 2.0).abs() * sigma / size.sqrt(),
    ))
  }

  pub(crate) fn evaluate_confidence_t(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(alpha) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(sigma) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(size) = self.number_arg(args, 2) else {
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
    if size == 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    let dist = StudentsT::new(0.0, 1.0, size - 1.0).ok()?;
    Some(FormulaValue::Number(
      dist.inverse_cdf(1.0 - alpha / 2.0).abs() * sigma / size.sqrt(),
    ))
  }

  pub(crate) fn evaluate_covariance(
    &self,
    args: &[FormulaAst<'doc>],
    sample: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let left = self.matrix_values(&self.evaluate(args.first()?)?);
    let right = self.matrix_values(&self.evaluate(args.get(1)?)?);
    let Some(pairs) = covariance_pairs(&left, &right) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    covariance(&pairs, sample)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_correl(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let left = self.value_numbers(&self.evaluate(args.first()?)?);
    let right = self.value_numbers(&self.evaluate(args.get(1)?)?);
    correlation(&left, &right)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Div0)))
  }

  pub(crate) fn evaluate_slope(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let y_values = self.value_numbers(&self.evaluate(args.first()?)?);
    let x_values = self.value_numbers(&self.evaluate(args.get(1)?)?);
    let state = regression_scalar_state_from_slices(&y_values, &x_values);
    if state.count < 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    state
      .slope()
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Div0)))
  }

  pub(crate) fn evaluate_intercept(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let y_value = self.evaluate(args.first()?)?;
    let x_value = self.evaluate(args.get(1)?)?;
    if matrix_dimensions(&self.matrix_values(&y_value))
      != matrix_dimensions(&self.matrix_values(&x_value))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    regression_scalar_state_for_values(self, &y_value, &x_value).map(|state| {
      state
        .intercept()
        .map(FormulaValue::Number)
        .unwrap_or(FormulaValue::Error(FormulaErrorValue::Div0))
    })
  }

  pub(crate) fn evaluate_forecast(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let y_value = self.evaluate(args.get(1)?)?;
    let x_value = self.evaluate(args.get(2)?)?;
    if matrix_dimensions(&self.matrix_values(&y_value))
      != matrix_dimensions(&self.matrix_values(&x_value))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    regression_scalar_state_for_values(self, &y_value, &x_value).map(|state| {
      state
        .forecast(x)
        .map(FormulaValue::Number)
        .unwrap_or(FormulaValue::Error(FormulaErrorValue::Div0))
    })
  }

  pub(crate) fn evaluate_forecast_ets(
    &self,
    args: &[FormulaAst<'doc>],
    kind: EtsKind,
  ) -> Option<FormulaValue<'doc>> {
    let valid_count = match kind {
      EtsKind::Add | EtsKind::Mult | EtsKind::StatAdd | EtsKind::StatMult => {
        (3..=6).contains(&args.len())
      }
      EtsKind::Season => (2..=4).contains(&args.len()),
    };
    if !valid_count {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let aggregation_index = match kind {
      EtsKind::Season => 3,
      _ => 5,
    };
    let data_completion_index = match kind {
      EtsKind::Season => 2,
      _ => 4,
    };
    let seasonality_index = 3;
    let aggregation = args
      .get(aggregation_index)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0)
      .floor() as i32;
    if !(1..=7).contains(&aggregation) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let data_completion = args
      .get(data_completion_index)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0);
    if data_completion != 0.0 && data_completion != 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let samples_in_period = if kind == EtsKind::Season {
      1
    } else {
      let value = args
        .get(seasonality_index)
        .and_then(|arg| self.evaluate(arg))
        .and_then(|value| self.number(&value))
        .unwrap_or(1.0);
      if value < 0.0 || value.fract() != 0.0 {
        return Some(FormulaValue::Error(FormulaErrorValue::Num));
      }
      value as usize
    };

    let type_matrix = if matches!(kind, EtsKind::StatAdd | EtsKind::StatMult) {
      let matrix = self.matrix_values(&self.evaluate(args.first()?)?);
      for value in matrix.iter().flatten() {
        let Some(number) = self.number(value) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        if !(1.0..=9.0).contains(&number) {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        }
      }
      Some(matrix)
    } else {
      None
    };
    let (target_arg, values_arg, timeline_arg) = match kind {
      EtsKind::Season => (None, args.first()?, args.get(1)?),
      EtsKind::StatAdd | EtsKind::StatMult => (None, args.get(1)?, args.get(2)?),
      EtsKind::Add | EtsKind::Mult => (Some(args.first()?), args.get(1)?, args.get(2)?),
    };
    let values = self.value_numbers(&self.evaluate(values_arg)?);
    let timeline = self.value_numbers(&self.evaluate(timeline_arg)?);
    if values.len() != timeline.len() || values.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let target_value = target_arg.and_then(|arg| self.evaluate(arg));
    let target_matrix = target_value.as_ref().map(|value| self.matrix_values(value));
    let target_first = target_matrix
      .as_ref()
      .and_then(|matrix| matrix.first())
      .and_then(|row| row.first())
      .and_then(|value| self.number(value));
    let mut calc = match EtsCalculation::new(
      &timeline,
      &values,
      samples_in_period,
      data_completion != 0.0,
      aggregation,
      target_first,
      kind,
    ) {
      Ok(calc) => calc,
      Err(error) => return Some(FormulaValue::Error(ets_error_value(error))),
    };
    match kind {
      EtsKind::Season => Some(FormulaValue::Number(calc.samples_in_period as f64)),
      EtsKind::StatAdd | EtsKind::StatMult => {
        let matrix = type_matrix?;
        Some(FormulaValue::Matrix(
          matrix
            .into_iter()
            .map(|row| {
              row
                .into_iter()
                .map(|value| {
                  let index = self.number(&value).unwrap_or(0.0).floor() as i32;
                  FormulaValue::Number(calc.statistic(index))
                })
                .collect()
            })
            .collect(),
        ))
      }
      EtsKind::Add | EtsKind::Mult => {
        let matrix = target_matrix?;
        let result = matrix
          .iter()
          .map(|row| {
            row
              .iter()
              .map(|value| {
                self
                  .number(value)
                  .map(|target| FormulaValue::Number(calc.forecast(target)))
                  .unwrap_or(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
              })
              .collect::<Vec<_>>()
          })
          .collect::<Vec<_>>();
        if result.len() == 1 && result.first().is_some_and(|row| row.len() == 1) {
          result.into_iter().next()?.into_iter().next()
        } else {
          Some(FormulaValue::Matrix(result))
        }
      }
    }
  }

  pub(crate) fn evaluate_rsq(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    regression_scalar_state(self, args).map(|state| {
      state
        .r_squared()
        .map(FormulaValue::Number)
        .unwrap_or(FormulaValue::Error(FormulaErrorValue::Div0))
    })
  }

  pub(crate) fn evaluate_steyx(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    regression_scalar_state(self, args).map(|state| {
      state
        .steyx()
        .map(FormulaValue::Number)
        .unwrap_or(FormulaValue::Error(FormulaErrorValue::Div0))
    })
  }

  pub(crate) fn evaluate_linest(
    &self,
    args: &[FormulaAst<'doc>],
    log_regression: bool,
  ) -> Option<FormulaValue<'doc>> {
    if !(1..=4).contains(&args.len()) {
      return None;
    }
    let constant = match args.get(2) {
      Some(arg) if is_missing_argument(arg) => true,
      Some(arg) => self.evaluate(arg).map(|value| self.truthy(&value))?,
      None => true,
    };
    let stats = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(false);
    let x_arg = args.get(1).filter(|arg| !is_missing_argument(arg));
    let data = match self.regression_data(args.first()?, x_arg, log_regression) {
      Ok(data) => data,
      Err(error) => return Some(FormulaValue::Error(error)),
    };
    regression_coefficients(&data, constant, stats, log_regression)
      .map(FormulaValue::Matrix)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_trend_growth(
    &self,
    args: &[FormulaAst<'doc>],
    growth: bool,
  ) -> Option<FormulaValue<'doc>> {
    if !(1..=4).contains(&args.len()) {
      return None;
    }
    let constant = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    let data = match self.regression_data(args.first()?, args.get(1), growth) {
      Ok(data) => data,
      Err(error) => return Some(FormulaValue::Error(error)),
    };
    let model = match regression_model(&data, constant) {
      Some(model) => model,
      None => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    };
    let prediction = match args.get(2).and_then(|arg| self.evaluate(arg)) {
      Some(value) => match data.prediction_matrix(self, &value) {
        Ok(prediction) => prediction,
        Err(error) => return Some(FormulaValue::Error(error)),
      },
      None => data.default_prediction_matrix(),
    };
    let values = prediction
      .design
      .iter()
      .map(|row| {
        let value = model.predict(row);
        FormulaValue::Number(if growth { value.exp() } else { value })
      })
      .collect::<Vec<_>>();
    Some(FormulaValue::Matrix(prediction.shape.materialize(values)))
  }

  pub(crate) fn regression_data(
    &self,
    y_arg: &FormulaAst<'doc>,
    x_arg: Option<&FormulaAst<'doc>>,
    log_y: bool,
  ) -> std::result::Result<RegressionData, FormulaErrorValue> {
    let y_value = self
      .evaluate(y_arg)
      .ok_or(FormulaErrorValue::IllegalArgument)?;
    let y_matrix = self.matrix_values(&y_value);
    let y_shape = MatrixShape::from_matrix(&y_matrix).ok_or(FormulaErrorValue::IllegalArgument)?;
    let mut y = matrix_numbers(self, &y_matrix).ok_or(FormulaErrorValue::IllegalArgument)?;
    if y.len() != y_shape.len() {
      return Err(FormulaErrorValue::IllegalArgument);
    }
    if log_y {
      if y.iter().any(|value| *value <= 0.0) {
        return Err(FormulaErrorValue::IllegalArgument);
      }
      y.iter_mut().for_each(|value| *value = value.ln());
    }

    let (x_matrix, x_shape) = if let Some(arg) = x_arg {
      let value = self
        .evaluate(arg)
        .ok_or(FormulaErrorValue::IllegalArgument)?;
      let matrix = self.matrix_values(&value);
      let shape = MatrixShape::from_matrix(&matrix).ok_or(FormulaErrorValue::IllegalArgument)?;
      (matrix, shape)
    } else {
      let values = y_shape.materialize(
        (1..=y.len())
          .map(|value| FormulaValue::Number(value as f64))
          .collect(),
      );
      (values, y_shape)
    };
    let x_numbers = matrix_numbers(self, &x_matrix).ok_or(FormulaErrorValue::IllegalArgument)?;
    if x_numbers.len() != x_shape.len() {
      return Err(FormulaErrorValue::IllegalArgument);
    }

    let (case, design) = if x_shape == y_shape {
      (
        RegressionCase::Simple,
        x_numbers.into_iter().map(|value| vec![value]).collect(),
      )
    } else if y_shape.columns != 1 && y_shape.rows != 1 {
      return Err(FormulaErrorValue::IllegalArgument);
    } else if y_shape.columns == 1 {
      if x_shape.rows != y_shape.rows {
        return Err(FormulaErrorValue::IllegalArgument);
      }
      let mut rows = Vec::with_capacity(x_shape.rows);
      for row in 0..x_shape.rows {
        let mut values = Vec::with_capacity(x_shape.columns);
        for column in 0..x_shape.columns {
          values.push(
            matrix_number_at(&x_matrix, row, column, self)
              .ok_or(FormulaErrorValue::IllegalArgument)?,
          );
        }
        rows.push(values);
      }
      (RegressionCase::ColumnY, rows)
    } else {
      if x_shape.columns != y_shape.columns {
        return Err(FormulaErrorValue::IllegalArgument);
      }
      let mut rows = Vec::with_capacity(x_shape.columns);
      for column in 0..x_shape.columns {
        let mut values = Vec::with_capacity(x_shape.rows);
        for row in 0..x_shape.rows {
          values.push(
            matrix_number_at(&x_matrix, row, column, self)
              .ok_or(FormulaErrorValue::IllegalArgument)?,
          );
        }
        rows.push(values);
      }
      (RegressionCase::RowY, rows)
    };
    let k = design.first().map_or(0, Vec::len);
    let n = y.len();
    if n < 1 || k < 1 {
      return Err(FormulaErrorValue::IllegalArgument);
    }
    Ok(RegressionData {
      case,
      x_shape,
      y,
      design,
    })
  }

  pub(crate) fn evaluate_expon_dist(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let lambda = self.number(&self.evaluate(args.get(1)?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(2)?)?);
    if x < 0.0 || lambda <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let dist = Exp::new(lambda).ok()?;
    Some(FormulaValue::Number(if cumulative {
      dist.cdf(x)
    } else {
      dist.pdf(x)
    }))
  }

  pub(crate) fn evaluate_f_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=4).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let df1 = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    let df2 = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?);
    let cumulative = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    if x < 0.0 || df1 < 1.0 || df2 < 1.0 || df1 >= 1.0e10 || df2 >= 1.0e10 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(if cumulative {
      1.0 - lo_f_dist_right_tail(x, df1, df2)
    } else {
      lo_f_dist_pdf(x, df1, df2)
    }))
  }

  pub(crate) fn evaluate_f_dist_rt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let df1 = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    let df2 = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?);
    if x < 0.0 || df1 < 1.0 || df2 < 1.0 || df1 >= 1.0e10 || df2 >= 1.0e10 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(lo_f_dist_right_tail(x, df1, df2)))
  }

  pub(crate) fn evaluate_f_inv(
    &self,
    args: &[FormulaAst<'doc>],
    right_tail: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let p = self.number(&self.evaluate(args.first()?)?)?;
    let df1 = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    let df2 = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?);
    if p <= 0.0 || p > 1.0 || df1 < 1.0 || df2 < 1.0 || df1 >= 1.0e10 || df2 >= 1.0e10 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    match lo_iterate_inverse(
      |x| (if right_tail { p } else { 1.0 - p }) - lo_f_dist_right_tail(x, df1, df2),
      df1 * 0.5,
      df1,
    ) {
      Ok(value) => Some(FormulaValue::Number(value)),
      Err(error) => Some(FormulaValue::Error(special_error_value(error))),
    }
  }

  pub(crate) fn evaluate_f_test(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let left = self.value_numbers(&self.evaluate(args.first()?)?);
    let right = self.value_numbers(&self.evaluate(args.get(1)?)?);
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

  pub(crate) fn evaluate_gamma_dist(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let alpha = self.number(&self.evaluate(args.get(1)?)?)?;
    let beta = self.number(&self.evaluate(args.get(2)?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(3)?)?);
    if x < 0.0 || alpha <= 0.0 || beta <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(if cumulative {
      lo_gamma_dist(x, alpha, beta)
    } else {
      match lo_gamma_dist_pdf(x, alpha, beta) {
        Ok(value) => value,
        Err(error) => return Some(FormulaValue::Error(special_error_value(error))),
      }
    }))
  }

  pub(crate) fn evaluate_gamma_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let p_value = self.evaluate(args.first()?)?;
    let alpha_value = self.evaluate(args.get(1)?)?;
    let beta_value = self.evaluate(args.get(2)?)?;
    let p_matrix = self.matrix_values(&p_value);
    let alpha_matrix = self.matrix_values(&alpha_value);
    let beta_matrix = self.matrix_values(&beta_value);
    let rows = p_matrix
      .len()
      .max(alpha_matrix.len())
      .max(beta_matrix.len());
    let columns = p_matrix
      .first()
      .map(Vec::len)
      .unwrap_or(1)
      .max(alpha_matrix.first().map(Vec::len).unwrap_or(1))
      .max(beta_matrix.first().map(Vec::len).unwrap_or(1));
    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let p = self.number(matrix_item(&p_matrix, row, column)?)?;
        let alpha = self.number(matrix_item(&alpha_matrix, row, column)?)?;
        let beta = self.number(matrix_item(&beta_matrix, row, column)?)?;
        if !(0.0..1.0).contains(&p) || alpha <= 0.0 || beta <= 0.0 {
          result_row.push(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
          continue;
        }
        result_row.push(FormulaValue::Number(if p == 0.0 {
          0.0
        } else {
          match lo_iterate_inverse(
            |x| p - lo_gamma_dist(x, alpha, beta),
            alpha * beta * 0.5,
            alpha * beta,
          ) {
            Ok(value) => value,
            Err(error) => {
              result_row.push(FormulaValue::Error(special_error_value(error)));
              continue;
            }
          }
        }));
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      result.into_iter().next()?.into_iter().next()
    } else {
      Some(FormulaValue::Matrix(result))
    }
  }

  pub(crate) fn evaluate_gamma(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.number(&self.evaluate(args.first()?)?)?;
    if value == 0.0 || (value < 0.0 && value.fract() == 0.0) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(gamma(value)))
  }

  pub(crate) fn evaluate_hypgeom_dist(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if !(4..=5).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(sample_success) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(sample_size) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(population_success) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(population_size) = self.number_arg(args, 3) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let cumulative = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(false);
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

  pub(crate) fn evaluate_lognorm_dist(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let mean = self.number(&self.evaluate(args.get(1)?)?)?;
    let sigma = self.number(&self.evaluate(args.get(2)?)?)?;
    let cumulative = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    if sigma <= 0.0 || (!cumulative && x <= 0.0) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(if cumulative {
      if x <= 0.0 {
        0.0
      } else {
        lo_integral_phi((x.ln() - mean) / sigma)
      }
    } else {
      lo_phi((x.ln() - mean) / sigma) / sigma / x
    }))
  }

  pub(crate) fn evaluate_lognorm_inv(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(p) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(mean) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(sigma) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if p <= 0.0 || p >= 1.0 || sigma <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(
      LogNormal::new(mean, sigma).ok()?.inverse_cdf(p),
    ))
  }

  pub(crate) fn evaluate_negbinom_dist(
    &self,
    args: &[FormulaAst<'doc>],
    microsoft: bool,
  ) -> Option<FormulaValue<'doc>> {
    if (microsoft && args.len() != 4) || (!microsoft && args.len() != 3) {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let failures = self.number(&self.evaluate(args.first()?)?)?.floor();
    let successes = self.number(&self.evaluate(args.get(1)?)?)?.floor();
    let p = self.number(&self.evaluate(args.get(2)?)?)?;
    let cumulative = microsoft
      && args
        .get(3)
        .and_then(|arg| self.evaluate(arg))
        .is_some_and(|value| self.truthy(&value));
    if (microsoft && (failures < 0.0 || successes < 1.0))
      || (!microsoft && failures + successes <= 1.0)
      || !(0.0..=1.0).contains(&p)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(if cumulative {
      1.0 - lo_beta_dist(1.0 - p, failures + 1.0, successes)
    } else {
      let mut factor = p.powf(successes);
      for i in 0..failures as u32 {
        factor *= (f64::from(i) + successes) / f64::from(i + 1) * (1.0 - p);
      }
      factor
    }))
  }

  pub(crate) fn evaluate_norm_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=4).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(x) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(mean) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(sigma) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let cumulative = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    if sigma <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(if cumulative {
      lo_integral_phi((x - mean) / sigma)
    } else {
      lo_phi((x - mean) / sigma) / sigma
    }))
  }

  pub(crate) fn evaluate_norm_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let p = self.number(&self.evaluate(args.first()?)?)?;
    let mean = self.number(&self.evaluate(args.get(1)?)?)?;
    let sigma = self.number(&self.evaluate(args.get(2)?)?)?;
    if sigma <= 0.0 || !(0.0..=1.0).contains(&p) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if p == 0.0 || p == 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    Some(FormulaValue::Number(
      Normal::new(mean, sigma).ok()?.inverse_cdf(p),
    ))
  }

  pub(crate) fn evaluate_norm_s_inv(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(p) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if !(0.0..=1.0).contains(&p) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if p == 0.0 || p == 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    Some(FormulaValue::Number(norm_s_inv(p)))
  }

  pub(crate) fn evaluate_norm_s_dist(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let z = self.number(&self.evaluate(args.first()?)?)?;
    let cumulative = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    Some(FormulaValue::Number(if cumulative {
      lo_integral_phi(z)
    } else {
      lo_phi(z)
    }))
  }

  pub(crate) fn evaluate_percent_rank(
    &self,
    args: &[FormulaAst<'doc>],
    kind: PercentileKind,
  ) -> Option<FormulaValue<'doc>> {
    let values = self.value_numbers(&self.evaluate(args.first()?)?);
    let x = self.number(&self.evaluate(args.get(1)?)?)?;
    let significance = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(approx_floor)
      .unwrap_or(3.0);
    if significance < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    percent_rank(values, x, significance, kind)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  pub(crate) fn evaluate_poisson_dist(
    &self,
    args: &[FormulaAst<'doc>],
    odff: bool,
  ) -> Option<FormulaValue<'doc>> {
    let min_args = if odff { 2 } else { 3 };
    if args.len() < min_args || args.len() > 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let x = self.number(&self.evaluate(args.first()?)?)?.floor();
    let lambda = self.number(&self.evaluate(args.get(1)?)?)?;
    let cumulative = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    if x < 0.0 || lambda <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(lo_poisson_dist(x, lambda, cumulative)))
  }

  pub(crate) fn evaluate_fisher(
    &self,
    args: &[FormulaAst<'doc>],
    inverse: bool,
  ) -> Option<FormulaValue<'doc>> {
    let value = self.number(&self.evaluate(args.first()?)?)?;
    if inverse {
      return Some(FormulaValue::Number(value.tanh()));
    }
    if value.abs() >= 1.0 {
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    } else {
      Some(FormulaValue::Number(value.atanh()))
    }
  }

  pub(crate) fn evaluate_bessel(
    &self,
    args: &[FormulaAst<'doc>],
    kind: BesselKind,
  ) -> Option<FormulaValue<'doc>> {
    let value = self.evaluate(args.first()?)?;
    let order = self.evaluate(args.get(1)?)?;
    if self.array_context
      && (matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        || matches!(order, FormulaValue::Reference(_) | FormulaValue::Matrix(_)))
    {
      return self.map_binary_values(value, order, |evaluator, value, order| {
        evaluator.bessel_value(value, order, kind)
      });
    }
    self.bessel_value(&value, &order, kind)
  }

  pub(crate) fn bessel_value(
    &self,
    value: &FormulaValue<'doc>,
    order: &FormulaValue<'doc>,
    kind: BesselKind,
  ) -> Option<FormulaValue<'doc>> {
    let value = self.number(value)?;
    let order = approx_floor(self.number(order)?) as i32;
    if order < 0 || matches!(kind, BesselKind::K | BesselKind::Y) && value <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let result = bessel(kind, value, order);
    if result.is_finite() {
      Some(FormulaValue::Number(result))
    } else {
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    }
  }

  pub(crate) fn evaluate_fourier(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=5).contains(&args.len()) {
      return None;
    }
    let input = self.matrix_values(&self.evaluate(args.first()?)?);
    if input.is_empty() || input.first()?.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let grouped_by_column = self.truthy(&self.evaluate(args.get(1)?)?);
    let inverse = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(false);
    let polar = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(false);
    let min_magnitude = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);

    let row_count = input.len();
    let column_count = input.first()?.len();
    if input.iter().any(|row| row.len() != column_count) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if (grouped_by_column && column_count > 2) || (!grouped_by_column && row_count > 2) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }

    let real_input = if grouped_by_column {
      column_count == 1
    } else {
      row_count == 1
    };
    let point_count = if grouped_by_column {
      row_count
    } else {
      column_count
    };
    let mut values = Vec::with_capacity(point_count);
    if grouped_by_column {
      for row in &input {
        let Some(real) = self.number(&row[0]) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        let imaginary = if real_input {
          0.0
        } else {
          let Some(imaginary) = self.number(&row[1]) else {
            return Some(FormulaValue::Error(FormulaErrorValue::Value));
          };
          imaginary
        };
        values.push(Complex::new(real, imaginary));
      }
    } else {
      for (real, imaginary) in input[0].iter().zip(input.get(1).into_iter().flatten()) {
        let Some(real) = self.number(real) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        let Some(imaginary) = self.number(imaginary) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        values.push(Complex::new(real, imaginary));
      }
      if real_input {
        values.clear();
        for real in &input[0] {
          let Some(real) = self.number(real) else {
            return Some(FormulaValue::Error(FormulaErrorValue::Value));
          };
          values.push(Complex::new(real, 0.0));
        }
      }
    }

    let values = self.engine.fourier_values(values, real_input, inverse);
    let scale = if inverse {
      1.0 / point_count as f64
    } else {
      1.0
    };
    Some(FormulaValue::Matrix(
      values
        .into_iter()
        .map(|value| {
          if polar {
            let mut magnitude = value.norm();
            let mut phase = if magnitude < min_magnitude {
              magnitude = 0.0;
              0.0
            } else {
              value.im.atan2(value.re)
            };
            if inverse {
              magnitude *= scale;
            }
            if !phase.is_finite() {
              phase = 0.0;
            }
            vec![FormulaValue::Number(magnitude), FormulaValue::Number(phase)]
          } else {
            vec![
              FormulaValue::Number(value.re * scale),
              FormulaValue::Number(value.im * scale),
            ]
          }
        })
        .collect(),
    ))
  }

  pub(crate) fn evaluate_complex_part(
    &self,
    args: &[FormulaAst<'doc>],
    imaginary: bool,
  ) -> Option<FormulaValue<'doc>> {
    let Some(arg) = args.first() else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let value = parse_complex_number(&self.text(&self.evaluate(arg)?))?;
    Some(FormulaValue::Number(if imaginary {
      value.imaginary()
    } else {
      value.real()
    }))
  }

  pub(crate) fn evaluate_complex(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let real = self.number(&self.evaluate(args.first()?)?)?;
    let imaginary = self.number(&self.evaluate(args.get(1)?)?)?;
    let suffix = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.text(&value))
      .unwrap_or_else(|| "i".to_string());
    if suffix != "i" && suffix != "j" && !suffix.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::String(Cow::Owned(format_complex_result(
      FormulaComplex::new(real, imaginary, if suffix == "j" { 'j' } else { 'i' }),
    ))))
  }

  pub(crate) fn evaluate_complex_argument(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = parse_complex_number(&self.text(&self.evaluate(args.first()?)?))?;
    Some(FormulaValue::Number(value.imaginary().atan2(value.real())))
  }

  pub(crate) fn evaluate_complex_abs(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = parse_complex_number(&self.text(&self.evaluate(args.first()?)?))?;
    Some(FormulaValue::Number(value.value().norm()))
  }

  pub(crate) fn evaluate_complex_unary(
    &self,
    args: &[FormulaAst<'doc>],
    op: impl FnOnce(Complex<f64>) -> Complex<f64>,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = parse_complex_number(&self.text(&self.evaluate(args.first()?)?))?;
    let result = op(value.value());
    Some(FormulaValue::String(Cow::Owned(format_complex_result(
      FormulaComplex::from_value(result, value.suffix()),
    ))))
  }

  pub(crate) fn evaluate_complex_binary(
    &self,
    args: &[FormulaAst<'doc>],
    op: impl Fn(f64, f64) -> f64,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(left) = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| parse_complex_number(&self.text(&value)))
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(right) = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| parse_complex_number(&self.text(&value)))
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    Some(FormulaValue::String(Cow::Owned(format_complex_result(
      FormulaComplex::new(
        op(left.real(), right.real()),
        op(left.imaginary(), right.imaginary()),
        binary_suffix(left, right),
      ),
    ))))
  }

  pub(crate) fn evaluate_complex_div(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(left) = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| parse_complex_number(&self.text(&value)))
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(right) = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| parse_complex_number(&self.text(&value)))
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    if right.real() == 0.0 && right.imaginary() == 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let result = left.value() / right.value();
    if !result.re.is_finite() || !result.im.is_finite() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::String(Cow::Owned(format_complex_result(
      FormulaComplex::from_value(result, binary_suffix(left, right)),
    ))))
  }

  pub(crate) fn evaluate_complex_power(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(value) = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| parse_complex_number(&self.text(&value)))
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let power = self.number(&self.evaluate(args.get(1)?)?)?;
    let result = value.value().powf(power);
    Some(FormulaValue::String(Cow::Owned(format_complex_result(
      FormulaComplex::from_value(result, value.suffix()),
    ))))
  }

  pub(crate) fn evaluate_complex_sum_product(
    &self,
    args: &[FormulaAst<'doc>],
    product: bool,
  ) -> Option<FormulaValue<'doc>> {
    let mut total = if product {
      Complex::new(1.0, 0.0)
    } else {
      Complex::new(0.0, 0.0)
    };
    let mut suffix = 'i';
    for source in self.values(args) {
      let Some(value) = parse_complex_number(&self.text(&source)) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      if value.suffix() == 'j' {
        suffix = 'j';
      }
      if product {
        total *= value.value();
      } else {
        total += value.value();
      }
    }
    Some(FormulaValue::String(Cow::Owned(format_complex_result(
      FormulaComplex::from_value(total, suffix),
    ))))
  }

  pub(crate) fn evaluate_t_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let t = self.number(&self.evaluate(args.first()?)?)?;
    let df = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    let cumulative = self.truthy(&self.evaluate(args.get(2)?)?);
    if df < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(if cumulative {
      lo_t_dist(t, df, 4)
    } else {
      lo_t_dist(t, df, 3)
    }))
  }

  pub(crate) fn evaluate_t_dist_tails(
    &self,
    args: &[FormulaAst<'doc>],
    tails: i32,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let t = self.number(&self.evaluate(args.first()?)?)?;
    let df = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    if df < 1.0 || (tails == 2 && t < 0.0) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let result = lo_t_dist(t, df, tails);
    Some(FormulaValue::Number(match tails {
      1 if t < 0.0 => 1.0 - result,
      1 | 2 => result,
      _ => return Some(FormulaValue::Error(FormulaErrorValue::Num)),
    }))
  }

  pub(crate) fn evaluate_t_dist_legacy(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let t = self.number(&self.evaluate(args.first()?)?)?;
    let df = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    let tails = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?);
    if t < 0.0 || df < 1.0 || (tails != 1.0 && tails != 2.0) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(lo_t_dist(t, df, tails as i32)))
  }

  pub(crate) fn evaluate_t_inv(
    &self,
    args: &[FormulaAst<'doc>],
    two_tailed: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let p = self.number(&self.evaluate(args.first()?)?)?;
    let df = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    if df < 1.0 || p <= 0.0 || p > 1.0 || (!two_tailed && p == 1.0) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if two_tailed {
      return match lo_iterate_inverse(|x| p - lo_t_dist(x, df, 2), df * 0.5, df) {
        Ok(value) => Some(FormulaValue::Number(value)),
        Err(error) => Some(FormulaValue::Error(special_error_value(error))),
      };
    }
    if p < 0.5 {
      match lo_iterate_inverse(|x| 1.0 - p - lo_t_dist(x, df, 4), df * 0.5, df) {
        Ok(value) => Some(FormulaValue::Number(-value)),
        Err(error) => Some(FormulaValue::Error(special_error_value(error))),
      }
    } else {
      match lo_iterate_inverse(|x| p - lo_t_dist(x, df, 4), df * 0.5, df) {
        Ok(value) => Some(FormulaValue::Number(value)),
        Err(error) => Some(FormulaValue::Error(special_error_value(error))),
      }
    }
  }

  pub(crate) fn evaluate_t_test(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let left = self.value_numbers(&self.evaluate(args.first()?)?);
    let right = self.value_numbers(&self.evaluate(args.get(1)?)?);
    let tails = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?) as i32;
    let test_type = approx_floor(self.number(&self.evaluate(args.get(3)?)?)?) as i32;
    if left.is_empty()
      || right.is_empty()
      || !(1..=2).contains(&tails)
      || !(1..=3).contains(&test_type)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mean_left = mean(&left)?;
    let mean_right = mean(&right)?;
    let var_left = variance_slice(&left, true)?;
    let var_right = variance_slice(&right, true)?;
    let (t, df) = if test_type == 1 {
      let diffs = left
        .iter()
        .zip(right.iter())
        .map(|(left, right)| left - right)
        .collect::<Vec<_>>();
      let mean_diff = mean(&diffs)?;
      let sd_diff = variance_slice(&diffs, true)?.sqrt();
      if sd_diff == 0.0 {
        return Some(FormulaValue::Error(FormulaErrorValue::Div0));
      }
      (
        mean_diff.abs() / (sd_diff / (diffs.len() as f64).sqrt()),
        diffs.len() as f64 - 1.0,
      )
    } else if test_type == 2 {
      let pooled = ((left.len() - 1) as f64 * var_left + (right.len() - 1) as f64 * var_right)
        / (left.len() + right.len() - 2) as f64;
      (
        (mean_left - mean_right).abs()
          / (pooled * (1.0 / left.len() as f64 + 1.0 / right.len() as f64)).sqrt(),
        (left.len() + right.len() - 2) as f64,
      )
    } else {
      let se = (var_left / left.len() as f64 + var_right / right.len() as f64).sqrt();
      let df_num = (var_left / left.len() as f64 + var_right / right.len() as f64).powi(2);
      let df_den = var_left.powi(2) / ((left.len() as f64).powi(2) * (left.len() - 1) as f64)
        + var_right.powi(2) / ((right.len() as f64).powi(2) * (right.len() - 1) as f64);
      ((mean_left - mean_right).abs() / se, df_num / df_den)
    };
    Some(FormulaValue::Number(lo_t_dist(t, df, tails)))
  }

  pub(crate) fn evaluate_weibull_dist(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let x_value = self.evaluate(args.first()?)?;
    let alpha_value = self.evaluate(args.get(1)?)?;
    let beta_value = self.evaluate(args.get(2)?)?;
    let cumulative_value = self.evaluate(args.get(3)?)?;
    let x_matrix = self.matrix_values(&x_value);
    let alpha_matrix = self.matrix_values(&alpha_value);
    let beta_matrix = self.matrix_values(&beta_value);
    let rows = x_matrix
      .len()
      .max(alpha_matrix.len())
      .max(beta_matrix.len());
    let columns = x_matrix
      .first()
      .map(Vec::len)
      .unwrap_or(1)
      .max(alpha_matrix.first().map(Vec::len).unwrap_or(1))
      .max(beta_matrix.first().map(Vec::len).unwrap_or(1));
    let cumulative = self.truthy(&cumulative_value);
    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let x = self.number(matrix_item(&x_matrix, row, column)?)?;
        let alpha = self.number(matrix_item(&alpha_matrix, row, column)?)?;
        let beta = self.number(matrix_item(&beta_matrix, row, column)?)?;
        if x < 0.0 || alpha <= 0.0 || beta <= 0.0 {
          result_row.push(FormulaValue::Error(FormulaErrorValue::Num));
          continue;
        }
        let dist = Weibull::new(alpha, beta).ok()?;
        result_row.push(FormulaValue::Number(if cumulative {
          dist.cdf(x)
        } else {
          dist.pdf(x)
        }));
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      result.into_iter().next()?.into_iter().next()
    } else {
      Some(FormulaValue::Matrix(result))
    }
  }

  pub(crate) fn evaluate_z_test(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = self.value_numbers_from_ast(args.first()?);
    let x = self.number(&self.evaluate(args.get(1)?)?)?;
    let sigma = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or_else(|| variance_slice(&values, true).unwrap_or(0.0).sqrt());
    if values.len() <= 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    if sigma <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let z = (mean(&values)? - x) / (sigma / (values.len() as f64).sqrt());
    Some(FormulaValue::Number(1.0 - norm_s_dist(z)))
  }

  pub(crate) fn evaluate_standardize(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(x) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(mean) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(sigma) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if sigma <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number((x - mean) / sigma))
  }

  pub(crate) fn evaluate_networkdays(
    &self,
    args: &[FormulaAst<'doc>],
    intl: bool,
  ) -> Option<FormulaValue<'doc>> {
    let mut start = self.number(&self.evaluate(args.first()?)?)?.floor() as i64;
    let mut end = self.number(&self.evaluate(args.get(1)?)?)?.floor() as i64;
    let weekend_arg = if intl {
      args.get(2).and_then(|arg| self.evaluate(arg))
    } else {
      args.get(3).and_then(|arg| self.evaluate(arg))
    };
    let weekend = if intl {
      weekend_mask(weekend_arg.as_ref(), false, self)
    } else if args.len() == 4 {
      old_networkdays_weekend_mask(weekend_arg.as_ref(), self)
    } else {
      weekend_mask(None, false, self)
    };
    let Some(weekend) = weekend else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let holiday_arg = args
      .get(if intl { 3 } else { 2 })
      .and_then(|arg| self.evaluate(arg));
    let holidays = holiday_serials(holiday_arg.as_ref(), self);
    let reverse = start > end;
    if reverse {
      std::mem::swap(&mut start, &mut end);
    }
    let mut count = 0i64;
    for serial in start..=end {
      if !weekend[weekday_index_from_serial(serial)] && holidays.binary_search(&serial).is_err() {
        count += 1;
      }
    }
    Some(FormulaValue::Number(if reverse {
      -(count as f64)
    } else {
      count as f64
    }))
  }

  pub(crate) fn evaluate_workday(
    &self,
    args: &[FormulaAst<'doc>],
    intl: bool,
  ) -> Option<FormulaValue<'doc>> {
    let Some(mut date) = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(|value| value.floor() as i64)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(mut days) = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(|value| value.floor() as i64)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let weekend_arg = if intl {
      match args.get(2) {
        Some(arg) if is_missing_argument(arg) => None,
        Some(arg) => {
          let value = self.evaluate(arg)?;
          match value {
            FormulaValue::Reference(reference) => {
              if reference.range.cell_count_hint() != 1 {
                return Some(FormulaValue::Error(FormulaErrorValue::Value));
              }
              Some(self.scalar_reference_value(&reference))
            }
            FormulaValue::RefList(_) => {
              return Some(FormulaValue::Error(FormulaErrorValue::Value));
            }
            value => Some(value),
          }
        }
        None => None,
      }
    } else {
      None
    };
    let Some(weekend) = weekend_mask(weekend_arg.as_ref(), true, self) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let holiday_arg = args
      .get(if intl { 3 } else { 2 })
      .and_then(|arg| self.evaluate(arg));
    let holidays = holiday_serials(holiday_arg.as_ref(), self);
    if days == 0 {
      return Some(FormulaValue::Number(date as f64));
    }
    let step = if days > 0 { 1 } else { -1 };
    while days != 0 {
      date += step;
      if weekend[weekday_index_from_serial(date)] {
        continue;
      }
      if holidays.binary_search(&date).is_ok() {
        continue;
      }
      days -= step;
    }
    Some(FormulaValue::Number(date as f64))
  }

  pub(crate) fn evaluate_subtotal(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(function) = self.number_arg(args, 0).map(|value| value as i32) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let values = args
      .get(1..)
      .unwrap_or_default()
      .iter()
      .filter_map(|arg| self.evaluate(arg))
      .collect::<Vec<_>>();
    let options = AggregateOptions {
      ignore_hidden: function >= 100,
      ignore_filtered: true,
      ignore_errors: false,
      ignore_nested: true,
    };
    aggregate_function_value(self, function.rem_euclid(100), &values, None, options).map(|result| {
      match result {
        Ok(value) => FormulaValue::Number(value),
        Err(error) => FormulaValue::Error(error),
      }
    })
  }

  pub(crate) fn evaluate_aggregate(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(function) = self.number_arg(args, 0).map(|value| value as i32) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(options_arg) = self.number_arg(args, 1).map(|value| value as i32) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if !(1..=19).contains(&function) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let Some(options) = aggregate_options(options_arg) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    if (14..=19).contains(&function) && args.len() < 4 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let mut evaluated = Vec::with_capacity(args.len().saturating_sub(2));
    for arg in args.get(2..).unwrap_or_default() {
      evaluated.push(self.evaluate(arg)?);
    }
    let k = if (14..=19).contains(&function) {
      evaluated.get(1).and_then(|value| self.number(value))
    } else {
      None
    };
    let data = if (14..=19).contains(&function) {
      evaluated.get(0..1)?
    } else {
      evaluated.as_slice()
    };
    aggregate_function_value(self, function, data, k, options)
      .map(|result| match result {
        Ok(value) => FormulaValue::Number(value),
        Err(error) => FormulaValue::Error(error),
      })
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_db(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=5).contains(&args.len()) {
      return None;
    }
    let cost = self.number(&self.evaluate(args.first()?)?)?;
    let salvage = self.number(&self.evaluate(args.get(1)?)?)?;
    let life = self.number(&self.evaluate(args.get(2)?)?)?;
    let period = self.number(&self.evaluate(args.get(3)?)?)?;
    let months = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(approx_floor)
      .unwrap_or(12.0);
    if !(1.0..=12.0).contains(&months)
      || life > 1200.0
      || salvage < 0.0
      || period > life + 1.0
      || salvage > cost
      || cost <= 0.0
      || life <= 0.0
      || period <= 0.0
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(financial_db(
      cost, salvage, life, period, months,
    )))
  }

  pub(crate) fn evaluate_price(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(6..=7).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    if has_missing_required_argument(args, &[0, 1, 2, 3, 4, 5]) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let Some(settle) = self
      .date_number_from_value(&self.evaluate(args.first()?)?)
      .map(|value| value.floor() as i32)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(maturity) = self
      .date_number_from_value(&self.evaluate(args.get(1)?)?)
      .map(|value| value.floor() as i32)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(rate) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(yield_value) = self.number_arg(args, 3) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(redemption) = self.number_arg(args, 4) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(frequency) = self.number_arg(args, 5).map(|value| value.floor() as i32) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let basis = args
      .get(6)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0)
      .floor() as i32;
    if yield_value < 0.0
      || rate < 0.0
      || redemption <= 0.0
      || !is_coupon_frequency(frequency)
      || settle >= maturity
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
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(
      FormulaErrorValue::IllegalArgument,
    )))
  }

  pub(crate) fn evaluate_yield(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(6..=7).contains(&args.len()) {
      return None;
    }
    if has_missing_required_argument(args, &[0, 1, 2, 3, 4, 5]) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let coupon = self.number_arg(args, 2)?;
    let price = self.number_arg(args, 3)?;
    let redemption = self.number_arg(args, 4)?;
    let frequency = self.number_arg(args, 5)?.floor() as i32;
    let basis = self.optional_basis(args, 6);
    if coupon < 0.0
      || price <= 0.0
      || redemption <= 0.0
      || !is_coupon_frequency(frequency)
      || settle >= maturity
      || !(0..=4).contains(&basis)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    finance_yield(
      settle, maturity, coupon, price, redemption, frequency, basis,
    )
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(
      FormulaErrorValue::IllegalArgument,
    )))
  }

  pub(crate) fn evaluate_pricedisc(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=5).contains(&args.len()) {
      return None;
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let discount = self.number_arg(args, 2)?;
    let redemption = self.number_arg(args, 3)?;
    let basis = self.optional_basis(args, 4);
    if discount <= 0.0 || redemption <= 0.0 || settle >= maturity || !(0..=4).contains(&basis) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    finance_year_diff(settle, maturity, basis)
      .map(|diff| redemption * (1.0 - discount * diff))
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_pricemat(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(5..=6).contains(&args.len()) {
      return None;
    }
    if has_missing_required_argument(args, &[0, 1, 2, 3, 4]) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let issue = self.date_arg(args, 2)?;
    let rate = self.number_arg(args, 3)?;
    let yield_value = self.number_arg(args, 4)?;
    let basis = self.optional_basis(args, 5);
    if rate < 0.0 || yield_value < 0.0 || settle >= maturity || !(0..=4).contains(&basis) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let iss_mat = yearfrac(issue, maturity, basis)?;
    let iss_set = yearfrac(issue, settle, basis)?;
    let set_mat = yearfrac(settle, maturity, basis)?;
    Some(FormulaValue::Number(
      ((1.0 + iss_mat * rate) / (1.0 + set_mat * yield_value) - iss_set * rate) * 100.0,
    ))
  }

  pub(crate) fn evaluate_yielddisc(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=5).contains(&args.len()) {
      return None;
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let price = self.number_arg(args, 2)?;
    let redemption = self.number_arg(args, 3)?;
    let basis = self.optional_basis(args, 4);
    if price <= 0.0 || redemption <= 0.0 || settle >= maturity || !(0..=4).contains(&basis) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    yearfrac(settle, maturity, basis)
      .map(|frac| ((redemption / price) - 1.0) / frac)
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_accrint(
    &self,
    args: &[FormulaAst<'doc>],
    at_maturity: bool,
  ) -> Option<FormulaValue<'doc>> {
    if at_maturity {
      if !(3..=5).contains(&args.len()) {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      }
      let Some(issue) = self.date_arg(args, 0) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      let Some(settle) = self.date_arg(args, 1) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      let Some(rate) = self.number_arg(args, 2) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      let par = self.optional_number_arg(args, 3, 1000.0);
      let basis = self.optional_basis(args, 4);
      if rate <= 0.0 || par <= 0.0 || issue >= settle || !(0..=4).contains(&basis) {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      return finance_year_diff(issue, settle, basis)
        .map(|diff| par * rate * diff)
        .filter(|value| value.is_finite())
        .map(FormulaValue::Number)
        .or(Some(FormulaValue::Error(
          FormulaErrorValue::IllegalArgument,
        )));
    }
    if !(5..=7).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let issue = self.date_arg(args, 0)?;
    let settle = self.date_arg(args, 2)?;
    let rate = self.number_arg(args, 3)?;
    let par = self.optional_number_arg(args, 4, 1000.0);
    let frequency = self.number_arg(args, 5)?.floor() as i32;
    let basis = self.optional_basis(args, 6);
    if rate <= 0.0
      || par <= 0.0
      || !is_coupon_frequency(frequency)
      || issue >= settle
      || !(0..=4).contains(&basis)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    finance_year_diff(issue, settle, basis)
      .map(|diff| par * rate * diff)
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_oddlprice(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(7..=8).contains(&args.len()) {
      return None;
    }
    if has_missing_required_argument(args, &[0, 1, 2, 3, 4, 5, 6]) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let last_interest = self.date_arg(args, 2)?;
    let rate = self.number_arg(args, 3)?;
    let yield_value = self.number_arg(args, 4)?;
    let redemption = self.number_arg(args, 5)?;
    let frequency = self.number_arg(args, 6)?.floor() as i32;
    let basis = self.optional_basis(args, 7);
    if rate <= 0.0
      || yield_value < 0.0
      || redemption <= 0.0
      || !is_coupon_frequency(frequency)
      || maturity <= settle
      || settle <= last_interest
      || !(0..=4).contains(&basis)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    financial_oddlprice(OddPeriodArgs {
      settle,
      maturity,
      last_coupon: last_interest,
      rate,
      value: yield_value,
      redemption,
      frequency,
      basis,
    })
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(
      FormulaErrorValue::IllegalArgument,
    )))
  }

  pub(crate) fn evaluate_oddlyield(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(7..=8).contains(&args.len()) {
      return None;
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let last_interest = self.date_arg(args, 2)?;
    let rate = self.number_arg(args, 3)?;
    let price = self.number_arg(args, 4)?;
    let redemption = self.number_arg(args, 5)?;
    let frequency = self.number_arg(args, 6)?.floor() as i32;
    let basis = self.optional_basis(args, 7);
    if rate <= 0.0
      || price <= 0.0
      || redemption <= 0.0
      || !is_coupon_frequency(frequency)
      || maturity <= settle
      || settle <= last_interest
      || !(0..=4).contains(&basis)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    financial_oddlyield(OddPeriodArgs {
      settle,
      maturity,
      last_coupon: last_interest,
      rate,
      value: price,
      redemption,
      frequency,
      basis,
    })
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(
      FormulaErrorValue::IllegalArgument,
    )))
  }

  pub(crate) fn evaluate_amorlinc(
    &self,
    args: &[FormulaAst<'doc>],
    degressive: bool,
  ) -> Option<FormulaValue<'doc>> {
    if !(6..=7).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let values = args
      .iter()
      .map(|arg| self.evaluate(arg))
      .collect::<Option<Vec<_>>>()?;
    if values.iter().any(|value| {
      let matrix = self.matrix_values(value);
      matrix.iter().map(Vec::len).sum::<usize>() > 1
    }) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let cost = self.number(values.first()?)?;
    let date = self.date_number_from_value(values.get(1)?)?.floor() as i32;
    let first_period = self.date_number_from_value(values.get(2)?)?.floor() as i32;
    let residual = self.number(values.get(3)?)?;
    let period = self.number(values.get(4)?)?;
    let rate = self.number(values.get(5)?)?;
    let basis = values
      .get(6)
      .and_then(|value| self.number(value))
      .unwrap_or(0.0)
      .floor() as i32;
    if cost <= 0.0
      || residual < 0.0
      || period < 0.0
      || rate <= 0.0
      || date > first_period
      || !(0..=4).contains(&basis)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let result = if degressive {
      financial_amordegrc(cost, date, first_period, residual, period, rate, basis)
    } else {
      financial_amorlinc(cost, date, first_period, residual, period, rate, basis)
    };
    result
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_disc(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=5).contains(&args.len()) {
      return None;
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let price = self.number_arg(args, 2)?;
    let redemption = self.number_arg(args, 3)?;
    let basis = self.optional_basis(args, 4);
    if price <= 0.0 || redemption <= 0.0 || settle >= maturity || !(0..=4).contains(&basis) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    yearfrac(settle, maturity, basis)
      .map(|frac| (1.0 - price / redemption) / frac)
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_received(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=5).contains(&args.len()) {
      return None;
    }
    if has_missing_required_argument(args, &[0, 1, 2, 3]) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let investment = self.number_arg(args, 2)?;
    let discount = self.number_arg(args, 3)?;
    let basis = self.optional_basis(args, 4);
    if investment <= 0.0 || discount <= 0.0 || settle >= maturity || !(0..=4).contains(&basis) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    finance_year_diff(settle, maturity, basis)
      .map(|diff| investment / (1.0 - discount * diff))
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_intrate(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=5).contains(&args.len()) {
      return None;
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let investment = self.number_arg(args, 2)?;
    let redemption = self.number_arg(args, 3)?;
    let basis = self.optional_basis(args, 4);
    if investment <= 0.0 || redemption <= 0.0 || settle >= maturity || !(0..=4).contains(&basis) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    finance_year_diff(settle, maturity, basis)
      .map(|diff| ((redemption / investment) - 1.0) / diff)
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_mduration(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(5..=6).contains(&args.len()) {
      return None;
    }
    if has_missing_required_argument(args, &[0, 1, 2, 3, 4]) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let coupon = self.number_arg(args, 2)?;
    let yield_value = self.number_arg(args, 3)?;
    let frequency = self.number_arg(args, 4)?.floor() as i32;
    let basis = self.optional_basis(args, 5);
    if coupon < 0.0
      || yield_value < 0.0
      || !is_coupon_frequency(frequency)
      || !(0..=4).contains(&basis)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    finance_duration(settle, maturity, coupon, yield_value, frequency, basis)
      .map(|duration| duration / (1.0 + yield_value / f64::from(frequency)))
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_tbilleq(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(settle) = self.date_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(maturity) = self.date_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(discount) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let maturity = maturity + 1;
    let diff = days360(settle, maturity, false)?;
    if discount <= 0.0 || settle >= maturity || diff > 360 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(
      (365.0 * discount) / (360.0 - discount * f64::from(diff)),
    ))
  }

  pub(crate) fn evaluate_tbillprice(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(settle) = self.date_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(maturity) = self.date_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(discount) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if discount <= 0.0 || settle > maturity {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let fraction = yearfrac(settle, maturity + 1, 0)?;
    if fraction.fract() == 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(100.0 * (1.0 - discount * fraction)))
  }

  pub(crate) fn evaluate_tbillyield(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(settle) = self.date_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(maturity) = self.date_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(price) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let diff = days360(settle, maturity, false)? + 1;
    if price <= 0.0 || settle >= maturity || diff > 360 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(
      ((100.0 / price) - 1.0) / f64::from(diff) * 360.0,
    ))
  }

  pub(crate) fn evaluate_coupon(
    &self,
    args: &[FormulaAst<'doc>],
    function: CouponFunction,
  ) -> Option<FormulaValue<'doc>> {
    if !(3..=4).contains(&args.len()) {
      return None;
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let frequency = self.number_arg(args, 2)?.floor() as i32;
    let basis = self.optional_basis(args, 3);
    if settle >= maturity || !is_coupon_frequency(frequency) || !(0..=4).contains(&basis) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let result = match function {
      CouponFunction::Days => finance_coupdays(settle, maturity, frequency, basis),
      CouponFunction::DayBs => finance_coupdaybs(settle, maturity, frequency, basis),
      CouponFunction::DaysNc => finance_coupdaysnc(settle, maturity, frequency, basis),
      CouponFunction::Ncd => finance_coupncd(settle, maturity, frequency, basis).map(f64::from),
      CouponFunction::Num => finance_coupnum(settle, maturity, frequency, basis),
      CouponFunction::Pcd => finance_couppcd(settle, maturity, frequency, basis).map(f64::from),
    };
    result
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_pv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=5).contains(&args.len()) {
      return None;
    }
    let rate = self.number_arg(args, 0)?;
    let nper = self.number_arg(args, 1)?;
    let pmt = self.number_arg(args, 2)?;
    let fv = self.number_arg(args, 3).unwrap_or(0.0);
    let pay_in_advance = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    Some(FormulaValue::Number(financial_pv(
      rate,
      nper,
      pmt,
      fv,
      pay_in_advance,
    )))
  }

  pub(crate) fn evaluate_nper(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=5).contains(&args.len()) {
      return None;
    }
    let rate = self.number_arg(args, 0)?;
    let pmt = self.number_arg(args, 1)?;
    let pv = self.number_arg(args, 2)?;
    let fv = self.number_arg(args, 3).unwrap_or(0.0);
    let pay_in_advance = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    financial_nper(rate, pmt, pv, fv, pay_in_advance)
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_rri(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let periods = self.number_arg(args, 0)?;
    let present = self.number_arg(args, 1)?;
    let future = self.number_arg(args, 2)?;
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

  pub(crate) fn evaluate_pduration(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let rate = self.number_arg(args, 0)?;
    let present = self.number_arg(args, 1)?;
    let future = self.number_arg(args, 2)?;
    if rate <= 0.0 || present <= 0.0 || future <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(
      (future / present).ln() / (1.0 + rate).ln(),
    ))
  }

  pub(crate) fn evaluate_seriessum(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 4 {
      return None;
    }
    let x = self.number_arg(args, 0)?;
    let mut exponent = self.number_arg(args, 1)?;
    let step = self.number_arg(args, 2)?;
    if x == 0.0 && exponent == 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let mut result = 0.0;
    if x != 0.0 {
      for coefficient in self.value_numbers(&self.evaluate(args.get(3)?)?) {
        result += coefficient * x.powf(exponent);
        exponent += step;
      }
    }
    Some(FormulaValue::Number(result))
  }

  pub(crate) fn evaluate_eastersunday(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    if is_multicell_scalar_argument(&value) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let Some(year_number) = self.number(&value) else {
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

  pub(crate) fn numeric_binary(
    &self,
    left: FormulaValue<'doc>,
    right: FormulaValue<'doc>,
    op: impl Fn(f64, f64) -> f64 + Copy,
  ) -> Option<FormulaValue<'doc>> {
    if matches!(left, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
      || matches!(right, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
    {
      return self.map_numeric_binary(left, right, op);
    }
    let Some(left) = arithmetic_matrix_number(&left) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let Some(right) = arithmetic_matrix_number(&right) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    Some(FormulaValue::Number(normalize_formula_number(op(
      left, right,
    ))))
  }

  pub(crate) fn map_numeric_binary(
    &self,
    left: FormulaValue<'doc>,
    right: FormulaValue<'doc>,
    op: impl Fn(f64, f64) -> f64 + Copy,
  ) -> Option<FormulaValue<'doc>> {
    let left_matrix = self.matrix_values(&left);
    let right_matrix = self.matrix_values(&right);
    let left_rows = left_matrix.len();
    let left_columns = left_matrix.first().map_or(0, Vec::len);
    let right_rows = right_matrix.len();
    let right_columns = right_matrix.first().map_or(0, Vec::len);
    if left_rows == 0 || left_columns == 0 || right_rows == 0 || right_columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let rows = matrix_binary_extent(left_rows, right_rows);
    let columns = matrix_binary_extent(left_columns, right_columns);

    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let left = &left_matrix[row.min(left_rows - 1)][column.min(left_columns - 1)];
        let right = &right_matrix[row.min(right_rows - 1)][column.min(right_columns - 1)];
        result_row.push(if let Some(error) = propagate_binary_error(left, right) {
          FormulaValue::Error(error)
        } else if let Some((left, right)) =
          arithmetic_operator_matrix_number(left).zip(arithmetic_operator_matrix_number(right))
        {
          FormulaValue::Number(normalize_formula_number(op(left, right)))
        } else {
          FormulaValue::Error(FormulaErrorValue::Value)
        });
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn map_binary_values(
    &self,
    left: FormulaValue<'doc>,
    right: FormulaValue<'doc>,
    op: impl Fn(&Self, &FormulaValue<'doc>, &FormulaValue<'doc>) -> Option<FormulaValue<'doc>> + Copy,
  ) -> Option<FormulaValue<'doc>> {
    let left_matrix = self.matrix_values(&left);
    let right_matrix = self.matrix_values(&right);
    let left_rows = left_matrix.len();
    let left_columns = left_matrix.first().map_or(0, Vec::len);
    let right_rows = right_matrix.len();
    let right_columns = right_matrix.first().map_or(0, Vec::len);
    if left_rows == 0 || left_columns == 0 || right_rows == 0 || right_columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let rows = matrix_binary_extent(left_rows, right_rows);
    let columns = matrix_binary_extent(left_columns, right_columns);

    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let left = &left_matrix[row.min(left_rows - 1)][column.min(left_columns - 1)];
        let right = &right_matrix[row.min(right_rows - 1)][column.min(right_columns - 1)];
        result_row.push(if let Some(error) = propagate_binary_error(left, right) {
          FormulaValue::Error(error)
        } else {
          op(self, left, right)?
        });
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn map_ternary_values(
    &self,
    first: FormulaValue<'doc>,
    second: FormulaValue<'doc>,
    third: FormulaValue<'doc>,
    op: impl Fn(
      &Self,
      &FormulaValue<'doc>,
      &FormulaValue<'doc>,
      &FormulaValue<'doc>,
    ) -> Option<FormulaValue<'doc>>
    + Copy,
  ) -> Option<FormulaValue<'doc>> {
    let first_matrix = self.matrix_values(&first);
    let second_matrix = self.matrix_values(&second);
    let third_matrix = self.matrix_values(&third);
    let first_rows = first_matrix.len();
    let first_columns = first_matrix.first().map_or(0, Vec::len);
    let second_rows = second_matrix.len();
    let second_columns = second_matrix.first().map_or(0, Vec::len);
    let third_rows = third_matrix.len();
    let third_columns = third_matrix.first().map_or(0, Vec::len);
    if first_rows == 0
      || first_columns == 0
      || second_rows == 0
      || second_columns == 0
      || third_rows == 0
      || third_columns == 0
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let rows = matrix_binary_extent(matrix_binary_extent(first_rows, second_rows), third_rows);
    let columns = matrix_binary_extent(
      matrix_binary_extent(first_columns, second_columns),
      third_columns,
    );

    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let first = &first_matrix[row.min(first_rows - 1)][column.min(first_columns - 1)];
        let second = &second_matrix[row.min(second_rows - 1)][column.min(second_columns - 1)];
        let third = &third_matrix[row.min(third_rows - 1)][column.min(third_columns - 1)];
        result_row.push(
          if let Some(error) = first_error_in_values(&[first, second, third]) {
            FormulaValue::Error(error)
          } else {
            op(self, first, second, third)?
          },
        );
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn map_unary_values(
    &self,
    value: FormulaValue<'doc>,
    op: impl Fn(&Self, &FormulaValue<'doc>) -> Option<FormulaValue<'doc>> + Copy,
  ) -> Option<FormulaValue<'doc>> {
    let matrix = self.matrix_values(&value);
    let rows = matrix.len();
    let columns = matrix.first().map_or(0, Vec::len);
    if rows == 0 || columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }

    let mut result = Vec::with_capacity(rows);
    for row in matrix {
      let mut result_row = Vec::with_capacity(columns);
      for value in row {
        result_row.push(op(self, &value)?);
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn values<'b>(
    &'b self,
    args: &'b [FormulaAst<'doc>],
  ) -> impl Iterator<Item = FormulaValue<'doc>> + 'b {
    args
      .iter()
      .filter_map(|arg| self.evaluate(arg))
      .flat_map(|value| match value {
        FormulaValue::Reference(range) => self.range_values(&range),
        FormulaValue::RefList(ranges) => ranges
          .into_iter()
          .flat_map(|range| self.range_values(&range))
          .collect(),
        FormulaValue::Matrix(rows) => rows.into_iter().flatten().collect(),
        value => vec![value],
      })
  }

  pub(crate) fn numeric_values<'b>(
    &'b self,
    args: &'b [FormulaAst<'doc>],
  ) -> impl Iterator<Item = f64> + 'b {
    // Source: LibreOffice ScInterpreter::GetNumberSequenceArray and
    // ScInterpreter::CalculateSkew use ScValueIterator for range/ref-list
    // arguments, which yields numeric cells and skips empty/text cells. Direct
    // scalar arguments still use normal value conversion, so a direct blank is
    // zero while a blank inside a range is ignored.
    args
      .iter()
      .flat_map(|arg| self.value_numbers_from_ast(arg).into_iter())
  }

  pub(crate) fn numeric_aggregate(
    &self,
    args: &[FormulaAst<'doc>],
    text_error: bool,
  ) -> std::result::Result<NumericAggregate, FormulaErrorValue> {
    let mut values = Vec::new();
    for arg in args {
      let value = self.evaluate(arg).ok_or(FormulaErrorValue::Unknown)?;
      match value {
        FormulaValue::Reference(reference) => {
          self.push_range_numeric_aggregate_values(&reference, &mut values)?;
        }
        FormulaValue::RefList(ranges) => {
          for range in ranges {
            self.push_range_numeric_aggregate_values(&range, &mut values)?;
          }
        }
        FormulaValue::Matrix(rows) => {
          for value in rows.into_iter().flatten() {
            match value {
              FormulaValue::Blank | FormulaValue::String(_) => {}
              value => self.push_direct_numeric_aggregate_value(value, text_error, &mut values)?,
            }
          }
        }
        value => self.push_direct_numeric_aggregate_value(value, text_error, &mut values)?,
      }
    }
    Ok(NumericAggregate { values })
  }

  pub(crate) fn count_numbers(&self, args: &[FormulaAst<'doc>]) -> usize {
    let mut count = 0usize;
    let array_evaluator = self.with_array_context();
    for arg in args {
      let ranges = self.reference_ranges_from_ast(arg);
      if !ranges.is_empty() {
        for range in ranges {
          count += self.count_numbers_in_range(&range);
        }
        continue;
      }
      let Some(value) = array_evaluator.evaluate(arg) else {
        continue;
      };
      match value {
        FormulaValue::Reference(reference) => {
          count += self.count_numbers_in_range(&reference);
        }
        FormulaValue::RefList(ranges) => {
          for range in ranges {
            count += self.count_numbers_in_range(&range);
          }
        }
        FormulaValue::Matrix(rows) => {
          count += rows
            .iter()
            .flatten()
            .filter(|value| matches!(value, FormulaValue::Number(_) | FormulaValue::Boolean(_)))
            .count();
        }
        FormulaValue::Number(_) | FormulaValue::Boolean(_) => count += 1,
        FormulaValue::String(value) => {
          if value.trim().parse::<f64>().is_ok() {
            count += 1;
          }
        }
        FormulaValue::Blank | FormulaValue::Error(_) => {}
      }
    }
    count
  }

  pub(crate) fn count_numbers_in_range(&self, reference: &QualifiedRange<'doc>) -> usize {
    self
      .range_values(reference)
      .iter()
      .filter(|value| matches!(value, FormulaValue::Number(_) | FormulaValue::Boolean(_)))
      .count()
  }

  pub(crate) fn count_all_values(&self, args: &[FormulaAst<'doc>]) -> usize {
    let mut count = 0usize;
    let array_evaluator = self.with_array_context();
    for arg in args {
      if is_missing_argument(arg) {
        count += 1;
        continue;
      }
      let ranges = self.reference_ranges_from_ast(arg);
      if !ranges.is_empty() {
        for range in ranges {
          count += self.count_all_values_in_range(&range);
        }
        continue;
      }
      let Some(value) = array_evaluator.evaluate(arg) else {
        continue;
      };
      match value {
        FormulaValue::Reference(reference) => {
          count += self.count_all_values_in_range(&reference);
        }
        FormulaValue::RefList(ranges) => {
          for range in ranges {
            count += self.count_all_values_in_range(&range);
          }
        }
        FormulaValue::Matrix(rows) => {
          count += rows
            .iter()
            .flatten()
            .filter(|value| !matches!(value, FormulaValue::Blank))
            .count();
        }
        FormulaValue::Blank => {}
        _ => count += 1,
      }
    }
    count
  }

  pub(crate) fn count_all_values_in_range(&self, reference: &QualifiedRange<'doc>) -> usize {
    self
      .range_values(reference)
      .iter()
      .filter(|value| !matches!(value, FormulaValue::Blank))
      .count()
  }

  pub(crate) fn push_range_numeric_aggregate_values(
    &self,
    reference: &QualifiedRange<'doc>,
    values: &mut Vec<f64>,
  ) -> std::result::Result<(), FormulaErrorValue> {
    for value in self.range_values(reference) {
      match value {
        FormulaValue::Number(value) => values.push(value),
        FormulaValue::Boolean(value) => values.push(if value { 1.0 } else { 0.0 }),
        FormulaValue::Error(error) => return Err(error),
        FormulaValue::String(_) | FormulaValue::Blank => {}
        FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => {}
      }
    }
    Ok(())
  }

  pub(crate) fn push_direct_numeric_aggregate_value(
    &self,
    value: FormulaValue<'doc>,
    text_error: bool,
    values: &mut Vec<f64>,
  ) -> std::result::Result<(), FormulaErrorValue> {
    match value {
      FormulaValue::Number(value) => values.push(value),
      FormulaValue::Boolean(value) => values.push(if value { 1.0 } else { 0.0 }),
      FormulaValue::Blank => values.push(0.0),
      FormulaValue::String(value) => {
        if let Ok(value) = value.trim().parse::<f64>() {
          values.push(value);
        } else if text_error {
          return Err(FormulaErrorValue::Value);
        }
      }
      FormulaValue::Error(error) => return Err(error),
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_) => {}
    }
    Ok(())
  }

  pub(crate) fn numeric_args(&self, args: &[FormulaAst<'doc>]) -> Vec<f64> {
    self.numeric_values(args).collect()
  }

  pub(crate) fn median_numeric_args(&self, args: &[FormulaAst<'doc>]) -> Vec<f64> {
    let mut values = Vec::new();
    for arg in args {
      if let Some(value) = self.evaluate(arg) {
        values.extend(self.median_numbers_from_value(&value));
      }
    }
    values
  }

  pub(crate) fn median_numbers_from_value(&self, value: &FormulaValue<'doc>) -> Vec<f64> {
    match value {
      FormulaValue::Reference(reference) => self
        .range_values(reference)
        .iter()
        .filter_map(number_only)
        .collect(),
      FormulaValue::RefList(ranges) => ranges
        .iter()
        .flat_map(|range| self.range_values(range))
        .filter_map(|value| number_only(&value))
        .collect(),
      FormulaValue::Matrix(rows) => rows.iter().flatten().filter_map(number_only).collect(),
      FormulaValue::Boolean(value) => vec![if *value { 1.0 } else { 0.0 }],
      FormulaValue::Number(value) => vec![*value],
      _ => Vec::new(),
    }
  }

  pub(crate) fn st_var_numbers(
    &self,
    args: &[FormulaAst<'doc>],
    direct_text_error: bool,
  ) -> std::result::Result<Vec<f64>, FormulaErrorValue> {
    let mut values = Vec::new();
    for arg in args {
      self.collect_st_var_numbers_from_ast(arg, direct_text_error, &mut values)?;
    }
    Ok(values)
  }

  pub(crate) fn collect_st_var_numbers_from_ast(
    &self,
    ast: &FormulaAst<'doc>,
    direct_text_error: bool,
    values: &mut Vec<f64>,
  ) -> std::result::Result<(), FormulaErrorValue> {
    match ast {
      FormulaAst::Binary {
        op: FormulaOperator::Union,
        left,
        right,
      } => {
        self.collect_st_var_numbers_from_ast(left, direct_text_error, values)?;
        self.collect_st_var_numbers_from_ast(right, direct_text_error, values)?;
      }
      _ => match self.evaluate(ast).unwrap_or_default() {
        FormulaValue::Reference(reference) => {
          values.extend(self.value_numbers(&FormulaValue::Reference(reference)))
        }
        FormulaValue::RefList(ranges) => {
          for range in ranges {
            values.extend(self.value_numbers(&FormulaValue::Reference(range)));
          }
        }
        FormulaValue::Matrix(rows) => {
          values.extend(self.value_numbers(&FormulaValue::Matrix(rows)))
        }
        FormulaValue::String(text) => {
          if let Ok(value) = text.trim().parse::<f64>() {
            values.push(value);
          } else if direct_text_error {
            return Err(FormulaErrorValue::Value);
          }
        }
        FormulaValue::Error(error) => return Err(error),
        value => values.extend(self.number(&value)),
      },
    }
    Ok(())
  }

  pub(crate) fn mode_ms_numeric_args(&self, args: &[FormulaAst<'doc>]) -> Vec<f64> {
    let mut values = Vec::new();
    let array_evaluator = self.with_array_context();
    for arg in args {
      let ranges = self.reference_ranges_from_ast(arg);
      if !ranges.is_empty() {
        for range in ranges {
          self.push_range_numbers_column_major(&range, &mut values);
        }
        continue;
      }
      let Some(value) = array_evaluator.evaluate(arg) else {
        continue;
      };
      match value {
        FormulaValue::Reference(reference) => {
          self.push_range_numbers_column_major(&reference, &mut values);
        }
        FormulaValue::RefList(ranges) => {
          for range in ranges {
            self.push_range_numbers_column_major(&range, &mut values);
          }
        }
        value => values.extend(self.value_numbers(&value)),
      }
    }
    values
  }

  pub(crate) fn push_range_numbers_column_major(
    &self,
    reference: &QualifiedRange<'doc>,
    values: &mut Vec<f64>,
  ) {
    if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
      return;
    }
    let sheet = self.range_sheet(reference);
    let start_column = reference.range.start.column.min(reference.range.end.column);
    let end_column = reference.range.start.column.max(reference.range.end.column);
    let start_row = reference.range.start.row.min(reference.range.end.row);
    let end_row = reference.range.start.row.max(reference.range.end.row);
    for column in start_column..=end_column {
      for row in start_row..=end_row {
        match self.book.cell_value(sheet, CellAddress { column, row }) {
          FormulaValue::Number(value) => values.push(value),
          FormulaValue::Boolean(value) => values.push(if value { 1.0 } else { 0.0 }),
          _ => {}
        }
      }
    }
  }

  pub(crate) fn value_numbers(&self, value: &FormulaValue<'doc>) -> Vec<f64> {
    match value {
      FormulaValue::Reference(reference) => self
        .range_values(reference)
        .iter()
        .filter_map(value_number_for_array)
        .collect(),
      FormulaValue::RefList(ranges) => ranges
        .iter()
        .flat_map(|range| self.range_values(range))
        .filter_map(|value| value_number_for_array(&value))
        .collect(),
      FormulaValue::Matrix(rows) => rows
        .iter()
        .flatten()
        .filter_map(value_number_for_array)
        .collect(),
      value => self.number(value).into_iter().collect(),
    }
  }

  pub(crate) fn value_numbers_from_ast(&self, ast: &FormulaAst<'doc>) -> Vec<f64> {
    let mut values = Vec::new();
    self.collect_value_numbers_from_ast(ast, &mut values);
    values
  }

  pub(crate) fn collect_value_numbers_from_ast(
    &self,
    ast: &FormulaAst<'doc>,
    values: &mut Vec<f64>,
  ) {
    match ast {
      FormulaAst::Binary {
        op: FormulaOperator::Union,
        left,
        right,
      } => {
        self.collect_value_numbers_from_ast(left, values);
        self.collect_value_numbers_from_ast(right, values);
      }
      _ => {
        if let Some(value) = self.evaluate(ast) {
          values.extend(self.value_numbers(&value));
        }
      }
    }
  }

  pub(crate) fn matrix_values(&self, value: &FormulaValue<'doc>) -> Vec<Vec<FormulaValue<'doc>>> {
    match value {
      FormulaValue::Reference(reference) => {
        if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
          return Vec::new();
        }
        let sheet = self.range_sheet(reference);
        (reference.range.start.row..=reference.range.end.row)
          .map(|row| {
            (reference.range.start.column..=reference.range.end.column)
              .map(|column| self.book.cell_value(sheet, CellAddress { column, row }))
              .collect()
          })
          .collect()
      }
      FormulaValue::RefList(ranges) => ranges
        .iter()
        .flat_map(|range| self.matrix_values(&FormulaValue::Reference(range.clone())))
        .collect(),
      FormulaValue::Matrix(rows) => rows.clone(),
      value => vec![vec![value.clone()]],
    }
  }

  pub(crate) fn lookup_reference_vector(
    &self,
    reference: &QualifiedRange<'doc>,
  ) -> Option<(Vec<FormulaValue<'doc>>, Option<Vec<usize>>, bool)> {
    let sheet = self.range_sheet(reference);
    let original_start_row = reference.range.start.row.min(reference.range.end.row);
    let original_end_row = reference.range.start.row.max(reference.range.end.row);
    let original_start_column = reference.range.start.column.min(reference.range.end.column);
    let original_end_column = reference.range.start.column.max(reference.range.end.column);
    let rows = original_end_row - original_start_row + 1;
    let columns = original_end_column - original_start_column + 1;
    if rows > 1 && columns > 1 {
      return None;
    }
    let range = if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
      self.book.data_area_subrange(sheet, reference.range)?
    } else {
      reference.range
    };
    let start_row = range.start.row.min(range.end.row);
    let end_row = range.start.row.max(range.end.row);
    let start_column = range.start.column.min(range.end.column);
    let end_column = range.start.column.max(range.end.column);
    let vertical = rows >= columns;
    let mut values = Vec::new();
    let mut indices = Vec::new();
    if vertical {
      for row in start_row..=end_row {
        let value = self.book.cell_value(
          sheet,
          CellAddress {
            column: start_column,
            row,
          },
        );
        if !matches!(value, FormulaValue::Blank) {
          values.push(value);
          indices.push((row - original_start_row) as usize);
        }
      }
    } else {
      for column in start_column..=end_column {
        let value = self.book.cell_value(
          sheet,
          CellAddress {
            column,
            row: start_row,
          },
        );
        if !matches!(value, FormulaValue::Blank) {
          values.push(value);
          indices.push((column - original_start_column) as usize);
        }
      }
    }
    Some((values, Some(indices), vertical))
  }

  pub(crate) fn query_grid_from_ast(&self, ast: &FormulaAst<'doc>) -> Option<QueryGrid<'doc>> {
    let value = self.evaluate(ast)?;
    self.query_grid_from_value(value)
  }

  pub(crate) fn query_grid_from_value(&self, value: FormulaValue<'doc>) -> Option<QueryGrid<'doc>> {
    Some(match value {
      FormulaValue::Reference(reference) => self.query_grid_from_reference(&reference),
      FormulaValue::RefList(ranges) if ranges.len() == 1 => {
        self.query_grid_from_reference(ranges.first()?)
      }
      FormulaValue::RefList(ranges) => {
        let mut values = Vec::new();
        let mut query_empty = Vec::new();
        for range in ranges {
          let grid = self.query_grid_from_reference(&range);
          values.extend(grid.values);
          query_empty.extend(grid.query_empty);
        }
        QueryGrid {
          values,
          query_empty,
        }
      }
      FormulaValue::Matrix(values) => {
        let query_empty = values
          .iter()
          .map(|row| vec![false; row.len()])
          .collect::<Vec<_>>();
        QueryGrid {
          values,
          query_empty,
        }
      }
      value => QueryGrid {
        values: vec![vec![value]],
        query_empty: vec![vec![false]],
      },
    })
  }

  pub(crate) fn query_grid_from_reference(
    &self,
    reference: &QualifiedRange<'doc>,
  ) -> QueryGrid<'doc> {
    let sheet = self.range_sheet(reference);
    let range = if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
      let Some(range) = self.book.data_area_subrange(sheet, reference.range) else {
        return QueryGrid {
          values: Vec::new(),
          query_empty: Vec::new(),
        };
      };
      range
    } else {
      reference.range
    };
    let start_row = range.start.row.min(range.end.row);
    let end_row = range.start.row.max(range.end.row);
    let start_column = range.start.column.min(range.end.column);
    let end_column = range.start.column.max(range.end.column);
    let mut values = Vec::new();
    let mut query_empty = Vec::new();
    for row in start_row..=end_row {
      let mut value_row = Vec::new();
      let mut empty_row = Vec::new();
      for column in start_column..=end_column {
        let address = CellAddress { column, row };
        value_row.push(self.book.query_cell_value(
          sheet,
          address,
          self.book.cell_value(sheet, address),
        ));
        empty_row.push(self.book.is_query_empty_cell(sheet, address));
      }
      values.push(value_row);
      query_empty.push(empty_row);
    }
    QueryGrid {
      values,
      query_empty,
    }
  }

  pub(crate) fn count_blank(&self, value: &FormulaValue<'doc>) -> usize {
    match value {
      FormulaValue::Reference(reference) => {
        if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
          return 0;
        }
        let sheet = self.range_sheet(reference);
        let single_cell = reference.range.cell_count_hint() == 1;
        let mut count = 0usize;
        for row in reference.range.start.row..=reference.range.end.row {
          for column in reference.range.start.column..=reference.range.end.column {
            let address = CellAddress { column, row };
            let value = self.book.cell_value(sheet, address);
            let formula = self.book.formula_text(sheet, address).is_some();
            let is_blank = match value {
              FormulaValue::Blank => !formula,
              FormulaValue::String(ref text) => text.is_empty() && (single_cell || formula),
              _ => false,
            };
            if is_blank {
              count += 1;
            }
          }
        }
        count
      }
      FormulaValue::RefList(ranges) => ranges
        .iter()
        .map(|range| self.count_blank(&FormulaValue::Reference(range.clone())))
        .sum(),
      FormulaValue::Matrix(rows) => rows
        .iter()
        .flatten()
        .filter(|value| is_blank_for_countblank(value))
        .count(),
      value if is_blank_for_countblank(value) => 1,
      _ => 0,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SumProductScalar {
  Number(f64),
  Error(FormulaErrorValue),
  NaN,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CouponFunction {
  Days,
  DayBs,
  DaysNc,
  Ncd,
  Num,
  Pcd,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum IfsAggregate {
  Sum,
  Count,
  Average,
  Min,
  Max,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DatabaseFunction {
  Sum,
  Count,
  CountA,
  Average,
  Get,
  Max,
  Min,
  Product,
  Var,
  VarP,
  StdDev,
  StdDevP,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum QueryValueKind {
  Number,
  Text,
  Blank,
  Empty,
  NonEmpty,
  Boolean,
  Error,
}

#[derive(Clone, Debug, PartialEq)]
struct QueryItem<'doc> {
  value: FormulaValue<'doc>,
  source_text: Option<Cow<'doc, str>>,
  kind: QueryValueKind,
  match_empty: bool,
  empty_matches_text: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct QueryEntry<'doc> {
  op: QueryOp,
  field: usize,
  item: QueryItem<'doc>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct QueryParam<'doc> {
  entries: Vec<QueryEntry<'doc>>,
  search_type: QuerySearchType,
  range_lookup: bool,
  match_whole_cell: bool,
  case_sensitive: bool,
}

impl<'doc> QueryParam<'doc> {
  fn single(entry: QueryEntry<'doc>, search_type: QuerySearchType, match_whole_cell: bool) -> Self {
    Self {
      entries: vec![entry],
      search_type,
      range_lookup: false,
      match_whole_cell,
      case_sensitive: false,
    }
  }

  fn from_criterion(
    evaluator: &FormulaEvaluator<'_, 'doc>,
    value: &FormulaValue<'doc>,
    field: usize,
  ) -> Self {
    let (entry, search_type) = QueryEntry::from_value(evaluator, value, field);
    Self::single(entry, search_type, evaluator.book.formula_match_whole_cell)
  }

  fn with_range_lookup(mut self, range_lookup: bool) -> Self {
    self.range_lookup = range_lookup;
    self
  }

  fn matches_value(
    &self,
    evaluator: &FormulaEvaluator<'_, 'doc>,
    candidate: &FormulaValue<'doc>,
    candidate_query_empty: bool,
  ) -> bool {
    QueryEvaluator {
      evaluator,
      param: self,
    }
    .matches_value(candidate, candidate_query_empty)
  }

  fn matches_row_with_empty(
    &self,
    evaluator: &FormulaEvaluator<'_, 'doc>,
    row: &[FormulaValue<'doc>],
    query_empty: &[bool],
  ) -> bool {
    QueryEvaluator {
      evaluator,
      param: self,
    }
    .matches_row_with_empty(row, query_empty)
  }
}

struct QueryEvaluator<'eval, 'ctx, 'doc> {
  evaluator: &'eval FormulaEvaluator<'ctx, 'doc>,
  param: &'eval QueryParam<'doc>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct QueryGrid<'doc> {
  values: Vec<Vec<FormulaValue<'doc>>>,
  query_empty: Vec<Vec<bool>>,
}

impl<'doc> QueryGrid<'doc> {
  fn dimensions(&self) -> (usize, usize) {
    matrix_dimensions(&self.values)
  }
}

impl<'eval, 'ctx, 'doc> QueryEvaluator<'eval, 'ctx, 'doc> {
  fn matches_value(&self, candidate: &FormulaValue<'doc>, candidate_query_empty: bool) -> bool {
    self
      .param
      .entries
      .first()
      .is_some_and(|entry| self.matches_entry(entry, candidate, candidate_query_empty))
  }

  fn matches_row_with_empty(&self, row: &[FormulaValue<'doc>], query_empty: &[bool]) -> bool {
    self.param.entries.iter().all(|entry| {
      row.get(entry.field).is_some_and(|candidate| {
        self.matches_entry(
          entry,
          candidate,
          query_empty.get(entry.field).copied().unwrap_or(false),
        )
      })
    })
  }

  fn matches_entry(
    &self,
    entry: &QueryEntry<'doc>,
    candidate: &FormulaValue<'doc>,
    candidate_query_empty: bool,
  ) -> bool {
    query_matches(
      self.evaluator,
      self.param,
      entry,
      candidate,
      candidate_query_empty,
    )
  }
}

impl<'doc> QueryEntry<'doc> {
  fn from_value(
    evaluator: &FormulaEvaluator<'_, 'doc>,
    value: &FormulaValue<'doc>,
    field: usize,
  ) -> (Self, QuerySearchType) {
    let value = evaluator.first_value(value);
    if let FormulaValue::String(text) = value {
      let (op, operand) = parse_criteria_operator(text.as_ref());
      let trimmed = operand.trim();
      let is_empty_criterion = operand.is_empty();
      let explicit_empty_operator = matches!(text.as_ref(), "=" | "<>");
      let operand_value = parse_query_number_format(trimmed, evaluator.book.date_system)
        .map(FormulaValue::Number)
        .unwrap_or_else(|_| FormulaValue::String(Cow::Owned(operand.to_string())));
      let source_text =
        matches!(operand_value, FormulaValue::Number(_)).then(|| Cow::Owned(operand.to_string()));
      let search_type = if matches!(operand_value, FormulaValue::String(_)) {
        detect_query_search_type(evaluator.book.formula_search_type, operand)
      } else {
        QuerySearchType::Normal
      };
      let kind = if matches!(operand_value, FormulaValue::Number(_)) {
        QueryValueKind::Number
      } else if is_empty_criterion && matches!(op, QueryOp::Equal | QueryOp::NotEqual) {
        if op == QueryOp::Equal {
          QueryValueKind::Empty
        } else {
          QueryValueKind::NonEmpty
        }
      } else {
        QueryValueKind::Text
      };
      (
        Self {
          op,
          field,
          item: QueryItem {
            value: operand_value,
            source_text,
            kind,
            match_empty: (op == QueryOp::Equal && is_empty_criterion)
              || (op == QueryOp::NotEqual && !is_empty_criterion),
            empty_matches_text: is_empty_criterion && !explicit_empty_operator,
          },
        },
        search_type,
      )
    } else {
      let value = if matches!(value, FormulaValue::Blank) {
        FormulaValue::Number(0.0)
      } else {
        value
      };
      (
        Self {
          op: QueryOp::Equal,
          field,
          item: QueryItem {
            kind: query_value_kind(&value),
            value,
            source_text: None,
            match_empty: false,
            empty_matches_text: false,
          },
        },
        QuerySearchType::Normal,
      )
    }
  }

  fn from_database_value(
    evaluator: &FormulaEvaluator<'_, 'doc>,
    value: &FormulaValue<'doc>,
    field: usize,
  ) -> (Self, QuerySearchType) {
    let value = evaluator.first_value(value);
    if let FormulaValue::String(text) = value {
      let (op, operand) = parse_criteria_operator(text.as_ref());
      let trimmed = operand.trim();
      let operand_value = parse_query_number_format(trimmed, evaluator.book.date_system)
        .map(FormulaValue::Number)
        .unwrap_or_else(|_| FormulaValue::String(Cow::Owned(operand.to_string())));
      let source_text =
        matches!(operand_value, FormulaValue::Number(_)).then(|| Cow::Owned(operand.to_string()));
      let search_type = if matches!(operand_value, FormulaValue::String(_)) {
        detect_query_search_type(evaluator.book.formula_search_type, operand)
      } else {
        QuerySearchType::Normal
      };
      let kind = if matches!(operand_value, FormulaValue::Number(_)) {
        QueryValueKind::Number
      } else {
        QueryValueKind::Text
      };
      (
        Self {
          op,
          field,
          item: QueryItem {
            value: operand_value,
            source_text,
            kind,
            match_empty: false,
            empty_matches_text: false,
          },
        },
        search_type,
      )
    } else {
      let value = if matches!(value, FormulaValue::Blank) {
        FormulaValue::Number(0.0)
      } else {
        value
      };
      (
        Self {
          op: QueryOp::Equal,
          field,
          item: QueryItem {
            kind: query_value_kind(&value),
            value,
            source_text: None,
            match_empty: false,
            empty_matches_text: false,
          },
        },
        QuerySearchType::Normal,
      )
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LookupSearchMode {
  Forward,
  Reverse,
  BinaryAscending,
  BinaryDescending,
}

impl LookupSearchMode {
  fn from_excel(value: i32) -> Option<Self> {
    match value {
      1 => Some(Self::Forward),
      -1 => Some(Self::Reverse),
      2 => Some(Self::BinaryAscending),
      -2 => Some(Self::BinaryDescending),
      _ => None,
    }
  }
}

fn query_matches<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  param: &QueryParam<'doc>,
  query: &QueryEntry<'doc>,
  candidate: &FormulaValue<'doc>,
  candidate_query_empty: bool,
) -> bool {
  if matches!(candidate, FormulaValue::Error(_)) {
    return false;
  }
  if matches!(
    query.item.kind,
    QueryValueKind::Empty | QueryValueKind::NonEmpty
  ) {
    let blank = candidate_query_empty
      || matches!(candidate, FormulaValue::Blank)
      || (query.item.empty_matches_text
        && matches!(candidate, FormulaValue::String(text) if text.is_empty()));
    return if query.item.kind == QueryValueKind::Empty {
      blank
    } else {
      !blank
    };
  }
  if query.item.match_empty && (candidate_query_empty || is_query_empty(candidate)) {
    return matches!(
      query.op,
      QueryOp::NotEqual | QueryOp::LessOrEqual | QueryOp::GreaterOrEqual
    );
  }
  if !param.range_lookup
    && query.item.kind == QueryValueKind::Number
    && query_candidate_number(candidate, candidate_query_empty).is_none()
  {
    if let Some(source_text) = &query.item.source_text
      && let FormulaValue::String(candidate_text) = candidate
      && matches!(query.op, QueryOp::Equal | QueryOp::NotEqual)
    {
      let matched = if param.match_whole_cell {
        compare_text(evaluator, candidate_text, source_text, param.case_sensitive) == 0
      } else if param.case_sensitive {
        candidate_text.contains(source_text.as_ref())
      } else {
        lookup_text_contains(candidate_text, source_text)
      };
      return if query.op == QueryOp::Equal {
        matched
      } else {
        !matched
      };
    }
    return matches!(query.op, QueryOp::NotEqual);
  }
  if !param.range_lookup
    && query.item.kind == QueryValueKind::Text
    && matches!(candidate, FormulaValue::Number(_))
  {
    return matches!(query.op, QueryOp::NotEqual);
  }
  if param.search_type == QuerySearchType::Wildcard {
    let FormulaValue::String(pattern) = &query.item.value else {
      return false;
    };
    let text = evaluator.text(candidate);
    let matched = if param.match_whole_cell {
      wildcard_match(pattern.as_ref(), &text)
    } else {
      wildcard_match(pattern.as_ref(), &text) || lookup_text_contains(&text, pattern.as_ref())
    };
    return match query.op {
      QueryOp::Equal => matched,
      QueryOp::NotEqual => !matched,
      _ => false,
    };
  }
  if param.search_type == QuerySearchType::Regex {
    let FormulaValue::String(pattern) = &query.item.value else {
      return false;
    };
    let matched =
      regex_match(pattern, &evaluator.text(candidate), param.match_whole_cell).unwrap_or(false);
    return match query.op {
      QueryOp::Equal => matched,
      QueryOp::NotEqual => !matched,
      _ => false,
    };
  }
  if !param.match_whole_cell
    && matches!(query.op, QueryOp::Equal | QueryOp::NotEqual)
    && let (FormulaValue::String(candidate_text), FormulaValue::String(query_text)) =
      (candidate, &query.item.value)
  {
    let matched = if param.case_sensitive {
      candidate_text.contains(query_text.as_ref())
    } else {
      lookup_text_contains(candidate_text, query_text)
    };
    return if query.op == QueryOp::Equal {
      matched
    } else {
      !matched
    };
  }
  let ordering = query_compare_value(
    evaluator,
    candidate,
    candidate_query_empty,
    &query.item.value,
    param,
  );
  if ordering.is_none() && param.range_lookup {
    return query_compare_by_range_lookup(query, candidate);
  }
  match query.op {
    QueryOp::Equal => ordering == Some(0),
    QueryOp::NotEqual => ordering != Some(0),
    QueryOp::Less => ordering == Some(-1),
    QueryOp::LessOrEqual => matches!(ordering, Some(-1 | 0)),
    QueryOp::Greater => ordering == Some(1),
    QueryOp::GreaterOrEqual => matches!(ordering, Some(0 | 1)),
  }
}

fn query_compare_by_range_lookup(query: &QueryEntry<'_>, candidate: &FormulaValue<'_>) -> bool {
  match query.item.kind {
    QueryValueKind::Text if !matches!(query.op, QueryOp::Less | QueryOp::LessOrEqual) => false,
    QueryValueKind::Text => query_candidate_number(candidate, false).is_some(),
    _ if !matches!(query.op, QueryOp::Greater | QueryOp::GreaterOrEqual) => false,
    _ => query_candidate_number(candidate, false).is_none(),
  }
}

fn query_compare_value<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  candidate: &FormulaValue<'doc>,
  candidate_query_empty: bool,
  query: &FormulaValue<'doc>,
  param: &QueryParam<'doc>,
) -> Option<i32> {
  if let Some((candidate, query)) =
    query_candidate_number(candidate, candidate_query_empty).zip(evaluator.number(query))
  {
    return Some(compare_numbers(candidate, query));
  }
  match (candidate, query) {
    (FormulaValue::String(candidate), FormulaValue::String(query)) => Some(compare_text(
      evaluator,
      candidate,
      query,
      param.case_sensitive,
    )),
    (FormulaValue::Blank, FormulaValue::Blank) => Some(0),
    (FormulaValue::Blank, _) if param.range_lookup => Some(-1),
    (_, FormulaValue::Blank) if param.range_lookup => Some(1),
    (FormulaValue::Number(_), FormulaValue::String(_)) if param.range_lookup => None,
    (FormulaValue::String(_), FormulaValue::Number(_)) if param.range_lookup => None,
    (FormulaValue::Boolean(left), FormulaValue::Boolean(right)) => Some(match left.cmp(right) {
      std::cmp::Ordering::Less => -1,
      std::cmp::Ordering::Equal => 0,
      std::cmp::Ordering::Greater => 1,
    }),
    _ => None,
  }
}

fn query_candidate_number(value: &FormulaValue<'_>, query_empty: bool) -> Option<f64> {
  if query_empty {
    return Some(0.0);
  }
  match value {
    FormulaValue::Number(value) => Some(*value),
    FormulaValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
    _ => None,
  }
}

fn query_value_kind(value: &FormulaValue<'_>) -> QueryValueKind {
  match value {
    FormulaValue::Number(_) => QueryValueKind::Number,
    FormulaValue::String(_) => QueryValueKind::Text,
    FormulaValue::Blank => QueryValueKind::Blank,
    FormulaValue::Boolean(_) => QueryValueKind::Boolean,
    FormulaValue::Error(_) => QueryValueKind::Error,
    FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_) => {
      QueryValueKind::Blank
    }
  }
}

fn parse_query_number_format(text: &str, date_system: DateSystem) -> std::result::Result<f64, ()> {
  if let Ok(value) = text.parse::<f64>() {
    return Ok(value);
  }
  if let Some(value) = parse_query_decimal_comma_number(text) {
    return Ok(value);
  }
  parse_date_input(text, date_system)
    .map(f64::floor)
    .ok_or(())
}

fn parse_query_decimal_comma_number(text: &str) -> Option<f64> {
  let trimmed = text.trim();
  if trimmed.contains('.') || trimmed.matches(',').count() != 1 {
    return None;
  }
  let (integer, fraction) = trimmed.split_once(',')?;
  if fraction.is_empty() || !fraction.bytes().all(|byte| byte.is_ascii_digit()) {
    return None;
  }
  let sign_stripped = integer
    .strip_prefix('+')
    .or_else(|| integer.strip_prefix('-'))
    .unwrap_or(integer);
  if !sign_stripped.is_empty() && !sign_stripped.bytes().all(|byte| byte.is_ascii_digit()) {
    return None;
  }
  trimmed.replace(',', ".").parse::<f64>().ok()
}

fn is_query_empty(value: &FormulaValue<'_>) -> bool {
  matches!(value, FormulaValue::Blank)
    || matches!(value, FormulaValue::String(text) if text.is_empty())
}

fn database_criterion_cell_present(value: &FormulaValue<'_>, query_empty: bool) -> bool {
  query_empty || !matches!(value, FormulaValue::Blank)
}

fn database_criterion_entry_present(value: &FormulaValue<'_>, query_empty: bool) -> bool {
  database_criterion_cell_present(value, query_empty)
    && !matches!(value, FormulaValue::String(text) if text.is_empty())
}

fn compare_numbers(left: f64, right: f64) -> i32 {
  if approx_equal(left, right) {
    0
  } else if left < right {
    -1
  } else {
    1
  }
}

fn rtl_valid_trig_arg(value: f64) -> bool {
  value.abs() <= (0x8000_0000u64 as f64) * (0x8000_0000u64 as f64) * 4.0
}

pub(crate) fn rtl_sin(value: f64) -> f64 {
  if rtl_valid_trig_arg(value) {
    value.sin()
  } else {
    f64::NAN
  }
}

pub(crate) fn rtl_cos(value: f64) -> f64 {
  if rtl_valid_trig_arg(value) {
    value.cos()
  } else {
    f64::NAN
  }
}

pub(crate) fn rtl_tan(value: f64) -> f64 {
  if rtl_valid_trig_arg(value) {
    value.tan()
  } else {
    f64::NAN
  }
}

fn compare_text(
  evaluator: &FormulaEvaluator<'_, '_>,
  left: &str,
  right: &str,
  case_sensitive: bool,
) -> i32 {
  let ordering =
    lookup_collator(evaluator.book.locale.as_deref(), case_sensitive).compare(left, right);
  match ordering {
    std::cmp::Ordering::Less => -1,
    std::cmp::Ordering::Equal => 0,
    std::cmp::Ordering::Greater => 1,
  }
}

fn lookup_collator(
  locale: Option<&str>,
  case_sensitive: bool,
) -> &'static CollatorBorrowed<'static> {
  type CollatorCache = Mutex<BTreeMap<(Option<String>, bool), &'static CollatorBorrowed<'static>>>;

  static COLLATORS: OnceLock<CollatorCache> = OnceLock::new();
  let key = (
    locale
      .filter(|value| !value.trim().is_empty())
      .map(str::to_string),
    case_sensitive,
  );
  let mut collators = COLLATORS
    .get_or_init(|| Mutex::new(BTreeMap::new()))
    .lock()
    .expect("lookup collator cache lock must not be poisoned");
  if let Some(collator) = collators.get(&key) {
    return collator;
  }
  let mut options = CollatorOptions::default();
  options.strength = Some(if case_sensitive {
    Strength::Tertiary
  } else {
    Strength::Secondary
  });
  let prefs = key
    .0
    .as_deref()
    .and_then(|locale| locale.parse::<Locale>().ok())
    .map(|locale| CollatorPreferences::from(&locale))
    .unwrap_or_default();
  let collator = Box::leak(Box::new(
    Collator::try_new(prefs, options)
      .expect("ICU compiled collator data must contain the requested locale"),
  ));
  collators.insert(key, collator);
  collator
}

#[derive(Clone, Copy)]
struct SearchVectorFlags {
  exact_type: bool,
  match_empty: bool,
  first_exact: bool,
}

impl SearchVectorFlags {
  fn new(exact_type: bool, match_empty: bool) -> Self {
    Self {
      exact_type,
      match_empty,
      first_exact: false,
    }
  }

  fn with_first_exact(mut self) -> Self {
    self.first_exact = true;
    self
  }
}

fn search_vector<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  lookup: &FormulaValue<'doc>,
  vector: &[FormulaValue<'doc>],
  op: QueryOp,
  mode: LookupSearchMode,
  exact_type: bool,
) -> Option<usize> {
  let search_type = if let FormulaValue::String(text) = lookup {
    detect_query_search_type(evaluator.book.formula_search_type, text)
  } else {
    QuerySearchType::Normal
  };
  search_vector_with_type(
    evaluator,
    lookup,
    vector,
    op,
    mode,
    search_type,
    SearchVectorFlags::new(exact_type, false),
  )
}

fn match_range_linear_index<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  lookup: &FormulaValue<'doc>,
  vector: &[FormulaValue<'doc>],
  mode: i32,
) -> Option<usize> {
  // Source: LibreOffice sc/source/core/data/queryiter.cxx
  // ScQueryCellIterator::FindEqualOrSortedLastInRange for horizontal MATCH
  // ranges.  Horizontal document ranges use the direct iterator path, not the
  // matrix binary search path.
  let op = match mode {
    1 => QueryOp::LessOrEqual,
    -1 => QueryOp::GreaterOrEqual,
    _ => return None,
  };
  let query = QueryEntry {
    op,
    field: 0,
    item: QueryItem {
      kind: query_value_kind(lookup),
      value: lookup.clone(),
      source_text: None,
      match_empty: false,
      empty_matches_text: false,
    },
  };
  let param = QueryParam::single(query, QuerySearchType::Normal, true).with_range_lookup(true);
  let query = param.entries.first()?;
  let mut found = None;
  let mut first_string_ignore = matches!(lookup, FormulaValue::String(_));
  for (index, candidate) in vector.iter().enumerate() {
    let exact = lookup_types_compatible(evaluator, lookup, candidate)
      && evaluator.compare(candidate, lookup, FormulaOperator::Equal);
    let valid = query_matches(evaluator, &param, query, candidate, false);
    if valid {
      found = Some(index);
      if exact {
        let mut last_equal = index;
        for (next_index, next) in vector.iter().enumerate().skip(index + 1) {
          if lookup_types_compatible(evaluator, lookup, next)
            && evaluator.compare(next, lookup, FormulaOperator::Equal)
          {
            last_equal = next_index;
          } else {
            break;
          }
        }
        return Some(last_equal);
      }
    } else if first_string_ignore && matches!(candidate, FormulaValue::String(_)) {
      first_string_ignore = false;
      continue;
    } else {
      break;
    }
    first_string_ignore = false;
  }
  found.filter(|index| lookup_candidate_type_matches(query, &vector[*index]))
}

fn search_vector_with_type<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  lookup: &FormulaValue<'doc>,
  vector: &[FormulaValue<'doc>],
  op: QueryOp,
  mode: LookupSearchMode,
  search_type: QuerySearchType,
  flags: SearchVectorFlags,
) -> Option<usize> {
  let range_lookup = !matches!(op, QueryOp::Equal | QueryOp::NotEqual);
  let query = QueryEntry {
    op,
    field: 0,
    item: QueryItem {
      kind: query_value_kind(lookup),
      value: lookup.clone(),
      source_text: None,
      match_empty: flags.match_empty,
      empty_matches_text: false,
    },
  };
  let param = QueryParam::single(query, search_type, true).with_range_lookup(range_lookup);
  let query = param.entries.first()?;
  match mode {
    LookupSearchMode::BinaryAscending | LookupSearchMode::BinaryDescending => {
      if search_type != QuerySearchType::Normal {
        return None;
      }
      lookup_binary_search(
        evaluator,
        vector,
        query,
        &param,
        matches!(mode, LookupSearchMode::BinaryAscending),
        true,
        flags.first_exact,
      )
    }
    LookupSearchMode::Forward => {
      let mut found = None;
      for (index, candidate) in vector.iter().enumerate() {
        if flags.exact_type && !lookup_types_compatible(evaluator, lookup, candidate) {
          continue;
        }
        if query_matches(evaluator, &param, query, candidate, false) {
          match op {
            QueryOp::Equal => {
              found = Some(index);
              break;
            }
            QueryOp::LessOrEqual => {
              if lookup_candidate_type_matches(query, candidate)
                && found.is_none_or(|found_index| {
                  lookup_compare_cells(evaluator, candidate, &vector[found_index]) >= 0
                })
              {
                found = Some(index);
              }
            }
            QueryOp::GreaterOrEqual => {
              if lookup_candidate_type_matches(query, candidate)
                && found.is_none_or(|found_index| {
                  lookup_compare_cells(evaluator, candidate, &vector[found_index]) <= 0
                })
              {
                found = Some(index);
              }
            }
            _ => {
              if lookup_candidate_type_matches(query, candidate) {
                found = Some(index);
              }
            }
          }
        }
      }
      found
    }
    LookupSearchMode::Reverse => {
      let mut found = None;
      for (index, candidate) in vector.iter().enumerate().rev() {
        if flags.exact_type && !lookup_types_compatible(evaluator, lookup, candidate) {
          continue;
        }
        if query_matches(evaluator, &param, query, candidate, false) {
          match op {
            QueryOp::Equal => {
              found = Some(index);
              break;
            }
            QueryOp::LessOrEqual => {
              if lookup_candidate_type_matches(query, candidate)
                && found.is_none_or(|found_index| {
                  lookup_compare_cells(evaluator, candidate, &vector[found_index]) > 0
                })
              {
                found = Some(index);
              }
            }
            QueryOp::GreaterOrEqual => {
              if lookup_candidate_type_matches(query, candidate)
                && found.is_none_or(|found_index| {
                  lookup_compare_cells(evaluator, candidate, &vector[found_index]) < 0
                })
              {
                found = Some(index);
              }
            }
            _ => {
              if lookup_candidate_type_matches(query, candidate) {
                found = Some(index);
              }
            }
          }
        }
      }
      found
    }
  }
}

fn lookup_binary_search<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  vector: &[FormulaValue<'doc>],
  query: &QueryEntry<'doc>,
  param: &QueryParam<'doc>,
  sorted_ascending: bool,
  empty_is_less: bool,
  first_exact: bool,
) -> Option<usize> {
  if vector.is_empty() {
    return None;
  }
  let mut low = 0usize;
  let mut high = vector.len();
  let mut exact = None;
  while low < high {
    let mid = low + (high - low) / 2;
    let cmp =
      lookup_compare_candidate_to_query(evaluator, &vector[mid], query, param, empty_is_less)?;
    if cmp == 0 {
      exact = Some(mid);
      break;
    }
    if sorted_ascending {
      if cmp < 0 {
        low = mid + 1;
      } else {
        high = mid;
      }
    } else {
      if cmp > 0 {
        low = mid + 1;
      } else {
        high = mid;
      }
    }
  }
  if let Some(mut index) = exact {
    if first_exact && sorted_ascending {
      while index > 0 && lookup_same_match_value(&vector[index], &vector[index - 1]) {
        index -= 1;
      }
    } else {
      while index + 1 < vector.len() && lookup_same_match_value(&vector[index], &vector[index + 1])
      {
        index += 1;
      }
    }
    return lookup_binary_result_index(vector, query, index);
  }
  if query.op == QueryOp::Equal {
    return None;
  }
  let index = match (sorted_ascending, query.op) {
    (true, QueryOp::LessOrEqual) => low.checked_sub(1),
    (true, QueryOp::GreaterOrEqual) => (low < vector.len()).then_some(low),
    (false, QueryOp::LessOrEqual) => (low < vector.len()).then_some(low),
    (false, QueryOp::GreaterOrEqual) => low.checked_sub(1),
    _ => None,
  }?;
  lookup_binary_result_index(vector, query, index)
}

fn lookup_binary_result_index<'doc>(
  vector: &[FormulaValue<'doc>],
  query: &QueryEntry<'doc>,
  index: usize,
) -> Option<usize> {
  (lookup_candidate_type_matches(query, vector.get(index)?)).then_some(index)
}

fn lookup_candidate_type_matches(query: &QueryEntry<'_>, candidate: &FormulaValue<'_>) -> bool {
  match query.item.kind {
    QueryValueKind::Text => !matches!(candidate, FormulaValue::Number(_)),
    QueryValueKind::Number => query_candidate_number(candidate, false).is_some(),
    _ => true,
  }
}

fn lookup_compare_candidate_to_query<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  candidate: &FormulaValue<'doc>,
  query: &QueryEntry<'doc>,
  param: &QueryParam<'doc>,
  empty_is_less: bool,
) -> Option<i32> {
  if matches!(candidate, FormulaValue::Blank) {
    return Some(if empty_is_less { -1 } else { 1 });
  }
  query_compare_value(evaluator, candidate, false, &query.item.value, param).or_else(|| match query
    .item
    .kind
  {
    QueryValueKind::Text if query_candidate_number(candidate, false).is_some() => Some(-1),
    QueryValueKind::Number if matches!(candidate, FormulaValue::String(_)) => Some(1),
    _ => None,
  })
}

fn lookup_compare_cells<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  left: &FormulaValue<'doc>,
  right: &FormulaValue<'doc>,
) -> i32 {
  match (left, right) {
    (FormulaValue::Blank, FormulaValue::Blank) => 0,
    (FormulaValue::Blank, _) => -1,
    (_, FormulaValue::Blank) => 1,
    (FormulaValue::Error(_), FormulaValue::Error(_)) => 0,
    (FormulaValue::Error(_), _) => 1,
    (_, FormulaValue::Error(_)) => -1,
    _ => {
      let left_number = query_candidate_number(left, false);
      let right_number = query_candidate_number(right, false);
      match (left_number, right_number) {
        (Some(left), Some(right)) => compare_numbers(left, right),
        (Some(_), None) => -1,
        (None, Some(_)) => 1,
        (None, None) => compare_text(
          evaluator,
          &evaluator.text(left),
          &evaluator.text(right),
          false,
        ),
      }
    }
  }
}

fn lookup_same_match_value(left: &FormulaValue<'_>, right: &FormulaValue<'_>) -> bool {
  match (left, right) {
    (FormulaValue::Number(left), FormulaValue::Number(right)) => left == right,
    (FormulaValue::Boolean(left), FormulaValue::Boolean(right)) => left == right,
    (FormulaValue::Blank, FormulaValue::Blank) => true,
    (FormulaValue::String(left), FormulaValue::String(right)) => left == right,
    (FormulaValue::Error(_), FormulaValue::Error(_)) => true,
    _ => false,
  }
}

fn lookup_types_compatible<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  lookup: &FormulaValue<'doc>,
  candidate: &FormulaValue<'doc>,
) -> bool {
  match (lookup, candidate) {
    (FormulaValue::Blank, FormulaValue::Blank) => true,
    (FormulaValue::Blank, _) | (_, FormulaValue::Blank) => false,
    (FormulaValue::String(_), FormulaValue::String(_)) => true,
    (FormulaValue::String(_), _) => false,
    (_, FormulaValue::String(_)) => false,
    _ => evaluator.number(lookup).is_some() == evaluator.number(candidate).is_some(),
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DatePart {
  Year,
  Month,
  Day,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TimePart {
  Hour,
  Minute,
  Second,
}

#[derive(Clone, Copy, Debug)]
struct AggregateOptions {
  ignore_hidden: bool,
  ignore_filtered: bool,
  ignore_errors: bool,
  ignore_nested: bool,
}

fn aggregate_options(option: i32) -> Option<AggregateOptions> {
  // Source: LibreOffice sc/source/core/tool/interpr1.cxx ScAggregate.
  Some(match option {
    0 => AggregateOptions {
      ignore_hidden: false,
      ignore_filtered: false,
      ignore_errors: false,
      ignore_nested: true,
    },
    1 => AggregateOptions {
      ignore_hidden: true,
      ignore_filtered: false,
      ignore_errors: false,
      ignore_nested: true,
    },
    2 => AggregateOptions {
      ignore_hidden: false,
      ignore_filtered: false,
      ignore_errors: true,
      ignore_nested: true,
    },
    3 => AggregateOptions {
      ignore_hidden: true,
      ignore_filtered: false,
      ignore_errors: true,
      ignore_nested: true,
    },
    4 => AggregateOptions {
      ignore_hidden: false,
      ignore_filtered: false,
      ignore_errors: false,
      ignore_nested: false,
    },
    5 => AggregateOptions {
      ignore_hidden: true,
      ignore_filtered: false,
      ignore_errors: false,
      ignore_nested: false,
    },
    6 => AggregateOptions {
      ignore_hidden: false,
      ignore_filtered: false,
      ignore_errors: true,
      ignore_nested: false,
    },
    7 => AggregateOptions {
      ignore_hidden: true,
      ignore_filtered: false,
      ignore_errors: true,
      ignore_nested: false,
    },
    _ => return None,
  })
}

fn let_binding_name(ast: &FormulaAst<'_>) -> Option<String> {
  let FormulaAst::Name(name) = ast else {
    return None;
  };
  Some(name.trim_start_matches("_xlpm.").to_ascii_uppercase())
}

fn aggregate_function_value<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  function: i32,
  args: &[FormulaValue<'doc>],
  k: Option<f64>,
  options: AggregateOptions,
) -> Option<std::result::Result<f64, FormulaErrorValue>> {
  if function == 2 {
    return Some(Ok(aggregate_count_numbers(evaluator, args, options) as f64));
  }
  if function == 3 {
    return aggregate_counta(evaluator, args, options)
      .map(|result| result.map(|count| count as f64));
  }
  let values = match aggregate_numbers(evaluator, args, options) {
    Ok(values) => values,
    Err(error) => return Some(Err(error)),
  };
  match function {
    1 => mean(&values),
    2 => Some(values.len() as f64),
    4 => Some(values.into_iter().reduce(f64::max).unwrap_or(0.0)),
    5 => Some(values.into_iter().reduce(f64::min).unwrap_or(0.0)),
    6 => Some(values.into_iter().product()),
    7 => variance_slice(&values, true).map(f64::sqrt),
    8 => variance_slice(&values, false).map(f64::sqrt),
    9 => Some(kahan_sum(values)),
    10 => variance_slice(&values, true),
    11 => variance_slice(&values, false),
    12 => {
      let mut values = values;
      percentile_sorted(&mut values, 0.5, PercentileKind::Inc)
    }
    13 => mode_slice(&values),
    14 => kth_value(values, k?.ceil(), true),
    15 => kth_value(values, k?.floor(), false),
    16 => {
      let mut values = values;
      percentile_sorted(&mut values, k?, PercentileKind::Inc)
    }
    17 => {
      let mut values = values;
      percentile_sorted(&mut values, k?.floor() / 4.0, PercentileKind::Inc)
    }
    18 => {
      let mut values = values;
      percentile_sorted(&mut values, k?, PercentileKind::Exc)
    }
    19 => {
      let mut values = values;
      percentile_sorted(&mut values, k?.floor() / 4.0, PercentileKind::Exc)
    }
    _ => None,
  }
  .map(Ok)
}

fn aggregate_numbers<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  args: &[FormulaValue<'doc>],
  options: AggregateOptions,
) -> std::result::Result<Vec<f64>, FormulaErrorValue> {
  let mut values = Vec::new();
  for arg in args {
    collect_aggregate_numbers(evaluator, arg, options, &mut values)?;
  }
  Ok(values)
}

fn aggregate_count_numbers<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  args: &[FormulaValue<'doc>],
  options: AggregateOptions,
) -> usize {
  let mut count = 0usize;
  for arg in args {
    count += aggregate_count_numbers_value(evaluator, arg, options);
  }
  count
}

fn aggregate_count_numbers_value<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  value: &FormulaValue<'doc>,
  options: AggregateOptions,
) -> usize {
  match value {
    FormulaValue::Reference(reference) => {
      if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
        return 0;
      }
      let sheet = evaluator.range_sheet(reference);
      let mut count = 0usize;
      for row in reference.range.start.row..=reference.range.end.row {
        if (options.ignore_filtered && evaluator.book.row_filtered(sheet, row))
          || (options.ignore_hidden && evaluator.book.row_hidden(sheet, row))
        {
          continue;
        }
        for column in reference.range.start.column..=reference.range.end.column {
          let address = CellAddress { column, row };
          if options.ignore_nested && evaluator.book.is_nested_aggregate(sheet, address) {
            continue;
          }
          if matches!(
            evaluator.book.cell_value(sheet, address),
            FormulaValue::Number(_)
          ) {
            count += 1;
          }
        }
      }
      count
    }
    FormulaValue::Matrix(rows) => rows
      .iter()
      .flatten()
      .filter(|value| matches!(value, FormulaValue::Number(_)))
      .count(),
    FormulaValue::Number(_) => 1,
    _ => 0,
  }
}

fn collect_aggregate_numbers<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  value: &FormulaValue<'doc>,
  options: AggregateOptions,
  values: &mut Vec<f64>,
) -> std::result::Result<(), FormulaErrorValue> {
  match value {
    FormulaValue::Reference(reference) => {
      if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
        return Ok(());
      }
      let sheet = evaluator.range_sheet(reference);
      for row in reference.range.start.row..=reference.range.end.row {
        if (options.ignore_filtered && evaluator.book.row_filtered(sheet, row))
          || (options.ignore_hidden && evaluator.book.row_hidden(sheet, row))
        {
          continue;
        }
        for column in reference.range.start.column..=reference.range.end.column {
          let address = CellAddress { column, row };
          if options.ignore_nested && evaluator.book.is_nested_aggregate(sheet, address) {
            continue;
          }
          collect_aggregate_scalar(evaluator.book.cell_value(sheet, address), options, values)?;
        }
      }
      Ok(())
    }
    FormulaValue::Matrix(rows) => {
      for value in rows.iter().flatten() {
        collect_aggregate_scalar(value.clone(), options, values)?;
      }
      Ok(())
    }
    value => collect_aggregate_scalar(value.clone(), options, values),
  }
}

fn collect_aggregate_scalar(
  value: FormulaValue<'_>,
  options: AggregateOptions,
  values: &mut Vec<f64>,
) -> std::result::Result<(), FormulaErrorValue> {
  match value {
    FormulaValue::Number(number) => values.push(number),
    FormulaValue::Boolean(value) => values.push(if value { 1.0 } else { 0.0 }),
    FormulaValue::Error(error) if !options.ignore_errors => return Err(error),
    _ => {}
  }
  Ok(())
}

fn matrix_can_broadcast(
  rows: usize,
  columns: usize,
  target_rows: usize,
  target_columns: usize,
) -> bool {
  (rows == target_rows || rows == 1) && (columns == target_columns || columns == 1)
}

fn matrix_binary_extent(left: usize, right: usize) -> usize {
  // Source: LibreOffice sc/source/core/tool/interpr5.cxx lcl_GetMinExtent.
  if left == 1 {
    right
  } else if right == 1 {
    left
  } else {
    left.min(right)
  }
}

fn formula_error_matches(value: &FormulaValue<'_>, na_only: bool) -> bool {
  matches!(
    (value, na_only),
    (FormulaValue::Error(FormulaErrorValue::NA), true) | (FormulaValue::Error(_), false)
  )
}

fn matrix_dimensions<T>(matrix: &[Vec<T>]) -> (usize, usize) {
  (matrix.len(), matrix.first().map_or(0, Vec::len))
}

fn ranges_intersect(left: &CellRange, right: &CellRange) -> bool {
  left.start.column <= right.end.column
    && right.start.column <= left.end.column
    && left.start.row <= right.end.row
    && right.start.row <= left.end.row
}

fn pivot_source_headers<'doc>(
  book: &FormulaEvaluationBook<'doc>,
  sheet: SheetId,
  pivot: &FormulaPivotTable<'doc>,
) -> Option<Vec<String>> {
  let source = &pivot.source.range;
  let mut fields = Vec::new();
  for column in source.start.column..=source.end.column {
    let value = book.cell_value(
      sheet,
      CellAddress {
        column,
        row: source.start.row,
      },
    );
    let FormulaValue::String(name) = value else {
      return None;
    };
    fields.push(name.into_owned());
  }
  Some(fields)
}

fn pivot_data_field<'doc>(
  pivot: &'doc FormulaPivotTable<'doc>,
  name: Option<&str>,
) -> Option<&'doc FormulaPivotField<'doc>> {
  let mut data_fields = pivot
    .fields
    .iter()
    .filter(|field| field.orientation == FormulaPivotFieldOrientation::Data);
  if let Some(name) = name {
    data_fields.find(|field| pivot_data_field_name_eq(&field.name, name))
  } else {
    data_fields.next()
  }
}

fn pivot_data_field_name_eq(field: &str, requested: &str) -> bool {
  pivot_name_eq(field, requested)
    || requested
      .strip_prefix("Sum - ")
      .is_some_and(|name| pivot_name_eq(field, name))
    || requested
      .strip_prefix("Count - ")
      .is_some_and(|name| pivot_name_eq(field, name))
}

fn pivot_name_eq(left: &str, right: &str) -> bool {
  left.eq_ignore_ascii_case(right)
}

fn pivot_value_eq(left: &str, right: &str) -> bool {
  left.eq_ignore_ascii_case(right)
}

fn pivot_row_filter_is_ambiguous<'doc>(
  book: &FormulaEvaluationBook<'doc>,
  sheet: SheetId,
  pivot: &FormulaPivotTable<'doc>,
  request: &PivotDataRequest<'doc>,
) -> bool {
  let row_fields = pivot
    .fields
    .iter()
    .filter(|field| field.orientation == FormulaPivotFieldOrientation::Row)
    .collect::<Vec<_>>();
  if row_fields.len() <= 1 {
    return false;
  }
  let row_filters = row_fields
    .iter()
    .filter_map(|field| {
      request
        .filters
        .iter()
        .find(|filter| pivot_name_eq(&field.name, &filter.field_name))
        .map(|filter| (field.name.as_ref(), filter.match_value.as_ref()))
    })
    .collect::<Vec<_>>();
  if row_filters.is_empty() || row_filters.len() == row_fields.len() {
    return false;
  }

  let target = &pivot.target.range;
  let mut inherited = vec![String::new(); row_fields.len()];
  let mut matches = 0usize;
  for row in target.start.row.saturating_add(1)..=target.end.row {
    for (index, inherited_value) in inherited.iter_mut().enumerate() {
      let value = book.cell_value(
        sheet,
        CellAddress {
          column: target.start.column + index as u32,
          row,
        },
      );
      if !matches!(value, FormulaValue::Blank) {
        *inherited_value = pivot_output_cell_text(&value);
      }
    }
    if row_filters.iter().all(|(field_name, expected)| {
      row_fields
        .iter()
        .position(|field| pivot_name_eq(&field.name, field_name))
        .is_some_and(|index| pivot_value_eq(&inherited[index], expected))
    }) {
      matches += 1;
      if matches > 1 {
        return true;
      }
    }
  }
  matches != 1
}

fn pivot_output_cell_text(value: &FormulaValue<'_>) -> String {
  match value {
    FormulaValue::String(value) => value.to_string(),
    FormulaValue::Blank => String::new(),
    _ => display_text_from_value(value),
  }
}

fn parse_getpivotdata_filter_text<'doc>(
  text: &str,
) -> (Option<Cow<'doc, str>>, Vec<PivotFieldFilter<'doc>>) {
  let mut filters = Vec::new();
  let mut index = 0usize;
  while let Some(open_rel) = text[index..].find('[') {
    let open = index + open_rel;
    let field = text[index..open].trim();
    let Some(close_rel) = text[open + 1..].find(']') else {
      break;
    };
    let close = open + 1 + close_rel;
    let mut item = text[open + 1..close].trim();
    if item.len() >= 2 && item.starts_with('\'') && item.ends_with('\'') {
      item = &item[1..item.len() - 1];
    }
    if !field.is_empty() {
      filters.push(PivotFieldFilter {
        field_name: Cow::Owned(field.to_string()),
        match_value: Cow::Owned(item.to_string()),
      });
    }
    index = close + 1;
  }
  (None, filters)
}

fn matrix_stat_number(value: &FormulaValue<'_>) -> Option<f64> {
  match value {
    FormulaValue::Number(value) => Some(*value),
    FormulaValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
    _ => None,
  }
}

fn formula_cell_numeric_value(value: &FormulaValue<'_>) -> Option<f64> {
  match value {
    FormulaValue::Number(value) => Some(*value),
    FormulaValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
    _ => None,
  }
}

fn sumproduct_merge_scalar(
  current: SumProductScalar,
  value: &FormulaValue<'_>,
) -> SumProductScalar {
  match value {
    FormulaValue::String(_) => SumProductScalar::NaN,
    FormulaValue::Blank => match current {
      SumProductScalar::Number(value) => SumProductScalar::Number(value * 0.0),
      value => value,
    },
    FormulaValue::Error(error) => match current {
      SumProductScalar::NaN => SumProductScalar::NaN,
      _ => SumProductScalar::Error(*error),
    },
    FormulaValue::Number(value) => match current {
      SumProductScalar::Number(current) => SumProductScalar::Number(current * value),
      value => value,
    },
    FormulaValue::Boolean(value) => match current {
      SumProductScalar::Number(current) => {
        SumProductScalar::Number(current * if *value { 1.0 } else { 0.0 })
      }
      value => value,
    },
    FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_) => current,
  }
}

fn value_number_for_array(value: &FormulaValue<'_>) -> Option<f64> {
  match value {
    FormulaValue::Number(value) => Some(*value),
    FormulaValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
    _ => None,
  }
}

fn number_only(value: &FormulaValue<'_>) -> Option<f64> {
  match value {
    FormulaValue::Number(value) => Some(*value),
    _ => None,
  }
}

fn covariance_pairs(
  left: &[Vec<FormulaValue<'_>>],
  right: &[Vec<FormulaValue<'_>>],
) -> Option<Vec<(f64, f64)>> {
  let left_rows = left.len();
  let left_columns = left.first().map_or(0, Vec::len);
  let right_rows = right.len();
  let right_columns = right.first().map_or(0, Vec::len);
  if left_rows != right_rows || left_columns != right_columns {
    return None;
  }
  let mut pairs = Vec::new();
  for row in 0..left_rows {
    if left[row].len() != left_columns || right[row].len() != right_columns {
      return None;
    }
    for column in 0..left_columns {
      let Some(left_value) = value_number_for_array(&left[row][column]) else {
        continue;
      };
      let Some(right_value) = value_number_for_array(&right[row][column]) else {
        continue;
      };
      pairs.push((left_value, right_value));
    }
  }
  Some(pairs)
}

fn arithmetic_matrix_number(value: &FormulaValue<'_>) -> Option<f64> {
  match value {
    FormulaValue::Number(value) => Some(*value),
    FormulaValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
    FormulaValue::Blank => Some(0.0),
    _ => None,
  }
}

fn arithmetic_operator_matrix_number(value: &FormulaValue<'_>) -> Option<f64> {
  match value {
    FormulaValue::Number(value) => Some(*value),
    FormulaValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
    FormulaValue::Blank => Some(0.0),
    _ => None,
  }
}

fn matrix_numbers<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  matrix: &[Vec<FormulaValue<'doc>>],
) -> Option<Vec<f64>> {
  matrix
    .iter()
    .flatten()
    .map(|value| evaluator.number(value))
    .collect()
}

fn matrix_number_at<'doc>(
  matrix: &[Vec<FormulaValue<'doc>>],
  row: usize,
  column: usize,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<f64> {
  matrix
    .get(row)
    .and_then(|values| values.get(column))
    .and_then(|value| evaluator.number(value))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MatrixShape {
  rows: usize,
  columns: usize,
}

impl MatrixShape {
  fn from_matrix<T>(matrix: &[Vec<T>]) -> Option<Self> {
    let rows = matrix.len();
    let columns = matrix.first().map_or(0, Vec::len);
    if rows == 0 || columns == 0 || matrix.iter().any(|row| row.len() != columns) {
      None
    } else {
      Some(Self { rows, columns })
    }
  }

  fn len(self) -> usize {
    self.rows * self.columns
  }

  fn materialize<'doc>(self, values: Vec<FormulaValue<'doc>>) -> Vec<Vec<FormulaValue<'doc>>> {
    let mut rows = Vec::with_capacity(self.rows);
    let mut iter = values.into_iter();
    for _ in 0..self.rows {
      rows.push(iter.by_ref().take(self.columns).collect());
    }
    rows
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RegressionCase {
  Simple,
  ColumnY,
  RowY,
}

#[derive(Clone, Debug)]
pub(crate) struct RegressionData {
  case: RegressionCase,
  x_shape: MatrixShape,
  y: Vec<f64>,
  design: Vec<Vec<f64>>,
}

#[derive(Clone, Debug)]
struct RegressionPrediction {
  shape: MatrixShape,
  design: Vec<Vec<f64>>,
}

impl RegressionData {
  fn k(&self) -> usize {
    self.design.first().map_or(0, Vec::len)
  }

  fn n(&self) -> usize {
    self.y.len()
  }

  fn default_prediction_matrix(&self) -> RegressionPrediction {
    let shape = match self.case {
      RegressionCase::Simple => self.x_shape,
      RegressionCase::ColumnY => MatrixShape {
        rows: self.design.len(),
        columns: 1,
      },
      RegressionCase::RowY => MatrixShape {
        rows: 1,
        columns: self.design.len(),
      },
    };
    RegressionPrediction {
      shape,
      design: self.design.clone(),
    }
  }

  fn prediction_matrix<'doc>(
    &self,
    evaluator: &FormulaEvaluator<'_, 'doc>,
    value: &FormulaValue<'doc>,
  ) -> std::result::Result<RegressionPrediction, FormulaErrorValue> {
    let matrix = evaluator.matrix_values(value);
    let shape = MatrixShape::from_matrix(&matrix).ok_or(FormulaErrorValue::IllegalArgument)?;
    let numbers = matrix_numbers(evaluator, &matrix).ok_or(FormulaErrorValue::IllegalArgument)?;
    if numbers.len() != shape.len() {
      return Err(FormulaErrorValue::IllegalArgument);
    }
    match self.case {
      RegressionCase::Simple => Ok(RegressionPrediction {
        shape,
        design: numbers.into_iter().map(|value| vec![value]).collect(),
      }),
      RegressionCase::ColumnY => {
        if shape.columns != self.k() {
          return Err(FormulaErrorValue::IllegalArgument);
        }
        let mut design = Vec::with_capacity(shape.rows);
        for row in 0..shape.rows {
          let mut values = Vec::with_capacity(shape.columns);
          for column in 0..shape.columns {
            values.push(
              matrix_number_at(&matrix, row, column, evaluator)
                .ok_or(FormulaErrorValue::IllegalArgument)?,
            );
          }
          design.push(values);
        }
        Ok(RegressionPrediction {
          shape: MatrixShape {
            rows: shape.rows,
            columns: 1,
          },
          design,
        })
      }
      RegressionCase::RowY => {
        if shape.rows != self.k() {
          return Err(FormulaErrorValue::IllegalArgument);
        }
        let mut design = Vec::with_capacity(shape.columns);
        for column in 0..shape.columns {
          let mut values = Vec::with_capacity(shape.rows);
          for row in 0..shape.rows {
            values.push(
              matrix_number_at(&matrix, row, column, evaluator)
                .ok_or(FormulaErrorValue::IllegalArgument)?,
            );
          }
          design.push(values);
        }
        Ok(RegressionPrediction {
          shape: MatrixShape {
            rows: 1,
            columns: shape.columns,
          },
          design,
        })
      }
    }
  }
}

fn regression_coefficients<'doc>(
  data: &RegressionData,
  constant: bool,
  stats: bool,
  log_regression: bool,
) -> Option<Vec<Vec<FormulaValue<'doc>>>> {
  if data.case == RegressionCase::Simple {
    return simple_regression_coefficients(data, constant, stats, log_regression);
  }
  let mut state = calc_regression_state(&data.y, &data.design, constant)?;
  let k = data.k();
  let rows = if stats { 5 } else { 1 };
  let mut result = vec![vec![FormulaValue::Error(FormulaErrorValue::NA); k + 1]; rows];
  result[0][k] = FormulaValue::Number(if log_regression {
    state.model.intercept.exp()
  } else {
    state.model.intercept
  });
  for index in 0..k {
    result[0][k - 1 - index] = FormulaValue::Number(if log_regression {
      state.model.slopes[index].exp()
    } else {
      state.model.slopes[index]
    });
  }
  if !stats {
    return Some(result);
  }
  regression_fill_stats(data, constant, &mut state, &mut result);
  Some(result)
}

fn simple_regression_coefficients<'doc>(
  data: &RegressionData,
  constant: bool,
  stats: bool,
  log_regression: bool,
) -> Option<Vec<Vec<FormulaValue<'doc>>>> {
  // Source: LibreOffice sc/source/core/tool/interpr5.cxx
  // ScInterpreter::CalculateRGPRKP nCase==1.
  let n = data.n();
  if (constant && n < 2) || n < 1 {
    return None;
  }
  let mut x = data
    .design
    .iter()
    .map(|row| row.first().copied())
    .collect::<Option<Vec<_>>>()?;
  let mut y = data.y.clone();
  let mut mean_x = 0.0;
  let mut mean_y = 0.0;
  if constant {
    mean_y = kahan_sum(y.iter().copied()) / n as f64;
    for value in &mut y {
      *value = approx_sub(*value, mean_y);
    }
    mean_x = kahan_sum(x.iter().copied()) / n as f64;
    for value in &mut x {
      *value = approx_sub(*value, mean_x);
    }
  }
  let sum_xy = kahan_sum(x.iter().zip(&y).map(|(x, y)| x * y));
  let sum_x2 = kahan_sum(x.iter().map(|value| value * value));
  if sum_x2 == 0.0 {
    return None;
  }
  let slope = sum_xy / sum_x2;
  let intercept = if constant {
    mean_y - slope * mean_x
  } else {
    0.0
  };
  let mut result = if stats {
    vec![vec![FormulaValue::Error(FormulaErrorValue::NA); 2]; 5]
  } else {
    vec![vec![FormulaValue::Error(FormulaErrorValue::NA); 2]; 1]
  };
  result[0][0] = FormulaValue::Number(if log_regression { slope.exp() } else { slope });
  result[0][1] = FormulaValue::Number(if log_regression {
    intercept.exp()
  } else {
    intercept
  });
  if !stats {
    return Some(result);
  }

  let ss_reg = slope * slope * sum_x2;
  let degrees_freedom = (if constant { n - 2 } else { n - 1 }) as f64;
  let ss_resid = kahan_sum(x.iter().zip(&y).map(|(x, y)| {
    let temp = y - slope * x;
    temp * temp
  }));
  result[4][0] = FormulaValue::Number(ss_reg);
  result[3][1] = FormulaValue::Number(degrees_freedom);
  result[4][1] = FormulaValue::Number(ss_resid);
  if degrees_freedom == 0.0 || ss_resid == 0.0 || ss_reg == 0.0 {
    result[4][1] = FormulaValue::Number(0.0);
    result[3][0] = FormulaValue::Error(FormulaErrorValue::NA);
    result[2][1] = FormulaValue::Number(0.0);
    result[1][0] = FormulaValue::Number(0.0);
    result[1][1] = if constant {
      FormulaValue::Number(0.0)
    } else {
      FormulaValue::Error(FormulaErrorValue::NA)
    };
    result[2][0] = FormulaValue::Number(1.0);
    return Some(result);
  }

  result[3][0] = FormulaValue::Number(ss_reg / (ss_resid / degrees_freedom));
  let rmse = (ss_resid / degrees_freedom).sqrt();
  result[2][1] = FormulaValue::Number(rmse);
  result[1][0] = FormulaValue::Number(rmse / sum_x2.sqrt());
  result[1][1] = if constant {
    FormulaValue::Number(rmse * (mean_x * mean_x / sum_x2 + 1.0 / n as f64).sqrt())
  } else {
    FormulaValue::Error(FormulaErrorValue::NA)
  };
  result[2][0] = FormulaValue::Number(ss_reg / (ss_reg + ss_resid));
  Some(result)
}

fn regression_model(data: &RegressionData, constant: bool) -> Option<RegressionModel> {
  if data.case == RegressionCase::Simple {
    return calc_regression_model(&data.y, &data.design, constant);
  }
  calc_regression_state(&data.y, &data.design, constant).map(|state| state.model)
}

fn regression_scalar_state<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  args: &[FormulaAst<'doc>],
) -> Option<RegressionScalarState> {
  if args.len() != 2 {
    return None;
  }
  regression_scalar_state_for_values(
    evaluator,
    &evaluator.evaluate(args.first()?)?,
    &evaluator.evaluate(args.get(1)?)?,
  )
}

fn regression_scalar_state_for_values<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  y_value: &FormulaValue<'doc>,
  x_value: &FormulaValue<'doc>,
) -> Option<RegressionScalarState> {
  let y_values = evaluator.value_numbers(y_value);
  let x_values = evaluator.value_numbers(x_value);
  Some(regression_scalar_state_from_slices(&y_values, &x_values))
}

fn regression_fill_stats<'doc>(
  data: &RegressionData,
  constant: bool,
  state: &mut RegressionState,
  result: &mut [Vec<FormulaValue<'doc>>],
) {
  let k = data.k();
  let n = data.n();
  let mut z = vec![0.0; n];
  for (row, value) in z.iter_mut().enumerate().take(k) {
    *value = state.r_diagonal[row] * state.model.slopes[row];
    for column in row + 1..k {
      *value += state.centered_x[row][column] * state.model.slopes[column];
    }
  }
  for column in (0..k).rev() {
    apply_householder(&state.centered_x, column, &mut z, n);
  }
  let ss_reg = z.iter().map(|value| value * value).sum::<f64>();
  let mut residual = state.centered_y.clone();
  for (value, fitted) in residual.iter_mut().zip(&z) {
    *value -= fitted;
  }
  let ss_resid = residual.iter().map(|value| value * value).sum::<f64>();
  result[4][0] = FormulaValue::Number(ss_reg);
  result[4][1] = FormulaValue::Number(ss_resid);
  let degrees_freedom = if constant {
    (n - k - 1) as f64
  } else {
    (n - k) as f64
  };
  result[3][1] = FormulaValue::Number(degrees_freedom);
  if degrees_freedom == 0.0 || ss_resid == 0.0 || ss_reg == 0.0 {
    result[4][1] = FormulaValue::Number(0.0);
    result[3][0] = FormulaValue::Error(FormulaErrorValue::NA);
    result[2][1] = FormulaValue::Number(0.0);
    for index in 0..k {
      result[1][k - 1 - index] = FormulaValue::Number(0.0);
    }
    result[1][k] = if constant {
      FormulaValue::Number(0.0)
    } else {
      FormulaValue::Error(FormulaErrorValue::NA)
    };
    result[2][0] = FormulaValue::Number(1.0);
    return;
  }

  result[3][0] = FormulaValue::Number((ss_reg / k as f64) / (ss_resid / degrees_freedom));
  let rmse = (ss_resid / degrees_freedom).sqrt();
  result[2][1] = FormulaValue::Number(rmse);
  let mut sigma_intercept = 0.0;
  for column in 0..k {
    let mut unit = vec![0.0; k];
    unit[column] = 1.0;
    solve_lower(&state.centered_x, &state.r_diagonal, &mut unit, k);
    solve_upper(&state.centered_x, &state.r_diagonal, &mut unit, k);
    result[1][k - 1 - column] = FormulaValue::Number(rmse * unit[column].sqrt());
    if constant {
      sigma_intercept += state.means[column]
        * state
          .means
          .iter()
          .zip(&unit)
          .map(|(m, u)| m * u)
          .sum::<f64>();
    }
  }
  result[1][k] = if constant {
    FormulaValue::Number(rmse * (sigma_intercept + 1.0 / n as f64).sqrt())
  } else {
    FormulaValue::Error(FormulaErrorValue::NA)
  };
  result[2][0] = FormulaValue::Number(ss_reg / (ss_reg + ss_resid));
}

fn lookup_vector<'doc>(
  matrix: &[Vec<FormulaValue<'doc>>],
) -> Option<(Vec<FormulaValue<'doc>>, bool)> {
  let rows = matrix.len();
  let columns = matrix.first().map_or(0, Vec::len);
  if rows == 0 || columns == 0 {
    return None;
  }
  let vertical = rows >= columns;
  lookup_vector_with_orientation(matrix, vertical).map(|values| (values, vertical))
}

fn lookup_vector_with_orientation<'doc>(
  matrix: &[Vec<FormulaValue<'doc>>],
  vertical: bool,
) -> Option<Vec<FormulaValue<'doc>>> {
  if matrix.is_empty() || matrix.first().is_none_or(Vec::is_empty) {
    return None;
  }
  let values = if vertical {
    matrix
      .iter()
      .filter_map(|row| row.first().cloned())
      .collect::<Vec<_>>()
  } else {
    matrix.first()?.clone()
  };
  Some(values)
}

fn lookup_search_vector_omitting_errors<'doc>(
  vector: &[FormulaValue<'doc>],
) -> (Option<Vec<FormulaValue<'doc>>>, Option<Vec<usize>>) {
  if !vector
    .iter()
    .all(|value| matches!(value, FormulaValue::Number(_) | FormulaValue::Error(_)))
  {
    return (None, None);
  }
  let mut values = Vec::new();
  let mut indices = Vec::new();
  for (index, value) in vector.iter().enumerate() {
    if matches!(value, FormulaValue::Number(_)) {
      values.push(value.clone());
      indices.push(index);
    }
  }
  if values.len() == vector.len() {
    (None, None)
  } else {
    (Some(values), Some(indices))
  }
}

fn vhlookup_matrix_index<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  lookup: &FormulaValue<'doc>,
  matrix: &[Vec<FormulaValue<'doc>>],
  horizontal: bool,
  sorted: bool,
) -> Option<usize> {
  let count = if horizontal {
    matrix.first().map_or(0, Vec::len)
  } else {
    matrix.len()
  };
  if count == 0 {
    return None;
  }
  let lookup_is_text = matches!(lookup, FormulaValue::String(_) | FormulaValue::Blank);
  let mut index = None;
  if lookup_is_text {
    let lookup_text = evaluator.text(lookup);
    if sorted {
      for candidate_index in 0..count {
        let candidate = vhlookup_search_cell(matrix, horizontal, candidate_index);
        if candidate.is_none_or(is_lookup_string_or_empty) {
          let candidate_text = candidate
            .map(|value| evaluator.text(value))
            .unwrap_or_default();
          let ordering = compare_text(evaluator, &candidate_text, &lookup_text, false);
          if ordering <= 0 {
            index = Some(candidate_index);
          } else if candidate_index > 0 {
            break;
          }
        } else {
          index = Some(candidate_index);
        }
      }
    } else {
      for candidate_index in 0..count {
        let candidate = vhlookup_search_cell(matrix, horizontal, candidate_index);
        if candidate.is_none_or(is_lookup_string_or_empty)
          && compare_text(
            evaluator,
            &candidate
              .map(|value| evaluator.text(value))
              .unwrap_or_default(),
            &lookup_text,
            false,
          ) == 0
        {
          index = Some(candidate_index);
          break;
        }
      }
    }
    if index.is_some_and(|index| {
      matches!(
        vhlookup_search_cell(matrix, horizontal, index),
        Some(FormulaValue::Number(_))
      )
    }) {
      return None;
    }
    return index;
  }

  let lookup_number = formula_cell_numeric_value(lookup)?;
  if sorted {
    for candidate_index in 0..count {
      let Some(candidate) = vhlookup_search_cell(matrix, horizontal, candidate_index) else {
        continue;
      };
      if is_lookup_string_or_empty(candidate) {
        continue;
      }
      let Some(candidate_number) = formula_cell_numeric_value(candidate) else {
        break;
      };
      if candidate_number <= lookup_number {
        index = Some(candidate_index);
      } else {
        break;
      }
    }
  } else {
    for candidate_index in 0..count {
      let Some(candidate) = vhlookup_search_cell(matrix, horizontal, candidate_index) else {
        continue;
      };
      if is_lookup_string_or_empty(candidate) {
        continue;
      }
      if formula_cell_numeric_value(candidate)
        .is_some_and(|candidate_number| compare_numbers(candidate_number, lookup_number) == 0)
      {
        index = Some(candidate_index);
        break;
      }
    }
  }
  index
}

fn vhlookup_search_cell<'doc>(
  matrix: &'doc [Vec<FormulaValue<'doc>>],
  horizontal: bool,
  index: usize,
) -> Option<&'doc FormulaValue<'doc>> {
  if horizontal {
    matrix.first().and_then(|row| row.get(index))
  } else {
    matrix.get(index).and_then(|row| row.first())
  }
}

fn is_lookup_string_or_empty(value: &FormulaValue<'_>) -> bool {
  matches!(value, FormulaValue::String(_) | FormulaValue::Blank)
}

fn choose_row_column_index(index: i64, len: usize) -> Option<usize> {
  if index == 0 {
    return None;
  }
  let len = i64::try_from(len).ok()?;
  let normalized = if index < 0 { len + index + 1 } else { index };
  (1..=len)
    .contains(&normalized)
    .then_some((normalized - 1) as usize)
}

fn is_blank_for_countblank(value: &FormulaValue<'_>) -> bool {
  matches!(value, FormulaValue::Blank)
    || matches!(value, FormulaValue::String(text) if text.is_empty())
}

fn aggregate_counta<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  args: &[FormulaValue<'doc>],
  options: AggregateOptions,
) -> Option<std::result::Result<usize, FormulaErrorValue>> {
  let mut count = 0usize;
  for arg in args {
    match aggregate_counta_value(evaluator, arg, options, &mut count) {
      Ok(()) => {}
      Err(error) => return Some(Err(error)),
    }
  }
  Some(Ok(count))
}

fn aggregate_counta_value<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  value: &FormulaValue<'doc>,
  options: AggregateOptions,
  count: &mut usize,
) -> std::result::Result<(), FormulaErrorValue> {
  match value {
    FormulaValue::Reference(reference) => {
      if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
        return Ok(());
      }
      let sheet = evaluator.range_sheet(reference);
      for row in reference.range.start.row..=reference.range.end.row {
        if (options.ignore_filtered && evaluator.book.row_filtered(sheet, row))
          || (options.ignore_hidden && evaluator.book.row_hidden(sheet, row))
        {
          continue;
        }
        for column in reference.range.start.column..=reference.range.end.column {
          let address = CellAddress { column, row };
          if options.ignore_nested && evaluator.book.is_nested_aggregate(sheet, address) {
            continue;
          }
          aggregate_counta_scalar(evaluator.book.cell_value(sheet, address), options, count)?;
        }
      }
      Ok(())
    }
    FormulaValue::Matrix(rows) => {
      for value in rows.iter().flatten() {
        aggregate_counta_scalar(value.clone(), options, count)?;
      }
      Ok(())
    }
    value => aggregate_counta_scalar(value.clone(), options, count),
  }
}

fn aggregate_counta_scalar(
  value: FormulaValue<'_>,
  options: AggregateOptions,
  count: &mut usize,
) -> std::result::Result<(), FormulaErrorValue> {
  match value {
    FormulaValue::Blank => {}
    FormulaValue::Error(_) if options.ignore_errors => {}
    _ => *count += 1,
  }
  Ok(())
}

fn permutationa_value<'doc>(count: f64, chosen: f64) -> FormulaValue<'doc> {
  let count = approx_floor(count);
  let chosen = approx_floor(chosen);
  match permutation_with_repetition_count(count, chosen) {
    Some(value) => FormulaValue::Number(value),
    None => FormulaValue::Error(FormulaErrorValue::Num),
  }
}

fn holiday_serials(
  value: Option<&FormulaValue<'_>>,
  evaluator: &FormulaEvaluator<'_, '_>,
) -> Vec<i64> {
  let Some(value) = value else {
    return Vec::new();
  };
  let mut holidays = Vec::new();
  for value in evaluator.matrix_values(value).into_iter().flatten() {
    match value {
      FormulaValue::Error(_) | FormulaValue::Blank => {}
      value => {
        if let Some(serial) = evaluator.date_number_from_value(&value) {
          holidays.push(serial.floor() as i64);
        }
      }
    }
  }
  holidays.sort_unstable();
  holidays.dedup();
  holidays
}

fn weekend_mask(
  value: Option<&FormulaValue<'_>>,
  workday_function: bool,
  evaluator: &FormulaEvaluator<'_, '_>,
) -> Option<[bool; 7]> {
  // Source: LibreOffice sc/source/core/tool/interpr2.cxx GetWeekendAndHolidayMasks_MS.
  let mut mask = [false; 7];
  let Some(value) = value else {
    mask[5] = true;
    mask[6] = true;
    return Some(mask);
  };
  let text = match value {
    FormulaValue::Blank if workday_function => return None,
    FormulaValue::Blank => String::new(),
    FormulaValue::Matrix(rows) => {
      return weekend_mask(
        rows.first().and_then(|row| row.first()),
        workday_function,
        evaluator,
      );
    }
    FormulaValue::Number(number) => {
      if (1.0..=17.0).contains(number) {
        display_text_from_value(&FormulaValue::Number(number.floor()))
      } else {
        return None;
      }
    }
    FormulaValue::String(text) => {
      if text.is_empty() || text.len() != 7 || (workday_function && text == "1111111") {
        return None;
      }
      text.to_string()
    }
    _ => evaluator.text(value),
  };
  if text.is_empty() {
    mask[5] = true;
    mask[6] = true;
    return Some(mask);
  }
  if workday_function && text == "1111111" {
    return None;
  }
  match text.len() {
    1 => match text.as_str() {
      "1" => {
        mask[5] = true;
        mask[6] = true;
      }
      "2" => {
        mask[6] = true;
        mask[0] = true;
      }
      "3" => {
        mask[0] = true;
        mask[1] = true;
      }
      "4" => {
        mask[1] = true;
        mask[2] = true;
      }
      "5" => {
        mask[2] = true;
        mask[3] = true;
      }
      "6" => {
        mask[3] = true;
        mask[4] = true;
      }
      "7" => {
        mask[4] = true;
        mask[5] = true;
      }
      _ => return None,
    },
    2 => {
      if !text.starts_with('1') {
        return None;
      }
      match text.as_bytes()[1] {
        b'1' => mask[6] = true,
        b'2' => mask[0] = true,
        b'3' => mask[1] = true,
        b'4' => mask[2] = true,
        b'5' => mask[3] = true,
        b'6' => mask[4] = true,
        b'7' => mask[5] = true,
        _ => return None,
      }
    }
    7 => {
      for (index, byte) in text.bytes().enumerate() {
        match byte {
          b'0' => mask[index] = false,
          b'1' => mask[index] = true,
          _ => return None,
        }
      }
    }
    _ => return None,
  }
  Some(mask)
}

fn old_networkdays_weekend_mask(
  value: Option<&FormulaValue<'_>>,
  evaluator: &FormulaEvaluator<'_, '_>,
) -> Option<[bool; 7]> {
  // Source: LibreOffice sc/source/core/tool/interpr2.cxx GetWeekendAndHolidayMasks.
  let value = value?;
  let values = evaluator
    .matrix_values(value)
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
  if values.len() != 7 {
    return None;
  }
  let mut source = [false; 7];
  for (index, value) in values.iter().enumerate() {
    source[index] = evaluator.number(value)? != 0.0;
  }
  let mut mask = [false; 7];
  for index in 0..7 {
    mask[index] = source[if index == 6 { 0 } else { index + 1 }];
  }
  Some(mask)
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

pub(crate) fn format_complex_result(value: FormulaComplex) -> String {
  format_formula_complex_number(value, format_complex_component)
}

fn format_complex_component(value: f64, leading_sign: bool) -> String {
  if !value.is_finite() {
    return error_text_value(FormulaErrorValue::Value).to_string();
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
  if let Some((head, tail)) = value.split_once('.') {
    let tail = tail.trim_end_matches('0');
    if tail.is_empty() {
      head.to_string()
    } else {
      format!("{head}.{tail}")
    }
  } else {
    value.to_string()
  }
}

pub(crate) fn split_indirect_intersection(formula: &str) -> Option<(&str, &str)> {
  let upper = formula.to_ascii_uppercase();
  if !upper.starts_with("INDIRECT(") {
    return None;
  }
  let mut depth = 0i32;
  for (index, ch) in formula.char_indices() {
    match ch {
      '(' => depth += 1,
      ')' => {
        depth -= 1;
        if depth == 0 {
          let rest = formula[index + ch.len_utf8()..].trim();
          if rest.to_ascii_uppercase().starts_with("INDIRECT(") {
            return Some((&formula[..=index], rest));
          }
        }
      }
      _ => {}
    }
  }
  None
}

pub(crate) fn range_intersection_value<'doc>(
  book: &FormulaEvaluationBook<'doc>,
  left: FormulaValue<'doc>,
  right: FormulaValue<'doc>,
) -> FormulaValue<'doc> {
  let (FormulaValue::Reference(left), FormulaValue::Reference(right)) = (left, right) else {
    return FormulaValue::Error(FormulaErrorValue::Value);
  };
  let left_sheet = left
    .sheet_name
    .as_ref()
    .and_then(|name| book.sheet_id_by_name(&name.0));
  let right_sheet = right
    .sheet_name
    .as_ref()
    .and_then(|name| book.sheet_id_by_name(&name.0));
  let left_sheet = left_sheet.unwrap_or(left.sheet);
  let right_sheet = right_sheet.unwrap_or(right.sheet);
  if left_sheet != right_sheet {
    return FormulaValue::Error(FormulaErrorValue::Value);
  }
  let start = CellAddress {
    column: left.range.start.column.max(right.range.start.column),
    row: left.range.start.row.max(right.range.start.row),
  };
  let end = CellAddress {
    column: left.range.end.column.min(right.range.end.column),
    row: left.range.end.row.min(right.range.end.row),
  };
  if start.column > end.column || start.row > end.row {
    return FormulaValue::Error(FormulaErrorValue::Value);
  }
  book.cell_value(left_sheet, start)
}

pub(crate) fn column_index_to_name(mut column: u32) -> String {
  let mut name = Vec::new();
  loop {
    name.push((b'A' + (column % 26) as u8) as char);
    column /= 26;
    if column == 0 {
      break;
    }
    column -= 1;
  }
  name.into_iter().rev().collect()
}

fn quote_sheet_name_for_reference(sheet: &str) -> String {
  if sheet
    .chars()
    .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
  {
    sheet.to_string()
  } else {
    format!("'{}'", sheet.replace('\'', "''"))
  }
}

fn parse_table_reference<'doc>(
  book: &FormulaEvaluationBook<'doc>,
  text: &str,
  current_address: Option<CellAddress>,
) -> Option<QualifiedRange<'doc>> {
  let selection = crate::parser::parse_table_reference_selection(text)?;
  let table = book
    .tables
    .get(&selection.table_name.to_ascii_uppercase())?;
  let mut range = table_reference_item_range(table, selection.items, current_address)?;
  if !selection.columns.is_empty() {
    let start = table_reference_column_offset(table, selection.columns[0].as_ref())?;
    let end = selection
      .columns
      .last()
      .and_then(|column| table_reference_column_offset(table, column.as_ref()))?;
    let first = start.min(end);
    let last = start.max(end);
    range.start.column = table.range.start.column + first;
    range.end.column = table.range.start.column + last;
  }
  Some(QualifiedRange {
    sheet: table.sheet,
    sheet_name: None,
    range,
    start_flags: AddressFlags::default(),
    end_flags: AddressFlags::default(),
  })
}

fn table_reference_item_range(
  table: &FormulaTable<'_>,
  items: crate::parser::TableReferenceItems,
  current_address: Option<CellAddress>,
) -> Option<CellRange> {
  use crate::parser::TableReferenceItems;

  let mut start_row = table.range.start.row;
  let mut end_row = table.range.end.row;
  if items.contains(TableReferenceItems::THIS_ROW) {
    let row = current_address?.row;
    if row < start_row || row > end_row {
      return None;
    }
    start_row = row;
    end_row = row;
  } else if items.contains(TableReferenceItems::ALL) {
  } else if items.contains(TableReferenceItems::HEADERS)
    && !items.contains(TableReferenceItems::DATA)
    && !items.contains(TableReferenceItems::TOTALS)
  {
    if table.header_rows == 0 {
      return None;
    }
    end_row = start_row + table.header_rows - 1;
  } else if items.contains(TableReferenceItems::TOTALS)
    && !items.contains(TableReferenceItems::HEADERS)
    && !items.contains(TableReferenceItems::DATA)
  {
    if table.totals_rows == 0 {
      return None;
    }
    start_row = end_row + 1 - table.totals_rows;
  } else {
    if !items.contains(TableReferenceItems::HEADERS) && table.header_rows > 0 {
      start_row += table.header_rows;
    }
    if !items.contains(TableReferenceItems::TOTALS) && table.totals_rows > 0 {
      end_row = end_row.saturating_sub(table.totals_rows);
    }
  }
  if start_row > end_row {
    return None;
  }
  Some(CellRange::new(
    CellAddress {
      column: table.range.start.column,
      row: start_row,
    },
    CellAddress {
      column: table.range.end.column,
      row: end_row,
    },
  ))
}

fn table_reference_column_offset(table: &FormulaTable<'_>, column: &str) -> Option<u32> {
  table
    .columns
    .iter()
    .position(|name| name.as_ref().eq_ignore_ascii_case(column))
    .map(|index| index as u32)
}

fn chisq_dist_value<'doc>(
  x: f64,
  df: f64,
  right_tail: bool,
  cumulative: Option<bool>,
) -> FormulaValue<'doc> {
  if df < 1.0 || x < 0.0 {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  if right_tail {
    return FormulaValue::Number(lo_chi_dist(x, df));
  }
  FormulaValue::Number(if cumulative.unwrap_or(true) {
    lo_chisq_dist_cdf(x, df)
  } else {
    lo_chisq_dist_pdf(x, df)
  })
}

fn chisq_inv_value<'doc>(p: f64, df: f64, right_tail: bool) -> FormulaValue<'doc> {
  if df < 1.0 || (right_tail && (p <= 0.0 || p > 1.0)) || (!right_tail && (p < 0.0 || p >= 1.0)) {
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

fn special_error_value(error: SpecialError) -> FormulaErrorValue {
  match error {
    SpecialError::IllegalArgument => FormulaErrorValue::IllegalArgument,
    SpecialError::Num => FormulaErrorValue::Num,
    SpecialError::Div0 => FormulaErrorValue::Div0,
  }
}

fn numeric_error_value(error: NumericError) -> FormulaErrorValue {
  match error {
    NumericError::IllegalArgument => FormulaErrorValue::IllegalArgument,
    NumericError::Div0 => FormulaErrorValue::Div0,
    NumericError::Value => FormulaErrorValue::Value,
  }
}

fn ets_error_value(error: EtsError) -> FormulaErrorValue {
  match error {
    EtsError::IllegalArgument => FormulaErrorValue::IllegalArgument,
    EtsError::Num => FormulaErrorValue::Num,
    EtsError::Value => FormulaErrorValue::Value,
    EtsError::Div0 => FormulaErrorValue::Div0,
  }
}

fn statistics_error_value(error: StatisticsError) -> FormulaErrorValue {
  match error {
    StatisticsError::Div0 => FormulaErrorValue::Div0,
    StatisticsError::IllegalArgument => FormulaErrorValue::IllegalArgument,
  }
}

fn propagate_binary_error(
  left: &FormulaValue<'_>,
  right: &FormulaValue<'_>,
) -> Option<FormulaErrorValue> {
  match (left, right) {
    (FormulaValue::Error(error), _) | (_, FormulaValue::Error(error)) => Some(*error),
    _ => None,
  }
}

fn first_error_in_values(values: &[&FormulaValue<'_>]) -> Option<FormulaErrorValue> {
  values.iter().find_map(|value| match value {
    FormulaValue::Error(error) => Some(*error),
    _ => None,
  })
}

fn first_error_in_value<'doc>(value: &FormulaValue<'doc>) -> Option<FormulaValue<'doc>> {
  match value {
    FormulaValue::Error(error) => Some(FormulaValue::Error(*error)),
    FormulaValue::Matrix(rows) => rows.iter().flatten().find_map(first_error_in_value),
    _ => None,
  }
}

fn format_dollar_value(value: f64, digits: usize) -> String {
  let sign = if value.is_sign_negative() { "-" } else { "" };
  let formatted = format!("{:.*}", digits, value.abs());
  let (integer, fraction) = formatted.split_once('.').unwrap_or((&formatted, ""));
  let mut grouped = String::new();
  for (index, ch) in integer.chars().rev().enumerate() {
    if index > 0 && index % 3 == 0 {
      grouped.push(',');
    }
    grouped.push(ch);
  }
  let integer = grouped.chars().rev().collect::<String>();
  if digits == 0 {
    format!("{sign}${integer}")
  } else {
    format!("{sign}${integer}.{fraction}")
  }
}

fn should_ignore_to_row_column_value(value: &FormulaValue<'_>, ignore: i32) -> bool {
  let ignore_blanks = matches!(ignore, 1 | 3);
  let ignore_errors = matches!(ignore, 2 | 3);
  (ignore_blanks && matches!(value, FormulaValue::Blank))
    || (ignore_errors && matches!(value, FormulaValue::Error(_)))
}

pub(crate) fn timevalue(text: &str) -> FormulaValue<'static> {
  let text = text.trim();
  let text = time_text_suffix(text);
  let lower = text.to_ascii_lowercase();
  let (body, meridiem) = if let Some(body) = lower.strip_suffix("am") {
    (body.trim(), Some(false))
  } else if let Some(body) = lower.strip_suffix("pm") {
    (body.trim(), Some(true))
  } else {
    (text, None)
  };
  let parts = body.split(':').collect::<Vec<_>>();
  if parts.len() < 2 && meridiem.is_none() {
    return FormulaValue::Error(FormulaErrorValue::Value);
  }
  let mut hour = match parts.first().and_then(|part| part.parse::<f64>().ok()) {
    Some(hour) => hour,
    None => return FormulaValue::Error(FormulaErrorValue::Value),
  };
  let minute = parts
    .get(1)
    .and_then(|part| part.parse::<f64>().ok())
    .unwrap_or(0.0);
  let second = parts
    .get(2)
    .and_then(|part| part.parse::<f64>().ok())
    .unwrap_or(0.0);
  if let Some(pm) = meridiem {
    if !(1.0..=12.0).contains(&hour) {
      return FormulaValue::Error(FormulaErrorValue::Value);
    }
    hour = if pm {
      if hour == 12.0 { 12.0 } else { hour + 12.0 }
    } else if hour == 12.0 {
      0.0
    } else {
      hour
    };
  }
  if hour < 0.0 || minute < 0.0 || second < 0.0 {
    return FormulaValue::Error(FormulaErrorValue::Value);
  }
  let seconds = (hour * 3600.0 + minute * 60.0 + second).rem_euclid(86_400.0);
  FormulaValue::Number(seconds / 86_400.0)
}

fn time_text_suffix(text: &str) -> &str {
  let trimmed = text.trim();
  if let Some(index) = trimmed.find('T') {
    let suffix = trimmed[index + 1..].trim();
    if suffix.contains(':') {
      return suffix;
    }
  }
  for (index, ch) in trimmed.char_indices().rev() {
    if ch.is_ascii_whitespace() {
      let suffix = trimmed[index..].trim();
      if suffix.contains(':') {
        return suffix;
      }
      break;
    }
  }
  trimmed
}

fn euro_convert(
  value: f64,
  from: &str,
  to: &str,
  full_precision: bool,
  precision: f64,
) -> Option<f64> {
  let (from_rate, _) = euro_currency_info(from)?;
  let (to_rate, to_decimals) = euro_currency_info(to)?;
  let mut result = if from.eq_ignore_ascii_case(to) {
    value
  } else if from.eq_ignore_ascii_case("EUR") {
    value * to_rate
  } else {
    let mut intermediate = value / from_rate;
    if precision != 0.0 {
      intermediate = round_to_decimal_places(intermediate, precision as i32);
    }
    intermediate * to_rate
  };
  if !full_precision && !from.eq_ignore_ascii_case(to) {
    result = round_to_decimal_places(result, to_decimals);
  }
  Some(result)
}

fn euro_currency_info(unit: &str) -> Option<(f64, i32)> {
  const CURRENCIES: &[(&str, f64, i32)] = &[
    ("EUR", 1.0, 2),
    ("ATS", 13.7603, 2),
    ("BEF", 40.3399, 0),
    ("DEM", 1.95583, 2),
    ("ESP", 166.386, 0),
    ("FIM", 5.94573, 2),
    ("FRF", 6.55957, 2),
    ("IEP", 0.787564, 2),
    ("ITL", 1936.27, 0),
    ("LUF", 40.3399, 0),
    ("NLG", 2.20371, 2),
    ("PTE", 200.482, 2),
    ("GRD", 340.750, 2),
    ("SIT", 239.640, 2),
    ("MTL", 0.429300, 2),
    ("CYP", 0.585274, 2),
    ("SKK", 30.1260, 2),
    ("EEK", 15.6466, 2),
    ("LVL", 0.702804, 2),
    ("LTL", 3.45280, 2),
    ("HRK", 7.53450, 2),
    ("BGN", 1.95583, 2),
  ];
  CURRENCIES
    .iter()
    .find(|(name, _, _)| name.eq_ignore_ascii_case(unit))
    .map(|(_, rate, decimals)| (*rate, *decimals))
}

fn expand_two_digit_year(year: i32) -> i32 {
  if year >= 30 { 1900 + year } else { 2000 + year }
}

#[derive(Clone, Copy, Debug)]
struct XorShift64 {
  state: u64,
}

impl XorShift64 {
  fn new(seed: u64) -> Self {
    Self {
      state: if seed == 0 {
        0x9e37_79b9_7f4a_7c15
      } else {
        seed
      },
    }
  }

  fn next_unit(&mut self) -> f64 {
    let mut value = self.state;
    value ^= value << 13;
    value ^= value >> 7;
    value ^= value << 17;
    self.state = value;
    (value as f64) / (u64::MAX as f64)
  }
}

fn time_seed() -> u64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .map(|duration| duration.as_nanos() as u64)
    .unwrap_or(0x9e37_79b9_7f4a_7c15)
}

fn split_text_multi(
  text: &str,
  delimiters: &[String],
  ignore_empty: bool,
  match_case_insensitive: bool,
) -> Vec<String> {
  if delimiters.is_empty() || text.is_empty() {
    return vec![text.to_string()];
  }
  let search_text = if match_case_insensitive {
    text.to_lowercase()
  } else {
    text.to_string()
  };
  let search_delimiters = delimiters
    .iter()
    .map(|delimiter| {
      if match_case_insensitive {
        delimiter.to_lowercase()
      } else {
        delimiter.clone()
      }
    })
    .collect::<Vec<_>>();
  let mut output = Vec::new();
  let mut start = 0usize;
  while start < text.len() {
    let mut index = text.len();
    let mut delimiter_len = 0usize;
    for delimiter in &search_delimiters {
      if delimiter.is_empty() {
        continue;
      }
      if let Some(position) = search_text[start..].find(delimiter) {
        let position = start + position;
        if position < index {
          index = position;
          delimiter_len = delimiter.len();
        }
      }
    }
    let segment = &text[start..index];
    if !ignore_empty || !segment.is_empty() {
      output.push(segment.to_string());
    }
    if delimiter_len == 0 {
      break;
    }
    start = index + delimiter_len;
  }
  output
}

pub(crate) fn datevalue(text: &str, date_system: DateSystem) -> FormulaValue<'static> {
  let text = text.trim();
  if text.is_empty() {
    return FormulaValue::Error(FormulaErrorValue::Value);
  }
  parse_date_input(text, date_system)
    .map(|value| FormulaValue::Number(value.floor()))
    .unwrap_or(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
}

fn parse_date_input(text: &str, date_system: DateSystem) -> Option<f64> {
  let date = date_input_prefix(text)?;
  parse_iso_date_input(date, date_system)
    .or_else(|| parse_numeric_date_input(date, date_system))
    .or_else(|| parse_month_date_input(date, date_system))
}

fn date_input_prefix(text: &str) -> Option<&str> {
  let trimmed = text.trim();
  if trimmed.is_empty() {
    return None;
  }
  let mut split = trimmed.len();
  for (index, ch) in trimmed.char_indices() {
    if ch == 'T' && trimmed[index + ch.len_utf8()..].contains(':') {
      split = index;
      break;
    }
    if ch.is_ascii_whitespace() {
      let rest = trimmed[index..].trim();
      if rest.is_empty() || rest.chars().any(|ch| ch == ':') {
        split = index;
        break;
      }
    }
  }
  Some(trimmed[..split].trim_end_matches(',').trim())
}

fn parse_iso_date_input(text: &str, date_system: DateSystem) -> Option<f64> {
  let mut parts = text.split('-');
  let year = parts.next()?.parse::<i32>().ok()?;
  let month = parts.next()?.parse::<i32>().ok()?;
  let day = parts.next()?.parse::<i32>().ok()?;
  if parts.next().is_some() {
    return None;
  }
  valid_date_serial_with_system(year, month, day, date_system)
}

fn parse_month_date_input(text: &str, date_system: DateSystem) -> Option<f64> {
  let tokens = date_input_tokens(text)?;
  let month_index = tokens
    .iter()
    .position(|token| matches!(token.kind, DateInputTokenKind::Month(_)))?;
  let month = match tokens[month_index].kind {
    DateInputTokenKind::Month(month) => month,
    DateInputTokenKind::Number(_) => return None,
  };
  let numbers = tokens
    .iter()
    .filter_map(|token| match token.kind {
      DateInputTokenKind::Number(value) => Some((value, token.text)),
      DateInputTokenKind::Month(_) => None,
    })
    .collect::<Vec<_>>();
  let (year, day) = match (month_index, numbers.as_slice()) {
    // Source: LibreOffice ImpSvNumberInputScan::GetDateRef, nMonthPos == 1
    // with two numeric substrings uses MDY in the default English locale.
    (0, [(day, _), (year, _)]) => (*year, *day),
    // For middle-month long dates, LO accepts both DMY and YMD depending on
    // locale/date order. Treat a four-digit leading number as year, otherwise
    // use the common DMY order.
    (1, [(left, left_text), (right, _)]) if left_text.len() >= 4 => (*left, *right),
    (1, [(day, _), (year, _)]) => (*year, *day),
    // Month at the end is accepted as YDM only when the locale long-date order
    // requests it. The English default used by the LO FODS corpus does not.
    (2, _) => return None,
    _ => return None,
  };
  valid_date_serial_with_system(year, i32::from(month), day, date_system)
}

fn parse_numeric_date_input(text: &str, date_system: DateSystem) -> Option<f64> {
  let separator = if text.contains('/') {
    '/'
  } else if text.contains('.') {
    '.'
  } else {
    return None;
  };
  let parts = text.split(separator).collect::<Vec<_>>();
  let [first, second, third] = parts.as_slice() else {
    return None;
  };
  let first_number = first.trim().parse::<i32>().ok()?;
  let second_number = second.trim().parse::<i32>().ok()?;
  let mut third_number = third.trim().parse::<i32>().ok()?;
  if third_number < 100 {
    third_number = expand_two_digit_year(third_number);
  }

  let (year, month, day) = if first.trim().len() >= 4 {
    (first_number, second_number, third_number)
  } else {
    // Source: LibreOffice's default English number formatter parses slash
    // dates such as 5/3/2011 as MDY.
    (third_number, first_number, second_number)
  };
  valid_date_serial_with_system(year, month, day, date_system)
}

#[derive(Clone, Copy)]
struct DateInputToken<'a> {
  kind: DateInputTokenKind,
  text: &'a str,
}

#[derive(Clone, Copy)]
enum DateInputTokenKind {
  Number(i32),
  Month(u8),
}

fn date_input_tokens(text: &str) -> Option<Vec<DateInputToken<'_>>> {
  let mut tokens = Vec::new();
  let mut index = 0usize;
  while index < text.len() {
    let rest = &text[index..];
    let ch = rest.chars().next()?;
    if ch.is_ascii_whitespace() || matches!(ch, ',' | '-' | '/' | '.') {
      index += ch.len_utf8();
      continue;
    }
    if ch.is_ascii_digit() {
      let start = index;
      index += ch.len_utf8();
      while index < text.len() {
        let ch = text[index..].chars().next()?;
        if !ch.is_ascii_digit() {
          break;
        }
        index += ch.len_utf8();
      }
      let value = text[start..index].parse::<i32>().ok()?;
      tokens.push(DateInputToken {
        kind: DateInputTokenKind::Number(value),
        text: &text[start..index],
      });
      continue;
    }
    if ch.is_ascii_alphabetic() {
      let start = index;
      index += ch.len_utf8();
      while index < text.len() {
        let ch = text[index..].chars().next()?;
        if !ch.is_ascii_alphabetic() {
          break;
        }
        index += ch.len_utf8();
      }
      let month = english_month_number(&text[start..index])?;
      tokens.push(DateInputToken {
        kind: DateInputTokenKind::Month(month),
        text: &text[start..index],
      });
      continue;
    }
    return None;
  }
  Some(tokens)
}

fn english_month_number(text: &str) -> Option<u8> {
  match text.to_ascii_uppercase().as_str() {
    "JAN" | "JANUARY" => Some(1),
    "FEB" | "FEBRUARY" => Some(2),
    "MAR" | "MARCH" => Some(3),
    "APR" | "APRIL" => Some(4),
    "MAY" => Some(5),
    "JUN" | "JUNE" => Some(6),
    "JUL" | "JULY" => Some(7),
    "AUG" | "AUGUST" => Some(8),
    "SEP" | "SEPT" | "SEPTEMBER" => Some(9),
    "OCT" | "OCTOBER" => Some(10),
    "NOV" | "NOVEMBER" => Some(11),
    "DEC" | "DECEMBER" => Some(12),
    _ => None,
  }
}

pub(crate) fn valid_date_serial_with_system(
  year: i32,
  month: i32,
  day: i32,
  date_system: DateSystem,
) -> Option<f64> {
  if !(1..=12).contains(&month) || day < 1 {
    return None;
  }
  let days = days_from_civil(year, month, day)?;
  let (actual_year, actual_month, actual_day) = civil_from_days(days)?;
  if actual_year != year || actual_month != month as u32 || actual_day != day as u32 {
    return None;
  }
  date_serial_with_system(year, month, day, date_system)
}

fn index_matrix<'doc>(
  rows: Vec<Vec<FormulaValue<'doc>>>,
  row: u32,
  column: u32,
  arg_count: usize,
) -> FormulaValue<'doc> {
  let height = rows.len();
  let width = rows.iter().map(Vec::len).max().unwrap_or(0);
  let b_row_vector_special = arg_count == 2 || column == 0;
  let b_row_vector_element = height == 1 && (column != 0 || (b_row_vector_special && row != 0));
  let b_vector_element = b_row_vector_element || (width == 1 && row != 0);
  if height == 0
    || width == 0
    || (!b_vector_element && (column as usize > width || row as usize > height))
  {
    return FormulaValue::Error(FormulaErrorValue::Ref);
  }
  if row == 0 && column == 0 {
    return FormulaValue::Matrix(rows);
  }
  if b_vector_element {
    let (element, other_dimension) = if b_row_vector_element && !b_row_vector_special {
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

fn push_unique_qualified_range<'doc>(
  ranges: &mut Vec<QualifiedRange<'doc>>,
  range: QualifiedRange<'doc>,
) {
  if !ranges.contains(&range) {
    ranges.push(range);
  }
}

fn is_multicell_scalar_argument(value: &FormulaValue<'_>) -> bool {
  match value {
    FormulaValue::Reference(reference) => reference.range.cell_count_hint() != 1,
    FormulaValue::RefList(_) => true,
    FormulaValue::Matrix(rows) => {
      rows.len() != 1 || rows.first().map_or(true, |row| row.len() != 1)
    }
    _ => false,
  }
}

fn is_matrix_argument(value: &FormulaValue<'_>) -> bool {
  matches!(
    value,
    FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
  )
}

fn cell_in_range(address: CellAddress, range: &CellRange) -> bool {
  let start_column = range.start.column.min(range.end.column);
  let end_column = range.start.column.max(range.end.column);
  let start_row = range.start.row.min(range.end.row);
  let end_row = range.start.row.max(range.end.row);
  (start_column..=end_column).contains(&address.column)
    && (start_row..=end_row).contains(&address.row)
}

fn intersect_qualified_ranges<'doc>(
  left: &QualifiedRange<'doc>,
  right: &QualifiedRange<'doc>,
) -> Option<QualifiedRange<'doc>> {
  let sheet_name = compatible_range_sheet_name(left, right)?;
  if left.sheet != right.sheet {
    return None;
  }
  let left_start_column = left.range.start.column.min(left.range.end.column);
  let left_end_column = left.range.start.column.max(left.range.end.column);
  let left_start_row = left.range.start.row.min(left.range.end.row);
  let left_end_row = left.range.start.row.max(left.range.end.row);
  let right_start_column = right.range.start.column.min(right.range.end.column);
  let right_end_column = right.range.start.column.max(right.range.end.column);
  let right_start_row = right.range.start.row.min(right.range.end.row);
  let right_end_row = right.range.start.row.max(right.range.end.row);

  let start_column = left_start_column.max(right_start_column);
  let end_column = left_end_column.min(right_end_column);
  let start_row = left_start_row.max(right_start_row);
  let end_row = left_end_row.min(right_end_row);
  if start_column > end_column || start_row > end_row {
    return None;
  }
  Some(QualifiedRange {
    sheet: left.sheet,
    sheet_name,
    range: CellRange::new(
      CellAddress {
        column: start_column,
        row: start_row,
      },
      CellAddress {
        column: end_column,
        row: end_row,
      },
    ),
    start_flags: left.start_flags,
    end_flags: left.end_flags,
  })
}

fn bounding_qualified_ranges<'doc>(
  ranges: &[QualifiedRange<'doc>],
) -> Option<QualifiedRange<'doc>> {
  let first = ranges.first()?;
  let mut result = first.clone();
  for range in &ranges[1..] {
    result = extend_qualified_range(&result, range)?;
  }
  Some(result)
}

fn extend_qualified_range<'doc>(
  left: &QualifiedRange<'doc>,
  right: &QualifiedRange<'doc>,
) -> Option<QualifiedRange<'doc>> {
  let sheet_name = compatible_range_sheet_name(left, right)?;
  if left.sheet != right.sheet {
    return None;
  }
  let left_start_column = left.range.start.column.min(left.range.end.column);
  let left_end_column = left.range.start.column.max(left.range.end.column);
  let left_start_row = left.range.start.row.min(left.range.end.row);
  let left_end_row = left.range.start.row.max(left.range.end.row);
  let right_start_column = right.range.start.column.min(right.range.end.column);
  let right_end_column = right.range.start.column.max(right.range.end.column);
  let right_start_row = right.range.start.row.min(right.range.end.row);
  let right_end_row = right.range.start.row.max(right.range.end.row);

  Some(QualifiedRange {
    sheet: left.sheet,
    sheet_name,
    range: CellRange::new(
      CellAddress {
        column: left_start_column.min(right_start_column),
        row: left_start_row.min(right_start_row),
      },
      CellAddress {
        column: left_end_column.max(right_end_column),
        row: left_end_row.max(right_end_row),
      },
    ),
    start_flags: left.start_flags,
    end_flags: right.end_flags,
  })
}

fn compatible_range_sheet_name<'doc>(
  left: &QualifiedRange<'doc>,
  right: &QualifiedRange<'doc>,
) -> Option<Option<SheetName<'doc>>> {
  match (&left.sheet_name, &right.sheet_name) {
    (Some(left), Some(right)) if left != right => None,
    (Some(left), _) => Some(Some(left.clone())),
    (_, Some(right)) => Some(Some(right.clone())),
    (None, None) => Some(None),
  }
}

pub(crate) fn qualified_range<'doc>(
  sheet: SheetId,
  reference: &str,
) -> Option<QualifiedRange<'doc>> {
  QualifiedRange::parse_a1(sheet, reference).ok()
}

fn logical_value(value: &FormulaValue<'_>) -> Option<bool> {
  match value {
    FormulaValue::Boolean(value) => Some(*value),
    FormulaValue::Number(value) => Some(*value != 0.0),
    _ => None,
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

fn sort_multi_key_order<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  keys: &[(Vec<FormulaValue<'doc>>, bool)],
  left: usize,
  right: usize,
) -> std::cmp::Ordering {
  for (values, ascending) in keys {
    let ordering = sort_value_order(evaluator, &values[left], &values[right], *ascending);
    if ordering != std::cmp::Ordering::Equal {
      return ordering;
    }
  }
  left.cmp(&right)
}

fn sort_value_order(
  evaluator: &FormulaEvaluator<'_, '_>,
  left: &FormulaValue<'_>,
  right: &FormulaValue<'_>,
  ascending: bool,
) -> std::cmp::Ordering {
  let ordering = match (left, right) {
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
  };
  if ascending {
    ordering
  } else {
    ordering.reverse()
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
