use proc_macro2::TokenStream;
use quote::quote;
use serde_json::Value;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use syn::{Arm, Attribute, Ident, ItemMod, parse_str, parse2};

use crate::Result;
use crate::sdk_code::codegen_ir::SchemaModuleDecl;
use crate::sdk_code::codegen_ir_builder::build_codegen_ir;
use crate::sdk_code::parts::{gen_part_module, gen_parts_mod};
use crate::sdk_code::schemas::{
  CodegenContext, TypeContainmentGraph, gen_schema_from_ir_with_type_graph,
};
use crate::sdk_code::versioning::version_cfg_attrs;
use crate::sdk_data::sdk_data_model::{
  Namespace as SdkDataNamespace, Part as SdkDataPart, Schema as SdkDataSchema,
};

pub mod codegen_ir;
pub mod codegen_ir_builder;
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

struct LoadedSchema {
  ir: SchemaModuleDecl,
}

enum SchemaInputRecord {
  Legacy(SdkDataSchema),
  Ir(SchemaModuleDecl),
}

pub fn gen_sdk_code<P: AsRef<Path>>(sdk_data_dir: P, out_dir: P) -> Result<()> {
  let sdk_data_schemas_dir_path = sdk_data_dir.as_ref().join("schemas");
  let sdk_data_parts_dir_path = sdk_data_dir.as_ref().join("parts");
  let loaded_schemas = read_schemas(&sdk_data_schemas_dir_path)?;
  let sdk_data_parts = read_parts(&sdk_data_parts_dir_path)?;
  let sdk_data_namespaces = read_namespaces(sdk_data_dir.as_ref().join("namespaces.json"))?;
  let out_dir_path = out_dir.as_ref();

  write_schemas(&loaded_schemas, out_dir_path)?;
  write_parts(&sdk_data_parts, out_dir_path)?;
  write_namespaces(&sdk_data_namespaces, out_dir_path)?;

  Ok(())
}

fn read_schemas(sdk_data_schemas_dir_path: &Path) -> Result<Vec<LoadedSchema>> {
  let mut input_records = vec![];

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
    let value: Value = serde_json::from_reader(reader)?;
    if is_codegen_ir_schema_json(&value) {
      input_records.push(SchemaInputRecord::Ir(serde_json::from_value(value)?));
    } else {
      input_records.push(SchemaInputRecord::Legacy(serde_json::from_value(value)?));
    }
  }

  input_records.sort_by(|a, b| schema_input_module_name(a).cmp(schema_input_module_name(b)));

  let legacy_schemas: Vec<SdkDataSchema> = input_records
    .iter()
    .filter_map(|item| match item {
      SchemaInputRecord::Legacy(schema) => Some(schema.clone()),
      SchemaInputRecord::Ir(_) => None,
    })
    .collect();
  let context = CodegenContext::new(&legacy_schemas);

  input_records
    .into_iter()
    .map(|record| match record {
      SchemaInputRecord::Legacy(legacy) => {
        let ir = build_codegen_ir(&legacy, &context)?;
        Ok(LoadedSchema { ir })
      }
      SchemaInputRecord::Ir(ir) => Ok(LoadedSchema { ir }),
    })
    .collect()
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

