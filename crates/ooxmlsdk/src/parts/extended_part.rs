#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExtendedPart {
  pub(crate) id: crate::common::PartId,
}

impl crate::sdk::SdkPartHandle for ExtendedPart {
  const DESCRIPTOR: crate::sdk::PartDescriptor =
    crate::sdk::PartDescriptor::new("", "", "", "extendedPart", "");

  #[inline]
  fn from_part_id(part_id: crate::common::PartId) -> Self {
    Self { id: part_id }
  }

  #[inline]
  fn part_id(self) -> crate::common::PartId {
    self.id
  }
}
