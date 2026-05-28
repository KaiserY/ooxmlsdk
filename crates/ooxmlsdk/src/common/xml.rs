use quick_xml::{
  Decoder, Reader, Writer,
  escape::unescape,
  events::{BytesRef, BytesText, Event, attributes::Attribute},
};
use std::io::BufRead;
use std::io::Cursor;

use super::{SdkError, invalid_field_value, unexpected_eof, unexpected_tag};

pub struct IoReader<R: BufRead> {
  reader: Reader<R>,
  buf: Vec<u8>,
  current: Option<Event<'static>>,
  pending: Option<Event<'static>>,
}

pub(crate) enum IoTagEvent {
  Start(quick_xml::events::BytesStart<'static>, bool),
  End(quick_xml::events::BytesEnd<'static>),
  Decl(bool),
  Eof,
  Other,
}

pub enum SliceTagEvent<'de> {
  Start(quick_xml::events::BytesStart<'de>, bool),
  End(quick_xml::events::BytesEnd<'de>),
  Decl(bool),
  Eof,
  Other,
}

impl<R: BufRead> IoReader<R> {
  pub fn new(reader: Reader<R>) -> Self {
    Self {
      reader,
      buf: Vec::new(),
      current: None,
      pending: None,
    }
  }

  #[inline]
  pub fn next(&mut self) -> Result<Event<'static>, SdkError> {
    self.current = None;
    if let Some(event) = self.pending.take() {
      return Ok(event);
    }

    self.buf.clear();
    Ok(self.reader.read_event_into(&mut self.buf)?.into_owned())
  }

  #[inline]
  pub fn next_borrowed(&mut self) -> Result<Event<'_>, SdkError> {
    self.current = None;
    if let Some(event) = self.pending.take() {
      self.current = Some(event);
      return Ok(self.current.as_ref().expect("current event").borrow());
    }

    self.buf.clear();
    Ok(self.reader.read_event_into(&mut self.buf)?)
  }

  #[inline]
  pub(crate) fn next_tag_event(&mut self) -> Result<IoTagEvent, SdkError> {
    self.current = None;
    if let Some(event) = self.pending.take() {
      return Ok(Self::tag_event_from_owned(event));
    }

    self.buf.clear();
    Ok(match self.reader.read_event_into(&mut self.buf)? {
      Event::Start(e) => IoTagEvent::Start(e.into_owned(), false),
      Event::Empty(e) => IoTagEvent::Start(e.into_owned(), true),
      Event::End(e) => IoTagEvent::End(e.into_owned()),
      Event::Decl(e) => IoTagEvent::Decl(matches!(
        e.standalone(),
        Some(Ok(value)) if value.as_ref().eq_ignore_ascii_case(b"yes")
      )),
      Event::Eof => IoTagEvent::Eof,
      _ => IoTagEvent::Other,
    })
  }

  #[inline]
  pub fn unread(&mut self, event: Event<'static>) -> Result<(), SdkError> {
    self.current = None;
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

  #[inline]
  fn tag_event_from_owned(event: Event<'static>) -> IoTagEvent {
    match event {
      Event::Start(e) => IoTagEvent::Start(e, false),
      Event::Empty(e) => IoTagEvent::Start(e, true),
      Event::End(e) => IoTagEvent::End(e),
      Event::Decl(e) => IoTagEvent::Decl(matches!(
        e.standalone(),
        Some(Ok(value)) if value.as_ref().eq_ignore_ascii_case(b"yes")
      )),
      Event::Eof => IoTagEvent::Eof,
      _ => IoTagEvent::Other,
    }
  }
}

pub struct SliceReader<'de> {
  reader: Reader<&'de [u8]>,
  pending: Option<Event<'de>>,
}

impl<'de> SliceReader<'de> {
  pub fn new(reader: Reader<&'de [u8]>) -> Self {
    Self {
      reader,
      pending: None,
    }
  }

  #[inline]
  pub fn next(&mut self) -> Result<Event<'de>, SdkError> {
    if let Some(event) = self.pending.take() {
      return Ok(event);
    }

    Ok(self.reader.read_event()?)
  }

  #[inline]
  pub fn next_tag_event(&mut self) -> Result<SliceTagEvent<'de>, SdkError> {
    if let Some(event) = self.pending.take() {
      return Ok(Self::tag_event_from_event(event));
    }

