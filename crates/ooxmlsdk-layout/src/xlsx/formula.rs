use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap, HashSet};

use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use super::styles::{DefinedNameBuiltin, DefinedNamesCatalog};
use super::workbook_catalog::WorkbookCatalog;
use super::worksheet::{CalcCell, CalcSheet, CellAddress, CellRange};

pub(crate) fn recalculate_formula_cells(
  sheets: &mut [CalcSheet],
  defined_names: &DefinedNamesCatalog,
  source_file_name: Option<&str>,
  workbook_catalog: &WorkbookCatalog,
) {
  let defined = DefinedNames::from_catalog(defined_names);
  apply_named_array_formulas(sheets, &defined);
  let formulas = sheets.iter().map(formula_cells).collect::<Vec<_>>();
  let mut book = FormulaBook::from_sheets(sheets, &defined, workbook_catalog);
  let mut formula_book = formula_evaluation_book_from_calc_book(&book, source_file_name);

  for _ in 0..12 {
    let mut changed = false;
    let mut changed_cells = Vec::new();
    let mut refresh_all_cells = false;
    formula_book.with_lookup_cache(|| {
      let mut sheet_index = 0;
      while sheet_index < sheets.len() {
        for formula_cell in &formulas[sheet_index] {
          if formula_contains_smart_quote(&formula_cell.formula) {
            continue;
          }
          let current_sheet = formula_book_sheet_id(&book, sheet_index);
          let current_cell = formula_address(formula_cell.address);
          let Some(value) =
            evaluate_formula_cell(&formula_book, current_sheet, current_cell, formula_cell)
              .map(|value| calc_value_from_formula_value(&book, value))
              .or_else(|| unresolved_foreign_addin_value(&formula_cell.formula))
          else {
            continue;
          };
          let value = lo_pdf_formula_value(&formula_cell.formula, value);
          if let Some(range) = formula_cell
            .reference
            .as_deref()
            .and_then(CellRange::parse_a1_range)
            && apply_array_formula_result(&book, &mut sheets[sheet_index], range, &value)
          {
            changed = true;
            refresh_all_cells = true;
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
            changed_cells.push((sheet_index, formula_cell.address));
          }
        }
        sheet_index += 1;
      }
    });
    if !changed {
      break;
    }
    refresh_formula_book_cells(
      sheets,
      &mut book,
      &mut formula_book,
      if refresh_all_cells {
        None
      } else {
        Some(&changed_cells)
      },
    );
  }
}

fn refresh_formula_book_cells(
  sheets: &[CalcSheet],
  book: &mut FormulaBook,
  formula_book: &mut ooxmlsdk_formula::FormulaEvaluationBook<'static>,
  changed_cells: Option<&[(usize, CellAddress)]>,
) {
  let all_cells;
  let changed_cells = if let Some(changed_cells) = changed_cells {
    changed_cells
  } else {
    all_cells = book.cells.keys().copied().collect::<Vec<_>>();
    &all_cells
  };
  for &(sheet_index, address) in changed_cells {
    let Some(cell) = sheets
      .get(sheet_index)
      .and_then(|sheet| sheet.cell_at(address))
    else {
      continue;
    };
    let value = formula_cell_value(cell);
    book.cells.insert((sheet_index, address), value.clone());
    formula_book.cells.insert(
      (
        formula_book_sheet_id(book, sheet_index),
        formula_address(address),
      ),
      formula_value_from_calc_value(&value),
    );
  }
}

fn evaluate_formula_cell<'doc>(
  book: &ooxmlsdk_formula::FormulaEvaluationBook<'doc>,
  current_sheet: ooxmlsdk_formula::SheetId,
  current_cell: ooxmlsdk_formula::CellAddress,
  formula_cell: &'doc FormulaCell,
) -> Option<ooxmlsdk_formula::FormulaValue<'doc>> {
  if formula_cell.is_array {
    let parsed = ooxmlsdk_formula::parse_formula_text(
      current_sheet,
      Cow::Borrowed(formula_cell.formula.as_str()),
    );
    return book.evaluate_parsed_formula_raw(current_sheet, Some(current_cell), &parsed, true);
  }
  book.evaluate_formula_text(current_sheet, Some(current_cell), &formula_cell.formula)
}

