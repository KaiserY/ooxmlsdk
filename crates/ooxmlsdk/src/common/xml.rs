use quick_xml::{
  Decoder, Reader, XmlVersion,
  events::{BytesStart, Event, attributes::Attribute},
};
use std::{borrow::Cow, io::BufRead, ops::Deref, ops::Range};

use super::{SdkError, invalid_field_value, unexpected_eof, unexpected_tag};

pub struct IoReader<R: BufRead> {
  reader: Reader<R>,
  buf: Vec<u8>,
  pending: Option<PayloadEvent<'static>>,
}

#[derive(Debug)]
pub enum PayloadEvent<'xml> {
  Start(quick_xml::events::BytesStart<'xml>, bool),
  End(quick_xml::events::BytesEnd<'xml>),
  Text(quick_xml::events::BytesText<'xml>),
  CData(quick_xml::events::BytesCData<'xml>),
  GeneralRef(quick_xml::events::BytesRef<'xml>),
  Eof,
}

impl<'xml> PayloadEvent<'xml> {
  #[inline]
  fn into_owned(self) -> PayloadEvent<'static> {
    match self {
      Self::Start(e, empty) => PayloadEvent::Start(e.into_owned(), empty),
      Self::End(e) => PayloadEvent::End(e.into_owned()),
      Self::Text(e) => PayloadEvent::Text(e.into_owned()),
      Self::CData(e) => PayloadEvent::CData(e.into_owned()),
      Self::GeneralRef(e) => PayloadEvent::GeneralRef(e.into_owned()),
      Self::Eof => PayloadEvent::Eof,
    }
  }

  #[cfg(feature = "flat-opc")]
  #[inline]
  fn into_event(self) -> Option<Event<'xml>> {
    Some(match self {
      Self::Start(e, false) => Event::Start(e),
      Self::Start(e, true) => Event::Empty(e),
      Self::End(e) => Event::End(e),
      Self::Text(e) => Event::Text(e),
      Self::CData(e) => Event::CData(e),
      Self::GeneralRef(e) => Event::GeneralRef(e),
      Self::Eof => Event::Eof,
    })
  }
}

#[inline]
const fn is_non_whitespace(ch: char) -> bool {
  !matches!(ch, ' ' | '\r' | '\n' | '\t')
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Text<'xml> {
  text: Cow<'xml, str>,
  content: Range<usize>,
}

impl<'xml> Text<'xml> {
  #[inline]
  fn new(text: Cow<'xml, str>) -> Self {
    let start = text.find(is_non_whitespace).unwrap_or(0);
    let end = text.rfind(is_non_whitespace).map_or(0, |index| index + 1);
    let content = if start >= end { 0..0 } else { start..end };
    Self { text, content }
  }

  #[inline]
  pub fn is_blank(&self) -> bool {
    self.content.is_empty()
  }

  #[inline]
  fn into_string(self) -> String {
    match self.text {
      Cow::Borrowed(text) => text.to_string(),
      Cow::Owned(text) => text,
    }
  }
}

impl Deref for Text<'_> {
  type Target = str;

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.text.deref()
  }
}

impl AsRef<str> for Text<'_> {
  #[inline]
  fn as_ref(&self) -> &str {
    self.text.as_ref()
  }
}

impl<'xml> From<Cow<'xml, str>> for Text<'xml> {
  #[inline]
  fn from(text: Cow<'xml, str>) -> Self {
    Self::new(text)
  }
}

impl<'xml> From<&'xml str> for Text<'xml> {
  #[inline]
  fn from(text: &'xml str) -> Self {
    Self::new(Cow::Borrowed(text))
  }
}

impl<'xml> From<String> for Text<'xml> {
  #[inline]
  fn from(text: String) -> Self {
    Self::new(Cow::Owned(text))
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeEvent<'xml> {
  Start(quick_xml::events::BytesStart<'xml>, bool),
  End(quick_xml::events::BytesEnd<'xml>),
  FastBytesText(quick_xml::events::BytesText<'xml>),
  Text(Text<'xml>),
  Eof,
}

#[inline(always)]
pub(crate) fn xml_local_name<'a>(name: quick_xml::name::QName<'a>) -> &'a [u8] {
  let raw_name = name.0;
  if raw_name.len() > 2 && raw_name[1] == b':' {
    &raw_name[2..]
  } else {
    name.local_name().into_inner()
  }
}

#[inline]
pub(crate) fn attribute_qname_has_namespace(
  start: &BytesStart<'_>,
  qname: &[u8],
  namespace_uri: &[u8],
) -> Result<bool, SdkError> {
  let Some(separator) = qname.iter().rposition(|byte| *byte == b':') else {
    return Ok(false);
  };
  let prefix = &qname[..separator];
  for attr in start.attributes().with_checks(false) {
    let attr = attr?;
    if attr
      .key
      .as_ref()
      .strip_prefix(b"xmlns:")
      .is_some_and(|candidate| candidate == prefix)
    {
      return Ok(attr.value.as_ref() == namespace_uri);
    }
  }
  Ok(false)
}

#[inline]
fn payload_event_from_event(event: Event<'_>) -> Option<PayloadEvent<'_>> {
  Some(match event {
    Event::Start(e) => PayloadEvent::Start(e, false),
    Event::Empty(e) => PayloadEvent::Start(e, true),
    Event::End(e) => PayloadEvent::End(e),
    Event::Text(e) => PayloadEvent::Text(e),
    Event::CData(e) => PayloadEvent::CData(e),
    Event::GeneralRef(e) => PayloadEvent::GeneralRef(e),
    Event::Eof => PayloadEvent::Eof,
    _ => return None,
  })
}

pub trait XmlRead<'xml> {
  fn next(&mut self) -> Result<PayloadEvent<'xml>, SdkError>;

  fn next_tag_event(&mut self) -> Result<PayloadEvent<'xml>, SdkError>;

  fn unread(&mut self, event: PayloadEvent<'xml>) -> Result<(), SdkError>;

  fn decoder(&self) -> Decoder;

  #[inline]
  fn next_de_event(
    &mut self,
    ty: &'static str,
    field: &'static str,
  ) -> Result<DeEvent<'xml>, SdkError>
  where
    Self: Sized,
  {
    read_de_event(self, ty, field)
  }

  fn read_text(
    &mut self,
    end: quick_xml::name::QName<'_>,
    ty: &'static str,
    field: &'static str,
  ) -> Result<String, SdkError>;

  fn read_raw_empty_xml_bytes(
    &mut self,
    start: quick_xml::events::BytesStart<'xml>,
  ) -> Result<Box<[u8]>, SdkError>;

  fn read_raw_element_xml_bytes(
    &mut self,
    start: quick_xml::events::BytesStart<'xml>,
  ) -> Result<Box<[u8]>, SdkError>;

  #[inline]
  fn read_raw_empty_xml_string(
    &mut self,
    start: quick_xml::events::BytesStart<'xml>,
  ) -> Result<String, SdkError> {
    raw_xml_bytes_to_string(self.read_raw_empty_xml_bytes(start)?)
  }

  #[inline]
  fn read_raw_element_xml_string(
    &mut self,
    start: quick_xml::events::BytesStart<'xml>,
  ) -> Result<String, SdkError> {
    raw_xml_bytes_to_string(self.read_raw_element_xml_bytes(start)?)
  }
}

impl<R: BufRead> IoReader<R> {
  pub fn new(reader: Reader<R>) -> Self {
    Self {
      reader,
      buf: Vec::new(),
      pending: None,
    }
  }

  #[inline]
  pub fn next(&mut self) -> Result<PayloadEvent<'static>, SdkError> {
    if let Some(event) = self.pending.take() {
      return Ok(event);
    }

    loop {
      self.buf.clear();
      if let Some(event) = payload_event_from_event(self.reader.read_event_into(&mut self.buf)?) {
        return Ok(event.into_owned());
      }
    }
  }

  #[inline]
  pub fn read_text(
    &mut self,
    end: quick_xml::name::QName<'_>,
    ty: &'static str,
    field: &'static str,
  ) -> Result<String, SdkError> {
    read_text_events(self, end, ty, field)
  }

  #[inline]
  pub fn next_tag_event(&mut self) -> Result<PayloadEvent<'static>, SdkError> {
    if let Some(event) = self.pending.take() {
      match event {
        PayloadEvent::Start(_, _) | PayloadEvent::End(_) | PayloadEvent::Eof => return Ok(event),
        PayloadEvent::Text(_) | PayloadEvent::CData(_) | PayloadEvent::GeneralRef(_) => {}
      }
    }

    loop {
      self.buf.clear();
      match self.reader.read_event_into(&mut self.buf)? {
        Event::Start(e) => return Ok(PayloadEvent::Start(e.into_owned(), false)),
        Event::Empty(e) => return Ok(PayloadEvent::Start(e.into_owned(), true)),
        Event::End(e) => return Ok(PayloadEvent::End(e.into_owned())),
        Event::Eof => return Ok(PayloadEvent::Eof),
        _ => {}
      }
    }
  }

  #[inline]
  pub fn unread(&mut self, event: PayloadEvent<'static>) -> Result<(), SdkError> {
    if self.pending.is_some() {
      return Err(SdkError::CommonError(
        "xml reader unread buffer already occupied".to_string(),
      ));
    }

    self.pending = Some(event);
    Ok(())
  }

  #[inline]
  pub fn decoder(&self) -> Decoder {
    self.reader.decoder()
  }

  #[cfg(feature = "flat-opc")]
  #[inline]
  fn raw_next(&mut self) -> Result<Event<'static>, SdkError> {
    if let Some(event) = self.pending.take()
      && let Some(event) = event.into_event()
    {
      return Ok(event);
    }

    self.buf.clear();
    Ok(self.reader.read_event_into(&mut self.buf)?.into_owned())
  }
}

impl<R: BufRead> XmlRead<'static> for IoReader<R> {
  #[inline]
  fn next(&mut self) -> Result<PayloadEvent<'static>, SdkError> {
    IoReader::next(self)
  }

  #[inline]
  fn next_tag_event(&mut self) -> Result<PayloadEvent<'static>, SdkError> {
    IoReader::next_tag_event(self)
  }

  #[inline]
  fn unread(&mut self, event: PayloadEvent<'static>) -> Result<(), SdkError> {
    IoReader::unread(self, event)
  }

  #[inline]
  fn decoder(&self) -> Decoder {
    IoReader::decoder(self)
  }

  #[inline]
  fn read_text(
    &mut self,
    end: quick_xml::name::QName<'_>,
    ty: &'static str,
    field: &'static str,
  ) -> Result<String, SdkError> {
    IoReader::read_text(self, end, ty, field)
  }

  #[inline]
  fn read_raw_empty_xml_bytes(
    &mut self,
    start: quick_xml::events::BytesStart<'static>,
  ) -> Result<Box<[u8]>, SdkError> {
    Ok(read_raw_empty_xml_start_bytes(start))
  }

  #[inline]
  fn read_raw_element_xml_bytes(
    &mut self,
    start: quick_xml::events::BytesStart<'static>,
  ) -> Result<Box<[u8]>, SdkError> {
    read_raw_element_xml_io_bytes(self, start)
  }
}

