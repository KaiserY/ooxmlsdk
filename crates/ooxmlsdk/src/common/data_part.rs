use std::collections::HashSet;
use std::io::{Read, Seek, Write};
use std::ops::{Deref, DerefMut};

use super::SdkError;

#[derive(Clone, Debug, Default)]
pub struct MediaDataPart {
  pub(crate) key: Option<crate::common::PartKey>,
  pub inner_path: String,
  pub part_content: Vec<u8>,
}

impl MediaDataPart {
  #[inline]
  pub(crate) fn from_part_slot(
    package_token: crate::common::PackageToken,
    part_slot: crate::common::PartSlot,
    path: impl Into<String>,
  ) -> Self {
    Self {
      key: Some(crate::common::PartKey::new(package_token, part_slot)),
      inner_path: path.into(),
      part_content: Vec::new(),
    }
  }

  #[inline]
  pub fn is_same_part(&self, other: &Self) -> bool {
    self.key.is_some() && self.key == other.key
  }

  #[inline]
  pub(crate) fn part_slot_for_package<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Result<crate::common::PartSlot, SdkError> {
    let key = self
      .key
      .ok_or_else(|| SdkError::CommonError("media data part is not package-backed".to_string()))?;
    key.resolve(package.storage())
  }

  #[inline]
  pub fn path<'a, P: crate::sdk::SdkPackage>(&'a self, package: &'a P) -> Option<&'a str> {
    let part_slot = self.key?.resolve_optional(package.storage())?;
    package.storage().part(part_slot).map(|part| part.path())
  }

  #[inline]
  pub fn content_type<'a, P: crate::sdk::SdkPackage>(&'a self, package: &'a P) -> Option<&'a str> {
    let part_slot = self.key?.resolve_optional(package.storage())?;
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
    let Some(key) = self.key else {
      return Ok(None);
    };
    let part_slot = key.resolve(package.storage())?;
    package.storage().part_bytes(part_slot).map(Some)
  }

  #[inline]
  pub fn data_part_reference_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    let part_slot = self
      .key
      .and_then(|key| key.resolve_optional(package.storage()));
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

  pub(crate) fn new_from_archive<R: Read + Seek>(
    path: &str,
    part_index: usize,
    archive: &mut zip::ZipArchive<R>,
  ) -> Result<Self, SdkError> {
    let mut zip_entry = archive.by_index(part_index)?;
    let mut part_content = Vec::with_capacity(zip_entry.size() as usize);
    zip_entry.read_to_end(&mut part_content)?;

    Ok(Self {
      key: None,
      inner_path: path.to_string(),
      part_content,
    })
  }

  pub(crate) fn save_zip<W: Write + Seek>(
    &self,
    parent_path: &str,
    zip: &mut zip::ZipWriter<W>,
    entry_set: &mut HashSet<String>,
  ) -> Result<(), SdkError> {
    let options = zip::write::SimpleFileOptions::default()
      .compression_method(zip::CompressionMethod::Deflated)
      .unix_permissions(0o755);

    let directory_path = super::resolve_zip_file_path(parent_path);
    if !directory_path.is_empty() && entry_set.insert(directory_path.clone()) {
      zip.add_directory(&directory_path, options)?;
    }

    let dir_path = super::parent_zip_path(&self.inner_path);
    let dir_path = dir_path.strip_suffix('/').unwrap_or(&dir_path);
    if !dir_path.is_empty() && entry_set.insert(dir_path.to_string()) {
      zip.add_directory(dir_path, options)?;
    }

    if entry_set.insert(self.inner_path.clone()) {
      zip.start_file(&self.inner_path, options)?;
      zip.write_all(&self.part_content)?;
    }

    Ok(())
  }
}

macro_rules! define_media_reference_relationship {
  ($ident:ident, $relationship_type:literal) => {
    #[derive(Clone, Debug, Default)]
    pub struct $ident {
      pub r_id: String,
      pub media_data_part: MediaDataPart,
    }

    impl $ident {
      pub const RELATIONSHIP_TYPE: &'static str = $relationship_type;

      pub(crate) fn new_from_archive<R: Read + Seek>(
        path: &str,
        r_id: &str,
        part_index: usize,
        archive: &mut zip::ZipArchive<R>,
      ) -> Result<Self, SdkError> {
        Ok(Self {
          r_id: r_id.to_string(),
          media_data_part: MediaDataPart::new_from_archive(path, part_index, archive)?,
        })
      }

      pub(crate) fn save_zip<W: Write + Seek>(
        &self,
        parent_path: &str,
        zip: &mut zip::ZipWriter<W>,
        entry_set: &mut HashSet<String>,
      ) -> Result<(), SdkError> {
        self.media_data_part.save_zip(parent_path, zip, entry_set)
      }
    }

    impl crate::sdk::SdkDataPartReference for $ident {
      const RELATIONSHIP_TYPE: &'static str = $relationship_type;

      fn new_from_archive<R: Read + Seek>(
        path: &str,
        r_id: &str,
        part_index: usize,
        archive: &mut zip::ZipArchive<R>,
      ) -> Result<Self, SdkError> {
        Self::new_from_archive(path, r_id, part_index, archive)
      }

      fn save_zip<W: Write + Seek>(
        &self,
        parent_path: &str,
        zip: &mut zip::ZipWriter<W>,
        entry_set: &mut HashSet<String>,
      ) -> Result<(), SdkError> {
        Self::save_zip(self, parent_path, zip, entry_set)
      }
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
