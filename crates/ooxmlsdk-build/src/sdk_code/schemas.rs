use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;
use std::collections::HashMap;
use syn::{Attribute, Ident, Type, Variant, parse_str, parse2};

use crate::Result;
use crate::sdk_code::codegen_ir::{
  Cardinality, ContentModelDecl, ElementKind, EnumDecl, FieldDecl, FieldWireDecl, MemberDecl,
  SchemaModuleDecl, SystemSupportDecl, TypeDecl, TypeKind, TypeRefDecl, ValidatorKind,
};
use crate::sdk_code::codegen_ir_builder::build_codegen_ir;
use crate::sdk_code::helpers::{
  AttrTypeKind, StructuredChoice, StructuredChoiceVariant, StructuredParticleKind,
  classify_attr_type, is_leaf_text_wrapper, needs_xml_header, structure_one_sequence_particles,
};
use crate::sdk_code::versioning::{effective_version, is_microsoft365_version, version_cfg_attrs};
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
    Self {
      suppress: self.suppress || is_microsoft365_version(version),
    }
  }
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
  pub repeated: bool,
  pub children: Vec<ResolvedCompositeChild<'a>>,
}

#[derive(Debug)]
pub struct ResolvedOneChoice<'a> {
  pub field_name: String,
  pub enum_name: String,
  pub variants: Vec<ResolvedOneSequenceChild<'a>>,
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
    if let Some(child) = schema_type
      .children
      .iter()
      .find(|child| child.name == particle_name)
    {
      if child.kind == SchemaTypeChildKind::Any {
        let field_name = if child.property_name.is_empty() {
          Cow::Borrowed("unknown_xml")
        } else {
          Cow::Owned(escape_snake_case(child.property_name.to_snake_case()))
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
      } else {
        variants.push(self.resolve_one_sequence_child(schema_type, child.name.as_str())?);
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
            self.collect_resolved_children(
              _schema_type,
              &child.children,
              &mut sequence_children,
              resolved_names,
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
}

pub(crate) fn one_sequence_choice_field_name(
  schema_type: &SchemaType,
  choice_slot_count: usize,
  slot_index: usize,
) -> String {
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

fn one_sequence_choice_enum_name_from_rust_name(
  rust_name: &str,
  choice_slot_count: usize,
  slot_index: usize,
) -> String {
  if choice_slot_count <= 1 {
    format!("{}Choice", rust_name.to_upper_camel_case())
  } else {
    format!(
      "{}Choice{}",
      rust_name.to_upper_camel_case(),
      slot_index + 1
    )
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
  let version_cfg = VersionCfgContext::new(suppress_version_cfg_attrs);
  let mut token_stream_list: Vec<TokenStream> = vec![];
  let type_decl_by_name: std::collections::HashMap<_, _> = ir
    .types
    .iter()
    .map(|ty| (ty.rust_name.as_str(), ty))
    .collect();
  let schema_type_names: std::collections::HashSet<_> = ir
    .types
    .iter()
    .filter(|ty| matches!(ty.kind, TypeKind::ElementStruct | TypeKind::LeafTextAlias))
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
    let sdk_type_attrs = if let Some(qname) = &type_decl.xml_qname {
      quote! {
        #[sdk(qname = #qname)]
      }
    } else {
      quote! {}
    };
    let summary_doc = format!(" {}", type_decl.docs);
    let version_doc = if schema_type_version.is_empty() {
      " Available in Office2007 and above.".to_string()
    } else {
      format!(" Available in {schema_type_version} and above.")
    };
    let qualified_doc = if type_decl
      .xml_qname
      .as_deref()
      .is_none_or(|qname| qname.ends_with('/'))
    {
      " When the object is serialized out as xml, it's qualified name is .".to_string()
    } else {
      let qname = type_decl.xml_qname.as_deref().unwrap_or_default();
      let qualified_str = &qname[qname.find('/').unwrap() + 1..];
      format!(" When the object is serialized out as xml, it's qualified name is {qualified_str}.")
    };

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
          #[doc = ""]
          #[doc = #version_doc]
          #[doc = ""]
          #[doc = #qualified_doc]
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
        #[doc = ""]
        #[doc = #version_doc]
        #[doc = ""]
        #[doc = #qualified_doc]
        #[derive(
          Clone,
          Debug,
          Default,
          ooxmlsdk_derive::SdkType
        )]
        #sdk_type_attrs
        pub struct #struct_name_ident {
          #( #fields )*
        }
      });

      continue;
    }

    let mut fields: Vec<TokenStream> = vec![];
    let mut child_choice_enums: Vec<TokenStream> = vec![];
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
            ir,
            field_version_cfg,
            true,
          )?);
        }
        ContentModelDecl::OneChoiceSingle | ContentModelDecl::SequenceSingleChoice => {
          fields.extend(gen_choice_fields_from_decl(
            &choice_fields,
            field_version_cfg,
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
            ir,
            field_version_cfg,
          )?);
        }
        ContentModelDecl::SequenceAnyOnly => {
          let choice_type_name = choice_fields
            .first()
            .map(|field| field.type_ref.rust_type.clone())
            .unwrap_or_else(|| {
              one_sequence_choice_enum_name_from_rust_name(&type_decl.rust_name, 1, 0)
            });
          let child_choice_enum_ident: Ident = parse_str(&choice_type_name)?;
          let field_attrs = module_version_cfg_attrs(
            choice_fields
              .first()
              .map(|field| field.version.as_str())
              .unwrap_or(""),
            field_version_cfg,
          );
          fields.push(quote! {
            #( #field_attrs )*
            #[sdk(choice)]
            pub children: Vec<#child_choice_enum_ident>,
          });

          child_choice_enums.push(quote! {
            #( #type_attrs )*
            #[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
            pub enum #child_choice_enum_ident {
              #[sdk(any)]
              UnknownXml(String),
            }
          });
        }
        ContentModelDecl::SequenceDirectChildren => {
          fields.extend(gen_direct_child_fields_from_decl(
            &direct_child_fields,
            ir,
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
            ir,
            field_version_cfg,
          )?);
        }
        ContentModelDecl::GenericChildrenFallback => {
          if !choice_fields.is_empty() {
            fields.extend(gen_choice_fields_from_decl(
              &choice_fields,
              field_version_cfg,
            )?);
          }
        }
        ContentModelDecl::None => {
          if !choice_fields.is_empty() {
            fields.extend(gen_choice_fields_from_decl(
              &choice_fields,
              field_version_cfg,
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
            ir,
            field_version_cfg,
            true,
          )?);
        }
        ContentModelDecl::OneChoiceSingle | ContentModelDecl::SequenceSingleChoice => {
          fields.extend(gen_choice_fields_from_decl(
            &choice_fields,
            field_version_cfg,
          )?);
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
            ir,
            field_version_cfg,
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
            ir,
            field_version_cfg,
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
            ir,
            field_version_cfg,
          )?);
        }
        ContentModelDecl::GenericChildrenFallback => {
          if !choice_fields.is_empty() {
            fields.extend(gen_choice_fields_from_decl(
              &choice_fields,
              field_version_cfg,
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
            ir,
            field_version_cfg,
            false,
          )?);
        }
        ContentModelDecl::SequenceAnyOnly => {
          let choice_type_name = choice_fields
            .first()
            .map(|field| field.type_ref.rust_type.clone())
            .unwrap_or_else(|| {
              one_sequence_choice_enum_name_from_rust_name(&type_decl.rust_name, 1, 0)
            });
          let child_choice_enum_ident: Ident = parse_str(&choice_type_name)?;
          let field_attrs = module_version_cfg_attrs(
            choice_fields
              .first()
              .map(|field| field.version.as_str())
              .unwrap_or(""),
            field_version_cfg,
          );
          fields.push(quote! {
            #( #field_attrs )*
            #[sdk(choice)]
            pub children: Vec<#child_choice_enum_ident>,
          });

          child_choice_enums.push(quote! {
            #( #type_attrs )*
            #[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
            pub enum #child_choice_enum_ident {
              #[sdk(any)]
              UnknownXml(String),
            }
          });
        }
        ContentModelDecl::None => {
          if !choice_fields.is_empty() {
            fields.extend(gen_choice_fields_from_decl(
              &choice_fields,
              field_version_cfg,
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

    let child_choice_tokens = if !child_choice_enums.is_empty() {
      quote! {
        #( #child_choice_enums )*
      }
    } else {
      quote! {}
    };

    token_stream_list.push(quote! {
      #( #type_attrs )*
      #[doc = #summary_doc]
      #[doc = ""]
      #[doc = #version_doc]
      #[doc = ""]
      #[doc = #qualified_doc]
      #[derive(
        Clone,
        Debug,
        Default,
        ooxmlsdk_derive::SdkType
      )]
      #sdk_type_attrs
      pub struct #struct_name_ident {
        #( #fields )*
      }
      #( #items )*
      #child_choice_tokens
    });
  }

  for type_decl in ir
    .types
    .iter()
    .filter(|type_decl| !schema_type_names.contains(type_decl.rust_name.as_str()))
  {
    match type_decl.kind {
      TypeKind::ChoiceEnum => {
        token_stream_list.push(gen_choice_type_decl(type_decl, ir, version_cfg)?)
      }
      TypeKind::HelperStruct => {
        token_stream_list.push(gen_helper_struct_type_decl(type_decl, ir, version_cfg)?)
      }
      _ => {}
    }
  }

  Ok(quote! {
    #( #token_stream_list )*
  })
}

