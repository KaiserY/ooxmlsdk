use std::borrow::Cow;
use std::collections::BTreeMap;

use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::workbook_part::WorkbookPart;
use ooxmlsdk::schemas::x;
use ooxmlsdk::sdk::SdkPart;

use crate::{
  AddressFlags, CellAddress, CellRange, DisplayValue, FormulaError, FormulaErrorValue,
  FormulaValue, QualifiedAddress, QualifiedRange, Result, SheetId,
};

const MAX_FORMULA_RECALC_PASSES: usize = 12;
const MAX_EXPANDED_RANGE_CELLS: u64 = 20_000;
const XLSX_MAX_COLUMN_ZERO_BASED: u32 = 16_383;
const XLSX_MAX_ROW_ZERO_BASED: u32 = 1_048_575;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorkbookValueModel<'doc> {
  pub identity: WorkbookIdentity<'doc>,
  pub sheets: Vec<WorksheetValueModel<'doc>>,
  pub defined_names: Vec<DefinedName<'doc>>,
  pub shared_formula_groups: Vec<SharedFormulaGroup<'doc>>,
  pub array_formula_groups: Vec<ArrayFormulaGroup<'doc>>,
  pub data_tables: Vec<DataTableFormula<'doc>>,
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
    let worksheet_parts = workbook_part.worksheet_parts(document).collect::<Vec<_>>();

    let identity = workbook_identity(&workbook).into_owned();
    let mut sheets = identity
      .sheets
      .iter()
      .map(|identity| {
        let worksheet = worksheet_parts
          .iter()
          .find(|part| part.relationship_id() == identity.relationship_id.as_deref())
          .and_then(|part| part.root_element(document).ok())
          .cloned();
        worksheet_value_model(identity, worksheet.as_ref(), &shared_strings)
          .map(WorksheetValueModel::into_owned)
      })
      .collect::<Result<Vec<_>>>()?;
    resolve_shared_formula_dependents(&mut sheets);
    mark_formula_recalc_state(&mut sheets);
    let defined_names: Vec<DefinedName<'doc>> = defined_names(&workbook)
      .into_iter()
      .map(DefinedName::into_owned)
      .collect();
    let shared_formula_groups = shared_formula_groups(&sheets);
    let array_formula_groups = array_formula_groups(&sheets);
    let data_tables = data_tables(&sheets);
    let dependency_graph = dependency_graph(&sheets, &defined_names);

    Ok(Self {
      calculation_settings: calculation_settings(&workbook),
      calc_chain: calc_chain(document, &workbook_part)?,
      external_references: external_references(&workbook)
        .into_iter()
        .map(ExternalReference::into_owned)
        .collect(),
      external_cached_cells: external_cached_cells(document, &workbook_part, &workbook)?
        .into_iter()
        .map(ExternalCachedCell::into_owned)
        .collect(),
      defined_names,
      shared_formula_groups,
      array_formula_groups,
      data_tables,
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
          let Some(ast) = parsed.ast.as_ref() else {
            if pass == 0 {
              unsupported.extend(parsed.unsupported.clone());
            }
            continue;
          };
          let context = FormulaEvaluator {
            book: &book,
            current_sheet: sheet_id,
            current_cell: Some(address),
            locals: BTreeMap::new(),
          };
          match context.evaluate(ast) {
            Some(value)
              if formula.reference.is_some() || !matches!(value, FormulaValue::Reference(_)) =>
            {
              let value = value.into_owned();
              if let Some(range) = formula.reference
                && let Some(items) = array_formula_result_items(&context, sheet_id, range, &value)
              {
                candidates.extend(items);
              } else {
                candidates.push(EvaluatedFormula {
                  sheet: sheet_id,
                  cell: formula.address,
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
    if !self.calc_chain.is_empty() {
      return self
        .calc_chain
        .iter()
        .filter_map(|entry| entry.sheet.map(|sheet| (sheet, entry.cell)))
        .collect();
    }
    self
      .sheets
      .iter()
      .flat_map(|sheet| {
        sheet
          .cells
          .iter()
          .filter(|(_, record)| record.formula.is_some())
          .map(move |(address, _)| (sheet.id, *address))
      })
      .collect()
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
    let old_value = record
      .formula
      .as_ref()
      .and_then(|formula| formula.evaluated_value.clone())
      .unwrap_or_else(|| record.raw_value.clone());
    if old_value == value {
      return false;
    }
    let number_format_id = record
      .formula
      .as_ref()
      .and_then(|formula| formula.number_format_context.as_ref())
      .and_then(|context| context.format_id);
    if let Some(formula) = record.formula.as_mut() {
      formula.evaluated_value = Some(value.clone());
      formula.formula_state = FormulaState::Clean;
    } else {
      record.raw_value = value.clone();
    }
    record.display_value = Some(DisplayValue {
      text: Cow::Owned(display_text_from_value(&value)),
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
  pub tokens: Vec<FormulaToken<'doc>>,
  pub ast: Option<FormulaAst<'doc>>,
  pub dependencies: Vec<FormulaDependency<'doc>>,
  pub unsupported: Vec<UnsupportedFormulaFeature<'doc>>,
}

impl<'doc> ParsedFormula<'doc> {
  fn into_owned(self) -> ParsedFormula<'static> {
    ParsedFormula {
      source: Cow::Owned(self.source.into_owned()),
      tokens: self
        .tokens
        .into_iter()
        .map(FormulaToken::into_owned)
        .collect(),
      ast: self.ast.map(FormulaAst::into_owned),
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
pub enum FormulaAst<'doc> {
  Literal(FormulaValue<'doc>),
  Reference(QualifiedRange<'doc>),
  ExternalReference(ExternalReferenceId<'doc>),
  Name(Cow<'doc, str>),
  Unary {
    op: FormulaOperator,
    expr: Box<FormulaAst<'doc>>,
  },
  Binary {
    op: FormulaOperator,
    left: Box<FormulaAst<'doc>>,
    right: Box<FormulaAst<'doc>>,
  },
  Function {
    name: Cow<'doc, str>,
    args: Vec<FormulaAst<'doc>>,
  },
  Array(Vec<Vec<FormulaAst<'doc>>>),
}

impl<'doc> FormulaAst<'doc> {
  fn into_owned(self) -> FormulaAst<'static> {
    match self {
      FormulaAst::Literal(value) => FormulaAst::Literal(value.into_owned()),
      FormulaAst::Reference(value) => FormulaAst::Reference(value.into_owned()),
      FormulaAst::ExternalReference(value) => FormulaAst::ExternalReference(value.into_owned()),
      FormulaAst::Name(value) => FormulaAst::Name(Cow::Owned(value.into_owned())),
      FormulaAst::Unary { op, expr } => FormulaAst::Unary {
        op,
        expr: Box::new(expr.into_owned()),
      },
      FormulaAst::Binary { op, left, right } => FormulaAst::Binary {
        op,
        left: Box::new(left.into_owned()),
        right: Box::new(right.into_owned()),
      },
      FormulaAst::Function { name, args } => FormulaAst::Function {
        name: Cow::Owned(name.into_owned()),
        args: args.into_iter().map(FormulaAst::into_owned).collect(),
      },
      FormulaAst::Array(rows) => FormulaAst::Array(
        rows
          .into_iter()
          .map(|row| row.into_iter().map(FormulaAst::into_owned).collect())
          .collect(),
      ),
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
pub enum FormulaDependency<'doc> {
  Cell {
    sheet: SheetId,
    address: CellAddress,
  },
  Range(QualifiedRange<'doc>),
  Name(Cow<'doc, str>),
  External(ExternalReferenceId<'doc>),
  Volatile,
}

impl<'doc> FormulaDependency<'doc> {
  fn into_owned(self) -> FormulaDependency<'static> {
    match self {
      FormulaDependency::Cell { sheet, address } => FormulaDependency::Cell { sheet, address },
      FormulaDependency::Range(value) => FormulaDependency::Range(value.into_owned()),
      FormulaDependency::Name(value) => FormulaDependency::Name(Cow::Owned(value.into_owned())),
      FormulaDependency::External(value) => FormulaDependency::External(value.into_owned()),
      FormulaDependency::Volatile => FormulaDependency::Volatile,
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExternalReferenceId<'doc> {
  pub book: Option<Cow<'doc, str>>,
  pub sheet: Option<Cow<'doc, str>>,
  pub name: Option<Cow<'doc, str>>,
}

impl<'doc> ExternalReferenceId<'doc> {
  fn into_owned(self) -> ExternalReferenceId<'static> {
    ExternalReferenceId {
      book: self.book.map(|value| Cow::Owned(value.into_owned())),
      sheet: self.sheet.map(|value| Cow::Owned(value.into_owned())),
      name: self.name.map(|value| Cow::Owned(value.into_owned())),
    }
  }
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FormulaEvaluationBook<'doc> {
  pub source_file_name: Option<Cow<'doc, str>>,
  pub sheet_names: Vec<SheetBinding<'doc>>,
  pub cells: BTreeMap<(SheetId, CellAddress), FormulaValue<'doc>>,
  pub formulas: BTreeMap<(SheetId, CellAddress), FormulaText<'doc>>,
  pub defined_names: BTreeMap<DefinedNameKey, Cow<'doc, str>>,
  pub defined_arrays: BTreeMap<DefinedNameKey, Vec<Vec<FormulaValue<'doc>>>>,
  pub external_cached_cells: BTreeMap<(usize, String, CellAddress), FormulaValue<'doc>>,
  pub row_states: BTreeMap<(SheetId, u32), FormulaRowState>,
  pub tables: BTreeMap<String, FormulaTable<'doc>>,
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
      ..Self::default()
    }
  }

  pub fn sheet_id_by_name(&self, name: &str) -> Option<SheetId> {
    let clean = name.trim_matches('\'').trim();
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

  pub fn evaluate_formula_text(
    &self,
    current_sheet: SheetId,
    current_cell: Option<CellAddress>,
    formula: &str,
  ) -> Option<FormulaValue<'doc>> {
    if let Some(value) = self.evaluate_special_formula_text(current_sheet, current_cell, formula) {
      return Some(value);
    }
    let (ast, unsupported) = parse_formula_ast(current_sheet, formula);
    if !unsupported.is_empty() {
      return None;
    }
    FormulaEvaluator {
      book: self,
      current_sheet,
      current_cell,
      locals: BTreeMap::new(),
    }
    .evaluate(ast.as_ref()?)
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
        FormulaEvaluator {
          book: self,
          current_sheet,
          current_cell: Some(address),
          locals: BTreeMap::new(),
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
    FormulaEvaluator {
      book: self,
      current_sheet,
      current_cell: Some(address),
      locals: BTreeMap::new(),
    }
    .number(&value)
  }

  fn evaluate_special_formula_text(
    &self,
    current_sheet: SheetId,
    current_cell: Option<CellAddress>,
    formula: &str,
  ) -> Option<FormulaValue<'doc>> {
    let clean = formula.trim().strip_prefix('=').unwrap_or(formula.trim());
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
    if is_formula_error_literal(clean) {
      return Some(FormulaValue::Error(error_value(clean)));
    }
    if let Some((left, right)) = split_indirect_intersection(clean) {
      let left = self.evaluate_formula_text(current_sheet, current_cell, left)?;
      let right = self.evaluate_formula_text(current_sheet, current_cell, right)?;
      return Some(range_intersection_value(self, left, right));
    }
    None
  }

  pub fn formula_text(&self, sheet: SheetId, address: CellAddress) -> Option<String> {
    let formula = self.formulas.get(&(sheet, address))?;
    Some(if formula.kind == FormulaKind::Array {
      format!("{{={}}}", formula.text)
    } else {
      format!("={}", formula.text)
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
      let text = formula
        .text
        .trim_start()
        .trim_start_matches("_xlfn.")
        .to_ascii_uppercase();
      text.starts_with("SUBTOTAL(") || text.starts_with("AGGREGATE(")
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DependencyGraph<'doc> {
  pub nodes: Vec<DependencyNode>,
  pub edges: Vec<DependencyEdge<'doc>>,
  pub defined_name_nodes: Vec<DefinedNameNode<'doc>>,
  pub defined_name_edges: Vec<DefinedNameDependencyEdge<'doc>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct DependencyNode {
  pub sheet: SheetId,
  pub cell: CellAddress,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DependencyEdge<'doc> {
  pub from: DependencyNode,
  pub to: FormulaDependency<'doc>,
  pub volatile: bool,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DefinedNameNode<'doc> {
  pub sheet: Option<SheetId>,
  pub name: Cow<'doc, str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefinedNameDependencyEdge<'doc> {
  pub from: DefinedNameNode<'doc>,
  pub to: FormulaDependency<'doc>,
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
          cell_value_record(identity.id, address, cell, shared_strings)?,
        );
      }
    }
  }

  Ok(WorksheetValueModel {
    id: identity.id,
    name: identity.name.clone(),
    cells,
  })
}

fn cell_value_record<'doc>(
  sheet: SheetId,
  address: CellAddress,
  cell: &'doc x::Cell,
  shared_strings: &[String],
) -> Result<CellValueRecord<'doc>> {
  let raw_value = cell_value(cell, shared_strings);
  let dirty = cell.cell_formula.as_ref().is_some_and(|formula| {
    formula.calculate_cell.is_some_and(|value| value.as_bool())
      || formula
        .always_calculate_array
        .is_some_and(|value| value.as_bool())
  });
  let formula = cell.cell_formula.as_ref().map(|formula| {
    let formula_text: Cow<'doc, str> = formula
      .xml_content
      .as_deref()
      .map(Cow::Borrowed)
      .unwrap_or(Cow::Borrowed(""));
    let parsed_formula = parse_formula(sheet, formula_text.clone());
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
      number_format_context: cell.style_index.map(|index| NumberFormatContext {
        format_id: Some(index),
        format_code: None,
        locale: None,
      }),
      dirty,
      volatile,
    }
  });
  let display_value = Some(DisplayValue {
    text: Cow::Owned(cell_display_text(cell, shared_strings)),
    source_value: raw_value.clone(),
    number_format_id: cell.style_index,
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
    x::CellValues::String => value
      .map(|value| FormulaValue::String(Cow::Borrowed(value)))
      .unwrap_or_default(),
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
    for record in sheet.cells.values_mut() {
      let Some(formula) = record.formula.as_mut() else {
        continue;
      };
      let FormulaKind::SharedDependent { group_index } = formula.formula_kind else {
        continue;
      };
      let Some((origin, source)) = definitions.get(&(sheet.id, group_index)) else {
        continue;
      };
      let translated = translate_shared_formula_text(source, *origin, formula.address);
      formula.formula_text = Cow::Owned(translated);
      formula.parsed_formula = Some(parse_formula(sheet.id, formula.formula_text.clone()));
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
  let delta_col = i64::from(target.column) - i64::from(origin.column);
  let delta_row = i64::from(target.row) - i64::from(origin.row);
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
        translate_one_based_index(self.col, delta_col)
      },
      absolute_row: self.absolute_row,
      row: if self.absolute_row {
        self.row
      } else {
        translate_one_based_index(self.row, delta_row)
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

fn translate_one_based_index(index: u32, delta: i64) -> u32 {
  u32::try_from((i64::from(index) + delta).max(1)).unwrap_or(u32::MAX)
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
      groups.push(SharedFormulaGroup {
        index: group_index,
        sheet: sheet.id,
        origin: *address,
        range: formula.reference,
        formula_text: formula.formula_text.clone(),
        dependents: sheet
          .cells
          .iter()
          .filter_map(|(dependent_address, dependent_record)| {
            dependent_record
              .formula
              .as_ref()
              .and_then(|dependent_formula| match dependent_formula.formula_kind {
                FormulaKind::SharedDependent {
                  group_index: dependent_index,
                } if dependent_index == group_index => Some(*dependent_address),
                _ => None,
              })
          })
          .collect(),
      });
    }
  }
  groups
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

fn dependency_graph<'doc>(
  sheets: &[WorksheetValueModel<'doc>],
  defined_names: &[DefinedName<'doc>],
) -> DependencyGraph<'doc> {
  let mut graph = DependencyGraph::default();
  for sheet in sheets {
    for (address, record) in &sheet.cells {
      let Some(formula) = &record.formula else {
        continue;
      };
      let node = DependencyNode {
        sheet: sheet.id,
        cell: *address,
      };
      graph.nodes.push(node);
      let dependencies = formula
        .parsed_formula
        .as_ref()
        .map(|parsed| parsed.dependencies.clone())
        .unwrap_or_else(|| formula_dependencies(sheet.id, &formula.formula_text));
      for dependency in dependencies {
        graph.edges.push(DependencyEdge {
          from: node,
          to: dependency,
          volatile: formula.volatile,
        });
      }
    }
  }
  for defined_name in defined_names {
    let node = DefinedNameNode {
      sheet: defined_name.sheet,
      name: defined_name.name.clone(),
    };
    graph.defined_name_nodes.push(node.clone());
    let dependencies = defined_name
      .parsed_formula
      .as_ref()
      .map(|parsed| parsed.dependencies.clone())
      .unwrap_or_else(|| defined_name.dependencies.clone());
    for dependency in dependencies {
      graph.defined_name_edges.push(DefinedNameDependencyEdge {
        from: node.clone(),
        to: dependency,
      });
    }
  }
  graph
}

fn formula_dependencies<'doc>(
  sheet: SheetId,
  formula_text: &Cow<'doc, str>,
) -> Vec<FormulaDependency<'doc>> {
  parse_formula(sheet, Cow::Owned(formula_text.to_string())).dependencies
}

fn parse_formula<'doc>(sheet: SheetId, source: Cow<'doc, str>) -> ParsedFormula<'doc> {
  let mut tokens = Vec::new();
  let mut dependencies = Vec::new();
  let mut unsupported = Vec::new();
  let text = source.as_ref().strip_prefix('=').unwrap_or(source.as_ref());
  let mut index = 0usize;

  while index < text.len() {
    let Some(ch) = text[index..].chars().next() else {
      break;
    };
    if ch.is_whitespace() {
      index += ch.len_utf8();
      continue;
    }
    if ch == '"' {
      let (value, next) = parse_formula_string(text, index);
      tokens.push(FormulaToken::Literal(FormulaValue::String(Cow::Owned(
        value,
      ))));
      index = next;
      continue;
    }
    if ch.is_ascii_digit()
      || (ch == '.'
        && text[index + ch.len_utf8()..].starts_with(|next: char| next.is_ascii_digit()))
    {
      let (value, next) = parse_formula_number(text, index);
      tokens.push(FormulaToken::Literal(FormulaValue::Number(value)));
      index = next;
      continue;
    }
    if let Some((error, next)) = parse_formula_error_literal_at(text, index) {
      tokens.push(FormulaToken::Literal(FormulaValue::Error(error_value(
        error,
      ))));
      index = next;
      continue;
    }
    if let Some((operator, next)) = parse_formula_operator(text, index) {
      tokens.push(FormulaToken::Operator(operator));
      index = next;
      continue;
    }

    match ch {
      '{' => {
        tokens.push(FormulaToken::ArrayOpen);
        index += ch.len_utf8();
      }
      '}' => {
        tokens.push(FormulaToken::ArrayClose);
        index += ch.len_utf8();
      }
      ',' => {
        tokens.push(FormulaToken::Separator(FormulaSeparator::Argument));
        index += ch.len_utf8();
      }
      ';' => {
        tokens.push(FormulaToken::Separator(FormulaSeparator::Row));
        index += ch.len_utf8();
      }
      '(' | ')' => {
        index += ch.len_utf8();
      }
      _ if is_formula_word_char(ch) => {
        let (word, next) = parse_formula_word(text, index);
        let next_non_space = text[next..].chars().find(|ch| !ch.is_whitespace());
        if next_non_space == Some('(') && QualifiedAddress::parse_a1(sheet, word).is_err() {
          if is_volatile_function(word) {
            dependencies.push(FormulaDependency::Volatile);
          }
          tokens.push(FormulaToken::Function(Cow::Owned(word.to_string())));
        } else if let Some(external) = parse_external_reference_id(word) {
          dependencies.push(FormulaDependency::External(external.clone()));
          tokens.push(FormulaToken::ExternalReference(external));
        } else if let Some(range) = parse_formula_range(sheet, word) {
          dependencies.push(dependency_from_range(sheet, &range));
          tokens.push(FormulaToken::Reference(range));
        } else if is_formula_error_literal(word) {
          tokens.push(FormulaToken::Literal(FormulaValue::Error(error_value(
            word,
          ))));
        } else {
          dependencies.push(FormulaDependency::Name(Cow::Owned(word.to_string())));
          tokens.push(FormulaToken::Name(Cow::Owned(word.to_string())));
        }
        index = next;
      }
      _ => {
        let feature = ch.to_string();
        unsupported.push(UnsupportedFormulaFeature {
          feature: Cow::Owned(feature.clone()),
          reason: Cow::Borrowed("unrecognized formula character"),
        });
        tokens.push(FormulaToken::Unsupported(Cow::Owned(feature)));
        index += ch.len_utf8();
      }
    }
  }

  let (ast, ast_unsupported) = parse_formula_ast(sheet, text);
  unsupported.extend(ast_unsupported);

  ParsedFormula {
    source,
    tokens,
    ast,
    dependencies,
    unsupported,
  }
}

fn parse_formula_ast<'doc>(
  sheet: SheetId,
  text: &str,
) -> (
  Option<FormulaAst<'doc>>,
  Vec<UnsupportedFormulaFeature<'doc>>,
) {
  let mut parser = FormulaAstParser::new(sheet, text);
  let ast = parser.parse_expression();
  parser.skip_ws();
  if ast.is_some() && parser.is_end() {
    (ast, parser.unsupported)
  } else {
    parser.unsupported.push(UnsupportedFormulaFeature {
      feature: Cow::Owned(text.to_string()),
      reason: Cow::Borrowed("formula expression is not fully parsed"),
    });
    (None, parser.unsupported)
  }
}

struct FormulaAstParser<'a, 'doc> {
  sheet: SheetId,
  text: &'a str,
  index: usize,
  unsupported: Vec<UnsupportedFormulaFeature<'doc>>,
}

impl<'a, 'doc> FormulaAstParser<'a, 'doc> {
  fn new(sheet: SheetId, text: &'a str) -> Self {
    Self {
      sheet,
      text,
      index: 0,
      unsupported: Vec::new(),
    }
  }

  fn parse_expression(&mut self) -> Option<FormulaAst<'doc>> {
    self.parse_comparison()
  }

  fn parse_comparison(&mut self) -> Option<FormulaAst<'doc>> {
    let mut left = self.parse_concat()?;
    loop {
      self.skip_ws();
      let Some(op) = self.consume_comparison_operator() else {
        break;
      };
      let right = self.parse_concat()?;
      left = FormulaAst::Binary {
        op,
        left: Box::new(left),
        right: Box::new(right),
      };
    }
    Some(left)
  }

  fn parse_concat(&mut self) -> Option<FormulaAst<'doc>> {
    let mut left = self.parse_add_sub()?;
    loop {
      self.skip_ws();
      if !self.consume_char('&') {
        break;
      }
      let right = self.parse_add_sub()?;
      left = FormulaAst::Binary {
        op: FormulaOperator::Concat,
        left: Box::new(left),
        right: Box::new(right),
      };
    }
    Some(left)
  }

  fn parse_add_sub(&mut self) -> Option<FormulaAst<'doc>> {
    let mut left = self.parse_mul_div()?;
    loop {
      self.skip_ws();
      let op = if self.consume_char('+') {
        FormulaOperator::Add
      } else if self.consume_char('-') {
        FormulaOperator::Subtract
      } else {
        break;
      };
      let right = self.parse_mul_div()?;
      left = FormulaAst::Binary {
        op,
        left: Box::new(left),
        right: Box::new(right),
      };
    }
    Some(left)
  }

  fn parse_mul_div(&mut self) -> Option<FormulaAst<'doc>> {
    let mut left = self.parse_power()?;
    loop {
      self.skip_ws();
      let op = if self.consume_char('*') {
        FormulaOperator::Multiply
      } else if self.consume_char('/') {
        FormulaOperator::Divide
      } else {
        break;
      };
      let right = self.parse_power()?;
      left = FormulaAst::Binary {
        op,
        left: Box::new(left),
        right: Box::new(right),
      };
    }
    Some(left)
  }

  fn parse_power(&mut self) -> Option<FormulaAst<'doc>> {
    let left = self.parse_unary()?;
    self.skip_ws();
    if self.consume_char('^') {
      let right = self.parse_power()?;
      Some(FormulaAst::Binary {
        op: FormulaOperator::Power,
        left: Box::new(left),
        right: Box::new(right),
      })
    } else {
      Some(left)
    }
  }

  fn parse_unary(&mut self) -> Option<FormulaAst<'doc>> {
    self.skip_ws();
    if self.consume_char('+') {
      return Some(FormulaAst::Unary {
        op: FormulaOperator::UnaryPlus,
        expr: Box::new(self.parse_unary()?),
      });
    }
    if self.consume_char('-') {
      return Some(FormulaAst::Unary {
        op: FormulaOperator::UnaryMinus,
        expr: Box::new(self.parse_unary()?),
      });
    }
    self.parse_percent()
  }

  fn parse_percent(&mut self) -> Option<FormulaAst<'doc>> {
    let mut expr = self.parse_primary()?;
    loop {
      self.skip_ws();
      if !self.consume_char('%') {
        break;
      }
      expr = FormulaAst::Unary {
        op: FormulaOperator::Percent,
        expr: Box::new(expr),
      };
    }
    Some(expr)
  }

  fn parse_primary(&mut self) -> Option<FormulaAst<'doc>> {
    self.skip_ws();
    if self.consume_char('(') {
      let expr = self.parse_expression()?;
      self.skip_ws();
      if !self.consume_char(')') {
        self.unsupported.push(UnsupportedFormulaFeature {
          feature: Cow::Borrowed("parenthesized expression"),
          reason: Cow::Borrowed("missing closing parenthesis"),
        });
      }
      return Some(expr);
    }
    if self.peek_char() == Some('"') {
      let (value, next) = parse_formula_string(self.text, self.index);
      self.index = next;
      return Some(FormulaAst::Literal(FormulaValue::String(Cow::Owned(value))));
    }
    if self.peek_char() == Some('{') {
      return self.parse_array();
    }
    if self.starts_number() {
      let (value, next) = parse_formula_number(self.text, self.index);
      self.index = next;
      return Some(FormulaAst::Literal(FormulaValue::Number(value)));
    }
    if let Some((error, next)) = parse_formula_error_literal_at(self.text, self.index) {
      self.index = next;
      return Some(FormulaAst::Literal(FormulaValue::Error(error_value(error))));
    }
    self.parse_identifier_reference_or_function()
  }

  fn parse_array(&mut self) -> Option<FormulaAst<'doc>> {
    self.consume_char('{');
    let mut rows = Vec::new();
    let mut row = Vec::new();
    loop {
      self.skip_ws();
      if self.consume_char('}') {
        break;
      }
      row.push(self.parse_expression()?);
      self.skip_ws();
      if self.consume_char(',') {
        continue;
      }
      if self.consume_char(';') {
        rows.push(row);
        row = Vec::new();
        continue;
      }
      if self.consume_char('}') {
        break;
      }
      return None;
    }
    if !row.is_empty() {
      rows.push(row);
    }
    Some(FormulaAst::Array(rows))
  }

  fn parse_identifier_reference_or_function(&mut self) -> Option<FormulaAst<'doc>> {
    let start = self.index;
    let (_, next) = parse_formula_word(self.text, self.index);
    if next == start {
      return None;
    }
    let word = &self.text[start..next];
    self.index = next;
    self.skip_ws();
    if self.peek_char() == Some('(') && QualifiedAddress::parse_a1(self.sheet, word).is_err() {
      self.consume_char('(');
      let mut args = Vec::new();
      loop {
        self.skip_ws();
        if self.consume_char(')') {
          break;
        }
        if self.consume_char(',') {
          args.push(FormulaAst::Literal(FormulaValue::Blank));
          continue;
        }
        args.push(self.parse_expression()?);
        self.skip_ws();
        if self.consume_char(')') {
          break;
        }
        if !self.consume_char(',') {
          return None;
        }
      }
      return Some(FormulaAst::Function {
        name: Cow::Owned(word.to_string()),
        args,
      });
    }
    if let Some(external) = parse_external_reference_id(word) {
      return Some(FormulaAst::ExternalReference(external));
    }
    if let Some(range) = parse_formula_range(self.sheet, word) {
      return Some(FormulaAst::Reference(range));
    }
    if is_formula_error_literal(word) {
      return Some(FormulaAst::Literal(FormulaValue::Error(error_value(word))));
    }
    if word.eq_ignore_ascii_case("TRUE") {
      return Some(FormulaAst::Literal(FormulaValue::Boolean(true)));
    }
    if word.eq_ignore_ascii_case("FALSE") {
      return Some(FormulaAst::Literal(FormulaValue::Boolean(false)));
    }
    Some(FormulaAst::Name(Cow::Owned(word.to_string())))
  }

  fn consume_comparison_operator(&mut self) -> Option<FormulaOperator> {
    if self.consume_str("<>") {
      Some(FormulaOperator::NotEqual)
    } else if self.consume_str("<=") {
      Some(FormulaOperator::LessOrEqual)
    } else if self.consume_str(">=") {
      Some(FormulaOperator::GreaterOrEqual)
    } else if self.consume_char('=') {
      Some(FormulaOperator::Equal)
    } else if self.consume_char('<') {
      Some(FormulaOperator::Less)
    } else if self.consume_char('>') {
      Some(FormulaOperator::Greater)
    } else {
      None
    }
  }

  fn skip_ws(&mut self) {
    while self.peek_char().is_some_and(char::is_whitespace) {
      self.index += self.peek_char().map(char::len_utf8).unwrap_or_default();
    }
  }

  fn is_end(&self) -> bool {
    self.index >= self.text.len()
  }

  fn starts_number(&self) -> bool {
    let mut chars = self.text[self.index..].chars();
    match chars.next() {
      Some(ch) if ch.is_ascii_digit() => true,
      Some('.') => chars.next().is_some_and(|ch| ch.is_ascii_digit()),
      _ => false,
    }
  }

  fn peek_char(&self) -> Option<char> {
    self.text[self.index..].chars().next()
  }

  fn consume_char(&mut self, expected: char) -> bool {
    if self.peek_char() == Some(expected) {
      self.index += expected.len_utf8();
      true
    } else {
      false
    }
  }

  fn consume_str(&mut self, expected: &str) -> bool {
    if self.text[self.index..].starts_with(expected) {
      self.index += expected.len();
      true
    } else {
      false
    }
  }
}

