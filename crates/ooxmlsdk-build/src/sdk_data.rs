use serde::Serialize;
use serde_json::to_writer_pretty;
use std::{ffi::OsStr, fs, fs::File, io::BufWriter, path::Path};

pub mod context;
pub mod mce;
pub mod opc_schemas;
pub mod open_xml;
pub mod parts;
pub mod schema_extensions;
pub mod schemas;
pub mod sdk_data_model;
pub mod xsd;

use crate::Result;
use crate::sdk_code::codegen_ir_builder::build_codegen_ir;
use crate::sdk_code::schemas::CodegenContext;
use crate::sdk_data::{
  context::Context,
  mce::gen_mc_schema_from_xsd,
  opc_schemas::read_opc_schemas,
  parts::gen_parts,
  schema_extensions::{apply_schema_extensions, read_schema_extensions},
  schemas::{assign_schema_particle_ids, gen_schemas},
};

pub fn gen_sdk_data<P: AsRef<Path>, Q: AsRef<Path>>(
  data_dir: P,
  out_dir: P,
  package_schemas_dir: Q,
) -> Result<()> {
  let mut gen_context = Context::new(data_dir.as_ref())?;
  let mc_schema_dir = package_schemas_dir
    .as_ref()
    .parent()
    .map(|path| path.join("mce"))
    .ok_or_else(|| "failed to resolve schemas/mce directory".to_string())?;
  if let Some(mc_schema) = gen_mc_schema_from_xsd(&mc_schema_dir)? {
    gen_context.schemas.push(mc_schema);
  }
  gen_context
    .schemas
    .sort_by(|left, right| left.module_name.cmp(&right.module_name));
  let out_dir = out_dir.as_ref();
  let out_parts_dir_path = out_dir.join("parts");
  let out_schemas_dir_path = out_dir.join("schemas");

  fs::create_dir_all(&out_parts_dir_path)?;
  fs::create_dir_all(&out_schemas_dir_path)?;
  clear_generated_json_files(&out_parts_dir_path)?;
  clear_generated_json_files(&out_schemas_dir_path)?;

  write_json(
    out_dir.join("namespaces.json"),
    &schemas::gen_namespaces(&gen_context),
  )?;

  let mut schemas = gen_schemas(&gen_context);
  schemas.extend(read_opc_schemas(package_schemas_dir.as_ref())?);
  schemas.sort_by(|left, right| left.module_name.cmp(&right.module_name));
  let schema_extensions = read_schema_extensions(&out_dir.join("schema_extensions"))?;
  apply_schema_extensions(&mut schemas, &schema_extensions)?;
  assign_schema_particle_ids(&mut schemas);
  let codegen_context = CodegenContext::new(&schemas);

  for schema in &schemas {
    let ir = build_codegen_ir(schema, &codegen_context).map_err(|err| {
      format!(
        "failed to build codegen IR for {}: {err}",
        schema.module_name
      )
    })?;
    write_json(
      out_schemas_dir_path.join(format!("{}.json", schema.module_name)),
      &ir,
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
