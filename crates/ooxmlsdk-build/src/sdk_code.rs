use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use syn::{Attribute, Ident, ItemMod, LitByteStr, parse_str, parse2};

use crate::Result;
use crate::sdk_code::codegen_ir::SchemaModuleDecl;
use crate::sdk_code::part_codegen_ir::PartModuleDecl;
use crate::sdk_code::parts::{gen_part_module, gen_parts_mod, relationship_type_aliases};
use crate::sdk_code::schemas::{TypeContainmentGraph, gen_schema_from_ir_with_type_graph};
use crate::sdk_code::versioning::version_cfg_attrs;
use crate::sdk_data::sdk_data_model::{
  Namespace as SdkDataNamespace, NamespaceAlias as SdkDataNamespaceAlias,
  NamespaceExtensions as SdkDataNamespaceExtensions,
};
use crate::utils::{escape_snake_case, escape_upper_camel_case};

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

struct LoadedPart {
  ir: PartModuleDecl,
}

struct GeneratedRelationshipUri {
  variant_name: String,
  uri: String,
  version: String,
}

pub fn gen_sdk_code<P: AsRef<Path>>(sdk_data_dir: P, out_dir: P) -> Result<()> {
  let sdk_data_schemas_dir_path = sdk_data_dir.as_ref().join("schemas");
  let sdk_data_parts_dir_path = sdk_data_dir.as_ref().join("parts");
  let loaded_schemas = read_schemas(&sdk_data_schemas_dir_path)?;
  let loaded_parts = read_parts(&sdk_data_parts_dir_path)?;
  let mut namespaces = read_namespaces(sdk_data_dir.as_ref().join("namespaces.json"))?;
  let namespace_extensions =
    read_namespace_extensions(sdk_data_dir.as_ref().join("namespace_extensions.json"))?;
  namespaces.extend(namespace_extensions.namespaces);
  sort_namespaces(&mut namespaces);
  let relationship_type_uris = relationship_type_known_uris(&loaded_parts);
  let relationship_type_aliases =
    relationship_type_generated_aliases(&loaded_parts, &namespace_extensions.aliases);
  let out_dir_path = out_dir.as_ref();

  write_schemas(&loaded_schemas, out_dir_path)?;
  write_parts(&loaded_parts, &namespace_extensions.aliases, out_dir_path)?;
  write_namespaces(NamespacesInput {
    sdk_data_namespaces: &namespaces,
    namespace_aliases: &namespace_extensions.aliases,
    generated_relationship_uris: &relationship_type_uris,
    generated_relationship_aliases: &relationship_type_aliases,
    out_dir_path,
    include_known_namespace: true,
    include_uri_by_prefix: false,
    include_default_namespace_style: false,
  })?;

  Ok(())
}

pub fn gen_derive_namespace_code<P: AsRef<Path>>(sdk_data_dir: P, out_dir: P) -> Result<()> {
  let mut namespaces = read_namespaces(sdk_data_dir.as_ref().join("namespaces.json"))?;
  let namespace_extensions =
    read_namespace_extensions(sdk_data_dir.as_ref().join("namespace_extensions.json"))?;
  namespaces.extend(namespace_extensions.namespaces);
  sort_namespaces(&mut namespaces);
  write_namespaces(NamespacesInput {
    sdk_data_namespaces: &namespaces,
    namespace_aliases: &[],
    generated_relationship_uris: &[],
    generated_relationship_aliases: &[],
    out_dir_path: out_dir.as_ref(),
    include_known_namespace: false,
    include_uri_by_prefix: true,
    include_default_namespace_style: true,
  })
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

fn read_namespace_extensions(path: impl AsRef<Path>) -> Result<SdkDataNamespaceExtensions> {
  let path = path.as_ref();
  if !path.exists() {
    return Ok(SdkDataNamespaceExtensions::default());
  }

  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let mut extensions: SdkDataNamespaceExtensions = serde_json::from_reader(reader)?;
  sort_namespaces(&mut extensions.namespaces);
  extensions.aliases.sort_by(|left, right| {
    left
      .uri
      .cmp(&right.uri)
      .then(left.canonical_uri.cmp(&right.canonical_uri))
  });
  Ok(extensions)
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

fn write_parts(
  loaded_parts: &[LoadedPart],
  namespace_aliases: &[SdkDataNamespaceAlias],
  out_dir_path: &Path,
) -> Result<()> {
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
      namespace_aliases,
    )?,
  )?;

  Ok(())
}

