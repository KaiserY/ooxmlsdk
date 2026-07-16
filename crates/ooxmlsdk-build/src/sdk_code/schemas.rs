use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Group, Ident as TokenIdent, Span, TokenStream, TokenTree};
use quote::quote;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet, VecDeque};
use syn::{Attribute, Ident, LitStr, Type, Variant, parse_str, parse2};

use crate::Result;
use crate::sdk_code::codegen_ir::{
  Cardinality, ContentModelDecl, ElementKind, EnumDecl, FieldDecl, FieldWireDecl, MemberDecl,
  SchemaModuleDecl, SystemSupportDecl, TypeDecl, TypeKind, TypeRefDecl, ValidatorKind, VariantDecl,
  VariantWireDecl,
};
use crate::sdk_code::codegen_ir_builder::build_codegen_ir;
use crate::sdk_code::helpers::{
  AttrTypeKind, StructuredChoice, StructuredChoiceVariant, StructuredParticleKind,
  classify_attr_type,
};
use crate::sdk_code::versioning::{common_choice_version, version_cfg_attrs};
use crate::sdk_data::sdk_data_model::{
  Namespace, Schema, SchemaEnum, SchemaType, SchemaTypeAttribute, SchemaTypeChild,
  SchemaTypeChildKind,
};
use crate::utils::{escape_snake_case, escape_upper_camel_case};

#[derive(Debug)]
pub struct CodegenContext<'a> {
  enum_type_map: HashMap<&'a str, &'a SchemaEnum>,
  enum_type_module_map: HashMap<&'a str, &'a str>,
  type_map: HashMap<&'a str, &'a SchemaType>,
  type_class_name_map: HashMap<&'a str, &'a SchemaType>,
  type_module_map: HashMap<&'a str, &'a str>,
  type_prefix_map: HashMap<&'a str, &'a str>,
  namespace_version_map: HashMap<&'a str, &'a str>,
  enum_module_by_typed_namespace_and_name: HashMap<(&'a str, &'a str), &'a str>,
}

fn schema_version_rank(version: &str) -> u8 {
  match version {
    "" | "Office2007" => 0,
    "Office2010" => 1,
    "Office2013" => 2,
    "Office2016" => 3,
    "Office2019" => 4,
    "Office2021" => 5,
    "Microsoft365" => 6,
    _ => 0,
  }
}

#[derive(Copy, Clone, Debug, Default)]
struct VersionCfgContext {
  suppress: bool,
}

impl VersionCfgContext {
  fn new(suppress: bool) -> Self {
    Self { suppress }
  }

  fn attrs(self, version: &str) -> Vec<Attribute> {
    if self.suppress {
      Vec::new()
    } else {
      version_cfg_attrs(version)
    }
  }

  fn child(self, version: &str) -> Self {
    let _ = version;
    Self {
      suppress: self.suppress,
    }
  }
}

fn sdk_version_markers(version: &str) -> Vec<TokenStream> {
  let _ = version;
  Vec::new()
}

fn mce_element_version_markers(type_decl: &TypeDecl) -> Result<Vec<TokenStream>> {
  let Some(version) = type_decl.support.element_version_override.as_deref() else {
    return Ok(Vec::new());
  };
  if type_decl.version.as_deref() != Some(version) {
    return Err(
      format!(
        "element version override for {} must match its source type version",
        type_decl.rust_name
      )
      .into(),
    );
  }
  let marker = Ident::new(&version.to_snake_case(), Span::call_site());
  Ok(vec![quote! { version = #marker }])
}

#[derive(Debug)]
pub struct ResolvedOneSequenceChild<'a> {
  pub name: &'a str,
  pub field_name: Cow<'a, str>,
  pub variant_name: Cow<'a, str>,
  pub property_comments: Cow<'a, str>,
  pub version: &'a str,
  pub kind: SchemaTypeChildKind,
  pub optional: bool,
  pub repeated: bool,
}

#[derive(Debug)]
pub struct ResolvedOneSequenceChoice<'a> {
  pub field_name: String,
  pub enum_name: String,
  pub property_comments: String,
  pub variants: Vec<ResolvedOneSequenceChild<'a>>,
}

#[derive(Debug)]
pub struct ResolvedOneSequenceSequenceField<'a> {
  pub child: ResolvedOneSequenceChild<'a>,
  pub optional: bool,
  pub repeated: bool,
  pub initial_version: &'a str,
}

#[derive(Debug)]
pub struct ResolvedOneSequenceSequenceVariant<'a> {
  pub variant_name: String,
  pub struct_name: String,
  pub property_comments: String,
  pub fields: Vec<ResolvedOneSequenceSequenceField<'a>>,
}

#[derive(Debug)]
pub enum ResolvedOneSequenceChoiceVariant<'a> {
  Leaf(ResolvedOneSequenceChild<'a>),
  Sequence(ResolvedOneSequenceSequenceVariant<'a>),
}

#[derive(Debug)]
pub struct ResolvedOneSequenceStructuredChoice<'a> {
  pub field_name: String,
  pub enum_name: String,
  pub property_comments: String,
  pub variants: Vec<ResolvedOneSequenceChoiceVariant<'a>>,
}

#[derive(Debug)]
pub struct ResolvedCompositeChild<'a> {
  pub name: &'a str,
  pub variant_name: Cow<'a, str>,
  pub version: &'a str,
  pub is_any: bool,
  pub kind: SchemaTypeChildKind,
  pub optional: bool,
  pub repeated: bool,
  pub children: Vec<ResolvedCompositeChild<'a>>,
}

#[derive(Debug)]
pub struct ResolvedOneChoice<'a> {
  pub field_name: String,
  pub enum_name: String,
  pub variants: Vec<ResolvedOneSequenceChild<'a>>,
}

#[derive(Debug, Default)]
pub(crate) struct TypeContainmentGraph {
  edges: HashMap<String, Vec<String>>,
  estimated_sizes: HashMap<String, usize>,
  enum_keys: HashSet<String>,
  empty_leaf_marker_ambiguous_rust_names: HashSet<String>,
  empty_leaf_marker_docs: HashMap<String, String>,
  empty_leaf_marker_docs_by_rust_name: HashMap<String, String>,
  any_children_alias_keys: HashSet<String>,
  leaf_text_alias_content: HashMap<String, TypeRefDecl>,
  leaf_text_alias_keys: HashSet<String>,
  module_alias_paths: HashMap<String, String>,
  no_prefix_only_keys: HashSet<String>,
  nodes: HashSet<String>,
  shallow_depths: HashMap<String, usize>,
  wrapped_base_keys: HashSet<String>,
}

impl TypeContainmentGraph {
  pub(crate) fn from_modules(modules: &[&SchemaModuleDecl]) -> Self {
    let mut graph = Self::default();
    let mut no_prefix_keys = HashSet::new();
    let mut type_decl_by_key = HashMap::<String, (&SchemaModuleDecl, &TypeDecl)>::new();

    for module in modules {
      if let Some(alias_path) = schema_prefix_alias_path(module) {
        graph
          .module_alias_paths
          .insert(module_type_namespace(&module.module_name), alias_path);
      }
      for schema_enum in &module.enums {
        graph
          .enum_keys
          .insert(local_type_key(module, &schema_enum.rust_name));
      }
      for type_decl in &module.types {
        let type_key = local_type_key(module, &type_decl.rust_name);
        graph.nodes.insert(type_key.clone());
        type_decl_by_key.insert(type_key.clone(), (*module, type_decl));
        if type_decl
          .xml_qname
          .as_deref()
          .is_some_and(|qname| type_uses_default_namespace(module.prefix.as_str(), qname))
        {
          no_prefix_keys.insert(type_key.clone());
        }
        if type_decl.estimated_size > 0 {
          graph
            .estimated_sizes
            .insert(type_key.clone(), type_decl.estimated_size);
        }
        if type_decl.kind == TypeKind::LeafTextAlias {
          graph.leaf_text_alias_keys.insert(type_key.clone());
          if let Some(xml_content) = &type_decl.xml_content {
            graph
              .leaf_text_alias_content
              .insert(type_key.clone(), xml_content.clone());
          }
        }
        if is_empty_leaf_marker_type(type_decl) {
          graph
            .empty_leaf_marker_docs
            .insert(type_key.clone(), type_decl.docs.clone());
          let generated_rust_name = type_decl.rust_name.to_upper_camel_case();
          if graph
            .empty_leaf_marker_docs_by_rust_name
            .insert(generated_rust_name.clone(), type_decl.docs.clone())
            .is_some()
          {
            graph
              .empty_leaf_marker_ambiguous_rust_names
              .insert(generated_rust_name);
          }
        }
        if can_alias_any_children_wrapper_decl(type_decl, &[]) {
          graph.any_children_alias_keys.insert(type_key);
        }
      }
    }

    let mut needs_prefixed_write_keys = HashSet::new();
    for module in modules {
      for type_decl in &module.types {
        let owner_key = local_type_key(module, &type_decl.rust_name);
        if !no_prefix_keys.contains(&owner_key) {
          continue;
        }
        let Some(parent_prefix) = type_decl.xml_qname.as_deref().and_then(type_qname_prefix) else {
          continue;
        };
        collect_prefixed_write_requirements_from_members(
          module,
          type_decl.members.as_slice(),
          parent_prefix,
          &no_prefix_keys,
          &type_decl_by_key,
          &mut needs_prefixed_write_keys,
        );
      }
    }
    let mut changed = true;
    while changed {
      changed = false;
      for owner_key in needs_prefixed_write_keys
        .iter()
        .cloned()
        .collect::<Vec<_>>()
      {
        let Some((module, type_decl)) = type_decl_by_key.get(&owner_key).copied() else {
          continue;
        };
        let len_before = needs_prefixed_write_keys.len();
        collect_prefixed_write_descendants_from_members(
          module,
          type_decl.members.as_slice(),
          &no_prefix_keys,
          &type_decl_by_key,
          &mut needs_prefixed_write_keys,
        );
        changed |= needs_prefixed_write_keys.len() != len_before;
      }
    }
    graph.no_prefix_only_keys = no_prefix_keys
      .difference(&needs_prefixed_write_keys)
      .cloned()
      .collect();

    for module in modules {
      for type_decl in &module.types {
        let owner_key = local_type_key(module, &type_decl.rust_name);
        let mut owner_edges = Vec::new();
        if let Some(base_rust_name) = &type_decl.base_rust_name {
          if let Some(base_module_path) = &type_decl.base_module_path {
            graph.wrapped_base_keys.insert(format!(
              "{base_module_path}::{}",
              base_rust_name.to_upper_camel_case()
            ));
          } else if let Some(base_type_decl) = module
            .types
            .iter()
            .find(|base_type_decl| base_type_decl.rust_name == *base_rust_name)
            && can_wrap_derived_to_base_decl(type_decl, base_type_decl)
          {
            graph
              .wrapped_base_keys
              .insert(local_type_key(module, base_rust_name));
          }
        }

        for member in &type_decl.members {
          match member {
            MemberDecl::Field(field) => {
              let referenced_target = match &field.wire {
                FieldWireDecl::Child { .. } | FieldWireDecl::Any | FieldWireDecl::Choice => {
                  schema_type_key_from_ref(module, &field.type_ref)
                }
                FieldWireDecl::TextChild { .. } => {
                  if is_value_like_type_ref(module, &field.type_ref, &graph) {
                    None
                  } else {
                    schema_type_key_from_ref(module, &field.type_ref)
                  }
                }
                FieldWireDecl::Attribute { .. } | FieldWireDecl::Text => None,
              };
              if matches!(field.cardinality, Cardinality::Many) {
                continue;
              }

              if let Some(target) = referenced_target {
                owner_edges.push(target);
              }
            }
            MemberDecl::Variant(variant) => {
              let target = match &variant.wire {
                crate::sdk_code::codegen_ir::VariantWireDecl::Any { .. }
                | crate::sdk_code::codegen_ir::VariantWireDecl::Text => {
                  schema_type_key_from_ref(module, &variant.payload)
                }
                crate::sdk_code::codegen_ir::VariantWireDecl::TextChild { .. } => {
                  if is_value_like_type_ref(module, &variant.payload, &graph) {
                    None
                  } else {
                    schema_type_key_from_ref(module, &variant.payload)
                  }
                }
                crate::sdk_code::codegen_ir::VariantWireDecl::Child { .. }
                | crate::sdk_code::codegen_ir::VariantWireDecl::Sequence { .. } => None,
              };

              if let Some(target) = target {
                owner_edges.push(target);
              }
            }
          }
        }

        graph.edges.insert(owner_key, owner_edges);
      }
    }

    let missing_leaf_size_nodes = graph
      .nodes
      .iter()
      .filter(|node| {
        !graph.estimated_sizes.contains_key(*node) && !graph.has_outgoing_edges(node.as_str())
      })
      .cloned()
      .collect::<Vec<_>>();
    for node in missing_leaf_size_nodes {
      graph.estimated_sizes.insert(node, 32);
    }
    graph.shallow_depths = graph.compute_shallow_depths();
    graph
  }

  fn contains_node(&self, node: &str) -> bool {
    self.nodes.contains(node)
  }

  fn has_outgoing_edges(&self, node: &str) -> bool {
    self
      .edges
      .get(node)
      .is_some_and(|children| !children.is_empty())
  }

  fn estimated_size(&self, node: &str) -> Option<usize> {
    self.estimated_sizes.get(node).copied()
  }

  fn shallow_depth(&self, node: &str) -> Option<usize> {
    self.shallow_depths.get(node).copied()
  }

  fn compute_shallow_depths(&self) -> HashMap<String, usize> {
    let mut indegrees = HashMap::<&str, usize>::new();
    for node in &self.nodes {
      indegrees.entry(node.as_str()).or_insert(0);
    }
    for children in self.edges.values() {
      for child in children {
        *indegrees.entry(child.as_str()).or_insert(0) += 1;
      }
    }

    let mut queue = VecDeque::<(&str, usize)>::new();
    for (node, indegree) in indegrees {
      if indegree == 0 {
        queue.push_back((node, 0));
      }
    }

    let mut depths = HashMap::<String, usize>::new();
    while let Some((node, depth)) = queue.pop_front() {
      if depths.get(node).is_some_and(|existing| *existing <= depth) {
        continue;
      }
      depths.insert(node.to_string(), depth);
      if depth >= 8 {
        continue;
      }
      if let Some(children) = self.edges.get(node) {
        for child in children {
          queue.push_back((child.as_str(), depth + 1));
        }
      }
    }

    depths
  }

  fn is_any_children_alias(&self, node: &str) -> bool {
    self.any_children_alias_keys.contains(node)
  }

  fn is_enum(&self, node: &str) -> bool {
    self.enum_keys.contains(node)
  }

  fn is_leaf_text_alias(&self, node: &str) -> bool {
    self.leaf_text_alias_keys.contains(node)
  }

  fn leaf_text_alias_content(&self, node: &str) -> Option<&TypeRefDecl> {
    self.leaf_text_alias_content.get(node)
  }

  fn is_wrapped_base(&self, node: &str) -> bool {
    self.wrapped_base_keys.contains(node)
  }

  fn is_no_prefix_only(&self, node: &str) -> bool {
    self.no_prefix_only_keys.contains(node)
  }

  fn rendered_module_path<'a>(&'a self, module_path: &'a str) -> &'a str {
    self
      .module_alias_paths
      .get(module_path)
      .map(String::as_str)
      .unwrap_or(module_path)
  }

  fn empty_leaf_marker_doc(&self, node: &str) -> Option<&str> {
    self.empty_leaf_marker_docs.get(node).map(String::as_str)
  }

  fn empty_leaf_marker_doc_by_rust_name(&self, rust_name: &str) -> Option<&str> {
    if self
      .empty_leaf_marker_ambiguous_rust_names
      .contains(rust_name)
    {
      return None;
    }
    self
      .empty_leaf_marker_docs_by_rust_name
      .get(rust_name)
      .map(String::as_str)
  }

  fn can_reach(&self, start: &str, goal: &str) -> bool {
    if start == goal {
      return true;
    }

    let mut stack = vec![start];
    let mut visited = HashSet::new();

    while let Some(node) = stack.pop() {
      if !visited.insert(node) {
        continue;
      }

      if let Some(children) = self.edges.get(node) {
        for child in children {
          if child == goal {
            return true;
          }
          stack.push(child.as_str());
        }
      }
    }

    false
  }
}

fn top_level_content_children(schema_type: &SchemaType) -> &[SchemaTypeChild] {
  if schema_type.children.len() == 1
    && schema_type.children[0].kind == SchemaTypeChildKind::Sequence
  {
    schema_type.children[0].children.as_slice()
  } else {
    schema_type.children.as_slice()
  }
}

pub(crate) fn schema_qname_element_name(qname: &str) -> &str {
  qname.split('/').nth(1).unwrap_or(qname)
}

fn child_qname_field_rust_name(name: &str) -> String {
  schema_child_field_rust_name(schema_qname_element_name(name))
}

fn child_qname_prefix(name: &str) -> Option<&str> {
  schema_qname_element_name(name)
    .split_once(':')
    .map(|(prefix, _)| prefix)
}

fn child_preferred_field_rust_name(
  child: &SchemaTypeChild,
  context: &CodegenContext<'_>,
) -> String {
  if !child.property_name.is_empty() {
    return schema_child_field_rust_name(child.property_name.as_str());
  }

  context
    .type_by_name(child.name.as_str())
    .map(|child_type| schema_child_field_rust_name(child_type.class_name.as_str()))
    .unwrap_or_else(|| child_qname_field_rust_name(child.name.as_str()))
}

fn child_conflict_field_rust_name(child: &SchemaTypeChild, context: &CodegenContext<'_>) -> String {
  let Some(child_type) = context.type_by_name(child.name.as_str()) else {
    return child_qname_field_rust_name(child.name.as_str());
  };

  if let Some(prefix) = child_qname_prefix(child.name.as_str()) {
    schema_child_field_rust_name(format!("{prefix}_{}", child_type.class_name).as_str())
  } else {
    child_qname_field_rust_name(child.name.as_str())
  }
}

pub(crate) fn duplicate_preferred_child_field_names<'a>(
  children: impl IntoIterator<Item = &'a SchemaTypeChild>,
  context: &CodegenContext<'_>,
) -> HashSet<String> {
  let mut counts = HashMap::<String, usize>::new();

  for child in children {
    if matches!(
      child.kind,
      SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild
    ) {
      *counts
        .entry(child_preferred_field_rust_name(child, context))
        .or_insert(0) += 1;
    }
  }

  counts
    .into_iter()
    .filter_map(|(name, count)| (count > 1).then_some(name))
    .collect()
}

pub(crate) fn schema_attr_field_rust_name(attr: &SchemaTypeAttribute) -> String {
  if attr.property_name.is_empty() {
    escape_snake_case(attr.q_name.to_snake_case())
  } else {
    escape_snake_case(attr.property_name.to_snake_case())
  }
}

pub(crate) fn duplicate_preferred_child_field_names_with_attrs<'a>(
  attrs: impl IntoIterator<Item = &'a SchemaTypeAttribute>,
  children: impl IntoIterator<Item = &'a SchemaTypeChild>,
  context: &CodegenContext<'_>,
) -> HashSet<String> {
  let mut counts = HashMap::<String, usize>::new();

  for attr in attrs {
    *counts.entry(schema_attr_field_rust_name(attr)).or_insert(0) += 1;
  }

  for child in children {
    if matches!(
      child.kind,
      SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild
    ) {
      *counts
        .entry(child_preferred_field_rust_name(child, context))
        .or_insert(0) += 1;
    }
  }

  counts
    .into_iter()
    .filter_map(|(name, count)| (count > 1).then_some(name))
    .collect()
}

pub(crate) fn schema_child_field_rust_name_with_context(
  child: &SchemaTypeChild,
  context: &CodegenContext<'_>,
  duplicate_preferred_names: &HashSet<String>,
) -> String {
  let preferred_name = child_preferred_field_rust_name(child, context);
  if child.property_name.is_empty() && duplicate_preferred_names.contains(&preferred_name) {
    child_conflict_field_rust_name(child, context)
  } else {
    preferred_name
  }
}

pub(crate) fn schema_any_child_field_rust_name(child: &SchemaTypeChild) -> String {
  if child.property_name.is_empty() {
    "unknown_xml".to_string()
  } else {
    schema_child_property_field_rust_name(child)
  }
}

pub(crate) fn schema_child_property_field_rust_name(child: &SchemaTypeChild) -> String {
  schema_child_field_rust_name(child.property_name.as_str())
}

pub(crate) fn schema_child_override_field_rust_name(child: &SchemaTypeChild) -> Option<String> {
  (!child.property_name.is_empty()).then(|| schema_child_property_field_rust_name(child))
}

pub(crate) fn schema_empty_text_child_field_name(child: &SchemaTypeChild) -> &str {
  if child.property_name.is_empty() {
    "Text"
  } else {
    child.property_name.as_str()
  }
}

pub(crate) fn schema_composite_any_variant_name(child: &SchemaTypeChild) -> &str {
  if child.property_name.is_empty() {
    "UnknownXml"
  } else {
    child.property_name.as_str()
  }
}

pub(crate) fn schema_composite_child_variant_name(child: &SchemaTypeChild) -> Result<&str> {
  let element_name = schema_qname_element_name(child.name.as_str());
  if element_name.is_empty() {
    Err(child.name.clone().into())
  } else {
    Ok(element_name)
  }
}

pub(crate) fn schema_choice_variant_name_from_qname(qname: &str) -> String {
  let element_qname = schema_qname_element_name(qname);
  let mut parts = element_qname.split(':');
  match (parts.next(), parts.next()) {
    (Some(prefix), Some(local)) => {
      format!(
        "{}{}",
        prefix.to_upper_camel_case(),
        local.to_upper_camel_case()
      )
    }
    (Some(local), None) => local.to_upper_camel_case(),
    _ => element_qname.to_upper_camel_case(),
  }
}

fn supports_default_namespace_write(module_prefix: &str) -> bool {
  matches!(
    module_prefix,
    "ap" | "cp" | "op" | "x" | "x14" | "xltc" | "xlrd" | "xlrd2" | "xnsv" | "pct"
  )
}

fn type_qname_prefix(qname: &str) -> Option<&str> {
  schema_qname_element_name(qname)
    .split_once(':')
    .map(|(prefix, _)| prefix)
}

fn type_uses_default_namespace(module_prefix: &str, qname: &str) -> bool {
  supports_default_namespace_write(module_prefix)
    && type_qname_prefix(qname).is_some_and(|prefix| prefix == module_prefix)
}

fn child_uses_parent_default_namespace(parent_prefix: &str, qname: &str) -> bool {
  schema_qname_element_name(qname)
    .split_once(':')
    .is_some_and(|(prefix, _)| prefix == parent_prefix)
}

fn no_prefix_sdk_attr_from_qname(
  module: &SchemaModuleDecl,
  type_decl: &TypeDecl,
  type_graph: &TypeContainmentGraph,
) -> TokenStream {
  let Some(qname) = type_decl.xml_qname.as_deref() else {
    return quote! {};
  };
  if !type_uses_default_namespace(module.prefix.as_str(), qname) {
    return quote! {};
  }
  let type_key = local_type_key(module, &type_decl.rust_name);
  if type_graph.is_no_prefix_only(&type_key) {
    quote! { no_prefix_only, }
  } else {
    quote! { no_prefix, }
  }
}

fn collect_prefixed_write_requirement_for_child(
  module: &SchemaModuleDecl,
  parent_prefix: &str,
  qname: &str,
  type_ref: &TypeRefDecl,
  no_prefix_keys: &HashSet<String>,
  needs_prefixed_write_keys: &mut HashSet<String>,
) {
  let Some(target_key) = schema_type_key_from_ref(module, type_ref) else {
    return;
  };
  if no_prefix_keys.contains(&target_key)
    && !child_uses_parent_default_namespace(parent_prefix, qname)
  {
    needs_prefixed_write_keys.insert(target_key);
  }
}

fn collect_prefixed_write_descendant_for_child(
  module: &SchemaModuleDecl,
  type_ref: &TypeRefDecl,
  no_prefix_keys: &HashSet<String>,
  needs_prefixed_write_keys: &mut HashSet<String>,
) {
  let Some(target_key) = schema_type_key_from_ref(module, type_ref) else {
    return;
  };
  if no_prefix_keys.contains(&target_key) {
    needs_prefixed_write_keys.insert(target_key);
  }
}

fn collect_prefixed_write_descendants_from_choice(
  choice_type_decl: &TypeDecl,
  choice_module: &SchemaModuleDecl,
  no_prefix_keys: &HashSet<String>,
  type_decl_by_key: &HashMap<String, (&SchemaModuleDecl, &TypeDecl)>,
  needs_prefixed_write_keys: &mut HashSet<String>,
) {
  for member in &choice_type_decl.members {
    let MemberDecl::Variant(variant) = member else {
      continue;
    };
    match &variant.wire {
      VariantWireDecl::Child { .. } => {
        collect_prefixed_write_descendant_for_child(
          choice_module,
          &variant.payload,
          no_prefix_keys,
          needs_prefixed_write_keys,
        );
      }
      VariantWireDecl::Sequence { .. } => {
        let Some(helper_key) = schema_type_key_from_ref(choice_module, &variant.payload) else {
          continue;
        };
        let Some((helper_module, helper_type_decl)) = type_decl_by_key.get(&helper_key).copied()
        else {
          continue;
        };
        collect_prefixed_write_descendants_from_members(
          helper_module,
          helper_type_decl.members.as_slice(),
          no_prefix_keys,
          type_decl_by_key,
          needs_prefixed_write_keys,
        );
      }
      VariantWireDecl::Any { .. } | VariantWireDecl::Text | VariantWireDecl::TextChild { .. } => {}
    }
  }
}

fn collect_prefixed_write_descendants_from_members(
  module: &SchemaModuleDecl,
  members: &[MemberDecl],
  no_prefix_keys: &HashSet<String>,
  type_decl_by_key: &HashMap<String, (&SchemaModuleDecl, &TypeDecl)>,
  needs_prefixed_write_keys: &mut HashSet<String>,
) {
  for member in members {
    let MemberDecl::Field(field) = member else {
      continue;
    };
    match &field.wire {
      FieldWireDecl::Child { .. } => {
        collect_prefixed_write_descendant_for_child(
          module,
          &field.type_ref,
          no_prefix_keys,
          needs_prefixed_write_keys,
        );
      }
      FieldWireDecl::Choice => {
        let Some(choice_key) = schema_type_key_from_ref(module, &field.type_ref) else {
          continue;
        };
        let Some((choice_module, choice_type_decl)) = type_decl_by_key.get(&choice_key).copied()
        else {
          continue;
        };
        collect_prefixed_write_descendants_from_choice(
          choice_type_decl,
          choice_module,
          no_prefix_keys,
          type_decl_by_key,
          needs_prefixed_write_keys,
        );
      }
      FieldWireDecl::Attribute { .. }
      | FieldWireDecl::Text
      | FieldWireDecl::TextChild { .. }
      | FieldWireDecl::Any => {}
    }
  }
}

fn collect_prefixed_write_requirements_from_choice(
  choice_type_decl: &TypeDecl,
  choice_module: &SchemaModuleDecl,
  parent_prefix: &str,
  no_prefix_keys: &HashSet<String>,
  type_decl_by_key: &HashMap<String, (&SchemaModuleDecl, &TypeDecl)>,
  needs_prefixed_write_keys: &mut HashSet<String>,
) {
  for member in &choice_type_decl.members {
    let MemberDecl::Variant(variant) = member else {
      continue;
    };
    match &variant.wire {
      VariantWireDecl::Child { qnames } => {
        for qname in qnames {
          collect_prefixed_write_requirement_for_child(
            choice_module,
            parent_prefix,
            qname,
            &variant.payload,
            no_prefix_keys,
            needs_prefixed_write_keys,
          );
        }
      }
      VariantWireDecl::Sequence { .. } => {
        let Some(helper_key) = schema_type_key_from_ref(choice_module, &variant.payload) else {
          continue;
        };
        let Some((helper_module, helper_type_decl)) = type_decl_by_key.get(&helper_key).copied()
        else {
          continue;
        };
        collect_prefixed_write_requirements_from_members(
          helper_module,
          helper_type_decl.members.as_slice(),
          parent_prefix,
          no_prefix_keys,
          type_decl_by_key,
          needs_prefixed_write_keys,
        );
      }
      VariantWireDecl::Any { .. } | VariantWireDecl::Text | VariantWireDecl::TextChild { .. } => {}
    }
  }
}

