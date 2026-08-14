//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtendedPart {
  pub(crate) key: crate::common::PartKey,
}
impl crate::sdk::SdkPartDescriptor for ExtendedPart {
  const KIND: crate::parts::PartKind = crate::parts::PartKind::ExtendedPart;
  const RELATIONSHIP_TYPE: &'static str = "";
  const PATH_PREFIX: &'static str = "";
  const CONTENT_TYPE: &'static str = "";
  const TARGET_NAME: &'static str = "extendedPart";
  const EXTENSION: &'static str = "";
}
impl crate::private::SdkPartHandle for ExtendedPart {
  #[inline]
  fn from_part_key(part_key: crate::common::PartKey) -> Self {
    Self { key: part_key }
  }
  #[inline]
  fn part_key(&self) -> crate::common::PartKey {
    self.key
  }
}
impl crate::sdk::SdkPart for ExtendedPart {
  const CHILD_PART_CONSTRAINTS: &'static [crate::sdk::PartConstraint] = &[];
  const ALLOWS_ANY_CHILD_PART: bool = true;
  #[inline]
  fn child_part_constraint(_kind: crate::parts::PartKind) -> Option<crate::sdk::PartConstraint> {
    None
  }
}
