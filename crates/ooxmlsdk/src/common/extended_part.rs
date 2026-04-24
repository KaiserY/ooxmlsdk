use std::collections::HashSet;
use std::io::{Read, Seek};

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