pub struct SliceReader<'de> {
  reader: Reader<&'de [u8]>,
  pending: Option<PayloadEvent<'de>>,
}

impl<'de> SliceReader<'de> {
  pub fn new(reader: Reader<&'de [u8]>) -> Self {
    Self {
      reader,
      pending: None,
    }
  }

  #[inline]
  pub fn next(&mut self) -> Result<PayloadEvent<'de>, SdkError> {
    if let Some(event) = self.pending.take() {
      return Ok(event);
    }

    loop {
      if let Some(event) = payload_event_from_event(self.reader.read_event()?) {
        return Ok(event);
      }
    }
  }

  #[inline]
  pub fn read_text(
    &mut self,
    end: quick_xml::name::QName<'_>,
    ty: &'static str,
    field: &'static str,
  ) -> Result<String, SdkError> {
    read_text_events(self, end, ty, field)
  }

  #[inline]
  pub fn next_tag_event(&mut self) -> Result<PayloadEvent<'de>, SdkError> {
    if let Some(event) = self.pending.take() {
      match event {
        PayloadEvent::Start(_, _) | PayloadEvent::End(_) | PayloadEvent::Eof => return Ok(event),
        PayloadEvent::Text(_) | PayloadEvent::CData(_) | PayloadEvent::GeneralRef(_) => {}
      }
    }

    loop {
      match self.reader.read_event()? {
        Event::Start(e) => return Ok(PayloadEvent::Start(e, false)),
        Event::Empty(e) => return Ok(PayloadEvent::Start(e, true)),
        Event::End(e) => return Ok(PayloadEvent::End(e)),
        Event::Eof => return Ok(PayloadEvent::Eof),
        _ => {}
      }
    }
  }

  #[inline]
  pub fn unread(&mut self, event: PayloadEvent<'de>) -> Result<(), SdkError> {
    if self.pending.is_some() {
      return Err(SdkError::CommonError(
        "xml reader unread buffer already occupied".to_string(),
      ));
    }

    self.pending = Some(event);
    Ok(())
  }

  #[inline]
  pub fn decoder(&self) -> Decoder {
    self.reader.decoder()
  }

  #[cfg(feature = "flat-opc")]
  #[inline]
  fn raw_next(&mut self) -> Result<Event<'de>, SdkError> {
    if let Some(event) = self.pending.take()
      && let Some(event) = event.into_event()
    {
      return Ok(event);
    }

    Ok(self.reader.read_event()?)
  }
}

impl<'de> XmlRead<'de> for SliceReader<'de> {
  #[inline]
  fn next(&mut self) -> Result<PayloadEvent<'de>, SdkError> {
    SliceReader::next(self)
  }

  #[inline]
  fn next_tag_event(&mut self) -> Result<PayloadEvent<'de>, SdkError> {
    SliceReader::next_tag_event(self)
  }

  #[inline]
  fn unread(&mut self, event: PayloadEvent<'de>) -> Result<(), SdkError> {
    SliceReader::unread(self, event)
  }

  #[inline]
  fn decoder(&self) -> Decoder {
    SliceReader::decoder(self)
  }

  #[inline]
  fn read_text(
    &mut self,
    end: quick_xml::name::QName<'_>,
    ty: &'static str,
    field: &'static str,
  ) -> Result<String, SdkError> {
    SliceReader::read_text(self, end, ty, field)
  }

  #[inline]
  fn read_raw_empty_xml_bytes(
    &mut self,
    start: quick_xml::events::BytesStart<'de>,
  ) -> Result<Box<[u8]>, SdkError> {
    Ok(read_raw_empty_xml_start_bytes(start))
  }

  #[inline]
  fn read_raw_element_xml_bytes(
    &mut self,
    start: quick_xml::events::BytesStart<'de>,
  ) -> Result<Box<[u8]>, SdkError> {
    read_raw_element_xml_borrowed_bytes(self, start)
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

pub fn resolve_relationship_target_path(parent_path: &str, target: &str) -> String {
  if target.starts_with('/') {
    resolve_zip_file_path(target)
  } else {
    let mut combined = String::with_capacity(parent_path.len() + target.len());
    combined.push_str(parent_path);
    combined.push_str(target);
    resolve_zip_file_path(&combined)
  }
}

#[inline]
pub(crate) fn from_reader_inner<R: BufRead>(reader: R) -> Result<IoReader<R>, SdkError> {
  let mut xml_reader = Reader::from_reader(reader);
  xml_reader.config_mut().check_end_names = false;
  Ok(IoReader::new(xml_reader))
}

#[inline]
pub(crate) fn from_bytes_inner(bytes: &[u8]) -> Result<SliceReader<'_>, SdkError> {
  let mut xml_reader = Reader::from_reader(bytes);
  xml_reader.config_mut().check_end_names = false;
  Ok(SliceReader::new(xml_reader))
}

#[cfg(feature = "parts")]
pub(crate) fn decode_utf16_xml_bytes(bytes: &[u8]) -> Result<Option<Vec<u8>>, SdkError> {
  let (endian, bytes) = match bytes {
    [0xFF, 0xFE, rest @ ..] => (Utf16Endian::Little, rest),
    [0xFE, 0xFF, rest @ ..] => (Utf16Endian::Big, rest),
    [b'<', 0, b'?', 0, ..] => (Utf16Endian::Little, bytes),
    [0, b'<', 0, b'?', ..] => (Utf16Endian::Big, bytes),
    _ => return Ok(None),
  };

  if bytes.len() % 2 != 0 {
    return Err(SdkError::CommonError(
      "invalid UTF-16 XML byte length".to_string(),
    ));
  }

  let code_units = bytes.chunks_exact(2).map(|chunk| match endian {
    Utf16Endian::Little => u16::from_le_bytes([chunk[0], chunk[1]]),
    Utf16Endian::Big => u16::from_be_bytes([chunk[0], chunk[1]]),
  });
  let xml = std::char::decode_utf16(code_units)
    .collect::<Result<String, _>>()
    .map_err(|err| SdkError::CommonError(format!("invalid UTF-16 XML: {err}")))?;

  Ok(Some(normalize_utf16_xml_decl(xml).into_bytes()))
}

#[derive(Clone, Copy)]
#[cfg(feature = "parts")]
enum Utf16Endian {
  Little,
  Big,
}

#[cfg(feature = "parts")]
fn normalize_utf16_xml_decl(mut xml: String) -> String {
  let Some(decl_end) = xml.find("?>").map(|end| end + 2) else {
    return xml;
  };
  if !xml[..decl_end].starts_with("<?xml") {
    return xml;
  }

  let Some(encoding_pos) = find_ascii_ignore_case(&xml[..decl_end], "encoding") else {
    return xml;
  };
  let bytes = xml.as_bytes();
  let mut pos = encoding_pos + "encoding".len();
  while pos < decl_end && bytes[pos].is_ascii_whitespace() {
    pos += 1;
  }
  if pos >= decl_end || bytes[pos] != b'=' {
    return xml;
  }
  pos += 1;
  while pos < decl_end && bytes[pos].is_ascii_whitespace() {
    pos += 1;
  }
  if pos >= decl_end || (bytes[pos] != b'"' && bytes[pos] != b'\'') {
    return xml;
  }

  let quote = bytes[pos];
  let value_start = pos + 1;
  let Some(value_end) = bytes[value_start..decl_end]
    .iter()
    .position(|&b| b == quote)
    .map(|offset| value_start + offset)
  else {
    return xml;
  };
  xml.replace_range(value_start..value_end, "UTF-8");
  xml
}

#[cfg(feature = "parts")]
fn find_ascii_ignore_case(haystack: &str, needle: &str) -> Option<usize> {
  haystack
    .as_bytes()
    .windows(needle.len())
    .position(|window| window.eq_ignore_ascii_case(needle.as_bytes()))
}

