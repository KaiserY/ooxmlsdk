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
  DeEvent, PayloadEvent, XmlRead, append_fast_bytes_text_field, append_text_field,
  decode_utf16_xml_bytes, from_bytes_inner, from_reader_inner, parse_attr_value,
  parse_borrowed_list_attr, parse_decimal_number_or_percent_attr, parse_enum_attr, parse_f32_attr,
  parse_f32_bytes_raw, parse_f64_attr, parse_f64_bytes_raw, parse_i8_attr, parse_i8_bytes,
  parse_i16_attr, parse_i16_bytes, parse_i32_attr, parse_i32_bytes, parse_i64_attr,
  parse_i64_bytes, parse_list_attr, parse_list_value, parse_measurement_or_percent_attr,
  parse_signed_twips_measure_attr, parse_text_child_value, parse_twips_measure_attr, parse_u8_attr,
  parse_u8_bytes, parse_u16_attr, parse_u16_bytes, parse_u32_attr, parse_u32_bytes, parse_u64_attr,
  parse_u64_bytes, parse_value, read_root_start_borrowed, read_root_start_io,
  read_text_child_value, root_element_matches_namespace_local,
  write_decimal_number_or_percent_value, write_escaped_bytes, write_escaped_content_str,
  write_escaped_content_text, write_escaped_str, write_escaped_text, write_f32_value,
  write_f64_value, write_i8_value, write_i16_value, write_i32_value, write_i64_value,
  write_list_text_content_value, write_list_value, write_measurement_or_percent_value,
  write_signed_twips_measure_value, write_twips_measure_value, write_u8_value, write_u16_value,
  write_u32_value, write_u64_value, write_xmlns_attr, xml_local_name,
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
pub(crate) const REL_OFFICE_DOCUMENT: &[u8] =
  b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
#[cfg(feature = "parts")]
const REL_CORE_PROPERTIES: &[u8] =
  b"http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties";
#[cfg(feature = "parts")]
const REL_EXTENDED_PROPERTIES: &[u8] =
  b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties";
#[cfg(feature = "parts")]
pub(crate) const REL_HYPERLINK: &[u8] =
  b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink";
#[cfg(feature = "parts")]
pub(crate) const REL_AUDIO: &[u8] =
  b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio";
#[cfg(feature = "parts")]
pub(crate) const REL_MEDIA: &[u8] = b"http://schemas.microsoft.com/office/2007/relationships/media";
#[cfg(feature = "parts")]
pub(crate) const REL_VIDEO: &[u8] =
  b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/video";
#[cfg(feature = "parts")]
pub(crate) const REL_AF_CHUNK: &[u8] =
  b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk";
#[cfg(feature = "parts")]
const TRANSITIONAL_OFFICE_REL_PREFIX: &[u8] =
  b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/";
#[cfg(feature = "parts")]
const STRICT_OFFICE_REL_PREFIX: &[u8] = b"http://purl.oclc.org/ooxml/officeDocument/relationships/";

#[cfg(feature = "parts")]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct XmlRelationshipNamespaceUri(Box<[u8]>);

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
    Self(uri.into())
  }

  #[inline]
  pub fn as_str(&self) -> &str {
    std::str::from_utf8(self.uri_bytes()).unwrap_or("")
  }

  #[inline]
  pub fn uri_bytes(&self) -> &[u8] {
    self.0.as_ref()
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
#[cfg(feature = "parts")]
pub(crate) fn relationship_type_matches_bytes(actual: &[u8], canonical: &[u8]) -> bool {
  actual == canonical
    || strict_office_relationship_type_matches(actual, canonical)
    || o12_relationship_type_matches(actual, canonical)
}

#[inline]
#[cfg(feature = "parts")]
fn strict_office_relationship_type_matches(left: &[u8], right: &[u8]) -> bool {
  let Some(left_suffix) = office_relationship_type_suffix(left) else {
    return false;
  };
  office_relationship_type_suffix(right).is_some_and(|right_suffix| left_suffix == right_suffix)
}

#[inline]
#[cfg(feature = "parts")]
fn office_relationship_type_suffix(value: &[u8]) -> Option<&[u8]> {
  value
    .strip_prefix(TRANSITIONAL_OFFICE_REL_PREFIX)
    .or_else(|| value.strip_prefix(STRICT_OFFICE_REL_PREFIX))
}

#[inline]
#[cfg(feature = "parts")]
fn o12_relationship_type_matches(actual: &[u8], canonical: &[u8]) -> bool {
  matches!(
    (actual, canonical),
    (
      b"http://schemas.microsoft.com/office/2006/relationships/officeDocument",
      REL_OFFICE_DOCUMENT
    ) | (
      b"http://schemas.microsoft.com/office/2006/relationships/docPropsApp",
      REL_EXTENDED_PROPERTIES
    ) | (
      b"http://schemas.microsoft.com/package/2005/06/relationships/metadata/core-properties",
      REL_CORE_PROPERTIES
    )
  )
}

#[inline]
#[cfg(feature = "parts")]
pub(crate) fn is_data_part_reference_relationship_type_bytes(relationship_type: &[u8]) -> bool {
  relationship_type_matches_bytes(relationship_type, REL_AUDIO)
    || relationship_type_matches_bytes(relationship_type, REL_MEDIA)
    || relationship_type_matches_bytes(relationship_type, REL_VIDEO)
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
  use quick_xml::Decoder;
  use quick_xml::events::attributes::Attribute;

  fn with_first_attr<T>(
    xml: &str,
    f: impl FnOnce(Attribute<'_>, Decoder) -> Result<T, SdkError>,
  ) -> Result<T, SdkError> {
    let mut reader = from_bytes_inner(xml.as_bytes())?;
    let event = reader.next()?;
    let e = match event {
      PayloadEvent::Start(e, _) => e,
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

  #[cfg(feature = "parts")]
  #[test]
  fn o12_relationship_aliases_match_only_known_standard_relationships() {
    assert!(relationship_type_matches_bytes(
      b"http://schemas.microsoft.com/office/2006/relationships/officeDocument",
      REL_OFFICE_DOCUMENT,
    ));
    assert!(relationship_type_matches_bytes(
      b"http://schemas.microsoft.com/office/2006/relationships/docPropsApp",
      REL_EXTENDED_PROPERTIES,
    ));
    assert!(relationship_type_matches_bytes(
      b"http://schemas.microsoft.com/package/2005/06/relationships/metadata/core-properties",
      REL_CORE_PROPERTIES,
    ));
    assert!(!relationship_type_matches_bytes(
      b"http://schemas.microsoft.com/office/2006/relationships/vbaProject",
      b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/vbaProject",
    ));
  }
}
