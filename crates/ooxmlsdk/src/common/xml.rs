use quick_xml::{
  Decoder, Reader,
  events::{BytesRef, BytesText, Event, attributes::Attribute},
};
use std::io::BufRead;

use super::{SdkError, invalid_field_value};

pub trait XmlReader<'de> {
  fn next(&mut self) -> Result<Event<'de>, SdkError>;
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

#[inline(always)]
pub(crate) fn from_reader_inner<R: BufRead>(reader: R) -> Result<IoReader<R>, SdkError> {
  let mut xml_reader = quick_xml::Reader::from_reader(reader);
  xml_reader.config_mut().check_end_names = false;
  Ok(IoReader::new(xml_reader))
}

#[inline(always)]
pub(crate) fn from_str_inner(s: &str) -> Result<SliceReader<'_>, SdkError> {
  let mut xml_reader = quick_xml::Reader::from_str(s);
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

#[inline(always)]
pub(crate) fn decode_attr_value(
  attr: &Attribute<'_>,
  decoder: Decoder,
) -> Result<String, SdkError> {
  if let Some(value) = attr_raw_value(attr) {
    Ok(decoder.decode(value)?.into_owned())
  } else {
    Ok(attr.decode_and_unescape_value(decoder)?.into_owned())
  }
}

#[inline(always)]
pub(crate) fn parse_bool_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  if let Some(value) = attr_raw_value(attr)
    && let Ok(value) = parse_bool_bytes(value)
  {
    Ok(value)
  } else {
    let value = decode_attr_value(attr, decoder)?;
    parse_bool_str(value.as_ref(), ty, field)
  }
}

#[inline(always)]
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

#[inline(always)]
pub(crate) fn parse_enum_attr<T>(
  attr: &Attribute<'_>,
  decoder: Decoder,
  _ty: &'static str,
) -> Result<T, SdkError>
where
  T: std::str::FromStr<Err = SdkError>,
{
  if let Some(value) = attr_raw_value(attr)
    && let Ok(value) = std::str::from_utf8(value)
  {
    return value.parse::<T>();
  }

  let value = decode_attr_value(attr, decoder)?;
  value.parse::<T>()
}

#[inline(always)]
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

#[inline(always)]
pub(crate) fn parse_bool_str(
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

#[inline(always)]
pub(crate) fn parse_bool_bytes(b: &[u8]) -> Result<bool, SdkError> {
  match b {
    b"true" | b"1" | b"True" | b"TRUE" | b"t" | b"Yes" | b"YES" | b"yes" | b"y" => Ok(true),
    b"false" | b"0" | b"False" | b"FALSE" | b"f" | b"No" | b"NO" | b"no" | b"n" | b"" => Ok(false),
    other => Err(invalid_field_value(
      "bool",
      "value",
      String::from_utf8_lossy(other).into_owned(),
    )),
  }
}

#[inline(always)]
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

#[inline(always)]
pub(crate) fn push_xml_general_ref(
  value: &mut Option<String>,
  text: BytesRef<'_>,
  ty: &'static str,
  field: &'static str,
) -> Result<(), SdkError> {
  let entity = text.xml10_content()?;
  let entity = quick_xml::escape::resolve_predefined_entity(entity.as_ref())
    .ok_or_else(|| invalid_field_value(ty, field, entity.to_string()))?;

  if let Some(value) = value {
    value.push_str(entity);
  } else {
    *value = Some(entity.to_owned());
  }

  Ok(())
}

#[inline(always)]
pub(crate) fn write_start_tag_open<W: std::fmt::Write>(
  writer: &mut W,
  default_namespace_prefix: &str,
  tag_prefix: &str,
  local_name: &str,
) -> Result<(), std::fmt::Error> {
  writer.write_char('<')?;
  write_qualified_name(writer, default_namespace_prefix, tag_prefix, local_name)
}

#[inline(always)]
pub(crate) fn write_end_tag<W: std::fmt::Write>(
  writer: &mut W,
  default_namespace_prefix: &str,
  tag_prefix: &str,
  local_name: &str,
) -> Result<(), std::fmt::Error> {
  writer.write_str("</")?;
  write_qualified_name(writer, default_namespace_prefix, tag_prefix, local_name)?;
  writer.write_char('>')
}

#[inline(always)]
fn write_qualified_name<W: std::fmt::Write>(
  writer: &mut W,
  default_namespace_prefix: &str,
  tag_prefix: &str,
  local_name: &str,
) -> Result<(), std::fmt::Error> {
  if !tag_prefix.is_empty() && default_namespace_prefix != tag_prefix {
    writer.write_str(tag_prefix)?;
    writer.write_char(':')?;
  }

  writer.write_str(local_name)
}

#[inline(always)]
pub(crate) fn write_escaped_text<W: std::fmt::Write, T: std::fmt::Display + ?Sized>(
  writer: &mut W,
  value: &T,
) -> Result<(), std::fmt::Error> {
  writer.write_str(&quick_xml::escape::escape(value.to_string()))
}

#[inline(always)]
pub(crate) fn write_attr_value<W: std::fmt::Write, T: std::fmt::Display + ?Sized>(
  writer: &mut W,
  attr_name: &str,
  value: &T,
) -> Result<(), std::fmt::Error> {
  writer.write_char(' ')?;
  writer.write_str(attr_name)?;
  writer.write_str("=\"")?;
  write_escaped_text(writer, value)?;
  writer.write_char('"')
}

#[inline(always)]
pub(crate) fn write_xmlns_attr<W: std::fmt::Write>(
  writer: &mut W,
  prefix: Option<&str>,
  uri: &str,
) -> Result<(), std::fmt::Error> {
  writer.write_str(" xmlns")?;
  if let Some(prefix) = prefix
    && !prefix.is_empty()
  {
    writer.write_char(':')?;
    writer.write_str(prefix)?;
  }
  writer.write_str("=\"")?;
  writer.write_str(uri)?;
  writer.write_char('"')
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
            return Err(crate::common::unexpected_eof("xml"));
          }
          _ => continue,
        }
      }
    };
    match e.name().as_ref() {
      $tag_prefix | $tag => (),
      found => {
        return Err(crate::common::unexpected_tag("xml", "xml", found));
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
