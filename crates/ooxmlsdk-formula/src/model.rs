use std::borrow::Cow;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::workbook_part::WorkbookPart;
use ooxmlsdk::schemas::x;
use ooxmlsdk::sdk::SdkPart;
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
use statrs::distribution::{
  Beta, Binomial, ChiSquared, Continuous, ContinuousCDF, Discrete, DiscreteCDF, Exp,
  FisherSnedecor, Gamma, Hypergeometric, LogNormal, NegativeBinomial, Normal, Poisson, StudentsT,
  Weibull,
};
use statrs::function::{erf as statrs_erf, gamma as statrs_gamma};

use crate::{
  AddressFlags, CellAddress, CellRange, DisplayValue, FormulaError, FormulaErrorValue,
  FormulaValue, QualifiedAddress, QualifiedRange, Result, SheetId,
};

const MAX_FORMULA_RECALC_PASSES: usize = 12;
const MAX_EXPANDED_RANGE_CELLS: u64 = 20_000;
const XLSX_MAX_COLUMN_ZERO_BASED: u32 = 16_383;
const XLSX_MAX_ROW_ZERO_BASED: u32 = 1_048_575;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ConvertClass {
  Mass,
  Length,
  Time,
  Pressure,
  Force,
  Energy,
  Power,
  Magnetism,
  Temperature,
  Volume,
  Area,
  Speed,
  Information,
}

#[derive(Clone, Copy, Debug)]
struct ConvertUnit {
  name: &'static str,
  factor: &'static str,
  offset: &'static str,
  class: ConvertClass,
  prefix: bool,
  linear: bool,
}

impl ConvertUnit {
  const fn proportional(
    name: &'static str,
    factor: &'static str,
    class: ConvertClass,
    prefix: bool,
  ) -> Self {
    Self {
      name,
      factor,
      offset: "0.0",
      class,
      prefix,
      linear: false,
    }
  }

  const fn linear(
    name: &'static str,
    factor: &'static str,
    offset: &'static str,
    class: ConvertClass,
    prefix: bool,
  ) -> Self {
    Self {
      name,
      factor,
      offset,
      class,
      prefix,
      linear: true,
    }
  }

  fn factor(&self) -> f64 {
    self.factor.parse().unwrap_or(0.0)
  }

  fn offset(&self) -> f64 {
    self.offset.parse().unwrap_or(0.0)
  }
}

const CONVERT_UNITS: &[ConvertUnit] = &[
  ConvertUnit::proportional("g", "1.0000000000000000E00", ConvertClass::Mass, true),
  ConvertUnit::proportional("sg", "6.8522050005347800E-05", ConvertClass::Mass, false),
  ConvertUnit::proportional("lbm", "2.2046229146913400E-03", ConvertClass::Mass, false),
  ConvertUnit::proportional("u", "6.0221370000000000E23", ConvertClass::Mass, true),
  ConvertUnit::proportional("ozm", "3.5273971800362700E-02", ConvertClass::Mass, false),
  ConvertUnit::proportional("stone", "1.574730E-04", ConvertClass::Mass, false),
  ConvertUnit::proportional("ton", "1.102311E-06", ConvertClass::Mass, false),
  ConvertUnit::proportional("grain", "1.543236E01", ConvertClass::Mass, false),
  ConvertUnit::proportional("pweight", "7.054792E-01", ConvertClass::Mass, false),
  ConvertUnit::proportional("hweight", "1.968413E-05", ConvertClass::Mass, false),
  ConvertUnit::proportional("shweight", "2.204623E-05", ConvertClass::Mass, false),
  ConvertUnit::proportional("brton", "9.842065E-07", ConvertClass::Mass, false),
  ConvertUnit::proportional("cwt", "2.2046226218487758E-05", ConvertClass::Mass, false),
  ConvertUnit::proportional(
    "shweight",
    "2.2046226218487758E-05",
    ConvertClass::Mass,
    false,
  ),
  ConvertUnit::proportional(
    "uk_cwt",
    "1.9684130552221213E-05",
    ConvertClass::Mass,
    false,
  ),
  ConvertUnit::proportional("lcwt", "1.9684130552221213E-05", ConvertClass::Mass, false),
  ConvertUnit::proportional(
    "hweight",
    "1.9684130552221213E-05",
    ConvertClass::Mass,
    false,
  ),
  ConvertUnit::proportional(
    "uk_ton",
    "9.8420652761106063E-07",
    ConvertClass::Mass,
    false,
  ),
  ConvertUnit::proportional("LTON", "9.8420652761106063E-07", ConvertClass::Mass, false),
  ConvertUnit::proportional("m", "1.0000000000000000E00", ConvertClass::Length, true),
  ConvertUnit::proportional("mi", "6.2137119223733397E-04", ConvertClass::Length, false),
  ConvertUnit::proportional("Nmi", "5.3995680345572354E-04", ConvertClass::Length, false),
  ConvertUnit::proportional("in", "3.9370078740157480E01", ConvertClass::Length, false),
  ConvertUnit::proportional("ft", "3.2808398950131234E00", ConvertClass::Length, false),
  ConvertUnit::proportional("yd", "1.0936132983377078E00", ConvertClass::Length, false),
  ConvertUnit::proportional("ang", "1.0000000000000000E10", ConvertClass::Length, true),
  ConvertUnit::proportional("Pica", "2.8346456692913386E03", ConvertClass::Length, false),
  ConvertUnit::proportional(
    "picapt",
    "2.8346456692913386E03",
    ConvertClass::Length,
    false,
  ),
  ConvertUnit::proportional("pica", "2.36220472441E02", ConvertClass::Length, false),
  ConvertUnit::proportional("ell", "8.748906E-01", ConvertClass::Length, false),
  ConvertUnit::proportional("parsec", "3.240779E-17", ConvertClass::Length, true),
  ConvertUnit::proportional("pc", "3.240779E-17", ConvertClass::Length, true),
  ConvertUnit::proportional(
    "lightyear",
    "1.0570234557732930E-16",
    ConvertClass::Length,
    true,
  ),
  ConvertUnit::proportional("ly", "1.0570234557732930E-16", ConvertClass::Length, true),
  ConvertUnit::proportional(
    "survey_mi",
    "6.2136994949494949E-04",
    ConvertClass::Length,
    false,
  ),
  ConvertUnit::proportional("yr", "3.1688087814028950E-08", ConvertClass::Time, false),
  ConvertUnit::proportional("day", "1.1574074074074074E-05", ConvertClass::Time, false),
  ConvertUnit::proportional("d", "1.1574074074074074E-05", ConvertClass::Time, false),
  ConvertUnit::proportional("hr", "2.7777777777777778E-04", ConvertClass::Time, false),
  ConvertUnit::proportional("mn", "1.6666666666666667E-02", ConvertClass::Time, false),
  ConvertUnit::proportional("min", "1.6666666666666667E-02", ConvertClass::Time, false),
  ConvertUnit::proportional("sec", "1.0000000000000000E00", ConvertClass::Time, true),
  ConvertUnit::proportional("s", "1.0000000000000000E00", ConvertClass::Time, true),
  ConvertUnit::proportional("Pa", "1.0000000000000000E00", ConvertClass::Pressure, true),
  ConvertUnit::proportional(
    "atm",
    "9.8692329999819300E-06",
    ConvertClass::Pressure,
    true,
  ),
  ConvertUnit::proportional("at", "9.8692329999819300E-06", ConvertClass::Pressure, true),
  ConvertUnit::proportional(
    "mmHg",
    "7.5006170799862700E-03",
    ConvertClass::Pressure,
    true,
  ),
  ConvertUnit::proportional(
    "Torr",
    "7.5006380000000000E-03",
    ConvertClass::Pressure,
    false,
  ),
  ConvertUnit::proportional(
    "psi",
    "1.4503770000000000E-04",
    ConvertClass::Pressure,
    false,
  ),
  ConvertUnit::proportional("N", "1.0000000000000000E00", ConvertClass::Force, true),
  ConvertUnit::proportional("dyn", "1.0000000000000000E05", ConvertClass::Force, true),
  ConvertUnit::proportional("dy", "1.0000000000000000E05", ConvertClass::Force, true),
  ConvertUnit::proportional("lbf", "2.24808923655339E-01", ConvertClass::Force, false),
  ConvertUnit::proportional("pond", "1.019716E02", ConvertClass::Force, true),
  ConvertUnit::proportional("J", "1.0000000000000000E00", ConvertClass::Energy, true),
  ConvertUnit::proportional("e", "1.0000000000000000E07", ConvertClass::Energy, true),
  ConvertUnit::proportional("c", "2.3900624947346700E-01", ConvertClass::Energy, true),
  ConvertUnit::proportional("cal", "2.3884619064201700E-01", ConvertClass::Energy, true),
  ConvertUnit::proportional("eV", "6.2414570000000000E18", ConvertClass::Energy, true),
  ConvertUnit::proportional("ev", "6.2414570000000000E18", ConvertClass::Energy, true),
  ConvertUnit::proportional("HPh", "3.7250611111111111E-07", ConvertClass::Energy, false),
  ConvertUnit::proportional("hh", "3.7250611111111111E-07", ConvertClass::Energy, false),
  ConvertUnit::proportional("Wh", "2.7777777777777778E-04", ConvertClass::Energy, true),
  ConvertUnit::proportional("wh", "2.7777777777777778E-04", ConvertClass::Energy, true),
  ConvertUnit::proportional("flb", "2.37304222192651E01", ConvertClass::Energy, false),
  ConvertUnit::proportional("BTU", "9.4781506734901500E-04", ConvertClass::Energy, false),
  ConvertUnit::proportional("btu", "9.4781506734901500E-04", ConvertClass::Energy, false),
  ConvertUnit::proportional("W", "1.0000000000000000E00", ConvertClass::Power, true),
  ConvertUnit::proportional("w", "1.0000000000000000E00", ConvertClass::Power, true),
  ConvertUnit::proportional("HP", "1.341022E-03", ConvertClass::Power, false),
  ConvertUnit::proportional("h", "1.341022E-03", ConvertClass::Power, false),
  ConvertUnit::proportional("PS", "1.359622E-03", ConvertClass::Power, false),
  ConvertUnit::proportional("T", "1.0000000000000000E00", ConvertClass::Magnetism, true),
  ConvertUnit::proportional("ga", "1.0000000000000000E04", ConvertClass::Magnetism, true),
  ConvertUnit::linear(
    "C",
    "1.0000000000000000E00",
    "-2.7315000000000000E02",
    ConvertClass::Temperature,
    false,
  ),
  ConvertUnit::linear(
    "cel",
    "1.0000000000000000E00",
    "-2.7315000000000000E02",
    ConvertClass::Temperature,
    false,
  ),
  ConvertUnit::linear(
    "F",
    "1.8000000000000000E00",
    "-2.5537222222222222E02",
    ConvertClass::Temperature,
    false,
  ),
  ConvertUnit::linear(
    "fah",
    "1.8000000000000000E00",
    "-2.5537222222222222E02",
    ConvertClass::Temperature,
    false,
  ),
  ConvertUnit::linear(
    "K",
    "1.0000000000000000E00",
    "0.0000000000000000E00",
    ConvertClass::Temperature,
    true,
  ),
  ConvertUnit::linear(
    "kel",
    "1.0000000000000000E00",
    "0.0000000000000000E00",
    ConvertClass::Temperature,
    true,
  ),
  ConvertUnit::linear(
    "Reau",
    "8.0000000000000000E-01",
    "-2.7315000000000000E02",
    ConvertClass::Temperature,
    false,
  ),
  ConvertUnit::linear(
    "Rank",
    "1.8000000000000000E00",
    "0.0000000000000000E00",
    ConvertClass::Temperature,
    false,
  ),
  ConvertUnit::proportional("tsp", "2.0288413621105798E02", ConvertClass::Volume, false),
  ConvertUnit::proportional("tbs", "6.7628045403685994E01", ConvertClass::Volume, false),
  ConvertUnit::proportional("oz", "3.3814022701842997E01", ConvertClass::Volume, false),
  ConvertUnit::proportional("cup", "4.2267528377303746E00", ConvertClass::Volume, false),
  ConvertUnit::proportional("pt", "2.1133764188651873E00", ConvertClass::Volume, false),
  ConvertUnit::proportional(
    "us_pt",
    "2.1133764188651873E00",
    ConvertClass::Volume,
    false,
  ),
  ConvertUnit::proportional(
    "uk_pt",
    "1.7597539863927023E00",
    ConvertClass::Volume,
    false,
  ),
  ConvertUnit::proportional("qt", "1.0566882094325937E00", ConvertClass::Volume, false),
  ConvertUnit::proportional("gal", "2.6417205235814842E-01", ConvertClass::Volume, false),
  ConvertUnit::proportional("l", "1.0000000000000000E00", ConvertClass::Volume, true),
  ConvertUnit::proportional("L", "1.0000000000000000E00", ConvertClass::Volume, true),
  ConvertUnit::proportional("lt", "1.0000000000000000E00", ConvertClass::Volume, true),
  ConvertUnit::proportional("m3", "1.0000000000000000E-03", ConvertClass::Volume, true),
  ConvertUnit::proportional("mi3", "2.3991275857892772E-13", ConvertClass::Volume, false),
  ConvertUnit::proportional(
    "Nmi3",
    "1.5742621468581148E-13",
    ConvertClass::Volume,
    false,
  ),
  ConvertUnit::proportional("in3", "6.1023744094732284E01", ConvertClass::Volume, false),
  ConvertUnit::proportional("ft3", "3.5314666721488590E-02", ConvertClass::Volume, false),
  ConvertUnit::proportional("yd3", "1.3079506193143922E-03", ConvertClass::Volume, false),
  ConvertUnit::proportional("ang3", "1.0000000000000000E27", ConvertClass::Volume, true),
  ConvertUnit::proportional(
    "Pica3",
    "2.2776990435870636E07",
    ConvertClass::Volume,
    false,
  ),
  ConvertUnit::proportional(
    "picapt3",
    "2.2776990435870636E07",
    ConvertClass::Volume,
    false,
  ),
  ConvertUnit::proportional("pica3", "1.31811287245E04", ConvertClass::Volume, false),
  ConvertUnit::proportional(
    "barrel",
    "6.2898107704321051E-03",
    ConvertClass::Volume,
    false,
  ),
  ConvertUnit::proportional("bushel", "2.837759E-02", ConvertClass::Volume, false),
  ConvertUnit::proportional("regton", "3.531467E-04", ConvertClass::Volume, false),
  ConvertUnit::proportional("GRT", "3.531467E-04", ConvertClass::Volume, false),
  ConvertUnit::proportional(
    "Schooner",
    "2.3529411764705882E00",
    ConvertClass::Volume,
    false,
  ),
  ConvertUnit::proportional(
    "Middy",
    "3.5087719298245614E00",
    ConvertClass::Volume,
    false,
  ),
  ConvertUnit::proportional(
    "Glass",
    "5.0000000000000000E00",
    ConvertClass::Volume,
    false,
  ),
  ConvertUnit::proportional("Sixpack", "0.5", ConvertClass::Volume, false),
  ConvertUnit::proportional("Humpen", "2.0", ConvertClass::Volume, false),
  ConvertUnit::proportional("ly3", "1.1810108125623799E-51", ConvertClass::Volume, false),
  ConvertUnit::proportional("MTON", "1.4125866688595436E00", ConvertClass::Volume, false),
  ConvertUnit::proportional("tspm", "2.0000000000000000E02", ConvertClass::Volume, false),
  ConvertUnit::proportional(
    "uk_gal",
    "2.1996924829908779E-01",
    ConvertClass::Volume,
    false,
  ),
  ConvertUnit::proportional(
    "uk_qt",
    "8.7987699319635115E-01",
    ConvertClass::Volume,
    false,
  ),
  ConvertUnit::proportional("m2", "1.0000000000000000E00", ConvertClass::Area, true),
  ConvertUnit::proportional("mi2", "3.8610215854244585E-07", ConvertClass::Area, false),
  ConvertUnit::proportional("Nmi2", "2.9155334959812286E-07", ConvertClass::Area, false),
  ConvertUnit::proportional("in2", "1.5500031000062000E03", ConvertClass::Area, false),
  ConvertUnit::proportional("ft2", "1.0763910416709722E01", ConvertClass::Area, false),
  ConvertUnit::proportional("yd2", "1.1959900463010803E00", ConvertClass::Area, false),
  ConvertUnit::proportional("ang2", "1.0000000000000000E20", ConvertClass::Area, true),
  ConvertUnit::proportional("Pica2", "8.0352160704321409E06", ConvertClass::Area, false),
  ConvertUnit::proportional(
    "picapt2",
    "8.0352160704321409E06",
    ConvertClass::Area,
    false,
  ),
  ConvertUnit::proportional("pica2", "5.58001116002232E04", ConvertClass::Area, false),
  ConvertUnit::proportional(
    "Morgen",
    "4.0000000000000000E-04",
    ConvertClass::Area,
    false,
  ),
  ConvertUnit::proportional("ar", "1.000000E-02", ConvertClass::Area, true),
  ConvertUnit::proportional("acre", "2.471053815E-04", ConvertClass::Area, false),
  ConvertUnit::proportional(
    "uk_acre",
    "2.4710538146716534E-04",
    ConvertClass::Area,
    false,
  ),
  ConvertUnit::proportional(
    "us_acre",
    "2.4710439304662790E-04",
    ConvertClass::Area,
    false,
  ),
  ConvertUnit::proportional("ly2", "1.1172985860549147E-32", ConvertClass::Area, false),
  ConvertUnit::proportional("ha", "1.000000E-04", ConvertClass::Area, false),
  ConvertUnit::proportional("m/s", "1.0000000000000000E00", ConvertClass::Speed, true),
  ConvertUnit::proportional("m/sec", "1.0000000000000000E00", ConvertClass::Speed, true),
  ConvertUnit::proportional("m/h", "3.6000000000000000E03", ConvertClass::Speed, true),
  ConvertUnit::proportional("m/hr", "3.6000000000000000E03", ConvertClass::Speed, true),
  ConvertUnit::proportional("mph", "2.2369362920544023E00", ConvertClass::Speed, false),
  ConvertUnit::proportional("kn", "1.9438444924406048E00", ConvertClass::Speed, false),
  ConvertUnit::proportional("admkn", "1.9438446603753486E00", ConvertClass::Speed, false),
  ConvertUnit::proportional(
    "ludicrous speed",
    "2.0494886343432328E-14",
    ConvertClass::Speed,
    false,
  ),
  ConvertUnit::proportional(
    "ridiculous speed",
    "4.0156958471424288E-06",
    ConvertClass::Speed,
    false,
  ),
  ConvertUnit::proportional("bit", "1.00E00", ConvertClass::Information, true),
  ConvertUnit::proportional("byte", "1.25E-01", ConvertClass::Information, true),
];

fn convert_unit(value: f64, from: &str, to: &str) -> std::result::Result<f64, FormulaErrorValue> {
  let (from_unit, from_level) =
    find_convert_unit(from).ok_or(FormulaErrorValue::IllegalArgument)?;
  let (to_unit, to_level) = find_convert_unit(to).ok_or(FormulaErrorValue::IllegalArgument)?;
  if from_unit.class != to_unit.class || from_unit.linear != to_unit.linear {
    return Err(FormulaErrorValue::IllegalArgument);
  }
  if from_unit.linear {
    let base = convert_linear_to_base(value, from_unit, from_level);
    return Ok(convert_linear_from_base(base, to_unit, to_level));
  }

  let mut result = value * to_unit.factor() / from_unit.factor();
  if from_unit.class == ConvertClass::Information {
    let binary_from = from_level > 0 && from_level % 10 == 0;
    let binary_to = to_level > 0 && to_level % 10 == 0;
    if binary_from || binary_to {
      if binary_from && binary_to {
        let level = from_level - to_level;
        if level != 0 {
          result *= 2.0_f64.powi(level as i32);
        }
      } else if binary_from {
        result *= 2.0_f64.powi(from_level as i32) / 10.0_f64.powi(to_level as i32);
      } else {
        result *= 10.0_f64.powi(from_level as i32) / 2.0_f64.powi(to_level as i32);
      }
      return Ok(result);
    }
  }

  let level = from_level - to_level;
  if level != 0 {
    result *= 10.0_f64.powi(level as i32);
  }
  Ok(result)
}

fn convert_linear_to_base(value: f64, unit: &ConvertUnit, level: i16) -> f64 {
  let mut result = value;
  if level != 0 {
    result *= 10.0_f64.powi(level as i32);
  }
  result /= unit.factor();
  result - unit.offset()
}

fn convert_linear_from_base(value: f64, unit: &ConvertUnit, level: i16) -> f64 {
  let mut result = value + unit.offset();
  result *= unit.factor();
  if level != 0 {
    result *= 10.0_f64.powi(-(level as i32));
  }
  result
}

fn find_convert_unit(unit: &str) -> Option<(&'static ConvertUnit, i16)> {
  let mut partial = None;
  for candidate in CONVERT_UNITS {
    if let Some(level) = convert_matching_level(candidate, unit) {
      if level == 0 {
        return Some((candidate, level));
      }
      partial = Some((candidate, level));
    }
  }
  partial
}

fn convert_matching_level(unit: &ConvertUnit, reference: &str) -> Option<i16> {
  let normalized = if let Some(index) = reference.rfind('^') {
    if index > 0 && reference[index..].chars().count() == 2 {
      let mut value = String::with_capacity(reference.len() - 1);
      value.push_str(&reference[..index]);
      value.push_str(&reference[index + 1..]);
      Cow::Owned(value)
    } else {
      Cow::Borrowed(reference)
    }
  } else {
    Cow::Borrowed(reference)
  };
  let reference = normalized.as_ref();
  if reference == unit.name {
    return Some(0);
  }
  if !unit.prefix || !reference.ends_with(unit.name) {
    return None;
  }
  let prefix = &reference[..reference.len() - unit.name.len()];
  let mut level = match prefix {
    "y" => -24,
    "z" => -21,
    "a" => -18,
    "f" => -15,
    "p" => -12,
    "n" => -9,
    "u" => -6,
    "m" => -3,
    "c" => -2,
    "d" => -1,
    "da" => 1,
    "e" => 1,
    "h" => 2,
    "k" => 3,
    "M" => 6,
    "G" => 9,
    "T" => 12,
    "P" => 15,
    "E" => 18,
    "Z" => 21,
    "Y" => 24,
    "ki" if unit.class == ConvertClass::Information => 10,
    "Mi" if unit.class == ConvertClass::Information => 20,
    "Gi" if unit.class == ConvertClass::Information => 30,
    "Ti" if unit.class == ConvertClass::Information => 40,
    "Pi" if unit.class == ConvertClass::Information => 50,
    "Ei" if unit.class == ConvertClass::Information => 60,
    "Zi" if unit.class == ConvertClass::Information => 70,
    "Yi" if unit.class == ConvertClass::Information => 80,
    _ => return None,
  };
  if !matches!(level, 10 | 20 | 30 | 40 | 50 | 60 | 70 | 80) {
    if reference.ends_with('2') {
      level *= 2;
    } else if reference.ends_with('3') {
      level *= 3;
    }
  }
  Some(level)
}

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

