use quick_xml::{
  Decoder, Reader, Writer,
  events::{BytesRef, BytesText, Event, attributes::Attribute},
};
use std::io::BufRead;
use std::io::Cursor;

use super::{SdkError, invalid_field_value, unexpected_eof};

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
  pub fn next_tag_event(&mut self) -> Result<IoTagEvent, SdkError> {
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
    resolve_zip_file_path(&format!("{parent_path}{target}"))
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
    Ok(attr.decode_and_unescape_value(decoder)?.into_owned())
  }
}

#[inline]
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

#[inline]
pub(crate) fn parse_boolean_value_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  if let Some(value) = attr_raw_value(attr)
    && let Ok(value) = parse_boolean_value_bytes(value)
  {
    Ok(value)
  } else {
    let value = decode_attr_value(attr, decoder)?;
    parse_boolean_value_str(value.as_ref(), ty, field)
  }
}

#[inline]
pub(crate) fn parse_on_off_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  if let Some(value) = attr_raw_value(attr)
    && let Ok(value) = std::str::from_utf8(value)
    && let Ok(parsed) = parse_on_off_str(value, ty, field)
  {
    Ok(parsed)
  } else {
    let value = decode_attr_value(attr, decoder)?;
    parse_on_off_str(value.as_ref(), ty, field)
  }
}

#[inline]
pub(crate) fn parse_true_false_blank_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  if let Some(value) = attr_raw_value(attr)
    && let Ok(value) = std::str::from_utf8(value)
    && let Ok(parsed) = parse_true_false_blank_str(value, ty, field)
  {
    Ok(parsed)
  } else {
    let value = decode_attr_value(attr, decoder)?;
    parse_true_false_blank_str(value.as_ref(), ty, field)
  }
}

#[inline]
pub(crate) fn parse_true_false_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  if let Some(value) = attr_raw_value(attr)
    && let Ok(value) = std::str::from_utf8(value)
    && let Ok(parsed) = parse_true_false_str(value, ty, field)
  {
    Ok(parsed)
  } else {
    let value = decode_attr_value(attr, decoder)?;
    parse_true_false_str(value.as_ref(), ty, field)
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
pub(crate) fn parse_boolean_value_str(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  match value.as_bytes() {
    b"true" | b"1" => Ok(true),
    b"false" | b"0" => Ok(false),
    _ => Err(invalid_field_value(ty, field, value)),
  }
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
pub(crate) fn parse_on_off_str(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  match value.as_bytes() {
    b"true" | b"1" | b"on" => Ok(true),
    b"false" | b"0" | b"off" => Ok(false),
    _ => Err(invalid_field_value(ty, field, value)),
  }
}

#[inline(always)]
pub(crate) fn parse_true_false_blank_str(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  match value.as_bytes() {
    b"true" | b"t" => Ok(true),
    b"false" | b"f" | b"" => Ok(false),
    _ => Err(invalid_field_value(ty, field, value)),
  }
}

#[inline(always)]
pub(crate) fn parse_true_false_str(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  match value.as_bytes() {
    b"true" | b"t" => Ok(true),
    b"false" | b"f" => Ok(false),
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
pub(crate) fn parse_boolean_value_bytes(b: &[u8]) -> Result<bool, SdkError> {
  match b {
    b"true" | b"1" => Ok(true),
    b"false" | b"0" => Ok(false),
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
