use heck::{ToSnakeCase, ToUpperCamelCase};
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
use crate::sdk_code::part_codegen_ir::PartModuleDecl;
use crate::sdk_code::parts::{gen_part_module_with_relationship_type_variants, gen_parts_mod};
use crate::sdk_code::schemas::{
  TypeContainmentGraph, gen_mce_process_content_mapping, gen_schema_from_ir_with_type_graph,
};
use crate::sdk_code::versioning::version_cfg_attrs;
use crate::sdk_data::sdk_data_model::Namespace as SdkDataNamespace;
use crate::utils::{escape_snake_case, escape_upper_camel_case};

pub mod codegen_ir;
pub mod codegen_ir_builder;
pub mod helpers;
pub mod part_codegen_ir;
pub mod parts;
pub mod schemas;
pub mod simple_type_mapping;
pub mod versioning;

const FILE_HEADER: &str = r#"//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//
"#;

// These namespaces are part of the generated static namespace model even
// though they are not present in the Open XML SDK namespace input data.
const KNOWN_NAMESPACE_SUPPLEMENTS: &[(&str, &str, &str, &str)] = &[
  (
    "Xsi",
    "xsi",
    "http://www.w3.org/2001/XMLSchema-instance",
    "Office2007",
  ),
  (
    "W14Preview2007",
    "w14",
    "http://schemas.microsoft.com/office/word/2007/5/30/wordml",
    "Office2010",
  ),
  (
    "W14Preview2008",
    "w14",
    "http://schemas.microsoft.com/office/word/2008/9/12/wordml",
    "Office2010",
  ),
  (
    "W14Preview",
    "w14",
    "http://schemas.microsoft.com/office/word/2009/2/wordml",
    "Office2010",
  ),
  (
    "W15Preview",
    "w15",
    "http://schemas.microsoft.com/office/word/2010/11/wordml",
    "Office2013",
  ),
  (
    "Cx1",
    "cx1",
    "http://schemas.microsoft.com/office/drawing/2015/9/8/chartex",
    "Office2016",
  ),
  (
    "Cx2",
    "cx2",
    "http://schemas.microsoft.com/office/drawing/2015/10/21/chartex",
    "Office2016",
  ),
  (
    "Cx3",
    "cx3",
    "http://schemas.microsoft.com/office/drawing/2016/5/9/chartex",
    "Office2016",
  ),
  (
    "Cx4",
    "cx4",
    "http://schemas.microsoft.com/office/drawing/2016/5/10/chartex",
    "Office2016",
  ),
  (
    "Cx5",
    "cx5",
    "http://schemas.microsoft.com/office/drawing/2016/5/11/chartex",
    "Office2016",
  ),
  (
    "Cx6",
    "cx6",
    "http://schemas.microsoft.com/office/drawing/2016/5/12/chartex",
    "Office2016",
  ),
  (
    "Cx7",
    "cx7",
    "http://schemas.microsoft.com/office/drawing/2016/5/13/chartex",
    "Office2016",
  ),
  (
    "Cx8",
    "cx8",
    "http://schemas.microsoft.com/office/drawing/2016/5/14/chartex",
    "Office2016",
  ),
  (
    "Wpi",
    "wpi",
    "http://schemas.microsoft.com/office/word/2010/wordprocessingInk",
    "Office2010",
  ),
  (
    "W16",
    "w16",
    "http://schemas.microsoft.com/office/word/2018/wordml",
    "Office2021",
  ),
  (
    "Ve",
    "ve",
    "http://schemas.openxmlformats.org/markup-compatibility/2006",
    "Office2007",
  ),
  (
    "Hs",
    "hs",
    "http://schemas.haansoft.com/office/spreadsheet/8.0",
    "Office2007",
  ),
];

// Historical preview namespaces retain their source URI for package fidelity,
// while generated schema matching treats them as their finalized namespace.
const KNOWN_NAMESPACE_SCHEMA_EQUIVALENTS: &[(&str, &str)] = &[
  ("W14Preview2007", "W14"),
  ("W14Preview2008", "W14"),
  ("W14Preview", "W14"),
  ("W15Preview", "W15"),
  ("W16", "W16cur"),
  ("Ve", "Mc"),
];

