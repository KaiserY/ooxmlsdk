use quick_xml::Decoder;
use quick_xml::events::attributes::Attribute;

#[cfg(feature = "parts")]
pub mod data_part;
mod error;
#[cfg(feature = "parts")]
pub mod extended_part;
#[cfg(feature = "parts")]
pub mod package;
#[cfg(feature = "parts")]
mod part;
mod xml;

#[cfg(feature = "parts")]
pub use data_part::{
  AudioReferenceRelationship, MediaDataPart, MediaReferenceRelationship, VideoReferenceRelationship,
};
pub use error::{
  SdkError, invalid_enum_value, invalid_field_value, missing_field, unexpected_eof, unexpected_tag,
  validation_error,
};
#[cfg(feature = "parts")]
pub use extended_part::ExtendedPart;
#[cfg(feature = "parts")]
pub(crate) use package::PackageId;
#[cfg(feature = "parts")]
pub use package::{
  NewPartDescriptor, NewPartTargetMode, PackageOpenMode, PartId, ReferenceRelationshipKind,
  RelationshipInfo, RelationshipSet, RelationshipTargetKind, SdkPackageStorage, StoredPart,
  StoredPartData, StoredPartDataKind,
};
#[cfg(feature = "parts")]
pub use part::{
  load_data_part_reference, load_extended_part, load_part_relationships, load_typed_child_part,
  resolve_relationship_part_path, save_data_part_reference, save_part_relationships,
  save_typed_child_part,
};
pub use xml::resolve_relationship_target_path;
pub use xml::resolve_zip_file_path;
pub(crate) use xml::{
  IoReader, IoTagEvent, SliceReader, SliceTagEvent, decode_attr_value, from_bytes_inner,
  from_reader_inner, from_str_inner, read_outer_xml_borrowed, read_outer_xml_io, write_attr_value,
  write_attr_value_str, write_end_tag, write_escaped_str, write_escaped_text, write_start_tag_open,
  write_xmlns_attr,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum XmlHeaderType {
  #[default]
  None,
  Plain,
  Standalone,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct XmlNamespaceDecl {
  pub prefix: String,
  pub uri: String,
}

impl XmlNamespaceDecl {
  #[inline]
  pub fn new(prefix: impl Into<String>, uri: impl Into<String>) -> Self {
    Self {
      prefix: prefix.into(),
      uri: uri.into(),
    }
  }

  #[inline]
  pub fn is_default(&self) -> bool {
    self.prefix.is_empty()
  }
}

#[inline]
pub fn find_xmlns_uri<'a>(declarations: &'a [XmlNamespaceDecl], prefix: &str) -> Option<&'a str> {
  declarations
    .iter()
    .find(|declaration| declaration.prefix == prefix)
    .map(|declaration| declaration.uri.as_str())
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
pub fn parse_u8_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<u8, SdkError> {
  xml::parse_u8_attr(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_i8_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<i8, SdkError> {
  xml::parse_i8_attr(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_u16_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<u16, SdkError> {
  xml::parse_u16_attr(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_i16_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<i16, SdkError> {
  xml::parse_i16_attr(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_u32_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<u32, SdkError> {
  xml::parse_u32_attr(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_i32_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<i32, SdkError> {
  xml::parse_i32_attr(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_u64_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<u64, SdkError> {
  xml::parse_u64_attr(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_i64_attr(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
  field: &'static str,
) -> Result<i64, SdkError> {
  xml::parse_i64_attr(attr, decoder, ty, field)
}

#[inline(always)]
pub fn parse_enum_attr<T>(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
) -> Result<T, SdkError>
where
  T: crate::sdk::SdkEnum,
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
pub fn relationship_type_matches(actual: &str, canonical: &str) -> bool {
  actual == canonical || relationship_type_matches_alias(actual, canonical)
}

#[inline(always)]
pub fn relationship_type_matches_alias(actual: &str, canonical: &str) -> bool {
  if let Some(suffix) =
    canonical.strip_prefix("http://schemas.openxmlformats.org/officeDocument/2006/relationships/")
  {
    let alias_suffix = match suffix {
      "custom-properties" => "customProperties",
      "extended-properties" => "extendedProperties",
      other => other,
    };
    return actual
      .strip_prefix("http://purl.oclc.org/ooxml/officeDocument/relationships/")
      .is_some_and(|actual_suffix| actual_suffix == alias_suffix);
  }

  if canonical == "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail"
  {
    return actual == "http://purl.oclc.org/ooxml/officeDocument/relationships/metadata/thumbnail";
  }

  if canonical == "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects" {
    return actual == "http://schemas.microsoft.com/office/2006/relationships/stylesWithtEffects";
  }

  false
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

#[inline]
#[cfg(feature = "parts")]
pub(crate) fn parent_zip_path(path: &str) -> String {
  path
    .rsplit_once('/')
    .map(|(dir_path, _)| {
      let mut resolved = resolve_zip_file_path(dir_path);
      if !resolved.is_empty() {
        resolved.push('/');
      }
      resolved
    })
    .unwrap_or_default()
}

#[inline]
#[cfg(feature = "parts")]
pub(crate) fn part_relationships_path(path: &str) -> String {
  let child_parent_path = parent_zip_path(path);
  let part_target = path.rsplit('/').next().unwrap_or_default();
  let mut rels_path = String::with_capacity(child_parent_path.len() + part_target.len() + 11);
  rels_path.push_str(&child_parent_path);
  rels_path.push_str("_rels/");
  rels_path.push_str(part_target);
  rels_path.push_str(".rels");
  resolve_zip_file_path(&rels_path)
}

#[inline]
#[cfg(feature = "parts")]
pub(crate) fn part_relationships_directory_path(path: &str) -> String {
  let mut rels_dir_path = parent_zip_path(path);
  rels_dir_path.push_str("_rels");
  resolve_zip_file_path(&rels_dir_path)
}

pub(crate) fn process_foreign_element_children_borrowed<'de, F>(
  xml_reader: &mut SliceReader<'de>,
  empty_tag: bool,
  visitor: &mut F,
) -> Result<(), SdkError>
where
  F: FnMut(
    &mut SliceReader<'de>,
    quick_xml::events::BytesStart<'de>,
    bool,
  ) -> Result<bool, SdkError>,
{
  if empty_tag {
    return Ok(());
  }

  loop {
    match xml_reader.next_tag_event()? {
      crate::common::SliceTagEvent::Start(e, false) => match visitor(xml_reader, e, false)? {
        true => {}
        false => {
          process_foreign_element_children_borrowed(xml_reader, false, visitor)?;
        }
      },
      crate::common::SliceTagEvent::Start(e, true) => {
        visitor(xml_reader, e, true)?;
      }
      crate::common::SliceTagEvent::End(_) => break,
      crate::common::SliceTagEvent::Eof => Err(unexpected_eof("process_foreign_element_children"))?,
      crate::common::SliceTagEvent::Decl(_) | crate::common::SliceTagEvent::Other => {}
    }
  }

  Ok(())
}

pub(crate) fn process_foreign_element_children_io<R, F>(
  xml_reader: &mut IoReader<R>,
  empty_tag: bool,
  visitor: &mut F,
) -> Result<(), SdkError>
where
  R: std::io::BufRead,
  F:
    FnMut(&mut IoReader<R>, quick_xml::events::BytesStart<'static>, bool) -> Result<bool, SdkError>,
{
  if empty_tag {
    return Ok(());
  }

  loop {
    let next_event = match xml_reader.next_borrowed()? {
      quick_xml::events::Event::Start(e) => Some((e.into_owned(), false)),
      quick_xml::events::Event::Empty(e) => Some((e.into_owned(), true)),
      quick_xml::events::Event::End(_) => break,
      quick_xml::events::Event::Eof => Err(unexpected_eof("process_foreign_element_children_io"))?,
      _ => None,
    };

    match next_event {
      Some((e, false)) => match visitor(xml_reader, e, false)? {
        true => {}
        false => {
          process_foreign_element_children_io(xml_reader, false, visitor)?;
        }
      },
      Some((e, true)) => {
        visitor(xml_reader, e, true)?;
      }
      None => {}
    }
  }

  Ok(())
}

pub(crate) fn skip_foreign_element_children_borrowed<'de>(
  xml_reader: &mut SliceReader<'de>,
  empty_tag: bool,
) -> Result<(), SdkError> {
  process_foreign_element_children_borrowed(
    xml_reader,
    empty_tag,
    &mut |_xml_reader, _e, _e_empty| Ok(false),
  )
}

pub(crate) fn skip_foreign_element_children_io<R: std::io::BufRead>(
  xml_reader: &mut IoReader<R>,
  empty_tag: bool,
) -> Result<(), SdkError> {
  process_foreign_element_children_io(xml_reader, empty_tag, &mut |_xml_reader, _e, _e_empty| {
    Ok(false)
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use quick_xml::events::Event;

  fn with_first_attr<T>(
    xml: &str,
    f: impl FnOnce(Attribute<'_>, Decoder) -> Result<T, SdkError>,
  ) -> Result<T, SdkError> {
    let mut reader = from_str_inner(xml)?;
    let event = reader.next()?;
    let e = match event {
      Event::Start(e) | Event::Empty(e) => e,
      other => panic!("expected start or empty tag, got {other:?}"),
    };
    let decoder = reader.decoder();
    let attr = e
      .attributes()
      .with_checks(false)
      .next()
      .expect("attribute")
      .unwrap();
    f(attr, decoder)
  }

  #[test]
  fn bool_like_attr_parsers_accept_raw_bytes_forms() {
    let on_off = with_first_attr(r#"<x val="off"/>"#, |attr, decoder| {
      parse_on_off_attr(&attr, decoder, "X", "val")
    })
    .expect("parse on_off");
    assert!(!on_off);

    let true_false_blank = with_first_attr(r#"<x val=""/>"#, |attr, decoder| {
      parse_true_false_blank_attr(&attr, decoder, "X", "val")
    })
    .expect("parse true_false_blank");
    assert!(!true_false_blank);

    let true_false = with_first_attr(r#"<x val="t"/>"#, |attr, decoder| {
      parse_true_false_attr(&attr, decoder, "X", "val")
    })
    .expect("parse true_false");
    assert!(true_false);
  }

  #[test]
  fn integer_attr_parsers_accept_bytes_fast_paths() {
    let unsigned = with_first_attr(r#"<x val="+42"/>"#, |attr, decoder| {
      parse_u32_attr(&attr, decoder, "X", "val")
    })
    .expect("parse u32");
    assert_eq!(unsigned, 42);

    let signed = with_first_attr(r#"<x val="-2147483648"/>"#, |attr, decoder| {
      parse_i32_attr(&attr, decoder, "X", "val")
    })
    .expect("parse i32");
    assert_eq!(signed, i32::MIN);

    let byte = with_first_attr(r#"<x val="255"/>"#, |attr, decoder| {
      parse_u8_attr(&attr, decoder, "X", "val")
    })
    .expect("parse u8");
    assert_eq!(byte, u8::MAX);
  }
}
