use std::borrow::Cow;
use std::collections::BTreeMap;

use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::workbook_part::WorkbookPart;
use ooxmlsdk::schemas::x;

use crate::{
  AddressFlags, CellAddress, CellRange, DisplayValue, FormulaError, FormulaErrorValue,
  FormulaValue, QualifiedAddress, QualifiedRange, Result, SheetId,
};

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

    let identity = workbook_identity(&workbook);
    let mut sheets = identity
      .sheets
      .iter()
      .enumerate()
      .map(|(index, identity)| {
        let worksheet = worksheet_parts
          .get(index)
          .and_then(|part| part.root_element(document).ok())
          .cloned();
        worksheet_value_model(identity, worksheet.as_ref(), &shared_strings)
      })
      .collect::<Result<Vec<_>>>()?;
    resolve_shared_formula_dependents(&mut sheets);
    mark_formula_recalc_state(&mut sheets);
    let defined_names = defined_names(&workbook);
    let shared_formula_groups = shared_formula_groups(&sheets);
    let array_formula_groups = array_formula_groups(&sheets);
    let data_tables = data_tables(&sheets);
    let dependency_graph = dependency_graph(&sheets, &defined_names);

    Ok(Self {
      calculation_settings: calculation_settings(&workbook),
      calc_chain: calc_chain(document, &workbook_part)?,
      external_references: external_references(&workbook),
      defined_names,
      shared_formula_groups,
      array_formula_groups,
      data_tables,
      dependency_graph,
      identity,
      sheets,
      ..Self::default()
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FormulaNamespace<'doc> {
  pub grammar: FormulaGrammar,
  pub function_namespace: Option<Cow<'doc, str>>,
  pub external_prefixes: Vec<Cow<'doc, str>>,
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
    let mut evaluated = Vec::new();
    let mut unsupported = Vec::new();

    let targets = self.evaluation_targets();
    for (sheet_id, address) in targets {
      let Some((formula, parsed)) = self.formula_at(sheet_id, address) else {
        continue;
      };
      let Some(ast) = parsed.ast.as_ref() else {
        unsupported.extend(parsed.unsupported.clone());
        continue;
      };
      let context = FormulaEvaluator { workbook: self };
      match context.evaluate(ast) {
        Some(value) if !matches!(value, FormulaValue::Reference(_)) => {
          let item = EvaluatedFormula {
            sheet: sheet_id,
            cell: formula.address,
            value,
          };
          if self.set_evaluated_formula(item.sheet, item.cell, item.value.clone()) {
            evaluated.push(item);
          }
        }
        _ => unsupported.extend(parsed.unsupported.clone()),
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

  fn set_evaluated_formula(
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
    let Some(formula) = record.formula.as_mut() else {
      return false;
    };
    if formula.evaluated_value.as_ref() == Some(&value) {
      return false;
    }
    formula.evaluated_value = Some(value.clone());
    formula.formula_state = FormulaState::Clean;
    record.display_value = Some(DisplayValue {
      text: Cow::Owned(display_text_from_value(&value)),
      source_value: value,
      number_format_id: formula
        .number_format_context
        .as_ref()
        .and_then(|context| context.format_id),
      stale: false,
      error_text: None,
    });
    true
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CellValueRecord<'doc> {
  pub raw_value: FormulaValue<'doc>,
  pub formula: Option<FormulaCell<'doc>>,
  pub display_value: Option<DisplayValue<'doc>>,
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExternalReferenceId<'doc> {
  pub book: Option<Cow<'doc, str>>,
  pub sheet: Option<Cow<'doc, str>>,
  pub name: Option<Cow<'doc, str>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnsupportedFormulaFeature<'doc> {
  pub feature: Cow<'doc, str>,
  pub reason: Cow<'doc, str>,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuiltInName {
  PrintArea,
  PrintTitles,
  Criteria,
  Extract,
  Database,
  SheetTitle,
  ConsolidateArea,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFormatContext<'doc> {
  pub format_id: Option<u32>,
  pub format_code: Option<Cow<'doc, str>>,
  pub locale: Option<Cow<'doc, str>>,
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
pub struct EvaluationContext<'doc> {
  pub current_sheet: SheetId,
  pub current_cell: CellAddress,
  pub settings: CalculationSettings,
  pub locale: Option<Cow<'doc, str>>,
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

fn workbook_identity<'doc>(workbook: &x::Workbook) -> WorkbookIdentity<'doc> {
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
      name: Cow::Owned(sheet.name.clone()),
      relationship_id: Some(Cow::Owned(sheet.id.clone())),
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
  worksheet: Option<&x::Worksheet>,
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
  cell: &x::Cell,
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
    let formula_text: Cow<'doc, str> = Cow::Owned(formula.xml_content.clone().unwrap_or_default());
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
      formula_state: if volatile {
        FormulaState::Stale
      } else if dirty {
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
      value.unwrap_or_default().to_string()
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

fn cell_value<'doc>(cell: &x::Cell, shared_strings: &[String]) -> FormulaValue<'doc> {
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
      .map(|value| FormulaValue::String(Cow::Owned(value.to_string())))
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
        (formula.address, formula.parsed_formula.clone()),
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
      let Some((origin, Some(parsed))) = definitions.get(&(sheet.id, group_index)) else {
        continue;
      };
      formula.parsed_formula = Some(translate_shared_formula(
        sheet.id,
        parsed,
        *origin,
        formula.address,
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

fn translate_shared_formula<'doc>(
  sheet: SheetId,
  parsed: &ParsedFormula<'doc>,
  origin: CellAddress,
  target: CellAddress,
) -> ParsedFormula<'doc> {
  let column_delta = i64::from(target.column) - i64::from(origin.column);
  let row_delta = i64::from(target.row) - i64::from(origin.row);
  let mut unsupported = parsed.unsupported.clone();
  let tokens = parsed
    .tokens
    .iter()
    .map(|token| match token {
      FormulaToken::Reference(range) => {
        let (translated, complete) = translate_shared_range(range, column_delta, row_delta);
        if !complete {
          unsupported.push(UnsupportedFormulaFeature {
            feature: Cow::Borrowed("shared formula reference translation"),
            reason: Cow::Borrowed("translated reference moved before sheet origin"),
          });
        }
        FormulaToken::Reference(translated)
      }
      token => token.clone(),
    })
    .collect::<Vec<_>>();
  let dependencies = tokens
    .iter()
    .filter_map(|token| match token {
      FormulaToken::Reference(range) => Some(dependency_from_range(sheet, range)),
      FormulaToken::ExternalReference(reference) => {
        Some(FormulaDependency::External(reference.clone()))
      }
      FormulaToken::Name(name) => Some(FormulaDependency::Name(name.clone())),
      _ => None,
    })
    .collect();

  ParsedFormula {
    source: parsed.source.clone(),
    tokens,
    ast: None,
    dependencies,
    unsupported,
  }
}

fn translate_shared_range<'doc>(
  range: &QualifiedRange<'doc>,
  column_delta: i64,
  row_delta: i64,
) -> (QualifiedRange<'doc>, bool) {
  let (start, start_complete) = translate_shared_address(
    range.range.start,
    range.start_flags,
    column_delta,
    row_delta,
  );
  let (end, end_complete) =
    translate_shared_address(range.range.end, range.end_flags, column_delta, row_delta);
  (
    QualifiedRange {
      sheet: range.sheet,
      sheet_name: range.sheet_name.clone(),
      range: CellRange { start, end },
      start_flags: range.start_flags,
      end_flags: range.end_flags,
    },
    start_complete && end_complete,
  )
}

fn translate_shared_address(
  address: CellAddress,
  flags: AddressFlags,
  column_delta: i64,
  row_delta: i64,
) -> (CellAddress, bool) {
  let (column, column_complete) = if flags.absolute_column || flags.whole_row {
    (address.column, true)
  } else {
    translate_coordinate(address.column, column_delta).unwrap_or((address.column, false))
  };
  let (row, row_complete) = if flags.absolute_row || flags.whole_column {
    (address.row, true)
  } else {
    translate_coordinate(address.row, row_delta).unwrap_or((address.row, false))
  };
  (CellAddress { column, row }, column_complete && row_complete)
}

fn translate_coordinate(value: u32, delta: i64) -> Option<(u32, bool)> {
  let translated = i64::from(value).checked_add(delta)?;
  (translated >= 0).then_some((translated as u32, true))
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
  workbook: &'a WorkbookValueModel<'doc>,
}

impl<'a, 'doc> FormulaEvaluator<'a, 'doc> {
  fn evaluate(&self, ast: &FormulaAst<'doc>) -> Option<FormulaValue<'doc>> {
    match ast {
      FormulaAst::Literal(value) => Some(value.clone()),
      FormulaAst::Reference(range) => Some(FormulaValue::Reference(range.clone())),
      FormulaAst::ExternalReference(_) | FormulaAst::Name(_) => None,
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
      // Source: LibreOffice sc/source/core/tool/interpr1.cxx ScIfJump().
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
      // Source: LibreOffice sc/source/core/tool/interpr1.cxx ScIfError().
      "IFERROR" | "IFNA" => {
        let value = self.evaluate(args.first()?)?;
        let use_fallback = match (&value, upper.as_str()) {
          (FormulaValue::Error(FormulaErrorValue::NA), "IFNA") => true,
          (FormulaValue::Error(_), "IFERROR") => true,
          _ => false,
        };
        if use_fallback {
          self.evaluate(args.get(1)?)
        } else {
          Some(value)
        }
      }
      // Source: LibreOffice sc/source/core/tool/interpr6.cxx IterateParameters().
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
      "MIN" => self
        .numeric_values(args)
        .reduce(f64::min)
        .map(FormulaValue::Number),
      "MAX" => self
        .numeric_values(args)
        .reduce(f64::max)
        .map(FormulaValue::Number),
      // Source: LibreOffice sc/source/core/tool/interpr1.cxx ScAnd()/ScOr().
      "AND" => Some(FormulaValue::Boolean(
        self.values(args).all(|value| self.truthy(&value)),
      )),
      "OR" => Some(FormulaValue::Boolean(
        self.values(args).any(|value| self.truthy(&value)),
      )),
      "ABS" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?)?.abs(),
      )),
      "ROUND" => {
        let value = self.number(&self.evaluate(args.first()?)?)?;
        let digits = args
          .get(1)
          .and_then(|arg| self.evaluate(arg))
          .and_then(|value| self.number(&value))
          .unwrap_or(0.0) as i32;
        Some(FormulaValue::Number(round_to_digits(value, digits)))
      }
      _ => None,
    }
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

  fn range_values(&self, range: &QualifiedRange<'doc>) -> Vec<FormulaValue<'doc>> {
    let sheet = self.range_sheet(range);
    let Some(model) = self.workbook.sheets.iter().find(|model| model.id == sheet) else {
      return Vec::new();
    };
    model
      .cells
      .iter()
      .filter(|(address, _)| cell_in_range(**address, &range.range))
      .map(|(_, record)| {
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
      })
      .collect()
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
      .and_then(|name| {
        self
          .workbook
          .identity
          .sheets
          .iter()
          .find(|sheet| sheet.name.as_ref().eq_ignore_ascii_case(name.0.as_ref()))
          .map(|sheet| sheet.id)
      })
      .unwrap_or(range.sheet)
  }
}

fn round_to_digits(value: f64, digits: i32) -> f64 {
  let scale = 10_f64.powi(digits);
  (value * scale).round() / scale
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

fn parse_formula_range<'doc>(sheet: SheetId, token: &str) -> Option<QualifiedRange<'doc>> {
  QualifiedRange::parse_a1(sheet, token).ok().or_else(|| {
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
  while index < value.len() {
    let Some(ch) = value[index..].chars().next() else {
      break;
    };
    if ch == '\'' {
      quoted = !quoted;
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
  ch.is_ascii_alphanumeric() || matches!(ch, '$' | ':' | '!' | '\'' | '[' | ']' | '.' | '_' | '#')
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
  // Source: LibreOffice formula/source/core/api/FormulaCompiler.cxx
  // FormulaCompiler::IsOpCodeVolatile and the ocExternal RANDBETWEEN branch.
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
      | "#SPILL!"
      | "#CALC!"
  )
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

fn calc_chain<'doc>(
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

fn external_references<'doc>(workbook: &x::Workbook) -> Vec<ExternalReference<'doc>> {
  workbook
    .external_references
    .as_ref()
    .map(|references| {
      references
        .external_reference
        .iter()
        .map(|reference| ExternalReference {
          id: Cow::Owned(reference.id.clone()),
          target: None,
          sheet_names: Vec::new(),
          defined_names: Vec::new(),
          unavailable: true,
        })
        .collect()
    })
    .unwrap_or_default()
}

fn defined_names<'doc>(workbook: &x::Workbook) -> Vec<DefinedName<'doc>> {
  workbook
    .defined_names
    .as_ref()
    .map(|defined_names| {
      defined_names
        .defined_name
        .iter()
        .map(|name| {
          let sheet = name.local_sheet_id.map(SheetId);
          let formula_text: Cow<'doc, str> =
            Cow::Owned(name.xml_content.clone().unwrap_or_default());
          let parsed_formula = Some(parse_formula(
            sheet.unwrap_or_default(),
            formula_text.clone(),
          ));
          let dependencies = parsed_formula
            .as_ref()
            .map(|parsed| parsed.dependencies.clone())
            .unwrap_or_default();
          DefinedName {
            name: Cow::Owned(name.name.clone()),
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
  match name {
    "_xlnm.Print_Area" => Some(BuiltInName::PrintArea),
    "_xlnm.Print_Titles" => Some(BuiltInName::PrintTitles),
    "_xlnm.Criteria" => Some(BuiltInName::Criteria),
    "_xlnm.Extract" => Some(BuiltInName::Extract),
    "_xlnm.Database" => Some(BuiltInName::Database),
    "_xlnm.Sheet_Title" => Some(BuiltInName::SheetTitle),
    "_xlnm.Consolidate_Area" => Some(BuiltInName::ConsolidateArea),
    _ => None,
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
}
