use serde::Serialize;
use serde_json::to_writer_pretty;
use std::{ffi::OsStr, fs, fs::File, io::BufWriter, path::Path};

#[path = "models/open_xml.rs"]
pub mod open_xml;
#[path = "models/sdk_data.rs"]
pub mod sdk_data_model;

pub mod context;
pub mod schemas;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, BoxError>;

use crate::sdk_data::{context::Context, schemas::gen_schemas};

pub fn gen_sdk_data<P: AsRef<Path>>(data_dir: P, out_dir: P) -> Result<()> {
  let gen_context = Context::new(data_dir.as_ref())?;
  let out_dir = out_dir.as_ref();
  let out_schemas_dir_path = out_dir.join("schemas");

  fs::create_dir_all(&out_schemas_dir_path)?;
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
