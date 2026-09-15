use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap, HashSet};

use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use super::styles::{DefinedNameBuiltin, DefinedNamesCatalog};
use super::workbook_catalog::WorkbookCatalog;
use super::worksheet::{CalcCell, CalcSheet, CellAddress, CellRange};

// Internal convergence guards, not OOXML or LibreOffice-defined values.
const MAX_FORMULA_RECALCULATION_PASSES: usize = 12;
const FORMULA_ZERO_TOLERANCE: f64 = 1.0e-12;

#[derive(Clone, Debug)]
pub(crate) struct FormulaContext {
  date_system: ooxmlsdk_formula::DateSystem,
  today_serial: Option<f64>,
  ui_language: Option<String>,
}

impl FormulaContext {
  pub(crate) fn new(
    date_1904: bool,
    datetime: Option<crate::options::FieldUpdateDateTime>,
    ui_language: Option<&str>,
  ) -> Self {
    let date_system = if date_1904 {
      ooxmlsdk_formula::DateSystem::Date1904
    } else {
      ooxmlsdk_formula::DateSystem::Date1900
    };
    // ECMA-376 §18.17.7.326: TODAY uses the workbook's date base.
    // The supplied timestamp is already local civil time, so no time-zone
    // conversion or time-of-day fraction belongs in this date-only value.
    let today_serial = datetime.and_then(|datetime| {
      ooxmlsdk_formula::calc::datetime::date_serial_with_system(
        datetime.year.into(),
        datetime.month.into(),
        datetime.day.into(),
        date_system,
      )
    });
    Self {
      date_system,
      today_serial,
      ui_language: ui_language.map(str::to_owned),
    }
  }
}

