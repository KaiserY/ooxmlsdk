use proc_macro2::TokenStream;
use quote::quote;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use syn::{Arm, Attribute, Ident, ItemMod, parse_str, parse2};

use crate::Result;
use crate::sdk_code::parts::{gen_part_module, gen_parts_mod};
use crate::sdk_code::schemas::{CodegenContext, gen_schema};
use crate::sdk_code::versioning::version_cfg_attrs;
use crate::sdk_data::sdk_data_model::{
  Namespace as SdkDataNamespace, Part as SdkDataPart, Schema as SdkDataSchema,
};

pub mod helpers;
pub mod parts;
pub mod schemas;
pub mod versioning;

const FILE_HEADER: &str = r#"//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//
"#;

pub fn gen_sdk_code<P: AsRef<Path>>(sdk_data_dir: P, out_dir: P) -> Result<()> {
  let sdk_data_schemas_dir_path = sdk_data_dir.as_ref().join("schemas");
  let sdk_data_parts_dir_path = sdk_data_dir.as_ref().join("parts");
  let sdk_data_schemas = read_schemas(&sdk_data_schemas_dir_path)?;
  let sdk_data_parts = read_parts(&sdk_data_parts_dir_path)?;
  let sdk_data_namespaces = read_namespaces(sdk_data_dir.as_ref().join("namespaces.json"))?;
  let out_dir_path = out_dir.as_ref();
  let context = CodegenContext::new(&sdk_data_schemas);

  write_schemas(&sdk_data_schemas, &context, out_dir_path)?;
  write_parts(&sdk_data_parts, out_dir_path)?;
  write_namespaces(&sdk_data_namespaces, out_dir_path)?;

  Ok(())
}

fn read_schemas(sdk_data_schemas_dir_path: &Path) -> Result<Vec<SdkDataSchema>> {
  let mut sdk_data_schemas = vec![];

  for entry in fs::read_dir(sdk_data_schemas_dir_path)? {
    let entry = entry?;
    let path = entry.path();

    if !path.is_file() || path.extension() != Some(OsStr::new("json")) {
      continue;
    }

    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
      continue;
    };
    if file_name.starts_with("package_") {
      continue;
    }
    let sdk_data_schema: SdkDataSchema = serde_json::from_reader(reader)?;
    sdk_data_schemas.push(sdk_data_schema);
  }

  sdk_data_schemas.sort_by(|a, b| a.module_name.cmp(&b.module_name));

  Ok(sdk_data_schemas)
}

fn read_parts(sdk_data_parts_dir_path: &Path) -> Result<Vec<SdkDataPart>> {
  let mut sdk_data_parts = vec![];

  if !sdk_data_parts_dir_path.exists() {
    return Ok(sdk_data_parts);
  }

  for entry in fs::read_dir(sdk_data_parts_dir_path)? {
    let entry = entry?;
    let path = entry.path();

    if !path.is_file() || path.extension() != Some(OsStr::new("json")) {
      continue;
    }

    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let sdk_data_part: SdkDataPart = serde_json::from_reader(reader)?;
    sdk_data_parts.push(sdk_data_part);
  }

  sdk_data_parts.sort_by(|a, b| a.module_name.cmp(&b.module_name));
  Ok(sdk_data_parts)
}

fn read_namespaces(path: impl AsRef<Path>) -> Result<Vec<SdkDataNamespace>> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let mut namespaces: Vec<SdkDataNamespace> = serde_json::from_reader(reader)?;
  namespaces.sort_by(|left, right| {
    left
      .prefix
      .cmp(&right.prefix)
      .then(left.uri.cmp(&right.uri))
  });
  Ok(namespaces)
}

fn write_schemas(
  sdk_data_schemas: &[SdkDataSchema],
  context: &CodegenContext<'_>,
  out_dir_path: &Path,
) -> Result<()> {
  let out_schemas_dir_path = out_dir_path.join("schemas");
  fs::create_dir_all(&out_schemas_dir_path)?;
  clear_generated_rs_files(&out_schemas_dir_path)?;

  let mut schemas_mod_list: Vec<ItemMod> = vec![];

  for sdk_data_schema in sdk_data_schemas {
    let schema_path = out_schemas_dir_path.join(format!("{}.rs", sdk_data_schema.module_name));
    write_generated_module(
      &schema_path,
      gen_schema(
        sdk_data_schema,
        context,
        schema_module_is_microsoft365_only(sdk_data_schema),
      )
      .map_err(|err| {
        format!(
          "failed to generate schema {}: {err:?}",
          sdk_data_schema.module_name
        )
      })?,
    )?;

    push_module_decl(
      &mut schemas_mod_list,
      &sdk_data_schema.module_name,
      schema_module_cfg_attrs(sdk_data_schema),
    )?;
  }

  let token_stream: TokenStream = quote! {
    #( #schemas_mod_list )*
  };
  let schemas_mod_path = out_dir_path.join("schemas.rs");
  write_generated_module(&schemas_mod_path, token_stream)?;

  Ok(())
}