macro_rules! define_unsigned_decimal_attr_parser {
  ($attr_name:ident, $bytes_name:ident, $ty:ty) => {
    #[inline]
    pub(crate) fn $attr_name(
      attr: &Attribute<'_>,
      decoder: Decoder,
      ty: &'static str,
      field: &'static str,
    ) -> Result<$ty, SdkError> {
      let raw = attr.value.as_ref();
      if let Some(value) = $bytes_name(raw) {
        return Ok(value);
      }

      let value = attr.decoded_and_normalized_value(XmlVersion::Implicit1_0, decoder)?;
      match $bytes_name(value.as_bytes()) {
        Some(value) => Ok(value),
        None => Err(invalid_field_value_bytes(ty, field, value.as_bytes())),
      }
    }

    #[inline]
    pub(crate) fn $bytes_name(value: &[u8]) -> Option<$ty> {
      let digits = match value {
        [b'+', rest @ ..] => rest,
        _ => value,
      };
      if digits.is_empty() {
        return None;
      }

      let mut parsed: $ty = 0;
      for &digit in digits {
        if !digit.is_ascii_digit() {
          return None;
        }

        parsed = parsed
          .checked_mul(10)
          .and_then(|current| current.checked_add((digit - b'0') as $ty))?;
      }

      Some(parsed)
    }
  };
}

macro_rules! define_signed_decimal_attr_parser {
  ($attr_name:ident, $bytes_name:ident, $ty:ty) => {
    #[inline]
    pub(crate) fn $attr_name(
      attr: &Attribute<'_>,
      decoder: Decoder,
      ty: &'static str,
      field: &'static str,
    ) -> Result<$ty, SdkError> {
      let raw = attr.value.as_ref();
      if let Some(value) = $bytes_name(raw) {
        return Ok(value);
      }

      let value = attr.decoded_and_normalized_value(XmlVersion::Implicit1_0, decoder)?;
      match $bytes_name(value.as_bytes()) {
        Some(value) => Ok(value),
        None => Err(invalid_field_value_bytes(ty, field, value.as_bytes())),
      }
    }

    #[inline]
    pub(crate) fn $bytes_name(value: &[u8]) -> Option<$ty> {
      let (negative, digits) = match value {
        [b'-', rest @ ..] => (true, rest),
        [b'+', rest @ ..] => (false, rest),
        _ => (false, value),
      };
      if digits.is_empty() {
        return None;
      }

      let mut parsed: $ty = 0;
      for &digit in digits {
        if !digit.is_ascii_digit() {
          return None;
        }

        parsed = parsed.checked_mul(10).and_then(|current| {
          if negative {
            current.checked_sub((digit - b'0') as $ty)
          } else {
            current.checked_add((digit - b'0') as $ty)
          }
        })?;
      }

      Some(parsed)
    }
  };
}

define_unsigned_decimal_attr_parser!(parse_u8_attr, parse_u8_bytes, u8);
define_signed_decimal_attr_parser!(parse_i8_attr, parse_i8_bytes, i8);
define_unsigned_decimal_attr_parser!(parse_u16_attr, parse_u16_bytes, u16);
define_signed_decimal_attr_parser!(parse_i16_attr, parse_i16_bytes, i16);
define_unsigned_decimal_attr_parser!(parse_u32_attr, parse_u32_bytes, u32);
define_signed_decimal_attr_parser!(parse_i32_attr, parse_i32_bytes, i32);
define_unsigned_decimal_attr_parser!(parse_u64_attr, parse_u64_bytes, u64);
define_signed_decimal_attr_parser!(parse_i64_attr, parse_i64_bytes, i64);

#[inline]
pub(crate) fn parse_f32_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<f32, SdkError> {
  if let Some(value) = parse_f32_bytes_raw(attr.value.as_ref()) {
    return Ok(value);
  }

  let value = attr.decoded_and_normalized_value(XmlVersion::Implicit1_0, decoder)?;
  parse_f32_bytes(value.as_bytes(), ty, field)
}

#[inline]
pub(crate) fn parse_f64_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<f64, SdkError> {
  if let Some(value) = parse_f64_bytes_raw(attr.value.as_ref()) {
    return Ok(value);
  }

  let value = attr.decoded_and_normalized_value(XmlVersion::Implicit1_0, decoder)?;
  parse_f64_bytes(value.as_bytes(), ty, field)
}

#[inline]
fn parse_f32_bytes(value: &[u8], ty: &'static str, field: &'static str) -> Result<f32, SdkError> {
  match parse_f32_bytes_raw(value) {
    Some(value) => Ok(value),
    None => Err(invalid_field_value_bytes(ty, field, value)),
  }
}

#[inline]
fn parse_f64_bytes(value: &[u8], ty: &'static str, field: &'static str) -> Result<f64, SdkError> {
  match parse_f64_bytes_raw(value) {
    Some(value) => Ok(value),
    None => Err(invalid_field_value_bytes(ty, field, value)),
  }
}

#[inline]
pub(crate) fn parse_f32_bytes_raw(value: &[u8]) -> Option<f32> {
  <f32 as lexical_parse_float::FromLexical>::from_lexical(value).ok()
}

#[inline]
pub(crate) fn parse_f64_bytes_raw(value: &[u8]) -> Option<f64> {
  <f64 as lexical_parse_float::FromLexical>::from_lexical(value).ok()
}

#[inline]
pub(crate) fn parse_twips_measure_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
) -> Result<crate::simple_type::TwipsMeasureValue, SdkError> {
  if let Ok(value) = crate::simple_type::TwipsMeasureValue::from_bytes(attr.value.as_ref()) {
    return Ok(value);
  }
  let value = attr.decoded_and_normalized_value(XmlVersion::Implicit1_0, decoder)?;
  parse_twips_measure_value(value.as_ref())
}

#[inline]
pub(crate) fn parse_twips_measure_value(
  value: impl AsRef<str>,
) -> Result<crate::simple_type::TwipsMeasureValue, SdkError> {
  let value = value.as_ref();
  match crate::simple_type::TwipsMeasureValue::from_bytes(value.as_bytes()) {
    Ok(value) => Ok(value),
    Err(_) => Err(invalid_field_value("TwipsMeasureValue", "value", value)),
  }
}

#[inline]
pub(crate) fn parse_signed_twips_measure_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
) -> Result<crate::simple_type::SignedTwipsMeasureValue, SdkError> {
  if let Ok(value) = crate::simple_type::SignedTwipsMeasureValue::from_bytes(attr.value.as_ref()) {
    return Ok(value);
  }
  let value = attr.decoded_and_normalized_value(XmlVersion::Implicit1_0, decoder)?;
  parse_signed_twips_measure_value(value.as_ref())
}

#[inline]
pub(crate) fn parse_signed_twips_measure_value(
  value: impl AsRef<str>,
) -> Result<crate::simple_type::SignedTwipsMeasureValue, SdkError> {
  let value = value.as_ref();
  match crate::simple_type::SignedTwipsMeasureValue::from_bytes(value.as_bytes()) {
    Ok(value) => Ok(value),
    Err(_) => Err(invalid_field_value(
      "SignedTwipsMeasureValue",
      "value",
      value,
    )),
  }
}

#[inline]
pub(crate) fn parse_decimal_number_or_percent_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
) -> Result<crate::simple_type::DecimalNumberOrPercentValue, SdkError> {
  if let Ok(value) =
    crate::simple_type::DecimalNumberOrPercentValue::from_bytes(attr.value.as_ref())
  {
    return Ok(value);
  }
  let value = attr.decoded_and_normalized_value(XmlVersion::Implicit1_0, decoder)?;
  parse_decimal_number_or_percent_value(value.as_ref())
}

#[inline]
pub(crate) fn parse_decimal_number_or_percent_value(
  value: impl AsRef<str>,
) -> Result<crate::simple_type::DecimalNumberOrPercentValue, SdkError> {
  let value = value.as_ref();
  match crate::simple_type::DecimalNumberOrPercentValue::from_bytes(value.as_bytes()) {
    Ok(value) => Ok(value),
    Err(_) => Err(invalid_field_value(
      "DecimalNumberOrPercentValue",
      "value",
      value,
    )),
  }
}

#[inline]
pub(crate) fn parse_measurement_or_percent_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
) -> Result<crate::simple_type::MeasurementOrPercentValue, SdkError> {
  if let Ok(value) = crate::simple_type::MeasurementOrPercentValue::from_bytes(attr.value.as_ref())
  {
    return Ok(value);
  }
  let value = attr.decoded_and_normalized_value(XmlVersion::Implicit1_0, decoder)?;
  parse_measurement_or_percent_value(value.as_ref())
}

#[inline]
pub(crate) fn parse_measurement_or_percent_value(
  value: impl AsRef<str>,
) -> Result<crate::simple_type::MeasurementOrPercentValue, SdkError> {
  let value = value.as_ref();
  match crate::simple_type::MeasurementOrPercentValue::from_bytes(value.as_bytes()) {
    Ok(value) => Ok(value),
    Err(_) => Err(invalid_field_value(
      "MeasurementOrPercentValue",
      "value",
      value,
    )),
  }
}

