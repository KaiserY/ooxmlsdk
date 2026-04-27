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
  pub(crate) modeled_relationships: Vec<crate::common::RelationshipInfo>,
}
impl crate::sdk::SdkPart for ExtendedPart {
  const DESCRIPTOR: crate::sdk::PartDescriptor =
    crate::sdk::PartDescriptor::new("", "", "", "extendedPart", "");
  #[inline]
  fn from_part_id(part_id: crate::common::PartId) -> Self {
    Self {
      relationship_id: None,
      id: part_id,
      fallback_parts: Vec::new(),
      relationship_order: Vec::new(),
      modeled_relationships: Vec::new(),
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
      modeled_relationships: Vec::new(),
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
    let mut part = Self::from_part_id(part_id);
    if let Some(relationships) = storage.relationships(part_id) {
      for relationship in relationships.iter() {
        if relationship.is_reference_relationship() {
          let item_index = part.modeled_relationships.len();
          part.modeled_relationships.push(relationship.clone());
          part
            .relationship_order
            .push(crate::sdk::RelationshipModelEntry::Relationship(item_index));
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
          let item_index = part.modeled_relationships.len();
          part.modeled_relationships.push(relationship.clone());
          part
            .relationship_order
            .push(crate::sdk::RelationshipModelEntry::Relationship(item_index));
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
    let mut part = Self::from_part_id_with_relationships(storage, part_id);
    part.relationship_id = Some(relationship_id.into());
    part
  }
  fn set_relationship_id(&mut self, relationship_id: String) {
    self.relationship_id = Some(relationship_id);
  }
}
impl crate::sdk::SdkPartInternal for ExtendedPart {}
