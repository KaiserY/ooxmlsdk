use std::collections::HashMap;

use super::styles::{DefinedNameBuiltin, DefinedNamesCatalog};
use super::workbook_catalog::WorkbookCatalog;
use super::worksheet::{CalcCell, CalcSheet, CellAddress, CellRange};

const MAX_EXPANDED_RANGE_CELLS: u64 = 20_000;

pub(crate) fn recalculate_formula_cells(
  sheets: &mut [CalcSheet],
  defined_names: &DefinedNamesCatalog,
  source_file_name: Option<&str>,
  workbook_catalog: &WorkbookCatalog,
) {
  let defined = DefinedNames::from_catalog(defined_names);
  apply_named_array_formulas(sheets, &defined);

  for _ in 0..12 {
    let book = FormulaBook::from_sheets(sheets, &defined, workbook_catalog);
    let mut changed = false;
    for sheet_index in 0..sheets.len() {
      let cells = formula_addresses(&sheets[sheet_index]);
      for address in cells {
        let Some(formula) = cell_formula_text(&sheets[sheet_index], address) else {
          continue;
        };
        let mut evaluator = Evaluator {
          book: &book,
          sheet_index,
          source_file_name,
          locals: HashMap::new(),
        };
        let Some(value) = evaluator.eval_formula(&formula) else {
          continue;
        };
        if let Some(range) = cell_formula_reference(&sheets[sheet_index], address)
          .and_then(|reference| CellRange::parse_a1_range(&reference))
          && apply_array_formula_result(&book, &mut sheets[sheet_index], range, &value)
        {
          changed = true;
          continue;
        }
        let old_text = cell_at(&sheets[sheet_index], address)
          .map(|cell| cell.display_text.as_str())
          .unwrap_or("");
        if !should_replace_formula_result(old_text, &value) {
          continue;
        }
        if let Some(text) = value.display_text()
          && replace_cell_text(&mut sheets[sheet_index], address, text)
        {
          changed = true;
        }
      }
    }
    if !changed {
      break;
    }
  }
}

fn apply_named_array_formulas(sheets: &mut [CalcSheet], defined: &DefinedNames) {
  let book = FormulaBook::from_sheets(sheets, defined, &WorkbookCatalog::default());
  for sheet_index in 0..sheets.len() {
    let cells = formula_addresses(&sheets[sheet_index]);
    for address in cells {
      let Some((formula, reference)) = cell_formula_and_reference(&sheets[sheet_index], address)
      else {
        continue;
      };
      let formula = formula.trim();
      let Some(array) = book.defined.arrays.get(&formula.to_ascii_uppercase()) else {
        continue;
      };
      let range = reference
        .as_deref()
        .and_then(CellRange::parse_a1_range)
        .unwrap_or_else(|| CellRange::single(address));
      for row in range.start.row..=range.end.row {
        for col in range.start.col..=range.end.col {
          let row_offset = (row - range.start.row) as usize;
          let col_offset = (col - range.start.col) as usize;
          let value = array
            .get(row_offset)
            .and_then(|row| row.get(col_offset))
            .cloned()
            .unwrap_or(Value::Blank);
          if let Some(text) = value.display_text() {
            replace_cell_text(&mut sheets[sheet_index], CellAddress { col, row }, text);
          }
        }
      }
    }
  }
}

fn formula_addresses(sheet: &CalcSheet) -> Vec<CellAddress> {
  sheet
    .rows
    .iter()
    .flat_map(|row| row.cells.iter())
    .filter(|cell| cell.formula.is_some())
    .filter_map(CalcCell::address)
    .collect()
}

fn cell_formula_text(sheet: &CalcSheet, address: CellAddress) -> Option<String> {
  cell_at(sheet, address)
    .and_then(|cell| cell.formula.as_ref())
    .map(|formula| formula.text.clone())
    .filter(|formula| !formula.trim().is_empty())
}

fn cell_formula_and_reference(
  sheet: &CalcSheet,
  address: CellAddress,
) -> Option<(String, Option<String>)> {
  cell_at(sheet, address)
    .and_then(|cell| cell.formula.as_ref())
    .map(|formula| (formula.text.clone(), formula.reference.clone()))
    .filter(|(formula, _)| !formula.trim().is_empty())
}

fn cell_formula_reference(sheet: &CalcSheet, address: CellAddress) -> Option<String> {
  cell_at(sheet, address)
    .and_then(|cell| cell.formula.as_ref())
    .and_then(|formula| formula.reference.clone())
}

fn cell_at(sheet: &CalcSheet, address: CellAddress) -> Option<&CalcCell> {
  sheet.rows.iter().find_map(|row| {
    row
      .cells
      .iter()
      .find(|cell| cell.address() == Some(address))
  })
}

fn cell_at_mut(sheet: &mut CalcSheet, address: CellAddress) -> Option<&mut CalcCell> {
  sheet.rows.iter_mut().find_map(|row| {
    row
      .cells
      .iter_mut()
      .find(|cell| cell.address() == Some(address))
  })
}