#[inline]
pub(crate) fn parse_enum_attr<T>(attr: &Attribute<'_>, decoder: Decoder) -> Result<T, SdkError>
where
  T: crate::sdk::SdkEnum,
{
  if let Some(value) = T::try_from_xml_bytes(attr.value.as_ref()) {
    return Ok(value);
  }

  let value = attr.decoded_and_normalized_value(XmlVersion::Implicit1_0, decoder)?;
  T::from_xml_bytes(value.as_bytes())
}

#[inline]
pub(crate) fn parse_value<T>(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<T, SdkError>
where
  T: std::str::FromStr,
{
  match value.parse::<T>() {
    Ok(value) => Ok(value),
    Err(_) => Err(invalid_field_value(ty, field, value)),
  }
}

#[inline]
pub(crate) fn parse_text_child_value<T>(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<T, SdkError>
where
  T: std::str::FromStr,
{
  match value.parse::<T>() {
    Ok(value) => Ok(value),
    Err(_) => {
      let trimmed = value.trim_matches([' ', '\t', '\n', '\r']);
      if trimmed.len() == value.len() {
        return Err(invalid_field_value(ty, field, value));
      }

      match trimmed.parse::<T>() {
        Ok(value) => Ok(value),
        Err(_) => Err(invalid_field_value(ty, field, value)),
      }
    }
  }
}

pub(crate) fn parse_list_attr<T>(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<Vec<T>, SdkError>
where
  T: std::str::FromStr,
{
  let value = attr.decoded_and_normalized_value(XmlVersion::Implicit1_0, decoder)?;
  parse_list_value(value.as_ref(), ty, field)
}

#[inline]
pub(crate) fn parse_bytes_list_attr<T, Parse>(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
  parse: Parse,
) -> Result<Vec<T>, SdkError>
where
  Parse: Fn(&[u8]) -> Option<T>,
{
  if let Some(value) = try_parse_bytes_list_value(attr.value.as_ref(), &parse) {
    return Ok(value);
  }

  let value = attr.decoded_and_normalized_value(XmlVersion::Implicit1_0, decoder)?;
  try_parse_bytes_list_value(value.as_bytes(), &parse)
    .ok_or_else(|| invalid_field_value_bytes(ty, field, value.as_bytes()))
}

#[inline]
pub(crate) fn try_parse_bytes_list_value<T, Parse>(value: &[u8], parse: &Parse) -> Option<Vec<T>>
where
  Parse: Fn(&[u8]) -> Option<T>,
{
  value
    .split(|byte| byte.is_ascii_whitespace())
    .filter(|item| !item.is_empty())
    .map(parse)
    .collect()
}

#[inline]
pub(crate) fn parse_list_value<T>(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<Vec<T>, SdkError>
where
  T: std::str::FromStr,
{
  value
    .split_whitespace()
    .map(|item| parse_value::<T>(item, ty, field))
    .collect()
}

#[inline]
fn invalid_field_value_bytes(ty: &'static str, field: &'static str, value: &[u8]) -> SdkError {
  invalid_field_value(ty, field, String::from_utf8_lossy(value).into_owned())
}

fn read_de_event<'xml, R: XmlRead<'xml>>(
  xml_reader: &mut R,
  ty: &'static str,
  field: &'static str,
) -> Result<DeEvent<'xml>, SdkError> {
  match xml_reader.next()? {
    PayloadEvent::Start(e, empty) => Ok(DeEvent::Start(e, empty)),
    PayloadEvent::End(e) => Ok(DeEvent::End(e)),
    PayloadEvent::Text(text) => read_de_text_from_text(xml_reader, text, ty, field),
    PayloadEvent::CData(text) => drain_de_text(xml_reader, text.xml10_content()?, ty, field),
    PayloadEvent::GeneralRef(text) => {
      let mut value = String::new();
      append_de_general_ref(&mut value, text, xml_reader.decoder(), ty, field)?;
      drain_de_text(xml_reader, Cow::Owned(value), ty, field)
    }
    PayloadEvent::Eof => Ok(DeEvent::Eof),
  }
}

fn read_de_text_from_text<'xml, R: XmlRead<'xml>>(
  xml_reader: &mut R,
  text: quick_xml::events::BytesText<'xml>,
  ty: &'static str,
  field: &'static str,
) -> Result<DeEvent<'xml>, SdkError> {
  match xml_reader.next()? {
    PayloadEvent::Text(next_text) => {
      let mut value = text.xml10_content()?;
      append_de_text_content(&mut value, next_text.xml10_content()?);
      drain_de_text(xml_reader, value, ty, field)
    }
    PayloadEvent::CData(next_text) => {
      let mut value = text.xml10_content()?;
      append_de_text_content(&mut value, next_text.xml10_content()?);
      drain_de_text(xml_reader, value, ty, field)
    }
    PayloadEvent::GeneralRef(next_text) => {
      let mut value = text.xml10_content()?;
      append_de_general_ref(value.to_mut(), next_text, xml_reader.decoder(), ty, field)?;
      drain_de_text(xml_reader, value, ty, field)
    }
    event => {
      xml_reader.unread(event)?;
      Ok(DeEvent::FastBytesText(text))
    }
  }
}

fn drain_de_text<'xml, R: XmlRead<'xml>>(
  xml_reader: &mut R,
  mut value: Cow<'xml, str>,
  ty: &'static str,
  field: &'static str,
) -> Result<DeEvent<'xml>, SdkError> {
  loop {
    match xml_reader.next()? {
      PayloadEvent::Text(text) => append_de_text_content(&mut value, text.xml10_content()?),
      PayloadEvent::CData(text) => append_de_text_content(&mut value, text.xml10_content()?),
      PayloadEvent::GeneralRef(text) => {
        append_de_general_ref(value.to_mut(), text, xml_reader.decoder(), ty, field)?;
      }
      event => {
        xml_reader.unread(event)?;
        return Ok(DeEvent::Text(Text::new(value)));
      }
    }
  }
}

#[inline]
fn append_de_text_content<'xml>(value: &mut Cow<'xml, str>, text: Cow<'_, str>) {
  value.to_mut().push_str(text.as_ref());
}

fn append_de_general_ref(
  value: &mut String,
  text: quick_xml::events::BytesRef<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<(), SdkError> {
  let entity = decoder.decode(&text)?;
  if let Some(number) = entity.strip_prefix('#') {
    let Some(ch) = parse_de_char_ref(number) else {
      return Err(invalid_field_value(ty, field, entity.to_string()));
    };
    value.push(ch);
    return Ok(());
  }

  let Some(entity) = quick_xml::escape::resolve_predefined_entity(entity.as_ref()) else {
    return Err(invalid_field_value(ty, field, entity.to_string()));
  };
  value.push_str(entity);
  Ok(())
}

fn parse_de_char_ref(number: &str) -> Option<char> {
  let code_point = if let Some(hex) = number.strip_prefix('x') {
    parse_unsigned_radix(hex, 16)?
  } else {
    parse_unsigned_radix(number, 10)?
  };
  if code_point == 0 {
    return None;
  }
  char::from_u32(code_point)
}

#[inline]
fn parse_unsigned_radix(src: &str, radix: u32) -> Option<u32> {
  match src.as_bytes().first().copied() {
    Some(b'+') | Some(b'-') => None,
    _ => u32::from_str_radix(src, radix).ok(),
  }
}

#[inline]
pub(crate) fn fast_bytes_text_to_string(
  text: quick_xml::events::BytesText<'_>,
  ty: &'static str,
  field: &'static str,
) -> Result<String, SdkError> {
  match String::from_utf8(text.into_inner().into_owned()) {
    Ok(value) => Ok(value),
    Err(err) => Err(invalid_field_value(ty, field, err.to_string())),
  }
}

#[inline]
pub(crate) fn append_de_text_field(value: &mut Option<String>, text: Text<'_>) {
  if let Some(value) = value {
    value.push_str(text.as_ref());
  } else {
    *value = Some(text.into_string());
  }
}

#[inline]
fn append_cow_text_field(value: &mut Option<String>, text: Cow<'_, str>) {
  if let Some(value) = value {
    value.push_str(text.as_ref());
  } else {
    *value = Some(match text {
      Cow::Borrowed(text) => text.to_string(),
      Cow::Owned(text) => text,
    });
  }
}

#[inline]
pub(crate) fn append_fast_bytes_text_field(
  value: &mut Option<String>,
  text: quick_xml::events::BytesText<'_>,
  ty: &'static str,
  field: &'static str,
) -> Result<(), SdkError> {
  let text = fast_bytes_text_to_string(text, ty, field)?;
  if let Some(value) = value {
    value.push_str(&text);
  } else {
    *value = Some(text);
  }
  Ok(())
}

#[inline]
fn read_text_events<'xml, R: XmlRead<'xml>>(
  xml_reader: &mut R,
  end: quick_xml::name::QName<'_>,
  ty: &'static str,
  field: &'static str,
) -> Result<String, SdkError> {
  let mut value = None;
  loop {
    match xml_reader.next_de_event(ty, field)? {
      DeEvent::FastBytesText(text) => append_fast_bytes_text_field(&mut value, text, ty, field)?,
      DeEvent::Text(text) => append_de_text_field(&mut value, text),
      DeEvent::End(e) if e.name() == end => return Ok(value.unwrap_or_default()),
      DeEvent::Start(e, _) => {
        return Err(unexpected_tag(ty, "text content", xml_local_name(e.name())));
      }
      DeEvent::Eof => return Err(unexpected_eof(ty)),
      _ => {}
    }
  }
}

#[inline]
pub(crate) fn read_text_child_value<'xml, R, T, RawParse, TextParse>(
  xml_reader: &mut R,
  end: quick_xml::name::QName<'_>,
  ty: &'static str,
  field: &'static str,
  parse_raw: RawParse,
  parse_text: TextParse,
) -> Result<T, SdkError>
where
  R: XmlRead<'xml>,
  RawParse: FnOnce(&[u8]) -> Option<T>,
  TextParse: FnOnce(&str) -> Result<T, SdkError>,
{
  let mut value = None;
  let mut event = xml_reader.next_de_event(ty, field)?;
  loop {
    match event {
      DeEvent::FastBytesText(text) => {
        let after_text = xml_reader.next_de_event(ty, field)?;
        match after_text {
          DeEvent::End(e) if e.name() == end => {
            if value.is_none()
              && let Some(parsed) = parse_raw(text.as_ref())
            {
              return Ok(parsed);
            }

            append_fast_bytes_text_field(&mut value, text, ty, field)?;
            let value = value.unwrap_or_default();
            return parse_text(value.as_ref());
          }
          event_after_text => {
            let text = text.xml10_content()?;
            append_cow_text_field(&mut value, text);
            event = event_after_text;
            continue;
          }
        }
      }
      DeEvent::Text(text) => append_de_text_field(&mut value, text),
      DeEvent::End(e) if e.name() == end => {
        let value = value.unwrap_or_default();
        return parse_text(value.as_ref());
      }
      DeEvent::Start(e, _) => {
        return Err(unexpected_tag(ty, "text content", xml_local_name(e.name())));
      }
      DeEvent::Eof => return Err(unexpected_eof(ty)),
      _ => {}
    }
    event = xml_reader.next_de_event(ty, field)?;
  }
}

macro_rules! define_integer_text_child_reader {
  ($name:ident, $parse_bytes:ident, $ty:ty) => {
    #[inline]
    pub(crate) fn $name<'xml, R>(
      xml_reader: &mut R,
      end: quick_xml::name::QName<'_>,
      ty: &'static str,
      field: &'static str,
    ) -> Result<$ty, SdkError>
    where
      R: XmlRead<'xml>,
    {
      read_text_child_value(xml_reader, end, ty, field, $parse_bytes, |value| {
        parse_text_child_value::<$ty>(value, ty, field)
      })
    }
  };
}

