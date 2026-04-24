use std::collections::HashMap;
use std::io::{Read, Seek};

use crate::schemas::opc_content_types::{Types, TypesChoice};
use crate::schemas::opc_relationships::{Relationship, Relationships, TargetMode};

use super::{
  SdkError, part_relationships_path, resolve_relationship_target_path, resolve_zip_file_path,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PackageOpenMode {
  #[default]
  Eager,
  Lazy,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PartId(u32);

impl PartId {
  #[inline]
  pub const fn from_index(index: usize) -> Self {
    Self(index as u32)
  }

  #[inline]
  pub const fn index(self) -> usize {
    self.0 as usize
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum StoredPartDataKind {
  Xml,
  Text,
  #[default]
  Binary,
}

#[derive(Clone, Debug)]
pub enum StoredPartData {
  Raw {
    kind: StoredPartDataKind,
    bytes: Vec<u8>,
  },
}

impl StoredPartData {
  #[inline]
  pub fn kind(&self) -> StoredPartDataKind {
    match self {
      Self::Raw { kind, .. } => *kind,
    }
  }

  #[inline]
  pub fn bytes(&self) -> &[u8] {
    match self {
      Self::Raw { bytes, .. } => bytes,
    }
  }
}

#[derive(Clone, Debug)]
pub struct RelationshipInfo {
  id: Box<str>,
  relationship_type: Box<str>,
  target: Box<str>,
  target_mode: Option<TargetMode>,
  target_kind: RelationshipTargetKind,
  target_part_id: Option<PartId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationshipTargetKind {
  InternalPart,
  External,
  Null,
  Missing,
}

impl RelationshipInfo {
  #[inline]
  pub fn id(&self) -> &str {
    &self.id
  }

  #[inline]
  pub fn relationship_type(&self) -> &str {
    &self.relationship_type
  }

  #[inline]
  pub fn target(&self) -> &str {
    &self.target
  }

  #[inline]
  pub fn target_mode(&self) -> TargetMode {
    self.target_mode.unwrap_or(TargetMode::Internal)
  }

  #[inline]
  pub fn target_kind(&self) -> RelationshipTargetKind {
    self.target_kind
  }

  #[inline]
  pub fn target_part_id(&self) -> Option<PartId> {
    self.target_part_id
  }

  fn to_relationship(&self) -> Relationship {
    Relationship {
      id: self.id().to_string(),
      r#type: self.relationship_type().to_string(),
      target: self.target().to_string(),
      target_mode: self.target_mode,
    }
  }
}

#[derive(Clone, Debug, Default)]
pub struct RelationshipSet {
  relationships: Vec<RelationshipInfo>,
  by_id: HashMap<Box<str>, usize>,
}

impl RelationshipSet {
  pub const HYPERLINK_RELATIONSHIP_TYPE: &'static str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink";
  pub const AUDIO_REFERENCE_RELATIONSHIP_TYPE: &'static str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio";
  pub const MEDIA_REFERENCE_RELATIONSHIP_TYPE: &'static str =
    "http://schemas.microsoft.com/office/2007/relationships/media";
  pub const VIDEO_REFERENCE_RELATIONSHIP_TYPE: &'static str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video";

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.relationships.is_empty()
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.relationships.len()
  }

  #[inline]
  pub fn iter(&self) -> impl Iterator<Item = &RelationshipInfo> {
    self.relationships.iter()
  }

  #[inline]
  pub fn get(&self, relationship_id: &str) -> Option<&RelationshipInfo> {
    self
      .by_id
      .get(relationship_id)
      .and_then(|index| self.relationships.get(*index))
  }

  #[inline]
  pub fn contains_id(&self, relationship_id: &str) -> bool {
    self.by_id.contains_key(relationship_id)
  }

  #[inline]
  pub fn by_relationship_type(
    &self,
    relationship_type: &str,
  ) -> impl Iterator<Item = &RelationshipInfo> {
    self.relationships.iter().filter(move |relationship| {
      super::relationship_type_matches(relationship.relationship_type(), relationship_type)
    })
  }

  #[inline]
  pub fn part_relationships(&self) -> impl Iterator<Item = &RelationshipInfo> {
    self
      .relationships
      .iter()
      .filter(|relationship| relationship.target_kind() == RelationshipTargetKind::InternalPart)
  }

  #[inline]
  pub fn external_relationships(&self) -> impl Iterator<Item = &RelationshipInfo> {
    self.relationships.iter().filter(|relationship| {
      relationship.target_kind() == RelationshipTargetKind::External
        && !super::relationship_type_matches(
          relationship.relationship_type(),
          Self::HYPERLINK_RELATIONSHIP_TYPE,
        )
    })
  }

  #[inline]
  pub fn hyperlink_relationships(&self) -> impl Iterator<Item = &RelationshipInfo> {
    self.by_relationship_type(Self::HYPERLINK_RELATIONSHIP_TYPE)
  }

  #[inline]
  pub fn data_part_reference_relationships(&self) -> impl Iterator<Item = &RelationshipInfo> {
    self.relationships.iter().filter(|relationship| {
      super::relationship_type_matches(
        relationship.relationship_type(),
        Self::AUDIO_REFERENCE_RELATIONSHIP_TYPE,
      ) || super::relationship_type_matches(
        relationship.relationship_type(),
        Self::MEDIA_REFERENCE_RELATIONSHIP_TYPE,
      ) || super::relationship_type_matches(
        relationship.relationship_type(),
        Self::VIDEO_REFERENCE_RELATIONSHIP_TYPE,
      )
    })
  }

  #[inline]
  pub fn first_target_part_by_relationship_type(&self, relationship_type: &str) -> Option<PartId> {
    self.relationships.iter().find_map(|relationship| {
      super::relationship_type_matches(relationship.relationship_type(), relationship_type)
        .then(|| relationship.target_part_id())
        .flatten()
    })
  }

  pub(crate) fn to_relationships(&self) -> Relationships {
    Relationships {
      xmlns: vec![super::XmlNamespaceDecl::new(
        "",
        "http://schemas.openxmlformats.org/package/2006/relationships",
      )],
      relationship: self
        .relationships
        .iter()
        .map(RelationshipInfo::to_relationship)
        .collect(),
      ..Default::default()
    }
  }

  fn from_relationships(
    relationships: Option<Relationships>,
    source_path: &str,
    by_path: &HashMap<Box<str>, PartId>,
  ) -> Self {
    let Some(relationships) = relationships else {
      return Self::default();
    };

    let source_parent_path = super::parent_zip_path(source_path);
    let mut set = Self {
      relationships: Vec::with_capacity(relationships.relationship.len()),
      by_id: HashMap::with_capacity(relationships.relationship.len()),
    };

    for relationship in relationships.relationship {
      let info = relationship_info(relationship, &source_parent_path, by_path);
      let index = set.relationships.len();
      set.by_id.insert(info.id.clone(), index);
      set.relationships.push(info);
    }

    set
  }
}

#[derive(Clone, Debug)]
pub struct StoredPart {
  path: Box<str>,
  content_type: Box<str>,
  relationship_type: Option<Box<str>>,
  relationships: RelationshipSet,
  data: StoredPartData,
}

impl StoredPart {
  #[inline]
  pub fn path(&self) -> &str {
    &self.path
  }

  #[inline]
  pub fn content_type(&self) -> &str {
    &self.content_type
  }

  #[inline]
  pub fn relationship_type(&self) -> Option<&str> {
    self.relationship_type.as_deref()
  }

  #[inline]
  pub fn relationships(&self) -> &RelationshipSet {
    &self.relationships
  }

  #[inline]
  pub fn data(&self) -> &StoredPartData {
    &self.data
  }
}

#[derive(Clone, Debug)]
pub struct SdkPackageStorage {
  content_types: Types,
  package_relationships: RelationshipSet,
  parts: Vec<StoredPart>,
  by_path: HashMap<Box<str>, PartId>,
  open_mode: PackageOpenMode,
}

impl SdkPackageStorage {
  pub fn open<R: Read + Seek>(reader: R, open_mode: PackageOpenMode) -> Result<Self, SdkError> {
    let mut archive = zip::ZipArchive::new(reader)?;
    let content_types = read_content_types(&mut archive)?;
    let mut raw_parts = read_raw_parts(&mut archive, &content_types)?;
    let mut by_path = HashMap::with_capacity(raw_parts.len());

    for (index, raw_part) in raw_parts.iter().enumerate() {
      by_path.insert(raw_part.path.clone(), PartId::from_index(index));
    }

    let package_relationships = read_relationships(&mut archive, "_rels/.rels")?;
    let package_relationships =
      RelationshipSet::from_relationships(package_relationships, "", &by_path);

    let mut part_relationships = Vec::with_capacity(raw_parts.len());
    for raw_part in &raw_parts {
      let rels_path = part_relationships_path(&raw_part.path);
      let relationships = read_relationships(&mut archive, &rels_path)?;
      part_relationships.push(RelationshipSet::from_relationships(
        relationships,
        &raw_part.path,
        &by_path,
      ));
    }

    let relationship_types =
      relationship_types_by_part(&package_relationships, &part_relationships)?;

    let mut parts = Vec::with_capacity(raw_parts.len());
    for (index, (raw_part, relationships)) in
      raw_parts.drain(..).zip(part_relationships).enumerate()
    {
      parts.push(StoredPart {
        path: raw_part.path,
        content_type: raw_part.content_type,
        relationship_type: relationship_types
          .get(&PartId::from_index(index))
          .map(|relationship_type| relationship_type.clone().into_boxed_str()),
        relationships,
        data: StoredPartData::Raw {
          kind: raw_part.data_kind,
          bytes: raw_part.bytes,
        },
      });
    }

    Ok(Self {
      content_types,
      package_relationships,
      parts,
      by_path,
      open_mode,
    })
  }

  #[inline]
  pub fn content_types(&self) -> &Types {
    &self.content_types
  }

  #[inline]
  pub fn package_relationships(&self) -> &RelationshipSet {
    &self.package_relationships
  }

  #[inline]
  pub fn open_mode(&self) -> PackageOpenMode {
    self.open_mode
  }

  #[inline]
  pub fn parts(&self) -> &[StoredPart] {
    &self.parts
  }

  #[inline]
  pub fn part(&self, part_id: PartId) -> Option<&StoredPart> {
    self.parts.get(part_id.index())
  }

  #[inline]
  pub fn part_by_path(&self, path: &str) -> Option<(PartId, &StoredPart)> {
    let part_id = *self.by_path.get(path)?;
    self.part(part_id).map(|part| (part_id, part))
  }

  #[inline]
  pub fn relationships(&self, part_id: PartId) -> Option<&RelationshipSet> {
    self.part(part_id).map(StoredPart::relationships)
  }

  #[inline]
  pub fn target_part_id(&self, source_part_id: PartId, relationship_id: &str) -> Option<PartId> {
    self
      .relationships(source_part_id)?
      .get(relationship_id)?
      .target_part_id()
  }
}

struct RawPart {
  path: Box<str>,
  content_type: Box<str>,
  data_kind: StoredPartDataKind,
  bytes: Vec<u8>,
}

fn relationship_types_by_part(
  package_relationships: &RelationshipSet,
  part_relationships: &[RelationshipSet],
) -> Result<HashMap<PartId, String>, SdkError> {
  let mut relationship_types: HashMap<PartId, String> = HashMap::new();

  for relationship_set in std::iter::once(package_relationships).chain(part_relationships) {
    for relationship in relationship_set.iter() {
      let Some(part_id) = relationship.target_part_id() else {
        continue;
      };
      if let Some(existing) = relationship_types.get(&part_id) {
        if existing != relationship.relationship_type()
          && !super::relationship_type_matches_alias(existing, relationship.relationship_type())
          && !super::relationship_type_matches_alias(relationship.relationship_type(), existing)
          && !data_part_reference_relationship_types_are_compatible(
            existing,
            relationship.relationship_type(),
          )
        {
          return Err(SdkError::CommonError(format!(
            "same part {:?} is referenced by different relationship types: {existing} and {}",
            part_id,
            relationship.relationship_type(),
          )));
        }
      } else {
        relationship_types.insert(part_id, relationship.relationship_type().to_string());
      }
    }
  }

  Ok(relationship_types)
}

fn data_part_reference_relationship_types_are_compatible(left: &str, right: &str) -> bool {
  is_data_part_reference_relationship_type(left) && is_data_part_reference_relationship_type(right)
}

fn is_data_part_reference_relationship_type(relationship_type: &str) -> bool {
  matches!(
    relationship_type,
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio"
      | "http://schemas.microsoft.com/office/2007/relationships/media"
      | "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video"
  )
}

fn read_content_types<R: Read + Seek>(archive: &mut zip::ZipArchive<R>) -> Result<Types, SdkError> {
  let mut entry = archive.by_name("[Content_Types].xml")?;
  let mut bytes = Vec::with_capacity(entry.size() as usize);
  entry.read_to_end(&mut bytes)?;
  Types::from_bytes(&bytes)
}

fn read_raw_parts<R: Read + Seek>(
  archive: &mut zip::ZipArchive<R>,
  content_types: &Types,
) -> Result<Vec<RawPart>, SdkError> {
  let mut parts = Vec::new();

  for index in 0..archive.len() {
    let mut entry = archive.by_index(index)?;
    if entry.is_dir() {
      continue;
    }

    let path = resolve_zip_file_path(entry.name());
    if path == "[Content_Types].xml" || is_relationships_part_path(&path) {
      continue;
    }

    let content_type = content_type_for_part(content_types, &path).unwrap_or_default();
    let data_kind = data_kind_for_content_type(content_type);
    let mut bytes = Vec::with_capacity(entry.size() as usize);
    entry.read_to_end(&mut bytes)?;
    parts.push(RawPart {
      path: path.into_boxed_str(),
      content_type: content_type.into(),
      data_kind,
      bytes,
    });
  }

  parts.sort_by(|left, right| left.path.cmp(&right.path));
  Ok(parts)
}

fn read_relationships<R: Read + Seek>(
  archive: &mut zip::ZipArchive<R>,
  path: &str,
) -> Result<Option<Relationships>, SdkError> {
  let Some(index) = archive.index_for_name(path) else {
    return Ok(None);
  };

  let mut entry = archive.by_index(index)?;
  let mut bytes = Vec::with_capacity(entry.size() as usize);
  entry.read_to_end(&mut bytes)?;
  Relationships::from_bytes(&bytes).map(Some)
}

fn relationship_info(
  relationship: Relationship,
  source_parent_path: &str,
  by_path: &HashMap<Box<str>, PartId>,
) -> RelationshipInfo {
  let target_mode = relationship.target_mode;
  let effective_target_mode = target_mode.unwrap_or(TargetMode::Internal);
  let (target_kind, target_part_id) = if matches!(effective_target_mode, TargetMode::Internal) {
    if relationship.target.eq_ignore_ascii_case("NULL") {
      (RelationshipTargetKind::Null, None)
    } else {
      let target_path = resolve_zip_file_path(&resolve_relationship_target_path(
        source_parent_path,
        &relationship.target,
      ));
      match by_path.get(target_path.as_str()).copied() {
        Some(part_id) => (RelationshipTargetKind::InternalPart, Some(part_id)),
        None => (RelationshipTargetKind::Missing, None),
      }
    }
  } else {
    (RelationshipTargetKind::External, None)
  };

  RelationshipInfo {
    id: relationship.id.into_boxed_str(),
    relationship_type: relationship.r#type.into_boxed_str(),
    target: relationship.target.into_boxed_str(),
    target_mode,
    target_kind,
    target_part_id,
  }
}

fn content_type_for_part<'a>(content_types: &'a Types, path: &str) -> Option<&'a str> {
  let normalized_part_name = format!("/{path}");
  let extension = path.rsplit_once('.').map(|(_, extension)| extension);
  let mut default_content_type = None;

  for child in &content_types.xml_children {
    match child {
      TypesChoice::Override(override_type) if override_type.part_name == normalized_part_name => {
        return Some(override_type.content_type.as_str());
      }
      TypesChoice::Default(default_type) if extension == Some(default_type.extension.as_str()) => {
        default_content_type = Some(default_type.content_type.as_str());
      }
      _ => {}
    }
  }

  default_content_type
}

fn data_kind_for_content_type(content_type: &str) -> StoredPartDataKind {
  if content_type.ends_with("+xml")
    || content_type == "application/xml"
    || content_type == "text/xml"
  {
    StoredPartDataKind::Xml
  } else if content_type.starts_with("text/") {
    StoredPartDataKind::Text
  } else {
    StoredPartDataKind::Binary
  }
}

fn is_relationships_part_path(path: &str) -> bool {
  path == "_rels/.rels" || path.contains("/_rels/") && path.ends_with(".rels")
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::{Cursor, Write};

  #[test]
  fn storage_resolves_package_relationship_target_part_id() {
    let mut buffer = Cursor::new(Vec::new());
    {
      let mut zip = zip::ZipWriter::new(&mut buffer);
      let options = zip::write::SimpleFileOptions::default();

      zip.start_file("[Content_Types].xml", options).unwrap();
      zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>"#,
      )
      .unwrap();

      zip.add_directory("_rels", options).unwrap();
      zip.start_file("_rels/.rels", options).unwrap();
      zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>"#,
      )
      .unwrap();

      zip.add_directory("word", options).unwrap();
      zip.start_file("word/document.xml", options).unwrap();
      zip.write_all(br#"<w:document xmlns:w="w"/>"#).unwrap();
      zip.finish().unwrap();
    }

    buffer.set_position(0);
    let storage = SdkPackageStorage::open(buffer, PackageOpenMode::Lazy).unwrap();
    let relationship = storage.package_relationships().get("rId1").unwrap();
    let part_id = relationship.target_part_id().unwrap();
    let part = storage.part(part_id).unwrap();

    assert_eq!(part.path(), "word/document.xml");
    assert_eq!(
      part.relationship_type(),
      Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument")
    );
    assert_eq!(
      relationship.target_kind(),
      RelationshipTargetKind::InternalPart
    );
    assert_eq!(part.data().kind(), StoredPartDataKind::Xml);
  }
}
