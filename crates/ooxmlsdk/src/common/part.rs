use std::collections::HashSet;
use std::io::{Read, Seek, Write};

use crate::schemas::opc_relationships::{Relationship, Relationships, TargetMode};

use super::{
  ExtendedPart, SdkError, parent_zip_path, resolve_relationship_target_path, resolve_zip_file_path,
};

#[inline(always)]
pub fn load_part_relationships<R: Read + Seek>(
  path: &str,
  file_path_set: &HashSet<String>,
  archive: &mut zip::ZipArchive<R>,
) -> Result<(String, Option<Relationships>), SdkError> {
  let child_parent_path = parent_zip_path(path);
  let part_target_str = path.rsplit('/').next().unwrap_or_default();
  let rels_candidate_path =
    resolve_zip_file_path(&format!("{child_parent_path}_rels/{part_target_str}.rels"));
  if let Some(file_path) = file_path_set.get(&rels_candidate_path) {
    let mut zip_entry = archive.by_name(file_path)?;
    let mut bytes = Vec::with_capacity(zip_entry.size() as usize);
    zip_entry.read_to_end(&mut bytes)?;
    Ok((
      file_path.to_string(),
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
  file_path_set: &HashSet<String>,
  archive: &mut zip::ZipArchive<R>,
  visited: &mut HashSet<String>,
) -> Result<Option<T>, SdkError>
where
  R: Read + Seek,
  T: crate::sdk::SdkPart,
{
  let target_path = resolve_relationship_part_path(child_parent_path, relationship);
  if !file_path_set.contains(&target_path) {
    return Ok(None);
  }
  let inserted = visited.insert(target_path.clone());
  if !inserted {
    return Ok(None);
  }
  let loaded = T::new_from_archive(
    child_parent_path,
    &target_path,
    &relationship.id,
    file_path_set,
    archive,
    visited,
  );
  visited.remove(&target_path);
  loaded.map(Some)
}

#[inline(always)]
pub fn load_data_part_reference<R, T>(
  child_parent_path: &str,
  relationship: &Relationship,
  file_path_set: &HashSet<String>,
  archive: &mut zip::ZipArchive<R>,
) -> Result<Option<T>, SdkError>
where
  R: Read + Seek,
  T: crate::sdk::SdkDataPartReference,
{
  let target_path = resolve_relationship_part_path(child_parent_path, relationship);
  if !file_path_set.contains(&target_path) {
    return Ok(None);
  }
  T::new_from_archive(&target_path, &relationship.id, archive).map(Some)
}

#[inline(always)]
pub fn load_extended_part<R: Read + Seek>(
  child_parent_path: &str,
  relationship: &Relationship,
  file_path_set: &HashSet<String>,
  archive: &mut zip::ZipArchive<R>,
  visited: &mut HashSet<String>,
) -> Result<Option<ExtendedPart>, SdkError> {
  if matches!(relationship.target_mode, Some(TargetMode::External)) {
    return Ok(None);
  }

  let target_path = resolve_relationship_part_path(child_parent_path, relationship);
  if !file_path_set.contains(&target_path) {
    return Ok(None);
  }
  let inserted = visited.insert(target_path.clone());
  if !inserted {
    return Ok(None);
  }
  let loaded = ExtendedPart::new_from_archive(
    &target_path,
    &relationship.id,
    relationship.r#type.as_str(),
    file_path_set,
    archive,
    visited,
  );
  visited.remove(&target_path);
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
  let rels_parent_path = parent_zip_path(inner_path);
  let rels_dir_path = resolve_zip_file_path(&format!("{rels_parent_path}_rels"));
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