define_integer_text_child_reader!(read_u8_text_child_value, parse_u8_bytes, u8);
define_integer_text_child_reader!(read_i8_text_child_value, parse_i8_bytes, i8);
define_integer_text_child_reader!(read_u16_text_child_value, parse_u16_bytes, u16);
define_integer_text_child_reader!(read_i16_text_child_value, parse_i16_bytes, i16);
define_integer_text_child_reader!(read_u32_text_child_value, parse_u32_bytes, u32);
define_integer_text_child_reader!(read_i32_text_child_value, parse_i32_bytes, i32);
define_integer_text_child_reader!(read_u64_text_child_value, parse_u64_bytes, u64);
define_integer_text_child_reader!(read_i64_text_child_value, parse_i64_bytes, i64);

#[inline]
pub(crate) fn read_f32_text_child_value<'xml, R>(
  xml_reader: &mut R,
  end: quick_xml::name::QName<'_>,
  ty: &'static str,
  field: &'static str,
) -> Result<f32, SdkError>
where
  R: XmlRead<'xml>,
{
  read_text_child_value(xml_reader, end, ty, field, parse_f32_bytes_raw, |value| {
    parse_text_child_value::<f32>(value, ty, field)
  })
}

#[inline]
pub(crate) fn read_f64_text_child_value<'xml, R>(
  xml_reader: &mut R,
  end: quick_xml::name::QName<'_>,
  ty: &'static str,
  field: &'static str,
) -> Result<f64, SdkError>
where
  R: XmlRead<'xml>,
{
  read_text_child_value(xml_reader, end, ty, field, parse_f64_bytes_raw, |value| {
    parse_text_child_value::<f64>(value, ty, field)
  })
}

#[inline]
pub(crate) fn read_enum_text_child_value<'xml, R, T>(
  xml_reader: &mut R,
  end: quick_xml::name::QName<'_>,
  ty: &'static str,
  field: &'static str,
) -> Result<T, SdkError>
where
  R: XmlRead<'xml>,
  T: crate::sdk::SdkEnum,
{
  read_text_child_value(xml_reader, end, ty, field, T::try_from_xml_bytes, |value| {
    T::from_xml_bytes(value.as_bytes())
  })
}

#[cfg(feature = "flat-opc")]
pub(crate) fn read_outer_xml_borrowed<'de>(
  xml_reader: &mut SliceReader<'de>,
  start: quick_xml::events::BytesStart<'de>,
  empty_tag: bool,
) -> Result<String, SdkError> {
  let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));

  if empty_tag {
    writer.write_event(Event::Empty(start))?;
  } else {
    writer.write_event(Event::Start(start))?;

    let mut depth = 1usize;
    loop {
      let event = xml_reader.raw_next()?;
      match &event {
        Event::Start(_) => {
          depth += 1;
        }
        Event::End(_) => {
          depth -= 1;
        }
        Event::Eof => return Err(unexpected_eof("read_outer_xml")),
        _ => {}
      }

      writer.write_event(event)?;

      if depth == 0 {
        break;
      }
    }
  }

  String::from_utf8(writer.into_inner().into_inner())
    .map_err(|err| SdkError::CommonError(format!("invalid utf-8 xml fragment: {err}")))
}

const RAW_EMPTY_XML_EXTRA_LEN: usize = 3;
const RAW_ELEMENT_XML_EXTRA_LEN: usize = 5;

#[inline]
fn raw_xml_bytes_to_string(xml: Box<[u8]>) -> Result<String, SdkError> {
  String::from_utf8(xml.into_vec())
    .map_err(|err| SdkError::CommonError(format!("invalid utf-8 xml: {err}")))
}

#[inline]
fn read_raw_empty_xml_start_bytes(start: quick_xml::events::BytesStart<'_>) -> Box<[u8]> {
  let start: &[u8] = start.as_ref();
  let mut xml = Vec::with_capacity(start.len() + RAW_EMPTY_XML_EXTRA_LEN);
  xml.push(b'<');
  xml.extend_from_slice(start);
  xml.extend_from_slice(b"/>");
  xml.into_boxed_slice()
}

#[inline]
fn build_raw_element_xml_bytes(start: &[u8], inner: &[u8], end_name: &[u8]) -> Box<[u8]> {
  let mut xml =
    Vec::with_capacity(start.len() + inner.len() + end_name.len() + RAW_ELEMENT_XML_EXTRA_LEN);
  xml.push(b'<');
  xml.extend_from_slice(start);
  xml.push(b'>');
  xml.extend_from_slice(inner);
  xml.extend_from_slice(b"</");
  xml.extend_from_slice(end_name);
  xml.push(b'>');
  xml.into_boxed_slice()
}

#[inline]
pub(crate) fn read_raw_element_xml_borrowed_bytes<'de>(
  xml_reader: &mut SliceReader<'de>,
  start: quick_xml::events::BytesStart<'de>,
) -> Result<Box<[u8]>, SdkError> {
  let start_bytes: &[u8] = start.as_ref();
  let end_name = start.name();
  let inner = xml_reader.reader.read_text(end_name)?;
  Ok(build_raw_element_xml_bytes(
    start_bytes,
    inner.as_ref(),
    end_name.as_ref(),
  ))
}

#[cfg(feature = "flat-opc")]
pub(crate) fn read_outer_xml_io<R: BufRead>(
  xml_reader: &mut IoReader<R>,
  start: quick_xml::events::BytesStart<'static>,
  empty_tag: bool,
) -> Result<String, SdkError> {
  let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));

  if empty_tag {
    writer.write_event(Event::Empty(start))?;
  } else {
    writer.write_event(Event::Start(start))?;

    let mut depth = 1usize;
    loop {
      let event = xml_reader.raw_next()?;
      match &event {
        Event::Start(_) => {
          depth += 1;
        }
        Event::End(_) => {
          depth -= 1;
        }
        Event::Eof => return Err(unexpected_eof("read_outer_xml_io")),
        _ => {}
      }

      writer.write_event(event)?;

      if depth == 0 {
        break;
      }
    }
  }

  String::from_utf8(writer.into_inner().into_inner())
    .map_err(|err| SdkError::CommonError(format!("invalid utf-8 xml fragment: {err}")))
}

#[inline]
pub(crate) fn read_raw_element_xml_io_bytes<R: BufRead>(
  xml_reader: &mut IoReader<R>,
  start: quick_xml::events::BytesStart<'static>,
) -> Result<Box<[u8]>, SdkError> {
  let end_name = start.name();
  let start_bytes: &[u8] = start.as_ref();
  let inner = xml_reader
    .reader
    .read_text_into(end_name, &mut xml_reader.buf)?;
  Ok(build_raw_element_xml_bytes(
    start_bytes,
    inner.as_ref(),
    end_name.as_ref(),
  ))
}

#[cfg(feature = "mce")]
const MC_ALTERNATE_CONTENT_NAMES: &[&[u8]] = &[b"mc:AlternateContent", b"AlternateContent"];
#[cfg(feature = "mce")]
const MC_CHOICE_NAMES: &[&[u8]] = &[b"mc:Choice", b"Choice"];
#[cfg(feature = "mce")]
const MC_FALLBACK_NAMES: &[&[u8]] = &[b"mc:Fallback", b"Fallback"];