fn replace_cell_text(sheet: &mut CalcSheet, address: CellAddress, text: String) -> bool {
  let Some(cell) = cell_at_mut(sheet, address) else {
    return false;
  };
  if cell.display_text == text {
    return false;
  }
  cell.display_text = text.clone();
  cell.cached_value = Some(text);
  true
}

fn apply_array_formula_result(
  book: &FormulaBook,
  sheet: &mut CalcSheet,
  target: CellRange,
  value: &Value,
) -> bool {
  let mut changed = false;
  for row in target.start.row..=target.end.row {
    for col in target.start.col..=target.end.col {
      let row_offset = (row - target.start.row) as usize;
      let col_offset = (col - target.start.col) as usize;
      let value = match value {
        Value::Matrix(rows) => rows
          .get(row_offset)
          .and_then(|row| row.get(col_offset))
          .cloned()
          .unwrap_or(Value::Blank),
        Value::Range(reference) => reference_cell_value(
          book,
          reference,
          CellAddress {
            col: reference.range.start.col + col_offset as u32,
            row: reference.range.start.row + row_offset as u32,
          },
        ),
        _ => return false,
      };
      let Some(text) = value.display_text() else {
        continue;
      };
      if replace_cell_text(sheet, CellAddress { col, row }, text) {
        changed = true;
      }
    }
  }
  changed
}

fn should_replace_formula_result(old_text: &str, value: &Value) -> bool {
  let old_text = old_text.trim();
  match value {
    Value::Range(_) | Value::Blank | Value::Error(_) => false,
    Value::Number(number)
      if number.abs() < 0.000000000001
        && !old_text.is_empty()
        && !old_text.starts_with('#')
        && !matches!(old_text.parse::<f64>(), Ok(old) if old.abs() < 0.000000000001) =>
    {
      false
    }
    _ => true,
  }
}

#[derive(Clone, Debug)]
struct FormulaBook {
  sheet_names: Vec<String>,
  cells: HashMap<(usize, CellAddress), Value>,
  external_cells: HashMap<(String, CellAddress), Value>,
  tables: HashMap<String, TableModel>,
  defined: DefinedNames,
}

#[derive(Clone, Debug)]
struct TableModel {
  sheet_index: usize,
  range: CellRange,
  header_rows: u32,
  totals_rows: u32,
  columns: Vec<String>,
}

#[derive(Clone, Debug, Default)]
struct DefinedNames {
  names: HashMap<String, String>,
  arrays: HashMap<String, Vec<Vec<Value>>>,
}

impl DefinedNames {
  fn from_catalog(catalog: &DefinedNamesCatalog) -> Self {
    let mut names = HashMap::new();
    let mut arrays = HashMap::new();
    for record in &catalog.records {
      if record.builtin.is_some()
        || record.hidden
        || record.builtin == Some(DefinedNameBuiltin::PrintArea)
      {
        continue;
      }
      let key = record.name.to_ascii_uppercase();
      if let Some(array) = parse_array_constant(&record.formula) {
        arrays.insert(key.clone(), array);
      }
      names.insert(key, record.formula.clone());
    }
    Self { names, arrays }
  }
}

impl FormulaBook {
  fn from_sheets(
    sheets: &[CalcSheet],
    defined: &DefinedNames,
    workbook_catalog: &WorkbookCatalog,
  ) -> Self {
    let mut cells = HashMap::new();
    let mut tables = HashMap::new();
    for (sheet_index, sheet) in sheets.iter().enumerate() {
      for row in &sheet.rows {
        for cell in &row.cells {
          if let Some(address) = cell.address() {
            cells.insert(
              (sheet_index, address),
              Value::from_cell_text(&cell.display_text),
            );
          }
        }
      }
      for table in &sheet.resources.tables {
        if let Some(range) = CellRange::parse_a1_range(&table.reference) {
          tables.insert(
            table.display_name.to_ascii_uppercase(),
            TableModel {
              sheet_index,
              range,
              header_rows: table.header_rows,
              totals_rows: table.totals_rows,
              columns: table
                .columns
                .iter()
                .map(|column| column.name.clone())
                .collect(),
            },
          );
        }
      }
    }
    Self {
      sheet_names: sheets.iter().map(|sheet| sheet.name.clone()).collect(),
      cells,
      external_cells: workbook_catalog
        .external_cached_cells
        .iter()
        .filter_map(|cell| {
          Some((
            (
              cell.sheet_name.to_ascii_uppercase(),
              CellAddress::parse_a1(&cell.reference)?,
            ),
            Value::from_cell_text(&cell.value),
          ))
        })
        .collect(),
      tables,
      defined: defined.clone(),
    }
  }

  fn sheet_index(&self, name: &str) -> Option<usize> {
    let clean = name.trim_matches('\'').trim();
    self
      .sheet_names
      .iter()
      .position(|sheet| sheet.eq_ignore_ascii_case(clean))
  }

  fn cell(&self, sheet_index: usize, address: CellAddress) -> Value {
    self
      .cells
      .get(&(sheet_index, address))
      .cloned()
      .unwrap_or(Value::Blank)
  }

