pub trait SdkPartHandle: Sized {
  fn from_part_key(part_key: crate::common::PartKey) -> Self;

  fn part_key(&self) -> crate::common::PartKey;

  #[inline]
  fn part_slot<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Result<crate::common::PartSlot, crate::common::SdkError> {
    self.part_key().resolve(package.storage())
  }

  #[inline]
  fn part_slot_optional<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::common::PartSlot> {
    self.part_key().resolve_optional(package.storage())
  }
}