#[cfg(feature = "mce")]
pub(crate) fn mce_choice_replacement_child_bytes(
  xml: &[u8],
  settings: &crate::sdk::MarkupCompatibilityProcessSettings,
  context: &crate::sdk::MceContext<'_>,
) -> Result<Option<Vec<Box<[u8]>>>, SdkError> {
  let mut reader = Reader::from_reader(xml);
  reader.config_mut().check_end_names = false;
  let mut fallback = None;

  loop {
    match reader.read_event()? {
      Event::Start(e) if qname_in(e.name().as_ref(), MC_ALTERNATE_CONTENT_NAMES) => {
        let namespaces = namespace_decls(&e)?;
        loop {
          match reader.read_event()? {
            Event::Start(e) if qname_in(e.name().as_ref(), MC_CHOICE_NAMES) => {
              let choice_namespaces = namespace_decls(&e)?;
              let requires = attr_value(&reader, &e, b"Requires")?;
              let children = read_mce_container_children_bytes(&mut reader, MC_CHOICE_NAMES)?;
              if choice_requires_supported(
                requires.as_deref(),
                context,
                &[choice_namespaces.as_slice(), namespaces.as_slice()],
                settings.target_file_format_version,
              )? {
                return Ok(Some(children));
              }
            }
            Event::Empty(e) if qname_in(e.name().as_ref(), MC_CHOICE_NAMES) => {
              let choice_namespaces = namespace_decls(&e)?;
              let requires = attr_value(&reader, &e, b"Requires")?;
              if choice_requires_supported(
                requires.as_deref(),
                context,
                &[choice_namespaces.as_slice(), namespaces.as_slice()],
                settings.target_file_format_version,
              )? {
                return Ok(Some(Vec::new()));
              }
            }
            Event::Start(e) if qname_in(e.name().as_ref(), MC_FALLBACK_NAMES) => {
              fallback = Some(read_mce_container_children_bytes(
                &mut reader,
                MC_FALLBACK_NAMES,
              )?);
            }
            Event::Empty(e) if qname_in(e.name().as_ref(), MC_FALLBACK_NAMES) => {
              fallback = Some(Vec::new());
            }
            Event::Start(e) => {
              skip_element(&mut reader, e.name())?;
            }
            Event::End(e) if qname_in(e.name().as_ref(), MC_ALTERNATE_CONTENT_NAMES) => {
              return Ok(Some(fallback.unwrap_or_default()));
            }
            Event::Eof => return Err(unexpected_eof("mce AlternateContent")),
            _ => {}
          }
        }
      }
      Event::Empty(e) if qname_in(e.name().as_ref(), MC_ALTERNATE_CONTENT_NAMES) => {
        return Ok(Some(Vec::new()));
      }
      Event::Start(e) => {
        return mce_unknown_element_replacement_bytes(&mut reader, e, context, false);
      }
      Event::Empty(e) => {
        return mce_unknown_element_replacement_bytes(&mut reader, e, context, true);
      }
      Event::Eof => return Ok(None),
      _ => {}
    }
  }
}

#[cfg(feature = "mce")]
fn mce_unknown_element_replacement_bytes(
  reader: &mut Reader<&[u8]>,
  start: quick_xml::events::BytesStart<'_>,
  context: &crate::sdk::MceContext<'_>,
  empty_tag: bool,
) -> Result<Option<Vec<Box<[u8]>>>, SdkError> {
  let qname = start.name();
  let qname = qname.as_ref();
  let namespaces = namespace_decls(&start)?;
  let namespace_frames = [namespaces.as_slice()];
  let is_ignorable = qname_prefix(qname)
    .and_then(|prefix| namespace_for_prefix_with_frames(context, &namespace_frames, prefix))
    .is_some_and(|ns| context.is_ignorable_namespace_bytes(ns));

  if is_ignorable && context.is_preserved_element_qname_with_current_bytes(&namespaces, qname) {
    return Ok(None);
  }

  if is_ignorable && context.is_process_content_qname_with_current_bytes(&namespaces, qname) {
    if empty_tag {
      return Ok(Some(Vec::new()));
    }
    return read_mce_container_children_qname_bytes(reader, start.name()).map(Some);
  }

  if is_ignorable {
    if !empty_tag {
      skip_element(reader, start.name())?;
    }
    return Ok(Some(Vec::new()));
  }

  if !qname.contains(&b':') {
    if !empty_tag {
      skip_element(reader, start.name())?;
    }
    return Ok(None);
  }

  if !empty_tag {
    skip_element(reader, start.name())?;
  }
  Ok(None)
}

#[cfg(feature = "mce")]
fn read_mce_container_children_bytes(
  reader: &mut Reader<&[u8]>,
  end_names: &[&[u8]],
) -> Result<Vec<Box<[u8]>>, SdkError> {
  let mut children = Vec::new();
  loop {
    match reader.read_event()? {
      Event::Start(e) => {
        children.push(read_outer_xml_from_str_reader_bytes(reader, e, false)?);
      }
      Event::Empty(e) => {
        children.push(read_outer_xml_from_str_reader_bytes(reader, e, true)?);
      }
      Event::End(e) if qname_in(e.name().as_ref(), end_names) => return Ok(children),
      Event::Eof => return Err(unexpected_eof("mce choice/fallback")),
      _ => {}
    }
  }
}

#[cfg(feature = "mce")]
fn read_mce_container_children_qname_bytes(
  reader: &mut Reader<&[u8]>,
  end_name: quick_xml::name::QName<'_>,
) -> Result<Vec<Box<[u8]>>, SdkError> {
  let mut children = Vec::new();
  loop {
    match reader.read_event()? {
      Event::Start(e) => {
        children.push(read_outer_xml_from_str_reader_bytes(reader, e, false)?);
      }
      Event::Empty(e) => {
        children.push(read_outer_xml_from_str_reader_bytes(reader, e, true)?);
      }
      Event::End(e) if e.name() == end_name => return Ok(children),
      Event::Eof => return Err(unexpected_eof("mce choice/fallback")),
      _ => {}
    }
  }
}

#[cfg(feature = "mce")]
fn read_outer_xml_from_str_reader_bytes(
  reader: &mut Reader<&[u8]>,
  start: quick_xml::events::BytesStart<'_>,
  empty_tag: bool,
) -> Result<Box<[u8]>, SdkError> {
  if empty_tag {
    return Ok(read_raw_empty_xml_start_bytes(start));
  }

  let start_bytes = start.as_ref();
  let end_name = start.name();
  let inner = reader.read_text(end_name)?;
  Ok(build_raw_element_xml_bytes(
    start_bytes,
    inner.as_ref(),
    end_name.as_ref(),
  ))
}

#[cfg(feature = "mce")]
fn qname_prefix(qname: &[u8]) -> Option<&[u8]> {
  qname
    .iter()
    .position(|byte| *byte == b':')
    .map(|index| &qname[..index])
}

#[cfg(feature = "mce")]
fn skip_element(
  reader: &mut Reader<&[u8]>,
  name: quick_xml::name::QName<'_>,
) -> Result<(), SdkError> {
  reader.read_to_end(name)?;
  Ok(())
}

#[cfg(feature = "mce")]
fn attr_value(
  reader: &Reader<&[u8]>,
  start: &quick_xml::events::BytesStart<'_>,
  name: &[u8],
) -> Result<Option<Box<[u8]>>, SdkError> {
  for attr in start.attributes() {
    let attr = attr?;
    if attr.key.as_ref() == name {
      return Ok(Some(
        attr
          .decoded_and_normalized_value(XmlVersion::Implicit1_0, reader.decoder())?
          .into_owned()
          .into_bytes()
          .into_boxed_slice(),
      ));
    }
  }
  Ok(None)
}

#[cfg(feature = "mce")]
fn choice_requires_supported(
  requires: Option<&[u8]>,
  context: &crate::sdk::MceContext<'_>,
  namespace_frames: &[&[crate::common::XmlNamespace]],
  target: crate::sdk::FileFormatVersion,
) -> Result<bool, SdkError> {
  let Some(requires) = requires else {
    return Ok(false);
  };
  let mut supported = true;
  crate::sdk::for_each_mce_prefix(requires, |prefix| {
    let Some(ns) = namespace_for_prefix_with_frames(context, namespace_frames, prefix) else {
      supported = false;
      return Ok(());
    };
    if !namespace_supported(ns, target) {
      supported = false;
    }
    Ok(())
  })?;
  Ok(supported)
}

#[cfg(feature = "mce")]
fn namespace_for_prefix_with_frames<'a>(
  context: &'a crate::sdk::MceContext<'_>,
  namespace_frames: &'a [&'a [crate::common::XmlNamespace]],
  prefix: &[u8],
) -> Option<&'a [u8]> {
  namespace_frames
    .iter()
    .find_map(|namespaces| namespace_for_prefix_in_frame(namespaces, prefix))
    .or_else(|| context.namespace_for_prefix_bytes(prefix))
}

#[cfg(feature = "mce")]
fn namespace_for_prefix_in_frame<'a>(
  namespaces: &'a [crate::common::XmlNamespace],
  prefix: &[u8],
) -> Option<&'a [u8]> {
  namespaces.iter().rev().find_map(|namespace| {
    let (namespace_prefix, namespace_uri) = namespace.parts();
    (namespace_prefix == prefix).then_some(namespace_uri)
  })
}

#[cfg(feature = "mce")]
fn namespace_decls(
  start: &quick_xml::events::BytesStart<'_>,
) -> Result<Vec<crate::common::XmlNamespace>, SdkError> {
  let mut namespaces = Vec::new();
  for attr in start.attributes() {
    let attr = attr?;
    let key = attr.key.as_ref();
    if let Some(prefix) = key.strip_prefix(b"xmlns:") {
      namespaces.push(crate::common::XmlNamespace::raw(
        prefix,
        attr.value.as_ref(),
      ));
    }
  }
  Ok(namespaces)
}

#[cfg(feature = "mce")]
fn namespace_supported(ns: &[u8], target: crate::sdk::FileFormatVersion) -> bool {
  crate::sdk::namespace_supported(ns, target)
}

#[cfg(feature = "mce")]
fn qname_in(name: &[u8], expected: &[&[u8]]) -> bool {
  if expected.contains(&name) {
    return true;
  }

  let mut index = name.len();
  while index > 0 {
    index -= 1;
    if name[index] == b':' {
      return expected.contains(&&name[index + 1..]);
    }
  }

  false
}