fn write_parts(sdk_data_parts: &[SdkDataPart], out_dir_path: &Path) -> Result<()> {
  let out_parts_dir_path = out_dir_path.join("parts");
  fs::create_dir_all(&out_parts_dir_path)?;
  clear_generated_rs_files(&out_parts_dir_path)?;
  write_generated_module(&out_parts_dir_path.join("mod.rs"), quote! {})?;

  for sdk_data_part in sdk_data_parts {
    let part_path = out_parts_dir_path.join(format!("{}.rs", sdk_data_part.module_name));
    write_generated_module(
      &part_path,
      gen_part_module(sdk_data_part).map_err(|err| {
        format!(
          "failed to generate part {}: {err}",
          sdk_data_part.module_name
        )
      })?,
    )?;
  }

  write_generated_module(
    &out_parts_dir_path.join("mod.rs"),
    gen_parts_mod(sdk_data_parts)?,
  )?;

  Ok(())
}

fn write_namespaces(sdk_data_namespaces: &[SdkDataNamespace], out_dir_path: &Path) -> Result<()> {
  let mut uri_to_prefix_arms: Vec<Arm> = vec![];
  let mut prefix_to_uri_arms: Vec<Arm> = vec![];

  for namespace in sdk_data_namespaces {
    let uri = namespace.uri.as_str();
    let prefix = namespace.prefix.as_str();
    let attrs = version_cfg_attrs(&namespace.version);

    uri_to_prefix_arms.push(parse2(quote! {
      #( #attrs )*
      #uri => Some(#prefix),
    })?);
    prefix_to_uri_arms.push(parse2(quote! {
      #( #attrs )*
      #prefix => Some(#uri),
    })?);
  }

  let token_stream: TokenStream = quote! {
    pub(crate) fn prefix_by_uri(uri: &str) -> Option<&'static str> {
      match uri {
        #( #uri_to_prefix_arms )*
        _ => None,
      }
    }

    pub(crate) fn uri_by_prefix(prefix: &str) -> Option<&'static str> {
      match prefix {
        #( #prefix_to_uri_arms )*
        _ => None,
      }
    }
  };

  let namespaces_path = out_dir_path.join("namespaces.rs");
  write_generated_module(&namespaces_path, token_stream)?;

  Ok(())
}

fn write_generated_module(path: &Path, token_stream: TokenStream) -> Result<()> {
  let syntax_tree: syn::File = parse2(token_stream)
    .map_err(|err| format!("failed to parse generated module {}: {err}", path.display()))?;
  let formatted = prettyplease::unparse(&syntax_tree);
  fs::write(path, format!("{FILE_HEADER}\n{formatted}"))?;
  Ok(())
}

fn push_module_decl(
  mod_list: &mut Vec<ItemMod>,
  module_name: &str,
  cfg_attrs: Vec<Attribute>,
) -> Result<()> {
  let module_ident: Ident = parse_str(module_name)?;
  mod_list.push(parse2(quote! {
    #( #cfg_attrs )*
    pub mod #module_ident;
  })?);
  Ok(())
}

fn schema_module_cfg_attrs(schema: &SdkDataSchema) -> Vec<Attribute> {
  if schema_module_is_microsoft365_only(schema) {
    version_cfg_attrs("Microsoft365")
  } else {
    Vec::new()
  }
}

fn schema_module_is_microsoft365_only(schema: &SdkDataSchema) -> bool {
  (!schema.types.is_empty() || !schema.enums.is_empty())
    && schema
      .types
      .iter()
      .all(|schema_type| versioning::is_microsoft365_version(&schema_type.version))
    && schema
      .enums
      .iter()
      .all(|schema_enum| versioning::is_microsoft365_version(&schema_enum.version))
}

fn clear_generated_rs_files(out_dir_path: &Path) -> Result<()> {
  for entry in fs::read_dir(out_dir_path)? {
    let entry = entry?;
    let path = entry.path();

    if path.is_file() && path.extension() == Some(OsStr::new("rs")) {
      fs::remove_file(path)?;
    }
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::sdk_data::sdk_data_model::{
    SchemaEnum as SdkDataSchemaEnum, SchemaType as SdkDataSchemaType,
  };

  fn schema(types: Vec<SdkDataSchemaType>, enums: Vec<SdkDataSchemaEnum>) -> SdkDataSchema {
    SdkDataSchema {
      types,
      enums,
      ..SdkDataSchema::default()
    }
  }

  fn schema_type(version: &str) -> SdkDataSchemaType {
    SdkDataSchemaType {
      version: version.to_string(),
      ..SdkDataSchemaType::default()
    }
  }

  fn schema_enum(version: &str) -> SdkDataSchemaEnum {
    SdkDataSchemaEnum {
      version: version.to_string(),
      ..SdkDataSchemaEnum::default()
    }
  }

  #[test]
  fn treats_enum_only_microsoft365_schema_modules_as_microsoft365_only() {
    let schema = schema(vec![], vec![schema_enum("Office2016")]);

    assert!(schema_module_is_microsoft365_only(&schema));
  }

  #[test]
  fn does_not_treat_empty_schema_modules_as_microsoft365_only() {
    let schema = schema(vec![], vec![]);

    assert!(!schema_module_is_microsoft365_only(&schema));
  }

  #[test]
  fn does_not_treat_mixed_version_schema_modules_as_microsoft365_only() {
    let schema = schema(
      vec![schema_type("Office2013")],
      vec![schema_enum("Office2007")],
    );

    assert!(!schema_module_is_microsoft365_only(&schema));
  }
}
