use proc_macro2::TokenStream;
use quote::quote;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use syn::{Arm, Ident, ItemMod, parse_str, parse2};

use crate::sdk_code::deserializer::gen_schema_deserializer;
use crate::sdk_code::package_schemas::gen_package_schema;
use crate::sdk_code::parts::{gen_part_module, gen_parts_mod};
use crate::sdk_code::schemas::{CodegenContext, gen_schema};
use crate::sdk_code::serializer::gen_schema_serializer;
use crate::sdk_data::sdk_data_model::{
  Namespace as SdkDataNamespace, PackageSchema as SdkDataPackageSchema, Part as SdkDataPart,
  Schema as SdkDataSchema,
};

pub mod deserializer;
pub mod helpers;
pub mod package_schemas;
pub mod parts;
pub mod schemas;
pub mod serializer;

const FILE_HEADER: &str = r#"//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//
"#;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, BoxError>;

pub fn gen_sdk_code<P: AsRef<Path>>(sdk_data_dir: P, out_dir: P) -> Result<()> {
  let sdk_data_schemas_dir_path = sdk_data_dir.as_ref().join("schemas");
  let sdk_data_package_schemas_dir_path = sdk_data_dir.as_ref().join("package_schemas");
  let sdk_data_parts_dir_path = sdk_data_dir.as_ref().join("parts");
  let sdk_data_schemas = read_schemas(&sdk_data_schemas_dir_path)?;
  let sdk_data_package_schemas = read_package_schemas(&sdk_data_package_schemas_dir_path)?;
  let sdk_data_parts = read_parts(&sdk_data_parts_dir_path)?;
  let sdk_data_namespaces = read_namespaces(sdk_data_dir.as_ref().join("namespaces.json"))?;
  let out_dir_path = out_dir.as_ref();
  let context = CodegenContext::new(&sdk_data_schemas);

  write_schemas(
    &sdk_data_schemas,
    &sdk_data_package_schemas,
    &context,
    out_dir_path,
  )?;
  write_deserializers(&sdk_data_schemas, &context, out_dir_path)?;
  write_serializers(&sdk_data_schemas, &context, out_dir_path)?;
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

    let file = File::open(path)?;
    let reader = BufReader::new(file);
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

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let sdk_data_part: SdkDataPart = serde_json::from_reader(reader)?;
    sdk_data_parts.push(sdk_data_part);
  }

  sdk_data_parts.sort_by(|a, b| a.module_name.cmp(&b.module_name));
  Ok(sdk_data_parts)
}

fn read_package_schemas(
  sdk_data_package_schemas_dir_path: &Path,
) -> Result<Vec<SdkDataPackageSchema>> {
  let mut sdk_data_package_schemas = vec![];

  if !sdk_data_package_schemas_dir_path.exists() {
    return Ok(sdk_data_package_schemas);
  }

  for entry in fs::read_dir(sdk_data_package_schemas_dir_path)? {
    let entry = entry?;
    let path = entry.path();

    if !path.is_file() || path.extension() != Some(OsStr::new("json")) {
      continue;
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let package_schema: SdkDataPackageSchema = serde_json::from_reader(reader)?;
    sdk_data_package_schemas.push(package_schema);
  }

  sdk_data_package_schemas.sort_by(|a, b| a.module_name.cmp(&b.module_name));
  Ok(sdk_data_package_schemas)
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
  sdk_data_package_schemas: &[SdkDataPackageSchema],
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
      gen_schema(sdk_data_schema, context).map_err(|err| {
        format!(
          "failed to generate schema {}: {err}",
          sdk_data_schema.module_name
        )
      })?,
    )?;

    push_module_decl(&mut schemas_mod_list, &sdk_data_schema.module_name)?;
  }

  for package_schema in sdk_data_package_schemas {
    let schema_path = out_schemas_dir_path.join(format!("{}.rs", package_schema.module_name));
    write_generated_module(
      &schema_path,
      gen_package_schema(package_schema).map_err(|err| {
        format!(
          "failed to generate package schema {}: {err}",
          package_schema.module_name
        )
      })?,
    )?;

    push_module_decl(&mut schemas_mod_list, &package_schema.module_name)?;
  }

  write_generated_module(
    &out_schemas_dir_path.join("simple_type.rs"),
    quote! {
      pub use crate::simple_type::*;
    },
  )?;

  let token_stream: TokenStream = quote! {
    pub mod simple_type;
    #( #schemas_mod_list )*
  };
  let schemas_mod_path = out_dir_path.join("schemas.rs");
  write_generated_module(&schemas_mod_path, token_stream)?;

  Ok(())
}

