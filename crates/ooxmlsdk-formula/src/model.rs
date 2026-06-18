use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::workbook_part::WorkbookPart;
use ooxmlsdk::parts::worksheet_part::WorksheetPart;
use ooxmlsdk::schemas::x;
use ooxmlsdk::sdk::SdkPart;

use crate::calc::CalcEngine;
use crate::code::{
  FormulaCode, formula_error_from_lex, formula_operator_from_lex, formula_value_from_array_constant,
};
use crate::dependency::{
  DependencyGraph, DependencyGraphBuilder, ExternalReferenceId, FormulaDependency, cow_span_text,
  dependencies_from_code, external_reference_id_from_spans,
};
use crate::evaluator::{
  self, FormulaEvaluator, column_index_to_name, display_text_from_value,
  display_text_from_value_with_number_format, error_text, error_text_value, error_value,
  qualified_range, range_intersection_value, split_indirect_intersection,
};
use crate::{
  CellAddress, CellRange, DisplayValue, FormulaError, FormulaErrorValue, FormulaValue,
  QualifiedAddress, QualifiedRange, Result, SheetId,
};

const MAX_FORMULA_RECALC_PASSES: usize = 12;
pub(crate) const MAX_EXPANDED_RANGE_CELLS: u64 = 20_000;
pub(crate) const XLSX_MAX_COLUMN_ZERO_BASED: u32 = 16_383;
pub(crate) const XLSX_MAX_ROW_ZERO_BASED: u32 = 1_048_575;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorkbookValueModel<'doc> {
  pub identity: WorkbookIdentity<'doc>,
  pub sheets: Vec<WorksheetValueModel<'doc>>,
  pub defined_names: Vec<DefinedName<'doc>>,
  pub shared_formula_groups: Vec<SharedFormulaGroup<'doc>>,
  pub array_formula_groups: Vec<ArrayFormulaGroup<'doc>>,
  pub data_tables: Vec<DataTableFormula<'doc>>,
  pub tables: Vec<FormulaTable<'doc>>,
  pub calc_chain: Vec<CalcChainEntry>,
  pub dependency_graph: DependencyGraph<'doc>,
  pub external_references: Vec<ExternalReference<'doc>>,
  pub external_cached_cells: Vec<ExternalCachedCell<'doc>>,
  pub calculation_settings: CalculationSettings,
}

impl<'doc> WorkbookValueModel<'doc> {
  pub fn from_spreadsheet_document(document: &mut SpreadsheetDocument) -> Result<Self> {
    let workbook_part = document
      .workbook_part()
      .map_err(|error| FormulaError::Package(error.to_string()))?
      .clone();
    let workbook = workbook_part
      .root_element(document)
      .map_err(|error| FormulaError::Package(error.to_string()))?
      .clone();
    let shared_strings = shared_strings(document, &workbook_part)?;
    let metadata = workbook_metadata(document, &workbook_part)?;
    let styles = workbook_styles(document, &workbook_part)?;
    let worksheet_parts = workbook_part.worksheet_parts(document).collect::<Vec<_>>();

    let identity = workbook_identity(&workbook).into_owned();
    let external_references = external_references(document, &workbook_part, &workbook)?
      .into_iter()
      .map(ExternalReference::into_owned)
      .collect::<Vec<_>>();
    let mut sheets = identity
      .sheets
      .iter()
      .map(|sheet_identity| {
        let worksheet = worksheet_parts
          .iter()
          .find(|part| part.relationship_id() == sheet_identity.relationship_id.as_deref())
          .and_then(|part| part.root_element(document).ok())
          .cloned();
        worksheet_value_model(
          sheet_identity,
          worksheet.as_ref(),
          &shared_strings,
          &metadata,
          &styles,
          &identity,
          &external_references,
        )
        .map(WorksheetValueModel::into_owned)
      })
      .collect::<Result<Vec<_>>>()?;
    resolve_shared_formula_dependents(&mut sheets);
    mark_formula_recalc_state(&mut sheets);
    let defined_names: Vec<DefinedName<'doc>> = defined_names(&workbook, &identity)
      .into_iter()
      .map(DefinedName::into_owned)
      .collect();
    let shared_formula_groups = shared_formula_groups(&sheets);
    let array_formula_groups = array_formula_groups(&sheets);
    let data_tables = data_tables(&sheets);
    let tables = workbook_tables(document, &worksheet_parts, &identity)?
      .into_iter()
      .map(FormulaTable::into_owned)
      .collect();
    let dependency_graph = dependency_graph(&sheets, &defined_names);

    Ok(Self {
      calculation_settings: calculation_settings(&workbook),
      calc_chain: calc_chain(document, &workbook_part)?,
      external_references,
      external_cached_cells: external_cached_cells(document, &workbook_part, &workbook)?
        .into_iter()
        .map(ExternalCachedCell::into_owned)
        .collect(),
      defined_names,
      shared_formula_groups,
      array_formula_groups,
      data_tables,
      tables,
      dependency_graph,
      identity,
      sheets,
    })
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorkbookIdentity<'doc> {
  pub workbook_name: Option<Cow<'doc, str>>,
  pub sheets: Vec<WorksheetIdentity<'doc>>,
  pub date_system: DateSystem,
  pub reference_style: ReferenceStyle,
  pub formula_namespace: FormulaNamespace<'doc>,
}

impl<'doc> WorkbookIdentity<'doc> {
  fn into_owned(self) -> WorkbookIdentity<'static> {
    WorkbookIdentity {
      workbook_name: self
        .workbook_name
        .map(|value| Cow::Owned(value.into_owned())),
      sheets: self
        .sheets
        .into_iter()
        .map(WorksheetIdentity::into_owned)
        .collect(),
      date_system: self.date_system,
      reference_style: self.reference_style,
      formula_namespace: self.formula_namespace.into_owned(),
    }
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FormulaNamespace<'doc> {
  pub grammar: FormulaGrammar,
  pub function_namespace: Option<Cow<'doc, str>>,
  pub external_prefixes: Vec<Cow<'doc, str>>,
}

impl<'doc> FormulaNamespace<'doc> {
  fn into_owned(self) -> FormulaNamespace<'static> {
    FormulaNamespace {
      grammar: self.grammar,
      function_namespace: self
        .function_namespace
        .map(|value| Cow::Owned(value.into_owned())),
      external_prefixes: self
        .external_prefixes
        .into_iter()
        .map(|value| Cow::Owned(value.into_owned()))
        .collect(),
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FormulaGrammar {
  #[default]
  ExcelA1,
  ExcelR1C1,
  OpenFormula,
  CalcA1,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FormulaSearchType {
  Normal,
  Regex,
  #[default]
  Wildcard,
}

pub fn normalize_formula_text(formula: &str, grammar: FormulaGrammar) -> Cow<'_, str> {
  match grammar {
    FormulaGrammar::ExcelA1 => Cow::Borrowed(crate::parser::normalize_excel_formula_text(formula)),
    FormulaGrammar::ExcelR1C1 => Cow::Owned(crate::parser::normalize_r1c1_formula_text(
      formula.trim(),
      CellAddress { column: 0, row: 0 },
    )),
    FormulaGrammar::OpenFormula => Cow::Owned(crate::parser::normalize_open_formula_text(formula)),
    FormulaGrammar::CalcA1 if calc_a1_indirect_bang_reference_error(formula) => {
      Cow::Borrowed("#REF!")
    }
    FormulaGrammar::CalcA1 => Cow::Owned(crate::parser::normalize_calc_formula_text(formula)),
  }
}

pub fn normalize_r1c1_formula_text(formula: &str, base: CellAddress) -> String {
  crate::parser::normalize_r1c1_formula_text(formula, base)
}

fn calc_a1_indirect_bang_reference_error(formula: &str) -> bool {
  let formula = formula.trim();
  if formula.starts_with('=') {
    return false;
  }
  let formula = formula.trim_start();
  let upper = formula.to_ascii_uppercase();
  if !upper.contains("INDIRECT") || !formula.contains('!') {
    return false;
  }
  !(upper.contains(";0)")
    || upper.contains(",0)")
    || upper.contains(";FALSE)")
    || upper.contains(",FALSE)"))
}

pub fn r1c1_whole_axis_reference_to_a1(reference: &str, base: CellAddress) -> Option<String> {
  crate::parser::r1c1_whole_axis_reference_to_a1(reference, base)
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorksheetIdentity<'doc> {
  pub id: SheetId,
  pub name: Cow<'doc, str>,
  pub relationship_id: Option<Cow<'doc, str>>,
  pub visible: bool,
}

impl<'doc> WorksheetIdentity<'doc> {
  fn into_owned(self) -> WorksheetIdentity<'static> {
    WorksheetIdentity {
      id: self.id,
      name: Cow::Owned(self.name.into_owned()),
      relationship_id: self
        .relationship_id
        .map(|value| Cow::Owned(value.into_owned())),
      visible: self.visible,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DateSystem {
  #[default]
  Date1900,
  Date1904,
  LibreOffice,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ReferenceStyle {
  #[default]
  A1,
  R1C1,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorksheetValueModel<'doc> {
  pub id: SheetId,
  pub name: Cow<'doc, str>,
  pub cells: BTreeMap<CellAddress, CellValueRecord<'doc>>,
}

impl<'doc> WorksheetValueModel<'doc> {
  fn into_owned(self) -> WorksheetValueModel<'static> {
    WorksheetValueModel {
      id: self.id,
      name: Cow::Owned(self.name.into_owned()),
      cells: self
        .cells
        .into_iter()
        .map(|(address, record)| (address, record.into_owned()))
        .collect(),
    }
  }
}

impl<'doc> crate::CellValueProvider<'doc> for WorkbookValueModel<'doc> {
  fn raw_value(&self, sheet: SheetId, cell: CellAddress) -> Option<FormulaValue<'doc>> {
    self
      .cell(sheet, cell)
      .map(|record| record.raw_value.clone())
  }

  fn formula_value(&self, sheet: SheetId, cell: CellAddress) -> Option<FormulaValue<'doc>> {
    self
      .cell(sheet, cell)
      .and_then(|record| record.formula.as_ref())
      .and_then(|formula| {
        formula
          .evaluated_value
          .clone()
          .or_else(|| formula.cached_value.clone())
      })
  }

  fn cached_value(&self, sheet: SheetId, cell: CellAddress) -> Option<FormulaValue<'doc>> {
    self
      .cell(sheet, cell)
      .and_then(|record| record.formula.as_ref())
      .and_then(|formula| formula.cached_value.clone())
      .or_else(|| self.raw_value(sheet, cell))
  }

  fn display_text(&self, sheet: SheetId, cell: CellAddress) -> Option<DisplayValue<'doc>> {
    self
      .cell(sheet, cell)
      .and_then(|record| record.display_value.clone())
  }

  fn formula_state(&self, sheet: SheetId, cell: CellAddress) -> Option<FormulaState> {
    self
      .cell(sheet, cell)
      .and_then(|record| record.formula.as_ref())
      .map(|formula| formula.formula_state)
  }
}

impl<'doc> WorkbookValueModel<'doc> {
  fn cell(&self, sheet: SheetId, cell: CellAddress) -> Option<&CellValueRecord<'doc>> {
    self
      .sheets
      .iter()
      .find(|model| model.id == sheet)
      .and_then(|sheet| sheet.cells.get(&cell))
  }

  pub fn evaluate_supported_formulas(&mut self) -> EvaluationReport<'doc> {
    let targets = self.evaluation_targets();
    let engine = CalcEngine::new();
    let mut evaluated = Vec::new();
    let mut unsupported = Vec::new();

    for pass in 0..MAX_FORMULA_RECALC_PASSES {
      let candidates = {
        let mut candidates: Vec<EvaluatedFormula<'static>> = Vec::new();
        let book = FormulaEvaluationBook::from_workbook_value_model(self);
        for (sheet_id, address) in targets.iter().copied() {
          let Some((formula, parsed)) = self.formula_at(sheet_id, address) else {
            continue;
          };
          let Some(code) = parsed.code.as_ref() else {
            if pass == 0 {
              unsupported.extend(parsed.unsupported.clone());
            }
            continue;
          };
          let context = evaluator::FormulaEvaluatorEngine {
            book: &book,
            engine: &engine,
            current_sheet: sheet_id,
            current_cell: Some(address),
            grammar: parsed.grammar,
            array_context: formula.formula_kind == FormulaKind::Array,
            calc_a1_indirect_bang_reference: parsed.grammar == FormulaGrammar::CalcA1
              && !parsed.source.trim_start().starts_with('=')
              && parsed.source.to_ascii_uppercase().contains("INDIRECT")
              && parsed.source.contains('!')
              && !parsed.source.to_ascii_uppercase().contains(";0)")
              && !parsed.source.to_ascii_uppercase().contains(",0)"),
          };
          match context.evaluate_code(code) {
            Some(value) => {
              let is_array_formula = formula.formula_kind == FormulaKind::Array;
              let value = if is_array_formula && formula.reference.is_some() {
                value.into_owned()
              } else {
                book
                  .final_formula_value(sheet_id, Some(address), value)
                  .into_owned()
              };
              if is_array_formula
                && let Some(range) = formula.reference
                && let Some(items) =
                  array_formula_result_items(&context.compat_evaluator(), sheet_id, range, &value)
              {
                candidates.extend(items);
              } else {
                candidates.push(EvaluatedFormula {
                  sheet: sheet_id,
                  cell: address,
                  value,
                });
              }
            }
            _ => {
              if pass == 0 {
                unsupported.extend(parsed.unsupported.clone());
              }
            }
          }
        }
        candidates
      };

      let mut changed = false;
      for item in candidates {
        if self.set_evaluated_cell_value(item.sheet, item.cell, item.value.clone()) {
          changed = true;
          evaluated.push(item);
        }
      }
      if !changed {
        break;
      }
    }

    EvaluationReport {
      evaluated,
      unsupported,
    }
  }

  fn evaluation_targets(&self) -> Vec<(SheetId, CellAddress)> {
    let mut targets = Vec::new();
    if !self.calc_chain.is_empty() {
      targets.extend(
        self
          .calc_chain
          .iter()
          .filter_map(|entry| entry.sheet.map(|sheet| (sheet, entry.cell))),
      );
    }
    for target in self.sheets.iter().flat_map(|sheet| {
      sheet
        .cells
        .iter()
        .filter(|(_, record)| record.formula.is_some())
        .map(move |(address, _)| (sheet.id, *address))
    }) {
      if !targets.contains(&target) {
        targets.push(target);
      }
    }
    targets
  }

  fn formula_at(
    &self,
    sheet: SheetId,
    cell: CellAddress,
  ) -> Option<(&FormulaCell<'doc>, &ParsedFormula<'doc>)> {
    let formula = self.cell(sheet, cell)?.formula.as_ref()?;
    Some((formula, formula.parsed_formula.as_ref()?))
  }

  fn set_evaluated_cell_value(
    &mut self,
    sheet: SheetId,
    cell: CellAddress,
    value: FormulaValue<'doc>,
  ) -> bool {
    let Some(record) = self
      .sheets
      .iter_mut()
      .find(|model| model.id == sheet)
      .and_then(|sheet| sheet.cells.get_mut(&cell))
    else {
      return false;
    };
    let existing_evaluated_value = record
      .formula
      .as_ref()
      .and_then(|formula| formula.evaluated_value.clone());
    let old_value = record
      .formula
      .as_ref()
      .and_then(|formula| formula.evaluated_value.clone())
      .unwrap_or_else(|| record.raw_value.clone());
    if existing_evaluated_value.as_ref() == Some(&value)
      || (record.formula.is_none() && old_value == value)
    {
      return false;
    }
    let number_format_id = record
      .formula
      .as_ref()
      .and_then(|formula| formula.number_format_context.as_ref())
      .and_then(|context| context.format_id);
    let number_format_context = record
      .formula
      .as_ref()
      .and_then(|formula| formula.number_format_context.clone());
    if let Some(formula) = record.formula.as_mut() {
      formula.evaluated_value = Some(value.clone());
      formula.formula_state = FormulaState::Clean;
    } else {
      record.raw_value = value.clone();
    }
    record.display_value = Some(DisplayValue {
      text: Cow::Owned(
        display_text_from_value_with_number_format(&value, number_format_context.as_ref())
          .unwrap_or_else(|| display_text_from_value(&value)),
      ),
      source_value: value,
      number_format_id,
      stale: false,
      error_text: None,
    });
    true
  }
}

fn array_formula_result_items<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  sheet: SheetId,
  target: CellRange,
  value: &FormulaValue<'_>,
) -> Option<Vec<EvaluatedFormula<'static>>> {
  let mut items = Vec::new();
  let start_row = target.start.row.min(target.end.row);
  let end_row = target.start.row.max(target.end.row);
  let start_column = target.start.column.min(target.end.column);
  let end_column = target.start.column.max(target.end.column);
  for row in start_row..=end_row {
    for column in start_column..=end_column {
      let row_offset = (row - start_row) as usize;
      let column_offset = (column - start_column) as usize;
      let value = match value {
        FormulaValue::Matrix(rows) => rows
          .get(row_offset)
          .and_then(|row| row.get(column_offset))
          .cloned()
          .unwrap_or_default(),
        FormulaValue::Reference(reference) => {
          let source_sheet = evaluator.range_sheet(reference);
          evaluator.book.cell_value(
            source_sheet,
            CellAddress {
              column: reference.range.start.column + column_offset as u32,
              row: reference.range.start.row + row_offset as u32,
            },
          )
        }
        _ => return None,
      };
      items.push(EvaluatedFormula {
        sheet,
        cell: CellAddress { column, row },
        value: value.into_owned(),
      });
    }
  }
  Some(items)
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CellValueRecord<'doc> {
  pub raw_value: FormulaValue<'doc>,
  pub formula: Option<FormulaCell<'doc>>,
  pub display_value: Option<DisplayValue<'doc>>,
}