// Keep the upstream entries aligned with OpenXmlNamespaceResolver. Aliases
// resolve to the canonical statically generated namespace and serialize through
// that namespace's canonical prefix and URI.
const KNOWN_NAMESPACE_ALIASES: &[(&str, &str)] = &[
  (
    "http://schemas.openxmlformats.org/wordprocessingml/2006/3/main",
    "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
  ),
  (
    "http://schemas.openxmlformats.org/wordprocessingml/2006/5/main",
    "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
  ),
  (
    "http://schemas.openxmlformats.org/wordprocessingml/2006/6/main",
    "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
  ),
  (
    "http://schemas.openxmlformats.org/spreadsheetml/2006/5/main",
    "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
  ),
  (
    "http://schemas.openxmlformats.org/spreadsheetml/2006/7/main",
    "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
  ),
  (
    "http://schemas.openxmlformats.org/presentationml/2006/3/main",
    "http://schemas.openxmlformats.org/presentationml/2006/main",
  ),
  (
    "http://schemas.openxmlformats.org/drawingml/2006/3/main",
    "http://schemas.openxmlformats.org/drawingml/2006/main",
  ),
  (
    "http://purl.oclc.org/ooxml/drawingml/chart",
    "http://schemas.openxmlformats.org/drawingml/2006/chart",
  ),
  (
    "http://purl.oclc.org/ooxml/drawingml/chartDrawing",
    "http://schemas.openxmlformats.org/drawingml/2006/chartDrawing",
  ),
  (
    "http://purl.oclc.org/ooxml/drawingml/diagram",
    "http://schemas.openxmlformats.org/drawingml/2006/diagram",
  ),
  (
    "http://purl.oclc.org/ooxml/drawingml/main",
    "http://schemas.openxmlformats.org/drawingml/2006/main",
  ),
  (
    "http://purl.oclc.org/ooxml/drawingml/picture",
    "http://schemas.openxmlformats.org/drawingml/2006/picture",
  ),
  (
    "http://purl.oclc.org/ooxml/drawingml/spreadsheetDrawing",
    "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing",
  ),
  (
    "http://purl.oclc.org/ooxml/drawingml/wordprocessingDrawing",
    "http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing",
  ),
  (
    "http://purl.oclc.org/ooxml/officeDocument/bibliography",
    "http://schemas.openxmlformats.org/officeDocument/2006/bibliography",
  ),
  (
    "http://purl.oclc.org/ooxml/officeDocument/customProperties",
    "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties",
  ),
  (
    "http://purl.oclc.org/ooxml/officeDocument/customXml",
    "http://schemas.openxmlformats.org/officeDocument/2006/customXml",
  ),
  (
    "http://purl.oclc.org/ooxml/officeDocument/docPropsVTypes",
    "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes",
  ),
  (
    "http://purl.oclc.org/ooxml/officeDocument/extendedProperties",
    "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties",
  ),
  (
    "http://purl.oclc.org/ooxml/officeDocument/math",
    "http://schemas.openxmlformats.org/officeDocument/2006/math",
  ),
  (
    "http://purl.oclc.org/ooxml/officeDocument/relationships",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
  ),
  (
    "http://purl.oclc.org/ooxml/presentationml/main",
    "http://schemas.openxmlformats.org/presentationml/2006/main",
  ),
  (
    "http://purl.oclc.org/ooxml/schemaLibrary/main",
    "http://schemas.openxmlformats.org/schemaLibrary/2006/main",
  ),
  (
    "http://purl.oclc.org/ooxml/spreadsheetml/main",
    "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
  ),
  (
    "http://purl.oclc.org/ooxml/wordprocessingml/main",
    "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
  ),
  (
    "http://purl.oclc.org/ooxml/drawingml/lockedCanvas",
    "http://schemas.openxmlformats.org/drawingml/2006/lockedCanvas",
  ),
  (
    "http://purl.oclc.org/ooxml/drawingml/compatibility",
    "http://schemas.openxmlformats.org/drawingml/2006/compatibility",
  ),
  (
    "http://purl.oclc.org/ooxml/officeDocument/relationships/customXml",
    "http://schemas.openxmlformats.org/officeDocument/2006/customXml",
  ),
];

struct LoadedSchema {
  ir: SchemaModuleDecl,
}

struct LoadedPart {
  ir: PartModuleDecl,
}