fn collect_prefixed_write_requirements_from_members(
  module: &SchemaModuleDecl,
  members: &[MemberDecl],
  parent_prefix: &str,
  no_prefix_keys: &HashSet<String>,
  type_decl_by_key: &HashMap<String, (&SchemaModuleDecl, &TypeDecl)>,
  needs_prefixed_write_keys: &mut HashSet<String>,
) {
  for member in members {
    let MemberDecl::Field(field) = member else {
      continue;
    };
    match &field.wire {
      FieldWireDecl::Child { qname } => collect_prefixed_write_requirement_for_child(
        module,
        parent_prefix,
        qname,
        &field.type_ref,
        no_prefix_keys,
        needs_prefixed_write_keys,
      ),
      FieldWireDecl::Choice => {
        let Some(choice_key) = schema_type_key_from_ref(module, &field.type_ref) else {
          continue;
        };
        let Some((choice_module, choice_type_decl)) = type_decl_by_key.get(&choice_key).copied()
        else {
          continue;
        };
        collect_prefixed_write_requirements_from_choice(
          choice_type_decl,
          choice_module,
          parent_prefix,
          no_prefix_keys,
          type_decl_by_key,
          needs_prefixed_write_keys,
        );
      }
      FieldWireDecl::Attribute { .. }
      | FieldWireDecl::Any
      | FieldWireDecl::Text
      | FieldWireDecl::TextChild { .. } => {}
    }
  }
}

pub(crate) fn schema_choice_variant_name_from_field_name(field_name: &str) -> String {
  field_name
    .strip_prefix("r#")
    .unwrap_or(field_name)
    .to_upper_camel_case()
}

pub(crate) fn schema_choice_variant_name_from_child(
  child: &SchemaTypeChild,
  context: &CodegenContext<'_>,
) -> String {
  if !child.property_name.is_empty() {
    return schema_choice_variant_name_from_field_name(child.property_name.as_str());
  }

  context
    .type_by_name(child.name.as_str())
    .map(|child_type| schema_choice_variant_name_from_field_name(child_type.class_name.as_str()))
    .unwrap_or_else(|| schema_choice_variant_name_from_qname(child.name.as_str()))
}

pub(crate) fn schema_choice_variant_conflict_name(prefix: &str, class_name: &str) -> String {
  format!(
    "{}{}",
    prefix.to_upper_camel_case(),
    class_name.to_upper_camel_case()
  )
}

pub(crate) fn schema_choice_variant_qname_prefix(qname: &str) -> Option<&str> {
  schema_qname_element_name(qname).split(':').next()
}

pub(crate) fn schema_recursive_choice_display_name(
  child: &SchemaTypeChild,
  child_index: usize,
) -> String {
  match child.kind {
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild => {
      schema_qname_element_name(child.name.as_str()).to_string()
    }
    SchemaTypeChildKind::Any => {
      if child.property_name.is_empty() {
        "any".to_string()
      } else {
        child.property_name.clone()
      }
    }
    SchemaTypeChildKind::Choice => {
      if child.property_name.is_empty() {
        format!("choice{}", child_index + 1)
      } else {
        child.property_name.clone()
      }
    }
    SchemaTypeChildKind::Sequence => {
      if child.property_name.is_empty() {
        format!("sequence{}", child_index + 1)
      } else {
        child.property_name.clone()
      }
    }
  }
}

pub(crate) fn schema_recursive_choice_variant_name(
  child: &SchemaTypeChild,
  child_index: usize,
  context: &CodegenContext<'_>,
) -> String {
  match child.kind {
    SchemaTypeChildKind::Choice | SchemaTypeChildKind::Sequence => {
      if child.property_name.is_empty() {
        format!(
          "{}{}",
          match child.kind {
            SchemaTypeChildKind::Choice => "Choice",
            SchemaTypeChildKind::Sequence => "Sequence",
            _ => unreachable!(),
          },
          child_index + 1
        )
      } else {
        child.property_name.to_upper_camel_case()
      }
    }
    _ => schema_choice_variant_name_from_child(child, context),
  }
}

pub(crate) fn schema_mixed_choice_leaf_field_rust_name(
  child: &ResolvedCompositeChild<'_>,
) -> String {
  schema_child_field_rust_name(child.variant_name.as_ref())
}

impl<'a> CodegenContext<'a> {
  pub fn new(schemas: &'a [Schema]) -> Self {
    Self::new_with_namespaces(schemas, &[])
  }

  pub fn new_with_namespaces(schemas: &'a [Schema], namespaces: &'a [Namespace]) -> Self {
    let mut enum_type_map = HashMap::new();
    let mut enum_type_module_map = HashMap::new();
    let mut type_map = HashMap::new();
    let mut type_class_name_map = HashMap::new();
    let mut type_module_map = HashMap::new();
    let mut type_prefix_map = HashMap::new();
    let namespace_version_map = namespaces
      .iter()
      .map(|namespace| (namespace.prefix.as_str(), namespace.version.as_str()))
      .collect();
    let mut enum_module_by_typed_namespace_and_name = HashMap::new();

    for schema in schemas {
      for schema_type in &schema.types {
        type_map.insert(schema_type.name.as_str(), schema_type);
        type_class_name_map.insert(schema_type.class_name.as_str(), schema_type);
        type_module_map.insert(schema_type.name.as_str(), schema.module_name.as_str());
        type_prefix_map.insert(schema_type.name.as_str(), schema.prefix.as_str());
      }

      for schema_enum in &schema.enums {
        enum_type_map.insert(schema_enum.r#type.as_str(), schema_enum);
        enum_type_module_map.insert(schema_enum.r#type.as_str(), schema.module_name.as_str());
        enum_module_by_typed_namespace_and_name.insert(
          (schema.typed_namespace.as_str(), schema_enum.name.as_str()),
          schema.module_name.as_str(),
        );
      }
    }

    Self {
      enum_type_map,
      enum_type_module_map,
      type_map,
      type_class_name_map,
      type_module_map,
      type_prefix_map,
      namespace_version_map,
      enum_module_by_typed_namespace_and_name,
    }
  }

  pub(crate) fn element_version_override(&self, schema_type: &'a SchemaType) -> Option<&'a str> {
    let qname = schema_type.name.rsplit('/').next()?;
    let (prefix, local_name) = qname.split_once(':')?;
    if local_name.is_empty() {
      return None;
    }
    let namespace_version = self
      .namespace_version_map
      .get(prefix)
      .copied()
      .unwrap_or("");
    let type_version = schema_type.version.as_deref().unwrap_or("");
    (schema_version_rank(type_version) > schema_version_rank(namespace_version))
      .then_some(type_version)
      .filter(|version| !version.is_empty())
  }

  pub fn enum_by_type(&self, ty: &str) -> Option<&'a SchemaEnum> {
    self.enum_type_map.get(ty).copied()
  }

  pub fn enum_module_by_type(&self, ty: &str) -> Option<&'a str> {
    self.enum_type_module_map.get(ty).copied()
  }

  pub fn type_by_name(&self, name: &str) -> Option<&'a SchemaType> {
    self.type_map.get(name).copied()
  }

  pub fn type_by_class_name(&self, class_name: &str) -> Option<&'a SchemaType> {
    self.type_class_name_map.get(class_name).copied()
  }

  pub fn derived_base_type(&self, schema_type: &SchemaType) -> Option<&'a SchemaType> {
    let base_type_name = schema_type
      .name
      .find('/')
      .map(|index| &schema_type.name[..index + 1])?;

    if base_type_name == schema_type.name {
      return None;
    }

    self.type_by_name(base_type_name)
  }

  pub fn type_prefix(&self, name: &str) -> Option<&'a str> {
    self.type_prefix_map.get(name).copied()
  }

  pub fn type_module(&self, name: &str) -> Option<&'a str> {
    self.type_module_map.get(name).copied()
  }

  pub fn enum_module_by_typed_namespace_and_name(
    &self,
    typed_namespace: &str,
    enum_name: &str,
  ) -> Option<&'a str> {
    self
      .enum_module_by_typed_namespace_and_name
      .get(&(typed_namespace, enum_name))
      .copied()
  }

  pub fn resolve_one_sequence_child(
    &self,
    schema_type: &'a SchemaType,
    particle_name: &'a str,
  ) -> Result<ResolvedOneSequenceChild<'a>> {
    if let Some(child) = top_level_content_children(schema_type)
      .iter()
      .find(|child| child.name == particle_name)
    {
      if child.kind == SchemaTypeChildKind::Any {
        let field_name = Cow::Owned(schema_any_child_field_rust_name(child));
        let property_comments = if child.property_comments.is_empty() {
          Cow::Borrowed(" _")
        } else {
          Cow::Borrowed(child.property_comments.as_str())
        };

        return Ok(ResolvedOneSequenceChild {
          name: child.name.as_str(),
          field_name,
          variant_name: Cow::Borrowed("XmlAny"),
          property_comments,
          version: child.initial_version.as_str(),
          kind: child.kind,
          optional: child.optional,
          repeated: child.repeated,
        });
      }

      let child_type = self.type_by_name(child.name.as_str()).ok_or_else(|| {
        format!(
          "missing direct one-sequence child type for schema {} child {:?} kind {:?}",
          schema_type.name, child.name, child.kind
        )
      })?;
      let duplicate_preferred_names = duplicate_preferred_child_field_names_with_attrs(
        schema_type.attributes.iter(),
        top_level_content_children(schema_type).iter(),
        self,
      );
      let field_name = Cow::Owned(schema_child_field_rust_name_with_context(
        child,
        self,
        &duplicate_preferred_names,
      ));
      let property_comments = if child.property_comments.is_empty() {
        Cow::Borrowed(" _")
      } else {
        Cow::Borrowed(child.property_comments.as_str())
      };

      return Ok(ResolvedOneSequenceChild {
        name: child.name.as_str(),
        field_name,
        variant_name: Cow::Owned(schema_choice_variant_name_from_child(child, self)),
        property_comments,
        version: schema_item_version(child_type),
        kind: child.kind,
        optional: child.optional,
        repeated: child.repeated,
      });
    }

    let child_type = self.type_by_name(particle_name).ok_or_else(|| {
      format!(
        "missing nested one-sequence child type for schema {} particle {:?}",
        schema_type.name, particle_name
      )
    })?;

    Ok(ResolvedOneSequenceChild {
      name: particle_name,
      field_name: Cow::Owned(schema_child_field_rust_name(child_type.class_name.as_str())),
      variant_name: Cow::Owned(schema_choice_variant_name_from_field_name(
        child_type.class_name.as_str(),
      )),
      property_comments: if child_type.summary.is_empty() {
        Cow::Borrowed(" _")
      } else {
        Cow::Owned(child_type.summary.clone())
      },
      version: schema_item_version(child_type),
      kind: child_kind_for_schema_type(child_type),
      optional: false,
      repeated: false,
    })
  }

  pub fn resolve_one_sequence_choice(
    &self,
    schema_type: &'a SchemaType,
    choice_child: &'a crate::sdk_data::sdk_data_model::SchemaTypeChild,
    choice_slot_count: usize,
    slot_index: usize,
  ) -> Result<ResolvedOneSequenceChoice<'a>> {
    let mut variants = Vec::new();

    for child in &choice_child.children {
      if child.kind == SchemaTypeChildKind::TextChild && child.name.is_empty() {
        variants.push(ResolvedOneSequenceChild {
          name: "",
          field_name: Cow::Borrowed(schema_empty_text_child_field_name(child)),
          variant_name: Cow::Borrowed(schema_empty_text_child_field_name(child)),
          property_comments: Cow::Borrowed(" _"),
          version: child.initial_version.as_str(),
          kind: child.kind,
          optional: child.optional,
          repeated: child.repeated,
        });
      } else if child.kind == SchemaTypeChildKind::Any {
        variants.push(ResolvedOneSequenceChild {
          name: "",
          field_name: Cow::Owned(schema_any_child_field_rust_name(child)),
          variant_name: Cow::Borrowed("XmlAny"),
          property_comments: if child.property_comments.is_empty() {
            Cow::Borrowed(" _")
          } else {
            Cow::Borrowed(child.property_comments.as_str())
          },
          version: child.initial_version.as_str(),
          kind: child.kind,
          optional: child.optional,
          repeated: child.repeated,
        });
      } else {
        let child_type = self.type_by_name(child.name.as_str()).ok_or_else(|| {
          format!(
            "missing direct one-sequence child type for schema {} child {:?} kind {:?}",
            schema_type.name, child.name, child.kind
          )
        })?;
        let duplicate_preferred_names =
          duplicate_preferred_child_field_names(choice_child.children.iter(), self);
        variants.push(ResolvedOneSequenceChild {
          name: child.name.as_str(),
          field_name: Cow::Owned(schema_child_field_rust_name_with_context(
            child,
            self,
            &duplicate_preferred_names,
          )),
          variant_name: Cow::Owned(schema_choice_variant_name_from_child(child, self)),
          property_comments: if child.property_comments.is_empty() {
            Cow::Borrowed(" _")
          } else {
            Cow::Borrowed(child.property_comments.as_str())
          },
          version: schema_item_version(child_type),
          kind: child.kind,
          optional: child.optional,
          repeated: child.repeated,
        });
      }
    }

    let field_name = one_sequence_choice_field_name(schema_type, choice_slot_count, slot_index);
    let enum_name = one_sequence_choice_enum_name(schema_type, choice_slot_count, slot_index);
    let property_comments = format!(
      "Choice of {}",
      variants
        .iter()
        .map(choice_child_display_name)
        .collect::<Vec<_>>()
        .join(", ")
    );

    Ok(ResolvedOneSequenceChoice {
      field_name,
      enum_name,
      property_comments,
      variants,
    })
  }

  pub fn resolve_one_sequence_structured_choice(
    &self,
    schema_type: &'a SchemaType,
    choice: &StructuredChoice<'a>,
    choice_slot_count: usize,
    slot_index: usize,
  ) -> Result<ResolvedOneSequenceStructuredChoice<'a>> {
    let field_name = one_sequence_choice_field_name(schema_type, choice_slot_count, slot_index);
    let enum_name = one_sequence_choice_enum_name(schema_type, choice_slot_count, slot_index);
    let mut variants = Vec::new();
    let mut property_comment_parts = Vec::new();

    for (variant_index, variant) in choice.variants.iter().enumerate() {
      match variant {
        StructuredChoiceVariant::Leaf(particle) => {
          let child = self.resolve_one_sequence_child(schema_type, particle.name.as_str())?;
          property_comment_parts.push(schema_qname_element_name(child.name).to_string());
          variants.push(ResolvedOneSequenceChoiceVariant::Leaf(child));
        }
        StructuredChoiceVariant::Sequence(sequence_particles) => {
          let struct_name = one_sequence_choice_sequence_struct_name(
            schema_type,
            choice_slot_count,
            slot_index,
            variant_index,
          );
          let variant_name = format!("Sequence{}", variant_index + 1);
          let mut fields = Vec::new();
          let mut field_parts = Vec::new();

          for particle in sequence_particles {
            let StructuredParticleKind::Leaf(leaf) = &particle.kind else {
              return Err(format!("{:?}", schema_type.name).into());
            };
            let child = self.resolve_one_sequence_child(schema_type, leaf.name.as_str())?;
            field_parts.push(schema_qname_element_name(child.name).to_string());
            fields.push(ResolvedOneSequenceSequenceField {
              child,
              optional: particle.optional,
              repeated: particle.repeated,
              initial_version: particle.initial_version,
            });
          }

          property_comment_parts.push(format!("sequence of {}", field_parts.join(", ")));
          variants.push(ResolvedOneSequenceChoiceVariant::Sequence(
            ResolvedOneSequenceSequenceVariant {
              variant_name,
              struct_name,
              property_comments: format!(" Sequence of {}", field_parts.join(", ")),
              fields,
            },
          ));
        }
      }
    }

    Ok(ResolvedOneSequenceStructuredChoice {
      field_name,
      enum_name,
      property_comments: format!("Choice of {}", property_comment_parts.join(", ")),
      variants,
    })
  }

  pub fn resolve_children(
    &self,
    schema_type: &'a SchemaType,
  ) -> Result<Vec<ResolvedCompositeChild<'a>>> {
    let mut resolved = Vec::new();
    let mut resolved_names = std::collections::HashSet::new();

    self.collect_resolved_children(
      schema_type,
      &schema_type.children,
      &mut resolved,
      &mut resolved_names,
      false,
    )?;

    Ok(resolved)
  }

  pub fn resolve_one_choice(&self, schema_type: &'a SchemaType) -> Result<ResolvedOneChoice<'a>> {
    let choice_child = schema_type
      .children
      .first()
      .filter(|child| child.kind == SchemaTypeChildKind::Choice)
      .ok_or_else(|| format!("{:?}", schema_type.name))?;

    let mut variants = Vec::with_capacity(choice_child.children.len());

    for child in &choice_child.children {
      if !matches!(
        child.kind,
        SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any
      ) {
        return Err(format!("{:?}", schema_type.name).into());
      }

      let mut variant = self.resolve_one_sequence_child(schema_type, child.name.as_str())?;
      variant.optional = child.optional;
      variant.repeated = child.repeated;
      variants.push(variant);
    }

    Ok(ResolvedOneChoice {
      field_name: one_sequence_choice_field_name(schema_type, 1, 0),
      enum_name: one_sequence_choice_enum_name(schema_type, 1, 0),
      variants,
    })
  }

  fn collect_resolved_children(
    &self,
    _schema_type: &'a SchemaType,
    children: &'a [SchemaTypeChild],
    resolved: &mut Vec<ResolvedCompositeChild<'a>>,
    resolved_names: &mut std::collections::HashSet<String>,
    preserve_sequences: bool,
  ) -> Result<()> {
    for child in children {
      match child.kind {
        SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any => {
          self.push_resolved_child(child, resolved, resolved_names)?;
        }
        SchemaTypeChildKind::Choice => {
          self.collect_resolved_children(
            _schema_type,
            &child.children,
            resolved,
            resolved_names,
            true,
          )?;
        }
        SchemaTypeChildKind::Sequence => {
          if preserve_sequences {
            let mut sequence_children = Vec::new();
            let mut sequence_resolved_names = std::collections::HashSet::new();
            self.collect_resolved_children(
              _schema_type,
              &child.children,
              &mut sequence_children,
              &mut sequence_resolved_names,
              true,
            )?;

            let mut leaf_versions = Vec::new();
            collect_resolved_sequence_leafs(&sequence_children, &mut leaf_versions);
            let sequence_version = common_choice_version(
              child.initial_version.as_str(),
              &leaf_versions
                .iter()
                .map(|field| field.version)
                .collect::<Vec<_>>(),
            );

            resolved.push(ResolvedCompositeChild {
              name: "",
              variant_name: Cow::Borrowed("Sequence"),
              version: sequence_version,
              is_any: false,
              kind: SchemaTypeChildKind::Sequence,
              optional: child.optional,
              repeated: child.repeated,
              children: sequence_children,
            });
          } else {
            self.collect_resolved_children(
              _schema_type,
              &child.children,
              resolved,
              resolved_names,
              preserve_sequences,
            )?;
          }
        }
      }
    }

    Ok(())
  }

  fn push_resolved_child(
    &self,
    child: &'a SchemaTypeChild,
    resolved: &mut Vec<ResolvedCompositeChild<'a>>,
    resolved_names: &mut std::collections::HashSet<String>,
  ) -> Result<()> {
    if child.kind == SchemaTypeChildKind::TextChild && child.name.is_empty() {
      resolved.push(ResolvedCompositeChild {
        name: "",
        variant_name: Cow::Borrowed(schema_empty_text_child_field_name(child)),
        version: child.initial_version.as_str(),
        is_any: false,
        kind: child.kind,
        optional: child.optional,
        repeated: child.repeated,
        children: Vec::new(),
      });

      return Ok(());
    }

    if !child.name.is_empty() && !resolved_names.insert(child.name.clone()) {
      return Ok(());
    }

    let (variant_name, is_any) = if child.kind == SchemaTypeChildKind::Any {
      (
        Cow::Borrowed(schema_composite_any_variant_name(child)),
        true,
      )
    } else {
      (
        Cow::Borrowed(schema_composite_child_variant_name(child)?),
        false,
      )
    };

    resolved.push(ResolvedCompositeChild {
      name: child.name.as_str(),
      variant_name,
      version: self
        .type_by_name(child.name.as_str())
        .map(schema_item_version)
        .unwrap_or_default(),
      is_any,
      kind: child.kind,
      optional: child.optional,
      repeated: child.repeated,
      children: Vec::new(),
    });

    Ok(())
  }

  pub fn resolve_attr_enum_module<'b>(&self, attr_type: &'b str) -> Result<(&'a str, &'b str)> {
    let AttrTypeKind::Enum {
      typed_namespace,
      enum_name,
    } = classify_attr_type(attr_type).ok_or_else(|| attr_type.to_string())?
    else {
      return Err(attr_type.to_string().into());
    };

    let enum_module_name = self
      .enum_module_by_typed_namespace_and_name(typed_namespace, enum_name)
      .ok_or_else(|| format!("{typed_namespace:?}:{enum_name:?}"))?;

    Ok((enum_module_name, enum_name))
  }

  pub fn resolve_enum_values(&self, attr_type: &str) -> Result<Vec<String>> {
    let schema_enum = self
      .enum_type_map
      .get(attr_type)
      .copied()
      .ok_or_else(|| format!("failed to resolve enum values for {attr_type}"))?;
    Ok(
      schema_enum
        .facets
        .iter()
        .flat_map(|facet| std::iter::once(facet.value.clone()).chain(facet.aliases.clone()))
        .collect(),
    )
  }
}

pub(crate) fn one_sequence_choice_field_name(
  schema_type: &SchemaType,
  choice_slot_count: usize,
  slot_index: usize,
) -> String {
  if choice_slot_count <= 1
    && let Some(property_name) = top_level_content_children(schema_type)
      .iter()
      .filter(|child| child.kind == SchemaTypeChildKind::Choice)
      .nth(slot_index)
      .map(|child| child.property_name.as_str())
      .filter(|property_name| !property_name.is_empty() && *property_name != "children")
  {
    return schema_child_field_rust_name(property_name);
  }

  if choice_slot_count <= 1 {
    format!("{}_choice", schema_type.class_name.to_snake_case())
  } else {
    format!(
      "{}_choice{}",
      schema_type.class_name.to_snake_case(),
      slot_index + 1
    )
  }
}

pub(crate) fn schema_child_field_rust_name(raw_name: &str) -> String {
  let rust_name = escape_snake_case(raw_name.to_snake_case());
  if rust_name == "children" {
    "xml_children".to_string()
  } else {
    rust_name
  }
}

pub(crate) fn one_sequence_choice_enum_name(
  schema_type: &SchemaType,
  choice_slot_count: usize,
  slot_index: usize,
) -> String {
  let stem = one_sequence_choice_type_stem(schema_type, choice_slot_count, slot_index);
  if choice_slot_count <= 1 {
    stem
  } else {
    stem.to_string()
  }
}

pub(crate) fn one_sequence_choice_sequence_struct_name(
  schema_type: &SchemaType,
  choice_slot_count: usize,
  slot_index: usize,
  variant_index: usize,
) -> String {
  let stem = one_sequence_choice_type_stem(schema_type, choice_slot_count, slot_index);
  format!("{stem}Sequence{}", variant_index + 1)
}

fn one_sequence_choice_type_stem(
  schema_type: &SchemaType,
  choice_slot_count: usize,
  slot_index: usize,
) -> String {
  if choice_slot_count <= 1 {
    format!("{}Choice", schema_type.class_name.to_upper_camel_case())
  } else {
    format!(
      "{}Choice{}",
      schema_type.class_name.to_upper_camel_case(),
      slot_index + 1
    )
  }
}

pub fn gen_schema(
  schema: &Schema,
  prebuilt_ir: Option<&SchemaModuleDecl>,
  context: &CodegenContext<'_>,
  suppress_version_cfg_attrs: bool,
) -> Result<TokenStream> {
  let owned_ir;
  let ir = if let Some(ir) = prebuilt_ir {
    ir
  } else {
    owned_ir = build_codegen_ir(schema, context)?;
    &owned_ir
  };
  gen_schema_from_ir(ir, suppress_version_cfg_attrs)
}

pub fn gen_schema_from_ir(
  ir: &SchemaModuleDecl,
  suppress_version_cfg_attrs: bool,
) -> Result<TokenStream> {
  let type_graph = TypeContainmentGraph::from_modules(&[ir]);
  gen_schema_from_ir_with_type_graph(ir, suppress_version_cfg_attrs, &type_graph)
}