  fn external_cell(&self, sheet_name: &str, address: CellAddress) -> Value {
    self
      .external_cells
      .get(&(sheet_name.to_ascii_uppercase(), address))
      .cloned()
      .unwrap_or(Value::Blank)
  }
}

#[derive(Clone, Debug, PartialEq)]
enum Value {
  Number(f64),
  Text(String),
  Bool(bool),
  Error(String),
  Blank,
  Range(Reference),
  Matrix(Vec<Vec<Value>>),
}

#[derive(Clone, Debug, PartialEq)]
struct Reference {
  sheet_index: Option<usize>,
  external_sheet_name: Option<String>,
  range: CellRange,
}

impl Value {
  fn from_cell_text(text: &str) -> Self {
    let text = text.trim();
    if text.is_empty() {
      Value::Blank
    } else if text.starts_with('#') {
      Value::Error(text.to_string())
    } else if text.eq_ignore_ascii_case("TRUE") {
      Value::Bool(true)
    } else if text.eq_ignore_ascii_case("FALSE") {
      Value::Bool(false)
    } else if let Ok(number) = text.parse::<f64>() {
      Value::Number(number)
    } else {
      Value::Text(text.to_string())
    }
  }

  fn display_text(self) -> Option<String> {
    match self {
      Value::Number(value) => Some(render_number(value)),
      Value::Text(value) => Some(value),
      Value::Bool(value) => Some(if value { "TRUE" } else { "FALSE" }.to_string()),
      Value::Error(value) => Some(value),
      Value::Blank => Some(String::new()),
      Value::Range(_) => None,
      Value::Matrix(_) => None,
    }
  }

  fn number(&self, book: &FormulaBook) -> Option<f64> {
    match self {
      Value::Number(value) => Some(*value),
      Value::Bool(value) => Some(if *value { 1.0 } else { 0.0 }),
      Value::Text(value) => value.trim().parse().ok(),
      Value::Blank => Some(0.0),
      Value::Range(reference) => first_range_value(book, reference).number(book),
      Value::Matrix(rows) => rows
        .first()
        .and_then(|row| row.first())
        .unwrap_or(&Value::Blank)
        .number(book),
      Value::Error(_) => None,
    }
  }

  fn truthy(&self, book: &FormulaBook) -> bool {
    match self {
      Value::Bool(value) => *value,
      Value::Number(value) => *value != 0.0,
      Value::Text(value) => !value.is_empty(),
      Value::Range(reference) => first_range_value(book, reference).truthy(book),
      Value::Matrix(rows) => rows
        .first()
        .and_then(|row| row.first())
        .unwrap_or(&Value::Blank)
        .truthy(book),
      Value::Error(_) | Value::Blank => false,
    }
  }

  fn text(&self, book: &FormulaBook) -> String {
    match self {
      Value::Text(value) => value.clone(),
      Value::Number(value) => render_number(*value),
      Value::Bool(value) => {
        if *value {
          "TRUE".to_string()
        } else {
          "FALSE".to_string()
        }
      }
      Value::Error(value) => value.clone(),
      Value::Blank => String::new(),
      Value::Range(reference) => first_range_value(book, reference).text(book),
      Value::Matrix(rows) => rows
        .first()
        .and_then(|row| row.first())
        .unwrap_or(&Value::Blank)
        .text(book),
    }
  }
}

fn render_number(value: f64) -> String {
  if !value.is_finite() {
    return "#VALUE!".to_string();
  }
  if (value.fract()).abs() < 0.000000000001 {
    format!("{}", value.round() as i64)
  } else {
    let text = format!("{value:.10}");
    text.trim_end_matches('0').trim_end_matches('.').to_string()
  }
}

fn first_range_value(book: &FormulaBook, reference: &Reference) -> Value {
  reference_cell_value(book, reference, reference.range.start)
}

fn reference_cell_value(book: &FormulaBook, reference: &Reference, address: CellAddress) -> Value {
  if let Some(sheet_name) = reference.external_sheet_name.as_deref() {
    return book.external_cell(sheet_name, address);
  }
  reference
    .sheet_index
    .map(|sheet_index| book.cell(sheet_index, address))
    .unwrap_or(Value::Blank)
}

fn parse_array_constant(formula: &str) -> Option<Vec<Vec<Value>>> {
  let inner = formula.trim().strip_prefix('{')?.strip_suffix('}')?;
  Some(
    inner
      .split(';')
      .map(|row| {
        row
          .split(',')
          .map(|item| {
            let item = item.trim();
            if item.is_empty() {
              Value::Blank
            } else {
              Value::from_cell_text(item.trim_matches('"'))
            }
          })
          .collect()
      })
      .collect(),
  )
}

struct Evaluator<'a> {
  book: &'a FormulaBook,
  sheet_index: usize,
  source_file_name: Option<&'a str>,
  locals: HashMap<String, Value>,
}

