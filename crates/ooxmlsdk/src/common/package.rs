use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewPartDescriptor {
  pub relationship_type: Cow<'static, str>,
  pub content_type: Cow<'static, str>,
  pub path_prefix: &'static str,
  pub target_name: &'static str,
  pub extension: Cow<'static, str>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NewPartTargetMode {
  Fixed,
  #[default]
  Indexed,
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

  #[inline]
  pub fn bytes_mut(&mut self) -> &mut Vec<u8> {
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
  fn internal_part(
    id: String,
    relationship_type: String,
    target: String,
    target_part_id: PartId,
  ) -> Self {
    Self {
      id: id.into_boxed_str(),
      relationship_type: relationship_type.into_boxed_str(),
      target: target.into_boxed_str(),
      target_mode: None,
      target_kind: RelationshipTargetKind::InternalPart,
      target_part_id: Some(target_part_id),
    }
  }

  fn external(
    id: String,
    relationship_type: String,
    target: String,
    target_mode: Option<TargetMode>,
  ) -> Self {
    Self {
      id: id.into_boxed_str(),
      relationship_type: relationship_type.into_boxed_str(),
      target: target.into_boxed_str(),
      target_mode,
      target_kind: RelationshipTargetKind::External,
      target_part_id: None,
    }
  }

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

  pub fn next_relationship_id(&self) -> String {
    let next = self
      .relationships
      .iter()
      .filter_map(|relationship| relationship.id().strip_prefix("rId"))
      .filter_map(|suffix| suffix.parse::<u32>().ok())
      .max()
      .unwrap_or_default()
      + 1;
    format!("rId{next}")
  }

  pub fn add_external_relationship(
    &mut self,
    relationship_id: impl Into<String>,
    relationship_type: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<&RelationshipInfo, SdkError> {
    self.push_relationship(RelationshipInfo::external(
      relationship_id.into(),
      relationship_type.into(),
      target.into(),
      Some(TargetMode::External),
    ))
  }

  pub fn add_hyperlink_relationship(
    &mut self,
    relationship_id: impl Into<String>,
    target: impl Into<String>,
  ) -> Result<&RelationshipInfo, SdkError> {
    self.add_external_relationship(relationship_id, Self::HYPERLINK_RELATIONSHIP_TYPE, target)
  }

  pub fn add_internal_part_relationship(
    &mut self,
    relationship_id: impl Into<String>,
    relationship_type: impl Into<String>,
    target: impl Into<String>,
    target_part_id: PartId,
  ) -> Result<&RelationshipInfo, SdkError> {
    self.push_relationship(RelationshipInfo::internal_part(
      relationship_id.into(),
      relationship_type.into(),
      target.into(),
      target_part_id,
    ))
  }

  pub fn remove(&mut self, relationship_id: &str) -> Option<RelationshipInfo> {
    let index = *self.by_id.get(relationship_id)?;
    let removed = self.relationships.remove(index);
    self.rebuild_index();
    Some(removed)
  }

  pub fn change_relationship_id(
    &mut self,
    relationship_id: &str,
    new_relationship_id: impl Into<String>,
  ) -> Result<(), SdkError> {
    let new_relationship_id = new_relationship_id.into();
    if relationship_id == new_relationship_id {
      return Ok(());
    }
    if self.contains_id(&new_relationship_id) {
      return Err(SdkError::CommonError(format!(
        "relationship id {new_relationship_id} already exists"
      )));
    }

    let Some(index) = self.by_id.get(relationship_id).copied() else {
      return Err(SdkError::CommonError(format!(
        "relationship id {relationship_id} does not exist"
      )));
    };

    self.relationships[index].id = new_relationship_id.into_boxed_str();
    self.rebuild_index();
    Ok(())
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
      set.push_relationship_unchecked(info);
    }

    set
  }

  fn push_relationship(
    &mut self,
    relationship: RelationshipInfo,
  ) -> Result<&RelationshipInfo, SdkError> {
    if self.contains_id(relationship.id()) {
      return Err(SdkError::CommonError(format!(
        "relationship id {} already exists",
        relationship.id()
      )));
    }
    self.push_relationship_unchecked(relationship);
    Ok(self.relationships.last().expect("pushed relationship"))
  }

  fn push_relationship_unchecked(&mut self, relationship: RelationshipInfo) {
    let index = self.relationships.len();
    self.by_id.insert(relationship.id.clone(), index);
    self.relationships.push(relationship);
  }

  fn rebuild_index(&mut self) {
    self.by_id.clear();
    self.by_id.reserve(self.relationships.len());
    for (index, relationship) in self.relationships.iter().enumerate() {
      self.by_id.insert(relationship.id.clone(), index);
    }
  }
}

#[derive(Clone, Debug)]
pub struct StoredPart {
  path: Box<str>,
  content_type: Box<str>,
  relationship_type: Option<Box<str>>,
  relationships: RelationshipSet,
  data: StoredPartData,
  deleted: bool,
}

impl StoredPart {
  #[inline]
  pub fn is_deleted(&self) -> bool {
    self.deleted
  }

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
  pub fn relationships_mut(&mut self) -> &mut RelationshipSet {
    &mut self.relationships
  }

  #[inline]
  pub fn data(&self) -> &StoredPartData {
    &self.data
  }

  #[inline]
  pub fn data_mut(&mut self) -> &mut StoredPartData {
    &mut self.data
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
        deleted: false,
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
  pub fn package_relationships_mut(&mut self) -> &mut RelationshipSet {
    &mut self.package_relationships
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
    self
      .parts
      .get(part_id.index())
      .filter(|part| !part.is_deleted())
  }

  #[inline]
  pub fn part_mut(&mut self, part_id: PartId) -> Option<&mut StoredPart> {
    self
      .parts
      .get_mut(part_id.index())
      .filter(|part| !part.is_deleted())
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
  pub fn relationships_mut(&mut self, part_id: PartId) -> Option<&mut RelationshipSet> {
    self.part_mut(part_id).map(StoredPart::relationships_mut)
  }

  #[inline]
  pub fn target_part_id(&self, source_part_id: PartId, relationship_id: &str) -> Option<PartId> {
    self
      .relationships(source_part_id)?
      .get(relationship_id)?
      .target_part_id()
  }

  pub fn delete_package_part(&mut self, relationship_id: &str) -> Result<bool, SdkError> {
    let Some(target_part_id) = self
      .package_relationships
      .get(relationship_id)
      .and_then(RelationshipInfo::target_part_id)
    else {
      return Ok(false);
    };

    self.package_relationships.remove(relationship_id);
    if !self.is_part_reachable(target_part_id) {
      self.delete_unreachable_part_tree(target_part_id);
    }
    Ok(true)
  }

  pub fn delete_child_part(
    &mut self,
    source_part_id: PartId,
    relationship_id: &str,
  ) -> Result<bool, SdkError> {
    let Some(target_part_id) = self.target_part_id(source_part_id, relationship_id) else {
      return Ok(false);
    };

    let relationships = self.relationships_mut(source_part_id).ok_or_else(|| {
      SdkError::CommonError(format!(
        "part id {source_part_id:?} is not present in package storage"
      ))
    })?;
    relationships.remove(relationship_id);
    if !self.is_part_reachable(target_part_id) {
      self.delete_unreachable_part_tree(target_part_id);
    }
    Ok(true)
  }

  pub fn add_package_relationship_to_part(
    &mut self,
    relationship_id: impl Into<String>,
    target_part_id: PartId,
  ) -> Result<String, SdkError> {
    let relationship_id = relationship_id.into();
    if let Some(existing_relationship_id) =
      self.existing_relationship_id_for_target(None, target_part_id)
    {
      if existing_relationship_id == relationship_id {
        return Ok(existing_relationship_id);
      }
      return Err(SdkError::CommonError(format!(
        "part id {target_part_id:?} is already referenced by relationship id {existing_relationship_id}"
      )));
    }

    let (relationship_type, target) = {
      let target_part = self.part(target_part_id).ok_or_else(|| {
        SdkError::CommonError(format!(
          "part id {target_part_id:?} is not present in package storage"
        ))
      })?;
      let relationship_type = target_part.relationship_type().ok_or_else(|| {
        SdkError::CommonError(format!(
          "part id {target_part_id:?} does not have a relationship type"
        ))
      })?;
      (
        relationship_type.to_string(),
        target_part.path().to_string(),
      )
    };

    self.package_relationships.add_internal_part_relationship(
      relationship_id.clone(),
      relationship_type,
      target,
      target_part_id,
    )?;
    Ok(relationship_id)
  }

  pub fn add_child_relationship_to_part(
    &mut self,
    source_part_id: PartId,
    relationship_id: impl Into<String>,
    target_part_id: PartId,
  ) -> Result<String, SdkError> {
    let relationship_id = relationship_id.into();
    if let Some(existing_relationship_id) =
      self.existing_relationship_id_for_target(Some(source_part_id), target_part_id)
    {
      if existing_relationship_id == relationship_id {
        return Ok(existing_relationship_id);
      }
      return Err(SdkError::CommonError(format!(
        "part id {target_part_id:?} is already referenced by relationship id {existing_relationship_id}"
      )));
    }

    let (relationship_type, relationship_target) = {
      let source_part_path = self
        .part(source_part_id)
        .ok_or_else(|| {
          SdkError::CommonError(format!(
            "part id {source_part_id:?} is not present in package storage"
          ))
        })?
        .path()
        .to_string();
      let target_part = self.part(target_part_id).ok_or_else(|| {
        SdkError::CommonError(format!(
          "part id {target_part_id:?} is not present in package storage"
        ))
      })?;
      let relationship_type = target_part.relationship_type().ok_or_else(|| {
        SdkError::CommonError(format!(
          "part id {target_part_id:?} does not have a relationship type"
        ))
      })?;
      (
        relationship_type.to_string(),
        relationship_target_from_source(&source_part_path, target_part.path()),
      )
    };

    self
      .relationships_mut(source_part_id)
      .expect("source part was already resolved")
      .add_internal_part_relationship(
        relationship_id.clone(),
        relationship_type,
        relationship_target,
        target_part_id,
      )?;
    Ok(relationship_id)
  }

  pub fn create_media_data_part(
    &mut self,
    content_type: impl Into<String>,
    extension: impl AsRef<str>,
  ) -> Result<PartId, SdkError> {
    let content_type = content_type.into();
    if content_type.is_empty() {
      return Err(SdkError::CommonError(
        "cannot add a media data part with an empty content type".to_string(),
      ));
    }
    let extension = normalized_part_extension(extension.as_ref());
    let path = self.next_data_part_path("media/media", &extension);
    let part_id = self.push_part(path, &content_type, None);
    Ok(part_id)
  }

  pub fn add_data_part_reference_relationship(
    &mut self,
    source_part_id: PartId,
    relationship_id: impl Into<String>,
    relationship_type: &str,
    target_part_id: PartId,
  ) -> Result<String, SdkError> {
    let relationship_id = relationship_id.into();
    let relationship_target = {
      let source_part_path = self
        .part(source_part_id)
        .ok_or_else(|| {
          SdkError::CommonError(format!(
            "part id {source_part_id:?} is not present in package storage"
          ))
        })?
        .path()
        .to_string();
      let target_part = self.part(target_part_id).ok_or_else(|| {
        SdkError::CommonError(format!(
          "part id {target_part_id:?} is not present in package storage"
        ))
      })?;
      relationship_target_from_source(&source_part_path, target_part.path())
    };

    self
      .relationships_mut(source_part_id)
      .expect("source part was already resolved")
      .add_internal_part_relationship(
        relationship_id.clone(),
        relationship_type,
        relationship_target,
        target_part_id,
      )?;
    Ok(relationship_id)
  }

  pub fn add_child_part(
    &mut self,
    source_part_id: PartId,
    relationship_id: impl Into<String>,
    descriptor: NewPartDescriptor,
  ) -> Result<PartId, SdkError> {
    if descriptor.relationship_type.is_empty() {
      return Err(SdkError::CommonError(
        "cannot add a part with an empty relationship type".to_string(),
      ));
    }
    if descriptor.content_type.is_empty() {
      return Err(SdkError::CommonError(
        "cannot add a part with an empty content type".to_string(),
      ));
    }
    if descriptor.target_name.is_empty() {
      return Err(SdkError::CommonError(
        "cannot add a part with an empty target name".to_string(),
      ));
    }

    let relationship_id = relationship_id.into();
    let source_part_path = self
      .part(source_part_id)
      .ok_or_else(|| {
        SdkError::CommonError(format!(
          "part id {source_part_id:?} is not present in package storage"
        ))
      })?
      .path()
      .to_string();
    if self
      .relationships(source_part_id)
      .is_some_and(|relationships| relationships.contains_id(&relationship_id))
    {
      return Err(SdkError::CommonError(format!(
        "relationship id {relationship_id} already exists"
      )));
    }

    let child_path = self.next_child_part_path(
      &source_part_path,
      descriptor.path_prefix,
      descriptor.target_name,
      descriptor.extension.as_ref(),
    );
    let relationship_target = relationship_target_from_source(&source_part_path, &child_path);
    let part_id = self.push_part(
      child_path,
      &descriptor.content_type,
      Some(descriptor.relationship_type.as_ref()),
    );
    self
      .relationships_mut(source_part_id)
      .expect("source part was already resolved")
      .add_internal_part_relationship(
        relationship_id,
        descriptor.relationship_type.as_ref(),
        relationship_target,
        part_id,
      )?;
    Ok(part_id)
  }

  pub fn add_package_part(
    &mut self,
    relationship_id: impl Into<String>,
    descriptor: NewPartDescriptor,
    target_mode: NewPartTargetMode,
  ) -> Result<PartId, SdkError> {
    if descriptor.relationship_type.is_empty() {
      return Err(SdkError::CommonError(
        "cannot add a part with an empty relationship type".to_string(),
      ));
    }
    if descriptor.content_type.is_empty() {
      return Err(SdkError::CommonError(
        "cannot add a part with an empty content type".to_string(),
      ));
    }
    if descriptor.target_name.is_empty() {
      return Err(SdkError::CommonError(
        "cannot add a part with an empty target name".to_string(),
      ));
    }

    let relationship_id = relationship_id.into();
    if self.package_relationships.contains_id(&relationship_id) {
      return Err(SdkError::CommonError(format!(
        "relationship id {relationship_id} already exists"
      )));
    }

    let part_path = self.package_part_path(&descriptor, target_mode)?;
    let part_id = self.push_part(
      part_path.clone(),
      &descriptor.content_type,
      Some(descriptor.relationship_type.as_ref()),
    );
    self.package_relationships.add_internal_part_relationship(
      relationship_id,
      descriptor.relationship_type.as_ref(),
      part_path,
      part_id,
    )?;
    Ok(part_id)
  }

  fn push_part(
    &mut self,
    path: String,
    content_type: &str,
    relationship_type: Option<&str>,
  ) -> PartId {
    let part_id = PartId::from_index(self.parts.len());
    let data_kind = data_kind_for_content_type(content_type);
    self.parts.push(StoredPart {
      path: path.clone().into_boxed_str(),
      content_type: content_type.into(),
      relationship_type: relationship_type.map(Into::into),
      relationships: RelationshipSet::default(),
      data: StoredPartData::Raw {
        kind: data_kind,
        bytes: Vec::new(),
      },
      deleted: false,
    });
    self.by_path.insert(path.clone().into_boxed_str(), part_id);
    self.add_content_type_override(&path, content_type);
    part_id
  }

  pub fn set_part_data(
    &mut self,
    part_id: PartId,
    data: impl Into<Vec<u8>>,
  ) -> Result<(), SdkError> {
    let part = self.part_mut(part_id).ok_or_else(|| {
      SdkError::CommonError(format!(
        "part id {part_id:?} is not present in package storage"
      ))
    })?;
    *part.data.bytes_mut() = data.into();
    Ok(())
  }

  pub fn feed_part_data<R: Read>(
    &mut self,
    part_id: PartId,
    reader: &mut R,
  ) -> Result<(), SdkError> {
    let part = self.part_mut(part_id).ok_or_else(|| {
      SdkError::CommonError(format!(
        "part id {part_id:?} is not present in package storage"
      ))
    })?;
    let bytes = part.data.bytes_mut();
    bytes.clear();
    reader.read_to_end(bytes)?;
    Ok(())
  }

  fn add_content_type_override(&mut self, path: &str, content_type: &str) {
    let part_name = format!("/{path}");
    if self.content_types.xml_children.iter().any(|child| {
      matches!(child, TypesChoice::Override(override_type) if override_type.part_name == part_name)
    }) {
      return;
    }

    self
      .content_types
      .xml_children
      .push(TypesChoice::Override(Box::new(
        crate::schemas::opc_content_types::Override {
          content_type: content_type.to_string(),
          part_name,
        },
      )));
  }

  fn remove_content_type_override(&mut self, path: &str) {
    let part_name = format!("/{path}");
    self.content_types.xml_children.retain(|child| {
      !matches!(child, TypesChoice::Override(override_type) if override_type.part_name == part_name)
    });
  }

  fn existing_relationship_id_for_target(
    &self,
    source_part_id: Option<PartId>,
    target_part_id: PartId,
  ) -> Option<String> {
    let relationships = match source_part_id {
      Some(source_part_id) => self.relationships(source_part_id)?,
      None => self.package_relationships(),
    };
    relationships.iter().find_map(|relationship| {
      (relationship.target_part_id() == Some(target_part_id)).then(|| relationship.id().to_string())
    })
  }

  fn is_part_reachable(&self, target_part_id: PartId) -> bool {
    let mut visited = HashSet::new();
    let mut stack: Vec<_> = self
      .package_relationships
      .part_relationships()
      .filter_map(RelationshipInfo::target_part_id)
      .collect();

    while let Some(part_id) = stack.pop() {
      if !visited.insert(part_id) {
        continue;
      }
      let Some(part) = self.part(part_id) else {
        continue;
      };
      if part_id == target_part_id {
        return true;
      }
      stack.extend(
        part
          .relationships()
          .part_relationships()
          .filter_map(RelationshipInfo::target_part_id),
      );
    }

    false
  }

  fn delete_unreachable_part_tree(&mut self, part_id: PartId) {
    let Some(part) = self.part(part_id) else {
      return;
    };
    let child_part_ids: Vec<_> = part
      .relationships()
      .part_relationships()
      .filter_map(RelationshipInfo::target_part_id)
      .collect();
    let path = part.path().to_string();

    if let Some(part) = self.parts.get_mut(part_id.index()) {
      part.deleted = true;
    }
    self.by_path.remove(path.as_str());
    self.remove_content_type_override(&path);

    for child_part_id in child_part_ids {
      if !self.is_part_reachable(child_part_id) {
        self.delete_unreachable_part_tree(child_part_id);
      }
    }
  }

  fn next_child_part_path(
    &self,
    source_part_path: &str,
    path_prefix: &str,
    target_name: &str,
    extension: &str,
  ) -> String {
    let directory_path = child_part_directory_path(source_part_path, path_prefix);
    let extension = if extension.is_empty() {
      ".xml"
    } else {
      extension
    };
    let extension = if extension.starts_with('.') {
      extension.to_string()
    } else {
      format!(".{extension}")
    };

    for index in 1.. {
      let path = if directory_path.is_empty() {
        format!("{target_name}{index}{extension}")
      } else {
        format!("{directory_path}{target_name}{index}{extension}")
      };
      if !self.by_path.contains_key(path.as_str()) {
        return path;
      }
    }

    unreachable!("usize iteration should always find a free part path")
  }

  fn package_part_path(
    &self,
    descriptor: &NewPartDescriptor,
    target_mode: NewPartTargetMode,
  ) -> Result<String, SdkError> {
    let directory_path = package_part_directory_path(descriptor.path_prefix);
    let extension = normalized_part_extension(descriptor.extension.as_ref());

    if matches!(target_mode, NewPartTargetMode::Fixed) {
      let path = if directory_path.is_empty() {
        format!("{}{}", descriptor.target_name, extension)
      } else {
        format!("{directory_path}{}{}", descriptor.target_name, extension)
      };
      if self.by_path.contains_key(path.as_str()) {
        return Err(SdkError::CommonError(format!(
          "part path {path} already exists"
        )));
      }
      return Ok(path);
    }

    for index in 1.. {
      let path = if directory_path.is_empty() {
        format!("{}{index}{}", descriptor.target_name, extension)
      } else {
        format!(
          "{directory_path}{}{index}{}",
          descriptor.target_name, extension
        )
      };
      if !self.by_path.contains_key(path.as_str()) {
        return Ok(path);
      }
    }

    unreachable!("usize iteration should always find a free part path")
  }

  fn next_data_part_path(&self, stem: &str, extension: &str) -> String {
    for index in 1.. {
      let path = format!("{stem}{index}{extension}");
      if !self.by_path.contains_key(path.as_str()) {
        return path;
      }
    }

    unreachable!("usize iteration should always find a free data part path")
  }
}

struct RawPart {
  path: Box<str>,
  content_type: Box<str>,
  data_kind: StoredPartDataKind,
  bytes: Vec<u8>,
}

fn child_part_directory_path(source_part_path: &str, path_prefix: &str) -> String {
  let source_parent_path = super::parent_zip_path(source_part_path);
  let mut path = if path_prefix.is_empty() || path_prefix == "." {
    source_parent_path
  } else if path_prefix.starts_with('/') {
    path_prefix.to_string()
  } else {
    let mut path = String::with_capacity(source_parent_path.len() + path_prefix.len() + 1);
    path.push_str(&source_parent_path);
    path.push_str(path_prefix);
    path
  };

  path = resolve_zip_file_path(&path);
  if !path.is_empty() && !path.ends_with('/') {
    path.push('/');
  }
  path
}

fn package_part_directory_path(path_prefix: &str) -> String {
  if path_prefix.is_empty() || path_prefix == "." {
    return String::new();
  }

  let mut path = resolve_zip_file_path(path_prefix);
  if !path.is_empty() && !path.ends_with('/') {
    path.push('/');
  }
  path
}

fn normalized_part_extension(extension: &str) -> String {
  let extension = if extension.is_empty() {
    ".xml"
  } else {
    extension
  };

  if extension.starts_with('.') {
    extension.to_string()
  } else {
    format!(".{extension}")
  }
}

fn relationship_target_from_source(source_part_path: &str, child_part_path: &str) -> String {
  let source_parent_path = super::parent_zip_path(source_part_path);
  if let Some(relative) = child_part_path.strip_prefix(&source_parent_path) {
    relative.to_string()
  } else {
    format!("/{child_part_path}")
  }
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
