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
pub(crate) use package::{
  NewPartDescriptor, NewPartTargetMode, PackageSaveEntry, PackageToken, PartKey, PartSlot,
  RelationshipInfo, RelationshipSet, SdkPackageStorage, StoredPart, create_package_temp_file,
  default_part_extension_for_content_type, replace_package_file,
};
#[cfg(feature = "parts")]
pub use package::{
  ReferenceRelationshipKind, Relationship, RelationshipRef, RelationshipTargetKind,
};
pub use xml::resolve_relationship_target_path;
pub use xml::resolve_zip_file_path;
pub(crate) use xml::{
  AttributeQNameTarget, DeEvent, PayloadEvent, ReadContext, XmlRead, append_de_text_field,
  append_fast_bytes_text_field, from_bytes_inner, from_reader_inner, parse_bytes_list_attr,
  parse_decimal_number_or_percent_attr, parse_enum_attr, parse_f32_attr, parse_f64_attr,
  parse_i8_attr, parse_i16_attr, parse_i32_attr, parse_i32_bytes, parse_i64_attr, parse_list_attr,
  parse_list_value, parse_measurement_or_percent_attr, parse_signed_twips_measure_attr,
  parse_text_child_value, parse_twips_measure_attr, parse_u8_attr, parse_u16_attr, parse_u32_attr,
  parse_u32_bytes, parse_u64_attr, parse_value, parse_xml_namespace_uri,
  read_enum_text_child_value, read_f32_text_child_value, read_f64_text_child_value,
  read_i8_text_child_value, read_i16_text_child_value, read_i32_text_child_value,
  read_i64_text_child_value, read_root_start_borrowed, read_root_start_io, read_text_child_value,
  read_u8_text_child_value, read_u16_text_child_value, read_u32_text_child_value,
  read_u64_text_child_value, write_coordinate_value, write_coordinate32_value,
  write_decimal_number_or_percent_value, write_drawingml_percentage_value,
  write_escaped_content_str, write_escaped_str, write_f32_value, write_f64_value,
  write_hps_measure_value, write_i8_value, write_i16_value, write_i32_value, write_i64_value,
  write_list_content_str_value, write_list_str_value, write_list_value_with,
  write_measurement_or_percent_value, write_signed_hps_measure_value,
  write_signed_twips_measure_value, write_text_bullet_size_value, write_text_point_value,
  write_twips_measure_value, write_u8_value, write_u16_value, write_u32_value, write_u64_value,
  write_xmlns_attr, xml_local_name,
};
#[cfg(feature = "parts")]
pub(crate) use xml::{decode_utf16_xml_bytes, root_element_matches_namespace_local};
#[cfg(feature = "flat-opc")]
pub(crate) use xml::{read_outer_xml_borrowed, read_outer_xml_io};

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
const REL_THUMBNAIL: &[u8] =
  b"http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail";
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
#[cfg(all(feature = "parts", feature = "flat-opc"))]
pub(crate) const REL_AF_CHUNK: &[u8] =
  b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk";
#[cfg(feature = "parts")]
const TRANSITIONAL_OFFICE_REL_PREFIX: &[u8] =
  b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/";
#[cfg(feature = "parts")]
const STRICT_OFFICE_REL_PREFIX: &[u8] = b"http://purl.oclc.org/ooxml/officeDocument/relationships/";
#[cfg(feature = "parts")]
const STRICT_REL_THUMBNAIL: &[u8] =
  b"http://purl.oclc.org/ooxml/officeDocument/relationships/metadata/thumbnail";

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

pub(crate) struct XmlNamespaceState {
  primary_canonical_dedup: Option<crate::namespaces::XmlKnownNamespace>,
  has_additional_canonical_dedup: bool,
}

impl XmlNamespaceState {
  #[inline]
  pub(crate) fn new() -> Self {
    Self {
      primary_canonical_dedup: None,
      has_additional_canonical_dedup: false,
    }
  }

