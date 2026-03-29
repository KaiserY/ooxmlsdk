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