pub(crate) fn evaluate_relative_formula_as_condition(
  import: &super::import::ExcelImport,
  sheet: &CalcSheet,
  formula: &str,
  base: CellAddress,
  address: CellAddress,
) -> bool {
  let Some(sheet_index) = calc_sheet_index(import, sheet) else {
    return false;
  };
  let relative_formula_context = import.relative_formula_context();
  let formula_book = &relative_formula_context.book;
  match formula_book.evaluate_relative_formula_text(
    relative_formula_context.sheet_id(sheet_index),
    formula,
    formula_address(base),
    formula_address(address),
  ) {
    Some(value) => formula_value_truthy(formula_book, &value),
    None => false,
  }
}

pub(crate) fn evaluate_relative_formula_as_number(
  import: &super::import::ExcelImport,
  sheet: &CalcSheet,
  formula: &str,
  base: CellAddress,
  address: CellAddress,
) -> Option<f64> {
  let sheet_index = calc_sheet_index(import, sheet)?;
  let relative_formula_context = import.relative_formula_context();
  let formula_book = &relative_formula_context.book;
  formula_book
    .evaluate_relative_formula_text(
      relative_formula_context.sheet_id(sheet_index),
      formula,
      formula_address(base),
      formula_address(address),
    )
    .and_then(|value| formula_value_number(formula_book, &value))
}

#[derive(Debug)]
pub(crate) struct RelativeFormulaEvaluationContext {
  sheet_workbook_indices: Vec<usize>,
  book: ooxmlsdk_formula::FormulaEvaluationBook<'static>,
}

impl RelativeFormulaEvaluationContext {
  pub(crate) fn from_import(
    sheets: &[CalcSheet],
    defined_names: &DefinedNamesCatalog,
    workbook_catalog: &WorkbookCatalog,
  ) -> Self {
    let defined = DefinedNames::from_catalog(defined_names);
    let calc_book = FormulaBook::from_sheets(sheets, &defined, workbook_catalog);
    let sheet_workbook_indices = calc_book.sheet_workbook_indices.clone();
    let book = formula_evaluation_book_from_calc_book(&calc_book, None);
    Self {
      sheet_workbook_indices,
      book,
    }
  }

  fn sheet_id(&self, sheet_index: usize) -> ooxmlsdk_formula::SheetId {
    ooxmlsdk_formula::SheetId(
      self
        .sheet_workbook_indices
        .get(sheet_index)
        .copied()
        .unwrap_or(sheet_index) as u32,
    )
  }
}