    Ok(Self::tag_event_from_event(self.reader.read_event()?))
  }

  #[inline]
  pub fn unread(&mut self, event: Event<'de>) -> Result<(), SdkError> {
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

  #[inline]
  fn tag_event_from_event(event: Event<'de>) -> SliceTagEvent<'de> {
    match event {
      Event::Start(e) => SliceTagEvent::Start(e, false),
      Event::Empty(e) => SliceTagEvent::Start(e, true),
      Event::End(e) => SliceTagEvent::End(e),
      Event::Decl(e) => SliceTagEvent::Decl(matches!(
        e.standalone(),
        Some(Ok(value)) if value.as_ref().eq_ignore_ascii_case(b"yes")
      )),
      Event::Eof => SliceTagEvent::Eof,
      _ => SliceTagEvent::Other,
    }
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

#[inline]
pub(crate) fn from_str_inner(s: &str) -> Result<SliceReader<'_>, SdkError> {
  let mut xml_reader = Reader::from_str(s);
  xml_reader.config_mut().check_end_names = false;
  Ok(SliceReader::new(xml_reader))
}

#[inline(always)]
pub(crate) fn attr_raw_value<'a>(attr: &'a Attribute<'a>) -> Option<&'a [u8]> {
  let value = attr.value.as_ref();
  if value.contains(&b'&') {
    None
  } else {
    Some(value)
  }
}

#[inline]
pub(crate) fn decode_attr_value(
  attr: &Attribute<'_>,
  decoder: Decoder,
) -> Result<String, SdkError> {
  if let Some(value) = attr_raw_value(attr) {
    Ok(decoder.decode(value)?.into_owned())
  } else {
    Ok(unescape(&decoder.decode(attr.value.as_ref())?)?.into_owned())
  }
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
      if let Some(value) = attr_raw_value(attr) {
        $bytes_name(value, ty, field)
      } else {
        let value = decode_attr_value(attr, decoder)?;
        $bytes_name(value.as_bytes(), ty, field)
      }
    }

    #[inline]
    fn $bytes_name(value: &[u8], ty: &'static str, field: &'static str) -> Result<$ty, SdkError> {
      let digits = match value {
        [b'+', rest @ ..] => rest,
        _ => value,
      };
      if digits.is_empty() {
        return Err(invalid_field_value_bytes(ty, field, value));
      }

      let mut parsed: $ty = 0;
      for &digit in digits {
        if !digit.is_ascii_digit() {
          return Err(invalid_field_value_bytes(ty, field, value));
        }

        parsed = parsed
          .checked_mul(10)
          .and_then(|current| current.checked_add((digit - b'0') as $ty))
          .ok_or_else(|| invalid_field_value_bytes(ty, field, value))?;
      }

      Ok(parsed)
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
      if let Some(value) = attr_raw_value(attr) {
        $bytes_name(value, ty, field)
      } else {
        let value = decode_attr_value(attr, decoder)?;
        $bytes_name(value.as_bytes(), ty, field)
      }
    }

    #[inline]
    fn $bytes_name(value: &[u8], ty: &'static str, field: &'static str) -> Result<$ty, SdkError> {
      let (negative, digits) = match value {
        [b'-', rest @ ..] => (true, rest),
        [b'+', rest @ ..] => (false, rest),
        _ => (false, value),
      };
      if digits.is_empty() {
        return Err(invalid_field_value_bytes(ty, field, value));
      }

      let mut parsed: $ty = 0;
      for &digit in digits {
        if !digit.is_ascii_digit() {
          return Err(invalid_field_value_bytes(ty, field, value));
        }

        parsed = parsed
          .checked_mul(10)
          .and_then(|current| {
            if negative {
              current.checked_sub((digit - b'0') as $ty)
            } else {
              current.checked_add((digit - b'0') as $ty)
            }
          })
          .ok_or_else(|| invalid_field_value_bytes(ty, field, value))?;
      }

      Ok(parsed)
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
pub(crate) fn parse_twips_measure_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
) -> Result<crate::simple_type::TwipsMeasureValue, SdkError> {
  if let Some(value) = attr_raw_value(attr)
    && let Ok(value) = crate::simple_type::TwipsMeasureValue::from_bytes(value)
  {
    return Ok(value);
  }
  let value = decode_attr_value(attr, decoder)?;
  parse_twips_measure_value(value)
}

#[inline]
pub(crate) fn parse_twips_measure_value(
  value: String,
) -> Result<crate::simple_type::TwipsMeasureValue, SdkError> {
  crate::simple_type::TwipsMeasureValue::from_bytes(value.as_bytes())
    .map_err(|_| invalid_field_value("TwipsMeasureValue", "value", value))
}

#[inline]
pub(crate) fn parse_signed_twips_measure_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
) -> Result<crate::simple_type::SignedTwipsMeasureValue, SdkError> {
  if let Some(value) = attr_raw_value(attr)
    && let Ok(value) = crate::simple_type::SignedTwipsMeasureValue::from_bytes(value)
  {
    return Ok(value);
  }
  let value = decode_attr_value(attr, decoder)?;
  parse_signed_twips_measure_value(value)
}

#[inline]
pub(crate) fn parse_signed_twips_measure_value(
  value: String,
) -> Result<crate::simple_type::SignedTwipsMeasureValue, SdkError> {
  crate::simple_type::SignedTwipsMeasureValue::from_bytes(value.as_bytes())
    .map_err(|_| invalid_field_value("SignedTwipsMeasureValue", "value", value))
}

#[inline]
pub(crate) fn parse_decimal_number_or_percent_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
) -> Result<crate::simple_type::DecimalNumberOrPercentValue, SdkError> {
  if let Some(value) = attr_raw_value(attr)
    && let Ok(value) = crate::simple_type::DecimalNumberOrPercentValue::from_bytes(value)
  {
    return Ok(value);
  }
  let value = decode_attr_value(attr, decoder)?;
  parse_decimal_number_or_percent_value(value)
}

#[inline]
pub(crate) fn parse_decimal_number_or_percent_value(
  value: String,
) -> Result<crate::simple_type::DecimalNumberOrPercentValue, SdkError> {
  crate::simple_type::DecimalNumberOrPercentValue::from_bytes(value.as_bytes())
    .map_err(|_| invalid_field_value("DecimalNumberOrPercentValue", "value", value))
}

#[inline]
pub(crate) fn parse_measurement_or_percent_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
) -> Result<crate::simple_type::MeasurementOrPercentValue, SdkError> {
  if let Some(value) = attr_raw_value(attr)
    && let Ok(value) = crate::simple_type::MeasurementOrPercentValue::from_bytes(value)
  {
    return Ok(value);
  }
  let value = decode_attr_value(attr, decoder)?;
  parse_measurement_or_percent_value(value)
}

