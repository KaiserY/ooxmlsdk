use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum IfsAggregate {
  Sum,
  Count,
  Average,
  Min,
  Max,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct QueryCell<'doc> {
  pub(crate) value: FormulaValue<'doc>,
  pub(crate) query_empty: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct QueryValueSource<'doc> {
  kind: QueryValueSourceKind<'doc>,
  rows: usize,
  columns: usize,
  rectangular: bool,
}

#[derive(Clone, Debug, PartialEq)]
enum QueryValueSourceKind<'doc> {
  Reference(QualifiedRange<'doc>),
  RefList(Vec<QueryValueSource<'doc>>),
  Matrix(Vec<Vec<FormulaValue<'doc>>>),
  Scalar(FormulaValue<'doc>),
  Empty,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CriteriaPlan<'doc> {
  op: QueryOp,
  operand: CriteriaOperand<'doc>,
  search_type: QuerySearchType,
  match_whole_cell: bool,
  case_sensitive: bool,
  match_empty: bool,
  empty_matches_text: bool,
}

#[derive(Clone, Debug, PartialEq)]
enum CriteriaOperand<'doc> {
  Number {
    value: f64,
    source_text: Option<Cow<'doc, str>>,
  },
  Text(Cow<'doc, str>),
  Empty,
  NonEmpty,
  Blank,
  Boolean(bool),
  Error(FormulaErrorValue),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FieldCriteriaPlan<'doc> {
  field: usize,
  criteria: CriteriaPlan<'doc>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct DatabaseCriteriaPlan<'doc> {
  groups: Vec<Vec<FieldCriteriaPlan<'doc>>>,
}

impl<'doc> QueryValueSource<'doc> {
  pub(crate) fn from_value(
    evaluator: &FormulaEvaluator<'_, 'doc>,
    value: FormulaValue<'doc>,
  ) -> Option<Self> {
    Some(match value {
      FormulaValue::Reference(reference) => Self::from_reference(evaluator, reference),
      FormulaValue::RefList(ranges) if ranges.len() == 1 => {
        Self::from_reference(evaluator, ranges.into_iter().next()?)
      }
      FormulaValue::RefList(ranges) => {
        let sources = ranges
          .into_iter()
          .map(|range| Self::from_reference(evaluator, range))
          .collect::<Vec<_>>();
        let rows = sources.iter().map(|source| source.rows).sum();
        let columns = sources.first().map_or(0, |source| source.columns);
        let rectangular = sources
          .iter()
          .all(|source| source.rectangular && source.columns == columns);
        Self {
          kind: QueryValueSourceKind::RefList(sources),
          rows,
          columns,
          rectangular,
        }
      }
      FormulaValue::Matrix(values) => {
        let rows = values.len();
        let columns = values.first().map_or(0, Vec::len);
        let rectangular = values.iter().all(|row| row.len() == columns);
        Self {
          kind: QueryValueSourceKind::Matrix(values),
          rows,
          columns,
          rectangular,
        }
      }
      value => Self {
        kind: QueryValueSourceKind::Scalar(value),
        rows: 1,
        columns: 1,
        rectangular: true,
      },
    })
  }

  pub(crate) fn from_reference(
    evaluator: &FormulaEvaluator<'_, 'doc>,
    mut reference: QualifiedRange<'doc>,
  ) -> Self {
    let sheet = evaluator.range_sheet(&reference);
    if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
      let Some(range) = evaluator.book.data_area_subrange(sheet, reference.range) else {
        return Self {
          kind: QueryValueSourceKind::Empty,
          rows: 0,
          columns: 0,
          rectangular: true,
        };
      };
      reference.range = range;
    }
    let rows = reference.range.start.row.abs_diff(reference.range.end.row) as usize + 1;
    let columns = reference
      .range
      .start
      .column
      .abs_diff(reference.range.end.column) as usize
      + 1;
    Self {
      kind: QueryValueSourceKind::Reference(reference),
      rows,
      columns,
      rectangular: true,
    }
  }

  pub(crate) fn dimensions(&self) -> (usize, usize) {
    (self.rows, self.columns)
  }

  pub(crate) fn is_ref_list(&self) -> bool {
    matches!(self.kind, QueryValueSourceKind::RefList(_))
  }

  pub(crate) fn is_rectangular(&self) -> bool {
    self.rectangular
  }

  pub(crate) fn resized_from_top_left(&self, rows: usize, columns: usize) -> Option<Self> {
    if rows == 0 || columns == 0 {
      return None;
    }
    match &self.kind {
      QueryValueSourceKind::Reference(reference) => {
        let start_row = reference.range.start.row.min(reference.range.end.row);
        let start_column = reference.range.start.column.min(reference.range.end.column);
        let row_count = u32::try_from(rows).ok()?;
        let column_count = u32::try_from(columns).ok()?;
        let end_row = start_row.checked_add(row_count)?.checked_sub(1)?;
        let end_column = start_column.checked_add(column_count)?.checked_sub(1)?;
        Some(Self {
          kind: QueryValueSourceKind::Reference(QualifiedRange {
            sheet: reference.sheet,
            sheet_name: reference.sheet_name.clone(),
            end_sheet_name: reference.end_sheet_name.clone(),
            range: CellRange {
              start: CellAddress {
                column: start_column,
                row: start_row,
              },
              end: CellAddress {
                column: end_column,
                row: end_row,
              },
            },
            start_flags: reference.start_flags,
            end_flags: reference.end_flags,
          }),
          rows,
          columns,
          rectangular: true,
        })
      }
      _ => None,
    }
  }

  pub(crate) fn value_at(
    &self,
    evaluator: &FormulaEvaluator<'_, 'doc>,
    row: usize,
    column: usize,
  ) -> Option<QueryCell<'doc>> {
    match &self.kind {
      QueryValueSourceKind::Reference(reference) => {
        if row >= self.rows || column >= self.columns {
          return None;
        }
        let sheet = evaluator.range_sheet(reference);
        let start_row = reference.range.start.row.min(reference.range.end.row);
        let start_column = reference.range.start.column.min(reference.range.end.column);
        let address = CellAddress {
          row: start_row + row as u32,
          column: start_column + column as u32,
        };
        let value = evaluator.book.query_cell_value(
          sheet,
          address,
          evaluator.book.cell_value(sheet, address),
        );
        Some(QueryCell {
          value,
          query_empty: evaluator.book.is_query_empty_cell(sheet, address),
        })
      }
      QueryValueSourceKind::RefList(sources) => {
        let mut row = row;
        for source in sources {
          if row < source.rows {
            return source.value_at(evaluator, row, column);
          }
          row -= source.rows;
        }
        None
      }
      QueryValueSourceKind::Matrix(values) => values.get(row).and_then(|values| {
        values.get(column).cloned().map(|value| QueryCell {
          value,
          query_empty: false,
        })
      }),
      QueryValueSourceKind::Scalar(value) => (row == 0 && column == 0).then(|| QueryCell {
        value: value.clone(),
        query_empty: false,
      }),
      QueryValueSourceKind::Empty => None,
    }
  }

  pub(crate) fn header_row(
    &self,
    evaluator: &FormulaEvaluator<'_, 'doc>,
  ) -> Option<Vec<FormulaValue<'doc>>> {
    (0..self.columns)
      .map(|column| self.value_at(evaluator, 0, column).map(|cell| cell.value))
      .collect()
  }
}

impl<'doc> CriteriaPlan<'doc> {
  pub(crate) fn from_criterion(
    evaluator: &FormulaEvaluator<'_, 'doc>,
    value: &FormulaValue<'doc>,
  ) -> Self {
    Self::from_value(evaluator, value, false)
  }

  pub(crate) fn from_database_value(
    evaluator: &FormulaEvaluator<'_, 'doc>,
    value: &FormulaValue<'doc>,
  ) -> Self {
    Self::from_value(evaluator, value, true)
  }

  fn from_value(
    evaluator: &FormulaEvaluator<'_, 'doc>,
    value: &FormulaValue<'doc>,
    database: bool,
  ) -> Self {
    let value = evaluator.first_value(value);
    if let FormulaValue::String(text) = value {
      let (op, operand) = parse_criteria_operator(text.as_ref());
      let trimmed = operand.trim();
      let is_empty_criterion = operand.is_empty();
      let explicit_empty_operator = matches!(text.as_ref(), "=" | "<>");
      let operand = if let Ok(number) =
        parse_query_number_format(trimmed, evaluator.book.date_system)
      {
        CriteriaOperand::Number {
          value: number,
          source_text: Some(Cow::Owned(operand.to_string())),
        }
      } else if let Some(error) = criteria_error_value(trimmed) {
        CriteriaOperand::Error(error)
      } else if !database && is_empty_criterion && matches!(op, QueryOp::Equal | QueryOp::NotEqual)
      {
        if op == QueryOp::Equal {
          CriteriaOperand::Empty
        } else {
          CriteriaOperand::NonEmpty
        }
      } else {
        CriteriaOperand::Text(Cow::Owned(operand.to_string()))
      };
      let search_type = if matches!(operand, CriteriaOperand::Text(_)) {
        if let CriteriaOperand::Text(pattern) = &operand {
          detect_query_search_type(evaluator.book.formula_search_type, pattern)
        } else {
          QuerySearchType::Normal
        }
      } else {
        QuerySearchType::Normal
      };
      Self {
        op,
        operand,
        search_type,
        match_whole_cell: evaluator.book.formula_match_whole_cell,
        case_sensitive: false,
        match_empty: !database
          && ((op == QueryOp::Equal && is_empty_criterion)
            || (op == QueryOp::NotEqual && !is_empty_criterion)),
        empty_matches_text: !database && is_empty_criterion && !explicit_empty_operator,
      }
    } else {
      let operand = match value {
        FormulaValue::Blank => CriteriaOperand::Number {
          value: 0.0,
          source_text: None,
        },
        FormulaValue::Number(value) => CriteriaOperand::Number {
          value,
          source_text: None,
        },
        FormulaValue::String(text) => CriteriaOperand::Text(text),
        FormulaValue::Boolean(value) => CriteriaOperand::Boolean(value),
        FormulaValue::Error(value) => CriteriaOperand::Error(value),
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_) => {
          CriteriaOperand::Blank
        }
      };
      Self {
        op: QueryOp::Equal,
        operand,
        search_type: QuerySearchType::Normal,
        match_whole_cell: evaluator.book.formula_match_whole_cell,
        case_sensitive: false,
        match_empty: false,
        empty_matches_text: false,
      }
    }
  }

  pub(crate) fn with_op(mut self, op: QueryOp) -> Self {
    self.op = op;
    self
  }

  pub(crate) fn matches(
    &self,
    evaluator: &FormulaEvaluator<'_, 'doc>,
    candidate: &FormulaValue<'doc>,
    candidate_query_empty: bool,
  ) -> bool {
    if let CriteriaOperand::Error(query) = self.operand {
      let ordering = match candidate {
        FormulaValue::Error(candidate) => {
          formula_error_criteria_code(*candidate).cmp(&formula_error_criteria_code(query))
        }
        _ => return matches!(self.op, QueryOp::NotEqual),
      };
      return self.compare_ordering(Some(match ordering {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
      }));
    }
    if matches!(candidate, FormulaValue::Error(_)) {
      return false;
    }
    match &self.operand {
      CriteriaOperand::Empty | CriteriaOperand::NonEmpty => {
        let blank = candidate_query_empty
          || matches!(candidate, FormulaValue::Blank)
          || (self.empty_matches_text
            && matches!(candidate, FormulaValue::String(text) if text.is_empty()));
        return matches!(
          (&self.operand, blank),
          (CriteriaOperand::Empty, true) | (CriteriaOperand::NonEmpty, false)
        );
      }
      _ => {}
    }
    if self.match_empty && (candidate_query_empty || is_query_empty(candidate)) {
      return matches!(
        self.op,
        QueryOp::NotEqual | QueryOp::LessOrEqual | QueryOp::GreaterOrEqual
      );
    }
    match &self.operand {
      CriteriaOperand::Number { value, source_text } => {
        self.matches_number(evaluator, candidate, *value, source_text)
      }
      CriteriaOperand::Text(pattern) => self.matches_text(evaluator, candidate, pattern),
      CriteriaOperand::Blank => self.compare_ordering(Some(match candidate {
        FormulaValue::Blank => 0,
        _ => return matches!(self.op, QueryOp::NotEqual),
      })),
      CriteriaOperand::Boolean(query) => self.compare_ordering(match candidate {
        FormulaValue::Boolean(candidate) => Some(match candidate.cmp(query) {
          std::cmp::Ordering::Less => -1,
          std::cmp::Ordering::Equal => 0,
          std::cmp::Ordering::Greater => 1,
        }),
        _ => None,
      }),
      CriteriaOperand::Error(_) => unreachable!(),
      CriteriaOperand::Empty | CriteriaOperand::NonEmpty => unreachable!(),
    }
  }

  fn matches_number(
    &self,
    evaluator: &FormulaEvaluator<'_, 'doc>,
    candidate: &FormulaValue<'doc>,
    query: f64,
    source_text: &Option<Cow<'doc, str>>,
  ) -> bool {
    if let Some(source_text) = source_text
      && !self.match_whole_cell
      && self.op == QueryOp::Equal
      && let FormulaValue::String(candidate_text) = candidate
    {
      let matched = if self.case_sensitive {
        candidate_text.contains(source_text.as_ref())
      } else {
        lookup_text_contains(candidate_text, source_text)
      };
      if matched {
        return true;
      }
    }
    let Some(candidate_number) =
      criteria_candidate_number(candidate, matches!(self.op, QueryOp::Equal))
    else {
      if let Some(source_text) = source_text
        && let FormulaValue::String(candidate_text) = candidate
        && self.op == QueryOp::Equal
      {
        let matched = if self.match_whole_cell {
          compare_text(evaluator, candidate_text, source_text, self.case_sensitive) == 0
        } else if self.case_sensitive {
          candidate_text.contains(source_text.as_ref())
        } else {
          lookup_text_contains(candidate_text, source_text)
        };
        return matched;
      }
      return matches!(self.op, QueryOp::NotEqual);
    };
    self.compare_ordering(Some(compare_numbers(candidate_number, query)))
  }

  fn matches_text(
    &self,
    evaluator: &FormulaEvaluator<'_, 'doc>,
    candidate: &FormulaValue<'doc>,
    pattern: &str,
  ) -> bool {
    if matches!(candidate, FormulaValue::Number(_)) {
      return matches!(self.op, QueryOp::NotEqual);
    }
    if self.search_type == QuerySearchType::Wildcard {
      let text = evaluator.text(candidate);
      let matched = if self.match_whole_cell {
        wildcard_match(pattern, &text)
      } else {
        wildcard_match(pattern, &text) || lookup_text_contains(&text, pattern)
      };
      return match self.op {
        QueryOp::Equal => matched,
        QueryOp::NotEqual => !matched,
        _ => false,
      };
    }
    if self.search_type == QuerySearchType::Regex {
      let matched =
        regex_match(pattern, &evaluator.text(candidate), self.match_whole_cell).unwrap_or(false);
      return match self.op {
        QueryOp::Equal => matched,
        QueryOp::NotEqual => !matched,
        _ => false,
      };
    }
    if !self.match_whole_cell && matches!(self.op, QueryOp::Equal | QueryOp::NotEqual) {
      let FormulaValue::String(candidate_text) = candidate else {
        return self.op == QueryOp::NotEqual;
      };
      let matched = if self.case_sensitive {
        candidate_text.contains(pattern)
      } else {
        lookup_text_contains(candidate_text, pattern)
      };
      return if self.op == QueryOp::Equal {
        matched
      } else {
        !matched
      };
    }
    self.compare_ordering(match candidate {
      FormulaValue::String(candidate) => Some(compare_text(
        evaluator,
        candidate,
        pattern,
        self.case_sensitive,
      )),
      _ => None,
    })
  }

  fn compare_ordering(&self, ordering: Option<i32>) -> bool {
    match self.op {
      QueryOp::Equal => ordering == Some(0),
      QueryOp::NotEqual => ordering != Some(0),
      QueryOp::Less => ordering == Some(-1),
      QueryOp::LessOrEqual => matches!(ordering, Some(-1 | 0)),
      QueryOp::Greater => ordering == Some(1),
      QueryOp::GreaterOrEqual => matches!(ordering, Some(0 | 1)),
    }
  }
}

