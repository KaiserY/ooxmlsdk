use quick_xml::Decoder;
use quick_xml::events::attributes::Attribute;

#[cfg(feature = "parts")]
pub mod data_part;
mod error;
#[cfg(feature = "parts")]
mod package;
mod xml;

#[cfg(feature = "parts")]
pub(crate) use crate::sdk::PackageOpenMode;
#[cfg(feature = "parts")]
pub use data_part::{
  AudioReferenceRelationship, MediaDataPart, MediaReferenceRelationship, VideoReferenceRelationship,
};
pub use error::{
  SdkError, invalid_enum_value, invalid_field_value, missing_field, unexpected_eof, unexpected_tag,
  validation_error,
};
#[cfg(feature = "parts")]
pub(crate) use package::PackageId;
#[cfg(feature = "parts")]
pub(crate) use package::{
  NewPartDescriptor, NewPartTargetMode, RelationshipInfo, RelationshipSet, SdkPackageStorage,
  StoredPart, default_part_extension_for_content_type,
};
#[cfg(feature = "parts")]
pub use package::{
  PartId, ReferenceRelationshipKind, Relationship, RelationshipRef, RelationshipTargetKind,
};
#[cfg(feature = "mce")]
pub(crate) use xml::mce_choice_replacement_children;
pub use xml::resolve_relationship_target_path;
pub use xml::resolve_zip_file_path;
pub(crate) use xml::{
  IoReader, IoTagEvent, SliceReader, SliceTagEvent, decode_attr_value, from_bytes_inner,
  from_reader_inner, from_str_inner, parse_decimal_number_or_percent_attr,
  parse_measurement_or_percent_attr, parse_signed_twips_measure_attr, parse_twips_measure_attr,
  read_outer_xml_borrowed, read_outer_xml_io, read_root_start_borrowed,
  read_root_start_borrowed_no_header, read_root_start_io, read_root_start_io_no_header,
  write_attr_value, write_attr_value_str, write_decimal_number_or_percent_attr, write_end_tag,
  write_end_tag_bytes, write_escaped_content_str, write_escaped_content_text, write_escaped_text,
  write_list_attr_value, write_list_text_content_value, write_measurement_or_percent_attr,
  write_signed_twips_measure_attr, write_start_tag_open, write_start_tag_open_bytes,
  write_twips_measure_attr, write_xmlns_attr,
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
  pub prefix: Box<str>,
  pub uri: Box<str>,
}

impl XmlNamespaceDecl {
  #[inline]
  pub fn new(prefix: impl Into<Box<str>>, uri: impl Into<Box<str>>) -> Self {
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
    .find(|declaration| declaration.prefix.as_ref() == prefix)
    .map(|declaration| declaration.uri.as_ref())
}

#[inline]
pub(crate) fn canonical_xmlns_prefix<'a>(prefix: &'a str, uri: &str) -> &'a str {
  match uri {
    "http://schemas.microsoft.com/office/excel/2006/main" => "xne",
    _ => prefix,
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
  xml::parse_attr_value(attr, decoder, ty, field)
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
  xml::parse_list_attr(attr, decoder, ty, field)
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
  xml::parse_list_value(value, ty, field)
}

macro_rules! define_attr_parser_forwarders {
  ($($name:ident -> $ty:ty),* $(,)?) => {
    $(
      #[inline]
      pub(crate) fn $name(
        attr: &Attribute<'_>,
        decoder: Decoder,
        ty: &'static str,
        field: &'static str,
      ) -> Result<$ty, SdkError> {
        xml::$name(attr, decoder, ty, field)
      }
    )*
  };
}

define_attr_parser_forwarders! {
  parse_u8_attr -> u8,
  parse_i8_attr -> i8,
  parse_u16_attr -> u16,
  parse_i16_attr -> i16,
  parse_u32_attr -> u32,
  parse_i32_attr -> i32,
  parse_i32_zero_on_overflow_attr -> i32,
  parse_u64_attr -> u64,
  parse_i64_attr -> i64,
}

#[inline]
pub(crate) fn parse_enum_attr<T>(
  attr: &Attribute<'_>,
  decoder: Decoder,
  ty: &'static str,
) -> Result<T, SdkError>
where
  T: crate::sdk::SdkEnum,
{
  xml::parse_enum_attr(attr, decoder, ty)
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
  xml::parse_value(value, ty, field)
}

#[inline]
#[cfg(feature = "parts")]
pub(crate) fn relationship_type_matches(actual: &str, canonical: &str) -> bool {
  actual == canonical || canonical_relationship_type(actual).as_ref() == canonical
}

#[inline]
#[cfg(feature = "parts")]
pub(crate) fn relationship_type_matches_alias(actual: &str, canonical: &str) -> bool {
  actual != canonical && canonical_relationship_type(actual).as_ref() == canonical
}

