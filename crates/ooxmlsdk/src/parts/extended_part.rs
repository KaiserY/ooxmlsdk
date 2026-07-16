//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtendedPart {
  pub(crate) id: crate::common::PartId,
}
impl crate::sdk::SdkPartDescriptor for ExtendedPart {
  const KIND: crate::parts::PartKind = crate::parts::PartKind::ExtendedPart;
  const RELATIONSHIP_TYPE: &'static str = "";
  const PATH_PREFIX: &'static str = "";
  const CONTENT_TYPE: &'static str = "";
  const TARGET_NAME: &'static str = "extendedPart";
  const EXTENSION: &'static str = "";
}
impl crate::sdk::SdkPart for ExtendedPart {
  #[inline]
  fn from_part_id(part_id: crate::common::PartId) -> Self {
    Self { id: part_id }
  }
  #[inline]
  fn part_id(&self) -> crate::common::PartId {
    self.id
  }
}