  #[inline(always)]
  fn contains_canonical(
    &self,
    declarations: &[XmlNamespace],
    namespace: crate::namespaces::XmlKnownNamespace,
  ) -> bool {
    self.primary_canonical_dedup == Some(namespace)
      || (self.has_additional_canonical_dedup
        && Self::contains_additional_canonical(declarations, namespace))
  }

  #[cold]
  #[inline(never)]
  fn contains_additional_canonical(
    declarations: &[XmlNamespace],
    namespace: crate::namespaces::XmlKnownNamespace,
  ) -> bool {
    declarations.iter().any(
      |declaration| matches!(declaration, XmlNamespace::Known(existing) if *existing == namespace),
    )
  }

  #[inline]
  fn insert_canonical(&mut self, namespace: crate::namespaces::XmlKnownNamespace) {
    if self.primary_canonical_dedup.is_none() {
      self.primary_canonical_dedup = Some(namespace);
    } else if self.primary_canonical_dedup != Some(namespace) {
      self.has_additional_canonical_dedup = true;
    }
  }
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

  /// Records a namespace declaration in its statically known form when
  /// possible. The canonical declaration makes generated QNames writable
  /// without schema-specific prefix repair. A non-canonical source alias is
  /// retained as a raw declaration because MCE QName lists and raw XML can
  /// still refer to that lexical prefix.
  #[cfg(test)]
  #[inline]
  pub(crate) fn push_normalized(
    declarations: &mut Vec<Self>,
    prefix: impl AsRef<[u8]>,
    uri: impl AsRef<[u8]>,
  ) {
    let prefix = prefix.as_ref();
    let uri = uri.as_ref();
    let known = crate::namespaces::XmlKnownNamespace::from_compatible_uri_bytes(uri);
    let canonical = known.is_some_and(|namespace| {
      namespace.schema_namespace() == namespace && namespace.prefix_bytes() == prefix
    });
    let mut state = XmlNamespaceState::new();
    for declaration in declarations.iter() {
      if let Self::Known(namespace) = declaration {
        state.insert_canonical(*namespace);
      }
    }
    Self::push_normalized_resolved(declarations, &mut state, prefix, uri, known, canonical);
  }

  #[inline(never)]
  pub(crate) fn push_normalized_resolved(
    declarations: &mut Vec<Self>,
    state: &mut XmlNamespaceState,
    prefix: &[u8],
    uri: &[u8],
    known: Option<crate::namespaces::XmlKnownNamespace>,
    canonical: bool,
  ) {
    Self::push_normalized_root_resolved(declarations, state, prefix, uri, known, canonical);
  }

  #[inline(always)]
  fn push_normalized_root_resolved(
    declarations: &mut Vec<Self>,
    state: &mut XmlNamespaceState,
    prefix: &[u8],
    uri: &[u8],
    known: Option<crate::namespaces::XmlKnownNamespace>,
    canonical: bool,
  ) {
    // The generated namespace table guarantees that an ordinary known variant
    // owns this lexical prefix. XML already rejects duplicate attributes, so
    // the common path only needs to retain the enum value. Preview and alias
    // variants continue through the normalization path below.
    if let Some(known) = known
      && canonical
    {
      if !state.contains_canonical(declarations, known) {
        declarations.push(Self::Known(known));
      }
      return;
    }

    Self::push_normalized_resolved_slow(declarations, state, prefix, uri, known);
  }

