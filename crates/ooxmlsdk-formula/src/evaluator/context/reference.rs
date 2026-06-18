use super::*;

impl<'a, 'doc> FormulaEvaluator<'a, 'doc> {
  pub(crate) fn evaluate_name(&self, name: &Cow<'doc, str>) -> Option<FormulaValue<'doc>> {
    let local_key = name.trim_start_matches("_xlpm.").to_ascii_uppercase();
    if let Some(value) = self.locals.get(&local_key) {
      return Some(value.clone());
    }
    if let Some(range) = parse_table_reference(self.book, name.as_ref(), self.current_cell) {
      return Some(FormulaValue::Reference(range));
    }
    self
      .evaluate_defined_name(name)
      .or_else(|| self.evaluate_auto_row_label_name(name.as_ref()))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Name)))
  }

  fn evaluate_auto_row_label_name(&self, name: &str) -> Option<FormulaValue<'doc>> {
    if !matches!(
      self.grammar,
      FormulaGrammar::OpenFormula | FormulaGrammar::CalcA1
    ) {
      return None;
    }
    let current = self.current_cell?;
    let sheet = self.current_sheet;
    let label_column = self
      .book
      .cells
      .iter()
      .filter_map(|((cell_sheet, address), value)| {
        if *cell_sheet == sheet
          && address.row == current.row
          && address.column > current.column
          && let FormulaValue::String(text) = value
          && text.eq_ignore_ascii_case(name)
        {
          Some(address.column)
        } else {
          None
        }
      })
      .min()?;
    let start_column = current.column.checked_add(1)?;
    if start_column >= label_column {
      return None;
    }
    Some(FormulaValue::Reference(QualifiedRange {
      sheet,
      sheet_name: None,
      end_sheet_name: None,
      range: CellRange::new(
        CellAddress {
          column: start_column,
          row: current.row,
        },
        CellAddress {
          column: label_column - 1,
          row: current.row,
        },
      ),
      start_flags: AddressFlags::default(),
      end_flags: AddressFlags::default(),
    }))
  }

  pub(crate) fn evaluate_defined_name(&self, name: &Cow<'doc, str>) -> Option<FormulaValue<'doc>> {
    if let Some(array) = self
      .book
      .defined_name_array(Some(self.current_sheet), name.as_ref())
    {
      return Some(FormulaValue::Matrix(array.clone()));
    }
    let formula = self
      .book
      .defined_name_formula(Some(self.current_sheet), name.as_ref())?;
    if formula.trim().parse::<f64>().is_err()
      && let Ok(reference) = QualifiedRange::parse_a1(self.current_sheet, formula.as_ref())
    {
      return Some(FormulaValue::Reference(reference));
    }
    let parsed = parse_formula(
      self.current_sheet,
      Cow::Owned(formula.to_string()),
      self.grammar,
    );
    if !parsed.unsupported.is_empty() {
      return None;
    }
    evaluate_code_with_context(parsed.code.as_ref()?, self).map(FormulaValue::into_owned)
  }

  pub(crate) fn evaluate_external_reference(
    &self,
    reference: &ExternalReferenceId<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let link_index = reference.book.as_deref()?.parse::<usize>().ok()?;
    let name = reference.name.as_deref()?;
    let sheet_name = reference.sheet.as_deref();
    if sheet_name.is_none() {
      let formula = self
        .book
        .external_defined_name_formula(link_index, None, name)?;
      return self
        .book
        .evaluate_formula_text(self.current_sheet, self.current_cell, formula);
    }
    let sheet_name = sheet_name?;
    if name.contains(':') {
      let range = QualifiedRange::parse_a1(self.current_sheet, name).ok()?;
      let start_row = range.range.start.row.min(range.range.end.row);
      let end_row = range.range.start.row.max(range.range.end.row);
      let start_column = range.range.start.column.min(range.range.end.column);
      let end_column = range.range.start.column.max(range.range.end.column);
      let mut rows = Vec::new();
      for row in start_row..=end_row {
        let mut values = Vec::new();
        for column in start_column..=end_column {
          values.push(self.book.external_cell_value(
            link_index,
            sheet_name,
            CellAddress { column, row },
          ));
        }
        rows.push(values);
      }
      return Some(FormulaValue::Matrix(rows));
    }
    if let Ok(address) = CellAddress::parse_a1(name) {
      return Some(
        self
          .book
          .external_cell_value(link_index, sheet_name, address),
      );
    }
    let formula = self
      .book
      .external_defined_name_formula(link_index, Some(sheet_name), name)?;
    self
      .book
      .evaluate_formula_text(self.current_sheet, self.current_cell, formula)
  }

  pub(crate) fn evaluate_intersection_ranges(
    &self,
    left_ranges: Vec<QualifiedRange<'doc>>,
    right_ranges: Vec<QualifiedRange<'doc>>,
  ) -> Option<FormulaValue<'doc>> {
    if left_ranges.is_empty() || right_ranges.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut intersections = Vec::new();
    for left_range in &left_ranges {
      for right_range in &right_ranges {
        if let Some(range) = intersect_qualified_ranges(left_range, right_range) {
          intersections.push(range);
        }
      }
    }
    match intersections.len() {
      0 => Some(FormulaValue::Error(FormulaErrorValue::Null)),
      1 => Some(FormulaValue::Reference(intersections.pop()?)),
      _ => Some(FormulaValue::RefList(intersections)),
    }
  }

  pub(crate) fn evaluate_range_ranges(
    &self,
    ranges: Vec<QualifiedRange<'doc>>,
  ) -> Option<FormulaValue<'doc>> {
    if ranges.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if ranges.len() == 1 {
      return ranges.into_iter().next().map(FormulaValue::Reference);
    }
    Some(FormulaValue::RefList(ranges))
  }

  pub(crate) fn evaluate_union_ranges(
    &self,
    mut left_ranges: Vec<QualifiedRange<'doc>>,
    right_ranges: Vec<QualifiedRange<'doc>>,
  ) -> Option<FormulaValue<'doc>> {
    if left_ranges.is_empty() || right_ranges.is_empty() {
      return None;
    }
    left_ranges.extend(right_ranges);
    Some(FormulaValue::RefList(left_ranges))
  }

  pub(crate) fn range_reference_ranges_from_ranges(
    &self,
    left_ranges: Vec<QualifiedRange<'doc>>,
    right_ranges: Vec<QualifiedRange<'doc>>,
  ) -> Vec<QualifiedRange<'doc>> {
    if left_ranges.len() > 1 || right_ranges.len() > 1 {
      return bounding_qualified_ranges(&left_ranges)
        .zip(bounding_qualified_ranges(&right_ranges))
        .and_then(|(left, right)| extend_qualified_range(&left, &right))
        .into_iter()
        .collect();
    }
    let mut ranges = Vec::new();
    for left_range in &left_ranges {
      for right_range in &right_ranges {
        if let Some(range) = extend_qualified_range(left_range, right_range) {
          push_unique_qualified_range(&mut ranges, range);
        }
      }
    }
    ranges
  }

  pub(crate) fn as_reference(&self, value: &FormulaValue<'doc>) -> Option<QualifiedRange<'doc>> {
    match value {
      FormulaValue::Reference(reference) => Some(reference.clone()),
      FormulaValue::RefList(ranges) if ranges.len() == 1 => ranges.first().cloned(),
      _ => None,
    }
  }

  pub(crate) fn reference_ranges_from_value(
    &self,
    value: &FormulaValue<'doc>,
  ) -> Vec<QualifiedRange<'doc>> {
    match value {
      FormulaValue::Reference(reference) => vec![reference.clone()],
      FormulaValue::RefList(ranges) => ranges.clone(),
      _ => Vec::new(),
    }
  }

  pub(crate) fn resolve_reference(&self, reference: &str) -> Option<QualifiedRange<'doc>> {
    let reference = reference.trim();
    let normalized;
    let reference = if self.grammar == FormulaGrammar::ExcelR1C1 {
      normalized =
        crate::parser::r1c1_reference_to_a1(reference, self.current_cell.unwrap_or_default())
          .unwrap_or_else(|| reference.to_string());
      normalized.as_str()
    } else {
      reference
    };
    if let Some(table) = parse_table_reference(self.book, reference, self.current_cell) {
      return Some(table);
    }
    crate::parser::parse_formula_range(self.current_sheet, reference)
  }

  pub(crate) fn range_values(&self, range: &QualifiedRange<'doc>) -> Vec<FormulaValue<'doc>> {
    self
      .range_cells(range)
      .into_iter()
      .map(|(_, value)| value)
      .collect()
  }

  pub(crate) fn range_cells(
    &self,
    range: &QualifiedRange<'doc>,
  ) -> Vec<(CellAddress, FormulaValue<'doc>)> {
    let sheets = self.range_sheet_ids(range);
    if range.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
      let mut cells = Vec::new();
      for sheet in sheets {
        let mut addresses = self
          .book
          .cells
          .range(
            (sheet, CellAddress { column: 0, row: 0 })
              ..=(
                sheet,
                CellAddress {
                  column: u32::MAX,
                  row: u32::MAX,
                },
              ),
          )
          .filter_map(|((cell_sheet, address), _)| {
            (*cell_sheet == sheet && cell_in_range(*address, &range.range)).then_some(*address)
          })
          .collect::<Vec<_>>();
        addresses.sort_by_key(|address| (address.row, address.column));
        cells.extend(
          addresses
            .into_iter()
            .map(|address| (address, self.book.cell_value(sheet, address))),
        );
      }
      return cells;
    }

    let start_row = range.range.start.row.min(range.range.end.row);
    let end_row = range.range.start.row.max(range.range.end.row);
    let start_column = range.range.start.column.min(range.range.end.column);
    let end_column = range.range.start.column.max(range.range.end.column);
    let mut values = Vec::new();
    for sheet in sheets {
      for row in start_row..=end_row {
        for column in start_column..=end_column {
          let address = CellAddress { column, row };
          values.push((address, self.book.cell_value(sheet, address)));
        }
      }
    }
    values
  }

  pub(crate) fn first_error_value(&self, value: &FormulaValue<'doc>) -> Option<FormulaValue<'doc>> {
    match value {
      FormulaValue::Reference(reference) => self
        .range_values(reference)
        .iter()
        .find_map(first_error_in_value),
      FormulaValue::RefList(ranges) => ranges
        .iter()
        .find_map(|range| self.first_error_value(&FormulaValue::Reference(range.clone()))),
      FormulaValue::Matrix(rows) => rows
        .iter()
        .flatten()
        .find_map(|value| self.first_error_value(value)),
      FormulaValue::Error(error) => Some(FormulaValue::Error(*error)),
      _ => None,
    }
  }

  pub(crate) fn pivot_data(
    &self,
    request: &PivotDataRequest<'doc>,
  ) -> std::result::Result<FormulaValue<'doc>, FormulaErrorValue> {
    let block_sheet = self.range_sheet(&request.block);
    let pivot = self
      .book
      .pivot_tables
      .iter()
      .rev()
      .find(|pivot| {
        self.range_sheet(&pivot.target) == block_sheet
          && ranges_intersect(&pivot.target.range, &request.block.range)
      })
      .ok_or(FormulaErrorValue::Ref)?;
    let source_sheet = self.range_sheet(&pivot.source);
    let fields =
      pivot_source_headers(self.book, source_sheet, pivot).ok_or(FormulaErrorValue::Ref)?;
    let data_field =
      pivot_data_field(pivot, request.data_field_name.as_deref()).ok_or(FormulaErrorValue::Ref)?;
    let data_column = fields
      .iter()
      .position(|field| pivot_name_eq(field, &data_field.name))
      .ok_or(FormulaErrorValue::Ref)?;
    let mut filter_columns = Vec::with_capacity(request.filters.len());
    for filter in &request.filters {
      let column = fields
        .iter()
        .position(|field| pivot_name_eq(field, &filter.field_name))
        .ok_or(FormulaErrorValue::Ref)?;
      filter_columns.push((column, filter.match_value.as_ref()));
    }
    if pivot_row_filter_is_ambiguous(self.book, block_sheet, pivot, request) {
      return Err(FormulaErrorValue::Ref);
    }

    let mut values = Vec::new();
    let source = &pivot.source.range;
    for row in source.start.row.saturating_add(1)..=source.end.row {
      let mut matched = true;
      for (column_offset, expected) in &filter_columns {
        let address = CellAddress {
          column: source.start.column + *column_offset as u32,
          row,
        };
        let actual = self.text(&self.book.cell_value(source_sheet, address));
        if !pivot_value_eq(&actual, expected) {
          matched = false;
          break;
        }
      }
      if matched {
        let address = CellAddress {
          column: source.start.column + data_column as u32,
          row,
        };
        if let Some(number) = self.number(&self.book.cell_value(source_sheet, address)) {
          values.push(number);
        }
      }
    }
    if values.is_empty() {
      return Err(FormulaErrorValue::Ref);
    }
    let result = match data_field.function {
      FormulaPivotFunction::Count => values.len() as f64,
      FormulaPivotFunction::Average => values.iter().sum::<f64>() / values.len() as f64,
      FormulaPivotFunction::Max => values
        .into_iter()
        .reduce(f64::max)
        .ok_or(FormulaErrorValue::Ref)?,
      FormulaPivotFunction::Min => values
        .into_iter()
        .reduce(f64::min)
        .ok_or(FormulaErrorValue::Ref)?,
      FormulaPivotFunction::Auto | FormulaPivotFunction::Sum => values.iter().sum(),
    };
    Ok(FormulaValue::Number(result))
  }

  pub(crate) fn range_sheet(&self, range: &QualifiedRange<'doc>) -> SheetId {
    range
      .sheet_name
      .as_ref()
      .and_then(|name| self.book.sheet_id_by_name(name.0.as_ref()))
      .unwrap_or(range.sheet)
  }

  fn range_sheet_ids(&self, range: &QualifiedRange<'doc>) -> Vec<SheetId> {
    let start = self.range_sheet(range);
    let Some(end_name) = range.end_sheet_name.as_ref() else {
      return vec![start];
    };
    let Some(end) = self.book.sheet_id_by_name(end_name.0.as_ref()) else {
      return vec![start];
    };
    let Some(start_index) = self
      .book
      .sheet_names
      .iter()
      .position(|sheet| sheet.id == start)
    else {
      return vec![start];
    };
    let Some(end_index) = self
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
    self.book.sheet_names[start_index..=end_index]
      .iter()
      .map(|sheet| sheet.id)
      .collect()
  }
}
