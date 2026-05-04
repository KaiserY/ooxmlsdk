use std::fmt::Display;
use std::ops::{Bound, RangeBounds};

pub trait SdkValidator {
  fn validate(&self) -> Vec<ValidationErrorInfo> {
    let mut context = ValidationContext::default();
    self.validate_into(&mut context);
    context.into_errors()
  }

  #[doc(hidden)]
  fn validate_into(&self, _context: &mut ValidationContext) {}
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ValidationErrorType {
  #[default]
  Schema,
  Semantic,
  #[cfg(feature = "parts")]
  Package,
  #[cfg(feature = "mce")]
  MarkupCompatibility,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationErrorInfo {
  pub error_type: ValidationErrorType,
  pub description: String,
  pub id: Option<&'static str>,
  pub type_name: Option<&'static str>,
  pub field_name: Option<&'static str>,
  #[cfg(feature = "parts")]
  pub part_uri: Option<String>,
}

impl ValidationErrorInfo {
  #[inline]
  pub fn from_error(error_type: ValidationErrorType, error: crate::common::SdkError) -> Self {
    Self {
      error_type,
      description: error.to_string(),
      id: validation_error_id(&error),
      type_name: validation_error_type_name(&error),
      field_name: validation_error_field_name(&error),
      #[cfg(feature = "parts")]
      part_uri: None,
    }
  }

  #[cfg(feature = "parts")]
  #[inline]
  pub fn with_part_uri(mut self, part_uri: impl Into<String>) -> Self {
    self.part_uri = Some(part_uri.into());
    self
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationSettings {
  #[cfg(feature = "parts")]
  pub file_format: crate::sdk::FileFormatVersion,
  pub max_number_of_errors: usize,
}

impl Default for ValidationSettings {
  #[inline]
  fn default() -> Self {
    Self {
      #[cfg(feature = "parts")]
      file_format: crate::sdk::FileFormatVersion::default(),
      max_number_of_errors: usize::MAX,
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationContext {
  settings: ValidationSettings,
  errors: Vec<ValidationErrorInfo>,
  #[cfg(feature = "parts")]
  part_uri: Option<String>,
}

impl ValidationContext {
  #[inline]
  pub const fn with_settings(settings: ValidationSettings) -> Self {
    Self {
      settings,
      errors: Vec::new(),
      #[cfg(feature = "parts")]
      part_uri: None,
    }
  }

  #[inline]
  pub const fn settings(&self) -> &ValidationSettings {
    &self.settings
  }

  #[inline]
  pub fn should_stop(&self) -> bool {
    self.errors.len() >= self.settings.max_number_of_errors
  }

  #[inline]
  pub fn check(&mut self, validate: impl FnOnce() -> Result<(), crate::common::SdkError>) {
    if self.should_stop() {
      return;
    }
    if let Err(error) = validate() {
      self.push_error(error);
    }
  }

  #[inline]
  pub fn push_error(&mut self, error: crate::common::SdkError) {
    if self.should_stop() {
      return;
    }
    let info = ValidationErrorInfo::from_error(ValidationErrorType::Schema, error);
    #[cfg(feature = "parts")]
    let info = if let Some(part_uri) = &self.part_uri {
      info.with_part_uri(part_uri.clone())
    } else {
      info
    };
    self.errors.push(info);
  }

  #[cfg(feature = "parts")]
  pub fn with_part_uri(&mut self, part_uri: impl Into<String>, validate: impl FnOnce(&mut Self)) {
    let previous = self.part_uri.replace(part_uri.into());
    validate(self);
    self.part_uri = previous;
  }

  #[inline]
  pub fn into_errors(self) -> Vec<ValidationErrorInfo> {
    self.errors
  }
}

impl Default for ValidationContext {
  #[inline]
  fn default() -> Self {
    Self::with_settings(ValidationSettings::default())
  }
}

fn validation_error_id(error: &crate::common::SdkError) -> Option<&'static str> {
  match error {
    crate::common::SdkError::ValidationError { validator, .. } => Some(validator),
    crate::common::SdkError::MissingField { .. } => Some("required"),
    crate::common::SdkError::InvalidEnumValue { .. } => Some("enum"),
    crate::common::SdkError::InvalidFieldValue { .. } => Some("field_value"),
    _ => None,
  }
}

fn validation_error_type_name(error: &crate::common::SdkError) -> Option<&'static str> {
  match error {
    crate::common::SdkError::ValidationError { ty, .. }
    | crate::common::SdkError::MissingField { ty, .. }
    | crate::common::SdkError::InvalidEnumValue { ty, .. }
    | crate::common::SdkError::InvalidFieldValue { ty, .. }
    | crate::common::SdkError::UnexpectedTag { ty, .. } => Some(ty),
    _ => None,
  }
}

fn validation_error_field_name(error: &crate::common::SdkError) -> Option<&'static str> {
  match error {
    crate::common::SdkError::ValidationError { field, .. }
    | crate::common::SdkError::MissingField { field, .. }
    | crate::common::SdkError::InvalidFieldValue { field, .. } => Some(field),
    _ => None,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BinaryFormatKind {
  Hex,
  Base64,
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
  validate_pattern_regex(ty, field, value, &regex)
}

pub fn validate_pattern_regex<T: Display>(
  ty: &'static str,
  field: &'static str,
  value: &T,
  regex: &regex::Regex,
) -> Result<(), crate::common::SdkError> {
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

pub fn validate_binary_format<T: Display>(
  ty: &'static str,
  field: &'static str,
  value: &T,
  kind: BinaryFormatKind,
) -> Result<(), crate::common::SdkError> {
  let value_string = value.to_string();
  let valid = match kind {
    BinaryFormatKind::Hex => crate::simple_type::is_valid_hex_binary(&value_string),
    BinaryFormatKind::Base64 => is_base64_binary(&value_string),
  };
  if valid {
    Ok(())
  } else {
    Err(crate::common::validation_error(
      ty,
      field,
      "binary_format",
      value_string,
      format!("value does not satisfy {kind:?} binary format"),
    ))
  }
}

pub fn validate_number_range<T: Display, R: RangeBounds<f64>>(
  ty: &'static str,
  field: &'static str,
  value: &T,
  range: R,
) -> Result<(), crate::common::SdkError> {
  let value_string = value.to_string();
  let parsed_value = value_string.parse::<f64>().map_err(|err| {
    crate::common::SdkError::CommonError(format!(
      "failed to parse numeric validator value for {ty}.{field}: {err}"
    ))
  })?;
  if parsed_value.is_nan() {
    return Err(crate::common::validation_error(
      ty,
      field,
      "number_range",
      value_string,
      "value must not be NaN".to_string(),
    ));
  }

  match range.start_bound() {
    Bound::Included(min) if parsed_value < *min => {
      return Err(crate::common::validation_error(
        ty,
        field,
        "number_range",
        value_string,
        format!("value must be >= {min}"),
      ));
    }
    Bound::Excluded(min) if parsed_value <= *min => {
      return Err(crate::common::validation_error(
        ty,
        field,
        "number_range",
        value_string,
        format!("value must be > {min}"),
      ));
    }
    Bound::Included(_) | Bound::Excluded(_) | Bound::Unbounded => {}
  }

  match range.end_bound() {
    Bound::Included(max) if parsed_value > *max => {
      return Err(crate::common::validation_error(
        ty,
        field,
        "number_range",
        value_string,
        format!("value must be <= {max}"),
      ));
    }
    Bound::Excluded(max) if parsed_value >= *max => {
      return Err(crate::common::validation_error(
        ty,
        field,
        "number_range",
        value_string,
        format!("value must be < {max}"),
      ));
    }
    Bound::Included(_) | Bound::Excluded(_) | Bound::Unbounded => {}
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
    "xsd:integer" => value_string.parse::<i128>().is_ok(),
    "xsd:nonNegativeInteger" => value_string.parse::<u128>().is_ok(),
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
  if value.is_empty() {
    return true;
  }
  let value = value.trim_matches([' ', '\t', '\n', '\r']);
  if value.is_empty() {
    return false;
  }
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

fn is_base64_binary(value: &str) -> bool {
  let mut padding_seen = false;
  let mut padding_count = 0usize;
  let mut len = 0usize;

  for ch in value.chars().filter(|ch| !ch.is_ascii_whitespace()) {
    len += 1;
    match ch {
      'A'..='Z' | 'a'..='z' | '0'..='9' | '+' | '/' if !padding_seen => {}
      '=' => {
        padding_seen = true;
        padding_count += 1;
        if padding_count > 2 {
          return false;
        }
      }
      _ => return false,
    }
  }

  if len == 0 {
    return true;
  }
  len.is_multiple_of(4)
}