pub fn normalize_formula_text(formula: &str, grammar: FormulaGrammar) -> Cow<'_, str> {
  let formula = formula.trim();
  let formula = formula
    .strip_prefix("of:=")
    .or_else(|| formula.strip_prefix("of:"))
    .or_else(|| formula.strip_prefix('='))
    .unwrap_or(formula);
  match grammar {
    FormulaGrammar::ExcelA1 => Cow::Borrowed(formula),
    FormulaGrammar::ExcelR1C1 => Cow::Owned(normalize_r1c1_formula_text(
      formula,
      CellAddress { column: 0, row: 0 },
    )),
    FormulaGrammar::OpenFormula => Cow::Owned(normalize_open_formula_text(formula)),
    FormulaGrammar::CalcA1 => Cow::Owned(normalize_calc_formula_text(formula)),
  }
}

pub fn normalize_r1c1_formula_text(formula: &str, base: CellAddress) -> String {
  if let Some(reference) = r1c1_whole_axis_reference_to_a1(formula.trim(), base) {
    reference
  } else {
    formula.to_string()
  }
}

pub fn r1c1_whole_axis_reference_to_a1(reference: &str, base: CellAddress) -> Option<String> {
  let reference = reference.trim().trim_start_matches('=');
  if let Some(offset) = parse_r1c1_relative(reference, 'C') {
    let column = base.column.checked_add_signed(offset)?.checked_add(1)?;
    let column = column_name(column);
    return Some(format!("{column}:{column}"));
  }
  if let Some(offset) = parse_r1c1_relative(reference, 'R') {
    let row = base.row.checked_add_signed(offset)?.checked_add(1)?;
    return Some(format!("{row}:{row}"));
  }
  None
}

fn normalize_open_formula_text(formula: &str) -> String {
  let text = normalize_formula_separators(formula);
  normalize_open_formula_references(&text)
}

fn normalize_calc_formula_text(formula: &str) -> String {
  normalize_formula_separators(formula)
}

fn canonical_function_name(name: &str) -> String {
  let mut upper = name
    .trim_start_matches("_xlfn.")
    .trim_start_matches("_xlws.")
    .to_ascii_uppercase();
  for prefix in ["COM.MICROSOFT.", "ORG.LIBREOFFICE.", "ORG.OPENOFFICE."] {
    if let Some(stripped) = upper.strip_prefix(prefix) {
      upper = stripped.to_string();
      break;
    }
  }
  upper
}

fn normalize_formula_separators(formula: &str) -> String {
  let mut output = String::with_capacity(formula.len());
  let mut quoted = false;
  for ch in formula.chars() {
    match ch {
      '"' => {
        quoted = !quoted;
        output.push(ch);
      }
      ';' if !quoted => output.push(','),
      _ => output.push(ch),
    }
  }
  output
}

fn normalize_open_formula_references(formula: &str) -> String {
  let mut output = String::with_capacity(formula.len());
  let mut chars = formula.chars().peekable();
  while let Some(ch) = chars.next() {
    if ch == '[' {
      let mut reference = String::new();
      let mut closed = false;
      for next in chars.by_ref() {
        if next == ']' {
          closed = true;
          break;
        }
        reference.push(next);
      }
      if closed {
        output.push_str(&normalize_open_formula_reference(&reference));
      } else {
        output.push('[');
        output.push_str(&reference);
      }
    } else {
      output.push(ch);
    }
  }
  output
}

fn normalize_open_formula_reference(reference: &str) -> String {
  let mut reference = reference.trim_start_matches('.').replace(":.", ":");
  let first_part_end = reference.find(':').unwrap_or(reference.len());
  if let Some(dot) = reference[..first_part_end].rfind('.')
    && !reference[..first_part_end].contains('!')
  {
    reference.replace_range(dot..=dot, "!");
  }
  reference
}