fn calc_sheet_index(import: &super::import::ExcelImport, sheet: &CalcSheet) -> Option<usize> {
  import
    .sheets
    .iter()
    .position(|candidate| std::ptr::eq(candidate, sheet))
    .or_else(|| {
      import.sheets.iter().position(|candidate| {
        candidate.workbook_index == sheet.workbook_index && candidate.name == sheet.name
      })
    })
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
      let Some(array) = book.defined.array(sheet_index, formula) else {
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
  sheet.cell_at(address)
}

fn cell_at_mut(sheet: &mut CalcSheet, address: CellAddress) -> Option<&mut CalcCell> {
  sheet.cell_at_mut(address)
}

fn replace_cell_value(sheet: &mut CalcSheet, address: CellAddress, value: &Value) -> bool {
  let Some(cell) = cell_at_mut(sheet, address) else {
    return false;
  };
  let data_type = formula_value_data_type(value);
  let Some(display_text) = value.clone().display_text() else {
    return false;
  };
  let cached_value = value.cached_text();
  let changed = cell.display_text != display_text
    || cell.cached_value != cached_value
    || cell.data_type != data_type;
  if changed {
    // The cached `t` attribute describes the old formula result. Once the
    // formula is recalculated, alignment and number formatting must follow
    // the newly evaluated scalar type rather than that stale cache type.
    cell.data_type = data_type;
    cell.display_text = display_text;
    cell.cached_value = cached_value;
  }
  changed
}

fn formula_value_data_type(value: &Value) -> Option<x::CellValues> {
  match value {
    Value::Number(_) => Some(x::CellValues::Number),
    Value::Text(_) => Some(x::CellValues::String),
    Value::Bool(_) => Some(x::CellValues::Boolean),
    Value::Error(_) => Some(x::CellValues::Error),
    Value::Blank => None,
    Value::Range(_) | Value::Matrix(_) => None,
  }
}

fn unresolved_foreign_addin_value(formula: &str) -> Option<Value> {
  let formula = formula.trim().trim_start_matches('=').trim_start();
  let function_name = formula.split_once('(')?.0.trim();
  function_name
    .get(.."com.sun.star.sheet.addin.".len())
    .filter(|prefix| prefix.eq_ignore_ascii_case("com.sun.star.sheet.addin."))
    // MS-XLSX permits dots in a user-defined function name. LibreOffice's
    // service name is therefore syntactically a UDF, but it has no matching
    // workbook definition or Office add-in and Excel recalculates it as
    // #NAME?. Keep cached values for every other unparsed formula.
    .map(|_| Value::Error("#NAME?".to_string()))
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

fn formula_contains_smart_quote(formula: &str) -> bool {
  formula
    .chars()
    .any(|ch| matches!(ch, '\u{2018}' | '\u{2019}' | '\u{201c}' | '\u{201d}'))
}

fn lo_pdf_formula_value(formula: &str, value: Value) -> Value {
  if matches!(value, Value::Error(ref error) if error == "#NUM!")
    && is_legacy_ceiling_floor_formula(formula)
  {
    return Value::Error("Err:502".to_string());
  }
  value
}

fn is_legacy_ceiling_floor_formula(formula: &str) -> bool {
  let trimmed = formula.trim().trim_start_matches('=').trim_start();
  formula_starts_with_function(trimmed, "CEILING") || formula_starts_with_function(trimmed, "FLOOR")
}

fn formula_starts_with_function(formula: &str, name: &str) -> bool {
  let Some(prefix) = formula.get(..name.len()) else {
    return false;
  };
  prefix.eq_ignore_ascii_case(name)
    && formula
      .get(name.len()..)
      .is_some_and(|rest| rest.trim_start().starts_with('('))
}

#[derive(Clone, Debug)]
struct FormulaBook {
  sheet_names: Vec<String>,
  sheet_workbook_indices: Vec<usize>,
  cells: BTreeMap<(usize, CellAddress), Value>,
  formulas: BTreeMap<(usize, CellAddress), FormulaText>,
  hidden_rows: HashSet<(usize, u32)>,
  filtered_rows: HashSet<(usize, u32)>,
  external_cells: HashMap<(usize, String, CellAddress), Value>,
  external_defined_names: HashMap<(usize, Option<String>, String), String>,
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
  names: HashMap<(Option<u32>, String), String>,
  arrays: HashMap<(Option<u32>, String), Vec<Vec<Value>>>,
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
      let scoped_key = (record.local_sheet_id, key);
      if let Some(array) = parse_array_constant(&record.formula) {
        arrays.insert(scoped_key.clone(), array);
      }
      names.insert(scoped_key, record.formula.clone());
    }
    Self { names, arrays }
  }

  fn array(&self, sheet_index: usize, name: &str) -> Option<&Vec<Vec<Value>>> {
    let name = name.to_ascii_uppercase();
    self
      .arrays
      .get(&(Some(sheet_index as u32), name.clone()))
      .or_else(|| self.arrays.get(&(None, name)))
  }
}

impl FormulaBook {
  fn from_sheets(
    sheets: &[CalcSheet],
    defined: &DefinedNames,
    workbook_catalog: &WorkbookCatalog,
  ) -> Self {
    let mut cells = BTreeMap::new();
    let mut formulas = BTreeMap::new();
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
      sheet_workbook_indices: sheets.iter().map(|sheet| sheet.workbook_index).collect(),
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
      external_defined_names: workbook_catalog
        .external_defined_names
        .iter()
        .map(|name| {
          (
            (
              name.link_index,
              name
                .sheet_name
                .as_ref()
                .map(|sheet| sheet.to_ascii_uppercase()),
              name.name.to_ascii_uppercase(),
            ),
            name.formula.clone(),
          )
        })
        .collect(),
      tables,
      defined: defined.clone(),
    }
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
}

