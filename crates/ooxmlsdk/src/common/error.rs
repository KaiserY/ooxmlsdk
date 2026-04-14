use quick_xml::{encoding::EncodingError, events::attributes::AttrError};
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SdkError {
  #[error("quick_xml error")]
  QuickXmlError(#[from] quick_xml::Error),
  #[error("quick_xml encoding error")]
  QuickEncodingError(#[from] EncodingError),
  #[error("quick_xml attr error")]
  AttrError(#[from] AttrError),
  #[error("ParseIntError")]
  ParseIntError(#[from] ParseIntError),
  #[error("ParseFloatError")]
  ParseFloatError(#[from] ParseFloatError),
  #[error("StdFmtError")]
  StdFmtError(#[from] std::fmt::Error),
  #[error("StdIoError")]
  StdIoError(#[from] std::io::Error),
  #[cfg(feature = "parts")]
  #[error("ZipError")]
  ZipError(#[from] zip::result::ZipError),
  #[error("mismatch error (expected {expected:?}, found {found:?})")]
  MismatchError { expected: String, found: String },
  #[error("CommonError")]
  CommonError(String),
  #[error("unexpected tag while parsing {ty} (expected {expected:?}, found {found:?})")]
  UnexpectedTag {
    ty: &'static str,
    expected: &'static str,
    found: String,
  },
  #[error("missing field `{field}` while parsing {ty}")]
  MissingField {
    ty: &'static str,
    field: &'static str,
  },
  #[error("invalid enum value while parsing {ty}: {value:?}")]
  InvalidEnumValue { ty: &'static str, value: String },
  #[error("invalid field `{field}` while parsing {ty}: {value:?}")]
  InvalidFieldValue {
    ty: &'static str,
    field: &'static str,
    value: String,
  },
  #[error("validation failed for `{field}` on {ty} with {validator}: {message} ({value:?})")]
  ValidationError {
    ty: &'static str,
    field: &'static str,
    validator: &'static str,
    value: String,
    message: String,
  },
  #[error("unexpected EOF while parsing {context}")]
  UnexpectedEof { context: &'static str },
  #[error("unknown error")]
  UnknownError,
}

#[inline]
pub fn unexpected_tag(ty: &'static str, expected: &'static str, found: &[u8]) -> SdkError {
  SdkError::UnexpectedTag {
    ty,
    expected,
    found: String::from_utf8_lossy(found).into_owned(),
  }
}

#[inline]
pub fn missing_field(ty: &'static str, field: &'static str) -> SdkError {
  SdkError::MissingField { ty, field }
}

#[inline]
pub fn invalid_enum_value(ty: &'static str, value: impl Into<String>) -> SdkError {
  SdkError::InvalidEnumValue {
    ty,
    value: value.into(),
  }
}

#[inline]
pub fn invalid_field_value(
  ty: &'static str,
  field: &'static str,
  value: impl Into<String>,
) -> SdkError {
  SdkError::InvalidFieldValue {
    ty,
    field,
    value: value.into(),
  }
}

#[inline]
pub fn validation_error(
  ty: &'static str,
  field: &'static str,
  validator: &'static str,
  value: impl Into<String>,
  message: impl Into<String>,
) -> SdkError {
  SdkError::ValidationError {
    ty,
    field,
    validator,
    value: value.into(),
    message: message.into(),
  }
}

#[inline]
pub fn unexpected_eof(context: &'static str) -> SdkError {
  SdkError::UnexpectedEof { context }
}