fn write_deserializers(
  sdk_data_schemas: &[SdkDataSchema],
  context: &CodegenContext<'_>,
  out_dir_path: &Path,
) -> Result<()> {
  let out_deserializers_dir_path = out_dir_path.join("deserializers");
  fs::create_dir_all(&out_deserializers_dir_path)?;
  clear_generated_rs_files(&out_deserializers_dir_path)?;

  let mut deserializers_mod_list: Vec<ItemMod> = vec![];

  for sdk_data_schema in sdk_data_schemas {
    let deserializer_path =
      out_deserializers_dir_path.join(format!("{}.rs", sdk_data_schema.module_name));
    write_generated_module(
      &deserializer_path,
      gen_schema_deserializer(sdk_data_schema, context).map_err(|err| {
        format!(
          "failed to generate deserializer {}: {err}",
          sdk_data_schema.module_name
        )
      })?,
    )?;

    push_module_decl(&mut deserializers_mod_list, &sdk_data_schema.module_name)?;
  }

  let token_stream: TokenStream = quote! {
    #( #deserializers_mod_list )*
  };
  let deserializers_mod_path = out_dir_path.join("deserializers.rs");
  write_generated_module(&deserializers_mod_path, token_stream)?;

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

  for namespace in sdk_data_namespaces {
    let uri = namespace.uri.as_str();
    let prefix = namespace.prefix.as_str();

    uri_to_prefix_arms.push(parse2(quote! {
      #uri => Some(#prefix),
    })?);
  }

  let token_stream: TokenStream = quote! {
    pub(crate) fn prefix_by_uri(uri: &str) -> Option<&'static str> {
      match uri {
        #( #uri_to_prefix_arms )*
        _ => None,
      }
    }
  };

  let namespaces_path = out_dir_path.join("namespaces.rs");
  write_generated_module(&namespaces_path, token_stream)?;

  Ok(())
}

fn write_serializers(
  sdk_data_schemas: &[SdkDataSchema],
  context: &CodegenContext<'_>,
  out_dir_path: &Path,
) -> Result<()> {
  let out_serializers_dir_path = out_dir_path.join("serializers");
  fs::create_dir_all(&out_serializers_dir_path)?;
  clear_generated_rs_files(&out_serializers_dir_path)?;

  let mut serializers_mod_list: Vec<ItemMod> = vec![];

  for sdk_data_schema in sdk_data_schemas {
    let serializer_path =
      out_serializers_dir_path.join(format!("{}.rs", sdk_data_schema.module_name));
    write_generated_module(
      &serializer_path,
      gen_schema_serializer(sdk_data_schema, context).map_err(|err| {
        format!(
          "failed to generate serializer {}: {err}",
          sdk_data_schema.module_name
        )
      })?,
    )?;

    push_module_decl(&mut serializers_mod_list, &sdk_data_schema.module_name)?;
  }

  let token_stream: TokenStream = quote! {
    #( #serializers_mod_list )*
  };
  let serializers_mod_path = out_dir_path.join("serializers.rs");
  write_generated_module(&serializers_mod_path, token_stream)?;

  Ok(())
}

fn write_generated_module(path: &Path, token_stream: TokenStream) -> Result<()> {
  let syntax_tree: syn::File = parse2(token_stream)
    .map_err(|err| format!("failed to parse generated module {}: {err}", path.display()))?;
  let formatted = prettyplease::unparse(&syntax_tree);
  fs::write(path, format!("{FILE_HEADER}\n{formatted}"))?;
  Ok(())
}

fn push_module_decl(mod_list: &mut Vec<ItemMod>, module_name: &str) -> Result<()> {
  let module_ident: Ident = parse_str(module_name)?;
  mod_list.push(parse2(quote! {
    pub mod #module_ident;
  })?);
  Ok(())
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
