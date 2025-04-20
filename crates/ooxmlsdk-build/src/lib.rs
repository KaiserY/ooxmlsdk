use proc_macro2::TokenStream;
use quote::quote;
use std::{fs, path::Path};
use syn::{Ident, ItemMod, parse_str};

use crate::generator::context::GenContext;
use crate::generator::deserializer::gen_deserializers;
use crate::generator::open_xml_part::gen_open_xml_parts;
use crate::generator::open_xml_schema::gen_open_xml_schemas;
use crate::generator::serializer::gen_serializer;
use crate::generator::validator::gen_validators;

pub mod generator;
pub mod includes;
pub mod models;
pub mod utils;

pub fn generate(data_dir: &str, out_dir: &str) {
  let out_dir_path = Path::new(out_dir);

  let mut gen_context = GenContext::new(data_dir);

  for namespace in gen_context.namespaces.iter() {
    gen_context
      .prefix_namespace_map
      .insert(&namespace.prefix, namespace);

    gen_context
      .uri_namespace_map
      .insert(&namespace.uri, namespace);
  }

  for typed_namespace in gen_context.typed_namespaces.iter() {
    gen_context
      .namespace_typed_namespace_map
      .insert(&typed_namespace.namespace, typed_namespace);
  }

  for typed_schema in gen_context.typed_schemas.iter() {
    for ty in typed_schema.iter() {
      if !ty.part_class_name.is_empty() {
        gen_context
          .part_name_type_name_map
          .insert(&ty.part_class_name, &ty.name);
      }
    }
  }

  for schema in gen_context.schemas.iter() {
    let namespace = gen_context
      .uri_namespace_map
      .get(schema.target_namespace.as_str())
      .ok_or(format!("{:?}", schema.target_namespace))
      .unwrap();

    gen_context
      .prefix_schema_map
      .insert(&namespace.prefix, schema);

    for e in schema.enums.iter() {
      gen_context.enum_type_enum_map.insert(&e.r#type, e);

      gen_context
        .enum_type_namespace_map
        .insert(&e.r#type, namespace);
    }

    for ty in schema.types.iter() {
      gen_context.type_name_type_map.insert(&ty.name, ty);

      gen_context
        .type_name_namespace_map
        .insert(&ty.name, namespace);

      if !ty.part.is_empty() {
        gen_context
          .part_name_type_name_map
          .insert(&ty.part, &ty.name);
      }
    }
  }

  gen_context
    .part_name_type_name_map
    .insert("StyleDefinitionsPart", "w:CT_Styles/w:styles");
  gen_context
    .part_name_type_name_map
    .insert("StylesWithEffectsPart", "w:CT_Styles/w:styles");

  write_schemas(&gen_context, out_dir_path);

  write_deserializers(&gen_context, out_dir_path);

  write_serializers(&gen_context, out_dir_path);

  #[cfg(feature = "parts")]
  let with_parts = true;
  #[cfg(not(feature = "parts"))]
  let with_parts = false;

  if with_parts {
    write_parts(&gen_context, out_dir_path);
  }

  #[cfg(feature = "validators")]
  let with_validators = true;
  #[cfg(not(feature = "validators"))]
  let with_validators = false;

  if with_validators {
    write_validators(&gen_context, out_dir_path);
  }
}

pub(crate) fn write_schemas(gen_context: &GenContext, out_dir_path: &Path) {
  let out_schemas_dir_path = out_dir_path.join("schemas");
  let out_common_dir_path = out_dir_path.join("common");

  fs::create_dir_all(&out_schemas_dir_path).unwrap();
  fs::create_dir_all(&out_common_dir_path).unwrap();

  let mut schemas_mod_use_list: Vec<ItemMod> = vec![];

  for schema in gen_context.schemas.iter() {
    let token_stream = gen_open_xml_schemas(schema, gen_context);
    let syntax_tree = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    let schema_path = out_schemas_dir_path.join(format!("{}.rs", &schema.module_name));
    fs::write(schema_path, formatted).unwrap();

    let schema_mod_ident: Ident = parse_str(&schema.module_name).unwrap();
    let shcemas_mod_use: ItemMod = parse_str(
      &quote! {
        pub mod #schema_mod_ident;
      }
      .to_string(),
    )
    .unwrap();
    schemas_mod_use_list.push(shcemas_mod_use);
  }

  let token_stream: TokenStream = parse_str(include_str!("includes/simple_type.rs")).unwrap();
  let syntax_tree = syn::parse2(token_stream).unwrap();
  let formatted = prettyplease::unparse(&syntax_tree);
  let schemas_mod_path = out_schemas_dir_path.join("simple_type.rs");
  fs::write(schemas_mod_path, formatted).unwrap();

  let token_stream: TokenStream = parse_str(include_str!("includes/common.rs")).unwrap();
  let syntax_tree = syn::parse2(token_stream).unwrap();
  let formatted = prettyplease::unparse(&syntax_tree);
  let schemas_mod_path = out_common_dir_path.join("mod.rs");
  fs::write(schemas_mod_path, formatted).unwrap();

  let token_stream: TokenStream =
    parse_str(include_str!("includes/packages/opc_content_types.rs")).unwrap();
  let syntax_tree = syn::parse2(token_stream).unwrap();
  let formatted = prettyplease::unparse(&syntax_tree);
  let schemas_mod_path = out_schemas_dir_path.join("opc_content_types.rs");
  fs::write(schemas_mod_path, formatted).unwrap();

  let token_stream: TokenStream =
    parse_str(include_str!("includes/packages/opc_relationships.rs")).unwrap();
  let syntax_tree = syn::parse2(token_stream).unwrap();
  let formatted = prettyplease::unparse(&syntax_tree);
  let schemas_mod_path = out_schemas_dir_path.join("opc_relationships.rs");
  fs::write(schemas_mod_path, formatted).unwrap();

  let token_stream: TokenStream =
    parse_str(include_str!("includes/packages/opc_core_properties.rs")).unwrap();
  let syntax_tree = syn::parse2(token_stream).unwrap();
  let formatted = prettyplease::unparse(&syntax_tree);
  let schemas_mod_path = out_schemas_dir_path.join("opc_core_properties.rs");
  fs::write(schemas_mod_path, formatted).unwrap();

  let token_stream: TokenStream = quote! {
    pub mod simple_type;
    pub mod opc_content_types;
    pub mod opc_core_properties;
    pub mod opc_relationships;
    #( #schemas_mod_use_list )*
  };
  let syntax_tree = syn::parse2(token_stream).unwrap();
  let formatted = prettyplease::unparse(&syntax_tree);
  let schemas_mod_path = out_schemas_dir_path.join("mod.rs");
  fs::write(schemas_mod_path, formatted).unwrap();
}

pub(crate) fn write_deserializers(gen_context: &GenContext, out_dir_path: &Path) {
  let out_deserializers_dir_path = &out_dir_path.join("deserializers");

  fs::create_dir_all(out_deserializers_dir_path).unwrap();

  let mut deserializers_mod_use_list: Vec<ItemMod> = vec![];

  for schema in gen_context.schemas.iter() {
    let token_stream = gen_deserializers(schema, gen_context);
    let syntax_tree = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    let part_path = out_deserializers_dir_path.join(format!("{}.rs", &schema.module_name));
    fs::write(part_path, formatted).unwrap();

    let deserializer_mod_ident: Ident = parse_str(&schema.module_name).unwrap();
    let deserializer_mod_use: ItemMod = parse_str(
      &quote! {
        pub mod #deserializer_mod_ident;
      }
      .to_string(),
    )
    .unwrap();
    deserializers_mod_use_list.push(deserializer_mod_use);
  }

  let token_stream: TokenStream = quote! {
    #( #deserializers_mod_use_list )*
  };
  let syntax_tree = syn::parse2(token_stream).unwrap();
  let formatted = prettyplease::unparse(&syntax_tree);
  let deserializers_mod_path = out_deserializers_dir_path.join("mod.rs");
  fs::write(deserializers_mod_path, formatted).unwrap();
}

pub(crate) fn write_serializers(gen_context: &GenContext, out_dir_path: &Path) {
  let out_serializers_dir_path = &out_dir_path.join("serializers");

  fs::create_dir_all(out_serializers_dir_path).unwrap();

  let mut serializers_mod_use_list: Vec<ItemMod> = vec![];

  for schema in gen_context.schemas.iter() {
    let token_stream = gen_serializer(schema, gen_context);
    let syntax_tree = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    let part_path = out_serializers_dir_path.join(format!("{}.rs", &schema.module_name));
    fs::write(part_path, formatted).unwrap();

    let serializer_mod_ident: Ident = parse_str(&schema.module_name).unwrap();
    let serializer_mod_use: ItemMod = parse_str(
      &quote! {
        pub mod #serializer_mod_ident;
      }
      .to_string(),
    )
    .unwrap();
    serializers_mod_use_list.push(serializer_mod_use);
  }

  let token_stream: TokenStream = quote! {
    #( #serializers_mod_use_list )*
  };
  let syntax_tree = syn::parse2(token_stream).unwrap();
  let formatted = prettyplease::unparse(&syntax_tree);
  let serializers_mod_path = out_serializers_dir_path.join("mod.rs");
  fs::write(serializers_mod_path, formatted).unwrap();
}

pub(crate) fn write_parts(gen_context: &GenContext, out_dir_path: &Path) {
  let out_parts_dir_path = &out_dir_path.join("parts");

  fs::create_dir_all(out_parts_dir_path).unwrap();

  let mut parts_mod_use_list: Vec<ItemMod> = vec![];

  for part in gen_context.parts.iter() {
    let token_stream = gen_open_xml_parts(part, gen_context);
    let syntax_tree = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    let part_path = out_parts_dir_path.join(format!("{}.rs", &part.module_name));
    fs::write(part_path, formatted).unwrap();

    let part_mod_ident: Ident = parse_str(&part.module_name).unwrap();
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

  let parts_mod_path = out_parts_dir_path.join("mod.rs");

  fs::write(parts_mod_path, formatted).unwrap();
}

pub(crate) fn write_validators(gen_context: &GenContext, out_dir_path: &Path) {
  let out_validators_dir_path = &out_dir_path.join("validators");

  fs::create_dir_all(out_validators_dir_path).unwrap();

  let mut validators_mod_use_list: Vec<ItemMod> = vec![];

  for part in gen_context.schemas.iter() {
    let token_stream = gen_validators(part, gen_context);
    let syntax_tree = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    let part_path = out_validators_dir_path.join(format!("{}.rs", &part.module_name));
    fs::write(part_path, formatted).unwrap();

    let validator_mod_ident: Ident = parse_str(&part.module_name).unwrap();
    let validator_mod_use: ItemMod = parse_str(
      &quote! {
        pub mod #validator_mod_ident;
      }
      .to_string(),
    )
    .unwrap();
    validators_mod_use_list.push(validator_mod_use);
  }

  let token_stream: TokenStream = quote! {
    #( #validators_mod_use_list )*
  };
  let syntax_tree = syn::parse2(token_stream).unwrap();
  let formatted = prettyplease::unparse(&syntax_tree);
  let validators_mod_path = out_validators_dir_path.join("mod.rs");
  fs::write(validators_mod_path, formatted).unwrap();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_gen() {
    generate("../ooxmlsdk/data", "src");
  }
}