#[inline]
#[cfg(feature = "parts")]
pub(crate) fn canonical_relationship_type(value: &str) -> std::borrow::Cow<'_, str> {
  if value == "http://purl.oclc.org/ooxml/officeDocument/relationships/metadata/thumbnail" {
    return std::borrow::Cow::Borrowed(
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail",
    );
  }

  if value == "http://schemas.microsoft.com/office/2006/relationships/stylesWithtEffects" {
    return std::borrow::Cow::Borrowed(
      "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects",
    );
  }

  if let Some(suffix) =
    value.strip_prefix("http://purl.oclc.org/ooxml/officeDocument/relationships/")
  {
    let alias_suffix = match suffix {
      "customProperties" => "custom-properties",
      "extendedProperties" => "extended-properties",
      other => other,
    };
    return std::borrow::Cow::Owned(format!(
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/{alias_suffix}"
    ));
  }

  std::borrow::Cow::Borrowed(value)
}

#[inline]
#[cfg(feature = "parts")]
pub(crate) fn part_descriptor_matches(
  actual_relationship_type: &str,
  actual_content_type: &str,
  actual_path: &str,
  descriptor_relationship_type: &str,
  descriptor_content_type: &str,
  descriptor_path_prefix: &str,
  descriptor_target_name: &str,
) -> bool {
  if !relationship_type_matches(actual_relationship_type, descriptor_relationship_type) {
    return false;
  }
  if descriptor_content_type.is_empty()
    && relationship_type_matches(
      actual_relationship_type,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
    )
  {
    return package_main_part_path_matches(
      actual_path,
      descriptor_path_prefix,
      descriptor_target_name,
    );
  }
  if descriptor_content_type.is_empty() || actual_content_type == descriptor_content_type {
    return true;
  }

  relationship_type_matches(
    actual_relationship_type,
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
  ) && package_main_part_path_matches(actual_path, descriptor_path_prefix, descriptor_target_name)
}

#[inline]
#[cfg(feature = "parts")]
fn package_main_part_path_matches(
  actual_path: &str,
  descriptor_path_prefix: &str,
  descriptor_target_name: &str,
) -> bool {
  if descriptor_target_name.is_empty() {
    return false;
  }

  let Some(actual_stem) = actual_path.strip_suffix(".xml") else {
    return false;
  };

  if descriptor_path_prefix.is_empty() || descriptor_path_prefix == "." {
    return actual_stem == descriptor_target_name;
  }

  let descriptor_path_prefix = descriptor_path_prefix.trim_matches('/');
  actual_stem.len() == descriptor_path_prefix.len() + descriptor_target_name.len() + 1
    && actual_stem.starts_with(descriptor_path_prefix)
    && actual_stem.as_bytes()[descriptor_path_prefix.len()] == b'/'
    && &actual_stem[descriptor_path_prefix.len() + 1..] == descriptor_target_name
}

#[inline]
pub(crate) fn push_xml_text(
  value: &mut Option<String>,
  text: quick_xml::events::BytesText<'_>,
) -> Result<(), SdkError> {
  xml::push_xml_text(value, text)
}

#[inline]
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

  prefix != expected_prefix.as_bytes()
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

  #[test]
  fn i32_zero_on_overflow_attr_parser_zeroes_only_overflow() {
    let normal = with_first_attr(r#"<x val="-42"/>"#, |attr, decoder| {
      parse_i32_zero_on_overflow_attr(&attr, decoder, "X", "val")
    })
    .expect("parse normal i32");
    assert_eq!(normal, -42);

    let overflow = with_first_attr(r#"<x val="4294961151"/>"#, |attr, decoder| {
      parse_i32_zero_on_overflow_attr(&attr, decoder, "X", "val")
    })
    .expect("parse overflow as zero");
    assert_eq!(overflow, 0);

    let invalid = with_first_attr(r#"<x val="abc"/>"#, |attr, decoder| {
      parse_i32_zero_on_overflow_attr(&attr, decoder, "X", "val")
    });
    assert!(invalid.is_err());
  }

  #[cfg(feature = "parts")]
  #[test]
  fn variable_content_main_part_descriptors_match_by_target_path() {
    assert!(part_descriptor_matches(
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
      "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml",
      "xl/workbook.xml",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
      "",
      "xl",
      "workbook",
    ));
    assert!(!part_descriptor_matches(
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
      "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml",
      "xl/workbook.xml",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
      "",
      "word",
      "document",
    ));
  }

  #[cfg(feature = "parts")]
  #[test]
  fn relationship_type_aliases_canonicalize_to_transitional_types() {
    assert_eq!(
      canonical_relationship_type("http://purl.oclc.org/ooxml/officeDocument/relationships/theme"),
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme"
    );
    assert_eq!(
      canonical_relationship_type(
        "http://purl.oclc.org/ooxml/officeDocument/relationships/metadata/thumbnail"
      ),
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail"
    );
    assert!(relationship_type_matches(
      "http://schemas.microsoft.com/office/2006/relationships/stylesWithtEffects",
      "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects",
    ));
  }
}