pub(crate) fn gen_schema_from_ir_with_type_graph(
  ir: &SchemaModuleDecl,
  suppress_version_cfg_attrs: bool,
  type_graph: &TypeContainmentGraph,
) -> Result<TokenStream> {
  let version_cfg = VersionCfgContext::new(suppress_version_cfg_attrs);
  let mut token_stream_list: Vec<TokenStream> = vec![];
  let mut helper_token_stream_list: Vec<(String, bool, bool, TokenStream)> = vec![];
  let omitted_empty_leaf_marker_type_names = empty_leaf_marker_type_names_to_omit(ir);
  let omitted_abstract_helper_type_names = omitted_abstract_helper_type_names(ir);
  let type_decl_by_name: std::collections::HashMap<_, _> = ir
    .types
    .iter()
    .map(|ty| (ty.rust_name.as_str(), ty))
    .collect();
  let schema_type_names: std::collections::HashSet<_> = ir
    .types
    .iter()
    .filter(|ty| matches!(ty.kind, TypeKind::ElementStruct | TypeKind::LeafTextAlias))
    .filter(|ty| !omitted_empty_leaf_marker_type_names.contains(ty.rust_name.as_str()))
    .filter(|ty| {
      should_emit_schema_type_decl(ty)
        || type_graph.is_wrapped_base(&local_type_key(ir, &ty.rust_name))
    })
    .map(|ty| ty.rust_name.as_str())
    .collect();

  for schema_enum in &ir.enums {
    token_stream_list.push(
      gen_schema_enum_from_decl(schema_enum, ir, version_cfg)
        .map_err(|err| format!("enum {}: {err}", schema_enum.rust_name))?,
    );
  }

  for type_decl in ir
    .types
    .iter()
    .filter(|ty| matches!(ty.kind, TypeKind::ElementStruct | TypeKind::LeafTextAlias))
    .filter(|ty| !omitted_empty_leaf_marker_type_names.contains(ty.rust_name.as_str()))
    .filter(|ty| {
      should_emit_schema_type_decl(ty)
        || type_graph.is_wrapped_base(&local_type_key(ir, &ty.rust_name))
    })
  {
    let attr_fields: Vec<&FieldDecl> = type_decl
      .members
      .iter()
      .filter_map(|member| match member {
        MemberDecl::Field(field) if matches!(field.wire, FieldWireDecl::Attribute { .. }) => {
          Some(field)
        }
        _ => None,
      })
      .collect();
    let direct_child_fields: Vec<&FieldDecl> = type_decl
      .members
      .iter()
      .filter_map(|member| match member {
        MemberDecl::Field(field)
          if matches!(
            field.wire,
            FieldWireDecl::Child { .. } | FieldWireDecl::TextChild { .. }
          ) =>
        {
          Some(field)
        }
        _ => None,
      })
      .collect();
    let choice_fields: Vec<&FieldDecl> = type_decl
      .members
      .iter()
      .filter_map(|member| match member {
        MemberDecl::Field(field) if matches!(field.wire, FieldWireDecl::Choice) => Some(field),
        _ => None,
      })
      .collect();

    let struct_name_ident: Ident = parse_str(&type_decl.rust_name.to_upper_camel_case())
      .map_err(|err| format!("type {} struct ident: {err}", type_decl.rust_name))?;
    let schema_type_version = type_decl.version.as_deref().unwrap_or_default();
    let type_attrs = version_cfg.attrs(schema_type_version);
    let field_version_cfg = if type_attrs.is_empty() {
      version_cfg
    } else {
      VersionCfgContext::new(true)
    };
    let type_sdk_version_markers = mce_element_version_markers(type_decl)?;
    let sdk_type_attrs = if let Some(raw_qname) = &type_decl.xml_qname {
      let qname = sdk_element_qname(raw_qname);
      let no_prefix = no_prefix_sdk_attr_from_qname(ir, type_decl, type_graph);
      let extra_xmlns = if type_decl.support.extra_xmlns.is_empty() {
        quote! {}
      } else {
        let prefixes = type_decl
          .support
          .extra_xmlns
          .iter()
          .map(|prefix| proc_macro2::Literal::string(prefix))
          .collect::<Vec<_>>();
        quote! { extra_xmlns(#(#prefixes),*), }
      };
      let canonical_namespace_prefix = if type_decl.support.canonical_namespace_prefixes.is_empty()
      {
        quote! {}
      } else {
        let prefixes = type_decl
          .support
          .canonical_namespace_prefixes
          .iter()
          .map(|prefix| proc_macro2::Literal::string(prefix))
          .collect::<Vec<_>>();
        quote! { canonical_namespace_prefix(#(#prefixes),*), }
      };
      let xml_header = if type_decl.support.has_xml_header {
        quote! { xml_header, }
      } else {
        quote! {}
      };
      quote! {
        #[sdk(#(#type_sdk_version_markers,)* #no_prefix #extra_xmlns #canonical_namespace_prefix #xml_header qname = #qname)]
      }
    } else {
      quote! {}
    };
    let summary_doc = format!(" {}", type_decl.docs);
    let sdk_type_derive = sdk_type_derive_tokens();

    if type_decl.kind == TypeKind::LeafTextAlias {
      let xml_content_type = type_from_decl_ref(
        type_decl
          .xml_content
          .as_ref()
          .ok_or_else(|| format!("type {} missing IR xml content", type_decl.rust_name))?,
        type_graph,
      )?;

      if can_alias_leaf_text_wrapper_decl(type_decl, &attr_fields) {
        token_stream_list.push(quote! {
          #( #type_attrs )*
          #[doc = #summary_doc]
          pub type #struct_name_ident = #xml_content_type;
        });

        continue;
      }

      let mut fields: Vec<TokenStream> = vec![];

      fields.extend(gen_support_fields(&type_decl.support));

      for attr in &attr_fields {
        fields.push(
          gen_attr_from_decl(attr, field_version_cfg, type_graph).map_err(|err| {
            format!(
              "type {} attr {}: {err}",
              type_decl.rust_name, attr.rust_name
            )
          })?,
        );
      }

      let sdk_text_attr = if is_list_type_ref(
        type_decl
          .xml_content
          .as_ref()
          .ok_or_else(|| format!("type {} missing IR xml content", type_decl.rust_name))?,
      ) {
        quote! { #[sdk(text(list))] }
      } else {
        quote! { #[sdk(text)] }
      };
      fields.push(quote! {
        #sdk_text_attr
        pub xml_content: Option<#xml_content_type>,
      });

      token_stream_list.push(quote! {
        #( #type_attrs )*
        #[doc = #summary_doc]
        #sdk_type_derive
        #sdk_type_attrs
        pub struct #struct_name_ident {
          #( #fields )*
        }
      });

      continue;
    }

    if can_alias_any_children_wrapper_decl(type_decl, &attr_fields) {
      token_stream_list.push(quote! {
        #( #type_attrs )*
        #[doc = #summary_doc]
        pub type #struct_name_ident = Vec<String>;
      });

      continue;
    }

    let derived_base_type = if let Some(base_type_decl) = type_decl
      .base_rust_name
      .as_deref()
      .and_then(|base_name| type_decl_by_name.get(base_name).copied())
      && can_wrap_derived_to_base_decl(type_decl, base_type_decl)
    {
      Some(TypeRefDecl {
        rust_type: base_type_decl.rust_name.clone(),
        module_path: None,
      })
    } else if type_decl.base_module_path.is_some()
      && type_decl.kind == TypeKind::ElementStruct
      && type_decl.element_kind == Some(ElementKind::Derived)
      && type_decl.members.is_empty()
      && type_decl.content_model.is_none()
      && type_decl.xml_content.is_some()
    {
      Some(TypeRefDecl {
        rust_type: type_decl
          .base_rust_name
          .clone()
          .ok_or_else(|| format!("type {} missing IR base type", type_decl.rust_name))?,
        module_path: type_decl.base_module_path.clone(),
      })
    } else {
      None
    };

    if let Some(base_type) = derived_base_type {
      let base_type_ident = type_from_decl_ref(&base_type, type_graph)?;

      token_stream_list.push(quote! {
        #( #type_attrs )*
        #[doc = #summary_doc]
        #sdk_type_derive
        #sdk_type_attrs
        pub struct #struct_name_ident(pub #base_type_ident);
      });

      continue;
    }

    let mut fields: Vec<TokenStream> = vec![];
    let items: Vec<TokenStream> = vec![];

    fields.extend(gen_support_fields(&type_decl.support));

    if type_decl.element_kind == Some(ElementKind::LeafText) {
      for attr in &attr_fields {
        fields.push(
          gen_attr_from_decl(attr, field_version_cfg, type_graph).map_err(|err| {
            format!(
              "type {} attr {}: {err}",
              type_decl.rust_name, attr.rust_name
            )
          })?,
        );
      }

      let simple_type_name = type_from_decl_ref(
        type_decl
          .xml_content
          .as_ref()
          .ok_or_else(|| format!("type {} missing IR xml content", type_decl.rust_name))?,
        type_graph,
      )
      .map_err(|err| format!("type {} xml content: {err}", type_decl.rust_name))?;
      let sdk_text_attr = if is_list_type_ref(
        type_decl
          .xml_content
          .as_ref()
          .ok_or_else(|| format!("type {} missing IR xml content", type_decl.rust_name))?,
      ) {
        quote! { #[sdk(text(list))] }
      } else {
        quote! { #[sdk(text)] }
      };
      fields.push(quote! {
        #sdk_text_attr
        pub xml_content: Option<#simple_type_name>,
      });
    } else if type_decl.element_kind == Some(ElementKind::Leaf) {
      for attr in &attr_fields {
        fields.push(gen_attr_from_decl(attr, field_version_cfg, type_graph)?);
      }
    } else if type_decl.element_kind == Some(ElementKind::Composite) {
      for attr in &attr_fields {
        fields.push(gen_attr_from_decl(attr, field_version_cfg, type_graph)?);
      }

      if let Some(xml_content) = type_decl.xml_content.as_ref() {
        let simple_type_name = type_from_decl_ref(xml_content, type_graph)
          .map_err(|err| format!("type {} xml content: {err}", type_decl.rust_name))?;
        let sdk_text_attr = if is_list_type_ref(xml_content) {
          quote! { #[sdk(text(list))] }
        } else {
          quote! { #[sdk(text)] }
        };
        fields.push(quote! {
          #sdk_text_attr
          pub xml_content: Option<#simple_type_name>,
        });
      }

      match type_decl.content_model.unwrap_or(ContentModelDecl::None) {
        ContentModelDecl::OneAllDirectChildren => {
          fields.extend(gen_direct_child_fields_from_decl(
            &direct_child_fields,
            &type_decl.rust_name,
            ir,
            type_graph,
            field_version_cfg,
            true,
          )?);
        }
        ContentModelDecl::DirectChildrenOnly => {
          fields.extend(gen_direct_child_fields_from_decl(
            &direct_child_fields,
            &type_decl.rust_name,
            ir,
            type_graph,
            field_version_cfg,
            false,
          )?);
        }
        ContentModelDecl::ChoiceOnly
        | ContentModelDecl::OneChoiceSingle
        | ContentModelDecl::SequenceSingleChoice => {
          if !direct_child_fields.is_empty() {
            fields.extend(gen_direct_child_fields_from_decl(
              &direct_child_fields,
              &type_decl.rust_name,
              ir,
              type_graph,
              field_version_cfg,
              false,
            )?);
          } else {
            fields.extend(gen_choice_fields_from_decl(
              &choice_fields,
              ir,
              type_graph,
              field_version_cfg,
              &HashSet::new(),
            )?);
          }
        }
        ContentModelDecl::MixedChoiceChildren => {
          let mixed_fields: Vec<&FieldDecl> = type_decl
            .members
            .iter()
            .filter_map(|member| match member {
              MemberDecl::Field(field)
                if matches!(
                  field.wire,
                  FieldWireDecl::Child { .. }
                    | FieldWireDecl::TextChild { .. }
                    | FieldWireDecl::Any
                    | FieldWireDecl::Choice
                ) =>
              {
                Some(field)
              }
              _ => None,
            })
            .collect();
          fields.extend(gen_flatten_one_sequence_fields_from_decl(
            &mixed_fields,
            &type_decl.rust_name,
            ir,
            type_graph,
            field_version_cfg,
            &HashSet::new(),
          )?);
        }
        ContentModelDecl::SequenceAnyOnly => {
          let any_field = type_decl.members.iter().find_map(|member| match member {
            MemberDecl::Field(field) if matches!(field.wire, FieldWireDecl::Any) => Some(field),
            _ => None,
          });
          let field_type: Type =
            parse_str("std::boxed::Box<[u8]>").expect("std::boxed::Box<[u8]> type");
          let field_attrs = module_version_cfg_attrs(
            any_field.map(|field| field.version.as_str()).unwrap_or(""),
            field_version_cfg,
          );
          fields.push(quote! {
            #( #field_attrs )*
            #[sdk(any)]
            pub xml_children: Vec<#field_type>,
          });
        }
        ContentModelDecl::SequenceDirectChildren => {
          fields.extend(gen_direct_child_fields_from_decl(
            &direct_child_fields,
            &type_decl.rust_name,
            ir,
            type_graph,
            field_version_cfg,
            false,
          )?);
        }
        ContentModelDecl::OneSequenceFlatten | ContentModelDecl::OneSequenceStructured => {
          let flatten_fields: Vec<&FieldDecl> = type_decl
            .members
            .iter()
            .filter_map(|member| match member {
              MemberDecl::Field(field)
                if matches!(
                  field.wire,
                  FieldWireDecl::Child { .. }
                    | FieldWireDecl::TextChild { .. }
                    | FieldWireDecl::Any
                    | FieldWireDecl::Choice
                ) =>
              {
                Some(field)
              }
              _ => None,
            })
            .collect();
          fields.extend(gen_flatten_one_sequence_fields_from_decl(
            &flatten_fields,
            &type_decl.rust_name,
            ir,
            type_graph,
            field_version_cfg,
            &HashSet::new(),
          )?);
        }
        ContentModelDecl::GenericChildrenFallback => {
          if !choice_fields.is_empty() {
            fields.extend(gen_choice_fields_from_decl(
              &choice_fields,
              ir,
              type_graph,
              field_version_cfg,
              &HashSet::new(),
            )?);
          } else if !direct_child_fields.is_empty() {
            fields.extend(gen_direct_child_fields_from_decl(
              &direct_child_fields,
              &type_decl.rust_name,
              ir,
              type_graph,
              field_version_cfg,
              false,
            )?);
          }
        }
        ContentModelDecl::None => {
          if !choice_fields.is_empty() {
            fields.extend(gen_choice_fields_from_decl(
              &choice_fields,
              ir,
              type_graph,
              field_version_cfg,
              &HashSet::new(),
            )?);
          }
        }
      }
    } else if type_decl.element_kind == Some(ElementKind::Derived) {
      let base_class_name = type_decl
        .base_rust_name
        .as_deref()
        .ok_or_else(|| format!("type {} missing IR base type", type_decl.rust_name))?;
      let base_type_decl = type_decl_by_name
        .get(base_class_name)
        .copied()
        .ok_or_else(|| format!("missing IR type for {:?}", base_class_name))?;
      let base_direct_child_fields: Vec<&FieldDecl> = base_type_decl
        .members
        .iter()
        .filter_map(|member| match member {
          MemberDecl::Field(field)
            if matches!(
              field.wire,
              FieldWireDecl::Child { .. } | FieldWireDecl::TextChild { .. }
            ) =>
          {
            Some(field)
          }
          _ => None,
        })
        .collect();

      let mut seen_attrs: Vec<&str> = attr_fields
        .iter()
        .flat_map(|attr| {
          let qname = match &attr.wire {
            FieldWireDecl::Attribute { qname, .. } => (!qname.is_empty()).then_some(qname.as_str()),
            _ => None,
          };
          [
            qname,
            (!attr.rust_name.is_empty()).then_some(attr.rust_name.as_str()),
          ]
          .into_iter()
          .flatten()
        })
        .collect();

      for attr in &attr_fields {
        fields.push(gen_attr_from_decl(attr, field_version_cfg, type_graph)?);
      }

      for attr in base_type_decl
        .members
        .iter()
        .filter_map(|member| match member {
          MemberDecl::Field(field) if matches!(field.wire, FieldWireDecl::Attribute { .. }) => {
            Some(field)
          }
          _ => None,
        })
      {
        let FieldWireDecl::Attribute { qname, .. } = &attr.wire else {
          continue;
        };

        if (!qname.is_empty() && seen_attrs.contains(&qname.as_str()))
          || (!attr.rust_name.is_empty() && seen_attrs.contains(&attr.rust_name.as_str()))
        {
          continue;
        }

        fields.push(gen_attr_from_decl(attr, field_version_cfg, type_graph)?);

        if !qname.is_empty() {
          seen_attrs.push(qname.as_str());
        }
        if !attr.rust_name.is_empty() {
          seen_attrs.push(attr.rust_name.as_str());
        }
      }

      match type_decl.content_model.unwrap_or(ContentModelDecl::None) {
        ContentModelDecl::OneAllDirectChildren => {
          fields.extend(gen_direct_child_fields_from_decl(
            if direct_child_fields.is_empty() {
              &base_direct_child_fields
            } else {
              &direct_child_fields
            },
            &type_decl.rust_name,
            ir,
            type_graph,
            field_version_cfg,
            true,
          )?);
        }
        ContentModelDecl::DirectChildrenOnly => {
          fields.extend(gen_direct_child_fields_from_decl(
            if direct_child_fields.is_empty() {
              &base_direct_child_fields
            } else {
              &direct_child_fields
            },
            &type_decl.rust_name,
            ir,
            type_graph,
            field_version_cfg,
            false,
          )?);
        }
        ContentModelDecl::ChoiceOnly
        | ContentModelDecl::OneChoiceSingle
        | ContentModelDecl::SequenceSingleChoice => {
          if !direct_child_fields.is_empty()
            || (choice_fields.is_empty() && !base_direct_child_fields.is_empty())
          {
            fields.extend(gen_direct_child_fields_from_decl(
              if direct_child_fields.is_empty() {
                &base_direct_child_fields
              } else {
                &direct_child_fields
              },
              &type_decl.rust_name,
              ir,
              type_graph,
              field_version_cfg,
              false,
            )?);
          } else {
            fields.extend(gen_choice_fields_from_decl(
              &choice_fields,
              ir,
              type_graph,
              field_version_cfg,
              &HashSet::new(),
            )?);
          }
        }
        ContentModelDecl::OneSequenceFlatten => {
          let flatten_fields: Vec<&FieldDecl> = type_decl
            .members
            .iter()
            .filter_map(|member| match member {
              MemberDecl::Field(field)
                if matches!(
                  field.wire,
                  FieldWireDecl::Child { .. }
                    | FieldWireDecl::TextChild { .. }
                    | FieldWireDecl::Any
                    | FieldWireDecl::Choice
                ) =>
              {
                Some(field)
              }
              _ => None,
            })
            .collect();
          fields.extend(gen_flatten_one_sequence_fields_from_decl(
            &flatten_fields,
            &type_decl.rust_name,
            ir,
            type_graph,
            field_version_cfg,
            &HashSet::new(),
          )?);
        }
        ContentModelDecl::MixedChoiceChildren => {
          let mixed_fields: Vec<&FieldDecl> = type_decl
            .members
            .iter()
            .filter_map(|member| match member {
              MemberDecl::Field(field)
                if matches!(
                  field.wire,
                  FieldWireDecl::Child { .. }
                    | FieldWireDecl::TextChild { .. }
                    | FieldWireDecl::Any
                    | FieldWireDecl::Choice
                ) =>
              {
                Some(field)
              }
              _ => None,
            })
            .collect();
          fields.extend(gen_flatten_one_sequence_fields_from_decl(
            &mixed_fields,
            &type_decl.rust_name,
            ir,
            type_graph,
            field_version_cfg,
            &HashSet::new(),
          )?);
        }
        ContentModelDecl::OneSequenceStructured => {
          let structured_fields: Vec<&FieldDecl> = type_decl
            .members
            .iter()
            .filter_map(|member| match member {
              MemberDecl::Field(field)
                if matches!(
                  field.wire,
                  FieldWireDecl::Child { .. }
                    | FieldWireDecl::TextChild { .. }
                    | FieldWireDecl::Any
                    | FieldWireDecl::Choice
                ) =>
              {
                Some(field)
              }
              _ => None,
            })
            .collect();
          fields.extend(gen_flatten_one_sequence_fields_from_decl(
            &structured_fields,
            &type_decl.rust_name,
            ir,
            type_graph,
            field_version_cfg,
            &HashSet::new(),
          )?);
        }
        ContentModelDecl::GenericChildrenFallback => {
          if !choice_fields.is_empty() {
            fields.extend(gen_choice_fields_from_decl(
              &choice_fields,
              ir,
              type_graph,
              field_version_cfg,
              &HashSet::new(),
            )?);
          } else if !direct_child_fields.is_empty() || !base_direct_child_fields.is_empty() {
            fields.extend(gen_direct_child_fields_from_decl(
              if direct_child_fields.is_empty() {
                &base_direct_child_fields
              } else {
                &direct_child_fields
              },
              &type_decl.rust_name,
              ir,
              type_graph,
              field_version_cfg,
              false,
            )?);
          }
        }
        ContentModelDecl::SequenceDirectChildren => {
          fields.extend(gen_direct_child_fields_from_decl(
            if direct_child_fields.is_empty() {
              &base_direct_child_fields
            } else {
              &direct_child_fields
            },
            &type_decl.rust_name,
            ir,
            type_graph,
            field_version_cfg,
            false,
          )?);
        }
        ContentModelDecl::SequenceAnyOnly => {
          let any_field = type_decl.members.iter().find_map(|member| match member {
            MemberDecl::Field(field) if matches!(field.wire, FieldWireDecl::Any) => Some(field),
            _ => None,
          });
          let field_type: Type =
            parse_str("std::boxed::Box<[u8]>").expect("std::boxed::Box<[u8]> type");
          let field_attrs = module_version_cfg_attrs(
            any_field.map(|field| field.version.as_str()).unwrap_or(""),
            field_version_cfg,
          );
          fields.push(quote! {
            #( #field_attrs )*
            #[sdk(any)]
            pub xml_children: Vec<#field_type>,
          });
        }
        ContentModelDecl::None => {
          if !choice_fields.is_empty() {
            fields.extend(gen_choice_fields_from_decl(
              &choice_fields,
              ir,
              type_graph,
              field_version_cfg,
              &HashSet::new(),
            )?);
          }
        }
      }

      if direct_child_fields.is_empty()
        && choice_fields.is_empty()
        && base_type_decl.element_kind == Some(ElementKind::LeafText)
      {
        let simple_type_name = type_from_decl_ref(
          type_decl
            .xml_content
            .as_ref()
            .ok_or_else(|| format!("type {} missing IR xml content", type_decl.rust_name))?,
          type_graph,
        )?;
        let sdk_text_attr = if is_list_type_ref(
          type_decl
            .xml_content
            .as_ref()
            .ok_or_else(|| format!("type {} missing IR xml content", type_decl.rust_name))?,
        ) {
          quote! { #[sdk(text(list))] }
        } else {
          quote! { #[sdk(text)] }
        };
        fields.push(quote! {
          #sdk_text_attr
          pub xml_content: Option<#simple_type_name>,
        });
      }
    } else {
      return Err(format!("{type_decl:?}").into());
    }

    token_stream_list.push(quote! {
      #( #type_attrs )*
      #[doc = #summary_doc]
      #sdk_type_derive
      #sdk_type_attrs
      pub struct #struct_name_ident {
        #( #fields )*
      }
      #( #items )*
    });
  }

  for type_decl in ir
    .types
    .iter()
    .filter(|type_decl| !schema_type_names.contains(type_decl.rust_name.as_str()))
  {
    match type_decl.kind {
      TypeKind::ChoiceEnum => {
        helper_token_stream_list.push((
          type_decl.rust_name.clone(),
          is_generated_anonymous_type_name(&type_decl.rust_name),
          omitted_abstract_helper_type_names.contains(type_decl.rust_name.as_str()),
          gen_choice_type_decl(type_decl, ir, type_graph, version_cfg)?,
        ));
      }
      TypeKind::HelperStruct => {
        helper_token_stream_list.push((
          type_decl.rust_name.clone(),
          is_generated_anonymous_type_name(&type_decl.rust_name),
          omitted_abstract_helper_type_names.contains(type_decl.rust_name.as_str()),
          gen_helper_struct_type_decl(type_decl, ir, type_graph, version_cfg)?,
        ));
      }
      _ => {}
    }
  }

  let kept_helper_tokens =
    prune_unreferenced_anonymous_emitted_types(&token_stream_list, helper_token_stream_list);
  let non_helper_type_names: HashSet<String> = schema_type_names
    .iter()
    .map(|name| (*name).to_string())
    .collect();
  let anonymous_type_rename_map =
    anonymous_emitted_type_rename_map(&non_helper_type_names, &kept_helper_tokens);

  if !anonymous_type_rename_map.is_empty() {
    token_stream_list = token_stream_list
      .into_iter()
      .map(|tokens| rename_token_stream_idents(tokens, &anonymous_type_rename_map))
      .collect();
  }

  token_stream_list.extend(
    kept_helper_tokens
      .into_iter()
      .map(|(_, _, _, tokens)| rename_token_stream_idents(tokens, &anonymous_type_rename_map)),
  );
  Ok(quote! {
    #( #token_stream_list )*
  })
}

fn sdk_type_derive_tokens() -> TokenStream {
  quote! {
    #[derive(
      Clone,
      Debug,
      Default,
      PartialEq,
      ooxmlsdk_derive::SdkType
    )]
  }
}

fn gen_schema_enum_from_decl(
  schema_enum: &EnumDecl,
  _module: &SchemaModuleDecl,
  version_cfg: VersionCfgContext,
) -> Result<TokenStream> {
  let enum_name_ident: Ident = parse_str(&schema_enum.rust_name.to_upper_camel_case())?;
  let schema_enum_version = schema_enum.version.as_deref().unwrap_or_default();
  let enum_attrs = version_cfg.attrs(schema_enum_version);
  let nested_version_cfg = version_cfg.child(schema_enum_version);
  let sdk_enum_attrs = quote! {};
  let default_facet = schema_enum.variants.first().expect("schema enum facet");

  let mut alias_map: HashMap<String, Vec<String>> = HashMap::new();
  for facet in &schema_enum.variants {
    for alias in &facet.aliases {
      alias_map
        .entry(facet.xml_value.clone())
        .or_default()
        .push(alias.clone());
    }
  }

  let variants = gen_schema_enum_decl_variants(
    &schema_enum.variants.iter().collect::<Vec<_>>(),
    default_facet,
    &alias_map,
    nested_version_cfg,
  )?;

  Ok(quote! {
    #( #enum_attrs )*
    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
    #sdk_enum_attrs
    pub enum #enum_name_ident {
      #( #variants, )*
    }
  })
}

fn gen_choice_type_decl(
  type_decl: &TypeDecl,
  module: &SchemaModuleDecl,
  type_graph: &TypeContainmentGraph,
  version_cfg: VersionCfgContext,
) -> Result<TokenStream> {
  let enum_ident: Ident = parse_str(&type_decl.rust_name.to_upper_camel_case())
    .map_err(|err| format!("choice {} enum ident: {err}", type_decl.rust_name))?;
  let type_version = type_decl.version.as_deref().unwrap_or_default();
  let variant_versions: Vec<&str> = type_decl
    .members
    .iter()
    .filter_map(|member| match member {
      MemberDecl::Variant(variant) => Some(variant.version.as_str()),
      _ => None,
    })
    .collect();
  let enum_version = common_choice_version(type_version, &variant_versions);
  let enum_attrs = version_cfg.attrs(enum_version);
  let variant_cfg = if enum_attrs.is_empty() {
    version_cfg
  } else {
    VersionCfgContext::new(true)
  };
  let rendered_variant_name_map = anonymous_variant_render_name_map(type_decl, module);
  let render_context = ChoiceVariantRenderContext {
    module,
    type_graph,
    variant_cfg,
    rendered_variant_name_map: &rendered_variant_name_map,
  };
  let mut variants = Vec::new();

  for member in type_decl
    .members
    .iter()
    .filter(|member| !matches!(member, MemberDecl::Variant(variant) if matches!(variant.wire, VariantWireDecl::Any { .. })))
    .chain(
      type_decl
        .members
        .iter()
        .filter(|member| matches!(member, MemberDecl::Variant(variant) if matches!(variant.wire, VariantWireDecl::Any { .. }))),
    )
  {
    let MemberDecl::Variant(variant) = member else {
      continue;
    };
    variants.extend(
      gen_choice_variant_tokens(variant, &render_context, quote! {}).map_err(|err| {
        format!(
          "choice {} variant {}: {err}",
          type_decl.rust_name, variant.rust_name
        )
      })?,
    );
  }

  Ok(quote! {
    #( #enum_attrs )*
    #[derive(Clone, Debug, PartialEq)]
    pub enum #enum_ident {
      #( #variants )*
    }
  })
}

fn type_decl_for_ref<'a>(
  module: &'a SchemaModuleDecl,
  type_ref: &TypeRefDecl,
) -> Option<&'a TypeDecl> {
  if type_ref.module_path.is_some() {
    return None;
  }
  module
    .types
    .iter()
    .find(|type_decl| type_decl.rust_name == type_ref.rust_type)
}

fn is_empty_leaf_marker_type(type_decl: &TypeDecl) -> bool {
  let is_leaf_element_marker = type_decl.element_kind == Some(ElementKind::Leaf)
    && type_decl.base_rust_name.as_deref() == Some("OpenXmlLeafElement");
  let is_empty_element_marker = matches!(
    type_decl.base_rust_name.as_deref(),
    Some("OpenXmlEmptyElement" | "EmptyType")
  );

  type_decl.kind == TypeKind::ElementStruct
    && (is_leaf_element_marker || is_empty_element_marker)
    && !type_decl.is_abstract
    && type_decl.xml_content.is_none()
    && type_decl.members.is_empty()
    && !type_decl.support.have_xmlns_fields
    && !type_decl.support.has_xml_header
    && !type_decl.support.has_extra_support_fields()
}

fn is_abstract_empty_base_type(type_decl: &TypeDecl) -> bool {
  type_decl.kind == TypeKind::ElementStruct
    && type_decl.is_abstract
    && type_decl
      .xml_qname
      .as_deref()
      .is_some_and(|qname| qname.ends_with('/'))
    && type_decl.xml_content.is_none()
    && type_decl.members.is_empty()
    && !type_decl.support.have_xmlns_fields
    && !type_decl.support.has_xml_header
    && !type_decl.support.has_extra_support_fields()
}

fn empty_leaf_marker_doc_for_ref<'a>(
  module: &SchemaModuleDecl,
  type_ref: &TypeRefDecl,
  type_graph: &'a TypeContainmentGraph,
) -> Option<&'a str> {
  if let Some(target_key) = schema_type_key_from_ref(module, type_ref)
    && let Some(doc) = type_graph.empty_leaf_marker_doc(&target_key)
  {
    return Some(doc);
  }

  type_graph.empty_leaf_marker_doc_by_rust_name(&type_ref.rust_type)
}

fn is_empty_leaf_marker_ref_with_graph(
  module: &SchemaModuleDecl,
  type_ref: &TypeRefDecl,
  type_graph: &TypeContainmentGraph,
) -> bool {
  empty_leaf_marker_doc_for_ref(module, type_ref, type_graph).is_some()
}

fn meaningful_doc_text(doc: &str) -> Option<String> {
  let trimmed = doc.trim();
  if trimmed.is_empty() || trimmed == "_" {
    None
  } else {
    Some(format!(" {trimmed}"))
  }
}

fn choice_variant_doc_attrs(
  variant: &VariantDecl,
  module: &SchemaModuleDecl,
  type_graph: &TypeContainmentGraph,
  prefer_payload_docs: bool,
) -> TokenStream {
  let payload_doc_text = || {
    empty_leaf_marker_doc_for_ref(module, &variant.payload, type_graph)
      .and_then(meaningful_doc_text)
      .or_else(|| {
        type_decl_for_ref(module, &variant.payload)
          .and_then(|type_decl| meaningful_doc_text(&type_decl.docs))
      })
  };
  let doc_text = if variant.docs.is_empty() {
    None
  } else if prefer_payload_docs {
    payload_doc_text().or_else(|| meaningful_doc_text(&variant.docs))
  } else {
    meaningful_doc_text(&variant.docs).or_else(payload_doc_text)
  };

  if let Some(doc_text) = doc_text {
    quote! { #[doc = #doc_text] }
  } else {
    quote! {}
  }
}

fn empty_leaf_marker_type_names_to_omit(module: &SchemaModuleDecl) -> HashSet<&str> {
  module
    .types
    .iter()
    .filter(|type_decl| {
      is_empty_leaf_marker_type(type_decl) || is_abstract_empty_base_type(type_decl)
    })
    .map(|type_decl| type_decl.rust_name.as_str())
    .collect()
}

fn single_field_sequence_variant_can_inline(field: &FieldDecl) -> bool {
  matches!(field.cardinality, Cardinality::One | Cardinality::Optional)
    && matches!(
      field.wire,
      FieldWireDecl::Child { .. } | FieldWireDecl::TextChild { .. }
    )
}

fn inline_single_field_sequence_variant_tokens(
  module: &SchemaModuleDecl,
  type_graph: &TypeContainmentGraph,
  variant_ident: &Ident,
  variant_attrs: &[Attribute],
  field: &FieldDecl,
) -> Result<Option<TokenStream>> {
  if !single_field_sequence_variant_can_inline(field) {
    return Ok(None);
  }

  let payload_type = type_from_decl_ref(&field.type_ref, type_graph)?;
  let empty_leaf_marker_doc = empty_leaf_marker_doc_for_ref(module, &field.type_ref, type_graph);
  let empty_leaf_doc_attrs = if let Some(doc) = empty_leaf_marker_doc.and_then(meaningful_doc_text)
  {
    quote! { #[doc = #doc] }
  } else {
    quote! {}
  };

  match &field.wire {
    FieldWireDecl::Child { .. } => {
      if empty_leaf_marker_doc.is_some() {
        Ok(Some(quote! {
          #( #variant_attrs )*
          #empty_leaf_doc_attrs
          #variant_ident,
        }))
      } else {
        Ok(Some(quote! {
          #( #variant_attrs )*
          #variant_ident(std::boxed::Box<#payload_type>),
        }))
      }
    }
    FieldWireDecl::TextChild { .. } => Ok(Some(quote! {
      #( #variant_attrs )*
      #variant_ident(#payload_type),
    })),
    _ => Ok(None),
  }
}

enum SequenceVariantShape<'a> {
  SingleField {
    variant_ident: Ident,
    field: &'a FieldDecl,
  },
  BoxedSequence {
    variant_ident: Ident,
    helper_fields: Vec<&'a FieldDecl>,
  },
}

fn sequence_variant_shape<'a>(
  variant: &VariantDecl,
  rendered_variant_name: &str,
  module: &'a SchemaModuleDecl,
) -> Result<SequenceVariantShape<'a>> {
  let variant_ident: Ident =
    parse_str(&escape_upper_camel_case(rendered_variant_name.to_string()))?;
  let Some(helper_type_decl) = sequence_payload_helper_type_decl(variant, module) else {
    return Ok(SequenceVariantShape::BoxedSequence {
      variant_ident,
      helper_fields: Vec::new(),
    });
  };
  let helper_fields: Vec<&FieldDecl> = helper_type_decl
    .members
    .iter()
    .filter_map(|member| match member {
      MemberDecl::Field(field) => Some(field),
      _ => None,
    })
    .collect();

  if helper_fields.len() == 1 && single_field_sequence_variant_can_inline(helper_fields[0]) {
    return Ok(SequenceVariantShape::SingleField {
      variant_ident: single_field_sequence_variant_ident(rendered_variant_name, helper_fields[0])?,
      field: helper_fields[0],
    });
  }

  Ok(SequenceVariantShape::BoxedSequence {
    variant_ident,
    helper_fields,
  })
}

fn single_field_sequence_variant_ident(
  rendered_variant_name: &str,
  field: &FieldDecl,
) -> Result<Ident> {
  if rendered_variant_name == "Sequence"
    || is_generated_anonymous_variant_name_of_kind(rendered_variant_name, "Sequence")
  {
    match &field.wire {
      FieldWireDecl::Child { .. } | FieldWireDecl::TextChild { .. } => {
        return Ok(parse_str(&escape_upper_camel_case(
          schema_choice_variant_name_from_field_name(field.rust_name.as_str()),
        ))?);
      }
      _ => {}
    }
  }

  Ok(parse_str(&escape_upper_camel_case(
    rendered_variant_name.to_string(),
  ))?)
}

struct ChoiceVariantRenderContext<'a> {
  module: &'a SchemaModuleDecl,
  type_graph: &'a TypeContainmentGraph,
  variant_cfg: VersionCfgContext,
  rendered_variant_name_map: &'a HashMap<String, String>,
}