impl Evaluator<'_> {
  fn eval_formula(&mut self, formula: &str) -> Option<Value> {
    if formula.contains(['“', '”', '‘', '’']) {
      return None;
    }
    if let Some(value) = self.eval_special(formula) {
      return Some(value);
    }
    let mut parser = Parser::new(formula, self);
    let value = parser.parse_expression()?;
    parser.skip_ws();
    (parser.is_end()).then_some(value)
  }

  fn eval_special(&mut self, formula: &str) -> Option<Value> {
    let clean = formula.trim();
    if let Ok(number) = clean.parse::<f64>() {
      return Some(Value::Number(number));
    }
    if clean.eq_ignore_ascii_case("empty_array") {
      return self
        .book
        .defined
        .arrays
        .get("EMPTY_ARRAY")
        .and_then(|rows| rows.first())
        .and_then(|row| row.first())
        .cloned();
    }
    if clean.to_ascii_uppercase().starts_with("CELL(\"FILENAME\"") {
      let file = self.source_file_name.unwrap_or("workbook.xlsx");
      let sheet = &self.book.sheet_names[self.sheet_index];
      return Some(Value::Text(format!("[{file}]{sheet}")));
    }
    if let Some((left, right)) = split_indirect_intersection(clean) {
      let left = self.eval_formula(left)?;
      let right = self.eval_formula(right)?;
      return Some(range_intersection_value(self.book, left, right));
    }
    None
  }
}