  #[cold]
  #[inline(never)]
  fn push_normalized_resolved_slow(
    declarations: &mut Vec<Self>,
    state: &mut XmlNamespaceState,
    prefix: &[u8],
    uri: &[u8],
    known: Option<crate::namespaces::XmlKnownNamespace>,
  ) {
    let Some(known) = known else {
      Self::push_raw_resolved(declarations, state, prefix, uri);
      return;
    };

    // A statically known lexical alias can share both the URI and schema
    // identity of the generated namespace while using a different prefix
    // (for example `ve` for `mc`).  Keep preview namespaces whose URI differs,
    // but use the generated schema namespace as the writable declaration when
    // the URI is identical.
    let schema_namespace = known.schema_namespace();
    let writable = if schema_namespace.uri_bytes() == known.uri_bytes() {
      schema_namespace
    } else {
      known
    };
    let canonical_prefix = writable.prefix_bytes();
    let mut has_known = false;
    let mut has_canonical_prefix = false;
    for declaration in declarations.iter() {
      match declaration {
        Self::Known(existing) => {
          has_known |= *existing == writable;
          has_canonical_prefix |= existing.prefix_bytes() == canonical_prefix;
        }
        Self::Raw(raw) => {
          has_canonical_prefix |= split_raw_namespace(raw).0 == canonical_prefix;
        }
      }
    }
    if !has_known {
      if has_canonical_prefix {
        declarations.retain(|declaration| declaration.parts().0 != canonical_prefix);
      }
      declarations.push(Self::Known(writable));
    }
    state.insert_canonical(writable);

    if prefix != canonical_prefix {
      Self::push_raw_resolved(declarations, state, prefix, uri);
    }
  }

