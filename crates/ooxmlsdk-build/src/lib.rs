use heck::ToSnakeCase;
use models::OpenXmlSchemaEnum;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::fs::File;
use std::{fs, path::Path};
use syn::{Ident, ItemMod, parse_str};

use crate::generator::context::{GenContext, GenContextNeo};
use crate::generator::deserializer::{gen_deserializer, gen_deserializers_neo};
use crate::generator::open_xml_part::{gen_open_xml_part, gen_open_xml_parts_neo};
use crate::generator::open_xml_schema::{gen_open_xml_schema, gen_open_xml_schemas_neo};
use crate::generator::serializer::{gen_serializer, gen_serializer_neo};
use crate::generator::validator::{gen_validator, gen_validators_neo};
use crate::models::{
  OpenXmlNamespace, OpenXmlPart, OpenXmlSchema, OpenXmlSchemaType, TypedNamespace,
};

pub mod generator;
pub mod includes;
pub mod models;
pub mod utils;

// pub mod common;
// pub mod deserializers;
// pub mod schemas;

pub fn generate_neo(data_dir: &str, out_dir: &str) {
  let out_dir_path = Path::new(out_dir);

  let mut gen_context = GenContextNeo::new(data_dir);

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
    }
  }

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

pub(crate) fn write_schemas(gen_context: &GenContextNeo, out_dir_path: &Path) {
  let out_schemas_dir_path = out_dir_path.join("schemas");
  let out_common_dir_path = out_dir_path.join("common");

  fs::create_dir_all(&out_schemas_dir_path).unwrap();
  fs::create_dir_all(&out_common_dir_path).unwrap();

  let mut schemas_mod_use_list: Vec<ItemMod> = vec![];

  for schema in gen_context.schemas.iter() {
    let token_stream = gen_open_xml_schemas_neo(schema, gen_context);
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

pub(crate) fn write_deserializers(gen_context: &GenContextNeo, out_dir_path: &Path) {
  let out_deserializers_dir_path = &out_dir_path.join("deserializers");

  fs::create_dir_all(out_deserializers_dir_path).unwrap();

  let mut deserializers_mod_use_list: Vec<ItemMod> = vec![];

  for schema in gen_context.schemas.iter() {
    let token_stream = gen_deserializers_neo(schema, gen_context);
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

pub(crate) fn write_serializers(gen_context: &GenContextNeo, out_dir_path: &Path) {
  let out_serializers_dir_path = &out_dir_path.join("serializers");

  fs::create_dir_all(out_serializers_dir_path).unwrap();

  let mut serializers_mod_use_list: Vec<ItemMod> = vec![];

  for schema in gen_context.schemas.iter() {
    let token_stream = gen_serializer_neo(schema, gen_context);
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

pub(crate) fn write_parts(gen_context: &GenContextNeo, out_dir_path: &Path) {
  let out_parts_dir_path = &out_dir_path.join("parts");

  fs::create_dir_all(out_parts_dir_path).unwrap();

  let mut parts_mod_use_list: Vec<ItemMod> = vec![];

  for part in gen_context.parts.iter() {
    let token_stream = gen_open_xml_parts_neo(part, gen_context);
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

pub(crate) fn write_validators(gen_context: &GenContextNeo, out_dir_path: &Path) {
  let out_validators_dir_path = &out_dir_path.join("validators");

  fs::create_dir_all(out_validators_dir_path).unwrap();

  let mut validators_mod_use_list: Vec<ItemMod> = vec![];

  for part in gen_context.schemas.iter() {
    let token_stream = gen_validators_neo(part, gen_context);
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

pub fn generate(data_dir: &str, out_dir: &str) {
  let out_dir_path = Path::new(out_dir);
  let out_parts_dir_path = &out_dir_path.join("parts");
  let out_schemas_dir_path = &out_dir_path.join("schemas");
  let out_deserializers_dir_path = &out_dir_path.join("deserializers");
  let out_serializers_dir_path = &out_dir_path.join("serializers");
  let out_common_dir_path = &out_dir_path.join("common");
  let out_validators_dir_path = &out_dir_path.join("validators");
  let out_packages_dir_path = &out_dir_path.join("packages");

  let data_dir_path = Path::new(data_dir);
  let data_parts_dir_path = &data_dir_path.join("parts");
  let data_schemas_dir_path = &data_dir_path.join("schemas");

  fs::create_dir_all(out_parts_dir_path).unwrap();
  fs::create_dir_all(out_schemas_dir_path).unwrap();
  fs::create_dir_all(out_deserializers_dir_path).unwrap();
  fs::create_dir_all(out_serializers_dir_path).unwrap();
  fs::create_dir_all(out_common_dir_path).unwrap();
  fs::create_dir_all(out_validators_dir_path).unwrap();
  fs::create_dir_all(out_packages_dir_path).unwrap();

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
  let enum_type_enum_map: HashMap<&str, &OpenXmlSchemaEnum> = HashMap::new();
  let enum_type_namespace_map: HashMap<&str, &OpenXmlNamespace> = HashMap::new();
  let enum_name_enum_map: HashMap<&str, &OpenXmlSchemaEnum> = HashMap::new();
  let part_name_type_map: HashMap<&str, &OpenXmlSchemaType> = HashMap::new();
  let prefix_schema_map: HashMap<&str, &OpenXmlSchema> = HashMap::new();
  let part_name_part_map: HashMap<&str, &OpenXmlPart> = HashMap::new();
  let part_name_part_mod_map: HashMap<&str, &str> = HashMap::new();
  let target_type_map: HashMap<String, &OpenXmlSchemaType> = HashMap::new();

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

  let file = File::open(data_dir_path.join("typed").join("namespaces.json")).unwrap();

  let typed_namespaces: Vec<TypedNamespace> = serde_json::from_reader(file).unwrap();

  let mut context = GenContext {
    parts,
    schemas,
    namespaces,
    typed_namespaces,
    schema_mods,
    part_mods,
    prefix_namespace_map,
    uri_namespace_map,
    prefix_schema_mod_map,
    uri_schema_mod_map,
    type_name_type_map,
    type_name_namespace_map,
    enum_type_enum_map,
    enum_type_namespace_map,
    enum_name_enum_map,
    part_name_type_map,
    prefix_schema_map,
    part_name_part_map,
    part_name_part_mod_map,
    target_type_map,
  };

  for namespace in context.namespaces.iter() {
    context
      .prefix_namespace_map
      .insert(&namespace.prefix, namespace);

    context.uri_namespace_map.insert(&namespace.uri, namespace);
  }

  for (i, part) in context.parts.iter().enumerate() {
    context.part_name_part_map.insert(&part.name, part);

    let part_mod = &context.part_mods[i];

    context.part_name_part_mod_map.insert(&part.name, part_mod);
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

      if !ty.part.is_empty() {
        context.part_name_type_map.insert(&ty.part, ty);
      }

      if ty.base_class == "OpenXmlPartRootElement" {
        context.target_type_map.insert(
          ty.name[ty.name.rfind(':').unwrap() + 1..ty.name.len()].to_string(),
          ty,
        );
      }
    }

    for e in schema.enums.iter() {
      context.enum_type_enum_map.insert(&e.r#type, e);

      context.enum_type_namespace_map.insert(&e.r#type, namespace);

      context.enum_name_enum_map.insert(&e.name, e);
    }

    context.prefix_schema_map.insert(&namespace.prefix, schema);
  }

  let mut schemas_mod_use_list: Vec<ItemMod> = vec![];

  for (i, schema) in context.schemas.iter().enumerate() {
    let schema_mod = &context.schema_mods[i];

    let token_stream = gen_open_xml_schema(schema, &context);

    let syntax_tree = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    let schema_path = out_schemas_dir_path.join(format!("{}.rs", schema_mod));

    fs::write(schema_path, formatted).unwrap();
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

  for schema_mod in context.schema_mods.iter() {
    let schema_mod_ident: Ident = parse_str(schema_mod).unwrap();

    let shcemas_mod_use: ItemMod = parse_str(
      &quote! {
        pub mod #schema_mod_ident;
      }
      .to_string(),
    )
    .unwrap();

    schemas_mod_use_list.push(shcemas_mod_use);
  }

  let token_stream: TokenStream = quote! {
    pub mod simple_type;
    #( #schemas_mod_use_list )*
  };

  let syntax_tree = syn::parse2(token_stream).unwrap();
  let formatted = prettyplease::unparse(&syntax_tree);

  let schemas_mod_path = out_schemas_dir_path.join("mod.rs");

  fs::write(schemas_mod_path, formatted).unwrap();

  let mut parts_mod_use_list: Vec<ItemMod> = vec![];

  for (i, part) in context.parts.iter().enumerate() {
    let part_mod = &context.part_mods[i];

    let token_stream = gen_open_xml_part(part, &context);

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

  let parts_mod_path = out_parts_dir_path.join("mod.rs");

  fs::write(parts_mod_path, formatted).unwrap();

  let mut deserializers_mod_use_list: Vec<ItemMod> = vec![];

  for (i, part) in context.schemas.iter().enumerate() {
    let schema_mod = &context.schema_mods[i];

    let token_stream = gen_deserializer(part, &context);

    let syntax_tree = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    let part_path = out_deserializers_dir_path.join(format!("{}.rs", schema_mod));

    fs::write(part_path, formatted).unwrap();
  }

  for schema_mod in context.schema_mods.iter() {
    let deserializer_mod_ident: Ident = parse_str(schema_mod).unwrap();

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

  let mut serializers_mod_use_list: Vec<ItemMod> = vec![];

  for (i, part) in context.schemas.iter().enumerate() {
    let schema_mod = &context.schema_mods[i];

    let token_stream = gen_serializer(part, &context);

    let syntax_tree = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    let part_path = out_serializers_dir_path.join(format!("{}.rs", schema_mod));

    fs::write(part_path, formatted).unwrap();
  }

  for schema_mod in context.schema_mods.iter() {
    let serializer_mod_ident: Ident = parse_str(schema_mod).unwrap();

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

  let mut validators_mod_use_list: Vec<ItemMod> = vec![];

  for (i, part) in context.schemas.iter().enumerate() {
    let schema_mod = &context.schema_mods[i];

    let token_stream = gen_validator(part, &context);

    let syntax_tree = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);

    let part_path = out_validators_dir_path.join(format!("{}.rs", schema_mod));

    fs::write(part_path, formatted).unwrap();
  }

  for schema_mod in context.schema_mods.iter() {
    let validator_mod_ident: Ident = parse_str(schema_mod).unwrap();

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

  let token_stream: TokenStream = parse_str(include_str!("includes/packages/mod.rs")).unwrap();

  let syntax_tree = syn::parse2(token_stream).unwrap();
  let formatted = prettyplease::unparse(&syntax_tree);

  let schemas_mod_path = out_packages_dir_path.join("mod.rs");

  fs::write(schemas_mod_path, formatted).unwrap();

  let token_stream: TokenStream =
    parse_str(include_str!("includes/packages/opc_content_types.rs")).unwrap();

  let syntax_tree = syn::parse2(token_stream).unwrap();
  let formatted = prettyplease::unparse(&syntax_tree);

  let schemas_mod_path = out_packages_dir_path.join("opc_content_types.rs");

  fs::write(schemas_mod_path, formatted).unwrap();

  let token_stream: TokenStream =
    parse_str(include_str!("includes/packages/opc_relationships.rs")).unwrap();

  let syntax_tree = syn::parse2(token_stream).unwrap();
  let formatted = prettyplease::unparse(&syntax_tree);

  let schemas_mod_path = out_packages_dir_path.join("opc_relationships.rs");

  fs::write(schemas_mod_path, formatted).unwrap();

  let token_stream: TokenStream =
    parse_str(include_str!("includes/packages/opc_core_properties.rs")).unwrap();

  let syntax_tree = syn::parse2(token_stream).unwrap();
  let formatted = prettyplease::unparse(&syntax_tree);

  let schemas_mod_path = out_packages_dir_path.join("opc_core_properties.rs");

  fs::write(schemas_mod_path, formatted).unwrap();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_gen() {
    generate("../ooxmlsdk/data", "src");
  }

  #[test]
  fn test_gen_new() {
    generate_neo("../ooxmlsdk/data", "src");
  }
}
