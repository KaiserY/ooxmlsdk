use serde::Serialize;
use serde_json::to_writer_pretty;
use std::{ffi::OsStr, fs, fs::File, io::BufWriter, path::Path};

pub mod compatibility;
pub mod context;
pub mod open_xml;
pub mod package_schemas;
pub mod parts;
pub mod schemas;
pub mod sdk_data_model;

use crate::Result;
use crate::sdk_data::{
  context::Context, package_schemas::read_package_schemas, parts::gen_parts, schemas::gen_schemas,
};

pub fn gen_sdk_data<P: AsRef<Path>, Q: AsRef<Path>>(
  data_dir: P,
  out_dir: P,
  package_schemas_dir: Q,
) -> Result<()> {
  let gen_context = Context::new(data_dir.as_ref())?;
  let out_dir = out_dir.as_ref();
  let out_package_schemas_dir_path = out_dir.join("package_schemas");
  let out_parts_dir_path = out_dir.join("parts");
  let out_schemas_dir_path = out_dir.join("schemas");

  fs::create_dir_all(&out_package_schemas_dir_path)?;
  fs::create_dir_all(&out_parts_dir_path)?;
  fs::create_dir_all(&out_schemas_dir_path)?;
  clear_generated_json_files(&out_package_schemas_dir_path)?;
  clear_generated_json_files(&out_parts_dir_path)?;
  clear_generated_json_files(&out_schemas_dir_path)?;

  write_json(
    out_dir.join("namespaces.json"),
    &schemas::gen_namespaces(&gen_context),
  )?;

  for schema in gen_schemas(&gen_context) {
    write_json(
      out_schemas_dir_path.join(format!("{}.json", schema.module_name)),
      &schema,
    )?;
  }

  for package_schema in read_package_schemas(package_schemas_dir.as_ref())? {
    write_json(
      out_package_schemas_dir_path.join(format!("{}.json", package_schema.module_name)),
      &package_schema,
    )?;
  }

  for part in gen_parts(&gen_context) {
    write_json(
      out_parts_dir_path.join(format!("{}.json", part.module_name)),
      &part,
    )?;
  }

  Ok(())
}

fn write_json<P: AsRef<Path>, T: Serialize>(path: P, value: &T) -> Result<()> {
  let file = File::create(path)?;
  let writer = BufWriter::new(file);
  to_writer_pretty(writer, value)?;
  Ok(())
}

fn clear_generated_json_files(out_dir_path: &Path) -> Result<()> {
  for entry in fs::read_dir(out_dir_path)? {
    let entry = entry?;
    let path = entry.path();

    if path.is_file() && path.extension() == Some(OsStr::new("json")) {
      fs::remove_file(path)?;
    }
  }

  Ok(())
}