  #[cold]
  #[inline(never)]
  fn push_raw_resolved(
    declarations: &mut Vec<Self>,
    state: &mut XmlNamespaceState,
    prefix: &[u8],
    uri: &[u8],
  ) {
    if crate::namespaces::XmlKnownNamespace::from_prefix_bytes(prefix)
      .is_some_and(|namespace| state.contains_canonical(declarations, namespace))
    {
      return;
    }
    declarations.push(Self::raw(prefix, uri));
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
pub(crate) fn parse_xml_namespace_list(
  read_context: &ReadContext,
  local_declarations: &[XmlNamespace],
  value: &[u8],
) -> Vec<XmlNamespace> {
  let mut namespaces = Vec::with_capacity(value.len() / 3 + 1);
  for prefix in value.split(|byte| matches!(byte, b' ' | b'\r' | b'\n' | b'\t')) {
    if !prefix.is_empty() {
      namespaces.push(
        local_declarations
          .iter()
          .rev()
          .find(|declaration| declaration.parts().0 == prefix)
          .map(|declaration| {
            crate::namespaces::XmlKnownNamespace::from_compatible_uri_bytes(declaration.parts().1)
              .map_or_else(|| declaration.clone(), XmlNamespace::Known)
          })
          .unwrap_or_else(|| read_context.namespace_for_prefix(prefix)),
      );
    }
  }
  namespaces
}

#[inline]
pub(crate) fn parse_xml_namespace_list_on(
  read_context: &ReadContext,
  start: &quick_xml::events::BytesStart<'_>,
  value: &[u8],
) -> Vec<XmlNamespace> {
  let mut namespaces = Vec::with_capacity(value.len() / 3 + 1);
  for prefix in value.split(|byte| matches!(byte, b' ' | b'\r' | b'\n' | b'\t')) {
    if !prefix.is_empty() {
      namespaces.push(read_context.namespace_for_prefix_on(start, prefix));
    }
  }
  namespaces
}

#[inline]
pub(crate) fn write_xml_namespace_list_value<W: std::io::Write>(
  writer: &mut W,
  namespaces: &[XmlNamespace],
) -> std::io::Result<()> {
  for (index, namespace) in namespaces.iter().enumerate() {
    if index != 0 {
      writer.write_all(b" ")?;
    }
    writer.write_all(namespace.parts().0)?;
  }
  Ok(())
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
pub(crate) fn write_mc_ignorable_attr<W: std::io::Write>(
  writer: &mut W,
  namespaces: &[XmlNamespace],
) -> std::io::Result<()> {
  write_xml_namespace_list_attr(writer, b" mc:Ignorable=\"", namespaces)
}

#[inline]
pub(crate) fn write_mc_preserve_attributes_attr<W: std::io::Write>(
  writer: &mut W,
  raw_value: &[u8],
) -> std::io::Result<()> {
  write_mc_attr(writer, b" mc:PreserveAttributes=\"", raw_value)
}

#[inline]
pub(crate) fn write_mc_process_content_attr<W: std::io::Write>(
  writer: &mut W,
  raw_value: &[u8],
) -> std::io::Result<()> {
  write_mc_attr(writer, b" mc:ProcessContent=\"", raw_value)
}

#[inline]
pub(crate) fn write_mc_must_understand_attr<W: std::io::Write>(
  writer: &mut W,
  namespaces: &[XmlNamespace],
) -> std::io::Result<()> {
  write_xml_namespace_list_attr(writer, b" mc:MustUnderstand=\"", namespaces)
}

#[inline]
fn write_xml_namespace_list_attr<W: std::io::Write>(
  writer: &mut W,
  prefix: &[u8],
  namespaces: &[XmlNamespace],
) -> std::io::Result<()> {
  writer.write_all(prefix)?;
  write_xml_namespace_list_value(writer, namespaces)?;
  writer.write_all(b"\"")
}

#[inline]
pub(crate) fn write_mc_attr<W: std::io::Write>(
  writer: &mut W,
  prefix: &[u8],
  raw_value: &[u8],
) -> std::io::Result<()> {
  writer.write_all(prefix)?;
  writer.write_all(raw_value)?;
  writer.write_all(b"\"")
}

#[inline]
#[cfg(feature = "parts")]
pub(crate) fn relationship_type_matches_bytes(actual: &[u8], canonical: &[u8]) -> bool {
  actual == canonical
    || strict_office_relationship_type_matches(actual, canonical)
    || strict_thumbnail_relationship_type_matches(actual, canonical)
    || o12_relationship_type_matches(actual, canonical)
}

#[inline]
#[cfg(feature = "parts")]
fn strict_office_relationship_type_matches(left: &[u8], right: &[u8]) -> bool {
  let Some(left_suffix) = office_relationship_type_suffix(left) else {
    return false;
  };
  office_relationship_type_suffix(right).is_some_and(|right_suffix| {
    left_suffix == right_suffix
      || matches!(
        (left_suffix, right_suffix),
        (b"customProperties", b"custom-properties")
          | (b"custom-properties", b"customProperties")
          | (b"extendedProperties", b"extended-properties")
          | (b"extended-properties", b"extendedProperties")
      )
  })
}

#[inline]
#[cfg(feature = "parts")]
fn strict_thumbnail_relationship_type_matches(left: &[u8], right: &[u8]) -> bool {
  matches!(
    (left, right),
    (STRICT_REL_THUMBNAIL, REL_THUMBNAIL) | (REL_THUMBNAIL, STRICT_REL_THUMBNAIL)
  )
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::sdk::SdkType;
  use quick_xml::Decoder;
  use quick_xml::events::attributes::Attribute;

  fn with_first_attr<T>(
    xml: &str,
    f: impl FnOnce(Attribute<'_>, Decoder) -> Result<T, SdkError>,
  ) -> Result<T, SdkError> {
    let mut reader = from_bytes_inner(xml.as_bytes());
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

  fn serialize_word_document(xml: &str) -> String {
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document::from_bytes(
      xml.as_bytes(),
    )
    .expect("parse document")
    .to_xml()
    .expect("serialize document")
  }

  #[test]
  fn word_document_canonicalizes_mc_attributes_without_dropping_aliases() {
    const W: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    const MC: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";

    for alias in ["ve", "ns1"] {
      let xml = format!(
        r#"<w:document xmlns:w="{W}" xmlns:{alias}="{MC}" {alias}:Ignorable="w14"><w:body/></w:document>"#,
      );
      let serialized = serialize_word_document(&xml);
      assert!(serialized.contains(&format!(r#"xmlns:mc="{MC}""#)));
      assert!(serialized.contains(&format!(r#"xmlns:{alias}="{MC}""#)));
      assert!(serialized.contains(r#"mc:Ignorable="w14""#));
    }
  }

  #[test]
  fn word_document_adds_canonical_word_namespace_for_source_alias() {
    const W: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    let xml = format!(
      r#"<ns0:document xmlns:ns0="{W}"><ns0:body><ns0:p><ns0:r><ns0:t>ref</ns0:t></ns0:r></ns0:p></ns0:body></ns0:document>"#,
    );

    let serialized = serialize_word_document(&xml);
    assert!(serialized.contains(&format!(r#"xmlns:w="{W}""#)));
    assert_eq!(serialized.matches("xmlns:w=").count(), 1);
    assert!(serialized.contains(&format!(r#"xmlns:ns0="{W}""#)));
    assert!(serialized.contains("<w:document"));
    assert!(serialized.contains("<w:body"));
    assert!(!serialized.contains("<ns0:"));
  }

  #[test]
  fn normalized_namespace_duplicate_preserves_first_known_position() {
    const CP: &[u8] = b"http://schemas.openxmlformats.org/package/2006/metadata/core-properties";
    const DC: &[u8] = b"http://purl.org/dc/elements/1.1/";
    let mut declarations = Vec::new();

    XmlNamespace::push_normalized(&mut declarations, b"cp", CP);
    XmlNamespace::push_normalized(&mut declarations, b"dc", DC);
    XmlNamespace::push_normalized(&mut declarations, b"", CP);

    assert_eq!(
      declarations
        .iter()
        .map(XmlNamespace::parts)
        .collect::<Vec<_>>(),
      vec![
        (b"cp".as_slice(), CP),
        (b"dc".as_slice(), DC),
        (b"".as_slice(), CP)
      ]
    );
  }

  #[test]
  fn word_document_canonicalizes_mce_prefix_lists_without_dropping_aliases() {
    const W: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    const MC: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";
    const W14: &str = "http://schemas.microsoft.com/office/word/2010/wordml";
    let xml = format!(
      r#"<w:document xmlns:w="{W}" xmlns:mc="{MC}" xmlns:future="{W14}" mc:Ignorable="future"><w:body/></w:document>"#,
    );

    let serialized = serialize_word_document(&xml);
    assert!(serialized.contains(&format!(r#"xmlns:w14="{W14}""#)));
    assert!(serialized.contains(&format!(r#"xmlns:future="{W14}""#)));
    assert!(serialized.contains(r#"mc:Ignorable="w14""#));
  }

  #[test]
  fn word_document_preserves_static_preview_namespace_uris() {
    const W: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    const MC: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";
    const W14_PREVIEW: &str = "http://schemas.microsoft.com/office/word/2009/2/wordml";
    const W15_PREVIEW: &str = "http://schemas.microsoft.com/office/word/2010/11/wordml";
    let xml = format!(
      r#"<w:document xmlns:w="{W}" xmlns:mc="{MC}" xmlns:w14="{W14_PREVIEW}" xmlns:w15="{W15_PREVIEW}" mc:Ignorable="w14 w15"><w:body/></w:document>"#,
    );

    let serialized = serialize_word_document(&xml);
    assert!(serialized.contains(&format!(r#"xmlns:w14="{W14_PREVIEW}""#)));
    assert!(serialized.contains(&format!(r#"xmlns:w15="{W15_PREVIEW}""#)));
    assert!(serialized.contains(r#"mc:Ignorable="w14 w15""#));
  }

  #[test]
  fn word_document_deduplicates_canonical_mc_namespace() {
    const W: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    const MC: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";
    let xml = format!(
      r#"<w:document xmlns:w="{W}" xmlns:ve="{MC}" xmlns:mc="{MC}" ve:Ignorable="w14"><w:body/></w:document>"#,
    );

    let serialized = serialize_word_document(&xml);
    assert_eq!(serialized.matches("xmlns:mc=").count(), 1);
    assert!(serialized.contains(&format!(r#"xmlns:ve="{MC}""#)));
    assert!(serialized.contains(r#"mc:Ignorable="w14""#));
  }

  #[test]
  fn word_document_does_not_rewrite_alias_with_unrelated_uri() {
    const W: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    const MC: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";
    let xml = format!(
      r#"<w:document xmlns:w="{W}" xmlns:ns1="urn:not-mc" xmlns:mc="{MC}" ns1:Ignorable="bad" mc:Ignorable="w14"><w:body/></w:document>"#,
    );

    let serialized = serialize_word_document(&xml);
    assert!(serialized.contains(r#"xmlns:ns1="urn:not-mc""#));
    assert_eq!(serialized.matches("xmlns:mc=").count(), 1);
    assert!(!serialized.contains("bad"));
    assert!(serialized.contains(r#"mc:Ignorable="w14""#));
  }

  #[test]
  fn read_context_resolves_root_and_current_element_attribute_aliases() {
    const W: &[u8] = b"http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    let xml = br#"<root xmlns:n="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><n:p n:rsidR="0001"/><n:p xmlns:n="urn:not-word" n:rsidR="0002"/><sheet d3p1:id="rId1" xmlns:d3p1="http://schemas.openxmlformats.org/officeDocument/2006/relationships"/></root>"#;
    let mut reader = from_bytes_inner(xml);
    let mut context = ReadContext::default();
    const W_TARGETS: &[AttributeQNameTarget] = &[AttributeQNameTarget::new(
      b"rsidR",
      crate::namespaces::XmlKnownNamespace::W,
      b"w:rsidR",
    )];
    const R_TARGETS: &[AttributeQNameTarget] = &[AttributeQNameTarget::new(
      b"id",
      crate::namespaces::XmlKnownNamespace::R,
      b"r:id",
    )];

    let PayloadEvent::Start(root, false) = reader.next().unwrap() else {
      panic!("expected root");
    };
    context.enter_root(&root, false, reader.decoder()).unwrap();
    let first = reader.next().unwrap();
    let PayloadEvent::Start(first, true) = first else {
      panic!("expected first empty paragraph");
    };
    assert!(context.attribute_qname_has_namespace(b"n:rsidR", W));
    assert_eq!(
      context.resolve_attribute_key(&first, b"n:rsidR", W_TARGETS),
      b"w:rsidR"
    );

    let shadowed = reader.next().unwrap();
    let PayloadEvent::Start(shadowed, true) = shadowed else {
      panic!("expected shadowed empty paragraph");
    };
    assert!(context.attribute_qname_has_namespace(b"n:rsidR", W));
    assert_eq!(
      context.resolve_attribute_key(&shadowed, b"n:rsidR", W_TARGETS),
      b""
    );

    let local = reader.next().unwrap();
    let PayloadEvent::Start(local, true) = local else {
      panic!("expected locally declared relationship attribute");
    };
    assert_eq!(
      context.resolve_attribute_key(&local, b"d3p1:id", R_TARGETS),
      b"r:id"
    );
  }

  #[test]
  fn read_context_normalizes_strict_and_static_beta_namespaces() {
    const W: &[u8] = b"http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    const W14: &[u8] = b"http://schemas.microsoft.com/office/word/2010/wordml";
    const W14_BETA_2007: &[u8] = b"http://schemas.microsoft.com/office/word/2007/5/30/wordml";
    const W14_BETA_2008: &[u8] = b"http://schemas.microsoft.com/office/word/2008/9/12/wordml";
    const W14_BETA_2009: &[u8] = b"http://schemas.microsoft.com/office/word/2009/2/wordml";
    let xml = br#"<root xmlns:s="http://purl.oclc.org/ooxml/wordprocessingml/main" xmlns:b7="http://schemas.microsoft.com/office/word/2007/5/30/wordml" xmlns:b8="http://schemas.microsoft.com/office/word/2008/9/12/wordml" xmlns:w14="http://schemas.microsoft.com/office/word/2009/2/wordml"><s:p s:rsidR="0001" b7:paraId="00000001" b8:textId="00000002" w14:editId="00000003"/></root>"#;
    let mut reader = from_bytes_inner(xml);
    let mut context = ReadContext::default();

    let PayloadEvent::Start(root, false) = reader.next().unwrap() else {
      panic!("expected root");
    };
    context.enter_root(&root, false, reader.decoder()).unwrap();
    let child = reader.next().unwrap();
    let PayloadEvent::Start(_child, true) = child else {
      panic!("expected empty paragraph");
    };
    assert!(context.attribute_qname_has_namespace(b"s:rsidR", W));
    assert!(context.attribute_qname_has_namespace(b"b7:paraId", W14));
    assert!(context.attribute_qname_has_namespace(b"b8:textId", W14));
    assert!(context.attribute_qname_has_namespace(b"w14:paraId", W14));
    assert_eq!(
      crate::namespaces::XmlKnownNamespace::from_uri_bytes(W14_BETA_2007),
      Some(crate::namespaces::XmlKnownNamespace::W14Preview2007)
    );
    assert_eq!(
      crate::namespaces::XmlKnownNamespace::from_uri_bytes(W14_BETA_2008),
      Some(crate::namespaces::XmlKnownNamespace::W14Preview2008)
    );
    assert_eq!(
      crate::namespaces::XmlKnownNamespace::from_uri_bytes(W14_BETA_2009),
      Some(crate::namespaces::XmlKnownNamespace::W14Preview)
    );
    assert_eq!(
      crate::namespaces::XmlKnownNamespace::from_compatible_uri_bytes(W14_BETA_2009),
      Some(crate::namespaces::XmlKnownNamespace::W14Preview)
    );
    assert_eq!(
      crate::namespaces::XmlKnownNamespace::W14Preview2007.schema_namespace(),
      crate::namespaces::XmlKnownNamespace::W14
    );
    assert_eq!(
      crate::namespaces::XmlKnownNamespace::W14Preview2008.schema_namespace(),
      crate::namespaces::XmlKnownNamespace::W14
    );
    assert_eq!(
      crate::namespaces::XmlKnownNamespace::W14Preview.schema_namespace(),
      crate::namespaces::XmlKnownNamespace::W14
    );
    assert_eq!(
      crate::namespaces::XmlKnownNamespace::W15Preview.schema_namespace(),
      crate::namespaces::XmlKnownNamespace::W15
    );
    assert_eq!(
      crate::namespaces::XmlKnownNamespace::from_prefix_bytes(b"w14"),
      Some(crate::namespaces::XmlKnownNamespace::W14)
    );
    assert_eq!(
      crate::namespaces::XmlKnownNamespace::from_prefix_bytes(b"w15"),
      Some(crate::namespaces::XmlKnownNamespace::W15)
    );
    assert_eq!(
      crate::namespaces::XmlKnownNamespace::from_uri_bytes(
        b"http://www.w3.org/2001/XMLSchema-instance"
      ),
      Some(crate::namespaces::XmlKnownNamespace::Xsi)
    );
    #[cfg(feature = "mce")]
    assert_eq!(
      crate::namespaces::minimum_version_by_uri(W14_BETA_2007),
      Some(crate::sdk::FileFormatVersion::Office2010)
    );
    #[cfg(feature = "mce")]
    assert_eq!(
      crate::namespaces::minimum_version_by_uri(W14_BETA_2008),
      Some(crate::sdk::FileFormatVersion::Office2010)
    );
    #[cfg(feature = "mce")]
    assert_eq!(
      crate::namespaces::minimum_version_by_uri(W14_BETA_2009),
      Some(crate::sdk::FileFormatVersion::Office2010)
    );
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

  #[cfg(feature = "parts")]
  #[test]
  fn strict_relationship_aliases_cover_non_identical_standard_suffixes() {
    assert!(relationship_type_matches_bytes(
      b"http://purl.oclc.org/ooxml/officeDocument/relationships/extendedProperties",
      REL_EXTENDED_PROPERTIES,
    ));
    assert!(relationship_type_matches_bytes(
      b"http://purl.oclc.org/ooxml/officeDocument/relationships/customProperties",
      b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties",
    ));
    assert!(relationship_type_matches_bytes(
      STRICT_REL_THUMBNAIL,
      REL_THUMBNAIL,
    ));
    assert!(relationship_type_matches_bytes(
      b"http://purl.oclc.org/ooxml/officeDocument/relationships/worksheet",
      b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet",
    ));
    assert!(!relationship_type_matches_bytes(
      b"http://purl.oclc.org/ooxml/officeDocument/relationships/extendedProperty",
      REL_EXTENDED_PROPERTIES,
    ));
  }
}