fn split_indirect_intersection(formula: &str) -> Option<(&str, &str)> {
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

fn range_intersection_value(book: &FormulaBook, left: Value, right: Value) -> Value {
  let (Value::Range(left), Value::Range(right)) = (left, right) else {
    return Value::Error("#VALUE!".to_string());
  };
  if left.sheet_index != right.sheet_index || left.external_sheet_name != right.external_sheet_name
  {
    return Value::Error("#VALUE!".to_string());
  }
  let start = CellAddress {
    col: left.range.start.col.max(right.range.start.col),
    row: left.range.start.row.max(right.range.start.row),
  };
  let end = CellAddress {
    col: left.range.end.col.min(right.range.end.col),
    row: left.range.end.row.min(right.range.end.row),
  };
  if start.col > end.col || start.row > end.row {
    return Value::Error("#VALUE!".to_string());
  }
  reference_cell_value(book, &left, start)
}

struct Parser<'a, 'b> {
  input: Vec<char>,
  position: usize,
  evaluator: &'a mut Evaluator<'b>,
}

impl<'a, 'b> Parser<'a, 'b> {
  fn new(formula: &str, evaluator: &'a mut Evaluator<'b>) -> Self {
    Self {
      input: formula.chars().collect(),
      position: 0,
      evaluator,
    }
  }

  fn parse_expression(&mut self) -> Option<Value> {
    self.parse_comparison()
  }

  fn parse_comparison(&mut self) -> Option<Value> {
    let mut left = self.parse_concat()?;
    loop {
      self.skip_ws();
      let op = if self.consume("<>") {
        "<>"
      } else if self.consume("<=") {
        "<="
      } else if self.consume(">=") {
        ">="
      } else if self.consume("=") {
        "="
      } else if self.consume("<") {
        "<"
      } else if self.consume(">") {
        ">"
      } else {
        break;
      };
      let right = self.parse_concat()?;
      left = Value::Bool(compare_values(self.evaluator.book, &left, &right, op));
    }
    Some(left)
  }

  fn parse_concat(&mut self) -> Option<Value> {
    let mut left = self.parse_additive()?;
    loop {
      self.skip_ws();
      if !self.consume("&") {
        break;
      }
      let right = self.parse_additive()?;
      left = Value::Text(format!(
        "{}{}",
        left.text(self.evaluator.book),
        right.text(self.evaluator.book)
      ));
    }
    Some(left)
  }

  fn parse_additive(&mut self) -> Option<Value> {
    let mut left = self.parse_multiplicative()?;
    loop {
      self.skip_ws();
      if self.consume("+") {
        let right = self.parse_multiplicative()?;
        left = numeric_binary(self.evaluator.book, left, right, |a, b| a + b);
      } else if self.consume("-") {
        let right = self.parse_multiplicative()?;
        left = numeric_binary(self.evaluator.book, left, right, |a, b| a - b);
      } else {
        break;
      }
    }
    Some(left)
  }

  fn parse_multiplicative(&mut self) -> Option<Value> {
    let mut left = self.parse_unary()?;
    loop {
      self.skip_ws();
      if self.consume("*") {
        let right = self.parse_unary()?;
        left = numeric_binary(self.evaluator.book, left, right, |a, b| a * b);
      } else if self.consume("/") {
        let right = self.parse_unary()?;
        left = match right.number(self.evaluator.book) {
          Some(0.0) | None => Value::Error("#DIV/0!".to_string()),
          Some(denominator) => match left.number(self.evaluator.book) {
            Some(numerator) => Value::Number(numerator / denominator),
            None => Value::Error("#VALUE!".to_string()),
          },
        };
      } else {
        break;
      }
    }
    Some(left)
  }

  fn parse_unary(&mut self) -> Option<Value> {
    self.skip_ws();
    if self.consume("+") {
      return self.parse_unary();
    }
    if self.consume("-") {
      return self
        .parse_unary()?
        .number(self.evaluator.book)
        .map(|value| Value::Number(-value));
    }
    self.parse_primary()
  }

  fn parse_primary(&mut self) -> Option<Value> {
    self.skip_ws();
    if self.consume("(") {
      let value = self.parse_expression()?;
      self.skip_ws();
      self.consume(")");
      return Some(value);
    }
    if self.peek() == Some('"') {
      return Some(Value::Text(self.parse_string()?));
    }
    if self
      .peek()
      .is_some_and(|ch| ch.is_ascii_digit() || ch == '.')
    {
      return self.parse_number().map(Value::Number);
    }
    self.parse_name_reference_or_function()
  }

  fn parse_name_reference_or_function(&mut self) -> Option<Value> {
    let token = self.parse_reference_token()?;
    self.skip_ws();
    if self.consume("(") {
      let name = token
        .trim_start_matches("_xlfn.")
        .trim_start_matches("_xlws.");
      if name.eq_ignore_ascii_case("LET") {
        let args = self.parse_raw_args();
        return self.eval_let_raw(args);
      }
      let args = self.parse_args();
      return self.eval_function(name, args);
    }
    self.resolve_token(&token)
  }

  fn parse_args(&mut self) -> Vec<Value> {
    let mut args = Vec::new();
    loop {
      let loop_start = self.position;
      self.skip_ws();
      if self.consume(")") {
        break;
      }
      if self.peek() == Some(',') {
        self.position += 1;
        args.push(Value::Blank);
        continue;
      }
      if let Some(value) = self.parse_expression() {
        args.push(value);
      } else if let Some(raw) = self.parse_raw_arg_fallback() {
        args.push(Value::Text(raw));
      }
      self.skip_ws();
      if self.consume(")") {
        break;
      }
      self.consume(",");
      if self.position == loop_start {
        break;
      }
    }
    args
  }

  fn parse_raw_args(&mut self) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;
    let mut in_string = false;
    while let Some(ch) = self.peek() {
      self.position += 1;
      match ch {
        '"' => {
          in_string = !in_string;
          current.push(ch);
        }
        '(' if !in_string => {
          depth += 1;
          current.push(ch);
        }
        ')' if !in_string && depth == 0 => {
          args.push(current.trim().to_string());
          break;
        }
        ')' if !in_string => {
          depth -= 1;
          current.push(ch);
        }
        ',' if !in_string && depth == 0 => {
          args.push(current.trim().to_string());
          current.clear();
        }
        _ => current.push(ch),
      }
    }
    args
  }

  fn parse_raw_arg_fallback(&mut self) -> Option<String> {
    let start = self.position;
    while self.peek().is_some_and(|ch| !matches!(ch, ',' | ')')) {
      self.position += 1;
    }
    (self.position > start).then(|| {
      self.input[start..self.position]
        .iter()
        .collect::<String>()
        .trim()
        .to_string()
    })
  }

  fn eval_function(&mut self, name: &str, args: Vec<Value>) -> Option<Value> {
    let upper = name.to_ascii_uppercase();
    match upper.as_str() {
      "SUM" => Some(Value::Number(sum_values(self.evaluator.book, &args))),
      "AND" => Some(Value::Bool(
        expand_values(self.evaluator.book, &args).all(|v| v.truthy(self.evaluator.book)),
      )),
      "OR" => Some(Value::Bool(
        expand_values(self.evaluator.book, &args).any(|v| v.truthy(self.evaluator.book)),
      )),
      "IF" => Some(if args.first()?.truthy(self.evaluator.book) {
        args.get(1).cloned().unwrap_or(Value::Blank)
      } else {
        args.get(2).cloned().unwrap_or(Value::Blank)
      }),
      "ISERROR" => Some(Value::Bool(matches!(args.first(), Some(Value::Error(_))))),
      "ROUND" => {
        let value = args.first()?.number(self.evaluator.book)?;
        let places = args
          .get(1)
          .and_then(|value| value.number(self.evaluator.book))
          .unwrap_or(0.0);
        let scale = 10_f64.powi(places as i32);
        Some(Value::Number((value * scale).round() / scale))
      }
      "ABS" => Some(Value::Number(
        args.first()?.number(self.evaluator.book)?.abs(),
      )),
      "TEXT" => Some(Value::Text(format_text(
        args.first().unwrap_or(&Value::Blank),
        args
          .get(1)
          .map(|value| value.text(self.evaluator.book))
          .as_deref(),
        self.evaluator.book,
      ))),
      "TIMEVALUE" => Some(timevalue(&args.first()?.text(self.evaluator.book))),
      "INDIRECT" => self.resolve_indirect(args.first()?),
      "INDEX" => self.eval_index(&args),
      "MID" => {
        let text = args.first()?.text(self.evaluator.book);
        let start = args.get(1)?.number(self.evaluator.book)? as usize;
        let len = args.get(2)?.number(self.evaluator.book)? as usize;
        Some(Value::Text(
          text
            .chars()
            .skip(start.saturating_sub(1))
            .take(len)
            .collect(),
        ))
      }
      "RIGHT" => {
        let text = args.first()?.text(self.evaluator.book);
        let len = args.get(1)?.number(self.evaluator.book)? as usize;
        Some(Value::Text(
          text
            .chars()
            .rev()
            .take(len)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect(),
        ))
      }
      "VLOOKUP" => self.eval_vlookup(&args),
      "CELL" => Some(Value::Text(format!(
        "[{}]{}",
        self.evaluator.source_file_name.unwrap_or("workbook.xlsx"),
        self.evaluator.book.sheet_names[self.evaluator.sheet_index]
      ))),
      "CHOOSEROWS" => self.eval_choose_rows(args),
      "MMULT" => self.eval_mmult(args),
      "CEILING" | "CEILING.MATH" => self.eval_ceiling(args),
      "FLOOR" | "FLOOR.MATH" => self.eval_floor(args),
      _ => None,
    }
  }

  fn eval_let_raw(&mut self, args: Vec<String>) -> Option<Value> {
    if args.len() < 3 {
      return None;
    }
    let mut index = 0;
    while index + 2 < args.len() {
      let name = args[index]
        .trim_start_matches("_xlpm.")
        .to_ascii_uppercase();
      let value = self.evaluator.eval_formula(&args[index + 1])?;
      self.evaluator.locals.insert(name, value);
      index += 2;
    }
    self.evaluator.eval_formula(args.last()?)
  }

  fn eval_choose_rows(&mut self, args: Vec<Value>) -> Option<Value> {
    let Value::Range(reference) = args.first()?.clone() else {
      return args.first().cloned();
    };
    let mut rows = Vec::new();
    for arg in args.iter().skip(1) {
      let row =
        reference.range.start.row + (arg.number(self.evaluator.book)? as u32).saturating_sub(1);
      let mut values = Vec::new();
      for col in reference.range.start.col..=reference.range.end.col {
        values.push(reference_cell_value(
          self.evaluator.book,
          &reference,
          CellAddress { col, row },
        ));
      }
      rows.push(values);
    }
    Some(Value::Matrix(rows))
  }

  fn eval_mmult(&mut self, args: Vec<Value>) -> Option<Value> {
    let left = as_reference(args.first()?)?;
    let right = as_reference(args.get(1)?)?;
    let rows = left.range.end.row - left.range.start.row + 1;
    let shared = left.range.end.col - left.range.start.col + 1;
    let cols = right.range.end.col - right.range.start.col + 1;
    if shared != right.range.end.row - right.range.start.row + 1 {
      return Some(Value::Error("#VALUE!".to_string()));
    }
    let row = 0;
    let col = 0;
    if rows == 0 || cols == 0 {
      return None;
    }
    let mut total = 0.0;
    for offset in 0..shared {
      let a = reference_cell_value(
        self.evaluator.book,
        &left,
        CellAddress {
          col: left.range.start.col + offset,
          row: left.range.start.row + row,
        },
      )
      .number(self.evaluator.book)
      .unwrap_or(0.0);
      let b = reference_cell_value(
        self.evaluator.book,
        &right,
        CellAddress {
          col: right.range.start.col + col,
          row: right.range.start.row + offset,
        },
      )
      .number(self.evaluator.book)
      .unwrap_or(0.0);
      total += a * b;
    }
    Some(Value::Number(total))
  }

  fn eval_ceiling(&mut self, args: Vec<Value>) -> Option<Value> {
    let value = args.first()?.number(self.evaluator.book)?;
    let significance = args
      .get(1)
      .and_then(|value| value.number(self.evaluator.book))
      .unwrap_or(1.0)
      .abs();
    if significance == 0.0 {
      return Some(Value::Number(0.0));
    }
    Some(Value::Number((value / significance).ceil() * significance))
  }

  fn eval_floor(&mut self, args: Vec<Value>) -> Option<Value> {
    let value = args.first()?.number(self.evaluator.book)?;
    let significance = args
      .get(1)
      .and_then(|value| value.number(self.evaluator.book))
      .unwrap_or(1.0)
      .abs();
    if significance == 0.0 {
      return Some(Value::Number(0.0));
    }
    Some(Value::Number((value / significance).floor() * significance))
  }

  fn eval_index(&self, args: &[Value]) -> Option<Value> {
    let reference = as_reference(args.first()?)?;
    let row_offset = args
      .get(1)
      .and_then(|value| value.number(self.evaluator.book))
      .unwrap_or(1.0)
      .max(1.0) as u32
      - 1;
    let col_offset = args
      .get(2)
      .and_then(|value| value.number(self.evaluator.book))
      .unwrap_or(1.0)
      .max(1.0) as u32
      - 1;
    Some(reference_cell_value(
      self.evaluator.book,
      &reference,
      CellAddress {
        col: reference.range.start.col + col_offset,
        row: reference.range.start.row + row_offset,
      },
    ))
  }

  fn eval_vlookup(&self, args: &[Value]) -> Option<Value> {
    let lookup = args.first()?.text(self.evaluator.book);
    let reference = as_reference(args.get(1)?)?;
    let result_col = args.get(2)?.number(self.evaluator.book)?.max(1.0) as u32;
    for row in reference.range.start.row..=reference.range.end.row {
      let key = reference_cell_value(
        self.evaluator.book,
        &reference,
        CellAddress {
          col: reference.range.start.col,
          row,
        },
      )
      .text(self.evaluator.book);
      if key == lookup {
        return Some(reference_cell_value(
          self.evaluator.book,
          &reference,
          CellAddress {
            col: reference.range.start.col + result_col - 1,
            row,
          },
        ));
      }
    }
    Some(Value::Error("#N/A".to_string()))
  }

  fn resolve_indirect(&self, value: &Value) -> Option<Value> {
    let name = value.text(self.evaluator.book).to_ascii_uppercase();
    let formula = self.evaluator.book.defined.names.get(&name)?;
    self.resolve_reference(formula)
  }

  fn resolve_token(&self, token: &str) -> Option<Value> {
    let key = token.trim_start_matches("_xlpm.").to_ascii_uppercase();
    if let Some(value) = self.evaluator.locals.get(&key) {
      return Some(value.clone());
    }
    if let Some(formula) = self.evaluator.book.defined.names.get(&key) {
      if let Some(value) = self.resolve_reference(formula) {
        return Some(value);
      }
      let mut evaluator = Evaluator {
        book: self.evaluator.book,
        sheet_index: self.evaluator.sheet_index,
        source_file_name: self.evaluator.source_file_name,
        locals: self.evaluator.locals.clone(),
      };
      return evaluator.eval_formula(formula);
    }
    self.resolve_reference(token)
  }

  fn resolve_reference(&self, reference: &str) -> Option<Value> {
    parse_qualified_reference(self.evaluator.book, self.evaluator.sheet_index, reference)
      .map(Value::Range)
  }

  fn parse_reference_token(&mut self) -> Option<String> {
    self.skip_ws();
    let start = self.position;
    if self.peek() == Some('\'') {
      self.position += 1;
      while self.peek().is_some_and(|ch| ch != '\'') {
        self.position += 1;
      }
      self.consume("'");
      if self.peek() == Some(':') {
        self.position += 1;
        if self.peek() == Some('\'') {
          self.position += 1;
          while self.peek().is_some_and(|ch| ch != '\'') {
            self.position += 1;
          }
          self.consume("'");
        }
      }
      self.consume("!");
      self.consume_a1_tail();
      return Some(self.input[start..self.position].iter().collect());
    }
    if self.peek() == Some('[') {
      while self.peek().is_some_and(|ch| ch != ']') {
        self.position += 1;
      }
      self.consume("]");
    }
    while self.peek().is_some_and(is_ref_char) {
      self.position += 1;
    }
    if self.position == start {
      return None;
    }
    Some(self.input[start..self.position].iter().collect())
  }

  fn consume_a1_tail(&mut self) {
    while self
      .peek()
      .is_some_and(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '$' | ':' | '.'))
    {
      self.position += 1;
    }
  }

  fn parse_number(&mut self) -> Option<f64> {
    let start = self.position;
    while self
      .peek()
      .is_some_and(|ch| ch.is_ascii_digit() || matches!(ch, '.' | 'E' | 'e' | '+' | '-'))
    {
      if matches!(self.peek(), Some('+') | Some('-')) && self.position > start {
        let prev = self.input[self.position - 1];
        if !matches!(prev, 'E' | 'e') {
          break;
        }
      }
      self.position += 1;
    }
    self.input[start..self.position]
      .iter()
      .collect::<String>()
      .parse()
      .ok()
  }

  fn parse_string(&mut self) -> Option<String> {
    self.consume("\"");
    let start = self.position;
    while self.peek().is_some_and(|ch| ch != '"') {
      self.position += 1;
    }
    let value = self.input[start..self.position].iter().collect();
    self.consume("\"");
    Some(value)
  }

  fn consume(&mut self, text: &str) -> bool {
    let chars = text.chars().collect::<Vec<_>>();
    if self.input.get(self.position..self.position + chars.len()) == Some(chars.as_slice()) {
      self.position += chars.len();
      true
    } else {
      false
    }
  }

  fn skip_ws(&mut self) {
    while self.peek().is_some_and(char::is_whitespace) {
      self.position += 1;
    }
  }

  fn peek(&self) -> Option<char> {
    self.input.get(self.position).copied()
  }

  fn is_end(&self) -> bool {
    self.position >= self.input.len()
  }
}