#[inline]
pub(crate) fn parse_measurement_or_percent_value(
  value: String,
) -> Result<crate::simple_type::MeasurementOrPercentValue, SdkError> {
  crate::simple_type::MeasurementOrPercentValue::from_bytes(value.as_bytes())
    .map_err(|_| invalid_field_value("MeasurementOrPercentValue", "value", value))
}

#[inline]
pub(crate) fn parse_i32_zero_on_overflow_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<i32, SdkError> {
  if let Some(value) = attr_raw_value(attr) {
    parse_i32_zero_on_overflow_bytes(value, ty, field)
  } else {
    let value = decode_attr_value(attr, decoder)?;
    parse_i32_zero_on_overflow_bytes(value.as_bytes(), ty, field)
  }
}

#[inline]
fn parse_i32_zero_on_overflow_bytes(
  value: &[u8],
  ty: &'static str,
  field: &'static str,
) -> Result<i32, SdkError> {
  let (negative, digits) = match value {
    [b'-', rest @ ..] => (true, rest),
    [b'+', rest @ ..] => (false, rest),
    _ => (false, value),
  };
  if digits.is_empty() {
    return Err(invalid_field_value_bytes(ty, field, value));
  }

  let limit = if negative {
    i32::MAX as u32 + 1
  } else {
    i32::MAX as u32
  };
  let mut parsed: u32 = 0;
  for &digit in digits {
    if !digit.is_ascii_digit() {
      return Err(invalid_field_value_bytes(ty, field, value));
    }

    parsed = match parsed
      .checked_mul(10)
      .and_then(|current| current.checked_add((digit - b'0') as u32))
    {
      Some(parsed) if parsed <= limit => parsed,
      _ => return Ok(0),
    };
  }

  if negative {
    if parsed == i32::MAX as u32 + 1 {
      Ok(i32::MIN)
    } else {
      Ok(-(parsed as i32))
    }
  } else {
    Ok(parsed as i32)
  }
}

#[inline]
pub(crate) fn parse_attr_value<T>(
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
    Ok(value)
  } else {
    let value = decode_attr_value(attr, decoder)?;
    parse_value(value.as_ref(), ty, field)
  }
}

#[inline]
pub(crate) fn parse_enum_attr<T>(
  attr: &Attribute<'_>,
  decoder: Decoder,
  _ty: &'static str,
) -> Result<T, SdkError>
where
  T: crate::sdk::SdkEnum,
{
  if let Some(value) = attr_raw_value(attr) {
    return T::from_xml_bytes(value);
  }

  let value = decode_attr_value(attr, decoder)?;
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
  value
    .parse::<T>()
    .map_err(|_| invalid_field_value(ty, field, value))
}