fn formula_evaluation_book_from_calc_book(
  book: &FormulaBook,
  source_file_name: Option<&str>,
) -> ooxmlsdk_formula::FormulaEvaluationBook<'static> {
  ooxmlsdk_formula::FormulaEvaluationBook {
    source_file_name: source_file_name.map(|name| Cow::Owned(name.to_string())),
    sheet_names: book
      .sheet_names
      .iter()
      .enumerate()
      .map(|(index, name)| ooxmlsdk_formula::SheetBinding {
        id: formula_book_sheet_id(book, index),
        name: Cow::Owned(name.clone()),
      })
      .collect(),
    cells: book
      .cells
      .iter()
      .map(|((sheet, address), value)| {
        (
          (
            formula_book_sheet_id(book, *sheet),
            formula_address(*address),
          ),
          formula_value_from_calc_value(value),
        )
      })
      .collect(),
    formulas: book
      .formulas
      .iter()
      .map(|((sheet, address), formula)| {
        (
          (
            formula_book_sheet_id(book, *sheet),
            formula_address(*address),
          ),
          ooxmlsdk_formula::FormulaText {
            text: Cow::Owned(formula.text.clone()),
            kind: if formula.is_array {
              ooxmlsdk_formula::FormulaKind::Array
            } else {
              ooxmlsdk_formula::FormulaKind::Normal
            },
            reference: None,
          },
        )
      })
      .collect(),
    defined_names: book
      .defined
      .names
      .iter()
      .map(|((sheet, name), formula)| {
        (
          ooxmlsdk_formula::DefinedNameKey {
            sheet: sheet.map(|sheet| formula_sheet_id(sheet as usize)),
            name_upper: name.clone(),
          },
          Cow::Owned(formula.clone()),
        )
      })
      .collect(),
    defined_arrays: book
      .defined
      .arrays
      .iter()
      .map(|((sheet, name), rows)| {
        (
          ooxmlsdk_formula::DefinedNameKey {
            sheet: sheet.map(|sheet| formula_sheet_id(sheet as usize)),
            name_upper: name.clone(),
          },
          rows
            .iter()
            .map(|row| row.iter().map(formula_value_from_calc_value).collect())
            .collect(),
        )
      })
      .collect(),
    external_cached_cells: book
      .external_cells
      .iter()
      .map(|((link_index, sheet_name, address), value)| {
        (
          (*link_index, sheet_name.clone(), formula_address(*address)),
          formula_value_from_calc_value(value),
        )
      })
      .collect(),
    external_defined_names: book
      .external_defined_names
      .iter()
      .map(|((link_index, sheet_name, name), formula)| {
        (
          (*link_index, sheet_name.clone(), name.clone()),
          Cow::Owned(formula.clone()),
        )
      })
      .collect(),
    row_states: formula_row_states(book),
    tables: book
      .tables
      .iter()
      .map(|(name, table)| {
        (
          name.clone(),
          ooxmlsdk_formula::FormulaTable {
            sheet: formula_book_sheet_id(book, table.sheet_index),
            name: Cow::Owned(name.clone()),
            range: formula_range(table.range),
            header_rows: table.header_rows,
            totals_rows: table.totals_rows,
            columns: table
              .columns
              .iter()
              .map(|column| Cow::Owned(column.clone()))
              .collect(),
          },
        )
      })
      .collect(),
    ..ooxmlsdk_formula::FormulaEvaluationBook::default()
  }
}

fn formula_row_states(
  book: &FormulaBook,
) -> BTreeMap<(ooxmlsdk_formula::SheetId, u32), ooxmlsdk_formula::FormulaRowState> {
  let mut states = BTreeMap::new();
  for (sheet, row) in &book.hidden_rows {
    states
      .entry((formula_book_sheet_id(book, *sheet), *row))
      .or_insert_with(ooxmlsdk_formula::FormulaRowState::default)
      .hidden = true;
  }
  for (sheet, row) in &book.filtered_rows {
    states
      .entry((formula_book_sheet_id(book, *sheet), *row))
      .or_insert_with(ooxmlsdk_formula::FormulaRowState::default)
      .filtered = true;
  }
  states
}

fn formula_sheet_id(sheet_index: usize) -> ooxmlsdk_formula::SheetId {
  ooxmlsdk_formula::SheetId(sheet_index as u32)
}

fn formula_book_sheet_id(book: &FormulaBook, sheet_index: usize) -> ooxmlsdk_formula::SheetId {
  ooxmlsdk_formula::SheetId(
    book
      .sheet_workbook_indices
      .get(sheet_index)
      .copied()
      .unwrap_or(sheet_index) as u32,
  )
}

fn formula_address(address: CellAddress) -> ooxmlsdk_formula::CellAddress {
  ooxmlsdk_formula::CellAddress {
    column: address.col.saturating_sub(1),
    row: address.row.saturating_sub(1),
  }
}

