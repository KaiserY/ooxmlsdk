use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use serde_json::Value;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use syn::{Attribute, Ident, ItemMod, parse_str, parse2};

use crate::Result;
use crate::sdk_code::codegen_ir::SchemaModuleDecl;
use crate::sdk_code::codegen_ir_builder::build_codegen_ir;
use crate::sdk_code::part_codegen_ir::PartModuleDecl;
use crate::sdk_code::parts::{gen_part_module, gen_parts_mod};
use crate::sdk_code::schemas::{
  CodegenContext, TypeContainmentGraph, gen_schema_from_ir_with_type_graph,
};
use crate::sdk_data::sdk_data_model::Schema as SdkDataSchema;
use crate::utils::escape_snake_case;

pub mod codegen_ir;
pub mod codegen_ir_builder;
pub mod helpers;
pub mod part_codegen_ir;
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

struct LoadedPart {
  ir: PartModuleDecl,
}

pub fn gen_sdk_code<P: AsRef<Path>>(sdk_data_dir: P, out_dir: P) -> Result<()> {
  let sdk_data_schemas_dir_path = sdk_data_dir.as_ref().join("schemas");
  let sdk_data_parts_dir_path = sdk_data_dir.as_ref().join("parts");
  let loaded_schemas = read_schemas(&sdk_data_schemas_dir_path)?;
  let loaded_parts = read_parts(&sdk_data_parts_dir_path)?;
  let out_dir_path = out_dir.as_ref();

  write_schemas(&loaded_schemas, out_dir_path)?;
  write_parts(&loaded_parts, out_dir_path)?;

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

fn read_parts(sdk_data_parts_dir_path: &Path) -> Result<Vec<LoadedPart>> {
  let mut loaded_parts = vec![];

  if !sdk_data_parts_dir_path.exists() {
    return Ok(vec![]);
  }

  for entry in fs::read_dir(sdk_data_parts_dir_path)? {
    let entry = entry?;
    let path = entry.path();

    if !path.is_file() || path.extension() != Some(OsStr::new("json")) {
      continue;
    }

    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let value: Value = serde_json::from_reader(reader)?;
    if !is_codegen_ir_part_json(&value) {
      return Err(
        format!(
          "expected part IR json in {}, found legacy/non-IR shape",
          path.display()
        )
        .into(),
      );
    }
    loaded_parts.push(LoadedPart {
      ir: serde_json::from_value(value)?,
    });
  }

  loaded_parts.sort_by(|a, b| a.ir.module_name.cmp(&b.ir.module_name));
  Ok(loaded_parts)
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
  let mut schema_alias_mod_list: Vec<ItemMod> = vec![];
  let module_names = loaded_schemas
    .iter()
    .map(|loaded| loaded.ir.module_name.as_str())
    .collect::<HashSet<_>>();
  let mut alias_names = HashSet::new();

  for loaded_schema in loaded_schemas {
    let schema_path = out_schemas_dir_path.join(format!("{}.rs", loaded_schema.ir.module_name));
    let schema_tokens = gen_schema_from_ir_with_type_graph(&loaded_schema.ir, false, &schema_graph)
      .map_err(|err| {
        format!(
          "failed to generate schema {}: {err:?}",
          loaded_schema.ir.module_name
        )
      })?;
    let schema_has_public_items = generated_schema_has_public_items(schema_tokens.clone())?;
    write_generated_module(&schema_path, schema_tokens)?;

    push_module_decl(
      &mut schemas_mod_list,
      &loaded_schema.ir.module_name,
      schema_module_cfg_attrs_ir(&loaded_schema.ir),
    )?;

    if let Some(alias_name) = schema_prefix_alias_name(&loaded_schema.ir) {
      if module_names.contains(alias_name.as_str()) || !alias_names.insert(alias_name.clone()) {
        return Err(format!(
          "schema prefix alias {alias_name} for module {} collides with an existing schema module or alias",
          loaded_schema.ir.module_name
        )
        .into());
      }
      push_schema_alias_decl(
        &mut schema_alias_mod_list,
        &alias_name,
        &loaded_schema.ir.module_name,
        schema_has_public_items,
        schema_module_cfg_attrs_ir(&loaded_schema.ir),
      )?;
    }
  }

  let token_stream: TokenStream = quote! {
    #( #schemas_mod_list )*
    #( #schema_alias_mod_list )*
  };
  let schemas_mod_path = out_dir_path.join("schemas.rs");
  write_generated_module(&schemas_mod_path, token_stream)?;

  Ok(())
}

fn write_parts(loaded_parts: &[LoadedPart], out_dir_path: &Path) -> Result<()> {
  let out_parts_dir_path = out_dir_path.join("parts");
  fs::create_dir_all(&out_parts_dir_path)?;
  clear_generated_rs_files(&out_parts_dir_path)?;

  for loaded_part in loaded_parts {
    let part_path = out_parts_dir_path.join(format!("{}.rs", loaded_part.ir.module_name));
    write_generated_module(
      &part_path,
      gen_part_module(&loaded_part.ir).map_err(|err| {
        format!(
          "failed to generate part {}: {err}",
          loaded_part.ir.module_name
        )
      })?,
    )?;
  }
  write_generated_module(
    &out_parts_dir_path.join("extended_part.rs"),
    quote! {
      #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct ExtendedPart {
          pub(crate) relationship_id: Option<String>,
          pub(crate) id: crate::common::PartId,
          pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
          pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
          pub(crate) modeled_relationships: Vec<crate::common::RelationshipInfo>,
        }
      impl crate::sdk::SdkPart for ExtendedPart {
        const RELATIONSHIP_TYPE: &'static str = "";
        const PATH_PREFIX: &'static str = "";
        const CONTENT_TYPE: &'static str = "";
        const TARGET_NAME: &'static str = "extendedPart";
        const EXTENSION: &'static str = "";

        #[inline]
        fn from_part_id(part_id: crate::common::PartId) -> Self {
          Self {
            relationship_id: None,
            id: part_id,
            fallback_parts: Vec::new(),
            relationship_order: Vec::new(),
            modeled_relationships: Vec::new(),
          }
        }

        #[inline]
        fn from_relationship_id(
          relationship_id: impl Into<String>,
          part_id: crate::common::PartId,
        ) -> Self {
          Self {
            relationship_id: Some(relationship_id.into()),
            id: part_id,
            fallback_parts: Vec::new(),
            relationship_order: Vec::new(),
            modeled_relationships: Vec::new(),
          }
        }

        #[inline]
        fn part_id(&self) -> crate::common::PartId {
          self.id
        }

        #[inline]
        fn relationship_id(&self) -> Option<&str> {
          self.relationship_id.as_deref()
        }

        fn set_relationship_id(&mut self, relationship_id: String) {
          self.relationship_id = Some(relationship_id);
        }

      }

      impl crate::sdk::SdkPartInternal for ExtendedPart {
        #[inline]
        fn from_part_id_with_relationships(
          storage: &crate::common::SdkPackageStorage,
          part_id: crate::common::PartId,
        ) -> Self {
          let mut part = <Self as crate::sdk::SdkPart>::from_part_id(part_id);
          if let Some(relationships) = storage.relationships(part_id) {
            for relationship in relationships.iter() {
              if relationship.is_reference_relationship() {
                let item_index = part.modeled_relationships.len();
                part.modeled_relationships.push(relationship.clone());
                part.relationship_order.push(
                  crate::sdk::RelationshipModelEntry::Relationship(item_index),
                );
              } else if relationship.target_kind() == crate::common::RelationshipTargetKind::InternalPart {
                if let Some(child) = crate::parts::PartRef::from_relationship_storage(storage, relationship) {
                  let item_index = part.fallback_parts.len();
                  part.fallback_parts.push(child);
                  part.relationship_order.push(
                    crate::sdk::RelationshipModelEntry::Fallback(item_index),
                  );
                }
              } else {
                let item_index = part.modeled_relationships.len();
                part.modeled_relationships.push(relationship.clone());
                part.relationship_order.push(
                  crate::sdk::RelationshipModelEntry::Relationship(item_index),
                );
              }
            }
          }
          part
        }

        #[inline]
        fn from_relationship_id_with_relationships(
          storage: &crate::common::SdkPackageStorage,
          relationship_id: impl Into<String>,
          part_id: crate::common::PartId,
        ) -> Self {
          let mut part =
            <Self as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(storage, part_id);
          part.relationship_id = Some(relationship_id.into());
          part
        }
      }
    },
  )?;

  write_generated_module(
    &out_dir_path.join("parts.rs"),
    gen_parts_mod(
      &loaded_parts
        .iter()
        .map(|loaded| &loaded.ir)
        .collect::<Vec<_>>(),
    )?,
  )?;

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

fn push_schema_alias_decl(
  mod_list: &mut Vec<ItemMod>,
  alias_name: &str,
  module_name: &str,
  has_public_items: bool,
  cfg_attrs: Vec<Attribute>,
) -> Result<()> {
  let alias_ident: Ident = parse_str(alias_name)?;
  let module_ident: Ident = parse_str(module_name)?;
  let reexport = if has_public_items {
    quote! {
      pub use super::#module_ident::*;
    }
  } else {
    quote! {}
  };
  mod_list.push(parse2(quote! {
    #( #cfg_attrs )*
    pub mod #alias_ident {
      #reexport
    }
  })?);
  Ok(())
}