#[inline]
pub(crate) fn parse_list_attr<T>(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<Vec<T>, SdkError>
where
  T: std::str::FromStr,
{
  let value = decode_attr_value(attr, decoder)?;
  parse_list_value(value.as_ref(), ty, field)
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

#[inline]
pub(crate) fn push_xml_text(
  value: &mut Option<String>,
  text: BytesText<'_>,
) -> Result<(), SdkError> {
  let text = text.xml10_content()?;
  if let Some(value) = value {
    value.push_str(text.as_ref());
  } else {
    *value = Some(text.into_owned());
  }

  Ok(())
}

#[inline]
pub(crate) fn push_xml_general_ref(
  value: &mut Option<String>,
  text: BytesRef<'_>,
  ty: &'static str,
  field: &'static str,
) -> Result<(), SdkError> {
  let entity = text.xml10_content()?;
  let entity = resolve_general_ref_entity(entity.as_ref())
    .ok_or_else(|| invalid_field_value(ty, field, entity.to_string()))?;

  if let Some(value) = value {
    value.push_str(entity.as_ref());
  } else {
    *value = Some(entity.into_owned());
  }

  Ok(())
}

#[inline]
fn resolve_general_ref_entity(entity: &str) -> Option<std::borrow::Cow<'_, str>> {
  if let Some(entity) = quick_xml::escape::resolve_predefined_entity(entity) {
    return Some(std::borrow::Cow::Borrowed(entity));
  }

  if let Some(hex) = entity
    .strip_prefix("#x")
    .or_else(|| entity.strip_prefix("#X"))
  {
    let code_point = u32::from_str_radix(hex, 16).ok()?;
    let ch = char::from_u32(code_point)?;
    return Some(std::borrow::Cow::Owned(ch.to_string()));
  }

  if let Some(decimal) = entity.strip_prefix('#') {
    let code_point = decimal.parse::<u32>().ok()?;
    let ch = char::from_u32(code_point)?;
    return Some(std::borrow::Cow::Owned(ch.to_string()));
  }

  None
}

#[cfg(feature = "flat-opc")]
pub(crate) fn read_outer_xml_borrowed<'de>(
  xml_reader: &mut SliceReader<'de>,
  start: quick_xml::events::BytesStart<'de>,
  empty_tag: bool,
) -> Result<String, SdkError> {
  let mut writer = Writer::new(Cursor::new(Vec::new()));

  if empty_tag {
    writer.write_event(Event::Empty(start))?;
  } else {
    writer.write_event(Event::Start(start))?;

    let mut depth = 1usize;
    loop {
      let event = xml_reader.next()?;
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
pub(crate) fn read_raw_empty_xml_borrowed<'de>(
  start: quick_xml::events::BytesStart<'de>,
) -> Result<String, SdkError> {
  let start: &[u8] = start.as_ref();
  let mut xml = Vec::with_capacity(start.len() + RAW_EMPTY_XML_EXTRA_LEN);
  xml.push(b'<');
  xml.extend_from_slice(start);
  xml.extend_from_slice(b"/>");

  String::from_utf8(xml)
    .map_err(|err| SdkError::CommonError(format!("invalid utf-8 xml fragment: {err}")))
}

#[inline]
pub(crate) fn read_raw_element_xml_borrowed<'de>(
  xml_reader: &mut SliceReader<'de>,
  start: quick_xml::events::BytesStart<'de>,
) -> Result<String, SdkError> {
  let start_bytes: &[u8] = start.as_ref();
  let end_name = start.name();
  let inner = xml_reader.reader.read_text(end_name)?;
  let inner: &[u8] = inner.as_ref();
  let end_name = end_name.as_ref();

  let mut xml = Vec::with_capacity(
    start_bytes.len() + inner.len() + end_name.len() + RAW_ELEMENT_XML_EXTRA_LEN,
  );
  xml.push(b'<');
  xml.extend_from_slice(start_bytes);
  xml.push(b'>');
  xml.extend_from_slice(inner);
  xml.extend_from_slice(b"</");
  xml.extend_from_slice(end_name);
  xml.push(b'>');

  String::from_utf8(xml)
    .map_err(|err| SdkError::CommonError(format!("invalid utf-8 xml fragment: {err}")))
}

#[cfg(feature = "flat-opc")]
pub(crate) fn read_outer_xml_io<R: BufRead>(
  xml_reader: &mut IoReader<R>,
  start: quick_xml::events::BytesStart<'static>,
  empty_tag: bool,
) -> Result<String, SdkError> {
  let mut writer = Writer::new(Cursor::new(Vec::new()));

  if empty_tag {
    writer.write_event(Event::Empty(start))?;
  } else {
    writer.write_event(Event::Start(start))?;

    let mut depth = 1usize;
    loop {
      let event = xml_reader.next()?;
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
pub(crate) fn read_raw_empty_xml_io(
  start: quick_xml::events::BytesStart<'static>,
) -> Result<String, SdkError> {
  let start: &[u8] = start.as_ref();
  let mut xml = Vec::with_capacity(start.len() + RAW_EMPTY_XML_EXTRA_LEN);
  xml.push(b'<');
  xml.extend_from_slice(start);
  xml.extend_from_slice(b"/>");

  String::from_utf8(xml)
    .map_err(|err| SdkError::CommonError(format!("invalid utf-8 xml fragment: {err}")))
}

#[inline]
pub(crate) fn read_raw_element_xml_io<R: BufRead>(
  xml_reader: &mut IoReader<R>,
  start: quick_xml::events::BytesStart<'static>,
) -> Result<String, SdkError> {
  let start_bytes: &[u8] = start.as_ref();
  let end_name = start.name();
  let inner = xml_reader
    .reader
    .read_text_into(end_name, &mut xml_reader.buf)?;
  let inner: &[u8] = inner.as_ref();
  let end_name = end_name.as_ref();

  let mut xml = Vec::with_capacity(
    start_bytes.len() + inner.len() + end_name.len() + RAW_ELEMENT_XML_EXTRA_LEN,
  );
  xml.push(b'<');
  xml.extend_from_slice(start_bytes);
  xml.push(b'>');
  xml.extend_from_slice(inner);
  xml.extend_from_slice(b"</");
  xml.extend_from_slice(end_name);
  xml.push(b'>');

  String::from_utf8(xml)
    .map_err(|err| SdkError::CommonError(format!("invalid utf-8 xml fragment: {err}")))
}

#[cfg(feature = "mce")]
const MC_ALTERNATE_CONTENT_NAMES: &[&[u8]] = &[b"mc:AlternateContent", b"AlternateContent"];
#[cfg(feature = "mce")]
const MC_CHOICE_NAMES: &[&[u8]] = &[b"mc:Choice", b"Choice"];
#[cfg(feature = "mce")]
const MC_FALLBACK_NAMES: &[&[u8]] = &[b"mc:Fallback", b"Fallback"];

#[cfg(feature = "mce")]
pub(crate) fn mce_choice_replacement_children(
  xml: &str,
  settings: &crate::sdk::MarkupCompatibilityProcessSettings,
  context: &crate::sdk::MceContext,
) -> Result<Option<Vec<String>>, SdkError> {
  let mut reader = Reader::from_str(xml);
  reader.config_mut().check_end_names = false;
  let mut fallback = None;

  loop {
    match reader.read_event()? {
      Event::Start(e) if qname_in(e.name().as_ref(), MC_ALTERNATE_CONTENT_NAMES) => {
        let namespaces = namespaces_from_context_with(context, &e)?;
        loop {
          match reader.read_event()? {
            Event::Start(e) if qname_in(e.name().as_ref(), MC_CHOICE_NAMES) => {
              let choice_namespaces = namespaces_with(&namespaces, &e)?;
              let requires = attr_value(&reader, &e, b"Requires")?;
              let children = read_mce_container_children(&mut reader, MC_CHOICE_NAMES)?;
              if choice_requires_supported(
                requires.as_deref(),
                &choice_namespaces,
                settings.target_file_format_version,
              )? {
                return Ok(Some(children));
              }
            }
            Event::Empty(e) if qname_in(e.name().as_ref(), MC_CHOICE_NAMES) => {
              let choice_namespaces = namespaces_with(&namespaces, &e)?;
              let requires = attr_value(&reader, &e, b"Requires")?;
              if choice_requires_supported(
                requires.as_deref(),
                &choice_namespaces,
                settings.target_file_format_version,
              )? {
                return Ok(Some(Vec::new()));
              }
            }
            Event::Start(e) if qname_in(e.name().as_ref(), MC_FALLBACK_NAMES) => {
              fallback = Some(read_mce_container_children(&mut reader, MC_FALLBACK_NAMES)?);
            }
            Event::Empty(e) if qname_in(e.name().as_ref(), MC_FALLBACK_NAMES) => {
              fallback = Some(Vec::new());
            }
            Event::Start(_) => {
              skip_element(&mut reader)?;
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
      Event::Start(e) => return mce_unknown_element_replacement(&mut reader, e, context, false),
      Event::Empty(e) => return mce_unknown_element_replacement(&mut reader, e, context, true),
      Event::Eof => return Ok(None),
      _ => {}
    }
  }
}

#[cfg(feature = "mce")]
fn mce_unknown_element_replacement(
  reader: &mut Reader<&[u8]>,
  start: quick_xml::events::BytesStart<'_>,
  context: &crate::sdk::MceContext,
  empty_tag: bool,
) -> Result<Option<Vec<String>>, SdkError> {
  let qname = String::from_utf8_lossy(start.name().as_ref()).into_owned();
  let namespaces = namespaces_from_context_with(context, &start)?;
  let ignorable_namespace = qname.split_once(':').and_then(|(prefix, _)| {
    namespaces
      .iter()
      .rev()
      .find_map(|(candidate, ns)| (candidate.as_ref() == prefix).then_some(ns.as_ref()))
      .filter(|ns| context.is_ignorable_namespace(ns))
  });

  if ignorable_namespace.is_some() && context.is_process_content_qname(qname.as_str()) {
    if empty_tag {
      return Ok(Some(Vec::new()));
    }
    let end_name = start.name().as_ref().to_vec();
    return read_mce_container_children(reader, &[end_name.as_slice()]).map(Some);
  }

  if ignorable_namespace.is_some() {
    if !empty_tag {
      skip_element(reader)?;
    }
    return Ok(Some(Vec::new()));
  }

  if !qname.contains(':') {
    if !empty_tag {
      skip_element(reader)?;
    }
    return Ok(None);
  }

  if !empty_tag {
    skip_element(reader)?;
  }
  Ok(None)
}

#[cfg(feature = "mce")]
fn read_mce_container_children(
  reader: &mut Reader<&[u8]>,
  end_names: &[&[u8]],
) -> Result<Vec<String>, SdkError> {
  let mut children = Vec::new();
  loop {
    match reader.read_event()? {
      Event::Start(e) => {
        children.push(read_outer_xml_from_str_reader(reader, e, false)?);
      }
      Event::Empty(e) => {
        children.push(read_outer_xml_from_str_reader(reader, e, true)?);
      }
      Event::End(e) if qname_in(e.name().as_ref(), end_names) => return Ok(children),
      Event::Eof => return Err(unexpected_eof("mce choice/fallback")),
      _ => {}
    }
  }
}

#[cfg(feature = "mce")]
fn read_outer_xml_from_str_reader(
  reader: &mut Reader<&[u8]>,
  start: quick_xml::events::BytesStart<'_>,
  empty_tag: bool,
) -> Result<String, SdkError> {
  let mut writer = Writer::new(Cursor::new(Vec::new()));
  if empty_tag {
    writer.write_event(Event::Empty(start))?;
    return String::from_utf8(writer.into_inner().into_inner())
      .map_err(|err| SdkError::CommonError(format!("invalid utf-8 xml fragment: {err}")));
  }

  writer.write_event(Event::Start(start))?;
  let mut depth = 1usize;
  loop {
    let event = reader.read_event()?;
    match &event {
      Event::Start(_) => depth += 1,
      Event::End(_) => depth -= 1,
      Event::Eof => return Err(unexpected_eof("mce selected child")),
      _ => {}
    }
    writer.write_event(event)?;
    if depth == 0 {
      break;
    }
  }

  String::from_utf8(writer.into_inner().into_inner())
    .map_err(|err| SdkError::CommonError(format!("invalid utf-8 xml fragment: {err}")))
}

#[cfg(feature = "mce")]
fn skip_element(reader: &mut Reader<&[u8]>) -> Result<(), SdkError> {
  let mut depth = 1usize;
  loop {
    match reader.read_event()? {
      Event::Start(_) => depth += 1,
      Event::End(_) => {
        depth -= 1;
        if depth == 0 {
          return Ok(());
        }
      }
      Event::Eof => return Err(unexpected_eof("mce skipped element")),
      _ => {}
    }
  }
}

#[cfg(feature = "mce")]
fn attr_value(
  reader: &Reader<&[u8]>,
  start: &quick_xml::events::BytesStart<'_>,
  name: &[u8],
) -> Result<Option<String>, SdkError> {
  for attr in start.attributes() {
    let attr = attr?;
    if attr.key.as_ref() == name {
      return Ok(Some(decode_attr_value(&attr, reader.decoder())?));
    }
  }
  Ok(None)
}

#[cfg(feature = "mce")]
fn choice_requires_supported(
  requires: Option<&str>,
  namespaces: &[crate::sdk::MceNamespace],
  target: crate::sdk::FileFormatVersion,
) -> Result<bool, SdkError> {
  let Some(requires) = requires else {
    return Ok(false);
  };
  for prefix in requires.split_whitespace() {
    let Some((_, ns)) = namespaces
      .iter()
      .rev()
      .find(|(candidate, _)| candidate.as_ref() == prefix)
    else {
      return Ok(false);
    };
    if !namespace_supported(ns, target) {
      return Ok(false);
    }
  }
  Ok(true)
}

#[cfg(feature = "mce")]
fn namespaces_with(
  namespaces: &[crate::sdk::MceNamespace],
  start: &quick_xml::events::BytesStart<'_>,
) -> Result<Vec<crate::sdk::MceNamespace>, SdkError> {
  let mut merged = namespaces.to_vec();
  merged.extend(namespace_decls(start)?);
  Ok(merged)
}

#[cfg(feature = "mce")]
fn namespaces_from_context_with(
  context: &crate::sdk::MceContext,
  start: &quick_xml::events::BytesStart<'_>,
) -> Result<Vec<crate::sdk::MceNamespace>, SdkError> {
  let mut namespaces = context.namespaces().to_vec();
  namespaces.extend(namespace_decls(start)?);
  Ok(namespaces)
}

#[cfg(feature = "mce")]
fn namespace_decls(
  start: &quick_xml::events::BytesStart<'_>,
) -> Result<Vec<crate::sdk::MceNamespace>, SdkError> {
  let mut namespaces = Vec::new();
  for attr in start.attributes() {
    let attr = attr?;
    let key = attr.key.as_ref();
    if let Some(prefix) = key.strip_prefix(b"xmlns:") {
      namespaces.push((
        String::from_utf8_lossy(prefix)
          .into_owned()
          .into_boxed_str(),
        String::from_utf8_lossy(attr.value.as_ref())
          .into_owned()
          .into_boxed_str(),
      ));
    }
  }
  Ok(namespaces)
}

#[cfg(feature = "mce")]
fn namespace_supported(ns: &str, target: crate::sdk::FileFormatVersion) -> bool {
  crate::sdk::namespace_supported(ns, target)
}

#[cfg(feature = "mce")]
fn qname_in(name: &[u8], expected: &[&[u8]]) -> bool {
  expected.contains(&name)
}

#[inline]
pub(crate) fn read_root_start_borrowed<'de>(
  reader: &mut SliceReader<'de>,
  owner: &'static str,
  tag_qname: &'static [u8],
  local_name: &'static [u8],
) -> Result<
  (
    quick_xml::events::BytesStart<'de>,
    bool,
    crate::common::XmlHeaderType,
  ),
  SdkError,
> {
  let mut xml_header = crate::common::XmlHeaderType::None;
  loop {
    match reader.next_tag_event()? {
      SliceTagEvent::Decl(standalone) => {
        xml_header = if standalone {
          crate::common::XmlHeaderType::Standalone
        } else {
          crate::common::XmlHeaderType::Plain
        };
      }
      SliceTagEvent::Start(e, empty) => {
        if e.name().as_ref() == tag_qname || e.name().as_ref() == local_name {
          return Ok((e, empty, xml_header));
        }
        return Err(unexpected_tag(owner, owner, e.name().as_ref()));
      }
      SliceTagEvent::Eof => return Err(unexpected_eof(owner)),
      SliceTagEvent::End(_) | SliceTagEvent::Other => {}
    }
  }
}

#[inline]
pub(crate) fn read_root_start_io<R: std::io::BufRead>(
  reader: &mut IoReader<R>,
  owner: &'static str,
  tag_qname: &'static [u8],
  local_name: &'static [u8],
) -> Result<
  (
    quick_xml::events::BytesStart<'static>,
    bool,
    crate::common::XmlHeaderType,
  ),
  SdkError,
> {
  let mut xml_header = crate::common::XmlHeaderType::None;
  loop {
    match reader.next_tag_event()? {
      IoTagEvent::Decl(standalone) => {
        xml_header = if standalone {
          crate::common::XmlHeaderType::Standalone
        } else {
          crate::common::XmlHeaderType::Plain
        };
      }
      IoTagEvent::Start(e, empty) => {
        if e.name().as_ref() == tag_qname || e.name().as_ref() == local_name {
          return Ok((e, empty, xml_header));
        }
        return Err(unexpected_tag(owner, owner, e.name().as_ref()));
      }
      IoTagEvent::Eof => return Err(unexpected_eof(owner)),
      IoTagEvent::End(_) | IoTagEvent::Other => {}
    }
  }
}

#[inline]
pub(crate) fn read_root_start_borrowed_no_header<'de>(
  reader: &mut SliceReader<'de>,
  owner: &'static str,
  tag_qname: &'static [u8],
  local_name: &'static [u8],
) -> Result<(quick_xml::events::BytesStart<'de>, bool), SdkError> {
  loop {
    match reader.next_tag_event()? {
      SliceTagEvent::Start(e, empty) => {
        if e.name().as_ref() == tag_qname || e.name().as_ref() == local_name {
          return Ok((e, empty));
        }
        return Err(unexpected_tag(owner, owner, e.name().as_ref()));
      }
      SliceTagEvent::Eof => return Err(unexpected_eof(owner)),
      SliceTagEvent::Decl(_) | SliceTagEvent::End(_) | SliceTagEvent::Other => {}
    }
  }
}

#[inline]
pub(crate) fn read_root_start_io_no_header<R: std::io::BufRead>(
  reader: &mut IoReader<R>,
  owner: &'static str,
  tag_qname: &'static [u8],
  local_name: &'static [u8],
) -> Result<(quick_xml::events::BytesStart<'static>, bool), SdkError> {
  loop {
    match reader.next_tag_event()? {
      IoTagEvent::Start(e, empty) => {
        if e.name().as_ref() == tag_qname || e.name().as_ref() == local_name {
          return Ok((e, empty));
        }
        return Err(unexpected_tag(owner, owner, e.name().as_ref()));
      }
      IoTagEvent::Eof => return Err(unexpected_eof(owner)),
      IoTagEvent::Decl(_) | IoTagEvent::End(_) | IoTagEvent::Other => {}
    }
  }
}

#[inline(always)]
pub(crate) fn write_start_tag_open<W: std::io::Write>(
  writer: &mut W,
  default_namespace_prefix: &str,
  tag_prefix: &str,
  local_name: &str,
) -> std::io::Result<()> {
  writer.write_all(b"<")?;
  write_qualified_name(writer, default_namespace_prefix, tag_prefix, local_name)
}

#[inline(always)]
pub(crate) fn write_start_tag_open_bytes<W: std::io::Write>(
  writer: &mut W,
  default_namespace_prefix: &str,
  tag_prefix: &[u8],
  local_name: &[u8],
) -> std::io::Result<()> {
  writer.write_all(b"<")?;
  write_qualified_name_bytes(
    writer,
    default_namespace_prefix.as_bytes(),
    tag_prefix,
    local_name,
  )
}

#[inline(always)]
pub(crate) fn write_end_tag<W: std::io::Write>(
  writer: &mut W,
  default_namespace_prefix: &str,
  tag_prefix: &str,
  local_name: &str,
) -> std::io::Result<()> {
  writer.write_all(b"</")?;
  write_qualified_name(writer, default_namespace_prefix, tag_prefix, local_name)?;
  writer.write_all(b">")
}

#[inline(always)]
pub(crate) fn write_end_tag_bytes<W: std::io::Write>(
  writer: &mut W,
  default_namespace_prefix: &str,
  tag_prefix: &[u8],
  local_name: &[u8],
) -> std::io::Result<()> {
  writer.write_all(b"</")?;
  write_qualified_name_bytes(
    writer,
    default_namespace_prefix.as_bytes(),
    tag_prefix,
    local_name,
  )?;
  writer.write_all(b">")
}

#[inline(always)]
fn write_qualified_name<W: std::io::Write>(
  writer: &mut W,
  default_namespace_prefix: &str,
  tag_prefix: &str,
  local_name: &str,
) -> std::io::Result<()> {
  if !tag_prefix.is_empty() && default_namespace_prefix != tag_prefix {
    writer.write_all(tag_prefix.as_bytes())?;
    writer.write_all(b":")?;
  }

  writer.write_all(local_name.as_bytes())
}

#[inline(always)]
fn write_qualified_name_bytes<W: std::io::Write>(
  writer: &mut W,
  default_namespace_prefix: &[u8],
  tag_prefix: &[u8],
  local_name: &[u8],
) -> std::io::Result<()> {
  if !tag_prefix.is_empty() && default_namespace_prefix != tag_prefix {
    writer.write_all(tag_prefix)?;
    writer.write_all(b":")?;
  }

  writer.write_all(local_name)
}

#[inline]
pub(crate) fn write_escaped_str<W: std::io::Write>(
  writer: &mut W,
  value: &str,
) -> std::io::Result<()> {
  match quick_xml::escape::escape(value) {
    std::borrow::Cow::Borrowed(value) => writer.write_all(value.as_bytes()),
    std::borrow::Cow::Owned(value) => writer.write_all(value.as_bytes()),
  }
}

#[inline]
pub(crate) fn write_escaped_content_str<W: std::io::Write>(
  writer: &mut W,
  value: &str,
) -> std::io::Result<()> {
  match quick_xml::escape::minimal_escape(value) {
    std::borrow::Cow::Borrowed(value) => writer.write_all(value.as_bytes()),
    std::borrow::Cow::Owned(value) => writer.write_all(value.as_bytes()),
  }
}

#[inline]
pub(crate) fn write_escaped_content_text<W: std::io::Write, T: std::fmt::Display + ?Sized>(
  writer: &mut W,
  value: &T,
) -> std::io::Result<()> {
  write_escaped_content_str(writer, &value.to_string())
}

#[inline]
pub(crate) fn write_escaped_text<W: std::io::Write, T: std::fmt::Display + ?Sized>(
  writer: &mut W,
  value: &T,
) -> std::io::Result<()> {
  write_escaped_str(writer, &value.to_string())
}

#[inline]
pub(crate) fn write_attr_value<W: std::io::Write, T: std::fmt::Display + ?Sized>(
  writer: &mut W,
  attr_name: &str,
  value: &T,
) -> std::io::Result<()> {
  writer.write_all(b" ")?;
  writer.write_all(attr_name.as_bytes())?;
  writer.write_all(b"=\"")?;
  write_escaped_text(writer, value)?;
  writer.write_all(b"\"")
}

#[inline]
pub(crate) fn write_attr_value_str<W: std::io::Write>(
  writer: &mut W,
  attr_name: &str,
  value: &str,
) -> std::io::Result<()> {
  writer.write_all(b" ")?;
  writer.write_all(attr_name.as_bytes())?;
  writer.write_all(b"=\"")?;
  write_escaped_str(writer, value)?;
  writer.write_all(b"\"")
}

#[inline]
pub(crate) fn write_list_attr_value<W, T>(
  writer: &mut W,
  attr_name: &str,
  values: &[T],
) -> std::io::Result<()>
where
  W: std::io::Write,
  T: std::fmt::Display,
{
  writer.write_all(b" ")?;
  writer.write_all(attr_name.as_bytes())?;
  writer.write_all(b"=\"")?;
  write_list_value(writer, values)?;
  writer.write_all(b"\"")
}

#[inline]
pub(crate) fn write_list_value<W, T>(writer: &mut W, values: &[T]) -> std::io::Result<()>
where
  W: std::io::Write,
  T: std::fmt::Display,
{
  let mut iter = values.iter();
  if let Some(first) = iter.next() {
    write_escaped_text(writer, first)?;
    for value in iter {
      writer.write_all(b" ")?;
      write_escaped_text(writer, value)?;
    }
  }
  Ok(())
}

#[inline]
pub(crate) fn write_list_text_content_value<W, T>(
  writer: &mut W,
  values: &[T],
) -> std::io::Result<()>
where
  W: std::io::Write,
  T: std::fmt::Display,
{
  let mut iter = values.iter();
  if let Some(first) = iter.next() {
    write_escaped_content_text(writer, first)?;
    for value in iter {
      writer.write_all(b" ")?;
      write_escaped_content_text(writer, value)?;
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
    crate::simple_type::TwipsMeasureValue::Twips(value) => write_escaped_text(writer, value),
    crate::simple_type::TwipsMeasureValue::UniversalMeasure(value) => {
      write_escaped_str(writer, value.to_lexical().as_str())
    }
  }
}

#[inline]
pub(crate) fn write_signed_twips_measure_value<W: std::io::Write>(
  writer: &mut W,
  value: &crate::simple_type::SignedTwipsMeasureValue,
) -> std::io::Result<()> {
  match value {
    crate::simple_type::SignedTwipsMeasureValue::Twips(value) => write_escaped_text(writer, value),
    crate::simple_type::SignedTwipsMeasureValue::UniversalMeasure(value) => {
      write_escaped_str(writer, value.to_lexical().as_str())
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
      write_escaped_text(writer, value)
    }
    crate::simple_type::DecimalNumberOrPercentValue::Percent(value) => write_escaped_str(
      writer,
      crate::units::format_percent_lexical(*value).as_str(),
    ),
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
      write_escaped_str(writer, value.to_lexical().as_str())
    }
  }
}

#[inline]
pub(crate) fn write_twips_measure_attr<W: std::io::Write>(
  writer: &mut W,
  attr_name: &str,
  value: &crate::simple_type::TwipsMeasureValue,
) -> std::io::Result<()> {
  write_simple_union_attr(writer, attr_name, |writer| {
    write_twips_measure_value(writer, value)
  })
}

#[inline]
pub(crate) fn write_signed_twips_measure_attr<W: std::io::Write>(
  writer: &mut W,
  attr_name: &str,
  value: &crate::simple_type::SignedTwipsMeasureValue,
) -> std::io::Result<()> {
  write_simple_union_attr(writer, attr_name, |writer| {
    write_signed_twips_measure_value(writer, value)
  })
}

#[inline]
pub(crate) fn write_decimal_number_or_percent_attr<W: std::io::Write>(
  writer: &mut W,
  attr_name: &str,
  value: &crate::simple_type::DecimalNumberOrPercentValue,
) -> std::io::Result<()> {
  write_simple_union_attr(writer, attr_name, |writer| {
    write_decimal_number_or_percent_value(writer, value)
  })
}

#[inline]
pub(crate) fn write_measurement_or_percent_attr<W: std::io::Write>(
  writer: &mut W,
  attr_name: &str,
  value: &crate::simple_type::MeasurementOrPercentValue,
) -> std::io::Result<()> {
  write_simple_union_attr(writer, attr_name, |writer| {
    write_measurement_or_percent_value(writer, value)
  })
}

#[inline]
fn write_simple_union_attr<W, F>(
  writer: &mut W,
  attr_name: &str,
  write_value: F,
) -> std::io::Result<()>
where
  W: std::io::Write,
  F: FnOnce(&mut W) -> std::io::Result<()>,
{
  writer.write_all(b" ")?;
  writer.write_all(attr_name.as_bytes())?;
  writer.write_all(b"=\"")?;
  write_value(writer)?;
  writer.write_all(b"\"")
}

#[inline]
pub(crate) fn write_xmlns_attr<W: std::io::Write>(
  writer: &mut W,
  prefix: Option<&str>,
  uri: &str,
) -> std::io::Result<()> {
  writer.write_all(b" xmlns")?;
  if let Some(prefix) = prefix
    && !prefix.is_empty()
  {
    writer.write_all(b":")?;
    writer.write_all(prefix.as_bytes())?;
  }
  writer.write_all(b"=\"")?;
  writer.write_all(uri.as_bytes())?;
  writer.write_all(b"\"")
}

#[cfg(test)]
mod tests {
  use quick_xml::{Reader, events::Event};

  use super::decode_attr_value;

  #[test]
  fn attribute_decode_preserves_literal_newlines_for_round_trip() {
    let mut reader = Reader::from_str("<x a=\"one\ntwo &amp; three\"/>");
    let Event::Empty(event) = reader.read_event().expect("empty event") else {
      panic!("expected empty element");
    };
    let attr = event
      .attributes()
      .with_checks(false)
      .next()
      .expect("attribute")
      .expect("valid attribute");

    let value = decode_attr_value(&attr, reader.decoder()).expect("decoded attribute");

    assert_eq!(value, "one\ntwo & three");
  }
}
