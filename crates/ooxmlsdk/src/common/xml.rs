use quick_xml::{
  Decoder, Reader,
  escape::unescape,
  events::{Event, attributes::Attribute},
};
use std::io::BufRead;

use super::{SdkError, invalid_field_value, unexpected_eof, unexpected_tag};

pub struct IoReader<R: BufRead> {
  reader: Reader<R>,
  buf: Vec<u8>,
  current: Option<Event<'static>>,
  pending: Option<Event<'static>>,
}

pub enum TagEvent<'xml> {
  Start(quick_xml::events::BytesStart<'xml>, bool),
  End(quick_xml::events::BytesEnd<'xml>),
  Decl(bool),
  Eof,
  Other,
}

pub trait XmlRead<'xml> {
  fn next(&mut self) -> Result<Event<'xml>, SdkError>;

  fn next_tag_event(&mut self) -> Result<TagEvent<'xml>, SdkError>;

  fn unread(&mut self, event: Event<'xml>) -> Result<(), SdkError>;

  fn decoder(&self) -> Decoder;

  fn read_inner<T: crate::sdk::SdkType>(
    &mut self,
    start: quick_xml::events::BytesStart<'xml>,
    empty: bool,
  ) -> Result<T, SdkError>;

  fn read_text(
    &mut self,
    end: quick_xml::name::QName<'_>,
    ty: &'static str,
    field: &'static str,
  ) -> Result<String, SdkError>;

  fn drain_text_field_from_event(
    &mut self,
    value: &mut Option<String>,
    first: Event<'xml>,
    ty: &'static str,
    field: &'static str,
  ) -> Result<(), SdkError>;

  fn read_raw_empty_xml_bytes(
    &mut self,
    start: quick_xml::events::BytesStart<'xml>,
  ) -> Result<Box<[u8]>, SdkError>;

  fn read_raw_element_xml_bytes(
    &mut self,
    start: quick_xml::events::BytesStart<'xml>,
  ) -> Result<Box<[u8]>, SdkError>;
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
  pub fn read_text(
    &mut self,
    end: quick_xml::name::QName<'_>,
    ty: &'static str,
    field: &'static str,
  ) -> Result<String, SdkError> {
    self.current = None;
    self.buf.clear();
    let mut value = None;
    loop {
      match self.next()? {
        event @ (Event::Text(_) | Event::CData(_) | Event::GeneralRef(_)) => {
          append_xml_text_event(&mut value, event, ty, field)?;
        }
        Event::End(e) if e.name() == end => {
          return Ok(value.unwrap_or_default());
        }
        Event::Start(e) | Event::Empty(e) => {
          return Err(unexpected_tag(
            ty,
            "text content",
            e.local_name().into_inner(),
          ));
        }
        Event::Eof => return Err(unexpected_eof(ty)),
        _ => {}
      }
    }
  }

  #[inline]
  pub fn drain_text_field_from_event(
    &mut self,
    value: &mut Option<String>,
    first: Event<'static>,
    ty: &'static str,
    field: &'static str,
  ) -> Result<(), SdkError> {
    self.current = None;
    let mut event = first;
    loop {
      match event {
        Event::Text(_) | Event::CData(_) | Event::GeneralRef(_) => {
          append_xml_text_event(value, event, ty, field)?;
        }
        event => {
          self.pending = Some(event);
          return Ok(());
        }
      }
      event = self.next()?;
    }
  }

  #[inline]
  pub fn next_tag_event(&mut self) -> Result<TagEvent<'static>, SdkError> {
    self.current = None;
    if let Some(event) = self.pending.take() {
      return Ok(Self::tag_event_from_owned(event));
    }

    self.buf.clear();
    Ok(match self.reader.read_event_into(&mut self.buf)? {
      Event::Start(e) => TagEvent::Start(e.into_owned(), false),
      Event::Empty(e) => TagEvent::Start(e.into_owned(), true),
      Event::End(e) => TagEvent::End(e.into_owned()),
      Event::Decl(e) => TagEvent::Decl(matches!(
        e.standalone(),
        Some(Ok(value)) if value.as_ref().eq_ignore_ascii_case(b"yes")
      )),
      Event::Eof => TagEvent::Eof,
      _ => TagEvent::Other,
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
  fn tag_event_from_owned(event: Event<'static>) -> TagEvent<'static> {
    match event {
      Event::Start(e) => TagEvent::Start(e, false),
      Event::Empty(e) => TagEvent::Start(e, true),
      Event::End(e) => TagEvent::End(e),
      Event::Decl(e) => TagEvent::Decl(matches!(
        e.standalone(),
        Some(Ok(value)) if value.as_ref().eq_ignore_ascii_case(b"yes")
      )),
      Event::Eof => TagEvent::Eof,
      _ => TagEvent::Other,
    }
  }
}

impl<R: BufRead> XmlRead<'static> for IoReader<R> {
  #[inline]
  fn next(&mut self) -> Result<Event<'static>, SdkError> {
    IoReader::next(self)
  }

  #[inline]
  fn next_tag_event(&mut self) -> Result<TagEvent<'static>, SdkError> {
    IoReader::next_tag_event(self)
  }

  #[inline]
  fn unread(&mut self, event: Event<'static>) -> Result<(), SdkError> {
    IoReader::unread(self, event)
  }

  #[inline]
  fn decoder(&self) -> Decoder {
    IoReader::decoder(self)
  }

  #[inline]
  fn read_inner<T: crate::sdk::SdkType>(
    &mut self,
    start: quick_xml::events::BytesStart<'static>,
    empty: bool,
  ) -> Result<T, SdkError> {
    T::read_inner(self, start, empty)
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
  fn drain_text_field_from_event(
    &mut self,
    value: &mut Option<String>,
    first: Event<'static>,
    ty: &'static str,
    field: &'static str,
  ) -> Result<(), SdkError> {
    IoReader::drain_text_field_from_event(self, value, first, ty, field)
  }

  #[inline]
  fn read_raw_empty_xml_bytes(
    &mut self,
    start: quick_xml::events::BytesStart<'static>,
  ) -> Result<Box<[u8]>, SdkError> {
    read_raw_empty_xml_io_bytes(start)
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
  pub fn read_text(
    &mut self,
    end: quick_xml::name::QName<'_>,
    ty: &'static str,
    field: &'static str,
  ) -> Result<String, SdkError> {
    let mut value = None;
    loop {
      match self.next()? {
        event @ (Event::Text(_) | Event::CData(_) | Event::GeneralRef(_)) => {
          append_xml_text_event(&mut value, event, ty, field)?;
        }
        Event::End(e) if e.name() == end => {
          return Ok(value.unwrap_or_default());
        }
        Event::Start(e) | Event::Empty(e) => {
          return Err(unexpected_tag(
            ty,
            "text content",
            e.local_name().into_inner(),
          ));
        }
        Event::Eof => return Err(unexpected_eof(ty)),
        _ => {}
      }
    }
  }

  #[inline]
  pub fn drain_text_field_from_event(
    &mut self,
    value: &mut Option<String>,
    first: Event<'de>,
    ty: &'static str,
    field: &'static str,
  ) -> Result<(), SdkError> {
    let mut event = first;
    loop {
      match event {
        Event::Text(_) | Event::CData(_) | Event::GeneralRef(_) => {
          append_xml_text_event(value, event, ty, field)?;
        }
        event => {
          self.pending = Some(event);
          return Ok(());
        }
      }
      event = self.next()?;
    }
  }

  #[inline]
  pub fn next_tag_event(&mut self) -> Result<TagEvent<'de>, SdkError> {
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
  fn tag_event_from_event(event: Event<'de>) -> TagEvent<'de> {
    match event {
      Event::Start(e) => TagEvent::Start(e, false),
      Event::Empty(e) => TagEvent::Start(e, true),
      Event::End(e) => TagEvent::End(e),
      Event::Decl(e) => TagEvent::Decl(matches!(
        e.standalone(),
        Some(Ok(value)) if value.as_ref().eq_ignore_ascii_case(b"yes")
      )),
      Event::Eof => TagEvent::Eof,
      _ => TagEvent::Other,
    }
  }
}

impl<'de> XmlRead<'de> for SliceReader<'de> {
  #[inline]
  fn next(&mut self) -> Result<Event<'de>, SdkError> {
    SliceReader::next(self)
  }

  #[inline]
  fn next_tag_event(&mut self) -> Result<TagEvent<'de>, SdkError> {
    SliceReader::next_tag_event(self)
  }

  #[inline]
  fn unread(&mut self, event: Event<'de>) -> Result<(), SdkError> {
    SliceReader::unread(self, event)
  }

  #[inline]
  fn decoder(&self) -> Decoder {
    SliceReader::decoder(self)
  }

  #[inline]
  fn read_inner<T: crate::sdk::SdkType>(
    &mut self,
    start: quick_xml::events::BytesStart<'de>,
    empty: bool,
  ) -> Result<T, SdkError> {
    T::read_inner(self, start, empty)
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
  fn drain_text_field_from_event(
    &mut self,
    value: &mut Option<String>,
    first: Event<'de>,
    ty: &'static str,
    field: &'static str,
  ) -> Result<(), SdkError> {
    SliceReader::drain_text_field_from_event(self, value, first, ty, field)
  }

  #[inline]
  fn read_raw_empty_xml_bytes(
    &mut self,
    start: quick_xml::events::BytesStart<'de>,
  ) -> Result<Box<[u8]>, SdkError> {
    read_raw_empty_xml_borrowed_bytes(start)
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
enum Utf16Endian {
  Little,
  Big,
}

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

fn find_ascii_ignore_case(haystack: &str, needle: &str) -> Option<usize> {
  haystack
    .as_bytes()
    .windows(needle.len())
    .position(|window| window.eq_ignore_ascii_case(needle.as_bytes()))
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

      trimmed
        .parse::<T>()
        .map_err(|_| invalid_field_value(ty, field, value))
    }
  }
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

fn append_xml_text_event(
  value: &mut Option<String>,
  event: Event<'_>,
  ty: &'static str,
  field: &'static str,
) -> Result<(), SdkError> {
  match event {
    Event::Text(text) => append_cow_text(value, text.xml10_content()?),
    Event::CData(text) => append_cow_text(value, text.xml10_content()?),
    Event::GeneralRef(text) => {
      let entity = text.xml10_content()?;
      let entity = resolve_general_ref_entity(entity.as_ref())
        .ok_or_else(|| invalid_field_value(ty, field, entity.to_string()))?;
      append_cow_text(value, entity);
    }
    _ => unreachable!("append_xml_text_event expects text-like XML events"),
  }
  Ok(())
}

#[inline]
fn append_cow_text(value: &mut Option<String>, text: std::borrow::Cow<'_, str>) {
  if let Some(value) = value {
    value.push_str(text.as_ref());
  } else {
    *value = Some(text.into_owned());
  }
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
  let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));

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
pub(crate) fn read_raw_empty_xml_borrowed_bytes<'de>(
  start: quick_xml::events::BytesStart<'de>,
) -> Result<Box<[u8]>, SdkError> {
  let start: &[u8] = start.as_ref();
  let mut xml = Vec::with_capacity(start.len() + RAW_EMPTY_XML_EXTRA_LEN);
  xml.push(b'<');
  xml.extend_from_slice(start);
  xml.extend_from_slice(b"/>");
  Ok(xml.into_boxed_slice())
}

#[inline]
pub(crate) fn read_raw_element_xml_borrowed_bytes<'de>(
  xml_reader: &mut SliceReader<'de>,
  start: quick_xml::events::BytesStart<'de>,
) -> Result<Box<[u8]>, SdkError> {
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
  Ok(xml.into_boxed_slice())
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
pub(crate) fn read_raw_empty_xml_io_bytes(
  start: quick_xml::events::BytesStart<'static>,
) -> Result<Box<[u8]>, SdkError> {
  let start: &[u8] = start.as_ref();
  let mut xml = Vec::with_capacity(start.len() + RAW_EMPTY_XML_EXTRA_LEN);
  xml.push(b'<');
  xml.extend_from_slice(start);
  xml.extend_from_slice(b"/>");
  Ok(xml.into_boxed_slice())
}

#[inline]
pub(crate) fn read_raw_element_xml_io_bytes<R: BufRead>(
  xml_reader: &mut IoReader<R>,
  start: quick_xml::events::BytesStart<'static>,
) -> Result<Box<[u8]>, SdkError> {
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
  Ok(xml.into_boxed_slice())
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
  context: &crate::sdk::MceContext,
) -> Result<Option<Vec<Box<[u8]>>>, SdkError> {
  let mut reader = Reader::from_reader(xml);
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
              let children = read_mce_container_children_bytes(&mut reader, MC_CHOICE_NAMES)?;
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
              fallback = Some(read_mce_container_children_bytes(
                &mut reader,
                MC_FALLBACK_NAMES,
              )?);
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
  context: &crate::sdk::MceContext,
  empty_tag: bool,
) -> Result<Option<Vec<Box<[u8]>>>, SdkError> {
  let qname = start.name();
  let qname = qname.as_ref();
  let namespaces = namespaces_from_context_with(context, &start)?;
  let is_ignorable = qname_prefix(qname).is_some_and(|prefix| {
    namespaces
      .iter()
      .rev()
      .find_map(|namespace| {
        let (namespace_prefix, namespace_uri) = namespace.parts();
        (namespace_prefix == prefix)
          .then_some(namespace_uri)
          .filter(|ns| context.is_ignorable_namespace_bytes(ns))
      })
      .is_some()
  });

  if is_ignorable && context.is_process_content_qname_bytes(qname) {
    if empty_tag {
      return Ok(Some(Vec::new()));
    }
    let end_name = start.name().as_ref().to_vec();
    return read_mce_container_children_bytes(reader, &[end_name.as_slice()]).map(Some);
  }

  if is_ignorable {
    if !empty_tag {
      skip_element(reader)?;
    }
    return Ok(Some(Vec::new()));
  }

  if !qname.contains(&b':') {
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
fn read_outer_xml_from_str_reader_bytes(
  reader: &mut Reader<&[u8]>,
  start: quick_xml::events::BytesStart<'_>,
  empty_tag: bool,
) -> Result<Box<[u8]>, SdkError> {
  let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
  if empty_tag {
    writer.write_event(Event::Empty(start))?;
    return Ok(writer.into_inner().into_inner().into_boxed_slice());
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

  Ok(writer.into_inner().into_inner().into_boxed_slice())
}

#[cfg(feature = "mce")]
fn qname_prefix(qname: &[u8]) -> Option<&[u8]> {
  qname
    .iter()
    .position(|byte| *byte == b':')
    .map(|index| &qname[..index])
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
) -> Result<Option<Box<[u8]>>, SdkError> {
  for attr in start.attributes() {
    let attr = attr?;
    if attr.key.as_ref() == name {
      return if let Some(value) = attr_raw_value(&attr) {
        Ok(Some(value.into()))
      } else {
        Ok(Some(
          decode_attr_value(&attr, reader.decoder())?
            .into_bytes()
            .into_boxed_slice(),
        ))
      };
    }
  }
  Ok(None)
}

#[cfg(feature = "mce")]
fn choice_requires_supported(
  requires: Option<&[u8]>,
  namespaces: &[crate::common::XmlNamespace],
  target: crate::sdk::FileFormatVersion,
) -> Result<bool, SdkError> {
  let Some(requires) = requires else {
    return Ok(false);
  };
  for prefix in requires
    .split(u8::is_ascii_whitespace)
    .filter(|part| !part.is_empty())
  {
    let Some(ns) = namespaces.iter().rev().find_map(|namespace| {
      let (namespace_prefix, namespace_uri) = namespace.parts();
      (namespace_prefix == prefix).then_some(namespace_uri)
    }) else {
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
  namespaces: &[crate::common::XmlNamespace],
  start: &quick_xml::events::BytesStart<'_>,
) -> Result<Vec<crate::common::XmlNamespace>, SdkError> {
  let mut merged = namespaces.to_vec();
  merged.extend(namespace_decls(start)?);
  Ok(merged)
}

#[cfg(feature = "mce")]
fn namespaces_from_context_with(
  context: &crate::sdk::MceContext,
  start: &quick_xml::events::BytesStart<'_>,
) -> Result<Vec<crate::common::XmlNamespace>, SdkError> {
  let mut namespaces = context.namespaces().to_vec();
  namespaces.extend(namespace_decls(start)?);
  Ok(namespaces)
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
  let mut xml_header = crate::common::XmlHeaderType::None;
  loop {
    match reader.next_tag_event()? {
      TagEvent::Decl(standalone) => {
        xml_header = if standalone {
          crate::common::XmlHeaderType::Standalone
        } else {
          crate::common::XmlHeaderType::Plain
        };
      }
      TagEvent::Start(e, empty) => {
        if e.local_name().into_inner() == local_name {
          return Ok((e, empty, xml_header));
        }
        return Err(unexpected_tag(owner, owner, e.name().as_ref()));
      }
      TagEvent::Eof => return Err(unexpected_eof(owner)),
      TagEvent::End(_) | TagEvent::Other => {}
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
  let mut xml_header = crate::common::XmlHeaderType::None;
  loop {
    match reader.next_tag_event()? {
      TagEvent::Decl(standalone) => {
        xml_header = if standalone {
          crate::common::XmlHeaderType::Standalone
        } else {
          crate::common::XmlHeaderType::Plain
        };
      }
      TagEvent::Start(e, empty) => {
        if e.local_name().into_inner() == local_name {
          return Ok((e, empty, xml_header));
        }
        return Err(unexpected_tag(owner, owner, e.name().as_ref()));
      }
      TagEvent::Eof => return Err(unexpected_eof(owner)),
      TagEvent::End(_) | TagEvent::Other => {}
    }
  }
}

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

fn split_qname(qname: &[u8]) -> (Option<&[u8]>, &[u8]) {
  if let Some(index) = qname.iter().position(|byte| *byte == b':') {
    (Some(&qname[..index]), &qname[index + 1..])
  } else {
    (None, qname)
  }
}

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
