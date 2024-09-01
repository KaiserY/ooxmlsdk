use gen::simple_type::gen_simple_type;
use heck::ToSnakeCase;
use models::OpenXmlSchemaEnum;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::fs::File;
use std::{fs, path::Path};
use syn::{parse_str, Ident, ItemMod};

use crate::gen::context::GenContext;
use crate::gen::open_xml_part::gen_open_xml_part;
use crate::gen::open_xml_schema::gen_open_xml_schema;
use crate::models::{OpenXmlNamespace, OpenXmlPart, OpenXmlSchema, OpenXmlSchemaType};

pub mod gen;
pub mod models;
pub mod utils;

pub fn gen(data_dir: &str, out_dir: &str) {
  let out_dir_path = Path::new(out_dir);
  let out_parts_dir_path = &out_dir_path.join("parts");
  let out_schemas_dir_path = &out_dir_path.join("schemas");

  let data_dir_path = Path::new(data_dir);
  let data_parts_dir_path = &data_dir_path.join("parts");
  let data_schemas_dir_path = &data_dir_path.join("schemas");

  fs::create_dir_all(out_parts_dir_path).unwrap();
  fs::create_dir_all(out_schemas_dir_path).unwrap();

  let mut parts: Vec<OpenXmlPart> = vec![];
  let mut part_mods: Vec<String> = vec![];
  let mut schemas: Vec<OpenXmlSchema> = vec![];
  let mut schema_mods: Vec<String> = vec![];
  let prefix_namespace_map: HashMap<&str, &OpenXmlNamespace> = HashMap::new();
  let uri_namespace_map: HashMap<&str, &OpenXmlNamespace> = HashMap::new();
  let prefix_schema_mod_map: HashMap<&str, &str> = HashMap::new();
  let uri_schema_mod_map: HashMap<&str, &str> = HashMap::new();
  let type_name_type_map: HashMap<&str, &OpenXmlSchemaType> = HashMap::new();
  let type_name_namespace_map: HashMap<&str, &OpenXmlNamespace> = HashMap::new();
  let type_base_class_type_map: HashMap<&str, &OpenXmlSchemaType> = HashMap::new();
  let enum_type_enum_map: HashMap<&str, &OpenXmlSchemaEnum> = HashMap::new();
  let enum_type_namespace_map: HashMap<&str, &OpenXmlNamespace> = HashMap::new();
  let enum_name_enum_map: HashMap<&str, &OpenXmlSchemaEnum> = HashMap::new();

  for entry in fs::read_dir(data_parts_dir_path).unwrap() {
    let entry = entry.unwrap();

    let file = File::open(entry.path()).unwrap();

    let open_xml_part: OpenXmlPart = serde_json::from_reader(file).unwrap();

    parts.push(open_xml_part);

    let part_mod = entry
      .path()
      .file_stem()
      .unwrap()
      .to_string_lossy()
      .to_snake_case();

    part_mods.push(part_mod);
  }

  for entry in fs::read_dir(data_schemas_dir_path).unwrap() {
    let entry = entry.unwrap();

    let file = File::open(entry.path()).unwrap();

    let open_xml_schema: OpenXmlSchema = serde_json::from_reader(file).unwrap();

    schemas.push(open_xml_schema);

    let schema_mod = entry
      .path()
      .file_stem()
      .unwrap()
      .to_string_lossy()
      .to_snake_case();

    schema_mods.push(schema_mod);
  }

  let file = File::open(data_dir_path.join("namespaces.json")).unwrap();

  let namespaces: Vec<OpenXmlNamespace> = serde_json::from_reader(file).unwrap();

  let mut context = GenContext {
    parts,
    schemas,
    namespaces,
    schema_mods,
    part_mods,
    prefix_namespace_map,
    uri_namespace_map,
    prefix_schema_mod_map,
    uri_schema_mod_map,
    type_name_type_map,
    type_name_namespace_map,
    type_base_class_type_map,
    enum_type_enum_map,
    enum_type_namespace_map,
    enum_name_enum_map,
  };

  for namespace in context.namespaces.iter() {
    context
      .prefix_namespace_map
      .insert(&namespace.prefix, namespace);

    context.uri_namespace_map.insert(&namespace.uri, namespace);
  }

  for (i, schema) in context.schemas.iter().enumerate() {
    let namespace = context
      .uri_namespace_map
      .get(schema.target_namespace.as_str())
      .ok_or(format!("{:?}", schema.target_namespace))
      .unwrap();

    let schema_mod = &context.schema_mods[i];

    context
      .prefix_schema_mod_map
      .insert(&namespace.prefix, schema_mod);

    context
      .uri_schema_mod_map
      .insert(&namespace.uri, schema_mod);

    for ty in schema.types.iter() {
      context.type_name_type_map.insert(&ty.name, ty);

      context.type_name_namespace_map.insert(&ty.name, namespace);

      context.type_base_class_type_map.insert(&ty.class_name, ty);
    }

    for e in schema.enums.iter() {
      context.enum_type_enum_map.insert(&e.r#type, e);

      context.enum_type_namespace_map.insert(&e.r#type, namespace);

      context.enum_name_enum_map.insert(&e.name, e);
    }
  }

  let mut shcemas_mod_use_list: Vec<ItemMod> = vec![];

  for (i, schema) in context.schemas.iter().enumerate() {
    let schema_mod = &context.schema_mods[i];

    let token_stream = gen_open_xml_schema(schema, &context);

    let syntax_tree = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    let schema_path = out_schemas_dir_path.join(format!("{}.rs", schema_mod));

    fs::write(schema_path, formatted).unwrap();
  }

  let token_stream: TokenStream = gen_simple_type().unwrap();

  let syntax_tree = syn::parse2(token_stream).unwrap();
  let formatted = prettyplease::unparse(&syntax_tree);

  let schemas_mod_path = out_schemas_dir_path.join("simple_type.rs");

  fs::write(schemas_mod_path, formatted).unwrap();

  for schema_mod in context.schema_mods.iter() {
    let schema_mod_ident: Ident = parse_str(schema_mod).unwrap();

    let shcemas_mod_use: ItemMod = parse_str(
      &quote! {
        pub mod #schema_mod_ident;
      }
      .to_string(),
    )
    .unwrap();

    shcemas_mod_use_list.push(shcemas_mod_use);
  }

  let token_stream: TokenStream = quote! {
    pub mod simple_type;
    #( #shcemas_mod_use_list )*
  };

  let syntax_tree = syn::parse2(token_stream).unwrap();
  let formatted = prettyplease::unparse(&syntax_tree);

  let schemas_mod_path = out_schemas_dir_path.join("mod.rs");

  fs::write(schemas_mod_path, formatted).unwrap();

  let mut parts_mod_use_list: Vec<ItemMod> = vec![];

  for (i, part) in context.parts.iter().enumerate() {
    let part_mod = &context.part_mods[i];

    let token_stream = gen_open_xml_part(part, &context).unwrap();

    let syntax_tree = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    let part_path = out_parts_dir_path.join(format!("{}.rs", part_mod));

    fs::write(part_path, formatted).unwrap();
  }

  for part_mod in context.part_mods.iter() {
    let part_mod_ident: Ident = parse_str(part_mod).unwrap();

    let part_mod_use: ItemMod = parse_str(
      &quote! {
        pub mod #part_mod_ident;
      }
      .to_string(),
    )
    .unwrap();

    parts_mod_use_list.push(part_mod_use);
  }

  let token_stream: TokenStream = quote! {
    #( #parts_mod_use_list )*
  };

  let syntax_tree = syn::parse2(token_stream).unwrap();
  let formatted = prettyplease::unparse(&syntax_tree);

  let schemas_mod_path = out_parts_dir_path.join("mod.rs");

  fs::write(schemas_mod_path, formatted).unwrap();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_gen() {
    gen("../ooxmlsdk/data", "src");
  }
}