fn formula_range(range: CellRange) -> ooxmlsdk_formula::CellRange {
  ooxmlsdk_formula::CellRange {
    start: formula_address(range.start),
    end: formula_address(range.end),
  }
}

fn calc_cell_address(address: ooxmlsdk_formula::CellAddress) -> CellAddress {
  CellAddress {
    col: address.column.saturating_add(1),
    row: address.row.saturating_add(1),
  }
}

fn calc_cell_range(range: ooxmlsdk_formula::CellRange) -> CellRange {
  CellRange {
    start: calc_cell_address(range.start),
    end: calc_cell_address(range.end),
  }
}

fn formula_value_from_calc_value(value: &Value) -> ooxmlsdk_formula::FormulaValue<'static> {
  match value {
    Value::Number(value) => ooxmlsdk_formula::FormulaValue::Number(*value),
    Value::Text(value) => ooxmlsdk_formula::FormulaValue::String(Cow::Owned(value.clone())),
    Value::Bool(value) => ooxmlsdk_formula::FormulaValue::Boolean(*value),
    Value::Error(value) => ooxmlsdk_formula::FormulaValue::Error(formula_error_value(value)),
    Value::Blank => ooxmlsdk_formula::FormulaValue::Blank,
    Value::Matrix(rows) => ooxmlsdk_formula::FormulaValue::Matrix(
      rows
        .iter()
        .map(|row| row.iter().map(formula_value_from_calc_value).collect())
        .collect(),
    ),
    Value::Range(reference) => {
      ooxmlsdk_formula::FormulaValue::Reference(ooxmlsdk_formula::QualifiedRange {
        sheet: reference
          .sheet_index
          .map(formula_sheet_id)
          .unwrap_or_default(),
        sheet_name: None,
        end_sheet_name: None,
        range: formula_range(reference.range),
        start_flags: ooxmlsdk_formula::AddressFlags::default(),
        end_flags: ooxmlsdk_formula::AddressFlags::default(),
      })
    }
  }
}

fn calc_value_from_formula_value(
  book: &FormulaBook,
  value: ooxmlsdk_formula::FormulaValue<'_>,
) -> Value {
  match value {
    ooxmlsdk_formula::FormulaValue::Number(value) => Value::Number(value),
    ooxmlsdk_formula::FormulaValue::String(value) => Value::Text(value.into_owned()),
    ooxmlsdk_formula::FormulaValue::Boolean(value) => Value::Bool(value),
    ooxmlsdk_formula::FormulaValue::Error(value) => {
      Value::Error(calc_error_text(value).to_string())
    }
    ooxmlsdk_formula::FormulaValue::Blank => Value::Blank,
    ooxmlsdk_formula::FormulaValue::Matrix(rows) => Value::Matrix(
      rows
        .into_iter()
        .map(|row| {
          row
            .into_iter()
            .map(|value| calc_value_from_formula_value(book, value))
            .collect()
        })
        .collect(),
    ),
    ooxmlsdk_formula::FormulaValue::Reference(reference) => Value::Range(Reference {
      sheet_index: book
        .sheet_workbook_indices
        .iter()
        .position(|index| *index as u32 == reference.sheet.0),
      external_link_index: None,
      external_sheet_name: None,
      range: calc_cell_range(reference.range),
    }),
    ooxmlsdk_formula::FormulaValue::RefList(mut references) => {
      if references.len() == 1 {
        let reference = references.remove(0);
        Value::Range(Reference {
          sheet_index: book
            .sheet_workbook_indices
            .iter()
            .position(|index| *index as u32 == reference.sheet.0),
          external_link_index: None,
          external_sheet_name: None,
          range: calc_cell_range(reference.range),
        })
      } else {
        Value::Error(calc_error_text(ooxmlsdk_formula::FormulaErrorValue::Value).to_string())
      }
    }
  }
}