struct FormulaEvaluator<'a, 'doc> {
  book: &'a FormulaEvaluationBook<'doc>,
  current_sheet: SheetId,
  current_cell: Option<CellAddress>,
  locals: BTreeMap<String, FormulaValue<'doc>>,
}

impl<'a, 'doc> FormulaEvaluator<'a, 'doc> {
  fn evaluate(&self, ast: &FormulaAst<'doc>) -> Option<FormulaValue<'doc>> {
    match ast {
      FormulaAst::Literal(value) => Some(value.clone()),
      FormulaAst::Reference(range) => Some(FormulaValue::Reference(range.clone())),
      FormulaAst::ExternalReference(reference) => self.evaluate_external_reference(reference),
      FormulaAst::Name(name) => self.evaluate_name(name),
      FormulaAst::Unary { op, expr } => self.evaluate_unary(*op, expr),
      FormulaAst::Binary { op, left, right } => self.evaluate_binary(*op, left, right),
      FormulaAst::Function { name, args } => self.evaluate_function(name, args),
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

  fn evaluate_unary(
    &self,
    op: FormulaOperator,
    expr: &FormulaAst<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let value = self.evaluate(expr)?;
    match op {
      FormulaOperator::UnaryPlus => Some(FormulaValue::Number(self.number(&value)?)),
      FormulaOperator::UnaryMinus => Some(FormulaValue::Number(-self.number(&value)?)),
      FormulaOperator::Percent => Some(FormulaValue::Number(self.number(&value)? / 100.0)),
      _ => None,
    }
  }

  fn evaluate_name(&self, name: &Cow<'doc, str>) -> Option<FormulaValue<'doc>> {
    let local_key = name.trim_start_matches("_xlpm.").to_ascii_uppercase();
    if let Some(value) = self.locals.get(&local_key) {
      return Some(value.clone());
    }
    if let Some(range) = parse_table_reference(self.book, name.as_ref(), self.current_cell) {
      return Some(FormulaValue::Reference(range));
    }
    if let Some(array) = self
      .book
      .defined_name_array(Some(self.current_sheet), name.as_ref())
    {
      return Some(FormulaValue::Matrix(array.clone()));
    }
    let formula = self
      .book
      .defined_name_formula(Some(self.current_sheet), name.as_ref())?;
    let (ast, unsupported) = parse_formula_ast(self.current_sheet, formula.as_ref());
    if !unsupported.is_empty() {
      return None;
    }
    self.evaluate(ast.as_ref()?)
  }

  fn evaluate_external_reference(
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

  fn evaluate_binary(
    &self,
    op: FormulaOperator,
    left: &FormulaAst<'doc>,
    right: &FormulaAst<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let left = self.evaluate(left)?;
    let right = self.evaluate(right)?;
    match op {
      FormulaOperator::Add => self.numeric_binary(left, right, |a, b| a + b),
      FormulaOperator::Subtract => self.numeric_binary(left, right, |a, b| a - b),
      FormulaOperator::Multiply => self.numeric_binary(left, right, |a, b| a * b),
      FormulaOperator::Divide => {
        let denominator = self.number(&right)?;
        if denominator == 0.0 {
          Some(FormulaValue::Error(FormulaErrorValue::Div0))
        } else {
          Some(FormulaValue::Number(self.number(&left)? / denominator))
        }
      }
      FormulaOperator::Power => self.numeric_binary(left, right, f64::powf),
      FormulaOperator::Concat => Some(FormulaValue::String(Cow::Owned(format!(
        "{}{}",
        self.text(&left),
        self.text(&right)
      )))),
      FormulaOperator::Equal
      | FormulaOperator::NotEqual
      | FormulaOperator::Less
      | FormulaOperator::LessOrEqual
      | FormulaOperator::Greater
      | FormulaOperator::GreaterOrEqual => {
        Some(FormulaValue::Boolean(self.compare(&left, &right, op)))
      }
      _ => None,
    }
  }

  fn evaluate_function(
    &self,
    name: &Cow<'doc, str>,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let upper = name
      .trim_start_matches("_xlfn.")
      .trim_start_matches("_xlws.")
      .to_ascii_uppercase();
    match upper.as_str() {
      "LET" => self.evaluate_let(args),
      "IF" => {
        let condition = self.evaluate(args.first()?)?;
        if self.truthy(&condition) {
          args
            .get(1)
            .map(|arg| self.evaluate(arg))
            .unwrap_or(Some(FormulaValue::Boolean(true)))
        } else {
          args
            .get(2)
            .map(|arg| self.evaluate(arg))
            .unwrap_or(Some(FormulaValue::Boolean(false)))
        }
      }
      "IFERROR" | "IFNA" => {
        let value = self.evaluate(args.first()?)?;
        let use_fallback = matches!(
          (&value, upper.as_str()),
          (FormulaValue::Error(FormulaErrorValue::NA), "IFNA")
            | (FormulaValue::Error(_), "IFERROR")
        );
        if use_fallback {
          self.evaluate(args.get(1)?)
        } else {
          Some(value)
        }
      }
      "SUM" => Some(FormulaValue::Number(self.numeric_values(args).sum())),
      "PRODUCT" => Some(FormulaValue::Number(self.numeric_values(args).product())),
      "AVERAGE" => {
        let values = self.numeric_values(args).collect::<Vec<_>>();
        (!values.is_empty())
          .then(|| FormulaValue::Number(values.iter().sum::<f64>() / values.len() as f64))
      }
      "COUNT" => Some(FormulaValue::Number(
        self.numeric_values(args).count() as f64
      )),
      "COUNTA" => Some(FormulaValue::Number(
        self
          .values(args)
          .filter(|value| !matches!(value, FormulaValue::Blank))
          .count() as f64,
      )),
      "ISERROR" => Some(FormulaValue::Boolean(matches!(
        args.first().and_then(|arg| self.evaluate(arg)),
        Some(FormulaValue::Error(_))
      ))),
      "ISNA" => Some(FormulaValue::Boolean(matches!(
        args.first().and_then(|arg| self.evaluate(arg)),
        Some(FormulaValue::Error(FormulaErrorValue::NA))
      ))),
      "ISERR" => Some(FormulaValue::Boolean(matches!(
        args.first().and_then(|arg| self.evaluate(arg)),
        Some(FormulaValue::Error(error)) if error != FormulaErrorValue::NA
      ))),
      "ISBLANK" => Some(FormulaValue::Boolean(matches!(
        args.first().and_then(|arg| self.evaluate(arg)),
        Some(FormulaValue::Blank)
      ))),
      "ISTEXT" => Some(FormulaValue::Boolean(matches!(
        args.first().and_then(|arg| self.evaluate(arg)),
        Some(FormulaValue::String(_))
      ))),
      "ISNONTEXT" => Some(FormulaValue::Boolean(!matches!(
        args.first().and_then(|arg| self.evaluate(arg)),
        Some(FormulaValue::String(_))
      ))),
      "ISLOGICAL" => Some(FormulaValue::Boolean(matches!(
        args.first().and_then(|arg| self.evaluate(arg)),
        Some(FormulaValue::Boolean(_))
      ))),
      "ISNUMBER" => Some(FormulaValue::Boolean(matches!(
        args.first().and_then(|arg| self.evaluate(arg)),
        Some(FormulaValue::Number(_))
      ))),
      "ISREF" => Some(FormulaValue::Boolean(matches!(
        args.first().and_then(|arg| self.evaluate(arg)),
        Some(FormulaValue::Reference(_))
      ))),
      "ISFORMULA" => self.evaluate_is_formula(args),
      "ERROR.TYPE" => self.evaluate_error_type(args),
      "MIN" => self
        .numeric_values(args)
        .reduce(f64::min)
        .map(FormulaValue::Number),
      "MAX" => self
        .numeric_values(args)
        .reduce(f64::max)
        .map(FormulaValue::Number),
      "AND" => Some(FormulaValue::Boolean(
        self.values(args).all(|value| self.truthy(&value)),
      )),
      "OR" => Some(FormulaValue::Boolean(
        self.values(args).any(|value| self.truthy(&value)),
      )),
      "NOT" => Some(FormulaValue::Boolean(
        !self.truthy(&self.evaluate(args.first()?)?),
      )),
      "TRUE" => Some(FormulaValue::Boolean(true)),
      "FALSE" => Some(FormulaValue::Boolean(false)),
      "N" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?).unwrap_or(0.0),
      )),
      "COUNTBLANK" => Some(FormulaValue::Number(
        self.count_blank(&self.evaluate(args.first()?)?) as f64,
      )),
      "ABS" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?)?.abs(),
      )),
      "SIGN" => Some(FormulaValue::Number(sign_number(
        self.number(&self.evaluate(args.first()?)?)?,
      ))),
      "INT" => Some(FormulaValue::Number(approx_floor(
        self.number(&self.evaluate(args.first()?)?)?,
      ))),
      "TRUNC" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?)?.trunc(),
      )),
      "MOD" => {
        let number = self.number(&self.evaluate(args.first()?)?)?;
        let divisor = self.number(&self.evaluate(args.get(1)?)?)?;
        if divisor == 0.0 {
          Some(FormulaValue::Error(FormulaErrorValue::Div0))
        } else {
          Some(FormulaValue::Number(
            number - divisor * approx_floor(number / divisor),
          ))
        }
      }
      "EVEN" => Some(FormulaValue::Number(even_odd(
        self.number(&self.evaluate(args.first()?)?)?,
        true,
      ))),
      "ODD" => Some(FormulaValue::Number(even_odd(
        self.number(&self.evaluate(args.first()?)?)?,
        false,
      ))),
      "SQRT" => {
        let value = self.number(&self.evaluate(args.first()?)?)?;
        if value < 0.0 {
          Some(FormulaValue::Error(FormulaErrorValue::Num))
        } else {
          Some(FormulaValue::Number(value.sqrt()))
        }
      }
      "POWER" => Some(FormulaValue::Number(
        self
          .number(&self.evaluate(args.first()?)?)?
          .powf(self.number(&self.evaluate(args.get(1)?)?)?),
      )),
      "PI" => Some(FormulaValue::Number(std::f64::consts::PI)),
      "RADIANS" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?)?.to_radians(),
      )),
      "DEGREES" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?)?.to_degrees(),
      )),
      "SIN" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?)?.sin(),
      )),
      "COS" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?)?.cos(),
      )),
      "TAN" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?)?.tan(),
      )),
      "ASIN" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?)?.asin(),
      )),
      "ACOS" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?)?.acos(),
      )),
      "ATAN" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?)?.atan(),
      )),
      "ATAN2" => Some(FormulaValue::Number(
        self
          .number(&self.evaluate(args.get(1)?)?)?
          .atan2(self.number(&self.evaluate(args.first()?)?)?),
      )),
      "EXP" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?)?.exp(),
      )),
      "LN" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?)?.ln(),
      )),
      "LOG10" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?)?.log10(),
      )),
      "LOG" => {
        let value = self.number(&self.evaluate(args.first()?)?)?;
        let base = args
          .get(1)
          .and_then(|arg| self.evaluate(arg))
          .and_then(|value| self.number(&value))
          .unwrap_or(10.0);
        Some(FormulaValue::Number(value.log(base)))
      }
      "SUMSQ" => Some(FormulaValue::Number(
        self.numeric_values(args).map(|value| value * value).sum(),
      )),
      "SUMPRODUCT" => self.evaluate_sumproduct(args),
      "ROUND" => {
        let value = self.number(&self.evaluate(args.first()?)?)?;
        let digits = args
          .get(1)
          .and_then(|arg| self.evaluate(arg))
          .and_then(|value| self.number(&value))
          .unwrap_or(0.0) as i32;
        Some(FormulaValue::Number(rtl_round(value, digits)))
      }
      "ROUNDUP" => self.evaluate_round_direction(args, true),
      "ROUNDDOWN" => self.evaluate_round_direction(args, false),
      "DATE" => self.evaluate_date(args),
      "TIME" => self.evaluate_time(args),
      "YEAR" => self.evaluate_date_part(args, DatePart::Year),
      "MONTH" => self.evaluate_date_part(args, DatePart::Month),
      "DAY" => self.evaluate_date_part(args, DatePart::Day),
      "WEEKDAY" => self.evaluate_weekday(args),
      "HOUR" => self.evaluate_time_part(args, TimePart::Hour),
      "MINUTE" => self.evaluate_time_part(args, TimePart::Minute),
      "SECOND" => self.evaluate_time_part(args, TimePart::Second),
      "DAYS" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?)?
          - self.number(&self.evaluate(args.get(1)?)?)?,
      )),
      "TRIM" => Some(FormulaValue::String(Cow::Owned(trim_formula_text(
        &self.text(&self.evaluate(args.first()?)?),
      )))),
      "UPPER" => Some(FormulaValue::String(Cow::Owned(
        self.text(&self.evaluate(args.first()?)?).to_uppercase(),
      ))),
      "LOWER" => Some(FormulaValue::String(Cow::Owned(
        self.text(&self.evaluate(args.first()?)?).to_lowercase(),
      ))),
      "LEN" => Some(FormulaValue::Number(
        self.text(&self.evaluate(args.first()?)?).chars().count() as f64,
      )),
      "T" => match self.evaluate(args.first()?)? {
        FormulaValue::String(text) => Some(FormulaValue::String(text)),
        _ => Some(FormulaValue::String(Cow::Borrowed(""))),
      },
      "VALUE" => self
        .text(&self.evaluate(args.first()?)?)
        .trim()
        .parse::<f64>()
        .ok()
        .map(FormulaValue::Number)
        .or(Some(FormulaValue::Error(FormulaErrorValue::Value))),
      "CODE" => self
        .text(&self.evaluate(args.first()?)?)
        .chars()
        .next()
        .map(|ch| FormulaValue::Number(ch as u32 as f64))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Value))),
      "CHAR" => {
        let code = self.number(&self.evaluate(args.first()?)?)? as u32;
        char::from_u32(code)
          .map(|ch| FormulaValue::String(Cow::Owned(ch.to_string())))
          .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
      }
      "CHOOSE" => self.evaluate_choose(args),
      "CONCAT" | "CONCATENATE" => Some(FormulaValue::String(Cow::Owned(
        self.values(args).map(|value| self.text(&value)).collect(),
      ))),
      "EXACT" => Some(FormulaValue::Boolean(
        self.text(&self.evaluate(args.first()?)?) == self.text(&self.evaluate(args.get(1)?)?),
      )),
      "FIND" => self.evaluate_find(args, true),
      "SEARCH" => self.evaluate_find(args, false),
      "REPT" => Some(FormulaValue::String(Cow::Owned(
        self
          .text(&self.evaluate(args.first()?)?)
          .repeat(self.number(&self.evaluate(args.get(1)?)?)?.max(0.0) as usize),
      ))),
      "SUBSTITUTE" => self.evaluate_substitute(args),
      "MEDIAN" => {
        let mut values = self.numeric_args(args);
        percentile_sorted(&mut values, 0.5, PercentileKind::Inc).map(FormulaValue::Number)
      }
      "STDEV.P" | "STDEVP" => variance_slice(&self.numeric_args(args), false)
        .map(|value| FormulaValue::Number(value.sqrt())),
      "STDEV.S" | "STDEV" => variance_slice(&self.numeric_args(args), true)
        .map(|value| FormulaValue::Number(value.sqrt())),
      "VAR.P" | "VARP" => variance_slice(&self.numeric_args(args), false).map(FormulaValue::Number),
      "VAR.S" | "VAR" => variance_slice(&self.numeric_args(args), true).map(FormulaValue::Number),
      "TEXT" => Some(FormulaValue::String(Cow::Owned(self.evaluate_text(args)?))),
      "TIMEVALUE" => Some(timevalue(&self.text(&self.evaluate(args.first()?)?))),
      "INDIRECT" => self.evaluate_indirect(args),
      "INDEX" => self.evaluate_index(args),
      "OFFSET" => self.evaluate_offset(args),
      "VLOOKUP" => self.evaluate_vlookup(args),
      "FORMULATEXT" => self.evaluate_formula_text(args),
      "CELL" => self.evaluate_cell(args),
      "ROW" => self.evaluate_row_column(args, false),
      "COLUMN" => self.evaluate_row_column(args, true),
      "ROWS" => self.evaluate_rows_columns(args, false),
      "COLUMNS" => self.evaluate_rows_columns(args, true),
      "MID" => self.evaluate_mid(args),
      "LEFT" => self.evaluate_left(args),
      "RIGHT" => self.evaluate_right(args),
      "CHOOSEROWS" => self.evaluate_choose_rows(args),
      "MMULT" => self.evaluate_mmult(args),
      "CEILING" => self.evaluate_ceiling(args, CeilingFloorKind::Odff),
      "CEILING.MATH" => self.evaluate_ceiling(args, CeilingFloorKind::Math),
      "CEILING.PRECISE" | "ISO.CEILING" => self.evaluate_ceiling(args, CeilingFloorKind::Precise),
      "FLOOR" => self.evaluate_floor(args, CeilingFloorKind::Odff),
      "FLOOR.MATH" => self.evaluate_floor(args, CeilingFloorKind::Math),
      "FLOOR.PRECISE" => self.evaluate_floor(args, CeilingFloorKind::Precise),
      "BETA.DIST" => self.evaluate_beta_dist(args),
      "BETA.INV" => self.evaluate_beta_inv(args),
      "BINOM.DIST" => self.evaluate_binom_dist(args),
      "BINOM.INV" => self.evaluate_binom_inv(args),
      "CHISQ.DIST" => self.evaluate_chisq_dist(args, false),
      "CHISQ.DIST.RT" => self.evaluate_chisq_dist(args, true),
      "CHISQ.INV" => self.evaluate_chisq_inv(args, false),
      "CHISQ.INV.RT" => self.evaluate_chisq_inv(args, true),
      "CHISQ.TEST" | "CHITEST" => self.evaluate_chisq_test(args),
      "CONFIDENCE.NORM" => self.evaluate_confidence_norm(args),
      "CONFIDENCE.T" => self.evaluate_confidence_t(args),
      "COVARIANCE.P" => self.evaluate_covariance(args, false),
      "COVARIANCE.S" => self.evaluate_covariance(args, true),
      "ERF.PRECISE" | "ERF" => Some(FormulaValue::Number(erf(
        self.number(&self.evaluate(args.first()?)?)?,
      ))),
      "ERFC.PRECISE" | "ERFC" => Some(FormulaValue::Number(erfc(
        self.number(&self.evaluate(args.first()?)?)?,
      ))),
      "EXPON.DIST" => self.evaluate_expon_dist(args),
      "F.DIST" => self.evaluate_f_dist(args),
      "F.DIST.RT" => self.evaluate_f_dist_rt(args),
      "F.INV" => self.evaluate_f_inv(args, false),
      "F.INV.RT" => self.evaluate_f_inv(args, true),
      "F.TEST" | "FTEST" => self.evaluate_f_test(args),
      "GAMMA.DIST" => self.evaluate_gamma_dist(args),
      "GAMMA.INV" => self.evaluate_gamma_inv(args),
      "GAMMALN.PRECISE" | "GAMMALN" => self
        .number(&self.evaluate(args.first()?)?)
        .filter(|value| *value > 0.0)
        .map(|value| FormulaValue::Number(log_gamma(value))),
      "HYPGEOM.DIST" => self.evaluate_hypgeom_dist(args),
      "LOGNORM.DIST" => self.evaluate_lognorm_dist(args),
      "LOGNORM.INV" => self.evaluate_lognorm_inv(args),
      "MODE.MULT" | "MODE.SNGL" | "MODE" => {
        mode_slice(&self.numeric_args(args)).map(FormulaValue::Number)
      }
      "NEGBINOM.DIST" => self.evaluate_negbinom_dist(args),
      "NORM.DIST" => self.evaluate_norm_dist(args),
      "NORM.INV" => self.evaluate_norm_inv(args),
      "NORM.S.DIST" => self.evaluate_norm_s_dist(args),
      "NORM.S.INV" => Some(FormulaValue::Number(norm_s_inv(
        self.number(&self.evaluate(args.first()?)?)?,
      ))),
      "PERCENTILE.EXC" => self.evaluate_percentile(args, PercentileKind::Exc),
      "PERCENTILE.INC" | "PERCENTILE" => self.evaluate_percentile(args, PercentileKind::Inc),
      "PERCENTRANK.INC" => self.evaluate_percent_rank(args),
      "POISSON.DIST" => self.evaluate_poisson_dist(args),
      "QUARTILE.EXC" => self.evaluate_quartile(args, PercentileKind::Exc),
      "QUARTILE.INC" | "QUARTILE" => self.evaluate_quartile(args, PercentileKind::Inc),
      "RANK.AVG" => self.evaluate_rank(args, true),
      "RANK.EQ" | "RANK" => self.evaluate_rank(args, false),
      "T.DIST" => self.evaluate_t_dist(args),
      "T.DIST.2T" => self.evaluate_t_dist_tails(args, 2),
      "T.DIST.RT" => self.evaluate_t_dist_tails(args, 1),
      "T.INV" => self.evaluate_t_inv(args, false),
      "T.INV.2T" => self.evaluate_t_inv(args, true),
      "T.TEST" => self.evaluate_t_test(args),
      "WEIBULL.DIST" => self.evaluate_weibull_dist(args),
      "NETWORKDAYS.INTL" | "NETWORKDAYS" => self.evaluate_networkdays(args),
      "WORKDAY.INTL" | "WORKDAY" => self.evaluate_workday(args),
      "Z.TEST" => self.evaluate_z_test(args),
      "SUBTOTAL" => self.evaluate_subtotal(args),
      "AGGREGATE" => self.evaluate_aggregate(args),
      _ => None,
    }
  }

  fn evaluate_text(&self, args: &[FormulaAst<'doc>]) -> Option<String> {
    let value = self.evaluate(args.first()?)?;
    let format = args.get(1).and_then(|arg| self.evaluate(arg));
    Some(format_text(
      &value,
      format.as_ref().map(|value| self.text(value)).as_deref(),
      self,
    ))
  }

  fn evaluate_let(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 || args.len().is_multiple_of(2) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut evaluator = FormulaEvaluator {
      book: self.book,
      current_sheet: self.current_sheet,
      current_cell: self.current_cell,
      locals: self.locals.clone(),
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

  fn evaluate_round_direction(
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

  fn evaluate_sumproduct(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let matrices = args
      .iter()
      .map(|arg| self.evaluate(arg).map(|value| self.matrix_values(&value)))
      .collect::<Option<Vec<_>>>()?;
    let first = matrices.first()?;
    let rows = first.len();
    let columns = first.first().map_or(0, Vec::len);
    if rows == 0 || columns == 0 {
      return Some(FormulaValue::Number(0.0));
    }
    if matrices
      .iter()
      .any(|matrix| matrix.len() != rows || matrix.first().map_or(0, Vec::len) != columns)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut total = 0.0;
    for row in 0..rows {
      for column in 0..columns {
        let mut product = 1.0;
        for matrix in &matrices {
          product *= self.number(&matrix[row][column]).unwrap_or(0.0);
        }
        total += product;
      }
    }
    Some(FormulaValue::Number(total))
  }

  fn evaluate_choose(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let index = self.number(&self.evaluate(args.first()?)?)?.floor() as usize;
    if index == 0 || index >= args.len() {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    self.evaluate(args.get(index)?)
  }

  fn evaluate_find(
    &self,
    args: &[FormulaAst<'doc>],
    case_sensitive: bool,
  ) -> Option<FormulaValue<'doc>> {
    let needle = self.text(&self.evaluate(args.first()?)?);
    let haystack = self.text(&self.evaluate(args.get(1)?)?);
    let start = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as usize;
    if start == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let skip = start - 1;
    let haystack_tail = haystack.chars().skip(skip).collect::<String>();
    let (haystack_search, needle_search) = if case_sensitive {
      (haystack_tail, needle)
    } else {
      (haystack_tail.to_lowercase(), needle.to_lowercase())
    };
    haystack_search
      .find(&needle_search)
      .map(|offset| {
        FormulaValue::Number((skip + haystack_search[..offset].chars().count() + 1) as f64)
      })
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  fn evaluate_substitute(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let text = self.text(&self.evaluate(args.first()?)?);
    let old = self.text(&self.evaluate(args.get(1)?)?);
    let new = self.text(&self.evaluate(args.get(2)?)?);
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

  fn evaluate_row_column(
    &self,
    args: &[FormulaAst<'doc>],
    column: bool,
  ) -> Option<FormulaValue<'doc>> {
    let address = if let Some(arg) = args.first() {
      self.as_reference(&self.evaluate(arg)?)?.range.start
    } else {
      self.current_cell.unwrap_or_default()
    };
    Some(FormulaValue::Number(if column {
      address.column as f64 + 1.0
    } else {
      address.row as f64 + 1.0
    }))
  }

  fn evaluate_rows_columns(
    &self,
    args: &[FormulaAst<'doc>],
    columns: bool,
  ) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_is_formula(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let reference = self.as_reference(&self.evaluate(args.first()?)?)?;
    let sheet = self.range_sheet(&reference);
    Some(FormulaValue::Boolean(
      self
        .book
        .formulas
        .contains_key(&(sheet, reference.range.start)),
    ))
  }

  fn evaluate_error_type(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.evaluate(args.first()?)?;
    let FormulaValue::Error(error) = value else {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    };
    Some(FormulaValue::Number(match error {
      FormulaErrorValue::Null => 1.0,
      FormulaErrorValue::Div0 => 2.0,
      FormulaErrorValue::Value => 3.0,
      FormulaErrorValue::Ref => 4.0,
      FormulaErrorValue::Name => 5.0,
      FormulaErrorValue::Num => 6.0,
      FormulaErrorValue::NA => 7.0,
      FormulaErrorValue::GettingData => 8.0,
      FormulaErrorValue::Spill => 9.0,
      FormulaErrorValue::Calc => 14.0,
      FormulaErrorValue::IllegalArgument | FormulaErrorValue::Unknown => 0.0,
    }))
  }

  fn evaluate_date(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let year = self.number(&self.evaluate(args.first()?)?)? as i32;
    let month = self.number(&self.evaluate(args.get(1)?)?)? as i32;
    let day = self.number(&self.evaluate(args.get(2)?)?)? as i32;
    if year < 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    date_serial(year, month, day).map(FormulaValue::Number)
  }

  fn evaluate_time(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_date_part(
    &self,
    args: &[FormulaAst<'doc>],
    part: DatePart,
  ) -> Option<FormulaValue<'doc>> {
    let serial = self.number(&self.evaluate(args.first()?)?)?.floor() as i32;
    let (year, month, day) = date_from_serial(serial)?;
    Some(FormulaValue::Number(match part {
      DatePart::Year => year as f64,
      DatePart::Month => month as f64,
      DatePart::Day => day as f64,
    }))
  }

  fn evaluate_weekday(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let serial = self.number(&self.evaluate(args.first()?)?)?.floor() as i64;
    let flag = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as i32;
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

  fn evaluate_time_part(
    &self,
    args: &[FormulaAst<'doc>],
    part: TimePart,
  ) -> Option<FormulaValue<'doc>> {
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let total_seconds = ((value.fract() * 86_400.0).round() as i64).rem_euclid(86_400);
    Some(FormulaValue::Number(match part {
      TimePart::Hour => (total_seconds / 3600) as f64,
      TimePart::Minute => ((total_seconds % 3600) / 60) as f64,
      TimePart::Second => (total_seconds % 60) as f64,
    }))
  }

  fn evaluate_indirect(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let text = self.text(&self.evaluate(args.first()?)?);
    self
      .resolve_reference(&text)
      .map(FormulaValue::Reference)
      .or_else(|| self.evaluate_name(&Cow::Owned(text)))
  }

  fn evaluate_index(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let reference = self.as_reference(&self.evaluate(args.first()?)?)?;
    let row_offset = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0)
      .max(1.0) as u32
      - 1;
    let column_offset = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0)
      .max(1.0) as u32
      - 1;
    Some(self.reference_cell_value(
      &reference,
      CellAddress {
        column: reference.range.start.column + column_offset,
        row: reference.range.start.row + row_offset,
      },
    ))
  }

  fn evaluate_offset(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_vlookup(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let lookup = self.text(&self.evaluate(args.first()?)?);
    let result_column = self.number(&self.evaluate(args.get(2)?)?)?.max(1.0) as u32;
    let table = self.evaluate(args.get(1)?)?;
    if let FormulaValue::Matrix(rows) = table {
      let result_index = result_column.checked_sub(1)? as usize;
      for row in rows {
        let key = row
          .first()
          .map(|value| self.text(value))
          .unwrap_or_default();
        if key == lookup {
          return row
            .get(result_index)
            .cloned()
            .or(Some(FormulaValue::Error(FormulaErrorValue::Ref)));
        }
      }
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    }

    let reference = self.as_reference(&table)?;
    for row in reference.range.start.row..=reference.range.end.row {
      let key = self.text(&self.reference_cell_value(
        &reference,
        CellAddress {
          column: reference.range.start.column,
          row,
        },
      ));
      if key == lookup {
        return Some(self.reference_cell_value(
          &reference,
          CellAddress {
            column: reference.range.start.column + result_column - 1,
            row,
          },
        ));
      }
    }
    Some(FormulaValue::Error(FormulaErrorValue::NA))
  }

  fn evaluate_formula_text(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let reference = self.as_reference(&self.evaluate(args.first()?)?)?;
    let sheet = self.range_sheet(&reference);
    self
      .book
      .formula_text(sheet, reference.range.start)
      .map(|text| FormulaValue::String(Cow::Owned(text)))
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  fn evaluate_cell(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_mid(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let text = self.text(&self.evaluate(args.first()?)?);
    let start = self.number(&self.evaluate(args.get(1)?)?)? as usize;
    let len = self.number(&self.evaluate(args.get(2)?)?)? as usize;
    Some(FormulaValue::String(Cow::Owned(
      text
        .chars()
        .skip(start.saturating_sub(1))
        .take(len)
        .collect(),
    )))
  }

  fn evaluate_left(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let text = self.text(&self.evaluate(args.first()?)?);
    let len = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as usize;
    Some(FormulaValue::String(Cow::Owned(
      text.chars().take(len).collect(),
    )))
  }

  fn evaluate_right(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let text = self.text(&self.evaluate(args.first()?)?);
    let len = self.number(&self.evaluate(args.get(1)?)?)? as usize;
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

  fn evaluate_choose_rows(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let reference = self.as_reference(&self.evaluate(args.first()?)?)?;
    let mut rows = Vec::new();
    for arg in args.iter().skip(1) {
      let row =
        reference.range.start.row + (self.number(&self.evaluate(arg)?)? as u32).saturating_sub(1);
      let mut values = Vec::new();
      for column in reference.range.start.column..=reference.range.end.column {
        values.push(self.reference_cell_value(&reference, CellAddress { column, row }));
      }
      rows.push(values);
    }
    Some(FormulaValue::Matrix(rows))
  }

  fn evaluate_mmult(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let left = self.as_reference(&self.evaluate(args.first()?)?)?;
    let right = self.as_reference(&self.evaluate(args.get(1)?)?)?;
    let rows = left.range.end.row - left.range.start.row + 1;
    let shared = left.range.end.column - left.range.start.column + 1;
    let columns = right.range.end.column - right.range.start.column + 1;
    if shared != right.range.end.row - right.range.start.row + 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let row = 0;
    let column = 0;
    if rows == 0 || columns == 0 {
      return None;
    }
    let mut total = 0.0;
    for offset in 0..shared {
      let left_value = self
        .number(&self.reference_cell_value(
          &left,
          CellAddress {
            column: left.range.start.column + offset,
            row: left.range.start.row + row,
          },
        ))
        .unwrap_or(0.0);
      let right_value = self
        .number(&self.reference_cell_value(
          &right,
          CellAddress {
            column: right.range.start.column + column,
            row: right.range.start.row + offset,
          },
        ))
        .unwrap_or(0.0);
      total += left_value * right_value;
    }
    Some(FormulaValue::Number(total))
  }

  fn evaluate_ceiling(
    &self,
    args: &[FormulaAst<'doc>],
    kind: CeilingFloorKind,
  ) -> Option<FormulaValue<'doc>> {
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let significance = match kind {
      CeilingFloorKind::Odff => args
        .get(1)
        .and_then(|arg| self.evaluate(arg))
        .and_then(|value| self.number(&value))
        .unwrap_or(if value < 0.0 { -1.0 } else { 1.0 }),
      CeilingFloorKind::Math => args
        .get(1)
        .and_then(|arg| self.evaluate(arg))
        .and_then(|value| self.number(&value))
        .unwrap_or(1.0),
      CeilingFloorKind::Precise => args
        .get(1)
        .and_then(|arg| self.evaluate(arg))
        .and_then(|value| self.number(&value))
        .unwrap_or(1.0)
        .abs(),
    };
    if value == 0.0 || significance == 0.0 {
      return Some(FormulaValue::Number(0.0));
    }
    match kind {
      CeilingFloorKind::Odff => {
        if value * significance < 0.0 {
          Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
        } else if value < 0.0 {
          let abs_mode = args
            .get(2)
            .and_then(|arg| self.evaluate(arg))
            .is_some_and(|value| self.truthy(&value));
          let quotient = value / significance;
          Some(FormulaValue::Number(
            if abs_mode {
              approx_ceil(quotient)
            } else {
              approx_floor(quotient)
            } * significance,
          ))
        } else {
          Some(FormulaValue::Number(
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
          .and_then(|arg| self.evaluate(arg))
          .is_some_and(|value| self.truthy(&value));
        let quotient = value / significance;
        Some(FormulaValue::Number(
          if !abs_mode && value < 0.0 {
            approx_floor(quotient)
          } else {
            approx_ceil(quotient)
          } * significance,
        ))
      }
      CeilingFloorKind::Precise => Some(FormulaValue::Number(
        approx_ceil(value / significance) * significance,
      )),
    }
  }

  fn evaluate_floor(
    &self,
    args: &[FormulaAst<'doc>],
    kind: CeilingFloorKind,
  ) -> Option<FormulaValue<'doc>> {
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let significance = match kind {
      CeilingFloorKind::Odff => args
        .get(1)
        .and_then(|arg| self.evaluate(arg))
        .and_then(|value| self.number(&value))
        .unwrap_or(if value < 0.0 { -1.0 } else { 1.0 }),
      CeilingFloorKind::Math => args
        .get(1)
        .and_then(|arg| self.evaluate(arg))
        .and_then(|value| self.number(&value))
        .unwrap_or(1.0),
      CeilingFloorKind::Precise => args
        .get(1)
        .and_then(|arg| self.evaluate(arg))
        .and_then(|value| self.number(&value))
        .unwrap_or(1.0)
        .abs(),
    };
    if value == 0.0 || significance == 0.0 {
      return Some(FormulaValue::Number(0.0));
    }
    match kind {
      CeilingFloorKind::Odff => {
        if value * significance < 0.0 {
          Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
        } else if value < 0.0 {
          let abs_mode = args
            .get(2)
            .and_then(|arg| self.evaluate(arg))
            .is_some_and(|value| self.truthy(&value));
          let quotient = value / significance;
          Some(FormulaValue::Number(
            if abs_mode {
              approx_floor(quotient)
            } else {
              approx_ceil(quotient)
            } * significance,
          ))
        } else {
          Some(FormulaValue::Number(
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
          .and_then(|arg| self.evaluate(arg))
          .is_some_and(|value| self.truthy(&value));
        let quotient = value / significance;
        Some(FormulaValue::Number(
          if !abs_mode && value < 0.0 {
            approx_ceil(quotient)
          } else {
            approx_floor(quotient)
          } * significance,
        ))
      }
      CeilingFloorKind::Precise => Some(FormulaValue::Number(
        approx_floor(value / significance) * significance,
      )),
    }
  }

  fn evaluate_percentile(
    &self,
    args: &[FormulaAst<'doc>],
    kind: PercentileKind,
  ) -> Option<FormulaValue<'doc>> {
    let mut values = self.value_numbers(&self.evaluate(args.first()?)?);
    let k = self.number(&self.evaluate(args.get(1)?)?)?;
    percentile_sorted(&mut values, k, kind).map(FormulaValue::Number)
  }

  fn evaluate_quartile(
    &self,
    args: &[FormulaAst<'doc>],
    kind: PercentileKind,
  ) -> Option<FormulaValue<'doc>> {
    let mut values = self.value_numbers(&self.evaluate(args.first()?)?);
    let quartile = self.number(&self.evaluate(args.get(1)?)?)?;
    percentile_sorted(&mut values, quartile / 4.0, kind).map(FormulaValue::Number)
  }

  fn evaluate_rank(&self, args: &[FormulaAst<'doc>], average: bool) -> Option<FormulaValue<'doc>> {
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let mut values = self.value_numbers(&self.evaluate(args.get(1)?)?);
    let ascending = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
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
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    }
    Some(FormulaValue::Number(if average {
      positions.iter().sum::<f64>() / positions.len() as f64
    } else {
      positions[0]
    }))
  }

  fn evaluate_beta_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let alpha = self.number(&self.evaluate(args.get(1)?)?)?;
    let beta = self.number(&self.evaluate(args.get(2)?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(3)?)?);
    let lower = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let upper = args
      .get(5)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0);
    if alpha <= 0.0 || beta <= 0.0 || upper <= lower || x < lower || x > upper {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let scaled = (x - lower) / (upper - lower);
    Some(FormulaValue::Number(if cumulative {
      beta_dist(scaled, alpha, beta)
    } else {
      beta_dist_pdf(scaled, alpha, beta) / (upper - lower)
    }))
  }

  fn evaluate_beta_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let p = self.number(&self.evaluate(args.first()?)?)?;
    let alpha = self.number(&self.evaluate(args.get(1)?)?)?;
    let beta = self.number(&self.evaluate(args.get(2)?)?)?;
    let lower = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let upper = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0);
    if !(0.0..=1.0).contains(&p) || alpha <= 0.0 || beta <= 0.0 || upper <= lower {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(
      lower + inverse_monotonic(p, 0.0, 1.0, |x| beta_dist(x, alpha, beta)) * (upper - lower),
    ))
  }

  fn evaluate_binom_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?.floor();
    let n = self.number(&self.evaluate(args.get(1)?)?)?.floor();
    let p = self.number(&self.evaluate(args.get(2)?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(3)?)?);
    if x < 0.0 || n < 0.0 || x > n || !(0.0..=1.0).contains(&p) {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(if cumulative {
      (0..=x as u64).map(|k| binom_dist_pmf(k as f64, n, p)).sum()
    } else {
      binom_dist_pmf(x, n, p)
    }))
  }

  fn evaluate_binom_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let n = self.number(&self.evaluate(args.first()?)?)?.floor();
    let p = self.number(&self.evaluate(args.get(1)?)?)?;
    let alpha = self.number(&self.evaluate(args.get(2)?)?)?;
    if n < 0.0 || !(0.0..=1.0).contains(&p) || !(0.0..=1.0).contains(&alpha) {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let mut cumulative = 0.0;
    for k in 0..=n as u64 {
      cumulative += binom_dist_pmf(k as f64, n, p);
      if cumulative >= alpha {
        return Some(FormulaValue::Number(k as f64));
      }
    }
    Some(FormulaValue::Number(n))
  }

  fn evaluate_chisq_dist(
    &self,
    args: &[FormulaAst<'doc>],
    right_tail: bool,
  ) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let df = self.number(&self.evaluate(args.get(1)?)?)?.floor();
    if df < 1.0 || x < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    if right_tail {
      return Some(FormulaValue::Number(upper_reg_igamma(df / 2.0, x / 2.0)));
    }
    let cumulative = self.truthy(&self.evaluate(args.get(2)?)?);
    Some(FormulaValue::Number(if cumulative {
      lower_reg_igamma(df / 2.0, x / 2.0)
    } else {
      chisq_dist_pdf(x, df)
    }))
  }

  fn evaluate_chisq_inv(
    &self,
    args: &[FormulaAst<'doc>],
    right_tail: bool,
  ) -> Option<FormulaValue<'doc>> {
    let p = self.number(&self.evaluate(args.first()?)?)?;
    let df = self.number(&self.evaluate(args.get(1)?)?)?.floor();
    if !(0.0..=1.0).contains(&p) || df < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let target = if right_tail { 1.0 - p } else { p };
    Some(FormulaValue::Number(inverse_positive(target, |x| {
      lower_reg_igamma(df / 2.0, x / 2.0)
    })))
  }

  fn evaluate_chisq_test(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
    let mut chi = 0.0;
    let mut has_value = false;
    for row in 0..rows {
      for column in 0..columns {
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
            chi += term;
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
    Some(FormulaValue::Number(upper_reg_igamma(df / 2.0, chi / 2.0)))
  }

  fn evaluate_confidence_norm(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let alpha = self.number(&self.evaluate(args.first()?)?)?;
    let sigma = self.number(&self.evaluate(args.get(1)?)?)?;
    let size = self.number(&self.evaluate(args.get(2)?)?)?;
    if !(0.0..1.0).contains(&alpha) || sigma <= 0.0 || size < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(
      norm_s_inv(1.0 - alpha / 2.0).abs() * sigma / size.sqrt(),
    ))
  }

  fn evaluate_confidence_t(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let alpha = self.number(&self.evaluate(args.first()?)?)?;
    let sigma = self.number(&self.evaluate(args.get(1)?)?)?;
    let size = self.number(&self.evaluate(args.get(2)?)?)?;
    if !(0.0..1.0).contains(&alpha) || sigma <= 0.0 || size < 2.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(
      t_inv_2t(alpha, size - 1.0) * sigma / size.sqrt(),
    ))
  }

  fn evaluate_covariance(
    &self,
    args: &[FormulaAst<'doc>],
    sample: bool,
  ) -> Option<FormulaValue<'doc>> {
    let left = self.value_numbers(&self.evaluate(args.first()?)?);
    let right = self.value_numbers(&self.evaluate(args.get(1)?)?);
    let count = left.len().min(right.len());
    if count == 0 || (sample && count < 2) {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    let left_mean = left.iter().take(count).sum::<f64>() / count as f64;
    let right_mean = right.iter().take(count).sum::<f64>() / count as f64;
    let sum = (0..count)
      .map(|index| (left[index] - left_mean) * (right[index] - right_mean))
      .sum::<f64>();
    Some(FormulaValue::Number(
      sum / if sample { count - 1 } else { count } as f64,
    ))
  }

  fn evaluate_expon_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let lambda = self.number(&self.evaluate(args.get(1)?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(2)?)?);
    if x < 0.0 || lambda <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(if cumulative {
      1.0 - (-lambda * x).exp()
    } else {
      lambda * (-lambda * x).exp()
    }))
  }

  fn evaluate_f_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let df1 = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    let df2 = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?);
    let cumulative = self.truthy(&self.evaluate(args.get(3)?)?);
    if x < 0.0 || df1 <= 0.0 || df2 <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(if cumulative {
      beta_dist(df1 * x / (df1 * x + df2), df1 / 2.0, df2 / 2.0)
    } else {
      let a = df1 / 2.0;
      let b = df2 / 2.0;
      (df1 / df2).powf(a) * x.powf(a - 1.0) / beta(a, b) / (1.0 + df1 * x / df2).powf(a + b)
    }))
  }

  fn evaluate_f_dist_rt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let df1 = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    let df2 = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?);
    if x < 0.0 || df1 <= 0.0 || df2 <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(beta_dist(
      df2 / (df2 + df1 * x),
      df2 / 2.0,
      df1 / 2.0,
    )))
  }

  fn evaluate_f_inv(
    &self,
    args: &[FormulaAst<'doc>],
    right_tail: bool,
  ) -> Option<FormulaValue<'doc>> {
    let p = self.number(&self.evaluate(args.first()?)?)?;
    let df1 = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    let df2 = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?);
    if !(0.0..=1.0).contains(&p) || df1 <= 0.0 || df2 <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let target = if right_tail { 1.0 - p } else { p };
    Some(FormulaValue::Number(inverse_positive(target, |x| {
      beta_dist(df1 * x / (df1 * x + df2), df1 / 2.0, df2 / 2.0)
    })))
  }

  fn evaluate_f_test(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
    let cdf = beta_dist(df1 * f / (df1 * f + df2), df1 / 2.0, df2 / 2.0);
    Some(FormulaValue::Number(2.0 * cdf.min(1.0 - cdf)))
  }

  fn evaluate_gamma_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let alpha = self.number(&self.evaluate(args.get(1)?)?)?;
    let beta = self.number(&self.evaluate(args.get(2)?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(3)?)?);
    if x < 0.0 || alpha <= 0.0 || beta <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(if cumulative {
      gamma_dist(x, alpha, beta)
    } else {
      gamma_dist_pdf(x, alpha, beta)
    }))
  }

  fn evaluate_gamma_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let p = self.number(&self.evaluate(args.first()?)?)?;
    let alpha = self.number(&self.evaluate(args.get(1)?)?)?;
    let beta = self.number(&self.evaluate(args.get(2)?)?)?;
    if !(0.0..=1.0).contains(&p) || alpha <= 0.0 || beta <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(inverse_positive(p, |x| {
      gamma_dist(x, alpha, beta)
    })))
  }

  fn evaluate_hypgeom_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let sample_success = self.number(&self.evaluate(args.first()?)?)?.floor();
    let sample_size = self.number(&self.evaluate(args.get(1)?)?)?.floor();
    let population_success = self.number(&self.evaluate(args.get(2)?)?)?.floor();
    let population_size = self.number(&self.evaluate(args.get(3)?)?)?.floor();
    let cumulative = self.truthy(&self.evaluate(args.get(4)?)?);
    if sample_success < 0.0
      || sample_size < 0.0
      || population_success < 0.0
      || population_size < 0.0
      || sample_size > population_size
      || population_success > population_size
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let pmf = |x: f64| {
      binom_coeff(population_success, x)
        * binom_coeff(population_size - population_success, sample_size - x)
        / binom_coeff(population_size, sample_size)
    };
    Some(FormulaValue::Number(if cumulative {
      (0..=sample_success as u64).map(|x| pmf(x as f64)).sum()
    } else {
      pmf(sample_success)
    }))
  }

  fn evaluate_lognorm_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let mean = self.number(&self.evaluate(args.get(1)?)?)?;
    let sigma = self.number(&self.evaluate(args.get(2)?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(3)?)?);
    if x <= 0.0 || sigma <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let z = (x.ln() - mean) / sigma;
    Some(FormulaValue::Number(if cumulative {
      norm_s_dist(z)
    } else {
      (-0.5 * z * z).exp() / (x * sigma * (2.0 * std::f64::consts::PI).sqrt())
    }))
  }

  fn evaluate_lognorm_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let p = self.number(&self.evaluate(args.first()?)?)?;
    let mean = self.number(&self.evaluate(args.get(1)?)?)?;
    let sigma = self.number(&self.evaluate(args.get(2)?)?)?;
    if !(0.0..1.0).contains(&p) || sigma <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number((mean + sigma * norm_s_inv(p)).exp()))
  }

  fn evaluate_negbinom_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let failures = self.number(&self.evaluate(args.first()?)?)?.floor();
    let successes = self.number(&self.evaluate(args.get(1)?)?)?.floor();
    let p = self.number(&self.evaluate(args.get(2)?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(3)?)?);
    if failures < 0.0 || successes < 1.0 || !(0.0..=1.0).contains(&p) {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let pmf = |f: f64| binom_coeff(f + successes - 1.0, f) * p.powf(successes) * (1.0 - p).powf(f);
    Some(FormulaValue::Number(if cumulative {
      (0..=failures as u64).map(|f| pmf(f as f64)).sum()
    } else {
      pmf(failures)
    }))
  }

  fn evaluate_norm_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let mean = self.number(&self.evaluate(args.get(1)?)?)?;
    let sigma = self.number(&self.evaluate(args.get(2)?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(3)?)?);
    if sigma <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let z = (x - mean) / sigma;
    Some(FormulaValue::Number(if cumulative {
      norm_s_dist(z)
    } else {
      norm_s_pdf(z) / sigma
    }))
  }

  fn evaluate_norm_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let p = self.number(&self.evaluate(args.first()?)?)?;
    let mean = self.number(&self.evaluate(args.get(1)?)?)?;
    let sigma = self.number(&self.evaluate(args.get(2)?)?)?;
    if !(0.0..1.0).contains(&p) || sigma <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(mean + sigma * norm_s_inv(p)))
  }

  fn evaluate_norm_s_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let z = self.number(&self.evaluate(args.first()?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(1)?)?);
    Some(FormulaValue::Number(if cumulative {
      norm_s_dist(z)
    } else {
      norm_s_pdf(z)
    }))
  }

  fn evaluate_percent_rank(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let mut values = self.value_numbers(&self.evaluate(args.first()?)?);
    values.sort_by(f64::total_cmp);
    let x = self.number(&self.evaluate(args.get(1)?)?)?;
    if values.is_empty() || x < *values.first()? || x > *values.last()? {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    }
    for (index, value) in values.iter().enumerate() {
      if *value == x {
        return Some(FormulaValue::Number(
          index as f64 / (values.len() - 1) as f64,
        ));
      }
      if *value > x {
        let previous = values[index - 1];
        let fraction = (x - previous) / (*value - previous);
        return Some(FormulaValue::Number(
          (index as f64 - 1.0 + fraction) / (values.len() - 1) as f64,
        ));
      }
    }
    Some(FormulaValue::Number(1.0))
  }

  fn evaluate_poisson_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?.floor();
    let lambda = self.number(&self.evaluate(args.get(1)?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(2)?)?);
    if x < 0.0 || lambda <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let pmf = |k: f64| (k * lambda.ln() - lambda - log_gamma(k + 1.0)).exp();
    Some(FormulaValue::Number(if cumulative {
      (0..=x as u64).map(|k| pmf(k as f64)).sum()
    } else {
      pmf(x)
    }))
  }

  fn evaluate_t_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let t = self.number(&self.evaluate(args.first()?)?)?;
    let df = self.number(&self.evaluate(args.get(1)?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(2)?)?);
    if df <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(if cumulative {
      t_dist(t, df, 4)
    } else {
      t_dist(t, df, 3)
    }))
  }

  fn evaluate_t_dist_tails(
    &self,
    args: &[FormulaAst<'doc>],
    tails: i32,
  ) -> Option<FormulaValue<'doc>> {
    let t = self.number(&self.evaluate(args.first()?)?)?;
    let df = self.number(&self.evaluate(args.get(1)?)?)?;
    if t < 0.0 || df <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(t_dist(t, df, tails)))
  }

  fn evaluate_t_inv(
    &self,
    args: &[FormulaAst<'doc>],
    two_tailed: bool,
  ) -> Option<FormulaValue<'doc>> {
    let p = self.number(&self.evaluate(args.first()?)?)?;
    let df = self.number(&self.evaluate(args.get(1)?)?)?;
    if !(0.0..1.0).contains(&p) || df <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(if two_tailed {
      t_inv_2t(p, df)
    } else {
      inverse_monotonic(p, -100.0, 100.0, |x| t_dist(x, df, 4))
    }))
  }

  fn evaluate_t_test(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let left = self.value_numbers(&self.evaluate(args.first()?)?);
    let right = self.value_numbers(&self.evaluate(args.get(1)?)?);
    let tails = self.number(&self.evaluate(args.get(2)?)?)? as i32;
    let test_type = self.number(&self.evaluate(args.get(3)?)?)? as i32;
    if left.is_empty() || right.is_empty() || !(1..=2).contains(&tails) {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
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
    Some(FormulaValue::Number(t_dist(t, df, tails)))
  }

  fn evaluate_weibull_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let alpha = self.number(&self.evaluate(args.get(1)?)?)?;
    let beta = self.number(&self.evaluate(args.get(2)?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(3)?)?);
    if x < 0.0 || alpha <= 0.0 || beta <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let pow = (x / beta).powf(alpha);
    Some(FormulaValue::Number(if cumulative {
      1.0 - (-pow).exp()
    } else {
      alpha / beta.powf(alpha) * x.powf(alpha - 1.0) * (-pow).exp()
    }))
  }

  fn evaluate_z_test(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = self.value_numbers(&self.evaluate(args.first()?)?);
    let x = self.number(&self.evaluate(args.get(1)?)?)?;
    let sigma = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or_else(|| variance_slice(&values, true).unwrap_or(0.0).sqrt());
    let z = (mean(&values)? - x) / (sigma / (values.len() as f64).sqrt());
    Some(FormulaValue::Number(1.0 - norm_s_dist(z)))
  }

  fn evaluate_networkdays(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let mut start = self.number(&self.evaluate(args.first()?)?)?.floor() as i64;
    let mut end = self.number(&self.evaluate(args.get(1)?)?)?.floor() as i64;
    let weekend_arg = args.get(2).and_then(|arg| self.evaluate(arg));
    let Some(weekend) = weekend_mask(weekend_arg.as_ref(), false, self) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let holiday_arg = args.get(3).and_then(|arg| self.evaluate(arg));
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

  fn evaluate_workday(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let mut date = self.number(&self.evaluate(args.first()?)?)?.floor() as i64;
    let mut days = self.number(&self.evaluate(args.get(1)?)?)?.floor() as i64;
    let weekend_arg = args.get(2).and_then(|arg| self.evaluate(arg));
    let Some(weekend) = weekend_mask(weekend_arg.as_ref(), true, self) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let holiday_arg = args.get(3).and_then(|arg| self.evaluate(arg));
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

  fn evaluate_subtotal(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let function = self.number(&self.evaluate(args.first()?)?)? as i32;
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

  fn evaluate_aggregate(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let function = self.number(&self.evaluate(args.first()?)?)? as i32;
    let options = aggregate_options(self.number(&self.evaluate(args.get(1)?)?)? as i32)?;
    let evaluated = args
      .get(2..)
      .unwrap_or_default()
      .iter()
      .filter_map(|arg| self.evaluate(arg))
      .collect::<Vec<_>>();
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
    aggregate_function_value(self, function, data, k, options).map(|result| match result {
      Ok(value) => FormulaValue::Number(value),
      Err(error) => FormulaValue::Error(error),
    })
  }

  fn numeric_binary(
    &self,
    left: FormulaValue<'doc>,
    right: FormulaValue<'doc>,
    op: impl FnOnce(f64, f64) -> f64,
  ) -> Option<FormulaValue<'doc>> {
    Some(FormulaValue::Number(op(
      self.number(&left)?,
      self.number(&right)?,
    )))
  }

  fn values<'b>(
    &'b self,
    args: &'b [FormulaAst<'doc>],
  ) -> impl Iterator<Item = FormulaValue<'doc>> + 'b {
    args
      .iter()
      .filter_map(|arg| self.evaluate(arg))
      .flat_map(|value| match value {
        FormulaValue::Reference(range) => self.range_values(&range),
        FormulaValue::Matrix(rows) => rows.into_iter().flatten().collect(),
        value => vec![value],
      })
  }

  fn numeric_values<'b>(&'b self, args: &'b [FormulaAst<'doc>]) -> impl Iterator<Item = f64> + 'b {
    self.values(args).filter_map(|value| self.number(&value))
  }

  fn numeric_args(&self, args: &[FormulaAst<'doc>]) -> Vec<f64> {
    self.numeric_values(args).collect()
  }

  fn value_numbers(&self, value: &FormulaValue<'doc>) -> Vec<f64> {
    match value {
      FormulaValue::Reference(reference) => self
        .range_values(reference)
        .iter()
        .filter_map(|value| self.number(value))
        .collect(),
      FormulaValue::Matrix(rows) => rows
        .iter()
        .flatten()
        .filter_map(|value| self.number(value))
        .collect(),
      value => self.number(value).into_iter().collect(),
    }
  }

  fn matrix_values(&self, value: &FormulaValue<'doc>) -> Vec<Vec<FormulaValue<'doc>>> {
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
      FormulaValue::Matrix(rows) => rows.clone(),
      value => vec![vec![value.clone()]],
    }
  }

  fn count_blank(&self, value: &FormulaValue<'doc>) -> usize {
    match value {
      FormulaValue::Reference(reference) => {
        if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
          return 0;
        }
        self
          .range_values(reference)
          .into_iter()
          .filter(|value| matches!(value, FormulaValue::Blank))
          .count()
      }
      FormulaValue::Matrix(rows) => rows
        .iter()
        .flatten()
        .filter(|value| matches!(value, FormulaValue::Blank))
        .count(),
      FormulaValue::Blank => 1,
      _ => 0,
    }
  }

  fn as_reference(&self, value: &FormulaValue<'doc>) -> Option<QualifiedRange<'doc>> {
    match value {
      FormulaValue::Reference(reference) => Some(reference.clone()),
      _ => None,
    }
  }

  fn resolve_reference(&self, reference: &str) -> Option<QualifiedRange<'doc>> {
    let reference = reference.trim();
    if let Some(table) = parse_table_reference(self.book, reference, self.current_cell) {
      return Some(table);
    }
    parse_formula_range(self.current_sheet, reference)
  }

  fn reference_cell_value(
    &self,
    reference: &QualifiedRange<'doc>,
    address: CellAddress,
  ) -> FormulaValue<'doc> {
    let sheet = self.range_sheet(reference);
    self.book.cell_value(sheet, address)
  }

  fn range_values(&self, range: &QualifiedRange<'doc>) -> Vec<FormulaValue<'doc>> {
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
        .map(|address| self.book.cell_value(sheet, address))
        .collect();
    }

    let start_row = range.range.start.row.min(range.range.end.row);
    let end_row = range.range.start.row.max(range.range.end.row);
    let start_column = range.range.start.column.min(range.range.end.column);
    let end_column = range.range.start.column.max(range.range.end.column);
    let mut values = Vec::new();
    for row in start_row..=end_row {
      for column in start_column..=end_column {
        values.push(self.book.cell_value(sheet, CellAddress { column, row }));
      }
    }
    values
  }

  fn first_value(&self, value: &FormulaValue<'doc>) -> FormulaValue<'doc> {
    match value {
      FormulaValue::Reference(range) => self
        .range_values(range)
        .into_iter()
        .next()
        .unwrap_or_default(),
      FormulaValue::Matrix(rows) => rows
        .first()
        .and_then(|row| row.first())
        .cloned()
        .unwrap_or_default(),
      value => value.clone(),
    }
  }

  fn number(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    match self.first_value(value) {
      FormulaValue::Number(value) => Some(value),
      FormulaValue::Boolean(value) => Some(if value { 1.0 } else { 0.0 }),
      FormulaValue::String(value) => value.trim().parse::<f64>().ok(),
      FormulaValue::Blank => Some(0.0),
      FormulaValue::Error(_) => None,
      FormulaValue::Matrix(_) | FormulaValue::Reference(_) => None,
    }
  }

  fn text(&self, value: &FormulaValue<'doc>) -> String {
    display_text_from_value(&self.first_value(value))
  }

  fn truthy(&self, value: &FormulaValue<'doc>) -> bool {
    match self.first_value(value) {
      FormulaValue::Boolean(value) => value,
      FormulaValue::Number(value) => value != 0.0,
      FormulaValue::String(value) => !value.is_empty(),
      FormulaValue::Blank | FormulaValue::Error(_) => false,
      FormulaValue::Matrix(_) | FormulaValue::Reference(_) => false,
    }
  }

  fn compare(
    &self,
    left: &FormulaValue<'doc>,
    right: &FormulaValue<'doc>,
    op: FormulaOperator,
  ) -> bool {
    let numeric = self.number(left).zip(self.number(right));
    let ordering = if let Some((left, right)) = numeric {
      left.partial_cmp(&right)
    } else {
      Some(self.text(left).cmp(&self.text(right)))
    };
    match op {
      FormulaOperator::Equal => ordering == Some(std::cmp::Ordering::Equal),
      FormulaOperator::NotEqual => ordering != Some(std::cmp::Ordering::Equal),
      FormulaOperator::Less => ordering == Some(std::cmp::Ordering::Less),
      FormulaOperator::LessOrEqual => matches!(
        ordering,
        Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
      ),
      FormulaOperator::Greater => ordering == Some(std::cmp::Ordering::Greater),
      FormulaOperator::GreaterOrEqual => matches!(
        ordering,
        Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
      ),
      _ => false,
    }
  }

  fn range_sheet(&self, range: &QualifiedRange<'doc>) -> SheetId {
    range
      .sheet_name
      .as_ref()
      .and_then(|name| self.book.sheet_id_by_name(name.0.as_ref()))
      .unwrap_or(range.sheet)
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PercentileKind {
  Inc,
  Exc,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CeilingFloorKind {
  Odff,
  Math,
  Precise,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DatePart {
  Year,
  Month,
  Day,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TimePart {
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
  let values = match aggregate_numbers(evaluator, args, options) {
    Ok(values) => values,
    Err(error) => return Some(Err(error)),
  };
  match function {
    1 => mean(&values),
    2 => Some(values.len() as f64),
    3 => match aggregate_counta(evaluator, args, options)? {
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
    FormulaValue::Error(error) if !options.ignore_errors => return Err(error),
    _ => *count += 1,
  }
  Ok(())
}

fn mean(values: &[f64]) -> Option<f64> {
  (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
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

fn percentile_sorted(values: &mut [f64], k: f64, kind: PercentileKind) -> Option<f64> {
  if values.is_empty() {
    return None;
  }
  values.sort_by(f64::total_cmp);
  let count = values.len() as f64;
  let rank = match kind {
    PercentileKind::Inc => 1.0 + k * (count - 1.0),
    PercentileKind::Exc => k * (count + 1.0),
  };
  if rank < 1.0 || rank > count {
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

fn kth_value(mut values: Vec<f64>, k: f64, descending: bool) -> Option<f64> {
  values.sort_by(f64::total_cmp);
  if descending {
    values.reverse();
  }
  values.get(k.max(1.0) as usize - 1).copied()
}

fn holiday_serials(
  value: Option<&FormulaValue<'_>>,
  evaluator: &FormulaEvaluator<'_, '_>,
) -> Vec<i64> {
  let Some(value) = value else {
    return Vec::new();
  };
  let mut holidays = evaluator
    .value_numbers(value)
    .into_iter()
    .map(|value| value.floor() as i64)
    .collect::<Vec<_>>();
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
    FormulaValue::Blank => String::new(),
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

fn sign_number(value: f64) -> f64 {
  if value < 0.0 {
    -1.0
  } else if value > 0.0 {
    1.0
  } else {
    0.0
  }
}

fn round_direction(value: f64, digits: i32, away_from_zero: bool) -> f64 {
  if value == 0.0 || !value.is_finite() {
    return value;
  }
  let factor = 10_f64.powi(digits.abs());
  if factor == 0.0 || !factor.is_finite() {
    return value;
  }
  let scaled = if digits < 0 {
    value / factor
  } else {
    value * factor
  };
  let rounded = if away_from_zero {
    if scaled.is_sign_negative() {
      approx_floor(scaled)
    } else {
      approx_ceil(scaled)
    }
  } else if scaled.is_sign_negative() {
    approx_ceil(scaled)
  } else {
    approx_floor(scaled)
  };
  if digits < 0 {
    rounded * factor
  } else {
    rounded / factor
  }
}

fn even_odd(value: f64, even: bool) -> f64 {
  if value == 0.0 {
    return if even { 0.0 } else { 1.0 };
  }
  let sign = if value.is_sign_negative() { -1.0 } else { 1.0 };
  let mut magnitude = value.abs().ceil();
  let is_even = (magnitude as i64) % 2 == 0;
  if is_even != even {
    magnitude += 1.0;
  }
  sign * magnitude
}

fn date_serial(year: i32, month: i32, day: i32) -> Option<f64> {
  let month_index = month - 1;
  let normalized_year = year + month_index.div_euclid(12);
  let normalized_month = month_index.rem_euclid(12) + 1;
  let days = days_from_civil(normalized_year, normalized_month, 1)? + i64::from(day - 1);
  let base = days_from_civil(1899, 12, 31)?;
  let mut serial = days - base;
  let leap_bug_start = days_from_civil(1900, 3, 1)?;
  if days >= leap_bug_start {
    serial += 1;
  }
  Some(serial as f64)
}

fn date_from_serial(serial: i32) -> Option<(i32, u32, u32)> {
  if serial == 60 {
    return Some((1900, 2, 29));
  }
  let base = days_from_civil(1899, 12, 31)?;
  let adjusted = if serial > 60 { serial - 1 } else { serial };
  civil_from_days(base + i64::from(adjusted))
}

fn days_from_civil(year: i32, month: i32, day: i32) -> Option<i64> {
  if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
    return None;
  }
  let year = year - i32::from(month <= 2);
  let era = if year >= 0 { year } else { year - 399 }.div_euclid(400);
  let yoe = year - era * 400;
  let month = month as i64;
  let doy = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + i64::from(day) - 1;
  let doe = i64::from(yoe) * 365 + i64::from(yoe) / 4 - i64::from(yoe) / 100 + doy;
  Some(i64::from(era) * 146_097 + doe - 719_468)
}

fn civil_from_days(days: i64) -> Option<(i32, u32, u32)> {
  let days = days + 719_468;
  let era = if days >= 0 { days } else { days - 146_096 }.div_euclid(146_097);
  let doe = days - era * 146_097;
  let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096).div_euclid(365);
  let year = yoe + era * 400;
  let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
  let mp = (5 * doy + 2).div_euclid(153);
  let day = doy - (153 * mp + 2).div_euclid(5) + 1;
  let month = mp + if mp < 10 { 3 } else { -9 };
  let year = year + i64::from(month <= 2);
  Some((year.try_into().ok()?, month as u32, day as u32))
}

fn trim_formula_text(value: &str) -> String {
  value.split_whitespace().collect::<Vec<_>>().join(" ")
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

fn range_intersection_value<'doc>(
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

fn column_index_to_name(mut column: u32) -> String {
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

fn parse_table_reference<'doc>(
  book: &FormulaEvaluationBook<'doc>,
  text: &str,
  current_address: Option<CellAddress>,
) -> Option<QualifiedRange<'doc>> {
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
  let column = unescape_table_reference_column(column);
  table
    .columns
    .iter()
    .position(|name| name.as_ref().eq_ignore_ascii_case(&column))
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
  let chars = selector.chars().collect::<Vec<_>>();
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
  // Source: LibreOffice sc/source/core/tool/compiler.cxx unescapeTableRefColumnSpecifier().
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
  // Source: LibreOffice sc/source/core/tool/interpr3.cxx ScInterpreter::gaussinv.
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

fn rtl_round(value: f64, decimal_places: i32) -> f64 {
  // Source: LibreOffice sal/rtl/math.cxx rtl_math_round.
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

fn approx_floor(value: f64) -> f64 {
  approx_value(value).floor()
}

fn approx_ceil(value: f64) -> f64 {
  approx_value(value).ceil()
}

fn approx_value(value: f64) -> f64 {
  // Source: LibreOffice include/rtl/math.hxx approxFloor/approxCeil.
  const TWO_POW_41: f64 = 2_199_023_255_552.0;
  if value == 0.0 || !value.is_finite() || value.abs() > TWO_POW_41 {
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

fn display_text_from_value(value: &FormulaValue<'_>) -> String {
  match value {
    FormulaValue::Number(value) if value.is_finite() && value.fract() == 0.0 => value.to_string(),
    FormulaValue::Number(value) if value.is_finite() => value.to_string(),
    FormulaValue::Number(_) => error_text_value(FormulaErrorValue::Value).to_string(),
    FormulaValue::String(value) => value.to_string(),
    FormulaValue::Boolean(value) => {
      if *value {
        "TRUE".to_string()
      } else {
        "FALSE".to_string()
      }
    }
    FormulaValue::Error(value) => error_text_value(*value).to_string(),
    FormulaValue::Blank => String::new(),
    FormulaValue::Matrix(_) | FormulaValue::Reference(_) => String::new(),
  }
}

fn format_text(
  value: &FormulaValue<'_>,
  format: Option<&str>,
  evaluator: &FormulaEvaluator<'_, '_>,
) -> String {
  let Some(number) = evaluator.number(value) else {
    return evaluator.text(value);
  };
  match format.unwrap_or("") {
    "##" | "0" => format!("{}", number.round() as i64),
    "" => evaluator.text(value),
    _ => evaluator.text(value),
  }
}

fn timevalue(text: &str) -> FormulaValue<'static> {
  let parts = text.split(':').collect::<Vec<_>>();
  if parts.len() < 2 {
    return FormulaValue::Error(FormulaErrorValue::Value);
  }
  let hour = parts[0].parse::<f64>().unwrap_or(0.0);
  let minute = parts[1].parse::<f64>().unwrap_or(0.0);
  if hour < 0.0 || minute < 0.0 {
    return FormulaValue::Error(FormulaErrorValue::Value);
  }
  FormulaValue::Number((hour * 60.0 + minute) / (24.0 * 60.0))
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

fn parse_array_constant_formula<'doc>(formula: &str) -> Option<Vec<Vec<FormulaValue<'doc>>>> {
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
              FormulaValue::Blank
            } else if item.starts_with('#') {
              FormulaValue::Error(error_value(item))
            } else if item.eq_ignore_ascii_case("TRUE") {
              FormulaValue::Boolean(true)
            } else if item.eq_ignore_ascii_case("FALSE") {
              FormulaValue::Boolean(false)
            } else if let Ok(number) = item.parse::<f64>() {
              FormulaValue::Number(number)
            } else {
              FormulaValue::String(Cow::Owned(item.trim_matches('"').to_string()))
            }
          })
          .collect()
      })
      .collect(),
  )
}

fn parse_formula_range<'doc>(sheet: SheetId, token: &str) -> Option<QualifiedRange<'doc>> {
  if token.contains(':') {
    return QualifiedRange::parse_a1(sheet, token).ok();
  }
  QualifiedAddress::parse_a1(sheet, token)
    .ok()
    .map(|address| QualifiedRange {
      sheet,
      sheet_name: address.sheet_name,
      range: CellRange {
        start: address.cell,
        end: address.cell,
      },
      start_flags: address.flags,
      end_flags: address.flags,
    })
}

fn parse_external_reference_id<'doc>(token: &str) -> Option<ExternalReferenceId<'doc>> {
  let (book, rest) = token.strip_prefix('[')?.split_once(']')?;
  let (sheet, name) = rest.rsplit_once('!').map_or((None, rest), |(sheet, name)| {
    (Some(sheet.trim_matches('\'')), name)
  });
  Some(ExternalReferenceId {
    book: Some(Cow::Owned(book.to_string())),
    sheet: sheet
      .filter(|sheet| !sheet.is_empty())
      .map(|sheet| Cow::Owned(sheet.replace("''", "'"))),
    name: (!name.is_empty()).then(|| Cow::Owned(name.to_string())),
  })
}

fn dependency_from_range<'doc>(
  sheet: SheetId,
  range: &QualifiedRange<'doc>,
) -> FormulaDependency<'doc> {
  if range.sheet_name.is_none() && range.range.start == range.range.end {
    FormulaDependency::Cell {
      sheet,
      address: range.range.start,
    }
  } else {
    FormulaDependency::Range(range.clone())
  }
}

fn parse_formula_string(value: &str, start: usize) -> (String, usize) {
  let mut parsed = String::new();
  let mut index = start + 1;
  while index < value.len() {
    let Some(ch) = value[index..].chars().next() else {
      break;
    };
    index += ch.len_utf8();
    if ch == '"' {
      if value[index..].starts_with('"') {
        parsed.push('"');
        index += 1;
      } else {
        break;
      }
    } else {
      parsed.push(ch);
    }
  }
  (parsed, index)
}

fn parse_formula_number(value: &str, start: usize) -> (f64, usize) {
  let mut index = start;
  let mut previous_was_exponent = false;
  while index < value.len() {
    let Some(ch) = value[index..].chars().next() else {
      break;
    };
    if ch.is_ascii_digit() || ch == '.' || matches!(ch, 'E' | 'e') {
      previous_was_exponent = matches!(ch, 'E' | 'e');
      index += ch.len_utf8();
    } else if matches!(ch, '+' | '-') && previous_was_exponent {
      previous_was_exponent = false;
      index += ch.len_utf8();
    } else {
      break;
    }
  }
  (
    value[start..index].parse::<f64>().unwrap_or_default(),
    index,
  )
}

fn parse_formula_operator(value: &str, start: usize) -> Option<(FormulaOperator, usize)> {
  let rest = &value[start..];
  let (operator, width) = if rest.starts_with("<>") {
    (FormulaOperator::NotEqual, 2)
  } else if rest.starts_with("<=") {
    (FormulaOperator::LessOrEqual, 2)
  } else if rest.starts_with(">=") {
    (FormulaOperator::GreaterOrEqual, 2)
  } else {
    match rest.chars().next()? {
      '+' => (FormulaOperator::Add, 1),
      '-' => (FormulaOperator::Subtract, 1),
      '*' => (FormulaOperator::Multiply, 1),
      '/' => (FormulaOperator::Divide, 1),
      '^' => (FormulaOperator::Power, 1),
      '&' => (FormulaOperator::Concat, 1),
      '=' => (FormulaOperator::Equal, 1),
      '<' => (FormulaOperator::Less, 1),
      '>' => (FormulaOperator::Greater, 1),
      '%' => (FormulaOperator::Percent, 1),
      _ => return None,
    }
  };
  Some((operator, start + width))
}

fn parse_formula_word(value: &str, start: usize) -> (&str, usize) {
  let mut index = start;
  let mut quoted = false;
  let mut table_ref_depth = 0i32;
  while index < value.len() {
    let Some(ch) = value[index..].chars().next() else {
      break;
    };
    if table_ref_depth > 0 {
      match ch {
        '[' => table_ref_depth += 1,
        ']' => table_ref_depth -= 1,
        _ => {}
      }
      index += ch.len_utf8();
      continue;
    }
    if ch == '\'' {
      quoted = !quoted;
      index += ch.len_utf8();
      continue;
    }
    if !quoted && ch == '[' {
      table_ref_depth += 1;
      index += ch.len_utf8();
      continue;
    }
    if !quoted && !is_formula_word_char(ch) {
      break;
    }
    index += ch.len_utf8();
  }
  (&value[start..index], index)
}

fn is_formula_word_char(ch: char) -> bool {
  ch.is_alphanumeric() || matches!(ch, '$' | ':' | '!' | '\'' | '[' | ']' | '.' | '_' | '#')
}

fn cell_in_range(address: CellAddress, range: &CellRange) -> bool {
  let start_column = range.start.column.min(range.end.column);
  let end_column = range.start.column.max(range.end.column);
  let start_row = range.start.row.min(range.end.row);
  let end_row = range.start.row.max(range.end.row);
  (start_column..=end_column).contains(&address.column)
    && (start_row..=end_row).contains(&address.row)
}

fn is_volatile_function(value: &str) -> bool {
  // Source: LibreOffice formula/source/core/api/FormulaCompiler.cxx IsOpCodeVolatile.
  matches!(
    value.to_ascii_uppercase().as_str(),
    "RAND"
      | "TODAY"
      | "NOW"
      | "FORMULA"
      | "INFO"
      | "INDIRECT"
      | "OFFSET"
      | "DEBUGVAR"
      | "RANDARRAY"
      | "RANDBETWEEN"
  )
}

fn is_formula_error_literal(value: &str) -> bool {
  formula_error_literals().contains(&value)
}

fn parse_formula_error_literal_at(value: &str, start: usize) -> Option<(&str, usize)> {
  let rest = value.get(start..)?;
  formula_error_literals()
    .iter()
    .find(|literal| rest.starts_with(**literal))
    .map(|literal| (*literal, start + literal.len()))
}

fn formula_error_literals() -> &'static [&'static str] {
  &[
    "#GETTING_DATA",
    "#DIV/0!",
    "#VALUE!",
    "#NULL!",
    "#REF!",
    "#NAME?",
    "#NUM!",
    "#SPILL!",
    "#CALC!",
    "Err:502",
    "#N/A",
  ]
}

fn qualified_range<'doc>(sheet: SheetId, reference: &str) -> Option<QualifiedRange<'doc>> {
  QualifiedRange::parse_a1(sheet, reference).ok()
}

fn error_text(value: &FormulaValue<'_>) -> Option<&'static str> {
  match value {
    FormulaValue::Error(error) => Some(error_text_value(*error)),
    _ => None,
  }
}

fn error_value(value: &str) -> FormulaErrorValue {
  match value {
    "#NULL!" => FormulaErrorValue::Null,
    "#DIV/0!" => FormulaErrorValue::Div0,
    "#VALUE!" => FormulaErrorValue::Value,
    "#REF!" => FormulaErrorValue::Ref,
    "#NAME?" => FormulaErrorValue::Name,
    "#NUM!" => FormulaErrorValue::Num,
    "#N/A" => FormulaErrorValue::NA,
    "#GETTING_DATA" => FormulaErrorValue::GettingData,
    "#SPILL!" => FormulaErrorValue::Spill,
    "#CALC!" => FormulaErrorValue::Calc,
    "Err:502" => FormulaErrorValue::IllegalArgument,
    _ => FormulaErrorValue::Unknown,
  }
}

fn error_text_value(value: FormulaErrorValue) -> &'static str {
  match value {
    FormulaErrorValue::Null => "#NULL!",
    FormulaErrorValue::Div0 => "#DIV/0!",
    FormulaErrorValue::Value => "#VALUE!",
    FormulaErrorValue::Ref => "#REF!",
    FormulaErrorValue::Name => "#NAME?",
    FormulaErrorValue::Num => "#NUM!",
    FormulaErrorValue::NA => "#N/A",
    FormulaErrorValue::GettingData => "#GETTING_DATA",
    FormulaErrorValue::Spill => "#SPILL!",
    FormulaErrorValue::Calc => "#CALC!",
    FormulaErrorValue::IllegalArgument => "Err:502",
    FormulaErrorValue::Unknown => "#UNKNOWN!",
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

fn external_references<'doc>(workbook: &'doc x::Workbook) -> Vec<ExternalReference<'doc>> {
  workbook
    .external_references
    .as_ref()
    .map(|references| {
      references
        .external_reference
        .iter()
        .map(|reference| ExternalReference {
          id: Cow::Borrowed(reference.id.as_str()),
          target: None,
          sheet_names: Vec::new(),
          defined_names: Vec::new(),
          unavailable: true,
        })
        .collect()
    })
    .unwrap_or_default()
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

fn defined_names<'doc>(workbook: &'doc x::Workbook) -> Vec<DefinedName<'doc>> {
  workbook
    .defined_names
    .as_ref()
    .map(|defined_names| {
      defined_names
        .defined_name
        .iter()
        .map(|name| {
          let sheet = name.local_sheet_id.map(SheetId);
          let formula_text: Cow<'doc, str> = name
            .xml_content
            .as_deref()
            .map(Cow::Borrowed)
            .unwrap_or(Cow::Borrowed(""));
          let parsed_formula = Some(parse_formula(
            sheet.unwrap_or_default(),
            formula_text.clone(),
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

    let sheet = worksheet_value_model(&identity, Some(&worksheet), &[]).unwrap();
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

    let sheet =
      worksheet_value_model(&identity, Some(&worksheet), &["Shared".to_string()]).unwrap();
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

    let sheet = worksheet_value_model(&identity, Some(&worksheet), &[]).unwrap();
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
        row: vec![x::Row {
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
        }],
      }),
      ..x::Worksheet::default()
    };
    let mut sheets = vec![worksheet_value_model(&identity, Some(&worksheet), &[]).unwrap()];
    resolve_shared_formula_dependents(&mut sheets);
    let groups = shared_formula_groups(&sheets);

    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].index, 7);
    assert_eq!(
      groups[0].range,
      Some(CellRange {
        start: CellAddress { column: 0, row: 0 },
        end: CellAddress { column: 0, row: 1 }
      })
    );
    assert_eq!(
      groups[0].dependents,
      vec![CellAddress { column: 0, row: 1 }]
    );
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
    let sheet = worksheet_value_model(&identity, Some(&worksheet), &[]).unwrap();
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
    let parsed = parse_formula(SheetId(3), Cow::Borrowed("SUM(B1:C2)+D4*2"));

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
      parsed.ast,
      Some(FormulaAst::Binary {
        op: FormulaOperator::Add,
        ..
      })
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
                parsed_formula: Some(parse_formula(SheetId(1), Cow::Borrowed("SUM(A1:A2)+3"))),
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
              parsed_formula: Some(parse_formula(SheetId(1), Cow::Borrowed("IF(0,1/0,7)"))),
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
    let parsed = parse_formula(
      SheetId(1),
      Cow::Borrowed("RAND()+[Book.xlsx]'Q1'!$A$1+LocalName"),
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
    let names = defined_names(&workbook);
    let graph = dependency_graph(&[], &names);

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

    let sheet = worksheet_value_model(&identity, Some(&worksheet), &[]).unwrap();
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

    let sheet = worksheet_value_model(&identity, Some(&worksheet), &[]).unwrap();

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
              parsed_formula: Some(parse_formula(SheetId(1), Cow::Borrowed("TaxRate*100"))),
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
              parsed_formula: Some(parse_formula(SheetId(1), Cow::Borrowed("[1]Remote!C3+1"))),
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
                parsed_formula: Some(parse_formula(SheetId(1), Cow::Borrowed("1+1"))),
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
                parsed_formula: Some(parse_formula(SheetId(1), Cow::Borrowed("A1+1"))),
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
                parsed_formula: Some(parse_formula(SheetId(1), Cow::Borrowed("{1,2;3,4}"))),
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
    let (ast, unsupported) = parse_formula_ast(
      SheetId(1),
      "SUM(MyTable1[[#This Row],[This is the first column]:[This is the,second column]])",
    );
    assert!(unsupported.is_empty());
    let evaluator = FormulaEvaluator {
      book: &book,
      current_sheet: SheetId(1),
      current_cell: Some(CellAddress { column: 0, row: 1 }),
      locals: BTreeMap::new(),
    };

    assert_eq!(
      ast.as_ref().and_then(|ast| evaluator.evaluate(ast)),
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
    let (ast, unsupported) = parse_formula_ast(SheetId(1), "SUBTOTAL(109,A1:A4)");
    assert!(unsupported.is_empty());
    let evaluator = FormulaEvaluator {
      book: &book,
      current_sheet: SheetId(1),
      current_cell: Some(CellAddress { column: 0, row: 4 }),
      locals: BTreeMap::new(),
    };

    assert_eq!(
      ast.as_ref().and_then(|ast| evaluator.evaluate(ast)),
      Some(FormulaValue::Number(1.0))
    );
  }

  #[test]
  fn evaluation_book_evaluates_reference_lookup_and_text_functions() {
    let book = FormulaEvaluationBook {
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
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "SUM(INDIRECT(\"A2:A3\"))"),
      Some(FormulaValue::Number(12.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "FORMULATEXT(D1)"),
      Some(FormulaValue::String(Cow::Owned("=SUM(A1:A3)".to_string())))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "TIMEVALUE(\"12:00\")"),
      Some(FormulaValue::Number(0.5))
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
      book.evaluate_formula_text(SheetId(1), None, "CHOOSE(2,\"a\",\"b\",\"c\")"),
      Some(FormulaValue::String(Cow::Borrowed("b")))
    );
    assert_eq!(
      book.evaluate_formula_text(
        SheetId(1),
        None,
        "CONCAT(\"a\",\"b\")&EXACT(\"x\",\"x\")&FIND(\"b\",\"abc\")"
      ),
      Some(FormulaValue::String(Cow::Owned("abTRUE2".to_string())))
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
      book.evaluate_formula_text(SheetId(1), None, "ERROR.TYPE(#DIV/0!)"),
      Some(FormulaValue::Number(2.0))
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
}