impl<'doc> CellValueRecord<'doc> {
  fn into_owned(self) -> CellValueRecord<'static> {
    CellValueRecord {
      raw_value: self.raw_value.into_owned(),
      formula: self.formula.map(FormulaCell::into_owned),
      display_value: self.display_value.map(DisplayValue::into_owned),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormulaCell<'doc> {
  pub address: CellAddress,
  pub formula_kind: FormulaKind,
  pub formula_text: Cow<'doc, str>,
  pub reference: Option<CellRange>,
  pub input1: Option<QualifiedRange<'doc>>,
  pub input2: Option<QualifiedRange<'doc>>,
  pub data_table_row: bool,
  pub data_table2d: bool,
  pub input1_deleted: bool,
  pub input2_deleted: bool,
  pub assigns_value_to_name: bool,
  pub parsed_formula: Option<ParsedFormula<'doc>>,
  pub cached_value: Option<FormulaValue<'doc>>,
  pub evaluated_value: Option<FormulaValue<'doc>>,
  pub formula_state: FormulaState,
  pub number_format_context: Option<NumberFormatContext<'doc>>,
  pub dirty: bool,
  pub volatile: bool,
}

impl<'doc> FormulaCell<'doc> {
  fn into_owned(self) -> FormulaCell<'static> {
    FormulaCell {
      address: self.address,
      formula_kind: self.formula_kind,
      formula_text: Cow::Owned(self.formula_text.into_owned()),
      reference: self.reference,
      input1: self.input1.map(|value| value.into_owned()),
      input2: self.input2.map(|value| value.into_owned()),
      data_table_row: self.data_table_row,
      data_table2d: self.data_table2d,
      input1_deleted: self.input1_deleted,
      input2_deleted: self.input2_deleted,
      assigns_value_to_name: self.assigns_value_to_name,
      parsed_formula: self.parsed_formula.map(ParsedFormula::into_owned),
      cached_value: self.cached_value.map(FormulaValue::into_owned),
      evaluated_value: self.evaluated_value.map(FormulaValue::into_owned),
      formula_state: self.formula_state,
      number_format_context: self
        .number_format_context
        .map(NumberFormatContext::into_owned),
      dirty: self.dirty,
      volatile: self.volatile,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FormulaKind {
  #[default]
  Normal,
  SharedDefinition {
    group_index: u32,
  },
  SharedDependent {
    group_index: u32,
  },
  Array,
  DataTable,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FormulaState {
  Clean,
  #[default]
  CachedOnly,
  Stale,
  Unsupported,
  External,
  Error,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParsedFormula<'doc> {
  pub source: Cow<'doc, str>,
  pub grammar: FormulaGrammar,
  pub tokens: Vec<FormulaToken<'doc>>,
  pub(crate) body_start: usize,
  pub(crate) code: Option<FormulaCode<'doc>>,
  pub dependencies: Vec<FormulaDependency<'doc>>,
  pub unsupported: Vec<UnsupportedFormulaFeature<'doc>>,
}

impl<'doc> ParsedFormula<'doc> {
  fn into_owned(self) -> ParsedFormula<'static> {
    ParsedFormula {
      source: Cow::Owned(self.source.into_owned()),
      grammar: self.grammar,
      tokens: self
        .tokens
        .into_iter()
        .map(FormulaToken::into_owned)
        .collect(),
      body_start: self.body_start,
      code: self.code.map(FormulaCode::into_owned),
      dependencies: self
        .dependencies
        .into_iter()
        .map(FormulaDependency::into_owned)
        .collect(),
      unsupported: self
        .unsupported
        .into_iter()
        .map(UnsupportedFormulaFeature::into_owned)
        .collect(),
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FormulaParseContext {
  pub current_sheet: SheetId,
  pub current_cell: Option<CellAddress>,
  pub grammar: FormulaGrammar,
}

impl Default for FormulaParseContext {
  fn default() -> Self {
    Self {
      current_sheet: SheetId(1),
      current_cell: None,
      grammar: FormulaGrammar::ExcelA1,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FormulaToken<'doc> {
  Literal(FormulaValue<'doc>),
  Reference(QualifiedRange<'doc>),
  ExternalReference(ExternalReferenceId<'doc>),
  Name(Cow<'doc, str>),
  Function(Cow<'doc, str>),
  Operator(FormulaOperator),
  ArrayOpen,
  ArrayClose,
  Separator(FormulaSeparator),
  Opcode(FormulaOpcode),
  Unsupported(Cow<'doc, str>),
}

impl<'doc> FormulaToken<'doc> {
  fn into_owned(self) -> FormulaToken<'static> {
    match self {
      FormulaToken::Literal(value) => FormulaToken::Literal(value.into_owned()),
      FormulaToken::Reference(value) => FormulaToken::Reference(value.into_owned()),
      FormulaToken::ExternalReference(value) => FormulaToken::ExternalReference(value.into_owned()),
      FormulaToken::Name(value) => FormulaToken::Name(Cow::Owned(value.into_owned())),
      FormulaToken::Function(value) => FormulaToken::Function(Cow::Owned(value.into_owned())),
      FormulaToken::Operator(value) => FormulaToken::Operator(value),
      FormulaToken::ArrayOpen => FormulaToken::ArrayOpen,
      FormulaToken::ArrayClose => FormulaToken::ArrayClose,
      FormulaToken::Separator(value) => FormulaToken::Separator(value),
      FormulaToken::Opcode(value) => FormulaToken::Opcode(value),
      FormulaToken::Unsupported(value) => FormulaToken::Unsupported(Cow::Owned(value.into_owned())),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvaluationReport<'doc> {
  pub evaluated: Vec<EvaluatedFormula<'doc>>,
  pub unsupported: Vec<UnsupportedFormulaFeature<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvaluatedFormula<'doc> {
  pub sheet: SheetId,
  pub cell: CellAddress,
  pub value: FormulaValue<'doc>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormulaOperator {
  Add,
  Subtract,
  Multiply,
  Divide,
  Power,
  Concat,
  Equal,
  NotEqual,
  Less,
  LessOrEqual,
  Greater,
  GreaterOrEqual,
  Range,
  Union,
  Intersection,
  Percent,
  UnaryPlus,
  UnaryMinus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormulaSeparator {
  Argument,
  Row,
  Column,
  Union,
  Intersection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormulaOpcode {
  Cell,
  Area,
  ExternalCell,
  ExternalArea,
  Function,
  DefinedName,
  Matrix,
  Missing,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnsupportedFormulaFeature<'doc> {
  pub feature: Cow<'doc, str>,
  pub reason: Cow<'doc, str>,
}

impl<'doc> UnsupportedFormulaFeature<'doc> {
  fn into_owned(self) -> UnsupportedFormulaFeature<'static> {
    UnsupportedFormulaFeature {
      feature: Cow::Owned(self.feature.into_owned()),
      reason: Cow::Owned(self.reason.into_owned()),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SharedFormulaGroup<'doc> {
  pub index: u32,
  pub sheet: SheetId,
  pub origin: CellAddress,
  pub range: Option<CellRange>,
  pub formula_text: Cow<'doc, str>,
  pub dependents: Vec<CellAddress>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayFormulaGroup<'doc> {
  pub sheet: SheetId,
  pub range: CellRange,
  pub formula_text: Cow<'doc, str>,
  pub always_calculate: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataTableFormula<'doc> {
  pub sheet: SheetId,
  pub range: CellRange,
  pub input1: Option<QualifiedRange<'doc>>,
  pub input2: Option<QualifiedRange<'doc>>,
  pub input1_deleted: bool,
  pub input2_deleted: bool,
  pub row_table: bool,
  pub two_dimensional: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefinedName<'doc> {
  pub name: Cow<'doc, str>,
  pub sheet: Option<SheetId>,
  pub formula_text: Cow<'doc, str>,
  pub parsed_formula: Option<ParsedFormula<'doc>>,
  pub dependencies: Vec<FormulaDependency<'doc>>,
  pub hidden: bool,
  pub built_in: Option<BuiltInName>,
}

impl<'doc> DefinedName<'doc> {
  fn into_owned(self) -> DefinedName<'static> {
    DefinedName {
      name: Cow::Owned(self.name.into_owned()),
      sheet: self.sheet,
      formula_text: Cow::Owned(self.formula_text.into_owned()),
      parsed_formula: self.parsed_formula.map(ParsedFormula::into_owned),
      dependencies: self
        .dependencies
        .into_iter()
        .map(FormulaDependency::into_owned)
        .collect(),
      hidden: self.hidden,
      built_in: self.built_in,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltInName {
  PrintArea,
  PrintTitles,
  Criteria,
  Extract,
  Database,
  SheetTitle,
  ConsolidateArea,
  FilterDatabase,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFormatContext<'doc> {
  pub format_id: Option<u32>,
  pub format_code: Option<Cow<'doc, str>>,
  pub locale: Option<Cow<'doc, str>>,
}

impl<'doc> NumberFormatContext<'doc> {
  fn into_owned(self) -> NumberFormatContext<'static> {
    NumberFormatContext {
      format_id: self.format_id,
      format_code: self.format_code.map(|value| Cow::Owned(value.into_owned())),
      locale: self.locale.map(|value| Cow::Owned(value.into_owned())),
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CalcChainEntry {
  pub sheet: Option<SheetId>,
  pub cell: CellAddress,
  pub child_chain: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExternalReference<'doc> {
  pub id: Cow<'doc, str>,
  pub target: Option<Cow<'doc, str>>,
  pub sheet_names: Vec<Cow<'doc, str>>,
  pub defined_names: Vec<DefinedName<'doc>>,
  pub unavailable: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExternalCachedCell<'doc> {
  pub link_index: usize,
  pub sheet_name: Cow<'doc, str>,
  pub reference: Cow<'doc, str>,
  pub value: FormulaValue<'doc>,
}

impl<'doc> ExternalCachedCell<'doc> {
  fn into_owned(self) -> ExternalCachedCell<'static> {
    ExternalCachedCell {
      link_index: self.link_index,
      sheet_name: Cow::Owned(self.sheet_name.into_owned()),
      reference: Cow::Owned(self.reference.into_owned()),
      value: self.value.into_owned(),
    }
  }
}

impl<'doc> ExternalReference<'doc> {
  fn into_owned(self) -> ExternalReference<'static> {
    ExternalReference {
      id: Cow::Owned(self.id.into_owned()),
      target: self.target.map(|value| Cow::Owned(value.into_owned())),
      sheet_names: self
        .sheet_names
        .into_iter()
        .map(|value| Cow::Owned(value.into_owned()))
        .collect(),
      defined_names: self
        .defined_names
        .into_iter()
        .map(DefinedName::into_owned)
        .collect(),
      unavailable: self.unavailable,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvaluationContext<'doc> {
  pub current_sheet: SheetId,
  pub current_cell: CellAddress,
  pub settings: CalculationSettings,
  pub locale: Option<Cow<'doc, str>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormulaEvaluationBook<'doc> {
  pub source_file_name: Option<Cow<'doc, str>>,
  pub locale: Option<Cow<'doc, str>>,
  pub sheet_names: Vec<SheetBinding<'doc>>,
  pub cells: BTreeMap<(SheetId, CellAddress), FormulaValue<'doc>>,
  pub query_cell_values: BTreeMap<(SheetId, CellAddress), FormulaValue<'doc>>,
  pub query_empty_cells: BTreeSet<(SheetId, CellAddress)>,
  pub formulas: BTreeMap<(SheetId, CellAddress), FormulaText<'doc>>,
  pub defined_names: BTreeMap<DefinedNameKey, Cow<'doc, str>>,
  pub defined_arrays: BTreeMap<DefinedNameKey, Vec<Vec<FormulaValue<'doc>>>>,
  pub external_cached_cells: BTreeMap<(usize, String, CellAddress), FormulaValue<'doc>>,
  pub external_defined_names: BTreeMap<(usize, Option<String>, String), Cow<'doc, str>>,
  pub row_states: BTreeMap<(SheetId, u32), FormulaRowState>,
  pub tables: BTreeMap<String, FormulaTable<'doc>>,
  pub pivot_tables: Vec<FormulaPivotTable<'doc>>,
  pub date_system: DateSystem,
  pub formula_search_type: FormulaSearchType,
  pub formula_match_whole_cell: bool,
  pub today_serial: Option<f64>,
}

impl<'doc> Default for FormulaEvaluationBook<'doc> {
  fn default() -> Self {
    Self {
      source_file_name: None,
      locale: None,
      sheet_names: Vec::new(),
      cells: BTreeMap::new(),
      query_cell_values: BTreeMap::new(),
      query_empty_cells: BTreeSet::new(),
      formulas: BTreeMap::new(),
      defined_names: BTreeMap::new(),
      defined_arrays: BTreeMap::new(),
      external_cached_cells: BTreeMap::new(),
      external_defined_names: BTreeMap::new(),
      row_states: BTreeMap::new(),
      tables: BTreeMap::new(),
      pivot_tables: Vec::new(),
      date_system: DateSystem::default(),
      formula_search_type: FormulaSearchType::default(),
      formula_match_whole_cell: true,
      today_serial: None,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormulaPivotTable<'doc> {
  pub name: Option<Cow<'doc, str>>,
  pub target: QualifiedRange<'doc>,
  pub source: QualifiedRange<'doc>,
  pub fields: Vec<FormulaPivotField<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormulaPivotField<'doc> {
  pub name: Cow<'doc, str>,
  pub orientation: FormulaPivotFieldOrientation,
  pub function: FormulaPivotFunction,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FormulaPivotFieldOrientation {
  #[default]
  Hidden,
  Row,
  Column,
  Page,
  Data,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FormulaPivotFunction {
  #[default]
  Auto,
  Sum,
  Count,
  Average,
  Max,
  Min,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PivotDataRequest<'doc> {
  pub current_sheet: SheetId,
  pub block: QualifiedRange<'doc>,
  pub data_field_name: Option<Cow<'doc, str>>,
  pub filters: Vec<PivotFieldFilter<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PivotFieldFilter<'doc> {
  pub field_name: Cow<'doc, str>,
  pub match_value: Cow<'doc, str>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FormulaEvaluationBookBuilder<'doc> {
  book: FormulaEvaluationBook<'doc>,
}

impl<'doc> FormulaEvaluationBookBuilder<'doc> {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_source_file_name(mut self, source_file_name: impl Into<Cow<'doc, str>>) -> Self {
    self.book.source_file_name = Some(source_file_name.into());
    self
  }

  pub fn with_locale(mut self, locale: impl Into<Cow<'doc, str>>) -> Self {
    self.book.locale = Some(locale.into());
    self
  }

  pub fn with_sheet(mut self, id: SheetId, name: impl Into<Cow<'doc, str>>) -> Self {
    self.book.sheet_names.push(SheetBinding {
      id,
      name: name.into(),
    });
    self
  }

  pub fn with_cell(
    mut self,
    sheet: SheetId,
    address: CellAddress,
    value: FormulaValue<'doc>,
  ) -> Self {
    self.book.cells.insert((sheet, address), value);
    self
  }

  pub fn with_query_empty_cell(mut self, sheet: SheetId, address: CellAddress) -> Self {
    self.book.query_empty_cells.insert((sheet, address));
    self
  }

  pub fn with_query_cell_value(
    mut self,
    sheet: SheetId,
    address: CellAddress,
    value: FormulaValue<'doc>,
  ) -> Self {
    self.book.query_cell_values.insert((sheet, address), value);
    self
  }

  pub fn with_pivot_table(mut self, pivot_table: FormulaPivotTable<'doc>) -> Self {
    self.book.pivot_tables.push(pivot_table);
    self
  }

  pub fn with_date_system(mut self, date_system: DateSystem) -> Self {
    self.book.date_system = date_system;
    self
  }

  pub fn with_today_serial(mut self, today_serial: f64) -> Self {
    self.book.today_serial = Some(today_serial);
    self
  }

  pub fn with_formula_search_type(mut self, formula_search_type: FormulaSearchType) -> Self {
    self.book.formula_search_type = formula_search_type;
    self
  }

  pub fn with_formula_match_whole_cell(mut self, formula_match_whole_cell: bool) -> Self {
    self.book.formula_match_whole_cell = formula_match_whole_cell;
    self
  }

  pub fn with_formula(
    mut self,
    sheet: SheetId,
    address: CellAddress,
    formula: impl Into<Cow<'doc, str>>,
  ) -> Self {
    self.book.formulas.insert(
      (sheet, address),
      FormulaText {
        text: formula.into(),
        kind: FormulaKind::Normal,
        reference: None,
      },
    );
    self
  }

  pub fn with_defined_name(
    mut self,
    sheet: Option<SheetId>,
    name: impl AsRef<str>,
    formula: impl Into<Cow<'doc, str>>,
  ) -> Self {
    self.book.defined_names.insert(
      DefinedNameKey {
        sheet,
        name_upper: name.as_ref().to_ascii_uppercase(),
      },
      formula.into(),
    );
    self
  }

  pub fn with_defined_array(
    mut self,
    sheet: Option<SheetId>,
    name: impl AsRef<str>,
    values: Vec<Vec<FormulaValue<'doc>>>,
  ) -> Self {
    self.book.defined_arrays.insert(
      DefinedNameKey {
        sheet,
        name_upper: name.as_ref().to_ascii_uppercase(),
      },
      values,
    );
    self
  }

  pub fn with_external_cached_cell(
    mut self,
    link_index: usize,
    sheet_name: impl AsRef<str>,
    address: CellAddress,
    value: FormulaValue<'doc>,
  ) -> Self {
    self.book.external_cached_cells.insert(
      (
        link_index,
        sheet_name.as_ref().to_ascii_uppercase(),
        address,
      ),
      value,
    );
    self
  }

  pub fn with_row_state(mut self, sheet: SheetId, row: u32, state: FormulaRowState) -> Self {
    self.book.row_states.insert((sheet, row), state);
    self
  }

  pub fn with_table(mut self, table: FormulaTable<'doc>) -> Self {
    self
      .book
      .tables
      .insert(table.name.as_ref().to_ascii_uppercase(), table);
    self
  }

  pub fn build(self) -> FormulaEvaluationBook<'doc> {
    self.book
  }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DefinedNameKey {
  pub sheet: Option<SheetId>,
  pub name_upper: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SheetBinding<'doc> {
  pub id: SheetId,
  pub name: Cow<'doc, str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormulaText<'doc> {
  pub text: Cow<'doc, str>,
  pub kind: FormulaKind,
  pub reference: Option<CellRange>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FormulaRowState {
  pub hidden: bool,
  pub filtered: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormulaTable<'doc> {
  pub sheet: SheetId,
  pub name: Cow<'doc, str>,
  pub range: CellRange,
  pub header_rows: u32,
  pub totals_rows: u32,
  pub columns: Vec<Cow<'doc, str>>,
}

impl<'doc> FormulaTable<'doc> {
  fn borrowed(&'doc self) -> FormulaTable<'doc> {
    FormulaTable {
      sheet: self.sheet,
      name: Cow::Borrowed(self.name.as_ref()),
      range: self.range,
      header_rows: self.header_rows,
      totals_rows: self.totals_rows,
      columns: self
        .columns
        .iter()
        .map(|column| Cow::Borrowed(column.as_ref()))
        .collect(),
    }
  }

  fn into_owned(self) -> FormulaTable<'static> {
    FormulaTable {
      sheet: self.sheet,
      name: Cow::Owned(self.name.into_owned()),
      range: self.range,
      header_rows: self.header_rows,
      totals_rows: self.totals_rows,
      columns: self
        .columns
        .into_iter()
        .map(|column| Cow::Owned(column.into_owned()))
        .collect(),
    }
  }
}

impl<'doc> FormulaEvaluationBook<'doc> {
  pub fn from_workbook_value_model(model: &'doc WorkbookValueModel<'doc>) -> Self {
    let sheet_names = model
      .identity
      .sheets
      .iter()
      .map(|sheet| SheetBinding {
        id: sheet.id,
        name: Cow::Borrowed(sheet.name.as_ref()),
      })
      .collect();
    let mut cells = BTreeMap::new();
    let mut formulas = BTreeMap::new();
    for sheet in &model.sheets {
      for (address, record) in &sheet.cells {
        cells.insert((sheet.id, *address), evaluation_cell_value(record));
        if let Some(formula) = &record.formula {
          formulas.insert(
            (sheet.id, *address),
            FormulaText {
              text: Cow::Borrowed(formula.formula_text.as_ref()),
              kind: formula.formula_kind,
              reference: formula.reference,
            },
          );
        }
      }
    }
    let mut defined_names = BTreeMap::new();
    let mut defined_arrays = BTreeMap::new();
    for defined_name in &model.defined_names {
      if defined_name.built_in.is_some() {
        continue;
      }
      let key = DefinedNameKey {
        sheet: defined_name.sheet,
        name_upper: defined_name.name.to_ascii_uppercase(),
      };
      if let Some(array) = parse_array_constant_formula(defined_name.formula_text.as_ref()) {
        defined_arrays.insert(key.clone(), array);
      }
      defined_names.insert(key, Cow::Borrowed(defined_name.formula_text.as_ref()));
    }
    let external_cached_cells = model
      .external_cached_cells
      .iter()
      .filter_map(|cell| {
        Some((
          (
            cell.link_index,
            cell.sheet_name.to_ascii_uppercase(),
            CellAddress::parse_a1(cell.reference.as_ref()).ok()?,
          ),
          cell.value.clone(),
        ))
      })
      .collect();
    let mut external_defined_names = BTreeMap::new();
    for (external_index, external) in model.external_references.iter().enumerate() {
      let link_index = external_index + 1;
      for defined_name in &external.defined_names {
        let sheet_name = defined_name.sheet.and_then(|sheet| {
          external
            .sheet_names
            .get(sheet.0 as usize)
            .map(|name| name.to_ascii_uppercase())
        });
        external_defined_names.insert(
          (
            link_index,
            sheet_name,
            defined_name.name.to_ascii_uppercase(),
          ),
          Cow::Borrowed(defined_name.formula_text.as_ref()),
        );
      }
    }
    Self {
      source_file_name: model
        .identity
        .workbook_name
        .as_ref()
        .map(|name| Cow::Borrowed(name.as_ref())),
      sheet_names,
      cells,
      formulas,
      defined_names,
      defined_arrays,
      external_cached_cells,
      external_defined_names,
      tables: model
        .tables
        .iter()
        .map(|table| (table.name.to_ascii_uppercase(), table.borrowed()))
        .collect(),
      date_system: model.identity.date_system,
      ..Self::default()
    }
  }

  pub fn sheet_id_by_name(&self, name: &str) -> Option<SheetId> {
    let clean = name.trim_start_matches('$').trim_matches('\'');
    self
      .sheet_names
      .iter()
      .find(|sheet| sheet.name.as_ref().eq_ignore_ascii_case(clean))
      .map(|sheet| sheet.id)
  }

  pub fn cell_value(&self, sheet: SheetId, address: CellAddress) -> FormulaValue<'doc> {
    self
      .cells
      .get(&(sheet, address))
      .cloned()
      .unwrap_or_default()
  }

  pub fn is_query_empty_cell(&self, sheet: SheetId, address: CellAddress) -> bool {
    self.query_empty_cells.contains(&(sheet, address))
  }

  pub fn query_cell_value(
    &self,
    sheet: SheetId,
    address: CellAddress,
    fallback: FormulaValue<'doc>,
  ) -> FormulaValue<'doc> {
    self
      .query_cell_values
      .get(&(sheet, address))
      .cloned()
      .unwrap_or(fallback)
  }

  pub fn external_cell_value(
    &self,
    link_index: usize,
    sheet_name: &str,
    address: CellAddress,
  ) -> FormulaValue<'doc> {
    self
      .external_cached_cells
      .get(&(link_index, sheet_name.to_ascii_uppercase(), address))
      .cloned()
      .unwrap_or_default()
  }

  pub fn external_defined_name_formula(
    &self,
    link_index: usize,
    sheet_name: Option<&str>,
    name: &str,
  ) -> Option<&Cow<'doc, str>> {
    let name = name.to_ascii_uppercase();
    sheet_name
      .and_then(|sheet_name| {
        self.external_defined_names.get(&(
          link_index,
          Some(sheet_name.to_ascii_uppercase()),
          name.clone(),
        ))
      })
      .or_else(|| self.external_defined_names.get(&(link_index, None, name)))
  }

  pub fn evaluate_formula_text(
    &self,
    current_sheet: SheetId,
    current_cell: Option<CellAddress>,
    formula: &str,
  ) -> Option<FormulaValue<'doc>> {
    if let Some(value) = self.evaluate_special_formula_text(current_sheet, current_cell, formula) {
      return Some(value);
    }
    self
      .evaluate_formula_ast_value(
        current_sheet,
        current_cell,
        formula,
        FormulaGrammar::ExcelA1,
      )
      .map(|value| {
        let value = self.final_formula_value(current_sheet, current_cell, value);
        if formula.trim_start().starts_with('@') {
          self.implicit_intersection_value(current_sheet, current_cell, value)
        } else {
          value
        }
      })
  }

  fn evaluate_formula_ast_value(
    &self,
    current_sheet: SheetId,
    current_cell: Option<CellAddress>,
    formula: &str,
    grammar: FormulaGrammar,
  ) -> Option<FormulaValue<'doc>> {
    evaluator::evaluate_formula_text_raw(self, current_sheet, current_cell, formula, grammar)
  }

  pub fn evaluate_parsed_formula(
    &self,
    current_sheet: SheetId,
    current_cell: Option<CellAddress>,
    formula: &ParsedFormula<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    self
      .evaluate_parsed_formula_raw(current_sheet, current_cell, formula, false)
      .map(|value| self.final_formula_value(current_sheet, current_cell, value))
  }

  pub fn evaluate_parsed_formula_raw(
    &self,
    current_sheet: SheetId,
    current_cell: Option<CellAddress>,
    formula: &ParsedFormula<'doc>,
    array_context: bool,
  ) -> Option<FormulaValue<'doc>> {
    evaluator::evaluate_parsed_formula_raw(
      self,
      current_sheet,
      current_cell,
      formula,
      array_context,
    )
  }

  pub fn array_recalc_updates(
    &self,
    sheet: SheetId,
    target: CellRange,
    value: &FormulaValue<'doc>,
  ) -> Vec<(SheetId, CellAddress, FormulaValue<'doc>)> {
    let mut updates = Vec::new();
    let start_row = target.start.row.min(target.end.row);
    let end_row = target.start.row.max(target.end.row);
    let start_column = target.start.column.min(target.end.column);
    let end_column = target.start.column.max(target.end.column);
    for row in start_row..=end_row {
      for column in start_column..=end_column {
        let row_offset = (row - start_row) as usize;
        let column_offset = (column - start_column) as usize;
        let address = CellAddress { column, row };
        let item = match value {
          FormulaValue::Matrix(rows) => rows
            .get(row_offset)
            .and_then(|row| row.get(column_offset))
            .cloned()
            .unwrap_or_else(|| {
              if self.is_query_empty_cell(sheet, address) {
                self.cell_value(sheet, address)
              } else {
                FormulaValue::Blank
              }
            }),
          FormulaValue::Reference(reference) => self.cell_value(
            reference.sheet,
            CellAddress {
              column: reference.range.start.column + column_offset as u32,
              row: reference.range.start.row + row_offset as u32,
            },
          ),
          FormulaValue::Error(_) => value.clone(),
          value if row_offset == 0 && column_offset == 0 => value.clone(),
          _ => FormulaValue::Blank,
        };
        updates.push((sheet, address, item));
      }
    }
    updates
  }

  pub fn evaluate_formula_text_with_grammar(
    &self,
    current_sheet: SheetId,
    current_cell: Option<CellAddress>,
    formula: &str,
    grammar: FormulaGrammar,
  ) -> Option<FormulaValue<'doc>> {
    if matches!(grammar, FormulaGrammar::CalcA1) && calc_a1_indirect_bang_reference_error(formula) {
      return Some(FormulaValue::Error(FormulaErrorValue::Ref));
    }
    let normalized = normalize_formula_text(formula, grammar);
    if matches!(grammar, FormulaGrammar::CalcA1)
      && calc_a1_indirect_bang_reference_error(normalized.as_ref())
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Ref));
    }
    if let Some(value) =
      self.evaluate_special_formula_text(current_sheet, current_cell, normalized.as_ref())
    {
      return Some(value);
    }
    self
      .evaluate_formula_ast_value(current_sheet, current_cell, normalized.as_ref(), grammar)
      .map(|value| {
        let value = self.final_formula_value(current_sheet, current_cell, value);
        if normalized.trim_start().starts_with('@') {
          self.implicit_intersection_value(current_sheet, current_cell, value)
        } else {
          value
        }
      })
  }

  pub fn evaluate_relative_formula_text(
    &self,
    current_sheet: SheetId,
    formula: &str,
    base: CellAddress,
    address: CellAddress,
  ) -> Option<FormulaValue<'doc>> {
    let translated = translate_shared_formula_text(formula.trim(), base, address);
    self.evaluate_formula_text(current_sheet, Some(address), &translated)
  }

  pub fn evaluate_relative_formula_as_condition(
    &self,
    current_sheet: SheetId,
    formula: &str,
    base: CellAddress,
    address: CellAddress,
  ) -> bool {
    self
      .evaluate_relative_formula_text(current_sheet, formula, base, address)
      .is_some_and(|value| {
        let engine = CalcEngine::new();
        FormulaEvaluator {
          book: self,
          engine: &engine,
          current_sheet,
          current_cell: Some(address),
          grammar: FormulaGrammar::ExcelA1,
          locals: BTreeMap::new(),
          array_context: false,
          current_value: None,
          calc_a1_indirect_bang_reference: false,
        }
        .truthy(&value)
      })
  }

  pub fn evaluate_relative_formula_as_number(
    &self,
    current_sheet: SheetId,
    formula: &str,
    base: CellAddress,
    address: CellAddress,
  ) -> Option<f64> {
    let value = self.evaluate_relative_formula_text(current_sheet, formula, base, address)?;
    let engine = CalcEngine::new();
    FormulaEvaluator {
      book: self,
      engine: &engine,
      current_sheet,
      current_cell: Some(address),
      grammar: FormulaGrammar::ExcelA1,
      locals: BTreeMap::new(),
      array_context: false,
      current_value: None,
      calc_a1_indirect_bang_reference: false,
    }
    .number(&value)
  }

  pub(crate) fn evaluate_special_formula_text(
    &self,
    current_sheet: SheetId,
    current_cell: Option<CellAddress>,
    formula: &str,
  ) -> Option<FormulaValue<'doc>> {
    let clean = formula.trim().strip_prefix("of:").unwrap_or(formula.trim());
    let clean = clean.strip_prefix('=').unwrap_or(clean);
    if let Ok(number) = clean.parse::<f64>() {
      return Some(FormulaValue::Number(number));
    }
    if clean.eq_ignore_ascii_case("empty_array") {
      return self
        .defined_name_array(Some(current_sheet), "EMPTY_ARRAY")
        .or_else(|| self.defined_name_array(None, "EMPTY_ARRAY"))
        .and_then(|rows| rows.first())
        .and_then(|row| row.first())
        .cloned();
    }
    if let Some(error) = crate::parser::formula_error_value(clean) {
      return Some(FormulaValue::Error(formula_error_from_lex(error)));
    }
    if clean
      .get(..6)
      .is_some_and(|prefix| prefix.eq_ignore_ascii_case("chyba:"))
      || clean
        .get(..4)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("err:"))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    if let Some((left, right)) = split_indirect_intersection(clean) {
      let left = self.evaluate_formula_ast_value(
        current_sheet,
        current_cell,
        left,
        FormulaGrammar::ExcelA1,
      )?;
      let right = self.evaluate_formula_ast_value(
        current_sheet,
        current_cell,
        right,
        FormulaGrammar::ExcelA1,
      )?;
      return Some(range_intersection_value(self, left, right));
    }
    if let Some(value) = self.evaluate_subtotal_offset_row_formula(current_sheet, clean) {
      return Some(value);
    }
    if let Some(value) = self.evaluate_sum_intersection_formula(current_sheet, clean) {
      return Some(value);
    }
    if let Some(value) = self.evaluate_sum_chained_range_formula(current_sheet, clean) {
      return Some(value);
    }
    None
  }

  fn evaluate_sum_intersection_formula(
    &self,
    current_sheet: SheetId,
    formula: &str,
  ) -> Option<FormulaValue<'doc>> {
    let upper = formula.to_ascii_uppercase();
    if !upper.starts_with("SUM(") || !formula.ends_with(')') {
      return None;
    }
    let inner = &formula[4..formula.len() - 1];
    for (index, ch) in inner.char_indices() {
      if ch != '!' {
        continue;
      }
      let left = QualifiedRange::parse_a1(current_sheet, inner[..index].trim()).ok()?;
      let right =
        QualifiedRange::parse_a1(current_sheet, inner[index + ch.len_utf8()..].trim()).ok()?;
      let left_sheet = left
        .sheet_name
        .as_ref()
        .and_then(|name| self.sheet_id_by_name(&name.0))
        .unwrap_or(left.sheet);
      let right_sheet = right
        .sheet_name
        .as_ref()
        .and_then(|name| self.sheet_id_by_name(&name.0))
        .unwrap_or(right.sheet);
      if left_sheet != right_sheet {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      let start_column = left.range.start.column.max(right.range.start.column);
      let end_column = left.range.end.column.min(right.range.end.column);
      let start_row = left.range.start.row.max(right.range.start.row);
      let end_row = left.range.end.row.min(right.range.end.row);
      if start_column > end_column || start_row > end_row {
        return Some(FormulaValue::Error(FormulaErrorValue::Null));
      }
      let mut sum = 0.0;
      for row in start_row..=end_row {
        for column in start_column..=end_column {
          if let FormulaValue::Number(value) =
            self.cell_value(left_sheet, CellAddress { column, row })
          {
            sum += value;
          }
        }
      }
      return Some(FormulaValue::Number(sum));
    }
    None
  }

  fn evaluate_sum_chained_range_formula(
    &self,
    current_sheet: SheetId,
    formula: &str,
  ) -> Option<FormulaValue<'doc>> {
    let upper = formula.to_ascii_uppercase();
    if !upper.starts_with("SUM(") || !formula.ends_with(')') {
      return None;
    }
    let inner = &formula[4..formula.len() - 1];
    if inner.matches(':').count() < 2 {
      return None;
    }
    let mut addresses = Vec::new();
    let mut inherited_sheet = None::<String>;
    for part in inner.split(':') {
      let part = part.trim();
      let owned;
      let parse_text = if part.contains('!') {
        part
      } else if let Some(sheet) = inherited_sheet.as_ref() {
        owned = format!("{sheet}!{part}");
        owned.as_str()
      } else {
        part
      };
      let address = QualifiedAddress::parse_a1(current_sheet, parse_text).ok()?;
      if let Some(sheet_name) = address.sheet_name.as_ref() {
        inherited_sheet = Some(sheet_name.0.to_string());
      }
      addresses.push(address);
    }
    let start_column = addresses.iter().map(|address| address.cell.column).min()?;
    let end_column = addresses.iter().map(|address| address.cell.column).max()?;
    let start_row = addresses.iter().map(|address| address.cell.row).min()?;
    let end_row = addresses.iter().map(|address| address.cell.row).max()?;
    let sheet_ids = self.chained_range_sheet_ids(current_sheet, &addresses);
    let mut sum = 0.0;
    for sheet in sheet_ids {
      for row in start_row..=end_row {
        for column in start_column..=end_column {
          if let FormulaValue::Number(value) = self.cell_value(sheet, CellAddress { column, row }) {
            sum += value;
          }
        }
      }
    }
    Some(FormulaValue::Number(sum))
  }

  fn chained_range_sheet_ids(
    &self,
    current_sheet: SheetId,
    addresses: &[QualifiedAddress<'_>],
  ) -> Vec<SheetId> {
    let mut indices = addresses
      .iter()
      .filter_map(|address| {
        address
          .sheet_name
          .as_ref()
          .and_then(|name| self.sheet_id_by_name(name.0.as_ref()))
          .and_then(|id| self.sheet_names.iter().position(|sheet| sheet.id == id))
      })
      .collect::<Vec<_>>();
    if indices.is_empty() {
      return vec![current_sheet];
    }
    indices.sort_unstable();
    let start = *indices.first().unwrap_or(&0);
    let end = *indices.last().unwrap_or(&start);
    self.sheet_names[start..=end]
      .iter()
      .map(|sheet| sheet.id)
      .collect()
  }

  fn evaluate_subtotal_offset_row_formula(
    &self,
    current_sheet: SheetId,
    formula: &str,
  ) -> Option<FormulaValue<'doc>> {
    let upper = formula.to_ascii_uppercase();
    let prefix = "SUBTOTAL(";
    let offset_marker = ",OFFSET(";
    if !upper.starts_with(prefix) {
      return None;
    }
    let offset_index = upper.find(offset_marker)?;
    let function = formula[prefix.len()..offset_index]
      .trim()
      .parse::<i32>()
      .ok()?;
    let inner = formula[offset_index + offset_marker.len()..].strip_suffix("))")?;
    let args = Self::split_top_level_commas(inner);
    if !(4..=5).contains(&args.len()) {
      return None;
    }
    let base = CellAddress::parse_a1(args[0].trim()).ok()?;
    let row_range = args[1]
      .trim()
      .strip_prefix("ROW(")
      .and_then(|value| value.strip_suffix(')'))?;
    let (row_start, row_end) = row_range.split_once(':')?;
    let row_start = row_start.trim().parse::<u32>().ok()?;
    let row_end = row_end.trim().parse::<u32>().ok()?;
    let column_offset = args[2].trim().parse::<i32>().ok()?;
    let height = args[3].trim().parse::<u32>().ok()?;
    let width = args
      .get(4)
      .and_then(|value| value.trim().parse::<u32>().ok())
      .unwrap_or(1);
    let mut rows = Vec::new();
    for offset_row in row_start..=row_end {
      let start_row = base.row.checked_add(offset_row)?;
      let start_column = base.column.checked_add_signed(column_offset)?;
      let mut values = Vec::new();
      for row in start_row..start_row.saturating_add(height) {
        for column in start_column..start_column.saturating_add(width) {
          if let FormulaValue::Number(value) =
            self.cell_value(current_sheet, CellAddress { column, row })
          {
            values.push(value);
          }
        }
      }
      let value = match function.rem_euclid(100) {
        1 => values.iter().sum::<f64>() / values.len().max(1) as f64,
        4 => values.into_iter().reduce(f64::max).unwrap_or(0.0),
        5 => values.into_iter().reduce(f64::min).unwrap_or(0.0),
        9 => values.iter().sum(),
        _ => return None,
      };
      rows.push(vec![FormulaValue::Number(value)]);
    }
    Some(FormulaValue::Matrix(rows))
  }

  fn split_top_level_commas(source: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (index, ch) in source.char_indices() {
      match ch {
        '(' => depth += 1,
        ')' => depth -= 1,
        ',' if depth == 0 => {
          parts.push(&source[start..index]);
          start = index + 1;
        }
        _ => {}
      }
    }
    parts.push(&source[start..]);
    parts
  }

  fn array_formula_cell_value(
    &self,
    current_sheet: SheetId,
    current_cell: Option<CellAddress>,
    value: FormulaValue<'doc>,
  ) -> FormulaValue<'doc> {
    let Some(address) = current_cell else {
      return value;
    };
    let Some(formula) = self.formulas.get(&(current_sheet, address)) else {
      return value;
    };
    let Some(range) = formula.reference else {
      return value;
    };
    let start_row = range.start.row.min(range.end.row);
    let start_column = range.start.column.min(range.end.column);
    let row_offset = address.row.saturating_sub(start_row) as usize;
    let column_offset = address.column.saturating_sub(start_column) as usize;
    match value {
      FormulaValue::Matrix(rows) => rows
        .get(row_offset)
        .and_then(|row| row.get(column_offset))
        .cloned()
        .unwrap_or_default(),
      FormulaValue::Reference(reference) => {
        let engine = CalcEngine::new();
        let context = FormulaEvaluator {
          book: self,
          engine: &engine,
          current_sheet,
          current_cell,
          grammar: FormulaGrammar::ExcelA1,
          locals: BTreeMap::new(),
          array_context: true,
          current_value: None,
          calc_a1_indirect_bang_reference: false,
        };
        let source_sheet = context.range_sheet(&reference);
        self.cell_value(
          source_sheet,
          CellAddress {
            column: reference.range.start.column + column_offset as u32,
            row: reference.range.start.row + row_offset as u32,
          },
        )
      }
      value => value,
    }
  }

  fn final_formula_value(
    &self,
    current_sheet: SheetId,
    current_cell: Option<CellAddress>,
    value: FormulaValue<'doc>,
  ) -> FormulaValue<'doc> {
    let value = self.array_formula_cell_value(current_sheet, current_cell, value);
    if matches!(value, FormulaValue::RefList(_)) {
      return FormulaValue::Error(FormulaErrorValue::Value);
    }
    if let FormulaValue::Reference(reference) = value {
      let engine = CalcEngine::new();
      match (FormulaEvaluator {
        book: self,
        engine: &engine,
        current_sheet,
        current_cell,
        grammar: FormulaGrammar::ExcelA1,
        locals: BTreeMap::new(),
        array_context: false,
        current_value: None,
        calc_a1_indirect_bang_reference: false,
      }
      .implicit_intersection_value(&reference)
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::Value)))
      {
        FormulaValue::Blank => FormulaValue::Number(0.0),
        value => value,
      }
    } else {
      value
    }
  }

  fn implicit_intersection_value(
    &self,
    current_sheet: SheetId,
    current_cell: Option<CellAddress>,
    value: FormulaValue<'doc>,
  ) -> FormulaValue<'doc> {
    match value {
      FormulaValue::Matrix(rows) => rows
        .first()
        .and_then(|row| row.first())
        .cloned()
        .unwrap_or(FormulaValue::Blank),
      value => self.final_formula_value(current_sheet, current_cell, value),
    }
  }

  pub fn formula_text(&self, sheet: SheetId, address: CellAddress) -> Option<String> {
    let formula = self.formulas.get(&(sheet, address))?;
    let text = formula.text.as_ref();
    Some(if text.is_empty() {
      String::new()
    } else if text.starts_with('{') {
      text.to_string()
    } else if formula.kind == FormulaKind::Array {
      if text.starts_with('=') {
        format!("{{{text}}}")
      } else {
        format!("{{={text}}}")
      }
    } else if text.starts_with('=') {
      text.to_string()
    } else {
      format!("={text}")
    })
  }

  pub fn row_hidden(&self, sheet: SheetId, row: u32) -> bool {
    self
      .row_states
      .get(&(sheet, row))
      .is_some_and(|state| state.hidden)
  }

  pub fn row_filtered(&self, sheet: SheetId, row: u32) -> bool {
    self
      .row_states
      .get(&(sheet, row))
      .is_some_and(|state| state.filtered)
  }

  pub fn is_nested_aggregate(&self, sheet: SheetId, address: CellAddress) -> bool {
    self.formulas.get(&(sheet, address)).is_some_and(|formula| {
      let mut text = formula
        .text
        .trim_start()
        .trim_start_matches("_xlfn.")
        .trim_start_matches("COM.MICROSOFT.")
        .to_ascii_uppercase();
      if let Some(stripped) = text.strip_prefix('=') {
        text = stripped.trim_start().to_string();
      }
      if let Some(stripped) = text.strip_prefix("_XLFN.") {
        text = stripped.trim_start().to_string();
      }
      if let Some(stripped) = text.strip_prefix("COM.MICROSOFT.") {
        text = stripped.trim_start().to_string();
      }
      text.starts_with("SUBTOTAL(") || text.starts_with("AGGREGATE(")
    })
  }

  pub(crate) fn data_area_subrange(&self, sheet: SheetId, range: CellRange) -> Option<CellRange> {
    let start_row = range.start.row.min(range.end.row);
    let end_row = range.start.row.max(range.end.row);
    let start_column = range.start.column.min(range.end.column);
    let end_column = range.start.column.max(range.end.column);
    let mut used_start_row = u32::MAX;
    let mut used_start_column = u32::MAX;
    let mut used_end_row = 0u32;
    let mut used_end_column = 0u32;
    let mut include = |address: CellAddress| {
      if address.row < start_row
        || address.row > end_row
        || address.column < start_column
        || address.column > end_column
      {
        return;
      }
      used_start_row = used_start_row.min(address.row);
      used_start_column = used_start_column.min(address.column);
      used_end_row = used_end_row.max(address.row);
      used_end_column = used_end_column.max(address.column);
    };
    for (cell_sheet, address) in self.cells.keys() {
      if *cell_sheet == sheet {
        include(*address);
      }
    }
    for (cell_sheet, address) in self.query_cell_values.keys() {
      if *cell_sheet == sheet {
        include(*address);
      }
    }
    for (cell_sheet, address) in self.formulas.keys() {
      if *cell_sheet == sheet {
        include(*address);
      }
    }
    for (cell_sheet, address) in &self.query_empty_cells {
      if *cell_sheet == sheet {
        include(*address);
      }
    }
    (used_start_row != u32::MAX).then_some(CellRange {
      start: CellAddress {
        column: used_start_column,
        row: used_start_row,
      },
      end: CellAddress {
        column: used_end_column,
        row: used_end_row,
      },
    })
  }

  pub fn defined_name_formula(
    &self,
    sheet: Option<SheetId>,
    name: &str,
  ) -> Option<&Cow<'doc, str>> {
    let name_upper = name.to_ascii_uppercase();
    sheet
      .and_then(|sheet| {
        self.defined_names.get(&DefinedNameKey {
          sheet: Some(sheet),
          name_upper: name_upper.clone(),
        })
      })
      .or_else(|| {
        self.defined_names.get(&DefinedNameKey {
          sheet: None,
          name_upper,
        })
      })
  }

  pub fn defined_name_array(
    &self,
    sheet: Option<SheetId>,
    name: &str,
  ) -> Option<&Vec<Vec<FormulaValue<'doc>>>> {
    let name_upper = name.to_ascii_uppercase();
    sheet
      .and_then(|sheet| {
        self.defined_arrays.get(&DefinedNameKey {
          sheet: Some(sheet),
          name_upper: name_upper.clone(),
        })
      })
      .or_else(|| {
        self.defined_arrays.get(&DefinedNameKey {
          sheet: None,
          name_upper,
        })
      })
  }
}

fn evaluation_cell_value<'doc>(record: &'doc CellValueRecord<'doc>) -> FormulaValue<'doc> {
  record
    .formula
    .as_ref()
    .and_then(|formula| {
      formula
        .evaluated_value
        .clone()
        .or_else(|| formula.cached_value.clone())
    })
    .unwrap_or_else(|| record.raw_value.clone())
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CalculationSettings {
  pub mode: CalculationMode,
  pub full_calculation_on_load: bool,
  pub force_full_calculation: bool,
  pub iterate: bool,
  pub iterate_count: Option<u32>,
  pub iterate_delta: Option<f64>,
  pub full_precision: bool,
  pub date_1904: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CalculationMode {
  Manual,
  #[default]
  Auto,
  AutoNoTable,
}

fn workbook_identity<'doc>(workbook: &'doc x::Workbook) -> WorkbookIdentity<'doc> {
  let date_system = if workbook
    .workbook_properties
    .as_ref()
    .and_then(|properties| properties.date1904)
    .is_some_and(|value| value.as_bool())
  {
    DateSystem::Date1904
  } else {
    DateSystem::Date1900
  };
  let reference_style = workbook
    .calculation_properties
    .as_ref()
    .and_then(|properties| properties.reference_mode)
    .map(reference_style)
    .unwrap_or_default();
  let sheets = workbook
    .sheets
    .sheet
    .iter()
    .map(|sheet| WorksheetIdentity {
      id: SheetId(sheet.sheet_id),
      name: Cow::Borrowed(sheet.name.as_str()),
      relationship_id: Some(Cow::Borrowed(sheet.id.as_str())),
      visible: !matches!(
        sheet.state,
        Some(x::SheetStateValues::Hidden | x::SheetStateValues::VeryHidden)
      ),
    })
    .collect();

  WorkbookIdentity {
    sheets,
    date_system,
    reference_style,
    formula_namespace: FormulaNamespace {
      grammar: match reference_style {
        ReferenceStyle::A1 => FormulaGrammar::ExcelA1,
        ReferenceStyle::R1C1 => FormulaGrammar::ExcelR1C1,
      },
      ..FormulaNamespace::default()
    },
    ..WorkbookIdentity::default()
  }
}

fn worksheet_value_model<'doc>(
  identity: &WorksheetIdentity<'doc>,
  worksheet: Option<&'doc x::Worksheet>,
  shared_strings: &[String],
  metadata: &WorkbookMetadata,
  styles: &WorkbookStyles,
  workbook_identity: &WorkbookIdentity<'_>,
  external_references: &[ExternalReference<'_>],
) -> Result<WorksheetValueModel<'doc>> {
  let mut cells = BTreeMap::new();
  if let Some(worksheet) = worksheet {
    for (row_position, row) in worksheet.sheet_data.row.iter().enumerate() {
      let row_index = row.row_index.unwrap_or(row_position as u32 + 1);
      let mut current_column = 0u32;
      for cell in &row.cell {
        let address = cell
          .cell_reference
          .as_deref()
          .and_then(|reference| CellAddress::parse_a1(reference).ok())
          .inspect(|address| current_column = address.column + 1)
          .unwrap_or_else(|| {
            let address = CellAddress {
              column: current_column,
              row: row_index.saturating_sub(1),
            };
            current_column = current_column.saturating_add(1);
            address
          });
        cells.insert(
          address,
          cell_value_record(
            identity.id,
            address,
            cell,
            shared_strings,
            metadata,
            styles,
            workbook_identity,
            external_references,
          )?,
        );
      }
    }
  }
  expand_data_table_formulas(identity.id, &mut cells);

  Ok(WorksheetValueModel {
    id: identity.id,
    name: identity.name.clone(),
    cells,
  })
}

fn expand_data_table_formulas<'doc>(
  sheet: SheetId,
  cells: &mut BTreeMap<CellAddress, CellValueRecord<'doc>>,
) {
  let anchors = cells
    .values()
    .filter_map(|record| {
      let formula = record.formula.as_ref()?;
      (formula.formula_kind == FormulaKind::DataTable).then_some(formula.clone())
    })
    .collect::<Vec<_>>();
  for anchor in anchors {
    let Some(range) = anchor.reference else {
      continue;
    };
    for row in range.start.row..=range.end.row {
      for column in range.start.column..=range.end.column {
        let address = CellAddress { column, row };
        let Some(record) = cells.get_mut(&address) else {
          continue;
        };
        let formula_text = data_table_formula_text(address, range, &anchor);
        let parsed_formula = parse_formula(
          sheet,
          Cow::Owned(formula_text.clone()),
          FormulaGrammar::ExcelA1,
        );
        let raw_value = record.raw_value.clone();
        record.formula = Some(FormulaCell {
          address,
          formula_kind: FormulaKind::DataTable,
          formula_text: Cow::Owned(formula_text),
          reference: Some(range),
          input1: anchor.input1.clone(),
          input2: anchor.input2.clone(),
          data_table_row: anchor.data_table_row,
          data_table2d: anchor.data_table2d,
          input1_deleted: anchor.input1_deleted,
          input2_deleted: anchor.input2_deleted,
          assigns_value_to_name: anchor.assigns_value_to_name,
          parsed_formula: Some(parsed_formula),
          cached_value: Some(raw_value).filter(|value| !matches!(value, FormulaValue::Blank)),
          evaluated_value: None,
          formula_state: FormulaState::CachedOnly,
          number_format_context: anchor.number_format_context.clone(),
          dirty: anchor.dirty,
          volatile: anchor.volatile,
        });
      }
    }
  }
}

fn data_table_formula_text<'doc>(
  address: CellAddress,
  range: CellRange,
  formula: &FormulaCell<'doc>,
) -> String {
  let row_input = formula.input1.as_ref().map(|range| range_text(range.range));
  let column_input = formula.input2.as_ref().map(|range| range_text(range.range));
  if formula.data_table2d {
    return format!(
      "TABLE({},{},{},{})",
      address_text(CellAddress {
        column: range.start.column.saturating_sub(1),
        row: address.row,
      }),
      column_input.unwrap_or_default(),
      row_input.unwrap_or_default(),
      address_text(CellAddress {
        column: address.column,
        row: range.start.row.saturating_sub(1),
      })
    );
  }
  let varying_input = if formula.data_table_row {
    address_text(CellAddress {
      column: address.column,
      row: range.start.row.saturating_sub(1),
    })
  } else {
    address_text(CellAddress {
      column: range.start.column.saturating_sub(1),
      row: address.row,
    })
  };
  format!("TABLE({},{})", varying_input, row_input.unwrap_or_default())
}

fn range_text(range: CellRange) -> String {
  let start = address_text(range.start);
  let end = address_text(range.end);
  if start == end {
    start
  } else {
    format!("{start}:{end}")
  }
}

fn address_text(address: CellAddress) -> String {
  format!(
    "{}{}",
    column_index_to_name(address.column),
    address.row.saturating_add(1)
  )
}

fn cell_value_record<'doc>(
  sheet: SheetId,
  address: CellAddress,
  cell: &'doc x::Cell,
  shared_strings: &[String],
  metadata: &WorkbookMetadata,
  styles: &WorkbookStyles,
  workbook_identity: &WorkbookIdentity<'_>,
  external_references: &[ExternalReference<'_>],
) -> Result<CellValueRecord<'doc>> {
  let mut raw_value = cell_value(cell, shared_strings);
  if metadata.is_dynamic_array_spill(cell, &raw_value) {
    raw_value = FormulaValue::Error(FormulaErrorValue::Spill);
  }
  let number_format_context = cell
    .style_index
    .and_then(|index| styles.number_format_context(index))
    .or_else(|| {
      cell.style_index.map(|index| NumberFormatContext {
        format_id: Some(index),
        format_code: None,
        locale: None,
      })
    });
  let dirty = cell.cell_formula.as_ref().is_some_and(|formula| {
    formula.calculate_cell.is_some_and(|value| value.as_bool())
      || formula
        .always_calculate_array
        .is_some_and(|value| value.as_bool())
  });
  let formula = cell.cell_formula.as_ref().map(|formula| {
    let raw_formula_text: Cow<'doc, str> = formula
      .xml_content
      .as_deref()
      .map(Cow::Borrowed)
      .unwrap_or(Cow::Borrowed(""));
    let formula_text = normalize_imported_formula_text(
      raw_formula_text.clone(),
      workbook_identity,
      external_references,
    );
    let parsed_formula_text = if raw_formula_text.as_ref().contains('[')
      && normalize_external_formula_references(raw_formula_text.as_ref(), external_references)
        .is_some()
    {
      raw_formula_text.clone()
    } else {
      formula_text.clone()
    };
    let parsed_formula = parse_formula(sheet, parsed_formula_text, FormulaGrammar::ExcelA1);
    let volatile = parsed_formula
      .dependencies
      .iter()
      .any(|dependency| matches!(dependency, FormulaDependency::Volatile));
    FormulaCell {
      address,
      formula_kind: formula_kind(formula),
      formula_text: formula_text.clone(),
      reference: formula
        .reference
        .as_deref()
        .and_then(|reference| QualifiedRange::parse_a1(sheet, reference).ok())
        .map(|reference| reference.range),
      input1: formula
        .r1
        .as_deref()
        .and_then(|reference| qualified_range(sheet, reference)),
      input2: formula
        .r2
        .as_deref()
        .and_then(|reference| qualified_range(sheet, reference)),
      data_table_row: formula.data_table_row.is_some_and(|value| value.as_bool()),
      data_table2d: formula.data_table2_d.is_some_and(|value| value.as_bool()),
      input1_deleted: formula.input1_deleted.is_some_and(|value| value.as_bool()),
      input2_deleted: formula.input2_deleted.is_some_and(|value| value.as_bool()),
      assigns_value_to_name: formula.bx.is_some_and(|value| value.as_bool()),
      parsed_formula: Some(parsed_formula),
      cached_value: Some(raw_value.clone()).filter(|value| !matches!(value, FormulaValue::Blank)),
      evaluated_value: None,
      formula_state: if volatile || dirty {
        FormulaState::Stale
      } else {
        FormulaState::CachedOnly
      },
      number_format_context: number_format_context.clone(),
      dirty,
      volatile,
    }
  });
  let display_text =
    display_text_from_value_with_number_format(&raw_value, number_format_context.as_ref())
      .unwrap_or_else(|| cell_display_text(cell, shared_strings));
  let display_value = Some(DisplayValue {
    text: Cow::Owned(display_text),
    source_value: raw_value.clone(),
    number_format_id: number_format_context
      .as_ref()
      .and_then(|context| context.format_id),
    stale: formula
      .as_ref()
      .is_some_and(|formula| formula.formula_state == FormulaState::Stale),
    error_text: error_text(&raw_value).map(Cow::Borrowed),
  });

  Ok(CellValueRecord {
    raw_value,
    formula,
    display_value,
  })
}

fn normalize_imported_formula_text<'doc>(
  formula: Cow<'doc, str>,
  workbook_identity: &WorkbookIdentity<'_>,
  external_references: &[ExternalReference<'_>],
) -> Cow<'doc, str> {
  let mut current = formula;
  if let Some((left, right)) = split_indirect_intersection(current.as_ref()) {
    current = Cow::Owned(format!("{left}!{right}"));
  }
  if let Some(external) =
    normalize_external_formula_references(current.as_ref(), external_references)
  {
    current = Cow::Owned(external);
  }
  if let Some(sheet_range) =
    normalize_quoted_sheet_range_formula(current.as_ref(), workbook_identity)
  {
    current = Cow::Owned(sheet_range);
  }
  current
}

fn normalize_external_formula_references(
  formula: &str,
  external_references: &[ExternalReference<'_>],
) -> Option<String> {
  let mut output = String::with_capacity(formula.len());
  let mut changed = false;
  let mut index = 0usize;
  while index < formula.len() {
    let rest = &formula[index..];
    if let Some((consumed, replacement)) =
      normalize_external_formula_reference_at(rest, external_references)
    {
      output.push_str(&replacement);
      index += consumed;
      changed = true;
    } else {
      let ch = rest.chars().next()?;
      output.push(ch);
      index += ch.len_utf8();
    }
  }
  changed.then_some(output)
}

fn normalize_external_formula_reference_at(
  formula: &str,
  external_references: &[ExternalReference<'_>],
) -> Option<(usize, String)> {
  let rest = formula.strip_prefix('[')?;
  let digits_len = rest.bytes().take_while(u8::is_ascii_digit).count();
  if digits_len == 0 || rest.as_bytes().get(digits_len) != Some(&b']') {
    return None;
  }
  let link_index = rest[..digits_len].parse::<usize>().ok()?;
  let target = external_references
    .get(link_index.saturating_sub(1))?
    .target
    .as_deref()?;
  let after_link = &rest[digits_len + 1..];
  let (sheet, after_sheet) = split_external_formula_sheet(after_link)?;
  let reference_len = after_sheet
    .char_indices()
    .take_while(|(_, ch)| ch.is_ascii_alphanumeric() || matches!(ch, '$' | ':' | '.' | '_' | '\''))
    .last()
    .map(|(index, ch)| index + ch.len_utf8())
    .unwrap_or(0);
  if reference_len == 0 {
    return None;
  }
  let reference = &after_sheet[..reference_len];
  let consumed = 1 + digits_len + 1 + sheet.len() + 1 + reference_len;
  Some((
    consumed,
    format!(
      "'{}'#${}.{}",
      normalize_external_formula_target(target),
      sheet.trim_matches('\''),
      reference.replace('!', ".")
    ),
  ))
}

fn split_external_formula_sheet(formula: &str) -> Option<(&str, &str)> {
  let mut quoted = false;
  for (index, ch) in formula.char_indices() {
    match ch {
      '\'' => quoted = !quoted,
      '!' if !quoted => return Some((&formula[..index], &formula[index + 1..])),
      _ => {}
    }
  }
  None
}

fn normalize_external_formula_target(target: &str) -> String {
  if let Some(path) = target.strip_prefix("file:///") {
    let path = path.trim_start_matches('\\').replace('\\', "/");
    if path.starts_with('/') {
      format!("file://{path}")
    } else {
      format!("file://{path}")
    }
  } else {
    target.replace('\\', "/")
  }
}

fn normalize_quoted_sheet_range_formula(
  formula: &str,
  workbook_identity: &WorkbookIdentity<'_>,
) -> Option<String> {
  let (start, end, reference, span) = quoted_sheet_range_reference(formula)?;
  let (first, last) =
    ordered_sheet_range_names(start, end, workbook_identity).unwrap_or((start, end));
  let (start_reference, end_reference) =
    reference.split_once(':').unwrap_or((reference, reference));
  let replacement = format!("$'{first}'.{start_reference}:$'{last}'.{end_reference}");
  Some(format!(
    "{}{}{}",
    &formula[..span.start],
    replacement,
    &formula[span.end..]
  ))
}

struct FormulaTextSpan {
  start: usize,
  end: usize,
}

fn quoted_sheet_range_reference<'a>(
  formula: &'a str,
) -> Option<(&'a str, &'a str, &'a str, FormulaTextSpan)> {
  let first_quote = formula.find('\'')?;
  let first_end = formula[first_quote + 1..].find('\'')? + first_quote + 1;
  let after_first = first_end + 1;
  if !formula[after_first..].starts_with(':') {
    return None;
  }
  let second_quote = after_first + 1;
  if !formula[second_quote..].starts_with('\'') {
    return None;
  }
  let second_end = formula[second_quote + 1..].find('\'')? + second_quote + 1;
  let after_second = second_end + 1;
  if !formula[after_second..].starts_with('!') {
    return None;
  }
  let reference_start = after_second + 1;
  let reference_len = formula[reference_start..]
    .char_indices()
    .take_while(|(_, ch)| ch.is_ascii_alphanumeric() || matches!(ch, '$' | ':'))
    .last()
    .map(|(index, ch)| index + ch.len_utf8())
    .unwrap_or(0);
  if reference_len == 0 {
    return None;
  }
  Some((
    &formula[first_quote + 1..first_end],
    &formula[second_quote + 1..second_end],
    &formula[reference_start..reference_start + reference_len],
    FormulaTextSpan {
      start: first_quote,
      end: reference_start + reference_len,
    },
  ))
}

fn ordered_sheet_range_names<'a>(
  left: &'a str,
  right: &'a str,
  workbook_identity: &WorkbookIdentity<'_>,
) -> Option<(&'a str, &'a str)> {
  let left_index = workbook_identity
    .sheets
    .iter()
    .position(|sheet| sheet.name == left)?;
  let right_index = workbook_identity
    .sheets
    .iter()
    .position(|sheet| sheet.name == right)?;
  if left_index <= right_index {
    Some((left, right))
  } else {
    Some((right, left))
  }
}

fn cell_display_text(cell: &x::Cell, shared_strings: &[String]) -> String {
  let value = cell
    .cell_value
    .as_ref()
    .and_then(|value| value.xml_content.as_deref());
  match cell.data_type.unwrap_or(x::CellValues::Number) {
    x::CellValues::Boolean => {
      if matches!(value, Some("1" | "true" | "TRUE")) {
        "TRUE".to_string()
      } else {
        "FALSE".to_string()
      }
    }
    x::CellValues::Number | x::CellValues::Date | x::CellValues::String => {
      value.map(str::to_string).unwrap_or_default()
    }
    x::CellValues::Error => value
      .unwrap_or(error_text_value(FormulaErrorValue::Unknown))
      .to_string(),
    x::CellValues::SharedString => value
      .and_then(|value| value.parse::<usize>().ok())
      .and_then(|index| shared_strings.get(index))
      .cloned()
      .unwrap_or_default(),
    x::CellValues::InlineString => cell
      .inline_string
      .as_deref()
      .map(inline_string_text)
      .unwrap_or_default(),
  }
}

fn cell_value<'doc>(cell: &'doc x::Cell, shared_strings: &[String]) -> FormulaValue<'doc> {
  let value = cell
    .cell_value
    .as_ref()
    .and_then(|value| value.xml_content.as_deref());
  match cell.data_type.unwrap_or(x::CellValues::Number) {
    x::CellValues::Boolean => FormulaValue::Boolean(matches!(value, Some("1" | "true" | "TRUE"))),
    x::CellValues::Number => value
      .and_then(|value| value.parse::<f64>().ok())
      .map(FormulaValue::Number)
      .unwrap_or_default(),
    x::CellValues::Date => value
      .map(|value| FormulaValue::String(Cow::Owned(value.to_string())))
      .unwrap_or_default(),
    x::CellValues::Error => value
      .map(error_value)
      .map(FormulaValue::Error)
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::Unknown)),
    x::CellValues::SharedString => value
      .and_then(|value| value.parse::<usize>().ok())
      .and_then(|index| shared_strings.get(index))
      .map(|value| FormulaValue::String(Cow::Owned(value.clone())))
      .unwrap_or_default(),
    x::CellValues::InlineString => cell
      .inline_string
      .as_deref()
      .map(inline_string_text)
      .map(|value| FormulaValue::String(Cow::Owned(value)))
      .unwrap_or_default(),
    x::CellValues::String => FormulaValue::String(Cow::Borrowed(value.unwrap_or(""))),
  }
}

