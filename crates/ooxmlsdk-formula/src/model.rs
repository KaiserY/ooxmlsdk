use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use icu_collator::options::{CollatorOptions, Strength};
use icu_collator::{Collator, CollatorBorrowed, CollatorPreferences};
use icu_locale::Locale;
use num_complex::Complex;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::workbook_part::WorkbookPart;
use ooxmlsdk::schemas::x;
use ooxmlsdk::sdk::SdkPart;
use regex::RegexBuilder;
use statrs::distribution::{
  Binomial, Continuous, ContinuousCDF, Discrete, DiscreteCDF, Exp, Hypergeometric, LogNormal,
  Normal, StudentsT, Weibull,
};

use crate::calc::CalcEngine;
use crate::calc::combinatorics::{
  combination_count, gcd_number, lcm_number, permutation_count, permutation_with_repetition_count,
};
use crate::calc::complex::{
  FormulaComplex, binary_suffix, format_complex_number as format_formula_complex_number,
  parse_complex_number,
};
use crate::calc::datetime::{
  basis_o_datetime_text, civil_from_days, date_diff_months, date_diff_years, date_from_serial,
  date_from_serial_with_system, date_serial, date_serial_with_system, days_from_civil, days360,
  is_leap_year, is_valid_libreoffice_gregorian_date, iso_weeknum_from_serial_with_system,
  last_day_of_month, normalized_date_components, weekday_index_from_serial,
  weeks_in_year_from_serial_with_system, weeks_mode_one_index, yearfrac,
};
use crate::calc::financial::{
  OddPeriodArgs, finance_coupdaybs, finance_coupdays, finance_coupdaysnc, finance_coupncd,
  finance_coupnum, finance_couppcd, finance_duration, finance_price, finance_year_diff,
  finance_yield, financial_amordegrc, financial_amorlinc, financial_cum, financial_db,
  financial_ddb, financial_fv, financial_ipmt, financial_irr, financial_mirr, financial_nper,
  financial_oddlprice, financial_oddlyield, financial_pmt, financial_pv, financial_rate,
  financial_vdb, financial_xirr, financial_xnpv, is_coupon_frequency,
};
use crate::calc::matrix::{
  apply_householder, determinant, lup_decompose, lup_solve, matrix_multiply, solve_lower,
  solve_upper,
};
use crate::calc::numeric::{
  CeilingFloorKind, KahanSum, NumericError, approx_add, approx_ceil, approx_equal, approx_floor,
  approx_sub, ceiling as numeric_ceiling, ceiling_excel_legacy, floor as numeric_floor,
  floor_excel_legacy, floor_to_i32, floor_to_u32, floor_to_u64, floor_to_usize, formula_mod,
  kahan_sum, mround, normalize_formula_number, quotient, round_direction,
  round_half_away_from_zero, round_to_decimal_places, round_to_significant_digits, trunc_to_u32,
};
use crate::calc::query::{
  FindTextError, QueryOp, QuerySearchType, detect_query_search_type, find_byte_text_position,
  find_text_position, lookup_text_contains, may_be_regex, may_be_wildcard, parse_criteria_operator,
  regex_match, wildcard_match,
};
use crate::calc::radix::{
  base_number_text, convert_from_decimal, convert_to_decimal, decimal_text_to_number,
};
use crate::calc::regression::{
  EtsCalculation, EtsError, EtsKind, RegressionModel, RegressionScalarState, RegressionState,
  regression_model as calc_regression_model, regression_state as calc_regression_state,
  scalar_state as regression_scalar_state_from_slices,
};
use crate::calc::special::{
  BesselKind, SpecialError, bessel, erf, gamma, lo_beta_dist, lo_beta_dist_pdf, lo_binom_dist_pmf,
  lo_binom_dist_range, lo_chi_dist, lo_chisq_dist_cdf, lo_chisq_dist_pdf, lo_f_dist_pdf,
  lo_f_dist_right_tail, lo_gamma_dist, lo_gamma_dist_pdf, lo_integral_phi, lo_iterate_inverse,
  lo_phi, lo_poisson_dist, lo_t_dist, log_gamma, norm_s_dist, norm_s_inv,
};
use crate::calc::statistics::{
  PercentileKind, StatisticsError, correlation, covariance, deviation_sum_squares,
  frequency_counts, kth_value, kurtosis, mean, mode_ms_values, mode_slice, percent_rank,
  percentile_sorted, rank_value, skewness, sum_x2 as stats_sum_x2, sum_xmy2 as stats_sum_xmy2,
  trim_mean, variance_slice,
};
use crate::calc::text::{
  baht_text, clean_formula_text, full_width_like_jis, half_width_like_asc, leftb, rightb,
  roman_text_libreoffice, text_byte_len, trim_formula_text,
};
use crate::calc::units::convert_unit;
use crate::code::{
  FormulaCode, FormulaOp, formula_error_from_lex, formula_operator_from_lex,
  formula_value_from_array_constant,
};
use crate::dependency::{
  DependencyGraph, DependencyGraphBuilder, ExternalReferenceId, FormulaDependency, cow_span_text,
  dependencies_from_code, external_reference_id_from_spans,
};
use crate::function::{FormulaFunctionId, canonical_function_name};
use crate::{
  AddressFlags, CellAddress, CellRange, DisplayValue, FormulaError, FormulaErrorValue,
  FormulaValue, QualifiedAddress, QualifiedRange, Result, SheetId, SheetName,
};

mod evaluator;

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
    let metadata = workbook_metadata(document, &workbook_part)?;
    let styles = workbook_styles(document, &workbook_part)?;
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
        worksheet_value_model(
          identity,
          worksheet.as_ref(),
          &shared_strings,
          &metadata,
          &styles,
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
    let dependency_graph = dependency_graph(&sheets, &defined_names);

    Ok(Self {
      calculation_settings: calculation_settings(&workbook),
      calc_chain: calc_chain(document, &workbook_part)?,
      external_references: external_references(document, &workbook_part, &workbook)?
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
    FormulaGrammar::CalcA1 => Cow::Owned(crate::parser::normalize_calc_formula_text(formula)),
  }
}

pub fn normalize_r1c1_formula_text(formula: &str, base: CellAddress) -> String {
  crate::parser::normalize_r1c1_formula_text(formula, base)
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
          };
          match context.evaluate_code(code) {
            Some(value) => {
              let value = if formula.reference.is_some() {
                value.into_owned()
              } else {
                book
                  .final_formula_value(sheet_id, Some(address), value)
                  .into_owned()
              };
              if let Some(range) = formula.reference
                && let Some(items) =
                  array_formula_result_items(&context.compat_evaluator(), sheet_id, range, &value)
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
pub(crate) enum FormulaAst<'doc> {
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
    function: Option<FormulaFunctionId>,
    args: Vec<FormulaAst<'doc>>,
  },
  Array(Vec<Vec<FormulaAst<'doc>>>),
}

fn is_missing_argument(ast: &FormulaAst<'_>) -> bool {
  matches!(ast, FormulaAst::Literal(FormulaValue::Blank))
}

fn has_missing_required_argument(args: &[FormulaAst<'_>], indices: &[usize]) -> bool {
  indices
    .iter()
    .any(|index| args.get(*index).is_some_and(is_missing_argument))
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
      date_system: model.identity.date_system,
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
      .map(|value| self.final_formula_value(current_sheet, current_cell, value))
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
    let normalized = normalize_formula_text(formula, grammar);
    if let Some(value) =
      self.evaluate_special_formula_text(current_sheet, current_cell, normalized.as_ref())
    {
      return Some(value);
    }
    self
      .evaluate_formula_ast_value(current_sheet, current_cell, normalized.as_ref(), grammar)
      .map(|value| self.final_formula_value(current_sheet, current_cell, value))
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
    }
    .number(&value)
  }

  fn evaluate_special_formula_text(
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
    None
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
    if matches!(value, FormulaValue::Reference(_)) {
      let engine = CalcEngine::new();
      FormulaEvaluator {
        book: self,
        engine: &engine,
        current_sheet,
        current_cell,
        grammar: FormulaGrammar::ExcelA1,
        locals: BTreeMap::new(),
        array_context: false,
        current_value: None,
      }
      .first_value(&value)
    } else {
      value
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

  fn data_area_subrange(&self, sheet: SheetId, range: CellRange) -> Option<CellRange> {
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
          cell_value_record(identity.id, address, cell, shared_strings, metadata, styles)?,
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
    let formula_text: Cow<'doc, str> = formula
      .xml_content
      .as_deref()
      .map(Cow::Borrowed)
      .unwrap_or(Cow::Borrowed(""));
    let parsed_formula = parse_formula(sheet, formula_text.clone(), FormulaGrammar::ExcelA1);
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

fn parse_formula<'doc>(
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

pub(crate) struct FormulaEvaluator<'a, 'doc> {
  pub(crate) book: &'a FormulaEvaluationBook<'doc>,
  pub(crate) engine: &'a CalcEngine,
  pub(crate) current_sheet: SheetId,
  pub(crate) current_cell: Option<CellAddress>,
  pub(crate) grammar: FormulaGrammar,
  pub(crate) locals: BTreeMap<String, FormulaValue<'doc>>,
  pub(crate) array_context: bool,
  pub(crate) current_value: Option<FormulaValue<'doc>>,
}

pub(crate) struct NumericAggregate {
  pub(crate) values: Vec<f64>,
}

impl<'a, 'doc> FormulaEvaluator<'a, 'doc> {
  pub(crate) fn with_array_context(&self) -> Self {
    Self {
      book: self.book,
      engine: self.engine,
      current_sheet: self.current_sheet,
      current_cell: self.current_cell,
      grammar: self.grammar,
      locals: self.locals.clone(),
      array_context: true,
      current_value: self.current_value.clone(),
    }
  }

  pub(crate) fn with_current_value(&self, current_value: FormulaValue<'doc>) -> Self {
    Self {
      book: self.book,
      engine: self.engine,
      current_sheet: self.current_sheet,
      current_cell: self.current_cell,
      grammar: self.grammar,
      locals: self.locals.clone(),
      array_context: self.array_context,
      current_value: Some(current_value),
    }
  }

  pub(crate) fn evaluate(&self, ast: &FormulaAst<'doc>) -> Option<FormulaValue<'doc>> {
    match ast {
      FormulaAst::Literal(value) => Some(value.clone()),
      FormulaAst::Reference(range) => Some(FormulaValue::Reference(range.clone())),
      FormulaAst::ExternalReference(reference) => self.evaluate_external_reference(reference),
      FormulaAst::Name(name) => self.evaluate_name(name),
      FormulaAst::Unary { op, expr } => self.evaluate_unary(*op, expr),
      FormulaAst::Binary { op, left, right } => self.evaluate_binary(*op, left, right),
      FormulaAst::Function {
        name,
        function,
        args,
      } => self.evaluate_function(*function, name, args),
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

  pub(crate) fn evaluate_unary(
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
    let ast = evaluator::ast_from_code(parsed.code.as_ref()?)?;
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

  pub(crate) fn evaluate_binary(
    &self,
    op: FormulaOperator,
    left: &FormulaAst<'doc>,
    right: &FormulaAst<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    if op == FormulaOperator::Intersection {
      return self.evaluate_intersection_ast(left, right);
    }
    if op == FormulaOperator::Range {
      return self.evaluate_range_ast(left, right);
    }
    if op == FormulaOperator::Union {
      let left_ranges = self.reference_ranges_from_ast(left);
      let right_ranges = self.reference_ranges_from_ast(right);
      if !left_ranges.is_empty() && !right_ranges.is_empty() {
        let mut ranges = left_ranges;
        ranges.extend(right_ranges);
        return Some(FormulaValue::RefList(ranges));
      }
    }
    let left = self.evaluate(left)?;
    let right = self.with_current_value(left.clone()).evaluate(right)?;
    if let Some(error) = propagate_binary_error(&left, &right) {
      return Some(FormulaValue::Error(error));
    }
    let (left, right) = if self.array_context {
      (left, right)
    } else {
      (
        self.scalar_binary_operand(left),
        self.scalar_binary_operand(right),
      )
    };
    if let Some(error) = propagate_binary_error(&left, &right) {
      return Some(FormulaValue::Error(error));
    }
    match op {
      FormulaOperator::Add => self.numeric_binary(left, right, approx_add),
      FormulaOperator::Subtract => self.numeric_binary(left, right, approx_sub),
      FormulaOperator::Multiply => self.numeric_binary(left, right, |a, b| a * b),
      FormulaOperator::Divide => {
        if matches!(left, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
          || matches!(right, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        {
          return self.map_binary_values(left, right, |evaluator, left, right| {
            let denominator = evaluator.number(right)?;
            if denominator == 0.0 {
              Some(FormulaValue::Error(FormulaErrorValue::Div0))
            } else {
              Some(FormulaValue::Number(evaluator.number(left)? / denominator))
            }
          });
        }
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
      FormulaOperator::Union => {
        let mut rows = self.matrix_values(&left);
        rows.extend(self.matrix_values(&right));
        Some(FormulaValue::Matrix(rows))
      }
      FormulaOperator::Equal
      | FormulaOperator::NotEqual
      | FormulaOperator::Less
      | FormulaOperator::LessOrEqual
      | FormulaOperator::Greater
      | FormulaOperator::GreaterOrEqual => {
        let left_is_matrix_compare = matches!(left, FormulaValue::Matrix(_))
          || matches!(
            &left,
            FormulaValue::Reference(reference) if reference.range.cell_count_hint() != 1
          )
          || matches!(left, FormulaValue::RefList(_));
        let right_is_matrix_compare = matches!(right, FormulaValue::Matrix(_))
          || matches!(
            &right,
            FormulaValue::Reference(reference) if reference.range.cell_count_hint() != 1
          )
          || matches!(right, FormulaValue::RefList(_));
        if left_is_matrix_compare || right_is_matrix_compare {
          return self.map_binary_values(left, right, |evaluator, left, right| {
            Some(FormulaValue::Boolean(evaluator.compare(left, right, op)))
          });
        }
        let left = self.scalar_value(left);
        let right = self.scalar_value(right);
        Some(FormulaValue::Boolean(self.compare(&left, &right, op)))
      }
      _ => None,
    }
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

  pub(crate) fn evaluate_function(
    &self,
    function: Option<FormulaFunctionId>,
    name: &Cow<'doc, str>,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    crate::function::evaluate_function(self, function, name, args)
  }

  pub(crate) fn evaluate_color(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=4).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let red = self.number_arg(args, 0)?;
    let green = self.number_arg(args, 1)?;
    let blue = self.number_arg(args, 2)?;
    let alpha = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    if [red, green, blue, alpha]
      .iter()
      .any(|value| !(0.0..=255.0).contains(value))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let red = red.floor() as u32;
    let green = green.floor() as u32;
    let blue = blue.floor() as u32;
    let alpha = alpha.floor() as u32;
    Some(FormulaValue::Number(
      ((alpha << 24) | (red << 16) | (green << 8) | blue) as f64,
    ))
  }

  pub(crate) fn evaluate_clean(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.evaluate(args.first()?)?;
    if self.array_context && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
      return Some(FormulaValue::Matrix(
        self
          .matrix_values(&value)
          .into_iter()
          .map(|row| {
            row
              .into_iter()
              .map(|value| {
                FormulaValue::String(Cow::Owned(clean_formula_text(&display_text_from_value(
                  &value,
                ))))
              })
              .collect()
          })
          .collect(),
      ));
    }
    Some(FormulaValue::String(Cow::Owned(clean_formula_text(
      &self.text(&value),
    ))))
  }

  pub(crate) fn evaluate_trim(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.evaluate(args.first()?)?;
    if self.array_context && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
      return Some(FormulaValue::Matrix(
        self
          .matrix_values(&value)
          .into_iter()
          .map(|row| {
            row
              .into_iter()
              .map(|value| {
                FormulaValue::String(Cow::Owned(trim_formula_text(&display_text_from_value(
                  &value,
                ))))
              })
              .collect()
          })
          .collect(),
      ));
    }
    if is_multicell_scalar_argument(&value) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    Some(FormulaValue::String(Cow::Owned(trim_formula_text(
      &self.text(&value),
    ))))
  }

  pub(crate) fn evaluate_t(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    // Source: LibreOffice sc/source/core/tool/interpr1.cxx ScInterpreter::ScT.
    let value = self.evaluate(args.first()?)?;
    let value = match value {
      FormulaValue::Reference(reference) => self.scalar_reference_value(&reference),
      FormulaValue::Matrix(rows) => rows
        .into_iter()
        .next()
        .and_then(|row| row.into_iter().next())
        .unwrap_or_default(),
      value => value,
    };
    match value {
      FormulaValue::String(text) => Some(FormulaValue::String(text)),
      FormulaValue::Error(error) => Some(FormulaValue::Error(error)),
      _ => Some(FormulaValue::String(Cow::Borrowed(""))),
    }
  }

  pub(crate) fn evaluate_value(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    // Source: LibreOffice sc/source/core/tool/interpr1.cxx ScInterpreter::ScValue.
    let value = self.evaluate(args.first()?)?;
    if self.array_context && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
      return Some(FormulaValue::Matrix(
        self
          .matrix_values(&value)
          .into_iter()
          .map(|row| {
            row
              .into_iter()
              .map(|value| self.value_from_formatter_text(&value))
              .collect()
          })
          .collect(),
      ));
    }
    let value = match value {
      FormulaValue::Reference(reference) => {
        if reference.range.cell_count_hint() != 1 {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        }
        self.scalar_reference_value(&reference)
      }
      value => value,
    };
    Some(self.value_from_formatter_text(&value))
  }

  pub(crate) fn value_from_formatter_text(&self, value: &FormulaValue<'doc>) -> FormulaValue<'doc> {
    match value {
      FormulaValue::Blank => return FormulaValue::Number(0.0),
      FormulaValue::Number(number) => return FormulaValue::Number(*number),
      FormulaValue::Error(error) => return FormulaValue::Error(*error),
      _ => {}
    }
    let text = self.text(value);
    let text = text.trim();
    if text.is_empty() {
      return FormulaValue::Number(0.0);
    }
    let parsed = text
      .parse::<f64>()
      .ok()
      .or_else(|| crate::parser::grouped_formula_number(text))
      .or_else(|| parse_date_input(text, self.book.date_system).map(f64::floor))
      .or_else(|| match timevalue(text) {
        FormulaValue::Number(number) => Some(number),
        _ => None,
      })
      .or_else(|| {
        text
          .strip_prefix('$')
          .and_then(|value| crate::parser::grouped_formula_number(value.trim()))
      });
    parsed
      .map(FormulaValue::Number)
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::Value))
  }

  pub(crate) fn evaluate_getpivotdata(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 || args.len() % 2 == 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Ref));
    }
    let old_syntax = args.len() == 2
      && matches!(
        self.evaluate(args.first()?)?,
        FormulaValue::Reference(_) | FormulaValue::Matrix(_)
      );
    let (block, data_field_name, filters) = if old_syntax {
      let block = self.as_reference(&self.evaluate(args.first()?)?)?;
      let filter_value = self.evaluate(args.get(1)?)?;
      let filter_text = self.text(&self.first_value(&filter_value));
      let (data_field_name, filters) = parse_getpivotdata_filter_text(&filter_text);
      (block, data_field_name, filters)
    } else {
      let data_field_name = Cow::Owned(self.pivot_argument_text(args.first()?)?);
      let block = self.as_reference(&self.evaluate(args.get(1)?)?)?;
      let mut filters = Vec::new();
      for pair in args[2..].chunks(2) {
        filters.push(PivotFieldFilter {
          field_name: Cow::Owned(self.pivot_argument_text(&pair[0])?),
          match_value: Cow::Owned(self.pivot_argument_text(&pair[1])?),
        });
      }
      (block, Some(data_field_name), filters)
    };
    let request = PivotDataRequest {
      current_sheet: self.current_sheet,
      block,
      data_field_name,
      filters,
    };
    Some(match self.pivot_data(&request) {
      Ok(value) => value,
      Err(error) => FormulaValue::Error(error),
    })
  }

  pub(crate) fn pivot_argument_text(&self, arg: &FormulaAst<'doc>) -> Option<String> {
    let value = self.evaluate(arg)?;
    Some(self.text(&self.first_value(&value)))
  }

  pub(crate) fn evaluate_if(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let condition = self.evaluate(args.first()?)?;
    let if_value = |arg: Option<&FormulaAst<'doc>>, default: FormulaValue<'doc>| {
      Some(match arg.and_then(|arg| self.evaluate(arg)) {
        Some(FormulaValue::Blank) => FormulaValue::Number(0.0),
        Some(value) => value,
        None => default,
      })
    };
    if self.array_context
      && matches!(
        condition,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      )
    {
      let true_value = if_value(args.get(1), FormulaValue::Boolean(true))?;
      let false_value = if_value(args.get(2), FormulaValue::Boolean(false))?;
      return self.map_if_values(condition, true_value, false_value);
    }
    if let FormulaValue::Error(error) = self.first_value(&condition) {
      return Some(FormulaValue::Error(error));
    }
    if self.truthy(&condition) {
      if_value(args.get(1), FormulaValue::Boolean(true))
    } else {
      if_value(args.get(2), FormulaValue::Boolean(false))
    }
  }

  pub(crate) fn map_if_values(
    &self,
    condition: FormulaValue<'doc>,
    true_value: FormulaValue<'doc>,
    false_value: FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let conditions = self.matrix_values(&condition);
    let true_values = self.matrix_values(&true_value);
    let false_values = self.matrix_values(&false_value);
    let (condition_rows, condition_columns) = matrix_dimensions(&conditions);
    let (true_rows, true_columns) = matrix_dimensions(&true_values);
    let (false_rows, false_columns) = matrix_dimensions(&false_values);
    if condition_rows == 0
      || condition_columns == 0
      || true_rows == 0
      || true_columns == 0
      || false_rows == 0
      || false_columns == 0
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let rows = condition_rows.max(true_rows).max(false_rows);
    let columns = condition_columns.max(true_columns).max(false_columns);
    if !matrix_can_broadcast(condition_rows, condition_columns, rows, columns)
      || !matrix_can_broadcast(true_rows, true_columns, rows, columns)
      || !matrix_can_broadcast(false_rows, false_columns, rows, columns)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }

    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let condition = &conditions[row.min(condition_rows - 1)][column.min(condition_columns - 1)];
        result_row.push(match condition {
          FormulaValue::Error(error) => FormulaValue::Error(*error),
          condition if self.truthy(condition) => {
            true_values[row.min(true_rows - 1)][column.min(true_columns - 1)].clone()
          }
          _ => false_values[row.min(false_rows - 1)][column.min(false_columns - 1)].clone(),
        });
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_if_error(
    &self,
    args: &[FormulaAst<'doc>],
    na_only: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    if args.first().is_some_and(is_missing_argument) {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let value = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::Unknown));
    if self.array_context
      && matches!(
        value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      )
    {
      let fallback = self.evaluate(args.get(1)?)?;
      return self.map_if_error_values(value, fallback, na_only);
    }
    let value = self.scalar_value(value);
    if formula_error_matches(&value, na_only) {
      self.evaluate(args.get(1)?)
    } else {
      Some(value)
    }
  }

  pub(crate) fn map_if_error_values(
    &self,
    value: FormulaValue<'doc>,
    fallback: FormulaValue<'doc>,
    na_only: bool,
  ) -> Option<FormulaValue<'doc>> {
    let values = self.matrix_values(&value);
    let fallbacks = self.matrix_values(&fallback);
    let (value_rows, value_columns) = matrix_dimensions(&values);
    let (fallback_rows, fallback_columns) = matrix_dimensions(&fallbacks);
    if value_rows == 0 || value_columns == 0 || fallback_rows == 0 || fallback_columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let rows = value_rows.max(fallback_rows);
    let columns = value_columns.max(fallback_columns);
    if !matrix_can_broadcast(value_rows, value_columns, rows, columns)
      || !matrix_can_broadcast(fallback_rows, fallback_columns, rows, columns)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }

    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let value = &values[row.min(value_rows - 1)][column.min(value_columns - 1)];
        if formula_error_matches(value, na_only) {
          result_row
            .push(fallbacks[row.min(fallback_rows - 1)][column.min(fallback_columns - 1)].clone());
        } else {
          result_row.push(value.clone());
        }
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_text(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let array_evaluator = self.with_array_context();
    let value = array_evaluator.evaluate(args.first()?)?;
    let format = array_evaluator.evaluate(args.get(1)?)?;
    let values = array_evaluator.matrix_values(&value);
    let formats = array_evaluator.matrix_values(&format);
    let value_rows = values.len();
    let value_columns = values.first().map_or(0, Vec::len);
    let format_rows = formats.len();
    let format_columns = formats.first().map_or(0, Vec::len);
    if value_rows == 0 || value_columns == 0 || format_rows == 0 || format_columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let rows = value_rows.max(format_rows);
    let columns = value_columns.max(format_columns);
    if !matrix_can_broadcast(value_rows, value_columns, rows, columns)
      || !matrix_can_broadcast(format_rows, format_columns, rows, columns)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }

    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let value = &values[row.min(value_rows - 1)][column.min(value_columns - 1)];
        let format = &formats[row.min(format_rows - 1)][column.min(format_columns - 1)];
        result_row.push(if let Some(error) = propagate_binary_error(value, format) {
          FormulaValue::Error(error)
        } else {
          FormulaValue::String(Cow::Owned(format_text(value, format, self)))
        });
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_numbervalue(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.is_empty() || args.len() > 3 {
      return None;
    }
    let mut text = self.text(&self.evaluate(args.first()?)?);
    let decimal = if let Some(arg) = args.get(1) {
      match self.evaluate(arg)? {
        FormulaValue::Blank => {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        }
        value => {
          let text = self.text(&value);
          if text.chars().count() != 1 {
            return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
          }
          text
        }
      }
    } else {
      String::new()
    };
    let group = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.text(&value))
      .unwrap_or_default();
    if !decimal.is_empty() && group.contains(&decimal) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let percent_count = text.chars().rev().take_while(|ch| *ch == '%').count();
    text.truncate(text.len() - percent_count);
    text = text
      .chars()
      .filter(|ch| !ch.is_whitespace() && !group.contains(*ch))
      .collect();
    if text.is_empty() {
      return Some(FormulaValue::Number(0.0));
    }
    if decimal.is_empty() && text.contains('.') {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    if decimal != "." && !decimal.is_empty() {
      text = text.replace(&decimal, ".");
    }
    text
      .trim()
      .parse::<f64>()
      .map(|value| value / 100_f64.powi(percent_count as i32))
      .map(FormulaValue::Number)
      .ok()
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_isblank(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return None;
    }
    let value = self.evaluate(args.first()?)?;
    if let FormulaValue::Reference(reference) = &value
      && reference.range.cell_count_hint() == 1
    {
      let sheet = self.range_sheet(reference);
      if self
        .book
        .formulas
        .contains_key(&(sheet, reference.range.start))
      {
        return Some(FormulaValue::Boolean(false));
      }
    }
    if self.array_context
      && let FormulaValue::Reference(reference) = &value
    {
      if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      let sheet = self.range_sheet(reference);
      let start_row = reference.range.start.row.min(reference.range.end.row);
      let end_row = reference.range.start.row.max(reference.range.end.row);
      let start_column = reference.range.start.column.min(reference.range.end.column);
      let end_column = reference.range.start.column.max(reference.range.end.column);
      let mut rows = Vec::new();
      for row in start_row..=end_row {
        let mut result_row = Vec::new();
        for column in start_column..=end_column {
          let address = CellAddress { column, row };
          result_row.push(FormulaValue::Boolean(
            !self.book.formulas.contains_key(&(sheet, address))
              && matches!(self.book.cell_value(sheet, address), FormulaValue::Blank),
          ));
        }
        rows.push(result_row);
      }
      return Some(FormulaValue::Matrix(rows));
    }
    if self.array_context && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
      let matrix = self.matrix_values(&value);
      return Some(FormulaValue::Matrix(
        matrix
          .into_iter()
          .map(|row| {
            row
              .into_iter()
              .map(|value| FormulaValue::Boolean(matches!(value, FormulaValue::Blank)))
              .collect()
          })
          .collect(),
      ));
    }
    Some(FormulaValue::Boolean(matches!(
      self.first_value(&value),
      FormulaValue::Blank
    )))
  }

  pub(crate) fn evaluate_isnumber(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return None;
    }
    let value = self.evaluate(args.first()?)?;
    if self.array_context
      && matches!(
        value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      )
    {
      return self.map_unary_values(value, |_, value| {
        Some(FormulaValue::Boolean(matches!(
          value,
          FormulaValue::Number(_) | FormulaValue::Boolean(_)
        )))
      });
    }
    Some(FormulaValue::Boolean(matches!(
      self.information_scalar_value(value),
      Some(FormulaValue::Number(_) | FormulaValue::Boolean(_))
    )))
  }

  pub(crate) fn evaluate_not(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    match &value {
      FormulaValue::Reference(reference) if reference.range.cell_count_hint() == 1 => {}
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_) => {
        return self.map_unary_values(value, |evaluator, value| match value {
          FormulaValue::Error(error) => Some(FormulaValue::Error(*error)),
          value => Some(FormulaValue::Boolean(!evaluator.truthy(value))),
        });
      }
      _ => {}
    }
    let value = self.scalar_value(value);
    match value {
      FormulaValue::Error(error) => Some(FormulaValue::Error(error)),
      value => Some(FormulaValue::Boolean(!self.truthy(&value))),
    }
  }

  pub(crate) fn evaluate_n(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    if self.array_context
      && matches!(
        value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      )
    {
      return self.map_unary_values(value, |_, value| match value {
        FormulaValue::Number(value) => Some(FormulaValue::Number(*value)),
        FormulaValue::Boolean(value) => Some(FormulaValue::Number(if *value { 1.0 } else { 0.0 })),
        FormulaValue::Error(error) => Some(FormulaValue::Error(*error)),
        _ => Some(FormulaValue::Number(0.0)),
      });
    }
    match &value {
      FormulaValue::Reference(reference) if reference.range.cell_count_hint() != 1 => {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      FormulaValue::RefList(_) => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
      _ => {}
    }
    Some(FormulaValue::Number(match self.first_value(&value) {
      FormulaValue::Number(value) => value,
      FormulaValue::Boolean(value) => {
        if value {
          1.0
        } else {
          0.0
        }
      }
      FormulaValue::Error(error) => return Some(FormulaValue::Error(error)),
      _ => 0.0,
    }))
  }

  pub(crate) fn evaluate_dollar(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.is_empty() || args.len() > 2 {
      return None;
    }
    let value = self.evaluate(args.first()?)?;
    let digits = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .unwrap_or(FormulaValue::Number(2.0));
    if self.array_context
      && (matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        || matches!(digits, FormulaValue::Reference(_) | FormulaValue::Matrix(_)))
    {
      return self.map_binary_values(value, digits, |evaluator, value, digits| {
        Some(evaluator.dollar_value(value, digits))
      });
    }
    Some(self.dollar_value(&value, &digits))
  }

  pub(crate) fn dollar_value(
    &self,
    value: &FormulaValue<'doc>,
    digits: &FormulaValue<'doc>,
  ) -> FormulaValue<'doc> {
    let Some(value) = self.number(value) else {
      return FormulaValue::Error(FormulaErrorValue::Value);
    };
    let Some(digits) = self.number(digits) else {
      return FormulaValue::Error(FormulaErrorValue::Value);
    };
    let Some(digits) = floor_to_i32(approx_floor(digits)) else {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    };
    if !(-15..=15).contains(&digits) {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    FormulaValue::String(Cow::Owned(format_dollar_value(
      round_to_decimal_places(value, digits),
      digits.max(0) as usize,
    )))
  }

  pub(crate) fn evaluate_dollar_decimal(
    &self,
    args: &[FormulaAst<'doc>],
    fractional_to_decimal: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let fraction = self.number(&self.evaluate(args.get(1)?)?)?.trunc();
    if fraction < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let integer = value.trunc();
    let decimal = value - integer;
    let result = if fractional_to_decimal {
      integer + decimal / fraction * 10.0_f64.powf(fraction.log10().ceil())
    } else {
      integer + decimal * fraction * 10.0_f64.powf(-fraction.log10().ceil())
    };
    Some(FormulaValue::Number(result))
  }

  pub(crate) fn evaluate_fixed(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.is_empty() || args.len() > 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value_arg = self.evaluate(args.first()?)?;
    if !self.array_context && is_multicell_scalar_argument(&value_arg) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let Some(value) = self
      .number(&value_arg)
      .or_else(|| crate::parser::grouped_formula_number(&self.text(&value_arg)))
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let digits = match args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
    {
      Some(digits) => match floor_to_i32(approx_floor(digits)) {
        Some(digits) => digits,
        None => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
      },
      None => 2,
    };
    if !(-15..=15).contains(&digits) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let no_commas = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    let rounded = round_to_decimal_places(value, digits);
    let text = if digits >= 0 {
      format!("{rounded:.digits$}", digits = digits as usize)
    } else {
      format!("{rounded:.0}")
    };
    Some(FormulaValue::String(Cow::Owned(if no_commas {
      text
    } else {
      add_group_separators(&text)
    })))
  }

  pub(crate) fn evaluate_decimal(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let text = self.text(&self.evaluate(args.first()?)?);
    let radix = self.number(&self.evaluate(args.get(1)?)?)?;
    let Some(radix) = trunc_to_u32(radix) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    if !(2..=36).contains(&radix) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    decimal_text_to_number(&text, radix)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_bit_binary(
    &self,
    args: &[FormulaAst<'doc>],
    op: impl FnOnce(u64, u64) -> u64,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(left) = self.number_arg(args, 0).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(right) = self.number_arg(args, 1).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if !(0.0..281_474_976_710_656.0).contains(&left)
      || !(0.0..281_474_976_710_656.0).contains(&right)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let Some(left) = floor_to_u64(left) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    };
    let Some(right) = floor_to_u64(right) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    };
    Some(FormulaValue::Number(op(left, right) as f64))
  }

  pub(crate) fn evaluate_bit_shift(
    &self,
    args: &[FormulaAst<'doc>],
    left_shift: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(value) = self.number_arg(args, 0).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(shift) = self.number_arg(args, 1).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(shift) = floor_to_i32(shift) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    };
    if !(0.0..281_474_976_710_656.0).contains(&value) || shift.abs() > 53 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let Some(value) = floor_to_u64(value) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    };
    let result = if left_shift == (shift >= 0) {
      value.checked_shl(shift.unsigned_abs()).unwrap_or(0)
    } else {
      value.checked_shr(shift.unsigned_abs()).unwrap_or(0)
    };
    Some(FormulaValue::Number(result as f64))
  }

  pub(crate) fn evaluate_convert(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let from = self.text(&self.evaluate(args.get(1)?)?);
    let to = self.text(&self.evaluate(args.get(2)?)?);
    Some(match convert_unit(value, &from, &to) {
      Ok(value) => FormulaValue::Number(value),
      Err(_) => FormulaValue::Error(FormulaErrorValue::IllegalArgument),
    })
  }

  pub(crate) fn evaluate_base(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let value = self.evaluate(args.first()?)?;
    let radix = self.evaluate(args.get(1)?)?;
    let min_len = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .unwrap_or(FormulaValue::Number(1.0));
    if self.array_context
      && (matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        || matches!(radix, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        || matches!(
          min_len,
          FormulaValue::Reference(_) | FormulaValue::Matrix(_)
        ))
    {
      return self.map_ternary_values(value, radix, min_len, |evaluator, value, radix, min_len| {
        evaluator.base_value(value, radix, min_len)
      });
    }
    self.base_value(&value, &radix, &min_len)
  }

  pub(crate) fn base_value(
    &self,
    value: &FormulaValue<'doc>,
    radix: &FormulaValue<'doc>,
    min_len: &FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let value = approx_floor(self.number(value)?);
    let radix = approx_floor(self.number(radix)?);
    let min_len_value = approx_floor(self.number(min_len)?);
    let min_len = if (1.0..u16::MAX as f64).contains(&min_len_value) {
      floor_to_usize(min_len_value)?
    } else if min_len_value == 0.0 {
      1
    } else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    if value < 0.0 || !(2.0..=36.0).contains(&radix) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let text = base_number_text(value, floor_to_u32(radix)?, min_len)?;
    Some(FormulaValue::String(Cow::Owned(text)))
  }

  pub(crate) fn evaluate_base_to_decimal(
    &self,
    args: &[FormulaAst<'doc>],
    base: u32,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(arg) = args.first() else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let value = self.evaluate(arg)?;
    if self.array_context && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
      return Some(FormulaValue::Matrix(
        self
          .matrix_values(&value)
          .into_iter()
          .map(|row| {
            row
              .into_iter()
              .map(|value| {
                let text = self.base_digits_text(&value);
                convert_to_decimal(&text, base, 10)
                  .map(FormulaValue::Number)
                  .unwrap_or(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
              })
              .collect()
          })
          .collect(),
      ));
    }
    let text = self.base_digits_text(&value);
    convert_to_decimal(&text, base, 10)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_base_to_base(
    &self,
    args: &[FormulaAst<'doc>],
    from_base: u32,
    to_base: u32,
    min: f64,
    max: f64,
  ) -> Option<FormulaValue<'doc>> {
    if !(1..=2).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(arg) = args.first() else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let value = self.evaluate(arg)?;
    let places = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(|value| approx_floor(value) as i32);

    if self.array_context && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
      return Some(FormulaValue::Matrix(
        self
          .matrix_values(&value)
          .into_iter()
          .map(|row| {
            row
              .into_iter()
              .map(|value| {
                self
                  .base_to_base_value(&value, from_base, to_base, min, max, places)
                  .unwrap_or(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
              })
              .collect()
          })
          .collect(),
      ));
    }

    self.base_to_base_value(&value, from_base, to_base, min, max, places)
  }

  pub(crate) fn base_to_base_value(
    &self,
    value: &FormulaValue<'doc>,
    from_base: u32,
    to_base: u32,
    min: f64,
    max: f64,
    places: Option<i32>,
  ) -> Option<FormulaValue<'doc>> {
    let text = self.base_digits_text(value);
    let value = convert_to_decimal(&text, from_base, 10)?;
    convert_from_decimal(value, min, max, to_base, places, 10)
      .map(|value| FormulaValue::String(Cow::Owned(value)))
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_decimal_to_base(
    &self,
    args: &[FormulaAst<'doc>],
    base: u32,
    min: f64,
    max: f64,
  ) -> Option<FormulaValue<'doc>> {
    if !(1..=2).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(arg) = args.first() else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let value = self.number(&self.evaluate(arg)?)?;
    let places = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(|value| approx_floor(value) as i32);
    convert_from_decimal(value, min, max, base, places, 10)
      .map(|value| FormulaValue::String(Cow::Owned(value)))
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn base_digits_text(&self, value: &FormulaValue<'doc>) -> String {
    match self.scalar_value(value.clone()) {
      FormulaValue::Boolean(value) => {
        if value {
          "1".to_string()
        } else {
          "0".to_string()
        }
      }
      FormulaValue::Number(value) if value.is_finite() => {
        display_text_from_value(&FormulaValue::Number(approx_floor(value)))
      }
      value => self.text(&value),
    }
  }

  pub(crate) fn evaluate_let(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 || args.len().is_multiple_of(2) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut evaluator = FormulaEvaluator {
      book: self.book,
      engine: self.engine,
      current_sheet: self.current_sheet,
      current_cell: self.current_cell,
      grammar: self.grammar,
      locals: self.locals.clone(),
      array_context: self.array_context,
      current_value: self.current_value.clone(),
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

  pub(crate) fn evaluate_round_direction(
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

  pub(crate) fn evaluate_mod(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let number = self.evaluate(args.first()?)?;
    let divisor = self.evaluate(args.get(1)?)?;
    if self.array_context
      && (matches!(
        number,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      ) || matches!(
        divisor,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      ))
    {
      return self.map_binary_values(number, divisor, |evaluator, number, divisor| {
        Some(evaluator.mod_value(number, divisor))
      });
    }
    Some(self.mod_value(&number, &divisor))
  }

  pub(crate) fn mod_value(
    &self,
    number: &FormulaValue<'doc>,
    divisor: &FormulaValue<'doc>,
  ) -> FormulaValue<'doc> {
    let Some(number) = self.number(number) else {
      return FormulaValue::Error(FormulaErrorValue::Value);
    };
    let Some(divisor) = self.number(divisor) else {
      return FormulaValue::Error(FormulaErrorValue::Value);
    };
    match formula_mod(number, divisor) {
      Ok(result) => FormulaValue::Number(result),
      Err(error) => FormulaValue::Error(numeric_error_value(error)),
    }
  }

  pub(crate) fn evaluate_trunc(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=2).contains(&args.len()) || is_missing_argument(args.first()?) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let digits = match args.get(1) {
      Some(arg) if is_missing_argument(arg) => 0.0,
      Some(arg) => {
        let value = self.number(&self.evaluate(arg)?)?;
        if value < 0.0 {
          approx_ceil(value)
        } else {
          approx_floor(value)
        }
      }
      None => 0.0,
    };
    let value = self.number(&self.evaluate(args.first()?)?)?;
    Some(FormulaValue::Number(round_direction(
      value,
      digits.clamp(f64::from(i16::MIN), f64::from(i16::MAX)) as i32,
      false,
    )))
  }

  pub(crate) fn evaluate_roundsig(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let digits = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    if digits < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if value == 0.0 {
      return Some(FormulaValue::Number(0.0));
    }
    Some(FormulaValue::Number(round_to_significant_digits(
      value, digits,
    )))
  }

  pub(crate) fn evaluate_raw_subtract(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut result = self.number(&self.evaluate(args.first()?)?)?;
    for arg in &args[1..] {
      result -= self.number(&self.evaluate(arg)?)?;
    }
    Some(FormulaValue::Number(result))
  }

  pub(crate) fn evaluate_sumproduct(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let array_evaluator = self.with_array_context();
    let matrices = args
      .iter()
      .map(|arg| {
        array_evaluator
          .evaluate(arg)
          .map(|value| array_evaluator.matrix_values(&value))
          .or(Some(vec![]))
      })
      .collect::<Option<Vec<_>>>()?;
    let first = matrices.first()?;
    let rows = first.len();
    let columns = first.first().map_or(0, Vec::len);
    if rows == 0 || columns == 0 {
      return Some(FormulaValue::Number(0.0));
    }
    if matrices
      .iter()
      .any(|matrix| matrix.len() != rows || matrix.iter().any(|row| row.len() != columns))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut total = KahanSum::default();
    for row in 0..rows {
      for column in 0..columns {
        let mut product = SumProductScalar::Number(1.0);
        for matrix in &matrices {
          product = sumproduct_merge_scalar(product, &matrix[row][column]);
        }
        match product {
          SumProductScalar::Number(value) => total.add(value),
          SumProductScalar::Error(error) => return Some(FormulaValue::Error(error)),
          SumProductScalar::NaN => {}
        }
      }
    }
    Some(FormulaValue::Number(total.finish()))
  }

  pub(crate) fn evaluate_choose(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let index = self.number(&self.evaluate(args.first()?)?)?.floor() as usize;
    if index == 0 || index >= args.len() {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    self.evaluate(args.get(index)?)
  }

  pub(crate) fn evaluate_find(
    &self,
    args: &[FormulaAst<'doc>],
    case_sensitive: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(needle_value) = self.evaluate(&args[0]) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(haystack_value) = self.evaluate(&args[1]) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let needle = self.text(&needle_value);
    let start = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as usize;
    if self.array_context
      && (matches!(
        needle_value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      ) || matches!(
        haystack_value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      ))
    {
      return self.map_find_values(needle_value, haystack_value, start, case_sensitive, false);
    }
    Some(self.find_text_value(&needle, &haystack_value, start, case_sensitive))
  }

  pub(crate) fn find_text_value(
    &self,
    needle: &str,
    haystack_value: &FormulaValue<'doc>,
    start: usize,
    case_sensitive: bool,
  ) -> FormulaValue<'doc> {
    let haystack = self.text(haystack_value);
    match find_text_position(
      needle,
      &haystack,
      start,
      case_sensitive,
      self.book.formula_search_type,
    ) {
      Ok(position) => FormulaValue::Number(position as f64),
      Err(_) => FormulaValue::Error(FormulaErrorValue::Value),
    }
  }

  pub(crate) fn evaluate_findb(
    &self,
    args: &[FormulaAst<'doc>],
    case_sensitive: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(needle_value) = self.evaluate(&args[0]) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(haystack_value) = self.evaluate(&args[1]) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let needle = self.text(&needle_value);
    let haystack = self.text(&haystack_value);
    let start = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as usize;
    if self.array_context
      && (matches!(
        needle_value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      ) || matches!(
        haystack_value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      ))
    {
      return self.map_find_values(needle_value, haystack_value, start, case_sensitive, true);
    }
    Some(self.findb_text_value(
      &needle,
      &FormulaValue::String(Cow::Owned(haystack)),
      start,
      case_sensitive,
    ))
  }

  pub(crate) fn findb_text_value(
    &self,
    needle: &str,
    haystack_value: &FormulaValue<'doc>,
    start: usize,
    case_sensitive: bool,
  ) -> FormulaValue<'doc> {
    let haystack = self.text(haystack_value);
    match find_byte_text_position(
      needle,
      &haystack,
      start,
      case_sensitive,
      self.book.formula_search_type,
    ) {
      Ok(position) => FormulaValue::Number(position as f64),
      Err(FindTextError::NotAvailable) => FormulaValue::Error(FormulaErrorValue::NA),
      Err(FindTextError::Value) => FormulaValue::Error(FormulaErrorValue::Value),
    }
  }

  pub(crate) fn map_find_values(
    &self,
    needle_value: FormulaValue<'doc>,
    haystack_value: FormulaValue<'doc>,
    start: usize,
    case_sensitive: bool,
    bytes: bool,
  ) -> Option<FormulaValue<'doc>> {
    let needles = self.matrix_values(&needle_value);
    let haystacks = self.matrix_values(&haystack_value);
    let (needle_rows, needle_columns) = matrix_dimensions(&needles);
    let (haystack_rows, haystack_columns) = matrix_dimensions(&haystacks);
    if needle_rows == 0 || needle_columns == 0 || haystack_rows == 0 || haystack_columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let rows = needle_rows.max(haystack_rows);
    let columns = needle_columns.max(haystack_columns);
    if !matrix_can_broadcast(needle_rows, needle_columns, rows, columns)
      || !matrix_can_broadcast(haystack_rows, haystack_columns, rows, columns)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let needle_row = if needle_rows == 1 { 0 } else { row };
        let needle_column = if needle_columns == 1 { 0 } else { column };
        let haystack_row = if haystack_rows == 1 { 0 } else { row };
        let haystack_column = if haystack_columns == 1 { 0 } else { column };
        let needle = needles.get(needle_row)?.get(needle_column)?;
        let haystack = haystacks.get(haystack_row)?.get(haystack_column)?;
        let needle_text = self.text(needle);
        result_row.push(if bytes {
          self.findb_text_value(&needle_text, haystack, start, case_sensitive)
        } else {
          self.find_text_value(&needle_text, haystack, start, case_sensitive)
        });
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_substitute(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(text_value) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(old_value) = args.get(1).and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(new_value) = args.get(2).and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let text = self.text(&text_value);
    let old = self.text(&old_value);
    let new = self.text(&new_value);
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

  pub(crate) fn evaluate_replace(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 4 {
      return None;
    }
    let text = self.text(&self.evaluate(args.first()?)?);
    let start = self.number(&self.evaluate(args.get(1)?)?)? as usize;
    let count = self.number(&self.evaluate(args.get(2)?)?)? as usize;
    let new_text = self.text(&self.evaluate(args.get(3)?)?);
    if start == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut chars = text.chars().collect::<Vec<_>>();
    let index = (start - 1).min(chars.len());
    let end = (index + count).min(chars.len());
    chars.splice(index..end, new_text.chars());
    Some(FormulaValue::String(Cow::Owned(
      chars.into_iter().collect(),
    )))
  }

  pub(crate) fn evaluate_text_before_after(
    &self,
    args: &[FormulaAst<'doc>],
    after: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(text_value) = self.evaluate(&args[0]) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(delimiter_value) = self.evaluate(&args[1]) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let text = self.text(&text_value);
    if text.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let delimiters = self.textsplit_delimiters(&delimiter_value);
    let mut instance = self.optional_number_arg(args, 2, 1.0) as i32;
    if instance == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    }
    let match_mode = self.optional_number_arg(args, 3, 0.0) != 0.0;
    let match_end = self.optional_number_arg(args, 4, 0.0) != 0.0;
    let if_not_found = args
      .get(5)
      .and_then(|arg| self.evaluate(arg))
      .filter(|value| !matches!(value, FormulaValue::Blank))
      .map(|value| self.text(&value));
    let mut positions = Vec::new();
    if match_end && after {
      positions.push(0usize);
    }
    let search_text = if match_mode {
      text.to_lowercase()
    } else {
      text.clone()
    };
    let search_delimiters = delimiters
      .iter()
      .map(|delimiter| {
        if match_mode {
          delimiter.to_lowercase()
        } else {
          delimiter.clone()
        }
      })
      .collect::<Vec<_>>();
    let mut start = 0usize;
    while start < search_text.len() {
      let mut found = None::<(usize, usize)>;
      for delimiter in &search_delimiters {
        if delimiter.is_empty() {
          continue;
        }
        if let Some(offset) = search_text[start..].find(delimiter) {
          let index = start + offset;
          if found.is_none_or(|(best, _)| index < best) {
            found = Some((index, delimiter.len()));
          }
        }
      }
      let Some((index, delimiter_len)) = found else {
        break;
      };
      positions.push(if after { index + delimiter_len } else { index });
      start = index + delimiter_len;
    }
    if match_end && !after {
      positions.push(text.len());
    }
    if positions.is_empty() || instance.unsigned_abs() as usize > positions.len() {
      return Some(if let Some(value) = if_not_found {
        FormulaValue::String(Cow::Owned(value))
      } else {
        FormulaValue::Error(FormulaErrorValue::NA)
      });
    }
    if instance < 0 {
      instance = positions.len() as i32 + instance + 1;
    };
    let position = positions[instance as usize - 1];
    Some(FormulaValue::String(Cow::Owned(if after {
      text[position..].to_string()
    } else {
      text[..position].to_string()
    })))
  }

  pub(crate) fn evaluate_textsplit(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.is_empty() || args.len() > 6 {
      return None;
    }
    let text = self.text(&self.evaluate(args.first()?)?);
    if text.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let column_delimiters = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.textsplit_delimiters(&value))
      .unwrap_or_default();
    let row_delimiters = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.textsplit_delimiters(&value))
      .unwrap_or_default();
    let ignore_empty = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    let match_mode = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    let pad_with = args.get(5).and_then(|arg| self.evaluate(arg));
    let pad_with = pad_with.as_ref().map(|value| self.text(value));
    let row_texts = split_text_multi(&text, &row_delimiters, ignore_empty, match_mode);
    let mut result_rows = Vec::with_capacity(row_texts.len());
    let mut columns = 1usize;
    for row in row_texts {
      let values = split_text_multi(&row, &column_delimiters, ignore_empty, match_mode);
      columns = columns.max(values.len());
      result_rows.push(values);
    }
    Some(FormulaValue::Matrix(
      result_rows
        .into_iter()
        .map(|row| {
          (0..columns)
            .map(|column| {
              row
                .get(column)
                .map(|value| FormulaValue::String(Cow::Owned(value.clone())))
                .unwrap_or_else(|| {
                  pad_with
                    .as_ref()
                    .map(|value| FormulaValue::String(Cow::Owned(value.clone())))
                    .unwrap_or(FormulaValue::Error(FormulaErrorValue::NA))
                })
            })
            .collect()
        })
        .collect(),
    ))
  }

  pub(crate) fn textsplit_delimiters(&self, value: &FormulaValue<'doc>) -> Vec<String> {
    self
      .matrix_values(value)
      .into_iter()
      .flatten()
      .map(|value| self.text(&value))
      .collect()
  }

  pub(crate) fn evaluate_textjoin(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 {
      return None;
    }
    let mut delimiters = self
      .values(&args[..1])
      .map(|value| self.text(&value))
      .collect::<Vec<_>>();
    if delimiters.is_empty() {
      delimiters.push(String::new());
    }
    let ignore_empty = self.truthy(&self.evaluate(args.get(1)?)?);
    let mut output = String::new();
    let mut count = 0usize;
    for value in self.values(&args[2..]) {
      if ignore_empty && matches!(self.first_value(&value), FormulaValue::Blank) {
        continue;
      }
      let text = self.text(&value);
      if ignore_empty && text.is_empty() {
        continue;
      }
      if count > 0 {
        output.push_str(&delimiters[(count - 1) % delimiters.len()]);
      }
      output.push_str(&text);
      count += 1;
    }
    Some(FormulaValue::String(Cow::Owned(output)))
  }

  pub(crate) fn evaluate_width_conversion(
    &self,
    args: &[FormulaAst<'doc>],
    full_width: bool,
  ) -> Option<FormulaValue<'doc>> {
    let text = self.text(&self.evaluate(args.first()?)?);
    Some(FormulaValue::String(Cow::Owned(if full_width {
      full_width_like_jis(&text)
    } else {
      half_width_like_asc(&text)
    })))
  }

  pub(crate) fn evaluate_row_column(
    &self,
    args: &[FormulaAst<'doc>],
    column: bool,
  ) -> Option<FormulaValue<'doc>> {
    let reference = if let Some(arg) = args.first() {
      let Some(value) = self.evaluate(arg) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      let Some(reference) = self.as_reference(&value) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      Some(reference)
    } else {
      None
    };
    let address = reference
      .as_ref()
      .map(|reference| reference.range.start)
      .unwrap_or_else(|| self.current_cell.unwrap_or_default());
    if let Some(reference) = reference {
      let range = reference.range;
      let start_column = range.start.column.min(range.end.column);
      let end_column = range.start.column.max(range.end.column);
      let start_row = range.start.row.min(range.end.row);
      let end_row = range.start.row.max(range.end.row);
      if column && end_column > start_column {
        return Some(FormulaValue::Matrix(vec![
          (start_column..=end_column)
            .map(|column| FormulaValue::Number(column as f64 + 1.0))
            .collect(),
        ]));
      }
      if !column && end_row > start_row {
        return Some(FormulaValue::Matrix(
          (start_row..=end_row)
            .map(|row| vec![FormulaValue::Number(row as f64 + 1.0)])
            .collect(),
        ));
      }
    }
    Some(FormulaValue::Number(if column {
      address.column as f64 + 1.0
    } else {
      address.row as f64 + 1.0
    }))
  }

  pub(crate) fn evaluate_rows_columns(
    &self,
    args: &[FormulaAst<'doc>],
    columns: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.is_empty() {
      return Some(FormulaValue::Number(0.0));
    }
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

  pub(crate) fn evaluate_is_formula(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let reference = self.as_reference(&self.evaluate(args.first()?)?)?;
    if !self.array_context && reference.range.cell_count_hint() != 1 {
      return Some(FormulaValue::Boolean(false));
    }
    let sheet = self.range_sheet(&reference);
    Some(FormulaValue::Boolean(
      self
        .book
        .formulas
        .contains_key(&(sheet, reference.range.start)),
    ))
  }

  pub(crate) fn evaluate_error_type(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(FormulaValue::Error(error)) = self.first_error_value(&value) else {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    };
    let code = match error {
      FormulaErrorValue::Null => 1.0,
      FormulaErrorValue::Div0 => 2.0,
      FormulaErrorValue::Value => 3.0,
      FormulaErrorValue::Ref => 4.0,
      FormulaErrorValue::Name => 5.0,
      FormulaErrorValue::Num => 6.0,
      FormulaErrorValue::NA => 7.0,
      FormulaErrorValue::GettingData
      | FormulaErrorValue::Spill
      | FormulaErrorValue::Calc
      | FormulaErrorValue::IllegalArgument
      | FormulaErrorValue::Parameter => return Some(FormulaValue::Error(FormulaErrorValue::NA)),
      FormulaErrorValue::Unknown => return Some(FormulaValue::Error(FormulaErrorValue::Unknown)),
    };
    Some(FormulaValue::Number(code))
  }

  pub(crate) fn evaluate_error_type_raw(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let direct_unknown_error = matches!(
      args.first(),
      Some(FormulaAst::Literal(FormulaValue::Error(
        FormulaErrorValue::Unknown
      )))
    );
    let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if let FormulaValue::Reference(reference) = &value
      && reference.range.cell_count_hint() == 1
      && self.first_error_value(&value).is_none()
    {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    }
    if matches!(
      value,
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
    ) && self.first_error_value(&value).is_none()
    {
      return Some(FormulaValue::Number(519.0));
    }
    let Some(FormulaValue::Error(error)) = self.first_error_value(&value) else {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    };
    if error == FormulaErrorValue::Unknown && direct_unknown_error {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    Some(FormulaValue::Number(match error {
      FormulaErrorValue::Null => 521.0,
      FormulaErrorValue::Div0 => 532.0,
      FormulaErrorValue::Value => 519.0,
      FormulaErrorValue::Ref => 524.0,
      FormulaErrorValue::Name => 525.0,
      FormulaErrorValue::Num => 503.0,
      FormulaErrorValue::NA => 32767.0,
      FormulaErrorValue::Spill => 541.0,
      FormulaErrorValue::IllegalArgument => 502.0,
      FormulaErrorValue::Parameter => 511.0,
      FormulaErrorValue::Unknown => 508.0,
      FormulaErrorValue::GettingData | FormulaErrorValue::Calc => 515.0,
    }))
  }

  pub(crate) fn evaluate_info(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let key = self
      .text(&self.evaluate(args.first()?)?)
      .to_ascii_uppercase();
    match key.as_str() {
      "SYSTEM" => Some(FormulaValue::String(Cow::Borrowed("LINUX"))),
      "OSVERSION" | "RELEASE" => Some(FormulaValue::String(Cow::Borrowed(""))),
      "NUMFILE" => Some(FormulaValue::Number(1.0)),
      "RECALC" => Some(FormulaValue::String(Cow::Borrowed("Automatic"))),
      "DIRECTORY" | "MEMAVAIL" | "MEMUSED" | "ORIGIN" | "TOTMEM" => {
        Some(FormulaValue::Error(FormulaErrorValue::NA))
      }
      _ => Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    }
  }

  pub(crate) fn evaluate_type(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if matches!(value, FormulaValue::Matrix(_) | FormulaValue::RefList(_)) {
      return Some(FormulaValue::Number(64.0));
    }
    Some(FormulaValue::Number(match self.first_value(&value) {
      FormulaValue::Number(_) => 1.0,
      FormulaValue::String(_) => 2.0,
      FormulaValue::Boolean(_) => 1.0,
      FormulaValue::Error(_) => 16.0,
      FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => 64.0,
      FormulaValue::Blank => 1.0,
    }))
  }

  pub(crate) fn evaluate_areas(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(value) = self.evaluate(args.first()?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let ranges = self.reference_ranges_from_value(&value);
    if !ranges.is_empty() {
      return Some(FormulaValue::Number(ranges.len() as f64));
    }
    Some(match value {
      FormulaValue::Matrix(_) => FormulaValue::Number(1.0),
      _ => FormulaValue::Error(FormulaErrorValue::Value),
    })
  }

  pub(crate) fn evaluate_ifs_function(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 || !args.len().is_multiple_of(2) {
      return None;
    }
    for pair in args.chunks_exact(2) {
      let condition = self.evaluate(&pair[0])?;
      if let FormulaValue::Error(error) = condition {
        return Some(FormulaValue::Error(error));
      }
      if self.truthy(&condition) {
        return self.evaluate(&pair[1]);
      }
    }
    Some(FormulaValue::Error(FormulaErrorValue::NA))
  }

  pub(crate) fn evaluate_switch(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 {
      return None;
    }
    let selector = self.scalar_value(self.evaluate(args.first()?)?);
    if let FormulaValue::Error(error) = &selector {
      return Some(FormulaValue::Error(*error));
    }
    let pairs_len = if args.len().is_multiple_of(2) {
      args.len() - 2
    } else {
      args.len() - 1
    };
    for pair in args[1..=pairs_len].chunks_exact(2) {
      let candidate = self.scalar_value(self.evaluate(&pair[0])?);
      if let FormulaValue::Error(error) = &candidate {
        return Some(FormulaValue::Error(*error));
      }
      let matches = match (&selector, &candidate) {
        (FormulaValue::String(left), FormulaValue::String(right)) => {
          left.eq_ignore_ascii_case(right)
        }
        _ => self.compare(&selector, &candidate, FormulaOperator::Equal),
      };
      if matches {
        return Some(self.scalar_value(self.evaluate(&pair[1])?));
      }
    }
    if args.len().is_multiple_of(2) {
      Some(self.scalar_value(self.evaluate(args.last()?)?))
    } else {
      Some(FormulaValue::Error(FormulaErrorValue::NA))
    }
  }

  pub(crate) fn evaluate_date(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 || args.first().is_some_and(is_missing_argument) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut year = self.number(&self.evaluate(args.first()?)?)? as i32;
    let month = self.number(&self.evaluate(args.get(1)?)?)? as i32;
    let day = self.number(&self.evaluate(args.get(2)?)?)? as i32;
    if year < 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    if year < 100 {
      year = expand_two_digit_year(year);
    }
    let (normalized_year, normalized_month, normalized_day) =
      normalized_date_components(year, month, day)?;
    if !is_valid_libreoffice_gregorian_date(normalized_year, normalized_month, normalized_day) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    date_serial_with_system(year, month, day, self.book.date_system).map(FormulaValue::Number)
  }

  pub(crate) fn evaluate_address(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=5).contains(&args.len()) {
      return None;
    }
    let row = self.number(&self.evaluate(args.first()?)?)? as i32;
    let column = self.number(&self.evaluate(args.get(1)?)?)? as i32;
    if row <= 0
      || column <= 0
      || row as u32 > XLSX_MAX_ROW_ZERO_BASED + 1
      || column as u32 > XLSX_MAX_COLUMN_ZERO_BASED + 1
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let abs_num = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| {
        if matches!(value, FormulaValue::Blank) {
          Some(1.0)
        } else {
          self.number(&value)
        }
      })
      .unwrap_or(1.0) as i32;
    let a1 = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| {
        if matches!(value, FormulaValue::Blank) {
          true
        } else {
          self.truthy(&value)
        }
      })
      .unwrap_or(true);
    let sheet = args.get(4).and_then(|arg| self.evaluate(arg)).map(|value| {
      let sheet = self.text(&value);
      if sheet.is_empty() {
        String::new()
      } else if a1 {
        let separator = match self.grammar {
          FormulaGrammar::OpenFormula | FormulaGrammar::CalcA1 => ".",
          FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1 => "!",
        };
        format!(
          "{}{}",
          quote_sheet_name_for_reference(sheet.as_ref()),
          separator
        )
      } else {
        format!("{}!", quote_sheet_name_for_reference(sheet.as_ref()))
      }
    });
    let reference = if a1 {
      let (abs_col, abs_row) = match abs_num {
        1 | 5 => (true, true),
        2 | 6 => (false, true),
        3 | 7 => (true, false),
        4 | 8 => (false, false),
        _ => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
      };
      format!(
        "{}{}{}{}",
        if abs_col { "$" } else { "" },
        column_index_to_name((column - 1) as u32),
        if abs_row { "$" } else { "" },
        row
      )
    } else {
      match abs_num {
        1 | 5 => format!("R{row}C{column}"),
        2 | 6 => format!("R{row}C[{column}]"),
        3 | 7 => format!("R[{row}]C{column}"),
        4 | 8 => format!("R[{row}]C[{column}]"),
        _ => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
      }
    };
    Some(FormulaValue::String(Cow::Owned(format!(
      "{}{reference}",
      sheet.unwrap_or_default()
    ))))
  }

  pub(crate) fn evaluate_time(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  pub(crate) fn evaluate_date_part(
    &self,
    args: &[FormulaAst<'doc>],
    part: DatePart,
  ) -> Option<FormulaValue<'doc>> {
    if self.array_context {
      let value = self.evaluate(args.first()?)?;
      if matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
        return Some(FormulaValue::Matrix(
          self
            .matrix_values(&value)
            .into_iter()
            .map(|row| {
              row
                .into_iter()
                .map(|value| match self.date_number_from_scalar(&value) {
                  Some(serial) => {
                    let serial = serial.floor() as i32;
                    match date_from_serial_with_system(serial, self.book.date_system) {
                      Some((year, month, day)) => FormulaValue::Number(match part {
                        DatePart::Year => year as f64,
                        DatePart::Month => month as f64,
                        DatePart::Day => day as f64,
                      }),
                      None => FormulaValue::Error(FormulaErrorValue::Value),
                    }
                  }
                  None => FormulaValue::Error(FormulaErrorValue::Value),
                })
                .collect()
            })
            .collect(),
        ));
      }
    }
    let Some(serial) = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.date_number_from_value(&value))
      .map(|value| value.floor() as i32)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let (year, month, day) = date_from_serial_with_system(serial, self.book.date_system)?;
    Some(FormulaValue::Number(match part {
      DatePart::Year => year as f64,
      DatePart::Month => month as f64,
      DatePart::Day => day as f64,
    }))
  }

  pub(crate) fn evaluate_today(&self) -> Option<FormulaValue<'doc>> {
    if let Some(value) = self.book.today_serial {
      return Some(FormulaValue::Number(value.floor()));
    }
    let unix_days = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .ok()
      .map(|duration| duration.as_secs() / 86_400)?;
    Some(FormulaValue::Number(
      date_serial_with_system(1970, 1, 1, self.book.date_system)? + unix_days as f64,
    ))
  }

  pub(crate) fn evaluate_days360(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let mut start = self
      .date_number_from_value(&self.evaluate(args.first()?)?)?
      .floor() as i32;
    let mut end = self
      .date_number_from_value(&self.evaluate(args.get(1)?)?)?
      .floor() as i32;
    let european = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .is_some_and(|value| value != 0.0);
    let mut sign = 1;
    if european && end < start {
      std::mem::swap(&mut start, &mut end);
      sign = -1;
    }
    days360(start, end, european)
      .map(|value| FormulaValue::Number(f64::from(sign * value)))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_eomonth(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(start) = self.date_number_from_value(&self.evaluate(args.first()?)?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(months) = self.number(&self.evaluate(args.get(1)?)?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let start = start.floor() as i32;
    let months = months.floor() as i32;
    let (year, month, _) = date_from_serial_with_system(start, self.book.date_system)?;
    date_serial_with_system(year, month as i32 + months + 1, 0, self.book.date_system)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_edate(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let start = self
      .date_number_from_value(&self.evaluate(args.first()?)?)?
      .floor() as i32;
    let months = self.number(&self.evaluate(args.get(1)?)?)?.floor() as i32;
    let (year, month, day) = date_from_serial_with_system(start, self.book.date_system)?;
    let (target_year, target_month, _) =
      normalized_date_components(year, month as i32 + months, 1)?;
    let target_day = day.min(last_day_of_month(target_year, target_month));
    let target = date_serial_with_system(
      target_year,
      target_month as i32,
      target_day as i32,
      self.book.date_system,
    );
    target
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_is_leap_year(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    if is_multicell_scalar_argument(&value) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let Some(serial) = self.date_number_from_value(&value) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let serial = serial as i32;
    let (year, _, _) = date_from_serial_with_system(serial, self.book.date_system)?;
    Some(FormulaValue::Boolean(is_leap_year(year)))
  }

  pub(crate) fn evaluate_basis_o_datetime(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return None;
    }
    basis_o_datetime_text(self.number(&self.evaluate(args.first()?)?)?)
      .map(|value| FormulaValue::String(Cow::Owned(value)))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_datedif(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let start = self.number(&self.evaluate(args.first()?)?)?.floor() as i32;
    let end = self.number(&self.evaluate(args.get(1)?)?)?.floor() as i32;
    let interval = self
      .text(&self.evaluate(args.get(2)?)?)
      .to_ascii_lowercase();
    if start > end {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let days = f64::from(end - start);
    if days == 0.0 || interval == "d" {
      return Some(FormulaValue::Number(days));
    }
    let (start_year, start_month, start_day) =
      date_from_serial_with_system(start, self.book.date_system)?;
    let (end_year, end_month, end_day) = date_from_serial_with_system(end, self.book.date_system)?;
    let month_diff = end_month as i32 - start_month as i32 + 12 * (end_year - start_year)
      - i32::from(start_day > end_day);
    match interval.as_str() {
      "m" => Some(FormulaValue::Number(month_diff as f64)),
      "y" => Some(FormulaValue::Number(if end_year > start_year
        && (end_month > start_month || (end_month == start_month && end_day >= start_day))
      {
        end_year - start_year
      } else if end_year > start_year {
        end_year - start_year - 1
      } else {
        0
      } as f64)),
      "md" => {
        if start_day <= end_day {
          return Some(FormulaValue::Number((end_day - start_day) as f64));
        }
        let (roll_year, roll_month) = if end_month == 1 {
          (end_year - 1, 12)
        } else {
          (end_year, end_month as i32 - 1)
        };
        let roll = date_serial_with_system(
          roll_year,
          roll_month,
          start_day as i32,
          self.book.date_system,
        )?;
        Some(FormulaValue::Number(f64::from(end) - roll))
      }
      "ym" => Some(FormulaValue::Number(month_diff.rem_euclid(12) as f64)),
      "yd" => {
        let same_year_start =
          if end_month > start_month || (end_month == start_month && end_day >= start_day) {
            date_serial_with_system(
              end_year,
              start_month as i32,
              start_day as i32,
              self.book.date_system,
            )?
          } else {
            date_serial_with_system(
              end_year - 1,
              start_month as i32,
              start_day as i32,
              self.book.date_system,
            )?
          };
        Some(FormulaValue::Number(f64::from(end) - same_year_start))
      }
      _ => Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    }
  }

  pub(crate) fn evaluate_yearfrac(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let start = self.date_number_from_value(&self.evaluate(args.first()?)?)? as i32;
    let end = self.date_number_from_value(&self.evaluate(args.get(1)?)?)? as i32;
    let basis = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0) as i32;
    yearfrac(start, end, basis)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_weeknum(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=2).contains(&args.len()) {
      return None;
    }
    let serial = self.date_number_from_value(&self.evaluate(args.first()?)?)? as i64;
    let mode = match args.get(1).and_then(|arg| self.evaluate(arg)) {
      Some(FormulaValue::Reference(_) | FormulaValue::Matrix(_) | FormulaValue::RefList(_)) => {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      Some(FormulaValue::Blank) | None => 1.0,
      Some(value) => self.number(&value)?,
    } as i32;
    let weekday = weekday_index_from_serial(serial) as i32;
    let year = date_from_serial_with_system(serial as i32, self.book.date_system)?.0;
    let jan1 = date_serial_with_system(year, 1, 1, self.book.date_system)? as i64;
    let jan1_weekday = weekday_index_from_serial(jan1) as i32;
    if matches!(mode, 21 | 150) {
      return iso_weeknum_from_serial_with_system(serial as i32, self.book.date_system)
        .map(|value| FormulaValue::Number(value as f64))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Value)));
    }
    let week_start = match mode {
      1 | 17 => 6,
      2 | 11 => 0,
      12 => 1,
      13 => 2,
      14 => 3,
      15 => 4,
      16 => 5,
      _ => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    };
    let offset = (jan1_weekday - week_start).rem_euclid(7);
    let week = ((serial - jan1 + offset as i64) / 7) + 1;
    let _ = weekday;
    Some(FormulaValue::Number(week as f64))
  }

  pub(crate) fn evaluate_iso_weeknum(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let value = self.evaluate(args.first()?)?;
    if is_multicell_scalar_argument(&value) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let serial = match self.date_number_from_value(&value) {
      Some(value) => value as i32,
      None => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    };
    iso_weeknum_from_serial_with_system(serial, self.book.date_system)
      .map(|value| FormulaValue::Number(value as f64))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_weeks(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    // Source: LibreOffice scaddins/source/datefunc/datefunc.cxx
    // ScaDateAddIn::getDiffWeeks.
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let start = match self.date_number_from_value(&self.evaluate(args.first()?)?) {
      Some(value) => value as i32,
      None => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    };
    let end = match self.date_number_from_value(&self.evaluate(args.get(1)?)?) {
      Some(value) => value as i32,
      None => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    };
    let mode_arg = self.evaluate(args.get(2)?)?;
    if matches!(mode_arg, FormulaValue::Blank) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mode = match self.number(&mode_arg) {
      Some(value) => value as i32,
      None => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    };
    match mode {
      0 => Some(FormulaValue::Number(((end - start) / 7) as f64)),
      1 => {
        let Some(start_week) = weeks_mode_one_index(start, self.book.date_system) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        let Some(end_week) = weeks_mode_one_index(end, self.book.date_system) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        Some(FormulaValue::Number((end_week - start_week) as f64))
      }
      _ => Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    }
  }

  pub(crate) fn evaluate_weeks_in_year(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let serial = match self.date_number_from_value(&self.evaluate(args.first()?)?) {
      Some(value) => value as i32,
      None => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    };
    weeks_in_year_from_serial_with_system(serial, self.book.date_system)
      .map(|value| FormulaValue::Number(value as f64))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_years_months(
    &self,
    args: &[FormulaAst<'doc>],
    years: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let start = self.date_number_from_value(&self.evaluate(args.first()?)?)? as i32;
    let end = self.date_number_from_value(&self.evaluate(args.get(1)?)?)? as i32;
    let mode_arg = self.evaluate(args.get(2)?)?;
    if matches!(mode_arg, FormulaValue::Blank) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mode = self.number(&mode_arg)? as i32;
    if !matches!(mode, 0 | 1) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let result = if years {
      date_diff_years(start, end, mode)?
    } else {
      date_diff_months(start, end, mode)?
    };
    Some(FormulaValue::Number(result as f64))
  }

  pub(crate) fn evaluate_days_in_month_year(
    &self,
    args: &[FormulaAst<'doc>],
    year: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let Some(serial) = self.date_number_from_value(&self.evaluate(args.first()?)?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let serial = serial as i32;
    let (year_value, month, _) = date_from_serial(serial)?;
    Some(FormulaValue::Number(if year {
      if is_leap_year(year_value) {
        366.0
      } else {
        365.0
      }
    } else {
      last_day_of_month(year_value, month) as f64
    }))
  }

  pub(crate) fn evaluate_weekday(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let serial = self
      .date_number_from_value(&self.evaluate(args.first()?)?)?
      .floor() as i64;
    let flag = match args.get(1).and_then(|arg| self.evaluate(arg)) {
      Some(FormulaValue::Reference(_) | FormulaValue::Matrix(_) | FormulaValue::RefList(_)) => {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      Some(value) => self.number(&value).unwrap_or(1.0),
      None => 1.0,
    } as i32;
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

  pub(crate) fn evaluate_time_part(
    &self,
    args: &[FormulaAst<'doc>],
    part: TimePart,
  ) -> Option<FormulaValue<'doc>> {
    let value = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.time_number_from_value(&value))
      .unwrap_or_default();
    let total_seconds = (value.fract() * 86_400.0).floor() as i64;
    Some(FormulaValue::Number(match part {
      TimePart::Hour => total_seconds.rem_euclid(86_400) / 3600,
      TimePart::Minute => total_seconds.rem_euclid(3_600) / 60,
      TimePart::Second => ((value.fract() * 86_400.0).round() as i64).rem_euclid(60),
    } as f64))
  }

  pub(crate) fn time_number_from_value(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    match self.first_value(value) {
      FormulaValue::String(text) => match timevalue(&text) {
        FormulaValue::Number(value) => Some(value),
        _ => None,
      },
      value => self.number(&value),
    }
  }

  pub(crate) fn evaluate_indirect(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(value) = self.evaluate(args.first()?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let text = self.text(&value);
    let use_a1 = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    let reference_text;
    let text = if use_a1 {
      text.as_str()
    } else {
      let Some(converted) =
        crate::parser::r1c1_reference_to_a1(&text, self.current_cell.unwrap_or_default())
      else {
        return Some(FormulaValue::Error(FormulaErrorValue::Ref));
      };
      reference_text = converted;
      reference_text.as_str()
    };
    self
      .resolve_reference(text)
      .map(FormulaValue::Reference)
      .or_else(|| self.evaluate_defined_name(&Cow::Owned(text.to_string())))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Ref)))
  }

  pub(crate) fn evaluate_index(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=4).contains(&args.len()) {
      return None;
    }
    let row = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let column = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let area = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0);
    if row < 0.0 || column < 0.0 || area < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let row = row as u32;
    let column = column as u32;
    let area = area as usize;
    let ranges = self.reference_ranges_from_ast(args.first()?);
    if !ranges.is_empty() {
      return Some(self.index_reference_area(&ranges, row, column, area, args.len()));
    }
    let Some(value) = self.evaluate(args.first()?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    if let FormulaValue::Matrix(rows) = value {
      return Some(index_matrix(rows, row, column, args.len()));
    }
    let Some(reference) = self.as_reference(&value) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    Some(self.index_reference_area(&[reference], row, column, area, args.len()))
  }

  pub(crate) fn index_reference_area(
    &self,
    ranges: &[QualifiedRange<'doc>],
    row: u32,
    column: u32,
    area: usize,
    arg_count: usize,
  ) -> FormulaValue<'doc> {
    let Some(reference) = ranges.get(area - 1) else {
      return FormulaValue::Error(FormulaErrorValue::Ref);
    };
    let start_column = reference.range.start.column.min(reference.range.end.column);
    let end_column = reference.range.start.column.max(reference.range.end.column);
    let start_row = reference.range.start.row.min(reference.range.end.row);
    let end_row = reference.range.start.row.max(reference.range.end.row);
    let b_row_array = arg_count == 2 && start_row == end_row;
    if (column > 0 && start_column + column - 1 > end_column)
      || (row > 0 && start_row + row - 1 > end_row && !b_row_array)
      || (b_row_array && row > end_column - start_column + 1)
    {
      return FormulaValue::Error(FormulaErrorValue::Ref);
    }
    if row == 0 && column == 0 {
      return FormulaValue::Reference(reference.clone());
    }
    let range = if row == 0 {
      let selected_column = start_column + column - 1;
      CellRange::new(
        CellAddress {
          column: selected_column,
          row: start_row,
        },
        CellAddress {
          column: selected_column,
          row: end_row,
        },
      )
    } else if column == 0 {
      if b_row_array {
        let selected_column = start_column + row - 1;
        CellRange::new(
          CellAddress {
            column: selected_column,
            row: start_row,
          },
          CellAddress {
            column: selected_column,
            row: start_row,
          },
        )
      } else {
        let selected_row = start_row + row - 1;
        CellRange::new(
          CellAddress {
            column: start_column,
            row: selected_row,
          },
          CellAddress {
            column: end_column,
            row: selected_row,
          },
        )
      }
    } else {
      CellRange::new(
        CellAddress {
          column: start_column + column - 1,
          row: start_row + row - 1,
        },
        CellAddress {
          column: start_column + column - 1,
          row: start_row + row - 1,
        },
      )
    };
    FormulaValue::Reference(QualifiedRange {
      sheet: reference.sheet,
      sheet_name: reference.sheet_name.clone(),
      range,
      start_flags: reference.start_flags,
      end_flags: reference.end_flags,
    })
  }

  pub(crate) fn evaluate_offset(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  pub(crate) fn evaluate_lookup(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let lookup = self.scalar_binary_operand(self.evaluate(args.first()?)?);
    if matches!(lookup, FormulaValue::Blank) {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    }
    let array_evaluator = self.with_array_context();
    let data = array_evaluator.evaluate(args.get(1)?)?;
    let data_matrix = self.matrix_values(&data);
    let result = if let Some(arg) = args.get(2) {
      Some(array_evaluator.evaluate(arg)?)
    } else {
      None
    };
    let result_matrix = result.as_ref().map(|value| self.matrix_values(value));
    let (data_vector, data_vertical) = lookup_vector(&data_matrix)?;
    let query = QueryEntry {
      op: QueryOp::LessOrEqual,
      field: 0,
      item: QueryItem {
        kind: query_value_kind(&lookup),
        value: lookup.clone(),
        source_text: None,
        match_empty: false,
        empty_matches_text: false,
      },
    };
    let param = QueryParam::single(query, QuerySearchType::Normal, true).with_range_lookup(true);
    let query = param.entries.first()?;
    let (search_vector, index_map) = lookup_search_vector_omitting_errors(&data_vector);
    let search_slice = search_vector.as_deref().unwrap_or(&data_vector);
    let Some(search_index) =
      lookup_binary_search(self, search_slice, query, &param, true, false, false)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    };
    let index = index_map
      .as_ref()
      .and_then(|indices| indices.get(search_index).copied())
      .unwrap_or(search_index);

    if let Some(FormulaValue::Reference(reference)) = result.as_ref() {
      let start_row = reference.range.start.row.min(reference.range.end.row);
      let end_row = reference.range.start.row.max(reference.range.end.row);
      let start_column = reference.range.start.column.min(reference.range.end.column);
      let end_column = reference.range.start.column.max(reference.range.end.column);
      if start_row != end_row && start_column != end_column {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      }
      let address = if start_row != end_row {
        let row = start_row.saturating_add(index as u32);
        if row > XLSX_MAX_ROW_ZERO_BASED {
          return Some(FormulaValue::Number(0.0));
        }
        CellAddress {
          column: start_column,
          row,
        }
      } else {
        let column = start_column.saturating_add(index as u32);
        if column > XLSX_MAX_COLUMN_ZERO_BASED {
          return Some(FormulaValue::Number(0.0));
        }
        CellAddress {
          column,
          row: start_row,
        }
      };
      return Some(self.book.cell_value(self.range_sheet(reference), address));
    }

    if let Some(result_matrix) = result_matrix {
      let rows = result_matrix.len();
      let columns = result_matrix.first().map_or(0, Vec::len);
      if rows == 1 && columns == 1 {
        if args
          .get(2)
          .is_some_and(|arg| matches!(arg, FormulaAst::Array(_)))
          && index != 0
        {
          return Some(FormulaValue::Error(FormulaErrorValue::NA));
        }
        return result_matrix
          .first()
          .and_then(|row| row.first())
          .cloned()
          .or(Some(FormulaValue::Error(FormulaErrorValue::NA)));
      }
      if rows > 1 && columns > 1 {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      }
      let result_vertical = result_matrix.len() >= result_matrix.first().map_or(0, Vec::len);
      let result_vector = lookup_vector_with_orientation(&result_matrix, result_vertical)?;
      return result_vector
        .get(index)
        .cloned()
        .or(Some(FormulaValue::Error(FormulaErrorValue::NA)));
    }

    if data_vertical {
      data_matrix
        .get(index)
        .and_then(|row| row.last())
        .cloned()
        .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
    } else {
      data_matrix
        .last()
        .and_then(|row| row.get(index))
        .cloned()
        .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
    }
  }

  pub(crate) fn evaluate_match(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let lookup = self.first_value(&self.evaluate(args.first()?)?);
    let data = self.evaluate(args.get(1)?)?;
    let (vector, index_map, data_vertical) = match &data {
      FormulaValue::Reference(reference) => self.lookup_reference_vector(reference)?,
      _ => {
        let matrix = self.matrix_values(&data);
        let (vector, data_vertical) = lookup_vector(&matrix)?;
        (vector, None, data_vertical)
      }
    };
    let mode = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as i32;
    let index = if index_map.is_some() && !data_vertical && matches!(mode, 1 | -1) {
      match_range_linear_index(self, &lookup, &vector, mode)
    } else {
      match mode {
        0 => search_vector(
          self,
          &lookup,
          &vector,
          QueryOp::Equal,
          LookupSearchMode::Forward,
          true,
        ),
        1 => search_vector(
          self,
          &lookup,
          &vector,
          QueryOp::LessOrEqual,
          LookupSearchMode::BinaryAscending,
          false,
        ),
        -1 => search_vector(
          self,
          &lookup,
          &vector,
          QueryOp::GreaterOrEqual,
          LookupSearchMode::BinaryDescending,
          false,
        ),
        _ => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
      }
    };
    let index = index.map(|index| {
      index_map
        .as_ref()
        .and_then(|indices| indices.get(index).copied())
        .unwrap_or(index)
    });
    index
      .map(|index| FormulaValue::Number(index as f64 + 1.0))
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  pub(crate) fn evaluate_xmatch(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=4).contains(&args.len()) {
      return None;
    }
    let lookup = self.scalar_value(self.evaluate(args.first()?)?);
    let data = self.with_array_context().evaluate(args.get(1)?)?;
    let matrix = self.matrix_values(&data);
    let (vector, _) = lookup_vector(&matrix)?;
    let match_mode = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0) as i32;
    let search_mode = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as i32;
    let search = LookupSearchMode::from_excel(search_mode)?;
    if matches!(match_mode, 2 | 3)
      && matches!(
        search,
        LookupSearchMode::BinaryAscending | LookupSearchMode::BinaryDescending
      )
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let index = match match_mode {
      0 => search_vector(self, &lookup, &vector, QueryOp::Equal, search, true),
      -1 => search_vector_with_type(
        self,
        &lookup,
        &vector,
        QueryOp::LessOrEqual,
        search,
        QuerySearchType::Normal,
        SearchVectorFlags::new(false, true),
      ),
      1 => search_vector_with_type(
        self,
        &lookup,
        &vector,
        QueryOp::GreaterOrEqual,
        search,
        QuerySearchType::Normal,
        SearchVectorFlags::new(false, true),
      ),
      2 | 3 => search_vector_with_type(
        self,
        &lookup,
        &vector,
        QueryOp::Equal,
        search,
        if !matches!(lookup, FormulaValue::String(_)) {
          QuerySearchType::Normal
        } else if match_mode == 2 && may_be_wildcard(self.text(&lookup).as_ref()) {
          QuerySearchType::Wildcard
        } else if match_mode == 3 && may_be_regex(self.text(&lookup).as_ref()) {
          QuerySearchType::Regex
        } else {
          QuerySearchType::Normal
        },
        SearchVectorFlags::new(true, false),
      ),
      _ => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    };
    index
      .map(|index| FormulaValue::Number(index as f64 + 1.0))
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  pub(crate) fn evaluate_vlookup(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=4).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let lookup = self.evaluate(args.first()?)?;
    let Some(result_column) = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let result_column = result_column.floor();
    if result_column < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let result_column = result_column as u32;
    let Some(table) = self.evaluate(args.get(1)?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let sorted = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    if self.array_context
      && matches!(
        lookup,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      )
    {
      return self.map_unary_values(lookup, |evaluator, lookup| {
        Some(evaluator.evaluate_vlookup_value(
          evaluator.scalar_value(lookup.clone()),
          &table,
          result_column,
          sorted,
        ))
      });
    }
    Some(self.evaluate_vlookup_value(self.scalar_value(lookup), &table, result_column, sorted))
  }

  pub(crate) fn evaluate_vlookup_value(
    &self,
    lookup: FormulaValue<'doc>,
    table: &FormulaValue<'doc>,
    result_column: u32,
    sorted: bool,
  ) -> FormulaValue<'doc> {
    if let FormulaValue::Matrix(rows) = table {
      let Some(result_index) = result_column.checked_sub(1).map(|value| value as usize) else {
        return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
      };
      if rows.first().is_none_or(|row| result_index >= row.len()) {
        return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
      }
      let index = vhlookup_matrix_index(self, &lookup, rows, false, sorted);
      let Some(index) = index else {
        return FormulaValue::Error(FormulaErrorValue::NA);
      };
      return rows
        .get(index)
        .and_then(|row| row.get(result_index))
        .cloned()
        .unwrap_or(FormulaValue::Error(FormulaErrorValue::Ref));
    }

    let Some(reference) = self.as_reference(table) else {
      return FormulaValue::Error(FormulaErrorValue::Value);
    };
    if reference.range.start.column + result_column - 1 > reference.range.end.column {
      return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
    }
    let index = self.vlookup_reference_row_index(&lookup, &reference, sorted);
    index
      .map(|row| {
        let sheet = self.range_sheet(&reference);
        self.book.cell_value(
          sheet,
          CellAddress {
            column: reference.range.start.column + result_column - 1,
            row,
          },
        )
      })
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::NA))
  }

  pub(crate) fn vlookup_reference_row_index(
    &self,
    lookup: &FormulaValue<'doc>,
    reference: &QualifiedRange<'doc>,
    sorted: bool,
  ) -> Option<u32> {
    let sheet = self.range_sheet(reference);
    let start_row = reference.range.start.row.min(reference.range.end.row);
    let end_row = reference.range.start.row.max(reference.range.end.row);
    let search_column = reference.range.start.column.min(reference.range.end.column);
    let (mut query, search_type) = QueryEntry::from_value(self, lookup, 0);
    if sorted {
      query.op = QueryOp::LessOrEqual;
    }
    let param = QueryParam::single(query, search_type, self.book.formula_match_whole_cell)
      .with_range_lookup(sorted);
    let query = param.entries.first()?;
    let mut found = None;
    for row in start_row..=end_row {
      let address = CellAddress {
        column: search_column,
        row,
      };
      let value = self
        .book
        .query_cell_value(sheet, address, self.book.cell_value(sheet, address));
      let query_empty = self.book.is_query_empty_cell(sheet, address);
      if !query_matches(self, &param, query, &value, query_empty) {
        if sorted
          && found.is_some()
          && lookup_candidate_type_matches(query, &value)
          && lookup_compare_candidate_to_query(self, &value, query, &param, true) == Some(1)
        {
          break;
        }
        continue;
      }
      if sorted {
        if lookup_candidate_type_matches(query, &value)
          && found.is_none_or(|found_row| {
            let found_address = CellAddress {
              column: search_column,
              row: found_row,
            };
            let found_value = self.book.query_cell_value(
              sheet,
              found_address,
              self.book.cell_value(sheet, found_address),
            );
            lookup_compare_cells(self, &value, &found_value) >= 0
          })
        {
          found = Some(row);
        }
      } else {
        return Some(row);
      }
    }
    found
  }

  pub(crate) fn evaluate_hlookup(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=4).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let lookup = self.scalar_value(self.evaluate(args.first()?)?);
    let row_number = self.number(&self.evaluate(args.get(2)?)?)?.floor();
    if row_number < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let row_index = row_number as usize - 1;
    let sorted = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    let matrix = self.matrix_values(&self.evaluate(args.get(1)?)?);
    if row_index >= matrix.len() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let index = vhlookup_matrix_index(self, &lookup, &matrix, true, sorted);
    let Some(index) = index else {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    };
    matrix
      .get(row_index)
      .and_then(|row| row.get(index))
      .cloned()
      .or(Some(FormulaValue::Error(FormulaErrorValue::Ref)))
  }

  pub(crate) fn evaluate_xlookup(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 {
      return None;
    }
    let lookup = self.scalar_value(self.evaluate(args.first()?)?);
    let array_evaluator = self.with_array_context();
    let lookup_matrix = self.matrix_values(&array_evaluator.evaluate(args.get(1)?)?);
    let return_matrix = self.matrix_values(&array_evaluator.evaluate(args.get(2)?)?);
    let lookup_rows = lookup_matrix.len();
    let lookup_columns = lookup_matrix.first().map_or(0, Vec::len);
    if lookup_rows > 1 && lookup_columns > 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let (lookup_vector, lookup_vertical) = lookup_vector(&lookup_matrix)?;
    let return_rows = return_matrix.len();
    let return_columns = return_matrix.first().map_or(0, Vec::len);
    if return_rows == 0
      || return_columns == 0
      || return_matrix.iter().any(|row| row.len() != return_columns)
      || (lookup_vertical && return_rows != lookup_vector.len())
      || (!lookup_vertical && return_columns != lookup_vector.len())
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let not_found = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .filter(|value| !matches!(value, FormulaValue::Blank));
    let match_mode = args
      .get(4)
      .and_then(|arg| self.optional_number_value(arg))
      .unwrap_or(0.0) as i32;
    let search_mode = args
      .get(5)
      .and_then(|arg| self.optional_number_value(arg))
      .unwrap_or(1.0) as i32;
    let Some(search) = LookupSearchMode::from_excel(search_mode) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    if matches!(match_mode, 2 | 3)
      && matches!(
        search,
        LookupSearchMode::BinaryAscending | LookupSearchMode::BinaryDescending
      )
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
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
      2 | 3 => search_vector_with_type(
        self,
        &lookup,
        &lookup_vector,
        QueryOp::Equal,
        search,
        if !matches!(lookup, FormulaValue::String(_)) {
          QuerySearchType::Normal
        } else if match_mode == 2 && may_be_wildcard(self.text(&lookup).as_ref()) {
          QuerySearchType::Wildcard
        } else if match_mode == 3 && may_be_regex(self.text(&lookup).as_ref()) {
          QuerySearchType::Regex
        } else {
          QuerySearchType::Normal
        },
        SearchVectorFlags::new(true, false),
      ),
      _ => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    };
    let Some(index) = index else {
      return not_found.or(Some(FormulaValue::Error(FormulaErrorValue::NA)));
    };
    if lookup_vertical {
      return_matrix
        .get(index)
        .and_then(|row| {
          if row.len() == 1 {
            row.first().cloned()
          } else {
            Some(FormulaValue::Matrix(vec![row.clone()]))
          }
        })
        .or(Some(FormulaValue::Error(FormulaErrorValue::Ref)))
    } else if return_matrix.len() == 1 {
      return_matrix
        .first()
        .and_then(|row| row.get(index))
        .cloned()
        .or(Some(FormulaValue::Error(FormulaErrorValue::Ref)))
    } else {
      let column = return_matrix
        .iter()
        .filter_map(|row| row.get(index).cloned())
        .map(|value| vec![value])
        .collect::<Vec<_>>();
      Some(FormulaValue::Matrix(column))
    }
  }

  pub(crate) fn evaluate_sheets(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if let Some(arg) = args.first() {
      if let Some(count) = self.sheets_count_from_ast(arg) {
        return Some(FormulaValue::Number(count as f64));
      }
      let Some(value) = self.evaluate(arg) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      return match value {
        FormulaValue::Reference(_) => Some(FormulaValue::Number(1.0)),
        FormulaValue::Matrix(_) => Some(FormulaValue::Error(FormulaErrorValue::Unknown)),
        _ => Some(FormulaValue::Error(FormulaErrorValue::Value)),
      };
    }
    Some(FormulaValue::Number(
      self.book.sheet_names.len().max(1) as f64
    ))
  }

  pub(crate) fn evaluate_sheet(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let sheet = if let Some(arg) = args.first() {
      let Some(value) = self.evaluate(arg) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      match value {
        FormulaValue::Reference(reference) => self.range_sheet(&reference),
        FormulaValue::String(name) => {
          let Some(sheet) = self.book.sheet_id_by_name(name.as_ref()) else {
            return Some(FormulaValue::Error(FormulaErrorValue::Value));
          };
          sheet
        }
        _ => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
      }
    } else {
      self.current_sheet
    };
    let index = self
      .book
      .sheet_names
      .iter()
      .position(|binding| binding.id == sheet)
      .map(|index| index + 1)
      .unwrap_or(sheet.0 as usize + 1);
    Some(FormulaValue::Number(index as f64))
  }

  pub(crate) fn sheets_count_from_ast(&self, arg: &FormulaAst<'doc>) -> Option<u32> {
    // Source: LibreOffice sc/source/core/tool/interpr1.cxx ScInterpreter::ScSheets.
    match arg {
      FormulaAst::Reference(reference) => Some(self.reference_sheet_count(reference)),
      FormulaAst::Name(name) => self.three_d_reference_sheet_count(name.as_ref()),
      _ => None,
    }
  }

  pub(crate) fn reference_sheet_count(&self, reference: &QualifiedRange<'doc>) -> u32 {
    let start = self.range_sheet(reference).0;
    let end = reference
      .sheet_name
      .as_ref()
      .and_then(|name| self.book.sheet_id_by_name(name.0.as_ref()))
      .map(|sheet| sheet.0)
      .unwrap_or(start);
    start.abs_diff(end) + 1
  }

  pub(crate) fn three_d_reference_sheet_count(&self, text: &str) -> Option<u32> {
    let (left, right) = text.split_once(':')?;
    let left = QualifiedAddress::parse_a1(self.current_sheet, left).ok()?;
    let right = QualifiedAddress::parse_a1(self.current_sheet, right).ok()?;
    let left_sheet = left
      .sheet_name
      .as_ref()
      .and_then(|name| self.book.sheet_id_by_name(name.0.as_ref()))?;
    let right_sheet = right
      .sheet_name
      .as_ref()
      .and_then(|name| self.book.sheet_id_by_name(name.0.as_ref()))?;
    Some(left_sheet.0.abs_diff(right_sheet.0) + 1)
  }

  pub(crate) fn evaluate_formula_text(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let reference = self.as_reference(&self.evaluate(args.first()?)?)?;
    let sheet = self.range_sheet(&reference);
    self
      .book
      .formula_text(sheet, reference.range.start)
      .map(|text| FormulaValue::String(Cow::Owned(text)))
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  pub(crate) fn evaluate_and(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let mut has_value = false;
    let mut result = true;
    for value in self.values(args) {
      if let FormulaValue::Error(error) = value {
        return Some(FormulaValue::Error(error));
      }
      if let Some(logical) = logical_value(&value) {
        has_value = true;
        result &= logical;
      }
    }
    Some(if has_value {
      FormulaValue::Boolean(result)
    } else {
      FormulaValue::Error(FormulaErrorValue::Value)
    })
  }

  pub(crate) fn evaluate_or(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let mut has_value = false;
    let mut result = false;
    for value in self.values(args) {
      if let FormulaValue::Error(error) = value {
        return Some(FormulaValue::Error(error));
      }
      if let Some(logical) = logical_value(&value) {
        has_value = true;
        result |= logical;
      }
    }
    Some(if has_value {
      FormulaValue::Boolean(result)
    } else {
      FormulaValue::Error(FormulaErrorValue::Value)
    })
  }

  pub(crate) fn evaluate_xor(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let mut has_value = false;
    let mut result = false;
    for value in self.values(args) {
      if let FormulaValue::Error(error) = value {
        return Some(FormulaValue::Error(error));
      }
      if let Some(logical) = logical_value(&value) {
        has_value = true;
        result ^= logical;
      }
    }
    Some(if has_value {
      FormulaValue::Boolean(result)
    } else {
      FormulaValue::Error(FormulaErrorValue::Value)
    })
  }

  pub(crate) fn evaluate_cell(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  pub(crate) fn evaluate_mid(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let text = self.text(&self.evaluate(args.first()?)?);
    let start = self.number(&self.evaluate(args.get(1)?)?)?;
    let len = self.number(&self.evaluate(args.get(2)?)?)?;
    if start < 1.0 || len < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let start = start as usize;
    let len = len as usize;
    Some(FormulaValue::String(Cow::Owned(
      text
        .chars()
        .skip(start.saturating_sub(1))
        .take(len)
        .collect(),
    )))
  }

  pub(crate) fn evaluate_midb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let text = self.text(&self.evaluate(args.first()?)?);
    let start = self.number(&self.evaluate(args.get(1)?)?)? as usize;
    let len = self.number(&self.evaluate(args.get(2)?)?)? as usize;
    if start == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let prefix = leftb(&text, start.saturating_sub(1));
    let suffix = text
      .chars()
      .skip(prefix.chars().count())
      .collect::<String>();
    Some(FormulaValue::String(Cow::Owned(leftb(&suffix, len))))
  }

  pub(crate) fn evaluate_left(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.evaluate(args.first()?)?;
    let len_value = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .unwrap_or(FormulaValue::Number(1.0));
    if self.array_context
      && (matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        || matches!(
          len_value,
          FormulaValue::Reference(_) | FormulaValue::Matrix(_)
        ))
    {
      return self.map_binary_values(value, len_value, |evaluator, value, len| {
        evaluator.left_value(value, len)
      });
    }
    self.left_value(&value, &len_value)
  }

  pub(crate) fn left_value(
    &self,
    value: &FormulaValue<'doc>,
    len: &FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let len = self.number(len)?;
    if len < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let len = len.floor() as usize;
    let text = self.text(&value);
    Some(FormulaValue::String(Cow::Owned(
      text.chars().take(len).collect(),
    )))
  }

  pub(crate) fn evaluate_leftb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.evaluate(args.first()?)?;
    let len_value = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .unwrap_or(FormulaValue::Number(1.0));
    if self.array_context
      && (matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        || matches!(
          len_value,
          FormulaValue::Reference(_) | FormulaValue::Matrix(_)
        ))
    {
      return self.map_binary_values(value, len_value, |evaluator, value, len| {
        evaluator.leftb_value(value, len)
      });
    }
    self.leftb_value(&value, &len_value)
  }

  pub(crate) fn leftb_value(
    &self,
    value: &FormulaValue<'doc>,
    len: &FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let len = self.number(len)?;
    if len < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let text = self.text(value);
    Some(FormulaValue::String(Cow::Owned(leftb(
      &text,
      len.floor() as usize,
    ))))
  }

  pub(crate) fn evaluate_right(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.evaluate(args.first()?)?;
    let len_value = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .unwrap_or(FormulaValue::Number(1.0));
    if self.array_context
      && (matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        || matches!(
          len_value,
          FormulaValue::Reference(_) | FormulaValue::Matrix(_)
        ))
    {
      return self.map_binary_values(value, len_value, |evaluator, value, len| {
        evaluator.right_value(value, len)
      });
    }
    self.right_value(&value, &len_value)
  }

  pub(crate) fn right_value(
    &self,
    value: &FormulaValue<'doc>,
    len: &FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let len = self.number(len)?;
    if len < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let len = len.floor() as usize;
    let text = self.text(value);
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

  pub(crate) fn evaluate_rightb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.evaluate(args.first()?)?;
    let len_value = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .unwrap_or(FormulaValue::Number(1.0));
    if self.array_context
      && (matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        || matches!(
          len_value,
          FormulaValue::Reference(_) | FormulaValue::Matrix(_)
        ))
    {
      return self.map_binary_values(value, len_value, |evaluator, value, len| {
        evaluator.rightb_value(value, len)
      });
    }
    self.rightb_value(&value, &len_value)
  }

  pub(crate) fn rightb_value(
    &self,
    value: &FormulaValue<'doc>,
    len: &FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    let len = self.number(len)?;
    if len < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let text = self.text(value);
    Some(FormulaValue::String(Cow::Owned(rightb(
      &text,
      len.floor() as usize,
    ))))
  }

  pub(crate) fn evaluate_roman(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=2).contains(&args.len()) {
      return None;
    }
    let value = self.number(&self.evaluate(args.first()?)?)?.floor() as i32;
    if !(0..=3999).contains(&value) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mode = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(approx_floor)
      .unwrap_or(0.0);
    if !(0.0..5.0).contains(&mode) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let output = roman_text_libreoffice(value as u16, mode as u16);
    Some(FormulaValue::String(Cow::Owned(output)))
  }

  pub(crate) fn evaluate_arabic(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return None;
    }
    let text = self
      .text(&self.evaluate(args.first()?)?)
      .to_ascii_uppercase();
    let mut previous = 0;
    let mut total = 0;
    for ch in text.chars().rev() {
      let value = match ch {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
      };
      if value < previous {
        total -= value;
      } else {
        total += value;
        previous = value;
      }
    }
    Some(FormulaValue::Number(total as f64))
  }

  pub(crate) fn evaluate_replaceb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 4 {
      return None;
    }
    let old_text = self.text(&self.evaluate(args.first()?)?);
    let start = self.number(&self.evaluate(args.get(1)?)?)? as i32;
    let count = self.number(&self.evaluate(args.get(2)?)?)? as i32;
    let new_text = self.text(&self.evaluate(args.get(3)?)?);
    let len = text_byte_len(&old_text) as i32;
    if start < 1 || start > len || count < 0 || start + count - 1 > len {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let left = leftb(&old_text, (start - 1) as usize);
    let right = rightb(&old_text, (len - start - count + 1).max(0) as usize);
    Some(FormulaValue::String(Cow::Owned(format!(
      "{left}{new_text}{right}"
    ))))
  }

  pub(crate) fn evaluate_hyperlink(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=2).contains(&args.len()) {
      return None;
    }
    let url_value = self.evaluate(args.first()?)?;
    let url = match self.scalar_value(url_value) {
      FormulaValue::Error(error) => {
        return Some(FormulaValue::Matrix(vec![
          vec![FormulaValue::Error(error)],
          vec![FormulaValue::Error(error)],
        ]));
      }
      value => self.text(&value),
    };
    let display = if let Some(display_arg) = args.get(1) {
      match self.scalar_value(self.evaluate(display_arg)?) {
        FormulaValue::Error(error) => FormulaValue::Error(error),
        FormulaValue::Number(value) => FormulaValue::Number(value),
        FormulaValue::String(value) => FormulaValue::String(value),
        FormulaValue::Boolean(value) => FormulaValue::Number(if value { 1.0 } else { 0.0 }),
        FormulaValue::Blank => FormulaValue::Number(0.0),
        value => FormulaValue::String(Cow::Owned(self.text(&value))),
      }
    } else {
      FormulaValue::String(Cow::Owned(url.clone()))
    };
    if let FormulaValue::Error(error) = display {
      return Some(FormulaValue::Matrix(vec![
        vec![FormulaValue::Error(error)],
        vec![FormulaValue::String(Cow::Owned(url))],
      ]));
    }
    Some(FormulaValue::Matrix(vec![
      vec![display],
      vec![FormulaValue::String(Cow::Owned(url))],
    ]))
  }

  pub(crate) fn evaluate_to_row_column(
    &self,
    args: &[FormulaAst<'doc>],
    row_result: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.is_empty() || args.len() > 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let source = self.evaluate(args.first()?)?;
    let matrix = self.matrix_values(&source);
    let ignore = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0) as i32;
    let scan_by_column = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    let rows = matrix.len();
    let columns = matrix.iter().map(Vec::len).max().unwrap_or(0);
    if rows == 0 || columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }

    let mut values = Vec::with_capacity(rows * columns);
    if scan_by_column {
      for column in 0..columns {
        for row in &matrix {
          if let Some(value) = row.get(column)
            && !should_ignore_to_row_column_value(value, ignore)
          {
            values.push(value.clone());
          }
        }
      }
    } else {
      for row in &matrix {
        for value in row {
          if !should_ignore_to_row_column_value(value, ignore) {
            values.push(value.clone());
          }
        }
      }
    }

    if row_result {
      Some(FormulaValue::Matrix(vec![values]))
    } else {
      Some(FormulaValue::Matrix(
        values.into_iter().map(|value| vec![value]).collect(),
      ))
    }
  }

  pub(crate) fn evaluate_choose_rows(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(source) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let matrix = self.matrix_values(&source);
    let row_count = matrix.len();
    if row_count == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut rows = Vec::new();
    for arg in args.iter().skip(1) {
      let Some(value) = self.evaluate(arg) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      for index_value in self.matrix_values(&value).into_iter().flatten() {
        let Some(index) = self.number(&index_value).map(|value| value.trunc() as i64) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        let Some(row) = choose_row_column_index(index, row_count) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        rows.push(matrix.get(row)?.clone());
      }
    }
    Some(FormulaValue::Matrix(rows))
  }

  pub(crate) fn evaluate_choose_cols(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(source) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let matrix = self.matrix_values(&source);
    let column_count = matrix.first().map_or(0, Vec::len);
    if matrix.is_empty() || column_count == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut indexes = Vec::new();
    for arg in args.iter().skip(1) {
      let value = self.evaluate(arg)?;
      for index_value in self.matrix_values(&value).into_iter().flatten() {
        let Some(index) = self.number(&index_value).map(|value| value.trunc() as i64) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        let Some(index) = choose_row_column_index(index, column_count) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        indexes.push(index);
      }
    }
    let mut values = Vec::new();
    for row in &matrix {
      let mut out = Vec::new();
      for index in &indexes {
        out.push(row.get(*index).cloned().unwrap_or_default());
      }
      values.push(out);
    }
    Some(FormulaValue::Matrix(values))
  }

  pub(crate) fn evaluate_expand(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=4).contains(&args.len()) {
      return None;
    }
    let source = self.evaluate(args.first()?)?;
    let matrix = self.matrix_values(&source);
    let source_rows = matrix.len();
    let source_cols = matrix.first().map_or(0, Vec::len);
    if source_rows == 0 || source_cols == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let rows = match self.evaluate(args.get(1)?)? {
      FormulaValue::Blank => source_rows,
      value => self.number(&value)?.abs() as usize,
    };
    let cols = match args.get(2) {
      Some(arg) => match self.evaluate(arg)? {
        FormulaValue::Blank => source_cols,
        value => self.number(&value)?.abs() as usize,
      },
      None => source_cols,
    };
    if rows < source_rows || cols < source_cols {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let pad = match args.get(3) {
      Some(arg) => self.evaluate(arg)?,
      None => FormulaValue::Error(FormulaErrorValue::NA),
    };
    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut output_row = Vec::with_capacity(cols);
      for col in 0..cols {
        output_row.push(
          matrix
            .get(row)
            .and_then(|source_row| source_row.get(col))
            .cloned()
            .unwrap_or_else(|| pad.clone()),
        );
      }
      result.push(output_row);
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_stack(
    &self,
    args: &[FormulaAst<'doc>],
    horizontal: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    if args.iter().any(is_missing_argument) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let matrices = args
      .iter()
      .map(|arg| self.evaluate(arg).map(|value| self.matrix_values(&value)))
      .collect::<Option<Vec<_>>>()?;
    if matrices
      .iter()
      .any(|matrix| matrix.is_empty() || matrix.first().is_none_or(Vec::is_empty))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let rows = if horizontal {
      matrices.iter().map(Vec::len).max().unwrap_or(0)
    } else {
      matrices.iter().map(Vec::len).sum()
    };
    let columns = if horizontal {
      matrices
        .iter()
        .map(|matrix| matrix.first().map_or(0, Vec::len))
        .sum()
    } else {
      matrices
        .iter()
        .map(|matrix| matrix.first().map_or(0, Vec::len))
        .max()
        .unwrap_or(0)
    };
    let pad = FormulaValue::Error(FormulaErrorValue::NA);
    let mut result = Vec::with_capacity(rows);
    if horizontal {
      for row in 0..rows {
        let mut result_row = Vec::with_capacity(columns);
        for matrix in &matrices {
          let width = matrix.first().map_or(0, Vec::len);
          for column in 0..width {
            result_row.push(
              matrix
                .get(row)
                .and_then(|source_row| source_row.get(column))
                .cloned()
                .unwrap_or_else(|| pad.clone()),
            );
          }
        }
        result.push(result_row);
      }
    } else {
      for matrix in &matrices {
        for source_row in matrix {
          let mut row = source_row.clone();
          row.resize(columns, pad.clone());
          result.push(row);
        }
      }
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_wrap(
    &self,
    args: &[FormulaAst<'doc>],
    by_columns: bool,
  ) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let matrix = self.matrix_values(&self.evaluate(args.first()?)?);
    let rows = matrix.len();
    let columns = matrix.first().map_or(0, Vec::len);
    if rows == 0 || columns == 0 || (rows > 1 && columns > 1) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let wrap = self.number_arg(args, 1)?.floor() as usize;
    if wrap == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let pad = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::NA));
    let values = matrix.into_iter().flatten().collect::<Vec<_>>();
    let outer = values.len().div_ceil(wrap);
    let result_rows = if by_columns { wrap } else { outer };
    let result_columns = if by_columns { outer } else { wrap };
    let mut result = vec![vec![pad; result_columns]; result_rows];
    for (index, value) in values.into_iter().enumerate() {
      let row = if by_columns {
        index % wrap
      } else {
        index / wrap
      };
      let column = if by_columns {
        index / wrap
      } else {
        index % wrap
      };
      result[row][column] = value;
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_filter(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let data = self.matrix_values(&self.evaluate(args.first()?)?);
    let include = self.matrix_values(&self.with_array_context().evaluate(args.get(1)?)?);
    if data.is_empty() || data.first().is_none_or(Vec::is_empty) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let include_rows = include.len();
    let include_columns = include.first().map_or(0, Vec::len);
    if include_rows == 0 || include_columns == 0 || (include_rows > 1 && include_columns > 1) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let include_values = include.into_iter().flatten().collect::<Vec<_>>();
    let mut result = Vec::new();
    if include_values.len() == data.len() {
      for (row, include) in data.into_iter().zip(include_values) {
        if self.truthy(&include) {
          result.push(row);
        }
      }
    } else if include_values.len() == data.first()?.len() {
      let columns = include_values
        .iter()
        .enumerate()
        .filter_map(|(index, value)| self.truthy(value).then_some(index))
        .collect::<Vec<_>>();
      for row in data {
        result.push(columns.iter().map(|column| row[*column].clone()).collect());
      }
    } else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    if result.is_empty() || result.first().is_some_and(Vec::is_empty) {
      return Some(
        args
          .get(2)
          .and_then(|arg| self.evaluate(arg))
          .unwrap_or(FormulaValue::Error(FormulaErrorValue::Calc)),
      );
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_unique(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=3).contains(&args.len()) {
      return None;
    }
    let data = self.matrix_values(&self.evaluate(args.first()?)?);
    let by_col = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    let exactly_once = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    if data.is_empty() || data.first().is_none_or(Vec::is_empty) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut keys = Vec::<Vec<String>>::new();
    let mut counts = Vec::<usize>::new();
    let mut positions = Vec::<usize>::new();
    let outer = if by_col { data[0].len() } else { data.len() };
    for index in 0..outer {
      let key = if by_col {
        data
          .iter()
          .map(|row| {
            row
              .get(index)
              .map(display_text_from_value)
              .map(|text| text.to_lowercase())
              .unwrap_or_default()
          })
          .collect::<Vec<_>>()
      } else {
        data[index]
          .iter()
          .map(display_text_from_value)
          .map(|text| text.to_lowercase())
          .collect::<Vec<_>>()
      };
      if let Some(existing) = keys.iter().position(|item| *item == key) {
        counts[existing] += 1;
      } else {
        keys.push(key);
        counts.push(1);
        positions.push(index);
      }
    }
    let selected = positions
      .into_iter()
      .zip(counts)
      .filter_map(|(position, count)| (!exactly_once || count == 1).then_some(position))
      .collect::<Vec<_>>();
    if selected.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    }
    if by_col {
      Some(FormulaValue::Matrix(
        data
          .iter()
          .map(|row| selected.iter().map(|column| row[*column].clone()).collect())
          .collect(),
      ))
    } else {
      Some(FormulaValue::Matrix(
        selected.into_iter().map(|row| data[row].clone()).collect(),
      ))
    }
  }

  pub(crate) fn evaluate_transpose(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let matrix = self.matrix_values(&self.evaluate(args.first()?)?);
    let rows = matrix.len();
    let columns = matrix.iter().map(Vec::len).max().unwrap_or(0);
    if rows == 0 || columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut result = Vec::with_capacity(columns);
    for column in 0..columns {
      let mut row = Vec::with_capacity(rows);
      for source_row in &matrix {
        row.push(source_row.get(column).cloned().unwrap_or_default());
      }
      result.push(row);
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_sequence(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.is_empty() || args.len() > 4 {
      return None;
    }
    let rows = args
      .first()
      .and_then(|arg| self.optional_number_value(arg))
      .unwrap_or(1.0)
      .floor() as usize;
    let columns = args
      .get(1)
      .and_then(|arg| self.optional_number_value(arg))
      .unwrap_or(1.0)
      .floor() as usize;
    let start = args
      .get(2)
      .and_then(|arg| self.optional_number_value(arg))
      .unwrap_or(1.0);
    let step = args
      .get(3)
      .and_then(|arg| self.optional_number_value(arg))
      .unwrap_or(1.0);
    if rows == 0 || columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut next = start;
    let mut output = Vec::with_capacity(rows);
    for _ in 0..rows {
      let mut row = Vec::with_capacity(columns);
      for _ in 0..columns {
        row.push(FormulaValue::Number(next));
        next += step;
      }
      output.push(row);
    }
    Some(FormulaValue::Matrix(output))
  }

  pub(crate) fn evaluate_randarray(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() > 5 {
      return None;
    }
    let rows = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0)
      .floor() as usize;
    let columns = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0)
      .floor() as usize;
    let mut min = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let mut max = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0);
    let whole = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    if whole {
      min = min.ceil();
      max = max.ceil();
    }
    if rows == 0 || columns == 0 || min > max {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut rng = XorShift64::new(time_seed());
    let mut result = Vec::with_capacity(rows);
    for _ in 0..rows {
      let mut row = Vec::with_capacity(columns);
      for _ in 0..columns {
        let mut value = min + rng.next_unit() * (max - min);
        if whole {
          value = value.floor();
        }
        row.push(FormulaValue::Number(value));
      }
      result.push(row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_take_drop(
    &self,
    args: &[FormulaAst<'doc>],
    take: bool,
  ) -> Option<FormulaValue<'doc>> {
    if !(1..=3).contains(&args.len()) {
      return None;
    }
    let matrix = self.matrix_values(&self.evaluate(args.first()?)?);
    let row_count = matrix.len();
    let col_count = matrix.first().map_or(0, Vec::len);
    if row_count == 0 || col_count == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let rows_arg = args
      .get(1)
      .and_then(|arg| self.optional_number_value(arg))
      .map(|value| value as isize);
    let cols_arg = args
      .get(2)
      .and_then(|arg| self.optional_number_value(arg))
      .map(|value| value as isize);
    let (row_start, row_end) = take_drop_bounds(row_count, rows_arg, take);
    let (col_start, col_end) = take_drop_bounds(col_count, cols_arg, take);
    if row_start >= row_end || col_start >= col_end {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    }
    let result = matrix[row_start..row_end]
      .iter()
      .map(|row| row[col_start..col_end].to_vec())
      .collect::<Vec<_>>();
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_sort(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=4).contains(&args.len()) {
      return None;
    }
    let data = self.matrix_values(&self.evaluate(args.first()?)?);
    if data.is_empty() || data.first().is_none_or(Vec::is_empty) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut sort_indices = args
      .get(1)
      .and_then(|arg| self.optional_number_array_values(arg))
      .unwrap_or_else(|| vec![1.0]);
    let sort_orders = args
      .get(2)
      .and_then(|arg| self.optional_number_array_values(arg))
      .unwrap_or_else(|| vec![1.0]);
    let by_col = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .filter(|value| !matches!(value, FormulaValue::Blank))
      .map(|value| self.truthy(&value))
      .unwrap_or(false);
    if sort_indices.is_empty()
      || sort_orders.is_empty()
      || (sort_indices.len() != sort_orders.len() && sort_orders.len() > 1)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if sort_indices.iter().any(|value| *value < 1.0)
      || sort_orders
        .iter()
        .any(|value| !matches!(*value, 1.0 | -1.0))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    for sort_index in &mut sort_indices {
      *sort_index = sort_index.floor() - 1.0;
    }
    let sort_keys = sort_indices
      .iter()
      .enumerate()
      .map(|(index, sort_index)| {
        (
          *sort_index as usize,
          sort_orders
            .get(index)
            .or_else(|| sort_orders.first())
            .copied()
            .unwrap_or(1.0)
            == 1.0,
        )
      })
      .collect::<Vec<_>>();
    if by_col {
      if sort_keys.iter().any(|(key, _)| *key >= data.len()) {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      let keys = sort_keys
        .iter()
        .map(|(key, ascending)| (data[*key].clone(), *ascending))
        .collect::<Vec<_>>();
      let mut order = (0..data[0].len()).collect::<Vec<_>>();
      order.sort_by(|left, right| sort_multi_key_order(self, &keys, *left, *right));
      Some(FormulaValue::Matrix(reorder_columns(&data, &order)))
    } else {
      if sort_keys.iter().any(|(key, _)| *key >= data[0].len()) {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      let keys = sort_keys
        .iter()
        .map(|(key, ascending)| {
          (
            data
              .iter()
              .map(|row| row.get(*key).cloned().unwrap_or_default())
              .collect::<Vec<_>>(),
            *ascending,
          )
        })
        .collect::<Vec<_>>();
      let mut order = (0..data.len()).collect::<Vec<_>>();
      order.sort_by(|left, right| sort_multi_key_order(self, &keys, *left, *right));
      Some(FormulaValue::Matrix(
        order.into_iter().map(|row| data[row].clone()).collect(),
      ))
    }
  }

  pub(crate) fn evaluate_sortby(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 {
      return None;
    }
    let data = self.matrix_values(&self.evaluate(args.first()?)?);
    if data.is_empty() || data.first().is_none_or(Vec::is_empty) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut keys = Vec::new();
    let mut by_rows = None;
    let mut index = 1;
    while index < args.len() {
      let matrix = self.matrix_values(&self.evaluate(&args[index])?);
      let rows = matrix.len();
      let cols = matrix.first().map_or(0, Vec::len);
      if rows == 0 || cols == 0 {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      let current_by_rows = if cols == 1 && rows > 1 {
        true
      } else if rows == 1 && cols > 1 {
        false
      } else if rows == 1 && cols == 1 {
        return Some(FormulaValue::Matrix(data));
      } else {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      };
      if let Some(expected) = by_rows {
        if expected != current_by_rows {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        }
      } else {
        by_rows = Some(current_by_rows);
      }
      let ascending = if let Some(order_arg) = args.get(index + 1) {
        let order = self.number(&self.evaluate(order_arg)?)?;
        if !matches!(order, 1.0 | -1.0) {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        }
        order == 1.0
      } else {
        true
      };
      let values = if current_by_rows {
        matrix
          .into_iter()
          .map(|row| row.into_iter().next().unwrap_or_default())
          .collect::<Vec<_>>()
      } else {
        matrix.into_iter().next().unwrap_or_default()
      };
      keys.push((values, ascending));
      index += 2;
    }
    let by_rows = by_rows.unwrap_or(true);
    if by_rows {
      if keys.iter().any(|(values, _)| values.len() != data.len()) {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      let mut order = (0..data.len()).collect::<Vec<_>>();
      order.sort_by(|left, right| sort_multi_key_order(self, &keys, *left, *right));
      Some(FormulaValue::Matrix(
        order.into_iter().map(|row| data[row].clone()).collect(),
      ))
    } else {
      if keys.iter().any(|(values, _)| values.len() != data[0].len()) {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      let mut order = (0..data[0].len()).collect::<Vec<_>>();
      order.sort_by(|left, right| sort_multi_key_order(self, &keys, *left, *right));
      Some(FormulaValue::Matrix(reorder_columns(&data, &order)))
    }
  }

  pub(crate) fn evaluate_mmult(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let left = self.matrix_values(&self.evaluate(args.first()?)?);
    let right = self.matrix_values(&self.evaluate(args.get(1)?)?);
    let rows = left.len();
    let shared = left.first().map_or(0, Vec::len);
    let right_rows = right.len();
    let columns = right.first().map_or(0, Vec::len);
    if rows == 0
      || shared == 0
      || right_rows == 0
      || columns == 0
      || left.iter().any(|row| row.len() != shared)
      || right.iter().any(|row| row.len() != columns)
      || shared != right_rows
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let left_numbers = left
      .iter()
      .map(|row| {
        row
          .iter()
          .map(|value| self.number(value).unwrap_or(0.0))
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
    let right_numbers = right
      .iter()
      .map(|row| {
        row
          .iter()
          .map(|value| self.number(value).unwrap_or(0.0))
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
    let Some(result_numbers) = matrix_multiply(&left_numbers, &right_numbers) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let result = result_numbers
      .into_iter()
      .map(|row| {
        row
          .into_iter()
          .map(FormulaValue::Number)
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_sumx2(
    &self,
    args: &[FormulaAst<'doc>],
    plus: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let left = self.matrix_values(&self.evaluate(args.first()?)?);
    let right = self.matrix_values(&self.evaluate(args.get(1)?)?);
    if matrix_dimensions(&left) != matrix_dimensions(&right) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();
    for (left_row, right_row) in left.iter().zip(&right) {
      for (left_value, right_value) in left_row.iter().zip(right_row) {
        let Some(left_number) = matrix_stat_number(left_value) else {
          continue;
        };
        let Some(right_number) = matrix_stat_number(right_value) else {
          continue;
        };
        left_numbers.push(left_number);
        right_numbers.push(right_number);
      }
    }
    stats_sum_x2(&left_numbers, &right_numbers, plus).map(FormulaValue::Number)
  }

  pub(crate) fn evaluate_sumxmy2(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let left = self.matrix_values(&self.evaluate(args.first()?)?);
    let right = self.matrix_values(&self.evaluate(args.get(1)?)?);
    if matrix_dimensions(&left) != matrix_dimensions(&right) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();
    for (left_row, right_row) in left.iter().zip(&right) {
      for (left_value, right_value) in left_row.iter().zip(right_row) {
        let Some(left_number) = matrix_stat_number(left_value) else {
          continue;
        };
        let Some(right_number) = matrix_stat_number(right_value) else {
          continue;
        };
        left_numbers.push(left_number);
        right_numbers.push(right_number);
      }
    }
    stats_sum_xmy2(&left_numbers, &right_numbers).map(FormulaValue::Number)
  }

  pub(crate) fn evaluate_frequency(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let array_evaluator = self.with_array_context();
    let data = self.value_numbers(&array_evaluator.evaluate(args.first()?)?);
    let bins = self.value_numbers(&array_evaluator.evaluate(args.get(1)?)?);
    let Some(counts) = frequency_counts(data, &bins) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    Some(FormulaValue::Matrix(
      counts
        .into_iter()
        .map(|count| vec![FormulaValue::Number(count as f64)])
        .collect(),
    ))
  }

  pub(crate) fn evaluate_prob(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=4).contains(&args.len()) {
      return None;
    }
    let weights = self.matrix_values(&self.evaluate(args.first()?)?);
    let probabilities = self.matrix_values(&self.evaluate(args.get(1)?)?);
    let mut lower = self.number(&self.evaluate(args.get(2)?)?)?;
    let mut upper = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(lower);
    if lower > upper {
      std::mem::swap(&mut lower, &mut upper);
    }
    let shape = matrix_dimensions(&weights);
    if shape != matrix_dimensions(&probabilities) || shape.0 == 0 || shape.1 == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    }
    let mut sum = 0.0;
    let mut result = 0.0;
    for (weight_row, probability_row) in weights.iter().zip(&probabilities) {
      for (weight, probability) in weight_row.iter().zip(probability_row) {
        let Some(weight) = matrix_stat_number(weight) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        let Some(probability) = matrix_stat_number(probability) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        if !(0.0..=1.0).contains(&probability) {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        }
        sum += probability;
        if weight >= lower && weight <= upper {
          result += probability;
        }
      }
    }
    if (sum - 1.0).abs() > 1.0e-7 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    Some(FormulaValue::Number(result))
  }

  pub(crate) fn evaluate_mdeterm(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let matrix = self.matrix_values(&self.evaluate(args.first()?)?);
    let rows = matrix.len();
    let columns = matrix.first().map_or(0, Vec::len);
    if rows == 0 || rows != columns || matrix.iter().any(|row| row.len() != columns) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut values = Vec::with_capacity(rows);
    for row in matrix {
      let mut out = Vec::with_capacity(columns);
      for value in row {
        let Some(number) = matrix_stat_number(&value) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        out.push(number);
      }
      values.push(out);
    }
    Some(FormulaValue::Number(determinant(values)))
  }

  pub(crate) fn evaluate_minverse(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let matrix = self.matrix_values(&self.evaluate(args.first()?)?);
    let rows = matrix.len();
    let columns = matrix.first().map_or(0, Vec::len);
    if rows == 0 || rows != columns || matrix.iter().any(|row| row.len() != columns) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut values = vec![vec![0.0; columns]; rows];
    for row in 0..rows {
      for column in 0..columns {
        let Some(number) = matrix_stat_number(&matrix[row][column]) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        values[row][column] = number;
      }
    }
    let Some(decomposition) = lup_decompose(&mut values) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let mut inverse = vec![vec![0.0; columns]; rows];
    for column in 0..columns {
      let mut rhs = vec![0.0; rows];
      rhs[column] = 1.0;
      let Some(solution) = lup_solve(&values, &decomposition, &rhs) else {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      };
      for row in 0..rows {
        inverse[row][column] = solution[row];
      }
    }
    Some(FormulaValue::Matrix(
      inverse
        .into_iter()
        .map(|row| row.into_iter().map(FormulaValue::Number).collect())
        .collect(),
    ))
  }

  pub(crate) fn evaluate_munit(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(size) = self.number_arg(args, 0).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if size < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let size = size as usize;
    let mut matrix = Vec::with_capacity(size);
    for row in 0..size {
      let mut values = Vec::with_capacity(size);
      for column in 0..size {
        values.push(FormulaValue::Number(if row == column { 1.0 } else { 0.0 }));
      }
      matrix.push(values);
    }
    Some(FormulaValue::Matrix(matrix))
  }

  pub(crate) fn evaluate_varpa(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = match self.stvar_text_as_zero_values(args) {
      Ok(values) => values,
      Err(error) => return Some(FormulaValue::Error(error)),
    };
    variance_slice(&values, false)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown)))
  }

  pub(crate) fn evaluate_vara(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = match self.stvar_text_as_zero_values(args) {
      Ok(values) => values,
      Err(error) => return Some(FormulaValue::Error(error)),
    };
    variance_slice(&values, true)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown)))
  }

  pub(crate) fn evaluate_mina(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let array_evaluator = self.with_array_context();
    array_evaluator
      .values(args)
      .map(|value| array_evaluator.number(&value).unwrap_or(0.0))
      .reduce(f64::min)
      .map(FormulaValue::Number)
  }

  pub(crate) fn evaluate_maxa(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let array_evaluator = self.with_array_context();
    array_evaluator
      .values(args)
      .map(|value| array_evaluator.number(&value).unwrap_or(0.0))
      .reduce(f64::max)
      .map(FormulaValue::Number)
  }

  pub(crate) fn evaluate_trimmean(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let values = self.value_numbers(&self.evaluate(args.first()?)?);
    let alpha = self.number(&self.evaluate(args.get(1)?)?)?;
    if !(0.0..1.0).contains(&alpha) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if values.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    trim_mean(values, alpha)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_large_small(
    &self,
    args: &[FormulaAst<'doc>],
    large: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let mut values = self.value_numbers(&value);
    if values.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let Some(rank_value) = args.get(1).and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    values.sort_by(f64::total_cmp);
    let rank_matrix = self.matrix_values(&rank_value);
    let rows = rank_matrix.len();
    let columns = rank_matrix.first().map_or(0, Vec::len);
    if rows == 0 || columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut result = Vec::with_capacity(rows);
    for row in rank_matrix {
      let mut result_row = Vec::with_capacity(columns);
      for rank in row {
        let value = match rank {
          FormulaValue::Error(error) => FormulaValue::Error(error),
          value => {
            let Some(k) = self.number(&value).map(|value| value.floor() as usize) else {
              return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
            };
            if k == 0 || k > values.len() {
              FormulaValue::Error(FormulaErrorValue::IllegalArgument)
            } else {
              let index = if large { values.len() - k } else { k - 1 };
              FormulaValue::Number(values[index])
            }
          }
        };
        result_row.push(value);
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn evaluate_stdeva(
    &self,
    args: &[FormulaAst<'doc>],
    sample: bool,
  ) -> Option<FormulaValue<'doc>> {
    let values = match self.stvar_text_as_zero_values(args) {
      Ok(values) => values,
      Err(error) => return Some(FormulaValue::Error(error)),
    };
    variance_slice(&values, sample)
      .map(|value| FormulaValue::Number(value.sqrt()))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Div0)))
  }

  pub(crate) fn stvar_text_as_zero_values(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> std::result::Result<Vec<f64>, FormulaErrorValue> {
    let mut values = Vec::new();
    for arg in args {
      let ranges = self.reference_ranges_from_ast(arg);
      if !ranges.is_empty() {
        for range in ranges {
          self.push_stvar_range_values(&range, &mut values)?;
        }
        continue;
      }
      match self.evaluate(arg).ok_or(FormulaErrorValue::Unknown)? {
        FormulaValue::Reference(reference) => {
          self.push_stvar_range_values(&reference, &mut values)?
        }
        FormulaValue::RefList(ranges) => {
          for range in ranges {
            self.push_stvar_range_values(&range, &mut values)?;
          }
        }
        FormulaValue::Matrix(rows) => {
          for value in rows.into_iter().flatten() {
            self.push_stvar_matrix_value(value, &mut values)?;
          }
        }
        value => self.push_stvar_direct_value(value, &mut values)?,
      }
    }
    Ok(values)
  }

  pub(crate) fn push_stvar_range_values(
    &self,
    reference: &QualifiedRange<'doc>,
    values: &mut Vec<f64>,
  ) -> std::result::Result<(), FormulaErrorValue> {
    for value in self.range_values(reference) {
      match value {
        FormulaValue::Number(value) => values.push(value),
        FormulaValue::Boolean(value) => values.push(if value { 1.0 } else { 0.0 }),
        FormulaValue::String(_) => values.push(0.0),
        FormulaValue::Error(error) => return Err(error),
        FormulaValue::Blank
        | FormulaValue::Matrix(_)
        | FormulaValue::Reference(_)
        | FormulaValue::RefList(_) => {}
      }
    }
    Ok(())
  }

  pub(crate) fn push_stvar_matrix_value(
    &self,
    value: FormulaValue<'doc>,
    values: &mut Vec<f64>,
  ) -> std::result::Result<(), FormulaErrorValue> {
    match value {
      FormulaValue::Number(value) => values.push(value),
      FormulaValue::Boolean(value) => values.push(if value { 1.0 } else { 0.0 }),
      FormulaValue::String(_) | FormulaValue::Blank => values.push(0.0),
      FormulaValue::Error(error) => return Err(error),
      FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => {}
    }
    Ok(())
  }

  pub(crate) fn push_stvar_direct_value(
    &self,
    value: FormulaValue<'doc>,
    values: &mut Vec<f64>,
  ) -> std::result::Result<(), FormulaErrorValue> {
    match value {
      FormulaValue::Number(value) => values.push(value),
      FormulaValue::Boolean(value) => values.push(if value { 1.0 } else { 0.0 }),
      FormulaValue::String(_) => values.push(0.0),
      FormulaValue::Blank => values.push(0.0),
      FormulaValue::Error(error) => return Err(error),
      FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => {}
    }
    Ok(())
  }

  pub(crate) fn evaluate_devsq(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = match self.numeric_aggregate(args, false) {
      Ok(aggregate) => aggregate.values,
      Err(error) => return Some(FormulaValue::Error(error)),
    };
    if values.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    deviation_sum_squares(&values).map(FormulaValue::Number)
  }

  pub(crate) fn evaluate_avedev(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = self.numeric_args(args);
    if values.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    Some(FormulaValue::Number(
      values.iter().map(|value| (value - mean).abs()).sum::<f64>() / values.len() as f64,
    ))
  }

  pub(crate) fn evaluate_averagea(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = self
      .values(args)
      .map(|value| self.number(&value).unwrap_or(0.0))
      .collect::<Vec<_>>();
    if values.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    Some(FormulaValue::Number(
      values.iter().sum::<f64>() / values.len() as f64,
    ))
  }

  pub(crate) fn evaluate_gcd_lcm(
    &self,
    args: &[FormulaAst<'doc>],
    lcm: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let mut result = if lcm { 1.0 } else { 0.0 };
    let values = self.numeric_values(args).collect::<Vec<_>>();
    if values.iter().any(|value| *value < 0.0) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    for value in values {
      let value = approx_floor(value);
      result = if lcm {
        lcm_number(result, value)
      } else {
        gcd_number(result, value)
      };
    }
    Some(FormulaValue::Number(result))
  }

  pub(crate) fn evaluate_fact(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(value) = self.number_arg(args, 0).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if value < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number((log_gamma(value + 1.0)).exp().round()))
  }

  pub(crate) fn evaluate_fact_double(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(value) = self.number_arg(args, 0).map(f64::trunc) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if !(0.0..=300.0).contains(&value) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut result = 1.0;
    let mut current = value as u64;
    while current > 1 {
      result *= current as f64;
      current = current.saturating_sub(2);
    }
    Some(FormulaValue::Number(result))
  }

  pub(crate) fn evaluate_multinomial(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let values = self.numeric_values(args).collect::<Vec<_>>();
    if values.iter().any(|value| *value < 0.0) {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let sum = values.iter().map(|value| approx_floor(*value)).sum::<f64>();
    let denominator = values
      .iter()
      .map(|value| log_gamma(approx_floor(*value) + 1.0))
      .sum::<f64>();
    Some(FormulaValue::Number(
      (log_gamma(sum + 1.0) - denominator).exp(),
    ))
  }

  pub(crate) fn evaluate_combin(
    &self,
    args: &[FormulaAst<'doc>],
    repetition: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(count) = self.number_arg(args, 0).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(chosen) = self.number_arg(args, 1).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    match combination_count(count, chosen, repetition, log_gamma) {
      Some(value) => Some(FormulaValue::Number(value)),
      None => Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    }
  }

  pub(crate) fn evaluate_permut(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let count = self.number(&self.evaluate(args.first()?)?)?.floor();
    let chosen = self.number(&self.evaluate(args.get(1)?)?)?.floor();
    match permutation_count(count, chosen, log_gamma) {
      Some(value) => Some(FormulaValue::Number(value)),
      None => Some(FormulaValue::Error(FormulaErrorValue::Num)),
    }
  }

  pub(crate) fn evaluate_permutationa(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let left = self.evaluate(args.first()?)?;
    let right = self.evaluate(args.get(1)?)?;
    if matches!(left, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
      || matches!(right, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
    {
      return self.map_binary_values(left, right, |evaluator, left, right| {
        Some(permutationa_value(
          evaluator.number(left)?,
          evaluator.number(right)?,
        ))
      });
    }
    Some(permutationa_value(
      self.number(&left)?,
      self.number(&right)?,
    ))
  }

  pub(crate) fn evaluate_mround(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 || args.iter().any(is_missing_argument) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(number) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(multiple) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    Some(FormulaValue::Number(mround(number, multiple)))
  }

  pub(crate) fn evaluate_quotient(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let numerator = self.number(&self.evaluate(args.first()?)?)?;
    let denominator = self.number(&self.evaluate(args.get(1)?)?)?;
    match quotient(numerator, denominator) {
      Ok(result) => Some(FormulaValue::Number(result)),
      Err(error) => Some(FormulaValue::Error(numeric_error_value(error))),
    }
  }

  pub(crate) fn evaluate_pmt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=5).contains(&args.len()) {
      return None;
    }
    let rate = self.number(&self.evaluate(args.first()?)?)?;
    let nper = self.number(&self.evaluate(args.get(1)?)?)?;
    let pv = self.number(&self.evaluate(args.get(2)?)?)?;
    let fv = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let pay_in_advance = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    let result = financial_pmt(rate, nper, pv, fv, pay_in_advance);
    if result.is_finite() {
      Some(FormulaValue::Number(result))
    } else {
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    }
  }

  pub(crate) fn evaluate_fv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=5).contains(&args.len()) {
      return None;
    }
    let rate = self.number(&self.evaluate(args.first()?)?)?;
    let nper = self.number(&self.evaluate(args.get(1)?)?)?;
    let payment = self.number(&self.evaluate(args.get(2)?)?)?;
    let pv = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let pay_in_advance = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    Some(FormulaValue::Number(financial_fv(
      rate,
      nper,
      payment,
      pv,
      pay_in_advance,
    )))
  }

  pub(crate) fn evaluate_npv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(rate) = self.npv_scalar_number(&self.evaluate(args.first()?)?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let mut period = 1i32;
    let mut result = 0.0;
    for arg in &args[1..] {
      let values = match self.npv_values_from_ast(arg) {
        Ok(values) => values,
        Err(error) => return Some(FormulaValue::Error(error)),
      };
      for number in values {
        result += number / (1.0 + rate).powi(period);
        period += 1;
      }
    }
    Some(FormulaValue::Number(result))
  }

  pub(crate) fn npv_values_from_ast(
    &self,
    ast: &FormulaAst<'doc>,
  ) -> std::result::Result<Vec<f64>, FormulaErrorValue> {
    let ranges = self.reference_ranges_from_ast(ast);
    if !ranges.is_empty() {
      let mut values = Vec::new();
      for range in ranges {
        values.extend(self.horizontal_range_numbers(&range)?);
      }
      return Ok(values);
    }
    match self.evaluate(ast).ok_or(FormulaErrorValue::Unknown)? {
      FormulaValue::Reference(reference) => self.horizontal_range_numbers(&reference),
      FormulaValue::RefList(ranges) => {
        let mut values = Vec::new();
        for range in ranges {
          values.extend(self.horizontal_range_numbers(&range)?);
        }
        Ok(values)
      }
      FormulaValue::Matrix(rows) => self.npv_matrix_numbers(&rows),
      value => self
        .npv_scalar_number(&value)
        .map(|value| vec![value])
        .ok_or(FormulaErrorValue::IllegalArgument),
    }
  }

  pub(crate) fn horizontal_range_numbers(
    &self,
    reference: &QualifiedRange<'doc>,
  ) -> std::result::Result<Vec<f64>, FormulaErrorValue> {
    let mut values = Vec::new();
    for (_address, value) in self.range_cells(reference) {
      match value {
        FormulaValue::Number(value) => values.push(value),
        FormulaValue::Boolean(value) => values.push(if value { 1.0 } else { 0.0 }),
        FormulaValue::Error(error) => return Err(error),
        FormulaValue::String(_)
        | FormulaValue::Blank
        | FormulaValue::Matrix(_)
        | FormulaValue::Reference(_)
        | FormulaValue::RefList(_) => {}
      }
    }
    Ok(values)
  }

  pub(crate) fn npv_matrix_numbers(
    &self,
    rows: &[Vec<FormulaValue<'doc>>],
  ) -> std::result::Result<Vec<f64>, FormulaErrorValue> {
    let columns = rows.iter().map(Vec::len).max().unwrap_or(0);
    if rows.is_empty() || columns == 0 {
      return Err(FormulaErrorValue::IllegalArgument);
    }
    let mut values = Vec::new();
    for column in 0..columns {
      for row in rows {
        let Some(value) = row.get(column) else {
          return Err(FormulaErrorValue::IllegalArgument);
        };
        let Some(number) = self.npv_scalar_number(value) else {
          return Err(match value {
            FormulaValue::Error(error) => *error,
            _ => FormulaErrorValue::IllegalArgument,
          });
        };
        values.push(number);
      }
    }
    Ok(values)
  }

  pub(crate) fn npv_scalar_number(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    match self.first_value(value) {
      FormulaValue::Number(value) => Some(value),
      FormulaValue::Boolean(value) => Some(if value { 1.0 } else { 0.0 }),
      _ => None,
    }
  }

  pub(crate) fn evaluate_fvschedule(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    if args.iter().any(is_missing_argument) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let mut value = self.number(&self.evaluate(args.first()?)?)?;
    let schedule = self.evaluate(args.get(1)?)?;
    if matches!(schedule, FormulaValue::Blank) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    for rate in self.value_numbers(&schedule) {
      value *= 1.0 + rate;
    }
    Some(FormulaValue::Number(value))
  }

  pub(crate) fn evaluate_effect(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let nominal = self.number(&self.evaluate(args.first()?)?)?;
    let periods = self.number(&self.evaluate(args.get(1)?)?)?.floor();
    if nominal <= 0.0 || periods < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(
      (1.0 + nominal / periods).powf(periods) - 1.0,
    ))
  }

  pub(crate) fn evaluate_rate(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=6).contains(&args.len()) {
      return None;
    }
    let nper = self.number(&self.evaluate(args.first()?)?)?;
    let payment = self.number(&self.evaluate(args.get(1)?)?)?;
    let pv = self.number(&self.evaluate(args.get(2)?)?)?;
    let fv = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let pay_type = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    let guess = args
      .get(5)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.1);
    if nper <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    match financial_rate(nper, payment, pv, fv, pay_type, guess, args.len() != 6) {
      Some(value) => Some(FormulaValue::Number(value)),
      None => Some(FormulaValue::Error(FormulaErrorValue::Num)),
    }
  }

  pub(crate) fn evaluate_ispmt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 4 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let rate = self.number(&self.evaluate(args.first()?)?)?;
    let period = self.number(&self.evaluate(args.get(1)?)?)?;
    let total = self.number(&self.evaluate(args.get(2)?)?)?;
    let investment = self.number(&self.evaluate(args.get(3)?)?)?;
    let result = investment * rate * (period / total - 1.0);
    if result.is_finite() {
      Some(FormulaValue::Number(result))
    } else {
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    }
  }

  pub(crate) fn evaluate_ipmt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=6).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(rate) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(period) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(nper) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(pv) = self.number_arg(args, 3) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let fv = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let pay_in_advance = args
      .get(5)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    if period < 1.0 || period > nper {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let (interest, _) = financial_ipmt(rate, period, nper, pv, fv, pay_in_advance);
    Some(FormulaValue::Number(interest))
  }

  pub(crate) fn evaluate_ppmt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=6).contains(&args.len()) {
      return None;
    }
    let rate = self.number(&self.evaluate(args.first()?)?)?;
    let period = self.number(&self.evaluate(args.get(1)?)?)?;
    let nper = self.number(&self.evaluate(args.get(2)?)?)?;
    let pv = self.number(&self.evaluate(args.get(3)?)?)?;
    let fv = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    let pay_in_advance = args
      .get(5)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    if period < 1.0 || period > nper {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let (interest, payment) = financial_ipmt(rate, period, nper, pv, fv, pay_in_advance);
    Some(FormulaValue::Number(payment - interest))
  }

  pub(crate) fn evaluate_cumipmt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    self.evaluate_cum_interest_principal(args, true)
  }

  pub(crate) fn evaluate_cumprinc(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    self.evaluate_cum_interest_principal(args, false)
  }

  pub(crate) fn evaluate_cum_interest_principal(
    &self,
    args: &[FormulaAst<'doc>],
    interest: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 6 {
      return None;
    }
    if args.iter().any(is_missing_argument) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let rate = self.number(&self.evaluate(args.first()?)?)?;
    let nper = self.number(&self.evaluate(args.get(1)?)?)?;
    let pv = self.number(&self.evaluate(args.get(2)?)?)?;
    let start = approx_floor(self.number(&self.evaluate(args.get(3)?)?)?);
    let end = approx_floor(self.number(&self.evaluate(args.get(4)?)?)?);
    let flag = self.number(&self.evaluate(args.get(5)?)?)?;
    if start < 1.0
      || end < start
      || rate <= 0.0
      || end > nper
      || nper <= 0.0
      || pv <= 0.0
      || (flag != 0.0 && flag != 1.0)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(financial_cum(
      rate,
      nper,
      pv,
      start as u64,
      end as u64,
      flag != 0.0,
      interest,
    )))
  }

  pub(crate) fn evaluate_xnpv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let rate = self.number(&self.evaluate(args.first()?)?)?;
    let values = self.value_numbers(&self.evaluate(args.get(1)?)?);
    let dates = self.value_numbers(&self.evaluate(args.get(2)?)?);
    financial_xnpv(rate, &values, &dates)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_xirr(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let values = self.value_numbers(&self.evaluate(args.first()?)?);
    let dates = self.value_numbers(&self.evaluate(args.get(1)?)?);
    let guess = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.1);
    financial_xirr(&values, &dates, guess)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_irr(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=2).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let values = self.value_numbers_from_ast(args.first()?);
    let guess = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.1);
    financial_irr(&values, guess)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Num)))
  }

  pub(crate) fn evaluate_mirr(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let values = self.value_numbers(&self.evaluate(args.first()?)?);
    let finance_rate = self.number(&self.evaluate(args.get(1)?)?)?;
    let reinvest_rate = self.number(&self.evaluate(args.get(2)?)?)?;
    financial_mirr(&values, finance_rate, reinvest_rate)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_euroconvert(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if !(3..=5).contains(&args.len()) {
      return None;
    }
    if args.get(4).is_some_and(is_missing_argument)
      || (args.len() == 4 && args.get(3).is_some_and(is_missing_argument))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let from = self.text(&self.evaluate(args.get(1)?)?);
    let to = self.text(&self.evaluate(args.get(2)?)?);
    let full_precision = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    let precision = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(approx_floor)
      .unwrap_or(0.0);
    if precision != 0.0 && precision < 3.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    euro_convert(value, &from, &to, full_precision, precision)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_bahttext(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    if self.array_context && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
      return self.map_unary_values(value, |evaluator, value| {
        evaluator
          .number(value)
          .map(|value| FormulaValue::String(Cow::Owned(baht_text(value))))
          .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown)))
      });
    }
    let Some(value) = self.number(&value) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    Some(FormulaValue::String(Cow::Owned(baht_text(value))))
  }

  pub(crate) fn evaluate_regex(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=4).contains(&args.len()) {
      return None;
    }
    let text = self.text(&self.evaluate(args.first()?)?);
    let pattern = self.text(&self.evaluate(args.get(1)?)?);
    let replacement = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.text(&value));
    let mut global = false;
    let mut occurrence = 1usize;
    if let Some(arg) = args.get(3) {
      let value = self.evaluate(arg)?;
      match value {
        FormulaValue::String(flags) => {
          if flags.as_ref() == "g" {
            global = true;
          } else if !flags.is_empty() {
            return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
          }
        }
        value => {
          let number = self.number(&value)?;
          if number < 0.0 {
            return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
          }
          occurrence = number.floor() as usize;
        }
      }
    }
    if occurrence == 0 {
      return Some(FormulaValue::String(Cow::Owned(text)));
    }
    let regex = match RegexBuilder::new(&pattern).build() {
      Ok(regex) => regex,
      Err(_) => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    };
    if let Some(replacement) = replacement {
      if global {
        return Some(FormulaValue::String(Cow::Owned(
          regex.replace_all(&text, replacement.as_str()).into_owned(),
        )));
      }
      if occurrence == 1 {
        return Some(FormulaValue::String(Cow::Owned(
          regex.replace(&text, replacement.as_str()).into_owned(),
        )));
      }
      let mut count = 0usize;
      let result = regex
        .replace_all(&text, |captures: &regex::Captures<'_>| {
          count += 1;
          if count == occurrence {
            replacement.clone()
          } else {
            captures
              .get(0)
              .map(|mat| mat.as_str().to_string())
              .unwrap_or_default()
          }
        })
        .into_owned();
      return Some(FormulaValue::String(Cow::Owned(result)));
    }
    regex
      .find_iter(&text)
      .nth(occurrence - 1)
      .map(|mat| FormulaValue::String(Cow::Owned(mat.as_str().to_string())))
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  pub(crate) fn evaluate_encodeurl(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let text = self.text(&self.evaluate(args.first()?)?);
    if text.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut output = String::with_capacity(text.len());
    for byte in text.bytes() {
      if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_') {
        output.push(byte as char);
      } else {
        output.push_str(&format!("%{byte:02X}"));
      }
    }
    Some(FormulaValue::String(Cow::Owned(output)))
  }

  pub(crate) fn evaluate_rot13(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let text = self.text(&self.evaluate(args.first()?)?);
    let result = text
      .chars()
      .map(|ch| match ch {
        'a'..='z' => ((((ch as u8 - b'a') + 13) % 26) + b'a') as char,
        'A'..='Z' => ((((ch as u8 - b'A') + 13) % 26) + b'A') as char,
        _ => ch,
      })
      .collect();
    Some(FormulaValue::String(Cow::Owned(result)))
  }

  pub(crate) fn evaluate_nominal(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let effective = self.number(&self.evaluate(args.first()?)?)?;
    let periods = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    if periods < 1.0 || effective <= 0.0 {
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    } else {
      Some(FormulaValue::Number(
        ((effective + 1.0).powf(1.0 / periods) - 1.0) * periods,
      ))
    }
  }

  pub(crate) fn evaluate_sln(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let cost = self.number(&self.evaluate(args.first()?)?)?;
    let salvage = self.number(&self.evaluate(args.get(1)?)?)?;
    let life = self.number(&self.evaluate(args.get(2)?)?)?;
    if life == 0.0 {
      Some(FormulaValue::Error(FormulaErrorValue::Div0))
    } else {
      Some(FormulaValue::Number((cost - salvage) / life))
    }
  }

  pub(crate) fn evaluate_syd(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 4 {
      return None;
    }
    let cost = self.number(&self.evaluate(args.first()?)?)?;
    let salvage = self.number(&self.evaluate(args.get(1)?)?)?;
    let life = self.number(&self.evaluate(args.get(2)?)?)?;
    let period = self.number(&self.evaluate(args.get(3)?)?)?;
    if life <= 0.0 || period <= 0.0 || period > life + 1.0 {
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    } else {
      Some(FormulaValue::Number(
        (cost - salvage) * (life - period + 1.0) * 2.0 / (life * (life + 1.0)),
      ))
    }
  }

  pub(crate) fn evaluate_ddb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=5).contains(&args.len()) {
      return None;
    }
    let cost = self.number(&self.evaluate(args.first()?)?)?;
    let salvage = self.number(&self.evaluate(args.get(1)?)?)?;
    let life = self.number(&self.evaluate(args.get(2)?)?)?;
    let period = self.number(&self.evaluate(args.get(3)?)?)?;
    let factor = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(2.0);
    if cost < 0.0
      || salvage < 0.0
      || factor <= 0.0
      || salvage > cost
      || period < 1.0
      || period > life
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(financial_ddb(
      cost, salvage, life, period, factor,
    )))
  }

  pub(crate) fn evaluate_vdb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(5..=7).contains(&args.len()) {
      return None;
    }
    let cost = self.number(&self.evaluate(args.first()?)?)?;
    let salvage = self.number(&self.evaluate(args.get(1)?)?)?;
    let life = self.number(&self.evaluate(args.get(2)?)?)?;
    let start = self.number(&self.evaluate(args.get(3)?)?)?;
    let end = self.number(&self.evaluate(args.get(4)?)?)?;
    let factor = args
      .get(5)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(2.0);
    let no_switch = args
      .get(6)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    if start < 0.0 || end < start || end > life || cost < 0.0 || salvage > cost || factor <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(financial_vdb(
      cost, salvage, life, start, end, factor, no_switch,
    )))
  }

  pub(crate) fn evaluate_skew(
    &self,
    args: &[FormulaAst<'doc>],
    population: bool,
  ) -> Option<FormulaValue<'doc>> {
    let values = self.numeric_args(args);
    match skewness(&values, population) {
      Ok(value) => Some(FormulaValue::Number(value)),
      Err(error) => Some(FormulaValue::Error(statistics_error_value(error))),
    }
  }

  pub(crate) fn evaluate_geo_har_mean(
    &self,
    args: &[FormulaAst<'doc>],
    harmonic: bool,
  ) -> Option<FormulaValue<'doc>> {
    let mut count = 0.0;
    let mut total = 0.0;
    for value in self.numeric_values(args) {
      if value < 0.0 || (harmonic && value == 0.0) {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      if value == 0.0 {
        return Some(FormulaValue::Number(0.0));
      }
      count += 1.0;
      total += if harmonic { value.recip() } else { value.ln() };
    }
    if count == 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    if harmonic {
      Some(FormulaValue::Number(count / total))
    } else {
      Some(FormulaValue::Number((total / count).exp()))
    }
  }

  pub(crate) fn evaluate_sumif(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    if let Some(sum_range) = args.get(2) {
      self.evaluate_ifs(Some(sum_range), &args[..2], IfsAggregate::Sum)
    } else {
      self.evaluate_ifs(Some(&args[0]), &args[..2], IfsAggregate::Sum)
    }
  }

  pub(crate) fn evaluate_countif(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    if let Some(value) = self.evaluate_countif_ref_list(args) {
      return Some(value);
    }
    self.evaluate_ifs(None, args, IfsAggregate::Count)
  }

  pub(crate) fn evaluate_countif_ref_list(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let ranges = self.reference_ranges_from_ast(args.first()?);
    if ranges.is_empty() {
      return None;
    }
    let criterion = QueryParam::from_criterion(self, &self.evaluate(args.get(1)?)?, 0);
    let mut count = 0.0;
    for range in ranges {
      let sheet = self.range_sheet(&range);
      let range = if criterion
        .entries
        .first()
        .is_some_and(|entry| !entry.item.match_empty)
      {
        self
          .book
          .data_area_subrange(sheet, range.range)
          .map(|data_area| QualifiedRange {
            range: data_area,
            ..range.clone()
          })
          .unwrap_or(range)
      } else {
        range
      };
      for (address, value) in self.range_cells(&range) {
        let value = self.book.query_cell_value(sheet, address, value);
        if criterion.matches_value(self, &value, self.book.is_query_empty_cell(sheet, address)) {
          count += 1.0;
        }
      }
    }
    Some(FormulaValue::Number(count))
  }

  pub(crate) fn evaluate_averageif(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    if let Some(average_range) = args.get(2) {
      self.evaluate_ifs(Some(average_range), &args[..2], IfsAggregate::Average)
    } else {
      self.evaluate_ifs(Some(&args[0]), &args[..2], IfsAggregate::Average)
    }
  }

  pub(crate) fn evaluate_sumifs(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 || args.len().is_multiple_of(2) {
      return None;
    }
    self.evaluate_ifs(Some(&args[0]), &args[1..], IfsAggregate::Sum)
  }

  pub(crate) fn evaluate_countifs(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 || !args.len().is_multiple_of(2) {
      return None;
    }
    self.evaluate_ifs(None, args, IfsAggregate::Count)
  }

  pub(crate) fn evaluate_averageifs(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 || args.len().is_multiple_of(2) {
      return None;
    }
    self.evaluate_ifs(Some(&args[0]), &args[1..], IfsAggregate::Average)
  }

  pub(crate) fn evaluate_minmaxifs(
    &self,
    args: &[FormulaAst<'doc>],
    max: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 || args.len().is_multiple_of(2) {
      return None;
    }
    self.evaluate_ifs(
      Some(&args[0]),
      &args[1..],
      if max {
        IfsAggregate::Max
      } else {
        IfsAggregate::Min
      },
    )
  }

  pub(crate) fn evaluate_database_function(
    &self,
    args: &[FormulaAst<'doc>],
    function: DatabaseFunction,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let Some(database) = self.query_grid_from_ast(args.first()?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(criteria) = self.query_grid_from_ast(args.get(2)?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    if database.values.len() < 2
      || database.values.first().is_none_or(Vec::is_empty)
      || criteria.values.len() < 2
      || criteria.values.first().is_none_or(Vec::is_empty)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let field = match self.database_field_index(args.get(1)?, &database.values[0], function) {
      Some(field) => field,
      None => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    };
    let rows = self.database_matching_rows(&database, &criteria);
    if field.is_none() && matches!(function, DatabaseFunction::Count | DatabaseFunction::CountA) {
      return Some(FormulaValue::Number(rows.len() as f64));
    }
    let Some(field) = field else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    if field >= database.values[0].len() {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }

    let mut values = Vec::new();
    let mut text_values = Vec::new();
    for row in rows {
      let value = row.get(field).cloned().unwrap_or_default();
      match function {
        DatabaseFunction::Count => {
          if formula_cell_numeric_value(&value).is_some() {
            values.push(1.0);
          }
        }
        DatabaseFunction::CountA => {
          if !matches!(value, FormulaValue::Blank) {
            values.push(1.0);
          }
        }
        DatabaseFunction::Get => {
          if !matches!(value, FormulaValue::Blank) {
            text_values.push(value);
          }
        }
        _ => {
          if let Some(number) = formula_cell_numeric_value(&value) {
            values.push(number);
          }
        }
      }
    }

    match function {
      DatabaseFunction::Count | DatabaseFunction::CountA => {
        Some(FormulaValue::Number(values.len() as f64))
      }
      DatabaseFunction::Sum => Some(FormulaValue::Number(kahan_sum(values.iter().copied()))),
      DatabaseFunction::Average if values.is_empty() => {
        Some(FormulaValue::Error(FormulaErrorValue::Div0))
      }
      DatabaseFunction::Average => Some(FormulaValue::Number(
        kahan_sum(values.iter().copied()) / values.len() as f64,
      )),
      DatabaseFunction::Get => match text_values.as_slice() {
        [value] => Some(value.clone()),
        [] => Some(FormulaValue::Error(FormulaErrorValue::Value)),
        _ => Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
      },
      DatabaseFunction::Max => Some(FormulaValue::Number(
        values.into_iter().reduce(f64::max).unwrap_or(0.0),
      )),
      DatabaseFunction::Min => Some(FormulaValue::Number(
        values.into_iter().reduce(f64::min).unwrap_or(0.0),
      )),
      DatabaseFunction::Product => Some(FormulaValue::Number(if values.is_empty() {
        0.0
      } else {
        values.into_iter().product()
      })),
      DatabaseFunction::Var => variance_slice(&values, true)
        .map(FormulaValue::Number)
        .or(Some(FormulaValue::Error(FormulaErrorValue::Div0))),
      DatabaseFunction::VarP => variance_slice(&values, false)
        .map(FormulaValue::Number)
        .or(Some(FormulaValue::Error(FormulaErrorValue::Div0))),
      DatabaseFunction::StdDev => variance_slice(&values, true)
        .map(|value| FormulaValue::Number(value.sqrt()))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Div0))),
      DatabaseFunction::StdDevP => variance_slice(&values, false)
        .map(|value| FormulaValue::Number(value.sqrt()))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Div0))),
    }
  }

  pub(crate) fn database_field_index(
    &self,
    field_arg: &FormulaAst<'doc>,
    headers: &[FormulaValue<'doc>],
    function: DatabaseFunction,
  ) -> Option<Option<usize>> {
    let value = self.evaluate(field_arg)?;
    let allow_missing = matches!(function, DatabaseFunction::Count | DatabaseFunction::CountA);
    match self.first_value(&value) {
      FormulaValue::Blank if allow_missing => Some(None),
      FormulaValue::Number(value) if allow_missing && value.floor() == 0.0 => Some(None),
      FormulaValue::Number(value) => {
        let index = value.floor() as i64 - 1;
        (index >= 0).then_some(Some(index as usize))
      }
      FormulaValue::String(name) => headers
        .iter()
        .position(|header| self.text(header).eq_ignore_ascii_case(name.trim()))
        .map(Some)
        .or(Some(Some(usize::MAX))),
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_) => {
        if allow_missing {
          Some(None)
        } else {
          None
        }
      }
      FormulaValue::Boolean(_) | FormulaValue::Error(_) | FormulaValue::Blank => None,
    }
  }

  pub(crate) fn database_matching_rows<'b>(
    &self,
    database: &'b QueryGrid<'doc>,
    criteria: &QueryGrid<'doc>,
  ) -> Vec<&'b [FormulaValue<'doc>]> {
    let params = self.database_query_params(&database.values[0], criteria);
    database
      .values
      .iter()
      .zip(database.query_empty.iter())
      .skip(1)
      .filter(|(row, query_empty)| {
        params
          .iter()
          .any(|param| param.matches_row_with_empty(self, row, query_empty))
      })
      .map(|(row, _)| row.as_slice())
      .collect()
  }

  pub(crate) fn database_query_params(
    &self,
    headers: &[FormulaValue<'doc>],
    criteria: &QueryGrid<'doc>,
  ) -> Vec<QueryParam<'doc>> {
    if let Some(params) = self.database_star_query_params(headers, criteria)
      && !params.is_empty()
    {
      return params;
    }
    let Some(criteria_headers) = criteria.values.first() else {
      return Vec::new();
    };
    criteria
      .values
      .iter()
      .zip(criteria.query_empty.iter())
      .skip(1)
      .filter_map(|(criteria_row, criteria_empty)| {
        let mut entries = Vec::new();
        let mut search_type = QuerySearchType::Normal;
        let mut invalid = false;
        let row_has_present_cell = criteria_row
          .iter()
          .zip(criteria_empty.iter())
          .any(|(value, query_empty)| database_criterion_cell_present(value, *query_empty));
        for (criteria_column, criterion_value) in criteria_row.iter().enumerate() {
          if !database_criterion_entry_present(
            criterion_value,
            criteria_empty
              .get(criteria_column)
              .copied()
              .unwrap_or(false),
          ) {
            continue;
          }
          let Some(header) = criteria_headers.get(criteria_column) else {
            continue;
          };
          let header = self.text(header);
          if header.is_empty() {
            continue;
          }
          let Some(field) = headers
            .iter()
            .position(|database_header| self.text(database_header).eq_ignore_ascii_case(&header))
          else {
            invalid = true;
            break;
          };
          let (entry, entry_search_type) =
            QueryEntry::from_database_value(self, criterion_value, field);
          if search_type == QuerySearchType::Normal {
            search_type = entry_search_type;
          }
          entries.push(entry);
        }
        if invalid {
          return None;
        }
        if entries.is_empty() && !row_has_present_cell {
          return None;
        }
        Some(QueryParam {
          entries,
          search_type,
          range_lookup: false,
          match_whole_cell: self.book.formula_match_whole_cell,
          case_sensitive: false,
        })
      })
      .collect()
  }

  pub(crate) fn database_star_query_params(
    &self,
    headers: &[FormulaValue<'doc>],
    criteria: &QueryGrid<'doc>,
  ) -> Option<Vec<QueryParam<'doc>>> {
    if criteria.values.first().map_or(0, Vec::len) < 4 {
      return None;
    }
    if !criteria.values.iter().any(|row| {
      let connector = self.text(row.first().unwrap_or(&FormulaValue::Blank));
      connector.eq_ignore_ascii_case("AND") || connector.eq_ignore_ascii_case("OR")
    }) {
      return None;
    }
    let mut params = Vec::new();
    let mut current = QueryParam {
      entries: Vec::new(),
      search_type: QuerySearchType::Normal,
      range_lookup: false,
      match_whole_cell: self.book.formula_match_whole_cell,
      case_sensitive: false,
    };
    for (row_index, (row, _row_empty)) in criteria
      .values
      .iter()
      .zip(criteria.query_empty.iter())
      .enumerate()
    {
      let connector = self.text(row.first().unwrap_or(&FormulaValue::Blank));
      if row_index > 0 && connector.eq_ignore_ascii_case("OR") {
        if !current.entries.is_empty() {
          params.push(current);
        }
        current = QueryParam {
          entries: Vec::new(),
          search_type: QuerySearchType::Normal,
          range_lookup: false,
          match_whole_cell: self.book.formula_match_whole_cell,
          case_sensitive: false,
        };
      } else if row_index > 0 && !connector.is_empty() && !connector.eq_ignore_ascii_case("AND") {
        return None;
      }
      let field_name = self.text(row.get(1).unwrap_or(&FormulaValue::Blank));
      if field_name.is_empty() {
        return None;
      }
      let field = headers
        .iter()
        .position(|header| self.text(header).eq_ignore_ascii_case(&field_name))?;
      let op_text = self.text(row.get(2).unwrap_or(&FormulaValue::Blank));
      let op = match op_text.trim() {
        "" | "=" => QueryOp::Equal,
        "<>" => QueryOp::NotEqual,
        "<" => QueryOp::Less,
        "<=" => QueryOp::LessOrEqual,
        ">" => QueryOp::Greater,
        ">=" => QueryOp::GreaterOrEqual,
        _ => return None,
      };
      let criterion = row.get(3).cloned().unwrap_or_default();
      let (mut entry, search_type) = QueryEntry::from_database_value(self, &criterion, field);
      entry.op = op;
      if current.search_type == QuerySearchType::Normal {
        current.search_type = search_type;
      }
      current.entries.push(entry);
    }
    if !current.entries.is_empty() {
      params.push(current);
    }
    Some(params)
  }

  pub(crate) fn evaluate_ifs(
    &self,
    main_range: Option<&FormulaAst<'doc>>,
    criteria_args: &[FormulaAst<'doc>],
    aggregate: IfsAggregate,
  ) -> Option<FormulaValue<'doc>> {
    let mut criteria_ranges = Vec::with_capacity(criteria_args.len() / 2);
    let mut criteria_sets = Vec::with_capacity(criteria_args.len() / 2);
    let mut result_shape = (1usize, 1usize);
    let mut result_len = 1usize;
    for pair in criteria_args.chunks_exact(2) {
      let range = self.query_grid_from_ast(&pair[0])?;
      let rows = range.values.len();
      let columns = range.values.first().map_or(0, Vec::len);
      if rows == 0
        || columns == 0
        || range.values.iter().any(|row| row.len() != columns)
        || range.query_empty.iter().any(|row| row.len() != columns)
      {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      let criteria_matrix = self.matrix_values(&self.evaluate(&pair[1])?);
      let criteria_rows = criteria_matrix.len();
      let criteria_columns = criteria_matrix.first().map_or(0, Vec::len);
      if criteria_rows == 0 || criteria_columns == 0 {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      if criteria_rows * criteria_columns > 1 {
        if result_len == 1 {
          result_shape = (criteria_rows, criteria_columns);
          result_len = criteria_rows * criteria_columns;
        } else if result_shape != (criteria_rows, criteria_columns) {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        }
      }
      criteria_ranges.push(range);
      criteria_sets.push(
        criteria_matrix
          .into_iter()
          .flatten()
          .map(|value| QueryParam::from_criterion(self, &value, 0))
          .collect::<Vec<_>>(),
      );
    }

    if criteria_ranges
      .windows(2)
      .any(|window| window[0].dimensions() != window[1].dimensions())
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let dimensions = criteria_ranges.first()?.dimensions();
    let main_values = if let Some(main_range) = main_range {
      let values = self.query_grid_from_ast(main_range)?;
      if values.dimensions() != dimensions {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      Some(values)
    } else {
      None
    };

    let mut outputs = Vec::with_capacity(result_len);
    for criteria_index in 0..result_len {
      let mut count = 0.0;
      let mut sum = KahanSum::default();
      let mut minmax = None::<f64>;
      for row in 0..dimensions.0 {
        for column in 0..dimensions.1 {
          let matches_all =
            criteria_ranges
              .iter()
              .zip(criteria_sets.iter())
              .all(|(range, criteria)| {
                let criteria = if criteria.len() == 1 {
                  &criteria[0]
                } else {
                  &criteria[criteria_index]
                };
                criteria.matches_value(
                  self,
                  &range.values[row][column],
                  range.query_empty[row][column],
                )
              });
          if !matches_all {
            continue;
          }
          match aggregate {
            IfsAggregate::Count => count += 1.0,
            IfsAggregate::Sum | IfsAggregate::Average | IfsAggregate::Min | IfsAggregate::Max => {
              if let Some(number) = main_values
                .as_ref()
                .and_then(|values| formula_cell_numeric_value(&values.values[row][column]))
              {
                count += 1.0;
                sum.add(number);
                minmax = Some(match (aggregate, minmax) {
                  (IfsAggregate::Min, Some(value)) => value.min(number),
                  (IfsAggregate::Max, Some(value)) => value.max(number),
                  _ => number,
                });
              }
            }
          }
        }
      }
      outputs.push(match aggregate {
        IfsAggregate::Count => FormulaValue::Number(count),
        IfsAggregate::Sum => FormulaValue::Number(sum.finish()),
        IfsAggregate::Average if count == 0.0 => FormulaValue::Error(FormulaErrorValue::Div0),
        IfsAggregate::Average => FormulaValue::Number(sum.finish() / count),
        IfsAggregate::Min | IfsAggregate::Max => FormulaValue::Number(minmax.unwrap_or(0.0)),
      });
    }

    if result_len == 1 {
      return outputs.into_iter().next();
    }
    let mut rows = Vec::with_capacity(result_shape.0);
    let mut iter = outputs.into_iter();
    for _ in 0..result_shape.0 {
      rows.push(iter.by_ref().take(result_shape.1).collect());
    }
    Some(FormulaValue::Matrix(rows))
  }

  pub(crate) fn evaluate_ceiling(
    &self,
    args: &[FormulaAst<'doc>],
    kind: CeilingFloorKind,
  ) -> Option<FormulaValue<'doc>> {
    if (kind == CeilingFloorKind::Precise && !(1..=2).contains(&args.len()))
      || (kind != CeilingFloorKind::Precise && !(1..=3).contains(&args.len()))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    let significance = args
      .get(1)
      .filter(|arg| !is_missing_argument(arg))
      .and_then(|arg| self.evaluate(arg));
    let abs_mode = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    if self.array_context
      && (is_matrix_argument(&value) || significance.as_ref().is_some_and(is_matrix_argument))
    {
      return match significance {
        Some(significance) => {
          self.map_binary_values(value, significance, |evaluator, value, significance| {
            Some(evaluator.ceiling_value(value, Some(significance), abs_mode, kind))
          })
        }
        None => self.map_unary_values(value, |evaluator, value| {
          Some(evaluator.ceiling_value(value, None, abs_mode, kind))
        }),
      };
    }
    Some(self.ceiling_value(&value, significance.as_ref(), abs_mode, kind))
  }

  pub(crate) fn ceiling_value(
    &self,
    value: &FormulaValue<'doc>,
    significance: Option<&FormulaValue<'doc>>,
    abs_mode: bool,
    kind: CeilingFloorKind,
  ) -> FormulaValue<'doc> {
    let Some(value) = self.number(value) else {
      return FormulaValue::Error(FormulaErrorValue::Value);
    };
    let significance = significance.and_then(|value| self.number(value));
    match numeric_ceiling(value, significance, abs_mode, kind) {
      Ok(result) => FormulaValue::Number(result),
      Err(error) => FormulaValue::Error(numeric_error_value(error)),
    }
  }

  pub(crate) fn evaluate_floor(
    &self,
    args: &[FormulaAst<'doc>],
    kind: CeilingFloorKind,
  ) -> Option<FormulaValue<'doc>> {
    if (kind == CeilingFloorKind::Precise && !(1..=2).contains(&args.len()))
      || (kind != CeilingFloorKind::Precise && !(1..=3).contains(&args.len()))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    let significance = args
      .get(1)
      .filter(|arg| !is_missing_argument(arg))
      .and_then(|arg| self.evaluate(arg));
    let abs_mode = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    if self.array_context
      && (is_matrix_argument(&value) || significance.as_ref().is_some_and(is_matrix_argument))
    {
      return match significance {
        Some(significance) => {
          self.map_binary_values(value, significance, |evaluator, value, significance| {
            Some(evaluator.floor_value(value, Some(significance), abs_mode, kind))
          })
        }
        None => self.map_unary_values(value, |evaluator, value| {
          Some(evaluator.floor_value(value, None, abs_mode, kind))
        }),
      };
    }
    Some(self.floor_value(&value, significance.as_ref(), abs_mode, kind))
  }

  pub(crate) fn floor_value(
    &self,
    value: &FormulaValue<'doc>,
    significance: Option<&FormulaValue<'doc>>,
    abs_mode: bool,
    kind: CeilingFloorKind,
  ) -> FormulaValue<'doc> {
    let Some(value) = self.number(value) else {
      return FormulaValue::Error(FormulaErrorValue::Value);
    };
    let significance = significance.and_then(|value| self.number(value));
    match numeric_floor(value, significance, abs_mode, kind) {
      Ok(result) => FormulaValue::Number(result),
      Err(error) => FormulaValue::Error(numeric_error_value(error)),
    }
  }

  pub(crate) fn evaluate_ceiling_excel_legacy(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let significance = self.number(&self.evaluate(args.get(1)?)?)?;
    match ceiling_excel_legacy(value, significance) {
      Ok(result) => Some(FormulaValue::Number(result)),
      Err(error) => Some(FormulaValue::Error(numeric_error_value(error))),
    }
  }

  pub(crate) fn evaluate_floor_excel_legacy(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let significance = self.number(&self.evaluate(args.get(1)?)?)?;
    match floor_excel_legacy(value, significance) {
      Ok(result) => Some(FormulaValue::Number(result)),
      Err(error) => Some(FormulaValue::Error(numeric_error_value(error))),
    }
  }

  pub(crate) fn evaluate_percentile(
    &self,
    args: &[FormulaAst<'doc>],
    kind: PercentileKind,
  ) -> Option<FormulaValue<'doc>> {
    let mut values = self.value_numbers(&self.evaluate(args.first()?)?);
    let k = self.number(&self.evaluate(args.get(1)?)?)?;
    percentile_sorted(&mut values, k, kind)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Num)))
  }

  pub(crate) fn evaluate_quartile(
    &self,
    args: &[FormulaAst<'doc>],
    kind: PercentileKind,
  ) -> Option<FormulaValue<'doc>> {
    let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let mut values = self.value_numbers(&value);
    let Some(quartile) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let quartile = approx_floor(quartile);
    percentile_sorted(&mut values, quartile / 4.0, kind)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_mode(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = self.numeric_args(args);
    mode_slice(&values)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  pub(crate) fn evaluate_mode_ms(
    &self,
    args: &[FormulaAst<'doc>],
    single: bool,
  ) -> Option<FormulaValue<'doc>> {
    let values = self.mode_ms_numeric_args(args);
    let modes = mode_ms_values(&values)?;
    if single {
      return modes
        .first()
        .copied()
        .map(FormulaValue::Number)
        .or(Some(FormulaValue::Error(FormulaErrorValue::NA)));
    }
    Some(FormulaValue::Matrix(
      modes
        .into_iter()
        .map(|value| vec![FormulaValue::Number(value)])
        .collect(),
    ))
  }

  pub(crate) fn evaluate_rank(
    &self,
    args: &[FormulaAst<'doc>],
    average: bool,
  ) -> Option<FormulaValue<'doc>> {
    let Some(value) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(range_value) = args.get(1).and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let values = self.value_numbers(&range_value);
    let ascending = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    rank_value(values, value, ascending, average)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  pub(crate) fn evaluate_kurt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = self.numeric_args(args);
    match kurtosis(&values) {
      Ok(value) => Some(FormulaValue::Number(value)),
      Err(error) => Some(FormulaValue::Error(statistics_error_value(error))),
    }
  }

  pub(crate) fn evaluate_beta_dist(
    &self,
    args: &[FormulaAst<'doc>],
    legacy: bool,
  ) -> Option<FormulaValue<'doc>> {
    if (legacy && !(3..=6).contains(&args.len())) || (!legacy && !(4..=6).contains(&args.len())) {
      return None;
    }
    let x_matrix = self.matrix_values(&self.evaluate(args.first()?)?);
    let alpha_matrix = self.matrix_values(&self.evaluate(args.get(1)?)?);
    let beta_matrix = self.matrix_values(&self.evaluate(args.get(2)?)?);
    let (cumulative_matrix, lower_matrix, upper_matrix) = if legacy {
      (
        args
          .get(5)
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.matrix_values(&value))
          .unwrap_or_else(|| vec![vec![FormulaValue::Boolean(true)]]),
        args
          .get(3)
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.matrix_values(&value))
          .unwrap_or_else(|| vec![vec![FormulaValue::Number(0.0)]]),
        args
          .get(4)
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.matrix_values(&value))
          .unwrap_or_else(|| vec![vec![FormulaValue::Number(1.0)]]),
      )
    } else {
      (
        self.matrix_values(&self.evaluate(args.get(3)?)?),
        args
          .get(4)
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.matrix_values(&value))
          .unwrap_or_else(|| vec![vec![FormulaValue::Number(0.0)]]),
        args
          .get(5)
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.matrix_values(&value))
          .unwrap_or_else(|| vec![vec![FormulaValue::Number(1.0)]]),
      )
    };
    let rows = x_matrix
      .len()
      .max(alpha_matrix.len())
      .max(beta_matrix.len())
      .max(lower_matrix.len())
      .max(upper_matrix.len())
      .max(cumulative_matrix.len());
    let columns = x_matrix
      .first()
      .map(Vec::len)
      .unwrap_or(1)
      .max(alpha_matrix.first().map(Vec::len).unwrap_or(1))
      .max(beta_matrix.first().map(Vec::len).unwrap_or(1))
      .max(lower_matrix.first().map(Vec::len).unwrap_or(1))
      .max(upper_matrix.first().map(Vec::len).unwrap_or(1))
      .max(cumulative_matrix.first().map(Vec::len).unwrap_or(1));
    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let x = self.number(matrix_item(&x_matrix, row, column)?)?;
        let alpha = self.number(matrix_item(&alpha_matrix, row, column)?)?;
        let beta = self.number(matrix_item(&beta_matrix, row, column)?)?;
        let lower = self.number(matrix_item(&lower_matrix, row, column)?)?;
        let upper = self.number(matrix_item(&upper_matrix, row, column)?)?;
        let cumulative = self.truthy(matrix_item(&cumulative_matrix, row, column)?);
        let scale = upper - lower;
        if alpha <= 0.0 || beta <= 0.0 || (legacy && scale <= 0.0) {
          result_row.push(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
          continue;
        }
        if !legacy && scale == 0.0 {
          result_row.push(FormulaValue::Error(FormulaErrorValue::Num));
          continue;
        }
        if !legacy && (x < lower || x > upper) {
          result_row.push(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
          continue;
        }
        if x < lower {
          result_row.push(FormulaValue::Number(0.0));
          continue;
        }
        if x > upper {
          result_row.push(if cumulative {
            FormulaValue::Number(1.0)
          } else {
            FormulaValue::Number(0.0)
          });
          continue;
        }
        let scaled = (x - lower) / scale;
        result_row.push(if cumulative {
          FormulaValue::Number(lo_beta_dist(scaled, alpha, beta))
        } else {
          match lo_beta_dist_pdf(scaled, alpha, beta) {
            Ok(value) => FormulaValue::Number(value / scale),
            Err(error) => FormulaValue::Error(special_error_value(error)),
          }
        });
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      result.into_iter().next()?.into_iter().next()
    } else {
      Some(FormulaValue::Matrix(result))
    }
  }

  pub(crate) fn evaluate_erf(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(lower) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let value = if let Some(upper) = args.get(1) {
      let Some(upper) = self.evaluate(upper).and_then(|value| self.number(&value)) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      erf(upper) - erf(lower)
    } else {
      erf(lower)
    };
    Some(FormulaValue::Number(value))
  }

  pub(crate) fn evaluate_delta(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(left) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let right = match args.get(1) {
      Some(arg) => {
        let value = self.evaluate(arg)?;
        let Some(number) = self.number(&value) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        number
      }
      None => 0.0,
    };
    Some(FormulaValue::Number((left == right) as u8 as f64))
  }

  pub(crate) fn evaluate_gestep(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(value) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let step = match args.get(1) {
      Some(arg) => {
        let value = self.evaluate(arg)?;
        let Some(number) = self.number(&value) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        number
      }
      None => 0.0,
    };
    Some(FormulaValue::Number((value >= step) as u8 as f64))
  }

  pub(crate) fn evaluate_beta_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let p_matrix = self.matrix_values(&self.evaluate(args.first()?)?);
    let alpha_matrix = self.matrix_values(&self.evaluate(args.get(1)?)?);
    let beta_matrix = self.matrix_values(&self.evaluate(args.get(2)?)?);
    let lower_matrix = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.matrix_values(&value))
      .unwrap_or_else(|| vec![vec![FormulaValue::Number(0.0)]]);
    let upper_matrix = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.matrix_values(&value))
      .unwrap_or_else(|| vec![vec![FormulaValue::Number(1.0)]]);
    let rows = p_matrix
      .len()
      .max(alpha_matrix.len())
      .max(beta_matrix.len())
      .max(lower_matrix.len())
      .max(upper_matrix.len());
    let columns = p_matrix
      .first()
      .map(Vec::len)
      .unwrap_or(1)
      .max(alpha_matrix.first().map(Vec::len).unwrap_or(1))
      .max(beta_matrix.first().map(Vec::len).unwrap_or(1))
      .max(lower_matrix.first().map(Vec::len).unwrap_or(1))
      .max(upper_matrix.first().map(Vec::len).unwrap_or(1));
    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let p = self.number(matrix_item(&p_matrix, row, column)?)?;
        let alpha = self.number(matrix_item(&alpha_matrix, row, column)?)?;
        let beta = self.number(matrix_item(&beta_matrix, row, column)?)?;
        let lower = self.number(matrix_item(&lower_matrix, row, column)?)?;
        let upper = self.number(matrix_item(&upper_matrix, row, column)?)?;
        if !(0.0..=1.0).contains(&p) || alpha <= 0.0 || beta <= 0.0 || lower >= upper {
          result_row.push(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
          continue;
        }
        if p == 0.0 {
          result_row.push(FormulaValue::Number(lower));
          continue;
        }
        if p == 1.0 {
          result_row.push(FormulaValue::Number(upper));
          continue;
        }
        result_row.push(
          match lo_iterate_inverse(|x| p - lo_beta_dist(x, alpha, beta), 0.0, 1.0) {
            Ok(value) => FormulaValue::Number(lower + value * (upper - lower)),
            Err(error) => FormulaValue::Error(special_error_value(error)),
          },
        );
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      result.into_iter().next()?.into_iter().next()
    } else {
      Some(FormulaValue::Matrix(result))
    }
  }

  pub(crate) fn evaluate_binom_dist(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?.floor();
    let n = self.number(&self.evaluate(args.get(1)?)?)?.floor();
    let p = self.number(&self.evaluate(args.get(2)?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(3)?)?);
    if x < 0.0 || n < 0.0 || x > n || !(0.0..=1.0).contains(&p) {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let dist = Binomial::new(p, n as u64).ok()?;
    Some(FormulaValue::Number(if cumulative {
      dist.cdf(x as u64)
    } else {
      dist.pmf(x as u64)
    }))
  }

  pub(crate) fn evaluate_b(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=4).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let n = approx_floor(self.number(&self.evaluate(args.first()?)?)?);
    let p = self.number(&self.evaluate(args.get(1)?)?)?;
    if args.len() == 3 {
      let x = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?);
      if n < 0.0 || x < 0.0 || x > n || !(0.0..=1.0).contains(&p) {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      if p == 0.0 {
        return Some(FormulaValue::Number((x == 0.0) as u8 as f64));
      }
      if p == 1.0 {
        return Some(FormulaValue::Number((x == n) as u8 as f64));
      }
      return Some(FormulaValue::Number(lo_binom_dist_pmf(x, n, p)));
    }

    let xs = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?);
    let xe = approx_floor(self.number(&self.evaluate(args.get(3)?)?)?);
    let valid_x = 0.0 <= xs && xs <= xe && xe <= n;
    if valid_x && 0.0 < p && p < 1.0 {
      if xs == xe {
        return Some(FormulaValue::Number(lo_binom_dist_pmf(xs, n, p)));
      }
      let q = (0.5 - p) + 0.5;
      let mut factor = q.powf(n);
      if factor > f64::MIN_POSITIVE {
        return Some(FormulaValue::Number(lo_binom_dist_range(
          n, xs, xe, factor, p, q,
        )));
      }
      factor = p.powf(n);
      if factor > f64::MIN_POSITIVE {
        return Some(FormulaValue::Number(lo_binom_dist_range(
          n,
          n - xe,
          n - xs,
          factor,
          q,
          p,
        )));
      }
      return Some(FormulaValue::Number(
        lo_beta_dist(q, n - xe, xe + 1.0) - lo_beta_dist(q, n - xs + 1.0, xs),
      ));
    }
    if valid_x {
      if p == 0.0 {
        Some(FormulaValue::Number((xs == 0.0) as u8 as f64))
      } else if p == 1.0 {
        Some(FormulaValue::Number((xe == n) as u8 as f64))
      } else {
        Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
      }
    } else {
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    }
  }

  pub(crate) fn evaluate_binom_dist_range(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    self.evaluate_b(args)
  }

  pub(crate) fn evaluate_binom_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let n = approx_floor(self.number(&self.evaluate(args.first()?)?)?);
    let p = self.number(&self.evaluate(args.get(1)?)?)?;
    let alpha = self.number(&self.evaluate(args.get(2)?)?)?;
    if n < 0.0 || !(0.0..=1.0).contains(&p) || !(0.0..=1.0).contains(&alpha) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if alpha == 0.0 {
      return Some(FormulaValue::Number(0.0));
    }
    if alpha == 1.0 {
      return Some(FormulaValue::Number(if p == 0.0 { 0.0 } else { n }));
    }

    let q = (0.5 - p) + 0.5;
    if q > p {
      let mut factor = q.powf(n);
      if factor > f64::MIN_POSITIVE {
        let mut sum = KahanSum::default();
        sum.add(factor);
        let mut i = 0_u32;
        let max = n as u32;
        while i < max && sum.finish() < alpha {
          factor *= (n - f64::from(i)) / f64::from(i + 1) * p / q;
          sum.add(factor);
          i += 1;
        }
        Some(FormulaValue::Number(f64::from(i)))
      } else {
        let mut sum = KahanSum::default();
        let mut i = 0_u32;
        let max = n as u32;
        while i < max && sum.finish() < alpha {
          let x = lo_beta_dist_pdf(p, f64::from(i + 1), n - f64::from(i) + 1.0).ok()? / (n + 1.0);
          sum.add(x);
          i += 1;
        }
        Some(FormulaValue::Number(f64::from(i.saturating_sub(1))))
      }
    } else {
      let mut factor = p.powf(n);
      if factor > f64::MIN_POSITIVE {
        let mut sum = KahanSum::default();
        sum.add(1.0);
        sum.add(-factor);
        let mut i = 0_u32;
        let max = n as u32;
        while i < max && sum.finish() >= alpha {
          factor *= (n - f64::from(i)) / f64::from(i + 1) * q / p;
          sum.add(-factor);
          i += 1;
        }
        Some(FormulaValue::Number(n - f64::from(i)))
      } else {
        let mut sum = KahanSum::default();
        let tail_alpha = 1.0 - alpha;
        let mut i = 0_u32;
        let max = n as u32;
        while i < max && sum.finish() < tail_alpha {
          let x = lo_beta_dist_pdf(q, f64::from(i + 1), n - f64::from(i) + 1.0).ok()? / (n + 1.0);
          sum.add(x);
          i += 1;
        }
        Some(FormulaValue::Number(n - f64::from(i) + 1.0))
      }
    }
  }

  pub(crate) fn evaluate_chisq_dist(
    &self,
    args: &[FormulaAst<'doc>],
    right_tail: bool,
  ) -> Option<FormulaValue<'doc>> {
    let x_value = self.evaluate(args.first()?)?;
    let df_value = self.evaluate(args.get(1)?)?;
    let cumulative_value = args.get(2).and_then(|arg| self.evaluate(arg));
    if self.array_context
      && (matches!(
        x_value,
        FormulaValue::Reference(_) | FormulaValue::Matrix(_)
      ) || matches!(
        df_value,
        FormulaValue::Reference(_) | FormulaValue::Matrix(_)
      ) || cumulative_value
        .as_ref()
        .is_some_and(|value| matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))))
    {
      let x_matrix = self.matrix_values(&x_value);
      let df_matrix = self.matrix_values(&df_value);
      let cumulative_matrix = cumulative_value
        .as_ref()
        .map(|value| self.matrix_values(value))
        .unwrap_or_default();
      let rows = x_matrix
        .len()
        .max(df_matrix.len())
        .max(cumulative_matrix.len());
      let columns = x_matrix
        .first()
        .map(Vec::len)
        .unwrap_or(1)
        .max(df_matrix.first().map(Vec::len).unwrap_or(1))
        .max(cumulative_matrix.first().map(Vec::len).unwrap_or(1));
      let mut result = Vec::with_capacity(rows);
      for row in 0..rows {
        let mut result_row = Vec::with_capacity(columns);
        for column in 0..columns {
          let x = self.number(matrix_item(&x_matrix, row, column)?)?;
          let df = approx_floor(self.number(matrix_item(&df_matrix, row, column)?)?);
          let cumulative = cumulative_value.as_ref().map(|_| {
            matrix_item(&cumulative_matrix, row, column)
              .map(|value| self.truthy(value))
              .unwrap_or(false)
          });
          result_row.push(chisq_dist_value(x, df, right_tail, cumulative));
        }
        result.push(result_row);
      }
      return Some(FormulaValue::Matrix(result));
    }
    let x = self.number(&x_value)?;
    let df = approx_floor(self.number(&df_value)?);
    let cumulative = cumulative_value.as_ref().map(|value| self.truthy(value));
    Some(chisq_dist_value(x, df, right_tail, cumulative))
  }

  pub(crate) fn evaluate_chisq_inv(
    &self,
    args: &[FormulaAst<'doc>],
    right_tail: bool,
  ) -> Option<FormulaValue<'doc>> {
    let p_value = self.evaluate(args.first()?)?;
    let df_value = self.evaluate(args.get(1)?)?;
    if self.array_context
      && (matches!(
        p_value,
        FormulaValue::Reference(_) | FormulaValue::Matrix(_)
      ) || matches!(
        df_value,
        FormulaValue::Reference(_) | FormulaValue::Matrix(_)
      ))
    {
      let p_matrix = self.matrix_values(&p_value);
      let df_matrix = self.matrix_values(&df_value);
      let rows = p_matrix.len().max(df_matrix.len());
      let columns = p_matrix
        .first()
        .map(Vec::len)
        .unwrap_or(1)
        .max(df_matrix.first().map(Vec::len).unwrap_or(1));
      let mut result = Vec::with_capacity(rows);
      for row in 0..rows {
        let mut result_row = Vec::with_capacity(columns);
        for column in 0..columns {
          let p = self.number(matrix_item(&p_matrix, row, column)?)?;
          let df = approx_floor(self.number(matrix_item(&df_matrix, row, column)?)?);
          result_row.push(chisq_inv_value(p, df, right_tail));
        }
        result.push(result_row);
      }
      return Some(FormulaValue::Matrix(result));
    }
    let p = self.number(&p_value)?;
    let df = approx_floor(self.number(&df_value)?);
    Some(chisq_inv_value(p, df, right_tail))
  }

  pub(crate) fn evaluate_chisq_test(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
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
    let mut chi = KahanSum::default();
    let mut has_value = false;
    for column in 0..columns {
      for row in 0..rows {
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
            chi.add(term);
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
    Some(FormulaValue::Number(lo_chi_dist(chi.finish(), df)))
  }

  pub(crate) fn evaluate_confidence_norm(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(alpha) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(sigma) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(size) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if !alpha.is_finite()
      || !sigma.is_finite()
      || !size.is_finite()
      || alpha <= 0.0
      || alpha >= 1.0
      || sigma <= 0.0
      || size.floor() < 1.0
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let size = size.floor();
    Some(FormulaValue::Number(
      norm_s_inv(1.0 - alpha / 2.0).abs() * sigma / size.sqrt(),
    ))
  }

  pub(crate) fn evaluate_confidence_t(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(alpha) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(sigma) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(size) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if !alpha.is_finite()
      || !sigma.is_finite()
      || !size.is_finite()
      || alpha <= 0.0
      || alpha >= 1.0
      || sigma <= 0.0
      || size.floor() < 1.0
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let size = size.floor();
    if size == 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    let dist = StudentsT::new(0.0, 1.0, size - 1.0).ok()?;
    Some(FormulaValue::Number(
      dist.inverse_cdf(1.0 - alpha / 2.0).abs() * sigma / size.sqrt(),
    ))
  }

  pub(crate) fn evaluate_covariance(
    &self,
    args: &[FormulaAst<'doc>],
    sample: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let left = self.matrix_values(&self.evaluate(args.first()?)?);
    let right = self.matrix_values(&self.evaluate(args.get(1)?)?);
    let Some(pairs) = covariance_pairs(&left, &right) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    covariance(&pairs, sample)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_correl(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let left = self.value_numbers(&self.evaluate(args.first()?)?);
    let right = self.value_numbers(&self.evaluate(args.get(1)?)?);
    correlation(&left, &right)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Div0)))
  }

  pub(crate) fn evaluate_slope(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let y_values = self.value_numbers(&self.evaluate(args.first()?)?);
    let x_values = self.value_numbers(&self.evaluate(args.get(1)?)?);
    let state = regression_scalar_state_from_slices(&y_values, &x_values);
    if state.count < 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    state
      .slope()
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Div0)))
  }

  pub(crate) fn evaluate_intercept(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let y_value = self.evaluate(args.first()?)?;
    let x_value = self.evaluate(args.get(1)?)?;
    if matrix_dimensions(&self.matrix_values(&y_value))
      != matrix_dimensions(&self.matrix_values(&x_value))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    regression_scalar_state_for_values(self, &y_value, &x_value).map(|state| {
      state
        .intercept()
        .map(FormulaValue::Number)
        .unwrap_or(FormulaValue::Error(FormulaErrorValue::Div0))
    })
  }

  pub(crate) fn evaluate_forecast(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let y_value = self.evaluate(args.get(1)?)?;
    let x_value = self.evaluate(args.get(2)?)?;
    if matrix_dimensions(&self.matrix_values(&y_value))
      != matrix_dimensions(&self.matrix_values(&x_value))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    regression_scalar_state_for_values(self, &y_value, &x_value).map(|state| {
      state
        .forecast(x)
        .map(FormulaValue::Number)
        .unwrap_or(FormulaValue::Error(FormulaErrorValue::Div0))
    })
  }

  pub(crate) fn evaluate_forecast_ets(
    &self,
    args: &[FormulaAst<'doc>],
    kind: EtsKind,
  ) -> Option<FormulaValue<'doc>> {
    let valid_count = match kind {
      EtsKind::Add | EtsKind::Mult | EtsKind::StatAdd | EtsKind::StatMult => {
        (3..=6).contains(&args.len())
      }
      EtsKind::Season => (2..=4).contains(&args.len()),
    };
    if !valid_count {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let aggregation_index = match kind {
      EtsKind::Season => 3,
      _ => 5,
    };
    let data_completion_index = match kind {
      EtsKind::Season => 2,
      _ => 4,
    };
    let seasonality_index = 3;
    let aggregation = args
      .get(aggregation_index)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0)
      .floor() as i32;
    if !(1..=7).contains(&aggregation) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let data_completion = args
      .get(data_completion_index)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0);
    if data_completion != 0.0 && data_completion != 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let samples_in_period = if kind == EtsKind::Season {
      1
    } else {
      let value = args
        .get(seasonality_index)
        .and_then(|arg| self.evaluate(arg))
        .and_then(|value| self.number(&value))
        .unwrap_or(1.0);
      if value < 0.0 || value.fract() != 0.0 {
        return Some(FormulaValue::Error(FormulaErrorValue::Num));
      }
      value as usize
    };

    let type_matrix = if matches!(kind, EtsKind::StatAdd | EtsKind::StatMult) {
      let matrix = self.matrix_values(&self.evaluate(args.first()?)?);
      for value in matrix.iter().flatten() {
        let Some(number) = self.number(value) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        if !(1.0..=9.0).contains(&number) {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        }
      }
      Some(matrix)
    } else {
      None
    };
    let (target_arg, values_arg, timeline_arg) = match kind {
      EtsKind::Season => (None, args.first()?, args.get(1)?),
      EtsKind::StatAdd | EtsKind::StatMult => (None, args.get(1)?, args.get(2)?),
      EtsKind::Add | EtsKind::Mult => (Some(args.first()?), args.get(1)?, args.get(2)?),
    };
    let values = self.value_numbers(&self.evaluate(values_arg)?);
    let timeline = self.value_numbers(&self.evaluate(timeline_arg)?);
    if values.len() != timeline.len() || values.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let target_value = target_arg.and_then(|arg| self.evaluate(arg));
    let target_matrix = target_value.as_ref().map(|value| self.matrix_values(value));
    let target_first = target_matrix
      .as_ref()
      .and_then(|matrix| matrix.first())
      .and_then(|row| row.first())
      .and_then(|value| self.number(value));
    let mut calc = match EtsCalculation::new(
      &timeline,
      &values,
      samples_in_period,
      data_completion != 0.0,
      aggregation,
      target_first,
      kind,
    ) {
      Ok(calc) => calc,
      Err(error) => return Some(FormulaValue::Error(ets_error_value(error))),
    };
    match kind {
      EtsKind::Season => Some(FormulaValue::Number(calc.samples_in_period as f64)),
      EtsKind::StatAdd | EtsKind::StatMult => {
        let matrix = type_matrix?;
        Some(FormulaValue::Matrix(
          matrix
            .into_iter()
            .map(|row| {
              row
                .into_iter()
                .map(|value| {
                  let index = self.number(&value).unwrap_or(0.0).floor() as i32;
                  FormulaValue::Number(calc.statistic(index))
                })
                .collect()
            })
            .collect(),
        ))
      }
      EtsKind::Add | EtsKind::Mult => {
        let matrix = target_matrix?;
        let result = matrix
          .iter()
          .map(|row| {
            row
              .iter()
              .map(|value| {
                self
                  .number(value)
                  .map(|target| FormulaValue::Number(calc.forecast(target)))
                  .unwrap_or(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
              })
              .collect::<Vec<_>>()
          })
          .collect::<Vec<_>>();
        if result.len() == 1 && result.first().is_some_and(|row| row.len() == 1) {
          result.into_iter().next()?.into_iter().next()
        } else {
          Some(FormulaValue::Matrix(result))
        }
      }
    }
  }

  pub(crate) fn evaluate_rsq(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    regression_scalar_state(self, args).map(|state| {
      state
        .r_squared()
        .map(FormulaValue::Number)
        .unwrap_or(FormulaValue::Error(FormulaErrorValue::Div0))
    })
  }

  pub(crate) fn evaluate_steyx(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    regression_scalar_state(self, args).map(|state| {
      state
        .steyx()
        .map(FormulaValue::Number)
        .unwrap_or(FormulaValue::Error(FormulaErrorValue::Div0))
    })
  }

  pub(crate) fn evaluate_linest(
    &self,
    args: &[FormulaAst<'doc>],
    log_regression: bool,
  ) -> Option<FormulaValue<'doc>> {
    if !(1..=4).contains(&args.len()) {
      return None;
    }
    let constant = match args.get(2) {
      Some(arg) if is_missing_argument(arg) => true,
      Some(arg) => self.evaluate(arg).map(|value| self.truthy(&value))?,
      None => true,
    };
    let stats = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(false);
    let x_arg = args.get(1).filter(|arg| !is_missing_argument(arg));
    let data = match self.regression_data(args.first()?, x_arg, log_regression) {
      Ok(data) => data,
      Err(error) => return Some(FormulaValue::Error(error)),
    };
    regression_coefficients(&data, constant, stats, log_regression)
      .map(FormulaValue::Matrix)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  pub(crate) fn evaluate_trend_growth(
    &self,
    args: &[FormulaAst<'doc>],
    growth: bool,
  ) -> Option<FormulaValue<'doc>> {
    if !(1..=4).contains(&args.len()) {
      return None;
    }
    let constant = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    let data = match self.regression_data(args.first()?, args.get(1), growth) {
      Ok(data) => data,
      Err(error) => return Some(FormulaValue::Error(error)),
    };
    let model = match regression_model(&data, constant) {
      Some(model) => model,
      None => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    };
    let prediction = match args.get(2).and_then(|arg| self.evaluate(arg)) {
      Some(value) => match data.prediction_matrix(self, &value) {
        Ok(prediction) => prediction,
        Err(error) => return Some(FormulaValue::Error(error)),
      },
      None => data.default_prediction_matrix(),
    };
    let values = prediction
      .design
      .iter()
      .map(|row| {
        let value = model.predict(row);
        FormulaValue::Number(if growth { value.exp() } else { value })
      })
      .collect::<Vec<_>>();
    Some(FormulaValue::Matrix(prediction.shape.materialize(values)))
  }

  pub(crate) fn regression_data(
    &self,
    y_arg: &FormulaAst<'doc>,
    x_arg: Option<&FormulaAst<'doc>>,
    log_y: bool,
  ) -> std::result::Result<RegressionData, FormulaErrorValue> {
    let y_value = self
      .evaluate(y_arg)
      .ok_or(FormulaErrorValue::IllegalArgument)?;
    let y_matrix = self.matrix_values(&y_value);
    let y_shape = MatrixShape::from_matrix(&y_matrix).ok_or(FormulaErrorValue::IllegalArgument)?;
    let mut y = matrix_numbers(self, &y_matrix).ok_or(FormulaErrorValue::IllegalArgument)?;
    if y.len() != y_shape.len() {
      return Err(FormulaErrorValue::IllegalArgument);
    }
    if log_y {
      if y.iter().any(|value| *value <= 0.0) {
        return Err(FormulaErrorValue::IllegalArgument);
      }
      y.iter_mut().for_each(|value| *value = value.ln());
    }

    let (x_matrix, x_shape) = if let Some(arg) = x_arg {
      let value = self
        .evaluate(arg)
        .ok_or(FormulaErrorValue::IllegalArgument)?;
      let matrix = self.matrix_values(&value);
      let shape = MatrixShape::from_matrix(&matrix).ok_or(FormulaErrorValue::IllegalArgument)?;
      (matrix, shape)
    } else {
      let values = y_shape.materialize(
        (1..=y.len())
          .map(|value| FormulaValue::Number(value as f64))
          .collect(),
      );
      (values, y_shape)
    };
    let x_numbers = matrix_numbers(self, &x_matrix).ok_or(FormulaErrorValue::IllegalArgument)?;
    if x_numbers.len() != x_shape.len() {
      return Err(FormulaErrorValue::IllegalArgument);
    }

    let (case, design) = if x_shape == y_shape {
      (
        RegressionCase::Simple,
        x_numbers.into_iter().map(|value| vec![value]).collect(),
      )
    } else if y_shape.columns != 1 && y_shape.rows != 1 {
      return Err(FormulaErrorValue::IllegalArgument);
    } else if y_shape.columns == 1 {
      if x_shape.rows != y_shape.rows {
        return Err(FormulaErrorValue::IllegalArgument);
      }
      let mut rows = Vec::with_capacity(x_shape.rows);
      for row in 0..x_shape.rows {
        let mut values = Vec::with_capacity(x_shape.columns);
        for column in 0..x_shape.columns {
          values.push(
            matrix_number_at(&x_matrix, row, column, self)
              .ok_or(FormulaErrorValue::IllegalArgument)?,
          );
        }
        rows.push(values);
      }
      (RegressionCase::ColumnY, rows)
    } else {
      if x_shape.columns != y_shape.columns {
        return Err(FormulaErrorValue::IllegalArgument);
      }
      let mut rows = Vec::with_capacity(x_shape.columns);
      for column in 0..x_shape.columns {
        let mut values = Vec::with_capacity(x_shape.rows);
        for row in 0..x_shape.rows {
          values.push(
            matrix_number_at(&x_matrix, row, column, self)
              .ok_or(FormulaErrorValue::IllegalArgument)?,
          );
        }
        rows.push(values);
      }
      (RegressionCase::RowY, rows)
    };
    let k = design.first().map_or(0, Vec::len);
    let n = y.len();
    if n < 1 || k < 1 {
      return Err(FormulaErrorValue::IllegalArgument);
    }
    Ok(RegressionData {
      case,
      x_shape,
      y,
      design,
    })
  }

  pub(crate) fn evaluate_expon_dist(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let lambda = self.number(&self.evaluate(args.get(1)?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(2)?)?);
    if x < 0.0 || lambda <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let dist = Exp::new(lambda).ok()?;
    Some(FormulaValue::Number(if cumulative {
      dist.cdf(x)
    } else {
      dist.pdf(x)
    }))
  }

  pub(crate) fn evaluate_f_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=4).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let df1 = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    let df2 = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?);
    let cumulative = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    if x < 0.0 || df1 < 1.0 || df2 < 1.0 || df1 >= 1.0e10 || df2 >= 1.0e10 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(if cumulative {
      1.0 - lo_f_dist_right_tail(x, df1, df2)
    } else {
      lo_f_dist_pdf(x, df1, df2)
    }))
  }

  pub(crate) fn evaluate_f_dist_rt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let df1 = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    let df2 = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?);
    if x < 0.0 || df1 < 1.0 || df2 < 1.0 || df1 >= 1.0e10 || df2 >= 1.0e10 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(lo_f_dist_right_tail(x, df1, df2)))
  }

  pub(crate) fn evaluate_f_inv(
    &self,
    args: &[FormulaAst<'doc>],
    right_tail: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let p = self.number(&self.evaluate(args.first()?)?)?;
    let df1 = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    let df2 = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?);
    if p <= 0.0 || p > 1.0 || df1 < 1.0 || df2 < 1.0 || df1 >= 1.0e10 || df2 >= 1.0e10 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    match lo_iterate_inverse(
      |x| (if right_tail { p } else { 1.0 - p }) - lo_f_dist_right_tail(x, df1, df2),
      df1 * 0.5,
      df1,
    ) {
      Ok(value) => Some(FormulaValue::Number(value)),
      Err(error) => Some(FormulaValue::Error(special_error_value(error))),
    }
  }

  pub(crate) fn evaluate_f_test(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
    let right_tail = lo_f_dist_right_tail(f, df1, df2);
    Some(FormulaValue::Number(2.0 * right_tail.min(1.0 - right_tail)))
  }

  pub(crate) fn evaluate_gamma_dist(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let alpha = self.number(&self.evaluate(args.get(1)?)?)?;
    let beta = self.number(&self.evaluate(args.get(2)?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(3)?)?);
    if x < 0.0 || alpha <= 0.0 || beta <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(if cumulative {
      lo_gamma_dist(x, alpha, beta)
    } else {
      match lo_gamma_dist_pdf(x, alpha, beta) {
        Ok(value) => value,
        Err(error) => return Some(FormulaValue::Error(special_error_value(error))),
      }
    }))
  }

  pub(crate) fn evaluate_gamma_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let p_value = self.evaluate(args.first()?)?;
    let alpha_value = self.evaluate(args.get(1)?)?;
    let beta_value = self.evaluate(args.get(2)?)?;
    let p_matrix = self.matrix_values(&p_value);
    let alpha_matrix = self.matrix_values(&alpha_value);
    let beta_matrix = self.matrix_values(&beta_value);
    let rows = p_matrix
      .len()
      .max(alpha_matrix.len())
      .max(beta_matrix.len());
    let columns = p_matrix
      .first()
      .map(Vec::len)
      .unwrap_or(1)
      .max(alpha_matrix.first().map(Vec::len).unwrap_or(1))
      .max(beta_matrix.first().map(Vec::len).unwrap_or(1));
    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let p = self.number(matrix_item(&p_matrix, row, column)?)?;
        let alpha = self.number(matrix_item(&alpha_matrix, row, column)?)?;
        let beta = self.number(matrix_item(&beta_matrix, row, column)?)?;
        if !(0.0..1.0).contains(&p) || alpha <= 0.0 || beta <= 0.0 {
          result_row.push(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
          continue;
        }
        result_row.push(FormulaValue::Number(if p == 0.0 {
          0.0
        } else {
          match lo_iterate_inverse(
            |x| p - lo_gamma_dist(x, alpha, beta),
            alpha * beta * 0.5,
            alpha * beta,
          ) {
            Ok(value) => value,
            Err(error) => {
              result_row.push(FormulaValue::Error(special_error_value(error)));
              continue;
            }
          }
        }));
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      result.into_iter().next()?.into_iter().next()
    } else {
      Some(FormulaValue::Matrix(result))
    }
  }

  pub(crate) fn evaluate_gamma(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.number(&self.evaluate(args.first()?)?)?;
    if value == 0.0 || (value < 0.0 && value.fract() == 0.0) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(gamma(value)))
  }

  pub(crate) fn evaluate_hypgeom_dist(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if !(4..=5).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(sample_success) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(sample_size) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(population_success) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(population_size) = self.number_arg(args, 3) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let cumulative = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(false);
    let sample_success = sample_success.floor();
    let sample_size = sample_size.floor();
    let population_success = population_success.floor();
    let population_size = population_size.floor();
    if sample_success < 0.0
      || sample_size < 0.0
      || population_success < 0.0
      || population_size < 0.0
      || sample_success > sample_size
      || sample_size > population_size
      || population_success > population_size
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let dist = Hypergeometric::new(
      population_size as u64,
      population_success as u64,
      sample_size as u64,
    )
    .ok()?;
    Some(FormulaValue::Number(if cumulative {
      dist.cdf(sample_success as u64)
    } else {
      dist.pmf(sample_success as u64)
    }))
  }

  pub(crate) fn evaluate_lognorm_dist(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let mean = self.number(&self.evaluate(args.get(1)?)?)?;
    let sigma = self.number(&self.evaluate(args.get(2)?)?)?;
    let cumulative = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    if sigma <= 0.0 || (!cumulative && x <= 0.0) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(if cumulative {
      if x <= 0.0 {
        0.0
      } else {
        lo_integral_phi((x.ln() - mean) / sigma)
      }
    } else {
      lo_phi((x.ln() - mean) / sigma) / sigma / x
    }))
  }

  pub(crate) fn evaluate_lognorm_inv(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(p) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(mean) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(sigma) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if p <= 0.0 || p >= 1.0 || sigma <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(
      LogNormal::new(mean, sigma).ok()?.inverse_cdf(p),
    ))
  }

  pub(crate) fn evaluate_negbinom_dist(
    &self,
    args: &[FormulaAst<'doc>],
    microsoft: bool,
  ) -> Option<FormulaValue<'doc>> {
    if (microsoft && args.len() != 4) || (!microsoft && args.len() != 3) {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let failures = self.number(&self.evaluate(args.first()?)?)?.floor();
    let successes = self.number(&self.evaluate(args.get(1)?)?)?.floor();
    let p = self.number(&self.evaluate(args.get(2)?)?)?;
    let cumulative = microsoft
      && args
        .get(3)
        .and_then(|arg| self.evaluate(arg))
        .is_some_and(|value| self.truthy(&value));
    if (microsoft && (failures < 0.0 || successes < 1.0))
      || (!microsoft && failures + successes <= 1.0)
      || !(0.0..=1.0).contains(&p)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(if cumulative {
      1.0 - lo_beta_dist(1.0 - p, failures + 1.0, successes)
    } else {
      let mut factor = p.powf(successes);
      for i in 0..failures as u32 {
        factor *= (f64::from(i) + successes) / f64::from(i + 1) * (1.0 - p);
      }
      factor
    }))
  }

  pub(crate) fn evaluate_norm_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=4).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(x) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(mean) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(sigma) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let cumulative = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    if sigma <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(if cumulative {
      lo_integral_phi((x - mean) / sigma)
    } else {
      lo_phi((x - mean) / sigma) / sigma
    }))
  }

  pub(crate) fn evaluate_norm_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let p = self.number(&self.evaluate(args.first()?)?)?;
    let mean = self.number(&self.evaluate(args.get(1)?)?)?;
    let sigma = self.number(&self.evaluate(args.get(2)?)?)?;
    if sigma <= 0.0 || !(0.0..=1.0).contains(&p) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if p == 0.0 || p == 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    Some(FormulaValue::Number(
      Normal::new(mean, sigma).ok()?.inverse_cdf(p),
    ))
  }

  pub(crate) fn evaluate_norm_s_inv(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let Some(p) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if !(0.0..=1.0).contains(&p) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if p == 0.0 || p == 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    Some(FormulaValue::Number(norm_s_inv(p)))
  }

  pub(crate) fn evaluate_norm_s_dist(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let z = self.number(&self.evaluate(args.first()?)?)?;
    let cumulative = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    Some(FormulaValue::Number(if cumulative {
      lo_integral_phi(z)
    } else {
      lo_phi(z)
    }))
  }

  pub(crate) fn evaluate_percent_rank(
    &self,
    args: &[FormulaAst<'doc>],
    kind: PercentileKind,
  ) -> Option<FormulaValue<'doc>> {
    let values = self.value_numbers(&self.evaluate(args.first()?)?);
    let x = self.number(&self.evaluate(args.get(1)?)?)?;
    let significance = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(approx_floor)
      .unwrap_or(3.0);
    if significance < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    percent_rank(values, x, significance, kind)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  pub(crate) fn evaluate_poisson_dist(
    &self,
    args: &[FormulaAst<'doc>],
    odff: bool,
  ) -> Option<FormulaValue<'doc>> {
    let min_args = if odff { 2 } else { 3 };
    if args.len() < min_args || args.len() > 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let x = self.number(&self.evaluate(args.first()?)?)?.floor();
    let lambda = self.number(&self.evaluate(args.get(1)?)?)?;
    let cumulative = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    if x < 0.0 || lambda <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(lo_poisson_dist(x, lambda, cumulative)))
  }

  pub(crate) fn evaluate_fisher(
    &self,
    args: &[FormulaAst<'doc>],
    inverse: bool,
  ) -> Option<FormulaValue<'doc>> {
    let value = self.number(&self.evaluate(args.first()?)?)?;
    if inverse {
      return Some(FormulaValue::Number(value.tanh()));
    }
    if value.abs() >= 1.0 {
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    } else {
      Some(FormulaValue::Number(value.atanh()))
    }
  }

  pub(crate) fn evaluate_bessel(
    &self,
    args: &[FormulaAst<'doc>],
    kind: BesselKind,
  ) -> Option<FormulaValue<'doc>> {
    let value = self.evaluate(args.first()?)?;
    let order = self.evaluate(args.get(1)?)?;
    if self.array_context
      && (matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        || matches!(order, FormulaValue::Reference(_) | FormulaValue::Matrix(_)))
    {
      return self.map_binary_values(value, order, |evaluator, value, order| {
        evaluator.bessel_value(value, order, kind)
      });
    }
    self.bessel_value(&value, &order, kind)
  }

  pub(crate) fn bessel_value(
    &self,
    value: &FormulaValue<'doc>,
    order: &FormulaValue<'doc>,
    kind: BesselKind,
  ) -> Option<FormulaValue<'doc>> {
    let value = self.number(value)?;
    let order = approx_floor(self.number(order)?) as i32;
    if order < 0 || matches!(kind, BesselKind::K | BesselKind::Y) && value <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let result = bessel(kind, value, order);
    if result.is_finite() {
      Some(FormulaValue::Number(result))
    } else {
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    }
  }

  pub(crate) fn evaluate_fourier(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=5).contains(&args.len()) {
      return None;
    }
    let input = self.matrix_values(&self.evaluate(args.first()?)?);
    if input.is_empty() || input.first()?.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let grouped_by_column = self.truthy(&self.evaluate(args.get(1)?)?);
    let inverse = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(false);
    let polar = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(false);
    let min_magnitude = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);

    let row_count = input.len();
    let column_count = input.first()?.len();
    if input.iter().any(|row| row.len() != column_count) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if (grouped_by_column && column_count > 2) || (!grouped_by_column && row_count > 2) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }

    let real_input = if grouped_by_column {
      column_count == 1
    } else {
      row_count == 1
    };
    let point_count = if grouped_by_column {
      row_count
    } else {
      column_count
    };
    let mut values = Vec::with_capacity(point_count);
    if grouped_by_column {
      for row in &input {
        let Some(real) = self.number(&row[0]) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        let imaginary = if real_input {
          0.0
        } else {
          let Some(imaginary) = self.number(&row[1]) else {
            return Some(FormulaValue::Error(FormulaErrorValue::Value));
          };
          imaginary
        };
        values.push(Complex::new(real, imaginary));
      }
    } else {
      for (real, imaginary) in input[0].iter().zip(input.get(1).into_iter().flatten()) {
        let Some(real) = self.number(real) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        let Some(imaginary) = self.number(imaginary) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        values.push(Complex::new(real, imaginary));
      }
      if real_input {
        values.clear();
        for real in &input[0] {
          let Some(real) = self.number(real) else {
            return Some(FormulaValue::Error(FormulaErrorValue::Value));
          };
          values.push(Complex::new(real, 0.0));
        }
      }
    }

    let values = self.engine.fourier_values(values, real_input, inverse);
    let scale = if inverse {
      1.0 / point_count as f64
    } else {
      1.0
    };
    Some(FormulaValue::Matrix(
      values
        .into_iter()
        .map(|value| {
          if polar {
            let mut magnitude = value.norm();
            let mut phase = if magnitude < min_magnitude {
              magnitude = 0.0;
              0.0
            } else {
              value.im.atan2(value.re)
            };
            if inverse {
              magnitude *= scale;
            }
            if !phase.is_finite() {
              phase = 0.0;
            }
            vec![FormulaValue::Number(magnitude), FormulaValue::Number(phase)]
          } else {
            vec![
              FormulaValue::Number(value.re * scale),
              FormulaValue::Number(value.im * scale),
            ]
          }
        })
        .collect(),
    ))
  }

  pub(crate) fn evaluate_complex_part(
    &self,
    args: &[FormulaAst<'doc>],
    imaginary: bool,
  ) -> Option<FormulaValue<'doc>> {
    let Some(arg) = args.first() else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let value = parse_complex_number(&self.text(&self.evaluate(arg)?))?;
    Some(FormulaValue::Number(if imaginary {
      value.imaginary()
    } else {
      value.real()
    }))
  }

  pub(crate) fn evaluate_complex(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let real = self.number(&self.evaluate(args.first()?)?)?;
    let imaginary = self.number(&self.evaluate(args.get(1)?)?)?;
    let suffix = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.text(&value))
      .unwrap_or_else(|| "i".to_string());
    if suffix != "i" && suffix != "j" && !suffix.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::String(Cow::Owned(format_complex_result(
      FormulaComplex::new(real, imaginary, if suffix == "j" { 'j' } else { 'i' }),
    ))))
  }

  pub(crate) fn evaluate_complex_argument(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = parse_complex_number(&self.text(&self.evaluate(args.first()?)?))?;
    Some(FormulaValue::Number(value.imaginary().atan2(value.real())))
  }

  pub(crate) fn evaluate_complex_abs(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = parse_complex_number(&self.text(&self.evaluate(args.first()?)?))?;
    Some(FormulaValue::Number(value.value().norm()))
  }

  pub(crate) fn evaluate_complex_unary(
    &self,
    args: &[FormulaAst<'doc>],
    op: impl FnOnce(Complex<f64>) -> Complex<f64>,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = parse_complex_number(&self.text(&self.evaluate(args.first()?)?))?;
    let result = op(value.value());
    Some(FormulaValue::String(Cow::Owned(format_complex_result(
      FormulaComplex::from_value(result, value.suffix()),
    ))))
  }

  pub(crate) fn evaluate_complex_binary(
    &self,
    args: &[FormulaAst<'doc>],
    op: impl Fn(f64, f64) -> f64,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(left) = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| parse_complex_number(&self.text(&value)))
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(right) = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| parse_complex_number(&self.text(&value)))
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    Some(FormulaValue::String(Cow::Owned(format_complex_result(
      FormulaComplex::new(
        op(left.real(), right.real()),
        op(left.imaginary(), right.imaginary()),
        binary_suffix(left, right),
      ),
    ))))
  }

  pub(crate) fn evaluate_complex_div(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(left) = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| parse_complex_number(&self.text(&value)))
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(right) = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| parse_complex_number(&self.text(&value)))
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    if right.real() == 0.0 && right.imaginary() == 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let result = left.value() / right.value();
    if !result.re.is_finite() || !result.im.is_finite() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::String(Cow::Owned(format_complex_result(
      FormulaComplex::from_value(result, binary_suffix(left, right)),
    ))))
  }

  pub(crate) fn evaluate_complex_power(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(value) = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| parse_complex_number(&self.text(&value)))
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let power = self.number(&self.evaluate(args.get(1)?)?)?;
    let result = value.value().powf(power);
    Some(FormulaValue::String(Cow::Owned(format_complex_result(
      FormulaComplex::from_value(result, value.suffix()),
    ))))
  }

  pub(crate) fn evaluate_complex_sum_product(
    &self,
    args: &[FormulaAst<'doc>],
    product: bool,
  ) -> Option<FormulaValue<'doc>> {
    let mut total = if product {
      Complex::new(1.0, 0.0)
    } else {
      Complex::new(0.0, 0.0)
    };
    let mut suffix = 'i';
    for source in self.values(args) {
      let Some(value) = parse_complex_number(&self.text(&source)) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      if value.suffix() == 'j' {
        suffix = 'j';
      }
      if product {
        total *= value.value();
      } else {
        total += value.value();
      }
    }
    Some(FormulaValue::String(Cow::Owned(format_complex_result(
      FormulaComplex::from_value(total, suffix),
    ))))
  }

  pub(crate) fn evaluate_t_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let t = self.number(&self.evaluate(args.first()?)?)?;
    let df = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    let cumulative = self.truthy(&self.evaluate(args.get(2)?)?);
    if df < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(if cumulative {
      lo_t_dist(t, df, 4)
    } else {
      lo_t_dist(t, df, 3)
    }))
  }

  pub(crate) fn evaluate_t_dist_tails(
    &self,
    args: &[FormulaAst<'doc>],
    tails: i32,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let t = self.number(&self.evaluate(args.first()?)?)?;
    let df = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    if df < 1.0 || (tails == 2 && t < 0.0) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let result = lo_t_dist(t, df, tails);
    Some(FormulaValue::Number(match tails {
      1 if t < 0.0 => 1.0 - result,
      1 | 2 => result,
      _ => return Some(FormulaValue::Error(FormulaErrorValue::Num)),
    }))
  }

  pub(crate) fn evaluate_t_dist_legacy(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let t = self.number(&self.evaluate(args.first()?)?)?;
    let df = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    let tails = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?);
    if t < 0.0 || df < 1.0 || (tails != 1.0 && tails != 2.0) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(lo_t_dist(t, df, tails as i32)))
  }

  pub(crate) fn evaluate_t_inv(
    &self,
    args: &[FormulaAst<'doc>],
    two_tailed: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let p = self.number(&self.evaluate(args.first()?)?)?;
    let df = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    if df < 1.0 || p <= 0.0 || p > 1.0 || (!two_tailed && p == 1.0) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if two_tailed {
      return match lo_iterate_inverse(|x| p - lo_t_dist(x, df, 2), df * 0.5, df) {
        Ok(value) => Some(FormulaValue::Number(value)),
        Err(error) => Some(FormulaValue::Error(special_error_value(error))),
      };
    }
    if p < 0.5 {
      match lo_iterate_inverse(|x| 1.0 - p - lo_t_dist(x, df, 4), df * 0.5, df) {
        Ok(value) => Some(FormulaValue::Number(-value)),
        Err(error) => Some(FormulaValue::Error(special_error_value(error))),
      }
    } else {
      match lo_iterate_inverse(|x| p - lo_t_dist(x, df, 4), df * 0.5, df) {
        Ok(value) => Some(FormulaValue::Number(value)),
        Err(error) => Some(FormulaValue::Error(special_error_value(error))),
      }
    }
  }

  pub(crate) fn evaluate_t_test(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let left = self.value_numbers(&self.evaluate(args.first()?)?);
    let right = self.value_numbers(&self.evaluate(args.get(1)?)?);
    let tails = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?) as i32;
    let test_type = approx_floor(self.number(&self.evaluate(args.get(3)?)?)?) as i32;
    if left.is_empty()
      || right.is_empty()
      || !(1..=2).contains(&tails)
      || !(1..=3).contains(&test_type)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
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
      if sd_diff == 0.0 {
        return Some(FormulaValue::Error(FormulaErrorValue::Div0));
      }
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
    Some(FormulaValue::Number(lo_t_dist(t, df, tails)))
  }

  pub(crate) fn evaluate_weibull_dist(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let x_value = self.evaluate(args.first()?)?;
    let alpha_value = self.evaluate(args.get(1)?)?;
    let beta_value = self.evaluate(args.get(2)?)?;
    let cumulative_value = self.evaluate(args.get(3)?)?;
    let x_matrix = self.matrix_values(&x_value);
    let alpha_matrix = self.matrix_values(&alpha_value);
    let beta_matrix = self.matrix_values(&beta_value);
    let rows = x_matrix
      .len()
      .max(alpha_matrix.len())
      .max(beta_matrix.len());
    let columns = x_matrix
      .first()
      .map(Vec::len)
      .unwrap_or(1)
      .max(alpha_matrix.first().map(Vec::len).unwrap_or(1))
      .max(beta_matrix.first().map(Vec::len).unwrap_or(1));
    let cumulative = self.truthy(&cumulative_value);
    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let x = self.number(matrix_item(&x_matrix, row, column)?)?;
        let alpha = self.number(matrix_item(&alpha_matrix, row, column)?)?;
        let beta = self.number(matrix_item(&beta_matrix, row, column)?)?;
        if x < 0.0 || alpha <= 0.0 || beta <= 0.0 {
          result_row.push(FormulaValue::Error(FormulaErrorValue::Num));
          continue;
        }
        let dist = Weibull::new(alpha, beta).ok()?;
        result_row.push(FormulaValue::Number(if cumulative {
          dist.cdf(x)
        } else {
          dist.pdf(x)
        }));
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      result.into_iter().next()?.into_iter().next()
    } else {
      Some(FormulaValue::Matrix(result))
    }
  }

  pub(crate) fn evaluate_z_test(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = self.value_numbers_from_ast(args.first()?);
    let x = self.number(&self.evaluate(args.get(1)?)?)?;
    let sigma = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or_else(|| variance_slice(&values, true).unwrap_or(0.0).sqrt());
    if values.len() <= 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    if sigma <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let z = (mean(&values)? - x) / (sigma / (values.len() as f64).sqrt());
    Some(FormulaValue::Number(1.0 - norm_s_dist(z)))
  }

  pub(crate) fn evaluate_standardize(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(x) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(mean) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(sigma) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if sigma <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number((x - mean) / sigma))
  }

  pub(crate) fn evaluate_networkdays(
    &self,
    args: &[FormulaAst<'doc>],
    intl: bool,
  ) -> Option<FormulaValue<'doc>> {
    let mut start = self.number(&self.evaluate(args.first()?)?)?.floor() as i64;
    let mut end = self.number(&self.evaluate(args.get(1)?)?)?.floor() as i64;
    let weekend_arg = if intl {
      args.get(2).and_then(|arg| self.evaluate(arg))
    } else {
      args.get(3).and_then(|arg| self.evaluate(arg))
    };
    let weekend = if intl {
      weekend_mask(weekend_arg.as_ref(), false, self)
    } else if args.len() == 4 {
      old_networkdays_weekend_mask(weekend_arg.as_ref(), self)
    } else {
      weekend_mask(None, false, self)
    };
    let Some(weekend) = weekend else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let holiday_arg = args
      .get(if intl { 3 } else { 2 })
      .and_then(|arg| self.evaluate(arg));
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

  pub(crate) fn evaluate_workday(
    &self,
    args: &[FormulaAst<'doc>],
    intl: bool,
  ) -> Option<FormulaValue<'doc>> {
    let Some(mut date) = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(|value| value.floor() as i64)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(mut days) = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(|value| value.floor() as i64)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let weekend_arg = if intl {
      match args.get(2) {
        Some(arg) if is_missing_argument(arg) => None,
        Some(arg) => {
          let value = self.evaluate(arg)?;
          match value {
            FormulaValue::Reference(reference) => {
              if reference.range.cell_count_hint() != 1 {
                return Some(FormulaValue::Error(FormulaErrorValue::Value));
              }
              Some(self.scalar_reference_value(&reference))
            }
            FormulaValue::RefList(_) => {
              return Some(FormulaValue::Error(FormulaErrorValue::Value));
            }
            value => Some(value),
          }
        }
        None => None,
      }
    } else {
      None
    };
    let Some(weekend) = weekend_mask(weekend_arg.as_ref(), true, self) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let holiday_arg = args
      .get(if intl { 3 } else { 2 })
      .and_then(|arg| self.evaluate(arg));
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

  pub(crate) fn evaluate_subtotal(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(function) = self.number_arg(args, 0).map(|value| value as i32) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
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

  pub(crate) fn evaluate_aggregate(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(function) = self.number_arg(args, 0).map(|value| value as i32) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(options_arg) = self.number_arg(args, 1).map(|value| value as i32) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if !(1..=19).contains(&function) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let Some(options) = aggregate_options(options_arg) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    if (14..=19).contains(&function) && args.len() < 4 {
      return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
    }
    let mut evaluated = Vec::with_capacity(args.len().saturating_sub(2));
    for arg in args.get(2..).unwrap_or_default() {
      evaluated.push(self.evaluate(arg)?);
    }
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
    aggregate_function_value(self, function, data, k, options)
      .map(|result| match result {
        Ok(value) => FormulaValue::Number(value),
        Err(error) => FormulaValue::Error(error),
      })
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_db(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=5).contains(&args.len()) {
      return None;
    }
    let cost = self.number(&self.evaluate(args.first()?)?)?;
    let salvage = self.number(&self.evaluate(args.get(1)?)?)?;
    let life = self.number(&self.evaluate(args.get(2)?)?)?;
    let period = self.number(&self.evaluate(args.get(3)?)?)?;
    let months = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(approx_floor)
      .unwrap_or(12.0);
    if !(1.0..=12.0).contains(&months)
      || life > 1200.0
      || salvage < 0.0
      || period > life + 1.0
      || salvage > cost
      || cost <= 0.0
      || life <= 0.0
      || period <= 0.0
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(financial_db(
      cost, salvage, life, period, months,
    )))
  }

  pub(crate) fn evaluate_price(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(6..=7).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    if has_missing_required_argument(args, &[0, 1, 2, 3, 4, 5]) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let Some(settle) = self
      .date_number_from_value(&self.evaluate(args.first()?)?)
      .map(|value| value.floor() as i32)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(maturity) = self
      .date_number_from_value(&self.evaluate(args.get(1)?)?)
      .map(|value| value.floor() as i32)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(rate) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(yield_value) = self.number_arg(args, 3) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(redemption) = self.number_arg(args, 4) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(frequency) = self.number_arg(args, 5).map(|value| value.floor() as i32) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let basis = args
      .get(6)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0)
      .floor() as i32;
    if yield_value < 0.0
      || rate < 0.0
      || redemption <= 0.0
      || !is_coupon_frequency(frequency)
      || settle >= maturity
      || !(0..=4).contains(&basis)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    finance_price(
      settle,
      maturity,
      rate,
      yield_value,
      redemption,
      frequency,
      basis,
    )
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(
      FormulaErrorValue::IllegalArgument,
    )))
  }

  pub(crate) fn evaluate_yield(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(6..=7).contains(&args.len()) {
      return None;
    }
    if has_missing_required_argument(args, &[0, 1, 2, 3, 4, 5]) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let coupon = self.number_arg(args, 2)?;
    let price = self.number_arg(args, 3)?;
    let redemption = self.number_arg(args, 4)?;
    let frequency = self.number_arg(args, 5)?.floor() as i32;
    let basis = self.optional_basis(args, 6);
    if coupon < 0.0
      || price <= 0.0
      || redemption <= 0.0
      || !is_coupon_frequency(frequency)
      || settle >= maturity
      || !(0..=4).contains(&basis)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    finance_yield(
      settle, maturity, coupon, price, redemption, frequency, basis,
    )
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(
      FormulaErrorValue::IllegalArgument,
    )))
  }

  pub(crate) fn evaluate_pricedisc(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=5).contains(&args.len()) {
      return None;
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let discount = self.number_arg(args, 2)?;
    let redemption = self.number_arg(args, 3)?;
    let basis = self.optional_basis(args, 4);
    if discount <= 0.0 || redemption <= 0.0 || settle >= maturity || !(0..=4).contains(&basis) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    finance_year_diff(settle, maturity, basis)
      .map(|diff| redemption * (1.0 - discount * diff))
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_pricemat(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(5..=6).contains(&args.len()) {
      return None;
    }
    if has_missing_required_argument(args, &[0, 1, 2, 3, 4]) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let issue = self.date_arg(args, 2)?;
    let rate = self.number_arg(args, 3)?;
    let yield_value = self.number_arg(args, 4)?;
    let basis = self.optional_basis(args, 5);
    if rate < 0.0 || yield_value < 0.0 || settle >= maturity || !(0..=4).contains(&basis) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let iss_mat = yearfrac(issue, maturity, basis)?;
    let iss_set = yearfrac(issue, settle, basis)?;
    let set_mat = yearfrac(settle, maturity, basis)?;
    Some(FormulaValue::Number(
      ((1.0 + iss_mat * rate) / (1.0 + set_mat * yield_value) - iss_set * rate) * 100.0,
    ))
  }

  pub(crate) fn evaluate_yielddisc(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=5).contains(&args.len()) {
      return None;
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let price = self.number_arg(args, 2)?;
    let redemption = self.number_arg(args, 3)?;
    let basis = self.optional_basis(args, 4);
    if price <= 0.0 || redemption <= 0.0 || settle >= maturity || !(0..=4).contains(&basis) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    yearfrac(settle, maturity, basis)
      .map(|frac| ((redemption / price) - 1.0) / frac)
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_accrint(
    &self,
    args: &[FormulaAst<'doc>],
    at_maturity: bool,
  ) -> Option<FormulaValue<'doc>> {
    if at_maturity {
      if !(3..=5).contains(&args.len()) {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      }
      let Some(issue) = self.date_arg(args, 0) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      let Some(settle) = self.date_arg(args, 1) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      let Some(rate) = self.number_arg(args, 2) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      let par = self.optional_number_arg(args, 3, 1000.0);
      let basis = self.optional_basis(args, 4);
      if rate <= 0.0 || par <= 0.0 || issue >= settle || !(0..=4).contains(&basis) {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      return finance_year_diff(issue, settle, basis)
        .map(|diff| par * rate * diff)
        .filter(|value| value.is_finite())
        .map(FormulaValue::Number)
        .or(Some(FormulaValue::Error(
          FormulaErrorValue::IllegalArgument,
        )));
    }
    if !(5..=7).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let issue = self.date_arg(args, 0)?;
    let settle = self.date_arg(args, 2)?;
    let rate = self.number_arg(args, 3)?;
    let par = self.optional_number_arg(args, 4, 1000.0);
    let frequency = self.number_arg(args, 5)?.floor() as i32;
    let basis = self.optional_basis(args, 6);
    if rate <= 0.0
      || par <= 0.0
      || !is_coupon_frequency(frequency)
      || issue >= settle
      || !(0..=4).contains(&basis)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    finance_year_diff(issue, settle, basis)
      .map(|diff| par * rate * diff)
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_oddlprice(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(7..=8).contains(&args.len()) {
      return None;
    }
    if has_missing_required_argument(args, &[0, 1, 2, 3, 4, 5, 6]) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let last_interest = self.date_arg(args, 2)?;
    let rate = self.number_arg(args, 3)?;
    let yield_value = self.number_arg(args, 4)?;
    let redemption = self.number_arg(args, 5)?;
    let frequency = self.number_arg(args, 6)?.floor() as i32;
    let basis = self.optional_basis(args, 7);
    if rate <= 0.0
      || yield_value < 0.0
      || redemption <= 0.0
      || !is_coupon_frequency(frequency)
      || maturity <= settle
      || settle <= last_interest
      || !(0..=4).contains(&basis)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    financial_oddlprice(OddPeriodArgs {
      settle,
      maturity,
      last_coupon: last_interest,
      rate,
      value: yield_value,
      redemption,
      frequency,
      basis,
    })
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(
      FormulaErrorValue::IllegalArgument,
    )))
  }

  pub(crate) fn evaluate_oddlyield(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(7..=8).contains(&args.len()) {
      return None;
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let last_interest = self.date_arg(args, 2)?;
    let rate = self.number_arg(args, 3)?;
    let price = self.number_arg(args, 4)?;
    let redemption = self.number_arg(args, 5)?;
    let frequency = self.number_arg(args, 6)?.floor() as i32;
    let basis = self.optional_basis(args, 7);
    if rate <= 0.0
      || price <= 0.0
      || redemption <= 0.0
      || !is_coupon_frequency(frequency)
      || maturity <= settle
      || settle <= last_interest
      || !(0..=4).contains(&basis)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    financial_oddlyield(OddPeriodArgs {
      settle,
      maturity,
      last_coupon: last_interest,
      rate,
      value: price,
      redemption,
      frequency,
      basis,
    })
    .filter(|value| value.is_finite())
    .map(FormulaValue::Number)
    .or(Some(FormulaValue::Error(
      FormulaErrorValue::IllegalArgument,
    )))
  }

  pub(crate) fn evaluate_amorlinc(
    &self,
    args: &[FormulaAst<'doc>],
    degressive: bool,
  ) -> Option<FormulaValue<'doc>> {
    if !(6..=7).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let values = args
      .iter()
      .map(|arg| self.evaluate(arg))
      .collect::<Option<Vec<_>>>()?;
    if values.iter().any(|value| {
      let matrix = self.matrix_values(value);
      matrix.iter().map(Vec::len).sum::<usize>() > 1
    }) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let cost = self.number(values.first()?)?;
    let date = self.date_number_from_value(values.get(1)?)?.floor() as i32;
    let first_period = self.date_number_from_value(values.get(2)?)?.floor() as i32;
    let residual = self.number(values.get(3)?)?;
    let period = self.number(values.get(4)?)?;
    let rate = self.number(values.get(5)?)?;
    let basis = values
      .get(6)
      .and_then(|value| self.number(value))
      .unwrap_or(0.0)
      .floor() as i32;
    if cost <= 0.0
      || residual < 0.0
      || period < 0.0
      || rate <= 0.0
      || date > first_period
      || !(0..=4).contains(&basis)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let result = if degressive {
      financial_amordegrc(cost, date, first_period, residual, period, rate, basis)
    } else {
      financial_amorlinc(cost, date, first_period, residual, period, rate, basis)
    };
    result
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_disc(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=5).contains(&args.len()) {
      return None;
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let price = self.number_arg(args, 2)?;
    let redemption = self.number_arg(args, 3)?;
    let basis = self.optional_basis(args, 4);
    if price <= 0.0 || redemption <= 0.0 || settle >= maturity || !(0..=4).contains(&basis) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    yearfrac(settle, maturity, basis)
      .map(|frac| (1.0 - price / redemption) / frac)
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_received(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=5).contains(&args.len()) {
      return None;
    }
    if has_missing_required_argument(args, &[0, 1, 2, 3]) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let investment = self.number_arg(args, 2)?;
    let discount = self.number_arg(args, 3)?;
    let basis = self.optional_basis(args, 4);
    if investment <= 0.0 || discount <= 0.0 || settle >= maturity || !(0..=4).contains(&basis) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    finance_year_diff(settle, maturity, basis)
      .map(|diff| investment / (1.0 - discount * diff))
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_intrate(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=5).contains(&args.len()) {
      return None;
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let investment = self.number_arg(args, 2)?;
    let redemption = self.number_arg(args, 3)?;
    let basis = self.optional_basis(args, 4);
    if investment <= 0.0 || redemption <= 0.0 || settle >= maturity || !(0..=4).contains(&basis) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    finance_year_diff(settle, maturity, basis)
      .map(|diff| ((redemption / investment) - 1.0) / diff)
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_mduration(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(5..=6).contains(&args.len()) {
      return None;
    }
    if has_missing_required_argument(args, &[0, 1, 2, 3, 4]) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let coupon = self.number_arg(args, 2)?;
    let yield_value = self.number_arg(args, 3)?;
    let frequency = self.number_arg(args, 4)?.floor() as i32;
    let basis = self.optional_basis(args, 5);
    if coupon < 0.0
      || yield_value < 0.0
      || !is_coupon_frequency(frequency)
      || !(0..=4).contains(&basis)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    finance_duration(settle, maturity, coupon, yield_value, frequency, basis)
      .map(|duration| duration / (1.0 + yield_value / f64::from(frequency)))
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_tbilleq(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(settle) = self.date_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(maturity) = self.date_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(discount) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let maturity = maturity + 1;
    let diff = days360(settle, maturity, false)?;
    if discount <= 0.0 || settle >= maturity || diff > 360 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(
      (365.0 * discount) / (360.0 - discount * f64::from(diff)),
    ))
  }

  pub(crate) fn evaluate_tbillprice(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(settle) = self.date_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(maturity) = self.date_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(discount) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if discount <= 0.0 || settle > maturity {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let fraction = yearfrac(settle, maturity + 1, 0)?;
    if fraction.fract() == 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(100.0 * (1.0 - discount * fraction)))
  }

  pub(crate) fn evaluate_tbillyield(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(settle) = self.date_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(maturity) = self.date_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(price) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let diff = days360(settle, maturity, false)? + 1;
    if price <= 0.0 || settle >= maturity || diff > 360 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(
      ((100.0 / price) - 1.0) / f64::from(diff) * 360.0,
    ))
  }

  pub(crate) fn evaluate_coupon(
    &self,
    args: &[FormulaAst<'doc>],
    function: CouponFunction,
  ) -> Option<FormulaValue<'doc>> {
    if !(3..=4).contains(&args.len()) {
      return None;
    }
    let settle = self.date_arg(args, 0)?;
    let maturity = self.date_arg(args, 1)?;
    let frequency = self.number_arg(args, 2)?.floor() as i32;
    let basis = self.optional_basis(args, 3);
    if settle >= maturity || !is_coupon_frequency(frequency) || !(0..=4).contains(&basis) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let result = match function {
      CouponFunction::Days => finance_coupdays(settle, maturity, frequency, basis),
      CouponFunction::DayBs => finance_coupdaybs(settle, maturity, frequency, basis),
      CouponFunction::DaysNc => finance_coupdaysnc(settle, maturity, frequency, basis),
      CouponFunction::Ncd => finance_coupncd(settle, maturity, frequency, basis).map(f64::from),
      CouponFunction::Num => finance_coupnum(settle, maturity, frequency, basis),
      CouponFunction::Pcd => finance_couppcd(settle, maturity, frequency, basis).map(f64::from),
    };
    result
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_pv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=5).contains(&args.len()) {
      return None;
    }
    let rate = self.number_arg(args, 0)?;
    let nper = self.number_arg(args, 1)?;
    let pmt = self.number_arg(args, 2)?;
    let fv = self.number_arg(args, 3).unwrap_or(0.0);
    let pay_in_advance = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    Some(FormulaValue::Number(financial_pv(
      rate,
      nper,
      pmt,
      fv,
      pay_in_advance,
    )))
  }

  pub(crate) fn evaluate_nper(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=5).contains(&args.len()) {
      return None;
    }
    let rate = self.number_arg(args, 0)?;
    let pmt = self.number_arg(args, 1)?;
    let pv = self.number_arg(args, 2)?;
    let fv = self.number_arg(args, 3).unwrap_or(0.0);
    let pay_in_advance = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    financial_nper(rate, pmt, pv, fv, pay_in_advance)
      .filter(|value| value.is_finite())
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn evaluate_rri(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let periods = self.number_arg(args, 0)?;
    let present = self.number_arg(args, 1)?;
    let future = self.number_arg(args, 2)?;
    if periods <= 0.0 || present == 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let result = (future / present).powf(1.0 / periods) - 1.0;
    if result.is_finite() {
      Some(FormulaValue::Number(result))
    } else {
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    }
  }

  pub(crate) fn evaluate_pduration(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let rate = self.number_arg(args, 0)?;
    let present = self.number_arg(args, 1)?;
    let future = self.number_arg(args, 2)?;
    if rate <= 0.0 || present <= 0.0 || future <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(
      (future / present).ln() / (1.0 + rate).ln(),
    ))
  }

  pub(crate) fn evaluate_seriessum(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 4 {
      return None;
    }
    let x = self.number_arg(args, 0)?;
    let mut exponent = self.number_arg(args, 1)?;
    let step = self.number_arg(args, 2)?;
    if x == 0.0 && exponent == 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let mut result = 0.0;
    if x != 0.0 {
      for coefficient in self.value_numbers(&self.evaluate(args.get(3)?)?) {
        result += coefficient * x.powf(exponent);
        exponent += step;
      }
    }
    Some(FormulaValue::Number(result))
  }

  pub(crate) fn evaluate_eastersunday(
    &self,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    if is_multicell_scalar_argument(&value) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let Some(year_number) = self.number(&value) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let mut year = year_number.trunc() as i32;
    if (0..100).contains(&year) {
      year = expand_two_digit_year(year);
    }
    if !(1583..=9956).contains(&year) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let n = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * n + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (n + 11 * h + 22 * l) / 451;
    let o = h + l - 7 * m + 114;
    let day = o % 31 + 1;
    let month = o / 31;
    date_serial(year, month, day)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  pub(crate) fn numeric_binary(
    &self,
    left: FormulaValue<'doc>,
    right: FormulaValue<'doc>,
    op: impl Fn(f64, f64) -> f64 + Copy,
  ) -> Option<FormulaValue<'doc>> {
    if matches!(left, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
      || matches!(right, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
    {
      return self.map_numeric_binary(left, right, op);
    }
    let Some(left) = arithmetic_matrix_number(&left) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let Some(right) = arithmetic_matrix_number(&right) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    Some(FormulaValue::Number(normalize_formula_number(op(
      left, right,
    ))))
  }

  pub(crate) fn map_numeric_binary(
    &self,
    left: FormulaValue<'doc>,
    right: FormulaValue<'doc>,
    op: impl Fn(f64, f64) -> f64 + Copy,
  ) -> Option<FormulaValue<'doc>> {
    let left_matrix = self.matrix_values(&left);
    let right_matrix = self.matrix_values(&right);
    let left_rows = left_matrix.len();
    let left_columns = left_matrix.first().map_or(0, Vec::len);
    let right_rows = right_matrix.len();
    let right_columns = right_matrix.first().map_or(0, Vec::len);
    if left_rows == 0 || left_columns == 0 || right_rows == 0 || right_columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let rows = matrix_binary_extent(left_rows, right_rows);
    let columns = matrix_binary_extent(left_columns, right_columns);

    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let left = &left_matrix[row.min(left_rows - 1)][column.min(left_columns - 1)];
        let right = &right_matrix[row.min(right_rows - 1)][column.min(right_columns - 1)];
        result_row.push(if let Some(error) = propagate_binary_error(left, right) {
          FormulaValue::Error(error)
        } else if let Some((left, right)) =
          arithmetic_operator_matrix_number(left).zip(arithmetic_operator_matrix_number(right))
        {
          FormulaValue::Number(normalize_formula_number(op(left, right)))
        } else {
          FormulaValue::Error(FormulaErrorValue::Value)
        });
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn map_binary_values(
    &self,
    left: FormulaValue<'doc>,
    right: FormulaValue<'doc>,
    op: impl Fn(&Self, &FormulaValue<'doc>, &FormulaValue<'doc>) -> Option<FormulaValue<'doc>> + Copy,
  ) -> Option<FormulaValue<'doc>> {
    let left_matrix = self.matrix_values(&left);
    let right_matrix = self.matrix_values(&right);
    let left_rows = left_matrix.len();
    let left_columns = left_matrix.first().map_or(0, Vec::len);
    let right_rows = right_matrix.len();
    let right_columns = right_matrix.first().map_or(0, Vec::len);
    if left_rows == 0 || left_columns == 0 || right_rows == 0 || right_columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let rows = matrix_binary_extent(left_rows, right_rows);
    let columns = matrix_binary_extent(left_columns, right_columns);

    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let left = &left_matrix[row.min(left_rows - 1)][column.min(left_columns - 1)];
        let right = &right_matrix[row.min(right_rows - 1)][column.min(right_columns - 1)];
        result_row.push(if let Some(error) = propagate_binary_error(left, right) {
          FormulaValue::Error(error)
        } else {
          op(self, left, right)?
        });
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn map_ternary_values(
    &self,
    first: FormulaValue<'doc>,
    second: FormulaValue<'doc>,
    third: FormulaValue<'doc>,
    op: impl Fn(
      &Self,
      &FormulaValue<'doc>,
      &FormulaValue<'doc>,
      &FormulaValue<'doc>,
    ) -> Option<FormulaValue<'doc>>
    + Copy,
  ) -> Option<FormulaValue<'doc>> {
    let first_matrix = self.matrix_values(&first);
    let second_matrix = self.matrix_values(&second);
    let third_matrix = self.matrix_values(&third);
    let first_rows = first_matrix.len();
    let first_columns = first_matrix.first().map_or(0, Vec::len);
    let second_rows = second_matrix.len();
    let second_columns = second_matrix.first().map_or(0, Vec::len);
    let third_rows = third_matrix.len();
    let third_columns = third_matrix.first().map_or(0, Vec::len);
    if first_rows == 0
      || first_columns == 0
      || second_rows == 0
      || second_columns == 0
      || third_rows == 0
      || third_columns == 0
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let rows = matrix_binary_extent(matrix_binary_extent(first_rows, second_rows), third_rows);
    let columns = matrix_binary_extent(
      matrix_binary_extent(first_columns, second_columns),
      third_columns,
    );

    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let first = &first_matrix[row.min(first_rows - 1)][column.min(first_columns - 1)];
        let second = &second_matrix[row.min(second_rows - 1)][column.min(second_columns - 1)];
        let third = &third_matrix[row.min(third_rows - 1)][column.min(third_columns - 1)];
        result_row.push(
          if let Some(error) = first_error_in_values(&[first, second, third]) {
            FormulaValue::Error(error)
          } else {
            op(self, first, second, third)?
          },
        );
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn map_unary_values(
    &self,
    value: FormulaValue<'doc>,
    op: impl Fn(&Self, &FormulaValue<'doc>) -> Option<FormulaValue<'doc>> + Copy,
  ) -> Option<FormulaValue<'doc>> {
    let matrix = self.matrix_values(&value);
    let rows = matrix.len();
    let columns = matrix.first().map_or(0, Vec::len);
    if rows == 0 || columns == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }

    let mut result = Vec::with_capacity(rows);
    for row in matrix {
      let mut result_row = Vec::with_capacity(columns);
      for value in row {
        result_row.push(op(self, &value)?);
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  pub(crate) fn values<'b>(
    &'b self,
    args: &'b [FormulaAst<'doc>],
  ) -> impl Iterator<Item = FormulaValue<'doc>> + 'b {
    args
      .iter()
      .filter_map(|arg| self.evaluate(arg))
      .flat_map(|value| match value {
        FormulaValue::Reference(range) => self.range_values(&range),
        FormulaValue::RefList(ranges) => ranges
          .into_iter()
          .flat_map(|range| self.range_values(&range))
          .collect(),
        FormulaValue::Matrix(rows) => rows.into_iter().flatten().collect(),
        value => vec![value],
      })
  }

  pub(crate) fn numeric_values<'b>(
    &'b self,
    args: &'b [FormulaAst<'doc>],
  ) -> impl Iterator<Item = f64> + 'b {
    // Source: LibreOffice ScInterpreter::GetNumberSequenceArray and
    // ScInterpreter::CalculateSkew use ScValueIterator for range/ref-list
    // arguments, which yields numeric cells and skips empty/text cells. Direct
    // scalar arguments still use normal value conversion, so a direct blank is
    // zero while a blank inside a range is ignored.
    args
      .iter()
      .flat_map(|arg| self.value_numbers_from_ast(arg).into_iter())
  }

  pub(crate) fn numeric_aggregate(
    &self,
    args: &[FormulaAst<'doc>],
    text_error: bool,
  ) -> std::result::Result<NumericAggregate, FormulaErrorValue> {
    let mut values = Vec::new();
    for arg in args {
      let value = self.evaluate(arg).ok_or(FormulaErrorValue::Unknown)?;
      match value {
        FormulaValue::Reference(reference) => {
          self.push_range_numeric_aggregate_values(&reference, &mut values)?;
        }
        FormulaValue::RefList(ranges) => {
          for range in ranges {
            self.push_range_numeric_aggregate_values(&range, &mut values)?;
          }
        }
        FormulaValue::Matrix(rows) => {
          for value in rows.into_iter().flatten() {
            match value {
              FormulaValue::Blank | FormulaValue::String(_) => {}
              value => self.push_direct_numeric_aggregate_value(value, text_error, &mut values)?,
            }
          }
        }
        value => self.push_direct_numeric_aggregate_value(value, text_error, &mut values)?,
      }
    }
    Ok(NumericAggregate { values })
  }

  pub(crate) fn count_numbers(&self, args: &[FormulaAst<'doc>]) -> usize {
    let mut count = 0usize;
    let array_evaluator = self.with_array_context();
    for arg in args {
      let ranges = self.reference_ranges_from_ast(arg);
      if !ranges.is_empty() {
        for range in ranges {
          count += self.count_numbers_in_range(&range);
        }
        continue;
      }
      let Some(value) = array_evaluator.evaluate(arg) else {
        continue;
      };
      match value {
        FormulaValue::Reference(reference) => {
          count += self.count_numbers_in_range(&reference);
        }
        FormulaValue::RefList(ranges) => {
          for range in ranges {
            count += self.count_numbers_in_range(&range);
          }
        }
        FormulaValue::Matrix(rows) => {
          count += rows
            .iter()
            .flatten()
            .filter(|value| matches!(value, FormulaValue::Number(_) | FormulaValue::Boolean(_)))
            .count();
        }
        FormulaValue::Number(_) | FormulaValue::Boolean(_) => count += 1,
        FormulaValue::String(value) => {
          if value.trim().parse::<f64>().is_ok() {
            count += 1;
          }
        }
        FormulaValue::Blank | FormulaValue::Error(_) => {}
      }
    }
    count
  }

  pub(crate) fn count_numbers_in_range(&self, reference: &QualifiedRange<'doc>) -> usize {
    self
      .range_values(reference)
      .iter()
      .filter(|value| matches!(value, FormulaValue::Number(_) | FormulaValue::Boolean(_)))
      .count()
  }

  pub(crate) fn count_all_values(&self, args: &[FormulaAst<'doc>]) -> usize {
    let mut count = 0usize;
    let array_evaluator = self.with_array_context();
    for arg in args {
      if is_missing_argument(arg) {
        count += 1;
        continue;
      }
      let ranges = self.reference_ranges_from_ast(arg);
      if !ranges.is_empty() {
        for range in ranges {
          count += self.count_all_values_in_range(&range);
        }
        continue;
      }
      let Some(value) = array_evaluator.evaluate(arg) else {
        continue;
      };
      match value {
        FormulaValue::Reference(reference) => {
          count += self.count_all_values_in_range(&reference);
        }
        FormulaValue::RefList(ranges) => {
          for range in ranges {
            count += self.count_all_values_in_range(&range);
          }
        }
        FormulaValue::Matrix(rows) => {
          count += rows
            .iter()
            .flatten()
            .filter(|value| !matches!(value, FormulaValue::Blank))
            .count();
        }
        FormulaValue::Blank => {}
        _ => count += 1,
      }
    }
    count
  }

  pub(crate) fn count_all_values_in_range(&self, reference: &QualifiedRange<'doc>) -> usize {
    self
      .range_values(reference)
      .iter()
      .filter(|value| !matches!(value, FormulaValue::Blank))
      .count()
  }

  pub(crate) fn push_range_numeric_aggregate_values(
    &self,
    reference: &QualifiedRange<'doc>,
    values: &mut Vec<f64>,
  ) -> std::result::Result<(), FormulaErrorValue> {
    for value in self.range_values(reference) {
      match value {
        FormulaValue::Number(value) => values.push(value),
        FormulaValue::Boolean(value) => values.push(if value { 1.0 } else { 0.0 }),
        FormulaValue::Error(error) => return Err(error),
        FormulaValue::String(_) | FormulaValue::Blank => {}
        FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => {}
      }
    }
    Ok(())
  }

  pub(crate) fn push_direct_numeric_aggregate_value(
    &self,
    value: FormulaValue<'doc>,
    text_error: bool,
    values: &mut Vec<f64>,
  ) -> std::result::Result<(), FormulaErrorValue> {
    match value {
      FormulaValue::Number(value) => values.push(value),
      FormulaValue::Boolean(value) => values.push(if value { 1.0 } else { 0.0 }),
      FormulaValue::Blank => values.push(0.0),
      FormulaValue::String(value) => {
        if let Ok(value) = value.trim().parse::<f64>() {
          values.push(value);
        } else if text_error {
          return Err(FormulaErrorValue::Value);
        }
      }
      FormulaValue::Error(error) => return Err(error),
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_) => {}
    }
    Ok(())
  }

  pub(crate) fn numeric_args(&self, args: &[FormulaAst<'doc>]) -> Vec<f64> {
    self.numeric_values(args).collect()
  }

  pub(crate) fn median_numeric_args(&self, args: &[FormulaAst<'doc>]) -> Vec<f64> {
    let mut values = Vec::new();
    for arg in args {
      if let Some(value) = self.evaluate(arg) {
        values.extend(self.median_numbers_from_value(&value));
      }
    }
    values
  }

  pub(crate) fn median_numbers_from_value(&self, value: &FormulaValue<'doc>) -> Vec<f64> {
    match value {
      FormulaValue::Reference(reference) => self
        .range_values(reference)
        .iter()
        .filter_map(number_only)
        .collect(),
      FormulaValue::RefList(ranges) => ranges
        .iter()
        .flat_map(|range| self.range_values(range))
        .filter_map(|value| number_only(&value))
        .collect(),
      FormulaValue::Matrix(rows) => rows.iter().flatten().filter_map(number_only).collect(),
      FormulaValue::Boolean(value) => vec![if *value { 1.0 } else { 0.0 }],
      FormulaValue::Number(value) => vec![*value],
      _ => Vec::new(),
    }
  }

  pub(crate) fn st_var_numbers(
    &self,
    args: &[FormulaAst<'doc>],
    direct_text_error: bool,
  ) -> std::result::Result<Vec<f64>, FormulaErrorValue> {
    let mut values = Vec::new();
    for arg in args {
      self.collect_st_var_numbers_from_ast(arg, direct_text_error, &mut values)?;
    }
    Ok(values)
  }

  pub(crate) fn collect_st_var_numbers_from_ast(
    &self,
    ast: &FormulaAst<'doc>,
    direct_text_error: bool,
    values: &mut Vec<f64>,
  ) -> std::result::Result<(), FormulaErrorValue> {
    match ast {
      FormulaAst::Binary {
        op: FormulaOperator::Union,
        left,
        right,
      } => {
        self.collect_st_var_numbers_from_ast(left, direct_text_error, values)?;
        self.collect_st_var_numbers_from_ast(right, direct_text_error, values)?;
      }
      _ => match self.evaluate(ast).unwrap_or_default() {
        FormulaValue::Reference(reference) => {
          values.extend(self.value_numbers(&FormulaValue::Reference(reference)))
        }
        FormulaValue::RefList(ranges) => {
          for range in ranges {
            values.extend(self.value_numbers(&FormulaValue::Reference(range)));
          }
        }
        FormulaValue::Matrix(rows) => {
          values.extend(self.value_numbers(&FormulaValue::Matrix(rows)))
        }
        FormulaValue::String(text) => {
          if let Ok(value) = text.trim().parse::<f64>() {
            values.push(value);
          } else if direct_text_error {
            return Err(FormulaErrorValue::Value);
          }
        }
        FormulaValue::Error(error) => return Err(error),
        value => values.extend(self.number(&value)),
      },
    }
    Ok(())
  }

  pub(crate) fn mode_ms_numeric_args(&self, args: &[FormulaAst<'doc>]) -> Vec<f64> {
    let mut values = Vec::new();
    let array_evaluator = self.with_array_context();
    for arg in args {
      let ranges = self.reference_ranges_from_ast(arg);
      if !ranges.is_empty() {
        for range in ranges {
          self.push_range_numbers_column_major(&range, &mut values);
        }
        continue;
      }
      let Some(value) = array_evaluator.evaluate(arg) else {
        continue;
      };
      match value {
        FormulaValue::Reference(reference) => {
          self.push_range_numbers_column_major(&reference, &mut values);
        }
        FormulaValue::RefList(ranges) => {
          for range in ranges {
            self.push_range_numbers_column_major(&range, &mut values);
          }
        }
        value => values.extend(self.value_numbers(&value)),
      }
    }
    values
  }

  pub(crate) fn push_range_numbers_column_major(
    &self,
    reference: &QualifiedRange<'doc>,
    values: &mut Vec<f64>,
  ) {
    if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
      return;
    }
    let sheet = self.range_sheet(reference);
    let start_column = reference.range.start.column.min(reference.range.end.column);
    let end_column = reference.range.start.column.max(reference.range.end.column);
    let start_row = reference.range.start.row.min(reference.range.end.row);
    let end_row = reference.range.start.row.max(reference.range.end.row);
    for column in start_column..=end_column {
      for row in start_row..=end_row {
        match self.book.cell_value(sheet, CellAddress { column, row }) {
          FormulaValue::Number(value) => values.push(value),
          FormulaValue::Boolean(value) => values.push(if value { 1.0 } else { 0.0 }),
          _ => {}
        }
      }
    }
  }

  pub(crate) fn value_numbers(&self, value: &FormulaValue<'doc>) -> Vec<f64> {
    match value {
      FormulaValue::Reference(reference) => self
        .range_values(reference)
        .iter()
        .filter_map(value_number_for_array)
        .collect(),
      FormulaValue::RefList(ranges) => ranges
        .iter()
        .flat_map(|range| self.range_values(range))
        .filter_map(|value| value_number_for_array(&value))
        .collect(),
      FormulaValue::Matrix(rows) => rows
        .iter()
        .flatten()
        .filter_map(value_number_for_array)
        .collect(),
      value => self.number(value).into_iter().collect(),
    }
  }

  pub(crate) fn value_numbers_from_ast(&self, ast: &FormulaAst<'doc>) -> Vec<f64> {
    let mut values = Vec::new();
    self.collect_value_numbers_from_ast(ast, &mut values);
    values
  }

  pub(crate) fn collect_value_numbers_from_ast(
    &self,
    ast: &FormulaAst<'doc>,
    values: &mut Vec<f64>,
  ) {
    match ast {
      FormulaAst::Binary {
        op: FormulaOperator::Union,
        left,
        right,
      } => {
        self.collect_value_numbers_from_ast(left, values);
        self.collect_value_numbers_from_ast(right, values);
      }
      _ => {
        if let Some(value) = self.evaluate(ast) {
          values.extend(self.value_numbers(&value));
        }
      }
    }
  }

  pub(crate) fn matrix_values(&self, value: &FormulaValue<'doc>) -> Vec<Vec<FormulaValue<'doc>>> {
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
      FormulaValue::RefList(ranges) => ranges
        .iter()
        .flat_map(|range| self.matrix_values(&FormulaValue::Reference(range.clone())))
        .collect(),
      FormulaValue::Matrix(rows) => rows.clone(),
      value => vec![vec![value.clone()]],
    }
  }

  pub(crate) fn lookup_reference_vector(
    &self,
    reference: &QualifiedRange<'doc>,
  ) -> Option<(Vec<FormulaValue<'doc>>, Option<Vec<usize>>, bool)> {
    let sheet = self.range_sheet(reference);
    let original_start_row = reference.range.start.row.min(reference.range.end.row);
    let original_end_row = reference.range.start.row.max(reference.range.end.row);
    let original_start_column = reference.range.start.column.min(reference.range.end.column);
    let original_end_column = reference.range.start.column.max(reference.range.end.column);
    let rows = original_end_row - original_start_row + 1;
    let columns = original_end_column - original_start_column + 1;
    if rows > 1 && columns > 1 {
      return None;
    }
    let range = if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
      self.book.data_area_subrange(sheet, reference.range)?
    } else {
      reference.range
    };
    let start_row = range.start.row.min(range.end.row);
    let end_row = range.start.row.max(range.end.row);
    let start_column = range.start.column.min(range.end.column);
    let end_column = range.start.column.max(range.end.column);
    let vertical = rows >= columns;
    let mut values = Vec::new();
    let mut indices = Vec::new();
    if vertical {
      for row in start_row..=end_row {
        let value = self.book.cell_value(
          sheet,
          CellAddress {
            column: start_column,
            row,
          },
        );
        if !matches!(value, FormulaValue::Blank) {
          values.push(value);
          indices.push((row - original_start_row) as usize);
        }
      }
    } else {
      for column in start_column..=end_column {
        let value = self.book.cell_value(
          sheet,
          CellAddress {
            column,
            row: start_row,
          },
        );
        if !matches!(value, FormulaValue::Blank) {
          values.push(value);
          indices.push((column - original_start_column) as usize);
        }
      }
    }
    Some((values, Some(indices), vertical))
  }

  pub(crate) fn query_grid_from_ast(&self, ast: &FormulaAst<'doc>) -> Option<QueryGrid<'doc>> {
    let value = self.evaluate(ast)?;
    Some(match value {
      FormulaValue::Reference(reference) => self.query_grid_from_reference(&reference),
      FormulaValue::RefList(ranges) if ranges.len() == 1 => {
        self.query_grid_from_reference(ranges.first()?)
      }
      FormulaValue::RefList(ranges) => {
        let mut values = Vec::new();
        let mut query_empty = Vec::new();
        for range in ranges {
          let grid = self.query_grid_from_reference(&range);
          values.extend(grid.values);
          query_empty.extend(grid.query_empty);
        }
        QueryGrid {
          values,
          query_empty,
        }
      }
      FormulaValue::Matrix(values) => {
        let query_empty = values
          .iter()
          .map(|row| vec![false; row.len()])
          .collect::<Vec<_>>();
        QueryGrid {
          values,
          query_empty,
        }
      }
      value => QueryGrid {
        values: vec![vec![value]],
        query_empty: vec![vec![false]],
      },
    })
  }

  pub(crate) fn query_grid_from_reference(
    &self,
    reference: &QualifiedRange<'doc>,
  ) -> QueryGrid<'doc> {
    let sheet = self.range_sheet(reference);
    let range = if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
      let Some(range) = self.book.data_area_subrange(sheet, reference.range) else {
        return QueryGrid {
          values: Vec::new(),
          query_empty: Vec::new(),
        };
      };
      range
    } else {
      reference.range
    };
    let start_row = range.start.row.min(range.end.row);
    let end_row = range.start.row.max(range.end.row);
    let start_column = range.start.column.min(range.end.column);
    let end_column = range.start.column.max(range.end.column);
    let mut values = Vec::new();
    let mut query_empty = Vec::new();
    for row in start_row..=end_row {
      let mut value_row = Vec::new();
      let mut empty_row = Vec::new();
      for column in start_column..=end_column {
        let address = CellAddress { column, row };
        value_row.push(self.book.query_cell_value(
          sheet,
          address,
          self.book.cell_value(sheet, address),
        ));
        empty_row.push(self.book.is_query_empty_cell(sheet, address));
      }
      values.push(value_row);
      query_empty.push(empty_row);
    }
    QueryGrid {
      values,
      query_empty,
    }
  }

  pub(crate) fn count_blank(&self, value: &FormulaValue<'doc>) -> usize {
    match value {
      FormulaValue::Reference(reference) => {
        if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
          return 0;
        }
        let sheet = self.range_sheet(reference);
        let single_cell = reference.range.cell_count_hint() == 1;
        let mut count = 0usize;
        for row in reference.range.start.row..=reference.range.end.row {
          for column in reference.range.start.column..=reference.range.end.column {
            let address = CellAddress { column, row };
            let value = self.book.cell_value(sheet, address);
            let formula = self.book.formula_text(sheet, address).is_some();
            let is_blank = match value {
              FormulaValue::Blank => !formula,
              FormulaValue::String(ref text) => text.is_empty() && (single_cell || formula),
              _ => false,
            };
            if is_blank {
              count += 1;
            }
          }
        }
        count
      }
      FormulaValue::RefList(ranges) => ranges
        .iter()
        .map(|range| self.count_blank(&FormulaValue::Reference(range.clone())))
        .sum(),
      FormulaValue::Matrix(rows) => rows
        .iter()
        .flatten()
        .filter(|value| is_blank_for_countblank(value))
        .count(),
      value if is_blank_for_countblank(value) => 1,
      _ => 0,
    }
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

  pub(crate) fn first_value(&self, value: &FormulaValue<'doc>) -> FormulaValue<'doc> {
    match value {
      FormulaValue::Reference(range) => self
        .range_values(range)
        .into_iter()
        .next()
        .unwrap_or_default(),
      FormulaValue::RefList(ranges) => ranges
        .first()
        .and_then(|range| self.range_values(range).into_iter().next())
        .unwrap_or_default(),
      FormulaValue::Matrix(rows) => rows
        .first()
        .and_then(|row| row.first())
        .cloned()
        .unwrap_or_default(),
      value => value.clone(),
    }
  }

  pub(crate) fn scalar_value(&self, value: FormulaValue<'doc>) -> FormulaValue<'doc> {
    match &value {
      FormulaValue::Reference(reference) => self.scalar_reference_value(reference),
      FormulaValue::RefList(ranges) => ranges
        .first()
        .map(|range| self.scalar_reference_value(range))
        .unwrap_or_default(),
      _ => self.first_value(&value),
    }
  }

  pub(crate) fn scalar_reference_value(
    &self,
    reference: &QualifiedRange<'doc>,
  ) -> FormulaValue<'doc> {
    let sheet = self.range_sheet(reference);
    let start_row = reference.range.start.row.min(reference.range.end.row);
    let end_row = reference.range.start.row.max(reference.range.end.row);
    let start_column = reference.range.start.column.min(reference.range.end.column);
    let end_column = reference.range.start.column.max(reference.range.end.column);
    if let Some(current) = self.current_cell {
      if start_column == end_column && (start_row..=end_row).contains(&current.row) {
        return self.book.cell_value(
          sheet,
          CellAddress {
            column: start_column,
            row: current.row,
          },
        );
      }
      if start_row == end_row && (start_column..=end_column).contains(&current.column) {
        return self.book.cell_value(
          sheet,
          CellAddress {
            column: current.column,
            row: start_row,
          },
        );
      }
    }
    self
      .range_values(reference)
      .into_iter()
      .next()
      .unwrap_or_default()
  }

  pub(crate) fn information_scalar_value(
    &self,
    value: FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    match value {
      FormulaValue::Reference(reference) if reference.range.cell_count_hint() == 1 => {
        self.range_values(&reference).into_iter().next()
      }
      FormulaValue::Reference(_) | FormulaValue::RefList(_) => None,
      FormulaValue::Matrix(rows) => rows
        .into_iter()
        .next()
        .and_then(|row| row.into_iter().next()),
      value => Some(value),
    }
  }

  pub(crate) fn evaluate_information_error(
    &self,
    args: &[FormulaAst<'doc>],
    matches_error: impl Fn(FormulaErrorValue) -> bool + Copy,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    if self.array_context
      && matches!(
        value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      )
    {
      return self.map_unary_values(value, |_, value| {
        Some(FormulaValue::Boolean(matches!(
          value,
          FormulaValue::Error(error) if matches_error(*error)
        )))
      });
    }
    Some(FormulaValue::Boolean(matches!(
      self.first_value(&value),
      FormulaValue::Error(error) if matches_error(error)
    )))
  }

  pub(crate) fn scalar_binary_operand(&self, value: FormulaValue<'doc>) -> FormulaValue<'doc> {
    match value {
      FormulaValue::Reference(reference) => self
        .implicit_intersection_value(&reference)
        .map(|value| self.first_value(&value))
        .unwrap_or(FormulaValue::Error(FormulaErrorValue::Value)),
      FormulaValue::RefList(ranges) => {
        if ranges.len() == 1 {
          self
            .implicit_intersection_value(&ranges[0])
            .map(|value| self.first_value(&value))
            .unwrap_or(FormulaValue::Error(FormulaErrorValue::Value))
        } else {
          FormulaValue::Error(FormulaErrorValue::Value)
        }
      }
      FormulaValue::Matrix(_) => value,
      value => value,
    }
  }

  pub(crate) fn implicit_intersection_value(
    &self,
    reference: &QualifiedRange<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    if reference.range.cell_count_hint() == 1 {
      return self.range_values(reference).into_iter().next();
    }
    let address = self.current_cell?;
    let start_row = reference.range.start.row.min(reference.range.end.row);
    let end_row = reference.range.start.row.max(reference.range.end.row);
    let start_column = reference.range.start.column.min(reference.range.end.column);
    let end_column = reference.range.start.column.max(reference.range.end.column);
    let sheet = self.range_sheet(reference);
    if start_column == end_column && (start_row..=end_row).contains(&address.row) {
      return Some(self.book.cell_value(
        sheet,
        CellAddress {
          column: start_column,
          row: address.row,
        },
      ));
    }
    if start_row == end_row && (start_column..=end_column).contains(&address.column) {
      return Some(self.book.cell_value(
        sheet,
        CellAddress {
          column: address.column,
          row: start_row,
        },
      ));
    }
    None
  }

  pub(crate) fn number(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    match self.first_value(value) {
      FormulaValue::Number(value) => Some(value),
      FormulaValue::Boolean(value) => Some(if value { 1.0 } else { 0.0 }),
      FormulaValue::String(value) => value.trim().parse::<f64>().ok(),
      FormulaValue::Blank => Some(0.0),
      FormulaValue::Error(_) => None,
      FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => None,
    }
  }

  pub(crate) fn number_arg(&self, args: &[FormulaAst<'doc>], index: usize) -> Option<f64> {
    args
      .get(index)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
  }

  pub(crate) fn optional_number_value(&self, arg: &FormulaAst<'doc>) -> Option<f64> {
    self
      .evaluate(arg)
      .filter(|value| !matches!(value, FormulaValue::Blank))
      .and_then(|value| self.number(&value))
  }

  pub(crate) fn optional_number_array_values(&self, arg: &FormulaAst<'doc>) -> Option<Vec<f64>> {
    let value = self.evaluate(arg)?;
    if matches!(value, FormulaValue::Blank) {
      return None;
    }
    let matrix = self.matrix_values(&value);
    let mut values = Vec::new();
    for value in matrix.into_iter().flatten() {
      values.push(self.number(&value)?);
    }
    Some(values)
  }

  pub(crate) fn optional_number_arg(
    &self,
    args: &[FormulaAst<'doc>],
    index: usize,
    default: f64,
  ) -> f64 {
    args
      .get(index)
      .and_then(|arg| self.optional_number_value(arg))
      .unwrap_or(default)
  }

  pub(crate) fn date_arg(&self, args: &[FormulaAst<'doc>], index: usize) -> Option<i32> {
    args
      .get(index)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.date_number_from_value(&value))
      .map(|value| value.floor() as i32)
  }

  pub(crate) fn optional_basis(&self, args: &[FormulaAst<'doc>], index: usize) -> i32 {
    args
      .get(index)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0)
      .floor() as i32
  }

  pub(crate) fn evaluate_numeric_unary(
    &self,
    args: &[FormulaAst<'doc>],
    op: impl Fn(f64) -> f64 + Copy,
  ) -> Option<FormulaValue<'doc>> {
    self.evaluate_numeric_unary_checked(args, |value| Some(op(value)))
  }

  pub(crate) fn evaluate_len(
    &self,
    args: &[FormulaAst<'doc>],
    bytes: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    let len = |evaluator: &Self, value: &FormulaValue<'doc>| {
      let text = evaluator.text(value);
      if bytes {
        text_byte_len(&text)
      } else {
        text.chars().count()
      }
    };
    if self.array_context
      && matches!(
        value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      )
    {
      return self.map_unary_values(value, |evaluator, value| {
        Some(FormulaValue::Number(len(evaluator, value) as f64))
      });
    }
    Some(FormulaValue::Number(len(self, &value) as f64))
  }

  pub(crate) fn evaluate_numeric_unary_checked(
    &self,
    args: &[FormulaAst<'doc>],
    op: impl Fn(f64) -> Option<f64> + Copy,
  ) -> Option<FormulaValue<'doc>> {
    self.evaluate_numeric_unary_checked_error(args, op, FormulaErrorValue::Unknown)
  }

  pub(crate) fn evaluate_numeric_unary_checked_error(
    &self,
    args: &[FormulaAst<'doc>],
    op: impl Fn(f64) -> Option<f64> + Copy,
    error: FormulaErrorValue,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    if (self.array_context
      && matches!(
        value,
        FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
      ))
      || matches!(value, FormulaValue::Matrix(_))
    {
      return self.map_unary_values(value, |evaluator, value| {
        if let FormulaValue::Error(error) = value {
          return Some(FormulaValue::Error(*error));
        }
        evaluator.number(value).map(|value| match op(value) {
          Some(result) if result.is_finite() => FormulaValue::Number(result),
          Some(_) => FormulaValue::Error(FormulaErrorValue::Value),
          None => FormulaValue::Error(error),
        })
      });
    }
    let value = self.scalar_binary_operand(value);
    if let FormulaValue::Error(error) = value {
      return Some(FormulaValue::Error(error));
    }
    self.number(&value).map(|value| match op(value) {
      Some(result) if result.is_finite() => FormulaValue::Number(result),
      Some(_) => FormulaValue::Error(FormulaErrorValue::Value),
      None => FormulaValue::Error(error),
    })
  }

  pub(crate) fn date_number_from_value(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    self.date_number_from_scalar(&self.first_value(value))
  }

  pub(crate) fn date_number_from_scalar(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    match value {
      FormulaValue::String(text) => match datevalue(text, self.book.date_system) {
        FormulaValue::Number(value) => Some(value),
        _ => None,
      },
      value => self.number(value),
    }
  }

  pub(crate) fn text(&self, value: &FormulaValue<'doc>) -> String {
    display_text_from_value(&self.first_value(value))
  }

  pub(crate) fn truthy(&self, value: &FormulaValue<'doc>) -> bool {
    match self.first_value(value) {
      FormulaValue::Boolean(value) => value,
      FormulaValue::Number(value) => value != 0.0,
      FormulaValue::String(value) => !value.is_empty(),
      FormulaValue::Blank | FormulaValue::Error(_) => false,
      FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => false,
    }
  }

  pub(crate) fn compare(
    &self,
    left: &FormulaValue<'doc>,
    right: &FormulaValue<'doc>,
    op: FormulaOperator,
  ) -> bool {
    let ordering = if let Some((left, right)) = self.number(left).zip(self.number(right)) {
      match compare_numbers(left, right) {
        -1 => Some(std::cmp::Ordering::Less),
        0 => Some(std::cmp::Ordering::Equal),
        1 => Some(std::cmp::Ordering::Greater),
        _ => None,
      }
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

  pub(crate) fn range_sheet(&self, range: &QualifiedRange<'doc>) -> SheetId {
    range
      .sheet_name
      .as_ref()
      .and_then(|name| self.book.sheet_id_by_name(name.0.as_ref()))
      .unwrap_or(range.sheet)
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SumProductScalar {
  Number(f64),
  Error(FormulaErrorValue),
  NaN,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CouponFunction {
  Days,
  DayBs,
  DaysNc,
  Ncd,
  Num,
  Pcd,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum IfsAggregate {
  Sum,
  Count,
  Average,
  Min,
  Max,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DatabaseFunction {
  Sum,
  Count,
  CountA,
  Average,
  Get,
  Max,
  Min,
  Product,
  Var,
  VarP,
  StdDev,
  StdDevP,
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
struct QueryItem<'doc> {
  value: FormulaValue<'doc>,
  source_text: Option<Cow<'doc, str>>,
  kind: QueryValueKind,
  match_empty: bool,
  empty_matches_text: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct QueryEntry<'doc> {
  op: QueryOp,
  field: usize,
  item: QueryItem<'doc>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct QueryParam<'doc> {
  entries: Vec<QueryEntry<'doc>>,
  search_type: QuerySearchType,
  range_lookup: bool,
  match_whole_cell: bool,
  case_sensitive: bool,
}

impl<'doc> QueryParam<'doc> {
  fn single(entry: QueryEntry<'doc>, search_type: QuerySearchType, match_whole_cell: bool) -> Self {
    Self {
      entries: vec![entry],
      search_type,
      range_lookup: false,
      match_whole_cell,
      case_sensitive: false,
    }
  }

  fn from_criterion(
    evaluator: &FormulaEvaluator<'_, 'doc>,
    value: &FormulaValue<'doc>,
    field: usize,
  ) -> Self {
    let (entry, search_type) = QueryEntry::from_value(evaluator, value, field);
    Self::single(entry, search_type, evaluator.book.formula_match_whole_cell)
  }

  fn with_range_lookup(mut self, range_lookup: bool) -> Self {
    self.range_lookup = range_lookup;
    self
  }

  fn matches_value(
    &self,
    evaluator: &FormulaEvaluator<'_, 'doc>,
    candidate: &FormulaValue<'doc>,
    candidate_query_empty: bool,
  ) -> bool {
    QueryEvaluator {
      evaluator,
      param: self,
    }
    .matches_value(candidate, candidate_query_empty)
  }

  fn matches_row_with_empty(
    &self,
    evaluator: &FormulaEvaluator<'_, 'doc>,
    row: &[FormulaValue<'doc>],
    query_empty: &[bool],
  ) -> bool {
    QueryEvaluator {
      evaluator,
      param: self,
    }
    .matches_row_with_empty(row, query_empty)
  }
}

struct QueryEvaluator<'eval, 'ctx, 'doc> {
  evaluator: &'eval FormulaEvaluator<'ctx, 'doc>,
  param: &'eval QueryParam<'doc>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct QueryGrid<'doc> {
  values: Vec<Vec<FormulaValue<'doc>>>,
  query_empty: Vec<Vec<bool>>,
}

impl<'doc> QueryGrid<'doc> {
  fn dimensions(&self) -> (usize, usize) {
    matrix_dimensions(&self.values)
  }
}

impl<'eval, 'ctx, 'doc> QueryEvaluator<'eval, 'ctx, 'doc> {
  fn matches_value(&self, candidate: &FormulaValue<'doc>, candidate_query_empty: bool) -> bool {
    self
      .param
      .entries
      .first()
      .is_some_and(|entry| self.matches_entry(entry, candidate, candidate_query_empty))
  }

  fn matches_row_with_empty(&self, row: &[FormulaValue<'doc>], query_empty: &[bool]) -> bool {
    self.param.entries.iter().all(|entry| {
      row.get(entry.field).is_some_and(|candidate| {
        self.matches_entry(
          entry,
          candidate,
          query_empty.get(entry.field).copied().unwrap_or(false),
        )
      })
    })
  }

  fn matches_entry(
    &self,
    entry: &QueryEntry<'doc>,
    candidate: &FormulaValue<'doc>,
    candidate_query_empty: bool,
  ) -> bool {
    query_matches(
      self.evaluator,
      self.param,
      entry,
      candidate,
      candidate_query_empty,
    )
  }
}

impl<'doc> QueryEntry<'doc> {
  fn from_value(
    evaluator: &FormulaEvaluator<'_, 'doc>,
    value: &FormulaValue<'doc>,
    field: usize,
  ) -> (Self, QuerySearchType) {
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
      (
        Self {
          op,
          field,
          item: QueryItem {
            value: operand_value,
            source_text,
            kind,
            match_empty: (op == QueryOp::Equal && is_empty_criterion)
              || (op == QueryOp::NotEqual && !is_empty_criterion),
            empty_matches_text: is_empty_criterion && !explicit_empty_operator,
          },
        },
        search_type,
      )
    } else {
      let value = if matches!(value, FormulaValue::Blank) {
        FormulaValue::Number(0.0)
      } else {
        value
      };
      (
        Self {
          op: QueryOp::Equal,
          field,
          item: QueryItem {
            kind: query_value_kind(&value),
            value,
            source_text: None,
            match_empty: false,
            empty_matches_text: false,
          },
        },
        QuerySearchType::Normal,
      )
    }
  }

  fn from_database_value(
    evaluator: &FormulaEvaluator<'_, 'doc>,
    value: &FormulaValue<'doc>,
    field: usize,
  ) -> (Self, QuerySearchType) {
    let value = evaluator.first_value(value);
    if let FormulaValue::String(text) = value {
      let (op, operand) = parse_criteria_operator(text.as_ref());
      let trimmed = operand.trim();
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
      } else {
        QueryValueKind::Text
      };
      (
        Self {
          op,
          field,
          item: QueryItem {
            value: operand_value,
            source_text,
            kind,
            match_empty: false,
            empty_matches_text: false,
          },
        },
        search_type,
      )
    } else {
      let value = if matches!(value, FormulaValue::Blank) {
        FormulaValue::Number(0.0)
      } else {
        value
      };
      (
        Self {
          op: QueryOp::Equal,
          field,
          item: QueryItem {
            kind: query_value_kind(&value),
            value,
            source_text: None,
            match_empty: false,
            empty_matches_text: false,
          },
        },
        QuerySearchType::Normal,
      )
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LookupSearchMode {
  Forward,
  Reverse,
  BinaryAscending,
  BinaryDescending,
}

impl LookupSearchMode {
  fn from_excel(value: i32) -> Option<Self> {
    match value {
      1 => Some(Self::Forward),
      -1 => Some(Self::Reverse),
      2 => Some(Self::BinaryAscending),
      -2 => Some(Self::BinaryDescending),
      _ => None,
    }
  }
}

fn query_matches<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  param: &QueryParam<'doc>,
  query: &QueryEntry<'doc>,
  candidate: &FormulaValue<'doc>,
  candidate_query_empty: bool,
) -> bool {
  if matches!(candidate, FormulaValue::Error(_)) {
    return false;
  }
  if matches!(
    query.item.kind,
    QueryValueKind::Empty | QueryValueKind::NonEmpty
  ) {
    let blank = candidate_query_empty
      || matches!(candidate, FormulaValue::Blank)
      || (query.item.empty_matches_text
        && matches!(candidate, FormulaValue::String(text) if text.is_empty()));
    return if query.item.kind == QueryValueKind::Empty {
      blank
    } else {
      !blank
    };
  }
  if query.item.match_empty && (candidate_query_empty || is_query_empty(candidate)) {
    return matches!(
      query.op,
      QueryOp::NotEqual | QueryOp::LessOrEqual | QueryOp::GreaterOrEqual
    );
  }
  if !param.range_lookup
    && query.item.kind == QueryValueKind::Number
    && query_candidate_number(candidate, candidate_query_empty).is_none()
  {
    if let Some(source_text) = &query.item.source_text
      && let FormulaValue::String(candidate_text) = candidate
      && matches!(query.op, QueryOp::Equal | QueryOp::NotEqual)
    {
      let matched = if param.match_whole_cell {
        compare_text(evaluator, candidate_text, source_text, param.case_sensitive) == 0
      } else if param.case_sensitive {
        candidate_text.contains(source_text.as_ref())
      } else {
        lookup_text_contains(candidate_text, source_text)
      };
      return if query.op == QueryOp::Equal {
        matched
      } else {
        !matched
      };
    }
    return matches!(query.op, QueryOp::NotEqual);
  }
  if !param.range_lookup
    && query.item.kind == QueryValueKind::Text
    && matches!(candidate, FormulaValue::Number(_))
  {
    return matches!(query.op, QueryOp::NotEqual);
  }
  if param.search_type == QuerySearchType::Wildcard {
    let FormulaValue::String(pattern) = &query.item.value else {
      return false;
    };
    let text = evaluator.text(candidate);
    let matched = if param.match_whole_cell {
      wildcard_match(pattern.as_ref(), &text)
    } else {
      wildcard_match(pattern.as_ref(), &text) || lookup_text_contains(&text, pattern.as_ref())
    };
    return match query.op {
      QueryOp::Equal => matched,
      QueryOp::NotEqual => !matched,
      _ => false,
    };
  }
  if param.search_type == QuerySearchType::Regex {
    let FormulaValue::String(pattern) = &query.item.value else {
      return false;
    };
    let matched =
      regex_match(pattern, &evaluator.text(candidate), param.match_whole_cell).unwrap_or(false);
    return match query.op {
      QueryOp::Equal => matched,
      QueryOp::NotEqual => !matched,
      _ => false,
    };
  }
  if !param.match_whole_cell
    && matches!(query.op, QueryOp::Equal | QueryOp::NotEqual)
    && let (FormulaValue::String(candidate_text), FormulaValue::String(query_text)) =
      (candidate, &query.item.value)
  {
    let matched = if param.case_sensitive {
      candidate_text.contains(query_text.as_ref())
    } else {
      lookup_text_contains(candidate_text, query_text)
    };
    return if query.op == QueryOp::Equal {
      matched
    } else {
      !matched
    };
  }
  let ordering = query_compare_value(
    evaluator,
    candidate,
    candidate_query_empty,
    &query.item.value,
    param,
  );
  if ordering.is_none() && param.range_lookup {
    return query_compare_by_range_lookup(query, candidate);
  }
  match query.op {
    QueryOp::Equal => ordering == Some(0),
    QueryOp::NotEqual => ordering != Some(0),
    QueryOp::Less => ordering == Some(-1),
    QueryOp::LessOrEqual => matches!(ordering, Some(-1 | 0)),
    QueryOp::Greater => ordering == Some(1),
    QueryOp::GreaterOrEqual => matches!(ordering, Some(0 | 1)),
  }
}

fn query_compare_by_range_lookup(query: &QueryEntry<'_>, candidate: &FormulaValue<'_>) -> bool {
  match query.item.kind {
    QueryValueKind::Text if !matches!(query.op, QueryOp::Less | QueryOp::LessOrEqual) => false,
    QueryValueKind::Text => query_candidate_number(candidate, false).is_some(),
    _ if !matches!(query.op, QueryOp::Greater | QueryOp::GreaterOrEqual) => false,
    _ => query_candidate_number(candidate, false).is_none(),
  }
}

fn query_compare_value<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  candidate: &FormulaValue<'doc>,
  candidate_query_empty: bool,
  query: &FormulaValue<'doc>,
  param: &QueryParam<'doc>,
) -> Option<i32> {
  if let Some((candidate, query)) =
    query_candidate_number(candidate, candidate_query_empty).zip(evaluator.number(query))
  {
    return Some(compare_numbers(candidate, query));
  }
  match (candidate, query) {
    (FormulaValue::String(candidate), FormulaValue::String(query)) => Some(compare_text(
      evaluator,
      candidate,
      query,
      param.case_sensitive,
    )),
    (FormulaValue::Blank, FormulaValue::Blank) => Some(0),
    (FormulaValue::Blank, _) if param.range_lookup => Some(-1),
    (_, FormulaValue::Blank) if param.range_lookup => Some(1),
    (FormulaValue::Number(_), FormulaValue::String(_)) if param.range_lookup => None,
    (FormulaValue::String(_), FormulaValue::Number(_)) if param.range_lookup => None,
    (FormulaValue::Boolean(left), FormulaValue::Boolean(right)) => Some(match left.cmp(right) {
      std::cmp::Ordering::Less => -1,
      std::cmp::Ordering::Equal => 0,
      std::cmp::Ordering::Greater => 1,
    }),
    _ => None,
  }
}

fn query_candidate_number(value: &FormulaValue<'_>, query_empty: bool) -> Option<f64> {
  if query_empty {
    return Some(0.0);
  }
  match value {
    FormulaValue::Number(value) => Some(*value),
    FormulaValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
    _ => None,
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

fn database_criterion_cell_present(value: &FormulaValue<'_>, query_empty: bool) -> bool {
  query_empty || !matches!(value, FormulaValue::Blank)
}

fn database_criterion_entry_present(value: &FormulaValue<'_>, query_empty: bool) -> bool {
  database_criterion_cell_present(value, query_empty)
    && !matches!(value, FormulaValue::String(text) if text.is_empty())
}

fn compare_numbers(left: f64, right: f64) -> i32 {
  if approx_equal(left, right) {
    0
  } else if left < right {
    -1
  } else {
    1
  }
}

fn rtl_valid_trig_arg(value: f64) -> bool {
  value.abs() <= (0x8000_0000u64 as f64) * (0x8000_0000u64 as f64) * 4.0
}

pub(crate) fn rtl_sin(value: f64) -> f64 {
  if rtl_valid_trig_arg(value) {
    value.sin()
  } else {
    f64::NAN
  }
}

pub(crate) fn rtl_cos(value: f64) -> f64 {
  if rtl_valid_trig_arg(value) {
    value.cos()
  } else {
    f64::NAN
  }
}

pub(crate) fn rtl_tan(value: f64) -> f64 {
  if rtl_valid_trig_arg(value) {
    value.tan()
  } else {
    f64::NAN
  }
}

fn compare_text(
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

#[derive(Clone, Copy)]
struct SearchVectorFlags {
  exact_type: bool,
  match_empty: bool,
  first_exact: bool,
}

impl SearchVectorFlags {
  fn new(exact_type: bool, match_empty: bool) -> Self {
    Self {
      exact_type,
      match_empty,
      first_exact: false,
    }
  }

  fn with_first_exact(mut self) -> Self {
    self.first_exact = true;
    self
  }
}

fn search_vector<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  lookup: &FormulaValue<'doc>,
  vector: &[FormulaValue<'doc>],
  op: QueryOp,
  mode: LookupSearchMode,
  exact_type: bool,
) -> Option<usize> {
  let search_type = if let FormulaValue::String(text) = lookup {
    detect_query_search_type(evaluator.book.formula_search_type, text)
  } else {
    QuerySearchType::Normal
  };
  search_vector_with_type(
    evaluator,
    lookup,
    vector,
    op,
    mode,
    search_type,
    SearchVectorFlags::new(exact_type, false),
  )
}

fn match_range_linear_index<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  lookup: &FormulaValue<'doc>,
  vector: &[FormulaValue<'doc>],
  mode: i32,
) -> Option<usize> {
  // Source: LibreOffice sc/source/core/data/queryiter.cxx
  // ScQueryCellIterator::FindEqualOrSortedLastInRange for horizontal MATCH
  // ranges.  Horizontal document ranges use the direct iterator path, not the
  // matrix binary search path.
  let op = match mode {
    1 => QueryOp::LessOrEqual,
    -1 => QueryOp::GreaterOrEqual,
    _ => return None,
  };
  let query = QueryEntry {
    op,
    field: 0,
    item: QueryItem {
      kind: query_value_kind(lookup),
      value: lookup.clone(),
      source_text: None,
      match_empty: false,
      empty_matches_text: false,
    },
  };
  let param = QueryParam::single(query, QuerySearchType::Normal, true).with_range_lookup(true);
  let query = param.entries.first()?;
  let mut found = None;
  let mut first_string_ignore = matches!(lookup, FormulaValue::String(_));
  for (index, candidate) in vector.iter().enumerate() {
    let exact = lookup_types_compatible(evaluator, lookup, candidate)
      && evaluator.compare(candidate, lookup, FormulaOperator::Equal);
    let valid = query_matches(evaluator, &param, query, candidate, false);
    if valid {
      found = Some(index);
      if exact {
        let mut last_equal = index;
        for (next_index, next) in vector.iter().enumerate().skip(index + 1) {
          if lookup_types_compatible(evaluator, lookup, next)
            && evaluator.compare(next, lookup, FormulaOperator::Equal)
          {
            last_equal = next_index;
          } else {
            break;
          }
        }
        return Some(last_equal);
      }
    } else if first_string_ignore && matches!(candidate, FormulaValue::String(_)) {
      first_string_ignore = false;
      continue;
    } else {
      break;
    }
    first_string_ignore = false;
  }
  found.filter(|index| lookup_candidate_type_matches(query, &vector[*index]))
}

fn search_vector_with_type<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  lookup: &FormulaValue<'doc>,
  vector: &[FormulaValue<'doc>],
  op: QueryOp,
  mode: LookupSearchMode,
  search_type: QuerySearchType,
  flags: SearchVectorFlags,
) -> Option<usize> {
  let range_lookup = !matches!(op, QueryOp::Equal | QueryOp::NotEqual);
  let query = QueryEntry {
    op,
    field: 0,
    item: QueryItem {
      kind: query_value_kind(lookup),
      value: lookup.clone(),
      source_text: None,
      match_empty: flags.match_empty,
      empty_matches_text: false,
    },
  };
  let param = QueryParam::single(query, search_type, true).with_range_lookup(range_lookup);
  let query = param.entries.first()?;
  match mode {
    LookupSearchMode::BinaryAscending | LookupSearchMode::BinaryDescending => {
      if search_type != QuerySearchType::Normal {
        return None;
      }
      lookup_binary_search(
        evaluator,
        vector,
        query,
        &param,
        matches!(mode, LookupSearchMode::BinaryAscending),
        true,
        flags.first_exact,
      )
    }
    LookupSearchMode::Forward => {
      let mut found = None;
      for (index, candidate) in vector.iter().enumerate() {
        if flags.exact_type && !lookup_types_compatible(evaluator, lookup, candidate) {
          continue;
        }
        if query_matches(evaluator, &param, query, candidate, false) {
          match op {
            QueryOp::Equal => {
              found = Some(index);
              break;
            }
            QueryOp::LessOrEqual => {
              if lookup_candidate_type_matches(query, candidate)
                && found.is_none_or(|found_index| {
                  lookup_compare_cells(evaluator, candidate, &vector[found_index]) >= 0
                })
              {
                found = Some(index);
              }
            }
            QueryOp::GreaterOrEqual => {
              if lookup_candidate_type_matches(query, candidate)
                && found.is_none_or(|found_index| {
                  lookup_compare_cells(evaluator, candidate, &vector[found_index]) <= 0
                })
              {
                found = Some(index);
              }
            }
            _ => {
              if lookup_candidate_type_matches(query, candidate) {
                found = Some(index);
              }
            }
          }
        }
      }
      found
    }
    LookupSearchMode::Reverse => {
      let mut found = None;
      for (index, candidate) in vector.iter().enumerate().rev() {
        if flags.exact_type && !lookup_types_compatible(evaluator, lookup, candidate) {
          continue;
        }
        if query_matches(evaluator, &param, query, candidate, false) {
          match op {
            QueryOp::Equal => {
              found = Some(index);
              break;
            }
            QueryOp::LessOrEqual => {
              if lookup_candidate_type_matches(query, candidate)
                && found.is_none_or(|found_index| {
                  lookup_compare_cells(evaluator, candidate, &vector[found_index]) > 0
                })
              {
                found = Some(index);
              }
            }
            QueryOp::GreaterOrEqual => {
              if lookup_candidate_type_matches(query, candidate)
                && found.is_none_or(|found_index| {
                  lookup_compare_cells(evaluator, candidate, &vector[found_index]) < 0
                })
              {
                found = Some(index);
              }
            }
            _ => {
              if lookup_candidate_type_matches(query, candidate) {
                found = Some(index);
              }
            }
          }
        }
      }
      found
    }
  }
}

fn lookup_binary_search<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  vector: &[FormulaValue<'doc>],
  query: &QueryEntry<'doc>,
  param: &QueryParam<'doc>,
  sorted_ascending: bool,
  empty_is_less: bool,
  first_exact: bool,
) -> Option<usize> {
  if vector.is_empty() {
    return None;
  }
  let mut low = 0usize;
  let mut high = vector.len();
  let mut exact = None;
  while low < high {
    let mid = low + (high - low) / 2;
    let cmp =
      lookup_compare_candidate_to_query(evaluator, &vector[mid], query, param, empty_is_less)?;
    if cmp == 0 {
      exact = Some(mid);
      break;
    }
    if sorted_ascending {
      if cmp < 0 {
        low = mid + 1;
      } else {
        high = mid;
      }
    } else {
      if cmp > 0 {
        low = mid + 1;
      } else {
        high = mid;
      }
    }
  }
  if let Some(mut index) = exact {
    if first_exact && sorted_ascending {
      while index > 0 && lookup_same_match_value(&vector[index], &vector[index - 1]) {
        index -= 1;
      }
    } else {
      while index + 1 < vector.len() && lookup_same_match_value(&vector[index], &vector[index + 1])
      {
        index += 1;
      }
    }
    return lookup_binary_result_index(vector, query, index);
  }
  if query.op == QueryOp::Equal {
    return None;
  }
  let index = match (sorted_ascending, query.op) {
    (true, QueryOp::LessOrEqual) => low.checked_sub(1),
    (true, QueryOp::GreaterOrEqual) => (low < vector.len()).then_some(low),
    (false, QueryOp::LessOrEqual) => (low < vector.len()).then_some(low),
    (false, QueryOp::GreaterOrEqual) => low.checked_sub(1),
    _ => None,
  }?;
  lookup_binary_result_index(vector, query, index)
}

fn lookup_binary_result_index<'doc>(
  vector: &[FormulaValue<'doc>],
  query: &QueryEntry<'doc>,
  index: usize,
) -> Option<usize> {
  (lookup_candidate_type_matches(query, vector.get(index)?)).then_some(index)
}

fn lookup_candidate_type_matches(query: &QueryEntry<'_>, candidate: &FormulaValue<'_>) -> bool {
  match query.item.kind {
    QueryValueKind::Text => !matches!(candidate, FormulaValue::Number(_)),
    QueryValueKind::Number => query_candidate_number(candidate, false).is_some(),
    _ => true,
  }
}

fn lookup_compare_candidate_to_query<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  candidate: &FormulaValue<'doc>,
  query: &QueryEntry<'doc>,
  param: &QueryParam<'doc>,
  empty_is_less: bool,
) -> Option<i32> {
  if matches!(candidate, FormulaValue::Blank) {
    return Some(if empty_is_less { -1 } else { 1 });
  }
  query_compare_value(evaluator, candidate, false, &query.item.value, param).or_else(|| match query
    .item
    .kind
  {
    QueryValueKind::Text if query_candidate_number(candidate, false).is_some() => Some(-1),
    QueryValueKind::Number if matches!(candidate, FormulaValue::String(_)) => Some(1),
    _ => None,
  })
}

fn lookup_compare_cells<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  left: &FormulaValue<'doc>,
  right: &FormulaValue<'doc>,
) -> i32 {
  match (left, right) {
    (FormulaValue::Blank, FormulaValue::Blank) => 0,
    (FormulaValue::Blank, _) => -1,
    (_, FormulaValue::Blank) => 1,
    (FormulaValue::Error(_), FormulaValue::Error(_)) => 0,
    (FormulaValue::Error(_), _) => 1,
    (_, FormulaValue::Error(_)) => -1,
    _ => {
      let left_number = query_candidate_number(left, false);
      let right_number = query_candidate_number(right, false);
      match (left_number, right_number) {
        (Some(left), Some(right)) => compare_numbers(left, right),
        (Some(_), None) => -1,
        (None, Some(_)) => 1,
        (None, None) => compare_text(
          evaluator,
          &evaluator.text(left),
          &evaluator.text(right),
          false,
        ),
      }
    }
  }
}

fn lookup_same_match_value(left: &FormulaValue<'_>, right: &FormulaValue<'_>) -> bool {
  match (left, right) {
    (FormulaValue::Number(left), FormulaValue::Number(right)) => left == right,
    (FormulaValue::Boolean(left), FormulaValue::Boolean(right)) => left == right,
    (FormulaValue::Blank, FormulaValue::Blank) => true,
    (FormulaValue::String(left), FormulaValue::String(right)) => left == right,
    (FormulaValue::Error(_), FormulaValue::Error(_)) => true,
    _ => false,
  }
}

fn lookup_types_compatible<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  lookup: &FormulaValue<'doc>,
  candidate: &FormulaValue<'doc>,
) -> bool {
  match (lookup, candidate) {
    (FormulaValue::Blank, FormulaValue::Blank) => true,
    (FormulaValue::Blank, _) | (_, FormulaValue::Blank) => false,
    (FormulaValue::String(_), FormulaValue::String(_)) => true,
    (FormulaValue::String(_), _) => false,
    (_, FormulaValue::String(_)) => false,
    _ => evaluator.number(lookup).is_some() == evaluator.number(candidate).is_some(),
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DatePart {
  Year,
  Month,
  Day,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TimePart {
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
  if function == 2 {
    return Some(Ok(aggregate_count_numbers(evaluator, args, options) as f64));
  }
  if function == 3 {
    return aggregate_counta(evaluator, args, options)
      .map(|result| result.map(|count| count as f64));
  }
  let values = match aggregate_numbers(evaluator, args, options) {
    Ok(values) => values,
    Err(error) => return Some(Err(error)),
  };
  match function {
    1 => mean(&values),
    2 => Some(values.len() as f64),
    4 => Some(values.into_iter().reduce(f64::max).unwrap_or(0.0)),
    5 => Some(values.into_iter().reduce(f64::min).unwrap_or(0.0)),
    6 => Some(values.into_iter().product()),
    7 => variance_slice(&values, true).map(f64::sqrt),
    8 => variance_slice(&values, false).map(f64::sqrt),
    9 => Some(kahan_sum(values)),
    10 => variance_slice(&values, true),
    11 => variance_slice(&values, false),
    12 => {
      let mut values = values;
      percentile_sorted(&mut values, 0.5, PercentileKind::Inc)
    }
    13 => mode_slice(&values),
    14 => kth_value(values, k?.ceil(), true),
    15 => kth_value(values, k?.floor(), false),
    16 => {
      let mut values = values;
      percentile_sorted(&mut values, k?, PercentileKind::Inc)
    }
    17 => {
      let mut values = values;
      percentile_sorted(&mut values, k?.floor() / 4.0, PercentileKind::Inc)
    }
    18 => {
      let mut values = values;
      percentile_sorted(&mut values, k?, PercentileKind::Exc)
    }
    19 => {
      let mut values = values;
      percentile_sorted(&mut values, k?.floor() / 4.0, PercentileKind::Exc)
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

fn aggregate_count_numbers<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  args: &[FormulaValue<'doc>],
  options: AggregateOptions,
) -> usize {
  let mut count = 0usize;
  for arg in args {
    count += aggregate_count_numbers_value(evaluator, arg, options);
  }
  count
}

fn aggregate_count_numbers_value<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  value: &FormulaValue<'doc>,
  options: AggregateOptions,
) -> usize {
  match value {
    FormulaValue::Reference(reference) => {
      if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
        return 0;
      }
      let sheet = evaluator.range_sheet(reference);
      let mut count = 0usize;
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
          if matches!(
            evaluator.book.cell_value(sheet, address),
            FormulaValue::Number(_)
          ) {
            count += 1;
          }
        }
      }
      count
    }
    FormulaValue::Matrix(rows) => rows
      .iter()
      .flatten()
      .filter(|value| matches!(value, FormulaValue::Number(_)))
      .count(),
    FormulaValue::Number(_) => 1,
    _ => 0,
  }
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

fn matrix_can_broadcast(
  rows: usize,
  columns: usize,
  target_rows: usize,
  target_columns: usize,
) -> bool {
  (rows == target_rows || rows == 1) && (columns == target_columns || columns == 1)
}

fn matrix_binary_extent(left: usize, right: usize) -> usize {
  // Source: LibreOffice sc/source/core/tool/interpr5.cxx lcl_GetMinExtent.
  if left == 1 {
    right
  } else if right == 1 {
    left
  } else {
    left.min(right)
  }
}

fn formula_error_matches(value: &FormulaValue<'_>, na_only: bool) -> bool {
  matches!(
    (value, na_only),
    (FormulaValue::Error(FormulaErrorValue::NA), true) | (FormulaValue::Error(_), false)
  )
}

fn matrix_dimensions<T>(matrix: &[Vec<T>]) -> (usize, usize) {
  (matrix.len(), matrix.first().map_or(0, Vec::len))
}

fn ranges_intersect(left: &CellRange, right: &CellRange) -> bool {
  left.start.column <= right.end.column
    && right.start.column <= left.end.column
    && left.start.row <= right.end.row
    && right.start.row <= left.end.row
}

fn pivot_source_headers<'doc>(
  book: &FormulaEvaluationBook<'doc>,
  sheet: SheetId,
  pivot: &FormulaPivotTable<'doc>,
) -> Option<Vec<String>> {
  let source = &pivot.source.range;
  let mut fields = Vec::new();
  for column in source.start.column..=source.end.column {
    let value = book.cell_value(
      sheet,
      CellAddress {
        column,
        row: source.start.row,
      },
    );
    let FormulaValue::String(name) = value else {
      return None;
    };
    fields.push(name.into_owned());
  }
  Some(fields)
}

fn pivot_data_field<'doc>(
  pivot: &'doc FormulaPivotTable<'doc>,
  name: Option<&str>,
) -> Option<&'doc FormulaPivotField<'doc>> {
  let mut data_fields = pivot
    .fields
    .iter()
    .filter(|field| field.orientation == FormulaPivotFieldOrientation::Data);
  if let Some(name) = name {
    data_fields.find(|field| pivot_data_field_name_eq(&field.name, name))
  } else {
    data_fields.next()
  }
}

fn pivot_data_field_name_eq(field: &str, requested: &str) -> bool {
  pivot_name_eq(field, requested)
    || requested
      .strip_prefix("Sum - ")
      .is_some_and(|name| pivot_name_eq(field, name))
    || requested
      .strip_prefix("Count - ")
      .is_some_and(|name| pivot_name_eq(field, name))
}

fn pivot_name_eq(left: &str, right: &str) -> bool {
  left.eq_ignore_ascii_case(right)
}

fn pivot_value_eq(left: &str, right: &str) -> bool {
  left.eq_ignore_ascii_case(right)
}

fn pivot_row_filter_is_ambiguous<'doc>(
  book: &FormulaEvaluationBook<'doc>,
  sheet: SheetId,
  pivot: &FormulaPivotTable<'doc>,
  request: &PivotDataRequest<'doc>,
) -> bool {
  let row_fields = pivot
    .fields
    .iter()
    .filter(|field| field.orientation == FormulaPivotFieldOrientation::Row)
    .collect::<Vec<_>>();
  if row_fields.len() <= 1 {
    return false;
  }
  let row_filters = row_fields
    .iter()
    .filter_map(|field| {
      request
        .filters
        .iter()
        .find(|filter| pivot_name_eq(&field.name, &filter.field_name))
        .map(|filter| (field.name.as_ref(), filter.match_value.as_ref()))
    })
    .collect::<Vec<_>>();
  if row_filters.is_empty() || row_filters.len() == row_fields.len() {
    return false;
  }

  let target = &pivot.target.range;
  let mut inherited = vec![String::new(); row_fields.len()];
  let mut matches = 0usize;
  for row in target.start.row.saturating_add(1)..=target.end.row {
    for (index, inherited_value) in inherited.iter_mut().enumerate() {
      let value = book.cell_value(
        sheet,
        CellAddress {
          column: target.start.column + index as u32,
          row,
        },
      );
      if !matches!(value, FormulaValue::Blank) {
        *inherited_value = pivot_output_cell_text(&value);
      }
    }
    if row_filters.iter().all(|(field_name, expected)| {
      row_fields
        .iter()
        .position(|field| pivot_name_eq(&field.name, field_name))
        .is_some_and(|index| pivot_value_eq(&inherited[index], expected))
    }) {
      matches += 1;
      if matches > 1 {
        return true;
      }
    }
  }
  matches != 1
}

fn pivot_output_cell_text(value: &FormulaValue<'_>) -> String {
  match value {
    FormulaValue::String(value) => value.to_string(),
    FormulaValue::Blank => String::new(),
    _ => display_text_from_value(value),
  }
}

fn parse_getpivotdata_filter_text<'doc>(
  text: &str,
) -> (Option<Cow<'doc, str>>, Vec<PivotFieldFilter<'doc>>) {
  let mut filters = Vec::new();
  let mut index = 0usize;
  while let Some(open_rel) = text[index..].find('[') {
    let open = index + open_rel;
    let field = text[index..open].trim();
    let Some(close_rel) = text[open + 1..].find(']') else {
      break;
    };
    let close = open + 1 + close_rel;
    let mut item = text[open + 1..close].trim();
    if item.len() >= 2 && item.starts_with('\'') && item.ends_with('\'') {
      item = &item[1..item.len() - 1];
    }
    if !field.is_empty() {
      filters.push(PivotFieldFilter {
        field_name: Cow::Owned(field.to_string()),
        match_value: Cow::Owned(item.to_string()),
      });
    }
    index = close + 1;
  }
  (None, filters)
}

fn matrix_stat_number(value: &FormulaValue<'_>) -> Option<f64> {
  match value {
    FormulaValue::Number(value) => Some(*value),
    FormulaValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
    _ => None,
  }
}

fn formula_cell_numeric_value(value: &FormulaValue<'_>) -> Option<f64> {
  match value {
    FormulaValue::Number(value) => Some(*value),
    FormulaValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
    _ => None,
  }
}

fn sumproduct_merge_scalar(
  current: SumProductScalar,
  value: &FormulaValue<'_>,
) -> SumProductScalar {
  match value {
    FormulaValue::String(_) => SumProductScalar::NaN,
    FormulaValue::Blank => match current {
      SumProductScalar::Number(value) => SumProductScalar::Number(value * 0.0),
      value => value,
    },
    FormulaValue::Error(error) => match current {
      SumProductScalar::NaN => SumProductScalar::NaN,
      _ => SumProductScalar::Error(*error),
    },
    FormulaValue::Number(value) => match current {
      SumProductScalar::Number(current) => SumProductScalar::Number(current * value),
      value => value,
    },
    FormulaValue::Boolean(value) => match current {
      SumProductScalar::Number(current) => {
        SumProductScalar::Number(current * if *value { 1.0 } else { 0.0 })
      }
      value => value,
    },
    FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_) => current,
  }
}

fn value_number_for_array(value: &FormulaValue<'_>) -> Option<f64> {
  match value {
    FormulaValue::Number(value) => Some(*value),
    FormulaValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
    _ => None,
  }
}

fn number_only(value: &FormulaValue<'_>) -> Option<f64> {
  match value {
    FormulaValue::Number(value) => Some(*value),
    _ => None,
  }
}

fn covariance_pairs(
  left: &[Vec<FormulaValue<'_>>],
  right: &[Vec<FormulaValue<'_>>],
) -> Option<Vec<(f64, f64)>> {
  let left_rows = left.len();
  let left_columns = left.first().map_or(0, Vec::len);
  let right_rows = right.len();
  let right_columns = right.first().map_or(0, Vec::len);
  if left_rows != right_rows || left_columns != right_columns {
    return None;
  }
  let mut pairs = Vec::new();
  for row in 0..left_rows {
    if left[row].len() != left_columns || right[row].len() != right_columns {
      return None;
    }
    for column in 0..left_columns {
      let Some(left_value) = value_number_for_array(&left[row][column]) else {
        continue;
      };
      let Some(right_value) = value_number_for_array(&right[row][column]) else {
        continue;
      };
      pairs.push((left_value, right_value));
    }
  }
  Some(pairs)
}

fn arithmetic_matrix_number(value: &FormulaValue<'_>) -> Option<f64> {
  match value {
    FormulaValue::Number(value) => Some(*value),
    FormulaValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
    FormulaValue::Blank => Some(0.0),
    _ => None,
  }
}

fn arithmetic_operator_matrix_number(value: &FormulaValue<'_>) -> Option<f64> {
  match value {
    FormulaValue::Number(value) => Some(*value),
    FormulaValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
    FormulaValue::Blank => Some(0.0),
    _ => None,
  }
}

fn matrix_numbers<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  matrix: &[Vec<FormulaValue<'doc>>],
) -> Option<Vec<f64>> {
  matrix
    .iter()
    .flatten()
    .map(|value| evaluator.number(value))
    .collect()
}

fn matrix_number_at<'doc>(
  matrix: &[Vec<FormulaValue<'doc>>],
  row: usize,
  column: usize,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<f64> {
  matrix
    .get(row)
    .and_then(|values| values.get(column))
    .and_then(|value| evaluator.number(value))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MatrixShape {
  rows: usize,
  columns: usize,
}

impl MatrixShape {
  fn from_matrix<T>(matrix: &[Vec<T>]) -> Option<Self> {
    let rows = matrix.len();
    let columns = matrix.first().map_or(0, Vec::len);
    if rows == 0 || columns == 0 || matrix.iter().any(|row| row.len() != columns) {
      None
    } else {
      Some(Self { rows, columns })
    }
  }

  fn len(self) -> usize {
    self.rows * self.columns
  }

  fn materialize<'doc>(self, values: Vec<FormulaValue<'doc>>) -> Vec<Vec<FormulaValue<'doc>>> {
    let mut rows = Vec::with_capacity(self.rows);
    let mut iter = values.into_iter();
    for _ in 0..self.rows {
      rows.push(iter.by_ref().take(self.columns).collect());
    }
    rows
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RegressionCase {
  Simple,
  ColumnY,
  RowY,
}

#[derive(Clone, Debug)]
pub(crate) struct RegressionData {
  case: RegressionCase,
  x_shape: MatrixShape,
  y: Vec<f64>,
  design: Vec<Vec<f64>>,
}

#[derive(Clone, Debug)]
struct RegressionPrediction {
  shape: MatrixShape,
  design: Vec<Vec<f64>>,
}

impl RegressionData {
  fn k(&self) -> usize {
    self.design.first().map_or(0, Vec::len)
  }

  fn n(&self) -> usize {
    self.y.len()
  }

  fn default_prediction_matrix(&self) -> RegressionPrediction {
    let shape = match self.case {
      RegressionCase::Simple => self.x_shape,
      RegressionCase::ColumnY => MatrixShape {
        rows: self.design.len(),
        columns: 1,
      },
      RegressionCase::RowY => MatrixShape {
        rows: 1,
        columns: self.design.len(),
      },
    };
    RegressionPrediction {
      shape,
      design: self.design.clone(),
    }
  }

  fn prediction_matrix<'doc>(
    &self,
    evaluator: &FormulaEvaluator<'_, 'doc>,
    value: &FormulaValue<'doc>,
  ) -> std::result::Result<RegressionPrediction, FormulaErrorValue> {
    let matrix = evaluator.matrix_values(value);
    let shape = MatrixShape::from_matrix(&matrix).ok_or(FormulaErrorValue::IllegalArgument)?;
    let numbers = matrix_numbers(evaluator, &matrix).ok_or(FormulaErrorValue::IllegalArgument)?;
    if numbers.len() != shape.len() {
      return Err(FormulaErrorValue::IllegalArgument);
    }
    match self.case {
      RegressionCase::Simple => Ok(RegressionPrediction {
        shape,
        design: numbers.into_iter().map(|value| vec![value]).collect(),
      }),
      RegressionCase::ColumnY => {
        if shape.columns != self.k() {
          return Err(FormulaErrorValue::IllegalArgument);
        }
        let mut design = Vec::with_capacity(shape.rows);
        for row in 0..shape.rows {
          let mut values = Vec::with_capacity(shape.columns);
          for column in 0..shape.columns {
            values.push(
              matrix_number_at(&matrix, row, column, evaluator)
                .ok_or(FormulaErrorValue::IllegalArgument)?,
            );
          }
          design.push(values);
        }
        Ok(RegressionPrediction {
          shape: MatrixShape {
            rows: shape.rows,
            columns: 1,
          },
          design,
        })
      }
      RegressionCase::RowY => {
        if shape.rows != self.k() {
          return Err(FormulaErrorValue::IllegalArgument);
        }
        let mut design = Vec::with_capacity(shape.columns);
        for column in 0..shape.columns {
          let mut values = Vec::with_capacity(shape.rows);
          for row in 0..shape.rows {
            values.push(
              matrix_number_at(&matrix, row, column, evaluator)
                .ok_or(FormulaErrorValue::IllegalArgument)?,
            );
          }
          design.push(values);
        }
        Ok(RegressionPrediction {
          shape: MatrixShape {
            rows: 1,
            columns: shape.columns,
          },
          design,
        })
      }
    }
  }
}

fn regression_coefficients<'doc>(
  data: &RegressionData,
  constant: bool,
  stats: bool,
  log_regression: bool,
) -> Option<Vec<Vec<FormulaValue<'doc>>>> {
  if data.case == RegressionCase::Simple {
    return simple_regression_coefficients(data, constant, stats, log_regression);
  }
  let mut state = calc_regression_state(&data.y, &data.design, constant)?;
  let k = data.k();
  let rows = if stats { 5 } else { 1 };
  let mut result = vec![vec![FormulaValue::Error(FormulaErrorValue::NA); k + 1]; rows];
  result[0][k] = FormulaValue::Number(if log_regression {
    state.model.intercept.exp()
  } else {
    state.model.intercept
  });
  for index in 0..k {
    result[0][k - 1 - index] = FormulaValue::Number(if log_regression {
      state.model.slopes[index].exp()
    } else {
      state.model.slopes[index]
    });
  }
  if !stats {
    return Some(result);
  }
  regression_fill_stats(data, constant, &mut state, &mut result);
  Some(result)
}

fn simple_regression_coefficients<'doc>(
  data: &RegressionData,
  constant: bool,
  stats: bool,
  log_regression: bool,
) -> Option<Vec<Vec<FormulaValue<'doc>>>> {
  // Source: LibreOffice sc/source/core/tool/interpr5.cxx
  // ScInterpreter::CalculateRGPRKP nCase==1.
  let n = data.n();
  if (constant && n < 2) || n < 1 {
    return None;
  }
  let mut x = data
    .design
    .iter()
    .map(|row| row.first().copied())
    .collect::<Option<Vec<_>>>()?;
  let mut y = data.y.clone();
  let mut mean_x = 0.0;
  let mut mean_y = 0.0;
  if constant {
    mean_y = kahan_sum(y.iter().copied()) / n as f64;
    for value in &mut y {
      *value = approx_sub(*value, mean_y);
    }
    mean_x = kahan_sum(x.iter().copied()) / n as f64;
    for value in &mut x {
      *value = approx_sub(*value, mean_x);
    }
  }
  let sum_xy = kahan_sum(x.iter().zip(&y).map(|(x, y)| x * y));
  let sum_x2 = kahan_sum(x.iter().map(|value| value * value));
  if sum_x2 == 0.0 {
    return None;
  }
  let slope = sum_xy / sum_x2;
  let intercept = if constant {
    mean_y - slope * mean_x
  } else {
    0.0
  };
  let mut result = if stats {
    vec![vec![FormulaValue::Error(FormulaErrorValue::NA); 2]; 5]
  } else {
    vec![vec![FormulaValue::Error(FormulaErrorValue::NA); 2]; 1]
  };
  result[0][0] = FormulaValue::Number(if log_regression { slope.exp() } else { slope });
  result[0][1] = FormulaValue::Number(if log_regression {
    intercept.exp()
  } else {
    intercept
  });
  if !stats {
    return Some(result);
  }

  let ss_reg = slope * slope * sum_x2;
  let degrees_freedom = (if constant { n - 2 } else { n - 1 }) as f64;
  let ss_resid = kahan_sum(x.iter().zip(&y).map(|(x, y)| {
    let temp = y - slope * x;
    temp * temp
  }));
  result[4][0] = FormulaValue::Number(ss_reg);
  result[3][1] = FormulaValue::Number(degrees_freedom);
  result[4][1] = FormulaValue::Number(ss_resid);
  if degrees_freedom == 0.0 || ss_resid == 0.0 || ss_reg == 0.0 {
    result[4][1] = FormulaValue::Number(0.0);
    result[3][0] = FormulaValue::Error(FormulaErrorValue::NA);
    result[2][1] = FormulaValue::Number(0.0);
    result[1][0] = FormulaValue::Number(0.0);
    result[1][1] = if constant {
      FormulaValue::Number(0.0)
    } else {
      FormulaValue::Error(FormulaErrorValue::NA)
    };
    result[2][0] = FormulaValue::Number(1.0);
    return Some(result);
  }

  result[3][0] = FormulaValue::Number(ss_reg / (ss_resid / degrees_freedom));
  let rmse = (ss_resid / degrees_freedom).sqrt();
  result[2][1] = FormulaValue::Number(rmse);
  result[1][0] = FormulaValue::Number(rmse / sum_x2.sqrt());
  result[1][1] = if constant {
    FormulaValue::Number(rmse * (mean_x * mean_x / sum_x2 + 1.0 / n as f64).sqrt())
  } else {
    FormulaValue::Error(FormulaErrorValue::NA)
  };
  result[2][0] = FormulaValue::Number(ss_reg / (ss_reg + ss_resid));
  Some(result)
}

fn regression_model(data: &RegressionData, constant: bool) -> Option<RegressionModel> {
  if data.case == RegressionCase::Simple {
    return calc_regression_model(&data.y, &data.design, constant);
  }
  calc_regression_state(&data.y, &data.design, constant).map(|state| state.model)
}

fn regression_scalar_state<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  args: &[FormulaAst<'doc>],
) -> Option<RegressionScalarState> {
  if args.len() != 2 {
    return None;
  }
  regression_scalar_state_for_values(
    evaluator,
    &evaluator.evaluate(args.first()?)?,
    &evaluator.evaluate(args.get(1)?)?,
  )
}

fn regression_scalar_state_for_values<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  y_value: &FormulaValue<'doc>,
  x_value: &FormulaValue<'doc>,
) -> Option<RegressionScalarState> {
  let y_values = evaluator.value_numbers(y_value);
  let x_values = evaluator.value_numbers(x_value);
  Some(regression_scalar_state_from_slices(&y_values, &x_values))
}

fn regression_fill_stats<'doc>(
  data: &RegressionData,
  constant: bool,
  state: &mut RegressionState,
  result: &mut [Vec<FormulaValue<'doc>>],
) {
  let k = data.k();
  let n = data.n();
  let mut z = vec![0.0; n];
  for (row, value) in z.iter_mut().enumerate().take(k) {
    *value = state.r_diagonal[row] * state.model.slopes[row];
    for column in row + 1..k {
      *value += state.centered_x[row][column] * state.model.slopes[column];
    }
  }
  for column in (0..k).rev() {
    apply_householder(&state.centered_x, column, &mut z, n);
  }
  let ss_reg = z.iter().map(|value| value * value).sum::<f64>();
  let mut residual = state.centered_y.clone();
  for (value, fitted) in residual.iter_mut().zip(&z) {
    *value -= fitted;
  }
  let ss_resid = residual.iter().map(|value| value * value).sum::<f64>();
  result[4][0] = FormulaValue::Number(ss_reg);
  result[4][1] = FormulaValue::Number(ss_resid);
  let degrees_freedom = if constant {
    (n - k - 1) as f64
  } else {
    (n - k) as f64
  };
  result[3][1] = FormulaValue::Number(degrees_freedom);
  if degrees_freedom == 0.0 || ss_resid == 0.0 || ss_reg == 0.0 {
    result[4][1] = FormulaValue::Number(0.0);
    result[3][0] = FormulaValue::Error(FormulaErrorValue::NA);
    result[2][1] = FormulaValue::Number(0.0);
    for index in 0..k {
      result[1][k - 1 - index] = FormulaValue::Number(0.0);
    }
    result[1][k] = if constant {
      FormulaValue::Number(0.0)
    } else {
      FormulaValue::Error(FormulaErrorValue::NA)
    };
    result[2][0] = FormulaValue::Number(1.0);
    return;
  }

  result[3][0] = FormulaValue::Number((ss_reg / k as f64) / (ss_resid / degrees_freedom));
  let rmse = (ss_resid / degrees_freedom).sqrt();
  result[2][1] = FormulaValue::Number(rmse);
  let mut sigma_intercept = 0.0;
  for column in 0..k {
    let mut unit = vec![0.0; k];
    unit[column] = 1.0;
    solve_lower(&state.centered_x, &state.r_diagonal, &mut unit, k);
    solve_upper(&state.centered_x, &state.r_diagonal, &mut unit, k);
    result[1][k - 1 - column] = FormulaValue::Number(rmse * unit[column].sqrt());
    if constant {
      sigma_intercept += state.means[column]
        * state
          .means
          .iter()
          .zip(&unit)
          .map(|(m, u)| m * u)
          .sum::<f64>();
    }
  }
  result[1][k] = if constant {
    FormulaValue::Number(rmse * (sigma_intercept + 1.0 / n as f64).sqrt())
  } else {
    FormulaValue::Error(FormulaErrorValue::NA)
  };
  result[2][0] = FormulaValue::Number(ss_reg / (ss_reg + ss_resid));
}

fn lookup_vector<'doc>(
  matrix: &[Vec<FormulaValue<'doc>>],
) -> Option<(Vec<FormulaValue<'doc>>, bool)> {
  let rows = matrix.len();
  let columns = matrix.first().map_or(0, Vec::len);
  if rows == 0 || columns == 0 {
    return None;
  }
  let vertical = rows >= columns;
  lookup_vector_with_orientation(matrix, vertical).map(|values| (values, vertical))
}

fn lookup_vector_with_orientation<'doc>(
  matrix: &[Vec<FormulaValue<'doc>>],
  vertical: bool,
) -> Option<Vec<FormulaValue<'doc>>> {
  if matrix.is_empty() || matrix.first().is_none_or(Vec::is_empty) {
    return None;
  }
  let values = if vertical {
    matrix
      .iter()
      .filter_map(|row| row.first().cloned())
      .collect::<Vec<_>>()
  } else {
    matrix.first()?.clone()
  };
  Some(values)
}

fn lookup_search_vector_omitting_errors<'doc>(
  vector: &[FormulaValue<'doc>],
) -> (Option<Vec<FormulaValue<'doc>>>, Option<Vec<usize>>) {
  if !vector
    .iter()
    .all(|value| matches!(value, FormulaValue::Number(_) | FormulaValue::Error(_)))
  {
    return (None, None);
  }
  let mut values = Vec::new();
  let mut indices = Vec::new();
  for (index, value) in vector.iter().enumerate() {
    if matches!(value, FormulaValue::Number(_)) {
      values.push(value.clone());
      indices.push(index);
    }
  }
  if values.len() == vector.len() {
    (None, None)
  } else {
    (Some(values), Some(indices))
  }
}

fn vhlookup_matrix_index<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  lookup: &FormulaValue<'doc>,
  matrix: &[Vec<FormulaValue<'doc>>],
  horizontal: bool,
  sorted: bool,
) -> Option<usize> {
  let count = if horizontal {
    matrix.first().map_or(0, Vec::len)
  } else {
    matrix.len()
  };
  if count == 0 {
    return None;
  }
  let lookup_is_text = matches!(lookup, FormulaValue::String(_) | FormulaValue::Blank);
  let mut index = None;
  if lookup_is_text {
    let lookup_text = evaluator.text(lookup);
    if sorted {
      for candidate_index in 0..count {
        let candidate = vhlookup_search_cell(matrix, horizontal, candidate_index);
        if candidate.is_none_or(is_lookup_string_or_empty) {
          let candidate_text = candidate
            .map(|value| evaluator.text(value))
            .unwrap_or_default();
          let ordering = compare_text(evaluator, &candidate_text, &lookup_text, false);
          if ordering <= 0 {
            index = Some(candidate_index);
          } else if candidate_index > 0 {
            break;
          }
        } else {
          index = Some(candidate_index);
        }
      }
    } else {
      for candidate_index in 0..count {
        let candidate = vhlookup_search_cell(matrix, horizontal, candidate_index);
        if candidate.is_none_or(is_lookup_string_or_empty)
          && compare_text(
            evaluator,
            &candidate
              .map(|value| evaluator.text(value))
              .unwrap_or_default(),
            &lookup_text,
            false,
          ) == 0
        {
          index = Some(candidate_index);
          break;
        }
      }
    }
    if index.is_some_and(|index| {
      matches!(
        vhlookup_search_cell(matrix, horizontal, index),
        Some(FormulaValue::Number(_))
      )
    }) {
      return None;
    }
    return index;
  }

  let lookup_number = formula_cell_numeric_value(lookup)?;
  if sorted {
    for candidate_index in 0..count {
      let Some(candidate) = vhlookup_search_cell(matrix, horizontal, candidate_index) else {
        continue;
      };
      if is_lookup_string_or_empty(candidate) {
        continue;
      }
      let Some(candidate_number) = formula_cell_numeric_value(candidate) else {
        break;
      };
      if candidate_number <= lookup_number {
        index = Some(candidate_index);
      } else {
        break;
      }
    }
  } else {
    for candidate_index in 0..count {
      let Some(candidate) = vhlookup_search_cell(matrix, horizontal, candidate_index) else {
        continue;
      };
      if is_lookup_string_or_empty(candidate) {
        continue;
      }
      if formula_cell_numeric_value(candidate)
        .is_some_and(|candidate_number| compare_numbers(candidate_number, lookup_number) == 0)
      {
        index = Some(candidate_index);
        break;
      }
    }
  }
  index
}

fn vhlookup_search_cell<'doc>(
  matrix: &'doc [Vec<FormulaValue<'doc>>],
  horizontal: bool,
  index: usize,
) -> Option<&'doc FormulaValue<'doc>> {
  if horizontal {
    matrix.first().and_then(|row| row.get(index))
  } else {
    matrix.get(index).and_then(|row| row.first())
  }
}

fn is_lookup_string_or_empty(value: &FormulaValue<'_>) -> bool {
  matches!(value, FormulaValue::String(_) | FormulaValue::Blank)
}

fn choose_row_column_index(index: i64, len: usize) -> Option<usize> {
  if index == 0 {
    return None;
  }
  let len = i64::try_from(len).ok()?;
  let normalized = if index < 0 { len + index + 1 } else { index };
  (1..=len)
    .contains(&normalized)
    .then_some((normalized - 1) as usize)
}

fn is_blank_for_countblank(value: &FormulaValue<'_>) -> bool {
  matches!(value, FormulaValue::Blank)
    || matches!(value, FormulaValue::String(text) if text.is_empty())
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
    FormulaValue::Error(_) if options.ignore_errors => {}
    _ => *count += 1,
  }
  Ok(())
}

fn permutationa_value<'doc>(count: f64, chosen: f64) -> FormulaValue<'doc> {
  let count = approx_floor(count);
  let chosen = approx_floor(chosen);
  match permutation_with_repetition_count(count, chosen) {
    Some(value) => FormulaValue::Number(value),
    None => FormulaValue::Error(FormulaErrorValue::Num),
  }
}

fn holiday_serials(
  value: Option<&FormulaValue<'_>>,
  evaluator: &FormulaEvaluator<'_, '_>,
) -> Vec<i64> {
  let Some(value) = value else {
    return Vec::new();
  };
  let mut holidays = Vec::new();
  for value in evaluator.matrix_values(value).into_iter().flatten() {
    match value {
      FormulaValue::Error(_) | FormulaValue::Blank => {}
      value => {
        if let Some(serial) = evaluator.date_number_from_value(&value) {
          holidays.push(serial.floor() as i64);
        }
      }
    }
  }
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
    FormulaValue::Blank if workday_function => return None,
    FormulaValue::Blank => String::new(),
    FormulaValue::Matrix(rows) => {
      return weekend_mask(
        rows.first().and_then(|row| row.first()),
        workday_function,
        evaluator,
      );
    }
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
  if workday_function && text == "1111111" {
    return None;
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

fn old_networkdays_weekend_mask(
  value: Option<&FormulaValue<'_>>,
  evaluator: &FormulaEvaluator<'_, '_>,
) -> Option<[bool; 7]> {
  // Source: LibreOffice sc/source/core/tool/interpr2.cxx GetWeekendAndHolidayMasks.
  let value = value?;
  let values = evaluator
    .matrix_values(value)
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
  if values.len() != 7 {
    return None;
  }
  let mut source = [false; 7];
  for (index, value) in values.iter().enumerate() {
    source[index] = evaluator.number(value)? != 0.0;
  }
  let mut mask = [false; 7];
  for index in 0..7 {
    mask[index] = source[if index == 6 { 0 } else { index + 1 }];
  }
  Some(mask)
}

fn add_group_separators(value: &str) -> String {
  let (sign, body) = value
    .strip_prefix('-')
    .map_or(("", value), |body| ("-", body));
  let (integer, fraction) = body.split_once('.').unwrap_or((body, ""));
  let mut grouped = String::new();
  for (index, ch) in integer.chars().rev().enumerate() {
    if index != 0 && index.is_multiple_of(3) {
      grouped.push(',');
    }
    grouped.push(ch);
  }
  let integer = grouped.chars().rev().collect::<String>();
  if fraction.is_empty() {
    format!("{sign}{integer}")
  } else {
    format!("{sign}{integer}.{fraction}")
  }
}

fn format_complex_result(value: FormulaComplex) -> String {
  format_formula_complex_number(value, format_complex_component)
}

fn format_complex_component(value: f64, leading_sign: bool) -> String {
  if !value.is_finite() {
    return error_text_value(FormulaErrorValue::Value).to_string();
  }
  let mut output = format_general_significant(value, 15);
  if leading_sign && value > 0.0 && !output.starts_with('+') {
    output.insert(0, '+');
  }
  output
}

fn format_general_significant(value: f64, precision: i32) -> String {
  if value == 0.0 {
    return "0".to_string();
  }
  let exponent = value.abs().log10().floor() as i32;
  if exponent >= -4 && exponent < precision {
    let decimals = (precision - exponent - 1).max(0) as usize;
    return trim_float_text(&format!("{value:.decimals$}"));
  }
  let decimals = (precision - 1).max(0) as usize;
  let output = format!("{value:.decimals$e}");
  let Some((mantissa, exponent)) = output.split_once('e') else {
    return trim_float_text(&output);
  };
  let mantissa = trim_float_text(mantissa);
  let exponent_value = exponent.parse::<i32>().unwrap_or(0);
  format!("{mantissa}e{exponent_value:+}")
}

fn trim_float_text(value: &str) -> String {
  if let Some((head, tail)) = value.split_once('.') {
    let tail = tail.trim_end_matches('0');
    if tail.is_empty() {
      head.to_string()
    } else {
      format!("{head}.{tail}")
    }
  } else {
    value.to_string()
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

fn quote_sheet_name_for_reference(sheet: &str) -> String {
  if sheet
    .chars()
    .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
  {
    sheet.to_string()
  } else {
    format!("'{}'", sheet.replace('\'', "''"))
  }
}

fn parse_table_reference<'doc>(
  book: &FormulaEvaluationBook<'doc>,
  text: &str,
  current_address: Option<CellAddress>,
) -> Option<QualifiedRange<'doc>> {
  let selection = crate::parser::parse_table_reference_selection(text)?;
  let table = book
    .tables
    .get(&selection.table_name.to_ascii_uppercase())?;
  let mut range = table_reference_item_range(table, selection.items, current_address)?;
  if !selection.columns.is_empty() {
    let start = table_reference_column_offset(table, selection.columns[0].as_ref())?;
    let end = selection
      .columns
      .last()
      .and_then(|column| table_reference_column_offset(table, column.as_ref()))?;
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
  items: crate::parser::TableReferenceItems,
  current_address: Option<CellAddress>,
) -> Option<CellRange> {
  use crate::parser::TableReferenceItems;

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
  table
    .columns
    .iter()
    .position(|name| name.as_ref().eq_ignore_ascii_case(column))
    .map(|index| index as u32)
}

fn chisq_dist_value<'doc>(
  x: f64,
  df: f64,
  right_tail: bool,
  cumulative: Option<bool>,
) -> FormulaValue<'doc> {
  if df < 1.0 || x < 0.0 {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  if right_tail {
    return FormulaValue::Number(lo_chi_dist(x, df));
  }
  FormulaValue::Number(if cumulative.unwrap_or(true) {
    lo_chisq_dist_cdf(x, df)
  } else {
    lo_chisq_dist_pdf(x, df)
  })
}

fn chisq_inv_value<'doc>(p: f64, df: f64, right_tail: bool) -> FormulaValue<'doc> {
  if df < 1.0 || (right_tail && (p <= 0.0 || p > 1.0)) || (!right_tail && (p < 0.0 || p >= 1.0)) {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  let result = if right_tail {
    lo_iterate_inverse(|x| p - lo_chi_dist(x, df), df * 0.5, df)
  } else {
    lo_iterate_inverse(|x| p - lo_chisq_dist_cdf(x, df), df * 0.5, df)
  };
  match result {
    Ok(value) => FormulaValue::Number(value),
    Err(error) => FormulaValue::Error(special_error_value(error)),
  }
}

fn special_error_value(error: SpecialError) -> FormulaErrorValue {
  match error {
    SpecialError::IllegalArgument => FormulaErrorValue::IllegalArgument,
    SpecialError::Num => FormulaErrorValue::Num,
    SpecialError::Div0 => FormulaErrorValue::Div0,
  }
}

fn numeric_error_value(error: NumericError) -> FormulaErrorValue {
  match error {
    NumericError::IllegalArgument => FormulaErrorValue::IllegalArgument,
    NumericError::Div0 => FormulaErrorValue::Div0,
    NumericError::Value => FormulaErrorValue::Value,
  }
}

fn ets_error_value(error: EtsError) -> FormulaErrorValue {
  match error {
    EtsError::IllegalArgument => FormulaErrorValue::IllegalArgument,
    EtsError::Num => FormulaErrorValue::Num,
    EtsError::Value => FormulaErrorValue::Value,
    EtsError::Div0 => FormulaErrorValue::Div0,
  }
}

fn statistics_error_value(error: StatisticsError) -> FormulaErrorValue {
  match error {
    StatisticsError::Div0 => FormulaErrorValue::Div0,
    StatisticsError::IllegalArgument => FormulaErrorValue::IllegalArgument,
  }
}

fn propagate_binary_error(
  left: &FormulaValue<'_>,
  right: &FormulaValue<'_>,
) -> Option<FormulaErrorValue> {
  match (left, right) {
    (FormulaValue::Error(error), _) | (_, FormulaValue::Error(error)) => Some(*error),
    _ => None,
  }
}

fn first_error_in_values(values: &[&FormulaValue<'_>]) -> Option<FormulaErrorValue> {
  values.iter().find_map(|value| match value {
    FormulaValue::Error(error) => Some(*error),
    _ => None,
  })
}

fn first_error_in_value<'doc>(value: &FormulaValue<'doc>) -> Option<FormulaValue<'doc>> {
  match value {
    FormulaValue::Error(error) => Some(FormulaValue::Error(*error)),
    FormulaValue::Matrix(rows) => rows.iter().flatten().find_map(first_error_in_value),
    _ => None,
  }
}

fn format_dollar_value(value: f64, digits: usize) -> String {
  let sign = if value.is_sign_negative() { "-" } else { "" };
  let formatted = format!("{:.*}", digits, value.abs());
  let (integer, fraction) = formatted.split_once('.').unwrap_or((&formatted, ""));
  let mut grouped = String::new();
  for (index, ch) in integer.chars().rev().enumerate() {
    if index > 0 && index % 3 == 0 {
      grouped.push(',');
    }
    grouped.push(ch);
  }
  let integer = grouped.chars().rev().collect::<String>();
  if digits == 0 {
    format!("{sign}${integer}")
  } else {
    format!("{sign}${integer}.{fraction}")
  }
}

fn should_ignore_to_row_column_value(value: &FormulaValue<'_>, ignore: i32) -> bool {
  let ignore_blanks = matches!(ignore, 1 | 3);
  let ignore_errors = matches!(ignore, 2 | 3);
  (ignore_blanks && matches!(value, FormulaValue::Blank))
    || (ignore_errors && matches!(value, FormulaValue::Error(_)))
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
    FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => {
      String::new()
    }
  }
}

fn display_text_from_value_with_number_format(
  value: &FormulaValue<'_>,
  context: Option<&NumberFormatContext<'_>>,
) -> Option<String> {
  let FormulaValue::Number(number) = value else {
    return None;
  };
  let format = context?.format_code.as_deref()?;
  let format = select_number_format_section(format, *number);
  format_simple_number_pattern(*number, &format)
}

fn format_text(
  value: &FormulaValue<'_>,
  format: &FormulaValue<'_>,
  evaluator: &FormulaEvaluator<'_, '_>,
) -> String {
  let format = evaluator.text(format);
  let Some(number) = evaluator.number(value) else {
    if format.is_empty() {
      return evaluator.text(value);
    }
    return evaluator.text(value);
  };
  if format.is_empty() {
    return String::new();
  }
  let format = select_number_format_section(&format, number);
  if let Some(text) = format_date_pattern(number, &format, evaluator.book.date_system) {
    return text;
  }
  if let Some(text) = format_simple_number_pattern(number, &format) {
    return text;
  }
  evaluator.text(value)
}

fn format_date_pattern(number: f64, format: &str, date_system: DateSystem) -> Option<String> {
  if !format_contains_date_time_token(format) {
    return None;
  }
  let day = number.floor();
  let fraction = number - day;
  let seconds = (fraction * 86_400.0).round() as i64;
  let day_adjust = seconds.div_euclid(86_400);
  let second_of_day = seconds.rem_euclid(86_400);
  let serial = day as i32 + day_adjust as i32;
  let (year, month, day) = date_from_serial_with_system(serial, date_system)?;
  let hour = second_of_day / 3600;
  let minute = (second_of_day % 3600) / 60;
  let second = second_of_day % 60;
  let mut result = String::new();
  let mut chars = format.chars().peekable();
  let mut after_hour = false;
  while let Some(ch) = chars.next() {
    if ch == '"' {
      for literal in chars.by_ref() {
        if literal == '"' {
          break;
        }
        result.push(literal);
      }
      continue;
    }
    if !ch.is_ascii_alphabetic() {
      result.push(ch);
      continue;
    }
    let lower = ch.to_ascii_lowercase();
    let mut len = 1usize;
    while chars
      .peek()
      .is_some_and(|next| next.to_ascii_lowercase() == lower)
    {
      chars.next();
      len += 1;
    }
    match lower {
      'y' => {
        if len <= 2 {
          result.push_str(&format!("{:02}", year.rem_euclid(100)));
        } else {
          result.push_str(&format!("{year:04}"));
        }
      }
      'd' => {
        if len == 1 {
          result.push_str(&day.to_string());
        } else {
          result.push_str(&format!("{day:02}"));
        }
      }
      'h' => {
        after_hour = true;
        if len == 1 {
          result.push_str(&hour.to_string());
        } else {
          result.push_str(&format!("{hour:02}"));
        }
      }
      'm' => {
        let value = if after_hour { minute as u32 } else { month };
        if len == 1 {
          result.push_str(&value.to_string());
        } else {
          result.push_str(&format!("{value:02}"));
        }
      }
      's' => {
        if len == 1 {
          result.push_str(&second.to_string());
        } else {
          result.push_str(&format!("{second:02}"));
        }
      }
      _ => {
        for _ in 0..len {
          result.push(ch);
        }
      }
    }
  }
  Some(result)
}

fn format_simple_number_pattern(number: f64, format: &str) -> Option<String> {
  let numeric = strip_number_format_directives(format.trim());
  if numeric.eq_ignore_ascii_case("General") {
    return None;
  }
  if numeric.starts_with('"') && numeric.ends_with('"') {
    return Some(numeric.trim_matches('"').to_string());
  }
  if numeric.contains('?') && numeric.contains('/') {
    return format_fraction_pattern(number, &numeric);
  }
  if numeric.to_ascii_uppercase().contains('E') {
    return format_scientific_pattern(number, &numeric);
  }
  if numeric.contains('%') {
    let body = numeric.replace('%', "");
    return format_simple_number_pattern(number * 100.0, &body).map(|text| format!("{text}%"));
  }
  if can_format_as_digit_mask(&numeric) && !numeric.contains('.') {
    return format_digit_mask(number, &numeric);
  }
  let grouping = numeric.contains(',');
  let prefix: String = numeric
    .chars()
    .take_while(|ch| !matches!(ch, '#' | '0' | '?' | '.' | ','))
    .collect();
  let suffix: String = numeric
    .chars()
    .rev()
    .take_while(|ch| !matches!(ch, '#' | '0' | '?' | '.' | ','))
    .collect::<String>()
    .chars()
    .rev()
    .collect();
  let digit_part = numeric
    .trim_start_matches(|ch| !matches!(ch, '#' | '0' | '?' | '.' | ','))
    .trim_end_matches(|ch| !matches!(ch, '#' | '0' | '?' | '.' | ','));
  let mut seen_digit = false;
  let mut min_integer_digits = 0usize;
  let mut decimal_places = 0usize;
  let mut after_decimal = false;
  for ch in digit_part.chars() {
    match ch {
      '#' | '0' | '?' => {
        seen_digit = true;
        if after_decimal {
          decimal_places += 1;
        } else if ch == '0' {
          min_integer_digits += 1;
        }
      }
      '.' => {
        if after_decimal {
          return None;
        }
        after_decimal = true;
      }
      ',' | ' ' => {}
      _ => return None,
    }
  }
  if !seen_digit {
    return None;
  }
  if decimal_places == 0 {
    let rounded = round_half_away_from_zero(number, 0) as i64;
    let mut text = format_integer_with_min_digits(rounded, min_integer_digits);
    if grouping {
      text = add_grouping_separators(&text);
    }
    return Some(format!("{prefix}{text}{suffix}"));
  }
  let rounded = round_half_away_from_zero(number, decimal_places as i32);
  let mut text = format!("{rounded:.decimal_places$}");
  if min_integer_digits > 0 {
    text = pad_integer_part(text, min_integer_digits);
  }
  if digit_part
    .rsplit_once('.')
    .is_some_and(|(_, fraction)| fraction.chars().all(|ch| ch == '#'))
  {
    while text.ends_with('0') {
      text.pop();
    }
    if text.ends_with('.') {
      text.pop();
    }
  }
  if grouping {
    text = add_grouping_to_decimal(&text);
  }
  Some(format!("{prefix}{text}{suffix}"))
}

fn select_number_format_section(format: &str, number: f64) -> String {
  let sections = split_number_format_sections(format);
  for section in &sections {
    if let Some((op, threshold, body)) = parse_format_condition(section)
      && compare_format_condition(number, op, threshold)
    {
      return body.trim().to_string();
    }
  }
  let non_conditional: Vec<&str> = sections
    .iter()
    .map(String::as_str)
    .filter(|section| parse_format_condition(section).is_none())
    .collect();
  if non_conditional.is_empty() {
    return format.to_string();
  }
  if number < 0.0 && non_conditional.len() >= 2 {
    return non_conditional[1].trim().to_string();
  }
  if number == 0.0 && non_conditional.len() >= 3 {
    return non_conditional[2].trim().to_string();
  }
  non_conditional[0].trim().to_string()
}

fn split_number_format_sections(format: &str) -> Vec<String> {
  let mut sections = Vec::new();
  let mut section = String::new();
  let mut in_quotes = false;
  let mut bracket_depth = 0usize;
  for ch in format.chars() {
    match ch {
      '"' => {
        in_quotes = !in_quotes;
        section.push(ch);
      }
      '[' if !in_quotes => {
        bracket_depth += 1;
        section.push(ch);
      }
      ']' if !in_quotes => {
        bracket_depth = bracket_depth.saturating_sub(1);
        section.push(ch);
      }
      ';' if !in_quotes && bracket_depth == 0 => {
        sections.push(section.trim().to_string());
        section.clear();
      }
      _ => section.push(ch),
    }
  }
  sections.push(section.trim().to_string());
  sections
}

fn parse_format_condition(section: &str) -> Option<(&str, f64, &str)> {
  let rest = section.trim().strip_prefix('[')?;
  let (condition, body) = rest.split_once(']')?;
  for op in ["<=", ">=", "<>", "<", ">", "="] {
    if let Some(value) = condition.strip_prefix(op)
      && let Ok(threshold) = value.trim().parse::<f64>()
    {
      return Some((op, threshold, body));
    }
  }
  None
}

fn compare_format_condition(value: f64, op: &str, threshold: f64) -> bool {
  match op {
    "<=" => value <= threshold,
    ">=" => value >= threshold,
    "<>" => (value - threshold).abs() > f64::EPSILON,
    "<" => value < threshold,
    ">" => value > threshold,
    "=" => (value - threshold).abs() <= f64::EPSILON,
    _ => false,
  }
}

fn strip_number_format_directives(format: &str) -> String {
  let mut result = String::new();
  let mut chars = format.chars().peekable();
  while let Some(ch) = chars.next() {
    if ch == '[' {
      let directive: String = chars.by_ref().take_while(|next| *next != ']').collect();
      if directive
        .chars()
        .next()
        .is_some_and(|value| matches!(value, '<' | '>' | '='))
      {
        continue;
      }
      continue;
    }
    if ch == '"' {
      for literal in chars.by_ref() {
        if literal == '"' {
          break;
        }
        result.push(literal);
      }
      continue;
    }
    if ch == '\\' {
      if let Some(literal) = chars.next() {
        result.push(literal);
      }
      continue;
    }
    result.push(ch);
  }
  result.trim().to_string()
}

fn format_contains_date_time_token(format: &str) -> bool {
  let mut in_quotes = false;
  let mut in_brackets = false;
  for ch in format.chars() {
    match ch {
      '"' => in_quotes = !in_quotes,
      '[' if !in_quotes => in_brackets = true,
      ']' if !in_quotes => in_brackets = false,
      _ if !in_quotes
        && !in_brackets
        && matches!(ch.to_ascii_lowercase(), 'y' | 'm' | 'd' | 'h' | 's') =>
      {
        return true;
      }
      _ => {}
    }
  }
  false
}

fn format_scientific_pattern(number: f64, format: &str) -> Option<String> {
  let upper = format.to_ascii_uppercase();
  let (mantissa_pattern, exponent_pattern) = upper.split_once('E')?;
  let decimals = mantissa_pattern.split_once('.').map_or(0, |(_, fraction)| {
    fraction
      .chars()
      .filter(|ch| matches!(ch, '0' | '#'))
      .count()
  });
  let exponent_digits = exponent_pattern
    .chars()
    .filter(|ch| matches!(ch, '0' | '#'))
    .count()
    .max(1);
  if number == 0.0 {
    return Some(format!(
      "{:.*}E+{:0width$}",
      decimals,
      0.0,
      0,
      width = exponent_digits
    ));
  }
  let sign = if number.is_sign_negative() { "-" } else { "" };
  let absolute = number.abs();
  let exponent = absolute.log10().floor() as i32;
  let mantissa = absolute / 10_f64.powi(exponent);
  let mantissa = round_half_away_from_zero(mantissa, decimals as i32);
  let (mantissa, exponent) = if mantissa >= 10.0 {
    (mantissa / 10.0, exponent + 1)
  } else {
    (mantissa, exponent)
  };
  Some(format!(
    "{sign}{mantissa:.decimals$}E{}{exp:0width$}",
    if exponent < 0 { "-" } else { "+" },
    exp = exponent.abs(),
    width = exponent_digits
  ))
}

fn format_fraction_pattern(number: f64, format: &str) -> Option<String> {
  if !format.contains('/') {
    return None;
  }
  let sign = if number < 0.0 { "-" } else { "" };
  let absolute = number.abs();
  let whole = absolute.floor() as i64;
  let fraction = absolute - whole as f64;
  let (numerator, denominator) = best_fraction(fraction, 9)?;
  if numerator == 0 {
    return Some(format!("{sign}{whole}"));
  }
  if whole == 0 {
    Some(format!("{sign}{numerator}/{denominator}"))
  } else {
    Some(format!("{sign}{whole} {numerator}/{denominator}"))
  }
}

fn best_fraction(value: f64, max_denominator: i64) -> Option<(i64, i64)> {
  let mut best = None;
  let mut best_error = f64::INFINITY;
  for denominator in 1..=max_denominator {
    let numerator = round_half_away_from_zero(value * denominator as f64, 0) as i64;
    let error = (value - numerator as f64 / denominator as f64).abs();
    if error < best_error {
      best_error = error;
      best = Some((numerator, denominator));
    }
  }
  best
}

fn can_format_as_digit_mask(format: &str) -> bool {
  let mut seen_digit = false;
  for ch in format.chars() {
    match ch {
      '#' | '0' => seen_digit = true,
      '?' | '.' | ',' => return false,
      _ => {}
    }
  }
  seen_digit && format.chars().any(|ch| !matches!(ch, '#' | '0'))
}

fn format_digit_mask(number: f64, format: &str) -> Option<String> {
  let rounded = round_half_away_from_zero(number.abs(), 0) as i64;
  let mut digits: Vec<char> = rounded.to_string().chars().collect();
  let mut result = Vec::new();
  for ch in format.chars().rev() {
    match ch {
      '0' => result.push(digits.pop().unwrap_or('0')),
      '#' => {
        if let Some(digit) = digits.pop() {
          result.push(digit);
        }
      }
      _ => result.push(ch),
    }
  }
  while let Some(digit) = digits.pop() {
    result.push(digit);
  }
  let mut text: String = result.into_iter().rev().collect();
  if number < 0.0 {
    text.insert(0, '-');
  }
  Some(text)
}

fn add_grouping_to_decimal(text: &str) -> String {
  let Some((integer, fraction)) = text.split_once('.') else {
    return add_grouping_separators(text);
  };
  format!("{}.{}", add_grouping_separators(integer), fraction)
}

fn add_grouping_separators(text: &str) -> String {
  let (negative, body) = text
    .strip_prefix('-')
    .map(|body| (true, body))
    .unwrap_or((false, text));
  let mut result = String::new();
  for (index, ch) in body.chars().rev().enumerate() {
    if index > 0 && index % 3 == 0 {
      result.push(',');
    }
    result.push(ch);
  }
  let mut grouped: String = result.chars().rev().collect();
  if negative {
    grouped.insert(0, '-');
  }
  grouped
}

fn format_integer_with_min_digits(value: i64, min_digits: usize) -> String {
  if min_digits == 0 {
    return value.to_string();
  }
  if value < 0 {
    format!("-{:0width$}", value.unsigned_abs(), width = min_digits)
  } else {
    format!("{value:0width$}", width = min_digits)
  }
}

fn pad_integer_part(text: String, min_digits: usize) -> String {
  let (negative, body) = text
    .strip_prefix('-')
    .map(|body| (true, body))
    .unwrap_or((false, text.as_str()));
  let Some((integer, fraction)) = body.split_once('.') else {
    return format_integer_with_min_digits(text.parse::<i64>().unwrap_or_default(), min_digits);
  };
  if integer.len() >= min_digits {
    return text;
  }
  let mut result = String::new();
  if negative {
    result.push('-');
  }
  result.extend(std::iter::repeat_n('0', min_digits - integer.len()));
  result.push_str(integer);
  result.push('.');
  result.push_str(fraction);
  result
}

pub(crate) fn timevalue(text: &str) -> FormulaValue<'static> {
  let text = text.trim();
  let text = time_text_suffix(text);
  let lower = text.to_ascii_lowercase();
  let (body, meridiem) = if let Some(body) = lower.strip_suffix("am") {
    (body.trim(), Some(false))
  } else if let Some(body) = lower.strip_suffix("pm") {
    (body.trim(), Some(true))
  } else {
    (text, None)
  };
  let parts = body.split(':').collect::<Vec<_>>();
  if parts.len() < 2 && meridiem.is_none() {
    return FormulaValue::Error(FormulaErrorValue::Value);
  }
  let mut hour = match parts.first().and_then(|part| part.parse::<f64>().ok()) {
    Some(hour) => hour,
    None => return FormulaValue::Error(FormulaErrorValue::Value),
  };
  let minute = parts
    .get(1)
    .and_then(|part| part.parse::<f64>().ok())
    .unwrap_or(0.0);
  let second = parts
    .get(2)
    .and_then(|part| part.parse::<f64>().ok())
    .unwrap_or(0.0);
  if let Some(pm) = meridiem {
    if !(1.0..=12.0).contains(&hour) {
      return FormulaValue::Error(FormulaErrorValue::Value);
    }
    hour = if pm {
      if hour == 12.0 { 12.0 } else { hour + 12.0 }
    } else if hour == 12.0 {
      0.0
    } else {
      hour
    };
  }
  if hour < 0.0 || minute < 0.0 || second < 0.0 {
    return FormulaValue::Error(FormulaErrorValue::Value);
  }
  let seconds = (hour * 3600.0 + minute * 60.0 + second).rem_euclid(86_400.0);
  FormulaValue::Number(seconds / 86_400.0)
}

fn time_text_suffix(text: &str) -> &str {
  let trimmed = text.trim();
  if let Some(index) = trimmed.find('T') {
    let suffix = trimmed[index + 1..].trim();
    if suffix.contains(':') {
      return suffix;
    }
  }
  for (index, ch) in trimmed.char_indices().rev() {
    if ch.is_ascii_whitespace() {
      let suffix = trimmed[index..].trim();
      if suffix.contains(':') {
        return suffix;
      }
      break;
    }
  }
  trimmed
}

fn euro_convert(
  value: f64,
  from: &str,
  to: &str,
  full_precision: bool,
  precision: f64,
) -> Option<f64> {
  let (from_rate, _) = euro_currency_info(from)?;
  let (to_rate, to_decimals) = euro_currency_info(to)?;
  let mut result = if from.eq_ignore_ascii_case(to) {
    value
  } else if from.eq_ignore_ascii_case("EUR") {
    value * to_rate
  } else {
    let mut intermediate = value / from_rate;
    if precision != 0.0 {
      intermediate = round_to_decimal_places(intermediate, precision as i32);
    }
    intermediate * to_rate
  };
  if !full_precision && !from.eq_ignore_ascii_case(to) {
    result = round_to_decimal_places(result, to_decimals);
  }
  Some(result)
}

fn euro_currency_info(unit: &str) -> Option<(f64, i32)> {
  const CURRENCIES: &[(&str, f64, i32)] = &[
    ("EUR", 1.0, 2),
    ("ATS", 13.7603, 2),
    ("BEF", 40.3399, 0),
    ("DEM", 1.95583, 2),
    ("ESP", 166.386, 0),
    ("FIM", 5.94573, 2),
    ("FRF", 6.55957, 2),
    ("IEP", 0.787564, 2),
    ("ITL", 1936.27, 0),
    ("LUF", 40.3399, 0),
    ("NLG", 2.20371, 2),
    ("PTE", 200.482, 2),
    ("GRD", 340.750, 2),
    ("SIT", 239.640, 2),
    ("MTL", 0.429300, 2),
    ("CYP", 0.585274, 2),
    ("SKK", 30.1260, 2),
    ("EEK", 15.6466, 2),
    ("LVL", 0.702804, 2),
    ("LTL", 3.45280, 2),
    ("HRK", 7.53450, 2),
    ("BGN", 1.95583, 2),
  ];
  CURRENCIES
    .iter()
    .find(|(name, _, _)| name.eq_ignore_ascii_case(unit))
    .map(|(_, rate, decimals)| (*rate, *decimals))
}

fn expand_two_digit_year(year: i32) -> i32 {
  if year >= 30 { 1900 + year } else { 2000 + year }
}

#[derive(Clone, Copy, Debug)]
struct XorShift64 {
  state: u64,
}

impl XorShift64 {
  fn new(seed: u64) -> Self {
    Self {
      state: if seed == 0 {
        0x9e37_79b9_7f4a_7c15
      } else {
        seed
      },
    }
  }

  fn next_unit(&mut self) -> f64 {
    let mut value = self.state;
    value ^= value << 13;
    value ^= value >> 7;
    value ^= value << 17;
    self.state = value;
    (value as f64) / (u64::MAX as f64)
  }
}

fn time_seed() -> u64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .map(|duration| duration.as_nanos() as u64)
    .unwrap_or(0x9e37_79b9_7f4a_7c15)
}

fn split_text_multi(
  text: &str,
  delimiters: &[String],
  ignore_empty: bool,
  match_case_insensitive: bool,
) -> Vec<String> {
  if delimiters.is_empty() || text.is_empty() {
    return vec![text.to_string()];
  }
  let search_text = if match_case_insensitive {
    text.to_lowercase()
  } else {
    text.to_string()
  };
  let search_delimiters = delimiters
    .iter()
    .map(|delimiter| {
      if match_case_insensitive {
        delimiter.to_lowercase()
      } else {
        delimiter.clone()
      }
    })
    .collect::<Vec<_>>();
  let mut output = Vec::new();
  let mut start = 0usize;
  while start < text.len() {
    let mut index = text.len();
    let mut delimiter_len = 0usize;
    for delimiter in &search_delimiters {
      if delimiter.is_empty() {
        continue;
      }
      if let Some(position) = search_text[start..].find(delimiter) {
        let position = start + position;
        if position < index {
          index = position;
          delimiter_len = delimiter.len();
        }
      }
    }
    let segment = &text[start..index];
    if !ignore_empty || !segment.is_empty() {
      output.push(segment.to_string());
    }
    if delimiter_len == 0 {
      break;
    }
    start = index + delimiter_len;
  }
  output
}

pub(crate) fn datevalue(text: &str, date_system: DateSystem) -> FormulaValue<'static> {
  let text = text.trim();
  if text.is_empty() {
    return FormulaValue::Error(FormulaErrorValue::Value);
  }
  parse_date_input(text, date_system)
    .map(|value| FormulaValue::Number(value.floor()))
    .unwrap_or(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
}

fn parse_date_input(text: &str, date_system: DateSystem) -> Option<f64> {
  let date = date_input_prefix(text)?;
  parse_iso_date_input(date, date_system)
    .or_else(|| parse_numeric_date_input(date, date_system))
    .or_else(|| parse_month_date_input(date, date_system))
}

fn date_input_prefix(text: &str) -> Option<&str> {
  let trimmed = text.trim();
  if trimmed.is_empty() {
    return None;
  }
  let mut split = trimmed.len();
  for (index, ch) in trimmed.char_indices() {
    if ch == 'T' && trimmed[index + ch.len_utf8()..].contains(':') {
      split = index;
      break;
    }
    if ch.is_ascii_whitespace() {
      let rest = trimmed[index..].trim();
      if rest.is_empty() || rest.chars().any(|ch| ch == ':') {
        split = index;
        break;
      }
    }
  }
  Some(trimmed[..split].trim_end_matches(',').trim())
}

fn parse_iso_date_input(text: &str, date_system: DateSystem) -> Option<f64> {
  let mut parts = text.split('-');
  let year = parts.next()?.parse::<i32>().ok()?;
  let month = parts.next()?.parse::<i32>().ok()?;
  let day = parts.next()?.parse::<i32>().ok()?;
  if parts.next().is_some() {
    return None;
  }
  valid_date_serial_with_system(year, month, day, date_system)
}

fn parse_month_date_input(text: &str, date_system: DateSystem) -> Option<f64> {
  let tokens = date_input_tokens(text)?;
  let month_index = tokens
    .iter()
    .position(|token| matches!(token.kind, DateInputTokenKind::Month(_)))?;
  let month = match tokens[month_index].kind {
    DateInputTokenKind::Month(month) => month,
    DateInputTokenKind::Number(_) => return None,
  };
  let numbers = tokens
    .iter()
    .filter_map(|token| match token.kind {
      DateInputTokenKind::Number(value) => Some((value, token.text)),
      DateInputTokenKind::Month(_) => None,
    })
    .collect::<Vec<_>>();
  let (year, day) = match (month_index, numbers.as_slice()) {
    // Source: LibreOffice ImpSvNumberInputScan::GetDateRef, nMonthPos == 1
    // with two numeric substrings uses MDY in the default English locale.
    (0, [(day, _), (year, _)]) => (*year, *day),
    // For middle-month long dates, LO accepts both DMY and YMD depending on
    // locale/date order. Treat a four-digit leading number as year, otherwise
    // use the common DMY order.
    (1, [(left, left_text), (right, _)]) if left_text.len() >= 4 => (*left, *right),
    (1, [(day, _), (year, _)]) => (*year, *day),
    // Month at the end is accepted as YDM only when the locale long-date order
    // requests it. The English default used by the LO FODS corpus does not.
    (2, _) => return None,
    _ => return None,
  };
  valid_date_serial_with_system(year, i32::from(month), day, date_system)
}

fn parse_numeric_date_input(text: &str, date_system: DateSystem) -> Option<f64> {
  let separator = if text.contains('/') {
    '/'
  } else if text.contains('.') {
    '.'
  } else {
    return None;
  };
  let parts = text.split(separator).collect::<Vec<_>>();
  let [first, second, third] = parts.as_slice() else {
    return None;
  };
  let first_number = first.trim().parse::<i32>().ok()?;
  let second_number = second.trim().parse::<i32>().ok()?;
  let mut third_number = third.trim().parse::<i32>().ok()?;
  if third_number < 100 {
    third_number = expand_two_digit_year(third_number);
  }

  let (year, month, day) = if first.trim().len() >= 4 {
    (first_number, second_number, third_number)
  } else {
    // Source: LibreOffice's default English number formatter parses slash
    // dates such as 5/3/2011 as MDY.
    (third_number, first_number, second_number)
  };
  valid_date_serial_with_system(year, month, day, date_system)
}

#[derive(Clone, Copy)]
struct DateInputToken<'a> {
  kind: DateInputTokenKind,
  text: &'a str,
}

#[derive(Clone, Copy)]
enum DateInputTokenKind {
  Number(i32),
  Month(u8),
}

fn date_input_tokens(text: &str) -> Option<Vec<DateInputToken<'_>>> {
  let mut tokens = Vec::new();
  let mut index = 0usize;
  while index < text.len() {
    let rest = &text[index..];
    let ch = rest.chars().next()?;
    if ch.is_ascii_whitespace() || matches!(ch, ',' | '-' | '/' | '.') {
      index += ch.len_utf8();
      continue;
    }
    if ch.is_ascii_digit() {
      let start = index;
      index += ch.len_utf8();
      while index < text.len() {
        let ch = text[index..].chars().next()?;
        if !ch.is_ascii_digit() {
          break;
        }
        index += ch.len_utf8();
      }
      let value = text[start..index].parse::<i32>().ok()?;
      tokens.push(DateInputToken {
        kind: DateInputTokenKind::Number(value),
        text: &text[start..index],
      });
      continue;
    }
    if ch.is_ascii_alphabetic() {
      let start = index;
      index += ch.len_utf8();
      while index < text.len() {
        let ch = text[index..].chars().next()?;
        if !ch.is_ascii_alphabetic() {
          break;
        }
        index += ch.len_utf8();
      }
      let month = english_month_number(&text[start..index])?;
      tokens.push(DateInputToken {
        kind: DateInputTokenKind::Month(month),
        text: &text[start..index],
      });
      continue;
    }
    return None;
  }
  Some(tokens)
}

fn english_month_number(text: &str) -> Option<u8> {
  match text.to_ascii_uppercase().as_str() {
    "JAN" | "JANUARY" => Some(1),
    "FEB" | "FEBRUARY" => Some(2),
    "MAR" | "MARCH" => Some(3),
    "APR" | "APRIL" => Some(4),
    "MAY" => Some(5),
    "JUN" | "JUNE" => Some(6),
    "JUL" | "JULY" => Some(7),
    "AUG" | "AUGUST" => Some(8),
    "SEP" | "SEPT" | "SEPTEMBER" => Some(9),
    "OCT" | "OCTOBER" => Some(10),
    "NOV" | "NOVEMBER" => Some(11),
    "DEC" | "DECEMBER" => Some(12),
    _ => None,
  }
}

fn valid_date_serial_with_system(
  year: i32,
  month: i32,
  day: i32,
  date_system: DateSystem,
) -> Option<f64> {
  if !(1..=12).contains(&month) || day < 1 {
    return None;
  }
  let days = days_from_civil(year, month, day)?;
  let (actual_year, actual_month, actual_day) = civil_from_days(days)?;
  if actual_year != year || actual_month != month as u32 || actual_day != day as u32 {
    return None;
  }
  date_serial_with_system(year, month, day, date_system)
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

fn index_matrix<'doc>(
  rows: Vec<Vec<FormulaValue<'doc>>>,
  row: u32,
  column: u32,
  arg_count: usize,
) -> FormulaValue<'doc> {
  let height = rows.len();
  let width = rows.iter().map(Vec::len).max().unwrap_or(0);
  let b_row_vector_special = arg_count == 2 || column == 0;
  let b_row_vector_element = height == 1 && (column != 0 || (b_row_vector_special && row != 0));
  let b_vector_element = b_row_vector_element || (width == 1 && row != 0);
  if height == 0
    || width == 0
    || (!b_vector_element && (column as usize > width || row as usize > height))
  {
    return FormulaValue::Error(FormulaErrorValue::Ref);
  }
  if row == 0 && column == 0 {
    return FormulaValue::Matrix(rows);
  }
  if b_vector_element {
    let (element, other_dimension) = if b_row_vector_element && !b_row_vector_special {
      (column as usize, row as usize)
    } else {
      (row as usize, column as usize)
    };
    if element == 0 || element > width * height || other_dimension > 1 {
      return FormulaValue::Error(FormulaErrorValue::Ref);
    }
    let index = element - 1;
    let row_index = index / width;
    let column_index = index % width;
    return rows
      .get(row_index)
      .and_then(|row| row.get(column_index))
      .cloned()
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::Ref));
  }
  if column == 0 {
    let row_index = row as usize - 1;
    return rows
      .get(row_index)
      .map(|row| FormulaValue::Matrix(vec![row.clone()]))
      .unwrap_or(FormulaValue::Error(FormulaErrorValue::Ref));
  }
  if row == 0 {
    let column_index = column as usize - 1;
    let values = rows
      .into_iter()
      .map(|row| {
        vec![
          row
            .get(column_index)
            .cloned()
            .unwrap_or(FormulaValue::Error(FormulaErrorValue::Ref)),
        ]
      })
      .collect();
    return FormulaValue::Matrix(values);
  }
  rows
    .get(row as usize - 1)
    .and_then(|row_values| row_values.get(column as usize - 1))
    .cloned()
    .unwrap_or(FormulaValue::Error(FormulaErrorValue::Ref))
}

fn push_unique_qualified_range<'doc>(
  ranges: &mut Vec<QualifiedRange<'doc>>,
  range: QualifiedRange<'doc>,
) {
  if !ranges.contains(&range) {
    ranges.push(range);
  }
}

fn is_multicell_scalar_argument(value: &FormulaValue<'_>) -> bool {
  match value {
    FormulaValue::Reference(reference) => reference.range.cell_count_hint() != 1,
    FormulaValue::RefList(_) => true,
    FormulaValue::Matrix(rows) => {
      rows.len() != 1 || rows.first().map_or(true, |row| row.len() != 1)
    }
    _ => false,
  }
}

fn is_matrix_argument(value: &FormulaValue<'_>) -> bool {
  matches!(
    value,
    FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
  )
}

fn cell_in_range(address: CellAddress, range: &CellRange) -> bool {
  let start_column = range.start.column.min(range.end.column);
  let end_column = range.start.column.max(range.end.column);
  let start_row = range.start.row.min(range.end.row);
  let end_row = range.start.row.max(range.end.row);
  (start_column..=end_column).contains(&address.column)
    && (start_row..=end_row).contains(&address.row)
}

fn intersect_qualified_ranges<'doc>(
  left: &QualifiedRange<'doc>,
  right: &QualifiedRange<'doc>,
) -> Option<QualifiedRange<'doc>> {
  let sheet_name = compatible_range_sheet_name(left, right)?;
  if left.sheet != right.sheet {
    return None;
  }
  let left_start_column = left.range.start.column.min(left.range.end.column);
  let left_end_column = left.range.start.column.max(left.range.end.column);
  let left_start_row = left.range.start.row.min(left.range.end.row);
  let left_end_row = left.range.start.row.max(left.range.end.row);
  let right_start_column = right.range.start.column.min(right.range.end.column);
  let right_end_column = right.range.start.column.max(right.range.end.column);
  let right_start_row = right.range.start.row.min(right.range.end.row);
  let right_end_row = right.range.start.row.max(right.range.end.row);

  let start_column = left_start_column.max(right_start_column);
  let end_column = left_end_column.min(right_end_column);
  let start_row = left_start_row.max(right_start_row);
  let end_row = left_end_row.min(right_end_row);
  if start_column > end_column || start_row > end_row {
    return None;
  }
  Some(QualifiedRange {
    sheet: left.sheet,
    sheet_name,
    range: CellRange::new(
      CellAddress {
        column: start_column,
        row: start_row,
      },
      CellAddress {
        column: end_column,
        row: end_row,
      },
    ),
    start_flags: left.start_flags,
    end_flags: left.end_flags,
  })
}

fn bounding_qualified_ranges<'doc>(
  ranges: &[QualifiedRange<'doc>],
) -> Option<QualifiedRange<'doc>> {
  let first = ranges.first()?;
  let mut result = first.clone();
  for range in &ranges[1..] {
    result = extend_qualified_range(&result, range)?;
  }
  Some(result)
}

fn extend_qualified_range<'doc>(
  left: &QualifiedRange<'doc>,
  right: &QualifiedRange<'doc>,
) -> Option<QualifiedRange<'doc>> {
  let sheet_name = compatible_range_sheet_name(left, right)?;
  if left.sheet != right.sheet {
    return None;
  }
  let left_start_column = left.range.start.column.min(left.range.end.column);
  let left_end_column = left.range.start.column.max(left.range.end.column);
  let left_start_row = left.range.start.row.min(left.range.end.row);
  let left_end_row = left.range.start.row.max(left.range.end.row);
  let right_start_column = right.range.start.column.min(right.range.end.column);
  let right_end_column = right.range.start.column.max(right.range.end.column);
  let right_start_row = right.range.start.row.min(right.range.end.row);
  let right_end_row = right.range.start.row.max(right.range.end.row);

  Some(QualifiedRange {
    sheet: left.sheet,
    sheet_name,
    range: CellRange::new(
      CellAddress {
        column: left_start_column.min(right_start_column),
        row: left_start_row.min(right_start_row),
      },
      CellAddress {
        column: left_end_column.max(right_end_column),
        row: left_end_row.max(right_end_row),
      },
    ),
    start_flags: left.start_flags,
    end_flags: right.end_flags,
  })
}

fn compatible_range_sheet_name<'doc>(
  left: &QualifiedRange<'doc>,
  right: &QualifiedRange<'doc>,
) -> Option<Option<SheetName<'doc>>> {
  match (&left.sheet_name, &right.sheet_name) {
    (Some(left), Some(right)) if left != right => None,
    (Some(left), _) => Some(Some(left.clone())),
    (_, Some(right)) => Some(Some(right.clone())),
    (None, None) => Some(None),
  }
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
  crate::parser::formula_error_value(value)
    .map(formula_error_from_lex)
    .unwrap_or(FormulaErrorValue::Unknown)
}

fn logical_value(value: &FormulaValue<'_>) -> Option<bool> {
  match value {
    FormulaValue::Boolean(value) => Some(*value),
    FormulaValue::Number(value) => Some(*value != 0.0),
    _ => None,
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
    FormulaErrorValue::Parameter => "Err:511",
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

fn reorder_columns<'doc>(
  matrix: &[Vec<FormulaValue<'doc>>],
  order: &[usize],
) -> Vec<Vec<FormulaValue<'doc>>> {
  matrix
    .iter()
    .map(|row| {
      order
        .iter()
        .filter_map(|column| row.get(*column).cloned())
        .collect()
    })
    .collect()
}

fn matrix_item<'doc>(
  matrix: &'doc [Vec<FormulaValue<'doc>>],
  row: usize,
  column: usize,
) -> Option<&'doc FormulaValue<'doc>> {
  if matrix.len() == 1 && matrix.first()?.len() == 1 {
    return matrix.first()?.first();
  }
  if matrix.len() == 1 {
    return matrix.first()?.get(column);
  }
  if matrix.first()?.len() == 1 {
    return matrix.get(row)?.first();
  }
  matrix.get(row)?.get(column)
}

fn take_drop_bounds(len: usize, arg: Option<isize>, take: bool) -> (usize, usize) {
  let Some(arg) = arg else {
    return (0, len);
  };
  let abs = arg.unsigned_abs();
  if abs >= len {
    return (0, len);
  }
  if take {
    if arg < 0 { (len - abs, len) } else { (0, abs) }
  } else if arg < 0 {
    (0, len - abs)
  } else {
    (abs, len)
  }
}

fn sort_multi_key_order<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  keys: &[(Vec<FormulaValue<'doc>>, bool)],
  left: usize,
  right: usize,
) -> std::cmp::Ordering {
  for (values, ascending) in keys {
    let ordering = sort_value_order(evaluator, &values[left], &values[right], *ascending);
    if ordering != std::cmp::Ordering::Equal {
      return ordering;
    }
  }
  left.cmp(&right)
}

fn sort_value_order(
  evaluator: &FormulaEvaluator<'_, '_>,
  left: &FormulaValue<'_>,
  right: &FormulaValue<'_>,
  ascending: bool,
) -> std::cmp::Ordering {
  let ordering = match (left, right) {
    (FormulaValue::Number(left), FormulaValue::Number(right)) => left.total_cmp(right),
    (FormulaValue::String(left), FormulaValue::String(right)) => {
      match compare_text(evaluator, left.as_ref(), right.as_ref(), false) {
        value if value < 0 => std::cmp::Ordering::Less,
        value if value > 0 => std::cmp::Ordering::Greater,
        _ => std::cmp::Ordering::Equal,
      }
    }
    (FormulaValue::Boolean(left), FormulaValue::Boolean(right)) => left.cmp(right),
    (FormulaValue::Blank, FormulaValue::Blank) => std::cmp::Ordering::Equal,
    (FormulaValue::Error(left), FormulaValue::Error(right)) => {
      error_text_value(*left).cmp(error_text_value(*right))
    }
    (left, right) => sort_value_rank(left).cmp(&sort_value_rank(right)),
  };
  if ascending {
    ordering
  } else {
    ordering.reverse()
  }
}

fn sort_value_rank(value: &FormulaValue<'_>) -> u8 {
  match value {
    FormulaValue::Number(_) => 0,
    FormulaValue::String(_) => 1,
    FormulaValue::Boolean(_) => 2,
    FormulaValue::Error(_) => 3,
    FormulaValue::Blank => 4,
    FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => 5,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
    let mut sheets = vec![
      worksheet_value_model(
        &identity,
        Some(&worksheet),
        &[],
        &WorkbookMetadata::default(),
        &WorkbookStyles::default(),
      )
      .unwrap(),
    ];
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
    let sheet = worksheet_value_model(
      &identity,
      Some(&worksheet),
      &[],
      &WorkbookMetadata::default(),
      &WorkbookStyles::default(),
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
    assert!(matches!(
      book.evaluate_formula_text(SheetId(1), None, "DEVSQ({1,-2,3,1/0,5})"),
      Some(FormulaValue::Error(_))
    ));
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
      book.evaluate_formula_text(SheetId(1), None, "LOGNORM.INV(0,0,1)"),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "NORM.INV(0,63,5)"),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "NORM.S.INV(0)"),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
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
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "POWER(0,-1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "GEOMEAN(C1:C1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "HARMEAN(C1:C1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Num))
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
      book.evaluate_formula_text(SheetId(1), None, "ISFORMULA(A1:A2)"),
      Some(FormulaValue::Boolean(false))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ISNUMBER(TRUE())"),
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
      Some(FormulaValue::Matrix(vec![
        vec![FormulaValue::String(Cow::Borrowed("label"))],
        vec![FormulaValue::String(Cow::Borrowed("url"))],
      ]))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "HYPERLINK(\"url\",1/0)"),
      Some(FormulaValue::Matrix(vec![
        vec![FormulaValue::Error(FormulaErrorValue::Div0)],
        vec![FormulaValue::String(Cow::Borrowed("url"))],
      ]))
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
      book.evaluate_formula_text(SheetId(1), None, "COUNTBLANK(D1)"),
      Some(FormulaValue::Number(1.0))
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
      book.evaluate_formula_text(SheetId(1), None, "AGGREGATE(3,4,F1:H1)"),
      Some(FormulaValue::Number(3.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "AGGREGATE(3,6,F1:H1)"),
      Some(FormulaValue::Number(2.0))
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
  fn evaluation_book_countblank_matches_libreoffice_formula_result_rules() {
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
      .build();

    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "COUNTBLANK(A1:A4)"),
      Some(FormulaValue::Number(2.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ISBLANK(A1)"),
      Some(FormulaValue::Boolean(true))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "ISBLANK(A3)"),
      Some(FormulaValue::Boolean(false))
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
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
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

    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DATE(,1,31)"),
      Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DATE(1983,,31)"),
      Some(FormulaValue::Number(30316.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DATE(0,12,31)"),
      Some(FormulaValue::Number(36891.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DATE(100,1,1)"),
      Some(FormulaValue::Error(FormulaErrorValue::Value))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DATE(1582,10,15)"),
      Some(FormulaValue::Number(-115859.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DAY(\"1899-12-29T15:26:14\")"),
      Some(FormulaValue::Number(29.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "DAYS(\"1990-01-01\",\"1980-10-10\")"),
      Some(FormulaValue::Number(3370.0))
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
