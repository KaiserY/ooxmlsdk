use std::collections::{HashMap, HashSet};

use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use super::styles::{DefinedNameBuiltin, DefinedNamesCatalog};
use super::workbook_catalog::WorkbookCatalog;
use super::worksheet::{CalcCell, CalcSheet, CellAddress, CellRange};

const MAX_EXPANDED_RANGE_CELLS: u64 = 20_000;
const XLSX_MAX_COLUMN: u32 = 16_384;
const XLSX_MAX_ROW: u32 = 1_048_576;

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
    let mut sheet_index = 0;
    while sheet_index < sheets.len() {
      let formulas = formula_cells(&sheets[sheet_index]);
      for formula_cell in formulas {
        let mut evaluator = Evaluator {
          book: &book,
          sheet_index,
          source_file_name,
          current_address: Some(formula_cell.address),
          locals: HashMap::new(),
        };
        let Some(value) = evaluator.eval_formula(&formula_cell.formula) else {
          continue;
        };
        if let Some(range) = formula_cell
          .reference
          .as_deref()
          .and_then(CellRange::parse_a1_range)
          && apply_array_formula_result(&book, &mut sheets[sheet_index], range, &value)
        {
          changed = true;
          continue;
        }
        let old_text = cell_at(&sheets[sheet_index], formula_cell.address)
          .map(|cell| cell.display_text.as_str())
          .unwrap_or("");
        if !should_replace_formula_result(old_text, &value) {
          continue;
        }
        if replace_cell_value(&mut sheets[sheet_index], formula_cell.address, &value) {
          changed = true;
        }
      }
      sheet_index += 1;
    }
    if !changed {
      break;
    }
  }
}

pub(crate) fn evaluate_relative_formula_as_condition(
  import: &super::import::ExcelImport,
  sheet: &CalcSheet,
  formula: &str,
  base: CellAddress,
  address: CellAddress,
) -> bool {
  evaluate_relative_formula(import, sheet, formula, base, address)
    .is_some_and(|(book, value)| value.truthy(&book))
}

pub(crate) fn evaluate_relative_formula_as_number(
  import: &super::import::ExcelImport,
  sheet: &CalcSheet,
  formula: &str,
  base: CellAddress,
  address: CellAddress,
) -> Option<f64> {
  evaluate_relative_formula(import, sheet, formula, base, address)
    .and_then(|(book, value)| value.number(&book))
}

fn evaluate_relative_formula(
  import: &super::import::ExcelImport,
  sheet: &CalcSheet,
  formula: &str,
  base: CellAddress,
  address: CellAddress,
) -> Option<(FormulaBook, Value)> {
  // Source: LibreOffice sc/source/filter/oox/condformatbuffer.cxx imports
  // conditional-format formulas with the top-left cell of the conditional
  // range as base, and sc/source/core/data/conditio.cxx evaluates relative
  // references from the current cell position.
  let defined = DefinedNames::from_catalog(&import.defined_names);
  let book = FormulaBook::from_sheets(&import.sheets, &defined, &import.workbook_catalog);
  let sheet_index = import
    .sheets
    .iter()
    .position(|candidate| std::ptr::eq(candidate, sheet))
    .or_else(|| {
      import.sheets.iter().position(|candidate| {
        candidate.workbook_index == sheet.workbook_index && candidate.name == sheet.name
      })
    })?;
  let translated = translate_shared_formula(formula.trim(), base, address);
  let mut evaluator = Evaluator {
    book: &book,
    sheet_index,
    source_file_name: None,
    current_address: Some(address),
    locals: HashMap::new(),
  };
  let value = evaluator.eval_formula(&translated)?;
  Some((book, value))
}

#[derive(Clone, Debug)]
struct FormulaCell {
  address: CellAddress,
  formula: String,
  reference: Option<String>,
  is_array: bool,
}

#[derive(Clone, Debug)]
struct SharedFormula {
  origin: CellAddress,
  formula: String,
}

fn formula_cells(sheet: &CalcSheet) -> Vec<FormulaCell> {
  // Source: LibreOffice sc/source/filter/oox/formulabuffer.cxx
  // Shared formula masters are recorded by id first, and cells that carry only
  // the shared id are later materialized from the master tokens at their own
  // address.
  let mut shared = HashMap::<u32, SharedFormula>::new();
  for cell in sheet.rows.iter().flat_map(|row| row.cells.iter()) {
    let Some(address) = cell.address() else {
      continue;
    };
    let Some(formula) = cell.formula.as_ref() else {
      continue;
    };
    if let Some(shared_index) = formula.shared_index
      && !formula.text.trim().is_empty()
    {
      shared.insert(
        shared_index,
        SharedFormula {
          origin: address,
          formula: formula.text.clone(),
        },
      );
    }
  }

  sheet
    .rows
    .iter()
    .flat_map(|row| row.cells.iter())
    .filter_map(|cell| {
      let address = cell.address()?;
      let formula = cell.formula.as_ref()?;
      let text = if !formula.text.trim().is_empty() {
        formula.text.clone()
      } else {
        let shared = shared.get(&formula.shared_index?)?;
        translate_shared_formula(&shared.formula, shared.origin, address)
      };
      (!text.trim().is_empty()).then(|| FormulaCell {
        address,
        formula: text,
        reference: formula.reference.clone(),
        is_array: formula.formula_type == x::CellFormulaValues::Array,
      })
    })
    .collect()
}

fn apply_named_array_formulas(sheets: &mut [CalcSheet], defined: &DefinedNames) {
  let book = FormulaBook::from_sheets(sheets, defined, &WorkbookCatalog::default());
  let mut sheet_index = 0;
  while sheet_index < sheets.len() {
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
          replace_cell_value(&mut sheets[sheet_index], CellAddress { col, row }, &value);
        }
      }
    }
    sheet_index += 1;
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

fn cell_formula_and_reference(
  sheet: &CalcSheet,
  address: CellAddress,
) -> Option<(String, Option<String>)> {
  cell_at(sheet, address)
    .and_then(|cell| cell.formula.as_ref())
    .map(|formula| (formula.text.clone(), formula.reference.clone()))
    .filter(|(formula, _)| !formula.trim().is_empty())
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

fn replace_cell_value(sheet: &mut CalcSheet, address: CellAddress, value: &Value) -> bool {
  let Some(cell) = cell_at_mut(sheet, address) else {
    return false;
  };
  let Some(display_text) = value.clone().display_text() else {
    return false;
  };
  let cached_value = value.cached_text();
  let changed = cell.display_text != display_text || cell.cached_value != cached_value;
  if changed {
    cell.display_text = display_text;
    cell.cached_value = cached_value;
  }
  changed
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
      if replace_cell_value(sheet, CellAddress { col, row }, &value) {
        changed = true;
      }
    }
  }
  changed
}