#[inline]
pub(crate) fn read_root_start_borrowed<'de>(
  reader: &mut SliceReader<'de>,
  owner: &'static str,
  local_name: &'static [u8],
) -> Result<
  (
    quick_xml::events::BytesStart<'de>,
    bool,
    crate::common::XmlHeaderType,
  ),
  SdkError,
> {
  loop {
    match reader.next_tag_event()? {
      PayloadEvent::Start(e, empty) => {
        if xml_local_name(e.name()) == local_name {
          return Ok((e, empty, crate::common::XmlHeaderType::None));
        }
        return Err(unexpected_tag(owner, owner, e.name().as_ref()));
      }
      PayloadEvent::Eof => return Err(unexpected_eof(owner)),
      PayloadEvent::End(_) => {}
      PayloadEvent::Text(_) | PayloadEvent::CData(_) | PayloadEvent::GeneralRef(_) => {}
    }
  }
}

#[inline]
pub(crate) fn read_root_start_io<R: std::io::BufRead>(
  reader: &mut IoReader<R>,
  owner: &'static str,
  local_name: &'static [u8],
) -> Result<
  (
    quick_xml::events::BytesStart<'static>,
    bool,
    crate::common::XmlHeaderType,
  ),
  SdkError,
> {
  loop {
    match reader.next_tag_event()? {
      PayloadEvent::Start(e, empty) => {
        if xml_local_name(e.name()) == local_name {
          return Ok((e, empty, crate::common::XmlHeaderType::None));
        }
        return Err(unexpected_tag(owner, owner, e.name().as_ref()));
      }
      PayloadEvent::Eof => return Err(unexpected_eof(owner)),
      PayloadEvent::End(_) => {}
      PayloadEvent::Text(_) | PayloadEvent::CData(_) | PayloadEvent::GeneralRef(_) => {}
    }
  }
}

#[cfg(feature = "parts")]
pub(crate) fn root_element_matches_namespace_local(
  bytes: &[u8],
  namespace_uri: &[u8],
  local_name: &[u8],
) -> Result<bool, SdkError> {
  if let Some(bytes) = decode_utf16_xml_bytes(bytes)? {
    return root_element_matches_namespace_local(&bytes, namespace_uri, local_name);
  }

  let mut reader = Reader::from_reader(bytes);
  loop {
    match reader.read_event()? {
      Event::Start(e) | Event::Empty(e) => {
        let name = e.name();
        let name = name.as_ref();
        let (prefix, root_local_name) = split_qname(name);
        return Ok(
          root_local_name == local_name && root_namespace_matches(&e, prefix, namespace_uri)?,
        );
      }
      Event::Eof => return Ok(false),
      _ => {}
    }
  }
}

#[cfg(feature = "parts")]
fn split_qname(qname: &[u8]) -> (Option<&[u8]>, &[u8]) {
  if let Some(index) = qname.iter().position(|byte| *byte == b':') {
    (Some(&qname[..index]), &qname[index + 1..])
  } else {
    (None, qname)
  }
}

#[cfg(feature = "parts")]
fn root_namespace_matches(
  start: &quick_xml::events::BytesStart<'_>,
  prefix: Option<&[u8]>,
  namespace_uri: &[u8],
) -> Result<bool, SdkError> {
  for attr in start.attributes() {
    let attr = attr?;
    let key = attr.key.as_ref();
    if match prefix {
      Some(prefix) => key.strip_prefix(b"xmlns:") == Some(prefix),
      None => key == b"xmlns",
    } {
      return Ok(attr.value.as_ref() == namespace_uri);
    }
  }
  Ok(false)
}

#[inline]
pub(crate) fn write_escaped_str<W: std::io::Write>(
  writer: &mut W,
  value: &str,
) -> std::io::Result<()> {
  write_escaped_attr_bytes(writer, value.as_bytes())
}

#[inline]
pub(crate) fn write_escaped_content_str<W: std::io::Write>(
  writer: &mut W,
  value: &str,
) -> std::io::Result<()> {
  write_escaped_content_bytes(writer, value.as_bytes())
}

#[inline]
pub(crate) fn write_u8_value<W: std::io::Write>(writer: &mut W, value: u8) -> std::io::Result<()> {
  write_integer_value(writer, value)
}

#[inline]
pub(crate) fn write_i8_value<W: std::io::Write>(writer: &mut W, value: i8) -> std::io::Result<()> {
  write_integer_value(writer, value)
}

#[inline]
pub(crate) fn write_u16_value<W: std::io::Write>(
  writer: &mut W,
  value: u16,
) -> std::io::Result<()> {
  write_integer_value(writer, value)
}

#[inline]
pub(crate) fn write_i16_value<W: std::io::Write>(
  writer: &mut W,
  value: i16,
) -> std::io::Result<()> {
  write_integer_value(writer, value)
}

#[inline]
pub(crate) fn write_u32_value<W: std::io::Write>(
  writer: &mut W,
  value: u32,
) -> std::io::Result<()> {
  write_integer_value(writer, value)
}

#[inline]
pub(crate) fn write_i32_value<W: std::io::Write>(
  writer: &mut W,
  value: i32,
) -> std::io::Result<()> {
  write_integer_value(writer, value)
}

#[inline]
pub(crate) fn write_u64_value<W: std::io::Write>(
  writer: &mut W,
  value: u64,
) -> std::io::Result<()> {
  write_integer_value(writer, value)
}

#[inline]
pub(crate) fn write_i64_value<W: std::io::Write>(
  writer: &mut W,
  value: i64,
) -> std::io::Result<()> {
  write_integer_value(writer, value)
}

#[inline]
fn write_integer_value<W, I>(writer: &mut W, value: I) -> std::io::Result<()>
where
  W: std::io::Write,
  I: itoa::Integer,
{
  let mut buffer = itoa::Buffer::new();
  writer.write_all(buffer.format(value).as_bytes())
}

#[inline]
pub(crate) fn write_f32_value<W: std::io::Write>(
  writer: &mut W,
  value: f32,
) -> std::io::Result<()> {
  write_float_value(writer, value)
}

#[inline]
pub(crate) fn write_f64_value<W: std::io::Write>(
  writer: &mut W,
  value: f64,
) -> std::io::Result<()> {
  write_float_value(writer, value)
}

#[inline]
fn write_float_value<W, F>(writer: &mut W, value: F) -> std::io::Result<()>
where
  W: std::io::Write,
  F: zmij::Float,
{
  let mut buffer = zmij::Buffer::new();
  writer.write_all(buffer.format_finite(value).as_bytes())
}

#[inline]
fn write_escaped_attr_bytes<W: std::io::Write>(
  writer: &mut W,
  bytes: &[u8],
) -> std::io::Result<()> {
  let mut copied = 0usize;
  let mut escaped = false;

  for (index, &byte) in bytes.iter().enumerate() {
    let replacement: Option<&[u8]> = match byte {
      b'<' => Some(b"&lt;"),
      b'&' => Some(b"&amp;"),
      b'"' => Some(b"&quot;"),
      _ => None,
    };

    if let Some(replacement) = replacement {
      escaped = true;
      if copied < index {
        writer.write_all(&bytes[copied..index])?;
      }
      writer.write_all(replacement)?;
      copied = index + 1;
    }
  }

  if !escaped {
    writer.write_all(bytes)
  } else if copied < bytes.len() {
    writer.write_all(&bytes[copied..])
  } else {
    Ok(())
  }
}

#[inline]
fn write_escaped_content_bytes<W: std::io::Write>(
  writer: &mut W,
  bytes: &[u8],
) -> std::io::Result<()> {
  let mut copied = 0usize;
  let mut escaped = false;

  for (index, &byte) in bytes.iter().enumerate() {
    let replacement: Option<&[u8]> = match byte {
      b'<' => Some(b"&lt;"),
      b'&' => Some(b"&amp;"),
      _ => None,
    };

    if let Some(replacement) = replacement {
      escaped = true;
      if copied < index {
        writer.write_all(&bytes[copied..index])?;
      }
      writer.write_all(replacement)?;
      copied = index + 1;
    }
  }

  if !escaped {
    writer.write_all(bytes)
  } else if copied < bytes.len() {
    writer.write_all(&bytes[copied..])
  } else {
    Ok(())
  }
}

#[inline]
pub(crate) fn write_escaped_content_text<W: std::io::Write, T: std::fmt::Display + ?Sized>(
  writer: &mut W,
  value: &T,
) -> std::io::Result<()> {
  write_escaped_display(writer, value)
}

#[inline]
fn write_escaped_display<W: std::io::Write, T: std::fmt::Display + ?Sized>(
  writer: &mut W,
  value: &T,
) -> std::io::Result<()> {
  let mut escaped_writer = EscapedDisplayWriter {
    writer,
    error: None,
  };
  match std::fmt::write(&mut escaped_writer, format_args!("{value}")) {
    Ok(()) => Ok(()),
    Err(_) => Err(
      escaped_writer
        .error
        .unwrap_or_else(|| std::io::Error::other("failed to format escaped XML value")),
    ),
  }
}

struct EscapedDisplayWriter<'a, W: std::io::Write> {
  writer: &'a mut W,
  error: Option<std::io::Error>,
}

impl<W: std::io::Write> std::fmt::Write for EscapedDisplayWriter<'_, W> {
  #[inline]
  fn write_str(&mut self, value: &str) -> std::fmt::Result {
    let result = write_escaped_content_bytes(self.writer, value.as_bytes());
    result.map_err(|err| {
      self.error = Some(err);
      std::fmt::Error
    })
  }
}

