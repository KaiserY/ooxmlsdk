use std::borrow::Cow;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnitConversionError {
  UnknownUnit,
  IncompatibleUnit,
}

pub fn convert_unit(value: f64, from: &str, to: &str) -> Result<f64, UnitConversionError> {
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

  let (from_unit, from_level) = find_convert_unit(from).ok_or(UnitConversionError::UnknownUnit)?;
  let (to_unit, to_level) = find_convert_unit(to).ok_or(UnitConversionError::UnknownUnit)?;
  if from_unit.class != to_unit.class || from_unit.linear != to_unit.linear {
    return Err(UnitConversionError::IncompatibleUnit);
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn converts_proportional_units_with_prefixes() {
    assert_eq!(convert_unit(1.0, "kg", "g"), Ok(1000.0));
    assert!((convert_unit(1.0, "mi", "m").unwrap() - 1609.344).abs() < 1.0e-9);
    assert_eq!(convert_unit(1.0, "m^2", "cm^2"), Ok(10_000.0));
  }

  #[test]
  fn converts_linear_temperature_units() {
    assert!(convert_unit(32.0, "F", "C").unwrap().abs() < 1.0e-12);
    assert!((convert_unit(100.0, "C", "F").unwrap() - 212.0).abs() < 1.0e-12);
  }

  #[test]
  fn distinguishes_decimal_and_binary_information_prefixes() {
    assert_eq!(convert_unit(1.0, "Mbyte", "byte"), Ok(1_000_000.0));
    assert_eq!(convert_unit(1.0, "Mibyte", "byte"), Ok(1_048_576.0));
  }

  #[test]
  fn rejects_unknown_and_incompatible_units() {
    assert_eq!(
      convert_unit(1.0, "missing", "m"),
      Err(UnitConversionError::UnknownUnit)
    );
    assert_eq!(
      convert_unit(1.0, "m", "g"),
      Err(UnitConversionError::IncompatibleUnit)
    );
  }
}