fn formula_value_number(
  book: &ooxmlsdk_formula::FormulaEvaluationBook<'_>,
  value: &ooxmlsdk_formula::FormulaValue<'_>,
) -> Option<f64> {
  match value {
    ooxmlsdk_formula::FormulaValue::Number(value) => Some(*value),
    ooxmlsdk_formula::FormulaValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
    ooxmlsdk_formula::FormulaValue::String(value) => value.trim().parse().ok(),
    ooxmlsdk_formula::FormulaValue::Blank => Some(0.0),
    ooxmlsdk_formula::FormulaValue::Matrix(rows) => rows
      .first()
      .and_then(|row| row.first())
      .and_then(|value| formula_value_number(book, value)),
    ooxmlsdk_formula::FormulaValue::Reference(reference) => formula_value_number(
      book,
      &book.cell_value(reference.sheet, reference.range.start),
    ),
    ooxmlsdk_formula::FormulaValue::RefList(references) => {
      let [reference] = references.as_slice() else {
        return None;
      };
      formula_value_number(
        book,
        &book.cell_value(reference.sheet, reference.range.start),
      )
    }
    ooxmlsdk_formula::FormulaValue::Error(_) => None,
  }
}

fn formula_value_truthy(
  book: &ooxmlsdk_formula::FormulaEvaluationBook<'_>,
  value: &ooxmlsdk_formula::FormulaValue<'_>,
) -> bool {
  match value {
    ooxmlsdk_formula::FormulaValue::Boolean(value) => *value,
    ooxmlsdk_formula::FormulaValue::Number(value) => *value != 0.0,
    ooxmlsdk_formula::FormulaValue::String(value) => !value.is_empty(),
    ooxmlsdk_formula::FormulaValue::Matrix(rows) => rows
      .first()
      .and_then(|row| row.first())
      .is_some_and(|value| formula_value_truthy(book, value)),
    ooxmlsdk_formula::FormulaValue::Reference(reference) => formula_value_truthy(
      book,
      &book.cell_value(reference.sheet, reference.range.start),
    ),
    ooxmlsdk_formula::FormulaValue::RefList(references) => {
      let [reference] = references.as_slice() else {
        return false;
      };
      formula_value_truthy(
        book,
        &book.cell_value(reference.sheet, reference.range.start),
      )
    }
    ooxmlsdk_formula::FormulaValue::Error(_) | ooxmlsdk_formula::FormulaValue::Blank => false,
  }
}

fn formula_error_value(value: &str) -> ooxmlsdk_formula::FormulaErrorValue {
  match value {
    "#NULL!" => ooxmlsdk_formula::FormulaErrorValue::Null,
    "#DIV/0!" => ooxmlsdk_formula::FormulaErrorValue::Div0,
    "#VALUE!" => ooxmlsdk_formula::FormulaErrorValue::Value,
    "#REF!" => ooxmlsdk_formula::FormulaErrorValue::Ref,
    "#NAME?" => ooxmlsdk_formula::FormulaErrorValue::Name,
    "#NUM!" => ooxmlsdk_formula::FormulaErrorValue::Num,
    "#N/A" => ooxmlsdk_formula::FormulaErrorValue::NA,
    "#GETTING_DATA" => ooxmlsdk_formula::FormulaErrorValue::GettingData,
    "#SPILL!" => ooxmlsdk_formula::FormulaErrorValue::Spill,
    "#CALC!" => ooxmlsdk_formula::FormulaErrorValue::Calc,
    "Err:502" => ooxmlsdk_formula::FormulaErrorValue::IllegalArgument,
    _ => ooxmlsdk_formula::FormulaErrorValue::Error,
  }
}