#[inline]
pub(crate) fn write_list_value_with<W, T, WriteValue>(
  writer: &mut W,
  values: &[T],
  mut write_value: WriteValue,
) -> std::io::Result<()>
where
  W: std::io::Write,
  WriteValue: FnMut(&mut W, &T) -> std::io::Result<()>,
{
  let mut values = values.iter();
  if let Some(value) = values.next() {
    write_value(writer, value)?;
    for value in values {
      writer.write_all(b" ")?;
      write_value(writer, value)?;
    }
  }
  Ok(())
}

#[inline]
pub(crate) fn write_list_str_value<W, T>(writer: &mut W, values: &[T]) -> std::io::Result<()>
where
  W: std::io::Write,
  T: AsRef<str>,
{
  let mut iter = values.iter();
  if let Some(first) = iter.next() {
    write_escaped_str(writer, first.as_ref())?;
    for value in iter {
      writer.write_all(b" ")?;
      write_escaped_str(writer, value.as_ref())?;
    }
  }
  Ok(())
}

#[inline]
pub(crate) fn write_list_content_str_value<W, T>(
  writer: &mut W,
  values: &[T],
) -> std::io::Result<()>
where
  W: std::io::Write,
  T: AsRef<str>,
{
  let mut iter = values.iter();
  if let Some(first) = iter.next() {
    write_escaped_content_str(writer, first.as_ref())?;
    for value in iter {
      writer.write_all(b" ")?;
      write_escaped_content_str(writer, value.as_ref())?;
    }
  }
  Ok(())
}

#[inline]
pub(crate) fn write_twips_measure_value<W: std::io::Write>(
  writer: &mut W,
  value: &crate::simple_type::TwipsMeasureValue,
) -> std::io::Result<()> {
  match value {
    crate::simple_type::TwipsMeasureValue::Twips(value) => write_u64_value(writer, *value),
    crate::simple_type::TwipsMeasureValue::UniversalMeasure(value) => {
      crate::units::write_universal_measure_lexical(writer, *value)
    }
  }
}

#[inline]
pub(crate) fn write_universal_measure_value<W: std::io::Write>(
  writer: &mut W,
  value: &crate::units::UniversalMeasureValue,
) -> std::io::Result<()> {
  crate::units::write_universal_measure_lexical(writer, *value)
}

#[inline]
pub(crate) fn write_hps_measure_value<W: std::io::Write>(
  writer: &mut W,
  value: &crate::units::HpsMeasureValue,
) -> std::io::Result<()> {
  match value {
    crate::units::HpsMeasureValue::HalfPoints(value) => write_u64_value(writer, *value),
    crate::units::HpsMeasureValue::UniversalMeasure(value) => {
      write_universal_measure_value(writer, value)
    }
  }
}

#[inline]
pub(crate) fn write_signed_hps_measure_value<W: std::io::Write>(
  writer: &mut W,
  value: &crate::units::SignedHpsMeasureValue,
) -> std::io::Result<()> {
  match value {
    crate::units::SignedHpsMeasureValue::HalfPoints(value) => write_i64_value(writer, *value),
    crate::units::SignedHpsMeasureValue::UniversalMeasure(value) => {
      write_universal_measure_value(writer, value)
    }
  }
}

#[inline]
pub(crate) fn write_coordinate_value<W: std::io::Write>(
  writer: &mut W,
  value: &crate::units::CoordinateValue,
) -> std::io::Result<()> {
  match value {
    crate::units::CoordinateValue::Emu(value) => write_i64_value(writer, *value),
    crate::units::CoordinateValue::UniversalMeasure(value) => {
      write_universal_measure_value(writer, value)
    }
  }
}

#[inline]
pub(crate) fn write_coordinate32_value<W: std::io::Write>(
  writer: &mut W,
  value: &crate::units::Coordinate32Value,
) -> std::io::Result<()> {
  match value {
    crate::units::Coordinate32Value::Emu(value) => write_i32_value(writer, *value),
    crate::units::Coordinate32Value::UniversalMeasure(value) => {
      write_universal_measure_value(writer, value)
    }
  }
}

#[inline]
pub(crate) fn write_drawingml_percentage_value<W: std::io::Write>(
  writer: &mut W,
  value: &crate::units::DrawingmlPercentageValue,
) -> std::io::Result<()> {
  match value {
    crate::units::DrawingmlPercentageValue::Decimal(value) => write_i32_value(writer, *value),
    crate::units::DrawingmlPercentageValue::PercentString(value) => {
      crate::units::write_percent_lexical(writer, *value)
    }
  }
}

#[inline]
pub(crate) fn write_text_point_value<W: std::io::Write>(
  writer: &mut W,
  value: &crate::units::TextPointValue,
) -> std::io::Result<()> {
  match value {
    crate::units::TextPointValue::Points100(value) => write_i32_value(writer, *value),
    crate::units::TextPointValue::UniversalMeasure(value) => {
      write_universal_measure_value(writer, value)
    }
  }
}

#[inline]
pub(crate) fn write_text_bullet_size_value<W: std::io::Write>(
  writer: &mut W,
  value: &crate::units::TextBulletSizeValue,
) -> std::io::Result<()> {
  match value {
    crate::units::TextBulletSizeValue::Decimal(value) => write_i32_value(writer, *value),
    crate::units::TextBulletSizeValue::PercentString(value) => {
      crate::units::write_percent_lexical(writer, *value)
    }
  }
}

#[inline]
pub(crate) fn write_signed_twips_measure_value<W: std::io::Write>(
  writer: &mut W,
  value: &crate::simple_type::SignedTwipsMeasureValue,
) -> std::io::Result<()> {
  match value {
    crate::simple_type::SignedTwipsMeasureValue::Twips(value) => write_i64_value(writer, *value),
    crate::simple_type::SignedTwipsMeasureValue::UniversalMeasure(value) => {
      crate::units::write_universal_measure_lexical(writer, *value)
    }
  }
}

#[inline]
pub(crate) fn write_decimal_number_or_percent_value<W: std::io::Write>(
  writer: &mut W,
  value: &crate::simple_type::DecimalNumberOrPercentValue,
) -> std::io::Result<()> {
  match value {
    crate::simple_type::DecimalNumberOrPercentValue::DecimalNumber(value) => {
      write_i64_value(writer, *value)
    }
    crate::simple_type::DecimalNumberOrPercentValue::Percent(value) => {
      crate::units::write_percent_lexical(writer, *value)
    }
  }
}

#[inline]
pub(crate) fn write_measurement_or_percent_value<W: std::io::Write>(
  writer: &mut W,
  value: &crate::simple_type::MeasurementOrPercentValue,
) -> std::io::Result<()> {
  match value {
    crate::simple_type::MeasurementOrPercentValue::DecimalNumberOrPercent(value) => {
      write_decimal_number_or_percent_value(writer, value)
    }
    crate::simple_type::MeasurementOrPercentValue::UniversalMeasure(value) => {
      crate::units::write_universal_measure_lexical(writer, *value)
    }
  }
}

#[inline]
pub(crate) fn write_xmlns_attr<W: std::io::Write>(
  writer: &mut W,
  prefix: Option<&[u8]>,
  uri: &[u8],
) -> std::io::Result<()> {
  writer.write_all(b" xmlns")?;
  if let Some(prefix) = prefix
    && !prefix.is_empty()
  {
    writer.write_all(b":")?;
    writer.write_all(prefix)?;
  }
  writer.write_all(b"=\"")?;
  writer.write_all(uri)?;
  writer.write_all(b"\"")
}

#[cfg(test)]
mod tests {
  use quick_xml::{Reader, events::Event};

  use super::{
    parse_bytes_list_attr, parse_u32_bytes, write_escaped_content_str, write_escaped_content_text,
    write_escaped_str,
  };

  struct DisplayXml;

  impl std::fmt::Display for DisplayXml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_str(r#"<tag attr="one&two">'text'</tag>"#)
    }
  }

  fn parse_u32_list_attr(xml: &str) -> Vec<u32> {
    let mut reader = Reader::from_str(xml);
    let Event::Empty(event) = reader.read_event().expect("empty event") else {
      panic!("expected empty element");
    };
    let attr = event
      .attributes()
      .with_checks(false)
      .next()
      .expect("attribute")
      .expect("valid attribute");
    parse_bytes_list_attr(&attr, reader.decoder(), "Test", "a", parse_u32_bytes)
      .expect("parsed list")
  }

  #[test]
  fn static_list_attr_parses_raw_and_escaped_values() {
    assert_eq!(parse_u32_list_attr("<x a=\"1 2 3\"/>"), vec![1, 2, 3]);
    assert_eq!(parse_u32_list_attr("<x a=\"1&#32;2 3\"/>"), vec![1, 2, 3]);
  }

  #[test]
  fn xml_writer_escapes_attributes_and_content() {
    let mut attr = Vec::new();
    write_escaped_str(&mut attr, r#"<tag attr="one&two">'text'</tag>"#).expect("write attr");
    assert_eq!(
      String::from_utf8(attr).expect("utf-8 attr"),
      "&lt;tag attr=&quot;one&amp;two&quot;>'text'&lt;/tag>"
    );

    let mut content = Vec::new();
    write_escaped_content_str(&mut content, r#"<tag attr="one&two">'text'</tag>"#)
      .expect("write content");
    assert_eq!(
      String::from_utf8(content).expect("utf-8 content"),
      r#"&lt;tag attr="one&amp;two">'text'&lt;/tag>"#
    );

    let mut display_content = Vec::new();
    write_escaped_content_text(&mut display_content, &DisplayXml).expect("write display content");
    assert_eq!(
      String::from_utf8(display_content).expect("utf-8 display content"),
      r#"&lt;tag attr="one&amp;two">'text'&lt;/tag>"#
    );
  }
}