fn write_schemas(loaded_schemas: &[LoadedSchema], out_dir_path: &Path) -> Result<()> {
  let out_schemas_dir_path = out_dir_path.join("schemas");
  fs::create_dir_all(&out_schemas_dir_path)?;
  clear_generated_rs_files(&out_schemas_dir_path)?;
  let schema_graph = TypeContainmentGraph::from_modules(
    &loaded_schemas
      .iter()
      .map(|loaded| &loaded.ir)
      .collect::<Vec<_>>(),
  );

  let mut schemas_mod_list: Vec<ItemMod> = vec![];

  for loaded_schema in loaded_schemas {
    let schema_path = out_schemas_dir_path.join(format!("{}.rs", loaded_schema.ir.module_name));
    write_generated_module(
      &schema_path,
      gen_schema_from_ir_with_type_graph(
        &loaded_schema.ir,
        schema_module_is_microsoft365_only_ir(&loaded_schema.ir),
        &schema_graph,
      )
      .map_err(|err| {
        format!(
          "failed to generate schema {}: {err:?}",
          loaded_schema.ir.module_name
        )
      })?,
    )?;

    push_module_decl(
      &mut schemas_mod_list,
      &loaded_schema.ir.module_name,
      schema_module_cfg_attrs_ir(&loaded_schema.ir),
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

fn schema_module_cfg_attrs_ir(schema: &SchemaModuleDecl) -> Vec<Attribute> {
  if schema_module_is_microsoft365_only_ir(schema) {
    version_cfg_attrs("Microsoft365")
  } else {
    Vec::new()
  }
}

fn schema_module_is_microsoft365_only_ir(schema: &SchemaModuleDecl) -> bool {
  let concrete_type_count = schema
    .types
    .iter()
    .filter(|schema_type| {
      !schema_type.is_abstract
        && matches!(
          schema_type.kind,
          crate::sdk_code::codegen_ir::TypeKind::ElementStruct
            | crate::sdk_code::codegen_ir::TypeKind::LeafTextAlias
        )
    })
    .count();

  (concrete_type_count > 0 || !schema.enums.is_empty())
    && schema
      .types
      .iter()
      .filter(|schema_type| {
        !schema_type.is_abstract
          && matches!(
            schema_type.kind,
            crate::sdk_code::codegen_ir::TypeKind::ElementStruct
              | crate::sdk_code::codegen_ir::TypeKind::LeafTextAlias
          )
      })
      .all(|schema_type| {
        schema_type
          .version
          .as_deref()
          .is_some_and(versioning::is_microsoft365_version)
      })
    && schema.enums.iter().all(|schema_enum| {
      schema_enum
        .version
        .as_deref()
        .is_some_and(versioning::is_microsoft365_version)
    })
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

fn schema_input_module_name(record: &SchemaInputRecord) -> &str {
  match record {
    SchemaInputRecord::Legacy(schema) => &schema.module_name,
    SchemaInputRecord::Ir(schema) => &schema.module_name,
  }
}

fn is_codegen_ir_schema_json(value: &Value) -> bool {
  value
    .get("Types")
    .and_then(Value::as_array)
    .and_then(|types| types.first())
    .and_then(Value::as_object)
    .is_some_and(|ty| ty.contains_key("RustName"))
    || value
      .get("Enums")
      .and_then(Value::as_array)
      .and_then(|enums| enums.first())
      .and_then(Value::as_object)
      .is_some_and(|en| en.contains_key("RustName"))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::sdk_code::codegen_ir::{EnumDecl, TypeDecl, TypeKind};
  use serde_json::json;

  fn schema(types: Vec<TypeDecl>, enums: Vec<EnumDecl>) -> SchemaModuleDecl {
    SchemaModuleDecl {
      types,
      enums,
      ..SchemaModuleDecl::default()
    }
  }

  fn schema_type(version: &str) -> TypeDecl {
    TypeDecl {
      version: Some(version.to_string()),
      kind: TypeKind::ElementStruct,
      ..TypeDecl::default()
    }
  }

  fn abstract_schema_type(version: &str) -> TypeDecl {
    TypeDecl {
      version: (!version.is_empty()).then(|| version.to_string()),
      is_abstract: true,
      kind: TypeKind::ElementStruct,
      ..TypeDecl::default()
    }
  }

  fn helper_type(version: &str) -> TypeDecl {
    TypeDecl {
      version: Some(version.to_string()),
      kind: TypeKind::HelperStruct,
      ..TypeDecl::default()
    }
  }

  fn schema_enum(version: &str) -> EnumDecl {
    EnumDecl {
      version: Some(version.to_string()),
      ..EnumDecl::default()
    }
  }

  #[test]
  fn treats_enum_only_microsoft365_schema_modules_as_microsoft365_only() {
    let schema = schema(vec![], vec![schema_enum("Office2016")]);

    assert!(schema_module_is_microsoft365_only_ir(&schema));
  }

  #[test]
  fn does_not_treat_empty_schema_modules_as_microsoft365_only() {
    let schema = schema(vec![], vec![]);

    assert!(!schema_module_is_microsoft365_only_ir(&schema));
  }

  #[test]
  fn does_not_treat_mixed_version_schema_modules_as_microsoft365_only() {
    let schema = schema(
      vec![schema_type("Office2013")],
      vec![schema_enum("Office2007")],
    );

    assert!(!schema_module_is_microsoft365_only_ir(&schema));
  }

  #[test]
  fn ignores_abstract_types_when_detecting_microsoft365_only_schema_modules() {
    let schema = schema(
      vec![schema_type("Office2010"), abstract_schema_type("")],
      vec![],
    );

    assert!(schema_module_is_microsoft365_only_ir(&schema));
  }

  #[test]
  fn does_not_treat_abstract_only_schema_modules_as_microsoft365_only() {
    let schema = schema(vec![abstract_schema_type("")], vec![]);

    assert!(!schema_module_is_microsoft365_only_ir(&schema));
  }

  #[test]
  fn ignores_helper_types_when_detecting_microsoft365_only_schema_modules() {
    let schema = schema(vec![helper_type("Office2013")], vec![]);

    assert!(!schema_module_is_microsoft365_only_ir(&schema));
  }

  #[test]
  fn detects_codegen_ir_schema_json_by_rust_name_keys() {
    let ir_json = json!({
      "ModuleName": "schemas_example",
      "TargetNamespace": "urn:example",
      "Prefix": "ex",
      "TypedNamespace": "DocumentFormat.OpenXml.Example",
      "Enums": [{"RustName": "ExampleEnum"}],
      "Types": [{"RustName": "ExampleType"}]
    });
    let legacy_json = json!({
      "ModuleName": "schemas_example",
      "TargetNamespace": "urn:example",
      "Prefix": "ex",
      "TypedNamespace": "DocumentFormat.OpenXml.Example",
      "Enums": [{"Name": "ExampleEnum"}],
      "Types": [{"ClassName": "ExampleType"}]
    });

    assert!(is_codegen_ir_schema_json(&ir_json));
    assert!(!is_codegen_ir_schema_json(&legacy_json));
  }
}