impl<'doc> FieldCriteriaPlan<'doc> {
  pub(crate) fn new(field: usize, criteria: CriteriaPlan<'doc>) -> Self {
    Self { field, criteria }
  }
}

impl<'doc> DatabaseCriteriaPlan<'doc> {
  pub(crate) fn push_group(&mut self, group: Vec<FieldCriteriaPlan<'doc>>) {
    self.groups.push(group);
  }

  pub(crate) fn is_empty(&self) -> bool {
    self.groups.is_empty()
  }

  pub(crate) fn matches_row(
    &self,
    evaluator: &FormulaEvaluator<'_, 'doc>,
    database: &QueryValueSource<'doc>,
    row: usize,
  ) -> bool {
    self.groups.iter().any(|group| {
      group.iter().all(|criteria| {
        database
          .value_at(evaluator, row, criteria.field)
          .is_some_and(|cell| {
            criteria
              .criteria
              .matches(evaluator, &cell.value, cell.query_empty)
          })
      })
    })
  }
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
struct LookupKey<'doc> {
  value: FormulaValue<'doc>,
  source_text: Option<Cow<'doc, str>>,
  kind: QueryValueKind,
  match_empty: bool,
  empty_matches_text: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LookupPlan<'doc> {
  op: QueryOp,
  key: LookupKey<'doc>,
  search_type: QuerySearchType,
  range_lookup: bool,
  match_whole_cell: bool,
  case_sensitive: bool,
}

