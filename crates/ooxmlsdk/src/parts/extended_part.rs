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
  pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl crate::sdk::SdkPartHandle for ExtendedPart {
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
            let item_index = part.data_part_reference_relationships.len();
            part
              .data_part_reference_relationships
              .push(relationship.clone());
            part
              .relationship_order
              .push(crate::sdk::RelationshipModelEntry::DataPartReference(
                item_index,
              ));
          } else {
            let item_index = part.reference_relationships.len();
            part.reference_relationships.push(relationship.clone());
            part
              .relationship_order
              .push(crate::sdk::RelationshipModelEntry::Reference(item_index));
          }
        } else if relationship.target_kind() == crate::common::RelationshipTargetKind::InternalPart
        {
          if let Some(child) =
            crate::parts::PartRef::from_relationship_storage(storage, relationship)
          {
            let item_index = part.fallback_parts.len();
            part.fallback_parts.push(child);
            part
              .relationship_order
              .push(crate::sdk::RelationshipModelEntry::Fallback(item_index));
          }
        } else {
          let item_index = part.raw_relationships.len();
          part.raw_relationships.push(relationship.clone());
          part
            .relationship_order
            .push(crate::sdk::RelationshipModelEntry::Raw(item_index));
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
  pub(crate) fn modeled_relationships<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Result<crate::common::RelationshipSet, crate::common::SdkError> {
    let mut relationships = crate::common::RelationshipSet::default();
    if self.relationship_order.is_empty() {
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
      return Ok(relationships);
    }
    for entry in &self.relationship_order {
      match entry {
        crate::sdk::RelationshipModelEntry::Child { .. } => {}
        crate::sdk::RelationshipModelEntry::Fallback(item_index) => {
          if let Some(part) = self.fallback_parts.get(*item_index) {
            crate::sdk::add_part_ref_to_relationship_set(
              &mut relationships,
              package.storage(),
              Some(self.id),
              part,
            )?;
          }
        }
        crate::sdk::RelationshipModelEntry::DataPartReference(item_index) => {
          if let Some(relationship) = self.data_part_reference_relationships.get(*item_index) {
            relationships.add_relationship_info(relationship.clone())?;
          }
        }
        crate::sdk::RelationshipModelEntry::Reference(item_index) => {
          if let Some(relationship) = self.reference_relationships.get(*item_index) {
            relationships.add_relationship_info(relationship.clone())?;
          }
        }
        crate::sdk::RelationshipModelEntry::Raw(item_index) => {
          if let Some(relationship) = self.raw_relationships.get(*item_index) {
            relationships.add_relationship_info(relationship.clone())?;
          }
        }
      }
    }
    Ok(relationships)
  }
}