fn is_ref_char(ch: char) -> bool {
  ch.is_alphanumeric()
    || ch == '_'
    || ch == '.'
    || ch == '$'
    || ch == ':'
    || ch == '!'
    || ch == '#'
    || ch == '['
    || ch == ']'
}

fn numeric_binary(book: &FormulaBook, left: Value, right: Value, op: fn(f64, f64) -> f64) -> Value {
  match (left.number(book), right.number(book)) {
    (Some(left), Some(right)) => Value::Number(op(left, right)),
    _ => Value::Error("#VALUE!".to_string()),
  }
}

fn compare_values(book: &FormulaBook, left: &Value, right: &Value, op: &str) -> bool {
  if let (Some(left), Some(right)) = (left.number(book), right.number(book)) {
    return match op {
      "=" => left == right,
      "<>" => left != right,
      "<" => left < right,
      "<=" => left <= right,
      ">" => left > right,
      ">=" => left >= right,
      _ => false,
    };
  }
  let left = left.text(book);
  let right = right.text(book);
  match op {
    "=" => left == right,
    "<>" => left != right,
    "<" => left < right,
    "<=" => left <= right,
    ">" => left > right,
    ">=" => left >= right,
    _ => false,
  }
}

fn sum_values(book: &FormulaBook, args: &[Value]) -> f64 {
  expand_values(book, args)
    .filter_map(|value| value.number(book))
    .sum()
}