pub fn gen_sdk_code<P: AsRef<Path>>(sdk_data_dir: P, out_dir: P) -> Result<()> {
  let sdk_data_schemas_dir_path = sdk_data_dir.as_ref().join("schemas");
  let sdk_data_parts_dir_path = sdk_data_dir.as_ref().join("parts");
  let loaded_schemas = read_schemas(&sdk_data_schemas_dir_path)?;
  let loaded_parts = read_parts(&sdk_data_parts_dir_path)?;
  let namespaces = read_namespaces(sdk_data_dir.as_ref().join("namespaces.json"))?;
  let out_dir_path = out_dir.as_ref();

  write_schemas(&loaded_schemas, out_dir_path)?;
  write_parts(&loaded_parts, out_dir_path)?;
  write_namespaces(NamespacesInput {
    sdk_data_namespaces: &namespaces,
    out_dir_path,
    include_known_namespace: true,
    include_uri_by_prefix: false,
    include_minimum_version_by_uri: true,
    include_default_namespace_style: false,
  })?;

  Ok(())
}

pub fn gen_derive_namespace_code<P: AsRef<Path>>(sdk_data_dir: P, out_dir: P) -> Result<()> {
  let loaded_schemas = read_schemas(&sdk_data_dir.as_ref().join("schemas"))?;
  let namespaces = read_namespaces(sdk_data_dir.as_ref().join("namespaces.json"))?;
  let module_refs = loaded_schemas
    .iter()
    .map(|loaded| &loaded.ir)
    .collect::<Vec<_>>();
  let schema_graph = TypeContainmentGraph::from_modules(&module_refs);
  write_namespaces(NamespacesInput {
    sdk_data_namespaces: &namespaces,
    out_dir_path: out_dir.as_ref(),
    include_known_namespace: false,
    include_uri_by_prefix: true,
    include_minimum_version_by_uri: false,
    include_default_namespace_style: false,
  })?;
  simple_type_mapping::write_simple_type_mapping(&loaded_schemas, out_dir.as_ref())?;
  let mce_mapping = gen_mce_process_content_mapping(&module_refs, &schema_graph)?;
  write_generated_module(
    &out_dir.as_ref().join("mce_process_content_mapping.rs"),
    mce_mapping,
  )
}

fn read_schemas(sdk_data_schemas_dir_path: &Path) -> Result<Vec<LoadedSchema>> {
  let mut loaded_schemas = vec![];

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
    let ir: SchemaModuleDecl = serde_json::from_reader(reader)?;
    if !is_valid_codegen_ir_schema(&ir) {
      return Err(
        format!(
          "expected schema IR json in {}, found legacy/non-IR shape",
          path.display()
        )
        .into(),
      );
    }
    loaded_schemas.push(LoadedSchema { ir });
  }

  loaded_schemas.sort_by(|a, b| a.ir.module_name.cmp(&b.ir.module_name));
  Ok(loaded_schemas)
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

fn read_namespaces(path: impl AsRef<Path>) -> Result<Vec<SdkDataNamespace>> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let mut namespaces: Vec<SdkDataNamespace> = serde_json::from_reader(reader)?;
  sort_namespaces(&mut namespaces);
  Ok(namespaces)
}

