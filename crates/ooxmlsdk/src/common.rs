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
pub(crate) use xml::mce_choice_replacement_child_bytes;
pub use xml::resolve_relationship_target_path;
pub use xml::resolve_zip_file_path;
pub(crate) use xml::{
  IoReader, IoTagEvent, SliceReader, SliceTagEvent, decode_attr_value, from_bytes_inner,
  from_reader_inner, from_str_inner, parse_decimal_number_or_percent_attr,
  parse_measurement_or_percent_attr, parse_signed_twips_measure_attr, parse_twips_measure_attr,
  read_raw_element_xml_borrowed, read_raw_element_xml_borrowed_bytes, read_raw_element_xml_io,
  read_raw_element_xml_io_bytes, read_raw_empty_xml_borrowed, read_raw_empty_xml_borrowed_bytes,
  read_raw_empty_xml_io, read_raw_empty_xml_io_bytes, read_root_start_borrowed,
  read_root_start_borrowed_no_header, read_root_start_io, read_root_start_io_no_header,
  write_attr_value, write_attr_value_str, write_decimal_number_or_percent_attr,
  write_escaped_content_str, write_escaped_content_text, write_escaped_text, write_list_attr_value,
  write_list_text_content_value, write_measurement_or_percent_attr,
  write_signed_twips_measure_attr, write_twips_measure_attr, write_xmlns_attr,
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

#[cfg(feature = "parts")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum XmlRelationshipNamespaceUri {
  Known(crate::namespaces::XmlKnownRelationshipNamespace),
  Custom(Box<[u8]>),
}

#[cfg(feature = "parts")]
impl XmlRelationshipNamespaceUri {
  #[inline]
  pub fn new(uri: impl AsRef<[u8]>) -> Self {
    Self::from_uri_bytes(uri.as_ref())
  }

  #[inline]
  pub fn from_uri(uri: &str) -> Self {
    Self::from_uri_bytes(uri.as_bytes())
  }

  #[inline]
  pub fn from_uri_bytes(uri: &[u8]) -> Self {
    if let Some(namespace) = crate::namespaces::XmlKnownRelationshipNamespace::from_uri_bytes(uri) {
      Self::Known(namespace)
    } else {
      Self::Custom(uri.into())
    }
  }

  #[inline]
  pub fn as_str(&self) -> &str {
    std::str::from_utf8(self.uri_bytes()).unwrap_or("")
  }

  #[inline]
  pub fn uri_bytes(&self) -> &[u8] {
    match self {
      Self::Known(namespace) => namespace.uri_bytes(),
      Self::Custom(uri) => uri.as_ref(),
    }
  }

  #[inline]
  pub fn known(&self) -> Option<crate::namespaces::XmlKnownRelationshipNamespace> {
    match self {
      Self::Known(namespace) => Some(*namespace),
      Self::Custom(_) => None,
    }
  }
}

#[cfg(feature = "parts")]
impl AsRef<str> for XmlRelationshipNamespaceUri {
  #[inline]
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum XmlNamespace {
  Known(crate::namespaces::XmlKnownNamespace),
  Raw(Box<[u8]>),
}

impl Default for XmlNamespace {
  #[inline]
  fn default() -> Self {
    Self::Raw(Box::new([]))
  }
}

impl XmlNamespace {
  #[inline]
  pub const fn known(namespace: crate::namespaces::XmlKnownNamespace) -> Self {
    Self::Known(namespace)
  }

  #[inline]
  pub(crate) fn raw(prefix: impl AsRef<[u8]>, uri: impl AsRef<[u8]>) -> Self {
    let prefix = prefix.as_ref();
    let uri = uri.as_ref();
    let mut raw = Vec::with_capacity(prefix.len() + 1 + uri.len());
    raw.extend_from_slice(prefix);
    raw.push(0);
    raw.extend_from_slice(uri);
    Self::Raw(raw.into_boxed_slice())
  }

