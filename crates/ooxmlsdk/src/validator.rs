use std::fmt::Display;

pub trait SdkValidator {
  fn validate(&self) -> Result<(), crate::common::SdkError> {
    Ok(())
  }

  fn is_valid(&self) -> bool {
    self.validate().is_ok()
  }
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
}