fn sort_namespaces(namespaces: &mut [SdkDataNamespace]) {
  namespaces.sort_by(|left, right| {
    left
      .prefix
      .cmp(&right.prefix)
      .then(left.uri.cmp(&right.uri))
  });
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
        return Err(
          format!(
            "schema prefix alias {alias_name} for module {} collides with an existing schema module or alias",
            loaded_schema.ir.module_name
          )
          .into(),
        );
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
      gen_part_module_with_relationship_type_variants(&loaded_part.ir).map_err(|err| {
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
      #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub struct ExtendedPart {
          pub(crate) key: crate::common::PartKey,
        }
      impl crate::sdk::SdkPartDescriptor for ExtendedPart {
        const KIND: crate::parts::PartKind = crate::parts::PartKind::ExtendedPart;
        const RELATIONSHIP_TYPE: &'static str = "";
        const PATH_PREFIX: &'static str = "";
        const CONTENT_TYPE: &'static str = "";
        const TARGET_NAME: &'static str = "extendedPart";
        const EXTENSION: &'static str = "";
      }

      impl crate::private::SdkPartHandle for ExtendedPart {
        #[inline]
        fn from_part_key(part_key: crate::common::PartKey) -> Self {
          Self { key: part_key }
        }

        #[inline]
        fn part_key(&self) -> crate::common::PartKey {
          self.key
        }
      }

      impl crate::sdk::SdkPart for ExtendedPart {
        const CHILD_PART_CONSTRAINTS: &'static [crate::sdk::PartConstraint] = &[];
        const ALLOWS_ANY_CHILD_PART: bool = true;

        #[inline]
        fn child_part_constraint(
          _kind: crate::parts::PartKind,
        ) -> Option<crate::sdk::PartConstraint> {
          None
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

struct NamespacesInput<'a> {
  sdk_data_namespaces: &'a [SdkDataNamespace],
  out_dir_path: &'a Path,
  include_known_namespace: bool,
  include_uri_by_prefix: bool,
  include_minimum_version_by_uri: bool,
  include_default_namespace_style: bool,
}

struct KnownNamespaceSpec<'a> {
  variant_name: String,
  prefix: &'a str,
  uri: &'a str,
  version: &'a str,
}

fn write_namespaces(input: NamespacesInput<'_>) -> Result<()> {
  let mut namespaces = input
    .sdk_data_namespaces
    .iter()
    .filter(|namespace| !namespace.prefix.is_empty() && !namespace.uri.is_empty())
    .map(|namespace| KnownNamespaceSpec {
      variant_name: namespace_variant_name(&namespace.prefix),
      prefix: &namespace.prefix,
      uri: &namespace.uri,
      version: &namespace.version,
    })
    .collect::<Vec<_>>();
  namespaces.extend(KNOWN_NAMESPACE_SUPPLEMENTS.iter().map(
    |&(variant_name, prefix, uri, version)| KnownNamespaceSpec {
      variant_name: variant_name.to_owned(),
      prefix,
      uri,
      version,
    },
  ));

  let mut prefix_to_uri_arms: Vec<syn::Arm> = vec![];
  let mut prefix_to_variant_arms: Vec<syn::Arm> = vec![];
  let mut prefix_to_minimum_version_arms: Vec<syn::Arm> = vec![];
  let mut uri_to_minimum_version_arms: Vec<syn::Arm> = vec![];
  let mut known_namespace_variants: Vec<TokenStream> = vec![];
  let mut seen_uris = HashSet::new();
  let mut seen_prefixes = HashSet::new();
  let mut seen_variants = HashSet::new();

  for namespace in &namespaces {
    let prefix = namespace.prefix;
    let uri = namespace.uri;
    let attrs = version_cfg_attrs(namespace.version);

    let first_uri = seen_uris.insert(uri);
    let variant_ident: Ident = parse_str(&namespace.variant_name)?;
    if seen_variants.insert(namespace.variant_name.as_str()) {
      let aliases = first_uri
        .then_some(())
        .into_iter()
        .flat_map(|()| KNOWN_NAMESPACE_ALIASES.iter())
        .filter_map(|&(alias, canonical)| (canonical == uri).then_some(alias));
      known_namespace_variants.push(quote! {
        #( #attrs )*
        #[sdk(#prefix, #uri)]
        #( #[sdk_alias(#aliases)] )*
        #variant_ident,
      });
    }
    if first_uri {
      let version = if namespace.version.is_empty() {
        "Office2007"
      } else {
        namespace.version
      };
      let version_ident: Ident = parse_str(version)?;
      let uri_bytes = syn::LitByteStr::new(uri.as_bytes(), proc_macro2::Span::call_site());
      uri_to_minimum_version_arms.push(parse2(quote! {
        #uri_bytes => Some(crate::sdk::FileFormatVersion::#version_ident),
      })?);
    }

    if seen_prefixes.insert(prefix) {
      prefix_to_uri_arms.push(parse2(quote! {
        #( #attrs )*
        #prefix => Some(#uri),
      })?);
      let variant_name = namespace.variant_name.as_str();
      prefix_to_variant_arms.push(parse2(quote! {
        #( #attrs )*
        #prefix => Some(#variant_name),
      })?);
      if !namespace.version.is_empty() {
        let version = namespace.version;
        prefix_to_minimum_version_arms.push(parse2(quote! {
          #prefix => Some(#version),
        })?);
      }
    }
  }

  let mut seen_aliases = HashSet::new();
  for &(alias, canonical) in KNOWN_NAMESPACE_ALIASES {
    if !seen_aliases.insert(alias) {
      return Err(format!("duplicate known namespace alias {alias}").into());
    }
    if seen_uris.contains(alias) {
      return Err(format!("known namespace alias {alias} is also an exact namespace URI").into());
    }
    let Some(namespace) = namespaces
      .iter()
      .find(|namespace| namespace.uri == canonical)
    else {
      return Err(format!("known namespace alias {alias} targets unknown URI {canonical}").into());
    };
    let version = if namespace.version.is_empty() {
      "Office2007"
    } else {
      namespace.version
    };
    let version_ident: Ident = parse_str(version)?;
    let alias_bytes = syn::LitByteStr::new(alias.as_bytes(), proc_macro2::Span::call_site());
    uri_to_minimum_version_arms.push(parse2(quote! {
      #alias_bytes => Some(crate::sdk::FileFormatVersion::#version_ident),
    })?);
  }

  let uri_by_prefix_tokens = if input.include_uri_by_prefix {
    quote! {
      pub(crate) fn uri_by_prefix(prefix: &str) -> Option<&'static str> {
        match prefix {
          #( #prefix_to_uri_arms )*
          _ => None,
        }
      }

      pub(crate) fn variant_by_prefix(prefix: &str) -> Option<&'static str> {
        match prefix {
          #( #prefix_to_variant_arms )*
          _ => None,
        }
      }

      pub(crate) fn minimum_version_by_prefix(prefix: &str) -> Option<&'static str> {
        match prefix {
          #( #prefix_to_minimum_version_arms )*
          _ => None,
        }
      }
    }
  } else {
    quote! {}
  };
  let minimum_version_by_uri_tokens = if input.include_minimum_version_by_uri {
    quote! {
      #[cfg(feature = "mce")]
      pub(crate) fn minimum_version_by_uri(
        uri: &[u8],
      ) -> Option<crate::sdk::FileFormatVersion> {
        match uri {
          #( #uri_to_minimum_version_arms )*
          _ => None,
        }
      }
    }
  } else {
    quote! {}
  };
  let default_namespace_style_tokens = if input.include_default_namespace_style {
    quote! {
      #[inline]
      pub(crate) fn uses_default_namespace(prefix: &str) -> bool {
        matches!(prefix, "x" | "pct")
      }
    }
  } else {
    quote! {}
  };
  let known_namespace_tokens = if input.include_known_namespace {
    let schema_namespace_arms = KNOWN_NAMESPACE_SCHEMA_EQUIVALENTS
      .iter()
      .map(|&(source, target)| {
        let source_ident: Ident = parse_str(source)?;
        let target_ident: Ident = parse_str(target)?;
        Ok(quote! { Self::#source_ident => Self::#target_ident, })
      })
      .collect::<Result<Vec<_>>>()?;
    quote! {
      #[repr(u16)]
      #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ooxmlsdk_derive::SdkXmlNamespace)]
      pub enum XmlKnownNamespace {
        #( #known_namespace_variants )*
      }

      impl Default for XmlKnownNamespace {
        #[inline]
        fn default() -> Self {
          Self::A
        }
      }

      impl XmlKnownNamespace {
        #[inline]
        pub(crate) const fn schema_namespace(self) -> Self {
          match self {
            #( #schema_namespace_arms )*
            namespace => namespace,
          }
        }
      }

    }
  } else {
    quote! {}
  };
  let token_stream: TokenStream = quote! {
    #known_namespace_tokens

    #uri_by_prefix_tokens
    #minimum_version_by_uri_tokens
    #default_namespace_style_tokens
  };

  write_generated_module(&input.out_dir_path.join("namespaces.rs"), token_stream)?;
  Ok(())
}

fn namespace_variant_name(prefix: &str) -> String {
  let mut name = prefix.to_upper_camel_case();
  if name.is_empty() {
    name.push_str("Default");
  }
  escape_upper_camel_case(name)
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

fn is_valid_codegen_ir_schema(schema: &SchemaModuleDecl) -> bool {
  !schema.module_name.is_empty()
    && schema.types.iter().all(|ty| !ty.rust_name.is_empty())
    && schema.enums.iter().all(|en| !en.rust_name.is_empty())
}

fn is_codegen_ir_part_json(value: &Value) -> bool {
  value.get("StructName").is_some() && value.get("Fields").is_some()
}
