use std::collections::HashSet;
use std::io::{Read, Seek, Write};

use crate::schemas::opc_relationships::{Relationships, TargetMode};

use super::SdkError;

#[derive(Clone, Debug, Default)]
pub struct ExtendedPart {
  pub r_id: String,
  pub relationship_type: String,
  pub inner_path: String,
  pub rels_path: String,
  pub relationships: Option<Relationships>,
  pub part_content: Vec<u8>,
  pub extended_parts: Vec<ExtendedPart>,
}

impl ExtendedPart {
  pub(crate) fn new_from_archive<R: Read + Seek>(
    path: &str,
    r_id: &str,
    relationship_type: &str,
    part_index: usize,
    archive: &mut zip::ZipArchive<R>,
    visited: &mut HashSet<usize>,
  ) -> Result<Self, SdkError> {
    let child_parent_path = super::parent_zip_path(path);
    let rels_candidate_path = super::part_relationships_path(path);
    let mut rels_path = String::new();
    let relationships = if let Some(rels_index) = archive.index_for_name(&rels_candidate_path) {
      rels_path = rels_candidate_path;
      Some({
        let mut zip_entry = archive.by_index(rels_index)?;
        let mut bytes = Vec::with_capacity(zip_entry.size() as usize);
        zip_entry.read_to_end(&mut bytes)?;
        Relationships::from_bytes(&bytes)?
      })
    } else {
      None
    };

    let mut part_content = {
      let mut zip_entry = archive.by_index(part_index)?;
      let mut bytes = Vec::with_capacity(zip_entry.size() as usize);
      zip_entry.read_to_end(&mut bytes)?;
      bytes
    };

    let mut extended_parts = Vec::new();
    if let Some(relationships) = &relationships {
      for relationship in &relationships.relationship {
        if matches!(relationship.target_mode, Some(TargetMode::External)) {
          continue;
        }

        let target_path = super::resolve_zip_file_path(&super::resolve_relationship_target_path(
          &child_parent_path,
          &relationship.target,
        ));

        let Some(target_index) = archive.index_for_name(&target_path) else {
          continue;
        };

        if !visited.insert(target_index) {
          continue;
        }

        let loaded = Self::new_from_archive(
          &target_path,
          &relationship.id,
          relationship.r#type.as_str(),
          target_index,
          archive,
          visited,
        )?;
        visited.remove(&target_index);
        extended_parts.push(loaded);
      }
    }

    Ok(Self {
      r_id: r_id.to_string(),
      relationship_type: relationship_type.to_string(),
      inner_path: path.to_string(),
      rels_path,
      relationships,
      part_content: std::mem::take(&mut part_content),
      extended_parts,
    })
  }

  #[allow(dead_code)]
  pub(crate) fn save_zip<W: Write + Seek>(
    &self,
    parent_path: &str,
    zip: &mut zip::ZipWriter<W>,
    entry_set: &mut HashSet<String>,
    visited: &mut HashSet<String>,
  ) -> Result<(), SdkError> {
    if !visited.insert(self.inner_path.clone()) {
      return Ok(());
    }

    let options = zip::write::SimpleFileOptions::default()
      .compression_method(zip::CompressionMethod::Deflated)
      .unix_permissions(0o755);

    let directory_path = super::resolve_zip_file_path(parent_path);
    if !directory_path.is_empty() && entry_set.insert(directory_path.clone()) {
      zip.add_directory(&directory_path, options)?;
    }

    let child_parent_path = super::parent_zip_path(&self.inner_path);
    let dir_path = child_parent_path
      .strip_suffix('/')
      .unwrap_or(&child_parent_path);
    if !dir_path.is_empty() && entry_set.insert(dir_path.to_string()) {
      zip.add_directory(dir_path, options)?;
    }

    if let Some(relationships) = &self.relationships {
      let rels_dir_path = super::part_relationships_directory_path(&self.inner_path);
      if !rels_dir_path.is_empty() && entry_set.insert(rels_dir_path.clone()) {
        zip.add_directory(&rels_dir_path, options)?;
      }
      if entry_set.insert(self.rels_path.clone()) {
        zip.start_file(&self.rels_path, options)?;
        let xml = relationships.to_xml_bytes()?;
        zip.write_all(&xml)?;
      }
    }

    if entry_set.insert(self.inner_path.clone()) {
      zip.start_file(&self.inner_path, options)?;
      zip.write_all(&self.part_content)?;
    }

    for part in &self.extended_parts {
      part.save_zip(&child_parent_path, zip, entry_set, visited)?;
    }

    Ok(())
  }
}

#[cfg(feature = "validators")]
impl crate::validator::SdkValidator for ExtendedPart {
  fn validate(&self) -> Result<(), SdkError> {
    for part in &self.extended_parts {
      crate::validator::SdkValidator::validate(part)?;
    }

    Ok(())
  }
}