  #[inline]
  pub(crate) fn parts(&self) -> (&[u8], &[u8]) {
    match self {
      Self::Known(namespace) => (namespace.prefix_bytes(), namespace.uri_bytes()),
      Self::Raw(raw) => split_raw_namespace(raw),
    }
  }
}

#[inline]
fn split_raw_namespace(raw: &[u8]) -> (&[u8], &[u8]) {
  if let Some(separator) = raw.iter().position(|byte| *byte == 0) {
    (&raw[..separator], &raw[separator + 1..])
  } else {
    (raw, &[])
  }
}

#[inline]
pub(crate) fn canonical_xmlns_prefix_bytes<'a>(prefix: &'a [u8], uri: &[u8]) -> &'a [u8] {
  match uri {
    b"http://schemas.microsoft.com/office/spreadsheetml/2009/9/main" => b"x14",
    b"http://schemas.microsoft.com/office/excel/2006/main" => b"xne",
    _ => prefix,
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct XmlOtherAttr(Box<[u8]>);

impl XmlOtherAttr {
  #[inline]
  pub fn new_raw(name: impl AsRef<[u8]>, raw_value: impl AsRef<[u8]>) -> Self {
    let name = name.as_ref();
    let raw_value = raw_value.as_ref();
    let mut attr = Vec::with_capacity(name.len() + 1 + raw_value.len());
    attr.extend_from_slice(name);
    attr.push(0);
    attr.extend_from_slice(raw_value);
    Self(attr.into_boxed_slice())
  }

  #[inline]
  pub fn name_bytes(&self) -> &[u8] {
    self.split_bytes().0
  }

  #[inline]
  pub fn raw_value_bytes(&self) -> &[u8] {
    self.split_bytes().1
  }

  #[inline]
  pub fn name(&self) -> &str {
    std::str::from_utf8(self.name_bytes()).unwrap_or("")
  }

  #[inline]
  pub fn raw_value(&self) -> &str {
    std::str::from_utf8(self.raw_value_bytes()).unwrap_or("")
  }

  #[inline]
  pub(crate) fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
    let (name, raw_value) = self.split_bytes();
    writer.write_all(b" ")?;
    writer.write_all(name)?;
    write_raw_attr_value(writer, raw_value)
  }

  #[inline]
  fn split_bytes(&self) -> (&[u8], &[u8]) {
    if let Some(separator) = self.0.iter().position(|byte| *byte == 0) {
      (&self.0[..separator], &self.0[separator + 1..])
    } else {
      (self.0.as_ref(), &[])
    }
  }
}

#[inline]
fn write_raw_attr_value<W: std::io::Write>(
  writer: &mut W,
  raw_value: &[u8],
) -> std::io::Result<()> {
  if !raw_value.contains(&b'"') {
    writer.write_all(b"=\"")?;
    writer.write_all(raw_value)?;
    writer.write_all(b"\"")
  } else if !raw_value.contains(&b'\'') {
    writer.write_all(b"='")?;
    writer.write_all(raw_value)?;
    writer.write_all(b"'")
  } else {
    writer.write_all(b"=\"")?;
    let mut chunks = raw_value.split(|byte| *byte == b'"');
    if let Some(first) = chunks.next() {
      writer.write_all(first)?;
    }
    for chunk in chunks {
      writer.write_all(b"&quot;")?;
      writer.write_all(chunk)?;
    }
    writer.write_all(b"\"")
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
pub(crate) fn relationship_type_matches_bytes(actual: &[u8], canonical: &[u8]) -> bool {
  if actual == canonical {
    return true;
  }
  relationship_type_matches_alias_bytes(actual, canonical)
}

#[inline]
#[cfg(feature = "parts")]
pub(crate) fn relationship_type_matches_alias_bytes(actual: &[u8], canonical: &[u8]) -> bool {
  if actual == canonical {
    return false;
  }
  let Some(canonical) = relationship_type_known_bytes(canonical) else {
    return false;
  };
  relationship_type_known_bytes(actual).is_some_and(|actual| actual == canonical)
}

#[inline]
#[cfg(feature = "parts")]
pub(crate) fn relationship_type_known_bytes(
  value: &[u8],
) -> Option<crate::namespaces::XmlKnownRelationshipNamespace> {
  crate::namespaces::XmlKnownRelationshipNamespace::from_uri_bytes(value)
}

#[inline]
#[cfg(feature = "parts")]
pub(crate) fn is_data_part_reference_relationship_type(
  relationship_type: Option<crate::namespaces::XmlKnownRelationshipNamespace>,
) -> bool {
  use crate::namespaces::XmlKnownRelationshipNamespace as RelationshipType;

  matches!(
    relationship_type,
    Some(
      RelationshipType::RelationshipAudio
        | RelationshipType::RelationshipMedia
        | RelationshipType::RelationshipVideo
    )
  )
}

#[inline]
#[cfg(feature = "parts")]
pub(crate) fn package_main_part_path_matches(
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
  fn variable_content_main_part_paths_match_by_target_path() {
    assert!(package_main_part_path_matches(
      "xl/workbook.xml",
      "xl",
      "workbook",
    ));
    assert!(!package_main_part_path_matches(
      "xl/workbook.xml",
      "word",
      "document",
    ));
  }
}