impl<'doc> LookupPlan<'doc> {
  pub(crate) fn new(
    value: &FormulaValue<'doc>,
    op: QueryOp,
    search_type: QuerySearchType,
    match_whole_cell: bool,
    range_lookup: bool,
    match_empty: bool,
  ) -> Self {
    Self {
      op,
      key: LookupKey {
        kind: query_value_kind(value),
        value: value.clone(),
        source_text: None,
        match_empty,
        empty_matches_text: false,
      },
      search_type,
      range_lookup,
      match_whole_cell,
      case_sensitive: false,
    }
  }

  pub(crate) fn op(&self) -> QueryOp {
    self.op
  }

  pub(crate) fn expects_text(&self) -> bool {
    self.key.kind == QueryValueKind::Text
  }

  pub(crate) fn expects_number(&self) -> bool {
    self.key.kind == QueryValueKind::Number
  }

  pub(crate) fn with_op(mut self, op: QueryOp) -> Self {
    self.op = op;
    self
  }

  pub(crate) fn from_criteria_value(
    evaluator: &FormulaEvaluator<'_, 'doc>,
    value: &FormulaValue<'doc>,
  ) -> Self {
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
      Self {
        op,
        key: LookupKey {
          value: operand_value,
          source_text,
          kind,
          match_empty: (op == QueryOp::Equal && is_empty_criterion)
            || (op == QueryOp::NotEqual && !is_empty_criterion),
          empty_matches_text: is_empty_criterion && !explicit_empty_operator,
        },
        search_type,
        range_lookup: false,
        match_whole_cell: evaluator.book.formula_match_whole_cell,
        case_sensitive: false,
      }
    } else {
      let value = if matches!(value, FormulaValue::Blank) {
        FormulaValue::Number(0.0)
      } else {
        value
      };
      Self {
        op: QueryOp::Equal,
        key: LookupKey {
          kind: query_value_kind(&value),
          value,
          source_text: None,
          match_empty: false,
          empty_matches_text: false,
        },
        search_type: QuerySearchType::Normal,
        range_lookup: false,
        match_whole_cell: evaluator.book.formula_match_whole_cell,
        case_sensitive: false,
      }
    }
  }

  pub(crate) fn with_range_lookup(mut self, range_lookup: bool) -> Self {
    self.range_lookup = range_lookup;
    self
  }

  pub(crate) fn matches(
    &self,
    evaluator: &FormulaEvaluator<'_, 'doc>,
    candidate: &FormulaValue<'doc>,
    candidate_query_empty: bool,
  ) -> bool {
    if matches!(candidate, FormulaValue::Error(_)) {
      return false;
    }
    if matches!(
      self.key.kind,
      QueryValueKind::Empty | QueryValueKind::NonEmpty
    ) {
      let blank = candidate_query_empty
        || matches!(candidate, FormulaValue::Blank)
        || (self.key.empty_matches_text
          && matches!(candidate, FormulaValue::String(text) if text.is_empty()));
      return if self.key.kind == QueryValueKind::Empty {
        blank
      } else {
        !blank
      };
    }
    if self.key.match_empty && (candidate_query_empty || is_query_empty(candidate)) {
      return matches!(
        self.op,
        QueryOp::NotEqual | QueryOp::LessOrEqual | QueryOp::GreaterOrEqual
      );
    }
    if !self.range_lookup
      && self.key.kind == QueryValueKind::Number
      && query_candidate_number(candidate, candidate_query_empty).is_none()
    {
      if let Some(source_text) = &self.key.source_text
        && let FormulaValue::String(candidate_text) = candidate
        && matches!(self.op, QueryOp::Equal | QueryOp::NotEqual)
      {
        let matched = if self.match_whole_cell {
          compare_text(evaluator, candidate_text, source_text, self.case_sensitive) == 0
        } else if self.case_sensitive {
          candidate_text.contains(source_text.as_ref())
        } else {
          lookup_text_contains(candidate_text, source_text)
        };
        return if self.op == QueryOp::Equal {
          matched
        } else {
          !matched
        };
      }
      return matches!(self.op, QueryOp::NotEqual);
    }
    if !self.range_lookup
      && self.key.kind == QueryValueKind::Text
      && matches!(candidate, FormulaValue::Number(_))
    {
      return matches!(self.op, QueryOp::NotEqual);
    }
    if self.search_type == QuerySearchType::Wildcard {
      let FormulaValue::String(pattern) = &self.key.value else {
        return false;
      };
      let text = evaluator.text(candidate);
      let matched = if self.match_whole_cell {
        wildcard_match(pattern.as_ref(), &text)
      } else {
        wildcard_match(pattern.as_ref(), &text) || lookup_text_contains(&text, pattern.as_ref())
      };
      return match self.op {
        QueryOp::Equal => matched,
        QueryOp::NotEqual => !matched,
        _ => false,
      };
    }
    if self.search_type == QuerySearchType::Regex {
      let FormulaValue::String(pattern) = &self.key.value else {
        return false;
      };
      let matched =
        regex_match(pattern, &evaluator.text(candidate), self.match_whole_cell).unwrap_or(false);
      return match self.op {
        QueryOp::Equal => matched,
        QueryOp::NotEqual => !matched,
        _ => false,
      };
    }
    if !self.match_whole_cell
      && matches!(self.op, QueryOp::Equal | QueryOp::NotEqual)
      && let (FormulaValue::String(candidate_text), FormulaValue::String(query_text)) =
        (candidate, &self.key.value)
    {
      let matched = if self.case_sensitive {
        candidate_text.contains(query_text.as_ref())
      } else {
        lookup_text_contains(candidate_text, query_text)
      };
      return if self.op == QueryOp::Equal {
        matched
      } else {
        !matched
      };
    }
    let ordering = self.compare_value(evaluator, candidate, candidate_query_empty);
    if ordering.is_none() && self.range_lookup {
      return self.compare_by_range_lookup(candidate);
    }
    self.compare_ordering(ordering)
  }

  pub(crate) fn candidate_type_matches(&self, candidate: &FormulaValue<'_>) -> bool {
    if self.expects_text() {
      !matches!(candidate, FormulaValue::Number(_))
    } else if self.expects_number() {
      query_candidate_number(candidate, false).is_some()
    } else {
      true
    }
  }

  pub(crate) fn compare_candidate(
    &self,
    evaluator: &FormulaEvaluator<'_, 'doc>,
    candidate: &FormulaValue<'doc>,
    empty_is_less: bool,
  ) -> Option<i32> {
    if matches!(candidate, FormulaValue::Blank) {
      return Some(if empty_is_less { -1 } else { 1 });
    }
    self.compare_value(evaluator, candidate, false).or_else(|| {
      if self.expects_text() && query_candidate_number(candidate, false).is_some() {
        Some(-1)
      } else if self.expects_number() && matches!(candidate, FormulaValue::String(_)) {
        Some(1)
      } else {
        None
      }
    })
  }

  fn compare_by_range_lookup(&self, candidate: &FormulaValue<'_>) -> bool {
    match self.key.kind {
      QueryValueKind::Text if !matches!(self.op, QueryOp::Less | QueryOp::LessOrEqual) => false,
      QueryValueKind::Text => query_candidate_number(candidate, false).is_some(),
      _ if !matches!(self.op, QueryOp::Greater | QueryOp::GreaterOrEqual) => false,
      _ => query_candidate_number(candidate, false).is_none(),
    }
  }

  fn compare_value(
    &self,
    evaluator: &FormulaEvaluator<'_, 'doc>,
    candidate: &FormulaValue<'doc>,
    candidate_query_empty: bool,
  ) -> Option<i32> {
    if let Some((candidate, query)) = query_candidate_number(candidate, candidate_query_empty)
      .zip(evaluator.number(&self.key.value))
    {
      return Some(compare_numbers(candidate, query));
    }
    match (candidate, &self.key.value) {
      (FormulaValue::String(candidate), FormulaValue::String(query)) => Some(compare_text(
        evaluator,
        candidate,
        query,
        self.case_sensitive,
      )),
      (FormulaValue::Blank, FormulaValue::Blank) => Some(0),
      (FormulaValue::Blank, _) if self.range_lookup => Some(-1),
      (_, FormulaValue::Blank) if self.range_lookup => Some(1),
      (FormulaValue::Number(_), FormulaValue::String(_)) if self.range_lookup => None,
      (FormulaValue::String(_), FormulaValue::Number(_)) if self.range_lookup => None,
      (FormulaValue::Boolean(left), FormulaValue::Boolean(right)) => Some(match left.cmp(right) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
      }),
      _ => None,
    }
  }

  fn compare_ordering(&self, ordering: Option<i32>) -> bool {
    match self.op {
      QueryOp::Equal => ordering == Some(0),
      QueryOp::NotEqual => ordering != Some(0),
      QueryOp::Less => ordering == Some(-1),
      QueryOp::LessOrEqual => matches!(ordering, Some(-1 | 0)),
      QueryOp::Greater => ordering == Some(1),
      QueryOp::GreaterOrEqual => matches!(ordering, Some(0 | 1)),
    }
  }
}

