use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Group, Ident as TokenIdent, Span, TokenStream, TokenTree};
use quote::quote;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use syn::{Attribute, Ident, Type, Variant, parse_str, parse2};

use crate::Result;
use crate::sdk_code::codegen_ir::{
  Cardinality, ContentModelDecl, ElementKind, EnumDecl, FieldDecl, FieldWireDecl, MemberDecl,
  SchemaModuleDecl, SystemSupportDecl, TypeDecl, TypeKind, TypeRefDecl, ValidatorKind, VariantDecl,
  VariantWireDecl, XmlHeaderMode,
};
use crate::sdk_code::codegen_ir_builder::build_codegen_ir;
use crate::sdk_code::helpers::{
  AttrTypeKind, StructuredChoice, StructuredChoiceVariant, StructuredParticleKind,
  classify_attr_type,
};
use crate::sdk_code::versioning::{common_choice_version, version_cfg_attrs};
use crate::sdk_data::sdk_data_model::{
  Schema, SchemaEnum, SchemaType, SchemaTypeChild, SchemaTypeChildKind,
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
  enum_module_by_typed_namespace_and_name: HashMap<(&'a str, &'a str), &'a str>,
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

fn sdk_version_marker(version: &str) -> Option<TokenStream> {
  if version.is_empty() || version == "Office2007" {
    return None;
  }

  let marker = TokenIdent::new(&version.to_snake_case(), Span::call_site());
  Some(quote! { #marker })
}

fn sdk_version_markers(version: &str) -> Vec<TokenStream> {
  sdk_version_marker(version).into_iter().collect()
}

#[derive(Debug)]
pub struct ResolvedOneSequenceChild<'a> {
  pub name: &'a str,
  pub field_name: Cow<'a, str>,
  pub property_comments: Cow<'a, str>,
  pub version: &'a str,
  pub kind: SchemaTypeChildKind,
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
  empty_leaf_marker_ambiguous_rust_names: HashSet<String>,
  empty_leaf_marker_docs: HashMap<String, String>,
  empty_leaf_marker_docs_by_rust_name: HashMap<String, String>,
  any_children_alias_keys: HashSet<String>,
  nodes: HashSet<String>,
}

impl TypeContainmentGraph {
  pub(crate) fn from_modules(modules: &[&SchemaModuleDecl]) -> Self {
    let mut graph = Self::default();

    for module in modules {
      for type_decl in &module.types {
        let type_key = local_type_key(module, &type_decl.rust_name);
        graph.nodes.insert(type_key.clone());
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

    for module in modules {
      for type_decl in &module.types {
        let owner_key = local_type_key(module, &type_decl.rust_name);
        let owner_edges = graph.edges.entry(owner_key).or_default();

        for member in &type_decl.members {
          match member {
            MemberDecl::Field(field) => {
              let referenced_target = match &field.wire {
                FieldWireDecl::Child { .. } | FieldWireDecl::Any | FieldWireDecl::Choice => {
                  schema_type_key_from_ref(module, &field.type_ref)
                }
                FieldWireDecl::TextChild { .. } => {
                  if is_value_like_type_ref(module, &field.type_ref) {
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
                crate::sdk_code::codegen_ir::VariantWireDecl::Any
                | crate::sdk_code::codegen_ir::VariantWireDecl::Text => {
                  schema_type_key_from_ref(module, &variant.payload)
                }
                crate::sdk_code::codegen_ir::VariantWireDecl::TextChild { .. } => {
                  if is_value_like_type_ref(module, &variant.payload) {
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
      }
    }

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

  fn is_any_children_alias(&self, node: &str) -> bool {
    self.any_children_alias_keys.contains(node)
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

impl<'a> CodegenContext<'a> {
  pub fn new(schemas: &'a [Schema]) -> Self {
    let mut enum_type_map = HashMap::new();
    let mut enum_type_module_map = HashMap::new();
    let mut type_map = HashMap::new();
    let mut type_class_name_map = HashMap::new();
    let mut type_module_map = HashMap::new();
    let mut type_prefix_map = HashMap::new();
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
      enum_module_by_typed_namespace_and_name,
    }
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
        let field_name = if child.property_name.is_empty() {
          Cow::Borrowed("unknown_xml")
        } else {
          Cow::Owned(schema_child_field_rust_name(child.property_name.as_str()))
        };
        let property_comments = if child.property_comments.is_empty() {
          Cow::Borrowed(" _")
        } else {
          Cow::Borrowed(child.property_comments.as_str())
        };

        return Ok(ResolvedOneSequenceChild {
          name: child.name.as_str(),
          field_name,
          property_comments,
          version: child.initial_version.as_str(),
          kind: child.kind,
        });
      }

      let child_type = self.type_by_name(child.name.as_str()).ok_or_else(|| {
        format!(
          "missing direct one-sequence child type for schema {} child {:?} kind {:?}",
          schema_type.name, child.name, child.kind
        )
      })?;
      let field_name = Cow::Owned(child_field_name(child, child_type));
      let property_comments = if child.property_comments.is_empty() {
        Cow::Borrowed(" _")
      } else {
        Cow::Borrowed(child.property_comments.as_str())
      };

      return Ok(ResolvedOneSequenceChild {
        name: child.name.as_str(),
        field_name,
        property_comments,
        version: schema_item_version(child_type),
        kind: child.kind,
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
      field_name: Cow::Owned(escape_snake_case(child_type.class_name.to_snake_case())),
      property_comments: if child_type.summary.is_empty() {
        Cow::Borrowed(" _")
      } else {
        Cow::Owned(child_type.summary.clone())
      },
      version: schema_item_version(child_type),
      kind: child_kind_for_schema_type(child_type),
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
          field_name: Cow::Borrowed("Text"),
          property_comments: Cow::Borrowed(" _"),
          version: child.initial_version.as_str(),
          kind: child.kind,
        });
      } else if child.kind == SchemaTypeChildKind::Any {
        variants.push(ResolvedOneSequenceChild {
          name: "",
          field_name: if child.property_name.is_empty() {
            Cow::Borrowed("unknown_xml")
          } else {
            Cow::Owned(schema_child_field_rust_name(child.property_name.as_str()))
          },
          property_comments: if child.property_comments.is_empty() {
            Cow::Borrowed(" _")
          } else {
            Cow::Borrowed(child.property_comments.as_str())
          },
          version: child.initial_version.as_str(),
          kind: child.kind,
        });
      } else {
        let child_type = self.type_by_name(child.name.as_str()).ok_or_else(|| {
          format!(
            "missing direct one-sequence child type for schema {} child {:?} kind {:?}",
            schema_type.name, child.name, child.kind
          )
        })?;
        variants.push(ResolvedOneSequenceChild {
          name: child.name.as_str(),
          field_name: Cow::Owned(child_field_name(child, child_type)),
          property_comments: if child.property_comments.is_empty() {
            Cow::Borrowed(" _")
          } else {
            Cow::Borrowed(child.property_comments.as_str())
          },
          version: schema_item_version(child_type),
          kind: child.kind,
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
          property_comment_parts.push(
            child
              .name
              .split('/')
              .nth(1)
              .unwrap_or(child.name)
              .to_string(),
          );
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
            field_parts.push(
              child
                .name
                .split('/')
                .nth(1)
                .unwrap_or(child.name)
                .to_string(),
            );
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

      variants.push(self.resolve_one_sequence_child(schema_type, child.name.as_str())?);
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
        variant_name: Cow::Borrowed(if child.property_name.is_empty() {
          "Text"
        } else {
          child.property_name.as_str()
        }),
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
        Cow::Borrowed(if child.property_name.is_empty() {
          "UnknownXml"
        } else {
          child.property_name.as_str()
        }),
        true,
      )
    } else {
      let child_last_name = child
        .name
        .split('/')
        .nth(1)
        .ok_or_else(|| child.name.clone())?;
      (Cow::Borrowed(child_last_name), false)
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
    .filter(|ty| !ty.is_abstract)
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
    .filter(|ty| !ty.is_abstract)
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

    let struct_name_ident: Ident = parse_str(&type_decl.rust_name.to_upper_camel_case())?;
    let schema_type_version = type_decl.version.as_deref().unwrap_or_default();
    let type_attrs = version_cfg.attrs(schema_type_version);
    let field_version_cfg = if type_attrs.is_empty() {
      version_cfg
    } else {
      VersionCfgContext::new(true)
    };
    let type_sdk_version_markers = sdk_version_markers(schema_type_version);
    let sdk_type_attrs = if let Some(qname) = &type_decl.xml_qname {
      quote! {
        #[sdk(#(#type_sdk_version_markers,)* qname = #qname)]
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
        fields.push(gen_attr_from_decl(attr, field_version_cfg).map_err(|err| {
          format!(
            "type {} attr {}: {err}",
            type_decl.rust_name, attr.rust_name
          )
        })?);
      }

      fields.push(quote! {
        #[sdk(text)]
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

    let mut fields: Vec<TokenStream> = vec![];
    let items: Vec<TokenStream> = vec![];

    fields.extend(gen_support_fields(&type_decl.support));

    if type_decl.element_kind == Some(ElementKind::LeafText) {
      for attr in &attr_fields {
        fields.push(gen_attr_from_decl(attr, field_version_cfg).map_err(|err| {
          format!(
            "type {} attr {}: {err}",
            type_decl.rust_name, attr.rust_name
          )
        })?);
      }

      let simple_type_name = type_from_decl_ref(
        type_decl
          .xml_content
          .as_ref()
          .ok_or_else(|| format!("type {} missing IR xml content", type_decl.rust_name))?,
      )
      .map_err(|err| format!("type {} xml content: {err}", type_decl.rust_name))?;
      fields.push(quote! {
        #[sdk(text)]
        pub xml_content: Option<#simple_type_name>,
      });
    } else if type_decl.element_kind == Some(ElementKind::Leaf) {
      for attr in &attr_fields {
        fields.push(gen_attr_from_decl(attr, field_version_cfg)?);
      }
    } else if type_decl.element_kind == Some(ElementKind::Composite) {
      for attr in &attr_fields {
        fields.push(gen_attr_from_decl(attr, field_version_cfg)?);
      }

      if let Some(xml_content) = type_decl.xml_content.as_ref() {
        let simple_type_name = type_from_decl_ref(xml_content)
          .map_err(|err| format!("type {} xml content: {err}", type_decl.rust_name))?;
        fields.push(quote! {
          #[sdk(text)]
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
          let field_type = any_field
            .map(|field| type_from_decl_ref(&field.type_ref))
            .transpose()?
            .unwrap_or_else(|| parse_str("String").expect("String type"));
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
        fields.push(gen_attr_from_decl(attr, field_version_cfg)?);
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

        fields.push(gen_attr_from_decl(attr, field_version_cfg)?);

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
          let field_type = any_field
            .map(|field| type_from_decl_ref(&field.type_ref))
            .transpose()?
            .unwrap_or_else(|| parse_str("String").expect("String type"));
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
        )?;
        fields.push(quote! {
          #[sdk(text)]
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
  let enum_ident: Ident = parse_str(&type_decl.rust_name.to_upper_camel_case())?;
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

  if choice_enum_is_pure_choice_wrapper(type_decl, module) {
    let wrapper_payloads = pure_choice_wrapper_payload_types(type_decl, module);
    let reserved_variant_names = HashSet::new();
    let helper_variants_are_unique = wrapper_payloads.iter().all(|helper_type_decl| {
      choice_helper_names_are_inline_safe(helper_type_decl, module, &reserved_variant_names)
    });

    if helper_variants_are_unique {
      let mut variants = Vec::new();
      let mut reserved_variant_names = HashSet::new();

      for member in &type_decl.members {
        let MemberDecl::Variant(variant) = member else {
          continue;
        };
        let Some(helper_type_decl) = pure_choice_wrapper_helper_type_decl(variant, module) else {
          continue;
        };

        let helper_type_version = helper_type_decl.version.as_deref().unwrap_or_default();
        let helper_variant_versions: Vec<&str> = helper_type_decl
          .members
          .iter()
          .filter_map(|member| match member {
            MemberDecl::Variant(variant) => Some(variant.version.as_str()),
            _ => None,
          })
          .collect();
        let helper_enum_version =
          common_choice_version(helper_type_version, &helper_variant_versions);
        let helper_enum_attrs = module_version_cfg_attrs(helper_enum_version, variant_cfg);
        let wrapper_variant_attrs = module_version_cfg_attrs(&variant.version, variant_cfg);
        let inline_prefix_attrs = quote! {
          #( #wrapper_variant_attrs )*
          #( #helper_enum_attrs )*
        };

        for helper_variant in helper_type_decl
          .members
          .iter()
          .filter_map(|member| match member {
            MemberDecl::Variant(variant) => Some(variant),
            _ => None,
          })
        {
          variants.extend(gen_choice_variant_tokens(
            helper_variant,
            &render_context,
            true,
            inline_prefix_attrs.clone(),
            &mut reserved_variant_names,
          )?);
          for helper_variant_name in choice_variant_rendered_names(helper_variant, module)
            .expect("pure choice wrapper variant rendered names")
          {
            reserved_variant_names.insert(helper_variant_name);
          }
        }
      }

      return Ok(quote! {
        #( #enum_attrs )*
        #[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
        pub enum #enum_ident {
          #( #variants )*
        }
      });
    }
  }

  let mut reserved_variant_names = inline_choice_helper_reserved_variant_names(type_decl, module);

  for member in &type_decl.members {
    let MemberDecl::Variant(variant) = member else {
      continue;
    };
    variants.extend(gen_choice_variant_tokens(
      variant,
      &render_context,
      true,
      quote! {},
      &mut reserved_variant_names,
    )?);
  }

  Ok(quote! {
    #( #enum_attrs )*
    #[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
    pub enum #enum_ident {
      #( #variants )*
    }
  })
}

fn helper_struct_is_inline_sequence_candidate(type_decl: &TypeDecl) -> bool {
  type_decl.kind == TypeKind::HelperStruct
    && matches!(type_decl.members.len(), 1 | 2)
    && type_decl.members.iter().all(|member| {
      matches!(
        member,
        MemberDecl::Field(FieldDecl {
          wire: FieldWireDecl::Child { .. } | FieldWireDecl::TextChild { .. },
          ..
        })
      )
    })
}

fn inline_sequence_field_forces_box(field: &FieldDecl) -> bool {
  matches!(
    (
      field.rust_name.as_str(),
      field.type_ref.rust_type.as_str(),
      field.cardinality,
      &field.wire,
    ),
    (
      "run_conflict_insertion",
      "RunConflictInsertion",
      Cardinality::Optional,
      FieldWireDecl::Child { .. },
    ) | (
      "run_conflict_deletion",
      "RunConflictDeletion",
      Cardinality::Optional,
      FieldWireDecl::Child { .. },
    )
  )
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
    && type_decl.support.xml_header == XmlHeaderMode::None
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
    && type_decl.support.xml_header == XmlHeaderMode::None
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
  let doc_text = if prefer_payload_docs {
    empty_leaf_marker_doc_for_ref(module, &variant.payload, type_graph)
      .and_then(meaningful_doc_text)
      .or_else(|| {
        type_decl_for_ref(module, &variant.payload)
          .and_then(|type_decl| meaningful_doc_text(&type_decl.docs))
      })
      .or_else(|| meaningful_doc_text(&variant.docs))
  } else {
    meaningful_doc_text(&variant.docs)
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

fn helper_struct_is_inline_sequence_clippy_safe(
  type_decl: &TypeDecl,
  module: &SchemaModuleDecl,
  type_graph: &TypeContainmentGraph,
) -> bool {
  type_decl.members.iter().all(|member| {
    let MemberDecl::Field(field) = member else {
      return false;
    };

    match &field.wire {
      FieldWireDecl::TextChild { .. } => {
        !matches!(field.cardinality, Cardinality::Many)
          && is_value_like_type_ref(module, &field.type_ref)
      }
      FieldWireDecl::Child { .. } => {
        !matches!(field.cardinality, Cardinality::Many)
          && (inline_sequence_field_forces_box(field)
            || is_value_like_type_ref(module, &field.type_ref)
            || direct_child_field_needs_box(&type_decl.rust_name, field, module, type_graph, false))
      }
      _ => false,
    }
  })
}

const MAX_NAMED_CHOICE_INLINE_VARIANTS_THROUGH_ANONYMOUS_WRAPPER: usize = 12;

fn inline_single_field_sequence_variant_tokens(
  module: &SchemaModuleDecl,
  type_graph: &TypeContainmentGraph,
  variant_ident: &Ident,
  variant_attrs: &[Attribute],
  field: &FieldDecl,
) -> Result<Option<TokenStream>> {
  if !matches!(field.cardinality, Cardinality::One | Cardinality::Optional) {
    return Ok(None);
  }

  let payload_type = type_from_decl_ref(&field.type_ref)?;
  let empty_leaf_marker_doc = empty_leaf_marker_doc_for_ref(module, &field.type_ref, type_graph);
  let empty_leaf_doc_attrs = if let Some(doc) = empty_leaf_marker_doc.and_then(meaningful_doc_text)
  {
    quote! { #[doc = #doc] }
  } else {
    quote! {}
  };
  let field_sdk_version_markers = sdk_version_markers(&field.version);

  match &field.wire {
    FieldWireDecl::Child { qname } => {
      if empty_leaf_marker_doc.is_some() {
        Ok(Some(quote! {
          #( #variant_attrs )*
          #empty_leaf_doc_attrs
          #[sdk(empty_child(#(#field_sdk_version_markers,)* qname = #qname))]
          #variant_ident,
        }))
      } else {
        Ok(Some(quote! {
          #( #variant_attrs )*
          #[sdk(child(#(#field_sdk_version_markers,)* qname = #qname))]
          #variant_ident(std::boxed::Box<#payload_type>),
        }))
      }
    }
    FieldWireDecl::TextChild { qname } => Ok(Some(quote! {
      #( #variant_attrs )*
      #[sdk(text_child(#(#field_sdk_version_markers,)* qname = #qname))]
      #variant_ident(#payload_type),
    })),
    _ => Ok(None),
  }
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
  flatten_anonymous_choice_wrappers: bool,
  prefix_attrs: TokenStream,
  reserved_variant_names: &mut HashSet<String>,
) -> Result<Vec<TokenStream>> {
  let rendered_variant_name = render_context
    .rendered_variant_name_map
    .get(&variant.rust_name)
    .map(String::as_str)
    .unwrap_or(&variant.rust_name);
  let variant_ident: Ident = parse_str(rendered_variant_name)?;
  let variant_attrs = module_version_cfg_attrs(&variant.version, render_context.variant_cfg);
  let variant_sdk_version_markers = sdk_version_markers(&variant.version);
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
    crate::sdk_code::codegen_ir::VariantWireDecl::Any => {
      let payload_type = type_from_decl_ref(&variant.payload)?;
      Ok(vec![quote! {
        #prefix_attrs
        #( #variant_attrs )*
        #variant_doc_attrs
        #[sdk(any)]
        #variant_ident(#payload_type),
      }])
    }
    crate::sdk_code::codegen_ir::VariantWireDecl::Sequence { .. } => {
      if let Some(helper_type_decl) =
        inline_sequence_helper_type_decl(variant, render_context.module)
      {
        let helper_fields: Vec<&FieldDecl> = helper_type_decl
          .members
          .iter()
          .filter_map(|member| match member {
            MemberDecl::Field(field) => Some(field),
            _ => None,
          })
          .collect();
        if helper_fields.len() == 1
          && let Some(single_field_tokens) = inline_single_field_sequence_variant_tokens(
            render_context.module,
            render_context.type_graph,
            &variant_ident,
            &variant_attrs,
            helper_fields[0],
          )?
        {
          return Ok(vec![quote! {
            #prefix_attrs
            #single_field_tokens
          }]);
        }
        if !helper_struct_is_inline_sequence_clippy_safe(
          helper_type_decl,
          render_context.module,
          render_context.type_graph,
        ) {
          let payload_type = type_from_decl_ref(&variant.payload)?;
          return Ok(vec![quote! {
            #prefix_attrs
            #( #variant_attrs )*
            #variant_doc_attrs
            #[sdk(sequence)]
            #variant_ident(std::boxed::Box<#payload_type>),
          }]);
        }
        let inline_fields = gen_inline_sequence_variant_fields_from_decl(
          &helper_fields,
          &helper_type_decl.rust_name,
          render_context.module,
          render_context.type_graph,
          VersionCfgContext::new(true),
        )?;
        Ok(vec![quote! {
          #prefix_attrs
          #( #variant_attrs )*
          #variant_doc_attrs
          #[sdk(sequence)]
          #variant_ident {
            #( #inline_fields )*
          },
        }])
      } else {
        let payload_type = type_from_decl_ref(&variant.payload)?;
        Ok(vec![quote! {
          #prefix_attrs
          #( #variant_attrs )*
          #variant_doc_attrs
          #[sdk(sequence)]
          #variant_ident(std::boxed::Box<#payload_type>),
        }])
      }
    }
    crate::sdk_code::codegen_ir::VariantWireDecl::TextChild { qnames } => {
      let payload_type = type_from_decl_ref(&variant.payload)?;
      if qnames.is_empty() {
        return Err(variant.rust_name.clone().into());
      }
      let qname_attrs = qnames
        .iter()
        .map(
          |qname| quote! { #[sdk(text_child(#(#variant_sdk_version_markers,)* qname = #qname))] },
        )
        .collect::<Vec<_>>();
      Ok(vec![quote! {
        #prefix_attrs
        #( #variant_attrs )*
        #variant_doc_attrs
        #( #qname_attrs )*
        #variant_ident(#payload_type),
      }])
    }
    crate::sdk_code::codegen_ir::VariantWireDecl::Child { qnames } => {
      if flatten_anonymous_choice_wrappers
        && let Some(helper_type_decl) =
          anonymous_choice_wrapper_helper_type_decl(variant, render_context.module)
      {
        let helper_type_version = helper_type_decl.version.as_deref().unwrap_or_default();
        let helper_variant_versions: Vec<&str> = helper_type_decl
          .members
          .iter()
          .filter_map(|member| match member {
            MemberDecl::Variant(variant) => Some(variant.version.as_str()),
            _ => None,
          })
          .collect();
        let helper_enum_version =
          common_choice_version(helper_type_version, &helper_variant_versions);
        let helper_enum_attrs =
          module_version_cfg_attrs(helper_enum_version, render_context.variant_cfg);
        if choice_helper_names_are_inline_safe(
          helper_type_decl,
          render_context.module,
          reserved_variant_names,
        ) {
          let helper_variants: Vec<&VariantDecl> = helper_type_decl
            .members
            .iter()
            .filter_map(|member| match member {
              MemberDecl::Variant(variant) => Some(variant),
              _ => None,
            })
            .collect();

          let helper_prefix_attrs = quote! {
            #prefix_attrs
            #( #variant_attrs )*
            #( #helper_enum_attrs )*
          };
          let mut tokens = Vec::new();
          for helper_variant in &helper_variants {
            tokens.extend(gen_choice_variant_tokens(
              helper_variant,
              render_context,
              true,
              helper_prefix_attrs.clone(),
              reserved_variant_names,
            )?);
            for helper_variant_name in
              choice_variant_rendered_names(helper_variant, render_context.module)
                .expect("choice helper variant rendered names")
            {
              reserved_variant_names.insert(helper_variant_name);
            }
          }
          Ok(tokens)
        } else {
          let payload_type = type_from_decl_ref(&variant.payload)?;
          Ok(vec![quote! {
            #prefix_attrs
            #( #variant_attrs )*
            #variant_doc_attrs
            #[sdk(choice)]
            #variant_ident(std::boxed::Box<#payload_type>),
          }])
        }
      } else {
        let payload_type = type_from_decl_ref(&variant.payload)?;
        let is_nested_choice = render_context
          .module
          .types
          .iter()
          .any(|ty| ty.rust_name == variant.payload.rust_type && ty.kind == TypeKind::ChoiceEnum);
        if is_nested_choice {
          Ok(vec![quote! {
            #prefix_attrs
            #( #variant_attrs )*
            #variant_doc_attrs
            #[sdk(choice)]
            #variant_ident(std::boxed::Box<#payload_type>),
          }])
        } else {
          if qnames.is_empty() {
            return Err(variant.rust_name.clone().into());
          }
          if is_empty_leaf_marker_ref_with_graph(
            render_context.module,
            &variant.payload,
            render_context.type_graph,
          ) {
            let qname_attrs = qnames
              .iter()
              .map(|qname| quote! { #[sdk(empty_child(#(#variant_sdk_version_markers,)* qname = #qname))] })
              .collect::<Vec<_>>();
            Ok(vec![quote! {
              #prefix_attrs
              #( #variant_attrs )*
              #empty_leaf_variant_doc_attrs
              #( #qname_attrs )*
              #variant_ident,
            }])
          } else {
            let is_any_children_alias = is_any_children_alias_type_ref(
              render_context.module,
              &variant.payload,
              render_context.type_graph,
            );
            let qname_attrs = qnames
              .iter()
              .map(|qname| {
                if is_any_children_alias {
                  quote! { #[sdk(any_child(#(#variant_sdk_version_markers,)* qname = #qname))] }
                } else {
                  quote! { #[sdk(child(#(#variant_sdk_version_markers,)* qname = #qname))] }
                }
              })
              .collect::<Vec<_>>();
            let payload_tokens = if is_any_children_alias {
              quote! { #payload_type }
            } else {
              quote! { std::boxed::Box<#payload_type> }
            };
            Ok(vec![quote! {
              #prefix_attrs
              #( #variant_attrs )*
              #variant_doc_attrs
              #( #qname_attrs )*
              #variant_ident(#payload_tokens),
            }])
          }
        }
      }
    }
    crate::sdk_code::codegen_ir::VariantWireDecl::Text => {
      let payload_type = type_from_decl_ref(&variant.payload)?;
      Ok(vec![quote! {
        #prefix_attrs
        #( #variant_attrs )*
        #variant_doc_attrs
        #[sdk(text)]
        #variant_ident(#payload_type),
      }])
    }
  }
}

fn inline_sequence_helper_type_decl<'a>(
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

  module
    .types
    .iter()
    .find(|type_decl| type_decl.rust_name == variant.payload.rust_type)
    .filter(|type_decl| helper_struct_is_inline_sequence_candidate(type_decl))
}

fn choice_enum_is_inline_candidate(type_decl: &TypeDecl, module: &SchemaModuleDecl) -> bool {
  type_decl.kind == TypeKind::ChoiceEnum
    && type_decl.members.iter().all(|member| match member {
      MemberDecl::Variant(VariantDecl {
        wire:
          crate::sdk_code::codegen_ir::VariantWireDecl::Child { .. }
          | crate::sdk_code::codegen_ir::VariantWireDecl::TextChild { .. },
        payload,
        ..
      }) => !module.types.iter().any(|type_decl| {
        type_decl.rust_name == payload.rust_type && type_decl.kind == TypeKind::ChoiceEnum
      }),
      _ => false,
    })
}

fn choice_enum_effective_flattened_variant_names(
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

      match &variant.wire {
        crate::sdk_code::codegen_ir::VariantWireDecl::Child { .. }
        | crate::sdk_code::codegen_ir::VariantWireDecl::TextChild { .. } => {
          let nested_choice = module.types.iter().find(|nested_type_decl| {
            nested_type_decl.rust_name == variant.payload.rust_type
              && nested_type_decl.kind == TypeKind::ChoiceEnum
          });

          if let Some(nested_choice) = nested_choice {
            if !is_generated_anonymous_type_name(&variant.rust_name) {
              visiting.remove(&type_decl.rust_name);
              return None;
            }
            collect(nested_choice, module, visiting, names)?;
          } else {
            names.push(variant.rust_name.clone());
          }
        }
        _ => {
          visiting.remove(&type_decl.rust_name);
          return None;
        }
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

fn local_choice_payload_type_decl<'a>(
  variant: &VariantDecl,
  module: &'a SchemaModuleDecl,
) -> Option<&'a TypeDecl> {
  if variant.payload.module_path.is_some() {
    return None;
  }

  module.types.iter().find(|type_decl| {
    type_decl.rust_name == variant.payload.rust_type && type_decl.kind == TypeKind::ChoiceEnum
  })
}

fn should_flatten_nested_choice_variant(
  variant: &VariantDecl,
  nested_choice: &TypeDecl,
  module: &SchemaModuleDecl,
) -> bool {
  if is_generated_anonymous_type_name(&variant.rust_name) {
    if is_generated_anonymous_type_name(&nested_choice.rust_name) {
      true
    } else {
      choice_enum_effective_flattened_variant_names(nested_choice, module).is_some_and(|names| {
        names.len() <= MAX_NAMED_CHOICE_INLINE_VARIANTS_THROUGH_ANONYMOUS_WRAPPER
          && names.iter().collect::<HashSet<_>>().len() == names.len()
      })
    }
  } else {
    choice_enum_is_inline_candidate(nested_choice, module)
  }
}

fn choice_variant_rendered_names(
  variant: &VariantDecl,
  module: &SchemaModuleDecl,
) -> Option<Vec<String>> {
  match &variant.wire {
    crate::sdk_code::codegen_ir::VariantWireDecl::Any
    | crate::sdk_code::codegen_ir::VariantWireDecl::Sequence { .. }
    | crate::sdk_code::codegen_ir::VariantWireDecl::Text
    | crate::sdk_code::codegen_ir::VariantWireDecl::TextChild { .. } => {
      Some(vec![variant.rust_name.clone()])
    }
    crate::sdk_code::codegen_ir::VariantWireDecl::Child { .. } => {
      if let Some(nested_choice) = local_choice_payload_type_decl(variant, module)
        && should_flatten_nested_choice_variant(variant, nested_choice, module)
      {
        choice_enum_rendered_variant_names(nested_choice, module)
      } else {
        Some(vec![variant.rust_name.clone()])
      }
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

fn choice_enum_is_small_named_inline_candidate(
  type_decl: &TypeDecl,
  module: &SchemaModuleDecl,
) -> bool {
  !is_generated_anonymous_type_name(&type_decl.rust_name)
    && choice_enum_effective_flattened_variant_names(type_decl, module).is_some_and(|names| {
      names.len() <= MAX_NAMED_CHOICE_INLINE_VARIANTS_THROUGH_ANONYMOUS_WRAPPER
        && names.iter().collect::<HashSet<_>>().len() == names.len()
    })
}

fn inline_choice_helper_type_decl<'a>(
  variant: &VariantDecl,
  module: &'a SchemaModuleDecl,
) -> Option<&'a TypeDecl> {
  if !matches!(
    variant.wire,
    crate::sdk_code::codegen_ir::VariantWireDecl::Child { .. }
  ) {
    return None;
  }

  if variant.payload.module_path.is_some() {
    return None;
  }

  module
    .types
    .iter()
    .find(|type_decl| type_decl.rust_name == variant.payload.rust_type)
    .filter(|type_decl| choice_enum_is_inline_candidate(type_decl, module))
}

fn anonymous_choice_wrapper_helper_type_decl<'a>(
  variant: &VariantDecl,
  module: &'a SchemaModuleDecl,
) -> Option<&'a TypeDecl> {
  if !is_generated_anonymous_type_name(&variant.rust_name) {
    return inline_choice_helper_type_decl(variant, module);
  }

  if !matches!(
    variant.wire,
    crate::sdk_code::codegen_ir::VariantWireDecl::Child { .. }
  ) {
    return None;
  }

  if variant.payload.module_path.is_some() {
    return None;
  }

  module
    .types
    .iter()
    .find(|type_decl| type_decl.rust_name == variant.payload.rust_type)
    .filter(|type_decl| {
      type_decl.kind == TypeKind::ChoiceEnum
        && (is_generated_anonymous_type_name(&type_decl.rust_name)
          || choice_enum_is_small_named_inline_candidate(type_decl, module))
    })
}

fn pure_choice_wrapper_helper_type_decl<'a>(
  variant: &VariantDecl,
  module: &'a SchemaModuleDecl,
) -> Option<&'a TypeDecl> {
  if !matches!(
    variant.wire,
    crate::sdk_code::codegen_ir::VariantWireDecl::Child { .. }
  ) {
    return None;
  }

  if variant.payload.module_path.is_some() {
    return None;
  }

  module
    .types
    .iter()
    .find(|type_decl| type_decl.rust_name == variant.payload.rust_type)
    .filter(|type_decl| type_decl.kind == TypeKind::ChoiceEnum)
}

fn choice_enum_is_pure_choice_wrapper(type_decl: &TypeDecl, module: &SchemaModuleDecl) -> bool {
  type_decl.kind == TypeKind::ChoiceEnum
    && !type_decl.members.is_empty()
    && type_decl.members.iter().all(|member| match member {
      MemberDecl::Variant(variant) => {
        pure_choice_wrapper_helper_type_decl(variant, module).is_some()
      }
      _ => false,
    })
}

fn pure_choice_wrapper_payload_types<'a>(
  type_decl: &'a TypeDecl,
  module: &'a SchemaModuleDecl,
) -> Vec<&'a TypeDecl> {
  type_decl
    .members
    .iter()
    .filter_map(|member| match member {
      MemberDecl::Variant(variant) => pure_choice_wrapper_helper_type_decl(variant, module),
      _ => None,
    })
    .collect()
}

fn inline_choice_helper_reserved_variant_names(
  type_decl: &TypeDecl,
  module: &SchemaModuleDecl,
) -> HashSet<String> {
  let mut reserved = HashSet::new();

  for member in &type_decl.members {
    let MemberDecl::Variant(variant) = member else {
      continue;
    };

    if anonymous_choice_wrapper_helper_type_decl(variant, module).is_none() {
      reserved.insert(variant.rust_name.clone());
    }
  }

  reserved
}

fn choice_helper_names_are_inline_safe(
  helper_type_decl: &TypeDecl,
  module: &SchemaModuleDecl,
  reserved_variant_names: &HashSet<String>,
) -> bool {
  choice_enum_rendered_variant_names(helper_type_decl, module).is_some_and(|helper_names| {
    helper_names
      .iter()
      .all(|helper_name| !reserved_variant_names.contains(helper_name))
      && helper_names.iter().collect::<HashSet<_>>().len() == helper_names.len()
  })
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

fn gen_attr_from_decl(attr: &FieldDecl, version_cfg: VersionCfgContext) -> Result<TokenStream> {
  let FieldWireDecl::Attribute { qname, bit } = &attr.wire else {
    return Err(format!("expected attribute field, got {:?}", attr.wire).into());
  };
  let attr_name_ident: Ident = parse_str(&attr.rust_name)?;
  let type_ident = type_from_decl_ref(&attr.type_ref)?;
  let bit_attrs = if let Some(bit) = bit {
    quote! {
      #[sdk(bit = #bit)]
    }
  } else {
    quote! {}
  };
  let attr_attrs = module_version_cfg_attrs(&attr.version, version_cfg);
  let attr_sdk_version_markers = sdk_version_markers(&attr.version);
  let sdk_attr_attrs = quote! {
    #[sdk(attr(#(#attr_sdk_version_markers,)* qname = #qname))]
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
  let property_comments_doc = format!(" {}", attr.docs);

  Ok(match attr.cardinality {
    Cardinality::One => quote! {
      #( #attr_attrs )*
      #[doc = #property_comments_doc]
      #sdk_attr_attrs
      #( #validator_attrs )*
      #bit_attrs
      pub #attr_name_ident: #type_ident,
    },
    Cardinality::Optional => quote! {
      #( #attr_attrs )*
      #[doc = #property_comments_doc]
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

fn type_from_decl_ref(type_ref: &TypeRefDecl) -> Result<Type> {
  if type_ref.rust_type.contains('<') || type_ref.rust_type.contains("::") {
    if let Some(module_path) = &type_ref.module_path {
      return Ok(parse_str(&format!(
        "{module_path}::{}",
        type_ref.rust_type
      ))?);
    }

    return Ok(parse_str(&type_ref.rust_type)?);
  }

  if let Some(module_path) = &type_ref.module_path {
    Ok(parse_str(&format!(
      "{module_path}::{}",
      type_ref.rust_type.to_upper_camel_case()
    ))?)
  } else {
    Ok(parse_str(&type_ref.rust_type.to_upper_camel_case())?)
  }
}

fn is_value_like_type_ref(module: &SchemaModuleDecl, type_ref: &TypeRefDecl) -> bool {
  if matches!(type_ref.module_path.as_deref(), Some("crate::simple_type")) {
    return true;
  }

  if type_ref.module_path.is_some() {
    return false;
  }

  if module
    .enums
    .iter()
    .any(|schema_enum| schema_enum.rust_name == type_ref.rust_type)
  {
    return true;
  }

  module.types.iter().any(|type_decl| {
    type_decl.rust_name == type_ref.rust_type && type_decl.kind == TypeKind::LeafTextAlias
  })
}

fn module_type_namespace(module_name: &str) -> String {
  format!("crate::schemas::{module_name}")
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
    FieldWireDecl::TextChild { .. } if !is_value_like_type_ref(module, &field.type_ref) => {}
    FieldWireDecl::TextChild { .. } => return false,
    _ => return false,
  }

  if matches!(effective_cardinality, Cardinality::Many) {
    return false;
  }

  if matches!(effective_cardinality, Cardinality::One) {
    return true;
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
      pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
    });
  }

  if support.xml_header != crate::sdk_code::codegen_ir::XmlHeaderMode::None {
    fields.push(quote! {
      pub xml_header: crate::common::XmlHeaderType,
    });
  }

  if support.have_xml_other_attrs {
    fields.push(quote! {
      pub xml_other_attrs: Vec<(String, String)>,
    });
  }

  if support.have_xml_other_children {
    fields.push(quote! {
      pub xml_other_children: Vec<(usize, String)>,
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
            wire: VariantWireDecl::Any,
            ..
          }) => true,
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

fn choice_type_specific_start_qname_groups(
  module: &SchemaModuleDecl,
  rust_type: &str,
  version_cfg: VersionCfgContext,
) -> (Vec<String>, Vec<String>) {
  let Some(type_decl) = module
    .types
    .iter()
    .find(|type_decl| type_decl.rust_name == rust_type && type_decl.kind == TypeKind::ChoiceEnum)
  else {
    return (Vec::new(), Vec::new());
  };

  let mut unconditional_qnames = Vec::new();
  let feature_gated_qnames = Vec::new();
  for member in &type_decl.members {
    let MemberDecl::Variant(variant) = member else {
      continue;
    };
    let _ = version_cfg;
    let target_qnames = &mut unconditional_qnames;

    match &variant.wire {
      VariantWireDecl::Child {
        qnames: variant_qnames,
      }
      | VariantWireDecl::Sequence {
        qnames: variant_qnames,
      }
      | VariantWireDecl::TextChild {
        qnames: variant_qnames,
      } => {
        for qname in variant_qnames {
          if !target_qnames.contains(qname) {
            target_qnames.push(qname.clone());
          }
        }
      }
      VariantWireDecl::Any | VariantWireDecl::Text => {}
    }
  }

  (unconditional_qnames, feature_gated_qnames)
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

  for field in fields {
    let attr = module_version_cfg_attrs(&field.version, field_cfg);
    let field_name_ident: Ident = parse_str(&field.rust_name)?;
    let empty_leaf_marker_doc = empty_leaf_marker_doc_for_ref(module, &field.type_ref, type_graph);
    let field_type = type_from_decl_ref(&field.type_ref)?;
    let property_comments_owned = empty_leaf_marker_doc
      .and_then(meaningful_doc_text)
      .or_else(|| meaningful_doc_text(&field.docs))
      .unwrap_or_else(|| " _".to_string());
    let property_comments = property_comments_owned.as_str();
    let field_sdk_version_markers = sdk_version_markers(&field.version);
    let is_any_children_alias = is_any_children_alias_type_ref(module, &field.type_ref, type_graph);
    let sdk_field_attrs = match &field.wire {
      FieldWireDecl::Child { qname } if empty_leaf_marker_doc.is_some() => {
        quote! { #[sdk(empty_child(#(#field_sdk_version_markers,)* qname = #qname))] }
      }
      FieldWireDecl::Child { qname } if is_any_children_alias => {
        quote! { #[sdk(any_child(#(#field_sdk_version_markers,)* qname = #qname))] }
      }
      FieldWireDecl::Child { qname } => {
        quote! { #[sdk(child(#(#field_sdk_version_markers,)* qname = #qname))] }
      }
      FieldWireDecl::TextChild { qname } => {
        quote! { #[sdk(text_child(#(#field_sdk_version_markers,)* qname = #qname))] }
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

fn gen_inline_sequence_variant_fields_from_decl(
  fields: &[&FieldDecl],
  owner_rust_name: &str,
  module: &SchemaModuleDecl,
  type_graph: &TypeContainmentGraph,
  field_cfg: VersionCfgContext,
) -> Result<Vec<TokenStream>> {
  let mut tokens = Vec::new();

  for field in fields {
    let attr = module_version_cfg_attrs(&field.version, field_cfg);
    let field_name_ident: Ident = parse_str(&field.rust_name)?;
    let empty_leaf_marker_doc = empty_leaf_marker_doc_for_ref(module, &field.type_ref, type_graph);
    let field_type = type_from_decl_ref(&field.type_ref)?;
    let property_comments_owned = empty_leaf_marker_doc
      .and_then(meaningful_doc_text)
      .or_else(|| meaningful_doc_text(&field.docs))
      .unwrap_or_else(|| " _".to_string());
    let property_comments = property_comments_owned.as_str();
    let field_sdk_version_markers = sdk_version_markers(&field.version);
    let is_any_children_alias = is_any_children_alias_type_ref(module, &field.type_ref, type_graph);
    let sdk_field_attrs = match &field.wire {
      FieldWireDecl::Child { qname } if empty_leaf_marker_doc.is_some() => {
        quote! { #[sdk(empty_child(#(#field_sdk_version_markers,)* qname = #qname))] }
      }
      FieldWireDecl::Child { qname } if is_any_children_alias => {
        quote! { #[sdk(any_child(#(#field_sdk_version_markers,)* qname = #qname))] }
      }
      FieldWireDecl::Child { qname } => {
        quote! { #[sdk(child(#(#field_sdk_version_markers,)* qname = #qname))] }
      }
      FieldWireDecl::TextChild { qname } => {
        quote! { #[sdk(text_child(#(#field_sdk_version_markers,)* qname = #qname))] }
      }
      _ => {
        return Err(
          format!(
            "expected inline sequence direct child or text child field, got {:?}",
            field.wire
          )
          .into(),
        );
      }
    };
    let field_type = if empty_leaf_marker_doc.is_some() {
      parse_str("()")?
    } else {
      field_type
    };
    let wrap_box = empty_leaf_marker_doc.is_none()
      && !is_any_children_alias
      && (inline_sequence_field_forces_box(field)
        || direct_child_field_needs_box(owner_rust_name, field, module, type_graph, false));

    let field_tokens = match field.cardinality {
      Cardinality::Many => quote! {
        #( #attr )*
        #[doc = #property_comments]
        #sdk_field_attrs
        #field_name_ident: Vec<#field_type>,
      },
      Cardinality::Optional if wrap_box => quote! {
        #( #attr )*
        #[doc = #property_comments]
        #sdk_field_attrs
        #field_name_ident: Option<std::boxed::Box<#field_type>>,
      },
      Cardinality::Optional => quote! {
        #( #attr )*
        #[doc = #property_comments]
        #sdk_field_attrs
        #field_name_ident: Option<#field_type>,
      },
      Cardinality::One if wrap_box => quote! {
        #( #attr )*
        #[doc = #property_comments]
        #sdk_field_attrs
        #field_name_ident: std::boxed::Box<#field_type>,
      },
      Cardinality::One => quote! {
        #( #attr )*
        #[doc = #property_comments]
        #sdk_field_attrs
        #field_name_ident: #field_type,
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
          field_cfg,
          &HashSet::new(),
        )?);
      }
      FieldWireDecl::Any => {
        let attrs = module_version_cfg_attrs(&field.version, field_cfg);
        let field_name_ident: Ident = parse_str(&field.rust_name)?;
        let property_comments = field.docs.as_str();
        let field_type = type_from_decl_ref(&field.type_ref)?;
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
  field_cfg: VersionCfgContext,
  _choice_dispatch_field_names: &HashSet<String>,
) -> Result<Vec<TokenStream>> {
  let mut tokens = Vec::new();

  for field in fields {
    let field_name_ident: Ident = parse_str(&field.rust_name)?;
    let field_type = type_from_decl_ref(&field.type_ref)?;
    let attrs = module_version_cfg_attrs(&field.version, field_cfg);
    let choice_accepts_text = choice_type_accepts_text(module, &field.type_ref.rust_type);
    let choice_accepts_any = choice_type_accepts_any(module, &field.type_ref.rust_type);
    let (choice_qnames, gated_choice_qnames) =
      choice_type_specific_start_qname_groups(module, &field.type_ref.rust_type, field_cfg);
    let choice_qname_attrs = choice_qnames
      .into_iter()
      .map(|qname| quote! { qname = #qname })
      .collect::<Vec<_>>();
    let gated_choice_qname_attrs = gated_choice_qnames
      .into_iter()
      .map(|qname| quote! { qname = #qname })
      .collect::<Vec<_>>();
    let mut sdk_choice_attrs = Vec::new();
    if choice_qname_attrs.is_empty() && !choice_accepts_text && !choice_accepts_any {
      sdk_choice_attrs.push(quote! { #[sdk(choice)] });
    } else if choice_accepts_text && choice_accepts_any {
      sdk_choice_attrs.push(quote! { #[sdk(choice(#(#choice_qname_attrs,)* text, any))] });
    } else if choice_accepts_text {
      sdk_choice_attrs.push(quote! { #[sdk(choice(#(#choice_qname_attrs,)* text))] });
    } else if choice_accepts_any {
      sdk_choice_attrs.push(quote! { #[sdk(choice(#(#choice_qname_attrs,)* any))] });
    } else {
      sdk_choice_attrs.push(quote! { #[sdk(choice(#(#choice_qname_attrs),*))] });
    }
    if !gated_choice_qname_attrs.is_empty() {
      sdk_choice_attrs.push(quote! {
        #[sdk(choice(#(#gated_choice_qname_attrs),*))]
      });
    };

    match field.cardinality {
      Cardinality::Many => tokens.push(quote! {
        #( #attrs )*
        #( #sdk_choice_attrs )*
        pub #field_name_ident: Vec<#field_type>,
      }),
      Cardinality::Optional => tokens.push(quote! {
        #( #attrs )*
        #( #sdk_choice_attrs )*
        pub #field_name_ident: Option<#field_type>,
      }),
      Cardinality::One => tokens.push(quote! {
        #( #attrs )*
        #( #sdk_choice_attrs )*
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
    && type_decl.support.xml_header == crate::sdk_code::codegen_ir::XmlHeaderMode::None
}

fn can_alias_any_children_wrapper_decl(type_decl: &TypeDecl, attr_fields: &[&FieldDecl]) -> bool {
  if type_decl.kind != TypeKind::ElementStruct
    || type_decl.content_model != Some(ContentModelDecl::SequenceAnyOnly)
    || !attr_fields.is_empty()
    || type_decl.xml_content.is_some()
    || type_decl.support.have_xmlns_fields
    || type_decl.support.has_extra_support_fields()
    || type_decl.support.xml_header != crate::sdk_code::codegen_ir::XmlHeaderMode::None
  {
    return false;
  }

  let [MemberDecl::Field(field)] = type_decl.members.as_slice() else {
    return false;
  };

  matches!(field.wire, FieldWireDecl::Any)
    && field.cardinality == Cardinality::Many
    && field.type_ref.module_path.is_none()
    && field.type_ref.rust_type == "String"
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

fn child_field_name(child: &SchemaTypeChild, _child_type: &SchemaType) -> String {
  let raw_name = if child.property_name.is_empty() {
    child.name.split('/').nth(1).unwrap_or(child.name.as_str())
  } else {
    child.property_name.as_str()
  };

  schema_child_field_rust_name(raw_name)
}

fn choice_child_display_name<'a>(child: &'a ResolvedOneSequenceChild<'a>) -> &'a str {
  if child.name.is_empty() {
    child.field_name.as_ref()
  } else {
    child.name.split('/').nth(1).unwrap_or(child.name)
  }
}

fn child_kind_for_schema_type(schema_type: &SchemaType) -> SchemaTypeChildKind {
  if schema_type.base_class == "OpenXmlLeafTextElement"
    && schema_type.attributes.is_empty()
    && !schema_type.have_xmlns_fields
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
  use crate::sdk_data::schemas::gen_schemas;
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

    assert!(generated.contains("# [sdk (empty_child (qname = \"vt:CT_Empty/vt:empty\"))] VtEmpty"));
    assert!(generated.contains("# [sdk (empty_child (qname = \"vt:CT_Null/vt:null\"))] VtNull"));
    assert!(!generated.contains("pub struct VtEmpty"));
    assert!(!generated.contains("pub struct VtNull"));
    assert!(!generated.contains("VtEmpty (std :: boxed :: Box < VtEmpty >)"));
    assert!(!generated.contains("VtNull (std :: boxed :: Box < VtNull >)"));
  }

  #[test]
  fn lowers_openxml_empty_element_fields_to_unit_fields() {
    let schema = read_codegen_ir_schema_json(
      "../../sdk_data/schemas/schemas_microsoft_com_office_drawing_2013_main_command.json",
    );
    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(
      generated.contains("# [sdk (empty_child (office2016 , qname = \"oac:CT_Empty/oac:fill\"))]")
    );
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

    assert!(
      generated.contains("# [sdk (empty_child (office2010 , qname = \"w:CT_Empty/w14:noFill\"))]")
    );
    assert!(generated.contains("W14NoFill ,"));
    assert!(!generated.contains("pub struct NoFillEmpty"));
    assert!(!generated.contains("W14NoFill (std :: boxed :: Box < NoFillEmpty >)"));
  }

  #[test]
  fn omits_abstract_empty_base_types_without_element_name() {
    let schema = read_codegen_ir_schema_json(
      "../../sdk_data/schemas/schemas_microsoft_com_office_powerpoint_2022_08_main.json",
    );
    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(!generated.contains("pub struct EmptyType"));
    assert!(
      generated.contains("# [sdk (empty_child (microsoft365 , qname = \"p:CT_Empty/p228:add\"))]")
    );
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
    assert!(generated.contains("pub xml_other_attrs : Vec < (String , String) >"));
    assert!(generated.contains("pub xml_other_children : Vec < (usize , String) >"));
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
    let leaked_schemas = Box::leak(Box::new(schemas)).as_slice();
    let codegen_context = CodegenContext::new(leaked_schemas);

    (leaked_schemas, codegen_context)
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
                rust_type: "String".to_string(),
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
    assert!(generated.contains("pub unknown_xml : Vec < String >"));
  }

  #[test]
  fn generates_any_only_choice_without_boxed_string_payload() {
    let schema = read_codegen_ir_schema_json(
      "../../sdk_data/schemas/schemas_microsoft_com_office_2006_metadata_content_type.json",
    );
    let generated = gen_schema_from_ir(&schema, false).unwrap().to_string();

    assert!(generated.contains("# [sdk (any)] pub xml_children : Vec < String >"));
    assert!(!generated.contains("pub enum ContentTypeSchemaChoice"));
    assert!(!generated.contains("UnknownXml (String)"));
    assert!(!generated.contains("UnknownXml (std :: boxed :: Box < String >)"));
  }

  #[test]
  fn keeps_multi_field_sequence_helper_variants_when_inline_payload_would_be_large() {
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

    assert!(
      generated
        .contains("# [sdk (sequence)] Sequence (std :: boxed :: Box < HolderChoiceSequence >)")
    );
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

    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_First/t:first\"))] EgThing (std :: boxed :: Box < First >)"
    ));
    assert!(!generated.contains("# [sdk (sequence)] EgThing {"));
  }

  #[test]
  fn inlines_small_sequence_helper_variants_with_text_child() {
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

    assert!(generated.contains("# [sdk (sequence)] Sequence {"));
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_Formula/t:f\"))] formula : std :: boxed :: Box < Formula >"
    ));
    assert!(
      generated.contains(
        "# [sdk (text_child (qname = \"xsd:string/t:v\"))] value : Option < crate :: simple_type :: StringValue >"
      )
    );
    assert!(!generated.contains("pub struct HolderChoiceSequence1"));
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
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_First/t:first\"))] TFirst (std :: boxed :: Box < First >)"
    ));
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_Second/t:second\"))] TSecond (std :: boxed :: Box < Second >)"
    ));
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

    assert!(generated.contains("pub enum HolderChoice2"));
    assert!(generated.contains("# [sdk (choice)] Choice1 (std :: boxed :: Box < HolderChoice2 >)"));
  }

  #[test]
  fn flattens_pure_choice_wrapper_variants() {
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
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_First/t:first\"))] TFirst (std :: boxed :: Box < First >)"
    ));
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_Second/t:second\"))] Sequence (std :: boxed :: Box < Second >)"
    ));
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_Third/t:third\"))] TThird (std :: boxed :: Box < Third >)"
    ));
    assert!(!generated.contains("# [sdk (choice)] Choice1"));
    assert!(!generated.contains("# [sdk (choice)] Choice2"));
  }

  #[test]
  fn drops_pure_choice_wrapper_payload_helpers_after_render_flattening() {
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
    assert!(!generated.contains("pub enum HolderChoice1"));
    assert!(!generated.contains("pub enum HolderChoice2"));
  }

  #[test]
  fn flattens_anonymous_choice_wrapper_even_when_parent_variant_name_matches_nested_choice() {
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
    assert!(!generated.contains("# [sdk (choice)] Choice2 (std :: boxed :: Box < RootChoice1 >)"));
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_ProofErr/t:proofErr\"))] WProofErr (std :: boxed :: Box < ProofErr >)"
    ));
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_PermStart/t:permStart\"))] Choice (std :: boxed :: Box < PermStart >)"
    ));
    assert!(!generated.contains("pub enum RootChoice1"));
    assert!(!generated.contains("pub enum RootChoice2"));
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

    assert!(generated.contains("# [sdk (choice (qname ="));
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

    assert!(generated.contains("# [sdk (choice (qname ="));
    assert!(generated.contains("t14:CT_OfficeOnly/t14:officeOnly"));
    assert!(!generated.contains("cfg_attr"));
  }

  #[test]
  fn flattens_multi_step_anonymous_choice_wrappers_inside_mixed_parent() {
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
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_First/t:first\"))] TFirst (std :: boxed :: Box < First >)"
    ));
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_Second/t:second\"))] TSecond (std :: boxed :: Box < Second >)"
    ));
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_Third/t:third\"))] TThird (std :: boxed :: Box < Third >)"
    ));
    assert!(!generated.contains("pub enum RootChoice1"));
    assert!(!generated.contains("pub enum RootChoice2"));
    assert!(!generated.contains("# [sdk (choice)] Choice1 (std :: boxed :: Box < RootChoice1 >)"));
  }

  #[test]
  fn flattens_anonymous_wrapper_around_small_named_choice_helper() {
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
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_First/t:first\"))] TFirst (std :: boxed :: Box < First >)"
    ));
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_Second/t:second\"))] TSecond (std :: boxed :: Box < Second >)"
    ));
    assert!(
      !generated.contains("# [sdk (choice)] Choice (std :: boxed :: Box < RangeMarkupChoice >)")
    );
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
    assert!(
      generated.contains("# [sdk (choice)] Choice (std :: boxed :: Box < RangeMarkupChoice >)")
    );
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

    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_Second/t:second\"))] Sequence (std :: boxed :: Box < Second >)"
    ));
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

    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_First/t:first\"))] Sequence1 (std :: boxed :: Box < First >)"
    ));
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_Second/t:second\"))] Sequence2 (std :: boxed :: Box < Second >)"
    ));
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

    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_First/t:first\"))] Sequence (std :: boxed :: Box < First >)"
    ));
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_Second/t:second\"))] Sequence32 (std :: boxed :: Box < Second >)"
    ));
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
    assert!(generated.contains("pub table_properties : std :: boxed :: Box < TableProperties >"));
    assert!(generated.contains("pub table_grid : std :: boxed :: Box < TableGrid >"));
    assert!(generated.contains("pub after_choice : Vec < BodyLikeChoice2 >"));
    assert!(generated.contains("pub enum BodyLikeChoice"));
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_Bookmark/t:bookmarkStart\"))] TBookmarkStart (std :: boxed :: Box < BookmarkStart >)"
    ));
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_MarkupRange/t:bookmarkEnd\"))] TBookmarkEnd (std :: boxed :: Box < BookmarkEnd >)"
    ));

    let before_idx = generated
      .find("pub before_choice : Vec < BodyLikeChoice >")
      .unwrap();
    let tbl_pr_idx = generated
      .find("pub table_properties : std :: boxed :: Box < TableProperties >")
      .unwrap();
    let tbl_grid_idx = generated
      .find("pub table_grid : std :: boxed :: Box < TableGrid >")
      .unwrap();
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

    assert!(generated.contains("t:CT_Leaf/t:leaf"));
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
          kind: TypeKind::ElementStruct,
          element_kind: Some(ElementKind::Leaf),
          base_rust_name: Some("OpenXmlLeafElement".to_string()),
          members: vec![MemberDecl::Field(FieldDecl {
            rust_name: "value".to_string(),
            docs: "Value".to_string(),
            wire: FieldWireDecl::Attribute {
              qname: ":val".to_string(),
              bit: None,
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
    assert!(generated.contains("# [sdk (empty_child (qname = \"t:CT_Marker/t:marker\"))]"));
    assert!(generated.contains("TMarker ,"));
    assert!(
      generated.contains("# [sdk (child (qname = \"t:CT_AttributedMarker/t:attributedMarker\"))]")
    );
    assert!(generated.contains("TAttributedMarker (std :: boxed :: Box < AttributedMarker >)"));
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

    assert!(generated.contains("pub leaf_child : std :: boxed :: Box < Leaf >"));
    assert!(generated.contains("pub text_child : Option < crate :: simple_type :: StringValue >"));
    assert!(generated.contains("pub unknown_xml : Vec < String >"));
    assert!(!generated.contains("pub enum SequenceHolderChoice"));
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

    assert!(generated.contains("pub leaf_a : std :: boxed :: Box < LeafA >"));
    assert!(generated.contains("pub structured_holder_choice : Option < StructuredHolderChoice >"));
    assert!(generated.contains("pub enum StructuredHolderChoice"));
    assert!(generated.contains("# [sdk (sequence)] Sequence"));
    assert!(generated.contains("Sequence {"));
    assert!(
      generated.contains(
        "# [sdk (child (qname = \"t:CT_C/t:c\"))] leaf_c : std :: boxed :: Box < LeafC >"
      )
    );
    assert!(
      generated.contains(
        "# [sdk (child (qname = \"t:CT_D/t:d\"))] leaf_d : std :: boxed :: Box < LeafD >"
      )
    );
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

    assert!(generated.contains("pub leaf_a : std :: boxed :: Box < LeafA >"));
    assert!(generated.contains("pub mixed_holder_choice : Option < MixedHolderChoice >"));
    assert!(generated.contains("pub trailing_leaf : std :: boxed :: Box < LeafD >"));
    assert!(generated.contains("pub enum MixedHolderChoice"));
    assert!(generated.contains("TB (std :: boxed :: Box < LeafB >)"));
    assert!(generated.contains("TC (std :: boxed :: Box < LeafC >)"));
    assert!(generated.contains("TD (std :: boxed :: Box < LeafD >)"));
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
    assert!(generated.contains("TA (std :: boxed :: Box < LeafA >)"));
    assert!(generated.contains(
      "# [sdk (child (qname = \"t:CT_B/t:b\"))] Sequence (std :: boxed :: Box < LeafB >)"
    ));
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

    let generated = gen_attr_from_decl(&attr, VersionCfgContext::default())
      .unwrap()
      .to_string();

    assert!(generated.contains("# [sdk (attr (office2016 , qname = \":creationId\"))]"));
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
    assert!(!generated.contains("# [sdk (any)] pub xml_children : Vec < String >"));
    assert!(!generated.contains("pub enum AnyHolderChoice"));
    assert!(!generated.contains("UnknownXml (String)"));
  }
}