pub(crate) fn recalculate_formula_cells(
  sheets: &mut [CalcSheet],
  defined_names: &DefinedNamesCatalog,
  source_file_name: Option<&str>,
  workbook_catalog: &WorkbookCatalog,
  context: &FormulaContext,
) {
  let defined = DefinedNames::from_catalog(defined_names);
  let formulas = sheets.iter().map(formula_cells).collect::<Vec<_>>();
  let mut book = FormulaBook::from_sheets(sheets, &defined, workbook_catalog);
  let mut formula_book = formula_evaluation_book_from_calc_book(&book, source_file_name, context);

  for _ in 0..MAX_FORMULA_RECALCULATION_PASSES {
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
              .or_else(|| unresolved_external_reference_value(&formula_cell.formula))
              .or_else(|| unresolved_foreign_addin_value(&formula_cell.formula))
          else {
            continue;
          };
          let repeat_scalar = formula_cell.is_array
            && !is_dynamic_array_cell(&sheets[sheet_index], formula_cell.address, workbook_catalog);
          if let Some(range) = formula_cell
            .reference
            .as_deref()
            .and_then(CellRange::parse_a1_range)
            && apply_array_formula_result(
              &book,
              &mut sheets[sheet_index],
              range,
              &value,
              repeat_scalar,
            )
          {
            changed = true;
            refresh_all_cells = true;
            continue;
          }
          if matches!(value, Value::Range(_) | Value::Blank) {
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
    context: &FormulaContext,
  ) -> Self {
    let defined = DefinedNames::from_catalog(defined_names);
    let calc_book = FormulaBook::from_sheets(sheets, &defined, workbook_catalog);
    let sheet_workbook_indices = calc_book.sheet_workbook_indices.clone();
    let book = formula_evaluation_book_from_calc_book(&calc_book, None, context);
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

fn cell_at(sheet: &CalcSheet, address: CellAddress) -> Option<&CalcCell> {
  sheet.cell_at(address)
}

fn is_dynamic_array_cell(
  sheet: &CalcSheet,
  address: CellAddress,
  catalog: &WorkbookCatalog,
) -> bool {
  cell_at(sheet, address)
    .and_then(|cell| cell.cell_metadata_index)
    .is_some_and(|index| {
      catalog
        .relationship_resources
        .cell_metadata
        .as_ref()
        .is_some_and(|metadata| metadata.dynamic_array_indices.contains(&index))
    })
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

fn unresolved_external_reference_value(formula: &str) -> Option<Value> {
  let formula = formula.trim().trim_start_matches('=').trim_start();
  let bang = formula.find('!')?;
  let qualifier = &formula[..bang];
  let open = qualifier.rfind('[')?;
  let close = qualifier[open + 1..].find(']')? + open + 1;
  (close > open + 1).then(|| Value::Error("#REF!".to_string()))
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
  repeat_scalar: bool,
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
        // A one-cell reference is a scalar result of a fixed array too
        // (Office 57798.xlsx). Repeating it must not read neighboring cells.
        Value::Range(reference)
          if repeat_scalar && reference.range.start == reference.range.end =>
        {
          reference_cell_value(book, reference, reference.range.start)
        }
        Value::Range(reference) => reference_cell_value(
          book,
          reference,
          CellAddress {
            col: reference.range.start.col + col_offset as u32,
            row: reference.range.start.row + row_offset as u32,
          },
        ),
        // A legacy CSE formula has a fixed output region. A scalar result
        // fills that region (Office ArrayFormula.xlsx: {=1+2} in A1:B2).
        // XLDAPR dynamic arrays instead spill the result's actual shape.
        Value::Number(_) | Value::Text(_) | Value::Bool(_) | Value::Error(_)
          if repeat_scalar && target.start != target.end =>
        {
          value.clone()
        }
        _ => return false,
      };
      if replace_cell_value(sheet, CellAddress { col, row }, &value) {
        changed = true;
      }
    }
  }
  changed
}

fn formula_contains_smart_quote(formula: &str) -> bool {
  formula
    .chars()
    .any(|ch| matches!(ch, '\u{2018}' | '\u{2019}' | '\u{201c}' | '\u{201d}'))
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
  kind: ooxmlsdk_formula::FormulaKind,
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
}

impl DefinedNames {
  fn from_catalog(catalog: &DefinedNamesCatalog) -> Self {
    let mut names = HashMap::new();
    for record in &catalog.records {
      if record.builtin.is_some()
        || record.hidden
        || record.builtin == Some(DefinedNameBuiltin::PrintArea)
      {
        continue;
      }
      // MS-OI29500 18.2.5 / MS-XLSX 2.2.2.5 require the name-formula
      // grammar. Office removes unqualified cell references from names;
      // constants, other names and explicit sheet-relative `!` remain valid.
      if ooxmlsdk_formula::parse_formula_text(
        ooxmlsdk_formula::SheetId(record.local_sheet_id.unwrap_or(0)),
        record.formula.as_str(),
      )
      .has_unqualified_cell_references()
      {
        continue;
      }
      let key = record.name.to_ascii_uppercase();
      let scoped_key = (record.local_sheet_id, key);
      names.insert(scoped_key, record.formula.clone());
    }
    Self { names }
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
            kind: if !formula.is_array {
              ooxmlsdk_formula::FormulaKind::Normal
            } else if is_dynamic_array_cell(sheet, formula.address, workbook_catalog) {
              ooxmlsdk_formula::FormulaKind::DynamicArray
            } else {
              ooxmlsdk_formula::FormulaKind::Array
            },
            text: formula.formula,
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
  context: &FormulaContext,
) -> ooxmlsdk_formula::FormulaEvaluationBook<'static> {
  ooxmlsdk_formula::FormulaEvaluationBook {
    date_system: context.date_system,
    today_serial: context.today_serial,
    ui_language: context.ui_language.clone().map(Cow::Owned),
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
            kind: formula.kind,
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
    defined_arrays: BTreeMap::new(),
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
      .entry((formula_book_sheet_id(book, *sheet), row.saturating_sub(1)))
      .or_insert_with(ooxmlsdk_formula::FormulaRowState::default)
      .hidden = true;
  }
  for (sheet, row) in &book.filtered_rows {
    states
      .entry((formula_book_sheet_id(book, *sheet), row.saturating_sub(1)))
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
  if cell.data_type == Some(x::CellValues::Boolean) {
    // OOXML caches booleans as 1/0; recalculated values may spell TRUE/FALSE.
    // Keep their type so equality and information functions see a logical value.
    return Value::Bool(text.trim() == "1" || text.trim().eq_ignore_ascii_case("true"));
  }
  if matches!(
    cell.data_type,
    Some(x::CellValues::SharedString | x::CellValues::InlineString | x::CellValues::String)
  ) {
    return Value::Text(text.to_string());
  }
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
  if (value.fract()).abs() < FORMULA_ZERO_TOLERANCE {
    format!("{}", value.round() as i64)
  } else {
    let text = format!("{value:.10}");
    text.trim_end_matches('0').trim_end_matches('.').to_string()
  }
}

fn reference_cell_value(book: &FormulaBook, reference: &Reference, address: CellAddress) -> Value {
  let value = if let Some(sheet_name) = reference.external_sheet_name.as_deref() {
    book.external_cell(reference.external_link_index, sheet_name, address)
  } else {
    reference
      .sheet_index
      .map(|sheet_index| book.cell(sheet_index, address))
      .unwrap_or(Value::Blank)
  };
  match value {
    // Office tdf162093.xlsx: the empty table total becomes 0 when referenced
    // by a spilling formula. Literal empty strings and array padding stay empty.
    Value::Blank if reference.range.contains(address) => Value::Number(0.0),
    value => value,
  }
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn evaluated_zero_replaces_nonzero_cache_and_updates_dependents() {
    use super::super::styles::StylesCatalog;
    use super::super::worksheet::{SheetIdentity, SheetResourceCatalog};
    use ooxmlsdk::sdk::SdkType;
    let worksheet = x::Worksheet::from_bytes(br#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row r="1"><c r="A1"><f>1-1</f><v>425</v></c><c r="B1"><f>50000-A1</f><v>49575</v></c><c r="C1"><f>SUMIF(D1:D1,"no",E1:E1)</f><v>100</v></c><c r="D1" t="str"><v>yes</v></c><c r="E1"><v>100</v></c></row></sheetData></worksheet>"#).unwrap();
    let mut sheets = vec![CalcSheet::from_worksheet(
      SheetIdentity {
        workbook_index: 0,
        name: "Sheet1".into(),
        state: None,
        active: true,
      },
      worksheet,
      SheetResourceCatalog::default(),
      &[],
      &StylesCatalog::default(),
      Default::default(),
    )];
    recalculate_formula_cells(
      &mut sheets,
      &DefinedNamesCatalog::default(),
      None,
      &WorkbookCatalog::default(),
      &FormulaContext::new(false, None, None),
    );
    for (address, expected) in [("A1", "0"), ("B1", "50000"), ("C1", "0")] {
      assert_eq!(
        cell_at(&sheets[0], CellAddress::parse_a1(address).unwrap())
          .unwrap()
          .display_text,
        expected,
        "{address}"
      );
    }
  }

  #[test]
  fn recalculation_omits_invalid_name_references_but_keeps_qualified_and_indirect_ones() {
    use super::super::styles::StylesCatalog;
    use super::super::worksheet::{SheetIdentity, SheetResourceCatalog};
    use ooxmlsdk::sdk::SdkType;

    let workbook = x::Workbook::from_bytes(br#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheets/><definedNames>
      <definedName name="Cnt">2+3</definedName>
      <definedName name="Cnt" localSheetId="0">COUNTA(A:A)</definedName>
      <definedName name="Bad">SUM(A1:A2)</definedName>
      <definedName name="Qualified">Sheet1!$A$1</definedName>
      <definedName name="Relative">!$A$1</definedName>
      <definedName name="Literal">INDIRECT("A1")</definedName>
      </definedNames></workbook>"#).unwrap();
    let worksheet = x::Worksheet::from_bytes(br#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row r="1">
      <c r="A1"><v>7</v></c><c r="B1"><f>Cnt</f><v>1</v></c>
      <c r="C1"><f>Bad</f><v>7</v></c><c r="D1"><f>SUM(Qualified)</f><v>0</v></c>
      <c r="E1"><f>SUM(Relative)</f><v>0</v></c><c r="F1"><f>SUM(Literal)</f><v>0</v></c>
      <c r="G1"><f>SUM(A1)</f><v>0</v></c></row></sheetData></worksheet>"#).unwrap();
    let mut sheets = vec![CalcSheet::from_worksheet(
      SheetIdentity {
        workbook_index: 0,
        name: "Sheet1".into(),
        state: None,
        active: true,
      },
      worksheet,
      SheetResourceCatalog::default(),
      &[],
      &StylesCatalog::default(),
      Default::default(),
    )];
    let names = DefinedNamesCatalog::from_workbook(&workbook);
    recalculate_formula_cells(
      &mut sheets,
      &names,
      None,
      &WorkbookCatalog::default(),
      &FormulaContext::new(false, None, None),
    );
    assert_eq!(names.records.len(), 6, "preserve the source catalog");
    for (address, expected) in [
      ("B1", "5"),
      ("C1", "#NAME?"),
      ("D1", "7"),
      ("E1", "7"),
      ("F1", "7"),
      ("G1", "7"),
    ] {
      assert_eq!(
        sheets[0]
          .cell_at(CellAddress::parse_a1(address).unwrap())
          .unwrap()
          .display_text,
        expected,
        "{address}"
      );
    }
  }

  #[test]
  fn recalculation_preserves_imported_boolean_types() {
    use super::super::styles::StylesCatalog;
    use super::super::worksheet::{SheetIdentity, SheetResourceCatalog};
    use ooxmlsdk::sdk::SdkType;

    for (boolean, logical) in [
      ("1", "TRUE"),
      ("true", "TRUE"),
      ("0", "FALSE"),
      ("false", "FALSE"),
    ] {
      let worksheet = x::Worksheet::from_bytes(format!(r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row r="1">
        <c r="A1" t="b"><v>{boolean}</v></c><c r="B1"><v>1</v></c>
        <c r="C1" t="str"><v>1</v></c><c r="D1" t="b"><f>{logical}()</f><v>1</v></c>
        <c r="E1" t="b"><f>A1=D1</f><v>0</v></c><c r="F1" t="b"><f>ISLOGICAL(A1)</f><v>0</v></c>
        <c r="G1" t="b"><f>ISNUMBER(A1)</f><v>1</v></c><c r="H1"><f>TYPE(A1)</f><v>1</v></c>
        <c r="I1" t="b"><f>A1=B1</f><v>1</v></c><c r="J1" t="b"><f>A1=C1</f><v>1</v></c>
      </row></sheetData></worksheet>"#).as_bytes()).unwrap();
      let sheet = CalcSheet::from_worksheet(
        SheetIdentity {
          workbook_index: 0,
          name: "Sheet1".into(),
          state: None,
          active: true,
        },
        worksheet,
        SheetResourceCatalog::default(),
        &[],
        &StylesCatalog::default(),
        Default::default(),
      );
      let mut sheets = vec![sheet];
      recalculate_formula_cells(
        &mut sheets,
        &DefinedNamesCatalog::default(),
        None,
        &WorkbookCatalog::default(),
        &FormulaContext::new(false, None, None),
      );
      for (address, expected) in [
        ("D1", logical),
        ("E1", "TRUE"),
        ("F1", "TRUE"),
        ("G1", "FALSE"),
        ("H1", "4"),
        ("I1", "FALSE"),
        ("J1", "FALSE"),
      ] {
        assert_eq!(
          sheets[0]
            .cell_at(CellAddress::parse_a1(address).unwrap())
            .unwrap()
            .display_text,
          expected,
          "{boolean}: {address}"
        );
      }
    }
  }

  #[test]
  fn array_recalculation_preserves_empty_strings_and_materializes_referenced_blanks() {
    use super::super::styles::StylesCatalog;
    use super::super::worksheet::{SheetIdentity, SheetResourceCatalog};
    use ooxmlsdk::sdk::SdkType;

    for (formula, expected) in [
      ("A1:C1", ["0", "", "7"]),
      ("A1", ["0", "0", "0"]),
      ("B1", ["", "", ""]),
      (r#"{0,"",7}"#, ["0", "", "7"]),
      (r#"{0,""}"#, ["0", "", ""]),
    ] {
      let worksheet = x::Worksheet::from_bytes(format!(r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row r="1">
        <c r="B1" t="str"><f>""</f><v></v></c><c r="C1"><v>7</v></c>
        <c r="D1"><f t="array" ref="D1:F1">{formula}</f><v>9</v></c>
        <c r="E1"><v>9</v></c><c r="F1"><v>9</v></c>
      </row></sheetData></worksheet>"#).as_bytes()).unwrap();
      let sheet = CalcSheet::from_worksheet(
        SheetIdentity {
          workbook_index: 0,
          name: "Sheet1".into(),
          state: None,
          active: true,
        },
        worksheet,
        SheetResourceCatalog::default(),
        &[],
        &StylesCatalog::default(),
        Default::default(),
      );
      let mut sheets = vec![sheet];
      recalculate_formula_cells(
        &mut sheets,
        &DefinedNamesCatalog::default(),
        None,
        &WorkbookCatalog::default(),
        &FormulaContext::new(false, None, None),
      );
      for (address, expected) in ["D1", "E1", "F1"].into_iter().zip(expected) {
        assert_eq!(
          sheets[0]
            .cell_at(CellAddress::parse_a1(address).unwrap())
            .unwrap()
            .display_text,
          expected,
          "{formula}: {address}"
        );
      }
    }
  }

  #[test]
  fn formulatext_recalculation_uses_ui_language_and_preserves_authored_text() {
    use super::super::styles::StylesCatalog;
    use super::super::worksheet::{SheetIdentity, SheetResourceCatalog};
    use ooxmlsdk::sdk::SdkType;

    for (language, expected) in [
      ("de-DE", "=myData[#Kopfzeilen]"),
      ("en-US", "=myData[#Headers]"),
    ] {
      // FORMULATEXT can also display a formula whose referenced table is absent.
      let worksheet = x::Worksheet::from_bytes(br#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData><row r="1">
        <c r="A1" t="e"><f>myData[#Headers]</f><v>#NAME?</v></c>
        <c r="B1" t="str"><f>_xlfn.FORMULATEXT(A1)</f><v>stale</v></c>
        <c r="C1" t="str"><v>=myData[#Headers]</v></c>
      </row></sheetData></worksheet>"#).unwrap();
      let sheet = CalcSheet::from_worksheet(
        SheetIdentity {
          workbook_index: 0,
          name: "Sheet1".into(),
          state: None,
          active: true,
        },
        worksheet,
        SheetResourceCatalog::default(),
        &[],
        &StylesCatalog::default(),
        Default::default(),
      );
      let mut sheets = vec![sheet];
      recalculate_formula_cells(
        &mut sheets,
        &DefinedNamesCatalog::default(),
        None,
        &WorkbookCatalog::default(),
        &FormulaContext::new(false, None, Some(language)),
      );
      for (address, expected) in [("B1", expected), ("C1", "=myData[#Headers]")] {
        assert_eq!(
          sheets[0]
            .cell_at(CellAddress::parse_a1(address).unwrap())
            .unwrap()
            .display_text,
          expected,
          "{language}: {address}"
        );
      }
    }
  }

  #[test]
  fn formulatext_preserves_dynamic_array_metadata_during_recalculation() {
    use super::super::styles::StylesCatalog;
    use super::super::workbook_catalog::CellMetadataResource;
    use super::super::worksheet::{SheetIdentity, SheetResourceCatalog};
    use ooxmlsdk::sdk::SdkType;

    let worksheet = x::Worksheet::from_bytes(
      br#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData>
      <row r="1">
        <c r="A1"><f t="array" ref="A1">SUM(C1:C2)</f><v>3</v></c>
        <c r="B1" cm="3"><f t="array" ref="B1">SUM(C1:C2)</f><v>3</v></c>
        <c r="C1"><v>1</v></c>
        <c r="D1"><f>C1:C2</f><v>1</v></c>
      </row><row r="2">
        <c r="A2" t="str"><f>_xlfn.FORMULATEXT(A1)</f><v>stale</v></c>
        <c r="B2" t="str"><f>_xlfn.FORMULATEXT(B1)</f><v>stale</v></c>
        <c r="C2"><v>2</v></c>
        <c r="D2" t="str"><f>_xlfn.FORMULATEXT(D1)</f><v>stale</v></c>
      </row><row r="3">
        <c r="A3"><f t="array" ref="A3">C1:C2</f><v>1</v></c>
        <c r="B3" cm="3"><f t="array" ref="B3">C1:C2</f><v>1</v></c>
      </row><row r="4">
        <c r="A4" t="str"><f>_xlfn.FORMULATEXT(A3)</f><v>stale</v></c>
        <c r="B4" t="str"><f>_xlfn.FORMULATEXT(B3)</f><v>stale</v></c>
      </row></sheetData></worksheet>"#,
    )
    .unwrap();
    let sheet = CalcSheet::from_worksheet(
      SheetIdentity {
        workbook_index: 0,
        name: "Sheet1".into(),
        state: None,
        active: true,
      },
      worksheet,
      SheetResourceCatalog::default(),
      &[],
      &StylesCatalog::default(),
      Default::default(),
    );
    let mut sheets = vec![sheet];
    let mut catalog = WorkbookCatalog::default();
    catalog.relationship_resources.cell_metadata = Some(CellMetadataResource {
      dynamic_array_indices: vec![3],
      ..Default::default()
    });
    recalculate_formula_cells(
      &mut sheets,
      &DefinedNamesCatalog::default(),
      None,
      &catalog,
      &FormulaContext::new(false, None, None),
    );
    for (address, expected) in [
      ("A1", "3"),
      ("B1", "3"),
      ("A2", "{=SUM(C1:C2)}"),
      ("B2", "=SUM(C1:C2)"),
      ("D2", "=@C1:C2"),
      ("A4", "{=C1:C2}"),
      ("B4", "=C1:C2"),
    ] {
      assert_eq!(
        sheets[0]
          .cell_at(CellAddress::parse_a1(address).unwrap())
          .unwrap()
          .display_text,
        expected
      );
    }
  }

  #[test]
  fn fixed_array_single_cell_reference_repeats_its_value() {
    use super::super::styles::StylesCatalog;
    use super::super::worksheet::{SheetIdentity, SheetResourceCatalog};
    use ooxmlsdk::sdk::SdkType;

    // POI 57798.xlsx: Office repeats the value of A1 throughout B1:B2.
    // Adjacent source cells must not leak into a fixed array's output region.
    for (formula, expected) in [
      ("A1", ["one", "one", "one", "one"]),
      ("$A$1", ["one", "one", "one", "one"]),
      ("A1:A1", ["one", "one", "one", "one"]),
      ("A1:B2", ["one", "right", "below", "bottom"]),
    ] {
      let worksheet = x::Worksheet::from_bytes(
        format!(
          r#"
        <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
          <sheetData>
            <row r="1">
              <c r="A1" t="str"><v>one</v></c><c r="B1" t="str"><v>right</v></c>
              <c r="C1" t="str"><f t="array" ref="C1:D2">{formula}</f><v>stale</v></c>
              <c r="D1" t="str"><v>stale</v></c>
            </row>
            <row r="2">
              <c r="A2" t="str"><v>below</v></c><c r="B2" t="str"><v>bottom</v></c>
              <c r="C2" t="str"><v>stale</v></c><c r="D2" t="str"><v>stale</v></c>
            </row>
          </sheetData>
        </worksheet>"#
        )
        .as_bytes(),
      )
      .unwrap();
      let sheet = CalcSheet::from_worksheet(
        SheetIdentity {
          workbook_index: 0,
          name: "Sheet1".into(),
          state: None,
          active: true,
        },
        worksheet,
        SheetResourceCatalog::default(),
        &[],
        &StylesCatalog::default(),
        Default::default(),
      );
      let mut sheets = vec![sheet];
      recalculate_formula_cells(
        &mut sheets,
        &DefinedNamesCatalog::default(),
        None,
        &WorkbookCatalog::default(),
        &FormulaContext::new(false, None, None),
      );
      let actual = ["C1", "D1", "C2", "D2"].map(|address| {
        sheets[0]
          .cell_at(CellAddress::parse_a1(address).unwrap())
          .unwrap()
          .display_text
          .as_str()
      });
      assert_eq!(actual, expected, "{formula}");
    }
  }

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
  fn foreign_eomonth_addin_is_not_an_excel_builtin() {
    use super::super::styles::StylesCatalog;
    use super::super::worksheet::{SheetIdentity, SheetResourceCatalog};
    use ooxmlsdk::sdk::SdkType;

    // Office tdf141495.xlsx recalculates the LibreOffice service name as
    // #NAME?, while the actual Excel EOMONTH builtin remains available.
    for (formula, expected) in [
      (
        "com.sun.star.sheet.addin.Analysis.getEomonth(A1,1)",
        "#NAME?",
      ),
      ("EOMONTH(A1,1)", "44255"),
      (
        "IFERROR(com.sun.star.sheet.addin.Analysis.getEomonth(A1,1),7)",
        "7",
      ),
      (
        "IF(FALSE,com.sun.star.sheet.addin.Analysis.getEomonth(A1,1),9)",
        "9",
      ),
    ] {
      let worksheet = x::Worksheet::from_bytes(
        format!(
          r#"
        <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
          <sheetData><row r="1">
            <c r="A1"><v>44227</v></c>
            <c r="B1"><f>{formula}</f><v>999</v></c>
          </row></sheetData>
        </worksheet>"#
        )
        .as_bytes(),
      )
      .unwrap();
      let sheet = CalcSheet::from_worksheet(
        SheetIdentity {
          workbook_index: 0,
          name: "Sheet1".into(),
          state: None,
          active: true,
        },
        worksheet,
        SheetResourceCatalog::default(),
        &[],
        &StylesCatalog::default(),
        Default::default(),
      );
      let mut sheets = vec![sheet];
      recalculate_formula_cells(
        &mut sheets,
        &DefinedNamesCatalog::default(),
        None,
        &WorkbookCatalog::default(),
        &FormulaContext::new(false, None, None),
      );
      assert_eq!(
        sheets[0]
          .cell_at(CellAddress::parse_a1("B1").unwrap())
          .unwrap()
          .display_text,
        expected,
        "{formula}"
      );
    }
  }

  #[test]
  fn unresolved_external_workbook_reference_is_an_office_ref_error() {
    assert_eq!(
      unresolved_external_reference_value("SUM('[missing.xlsx]Sheet0'!A1:B1)"),
      Some(Value::Error("#REF!".to_string()))
    );
    assert_eq!(
      unresolved_external_reference_value("SUM(Table1[Amount])"),
      None
    );
  }

  #[test]
  fn worksheet_row_states_are_lowered_to_zero_based_formula_rows() {
    let book = FormulaBook {
      sheet_names: vec!["Sheet1".to_string()],
      sheet_workbook_indices: vec![4],
      cells: BTreeMap::new(),
      formulas: BTreeMap::new(),
      hidden_rows: HashSet::from([(0, 7)]),
      filtered_rows: HashSet::from([(0, 5)]),
      external_cells: HashMap::new(),
      external_defined_names: HashMap::new(),
      tables: HashMap::new(),
      defined: DefinedNames::default(),
    };

    let states = formula_row_states(&book);
    assert!(states[&(ooxmlsdk_formula::SheetId(4), 6)].hidden);
    assert!(states[&(ooxmlsdk_formula::SheetId(4), 4)].filtered);
  }
}
