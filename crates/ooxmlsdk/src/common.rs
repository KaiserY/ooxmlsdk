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
  read_raw_element_xml_borrowed, read_raw_element_xml_io, read_raw_empty_xml_borrowed,
  read_raw_empty_xml_io, read_root_start_borrowed, read_root_start_borrowed_no_header,
  read_root_start_io, read_root_start_io_no_header, write_attr_value, write_attr_value_str,
  write_decimal_number_or_percent_attr, write_end_tag_bytes, write_escaped_content_str,
  write_escaped_content_text, write_escaped_text, write_list_attr_value,
  write_list_text_content_value, write_measurement_or_percent_attr,
  write_signed_twips_measure_attr, write_start_tag_open_bytes, write_twips_measure_attr,
  write_xmlns_attr,
};
#[cfg(feature = "flat-opc")]
pub(crate) use xml::{read_outer_xml_borrowed, read_outer_xml_io};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum XmlHeaderType {
  #[default]
  None,
  Plain,
  Standalone,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct XmlPrefix(smallvec::SmallVec<[u8; 8]>);

impl XmlPrefix {
  #[inline]
  pub fn new(prefix: impl AsRef<[u8]>) -> Self {
    Self(smallvec::SmallVec::from_slice(prefix.as_ref()))
  }

  #[inline]
  pub fn as_bytes(&self) -> &[u8] {
    self.0.as_slice()
  }

  #[inline]
  pub fn as_str(&self) -> Option<&str> {
    std::str::from_utf8(self.as_bytes()).ok()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }
}

impl AsRef<[u8]> for XmlPrefix {
  #[inline]
  fn as_ref(&self) -> &[u8] {
    self.as_bytes()
  }
}

impl From<&str> for XmlPrefix {
  #[inline]
  fn from(value: &str) -> Self {
    Self::new(value.as_bytes())
  }
}

impl From<String> for XmlPrefix {
  #[inline]
  fn from(value: String) -> Self {
    Self::new(value.as_bytes())
  }
}

impl From<Box<str>> for XmlPrefix {
  #[inline]
  fn from(value: Box<str>) -> Self {
    Self::new(value.as_bytes())
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum XmlNamespaceUri {
  Known(crate::namespaces::XmlKnownNamespace),
  Custom(Box<str>),
}

impl Default for XmlNamespaceUri {
  #[inline]
  fn default() -> Self {
    Self::Custom(Box::from(""))
  }
}

impl XmlNamespaceUri {
  #[inline]
  pub fn new(uri: impl AsRef<str>) -> Self {
    Self::from_uri(uri.as_ref())
  }

  #[inline]
  pub fn from_uri(uri: &str) -> Self {
    if let Some(namespace) = crate::namespaces::XmlKnownNamespace::from_uri(uri) {
      Self::Known(namespace)
    } else {
      Self::Custom(uri.into())
    }
  }

  #[inline]
  pub fn as_str(&self) -> &str {
    match self {
      Self::Known(namespace) => namespace.uri(),
      Self::Custom(uri) => uri.as_ref(),
    }
  }

  #[inline]
  pub(crate) fn canonical_prefix<'a>(&self, prefix: &'a str) -> &'a str {
    match self {
      Self::Known(namespace) => namespace.prefix(),
      Self::Custom(_) => prefix,
    }
  }
}

impl AsRef<str> for XmlNamespaceUri {
  #[inline]
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct XmlNamespace {
  pub prefix: XmlPrefix,
  pub uri: XmlNamespaceUri,
}

impl XmlNamespace {
  #[inline]
  pub fn new(prefix: impl AsRef<[u8]>, uri: impl AsRef<str>) -> Self {
    Self {
      prefix: XmlPrefix::new(prefix),
      uri: XmlNamespaceUri::new(uri),
    }
  }

  #[inline]
  pub fn is_default(&self) -> bool {
    self.prefix.is_empty()
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
