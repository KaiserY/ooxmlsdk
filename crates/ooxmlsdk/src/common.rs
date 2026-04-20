use quick_xml::Decoder;
use quick_xml::events::attributes::Attribute;

mod error;
mod xml;

pub use error::{
  SdkError, invalid_enum_value, invalid_field_value, missing_field, unexpected_eof, unexpected_tag,
  validation_error,
};
pub use xml::resolve_relationship_target_path;
pub use xml::resolve_zip_file_path;
pub(crate) use xml::{
  XmlReader, decode_attr_value, from_reader_inner, from_str_inner, read_outer_xml,
  write_attr_value, write_attr_value_str, write_end_tag, write_escaped_str, write_escaped_text,
  write_start_tag_open, write_xmlns_attr,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum XmlHeaderType {
  #[default]
  None,
  Plain,
  Standalone,
}

#[inline(always)]
pub fn parse_bool_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_bool_attr(attr, decoder, ty, field)
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
  xml::parse_attr_value(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_enum_attr<T>(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
) -> Result<T, SdkError>
where
  T: std::str::FromStr<Err = SdkError>,
{
  xml::parse_enum_attr(attr, decoder, ty)
}

#[inline(always)]
pub fn parse_value<T>(value: &str, ty: &'static str, field: &'static str) -> Result<T, SdkError>
where
  T: std::str::FromStr,
{
  xml::parse_value(value, ty, field)
}

#[inline(always)]
pub fn parse_bool_str(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_bool_str(value, ty, field)
}

#[inline(always)]
pub fn parse_bool_bytes(b: &[u8]) -> Result<bool, SdkError> {
  xml::parse_bool_bytes(b)
}

#[inline(always)]
pub fn parse_boolean_value_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_boolean_value_attr(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_on_off_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_on_off_attr(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_true_false_blank_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_true_false_blank_attr(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_true_false_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_true_false_attr(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_boolean_value_str(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_boolean_value_str(value, ty, field)
}

#[inline(always)]
pub fn parse_on_off_str(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_on_off_str(value, ty, field)
}

#[inline(always)]
pub fn parse_true_false_blank_str(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_true_false_blank_str(value, ty, field)
}

#[inline(always)]
pub fn parse_true_false_str(
  value: &str,
  ty: &'static str,
  field: &'static str,
) -> Result<bool, SdkError> {
  xml::parse_true_false_str(value, ty, field)
}

#[inline(always)]
pub(crate) fn push_xml_text(
  value: &mut Option<String>,
  text: quick_xml::events::BytesText<'_>,
) -> Result<(), SdkError> {
  xml::push_xml_text(value, text)
}

#[inline(always)]
pub(crate) fn push_xml_general_ref(
  value: &mut Option<String>,
  text: quick_xml::events::BytesRef<'_>,
  ty: &'static str,
  field: &'static str,
) -> Result<(), SdkError> {
  xml::push_xml_general_ref(value, text, ty, field)
}

pub(crate) fn is_foreign_prefixed_child(name: &[u8], expected_prefix: &str) -> bool {
  let Some(separator_index) = name.iter().position(|b| *b == b':') else {
    return false;
  };
  let prefix = &name[..separator_index];

  prefix != b"mc" && prefix != expected_prefix.as_bytes()
}

#[inline(always)]
#[cfg(feature = "parts")]
pub(crate) fn parent_zip_path(path: &str) -> String {
  path
    .rsplit_once('/')
    .map(|(dir_path, _)| {
      let resolved = resolve_zip_file_path(&format!("{dir_path}/"));
      if resolved.is_empty() {
        resolved
      } else {
        format!("{resolved}/")
      }
    })
    .unwrap_or_default()
}

pub(crate) fn process_foreign_element_children<'de, R, F>(
  xml_reader: &mut R,
  empty_tag: bool,
  visitor: &mut F,
) -> Result<(), SdkError>
where
  R: XmlReader<'de>,
  F: FnMut(&mut R, quick_xml::events::BytesStart<'de>, bool) -> Result<bool, SdkError>,
{
  if empty_tag {
    return Ok(());
  }

  loop {
    match xml_reader.next()? {
      quick_xml::events::Event::Start(e) => match visitor(xml_reader, e, false)? {
        true => {}
        false => {
          process_foreign_element_children(xml_reader, false, visitor)?;
        }
      },
      quick_xml::events::Event::Empty(e) => {
        visitor(xml_reader, e, true)?;
      }
      quick_xml::events::Event::End(_) => break,
      quick_xml::events::Event::Eof => Err(unexpected_eof("process_foreign_element_children"))?,
      _ => {}
    }
  }

  Ok(())
}

pub(crate) fn process_markup_compatibility_children<'de, R, F>(
  xml_reader: &mut R,
  empty_tag: bool,
  visitor: &mut F,
) -> Result<(), SdkError>
where
  R: XmlReader<'de>,
  F: FnMut(&mut R, quick_xml::events::BytesStart<'de>, bool) -> Result<bool, SdkError>,
{
  if empty_tag {
    return Ok(());
  }

  let mut selected_branch = false;

  loop {
    match xml_reader.next()? {
      quick_xml::events::Event::Start(e) => match e.name().as_ref() {
        b"mc:Choice" | b"Choice" => {
          let should_use =
            !selected_branch && markup_compatibility_choice_supported(&e, xml_reader.decoder())?;
          if should_use {
            selected_branch = true;
            process_foreign_element_children(xml_reader, false, visitor)?;
          } else {
            skip_foreign_element_children(xml_reader, false)?;
          }
        }
        b"mc:Fallback" | b"Fallback" => {
          if selected_branch {
            skip_foreign_element_children(xml_reader, false)?;
          } else {
            selected_branch = true;
            process_foreign_element_children(xml_reader, false, visitor)?;
          }
        }
        _ => {
          skip_foreign_element_children(xml_reader, false)?;
        }
      },
      quick_xml::events::Event::Empty(e) => match e.name().as_ref() {
        b"mc:Choice" | b"Choice"
          if !selected_branch
            && markup_compatibility_choice_supported(&e, xml_reader.decoder())? =>
        {
          selected_branch = true;
        }
        b"mc:Fallback" | b"Fallback" if !selected_branch => {
          selected_branch = true;
        }
        _ => {}
      },
      quick_xml::events::Event::End(e) => match e.name().as_ref() {
        b"mc:AlternateContent" | b"AlternateContent" => break,
        _ => {}
      },
      quick_xml::events::Event::Eof => {
        Err(unexpected_eof("process_markup_compatibility_children"))?
      }
      _ => {}
    }
  }

  Ok(())
}

fn markup_compatibility_choice_supported<'a>(
  choice_start: &quick_xml::events::BytesStart<'a>,
  decoder: Decoder,
) -> Result<bool, SdkError> {
  let mut requires = None;

  for attr in choice_start.attributes().with_checks(false) {
    let attr = attr?;
    if attr.key.as_ref() == b"Requires" {
      requires = Some(decode_attr_value(&attr, decoder)?);
      break;
    }
  }

  let Some(requires) = requires else {
    return Ok(false);
  };

  for prefix in requires.split_ascii_whitespace() {
    if crate::namespaces::uri_by_prefix(prefix).is_none() {
      return Ok(false);
    }
  }

  Ok(true)
}

fn skip_foreign_element_children<'de, R: XmlReader<'de>>(
  xml_reader: &mut R,
  empty_tag: bool,
) -> Result<(), SdkError> {
  process_foreign_element_children(xml_reader, empty_tag, &mut |_xml_reader, _e, _e_empty| {
    Ok(false)
  })
}
