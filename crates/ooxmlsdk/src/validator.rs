use std::fmt::Display;

pub trait SdkValidator {
  fn validate(&self) -> Result<(), crate::common::SdkError> {
    Ok(())
  }

  fn is_valid(&self) -> bool {
    self.validate().is_ok()
  }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StringFormatKind {
  Token,
  NcName,
  QName,
  Uri,
  Id,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NumberSignKind {
  NonNegative,
  Positive,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StringLengthKind {
  Characters,
  HexBinaryBytes,
}

pub fn validate_pattern<T: Display>(
  ty: &'static str,
  field: &'static str,
  value: &T,
  regex: &str,
) -> Result<(), crate::common::SdkError> {
  let anchored_regex = format!(r"\A(?:{regex})\z");
  let regex = regex::Regex::new(&anchored_regex).map_err(|err| {
    crate::common::SdkError::CommonError(format!("invalid validator regex for {ty}.{field}: {err}"))
  })?;
  let value_string = value.to_string();
  if regex.is_match(&value_string) {
    Ok(())
  } else {
    Err(crate::common::validation_error(
      ty,
      field,
      "pattern",
      value_string,
      format!("value does not match regex {regex}"),
    ))
  }
}

pub fn validate_string_length<T: Display>(
  ty: &'static str,
  field: &'static str,
  value: &T,
  min: Option<u32>,
  max: Option<u32>,
) -> Result<(), crate::common::SdkError> {
  let value_string = value.to_string();
  validate_string_length_with_kind(
    ty,
    field,
    value_string,
    min,
    max,
    StringLengthKind::Characters,
  )
}

pub fn validate_string_length_with_kind(
  ty: &'static str,
  field: &'static str,
  value_string: String,
  min: Option<u32>,
  max: Option<u32>,
  kind: StringLengthKind,
) -> Result<(), crate::common::SdkError> {
  let value_len = match kind {
    StringLengthKind::Characters => value_string.chars().count() as u32,
    StringLengthKind::HexBinaryBytes => (value_string.chars().count() / 2) as u32,
  };
  if let Some(min) = min
    && value_len < min
  {
    return Err(crate::common::validation_error(
      ty,
      field,
      "string_length",
      value_string,
      format!("length {value_len} is below minimum {min}"),
    ));
  }
  if let Some(max) = max
    && value_len > max
  {
    return Err(crate::common::validation_error(
      ty,
      field,
      "string_length",
      value_string,
      format!("length {value_len} exceeds maximum {max}"),
    ));
  }
  Ok(())
}

pub fn validate_string_format<T: Display>(
  ty: &'static str,
  field: &'static str,
  value: &T,
  kind: StringFormatKind,
) -> Result<(), crate::common::SdkError> {
  let value_string = value.to_string();
  let valid = match kind {
    StringFormatKind::Token => is_token(&value_string),
    StringFormatKind::NcName | StringFormatKind::Id => is_ncname(&value_string),
    StringFormatKind::QName => is_qname(&value_string),
    StringFormatKind::Uri => is_uri(&value_string),
  };
  if valid {
    Ok(())
  } else {
    Err(crate::common::validation_error(
      ty,
      field,
      "string_format",
      value_string,
      format!("value does not satisfy {kind:?} format"),
    ))
  }
}

pub fn validate_number_range<T: Display>(
  ty: &'static str,
  field: &'static str,
  value: &T,
  min: Option<&str>,
  max: Option<&str>,
  min_inclusive: bool,
  max_inclusive: bool,
) -> Result<(), crate::common::SdkError> {
  let value_string = value.to_string();
  let parsed_value = value_string.parse::<f64>().map_err(|err| {
    crate::common::SdkError::CommonError(format!(
      "failed to parse numeric validator value for {ty}.{field}: {err}"
    ))
  })?;

  if let Some(min) = min {
    let parsed_min = min.parse::<f64>().map_err(|err| {
      crate::common::SdkError::CommonError(format!(
        "failed to parse numeric validator minimum for {ty}.{field}: {err}"
      ))
    })?;
    let valid = if min_inclusive {
      parsed_value >= parsed_min
    } else {
      parsed_value > parsed_min
    };
    if !valid {
      let comparator = if min_inclusive { ">=" } else { ">" };
      return Err(crate::common::validation_error(
        ty,
        field,
        "number_range",
        value_string,
        format!("value must be {comparator} {min}"),
      ));
    }
  }

  if let Some(max) = max {
    let parsed_max = max.parse::<f64>().map_err(|err| {
      crate::common::SdkError::CommonError(format!(
        "failed to parse numeric validator maximum for {ty}.{field}: {err}"
      ))
    })?;
    let valid = if max_inclusive {
      parsed_value <= parsed_max
    } else {
      parsed_value < parsed_max
    };
    if !valid {
      let comparator = if max_inclusive { "<=" } else { "<" };
      return Err(crate::common::validation_error(
        ty,
        field,
        "number_range",
        value_string,
        format!("value must be {comparator} {max}"),
      ));
    }
  }

  Ok(())
}

pub fn validate_number_sign<T: Display>(
  ty: &'static str,
  field: &'static str,
  value: &T,
  kind: NumberSignKind,
) -> Result<(), crate::common::SdkError> {
  let value_string = value.to_string();
  let parsed_value = value_string.parse::<f64>().map_err(|err| {
    crate::common::SdkError::CommonError(format!(
      "failed to parse numeric validator value for {ty}.{field}: {err}"
    ))
  })?;
  let valid = match kind {
    NumberSignKind::NonNegative => parsed_value >= 0.0,
    NumberSignKind::Positive => parsed_value > 0.0,
  };
  if valid && !parsed_value.is_nan() {
    Ok(())
  } else {
    Err(crate::common::validation_error(
      ty,
      field,
      "number_sign",
      value_string,
      format!("value does not satisfy {kind:?} constraint"),
    ))
  }
}

pub fn validate_number_type<T: Display>(
  ty: &'static str,
  field: &'static str,
  value: &T,
  type_name: &str,
) -> Result<(), crate::common::SdkError> {
  let value_string = value.to_string();
  let valid = match type_name {
    "xsd:byte" => value_string.parse::<i8>().is_ok(),
    "xsd:short" => value_string.parse::<i16>().is_ok(),
    "xsd:int" => value_string.parse::<i32>().is_ok(),
    "xsd:long" => value_string.parse::<i64>().is_ok(),
    "xsd:unsignedByte" => value_string.parse::<u8>().is_ok(),
    "xsd:unsignedShort" => value_string.parse::<u16>().is_ok(),
    "xsd:unsignedInt" => value_string.parse::<u32>().is_ok(),
    "xsd:unsignedLong" => value_string.parse::<u64>().is_ok(),
    "xsd:integer" | "xsd:nonNegativeInteger" => value_string.parse::<i128>().is_ok(),
    "xsd:decimal" | "xsd:double" => value_string.parse::<f64>().is_ok(),
    "xsd:float" => value_string.parse::<f32>().is_ok(),
    "wp:ST_PositionOffset"
    | "xdr:ST_ColID"
    | "xdr:ST_RowID"
    | "a:ST_Angle"
    | "a:ST_Percentage"
    | "a:ST_PositivePercentage"
    | "a:ST_PositiveFixedPercentage"
    | "w:ST_DecimalNumber"
    | "w:ST_NonNegativeDecimalNumber"
    | "w:ST_SignedDecimalNumberMax-1"
    | "w:ST_SignedDecimalNumberMax-2"
    | "w:ST_SignedHpsMeasure_O12"
    | "w:ST_SignedTwipsMeasure_O12"
    | "w:ST_UnsignedDecimalNumberMin1" => value_string.parse::<i32>().is_ok(),
    "a:ST_Coordinate" => value_string.parse::<i64>().is_ok(),
    "ask:ST_LineSketchSeed"
    | "cx:ST_AxisId"
    | "a:ST_DrawingElementId"
    | "w:ST_HpsMeasure_O12"
    | "w:ST_TwipsMeasure_O12"
    | "w:ST_UnsignedDecimalNumber" => value_string.parse::<u32>().is_ok(),
    _ => false,
  };

  if valid {
    Ok(())
  } else {
    Err(crate::common::validation_error(
      ty,
      field,
      "number_type",
      value_string,
      format!("value does not satisfy {type_name} numeric type"),
    ))
  }
}

pub fn validate_string_set<T: Display>(
  ty: &'static str,
  field: &'static str,
  value: &T,
  values: &[&str],
) -> Result<(), crate::common::SdkError> {
  let value_string = value.to_string();
  if values.contains(&value_string.as_str()) {
    Ok(())
  } else {
    Err(crate::common::validation_error(
      ty,
      field,
      "string_set",
      value_string,
      format!("value is not one of {}", values.join(", ")),
    ))
  }
}

fn is_token(value: &str) -> bool {
  value.trim() == value
    && !value.chars().any(|c| matches!(c, '\n' | '\r' | '\t'))
    && !value.contains("  ")
}

fn is_ncname(value: &str) -> bool {
  let mut chars = value.chars();
  let Some(first) = chars.next() else {
    return false;
  };
  if !(first == '_' || first.is_alphabetic()) {
    return false;
  }
  chars.all(|ch| ch == '_' || ch == '-' || ch == '.' || ch.is_alphanumeric())
}

fn is_qname(value: &str) -> bool {
  let mut parts = value.split(':');
  let Some(first) = parts.next() else {
    return false;
  };
  let Some(second) = parts.next() else {
    return is_ncname(first);
  };
  parts.next().is_none() && is_ncname(first) && is_ncname(second)
}

fn is_uri(value: &str) -> bool {
  if value.chars().any(char::is_whitespace) {
    return false;
  }
  if value.contains("##") {
    return false;
  }
  if let Some(scheme_end) = value.find("://")
    && value[scheme_end + 3..].starts_with('/')
  {
    return false;
  }
  true
}
