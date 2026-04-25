//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtendedPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<Box<str>>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl ExtendedPart {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] = &[];
}
impl crate::sdk::SdkPartHandle for ExtendedPart {
  const CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] =
    Self::GENERATED_CHILD_DESCRIPTORS;
  const DESCRIPTOR: crate::sdk::PartDescriptor =
    crate::sdk::PartDescriptor::new("", "", "", "extendedPart", "");
  #[inline]
  fn from_part_id(part_id: crate::common::PartId) -> Self {
    Self {
      relationship_id: None,
      id: part_id,
      fallback_parts: Vec::new(),
      relationship_order: Vec::new(),
      data_part_reference_relationships: Vec::new(),
      reference_relationships: Vec::new(),
      raw_relationships: Vec::new(),
    }
  }
  #[inline]
  fn from_relationship_id(
    relationship_id: impl Into<String>,
    part_id: crate::common::PartId,
  ) -> Self {
    Self {
      relationship_id: Some(relationship_id.into()),
      id: part_id,
      fallback_parts: Vec::new(),
      relationship_order: Vec::new(),
      data_part_reference_relationships: Vec::new(),
      reference_relationships: Vec::new(),
      raw_relationships: Vec::new(),
    }
  }
  #[inline]
  fn part_id(&self) -> crate::common::PartId {
    self.id
  }
  #[inline]
  fn relationship_id(&self) -> Option<&str> {
    self.relationship_id.as_deref()
  }
  #[inline]
  fn from_part_id_with_relationships(
    storage: &crate::common::SdkPackageStorage,
    part_id: crate::common::PartId,
  ) -> Self {
    let mut visited = std::collections::HashSet::new();
    Self::from_part_id_with_relationships_visited(storage, part_id, &mut visited)
  }
  fn from_part_id_with_relationships_visited(
    storage: &crate::common::SdkPackageStorage,
    part_id: crate::common::PartId,
    visited: &mut std::collections::HashSet<crate::common::PartId>,
  ) -> Self {
    if !visited.insert(part_id) {
      return Self::from_part_id(part_id);
    }
    let mut part = Self::from_part_id(part_id);
    if let Some(relationships) = storage.relationships(part_id) {
      part.relationship_order = relationships
        .iter()
        .map(|relationship| relationship.id().into())
        .collect();
      for relationship in relationships.iter() {
        if relationship.is_reference_relationship() {
          if relationship.reference_kind().is_some_and(|kind| {
            matches!(
              kind,
              crate::common::ReferenceRelationshipKind::Audio
                | crate::common::ReferenceRelationshipKind::Media
                | crate::common::ReferenceRelationshipKind::Video
            )
          }) {
            part
              .data_part_reference_relationships
              .push(relationship.clone());
          } else {
            part.reference_relationships.push(relationship.clone());
          }
        } else if relationship.target_kind() == crate::common::RelationshipTargetKind::InternalPart
        {
          if let Some(child) =
            crate::parts::PartRef::from_relationship_storage(storage, relationship)
          {
            part.fallback_parts.push(child);
          }
        } else {
          part.raw_relationships.push(relationship.clone());
        }
      }
    }
    part
  }
  #[inline]
  fn from_relationship_id_with_relationships(
    storage: &crate::common::SdkPackageStorage,
    relationship_id: impl Into<String>,
    part_id: crate::common::PartId,
  ) -> Self {
    let mut visited = std::collections::HashSet::new();
    Self::from_relationship_id_with_relationships_visited(
      storage,
      relationship_id,
      part_id,
      &mut visited,
    )
  }
  fn from_relationship_id_with_relationships_visited(
    storage: &crate::common::SdkPackageStorage,
    relationship_id: impl Into<String>,
    part_id: crate::common::PartId,
    visited: &mut std::collections::HashSet<crate::common::PartId>,
  ) -> Self {
    let mut part = Self::from_part_id_with_relationships_visited(storage, part_id, visited);
    part.relationship_id = Some(relationship_id.into());
    part
  }
  fn set_relationship_id(&mut self, relationship_id: String) {
    self.relationship_id = Some(relationship_id);
  }
  fn collect_modeled_part_relationships<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
    relationships: &mut std::collections::HashMap<
      crate::common::PartId,
      crate::common::RelationshipSet,
    >,
  ) -> Result<(), crate::common::SdkError> {
    let Some(part) = package.storage().part(self.id) else {
      return Ok(());
    };
    if part.is_deleted() {
      return Ok(());
    }
    if relationships.contains_key(&self.id) {
      return Ok(());
    }
    relationships.insert(self.id, self.modeled_relationships(package)?);
    for part in &self.fallback_parts {
      part.collect_modeled_part_relationships(package, relationships)?;
    }
    Ok(())
  }
}
impl ExtendedPart {
  pub fn modeled_relationships<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Result<crate::common::RelationshipSet, crate::common::SdkError> {
    let mut relationships = crate::common::RelationshipSet::default();
    for part in &self.fallback_parts {
      crate::sdk::add_part_ref_to_relationship_set(
        &mut relationships,
        package.storage(),
        Some(self.id),
        part,
      )?;
    }
    for relationship in self
      .data_part_reference_relationships
      .iter()
      .chain(self.reference_relationships.iter())
      .chain(self.raw_relationships.iter())
    {
      relationships.add_relationship_info(relationship.clone())?;
    }
    relationships.reorder_by_ids(&self.relationship_order);
    Ok(relationships)
  }
}