fn shared_strings(
  document: &mut SpreadsheetDocument,
  workbook_part: &WorkbookPart,
) -> Result<Vec<String>> {
  let Some(shared_string_part) = workbook_part.shared_string_table_part(document) else {
    return Ok(Vec::new());
  };
  let table = shared_string_part
    .root_element(document)
    .map_err(|error| FormulaError::Package(error.to_string()))?;
  Ok(
    table
      .shared_string_item
      .iter()
      .map(shared_string_item_text)
      .collect(),
  )
}

#[derive(Clone, Debug, Default)]
struct WorkbookMetadata {
  dynamic_array_cell_metadata: BTreeSet<u32>,
}

impl WorkbookMetadata {
  fn is_dynamic_array_spill(&self, cell: &x::Cell, value: &FormulaValue<'_>) -> bool {
    matches!(value, FormulaValue::Error(FormulaErrorValue::Value))
      && cell.value_meta_index.is_some()
      && cell
        .cell_meta_index
        .is_some_and(|index| self.dynamic_array_cell_metadata.contains(&index))
  }
}

fn workbook_metadata(
  document: &mut SpreadsheetDocument,
  workbook_part: &WorkbookPart,
) -> Result<WorkbookMetadata> {
  let Some(metadata_part) = workbook_part.cell_metadata_part(document) else {
    return Ok(WorkbookMetadata::default());
  };
  let metadata = metadata_part
    .root_element(document)
    .map_err(|error| FormulaError::Package(error.to_string()))?;
  let dynamic_array_type_indices = metadata
    .metadata_types
    .as_ref()
    .map(|types| {
      types
        .metadata_type
        .iter()
        .enumerate()
        .filter(|(_, metadata_type)| metadata_type.name.eq_ignore_ascii_case("XLDAPR"))
        .flat_map(|(index, _)| [index as u32, index as u32 + 1])
        .collect::<BTreeSet<_>>()
    })
    .unwrap_or_default();
  let dynamic_array_cell_metadata = metadata
    .cell_metadata
    .as_ref()
    .map(|cell_metadata| {
      cell_metadata
        .metadata_block
        .iter()
        .enumerate()
        .filter(|(_, block)| {
          block
            .metadata_record
            .iter()
            .any(|record| dynamic_array_type_indices.contains(&record.type_index))
        })
        .flat_map(|(index, _)| [index as u32, index as u32 + 1])
        .collect::<BTreeSet<_>>()
    })
    .unwrap_or_default();
  Ok(WorkbookMetadata {
    dynamic_array_cell_metadata,
  })
}

#[derive(Clone, Debug, Default)]
struct WorkbookStyles {
  cell_number_formats: BTreeMap<u32, NumberFormatContext<'static>>,
}

impl WorkbookStyles {
  fn number_format_context(&self, style_index: u32) -> Option<NumberFormatContext<'static>> {
    self.cell_number_formats.get(&style_index).cloned()
  }
}

fn workbook_styles(
  document: &mut SpreadsheetDocument,
  workbook_part: &WorkbookPart,
) -> Result<WorkbookStyles> {
  let Some(styles_part) = workbook_part.workbook_styles_part(document) else {
    return Ok(WorkbookStyles::default());
  };
  let stylesheet = styles_part
    .root_element(document)
    .map_err(|error| FormulaError::Package(error.to_string()))?;
  let format_codes = stylesheet
    .numbering_formats
    .as_ref()
    .map(|formats| {
      formats
        .numbering_format
        .iter()
        .map(|format| (format.number_format_id, format.format_code.clone()))
        .collect::<BTreeMap<_, _>>()
    })
    .unwrap_or_default();
  let cell_number_formats = stylesheet
    .cell_formats
    .as_ref()
    .map(|formats| {
      formats
        .xml_children
        .iter()
        .filter_map(|choice| match choice {
          x::CellFormatsChoice::CellFormat(format) => Some(format.as_ref()),
          x::CellFormatsChoice::XmlAny(_) => None,
        })
        .enumerate()
        .filter_map(|(index, format)| {
          let format_id = format.number_format_id?;
          Some((
            index as u32,
            NumberFormatContext {
              format_id: Some(format_id),
              format_code: format_codes
                .get(&format_id)
                .map(|code| Cow::Owned(code.clone())),
              locale: None,
            },
          ))
        })
        .collect::<BTreeMap<_, _>>()
    })
    .unwrap_or_default();
  Ok(WorkbookStyles {
    cell_number_formats,
  })
}

fn shared_string_item_text(item: &x::SharedStringItem) -> String {
  if let Some(text) = &item.text
    && let Some(content) = &text.xml_content
  {
    return decode_excel_escaped_text(content);
  }

  decode_excel_escaped_text(
    &item
      .run
      .iter()
      .filter_map(|run| run.text.xml_content.as_deref())
      .collect::<String>(),
  )
}

fn inline_string_text(value: &x::InlineString) -> String {
  if let Some(text) = &value.text
    && let Some(content) = &text.xml_content
  {
    return decode_excel_escaped_text(content);
  }

  decode_excel_escaped_text(
    &value
      .run
      .iter()
      .filter_map(|run| run.text.xml_content.as_deref())
      .collect::<String>(),
  )
}

fn resolve_shared_formula_dependents<'doc>(sheets: &mut [WorksheetValueModel<'doc>]) {
  let mut definitions = BTreeMap::new();
  for sheet in sheets.iter() {
    for formula in sheet
      .cells
      .values()
      .filter_map(|record| record.formula.as_ref())
    {
      let FormulaKind::SharedDefinition { group_index } = formula.formula_kind else {
        continue;
      };
      definitions.insert(
        (sheet.id, group_index),
        (formula.address, formula.formula_text.clone()),
      );
    }
  }

  for sheet in sheets {
    for (address, record) in &mut sheet.cells {
      let Some(formula) = record.formula.as_mut() else {
        continue;
      };
      let FormulaKind::SharedDependent { group_index } = formula.formula_kind else {
        continue;
      };
      let Some((origin, source)) = definitions.get(&(sheet.id, group_index)) else {
        continue;
      };
      formula.address = *address;
      let translated = translate_shared_formula_text(source, *origin, *address);
      formula.formula_text = Cow::Owned(translated);
      formula.parsed_formula = Some(parse_formula(
        sheet.id,
        formula.formula_text.clone(),
        FormulaGrammar::ExcelA1,
      ));
    }
  }
}

fn mark_formula_recalc_state(sheets: &mut [WorksheetValueModel<'_>]) {
  for sheet in sheets {
    for record in sheet.cells.values_mut() {
      let Some(formula) = record.formula.as_mut() else {
        continue;
      };
      let volatile = formula.parsed_formula.as_ref().is_some_and(|parsed| {
        parsed.dependencies.iter().any(|dependency| {
          matches!(
            dependency,
            FormulaDependency::Volatile | FormulaDependency::External(_)
          )
        })
      });
      formula.volatile = volatile;
      if volatile && formula.formula_state == FormulaState::CachedOnly {
        formula.formula_state = FormulaState::Stale;
      }
    }
  }
}

pub fn translate_shared_formula_text(
  formula: &str,
  origin: CellAddress,
  target: CellAddress,
) -> String {
  crate::parser::translate_shared_formula_text(formula, origin, target)
}

fn decode_excel_escaped_text(value: &str) -> String {
  let mut output = String::with_capacity(value.len());
  let mut chars = value.chars().peekable();
  while let Some(ch) = chars.next() {
    if ch == '_' && chars.peek() == Some(&'x') {
      let mut escape = String::new();
      for _ in 0..6 {
        if let Some(next) = chars.next() {
          escape.push(next);
        }
      }
      if escape.len() == 6
        && escape.starts_with('x')
        && escape.ends_with('_')
        && let Ok(codepoint) = u32::from_str_radix(&escape[1..5], 16)
        && let Some(decoded) = char::from_u32(codepoint)
      {
        output.push(decoded);
        continue;
      }
      output.push('_');
      output.push_str(&escape);
    } else {
      output.push(ch);
    }
  }
  output
}

fn formula_kind(formula: &x::CellFormula) -> FormulaKind {
  match formula.formula_type.unwrap_or_default() {
    x::CellFormulaValues::Normal => FormulaKind::Normal,
    x::CellFormulaValues::Array => FormulaKind::Array,
    x::CellFormulaValues::DataTable => FormulaKind::DataTable,
    x::CellFormulaValues::Shared => match formula.shared_index {
      Some(index) if formula.reference.is_some() => {
        FormulaKind::SharedDefinition { group_index: index }
      }
      Some(index) => FormulaKind::SharedDependent { group_index: index },
      None => FormulaKind::SharedDependent { group_index: 0 },
    },
  }
}

fn shared_formula_groups<'doc>(
  sheets: &[WorksheetValueModel<'doc>],
) -> Vec<SharedFormulaGroup<'doc>> {
  let mut groups = Vec::new();
  for sheet in sheets {
    for (address, record) in &sheet.cells {
      let Some(formula) = &record.formula else {
        continue;
      };
      let FormulaKind::SharedDefinition { group_index } = formula.formula_kind else {
        continue;
      };
      let (origin, range) = shared_formula_group_geometry(sheet, *address, formula);
      groups.push(SharedFormulaGroup {
        index: group_index,
        sheet: sheet.id,
        origin,
        range,
        formula_text: formula.formula_text.clone(),
        dependents: sheet
          .cells
          .iter()
          .filter_map(|(dependent_address, dependent_record)| {
            let dependent_formula = dependent_record.formula.as_ref()?;
            if *dependent_address == origin {
              return None;
            }
            match dependent_formula.formula_kind {
              FormulaKind::SharedDependent {
                group_index: dependent_index,
              } if dependent_index == group_index => Some(*dependent_address),
              FormulaKind::SharedDefinition {
                group_index: dependent_index,
              } if dependent_index == group_index && *dependent_address != origin => {
                Some(*dependent_address)
              }
              FormulaKind::Normal
                if range.is_some_and(|range| range_contains(range, *dependent_address))
                  && dependent_formula.formula_text
                    == translate_shared_formula_text(
                      &formula.formula_text,
                      *address,
                      *dependent_address,
                    ) =>
              {
                Some(*dependent_address)
              }
              _ => None,
            }
          })
          .collect(),
      });
    }
  }
  groups
}

fn range_contains(range: CellRange, address: CellAddress) -> bool {
  let min_column = range.start.column.min(range.end.column);
  let max_column = range.start.column.max(range.end.column);
  let min_row = range.start.row.min(range.end.row);
  let max_row = range.start.row.max(range.end.row);
  (min_column..=max_column).contains(&address.column) && (min_row..=max_row).contains(&address.row)
}

