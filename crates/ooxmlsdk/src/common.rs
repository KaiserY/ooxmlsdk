use quick_xml::{
  Decoder, Reader,
  encoding::EncodingError,
  events::{
    Event,
    attributes::{AttrError, Attribute},
  },
  name::QName,
};
use std::{
  io::BufRead,
  num::{ParseFloatError, ParseIntError},
};
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
pub fn unexpected_eof(context: &'static str) -> SdkError {
  SdkError::UnexpectedEof { context }
}

pub trait XmlReader<'de> {
  fn next(&mut self) -> Result<Event<'de>, SdkError>;
  fn read_to_end(&mut self, end: QName) -> Result<(), SdkError>;
  fn decoder(&self) -> Decoder;
}

pub struct IoReader<R: BufRead> {
  reader: Reader<R>,
  buf: Vec<u8>,
}

impl<R: BufRead> IoReader<R> {
  pub fn new(reader: Reader<R>) -> Self {
    Self {
      reader,
      buf: Vec::new(),
    }
  }
}

impl<'de, R: BufRead> XmlReader<'de> for IoReader<R> {
  #[inline]
  fn next(&mut self) -> Result<Event<'de>, SdkError> {
    self.buf.clear();
    Ok(self.reader.read_event_into(&mut self.buf)?.into_owned())
  }
  #[inline]
  fn read_to_end(&mut self, end: QName) -> Result<(), SdkError> {
    self.reader.read_to_end_into(end, &mut self.buf)?;
    Ok(())
  }
  #[inline]
  fn decoder(&self) -> Decoder {
    self.reader.decoder()
  }
}
pub struct SliceReader<'de> {
  reader: Reader<&'de [u8]>,
}

impl<'de> SliceReader<'de> {
  pub fn new(reader: Reader<&'de [u8]>) -> Self {
    Self { reader }
  }
}

impl<'de> XmlReader<'de> for SliceReader<'de> {
  #[inline]
  fn next(&mut self) -> Result<Event<'de>, SdkError> {
    Ok(self.reader.read_event()?)
  }
  #[inline]
  fn read_to_end(&mut self, end: QName) -> Result<(), SdkError> {
    self.reader.read_to_end(end)?;
    Ok(())
  }
  #[inline]
  fn decoder(&self) -> Decoder {
    self.reader.decoder()
  }
}

pub fn resolve_zip_file_path(path: &str) -> String {
  let mut stack = Vec::new();
  for component in path.split('/') {
    match component {
      "" | "." => {}
      ".." => {
        stack.pop();
      }
      _ => {
        stack.push(component);
      }
    }
  }
  stack.join("/")
}

#[inline]
pub(crate) fn from_reader_inner<R: BufRead>(reader: R) -> Result<IoReader<R>, SdkError> {
  let mut xml_reader = quick_xml::Reader::from_reader(reader);
  xml_reader.config_mut().check_end_names = false;
  Ok(IoReader::new(xml_reader))
}

#[inline]
pub(crate) fn from_str_inner(s: &str) -> Result<SliceReader<'_>, SdkError> {
  let mut xml_reader = quick_xml::Reader::from_str(s);
  xml_reader.config_mut().check_end_names = false;
  Ok(SliceReader::new(xml_reader))
}

#[inline(always)]
fn attr_raw_value<'a>(attr: &'a Attribute<'a>) -> Option<&'a [u8]> {
  let value = attr.value.as_ref();
  if value.contains(&b'&') {
    None
  } else {
    Some(value)
  }
}

#[inline(always)]
pub fn decode_attr_value(attr: &Attribute<'_>, decoder: Decoder) -> Result<String, SdkError> {
  if let Some(value) = attr_raw_value(attr) {
    return Ok(decoder.decode(value)?.into_owned());
  }

  Ok(attr.decode_and_unescape_value(decoder)?.into_owned())
}

#[inline]
pub fn parse_value<T>(value: &str, ty: &'static str, field: &'static str) -> Result<T, SdkError>
where
  T: std::str::FromStr,
{
  value
    .parse::<T>()
    .map_err(|_| invalid_field_value(ty, field, value))
}

#[inline]
pub fn parse_bool_str(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  match value.as_bytes() {
    b"true" | b"1" | b"True" | b"TRUE" | b"t" | b"Yes" | b"YES" | b"yes" | b"y" => Ok(true),
    b"false" | b"0" | b"False" | b"FALSE" | b"f" | b"No" | b"NO" | b"no" | b"n" | b"" => Ok(false),
    _ => Err(invalid_field_value(ty, field, value)),
  }
}

#[inline]
pub fn parse_bool_bytes(b: &[u8]) -> Result<bool, SdkError> {
  match b {
    b"true" | b"1" | b"True" | b"TRUE" | b"t" | b"Yes" | b"YES" | b"yes" | b"y" => Ok(true),
    b"false" | b"0" | b"False" | b"FALSE" | b"f" | b"No" | b"NO" | b"no" | b"n" | b"" => Ok(false),
    other => Err(SdkError::CommonError(
      String::from_utf8_lossy(other).into_owned(),
    )),
  }
}

#[inline(always)]
pub fn parse_bool_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  if let Some(value) = attr_raw_value(attr)
    && let Ok(value) = parse_bool_bytes(value)
  {
    return Ok(value);
  }

  let value = decode_attr_value(attr, decoder)?;
  parse_bool_str(value.as_ref(), ty, field)
}

#[inline(always)]
pub fn parse_attr_value<T>(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<T, SdkError>
where
  T: std::str::FromStr,
{
  if let Some(value) = attr_raw_value(attr)
    && let Ok(value) = std::str::from_utf8(value)
    && let Ok(value) = value.parse::<T>()
  {
    return Ok(value);
  }

  let value = decode_attr_value(attr, decoder)?;
  parse_value(value.as_ref(), ty, field)
}
macro_rules! expect_event_start {
  ($xml_reader:expr, $xml_event:expr, $tag_prefix:expr, $tag:expr) => {{
    let (e, empty_tag) = if let Some((e, empty_tag)) = $xml_event {
      (e, empty_tag)
    } else {
      loop {
        match $xml_reader.next()? {
          quick_xml::events::Event::Start(b) => break (b, false),
          quick_xml::events::Event::Empty(b) => break (b, true),
          quick_xml::events::Event::Eof => {
            return Err(super::super::common::unexpected_eof("xml"));
          }
          _ => continue,
        }
      }
    };
    match e.name().as_ref() {
      $tag_prefix | $tag => (),
      found => {
        return Err(super::super::common::unexpected_tag("xml", "xml", found));
      }
    }
    (e, empty_tag)
  }};
  (
        $xml_reader:expr, $xml_event:expr, $context:expr, $expected:expr,
        $tag_prefix:expr, $tag:expr
    ) => {{
    let (e, empty_tag) = if let Some((e, empty_tag)) = $xml_event {
      (e, empty_tag)
    } else {
      loop {
        match $xml_reader.next()? {
          quick_xml::events::Event::Start(b) => break (b, false),
          quick_xml::events::Event::Empty(b) => break (b, true),
          quick_xml::events::Event::Eof => {
            return Err(crate::common::unexpected_eof($context));
          }
          _ => continue,
        }
      }
    };
    match e.name().as_ref() {
      $tag_prefix | $tag => (),
      found => {
        return Err(crate::common::unexpected_tag($context, $expected, found));
      }
    }
    (e, empty_tag)
  }};
}

pub(crate) use expect_event_start;