pub(crate) fn query_candidate_number(value: &FormulaValue<'_>, query_empty: bool) -> Option<f64> {
  if query_empty {
    return Some(0.0);
  }
  match value {
    FormulaValue::Number(value) => Some(*value),
    FormulaValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
    _ => None,
  }
}

fn criteria_candidate_text_number(value: &FormulaValue<'_>) -> Option<f64> {
  let FormulaValue::String(text) = value else {
    return None;
  };
  text.trim().parse::<f64>().ok()
}

fn criteria_candidate_number(value: &FormulaValue<'_>, parse_text: bool) -> Option<f64> {
  match value {
    FormulaValue::Number(value) => Some(*value),
    FormulaValue::String(_) if parse_text => criteria_candidate_text_number(value),
    _ => None,
  }
}

fn criteria_error_value(value: &str) -> Option<FormulaErrorValue> {
  match value.trim().to_ascii_uppercase().as_str() {
    "#NULL!" => Some(FormulaErrorValue::Null),
    "#DIV/0!" => Some(FormulaErrorValue::Div0),
    "#VALUE!" => Some(FormulaErrorValue::Value),
    "#REF!" => Some(FormulaErrorValue::Ref),
    "#NAME?" => Some(FormulaErrorValue::Name),
    "#NUM!" => Some(FormulaErrorValue::Num),
    "#N/A" => Some(FormulaErrorValue::NA),
    "#GETTING_DATA" => Some(FormulaErrorValue::GettingData),
    "#SPILL!" => Some(FormulaErrorValue::Spill),
    "#CALC!" => Some(FormulaErrorValue::Calc),
    _ => None,
  }
}