fn gen_choice_variant_tokens(
  variant: &VariantDecl,
  render_context: &ChoiceVariantRenderContext<'_>,
  prefix_attrs: TokenStream,
) -> Result<Vec<TokenStream>> {
  let rendered_variant_name = render_context
    .rendered_variant_name_map
    .get(&variant.rust_name)
    .map(String::as_str)
    .unwrap_or(&variant.rust_name);
  let variant_ident: Ident =
    parse_str(&escape_upper_camel_case(rendered_variant_name.to_string()))?;
  let variant_attrs = module_version_cfg_attrs(&variant.version, render_context.variant_cfg);
  let variant_doc_attrs = choice_variant_doc_attrs(
    variant,
    render_context.module,
    render_context.type_graph,
    false,
  );
  let empty_leaf_variant_doc_attrs = choice_variant_doc_attrs(
    variant,
    render_context.module,
    render_context.type_graph,
    true,
  );

  match &variant.wire {
    crate::sdk_code::codegen_ir::VariantWireDecl::Any { .. } => {
      let payload_type = type_from_decl_ref(&variant.payload, render_context.type_graph)?;
      Ok(vec![quote! {
        #prefix_attrs
        #( #variant_attrs )*
        #variant_doc_attrs
        #variant_ident(#payload_type),
      }])
    }
    crate::sdk_code::codegen_ir::VariantWireDecl::Sequence { .. } => {
      match sequence_variant_shape(variant, rendered_variant_name, render_context.module)? {
        SequenceVariantShape::SingleField {
          variant_ident,
          field,
        } => {
          let single_field_tokens = inline_single_field_sequence_variant_tokens(
            render_context.module,
            render_context.type_graph,
            &variant_ident,
            &variant_attrs,
            field,
          )?
          .expect("single-field sequence shape must render");
          Ok(vec![quote! {
            #prefix_attrs
            #single_field_tokens
          }])
        }
        SequenceVariantShape::BoxedSequence { variant_ident, .. } => {
          let payload_type = type_from_decl_ref(&variant.payload, render_context.type_graph)?;
          Ok(vec![quote! {
            #prefix_attrs
            #( #variant_attrs )*
            #variant_doc_attrs
            #variant_ident(std::boxed::Box<#payload_type>),
          }])
        }
      }
    }
    crate::sdk_code::codegen_ir::VariantWireDecl::TextChild { qnames } => {
      let payload_type = type_from_decl_ref(&variant.payload, render_context.type_graph)?;
      if qnames.is_empty() {
        return Err(variant.rust_name.clone().into());
      }
      Ok(vec![quote! {
        #prefix_attrs
        #( #variant_attrs )*
        #variant_doc_attrs
        #variant_ident(#payload_type),
      }])
    }
    crate::sdk_code::codegen_ir::VariantWireDecl::Child { qnames } => {
      let payload_type = type_from_decl_ref(&variant.payload, render_context.type_graph)?;
      if qnames.is_empty() {
        return Err(variant.rust_name.clone().into());
      }
      if is_empty_leaf_marker_ref_with_graph(
        render_context.module,
        &variant.payload,
        render_context.type_graph,
      ) {
        Ok(vec![quote! {
          #prefix_attrs
          #( #variant_attrs )*
          #empty_leaf_variant_doc_attrs
          #variant_ident,
        }])
      } else {
        let is_any_children_alias = is_any_children_alias_type_ref(
          render_context.module,
          &variant.payload,
          render_context.type_graph,
        );
        let payload_tokens = if is_any_children_alias
          || choice_child_variant_can_inline(
            &variant.payload,
            render_context.module,
            render_context.type_graph,
          ) {
          quote! { #payload_type }
        } else {
          quote! { std::boxed::Box<#payload_type> }
        };
        Ok(vec![quote! {
          #prefix_attrs
          #( #variant_attrs )*
          #variant_doc_attrs
          #variant_ident(#payload_tokens),
        }])
      }
    }
    crate::sdk_code::codegen_ir::VariantWireDecl::Text => {
      let payload_type = type_from_decl_ref(&variant.payload, render_context.type_graph)?;
      Ok(vec![quote! {
        #prefix_attrs
        #( #variant_attrs )*
        #variant_doc_attrs
        #variant_ident(#payload_type),
      }])
    }
  }
}

fn sequence_payload_helper_type_decl<'a>(
  variant: &VariantDecl,
  module: &'a SchemaModuleDecl,
) -> Option<&'a TypeDecl> {
  if !matches!(
    variant.wire,
    crate::sdk_code::codegen_ir::VariantWireDecl::Sequence { .. }
  ) {
    return None;
  }

  if variant.payload.module_path.is_some() {
    return None;
  }

  module.types.iter().find(|type_decl| {
    type_decl.rust_name == variant.payload.rust_type && type_decl.kind == TypeKind::HelperStruct
  })
}

fn choice_variant_rendered_names(
  variant: &VariantDecl,
  _module: &SchemaModuleDecl,
) -> Option<Vec<String>> {
  match &variant.wire {
    crate::sdk_code::codegen_ir::VariantWireDecl::Any { .. }
    | crate::sdk_code::codegen_ir::VariantWireDecl::Sequence { .. }
    | crate::sdk_code::codegen_ir::VariantWireDecl::Text
    | crate::sdk_code::codegen_ir::VariantWireDecl::TextChild { .. }
    | crate::sdk_code::codegen_ir::VariantWireDecl::Child { .. } => {
      Some(vec![variant.rust_name.clone()])
    }
  }
}

fn is_generated_anonymous_variant_name_of_kind(name: &str, kind: &str) -> bool {
  name
    .strip_prefix(kind)
    .is_some_and(|suffix| !suffix.is_empty() && suffix.chars().all(|ch| ch.is_ascii_digit()))
}

fn anonymous_variant_render_name_map(
  type_decl: &TypeDecl,
  module: &SchemaModuleDecl,
) -> HashMap<String, String> {
  let Some(rendered_names) = choice_enum_rendered_variant_names(type_decl, module) else {
    return HashMap::new();
  };

  let non_anonymous_names: HashSet<String> = rendered_names
    .iter()
    .filter(|name| !is_generated_anonymous_type_name(name))
    .cloned()
    .collect();
  let mut result = HashMap::new();

  for kind in ["Choice", "Sequence"] {
    let anonymous_names: Vec<String> = rendered_names
      .iter()
      .filter(|name| is_generated_anonymous_variant_name_of_kind(name, kind))
      .cloned()
      .collect();

    if anonymous_names.is_empty() {
      continue;
    }

    let normalized_names: Vec<String> = if anonymous_names.len() == 1 {
      vec![kind.to_string()]
    } else {
      (1..=anonymous_names.len())
        .map(|index| format!("{kind}{index}"))
        .collect()
    };

    let mut occupied_names = non_anonymous_names.clone();
    let mut can_normalize = true;
    for normalized_name in &normalized_names {
      if !occupied_names.insert(normalized_name.clone()) {
        can_normalize = false;
        break;
      }
    }

    if can_normalize {
      for (raw_name, normalized_name) in anonymous_names.into_iter().zip(normalized_names) {
        result.insert(raw_name, normalized_name);
      }
    }
  }

  result
}

fn choice_enum_rendered_variant_names(
  type_decl: &TypeDecl,
  module: &SchemaModuleDecl,
) -> Option<Vec<String>> {
  fn collect(
    type_decl: &TypeDecl,
    module: &SchemaModuleDecl,
    visiting: &mut HashSet<String>,
    names: &mut Vec<String>,
  ) -> Option<()> {
    if !visiting.insert(type_decl.rust_name.clone()) {
      return None;
    }

    for member in &type_decl.members {
      let MemberDecl::Variant(variant) = member else {
        visiting.remove(&type_decl.rust_name);
        return None;
      };
      if let Some(rendered_variant_names) = choice_variant_rendered_names(variant, module) {
        names.extend(rendered_variant_names);
      } else {
        visiting.remove(&type_decl.rust_name);
        return None;
      }
    }

    visiting.remove(&type_decl.rust_name);
    Some(())
  }

  let mut visiting = HashSet::new();
  let mut names = Vec::new();
  collect(type_decl, module, &mut visiting, &mut names)?;
  Some(names)
}

fn is_generated_anonymous_type_name(name: &str) -> bool {
  let Some(last_non_digit_idx) = name.rfind(|ch: char| !ch.is_ascii_digit()) else {
    return false;
  };

  last_non_digit_idx + 1 < name.len()
    && (name[..=last_non_digit_idx].ends_with("Choice")
      || name[..=last_non_digit_idx].ends_with("Sequence"))
}

fn prune_unreferenced_anonymous_emitted_types(
  non_helper_tokens: &[TokenStream],
  helper_tokens: Vec<(String, bool, bool, TokenStream)>,
) -> Vec<(String, bool, bool, TokenStream)> {
  let mut kept = helper_tokens;

  loop {
    let rendered_non_helper: Vec<String> =
      non_helper_tokens.iter().map(ToString::to_string).collect();
    let rendered_helper: Vec<String> = kept
      .iter()
      .map(|(_, _, _, tokens)| tokens.to_string())
      .collect();
    let before_len = kept.len();

    kept.retain(|(rust_name, is_anonymous, prune_if_unreferenced, _)| {
      if !*is_anonymous && !*prune_if_unreferenced {
        return true;
      }

      let total_refs = rendered_non_helper
        .iter()
        .chain(rendered_helper.iter())
        .map(|rendered| count_ident_occurrences(rendered, rust_name))
        .sum::<usize>();

      total_refs > 1
    });

    if kept.len() == before_len {
      break;
    }
  }

  kept
}

fn anonymous_emitted_type_rename_map(
  non_helper_type_names: &HashSet<String>,
  helper_tokens: &[(String, bool, bool, TokenStream)],
) -> HashMap<String, String> {
  let helper_names: HashSet<String> = helper_tokens
    .iter()
    .map(|(rust_name, _, _, _)| rust_name.clone())
    .collect();
  let mut groups: HashMap<String, Vec<String>> = HashMap::new();

  for (rust_name, is_anonymous, _, _) in helper_tokens {
    if !*is_anonymous {
      continue;
    }

    let Some(base_name) = anonymous_type_base_name(rust_name) else {
      continue;
    };
    groups
      .entry(base_name.to_string())
      .or_default()
      .push(rust_name.clone());
  }

  for (base_name, raw_names) in &mut groups {
    if helper_names.contains(base_name) && !raw_names.iter().any(|name| name == base_name) {
      raw_names.insert(0, base_name.clone());
    }
  }

  let mut rename_map = HashMap::new();

  for (base_name, raw_names) in groups {
    let normalized_names: Vec<String> = if raw_names.len() == 1 {
      vec![base_name.clone()]
    } else {
      std::iter::once(base_name.clone())
        .chain((2..=raw_names.len()).map(|index| format!("{base_name}{index}")))
        .collect()
    };

    let mut occupied_names = non_helper_type_names.clone();
    occupied_names.extend(
      helper_names
        .iter()
        .filter(|name| !raw_names.contains(*name))
        .cloned(),
    );

    if normalized_names
      .iter()
      .any(|normalized_name| occupied_names.contains(normalized_name))
    {
      continue;
    }

    for (raw_name, normalized_name) in raw_names.into_iter().zip(normalized_names) {
      if raw_name != normalized_name {
        rename_map.insert(raw_name, normalized_name);
      }
    }
  }

  rename_map
}

fn anonymous_type_base_name(name: &str) -> Option<&str> {
  let last_non_digit_idx = name.rfind(|ch: char| !ch.is_ascii_digit())?;
  if last_non_digit_idx + 1 >= name.len() {
    return None;
  }

  let base = &name[..=last_non_digit_idx];
  (base.ends_with("Choice") || base.ends_with("Sequence")).then_some(base)
}

fn rename_token_stream_idents(
  tokens: TokenStream,
  rename_map: &HashMap<String, String>,
) -> TokenStream {
  tokens
    .into_iter()
    .map(|tree| match tree {
      TokenTree::Group(group) => {
        let mut new_group = Group::new(
          group.delimiter(),
          rename_token_stream_idents(group.stream(), rename_map),
        );
        new_group.set_span(group.span());
        TokenTree::Group(new_group)
      }
      TokenTree::Ident(ident) => rename_map
        .get(&ident.to_string())
        .map(|renamed| TokenTree::Ident(TokenIdent::new(renamed, ident.span())))
        .unwrap_or(TokenTree::Ident(ident)),
      other => other,
    })
    .collect()
}

fn count_ident_occurrences(haystack: &str, needle: &str) -> usize {
  if needle.is_empty() {
    return 0;
  }

  let haystack_bytes = haystack.as_bytes();
  let needle_bytes = needle.as_bytes();
  let mut count = 0;
  let mut start = 0;

  while let Some(offset) = haystack[start..].find(needle) {
    let absolute = start + offset;
    let before = absolute
      .checked_sub(1)
      .and_then(|idx| haystack_bytes.get(idx))
      .copied();
    let after = haystack_bytes.get(absolute + needle_bytes.len()).copied();

    if !before.is_some_and(is_rust_ident_char) && !after.is_some_and(is_rust_ident_char) {
      count += 1;
    }

    start = absolute + needle_bytes.len();
  }

  count
}

fn is_rust_ident_char(ch: u8) -> bool {
  ch.is_ascii_alphanumeric() || ch == b'_'
}

fn gen_helper_struct_type_decl(
  type_decl: &TypeDecl,
  module: &SchemaModuleDecl,
  type_graph: &TypeContainmentGraph,
  version_cfg: VersionCfgContext,
) -> Result<TokenStream> {
  let struct_ident: Ident = parse_str(&type_decl.rust_name.to_upper_camel_case())?;
  let type_version = type_decl.version.as_deref().unwrap_or_default();
  let type_attrs = version_cfg.attrs(type_version);
  let nested_cfg = if type_attrs.is_empty() {
    version_cfg
  } else {
    VersionCfgContext::new(true)
  };
  let helper_fields: Vec<&FieldDecl> = type_decl
    .members
    .iter()
    .filter_map(|member| match member {
      MemberDecl::Field(field)
        if matches!(
          field.wire,
          FieldWireDecl::Child { .. } | FieldWireDecl::TextChild { .. } | FieldWireDecl::Any
        ) =>
      {
        Some(field)
      }
      _ => None,
    })
    .collect();
  let fields = gen_flatten_one_sequence_fields_from_decl(
    &helper_fields,
    &type_decl.rust_name,
    module,
    type_graph,
    nested_cfg,
    &HashSet::new(),
  )?;
  let sdk_type_derive = sdk_type_derive_tokens();
  Ok(quote! {
    #( #type_attrs )*
    #sdk_type_derive
    pub struct #struct_ident {
      #( #fields )*
    }
  })
}

