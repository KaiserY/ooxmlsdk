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
      .or(Some(FormulaValue::Error(FormulaErrorValue::Name)))
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
    let ast = crate::evaluator::ast_from_code(parsed.code.as_ref()?)?;
    self.evaluate(&ast).map(FormulaValue::into_owned)
  }

  pub(crate) fn evaluate_external_reference(
    &self,
    reference: &ExternalReferenceId<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let link_index = reference.book.as_deref()?.parse::<usize>().ok()?;
    let sheet_name = reference.sheet.as_deref()?;
    let name = reference.name.as_deref()?;
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
    let address = CellAddress::parse_a1(name).ok()?;
    Some(
      self
        .book
        .external_cell_value(link_index, sheet_name, address),
    )
  }

  pub(crate) fn evaluate_intersection_ast(
    &self,
    left: &FormulaAst<'doc>,
    right: &FormulaAst<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let left_ranges = self.reference_ranges_from_ast(left);
    let right_ranges = self.reference_ranges_from_ast(right);
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

  pub(crate) fn evaluate_range_ast(
    &self,
    left: &FormulaAst<'doc>,
    right: &FormulaAst<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let ranges = self.range_reference_ranges_from_ast(left, right);
    if ranges.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if ranges.len() == 1 {
      return ranges.into_iter().next().map(FormulaValue::Reference);
    }
    Some(FormulaValue::RefList(ranges))
  }

  pub(crate) fn range_reference_ranges_from_ast(
    &self,
    left: &FormulaAst<'doc>,
    right: &FormulaAst<'doc>,
  ) -> Vec<QualifiedRange<'doc>> {
    let left_ranges = self.reference_ranges_from_ast(left);
    let right_ranges = self.reference_ranges_from_ast(right);
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

  pub(crate) fn reference_ranges_from_ast(
    &self,
    ast: &FormulaAst<'doc>,
  ) -> Vec<QualifiedRange<'doc>> {
    match ast {
      FormulaAst::Reference(range) => vec![range.clone()],
      FormulaAst::Binary {
        op: FormulaOperator::Union,
        left,
        right,
      } => {
        let mut ranges = self.reference_ranges_from_ast(left);
        ranges.extend(self.reference_ranges_from_ast(right));
        ranges
      }
      FormulaAst::Binary {
        op: FormulaOperator::Range,
        left,
        right,
      } => self.range_reference_ranges_from_ast(left, right),
      FormulaAst::Binary {
        op: FormulaOperator::Intersection,
        left,
        right,
      } => {
        let left_ranges = self.reference_ranges_from_ast(left);
        let right_ranges = self.reference_ranges_from_ast(right);
        let mut intersections = Vec::new();
        for left_range in &left_ranges {
          for right_range in &right_ranges {
            if let Some(range) = intersect_qualified_ranges(left_range, right_range) {
              intersections.push(range);
            }
          }
        }
        intersections
      }
      FormulaAst::Function { name, args, .. }
        if canonical_function_name(name).as_ref() == "XLOOKUP" =>
      {
        self.xlookup_reference_ranges(args).unwrap_or_default()
      }
      FormulaAst::Name(_) | FormulaAst::ExternalReference(_) | FormulaAst::Function { .. } => self
        .evaluate(ast)
        .map(|value| self.reference_ranges_from_value(&value))
        .unwrap_or_default(),
      _ => Vec::new(),
    }
  }

  pub(crate) fn xlookup_reference_ranges(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<Vec<QualifiedRange<'doc>>> {
    if args.len() < 3 {
      return None;
    }
    let lookup = self.scalar_value(self.evaluate(args.first()?)?);
    let lookup_reference = self.reference_ranges_from_ast(args.get(1)?).pop()?;
    let return_reference = self.reference_ranges_from_ast(args.get(2)?).pop()?;
    let lookup_matrix = self.matrix_values(&FormulaValue::Reference(lookup_reference.clone()));
    let lookup_rows = lookup_matrix.len();
    let lookup_columns = lookup_matrix.first().map_or(0, Vec::len);
    if lookup_rows > 1 && lookup_columns > 1 {
      return None;
    }
    let (lookup_vector, lookup_vertical) = lookup_vector(&lookup_matrix)?;
    let return_rows = return_reference
      .range
      .start
      .row
      .abs_diff(return_reference.range.end.row)
      + 1;
    let return_columns = return_reference
      .range
      .start
      .column
      .abs_diff(return_reference.range.end.column)
      + 1;
    if (lookup_vertical && return_rows as usize != lookup_vector.len())
      || (!lookup_vertical && return_columns as usize != lookup_vector.len())
    {
      return None;
    }
    let match_mode = args
      .get(4)
      .and_then(|arg| self.optional_number_value(arg))
      .unwrap_or(0.0) as i32;
    let search_mode = args
      .get(5)
      .and_then(|arg| self.optional_number_value(arg))
      .unwrap_or(1.0) as i32;
    let search = LookupSearchMode::from_excel(search_mode)?;
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
      _ => return None,
    }?;
    let address = if lookup_vertical {
      CellAddress {
        column: return_reference.range.start.column,
        row: return_reference.range.start.row + index as u32,
      }
    } else {
      CellAddress {
        column: return_reference.range.start.column + index as u32,
        row: return_reference.range.start.row,
      }
    };
    Some(vec![QualifiedRange {
      sheet: return_reference.sheet,
      sheet_name: return_reference.sheet_name,
      range: CellRange {
        start: address,
        end: address,
      },
      start_flags: return_reference.start_flags,
      end_flags: return_reference.end_flags,
    }])
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
    let sheet = self.range_sheet(range);
    if range.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
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
      return addresses
        .into_iter()
        .map(|address| (address, self.book.cell_value(sheet, address)))
        .collect();
    }

    let start_row = range.range.start.row.min(range.range.end.row);
    let end_row = range.range.start.row.max(range.range.end.row);
    let start_column = range.range.start.column.min(range.range.end.column);
    let end_column = range.range.start.column.max(range.range.end.column);
    let mut values = Vec::new();
    for row in start_row..=end_row {
      for column in start_column..=end_column {
        let address = CellAddress { column, row };
        values.push((address, self.book.cell_value(sheet, address)));
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
}