fn calc_error_text(value: ooxmlsdk_formula::FormulaErrorValue) -> &'static str {
  match value {
    ooxmlsdk_formula::FormulaErrorValue::Null => "#NULL!",
    ooxmlsdk_formula::FormulaErrorValue::Div0 => "#DIV/0!",
    ooxmlsdk_formula::FormulaErrorValue::Value => "#VALUE!",
    ooxmlsdk_formula::FormulaErrorValue::Ref => "#REF!",
    ooxmlsdk_formula::FormulaErrorValue::Name => "#NAME?",
    ooxmlsdk_formula::FormulaErrorValue::Num => "#NUM!",
    ooxmlsdk_formula::FormulaErrorValue::NA => "#N/A",
    ooxmlsdk_formula::FormulaErrorValue::GettingData => "#GETTING_DATA",
    ooxmlsdk_formula::FormulaErrorValue::Spill => "#SPILL!",
    ooxmlsdk_formula::FormulaErrorValue::Calc => "#CALC!",
    ooxmlsdk_formula::FormulaErrorValue::Error => "#ERROR!",
    ooxmlsdk_formula::FormulaErrorValue::NotImplemented => "#N/IMPL!",
    ooxmlsdk_formula::FormulaErrorValue::CircularReference => "#CIRC!",
    ooxmlsdk_formula::FormulaErrorValue::IllegalChar => "Err:501",
    ooxmlsdk_formula::FormulaErrorValue::IllegalArgument => "Err:502",
    ooxmlsdk_formula::FormulaErrorValue::IllegalParameter => "Err:504",
    ooxmlsdk_formula::FormulaErrorValue::Pair => "Err:507",
    ooxmlsdk_formula::FormulaErrorValue::PairExpected => "Err:508",
    ooxmlsdk_formula::FormulaErrorValue::OperatorExpected => "Err:509",
    ooxmlsdk_formula::FormulaErrorValue::VariableExpected => "Err:510",
    ooxmlsdk_formula::FormulaErrorValue::Parameter => "Err:511",
    ooxmlsdk_formula::FormulaErrorValue::CodeOverflow => "Err:512",
    ooxmlsdk_formula::FormulaErrorValue::StringOverflow => "Err:513",
    ooxmlsdk_formula::FormulaErrorValue::StackOverflow => "Err:514",
    ooxmlsdk_formula::FormulaErrorValue::InvalidVariable => "Err:516",
    ooxmlsdk_formula::FormulaErrorValue::InvalidOpcode => "Err:517",
    ooxmlsdk_formula::FormulaErrorValue::InvalidStackValue => "Err:518",
    ooxmlsdk_formula::FormulaErrorValue::InvalidToken => "Err:520",
    ooxmlsdk_formula::FormulaErrorValue::NoConvergence => "Err:523",
    ooxmlsdk_formula::FormulaErrorValue::NoAddin => "Err:530",
    ooxmlsdk_formula::FormulaErrorValue::NoMacro => "Err:531",
    ooxmlsdk_formula::FormulaErrorValue::NestedArray => "Err:533",
    ooxmlsdk_formula::FormulaErrorValue::MatrixSize => "Err:538",
    ooxmlsdk_formula::FormulaErrorValue::BadArrayContent => "Err:539",
    ooxmlsdk_formula::FormulaErrorValue::LinkFormulaNeedingCheck => "Err:540",
  }
}

fn formula_cell_value(cell: &CalcCell) -> Value {
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
  ooxmlsdk_formula::translate_shared_formula_text(
    formula,
    ooxmlsdk_formula::CellAddress {
      column: origin.col.saturating_sub(1),
      row: origin.row.saturating_sub(1),
    },
    ooxmlsdk_formula::CellAddress {
      column: target.col.saturating_sub(1),
      row: target.row.saturating_sub(1),
    },
  )
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn recalculated_formula_type_follows_the_evaluated_scalar() {
    assert_eq!(
      formula_value_data_type(&Value::Number(5.0)),
      Some(x::CellValues::Number)
    );
    assert_eq!(
      formula_value_data_type(&Value::Bool(true)),
      Some(x::CellValues::Boolean)
    );
    assert_eq!(
      formula_value_data_type(&Value::Error("#NAME?".to_string())),
      Some(x::CellValues::Error)
    );
    assert_eq!(formula_value_data_type(&Value::Blank), None);
  }

  #[test]
  fn unresolved_libreoffice_addin_is_an_office_name_error() {
    assert_eq!(
      unresolved_foreign_addin_value("com.sun.star.sheet.addin.Analysis.getEomonth(A5,1)"),
      Some(Value::Error("#NAME?".to_string()))
    );
    assert_eq!(unresolved_foreign_addin_value("EOMONTH(A5,1)"), None);
    assert_eq!(
      unresolved_foreign_addin_value("_xlfn.XLOOKUP(A1,B:B,C:C)"),
      None
    );
  }

  #[test]
  fn lo_pdf_formula_value_maps_legacy_ceiling_floor_num_to_illegal_argument() {
    assert_eq!(
      lo_pdf_formula_value("CEILING(C$1,$A5)", Value::Error("#NUM!".to_string())),
      Value::Error("Err:502".to_string())
    );
    assert_eq!(
      lo_pdf_formula_value("FLOOR(C$1,$A5)", Value::Error("#NUM!".to_string())),
      Value::Error("Err:502".to_string())
    );
    assert_eq!(
      lo_pdf_formula_value("FLOOR.MATH(C$1,$A5)", Value::Error("#NUM!".to_string())),
      Value::Error("#NUM!".to_string())
    );
  }
}