fn gen_schema_enum_decl_variants(
  facets: &[&crate::sdk_code::codegen_ir::EnumVariantDecl],
  default_facet: &crate::sdk_code::codegen_ir::EnumVariantDecl,
  alias_map: &HashMap<String, Vec<String>>,
  version_cfg: VersionCfgContext,
) -> Result<Vec<Variant>> {
  let mut variants = Vec::with_capacity(facets.len());

  for facet in facets {
    let variant_ident: Ident = if facet.rust_name.is_empty() {
      parse_str(&escape_upper_camel_case(
        facet.xml_value.to_upper_camel_case(),
      ))?
    } else {
      parse_str(&escape_upper_camel_case(
        facet.rust_name.to_upper_camel_case(),
      ))?
    };
    let variant_attrs = version_cfg.attrs(&facet.version);
    let rename_attrs = if facet.xml_value.is_empty() {
      quote! {}
    } else {
      let value = &facet.xml_value;
      let aliases = alias_map.get(value);
      if let Some(aliases) = aliases {
        let alias_attrs = aliases.iter().map(|alias| quote! { alias = #alias });
        quote! {
          #[sdk(rename = #value, #(#alias_attrs),*)]
        }
      } else {
        quote! {
          #[sdk(rename = #value)]
        }
      }
    };
    let default_attr = if std::ptr::eq(*facet, default_facet) {
      quote! { #[default] }
    } else {
      quote! {}
    };

    variants.push(parse2(quote! {
      #( #variant_attrs )*
      #rename_attrs
      #default_attr
      #variant_ident
    })?);
  }

  Ok(variants)
}

fn module_version_cfg_attrs(version: &str, version_cfg: VersionCfgContext) -> Vec<Attribute> {
  version_cfg.attrs(version)
}

fn schema_item_version(schema_type: &SchemaType) -> &str {
  schema_type.version.as_deref().unwrap_or_default()
}

fn number_range_bound_tokens(value: &str, use_literal: bool) -> TokenStream {
  if use_literal
    && is_numeric_literal(value)
    && let Ok(value_tokens) = value.parse::<TokenStream>()
  {
    value_tokens
  } else {
    quote! { #value }
  }
}

fn number_range_attr(
  min: Option<&String>,
  max: Option<&String>,
  min_inclusive: bool,
  max_inclusive: bool,
) -> Option<TokenStream> {
  if !min_inclusive {
    return None;
  }
  let min = min.map(|min| number_range_bound_tokens(min, true));
  let max = max.map(|max| number_range_bound_tokens(max, true));
  Some(match (min, max, max_inclusive) {
    (Some(min), Some(max), true) => quote! { range = #min..=#max, },
    (Some(min), Some(max), false) => quote! { range = #min..#max, },
    (Some(min), None, _) => quote! { range = #min.., },
    (None, Some(max), true) => quote! { range = ..=#max, },
    (None, Some(max), false) => quote! { range = ..#max, },
    (None, None, _) => quote! {},
  })
}

fn number_range_limit_attr(name: &str, value: &str, use_literal: bool) -> TokenStream {
  let name_ident = TokenIdent::new(name, Span::call_site());
  if use_literal
    && is_numeric_literal(value)
    && let Ok(value_tokens) = value.parse::<TokenStream>()
  {
    quote! { #name_ident = #value_tokens, }
  } else {
    quote! { #name_ident = #value, }
  }
}

fn is_numeric_literal(value: &str) -> bool {
  let value = value.strip_prefix('-').unwrap_or(value);
  let mut parts = value.split('.');
  let Some(whole) = parts.next() else {
    return false;
  };
  if whole.is_empty() || !whole.bytes().all(|byte| byte.is_ascii_digit()) {
    return false;
  }
  match (parts.next(), parts.next()) {
    (None, None) => true,
    (Some(fraction), None) => {
      !fraction.is_empty() && fraction.bytes().all(|byte| byte.is_ascii_digit())
    }
    _ => false,
  }
}

fn gen_attr_from_decl(
  attr: &FieldDecl,
  version_cfg: VersionCfgContext,
  type_graph: &TypeContainmentGraph,
) -> Result<TokenStream> {
  let FieldWireDecl::Attribute {
    qname,
    bit,
    list,
    match_local_name,
    empty_as_none,
  } = &attr.wire
  else {
    return Err(format!("expected attribute field, got {:?}", attr.wire).into());
  };
  let attr_name_ident: Ident = parse_str(&attr.rust_name)?;
  let type_ident = type_from_decl_ref(&attr.type_ref, type_graph)?;
  let bit_attrs = if let Some(bit) = bit {
    quote! {
      #[sdk(bit = #bit)]
    }
  } else {
    quote! {}
  };
  let attr_attrs = module_version_cfg_attrs(&attr.version, version_cfg);
  let attr_sdk_version_markers = sdk_version_markers(&attr.version);
  let list_attr = list.then_some(quote! { list, });
  let match_local_name_attr = match_local_name.then_some(quote! { match_local_name, });
  let empty_as_none_attr = empty_as_none.then_some(quote! { empty_as_none, });
  let sdk_attr_attrs = quote! {
    #[sdk(attr(#(#attr_sdk_version_markers,)* #list_attr #match_local_name_attr #empty_as_none_attr qname = #qname))]
  };
  let validator_attrs: Vec<TokenStream> = attr
    .validators
    .iter()
    .map(|validator| {
      let source_attr = validator
        .union_id
        .map(|_| {
          let source_id = validator.source_id;
          quote! { source = #source_id, }
        })
        .unwrap_or_default();
      let union_attr = validator
        .union_id
        .map(|union_id| quote! { union = #union_id, });
      match &validator.kind {
        ValidatorKind::Pattern { regex } => quote! {
          #[sdk(pattern(#source_attr #union_attr regex = #regex))]
        },
        ValidatorKind::StringFormat { kind } => {
          let kind_lit = match kind {
            crate::sdk_code::codegen_ir::StringFormatKind::Token => "token",
            crate::sdk_code::codegen_ir::StringFormatKind::NcName => "ncname",
            crate::sdk_code::codegen_ir::StringFormatKind::QName => "qname",
            crate::sdk_code::codegen_ir::StringFormatKind::Uri => "uri",
            crate::sdk_code::codegen_ir::StringFormatKind::Id => "id",
          };
          quote! {
            #[sdk(string_format(#source_attr #union_attr kind = #kind_lit))]
          }
        }
        ValidatorKind::StringLength {
          min,
          max,
          exact,
          type_name,
        } => {
          let effective_min = exact.or(*min);
          let effective_max = exact.or(*max);
          let min_attr = effective_min.map(|min| quote! { min = #min, });
          let max_attr = effective_max.map(|max| quote! { max = #max, });
          let type_name_attr = type_name
            .as_ref()
            .map(|type_name| quote! { type_name = #type_name, });
          quote! {
            #[sdk(string_length(#source_attr #union_attr #type_name_attr #min_attr #max_attr))]
          }
        }
        ValidatorKind::NumberRange {
          min,
          max,
          min_inclusive,
          max_inclusive,
        } => {
          let use_range_attr = validator.union_id.is_none();
          let range_attr = use_range_attr
            .then(|| number_range_attr(min.as_ref(), max.as_ref(), *min_inclusive, *max_inclusive))
            .flatten();
          let use_limit_literal_bounds = validator.union_id.is_none();
          let min_attr = if range_attr.is_some() {
            None
          } else {
            min
              .as_ref()
              .map(|min| number_range_limit_attr("min", min, use_limit_literal_bounds))
          };
          let max_attr = if range_attr.is_some() {
            None
          } else {
            max
              .as_ref()
              .map(|max| number_range_limit_attr("max", max, use_limit_literal_bounds))
          };
          let min_inclusive_attr =
            if range_attr.is_some() || (validator.union_id.is_none() && *min_inclusive) {
              quote! {}
            } else {
              quote! { min_inclusive = #min_inclusive, }
            };
          let max_inclusive_attr =
            if range_attr.is_some() || (validator.union_id.is_none() && *max_inclusive) {
              quote! {}
            } else {
              quote! { max_inclusive = #max_inclusive, }
            };
          quote! {
          #[sdk(number_range(
            #source_attr
            #union_attr
            #range_attr
            #min_attr
            #max_attr
            #min_inclusive_attr
              #max_inclusive_attr
            ))]
          }
        }
        ValidatorKind::NumberType { type_name } => quote! {
          #[sdk(number_type(#source_attr #union_attr type_name = #type_name))]
        },
        ValidatorKind::NumberSign { kind } => {
          let kind_lit = match kind {
            crate::sdk_code::codegen_ir::NumberSignKind::NonNegative => "non_negative",
            crate::sdk_code::codegen_ir::NumberSignKind::Positive => "positive",
          };
          quote! {
            #[sdk(number_sign(#source_attr #union_attr kind = #kind_lit))]
          }
        }
        ValidatorKind::StringSet { values } => quote! {
          #[sdk(string_set(#source_attr #union_attr values = &[ #( #values ),* ]))]
        },
        ValidatorKind::Required
        | ValidatorKind::EnumRef { .. }
        | ValidatorKind::Unsupported { .. }
        | ValidatorKind::Placeholder => quote! {},
      }
    })
    .collect();
  let property_comments_doc = if attr.docs.trim().is_empty() {
    quote! {}
  } else {
    let docs = format!(" {}", attr.docs);
    quote! { #[doc = #docs] }
  };

  Ok(match attr.cardinality {
    Cardinality::One => quote! {
      #( #attr_attrs )*
      #property_comments_doc
      #sdk_attr_attrs
      #( #validator_attrs )*
      #bit_attrs
      pub #attr_name_ident: #type_ident,
    },
    Cardinality::Optional => quote! {
      #( #attr_attrs )*
      #property_comments_doc
      #sdk_attr_attrs
      #( #validator_attrs )*
      #bit_attrs
      pub #attr_name_ident: Option<#type_ident>,
    },
    Cardinality::Many => {
      return Err(format!("attribute {} cannot have many cardinality", attr.rust_name).into());
    }
  })
}

fn type_from_decl_ref(type_ref: &TypeRefDecl, type_graph: &TypeContainmentGraph) -> Result<Type> {
  if type_ref.rust_type.contains('<') || type_ref.rust_type.contains("::") {
    if let Some(module_path) = &type_ref.module_path {
      let module_path = type_graph.rendered_module_path(module_path);
      return Ok(parse_str(&format!(
        "{module_path}::{}",
        type_ref.rust_type
      ))?);
    }

    return Ok(parse_str(&type_ref.rust_type)?);
  }

  if let Some(module_path) = &type_ref.module_path {
    let module_path = type_graph.rendered_module_path(module_path);
    Ok(parse_str(&format!(
      "{module_path}::{}",
      type_ref.rust_type.to_upper_camel_case()
    ))?)
  } else {
    Ok(parse_str(&type_ref.rust_type.to_upper_camel_case())?)
  }
}

fn is_value_like_type_ref(
  module: &SchemaModuleDecl,
  type_ref: &TypeRefDecl,
  type_graph: &TypeContainmentGraph,
) -> bool {
  if matches!(type_ref.module_path.as_deref(), Some("crate::simple_type")) {
    return true;
  }

  if module
    .enums
    .iter()
    .any(|schema_enum| schema_enum.rust_name == type_ref.rust_type)
  {
    return true;
  }

  let Some(type_key) = schema_type_key_from_ref(module, type_ref) else {
    return false;
  };

  type_graph.is_leaf_text_alias(&type_key)
}

fn is_sdk_enum_type_ref(
  module: &SchemaModuleDecl,
  type_ref: &TypeRefDecl,
  type_graph: &TypeContainmentGraph,
) -> bool {
  schema_type_key_from_ref(module, type_ref).is_some_and(|type_key| type_graph.is_enum(&type_key))
}

fn text_child_value_metadata_attrs_from_type_ref(
  module: &SchemaModuleDecl,
  type_ref: &TypeRefDecl,
  type_graph: &TypeContainmentGraph,
) -> TokenStream {
  if is_sdk_enum_type_ref(module, type_ref, type_graph) {
    return quote! { enum, };
  }

  let Some(type_key) = schema_type_key_from_ref(module, type_ref) else {
    return quote! {};
  };
  let Some(xml_content) = type_graph.leaf_text_alias_content(&type_key) else {
    return quote! {};
  };

  text_child_value_metadata_attrs_from_type_ref(module, xml_content, type_graph)
}

fn is_list_type_ref(type_ref: &TypeRefDecl) -> bool {
  type_ref.module_path.is_none() && type_ref.rust_type.starts_with("Vec<")
}

fn sdk_element_qname(qname: &str) -> String {
  schema_qname_element_name(qname).to_string()
}

fn module_type_namespace(module_name: &str) -> String {
  format!("crate::schemas::{module_name}")
}

fn schema_prefix_alias_path(module: &SchemaModuleDecl) -> Option<String> {
  let prefix = module.prefix.trim();
  if prefix.is_empty() {
    return None;
  }

  Some(format!(
    "crate::schemas::{}",
    escape_snake_case(prefix.to_snake_case())
  ))
}

fn local_type_key(module: &SchemaModuleDecl, rust_type: &str) -> String {
  format!(
    "{}::{rust_type}",
    module_type_namespace(&module.module_name),
    rust_type = rust_type.to_upper_camel_case()
  )
}

fn schema_type_key_from_ref(module: &SchemaModuleDecl, type_ref: &TypeRefDecl) -> Option<String> {
  let module_path = match type_ref.module_path.as_deref() {
    Some("crate::simple_type") => return None,
    Some(module_path) => module_path.to_string(),
    None => module_type_namespace(&module.module_name),
  };

  Some(format!(
    "{module_path}::{}",
    type_ref.rust_type.to_upper_camel_case()
  ))
}

#[derive(Clone, Copy)]
pub(crate) struct MceProcessContentPromotion<'a> {
  outer_module: &'a SchemaModuleDecl,
  outer_choice: &'a TypeDecl,
  wrapper_variant: &'a VariantDecl,
  wrapper_module: &'a SchemaModuleDecl,
  wrapper_type: &'a TypeDecl,
}

fn type_decl_lookup<'a>(
  modules: &[&'a SchemaModuleDecl],
) -> HashMap<String, (&'a SchemaModuleDecl, &'a TypeDecl)> {
  let mut lookup = HashMap::new();
  for module in modules {
    for ty in &module.types {
      lookup.insert(local_type_key(module, &ty.rust_name), (*module, ty));
      if let Some(alias) = schema_prefix_alias_path(module) {
        lookup.insert(
          format!("{alias}::{}", ty.rust_name.to_upper_camel_case()),
          (*module, ty),
        );
      }
    }
  }
  lookup
}

fn wrapper_promotion_payloads_match(
  outer_module: &SchemaModuleDecl,
  outer_choice: &TypeDecl,
  wrapper_module: &SchemaModuleDecl,
  wrapper_type: &TypeDecl,
  lookup: &HashMap<String, (&SchemaModuleDecl, &TypeDecl)>,
) -> bool {
  let mut matched = false;
  for member in &wrapper_type.members {
    let MemberDecl::Field(field) = member else {
      continue;
    };
    match &field.wire {
      FieldWireDecl::Child { qname } | FieldWireDecl::TextChild { qname } => {
        let qname = schema_qname_element_name(qname);
        let Some(outer_variant) = promotion_outer_variant_by_qname(outer_choice, qname) else {
          return false;
        };
        if schema_type_key_from_ref(wrapper_module, &field.type_ref)
          != schema_type_key_from_ref(outer_module, &outer_variant.payload)
        {
          return false;
        }
        matched = true;
      }
      FieldWireDecl::Choice => {
        let Some(inner_key) = schema_type_key_from_ref(wrapper_module, &field.type_ref) else {
          return false;
        };
        let Some((inner_module, inner_choice)) = lookup.get(&inner_key).copied() else {
          return false;
        };
        for inner_member in &inner_choice.members {
          let MemberDecl::Variant(inner_variant) = inner_member else {
            continue;
          };
          let VariantWireDecl::Child { qnames } = &inner_variant.wire else {
            continue;
          };
          let Some(qname) = qnames.first().map(|qname| schema_qname_element_name(qname)) else {
            continue;
          };
          let Some(outer_variant) = promotion_outer_variant_by_qname(outer_choice, qname) else {
            return false;
          };
          if schema_type_key_from_ref(inner_module, &inner_variant.payload)
            != schema_type_key_from_ref(outer_module, &outer_variant.payload)
          {
            return false;
          }
          matched = true;
        }
      }
      FieldWireDecl::Attribute { .. } | FieldWireDecl::Text | FieldWireDecl::Any => {}
    }
  }
  matched
}

pub(crate) fn collect_mce_process_content_promotions<'a>(
  modules: &[&'a SchemaModuleDecl],
) -> Vec<MceProcessContentPromotion<'a>> {
  let lookup = type_decl_lookup(modules);
  let repeated_choice_key = |module: &SchemaModuleDecl, field: &FieldDecl| {
    (field.wire == FieldWireDecl::Choice && field.cardinality == Cardinality::Many)
      .then(|| schema_type_key_from_ref(module, &field.type_ref))
      .flatten()
  };
  let repeated_choice_keys = modules
    .iter()
    .flat_map(|module| {
      module.types.iter().flat_map(|ty| {
        ty.members.iter().filter_map(|member| {
          let MemberDecl::Field(field) = member else {
            return None;
          };
          repeated_choice_key(module, field)
        })
      })
    })
    .collect::<HashSet<_>>();
  let mut candidates = Vec::new();
  for outer_module in modules {
    for outer_choice in &outer_module.types {
      if outer_choice.kind != TypeKind::ChoiceEnum {
        continue;
      }
      if !repeated_choice_keys.contains(&local_type_key(outer_module, &outer_choice.rust_name)) {
        continue;
      }
      for member in &outer_choice.members {
        let MemberDecl::Variant(wrapper_variant) = member else {
          continue;
        };
        if !crate::sdk_code::versioning::is_post_office2007_version(&wrapper_variant.version)
          || !matches!(wrapper_variant.wire, VariantWireDecl::Child { .. })
        {
          continue;
        }
        let Some(wrapper_key) = schema_type_key_from_ref(outer_module, &wrapper_variant.payload)
        else {
          continue;
        };
        let Some((wrapper_module, wrapper_type)) = lookup.get(&wrapper_key).copied() else {
          continue;
        };
        if wrapper_promotion_payloads_match(
          outer_module,
          outer_choice,
          wrapper_module,
          wrapper_type,
          &lookup,
        ) {
          candidates.push(MceProcessContentPromotion {
            outer_module,
            outer_choice,
            wrapper_variant,
            wrapper_module,
            wrapper_type,
          });
        }
      }
    }
  }

  let mut eligible_choice_keys = modules
    .iter()
    .flat_map(|module| {
      module
        .types
        .iter()
        .filter(|ty| {
          ty.kind == TypeKind::ElementStruct
            && !ty.is_abstract
            && schema_version_rank(ty.version.as_deref().unwrap_or("")) == 0
            && ty.xml_qname.as_deref().is_some_and(|qname| {
              qname
                .rsplit('/')
                .next()
                .is_some_and(|element| element.contains(':') && !element.ends_with(':'))
            })
        })
        .flat_map(|ty| {
          ty.members.iter().filter_map(|member| {
            let MemberDecl::Field(field) = member else {
              return None;
            };
            repeated_choice_key(module, field)
          })
        })
    })
    .collect::<HashSet<_>>();
  let mut selected = vec![false; candidates.len()];
  loop {
    let mut changed = false;
    for (index, promotion) in candidates.iter().enumerate() {
      if selected[index]
        || !eligible_choice_keys.contains(&local_type_key(
          promotion.outer_module,
          &promotion.outer_choice.rust_name,
        ))
      {
        continue;
      }
      selected[index] = true;
      changed = true;
      for member in &promotion.wrapper_type.members {
        let MemberDecl::Field(field) = member else {
          continue;
        };
        if let Some(choice_key) = repeated_choice_key(promotion.wrapper_module, field) {
          eligible_choice_keys.insert(choice_key);
        }
      }
    }
    if !changed {
      break;
    }
  }

  candidates
    .into_iter()
    .zip(selected)
    .filter_map(|(promotion, selected)| selected.then_some(promotion))
    .collect()
}

fn promotion_outer_variant_by_qname<'a>(
  outer_choice: &'a TypeDecl,
  qname: &str,
) -> Option<&'a VariantDecl> {
  outer_choice.members.iter().find_map(|member| {
    let MemberDecl::Variant(variant) = member else {
      return None;
    };
    let VariantWireDecl::Child { qnames } = &variant.wire else {
      return None;
    };
    qnames
      .iter()
      .any(|candidate| schema_qname_element_name(candidate) == qname)
      .then_some(variant)
  })
}

fn promotion_variant_ident(
  variant: &VariantDecl,
  owner: &TypeDecl,
  module: &SchemaModuleDecl,
) -> Result<Ident> {
  let rendered = anonymous_variant_render_name_map(owner, module)
    .get(&variant.rust_name)
    .cloned()
    .unwrap_or_else(|| variant.rust_name.clone());
  Ok(parse_str(&escape_upper_camel_case(rendered))?)
}

fn promotion_payload_shape_matches(
  source_module: &SchemaModuleDecl,
  source: &TypeRefDecl,
  target_module: &SchemaModuleDecl,
  target: &TypeRefDecl,
  type_graph: &TypeContainmentGraph,
) -> bool {
  schema_type_key_from_ref(source_module, source) == schema_type_key_from_ref(target_module, target)
    && choice_child_variant_can_inline(source, source_module, type_graph)
      == choice_child_variant_can_inline(target, target_module, type_graph)
}

fn mce_mapping_cardinality_tokens(cardinality: Cardinality) -> TokenStream {
  match cardinality {
    Cardinality::One => quote! { Cardinality::One },
    Cardinality::Optional => quote! { Cardinality::Optional },
    Cardinality::Many => quote! { Cardinality::Many },
  }
}

fn gen_promoted_choice_field_mapping(
  promotion: MceProcessContentPromotion<'_>,
  field: &FieldDecl,
  lookup: &HashMap<String, (&SchemaModuleDecl, &TypeDecl)>,
  type_graph: &TypeContainmentGraph,
) -> Result<Option<TokenStream>> {
  let Some(inner_key) = schema_type_key_from_ref(promotion.wrapper_module, &field.type_ref) else {
    return Ok(None);
  };
  let Some((inner_module, inner_choice)) = lookup.get(&inner_key).copied() else {
    return Ok(None);
  };
  let mut variants = Vec::new();
  let mut unit_variants = Vec::new();
  let mut renamed_variants = Vec::new();
  for member in &inner_choice.members {
    let MemberDecl::Variant(inner_variant) = member else {
      continue;
    };
    let VariantWireDecl::Child { qnames } = &inner_variant.wire else {
      continue;
    };
    let Some(qname) = qnames.first().map(|qname| schema_qname_element_name(qname)) else {
      continue;
    };
    let outer_variant = promotion_outer_variant_by_qname(promotion.outer_choice, qname)
      .ok_or_else(|| format!("missing ProcessContent target for {qname}"))?;
    if !promotion_payload_shape_matches(
      inner_module,
      &inner_variant.payload,
      promotion.outer_module,
      &outer_variant.payload,
      type_graph,
    ) {
      return Err(
        format!(
          "ProcessContent payload mismatch for {} {} -> {}",
          promotion.outer_choice.rust_name, inner_variant.rust_name, outer_variant.rust_name
        )
        .into(),
      );
    }
    let inner_variant_ident = promotion_variant_ident(inner_variant, inner_choice, inner_module)?;
    let outer_variant_ident = promotion_variant_ident(
      outer_variant,
      promotion.outer_choice,
      promotion.outer_module,
    )?;
    let unit =
      is_empty_leaf_marker_ref_with_graph(inner_module, &inner_variant.payload, type_graph);
    let inner_variant = inner_variant_ident.to_string();
    let outer_variant = outer_variant_ident.to_string();
    if unit {
      unit_variants.push(inner_variant.clone());
    }
    if inner_variant != outer_variant {
      renamed_variants.push(quote! { (#inner_variant, #outer_variant) });
    }
    variants.push(inner_variant);
  }
  let field_name = field.rust_name.to_snake_case();
  let source_module = inner_module.module_name.as_str();
  let source_choice = inner_choice.rust_name.to_upper_camel_case();
  let cardinality = mce_mapping_cardinality_tokens(field.cardinality);
  let exhaustive = inner_choice.members.iter().all(|member| {
    matches!(
      member,
      MemberDecl::Variant(VariantDecl {
        wire: VariantWireDecl::Child { .. },
        ..
      })
    )
  });
  Ok(Some(quote! {
    FieldMapping::Choice {
      field: #field_name,
      source_module: #source_module,
      source_choice: #source_choice,
      cardinality: #cardinality,
      exhaustive: #exhaustive,
      variants: &[ #( #variants, )* ],
      unit_variants: &[ #( #unit_variants, )* ],
      renamed_variants: &[ #( #renamed_variants, )* ],
    }
  }))
}

fn gen_promoted_direct_field_mapping(
  promotion: MceProcessContentPromotion<'_>,
  field: &FieldDecl,
  type_graph: &TypeContainmentGraph,
) -> Result<Option<TokenStream>> {
  let qname = match &field.wire {
    FieldWireDecl::Child { qname } | FieldWireDecl::TextChild { qname } => {
      schema_qname_element_name(qname)
    }
    _ => return Ok(None),
  };
  let outer_variant = promotion_outer_variant_by_qname(promotion.outer_choice, qname)
    .ok_or_else(|| format!("missing ProcessContent target for {qname}"))?;
  if schema_type_key_from_ref(promotion.wrapper_module, &field.type_ref)
    != schema_type_key_from_ref(promotion.outer_module, &outer_variant.payload)
  {
    return Err(
      format!(
        "ProcessContent direct payload mismatch for {} {} -> {}",
        promotion.outer_choice.rust_name, field.rust_name, outer_variant.rust_name
      )
      .into(),
    );
  }
  let field_ident = Ident::new(&field.rust_name.to_snake_case(), Span::call_site());
  let outer_variant_ident = promotion_variant_ident(
    outer_variant,
    promotion.outer_choice,
    promotion.outer_module,
  )?;
  let field_is_boxed = direct_child_field_needs_box(
    &promotion.wrapper_type.rust_name,
    field,
    promotion.wrapper_module,
    type_graph,
    false,
  );
  let outer_is_boxed =
    !choice_child_variant_can_inline(&outer_variant.payload, promotion.outer_module, type_graph);
  if field_is_boxed != outer_is_boxed {
    return Err(
      format!(
        "ProcessContent boxing mismatch for {} {}",
        promotion.outer_choice.rust_name, field.rust_name
      )
      .into(),
    );
  }
  let field_name = field_ident.to_string();
  let target_variant = outer_variant_ident.to_string();
  let cardinality = mce_mapping_cardinality_tokens(field.cardinality);
  Ok(Some(quote! {
    FieldMapping::Direct {
      field: #field_name,
      target_variant: #target_variant,
      cardinality: #cardinality,
    }
  }))
}

fn gen_mce_process_content_mapping_entry(
  entries: &[MceProcessContentPromotion<'_>],
  lookup: &HashMap<String, (&SchemaModuleDecl, &TypeDecl)>,
  type_graph: &TypeContainmentGraph,
) -> Result<TokenStream> {
  let mut wrappers = Vec::new();
  for promotion in entries {
    let promotion = *promotion;
    let wrapper_variant_ident = promotion_variant_ident(
      promotion.wrapper_variant,
      promotion.outer_choice,
      promotion.outer_module,
    )?;
    let wrapper_is_inline = choice_child_variant_can_inline(
      &promotion.wrapper_variant.payload,
      promotion.outer_module,
      type_graph,
    );
    let wrapper_variant = wrapper_variant_ident.to_string();
    let boxed = !wrapper_is_inline;
    let mut fields = Vec::new();
    for member in &promotion.wrapper_type.members {
      let MemberDecl::Field(field) = member else {
        continue;
      };
      let mapping = match field.wire {
        FieldWireDecl::Choice => {
          gen_promoted_choice_field_mapping(promotion, field, lookup, type_graph)?
        }
        FieldWireDecl::Child { .. } | FieldWireDecl::TextChild { .. } => {
          gen_promoted_direct_field_mapping(promotion, field, type_graph)?
        }
        FieldWireDecl::Attribute { .. } | FieldWireDecl::Text | FieldWireDecl::Any => None,
      };
      if let Some(mapping) = mapping {
        fields.push(mapping);
      }
    }
    wrappers.push(quote! {
      WrapperMapping {
        variant: #wrapper_variant,
        boxed: #boxed,
        fields: &[ #( #fields, )* ],
      }
    });
  }

  Ok(quote! {
    Mapping {
      wrappers: &[ #( #wrappers, )* ],
    }
  })
}

pub(crate) fn gen_mce_process_content_mapping(
  modules: &[&SchemaModuleDecl],
  type_graph: &TypeContainmentGraph,
) -> Result<TokenStream> {
  let lookup = type_decl_lookup(modules);
  let promotions = collect_mce_process_content_promotions(modules);
  let mut grouped = std::collections::BTreeMap::<String, Vec<_>>::new();
  for promotion in promotions {
    grouped
      .entry(local_type_key(
        promotion.outer_module,
        &promotion.outer_choice.rust_name,
      ))
      .or_default()
      .push(promotion);
  }
  let mut mapping_by_choice = std::collections::BTreeMap::new();
  for (choice_key, entries) in grouped {
    mapping_by_choice.insert(
      choice_key,
      gen_mce_process_content_mapping_entry(&entries, &lookup, type_graph)?,
    );
  }

  let mut mapping_arms = Vec::new();
  let mut mapped_choices = HashSet::new();
  let mut mapping_keys = HashSet::new();
  for module in modules {
    for owner in &module.types {
      let Some(owner_qname) = owner
        .xml_qname
        .as_deref()
        .map(schema_qname_element_name)
        .filter(|qname| !qname.is_empty())
      else {
        continue;
      };
      for member in &owner.members {
        let MemberDecl::Field(field) = member else {
          continue;
        };
        if field.wire != FieldWireDecl::Choice || field.cardinality != Cardinality::Many {
          continue;
        }
        let Some(choice_key) = schema_type_key_from_ref(module, &field.type_ref) else {
          continue;
        };
        let Some(mapping) = mapping_by_choice.get(&choice_key) else {
          continue;
        };
        if !mapped_choices.insert(choice_key.clone()) {
          return Err(
            format!(
              "ProcessContent choice {choice_key} is referenced by more than one generated field"
            )
            .into(),
          );
        }
        let owner_type = owner.rust_name.to_upper_camel_case();
        let field_name = field.rust_name.to_snake_case();
        let choice_type = field.type_ref.rust_type.to_upper_camel_case();
        let key = (
          owner_qname.to_string(),
          owner_type.clone(),
          field_name.clone(),
          choice_type.clone(),
        );
        if !mapping_keys.insert(key.clone()) {
          return Err(format!("duplicate ProcessContent derive mapping key {key:?}").into());
        }
        mapping_arms.push(quote! {
          (#owner_qname, #owner_type, #field_name, #choice_type) => {
            Some(#mapping)
          }
        });
      }
    }
  }

  if mapped_choices.len() != mapping_by_choice.len() {
    let missing = mapping_by_choice
      .keys()
      .filter(|choice| !mapped_choices.contains(*choice))
      .cloned()
      .collect::<Vec<_>>();
    return Err(format!("ProcessContent choices have no derive owner field: {missing:?}").into());
  }

  Ok(quote! {
    #[derive(Clone, Copy, Debug)]
    pub(crate) enum Cardinality {
      One,
      Optional,
      Many,
    }

    #[derive(Clone, Copy, Debug)]
    pub(crate) enum FieldMapping {
      Direct {
        field: &'static str,
        target_variant: &'static str,
        cardinality: Cardinality,
      },
      Choice {
        field: &'static str,
        source_module: &'static str,
        source_choice: &'static str,
        cardinality: Cardinality,
        exhaustive: bool,
        variants: &'static [&'static str],
        unit_variants: &'static [&'static str],
        renamed_variants: &'static [(&'static str, &'static str)],
      },
    }

    #[derive(Clone, Copy, Debug)]
    pub(crate) struct WrapperMapping {
      pub(crate) variant: &'static str,
      pub(crate) boxed: bool,
      pub(crate) fields: &'static [FieldMapping],
    }

    #[derive(Clone, Copy, Debug)]
    pub(crate) struct Mapping {
      pub(crate) wrappers: &'static [WrapperMapping],
    }

    pub(crate) fn resolve(
      owner_qname: &str,
      owner_type: &str,
      field_name: &str,
      choice_type: &str,
    ) -> Option<Mapping> {
      match (owner_qname, owner_type, field_name, choice_type) {
        #( #mapping_arms, )*
        _ => None,
      }
    }
  })
}

const DIRECT_CHILD_INLINE_SIZE_LIMIT: usize = 64;
const DIRECT_CHILD_INLINE_DEPTH_LIMIT: usize = 2;
const CHOICE_CHILD_INLINE_SIZE_LIMIT: usize = 96;
const CHOICE_CHILD_INLINE_DEPTH_LIMIT: usize = 2;

fn type_graph_small_leaf_like(
  type_graph: &TypeContainmentGraph,
  target_key: &str,
  size_limit: usize,
) -> bool {
  type_graph.contains_node(target_key)
    && !type_graph.has_outgoing_edges(target_key)
    && type_graph
      .estimated_size(target_key)
      .is_some_and(|size| size <= size_limit)
}

fn direct_child_field_can_inline(
  owner_rust_name: &str,
  field: &FieldDecl,
  module: &SchemaModuleDecl,
  type_graph: &TypeContainmentGraph,
) -> bool {
  let Some(target_key) = schema_type_key_from_ref(module, &field.type_ref) else {
    return false;
  };
  let owner_key = local_type_key(module, owner_rust_name);
  if type_graph.can_reach(&target_key, &owner_key) {
    return false;
  }
  type_graph_small_leaf_like(type_graph, &target_key, DIRECT_CHILD_INLINE_SIZE_LIMIT)
    && type_graph
      .shallow_depth(&target_key)
      .is_some_and(|depth| depth <= DIRECT_CHILD_INLINE_DEPTH_LIMIT)
}

fn choice_child_variant_can_inline(
  payload: &TypeRefDecl,
  module: &SchemaModuleDecl,
  type_graph: &TypeContainmentGraph,
) -> bool {
  let Some(target_key) = schema_type_key_from_ref(module, payload) else {
    return false;
  };

  type_graph_small_leaf_like(type_graph, &target_key, CHOICE_CHILD_INLINE_SIZE_LIMIT)
    && type_graph
      .shallow_depth(&target_key)
      .is_some_and(|depth| depth <= CHOICE_CHILD_INLINE_DEPTH_LIMIT)
}

fn direct_child_field_needs_box(
  owner_rust_name: &str,
  field: &FieldDecl,
  module: &SchemaModuleDecl,
  type_graph: &TypeContainmentGraph,
  force_optional_when_not_repeated: bool,
) -> bool {
  let effective_cardinality =
    if force_optional_when_not_repeated && !matches!(field.cardinality, Cardinality::Many) {
      Cardinality::Optional
    } else {
      field.cardinality
    };

  if is_any_children_alias_type_ref(module, &field.type_ref, type_graph) {
    return false;
  }

  match &field.wire {
    FieldWireDecl::Child { .. } => {}
    FieldWireDecl::TextChild { .. }
      if !is_value_like_type_ref(module, &field.type_ref, type_graph) => {}
    FieldWireDecl::TextChild { .. } => return false,
    _ => return false,
  }

  if matches!(effective_cardinality, Cardinality::Many) {
    return false;
  }

  if matches!(effective_cardinality, Cardinality::One) {
    return !direct_child_field_can_inline(owner_rust_name, field, module, type_graph);
  }

  let Some(target_key) = schema_type_key_from_ref(module, &field.type_ref) else {
    return false;
  };

  if !type_graph.contains_node(&target_key) {
    return true;
  }

  if type_graph.has_outgoing_edges(&target_key) {
    return true;
  }

  let owner_key = local_type_key(module, owner_rust_name);
  type_graph.can_reach(&target_key, &owner_key)
}

fn gen_support_fields(support: &SystemSupportDecl) -> Vec<TokenStream> {
  let mut fields = Vec::new();

  if support.have_xmlns_fields {
    fields.push(quote! {
      pub xmlns: Vec<crate::common::XmlNamespace>,
    });
  }

  if support.have_mc_ignorable {
    fields.push(quote! {
      pub mc_ignorable: Option<std::boxed::Box<[u8]>>,
    });
  }

  if support.have_mc_preserve_attributes {
    fields.push(quote! {
      pub mc_preserve_attributes: Option<std::boxed::Box<[u8]>>,
    });
  }

  if support.have_mc_preserve_elements {
    fields.push(quote! {
      pub mc_preserve_elements: Option<std::boxed::Box<[u8]>>,
    });
  }

  if support.have_mc_process_content {
    fields.push(quote! {
      pub mc_process_content: Option<std::boxed::Box<[u8]>>,
    });
  }

  if support.have_mc_must_understand {
    fields.push(quote! {
      pub mc_must_understand: Option<std::boxed::Box<[u8]>>,
    });
  }

  fields
}

fn choice_type_accepts_text(module: &SchemaModuleDecl, rust_type: &str) -> bool {
  module
    .types
    .iter()
    .find(|type_decl| type_decl.rust_name == rust_type && type_decl.kind == TypeKind::ChoiceEnum)
    .map(|type_decl| {
      type_decl.members.iter().any(|member| {
        matches!(
          member,
          MemberDecl::Variant(VariantDecl {
            wire: VariantWireDecl::Text,
            ..
          })
        )
      })
    })
    .unwrap_or(false)
}

fn choice_type_accepts_any(module: &SchemaModuleDecl, rust_type: &str) -> bool {
  fn type_accepts_any_recursive(
    module: &SchemaModuleDecl,
    rust_type: &str,
    visited: &mut HashSet<String>,
  ) -> bool {
    if !visited.insert(rust_type.to_string()) {
      return false;
    }

    let result = module
      .types
      .iter()
      .find(|type_decl| type_decl.rust_name == rust_type)
      .map(|type_decl| match type_decl.kind {
        TypeKind::ChoiceEnum => type_decl.members.iter().any(|member| match member {
          MemberDecl::Variant(VariantDecl {
            wire: VariantWireDecl::Any { qnames },
            ..
          }) => qnames.is_empty(),
          MemberDecl::Variant(VariantDecl {
            wire: VariantWireDecl::Child { .. },
            payload,
            ..
          })
          | MemberDecl::Variant(VariantDecl {
            wire: VariantWireDecl::Sequence { .. },
            payload,
            ..
          })
          | MemberDecl::Variant(VariantDecl {
            wire: VariantWireDecl::TextChild { .. },
            payload,
            ..
          }) => type_accepts_any_recursive(module, &payload.rust_type, visited),
          _ => false,
        }),
        TypeKind::HelperStruct => type_decl.members.iter().any(|member| match member {
          MemberDecl::Field(FieldDecl {
            wire: FieldWireDecl::Any,
            ..
          }) => true,
          MemberDecl::Field(FieldDecl {
            wire: FieldWireDecl::Choice,
            type_ref,
            ..
          }) => type_accepts_any_recursive(module, &type_ref.rust_type, visited),
          _ => false,
        }),
        _ => false,
      })
      .unwrap_or(false);

    visited.remove(rust_type);
    result
  }

  type_accepts_any_recursive(module, rust_type, &mut HashSet::new())
}

fn choice_metadata_qname(qname: &str) -> String {
  schema_qname_element_name(qname).to_string()
}

fn choice_sequence_child_metadata_tokens(
  field: &FieldDecl,
  variant_ident: Option<&Ident>,
  include_option_field: bool,
  module: &SchemaModuleDecl,
  type_graph: &TypeContainmentGraph,
) -> Result<Option<TokenStream>> {
  let variant_attr = variant_ident.map(|variant_ident| quote! { variant = #variant_ident, });
  let option_field_name = include_option_field.then(|| {
    let ident: Result<Ident> = parse_str(&field.rust_name).map_err(Into::into);
    ident
  });
  let option_field_name = option_field_name.transpose()?;
  let option_field_attr = option_field_name
    .as_ref()
    .map(|field_name| quote! { option_field = #field_name, });

  Ok(match &field.wire {
    FieldWireDecl::Child { qname } => {
      let qname = choice_metadata_qname(qname);
      if empty_leaf_marker_doc_for_ref(module, &field.type_ref, type_graph).is_some() {
        Some(quote! { empty_child(#variant_attr #option_field_attr qname = #qname) })
      } else if is_any_children_alias_type_ref(module, &field.type_ref, type_graph) {
        Some(quote! { any_child(#variant_attr #option_field_attr qname = #qname) })
      } else {
        Some(quote! { child(#variant_attr #option_field_attr qname = #qname) })
      }
    }
    FieldWireDecl::TextChild { qname } => {
      let metadata_attrs =
        text_child_value_metadata_attrs_from_type_ref(module, &field.type_ref, type_graph);
      let qname = choice_metadata_qname(qname);
      Some(quote! { text_child(#variant_attr #option_field_attr #metadata_attrs qname = #qname) })
    }
    _ => None,
  })
}

fn choice_variant_metadata_tokens(
  variant: &VariantDecl,
  type_decl: &TypeDecl,
  module: &SchemaModuleDecl,
  type_graph: &TypeContainmentGraph,
) -> Result<Vec<TokenStream>> {
  let rendered_variant_name_map = anonymous_variant_render_name_map(type_decl, module);
  let rendered_variant_name = rendered_variant_name_map
    .get(&variant.rust_name)
    .map(String::as_str)
    .unwrap_or(&variant.rust_name);
  let variant_ident: Ident =
    parse_str(&escape_upper_camel_case(rendered_variant_name.to_string()))?;
  match &variant.wire {
    VariantWireDecl::Any { qnames } if qnames.is_empty() => Ok(vec![quote! { any }]),
    VariantWireDecl::Any { qnames } => {
      let qnames = qnames
        .iter()
        .map(|qname| LitStr::new(qname, Span::call_site()));
      Ok(vec![quote! { any(qnames = [#(#qnames),*]) }])
    }
    VariantWireDecl::Text => Ok(vec![quote! { text }]),
    VariantWireDecl::TextChild { qnames } => Ok(
      qnames
        .iter()
        .map(|qname| {
          let metadata_attrs =
            text_child_value_metadata_attrs_from_type_ref(module, &variant.payload, type_graph);
          let qname = choice_metadata_qname(qname);
          quote! { text_child(variant = #variant_ident, #metadata_attrs qname = #qname) }
        })
        .collect(),
    ),
    VariantWireDecl::Child { qnames } => {
      let is_any_children_alias =
        is_any_children_alias_type_ref(module, &variant.payload, type_graph);
      let is_empty_leaf_marker =
        is_empty_leaf_marker_ref_with_graph(module, &variant.payload, type_graph);
      let item_name = if is_empty_leaf_marker {
        quote! { empty_child }
      } else if is_any_children_alias {
        quote! { any_child }
      } else {
        quote! { child }
      };
      Ok(
        qnames
          .iter()
          .map(|qname| {
            let qname = choice_metadata_qname(qname);
            let boxed = (!is_empty_leaf_marker
              && !is_any_children_alias
              && !choice_child_variant_can_inline(&variant.payload, module, type_graph))
            .then(|| quote! { boxed, });
            quote! { #item_name(variant = #variant_ident, #boxed qname = #qname) }
          })
          .collect(),
      )
    }
    VariantWireDecl::Sequence { .. } => {
      match sequence_variant_shape(variant, rendered_variant_name, module)? {
        SequenceVariantShape::SingleField {
          variant_ident,
          field,
        } => Ok(
          choice_sequence_child_metadata_tokens(
            field,
            Some(&variant_ident),
            false,
            module,
            type_graph,
          )?
          .into_iter()
          .collect(),
        ),
        SequenceVariantShape::BoxedSequence {
          variant_ident,
          helper_fields,
        } => {
          let include_option_field = helper_fields.iter().all(|field| {
            field.cardinality == Cardinality::Optional
              && matches!(field.wire, FieldWireDecl::Child { .. })
          });
          let mut children = Vec::new();
          for field in helper_fields {
            if let Some(child) = choice_sequence_child_metadata_tokens(
              field,
              None,
              include_option_field,
              module,
              type_graph,
            )? {
              children.push(child);
            }
          }

          Ok(vec![quote! {
            sequence(variant = #variant_ident #(, #children)*)
          }])
        }
      }
    }
  }
}

fn choice_field_metadata_tokens(
  module: &SchemaModuleDecl,
  rust_type: &str,
  type_graph: &TypeContainmentGraph,
) -> Result<Vec<TokenStream>> {
  let Some(type_decl) = module
    .types
    .iter()
    .find(|type_decl| type_decl.rust_name == rust_type && type_decl.kind == TypeKind::ChoiceEnum)
  else {
    return Ok(Vec::new());
  };

  let mut tokens = Vec::new();
  for member in &type_decl.members {
    let MemberDecl::Variant(variant) = member else {
      continue;
    };
    tokens.extend(choice_variant_metadata_tokens(
      variant, type_decl, module, type_graph,
    )?);
  }
  Ok(tokens)
}

fn gen_direct_child_fields_from_decl(
  fields: &[&FieldDecl],
  owner_rust_name: &str,
  module: &SchemaModuleDecl,
  type_graph: &TypeContainmentGraph,
  field_cfg: VersionCfgContext,
  force_optional_when_not_repeated: bool,
) -> Result<Vec<TokenStream>> {
  gen_direct_child_fields_from_decl_with_context(
    fields,
    owner_rust_name,
    module,
    type_graph,
    field_cfg,
    force_optional_when_not_repeated,
  )
}

fn gen_direct_child_fields_from_decl_with_context(
  fields: &[&FieldDecl],
  owner_rust_name: &str,
  module: &SchemaModuleDecl,
  type_graph: &TypeContainmentGraph,
  field_cfg: VersionCfgContext,
  force_optional_when_not_repeated: bool,
) -> Result<Vec<TokenStream>> {
  let mut tokens = Vec::new();
  let alternate_content_children = module
    .types
    .iter()
    .find(|type_decl| type_decl.rust_name == owner_rust_name)
    .map(|type_decl| &type_decl.support.alternate_content_children);

  for field in fields {
    let attr = module_version_cfg_attrs(&field.version, field_cfg);
    let field_name_ident: Ident = parse_str(&field.rust_name)?;
    let empty_leaf_marker_doc = empty_leaf_marker_doc_for_ref(module, &field.type_ref, type_graph);
    let field_type = type_from_decl_ref(&field.type_ref, type_graph)?;
    let property_comments_owned = empty_leaf_marker_doc
      .and_then(meaningful_doc_text)
      .or_else(|| meaningful_doc_text(&field.docs))
      .unwrap_or_else(|| " _".to_string());
    let property_comments = property_comments_owned.as_str();
    let field_sdk_version_markers = sdk_version_markers(&field.version);
    let is_any_children_alias = is_any_children_alias_type_ref(module, &field.type_ref, type_graph);
    let sdk_field_attrs = match &field.wire {
      FieldWireDecl::Child { qname } if empty_leaf_marker_doc.is_some() => {
        let qname = sdk_element_qname(qname);
        quote! { #[sdk(empty_child(#(#field_sdk_version_markers,)* qname = #qname))] }
      }
      FieldWireDecl::Child { qname } if is_any_children_alias => {
        let qname = sdk_element_qname(qname);
        quote! { #[sdk(any_child(#(#field_sdk_version_markers,)* qname = #qname))] }
      }
      FieldWireDecl::Child { qname } if qname == "mc:AlternateContent" => {
        let qname = sdk_element_qname(qname);
        let children = alternate_content_children
          .and_then(|children| children.get(&field.rust_name))
          .ok_or_else(|| {
            format!(
              "static MCE field {owner_rust_name}.{} must declare children",
              field.rust_name
            )
          })?
          .iter()
          .map(|child| parse_str::<Ident>(child))
          .collect::<syn::Result<Vec<_>>>()?;
        quote! { #[sdk(mce(qname = #qname, children = [#(#children),*]))] }
      }
      FieldWireDecl::Child { qname } => {
        let qname = sdk_element_qname(qname);
        quote! { #[sdk(child(#(#field_sdk_version_markers,)* qname = #qname))] }
      }
      FieldWireDecl::TextChild { qname } => {
        let list_attr = is_list_type_ref(&field.type_ref).then_some(quote! { list, });
        let qname = sdk_element_qname(qname);
        quote! { #[sdk(text_child(#(#field_sdk_version_markers,)* #list_attr qname = #qname))] }
      }
      _ => return Err(format!("expected direct child field, got {:?}", field.wire).into()),
    };

    let field_type = if empty_leaf_marker_doc.is_some() {
      parse_str("()")?
    } else {
      field_type
    };
    let wrap_box = empty_leaf_marker_doc.is_none()
      && !is_any_children_alias
      && !is_list_type_ref(&field.type_ref)
      && direct_child_field_needs_box(
        owner_rust_name,
        field,
        module,
        type_graph,
        force_optional_when_not_repeated,
      );

    let effective_cardinality =
      if force_optional_when_not_repeated && !matches!(field.cardinality, Cardinality::Many) {
        Cardinality::Optional
      } else {
        field.cardinality
      };

    let field_tokens = match effective_cardinality {
      Cardinality::Many => quote! {
        #( #attr )*
        #[doc = #property_comments]
        #sdk_field_attrs
        pub #field_name_ident: Vec<#field_type>,
      },
      Cardinality::Optional if wrap_box => quote! {
        #( #attr )*
        #[doc = #property_comments]
        #sdk_field_attrs
        pub #field_name_ident: Option<std::boxed::Box<#field_type>>,
      },
      Cardinality::Optional => quote! {
        #( #attr )*
        #[doc = #property_comments]
        #sdk_field_attrs
        pub #field_name_ident: Option<#field_type>,
      },
      Cardinality::One if wrap_box => quote! {
        #( #attr )*
        #[doc = #property_comments]
        #sdk_field_attrs
        pub #field_name_ident: std::boxed::Box<#field_type>,
      },
      Cardinality::One => quote! {
        #( #attr )*
        #[doc = #property_comments]
        #sdk_field_attrs
        pub #field_name_ident: #field_type,
      },
    };

    tokens.push(field_tokens);
  }

  Ok(tokens)
}

fn gen_flatten_one_sequence_fields_from_decl(
  fields: &[&FieldDecl],
  owner_rust_name: &str,
  module: &SchemaModuleDecl,
  type_graph: &TypeContainmentGraph,
  field_cfg: VersionCfgContext,
  _choice_dispatch_field_names: &HashSet<String>,
) -> Result<Vec<TokenStream>> {
  let mut tokens = Vec::new();

  for field in fields {
    match &field.wire {
      FieldWireDecl::Choice => {
        tokens.extend(gen_choice_fields_from_decl(
          std::slice::from_ref(field),
          module,
          type_graph,
          field_cfg,
          &HashSet::new(),
        )?);
      }
      FieldWireDecl::Any => {
        let attrs = module_version_cfg_attrs(&field.version, field_cfg);
        let field_name_ident: Ident = parse_str(&field.rust_name)?;
        let property_comments = field.docs.as_str();
        let field_type = type_from_decl_ref(&field.type_ref, type_graph)?;
        tokens.push(match field.cardinality {
          Cardinality::Many => quote! {
            #( #attrs )*
            #[doc = #property_comments]
            #[sdk(any)]
            pub #field_name_ident: Vec<#field_type>,
          },
          Cardinality::Optional => quote! {
            #( #attrs )*
            #[doc = #property_comments]
            #[sdk(any)]
            pub #field_name_ident: Option<#field_type>,
          },
          Cardinality::One => quote! {
            #( #attrs )*
            #[doc = #property_comments]
            #[sdk(any)]
            pub #field_name_ident: #field_type,
          },
        });
      }
      FieldWireDecl::Child { .. } | FieldWireDecl::TextChild { .. } => {
        tokens.extend(gen_direct_child_fields_from_decl_with_context(
          std::slice::from_ref(field),
          owner_rust_name,
          module,
          type_graph,
          field_cfg,
          false,
        )?);
      }
      _ => return Err(format!("expected flatten one-sequence field, got {:?}", field.wire).into()),
    }
  }

  Ok(tokens)
}

fn gen_choice_fields_from_decl(
  fields: &[&FieldDecl],
  module: &SchemaModuleDecl,
  type_graph: &TypeContainmentGraph,
  field_cfg: VersionCfgContext,
  _choice_dispatch_field_names: &HashSet<String>,
) -> Result<Vec<TokenStream>> {
  let mut tokens = Vec::new();

  for field in fields {
    let field_name_ident: Ident = parse_str(&field.rust_name)?;
    let field_type = type_from_decl_ref(&field.type_ref, type_graph)?;
    let attrs = module_version_cfg_attrs(&field.version, field_cfg);
    let choice_accepts_text = choice_type_accepts_text(module, &field.type_ref.rust_type);
    let choice_accepts_any = choice_type_accepts_any(module, &field.type_ref.rust_type);
    let mut choice_items =
      choice_field_metadata_tokens(module, &field.type_ref.rust_type, type_graph)?;
    if choice_accepts_text
      && !choice_items
        .iter()
        .any(|tokens| tokens.to_string().starts_with("text"))
    {
      choice_items.push(quote! { text });
    }
    if choice_accepts_any
      && !choice_items
        .iter()
        .any(|tokens| tokens.to_string().starts_with("any"))
    {
      choice_items.push(quote! { any });
    }
    let mut choice_field_attrs = Vec::new();
    if choice_items.is_empty() {
      choice_field_attrs.push(quote! { #[sdk(choice)] });
    } else {
      choice_field_attrs.push(quote! { #[sdk(choice(#(#choice_items),*))] });
    };

    match field.cardinality {
      Cardinality::Many => tokens.push(quote! {
        #( #attrs )*
        #( #choice_field_attrs )*
        pub #field_name_ident: Vec<#field_type>,
      }),
      Cardinality::Optional => tokens.push(quote! {
        #( #attrs )*
        #( #choice_field_attrs )*
        pub #field_name_ident: Option<#field_type>,
      }),
      Cardinality::One => tokens.push(quote! {
        #( #attrs )*
        #( #choice_field_attrs )*
        pub #field_name_ident: #field_type,
      }),
    }
  }

  Ok(tokens)
}

fn collect_resolved_sequence_leafs<'a>(
  children: &'a [ResolvedCompositeChild<'a>],
  out: &mut Vec<&'a ResolvedCompositeChild<'a>>,
) {
  for child in children {
    match child.kind {
      SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any => {
        out.push(child);
      }
      SchemaTypeChildKind::Choice | SchemaTypeChildKind::Sequence => {
        collect_resolved_sequence_leafs(&child.children, out);
      }
    }
  }
}

fn can_alias_leaf_text_wrapper_decl(type_decl: &TypeDecl, attr_fields: &[&FieldDecl]) -> bool {
  type_decl.kind == TypeKind::LeafTextAlias
    && attr_fields.is_empty()
    && !type_decl.support.have_xmlns_fields
    && !type_decl.support.has_extra_support_fields()
    && !type_decl.support.has_xml_header
}

fn can_alias_any_children_wrapper_decl(type_decl: &TypeDecl, attr_fields: &[&FieldDecl]) -> bool {
  if type_decl.kind != TypeKind::ElementStruct
    || type_decl.content_model != Some(ContentModelDecl::SequenceAnyOnly)
    || !attr_fields.is_empty()
    || type_decl.xml_content.is_some()
    || type_decl.support.have_xmlns_fields
    || type_decl.support.has_extra_support_fields()
    || type_decl.support.has_xml_header
  {
    return false;
  }

  let [MemberDecl::Field(field)] = type_decl.members.as_slice() else {
    return false;
  };

  matches!(field.wire, FieldWireDecl::Any)
    && field.cardinality == Cardinality::Many
    && field.type_ref.module_path.is_none()
    && field.type_ref.rust_type == "std::boxed::Box<[u8]>"
}

fn can_wrap_derived_to_base_decl(type_decl: &TypeDecl, base_type_decl: &TypeDecl) -> bool {
  type_decl.kind == TypeKind::ElementStruct
    && type_decl.element_kind == Some(ElementKind::Derived)
    && base_type_decl.kind == TypeKind::ElementStruct
    && base_type_decl.element_kind == Some(ElementKind::LeafText)
    && type_decl.members.is_empty()
    && type_decl.content_model.is_none()
    && type_decl.xml_content.is_some()
    && type_decl.support == base_type_decl.support
}

fn should_emit_schema_type_decl(type_decl: &TypeDecl) -> bool {
  !type_decl.is_abstract
    || (type_decl.kind == TypeKind::ElementStruct
      && type_decl.element_kind == Some(ElementKind::LeafText)
      && type_decl.xml_content.is_some()
      && type_decl.support.has_mce_attributes())
}

fn is_any_children_alias_type_ref(
  module: &SchemaModuleDecl,
  type_ref: &TypeRefDecl,
  type_graph: &TypeContainmentGraph,
) -> bool {
  schema_type_key_from_ref(module, type_ref)
    .as_deref()
    .is_some_and(|key| type_graph.is_any_children_alias(key))
}

fn omitted_abstract_helper_type_names(ir: &SchemaModuleDecl) -> HashSet<&str> {
  let helper_type_names: HashSet<&str> = ir
    .types
    .iter()
    .filter(|type_decl| {
      matches!(
        type_decl.kind,
        TypeKind::ChoiceEnum | TypeKind::HelperStruct
      )
    })
    .map(|type_decl| type_decl.rust_name.as_str())
    .collect();

  ir.types
    .iter()
    .filter(|type_decl| type_decl.is_abstract)
    .flat_map(|type_decl| &type_decl.members)
    .filter_map(|member| match member {
      MemberDecl::Field(field) if field.type_ref.module_path.is_none() => {
        Some(field.type_ref.rust_type.as_str())
      }
      _ => None,
    })
    .filter(|rust_type| helper_type_names.contains(rust_type))
    .collect()
}

fn choice_child_display_name<'a>(child: &'a ResolvedOneSequenceChild<'a>) -> &'a str {
  if child.name.is_empty() {
    child.field_name.as_ref()
  } else {
    schema_qname_element_name(child.name)
  }
}

fn child_kind_for_schema_type(schema_type: &SchemaType) -> SchemaTypeChildKind {
  if schema_type.base_class == "OpenXmlLeafTextElement"
    && schema_type.attributes.is_empty()
    && !schema_type.have_xmlns_fields
    && !schema_type.have_mc_ignorable
    && !schema_type.have_mc_preserve_attributes
    && !schema_type.have_mc_preserve_elements
    && !schema_type.have_mc_process_content
    && !schema_type.have_mc_must_understand
  {
    SchemaTypeChildKind::TextChild
  } else {
    SchemaTypeChildKind::Child
  }
}

pub fn one_sequence_child_variant_type(
  child: &ResolvedOneSequenceChild<'_>,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<Type> {
  let child_type = context
    .type_map
    .get(child.name)
    .ok_or_else(|| format!("{:?}", child.name))?;
  let child_prefix = context
    .type_prefix_map
    .get(child.name)
    .ok_or_else(|| format!("{:?}", child.name))?;

  if *child_prefix != schema.prefix {
    let child_module_name = context
      .type_module(child.name)
      .ok_or_else(|| format!("{:?}", child.name))?;

    Ok(parse_str(&format!(
      "crate::schemas::{}::{}",
      child_module_name,
      child_type.class_name.to_upper_camel_case()
    ))?)
  } else {
    Ok(parse_str(&child_type.class_name.to_upper_camel_case())?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::sdk_code::codegen_ir::SchemaModuleDecl;
  use crate::sdk_code::codegen_ir_builder::build_codegen_ir;
  use crate::sdk_data::context::Context;
  use crate::sdk_data::schemas::{gen_namespaces, gen_schemas};
  use crate::sdk_data::sdk_data_model::{SchemaTypeApiKind, SchemaTypeCompositeKind};
  use serde_json::Value;
  use std::fs;
  use std::fs::File;
  use std::io::BufReader;
  use std::path::Path;

  fn load_legacy_schema_context(schema_path: &str) -> Vec<Schema> {
    let schema_dir = Path::new(schema_path).parent().unwrap();
    let mut schemas: Vec<Schema> = vec![];

    for entry in fs::read_dir(schema_dir).unwrap() {
      let entry = entry.unwrap();
      let path = entry.path();

      if !path.is_file() || path.extension().and_then(|ext| ext.to_str()) != Some("json") {
        continue;
      }

      let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        continue;
      };
      if file_name.starts_with("package_") {
        continue;
      }

      let file = File::open(&path).unwrap();
      let value: Value = serde_json::from_reader(BufReader::new(file)).unwrap();
      if value
        .get("Types")
        .and_then(|types| types.as_array())
        .and_then(|types| types.first())
        .and_then(|ty| ty.get("RustName"))
        .is_some()
      {
        continue;
      }

      schemas.push(serde_json::from_value(value).unwrap());
    }

    schemas.sort_by(|left, right| left.module_name.cmp(&right.module_name));
    schemas
  }

  #[test]
  fn lowers_empty_leaf_marker_choice_variants_to_unit_variants() {
    let schema = read_codegen_ir_schema_json(
      "../../sdk_data/schemas/schemas_openxmlformats_org_office_document_2006_doc_props_v_types.json",
    );
    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("empty_child (variant = VtEmpty , qname = \"vt:empty\")"));
    assert!(generated.contains("empty_child (variant = VtNull , qname = \"vt:null\")"));
    assert!(generated.contains("text_child (variant = VtInt32 , qname = \"vt:i4\")"));
    assert!(generated.contains("text_child (variant = VtUnsignedInt32 , qname = \"vt:ui4\")"));
    assert!(generated.contains("text_child (variant = VtBlob , qname = \"vt:blob\")"));
    assert!(generated.contains("text_child (variant = Vtlpstr , qname = \"vt:lpstr\")"));
    assert!(!generated.contains("simple_type ="));
    assert!(generated.contains("VtEmpty ,"));
    assert!(generated.contains("VtNull ,"));
    assert!(!generated.contains("pub struct VtEmpty"));
    assert!(!generated.contains("pub struct VtNull"));
    assert!(!generated.contains("VtEmpty (std :: boxed :: Box < VtEmpty >)"));
    assert!(!generated.contains("VtNull (std :: boxed :: Box < VtNull >)"));
  }

  #[test]
  fn marks_choice_text_child_schema_enums() {
    let schema =
      read_codegen_ir_schema_json("../../sdk_data/schemas/schemas_microsoft_com_office_excel.json");
    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(
      generated.contains("text_child (variant = AutoFill , enum , qname = \"xvml:AutoFill\")")
    );
    assert!(generated.contains("text_child (variant = Anchor , qname = \"xvml:Anchor\")"));
    assert!(!generated.contains("simple_type ="));
  }

  #[test]
  fn lowers_openxml_empty_element_fields_to_unit_fields() {
    let generated =
      render_workspace_schema("schemas_microsoft_com_office_drawing_2013_main_command");

    assert!(generated.contains("# [sdk (empty_child (qname = \"oac:fill\"))]"));
    assert!(generated.contains("pub fill_empty : Option < () >"));
    assert!(!generated.contains("pub struct FillEmpty"));
    assert!(!generated.contains("pub fill_empty : Option < FillEmpty >"));
  }

  #[test]
  fn lowers_empty_type_derived_choice_variants_to_unit_variants() {
    let schema = read_codegen_ir_schema_json(
      "../../sdk_data/schemas/schemas_microsoft_com_office_word_2010_wordml.json",
    );
    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("empty_child (variant = NoFillEmpty , qname = \"w14:noFill\")"));
    assert!(generated.contains("NoFillEmpty ,"));
    assert!(!generated.contains("pub struct NoFillEmpty"));
    assert!(!generated.contains("NoFillEmpty (std :: boxed :: Box < NoFillEmpty >)"));
  }

  #[test]
  fn omits_abstract_empty_base_types_without_element_name() {
    let generated = render_workspace_schema("schemas_microsoft_com_office_powerpoint_2022_08_main");

    assert!(!generated.contains("pub struct EmptyType"));
    assert!(generated.contains("empty_child (variant = AddEmpty , qname = \"p228:add\")"));
    assert!(generated.contains("AddEmpty ,"));
  }

  #[test]
  fn omits_abstract_schema_base_structs_but_keeps_derived_support_fields() {
    let schema = read_codegen_ir_schema_json(
      "../../sdk_data/schemas/schemas_openxmlformats_org_drawingml_2006_main.json",
    );
    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(!generated.contains("pub struct StyleMatrixReferenceType"));
    assert!(!generated.contains("pub enum StyleMatrixReferenceTypeChoice"));
    assert!(generated.contains("pub struct FillReference"));
    assert!(generated.contains("pub xmlns : Vec < crate :: common :: XmlNamespace >"));
  }

  #[test]
  fn keeps_xsd_unit_aliases_without_emitting_simple_type_metadata() {
    let drawing =
      render_workspace_schema("schemas_microsoft_com_office_word_2010_wordprocessing_drawing");
    assert!(drawing.contains(
      "pub type PercentagePositionHeightOffset = crate :: simple_type :: DrawingmlPercentageValue"
    ));
    assert!(drawing.contains("text_child (qname = \"wp14:pctWidth\")"));
    assert!(!drawing.contains("simple_type ="));

    let math = render_workspace_schema("schemas_openxmlformats_org_office_document_2006_math");
    assert!(math.contains("pub val : crate :: simple_type :: TwipsMeasureValue"));
    assert!(!math.contains("simple_type ="));
  }

  fn read_codegen_ir_schema_json(path: &str) -> SchemaModuleDecl {
    let file = File::open(path).unwrap();
    let value: Value = serde_json::from_reader(BufReader::new(file)).unwrap();

    if value
      .get("Types")
      .and_then(|types| types.as_array())
      .and_then(|types| types.first())
      .and_then(|ty| ty.get("RustName"))
      .is_some()
    {
      serde_json::from_value(value).unwrap()
    } else {
      let schema: Schema = serde_json::from_value(value).unwrap();
      let legacy_schemas = load_legacy_schema_context(path);
      let context = CodegenContext::new(&legacy_schemas);
      build_codegen_ir(&schema, &context).unwrap()
    }
  }

  fn load_workspace_codegen_inputs() -> (&'static [Schema], CodegenContext<'static>) {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
      .parent()
      .and_then(|path| path.parent())
      .expect("workspace root");
    let context = Context::new(&workspace_root.join("data")).expect("context");
    let schemas = gen_schemas(&context);
    let namespaces = Box::leak(Box::new(gen_namespaces(&context))).as_slice();
    let leaked_schemas = Box::leak(Box::new(schemas)).as_slice();
    let codegen_context = CodegenContext::new_with_namespaces(leaked_schemas, namespaces);

    (leaked_schemas, codegen_context)
  }

  #[test]
  fn mce_element_version_overrides_are_exact_and_source_backed() {
    let (schemas, context) = load_workspace_codegen_inputs();
    let mut actual_count = 0usize;
    let mut expected_count = 0usize;
    for schema in schemas {
      let ir = build_codegen_ir(schema, &context).unwrap();
      let expected = schema
        .types
        .iter()
        .filter(|ty| ty.api_kind != SchemaTypeApiKind::LeafTextWrapper)
        .filter(|ty| context.element_version_override(ty).is_some())
        .map(|ty| ty.class_name.as_str())
        .collect::<HashSet<_>>();
      let actual = ir
        .types
        .iter()
        .filter(|ty| !mce_element_version_markers(ty).unwrap().is_empty())
        .map(|ty| {
          assert_eq!(ty.kind, TypeKind::ElementStruct);
          ty.rust_name.as_str()
        })
        .collect::<HashSet<_>>();
      assert_eq!(actual, expected, "{}", schema.target_namespace);
      actual_count += actual.len();
      expected_count += expected.len();
      for ty in &ir.types {
        if let Some(version) = &ty.support.element_version_override {
          assert_eq!(ty.version.as_deref(), Some(version.as_str()));
        }
      }
    }
    assert_eq!(actual_count, expected_count);
    assert_eq!(actual_count, 22);
  }

  #[test]
  fn process_content_promotion_set_is_bounded() {
    let (schemas, context) = load_workspace_codegen_inputs();
    let modules = schemas
      .iter()
      .map(|schema| build_codegen_ir(schema, &context).unwrap())
      .collect::<Vec<_>>();
    let module_refs = modules.iter().collect::<Vec<_>>();
    let promotions = collect_mce_process_content_promotions(&module_refs);
    let promotion_names = promotions
      .iter()
      .map(|promotion| {
        format!(
          "{}::{}::{}",
          promotion.outer_module.module_name,
          promotion.outer_choice.rust_name,
          promotion.wrapper_variant.rust_name
        )
      })
      .collect::<Vec<_>>();
    assert_eq!(promotions.len(), 42, "{promotion_names:#?}");
    let owners = promotions
      .iter()
      .map(|promotion| local_type_key(promotion.outer_module, &promotion.outer_choice.rust_name))
      .collect::<HashSet<_>>();
    assert_eq!(owners.len(), 14);
    let owner_fields = module_refs
      .iter()
      .flat_map(|module| {
        module.types.iter().flat_map(|ty| {
          ty.members.iter().filter_map(|member| {
            let MemberDecl::Field(field) = member else {
              return None;
            };
            (field.wire == FieldWireDecl::Choice
              && field.cardinality == Cardinality::Many
              && schema_type_key_from_ref(module, &field.type_ref)
                .is_some_and(|key| owners.contains(&key)))
            .then_some((ty.rust_name.as_str(), field.rust_name.as_str()))
          })
        })
      })
      .collect::<HashSet<_>>();
    assert_eq!(owner_fields.len(), 14);
    let type_graph = TypeContainmentGraph::from_modules(&module_refs);
    let mapping = gen_mce_process_content_mapping(&module_refs, &type_graph)
      .unwrap()
      .to_string();
    for forbidden in [
      "TokenStream",
      "quote !",
      "__ooxmlsdk_promote_process_content",
      "Self ::",
      "output . push",
    ] {
      assert!(!mapping.contains(forbidden), "{forbidden}: {mapping}");
    }
    assert!(mapping.contains("struct Mapping"));
    assert!(mapping.contains("enum FieldMapping"));
  }

  fn render_workspace_schema(module_name: &str) -> String {
    let (schemas, context) = load_workspace_codegen_inputs();
    let ir_modules: Vec<_> = schemas
      .iter()
      .map(|schema| build_codegen_ir(schema, &context).expect("build ir"))
      .collect();
    let ir_refs: Vec<_> = ir_modules.iter().collect();
    let type_graph = TypeContainmentGraph::from_modules(&ir_refs);
    let ir = ir_modules
      .iter()
      .find(|ir| ir.module_name == module_name)
      .expect("schema module");

    gen_schema_from_ir_with_type_graph(ir, false, &type_graph)
      .expect("render schema")
      .to_string()
  }

  #[test]
  fn generates_slide_moniker_list_as_flat_sequence_with_any_tail() {
    let schema = SchemaModuleDecl {
      module_name: "schemas_microsoft_com_office_powerpoint_2013_main_command".to_string(),
      target_namespace: "http://schemas.microsoft.com/office/powerpoint/2013/main/command"
        .to_string(),
      prefix: "pc".to_string(),
      typed_namespace: "Test.Command".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "CSlideMoniker".to_string(),
          xml_qname: Some("pc:CT_CSlideMoniker/pc:cSldMk".to_string()),
          version: Some("Office2013".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "SlideMoniker".to_string(),
          xml_qname: Some("pc:CT_SlideMoniker/pc:sldMk".to_string()),
          version: Some("Office2013".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "SlideMonikerList".to_string(),
          xml_qname: Some("pc:CT_SlideMonikerList/pc:sldMkLst".to_string()),
          version: Some("Office2013".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Composite),
          content_model: Some(ContentModelDecl::OneSequenceFlatten),
          estimated_size: 0,
          members: vec![
            MemberDecl::Field(FieldDecl {
              rust_name: "c_sld_moniker".to_string(),
              docs: " _".to_string(),
              version: "Office2013".to_string(),
              wire: FieldWireDecl::Child {
                qname: "pc:cSldMk".to_string(),
              },
              cardinality: Cardinality::Optional,
              type_ref: TypeRefDecl {
                rust_type: "CSlideMoniker".to_string(),
                module_path: None,
              },
              validators: vec![],
            }),
            MemberDecl::Field(FieldDecl {
              rust_name: "sld_moniker".to_string(),
              docs: " _".to_string(),
              version: "Office2013".to_string(),
              wire: FieldWireDecl::Child {
                qname: "pc:sldMk".to_string(),
              },
              cardinality: Cardinality::Optional,
              type_ref: TypeRefDecl {
                rust_type: "SlideMoniker".to_string(),
                module_path: None,
              },
              validators: vec![],
            }),
            MemberDecl::Field(FieldDecl {
              rust_name: "unknown_xml".to_string(),
              docs: " _".to_string(),
              version: "Office2013".to_string(),
              wire: FieldWireDecl::Any,
              cardinality: Cardinality::Many,
              type_ref: TypeRefDecl {
                rust_type: "std::boxed::Box<[u8]>".to_string(),
                module_path: None,
              },
              validators: vec![],
            }),
          ],
          ..Default::default()
        },
      ],
      ..Default::default()
    };
    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    let slide_moniker_list = schema
      .types
      .iter()
      .find(|item| item.rust_name == "SlideMonikerList")
      .unwrap();
    assert_eq!(
      slide_moniker_list.content_model,
      Some(ContentModelDecl::OneSequenceFlatten)
    );
    assert!(generated.contains("pub c_sld_moniker :"));
    assert!(generated.contains("pub sld_moniker :"));
    assert!(generated.contains("pub unknown_xml : Vec < std :: boxed :: Box < [u8] > >"));
  }

  #[test]
  fn generates_any_only_choice_without_boxed_string_payload() {
    let schema = read_codegen_ir_schema_json(
      "../../sdk_data/schemas/schemas_microsoft_com_office_2006_metadata_content_type.json",
    );
    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(
      generated.contains("# [sdk (any)] pub xml_children : Vec < std :: boxed :: Box < [u8] > >")
    );
    assert!(!generated.contains("pub enum ContentTypeSchemaChoice"));
    assert!(!generated.contains("UnknownXml (String)"));
    assert!(!generated.contains("UnknownXml (std :: boxed :: Box < String >)"));
  }

  #[test]
  fn keeps_multi_field_sequence_helper_variants() {
    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "First".to_string(),
          xml_qname: Some("t:CT_First/t:first".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "Second".to_string(),
          xml_qname: Some("t:CT_Second/t:second".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![MemberDecl::Variant(
            crate::sdk_code::codegen_ir::VariantDecl {
              rust_name: "Sequence1".to_string(),
              docs: " Sequence of t:first, t:second".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Sequence {
                qnames: vec![
                  "t:CT_First/t:first".to_string(),
                  "t:CT_Second/t:second".to_string(),
                ],
              },
              payload: TypeRefDecl {
                rust_type: "HolderChoiceSequence1".to_string(),
                module_path: None,
              },
            },
          )],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoiceSequence1".to_string(),
          kind: TypeKind::HelperStruct,
          estimated_size: 0,
          members: vec![
            MemberDecl::Field(FieldDecl {
              rust_name: "first".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: FieldWireDecl::Child {
                qname: "t:CT_First/t:first".to_string(),
              },
              cardinality: Cardinality::Optional,
              type_ref: TypeRefDecl {
                rust_type: "First".to_string(),
                module_path: None,
              },
              validators: vec![],
            }),
            MemberDecl::Field(FieldDecl {
              rust_name: "second".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: FieldWireDecl::Child {
                qname: "t:CT_Second/t:second".to_string(),
              },
              cardinality: Cardinality::Optional,
              type_ref: TypeRefDecl {
                rust_type: "Second".to_string(),
                module_path: None,
              },
              validators: vec![],
            }),
          ],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("Sequence (std :: boxed :: Box < HolderChoiceSequence >)"));
    assert!(generated.contains("pub struct HolderChoiceSequence"));
  }

  #[test]
  fn collapses_single_field_sequence_variant_into_direct_child_variant_at_render_time() {
    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "First".to_string(),
          xml_qname: Some("t:CT_First/t:first".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![MemberDecl::Variant(VariantDecl {
            rust_name: "EgThing".to_string(),
            docs: " Sequence of t:first".to_string(),
            version: String::new(),
            wire: crate::sdk_code::codegen_ir::VariantWireDecl::Sequence {
              qnames: vec!["t:CT_First/t:first".to_string()],
            },
            payload: TypeRefDecl {
              rust_type: "HolderChoiceSequence1".to_string(),
              module_path: None,
            },
          })],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoiceSequence1".to_string(),
          kind: TypeKind::HelperStruct,
          estimated_size: 0,
          members: vec![MemberDecl::Field(FieldDecl {
            rust_name: "first".to_string(),
            docs: " _".to_string(),
            version: String::new(),
            wire: FieldWireDecl::Child {
              qname: "t:CT_First/t:first".to_string(),
            },
            cardinality: Cardinality::Optional,
            type_ref: TypeRefDecl {
              rust_type: "First".to_string(),
              module_path: None,
            },
            validators: vec![],
          })],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("EgThing (std :: boxed :: Box < First >)"));
    assert!(!generated.contains("# [sdk (sequence)] EgThing {"));
  }

  #[test]
  fn keeps_small_sequence_helper_variants_with_text_child() {
    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "Formula".to_string(),
          xml_qname: Some("t:CT_Formula/t:f".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![MemberDecl::Variant(
            crate::sdk_code::codegen_ir::VariantDecl {
              rust_name: "Sequence1".to_string(),
              docs: " Sequence of t:f, t:v".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Sequence {
                qnames: vec!["t:CT_Formula/t:f".to_string(), "xsd:string/t:v".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "HolderChoiceSequence1".to_string(),
                module_path: None,
              },
            },
          )],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoiceSequence1".to_string(),
          kind: TypeKind::HelperStruct,
          estimated_size: 0,
          members: vec![
            MemberDecl::Field(FieldDecl {
              rust_name: "formula".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: FieldWireDecl::Child {
                qname: "t:CT_Formula/t:f".to_string(),
              },
              cardinality: Cardinality::One,
              type_ref: TypeRefDecl {
                rust_type: "Formula".to_string(),
                module_path: None,
              },
              validators: vec![],
            }),
            MemberDecl::Field(FieldDecl {
              rust_name: "value".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: FieldWireDecl::TextChild {
                qname: "xsd:string/t:v".to_string(),
              },
              cardinality: Cardinality::Optional,
              type_ref: TypeRefDecl {
                rust_type: "StringValue".to_string(),
                module_path: Some("crate::simple_type".to_string()),
              },
              validators: vec![],
            }),
          ],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("Sequence (std :: boxed :: Box < HolderChoiceSequence >)"));
    assert!(generated.contains("pub struct HolderChoiceSequence"));
    assert!(!generated.contains("Sequence { formula"));
  }

  #[test]
  fn inlines_nested_choice_helper_variants() {
    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "First".to_string(),
          xml_qname: Some("t:CT_First/t:first".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "Second".to_string(),
          xml_qname: Some("t:CT_Second/t:second".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![MemberDecl::Variant(VariantDecl {
            rust_name: "Choice1".to_string(),
            docs: " Choice wrapper".to_string(),
            version: String::new(),
            wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
              qnames: vec!["wrapper".to_string()],
            },
            payload: TypeRefDecl {
              rust_type: "HolderChoice1".to_string(),
              module_path: None,
            },
          })],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice1".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "TFirst".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_First/t:first".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "First".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "TSecond".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_Second/t:second".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "Second".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("pub enum HolderChoice"));
    assert!(generated.contains("TFirst (First)"));
    assert!(generated.contains("TSecond (Second)"));
    assert!(!generated.contains("# [sdk (choice)] Choice1"));
    assert!(!generated.contains("pub enum HolderChoice1"));
  }

  #[test]
  fn keeps_nested_choice_helper_when_non_inline_reference_remains() {
    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "First".to_string(),
          xml_qname: Some("t:CT_First/t:first".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "Second".to_string(),
          xml_qname: Some("t:CT_Second/t:second".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "TFirst".to_string(),
              docs: " Direct child".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_First/t:first".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "First".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "Choice1".to_string(),
              docs: " Choice wrapper".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["wrapper".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "HolderChoice1".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice1".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "TFirst".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_First/t:first".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "First".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "TSecond".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_Second/t:second".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "Second".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("pub enum HolderChoice"));
    assert!(generated.contains("Choice (HolderChoice2)"));
  }

  #[test]
  fn keeps_pure_choice_wrapper_variants_nested() {
    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "First".to_string(),
          xml_qname: Some("t:CT_First/t:first".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "Second".to_string(),
          xml_qname: Some("t:CT_Second/t:second".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "Third".to_string(),
          xml_qname: Some("t:CT_Third/t:third".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "Choice1".to_string(),
              docs: " Wrapper 1".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["wrapper1".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "HolderChoice1".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "Choice2".to_string(),
              docs: " Wrapper 2".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["wrapper2".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "HolderChoice2".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice1".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "TFirst".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_First/t:first".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "First".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "Sequence1".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Sequence {
                qnames: vec!["t:CT_Second/t:second".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "HolderChoiceSequence1".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice2".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![MemberDecl::Variant(VariantDecl {
            rust_name: "TThird".to_string(),
            docs: " _".to_string(),
            version: String::new(),
            wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
              qnames: vec!["t:CT_Third/t:third".to_string()],
            },
            payload: TypeRefDecl {
              rust_type: "Third".to_string(),
              module_path: None,
            },
          })],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoiceSequence1".to_string(),
          kind: TypeKind::HelperStruct,
          estimated_size: 0,
          members: vec![MemberDecl::Field(FieldDecl {
            rust_name: "second".to_string(),
            docs: " _".to_string(),
            version: String::new(),
            wire: FieldWireDecl::Child {
              qname: "t:CT_Second/t:second".to_string(),
            },
            cardinality: Cardinality::One,
            type_ref: TypeRefDecl {
              rust_type: "Second".to_string(),
              module_path: None,
            },
            validators: vec![],
          })],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("pub enum HolderChoice"));
    assert!(generated.contains("Choice1 (HolderChoice2)"));
    assert!(generated.contains("Second (std :: boxed :: Box < Second >)"));
  }

  #[test]
  fn keeps_pure_choice_wrapper_when_helper_variants_collide() {
    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "First".to_string(),
          xml_qname: Some("t:CT_First/t:first".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "Choice1".to_string(),
              docs: " Wrapper 1".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["wrapper1".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "HolderChoice1".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "Choice2".to_string(),
              docs: " Wrapper 2".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["wrapper2".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "HolderChoice2".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice1".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![MemberDecl::Variant(VariantDecl {
            rust_name: "TFirst".to_string(),
            docs: " _".to_string(),
            version: String::new(),
            wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
              qnames: vec!["t:CT_First/t:first".to_string()],
            },
            payload: TypeRefDecl {
              rust_type: "First".to_string(),
              module_path: None,
            },
          })],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice2".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![MemberDecl::Variant(VariantDecl {
            rust_name: "TFirst".to_string(),
            docs: " _".to_string(),
            version: String::new(),
            wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
              qnames: vec!["t:CT_First/t:first".to_string()],
            },
            payload: TypeRefDecl {
              rust_type: "First".to_string(),
              module_path: None,
            },
          })],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("pub enum HolderChoice2"));
    assert!(generated.contains("pub enum HolderChoice3"));
    assert!(generated.contains("Choice1 (HolderChoice2)"));
    assert!(generated.contains("Choice2 (HolderChoice3)"));
  }

  #[test]
  fn keeps_pure_choice_wrapper_payload_helpers_without_render_flattening() {
    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "First".to_string(),
          xml_qname: Some("t:CT_First/t:first".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "Second".to_string(),
          xml_qname: Some("t:CT_Second/t:second".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "Choice1".to_string(),
              docs: " Wrapper 1".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["wrapper1".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "HolderChoice1".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "Choice2".to_string(),
              docs: " Wrapper 2".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["wrapper2".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "HolderChoice2".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice1".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![MemberDecl::Variant(VariantDecl {
            rust_name: "TFirst".to_string(),
            docs: " _".to_string(),
            version: String::new(),
            wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
              qnames: vec!["t:CT_First/t:first".to_string()],
            },
            payload: TypeRefDecl {
              rust_type: "First".to_string(),
              module_path: None,
            },
          })],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice2".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![MemberDecl::Variant(VariantDecl {
            rust_name: "TSecond".to_string(),
            docs: " _".to_string(),
            version: String::new(),
            wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
              qnames: vec!["t:CT_Second/t:second".to_string()],
            },
            payload: TypeRefDecl {
              rust_type: "Second".to_string(),
              module_path: None,
            },
          })],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("pub enum HolderChoice"));
    assert!(generated.contains("Choice1 (HolderChoice2)"));
    assert!(generated.contains("Choice2 (HolderChoice3)"));
  }

  #[test]
  fn keeps_anonymous_choice_wrapper_even_when_parent_variant_name_matches_nested_choice() {
    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "AltChunk".to_string(),
          xml_qname: Some("t:CT_AltChunk/t:altChunk".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "ProofErr".to_string(),
          xml_qname: Some("t:CT_ProofErr/t:proofErr".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "PermStart".to_string(),
          xml_qname: Some("t:CT_PermStart/t:permStart".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "RootChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "WAltChunk".to_string(),
              docs: " Direct".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_AltChunk/t:altChunk".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "AltChunk".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "Choice2".to_string(),
              docs: " Wrapper".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["wrapper".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "RootChoice1".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "RootChoice1".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "Choice1".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_ProofErr/t:proofErr".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "RootChoice2".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "Choice2".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_PermStart/t:permStart".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "PermStart".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "RootChoice2".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![MemberDecl::Variant(VariantDecl {
            rust_name: "WProofErr".to_string(),
            docs: " _".to_string(),
            version: String::new(),
            wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
              qnames: vec!["t:CT_ProofErr/t:proofErr".to_string()],
            },
            payload: TypeRefDecl {
              rust_type: "ProofErr".to_string(),
              module_path: None,
            },
          })],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("pub enum RootChoice"));
    assert!(generated.contains("Choice (RootChoice2)"));
  }

  #[test]
  fn emits_choice_dispatch_marker_for_specific_choice_fields() {
    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "First".to_string(),
          xml_qname: Some("t:CT_First/t:first".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "Second".to_string(),
          xml_qname: Some("t:CT_Second/t:second".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "TFirst".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: VariantWireDecl::Child {
                qnames: vec!["t:CT_First/t:first".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "First".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "TSecond".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: VariantWireDecl::Sequence {
                qnames: vec!["t:CT_Second/t:second".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "Second".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "Holder".to_string(),
          xml_qname: Some("t:CT_Holder/t:holder".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Composite),
          estimated_size: 0,
          members: vec![MemberDecl::Field(FieldDecl {
            rust_name: "holder_choice".to_string(),
            docs: " _".to_string(),
            version: String::new(),
            wire: FieldWireDecl::Choice,
            cardinality: Cardinality::Optional,
            type_ref: TypeRefDecl {
              rust_type: "HolderChoice".to_string(),
              module_path: None,
            },
            validators: vec![],
          })],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("child (variant = TFirst"));
    assert!(generated.contains("qname = \"t:first\""));
  }

  #[test]
  fn includes_versioned_choice_qnames_in_parent_field_metadata() {
    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "AlwaysHere".to_string(),
          xml_qname: Some("t:CT_Always/t:always".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "OfficeOnly".to_string(),
          xml_qname: Some("t14:CT_OfficeOnly/t14:officeOnly".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "Always".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: VariantWireDecl::Child {
                qnames: vec!["t:CT_Always/t:always".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "AlwaysHere".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "Office".to_string(),
              docs: " _".to_string(),
              version: "Office2010".to_string(),
              wire: VariantWireDecl::Child {
                qnames: vec!["t14:CT_OfficeOnly/t14:officeOnly".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "OfficeOnly".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "Holder".to_string(),
          xml_qname: Some("t:CT_Holder/t:holder".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Composite),
          estimated_size: 0,
          members: vec![MemberDecl::Field(FieldDecl {
            rust_name: "holder_choice".to_string(),
            docs: " _".to_string(),
            version: String::new(),
            wire: FieldWireDecl::Choice,
            cardinality: Cardinality::Optional,
            type_ref: TypeRefDecl {
              rust_type: "HolderChoice".to_string(),
              module_path: None,
            },
            validators: vec![],
          })],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("# [sdk (choice ("));
    assert!(generated.contains("qname = \"t:always\""));
    assert!(generated.contains("qname = \"t14:officeOnly\""));
    assert!(!generated.contains("cfg_attr"));
  }

  #[test]
  fn keeps_multi_step_anonymous_choice_wrappers_inside_mixed_parent() {
    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "First".to_string(),
          xml_qname: Some("t:CT_First/t:first".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "Second".to_string(),
          xml_qname: Some("t:CT_Second/t:second".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "Third".to_string(),
          xml_qname: Some("t:CT_Third/t:third".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "RootChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "Choice1".to_string(),
              docs: " Wrapper".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["wrapper".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "RootChoice1".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "TThird".to_string(),
              docs: " Direct".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_Third/t:third".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "Third".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "RootChoice1".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![MemberDecl::Variant(VariantDecl {
            rust_name: "Choice1".to_string(),
            docs: " Nested wrapper".to_string(),
            version: String::new(),
            wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
              qnames: vec!["wrapper2".to_string()],
            },
            payload: TypeRefDecl {
              rust_type: "RootChoice2".to_string(),
              module_path: None,
            },
          })],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "RootChoice2".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "TFirst".to_string(),
              docs: " First".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_First/t:first".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "First".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "TSecond".to_string(),
              docs: " Second".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_Second/t:second".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "Second".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("pub enum RootChoice"));
    assert!(generated.contains("TThird (Third)"));
    assert!(generated.contains("Choice (RootChoice2)"));
  }

  #[test]
  fn keeps_anonymous_wrapper_around_small_named_choice_helper() {
    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "Direct".to_string(),
          xml_qname: Some("t:CT_Direct/t:direct".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "First".to_string(),
          xml_qname: Some("t:CT_First/t:first".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "Second".to_string(),
          xml_qname: Some("t:CT_Second/t:second".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "RootChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "TDirect".to_string(),
              docs: " Direct".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_Direct/t:direct".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "Direct".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "Choice2".to_string(),
              docs: " Wrapper".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["wrapper".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "RangeMarkupChoice".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "RangeMarkupChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "TFirst".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_First/t:first".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "First".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "TSecond".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_Second/t:second".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "Second".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("pub enum RootChoice"));
    assert!(generated.contains("Choice (RangeMarkupChoice)"));
    assert!(generated.contains("pub enum RangeMarkupChoice"));
  }

  #[test]
  fn keeps_anonymous_wrapper_around_large_named_choice_helper() {
    let leaf = TypeDecl {
      rust_name: "Leaf".to_string(),
      xml_qname: Some("t:CT_Leaf/t:leaf".to_string()),
      kind: TypeKind::ElementStruct,
      element_kind: Some(ElementKind::Leaf),
      ..Default::default()
    };
    let direct = TypeDecl {
      rust_name: "Direct".to_string(),
      xml_qname: Some("t:CT_Direct/t:direct".to_string()),
      kind: TypeKind::ElementStruct,
      element_kind: Some(ElementKind::Leaf),
      ..Default::default()
    };
    let mut members = Vec::new();
    for idx in 1..=13 {
      members.push(MemberDecl::Variant(VariantDecl {
        rust_name: format!("TLeaf{idx}"),
        docs: " _".to_string(),
        version: String::new(),
        wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
          qnames: vec![format!("t:CT_Leaf/t:leaf{idx}")],
        },
        payload: TypeRefDecl {
          rust_type: "Leaf".to_string(),
          module_path: None,
        },
      }));
    }

    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        leaf,
        direct,
        TypeDecl {
          rust_name: "RootChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "TDirect".to_string(),
              docs: " Direct".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_Direct/t:direct".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "Direct".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "Choice2".to_string(),
              docs: " Wrapper".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["wrapper".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "RangeMarkupChoice".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "RangeMarkupChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          members,
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("pub enum RangeMarkupChoice"));
    assert!(generated.contains("Choice (RangeMarkupChoice)"));
  }

  #[test]
  fn renames_single_anonymous_sequence_variant_without_numeric_suffix() {
    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "First".to_string(),
          xml_qname: Some("t:CT_First/t:first".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "Second".to_string(),
          xml_qname: Some("t:CT_Second/t:second".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "TFirst".to_string(),
              docs: " Direct".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_First/t:first".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "First".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "Sequence32".to_string(),
              docs: " Anonymous sequence".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Sequence {
                qnames: vec!["t:CT_Second/t:second".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "HolderChoiceSequence32".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoiceSequence32".to_string(),
          kind: TypeKind::HelperStruct,
          estimated_size: 0,
          members: vec![MemberDecl::Field(FieldDecl {
            rust_name: "second".to_string(),
            docs: " _".to_string(),
            version: String::new(),
            wire: FieldWireDecl::Child {
              qname: "t:CT_Second/t:second".to_string(),
            },
            cardinality: Cardinality::One,
            type_ref: TypeRefDecl {
              rust_type: "Second".to_string(),
              module_path: None,
            },
            validators: vec![],
          })],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("Second (std :: boxed :: Box < Second >)"));
    assert!(!generated.contains("Sequence32"));
  }

  #[test]
  fn renumbers_multiple_anonymous_sequence_variants_by_render_order() {
    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "First".to_string(),
          xml_qname: Some("t:CT_First/t:first".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "Second".to_string(),
          xml_qname: Some("t:CT_Second/t:second".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "Sequence32".to_string(),
              docs: " first".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Sequence {
                qnames: vec!["t:CT_First/t:first".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "HolderChoiceSequence32".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "Sequence8".to_string(),
              docs: " second".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Sequence {
                qnames: vec!["t:CT_Second/t:second".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "HolderChoiceSequence8".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoiceSequence32".to_string(),
          kind: TypeKind::HelperStruct,
          estimated_size: 0,
          members: vec![MemberDecl::Field(FieldDecl {
            rust_name: "first".to_string(),
            docs: " _".to_string(),
            version: String::new(),
            wire: FieldWireDecl::Child {
              qname: "t:CT_First/t:first".to_string(),
            },
            cardinality: Cardinality::One,
            type_ref: TypeRefDecl {
              rust_type: "First".to_string(),
              module_path: None,
            },
            validators: vec![],
          })],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoiceSequence8".to_string(),
          kind: TypeKind::HelperStruct,
          estimated_size: 0,
          members: vec![MemberDecl::Field(FieldDecl {
            rust_name: "second".to_string(),
            docs: " _".to_string(),
            version: String::new(),
            wire: FieldWireDecl::Child {
              qname: "t:CT_Second/t:second".to_string(),
            },
            cardinality: Cardinality::One,
            type_ref: TypeRefDecl {
              rust_type: "Second".to_string(),
              module_path: None,
            },
            validators: vec![],
          })],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("First (std :: boxed :: Box < First >)"));
    assert!(generated.contains("Second (std :: boxed :: Box < Second >)"));
    assert!(!generated.contains("Sequence32"));
    assert!(!generated.contains("Sequence8"));
  }

  #[test]
  fn keeps_anonymous_sequence_suffix_when_plain_name_would_collide() {
    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "First".to_string(),
          xml_qname: Some("t:CT_First/t:first".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "Second".to_string(),
          xml_qname: Some("t:CT_Second/t:second".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "Sequence".to_string(),
              docs: " Named".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_First/t:first".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "First".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "Sequence32".to_string(),
              docs: " Anonymous".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Sequence {
                qnames: vec!["t:CT_Second/t:second".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "HolderChoiceSequence32".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "HolderChoiceSequence32".to_string(),
          kind: TypeKind::HelperStruct,
          estimated_size: 0,
          members: vec![MemberDecl::Field(FieldDecl {
            rust_name: "second".to_string(),
            docs: " _".to_string(),
            version: String::new(),
            wire: FieldWireDecl::Child {
              qname: "t:CT_Second/t:second".to_string(),
            },
            cardinality: Cardinality::One,
            type_ref: TypeRefDecl {
              rust_type: "Second".to_string(),
              module_path: None,
            },
            validators: vec![],
          })],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("Sequence (First)"));
    assert!(generated.contains("Second (std :: boxed :: Box < Second >)"));
  }

  #[test]
  fn keeps_repeated_choice_segments_split_by_fixed_children() {
    let schema = SchemaModuleDecl {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "BookmarkStart".to_string(),
          xml_qname: Some("t:CT_Bookmark/t:bookmarkStart".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "BookmarkEnd".to_string(),
          xml_qname: Some("t:CT_MarkupRange/t:bookmarkEnd".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "TableProperties".to_string(),
          xml_qname: Some("t:CT_TblPr/t:tblPr".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "TableGrid".to_string(),
          xml_qname: Some("t:CT_TblGrid/t:tblGrid".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "TableRow".to_string(),
          xml_qname: Some("t:CT_Row/t:tr".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "BodyLike".to_string(),
          xml_qname: Some("t:CT_BodyLike/t:bodyLike".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Composite),
          content_model: Some(ContentModelDecl::MixedChoiceChildren),
          estimated_size: 0,
          members: vec![
            MemberDecl::Field(FieldDecl {
              rust_name: "before_choice".to_string(),
              docs: " before".to_string(),
              version: String::new(),
              wire: FieldWireDecl::Choice,
              cardinality: Cardinality::Many,
              type_ref: TypeRefDecl {
                rust_type: "BodyLikeChoice1".to_string(),
                module_path: None,
              },
              validators: vec![],
            }),
            MemberDecl::Field(FieldDecl {
              rust_name: "table_properties".to_string(),
              docs: " tblPr".to_string(),
              version: String::new(),
              wire: FieldWireDecl::Child {
                qname: "t:CT_TblPr/t:tblPr".to_string(),
              },
              cardinality: Cardinality::One,
              type_ref: TypeRefDecl {
                rust_type: "TableProperties".to_string(),
                module_path: None,
              },
              validators: vec![],
            }),
            MemberDecl::Field(FieldDecl {
              rust_name: "table_grid".to_string(),
              docs: " tblGrid".to_string(),
              version: String::new(),
              wire: FieldWireDecl::Child {
                qname: "t:CT_TblGrid/t:tblGrid".to_string(),
              },
              cardinality: Cardinality::One,
              type_ref: TypeRefDecl {
                rust_type: "TableGrid".to_string(),
                module_path: None,
              },
              validators: vec![],
            }),
            MemberDecl::Field(FieldDecl {
              rust_name: "after_choice".to_string(),
              docs: " after".to_string(),
              version: String::new(),
              wire: FieldWireDecl::Choice,
              cardinality: Cardinality::Many,
              type_ref: TypeRefDecl {
                rust_type: "BodyLikeChoice2".to_string(),
                module_path: None,
              },
              validators: vec![],
            }),
          ],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "BodyLikeChoice1".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![MemberDecl::Variant(VariantDecl {
            rust_name: "Choice1".to_string(),
            docs: " wrapper".to_string(),
            version: String::new(),
            wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
              qnames: vec!["wrapper".to_string()],
            },
            payload: TypeRefDecl {
              rust_type: "RangeMarkupChoice".to_string(),
              module_path: None,
            },
          })],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "RangeMarkupChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "TBookmarkStart".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_Bookmark/t:bookmarkStart".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "BookmarkStart".to_string(),
                module_path: None,
              },
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "TBookmarkEnd".to_string(),
              docs: " _".to_string(),
              version: String::new(),
              wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
                qnames: vec!["t:CT_MarkupRange/t:bookmarkEnd".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "BookmarkEnd".to_string(),
                module_path: None,
              },
            }),
          ],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "BodyLikeChoice2".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![MemberDecl::Variant(VariantDecl {
            rust_name: "TRow".to_string(),
            docs: " _".to_string(),
            version: String::new(),
            wire: crate::sdk_code::codegen_ir::VariantWireDecl::Child {
              qnames: vec!["t:CT_Row/t:tr".to_string()],
            },
            payload: TypeRefDecl {
              rust_type: "TableRow".to_string(),
              module_path: None,
            },
          })],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("pub struct BodyLike"));
    assert!(generated.contains("pub before_choice : Vec < BodyLikeChoice >"));
    assert!(generated.contains("pub table_properties : TableProperties"));
    assert!(generated.contains("pub table_grid : TableGrid"));
    assert!(generated.contains("pub after_choice : Vec < BodyLikeChoice2 >"));
    assert!(generated.contains("pub enum BodyLikeChoice"));
    assert!(generated.contains("TBookmarkStart (BookmarkStart)"));
    assert!(generated.contains("TBookmarkEnd (BookmarkEnd)"));

    let before_idx = generated
      .find("pub before_choice : Vec < BodyLikeChoice >")
      .unwrap();
    let tbl_pr_idx = generated
      .find("pub table_properties : TableProperties")
      .unwrap();
    let tbl_grid_idx = generated.find("pub table_grid : TableGrid").unwrap();
    let after_idx = generated
      .find("pub after_choice : Vec < BodyLikeChoice2 >")
      .unwrap();

    assert!(before_idx < tbl_pr_idx);
    assert!(tbl_pr_idx < tbl_grid_idx);
    assert!(tbl_grid_idx < after_idx);
  }

  #[test]
  fn prebuilt_ir_generation_ignores_schema_particle_shape() {
    let (schemas, context) = load_workspace_codegen_inputs();
    let schema = schemas
      .iter()
      .find(|schema| schema.module_name == "schemas_openxmlformats_org_wordprocessingml_2006_main")
      .expect("wordprocessing schema");
    let ir = build_codegen_ir(schema, &context).expect("build ir");

    let from_ir = gen_schema_from_ir(&ir, false)
      .expect("render from ir")
      .to_string();
    let from_prebuilt = gen_schema(schema, Some(&ir), &context, false)
      .expect("render from prebuilt ir")
      .to_string();

    let mut poisoned_schema = schema.clone();
    for schema_type in &mut poisoned_schema.types {
      schema_type.children.clear();
      schema_type.composite_kind = SchemaTypeCompositeKind::None;
    }

    let from_poisoned = gen_schema(&poisoned_schema, Some(&ir), &context, false)
      .expect("render from poisoned prebuilt ir")
      .to_string();

    assert_eq!(from_prebuilt, from_ir);
    assert_eq!(from_poisoned, from_ir);
  }

  #[test]
  fn generates_simple_one_choice_from_codegen_ir() {
    let schema = Schema {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        SchemaType {
          name: "t:CT_Leaf/t:leaf".to_string(),
          class_name: "Leaf".to_string(),
          version: Some("Office2010".to_string()),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_ChoiceHolder/t:holder".to_string(),
          class_name: "ChoiceHolder".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Composite,
          composite_kind: SchemaTypeCompositeKind::OneChoice,
          children: vec![SchemaTypeChild {
            kind: SchemaTypeChildKind::Choice,
            initial_version: "Office2010".to_string(),
            children: vec![SchemaTypeChild {
              name: "t:CT_Leaf/t:leaf".to_string(),
              property_name: "LeafChild".to_string(),
              property_comments: "Leaf child".to_string(),
              kind: SchemaTypeChildKind::Child,
              initial_version: "Office2010".to_string(),
              ..Default::default()
            }],
            ..Default::default()
          }],
          ..Default::default()
        },
      ],
      ..Default::default()
    };
    let context = CodegenContext::new(std::slice::from_ref(&schema));

    let generated = gen_schema(&schema, None, &context, false)
      .unwrap()
      .to_string();

    assert!(generated.contains("qname = \"t:leaf\""));
    assert!(!generated.contains("pub enum ChoiceHolderChoice"));
  }

  #[test]
  fn renders_empty_leaf_marker_choice_variants_as_unit_variants() {
    let schema = SchemaModuleDecl {
      module_name: "test_empty_leaf_choice".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.EmptyLeafChoice".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "Marker".to_string(),
          xml_qname: Some("t:CT_Marker/t:marker".to_string()),
          docs: "Defines the Marker Class.".to_string(),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          base_rust_name: Some("OpenXmlLeafElement".to_string()),
          ..Default::default()
        },
        TypeDecl {
          rust_name: "AttributedMarker".to_string(),
          xml_qname: Some("t:CT_AttributedMarker/t:attributedMarker".to_string()),
          docs: "Defines the AttributedMarker Class.".to_string(),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          base_rust_name: Some("OpenXmlLeafElement".to_string()),
          estimated_size: 0,
          members: vec![MemberDecl::Field(FieldDecl {
            rust_name: "value".to_string(),
            docs: "Value".to_string(),
            wire: FieldWireDecl::Attribute {
              qname: ":val".to_string(),
              bit: None,
              list: false,
              match_local_name: false,
              empty_as_none: false,
            },
            type_ref: TypeRefDecl {
              rust_type: "StringValue".to_string(),
              module_path: Some("crate::simple_type".to_string()),
            },
            ..Default::default()
          })],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "MarkerChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![
            MemberDecl::Variant(VariantDecl {
              rust_name: "TMarker".to_string(),
              docs: " _".to_string(),
              wire: VariantWireDecl::Child {
                qnames: vec!["t:CT_Marker/t:marker".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "Marker".to_string(),
                module_path: None,
              },
              ..Default::default()
            }),
            MemberDecl::Variant(VariantDecl {
              rust_name: "TAttributedMarker".to_string(),
              wire: VariantWireDecl::Child {
                qnames: vec!["t:CT_AttributedMarker/t:attributedMarker".to_string()],
              },
              payload: TypeRefDecl {
                rust_type: "AttributedMarker".to_string(),
                module_path: None,
              },
              ..Default::default()
            }),
          ],
          ..Default::default()
        },
      ],
      ..Default::default()
    };
    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(!generated.contains("pub struct Marker"));
    assert!(generated.contains("pub struct AttributedMarker"));
    assert!(generated.contains("# [doc = \" Defines the Marker Class.\"]"));
    assert!(generated.contains("TMarker ,"));
    assert!(generated.contains("# [doc = \" Defines the AttributedMarker Class.\"]"));
    assert!(generated.contains("TAttributedMarker (AttributedMarker)"));
  }

  #[test]
  fn generates_flatten_one_sequence_from_codegen_ir() {
    let schema = Schema {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        SchemaType {
          name: "t:CT_Leaf/t:leaf".to_string(),
          class_name: "Leaf".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_Text/t:text".to_string(),
          class_name: "TextLeaf".to_string(),
          base_class: "OpenXmlLeafTextElement".to_string(),
          api_kind: SchemaTypeApiKind::LeafTextWrapper,
          text_value_type: "StringValue".to_string(),
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_Seq/t:seq".to_string(),
          class_name: "SequenceHolder".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Composite,
          composite_kind: SchemaTypeCompositeKind::OneSequence,
          children: vec![
            SchemaTypeChild {
              name: "t:CT_Leaf/t:leaf".to_string(),
              property_name: "LeafChild".to_string(),
              property_comments: "Leaf child".to_string(),
              kind: SchemaTypeChildKind::Child,
              ..Default::default()
            },
            SchemaTypeChild {
              name: "wrapper".to_string(),
              kind: SchemaTypeChildKind::Sequence,
              optional: true,
              children: vec![SchemaTypeChild {
                kind: SchemaTypeChildKind::Choice,
                children: vec![SchemaTypeChild {
                  name: "t:CT_Text/t:text".to_string(),
                  property_name: "TextChild".to_string(),
                  property_comments: "Text child".to_string(),
                  kind: SchemaTypeChildKind::TextChild,
                  ..Default::default()
                }],
                ..Default::default()
              }],
              ..Default::default()
            },
            SchemaTypeChild {
              kind: SchemaTypeChildKind::Any,
              property_name: "UnknownXml".to_string(),
              property_comments: "Unknown xml".to_string(),
              repeated: true,
              ..Default::default()
            },
          ],
          ..Default::default()
        },
      ],
      ..Default::default()
    };
    let context = CodegenContext::new(std::slice::from_ref(&schema));

    let generated = gen_schema(&schema, None, &context, false)
      .unwrap()
      .to_string();

    assert!(generated.contains("pub leaf_child : Leaf"));
    assert!(generated.contains("pub text_child : Option < TextLeaf >"));
    assert!(generated.contains("pub unknown_xml : Vec < std :: boxed :: Box < [u8] > >"));
    assert!(!generated.contains("pub enum SequenceHolderChoice"));
  }

  #[test]
  fn keeps_list_text_child_as_vec_in_generated_schema() {
    let schema = Schema {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        SchemaType {
          name: "t:CT_List/t:list".to_string(),
          class_name: "ListLeaf".to_string(),
          base_class: "OpenXmlLeafTextElement".to_string(),
          api_kind: SchemaTypeApiKind::LeafTextWrapper,
          text_value_type: "ListValue<StringValue>".to_string(),
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_Holder/t:holder".to_string(),
          class_name: "Holder".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Composite,
          composite_kind: SchemaTypeCompositeKind::OneSequence,
          children: vec![SchemaTypeChild {
            name: "wrapper".to_string(),
            kind: SchemaTypeChildKind::Sequence,
            children: vec![SchemaTypeChild {
              kind: SchemaTypeChildKind::Choice,
              children: vec![SchemaTypeChild {
                name: "t:CT_List/t:list".to_string(),
                property_name: "ListChild".to_string(),
                property_comments: "List child".to_string(),
                kind: SchemaTypeChildKind::TextChild,
                ..Default::default()
              }],
              ..Default::default()
            }],
            ..Default::default()
          }],
          ..Default::default()
        },
      ],
      ..Default::default()
    };
    let context = CodegenContext::new(std::slice::from_ref(&schema));

    let generated = gen_schema(&schema, None, &context, false)
      .unwrap()
      .to_string();

    assert!(generated.contains("# [sdk (text_child (list , qname = \"t:list\"))]"));
    assert!(generated.contains("pub list_child : Vec < crate :: simple_type :: StringValue >"));
  }

  #[test]
  fn keeps_box_for_recursive_direct_child_fields() {
    let schema = SchemaModuleDecl {
      module_name: "test_recursive_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "Node".to_string(),
          xml_qname: Some("t:CT_Node/t:node".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Composite),
          content_model: Some(ContentModelDecl::DirectChildrenOnly),
          estimated_size: 0,
          members: vec![MemberDecl::Field(FieldDecl {
            rust_name: "child".to_string(),
            docs: "Child".to_string(),
            version: String::new(),
            wire: FieldWireDecl::Child {
              qname: "t:node".to_string(),
            },
            cardinality: Cardinality::Optional,
            type_ref: TypeRefDecl {
              rust_type: "Node".to_string(),
              module_path: None,
            },
            validators: vec![],
          })],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "Wrapper".to_string(),
          xml_qname: Some("t:CT_Wrapper/t:wrapper".to_string()),
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Composite),
          content_model: Some(ContentModelDecl::DirectChildrenOnly),
          estimated_size: 0,
          members: vec![MemberDecl::Field(FieldDecl {
            rust_name: "node".to_string(),
            docs: "Node".to_string(),
            version: String::new(),
            wire: FieldWireDecl::Child {
              qname: "t:node".to_string(),
            },
            cardinality: Cardinality::Optional,
            type_ref: TypeRefDecl {
              rust_type: "Node".to_string(),
              module_path: None,
            },
            validators: vec![],
          })],
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("pub child : Option <"));
    assert!(generated.contains("std :: boxed :: Box < Node >"));
    assert!(!generated.contains("pub child : Option < Node >"));
    assert!(generated.contains("pub node : Option <"));
    assert!(generated.contains("std :: boxed :: Box < Node >"));
    assert!(!generated.contains("pub node : Option < Node >"));
  }

  #[test]
  fn keeps_cross_module_text_child_leaf_alias_unboxed() {
    let provider = SchemaModuleDecl {
      module_name: "provider".to_string(),
      target_namespace: "urn:provider".to_string(),
      prefix: "p".to_string(),
      typed_namespace: "Test.Provider".to_string(),
      types: vec![TypeDecl {
        rust_name: "SharedText".to_string(),
        docs: "Shared text".to_string(),
        kind: TypeKind::LeafTextAlias,
        xml_content: Some(TypeRefDecl {
          rust_type: "StringValue".to_string(),
          module_path: Some("crate::simple_type".to_string()),
        }),
        ..Default::default()
      }],
      ..Default::default()
    };
    let consumer = SchemaModuleDecl {
      module_name: "consumer".to_string(),
      target_namespace: "urn:consumer".to_string(),
      prefix: "c".to_string(),
      typed_namespace: "Test.Consumer".to_string(),
      types: vec![TypeDecl {
        rust_name: "Holder".to_string(),
        xml_qname: Some("c:CT_Holder/c:holder".to_string()),
        kind: TypeKind::ElementStruct,
        element_kind: Some(ElementKind::Composite),
        content_model: Some(ContentModelDecl::DirectChildrenOnly),
        estimated_size: 0,
        members: vec![MemberDecl::Field(FieldDecl {
          rust_name: "shared_text".to_string(),
          docs: "Shared text".to_string(),
          wire: FieldWireDecl::TextChild {
            qname: "p:CT_SharedText/p:sharedText".to_string(),
          },
          cardinality: Cardinality::One,
          type_ref: TypeRefDecl {
            rust_type: "SharedText".to_string(),
            module_path: Some("crate::schemas::provider".to_string()),
          },
          validators: vec![],
          ..Default::default()
        })],
        ..Default::default()
      }],
      ..Default::default()
    };

    let type_graph = TypeContainmentGraph::from_modules(&[&provider, &consumer]);
    let generated = gen_schema_from_ir_with_type_graph(&consumer, false, &type_graph)
      .unwrap()
      .to_string();

    assert!(generated.contains("pub shared_text : crate :: schemas :: p :: SharedText"));
    assert!(
      !generated
        .contains("pub shared_text : std :: boxed :: Box < crate :: schemas :: p :: SharedText >")
    );
  }

  #[test]
  fn generates_structured_one_sequence_from_codegen_ir() {
    let schema = Schema {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        SchemaType {
          name: "t:CT_A/t:a".to_string(),
          class_name: "LeafA".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_B/t:b".to_string(),
          class_name: "LeafB".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_C/t:c".to_string(),
          class_name: "LeafC".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_D/t:d".to_string(),
          class_name: "LeafD".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_Seq/t:seq".to_string(),
          class_name: "StructuredHolder".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Composite,
          composite_kind: SchemaTypeCompositeKind::OneSequence,
          children: vec![SchemaTypeChild {
            name: "wrapper".to_string(),
            kind: SchemaTypeChildKind::Sequence,
            children: vec![
              SchemaTypeChild {
                name: "t:CT_A/t:a".to_string(),
                property_name: "LeafA".to_string(),
                property_comments: "Leaf A".to_string(),
                kind: SchemaTypeChildKind::Child,
                ..Default::default()
              },
              SchemaTypeChild {
                name: "choice_wrapper".to_string(),
                kind: SchemaTypeChildKind::Choice,
                children: vec![
                  SchemaTypeChild {
                    name: "t:CT_B/t:b".to_string(),
                    property_name: "LeafB".to_string(),
                    property_comments: "Leaf B".to_string(),
                    kind: SchemaTypeChildKind::Child,
                    ..Default::default()
                  },
                  SchemaTypeChild {
                    name: "seq_variant".to_string(),
                    kind: SchemaTypeChildKind::Sequence,
                    children: vec![
                      SchemaTypeChild {
                        name: "t:CT_C/t:c".to_string(),
                        property_name: "LeafC".to_string(),
                        property_comments: "Leaf C".to_string(),
                        kind: SchemaTypeChildKind::Child,
                        ..Default::default()
                      },
                      SchemaTypeChild {
                        name: "t:CT_D/t:d".to_string(),
                        property_name: "LeafD".to_string(),
                        property_comments: "Leaf D".to_string(),
                        kind: SchemaTypeChildKind::Child,
                        ..Default::default()
                      },
                    ],
                    ..Default::default()
                  },
                ],
                ..Default::default()
              },
            ],
            ..Default::default()
          }],
          ..Default::default()
        },
      ],
      ..Default::default()
    };
    let context = CodegenContext::new(std::slice::from_ref(&schema));

    let generated = gen_schema(&schema, None, &context, false)
      .unwrap()
      .to_string();

    assert!(generated.contains("pub leaf_a : LeafA"));
    assert!(generated.contains("pub structured_holder_choice : Option < StructuredHolderChoice >"));
    assert!(generated.contains("pub enum StructuredHolderChoice"));
    assert!(generated.contains("Sequence {"));
    assert!(generated.contains("leaf_c : LeafC"));
    assert!(generated.contains("leaf_d : LeafD"));
    assert!(!generated.contains("pub struct StructuredHolderChoiceSequence2"));
  }

  #[test]
  fn generates_mixed_choice_children_from_codegen_ir() {
    let schema = Schema {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        SchemaType {
          name: "t:CT_A/t:a".to_string(),
          class_name: "LeafA".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_B/t:b".to_string(),
          class_name: "LeafB".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_C/t:c".to_string(),
          class_name: "LeafC".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_D/t:d".to_string(),
          class_name: "LeafD".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_Mixed/t:mixed".to_string(),
          class_name: "MixedHolder".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Composite,
          composite_kind: SchemaTypeCompositeKind::OneSequence,
          children: vec![
            SchemaTypeChild {
              name: "t:CT_A/t:a".to_string(),
              property_name: "LeafA".to_string(),
              property_comments: "Leaf A".to_string(),
              kind: SchemaTypeChildKind::Child,
              ..Default::default()
            },
            SchemaTypeChild {
              name: "choice_wrapper".to_string(),
              kind: SchemaTypeChildKind::Choice,
              children: vec![
                SchemaTypeChild {
                  name: "t:CT_B/t:b".to_string(),
                  property_name: "LeafB".to_string(),
                  property_comments: "Leaf B".to_string(),
                  kind: SchemaTypeChildKind::Child,
                  ..Default::default()
                },
                SchemaTypeChild {
                  name: "seq_variant".to_string(),
                  kind: SchemaTypeChildKind::Sequence,
                  children: vec![
                    SchemaTypeChild {
                      name: "t:CT_C/t:c".to_string(),
                      property_name: "LeafC".to_string(),
                      property_comments: "Leaf C".to_string(),
                      kind: SchemaTypeChildKind::Child,
                      ..Default::default()
                    },
                    SchemaTypeChild {
                      name: "t:CT_D/t:d".to_string(),
                      property_name: "LeafD".to_string(),
                      property_comments: "Leaf D".to_string(),
                      kind: SchemaTypeChildKind::Child,
                      ..Default::default()
                    },
                  ],
                  ..Default::default()
                },
              ],
              ..Default::default()
            },
            SchemaTypeChild {
              name: "t:CT_D/t:d".to_string(),
              property_name: "TrailingLeaf".to_string(),
              property_comments: "Trailing leaf".to_string(),
              kind: SchemaTypeChildKind::Child,
              ..Default::default()
            },
          ],
          ..Default::default()
        },
      ],
      ..Default::default()
    };
    let context = CodegenContext::new(std::slice::from_ref(&schema));

    let generated = gen_schema(&schema, None, &context, false)
      .unwrap()
      .to_string();

    assert!(generated.contains("pub leaf_a : LeafA"));
    assert!(generated.contains("pub mixed_holder_choice : Option < MixedHolderChoice >"));
    assert!(generated.contains("pub trailing_leaf : LeafD"));
    assert!(generated.contains("pub enum MixedHolderChoice"));
    assert!(generated.contains("B (LeafB)"));
    assert!(generated.contains("LeafC (LeafC)"));
    assert!(generated.contains("LeafD (LeafD)"));
  }

  #[test]
  fn generates_generic_children_fallback_from_codegen_ir() {
    let schema = Schema {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![
        SchemaType {
          name: "t:CT_A/t:a".to_string(),
          class_name: "LeafA".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_B/t:b".to_string(),
          class_name: "LeafB".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          ..Default::default()
        },
        SchemaType {
          name: "t:CT_Fallback/t:fb".to_string(),
          class_name: "FallbackHolder".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Composite,
          children: vec![SchemaTypeChild {
            name: "wrapper".to_string(),
            kind: SchemaTypeChildKind::Sequence,
            children: vec![SchemaTypeChild {
              name: "choice".to_string(),
              kind: SchemaTypeChildKind::Choice,
              children: vec![
                SchemaTypeChild {
                  name: "t:CT_A/t:a".to_string(),
                  property_name: "LeafA".to_string(),
                  property_comments: "Leaf A".to_string(),
                  kind: SchemaTypeChildKind::Child,
                  ..Default::default()
                },
                SchemaTypeChild {
                  name: "inner".to_string(),
                  kind: SchemaTypeChildKind::Sequence,
                  children: vec![SchemaTypeChild {
                    name: "t:CT_B/t:b".to_string(),
                    property_name: "LeafB".to_string(),
                    property_comments: "Leaf B".to_string(),
                    kind: SchemaTypeChildKind::Child,
                    ..Default::default()
                  }],
                  ..Default::default()
                },
              ],
              ..Default::default()
            }],
            ..Default::default()
          }],
          ..Default::default()
        },
      ],
      ..Default::default()
    };
    let context = CodegenContext::new(std::slice::from_ref(&schema));

    let generated = gen_schema(&schema, None, &context, false)
      .unwrap()
      .to_string();

    assert!(generated.contains("pub fallback_holder_choice : Option < FallbackHolderChoice >"));
    assert!(generated.contains("pub enum FallbackHolderChoice"));
    assert!(generated.contains("A (LeafA)"));
    assert!(generated.contains("LeafB (std :: boxed :: Box < LeafB >)"));
    assert!(!generated.contains("pub struct FallbackHolderChoiceSequence2"));
    assert!(!generated.contains("leaf_b"));
  }

  #[test]
  fn emits_attribute_validator_attrs_from_codegen_ir() {
    let attr = FieldDecl {
      rust_name: "creation_id".to_string(),
      docs: String::new(),
      version: "Office2016".to_string(),
      wire: FieldWireDecl::Attribute {
        qname: ":creationId".to_string(),
        bit: None,
        list: false,
        match_local_name: false,
        empty_as_none: false,
      },
      cardinality: Cardinality::Optional,
      type_ref: TypeRefDecl {
        rust_type: "StringValue".to_string(),
        module_path: Some("crate::simple_type".to_string()),
      },
      validators: vec![
        crate::sdk_code::codegen_ir::ValidatorDecl {
          version: String::new(),
          source_id: 0,
          union_id: None,
          kind: ValidatorKind::Pattern {
            regex: "[A-Z]+".to_string(),
          },
        },
        crate::sdk_code::codegen_ir::ValidatorDecl {
          version: String::new(),
          source_id: 1,
          union_id: None,
          kind: ValidatorKind::StringFormat {
            kind: crate::sdk_code::codegen_ir::StringFormatKind::Token,
          },
        },
        crate::sdk_code::codegen_ir::ValidatorDecl {
          version: String::new(),
          source_id: 2,
          union_id: None,
          kind: ValidatorKind::StringLength {
            min: Some(2),
            max: Some(8),
            exact: None,
            type_name: None,
          },
        },
        crate::sdk_code::codegen_ir::ValidatorDecl {
          version: String::new(),
          source_id: 3,
          union_id: None,
          kind: ValidatorKind::NumberRange {
            min: Some("0".to_string()),
            max: Some("10".to_string()),
            min_inclusive: true,
            max_inclusive: false,
          },
        },
        crate::sdk_code::codegen_ir::ValidatorDecl {
          version: String::new(),
          source_id: 4,
          union_id: None,
          kind: ValidatorKind::NumberSign {
            kind: crate::sdk_code::codegen_ir::NumberSignKind::NonNegative,
          },
        },
      ],
    };

    let generated = gen_attr_from_decl(
      &attr,
      VersionCfgContext::default(),
      &TypeContainmentGraph::default(),
    )
    .unwrap()
    .to_string();

    assert!(generated.contains("# [sdk (attr (qname = \":creationId\"))]"));
    assert!(!generated.contains("# [doc"));
    assert!(generated.contains("# [sdk (pattern (regex = \"[A-Z]+\"))]"));
    assert!(generated.contains("# [sdk (string_format (kind = \"token\"))]"));
    assert!(generated.contains("# [sdk (string_length (min = 2u32 , max = 8u32 ,))]"));
    assert!(generated.contains("# [sdk (number_range (range = 0 .. 10 ,))]"));
    assert!(generated.contains("# [sdk (number_sign (kind = \"non_negative\"))]"));
  }

  #[test]
  fn generates_sequence_any_only_without_source_composite_kind() {
    let schema = Schema {
      module_name: "test_module".to_string(),
      target_namespace: "urn:test".to_string(),
      prefix: "t".to_string(),
      typed_namespace: "Test.Namespace".to_string(),
      types: vec![SchemaType {
        name: "t:CT_Any/t:any".to_string(),
        class_name: "AnyHolder".to_string(),
        kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Composite,
        children: vec![SchemaTypeChild {
          kind: SchemaTypeChildKind::Any,
          property_name: "UnknownXml".to_string(),
          ..Default::default()
        }],
        ..Default::default()
      }],
      ..Default::default()
    };
    let context = CodegenContext::new(std::slice::from_ref(&schema));

    let generated = gen_schema(&schema, None, &context, false)
      .unwrap()
      .to_string();

    assert!(generated.contains("pub type AnyHolder = Vec < String >"));
    assert!(
      !generated.contains("# [sdk (any)] pub xml_children : Vec < std :: boxed :: Box < [u8] > >")
    );
    assert!(!generated.contains("pub enum AnyHolderChoice"));
    assert!(!generated.contains("UnknownXml (String)"));
  }
}
