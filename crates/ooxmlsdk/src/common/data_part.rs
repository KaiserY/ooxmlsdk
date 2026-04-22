use std::collections::HashSet;
use std::io::{Read, Seek, Write};
use std::ops::{Deref, DerefMut};

use super::SdkError;

#[derive(Clone, Debug, Default)]
pub struct MediaDataPart {
  pub inner_path: String,
  pub part_content: Vec<u8>,
}

impl MediaDataPart {
  pub(crate) fn new_from_archive<R: Read + Seek>(
    path: &str,
    archive: &mut zip::ZipArchive<R>,
  ) -> Result<Self, SdkError> {
    let mut zip_entry = archive.by_name(path)?;
    let mut part_content = Vec::with_capacity(zip_entry.size() as usize);
    zip_entry.read_to_end(&mut part_content)?;

    Ok(Self {
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

    let dir_path = self
      .inner_path
      .rsplit_once('/')
      .map(|(dir_path, _)| super::resolve_zip_file_path(&format!("{dir_path}/")))
      .unwrap_or_default();
    if !dir_path.is_empty() && entry_set.insert(dir_path.clone()) {
      zip.add_directory(&dir_path, options)?;
    }

    if entry_set.insert(self.inner_path.clone()) {
      zip.start_file(&self.inner_path, options)?;
      zip.write_all(&self.part_content)?;
    }

    Ok(())
  }
}

#[cfg(feature = "validators")]
impl crate::validator::SdkValidator for MediaDataPart {}

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
        archive: &mut zip::ZipArchive<R>,
      ) -> Result<Self, SdkError> {
        Ok(Self {
          r_id: r_id.to_string(),
          media_data_part: MediaDataPart::new_from_archive(path, archive)?,
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
        archive: &mut zip::ZipArchive<R>,
      ) -> Result<Self, SdkError> {
        Self::new_from_archive(path, r_id, archive)
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

    #[cfg(feature = "validators")]
    impl crate::validator::SdkValidator for $ident {}
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