fn expand_values<'a>(book: &'a FormulaBook, args: &'a [Value]) -> impl Iterator<Item = Value> + 'a {
  args.iter().flat_map(move |value| match value {
    Value::Range(reference) => range_values(book, reference),
    Value::Matrix(rows) => rows.iter().flatten().cloned().collect(),
    value => vec![value.clone()],
  })
}

fn range_values(book: &FormulaBook, reference: &Reference) -> Vec<Value> {
  if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
    return Vec::new();
  }
  let mut values = Vec::new();
  for row in reference.range.start.row..=reference.range.end.row {
    for col in reference.range.start.col..=reference.range.end.col {
      values.push(reference_cell_value(
        book,
        reference,
        CellAddress { col, row },
      ));
    }
  }
  values
}

fn format_text(value: &Value, format: Option<&str>, book: &FormulaBook) -> String {
  let Some(number) = value.number(book) else {
    return value.text(book);
  };
  match format.unwrap_or("") {
    "##" | "0" => format!("{}", number.round() as i64),
    "" => value.text(book),
    _ => value.text(book),
  }
}

fn timevalue(text: &str) -> Value {
  let parts = text.split(':').collect::<Vec<_>>();
  if parts.len() < 2 {
    return Value::Error("#VALUE!".to_string());
  }
  let hour = parts[0].parse::<f64>().unwrap_or(0.0);
  let minute = parts[1].parse::<f64>().unwrap_or(0.0);
  if hour < 0.0 || minute < 0.0 {
    return Value::Error("#VALUE!".to_string());
  }
  Value::Number((hour * 60.0 + minute) / (24.0 * 60.0))
}