struct NamespacesInput<'a> {
  sdk_data_namespaces: &'a [SdkDataNamespace],
  namespace_aliases: &'a [SdkDataNamespaceAlias],
  generated_relationship_uris: &'a [GeneratedRelationshipUri],
  generated_relationship_aliases: &'a [(String, String)],
  out_dir_path: &'a Path,
  include_known_namespace: bool,
  include_uri_by_prefix: bool,
  include_default_namespace_style: bool,
}

fn write_namespaces(input: NamespacesInput<'_>) -> Result<()> {
  let mut prefix_to_uri_arms: Vec<syn::Arm> = vec![];
  let mut known_namespace_variants: Vec<TokenStream> = vec![];
  let mut known_prefix_bytes_arms: Vec<syn::Arm> = vec![];
  let mut known_uri_bytes_arms: Vec<syn::Arm> = vec![];
  let mut known_from_uri_bytes_arms: Vec<syn::Arm> = vec![];
  let mut seen_uris = HashSet::new();
  let mut seen_prefixes = HashSet::new();
  let mut seen_variants = HashSet::new();
  let mut seen_from_uri_inputs = HashSet::new();
  let mut namespace_by_uri: HashMap<String, (Ident, Vec<Attribute>)> = HashMap::new();

  for namespace in input.sdk_data_namespaces {
    if namespace.prefix.is_empty() || namespace.uri.is_empty() {
      continue;
    }
    let prefix = namespace.prefix.as_str();
    let uri = namespace.uri.as_str();
    let attrs = version_cfg_attrs(&namespace.version);

    if seen_uris.insert(uri) {
      let variant_name = namespace_variant_name(prefix);
      let variant_ident: Ident = parse_str(&variant_name)?;
      namespace_by_uri.insert(uri.to_string(), (variant_ident.clone(), attrs.clone()));
      if seen_variants.insert(variant_name) {
        known_namespace_variants.push(quote! {
          #( #attrs )*
          #variant_ident,
        });
        known_prefix_bytes_arms.push(parse2(quote! {
          #( #attrs )*
          Self::#variant_ident => #prefix.as_bytes(),
        })?);
        known_uri_bytes_arms.push(parse2(quote! {
          #( #attrs )*
          Self::#variant_ident => #uri.as_bytes(),
        })?);
      }
      let uri_bytes = LitByteStr::new(uri.as_bytes(), Span::call_site());
      if seen_from_uri_inputs.insert(uri.to_string()) {
        known_from_uri_bytes_arms.push(parse2(quote! {
          #( #attrs )*
          #uri_bytes => Some(Self::#variant_ident),
        })?);
      }
    }

    if seen_prefixes.insert(prefix) {
      prefix_to_uri_arms.push(parse2(quote! {
        #( #attrs )*
        #prefix => Some(#uri),
      })?);
    }
  }

  for alias in input.namespace_aliases {
    if alias.uri.is_empty() || alias.canonical_uri.is_empty() {
      return Err("namespace aliases must have non-empty Uri and CanonicalUri".into());
    }
    if alias.uri == alias.canonical_uri {
      return Err(
        format!(
          "namespace alias {} points to itself as CanonicalUri",
          alias.uri
        )
        .into(),
      );
    }
    let Some((variant_ident, attrs)) = namespace_by_uri.get(alias.canonical_uri.as_str()) else {
      return Err(
        format!(
          "namespace alias {} points to unknown CanonicalUri {}",
          alias.uri, alias.canonical_uri
        )
        .into(),
      );
    };
    let uri = alias.uri.as_str();
    let uri_bytes = LitByteStr::new(uri.as_bytes(), Span::call_site());
    if seen_from_uri_inputs.insert(uri.to_string()) {
      known_from_uri_bytes_arms.push(parse2(quote! {
        #( #attrs )*
        #uri_bytes => Some(Self::#variant_ident),
      })?);
    }
  }

  let uri_by_prefix_tokens = if input.include_uri_by_prefix {
    quote! {
      pub(crate) fn uri_by_prefix(prefix: &str) -> Option<&'static str> {
        match prefix {
          #( #prefix_to_uri_arms )*
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
        matches!(prefix, "x")
      }
    }
  } else {
    quote! {}
  };
  let known_namespace_tokens = if input.include_known_namespace {
    quote! {
      #[repr(u16)]
      #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
        pub const fn prefix_bytes(self) -> &'static [u8] {
          match self {
            #( #known_prefix_bytes_arms )*
          }
        }

        pub const fn prefix(self) -> &'static str {
          // Generated namespace prefixes are ASCII and therefore valid UTF-8.
          unsafe { std::str::from_utf8_unchecked(self.prefix_bytes()) }
        }

        pub const fn uri_bytes(self) -> &'static [u8] {
          match self {
            #( #known_uri_bytes_arms )*
          }
        }

        pub const fn uri(self) -> &'static str {
          // Generated namespace URIs are ASCII and therefore valid UTF-8.
          unsafe { std::str::from_utf8_unchecked(self.uri_bytes()) }
        }

        pub fn from_uri(uri: &str) -> Option<Self> {
          Self::from_uri_bytes(uri.as_bytes())
        }

        pub fn from_uri_bytes(uri: &[u8]) -> Option<Self> {
          match uri {
            #( #known_from_uri_bytes_arms )*
            _ => None,
          }
        }
      }

    }
  } else {
    quote! {}
  };
  let known_relationship_namespace_tokens = if input.include_known_namespace {
    relationship_namespace_tokens(
      input.generated_relationship_uris,
      input.generated_relationship_aliases,
    )?
  } else {
    quote! {}
  };
  let token_stream: TokenStream = quote! {
    #known_namespace_tokens
    #known_relationship_namespace_tokens

    #uri_by_prefix_tokens
    #default_namespace_style_tokens
  };

  write_generated_module(&input.out_dir_path.join("namespaces.rs"), token_stream)?;
  Ok(())
}