fn formula_error_criteria_code(value: FormulaErrorValue) -> i32 {
  match value {
    FormulaErrorValue::Null => 0,
    FormulaErrorValue::Div0 => 7,
    FormulaErrorValue::Value => 15,
    FormulaErrorValue::Ref => 23,
    FormulaErrorValue::Name => 29,
    FormulaErrorValue::Num => 36,
    FormulaErrorValue::NA => 42,
    FormulaErrorValue::GettingData => 43,
    FormulaErrorValue::Spill => 44,
    FormulaErrorValue::Calc => 45,
    FormulaErrorValue::IllegalArgument => 502,
    FormulaErrorValue::Parameter => 511,
    FormulaErrorValue::Unknown => i32::MAX,
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

pub(crate) fn database_criterion_cell_present(value: &FormulaValue<'_>, query_empty: bool) -> bool {
  query_empty || !matches!(value, FormulaValue::Blank)
}

pub(crate) fn database_criterion_entry_present(
  value: &FormulaValue<'_>,
  query_empty: bool,
) -> bool {
  database_criterion_cell_present(value, query_empty)
    && !matches!(value, FormulaValue::String(text) if text.is_empty())
}

pub(crate) fn compare_numbers(left: f64, right: f64) -> i32 {
  if approx_equal(left, right) {
    0
  } else if left < right {
    -1
  } else {
    1
  }
}

pub(crate) fn compare_text(
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