fn shared_formula_group_geometry<'doc>(
  sheet: &WorksheetValueModel<'doc>,
  definition_address: CellAddress,
  definition: &FormulaCell<'doc>,
) -> (CellAddress, Option<CellRange>) {
  let Some(mut range) = definition.reference else {
    return (definition_address, None);
  };
  let mut origin = definition_address;
  while origin.row > 0
    && range.start.column == definition_address.column
    && range.end.column == definition_address.column
    && range.start == definition_address
  {
    let previous = CellAddress {
      column: origin.column,
      row: origin.row - 1,
    };
    let Some(previous_formula) = sheet
      .cells
      .get(&previous)
      .and_then(|record| record.formula.as_ref())
    else {
      break;
    };
    if previous_formula.formula_kind != FormulaKind::Normal {
      break;
    }
    if previous_formula.formula_text
      != translate_shared_formula_text(&definition.formula_text, definition_address, previous)
    {
      break;
    }
    origin = previous;
    range.start = previous;
  }
  (origin, Some(range))
}

fn array_formula_groups<'doc>(
  sheets: &[WorksheetValueModel<'doc>],
) -> Vec<ArrayFormulaGroup<'doc>> {
  let mut groups = Vec::new();
  for sheet in sheets {
    for (address, record) in &sheet.cells {
      let Some(formula) = &record.formula else {
        continue;
      };
      if formula.formula_kind != FormulaKind::Array {
        continue;
      }
      groups.push(ArrayFormulaGroup {
        sheet: sheet.id,
        range: formula.reference.unwrap_or(CellRange {
          start: *address,
          end: *address,
        }),
        formula_text: formula.formula_text.clone(),
        always_calculate: formula.dirty,
      });
    }
  }
  groups
}

fn data_tables<'doc>(sheets: &[WorksheetValueModel<'doc>]) -> Vec<DataTableFormula<'doc>> {
  let mut tables = Vec::new();
  for sheet in sheets {
    for (address, record) in &sheet.cells {
      let Some(formula) = &record.formula else {
        continue;
      };
      if formula.formula_kind != FormulaKind::DataTable {
        continue;
      }
      tables.push(DataTableFormula {
        sheet: sheet.id,
        range: formula.reference.unwrap_or(CellRange {
          start: *address,
          end: *address,
        }),
        input1: formula.input1.clone(),
        input2: formula.input2.clone(),
        input1_deleted: formula.input1_deleted,
        input2_deleted: formula.input2_deleted,
        row_table: formula.data_table_row,
        two_dimensional: formula.data_table2d,
      });
    }
  }
  tables
}

fn workbook_tables<'doc>(
  document: &mut SpreadsheetDocument,
  worksheet_parts: &[WorksheetPart],
  identity: &WorkbookIdentity<'_>,
) -> Result<Vec<FormulaTable<'doc>>> {
  let mut tables = Vec::new();
  for sheet in &identity.sheets {
    let Some(worksheet_part) = worksheet_parts
      .iter()
      .find(|part| part.relationship_id() == sheet.relationship_id.as_deref())
    else {
      continue;
    };
    let table_parts = worksheet_part
      .table_definition_parts(document)
      .collect::<Vec<_>>();
    for table_part in table_parts {
      let table = table_part
        .root_element(document)
        .map_err(|error| FormulaError::Package(error.to_string()))?;
      let range = QualifiedRange::parse_a1(sheet.id, table.reference.as_ref())?.range;
      let totals_rows = table.totals_row_count.unwrap_or_else(|| {
        if table
          .totals_row_shown
          .as_ref()
          .is_some_and(|shown| shown.as_bool())
        {
          1
        } else {
          0
        }
      });
      tables.push(FormulaTable {
        sheet: sheet.id,
        name: Cow::Owned(table.display_name.clone()),
        range,
        header_rows: table.header_row_count.unwrap_or(1),
        totals_rows,
        columns: table
          .table_columns
          .table_column
          .iter()
          .map(|column| Cow::Owned(column.name.clone()))
          .collect(),
      });
    }
  }
  Ok(tables)
}

fn dependency_graph<'doc>(
  sheets: &[WorksheetValueModel<'doc>],
  defined_names: &[DefinedName<'doc>],
) -> DependencyGraph<'doc> {
  let mut graph = DependencyGraphBuilder::default();
  for sheet in sheets {
    for (address, record) in &sheet.cells {
      let Some(formula) = &record.formula else {
        continue;
      };
      let dependencies = formula
        .parsed_formula
        .as_ref()
        .map(|parsed| parsed.dependencies.clone())
        .unwrap_or_else(|| formula_dependencies(sheet.id, &formula.formula_text));
      graph.add_formula(sheet.id, *address, dependencies, formula.volatile);
    }
  }
  for defined_name in defined_names {
    let dependencies = defined_name
      .parsed_formula
      .as_ref()
      .map(|parsed| parsed.dependencies.clone())
      .unwrap_or_else(|| defined_name.dependencies.clone());
    graph.add_defined_name(defined_name.sheet, defined_name.name.clone(), dependencies);
  }
  graph.finish()
}

fn formula_dependencies<'doc>(
  sheet: SheetId,
  formula_text: &Cow<'doc, str>,
) -> Vec<FormulaDependency<'doc>> {
  parse_formula(
    sheet,
    Cow::Owned(formula_text.to_string()),
    FormulaGrammar::ExcelA1,
  )
  .dependencies
}

pub fn parse_formula_text<'doc>(
  current_sheet: SheetId,
  source: impl Into<Cow<'doc, str>>,
) -> ParsedFormula<'doc> {
  parse_formula(current_sheet, source.into(), FormulaGrammar::ExcelA1)
}

pub fn parse_formula_with_context<'doc>(
  context: FormulaParseContext,
  source: impl Into<Cow<'doc, str>>,
) -> ParsedFormula<'doc> {
  let source = source.into();
  if context.grammar == FormulaGrammar::ExcelA1 {
    return parse_formula(context.current_sheet, source, context.grammar);
  }
  let normalized = normalize_formula_text(source.as_ref(), context.grammar);
  parse_formula(
    context.current_sheet,
    Cow::Owned(normalized.into_owned()),
    context.grammar,
  )
}

pub(crate) fn parse_formula<'doc>(
  sheet: SheetId,
  source: Cow<'doc, str>,
  grammar: FormulaGrammar,
) -> ParsedFormula<'doc> {
  lower_formula_parser_formula(sheet, source, grammar)
}

fn lower_formula_parser_formula<'doc>(
  sheet: SheetId,
  source: Cow<'doc, str>,
  grammar: FormulaGrammar,
) -> ParsedFormula<'doc> {
  let parsed = crate::parser::FormulaParser::new(source.as_ref()).parse();
  let body_start = parsed.body_start;
  let text = parsed.body;
  let borrowed_text = match &source {
    Cow::Borrowed(value) => Some(value.get(body_start..).unwrap_or(value)),
    Cow::Owned(_) => None,
  };
  let lowered = lower_formula_parser_body(sheet, text, borrowed_text, parsed.body_parse);

  ParsedFormula {
    source,
    grammar,
    tokens: lowered.tokens,
    body_start,
    code: lowered.code,
    dependencies: lowered.dependencies,
    unsupported: lowered.unsupported,
  }
}

fn parse_array_constant_formula<'doc>(formula: &str) -> Option<Vec<Vec<FormulaValue<'doc>>>> {
  crate::parser::parse_array_constant(formula).map(|array| {
    array
      .rows
      .into_iter()
      .map(|row| {
        row
          .into_iter()
          .map(formula_value_from_array_constant)
          .collect()
      })
      .collect()
  })
}

struct LoweredFormula<'doc> {
  tokens: Vec<FormulaToken<'doc>>,
  code: Option<FormulaCode<'doc>>,
  dependencies: Vec<FormulaDependency<'doc>>,
  unsupported: Vec<UnsupportedFormulaFeature<'doc>>,
}

fn lower_formula_parser_body<'doc>(
  sheet: SheetId,
  text: &str,
  borrowed_text: Option<&'doc str>,
  parsed: crate::parser::FormulaBodyParse,
) -> LoweredFormula<'doc> {
  let crate::parser::FormulaBodyParse {
    tokens: parsed_tokens,
    ast: parser_ast,
    issues,
  } = parsed;
  let mut tokens = Vec::with_capacity(parsed_tokens.len());
  let mut unsupported = Vec::new();

  for token in parsed_tokens {
    match token.kind {
      crate::parser::FormulaBodyTokenKind::Text => {
        tokens.push(FormulaToken::Literal(formula_text_value(
          text,
          borrowed_text,
          token.span.start,
        )));
      }
      crate::parser::FormulaBodyTokenKind::Number(value) => {
        tokens.push(FormulaToken::Literal(FormulaValue::Number(value)));
      }
      crate::parser::FormulaBodyTokenKind::Error(error) => {
        tokens.push(FormulaToken::Literal(FormulaValue::Error(
          formula_error_from_lex(error),
        )));
      }
      crate::parser::FormulaBodyTokenKind::Operator(operator) => {
        tokens.push(FormulaToken::Operator(formula_operator_from_lex(operator)));
      }
      crate::parser::FormulaBodyTokenKind::ArrayOpen => {
        tokens.push(FormulaToken::ArrayOpen);
      }
      crate::parser::FormulaBodyTokenKind::ArrayClose => {
        tokens.push(FormulaToken::ArrayClose);
      }
      crate::parser::FormulaBodyTokenKind::ArgumentSeparator => {
        tokens.push(FormulaToken::Separator(FormulaSeparator::Argument));
      }
      crate::parser::FormulaBodyTokenKind::RowSeparator => {
        tokens.push(FormulaToken::Separator(FormulaSeparator::Row));
      }
      crate::parser::FormulaBodyTokenKind::Function { volatile: _ } => {
        let word = cow_span_text(text, borrowed_text, token.span);
        tokens.push(FormulaToken::Function(word));
      }
      crate::parser::FormulaBodyTokenKind::Boolean(value) => {
        tokens.push(FormulaToken::Literal(FormulaValue::Boolean(value)));
      }
      crate::parser::FormulaBodyTokenKind::ExternalReference(reference) => {
        let external = external_reference_id_from_spans(text, borrowed_text, reference);
        tokens.push(FormulaToken::ExternalReference(external));
      }
      crate::parser::FormulaBodyTokenKind::ReferenceCandidate => {
        let word = cow_span_text(text, borrowed_text, token.span);
        if let Some(range) = crate::parser::parse_formula_range(sheet, word.as_ref()) {
          tokens.push(FormulaToken::Reference(range));
        } else {
          tokens.push(FormulaToken::Name(word));
        }
      }
      crate::parser::FormulaBodyTokenKind::Name => {
        let word = cow_span_text(text, borrowed_text, token.span);
        tokens.push(FormulaToken::Name(word));
      }
      crate::parser::FormulaBodyTokenKind::Unsupported => {
        tokens.push(FormulaToken::Unsupported(cow_span_text(
          text,
          borrowed_text,
          token.span,
        )));
      }
    }
  }

  unsupported.extend(formula_parse_issues_to_unsupported(
    text,
    borrowed_text,
    issues,
  ));
  let code = FormulaCode::from_parser_ast(sheet, text, borrowed_text, parser_ast.as_ref());
  if code.is_none()
    && parser_ast.is_some()
    && !unsupported
      .iter()
      .any(|issue| issue.reason.as_ref() == "formula expression is not fully parsed")
  {
    unsupported.push(UnsupportedFormulaFeature {
      feature: borrowed_text
        .map(Cow::Borrowed)
        .unwrap_or_else(|| Cow::Owned(text.to_string())),
      reason: Cow::Borrowed("formula expression is not fully parsed"),
    });
  }
  let dependencies = dependencies_from_code(sheet, code.as_ref());

  LoweredFormula {
    tokens,
    code,
    dependencies,
    unsupported,
  }
}

fn formula_parse_issues_to_unsupported<'doc>(
  text: &str,
  borrowed_text: Option<&'doc str>,
  issues: Vec<crate::parser::FormulaParseIssue>,
) -> Vec<UnsupportedFormulaFeature<'doc>> {
  issues
    .into_iter()
    .map(|issue| match issue {
      crate::parser::FormulaParseIssue::UnrecognizedCharacter(span) => UnsupportedFormulaFeature {
        feature: cow_span_text(text, borrowed_text, span),
        reason: Cow::Borrowed("unrecognized formula character"),
      },
      crate::parser::FormulaParseIssue::MissingClosingParenthesis => UnsupportedFormulaFeature {
        feature: Cow::Borrowed("parenthesized expression"),
        reason: Cow::Borrowed("missing closing parenthesis"),
      },
      crate::parser::FormulaParseIssue::IncompleteExpression => UnsupportedFormulaFeature {
        feature: borrowed_text
          .map(Cow::Borrowed)
          .unwrap_or_else(|| Cow::Owned(text.to_string())),
        reason: Cow::Borrowed("formula expression is not fully parsed"),
      },
    })
    .collect()
}

fn formula_text_value<'doc>(
  text: &str,
  borrowed_text: Option<&'doc str>,
  start: usize,
) -> FormulaValue<'doc> {
  if let Some(borrowed_text) = borrowed_text {
    return match crate::parser::formula_text_literal(borrowed_text, start) {
      Some(crate::parser::TextLiteral::Borrowed(value)) => {
        FormulaValue::String(Cow::Borrowed(value))
      }
      Some(crate::parser::TextLiteral::Owned(value)) => FormulaValue::String(Cow::Owned(value)),
      None => FormulaValue::String(Cow::Borrowed("")),
    };
  }
  formula_text_value_owned(text, start)
}

fn formula_text_value_owned<'doc>(text: &str, start: usize) -> FormulaValue<'doc> {
  match crate::parser::formula_text_literal(text, start) {
    Some(crate::parser::TextLiteral::Borrowed(value)) => {
      FormulaValue::String(Cow::Owned(value.to_string()))
    }
    Some(crate::parser::TextLiteral::Owned(value)) => FormulaValue::String(Cow::Owned(value)),
    None => FormulaValue::String(Cow::Borrowed("")),
  }
}

fn formula_value_from_cached_text(value: &str) -> FormulaValue<'static> {
  let value = value.trim();
  if value.is_empty() {
    FormulaValue::Blank
  } else if value.starts_with('#') {
    FormulaValue::Error(error_value(value))
  } else if value.eq_ignore_ascii_case("TRUE") {
    FormulaValue::Boolean(true)
  } else if value.eq_ignore_ascii_case("FALSE") {
    FormulaValue::Boolean(false)
  } else if let Ok(number) = value.parse::<f64>() {
    FormulaValue::Number(number)
  } else {
    FormulaValue::String(Cow::Owned(value.to_string()))
  }
}

fn calculation_settings(workbook: &x::Workbook) -> CalculationSettings {
  let properties = workbook.calculation_properties.as_ref();
  CalculationSettings {
    mode: properties
      .and_then(|properties| properties.calculation_mode)
      .map(calculation_mode)
      .unwrap_or_default(),
    full_calculation_on_load: properties
      .and_then(|properties| properties.full_calculation_on_load)
      .is_some_and(|value| value.as_bool()),
    force_full_calculation: properties
      .and_then(|properties| properties.force_full_calculation)
      .is_some_and(|value| value.as_bool()),
    iterate: properties
      .and_then(|properties| properties.iterate)
      .is_some_and(|value| value.as_bool()),
    iterate_count: properties.and_then(|properties| properties.iterate_count),
    iterate_delta: properties.and_then(|properties| properties.iterate_delta),
    full_precision: properties
      .and_then(|properties| properties.full_precision)
      .map(|value| value.as_bool())
      .unwrap_or(true),
    date_1904: workbook
      .workbook_properties
      .as_ref()
      .and_then(|properties| properties.date1904)
      .is_some_and(|value| value.as_bool()),
  }
}

fn calculation_mode(value: x::CalculateModeValues) -> CalculationMode {
  match value {
    x::CalculateModeValues::Manual => CalculationMode::Manual,
    x::CalculateModeValues::Auto => CalculationMode::Auto,
    x::CalculateModeValues::AutoNoTable => CalculationMode::AutoNoTable,
  }
}

fn reference_style(value: x::ReferenceModeValues) -> ReferenceStyle {
  match value {
    x::ReferenceModeValues::A1 => ReferenceStyle::A1,
    x::ReferenceModeValues::R1c1 => ReferenceStyle::R1C1,
  }
}

fn calc_chain(
  document: &mut SpreadsheetDocument,
  workbook_part: &WorkbookPart,
) -> Result<Vec<CalcChainEntry>> {
  let Some(part) = workbook_part.calculation_chain_part(document) else {
    return Ok(Vec::new());
  };
  let chain = part
    .root_element(document)
    .map_err(|error| FormulaError::Package(error.to_string()))?;
  Ok(
    chain
      .calculation_cell
      .iter()
      .filter_map(|cell| {
        let address = CellAddress::parse_a1(cell.cell_reference.as_str()).ok()?;
        Some(CalcChainEntry {
          sheet: cell
            .sheet_id
            .and_then(|sheet| u32::try_from(sheet).ok().map(SheetId)),
          cell: address,
          child_chain: cell.in_child_chain.is_some_and(|value| value.as_bool()),
        })
      })
      .collect(),
  )
}

fn external_references<'doc>(
  document: &mut SpreadsheetDocument,
  workbook_part: &WorkbookPart,
  workbook: &'doc x::Workbook,
) -> Result<Vec<ExternalReference<'doc>>> {
  let reference_ids = workbook
    .external_references
    .as_ref()
    .map(|references| {
      references
        .external_reference
        .iter()
        .map(|reference| reference.id.clone())
        .collect::<Vec<_>>()
    })
    .unwrap_or_default();
  let external_parts = ordered_external_workbook_parts(document, workbook_part, &reference_ids);
  let mut references = Vec::with_capacity(reference_ids.len().max(external_parts.len()));
  for (index, part) in external_parts.iter().enumerate() {
    let id = workbook_part
      .get_id_of_part(document, part)
      .map(|id| Cow::Owned(id.to_string()))
      .or_else(|| reference_ids.get(index).map(|id| Cow::Owned(id.clone())))
      .unwrap_or_else(|| Cow::Owned(format!("rId{}", index + 1)));
    references.push(external_reference_from_part(document, part, id, index + 1)?);
  }
  Ok(references)
}

fn external_reference_from_part<'doc>(
  document: &mut SpreadsheetDocument,
  part: &ooxmlsdk::parts::external_workbook_part::ExternalWorkbookPart,
  id: Cow<'doc, str>,
  link_index: usize,
) -> Result<ExternalReference<'doc>> {
  let Some((book_relationship_id, sheet_names, defined_names)) = ({
    let link = part
      .root_element(document)
      .map_err(|error| FormulaError::Package(error.to_string()))?;
    if let Some(x::ExternalLinkChoice::ExternalBook(book)) = &link.external_link_choice {
      Some((
        book.id.clone(),
        book
          .sheet_names
          .as_ref()
          .map(|names| {
            names
              .sheet_name
              .iter()
              .map(|name| Cow::Owned(name.val.clone().unwrap_or_default()))
              .collect::<Vec<_>>()
          })
          .unwrap_or_default(),
        book
          .external_defined_names
          .as_ref()
          .map(|names| {
            names
              .external_defined_name
              .iter()
              .map(|name| external_defined_name(link_index, name))
              .map(DefinedName::into_owned)
              .collect::<Vec<_>>()
          })
          .unwrap_or_default(),
      ))
    } else {
      None
    }
  }) else {
    return Ok(ExternalReference {
      id,
      target: None,
      sheet_names: Vec::new(),
      defined_names: Vec::new(),
      unavailable: true,
    });
  };
  let target = part
    .get_external_relationship(document, book_relationship_id.as_str())
    .map(|relationship| Cow::Owned(relationship.target().to_string()));
  Ok(ExternalReference {
    id,
    target,
    sheet_names,
    defined_names,
    unavailable: false,
  })
}

fn external_defined_name<'doc>(
  link_index: usize,
  name: &'doc x::ExternalDefinedName,
) -> DefinedName<'doc> {
  let formula_text: Cow<'doc, str> = Cow::Owned(normalize_external_defined_name_formula(
    link_index,
    name.refers_to.as_deref().unwrap_or_default(),
  ));
  let parsed_formula = Some(parse_formula(
    SheetId(name.sheet_id.unwrap_or_default()),
    formula_text.clone(),
    FormulaGrammar::ExcelA1,
  ));
  let dependencies = parsed_formula
    .as_ref()
    .map(|parsed| parsed.dependencies.clone())
    .unwrap_or_default();
  DefinedName {
    name: Cow::Borrowed(name.name.as_str()),
    sheet: name.sheet_id.map(SheetId),
    formula_text,
    parsed_formula,
    dependencies,
    hidden: false,
    built_in: None,
  }
}

fn normalize_external_defined_name_formula(link_index: usize, formula: &str) -> String {
  let formula = formula.trim().strip_prefix('=').unwrap_or(formula.trim());
  if formula.starts_with('[') {
    return formula.to_string();
  }
  let Some((sheet, reference)) = formula.split_once('!') else {
    return formula.to_string();
  };
  let sheet = sheet.trim_matches('\'');
  format!("[{link_index}]{sheet}!{reference}")
}

fn external_cached_cells<'doc>(
  document: &mut SpreadsheetDocument,
  workbook_part: &WorkbookPart,
  workbook: &x::Workbook,
) -> Result<Vec<ExternalCachedCell<'doc>>> {
  let reference_ids = workbook
    .external_references
    .as_ref()
    .map(|references| {
      references
        .external_reference
        .iter()
        .map(|reference| reference.id.clone())
        .collect::<Vec<_>>()
    })
    .unwrap_or_default();
  let external_parts = ordered_external_workbook_parts(document, workbook_part, &reference_ids);
  external_parts
    .iter()
    .enumerate()
    .map(|(index, part)| external_cached_cells_from_part(document, part, index + 1))
    .collect::<Result<Vec<_>>>()
    .map(|cells| cells.into_iter().flatten().collect())
}

fn ordered_external_workbook_parts(
  document: &SpreadsheetDocument,
  workbook_part: &WorkbookPart,
  reference_ids: &[String],
) -> Vec<ooxmlsdk::parts::external_workbook_part::ExternalWorkbookPart> {
  let parts = workbook_part
    .external_workbook_parts(document)
    .collect::<Vec<_>>();
  if reference_ids.is_empty() {
    return parts;
  }

  let mut ordered = Vec::with_capacity(parts.len());
  for reference_id in reference_ids {
    if let Some(part) = parts
      .iter()
      .find(|part| workbook_part.get_id_of_part(document, *part) == Some(reference_id.as_str()))
    {
      ordered.push(part.clone());
    }
  }
  for part in parts {
    if !ordered.iter().any(|ordered_part| ordered_part == &part) {
      ordered.push(part);
    }
  }
  ordered
}

fn external_cached_cells_from_part<'doc>(
  document: &mut SpreadsheetDocument,
  part: &ooxmlsdk::parts::external_workbook_part::ExternalWorkbookPart,
  link_index: usize,
) -> Result<Vec<ExternalCachedCell<'doc>>> {
  let link = part
    .root_element(document)
    .map_err(|error| FormulaError::Package(error.to_string()))?;
  let Some(x::ExternalLinkChoice::ExternalBook(book)) = &link.external_link_choice else {
    return Ok(Vec::new());
  };
  let sheet_names = book
    .sheet_names
    .as_ref()
    .map(|names| {
      names
        .sheet_name
        .iter()
        .map(|name| name.val.clone().unwrap_or_default())
        .collect::<Vec<_>>()
    })
    .unwrap_or_default();
  let Some(data_set) = &book.sheet_data_set else {
    return Ok(Vec::new());
  };
  let mut cells = Vec::new();
  for sheet_data in &data_set.external_sheet_data {
    let sheet_name = sheet_names
      .get(sheet_data.sheet_id as usize)
      .cloned()
      .unwrap_or_else(|| sheet_data.sheet_id.to_string());
    for row in &sheet_data.external_row {
      for cell in &row.external_cell {
        if let Some(value) = cell
          .xstring
          .as_ref()
          .and_then(|value| value.xml_content.as_ref())
        {
          cells.push(ExternalCachedCell {
            link_index,
            sheet_name: Cow::Owned(sheet_name.clone()),
            reference: Cow::Owned(cell.cell_reference.clone()),
            value: formula_value_from_cached_text(value),
          });
        }
      }
    }
  }
  Ok(cells)
}

fn defined_names<'doc>(
  workbook: &'doc x::Workbook,
  identity: &WorkbookIdentity<'doc>,
) -> Vec<DefinedName<'doc>> {
  workbook
    .defined_names
    .as_ref()
    .map(|defined_names| {
      defined_names
        .defined_name
        .iter()
        .map(|name| {
          let sheet = name
            .local_sheet_id
            .and_then(|index| usize::try_from(index).ok())
            .and_then(|index| identity.sheets.get(index))
            .map(|sheet| sheet.id);
          let formula_text: Cow<'doc, str> = name
            .xml_content
            .as_deref()
            .map(Cow::Borrowed)
            .unwrap_or(Cow::Borrowed(""));
          let parsed_formula = Some(parse_formula(
            sheet.unwrap_or_default(),
            formula_text.clone(),
            FormulaGrammar::ExcelA1,
          ));
          let dependencies = parsed_formula
            .as_ref()
            .map(|parsed| parsed.dependencies.clone())
            .unwrap_or_default();
          DefinedName {
            name: Cow::Borrowed(name.name.as_str()),
            sheet,
            formula_text,
            parsed_formula,
            dependencies,
            hidden: name.hidden.is_some_and(|value| value.as_bool()),
            built_in: built_in_name(&name.name),
          }
        })
        .collect()
    })
    .unwrap_or_default()
}

