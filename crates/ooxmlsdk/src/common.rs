use quick_xml::Decoder;
use quick_xml::events::attributes::Attribute;

mod error;
mod xml;

pub use error::{
  SdkError, invalid_enum_value, invalid_field_value, missing_field, unexpected_eof, unexpected_tag,
};
pub use xml::resolve_zip_file_path;
pub(crate) use xml::{
  XmlReader, decode_attr_value, expect_event_start, from_reader_inner, from_str_inner,
  write_attr_value, write_end_tag, write_escaped_text, write_start_tag_open, write_xmlns_attr,
};

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
    Ok(())
  } else {
    loop {
      match xml_reader.next()? {
        quick_xml::events::Event::Start(e) => {
          if !visitor(xml_reader, e, false)? {
            process_foreign_element_children(xml_reader, false, visitor)?;
          }
        }
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
}