fn relationship_namespace_tokens(
  generated_uris: &[GeneratedRelationshipUri],
  generated_aliases: &[(String, String)],
) -> Result<TokenStream> {
  if generated_uris.is_empty() {
    return Ok(quote! {});
  }

  let mut variants = Vec::new();
  let mut uri_bytes_arms: Vec<syn::Arm> = Vec::new();
  let mut from_uri_bytes_arms: Vec<syn::Arm> = Vec::new();
  let mut namespace_by_uri = HashMap::new();
  let mut seen_from_uri_inputs = HashSet::new();

  for generated_uri in generated_uris {
    if generated_uri.uri.is_empty() {
      continue;
    }
    let attrs = version_cfg_attrs(&generated_uri.version);
    let variant_ident: Ident = parse_str(&generated_uri.variant_name)?;
    namespace_by_uri.insert(
      generated_uri.uri.as_str(),
      (variant_ident.clone(), attrs.clone()),
    );
    variants.push(quote! {
      #( #attrs )*
      #variant_ident,
    });
    let uri_bytes = LitByteStr::new(generated_uri.uri.as_bytes(), Span::call_site());
    uri_bytes_arms.push(parse2(quote! {
      #( #attrs )*
      Self::#variant_ident => #uri_bytes,
    })?);
    if seen_from_uri_inputs.insert(generated_uri.uri.as_str()) {
      from_uri_bytes_arms.push(parse2(quote! {
        #( #attrs )*
        #uri_bytes => Some(Self::#variant_ident),
      })?);
    }
  }

  for (uri, canonical_uri) in generated_aliases {
    if uri.is_empty() || canonical_uri.is_empty() {
      return Err(
        "generated relationship aliases must have non-empty URI and canonical URI".into(),
      );
    }
    let Some((variant_ident, attrs)) = namespace_by_uri.get(canonical_uri.as_str()) else {
      return Err(
        format!(
          "generated relationship alias {uri} points to unknown canonical URI {canonical_uri}"
        )
        .into(),
      );
    };
    if seen_from_uri_inputs.insert(uri.as_str()) {
      let uri_bytes = LitByteStr::new(uri.as_bytes(), Span::call_site());
      from_uri_bytes_arms.push(parse2(quote! {
        #( #attrs )*
        #uri_bytes => Some(Self::#variant_ident),
      })?);
    }
  }

  Ok(quote! {
    #[repr(u16)]
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum XmlKnownRelationshipNamespace {
      #( #variants )*
    }

    impl XmlKnownRelationshipNamespace {
      pub const fn uri_bytes(self) -> &'static [u8] {
        match self {
          #( #uri_bytes_arms )*
        }
      }

      pub const fn uri(self) -> &'static str {
        // Generated relationship URIs are ASCII and therefore valid UTF-8.
        unsafe { std::str::from_utf8_unchecked(self.uri_bytes()) }
      }

      pub fn from_uri(uri: &str) -> Option<Self> {
        Self::from_uri_bytes(uri.as_bytes())
      }

      pub fn from_uri_bytes(uri: &[u8]) -> Option<Self> {
        match uri {
          #( #from_uri_bytes_arms )*
          _ => None,
        }
      }
    }
  })
}