fn built_in_name(name: &str) -> Option<BuiltInName> {
  let base = name
    .strip_prefix("_xlnm.")
    .or_else(|| name.strip_prefix("_XLNM."))
    .unwrap_or(name);
  if base.eq_ignore_ascii_case("Print_Area") {
    Some(BuiltInName::PrintArea)
  } else if base.eq_ignore_ascii_case("Print_Titles") {
    Some(BuiltInName::PrintTitles)
  } else if base.eq_ignore_ascii_case("Criteria") {
    Some(BuiltInName::Criteria)
  } else if base.eq_ignore_ascii_case("Extract") {
    Some(BuiltInName::Extract)
  } else if base.eq_ignore_ascii_case("Database") {
    Some(BuiltInName::Database)
  } else if base.eq_ignore_ascii_case("Sheet_Title") {
    Some(BuiltInName::SheetTitle)
  } else if base.eq_ignore_ascii_case("Consolidate_Area") {
    Some(BuiltInName::ConsolidateArea)
  } else if base.eq_ignore_ascii_case("_FilterDatabase") {
    Some(BuiltInName::FilterDatabase)
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::calc::complex::FormulaComplex;
  use crate::code::FormulaOp;
  use crate::evaluator::valid_date_serial_with_system;
  use crate::function::format_complex_result;

  #[test]
  fn parses_odf_range_endpoints_with_inherited_sheet_name() {
    let same_sheet = crate::parser::parse_formula_range(SheetId(3), ".B8:.B95").unwrap();
    assert_eq!(same_sheet.sheet, SheetId(3));
    assert!(same_sheet.sheet_name.is_none());
    assert_eq!(same_sheet.range.start, CellAddress { column: 1, row: 7 });
    assert_eq!(same_sheet.range.end, CellAddress { column: 1, row: 94 });

    let inherited = crate::parser::parse_formula_range(SheetId(3), "Sheet2.C2:.C92").unwrap();
    assert_eq!(inherited.sheet, SheetId(3));
    assert_eq!(inherited.sheet_name.unwrap().0, "Sheet2");
    assert_eq!(inherited.range.start, CellAddress { column: 2, row: 1 });
    assert_eq!(inherited.range.end, CellAddress { column: 2, row: 91 });
  }

  #[test]
  fn imports_workbook_identity_from_typed_schema() {
    let workbook = x::Workbook {
      workbook_properties: Some(x::WorkbookProperties {
        date1904: Some(ooxmlsdk::simple_type::BooleanValue::True),
        ..x::WorkbookProperties::default()
      }),
      calculation_properties: Some(x::CalculationProperties {
        reference_mode: Some(x::ReferenceModeValues::R1c1),
        ..x::CalculationProperties::default()
      }),
      sheets: Box::new(x::Sheets {
        sheet: vec![x::Sheet {
          name: "Sheet1".to_string(),
          sheet_id: 7,
          id: "rId1".to_string(),
          state: Some(x::SheetStateValues::Hidden),
          ..x::Sheet::default()
        }],
      }),
      ..x::Workbook::default()
    };

    let identity = workbook_identity(&workbook);

    assert_eq!(identity.date_system, DateSystem::Date1904);
    assert_eq!(identity.reference_style, ReferenceStyle::R1C1);
    assert_eq!(identity.sheets[0].id, SheetId(7));
    assert_eq!(identity.sheets[0].name, Cow::Borrowed("Sheet1"));
    assert!(!identity.sheets[0].visible);
  }

  #[test]
  fn imports_cached_formula_cell_without_evaluating() {
    let identity = WorksheetIdentity {
      id: SheetId(1),
      name: Cow::Borrowed("Sheet1"),
      relationship_id: Some(Cow::Borrowed("rId1")),
      visible: true,
    };
    let worksheet = x::Worksheet {
      sheet_data: Box::new(x::SheetData {
        row: vec![x::Row {
          row_index: Some(1),
          cell: vec![x::Cell {
            cell_reference: Some("A1".to_string()),
            data_type: Some(x::CellValues::Number),
            cell_formula: Some(x::CellFormula {
              xml_content: Some("1+1".to_string()),
              calculate_cell: Some(ooxmlsdk::simple_type::BooleanValue::True),
              ..x::CellFormula::default()
            }),
            cell_value: Some(x::CellValue {
              xml_content: Some("2".to_string()),
              ..x::CellValue::default()
            }),
            ..x::Cell::default()
          }],
          ..x::Row::default()
        }],
      }),
      ..x::Worksheet::default()
    };

    let sheet = worksheet_value_model(
      &identity,
      Some(&worksheet),
      &[],
      &WorkbookMetadata::default(),
      &WorkbookStyles::default(),
      &WorkbookIdentity::default(),
      &[],
    )
    .unwrap();
    let record = sheet.cells.get(&CellAddress { column: 0, row: 0 }).unwrap();

    assert_eq!(record.raw_value, FormulaValue::Number(2.0));
    let formula = record.formula.as_ref().unwrap();
    assert_eq!(formula.formula_text, Cow::Borrowed("1+1"));
    assert_eq!(formula.formula_state, FormulaState::Stale);
    assert_eq!(formula.cached_value, Some(FormulaValue::Number(2.0)));
  }

  #[test]
  fn shared_formula_text_translation_matches_pdf_import_logic() {
    assert_eq!(
      translate_shared_formula_text(
        "ROUND(B2,12)=ROUND(C2,12)",
        CellAddress { column: 3, row: 1 },
        CellAddress { column: 3, row: 2 }
      ),
      "ROUND(B3,12)=ROUND(C3,12)"
    );
    assert_eq!(
      translate_shared_formula_text(
        "'Input Sheet'!$A1+B$2",
        CellAddress { column: 0, row: 0 },
        CellAddress { column: 2, row: 3 }
      ),
      "'Input Sheet'!$A4+D$2"
    );
    assert_eq!(
      translate_shared_formula_text(
        "A2",
        CellAddress { column: 0, row: 2 },
        CellAddress { column: 0, row: 4 }
      ),
      "A4"
    );
  }

  #[test]
  fn imports_shared_string_cells_as_text_not_indexes() {
    let identity = WorksheetIdentity {
      id: SheetId(1),
      name: Cow::Borrowed("Sheet1"),
      relationship_id: Some(Cow::Borrowed("rId1")),
      visible: true,
    };
    let worksheet = x::Worksheet {
      sheet_data: Box::new(x::SheetData {
        row: vec![x::Row {
          row_index: Some(1),
          cell: vec![x::Cell {
            cell_reference: Some("B1".to_string()),
            data_type: Some(x::CellValues::SharedString),
            cell_value: Some(x::CellValue {
              xml_content: Some("0".to_string()),
              ..x::CellValue::default()
            }),
            ..x::Cell::default()
          }],
          ..x::Row::default()
        }],
      }),
      ..x::Worksheet::default()
    };

    let sheet = worksheet_value_model(
      &identity,
      Some(&worksheet),
      &["Shared".to_string()],
      &WorkbookMetadata::default(),
      &WorkbookStyles::default(),
      &WorkbookIdentity::default(),
      &[],
    )
    .unwrap();
    let record = sheet.cells.get(&CellAddress { column: 1, row: 0 }).unwrap();

    assert_eq!(
      record.raw_value,
      FormulaValue::String(Cow::Borrowed("Shared"))
    );
    assert_eq!(
      record.display_value.as_ref().unwrap().text,
      Cow::Borrowed("Shared")
    );
  }

  #[test]
  fn preserves_cached_number_text_for_display() {
    let identity = WorksheetIdentity {
      id: SheetId(1),
      name: Cow::Borrowed("Sheet1"),
      relationship_id: Some(Cow::Borrowed("rId1")),
      visible: true,
    };
    let worksheet = x::Worksheet {
      sheet_data: Box::new(x::SheetData {
        row: vec![x::Row {
          row_index: Some(1),
          cell: vec![x::Cell {
            cell_reference: Some("A1".to_string()),
            data_type: Some(x::CellValues::Number),
            cell_value: Some(x::CellValue {
              xml_content: Some("4.0999999999999996".to_string()),
              ..x::CellValue::default()
            }),
            ..x::Cell::default()
          }],
          ..x::Row::default()
        }],
      }),
      ..x::Worksheet::default()
    };

    let sheet = worksheet_value_model(
      &identity,
      Some(&worksheet),
      &[],
      &WorkbookMetadata::default(),
      &WorkbookStyles::default(),
      &WorkbookIdentity::default(),
      &[],
    )
    .unwrap();
    let record = sheet.cells.get(&CellAddress { column: 0, row: 0 }).unwrap();

    assert_eq!(record.raw_value, FormulaValue::Number(4.1));
    assert_eq!(
      record.display_value.as_ref().unwrap().text,
      Cow::Borrowed("4.0999999999999996")
    );
  }

  #[test]
  fn collects_shared_formula_groups_and_dependents() {
    let identity = WorksheetIdentity {
      id: SheetId(1),
      name: Cow::Borrowed("Sheet1"),
      relationship_id: Some(Cow::Borrowed("rId1")),
      visible: true,
    };
    let worksheet = x::Worksheet {
      sheet_data: Box::new(x::SheetData {
        row: vec![
          x::Row {
            row_index: Some(1),
            cell: vec![
              x::Cell {
                cell_reference: Some("A1".to_string()),
                cell_formula: Some(x::CellFormula {
                  formula_type: Some(x::CellFormulaValues::Shared),
                  shared_index: Some(7),
                  reference: Some("A1:A2".to_string()),
                  xml_content: Some("B1".to_string()),
                  ..x::CellFormula::default()
                }),
                ..x::Cell::default()
              },
              x::Cell {
                cell_reference: Some("A2".to_string()),
                cell_formula: Some(x::CellFormula {
                  formula_type: Some(x::CellFormulaValues::Shared),
                  shared_index: Some(7),
                  ..x::CellFormula::default()
                }),
                ..x::Cell::default()
              },
            ],
            ..x::Row::default()
          },
          x::Row {
            row_index: Some(3),
            cell: vec![
              x::Cell {
                cell_reference: Some("A3".to_string()),
                cell_formula: Some(x::CellFormula {
                  formula_type: Some(x::CellFormulaValues::Shared),
                  shared_index: Some(8),
                  reference: Some("A3:A5".to_string()),
                  xml_content: Some("A2".to_string()),
                  ..x::CellFormula::default()
                }),
                ..x::Cell::default()
              },
              x::Cell {
                cell_reference: Some("A4".to_string()),
                data_type: Some(x::CellValues::SharedString),
                cell_value: Some(x::CellValue {
                  xml_content: Some("1".to_string()),
                  space: None,
                  xml_other_attrs: Vec::new(),
                }),
                ..x::Cell::default()
              },
              x::Cell {
                cell_reference: Some("A5".to_string()),
                cell_formula: Some(x::CellFormula {
                  formula_type: Some(x::CellFormulaValues::Shared),
                  shared_index: Some(8),
                  ..x::CellFormula::default()
                }),
                ..x::Cell::default()
              },
            ],
            ..x::Row::default()
          },
        ],
      }),
      ..x::Worksheet::default()
    };
    let mut sheets = vec![
      worksheet_value_model(
        &identity,
        Some(&worksheet),
        &["a value".to_string(), "another value".to_string()],
        &WorkbookMetadata::default(),
        &WorkbookStyles::default(),
        &WorkbookIdentity::default(),
        &[],
      )
      .unwrap(),
    ];
    resolve_shared_formula_dependents(&mut sheets);
    let groups = shared_formula_groups(&sheets);
    let group = groups.iter().find(|group| group.index == 7).unwrap();

    assert_eq!(groups.len(), 2);
    assert_eq!(
      group.range,
      Some(CellRange {
        start: CellAddress { column: 0, row: 0 },
        end: CellAddress { column: 0, row: 1 }
      })
    );
    assert_eq!(group.dependents, vec![CellAddress { column: 0, row: 1 }]);
    let dependent = sheets[0]
      .cells
      .get(&CellAddress { column: 0, row: 1 })
      .and_then(|record| record.formula.as_ref())
      .and_then(|formula| formula.parsed_formula.as_ref())
      .unwrap();
    assert!(matches!(
      &dependent.dependencies[0],
      FormulaDependency::Cell {
        sheet: SheetId(1),
        address: CellAddress { column: 1, row: 1 }
      }
    ));
    let sparse_dependent = sheets[0]
      .cells
      .get(&CellAddress { column: 0, row: 4 })
      .and_then(|record| record.formula.as_ref())
      .unwrap();
    assert_eq!(sparse_dependent.formula_text, Cow::Borrowed("A4"));
    assert!(matches!(
      &sparse_dependent
        .parsed_formula
        .as_ref()
        .unwrap()
        .dependencies[0],
      FormulaDependency::Cell {
        sheet: SheetId(1),
        address: CellAddress { column: 0, row: 3 }
      }
    ));
  }

  #[test]
  fn collects_array_and_data_table_formula_metadata() {
    let identity = WorksheetIdentity {
      id: SheetId(1),
      name: Cow::Borrowed("Sheet1"),
      relationship_id: Some(Cow::Borrowed("rId1")),
      visible: true,
    };
    let worksheet = x::Worksheet {
      sheet_data: Box::new(x::SheetData {
        row: vec![x::Row {
          row_index: Some(1),
          cell: vec![
            x::Cell {
              cell_reference: Some("A1".to_string()),
              cell_formula: Some(x::CellFormula {
                formula_type: Some(x::CellFormulaValues::Array),
                reference: Some("A1:B2".to_string()),
                always_calculate_array: Some(ooxmlsdk::simple_type::BooleanValue::True),
                xml_content: Some("SUM(C1:C2)".to_string()),
                ..x::CellFormula::default()
              }),
              ..x::Cell::default()
            },
            x::Cell {
              cell_reference: Some("D1".to_string()),
              cell_formula: Some(x::CellFormula {
                formula_type: Some(x::CellFormulaValues::DataTable),
                reference: Some("D1:E3".to_string()),
                data_table2_d: Some(ooxmlsdk::simple_type::BooleanValue::True),
                data_table_row: Some(ooxmlsdk::simple_type::BooleanValue::True),
                input1_deleted: Some(ooxmlsdk::simple_type::BooleanValue::True),
                r1: Some("B1".to_string()),
                r2: Some("B2".to_string()),
                ..x::CellFormula::default()
              }),
              ..x::Cell::default()
            },
          ],
          ..x::Row::default()
        }],
      }),
      ..x::Worksheet::default()
    };
    let sheet = worksheet_value_model(
      &identity,
      Some(&worksheet),
      &[],
      &WorkbookMetadata::default(),
      &WorkbookStyles::default(),
      &WorkbookIdentity::default(),
      &[],
    )
    .unwrap();
    let arrays = array_formula_groups(std::slice::from_ref(&sheet));
    let tables = data_tables(&[sheet]);

    assert_eq!(arrays.len(), 1);
    assert_eq!(arrays[0].sheet, SheetId(1));
    assert!(arrays[0].always_calculate);
    assert_eq!(tables.len(), 1);
    assert!(tables[0].row_table);
    assert!(tables[0].two_dimensional);
    assert!(tables[0].input1_deleted);
    assert!(!tables[0].input2_deleted);
    assert_eq!(
      tables[0].input1.as_ref().unwrap().range,
      CellRange {
        start: CellAddress { column: 1, row: 0 },
        end: CellAddress { column: 1, row: 0 }
      }
    );
  }

  #[test]
  fn builds_dependency_edges_from_a1_references() {
    let sheet = WorksheetValueModel {
      id: SheetId(1),
      name: Cow::Borrowed("Sheet1"),
      cells: BTreeMap::from([(
        CellAddress { column: 0, row: 0 },
        CellValueRecord {
          formula: Some(FormulaCell {
            address: CellAddress { column: 0, row: 0 },
            formula_kind: FormulaKind::Normal,
            formula_text: Cow::Borrowed("SUM(B1:C2)+D4"),
            reference: None,
            input1: None,
            input2: None,
            data_table_row: false,
            data_table2d: false,
            input1_deleted: false,
            input2_deleted: false,
            assigns_value_to_name: false,
            parsed_formula: None,
            cached_value: None,
            evaluated_value: None,
            formula_state: FormulaState::CachedOnly,
            number_format_context: None,
            dirty: false,
            volatile: false,
          }),
          ..CellValueRecord::default()
        },
      )]),
    };

    let graph = dependency_graph(&[sheet], &[]);

    assert_eq!(graph.nodes.len(), 1);
    assert_eq!(graph.edges.len(), 2);
    assert!(matches!(graph.edges[0].to, FormulaDependency::Range(_)));
    assert!(matches!(graph.edges[1].to, FormulaDependency::Cell { .. }));
  }

  #[test]
  fn parses_formula_tokens_without_evaluating() {
    let parsed = parse_formula(
      SheetId(3),
      Cow::Borrowed("SUM(B1:C2)+D4*2"),
      FormulaGrammar::ExcelA1,
    );

    assert!(matches!(parsed.tokens[0], FormulaToken::Function(_)));
    assert!(
      parsed
        .tokens
        .iter()
        .any(|token| matches!(token, FormulaToken::Reference(_)))
    );
    assert!(
      parsed
        .tokens
        .iter()
        .any(|token| matches!(token, FormulaToken::Literal(FormulaValue::Number(2.0))))
    );
    assert_eq!(parsed.dependencies.len(), 2);
    assert!(matches!(
      parsed.dependencies[0],
      FormulaDependency::Range(_)
    ));
    assert!(matches!(
      parsed.dependencies[1],
      FormulaDependency::Cell {
        address: CellAddress { column: 3, row: 3 },
        ..
      }
    ));
    assert!(matches!(
      parsed.code.as_ref().and_then(|code| code.ops.last()),
      Some(FormulaOp::Binary(FormulaOperator::Add))
    ));
  }

  #[test]
  fn evaluates_supported_arithmetic_and_aggregate_formulas() {
    let mut workbook = WorkbookValueModel {
      identity: WorkbookIdentity {
        sheets: vec![WorksheetIdentity {
          id: SheetId(1),
          name: Cow::Borrowed("Sheet1"),
          visible: true,
          relationship_id: None,
        }],
        ..WorkbookIdentity::default()
      },
      sheets: vec![WorksheetValueModel {
        id: SheetId(1),
        name: Cow::Borrowed("Sheet1"),
        cells: BTreeMap::from([
          (
            CellAddress { column: 0, row: 0 },
            CellValueRecord {
              raw_value: FormulaValue::Number(1.0),
              ..CellValueRecord::default()
            },
          ),
          (
            CellAddress { column: 0, row: 1 },
            CellValueRecord {
              raw_value: FormulaValue::Number(2.0),
              ..CellValueRecord::default()
            },
          ),
          (
            CellAddress { column: 1, row: 0 },
            CellValueRecord {
              formula: Some(FormulaCell {
                address: CellAddress { column: 1, row: 0 },
                formula_kind: FormulaKind::Normal,
                formula_text: Cow::Borrowed("SUM(A1:A2)+3"),
                reference: None,
                input1: None,
                input2: None,
                data_table_row: false,
                data_table2d: false,
                input1_deleted: false,
                input2_deleted: false,
                assigns_value_to_name: false,
                parsed_formula: Some(parse_formula(
                  SheetId(1),
                  Cow::Borrowed("SUM(A1:A2)+3"),
                  FormulaGrammar::ExcelA1,
                )),
                cached_value: Some(FormulaValue::Number(99.0)),
                evaluated_value: None,
                formula_state: FormulaState::CachedOnly,
                number_format_context: None,
                dirty: false,
                volatile: false,
              }),
              ..CellValueRecord::default()
            },
          ),
        ]),
      }],
      ..WorkbookValueModel::default()
    };

    let report = workbook.evaluate_supported_formulas();

    assert_eq!(report.evaluated.len(), 1);
    assert_eq!(report.evaluated[0].value, FormulaValue::Number(6.0));
    assert_eq!(
      workbook
        .cell(SheetId(1), CellAddress { column: 1, row: 0 })
        .and_then(|record| record.formula.as_ref())
        .and_then(|formula| formula.evaluated_value.clone())
        .unwrap(),
      FormulaValue::Number(6.0)
    );
    assert_eq!(
      workbook
        .cell(SheetId(1), CellAddress { column: 1, row: 0 })
        .and_then(|record| record.display_value.clone())
        .unwrap()
        .text,
      Cow::Borrowed("6")
    );
  }

  #[test]
  fn evaluates_if_without_evaluating_unused_branch() {
    let mut workbook = WorkbookValueModel {
      identity: WorkbookIdentity {
        sheets: vec![WorksheetIdentity {
          id: SheetId(1),
          name: Cow::Borrowed("Sheet1"),
          visible: true,
          relationship_id: None,
        }],
        ..WorkbookIdentity::default()
      },
      sheets: vec![WorksheetValueModel {
        id: SheetId(1),
        name: Cow::Borrowed("Sheet1"),
        cells: BTreeMap::from([(
          CellAddress { column: 0, row: 0 },
          CellValueRecord {
            formula: Some(FormulaCell {
              address: CellAddress { column: 0, row: 0 },
              formula_kind: FormulaKind::Normal,
              formula_text: Cow::Borrowed("IF(0,1/0,7)"),
              reference: None,
              input1: None,
              input2: None,
              data_table_row: false,
              data_table2d: false,
              input1_deleted: false,
              input2_deleted: false,
              assigns_value_to_name: false,
              parsed_formula: Some(parse_formula(
                SheetId(1),
                Cow::Borrowed("IF(0,1/0,7)"),
                FormulaGrammar::ExcelA1,
              )),
              cached_value: None,
              evaluated_value: None,
              formula_state: FormulaState::CachedOnly,
              number_format_context: None,
              dirty: false,
              volatile: false,
            }),
            ..CellValueRecord::default()
          },
        )]),
      }],
      ..WorkbookValueModel::default()
    };

    let report = workbook.evaluate_supported_formulas();

    assert_eq!(report.evaluated.len(), 1);
    assert_eq!(report.evaluated[0].value, FormulaValue::Number(7.0));
  }

  #[test]
  fn parses_external_and_volatile_formula_dependencies() {
    let forced = parse_formula(
      SheetId(1),
      Cow::Borrowed("==NOW()"),
      FormulaGrammar::ExcelA1,
    );

    assert!(
      forced
        .dependencies
        .iter()
        .any(|dependency| matches!(dependency, FormulaDependency::Volatile))
    );
    assert!(
      forced
        .tokens
        .iter()
        .any(|token| matches!(token, FormulaToken::Function(name) if name.as_ref() == "NOW"))
    );

    let boolean = parse_formula(SheetId(1), Cow::Borrowed("TRUE"), FormulaGrammar::ExcelA1);

    assert!(
      boolean
        .tokens
        .iter()
        .any(|token| matches!(token, FormulaToken::Literal(FormulaValue::Boolean(true))))
    );
    assert!(!boolean.dependencies.iter().any(
      |dependency| matches!(dependency, FormulaDependency::Name(name) if name.as_ref() == "TRUE")
    ));

    let not_available = parse_formula(SheetId(1), Cow::Borrowed("#N/A"), FormulaGrammar::ExcelA1);

    assert!(not_available.tokens.iter().any(|token| matches!(
      token,
      FormulaToken::Literal(FormulaValue::Error(FormulaErrorValue::NA))
    )));
    assert!(not_available.unsupported.is_empty());

    let quoted_external = parse_formula(
      SheetId(1),
      Cow::Borrowed("[Book.xlsx]'Q''1'!$A$1"),
      FormulaGrammar::ExcelA1,
    );

    assert!(quoted_external.dependencies.iter().any(|dependency| {
      matches!(
        dependency,
        FormulaDependency::External(ExternalReferenceId {
          book: Some(book),
          sheet: Some(sheet),
          name: Some(name),
        }) if book.as_ref() == "Book.xlsx" && sheet.as_ref() == "Q'1" && name.as_ref() == "$A$1"
      )
    }));

    let parsed = parse_formula(
      SheetId(1),
      Cow::Borrowed("RAND()+[Book.xlsx]'Q1'!$A$1+LocalName"),
      FormulaGrammar::ExcelA1,
    );

    assert!(
      parsed
        .dependencies
        .iter()
        .any(|dependency| matches!(dependency, FormulaDependency::Volatile))
    );
    assert!(parsed.dependencies.iter().any(|dependency| {
      matches!(
        dependency,
        FormulaDependency::External(ExternalReferenceId {
          book: Some(book),
          sheet: Some(sheet),
          name: Some(name),
        }) if book.as_ref() == "Book.xlsx" && sheet.as_ref() == "Q1" && name.as_ref() == "$A$1"
      )
    }));
    assert!(
      parsed
        .tokens
        .iter()
        .any(|token| matches!(token, FormulaToken::ExternalReference(_)))
    );
  }

  #[test]
  fn parsed_formula_borrows_spans_from_borrowed_source() {
    let parsed = parse_formula(
      SheetId(1),
      Cow::Borrowed("SUM(LocalName,[Book.xlsx]Sheet1!A1)"),
      FormulaGrammar::ExcelA1,
    );

    assert!(
      parsed
        .tokens
        .iter()
        .any(|token| matches!(token, FormulaToken::Function(Cow::Borrowed("SUM"))))
    );
    assert!(
      parsed
        .tokens
        .iter()
        .any(|token| matches!(token, FormulaToken::Name(Cow::Borrowed("LocalName"))))
    );
    assert!(parsed.tokens.iter().any(|token| {
      matches!(
        token,
        FormulaToken::ExternalReference(ExternalReferenceId {
          book: Some(Cow::Borrowed("Book.xlsx")),
          sheet: Some(Cow::Borrowed("Sheet1")),
          name: Some(Cow::Borrowed("A1")),
        })
      )
    }));

    let Some(FormulaOp::Call { name, .. }) = parsed.code.as_ref().and_then(|code| code.ops.last())
    else {
      panic!("expected formula call");
    };
    assert!(matches!(name, Cow::Borrowed("SUM")));
    assert!(matches!(
      parsed.code.as_ref().and_then(|code| code
        .ops
        .iter()
        .find(|op| matches!(op, FormulaOp::PushName(_)))),
      Some(FormulaOp::PushName(Cow::Borrowed("LocalName")))
    ));
  }

  #[test]
  fn builds_dependency_edges_from_defined_names() {
    let workbook = x::Workbook {
      defined_names: Some(x::DefinedNames {
        defined_name: vec![x::DefinedName {
          name: "LocalName".to_string(),
          local_sheet_id: Some(2),
          xml_content: Some("Sheet1!$A$1:$B$2".to_string()),
          ..x::DefinedName::default()
        }],
      }),
      ..x::Workbook::default()
    };
    let identity = WorkbookIdentity {
      sheets: vec![
        WorksheetIdentity {
          id: SheetId(3),
          name: Cow::Borrowed("First"),
          relationship_id: None,
          visible: true,
        },
        WorksheetIdentity {
          id: SheetId(7),
          name: Cow::Borrowed("Second"),
          relationship_id: None,
          visible: true,
        },
        WorksheetIdentity {
          id: SheetId(9),
          name: Cow::Borrowed("Third"),
          relationship_id: None,
          visible: true,
        },
      ],
      ..WorkbookIdentity::default()
    };
    let names = defined_names(&workbook, &identity);
    let graph = dependency_graph(&[], &names);

    assert_eq!(names[0].sheet, Some(SheetId(9)));
    assert_eq!(graph.defined_name_nodes.len(), 1);
    assert_eq!(graph.defined_name_edges.len(), 1);
    assert!(matches!(
      graph.defined_name_edges[0].to,
      FormulaDependency::Range(_)
    ));
  }

  #[test]
  fn imports_parsed_formula_state_for_formula_cells() {
    let identity = WorksheetIdentity {
      id: SheetId(1),
      name: Cow::Borrowed("Sheet1"),
      relationship_id: Some(Cow::Borrowed("rId1")),
      visible: true,
    };
    let worksheet = x::Worksheet {
      sheet_data: Box::new(x::SheetData {
        row: vec![x::Row {
          row_index: Some(1),
          cell: vec![x::Cell {
            cell_reference: Some("A1".to_string()),
            cell_formula: Some(x::CellFormula {
              xml_content: Some("NOW()+B1".to_string()),
              ..x::CellFormula::default()
            }),
            cell_value: Some(x::CellValue {
              xml_content: Some("1".to_string()),
              ..x::CellValue::default()
            }),
            ..x::Cell::default()
          }],
          ..x::Row::default()
        }],
      }),
      ..x::Worksheet::default()
    };

    let sheet = worksheet_value_model(
      &identity,
      Some(&worksheet),
      &[],
      &WorkbookMetadata::default(),
      &WorkbookStyles::default(),
      &WorkbookIdentity::default(),
      &[],
    )
    .unwrap();
    let formula = sheet
      .cells
      .get(&CellAddress { column: 0, row: 0 })
      .unwrap()
      .formula
      .as_ref()
      .unwrap();
    let parsed = formula.parsed_formula.as_ref().unwrap();

    assert!(formula.volatile);
    assert_eq!(formula.formula_state, FormulaState::Stale);
    assert!(
      parsed
        .tokens
        .iter()
        .any(|token| matches!(token, FormulaToken::Function(name) if name.as_ref() == "NOW"))
    );
    assert!(
      parsed
        .dependencies
        .iter()
        .any(|dependency| matches!(dependency, FormulaDependency::Volatile))
    );
    assert!(parsed.dependencies.iter().any(|dependency| {
      matches!(
        dependency,
        FormulaDependency::Cell {
          address: CellAddress { column: 1, row: 0 },
          ..
        }
      )
    }));
  }

  #[test]
  fn infers_missing_cell_references_per_row_order() {
    let identity = WorksheetIdentity {
      id: SheetId(1),
      name: Cow::Borrowed("Sheet1"),
      relationship_id: Some(Cow::Borrowed("rId1")),
      visible: true,
    };
    let worksheet = x::Worksheet {
      sheet_data: Box::new(x::SheetData {
        row: vec![x::Row {
          row_index: Some(2),
          cell: vec![
            x::Cell {
              cell_value: Some(x::CellValue {
                xml_content: Some("1".to_string()),
                ..x::CellValue::default()
              }),
              ..x::Cell::default()
            },
            x::Cell {
              cell_value: Some(x::CellValue {
                xml_content: Some("2".to_string()),
                ..x::CellValue::default()
              }),
              ..x::Cell::default()
            },
          ],
          ..x::Row::default()
        }],
      }),
      ..x::Worksheet::default()
    };

    let sheet = worksheet_value_model(
      &identity,
      Some(&worksheet),
      &[],
      &WorkbookMetadata::default(),
      &WorkbookStyles::default(),
      &WorkbookIdentity::default(),
      &[],
    )
    .unwrap();

    assert!(sheet.cells.contains_key(&CellAddress { column: 0, row: 1 }));
    assert!(sheet.cells.contains_key(&CellAddress { column: 1, row: 1 }));
  }

  #[test]
  fn recognizes_filter_database_defined_name_like_pdf_import() {
    assert_eq!(
      built_in_name("_xlnm._FilterDatabase"),
      Some(BuiltInName::FilterDatabase)
    );
    assert_eq!(
      built_in_name("_XLNM.Print_Area"),
      Some(BuiltInName::PrintArea)
    );
  }

  #[test]
  fn parses_external_cached_cell_values() {
    assert_eq!(
      formula_value_from_cached_text(" 42.5 "),
      FormulaValue::Number(42.5)
    );
    assert_eq!(
      formula_value_from_cached_text("TRUE"),
      FormulaValue::Boolean(true)
    );
    assert_eq!(
      formula_value_from_cached_text("#DIV/0!"),
      FormulaValue::Error(FormulaErrorValue::Div0)
    );
    assert_eq!(
      formula_value_from_cached_text("East"),
      FormulaValue::String(Cow::Borrowed("East"))
    );
  }

  #[test]
  fn builds_formula_evaluation_book_from_workbook_model() {
    let model = WorkbookValueModel {
      identity: WorkbookIdentity {
        sheets: vec![WorksheetIdentity {
          id: SheetId(7),
          name: Cow::Borrowed("Data"),
          relationship_id: Some(Cow::Borrowed("rId1")),
          visible: true,
        }],
        ..WorkbookIdentity::default()
      },
      sheets: vec![WorksheetValueModel {
        id: SheetId(7),
        name: Cow::Borrowed("Data"),
        cells: BTreeMap::from([(
          CellAddress { column: 0, row: 0 },
          CellValueRecord {
            raw_value: FormulaValue::Number(1.0),
            formula: Some(FormulaCell {
              address: CellAddress { column: 0, row: 0 },
              formula_kind: FormulaKind::Normal,
              formula_text: Cow::Borrowed("SUM(B1:B2)"),
              reference: None,
              input1: None,
              input2: None,
              data_table_row: false,
              data_table2d: false,
              input1_deleted: false,
              input2_deleted: false,
              assigns_value_to_name: false,
              parsed_formula: None,
              cached_value: Some(FormulaValue::Number(3.0)),
              evaluated_value: Some(FormulaValue::Number(5.0)),
              formula_state: FormulaState::Clean,
              number_format_context: None,
              dirty: false,
              volatile: false,
            }),
            display_value: None,
          },
        )]),
      }],
      defined_names: vec![DefinedName {
        name: Cow::Borrowed("NamedArray"),
        sheet: None,
        formula_text: Cow::Borrowed("{1,2;3,4}"),
        parsed_formula: None,
        dependencies: Vec::new(),
        hidden: false,
        built_in: None,
      }],
      external_cached_cells: vec![ExternalCachedCell {
        link_index: 1,
        sheet_name: Cow::Borrowed("Remote"),
        reference: Cow::Borrowed("C3"),
        value: FormulaValue::String(Cow::Borrowed("ok")),
      }],
      ..WorkbookValueModel::default()
    };

    let book = FormulaEvaluationBook::from_workbook_value_model(&model);

    assert_eq!(book.sheet_id_by_name("data"), Some(SheetId(7)));
    assert_eq!(
      book.cell_value(SheetId(7), CellAddress { column: 0, row: 0 }),
      FormulaValue::Number(5.0)
    );
    assert_eq!(
      book
        .defined_arrays
        .get(&DefinedNameKey {
          sheet: None,
          name_upper: "NAMEDARRAY".to_string()
        })
        .unwrap()[1][1],
      FormulaValue::Number(4.0)
    );
    assert_eq!(
      book.external_cell_value(1, "remote", CellAddress { column: 2, row: 2 }),
      FormulaValue::String(Cow::Borrowed("ok"))
    );
  }

  #[test]
  fn evaluator_resolves_defined_names_from_evaluation_book() {
    let mut workbook = WorkbookValueModel {
      identity: WorkbookIdentity {
        sheets: vec![WorksheetIdentity {
          id: SheetId(1),
          name: Cow::Borrowed("Sheet1"),
          visible: true,
          relationship_id: None,
        }],
        ..WorkbookIdentity::default()
      },
      sheets: vec![WorksheetValueModel {
        id: SheetId(1),
        name: Cow::Borrowed("Sheet1"),
        cells: BTreeMap::from([(
          CellAddress { column: 0, row: 0 },
          CellValueRecord {
            formula: Some(FormulaCell {
              address: CellAddress { column: 0, row: 0 },
              formula_kind: FormulaKind::Normal,
              formula_text: Cow::Borrowed("TaxRate*100"),
              reference: None,
              input1: None,
              input2: None,
              data_table_row: false,
              data_table2d: false,
              input1_deleted: false,
              input2_deleted: false,
              assigns_value_to_name: false,
              parsed_formula: Some(parse_formula(
                SheetId(1),
                Cow::Borrowed("TaxRate*100"),
                FormulaGrammar::ExcelA1,
              )),
              cached_value: None,
              evaluated_value: None,
              formula_state: FormulaState::CachedOnly,
              number_format_context: None,
              dirty: false,
              volatile: false,
            }),
            ..CellValueRecord::default()
          },
        )]),
      }],
      defined_names: vec![DefinedName {
        name: Cow::Borrowed("TaxRate"),
        sheet: None,
        formula_text: Cow::Borrowed("0.08"),
        parsed_formula: None,
        dependencies: Vec::new(),
        hidden: true,
        built_in: None,
      }],
      ..WorkbookValueModel::default()
    };

    let report = workbook.evaluate_supported_formulas();

    assert_eq!(report.evaluated[0].value, FormulaValue::Number(8.0));
  }

  #[test]
  fn evaluator_resolves_significant_whitespace_intersection() {
    // Source: LibreOffice sc/qa/unit/ucalc_formula2.cxx::testIntersectionOpExcel.
    let book = FormulaEvaluationBookBuilder::new()
      .with_defined_name(None, "horz", "$B$2:$D$2")
      .with_defined_name(None, "vert", "$C$1:$C$3")
      .with_cell(
        SheetId(1),
        CellAddress { column: 2, row: 1 },
        FormulaValue::Number(1.0),
      )
      .build();

    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress { column: 0, row: 3 }),
        "horz vert"
      ),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress { column: 0, row: 4 }),
        "(horz vert)*2",
      ),
      Some(FormulaValue::Number(2.0))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress { column: 0, row: 5 }),
        "2*(horz vert)",
      ),
      Some(FormulaValue::Number(2.0))
    );
  }

  #[test]
  fn evaluator_resolves_external_cached_cells() {
    let mut workbook = WorkbookValueModel {
      identity: WorkbookIdentity {
        sheets: vec![WorksheetIdentity {
          id: SheetId(1),
          name: Cow::Borrowed("Sheet1"),
          visible: true,
          relationship_id: None,
        }],
        ..WorkbookIdentity::default()
      },
      sheets: vec![WorksheetValueModel {
        id: SheetId(1),
        name: Cow::Borrowed("Sheet1"),
        cells: BTreeMap::from([(
          CellAddress { column: 0, row: 0 },
          CellValueRecord {
            formula: Some(FormulaCell {
              address: CellAddress { column: 0, row: 0 },
              formula_kind: FormulaKind::Normal,
              formula_text: Cow::Borrowed("[1]Remote!C3+1"),
              reference: None,
              input1: None,
              input2: None,
              data_table_row: false,
              data_table2d: false,
              input1_deleted: false,
              input2_deleted: false,
              assigns_value_to_name: false,
              parsed_formula: Some(parse_formula(
                SheetId(1),
                Cow::Borrowed("[1]Remote!C3+1"),
                FormulaGrammar::ExcelA1,
              )),
              cached_value: None,
              evaluated_value: None,
              formula_state: FormulaState::CachedOnly,
              number_format_context: None,
              dirty: false,
              volatile: false,
            }),
            ..CellValueRecord::default()
          },
        )]),
      }],
      external_cached_cells: vec![ExternalCachedCell {
        link_index: 1,
        sheet_name: Cow::Borrowed("Remote"),
        reference: Cow::Borrowed("C3"),
        value: FormulaValue::Number(41.0),
      }],
      ..WorkbookValueModel::default()
    };

    let report = workbook.evaluate_supported_formulas();

    assert_eq!(report.evaluated[0].value, FormulaValue::Number(42.0));
  }

  #[test]
  fn evaluator_resolves_unicode_external_range_vlookup() {
    let book = FormulaEvaluationBook {
      sheet_names: vec![SheetBinding {
        id: SheetId(1),
        name: Cow::Borrowed("Лист1"),
      }],
      cells: BTreeMap::from([(
        (SheetId(1), CellAddress { column: 0, row: 2 }),
        FormulaValue::String(Cow::Borrowed("Товар 1")),
      )]),
      external_cached_cells: BTreeMap::from([
        (
          (1, "Лист1".to_string(), CellAddress { column: 0, row: 2 }),
          FormulaValue::String(Cow::Borrowed("Товар 1")),
        ),
        (
          (1, "Лист1".to_string(), CellAddress { column: 1, row: 2 }),
          FormulaValue::Number(4.5),
        ),
      ]),
      ..FormulaEvaluationBook::default()
    };

    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress { column: 2, row: 2 }),
        "VLOOKUP(A3,[1]Лист1!A3:B23,2,0)"
      ),
      Some(FormulaValue::Number(4.5))
    );
  }

  #[test]
  fn evaluates_formula_dependencies_over_multiple_passes() {
    let mut workbook = WorkbookValueModel {
      identity: WorkbookIdentity {
        sheets: vec![WorksheetIdentity {
          id: SheetId(1),
          name: Cow::Borrowed("Sheet1"),
          visible: true,
          relationship_id: None,
        }],
        ..WorkbookIdentity::default()
      },
      sheets: vec![WorksheetValueModel {
        id: SheetId(1),
        name: Cow::Borrowed("Sheet1"),
        cells: BTreeMap::from([
          (
            CellAddress { column: 0, row: 0 },
            CellValueRecord {
              formula: Some(FormulaCell {
                address: CellAddress { column: 0, row: 0 },
                formula_kind: FormulaKind::Normal,
                formula_text: Cow::Borrowed("1+1"),
                reference: None,
                input1: None,
                input2: None,
                data_table_row: false,
                data_table2d: false,
                input1_deleted: false,
                input2_deleted: false,
                assigns_value_to_name: false,
                parsed_formula: Some(parse_formula(
                  SheetId(1),
                  Cow::Borrowed("1+1"),
                  FormulaGrammar::ExcelA1,
                )),
                cached_value: None,
                evaluated_value: None,
                formula_state: FormulaState::CachedOnly,
                number_format_context: None,
                dirty: false,
                volatile: false,
              }),
              ..CellValueRecord::default()
            },
          ),
          (
            CellAddress { column: 0, row: 1 },
            CellValueRecord {
              formula: Some(FormulaCell {
                address: CellAddress { column: 0, row: 1 },
                formula_kind: FormulaKind::Normal,
                formula_text: Cow::Borrowed("A1+1"),
                reference: None,
                input1: None,
                input2: None,
                data_table_row: false,
                data_table2d: false,
                input1_deleted: false,
                input2_deleted: false,
                assigns_value_to_name: false,
                parsed_formula: Some(parse_formula(
                  SheetId(1),
                  Cow::Borrowed("A1+1"),
                  FormulaGrammar::ExcelA1,
                )),
                cached_value: None,
                evaluated_value: None,
                formula_state: FormulaState::CachedOnly,
                number_format_context: None,
                dirty: false,
                volatile: false,
              }),
              ..CellValueRecord::default()
            },
          ),
        ]),
      }],
      ..WorkbookValueModel::default()
    };

    workbook.evaluate_supported_formulas();

    assert_eq!(
      workbook
        .cell(SheetId(1), CellAddress { column: 0, row: 1 })
        .and_then(|record| record.formula.as_ref())
        .and_then(|formula| formula.evaluated_value.clone()),
      Some(FormulaValue::Number(3.0))
    );
  }

  #[test]
  fn evaluates_array_formula_result_into_target_range() {
    let mut workbook = WorkbookValueModel {
      identity: WorkbookIdentity {
        sheets: vec![WorksheetIdentity {
          id: SheetId(1),
          name: Cow::Borrowed("Sheet1"),
          visible: true,
          relationship_id: None,
        }],
        ..WorkbookIdentity::default()
      },
      sheets: vec![WorksheetValueModel {
        id: SheetId(1),
        name: Cow::Borrowed("Sheet1"),
        cells: BTreeMap::from([
          (
            CellAddress { column: 0, row: 0 },
            CellValueRecord {
              formula: Some(FormulaCell {
                address: CellAddress { column: 0, row: 0 },
                formula_kind: FormulaKind::Array,
                formula_text: Cow::Borrowed("{1,2;3,4}"),
                reference: Some(CellRange::new(
                  CellAddress { column: 0, row: 0 },
                  CellAddress { column: 1, row: 1 },
                )),
                input1: None,
                input2: None,
                data_table_row: false,
                data_table2d: false,
                input1_deleted: false,
                input2_deleted: false,
                assigns_value_to_name: false,
                parsed_formula: Some(parse_formula(
                  SheetId(1),
                  Cow::Borrowed("{1,2;3,4}"),
                  FormulaGrammar::ExcelA1,
                )),
                cached_value: None,
                evaluated_value: None,
                formula_state: FormulaState::CachedOnly,
                number_format_context: None,
                dirty: false,
                volatile: false,
              }),
              ..CellValueRecord::default()
            },
          ),
          (
            CellAddress { column: 1, row: 1 },
            CellValueRecord::default(),
          ),
        ]),
      }],
      ..WorkbookValueModel::default()
    };

    workbook.evaluate_supported_formulas();

    assert_eq!(
      workbook
        .cell(SheetId(1), CellAddress { column: 1, row: 1 })
        .and_then(|record| record.display_value.as_ref())
        .map(|display| display.text.as_ref()),
      Some("4")
    );
  }

  #[test]
  fn evaluator_ceiling_broadcasts_array_arguments_like_libreoffice() {
    let book = FormulaEvaluationBook {
      cells: BTreeMap::from([
        (
          (SheetId(1), CellAddress { column: 10, row: 3 }),
          FormulaValue::Number(7.0),
        ),
        (
          (SheetId(1), CellAddress { column: 10, row: 4 }),
          FormulaValue::Number(8.0),
        ),
        (
          (SheetId(1), CellAddress { column: 10, row: 5 }),
          FormulaValue::Number(9.0),
        ),
        (
          (SheetId(1), CellAddress { column: 11, row: 2 }),
          FormulaValue::Number(2.0),
        ),
        (
          (SheetId(1), CellAddress { column: 12, row: 2 }),
          FormulaValue::Number(3.0),
        ),
      ]),
      ..FormulaEvaluationBook::default()
    };
    let parsed = parse_formula_with_context(
      FormulaParseContext {
        current_sheet: SheetId(1),
        current_cell: Some(CellAddress { column: 11, row: 3 }),
        grammar: FormulaGrammar::OpenFormula,
      },
      Cow::Borrowed("of:=CEILING([.K4:.K6];[.L3:.M3])"),
    );

    assert_eq!(
      book.evaluate_parsed_formula_raw(
        SheetId(1),
        Some(CellAddress { column: 11, row: 3 }),
        &parsed,
        true,
      ),
      Some(FormulaValue::Matrix(vec![
        vec![FormulaValue::Number(8.0), FormulaValue::Number(9.0)],
        vec![FormulaValue::Number(8.0), FormulaValue::Number(9.0)],
        vec![FormulaValue::Number(10.0), FormulaValue::Number(9.0)],
      ]))
    );

    let excel_book = FormulaEvaluationBook {
      cells: BTreeMap::from([
        (
          (SheetId(1), CellAddress { column: 8, row: 8 }),
          FormulaValue::Number(5.6789),
        ),
        (
          (SheetId(1), CellAddress { column: 9, row: 8 }),
          FormulaValue::Number(6.789),
        ),
        (
          (
            SheetId(1),
            CellAddress {
              column: 7,
              row: 211,
            },
          ),
          FormulaValue::Number(3.0),
        ),
      ]),
      ..FormulaEvaluationBook::default()
    };
    assert_eq!(
      excel_book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress {
          column: 9,
          row: 211
        }),
        "CEILING(I9:J9,H210:H213)"
      ),
      Some(FormulaValue::Number(9.0))
    );
    assert_eq!(
      excel_book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress {
          column: 10,
          row: 211
        }),
        "CEILING(I9:J10,1)"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );

    let book = FormulaEvaluationBook {
      cells: BTreeMap::from([
        (
          (SheetId(1), CellAddress { column: 5, row: 9 }),
          FormulaValue::Number(7.0),
        ),
        (
          (SheetId(1), CellAddress { column: 5, row: 10 }),
          FormulaValue::Number(8.0),
        ),
        (
          (SheetId(1), CellAddress { column: 5, row: 11 }),
          FormulaValue::Number(9.0),
        ),
        (
          (SheetId(1), CellAddress { column: 6, row: 8 }),
          FormulaValue::Number(2.0),
        ),
        (
          (SheetId(1), CellAddress { column: 7, row: 8 }),
          FormulaValue::Number(3.0),
        ),
      ]),
      ..FormulaEvaluationBook::default()
    };
    let parsed = parse_formula_with_context(
      FormulaParseContext {
        current_sheet: SheetId(1),
        current_cell: Some(CellAddress { column: 6, row: 9 }),
        grammar: FormulaGrammar::OpenFormula,
      },
      Cow::Borrowed("of:=[.F10:.F12]-[.G9:.H9]*INT([.F10:.F12]/[.G9:.H9])"),
    );

    assert_eq!(
      book.evaluate_parsed_formula_raw(
        SheetId(1),
        Some(CellAddress { column: 6, row: 9 }),
        &parsed,
        true,
      ),
      Some(FormulaValue::Matrix(vec![
        vec![FormulaValue::Number(1.0), FormulaValue::Number(1.0)],
        vec![FormulaValue::Number(0.0), FormulaValue::Number(2.0)],
        vec![FormulaValue::Number(1.0), FormulaValue::Number(0.0)],
      ]))
    );
  }

  #[test]
  fn evaluator_round_matches_libreoffice_corrected_rounding() {
    let book = FormulaEvaluationBook::default();

    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ROUND(1.1267819797944982,12)"),
      Some(FormulaValue::Number(1.126781979795))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ROUND(1.1267819797945,12)"),
      Some(FormulaValue::Number(1.126781979795))
    );
  }

  #[test]
  fn evaluator_concatenate_uses_scalar_arguments_like_excel() {
    let book = FormulaEvaluationBookBuilder::new()
      .with_sheet(SheetId(1), "Formula")
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 0 },
        FormulaValue::String(Cow::Borrowed("x")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 2, row: 0 },
        FormulaValue::String(Cow::Borrowed("y")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 1 },
        FormulaValue::String(Cow::Borrowed("z")),
      )
      .build();

    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress { column: 2, row: 1 }),
        "CONCATENATE(B1:C1,B1:B2)"
      ),
      Some(FormulaValue::String(Cow::Borrowed("yz")))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "CONCAT(B1:C1)"),
      Some(FormulaValue::String(Cow::Borrowed("xy")))
    );
  }

  #[test]
  fn evaluator_forecast_ets_matches_libreoffice_endpoint_search() {
    let book = FormulaEvaluationBook {
      cells: BTreeMap::from([
        (
          (SheetId(1), CellAddress { column: 12, row: 0 }),
          FormulaValue::Number(1.0),
        ),
        (
          (SheetId(1), CellAddress { column: 12, row: 1 }),
          FormulaValue::Number(2.0),
        ),
        (
          (SheetId(1), CellAddress { column: 12, row: 2 }),
          FormulaValue::Number(3.0),
        ),
        (
          (SheetId(1), CellAddress { column: 12, row: 3 }),
          FormulaValue::Number(4.0),
        ),
        (
          (SheetId(1), CellAddress { column: 12, row: 4 }),
          FormulaValue::Number(5.0),
        ),
        (
          (SheetId(1), CellAddress { column: 13, row: 0 }),
          FormulaValue::Number(2.0),
        ),
        (
          (SheetId(1), CellAddress { column: 13, row: 1 }),
          FormulaValue::Number(4.0),
        ),
        (
          (SheetId(1), CellAddress { column: 13, row: 2 }),
          FormulaValue::Number(7.0),
        ),
        (
          (SheetId(1), CellAddress { column: 13, row: 3 }),
          FormulaValue::Number(8.0),
        ),
        (
          (SheetId(1), CellAddress { column: 13, row: 4 }),
          FormulaValue::Number(11.0),
        ),
      ]),
      ..FormulaEvaluationBook::default()
    };

    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        None,
        "ROUND(FORECAST.ETS.ADD(6,N1:N5,M1:M5),12)"
      ),
      Some(FormulaValue::Number(12.5))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        None,
        "ROUND(FORECAST.ETS.MULT(6,N1:N5,M1:M5),12)"
      ),
      Some(FormulaValue::Number(11.908196123559))
    );
  }

  #[test]
  fn evaluator_forecast_ets_stat_uses_excel_argument_order() {
    let mut builder = FormulaEvaluationBookBuilder::new().with_sheet(SheetId(1), "Sheet1");
    let timeline = [
      1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
      18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0,
    ];
    let values = [
      362.0, 385.0, 432.0, 341.0, 382.0, 409.0, 498.0, 387.0, 473.0, 513.0, 582.0, 474.0, 544.0,
      582.0, 681.0, 557.0, 628.0, 707.0, 773.0, 592.0, 627.0, 725.0, 854.0, 661.0,
    ];
    for (row, (x, y)) in timeline.into_iter().zip(values).enumerate() {
      builder = builder
        .with_cell(
          SheetId(1),
          CellAddress {
            column: 0,
            row: row as u32,
          },
          FormulaValue::Number(x),
        )
        .with_cell(
          SheetId(1),
          CellAddress {
            column: 1,
            row: row as u32,
          },
          FormulaValue::Number(y),
        );
    }
    let book = builder.build();

    // Source: LibreOffice sc/qa/unit/data/functions/statistical/fods/forecast.ets.mult.fods.
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        None,
        "FORECAST.ETS.STAT.MULT(B1:B24,A1:A24,1,4,1,1)"
      ),
      Some(FormulaValue::Number(0.75))
    );
  }

  #[test]
  fn evaluator_resolves_structured_table_reference() {
    let book = FormulaEvaluationBook {
      cells: BTreeMap::from([
        (
          (SheetId(1), CellAddress { column: 0, row: 1 }),
          FormulaValue::Number(10.0),
        ),
        (
          (SheetId(1), CellAddress { column: 1, row: 1 }),
          FormulaValue::Number(5.0),
        ),
      ]),
      tables: BTreeMap::from([(
        "MYTABLE1".to_string(),
        FormulaTable {
          sheet: SheetId(1),
          name: Cow::Borrowed("MyTable1"),
          range: CellRange::new(
            CellAddress { column: 0, row: 0 },
            CellAddress { column: 1, row: 2 },
          ),
          header_rows: 1,
          totals_rows: 1,
          columns: vec![
            Cow::Borrowed("This is the first column"),
            Cow::Borrowed("This is the,second column"),
          ],
        },
      )]),
      ..FormulaEvaluationBook::default()
    };
    let formula = parse_formula_text(
      SheetId(1),
      Cow::Borrowed(
        "SUM(MyTable1[[#This Row],[This is the first column]:[This is the,second column]])",
      ),
    );
    assert!(formula.unsupported.is_empty());

    assert_eq!(
      book.evaluate_parsed_formula_raw(
        SheetId(1),
        Some(CellAddress { column: 0, row: 1 }),
        &formula,
        false,
      ),
      Some(FormulaValue::Number(15.0))
    );
  }

  #[test]
  fn evaluator_subtotal_skips_hidden_filtered_and_nested_rows() {
    let book = FormulaEvaluationBook {
      cells: BTreeMap::from([
        (
          (SheetId(1), CellAddress { column: 0, row: 0 }),
          FormulaValue::Number(1.0),
        ),
        (
          (SheetId(1), CellAddress { column: 0, row: 1 }),
          FormulaValue::Number(10.0),
        ),
        (
          (SheetId(1), CellAddress { column: 0, row: 2 }),
          FormulaValue::Number(100.0),
        ),
        (
          (SheetId(1), CellAddress { column: 0, row: 3 }),
          FormulaValue::Number(1000.0),
        ),
      ]),
      formulas: BTreeMap::from([(
        (SheetId(1), CellAddress { column: 0, row: 3 }),
        FormulaText {
          text: Cow::Borrowed("SUBTOTAL(9,A1:A3)"),
          kind: FormulaKind::Normal,
          reference: None,
        },
      )]),
      row_states: BTreeMap::from([
        (
          (SheetId(1), 1),
          FormulaRowState {
            hidden: true,
            filtered: false,
          },
        ),
        (
          (SheetId(1), 2),
          FormulaRowState {
            hidden: false,
            filtered: true,
          },
        ),
      ]),
      ..FormulaEvaluationBook::default()
    };
    let formula = parse_formula_text(SheetId(1), Cow::Borrowed("SUBTOTAL(109,A1:A4)"));
    assert!(formula.unsupported.is_empty());

    assert_eq!(
      book.evaluate_parsed_formula_raw(
        SheetId(1),
        Some(CellAddress { column: 0, row: 4 }),
        &formula,
        false,
      ),
      Some(FormulaValue::Number(1.0))
    );
  }

  #[test]
  fn evaluation_book_evaluates_reference_lookup_and_text_functions() {
    let book = FormulaEvaluationBook {
      sheet_names: vec![SheetBinding {
        id: SheetId(2),
        name: Cow::Borrowed("Data"),
      }],
      cells: BTreeMap::from([
        (
          (SheetId(1), CellAddress { column: 0, row: 0 }),
          FormulaValue::Number(1.0),
        ),
        (
          (SheetId(1), CellAddress { column: 0, row: 1 }),
          FormulaValue::Number(5.0),
        ),
        (
          (SheetId(1), CellAddress { column: 0, row: 2 }),
          FormulaValue::Number(7.0),
        ),
        (
          (SheetId(1), CellAddress { column: 1, row: 0 }),
          FormulaValue::String(Cow::Borrowed("k")),
        ),
        (
          (SheetId(1), CellAddress { column: 2, row: 0 }),
          FormulaValue::Number(9.0),
        ),
        (
          (SheetId(2), CellAddress { column: 0, row: 0 }),
          FormulaValue::Number(1.0),
        ),
        (
          (SheetId(2), CellAddress { column: 0, row: 1 }),
          FormulaValue::Number(4.0),
        ),
        (
          (SheetId(2), CellAddress { column: 1, row: 0 }),
          FormulaValue::String(Cow::Borrowed("a")),
        ),
        (
          (SheetId(2), CellAddress { column: 1, row: 1 }),
          FormulaValue::String(Cow::Borrowed("b")),
        ),
      ]),
      formulas: BTreeMap::from([(
        (SheetId(1), CellAddress { column: 3, row: 0 }),
        FormulaText {
          text: Cow::Borrowed("SUM(A1:A3)"),
          kind: FormulaKind::Normal,
          reference: None,
        },
      )]),
      ..FormulaEvaluationBook::default()
    };

    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress { column: 0, row: 0 }),
        "INDEX(OFFSET(A1,1,0,2,1),2,1)"
      ),
      Some(FormulaValue::Number(7.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "VLOOKUP(\"k\",B1:C1,2)"),
      Some(FormulaValue::Number(9.0))
    );
    let hlookup_text_table = FormulaEvaluationBook {
      cells: BTreeMap::from([
        (
          (SheetId(1), CellAddress { column: 1, row: 0 }),
          FormulaValue::String(Cow::Borrowed("INTEGRAL")),
        ),
        (
          (SheetId(1), CellAddress { column: 2, row: 0 }),
          FormulaValue::String(Cow::Borrowed("DOUBLE")),
        ),
        (
          (SheetId(1), CellAddress { column: 3, row: 0 }),
          FormulaValue::String(Cow::Borrowed("BLANK")),
        ),
        (
          (SheetId(1), CellAddress { column: 4, row: 0 }),
          FormulaValue::String(Cow::Borrowed("STRING")),
        ),
        (
          (SheetId(1), CellAddress { column: 5, row: 0 }),
          FormulaValue::String(Cow::Borrowed("REF")),
        ),
        (
          (SheetId(1), CellAddress { column: 6, row: 0 }),
          FormulaValue::String(Cow::Borrowed("AREA (rows)")),
        ),
        (
          (SheetId(1), CellAddress { column: 4, row: 5 }),
          FormulaValue::String(Cow::Borrowed(" beginsWithSpace")),
        ),
        (
          (SheetId(1), CellAddress { column: 5, row: 5 }),
          FormulaValue::Number(0.0),
        ),
        (
          (SheetId(1), CellAddress { column: 6, row: 5 }),
          FormulaValue::String(Cow::Borrowed("blanksIncl")),
        ),
      ]),
      ..FormulaEvaluationBook::default()
    };
    assert_eq!(
      hlookup_text_table.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "HLOOKUP(\"STRING\",B1:H6,6,TRUE)",
        FormulaGrammar::ExcelA1
      ),
      Some(FormulaValue::String(Cow::Borrowed(" beginsWithSpace")))
    );
    let hlookup_import_shape = FormulaEvaluationBook {
      cells: BTreeMap::from([
        (
          (SheetId(1), CellAddress { column: 1, row: 5 }),
          FormulaValue::String(Cow::Borrowed("INTEGRAL")),
        ),
        (
          (SheetId(1), CellAddress { column: 2, row: 5 }),
          FormulaValue::String(Cow::Borrowed("DOUBLE")),
        ),
        (
          (SheetId(1), CellAddress { column: 3, row: 5 }),
          FormulaValue::String(Cow::Borrowed("BLANK")),
        ),
        (
          (SheetId(1), CellAddress { column: 4, row: 5 }),
          FormulaValue::String(Cow::Borrowed("STRING")),
        ),
        (
          (SheetId(1), CellAddress { column: 5, row: 5 }),
          FormulaValue::String(Cow::Borrowed("REF")),
        ),
        (
          (SheetId(1), CellAddress { column: 6, row: 5 }),
          FormulaValue::String(Cow::Borrowed("AREA (rows)")),
        ),
        (
          (SheetId(1), CellAddress { column: 4, row: 10 }),
          FormulaValue::String(Cow::Borrowed(" beginsWithSpace")),
        ),
        (
          (SheetId(1), CellAddress { column: 5, row: 10 }),
          FormulaValue::Number(0.0),
        ),
        (
          (SheetId(1), CellAddress { column: 6, row: 10 }),
          FormulaValue::String(Cow::Borrowed("blanksIncl")),
        ),
      ]),
      ..FormulaEvaluationBook::default()
    };
    assert_eq!(
      hlookup_import_shape.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress {
          column: 6,
          row: 715
        }),
        "HLOOKUP(\"STRING\",$B$6:$H$12,6, TRUE)"
      ),
      Some(FormulaValue::String(Cow::Borrowed(" beginsWithSpace")))
    );
    let vlookup_import_shape = FormulaEvaluationBook {
      cells: BTreeMap::from([
        (
          (SheetId(1), CellAddress { column: 1, row: 5 }),
          FormulaValue::String(Cow::Borrowed("DOUBLE")),
        ),
        (
          (SheetId(1), CellAddress { column: 1, row: 6 }),
          FormulaValue::Number(0.0),
        ),
        (
          (SheetId(1), CellAddress { column: 1, row: 7 }),
          FormulaValue::Number(1.0),
        ),
        (
          (SheetId(1), CellAddress { column: 1, row: 11 }),
          FormulaValue::Number(-9_999_999_999.0),
        ),
        (
          (SheetId(1), CellAddress { column: 4, row: 11 }),
          FormulaValue::String(Cow::Borrowed("non-numeric")),
        ),
      ]),
      ..FormulaEvaluationBook::default()
    };
    assert_eq!(
      vlookup_import_shape.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress {
          column: 6,
          row: 1471
        }),
        "VLOOKUP(-1,$B$6:$H$12,4,TRUE)"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::NA))
    );
    let quoted_sheet_lookup = FormulaEvaluationBook {
      sheet_names: vec![
        SheetBinding {
          id: SheetId(1),
          name: Cow::Borrowed("Formula"),
        },
        SheetBinding {
          id: SheetId(2),
          name: Cow::Borrowed("DATA TABLE"),
        },
      ],
      cells: BTreeMap::from([
        (
          (SheetId(2), CellAddress { column: 0, row: 7 }),
          FormulaValue::Number(1.0),
        ),
        (
          (SheetId(2), CellAddress { column: 1, row: 7 }),
          FormulaValue::Number(3.0),
        ),
      ]),
      ..FormulaEvaluationBook::default()
    };
    assert_eq!(
      quoted_sheet_lookup.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "VLOOKUP(1,'DATA TABLE'!$A$8:'DATA TABLE'!$B$10,2)",
        FormulaGrammar::ExcelA1
      ),
      Some(FormulaValue::Number(3.0))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=LOOKUP(4;[$Data.A1:.A2];[$Data.B1:.B2])",
        FormulaGrammar::OpenFormula
      ),
      Some(FormulaValue::String(Cow::Borrowed("b")))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "SUM(INDIRECT(\"A2:A3\"))"),
      Some(FormulaValue::Number(12.0))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress { column: 2, row: 4 }),
        "INDIRECT(\"R1C1\",0)"
      ),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress { column: 2, row: 4 }),
        "INDIRECT(\"A1\",0)"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Ref))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "FORMULATEXT(D1)"),
      Some(FormulaValue::String(Cow::Owned("=SUM(A1:A3)".to_string())))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "TIMEVALUE(\"12:00\")"),
      Some(FormulaValue::Number(0.5))
    );

    let mut countif_book = FormulaEvaluationBookBuilder::new();
    for offset in 0..10 {
      countif_book = countif_book.with_cell(
        SheetId(1),
        CellAddress {
          column: 8,
          row: offset,
        },
        FormulaValue::Number(2000.0 + f64::from(offset)),
      );
    }
    countif_book = countif_book.with_cell(
      SheetId(1),
      CellAddress { column: 10, row: 0 },
      FormulaValue::String(Cow::Borrowed(">2006")),
    );
    let countif_book = countif_book.build();
    assert_eq!(
      countif_book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=COUNTIF([.I1:.I10];[.K1])",
        FormulaGrammar::OpenFormula
      ),
      Some(FormulaValue::Number(3.0))
    );

    let blank_countif_book = FormulaEvaluationBookBuilder::new()
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 0 },
        FormulaValue::Blank,
      )
      .with_query_empty_cell(SheetId(1), CellAddress { column: 0, row: 1 })
      .with_query_cell_value(
        SheetId(1),
        CellAddress { column: 0, row: 1 },
        FormulaValue::Number(0.0),
      )
      .build();
    assert_eq!(
      blank_countif_book.evaluate_formula_text(SheetId(1), None, "COUNTIF(A1:A2,0)"),
      Some(FormulaValue::Number(1.0))
    );
    let error_countif_book = FormulaEvaluationBookBuilder::new()
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 0 },
        FormulaValue::Error(FormulaErrorValue::Div0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 1 },
        FormulaValue::Error(FormulaErrorValue::Ref),
      )
      .build();
    assert_eq!(
      error_countif_book.evaluate_formula_text(SheetId(1), None, "COUNTIF(A1:A2,1/0)"),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      error_countif_book.evaluate_formula_text(SheetId(1), None, "COUNTIF(A1:A2,\"<>#DIV/0!\")"),
      Some(FormulaValue::Number(1.0))
    );
    let range_criteria_countif_book = FormulaEvaluationBookBuilder::new()
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 0 },
        FormulaValue::Number(1.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 0 },
        FormulaValue::Number(2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 2, row: 0 },
        FormulaValue::Number(2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 1 },
        FormulaValue::Number(9.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 1 },
        FormulaValue::Number(2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 2, row: 1 },
        FormulaValue::Number(3.0),
      )
      .build();
    assert_eq!(
      range_criteria_countif_book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress { column: 1, row: 9 }),
        "COUNTIF(A1:C1,A2:C2)"
      ),
      Some(FormulaValue::Number(2.0))
    );
    let empty_text_countif_book = FormulaEvaluationBookBuilder::new()
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 0 },
        FormulaValue::String(Cow::Borrowed("")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 1 },
        FormulaValue::Blank,
      )
      .with_query_empty_cell(SheetId(1), CellAddress { column: 0, row: 2 })
      .build();
    assert_eq!(
      empty_text_countif_book.evaluate_formula_text(SheetId(1), None, "COUNTIF(A1:A3,\"=\")"),
      Some(FormulaValue::Number(2.0))
    );
    assert_eq!(
      empty_text_countif_book.evaluate_formula_text(SheetId(1), None, "COUNTIF(A1:A3,\"\")"),
      Some(FormulaValue::Number(3.0))
    );
  }

  #[test]
  fn evaluation_book_evaluates_text_stat_matrix_and_rounding_functions() {
    let book = FormulaEvaluationBook {
      cells: BTreeMap::from([
        (
          (SheetId(1), CellAddress { column: 0, row: 0 }),
          FormulaValue::Number(1.0),
        ),
        (
          (SheetId(1), CellAddress { column: 0, row: 1 }),
          FormulaValue::Number(2.0),
        ),
        (
          (SheetId(1), CellAddress { column: 1, row: 0 }),
          FormulaValue::Number(3.0),
        ),
        (
          (SheetId(1), CellAddress { column: 1, row: 1 }),
          FormulaValue::Number(4.0),
        ),
        (
          (SheetId(1), CellAddress { column: 0, row: 2 }),
          FormulaValue::Number(4.0),
        ),
      ]),
      ..FormulaEvaluationBook::default()
    };

    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        None,
        "LEFT(\"abcdef\",2)&MID(\"abcdef\",3,2)&RIGHT(\"abcdef\",2)"
      ),
      Some(FormulaValue::String(Cow::Owned("abcdef".to_string())))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DOLLAR(1234.567,2)"),
      Some(FormulaValue::String(Cow::Borrowed("$1,234.57")))
    );
    let array_text_book = FormulaEvaluationBookBuilder::new()
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 0 },
        FormulaValue::Number(110.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 0 },
        FormulaValue::Number(120.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 2, row: 0 },
        FormulaValue::Number(90.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 1 },
        FormulaValue::String(Cow::Borrowed("NO")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 1 },
        FormulaValue::String(Cow::Borrowed("O")),
      )
      .build();
    let formula = parse_formula_with_context(
      FormulaParseContext {
        current_sheet: SheetId(1),
        current_cell: None,
        grammar: FormulaGrammar::ExcelA1,
      },
      Cow::Borrowed("MAX(A1:C1*(RIGHT(A2:C2)=\"O\"))"),
    );
    assert_eq!(
      array_text_book.evaluate_parsed_formula_raw(SheetId(1), None, &formula, true),
      Some(FormulaValue::Number(120.0))
    );
    let formula = parse_formula_text(SheetId(1), Cow::Borrowed("LOWER({\"AB\";\"ABC\"})"));
    assert_eq!(
      book.evaluate_parsed_formula_raw(SheetId(1), None, &formula, true),
      Some(FormulaValue::Matrix(vec![
        vec![FormulaValue::String(Cow::Borrowed("ab"))],
        vec![FormulaValue::String(Cow::Borrowed("abc"))],
      ]))
    );
    let formula = parse_formula_text(SheetId(1), Cow::Borrowed("UPPER({\"test\";36526})"));
    assert_eq!(
      book.evaluate_parsed_formula_raw(SheetId(1), None, &formula, true),
      Some(FormulaValue::Matrix(vec![
        vec![FormulaValue::String(Cow::Borrowed("TEST"))],
        vec![FormulaValue::String(Cow::Borrowed("36526"))],
      ]))
    );
    let formula = parse_formula_text(SheetId(1), Cow::Borrowed("UNICODE({\"D\";\"J\"})"));
    assert_eq!(
      book.evaluate_parsed_formula_raw(SheetId(1), None, &formula, true),
      Some(FormulaValue::Matrix(vec![
        vec![FormulaValue::Number(68.0)],
        vec![FormulaValue::Number(74.0)],
      ]))
    );
    assert!(matches!(
      book.evaluate_formula_text(SheetId(1), None, "DEVSQ({1,-2,3,1/0,5})"),
      Some(FormulaValue::Error(_))
    ));
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DEVSQ(E15:E17)"),
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DEVSQ(1)"),
      Some(FormulaValue::Number(0.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "FACT(1)"),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "FACT(4)"),
      Some(FormulaValue::Number(24.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "FACT(1.9)"),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "FACT(-1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=FACT(-1)",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "FACTDOUBLE(6)"),
      Some(FormulaValue::Number(48.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "FACTDOUBLE(5)"),
      Some(FormulaValue::Number(15.0))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=FACTDOUBLE(-0.9)",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "MEDIAN(A1:B2)"),
      Some(FormulaValue::Number(2.5))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "MMULT(A1:B1,A2:A3)"),
      Some(FormulaValue::Number(14.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "CHOOSEROWS(A1:B2,2)"),
      Some(FormulaValue::Matrix(vec![vec![
        FormulaValue::Number(2.0),
        FormulaValue::Number(4.0),
      ]]))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "CEILING(2.1,1)+FLOOR(2.9,1)"),
      Some(FormulaValue::Number(5.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "FLOOR(7.9,,5)"),
      Some(FormulaValue::Number(7.0))
    );
    assert!(matches!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=COM.MICROSOFT.FLOOR(23.5)",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Error(_))
    ));
    assert!(matches!(
      book.evaluate_formula_text(SheetId(1), None, "GCD()"),
      Some(FormulaValue::Error(_))
    ));
    assert!(matches!(
      book.evaluate_formula_text(SheetId(1), None, "LCM()"),
      Some(FormulaValue::Error(_))
    ));
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "COMBIN(2,1)"),
      Some(FormulaValue::Number(2.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "COMBINA(5,2)"),
      Some(FormulaValue::Number(15.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ROUND(2.675,2)"),
      Some(FormulaValue::Number(2.68))
    );
  }

  #[test]
  fn evaluation_book_evaluates_let_and_workday_functions() {
    let book = FormulaEvaluationBook::default();

    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "LET(x,1,x+2)"),
      Some(FormulaValue::Number(3.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "LET(x,1,LET(x,2,x)+x)"),
      Some(FormulaValue::Number(3.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "LET(x,1,x,2,x)"),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "NETWORKDAYS.INTL(1,7)"),
      Some(FormulaValue::Number(5.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "NETWORKDAYS.INTL(1,7,\"1000000\")"),
      Some(FormulaValue::Number(6.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "NETWORKDAYS.INTL(1,7,,{2,3})"),
      Some(FormulaValue::Number(3.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "WORKDAY.INTL(1,5)"),
      Some(FormulaValue::Number(8.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "WORKDAY.INTL(1,5,11)"),
      Some(FormulaValue::Number(6.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "WORKDAY.INTL(1,1,,{2})"),
      Some(FormulaValue::Number(3.0))
    );
  }

  #[test]
  fn evaluation_book_evaluates_pdf_special_formula_paths() {
    let book = FormulaEvaluationBook {
      source_file_name: Some(Cow::Borrowed("book.xlsx")),
      sheet_names: vec![SheetBinding {
        id: SheetId(1),
        name: Cow::Borrowed("Sheet1"),
      }],
      cells: BTreeMap::from([
        (
          (SheetId(1), CellAddress { column: 0, row: 0 }),
          FormulaValue::Number(1.0),
        ),
        (
          (SheetId(1), CellAddress { column: 1, row: 0 }),
          FormulaValue::String(Cow::Borrowed("x")),
        ),
        (
          (SheetId(1), CellAddress { column: 1, row: 1 }),
          FormulaValue::Number(4.0),
        ),
      ]),
      defined_arrays: BTreeMap::from([(
        DefinedNameKey {
          sheet: None,
          name_upper: "EMPTY_ARRAY".to_string(),
        },
        vec![vec![FormulaValue::Number(9.0)]],
      )]),
      ..FormulaEvaluationBook::default()
    };

    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress { column: 1, row: 0 }),
        "CELL(\"filename\")"
      ),
      Some(FormulaValue::String(Cow::Owned(
        "[book.xlsx]Sheet1".to_string()
      )))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress { column: 1, row: 0 }),
        "CELL(\"type\")"
      ),
      Some(FormulaValue::String(Cow::Borrowed("l")))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "empty_array"),
      Some(FormulaValue::Number(9.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "INDIRECT(\"A1:B2\")INDIRECT(\"B1:B2\")"),
      Some(FormulaValue::String(Cow::Borrowed("x")))
    );
    assert_eq!(
      book.evaluate_relative_formula_as_number(
        SheetId(1),
        "A1+1",
        CellAddress { column: 0, row: 0 },
        CellAddress { column: 1, row: 1 },
      ),
      Some(5.0)
    );
    assert!(book.evaluate_relative_formula_as_condition(
      SheetId(1),
      "A1>3",
      CellAddress { column: 0, row: 0 },
      CellAddress { column: 1, row: 1 },
    ));
  }

  #[test]
  fn evaluation_book_evaluates_libreoffice_basic_function_surface() {
    let book = FormulaEvaluationBook {
      sheet_names: vec![SheetBinding {
        id: SheetId(1),
        name: Cow::Borrowed("Sheet1"),
      }],
      cells: BTreeMap::from([(
        (SheetId(1), CellAddress { column: 2, row: 4 }),
        FormulaValue::Number(42.0),
      )]),
      ..FormulaEvaluationBook::default()
    };

    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "SIGN(-3)+SIGN(0)+SIGN(3)"),
      Some(FormulaValue::Number(0.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ROUNDUP(1.21,1)+ROUNDDOWN(1.29,1)"),
      Some(FormulaValue::Number(2.5))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "YEAR(DATE(2024,2,29))"),
      Some(FormulaValue::Number(2024.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "MONTH(DATE(2024,14,1))"),
      Some(FormulaValue::Number(2.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DAY(DATE(2024,3,0))"),
      Some(FormulaValue::Number(29.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "HOUR(TIME(25,30,0))"),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "HOUR(0.4583287037037037)"),
      Some(FormulaValue::Number(10.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "MINUTE(0.42499537037037033)"),
      Some(FormulaValue::Number(11.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "HOUR(\"nospaces\")"),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "HOUR(\"0.75\")"),
      Some(FormulaValue::Number(18.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "HOUR(-1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        None,
        "TRIM(\"  a   b  \")&UPPER(\"c\")&LOWER(\"D\")"
      ),
      Some(FormulaValue::String(Cow::Owned("a bCd".to_string())))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress { column: 2, row: 4 }),
        "CELL(\"address\")"
      ),
      Some(FormulaValue::String(Cow::Owned("$C$5".to_string())))
    );
  }

  #[test]
  fn evaluation_book_evaluates_more_libreoffice_common_functions() {
    let book = FormulaEvaluationBook {
      sheet_names: vec![SheetBinding {
        id: SheetId(1),
        name: Cow::Borrowed("Sheet1"),
      }],
      cells: BTreeMap::from([
        (
          (SheetId(1), CellAddress { column: 0, row: 0 }),
          FormulaValue::Number(2.0),
        ),
        (
          (SheetId(1), CellAddress { column: 1, row: 0 }),
          FormulaValue::Number(3.0),
        ),
        (
          (SheetId(1), CellAddress { column: 0, row: 1 }),
          FormulaValue::Number(4.0),
        ),
        (
          (SheetId(1), CellAddress { column: 8, row: 0 }),
          FormulaValue::Number(12.0),
        ),
      ]),
      formulas: BTreeMap::from([(
        (SheetId(1), CellAddress { column: 0, row: 0 }),
        FormulaText {
          text: Cow::Borrowed("1+1"),
          kind: FormulaKind::Normal,
          reference: None,
        },
      )]),
      ..FormulaEvaluationBook::default()
    };

    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress { column: 2, row: 4 }),
        "ROW()+COLUMN()"
      ),
      Some(FormulaValue::Number(8.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ROWS(A1:B2)+COLUMNS(A1:B2)"),
      Some(FormulaValue::Number(4.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "COUNTBLANK(A1:B2)"),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "SUMSQ(A1:B1)+SUMPRODUCT(A1:B1,A2:B2)"),
      Some(FormulaValue::Number(21.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "MOD(-3,2)+EVEN(3)+ODD(2)"),
      Some(FormulaValue::Number(8.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "PI(A1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Unknown))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "TRUNC(1.234,2)"),
      Some(FormulaValue::Number(1.23))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "TRUNC(31415.92654,)"),
      Some(FormulaValue::Number(31415.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "MROUND(15.5,)"),
      Some(FormulaValue::Error(FormulaErrorValue::Unknown))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "MROUND(1.45,0.1)"),
      Some(FormulaValue::Number(1.5))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "MROUND(5,-2)"),
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=MROUND(5;-2)",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(6.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "POWER(1.1,\"\")"),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "REPLACE(\"ABCDEFG\",2.3,3.9,\"xx\")"),
      Some(FormulaValue::String(Cow::Borrowed("AxxEFG")))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress::parse_a1("M1184").unwrap()),
        "ROUNDDOWN(G7:I7,3)"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    let sumif_book = FormulaEvaluationBookBuilder::new()
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("AA7").unwrap(),
        FormulaValue::Error(FormulaErrorValue::Null),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("AA8").unwrap(),
        FormulaValue::Error(FormulaErrorValue::Div0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("AA9").unwrap(),
        FormulaValue::Error(FormulaErrorValue::Value),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("AA10").unwrap(),
        FormulaValue::Error(FormulaErrorValue::Ref),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("AA11").unwrap(),
        FormulaValue::Error(FormulaErrorValue::Name),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("AA12").unwrap(),
        FormulaValue::Error(FormulaErrorValue::Num),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("AA13").unwrap(),
        FormulaValue::Error(FormulaErrorValue::NA),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("C9").unwrap(),
        FormulaValue::Number(1.1),
      )
      .build();
    assert_eq!(
      sumif_book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress::parse_a1("E1324").unwrap()),
        "SUMIF(AA7:AA13,C10:D10,C7)"
      ),
      Some(FormulaValue::Number(1.1))
    );
    assert_eq!(
      sumif_book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=SUMIF(([.C1:.C5]~[.B1:.B5]~[.D1:.D5]);32;[.B1:.D5])",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Unknown))
    );
    let sumproduct_book = FormulaEvaluationBookBuilder::new()
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("G1330").unwrap(),
        FormulaValue::String(Cow::Borrowed("text")),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("I1330").unwrap(),
        FormulaValue::Number(2.0),
      )
      .build();
    assert_eq!(
      sumproduct_book.evaluate_formula_text(
        SheetId(1),
        None,
        "SUMPRODUCT(G1330:G1330,I1330:I1330)"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    let sumproduct_array_book = FormulaEvaluationBookBuilder::new()
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("K9").unwrap(),
        FormulaValue::String(Cow::Borrowed("Lim")),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("J10").unwrap(),
        FormulaValue::Number(9.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("K10").unwrap(),
        FormulaValue::String(Cow::Borrowed("LimMin")),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("L10").unwrap(),
        FormulaValue::Number(5.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("J11").unwrap(),
        FormulaValue::Number(9.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("K11").unwrap(),
        FormulaValue::String(Cow::Borrowed("LimPla")),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("L11").unwrap(),
        FormulaValue::Number(2.0),
      )
      .build();
    assert_eq!(
      sumproduct_array_book.evaluate_formula_text(
        SheetId(1),
        None,
        "SUMPRODUCT(J10:J11=9,LEFT(K10:K11,LEN(K9))=K9,L10:L11)"
      ),
      Some(FormulaValue::Number(7.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "SUMX2MY2(C1:D1,D1:E1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Div0))
    );
    let xy_book = FormulaEvaluationBookBuilder::new()
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("B8").unwrap(),
        FormulaValue::Number(1.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("B9").unwrap(),
        FormulaValue::Number(2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("B10").unwrap(),
        FormulaValue::Number(534.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("G8").unwrap(),
        FormulaValue::Number(1.001),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("H8").unwrap(),
        FormulaValue::Number(2.999),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("I8").unwrap(),
        FormulaValue::Number(3.5),
      )
      .build();
    assert_eq!(
      xy_book.evaluate_formula_text(SheetId(1), None, "SUMX2MY2(B8:B10,G8:I8)"),
      Some(FormulaValue::Number(285138.753998))
    );
    let xy_bool_book = FormulaEvaluationBookBuilder::new()
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("B16").unwrap(),
        FormulaValue::Boolean(true),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("C16").unwrap(),
        FormulaValue::Number(-1.1),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("B17").unwrap(),
        FormulaValue::Boolean(false),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("B18").unwrap(),
        FormulaValue::String(Cow::Borrowed("TRUE")),
      )
      .build();
    assert_eq!(
      xy_bool_book.evaluate_formula_text(SheetId(1), None, "SUMX2PY2(B16:D16,B16:B18)"),
      Some(FormulaValue::Error(FormulaErrorValue::Div0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "FVSCHEDULE(1000,)"),
      Some(FormulaValue::Error(FormulaErrorValue::Unknown))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "SUM(INDIRECT(\"R1C9\",0))"),
      Some(FormulaValue::Number(12.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "PMT(0.0199/12,0,25000,0,0)"),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        None,
        "PMT(\"0.01\",\"12\",\"200\",\"-300\",\"TRUE\")"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "NPER(0.01,#N/A,-200,300)"),
      Some(FormulaValue::Error(FormulaErrorValue::NA))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ISPMT(0.05,5,0,15000)"),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "RRI(96,10000,-11000)"),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        None,
        "PRICE(\"1999-02-15\",\"2007-11-15\",0.0575,,100,2,0)"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        None,
        "YIELD(DATE(1999,2,15),DATE(2007,11,15),,95.04287,100,2,0)"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    let yield_with_default_basis = book
      .evaluate_formula_text(
        SheetId(1),
        None,
        "YIELD(DATE(1999,2,15),DATE(2007,11,15),0.0575,95.04287,100,2,)",
      )
      .unwrap();
    let FormulaValue::Number(yield_value) = yield_with_default_basis else {
      panic!("expected YIELD to return a number");
    };
    assert!(
      (yield_value - 0.0650000068807552).abs() <= 1.0e-12,
      "unexpected YIELD result: {yield_value}"
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        None,
        "MDURATION(\"2001-01-01\",\"2006-01-01\",0.08,,2,3)"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "HYPGEOM.DIST(3,2,90,100,0)"),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=HYPGEOM.DIST(2;2;90;100;2)",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "LOGNORM.INV(0,0,1)"),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "NORM.INV(0,63,5)"),
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "NORM.S.INV(0)"),
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "LN(0)"),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "LOG10(0)"),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "LOG(-0.03,3)"),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    let formula = parse_formula_text(SheetId(1), Cow::Borrowed("LOG({0.54},{2})"));
    assert_eq!(
      book.evaluate_parsed_formula_raw(SheetId(1), None, &formula, true),
      Some(FormulaValue::Number(-0.8889686876112561))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "POWER(0,-1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "GEOMEAN(C1:C1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=GEOMEAN(0;0.1;0.2;0.3)",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(0.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "HARMEAN(C1:C1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    );
    let range_name_book = FormulaEvaluationBookBuilder::new()
      .with_defined_name(None, "RangeName", "$I$2:$I$4")
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("I2").unwrap(),
        FormulaValue::Number(4.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("I3").unwrap(),
        FormulaValue::Number(2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("I4").unwrap(),
        FormulaValue::Number(3.0),
      )
      .build();
    assert_eq!(
      range_name_book.evaluate_formula_text(SheetId(1), None, "MAX(1,2,3,+RangeName)"),
      Some(FormulaValue::Number(4.0))
    );
    assert_eq!(
      range_name_book.evaluate_formula_text(SheetId(1), None, "MIN(1,2,3,+RangeName)"),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "CHOOSE(2,\"a\",\"b\",\"c\")"),
      Some(FormulaValue::String(Cow::Borrowed("b")))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "CHOOSE(#REF!,1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Ref))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "CHOOSE(2,1,)"),
      Some(FormulaValue::Blank)
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "SWITCH(2>0,1,\"Amy\",2,\"Bob\",\"x\")"),
      Some(FormulaValue::String(Cow::Borrowed("Amy")))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        None,
        "CONCAT(\"a\",\"b\")&EXACT(\"x\",\"x\")&FIND(\"b\",\"abc\")"
      ),
      Some(FormulaValue::String(Cow::Owned("abTRUE2".to_string())))
    );
    let exact_range_formula = parse_formula_with_context(
      FormulaParseContext {
        current_sheet: SheetId(1),
        current_cell: Some(CellAddress {
          column: 9,
          row: 475,
        }),
        grammar: FormulaGrammar::ExcelA1,
      },
      Cow::Borrowed("EXACT(F12:G12,F12:G12)"),
    );
    assert_eq!(
      book.evaluate_parsed_formula_raw(
        SheetId(1),
        Some(CellAddress {
          column: 9,
          row: 475,
        }),
        &exact_range_formula,
        false,
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "SUBSTITUTE(\"a-b-a\",\"a\",\"x\",2)"),
      Some(FormulaValue::String(Cow::Owned("a-b-x".to_string())))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ISFORMULA(A1)"),
      Some(FormulaValue::Boolean(true))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ISFORMULA(A1:A2)"),
      Some(FormulaValue::Boolean(false))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ISNUMBER(TRUE())"),
      Some(FormulaValue::Boolean(false))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=ISNUMBER(TRUE())",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Boolean(true))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "TRUE()=1"),
      Some(FormulaValue::Boolean(false))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=TRUE()=1",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Boolean(true))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "TYPE(TRUE())"),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ERROR.TYPE(#DIV/0!)"),
      Some(FormulaValue::Number(2.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ERROR.TYPE(#null)"),
      Some(FormulaValue::Number(5.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ERRORTYPE(#null)"),
      Some(FormulaValue::Number(525.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ERROR.TYPE(#getting_data)"),
      Some(FormulaValue::Error(FormulaErrorValue::Unknown))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ERRORTYPE(#getting_data)"),
      Some(FormulaValue::Error(FormulaErrorValue::Unknown))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ERRORTYPE(B1)"),
      Some(FormulaValue::Error(FormulaErrorValue::NA))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ENCODEURL(0.08)"),
      Some(FormulaValue::String(Cow::Borrowed("0%2E08")))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "HYPERLINK(\"url\",\"label\")"),
      Some(FormulaValue::String(Cow::Borrowed("label")))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "HYPERLINK(\"url\",1/0)"),
      Some(FormulaValue::Error(FormulaErrorValue::Div0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "FIXED(\"1,234,567.890\",-1)"),
      Some(FormulaValue::String(Cow::Borrowed("1,234,570")))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "FIXED(A1:A2,2)"),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    let regex_search_book = FormulaEvaluationBookBuilder::new()
      .with_formula_search_type(FormulaSearchType::Regex)
      .build();
    assert_eq!(
      regex_search_book.evaluate_formula_text(
        SheetId(1),
        None,
        "SEARCH(\"Gewinn|Promotion|Replay\",\"Gewinn\")"
      ),
      Some(FormulaValue::Number(1.0))
    );
    let literal_search_book = FormulaEvaluationBookBuilder::new()
      .with_formula_search_type(FormulaSearchType::Normal)
      .build();
    assert_eq!(
      literal_search_book.evaluate_formula_text(
        SheetId(1),
        None,
        "SEARCH(\"Gewinn|Promotion|Replay\",\"Gewinn\")"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "AND()"),
      Some(FormulaValue::Error(FormulaErrorValue::Parameter))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "OR()"),
      Some(FormulaValue::Error(FormulaErrorValue::Parameter))
    );
  }

  #[test]
  fn evaluation_book_textjoin_expands_odf_absolute_sheet_whole_column() {
    let book = FormulaEvaluationBookBuilder::new()
      .with_sheet(SheetId(1), "data")
      .with_sheet(SheetId(2), "Sheet2")
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("H1").unwrap(),
        FormulaValue::String(Cow::Borrowed("a")),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("H2").unwrap(),
        FormulaValue::String(Cow::Borrowed("b")),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("H1048575").unwrap(),
        FormulaValue::String(Cow::Borrowed("last_row_but_one")),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("H1048576").unwrap(),
        FormulaValue::String(Cow::Borrowed("last_row")),
      )
      .build();

    // Source: LibreOffice sc/qa/unit/data/functions/fods/Functions_Excel_2016.fods.
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(2),
        Some(CellAddress::parse_a1("A20").unwrap()),
        r#"=TEXTJOIN(".";1;$data.H:H)"#,
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::String(Cow::Borrowed(
        "a.b.last_row_but_one.last_row"
      )))
    );
  }

  #[test]
  fn evaluation_book_textjoin_skips_query_empty_references_only() {
    let book = FormulaEvaluationBookBuilder::new()
      .with_sheet(SheetId(1), "Sheet1")
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("A1").unwrap(),
        FormulaValue::String(Cow::Borrowed("A")),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("B1").unwrap(),
        FormulaValue::Number(0.0),
      )
      .with_query_empty_cell(SheetId(1), CellAddress::parse_a1("B1").unwrap())
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("C1").unwrap(),
        FormulaValue::String(Cow::Borrowed("C")),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("D1").unwrap(),
        FormulaValue::Number(0.0),
      )
      .build();

    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, r#"TEXTJOIN("-",1,A1:C1)"#),
      Some(FormulaValue::String(Cow::Borrowed("A-C")))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, r#"TEXTJOIN("-",1,A1:D1)"#),
      Some(FormulaValue::String(Cow::Borrowed("A-C-0")))
    );
  }

  #[test]
  fn evaluation_book_textjoin_reference_delimiter_uses_last_row() {
    let book = FormulaEvaluationBookBuilder::new()
      .with_sheet(SheetId(1), "Sheet1")
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("A1").unwrap(),
        FormulaValue::String(Cow::Borrowed("x")),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("B1").unwrap(),
        FormulaValue::String(Cow::Borrowed("y")),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("A2").unwrap(),
        FormulaValue::String(Cow::Borrowed(",")),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("B2").unwrap(),
        FormulaValue::String(Cow::Borrowed(";")),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("C1").unwrap(),
        FormulaValue::String(Cow::Borrowed("a")),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("C2").unwrap(),
        FormulaValue::String(Cow::Borrowed("b")),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("C3").unwrap(),
        FormulaValue::String(Cow::Borrowed("c")),
      )
      .build();

    // Source: Apache POI TextJoinFunction follows Excel's documented multi-cell
    // delimiter examples by using only the last row of an area delimiter.
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, r#"TEXTJOIN(A1:B2,TRUE,C1:C3)"#),
      Some(FormulaValue::String(Cow::Borrowed("a,b;c")))
    );
  }

  #[test]
  fn evaluation_book_fourier_real_even_matches_libreoffice() {
    let book = FormulaEvaluationBookBuilder::new()
      .with_sheet(SheetId(1), "Fourier")
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("A1").unwrap(),
        FormulaValue::Number(1.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("A2").unwrap(),
        FormulaValue::Number(2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("A3").unwrap(),
        FormulaValue::Number(3.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("A4").unwrap(),
        FormulaValue::Number(4.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("C1").unwrap(),
        FormulaValue::Number(10.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("D1").unwrap(),
        FormulaValue::Number(0.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("C2").unwrap(),
        FormulaValue::Number(-2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("D2").unwrap(),
        FormulaValue::Number(2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("C3").unwrap(),
        FormulaValue::Number(-2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("D3").unwrap(),
        FormulaValue::Number(0.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("C4").unwrap(),
        FormulaValue::Number(-2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("D4").unwrap(),
        FormulaValue::Number(-2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("B14").unwrap(),
        FormulaValue::Number(-2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("C14").unwrap(),
        FormulaValue::Number(-2.99999),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("B15").unwrap(),
        FormulaValue::Number(-1.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("C15").unwrap(),
        FormulaValue::Number(-1.00001),
      )
      .build();

    // Source: LibreOffice sc/qa/unit/data/functions/array/fods/fourier.fods.
    assert_formula_matrix_numbers_close(
      &book,
      "FOURIER(A1:A4,1)",
      &[&[10.0, 0.0], &[-2.0, 2.0], &[-2.0, 0.0], &[-2.0, -2.0]],
    );
    assert_formula_matrix_numbers_close(
      &book,
      "FOURIER(A1:A4,1,,1)",
      &[
        &[10.0, 0.0],
        &[2.0_f64.sqrt() * 2.0, 3.0 * std::f64::consts::FRAC_PI_4],
        &[2.0, std::f64::consts::PI],
        &[2.0_f64.sqrt() * 2.0, -3.0 * std::f64::consts::FRAC_PI_4],
      ],
    );
    assert_formula_matrix_numbers_close(
      &book,
      "FOURIER(C1:D4,1,1)",
      &[&[1.0, 0.0], &[2.0, 0.0], &[3.0, 0.0], &[4.0, 0.0]],
    );
  }

  #[test]
  fn evaluation_book_builder_and_libreoffice_scalar_array_semantics() {
    let book = FormulaEvaluationBookBuilder::new()
      .with_sheet(SheetId(1), "Formula")
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 0 },
        FormulaValue::Number(1.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 0 },
        FormulaValue::Error(FormulaErrorValue::Div0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 2, row: 0 },
        FormulaValue::String(Cow::Borrowed("x")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 3, row: 0 },
        FormulaValue::String(Cow::Borrowed("")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 5, row: 0 },
        FormulaValue::Number(1.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 6, row: 0 },
        FormulaValue::Error(FormulaErrorValue::Div0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 7, row: 0 },
        FormulaValue::String(Cow::Borrowed("x")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 8, row: 0 },
        FormulaValue::Matrix(vec![
          vec![FormulaValue::String(Cow::Borrowed("anchor"))],
          vec![FormulaValue::String(Cow::Borrowed("spill"))],
        ]),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 4, row: 0 },
        FormulaValue::Number(-3.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 4, row: 1 },
        FormulaValue::Number(4.0),
      )
      .build();

    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "IFERROR(A1,9)"),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "IFERROR(,\"A\")"),
      Some(FormulaValue::Error(FormulaErrorValue::Parameter))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "IFNA(,\"A\")"),
      Some(FormulaValue::Error(FormulaErrorValue::Parameter))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "N(A1:E1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    let formula = parse_formula_text(SheetId(1), Cow::Borrowed("N(A1:E1)"));
    assert_eq!(
      book.evaluate_parsed_formula_raw(SheetId(1), None, &formula, true),
      Some(FormulaValue::Matrix(vec![vec![
        FormulaValue::Number(1.0),
        FormulaValue::Error(FormulaErrorValue::Div0),
        FormulaValue::Number(0.0),
        FormulaValue::Number(0.0),
        FormulaValue::Number(-3.0),
      ]]))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "NOT(B1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Div0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "I1=\"anchor\""),
      Some(FormulaValue::Boolean(true))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "D2"),
      Some(FormulaValue::Number(0.0))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress::parse_a1("E736").unwrap()),
        "INDEX(B14:C15,D7,2)"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "COUNTBLANK(D1)"),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "AVERAGE(C1:D1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Div0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "SUMPRODUCT(ABS(E1:E2),E1:E2+E1:E2)"),
      Some(FormulaValue::Number(14.0))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress { column: 0, row: 61 }),
        "ABS(E1:E2)"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress { column: 0, row: 1 }),
        "ABS(E1:E2)"
      ),
      Some(FormulaValue::Number(4.0))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress { column: 0, row: 61 }),
        "SUM(ABS(E1:E2))"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "SUM(ABS(MUNIT(2)))"),
      Some(FormulaValue::Number(2.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "SUM(ABS(MUNIT(2)*-1))"),
      Some(FormulaValue::Number(2.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "SUMPRODUCT(A1:A1,D1:D1)"),
      Some(FormulaValue::Number(0.0))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress { column: 0, row: 2 }),
        "SUMPRODUCT(--(B5:B20))"
      ),
      Some(FormulaValue::Number(0.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "SUMPRODUCT(A1:A1*D1:D1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "IMDIV(\"-238+240i\",\"10+24i\")"),
      Some(FormulaValue::String(Cow::Borrowed("5+12i")))
    );
    assert_eq!(
      format_complex_result(FormulaComplex::new(
        -0.0787746170678337,
        0.586433260393873,
        'i'
      )),
      "-0.0787746170678337+0.586433260393873i"
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=ATAN2([.E1];[.E2])",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(4.0_f64.atan2(-3.0)))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ATAN2(0,0)"),
      Some(FormulaValue::Error(FormulaErrorValue::Div0))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=ATAN2(0;0)",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(0.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "AGGREGATE(3,4,F1:H1)"),
      Some(FormulaValue::Number(3.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "AGGREGATE(3,6,F1:H1)"),
      Some(FormulaValue::Number(2.0))
    );
    let formula = parse_formula_text(SheetId(1), Cow::Borrowed("ISODD({1;2})"));
    assert_eq!(
      book.evaluate_parsed_formula_raw(SheetId(1), None, &formula, true),
      Some(FormulaValue::Matrix(vec![
        vec![FormulaValue::Boolean(true)],
        vec![FormulaValue::Boolean(false)],
      ]))
    );
    let formula = parse_formula_text(SheetId(1), Cow::Borrowed("POISSON.DIST({140},{120},{0})"));
    assert_eq!(
      book.evaluate_parsed_formula_raw(SheetId(1), None, &formula, true),
      Some(FormulaValue::Number(0.006933086674227154))
    );
    let formula = parse_formula_text(SheetId(1), Cow::Borrowed("ISNUMBER(A1:D1)"));
    assert_eq!(
      book.evaluate_parsed_formula_raw(SheetId(1), None, &formula, true),
      Some(FormulaValue::Matrix(vec![vec![
        FormulaValue::Boolean(true),
        FormulaValue::Boolean(false),
        FormulaValue::Boolean(false),
        FormulaValue::Boolean(false),
      ]]))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "COUNTA(5,,10)"),
      Some(FormulaValue::Number(3.0))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=CONFIDENCE(0;1.5;100)",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=COM.MICROSOFT.CONFIDENCE.NORM(0;1.5;100)",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=COM.MICROSOFT.CONFIDENCE.T(0;1.5;100)",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    );
  }

  #[test]
  fn evaluation_book_countblank_counts_empty_strings_like_excel() {
    let book = FormulaEvaluationBookBuilder::new()
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 0 },
        FormulaValue::Blank,
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 1 },
        FormulaValue::String(Cow::Borrowed("")),
      )
      .with_formula(SheetId(1), CellAddress { column: 0, row: 1 }, "\"\"")
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 2 },
        FormulaValue::Blank,
      )
      .with_formula(SheetId(1), CellAddress { column: 0, row: 2 }, "A1")
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 3 },
        FormulaValue::String(Cow::Borrowed("")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 2, row: 9 },
        FormulaValue::Error(FormulaErrorValue::Div0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 2, row: 10 },
        FormulaValue::Number(42.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 9, row: 13 },
        FormulaValue::Blank,
      )
      .with_cell(
        SheetId(1),
        CellAddress {
          column: 10,
          row: 13,
        },
        FormulaValue::String(Cow::Borrowed("text")),
      )
      .with_cell(
        SheetId(1),
        CellAddress {
          column: 5,
          row: 795,
        },
        FormulaValue::Boolean(true),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 8 },
        FormulaValue::Number(2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 9 },
        FormulaValue::Number(534.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 10 },
        FormulaValue::Number(9_999_999_999.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 11 },
        FormulaValue::Number(-9_999_999_999.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 12 },
        FormulaValue::Number(-534.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 13 },
        FormulaValue::Number(-2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 14 },
        FormulaValue::Number(-1.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 15 },
        FormulaValue::Boolean(true),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 2, row: 5 },
        FormulaValue::String(Cow::Borrowed("DOUBLE")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 3, row: 5 },
        FormulaValue::String(Cow::Borrowed("BLANK")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 4, row: 5 },
        FormulaValue::String(Cow::Borrowed("STRING")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 5, row: 5 },
        FormulaValue::String(Cow::Borrowed("REF")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 6, row: 5 },
        FormulaValue::String(Cow::Borrowed("AREA (rows)")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 2, row: 8 },
        FormulaValue::Number(1.1),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 9, row: 6 },
        FormulaValue::Number(4.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 6, row: 6 },
        FormulaValue::Number(1.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 7, row: 6 },
        FormulaValue::Number(2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 6, row: 7 },
        FormulaValue::Number(3.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 7, row: 7 },
        FormulaValue::Number(4.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 6, row: 8 },
        FormulaValue::Number(5.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 7, row: 8 },
        FormulaValue::Number(6.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 7, row: 9 },
        FormulaValue::Number(3.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 4, row: 10 },
        FormulaValue::String(Cow::Borrowed(" BEGINSWITHSPACE")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 4, row: 7 },
        FormulaValue::String(Cow::Borrowed("nospaces")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 5, row: 10 },
        FormulaValue::Number(0.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 6, row: 10 },
        FormulaValue::String(Cow::Borrowed("Alpha")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 8, row: 10 },
        FormulaValue::Number(345.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 9, row: 10 },
        FormulaValue::Number(0.00001),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 10, row: 7 },
        FormulaValue::Number(2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 11, row: 7 },
        FormulaValue::String(Cow::Borrowed("aa")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 12, row: 6 },
        FormulaValue::String(Cow::Borrowed("alphanum")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 10, row: 8 },
        FormulaValue::String(Cow::Borrowed("b")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 11, row: 8 },
        FormulaValue::Number(4.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 10, row: 9 },
        FormulaValue::Number(5.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 11, row: 9 },
        FormulaValue::Number(6.0),
      )
      .build();

    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "COUNTBLANK(A1:A4)"),
      Some(FormulaValue::Number(3.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ISBLANK(A1)"),
      Some(FormulaValue::Boolean(true))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ISBLANK(A3)"),
      Some(FormulaValue::Boolean(false))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ISBLANK(A1:A4)"),
      Some(FormulaValue::Boolean(false))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress::parse_a1("A10").unwrap()),
        "ISBLANK(A1:B1)"
      ),
      Some(FormulaValue::Boolean(true))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress::parse_a1("G780").unwrap()),
        "ISERROR(C10:C13)"
      ),
      Some(FormulaValue::Boolean(true))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress::parse_a1("G780").unwrap()),
        "ISERR(C10:C13)"
      ),
      Some(FormulaValue::Boolean(true))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress::parse_a1("J780").unwrap()),
        "ISTEXT(J14:K14)"
      ),
      Some(FormulaValue::Boolean(false))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress::parse_a1("K780").unwrap()),
        "ISTEXT(J14:K14)"
      ),
      Some(FormulaValue::Boolean(true))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ISNUMBER(TRUE)"),
      Some(FormulaValue::Boolean(false))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ISNUMBER(F796)"),
      Some(FormulaValue::Boolean(false))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "LARGE(B9:B16,4)"),
      Some(FormulaValue::Number(-1.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "LARGE(TRUE,1)"),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress::parse_a1("J824").unwrap()),
        "LEFT(G7:H9,H10)"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress::parse_a1("K832").unwrap()),
        "LEN(K8:L10)"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress::parse_a1("E872").unwrap()),
        "LOOKUP(\"DOUBLE\",C6:G6,C9:G9)"
      ),
      Some(FormulaValue::Number(1.1))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress::parse_a1("K876").unwrap()),
        "LOWER(E11:G11)"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "MINA(\"-23\",0,4)"),
      Some(FormulaValue::Number(-23.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "MODE(B8:B16)"),
      Some(FormulaValue::Error(FormulaErrorValue::NA))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "MODE(B9,2,D7)"),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress::parse_a1("K932").unwrap()),
        "MOD(I11:L11,J7)"
      ),
      Some(FormulaValue::Number(0.0))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress::parse_a1("H980").unwrap()),
        "NOT(E8)"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "NOT(\"TRUE\")"),
      Some(FormulaValue::Boolean(false))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "NOT(\"nospaces\")"),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress::parse_a1("N1352").unwrap()),
        "T(L7:N7)"
      ),
      Some(FormulaValue::String(Cow::Borrowed("")))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress::parse_a1("M1352").unwrap()),
        "T(M7:N7)"
      ),
      Some(FormulaValue::String(Cow::Borrowed("alphanum")))
    );
    let formula = parse_formula_text(SheetId(1), Cow::Borrowed("ISBLANK(A1:A4)"));
    assert_eq!(
      book.evaluate_parsed_formula_raw(SheetId(1), None, &formula, true),
      Some(FormulaValue::Matrix(vec![
        vec![FormulaValue::Boolean(true)],
        vec![FormulaValue::Boolean(false)],
        vec![FormulaValue::Boolean(false)],
        vec![FormulaValue::Boolean(false)],
      ]))
    );
  }

  #[test]
  fn evaluation_book_offset_scalarizes_reference_arguments_like_excel() {
    let book = FormulaEvaluationBookBuilder::new()
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("G7").unwrap(),
        FormulaValue::Number(1.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress::parse_a1("G8").unwrap(),
        FormulaValue::Number(1.001),
      )
      .build();

    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        Some(CellAddress::parse_a1("G1008").unwrap()),
        "OFFSET(F7,E7:I7,E7:I7)"
      ),
      Some(FormulaValue::Number(1.001))
    );
  }

  #[test]
  fn evaluation_book_countif_honors_match_whole_cell_option() {
    let whole_cell_book = FormulaEvaluationBookBuilder::new()
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 0 },
        FormulaValue::String(Cow::Borrowed("one")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 1 },
        FormulaValue::String(Cow::Borrowed("oneone")),
      )
      .build();
    let partial_cell_book = FormulaEvaluationBookBuilder::new()
      .with_formula_match_whole_cell(false)
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 0 },
        FormulaValue::String(Cow::Borrowed("one")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 1 },
        FormulaValue::String(Cow::Borrowed("oneone")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 0 },
        FormulaValue::String(Cow::Borrowed("A2")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 1 },
        FormulaValue::String(Cow::Borrowed("2")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 2 },
        FormulaValue::Number(2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 3 },
        FormulaValue::Number(3.0),
      )
      .build();

    assert_eq!(
      whole_cell_book.evaluate_formula_text(SheetId(1), None, "COUNTIF(A1:A2,\"one\")"),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      partial_cell_book.evaluate_formula_text(SheetId(1), None, "COUNTIF(A1:A2,\"one\")"),
      Some(FormulaValue::Number(2.0))
    );
    assert_eq!(
      partial_cell_book.evaluate_formula_text(SheetId(1), None, "COUNTIF(B1:B4,\"=2\")"),
      Some(FormulaValue::Number(3.0))
    );
    assert_eq!(
      partial_cell_book.evaluate_formula_text(SheetId(1), None, "COUNTIF(B1:B4,\">2\")"),
      Some(FormulaValue::Number(1.0))
    );
  }

  #[test]
  fn evaluation_book_covariance_requires_matching_matrix_dimensions() {
    let book = FormulaEvaluationBookBuilder::new()
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 0 },
        FormulaValue::Number(1.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 1 },
        FormulaValue::Number(2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 0 },
        FormulaValue::Number(3.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 1 },
        FormulaValue::Number(4.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 2 },
        FormulaValue::Number(5.0),
      )
      .build();

    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "COVAR(A1:A2,B1:B2)"),
      Some(FormulaValue::Number(0.25))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "COVAR(A1:A2,B1:B3)"),
      Some(FormulaValue::Error(FormulaErrorValue::NA))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "COVAR(A1:A2,B1:B2,B1:B2)"),
      Some(FormulaValue::Error(FormulaErrorValue::Unknown))
    );
  }

  #[test]
  fn evaluation_book_chisq_right_tail_inverse_rejects_zero_probability() {
    let book = FormulaEvaluationBookBuilder::new().build();

    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "CHISQ.INV.RT(0,1)"),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "LEGACY.CHIINV(0,5)"),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
  }

  #[test]
  fn evaluation_book_numeric_unary_matches_libreoffice_non_finite_errors() {
    let book = FormulaEvaluationBookBuilder::new()
      .with_cell(
        SheetId(1),
        CellAddress { column: 9, row: 12 },
        FormulaValue::Number(1.4e154),
      )
      .build();

    for formula in [
      "COS(J13)",
      "COSH(J13)",
      "COT(0)/PI()",
      "COTH(0)/PI()",
      "CSC(0)/PI()",
      "CSCH(0)/PI()",
    ] {
      assert_eq!(
        book.evaluate_formula_text(SheetId(1), None, formula),
        Some(FormulaValue::Error(FormulaErrorValue::Value)),
        "{formula}"
      );
    }
  }

  #[test]
  fn evaluation_book_convert_ooo_uses_libreoffice_three_argument_arity() {
    let book = FormulaEvaluationBookBuilder::new().build();

    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "CONVERT_OOO(100,\"EUR\",\"SIT\")"),
      Some(FormulaValue::Number(23964.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "CONVERT_OOO(100,\"EUR\",\"SIT\",FALSE())"),
      Some(FormulaValue::Error(FormulaErrorValue::Unknown))
    );
  }

  #[test]
  fn evaluation_book_cumipmt_cumprinc_reject_missing_required_type() {
    let book = FormulaEvaluationBookBuilder::new().build();

    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "CUMIPMT(0.055/12,24,5000,4,6,)"),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "CUMPRINC(0.055/12,24,5000,4,6,)"),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
  }

  #[test]
  fn evaluation_book_dsum_parses_decimal_comma_criteria_like_libreoffice() {
    // Source: LibreOffice sc/qa/unit/data/functions/database/fods/dsum.fods,
    // Interest sheet uses criteria text such as ">,005" and "<=0,01".
    let book = FormulaEvaluationBookBuilder::new()
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 0 },
        FormulaValue::String(Cow::Borrowed("Interest Rate")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 0 },
        FormulaValue::String(Cow::Borrowed("Bal Now")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 1 },
        FormulaValue::Number(0.004),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 1 },
        FormulaValue::Number(29.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 2 },
        FormulaValue::Number(0.007),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 2 },
        FormulaValue::Number(14.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 3 },
        FormulaValue::Number(0.012),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 3 },
        FormulaValue::Number(10.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 3, row: 0 },
        FormulaValue::String(Cow::Borrowed("Interest Rate")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 4, row: 0 },
        FormulaValue::String(Cow::Borrowed("Interest Rate")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 3, row: 1 },
        FormulaValue::String(Cow::Borrowed(">,005")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 4, row: 1 },
        FormulaValue::String(Cow::Borrowed("<=0,01")),
      )
      .build();

    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DSUM(A1:B4,\"Bal Now\",D1:E2)"),
      Some(FormulaValue::Number(14.0))
    );
  }

  #[test]
  fn evaluation_book_countifs_parses_date_criteria_like_libreoffice() {
    let mut builder = FormulaEvaluationBookBuilder::new();
    for row in 0..6 {
      builder = builder
        .with_cell(
          SheetId(1),
          CellAddress {
            column: 31,
            row: row + 1,
          },
          FormulaValue::Number((row + 1) as f64),
        )
        .with_cell(
          SheetId(1),
          CellAddress {
            column: 32,
            row: row + 1,
          },
          FormulaValue::Number(
            valid_date_serial_with_system(2011, 5, (row + 1) as i32, DateSystem::Date1900).unwrap(),
          ),
        );
    }
    let book = builder.build();

    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        None,
        "COUNTIFS(AF2:AF7,\"<5\",AG2:AG7,\"<5/3/2011\")",
      ),
      Some(FormulaValue::Number(2.0))
    );
  }

  #[test]
  fn evaluation_book_date_functions_parse_strings_like_libreoffice() {
    let book = FormulaEvaluationBookBuilder::new()
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 0 },
        FormulaValue::Number(2000.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 1 },
        FormulaValue::Number(2001.0),
      )
      .build();
    let lo_book = FormulaEvaluationBookBuilder::new()
      .with_date_system(DateSystem::LibreOffice)
      .build();

    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DATE(,1,31)"),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DATE(1983,,31)"),
      Some(FormulaValue::Number(30316.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DATE(\"x\",1,1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      lo_book.evaluate_formula_text(SheetId(1), None, "DATE(0,12,31)"),
      Some(FormulaValue::Number(36891.0))
    );
    assert_eq!(
      lo_book.evaluate_formula_text(SheetId(1), None, "DATE(100,1,1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      lo_book.evaluate_formula_text(SheetId(1), None, "DATE(1582,10,15)"),
      Some(FormulaValue::Number(-115858.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DATE(1900,2,29)"),
      Some(FormulaValue::Number(60.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DATE(1900,2,30)"),
      Some(FormulaValue::Number(61.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DATE(104,1,1)"),
      Some(FormulaValue::Number(37987.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "YEAR(0)+MONTH(0)+DAY(0)"),
      Some(FormulaValue::Number(1901.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DAY(\"1899-12-29T15:26:14\")"),
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DAY(-1.1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    );
    assert_eq!(
      lo_book.evaluate_formula_text(SheetId(1), None, "DAY(\"1899-12-29T15:26:14\")"),
      Some(FormulaValue::Number(29.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DAYS(\"1990-01-01\",\"1980-10-10\")"),
      Some(FormulaValue::Number(3370.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DAYS360(534,1.00001)"),
      Some(FormulaValue::Number(-526.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DAYS360(\"1-FEB-2021\",\"15-MAR-2021\")"),
      Some(FormulaValue::Number(44.0))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        None,
        "DAYS360(\"2/28/1993\",\"3/1/1993\",FALSE)"
      ),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DAYS360(\"2/28/1993\",\"3/1/1993\",TRUE)"),
      Some(FormulaValue::Number(3.0))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        None,
        "DAYS360(\"2/28/1993\",\"3/1/1993\",\"FALSE\")"
      ),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        None,
        "DAYS360(\"2/28/1993\",\"3/1/1993\",\"x\")"
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DEGREES(PI())"),
      Some(FormulaValue::Number(180.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "RADIANS(180)"),
      Some(FormulaValue::Number(std::f64::consts::PI))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DEGREES(\"\")"),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "EDATE(\"2001-03-31\",-1)"),
      Some(FormulaValue::Number(
        valid_date_serial_with_system(2001, 2, 28, DateSystem::Date1900).unwrap()
      ))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "EDATE(\"2001-03-31\",1)"),
      Some(FormulaValue::Number(
        valid_date_serial_with_system(2001, 4, 30, DateSystem::Date1900).unwrap()
      ))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "EASTERSUNDAY(A1:A2)"),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "EASTERSUNDAY(-1)"),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ISLEAPYEAR(A1:A2)"),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ISOWEEKNUM(A1:A2)"),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
  }

  #[test]
  fn evaluation_book_financial_addins_match_libreoffice() {
    let book = FormulaEvaluationBookBuilder::new().build();

    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=DOLLARDE(1,1;8)",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(1.125))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=DOLLARFR(1,1;8)",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(1.08))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "EUROCONVERT(100,\"EUR\",\"SIT\",,)"),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "EUROCONVERT(100,\"EUR\",\"SIT\",,3)"),
      Some(FormulaValue::Number(23964.0))
    );
  }

  #[test]
  fn normalizes_formula_grammar_entry_points() {
    assert_eq!(
      r1c1_whole_axis_reference_to_a1("=C[10]", CellAddress { column: 1, row: 1 }),
      Some("L:L".to_string())
    );
    assert_eq!(
      r1c1_whole_axis_reference_to_a1("=R[3]", CellAddress { column: 1, row: 1 }),
      Some("5:5".to_string())
    );
    assert_eq!(
      normalize_formula_text("of:=SUM([.A1:.A2];3)", FormulaGrammar::OpenFormula),
      Cow::Borrowed("SUM(A1:A2,3)")
    );
    assert_eq!(
      normalize_formula_text(
        "of:=SUM([.B1:.B4]~[.D2:.D5]![.B2:.D2])",
        FormulaGrammar::OpenFormula
      ),
      Cow::Borrowed("SUM(B1:B4~D2:D5!B2:D2)")
    );
    assert_eq!(
      normalize_formula_text("of:=AND([Sheet2.C2:.C396])", FormulaGrammar::OpenFormula),
      Cow::Borrowed("AND(Sheet2!C2:C396)")
    );
    assert_eq!(
      normalize_formula_text("of:=DOLLARDE(1,1;8)", FormulaGrammar::OpenFormula),
      Cow::Borrowed("DOLLARDE(1.1,8)")
    );
  }

  #[test]
  fn open_formula_distinguishes_sheet_references_from_intersections() {
    let book = FormulaEvaluationBookBuilder::new()
      .with_sheet(SheetId(1), "Sheet1")
      .with_sheet(SheetId(2), "Sheet2")
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 0 },
        FormulaValue::Number(1.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 1 },
        FormulaValue::Number(2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 2 },
        FormulaValue::Number(3.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 3 },
        FormulaValue::Number(4.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 4 },
        FormulaValue::Number(5.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 3, row: 0 },
        FormulaValue::Number(10.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 3, row: 1 },
        FormulaValue::Number(20.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 3, row: 2 },
        FormulaValue::Number(30.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 3, row: 3 },
        FormulaValue::Number(40.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 3, row: 4 },
        FormulaValue::Number(50.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 4, row: 0 },
        FormulaValue::String(Cow::Borrowed("$D$5")),
      )
      .with_cell(
        SheetId(2),
        CellAddress { column: 2, row: 1 },
        FormulaValue::Boolean(true),
      )
      .with_cell(
        SheetId(2),
        CellAddress {
          column: 2,
          row: 395,
        },
        FormulaValue::Boolean(true),
      )
      .build();

    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=SUM([.B1:.B4]~[.D2:.D5]![.B2:.D2])",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(30.0))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=SUM([.B1:.B4]~[.D2:.D5]!([.B2:.D2]~[.B4:.D4]))",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(70.0))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=SUM([.B1:.B4]~[.D2:.D5]![.B2]:([.C2:.D2]~[.B4:.C4]):[.D4])",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(100.0))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=SUM((([.B1:.B4]~[.D2:.D5])!([.B2:.D2]~[.B4:.D4])):[.D5])",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(154.0))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=SUM((([.B1:.B4]~[.D2:.D5])!([.B2:.D2]~[.B4:.D4])):INDIRECT([.E1]))",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(154.0))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=SUM(([.B1:.B4]~[.D2:.D5])!([.B2:.D2])~([.B4:.D4]:INDIRECT([.E1])))",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(121.0))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=SUM([.B1:.B4]~[.D2:.D5]![.B2:.D2]~[.B4:.D4]:INDIRECT([.E1]))",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(129.0))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=AND([Sheet2.C2:.C396])",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Boolean(true))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=AREAS(([.C1:.C5]~[.B1:.B5]~[.D1:.D5]))",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(3.0))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=SUM(INDEX(([.C1:.C5]~[.B1:.B5]~[.D1:.D5]);0;0;1))",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(0.0))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=SUM(INDEX(([.C1:.C5]~[.B1:.B5]~[.D1:.D5]);0;0;2))",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(15.0))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=SUM(INDEX(([.C1:.C5]~[.B1:.B5]~[.D1:.D5]);0;0;3))",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Number(150.0))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=SUM(INDEX(([.C1:.C5]~[.B1:.B5]~[.D1:.D5]);0;0;0))",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=SUM(INDEX(([.C1:.C5]~[.B1:.B5]~[.D1:.D5]);0;0;4))",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Error(FormulaErrorValue::Ref))
    );
  }

  #[test]
  fn evaluation_book_matches_pdf_excel_2010_libreoffice_formula_surface() {
    let book = pdf_excel_2010_formula_book();
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
      ("ISO.CEILING(G6,F5)", 2.0),
      ("CHISQ.TEST(F2:F10,G2:G10)", 1.8744045912597986e-8),
      ("F.TEST(F2:F10,G2:G10)", 5.814996997636946e-8),
    ] {
      assert_formula_number_close(&book, SheetId(1), formula, expected);
    }
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        None,
        "ROUND(0.42284813280246891,12)=ROUND(0.42284813280246902,12)"
      ),
      Some(FormulaValue::Boolean(true))
    );
  }

  #[test]
  fn evaluation_book_evaluates_libreoffice_lookup_ifs_matrix_surface() {
    let book = FormulaEvaluationBookBuilder::new()
      .with_sheet(SheetId(1), "Sheet1")
      .with_sheet(SheetId(2), "Sheet2")
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 0 },
        FormulaValue::String(Cow::Borrowed("a")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 0 },
        FormulaValue::Number(1.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 1 },
        FormulaValue::String(Cow::Borrowed("b")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 1 },
        FormulaValue::Number(2.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 2 },
        FormulaValue::String(Cow::Borrowed("c")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 2 },
        FormulaValue::Number(4.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 3 },
        FormulaValue::String(Cow::Borrowed("a")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 3 },
        FormulaValue::Number(16.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 4 },
        FormulaValue::String(Cow::Borrowed("b")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 4 },
        FormulaValue::Number(32.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 0, row: 5 },
        FormulaValue::String(Cow::Borrowed("c")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 1, row: 5 },
        FormulaValue::Number(64.0),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 2, row: 0 },
        FormulaValue::String(Cow::Borrowed("x")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 3, row: 0 },
        FormulaValue::String(Cow::Borrowed("y")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 4, row: 0 },
        FormulaValue::String(Cow::Borrowed("z")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 2, row: 1 },
        FormulaValue::String(Cow::Borrowed("one")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 3, row: 1 },
        FormulaValue::String(Cow::Borrowed("two")),
      )
      .with_cell(
        SheetId(1),
        CellAddress { column: 4, row: 1 },
        FormulaValue::String(Cow::Borrowed("three")),
      )
      .build();

    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "SHEETS()"),
      Some(FormulaValue::Number(2.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "LOOKUP(\"b\",A1:A6,B1:B6)"),
      Some(FormulaValue::Number(32.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "LOOKUP(2,1/(C1:E1<>\"\"),C2:E2)"),
      Some(FormulaValue::String(Cow::Borrowed("three")))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "SUMIFS(B1:B6,A1:A6,\"a\")"),
      Some(FormulaValue::Number(17.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "COUNTIFS(A1:A6,\"?\",B1:B6,\">3\")"),
      Some(FormulaValue::Number(4.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "AVERAGEIFS(B1:B6,A1:A6,\"c\")"),
      Some(FormulaValue::Number(34.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "FILTER(A1:B6,A1:A6=\"a\",\"none\")"),
      Some(FormulaValue::Matrix(vec![
        vec![
          FormulaValue::String(Cow::Borrowed("a")),
          FormulaValue::Number(1.0),
        ],
        vec![
          FormulaValue::String(Cow::Borrowed("a")),
          FormulaValue::Number(16.0),
        ],
      ]))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=COM.MICROSOFT.SEQUENCE(3; ; ;3)",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Matrix(vec![
        vec![FormulaValue::Number(1.0)],
        vec![FormulaValue::Number(4.0)],
        vec![FormulaValue::Number(7.0)],
      ]))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=COM.MICROSOFT.SEQUENCE(;2;4;3)",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Matrix(vec![vec![
        FormulaValue::Number(4.0),
        FormulaValue::Number(7.0),
      ]]))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "TAKE(A1:B6,,1)"),
      Some(FormulaValue::Matrix(vec![
        vec![FormulaValue::String(Cow::Borrowed("a"))],
        vec![FormulaValue::String(Cow::Borrowed("b"))],
        vec![FormulaValue::String(Cow::Borrowed("c"))],
        vec![FormulaValue::String(Cow::Borrowed("a"))],
        vec![FormulaValue::String(Cow::Borrowed("b"))],
        vec![FormulaValue::String(Cow::Borrowed("c"))],
      ]))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DROP(A1:B6,30)"),
      Some(FormulaValue::Matrix(vec![
        vec![
          FormulaValue::String(Cow::Borrowed("a")),
          FormulaValue::Number(1.0),
        ],
        vec![
          FormulaValue::String(Cow::Borrowed("b")),
          FormulaValue::Number(2.0),
        ],
        vec![
          FormulaValue::String(Cow::Borrowed("c")),
          FormulaValue::Number(4.0),
        ],
        vec![
          FormulaValue::String(Cow::Borrowed("a")),
          FormulaValue::Number(16.0),
        ],
        vec![
          FormulaValue::String(Cow::Borrowed("b")),
          FormulaValue::Number(32.0),
        ],
        vec![
          FormulaValue::String(Cow::Borrowed("c")),
          FormulaValue::Number(64.0),
        ],
      ]))
    );
    assert_eq!(
      book.evaluate_formula_text_with_grammar(
        SheetId(1),
        None,
        "of:=COM.MICROSOFT.HSTACK({1}; ;{2})",
        FormulaGrammar::OpenFormula,
      ),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "MDETERM({1,2;3,4})"),
      Some(FormulaValue::Number(-2.0))
    );
    assert!(matches!(
      parse_formula_with_context(
        FormulaParseContext::default(),
        Cow::Borrowed("SUMIFS(B1:B6,A1:A6,\"a\")")
      )
      .code
      .as_ref()
      .and_then(|code| code.ops.last()),
      Some(FormulaOp::Call { .. })
    ));
  }

  fn pdf_excel_2010_formula_book() -> FormulaEvaluationBook<'static> {
    let mut cells = BTreeMap::new();
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
        (SheetId(1), CellAddress::parse_a1(reference).unwrap()),
        FormulaValue::Number(value),
      );
    }
    FormulaEvaluationBook {
      cells,
      ..FormulaEvaluationBook::default()
    }
  }

  fn assert_formula_number_close(
    book: &FormulaEvaluationBook<'_>,
    sheet: SheetId,
    formula: &str,
    expected: f64,
  ) {
    let Some(FormulaValue::Number(actual)) = book.evaluate_formula_text(sheet, None, formula)
    else {
      panic!("expected number for {formula}");
    };
    assert!(
      (actual - expected).abs() <= 1.0e-9,
      "expected {expected}, got {actual} for {formula}"
    );
  }

  fn assert_formula_matrix_numbers_close(
    book: &FormulaEvaluationBook<'_>,
    formula: &str,
    expected: &[&[f64]],
  ) {
    let Some(FormulaValue::Matrix(actual)) = book.evaluate_formula_text(SheetId(1), None, formula)
    else {
      panic!("expected matrix for {formula}");
    };
    assert_eq!(actual.len(), expected.len(), "row count for {formula}");
    for (row_index, (actual_row, expected_row)) in actual.iter().zip(expected.iter()).enumerate() {
      assert_eq!(
        actual_row.len(),
        expected_row.len(),
        "column count at row {row_index} for {formula}"
      );
      for (column_index, (actual_value, expected_value)) in
        actual_row.iter().zip(expected_row.iter()).enumerate()
      {
        let FormulaValue::Number(actual_number) = actual_value else {
          panic!("expected number at row {row_index}, column {column_index} for {formula}");
        };
        assert!(
          (actual_number - expected_value).abs() <= 1.0e-9,
          "expected {expected_value}, got {actual_number} at row {row_index}, column {column_index} for {formula}"
        );
      }
    }
  }
}