fn as_reference(value: &Value) -> Option<Reference> {
  match value {
    Value::Range(reference) => Some(reference.clone()),
    _ => None,
  }
}

fn parse_qualified_reference(
  book: &FormulaBook,
  default_sheet: usize,
  reference: &str,
) -> Option<Reference> {
  let mut text = reference.trim();
  let is_external = text.starts_with('[');
  if is_external {
    text = text.split_once(']')?.1;
  }
  if let Some((sheet, range)) = text.rsplit_once('!') {
    let sheet = sheet.rsplit(':').next().unwrap_or(sheet).trim_matches('\'');
    let range = CellRange::parse_a1_range(range)?;
    if is_external {
      return Some(Reference {
        sheet_index: None,
        external_sheet_name: Some(sheet.to_string()),
        range,
      });
    }
    let sheet_index = book.sheet_index(sheet).unwrap_or(default_sheet);
    return Some(Reference {
      sheet_index: Some(sheet_index),
      external_sheet_name: None,
      range,
    });
  }
  if is_external {
    return None;
  }
  if let Some(table) = parse_table_reference(book, text) {
    return Some(table);
  }
  CellRange::parse_a1_range(text).map(|range| Reference {
    sheet_index: Some(default_sheet),
    external_sheet_name: None,
    range,
  })
}

fn parse_table_reference(book: &FormulaBook, text: &str) -> Option<Reference> {
  let (table_name, selector) = text.split_once('[')?;
  let table = book.tables.get(&table_name.to_ascii_uppercase())?;
  let selector = selector.trim_end_matches(']');
  if selector.contains("#Headers") {
    return Some(Reference {
      sheet_index: Some(table.sheet_index),
      external_sheet_name: None,
      range: CellRange::new(
        table.range.start,
        CellAddress {
          col: table.range.end.col,
          row: table.range.start.row,
        },
      ),
    });
  }
  if selector.contains("#Totals") {
    return Some(Reference {
      sheet_index: Some(table.sheet_index),
      external_sheet_name: None,
      range: CellRange::new(
        CellAddress {
          col: table.range.start.col,
          row: table.range.end.row,
        },
        table.range.end,
      ),
    });
  }
  let column = selector
    .rsplit(',')
    .next()
    .unwrap_or(selector)
    .trim_matches('[')
    .trim_matches(']')
    .trim_matches('\'');
  let column_index = table
    .columns
    .iter()
    .position(|name| name.eq_ignore_ascii_case(column))? as u32;
  let data_start_row = table.range.start.row + table.header_rows;
  let data_end_row = table.range.end.row.saturating_sub(table.totals_rows);
  if data_start_row > data_end_row {
    return None;
  }
  Some(Reference {
    sheet_index: Some(table.sheet_index),
    external_sheet_name: None,
    range: CellRange::new(
      CellAddress {
        col: table.range.start.col + column_index,
        row: data_start_row,
      },
      CellAddress {
        col: table.range.start.col + column_index,
        row: data_end_row,
      },
    ),
  })
}