fn parse_r1c1_relative(reference: &str, axis: char) -> Option<i32> {
  let rest = reference.strip_prefix(axis)?;
  if rest.is_empty() {
    return Some(0);
  }
  let offset = rest.strip_prefix('[')?.strip_suffix(']')?;
  offset.parse::<i32>().ok()
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
    self
      .evaluate_formula_ast_value(current_sheet, current_cell, formula)
      .map(|value| self.final_formula_value(current_sheet, current_cell, value))
  }

  fn evaluate_formula_ast_value(
    &self,
    current_sheet: SheetId,
    current_cell: Option<CellAddress>,
    formula: &str,
  ) -> Option<FormulaValue<'doc>> {
    let (ast, unsupported) = parse_formula_ast(current_sheet, formula);
    if !unsupported.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    FormulaEvaluator {
      book: self,
      current_sheet,
      current_cell,
      locals: BTreeMap::new(),
    }
    .evaluate(ast.as_ref()?)
  }

  pub fn evaluate_parsed_formula(
    &self,
    current_sheet: SheetId,
    current_cell: Option<CellAddress>,
    formula: &ParsedFormula<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    if !formula.unsupported.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    if let Some(value) =
      self.evaluate_special_formula_text(current_sheet, current_cell, formula.source.as_ref())
    {
      return Some(value);
    }
    FormulaEvaluator {
      book: self,
      current_sheet,
      current_cell,
      locals: BTreeMap::new(),
    }
    .evaluate(formula.ast.as_ref()?)
    .map(|value| self.final_formula_value(current_sheet, current_cell, value))
  }

  pub fn evaluate_formula_text_with_grammar(
    &self,
    current_sheet: SheetId,
    current_cell: Option<CellAddress>,
    formula: &str,
    grammar: FormulaGrammar,
  ) -> Option<FormulaValue<'doc>> {
    let normalized = normalize_formula_text(formula, grammar);
    self.evaluate_formula_text(current_sheet, current_cell, normalized.as_ref())
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
    if is_formula_error_literal(clean) {
      return Some(FormulaValue::Error(error_value(clean)));
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
      let left = self.evaluate_formula_ast_value(current_sheet, current_cell, left)?;
      let right = self.evaluate_formula_ast_value(current_sheet, current_cell, right)?;
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
        let context = FormulaEvaluator {
          book: self,
          current_sheet,
          current_cell,
          locals: BTreeMap::new(),
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
    if matches!(value, FormulaValue::Reference(_)) {
      FormulaEvaluator {
        book: self,
        current_sheet,
        current_cell,
        locals: BTreeMap::new(),
      }
      .first_value(&value)
    } else {
      value
    }
  }

  pub fn formula_text(&self, sheet: SheetId, address: CellAddress) -> Option<String> {
    let formula = self.formulas.get(&(sheet, address))?;
    let text = formula.text.as_ref();
    Some(if text.starts_with('{') {
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

pub fn parse_formula_text<'doc>(
  current_sheet: SheetId,
  source: impl Into<Cow<'doc, str>>,
) -> ParsedFormula<'doc> {
  parse_formula(current_sheet, source.into())
}

pub fn parse_formula_with_context<'doc>(
  context: FormulaParseContext,
  source: impl Into<Cow<'doc, str>>,
) -> ParsedFormula<'doc> {
  let source = source.into();
  if context.grammar == FormulaGrammar::ExcelA1 {
    return parse_formula(context.current_sheet, source);
  }
  let normalized = normalize_formula_text(source.as_ref(), context.grammar);
  parse_formula(context.current_sheet, Cow::Owned(normalized.into_owned()))
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
      '|' => {
        tokens.push(FormulaToken::Separator(FormulaSeparator::Row));
        index += ch.len_utf8();
      }
      '(' | ')' => {
        index += ch.len_utf8();
      }
      _ if is_formula_word_char(ch) => {
        let (word, next) = parse_formula_word(text, index);
        let next_non_space = text[next..].chars().find(|ch| !ch.is_whitespace());
        if next_non_space == Some('(') {
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
    self.parse_logical()
  }

  fn parse_logical(&mut self) -> Option<FormulaAst<'doc>> {
    let mut left = self.parse_comparison()?;
    loop {
      self.skip_ws();
      let Some(name) = self.consume_adjacent_logical_function_name() else {
        break;
      };
      let mut args = self.parse_argument_list()?;
      args.insert(0, left);
      left = FormulaAst::Function {
        name: Cow::Owned(name.to_string()),
        args,
      };
    }
    Some(left)
  }

  fn parse_comparison(&mut self) -> Option<FormulaAst<'doc>> {
    let mut left = self.parse_union()?;
    loop {
      self.skip_ws();
      let Some(op) = self.consume_comparison_operator() else {
        break;
      };
      let right = self.parse_union()?;
      left = FormulaAst::Binary {
        op,
        left: Box::new(left),
        right: Box::new(right),
      };
    }
    Some(left)
  }

  fn parse_union(&mut self) -> Option<FormulaAst<'doc>> {
    let mut left = self.parse_range()?;
    loop {
      self.skip_ws();
      if !self.consume_char('~') {
        break;
      }
      let right = self.parse_range()?;
      left = FormulaAst::Binary {
        op: FormulaOperator::Union,
        left: Box::new(left),
        right: Box::new(right),
      };
    }
    Some(left)
  }

  fn parse_range(&mut self) -> Option<FormulaAst<'doc>> {
    let mut left = self.parse_intersection()?;
    loop {
      self.skip_ws();
      if !self.consume_char(':') {
        break;
      }
      let right = self.parse_intersection()?;
      left = FormulaAst::Binary {
        op: FormulaOperator::Range,
        left: Box::new(left),
        right: Box::new(right),
      };
    }
    Some(left)
  }

  fn parse_intersection(&mut self) -> Option<FormulaAst<'doc>> {
    let mut left = self.parse_concat()?;
    loop {
      self.skip_ws();
      if !self.consume_char('!') {
        break;
      }
      let right = self.parse_concat()?;
      left = FormulaAst::Binary {
        op: FormulaOperator::Intersection,
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
        if !row.is_empty() {
          rows.push(row);
        } else if !rows.is_empty() {
          rows.push(vec![FormulaAst::Literal(FormulaValue::Blank)]);
        }
        break;
      }
      if self.consume_char(',') {
        row.push(FormulaAst::Literal(FormulaValue::Blank));
        continue;
      }
      if self.consume_char(';') || self.consume_char('|') {
        if row.is_empty() {
          row.push(FormulaAst::Literal(FormulaValue::Blank));
        }
        rows.push(row);
        row = Vec::new();
        continue;
      }
      row.push(self.parse_expression()?);
      self.skip_ws();
      if self.consume_char(',') {
        continue;
      }
      if self.consume_char(';') || self.consume_char('|') {
        rows.push(row);
        row = Vec::new();
        continue;
      }
      if self.consume_char('}') {
        rows.push(row);
        break;
      }
      return None;
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
    if self.peek_char() == Some('(') {
      let args = self.parse_argument_list()?;
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

  fn parse_argument_list(&mut self) -> Option<Vec<FormulaAst<'doc>>> {
    if !self.consume_char('(') {
      return None;
    }
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
      self.skip_ws();
      if self.peek_char() == Some(')') {
        args.push(FormulaAst::Literal(FormulaValue::Blank));
      }
    }
    Some(args)
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

  fn consume_adjacent_logical_function_name(&mut self) -> Option<&'static str> {
    let rest = &self.text[self.index..];
    let name = if rest
      .get(..3)
      .is_some_and(|value| value.eq_ignore_ascii_case("AND"))
    {
      "AND"
    } else if rest
      .get(..2)
      .is_some_and(|value| value.eq_ignore_ascii_case("OR"))
    {
      "OR"
    } else if rest
      .get(..3)
      .is_some_and(|value| value.eq_ignore_ascii_case("NOT"))
    {
      "NOT"
    } else {
      return None;
    };
    let next = self.index + name.len();
    if next < self.text.len() {
      let next_char = self.text[next..].chars().next()?;
      if is_formula_word_char(next_char) {
        return None;
      }
    }
    let mut probe = next;
    while let Some(ch) = self.text[probe..].chars().next() {
      if !ch.is_whitespace() {
        break;
      }
      probe += ch.len_utf8();
    }
    if self.text[probe..].starts_with('(') {
      self.index = next;
      Some(name)
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
    if op == FormulaOperator::Intersection {
      return self.evaluate_intersection_ast(left, right);
    }
    if op == FormulaOperator::Range {
      return self.evaluate_range_ast(left, right);
    }
    let left = self.evaluate(left)?;
    let right = self.evaluate(right)?;
    if let Some(error) = propagate_binary_error(&left, &right) {
      return Some(FormulaValue::Error(error));
    }
    match op {
      FormulaOperator::Add => self.numeric_binary(left, right, |a, b| a + b),
      FormulaOperator::Subtract => self.numeric_binary(left, right, |a, b| a - b),
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
        if matches!(left, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
          || matches!(right, FormulaValue::Reference(_) | FormulaValue::Matrix(_))
        {
          return self.map_binary_values(left, right, |evaluator, left, right| {
            Some(FormulaValue::Boolean(evaluator.compare(left, right, op)))
          });
        }
        Some(FormulaValue::Boolean(self.compare(&left, &right, op)))
      }
      _ => None,
    }
  }

  fn evaluate_intersection_ast(
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
      _ => {
        let rows = intersections
          .into_iter()
          .flat_map(|range| self.matrix_values(&FormulaValue::Reference(range)))
          .collect();
        Some(FormulaValue::Matrix(rows))
      }
    }
  }

  fn evaluate_range_ast(
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
    let rows = ranges
      .into_iter()
      .flat_map(|range| self.matrix_values(&FormulaValue::Reference(range)))
      .collect();
    Some(FormulaValue::Matrix(rows))
  }

  fn range_reference_ranges_from_ast(
    &self,
    left: &FormulaAst<'doc>,
    right: &FormulaAst<'doc>,
  ) -> Vec<QualifiedRange<'doc>> {
    let left_ranges = self.reference_ranges_from_ast(left);
    let right_ranges = self.reference_ranges_from_ast(right);
    let mut ranges = Vec::new();
    for left_range in &left_ranges {
      for right_range in &right_ranges {
        if let Some(range) = extend_qualified_range(left_range, right_range) {
          ranges.push(range);
        }
      }
    }
    ranges
  }

  fn evaluate_function(
    &self,
    name: &Cow<'doc, str>,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let upper = canonical_function_name(name);
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
        if args.len() != 2 {
          return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
        }
        let value = match args.first().and_then(|arg| self.evaluate(arg)) {
          Some(value) => self.scalar_value(value),
          None => FormulaValue::Error(FormulaErrorValue::Unknown),
        };
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
      "IFS" => self.evaluate_ifs_function(args),
      "SWITCH" => self.evaluate_switch(args),
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
        args
          .first()
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.scalar_value(value)),
        Some(FormulaValue::Error(_))
      ))),
      "ISNA" => Some(FormulaValue::Boolean(matches!(
        args
          .first()
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.scalar_value(value)),
        Some(FormulaValue::Error(FormulaErrorValue::NA))
      ))),
      "ISERR" => Some(FormulaValue::Boolean(matches!(
        args
          .first()
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.scalar_value(value)),
        Some(FormulaValue::Error(error)) if error != FormulaErrorValue::NA
      ))),
      "ISBLANK" => Some(FormulaValue::Boolean(matches!(
        args
          .first()
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.scalar_value(value)),
        Some(FormulaValue::Blank)
      ))),
      "ISTEXT" => Some(FormulaValue::Boolean(matches!(
        args
          .first()
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.scalar_value(value)),
        Some(FormulaValue::String(_))
      ))),
      "ISNONTEXT" => Some(FormulaValue::Boolean(!matches!(
        args
          .first()
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.scalar_value(value)),
        Some(FormulaValue::String(_))
      ))),
      "ISLOGICAL" => Some(FormulaValue::Boolean(matches!(
        args
          .first()
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.scalar_value(value)),
        Some(FormulaValue::Boolean(_))
      ))),
      "ISNUMBER" => Some(FormulaValue::Boolean(matches!(
        args
          .first()
          .and_then(|arg| self.evaluate(arg))
          .map(|value| self.scalar_value(value)),
        Some(FormulaValue::Number(_))
      ))),
      "ISREF" => Some(FormulaValue::Boolean(matches!(
        args.first().and_then(|arg| self.evaluate(arg)),
        Some(FormulaValue::Reference(_))
      ))),
      "ISFORMULA" => self.evaluate_is_formula(args),
      "ERROR.TYPE" | "ERRORTYPE" => self.evaluate_error_type(args),
      "TYPE" => self.evaluate_type(args),
      "AREAS" => self.evaluate_areas(args),
      "MIN" => self
        .numeric_values(args)
        .reduce(f64::min)
        .map(FormulaValue::Number)
        .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
      "MINA" => self.evaluate_mina(args),
      "MAX" => self
        .numeric_values(args)
        .reduce(f64::max)
        .map(FormulaValue::Number)
        .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
      "MAXA" => self.evaluate_maxa(args),
      "AND" => self.evaluate_and(args),
      "OR" => self.evaluate_or(args),
      "XOR" => self.evaluate_xor(args),
      "NOT" => {
        if args.len() != 1 {
          return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
        };
        let Some(value) = self.evaluate(&args[0]) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
        };
        Some(FormulaValue::Boolean(!self.truthy(&value)))
      }
      "TRUE" => Some(FormulaValue::Boolean(true)),
      "FALSE" => Some(FormulaValue::Boolean(false)),
      "NA" => Some(FormulaValue::Error(FormulaErrorValue::NA)),
      "STYLE" | "CURRENT" => Some(FormulaValue::Number(0.0)),
      "N" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?).unwrap_or(0.0),
      )),
      "COUNTBLANK" => Some(FormulaValue::Number(
        self.count_blank(&self.evaluate(args.first()?)?) as f64,
      )),
      "ABS" => {
        let Some(arg) = args.first() else {
          return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
        };
        let Some(value) = self.evaluate(arg) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
        };
        self
          .map_numeric_value(value, |value| value.abs())
          .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown)))
      }
      "SIGN" => self
        .number_arg(args, 0)
        .map(|value| FormulaValue::Number(sign_number(value)))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
      "INT" => {
        let Some(arg) = args.first() else {
          return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
        };
        Some(FormulaValue::Number(approx_floor(
          self.number(&self.evaluate(arg)?)?,
        )))
      }
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
        match self.number_arg(args, 0) {
          Some(value) => value,
          None => return Some(FormulaValue::Error(FormulaErrorValue::Unknown)),
        },
        true,
      ))),
      "ODD" => Some(FormulaValue::Number(even_odd(
        match self.number_arg(args, 0) {
          Some(value) => value,
          None => return Some(FormulaValue::Error(FormulaErrorValue::Unknown)),
        },
        false,
      ))),
      "RAWSUBTRACT" => self.evaluate_raw_subtract(args),
      "ISEVEN" => {
        let Some(value) = self.number_arg(args, 0) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
        };
        Some(FormulaValue::Boolean(
          approx_floor(value.abs()) as i64 % 2 == 0,
        ))
      }
      "ISODD" => {
        let Some(value) = self.number_arg(args, 0) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
        };
        Some(FormulaValue::Boolean(
          approx_floor(value.abs()) as i64 % 2 != 0,
        ))
      }
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
      "SQRTPI" => {
        let Some(value) = self.number_arg(args, 0) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
        };
        if value < 0.0 {
          Some(FormulaValue::Error(FormulaErrorValue::Num))
        } else {
          Some(FormulaValue::Number((value * std::f64::consts::PI).sqrt()))
        }
      }
      "RADIANS" => args
        .first()
        .and_then(|arg| self.evaluate(arg))
        .and_then(|value| self.number(&value))
        .map(|value| FormulaValue::Number(value.to_radians()))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
      "DEGREES" => args
        .first()
        .and_then(|arg| self.evaluate(arg))
        .and_then(|value| self.number(&value))
        .map(|value| FormulaValue::Number(value.to_degrees()))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
      "SIN" => self.evaluate_numeric_unary(args, f64::sin),
      "CSC" => self.evaluate_numeric_unary(args, |value| 1.0 / value.sin()),
      "COS" => self.evaluate_numeric_unary(args, f64::cos),
      "SEC" => self.evaluate_numeric_unary(args, |value| 1.0 / value.cos()),
      "TAN" => self.evaluate_numeric_unary(args, f64::tan),
      "COT" => self.evaluate_numeric_unary(args, |value| 1.0 / value.tan()),
      "SINH" => self.evaluate_numeric_unary(args, f64::sinh),
      "CSCH" => self.evaluate_numeric_unary(args, |value| 1.0 / value.sinh()),
      "COSH" => self.evaluate_numeric_unary(args, f64::cosh),
      "SECH" => self.evaluate_numeric_unary(args, |value| 1.0 / value.cosh()),
      "TANH" => self.evaluate_numeric_unary(args, f64::tanh),
      "COTH" => self.evaluate_numeric_unary(args, |value| 1.0 / value.tanh()),
      "ASIN" => self.evaluate_numeric_unary(args, f64::asin),
      "ASINH" => self.evaluate_numeric_unary(args, f64::asinh),
      "ACOS" => self.evaluate_numeric_unary(args, f64::acos),
      "ACOSH" => self.evaluate_numeric_unary(args, f64::acosh),
      "ACOT" => {
        self.evaluate_numeric_unary(args, |value| std::f64::consts::FRAC_PI_2 - value.atan())
      }
      "ATAN" => self.evaluate_numeric_unary(args, f64::atan),
      "ATANH" => {
        let Some(value) = self.number_arg(args, 0) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
        };
        if value.abs() >= 1.0 {
          Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
        } else {
          Some(FormulaValue::Number(value.atanh()))
        }
      }
      "ACOTH" => {
        let Some(value) = self.number_arg(args, 0) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
        };
        if value.abs() <= 1.0 {
          Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
        } else {
          Some(FormulaValue::Number((1.0 / value).atanh()))
        }
      }
      "ATAN2" => Some(FormulaValue::Number(
        self
          .number(&self.evaluate(args.get(1)?)?)?
          .atan2(self.number(&self.evaluate(args.first()?)?)?),
      )),
      "EXP" => self
        .number_arg(args, 0)
        .map(|value| FormulaValue::Number(value.exp()))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
      "LN" => self
        .number_arg(args, 0)
        .map(|value| FormulaValue::Number(value.ln()))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
      "LOG10" => Some(FormulaValue::Number(
        self.number(&self.evaluate(args.first()?)?)?.log10(),
      )),
      "LOG" => {
        let Some(value) = self.number_arg(args, 0) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
        };
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
      "ROUNDSIG" => self.evaluate_roundsig(args),
      "ROUNDUP" => self.evaluate_round_direction(args, true),
      "ROUNDDOWN" => self.evaluate_round_direction(args, false),
      "DATE" => self.evaluate_date(args),
      "DATEVALUE" => Some(datevalue(&self.text(&self.evaluate(args.first()?)?))),
      "TIME" => self.evaluate_time(args),
      "YEAR" => self.evaluate_date_part(args, DatePart::Year),
      "MONTH" => self.evaluate_date_part(args, DatePart::Month),
      "DAY" => self.evaluate_date_part(args, DatePart::Day),
      "WEEKDAY" => self.evaluate_weekday(args),
      "HOUR" => self.evaluate_time_part(args, TimePart::Hour),
      "MINUTE" => self.evaluate_time_part(args, TimePart::Minute),
      "SECOND" => self.evaluate_time_part(args, TimePart::Second),
      "DAYS" => match (self.number_arg(args, 0), self.number_arg(args, 1)) {
        (Some(end), Some(start)) => Some(FormulaValue::Number(end - start)),
        _ => Some(FormulaValue::Error(FormulaErrorValue::Unknown)),
      },
      "DAYS360" => self.evaluate_days360(args),
      "TODAY" => self.evaluate_today(),
      "EOMONTH" => self.evaluate_eomonth(args),
      "EDATE" => self.evaluate_edate(args),
      "ISLEAPYEAR" => self.evaluate_is_leap_year(args),
      "TRIM" => Some(FormulaValue::String(Cow::Owned(trim_formula_text(
        &self.text(&self.evaluate(args.first()?)?),
      )))),
      "UPPER" => Some(FormulaValue::String(Cow::Owned(
        self.text(&self.evaluate(args.first()?)?).to_uppercase(),
      ))),
      "LOWER" => Some(FormulaValue::String(Cow::Owned(
        self.text(&self.evaluate(args.first()?)?).to_lowercase(),
      ))),
      "ROT13" => Some(FormulaValue::String(Cow::Owned(rot13_formula_text(
        &self.text(&self.evaluate(args.first()?)?),
      )))),
      "PROPER" => {
        let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
        };
        Some(FormulaValue::String(Cow::Owned(proper_formula_text(
          &self.text(&value),
        ))))
      }
      "LEN" => {
        let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
        };
        Some(FormulaValue::Number(
          self.text(&value).chars().count() as f64
        ))
      }
      "LENB" => {
        let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
        };
        Some(FormulaValue::Number(
          text_byte_len(&self.text(&value)) as f64
        ))
      }
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
      "NUMBERVALUE" => self.evaluate_numbervalue(args),
      "DOLLAR" => self.evaluate_dollar(args),
      "FIXED" => self.evaluate_fixed(args),
      "BASE" => self.evaluate_base(args),
      "DECIMAL" | "DEC" => self.evaluate_decimal(args),
      "CONVERT" => self.evaluate_convert(args),
      "BIN2DEC" => self.evaluate_base_to_decimal(args, 2),
      "OCT2DEC" => self.evaluate_base_to_decimal(args, 8),
      "HEX2DEC" => self.evaluate_base_to_decimal(args, 16),
      "BIN2OCT" => self.evaluate_base_to_base(args, 2, 8, -536_870_912.0, 536_870_911.0),
      "BIN2HEX" => self.evaluate_base_to_base(args, 2, 16, -549_755_813_888.0, 549_755_813_887.0),
      "OCT2BIN" => self.evaluate_base_to_base(args, 8, 2, -512.0, 511.0),
      "OCT2HEX" => self.evaluate_base_to_base(args, 8, 16, -549_755_813_888.0, 549_755_813_887.0),
      "HEX2BIN" => self.evaluate_base_to_base(args, 16, 2, -512.0, 511.0),
      "HEX2OCT" => self.evaluate_base_to_base(args, 16, 8, -536_870_912.0, 536_870_911.0),
      "DEC2BIN" => self.evaluate_decimal_to_base(args, 2, -512.0, 511.0),
      "DEC2OCT" => self.evaluate_decimal_to_base(args, 8, -536_870_912.0, 536_870_911.0),
      "DEC2HEX" => self.evaluate_decimal_to_base(args, 16, -549_755_813_888.0, 549_755_813_887.0),
      "BITAND" => self.evaluate_bit_binary(args, |left, right| left & right),
      "BITOR" => self.evaluate_bit_binary(args, |left, right| left | right),
      "BITXOR" => self.evaluate_bit_binary(args, |left, right| left ^ right),
      "BITLSHIFT" => self.evaluate_bit_shift(args, true),
      "BITRSHIFT" => self.evaluate_bit_shift(args, false),
      "CODE" => self
        .text(&self.evaluate(args.first()?)?)
        .chars()
        .next()
        .map(|ch| FormulaValue::Number(ch as u32 as f64))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Value))),
      "UNICODE" => self
        .text(&self.evaluate(args.first()?)?)
        .chars()
        .next()
        .map(|ch| FormulaValue::Number(ch as u32 as f64))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Value))),
      "CHAR" => {
        let Some(code) = self.number_arg(args, 0) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
        };
        let code = code as u32;
        char::from_u32(code)
          .map(|ch| FormulaValue::String(Cow::Owned(ch.to_string())))
          .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
      }
      "UNICHAR" => {
        let code = self.number(&self.evaluate(args.first()?)?)? as u32;
        if code == 0 {
          Some(FormulaValue::String(Cow::Borrowed("")))
        } else {
          char::from_u32(code)
            .map(|ch| FormulaValue::String(Cow::Owned(ch.to_string())))
            .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
        }
      }
      "CLEAN" => Some(FormulaValue::String(Cow::Owned(clean_formula_text(
        &self.text(&self.evaluate(args.first()?)?),
      )))),
      "CHOOSE" => self.evaluate_choose(args),
      "CONCAT" | "CONCATENATE" => Some(FormulaValue::String(Cow::Owned(
        self.values(args).map(|value| self.text(&value)).collect(),
      ))),
      "EXACT" => Some(FormulaValue::Boolean(
        self.text(&self.evaluate(args.first()?)?) == self.text(&self.evaluate(args.get(1)?)?),
      )),
      "FIND" => self.evaluate_find(args, true),
      "FINDB" => self.evaluate_findb(args),
      "SEARCH" => self.evaluate_find(args, false),
      "SEARCHB" => self.evaluate_findb(args),
      "REPT" => Some(FormulaValue::String(Cow::Owned(
        self
          .text(&self.evaluate(args.first()?)?)
          .repeat(self.number(&self.evaluate(args.get(1)?)?)?.max(0.0) as usize),
      ))),
      "SUBSTITUTE" => self.evaluate_substitute(args),
      "REPLACE" => self.evaluate_replace(args),
      "TEXTBEFORE" => self.evaluate_text_before_after(args, false),
      "TEXTAFTER" => self.evaluate_text_before_after(args, true),
      "TEXTSPLIT" => self.evaluate_textsplit(args),
      "TEXTJOIN" => self.evaluate_textjoin(args),
      "REPLACEB" => self.evaluate_replaceb(args),
      "ASC" => self.evaluate_width_conversion(args, false),
      "JIS" => self.evaluate_width_conversion(args, true),
      "MEDIAN" => {
        let mut values = self.numeric_args(args);
        percentile_sorted(&mut values, 0.5, PercentileKind::Inc)
          .map(FormulaValue::Number)
          .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown)))
      }
      "LARGE" => self.evaluate_large_small(args, true),
      "SMALL" => self.evaluate_large_small(args, false),
      "TRIMMEAN" => self.evaluate_trimmean(args),
      "DEVSQ" => self.evaluate_devsq(args),
      "AVEDEV" => self.evaluate_avedev(args),
      "AVERAGEA" => self.evaluate_averagea(args),
      "STDEV.P" | "STDEVP" => variance_slice(&self.numeric_args(args), false)
        .map(|value| FormulaValue::Number(value.sqrt())),
      "STDEV.S" | "STDEV" => variance_slice(&self.numeric_args(args), true)
        .map(|value| FormulaValue::Number(value.sqrt()))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
      "STDEVA" => self.evaluate_stdeva(args, true),
      "STDEVPA" => self.evaluate_stdeva(args, false),
      "VAR.P" | "VARP" => variance_slice(&self.numeric_args(args), false)
        .map(FormulaValue::Number)
        .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
      "VAR.S" | "VAR" => variance_slice(&self.numeric_args(args), true)
        .map(FormulaValue::Number)
        .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
      "VARA" => self.evaluate_vara(args),
      "VARPA" => self.evaluate_varpa(args),
      "SKEW" => self.evaluate_skew(args, false),
      "SKEW.P" | "SKEWP" => self.evaluate_skew(args, true),
      "GEOMEAN" => self.evaluate_geo_har_mean(args, false),
      "HARMEAN" => self.evaluate_geo_har_mean(args, true),
      "GAUSS" => Some(FormulaValue::Number(
        norm_s_dist(self.number(&self.evaluate(args.first()?)?)?) - 0.5,
      )),
      "TEXT" => Some(FormulaValue::String(Cow::Owned(self.evaluate_text(args)?))),
      "TIMEVALUE" => Some(timevalue(&self.text(&self.evaluate(args.first()?)?))),
      "BASISODATETIME" => self.evaluate_basis_o_datetime(args),
      "DATEDIF" => self.evaluate_datedif(args),
      "YEARFRAC" => self.evaluate_yearfrac(args),
      "WEEKNUM" => self.evaluate_weeknum(args),
      "ISOWEEKNUM" => self.evaluate_iso_weeknum(args),
      "WEEKS" => self.evaluate_weeks(args),
      "WEEKSINYEAR" => self.evaluate_weeks_in_year(args),
      "INDIRECT" => self.evaluate_indirect(args),
      "INDEX" => self.evaluate_index(args),
      "OFFSET" => self.evaluate_offset(args),
      "LOOKUP" => self.evaluate_lookup(args),
      "MATCH" => self.evaluate_match(args),
      "XMATCH" => self.evaluate_xmatch(args),
      "VLOOKUP" => self.evaluate_vlookup(args),
      "HLOOKUP" => self.evaluate_hlookup(args),
      "XLOOKUP" => self.evaluate_xlookup(args),
      "SHEETS" => self.evaluate_sheets(args),
      "SHEET" => self.evaluate_sheet(args),
      "FORMULA" | "FORMULATEXT" => self.evaluate_formula_text(args),
      "CELL" => self.evaluate_cell(args),
      "ADDRESS" => self.evaluate_address(args),
      "ROW" => self.evaluate_row_column(args, false),
      "COLUMN" => self.evaluate_row_column(args, true),
      "ROWS" => self.evaluate_rows_columns(args, false),
      "COLUMNS" => self.evaluate_rows_columns(args, true),
      "MID" => self.evaluate_mid(args),
      "MIDB" => self.evaluate_midb(args),
      "LEFT" => self.evaluate_left(args),
      "LEFTB" => self.evaluate_leftb(args),
      "RIGHT" => self.evaluate_right(args),
      "RIGHTB" => self.evaluate_rightb(args),
      "ROMAN" => self.evaluate_roman(args),
      "ARABIC" => self.evaluate_arabic(args),
      "HYPERLINK" => self.evaluate_hyperlink(args),
      "TOCOL" => self.evaluate_to_row_column(args, false),
      "TOROW" => self.evaluate_to_row_column(args, true),
      "CHOOSEROWS" => self.evaluate_choose_rows(args),
      "CHOOSECOLS" => self.evaluate_choose_cols(args),
      "EXPAND" => self.evaluate_expand(args),
      "SEQUENCE" => self.evaluate_sequence(args),
      "TRANSPOSE" => self.evaluate_transpose(args),
      "DROP" => self.evaluate_take_drop(args, false),
      "TAKE" => self.evaluate_take_drop(args, true),
      "SORT" => self.evaluate_sort(args),
      "SORTBY" => self.evaluate_sortby(args),
      "MMULT" => self.evaluate_mmult(args),
      "MDETERM" => self.evaluate_mdeterm(args),
      "MUNIT" => self.evaluate_munit(args),
      "GCD" => self.evaluate_gcd_lcm(args, false),
      "LCM" => self.evaluate_gcd_lcm(args, true),
      "FACT" => self.evaluate_fact(args),
      "FACTDOUBLE" => self.evaluate_fact_double(args),
      "MULTINOMIAL" => self.evaluate_multinomial(args),
      "COMBIN" => self.evaluate_combin(args, false),
      "COMBINA" => self.evaluate_combin(args, true),
      "PERMUT" => self.evaluate_permut(args),
      "PERMUTATIONA" => self.evaluate_permutationa(args),
      "MROUND" => self.evaluate_mround(args),
      "QUOTIENT" => self.evaluate_quotient(args),
      "PMT" => self.evaluate_pmt(args),
      "FV" => self.evaluate_fv(args),
      "FVSCHEDULE" => self.evaluate_fvschedule(args),
      "NPV" => self.evaluate_npv(args),
      "EFFECT" => self.evaluate_effect(args),
      "RATE" => self.evaluate_rate(args),
      "ISPMT" => self.evaluate_ispmt(args),
      "IPMT" => self.evaluate_ipmt(args),
      "PPMT" => self.evaluate_ppmt(args),
      "CUMIPMT" => self.evaluate_cumipmt(args),
      "CUMPRINC" => self.evaluate_cumprinc(args),
      "DDB" => self.evaluate_ddb(args),
      "VDB" => self.evaluate_vdb(args),
      "SUMIFS" => self.evaluate_sumifs(args),
      "COUNTIFS" => self.evaluate_countifs(args),
      "MAXIFS" => self.evaluate_minmaxifs(args, true),
      "MINIFS" => self.evaluate_minmaxifs(args, false),
      "SUMIF" => self.evaluate_sumif(args),
      "COUNTIF" => self.evaluate_countif(args),
      "AVERAGEIF" => self.evaluate_averageif(args),
      "AVERAGEIFS" => self.evaluate_averageifs(args),
      "DSUM" => self.evaluate_database_function(args, DatabaseFunction::Sum),
      "DCOUNT" => self.evaluate_database_function(args, DatabaseFunction::Count),
      "DCOUNTA" => self.evaluate_database_function(args, DatabaseFunction::CountA),
      "DAVERAGE" => self.evaluate_database_function(args, DatabaseFunction::Average),
      "DGET" => self.evaluate_database_function(args, DatabaseFunction::Get),
      "DMAX" => self.evaluate_database_function(args, DatabaseFunction::Max),
      "DMIN" => self.evaluate_database_function(args, DatabaseFunction::Min),
      "DPRODUCT" => self.evaluate_database_function(args, DatabaseFunction::Product),
      "DVAR" => self.evaluate_database_function(args, DatabaseFunction::Var),
      "DVARP" => self.evaluate_database_function(args, DatabaseFunction::VarP),
      "DSTDEV" => self.evaluate_database_function(args, DatabaseFunction::StdDev),
      "DSTDEVP" => self.evaluate_database_function(args, DatabaseFunction::StdDevP),
      "CEILING" => self.evaluate_ceiling(args, CeilingFloorKind::Odff),
      "CEILING.MATH" => self.evaluate_ceiling(args, CeilingFloorKind::Math),
      "CEILING.PRECISE" | "ISO.CEILING" => self.evaluate_ceiling(args, CeilingFloorKind::Precise),
      "FLOOR" => self.evaluate_floor(args, CeilingFloorKind::Odff),
      "FLOOR.MATH" => self.evaluate_floor(args, CeilingFloorKind::Math),
      "FLOOR.PRECISE" => self.evaluate_floor(args, CeilingFloorKind::Precise),
      "BETA.DIST" => self.evaluate_beta_dist(args, false),
      "BETADIST" => self.evaluate_beta_dist(args, true),
      "BETA.INV" | "BETAINV" => self.evaluate_beta_inv(args),
      "BINOM.DIST" | "BINOMDIST" => self.evaluate_binom_dist(args),
      "BINOM.DIST.RANGE" => self.evaluate_binom_dist_range(args),
      "BINOM.INV" => self.evaluate_binom_inv(args),
      "CHISQ.DIST" => self.evaluate_chisq_dist(args, false),
      "CHISQ.DIST.RT" | "CHIDIST" | "LEGACY.CHIDIST" => self.evaluate_chisq_dist(args, true),
      "CHISQDIST" => self.evaluate_chisq_dist(args, false),
      "CHISQ.INV" => self.evaluate_chisq_inv(args, false),
      "CHISQINV" => self.evaluate_chisq_inv(args, false),
      "CHISQ.INV.RT" | "LEGACY.CHIINV" => self.evaluate_chisq_inv(args, true),
      "CHISQ.TEST" | "CHITEST" | "LEGACY.CHITEST" => self.evaluate_chisq_test(args),
      "CONFIDENCE.NORM" | "CONFIDENCE" => self.evaluate_confidence_norm(args),
      "CONFIDENCE.T" => self.evaluate_confidence_t(args),
      "COVARIANCE.P" => self.evaluate_covariance(args, false),
      "COVARIANCE.S" => self.evaluate_covariance(args, true),
      "COVAR" => self.evaluate_covariance(args, false),
      "CORREL" => self.evaluate_correl(args),
      "PEARSON" => self.evaluate_correl(args),
      "PHI" => Some(FormulaValue::Number(
        Normal::standard().pdf(self.number(&self.evaluate(args.first()?)?)?),
      )),
      "ERF.PRECISE" => Some(FormulaValue::Number(erf(
        self.number(&self.evaluate(args.first()?)?)?,
      ))),
      "ERF" => self.evaluate_erf(args),
      "ERFC.PRECISE" | "ERFC" => self.evaluate_numeric_unary(args, erfc),
      "EXPON.DIST" | "EXPONDIST" => self.evaluate_expon_dist(args),
      "F.DIST" => self.evaluate_f_dist(args),
      "F.DIST.RT" | "FDIST" | "LEGACY.FDIST" => self.evaluate_f_dist_rt(args),
      "F.INV" => self.evaluate_f_inv(args, false),
      "F.INV.RT" | "FINV" | "LEGACY.FINV" => self.evaluate_f_inv(args, true),
      "F.TEST" | "FTEST" => self.evaluate_f_test(args),
      "GAMMA.DIST" | "GAMMADIST" => self.evaluate_gamma_dist(args),
      "GAMMA.INV" | "GAMMAINV" => self.evaluate_gamma_inv(args),
      "GAMMA" => self.evaluate_gamma(args),
      "GAMMALN.PRECISE" | "GAMMALN" => self
        .number(&self.evaluate(args.first()?)?)
        .filter(|value| *value > 0.0)
        .map(|value| FormulaValue::Number(log_gamma(value)))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
      "HYPGEOM.DIST" | "HYPGEOMDIST" => self.evaluate_hypgeom_dist(args),
      "LOGNORM.DIST" | "LOGNORMDIST" => self.evaluate_lognorm_dist(args),
      "LOGNORM.INV" | "LOGINV" => self.evaluate_lognorm_inv(args),
      "MODE.MULT" | "MODE.SNGL" | "MODE" => mode_slice(&self.numeric_args(args))
        .map(FormulaValue::Number)
        .or(Some(FormulaValue::Error(FormulaErrorValue::NA))),
      "NEGBINOM.DIST" | "NEGBINOMDIST" => self.evaluate_negbinom_dist(args),
      "NORM.DIST" | "NORMDIST" => self.evaluate_norm_dist(args),
      "NORM.INV" => self.evaluate_norm_inv(args),
      "NORM.S.DIST" | "NORMSDIST" | "LEGACY.NORMSDIST" => self.evaluate_norm_s_dist(args),
      "NORM.S.INV" | "LEGACY.NORMSINV" => self
        .number_arg(args, 0)
        .map(|value| FormulaValue::Number(norm_s_inv(value)))
        .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
      "PERCENTILE.EXC" => self.evaluate_percentile(args, PercentileKind::Exc),
      "PERCENTILE.INC" | "PERCENTILE" => self.evaluate_percentile(args, PercentileKind::Inc),
      "PERCENTRANK" | "PERCENTRANK.INC" => self.evaluate_percent_rank(args, PercentileKind::Inc),
      "PERCENTRANK.EXC" => self.evaluate_percent_rank(args, PercentileKind::Exc),
      "POISSON.DIST" | "POISSON" => self.evaluate_poisson_dist(args),
      "QUARTILE.EXC" => self.evaluate_quartile(args, PercentileKind::Exc),
      "QUARTILE.INC" | "QUARTILE" => self.evaluate_quartile(args, PercentileKind::Inc),
      "RANK.AVG" => self.evaluate_rank(args, true),
      "RANK.EQ" | "RANK" => self.evaluate_rank(args, false),
      "KURT" => self.evaluate_kurt(args),
      "T.DIST" => self.evaluate_t_dist(args),
      "T.DIST.2T" => self.evaluate_t_dist_tails(args, 2),
      "T.DIST.RT" => self.evaluate_t_dist_tails(args, 1),
      "T.INV" => self.evaluate_t_inv(args, false),
      "T.INV.2T" | "TINV" => self.evaluate_t_inv(args, true),
      "T.TEST" | "TTEST" => self.evaluate_t_test(args),
      "FISHER" => self.evaluate_fisher(args, false),
      "FISHERINV" => self.evaluate_fisher(args, true),
      "BESSELI" => self.evaluate_bessel(args, BesselKind::I),
      "BESSELJ" => self.evaluate_bessel(args, BesselKind::J),
      "BESSELK" => self.evaluate_bessel(args, BesselKind::K),
      "BESSELY" => self.evaluate_bessel(args, BesselKind::Y),
      "FOURIER" | "ORG.LIBREOFFICE.FOURIER" => self.evaluate_fourier(args),
      "IMREAL" => self.evaluate_complex_part(args, false),
      "IMAGINARY" => self.evaluate_complex_part(args, true),
      "IMABS" => self.evaluate_complex_abs(args),
      "IMARGUMENT" => self.evaluate_complex_argument(args),
      "IMCONJUGATE" => self.evaluate_complex_unary(args, |value| value.conj()),
      "IMEXP" => self.evaluate_complex_unary(args, |value| value.exp()),
      "IMLN" => self.evaluate_complex_unary(args, |value| value.ln()),
      "IMLOG10" => self.evaluate_complex_unary(args, |value| value.log10()),
      "IMLOG2" => self.evaluate_complex_unary(args, |value| value.log(2.0)),
      "IMSIN" => self.evaluate_complex_unary(args, |value| value.sin()),
      "IMSINH" => self.evaluate_complex_unary(args, |value| value.sinh()),
      "IMCOSH" => self.evaluate_complex_unary(args, |value| value.cosh()),
      "IMTAN" => self.evaluate_complex_unary(args, |value| value.tan()),
      "IMCOT" => self.evaluate_complex_unary(args, |value| Complex::new(1.0, 0.0) / value.tan()),
      "IMCSC" => self.evaluate_complex_unary(args, |value| Complex::new(1.0, 0.0) / value.sin()),
      "IMSQRT" => self.evaluate_complex_unary(args, |value| value.sqrt()),
      "IMCOS" => self.evaluate_complex_unary(args, |value| value.cos()),
      "IMSEC" => self.evaluate_complex_unary(args, |value| Complex::new(1.0, 0.0) / value.cos()),
      "IMSECH" => self.evaluate_complex_unary(args, |value| Complex::new(1.0, 0.0) / value.cosh()),
      "IMCSCH" => self.evaluate_complex_unary(args, |value| Complex::new(1.0, 0.0) / value.sinh()),
      "IMSUB" => self.evaluate_complex_binary(args, |left, right| left - right),
      "IMSUM" => self.evaluate_complex_sum_product(args, false),
      "IMPRODUCT" => self.evaluate_complex_sum_product(args, true),
      "COMPLEX" => self.evaluate_complex(args),
      "DELTA" => self.evaluate_delta(args),
      "GESTEP" => self.evaluate_gestep(args),
      "SLOPE" => self.evaluate_slope(args),
      "WEIBULL.DIST" | "WEIBULL" => self.evaluate_weibull_dist(args),
      "NETWORKDAYS.INTL" | "NETWORKDAYS" => self.evaluate_networkdays(args),
      "WORKDAY.INTL" | "WORKDAY" => self.evaluate_workday(args),
      "Z.TEST" | "ZTEST" => self.evaluate_z_test(args),
      "STANDARDIZE" => self.evaluate_standardize(args),
      "SUBTOTAL" => self.evaluate_subtotal(args),
      "AGGREGATE" => self.evaluate_aggregate(args),
      "DB" => self.evaluate_db(args),
      "DOLLARDE" => self.evaluate_dollar_decimal(args, true),
      "DOLLARFR" => self.evaluate_dollar_decimal(args, false),
      "INFO" => self.evaluate_info(args),
      "PRICE"
      | "ODDLYIELD"
      | "ODDLPRICE"
      | "AMORLINC"
      | "AMORDEGRC"
      | "EUROCONVERT"
      | "GETPIVOTDATA"
      | "LINEST"
      | "MDURATION"
      | "TBILLPRICE"
      | "TBILLYIELD"
      | "XIRR"
      | "YIELD"
      | "DISC"
      | "RECEIVED"
      | "INTRATE"
      | "IRR"
      | "MIRR"
      | "PRICEDISC"
      | "PRICEMAT"
      | "YIELDDISC"
      | "COUPDAYS"
      | "BAHTTEXT"
      | "FORECAST.ETS"
      | "FORECAST.ETS.MULT"
      | "FORECAST.ETS.STAT.MULT"
      | "LOGEST"
      | "CRITBINOM"
      | "MINVERSE"
      | "NORMINV"
      | "LEGACY.TDIST"
      | "TDIST"
      | "NPER"
      | "PV"
      | "PROB"
      | "IMDIV"
      | "UNIQUE"
      | "COUPDAYBS"
      | "COUPDAYSNC"
      | "COUPNCD"
      | "COUPNUM"
      | "COUPPCD"
      | "NOMINAL"
      | "SLN"
      | "SYD"
      | "ACCRINT"
      | "ACCRINTM"
      | "EASTERSUNDAY"
      | "IMPOWER"
      | "INTERCEPT"
      | "RSQ"
      | "HSTACK"
      | "VSTACK"
      | "WRAPCOLS"
      | "WRAPROWS"
      | "FILTER"
      | "FREQUENCY"
      | "GROWTH"
      | "TREND"
      | "YEARS"
      | "REGEX"
      | "RRI"
      | "STEYX"
      | "TBILLEQ"
      | "FORECAST"
      | "DAYSINMONTH"
      | "DAYSINYEAR"
      | "MONTHS"
      | "SUMX2MY2"
      | "SUMX2PY2"
      | "SUMXMY2"
      | "PDURATION"
      | "XNPV"
      | "SERIESSUM"
      | "COLOR"
      | "ENCODEURL"
      | "DHFG"
      | "OF"
      | "ROT"
      | "TESTFUNCBOOL"
      | "TESTFUNCCURR"
      | "TESTFUNCDATE"
      | "RANDARRAY" => Some(FormulaValue::Error(FormulaErrorValue::Unknown)),
      "TESTFUNCINT" | "TESTFUNCLONG" => Some(FormulaValue::Number(6.0)),
      "TESTFUNCSINGLE" | "TESTFUNCDOUBLE" => Some(FormulaValue::Number(5.5)),
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

  fn evaluate_numbervalue(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.is_empty() || args.len() > 3 {
      return None;
    }
    let mut text = self.text(&self.evaluate(args.first()?)?);
    let decimal = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.text(&value))
      .unwrap_or_else(|| ".".to_string());
    let group = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.text(&value))
      .unwrap_or_else(|| ",".to_string());
    if !group.is_empty() {
      text = text.replace(&group, "");
    }
    if decimal != "." && !decimal.is_empty() {
      text = text.replace(&decimal, ".");
    }
    text
      .trim()
      .parse::<f64>()
      .map(FormulaValue::Number)
      .ok()
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  fn evaluate_dollar(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.is_empty() || args.len() > 2 {
      return None;
    }
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let digits = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(2.0) as i32;
    let rounded = rtl_round(value, digits);
    if digits >= 0 {
      Some(FormulaValue::String(Cow::Owned(format!(
        "${rounded:.digits$}",
        digits = digits as usize
      ))))
    } else {
      Some(FormulaValue::String(Cow::Owned(format!("${rounded:.0}"))))
    }
  }

  fn evaluate_dollar_decimal(
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
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    let integer = value.trunc();
    let decimal = value - integer;
    let result = if fractional_to_decimal {
      integer + decimal * 100.0 / fraction
    } else {
      integer + decimal * fraction / 100.0
    };
    Some(FormulaValue::Number(result))
  }

  fn evaluate_fixed(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.is_empty() || args.len() > 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(value) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let digits = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(approx_floor)
      .unwrap_or(2.0) as i32;
    if !(-15..=15).contains(&digits) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let no_commas = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    let rounded = rtl_round(value, digits);
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

  fn evaluate_decimal(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let text = self.text(&self.evaluate(args.first()?)?);
    let radix = self.number(&self.evaluate(args.get(1)?)?)? as u32;
    if !(2..=36).contains(&radix) {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    match i128::from_str_radix(text.trim(), radix) {
      Ok(value) => Some(FormulaValue::Number(value as f64)),
      Err(_) => Some(FormulaValue::Error(FormulaErrorValue::Num)),
    }
  }

  fn evaluate_bit_binary(
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
    Some(FormulaValue::Number(op(left as u64, right as u64) as f64))
  }

  fn evaluate_bit_shift(
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
    let shift = shift as i32;
    if !(0.0..281_474_976_710_656.0).contains(&value) || shift.abs() > 53 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let value = value as u64;
    let result = if left_shift == (shift >= 0) {
      value.checked_shl(shift.unsigned_abs()).unwrap_or(0)
    } else {
      value.checked_shr(shift.unsigned_abs()).unwrap_or(0)
    };
    Some(FormulaValue::Number(result as f64))
  }

  fn evaluate_convert(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let from = self.text(&self.evaluate(args.get(1)?)?);
    let to = self.text(&self.evaluate(args.get(2)?)?);
    Some(match convert_unit(value, &from, &to) {
      Ok(value) => FormulaValue::Number(value),
      Err(error) => FormulaValue::Error(error),
    })
  }

  fn evaluate_base(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let value = self.number(&self.evaluate(args.first()?)?)?.floor() as i64;
    let radix = self.number(&self.evaluate(args.get(1)?)?)? as u32;
    let min_len = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0) as usize;
    if value < 0 || !(2..=36).contains(&radix) {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let mut text = format_radix(value as i128, radix)?;
    if text.len() < min_len {
      text = format!("{}{}", "0".repeat(min_len - text.len()), text);
    }
    Some(FormulaValue::String(Cow::Owned(text)))
  }

  fn evaluate_base_to_decimal(
    &self,
    args: &[FormulaAst<'doc>],
    base: u32,
  ) -> Option<FormulaValue<'doc>> {
    let Some(arg) = args.first() else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let text = self.base_digits_text(&self.evaluate(arg)?);
    convert_to_decimal(&text, base, 10)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  fn evaluate_base_to_base(
    &self,
    args: &[FormulaAst<'doc>],
    from_base: u32,
    to_base: u32,
    min: f64,
    max: f64,
  ) -> Option<FormulaValue<'doc>> {
    let Some(arg) = args.first() else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let text = self.base_digits_text(&self.evaluate(arg)?);
    let Some(value) = convert_to_decimal(&text, from_base, 10) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let places = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(|value| approx_floor(value) as i32);
    convert_from_decimal(value, min, max, to_base, places, 10)
      .map(|value| FormulaValue::String(Cow::Owned(value)))
      .or(Some(FormulaValue::Error(
        FormulaErrorValue::IllegalArgument,
      )))
  }

  fn evaluate_decimal_to_base(
    &self,
    args: &[FormulaAst<'doc>],
    base: u32,
    min: f64,
    max: f64,
  ) -> Option<FormulaValue<'doc>> {
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

  fn base_digits_text(&self, value: &FormulaValue<'doc>) -> String {
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

  fn evaluate_roundsig(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let digits = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    if digits < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if value == 0.0 {
      return Some(FormulaValue::Number(0.0));
    }
    Some(FormulaValue::Number(round_significant(value, digits)))
  }

  fn evaluate_raw_subtract(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut result = self.number(&self.evaluate(args.first()?)?)?;
    for arg in &args[1..] {
      result -= self.number(&self.evaluate(arg)?)?;
    }
    Some(FormulaValue::Number(result))
  }

  fn evaluate_sumproduct(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let matrices = args
      .iter()
      .map(|arg| {
        self
          .evaluate(arg)
          .map(|value| self.matrix_values(&value))
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

  fn evaluate_findb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
    let haystack_len = text_byte_len(&haystack);
    let needle_len = text_byte_len(&needle);
    if start == 0 || start > haystack_len.saturating_sub(needle_len) + 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let tail = rightb(&haystack, haystack_len - start + 1);
    tail
      .find(&needle)
      .map(|offset| FormulaValue::Number((start + text_byte_len(&tail[..offset])) as f64))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  fn evaluate_substitute(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_replace(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_text_before_after(
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
    let delimiter = self.text(&delimiter_value);
    let instance = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as i32;
    if delimiter.is_empty() || instance == 0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let matches = text.match_indices(&delimiter).collect::<Vec<_>>();
    let position = if instance > 0 {
      matches.get(instance as usize - 1).map(|(index, _)| *index)
    } else {
      let Some(index) = matches.len().checked_sub(instance.unsigned_abs() as usize) else {
        return Some(FormulaValue::Error(FormulaErrorValue::NA));
      };
      matches.get(index).map(|(index, _)| *index)
    };
    let Some(position) = position else {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    };
    Some(FormulaValue::String(Cow::Owned(if after {
      text[position + delimiter.len()..].to_string()
    } else {
      text[..position].to_string()
    })))
  }

  fn evaluate_textsplit(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let text = self.text(&self.evaluate(args.first()?)?);
    let column_delimiter = self.text(&self.evaluate(args.get(1)?)?);
    if column_delimiter.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let row_delimiter = args.get(2).and_then(|arg| self.evaluate(arg)).map(|value| {
      let delimiter = self.text(&value);
      if delimiter.is_empty() {
        "\n".to_string()
      } else {
        delimiter
      }
    });
    let rows = row_delimiter
      .as_deref()
      .map(|delimiter| text.split(delimiter).collect::<Vec<_>>())
      .unwrap_or_else(|| vec![text.as_str()]);
    Some(FormulaValue::Matrix(
      rows
        .into_iter()
        .map(|row| {
          row
            .split(&column_delimiter)
            .map(|value| FormulaValue::String(Cow::Owned(value.to_string())))
            .collect()
        })
        .collect(),
    ))
  }

  fn evaluate_textjoin(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_width_conversion(
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

  fn evaluate_row_column(
    &self,
    args: &[FormulaAst<'doc>],
    column: bool,
  ) -> Option<FormulaValue<'doc>> {
    let address = if let Some(arg) = args.first() {
      let Some(value) = self.evaluate(arg) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      let Some(reference) = self.as_reference(&value) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      reference.range.start
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
    let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
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

  fn evaluate_info(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_type(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    Some(FormulaValue::Number(match self.first_value(&value) {
      FormulaValue::Number(_) => 1.0,
      FormulaValue::String(_) => 2.0,
      FormulaValue::Boolean(_) => 4.0,
      FormulaValue::Error(_) => 16.0,
      FormulaValue::Matrix(_) | FormulaValue::Reference(_) => 64.0,
      FormulaValue::Blank => 1.0,
    }))
  }

  fn evaluate_areas(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(value) = self.evaluate(args.first()?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    Some(match value {
      FormulaValue::Reference(_) | FormulaValue::Matrix(_) => FormulaValue::Number(1.0),
      _ => FormulaValue::Error(FormulaErrorValue::Value),
    })
  }

  fn evaluate_ifs_function(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 || !args.len().is_multiple_of(2) {
      return None;
    }
    for pair in args.chunks_exact(2) {
      if self.truthy(&self.evaluate(&pair[0])?) {
        return self.evaluate(&pair[1]);
      }
    }
    Some(FormulaValue::Error(FormulaErrorValue::NA))
  }

  fn evaluate_switch(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 {
      return None;
    }
    let selector = self.scalar_value(self.evaluate(args.first()?)?);
    let pairs_len = if args.len().is_multiple_of(2) {
      args.len() - 2
    } else {
      args.len() - 1
    };
    for pair in args[1..=pairs_len].chunks_exact(2) {
      let candidate = self.scalar_value(self.evaluate(&pair[0])?);
      let matches = match (&selector, &candidate) {
        (FormulaValue::String(left), FormulaValue::String(right)) => {
          left.eq_ignore_ascii_case(right)
        }
        _ => self.compare(&selector, &candidate, FormulaOperator::Equal),
      };
      if matches {
        return self.evaluate(&pair[1]);
      }
    }
    if args.len().is_multiple_of(2) {
      self.evaluate(args.last()?)
    } else {
      Some(FormulaValue::Error(FormulaErrorValue::NA))
    }
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

  fn evaluate_address(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as i32;
    let a1 = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    let sheet = args.get(4).and_then(|arg| self.evaluate(arg)).map(|value| {
      let sheet = self.text(&value);
      if sheet.is_empty() {
        String::new()
      } else if a1 {
        format!("{sheet}.")
      } else {
        format!("{sheet}!")
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
      format!("R{row}C{column}")
    };
    Some(FormulaValue::String(Cow::Owned(format!(
      "{}{reference}",
      sheet.unwrap_or_default()
    ))))
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
    let Some(serial) = self.number_arg(args, 0).map(|value| value.floor() as i32) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let (year, month, day) = date_from_serial(serial)?;
    Some(FormulaValue::Number(match part {
      DatePart::Year => year as f64,
      DatePart::Month => month as f64,
      DatePart::Day => day as f64,
    }))
  }

  fn evaluate_today(&self) -> Option<FormulaValue<'doc>> {
    if let Some(cell) = self.current_cell
      && let Some(value) = self
        .book
        .cells
        .get(&(self.current_sheet, cell))
        .and_then(|value| self.number(value))
    {
      return Some(FormulaValue::Number(value.floor()));
    }
    let unix_days = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .ok()
      .map(|duration| duration.as_secs() / 86_400)?;
    Some(FormulaValue::Number(
      date_serial(1970, 1, 1)? + unix_days as f64,
    ))
  }

  fn evaluate_days360(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_eomonth(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
    let (year, month, _) = date_from_serial(start)?;
    date_serial(year, month as i32 + months + 1, 0)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  fn evaluate_edate(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let start = self
      .date_number_from_value(&self.evaluate(args.first()?)?)?
      .floor() as i32;
    let months = self.number(&self.evaluate(args.get(1)?)?)?.floor() as i32;
    let (year, month, day) = date_from_serial(start)?;
    let target = date_serial(year, month as i32 + months, day as i32).or_else(|| {
      let last = last_day_of_month(year, month);
      date_serial(year, month as i32 + months, last as i32)
    });
    target
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  fn evaluate_is_leap_year(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(serial) = self.date_number_from_value(&self.evaluate(args.first()?)?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let serial = serial as i32;
    let (year, _, _) = date_from_serial(serial)?;
    Some(FormulaValue::Boolean(is_leap_year(year)))
  }

  fn evaluate_basis_o_datetime(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return None;
    }
    basis_o_datetime_text(self.number(&self.evaluate(args.first()?)?)?)
      .map(|value| FormulaValue::String(Cow::Owned(value)))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  fn evaluate_datedif(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
    let (start_year, start_month, start_day) = date_from_serial(start)?;
    let (end_year, end_month, end_day) = date_from_serial(end)?;
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
        let roll = date_serial(roll_year, roll_month, start_day as i32)?;
        Some(FormulaValue::Number(f64::from(end) - roll))
      }
      "ym" => Some(FormulaValue::Number(month_diff.rem_euclid(12) as f64)),
      "yd" => {
        let same_year_start =
          if end_month > start_month || (end_month == start_month && end_day >= start_day) {
            date_serial(end_year, start_month as i32, start_day as i32)?
          } else {
            date_serial(end_year - 1, start_month as i32, start_day as i32)?
          };
        Some(FormulaValue::Number(f64::from(end) - same_year_start))
      }
      _ => Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    }
  }

  fn evaluate_yearfrac(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_weeknum(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=2).contains(&args.len()) {
      return None;
    }
    let serial = self.date_number_from_value(&self.evaluate(args.first()?)?)? as i64;
    let mode = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as i32;
    let weekday = weekday_index_from_serial(serial) as i32;
    let year = date_from_serial(serial as i32)?.0;
    let jan1 = date_serial(year, 1, 1)? as i64;
    let jan1_weekday = weekday_index_from_serial(jan1) as i32;
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

  fn evaluate_iso_weeknum(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let serial = match self.date_number_from_value(&self.evaluate(args.first()?)?) {
      Some(value) => value as i32,
      None => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    };
    iso_weeknum_from_serial(serial)
      .map(|value| FormulaValue::Number(value as f64))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  fn evaluate_weeks(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
    let mode = match self.number(&self.evaluate(args.get(2)?)?) {
      Some(value) => value as i32,
      None => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    };
    match mode {
      0 => Some(FormulaValue::Number(((end - start) / 7) as f64)),
      1 => {
        let Some(start_week) = proleptic_gregorian_week_index_from_serial(start) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        let Some(end_week) = proleptic_gregorian_week_index_from_serial(end) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        Some(FormulaValue::Number((end_week - start_week) as f64))
      }
      _ => Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    }
  }

  fn evaluate_weeks_in_year(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let serial = match self.date_number_from_value(&self.evaluate(args.first()?)?) {
      Some(value) => value as i32,
      None => return Some(FormulaValue::Error(FormulaErrorValue::Value)),
    };
    weeks_in_year_from_serial(serial)
      .map(|value| FormulaValue::Number(value as f64))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  fn evaluate_weekday(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let serial = self
      .date_number_from_value(&self.evaluate(args.first()?)?)?
      .floor() as i64;
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
    let value = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or_default();
    let total_seconds = ((value.fract() * 86_400.0).round() as i64).rem_euclid(86_400);
    Some(FormulaValue::Number(match part {
      TimePart::Hour => (total_seconds / 3600) as f64,
      TimePart::Minute => ((total_seconds % 3600) / 60) as f64,
      TimePart::Second => (total_seconds % 60) as f64,
    }))
  }

  fn evaluate_indirect(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(value) = self.evaluate(args.first()?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let text = self.text(&value);
    self
      .resolve_reference(&text)
      .map(FormulaValue::Reference)
      .or_else(|| self.evaluate_name(&Cow::Owned(text)))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Ref)))
  }

  fn evaluate_index(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(value) = self.evaluate(args.first()?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    if let FormulaValue::Matrix(rows) = value {
      let row_offset = args
        .get(1)
        .and_then(|arg| self.evaluate(arg))
        .and_then(|value| self.number(&value))
        .unwrap_or(1.0)
        .max(1.0) as usize
        - 1;
      let column_offset = args
        .get(2)
        .and_then(|arg| self.evaluate(arg))
        .and_then(|value| self.number(&value))
        .unwrap_or(1.0)
        .max(1.0) as usize
        - 1;
      return rows
        .get(row_offset)
        .and_then(|row| row.get(column_offset))
        .cloned()
        .or(Some(FormulaValue::Error(FormulaErrorValue::Ref)));
    }
    let Some(reference) = self.as_reference(&value) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
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

  fn evaluate_lookup(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let lookup = self.scalar_value(self.evaluate(args.first()?)?);
    let data = self.evaluate(args.get(1)?)?;
    let data_matrix = self.matrix_values(&data);
    let result = if let Some(arg) = args.get(2) {
      Some(self.evaluate(arg)?)
    } else {
      None
    };
    let result_matrix = result.as_ref().map(|value| self.matrix_values(value));
    let (data_vector, data_vertical) = lookup_vector(&data_matrix)?;
    let Some(index) = lookup_approximate_index(self, &lookup, &data_vector) else {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    };

    if let Some(result_matrix) = result_matrix {
      let (result_vector, _) = lookup_vector(&result_matrix)?;
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

  fn evaluate_match(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let lookup = self.scalar_value(self.evaluate(args.first()?)?);
    let data = self.evaluate(args.get(1)?)?;
    let matrix = self.matrix_values(&data);
    let (vector, _) = lookup_vector(&matrix)?;
    let mode = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as i32;
    let index = match mode {
      0 => lookup_exact_index(self, &lookup, &vector),
      1 => lookup_approximate_index(self, &lookup, &vector),
      -1 => lookup_descending_index(self, &lookup, &vector),
      _ => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    };
    index
      .map(|index| FormulaValue::Number(index as f64 + 1.0))
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  fn evaluate_xmatch(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=4).contains(&args.len()) {
      return None;
    }
    let lookup = self.scalar_value(self.evaluate(args.first()?)?);
    let data = self.evaluate(args.get(1)?)?;
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
    let index = match (match_mode, search_mode) {
      (0, 1) => lookup_exact_index(self, &lookup, &vector),
      (0, -1) => vector
        .iter()
        .enumerate()
        .rev()
        .find(|(_, value)| self.compare(&lookup, value, FormulaOperator::Equal))
        .map(|(index, _)| index),
      (-1, 1) => lookup_approximate_index(self, &lookup, &vector),
      (1, 1) => lookup_descending_index(self, &lookup, &vector),
      _ => return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
    };
    index
      .map(|index| FormulaValue::Number(index as f64 + 1.0))
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  fn evaluate_vlookup(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(lookup_value) = self.evaluate(args.first()?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let lookup = self.text(&lookup_value);
    let Some(result_column) = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(|value| value.max(1.0) as u32)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let Some(table) = self.evaluate(args.get(1)?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
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

    let Some(reference) = self.as_reference(&table) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
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

  fn evaluate_hlookup(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let lookup = self.scalar_value(self.evaluate(args.first()?)?);
    let row_index = self.number(&self.evaluate(args.get(2)?)?)?.max(1.0) as usize - 1;
    let sorted = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    let matrix = self.matrix_values(&self.evaluate(args.get(1)?)?);
    let first_row = matrix.first()?;
    let index = if sorted {
      lookup_approximate_index(self, &lookup, first_row)
    } else {
      lookup_exact_index(self, &lookup, first_row)
    };
    let Some(index) = index else {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    };
    matrix
      .get(row_index)
      .and_then(|row| row.get(index))
      .cloned()
      .or(Some(FormulaValue::Error(FormulaErrorValue::Ref)))
  }

  fn evaluate_xlookup(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 {
      return None;
    }
    let lookup = self.scalar_value(self.evaluate(args.first()?)?);
    let lookup_matrix = self.matrix_values(&self.evaluate(args.get(1)?)?);
    let return_matrix = self.matrix_values(&self.evaluate(args.get(2)?)?);
    let (lookup_vector, lookup_vertical) = lookup_vector(&lookup_matrix)?;
    let not_found = args.get(3).and_then(|arg| self.evaluate(arg));
    let match_mode = args
      .get(4)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0) as i32;
    let search_mode = args
      .get(5)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as i32;
    let index = match match_mode {
      0 if search_mode == -1 => lookup_exact_index_reverse(self, &lookup, &lookup_vector),
      0 => lookup_exact_index(self, &lookup, &lookup_vector),
      -1 => lookup_approximate_index(self, &lookup, &lookup_vector),
      1 => lookup_next_larger_index(self, &lookup, &lookup_vector),
      _ if search_mode == -1 => lookup_exact_index_reverse(self, &lookup, &lookup_vector),
      _ => lookup_exact_index(self, &lookup, &lookup_vector),
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

  fn evaluate_sheets(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if let Some(arg) = args.first() {
      let Some(value) = self.evaluate(arg) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      return match value {
        FormulaValue::Reference(_) => Some(FormulaValue::Number(1.0)),
        FormulaValue::Matrix(_) => Some(FormulaValue::Number(1.0)),
        _ => Some(FormulaValue::Error(FormulaErrorValue::Value)),
      };
    }
    let mut sheets = self
      .book
      .sheet_names
      .iter()
      .map(|sheet| sheet.id)
      .collect::<Vec<_>>();
    sheets.extend(self.book.cells.keys().map(|(sheet, _)| *sheet));
    sheets.extend(self.book.formulas.keys().map(|(sheet, _)| *sheet));
    sheets.extend(self.book.row_states.keys().map(|(sheet, _)| *sheet));
    sheets.extend(self.book.tables.values().map(|table| table.sheet));
    sheets.sort_unstable();
    sheets.dedup();
    Some(FormulaValue::Number(sheets.len().max(1) as f64))
  }

  fn evaluate_sheet(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let sheet = if let Some(arg) = args.first() {
      let Some(value) = self.evaluate(arg) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      match value {
        FormulaValue::Reference(reference) => self.range_sheet(&reference),
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

  fn evaluate_formula_text(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let reference = self.as_reference(&self.evaluate(args.first()?)?)?;
    let sheet = self.range_sheet(&reference);
    self
      .book
      .formula_text(sheet, reference.range.start)
      .map(|text| FormulaValue::String(Cow::Owned(text)))
      .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
  }

  fn evaluate_and(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_or(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_xor(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_midb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_leftb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let text = self.text(&self.evaluate(args.first()?)?);
    let len = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0);
    if len < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::String(Cow::Owned(leftb(
      &text,
      len.floor() as usize,
    ))))
  }

  fn evaluate_right(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let text = self.text(&self.evaluate(args.first()?)?);
    let len = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0);
    if len < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let len = len.floor() as usize;
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

  fn evaluate_rightb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let text = self.text(&self.evaluate(args.first()?)?);
    let len = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0);
    if len < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::String(Cow::Owned(rightb(
      &text,
      len.floor() as usize,
    ))))
  }

  fn evaluate_roman(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=2).contains(&args.len()) {
      return None;
    }
    let value = self.number(&self.evaluate(args.first()?)?)?.floor() as i32;
    if !(0..=3999).contains(&value) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut number = value;
    let mut output = String::new();
    for (arabic, roman) in [
      (1000, "M"),
      (900, "CM"),
      (500, "D"),
      (400, "CD"),
      (100, "C"),
      (90, "XC"),
      (50, "L"),
      (40, "XL"),
      (10, "X"),
      (9, "IX"),
      (5, "V"),
      (4, "IV"),
      (1, "I"),
    ] {
      while number >= arabic {
        output.push_str(roman);
        number -= arabic;
      }
    }
    Some(FormulaValue::String(Cow::Owned(output)))
  }

  fn evaluate_arabic(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_replaceb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_hyperlink(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=2).contains(&args.len()) {
      return None;
    }
    if let Some(display) = args.get(1).and_then(|arg| self.evaluate(arg)) {
      Some(self.scalar_value(display))
    } else {
      Some(FormulaValue::String(Cow::Owned(
        self.text(&self.evaluate(args.first()?)?),
      )))
    }
  }

  fn evaluate_to_row_column(
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

  fn evaluate_choose_rows(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(reference) = self.as_reference(&value) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let mut rows = Vec::new();
    for arg in args.iter().skip(1) {
      let Some(index) = self
        .evaluate(arg)
        .and_then(|value| self.number(&value))
        .map(|value| value as u32)
      else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      let row = reference.range.start.row + index.saturating_sub(1);
      let mut values = Vec::new();
      for column in reference.range.start.column..=reference.range.end.column {
        values.push(self.reference_cell_value(&reference, CellAddress { column, row }));
      }
      rows.push(values);
    }
    Some(FormulaValue::Matrix(rows))
  }

  fn evaluate_choose_cols(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(reference) = self.as_reference(&value) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let mut values = Vec::new();
    for row in reference.range.start.row..=reference.range.end.row {
      let mut out = Vec::new();
      for arg in args.iter().skip(1) {
        let Some(index) = self
          .evaluate(arg)
          .and_then(|value| self.number(&value))
          .map(|value| value as u32)
        else {
          return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
        };
        let column = reference.range.start.column + index.saturating_sub(1);
        out.push(self.reference_cell_value(&reference, CellAddress { column, row }));
      }
      values.push(out);
    }
    Some(FormulaValue::Matrix(values))
  }

  fn evaluate_expand(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_transpose(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_sequence(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.is_empty() || args.len() > 4 {
      return None;
    }
    let rows = self.number(&self.evaluate(args.first()?)?)?.floor() as usize;
    let columns = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0)
      .floor() as usize;
    let start = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0);
    let step = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
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

  fn evaluate_take_drop(
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
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(|value| value as isize);
    let cols_arg = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
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

  fn evaluate_sort(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(1..=4).contains(&args.len()) {
      return None;
    }
    let data = self.matrix_values(&self.evaluate(args.first()?)?);
    if data.is_empty() || data.first().is_none_or(Vec::is_empty) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let sort_index = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0);
    let sort_order = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0);
    let by_col = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(false);
    if sort_index < 1.0 || !matches!(sort_order, 1.0 | -1.0) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let key = sort_index as usize - 1;
    let ascending = sort_order == 1.0;
    if by_col {
      if key >= data.len() {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      let mut order = (0..data[0].len()).collect::<Vec<_>>();
      order
        .sort_by(|left, right| sort_value_order(&data[key][*left], &data[key][*right], ascending));
      Some(FormulaValue::Matrix(reorder_columns(&data, &order)))
    } else {
      if key >= data[0].len() {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      let mut order = (0..data.len()).collect::<Vec<_>>();
      order
        .sort_by(|left, right| sort_value_order(&data[*left][key], &data[*right][key], ascending));
      Some(FormulaValue::Matrix(
        order.into_iter().map(|row| data[row].clone()).collect(),
      ))
    }
  }

  fn evaluate_sortby(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
      order.sort_by(|left, right| sort_multi_key_order(&keys, *left, *right));
      Some(FormulaValue::Matrix(
        order.into_iter().map(|row| data[row].clone()).collect(),
      ))
    } else {
      if keys.iter().any(|(values, _)| values.len() != data[0].len()) {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      let mut order = (0..data[0].len()).collect::<Vec<_>>();
      order.sort_by(|left, right| sort_multi_key_order(&keys, *left, *right));
      Some(FormulaValue::Matrix(reorder_columns(&data, &order)))
    }
  }

  fn evaluate_mmult(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
    let right_columns = (0..columns)
      .map(|column| right.iter().map(|row| &row[column]).collect::<Vec<_>>())
      .collect::<Vec<_>>();
    let mut result = Vec::with_capacity(rows);
    for left_row in &left {
      let mut result_row = Vec::with_capacity(columns);
      for right_column in &right_columns {
        let mut total = 0.0;
        for (left_value, right_value) in left_row.iter().zip(right_column) {
          total += self.number(left_value).unwrap_or(0.0) * self.number(right_value).unwrap_or(0.0);
        }
        result_row.push(FormulaValue::Number(total));
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  fn evaluate_mdeterm(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
        let Some(number) = self.number(&value) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        out.push(number);
      }
      values.push(out);
    }
    Some(FormulaValue::Number(matrix_determinant(values)))
  }

  fn evaluate_munit(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_varpa(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = self
      .values(args)
      .map(|value| self.number(&value).unwrap_or(0.0))
      .collect::<Vec<_>>();
    variance_slice(&values, false)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown)))
  }

  fn evaluate_vara(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = self
      .values(args)
      .map(|value| self.number(&value).unwrap_or(0.0))
      .collect::<Vec<_>>();
    variance_slice(&values, true)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown)))
  }

  fn evaluate_mina(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    self
      .values(args)
      .map(|value| self.number(&value).unwrap_or(0.0))
      .reduce(f64::min)
      .map(FormulaValue::Number)
  }

  fn evaluate_maxa(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    self
      .values(args)
      .map(|value| self.number(&value).unwrap_or(0.0))
      .reduce(f64::max)
      .map(FormulaValue::Number)
  }

  fn evaluate_trimmean(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let mut values = self.value_numbers(&self.evaluate(args.first()?)?);
    let alpha = self.number(&self.evaluate(args.get(1)?)?)?;
    if !(0.0..1.0).contains(&alpha) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if values.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    values.sort_by(|left, right| left.total_cmp(right));
    let mut trim = approx_floor(alpha * values.len() as f64) as usize;
    if !trim.is_multiple_of(2) {
      trim -= 1;
    }
    trim /= 2;
    if trim * 2 >= values.len() {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let kept = &values[trim..values.len() - trim];
    Some(FormulaValue::Number(
      kept.iter().sum::<f64>() / kept.len() as f64,
    ))
  }

  fn evaluate_large_small(
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
    let Some(k) = self.number_arg(args, 1).map(|value| value.floor() as usize) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if k == 0 || k > values.len() {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    values.sort_by(f64::total_cmp);
    let index = if large { values.len() - k } else { k - 1 };
    Some(FormulaValue::Number(values[index]))
  }

  fn evaluate_stdeva(&self, args: &[FormulaAst<'doc>], sample: bool) -> Option<FormulaValue<'doc>> {
    let values = self
      .values(args)
      .map(|value| self.number(&value).unwrap_or(0.0))
      .collect::<Vec<_>>();
    variance_slice(&values, sample)
      .map(|value| FormulaValue::Number(value.sqrt()))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Div0)))
  }

  fn evaluate_devsq(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = self.numeric_args(args);
    if values.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    Some(FormulaValue::Number(
      values
        .iter()
        .map(|value| {
          let delta = value - mean;
          delta * delta
        })
        .sum(),
    ))
  }

  fn evaluate_avedev(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = self.numeric_args(args);
    if values.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    Some(FormulaValue::Number(
      values.iter().map(|value| (value - mean).abs()).sum::<f64>() / values.len() as f64,
    ))
  }

  fn evaluate_averagea(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_gcd_lcm(&self, args: &[FormulaAst<'doc>], lcm: bool) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_fact(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_fact_double(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(value) = self.number_arg(args, 0).map(f64::floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if value < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let mut result = 1.0;
    let mut current = value as u64;
    while current > 1 {
      result *= current as f64;
      current = current.saturating_sub(2);
    }
    Some(FormulaValue::Number(result))
  }

  fn evaluate_multinomial(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_combin(
    &self,
    args: &[FormulaAst<'doc>],
    repetition: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(mut count) = self.number_arg(args, 0).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(chosen) = self.number_arg(args, 1).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if count < 0.0 || chosen < 0.0 || (!repetition && chosen > count) {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    if repetition {
      if count == 0.0 && chosen > 0.0 {
        return Some(FormulaValue::Error(FormulaErrorValue::Num));
      }
      count += chosen - 1.0;
    }
    Some(FormulaValue::Number(
      (log_gamma(count + 1.0) - log_gamma(chosen + 1.0) - log_gamma(count - chosen + 1.0))
        .exp()
        .round(),
    ))
  }

  fn evaluate_permut(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let count = self.number(&self.evaluate(args.first()?)?)?.floor();
    let chosen = self.number(&self.evaluate(args.get(1)?)?)?.floor();
    if count < 0.0 || chosen < 0.0 || chosen > count {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(
      (log_gamma(count + 1.0) - log_gamma(count - chosen + 1.0))
        .exp()
        .round(),
    ))
  }

  fn evaluate_permutationa(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_mround(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(number) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(multiple) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if multiple == 0.0 {
      return Some(FormulaValue::Number(0.0));
    }
    Some(FormulaValue::Number((number / multiple).round() * multiple))
  }

  fn evaluate_quotient(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let numerator = self.number(&self.evaluate(args.first()?)?)?;
    let denominator = self.number(&self.evaluate(args.get(1)?)?)?;
    if denominator == 0.0 {
      Some(FormulaValue::Error(FormulaErrorValue::Div0))
    } else {
      Some(FormulaValue::Number((numerator / denominator).trunc()))
    }
  }

  fn evaluate_pmt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
    Some(FormulaValue::Number(financial_pmt(
      rate,
      nper,
      pv,
      fv,
      pay_in_advance,
    )))
  }

  fn evaluate_fv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
    Some(FormulaValue::Number(-financial_fv(
      rate,
      nper,
      payment,
      pv,
      pay_in_advance,
    )))
  }

  fn evaluate_npv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(rate) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let mut period = 1i32;
    let mut result = 0.0;
    for value in self.values(&args[1..]) {
      if let Some(number) = self.number(&value) {
        result += number / (1.0 + rate).powi(period);
        period += 1;
      }
    }
    Some(FormulaValue::Number(result))
  }

  fn evaluate_fvschedule(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let mut value = self.number(&self.evaluate(args.first()?)?)?;
    for rate in self.value_numbers(&self.evaluate(args.get(1)?)?) {
      value *= 1.0 + rate;
    }
    Some(FormulaValue::Number(value))
  }

  fn evaluate_effect(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_rate(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_ispmt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 4 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let rate = self.number(&self.evaluate(args.first()?)?)?;
    let period = self.number(&self.evaluate(args.get(1)?)?)?;
    let total = self.number(&self.evaluate(args.get(2)?)?)?;
    let investment = self.number(&self.evaluate(args.get(3)?)?)?;
    Some(FormulaValue::Number(
      investment * rate * (period / total - 1.0),
    ))
  }

  fn evaluate_ipmt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_ppmt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_cumipmt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    self.evaluate_cum_interest_principal(args, true)
  }

  fn evaluate_cumprinc(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    self.evaluate_cum_interest_principal(args, false)
  }

  fn evaluate_cum_interest_principal(
    &self,
    args: &[FormulaAst<'doc>],
    interest: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 6 {
      return None;
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

  fn evaluate_ddb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_vdb(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_skew(
    &self,
    args: &[FormulaAst<'doc>],
    population: bool,
  ) -> Option<FormulaValue<'doc>> {
    let values = self.numeric_args(args);
    let count = values.len() as f64;
    if count < 3.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    let mean = values.iter().sum::<f64>() / count;
    let variance_sum = values
      .iter()
      .map(|value| {
        let diff = value - mean;
        diff * diff
      })
      .sum::<f64>();
    let std_dev = (variance_sum / if population { count } else { count - 1.0 }).sqrt();
    if std_dev == 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let cube_sum = values
      .iter()
      .map(|value| {
        let normalized = (value - mean) / std_dev;
        normalized * normalized * normalized
      })
      .sum::<f64>();
    Some(FormulaValue::Number(if population {
      cube_sum / count
    } else {
      cube_sum * count / (count - 1.0) / (count - 2.0)
    }))
  }

  fn evaluate_geo_har_mean(
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
    if harmonic {
      Some(FormulaValue::Number(count / total))
    } else {
      Some(FormulaValue::Number((total / count).exp()))
    }
  }

  fn evaluate_sumif(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    if let Some(sum_range) = args.get(2) {
      self.evaluate_ifs(Some(sum_range), &args[..2], IfsAggregate::Sum)
    } else {
      self.evaluate_ifs(Some(&args[0]), &args[..2], IfsAggregate::Sum)
    }
  }

  fn evaluate_countif(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    self.evaluate_ifs(None, args, IfsAggregate::Count)
  }

  fn evaluate_averageif(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    if let Some(average_range) = args.get(2) {
      self.evaluate_ifs(Some(average_range), &args[..2], IfsAggregate::Average)
    } else {
      self.evaluate_ifs(Some(&args[0]), &args[..2], IfsAggregate::Average)
    }
  }

  fn evaluate_sumifs(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 || args.len().is_multiple_of(2) {
      return None;
    }
    self.evaluate_ifs(Some(&args[0]), &args[1..], IfsAggregate::Sum)
  }

  fn evaluate_countifs(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 2 || !args.len().is_multiple_of(2) {
      return None;
    }
    self.evaluate_ifs(None, args, IfsAggregate::Count)
  }

  fn evaluate_averageifs(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 || args.len().is_multiple_of(2) {
      return None;
    }
    self.evaluate_ifs(Some(&args[0]), &args[1..], IfsAggregate::Average)
  }

  fn evaluate_minmaxifs(&self, args: &[FormulaAst<'doc>], max: bool) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_database_function(
    &self,
    args: &[FormulaAst<'doc>],
    function: DatabaseFunction,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let Some(database_arg) = self.evaluate(args.first()?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(criteria_arg) = self.evaluate(args.get(2)?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let database = self.matrix_values(&database_arg);
    let criteria = self.matrix_values(&criteria_arg);
    if database.len() < 2
      || database.first().is_none_or(Vec::is_empty)
      || criteria.len() < 2
      || criteria.first().is_none_or(Vec::is_empty)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let field = match self.database_field_index(args.get(1)?, &database[0], function) {
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
    if field >= database[0].len() {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }

    let mut values = Vec::new();
    let mut text_values = Vec::new();
    for row in rows {
      let value = row.get(field).cloned().unwrap_or_default();
      match function {
        DatabaseFunction::Count => {
          if self.number(&value).is_some() {
            values.push(1.0);
          }
        }
        DatabaseFunction::CountA => {
          if !matches!(value, FormulaValue::Blank) {
            values.push(1.0);
          }
        }
        DatabaseFunction::Get => text_values.push(value),
        _ => {
          if let Some(number) = self.number(&value) {
            values.push(number);
          }
        }
      }
    }

    match function {
      DatabaseFunction::Count | DatabaseFunction::CountA => {
        Some(FormulaValue::Number(values.len() as f64))
      }
      DatabaseFunction::Sum => Some(FormulaValue::Number(values.iter().sum())),
      DatabaseFunction::Average if values.is_empty() => {
        Some(FormulaValue::Error(FormulaErrorValue::Div0))
      }
      DatabaseFunction::Average => Some(FormulaValue::Number(
        values.iter().sum::<f64>() / values.len() as f64,
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

  fn database_field_index(
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
      FormulaValue::Reference(_) | FormulaValue::Matrix(_) => {
        if allow_missing {
          Some(None)
        } else {
          None
        }
      }
      FormulaValue::Boolean(_) | FormulaValue::Error(_) | FormulaValue::Blank => None,
    }
  }

  fn database_matching_rows<'b>(
    &self,
    database: &'b [Vec<FormulaValue<'doc>>],
    criteria: &[Vec<FormulaValue<'doc>>],
  ) -> Vec<&'b [FormulaValue<'doc>]> {
    let headers = &database[0];
    let criteria_headers = &criteria[0];
    database[1..]
      .iter()
      .filter(|row| {
        criteria[1..].iter().any(|criteria_row| {
          criteria_row
            .iter()
            .enumerate()
            .filter(|(_, value)| !matches!(value, FormulaValue::Blank))
            .all(|(criteria_column, criterion_value)| {
              let Some(header) = criteria_headers.get(criteria_column) else {
                return true;
              };
              let header = self.text(header);
              if header.is_empty() {
                return true;
              }
              let Some(field_column) = headers.iter().position(|database_header| {
                self.text(database_header).eq_ignore_ascii_case(&header)
              }) else {
                return false;
              };
              let candidate = row.get(field_column).cloned().unwrap_or_default();
              FormulaCriteria::from_value(self, criterion_value).matches(self, &candidate)
            })
        })
      })
      .map(Vec::as_slice)
      .collect()
  }

  fn evaluate_ifs(
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
      let range = self.matrix_values(&self.evaluate(&pair[0])?);
      let rows = range.len();
      let columns = range.first().map_or(0, Vec::len);
      if rows == 0 || columns == 0 || range.iter().any(|row| row.len() != columns) {
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
          .map(|value| FormulaCriteria::from_value(self, &value))
          .collect::<Vec<_>>(),
      );
    }

    if criteria_ranges
      .windows(2)
      .any(|window| matrix_dimensions(&window[0]) != matrix_dimensions(&window[1]))
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let dimensions = matrix_dimensions(criteria_ranges.first()?);
    let main_values = if let Some(main_range) = main_range {
      let values = self.matrix_values(&self.evaluate(main_range)?);
      if matrix_dimensions(&values) != dimensions {
        return Some(FormulaValue::Error(FormulaErrorValue::Value));
      }
      Some(values)
    } else {
      None
    };

    let mut outputs = Vec::with_capacity(result_len);
    for criteria_index in 0..result_len {
      let mut count = 0.0;
      let mut sum = 0.0;
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
                criteria.matches(self, &range[row][column])
              });
          if !matches_all {
            continue;
          }
          match aggregate {
            IfsAggregate::Count => count += 1.0,
            IfsAggregate::Sum | IfsAggregate::Average | IfsAggregate::Min | IfsAggregate::Max => {
              if let Some(number) = main_values
                .as_ref()
                .and_then(|values| self.number(&values[row][column]))
              {
                count += 1.0;
                sum += number;
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
        IfsAggregate::Sum => FormulaValue::Number(sum),
        IfsAggregate::Average if count == 0.0 => FormulaValue::Error(FormulaErrorValue::Div0),
        IfsAggregate::Average => FormulaValue::Number(sum / count),
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
    percentile_sorted(&mut values, k, kind)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Num)))
  }

  fn evaluate_quartile(
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
    percentile_sorted(&mut values, quartile / 4.0, kind)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Num)))
  }

  fn evaluate_rank(&self, args: &[FormulaAst<'doc>], average: bool) -> Option<FormulaValue<'doc>> {
    let Some(value) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(range_value) = args.get(1).and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let mut values = self.value_numbers(&range_value);
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

  fn evaluate_kurt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let values = self.numeric_args(args);
    let n = values.len();
    if n < 4 {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mean = values.iter().sum::<f64>() / n as f64;
    let mut second = 0.0;
    let mut fourth = 0.0;
    for value in &values {
      let diff = value - mean;
      let square = diff * diff;
      second += square;
      fourth += square * square;
    }
    if second == 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    let n = n as f64;
    let sample_variance = second / (n - 1.0);
    let sample_fourth = fourth / (sample_variance * sample_variance);
    Some(FormulaValue::Number(
      n * (n + 1.0) * sample_fourth / ((n - 1.0) * (n - 2.0) * (n - 3.0))
        - 3.0 * (n - 1.0) * (n - 1.0) / ((n - 2.0) * (n - 3.0)),
    ))
  }

  fn evaluate_beta_dist(
    &self,
    args: &[FormulaAst<'doc>],
    legacy: bool,
  ) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let alpha = self.number(&self.evaluate(args.get(1)?)?)?;
    let beta = self.number(&self.evaluate(args.get(2)?)?)?;
    let (cumulative, lower, upper) = if legacy {
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
      let cumulative = args
        .get(5)
        .and_then(|arg| self.evaluate(arg))
        .map(|value| self.truthy(&value))
        .unwrap_or(true);
      (cumulative, lower, upper)
    } else {
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
      (cumulative, lower, upper)
    };
    if alpha <= 0.0 || beta <= 0.0 || upper <= lower {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    if x < lower {
      return Some(if legacy && cumulative {
        FormulaValue::Number(0.0)
      } else {
        FormulaValue::Error(FormulaErrorValue::Num)
      });
    }
    if x > upper {
      return Some(if legacy && cumulative {
        FormulaValue::Number(1.0)
      } else {
        FormulaValue::Error(FormulaErrorValue::Num)
      });
    }
    let scaled = (x - lower) / (upper - lower);
    let dist = Beta::new(alpha, beta).ok()?;
    Some(FormulaValue::Number(if cumulative {
      dist.cdf(scaled)
    } else {
      dist.pdf(scaled) / (upper - lower)
    }))
  }

  fn evaluate_erf(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_delta(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(left) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let right = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    Some(FormulaValue::Number((left == right) as u8 as f64))
  }

  fn evaluate_gestep(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(value) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let step = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0);
    Some(FormulaValue::Number((value >= step) as u8 as f64))
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
    if p == 0.0 {
      return Some(FormulaValue::Number(lower));
    }
    if p == 1.0 {
      return Some(FormulaValue::Number(upper));
    }
    let dist = Beta::new(alpha, beta).ok()?;
    Some(FormulaValue::Number(
      lower + dist.inverse_cdf(p) * (upper - lower),
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
    let dist = Binomial::new(p, n as u64).ok()?;
    Some(FormulaValue::Number(if cumulative {
      dist.cdf(x as u64)
    } else {
      dist.pmf(x as u64)
    }))
  }

  fn evaluate_binom_dist_range(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=4).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(n) = self.number_arg(args, 0).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(p) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(start) = self.number_arg(args, 2).map(approx_floor) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let end = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .map(|value| value.floor())
      .unwrap_or(start);
    if start < 0.0 || start > end || end > n {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    if p == 0.0 {
      return Some(FormulaValue::Number(if start == 0.0 { 1.0 } else { 0.0 }));
    }
    if p == 1.0 {
      return Some(FormulaValue::Number(if end == n { 1.0 } else { 0.0 }));
    }
    if !(0.0..=1.0).contains(&p) || n < 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let dist = Binomial::new(p, n as u64).ok()?;
    Some(FormulaValue::Number(
      (start as u64..=end as u64)
        .map(|k| dist.pmf(k))
        .sum::<f64>()
        .min(1.0),
    ))
  }

  fn evaluate_binom_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let n = self.number(&self.evaluate(args.first()?)?)?.floor();
    let p = self.number(&self.evaluate(args.get(1)?)?)?;
    let alpha = self.number(&self.evaluate(args.get(2)?)?)?;
    if n < 0.0 || !(0.0..=1.0).contains(&p) || !(0.0..=1.0).contains(&alpha) {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let dist = Binomial::new(p, n as u64).ok()?;
    let mut cumulative = 0.0;
    for k in 0..=n as u64 {
      cumulative += dist.pmf(k);
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
    let dist = ChiSquared::new(df).ok()?;
    if right_tail {
      return Some(FormulaValue::Number(dist.sf(x)));
    }
    let cumulative = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    Some(FormulaValue::Number(if cumulative {
      dist.cdf(x)
    } else {
      dist.pdf(x)
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
    let dist = ChiSquared::new(df).ok()?;
    Some(FormulaValue::Number(dist.inverse_cdf(if right_tail {
      1.0 - p
    } else {
      p
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
    Some(FormulaValue::Number(ChiSquared::new(df).ok()?.sf(chi)))
  }

  fn evaluate_confidence_norm(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(alpha) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(sigma) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(size) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if !(0.0..1.0).contains(&alpha) || sigma <= 0.0 || size < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(
      norm_s_inv(1.0 - alpha / 2.0).abs() * sigma / size.sqrt(),
    ))
  }

  fn evaluate_confidence_t(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(alpha) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(sigma) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(size) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if !(0.0..1.0).contains(&alpha) || sigma <= 0.0 || size < 2.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let dist = StudentsT::new(0.0, 1.0, size - 1.0).ok()?;
    Some(FormulaValue::Number(
      dist.inverse_cdf(1.0 - alpha / 2.0).abs() * sigma / size.sqrt(),
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

  fn evaluate_correl(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let left = self.value_numbers(&self.evaluate(args.first()?)?);
    let right = self.value_numbers(&self.evaluate(args.get(1)?)?);
    let count = left.len().min(right.len());
    if count < 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    let left_mean = left.iter().take(count).sum::<f64>() / count as f64;
    let right_mean = right.iter().take(count).sum::<f64>() / count as f64;
    let mut numerator = 0.0;
    let mut left_sum = 0.0;
    let mut right_sum = 0.0;
    for index in 0..count {
      let left_delta = left[index] - left_mean;
      let right_delta = right[index] - right_mean;
      numerator += left_delta * right_delta;
      left_sum += left_delta * left_delta;
      right_sum += right_delta * right_delta;
    }
    if left_sum == 0.0 || right_sum == 0.0 {
      Some(FormulaValue::Error(FormulaErrorValue::Div0))
    } else {
      Some(FormulaValue::Number(
        numerator / (left_sum * right_sum).sqrt(),
      ))
    }
  }

  fn evaluate_slope(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let y_values = self.value_numbers(&self.evaluate(args.first()?)?);
    let x_values = self.value_numbers(&self.evaluate(args.get(1)?)?);
    let count = y_values.len().min(x_values.len());
    if count < 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Div0));
    }
    let x_mean = x_values.iter().take(count).sum::<f64>() / count as f64;
    let y_mean = y_values.iter().take(count).sum::<f64>() / count as f64;
    let mut numerator = 0.0;
    let mut denominator = 0.0;
    for index in 0..count {
      let x_delta = x_values[index] - x_mean;
      numerator += x_delta * (y_values[index] - y_mean);
      denominator += x_delta * x_delta;
    }
    if denominator == 0.0 {
      Some(FormulaValue::Error(FormulaErrorValue::Div0))
    } else {
      Some(FormulaValue::Number(numerator / denominator))
    }
  }

  fn evaluate_expon_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_f_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let df1 = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    let df2 = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?);
    let cumulative = self.truthy(&self.evaluate(args.get(3)?)?);
    if x < 0.0 || df1 <= 0.0 || df2 <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let dist = FisherSnedecor::new(df1, df2).ok()?;
    Some(FormulaValue::Number(if cumulative {
      dist.cdf(x)
    } else {
      dist.pdf(x)
    }))
  }

  fn evaluate_f_dist_rt(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let df1 = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?);
    let df2 = approx_floor(self.number(&self.evaluate(args.get(2)?)?)?);
    if x < 0.0 || df1 <= 0.0 || df2 <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(
      FisherSnedecor::new(df1, df2).ok()?.sf(x),
    ))
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
    Some(FormulaValue::Number(
      FisherSnedecor::new(df1, df2)
        .ok()?
        .inverse_cdf(if right_tail { 1.0 - p } else { p }),
    ))
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
    let cdf = FisherSnedecor::new(df1, df2).ok()?.cdf(f);
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
    let dist = Gamma::new(alpha, 1.0 / beta).ok()?;
    Some(FormulaValue::Number(if cumulative {
      dist.cdf(x)
    } else {
      dist.pdf(x)
    }))
  }

  fn evaluate_gamma_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let p = self.number(&self.evaluate(args.first()?)?)?;
    let alpha = self.number(&self.evaluate(args.get(1)?)?)?;
    let beta = self.number(&self.evaluate(args.get(2)?)?)?;
    if !(0.0..=1.0).contains(&p) || alpha <= 0.0 || beta <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(
      Gamma::new(alpha, 1.0 / beta).ok()?.inverse_cdf(p),
    ))
  }

  fn evaluate_gamma(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.number(&self.evaluate(args.first()?)?)?;
    if value == 0.0 || (value < 0.0 && value.fract() == 0.0) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(gamma(value)))
  }

  fn evaluate_hypgeom_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
      || sample_size > population_size
      || population_success > population_size
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
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

  fn evaluate_lognorm_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?;
    let mean = self.number(&self.evaluate(args.get(1)?)?)?;
    let sigma = self.number(&self.evaluate(args.get(2)?)?)?;
    let cumulative = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    if x <= 0.0 || sigma <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let dist = LogNormal::new(mean, sigma).ok()?;
    Some(FormulaValue::Number(if cumulative {
      dist.cdf(x)
    } else {
      dist.pdf(x)
    }))
  }

  fn evaluate_lognorm_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(p) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(mean) = self.number_arg(args, 1) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(sigma) = self.number_arg(args, 2) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    if !(0.0..1.0).contains(&p) || sigma <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(
      LogNormal::new(mean, sigma).ok()?.inverse_cdf(p),
    ))
  }

  fn evaluate_negbinom_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let failures = self.number(&self.evaluate(args.first()?)?)?.floor();
    let successes = self.number(&self.evaluate(args.get(1)?)?)?.floor();
    let p = self.number(&self.evaluate(args.get(2)?)?)?;
    let cumulative = args
      .get(3)
      .and_then(|arg| self.evaluate(arg))
      .is_some_and(|value| self.truthy(&value));
    if failures < 0.0 || successes < 1.0 || !(0.0..=1.0).contains(&p) {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let dist = NegativeBinomial::new(successes, p).ok()?;
    Some(FormulaValue::Number(if cumulative {
      dist.cdf(failures as u64)
    } else {
      dist.pmf(failures as u64)
    }))
  }

  fn evaluate_norm_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
    let dist = Normal::new(mean, sigma).ok()?;
    Some(FormulaValue::Number(if cumulative {
      dist.cdf(x)
    } else {
      dist.pdf(x)
    }))
  }

  fn evaluate_norm_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let p = self.number(&self.evaluate(args.first()?)?)?;
    let mean = self.number(&self.evaluate(args.get(1)?)?)?;
    let sigma = self.number(&self.evaluate(args.get(2)?)?)?;
    if !(0.0..1.0).contains(&p) || sigma <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    Some(FormulaValue::Number(
      Normal::new(mean, sigma).ok()?.inverse_cdf(p),
    ))
  }

  fn evaluate_norm_s_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let z = self.number(&self.evaluate(args.first()?)?)?;
    let cumulative = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    let dist = Normal::new(0.0, 1.0).ok()?;
    Some(FormulaValue::Number(if cumulative {
      dist.cdf(z)
    } else {
      dist.pdf(z)
    }))
  }

  fn evaluate_percent_rank(
    &self,
    args: &[FormulaAst<'doc>],
    kind: PercentileKind,
  ) -> Option<FormulaValue<'doc>> {
    let mut values = self.value_numbers(&self.evaluate(args.first()?)?);
    values.sort_by(f64::total_cmp);
    let x = self.number(&self.evaluate(args.get(1)?)?)?;
    if values.is_empty() || x < *values.first()? || x > *values.last()? {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    }
    let denominator = match kind {
      PercentileKind::Inc => values.len() as f64 - 1.0,
      PercentileKind::Exc => values.len() as f64 + 1.0,
    };
    let offset = match kind {
      PercentileKind::Inc => 0.0,
      PercentileKind::Exc => 1.0,
    };
    for (index, value) in values.iter().enumerate() {
      if *value == x {
        return Some(FormulaValue::Number((index as f64 + offset) / denominator));
      }
      if *value > x {
        let previous = values[index - 1];
        let fraction = (x - previous) / (*value - previous);
        return Some(FormulaValue::Number(
          (index as f64 - 1.0 + fraction + offset) / denominator,
        ));
      }
    }
    Some(FormulaValue::Number(match kind {
      PercentileKind::Inc => 1.0,
      PercentileKind::Exc => values.len() as f64 / denominator,
    }))
  }

  fn evaluate_poisson_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let x = self.number(&self.evaluate(args.first()?)?)?.floor();
    let lambda = self.number(&self.evaluate(args.get(1)?)?)?;
    let cumulative = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.truthy(&value))
      .unwrap_or(true);
    if x < 0.0 || lambda <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let dist = Poisson::new(lambda).ok()?;
    Some(FormulaValue::Number(if cumulative {
      dist.cdf(x as u64)
    } else {
      dist.pmf(x as u64)
    }))
  }

  fn evaluate_fisher(
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

  fn evaluate_bessel(
    &self,
    args: &[FormulaAst<'doc>],
    kind: BesselKind,
  ) -> Option<FormulaValue<'doc>> {
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let order = approx_floor(self.number(&self.evaluate(args.get(1)?)?)?) as i32;
    if order < 0 || matches!(kind, BesselKind::K | BesselKind::Y) && value <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let result = match kind {
      BesselKind::I => bessel_i(value, order),
      BesselKind::J => bessel_j(value, order),
      BesselKind::K => bessel_k(value, order),
      BesselKind::Y => bessel_y(value, order),
    };
    if result.is_finite() {
      Some(FormulaValue::Number(result))
    } else {
      Some(FormulaValue::Error(FormulaErrorValue::Num))
    }
  }

  fn evaluate_fourier(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

    let mut planner = FftPlanner::<f64>::new();
    let fft = if inverse {
      planner.plan_fft_inverse(point_count)
    } else {
      planner.plan_fft_forward(point_count)
    };
    fft.process(&mut values);

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

  fn evaluate_complex_part(
    &self,
    args: &[FormulaAst<'doc>],
    imaginary: bool,
  ) -> Option<FormulaValue<'doc>> {
    let Some(arg) = args.first() else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let value = parse_complex_number(&self.text(&self.evaluate(arg)?))?;
    Some(FormulaValue::Number(if imaginary {
      value.1
    } else {
      value.0
    }))
  }

  fn evaluate_complex(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
    Some(FormulaValue::String(Cow::Owned(format_complex_number(
      real,
      imaginary,
      if suffix == "j" { 'j' } else { 'i' },
    ))))
  }

  fn evaluate_complex_argument(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let (real, imaginary, _) = parse_complex_number(&self.text(&self.evaluate(args.first()?)?))?;
    Some(FormulaValue::Number(imaginary.atan2(real)))
  }

  fn evaluate_complex_abs(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let (real, imaginary, _) = parse_complex_number(&self.text(&self.evaluate(args.first()?)?))?;
    Some(FormulaValue::Number(real.hypot(imaginary)))
  }

  fn evaluate_complex_unary(
    &self,
    args: &[FormulaAst<'doc>],
    op: impl FnOnce(Complex<f64>) -> Complex<f64>,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let (real, imaginary, suffix) =
      parse_complex_number(&self.text(&self.evaluate(args.first()?)?))?;
    let result = op(Complex::new(real, imaginary));
    Some(FormulaValue::String(Cow::Owned(format_complex_number(
      result.re, result.im, suffix,
    ))))
  }

  fn evaluate_complex_binary(
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
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(right) = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| parse_complex_number(&self.text(&value)))
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let suffix = if left.2 == 'j' || right.2 == 'j' {
      'j'
    } else {
      'i'
    };
    Some(FormulaValue::String(Cow::Owned(format_complex_number(
      op(left.0, right.0),
      op(left.1, right.1),
      suffix,
    ))))
  }

  fn evaluate_complex_sum_product(
    &self,
    args: &[FormulaAst<'doc>],
    product: bool,
  ) -> Option<FormulaValue<'doc>> {
    let mut real = if product { 1.0 } else { 0.0 };
    let mut imaginary = 0.0;
    let mut suffix = 'i';
    for source in self.values(args) {
      let Some(value) = parse_complex_number(&self.text(&source)) else {
        return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
      };
      if value.2 == 'j' {
        suffix = 'j';
      }
      if product {
        let next_real = real * value.0 - imaginary * value.1;
        let next_imaginary = real * value.1 + imaginary * value.0;
        real = next_real;
        imaginary = next_imaginary;
      } else {
        real += value.0;
        imaginary += value.1;
      }
    }
    Some(FormulaValue::String(Cow::Owned(format_complex_number(
      real, imaginary, suffix,
    ))))
  }

  fn evaluate_t_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let t = self.number(&self.evaluate(args.first()?)?)?;
    let df = self.number(&self.evaluate(args.get(1)?)?)?;
    let cumulative = self.truthy(&self.evaluate(args.get(2)?)?);
    if df <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::Num));
    }
    let dist = StudentsT::new(0.0, 1.0, df).ok()?;
    Some(FormulaValue::Number(if cumulative {
      dist.cdf(t)
    } else {
      dist.pdf(t)
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
    let dist = StudentsT::new(0.0, 1.0, df).ok()?;
    Some(FormulaValue::Number(match tails {
      1 => dist.sf(t),
      2 => 2.0 * dist.sf(t),
      _ => return Some(FormulaValue::Error(FormulaErrorValue::Num)),
    }))
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
    let dist = StudentsT::new(0.0, 1.0, df).ok()?;
    Some(FormulaValue::Number(if two_tailed {
      dist.inverse_cdf(1.0 - p / 2.0)
    } else {
      dist.inverse_cdf(p)
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
    let dist = StudentsT::new(0.0, 1.0, df).ok()?;
    Some(FormulaValue::Number(match tails {
      1 => dist.sf(t),
      2 => 2.0 * dist.sf(t),
      _ => return Some(FormulaValue::Error(FormulaErrorValue::Num)),
    }))
  }

  fn evaluate_weibull_dist(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_standardize(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_aggregate(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() < 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(function) = self.number_arg(args, 0).map(|value| value as i32) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(options_arg) = self.number_arg(args, 1).map(|value| value as i32) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let Some(options) = aggregate_options(options_arg) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
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
    aggregate_function_value(self, function, data, k, options)
      .map(|result| match result {
        Ok(value) => FormulaValue::Number(value),
        Err(error) => FormulaValue::Error(error),
      })
      .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown)))
  }

  fn evaluate_db(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn numeric_binary(
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
    Some(FormulaValue::Number(op(
      self.number(&left)?,
      self.number(&right)?,
    )))
  }

  fn map_numeric_value(
    &self,
    value: FormulaValue<'doc>,
    op: impl Fn(f64) -> f64 + Copy,
  ) -> Option<FormulaValue<'doc>> {
    if !matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
      return Some(FormulaValue::Number(op(self.number(&value)?)));
    }
    let values = self.matrix_values(&value);
    values
      .into_iter()
      .map(|row| {
        row
          .into_iter()
          .map(|value| {
            self
              .number(&value)
              .map(|value| FormulaValue::Number(op(value)))
          })
          .collect::<Option<Vec<_>>>()
      })
      .collect::<Option<Vec<_>>>()
      .map(FormulaValue::Matrix)
  }

  fn map_numeric_binary(
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
    let rows = left_rows.max(right_rows);
    let columns = left_columns.max(right_columns);
    if !matrix_can_broadcast(left_rows, left_columns, rows, columns)
      || !matrix_can_broadcast(right_rows, right_columns, rows, columns)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }

    let mut result = Vec::with_capacity(rows);
    for row in 0..rows {
      let mut result_row = Vec::with_capacity(columns);
      for column in 0..columns {
        let left = &left_matrix[row.min(left_rows - 1)][column.min(left_columns - 1)];
        let right = &right_matrix[row.min(right_rows - 1)][column.min(right_columns - 1)];
        result_row.push(if let Some(error) = propagate_binary_error(left, right) {
          FormulaValue::Error(error)
        } else {
          FormulaValue::Number(op(self.number(left)?, self.number(right)?))
        });
      }
      result.push(result_row);
    }
    if rows == 1 && columns == 1 {
      return result.into_iter().next()?.into_iter().next();
    }
    Some(FormulaValue::Matrix(result))
  }

  fn map_binary_values(
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
    let rows = left_rows.max(right_rows);
    let columns = left_columns.max(right_columns);
    if !matrix_can_broadcast(left_rows, left_columns, rows, columns)
      || !matrix_can_broadcast(right_rows, right_columns, rows, columns)
    {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }

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
          .filter(|value| is_blank_for_countblank(value))
          .count()
      }
      FormulaValue::Matrix(rows) => rows
        .iter()
        .flatten()
        .filter(|value| is_blank_for_countblank(value))
        .count(),
      value if is_blank_for_countblank(value) => 1,
      _ => 0,
    }
  }

  fn as_reference(&self, value: &FormulaValue<'doc>) -> Option<QualifiedRange<'doc>> {
    match value {
      FormulaValue::Reference(reference) => Some(reference.clone()),
      _ => None,
    }
  }

  fn reference_ranges_from_ast(&self, ast: &FormulaAst<'doc>) -> Vec<QualifiedRange<'doc>> {
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
        left: _,
        right: _,
      } => self
        .evaluate(ast)
        .and_then(|value| self.as_reference(&value))
        .into_iter()
        .collect(),
      FormulaAst::Name(_) | FormulaAst::ExternalReference(_) | FormulaAst::Function { .. } => self
        .evaluate(ast)
        .and_then(|value| self.as_reference(&value))
        .into_iter()
        .collect(),
      _ => Vec::new(),
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

  fn scalar_value(&self, value: FormulaValue<'doc>) -> FormulaValue<'doc> {
    self.first_value(&value)
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

  fn number_arg(&self, args: &[FormulaAst<'doc>], index: usize) -> Option<f64> {
    args
      .get(index)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
  }

  fn evaluate_numeric_unary(
    &self,
    args: &[FormulaAst<'doc>],
    op: impl FnOnce(f64) -> f64,
  ) -> Option<FormulaValue<'doc>> {
    self
      .number_arg(args, 0)
      .map(|value| FormulaValue::Number(op(value)))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown)))
  }

  fn date_number_from_value(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    match self.first_value(value) {
      FormulaValue::String(text) => match datevalue(&text) {
        FormulaValue::Number(value) => Some(value),
        _ => None,
      },
      value => self.number(&value),
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
enum BesselKind {
  I,
  J,
  K,
  Y,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum IfsAggregate {
  Sum,
  Count,
  Average,
  Min,
  Max,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DatabaseFunction {
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
enum CriteriaOperator {
  Equal,
  NotEqual,
  Less,
  LessOrEqual,
  Greater,
  GreaterOrEqual,
}

#[derive(Clone, Debug, PartialEq)]
struct FormulaCriteria<'doc> {
  op: CriteriaOperator,
  value: FormulaValue<'doc>,
  wildcard: bool,
}

impl<'doc> FormulaCriteria<'doc> {
  fn from_value(evaluator: &FormulaEvaluator<'_, 'doc>, value: &FormulaValue<'doc>) -> Self {
    let value = evaluator.first_value(value);
    if let FormulaValue::String(text) = value {
      let (op, operand) = parse_criteria_operator(text.as_ref());
      let operand_value = operand
        .trim()
        .parse::<f64>()
        .map(FormulaValue::Number)
        .unwrap_or_else(|_| FormulaValue::String(Cow::Owned(operand.to_string())));
      let wildcard = matches!(operand_value, FormulaValue::String(_))
        && operand.chars().any(|ch| matches!(ch, '*' | '?' | '~'));
      Self {
        op,
        value: operand_value,
        wildcard,
      }
    } else {
      Self {
        op: CriteriaOperator::Equal,
        value,
        wildcard: false,
      }
    }
  }

  fn matches(
    &self,
    evaluator: &FormulaEvaluator<'_, 'doc>,
    candidate: &FormulaValue<'doc>,
  ) -> bool {
    if matches!(candidate, FormulaValue::Error(_)) {
      return false;
    }
    if self.wildcard {
      let FormulaValue::String(pattern) = &self.value else {
        return false;
      };
      let matched = wildcard_match(pattern.as_ref(), &evaluator.text(candidate));
      return match self.op {
        CriteriaOperator::Equal => matched,
        CriteriaOperator::NotEqual => !matched,
        _ => false,
      };
    }
    let ordering = if let Some((candidate, value)) = evaluator
      .number(candidate)
      .zip(evaluator.number(&self.value))
    {
      candidate.partial_cmp(&value)
    } else {
      Some(evaluator.text(candidate).cmp(&evaluator.text(&self.value)))
    };
    match self.op {
      CriteriaOperator::Equal => ordering == Some(std::cmp::Ordering::Equal),
      CriteriaOperator::NotEqual => ordering != Some(std::cmp::Ordering::Equal),
      CriteriaOperator::Less => ordering == Some(std::cmp::Ordering::Less),
      CriteriaOperator::LessOrEqual => matches!(
        ordering,
        Some(std::cmp::Ordering::Less | std::cmp::Ordering::Equal)
      ),
      CriteriaOperator::Greater => ordering == Some(std::cmp::Ordering::Greater),
      CriteriaOperator::GreaterOrEqual => matches!(
        ordering,
        Some(std::cmp::Ordering::Greater | std::cmp::Ordering::Equal)
      ),
    }
  }
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
    4 => Some(values.into_iter().reduce(f64::max).unwrap_or(0.0)),
    5 => Some(values.into_iter().reduce(f64::min).unwrap_or(0.0)),
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

fn matrix_can_broadcast(
  rows: usize,
  columns: usize,
  target_rows: usize,
  target_columns: usize,
) -> bool {
  (rows == target_rows || rows == 1) && (columns == target_columns || columns == 1)
}

fn matrix_dimensions<T>(matrix: &[Vec<T>]) -> (usize, usize) {
  (matrix.len(), matrix.first().map_or(0, Vec::len))
}

fn matrix_determinant(mut matrix: Vec<Vec<f64>>) -> f64 {
  let n = matrix.len();
  let mut sign = 1.0;
  let mut det = 1.0;
  for pivot in 0..n {
    let mut best = pivot;
    for row in pivot + 1..n {
      if matrix[row][pivot].abs() > matrix[best][pivot].abs() {
        best = row;
      }
    }
    if matrix[best][pivot] == 0.0 {
      return 0.0;
    }
    if best != pivot {
      matrix.swap(best, pivot);
      sign = -sign;
    }
    let pivot_value = matrix[pivot][pivot];
    det *= pivot_value;
    let (head, tail) = matrix.split_at_mut(pivot + 1);
    let pivot_row = &head[pivot];
    for row in tail.iter_mut() {
      let factor = row[pivot] / pivot_value;
      row[pivot] = 0.0;
      for (cell, pivot_cell) in row[pivot + 1..].iter_mut().zip(&pivot_row[pivot + 1..]) {
        *cell -= factor * pivot_cell;
      }
    }
  }
  sign * det
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
  let values = if vertical {
    matrix
      .iter()
      .filter_map(|row| row.first().cloned())
      .collect::<Vec<_>>()
  } else {
    matrix.first()?.clone()
  };
  Some((values, vertical))
}

fn lookup_approximate_index<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  lookup: &FormulaValue<'doc>,
  vector: &[FormulaValue<'doc>],
) -> Option<usize> {
  let mut found = None;
  for (index, candidate) in vector.iter().enumerate() {
    if matches!(candidate, FormulaValue::Error(_)) {
      continue;
    }
    if lookup_compare(evaluator, candidate, lookup).is_some_and(|ordering| ordering <= 0) {
      found = Some(index);
    }
  }
  found
}

fn lookup_next_larger_index<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  lookup: &FormulaValue<'doc>,
  vector: &[FormulaValue<'doc>],
) -> Option<usize> {
  let mut found = None;
  for (index, candidate) in vector.iter().enumerate() {
    if matches!(candidate, FormulaValue::Error(_)) {
      continue;
    }
    if lookup_compare(evaluator, candidate, lookup).is_some_and(|ordering| ordering >= 0) {
      let replace = found.is_none_or(|found_index| {
        lookup_compare(evaluator, candidate, &vector[found_index])
          .is_some_and(|ordering| ordering < 0)
      });
      if replace {
        found = Some(index);
      }
    }
  }
  found
}

fn lookup_descending_index<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  lookup: &FormulaValue<'doc>,
  vector: &[FormulaValue<'doc>],
) -> Option<usize> {
  let mut found = None;
  for (index, candidate) in vector.iter().enumerate() {
    if matches!(candidate, FormulaValue::Error(_)) {
      continue;
    }
    if lookup_compare(evaluator, candidate, lookup).is_some_and(|ordering| ordering >= 0) {
      found = Some(index);
    }
  }
  found
}

fn lookup_exact_index<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  lookup: &FormulaValue<'doc>,
  vector: &[FormulaValue<'doc>],
) -> Option<usize> {
  for (index, candidate) in vector.iter().enumerate() {
    if matches!(candidate, FormulaValue::Error(_)) {
      continue;
    }
    if lookup_exact_match(evaluator, candidate, lookup) {
      return Some(index);
    }
  }
  None
}

fn lookup_exact_index_reverse<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  lookup: &FormulaValue<'doc>,
  vector: &[FormulaValue<'doc>],
) -> Option<usize> {
  for (index, candidate) in vector.iter().enumerate().rev() {
    if matches!(candidate, FormulaValue::Error(_)) {
      continue;
    }
    if lookup_exact_match(evaluator, candidate, lookup) {
      return Some(index);
    }
  }
  None
}

fn lookup_exact_match<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  candidate: &FormulaValue<'doc>,
  lookup: &FormulaValue<'doc>,
) -> bool {
  match (candidate, lookup) {
    (FormulaValue::String(candidate), FormulaValue::String(lookup)) => {
      if lookup.contains('*') || lookup.contains('?') || lookup.contains('~') {
        wildcard_match(lookup, candidate)
      } else {
        candidate.eq_ignore_ascii_case(lookup)
      }
    }
    (FormulaValue::String(_), _) | (_, FormulaValue::String(_)) => false,
    _ => lookup_compare(evaluator, candidate, lookup) == Some(0),
  }
}

fn lookup_compare<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  candidate: &FormulaValue<'doc>,
  lookup: &FormulaValue<'doc>,
) -> Option<i32> {
  match (candidate, lookup) {
    (FormulaValue::String(candidate), FormulaValue::String(lookup)) => Some(
      match candidate
        .to_ascii_lowercase()
        .cmp(&lookup.to_ascii_lowercase())
      {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
      },
    ),
    (FormulaValue::String(_), _) | (_, FormulaValue::String(_)) => None,
    _ => evaluator
      .number(candidate)
      .zip(evaluator.number(lookup))
      .map(|(candidate, lookup)| match candidate.total_cmp(&lookup) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
      }),
  }
}

fn parse_criteria_operator(value: &str) -> (CriteriaOperator, &str) {
  if let Some(rest) = value.strip_prefix("<>") {
    (CriteriaOperator::NotEqual, rest)
  } else if let Some(rest) = value.strip_prefix("<=") {
    (CriteriaOperator::LessOrEqual, rest)
  } else if let Some(rest) = value.strip_prefix(">=") {
    (CriteriaOperator::GreaterOrEqual, rest)
  } else if let Some(rest) = value.strip_prefix('=') {
    (CriteriaOperator::Equal, rest)
  } else if let Some(rest) = value.strip_prefix('<') {
    (CriteriaOperator::Less, rest)
  } else if let Some(rest) = value.strip_prefix('>') {
    (CriteriaOperator::Greater, rest)
  } else {
    (CriteriaOperator::Equal, value)
  }
}

fn wildcard_match(pattern: &str, text: &str) -> bool {
  let pattern = pattern.to_ascii_lowercase().chars().collect::<Vec<_>>();
  let text = text.to_ascii_lowercase().chars().collect::<Vec<_>>();
  let mut p = 0usize;
  let mut t = 0usize;
  let mut star = None;
  let mut star_text = 0usize;
  while t < text.len() {
    if p < pattern.len() {
      if pattern[p] == '~' && p + 1 < pattern.len() {
        if pattern[p + 1] == text[t] {
          p += 2;
          t += 1;
          continue;
        }
      } else if pattern[p] == '?' || pattern[p] == text[t] {
        p += 1;
        t += 1;
        continue;
      } else if pattern[p] == '*' {
        star = Some(p);
        p += 1;
        star_text = t;
        continue;
      }
    }
    if let Some(star_index) = star {
      p = star_index + 1;
      star_text += 1;
      t = star_text;
    } else {
      return false;
    }
  }
  while p < pattern.len() && pattern[p] == '*' {
    p += 1;
  }
  p == pattern.len()
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

fn permutationa_value<'doc>(count: f64, chosen: f64) -> FormulaValue<'doc> {
  let count = approx_floor(count);
  let chosen = approx_floor(chosen);
  if count < 0.0 || chosen < 0.0 {
    FormulaValue::Error(FormulaErrorValue::Num)
  } else {
    FormulaValue::Number(count.powf(chosen))
  }
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

fn half_width_like_asc(text: &str) -> String {
  let mut output = String::with_capacity(text.len());
  for ch in text.chars() {
    match ch {
      '\u{2015}' | '\u{30FC}' => output.push('\u{FF70}'),
      '\u{2018}' => output.push('`'),
      '\u{2019}' => output.push('\''),
      '\u{201D}' => output.push('"'),
      '\u{3001}' => output.push('\u{FF64}'),
      '\u{3002}' => output.push('\u{FF61}'),
      '\u{300C}' => output.push('\u{FF62}'),
      '\u{300D}' => output.push('\u{FF63}'),
      '\u{309B}' => output.push('\u{FF9E}'),
      '\u{309C}' => output.push('\u{FF9F}'),
      '\u{30FB}' => output.push('\u{FF65}'),
      '\u{FF01}'..='\u{FF5E}' => {
        output.push(char::from_u32(ch as u32 - 0xFEE0).unwrap_or(ch));
      }
      _ => output.push(full_katakana_to_half(ch).unwrap_or(ch)),
    }
  }
  output
}

fn full_width_like_jis(text: &str) -> String {
  let mut output = String::with_capacity(text.len());
  let mut chars = text.chars().peekable();
  while let Some(ch) = chars.next() {
    match ch {
      '!'..='~' => output.push(match ch {
        '\\' => '\u{FFE5}',
        '`' => '\u{2018}',
        _ => char::from_u32(ch as u32 + 0xFEE0).unwrap_or(ch),
      }),
      '\u{FF61}' => output.push('\u{3002}'),
      '\u{FF62}' => output.push('\u{300C}'),
      '\u{FF63}' => output.push('\u{300D}'),
      '\u{FF64}' => output.push('\u{3001}'),
      '\u{FF65}' => output.push('\u{30FB}'),
      '\u{FF70}' => output.push('\u{30FC}'),
      '\u{FF9E}' => output.push('\u{309B}'),
      '\u{FF9F}' => output.push('\u{309C}'),
      _ => {
        if let Some(full) = half_katakana_to_full(ch) {
          if let Some(composed) = chars
            .peek()
            .copied()
            .and_then(|mark| compose_katakana_mark(full, mark))
          {
            chars.next();
            output.push(composed);
          } else {
            output.push(full);
          }
        } else {
          output.push(ch);
        }
      }
    }
  }
  output
}

fn full_katakana_to_half(ch: char) -> Option<char> {
  let index = FULL_KATAKANA.iter().position(|full| *full == ch)?;
  Some(HALF_KATAKANA[index])
}

fn half_katakana_to_full(ch: char) -> Option<char> {
  let index = HALF_KATAKANA.iter().position(|half| *half == ch)?;
  Some(FULL_KATAKANA[index])
}

fn compose_katakana_mark(ch: char, mark: char) -> Option<char> {
  match mark {
    '\u{FF9E}' => Some(match ch {
      '\u{30A6}' => '\u{30F4}',
      '\u{30AB}' => '\u{30AC}',
      '\u{30AD}' => '\u{30AE}',
      '\u{30AF}' => '\u{30B0}',
      '\u{30B1}' => '\u{30B2}',
      '\u{30B3}' => '\u{30B4}',
      '\u{30B5}' => '\u{30B6}',
      '\u{30B7}' => '\u{30B8}',
      '\u{30B9}' => '\u{30BA}',
      '\u{30BB}' => '\u{30BC}',
      '\u{30BD}' => '\u{30BE}',
      '\u{30BF}' => '\u{30C0}',
      '\u{30C1}' => '\u{30C2}',
      '\u{30C4}' => '\u{30C5}',
      '\u{30C6}' => '\u{30C7}',
      '\u{30C8}' => '\u{30C9}',
      '\u{30CF}' => '\u{30D0}',
      '\u{30D2}' => '\u{30D3}',
      '\u{30D5}' => '\u{30D6}',
      '\u{30D8}' => '\u{30D9}',
      '\u{30DB}' => '\u{30DC}',
      _ => return None,
    }),
    '\u{FF9F}' => Some(match ch {
      '\u{30CF}' => '\u{30D1}',
      '\u{30D2}' => '\u{30D4}',
      '\u{30D5}' => '\u{30D7}',
      '\u{30D8}' => '\u{30DA}',
      '\u{30DB}' => '\u{30DD}',
      _ => return None,
    }),
    _ => None,
  }
}

const FULL_KATAKANA: [char; 58] = [
  '\u{30A1}', '\u{30A2}', '\u{30A3}', '\u{30A4}', '\u{30A5}', '\u{30A6}', '\u{30A7}', '\u{30A8}',
  '\u{30A9}', '\u{30AA}', '\u{30AB}', '\u{30AD}', '\u{30AF}', '\u{30B1}', '\u{30B3}', '\u{30B5}',
  '\u{30B7}', '\u{30B9}', '\u{30BB}', '\u{30BD}', '\u{30BF}', '\u{30C1}', '\u{30C3}', '\u{30C4}',
  '\u{30C6}', '\u{30C8}', '\u{30CA}', '\u{30CB}', '\u{30CC}', '\u{30CD}', '\u{30CE}', '\u{30CF}',
  '\u{30D2}', '\u{30D5}', '\u{30D8}', '\u{30DB}', '\u{30DE}', '\u{30DF}', '\u{30E0}', '\u{30E1}',
  '\u{30E2}', '\u{30E3}', '\u{30E4}', '\u{30E5}', '\u{30E6}', '\u{30E7}', '\u{30E8}', '\u{30E9}',
  '\u{30EA}', '\u{30EB}', '\u{30EC}', '\u{30ED}', '\u{30EF}', '\u{30F2}', '\u{30F3}', '\u{30FB}',
  '\u{30FC}', '\u{309B}',
];

const HALF_KATAKANA: [char; 58] = [
  '\u{FF67}', '\u{FF71}', '\u{FF68}', '\u{FF72}', '\u{FF69}', '\u{FF73}', '\u{FF6A}', '\u{FF74}',
  '\u{FF6B}', '\u{FF75}', '\u{FF76}', '\u{FF77}', '\u{FF78}', '\u{FF79}', '\u{FF7A}', '\u{FF7B}',
  '\u{FF7C}', '\u{FF7D}', '\u{FF7E}', '\u{FF7F}', '\u{FF80}', '\u{FF81}', '\u{FF6F}', '\u{FF82}',
  '\u{FF83}', '\u{FF84}', '\u{FF85}', '\u{FF86}', '\u{FF87}', '\u{FF88}', '\u{FF89}', '\u{FF8A}',
  '\u{FF8B}', '\u{FF8C}', '\u{FF8D}', '\u{FF8E}', '\u{FF8F}', '\u{FF90}', '\u{FF91}', '\u{FF92}',
  '\u{FF93}', '\u{FF6C}', '\u{FF94}', '\u{FF6D}', '\u{FF95}', '\u{FF6E}', '\u{FF96}', '\u{FF97}',
  '\u{FF98}', '\u{FF99}', '\u{FF9A}', '\u{FF9B}', '\u{FF9C}', '\u{FF66}', '\u{FF9D}', '\u{FF65}',
  '\u{FF70}', '\u{FF9E}',
];

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

fn gcd_number(mut left: f64, mut right: f64) -> f64 {
  left = left.abs();
  right = right.abs();
  while right != 0.0 {
    let remainder = left % right;
    left = right;
    right = remainder;
  }
  left
}

fn lcm_number(left: f64, right: f64) -> f64 {
  if left == 0.0 || right == 0.0 {
    return 0.0;
  }
  (left / gcd_number(left, right) * right).abs()
}

fn bessel_i(x: f64, order: i32) -> f64 {
  let max_iteration = 2000;
  let half = x / 2.0;
  let mut term = 1.0;
  for n in 1..=order {
    term = term / f64::from(n) * half;
  }
  let mut result = term;
  if term != 0.0 {
    for k in 1..max_iteration {
      term = term * half / f64::from(k) * half / f64::from(k + order);
      result += term;
      if term.abs() <= result.abs() * 1.0e-15 {
        break;
      }
    }
  }
  result
}

fn bessel_j(x: f64, order: i32) -> f64 {
  if order == 0 {
    return libm::j0(x);
  }
  if order == 1 {
    return libm::j1(x);
  }
  if x == 0.0 {
    return 0.0;
  }
  let mut previous = libm::j0(x);
  let mut current = libm::j1(x);
  for n in 1..order {
    let next = (2.0 * f64::from(n) / x) * current - previous;
    previous = current;
    current = next;
  }
  current
}

fn bessel_k(x: f64, order: i32) -> f64 {
  match order {
    0 => bessel_k0(x),
    1 => bessel_k1(x),
    _ => {
      let factor = 2.0 / x;
      let mut previous = bessel_k0(x);
      let mut current = bessel_k1(x);
      for n in 1..order {
        let next = previous + f64::from(n) * factor * current;
        previous = current;
        current = next;
      }
      current
    }
  }
}

fn bessel_k0(x: f64) -> f64 {
  if x <= 2.0 {
    let half = x * 0.5;
    let y = half * half;
    -half.ln() * bessel_i(x, 0)
      + (-0.57721566
        + y
          * (0.42278420
            + y
              * (0.23069756
                + y * (0.3488590e-1 + y * (0.262698e-2 + y * (0.10750e-3 + y * 0.74e-5))))))
  } else {
    let y = 2.0 / x;
    (-x).exp() / x.sqrt()
      * (1.25331414
        + y
          * (-0.7832358e-1
            + y
              * (0.2189568e-1
                + y * (-0.1062446e-1 + y * (0.587872e-2 + y * (-0.251540e-2 + y * 0.53208e-3))))))
  }
}

fn bessel_k1(x: f64) -> f64 {
  if x <= 2.0 {
    let half = x * 0.5;
    let y = half * half;
    half.ln() * bessel_i(x, 1)
      + (1.0
        + y
          * (0.15443144
            + y
              * (-0.67278579
                + y * (-0.18156897 + y * (-0.1919402e-1 + y * (-0.110404e-2 + y * -0.4686e-4))))))
        / x
  } else {
    let y = 2.0 / x;
    (-x).exp() / x.sqrt()
      * (1.25331414
        + y
          * (0.23498619
            + y
              * (-0.3655620e-1
                + y * (0.1504268e-1 + y * (-0.780353e-2 + y * (0.325614e-2 + y * -0.68245e-3))))))
  }
}

fn bessel_y(x: f64, order: i32) -> f64 {
  match order {
    0 => libm::y0(x),
    1 => libm::y1(x),
    _ => {
      let factor = 2.0 / x;
      let mut previous = libm::y0(x);
      let mut current = libm::y1(x);
      for n in 1..order {
        let next = f64::from(n) * factor * current - previous;
        previous = current;
        current = next;
      }
      current
    }
  }
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

fn basis_o_datetime_text(serial: f64) -> Option<String> {
  if !serial.is_finite() {
    return None;
  }
  let day = serial.floor();
  let fraction = serial - day;
  let seconds = (fraction * 86_400.0).round() as i64;
  let day_adjust = seconds.div_euclid(86_400);
  let second_of_day = seconds.rem_euclid(86_400);
  let base = days_from_civil(1899, 12, 30)?;
  let (year, month, date) = civil_from_days(base + day as i64 + day_adjust)?;
  Some(format!(
    "{year:04}-{month:02}-{date:02} {:02}:{:02}:{:02}",
    second_of_day / 3600,
    (second_of_day % 3600) / 60,
    second_of_day % 60
  ))
}

fn proleptic_gregorian_week_index_from_serial(serial: i32) -> Option<i64> {
  let (year, month, day) = date_from_serial(serial)?;
  let days = days_from_civil(year, month as i32, day as i32)? - days_from_civil(1, 1, 1)?;
  Some(days.div_euclid(7))
}

fn weeks_in_year_from_serial(serial: i32) -> Option<i32> {
  let (year, _, _) = date_from_serial(serial)?;
  iso_weeks_in_year(year)
}

fn iso_weeknum_from_serial(serial: i32) -> Option<i32> {
  let (year, month, day) = date_from_serial(serial)?;
  let days = days_from_civil(year, month as i32, day as i32)?;
  let year_start = days_from_civil(year, 1, 1)?;
  let day_of_year = (days - year_start + 1) as i32;
  let weekday = ((days - days_from_civil(1, 1, 1)?).rem_euclid(7) + 1) as i32;
  let week = (day_of_year - weekday + 10).div_euclid(7);
  if week < 1 {
    iso_weeks_in_year(year - 1)
  } else {
    let weeks = iso_weeks_in_year(year)?;
    Some(if week > weeks { 1 } else { week })
  }
}

fn iso_weeks_in_year(year: i32) -> Option<i32> {
  let jan1_days = days_from_civil(year, 1, 1)? - days_from_civil(1, 1, 1)?;
  let jan1_weekday = jan1_days.rem_euclid(7);
  if jan1_weekday == 3 || (jan1_weekday == 2 && is_leap_year(year)) {
    Some(53)
  } else {
    Some(52)
  }
}

fn days360(start: i32, end: i32, european: bool) -> Option<i32> {
  let (year1, month1, day1) = date_from_serial(start)?;
  let (year2, month2, day2) = date_from_serial(end)?;
  let mut day1 = day1 as i32;
  let mut day2 = day2 as i32;
  if day1 == 31 {
    day1 = 30;
  } else if !european && month1 == 2 {
    let last = last_day_of_month(year1, month1) as i32;
    if day1 == last {
      day1 = 30;
    }
  }
  if day2 == 31 && (european || day1 == 30) {
    day2 = 30;
  }
  Some((year2 - year1) * 360 + (month2 as i32 - month1 as i32) * 30 + (day2 - day1))
}

fn yearfrac(start: i32, end: i32, basis: i32) -> Option<f64> {
  let (start, end) = if start <= end {
    (start, end)
  } else {
    (end, start)
  };
  if start == end {
    return Some(0.0);
  }
  let (year1, month1, day1) = date_from_serial(start)?;
  let (year2, month2, day2) = date_from_serial(end)?;
  let day_diff = match basis {
    0 => {
      let mut day1 = day1 as i32;
      let mut day2 = day2 as i32;
      if day1 == 31 {
        day1 -= 1;
      }
      if day1 == 30 && day2 == 31 {
        day2 -= 1;
      } else if month1 == 2 && day1 == last_day_of_month(year1, month1) as i32 {
        day1 = 30;
        if month2 == 2 && day2 == last_day_of_month(year2, month2) as i32 {
          day2 = 30;
        }
      }
      (year2 - year1) * 360 + (month2 as i32 - month1 as i32) * 30 + (day2 - day1)
    }
    1..=3 => end - start,
    4 => {
      let day1 = if day1 == 31 { 30 } else { day1 } as i32;
      let day2 = if day2 == 31 { 30 } else { day2 } as i32;
      (year2 - year1) * 360 + (month2 as i32 - month1 as i32) * 30 + (day2 - day1)
    }
    _ => return None,
  };
  let days_in_year = match basis {
    0 | 2 | 4 => 360.0,
    3 => 365.0,
    1 => {
      if year1 != year2
        && (year2 != year1 + 1 || month1 < month2 || (month1 == month2 && day1 < day2))
      {
        let day_count = (year1..=year2)
          .map(|year| if is_leap_year(year) { 366 } else { 365 })
          .sum::<i32>();
        day_count as f64 / (year2 - year1 + 1) as f64
      } else if (year1 == year2 && is_leap_year(year1))
        || (year1 != year2
          && ((is_leap_year(year1) && (month1 < 2 || (month1 == 2 && day1 <= 29)))
            || (is_leap_year(year2) && (month2 > 2 || (month2 == 2 && day2 == 29)))))
      {
        366.0
      } else {
        365.0
      }
    }
    _ => return None,
  };
  Some(day_diff as f64 / days_in_year)
}

fn is_leap_year(year: i32) -> bool {
  (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn last_day_of_month(year: i32, month: u32) -> u32 {
  match month {
    1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
    4 | 6 | 9 | 11 => 30,
    2 if is_leap_year(year) => 29,
    2 => 28,
    _ => 31,
  }
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

fn proper_formula_text(value: &str) -> String {
  let mut result = String::with_capacity(value.len());
  let mut start_word = true;
  for ch in value.chars() {
    if ch.is_alphanumeric() {
      if start_word {
        result.extend(ch.to_uppercase());
      } else {
        result.extend(ch.to_lowercase());
      }
      start_word = false;
    } else {
      result.push(ch);
      start_word = true;
    }
  }
  result
}

fn rot13_formula_text(value: &str) -> String {
  value
    .chars()
    .map(|ch| match ch {
      'a'..='m' | 'A'..='M' => char::from_u32(ch as u32 + 13).unwrap_or(ch),
      'n'..='z' | 'N'..='Z' => char::from_u32(ch as u32 - 13).unwrap_or(ch),
      _ => ch,
    })
    .collect()
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

fn clean_formula_text(value: &str) -> String {
  value
    .chars()
    .filter(|ch| !ch.is_control() && !is_unicode_noncharacter(*ch))
    .collect()
}

fn is_unicode_noncharacter(ch: char) -> bool {
  let code = ch as u32;
  (0xfdd0..=0xfdef).contains(&code) || code & 0xfffe == 0xfffe
}

fn parse_complex_number(value: &str) -> Option<(f64, f64, char)> {
  let normalized;
  let value = if value.contains(',') {
    normalized = value.replace(',', ".");
    normalized.trim()
  } else {
    value.trim()
  };
  if value == "i" || value == "j" {
    return Some((0.0, 1.0, value.chars().next()?));
  }
  let suffix = value.chars().last()?;
  if suffix != 'i' && suffix != 'j' {
    return value.parse::<f64>().ok().map(|real| (real, 0.0, 'i'));
  }
  let body = &value[..value.len() - suffix.len_utf8()];
  if body.is_empty() || body == "+" {
    return Some((0.0, 1.0, suffix));
  }
  if body == "-" {
    return Some((0.0, -1.0, suffix));
  }
  let mut split = None;
  for (index, ch) in body.char_indices().skip(1) {
    let previous = body[..index].chars().next_back();
    if (ch == '+' || ch == '-') && !matches!(previous, Some('e' | 'E')) {
      split = Some(index);
    }
  }
  if let Some(index) = split {
    let real = body[..index].parse::<f64>().ok()?;
    let imaginary = parse_complex_imaginary_part(&body[index..])?;
    Some((real, imaginary, suffix))
  } else {
    parse_complex_imaginary_part(body).map(|imaginary| (0.0, imaginary, suffix))
  }
}

fn parse_complex_imaginary_part(value: &str) -> Option<f64> {
  match value {
    "" | "+" => Some(1.0),
    "-" => Some(-1.0),
    _ => value.parse::<f64>().ok(),
  }
}

fn format_complex_number(real: f64, imaginary: f64, suffix: char) -> String {
  let has_imaginary = imaginary != 0.0;
  let has_real = !has_imaginary || real != 0.0;
  let mut output = String::new();
  if has_real {
    output.push_str(&display_text_from_value(&FormulaValue::Number(real)));
  }
  if has_imaginary {
    if imaginary == 1.0 {
      if has_real {
        output.push('+');
      }
    } else if imaginary == -1.0 {
      output.push('-');
    } else {
      if has_real && imaginary > 0.0 {
        output.push('+');
      }
      output.push_str(&display_text_from_value(&FormulaValue::Number(imaginary)));
    }
    output.push(suffix);
  }
  output
}

fn convert_to_decimal(value: &str, base: u32, char_limit: usize) -> Option<f64> {
  if !(2..=36).contains(&base) {
    return None;
  }
  let value = value.trim();
  if value.len() > char_limit {
    return None;
  }
  if value.is_empty() {
    return Some(0.0);
  }
  let mut result = 0.0;
  let mut first_digit = None;
  for ch in value.chars() {
    let digit = ch.to_digit(base)?;
    if first_digit.is_none() {
      first_digit = Some(digit);
    }
    result = result * base as f64 + digit as f64;
  }
  if value.len() == char_limit && first_digit.is_some_and(|digit| digit >= base / 2) {
    result += -(base as f64).powi(char_limit as i32);
  }
  Some(result)
}

fn convert_from_decimal(
  value: f64,
  min: f64,
  max: f64,
  base: u32,
  places: Option<i32>,
  max_places: usize,
) -> Option<String> {
  let value = approx_floor(value);
  if value < approx_floor(min) || value > approx_floor(max) {
    return None;
  }
  if places.is_some_and(|places| places <= 0 || places as usize > max_places) {
    return None;
  }
  let negative = value < 0.0;
  let mut number = value as i128;
  if negative {
    number += (base as i128).pow(max_places as u32);
  }
  let mut output = format_radix(number, base)?;
  if let Some(places) = places {
    let places = places as usize;
    if !negative && output.len() > places {
      return None;
    }
    let target = if negative { max_places } else { places };
    if output.len() < target {
      let pad = if negative { max_digit_char(base)? } else { '0' };
      let mut padded = String::with_capacity(target);
      padded.extend(std::iter::repeat_n(pad, target - output.len()));
      padded.push_str(&output);
      output = padded;
    }
  }
  Some(output)
}

fn format_radix(mut value: i128, base: u32) -> Option<String> {
  if !(2..=36).contains(&base) || value < 0 {
    return None;
  }
  if value == 0 {
    return Some("0".to_string());
  }
  let mut digits = Vec::new();
  while value > 0 {
    let digit = (value % base as i128) as u32;
    digits.push(char::from_digit(digit, base)?.to_ascii_uppercase());
    value /= base as i128;
  }
  digits.reverse();
  Some(digits.into_iter().collect())
}

fn max_digit_char(base: u32) -> Option<char> {
  char::from_digit(base.checked_sub(1)?, base).map(|ch| ch.to_ascii_uppercase())
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

fn gamma(value: f64) -> f64 {
  statrs_gamma::gamma(value)
}

fn log_gamma(z: f64) -> f64 {
  statrs_gamma::ln_gamma(z)
}

fn norm_s_dist(x: f64) -> f64 {
  Normal::standard().cdf(x)
}

fn norm_s_inv(p: f64) -> f64 {
  if p.is_nan() {
    return f64::NAN;
  }
  if p <= 0.0 {
    return f64::NEG_INFINITY;
  }
  if p >= 1.0 {
    return f64::INFINITY;
  }
  Normal::standard().inverse_cdf(p)
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

fn should_ignore_to_row_column_value(value: &FormulaValue<'_>, ignore: i32) -> bool {
  let ignore_blanks = matches!(ignore, 1 | 3);
  let ignore_errors = matches!(ignore, 2 | 3);
  (ignore_blanks && matches!(value, FormulaValue::Blank))
    || (ignore_errors && matches!(value, FormulaValue::Error(_)))
}

fn erf(x: f64) -> f64 {
  statrs_erf::erf(x)
}

fn erfc(x: f64) -> f64 {
  statrs_erf::erfc(x)
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

fn round_significant(value: f64, digits: f64) -> f64 {
  let scale = value.abs().log10().floor() + 1.0 - digits;
  let mut input = value;
  if scale < 0.0 {
    input *= 10.0_f64.powf(-scale);
  } else {
    input /= 10.0_f64.powf(scale);
  }
  let mut result = rtl_round(input, 0);
  if scale < 0.0 {
    result /= 10.0_f64.powf(-scale);
  } else {
    result *= 10.0_f64.powf(scale);
  }
  result
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

fn financial_pmt(rate: f64, nper: f64, pv: f64, fv: f64, pay_in_advance: bool) -> f64 {
  let payment = if rate == 0.0 {
    (pv + fv) / nper
  } else {
    let log_rate = rate.ln_1p();
    if pay_in_advance {
      (fv + pv * (nper * log_rate).exp()) * rate / (((nper + 1.0) * log_rate).exp_m1() - rate)
    } else {
      (fv + pv * (nper * log_rate).exp()) * rate / (nper * log_rate).exp_m1()
    }
  };
  -payment
}

fn financial_fv(rate: f64, nper: f64, pmt: f64, pv: f64, pay_in_advance: bool) -> f64 {
  if rate == 0.0 {
    pv + pmt * nper
  } else if pay_in_advance {
    pv * (nper * rate.ln_1p()).exp() + pmt * (1.0 + rate) * (nper * rate.ln_1p()).exp_m1() / rate
  } else {
    pv * (nper * rate.ln_1p()).exp() + pmt * (nper * rate.ln_1p()).exp_m1() / rate
  }
}

fn financial_ipmt(
  rate: f64,
  period: f64,
  nper: f64,
  pv: f64,
  fv: f64,
  pay_in_advance: bool,
) -> (f64, f64) {
  let pmt = financial_pmt(rate, nper, pv, fv, pay_in_advance);
  let ipmt = if period == 1.0 {
    if pay_in_advance { 0.0 } else { -pv }
  } else if pay_in_advance {
    financial_fv(rate, period - 2.0, pmt, pv, true) - pmt
  } else {
    financial_fv(rate, period - 1.0, pmt, pv, false)
  };
  (ipmt * rate, pmt)
}

fn financial_cum(
  rate: f64,
  nper: f64,
  pv: f64,
  start: u64,
  end: u64,
  pay_in_advance: bool,
  interest: bool,
) -> f64 {
  let pmt = financial_pmt(rate, nper, pv, 0.0, pay_in_advance);
  let mut total = 0.0;
  let mut current = start;
  if current == 1 {
    if interest {
      if !pay_in_advance {
        total = -pv;
      }
    } else {
      total = if pay_in_advance { pmt } else { pmt + pv * rate };
    }
    current += 1;
  }
  while current <= end {
    if interest {
      total += if pay_in_advance {
        financial_fv(rate, current as f64 - 2.0, pmt, pv, true) - pmt
      } else {
        financial_fv(rate, current as f64 - 1.0, pmt, pv, false)
      };
    } else {
      total += if pay_in_advance {
        pmt - (financial_fv(rate, current as f64 - 2.0, pmt, pv, true) - pmt) * rate
      } else {
        pmt - financial_fv(rate, current as f64 - 1.0, pmt, pv, false) * rate
      };
    }
    current += 1;
  }
  if interest { total * rate } else { total }
}

fn financial_ddb(cost: f64, salvage: f64, life: f64, period: f64, factor: f64) -> f64 {
  let mut rate = factor / life;
  let old_value = if rate >= 1.0 {
    rate = 1.0;
    if period == 1.0 { cost } else { 0.0 }
  } else {
    cost * (1.0 - rate).powf(period - 1.0)
  };
  let new_value = cost * (1.0 - rate).powf(period);
  let ddb = if new_value < salvage {
    old_value - salvage
  } else {
    old_value - new_value
  };
  ddb.max(0.0)
}

fn financial_db(cost: f64, salvage: f64, life: f64, period: f64, months: f64) -> f64 {
  let off_rate = approx_floor(((1.0 - (salvage / cost).powf(1.0 / life)) * 1000.0) + 0.5) / 1000.0;
  let first_off_rate = cost * off_rate * months / 12.0;
  if approx_floor(period) == 1.0 {
    return first_off_rate;
  }
  let mut db = 0.0;
  let mut sum_off_rate = first_off_rate;
  let max = life.min(period).floor() as u16;
  for _ in 2..=max {
    db = -(sum_off_rate - cost) * off_rate;
    sum_off_rate += db;
  }
  if period > life {
    db = -(sum_off_rate - cost) * off_rate * (12.0 - months) / 12.0;
  }
  db
}

fn financial_inter_vdb(
  cost: f64,
  salvage: f64,
  life: f64,
  life1: f64,
  period: f64,
  factor: f64,
) -> f64 {
  let int_end = approx_ceil(period);
  let loop_end = int_end as u64;
  let mut vdb = 0.0;
  let mut sln = 0.0;
  let mut salvage_value = cost - salvage;
  let mut now_sln = false;
  for i in 1..=loop_end {
    let mut term;
    if !now_sln {
      let ddb = financial_ddb(cost, salvage, life, i as f64, factor);
      sln = salvage_value / (life1 - (i - 1) as f64);
      if sln > ddb {
        term = sln;
        now_sln = true;
      } else {
        term = ddb;
        salvage_value -= ddb;
      }
    } else {
      term = sln;
    }
    if i == loop_end {
      term *= period + 1.0 - int_end;
    }
    vdb += term;
  }
  vdb
}

fn financial_vdb(
  cost: f64,
  salvage: f64,
  life: f64,
  start: f64,
  end: f64,
  factor: f64,
  no_switch: bool,
) -> f64 {
  let int_start = approx_floor(start);
  let int_end = approx_ceil(end);
  let loop_start = int_start as u64;
  let loop_end = int_end as u64;
  let mut vdb = 0.0;
  if no_switch {
    for i in loop_start + 1..=loop_end {
      let mut term = financial_ddb(cost, salvage, life, i as f64, factor);
      if i == loop_start + 1 {
        term *= end.min(int_start + 1.0) - start;
      } else if i == loop_end {
        term *= end + 1.0 - int_end;
      }
      vdb += term;
    }
  } else {
    let mut part = 0.0;
    if start != int_start || end != int_end {
      if start != int_start {
        part += financial_inter_vdb(cost, salvage, life, int_start + 1.0, start, factor);
      }
      if end != int_end {
        part += financial_inter_vdb(cost, salvage, life, int_end, end, factor)
          - financial_inter_vdb(cost, salvage, life, int_end, int_end - 1.0, factor);
      }
    }
    vdb = financial_inter_vdb(cost, salvage, life, life, int_end - 1.0, factor)
      - financial_inter_vdb(cost, salvage, life, life, int_start, factor)
      + part;
  }
  vdb
}

fn financial_rate(
  nper: f64,
  payment: f64,
  pv: f64,
  fv: f64,
  pay_type: bool,
  guess: f64,
  default_guess: bool,
) -> Option<f64> {
  let original_guess = guess;
  if let Some(value) = financial_rate_iteration(nper, payment, pv, fv, pay_type, guess) {
    return Some(value);
  }
  if default_guess {
    for step in 2..=10 {
      if let Some(value) = financial_rate_iteration(
        nper,
        payment,
        pv,
        fv,
        pay_type,
        original_guess * step as f64,
      ) {
        return Some(value);
      }
      if let Some(value) = financial_rate_iteration(
        nper,
        payment,
        pv,
        fv,
        pay_type,
        original_guess / step as f64,
      ) {
        return Some(value);
      }
    }
  }
  None
}

fn financial_rate_iteration(
  nper: f64,
  payment: f64,
  mut pv: f64,
  mut fv: f64,
  pay_type: bool,
  guess: f64,
) -> Option<f64> {
  const ITERATIONS_MAX: usize = 150;
  const EPSILON_SMALL: f64 = 1.0E-14;
  const SCD_EPSILON: f64 = 1.0E-7;
  if pay_type {
    fv -= payment;
    pv += payment;
  }
  let integer_nper = nper == rtl_round(nper, 0);
  let mut x = if integer_nper { guess } else { guess.max(-1.0) };
  let mut found = false;
  for _ in 0..ITERATIONS_MAX {
    let (geo_series, geo_derivation, pow_n, pow_n_minus_1) = if x == 0.0 {
      (nper, nper * (nper - 1.0) / 2.0, 1.0, 1.0)
    } else if integer_nper {
      let pow_n_minus_1 = (1.0 + x).powf(nper - 1.0);
      let pow_n = pow_n_minus_1 * (1.0 + x);
      (
        (pow_n - 1.0) / x,
        nper * pow_n_minus_1 / x - ((pow_n - 1.0) / x) / x,
        pow_n,
        pow_n_minus_1,
      )
    } else {
      let pow_n = (1.0 + x).powf(nper);
      let pow_n_minus_1 = (1.0 + x).powf(nper - 1.0);
      (
        (pow_n - 1.0) / x,
        nper * pow_n_minus_1 / x - ((pow_n - 1.0) / x) / x,
        pow_n,
        pow_n_minus_1,
      )
    };
    let term = fv + pv * pow_n + payment * geo_series;
    let term_derivation = pv * nper * pow_n_minus_1 + payment * geo_derivation;
    if term.abs() < EPSILON_SMALL {
      found = true;
      break;
    }
    let x_new = if term_derivation == 0.0 {
      x + 1.1 * SCD_EPSILON
    } else {
      x - term / term_derivation
    };
    found = (x_new - x).abs() < SCD_EPSILON;
    x = x_new;
    if found {
      break;
    }
    if !integer_nper && x < -1.0 {
      return None;
    }
  }
  if found && x > -1.0 { Some(x) } else { None }
}

fn datevalue(text: &str) -> FormulaValue<'static> {
  let Some((date, _time)) = text.trim().split_once(' ').or(Some((text.trim(), ""))) else {
    return FormulaValue::Error(FormulaErrorValue::Value);
  };
  let mut parts = date.split('-');
  let Some(year) = parts.next().and_then(|part| part.parse::<i32>().ok()) else {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  };
  let Some(month) = parts.next().and_then(|part| part.parse::<i32>().ok()) else {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  };
  let Some(day) = parts.next().and_then(|part| part.parse::<i32>().ok()) else {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  };
  if parts.next().is_some() {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  date_serial(year, month, day)
    .map(|value| FormulaValue::Number(value.floor()))
    .unwrap_or(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
}

fn text_byte_len(text: &str) -> usize {
  text.chars().map(char_byte_len).sum()
}

fn char_byte_len(ch: char) -> usize {
  match ch as u32 {
    0x1100..=0x11FF
    | 0x2E80..=0xA4CF
    | 0xAC00..=0xD7AF
    | 0xF900..=0xFAFF
    | 0xFE30..=0xFE4F
    | 0xFF00..=0xFFEF
    | 0x20000..=0x2FA1F => 2,
    _ => 1,
  }
}

fn leftb(text: &str, mut count: usize) -> String {
  let mut result = String::new();
  for ch in text.chars() {
    if count == 0 {
      break;
    }
    let len = char_byte_len(ch);
    if count < len {
      result.push(' ');
      break;
    }
    result.push(ch);
    count -= len;
  }
  result
}

fn rightb(text: &str, mut count: usize) -> String {
  let mut chars = Vec::new();
  for ch in text.chars().rev() {
    if count == 0 {
      break;
    }
    let len = char_byte_len(ch);
    if count < len {
      chars.push(' ');
      break;
    }
    chars.push(ch);
    count -= len;
  }
  chars.into_iter().rev().collect()
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
      ':' => (FormulaOperator::Range, 1),
      '~' => (FormulaOperator::Union, 1),
      '!' => (FormulaOperator::Intersection, 1),
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

fn intersect_qualified_ranges<'doc>(
  left: &QualifiedRange<'doc>,
  right: &QualifiedRange<'doc>,
) -> Option<QualifiedRange<'doc>> {
  if left.sheet != right.sheet || left.sheet_name != right.sheet_name {
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
    sheet_name: left.sheet_name.clone(),
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

fn extend_qualified_range<'doc>(
  left: &QualifiedRange<'doc>,
  right: &QualifiedRange<'doc>,
) -> Option<QualifiedRange<'doc>> {
  if left.sheet != right.sheet || left.sheet_name != right.sheet_name {
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
    sheet_name: left.sheet_name.clone(),
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
  formula_error_literals()
    .iter()
    .any(|literal| literal.eq_ignore_ascii_case(value))
}

fn parse_formula_error_literal_at(value: &str, start: usize) -> Option<(&str, usize)> {
  let rest = value.get(start..)?;
  formula_error_literals()
    .iter()
    .find(|literal| {
      rest
        .get(..literal.len())
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(literal))
    })
    .map(|literal| (*literal, start + literal.len()))
}

fn formula_error_literals() -> &'static [&'static str] {
  &[
    "#GETTING_DATA",
    "#DIV/0!",
    "#VALUE!",
    "#NULL!",
    "#NULL",
    "#REF!",
    "#NAME?",
    "#NUM!",
    "#SPILL!",
    "#CALC!",
    "Err:502",
    "Err:504",
    "Err:508",
    "Err:511",
    "#ERR502!",
    "#ERR508!",
    "#ERR504!",
    "#ERR511!",
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
  match value.to_ascii_uppercase().as_str() {
    "#NULL" | "#NULL!" => FormulaErrorValue::Null,
    "#DIV/0!" => FormulaErrorValue::Div0,
    "#VALUE!" => FormulaErrorValue::Value,
    "#REF!" => FormulaErrorValue::Ref,
    "#NAME?" => FormulaErrorValue::Name,
    "#NUM!" => FormulaErrorValue::Num,
    "#N/A" => FormulaErrorValue::NA,
    "#GETTING_DATA" => FormulaErrorValue::GettingData,
    "#SPILL!" => FormulaErrorValue::Spill,
    "#CALC!" => FormulaErrorValue::Calc,
    "ERR:502" => FormulaErrorValue::IllegalArgument,
    "ERR:504" | "ERR:508" | "ERR:511" | "#ERR502!" | "#ERR508!" | "#ERR504!" | "#ERR511!" => {
      FormulaErrorValue::Unknown
    }
    _ => FormulaErrorValue::Unknown,
  }
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
    return if take { (0, len) } else { (0, 0) };
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
  keys: &[(Vec<FormulaValue<'doc>>, bool)],
  left: usize,
  right: usize,
) -> std::cmp::Ordering {
  for (values, ascending) in keys {
    let ordering = sort_value_order(&values[left], &values[right], *ascending);
    if ordering != std::cmp::Ordering::Equal {
      return ordering;
    }
  }
  left.cmp(&right)
}

fn sort_value_order(
  left: &FormulaValue<'_>,
  right: &FormulaValue<'_>,
  ascending: bool,
) -> std::cmp::Ordering {
  let ordering = match (left, right) {
    (FormulaValue::Number(left), FormulaValue::Number(right)) => left.total_cmp(right),
    (FormulaValue::String(left), FormulaValue::String(right)) => left.cmp(right),
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
    FormulaValue::Blank => 3,
    FormulaValue::Error(_) => 4,
    FormulaValue::Matrix(_) | FormulaValue::Reference(_) => 5,
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
        CellAddress { column: 3, row: 0 },
        FormulaValue::String(Cow::Borrowed("")),
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
      book.evaluate_formula_text(SheetId(1), None, "COUNTBLANK(D1)"),
      Some(FormulaValue::Number(1.0))
    );
    assert_eq!(
      book.evaluate_formula_text(SheetId(1), None, "SUMPRODUCT(ABS(E1:E2),E1:E2+E1:E2)"),
      Some(FormulaValue::Number(14.0))
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
      book.evaluate_formula_text(SheetId(1), None, "MDETERM({1,2;3,4})"),
      Some(FormulaValue::Number(-2.0))
    );
    assert!(matches!(
      parse_formula_with_context(
        FormulaParseContext::default(),
        Cow::Borrowed("SUMIFS(B1:B6,A1:A6,\"a\")")
      )
      .ast,
      Some(FormulaAst::Function { .. })
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
}
