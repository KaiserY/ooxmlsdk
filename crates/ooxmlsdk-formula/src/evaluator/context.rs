use super::*;
use crate::function::FunctionArgReader;

mod coerce;
mod display;
mod operator;
mod reference;

pub(crate) use display::{
  display_text_from_value, display_text_from_value_with_number_format, error_text,
  error_text_value, error_value, format_number_with_format_code,
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
  pub(crate) calc_a1_indirect_bang_reference: bool,
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
      calc_a1_indirect_bang_reference: self.calc_a1_indirect_bang_reference,
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
      calc_a1_indirect_bang_reference: self.calc_a1_indirect_bang_reference,
    }
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
      return FormulaValue::Error(FormulaErrorValue::Value);
    }
    let parsed = formula_number_from_text(text, self.book.date_system);
    parsed
      .map(FormulaValue::Number)
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::Value))
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

  pub(crate) fn time_number_from_value(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    match self.first_value(value) {
      FormulaValue::String(text) => match timevalue(&text) {
        FormulaValue::Number(value) => Some(value),
        _ => formula_number_from_text(&text, self.book.date_system),
      },
      value => self.number(&value),
    }
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
      end_sheet_name: reference.end_sheet_name.clone(),
      range,
      start_flags: reference.start_flags,
      end_flags: reference.end_flags,
    })
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

  pub(crate) fn evaluate_hlookup_value(
    &self,
    lookup: FormulaValue<'doc>,
    table: &FormulaValue<'doc>,
    result_row: u32,
    sorted: bool,
  ) -> FormulaValue<'doc> {
    let row_index = result_row as usize - 1;
    let matrix = self.matrix_values(table);
    if row_index >= matrix.len() {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    let Some(index) = vhlookup_matrix_index(self, &lookup, &matrix, true, sorted) else {
      return FormulaValue::Error(FormulaErrorValue::NA);
    };
    matrix
      .get(row_index)
      .and_then(|row| row.get(index))
      .cloned()
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::Ref))
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
    let plan = LookupPlan::from_criteria_value(self, lookup);
    let plan = if sorted {
      plan.with_op(QueryOp::LessOrEqual)
    } else {
      plan
    }
    .with_range_lookup(sorted);
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
      if !plan.matches(self, &value, query_empty) {
        if sorted
          && lookup_candidate_type_matches(&plan, &value)
          && lookup_compare_candidate_to_query(self, &value, &plan, true) == Some(1)
        {
          break;
        }
        continue;
      }
      if sorted {
        if lookup_candidate_type_matches(&plan, &value)
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

  pub(crate) fn evaluate_lookup_reader(
    &self,
    args: FunctionArgReader<'_, '_, 'doc>,
  ) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let lookup = self.scalar_binary_operand(args.value(0)?);
    if matches!(lookup, FormulaValue::Blank) {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    }
    let data = args.array_value(1)?;
    let data_matrix = self.matrix_values(&data);
    let result = if args.len() == 3 {
      Some(args.array_value(2)?)
    } else {
      None
    };
    let result_matrix = result.as_ref().map(|value| self.matrix_values(value));
    let (data_vector, data_vertical) = lookup_vector(&data_matrix)?;
    let plan = LookupPlan::new(
      &lookup,
      QueryOp::LessOrEqual,
      QuerySearchType::Normal,
      true,
      true,
      false,
    );
    let index = if matches!(lookup, FormulaValue::String(_)) {
      lookup_initial_exact_run_index(self, &lookup, &data_vector)
    } else {
      None
    };
    let index = if let Some(index) = index {
      index
    } else {
      let (search_vector, index_map) = lookup_search_vector_omitting_errors(&data_vector);
      let search_slice = search_vector.as_deref().unwrap_or(&data_vector);
      let Some(search_index) = lookup_binary_search(self, search_slice, &plan, true, false, false)
      else {
        return Some(FormulaValue::Error(FormulaErrorValue::NA));
      };
      index_map
        .as_ref()
        .and_then(|indices| indices.get(search_index).copied())
        .unwrap_or(search_index)
    };

    if let Some(FormulaValue::Reference(reference)) = result.as_ref() {
      let start_row = reference.range.start.row.min(reference.range.end.row);
      let end_row = reference.range.start.row.max(reference.range.end.row);
      let start_column = reference.range.start.column.min(reference.range.end.column);
      let end_column = reference.range.start.column.max(reference.range.end.column);
      if start_row != end_row && start_column != end_column {
        return Some(FormulaValue::Error(FormulaErrorValue::Error));
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
        if args.is_array_literal(2) && index != 0 {
          return Some(FormulaValue::Error(FormulaErrorValue::NA));
        }
        return result_matrix
          .first()
          .and_then(|row| row.first())
          .cloned()
          .or(Some(FormulaValue::Error(FormulaErrorValue::NA)));
      }
      if rows > 1 && columns > 1 {
        return Some(FormulaValue::Error(FormulaErrorValue::Error));
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

  pub(crate) fn evaluate_match_reader(
    &self,
    args: FunctionArgReader<'_, '_, 'doc>,
  ) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let lookup = self.first_value(&args.value(0)?);
    let data = args.value(1)?;
    let (vector, index_map, data_vertical) = match &data {
      FormulaValue::Reference(reference) => self.lookup_reference_vector(reference)?,
      _ => {
        let matrix = self.matrix_values(&data);
        let (vector, data_vertical) = lookup_vector(&matrix)?;
        (vector, None, data_vertical)
      }
    };
    let mode = args
      .value(2)
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as i32;
    let index = if mode == -1
      || (mode == 1
        && (matches!(lookup, FormulaValue::String(_)) || (index_map.is_some() && !data_vertical)))
    {
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

  pub(crate) fn evaluate_xmatch_reader(
    &self,
    args: FunctionArgReader<'_, '_, 'doc>,
  ) -> Option<FormulaValue<'doc>> {
    if !(2..=4).contains(&args.len()) {
      return None;
    }
    let lookup = self.scalar_value(args.value(0)?);
    let data = args.array_value(1)?;
    let matrix = self.matrix_values(&data);
    let (vector, _) = lookup_vector(&matrix)?;
    let match_mode = args
      .value(2)
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0) as i32;
    let search_mode = args
      .value(3)
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
      0 => search_vector_with_type(
        self,
        &lookup,
        &vector,
        QueryOp::Equal,
        search,
        QuerySearchType::Normal,
        SearchVectorFlags::new(true, false).with_first_exact(),
      ),
      -1 => search_vector_with_type(
        self,
        &lookup,
        &vector,
        QueryOp::Equal,
        search,
        QuerySearchType::Normal,
        SearchVectorFlags::new(true, true).with_first_exact(),
      )
      .or_else(|| {
        search_vector_with_type(
          self,
          &lookup,
          &vector,
          QueryOp::LessOrEqual,
          search,
          QuerySearchType::Normal,
          SearchVectorFlags::new(false, true),
        )
      }),
      1 => search_vector_with_type(
        self,
        &lookup,
        &vector,
        QueryOp::Equal,
        search,
        QuerySearchType::Normal,
        SearchVectorFlags::new(true, true).with_first_exact(),
      )
      .or_else(|| {
        search_vector_with_type(
          self,
          &lookup,
          &vector,
          QueryOp::GreaterOrEqual,
          search,
          QuerySearchType::Normal,
          SearchVectorFlags::new(false, true),
        )
      }),
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
        SearchVectorFlags::new(true, false).with_first_exact(),
      ),
      _ => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    };
    index
      .map(|index| FormulaValue::Number(index as f64 + 1.0))
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  pub(crate) fn evaluate_xlookup_reader(
    &self,
    args: FunctionArgReader<'_, '_, 'doc>,
  ) -> Option<FormulaValue<'doc>> {
    if !(3..=6).contains(&args.len()) {
      return None;
    }
    let lookup = self.scalar_value(args.value(0)?);
    let lookup_array = args.array_value(1)?;
    let return_array = args.value(2)?;
    let matrix = self.matrix_values(&lookup_array);
    let (lookup_rows, lookup_columns) = matrix_dimensions(&matrix);
    if lookup_rows > 1 && lookup_columns > 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let (vector, lookup_vertical) = lookup_vector(&matrix)?;
    if !self.xlookup_return_shape_matches(&return_array, vector.len(), lookup_vertical) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let match_mode = args
      .raw_arg(4)
      .filter(|_| !args.is_missing(4))
      .and_then(|_| args.value(4))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0) as i32;
    let search_mode = args
      .raw_arg(5)
      .filter(|_| !args.is_missing(5))
      .and_then(|_| args.value(5))
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
    let index = if matches!(lookup, FormulaValue::Blank) && match_mode == 0 {
      vector
        .iter()
        .position(|value| matches!(value, FormulaValue::Blank))
    } else {
      match match_mode {
        0 => search_vector_with_type(
          self,
          &lookup,
          &vector,
          QueryOp::Equal,
          search,
          QuerySearchType::Normal,
          SearchVectorFlags::new(true, true).with_first_exact(),
        ),
        -1 => search_vector_with_type(
          self,
          &lookup,
          &vector,
          QueryOp::Equal,
          search,
          QuerySearchType::Normal,
          SearchVectorFlags::new(true, true).with_first_exact(),
        )
        .or_else(|| {
          search_vector_with_type(
            self,
            &lookup,
            &vector,
            QueryOp::LessOrEqual,
            search,
            QuerySearchType::Normal,
            SearchVectorFlags::new(false, true),
          )
        }),
        1 => search_vector_with_type(
          self,
          &lookup,
          &vector,
          QueryOp::Equal,
          search,
          QuerySearchType::Normal,
          SearchVectorFlags::new(true, true).with_first_exact(),
        )
        .or_else(|| {
          search_vector_with_type(
            self,
            &lookup,
            &vector,
            QueryOp::GreaterOrEqual,
            search,
            QuerySearchType::Normal,
            SearchVectorFlags::new(false, true),
          )
        }),
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
          SearchVectorFlags::new(true, true).with_first_exact(),
        ),
        _ => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
      }
    };
    let Some(index) = index else {
      let value = args
        .raw_arg(3)
        .filter(|_| !args.is_missing(3))
        .and_then(|_| args.value(3))
        .unwrap_or(FormulaValue::Error(FormulaErrorValue::NA));
      return Some(self.xlookup_not_found_value(value, &return_array, lookup_vertical));
    };
    Some(self.xlookup_return_value(return_array, index, lookup_vertical))
  }

  fn xlookup_not_found_value(
    &self,
    value: FormulaValue<'doc>,
    return_array: &FormulaValue<'doc>,
    lookup_vertical: bool,
  ) -> FormulaValue<'doc> {
    if !self.array_context || matches!(value, FormulaValue::Error(_)) {
      return value;
    }
    let (rows, columns) = xlookup_result_dimensions(self, return_array, lookup_vertical);
    if rows <= 1 && columns <= 1 {
      return value;
    }
    if matches!(
      self.grammar,
      FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1
    ) {
      let mut matrix =
        vec![vec![FormulaValue::String(Cow::Borrowed("")); columns.max(1)]; rows.max(1)];
      matrix[0][0] = value;
      FormulaValue::Matrix(matrix)
    } else {
      FormulaValue::Matrix(vec![vec![value; columns.max(1)]; rows.max(1)])
    }
  }

  fn xlookup_return_value(
    &self,
    return_array: FormulaValue<'doc>,
    index: usize,
    lookup_vertical: bool,
  ) -> FormulaValue<'doc> {
    if let FormulaValue::Reference(reference) = &return_array {
      let start_row = reference.range.start.row.min(reference.range.end.row);
      let end_row = reference.range.start.row.max(reference.range.end.row);
      let start_column = reference.range.start.column.min(reference.range.end.column);
      let end_column = reference.range.start.column.max(reference.range.end.column);
      let range = if lookup_vertical {
        let row = start_row.saturating_add(index as u32);
        if row > end_row {
          return FormulaValue::Error(FormulaErrorValue::NA);
        }
        CellRange::new(
          CellAddress {
            column: start_column,
            row,
          },
          CellAddress {
            column: end_column,
            row,
          },
        )
      } else {
        let column = start_column.saturating_add(index as u32);
        if column > end_column {
          return FormulaValue::Error(FormulaErrorValue::NA);
        }
        CellRange::new(
          CellAddress {
            column,
            row: start_row,
          },
          CellAddress {
            column,
            row: end_row,
          },
        )
      };
      let reference = QualifiedRange {
        sheet: reference.sheet,
        sheet_name: reference.sheet_name.clone(),
        end_sheet_name: reference.end_sheet_name.clone(),
        range,
        start_flags: reference.start_flags,
        end_flags: reference.end_flags,
      };
      if self.array_context && reference.range.cell_count_hint() > 1 {
        return FormulaValue::Matrix(self.matrix_values(&FormulaValue::Reference(reference)));
      }
      return FormulaValue::Reference(reference);
    }
    let matrix = self.matrix_values(&return_array);
    let Some(vector) = lookup_vector_with_orientation(&matrix, lookup_vertical) else {
      return FormulaValue::Error(FormulaErrorValue::NA);
    };
    vector
      .get(index)
      .cloned()
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::NA))
  }

  fn xlookup_return_shape_matches(
    &self,
    return_array: &FormulaValue<'doc>,
    lookup_len: usize,
    lookup_vertical: bool,
  ) -> bool {
    match return_array {
      FormulaValue::Reference(reference) => {
        let rows = reference.range.start.row.abs_diff(reference.range.end.row) as usize + 1;
        let columns = reference
          .range
          .start
          .column
          .abs_diff(reference.range.end.column) as usize
          + 1;
        if lookup_vertical {
          rows == lookup_len
        } else {
          columns == lookup_len
        }
      }
      value => {
        let matrix = self.matrix_values(value);
        let (rows, columns) = matrix_dimensions(&matrix);
        if lookup_vertical {
          rows == lookup_len
        } else {
          columns == lookup_len
        }
      }
    }
  }

  pub(crate) fn evaluate_sheets_reader(
    &self,
    args: FunctionArgReader<'_, '_, 'doc>,
  ) -> Option<FormulaValue<'doc>> {
    if args.is_empty() {
      return Some(FormulaValue::Number(
        self.book.sheet_names.len().max(1) as f64
      ));
    }
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Error));
    }
    if let Some(count) = self.raw_sheets_reference_count(args) {
      return Some(FormulaValue::Number(count as f64));
    }
    match args.value(0)? {
      FormulaValue::Reference(reference) => Some(FormulaValue::Number(
        self.reference_sheet_count(&reference) as f64,
      )),
      FormulaValue::RefList(ranges) => Some(FormulaValue::Number(
        ranges
          .iter()
          .map(|range| self.reference_sheet_count(range))
          .sum::<u32>() as f64,
      )),
      FormulaValue::Matrix(_) => Some(FormulaValue::Error(FormulaErrorValue::Error)),
      _ => Some(FormulaValue::Error(FormulaErrorValue::Value)),
    }
  }

  fn raw_sheets_reference_count(&self, args: FunctionArgReader<'_, '_, 'doc>) -> Option<u32> {
    let arg = args.raw_arg(0)?;
    let ops = &arg.ops[arg.range.start..arg.range.end];
    match ops {
      [FormulaOp::PushReference(reference)] => Some(self.reference_sheet_count(reference)),
      [
        FormulaOp::PushReference(left),
        FormulaOp::PushReference(right),
        FormulaOp::Binary(FormulaOperator::Range),
      ] => {
        let left_sheet = self.range_sheet(left).0;
        let right_sheet = self.range_sheet(right).0;
        Some(left_sheet.abs_diff(right_sheet) + 1)
      }
      _ => None,
    }
  }

  pub(crate) fn evaluate_sheet_reader(
    &self,
    args: FunctionArgReader<'_, '_, 'doc>,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() > 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Error));
    }
    let sheet = if args.is_empty() {
      self.current_sheet
    } else {
      match args.value(0)? {
        FormulaValue::Reference(reference) => self.range_sheet(&reference),
        FormulaValue::String(name) => {
          let Some(sheet) = self.book.sheet_id_by_name(name.as_ref()) else {
            return Some(FormulaValue::Error(FormulaErrorValue::Value));
          };
          sheet
        }
        _ => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
      }
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

  pub(crate) fn reference_sheet_count(&self, reference: &QualifiedRange<'doc>) -> u32 {
    let start = self.range_sheet(reference).0;
    let end = reference
      .end_sheet_name
      .as_ref()
      .or(reference.sheet_name.as_ref())
      .and_then(|name| self.book.sheet_id_by_name(name.0.as_ref()))
      .map(|sheet| sheet.0)
      .unwrap_or(start);
    start.abs_diff(end) + 1
  }

  pub(crate) fn left_value(
    &self,
    value: &FormulaValue<'doc>,
    len: &FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let len = self.number(len)?;
    if len < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let len = len.floor() as usize;
    let text = self.text(value);
    Some(FormulaValue::String(Cow::Owned(
      text.chars().take(len).collect(),
    )))
  }

  pub(crate) fn leftb_value(
    &self,
    value: &FormulaValue<'doc>,
    len: &FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let len = self.number(len)?;
    if len < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let text = self.text(value);
    Some(FormulaValue::String(Cow::Owned(leftb(
      &text,
      len.floor() as usize,
    ))))
  }

  pub(crate) fn right_value(
    &self,
    value: &FormulaValue<'doc>,
    len: &FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let len = self.number(len)?;
    if len < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
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

  pub(crate) fn rightb_value(
    &self,
    value: &FormulaValue<'doc>,
    len: &FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let len = self.number(len)?;
    if len < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let text = self.text(value);
    Some(FormulaValue::String(Cow::Owned(rightb(
      &text,
      len.floor() as usize,
    ))))
  }

  pub(crate) fn evaluate_ifs_reader(
    &self,
    args: FunctionArgReader<'_, '_, 'doc>,
    main_range: Option<usize>,
    resize_main_range: bool,
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
      let range = args.query_source(range_index)?;
      let (rows, columns) = range.dimensions();
      if rows == 0 || columns == 0 || !range.is_rectangular() {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      let criteria_value = args.value(range_index + 1)?;
      let criteria_matrix = if self.array_context {
        self.matrix_values(&criteria_value)
      } else {
        vec![vec![self.scalar_binary_operand(criteria_value)]]
      };
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
          .map(|value| CriteriaPlan::from_criterion(self, &value))
          .collect::<Vec<_>>(),
      );
    }

    if criteria_ranges
      .windows(2)
      .any(|window| window[0].dimensions() != window[1].dimensions())
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    if main_range.is_some()
      && resize_main_range
      && criteria_ranges.iter().any(QueryValueSource::is_ref_list)
      && matches!(
        self.grammar,
        FormulaGrammar::CalcA1 | FormulaGrammar::OpenFormula
      )
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Error));
    }
    let dimensions = criteria_ranges.first()?.dimensions();
    let main_values = if let Some(main_range) = main_range {
      let values = args.query_source(main_range)?;
      let values = if values.dimensions() != dimensions {
        if resize_main_range {
          match values.resized_from_top_left(dimensions.0, dimensions.1) {
            Some(values) => values,
            None => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
          }
        } else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        }
      } else {
        values
      };
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
                range
                  .value_at(self, row, column)
                  .is_some_and(|cell| criteria.matches(self, &cell.value, cell.query_empty))
              });
          if !matches_all {
            continue;
          }
          match aggregate {
            IfsAggregate::Count => count += 1.0,
            IfsAggregate::Sum | IfsAggregate::Average | IfsAggregate::Min | IfsAggregate::Max => {
              let value = main_values
                .as_ref()
                .and_then(|values| values.value_at(self, row, column))
                .map(|cell| cell.value);
              if let Some(FormulaValue::Error(error)) = value {
                return Some(FormulaValue::Error(error));
              }
              if let Some(number) = value.as_ref().and_then(formula_cell_numeric_value) {
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

  pub(crate) fn evaluate_networkdays_reader(
    &self,
    args: FunctionArgReader<'_, '_, 'doc>,
    intl: bool,
  ) -> Option<FormulaValue<'doc>> {
    let Some(mut start) = self
      .number(&args.value(0)?)
      .map(|value| value.floor() as i64)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let Some(mut end) = self
      .number(&args.value(1)?)
      .map(|value| value.floor() as i64)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let weekend_arg = if intl { args.value(2) } else { args.value(3) };
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
    let holiday_arg = args.value(if intl { 3 } else { 2 });
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

  pub(crate) fn evaluate_workday_reader(
    &self,
    args: FunctionArgReader<'_, '_, 'doc>,
    intl: bool,
  ) -> Option<FormulaValue<'doc>> {
    let Some(mut date) = args
      .value(0)
      .and_then(|value| self.number(&value))
      .map(|value| value.floor() as i64)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let Some(mut days) = args
      .value(1)
      .and_then(|value| self.number(&value))
      .map(|value| value.floor() as i64)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let weekend_arg = if intl {
      if args.raw_arg(2).is_none() || args.is_missing(2) {
        None
      } else {
        match args.value(2)? {
          FormulaValue::Reference(reference) => {
            if reference.range.cell_count_hint() != 1 {
              return Some(FormulaValue::Error(FormulaErrorValue::Value));
            }
            Some(self.scalar_reference_value(&reference))
          }
          FormulaValue::RefList(_) => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
          value => Some(value),
        }
      }
    } else {
      None
    };
    let Some(weekend) = weekend_mask(weekend_arg.as_ref(), true, self) else {
      return Some(FormulaValue::Error(if intl {
        FormulaErrorValue::Num
      } else {
        FormulaErrorValue::Value
      }));
    };
    let holiday_arg = args.value(if intl { 3 } else { 2 });
    let holidays = holiday_serials(holiday_arg.as_ref(), self);
    if days == 0 {
      return Some(FormulaValue::Number(date as f64));
    }
    let step = if days > 0 { 1 } else { -1 };
    while days != 0 {
      date += step;
      if weekend[weekday_index_from_serial(date)] || holidays.binary_search(&date).is_ok() {
        continue;
      }
      days -= step;
    }
    Some(FormulaValue::Number(date as f64))
  }

  pub(crate) fn evaluate_subtotal_reader(
    &self,
    args: FunctionArgReader<'_, '_, 'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let Some(function) = args
      .value(0)
      .and_then(|value| self.number(&value))
      .map(|value| value as i32)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Error));
    };
    let function_id = function.rem_euclid(100);
    if !(1..=11).contains(&function_id) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut values = Vec::with_capacity(args.len().saturating_sub(1));
    for index in 1..args.len() {
      values.push(args.value(index)?);
    }
    let options = AggregateOptions {
      ignore_hidden: function >= 100,
      ignore_filtered: true,
      ignore_errors: false,
      ignore_nested: true,
    };
    if let [FormulaValue::RefList(ranges)] = values.as_slice() {
      let mut rows = Vec::with_capacity(ranges.len());
      for range in ranges {
        let value = aggregate_function_value(
          self,
          function_id,
          &[FormulaValue::Reference(range.clone())],
          None,
          options,
        )
        .map(|result| match result {
          Ok(value) => FormulaValue::Number(value),
          Err(error) => FormulaValue::Error(error),
        })?;
        rows.push(vec![value]);
      }
      return Some(FormulaValue::Matrix(rows));
    }
    aggregate_function_value(self, function_id, &values, None, options).map(|result| match result {
      Ok(value) => FormulaValue::Number(value),
      Err(error) => FormulaValue::Error(error),
    })
  }

  pub(crate) fn evaluate_aggregate_reader(
    &self,
    args: FunctionArgReader<'_, '_, 'doc>,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Error));
    }
    let Some(function) = args
      .value(0)
      .and_then(|value| self.number(&value))
      .map(|value| value as i32)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Error));
    };
    let Some(options_arg) = args
      .value(1)
      .and_then(|value| self.number(&value))
      .map(|value| value as i32)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Error));
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
    for index in 2..args.len() {
      evaluated.push(args.value(index)?);
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

  pub(crate) fn evaluate_database_function_reader(
    &self,
    args: FunctionArgReader<'_, '_, 'doc>,
    function: DatabaseFunction,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let Some(database) = args.query_source(0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(criteria) = args.query_source(2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let (database_rows, database_columns) = database.dimensions();
    let (criteria_rows, criteria_columns) = criteria.dimensions();
    if database_rows < 2 || database_columns == 0 || criteria_rows < 2 || criteria_columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let Some(headers) = database.header_row(self) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let field = match self.database_field_index_value(args.value(1)?, &headers, function) {
      Some(field) => field,
      None => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    };
    let plan = self.database_criteria_plan(&headers, &criteria);
    let matching_rows = (1..database_rows)
      .filter(|row| plan.matches_row(self, &database, *row))
      .collect::<Vec<_>>();
    if field.is_none() && matches!(function, DatabaseFunction::Count | DatabaseFunction::CountA) {
      return Some(FormulaValue::Number(matching_rows.len() as f64));
    }
    let Some(field) = field else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    if field >= database_columns {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }

    let mut values = Vec::new();
    let mut text_values = Vec::new();
    for row in matching_rows {
      let value = database
        .value_at(self, row, field)
        .map(|cell| cell.value)
        .unwrap_or_default();
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

  pub(crate) fn database_field_index_value(
    &self,
    value: FormulaValue<'doc>,
    headers: &[FormulaValue<'doc>],
    function: DatabaseFunction,
  ) -> Option<Option<usize>> {
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

  pub(crate) fn database_criteria_plan(
    &self,
    headers: &[FormulaValue<'doc>],
    criteria: &QueryValueSource<'doc>,
  ) -> DatabaseCriteriaPlan<'doc> {
    if let Some(plan) = self.database_star_criteria_plan(headers, criteria)
      && !plan.is_empty()
    {
      return plan;
    }
    let (rows, columns) = criteria.dimensions();
    let mut plan = DatabaseCriteriaPlan::default();
    for row in 1..rows {
      let mut group = Vec::new();
      let mut invalid = false;
      let row_has_present_cell = (0..columns).any(|column| {
        criteria
          .value_at(self, row, column)
          .is_some_and(|cell| database_criterion_cell_present(&cell.value, cell.query_empty))
      });
      for criteria_column in 0..columns {
        let Some(criterion_cell) = criteria.value_at(self, row, criteria_column) else {
          continue;
        };
        if !database_criterion_entry_present(&criterion_cell.value, criterion_cell.query_empty) {
          continue;
        }
        let Some(header_cell) = criteria.value_at(self, 0, criteria_column) else {
          continue;
        };
        let header = self.text(&header_cell.value);
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
        group.push(FieldCriteriaPlan::new(
          field,
          CriteriaPlan::from_database_value(self, &criterion_cell.value),
        ));
      }
      if invalid {
        continue;
      }
      if !group.is_empty() || row_has_present_cell {
        plan.push_group(group);
      }
    }
    plan
  }

  pub(crate) fn database_star_criteria_plan(
    &self,
    headers: &[FormulaValue<'doc>],
    criteria: &QueryValueSource<'doc>,
  ) -> Option<DatabaseCriteriaPlan<'doc>> {
    let (rows, columns) = criteria.dimensions();
    if columns < 4 {
      return None;
    }
    if !(0..rows).any(|row| {
      let connector = criteria
        .value_at(self, row, 0)
        .map(|cell| self.text(&cell.value))
        .unwrap_or_default();
      connector.eq_ignore_ascii_case("AND") || connector.eq_ignore_ascii_case("OR")
    }) {
      return None;
    }
    let mut plan = DatabaseCriteriaPlan::default();
    let mut current = Vec::new();
    for row_index in 0..rows {
      let connector = criteria
        .value_at(self, row_index, 0)
        .map(|cell| self.text(&cell.value))
        .unwrap_or_default();
      if row_index > 0 && connector.eq_ignore_ascii_case("OR") {
        if !current.is_empty() {
          plan.push_group(std::mem::take(&mut current));
        }
      } else if row_index > 0 && !connector.is_empty() && !connector.eq_ignore_ascii_case("AND") {
        return None;
      }
      let field_name = criteria
        .value_at(self, row_index, 1)
        .map(|cell| self.text(&cell.value))
        .unwrap_or_default();
      if field_name.is_empty() {
        return None;
      }
      let field = headers
        .iter()
        .position(|header| self.text(header).eq_ignore_ascii_case(&field_name))?;
      let op_text = criteria
        .value_at(self, row_index, 2)
        .map(|cell| self.text(&cell.value))
        .unwrap_or_default();
      let op = match op_text.trim() {
        "" | "=" => QueryOp::Equal,
        "<>" => QueryOp::NotEqual,
        "<" => QueryOp::Less,
        "<=" => QueryOp::LessOrEqual,
        ">" => QueryOp::Greater,
        ">=" => QueryOp::GreaterOrEqual,
        _ => return None,
      };
      let criterion = criteria
        .value_at(self, row_index, 3)
        .map(|cell| cell.value)
        .unwrap_or_default();
      current.push(FieldCriteriaPlan::new(
        field,
        CriteriaPlan::from_database_value(self, &criterion).with_op(op),
      ));
    }
    if !current.is_empty() {
      plan.push_group(current);
    }
    Some(plan)
  }

  pub(crate) fn evaluate_cum_interest_principal_reader(
    &self,
    args: FunctionArgReader<'_, '_, 'doc>,
    interest: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 6 {
      return None;
    }
    if (0..args.len()).any(|index| args.is_missing(index)) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let rate = self.number(&args.value(0)?)?;
    let nper = self.number(&args.value(1)?)?;
    let pv = self.number(&args.value(2)?)?;
    let start = approx_floor(self.number(&args.value(3)?)?);
    let end = approx_floor(self.number(&args.value(4)?)?);
    let flag = self.number(&args.value(5)?)?;
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

  pub(crate) fn evaluate_euroconvert_reader(
    &self,
    args: FunctionArgReader<'_, '_, 'doc>,
  ) -> Option<FormulaValue<'doc>> {
    if !(3..=5).contains(&args.len()) {
      return None;
    }
    if args.is_missing(4) || (args.len() == 4 && args.is_missing(3)) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let value = self.number(&args.value(0)?)?;
    let from = self.text(&args.value(1)?);
    let to = self.text(&args.value(2)?);
    let full_precision = args
      .raw_arg(3)
      .and_then(|_| args.value(3))
      .is_some_and(|value| self.truthy(&value));
    let precision = args
      .raw_arg(4)
      .and_then(|_| args.value(4))
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

  pub(crate) fn evaluate_forecast_reader(
    &self,
    args: FunctionArgReader<'_, '_, 'doc>,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let x = self.number(&args.value(0)?)?;
    let y_value = args.value(1)?;
    let x_value = args.value(2)?;
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

  pub(crate) fn evaluate_forecast_ets_reader(
    &self,
    args: FunctionArgReader<'_, '_, 'doc>,
    kind: EtsKind,
  ) -> Option<FormulaValue<'doc>> {
    let valid_count = match kind {
      EtsKind::Add | EtsKind::Mult | EtsKind::StatAdd | EtsKind::StatMult => {
        (3..=6).contains(&args.len())
      }
      EtsKind::PiAdd | EtsKind::PiMult => (3..=7).contains(&args.len()),
      EtsKind::Season => (2..=4).contains(&args.len()),
    };
    if !valid_count {
      return Some(FormulaValue::Error(FormulaErrorValue::Error));
    }
    let aggregation_index = match kind {
      EtsKind::Season => 3,
      EtsKind::PiAdd | EtsKind::PiMult => 6,
      _ => 5,
    };
    let data_completion_index = match kind {
      EtsKind::Season => 2,
      EtsKind::PiAdd | EtsKind::PiMult => 5,
      _ => 4,
    };
    let seasonality_index = match kind {
      EtsKind::PiAdd | EtsKind::PiMult => 4,
      _ => 3,
    };
    let aggregation = args
      .value(aggregation_index)
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0)
      .floor() as i32;
    if !(1..=7).contains(&aggregation) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let data_completion = args
      .value(data_completion_index)
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0);
    if data_completion != 0.0 && data_completion != 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let samples_in_period = if kind == EtsKind::Season {
      1
    } else {
      let value = args
        .value(seasonality_index)
        .and_then(|value| self.number(&value))
        .unwrap_or(1.0);
      if value < 0.0 || value.fract() != 0.0 {
        return Some(FormulaValue::Error(FormulaErrorValue::Num));
      }
      value as usize
    };

    let type_matrix = if matches!(kind, EtsKind::StatAdd | EtsKind::StatMult) {
      let matrix = self.matrix_values(&args.value(2)?);
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
    let (target_arg, values_index, timeline_index) = match kind {
      EtsKind::Season => (None, 0, 1),
      EtsKind::StatAdd | EtsKind::StatMult => (None, 0, 1),
      EtsKind::Add | EtsKind::Mult | EtsKind::PiAdd | EtsKind::PiMult => (Some(0), 1, 2),
    };
    let values = self.value_numbers(&args.value(values_index)?);
    let timeline = self.value_numbers(&args.value(timeline_index)?);
    if values.len() != timeline.len() || values.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let target_value = target_arg.and_then(|index| args.value(index));
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
      self.book.date_system,
    ) {
      Ok(calc) => calc,
      Err(error) => return Some(FormulaValue::Error(ets_error_value(error))),
    };
    match kind {
      EtsKind::Season => Some(FormulaValue::Number(calc.samples_in_period as f64)),
      EtsKind::StatAdd | EtsKind::StatMult => {
        let matrix = type_matrix?;
        let result = matrix
          .into_iter()
          .map(|row| {
            row
              .into_iter()
              .map(|value| {
                let index = self.number(&value).unwrap_or(0.0).floor() as i32;
                FormulaValue::Number(calc.statistic(index))
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
      EtsKind::PiAdd | EtsKind::PiMult => {
        let level = args
          .value(3)
          .and_then(|value| self.number(&value))
          .unwrap_or(0.95);
        if !(0.0..=1.0).contains(&level) {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        }
        let matrix = target_matrix?;
        let result = matrix
          .iter()
          .map(|row| {
            row
              .iter()
              .map(|value| {
                let Some(target) = self.number(value) else {
                  return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
                };
                match calc.prediction_interval(target, level) {
                  Ok(interval) => FormulaValue::Number(interval),
                  Err(error) => FormulaValue::Error(ets_error_value(error)),
                }
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

  pub(crate) fn evaluate_fourier_reader(
    &self,
    args: FunctionArgReader<'_, '_, 'doc>,
  ) -> Option<FormulaValue<'doc>> {
    if !(2..=5).contains(&args.len()) {
      return None;
    }
    let input = self.matrix_values(&args.value(0)?);
    if input.is_empty() || input.first()?.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let grouped_by_column = self.truthy(&args.value(1)?);
    let inverse = args.value(2).is_some_and(|value| self.truthy(&value));
    let polar = args.value(3).is_some_and(|value| self.truthy(&value));
    let min_magnitude = args
      .value(4)
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);

    let row_count = input.len();
    let column_count = input.first()?.len();
    if input.iter().any(|row| row.len() != column_count)
      || (grouped_by_column && column_count > 2)
      || (!grouped_by_column && row_count > 2)
    {
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
    } else if real_input {
      for real in &input[0] {
        let Some(real) = self.number(real) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        values.push(Complex::new(real, 0.0));
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
    let Some(left) = self.number(&left) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let Some(right) = self.number(&right) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let result = op(left, right);
    if result.is_finite() {
      Some(FormulaValue::Number(normalize_formula_number(result)))
    } else {
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    }
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
          let value = op(left, right);
          if value.is_finite() {
            FormulaValue::Number(normalize_formula_number(value))
          } else {
            FormulaValue::Error(FormulaErrorValue::Num)
          }
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

  pub(crate) fn count_numbers_in_range(&self, reference: &QualifiedRange<'doc>) -> usize {
    self
      .range_values(reference)
      .iter()
      .filter(|value| matches!(value, FormulaValue::Number(_) | FormulaValue::Boolean(_)))
      .count()
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
        FormulaValue::Error(error) => return Err(error),
        FormulaValue::Boolean(_) | FormulaValue::String(_) | FormulaValue::Blank => {}
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

  pub(crate) fn query_source_from_value(
    &self,
    value: FormulaValue<'doc>,
  ) -> Option<QueryValueSource<'doc>> {
    QueryValueSource::from_value(self, value)
  }

  pub(crate) fn count_blank(&self, value: &FormulaValue<'doc>) -> usize {
    match value {
      FormulaValue::Reference(reference) => {
        if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
          return 0;
        }
        let sheet = self.range_sheet(reference);
        let mut count = 0usize;
        for row in reference.range.start.row..=reference.range.end.row {
          for column in reference.range.start.column..=reference.range.end.column {
            let address = CellAddress { column, row };
            let value = self.book.cell_value(sheet, address);
            let formula = self.book.formula_text(sheet, address).is_some();
            let is_blank = match value {
              FormulaValue::Blank => !formula,
              FormulaValue::String(ref text) => text.is_empty(),
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
  let op = match mode {
    1 => QueryOp::LessOrEqual,
    -1 => QueryOp::GreaterOrEqual,
    _ => return None,
  };
  if mode == 1
    && let FormulaValue::String(pattern) = lookup
    && detect_query_search_type(evaluator.book.formula_search_type, pattern)
      == QuerySearchType::Wildcard
  {
    return vector
      .iter()
      .enumerate()
      .rev()
      .find_map(|(index, candidate)| {
        let FormulaValue::String(text) = candidate else {
          return None;
        };
        wildcard_match(pattern, text).then_some(index)
      });
  }
  let plan = LookupPlan::new(lookup, op, QuerySearchType::Normal, true, true, false);
  let mut found = None;
  let mut first_string_ignore = matches!(lookup, FormulaValue::String(_));
  for (index, candidate) in vector.iter().enumerate() {
    let exact = lookup_types_compatible(evaluator, lookup, candidate)
      && evaluator.compare(candidate, lookup, FormulaOperator::Equal);
    let valid = plan.matches(evaluator, candidate, false);
    if valid {
      found = Some(index);
      if exact {
        if mode == -1
          && matches!(
            evaluator.grammar,
            FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1
          )
        {
          return Some(index);
        }
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
  found.filter(|index| lookup_candidate_type_matches(&plan, &vector[*index]))
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
  let plan = LookupPlan::new(
    lookup,
    op,
    search_type,
    true,
    range_lookup,
    flags.match_empty,
  );
  match mode {
    LookupSearchMode::BinaryAscending | LookupSearchMode::BinaryDescending => {
      if search_type != QuerySearchType::Normal {
        return None;
      }
      let sorted_ascending = matches!(mode, LookupSearchMode::BinaryAscending);
      lookup_binary_search(
        evaluator,
        vector,
        &plan,
        sorted_ascending,
        true,
        flags.first_exact,
      )
      .or_else(|| range_lookup.then(|| lookup_best_range_match(evaluator, vector, &plan))?)
    }
    LookupSearchMode::Forward => {
      let mut found = None;
      for (index, candidate) in vector.iter().enumerate() {
        if flags.exact_type && !lookup_types_compatible(evaluator, lookup, candidate) {
          continue;
        }
        if plan.matches(evaluator, candidate, false) {
          match op {
            QueryOp::Equal => {
              found = Some(index);
              break;
            }
            QueryOp::LessOrEqual => {
              if lookup_candidate_type_matches(&plan, candidate)
                && found.is_none_or(|found_index| {
                  lookup_compare_cells(evaluator, candidate, &vector[found_index]) >= 0
                })
              {
                found = Some(index);
              }
            }
            QueryOp::GreaterOrEqual => {
              if lookup_candidate_type_matches(&plan, candidate)
                && found.is_none_or(|found_index| {
                  lookup_compare_cells(evaluator, candidate, &vector[found_index]) <= 0
                })
              {
                found = Some(index);
              }
            }
            _ => {
              if lookup_candidate_type_matches(&plan, candidate) {
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
        if plan.matches(evaluator, candidate, false) {
          match op {
            QueryOp::Equal => {
              found = Some(index);
              break;
            }
            QueryOp::LessOrEqual => {
              if lookup_candidate_type_matches(&plan, candidate)
                && found.is_none_or(|found_index| {
                  lookup_compare_cells(evaluator, candidate, &vector[found_index]) > 0
                })
              {
                found = Some(index);
              }
            }
            QueryOp::GreaterOrEqual => {
              if lookup_candidate_type_matches(&plan, candidate)
                && found.is_none_or(|found_index| {
                  lookup_compare_cells(evaluator, candidate, &vector[found_index]) < 0
                })
              {
                found = Some(index);
              }
            }
            _ => {
              if lookup_candidate_type_matches(&plan, candidate) {
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

fn lookup_best_range_match<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  vector: &[FormulaValue<'doc>],
  plan: &LookupPlan<'doc>,
) -> Option<usize> {
  let mut found = None;
  for (index, candidate) in vector.iter().enumerate() {
    if !lookup_candidate_type_matches(plan, candidate) || !plan.matches(evaluator, candidate, false)
    {
      continue;
    }
    let replace = match (plan.op(), found) {
      (QueryOp::LessOrEqual, Some(found_index)) => {
        lookup_compare_cells(evaluator, candidate, &vector[found_index]) > 0
      }
      (QueryOp::GreaterOrEqual, Some(found_index)) => {
        lookup_compare_cells(evaluator, candidate, &vector[found_index]) < 0
      }
      (_, Some(_)) => false,
      (_, None) => true,
    };
    if replace {
      found = Some(index);
    }
  }
  found
}

fn lookup_binary_search<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  vector: &[FormulaValue<'doc>],
  plan: &LookupPlan<'doc>,
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
    let cmp = lookup_compare_candidate_to_query(evaluator, &vector[mid], plan, empty_is_less)?;
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
    } else if cmp > 0 {
      low = mid + 1;
    } else {
      high = mid;
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
    return lookup_binary_result_index(vector, plan, index);
  }
  if plan.op() == QueryOp::Equal {
    return None;
  }
  let index = match (sorted_ascending, plan.op()) {
    (true, QueryOp::LessOrEqual) => low.checked_sub(1),
    (true, QueryOp::GreaterOrEqual) => (low < vector.len()).then_some(low),
    (false, QueryOp::LessOrEqual) => (low < vector.len()).then_some(low),
    (false, QueryOp::GreaterOrEqual) => low.checked_sub(1),
    _ => None,
  }?;
  lookup_binary_result_index(vector, plan, index)
}

fn lookup_binary_result_index<'doc>(
  vector: &[FormulaValue<'doc>],
  plan: &LookupPlan<'doc>,
  index: usize,
) -> Option<usize> {
  (lookup_candidate_type_matches(plan, vector.get(index)?)).then_some(index)
}

fn lookup_candidate_type_matches(plan: &LookupPlan<'_>, candidate: &FormulaValue<'_>) -> bool {
  plan.candidate_type_matches(candidate)
}

fn lookup_compare_candidate_to_query<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  candidate: &FormulaValue<'doc>,
  plan: &LookupPlan<'doc>,
  empty_is_less: bool,
) -> Option<i32> {
  plan.compare_candidate(evaluator, candidate, empty_is_less)
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

fn matrix_can_broadcast(
  rows: usize,
  columns: usize,
  target_rows: usize,
  target_columns: usize,
) -> bool {
  (rows == target_rows || rows == 1) && (columns == target_columns || columns == 1)
}

fn matrix_binary_extent(left: usize, right: usize) -> usize {
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

pub(crate) fn matrix_dimensions<T>(matrix: &[Vec<T>]) -> (usize, usize) {
  (matrix.len(), matrix.first().map_or(0, Vec::len))
}

fn xlookup_result_dimensions<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  value: &FormulaValue<'doc>,
  lookup_vertical: bool,
) -> (usize, usize) {
  let (rows, columns) = match value {
    FormulaValue::Reference(reference) => (
      reference.range.start.row.abs_diff(reference.range.end.row) as usize + 1,
      reference
        .range
        .start
        .column
        .abs_diff(reference.range.end.column) as usize
        + 1,
    ),
    value => matrix_dimensions(&evaluator.matrix_values(value)),
  };
  if lookup_vertical {
    (1, columns)
  } else {
    (rows, 1)
  }
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

fn formula_cell_numeric_value(value: &FormulaValue<'_>) -> Option<f64> {
  match value {
    FormulaValue::Number(value) => Some(*value),
    _ => None,
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

fn arithmetic_operator_matrix_number(value: &FormulaValue<'_>) -> Option<f64> {
  match value {
    FormulaValue::Number(value) => Some(*value),
    FormulaValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
    FormulaValue::Blank => Some(0.0),
    _ => None,
  }
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

fn lookup_initial_exact_run_index<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  lookup: &FormulaValue<'doc>,
  vector: &[FormulaValue<'doc>],
) -> Option<usize> {
  let first = vector.first()?;
  if !lookup_types_compatible(evaluator, lookup, first)
    || !evaluator.compare(first, lookup, FormulaOperator::Equal)
  {
    return None;
  }
  let mut last_equal = 0;
  for (index, candidate) in vector.iter().enumerate().skip(1) {
    if lookup_types_compatible(evaluator, lookup, candidate)
      && evaluator.compare(candidate, lookup, FormulaOperator::Equal)
    {
      last_equal = index;
    } else {
      break;
    }
  }
  Some(last_equal)
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
      if matches!(lookup, FormulaValue::String(_))
        && !vhlookup_string_vector_is_sorted(evaluator, matrix, horizontal, count)
      {
        return vhlookup_excel_unsorted_text_index(
          evaluator,
          &lookup_text,
          matrix,
          horizontal,
          count,
        );
      }
      for candidate_index in 0..count {
        let candidate = vhlookup_search_cell(matrix, horizontal, candidate_index);
        if candidate.is_none_or(is_lookup_string_or_empty) {
          let candidate_text = candidate
            .map(|value| evaluator.text(value))
            .unwrap_or_default();
          let ordering = compare_text(evaluator, &candidate_text, &lookup_text, false);
          if ordering <= 0 {
            index = Some(candidate_index);
            if ordering == 0 {
              break;
            }
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

fn vhlookup_string_vector_is_sorted<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  matrix: &[Vec<FormulaValue<'doc>>],
  horizontal: bool,
  count: usize,
) -> bool {
  let mut previous: Option<String> = None;
  for candidate_index in 0..count {
    let candidate = vhlookup_search_cell(matrix, horizontal, candidate_index);
    let Some(candidate) = candidate.filter(|value| is_lookup_string_or_empty(value)) else {
      continue;
    };
    let text = evaluator.text(candidate);
    if previous
      .as_ref()
      .is_some_and(|previous| compare_text(evaluator, previous, &text, false) > 0)
    {
      return false;
    }
    previous = Some(text);
  }
  true
}

fn vhlookup_excel_unsorted_text_index<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  lookup_text: &str,
  matrix: &[Vec<FormulaValue<'doc>>],
  horizontal: bool,
  count: usize,
) -> Option<usize> {
  let mut index = None;
  for candidate_index in 0..count {
    let candidate = vhlookup_search_cell(matrix, horizontal, candidate_index);
    if candidate.is_none_or(is_lookup_string_or_empty) {
      let candidate_text = candidate
        .map(|value| evaluator.text(value))
        .unwrap_or_default();
      let ordering = compare_text(evaluator, &candidate_text, lookup_text, false);
      if ordering < 0 {
        index = Some(candidate_index);
      } else if ordering == 0 {
        index = Some(candidate_index);
        break;
      } else if candidate_index > 0 {
        break;
      }
    } else {
      index = Some(candidate_index);
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
  index
}

fn is_blank_for_countblank(value: &FormulaValue<'_>) -> bool {
  matches!(value, FormulaValue::Blank)
    || matches!(value, FormulaValue::String(text) if text.is_empty())
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
      let sheet = evaluator.range_sheet(reference);
      let range = if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
        let Some(range) = evaluator.book.data_area_subrange(sheet, reference.range) else {
          return 0;
        };
        range
      } else {
        reference.range
      };
      let mut count = 0usize;
      for sheet in aggregate_reference_sheet_ids(evaluator, reference) {
        for row in range.start.row..=range.end.row {
          if (options.ignore_filtered && evaluator.book.row_filtered(sheet, row))
            || (options.ignore_hidden && evaluator.book.row_hidden(sheet, row))
          {
            continue;
          }
          for column in range.start.column..=range.end.column {
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
      let sheet = evaluator.range_sheet(reference);
      let range = if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
        let Some(range) = evaluator.book.data_area_subrange(sheet, reference.range) else {
          return Ok(());
        };
        range
      } else {
        reference.range
      };
      for sheet in aggregate_reference_sheet_ids(evaluator, reference) {
        for row in range.start.row..=range.end.row {
          if (options.ignore_filtered && evaluator.book.row_filtered(sheet, row))
            || (options.ignore_hidden && evaluator.book.row_hidden(sheet, row))
          {
            continue;
          }
          for column in range.start.column..=range.end.column {
            let address = CellAddress { column, row };
            if options.ignore_nested && evaluator.book.is_nested_aggregate(sheet, address) {
              continue;
            }
            collect_aggregate_scalar(evaluator.book.cell_value(sheet, address), options, values)?;
          }
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

fn aggregate_reference_sheet_ids<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  reference: &QualifiedRange<'doc>,
) -> Vec<SheetId> {
  let start = evaluator.range_sheet(reference);
  let Some(end_name) = reference.end_sheet_name.as_ref() else {
    return vec![start];
  };
  let Some(end) = evaluator.book.sheet_id_by_name(end_name.0.as_ref()) else {
    return vec![start];
  };
  let Some(start_index) = evaluator
    .book
    .sheet_names
    .iter()
    .position(|sheet| sheet.id == start)
  else {
    return vec![start];
  };
  let Some(end_index) = evaluator
    .book
    .sheet_names
    .iter()
    .position(|sheet| sheet.id == end)
  else {
    return vec![start];
  };
  let (start_index, end_index) = if start_index <= end_index {
    (start_index, end_index)
  } else {
    (end_index, start_index)
  };
  evaluator.book.sheet_names[start_index..=end_index]
    .iter()
    .map(|sheet| sheet.id)
    .collect()
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
          let rest = rest.strip_prefix('!').unwrap_or(rest).trim_start();
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
    end_sheet_name: None,
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

fn propagate_binary_error(
  left: &FormulaValue<'_>,
  right: &FormulaValue<'_>,
) -> Option<FormulaErrorValue> {
  match (left, right) {
    (FormulaValue::Error(error), _) | (_, FormulaValue::Error(error)) => Some(*error),
    _ => None,
  }
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

pub(crate) fn datevalue(text: &str, date_system: DateSystem) -> FormulaValue<'static> {
  let text = text.trim();
  if text.is_empty() {
    return FormulaValue::Error(FormulaErrorValue::Value);
  }
  parse_date_input(text, date_system)
    .map(|value| FormulaValue::Number(value.floor()))
    .unwrap_or(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
}

pub(crate) fn parse_date_input(text: &str, date_system: DateSystem) -> Option<f64> {
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
  } else if text.contains('-') {
    '-'
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

  let (year, month, day) = if first.trim().len() >= 4 {
    (first_number, second_number, third_number)
  } else if separator == '-' {
    if third_number < 100 {
      third_number = expand_two_digit_year(third_number);
    }
    (third_number, second_number, first_number)
  } else {
    if third_number < 100 {
      third_number = expand_two_digit_year(third_number);
    }
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

pub(crate) fn formula_number_from_text(text: &str, date_system: DateSystem) -> Option<f64> {
  let text = text.trim();
  if text.is_empty() {
    return None;
  }
  text
    .parse::<f64>()
    .ok()
    .or_else(|| crate::parser::grouped_formula_number(text))
    .or_else(|| parse_date_time_number(text, date_system))
    .or_else(|| parse_permissive_excel_number(text))
}

fn parse_date_time_number(text: &str, date_system: DateSystem) -> Option<f64> {
  let date = parse_date_input(text, date_system);
  let time = match timevalue(text) {
    FormulaValue::Number(number) => Some(number),
    _ => None,
  };
  match (date, time) {
    (Some(date), Some(time)) if text.contains(':') => Some(date.floor() + time),
    (Some(date), _) => Some(date.floor()),
    (None, Some(time)) => Some(time),
    (None, None) => None,
  }
}

fn parse_permissive_excel_number(text: &str) -> Option<f64> {
  let mut previous_was_digit = false;
  let mut whitespace_after_digit = false;
  for ch in text.chars() {
    if ch.is_whitespace() {
      whitespace_after_digit |= previous_was_digit;
      continue;
    }
    if ch.is_ascii_digit() && whitespace_after_digit {
      return None;
    }
    previous_was_digit = ch.is_ascii_digit();
    whitespace_after_digit = false;
  }
  let mut text = text
    .chars()
    .filter(|ch| !ch.is_whitespace())
    .collect::<String>();
  let percent_count = text.chars().rev().take_while(|ch| *ch == '%').count();
  if percent_count != 0 {
    text.truncate(text.len() - percent_count);
  }

  let mut consumed_sign = false;
  let sign = if let Some(rest) = text.strip_prefix('+') {
    text = rest.to_string();
    consumed_sign = true;
    ""
  } else if let Some(rest) = text.strip_prefix('-') {
    text = rest.to_string();
    consumed_sign = true;
    "-"
  } else {
    ""
  };
  if let Some(rest) = text.strip_prefix('$') {
    text = rest.to_string();
  }
  if consumed_sign && (text.starts_with('+') || text.starts_with('-')) {
    return None;
  }
  let sign = if sign.is_empty() && !consumed_sign {
    if let Some(rest) = text.strip_prefix('+') {
      text = rest.to_string();
      ""
    } else if let Some(rest) = text.strip_prefix('-') {
      text = rest.to_string();
      "-"
    } else {
      ""
    }
  } else {
    sign
  };
  if text.is_empty() || text.starts_with(',') || text.matches('$').count() != 0 {
    return None;
  }
  if text.contains(',') {
    let mantissa_end = text
      .find(|ch| matches!(ch, 'e' | 'E'))
      .unwrap_or(text.len());
    let mantissa = &text[..mantissa_end];
    if mantissa.contains('.') && mantissa.find(',') > mantissa.find('.') {
      return None;
    }
    let integer_end = mantissa.find('.').unwrap_or(mantissa.len());
    let integer = &mantissa[..integer_end];
    let groups = integer.split(',').collect::<Vec<_>>();
    if groups.len() < 2
      || groups[0].is_empty()
      || groups[0].len() > 3
      || groups.iter().skip(1).any(|group| group.len() < 3)
      || groups
        .iter()
        .any(|group| group.is_empty() || !group.chars().all(|ch| ch.is_ascii_digit()))
    {
      return None;
    }
    text = text.replace(',', "");
  }
  format!("{sign}{text}")
    .parse::<f64>()
    .ok()
    .map(|value| value / 100_f64.powi(percent_count as i32))
}

fn push_unique_qualified_range<'doc>(
  ranges: &mut Vec<QualifiedRange<'doc>>,
  range: QualifiedRange<'doc>,
) {
  if !ranges.contains(&range) {
    ranges.push(range);
  }
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
    end_sheet_name: None,
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
    end_sheet_name: None,
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
