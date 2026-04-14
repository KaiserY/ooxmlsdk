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

pub fn validate_pattern<T: Display>(
  ty: &'static str,
  field: &'static str,
  value: &T,
  regex: &str,
) -> Result<(), crate::common::SdkError> {
  let regex = regex::Regex::new(regex).map_err(|err| {
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
  let value_len = value_string.chars().count() as u32;
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

fn is_token(value: &str) -> bool {
  !value.is_empty()
    && value.trim() == value
    && !value.chars().any(|c| matches!(c, '\n' | '\r' | '\t'))
    && !value.contains("  ")
}

fn is_ncname(value: &str) -> bool {
  let mut chars = value.chars();
  let Some(first) = chars.next() else {
    return false;
  };
  if !matches!(first, 'A'..='Z' | 'a'..='z' | '_') {
    return false;
  }
  chars.all(|ch| matches!(ch, 'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '-' | '.'))
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
  !value.is_empty() && !value.chars().any(char::is_whitespace)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn validates_pattern() {
    assert!(validate_pattern("TestType", "field", &"ABC-123", "^[A-Z]+-[0-9]+$").is_ok());
    assert!(validate_pattern("TestType", "field", &"abc", "^[A-Z]+$").is_err());
  }

  #[test]
  fn validates_string_length_bounds() {
    assert!(validate_string_length("TestType", "field", &"abcd", Some(2), Some(4)).is_ok());
    assert!(validate_string_length("TestType", "field", &"a", Some(2), None).is_err());
    assert!(validate_string_length("TestType", "field", &"abcdef", None, Some(4)).is_err());
  }

  #[test]
  fn validates_number_range_bounds() {
    assert!(
      validate_number_range("TestType", "field", &5, Some("0"), Some("5"), true, true).is_ok()
    );
    assert!(validate_number_range("TestType", "field", &5, Some("5"), None, false, true).is_err());
    assert!(validate_number_range("TestType", "field", &5, None, Some("5"), true, false).is_err());
  }

  #[test]
  fn validates_string_formats() {
    assert!(
      validate_string_format("TestType", "field", &"abc-def", StringFormatKind::NcName).is_ok()
    );
    assert!(
      validate_string_format("TestType", "field", &"ns:value", StringFormatKind::QName).is_ok()
    );
    assert!(
      validate_string_format("TestType", "field", &"two  spaces", StringFormatKind::Token).is_err()
    );
    assert!(
      validate_string_format(
        "TestType",
        "field",
        &"bad:name:extra",
        StringFormatKind::QName
      )
      .is_err()
    );
    assert!(
      validate_string_format("TestType", "field", &"urn:test", StringFormatKind::Uri).is_ok()
    );
  }

  #[test]
  fn validates_number_signs() {
    assert!(validate_number_sign("TestType", "field", &0, NumberSignKind::NonNegative).is_ok());
    assert!(validate_number_sign("TestType", "field", &1, NumberSignKind::Positive).is_ok());
    assert!(validate_number_sign("TestType", "field", &-1, NumberSignKind::NonNegative).is_err());
    assert!(validate_number_sign("TestType", "field", &0, NumberSignKind::Positive).is_err());
  }
}