fn relationship_type_known_uris(loaded_parts: &[LoadedPart]) -> Vec<GeneratedRelationshipUri> {
  let mut known_uris = HashSet::new();
  let mut used_variants = HashSet::new();
  let mut generated = Vec::new();

  for relationship_type in EXTRA_RELATIONSHIP_TYPES {
    known_uris.insert((*relationship_type).to_string());
    generated.push(GeneratedRelationshipUri {
      variant_name: relationship_type_variant_name(relationship_type, &mut used_variants),
      uri: (*relationship_type).to_string(),
      version: "Office2007".to_string(),
    });
  }

  for part in loaded_parts {
    let relationship_type = part.ir.relationship_type.as_str();
    if relationship_type.is_empty() || !known_uris.insert(relationship_type.to_string()) {
      continue;
    }

    generated.push(GeneratedRelationshipUri {
      variant_name: relationship_type_variant_name(relationship_type, &mut used_variants),
      uri: relationship_type.to_string(),
      version: part.ir.version.clone(),
    });
  }
  generated
}

const EXTRA_RELATIONSHIP_TYPES: &[&str] = &[
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink",
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio",
  "http://schemas.microsoft.com/office/2007/relationships/media",
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/video",
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/attachedTemplate",
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLinkPath",
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/mailMergeSource",
  "http://schemas.microsoft.com/office/2006/relationships/txbx",
  "http://schemas.microsoft.com/office/2007/relationships/hdphoto",
  "http://schemas.microsoft.com/office/2015/10/relationships/revisionInfo",
  "http://schemas.microsoft.com/office/2016/11/relationships/changesInfo",
];

fn relationship_type_generated_aliases(
  loaded_parts: &[LoadedPart],
  namespace_aliases: &[SdkDataNamespaceAlias],
) -> Vec<(String, String)> {
  let mut aliases = HashSet::new();
  for relationship_type in EXTRA_RELATIONSHIP_TYPES {
    for alias_uri in relationship_type_aliases(relationship_type, namespace_aliases) {
      aliases.insert((alias_uri, (*relationship_type).to_string()));
    }
  }

  for part in loaded_parts {
    let relationship_type = part.ir.relationship_type.as_str();
    if relationship_type.is_empty() {
      continue;
    }

    for alias_uri in relationship_type_aliases(relationship_type, namespace_aliases) {
      aliases.insert((alias_uri, relationship_type.to_string()));
    }
  }
  let mut aliases: Vec<_> = aliases.into_iter().collect();
  aliases.sort();
  aliases
}

fn namespace_variant_name(prefix: &str) -> String {
  let mut name = prefix.to_upper_camel_case();
  if name.is_empty() {
    name.push_str("Default");
  }
  escape_upper_camel_case(name)
}

fn relationship_type_variant_name(uri: &str, used_variants: &mut HashSet<String>) -> String {
  let suffix = uri.rsplit('/').next().unwrap_or(uri);
  let mut base = String::from("Relationship");
  base.push_str(&suffix.to_upper_camel_case());
  let base = escape_upper_camel_case(base);
  unique_variant_name(base, used_variants)
}

fn unique_variant_name(base: String, used_variants: &mut HashSet<String>) -> String {
  if used_variants.insert(base.clone()) {
    return base;
  }
  let mut index = 2usize;
  loop {
    let candidate = format!("{base}{index}");
    if used_variants.insert(candidate.clone()) {
      return candidate;
    }
    index += 1;
  }
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
