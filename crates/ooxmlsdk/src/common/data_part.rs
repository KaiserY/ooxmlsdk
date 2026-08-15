use std::ops::{Deref, DerefMut};

use super::SdkError;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MediaDataPart {
  pub(crate) key: crate::common::PartKey,
}

impl MediaDataPart {
  #[inline]
  pub(crate) fn from_part_slot(
    package_token: crate::common::PackageToken,
    part_slot: crate::common::PartSlot,
  ) -> Self {
    Self {
      key: crate::common::PartKey::new(package_token, part_slot),
    }
  }

  #[inline]
  pub fn is_same_part(&self, other: &Self) -> bool {
    self == other
  }

  #[inline]
  pub(crate) fn part_slot_for_package<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Result<crate::common::PartSlot, SdkError> {
    self.key.resolve(package.storage())
  }

  #[inline]
  pub fn path<'a, P: crate::sdk::SdkPackage>(&'a self, package: &'a P) -> Option<&'a str> {
    let part_slot = self.key.resolve_optional(package.storage())?;
    package.storage().part(part_slot).map(|part| part.path())
  }

  #[inline]
  pub fn content_type<'a, P: crate::sdk::SdkPackage>(&'a self, package: &'a P) -> Option<&'a str> {
    let part_slot = self.key.resolve_optional(package.storage())?;
    package
      .storage()
      .part(part_slot)
      .map(|part| part.content_type())
  }

  #[inline]
  pub fn data<'a, P: crate::sdk::SdkPackage>(&'a self, package: &'a P) -> Option<&'a [u8]> {
    self.try_data(package).ok().flatten()
  }

  #[inline]
  pub fn try_data<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> Result<Option<&'a [u8]>, SdkError> {
    let part_slot = self.key.resolve(package.storage())?;
    package.storage().part_bytes(part_slot).map(Some)
  }

  /// Returns an owned, shared view of the media payload.
  ///
  /// Loading an archived part may allocate once; cloning the returned
  /// [`bytes::Bytes`] reuses that cached payload without copying its contents.
  #[inline]
  pub fn try_data_bytes<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Result<bytes::Bytes, SdkError> {
    let part_slot = self.key.resolve(package.storage())?;
    package.storage().part_bytes_owned(part_slot)
  }

  #[inline]
  pub fn data_part_reference_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    let part_slot = self.key.resolve_optional(package.storage());
    let package_token = package.storage().token();
    part_slot.into_iter().flat_map(move |part_slot| {
      package
        .storage()
        .data_part_reference_relationships_to(part_slot)
        .map(move |relationship| crate::common::RelationshipRef::new(package_token, relationship))
    })
  }

  #[inline]
  pub fn is_orphan<P: crate::sdk::SdkPackage>(&self, package: &P) -> bool {
    self
      .data_part_reference_relationships(package)
      .next()
      .is_none()
  }

  #[inline]
  pub fn set_data<P: crate::sdk::SdkPackage>(
    &self,
    package: &mut P,
    data: Vec<u8>,
  ) -> Result<(), SdkError> {
    let part_slot = self.part_slot_for_package(package)?;
    package.storage_mut().set_part_data(part_slot, data)
  }
}

macro_rules! define_media_reference_relationship {
  ($ident:ident, $relationship_type:literal) => {
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct $ident {
      pub r_id: String,
      pub media_data_part: MediaDataPart,
    }

    impl $ident {
      pub const RELATIONSHIP_TYPE: &'static str = $relationship_type;
    }

    impl Deref for $ident {
      type Target = MediaDataPart;

      fn deref(&self) -> &Self::Target {
        &self.media_data_part
      }
    }

    impl DerefMut for $ident {
      fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.media_data_part
      }
    }
  };
}

define_media_reference_relationship!(
  AudioReferenceRelationship,
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio"
);
define_media_reference_relationship!(
  MediaReferenceRelationship,
  "http://schemas.microsoft.com/office/2007/relationships/media"
);
define_media_reference_relationship!(
  VideoReferenceRelationship,
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video"
);
