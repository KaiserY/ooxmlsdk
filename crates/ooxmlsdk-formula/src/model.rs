use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use encoding_rs::WINDOWS_1252;
use icu_collator::options::{CollatorOptions, Strength};
use icu_collator::{Collator, CollatorBorrowed, CollatorPreferences};
use icu_locale::Locale;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::workbook_part::WorkbookPart;
use ooxmlsdk::schemas::x;
use ooxmlsdk::sdk::SdkPart;
use regex::RegexBuilder;
use rustfft::FftPlanner;
use rustfft::num_complex::Complex;
use statrs::distribution::{
  Binomial, Continuous, ContinuousCDF, Discrete, DiscreteCDF, Exp, FisherSnedecor, Hypergeometric,
  LogNormal, NegativeBinomial, Normal, Poisson, StudentsT, Weibull,
};
use statrs::function::{erf as statrs_erf, gamma as statrs_gamma};

use crate::{
  AddressFlags, CellAddress, CellRange, DisplayValue, FormulaError, FormulaErrorValue,
  FormulaValue, QualifiedAddress, QualifiedRange, Result, SheetId, SheetName,
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
struct UnitConversionRule {
  from: &'static str,
  to: &'static str,
  factor: f64,
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

const UNIT_CONVERSION_RULES: &[UnitConversionRule] = &[
  UnitConversionRule {
    from: "EUR",
    to: "ATS",
    factor: 13.7603,
  },
  UnitConversionRule {
    from: "EUR",
    to: "BEF",
    factor: 40.3399,
  },
  UnitConversionRule {
    from: "EUR",
    to: "DEM",
    factor: 1.95583,
  },
  UnitConversionRule {
    from: "EUR",
    to: "ESP",
    factor: 166.386,
  },
  UnitConversionRule {
    from: "EUR",
    to: "FIM",
    factor: 5.94573,
  },
  UnitConversionRule {
    from: "EUR",
    to: "FRF",
    factor: 6.55957,
  },
  UnitConversionRule {
    from: "EUR",
    to: "IEP",
    factor: 0.787564,
  },
  UnitConversionRule {
    from: "EUR",
    to: "ITL",
    factor: 1936.27,
  },
  UnitConversionRule {
    from: "EUR",
    to: "LUF",
    factor: 40.3399,
  },
  UnitConversionRule {
    from: "EUR",
    to: "NLG",
    factor: 2.20371,
  },
  UnitConversionRule {
    from: "EUR",
    to: "PTE",
    factor: 200.482,
  },
  UnitConversionRule {
    from: "EUR",
    to: "GRD",
    factor: 340.750,
  },
  UnitConversionRule {
    from: "EUR",
    to: "SIT",
    factor: 239.640,
  },
  UnitConversionRule {
    from: "EUR",
    to: "MTL",
    factor: 0.429300,
  },
  UnitConversionRule {
    from: "EUR",
    to: "CYP",
    factor: 0.585274,
  },
  UnitConversionRule {
    from: "EUR",
    to: "SKK",
    factor: 30.1260,
  },
  UnitConversionRule {
    from: "EUR",
    to: "EEK",
    factor: 15.6466,
  },
  UnitConversionRule {
    from: "EUR",
    to: "LVL",
    factor: 0.702804,
  },
  UnitConversionRule {
    from: "EUR",
    to: "LTL",
    factor: 3.45280,
  },
  UnitConversionRule {
    from: "EUR",
    to: "HRK",
    factor: 7.53450,
  },
  UnitConversionRule {
    from: "EUR",
    to: "BGN",
    factor: 1.95583,
  },
];

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
  if let Some(rule) = UNIT_CONVERSION_RULES
    .iter()
    .find(|rule| rule.from == from && rule.to == to)
  {
    return Ok(value * rule.factor);
  }
  if let Some(rule) = UNIT_CONVERSION_RULES
    .iter()
    .find(|rule| rule.from == to && rule.to == from)
  {
    return Ok(value / rule.factor);
  }

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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FormulaSearchType {
  Normal,
  Regex,
  #[default]
  Wildcard,
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
  let mut chars = formula.chars().peekable();
  while let Some(ch) = chars.next() {
    match ch {
      '"' => {
        output.push(ch);
        if quoted && chars.peek() == Some(&'"') {
          output.push('"');
          chars.next();
        } else {
          quoted = !quoted;
        }
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
  let mut quoted = false;
  while let Some(ch) = chars.next() {
    match ch {
      '"' => {
        output.push(ch);
        if quoted && chars.peek() == Some(&'"') {
          output.push('"');
          chars.next();
        } else {
          quoted = !quoted;
        }
      }
      '[' if !quoted => {
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
      }
      _ => output.push(ch),
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
    if reference.starts_with('$') {
      reference.remove(0);
    }
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

fn r1c1_reference_to_a1(reference: &str, base: CellAddress) -> Option<String> {
  let (start, end) = reference.split_once(':').unwrap_or((reference, reference));
  let start = parse_r1c1_cell(start.trim(), base)?;
  let end = parse_r1c1_cell(end.trim(), base)?;
  let start = format!("{}{}", column_index_to_name(start.column), start.row + 1);
  let end = format!("{}{}", column_index_to_name(end.column), end.row + 1);
  if start == end {
    Some(start)
  } else {
    Some(format!("{start}:{end}"))
  }
}

fn parse_r1c1_cell(reference: &str, base: CellAddress) -> Option<CellAddress> {
  let reference = reference.trim().trim_start_matches('=');
  let upper = reference.to_ascii_uppercase();
  let rest = upper.strip_prefix('R')?;
  let column_marker = rest.find('C')?;
  let (row_text, column_text) = rest.split_at(column_marker);
  let column_text = column_text.strip_prefix('C')?;
  let row = parse_r1c1_axis(row_text, base.row)?;
  let column = parse_r1c1_axis(column_text, base.column)?;
  Some(CellAddress { column, row })
}

fn parse_r1c1_axis(text: &str, base: u32) -> Option<u32> {
  if text.is_empty() {
    return Some(base);
  }
  if let Some(relative) = text
    .strip_prefix('[')
    .and_then(|text| text.strip_suffix(']'))
  {
    return base.checked_add_signed(relative.parse::<i32>().ok()?);
  }
  text.parse::<u32>().ok()?.checked_sub(1)
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
            grammar: parsed.grammar,
            locals: BTreeMap::new(),
            array_context: formula.formula_kind == FormulaKind::Array,
            current_value: None,
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
  pub grammar: FormulaGrammar,
  pub tokens: Vec<FormulaToken<'doc>>,
  pub ast: Option<FormulaAst<'doc>>,
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

fn is_missing_argument(ast: &FormulaAst<'_>) -> bool {
  matches!(ast, FormulaAst::Literal(FormulaValue::Blank))
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
    let (ast, unsupported) = parse_formula_ast(current_sheet, formula);
    if !unsupported.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    FormulaEvaluator {
      book: self,
      current_sheet,
      current_cell,
      grammar,
      locals: BTreeMap::new(),
      array_context: false,
      current_value: None,
    }
    .evaluate(ast.as_ref()?)
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
      grammar: formula.grammar,
      locals: BTreeMap::new(),
      array_context,
      current_value: None,
    }
    .evaluate(formula.ast.as_ref()?)
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
        FormulaEvaluator {
          book: self,
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
    FormulaEvaluator {
      book: self,
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
        let context = FormulaEvaluator {
          book: self,
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
      FormulaEvaluator {
        book: self,
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
    grammar,
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
    let mut left = self.parse_intersection()?;
    loop {
      self.skip_ws();
      if !self.consume_char('~') {
        break;
      }
      let right = self.parse_intersection()?;
      left = FormulaAst::Binary {
        op: FormulaOperator::Union,
        left: Box::new(left),
        right: Box::new(right),
      };
    }
    Some(left)
  }

  fn parse_intersection(&mut self) -> Option<FormulaAst<'doc>> {
    let mut left = self.parse_range()?;
    loop {
      self.skip_ws();
      if !self.consume_char('!') {
        break;
      }
      let right = self.parse_range()?;
      left = FormulaAst::Binary {
        op: FormulaOperator::Intersection,
        left: Box::new(left),
        right: Box::new(right),
      };
    }
    Some(left)
  }

  fn parse_range(&mut self) -> Option<FormulaAst<'doc>> {
    let mut left = self.parse_concat()?;
    loop {
      self.skip_ws();
      if !self.consume_char(':') {
        break;
      }
      let right = self.parse_concat()?;
      left = FormulaAst::Binary {
        op: FormulaOperator::Range,
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
    if let Some((range, next)) = parse_reference_prefix_before_intersection(self.sheet, word, start)
    {
      self.index = next;
      return Some(FormulaAst::Reference(range));
    }
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
  grammar: FormulaGrammar,
  locals: BTreeMap<String, FormulaValue<'doc>>,
  array_context: bool,
  current_value: Option<FormulaValue<'doc>>,
}

struct NumericAggregate {
  values: Vec<f64>,
}

impl<'a, 'doc> FormulaEvaluator<'a, 'doc> {
  fn with_array_context(&self) -> Self {
    Self {
      book: self.book,
      current_sheet: self.current_sheet,
      current_cell: self.current_cell,
      grammar: self.grammar,
      locals: self.locals.clone(),
      array_context: true,
      current_value: self.current_value.clone(),
    }
  }

  fn with_current_value(&self, current_value: FormulaValue<'doc>) -> Self {
    Self {
      book: self.book,
      current_sheet: self.current_sheet,
      current_cell: self.current_cell,
      grammar: self.grammar,
      locals: self.locals.clone(),
      array_context: self.array_context,
      current_value: Some(current_value),
    }
  }

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
    self
      .evaluate_defined_name(name)
      .or(Some(FormulaValue::Error(FormulaErrorValue::Name)))
  }

  fn evaluate_defined_name(&self, name: &Cow<'doc, str>) -> Option<FormulaValue<'doc>> {
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
      _ => Some(FormulaValue::RefList(intersections)),
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
    Some(FormulaValue::RefList(ranges))
  }

  fn range_reference_ranges_from_ast(
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

  fn evaluate_function(
    &self,
    name: &Cow<'doc, str>,
    args: &[FormulaAst<'doc>],
  ) -> Option<FormulaValue<'doc>> {
    let raw_upper = name
      .trim_start_matches("_xlfn.")
      .trim_start_matches("_xlws.")
      .to_ascii_uppercase();
    if raw_upper == "ORG.OPENOFFICE.ERRORTYPE" {
      return self.evaluate_error_type_raw(args);
    }
    if raw_upper == "COM.MICROSOFT.CEILING" {
      return self.evaluate_ceiling_excel_legacy(args);
    }
    if raw_upper == "COM.MICROSOFT.FLOOR" {
      return self.evaluate_floor_excel_legacy(args);
    }
    let upper = canonical_function_name(name);
    match upper.as_str() {
      "LET" => self.evaluate_let(args),
      "IF" => self.evaluate_if(args),
      "IFERROR" => self.evaluate_if_error(args, false),
      "IFNA" => self.evaluate_if_error(args, true),
      "IFS" => self.evaluate_ifs_function(args),
      "SWITCH" => self.evaluate_switch(args),
      "SUM" => match self.numeric_aggregate(args, true) {
        Ok(aggregate) => Some(FormulaValue::Number(kahan_sum(aggregate.values))),
        Err(error) => Some(FormulaValue::Error(error)),
      },
      "PRODUCT" => match self.numeric_aggregate(args, true) {
        Ok(aggregate) => {
          if aggregate.values.is_empty() {
            Some(FormulaValue::Number(0.0))
          } else {
            Some(FormulaValue::Number(aggregate.values.iter().product()))
          }
        }
        Err(error) => Some(FormulaValue::Error(error)),
      },
      "AVERAGE" => {
        let values = match self.numeric_aggregate(args, true) {
          Ok(aggregate) => aggregate.values,
          Err(error) => return Some(FormulaValue::Error(error)),
        };
        (!values.is_empty())
          .then(|| FormulaValue::Number(kahan_sum(values.iter().copied()) / values.len() as f64))
      }
      "COUNT" => Some(FormulaValue::Number(self.count_numbers(args) as f64)),
      "COUNTA" => Some(FormulaValue::Number(self.count_all_values(args) as f64)),
      "ISERROR" => self.evaluate_information_error(args, |_| true),
      "ISNA" => self.evaluate_information_error(args, |error| error == FormulaErrorValue::NA),
      "ISERR" => self.evaluate_information_error(args, |error| error != FormulaErrorValue::NA),
      "ISBLANK" => self.evaluate_isblank(args),
      "ISTEXT" => Some(FormulaValue::Boolean(matches!(
        args
          .first()
          .and_then(|arg| self.evaluate(arg))
          .and_then(|value| self.information_scalar_value(value)),
        Some(FormulaValue::String(_))
      ))),
      "ISNONTEXT" => Some(FormulaValue::Boolean(!matches!(
        args
          .first()
          .and_then(|arg| self.evaluate(arg))
          .and_then(|value| self.information_scalar_value(value)),
        Some(FormulaValue::String(_))
      ))),
      "ISLOGICAL" => Some(FormulaValue::Boolean(matches!(
        args
          .first()
          .and_then(|arg| self.evaluate(arg))
          .and_then(|value| self.information_scalar_value(value)),
        Some(FormulaValue::Boolean(_))
      ))),
      "ISNUMBER" => Some(FormulaValue::Boolean(matches!(
        args
          .first()
          .and_then(|arg| self.evaluate(arg))
          .and_then(|value| self.information_scalar_value(value)),
        Some(FormulaValue::Number(_))
      ))),
      "ISREF" => Some(FormulaValue::Boolean(matches!(
        args.first().and_then(|arg| self.evaluate(arg)),
        Some(FormulaValue::Reference(_) | FormulaValue::RefList(_))
      ))),
      "ISFORMULA" => self.evaluate_is_formula(args),
      "ERROR.TYPE" | "ERRORTYPE" => self.evaluate_error_type(args),
      "TYPE" => self.evaluate_type(args),
      "AREAS" => self.evaluate_areas(args),
      "MIN" => Some(
        self
          .numeric_aggregate(args, true)
          .map(|aggregate| {
            aggregate
              .values
              .iter()
              .copied()
              .reduce(f64::min)
              .map(FormulaValue::Number)
              .unwrap_or(FormulaValue::Number(0.0))
          })
          .unwrap_or_else(FormulaValue::Error),
      ),
      "MINA" => self.evaluate_mina(args),
      "MAX" => Some(
        self
          .numeric_aggregate(args, true)
          .map(|aggregate| {
            aggregate
              .values
              .iter()
              .copied()
              .reduce(f64::max)
              .map(FormulaValue::Number)
              .unwrap_or(FormulaValue::Number(0.0))
          })
          .unwrap_or_else(FormulaValue::Error),
      ),
      "MAXA" => self.evaluate_maxa(args),
      "AND" => self.evaluate_and(args),
      "OR" => self.evaluate_or(args),
      "XOR" => self.evaluate_xor(args),
      "NOT" => self.evaluate_not(args),
      "TRUE" => Some(FormulaValue::Boolean(true)),
      "FALSE" => Some(FormulaValue::Boolean(false)),
      "NA" => Some(FormulaValue::Error(FormulaErrorValue::NA)),
      "STYLE" => Some(FormulaValue::Number(0.0)),
      "CURRENT" => self
        .current_value
        .clone()
        .or(Some(FormulaValue::Error(FormulaErrorValue::Unknown))),
      "N" => Some(FormulaValue::Number(
        match self.first_value(&self.evaluate(args.first()?)?) {
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
        },
      )),
      "COUNTBLANK" => Some(FormulaValue::Number(
        self.count_blank(&self.evaluate(args.first()?)?) as f64,
      )),
      "ABS" => self.evaluate_numeric_unary(args, f64::abs),
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
      "ACOSH" => self.evaluate_numeric_unary_checked_error(
        args,
        |value| (value >= 1.0).then(|| value.acosh()),
        FormulaErrorValue::IllegalArgument,
      ),
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
      "SUMSQ" => Some(FormulaValue::Number(kahan_sum(
        self.numeric_values(args).map(|value| value * value),
      ))),
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
      "DATEVALUE" => Some(datevalue(
        &self.text(&self.evaluate(args.first()?)?),
        self.book.date_system,
      )),
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
      "LEN" => self.evaluate_len(args, false),
      "LENB" => self.evaluate_len(args, true),
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
        .map(legacy_text_code)
        .map(|code| FormulaValue::Number(code as f64))
        .or(Some(FormulaValue::Number(0.0))),
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
        legacy_char_text(code).map_or(
          Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument)),
          |text| Some(FormulaValue::String(Cow::Owned(text))),
        )
      }
      "UNICHAR" => {
        // Source: LibreOffice sc/source/core/tool/interpr1.cxx ScInterpreter::ScUnichar.
        let Some(code) = self.number(&self.evaluate(args.first()?)?) else {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        };
        if code < 0.0 || code > u32::MAX as f64 {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        }
        char::from_u32(code as u32)
          .map(|ch| FormulaValue::String(Cow::Owned(ch.to_string())))
          .or(Some(FormulaValue::Error(
            FormulaErrorValue::IllegalArgument,
          )))
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
      "TEXT" => self.evaluate_text(args),
      "TIMEVALUE" => Some(timevalue(&self.text(&self.evaluate(args.first()?)?))),
      "BASISODATETIME" => self.evaluate_basis_o_datetime(args),
      "DATEDIF" => self.evaluate_datedif(args),
      "YEARFRAC" => self.evaluate_yearfrac(args),
      "WEEKNUM" => self.evaluate_weeknum(args),
      "ISOWEEKNUM" => self.evaluate_iso_weeknum(args),
      "WEEKS" => self.evaluate_weeks(args),
      "WEEKSINYEAR" => self.evaluate_weeks_in_year(args),
      "YEARS" => self.evaluate_years_months(args, true),
      "MONTHS" => self.evaluate_years_months(args, false),
      "DAYSINMONTH" => self.evaluate_days_in_month_year(args, false),
      "DAYSINYEAR" => self.evaluate_days_in_month_year(args, true),
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
      "HSTACK" => self.evaluate_stack(args, true),
      "VSTACK" => self.evaluate_stack(args, false),
      "WRAPCOLS" => self.evaluate_wrap(args, true),
      "WRAPROWS" => self.evaluate_wrap(args, false),
      "FILTER" => self.evaluate_filter(args),
      "UNIQUE" => self.evaluate_unique(args),
      "SEQUENCE" => self.evaluate_sequence(args),
      "TRANSPOSE" => self.evaluate_transpose(args),
      "DROP" => self.evaluate_take_drop(args, false),
      "TAKE" => self.evaluate_take_drop(args, true),
      "SORT" => self.evaluate_sort(args),
      "SORTBY" => self.evaluate_sortby(args),
      "MMULT" => self.evaluate_mmult(args),
      "MDETERM" => self.evaluate_mdeterm(args),
      "MINVERSE" => self.evaluate_minverse(args),
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
      "B" => self.evaluate_b(args),
      "BETA.DIST" => self.evaluate_beta_dist(args, false),
      "BETADIST" => self.evaluate_beta_dist(args, true),
      "BETA.INV" | "BETAINV" => self.evaluate_beta_inv(args),
      "BINOM.DIST" | "BINOMDIST" => self.evaluate_binom_dist(args),
      "BINOM.DIST.RANGE" => self.evaluate_binom_dist_range(args),
      "BINOM.INV" | "CRITBINOM" => self.evaluate_binom_inv(args),
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
      "GAMMALN.PRECISE" | "GAMMALN" => {
        self.evaluate_numeric_unary_checked(args, |value| (value > 0.0).then(|| log_gamma(value)))
      }
      "HYPGEOM.DIST" | "HYPGEOMDIST" => self.evaluate_hypgeom_dist(args),
      "LOGNORM.DIST" | "LOGNORMDIST" => self.evaluate_lognorm_dist(args),
      "LOGNORM.INV" | "LOGINV" => self.evaluate_lognorm_inv(args),
      "MODE.MULT" | "MODE.SNGL" | "MODE" => mode_slice(&self.numeric_args(args))
        .map(FormulaValue::Number)
        .or(Some(FormulaValue::Error(FormulaErrorValue::NA))),
      "NEGBINOM.DIST" | "NEGBINOMDIST" => self.evaluate_negbinom_dist(args),
      "NORM.DIST" | "NORMDIST" => self.evaluate_norm_dist(args),
      "NORM.INV" | "NORMINV" => self.evaluate_norm_inv(args),
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
      "T.DIST.RT" | "TDIST" | "LEGACY.TDIST" => self.evaluate_t_dist_tails(args, 1),
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
      "IMDIV" => self.evaluate_complex_div(args),
      "IMPOWER" => self.evaluate_complex_power(args),
      "IMSUM" => self.evaluate_complex_sum_product(args, false),
      "IMPRODUCT" => self.evaluate_complex_sum_product(args, true),
      "COMPLEX" => self.evaluate_complex(args),
      "DELTA" => self.evaluate_delta(args),
      "GESTEP" => self.evaluate_gestep(args),
      "SLOPE" => self.evaluate_slope(args),
      "INTERCEPT" => self.evaluate_intercept(args),
      "FORECAST" => self.evaluate_forecast(args),
      "FORECAST.ETS" | "FORECAST.ETS.ADD" => self.evaluate_forecast_ets(args, EtsKind::Add),
      "FORECAST.ETS.MULT" => self.evaluate_forecast_ets(args, EtsKind::Mult),
      "FORECAST.ETS.STAT" | "FORECAST.ETS.STAT.ADD" => {
        self.evaluate_forecast_ets(args, EtsKind::StatAdd)
      }
      "FORECAST.ETS.STAT.MULT" => self.evaluate_forecast_ets(args, EtsKind::StatMult),
      "FORECAST.ETS.SEASONALITY" => self.evaluate_forecast_ets(args, EtsKind::Season),
      "RSQ" => self.evaluate_rsq(args),
      "STEYX" => self.evaluate_steyx(args),
      "LINEST" => self.evaluate_linest(args, false),
      "LOGEST" => self.evaluate_linest(args, true),
      "TREND" => self.evaluate_trend_growth(args, false),
      "GROWTH" => self.evaluate_trend_growth(args, true),
      "WEIBULL.DIST" | "WEIBULL" => self.evaluate_weibull_dist(args),
      "NETWORKDAYS.INTL" => self.evaluate_networkdays(args, true),
      "NETWORKDAYS" => self.evaluate_networkdays(args, false),
      "WORKDAY.INTL" => self.evaluate_workday(args, true),
      "WORKDAY" => self.evaluate_workday(args, false),
      "Z.TEST" | "ZTEST" => self.evaluate_z_test(args),
      "STANDARDIZE" => self.evaluate_standardize(args),
      "SUBTOTAL" => self.evaluate_subtotal(args),
      "AGGREGATE" => self.evaluate_aggregate(args),
      "DB" => self.evaluate_db(args),
      "PRICE" => self.evaluate_price(args),
      "YIELD" => self.evaluate_yield(args),
      "PRICEDISC" => self.evaluate_pricedisc(args),
      "PRICEMAT" => self.evaluate_pricemat(args),
      "YIELDDISC" => self.evaluate_yielddisc(args),
      "ACCRINT" => self.evaluate_accrint(args, false),
      "ACCRINTM" => self.evaluate_accrint(args, true),
      "ODDLPRICE" => self.evaluate_oddlprice(args),
      "ODDLYIELD" => self.evaluate_oddlyield(args),
      "AMORLINC" => self.evaluate_amorlinc(args, false),
      "AMORDEGRC" => self.evaluate_amorlinc(args, true),
      "DISC" => self.evaluate_disc(args),
      "RECEIVED" => self.evaluate_received(args),
      "INTRATE" => self.evaluate_intrate(args),
      "MDURATION" => self.evaluate_mduration(args),
      "COUPDAYS" => self.evaluate_coupon(args, CouponFunction::Days),
      "COUPDAYBS" => self.evaluate_coupon(args, CouponFunction::DayBs),
      "COUPDAYSNC" => self.evaluate_coupon(args, CouponFunction::DaysNc),
      "COUPNCD" => self.evaluate_coupon(args, CouponFunction::Ncd),
      "COUPNUM" => self.evaluate_coupon(args, CouponFunction::Num),
      "COUPPCD" => self.evaluate_coupon(args, CouponFunction::Pcd),
      "TBILLEQ" => self.evaluate_tbilleq(args),
      "TBILLPRICE" => self.evaluate_tbillprice(args),
      "TBILLYIELD" => self.evaluate_tbillyield(args),
      "PV" => self.evaluate_pv(args),
      "NPER" => self.evaluate_nper(args),
      "RRI" => self.evaluate_rri(args),
      "PDURATION" => self.evaluate_pduration(args),
      "SERIESSUM" => self.evaluate_seriessum(args),
      "EASTERSUNDAY" | "ORG.OPENOFFICE.EASTERSUNDAY" => self.evaluate_eastersunday(args),
      "XIRR" => self.evaluate_xirr(args),
      "XNPV" => self.evaluate_xnpv(args),
      "IRR" => self.evaluate_irr(args),
      "MIRR" => self.evaluate_mirr(args),
      "EUROCONVERT" => self.evaluate_euroconvert(args),
      "BAHTTEXT" => self.evaluate_bahttext(args),
      "NOMINAL" => self.evaluate_nominal(args),
      "SLN" => self.evaluate_sln(args),
      "SYD" => self.evaluate_syd(args),
      "PROB" => self.evaluate_prob(args),
      "FREQUENCY" => self.evaluate_frequency(args),
      "SUMX2MY2" => self.evaluate_sumx2(args, false),
      "SUMX2PY2" => self.evaluate_sumx2(args, true),
      "SUMXMY2" => self.evaluate_sumxmy2(args),
      "DOLLARDE" => self.evaluate_dollar_decimal(args, true),
      "DOLLARFR" => self.evaluate_dollar_decimal(args, false),
      "INFO" => self.evaluate_info(args),
      "REGEX" => self.evaluate_regex(args),
      "ENCODEURL" => self.evaluate_encodeurl(args),
      "ROT" | "ORG.OPENOFFICE.ROT13" => self.evaluate_rot13(args),
      "GETPIVOTDATA" => self.evaluate_getpivotdata(args),
      "COLOR" | "ORG.LIBREOFFICE.COLOR" => self.evaluate_color(args),
      "DHFG" | "OF" => Some(FormulaValue::Error(FormulaErrorValue::Name)),
      "TESTFUNCBOOL" => Some(FormulaValue::Boolean(true)),
      "TESTFUNCCURR" => Some(FormulaValue::Number(5.5)),
      "TESTFUNCDATE" => Some(FormulaValue::Number(5590.0)),
      "RANDARRAY" => self.evaluate_randarray(args),
      "TESTFUNCINT" | "TESTFUNCLONG" => Some(FormulaValue::Number(6.0)),
      "TESTFUNCSINGLE" | "TESTFUNCDOUBLE" => Some(FormulaValue::Number(5.5)),
      _ => None,
    }
  }

  fn evaluate_color(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_getpivotdata(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn pivot_argument_text(&self, arg: &FormulaAst<'doc>) -> Option<String> {
    let value = self.evaluate(arg)?;
    Some(self.text(&self.first_value(&value)))
  }

  fn evaluate_if(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn map_if_values(
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

  fn evaluate_if_error(
    &self,
    args: &[FormulaAst<'doc>],
    na_only: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
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

  fn map_if_error_values(
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

  fn evaluate_text(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_numbervalue(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.is_empty() || args.len() > 3 {
      return None;
    }
    let mut text = self.text(&self.evaluate(args.first()?)?);
    let decimal = if let Some(arg) = args.get(1) {
      match self.evaluate(arg)? {
        FormulaValue::Blank => {
          return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
        }
        value => self.text(&value),
      }
    } else {
      ".".to_string()
    };
    let group = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .map(|value| self.text(&value))
      .unwrap_or_else(|| ",".to_string());
    let percent_count = text.chars().rev().take_while(|ch| *ch == '%').count();
    text.truncate(text.len() - percent_count);
    text = text
      .chars()
      .filter(|ch| !ch.is_whitespace() && !group.contains(*ch))
      .collect();
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

  fn evaluate_isblank(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return None;
    }
    let value = self.evaluate(args.first()?)?;
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

  fn evaluate_not(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.evaluate(args.first()?)?;
    if matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
      let matrix = self.matrix_values(&value);
      return Some(FormulaValue::Matrix(
        matrix
          .into_iter()
          .map(|row| {
            row
              .into_iter()
              .map(|value| FormulaValue::Boolean(!self.truthy(&value)))
              .collect()
          })
          .collect(),
      ));
    }
    Some(FormulaValue::Boolean(!self.truthy(&value)))
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

  fn base_value(
    &self,
    value: &FormulaValue<'doc>,
    radix: &FormulaValue<'doc>,
    min_len: &FormulaValue<'doc>,
  ) -> Option<FormulaValue<'doc>> {
    // Source: LibreOffice sc/source/core/tool/interpr2.cxx ScInterpreter::ScBase.
    let value = approx_floor(self.number(value)?);
    let radix = approx_floor(self.number(radix)?);
    let min_len_value = approx_floor(self.number(min_len)?);
    let min_len = if (1.0..u16::MAX as f64).contains(&min_len_value) {
      min_len_value as usize
    } else if min_len_value == 0.0 {
      1
    } else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    if value < 0.0 || !(2.0..=36.0).contains(&radix) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut text = format_radix(value as i128, radix as u32)?;
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

  fn evaluate_base_to_base(
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

  fn base_to_base_value(
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

  fn evaluate_decimal_to_base(
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

  fn evaluate_textsplit(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn textsplit_delimiters(&self, value: &FormulaValue<'doc>) -> Vec<String> {
    self
      .matrix_values(value)
      .into_iter()
      .flatten()
      .map(|value| self.text(&value))
      .collect()
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
      | FormulaErrorValue::Parameter
      | FormulaErrorValue::Unknown => return Some(FormulaValue::Error(FormulaErrorValue::NA)),
    };
    Some(FormulaValue::Number(code))
  }

  fn evaluate_error_type_raw(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(value) = args.first().and_then(|arg| self.evaluate(arg)) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
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
      FormulaErrorValue::GettingData | FormulaErrorValue::Calc | FormulaErrorValue::Unknown => {
        515.0
      }
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
      FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => 64.0,
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
    let ranges = self.reference_ranges_from_value(&value);
    if !ranges.is_empty() {
      return Some(FormulaValue::Number(ranges.len() as f64));
    }
    Some(match value {
      FormulaValue::Matrix(_) => FormulaValue::Number(1.0),
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
    date_serial_with_system(year, month, day, self.book.date_system).map(FormulaValue::Number)
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
    let Some(serial) = self.number_arg(args, 0).map(|value| value.floor() as i32) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let (year, month, day) = date_from_serial_with_system(serial, self.book.date_system)?;
    Some(FormulaValue::Number(match part {
      DatePart::Year => year as f64,
      DatePart::Month => month as f64,
      DatePart::Day => day as f64,
    }))
  }

  fn evaluate_today(&self) -> Option<FormulaValue<'doc>> {
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
    let (year, month, _) = date_from_serial_with_system(start, self.book.date_system)?;
    date_serial_with_system(year, month as i32 + months + 1, 0, self.book.date_system)
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
    let (year, month, day) = date_from_serial_with_system(start, self.book.date_system)?;
    let target = date_serial_with_system(
      year,
      month as i32 + months,
      day as i32,
      self.book.date_system,
    )
    .or_else(|| {
      let last = last_day_of_month(year, month);
      date_serial_with_system(
        year,
        month as i32 + months,
        last as i32,
        self.book.date_system,
      )
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
    let (year, _, _) = date_from_serial_with_system(serial, self.book.date_system)?;
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
    let year = date_from_serial_with_system(serial as i32, self.book.date_system)?.0;
    let jan1 = date_serial_with_system(year, 1, 1, self.book.date_system)? as i64;
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
    iso_weeknum_from_serial_with_system(serial, self.book.date_system)
      .map(|value| FormulaValue::Number(value as f64))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Value)))
  }

  fn evaluate_weeks(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_weeks_in_year(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_years_months(
    &self,
    args: &[FormulaAst<'doc>],
    years: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let start = self.date_number_from_value(&self.evaluate(args.first()?)?)? as i32;
    let end = self.date_number_from_value(&self.evaluate(args.get(1)?)?)? as i32;
    let mode = self.number(&self.evaluate(args.get(2)?)?)? as i32;
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

  fn evaluate_days_in_month_year(
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
      .and_then(|value| self.time_number_from_value(&value))
      .unwrap_or_default();
    let total_seconds = ((value.fract() * 86_400.0).round() as i64).rem_euclid(86_400);
    Some(FormulaValue::Number(match part {
      TimePart::Hour => (total_seconds / 3600) as f64,
      TimePart::Minute => ((total_seconds % 3600) / 60) as f64,
      TimePart::Second => (total_seconds % 60) as f64,
    }))
  }

  fn time_number_from_value(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    match self.first_value(value) {
      FormulaValue::String(text) => match timevalue(&text) {
        FormulaValue::Number(value) => Some(value),
        _ => None,
      },
      value => self.number(&value),
    }
  }

  fn evaluate_indirect(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let Some(value) = self.evaluate(args.first()?) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    };
    let text = self.text(&value);
    self
      .resolve_reference(&text)
      .map(FormulaValue::Reference)
      .or_else(|| self.evaluate_defined_name(&Cow::Owned(text)))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Ref)))
  }

  fn evaluate_index(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn index_reference_area(
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
    let Some(search_index) = lookup_binary_search(self, search_slice, query, &param, true, false)
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::NA));
    };
    let index = index_map
      .as_ref()
      .and_then(|indices| indices.get(search_index).copied())
      .unwrap_or(search_index);

    if let Some(result_matrix) = result_matrix {
      let rows = result_matrix.len();
      let columns = result_matrix.first().map_or(0, Vec::len);
      if rows == 1 && columns == 1 {
        return result_matrix
          .first()
          .and_then(|row| row.first())
          .cloned()
          .map(lookup_result_value)
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
        .map(lookup_result_value)
        .or(Some(FormulaValue::Error(FormulaErrorValue::NA)));
    }

    if data_vertical {
      data_matrix
        .get(index)
        .and_then(|row| row.last())
        .cloned()
        .map(lookup_result_value)
        .or(Some(FormulaValue::Error(FormulaErrorValue::NA)))
    } else {
      data_matrix
        .last()
        .and_then(|row| row.get(index))
        .cloned()
        .map(lookup_result_value)
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

  fn evaluate_vlookup(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_vlookup_value(
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

  fn vlookup_reference_row_index(
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
    let param = QueryParam::single(query, search_type, true).with_range_lookup(sorted);
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

  fn evaluate_hlookup(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_xlookup(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
        SearchVectorFlags::new(true, false),
      ),
      -1 => search_vector_with_type(
        self,
        &lookup,
        &lookup_vector,
        QueryOp::Equal,
        search,
        QuerySearchType::Normal,
        SearchVectorFlags::new(true, false),
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
        SearchVectorFlags::new(true, false),
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
    let value = self.evaluate(args.first()?)?;
    let len = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0) as usize;
    if self.array_context && matches!(value, FormulaValue::Reference(_) | FormulaValue::Matrix(_)) {
      let matrix = self.matrix_values(&value);
      return Some(FormulaValue::Matrix(
        matrix
          .into_iter()
          .map(|row| {
            row
              .into_iter()
              .map(|value| {
                FormulaValue::String(Cow::Owned(self.text(&value).chars().take(len).collect()))
              })
              .collect()
          })
          .collect(),
      ));
    }
    let text = self.text(&value);
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

  fn evaluate_choose_cols(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_stack(
    &self,
    args: &[FormulaAst<'doc>],
    horizontal: bool,
  ) -> Option<FormulaValue<'doc>> {
    if args.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
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

  fn evaluate_wrap(
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

  fn evaluate_filter(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(2..=3).contains(&args.len()) {
      return None;
    }
    let data = self.matrix_values(&self.evaluate(args.first()?)?);
    let include = self.matrix_values(&self.evaluate(args.get(1)?)?);
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

  fn evaluate_unique(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
              .unwrap_or_default()
          })
          .collect::<Vec<_>>()
      } else {
        data[index]
          .iter()
          .map(display_text_from_value)
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

  fn evaluate_randarray(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_sumx2(&self, args: &[FormulaAst<'doc>], plus: bool) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let left = self.matrix_values(&self.evaluate(args.first()?)?);
    let right = self.matrix_values(&self.evaluate(args.get(1)?)?);
    if matrix_dimensions(&left) != matrix_dimensions(&right) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut total = KahanSum::default();
    for (left_row, right_row) in left.iter().zip(&right) {
      for (left_value, right_value) in left_row.iter().zip(right_row) {
        let Some(left_number) = matrix_stat_number(left_value) else {
          continue;
        };
        let Some(right_number) = matrix_stat_number(right_value) else {
          continue;
        };
        total.add(left_number * left_number);
        if plus {
          total.add(right_number * right_number);
        } else {
          total.add(-(right_number * right_number));
        }
      }
    }
    Some(FormulaValue::Number(total.finish()))
  }

  fn evaluate_sumxmy2(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let left = self.matrix_values(&self.evaluate(args.first()?)?);
    let right = self.matrix_values(&self.evaluate(args.get(1)?)?);
    if matrix_dimensions(&left) != matrix_dimensions(&right) {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let mut total = KahanSum::default();
    for (left_row, right_row) in left.iter().zip(&right) {
      for (left_value, right_value) in left_row.iter().zip(right_row) {
        let Some(left_number) = matrix_stat_number(left_value) else {
          continue;
        };
        let Some(right_number) = matrix_stat_number(right_value) else {
          continue;
        };
        let difference = approx_sub(left_number, right_number);
        total.add(difference * difference);
      }
    }
    Some(FormulaValue::Number(total.finish()))
  }

  fn evaluate_frequency(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return None;
    }
    let mut data = self.value_numbers(&self.evaluate(args.first()?)?);
    let bins = self.value_numbers(&self.evaluate(args.get(1)?)?);
    if data.is_empty() {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    data.sort_by(|left, right| left.total_cmp(right));
    let mut ordered_bins = bins
      .iter()
      .copied()
      .enumerate()
      .map(|(index, value)| (value, index))
      .collect::<Vec<_>>();
    ordered_bins.sort_by(|left, right| left.0.total_cmp(&right.0));
    let mut counts = vec![0usize; bins.len() + 1];
    let mut data_index = 0;
    for (bin_value, original_index) in ordered_bins {
      let mut count = 0;
      while data_index < data.len() && data[data_index] <= bin_value {
        count += 1;
        data_index += 1;
      }
      counts[original_index] = count;
    }
    counts[bins.len()] = data.len() - data_index;
    Some(FormulaValue::Matrix(
      counts
        .into_iter()
        .map(|count| vec![FormulaValue::Number(count as f64)])
        .collect(),
    ))
  }

  fn evaluate_prob(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_minverse(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let matrix = self.matrix_values(&self.evaluate(args.first()?)?);
    let rows = matrix.len();
    let columns = matrix.first().map_or(0, Vec::len);
    if rows == 0 || rows != columns || matrix.iter().any(|row| row.len() != columns) {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let mut values = vec![vec![0.0; columns * 2]; rows];
    for row in 0..rows {
      for column in 0..columns {
        let Some(number) = self.number(&matrix[row][column]) else {
          return Some(FormulaValue::Error(FormulaErrorValue::Value));
        };
        values[row][column] = number;
      }
      values[row][columns + row] = 1.0;
    }
    for pivot in 0..rows {
      let mut pivot_row = pivot;
      for row in (pivot + 1)..rows {
        if values[row][pivot].abs() > values[pivot_row][pivot].abs() {
          pivot_row = row;
        }
      }
      if values[pivot_row][pivot].abs() < f64::EPSILON {
        return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
      }
      if pivot_row != pivot {
        values.swap(pivot, pivot_row);
      }
      let pivot_value = values[pivot][pivot];
      for value in values[pivot].iter_mut().take(columns * 2) {
        *value /= pivot_value;
      }
      let pivot_values = values[pivot].clone();
      for (row, row_values) in values.iter_mut().enumerate().take(rows) {
        if row == pivot {
          continue;
        }
        let factor = row_values[pivot];
        if factor == 0.0 {
          continue;
        }
        for (value, pivot_value) in row_values
          .iter_mut()
          .zip(pivot_values.iter())
          .take(columns * 2)
        {
          *value -= factor * pivot_value;
        }
      }
    }
    Some(FormulaValue::Matrix(
      values
        .into_iter()
        .map(|row| {
          row[columns..]
            .iter()
            .map(|value| FormulaValue::Number(*value))
            .collect()
        })
        .collect(),
    ))
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

  fn evaluate_stdeva(&self, args: &[FormulaAst<'doc>], sample: bool) -> Option<FormulaValue<'doc>> {
    let values = match self.stvar_text_as_zero_values(args) {
      Ok(values) => values,
      Err(error) => return Some(FormulaValue::Error(error)),
    };
    variance_slice(&values, sample)
      .map(|value| FormulaValue::Number(value.sqrt()))
      .or(Some(FormulaValue::Error(FormulaErrorValue::Div0)))
  }

  fn stvar_text_as_zero_values(
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

  fn push_stvar_range_values(
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

  fn push_stvar_matrix_value(
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

  fn push_stvar_direct_value(
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
    if count < 0.0 || chosen < 0.0 || chosen > count {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if repetition {
      if count == 0.0 && chosen == 0.0 {
        return Some(FormulaValue::Number(0.0));
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
    Some(FormulaValue::Number(financial_fv(
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

  fn npv_values_from_ast(
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

  fn horizontal_range_numbers(
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

  fn npv_matrix_numbers(
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

  fn npv_scalar_number(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    match self.first_value(value) {
      FormulaValue::Number(value) => Some(value),
      FormulaValue::Boolean(value) => Some(if value { 1.0 } else { 0.0 }),
      _ => None,
    }
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

  fn evaluate_xnpv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_xirr(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_irr(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_mirr(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_euroconvert(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(3..=5).contains(&args.len()) {
      return None;
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

  fn evaluate_bahttext(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_regex(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_encodeurl(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let text = self.text(&self.evaluate(args.first()?)?);
    let mut output = String::with_capacity(text.len());
    for byte in text.bytes() {
      if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
        output.push(byte as char);
      } else {
        output.push_str(&format!("%{byte:02X}"));
      }
    }
    Some(FormulaValue::String(Cow::Owned(output)))
  }

  fn evaluate_rot13(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_nominal(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_sln(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_syd(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
    if let Some(value) = self.evaluate_countif_ref_list(args) {
      return Some(value);
    }
    self.evaluate_ifs(None, args, IfsAggregate::Count)
  }

  fn evaluate_countif_ref_list(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn database_matching_rows<'b>(
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

  fn database_query_params(
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

  fn database_star_query_params(
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

  fn evaluate_ceiling(
    &self,
    args: &[FormulaAst<'doc>],
    kind: CeilingFloorKind,
  ) -> Option<FormulaValue<'doc>> {
    if kind == CeilingFloorKind::Precise && !(1..=2).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
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
    if kind == CeilingFloorKind::Precise && !(1..=2).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
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

  fn evaluate_ceiling_excel_legacy(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let significance = self.number(&self.evaluate(args.get(1)?)?)?;
    if value == 0.0 || significance == 0.0 {
      return Some(FormulaValue::Number(0.0));
    }
    if value * significance > 0.0 {
      return Some(FormulaValue::Number(
        approx_ceil(value / significance) * significance,
      ));
    }
    if value < 0.0 {
      return Some(FormulaValue::Number(
        approx_floor(value / -significance) * -significance,
      ));
    }
    Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
  }

  fn evaluate_floor_excel_legacy(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    let value = self.number(&self.evaluate(args.first()?)?)?;
    let significance = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(1.0);
    if value == 0.0 {
      return Some(FormulaValue::Number(0.0));
    }
    if value * significance > 0.0 {
      return Some(FormulaValue::Number(
        approx_floor(value / significance) * significance,
      ));
    }
    if significance == 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    if value < 0.0 {
      return Some(FormulaValue::Number(
        approx_ceil(value / -significance) * -significance,
      ));
    }
    Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument))
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
            Err(error) => FormulaValue::Error(error),
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

  fn evaluate_gestep(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_beta_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
            Err(error) => FormulaValue::Error(error),
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

  fn evaluate_b(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_binom_dist_range(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    self.evaluate_b(args)
  }

  fn evaluate_binom_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_chisq_dist(
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

  fn evaluate_chisq_inv(
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
    Some(FormulaValue::Number(lo_chi_dist(chi, df)))
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

  fn evaluate_intercept(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    regression_scalar_state(self, args).map(|state| {
      if state.x_sum_sq == 0.0 {
        FormulaValue::Error(FormulaErrorValue::Div0)
      } else {
        FormulaValue::Number(state.y_mean - state.xy_sum / state.x_sum_sq * state.x_mean)
      }
    })
  }

  fn evaluate_forecast(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let x = self.number(&self.evaluate(args.first()?)?)?;
    regression_scalar_state_for_values(
      self,
      &self.evaluate(args.get(1)?)?,
      &self.evaluate(args.get(2)?)?,
    )
    .map(|state| {
      if state.x_sum_sq == 0.0 {
        FormulaValue::Error(FormulaErrorValue::Div0)
      } else {
        FormulaValue::Number(state.y_mean + state.xy_sum / state.x_sum_sq * (x - state.x_mean))
      }
    })
  }

  fn evaluate_forecast_ets(
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
      Err(error) => return Some(FormulaValue::Error(error)),
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

  fn evaluate_rsq(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    regression_scalar_state(self, args).map(|state| {
      if state.x_sum_sq == 0.0 || state.y_sum_sq == 0.0 {
        FormulaValue::Error(FormulaErrorValue::Div0)
      } else {
        let pearson = state.xy_sum / (state.x_sum_sq * state.y_sum_sq).sqrt();
        FormulaValue::Number(pearson * pearson)
      }
    })
  }

  fn evaluate_steyx(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    regression_scalar_state(self, args).map(|state| {
      if state.count < 3 || state.x_sum_sq == 0.0 {
        FormulaValue::Error(FormulaErrorValue::Div0)
      } else {
        FormulaValue::Number(
          ((state.y_sum_sq - state.xy_sum * state.xy_sum / state.x_sum_sq)
            / (state.count as f64 - 2.0))
            .sqrt(),
        )
      }
    })
  }

  fn evaluate_linest(
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

  fn evaluate_trend_growth(
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

  fn regression_data(
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
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(if cumulative {
      lo_gamma_dist(x, alpha, beta)
    } else {
      match lo_gamma_dist_pdf(x, alpha, beta) {
        Ok(value) => value,
        Err(error) => return Some(FormulaValue::Error(error)),
      }
    }))
  }

  fn evaluate_gamma_inv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
              result_row.push(FormulaValue::Error(error));
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
    let significance = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(3.0)
      .floor();
    if significance < 1.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let denominator = match kind {
      PercentileKind::Inc => values.len() as f64 - 1.0,
      PercentileKind::Exc => values.len() as f64 + 1.0,
    };
    let offset = match kind {
      PercentileKind::Inc => 0.0,
      PercentileKind::Exc => 1.0,
    };
    let round_result = |value: f64| {
      if value == 0.0 {
        value
      } else {
        let exp = value.abs().log10().floor() + 1.0 - significance;
        (value * 10f64.powf(-exp)).round() / 10f64.powf(-exp)
      }
    };
    for (index, value) in values.iter().enumerate() {
      if *value == x {
        return Some(FormulaValue::Number(round_result(
          (index as f64 + offset) / denominator,
        )));
      }
      if *value > x {
        let previous = values[index - 1];
        let fraction = (x - previous) / (*value - previous);
        return Some(FormulaValue::Number(round_result(
          (index as f64 - 1.0 + fraction + offset) / denominator,
        )));
      }
    }
    Some(FormulaValue::Number(round_result(match kind {
      PercentileKind::Inc => 1.0,
      PercentileKind::Exc => values.len() as f64 / denominator,
    })))
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

  fn bessel_value(
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
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    };
    let Some(right) = args
      .get(1)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| parse_complex_number(&self.text(&value)))
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
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

  fn evaluate_complex_div(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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
    if right.0 == 0.0 && right.1 == 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let result = Complex::new(left.0, left.1) / Complex::new(right.0, right.1);
    if !result.re.is_finite() || !result.im.is_finite() {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    let suffix = if left.2 == 'j' || right.2 == 'j' {
      'j'
    } else {
      'i'
    };
    Some(FormulaValue::String(Cow::Owned(format_complex_number(
      result.re, result.im, suffix,
    ))))
  }

  fn evaluate_complex_power(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 2 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some((real, imaginary, suffix)) = args
      .first()
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| parse_complex_number(&self.text(&value)))
    else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let power = self.number(&self.evaluate(args.get(1)?)?)?;
    let result = Complex::new(real, imaginary).powf(power);
    Some(FormulaValue::String(Cow::Owned(format_complex_number(
      result.re, result.im, suffix,
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
    let values = self.value_numbers_from_ast(args.first()?);
    let x = self.number(&self.evaluate(args.get(1)?)?)?;
    let sigma = args
      .get(2)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or_else(|| variance_slice(&values, true).unwrap_or(0.0).sqrt());
    if values.is_empty() || sigma <= 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
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

  fn evaluate_networkdays(
    &self,
    args: &[FormulaAst<'doc>],
    intl: bool,
  ) -> Option<FormulaValue<'doc>> {
    let mut start = self.number(&self.evaluate(args.first()?)?)?.floor() as i64;
    let mut end = self.number(&self.evaluate(args.get(1)?)?)?.floor() as i64;
    let weekend_arg = intl
      .then(|| args.get(2).and_then(|arg| self.evaluate(arg)))
      .flatten();
    let Some(weekend) = weekend_mask(weekend_arg.as_ref(), false, self) else {
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

  fn evaluate_workday(&self, args: &[FormulaAst<'doc>], intl: bool) -> Option<FormulaValue<'doc>> {
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
    let weekend_arg = intl
      .then(|| args.get(2).and_then(|arg| self.evaluate(arg)))
      .flatten();
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

  fn evaluate_price(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(6..=7).contains(&args.len()) {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
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

  fn evaluate_yield(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(6..=7).contains(&args.len()) {
      return None;
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

  fn evaluate_pricedisc(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_pricemat(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(5..=6).contains(&args.len()) {
      return None;
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

  fn evaluate_yielddisc(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_accrint(
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

  fn evaluate_oddlprice(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(7..=8).contains(&args.len()) {
      return None;
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

  fn evaluate_oddlyield(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_amorlinc(
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

  fn evaluate_disc(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_received(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(4..=5).contains(&args.len()) {
      return None;
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

  fn evaluate_intrate(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_mduration(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if !(5..=6).contains(&args.len()) {
      return None;
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

  fn evaluate_tbilleq(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_tbillprice(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_tbillyield(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_coupon(
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

  fn evaluate_pv(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_nper(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_rri(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 3 {
      return None;
    }
    let periods = self.number_arg(args, 0)?;
    let present = self.number_arg(args, 1)?;
    let future = self.number_arg(args, 2)?;
    if periods <= 0.0 || present == 0.0 {
      return Some(FormulaValue::Error(FormulaErrorValue::IllegalArgument));
    }
    Some(FormulaValue::Number(
      (future / present).powf(1.0 / periods) - 1.0,
    ))
  }

  fn evaluate_pduration(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_seriessum(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_eastersunday(&self, args: &[FormulaAst<'doc>]) -> Option<FormulaValue<'doc>> {
    if args.len() != 1 {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    }
    let Some(year_number) = self.number_arg(args, 0) else {
      return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
    };
    let mut year = year_number.trunc() as i32;
    if year < 100 {
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

  fn map_ternary_values(
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

  fn map_unary_values(
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

  fn values<'b>(
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

  fn numeric_values<'b>(&'b self, args: &'b [FormulaAst<'doc>]) -> impl Iterator<Item = f64> + 'b {
    // Source: LibreOffice ScInterpreter::GetNumberSequenceArray and
    // ScInterpreter::CalculateSkew use ScValueIterator for range/ref-list
    // arguments, which yields numeric cells and skips empty/text cells. Direct
    // scalar arguments still use normal value conversion, so a direct blank is
    // zero while a blank inside a range is ignored.
    args
      .iter()
      .flat_map(|arg| self.value_numbers_from_ast(arg).into_iter())
  }

  fn numeric_aggregate(
    &self,
    args: &[FormulaAst<'doc>],
    text_error: bool,
  ) -> std::result::Result<NumericAggregate, FormulaErrorValue> {
    let mut values = Vec::new();
    let array_evaluator = self.with_array_context();
    for arg in args {
      let ranges = self.reference_ranges_from_ast(arg);
      if !ranges.is_empty() {
        for range in ranges {
          self.push_range_numeric_aggregate_values(&range, &mut values)?;
        }
        continue;
      }
      let value = array_evaluator
        .evaluate(arg)
        .ok_or(FormulaErrorValue::Unknown)?;
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

  fn count_numbers(&self, args: &[FormulaAst<'doc>]) -> usize {
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

  fn count_numbers_in_range(&self, reference: &QualifiedRange<'doc>) -> usize {
    self
      .range_values(reference)
      .iter()
      .filter(|value| matches!(value, FormulaValue::Number(_) | FormulaValue::Boolean(_)))
      .count()
  }

  fn count_all_values(&self, args: &[FormulaAst<'doc>]) -> usize {
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

  fn count_all_values_in_range(&self, reference: &QualifiedRange<'doc>) -> usize {
    self
      .range_values(reference)
      .iter()
      .filter(|value| !matches!(value, FormulaValue::Blank))
      .count()
  }

  fn push_range_numeric_aggregate_values(
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

  fn push_direct_numeric_aggregate_value(
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

  fn numeric_args(&self, args: &[FormulaAst<'doc>]) -> Vec<f64> {
    self.numeric_values(args).collect()
  }

  fn value_numbers(&self, value: &FormulaValue<'doc>) -> Vec<f64> {
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

  fn value_numbers_from_ast(&self, ast: &FormulaAst<'doc>) -> Vec<f64> {
    let mut values = Vec::new();
    self.collect_value_numbers_from_ast(ast, &mut values);
    values
  }

  fn collect_value_numbers_from_ast(&self, ast: &FormulaAst<'doc>, values: &mut Vec<f64>) {
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
      FormulaValue::RefList(ranges) => ranges
        .iter()
        .flat_map(|range| self.matrix_values(&FormulaValue::Reference(range.clone())))
        .collect(),
      FormulaValue::Matrix(rows) => rows.clone(),
      value => vec![vec![value.clone()]],
    }
  }

  fn query_grid_from_ast(&self, ast: &FormulaAst<'doc>) -> Option<QueryGrid<'doc>> {
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

  fn query_grid_from_reference(&self, reference: &QualifiedRange<'doc>) -> QueryGrid<'doc> {
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

  fn count_blank(&self, value: &FormulaValue<'doc>) -> usize {
    match value {
      FormulaValue::Reference(reference) => {
        if reference.range.cell_count_hint() > MAX_EXPANDED_RANGE_CELLS {
          return 0;
        }
        let sheet = self.range_sheet(reference);
        let mut count = 0usize;
        for row in reference.range.start.row..=reference.range.end.row {
          for column in reference.range.start.column..=reference.range.end.column {
            let address = CellAddress { column, row };
            let value = self.book.cell_value(sheet, address);
            let is_blank = if self.book.formula_text(sheet, address).is_some() {
              matches!(value, FormulaValue::String(ref text) if text.is_empty())
            } else {
              matches!(value, FormulaValue::Blank)
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

  fn as_reference(&self, value: &FormulaValue<'doc>) -> Option<QualifiedRange<'doc>> {
    match value {
      FormulaValue::Reference(reference) => Some(reference.clone()),
      FormulaValue::RefList(ranges) if ranges.len() == 1 => ranges.first().cloned(),
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
      FormulaAst::Function { name, args } if canonical_function_name(name) == "XLOOKUP" => {
        self.xlookup_reference_ranges(args).unwrap_or_default()
      }
      FormulaAst::Name(_) | FormulaAst::ExternalReference(_) | FormulaAst::Function { .. } => self
        .evaluate(ast)
        .map(|value| self.reference_ranges_from_value(&value))
        .unwrap_or_default(),
      _ => Vec::new(),
    }
  }

  fn xlookup_reference_ranges(
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
        SearchVectorFlags::new(true, false),
      ),
      -1 => search_vector_with_type(
        self,
        &lookup,
        &lookup_vector,
        QueryOp::Equal,
        search,
        QuerySearchType::Normal,
        SearchVectorFlags::new(true, false),
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
        SearchVectorFlags::new(true, false),
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

  fn reference_ranges_from_value(&self, value: &FormulaValue<'doc>) -> Vec<QualifiedRange<'doc>> {
    match value {
      FormulaValue::Reference(reference) => vec![reference.clone()],
      FormulaValue::RefList(ranges) => ranges.clone(),
      _ => Vec::new(),
    }
  }

  fn resolve_reference(&self, reference: &str) -> Option<QualifiedRange<'doc>> {
    let reference = reference.trim();
    let normalized;
    let reference = if self.grammar == FormulaGrammar::ExcelR1C1 {
      normalized = r1c1_reference_to_a1(reference, self.current_cell.unwrap_or_default())
        .unwrap_or_else(|| reference.to_string());
      normalized.as_str()
    } else {
      reference
    };
    if let Some(table) = parse_table_reference(self.book, reference, self.current_cell) {
      return Some(table);
    }
    parse_formula_range(self.current_sheet, reference)
  }

  fn range_values(&self, range: &QualifiedRange<'doc>) -> Vec<FormulaValue<'doc>> {
    self
      .range_cells(range)
      .into_iter()
      .map(|(_, value)| value)
      .collect()
  }

  fn range_cells(&self, range: &QualifiedRange<'doc>) -> Vec<(CellAddress, FormulaValue<'doc>)> {
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

  fn first_error_value(&self, value: &FormulaValue<'doc>) -> Option<FormulaValue<'doc>> {
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

  fn pivot_data(
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

  fn first_value(&self, value: &FormulaValue<'doc>) -> FormulaValue<'doc> {
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

  fn scalar_value(&self, value: FormulaValue<'doc>) -> FormulaValue<'doc> {
    match &value {
      FormulaValue::Reference(reference) => self.scalar_reference_value(reference),
      FormulaValue::RefList(ranges) => ranges
        .first()
        .map(|range| self.scalar_reference_value(range))
        .unwrap_or_default(),
      _ => self.first_value(&value),
    }
  }

  fn scalar_reference_value(&self, reference: &QualifiedRange<'doc>) -> FormulaValue<'doc> {
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

  fn information_scalar_value(&self, value: FormulaValue<'doc>) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_information_error(
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

  fn scalar_binary_operand(&self, value: FormulaValue<'doc>) -> FormulaValue<'doc> {
    match value {
      FormulaValue::Reference(reference) => self
        .implicit_intersection_value(&reference)
        .unwrap_or(FormulaValue::Error(FormulaErrorValue::Value)),
      FormulaValue::RefList(ranges) => {
        if ranges.len() == 1 {
          self
            .implicit_intersection_value(&ranges[0])
            .unwrap_or(FormulaValue::Error(FormulaErrorValue::Value))
        } else {
          FormulaValue::Error(FormulaErrorValue::Value)
        }
      }
      FormulaValue::Matrix(rows) => rows
        .into_iter()
        .next()
        .and_then(|row| row.into_iter().next())
        .unwrap_or_default(),
      value => value,
    }
  }

  fn implicit_intersection_value(
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

  fn number(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    match self.first_value(value) {
      FormulaValue::Number(value) => Some(value),
      FormulaValue::Boolean(value) => Some(if value { 1.0 } else { 0.0 }),
      FormulaValue::String(value) => value.trim().parse::<f64>().ok(),
      FormulaValue::Blank => Some(0.0),
      FormulaValue::Error(_) => None,
      FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => None,
    }
  }

  fn number_arg(&self, args: &[FormulaAst<'doc>], index: usize) -> Option<f64> {
    args
      .get(index)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
  }

  fn optional_number_value(&self, arg: &FormulaAst<'doc>) -> Option<f64> {
    self
      .evaluate(arg)
      .filter(|value| !matches!(value, FormulaValue::Blank))
      .and_then(|value| self.number(&value))
  }

  fn optional_number_array_values(&self, arg: &FormulaAst<'doc>) -> Option<Vec<f64>> {
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

  fn optional_number_arg(&self, args: &[FormulaAst<'doc>], index: usize, default: f64) -> f64 {
    args
      .get(index)
      .and_then(|arg| self.optional_number_value(arg))
      .unwrap_or(default)
  }

  fn date_arg(&self, args: &[FormulaAst<'doc>], index: usize) -> Option<i32> {
    args
      .get(index)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.date_number_from_value(&value))
      .map(|value| value.floor() as i32)
  }

  fn optional_basis(&self, args: &[FormulaAst<'doc>], index: usize) -> i32 {
    args
      .get(index)
      .and_then(|arg| self.evaluate(arg))
      .and_then(|value| self.number(&value))
      .unwrap_or(0.0)
      .floor() as i32
  }

  fn evaluate_numeric_unary(
    &self,
    args: &[FormulaAst<'doc>],
    op: impl Fn(f64) -> f64 + Copy,
  ) -> Option<FormulaValue<'doc>> {
    self.evaluate_numeric_unary_checked(args, |value| Some(op(value)))
  }

  fn evaluate_len(&self, args: &[FormulaAst<'doc>], bytes: bool) -> Option<FormulaValue<'doc>> {
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

  fn evaluate_numeric_unary_checked(
    &self,
    args: &[FormulaAst<'doc>],
    op: impl Fn(f64) -> Option<f64> + Copy,
  ) -> Option<FormulaValue<'doc>> {
    self.evaluate_numeric_unary_checked_error(args, op, FormulaErrorValue::Unknown)
  }

  fn evaluate_numeric_unary_checked_error(
    &self,
    args: &[FormulaAst<'doc>],
    op: impl Fn(f64) -> Option<f64> + Copy,
    error: FormulaErrorValue,
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
      return self.map_unary_values(value, |evaluator, value| {
        if let FormulaValue::Error(error) = value {
          return Some(FormulaValue::Error(*error));
        }
        evaluator
          .number(value)
          .and_then(op)
          .map(FormulaValue::Number)
          .or(Some(FormulaValue::Error(error)))
      });
    }
    if let FormulaValue::Error(error) = value {
      return Some(FormulaValue::Error(error));
    }
    self
      .number(&value)
      .and_then(op)
      .map(FormulaValue::Number)
      .or(Some(FormulaValue::Error(error)))
  }

  fn date_number_from_value(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    self.date_number_from_scalar(&self.first_value(value))
  }

  fn date_number_from_scalar(&self, value: &FormulaValue<'doc>) -> Option<f64> {
    match value {
      FormulaValue::String(text) => match datevalue(text, self.book.date_system) {
        FormulaValue::Number(value) => Some(value),
        _ => None,
      },
      value => self.number(value),
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
      FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => false,
    }
  }

  fn compare(
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
enum EtsKind {
  Add,
  Mult,
  Season,
  StatAdd,
  StatMult,
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum SumProductScalar {
  Number(f64),
  Error(FormulaErrorValue),
  NaN,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CouponFunction {
  Days,
  DayBs,
  DaysNc,
  Ncd,
  Num,
  Pcd,
}

#[derive(Clone, Copy, Debug)]
struct EtsDataPoint {
  x: f64,
  y: f64,
}

#[derive(Clone, Debug)]
struct EtsCalculation {
  data: Vec<EtsDataPoint>,
  base: Vec<f64>,
  trend: Vec<f64>,
  period_index: Vec<f64>,
  forecast_values: Vec<f64>,
  samples_in_period: usize,
  step_size: f64,
  alpha: f64,
  beta: f64,
  gamma: f64,
  mae: f64,
  mase: f64,
  mse: f64,
  rmse: f64,
  smape: f64,
  additive: bool,
  double_smoothing: bool,
  initialized: bool,
}

impl EtsCalculation {
  const MIN_RESOLUTION: f64 = 0.001;

  fn new(
    timeline: &[f64],
    values: &[f64],
    samples_in_period: usize,
    data_completion: bool,
    aggregation: i32,
    target: Option<f64>,
    kind: EtsKind,
  ) -> std::result::Result<Self, FormulaErrorValue> {
    let mut data = timeline
      .iter()
      .zip(values)
      .map(|(x, y)| EtsDataPoint { x: *x, y: *y })
      .collect::<Vec<_>>();
    data.sort_by(|left, right| left.x.total_cmp(&right.x));
    if let Some(target) = target
      && target < data.first().map_or(0.0, |point| point.x)
    {
      return Err(FormulaErrorValue::Num);
    }

    let mut index = 1usize;
    let mut step_size = f64::MAX;
    while index < data.len() {
      let mut step = data[index].x - data[index - 1].x;
      if step == 0.0 {
        aggregate_duplicate_ets_points(&mut data, index, aggregation)?;
        if index < data.len() {
          step = data[index].x - data[index - 1].x;
        } else {
          step = step_size;
        }
      }
      if step > 0.0 && step < step_size {
        step_size = step;
      }
      index += 1;
    }
    if data.len() < 2 || !step_size.is_finite() || step_size == f64::MAX {
      return Err(FormulaErrorValue::Value);
    }
    let mut has_gap = false;
    for index in 1..data.len() {
      let step = data[index].x - data[index - 1].x;
      if step != step_size {
        if (step % step_size).abs() >= f64::EPSILON {
          return Err(FormulaErrorValue::Value);
        }
        has_gap = true;
      }
    }
    if has_gap {
      fill_ets_gaps(&mut data, step_size, data_completion)?;
    }

    let mut samples_in_period = if samples_in_period != 1 {
      samples_in_period
    } else {
      ets_period_len(&data)
    };
    let double_smoothing = samples_in_period == 0 || samples_in_period == 1;
    if double_smoothing {
      samples_in_period = 0;
    }
    let additive = matches!(kind, EtsKind::Add | EtsKind::Season | EtsKind::StatAdd);
    let mut calculation = Self {
      base: vec![0.0; data.len()],
      trend: vec![0.0; data.len()],
      period_index: vec![0.0; data.len()],
      forecast_values: vec![0.0; data.len()],
      samples_in_period,
      step_size,
      alpha: 0.0,
      beta: 0.0,
      gamma: 0.0,
      mae: 0.0,
      mase: 0.0,
      mse: 0.0,
      rmse: 0.0,
      smape: 0.0,
      additive,
      double_smoothing,
      initialized: false,
      data,
    };
    calculation.init_data()?;
    Ok(calculation)
  }

  fn init_data(&mut self) -> std::result::Result<(), FormulaErrorValue> {
    self.forecast_values[0] = self.data[0].y;
    self.prefill_trend()?;
    self.prefill_period_index()?;
    self.base[0] = if self.double_smoothing {
      self.data[0].y
    } else {
      self.data[0].y / self.period_index[0]
    };
    Ok(())
  }

  fn prefill_trend(&mut self) -> std::result::Result<(), FormulaErrorValue> {
    if self.double_smoothing {
      self.trend[0] = (self.data.last().unwrap().y - self.data[0].y) / (self.data.len() - 1) as f64;
      return Ok(());
    }
    if self.data.len() < 2 * self.samples_in_period {
      return Err(FormulaErrorValue::Value);
    }
    let mut sum = 0.0;
    for index in 0..self.samples_in_period {
      sum += self.data[index + self.samples_in_period].y - self.data[index].y;
    }
    self.trend[0] = sum / (self.samples_in_period * self.samples_in_period) as f64;
    Ok(())
  }

  fn prefill_period_index(&mut self) -> std::result::Result<(), FormulaErrorValue> {
    if self.double_smoothing {
      return Ok(());
    }
    let periods = self.data.len() / self.samples_in_period;
    let mut period_average = vec![0.0; periods];
    for (period, average) in period_average.iter_mut().enumerate() {
      let start = period * self.samples_in_period;
      *average = self.data[start..start + self.samples_in_period]
        .iter()
        .map(|point| point.y)
        .sum::<f64>()
        / self.samples_in_period as f64;
      if *average == 0.0 {
        return Err(FormulaErrorValue::Div0);
      }
    }
    for sample in 0..self.samples_in_period {
      let mut total = 0.0;
      for (period, average) in period_average.iter().enumerate() {
        let trend_adjust =
          (sample as f64 - 0.5 * (self.samples_in_period - 1) as f64) * self.trend[0];
        let y = self.data[period * self.samples_in_period + sample].y;
        total += if self.additive {
          y - (*average + trend_adjust)
        } else {
          y / (*average + trend_adjust)
        };
      }
      self.period_index[sample] = total / periods as f64;
    }
    if self.samples_in_period < self.data.len() {
      self.period_index[self.samples_in_period] = 0.0;
    }
    Ok(())
  }

  fn initialize(&mut self) {
    if self.initialized {
      return;
    }
    self.calc_alpha_beta_gamma();
    self.initialized = true;
    self.calc_accuracy();
  }

  fn calc_alpha_beta_gamma(&mut self) {
    self.alpha = 0.0;
    if self.double_smoothing {
      self.beta = 0.0;
      self.calc_gamma();
    } else {
      self.calc_beta_gamma();
    }
    self.refill();
    let mut low_error = self.mse;
    self.alpha = 1.0;
    if self.double_smoothing {
      self.calc_gamma();
    } else {
      self.calc_beta_gamma();
    }
    self.refill();
    let mut high_error = self.mse;
    self.alpha = 0.5;
    if self.double_smoothing {
      self.calc_gamma();
    } else {
      self.calc_beta_gamma();
    }
    self.refill();
    if low_error == self.mse && self.mse == high_error {
      self.alpha = 0.0;
      if self.double_smoothing {
        self.calc_gamma();
      } else {
        self.calc_beta_gamma();
      }
      self.refill();
      return;
    }
    let mut low = 0.0;
    let mut mid = 0.5;
    let mut high = 1.0;
    while high - mid > Self::MIN_RESOLUTION {
      if high_error > low_error {
        high = mid;
        high_error = self.mse;
        mid = (low + mid) / 2.0;
      } else {
        low = mid;
        low_error = self.mse;
        mid = (mid + high) / 2.0;
      }
      self.alpha = mid;
      if self.double_smoothing {
        self.calc_gamma();
      } else {
        self.calc_beta_gamma();
      }
      self.refill();
    }
  }

  fn calc_beta_gamma(&mut self) {
    self.beta = 0.0;
    self.calc_gamma();
    self.refill();
    let mut low_error = self.mse;
    self.beta = 1.0;
    self.calc_gamma();
    self.refill();
    let mut high_error = self.mse;
    self.beta = 0.5;
    self.calc_gamma();
    self.refill();
    if low_error == self.mse && self.mse == high_error {
      self.beta = 0.0;
      self.calc_gamma();
      self.refill();
      return;
    }
    let mut low = 0.0;
    let mut mid = 0.5;
    let mut high = 1.0;
    while high - mid > Self::MIN_RESOLUTION {
      if high_error > low_error {
        high = mid;
        high_error = self.mse;
        mid = (low + mid) / 2.0;
      } else {
        low = mid;
        low_error = self.mse;
        mid = (mid + high) / 2.0;
      }
      self.beta = mid;
      self.calc_gamma();
      self.refill();
    }
  }

  fn calc_gamma(&mut self) {
    self.gamma = 0.0;
    self.refill();
    let mut low_error = self.mse;
    self.gamma = 1.0;
    self.refill();
    let mut high_error = self.mse;
    self.gamma = 0.5;
    self.refill();
    if low_error == self.mse && self.mse == high_error {
      self.gamma = 0.0;
      self.refill();
      return;
    }
    let mut low = 0.0;
    let mut mid = 0.5;
    let mut high = 1.0;
    while high - mid > Self::MIN_RESOLUTION {
      if high_error > low_error {
        high = mid;
        high_error = self.mse;
        mid = (low + mid) / 2.0;
      } else {
        low = mid;
        low_error = self.mse;
        mid = (mid + high) / 2.0;
      }
      self.gamma = mid;
      self.refill();
    }
  }

  fn refill(&mut self) {
    for index in 1..self.data.len() {
      if self.double_smoothing {
        self.base[index] = self.alpha * self.data[index].y
          + (1.0 - self.alpha) * (self.base[index - 1] + self.trend[index - 1]);
        self.trend[index] = self.gamma * (self.base[index] - self.base[index - 1])
          + (1.0 - self.gamma) * self.trend[index - 1];
        self.forecast_values[index] = self.base[index - 1] + self.trend[index - 1];
      } else {
        let period_index = if self.additive {
          if index > self.samples_in_period {
            index - self.samples_in_period
          } else {
            index
          }
        } else if index >= self.samples_in_period {
          index - self.samples_in_period
        } else {
          index
        };
        if self.additive {
          self.base[index] = self.alpha * (self.data[index].y - self.period_index[period_index])
            + (1.0 - self.alpha) * (self.base[index - 1] + self.trend[index - 1]);
          self.period_index[index] = self.beta * (self.data[index].y - self.base[index])
            + (1.0 - self.beta) * self.period_index[period_index];
        } else {
          self.base[index] = self.alpha * (self.data[index].y / self.period_index[period_index])
            + (1.0 - self.alpha) * (self.base[index - 1] + self.trend[index - 1]);
          self.period_index[index] = self.beta * (self.data[index].y / self.base[index])
            + (1.0 - self.beta) * self.period_index[period_index];
        }
        self.trend[index] = self.gamma * (self.base[index] - self.base[index - 1])
          + (1.0 - self.gamma) * self.trend[index - 1];
        self.forecast_values[index] = if self.additive {
          self.base[index - 1] + self.trend[index - 1] + self.period_index[period_index]
        } else {
          (self.base[index - 1] + self.trend[index - 1]) * self.period_index[period_index]
        };
      }
    }
    self.calc_accuracy();
  }

  fn calc_accuracy(&mut self) {
    let mut sum_abs_error = 0.0;
    let mut sum_divisor = 0.0;
    let mut sum_error_sq = 0.0;
    let mut sum_abs_percent_error = 0.0;
    for index in 1..self.data.len() {
      let error = self.forecast_values[index] - self.data[index].y;
      sum_abs_error += error.abs();
      sum_error_sq += error * error;
      sum_abs_percent_error +=
        error.abs() / (self.forecast_values[index].abs() + self.data[index].y.abs());
    }
    for index in 2..self.data.len() {
      sum_divisor += (self.data[index].y - self.data[index - 1].y).abs();
    }
    let count = (self.data.len() - 1) as f64;
    self.mae = sum_abs_error / count;
    self.mase = if sum_divisor == 0.0 {
      0.0
    } else {
      sum_abs_error / (count * sum_divisor / (count - 1.0))
    };
    self.mse = sum_error_sq / count;
    self.rmse = self.mse.sqrt();
    self.smape = sum_abs_percent_error * 2.0 / count;
  }

  fn forecast(&mut self, target: f64) -> f64 {
    self.initialize();
    let last = self.data.len() - 1;
    if target <= self.data[last].x {
      let index = ((target - self.data[0].x) / self.step_size) as usize;
      let interpolate = (target - self.data[0].x) % self.step_size;
      let mut result = self.data[index].y;
      if interpolate >= Self::MIN_RESOLUTION && index + 1 < self.forecast_values.len() {
        let factor = interpolate / self.step_size;
        result += factor * (self.forecast_values[index + 1] - result);
      }
      return result;
    }
    let steps = ((target - self.data[last].x) / self.step_size) as usize;
    let interpolate = (target - self.data[last].x) % self.step_size;
    let mut result = self.forecast_future(steps);
    if interpolate >= Self::MIN_RESOLUTION {
      let next = self.forecast_future(steps + 1);
      result += interpolate / self.step_size * (next - result);
    }
    result
  }

  fn forecast_future(&self, steps: usize) -> f64 {
    let last = self.data.len() - 1;
    if self.double_smoothing {
      self.base[last] + steps as f64 * self.trend[last]
    } else {
      let index = last - self.samples_in_period + (steps % self.samples_in_period);
      if self.additive {
        self.base[last] + steps as f64 * self.trend[last] + self.period_index[index]
      } else {
        (self.base[last] + steps as f64 * self.trend[last]) * self.period_index[index]
      }
    }
  }

  fn statistic(&mut self, index: i32) -> f64 {
    self.initialize();
    match index {
      1 => self.alpha,
      2 => self.gamma,
      3 => self.beta,
      4 => self.mase,
      5 => self.smape,
      6 => self.mae,
      7 => self.rmse,
      8 => self.step_size,
      9 => self.samples_in_period as f64,
      _ => f64::NAN,
    }
  }
}

fn aggregate_duplicate_ets_points(
  data: &mut Vec<EtsDataPoint>,
  index: usize,
  aggregation: i32,
) -> std::result::Result<(), FormulaErrorValue> {
  let x = data[index - 1].x;
  let mut values = vec![data[index - 1].y];
  while index < data.len() && data[index].x == x {
    values.push(data.remove(index).y);
  }
  data[index - 1].y = match aggregation {
    1 => values[0],
    2 | 3 => values.len() as f64,
    4 => values
      .into_iter()
      .reduce(f64::max)
      .ok_or(FormulaErrorValue::Value)?,
    5 => {
      values.sort_by(f64::total_cmp);
      if values.len() % 2 == 1 {
        values[values.len() / 2]
      } else {
        (values[values.len() / 2] + values[values.len() / 2 - 1]) / 2.0
      }
    }
    6 => values
      .into_iter()
      .reduce(f64::min)
      .ok_or(FormulaErrorValue::Value)?,
    7 => kahan_sum(values.iter().copied()),
    _ => return Err(FormulaErrorValue::IllegalArgument),
  };
  Ok(())
}

fn fill_ets_gaps(
  data: &mut Vec<EtsDataPoint>,
  step_size: f64,
  data_completion: bool,
) -> std::result::Result<(), FormulaErrorValue> {
  let original_count = data.len() as f64;
  let mut missing_count = 0usize;
  let mut index = 1usize;
  while index < data.len() {
    let distance = data[index].x - data[index - 1].x;
    if distance > step_size {
      let y = if data_completion {
        (data[index].y + data[index - 1].y) / 2.0
      } else {
        0.0
      };
      let mut x = data[index - 1].x + step_size;
      while x < data[index].x {
        data.insert(index, EtsDataPoint { x, y });
        missing_count += 1;
        if missing_count as f64 / original_count > 0.3 {
          return Err(FormulaErrorValue::Value);
        }
        index += 1;
        x += step_size;
      }
    }
    index += 1;
  }
  Ok(())
}

fn ets_period_len(data: &[EtsDataPoint]) -> usize {
  let mut best = data.len();
  let mut best_error = f64::MAX;
  for period_len in (1..=data.len() / 2).rev() {
    let periods = data.len() / period_len;
    let start = data.len() - (periods * period_len) + 1;
    let mut mean_error = 0.0;
    for index in start..(data.len() - period_len) {
      mean_error += ((data[index].y - data[index - 1].y)
        - (data[period_len + index].y - data[period_len + index - 1].y))
        .abs();
    }
    mean_error /= ((periods - 1) * period_len - 1) as f64;
    if mean_error <= best_error || mean_error == 0.0 {
      best = period_len;
      best_error = mean_error;
    }
  }
  best
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
enum QueryOp {
  Equal,
  NotEqual,
  Less,
  LessOrEqual,
  Greater,
  GreaterOrEqual,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum QuerySearchType {
  Normal,
  Wildcard,
  Regex,
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
struct QueryParam<'doc> {
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
struct QueryGrid<'doc> {
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
      let operand_value = trimmed
        .parse::<f64>()
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
      let operand_value = trimmed
        .parse::<f64>()
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

fn is_query_empty(value: &FormulaValue<'_>) -> bool {
  matches!(value, FormulaValue::Blank)
    || matches!(value, FormulaValue::String(text) if text.is_empty())
}

fn database_criterion_cell_present(value: &FormulaValue<'_>, query_empty: bool) -> bool {
  !query_empty && !matches!(value, FormulaValue::Blank)
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

fn lookup_search_key(value: &str) -> String {
  value.chars().flat_map(char::to_lowercase).collect()
}

fn lookup_text_contains(text: &str, pattern: &str) -> bool {
  lookup_search_key(text).contains(&lookup_search_key(pattern))
}

#[derive(Clone, Copy)]
struct SearchVectorFlags {
  exact_type: bool,
  match_empty: bool,
}

impl SearchVectorFlags {
  fn new(exact_type: bool, match_empty: bool) -> Self {
    Self {
      exact_type,
      match_empty,
    }
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
    while index + 1 < vector.len() && lookup_same_match_value(&vector[index], &vector[index + 1]) {
      index += 1;
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
struct RegressionData {
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

#[derive(Clone, Debug)]
struct RegressionModel {
  slopes: Vec<f64>,
  intercept: f64,
}

impl RegressionModel {
  fn predict(&self, row: &[f64]) -> f64 {
    self.intercept
      + row
        .iter()
        .zip(&self.slopes)
        .map(|(x, slope)| x * slope)
        .sum::<f64>()
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
  let mut state = regression_state(data, constant)?;
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
    return simple_regression_model(data, constant);
  }
  regression_state(data, constant).map(|state| state.model)
}

#[derive(Clone, Debug)]
struct RegressionState {
  centered_x: Vec<Vec<f64>>,
  centered_y: Vec<f64>,
  means: Vec<f64>,
  r_diagonal: Vec<f64>,
  model: RegressionModel,
}

#[derive(Clone, Copy, Debug)]
struct RegressionScalarState {
  count: usize,
  x_mean: f64,
  y_mean: f64,
  xy_sum: f64,
  x_sum_sq: f64,
  y_sum_sq: f64,
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
  let count = y_values.len().min(x_values.len());
  if count < 2 {
    return Some(RegressionScalarState {
      count,
      x_mean: 0.0,
      y_mean: 0.0,
      xy_sum: 0.0,
      x_sum_sq: 0.0,
      y_sum_sq: 0.0,
    });
  }
  let x_mean = x_values.iter().take(count).sum::<f64>() / count as f64;
  let y_mean = y_values.iter().take(count).sum::<f64>() / count as f64;
  let mut xy_sum = 0.0;
  let mut x_sum_sq = 0.0;
  let mut y_sum_sq = 0.0;
  for index in 0..count {
    let x_delta = x_values[index] - x_mean;
    let y_delta = y_values[index] - y_mean;
    xy_sum += x_delta * y_delta;
    x_sum_sq += x_delta * x_delta;
    y_sum_sq += y_delta * y_delta;
  }
  Some(RegressionScalarState {
    count,
    x_mean,
    y_mean,
    xy_sum,
    x_sum_sq,
    y_sum_sq,
  })
}

fn regression_state(data: &RegressionData, constant: bool) -> Option<RegressionState> {
  let n = data.n();
  let k = data.k();
  if (constant && n < k + 1) || (!constant && n < k) || n < 1 || k < 1 {
    return None;
  }
  let mut centered_x = data.design.clone();
  let mut centered_y = data.y.clone();
  let mut means = vec![0.0; k];
  let mut mean_y = 0.0;
  if constant {
    mean_y = centered_y.iter().sum::<f64>() / n as f64;
    for value in &mut centered_y {
      *value = approx_sub(*value, mean_y);
    }
    for column in 0..k {
      means[column] = centered_x.iter().map(|row| row[column]).sum::<f64>() / n as f64;
      for row in &mut centered_x {
        row[column] = approx_sub(row[column], means[column]);
      }
    }
  }
  let mut qr = centered_x.clone();
  let r_diagonal = qr_decompose(&mut qr, k, n)?;
  if r_diagonal.contains(&0.0) {
    return None;
  }
  let mut z = centered_y.clone();
  for column in 0..k {
    apply_householder(&qr, column, &mut z, n);
  }
  let mut slopes = z.iter().take(k).copied().collect::<Vec<_>>();
  solve_upper(&qr, &r_diagonal, &mut slopes, k);
  let intercept = if constant {
    mean_y
      - means
        .iter()
        .zip(&slopes)
        .map(|(mean, slope)| mean * slope)
        .sum::<f64>()
  } else {
    0.0
  };
  Some(RegressionState {
    centered_x: qr,
    centered_y,
    means,
    r_diagonal,
    model: RegressionModel { slopes, intercept },
  })
}

fn simple_regression_model(data: &RegressionData, constant: bool) -> Option<RegressionModel> {
  // Source: LibreOffice sc/source/core/tool/interpr5.cxx
  // ScInterpreter::CalculateRGPRKP nCase==1.
  if data.case != RegressionCase::Simple {
    return None;
  }
  let n = data.n();
  if (constant && n < 2) || n < 1 {
    return None;
  }
  let x = data
    .design
    .iter()
    .map(|row| row.first().copied())
    .collect::<Option<Vec<_>>>()?;
  let mut y = data.y.clone();
  let mut x_values = x;
  let mut mean_y = 0.0;
  let mut mean_x = 0.0;
  if constant {
    mean_y = kahan_sum(y.iter().copied()) / n as f64;
    for value in &mut y {
      *value = approx_sub(*value, mean_y);
    }
    mean_x = kahan_sum(x_values.iter().copied()) / n as f64;
    for value in &mut x_values {
      *value = approx_sub(*value, mean_x);
    }
  }
  let sum_xy = kahan_sum(x_values.iter().zip(&y).map(|(x, y)| x * y));
  let sum_x2 = kahan_sum(x_values.iter().map(|value| value * value));
  if sum_x2 == 0.0 {
    return None;
  }
  let slope = sum_xy / sum_x2;
  let intercept = if constant {
    mean_y - slope * mean_x
  } else {
    0.0
  };
  Some(RegressionModel {
    slopes: vec![slope],
    intercept,
  })
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

fn qr_decompose(matrix: &mut [Vec<f64>], k: usize, n: usize) -> Option<Vec<f64>> {
  let mut r_diagonal = vec![0.0; k];
  for column in 0..k {
    let scale = (column..n)
      .map(|row| matrix[row][column].abs())
      .fold(0.0, f64::max);
    if scale == 0.0 {
      return None;
    }
    for row_values in matrix.iter_mut().take(n).skip(column) {
      row_values[column] /= scale;
    }
    let euclid = (column..n)
      .map(|row| matrix[row][column] * matrix[row][column])
      .sum::<f64>()
      .sqrt();
    let factor = 1.0 / euclid / (euclid + matrix[column][column].abs());
    let sign = if matrix[column][column] >= 0.0 {
      1.0
    } else {
      -1.0
    };
    matrix[column][column] += sign * euclid;
    r_diagonal[column] = -sign * scale * euclid;
    for c in column + 1..k {
      let sum = (column..n)
        .map(|row| matrix[row][column] * matrix[row][c])
        .sum::<f64>();
      for row_values in matrix.iter_mut().take(n).skip(column) {
        row_values[c] -= sum * factor * row_values[column];
      }
    }
  }
  Some(r_diagonal)
}

fn apply_householder(matrix: &[Vec<f64>], column: usize, values: &mut [f64], n: usize) {
  let denominator = (column..n)
    .map(|row| matrix[row][column] * matrix[row][column])
    .sum::<f64>();
  let numerator = (column..n)
    .map(|row| matrix[row][column] * values[row])
    .sum::<f64>();
  let factor = if denominator == 0.0 {
    0.0
  } else {
    2.0 * numerator / denominator
  };
  for row in column..n {
    values[row] -= factor * matrix[row][column];
  }
}

fn solve_upper(matrix: &[Vec<f64>], diagonal: &[f64], values: &mut [f64], k: usize) {
  for row in (0..k).rev() {
    let mut sum = values[row];
    for column in row + 1..k {
      sum -= matrix[row][column] * values[column];
    }
    values[row] = sum / diagonal[row];
  }
}

fn solve_lower(matrix: &[Vec<f64>], diagonal: &[f64], values: &mut [f64], k: usize) {
  for row in 0..k {
    let mut sum = values[row];
    for column in 0..row {
      sum -= matrix[column][row] * values[column];
    }
    values[row] = sum / diagonal[row];
  }
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

fn lookup_result_value(value: FormulaValue<'_>) -> FormulaValue<'_> {
  match value {
    FormulaValue::Blank => FormulaValue::Number(0.0),
    value => value,
  }
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

fn parse_criteria_operator(value: &str) -> (QueryOp, &str) {
  if let Some(rest) = value.strip_prefix("<>") {
    (QueryOp::NotEqual, rest)
  } else if let Some(rest) = value.strip_prefix("<=") {
    (QueryOp::LessOrEqual, rest)
  } else if let Some(rest) = value.strip_prefix(">=") {
    (QueryOp::GreaterOrEqual, rest)
  } else if let Some(rest) = value.strip_prefix('=') {
    (QueryOp::Equal, rest)
  } else if let Some(rest) = value.strip_prefix('<') {
    (QueryOp::Less, rest)
  } else if let Some(rest) = value.strip_prefix('>') {
    (QueryOp::Greater, rest)
  } else {
    (QueryOp::Equal, value)
  }
}

fn wildcard_match(pattern: &str, text: &str) -> bool {
  let pattern = lookup_search_key(pattern).chars().collect::<Vec<_>>();
  let text = lookup_search_key(text).chars().collect::<Vec<_>>();
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

fn may_be_wildcard(value: &str) -> bool {
  value.chars().any(|ch| matches!(ch, '*' | '?' | '~'))
}

fn may_be_regex(value: &str) -> bool {
  if value.is_empty() || (value.chars().count() == 1 && value != ".") {
    return false;
  }
  value.chars().any(|ch| {
    matches!(
      ch,
      '?' | '*' | '+' | '.' | '[' | ']' | '^' | '$' | '\\' | '<' | '>' | '(' | ')' | '|'
    )
  })
}

fn detect_query_search_type(search_type: FormulaSearchType, value: &str) -> QuerySearchType {
  match search_type {
    FormulaSearchType::Wildcard if may_be_wildcard(value) => QuerySearchType::Wildcard,
    FormulaSearchType::Regex if may_be_regex(value) => QuerySearchType::Regex,
    _ => QuerySearchType::Normal,
  }
}

fn regex_match(pattern: &str, text: &str, whole_cell: bool) -> Option<bool> {
  let pattern = if whole_cell {
    Cow::Owned(format!("^(?:{pattern})$"))
  } else {
    Cow::Borrowed(pattern)
  };
  RegexBuilder::new(pattern.as_ref())
    .case_insensitive(true)
    .build()
    .ok()
    .map(|regex| regex.is_match(text))
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
  if k < 1.0 || values.is_empty() {
    return None;
  }
  values.sort_by(f64::total_cmp);
  if descending {
    values.reverse();
  }
  values.get(k as usize - 1).copied()
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
      '\u{FFE5}' => output.push('\\'),
      '\u{FF01}'..='\u{FF5E}' => {
        output.push(char::from_u32(ch as u32 - 0xFEE0).unwrap_or(ch));
      }
      _ => {
        if let Some((base, mark)) = decompose_katakana_mark(ch) {
          output.push(base);
          output.push(mark);
        } else {
          output.push(full_katakana_to_half(ch).unwrap_or(ch));
        }
      }
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

fn decompose_katakana_mark(ch: char) -> Option<(char, char)> {
  Some(match ch {
    '\u{30F4}' => ('\u{FF73}', '\u{FF9E}'),
    '\u{30AC}' => ('\u{FF76}', '\u{FF9E}'),
    '\u{30AE}' => ('\u{FF77}', '\u{FF9E}'),
    '\u{30B0}' => ('\u{FF78}', '\u{FF9E}'),
    '\u{30B2}' => ('\u{FF79}', '\u{FF9E}'),
    '\u{30B4}' => ('\u{FF7A}', '\u{FF9E}'),
    '\u{30B6}' => ('\u{FF7B}', '\u{FF9E}'),
    '\u{30B8}' => ('\u{FF7C}', '\u{FF9E}'),
    '\u{30BA}' => ('\u{FF7D}', '\u{FF9E}'),
    '\u{30BC}' => ('\u{FF7E}', '\u{FF9E}'),
    '\u{30BE}' => ('\u{FF7F}', '\u{FF9E}'),
    '\u{30C0}' => ('\u{FF80}', '\u{FF9E}'),
    '\u{30C2}' => ('\u{FF81}', '\u{FF9E}'),
    '\u{30C5}' => ('\u{FF82}', '\u{FF9E}'),
    '\u{30C7}' => ('\u{FF83}', '\u{FF9E}'),
    '\u{30C9}' => ('\u{FF84}', '\u{FF9E}'),
    '\u{30D0}' => ('\u{FF8A}', '\u{FF9E}'),
    '\u{30D3}' => ('\u{FF8B}', '\u{FF9E}'),
    '\u{30D6}' => ('\u{FF8C}', '\u{FF9E}'),
    '\u{30D9}' => ('\u{FF8D}', '\u{FF9E}'),
    '\u{30DC}' => ('\u{FF8E}', '\u{FF9E}'),
    '\u{30D1}' => ('\u{FF8A}', '\u{FF9F}'),
    '\u{30D4}' => ('\u{FF8B}', '\u{FF9F}'),
    '\u{30D7}' => ('\u{FF8C}', '\u{FF9F}'),
    '\u{30DA}' => ('\u{FF8D}', '\u{FF9F}'),
    '\u{30DD}' => ('\u{FF8E}', '\u{FF9F}'),
    _ => return None,
  })
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

fn date_diff_months(start: i32, end: i32, mode: i32) -> Option<i32> {
  let (start_year, start_month, start_day) = date_from_serial(start)?;
  let (end_year, end_month, end_day) = date_from_serial(end)?;
  let mut result = end_month as i32 - start_month as i32 + (end_year - start_year) * 12;
  if mode == 1 || start == end {
    return Some(result);
  }
  if start < end {
    if start_day > end_day {
      result -= 1;
    }
  } else if start_day < end_day {
    result += 1;
  }
  Some(result)
}

fn date_diff_years(start: i32, end: i32, mode: i32) -> Option<i32> {
  if mode != 1 {
    return date_diff_months(start, end, mode).map(|months| months / 12);
  }
  let (start_year, _, _) = date_from_serial(start)?;
  let (end_year, _, _) = date_from_serial(end)?;
  Some(end_year - start_year)
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
  if x == 0.0 {
    return if order == 0 { 1.0 } else { 0.0 };
  }
  let sign = if order % 2 == 1 && x < 0.0 { -1.0 } else { 1.0 };
  let x = x.abs();
  let max_iteration = 9_000_000.0;
  let estimate_iteration = x * 1.5 + f64::from(order);
  let asymptotic_possible = x.powf(0.4) > f64::from(order);
  if estimate_iteration > max_iteration {
    if !asymptotic_possible {
      return f64::NAN;
    }
    return sign
      * (std::f64::consts::FRAC_2_PI / x).sqrt()
      * (x - f64::from(order) * std::f64::consts::FRAC_PI_2 - std::f64::consts::FRAC_PI_4).cos();
  }

  let epsilon = 1.0e-15;
  let mut k;
  let mut u;
  let mut m_bar;
  let mut g_bar;
  let mut g_bar_delta_u;
  let mut g = 0.0;
  let mut delta_u = 0.0;
  let mut f_bar = -1.0;

  if order == 0 {
    u = 1.0;
    g_bar_delta_u = 0.0;
    g_bar = -2.0 / x;
    delta_u = g_bar_delta_u / g_bar;
    u += delta_u;
    g = -1.0 / g_bar;
    f_bar *= g;
    k = 2.0;
  } else {
    u = 0.0;
    k = 1.0;
    while k <= f64::from(order - 1) {
      m_bar = 2.0 * (k - 1.0).rem_euclid(2.0) * f_bar;
      g_bar_delta_u = -g * delta_u - m_bar * u;
      g_bar = m_bar - 2.0 * k / x + g;
      delta_u = g_bar_delta_u / g_bar;
      u += delta_u;
      g = -1.0 / g_bar;
      f_bar *= g;
      k += 1.0;
    }
    m_bar = 2.0 * (k - 1.0).rem_euclid(2.0) * f_bar;
    g_bar_delta_u = f_bar - g * delta_u - m_bar * u;
    g_bar = m_bar - 2.0 * k / x + g;
    delta_u = g_bar_delta_u / g_bar;
    u += delta_u;
    g = -1.0 / g_bar;
    f_bar *= g;
    k += 1.0;
  }

  loop {
    m_bar = 2.0 * (k - 1.0).rem_euclid(2.0) * f_bar;
    g_bar_delta_u = -g * delta_u - m_bar * u;
    g_bar = m_bar - 2.0 * k / x + g;
    delta_u = g_bar_delta_u / g_bar;
    u += delta_u;
    g = -1.0 / g_bar;
    f_bar *= g;
    if delta_u.abs() <= u.abs() * epsilon {
      return u * sign;
    }
    k += 1.0;
    if k > max_iteration {
      return f64::NAN;
    }
  }
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
  date_serial_with_system(year, month, day, DateSystem::Date1900)
}

fn date_serial_with_system(
  year: i32,
  month: i32,
  day: i32,
  date_system: DateSystem,
) -> Option<f64> {
  let month_index = month - 1;
  let normalized_year = year + month_index.div_euclid(12);
  let normalized_month = month_index.rem_euclid(12) + 1;
  let days = days_from_civil(normalized_year, normalized_month, 1)? + i64::from(day - 1);
  let serial = match date_system {
    DateSystem::Date1900 => {
      let base = days_from_civil(1899, 12, 31)?;
      let mut serial = days - base;
      let leap_bug_start = days_from_civil(1900, 3, 1)?;
      if days >= leap_bug_start {
        serial += 1;
      }
      serial
    }
    DateSystem::Date1904 => days - days_from_civil(1904, 1, 1)?,
    DateSystem::LibreOffice => days - days_from_civil(1899, 12, 30)?,
  };
  Some(serial as f64)
}

fn date_from_serial(serial: i32) -> Option<(i32, u32, u32)> {
  date_from_serial_with_system(serial, DateSystem::Date1900)
}

fn date_from_serial_with_system(serial: i32, date_system: DateSystem) -> Option<(i32, u32, u32)> {
  match date_system {
    DateSystem::Date1900 => {
      if serial == 60 {
        return Some((1900, 2, 29));
      }
      let base = days_from_civil(1899, 12, 31)?;
      let adjusted = if serial > 60 { serial - 1 } else { serial };
      civil_from_days(base + i64::from(adjusted))
    }
    DateSystem::Date1904 => civil_from_days(days_from_civil(1904, 1, 1)? + i64::from(serial)),
    DateSystem::LibreOffice => civil_from_days(days_from_civil(1899, 12, 30)? + i64::from(serial)),
  }
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

fn weeks_mode_one_index(serial: i32, date_system: DateSystem) -> Option<i64> {
  // Source: LibreOffice ScaDateAddIn::getDiffWeeks:
  // floor((serial + NullDate - 1) / 7.0), with NullDate in DateToDays units.
  let null_date_days = match date_system {
    DateSystem::Date1900 | DateSystem::LibreOffice => date_to_days(1899, 12, 30)?,
    DateSystem::Date1904 => date_to_days(1904, 1, 1)?,
  };
  Some((i64::from(serial) + null_date_days - 1).div_euclid(7))
}

fn date_to_days(year: i32, month: i32, day: i32) -> Option<i64> {
  // Source: LibreOffice scaddins/source/datefunc/datefunc.cxx DateToDays.
  Some(days_from_civil(year, month, day)? - days_from_civil(1, 1, 1)? + 1)
}

fn weeks_in_year_from_serial_with_system(serial: i32, date_system: DateSystem) -> Option<i32> {
  let (year, _, _) = date_from_serial_with_system(serial, date_system)?;
  iso_weeks_in_year(year)
}

fn iso_weeknum_from_serial_with_system(serial: i32, date_system: DateSystem) -> Option<i32> {
  let (year, month, day) = date_from_serial_with_system(serial, date_system)?;
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
  let chars = value.chars().collect::<Vec<_>>();
  let mut output = String::with_capacity(value.len());
  let mut index = 0usize;
  while index < chars.len() {
    if legacy_c1_text_len(&chars[index..]).is_some() {
      index += 2;
      continue;
    }
    let ch = chars[index];
    if !ch.is_control() && !is_unicode_noncharacter(ch) {
      output.push(ch);
    }
    index += 1;
  }
  output
}

fn legacy_char_text(code: f64) -> Option<String> {
  if !(0.0..256.0).contains(&code) {
    return None;
  }
  let byte = code as u8;
  let bytes = [byte];
  let (text, _had_errors) = WINDOWS_1252.decode_without_bom_handling(&bytes);
  Some(text.into_owned())
}

fn legacy_text_code(ch: char) -> u8 {
  let text = ch.to_string();
  let (bytes, _encoding, had_errors) = WINDOWS_1252.encode(&text);
  if had_errors {
    b'?'
  } else {
    bytes.first().copied().unwrap_or(0)
  }
}

fn legacy_c1_text_len(chars: &[char]) -> Option<usize> {
  if chars.len() < 2 || chars.first().copied()? != '\u{00c2}' {
    return None;
  }
  (0x80..=0x9f)
    .filter_map(|byte| legacy_char_text(byte as f64))
    .filter_map(|text| text.chars().next())
    .any(|ch| chars.get(1).copied() == Some(ch))
    .then_some(2)
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
    output.push_str(&format_complex_component(real, false));
  }
  if has_imaginary {
    if imaginary == 1.0 {
      if has_real {
        output.push('+');
      }
    } else if imaginary == -1.0 {
      output.push('-');
    } else {
      output.push_str(&format_complex_component(imaginary, has_real));
    }
    output.push(suffix);
  }
  output
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
  lo_gamma(value).unwrap_or_else(|_| statrs_gamma::gamma(value))
}

fn log_gamma(z: f64) -> f64 {
  lo_log_gamma(z)
}

const LO_MAX_GAMMA_ARGUMENT: f64 = 171.624376956302;
const LO_LANCZOS_G: f64 = 6.024_680_040_776_73;
const LO_HALF_MACH_EPS: f64 = 0.5 * f64::EPSILON;

fn lo_lanczos_sum(z: f64) -> f64 {
  const NUM: [f64; 13] = [
    23_531_376_880.410_76,
    42_919_803_642.649_1,
    35_711_959_237.355_67,
    17_921_034_426.037_21,
    6_039_542_586.352_028,
    1_439_720_407.311_721_6,
    248_874_557.862_054_17,
    31_426_415.585_400_194,
    2_876_370.628_935_372_5,
    186_056.265_395_223_48,
    8_071.672_002_365_816,
    210.824_277_751_579_36,
    2.506_628_274_631_000_2,
  ];
  const DENOM: [f64; 13] = [
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
  if z <= 1.0 {
    let mut sum_num = NUM[12];
    let mut sum_denom = DENOM[12];
    for index in (0..12).rev() {
      sum_num = sum_num * z + NUM[index];
      sum_denom = sum_denom * z + DENOM[index];
    }
    sum_num / sum_denom
  } else {
    let z_inv = 1.0 / z;
    let mut sum_num = NUM[0];
    let mut sum_denom = DENOM[0];
    for index in 1..=12 {
      sum_num = sum_num * z_inv + NUM[index];
      sum_denom = sum_denom * z_inv + DENOM[index];
    }
    sum_num / sum_denom
  }
}

fn lo_gamma_helper(z: f64) -> f64 {
  let z_g = z + LO_LANCZOS_G - 0.5;
  let half_power = z_g.powf(z / 2.0 - 0.25);
  let mut result = lo_lanczos_sum(z);
  result *= half_power;
  result /= z_g.exp();
  result *= half_power;
  if z <= 20.0 && z == approx_floor(z) {
    result = result.round();
  }
  result
}

fn lo_log_gamma_helper(z: f64) -> f64 {
  let z_g = z + LO_LANCZOS_G - 0.5;
  lo_lanczos_sum(z).ln() + (z - 0.5) * z_g.ln() - z_g
}

fn lo_gamma(z: f64) -> std::result::Result<f64, FormulaErrorValue> {
  let log_pi = std::f64::consts::PI.ln();
  let log_dbl_max = f64::MAX.ln();
  if z > LO_MAX_GAMMA_ARGUMENT {
    return Err(FormulaErrorValue::IllegalArgument);
  }
  if z >= 1.0 {
    return Ok(lo_gamma_helper(z));
  }
  if z >= 0.5 {
    return Ok(lo_gamma_helper(z + 1.0) / z);
  }
  if z >= -0.5 {
    let log_test = lo_log_gamma_helper(z + 2.0) - z.ln_1p() - z.abs().ln();
    if log_test >= log_dbl_max {
      return Err(FormulaErrorValue::IllegalArgument);
    }
    return Ok(lo_gamma_helper(z + 2.0) / (z + 1.0) / z);
  }
  let sin_pi_z = (std::f64::consts::PI * z).sin();
  let log_divisor = lo_log_gamma_helper(1.0 - z) + sin_pi_z.abs().ln();
  if log_divisor - log_pi >= log_dbl_max {
    return Ok(0.0);
  }
  if log_divisor < 0.0 && log_pi - log_divisor > log_dbl_max {
    return Err(FormulaErrorValue::IllegalArgument);
  }
  Ok((log_pi - log_divisor).exp() * if sin_pi_z < 0.0 { -1.0 } else { 1.0 })
}

fn lo_log_gamma(z: f64) -> f64 {
  if z >= LO_MAX_GAMMA_ARGUMENT {
    return lo_log_gamma_helper(z);
  }
  if z >= 1.0 {
    return lo_gamma_helper(z).ln();
  }
  if z >= 0.5 {
    return (lo_gamma_helper(z + 1.0) / z).ln();
  }
  lo_log_gamma_helper(z + 2.0) - z.ln_1p() - z.ln()
}

fn lo_beta(alpha: f64, beta: f64) -> f64 {
  let (a, b) = if alpha > beta {
    (alpha, beta)
  } else {
    (beta, alpha)
  };
  if a + b < LO_MAX_GAMMA_ARGUMENT {
    return lo_gamma(a).unwrap_or(f64::INFINITY) / lo_gamma(a + b).unwrap_or(f64::INFINITY)
      * lo_gamma(b).unwrap_or(f64::INFINITY);
  }
  lo_log_beta(a, b).exp()
}

fn lo_log_beta(alpha: f64, beta: f64) -> f64 {
  let (a, b) = if alpha > beta {
    (alpha, beta)
  } else {
    (beta, alpha)
  };
  let g_minus_half = LO_LANCZOS_G - 0.5;
  let lanczos = lo_lanczos_sum(a) / lo_lanczos_sum(a + b) * lo_lanczos_sum(b);
  let ab_g = a + b + g_minus_half;
  let log_lanczos =
    lanczos.ln() + 0.5 * (ab_g.ln() - (a + g_minus_half).ln() - (b + g_minus_half).ln());
  -a * (b / (a + g_minus_half)).ln_1p() - b * (a / (b + g_minus_half)).ln_1p() - g_minus_half
    + log_lanczos
}

fn lo_beta_dist_pdf(x: f64, a: f64, b: f64) -> std::result::Result<f64, FormulaErrorValue> {
  if a == 1.0 {
    if b == 1.0 {
      return Ok(1.0);
    }
    if b == 2.0 {
      return Ok(-2.0 * x + 2.0);
    }
    if x == 1.0 && b < 1.0 {
      return Err(FormulaErrorValue::IllegalArgument);
    }
    return Ok(if x <= 0.01 {
      b + b * ((b - 1.0) * (-x).ln_1p()).exp_m1()
    } else {
      b * (0.5 - x + 0.5).powf(b - 1.0)
    });
  }
  if b == 1.0 {
    if a == 2.0 {
      return Ok(a * x);
    }
    if x == 0.0 && a < 1.0 {
      return Err(FormulaErrorValue::IllegalArgument);
    }
    return Ok(a * x.powf(a - 1.0));
  }
  if x <= 0.0 {
    if a < 1.0 && x == 0.0 {
      return Err(FormulaErrorValue::IllegalArgument);
    }
    return Ok(0.0);
  }
  if x >= 1.0 {
    if b < 1.0 && x == 1.0 {
      return Err(FormulaErrorValue::IllegalArgument);
    }
    return Ok(0.0);
  }

  let log_dbl_max = f64::MAX.ln();
  let log_dbl_min = f64::MIN_POSITIVE.ln();
  let log_y = if x < 0.1 {
    (-x).ln_1p()
  } else {
    (0.5 - x + 0.5).ln()
  };
  let log_x = x.ln();
  let a_log_x = (a - 1.0) * log_x;
  let b_log_y = (b - 1.0) * log_y;
  let log_beta = lo_log_beta(a, b);
  if a_log_x < log_dbl_max
    && a_log_x > log_dbl_min
    && b_log_y < log_dbl_max
    && b_log_y > log_dbl_min
    && log_beta < log_dbl_max
    && log_beta > log_dbl_min
    && a_log_x + b_log_y < log_dbl_max
    && a_log_x + b_log_y > log_dbl_min
  {
    Ok(x.powf(a - 1.0) * (0.5 - x + 0.5).powf(b - 1.0) / lo_beta(a, b))
  } else {
    Ok((a_log_x + b_log_y - log_beta).exp())
  }
}

fn lo_beta_helper_cont_frac(x: f64, a: f64, b: f64) -> f64 {
  let mut a1 = 1.0;
  let mut b1 = 1.0;
  let mut b2 = 1.0 - (a + b) / (a + 1.0) * x;
  let mut a2;
  let mut fnorm;
  let mut cf;
  if b2 == 0.0 {
    a2 = 0.0;
    fnorm = 1.0;
    cf = 1.0;
  } else {
    a2 = 1.0;
    fnorm = 1.0 / b2;
    cf = a2 * fnorm;
  }
  let mut cf_new = 1.0;
  let mut rm = 1.0;
  let mut finished = false;
  while rm < 50000.0 && !finished {
    let apl2m = a + 2.0 * rm;
    let d2m = rm * (b - rm) * x / ((apl2m - 1.0) * apl2m);
    let d2m1 = -(a + rm) * (a + b + rm) * x / (apl2m * (apl2m + 1.0));
    a1 = (a2 + d2m * a1) * fnorm;
    b1 = (b2 + d2m * b1) * fnorm;
    a2 = a1 + d2m1 * a2 * fnorm;
    b2 = b1 + d2m1 * b2 * fnorm;
    if b2 != 0.0 {
      fnorm = 1.0 / b2;
      cf_new = a2 * fnorm;
      finished = (cf - cf_new).abs() < cf.abs() * f64::EPSILON;
    }
    cf = cf_new;
    rm += 1.0;
  }
  cf
}

fn lo_beta_dist(x_in: f64, alpha: f64, beta: f64) -> f64 {
  if x_in <= 0.0 {
    return 0.0;
  }
  if x_in >= 1.0 {
    return 1.0;
  }
  if beta == 1.0 {
    return x_in.powf(alpha);
  }
  if alpha == 1.0 {
    return -(beta * (-x_in).ln_1p()).exp_m1();
  }
  let mut y = 0.5 - x_in + 0.5;
  let mut ln_y = (-x_in).ln_1p();
  let mut x = x_in;
  let mut ln_x = x_in.ln();
  let mut a = alpha;
  let mut b = beta;
  let reflect = x_in > alpha / (alpha + beta);
  if reflect {
    a = beta;
    b = alpha;
    x = y;
    y = x_in;
    ln_x = ln_y;
    ln_y = x_in.ln();
  }
  let mut result = lo_beta_helper_cont_frac(x, a, b) / a;
  let p = a / (a + b);
  let q = b / (a + b);
  let temp = if a > 1.0 && b > 1.0 && p < 0.97 && q < 0.97 {
    lo_beta_dist_pdf(x, a, b).unwrap_or(f64::INFINITY) * x * y
  } else {
    (a * ln_x + b * ln_y - lo_log_beta(a, b)).exp()
  };
  result *= temp;
  if reflect {
    result = 0.5 - result + 0.5;
  }
  result.clamp(0.0, 1.0)
}

fn lo_binom_dist_pmf(x: f64, n: f64, p: f64) -> f64 {
  let q = (0.5 - p) + 0.5;
  let mut factor = q.powf(n);
  if factor <= f64::MIN_POSITIVE {
    factor = p.powf(n);
    if factor <= f64::MIN_POSITIVE {
      lo_beta_dist_pdf(p, x + 1.0, n - x + 1.0).unwrap_or(f64::INFINITY) / (n + 1.0)
    } else {
      for i in 0..((n - x) as u32) {
        factor *= (n - f64::from(i)) / f64::from(i + 1) * q / p;
      }
      factor
    }
  } else {
    for i in 0..(x as u32) {
      factor *= (n - f64::from(i)) / f64::from(i + 1) * p / q;
    }
    factor
  }
}

fn lo_binom_dist_range(n: f64, xs: f64, xe: f64, mut factor: f64, p: f64, q: f64) -> f64 {
  let xs = xs as u32;
  for i in 1..=xs {
    if factor <= 0.0 {
      break;
    }
    factor *= (n - f64::from(i) + 1.0) / f64::from(i) * p / q;
  }
  let mut values = vec![factor];
  for i in xs + 1..=xe as u32 {
    if factor <= 0.0 {
      break;
    }
    factor *= (n - f64::from(i) + 1.0) / f64::from(i) * p / q;
    values.push(factor);
  }
  kahan_sum(values).min(1.0)
}

fn lo_gamma_cont_fraction(a: f64, x: f64) -> std::result::Result<f64, FormulaErrorValue> {
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
  let mut finished = false;
  while !finished && count < 10000.0 {
    count += 1.0;
    y += 1.0;
    let num = y * count;
    denom += 2.0;
    let pk = pkm1 * denom - pkm2 * num;
    let qk = qkm1 * denom - qkm2 * num;
    if qk != 0.0 {
      let r = pk / qk;
      finished = ((approx - r) / r).abs() <= LO_HALF_MACH_EPS;
      approx = r;
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
  finished.then_some(approx).ok_or(FormulaErrorValue::Num)
}

fn lo_gamma_series(a: f64, x: f64) -> std::result::Result<f64, FormulaErrorValue> {
  let mut denom_factor = a;
  let mut summand = 1.0 / a;
  let mut sum = summand;
  let mut count = 1;
  while summand / sum > LO_HALF_MACH_EPS && count <= 10000 {
    denom_factor += 1.0;
    summand = summand * x / denom_factor;
    sum += summand;
    count += 1;
  }
  (count <= 10000)
    .then_some(sum)
    .ok_or(FormulaErrorValue::Num)
}

fn lo_low_reg_igamma(a: f64, x: f64) -> std::result::Result<f64, FormulaErrorValue> {
  let factor = (a * x.ln() - x - lo_log_gamma(a)).exp();
  if x > a + 1.0 {
    Ok(1.0 - factor * lo_gamma_cont_fraction(a, x)?)
  } else {
    Ok(factor * lo_gamma_series(a, x)?)
  }
}

fn lo_up_reg_igamma(a: f64, x: f64) -> std::result::Result<f64, FormulaErrorValue> {
  let factor = (a * x.ln() - x - lo_log_gamma(a)).exp();
  if x > a + 1.0 {
    Ok(factor * lo_gamma_cont_fraction(a, x)?)
  } else {
    Ok(1.0 - factor * lo_gamma_series(a, x)?)
  }
}

fn lo_gamma_dist_pdf(
  x: f64,
  alpha: f64,
  lambda: f64,
) -> std::result::Result<f64, FormulaErrorValue> {
  if x < 0.0 {
    return Ok(0.0);
  }
  if x == 0.0 {
    if alpha < 1.0 {
      return Err(FormulaErrorValue::Div0);
    }
    if alpha == 1.0 {
      return Ok(1.0 / lambda);
    }
    return Ok(0.0);
  }
  let xr = x / lambda;
  if xr > 1.0 {
    let log_dbl_max = f64::MAX.ln();
    if xr.ln() * (alpha - 1.0) < log_dbl_max && alpha < LO_MAX_GAMMA_ARGUMENT {
      Ok(xr.powf(alpha - 1.0) * (-xr).exp() / lambda / lo_gamma(alpha)?)
    } else {
      Ok(((alpha - 1.0) * xr.ln() - xr - lambda.ln() - lo_log_gamma(alpha)).exp())
    }
  } else if alpha < LO_MAX_GAMMA_ARGUMENT {
    Ok(xr.powf(alpha - 1.0) * (-xr).exp() / lambda / lo_gamma(alpha)?)
  } else {
    Ok(xr.powf(alpha - 1.0) * (-xr).exp() / lambda / lo_log_gamma(alpha).exp())
  }
}

fn lo_gamma_dist(x: f64, alpha: f64, lambda: f64) -> f64 {
  if x <= 0.0 {
    0.0
  } else {
    lo_low_reg_igamma(alpha, x / lambda).unwrap_or(f64::NAN)
  }
}

fn lo_chi_dist(x: f64, df: f64) -> f64 {
  if x <= 0.0 {
    1.0
  } else {
    lo_up_reg_igamma(df / 2.0, x / 2.0).unwrap_or(f64::NAN)
  }
}

fn lo_chisq_dist_cdf(x: f64, df: f64) -> f64 {
  if x <= 0.0 {
    0.0
  } else {
    lo_low_reg_igamma(df / 2.0, x / 2.0).unwrap_or(f64::NAN)
  }
}

fn lo_chisq_dist_pdf(x: f64, df: f64) -> f64 {
  if x <= 0.0 {
    return 0.0;
  }
  if df * x > 1_391_000.0 {
    return (((0.5 * df - 1.0) * (x * 0.5).ln()) - 0.5 * x - 2.0_f64.ln() - lo_log_gamma(0.5 * df))
      .exp();
  }
  let mut value;
  let mut count;
  if df % 2.0 < 0.5 {
    value = 0.5;
    count = 2.0;
  } else {
    value = 1.0 / (x * 2.0 * std::f64::consts::PI).sqrt();
    count = 1.0;
  }
  while count < df {
    value *= x / count;
    count += 2.0;
  }
  if x >= 1425.0 {
    (value.ln() - x / 2.0).exp()
  } else {
    value * (-x / 2.0).exp()
  }
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
  if df < 1.0
    || (right_tail && !(0.0..=1.0).contains(&p))
    || (!right_tail && !(0.0..1.0).contains(&p))
  {
    return FormulaValue::Error(FormulaErrorValue::IllegalArgument);
  }
  let result = if right_tail {
    lo_iterate_inverse(|x| p - lo_chi_dist(x, df), df * 0.5, df)
  } else {
    lo_iterate_inverse(|x| p - lo_chisq_dist_cdf(x, df), df * 0.5, df)
  };
  match result {
    Ok(value) => FormulaValue::Number(value),
    Err(error) => FormulaValue::Error(error),
  }
}

fn lo_has_change_of_sign(left: f64, right: f64) -> bool {
  (left < 0.0 && right > 0.0) || (left > 0.0 && right < 0.0)
}

fn lo_iterate_inverse<F>(
  function: F,
  mut ax: f64,
  mut bx: f64,
) -> std::result::Result<f64, FormulaErrorValue>
where
  F: Fn(f64) -> f64,
{
  let y_eps = 1.0e-307;
  let x_eps = f64::EPSILON;
  let mut ay = function(ax);
  let mut by = function(bx);
  for _ in 0..1000 {
    if lo_has_change_of_sign(ay, by) {
      break;
    }
    if ay.abs() <= by.abs() {
      let previous = ax;
      ax += (ax - bx) * 2.0;
      if ax < 0.0 {
        ax = 0.0;
      }
      bx = previous;
      by = ay;
      ay = function(ax);
    } else {
      let previous = bx;
      bx += (bx - ax) * 2.0;
      ax = previous;
      ay = by;
      by = function(bx);
    }
  }
  if ay == 0.0 {
    return Ok(ax);
  }
  if by == 0.0 {
    return Ok(bx);
  }
  if !lo_has_change_of_sign(ay, by) {
    return Err(FormulaErrorValue::Num);
  }

  let mut px = ax;
  let mut py = ay;
  let mut qx = bx;
  let mut qy = by;
  let mut rx = ax;
  let mut ry = ay;
  let mut sx = 0.5 * (ax + bx);
  let mut interpolate = true;
  let mut count = 0;
  while count < 500 && ry.abs() > y_eps && (bx - ax) > ax.abs().max(bx.abs()) * x_eps {
    if interpolate {
      if py != qy && qy != ry && ry != py {
        sx = px * ry * qy / (ry - py) / (qy - py)
          + rx * qy * py / (qy - ry) / (py - ry)
          + qx * py * ry / (py - qy) / (ry - qy);
        interpolate = ax < sx && sx < bx;
      } else {
        interpolate = false;
      }
    }
    if !interpolate {
      sx = 0.5 * (ax + bx);
      qx = bx;
      qy = by;
      interpolate = true;
    }
    px = qx;
    qx = rx;
    rx = sx;
    py = qy;
    qy = ry;
    ry = function(sx);
    if lo_has_change_of_sign(ay, ry) {
      bx = rx;
      by = ry;
    } else {
      ax = rx;
      ay = ry;
    }
    interpolate = interpolate && ry.abs() * 2.0 <= qy.abs();
    count += 1;
  }
  Ok(rx)
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
  let factor = 10.0_f64.powf(scale.abs());
  if scale < 0.0 {
    input *= factor;
  } else {
    input /= factor;
  }
  let mut result = rtl_round(input, 0);
  if scale < 0.0 {
    result /= factor;
  } else {
    result *= factor;
  }
  result
}

#[derive(Clone, Copy, Debug, Default)]
struct KahanSum {
  sum: f64,
  error: f64,
  memory: f64,
}

impl KahanSum {
  fn add(&mut self, value: f64) {
    if value == 0.0 {
      return;
    }
    if self.memory == 0.0 {
      self.memory = value;
      return;
    }
    Self::sum_neumaier(&mut self.sum, &mut self.error, self.memory);
    self.memory = value;
  }

  fn finish(mut self) -> f64 {
    let total = self.sum + self.error;
    if self.memory == 0.0 {
      return total;
    }
    if ((self.memory < 0.0 && total > 0.0) || (total < 0.0 && self.memory > 0.0))
      && approx_equal(self.memory, -total)
    {
      return 0.0;
    }
    Self::sum_neumaier(&mut self.sum, &mut self.error, self.memory);
    self.memory = 0.0;
    self.sum + self.error
  }

  fn sum_neumaier(sum: &mut f64, error: &mut f64, value: f64) {
    let next = *sum + value;
    if sum.abs() >= value.abs() {
      *error += (*sum - next) + value;
    } else {
      *error += (value - next) + *sum;
    }
    *sum = next;
  }
}

fn kahan_sum(values: impl IntoIterator<Item = f64>) -> f64 {
  let mut sum = KahanSum::default();
  for value in values {
    sum.add(value);
  }
  sum.finish()
}

fn approx_floor(value: f64) -> f64 {
  approx_value(value).floor()
}

fn approx_ceil(value: f64) -> f64 {
  approx_value(value).ceil()
}

fn approx_equal(left: f64, right: f64) -> bool {
  if left == right {
    return true;
  }
  if left == 0.0 || right == 0.0 || left.is_sign_negative() != right.is_sign_negative() {
    return false;
  }
  let difference = (left - right).abs();
  if !difference.is_finite() {
    return false;
  }
  let left = left.abs();
  let right = right.abs();
  let min_value = left.min(right);
  let threshold1 = min_value * 2_f64.powi(-48);
  let threshold2 = pow10_exp(min_value.log10().floor() as i32) * 5.0e-15;
  if difference >= threshold1.max(threshold2) {
    return false;
  }
  !(is_representable_integer(left) && is_representable_integer(right))
}

fn approx_sub(left: f64, right: f64) -> f64 {
  if ((left < 0.0 && right < 0.0) || (left > 0.0 && right > 0.0)) && approx_equal(left, right) {
    return 0.0;
  }
  normalize_duration_difference(left, right, left - right)
}

fn approx_add(left: f64, right: f64) -> f64 {
  // Source: LibreOffice include/rtl/math.hxx rtl::math::approxAdd.
  if ((left < 0.0 && right > 0.0) || (right < 0.0 && left > 0.0)) && approx_equal(left, -right) {
    return 0.0;
  }
  left + right
}

fn normalize_formula_number(value: f64) -> f64 {
  if approx_equal(value, 0.0) { 0.0 } else { value }
}

fn normalize_duration_difference(left: f64, right: f64, value: f64) -> f64 {
  // Source: LibreOffice sc/source/core/tool/interpr5.cxx
  // ScInterpreter::CalculateAddSub uses tools::Duration with microsecond
  // accuracy when date/duration formatted values participate in subtraction.
  if value != 0.0
    && value.abs() < 1.0
    && (left.abs() >= 1.0 || right.abs() >= 1.0)
    && (left - right).abs() <= i32::MAX as f64
  {
    let micros_per_day = 86_400_000_000.0;
    return (value * micros_per_day).round() / micros_per_day;
  }
  value
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

fn pow10_exp(exp: i32) -> f64 {
  10_f64.powi(exp)
}

fn is_representable_integer(value: f64) -> bool {
  // Source: LibreOffice sal/rtl/math.cxx isRepresentableInteger.
  value.is_finite() && value >= 0.0 && value < 2_f64.powi(53) && value.fract() == 0.0
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
    FormulaValue::Matrix(_) | FormulaValue::Reference(_) | FormulaValue::RefList(_) => {
      String::new()
    }
  }
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

fn round_half_away_from_zero(value: f64, digits: i32) -> f64 {
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
  let rounded = if scaled.is_sign_negative() {
    (scaled - 0.5).ceil()
  } else {
    (scaled + 0.5).floor()
  };
  if digits < 0 {
    rounded * factor
  } else {
    rounded / factor
  }
}

fn timevalue(text: &str) -> FormulaValue<'static> {
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

fn financial_pv(rate: f64, nper: f64, pmt: f64, fv: f64, pay_in_advance: bool) -> f64 {
  let pv = if rate == 0.0 {
    fv + pmt * nper
  } else if pay_in_advance {
    fv * (1.0 + rate).powf(-nper) + pmt * (1.0 - (1.0 + rate).powf(-nper + 1.0)) / rate + pmt
  } else {
    fv * (1.0 + rate).powf(-nper) + pmt * (1.0 - (1.0 + rate).powf(-nper)) / rate
  };
  -pv
}

fn financial_nper(rate: f64, pmt: f64, pv: f64, fv: f64, pay_in_advance: bool) -> Option<f64> {
  if pv + fv == 0.0 {
    return Some(0.0);
  }
  if rate == 0.0 {
    return Some(-(pv + fv) / pmt);
  }
  let numerator = if pay_in_advance {
    -(rate * fv - pmt * (1.0 + rate)) / (rate * pv + pmt * (1.0 + rate))
  } else {
    -(rate * fv - pmt) / (rate * pv + pmt)
  };
  (numerator > 0.0).then(|| numerator.ln() / rate.ln_1p())
}

fn financial_fv(rate: f64, nper: f64, pmt: f64, pv: f64, pay_in_advance: bool) -> f64 {
  let fv = if rate == 0.0 {
    pv + pmt * nper
  } else if pay_in_advance {
    pv * (nper * rate.ln_1p()).exp() + pmt * (1.0 + rate) * (nper * rate.ln_1p()).exp_m1() / rate
  } else {
    pv * (nper * rate.ln_1p()).exp() + pmt * (nper * rate.ln_1p()).exp_m1() / rate
  };
  -fv
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

fn finance_price(
  settle: i32,
  maturity: i32,
  rate: f64,
  yield_value: f64,
  redemption: f64,
  frequency: i32,
  basis: i32,
) -> Option<f64> {
  let frequency_value = f64::from(frequency);
  let coupon_days = finance_coupdays(settle, maturity, frequency, basis)?;
  let dsc_e = finance_coupdaysnc(settle, maturity, frequency, basis)? / coupon_days;
  let coupon_count = finance_coupnum(settle, maturity, frequency, basis)?;
  let accrued_days = finance_coupdaybs(settle, maturity, frequency, basis)?;

  let discount = 1.0 + yield_value / frequency_value;
  let mut price = redemption / discount.powf(coupon_count - 1.0 + dsc_e);
  price -= 100.0 * rate / frequency_value * accrued_days / coupon_days;

  let coupon = 100.0 * rate / frequency_value;
  let mut k = 0.0;
  while k < coupon_count {
    price += coupon / discount.powf(k + dsc_e);
    k += 1.0;
  }
  Some(price)
}

#[derive(Clone, Copy)]
struct OddPeriodArgs {
  settle: i32,
  maturity: i32,
  last_coupon: i32,
  rate: f64,
  value: f64,
  redemption: f64,
  frequency: i32,
  basis: i32,
}

fn financial_oddlprice(args: OddPeriodArgs) -> Option<f64> {
  let frequency = f64::from(args.frequency);
  let dci = yearfrac(args.last_coupon, args.maturity, args.basis)? * frequency;
  let dsci = yearfrac(args.settle, args.maturity, args.basis)? * frequency;
  let ai = yearfrac(args.last_coupon, args.settle, args.basis)? * frequency;
  let mut price = args.redemption + dci * 100.0 * args.rate / frequency;
  price /= dsci * args.value / frequency + 1.0;
  price -= ai * 100.0 * args.rate / frequency;
  Some(price)
}

fn finance_yield(
  settle: i32,
  maturity: i32,
  coupon: f64,
  price: f64,
  redemption: f64,
  frequency: i32,
  basis: i32,
) -> Option<f64> {
  let rate = coupon;
  let mut price_n = 0.0;
  let mut yield_1 = 0.0;
  let mut yield_2 = 1.0;
  let mut price_1 = finance_price(
    settle, maturity, rate, yield_1, redemption, frequency, basis,
  )?;
  let mut price_2 = finance_price(
    settle, maturity, rate, yield_2, redemption, frequency, basis,
  )?;
  let mut yield_n = (yield_2 - yield_1) * 0.5;

  for _ in 0..100 {
    if approx_equal(price_n, price) {
      break;
    }
    price_n = finance_price(
      settle, maturity, rate, yield_n, redemption, frequency, basis,
    )?;
    if approx_equal(price, price_1) {
      return Some(yield_1);
    } else if approx_equal(price, price_2) {
      return Some(yield_2);
    } else if approx_equal(price, price_n) {
      return Some(yield_n);
    } else if price < price_2 {
      yield_2 *= 2.0;
      price_2 = finance_price(
        settle, maturity, rate, yield_2, redemption, frequency, basis,
      )?;
      yield_n = (yield_2 - yield_1) * 0.5;
    } else {
      if price < price_n {
        yield_1 = yield_n;
        price_1 = price_n;
      } else {
        yield_2 = yield_n;
        price_2 = price_n;
      }
      yield_n = yield_2 - (yield_2 - yield_1) * ((price - price_2) / (price_1 - price_2));
    }
  }

  if (price - price_n).abs() > price / 100.0 {
    None
  } else {
    Some(yield_n)
  }
}

fn financial_oddlyield(args: OddPeriodArgs) -> Option<f64> {
  let frequency = f64::from(args.frequency);
  let dci = yearfrac(args.last_coupon, args.maturity, args.basis)? * frequency;
  let dsci = yearfrac(args.settle, args.maturity, args.basis)? * frequency;
  let ai = yearfrac(args.last_coupon, args.settle, args.basis)? * frequency;
  let mut yield_value = args.redemption + dci * 100.0 * args.rate / frequency;
  yield_value /= args.value + ai * 100.0 * args.rate / frequency;
  yield_value -= 1.0;
  Some(yield_value * frequency / dsci)
}

fn financial_amordegrc(
  cost: f64,
  date: i32,
  first_period: i32,
  residual: f64,
  period: f64,
  rate: f64,
  basis: i32,
) -> Option<f64> {
  let period = period as u32;
  let use_period = 1.0 / rate;
  let coefficient = if use_period < 3.0 {
    1.0
  } else if use_period < 5.0 {
    1.5
  } else if use_period <= 6.0 {
    2.0
  } else {
    2.5
  };
  let rate = rate * coefficient;
  let mut depreciation = (yearfrac(date, first_period, basis)? * rate * cost).round();
  let mut cost = cost - depreciation;
  let mut rest = cost - residual;
  for index in 0..period {
    depreciation = (rate * cost).round();
    rest -= depreciation;
    if rest < 0.0 {
      return Some(match period - index {
        0 | 1 => (cost * 0.5).round(),
        _ => 0.0,
      });
    }
    cost -= depreciation;
  }
  Some(depreciation)
}

fn financial_amorlinc(
  cost: f64,
  date: i32,
  first_period: i32,
  residual: f64,
  period: f64,
  rate: f64,
  basis: i32,
) -> Option<f64> {
  let period = period as u32;
  let one_rate = cost * rate;
  let cost_delta = cost - residual;
  let first_rate = yearfrac(date, first_period, basis)? * rate * cost;
  let full_periods = ((cost - residual - first_rate) / one_rate) as u32;
  let result = if period == 0 {
    first_rate
  } else if period <= full_periods {
    one_rate
  } else if period == full_periods + 1 {
    cost_delta - one_rate * f64::from(full_periods) - first_rate
  } else {
    0.0
  };
  Some(result.max(0.0))
}

fn finance_duration(
  settle: i32,
  maturity: i32,
  coupon: f64,
  yield_value: f64,
  frequency: i32,
  basis: i32,
) -> Option<f64> {
  let yearfrac = yearfrac(settle, maturity, basis)?;
  let coupon_count = finance_coupnum(settle, maturity, frequency, basis)?;
  let coupon_cash = coupon * 100.0 / f64::from(frequency);
  let discount = 1.0 + yield_value / f64::from(frequency);
  let diff = yearfrac * f64::from(frequency) - coupon_count;

  let mut duration = 0.0;
  let mut period = 1.0;
  while period < coupon_count {
    duration += (period + diff) * coupon_cash / discount.powf(period + diff);
    period += 1.0;
  }
  duration += (coupon_count + diff) * (coupon_cash + 100.0) / discount.powf(coupon_count + diff);

  let mut price = 0.0;
  let mut period = 1.0;
  while period < coupon_count {
    price += coupon_cash / discount.powf(period + diff);
    period += 1.0;
  }
  price += (coupon_cash + 100.0) / discount.powf(coupon_count + diff);
  Some(duration / price / f64::from(frequency))
}

fn finance_year_diff(start: i32, end: i32, basis: i32) -> Option<f64> {
  let (start, end, negative) = if start > end {
    (end, start, true)
  } else {
    (start, end, false)
  };
  let (year1, month1, day1) = date_from_serial(start)?;
  let (year2, month2, day2) = date_from_serial(end)?;
  let (day_count, year_days) = match basis {
    0 | 4 => {
      let months = month2 as i32 - month1 as i32 + (year2 - year1) * 12;
      let mut days = day2 as i32 - day1 as i32;
      let mut total = months * 30 + days;
      if basis == 0 && month1 == 2 && month2 != 2 && year1 == year2 {
        days = if is_leap_year(year1) { 1 } else { 2 };
        total -= days;
      }
      (total, 360)
    }
    1 => {
      let year_days = if is_leap_year(year1) { 366 } else { 365 };
      (end - start, year_days)
    }
    2 => (end - start, 360),
    3 => (end - start, 365),
    _ => return None,
  };
  let result = f64::from(day_count) / f64::from(year_days);
  Some(if negative { -result } else { result })
}

fn financial_xnpv(rate: f64, values: &[f64], dates: &[f64]) -> Option<f64> {
  if values.len() != dates.len() || values.len() < 2 {
    return None;
  }
  let base = dates[0];
  let rate = rate + 1.0;
  let result = values
    .iter()
    .zip(dates)
    .map(|(value, date)| value / rate.powf((date - base) / 365.0))
    .sum::<f64>();
  result.is_finite().then_some(result)
}

fn financial_xirr(values: &[f64], dates: &[f64], guess: f64) -> Option<f64> {
  if values.len() != dates.len() || values.len() < 2 || guess <= -1.0 {
    return None;
  }
  const MAX_EPS: f64 = 1e-10;
  const MAX_ITER: usize = 50;
  let mut scan = 0usize;
  let mut rate = guess;
  loop {
    if scan >= 1 {
      rate = -0.99 + (scan - 1) as f64 * 0.01;
    }
    let mut iter = 0usize;
    let mut cont;
    let mut value;
    loop {
      value = financial_xirr_result(values, dates, rate);
      let derivative = financial_xirr_derivative(values, dates, rate);
      let new_rate = if derivative == 0.0 {
        f64::NAN
      } else {
        rate - value / derivative
      };
      let rate_eps = (new_rate - rate).abs();
      rate = new_rate;
      cont = rate_eps > MAX_EPS && value.abs() > MAX_EPS;
      if !cont || iter >= MAX_ITER {
        break;
      }
      iter += 1;
    }
    if !rate.is_finite() || !value.is_finite() {
      cont = true;
    }
    scan += 1;
    if !cont {
      return rate.is_finite().then_some(rate);
    }
    if scan >= 200 {
      return None;
    }
  }
}

fn financial_xirr_result(values: &[f64], dates: &[f64], rate: f64) -> f64 {
  let base = dates[0];
  let r = rate + 1.0;
  let mut result = values[0];
  for index in 1..values.len() {
    result += values[index] / r.powf((dates[index] - base) / 365.0);
  }
  result
}

fn financial_xirr_derivative(values: &[f64], dates: &[f64], rate: f64) -> f64 {
  let base = dates[0];
  let r = rate + 1.0;
  let mut result = 0.0;
  for index in 1..values.len() {
    let exponent = (dates[index] - base) / 365.0;
    result -= exponent * values[index] / r.powf(exponent + 1.0);
  }
  result
}

fn financial_irr(values: &[f64], guess: f64) -> Option<f64> {
  if values.is_empty() {
    return None;
  }
  const EPSILON: f64 = 1.0e-7;
  const MAX_ITER: usize = 20;
  let mut x = if guess == -1.0 { 0.1 } else { guess };
  let mut epsilon = 1.0;
  let mut iterations = 0usize;
  while epsilon > EPSILON && iterations < MAX_ITER {
    let mut numerator = 0.0;
    let mut denominator = 0.0;
    for (index, value) in values.iter().enumerate() {
      let count = index as f64;
      numerator += value / (1.0 + x).powf(count);
      denominator -= count * value / (1.0 + x).powf(count + 1.0);
    }
    let x_new = if denominator == 0.0 {
      f64::NAN
    } else {
      x - numerator / denominator
    };
    iterations += 1;
    epsilon = (x_new - x).abs();
    x = x_new;
    if !x.is_finite() {
      return None;
    }
  }
  if guess == 0.0 && x.abs() < EPSILON {
    x = 0.0;
  }
  (epsilon < EPSILON).then_some(x)
}

fn financial_mirr(values: &[f64], finance_rate: f64, reinvest_rate: f64) -> Option<f64> {
  if values.len() < 2 {
    return None;
  }
  let reinvest = reinvest_rate + 1.0;
  let invest = finance_rate + 1.0;
  let mut npv_reinvest = 0.0;
  let mut pow_reinvest = 1.0;
  let mut npv_invest = 0.0;
  let mut pow_invest = 1.0;
  let mut has_positive = false;
  let mut has_negative = false;
  let mut count = 0usize;
  for value in values {
    if *value > 0.0 {
      has_positive = true;
      npv_reinvest += value * pow_reinvest;
    } else if *value < 0.0 {
      has_negative = true;
      npv_invest += value * pow_invest;
    }
    pow_reinvest /= reinvest;
    pow_invest /= invest;
    count += 1;
  }
  if !(has_positive && has_negative) || npv_invest == 0.0 || count < 2 {
    return None;
  }
  let mut result = -(npv_reinvest / npv_invest);
  result *= reinvest.powf((count - 1) as f64);
  Some(result.powf(1.0 / (count - 1) as f64) - 1.0)
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
      intermediate = rtl_round(intermediate, precision as i32);
    }
    intermediate * to_rate
  };
  if !full_precision && !from.eq_ignore_ascii_case(to) {
    result = rtl_round(result, to_decimals);
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

fn baht_text(value: f64) -> String {
  const TH_ZERO: &str = "ศูนย์";
  const TH_BAHT: &str = "บาท";
  const TH_SATANG: &str = "สตางค์";
  const TH_EXACT: &str = "ถ้วน";
  const TH_MINUS: &str = "ลบ";

  let formatted = format!("{:.2}", value.abs());
  let (baht, satang) = formatted.split_once('.').unwrap_or((&formatted, "00"));
  let no_baht = baht == "0";
  let no_satang = satang == "00";
  if no_baht && no_satang {
    return format!("{TH_ZERO}{TH_BAHT}{TH_EXACT}");
  }

  let mut text = String::new();
  if value < 0.0 {
    text.push_str(TH_MINUS);
  }
  if !no_baht {
    let mut rest = baht;
    let mut block_size = rest.len() % 6;
    if block_size == 0 {
      block_size = 6;
    }
    while !rest.is_empty() {
      let (block, tail) = rest.split_at(block_size);
      append_thai_number_block(&mut text, block);
      rest = tail;
      block_size = 6;
      if !rest.is_empty() {
        text.push_str("ล้าน");
      }
    }
    text.push_str(TH_BAHT);
  }
  if no_satang {
    text.push_str(TH_EXACT);
  } else {
    append_thai_number_block(&mut text, satang);
    text.push_str(TH_SATANG);
  }
  text
}

fn append_thai_number_block(text: &mut String, block: &str) {
  let digits = block.as_bytes();
  for (index, digit) in digits.iter().enumerate() {
    let remaining = digits.len() - index - 1;
    if remaining >= 2 && *digit != b'0' {
      append_thai_digit(text, *digit);
      text.push_str(match remaining {
        2 => "ร้อย",
        3 => "พัน",
        4 => "หมื่น",
        5 => "แสน",
        _ => "",
      });
    }
  }
  let ten = if digits.len() > 1 {
    digits[digits.len() - 2]
  } else {
    b'0'
  };
  let one = *digits.last().unwrap_or(&b'0');
  if ten >= b'1' {
    if ten >= b'3' {
      append_thai_digit(text, ten);
    } else if ten == b'2' {
      text.push_str("ยี่");
    }
    text.push_str("สิบ");
  }
  if ten > b'0' && one == b'1' {
    text.push_str("เอ็ด");
  } else if one > b'0' {
    append_thai_digit(text, one);
  }
}

fn append_thai_digit(text: &mut String, digit: u8) {
  text.push_str(match digit {
    b'1' => "หนึ่ง",
    b'2' => "สอง",
    b'3' => "สาม",
    b'4' => "สี่",
    b'5' => "ห้า",
    b'6' => "หก",
    b'7' => "เจ็ด",
    b'8' => "แปด",
    b'9' => "เก้า",
    _ => "",
  });
}

fn is_coupon_frequency(value: i32) -> bool {
  matches!(value, 1 | 2 | 4)
}

fn finance_couppcd(settle: i32, maturity: i32, frequency: i32, basis: i32) -> Option<i32> {
  if settle >= maturity || !is_coupon_frequency(frequency) {
    return None;
  }
  let settle = FinanceDate::from_serial(settle, basis)?;
  let maturity = FinanceDate::from_serial(maturity, basis)?;
  let mut date = maturity.clone();
  date.set_year(settle.year)?;
  if date < settle {
    date.add_years(1)?;
  }
  while date > settle {
    date.add_months(-12 / frequency)?;
  }
  date.serial()
}

fn finance_coupncd(settle: i32, maturity: i32, frequency: i32, basis: i32) -> Option<i32> {
  if settle >= maturity || !is_coupon_frequency(frequency) {
    return None;
  }
  let settle = FinanceDate::from_serial(settle, basis)?;
  let maturity = FinanceDate::from_serial(maturity, basis)?;
  let mut date = maturity.clone();
  date.set_year(settle.year)?;
  if date > settle {
    date.add_years(-1)?;
  }
  while date <= settle {
    date.add_months(12 / frequency)?;
  }
  date.serial()
}

fn finance_coupdaybs(settle: i32, maturity: i32, frequency: i32, basis: i32) -> Option<f64> {
  if settle >= maturity || !is_coupon_frequency(frequency) {
    return None;
  }
  let settle_date = FinanceDate::from_serial(settle, basis)?;
  let previous =
    FinanceDate::from_serial(finance_couppcd(settle, maturity, frequency, basis)?, basis)?;
  Some(f64::from(FinanceDate::diff(&previous, &settle_date)))
}

fn finance_coupdaysnc(settle: i32, maturity: i32, frequency: i32, basis: i32) -> Option<f64> {
  if settle >= maturity || !is_coupon_frequency(frequency) {
    return None;
  }
  if basis != 0 && basis != 4 {
    let settle_date = FinanceDate::from_serial(settle, basis)?;
    let next =
      FinanceDate::from_serial(finance_coupncd(settle, maturity, frequency, basis)?, basis)?;
    return Some(f64::from(FinanceDate::diff(&settle_date, &next)));
  }
  Some(
    finance_coupdays(settle, maturity, frequency, basis)?
      - finance_coupdaybs(settle, maturity, frequency, basis)?,
  )
}

fn finance_coupdays(settle: i32, maturity: i32, frequency: i32, basis: i32) -> Option<f64> {
  if settle >= maturity || !is_coupon_frequency(frequency) {
    return None;
  }
  if basis == 1 {
    let previous =
      FinanceDate::from_serial(finance_couppcd(settle, maturity, frequency, basis)?, basis)?;
    let mut next = previous.clone();
    next.add_months(12 / frequency)?;
    return Some(f64::from(FinanceDate::diff(&previous, &next)));
  }
  Some(f64::from(finance_days_in_year(basis)?) / f64::from(frequency))
}

fn finance_coupnum(settle: i32, maturity: i32, frequency: i32, basis: i32) -> Option<f64> {
  if settle >= maturity || !is_coupon_frequency(frequency) {
    return None;
  }
  let maturity_date = FinanceDate::from_serial(maturity, basis)?;
  let previous =
    FinanceDate::from_serial(finance_couppcd(settle, maturity, frequency, basis)?, basis)?;
  let months = (maturity_date.year - previous.year) * 12 + maturity_date.month - previous.month;
  Some(f64::from(months * frequency / 12))
}

fn finance_days_in_year(basis: i32) -> Option<i32> {
  match basis {
    0 | 2 | 4 => Some(360),
    1 => Some(365),
    3 => Some(365),
    _ => None,
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FinanceDate {
  orig_day: i32,
  day: i32,
  month: i32,
  year: i32,
  last_day_mode: bool,
  last_day: bool,
  days_30: bool,
  us_mode: bool,
}

impl FinanceDate {
  fn from_serial(serial: i32, basis: i32) -> Option<Self> {
    let (year, month, day) = date_from_serial(serial)?;
    let mut value = Self {
      orig_day: day as i32,
      day: day as i32,
      month: month as i32,
      year,
      last_day_mode: basis != 5,
      last_day: day as i32 >= last_day_of_month(year, month) as i32,
      days_30: matches!(basis, 0 | 4),
      us_mode: basis == 0,
    };
    value.set_day();
    Some(value)
  }

  fn set_day(&mut self) {
    if self.days_30 {
      self.day = self.orig_day.min(30);
      if self.last_day || self.day >= self.days_in_month_for(self.month) {
        self.day = 30;
      }
    } else {
      let last = self.days_in_month_for(self.month);
      self.day = if self.last_day {
        last
      } else {
        self.orig_day.min(last)
      };
    }
  }

  fn days_in_month_for(&self, month: i32) -> i32 {
    if self.days_30 {
      30
    } else {
      last_day_of_month(self.year, month as u32) as i32
    }
  }

  fn days_in_month_range(&self, from: i32, to: i32) -> i32 {
    if from > to {
      return 0;
    }
    if self.days_30 {
      (to - from + 1) * 30
    } else {
      (from..=to).map(|month| self.days_in_month_for(month)).sum()
    }
  }

  fn days_in_year_range(&self, from: i32, to: i32) -> i32 {
    if from > to {
      return 0;
    }
    if self.days_30 {
      (to - from + 1) * 360
    } else {
      (from..=to)
        .map(|year| if is_leap_year(year) { 366 } else { 365 })
        .sum()
    }
  }

  fn set_year(&mut self, year: i32) -> Option<()> {
    if !(0..=0x7fff).contains(&year) {
      return None;
    }
    self.year = year;
    self.set_day();
    Some(())
  }

  fn add_years(&mut self, years: i32) -> Option<()> {
    self.set_year(self.year + years)
  }

  fn add_months(&mut self, months: i32) -> Option<()> {
    let mut new_month = self.month + months;
    if new_month > 12 {
      new_month -= 1;
      self.set_year(self.year + new_month / 12)?;
      self.month = new_month % 12 + 1;
    } else if new_month < 1 {
      self.set_year(self.year + new_month / 12 - 1)?;
      self.month = new_month % 12 + 12;
    } else {
      self.month = new_month;
    }
    self.set_day();
    Some(())
  }

  fn serial(&self) -> Option<i32> {
    let last = last_day_of_month(self.year, self.month as u32) as i32;
    let real_day = if self.last_day_mode && self.last_day {
      last
    } else {
      last.min(self.orig_day)
    };
    date_serial(self.year, self.month, real_day).map(|value| value as i32)
  }

  fn diff(from: &Self, to: &Self) -> i32 {
    if from > to {
      return Self::diff(to, from);
    }
    let mut diff = 0;
    let mut from = from.clone();
    let mut to = to.clone();

    if to.days_30 {
      if to.us_mode {
        if ((from.month == 2) || (from.day < 30)) && to.orig_day == 31 {
          to.day = 31;
        } else if to.month == 2 && to.last_day {
          to.day = last_day_of_month(to.year, 2) as i32;
        }
      } else {
        if from.month == 2 && from.day == 30 {
          from.day = last_day_of_month(from.year, 2) as i32;
        }
        if to.month == 2 && to.day == 30 {
          to.day = last_day_of_month(to.year, 2) as i32;
        }
      }
    }

    if from.year < to.year || (from.year == to.year && from.month < to.month) {
      diff = from.days_in_month_current() - from.day + 1;
      from.orig_day = 1;
      from.day = 1;
      from.last_day = false;
      let _ = from.add_months(1);

      if from.year < to.year {
        diff += from.days_in_month_range(from.month, 12);
        let add = 13 - from.month;
        let _ = from.add_months(add);
        diff += from.days_in_year_range(from.year, to.year - 1);
        let _ = from.add_years(to.year - from.year);
      }

      diff += from.days_in_month_range(from.month, to.month - 1);
      let _ = from.add_months(to.month - from.month);
    }
    diff += to.day - from.day;
    diff.max(0)
  }

  fn days_in_month_current(&self) -> i32 {
    self.days_in_month_for(self.month)
  }
}

impl PartialOrd for FinanceDate {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for FinanceDate {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self
      .year
      .cmp(&other.year)
      .then_with(|| self.month.cmp(&other.month))
      .then_with(|| self.day.cmp(&other.day))
      .then_with(|| match (self.last_day, other.last_day) {
        (left, right) if left != right => left.cmp(&right),
        _ => self.orig_day.cmp(&other.orig_day),
      })
  }
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
    if !approx_equal(start, int_start) || !approx_equal(end, int_end) {
      if !approx_equal(start, int_start) {
        let temp_int_end = int_start + 1.0;
        let temp_value = cost - financial_inter_vdb(cost, salvage, life, life, int_start, factor);
        part += (start - int_start)
          * financial_inter_vdb(
            temp_value,
            salvage,
            life,
            life - int_start,
            temp_int_end - int_start,
            factor,
          );
      }
      if !approx_equal(end, int_end) {
        let temp_int_start = int_end - 1.0;
        let temp_value =
          cost - financial_inter_vdb(cost, salvage, life, life, temp_int_start, factor);
        part += (int_end - end)
          * financial_inter_vdb(
            temp_value,
            salvage,
            life,
            life - temp_int_start,
            int_end - temp_int_start,
            factor,
          );
      }
    }
    let adjusted_cost = cost - financial_inter_vdb(cost, salvage, life, life, int_start, factor);
    vdb = financial_inter_vdb(
      adjusted_cost,
      salvage,
      life,
      life - int_start,
      int_end - int_start,
      factor,
    ) - part;
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

fn datevalue(text: &str, date_system: DateSystem) -> FormulaValue<'static> {
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
  parse_iso_date_input(date, date_system).or_else(|| parse_month_date_input(date, date_system))
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
    0x10000.. => 2,
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
    return token
      .split_once(':')
      .and_then(|(left, right)| {
        let left = QualifiedAddress::parse_a1(sheet, left).ok()?;
        let right = QualifiedAddress::parse_a1(sheet, right).ok()?;
        extend_qualified_range(
          &QualifiedRange {
            sheet,
            sheet_name: left.sheet_name,
            range: CellRange {
              start: left.cell,
              end: left.cell,
            },
            start_flags: left.flags,
            end_flags: left.flags,
          },
          &QualifiedRange {
            sheet,
            sheet_name: right.sheet_name,
            range: CellRange {
              start: right.cell,
              end: right.cell,
            },
            start_flags: right.flags,
            end_flags: right.flags,
          },
        )
      })
      .or_else(|| QualifiedRange::parse_a1(sheet, token).ok());
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

fn parse_reference_prefix_before_intersection<'doc>(
  sheet: SheetId,
  token: &str,
  start: usize,
) -> Option<(QualifiedRange<'doc>, usize)> {
  let mut quoted = false;
  let mut chars = token.char_indices().peekable();
  while let Some((index, ch)) = chars.next() {
    match ch {
      '\'' => {
        if quoted && chars.peek().is_some_and(|(_, next)| *next == '\'') {
          chars.next();
        } else {
          quoted = !quoted;
        }
      }
      '!' if !quoted => {
        let prefix = &token[..index];
        if prefix.contains(':') {
          let range = parse_formula_range(sheet, prefix)?;
          return Some((range, start + index));
        }
      }
      _ => {}
    }
  }
  None
}

fn push_unique_qualified_range<'doc>(
  ranges: &mut Vec<QualifiedRange<'doc>>,
  range: QualifiedRange<'doc>,
) {
  if !ranges.contains(&range) {
    ranges.push(range);
  }
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
    if !quoted && ch == ':' && should_stop_formula_word_at_range_operator(value, index) {
      break;
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

fn should_stop_formula_word_at_range_operator(value: &str, index: usize) -> bool {
  let mut next = index + ':'.len_utf8();
  while next < value.len() {
    let Some(ch) = value[next..].chars().next() else {
      break;
    };
    if !ch.is_whitespace() {
      break;
    }
    next += ch.len_utf8();
  }
  if value[next..].starts_with('(') {
    return true;
  }
  let Some(ch) = value[next..].chars().next() else {
    return false;
  };
  if !(ch.is_ascii_alphabetic() || ch == '_' || ch == '.') {
    return false;
  }
  let start = next;
  next += ch.len_utf8();
  while next < value.len() {
    let Some(ch) = value[next..].chars().next() else {
      break;
    };
    if !(ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_')) {
      break;
    }
    next += ch.len_utf8();
  }
  if value[start..next]
    .chars()
    .any(|ch| ch.is_ascii_digit() || ch == '.')
  {
    return false;
  }
  while next < value.len() {
    let Some(ch) = value[next..].chars().next() else {
      break;
    };
    if !ch.is_whitespace() {
      break;
    }
    next += ch.len_utf8();
  }
  value[next..].starts_with('(')
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
    "ERR:511" | "#ERR511!" => FormulaErrorValue::Parameter,
    "ERR:504" | "ERR:508" | "#ERR502!" | "#ERR508!" | "#ERR504!" => FormulaErrorValue::Unknown,
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
    let same_sheet = parse_formula_range(SheetId(3), ".B8:.B95").unwrap();
    assert_eq!(same_sheet.sheet, SheetId(3));
    assert!(same_sheet.sheet_name.is_none());
    assert_eq!(same_sheet.range.start, CellAddress { column: 1, row: 7 });
    assert_eq!(same_sheet.range.end, CellAddress { column: 1, row: 94 });

    let inherited = parse_formula_range(SheetId(3), "Sheet2.C2:.C92").unwrap();
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
      grammar: FormulaGrammar::ExcelA1,
      locals: BTreeMap::new(),
      array_context: false,
      current_value: None,
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
      grammar: FormulaGrammar::ExcelA1,
      locals: BTreeMap::new(),
      array_context: false,
      current_value: None,
    };

    assert_eq!(
      ast.as_ref().and_then(|ast| evaluator.evaluate(ast)),
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
      format_complex_number(-0.0787746170678337, 0.586433260393873, 'i'),
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