fn common_choice_version<'a>(container_version: &'a str, variant_versions: &[&str]) -> &'a str {
  if is_microsoft365_version(container_version)
    || (!variant_versions.is_empty()
      && variant_versions
        .iter()
        .all(|version| is_microsoft365_version(version)))
  {
    "Microsoft365"
  } else {
    ""
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
  let baseline_facets: Vec<_> = schema_enum
    .variants
    .iter()
    .filter(|facet| !is_microsoft365_version(&facet.version))
    .collect();
  let default_facet = baseline_facets
    .first()
    .copied()
    .unwrap_or_else(|| schema_enum.variants.first().expect("schema enum facet"));

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
    #[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
    #sdk_enum_attrs
    pub enum #enum_name_ident {
      #( #variants, )*
    }
  })
}

fn gen_choice_type_decl(
  type_decl: &TypeDecl,
  module: &SchemaModuleDecl,
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
  let mut variants = Vec::new();

  for member in &type_decl.members {
    let MemberDecl::Variant(variant) = member else {
      continue;
    };
    let variant_ident: Ident = parse_str(&variant.rust_name)?;
    let variant_attrs = module_version_cfg_attrs(&variant.version, variant_cfg);
    let payload_type = type_from_decl_ref(&variant.payload)?;
    let (sdk_variant_attrs, wrap_box) = match &variant.wire {
      crate::sdk_code::codegen_ir::VariantWireDecl::Any => (quote! { #[sdk(any)] }, false),
      crate::sdk_code::codegen_ir::VariantWireDecl::TextChild { qnames } => {
        let qname = qnames.first().ok_or_else(|| variant.rust_name.clone())?;
        (
          quote! { #[sdk(text_child(qname = #qname))] },
          !is_value_like_type_ref(module, &variant.payload),
        )
      }
      crate::sdk_code::codegen_ir::VariantWireDecl::Child { qnames } => {
        if qnames.is_empty() {
          return Err(variant.rust_name.clone().into());
        }
        (quote! { #( #[sdk(child(qname = #qnames))] )* }, true)
      }
      crate::sdk_code::codegen_ir::VariantWireDecl::Text => (quote! { #[sdk(text)] }, false),
    };

    if wrap_box {
      variants.push(quote! {
        #( #variant_attrs )*
        #sdk_variant_attrs
        #variant_ident(std::boxed::Box<#payload_type>),
      });
    } else {
      variants.push(quote! {
        #( #variant_attrs )*
        #sdk_variant_attrs
        #variant_ident(#payload_type),
      });
    }
  }

  Ok(quote! {
    #( #enum_attrs )*
    #[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
    pub enum #enum_ident {
      #( #variants )*
    }
  })
}

fn gen_helper_struct_type_decl(
  type_decl: &TypeDecl,
  module: &SchemaModuleDecl,
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
  let fields = gen_flatten_one_sequence_fields_from_decl(&helper_fields, module, nested_cfg)?;

  Ok(quote! {
    #( #type_attrs )*
    #[derive(
      Clone,
      Debug,
      Default,
      ooxmlsdk_derive::SdkType
    )]
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
  let sdk_attr_attrs = quote! {
    #[sdk(attr(qname = #qname))]
  };
  let validator_attrs: Vec<TokenStream> = attr
    .validators
    .iter()
    .map(|validator| match &validator.kind {
      ValidatorKind::Pattern { regex } => quote! {
        #[sdk(pattern(regex = #regex))]
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
          #[sdk(string_format(kind = #kind_lit))]
        }
      }
      ValidatorKind::StringLength { min, max, exact } => {
        let effective_min = exact.or(*min);
        let effective_max = exact.or(*max);
        let min_attr = effective_min.map(|min| quote! { min = #min, });
        let max_attr = effective_max.map(|max| quote! { max = #max, });
        quote! {
          #[sdk(string_length(#min_attr #max_attr))]
        }
      }
      ValidatorKind::NumberRange {
        min,
        max,
        min_inclusive,
        max_inclusive,
      } => {
        let min_attr = min.as_ref().map(|min| quote! { min = #min, });
        let max_attr = max.as_ref().map(|max| quote! { max = #max, });
        quote! {
          #[sdk(number_range(
            #min_attr
            #max_attr
            min_inclusive = #min_inclusive,
            max_inclusive = #max_inclusive
          ))]
        }
      }
      ValidatorKind::NumberSign { kind } => {
        let kind_lit = match kind {
          crate::sdk_code::codegen_ir::NumberSignKind::NonNegative => "non_negative",
          crate::sdk_code::codegen_ir::NumberSignKind::Positive => "positive",
        };
        quote! {
          #[sdk(number_sign(kind = #kind_lit))]
        }
      }
      ValidatorKind::Required
      | ValidatorKind::EnumRef { .. }
      | ValidatorKind::StringSet { .. }
      | ValidatorKind::Unsupported { .. }
      | ValidatorKind::Placeholder => quote! {},
    })
    .collect();
  let property_comments_doc = format!(" {}", attr.docs);
  let version_doc = if attr.version.is_empty() {
    " Available in Office2007 and above.".to_string()
  } else {
    format!(" Available in {} and above.", attr.version)
  };
  let qualified_doc = format!(
    " Represents the following attribute in the schema: {}",
    qname
  );

  Ok(match attr.cardinality {
    Cardinality::One => quote! {
      #( #attr_attrs )*
      #[doc = #property_comments_doc]
      #[doc = ""]
      #[doc = #version_doc]
      #[doc = ""]
      #[doc = #qualified_doc]
      #sdk_attr_attrs
      #( #validator_attrs )*
      #bit_attrs
      pub #attr_name_ident: #type_ident,
    },
    Cardinality::Optional => quote! {
      #( #attr_attrs )*
      #[doc = #property_comments_doc]
      #[doc = ""]
      #[doc = #version_doc]
      #[doc = ""]
      #[doc = #qualified_doc]
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
  if let Some(module_path) = &type_ref.module_path {
    Ok(parse_str(&format!(
      "{module_path}::{}",
      type_ref.rust_type
    ))?)
  } else {
    Ok(parse_str(&type_ref.rust_type)?)
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

fn gen_support_fields(support: &SystemSupportDecl) -> Vec<TokenStream> {
  let mut fields = Vec::new();

  if support.xmlns_mode == crate::sdk_code::codegen_ir::XmlnsMode::MapOnly {
    fields.push(quote! {
      #[sdk(xmlns)]
      pub xmlns: Option<String>,
    });
    fields.push(quote! {
      #[sdk(xmlns)]
      pub xmlns_map: std::collections::HashMap<String, String>,
    });
  }

  if support.xml_header != crate::sdk_code::codegen_ir::XmlHeaderMode::None {
    fields.push(quote! {
      #[sdk(xml_header)]
      pub xml_header: crate::common::XmlHeaderType,
    });
  }

  if support.has_mce {
    fields.push(quote! {
      #[sdk(mce)]
      pub mc_ignorable: Option<String>,
    });
  }

  fields
}

fn gen_direct_child_fields_from_decl(
  fields: &[&FieldDecl],
  module: &SchemaModuleDecl,
  field_cfg: VersionCfgContext,
  force_optional_when_not_repeated: bool,
) -> Result<Vec<TokenStream>> {
  let mut tokens = Vec::new();

  for field in fields {
    let attr = module_version_cfg_attrs(&field.version, field_cfg);
    let field_name_ident: Ident = parse_str(&field.rust_name)?;
    let field_type = type_from_decl_ref(&field.type_ref)?;
    let property_comments = field.docs.as_str();
    let (sdk_field_attrs, wrap_box) = match &field.wire {
      FieldWireDecl::Child { qname } => (quote! { #[sdk(child(qname = #qname))] }, true),
      FieldWireDecl::TextChild { qname } => (
        quote! { #[sdk(text_child(qname = #qname))] },
        !is_value_like_type_ref(module, &field.type_ref),
      ),
      _ => return Err(format!("expected direct child field, got {:?}", field.wire).into()),
    };

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
  module: &SchemaModuleDecl,
  field_cfg: VersionCfgContext,
) -> Result<Vec<TokenStream>> {
  let mut tokens = Vec::new();

  for field in fields {
    match &field.wire {
      FieldWireDecl::Choice => {
        tokens.extend(gen_choice_fields_from_decl(
          std::slice::from_ref(field),
          field_cfg,
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
        tokens.extend(gen_direct_child_fields_from_decl(
          std::slice::from_ref(field),
          module,
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
  field_cfg: VersionCfgContext,
) -> Result<Vec<TokenStream>> {
  let mut tokens = Vec::new();

  for field in fields {
    let field_name_ident: Ident = parse_str(&field.rust_name)?;
    let field_type = type_from_decl_ref(&field.type_ref)?;
    let attrs = module_version_cfg_attrs(&field.version, field_cfg);

    match field.cardinality {
      Cardinality::Many => tokens.push(quote! {
        #( #attrs )*
        #[sdk(choice)]
        pub #field_name_ident: Vec<#field_type>,
      }),
      Cardinality::Optional => tokens.push(quote! {
        #( #attrs )*
        #[sdk(choice)]
        pub #field_name_ident: Option<#field_type>,
      }),
      Cardinality::One => tokens.push(quote! {
        #( #attrs )*
        #[sdk(choice)]
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

fn can_alias_leaf_text_wrapper(schema_type: &SchemaType) -> bool {
  is_leaf_text_wrapper(schema_type)
    && schema_type.attributes.is_empty()
    && !schema_type.has_xmlns_fields
    && !schema_type.has_mc_ignorable_field
    && !needs_xml_header(schema_type)
}

fn can_alias_leaf_text_wrapper_decl(type_decl: &TypeDecl, attr_fields: &[&FieldDecl]) -> bool {
  type_decl.kind == TypeKind::LeafTextAlias
    && attr_fields.is_empty()
    && type_decl.support.xmlns_mode == crate::sdk_code::codegen_ir::XmlnsMode::None
    && !type_decl.support.has_mce
    && type_decl.support.xml_header == crate::sdk_code::codegen_ir::XmlHeaderMode::None
}

fn can_inline_text_child(schema_type: &SchemaType) -> bool {
  can_alias_leaf_text_wrapper(schema_type)
}

fn child_field_name(child: &SchemaTypeChild, _child_type: &SchemaType) -> String {
  let raw_name = if child.property_name.is_empty() {
    child.name.split('/').nth(1).unwrap_or(child.name.as_str())
  } else {
    child.property_name.as_str()
  };

  escape_snake_case(raw_name.to_snake_case())
}

fn choice_child_display_name<'a>(child: &'a ResolvedOneSequenceChild<'a>) -> &'a str {
  if child.name.is_empty() {
    child.field_name.as_ref()
  } else {
    child.name.split('/').nth(1).unwrap_or(child.name)
  }
}

fn leaf_text_wrapper_alias_type(
  child_name: &str,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<Type> {
  let child_type = context
    .type_map
    .get(child_name)
    .ok_or_else(|| format!("{child_name:?}"))?;
  let child_prefix = context
    .type_prefix_map
    .get(child_name)
    .ok_or_else(|| format!("{child_name:?}"))?;

  if *child_prefix != schema.prefix {
    let child_module_name = context
      .type_module(child_name)
      .ok_or_else(|| format!("{child_name:?}"))?;

    Ok(parse_str(&format!(
      "crate::schemas::{}::{}",
      child_module_name,
      child_type.class_name.to_upper_camel_case()
    ))?)
  } else {
    Ok(parse_str(&child_type.class_name.to_upper_camel_case())?)
  }
}

fn child_kind_for_schema_type(schema_type: &SchemaType) -> SchemaTypeChildKind {
  if schema_type.base_class == "OpenXmlLeafTextElement"
    && schema_type.attributes.is_empty()
    && !schema_type.has_xmlns_fields
  {
    SchemaTypeChildKind::TextChild
  } else {
    SchemaTypeChildKind::Child
  }
}

fn child_field_shape(
  child: &ResolvedOneSequenceChild<'_>,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<(TokenStream, Type, bool)> {
  let child_qname = child.name;

  if child.kind == SchemaTypeChildKind::Any {
    return Ok((quote! { #[sdk(any)] }, parse_str("String")?, false));
  }

  let child_type = context
    .type_map
    .get(child.name)
    .ok_or_else(|| format!("{:?}", child.name))?;

  if can_inline_text_child(child_type) {
    if can_alias_leaf_text_wrapper(child_type) {
      return Ok((
        quote! { #[sdk(text_child(qname = #child_qname))] },
        leaf_text_wrapper_alias_type(child.name, schema, context)?,
        false,
      ));
    }

    let wrap_box = true;
    return Ok((
      quote! { #[sdk(text_child(qname = #child_qname))] },
      one_sequence_child_variant_type(child, schema, context)?,
      wrap_box,
    ));
  }

  let child_type = context
    .type_map
    .get(child.name)
    .ok_or_else(|| format!("{:?}", child.name))?;
  let child_prefix = context
    .type_prefix_map
    .get(child.name)
    .ok_or_else(|| format!("{:?}", child.name))?;

  let child_variant_type: Type = if *child_prefix != schema.prefix {
    let child_module_name = context
      .type_module(child.name)
      .ok_or_else(|| format!("{:?}", child.name))?;

    parse_str(&format!(
      "crate::schemas::{}::{}",
      child_module_name,
      child_type.class_name.to_upper_camel_case()
    ))?
  } else {
    parse_str(&child_type.class_name.to_upper_camel_case())?
  };

  Ok((
    quote! { #[sdk(child(qname = #child_qname))] },
    child_variant_type,
    true,
  ))
}

fn choice_child_variant_shape(
  child: &ResolvedOneSequenceChild<'_>,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<(TokenStream, Type, bool)> {
  let child_qname = child.name;

  if child.kind == SchemaTypeChildKind::TextChild && child.name.is_empty() {
    return Ok((
      quote! { #[sdk(text)] },
      parse_str("crate::simple_type::StringValue")?,
      false,
    ));
  }

  if child.kind == SchemaTypeChildKind::Any {
    return Ok((quote! { #[sdk(any)] }, parse_str("String")?, false));
  }

  let child_type = context
    .type_map
    .get(child.name)
    .ok_or_else(|| format!("{:?}", child.name))?;

  if can_inline_text_child(child_type) {
    if can_alias_leaf_text_wrapper(child_type) {
      return Ok((
        quote! { #[sdk(text_child(qname = #child_qname))] },
        leaf_text_wrapper_alias_type(child.name, schema, context)?,
        false,
      ));
    }

    let wrap_box = true;
    return Ok((
      quote! { #[sdk(text_child(qname = #child_qname))] },
      one_sequence_child_variant_type(child, schema, context)?,
      wrap_box,
    ));
  }

  Ok((
    quote! { #[sdk(child(qname = #child_qname))] },
    one_sequence_child_variant_type(child, schema, context)?,
    true,
  ))
}

#[allow(dead_code)]
fn gen_structured_one_sequence_fields(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
  field_cfg: VersionCfgContext,
  item_cfg: VersionCfgContext,
) -> Result<(Vec<TokenStream>, Vec<TokenStream>)> {
  let mut fields: Vec<TokenStream> = vec![];
  let mut paragraph_prefix_fields: Vec<TokenStream> = vec![];
  let mut items: Vec<TokenStream> = vec![];
  let mut field_name_set = std::collections::HashSet::new();
  let structured_particles = structure_one_sequence_particles(schema_type);
  let choice_slot_count = structured_particles
    .iter()
    .filter(|particle| matches!(particle.kind, StructuredParticleKind::Choice(_)))
    .count();
  let mut choice_slot_index = 0usize;

  for particle in structured_particles.into_iter() {
    match particle.kind {
      StructuredParticleKind::Leaf(leaf) => {
        let child = context.resolve_one_sequence_child(schema_type, leaf.name.as_str())?;
        let (sdk_field_attrs, child_variant_type, wrap_box) =
          child_field_shape(&child, schema, context)?;
        let child_name_ident: Ident = parse_str(&child.field_name)?;

        if !field_name_set.insert(child_name_ident.to_string()) {
          continue;
        }

        let property_comments = child.property_comments.as_ref();
        let field_attrs = module_version_cfg_attrs(
          effective_version(child.version, particle.initial_version),
          field_cfg,
        );
        if particle.repeated {
          let field_tokens = quote! {
            #( #field_attrs )*
            #[doc = #property_comments]
            #sdk_field_attrs
            pub #child_name_ident: Vec<#child_variant_type>,
          };
          if schema_type.class_name == "Paragraph" && child.field_name == "paragraph_properties" {
            paragraph_prefix_fields.push(field_tokens);
          } else {
            fields.push(field_tokens);
          }
        } else if particle.optional {
          let field_tokens = if wrap_box {
            quote! {
              #( #field_attrs )*
              #[doc = #property_comments]
              #sdk_field_attrs
              pub #child_name_ident: Option<std::boxed::Box<#child_variant_type>>,
            }
          } else {
            quote! {
              #( #field_attrs )*
              #[doc = #property_comments]
              #sdk_field_attrs
              pub #child_name_ident: Option<#child_variant_type>,
            }
          };
          if schema_type.class_name == "Paragraph" && child.field_name == "paragraph_properties" {
            paragraph_prefix_fields.push(field_tokens);
          } else {
            fields.push(field_tokens);
          }
        } else {
          let field_tokens = if wrap_box {
            quote! {
              #( #field_attrs )*
              #[doc = #property_comments]
              #sdk_field_attrs
              pub #child_name_ident: std::boxed::Box<#child_variant_type>,
            }
          } else {
            quote! {
              #( #field_attrs )*
              #[doc = #property_comments]
              #sdk_field_attrs
              pub #child_name_ident: #child_variant_type,
            }
          };
          if schema_type.class_name == "Paragraph" && child.field_name == "paragraph_properties" {
            paragraph_prefix_fields.push(field_tokens);
          } else {
            fields.push(field_tokens);
          }
        }
      }
      StructuredParticleKind::Choice(choice) => {
        let choice = context.resolve_one_sequence_structured_choice(
          schema_type,
          &choice,
          choice_slot_count,
          choice_slot_index,
        )?;
        choice_slot_index += 1;
        let choice_field_ident: Ident = parse_str(&choice.field_name)?;

        if !field_name_set.insert(choice_field_ident.to_string()) {
          continue;
        }

        let choice_enum_ident: Ident = parse_str(&choice.enum_name)?;
        let property_comments = choice.property_comments.as_str();
        let choice_version = common_choice_version(
          particle.initial_version,
          &choice
            .variants
            .iter()
            .map(|variant| match variant {
              ResolvedOneSequenceChoiceVariant::Leaf(child) => child.version,
              ResolvedOneSequenceChoiceVariant::Sequence(sequence_variant) => {
                common_choice_version(
                  "",
                  &sequence_variant
                    .fields
                    .iter()
                    .map(|field| effective_version(field.child.version, field.initial_version))
                    .collect::<Vec<_>>(),
                )
              }
            })
            .collect::<Vec<_>>(),
        );
        let field_attrs = module_version_cfg_attrs(choice_version, field_cfg);
        let sdk_choice_attrs = quote! {
          #[sdk(choice)]
        };
        let enum_attrs = module_version_cfg_attrs(choice_version, item_cfg);
        let variant_cfg = if enum_attrs.is_empty() {
          item_cfg
        } else {
          VersionCfgContext::new(true)
        };
        let mut variants = Vec::new();
        let mut default_variant = None;

        for variant in &choice.variants {
          match variant {
            ResolvedOneSequenceChoiceVariant::Leaf(child) => {
              let (sdk_variant_attrs, variant_type, wrap_box) =
                choice_child_variant_shape(child, schema, context)?;
              let variant_ident: Ident = parse_str(&child.field_name.to_upper_camel_case())?;
              let variant_attrs = module_version_cfg_attrs(child.version, variant_cfg);
              if default_variant.is_none()
                && (choice_version == child.version
                  || (choice_version.is_empty() && !is_microsoft365_version(child.version)))
              {
                default_variant = Some((variant_ident.clone(), variant_type.clone(), wrap_box));
              }
              if wrap_box {
                variants.push(quote! {
                  #( #variant_attrs )*
                  #sdk_variant_attrs
                  #variant_ident(std::boxed::Box<#variant_type>),
                });
              } else {
                variants.push(quote! {
                  #( #variant_attrs )*
                  #sdk_variant_attrs
                  #variant_ident(#variant_type),
                });
              }
            }
            ResolvedOneSequenceChoiceVariant::Sequence(sequence_variant) => {
              let struct_ident: Ident = parse_str(&sequence_variant.struct_name)?;
              let variant_ident: Ident = parse_str(&sequence_variant.variant_name)?;
              let sequence_property_comments = sequence_variant.property_comments.as_str();
              let sequence_child_qnames: Vec<&str> = sequence_variant
                .fields
                .iter()
                .map(|field| field.child.name)
                .collect();
              let sequence_version = common_choice_version(
                "",
                &sequence_variant
                  .fields
                  .iter()
                  .map(|field| effective_version(field.child.version, field.initial_version))
                  .collect::<Vec<_>>(),
              );
              let sequence_attrs = module_version_cfg_attrs(sequence_version, item_cfg);
              let sequence_field_cfg = if sequence_attrs.is_empty() {
                item_cfg
              } else {
                VersionCfgContext::new(true)
              };
              let sequence_fields =
                gen_sequence_variant_fields(sequence_variant, schema, context, sequence_field_cfg)?;
              if default_variant.is_none()
                && (choice_version == sequence_version
                  || (choice_version.is_empty() && !is_microsoft365_version(sequence_version)))
              {
                default_variant = Some((
                  variant_ident.clone(),
                  parse2(quote! { #struct_ident })?,
                  false,
                ));
              }

              items.push(quote! {
                #( #sequence_attrs )*
                #[doc = #sequence_property_comments]
                #[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
                pub struct #struct_ident {
                  #( #sequence_fields )*
                }
              });

              variants.push(quote! {
                #( #sequence_attrs )*
                #( #[sdk(child(qname = #sequence_child_qnames))] )*
                #variant_ident(#struct_ident),
              });
            }
          }
        }

        default_variant
          .or_else(|| {
            choice.variants.first().and_then(|variant| match variant {
              ResolvedOneSequenceChoiceVariant::Leaf(child) => Some((
                parse_str::<Ident>(&child.field_name.to_upper_camel_case()).ok()?,
                one_sequence_child_variant_type(child, schema, context).ok()?,
                matches!(
                  choice_child_variant_shape(child, schema, context).ok()?,
                  (_, _, true)
                ),
              )),
              ResolvedOneSequenceChoiceVariant::Sequence(sequence_variant) => Some((
                parse_str::<Ident>(&sequence_variant.variant_name).ok()?,
                parse_str::<Type>(&sequence_variant.struct_name).ok()?,
                false,
              )),
            })
          })
          .ok_or_else(|| choice.enum_name.clone())?;

        items.push(quote! {
          #( #enum_attrs )*
          #[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
          pub enum #choice_enum_ident {
            #( #variants )*
          }
        });
        if particle.repeated {
          fields.push(quote! {
            #( #field_attrs )*
            #[doc = #property_comments]
            #sdk_choice_attrs
            pub #choice_field_ident: Vec<#choice_enum_ident>,
          });
        } else {
          fields.push(quote! {
            #( #field_attrs )*
            #[doc = #property_comments]
            #sdk_choice_attrs
            pub #choice_field_ident: Option<#choice_enum_ident>,
          });
        }
      }
    }
  }

  if !paragraph_prefix_fields.is_empty() {
    paragraph_prefix_fields.extend(fields);
    fields = paragraph_prefix_fields;
  }

  Ok((fields, items))
}

fn gen_sequence_variant_fields(
  sequence_variant: &ResolvedOneSequenceSequenceVariant<'_>,
  schema: &Schema,
  context: &CodegenContext<'_>,
  version_cfg: VersionCfgContext,
) -> Result<Vec<TokenStream>> {
  let mut fields = Vec::new();

  for field in &sequence_variant.fields {
    let child = &field.child;
    let (sdk_field_attrs, child_variant_type, wrap_box) =
      child_field_shape(child, schema, context)?;

    let child_name_ident: Ident = parse_str(&child.field_name)?;
    let property_comments = child.property_comments.as_ref();
    let field_attrs = module_version_cfg_attrs(
      effective_version(child.version, field.initial_version),
      version_cfg,
    );

    if field.repeated {
      fields.push(quote! {
        #( #field_attrs )*
        #[doc = #property_comments]
        #sdk_field_attrs
        pub #child_name_ident: Vec<#child_variant_type>,
      });
    } else if field.optional {
      if wrap_box {
        fields.push(quote! {
          #( #field_attrs )*
          #[doc = #property_comments]
          #sdk_field_attrs
          pub #child_name_ident: Option<std::boxed::Box<#child_variant_type>>,
        });
      } else {
        fields.push(quote! {
          #( #field_attrs )*
          #[doc = #property_comments]
          #sdk_field_attrs
          pub #child_name_ident: Option<#child_variant_type>,
        });
      }
    } else {
      if wrap_box {
        fields.push(quote! {
          #( #field_attrs )*
          #[doc = #property_comments]
          #sdk_field_attrs
          pub #child_name_ident: std::boxed::Box<#child_variant_type>,
        });
      } else {
        fields.push(quote! {
          #( #field_attrs )*
          #[doc = #property_comments]
          #sdk_field_attrs
          pub #child_name_ident: #child_variant_type,
        });
      }
    }
  }

  Ok(fields)
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

    assert!(generated.contains("UnknownXml (String)"));
    assert!(!generated.contains("UnknownXml (std :: boxed :: Box < String >)"));
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

    assert!(generated.contains("pub choice_holder_choice : Vec < ChoiceHolderChoice >"));
    assert!(generated.contains("pub enum ChoiceHolderChoice"));
    assert!(generated.contains("# [sdk (child (qname = \"t:CT_Leaf/t:leaf\"))]"));
    assert!(generated.contains("TLeaf (std :: boxed :: Box < Leaf >)"));
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
    assert!(generated.contains("pub sequence_holder_choice : Option < SequenceHolderChoice >"));
    assert!(generated.contains("pub unknown_xml : Vec < String >"));
    assert!(generated.contains("pub enum SequenceHolderChoice"));
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
    assert!(
      generated.contains("Sequence2 (std :: boxed :: Box < StructuredHolderChoiceSequence2 >)")
    );
    assert!(generated.contains("pub struct StructuredHolderChoiceSequence2"));
    assert!(generated.contains("pub leaf_c : std :: boxed :: Box < LeafC >"));
    assert!(generated.contains("pub leaf_d : std :: boxed :: Box < LeafD >"));
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

    assert!(generated.contains("pub fallback_holder_choice : Vec < FallbackHolderChoice >"));
    assert!(generated.contains("pub enum FallbackHolderChoice"));
    assert!(generated.contains("TA (std :: boxed :: Box < LeafA >)"));
    assert!(
      generated.contains("Sequence2 (std :: boxed :: Box < FallbackHolderChoiceSequence2 >)")
    );
    assert!(generated.contains("pub struct FallbackHolderChoiceSequence2"));
    assert!(generated.contains("FallbackHolderChoiceSequence2"));
    assert!(generated.contains("leaf_b"));
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
          kind: ValidatorKind::Pattern {
            regex: "[A-Z]+".to_string(),
          },
        },
        crate::sdk_code::codegen_ir::ValidatorDecl {
          version: String::new(),
          kind: ValidatorKind::StringFormat {
            kind: crate::sdk_code::codegen_ir::StringFormatKind::Token,
          },
        },
        crate::sdk_code::codegen_ir::ValidatorDecl {
          version: String::new(),
          kind: ValidatorKind::StringLength {
            min: Some(2),
            max: Some(8),
            exact: None,
          },
        },
        crate::sdk_code::codegen_ir::ValidatorDecl {
          version: String::new(),
          kind: ValidatorKind::NumberRange {
            min: Some("0".to_string()),
            max: Some("10".to_string()),
            min_inclusive: true,
            max_inclusive: false,
          },
        },
        crate::sdk_code::codegen_ir::ValidatorDecl {
          version: String::new(),
          kind: ValidatorKind::NumberSign {
            kind: crate::sdk_code::codegen_ir::NumberSignKind::NonNegative,
          },
        },
      ],
    };

    let generated = gen_attr_from_decl(&attr, VersionCfgContext::default())
      .unwrap()
      .to_string();

    assert!(generated.contains("# [sdk (attr (qname = \":creationId\"))]"));
    assert!(generated.contains("# [sdk (pattern (regex = \"[A-Z]+\"))]"));
    assert!(generated.contains("# [sdk (string_format (kind = \"token\"))]"));
    assert!(generated.contains("# [sdk (string_length (min = 2u32 , max = 8u32 ,))]"));
    assert!(generated.contains(
      "# [sdk (number_range (min = \"0\" , max = \"10\" , min_inclusive = true , max_inclusive = false))]"
    ));
    assert!(generated.contains("# [sdk (number_sign (kind = \"non_negative\"))]"));
  }
}
