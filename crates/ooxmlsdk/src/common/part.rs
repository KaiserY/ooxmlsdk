use std::collections::HashSet;
use std::io::{Read, Seek, Write};

use crate::schemas::opc_relationships::{Relationship, Relationships, TargetMode};

use super::{
  ExtendedPart, SdkError, part_relationships_directory_path, part_relationships_path,
  resolve_relationship_target_path, resolve_zip_file_path,
};

#[inline(always)]
pub fn load_part_relationships<R: Read + Seek>(
  path: &str,
  archive: &mut zip::ZipArchive<R>,
) -> Result<(String, Option<Relationships>), SdkError> {
  let rels_candidate_path = part_relationships_path(path);
  if let Some(rels_index) = archive.index_for_name(&rels_candidate_path) {
    let mut zip_entry = archive.by_index(rels_index)?;
    let mut bytes = Vec::with_capacity(zip_entry.size() as usize);
    zip_entry.read_to_end(&mut bytes)?;
    Ok((
      rels_candidate_path,
      Some(Relationships::from_bytes(&bytes)?),
    ))
  } else {
    Ok((String::new(), None))
  }
}

#[inline(always)]
pub fn resolve_relationship_part_path(
  child_parent_path: &str,
  relationship: &Relationship,
) -> String {
  resolve_zip_file_path(&resolve_relationship_target_path(
    child_parent_path,
    &relationship.target,
  ))
}

#[inline(always)]
pub fn load_typed_child_part<R, T>(
  child_parent_path: &str,
  relationship: &Relationship,
  archive: &mut zip::ZipArchive<R>,
  visited: &mut HashSet<usize>,
) -> Result<Option<T>, SdkError>
where
  R: Read + Seek,
  T: crate::sdk::SdkPart,
{
  let target_path = resolve_relationship_part_path(child_parent_path, relationship);
  let Some(target_index) = archive.index_for_name(&target_path) else {
    return Ok(None);
  };
  if !visited.insert(target_index) {
    return Ok(None);
  }
  let loaded = T::new_from_archive(
    child_parent_path,
    &target_path,
    &relationship.id,
    Some(target_index),
    archive,
    visited,
  );
  visited.remove(&target_index);
  loaded.map(Some)
}

#[inline(always)]
pub fn load_data_part_reference<R, T>(
  child_parent_path: &str,
  relationship: &Relationship,
  archive: &mut zip::ZipArchive<R>,
) -> Result<Option<T>, SdkError>
where
  R: Read + Seek,
  T: crate::sdk::SdkDataPartReference,
{
  let target_path = resolve_relationship_part_path(child_parent_path, relationship);
  let Some(target_index) = archive.index_for_name(&target_path) else {
    return Ok(None);
  };
  T::new_from_archive(&target_path, &relationship.id, target_index, archive).map(Some)
}

#[inline(always)]
pub fn load_extended_part<R: Read + Seek>(
  child_parent_path: &str,
  relationship: &Relationship,
  archive: &mut zip::ZipArchive<R>,
  visited: &mut HashSet<usize>,
) -> Result<Option<ExtendedPart>, SdkError> {
  if matches!(relationship.target_mode, Some(TargetMode::External)) {
    return Ok(None);
  }

  let target_path = resolve_relationship_part_path(child_parent_path, relationship);
  let Some(target_index) = archive.index_for_name(&target_path) else {
    return Ok(None);
  };
  if !visited.insert(target_index) {
    return Ok(None);
  }
  let loaded = ExtendedPart::new_from_archive(
    &target_path,
    &relationship.id,
    relationship.r#type.as_str(),
    target_index,
    archive,
    visited,
  );
  visited.remove(&target_index);
  loaded.map(Some)
}

#[inline(always)]
pub fn save_part_relationships<W: Write + Seek>(
  inner_path: &str,
  rels_path: &str,
  relationships: &Relationships,
  zip: &mut zip::ZipWriter<W>,
  entry_set: &mut HashSet<String>,
) -> Result<(), SdkError> {
  let options = zip::write::SimpleFileOptions::default()
    .compression_method(zip::CompressionMethod::Deflated)
    .unix_permissions(0o755);
  let rels_dir_path = part_relationships_directory_path(inner_path);
  if !rels_dir_path.is_empty() && entry_set.insert(rels_dir_path.clone()) {
    zip.add_directory(&rels_dir_path, options)?;
  }
  if entry_set.insert(rels_path.to_string()) {
    zip.start_file(rels_path, options)?;
    let xml = relationships.to_xml_bytes()?;
    zip.write_all(&xml)?;
  }
  Ok(())
}

#[inline(always)]
pub fn save_typed_child_part<W, T>(
  child: &T,
  child_parent_path: &str,
  zip: &mut zip::ZipWriter<W>,
  entry_set: &mut HashSet<String>,
  visited: &mut HashSet<String>,
) -> Result<(), SdkError>
where
  W: Write + Seek,
  T: crate::sdk::SdkPart,
{
  child.save_zip(child_parent_path, zip, entry_set, visited)
}

#[inline(always)]
pub fn save_data_part_reference<W, T>(
  data_ref: &T,
  child_parent_path: &str,
  zip: &mut zip::ZipWriter<W>,
  entry_set: &mut HashSet<String>,
) -> Result<(), SdkError>
where
  W: Write + Seek,
  T: crate::sdk::SdkDataPartReference,
{
  data_ref.save_zip(child_parent_path, zip, entry_set)
}