fn generated_schema_has_public_items(token_stream: TokenStream) -> Result<bool> {
  let syntax_tree: syn::File = parse2(token_stream)?;
  Ok(syntax_tree.items.iter().any(|item| match item {
    syn::Item::Const(item) => matches!(item.vis, syn::Visibility::Public(_)),
    syn::Item::Enum(item) => matches!(item.vis, syn::Visibility::Public(_)),
    syn::Item::Fn(item) => matches!(item.vis, syn::Visibility::Public(_)),
    syn::Item::Mod(item) => matches!(item.vis, syn::Visibility::Public(_)),
    syn::Item::Static(item) => matches!(item.vis, syn::Visibility::Public(_)),
    syn::Item::Struct(item) => matches!(item.vis, syn::Visibility::Public(_)),
    syn::Item::Trait(item) => matches!(item.vis, syn::Visibility::Public(_)),
    syn::Item::TraitAlias(item) => matches!(item.vis, syn::Visibility::Public(_)),
    syn::Item::Type(item) => matches!(item.vis, syn::Visibility::Public(_)),
    syn::Item::Union(item) => matches!(item.vis, syn::Visibility::Public(_)),
    syn::Item::Use(item) => matches!(item.vis, syn::Visibility::Public(_)),
    _ => false,
  }))
}

fn schema_prefix_alias_name(schema: &SchemaModuleDecl) -> Option<String> {
  let prefix = schema.prefix.trim();
  if prefix.is_empty() {
    return None;
  }

  Some(escape_snake_case(prefix.to_snake_case()))
}

fn schema_module_cfg_attrs_ir(schema: &SchemaModuleDecl) -> Vec<Attribute> {
  let _ = schema;
  Vec::new()
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

fn is_codegen_ir_part_json(value: &Value) -> bool {
  value.get("StructName").is_some() && value.get("Fields").is_some()
}