fn should_replace_formula_result(old_text: &str, value: &Value) -> bool {
  let old_text = old_text.trim();
  match value {
    Value::Range(_) | Value::Blank => false,
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
  formulas: HashMap<(usize, CellAddress), FormulaText>,
  hidden_rows: HashSet<(usize, u32)>,
  filtered_rows: HashSet<(usize, u32)>,
  external_cells: HashMap<(usize, String, CellAddress), Value>,
  tables: HashMap<String, TableModel>,
  defined: DefinedNames,
}

#[derive(Clone, Debug)]
struct FormulaText {
  text: String,
  is_array: bool,
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
    let mut formulas = HashMap::new();
    let mut hidden_rows = HashSet::new();
    let mut filtered_rows = HashSet::new();
    let mut tables = HashMap::new();
    for (sheet_index, sheet) in sheets.iter().enumerate() {
      let filtered_range = sheet
        .metrics
        .settings
        .properties
        .filter_mode
        .then(|| {
          sheet
            .metrics
            .settings
            .auto_filter
            .as_ref()
            .and_then(|filter| filter.reference.as_deref())
            .and_then(CellRange::parse_a1_range)
        })
        .flatten();
      for (row_position, row) in sheet.rows.iter().enumerate() {
        let row_index = row.row_index.unwrap_or(row_position as u32 + 1);
        if row.hidden {
          hidden_rows.insert((sheet_index, row_index));
          if filtered_range
            .is_some_and(|range| row_index >= range.start.row && row_index <= range.end.row)
          {
            filtered_rows.insert((sheet_index, row_index));
          }
        }
        for cell in &row.cells {
          if let Some(address) = cell.address() {
            cells.insert((sheet_index, address), formula_cell_value(cell));
          }
        }
      }
      for formula in formula_cells(sheet) {
        formulas.insert(
          (sheet_index, formula.address),
          FormulaText {
            text: formula.formula,
            is_array: formula.is_array,
          },
        );
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
      formulas,
      hidden_rows,
      filtered_rows,
      external_cells: workbook_catalog
        .external_cached_cells
        .iter()
        .filter_map(|cell| {
          Some((
            (
              cell.link_index,
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

  fn external_cell(
    &self,
    link_index: Option<usize>,
    sheet_name: &str,
    address: CellAddress,
  ) -> Value {
    let Some(link_index) = link_index else {
      return Value::Blank;
    };
    self
      .external_cells
      .get(&(link_index, sheet_name.to_ascii_uppercase(), address))
      .cloned()
      .unwrap_or(Value::Blank)
  }

  fn formula_text(&self, sheet_index: usize, address: CellAddress) -> Option<String> {
    let formula = self.formulas.get(&(sheet_index, address))?;
    Some(if formula.is_array {
      format!("{{={}}}", formula.text)
    } else {
      format!("={}", formula.text)
    })
  }

  fn row_hidden(&self, sheet_index: usize, row: u32) -> bool {
    self.hidden_rows.contains(&(sheet_index, row))
  }

  fn row_filtered(&self, sheet_index: usize, row: u32) -> bool {
    self.filtered_rows.contains(&(sheet_index, row))
  }

  fn is_nested_aggregate(&self, sheet_index: usize, address: CellAddress) -> bool {
    self
      .formulas
      .get(&(sheet_index, address))
      .is_some_and(|formula| {
        let text = formula
          .text
          .trim_start()
          .trim_start_matches("_xlfn.")
          .to_ascii_uppercase();
        text.starts_with("SUBTOTAL(") || text.starts_with("AGGREGATE(")
      })
  }
}

fn formula_cell_value(cell: &CalcCell) -> Value {
  // Source: LibreOffice sc/source/filter/oox/worksheethelper.cxx and
  // shared-string import model: for t="s" the raw <v> is an SST index, while
  // cached numeric/formula values carry the actual scalar value.
  let text = match cell.data_type {
    Some(x::CellValues::SharedString | x::CellValues::InlineString) => &cell.display_text,
    _ => cell.cached_value.as_deref().unwrap_or(&cell.display_text),
  };
  Value::from_cell_text(text)
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
  external_link_index: Option<usize>,
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

  fn cached_text(&self) -> Option<String> {
    match self {
      Value::Number(value) => Some(value.to_string()),
      Value::Text(value) => Some(value.clone()),
      Value::Bool(value) => Some(if *value { "TRUE" } else { "FALSE" }.to_string()),
      Value::Error(value) => Some(value.clone()),
      Value::Blank => Some(String::new()),
      Value::Range(_) | Value::Matrix(_) => None,
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
    return book.external_cell(reference.external_link_index, sheet_name, address);
  }
  reference
    .sheet_index
    .map(|sheet_index| book.cell(sheet_index, address))
    .unwrap_or(Value::Blank)
}

fn translate_shared_formula(formula: &str, origin: CellAddress, target: CellAddress) -> String {
  let delta_col = target.col as i64 - origin.col as i64;
  let delta_row = target.row as i64 - origin.row as i64;
  if delta_col == 0 && delta_row == 0 {
    return formula.to_string();
  }

  let chars = formula.chars().collect::<Vec<_>>();
  let mut output = String::new();
  let mut index = 0;
  while index < chars.len() {
    match chars[index] {
      '"' => {
        let start = index;
        index += 1;
        while index < chars.len() {
          let ch = chars[index];
          index += 1;
          if ch == '"' {
            if chars.get(index) == Some(&'"') {
              index += 1;
              continue;
            }
            break;
          }
        }
        output.extend(chars[start..index].iter());
      }
      '\'' => {
        let start = index;
        index += 1;
        while index < chars.len() {
          let ch = chars[index];
          index += 1;
          if ch == '\'' {
            if chars.get(index) == Some(&'\'') {
              index += 1;
              continue;
            }
            break;
          }
        }
        if chars.get(index) == Some(&'!') {
          index += 1;
          while index < chars.len() && is_a1_tail_char(chars[index]) {
            index += 1;
          }
          let token = chars[start..index].iter().collect::<String>();
          output.push_str(&translate_reference_token(&token, delta_col, delta_row));
        } else {
          output.extend(chars[start..index].iter());
        }
      }
      ch if is_formula_token_start(ch) => {
        let start = index;
        index += 1;
        while index < chars.len() && is_formula_token_char(chars[index]) {
          index += 1;
        }
        let token = chars[start..index].iter().collect::<String>();
        output.push_str(&translate_reference_token(&token, delta_col, delta_row));
      }
      ch => {
        output.push(ch);
        index += 1;
      }
    }
  }
  output
}

fn is_formula_token_start(ch: char) -> bool {
  ch.is_ascii_alphabetic() || ch == '$' || ch == '[' || ch == '_'
}

fn is_formula_token_char(ch: char) -> bool {
  ch.is_ascii_alphanumeric() || matches!(ch, '$' | '.' | '_' | ':' | '!' | '[' | ']')
}

fn is_a1_tail_char(ch: char) -> bool {
  ch.is_ascii_alphanumeric() || matches!(ch, '$' | ':' | '.')
}

fn translate_reference_token(token: &str, delta_col: i64, delta_row: i64) -> String {
  if token.contains('[') && !token.starts_with('[') {
    return token.to_string();
  }
  let Some((prefix, range)) = split_reference_prefix(token) else {
    return token.to_string();
  };
  let Some(translated) = translate_a1_range(range, delta_col, delta_row) else {
    return token.to_string();
  };
  format!("{prefix}{translated}")
}

fn split_reference_prefix(token: &str) -> Option<(&str, &str)> {
  if let Some((prefix, range)) = token.rsplit_once('!') {
    return Some((&token[..prefix.len() + 1], range));
  }
  Some(("", token))
}

fn translate_a1_range(range: &str, delta_col: i64, delta_row: i64) -> Option<String> {
  let (start, end) = range.split_once(':').unwrap_or((range, ""));
  let start = translate_a1_reference(start, delta_col, delta_row)?;
  if end.is_empty() {
    return Some(start);
  }
  let end = translate_a1_reference(end, delta_col, delta_row)?;
  Some(format!("{start}:{end}"))
}

fn translate_a1_reference(reference: &str, delta_col: i64, delta_row: i64) -> Option<String> {
  let parsed = A1Reference::parse(reference)?;
  Some(parsed.translate(delta_col, delta_row).format())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct A1Reference {
  absolute_col: bool,
  col: u32,
  absolute_row: bool,
  row: u32,
}

impl A1Reference {
  fn parse(reference: &str) -> Option<Self> {
    let mut chars = reference.chars().peekable();
    let absolute_col = chars.next_if_eq(&'$').is_some();
    let mut col = 0u32;
    while chars.peek().is_some_and(|ch| ch.is_ascii_alphabetic()) {
      let ch = chars.next()?.to_ascii_uppercase();
      col = col
        .saturating_mul(26)
        .saturating_add(ch as u32 - 'A' as u32 + 1);
    }
    let absolute_row = chars.next_if_eq(&'$').is_some();
    let mut row = 0u32;
    while chars.peek().is_some_and(|ch| ch.is_ascii_digit()) {
      let ch = chars.next()?;
      row = row
        .saturating_mul(10)
        .saturating_add(ch as u32 - '0' as u32);
    }
    (col > 0 && row > 0 && chars.next().is_none()).then_some(Self {
      absolute_col,
      col,
      absolute_row,
      row,
    })
  }

  fn translate(self, delta_col: i64, delta_row: i64) -> Self {
    Self {
      absolute_col: self.absolute_col,
      col: if self.absolute_col {
        self.col
      } else {
        translate_index(self.col, delta_col)
      },
      absolute_row: self.absolute_row,
      row: if self.absolute_row {
        self.row
      } else {
        translate_index(self.row, delta_row)
      },
    }
  }

  fn format(self) -> String {
    format!(
      "{}{}{}{}",
      if self.absolute_col { "$" } else { "" },
      column_name(self.col),
      if self.absolute_row { "$" } else { "" },
      self.row
    )
  }
}

fn translate_index(index: u32, delta: i64) -> u32 {
  u32::try_from((index as i64 + delta).max(1)).unwrap_or(u32::MAX)
}

fn column_name(mut col: u32) -> String {
  let mut chars = Vec::new();
  while col > 0 {
    col -= 1;
    chars.push(char::from_u32('A' as u32 + col % 26).unwrap_or('A'));
    col /= 26;
  }
  chars.into_iter().rev().collect()
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
  current_address: Option<CellAddress>,
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
    if is_formula_error_literal(clean) {
      return Some(Value::Error(clean.to_string()));
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
  if left.sheet_index != right.sheet_index
    || left.external_link_index != right.external_link_index
    || left.external_sheet_name != right.external_sheet_name
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

fn is_formula_error_literal(value: &str) -> bool {
  matches!(
    value,
    "#NULL!"
      | "#DIV/0!"
      | "#VALUE!"
      | "#REF!"
      | "#NAME?"
      | "#NUM!"
      | "#N/A"
      | "#GETTING_DATA"
      | "Err:502"
  )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CeilingFloorKind {
  Odff,
  Math,
  Precise,
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
      if name.eq_ignore_ascii_case("IF") {
        let args = self.parse_raw_args();
        return self.eval_if_raw(args);
      }
      if name.eq_ignore_ascii_case("IFERROR") || name.eq_ignore_ascii_case("IFNA") {
        let args = self.parse_raw_args();
        return self.eval_if_error_raw(args, name.eq_ignore_ascii_case("IFNA"));
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
    let mut array_depth = 0i32;
    let mut in_string = false;
    let mut in_sheet_name = false;
    let mut in_external_name = false;
    while let Some(ch) = self.peek() {
      self.position += 1;
      match ch {
        '"' => {
          current.push(ch);
          if in_string && self.peek() == Some('"') {
            current.push('"');
            self.position += 1;
          } else if !in_sheet_name && !in_external_name {
            in_string = !in_string;
          }
        }
        '\'' if !in_string && !in_external_name => {
          current.push(ch);
          if in_sheet_name && self.peek() == Some('\'') {
            current.push('\'');
            self.position += 1;
          } else {
            in_sheet_name = !in_sheet_name;
          }
        }
        '[' if !in_string && !in_sheet_name => {
          in_external_name = true;
          current.push(ch);
        }
        ']' if !in_string && !in_sheet_name => {
          in_external_name = false;
          current.push(ch);
        }
        '(' if !in_string && !in_sheet_name && !in_external_name => {
          depth += 1;
          current.push(ch);
        }
        ')' if !in_string && !in_sheet_name && !in_external_name && depth == 0 => {
          args.push(current.trim().to_string());
          break;
        }
        ')' if !in_string && !in_sheet_name && !in_external_name => {
          depth -= 1;
          current.push(ch);
        }
        '{' if !in_string && !in_sheet_name && !in_external_name => {
          array_depth += 1;
          current.push(ch);
        }
        '}' if !in_string && !in_sheet_name && !in_external_name && array_depth > 0 => {
          array_depth -= 1;
          current.push(ch);
        }
        ','
          if !in_string
            && !in_sheet_name
            && !in_external_name
            && depth == 0
            && array_depth == 0 =>
        {
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
      "ISERROR" => Some(Value::Bool(matches!(args.first(), Some(Value::Error(_))))),
      "ISNA" => Some(Value::Bool(matches!(
        args.first(),
        Some(Value::Error(error)) if error == "#N/A"
      ))),
      "ROUND" => {
        let value = args.first()?.number(self.evaluator.book)?;
        let places = args
          .get(1)
          .and_then(|value| value.number(self.evaluator.book))
          .unwrap_or(0.0);
        Some(Value::Number(rtl_round(value, approx_floor(places) as i32)))
      }
      "ABS" => Some(Value::Number(
        args.first()?.number(self.evaluator.book)?.abs(),
      )),
      "AVERAGE" => average_values(self.evaluator.book, &args).map(Value::Number),
      "COUNT" => Some(Value::Number(
        numeric_values(self.evaluator.book, &args).len() as f64,
      )),
      "COUNTA" => Some(Value::Number(
        expand_values(self.evaluator.book, &args)
          .filter(|value| !matches!(value, Value::Blank))
          .count() as f64,
      )),
      "MAX" => numeric_values(self.evaluator.book, &args)
        .into_iter()
        .reduce(f64::max)
        .map(Value::Number),
      "MIN" => numeric_values(self.evaluator.book, &args)
        .into_iter()
        .reduce(f64::min)
        .map(Value::Number),
      "PRODUCT" => Some(Value::Number(
        numeric_values(self.evaluator.book, &args)
          .into_iter()
          .product(),
      )),
      "MEDIAN" => {
        percentile_values(self.evaluator.book, &args, 0.5, PercentileKind::Inc).map(Value::Number)
      }
      "STDEV.P" | "STDEVP" => {
        variance_values(self.evaluator.book, &args, false).map(|value| Value::Number(value.sqrt()))
      }
      "STDEV.S" | "STDEV" => {
        variance_values(self.evaluator.book, &args, true).map(|value| Value::Number(value.sqrt()))
      }
      "VAR.P" | "VARP" => variance_values(self.evaluator.book, &args, false).map(Value::Number),
      "VAR.S" | "VAR" => variance_values(self.evaluator.book, &args, true).map(Value::Number),
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
      "OFFSET" => self.eval_offset(&args),
      "SUBTOTAL" => self.eval_subtotal(&args),
      "AGGREGATE" => self.eval_aggregate(&args),
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
      "LEFT" => {
        let text = args.first()?.text(self.evaluator.book);
        let len = args
          .get(1)
          .and_then(|value| value.number(self.evaluator.book))
          .unwrap_or(1.0) as usize;
        Some(Value::Text(text.chars().take(len).collect()))
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
      "FORMULATEXT" => self.eval_formula_text(&args),
      "CELL" => Some(Value::Text(format!(
        "[{}]{}",
        self.evaluator.source_file_name.unwrap_or("workbook.xlsx"),
        self.evaluator.book.sheet_names[self.evaluator.sheet_index]
      ))),
      "CHOOSEROWS" => self.eval_choose_rows(args),
      "MMULT" => self.eval_mmult(args),
      "CEILING" => self.eval_ceiling(args, CeilingFloorKind::Odff),
      "CEILING.MATH" => self.eval_ceiling(args, CeilingFloorKind::Math),
      "CEILING.PRECISE" | "ISO.CEILING" => self.eval_ceiling(args, CeilingFloorKind::Precise),
      "FLOOR" => self.eval_floor(args, CeilingFloorKind::Odff),
      "FLOOR.MATH" => self.eval_floor(args, CeilingFloorKind::Math),
      "FLOOR.PRECISE" => self.eval_floor(args, CeilingFloorKind::Precise),
      "BETA.DIST" => self.eval_beta_dist(&args),
      "BETA.INV" => self.eval_beta_inv(&args),
      "BINOM.DIST" => self.eval_binom_dist(&args),
      "BINOM.INV" => self.eval_binom_inv(&args),
      "CHISQ.DIST" => self.eval_chisq_dist(&args, false),
      "CHISQ.DIST.RT" => self.eval_chisq_dist(&args, true),
      "CHISQ.INV" => self.eval_chisq_inv(&args, false),
      "CHISQ.INV.RT" => self.eval_chisq_inv(&args, true),
      "CHISQ.TEST" | "CHITEST" => self.eval_chisq_test(&args),
      "CONFIDENCE.NORM" => self.eval_confidence_norm(&args),
      "CONFIDENCE.T" => self.eval_confidence_t(&args),
      "COVARIANCE.P" => self.eval_covariance(&args, false),
      "COVARIANCE.S" => self.eval_covariance(&args, true),
      "ERF.PRECISE" | "ERF" => args
        .first()?
        .number(self.evaluator.book)
        .map(|value| Value::Number(erf(value))),
      "ERFC.PRECISE" | "ERFC" => args
        .first()?
        .number(self.evaluator.book)
        .map(|value| Value::Number(erfc(value))),
      "EXPON.DIST" => self.eval_expon_dist(&args),
      "F.DIST" => self.eval_f_dist(&args),
      "F.DIST.RT" => self.eval_f_dist_rt(&args),
      "F.INV" => self.eval_f_inv(&args, false),
      "F.INV.RT" => self.eval_f_inv(&args, true),
      "F.TEST" | "FTEST" => self.eval_f_test(&args),
      "GAMMA.DIST" => self.eval_gamma_dist(&args),
      "GAMMA.INV" => self.eval_gamma_inv(&args),
      "GAMMALN.PRECISE" | "GAMMALN" => args
        .first()?
        .number(self.evaluator.book)
        .filter(|value| *value > 0.0)
        .map(|value| Value::Number(log_gamma(value))),
      "HYPGEOM.DIST" => self.eval_hypgeom_dist(&args),
      "LOGNORM.DIST" => self.eval_lognorm_dist(&args),
      "LOGNORM.INV" => self.eval_lognorm_inv(&args),
      "MODE.MULT" | "MODE.SNGL" | "MODE" => {
        mode_value(self.evaluator.book, &args).map(Value::Number)
      }
      "NEGBINOM.DIST" => self.eval_negbinom_dist(&args),
      "NORM.DIST" => self.eval_norm_dist(&args),
      "NORM.INV" => self.eval_norm_inv(&args),
      "NORM.S.DIST" => self.eval_norm_s_dist(&args),
      "NORM.S.INV" => args
        .first()?
        .number(self.evaluator.book)
        .map(|value| Value::Number(norm_s_inv(value))),
      "PERCENTILE.EXC" => self.eval_percentile(&args, PercentileKind::Exc),
      "PERCENTILE.INC" | "PERCENTILE" => self.eval_percentile(&args, PercentileKind::Inc),
      "PERCENTRANK.INC" => self.eval_percent_rank(&args),
      "POISSON.DIST" => self.eval_poisson_dist(&args),
      "QUARTILE.EXC" => self.eval_quartile(&args, PercentileKind::Exc),
      "QUARTILE.INC" | "QUARTILE" => self.eval_quartile(&args, PercentileKind::Inc),
      "RANK.AVG" => self.eval_rank(&args, true),
      "RANK.EQ" | "RANK" => self.eval_rank(&args, false),
      "T.DIST" => self.eval_t_dist(&args),
      "T.DIST.2T" => self.eval_t_dist_tails(&args, 2),
      "T.DIST.RT" => self.eval_t_dist_tails(&args, 1),
      "T.INV" => self.eval_t_inv(&args, false),
      "T.INV.2T" => self.eval_t_inv(&args, true),
      "T.TEST" => self.eval_t_test(&args),
      "WEIBULL.DIST" => self.eval_weibull_dist(&args),
      "NETWORKDAYS.INTL" | "NETWORKDAYS" => self.eval_networkdays(&args),
      "WORKDAY.INTL" | "WORKDAY" => self.eval_workday(&args),
      "Z.TEST" => self.eval_z_test(&args),
      _ => None,
    }
  }

  fn eval_let_raw(&mut self, args: Vec<String>) -> Option<Value> {
    // Source: LibreOffice sc/source/core/tool/interpr1.cxx ScInterpreter::ScLet().
    // LET takes name/value pairs followed by a final calculation. Duplicate
    // names inside one LET are illegal, and the bindings are scoped to the
    // current LET expression.
    if args.len() < 3 || args.len().is_multiple_of(2) {
      return Some(Value::Error("#VALUE!".to_string()));
    }
    let mut local_names = HashSet::new();
    let mut saved_values = Vec::new();
    let mut index = 0;
    while index + 2 < args.len() {
      let name = args[index]
        .trim_start_matches("_xlpm.")
        .to_ascii_uppercase();
      if name.is_empty() || !local_names.insert(name.clone()) {
        restore_let_locals(&mut self.evaluator.locals, saved_values);
        return Some(Value::Error("#VALUE!".to_string()));
      }
      let Some(value) = self.evaluator.eval_formula(&args[index + 1]) else {
        restore_let_locals(&mut self.evaluator.locals, saved_values);
        return None;
      };
      let old_value = self.evaluator.locals.insert(name.clone(), value);
      saved_values.push((name, old_value));
      index += 2;
    }
    let result = self.evaluator.eval_formula(args.last()?);
    restore_let_locals(&mut self.evaluator.locals, saved_values);
    result
  }

  fn eval_if_raw(&mut self, args: Vec<String>) -> Option<Value> {
    let condition = self.evaluator.eval_formula(args.first()?)?;
    if matches!(condition, Value::Error(_)) {
      return Some(condition);
    }
    if condition.truthy(self.evaluator.book) {
      args
        .get(1)
        .map(|formula| self.evaluator.eval_formula(formula))
        .unwrap_or(Some(Value::Bool(true)))
    } else {
      args
        .get(2)
        .map(|formula| self.evaluator.eval_formula(formula))
        .unwrap_or(Some(Value::Bool(false)))
    }
  }

  fn eval_if_error_raw(&mut self, args: Vec<String>, na_only: bool) -> Option<Value> {
    if args.len() != 2 {
      return None;
    }
    let value = self.evaluator.eval_formula(&args[0])?;
    let use_fallback = match &value {
      Value::Error(error) if na_only => error == "#N/A",
      Value::Error(_) => true,
      _ => false,
    };
    if use_fallback {
      self.evaluator.eval_formula(&args[1])
    } else {
      Some(value)
    }
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

  fn eval_subtotal(&self, args: &[Value]) -> Option<Value> {
    let function = args.first()?.number(self.evaluator.book)? as i32;
    let ignore_hidden = function >= 100;
    let options = AggregateOptions {
      ignore_hidden,
      ignore_filtered: true,
      ignore_errors: false,
      ignore_nested: true,
    };
    aggregate_function_value(
      self.evaluator.book,
      function.rem_euclid(100),
      args.get(1..).unwrap_or_default(),
      None,
      options,
    )
    .map(|result| result.map(Value::Number).unwrap_or_else(|error| error))
  }

  fn eval_aggregate(&self, args: &[Value]) -> Option<Value> {
    let function = args.first()?.number(self.evaluator.book)? as i32;
    let options = aggregate_options(args.get(1)?.number(self.evaluator.book)? as i32)?;
    let k = args
      .get(3)
      .and_then(|value| value.number(self.evaluator.book));
    let data = if (14..=19).contains(&function) {
      args.get(2..3)?
    } else {
      args.get(2..)?
    };
    aggregate_function_value(self.evaluator.book, function, data, k, options)
      .map(|result| result.map(Value::Number).unwrap_or_else(|error| error))
  }

  fn eval_beta_dist(&self, args: &[Value]) -> Option<Value> {
    let x = args.first()?.number(self.evaluator.book)?;
    let alpha = args.get(1)?.number(self.evaluator.book)?;
    let beta = args.get(2)?.number(self.evaluator.book)?;
    let cumulative = args.get(3)?.truthy(self.evaluator.book);
    let lower = args
      .get(4)
      .and_then(|value| value.number(self.evaluator.book))
      .unwrap_or(0.0);
    let upper = args
      .get(5)
      .and_then(|value| value.number(self.evaluator.book))
      .unwrap_or(1.0);
    if alpha <= 0.0 || beta <= 0.0 || upper <= lower || x < lower || x > upper {
      return Some(Value::Error("#NUM!".to_string()));
    }
    let scaled = (x - lower) / (upper - lower);
    Some(Value::Number(if cumulative {
      beta_dist(scaled, alpha, beta)
    } else {
      beta_dist_pdf(scaled, alpha, beta) / (upper - lower)
    }))
  }

  fn eval_beta_inv(&self, args: &[Value]) -> Option<Value> {
    let p = args.first()?.number(self.evaluator.book)?;
    let alpha = args.get(1)?.number(self.evaluator.book)?;
    let beta = args.get(2)?.number(self.evaluator.book)?;
    let lower = args
      .get(3)
      .and_then(|value| value.number(self.evaluator.book))
      .unwrap_or(0.0);
    let upper = args
      .get(4)
      .and_then(|value| value.number(self.evaluator.book))
      .unwrap_or(1.0);
    if !(0.0..=1.0).contains(&p) || alpha <= 0.0 || beta <= 0.0 || upper <= lower {
      return Some(Value::Error("#NUM!".to_string()));
    }
    Some(Value::Number(
      lower + inverse_monotonic(p, 0.0, 1.0, |x| beta_dist(x, alpha, beta)) * (upper - lower),
    ))
  }

  fn eval_binom_dist(&self, args: &[Value]) -> Option<Value> {
    let x = args.first()?.number(self.evaluator.book)?.floor();
    let n = args.get(1)?.number(self.evaluator.book)?.floor();
    let p = args.get(2)?.number(self.evaluator.book)?;
    let cumulative = args.get(3)?.truthy(self.evaluator.book);
    if x < 0.0 || n < 0.0 || x > n || !(0.0..=1.0).contains(&p) {
      return Some(Value::Error("#NUM!".to_string()));
    }
    Some(Value::Number(if cumulative {
      (0..=x as u64).map(|k| binom_dist_pmf(k as f64, n, p)).sum()
    } else {
      binom_dist_pmf(x, n, p)
    }))
  }

  fn eval_binom_inv(&self, args: &[Value]) -> Option<Value> {
    let n = args.first()?.number(self.evaluator.book)?.floor();
    let p = args.get(1)?.number(self.evaluator.book)?;
    let alpha = args.get(2)?.number(self.evaluator.book)?;
    if n < 0.0 || !(0.0..=1.0).contains(&p) || !(0.0..=1.0).contains(&alpha) {
      return Some(Value::Error("#NUM!".to_string()));
    }
    let mut cumulative = 0.0;
    for k in 0..=n as u64 {
      cumulative += binom_dist_pmf(k as f64, n, p);
      if cumulative >= alpha {
        return Some(Value::Number(k as f64));
      }
    }
    Some(Value::Number(n))
  }

  fn eval_chisq_dist(&self, args: &[Value], right_tail: bool) -> Option<Value> {
    let x = args.first()?.number(self.evaluator.book)?;
    let df = args.get(1)?.number(self.evaluator.book)?.floor();
    if df < 1.0 || x < 0.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    if right_tail {
      return Some(Value::Number(upper_reg_igamma(df / 2.0, x / 2.0)));
    }
    let cumulative = args.get(2)?.truthy(self.evaluator.book);
    Some(Value::Number(if cumulative {
      lower_reg_igamma(df / 2.0, x / 2.0)
    } else {
      chisq_dist_pdf(x, df)
    }))
  }

  fn eval_chisq_inv(&self, args: &[Value], right_tail: bool) -> Option<Value> {
    let p = args.first()?.number(self.evaluator.book)?;
    let df = args.get(1)?.number(self.evaluator.book)?.floor();
    if !(0.0..=1.0).contains(&p) || df < 1.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    let target = if right_tail { 1.0 - p } else { p };
    Some(Value::Number(inverse_positive(target, |x| {
      lower_reg_igamma(df / 2.0, x / 2.0)
    })))
  }

  fn eval_chisq_test(&self, args: &[Value]) -> Option<Value> {
    let actual = matrix_values(self.evaluator.book, args.first()?);
    let expected = matrix_values(self.evaluator.book, args.get(1)?);
    if actual.is_empty()
      || expected.is_empty()
      || actual.len() != expected.len()
      || actual.first()?.len() != expected.first()?.len()
    {
      return Some(Value::Error("#NUM!".to_string()));
    }
    let rows = actual.len();
    let cols = actual.first()?.len();
    let mut chi = 0.0;
    let mut has_value = false;
    for row in 0..rows {
      for col in 0..cols {
        match (&actual[row][col], &expected[row][col]) {
          (Value::Blank, _) | (_, Value::Blank) => {}
          (left, right) => {
            let Some(observed) = left.number(self.evaluator.book) else {
              return Some(Value::Error("#VALUE!".to_string()));
            };
            let Some(expect) = right.number(self.evaluator.book) else {
              return Some(Value::Error("#VALUE!".to_string()));
            };
            if expect == 0.0 {
              return Some(Value::Error("#DIV/0!".to_string()));
            }
            has_value = true;
            let delta = observed - expect;
            let term = delta * delta / expect;
            if term.is_infinite() {
              return Some(Value::Error("#NUM!".to_string()));
            }
            chi += term;
          }
        }
      }
    }
    if !has_value {
      return Some(Value::Error("#NUM!".to_string()));
    }
    let df = if rows == 1 || cols == 1 {
      (rows * cols).saturating_sub(1) as f64
    } else {
      ((rows - 1) * (cols - 1)) as f64
    };
    if df == 0.0 {
      return Some(Value::Error("#VALUE!".to_string()));
    }
    Some(Value::Number(upper_reg_igamma(df / 2.0, chi / 2.0)))
  }

  fn eval_confidence_norm(&self, args: &[Value]) -> Option<Value> {
    let alpha = args.first()?.number(self.evaluator.book)?;
    let sigma = args.get(1)?.number(self.evaluator.book)?;
    let size = args.get(2)?.number(self.evaluator.book)?;
    if !(0.0..1.0).contains(&alpha) || sigma <= 0.0 || size < 1.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    Some(Value::Number(
      norm_s_inv(1.0 - alpha / 2.0).abs() * sigma / size.sqrt(),
    ))
  }

  fn eval_confidence_t(&self, args: &[Value]) -> Option<Value> {
    let alpha = args.first()?.number(self.evaluator.book)?;
    let sigma = args.get(1)?.number(self.evaluator.book)?;
    let size = args.get(2)?.number(self.evaluator.book)?;
    if !(0.0..1.0).contains(&alpha) || sigma <= 0.0 || size < 2.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    Some(Value::Number(
      t_inv_2t(alpha, size - 1.0) * sigma / size.sqrt(),
    ))
  }

  fn eval_covariance(&self, args: &[Value], sample: bool) -> Option<Value> {
    let left = value_numbers(self.evaluator.book, args.first()?);
    let right = value_numbers(self.evaluator.book, args.get(1)?);
    let count = left.len().min(right.len());
    if count == 0 || (sample && count < 2) {
      return Some(Value::Error("#DIV/0!".to_string()));
    }
    let left_mean = left.iter().take(count).sum::<f64>() / count as f64;
    let right_mean = right.iter().take(count).sum::<f64>() / count as f64;
    let sum = (0..count)
      .map(|index| (left[index] - left_mean) * (right[index] - right_mean))
      .sum::<f64>();
    Some(Value::Number(
      sum / if sample { count - 1 } else { count } as f64,
    ))
  }

  fn eval_expon_dist(&self, args: &[Value]) -> Option<Value> {
    let x = args.first()?.number(self.evaluator.book)?;
    let lambda = args.get(1)?.number(self.evaluator.book)?;
    let cumulative = args.get(2)?.truthy(self.evaluator.book);
    if x < 0.0 || lambda <= 0.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    Some(Value::Number(if cumulative {
      1.0 - (-lambda * x).exp()
    } else {
      lambda * (-lambda * x).exp()
    }))
  }

  fn eval_f_dist(&self, args: &[Value]) -> Option<Value> {
    let x = args.first()?.number(self.evaluator.book)?;
    let df1 = approx_floor(args.get(1)?.number(self.evaluator.book)?);
    let df2 = approx_floor(args.get(2)?.number(self.evaluator.book)?);
    let cumulative = args.get(3)?.truthy(self.evaluator.book);
    if x < 0.0 || df1 <= 0.0 || df2 <= 0.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    Some(Value::Number(if cumulative {
      beta_dist(df1 * x / (df1 * x + df2), df1 / 2.0, df2 / 2.0)
    } else {
      let a = df1 / 2.0;
      let b = df2 / 2.0;
      (df1 / df2).powf(a) * x.powf(a - 1.0) / beta(a, b) / (1.0 + df1 * x / df2).powf(a + b)
    }))
  }

  fn eval_f_dist_rt(&self, args: &[Value]) -> Option<Value> {
    let x = args.first()?.number(self.evaluator.book)?;
    let df1 = approx_floor(args.get(1)?.number(self.evaluator.book)?);
    let df2 = approx_floor(args.get(2)?.number(self.evaluator.book)?);
    if x < 0.0 || df1 <= 0.0 || df2 <= 0.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    Some(Value::Number(beta_dist(
      df2 / (df2 + df1 * x),
      df2 / 2.0,
      df1 / 2.0,
    )))
  }

  fn eval_f_inv(&self, args: &[Value], right_tail: bool) -> Option<Value> {
    let p = args.first()?.number(self.evaluator.book)?;
    let df1 = approx_floor(args.get(1)?.number(self.evaluator.book)?);
    let df2 = approx_floor(args.get(2)?.number(self.evaluator.book)?);
    if !(0.0..=1.0).contains(&p) || df1 <= 0.0 || df2 <= 0.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    let target = if right_tail { 1.0 - p } else { p };
    Some(Value::Number(inverse_positive(target, |x| {
      beta_dist(df1 * x / (df1 * x + df2), df1 / 2.0, df2 / 2.0)
    })))
  }

  fn eval_f_test(&self, args: &[Value]) -> Option<Value> {
    let left = value_numbers(self.evaluator.book, args.first()?);
    let right = value_numbers(self.evaluator.book, args.get(1)?);
    if left.len() < 2 || right.len() < 2 {
      return Some(Value::Error("#VALUE!".to_string()));
    }
    let var_left = variance_slice(&left, true)?;
    let var_right = variance_slice(&right, true)?;
    if var_left == 0.0 || var_right == 0.0 {
      return Some(Value::Error("#VALUE!".to_string()));
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
    let cdf = beta_dist(df1 * f / (df1 * f + df2), df1 / 2.0, df2 / 2.0);
    Some(Value::Number(2.0 * cdf.min(1.0 - cdf)))
  }

  fn eval_gamma_dist(&self, args: &[Value]) -> Option<Value> {
    let x = args.first()?.number(self.evaluator.book)?;
    let alpha = args.get(1)?.number(self.evaluator.book)?;
    let beta = args.get(2)?.number(self.evaluator.book)?;
    let cumulative = args.get(3)?.truthy(self.evaluator.book);
    if x < 0.0 || alpha <= 0.0 || beta <= 0.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    Some(Value::Number(if cumulative {
      gamma_dist(x, alpha, beta)
    } else {
      gamma_dist_pdf(x, alpha, beta)
    }))
  }

  fn eval_gamma_inv(&self, args: &[Value]) -> Option<Value> {
    let p = args.first()?.number(self.evaluator.book)?;
    let alpha = args.get(1)?.number(self.evaluator.book)?;
    let beta = args.get(2)?.number(self.evaluator.book)?;
    if !(0.0..=1.0).contains(&p) || alpha <= 0.0 || beta <= 0.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    Some(Value::Number(inverse_positive(p, |x| {
      gamma_dist(x, alpha, beta)
    })))
  }

  fn eval_hypgeom_dist(&self, args: &[Value]) -> Option<Value> {
    let sample_success = args.first()?.number(self.evaluator.book)?.floor();
    let sample_size = args.get(1)?.number(self.evaluator.book)?.floor();
    let population_success = args.get(2)?.number(self.evaluator.book)?.floor();
    let population_size = args.get(3)?.number(self.evaluator.book)?.floor();
    let cumulative = args.get(4)?.truthy(self.evaluator.book);
    if sample_success < 0.0
      || sample_size < 0.0
      || population_success < 0.0
      || population_size < 0.0
      || sample_size > population_size
      || population_success > population_size
    {
      return Some(Value::Error("#NUM!".to_string()));
    }
    let pmf = |x: f64| {
      binom_coeff(population_success, x)
        * binom_coeff(population_size - population_success, sample_size - x)
        / binom_coeff(population_size, sample_size)
    };
    Some(Value::Number(if cumulative {
      (0..=sample_success as u64).map(|x| pmf(x as f64)).sum()
    } else {
      pmf(sample_success)
    }))
  }

  fn eval_lognorm_dist(&self, args: &[Value]) -> Option<Value> {
    let x = args.first()?.number(self.evaluator.book)?;
    let mean = args.get(1)?.number(self.evaluator.book)?;
    let sigma = args.get(2)?.number(self.evaluator.book)?;
    let cumulative = args.get(3)?.truthy(self.evaluator.book);
    if x <= 0.0 || sigma <= 0.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    let z = (x.ln() - mean) / sigma;
    Some(Value::Number(if cumulative {
      norm_s_dist(z)
    } else {
      (-0.5 * z * z).exp() / (x * sigma * (2.0 * std::f64::consts::PI).sqrt())
    }))
  }

  fn eval_lognorm_inv(&self, args: &[Value]) -> Option<Value> {
    let p = args.first()?.number(self.evaluator.book)?;
    let mean = args.get(1)?.number(self.evaluator.book)?;
    let sigma = args.get(2)?.number(self.evaluator.book)?;
    if !(0.0..1.0).contains(&p) || sigma <= 0.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    Some(Value::Number((mean + sigma * norm_s_inv(p)).exp()))
  }

  fn eval_negbinom_dist(&self, args: &[Value]) -> Option<Value> {
    let failures = args.first()?.number(self.evaluator.book)?.floor();
    let successes = args.get(1)?.number(self.evaluator.book)?.floor();
    let p = args.get(2)?.number(self.evaluator.book)?;
    let cumulative = args.get(3)?.truthy(self.evaluator.book);
    if failures < 0.0 || successes < 1.0 || !(0.0..=1.0).contains(&p) {
      return Some(Value::Error("#NUM!".to_string()));
    }
    let pmf = |f: f64| binom_coeff(f + successes - 1.0, f) * p.powf(successes) * (1.0 - p).powf(f);
    Some(Value::Number(if cumulative {
      (0..=failures as u64).map(|f| pmf(f as f64)).sum()
    } else {
      pmf(failures)
    }))
  }

  fn eval_norm_dist(&self, args: &[Value]) -> Option<Value> {
    let x = args.first()?.number(self.evaluator.book)?;
    let mean = args.get(1)?.number(self.evaluator.book)?;
    let sigma = args.get(2)?.number(self.evaluator.book)?;
    let cumulative = args.get(3)?.truthy(self.evaluator.book);
    if sigma <= 0.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    let z = (x - mean) / sigma;
    Some(Value::Number(if cumulative {
      norm_s_dist(z)
    } else {
      norm_s_pdf(z) / sigma
    }))
  }

  fn eval_norm_inv(&self, args: &[Value]) -> Option<Value> {
    let p = args.first()?.number(self.evaluator.book)?;
    let mean = args.get(1)?.number(self.evaluator.book)?;
    let sigma = args.get(2)?.number(self.evaluator.book)?;
    if !(0.0..1.0).contains(&p) || sigma <= 0.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    Some(Value::Number(mean + sigma * norm_s_inv(p)))
  }

  fn eval_norm_s_dist(&self, args: &[Value]) -> Option<Value> {
    let z = args.first()?.number(self.evaluator.book)?;
    let cumulative = args.get(1)?.truthy(self.evaluator.book);
    Some(Value::Number(if cumulative {
      norm_s_dist(z)
    } else {
      norm_s_pdf(z)
    }))
  }

  fn eval_percentile(&self, args: &[Value], kind: PercentileKind) -> Option<Value> {
    let k = args.get(1)?.number(self.evaluator.book)?;
    percentile_values(self.evaluator.book, &[args.first()?.clone()], k, kind).map(Value::Number)
  }

  fn eval_percent_rank(&self, args: &[Value]) -> Option<Value> {
    let mut values = value_numbers(self.evaluator.book, args.first()?);
    values.sort_by(f64::total_cmp);
    let x = args.get(1)?.number(self.evaluator.book)?;
    if values.is_empty() || x < *values.first()? || x > *values.last()? {
      return Some(Value::Error("#N/A".to_string()));
    }
    for (index, value) in values.iter().enumerate() {
      if *value == x {
        return Some(Value::Number(index as f64 / (values.len() - 1) as f64));
      }
      if *value > x {
        let previous = values[index - 1];
        let fraction = (x - previous) / (*value - previous);
        return Some(Value::Number(
          (index as f64 - 1.0 + fraction) / (values.len() - 1) as f64,
        ));
      }
    }
    Some(Value::Number(1.0))
  }

  fn eval_poisson_dist(&self, args: &[Value]) -> Option<Value> {
    let x = args.first()?.number(self.evaluator.book)?.floor();
    let lambda = args.get(1)?.number(self.evaluator.book)?;
    let cumulative = args.get(2)?.truthy(self.evaluator.book);
    if x < 0.0 || lambda <= 0.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    let pmf = |k: f64| (k * lambda.ln() - lambda - log_gamma(k + 1.0)).exp();
    Some(Value::Number(if cumulative {
      (0..=x as u64).map(|k| pmf(k as f64)).sum()
    } else {
      pmf(x)
    }))
  }

  fn eval_quartile(&self, args: &[Value], kind: PercentileKind) -> Option<Value> {
    let quart = args.get(1)?.number(self.evaluator.book)?;
    percentile_values(
      self.evaluator.book,
      &[args.first()?.clone()],
      quart / 4.0,
      kind,
    )
    .map(Value::Number)
  }

  fn eval_rank(&self, args: &[Value], average: bool) -> Option<Value> {
    let value = args.first()?.number(self.evaluator.book)?;
    let mut values = value_numbers(self.evaluator.book, args.get(1)?);
    let ascending = args
      .get(2)
      .is_some_and(|value| value.truthy(self.evaluator.book));
    values.sort_by(f64::total_cmp);
    if !ascending {
      values.reverse();
    }
    let positions = values
      .iter()
      .enumerate()
      .filter_map(|(index, candidate)| (*candidate == value).then_some(index as f64 + 1.0))
      .collect::<Vec<_>>();
    if positions.is_empty() {
      return Some(Value::Error("#N/A".to_string()));
    }
    Some(Value::Number(if average {
      positions.iter().sum::<f64>() / positions.len() as f64
    } else {
      positions[0]
    }))
  }

  fn eval_t_dist(&self, args: &[Value]) -> Option<Value> {
    let t = args.first()?.number(self.evaluator.book)?;
    let df = args.get(1)?.number(self.evaluator.book)?;
    let cumulative = args.get(2)?.truthy(self.evaluator.book);
    if df <= 0.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    Some(Value::Number(if cumulative {
      t_dist(t, df, 4)
    } else {
      t_dist(t, df, 3)
    }))
  }

  fn eval_t_dist_tails(&self, args: &[Value], tails: i32) -> Option<Value> {
    let t = args.first()?.number(self.evaluator.book)?;
    let df = args.get(1)?.number(self.evaluator.book)?;
    if t < 0.0 || df <= 0.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    Some(Value::Number(t_dist(t, df, tails)))
  }

  fn eval_t_inv(&self, args: &[Value], two_tailed: bool) -> Option<Value> {
    let p = args.first()?.number(self.evaluator.book)?;
    let df = args.get(1)?.number(self.evaluator.book)?;
    if !(0.0..1.0).contains(&p) || df <= 0.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    Some(Value::Number(if two_tailed {
      t_inv_2t(p, df)
    } else {
      inverse_monotonic(p, -100.0, 100.0, |x| t_dist(x, df, 4))
    }))
  }

  fn eval_t_test(&self, args: &[Value]) -> Option<Value> {
    let left = value_numbers(self.evaluator.book, args.first()?);
    let right = value_numbers(self.evaluator.book, args.get(1)?);
    let tails = args.get(2)?.number(self.evaluator.book)? as i32;
    let test_type = args.get(3)?.number(self.evaluator.book)? as i32;
    if left.is_empty() || right.is_empty() || !(1..=2).contains(&tails) {
      return Some(Value::Error("#NUM!".to_string()));
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
    Some(Value::Number(t_dist(t, df, tails)))
  }

  fn eval_weibull_dist(&self, args: &[Value]) -> Option<Value> {
    let x = args.first()?.number(self.evaluator.book)?;
    let alpha = args.get(1)?.number(self.evaluator.book)?;
    let beta = args.get(2)?.number(self.evaluator.book)?;
    let cumulative = args.get(3)?.truthy(self.evaluator.book);
    if x < 0.0 || alpha <= 0.0 || beta <= 0.0 {
      return Some(Value::Error("#NUM!".to_string()));
    }
    let pow = (x / beta).powf(alpha);
    Some(Value::Number(if cumulative {
      1.0 - (-pow).exp()
    } else {
      alpha / beta.powf(alpha) * x.powf(alpha - 1.0) * (-pow).exp()
    }))
  }

  fn eval_networkdays(&self, args: &[Value]) -> Option<Value> {
    let mut start = args.first()?.number(self.evaluator.book)?.floor() as i64;
    let mut end = args.get(1)?.number(self.evaluator.book)?.floor() as i64;
    let weekend = weekend_mask(args.get(2), false, self.evaluator.book)?;
    let holidays = holiday_serials(args.get(3), self.evaluator.book);
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
    Some(Value::Number(if reverse {
      -(count as f64)
    } else {
      count as f64
    }))
  }

  fn eval_workday(&self, args: &[Value]) -> Option<Value> {
    let mut date = args.first()?.number(self.evaluator.book)?.floor() as i64;
    let mut days = args.get(1)?.number(self.evaluator.book)?.floor() as i64;
    let weekend = weekend_mask(args.get(2), true, self.evaluator.book)?;
    let holidays = holiday_serials(args.get(3), self.evaluator.book);
    if days == 0 {
      return Some(Value::Number(date as f64));
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
    Some(Value::Number(date as f64))
  }

  fn eval_z_test(&self, args: &[Value]) -> Option<Value> {
    let values = value_numbers(self.evaluator.book, args.first()?);
    let x = args.get(1)?.number(self.evaluator.book)?;
    let sigma = args
      .get(2)
      .and_then(|value| value.number(self.evaluator.book))
      .unwrap_or_else(|| variance_slice(&values, true).unwrap_or(0.0).sqrt());
    let z = (mean(&values)? - x) / (sigma / (values.len() as f64).sqrt());
    Some(Value::Number(1.0 - norm_s_dist(z)))
  }

  fn eval_ceiling(&mut self, args: Vec<Value>, kind: CeilingFloorKind) -> Option<Value> {
    let value = args.first()?.number(self.evaluator.book)?;
    let significance = match kind {
      CeilingFloorKind::Odff => args
        .get(1)
        .and_then(|value| value.number(self.evaluator.book))
        .unwrap_or(if value < 0.0 { -1.0 } else { 1.0 }),
      CeilingFloorKind::Math => args
        .get(1)
        .and_then(|value| value.number(self.evaluator.book))
        .unwrap_or(1.0),
      CeilingFloorKind::Precise => args
        .get(1)
        .and_then(|value| value.number(self.evaluator.book))
        .unwrap_or(1.0)
        .abs(),
    };
    if value == 0.0 || significance == 0.0 {
      return Some(Value::Number(0.0));
    }
    match kind {
      CeilingFloorKind::Odff => {
        if value * significance < 0.0 {
          Some(Value::Error("Err:502".to_string()))
        } else if value < 0.0 {
          let significance = if value * significance < 0.0 {
            -significance
          } else {
            significance
          };
          let abs_mode = args
            .get(2)
            .is_some_and(|value| value.truthy(self.evaluator.book));
          let quotient = value / significance;
          Some(Value::Number(
            if abs_mode {
              approx_ceil(quotient)
            } else {
              approx_floor(quotient)
            } * significance,
          ))
        } else {
          Some(Value::Number(
            approx_ceil(value / significance) * significance,
          ))
        }
      }
      CeilingFloorKind::Math => {
        let significance = if value * significance < 0.0 {
          -significance
        } else {
          significance
        };
        let abs_mode = args
          .get(2)
          .is_some_and(|value| value.truthy(self.evaluator.book));
        let quotient = value / significance;
        Some(Value::Number(
          if !abs_mode && value < 0.0 {
            approx_floor(quotient)
          } else {
            approx_ceil(quotient)
          } * significance,
        ))
      }
      CeilingFloorKind::Precise => Some(Value::Number(
        approx_ceil(value / significance) * significance,
      )),
    }
  }

  fn eval_floor(&mut self, args: Vec<Value>, kind: CeilingFloorKind) -> Option<Value> {
    let value = args.first()?.number(self.evaluator.book)?;
    let significance = match kind {
      CeilingFloorKind::Odff => args
        .get(1)
        .and_then(|value| value.number(self.evaluator.book))
        .unwrap_or(if value < 0.0 { -1.0 } else { 1.0 }),
      CeilingFloorKind::Math => args
        .get(1)
        .and_then(|value| value.number(self.evaluator.book))
        .unwrap_or(1.0),
      CeilingFloorKind::Precise => args
        .get(1)
        .and_then(|value| value.number(self.evaluator.book))
        .unwrap_or(1.0)
        .abs(),
    };
    if value == 0.0 || significance == 0.0 {
      return Some(Value::Number(0.0));
    }
    match kind {
      CeilingFloorKind::Odff => {
        if value * significance < 0.0 {
          Some(Value::Error("Err:502".to_string()))
        } else if value < 0.0 {
          let significance = if value * significance < 0.0 {
            -significance
          } else {
            significance
          };
          let abs_mode = args
            .get(2)
            .is_some_and(|value| value.truthy(self.evaluator.book));
          let quotient = value / significance;
          Some(Value::Number(
            if abs_mode {
              approx_floor(quotient)
            } else {
              approx_ceil(quotient)
            } * significance,
          ))
        } else {
          Some(Value::Number(
            approx_floor(value / significance) * significance,
          ))
        }
      }
      CeilingFloorKind::Math => {
        let significance = if value * significance < 0.0 {
          -significance
        } else {
          significance
        };
        let abs_mode = args
          .get(2)
          .is_some_and(|value| value.truthy(self.evaluator.book));
        let quotient = value / significance;
        Some(Value::Number(
          if !abs_mode && value < 0.0 {
            approx_ceil(quotient)
          } else {
            approx_floor(quotient)
          } * significance,
        ))
      }
      CeilingFloorKind::Precise => Some(Value::Number(
        approx_floor(value / significance) * significance,
      )),
    }
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

  fn eval_offset(&self, args: &[Value]) -> Option<Value> {
    // Source: LibreOffice sc/source/core/tool/interpr1.cxx ScInterpreter::ScOffset().
    // OFFSET keeps reference identity and shifts/resizes the input reference.
    if !(3..=5).contains(&args.len()) {
      return None;
    }
    let reference = as_reference(args.first()?)?;
    let row_offset = args.get(1)?.number(self.evaluator.book)? as i64;
    let col_offset = args.get(2)?.number(self.evaluator.book)? as i64;
    let height = args
      .get(3)
      .and_then(|value| value.number(self.evaluator.book))
      .map(|value| value as i64)
      .unwrap_or_else(|| i64::from(reference.range.end.row - reference.range.start.row + 1));
    let width = args
      .get(4)
      .and_then(|value| value.number(self.evaluator.book))
      .map(|value| value as i64)
      .unwrap_or_else(|| i64::from(reference.range.end.col - reference.range.start.col + 1));
    if width <= 0 || height <= 0 {
      return Some(Value::Error("#VALUE!".to_string()));
    }
    let start_col = i64::from(reference.range.start.col) + col_offset;
    let start_row = i64::from(reference.range.start.row) + row_offset;
    let end_col = start_col + width - 1;
    let end_row = start_row + height - 1;
    if start_col < 1
      || start_row < 1
      || end_col > i64::from(XLSX_MAX_COLUMN)
      || end_row > i64::from(XLSX_MAX_ROW)
    {
      return Some(Value::Error("#VALUE!".to_string()));
    }
    Some(Value::Range(Reference {
      sheet_index: reference.sheet_index,
      external_link_index: reference.external_link_index,
      external_sheet_name: reference.external_sheet_name,
      range: CellRange::new(
        CellAddress {
          col: start_col as u32,
          row: start_row as u32,
        },
        CellAddress {
          col: end_col as u32,
          row: end_row as u32,
        },
      ),
    }))
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

  fn eval_formula_text(&self, args: &[Value]) -> Option<Value> {
    let reference = as_reference(args.first()?)?;
    let sheet_index = reference.sheet_index?;
    self
      .evaluator
      .book
      .formula_text(sheet_index, reference.range.start)
      .map(Value::Text)
      .or_else(|| Some(Value::Error("#N/A".to_string())))
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
        current_address: self.evaluator.current_address,
        locals: self.evaluator.locals.clone(),
      };
      return evaluator.eval_formula(formula);
    }
    self.resolve_reference(token)
  }

  fn resolve_reference(&self, reference: &str) -> Option<Value> {
    parse_qualified_reference(
      self.evaluator.book,
      self.evaluator.sheet_index,
      self.evaluator.current_address,
      reference,
    )
    .map(Value::Range)
  }

  fn parse_reference_token(&mut self) -> Option<String> {
    self.skip_ws();
    let start = self.position;
    if self.peek() == Some('\'') {
      self.position += 1;
      while let Some(ch) = self.peek() {
        self.position += 1;
        if ch == '\'' {
          if self.peek() == Some('\'') {
            self.position += 1;
          } else {
            break;
          }
        }
      }
      if self.peek() == Some(':') {
        self.position += 1;
        if self.peek() == Some('\'') {
          self.position += 1;
          while let Some(ch) = self.peek() {
            self.position += 1;
            if ch == '\'' {
              if self.peek() == Some('\'') {
                self.position += 1;
              } else {
                break;
              }
            }
          }
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
    let mut table_ref_depth = 0i32;
    while let Some(ch) = self.peek() {
      if table_ref_depth > 0 {
        match ch {
          '[' => table_ref_depth += 1,
          ']' => table_ref_depth -= 1,
          _ => {}
        }
        self.position += 1;
        continue;
      }
      if ch == '[' {
        table_ref_depth += 1;
        self.position += 1;
        continue;
      }
      if !is_ref_char(ch) {
        break;
      }
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
    let mut value = String::new();
    while let Some(ch) = self.peek() {
      self.position += 1;
      if ch == '"' {
        if self.peek() == Some('"') {
          value.push('"');
          self.position += 1;
          continue;
        }
        return Some(value);
      }
      value.push(ch);
    }
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
      "=" => left == right || approx_equal(left, right),
      "<>" => left != right && !approx_equal(left, right),
      "<" => left < right,
      "<=" => left <= right || approx_equal(left, right),
      ">" => left > right,
      ">=" => left >= right || approx_equal(left, right),
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PercentileKind {
  Inc,
  Exc,
}

#[derive(Clone, Copy, Debug)]
struct AggregateOptions {
  ignore_hidden: bool,
  ignore_filtered: bool,
  ignore_errors: bool,
  ignore_nested: bool,
}

fn aggregate_options(option: i32) -> Option<AggregateOptions> {
  // Source: LibreOffice sc/source/core/tool/interpr1.cxx ScAggregate,
  // where options 0..7 combine hidden-row, error-value and nested aggregate
  // filtering.
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

fn restore_let_locals(
  locals: &mut HashMap<String, Value>,
  saved_values: Vec<(String, Option<Value>)>,
) {
  for (name, value) in saved_values.into_iter().rev() {
    if let Some(value) = value {
      locals.insert(name, value);
    } else {
      locals.remove(&name);
    }
  }
}

fn numeric_values(book: &FormulaBook, args: &[Value]) -> Vec<f64> {
  expand_values(book, args)
    .filter_map(|value| value.number(book))
    .collect()
}

fn value_numbers(book: &FormulaBook, value: &Value) -> Vec<f64> {
  match value {
    Value::Range(reference) => range_values(book, reference)
      .iter()
      .filter_map(|value| value.number(book))
      .collect(),
    Value::Matrix(rows) => rows
      .iter()
      .flatten()
      .filter_map(|value| value.number(book))
      .collect(),
    value => value.number(book).into_iter().collect(),
  }
}

fn matrix_values(book: &FormulaBook, value: &Value) -> Vec<Vec<Value>> {
  match value {
    Value::Range(reference) => {
      if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
        return Vec::new();
      }
      (reference.range.start.row..=reference.range.end.row)
        .map(|row| {
          (reference.range.start.col..=reference.range.end.col)
            .map(|col| reference_cell_value(book, reference, CellAddress { col, row }))
            .collect()
        })
        .collect()
    }
    Value::Matrix(rows) => rows.clone(),
    value => vec![vec![value.clone()]],
  }
}

fn holiday_serials(value: Option<&Value>, book: &FormulaBook) -> Vec<i64> {
  let Some(value) = value else {
    return Vec::new();
  };
  let mut holidays = value_numbers(book, value)
    .into_iter()
    .map(|value| value.floor() as i64)
    .collect::<Vec<_>>();
  holidays.sort_unstable();
  holidays.dedup();
  holidays
}

fn weekend_mask(
  value: Option<&Value>,
  workday_function: bool,
  book: &FormulaBook,
) -> Option<[bool; 7]> {
  // Source: LibreOffice sc/source/core/tool/interpr2.cxx, GetWeekendAndHolidayMasks_MS.
  let mut mask = [false; 7];
  let Some(value) = value else {
    mask[5] = true;
    mask[6] = true;
    return Some(mask);
  };
  let text = match value {
    Value::Blank => String::new(),
    Value::Number(number) => {
      if (1.0..=17.0).contains(number) {
        render_number(number.floor())
      } else {
        return None;
      }
    }
    Value::Text(text) => {
      if text.is_empty() || text.len() != 7 || (workday_function && text == "1111111") {
        return None;
      }
      text.clone()
    }
    _ => value.text(book),
  };
  if text.is_empty() {
    mask[5] = true;
    mask[6] = true;
    return Some(mask);
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

fn weekday_index_from_serial(serial: i64) -> usize {
  let days_since_unix = if serial < 60 {
    serial - 25_568
  } else {
    serial - 25_569
  };
  (days_since_unix + 3).rem_euclid(7) as usize
}

fn average_values(book: &FormulaBook, args: &[Value]) -> Option<f64> {
  mean(&numeric_values(book, args))
}

fn mean(values: &[f64]) -> Option<f64> {
  (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
}

fn variance_values(book: &FormulaBook, args: &[Value], sample: bool) -> Option<f64> {
  variance_slice(&numeric_values(book, args), sample)
}

fn variance_slice(values: &[f64], sample: bool) -> Option<f64> {
  if values.is_empty() || (sample && values.len() < 2) {
    return None;
  }
  let mean = mean(values)?;
  let sum = values
    .iter()
    .map(|value| {
      let delta = value - mean;
      delta * delta
    })
    .sum::<f64>();
  Some(
    sum
      / if sample {
        values.len() - 1
      } else {
        values.len()
      } as f64,
  )
}

fn percentile_values(
  book: &FormulaBook,
  args: &[Value],
  k: f64,
  kind: PercentileKind,
) -> Option<f64> {
  let mut values = numeric_values(book, args);
  percentile_sorted(&mut values, k, kind)
}

fn percentile_sorted(values: &mut [f64], k: f64, kind: PercentileKind) -> Option<f64> {
  if values.is_empty() {
    return None;
  }
  values.sort_by(f64::total_cmp);
  let n = values.len() as f64;
  let rank = match kind {
    PercentileKind::Inc => 1.0 + k * (n - 1.0),
    PercentileKind::Exc => k * (n + 1.0),
  };
  if rank < 1.0 || rank > n {
    return None;
  }
  let lower = rank.floor();
  let upper = rank.ceil();
  let lower_value = values[(lower as usize).saturating_sub(1)];
  if lower == upper {
    return Some(lower_value);
  }
  let upper_value = values[(upper as usize).saturating_sub(1)];
  Some(lower_value + (rank - lower) * (upper_value - lower_value))
}

fn mode_value(book: &FormulaBook, args: &[Value]) -> Option<f64> {
  let values = numeric_values(book, args);
  mode_slice(&values)
}

fn mode_slice(values: &[f64]) -> Option<f64> {
  let mut values = values.to_vec();
  values.sort_by(f64::total_cmp);
  let mut best_value = None;
  let mut best_count = 1usize;
  let mut current_value = None;
  let mut current_count = 0usize;
  for value in values {
    if current_value == Some(value) {
      current_count += 1;
    } else {
      current_value = Some(value);
      current_count = 1;
    }
    if current_count > best_count {
      best_count = current_count;
      best_value = current_value;
    }
  }
  best_value
}

fn aggregate_function_value(
  book: &FormulaBook,
  function: i32,
  args: &[Value],
  k: Option<f64>,
  options: AggregateOptions,
) -> Option<Result<f64, Value>> {
  let values = match aggregate_numbers(book, args, options) {
    Ok(values) => values,
    Err(error) => return Some(Err(error)),
  };
  match function {
    1 => mean(&values),
    2 => Some(values.len() as f64),
    3 => match aggregate_counta(book, args, options)? {
      Ok(count) => Some(count as f64),
      Err(error) => return Some(Err(error)),
    },
    4 => values.into_iter().reduce(f64::max),
    5 => values.into_iter().reduce(f64::min),
    6 => Some(values.into_iter().product()),
    7 => variance_slice(&values, true).map(f64::sqrt),
    8 => variance_slice(&values, false).map(f64::sqrt),
    9 => Some(values.into_iter().sum()),
    10 => variance_slice(&values, true),
    11 => variance_slice(&values, false),
    12 => {
      let mut values = values;
      percentile_sorted(&mut values, 0.5, PercentileKind::Inc)
    }
    13 => mode_slice(&values),
    14 => kth_value(values, k?, true),
    15 => kth_value(values, k?, false),
    16 => {
      let mut values = values;
      percentile_sorted(&mut values, k?, PercentileKind::Inc)
    }
    17 => {
      let mut values = values;
      percentile_sorted(&mut values, k? / 4.0, PercentileKind::Inc)
    }
    18 => {
      let mut values = values;
      percentile_sorted(&mut values, k?, PercentileKind::Exc)
    }
    19 => {
      let mut values = values;
      percentile_sorted(&mut values, k? / 4.0, PercentileKind::Exc)
    }
    _ => None,
  }
  .map(Ok)
}

fn aggregate_numbers(
  book: &FormulaBook,
  args: &[Value],
  options: AggregateOptions,
) -> Result<Vec<f64>, Value> {
  let mut values = Vec::new();
  for arg in args {
    collect_aggregate_numbers(book, arg, options, &mut values)?;
  }
  Ok(values)
}

fn collect_aggregate_numbers(
  book: &FormulaBook,
  value: &Value,
  options: AggregateOptions,
  values: &mut Vec<f64>,
) -> Result<(), Value> {
  match value {
    Value::Range(reference) => {
      if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
        return Ok(());
      }
      let Some(sheet_index) = reference.sheet_index else {
        return Ok(());
      };
      for row in reference.range.start.row..=reference.range.end.row {
        if (options.ignore_filtered && book.row_filtered(sheet_index, row))
          || (options.ignore_hidden && book.row_hidden(sheet_index, row))
        {
          continue;
        }
        for col in reference.range.start.col..=reference.range.end.col {
          let address = CellAddress { col, row };
          if options.ignore_nested && book.is_nested_aggregate(sheet_index, address) {
            continue;
          }
          collect_aggregate_scalar(book.cell(sheet_index, address), options, values)?;
        }
      }
      Ok(())
    }
    Value::Matrix(rows) => {
      for value in rows.iter().flatten() {
        collect_aggregate_scalar(value.clone(), options, values)?;
      }
      Ok(())
    }
    value => collect_aggregate_scalar(value.clone(), options, values),
  }
}

fn collect_aggregate_scalar(
  value: Value,
  options: AggregateOptions,
  values: &mut Vec<f64>,
) -> Result<(), Value> {
  match value {
    Value::Number(number) => values.push(number),
    Value::Bool(value) => values.push(if value { 1.0 } else { 0.0 }),
    Value::Error(error) if !options.ignore_errors => return Err(Value::Error(error)),
    _ => {}
  }
  Ok(())
}

fn aggregate_counta(
  book: &FormulaBook,
  args: &[Value],
  options: AggregateOptions,
) -> Option<Result<usize, Value>> {
  let mut count = 0usize;
  for arg in args {
    match aggregate_counta_value(book, arg, options, &mut count) {
      Ok(()) => {}
      Err(error) => return Some(Err(error)),
    }
  }
  Some(Ok(count))
}

fn aggregate_counta_value(
  book: &FormulaBook,
  value: &Value,
  options: AggregateOptions,
  count: &mut usize,
) -> Result<(), Value> {
  match value {
    Value::Range(reference) => {
      if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
        return Ok(());
      }
      let Some(sheet_index) = reference.sheet_index else {
        return Ok(());
      };
      for row in reference.range.start.row..=reference.range.end.row {
        if (options.ignore_filtered && book.row_filtered(sheet_index, row))
          || (options.ignore_hidden && book.row_hidden(sheet_index, row))
        {
          continue;
        }
        for col in reference.range.start.col..=reference.range.end.col {
          let address = CellAddress { col, row };
          if options.ignore_nested && book.is_nested_aggregate(sheet_index, address) {
            continue;
          }
          aggregate_counta_scalar(book.cell(sheet_index, address), options, count)?;
        }
      }
      Ok(())
    }
    Value::Matrix(rows) => {
      for value in rows.iter().flatten() {
        aggregate_counta_scalar(value.clone(), options, count)?;
      }
      Ok(())
    }
    value => aggregate_counta_scalar(value.clone(), options, count),
  }
}

fn aggregate_counta_scalar(
  value: Value,
  options: AggregateOptions,
  count: &mut usize,
) -> Result<(), Value> {
  match value {
    Value::Blank => {}
    Value::Error(error) if !options.ignore_errors => return Err(Value::Error(error)),
    _ => *count += 1,
  }
  Ok(())
}

fn kth_value(mut values: Vec<f64>, k: f64, descending: bool) -> Option<f64> {
  values.sort_by(f64::total_cmp);
  if descending {
    values.reverse();
  }
  values.get(k.max(1.0) as usize - 1).copied()
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
    let mut addresses = if let Some(sheet_index) = reference.sheet_index {
      book
        .cells
        .keys()
        .filter_map(|(cell_sheet_index, address)| {
          (*cell_sheet_index == sheet_index && reference.range.contains(*address))
            .then_some(*address)
        })
        .collect::<Vec<_>>()
    } else if let Some(sheet_name) = reference.external_sheet_name.as_deref() {
      let sheet_name = sheet_name.to_ascii_uppercase();
      book
        .external_cells
        .keys()
        .filter_map(|(link_index, cell_sheet_name, address)| {
          (Some(*link_index) == reference.external_link_index
            && *cell_sheet_name == sheet_name
            && reference.range.contains(*address))
          .then_some(*address)
        })
        .collect::<Vec<_>>()
    } else {
      Vec::new()
    };
    addresses.sort_by_key(|address| (address.row, address.col));
    return addresses
      .into_iter()
      .map(|address| reference_cell_value(book, reference, address))
      .collect();
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

fn gamma_lanczos_sum(z: f64) -> f64 {
  // Source: LibreOffice sc/source/core/tool/interpr3.cxx, lanczos13m53.
  const NUM: [f64; 13] = [
    23531376880.41076,
    42919803642.6491,
    35711959237.35567,
    17921034426.03721,
    6039542586.352028,
    1439720407.3117216,
    248874557.86205417,
    31426415.585400194,
    2876370.6289353725,
    186056.2653952235,
    8071.672002365816,
    210.82427775157935,
    2.5066282746310002,
  ];
  const DEN: [f64; 13] = [
    0.0,
    39916800.0,
    120543840.0,
    150917976.0,
    105258076.0,
    45995730.0,
    13339535.0,
    2637558.0,
    357423.0,
    32670.0,
    1925.0,
    66.0,
    1.0,
  ];
  let (mut sum_num, mut sum_den);
  if z <= 1.0 {
    sum_num = NUM[12];
    sum_den = DEN[12];
    for index in (0..12).rev() {
      sum_num = sum_num * z + NUM[index];
      sum_den = sum_den * z + DEN[index];
    }
  } else {
    let inv = 1.0 / z;
    sum_num = NUM[0];
    sum_den = DEN[0];
    for index in 1..=12 {
      sum_num = sum_num * inv + NUM[index];
      sum_den = sum_den * inv + DEN[index];
    }
  }
  sum_num / sum_den
}

fn gamma(value: f64) -> f64 {
  if value >= 1.0 {
    return gamma_helper(value);
  }
  if value >= 0.5 {
    return gamma_helper(value + 1.0) / value;
  }
  std::f64::consts::PI / (gamma_helper(1.0 - value) * (std::f64::consts::PI * value).sin())
}

fn gamma_helper(z: f64) -> f64 {
  let g = 6.02468004077673;
  let help = z + g - 0.5;
  let half = help.powf(z / 2.0 - 0.25);
  gamma_lanczos_sum(z) * half / help.exp() * half
}

fn log_gamma(z: f64) -> f64 {
  let g = 6.02468004077673;
  let help = z + g - 0.5;
  if z >= 1.0 {
    gamma_lanczos_sum(z).ln() + (z - 0.5) * help.ln() - help
  } else if z >= 0.5 {
    gamma(z).ln()
  } else {
    gamma_lanczos_sum(z + 2.0).ln() + (z + 1.5) * (z + 2.0 + g - 0.5).ln()
      - (z + 2.0 + g - 0.5)
      - (1.0 + z).ln()
      - z.ln()
  }
}

fn beta(a: f64, b: f64) -> f64 {
  (log_gamma(a) + log_gamma(b) - log_gamma(a + b)).exp()
}

fn log_beta(a: f64, b: f64) -> f64 {
  log_gamma(a) + log_gamma(b) - log_gamma(a + b)
}

fn beta_dist_pdf(x: f64, a: f64, b: f64) -> f64 {
  if x <= 0.0 {
    return if a < 1.0 && x == 0.0 {
      f64::INFINITY
    } else {
      0.0
    };
  }
  if x >= 1.0 {
    return if b < 1.0 && x == 1.0 {
      f64::INFINITY
    } else {
      0.0
    };
  }
  ((a - 1.0) * x.ln() + (b - 1.0) * (1.0 - x).ln() - log_beta(a, b)).exp()
}

fn beta_cont_frac(x: f64, a: f64, b: f64) -> f64 {
  // Source: LibreOffice lcl_GetBetaHelperContFrac.
  let mut a1 = 1.0;
  let mut b1 = 1.0;
  let mut b2 = 1.0 - (a + b) / (a + 1.0) * x;
  let mut a2;
  let mut norm;
  let mut cf;
  if b2 == 0.0 {
    a2 = 0.0;
    norm = 1.0;
    cf = 1.0;
  } else {
    a2 = 1.0;
    norm = 1.0 / b2;
    cf = a2 * norm;
  }
  let mut rm = 1.0;
  while rm < 50_000.0 {
    let apl2m = a + 2.0 * rm;
    let d2m = rm * (b - rm) * x / ((apl2m - 1.0) * apl2m);
    let d2m1 = -(a + rm) * (a + b + rm) * x / (apl2m * (apl2m + 1.0));
    a1 = (a2 + d2m * a1) * norm;
    b1 = (b2 + d2m * b1) * norm;
    a2 = a1 + d2m1 * a2 * norm;
    b2 = b1 + d2m1 * b2 * norm;
    if b2 != 0.0 {
      norm = 1.0 / b2;
      let next = a2 * norm;
      if (cf - next).abs() < cf.abs() * f64::EPSILON {
        return next;
      }
      cf = next;
    }
    rm += 1.0;
  }
  cf
}

fn beta_dist(x_in: f64, alpha: f64, beta_value: f64) -> f64 {
  if x_in <= 0.0 {
    return 0.0;
  }
  if x_in >= 1.0 {
    return 1.0;
  }
  if beta_value == 1.0 {
    return x_in.powf(alpha);
  }
  if alpha == 1.0 {
    return -((beta_value * (1.0 - x_in).ln()).exp_m1());
  }
  let mut x = x_in;
  let mut y = 1.0 - x_in;
  let mut a = alpha;
  let mut b = beta_value;
  let reflect = x_in > alpha / (alpha + beta_value);
  if reflect {
    a = beta_value;
    b = alpha;
    x = y;
    y = x_in;
  }
  let mut result = beta_cont_frac(x, a, b) / a;
  let p = a / (a + b);
  let q = b / (a + b);
  let factor = if a > 1.0 && b > 1.0 && p < 0.97 && q < 0.97 {
    beta_dist_pdf(x, a, b) * x * y
  } else {
    (a * x.ln() + b * y.ln() - log_beta(a, b)).exp()
  };
  result *= factor;
  if reflect {
    result = 1.0 - result;
  }
  result.clamp(0.0, 1.0)
}

fn gamma_cont_fraction(a: f64, x: f64) -> f64 {
  let big_inv = f64::EPSILON;
  let big = 1.0 / big_inv;
  let mut count = 0.0;
  let mut y = 1.0 - a;
  let mut denom = x + 2.0 - a;
  let mut pkm1 = x + 1.0;
  let mut pkm2 = 1.0;
  let mut qkm1 = denom * x;
  let mut qkm2 = x;
  let mut approx = pkm1 / qkm1;
  while count < 10_000.0 {
    count += 1.0;
    y += 1.0;
    let num = y * count;
    denom += 2.0;
    let pk = pkm1 * denom - pkm2 * num;
    let qk = qkm1 * denom - qkm2 * num;
    if qk != 0.0 {
      let next = pk / qk;
      if ((approx - next) / next).abs() <= f64::EPSILON {
        return next;
      }
      approx = next;
    }
    pkm2 = pkm1;
    pkm1 = pk;
    qkm2 = qkm1;
    qkm1 = qk;
    if pk.abs() > big {
      pkm2 *= big_inv;
      pkm1 *= big_inv;
      qkm2 *= big_inv;
      qkm1 *= big_inv;
    }
  }
  approx
}

fn gamma_series(a: f64, x: f64) -> f64 {
  let mut denom = a;
  let mut summand = 1.0 / a;
  let mut sum = summand;
  for _ in 1..=10_000 {
    denom += 1.0;
    summand = summand * x / denom;
    sum += summand;
    if (summand / sum).abs() <= f64::EPSILON {
      break;
    }
  }
  sum
}

fn lower_reg_igamma(a: f64, x: f64) -> f64 {
  if x <= 0.0 {
    return 0.0;
  }
  let factor = (a * x.ln() - x - log_gamma(a)).exp();
  if x > a + 1.0 {
    1.0 - factor * gamma_cont_fraction(a, x)
  } else {
    factor * gamma_series(a, x)
  }
}

fn upper_reg_igamma(a: f64, x: f64) -> f64 {
  if x <= 0.0 {
    return 1.0;
  }
  let factor = (a * x.ln() - x - log_gamma(a)).exp();
  if x > a + 1.0 {
    factor * gamma_cont_fraction(a, x)
  } else {
    1.0 - factor * gamma_series(a, x)
  }
}

fn gamma_dist_pdf(x: f64, alpha: f64, lambda: f64) -> f64 {
  if x < 0.0 {
    0.0
  } else if x == 0.0 {
    if alpha == 1.0 { 1.0 / lambda } else { 0.0 }
  } else {
    let xr = x / lambda;
    ((alpha - 1.0) * xr.ln() - xr - lambda.ln() - log_gamma(alpha)).exp()
  }
}

fn gamma_dist(x: f64, alpha: f64, lambda: f64) -> f64 {
  if x <= 0.0 {
    0.0
  } else {
    lower_reg_igamma(alpha, x / lambda)
  }
}

fn binom_coeff(n: f64, k: f64) -> f64 {
  let k = k.floor();
  if n < k || k < 0.0 {
    return 0.0;
  }
  if k == 0.0 {
    return 1.0;
  }
  (log_gamma(n + 1.0) - log_gamma(k + 1.0) - log_gamma(n - k + 1.0)).exp()
}

fn binom_dist_pmf(x: f64, n: f64, p: f64) -> f64 {
  if p == 0.0 {
    return if x == 0.0 { 1.0 } else { 0.0 };
  }
  if p == 1.0 {
    return if x == n { 1.0 } else { 0.0 };
  }
  binom_coeff(n, x) * p.powf(x) * (1.0 - p).powf(n - x)
}

fn chisq_dist_pdf(x: f64, df: f64) -> f64 {
  if x <= 0.0 {
    0.0
  } else {
    ((0.5 * df - 1.0) * (0.5 * x).ln() - 0.5 * x - 2.0_f64.ln() - log_gamma(0.5 * df)).exp()
  }
}

fn norm_s_pdf(x: f64) -> f64 {
  (-0.5 * x * x).exp() / (2.0 * std::f64::consts::PI).sqrt()
}

fn norm_s_dist(x: f64) -> f64 {
  0.5 * erfc(-x / 2.0_f64.sqrt())
}

fn norm_s_inv(p: f64) -> f64 {
  // Source: LibreOffice sc/source/core/tool/interpr3.cxx
  // ScInterpreter::gaussinv.
  if p <= 0.0 {
    return f64::NEG_INFINITY;
  }
  if p >= 1.0 {
    return f64::INFINITY;
  }
  let q = p - 0.5;
  if q.abs() <= 0.425 {
    let t = 0.180625 - q * q;
    return q
      * (((((((t * 2509.0809287301227 + 33430.57558358813) * t + 67265.7709270087) * t
        + 45921.95393154987)
        * t
        + 13731.69376550946)
        * t
        + 1971.5909503065514)
        * t
        + 133.14166789178438)
        * t
        + 3.3871328727963665)
      / (((((((t * 5226.495278852855 + 28729.085735721943) * t + 39307.89580009271) * t
        + 21213.794301586596)
        * t
        + 5394.196021424751)
        * t
        + 687.1870074920579)
        * t
        + 42.31333070160091)
        * t
        + 1.0);
  }

  let mut t = if q > 0.0 { 1.0 - p } else { p };
  t = (-t.ln()).sqrt();
  let mut z = if t <= 5.0 {
    t += -1.6;
    (((((((t * 7.745450142783414e-4 + 0.022723844989269185) * t + 0.2417807251774506) * t
      + 1.2704582524523684)
      * t
      + 3.6478483247632045)
      * t
      + 5.769497221460691)
      * t
      + 4.630337846156545)
      * t
      + 1.4234371107496835)
      / (((((((t * 1.0507500716444169e-9 + 5.475938084995345e-4) * t + 0.015198666563616457)
        * t
        + 0.14810397642748008)
        * t
        + 0.6897673349851)
        * t
        + 1.6763848301838038)
        * t
        + 2.053191626637759)
        * t
        + 1.0)
  } else {
    t += -5.0;
    (((((((t * 2.010334399292288e-7 + 2.7115555687434876e-5) * t + 0.0012426609473880784) * t
      + 0.026532189526576123)
      * t
      + 0.2965605718285049)
      * t
      + 1.7848265399172913)
      * t
      + 5.463784911164114)
      * t
      + 6.657904643501104)
      / (((((((t * 2.0442631033899397e-15 + 1.421_511_758_316_446e-7) * t
        + 1.8463183175100547e-5)
        * t
        + 7.868691311456133e-4)
        * t
        + 0.014875361290850615)
        * t
        + 0.1369298809227358)
        * t
        + 0.5998322065558879)
        * t
        + 1.0)
  };
  if q < 0.0 {
    z = -z;
  }
  z
}

fn erf(x: f64) -> f64 {
  libm::erf(x)
}

fn erfc(x: f64) -> f64 {
  libm::erfc(x)
}

fn approx_floor(value: f64) -> f64 {
  approx_value(value).floor()
}

fn approx_ceil(value: f64) -> f64 {
  approx_value(value).ceil()
}

fn rtl_round(value: f64, decimal_places: i32) -> f64 {
  // Source: LibreOffice sal/rtl/math.cxx rtl_math_round with
  // rtl_math_RoundingMode_Corrected.
  if !value.is_finite() || value == 0.0 {
    return value;
  }
  let original = value;
  let sign = value.is_sign_negative();
  let mut value = value.abs();
  if decimal_places >= 0 && (value >= 2_f64.powi(52) || is_representable_integer(value)) {
    return original;
  }
  let mut places = decimal_places;
  let mut factor = 0.0;
  if places != 0 {
    if places > 0 {
      let exponent = ((value.to_bits() >> 52) & 0x7ff) as i32 - 1023;
      let decimals = 52 - exponent;
      if decimals <= 0 {
        return original;
      }
      if decimals < places {
        places = decimals;
      }
    }
    factor = 10_f64.powi(places.abs());
    if factor == 0.0 || (places < 0 && !factor.is_finite()) {
      return 0.0;
    }
    if !factor.is_finite() {
      return original;
    }
    if places < 0 {
      value /= factor;
    } else {
      value *= factor;
    }
    if !value.is_finite() {
      return original;
    }
  }
  if value < 2_f64.powi(52) {
    value = approx_floor(value + 0.5);
  }
  if places != 0 {
    if places < 0 {
      value *= factor;
    } else {
      value /= factor;
    }
  }
  if !value.is_finite() {
    return original;
  }
  if sign { -value } else { value }
}

fn approx_value(value: f64) -> f64 {
  // Source: LibreOffice include/rtl/math.hxx approxFloor/approxCeil and
  // sal/rtl/math.cxx rtl_math_approxValue.
  const BIG_INT: f64 = 2_199_023_255_552.0; // 2^41
  if value == 0.0 || !value.is_finite() || value.abs() > BIG_INT {
    return value;
  }
  let sign = value.is_sign_negative();
  let positive = value.abs();
  if positive.fract() == 0.0 || fraction_bit_count(positive) <= 11 {
    return value;
  }
  let exp = 14 - positive.log10().floor() as i32;
  let scale = 10_f64.powi(exp.abs());
  let rounded = if exp < 0 {
    (positive / scale).round() * scale
  } else {
    (positive * scale).round() / scale
  };
  if !rounded.is_finite() {
    return value;
  }
  if sign { -rounded } else { rounded }
}

fn approx_equal(a: f64, b: f64) -> bool {
  // Source: LibreOffice sal/rtl/math.cxx rtl_math_approxEqual.
  const E48: f64 = 3.552713678800501e-15;
  const HALF_15TH_SIGNIFICAND: f64 = 5e-15;
  if a == b {
    return true;
  }
  if a == 0.0 || b == 0.0 || a.is_sign_negative() != b.is_sign_negative() {
    return false;
  }
  let diff = (a - b).abs();
  if !diff.is_finite() {
    return false;
  }
  let a_abs = a.abs();
  let b_abs = b.abs();
  let min_ab = a_abs.min(b_abs);
  let threshold1 = min_ab * E48;
  let threshold2 = 10_f64.powf(min_ab.log10().floor()) * HALF_15TH_SIGNIFICAND;
  if diff >= threshold1.max(threshold2) {
    return false;
  }
  if is_representable_integer(a_abs) && is_representable_integer(b_abs) {
    return false;
  }
  true
}

fn is_representable_integer(value: f64) -> bool {
  value.is_finite() && value.fract() == 0.0
}

fn fraction_bit_count(value: f64) -> u32 {
  if value <= 0.0 || !value.is_finite() {
    return 0;
  }
  let bits = value.to_bits();
  let exponent = ((bits >> 52) & 0x7ff) as i32 - 1023;
  if exponent >= 52 {
    0
  } else if exponent < 0 {
    53
  } else {
    let mask = (1_u64 << (52 - exponent as u32)) - 1;
    (bits & mask).count_ones()
  }
}

fn t_dist(t: f64, df: f64, kind: i32) -> f64 {
  match kind {
    1 => 0.5 * beta_dist(df / (df + t * t), df / 2.0, 0.5),
    2 => beta_dist(df / (df + t * t), df / 2.0, 0.5),
    3 => (1.0 + t * t / df).powf(-(df + 1.0) / 2.0) / (df.sqrt() * beta(0.5, df / 2.0)),
    4 => {
      let x = df / (t * t + df);
      let r = 0.5 * beta_dist(x, 0.5 * df, 0.5);
      if t < 0.0 { r } else { 1.0 - r }
    }
    _ => f64::NAN,
  }
}

fn t_inv_2t(p: f64, df: f64) -> f64 {
  inverse_monotonic(1.0 - p / 2.0, 0.0, 100.0, |x| t_dist(x, df, 4))
}

fn inverse_positive(target: f64, f: impl Fn(f64) -> f64) -> f64 {
  let mut high = 1.0;
  while f(high) < target && high < 1.0e10 {
    high *= 2.0;
  }
  inverse_monotonic(target, 0.0, high, f)
}

fn inverse_monotonic(target: f64, mut low: f64, mut high: f64, f: impl Fn(f64) -> f64) -> f64 {
  for _ in 0..100 {
    let mid = (low + high) / 2.0;
    if f(mid) < target {
      low = mid;
    } else {
      high = mid;
    }
  }
  (low + high) / 2.0
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
  current_address: Option<CellAddress>,
  reference: &str,
) -> Option<Reference> {
  let mut text = reference.trim();
  let external_link_index = parse_external_reference_index(text);
  if external_link_index.is_some() {
    text = text.split_once(']')?.1;
  }
  if let Some((sheet, range)) = text.rsplit_once('!') {
    let sheet = sheet.rsplit(':').next().unwrap_or(sheet).trim_matches('\'');
    let range = parse_formula_a1_range(range)?;
    if external_link_index.is_some() {
      return Some(Reference {
        sheet_index: None,
        external_link_index,
        external_sheet_name: Some(sheet.to_string()),
        range,
      });
    }
    let sheet_index = book.sheet_index(sheet).unwrap_or(default_sheet);
    return Some(Reference {
      sheet_index: Some(sheet_index),
      external_link_index: None,
      external_sheet_name: None,
      range,
    });
  }
  if external_link_index.is_some() {
    return None;
  }
  if let Some(table) = parse_table_reference(book, text, current_address) {
    return Some(table);
  }
  parse_formula_a1_range(text).map(|range| Reference {
    sheet_index: Some(default_sheet),
    external_link_index: None,
    external_sheet_name: None,
    range,
  })
}

fn parse_external_reference_index(reference: &str) -> Option<usize> {
  let (index, _) = reference.strip_prefix('[')?.split_once(']')?;
  let index = index.parse::<usize>().ok()?;
  (index > 0).then_some(index)
}

fn parse_formula_a1_range(reference: &str) -> Option<CellRange> {
  if let Some(range) = CellRange::parse_a1_range(reference) {
    return Some(range);
  }
  let reference = reference.trim();
  let reference = reference
    .rsplit_once('!')
    .map_or(reference, |(_, range)| range)
    .trim_matches('\'');
  let (start, end) = reference.split_once(':')?;
  let start = start.trim_matches('$');
  let end = end.trim_matches('$');
  if start.chars().all(|ch| ch.is_ascii_alphabetic())
    && end.chars().all(|ch| ch.is_ascii_alphabetic())
  {
    return Some(CellRange::new(
      CellAddress {
        col: column_name_to_index(start)?,
        row: 1,
      },
      CellAddress {
        col: column_name_to_index(end)?,
        row: XLSX_MAX_ROW,
      },
    ));
  }
  if start.chars().all(|ch| ch.is_ascii_digit()) && end.chars().all(|ch| ch.is_ascii_digit()) {
    let start_row = start.parse::<u32>().ok()?;
    let end_row = end.parse::<u32>().ok()?;
    if start_row == 0 || end_row == 0 || start_row > XLSX_MAX_ROW || end_row > XLSX_MAX_ROW {
      return None;
    }
    return Some(CellRange::new(
      CellAddress {
        col: 1,
        row: start_row,
      },
      CellAddress {
        col: XLSX_MAX_COLUMN,
        row: end_row,
      },
    ));
  }
  None
}

fn column_name_to_index(name: &str) -> Option<u32> {
  let mut col = 0u32;
  for ch in name.chars() {
    if !ch.is_ascii_alphabetic() {
      return None;
    }
    col = col
      .saturating_mul(26)
      .saturating_add(ch.to_ascii_uppercase() as u32 - 'A' as u32 + 1);
  }
  (col > 0 && col <= XLSX_MAX_COLUMN).then_some(col)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TableReferenceItems(u8);

impl TableReferenceItems {
  const TABLE: Self = Self(0);
  const ALL: Self = Self(1);
  const HEADERS: Self = Self(2);
  const DATA: Self = Self(4);
  const TOTALS: Self = Self(8);
  const THIS_ROW: Self = Self(16);

  fn add(&mut self, item: Self) {
    self.0 |= item.0;
  }

  fn contains(self, item: Self) -> bool {
    self.0 & item.0 != 0
  }
}

fn parse_table_reference(
  book: &FormulaBook,
  text: &str,
  current_address: Option<CellAddress>,
) -> Option<Reference> {
  let (table_name, selector) = text.split_once('[')?;
  let table = book.tables.get(&table_name.to_ascii_uppercase())?;
  let specifiers = table_reference_specifiers(selector)?;
  let mut items = TableReferenceItems::TABLE;
  let mut columns = Vec::new();
  for specifier in specifiers {
    match specifier.to_ascii_uppercase().as_str() {
      "#ALL" => items.add(TableReferenceItems::ALL),
      "#HEADERS" => items.add(TableReferenceItems::HEADERS),
      "#DATA" => items.add(TableReferenceItems::DATA),
      "#TOTALS" => items.add(TableReferenceItems::TOTALS),
      "#THIS ROW" => items.add(TableReferenceItems::THIS_ROW),
      _ => columns.push(specifier),
    }
  }
  let mut range = table_reference_item_range(table, items, current_address)?;
  if !columns.is_empty() {
    let start = table_reference_column_offset(table, &columns[0])?;
    let end = columns
      .last()
      .and_then(|column| table_reference_column_offset(table, column))?;
    let first = start.min(end);
    let last = start.max(end);
    range.start.col = table.range.start.col + first;
    range.end.col = table.range.start.col + last;
  }
  Some(Reference {
    sheet_index: Some(table.sheet_index),
    external_link_index: None,
    external_sheet_name: None,
    range,
  })
}

fn table_reference_item_range(
  table: &TableModel,
  items: TableReferenceItems,
  current_address: Option<CellAddress>,
) -> Option<CellRange> {
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
      col: table.range.start.col,
      row: start_row,
    },
    CellAddress {
      col: table.range.end.col,
      row: end_row,
    },
  ))
}

fn table_reference_column_offset(table: &TableModel, column: &str) -> Option<u32> {
  let column = unescape_table_reference_column(column);
  table
    .columns
    .iter()
    .position(|name| name.eq_ignore_ascii_case(&column))
    .map(|index| index as u32)
}

fn table_reference_specifiers(selector_tail: &str) -> Option<Vec<String>> {
  let selector = selector_tail.trim();
  let selector = selector.strip_suffix(']')?;
  if !selector.starts_with('[') {
    return Some(vec![unescape_table_reference_column(selector)]);
  }
  let mut specifiers = Vec::new();
  let mut depth = 0i32;
  let mut start = None;
  let chars: Vec<char> = selector.chars().collect();
  for (index, ch) in chars.iter().enumerate() {
    match ch {
      '[' => {
        if depth == 0 {
          start = Some(index + 1);
        }
        depth += 1;
      }
      ']' => {
        depth -= 1;
        if depth == 0 {
          let start = start.take()?;
          specifiers.push(chars[start..index].iter().collect::<String>());
        }
      }
      _ => {}
    }
  }
  if depth != 0 {
    return None;
  }
  if specifiers.is_empty() {
    Some(vec![unescape_table_reference_column(selector)])
  } else {
    Some(specifiers)
  }
}

fn unescape_table_reference_column(value: &str) -> String {
  // Source: LibreOffice sc/source/core/tool/compiler.cxx
  // unescapeTableRefColumnSpecifier(): '#', '[', ']' and '\'' are escaped with '\''.
  let mut result = String::new();
  let mut escaped = false;
  for ch in value.chars() {
    if escaped {
      result.push(ch);
      escaped = false;
    } else if ch == '\'' {
      escaped = true;
    } else {
      result.push(ch);
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs::File;

  use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
  use ooxmlsdk::sdk::{
    FileFormatVersion, MarkupCompatibilityProcessMode, MarkupCompatibilityProcessSettings,
    OpenSettings,
  };

  fn test_book() -> FormulaBook {
    let mut cells = HashMap::new();
    let mut formulas = HashMap::new();
    let mut hidden_rows = HashSet::new();
    let sheet_names = vec!["Sheet1".to_string(), "Sheet2".to_string()];

    for (reference, value) in [
      ("F2", 2.0),
      ("F3", 1.5),
      ("F4", 2.0),
      ("F5", 2.0 / 15.0),
      ("F6", 20.0 / 15.0),
      ("F7", 2.0),
      ("F8", 2.0),
      ("F9", 4.0),
      ("F10", 2.0),
      ("G2", 44.0),
      ("G3", 20.0 / 15.0),
      ("G4", 5.0),
      ("G5", 1.0),
      ("G6", 2.0),
      ("G7", 6.0),
      ("G8", 6.6),
      ("G9", 8.0),
      ("G10", 1.0),
      ("H3", 39448.0),
      ("H4", 39508.0),
      ("H5", 39751.0),
      ("F44", 41709.0),
      ("F45", 41733.0),
      ("G44", 41714.0),
      ("G45", 41733.0),
      ("G46", 41718.0),
      ("G47", 41640.0),
    ] {
      cells.insert(
        (0, CellAddress::parse_a1(reference).unwrap()),
        Value::Number(value),
      );
    }

    for (reference, value) in [
      ("C1", 15.0),
      ("C2", 77.0),
      ("C3", 30.0),
      ("C4", 28.0),
      ("C5", 31.0),
      ("C6", 96.0),
      ("C7", 77.0),
      ("C8", 53.0),
      ("C9", 34.0),
      ("C10", 12.545454545454545),
      ("C11", 91.0),
      ("D1", 8.0),
      ("D2", 65.0),
      ("D3", 60.0),
      ("D4", 63.0),
      ("D5", 53.0),
      ("D6", 71.0),
      ("D7", 55.0),
      ("D8", 83.0),
      ("D9", -500.0),
      ("D10", 91.0),
      ("D11", 89.0),
      ("E1", 15.0),
      ("E2", 77.0),
      ("E3", 30.0),
      ("E4", 28.0),
      ("E6", 12.0),
      ("E7", 77.0),
      ("E8", 53.0),
      ("E9", 34.0),
      ("E10", 13.0),
      ("E11", 91.0),
    ] {
      cells.insert(
        (1, CellAddress::parse_a1(reference).unwrap()),
        Value::Number(value),
      );
    }
    cells.insert(
      (1, CellAddress::parse_a1("E5").unwrap()),
      Value::Error("#DIV/0!".to_string()),
    );

    hidden_rows.insert((1, 6));
    for reference in ["C10", "C11", "E10", "E11"] {
      formulas.insert(
        (1, CellAddress::parse_a1(reference).unwrap()),
        FormulaText {
          text: if reference.ends_with("10") {
            "AGGREGATE(1,4,D1:D11)".to_string()
          } else {
            "SUBTOTAL(4,D1:D11)".to_string()
          },
          is_array: false,
        },
      );
    }

    FormulaBook {
      sheet_names,
      cells,
      formulas,
      hidden_rows,
      filtered_rows: HashSet::new(),
      external_cells: HashMap::new(),
      tables: HashMap::new(),
      defined: DefinedNames::default(),
    }
  }

  fn eval_number(book: &FormulaBook, sheet_index: usize, formula: &str) -> f64 {
    let mut evaluator = Evaluator {
      book,
      sheet_index,
      source_file_name: None,
      current_address: None,
      locals: HashMap::new(),
    };
    match evaluator.eval_formula(formula).unwrap() {
      Value::Number(value) => value,
      value => panic!("expected number for {formula}, got {value:?}"),
    }
  }

  fn eval_bool(book: &FormulaBook, sheet_index: usize, formula: &str) -> bool {
    let mut evaluator = Evaluator {
      book,
      sheet_index,
      source_file_name: None,
      current_address: None,
      locals: HashMap::new(),
    };
    match evaluator.eval_formula(formula).unwrap() {
      Value::Bool(value) => value,
      value => panic!("expected bool for {formula}, got {value:?}"),
    }
  }

  fn eval_value(book: &FormulaBook, sheet_index: usize, formula: &str) -> Value {
    let mut evaluator = Evaluator {
      book,
      sheet_index,
      source_file_name: None,
      current_address: None,
      locals: HashMap::new(),
    };
    evaluator.eval_formula(formula).unwrap()
  }

  #[test]
  fn structured_reference_this_row_preserves_comma_column_names() {
    let mut book = test_book();
    book.tables.insert(
      "MYTABLE1".to_string(),
      TableModel {
        sheet_index: 0,
        range: CellRange::parse_a1_range("A1:C3").unwrap(),
        header_rows: 1,
        totals_rows: 0,
        columns: vec![
          "This is the first column".to_string(),
          "This is the,second column".to_string(),
          "Summing".to_string(),
        ],
      },
    );
    for (reference, value) in [("A2", 12.0), ("B2", 23.0), ("A3", 36.0), ("B3", 45.0)] {
      book.cells.insert(
        (0, CellAddress::parse_a1(reference).unwrap()),
        Value::Number(value),
      );
    }
    let mut evaluator = Evaluator {
      book: &book,
      sheet_index: 0,
      source_file_name: None,
      current_address: Some(CellAddress::parse_a1("C3").unwrap()),
      locals: HashMap::new(),
    };
    assert_eq!(
      evaluator
        .eval_formula(
          "MyTable1[[#This Row],[This is the first column]]+MyTable1[[#This Row],[This is the,second column]]"
        )
        .unwrap()
        .number(&book),
      Some(81.0)
    );
  }

  #[test]
  fn structured_reference_items_match_table_sections() {
    let mut book = test_book();
    book.tables.insert(
      "MYDATA".to_string(),
      TableModel {
        sheet_index: 0,
        range: CellRange::parse_a1_range("B4:D15").unwrap(),
        header_rows: 1,
        totals_rows: 1,
        columns: vec![
          "Surname".to_string(),
          "Count".to_string(),
          "Region".to_string(),
        ],
      },
    );

    let headers =
      parse_qualified_reference(&book, 0, None, "myData[#Headers]").expect("headers ref");
    assert_eq!(headers.range, CellRange::parse_a1_range("B4:D4").unwrap());
    let data = parse_qualified_reference(&book, 0, None, "myData[#Data]").expect("data ref");
    assert_eq!(data.range, CellRange::parse_a1_range("B5:D14").unwrap());
    let totals = parse_qualified_reference(&book, 0, None, "myData[#Totals]").expect("totals ref");
    assert_eq!(totals.range, CellRange::parse_a1_range("B15:D15").unwrap());
  }

  #[test]
  fn offset_counts_whole_column_reference_like_calc() {
    let mut book = test_book();
    for (reference, value) in [
      ("A1", Value::Text("a".to_string())),
      ("A2", Value::Text("b".to_string())),
      ("A3", Value::Text("c".to_string())),
      ("A4", Value::Text("d".to_string())),
      ("A5", Value::Text("e".to_string())),
      ("B1", Value::Number(1.0)),
      ("B2", Value::Number(2.0)),
      ("B3", Value::Number(3.0)),
      ("B4", Value::Number(4.0)),
      ("B5", Value::Number(5.0)),
      ("B6", Value::Number(6.0)),
      ("B7", Value::Number(7.0)),
      ("B8", Value::Number(8.0)),
    ] {
      book
        .cells
        .insert((0, CellAddress::parse_a1(reference).unwrap()), value);
    }

    assert_close(eval_number(&book, 0, "COUNTA($A:$A)"), 5.0);
    assert_close(
      eval_number(&book, 0, "SUM(OFFSET($B$1,3,0,COUNTA($A:$A)))"),
      30.0,
    );
  }

  #[test]
  fn external_vlookup_uses_ooxml_external_link_index() {
    let mut book = test_book();
    book.cells.insert(
      (0, CellAddress::parse_a1("B1").unwrap()),
      Value::Text("C".to_string()),
    );
    for (link_index, result) in [(1, 111.0), (2, 222.0)] {
      for (reference, value) in [
        ("A1", Value::Text("A".to_string())),
        ("B1", Value::Number(10.0)),
        ("A2", Value::Text("B".to_string())),
        ("B2", Value::Number(20.0)),
        ("A3", Value::Text("C".to_string())),
        ("B3", Value::Number(result)),
      ] {
        book.external_cells.insert(
          (
            link_index,
            "SHEET1".to_string(),
            CellAddress::parse_a1(reference).unwrap(),
          ),
          value,
        );
      }
    }

    assert_close(
      eval_number(&book, 0, "VLOOKUP(B1,[1]Sheet1!A1:B3,2)"),
      111.0,
    );
    assert_close(
      eval_number(&book, 0, "VLOOKUP(B1,[2]Sheet1!A1:B3,2)"),
      222.0,
    );
  }

  #[test]
  fn let_matches_libreoffice_argument_and_scope_rules() {
    let book = test_book();
    assert_close(eval_number(&book, 0, "LET(x,1,x+2)"), 3.0);
    assert_close(eval_number(&book, 0, "LET(x,1,LET(x,2,x)+x)"), 3.0);
    assert_eq!(
      eval_value(&book, 0, "LET(x,1,x,2,x)"),
      Value::Error("#VALUE!".to_string())
    );
    assert_eq!(
      eval_value(&book, 0, "LET(x,1,y,2)"),
      Value::Error("#VALUE!".to_string())
    );
  }

  fn assert_close(actual: f64, expected: f64) {
    assert!(
      (actual - expected).abs() <= 1.0e-9,
      "expected {expected}, got {actual}"
    );
  }

  fn imported_cell_text(sheet: &CalcSheet, reference: &str) -> String {
    cell_at(sheet, CellAddress::parse_a1(reference).unwrap())
      .map(|cell| cell.display_text.clone())
      .unwrap_or_default()
  }

  #[test]
  fn functions_excel_2010_date_functions_match_libreoffice() {
    let book = test_book();
    for (formula, expected) in [
      ("NETWORKDAYS.INTL(H3,H5,1,H4)", 218.0),
      ("NETWORKDAYS.INTL(F44,F45,\"1001000\")", 18.0),
      ("NETWORKDAYS.INTL(F44,F45)", 19.0),
      ("NETWORKDAYS.INTL(F44,F45,2,G44:G47)", 17.0),
      ("WORKDAY.INTL(H3,H5,1,H4)", 95099.0),
      ("WORKDAY.INTL(F44,24)", 41743.0),
      ("WORKDAY.INTL(F44,24,,G44:G47)", 41745.0),
      ("WORKDAY.INTL(F44,24,13,G44:G47)", 41740.0),
      ("WORKDAY.INTL(F44,24,\"0101010\",G44:G47)", 41754.0),
    ] {
      assert_close(eval_number(&book, 0, formula), expected);
    }
  }

  #[test]
  fn functions_excel_2010_statistical_tests_match_libreoffice() {
    let book = test_book();
    assert!(eval_bool(
      &book,
      0,
      "ROUND(0.42284813280246891,12)=ROUND(0.42284813280246902,12)"
    ));
    assert_eq!(
      translate_shared_formula(
        "ROUND(B2,12)=ROUND(C2,12)",
        CellAddress { col: 4, row: 2 },
        CellAddress { col: 4, row: 3 }
      ),
      "ROUND(B3,12)=ROUND(C3,12)"
    );
    assert_close(eval_number(&book, 0, "ISO.CEILING(G6,F5)"), 2.0);
    assert_close(
      eval_number(&book, 0, "CHISQ.TEST(F2:F10,G2:G10)"),
      1.8744045912597986e-8,
    );
    assert_close(
      eval_number(&book, 0, "F.TEST(F2:F10,G2:G10)"),
      5.814996997636946e-8,
    );
  }

  #[test]
  fn imported_functions_excel_2010_recalculates_equal_column_like_libreoffice() {
    // Source: ../core/sc/qa/unit/subsequent_export_test3.cxx:testFunctionsExcel2010XLSX
    let settings = OpenSettings {
      markup_compatibility_process_settings: MarkupCompatibilityProcessSettings {
        process_mode: MarkupCompatibilityProcessMode::ProcessLoadedPartsOnly,
        target_file_format_version: FileFormatVersion::Microsoft365,
      },
      ..Default::default()
    };
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
      .join("../../test-data/ooxmlsdk-pdf-test/libreoffice/xlsx/functions-excel-2010.xlsx");
    let mut document =
      SpreadsheetDocument::new_with_settings(File::open(path).unwrap(), settings).unwrap();
    let import = super::super::import::ExcelImport::import_document(
      &mut document,
      &crate::options::PdfOptions::default(),
    )
    .unwrap();
    let sheet = &import.sheets[0];
    assert_eq!(imported_cell_text(sheet, "B10"), "2");
    assert_eq!(imported_cell_text(sheet, "D3"), "TRUE");
    assert_eq!(imported_cell_text(sheet, "D10"), "TRUE");
  }

  #[test]
  fn aggregate_options_match_libreoffice_hidden_error_and_nested_rules() {
    let book = test_book();
    for (formula, expected) in [
      ("AGGREGATE(1,0,Sheet2!C1:C11)", 49.0),
      ("AGGREGATE(1,1,Sheet2!C1:C11)", 43.125),
      ("AGGREGATE(1,4,Sheet2!C1:C11)", 49.504_132_231_404_95),
      ("AGGREGATE(1,6,Sheet2!E1:E11)", 43.0),
      ("AGGREGATE(14,0,Sheet2!C1:C11,2)", 77.0),
      ("AGGREGATE(15,0,Sheet2!C1:C11,2)", 28.0),
      ("AGGREGATE(16,0,Sheet2!C1:C11,0.4)", 31.6),
      ("AGGREGATE(17,0,Sheet2!C1:C11,1)", 30.0),
      ("AGGREGATE(18,0,Sheet2!C1:C11,0.8)", 77.0),
      ("AGGREGATE(19,0,Sheet2!C1:C11,1)", 29.0),
    ] {
      assert_close(eval_number(&book, 0, formula), expected);
    }
  }
}
