use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;
use std::collections::HashMap;
use syn::{Attribute, Ident, ItemEnum, Type, Variant, parse_str, parse2};

use crate::Result;
use crate::sdk_code::helpers::{
  AttrTypeKind, FlatParticleKind, StructuredChoice, StructuredChoiceVariant,
  StructuredParticleKind, classify_attr_type, flatten_one_sequence_particles, has_xmlns_fields,
  is_composite_type, is_derived_type, is_leaf_element_type, is_leaf_text_type,
  is_leaf_text_wrapper, is_one_sequence_flatten, is_one_sequence_structurable, needs_xml_header,
  structure_one_sequence_particles,
};
use crate::sdk_code::versioning::{effective_version, is_microsoft365_version, version_cfg_attrs};
use crate::sdk_data::sdk_data_model::{
  Schema, SchemaEnum, SchemaType, SchemaTypeAttribute, SchemaTypeChild, SchemaTypeChildKind,
  SchemaTypeCompositeKind,
};
use crate::simple_type::simple_type_mapping;
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
      let child_type = self
        .type_by_name(child.name.as_str())
        .ok_or_else(|| child.name.clone())?;
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
        version: child_type.version.as_str(),
        kind: child.kind,
      });
    }

    let child_type = self
      .type_by_name(particle_name)
      .ok_or_else(|| particle_name.to_string())?;

    Ok(ResolvedOneSequenceChild {
      name: particle_name,
      field_name: Cow::Owned(escape_snake_case(child_type.class_name.to_snake_case())),
      property_comments: if child_type.summary.is_empty() {
        Cow::Borrowed(" _")
      } else {
        Cow::Owned(child_type.summary.clone())
      },
      version: child_type.version.as_str(),
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
    let enum_name = one_sequence_choice_enum_name(schema_type, choice_slot_count, &field_name);
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
    let enum_name = one_sequence_choice_enum_name(schema_type, choice_slot_count, &field_name);
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
            &field_name,
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
            let mut sequence_names = std::collections::HashSet::new();
            self.collect_resolved_children(
              _schema_type,
              &child.children,
              &mut sequence_children,
              &mut sequence_names,
              true,
            )?;

            resolved.push(ResolvedCompositeChild {
              name: child.name.as_str(),
              variant_name: Cow::Borrowed(if child.property_name.is_empty() {
                "Sequence"
              } else {
                child.property_name.as_str()
              }),
              version: sequence_children
                .first()
                .map(|item| item.version)
                .unwrap_or_default(),
              is_any: false,
              kind: child.kind,
              repeated: child.repeated,
              children: sequence_children,
            });
          } else {
            self.collect_resolved_children(
              _schema_type,
              &child.children,
              resolved,
              resolved_names,
              false,
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
        .map(|item| item.version.as_str())
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

fn one_sequence_choice_field_name(
  schema_type: &SchemaType,
  choice_slot_count: usize,
  slot_index: usize,
) -> String {
  if choice_slot_count <= 1 {
    format!("{}_choice", schema_type.class_name.to_snake_case())
  } else {
    format!(
      "{}_choice_{}",
      schema_type.class_name.to_snake_case(),
      slot_index + 1
    )
  }
}

fn one_sequence_choice_enum_name(
  schema_type: &SchemaType,
  choice_slot_count: usize,
  field_name: &str,
) -> String {
  let stem = one_sequence_choice_type_stem(schema_type, field_name);
  if choice_slot_count <= 1 {
    if stem.ends_with("Choice") {
      stem
    } else {
      format!("{stem}Choice")
    }
  } else {
    stem
  }
}

fn one_sequence_choice_sequence_struct_name(
  schema_type: &SchemaType,
  choice_slot_count: usize,
  field_name: &str,
  variant_index: usize,
) -> String {
  let stem = one_sequence_choice_type_stem(schema_type, field_name);
  if choice_slot_count <= 1 {
    if stem.ends_with("Choice") {
      format!("{stem}Sequence{}", variant_index + 1)
    } else {
      format!("{stem}ChoiceSequence{}", variant_index + 1)
    }
  } else {
    format!("{stem}Sequence{}", variant_index + 1)
  }
}

fn one_sequence_choice_type_stem(schema_type: &SchemaType, field_name: &str) -> String {
  let owner_prefix = format!("{}_", schema_type.class_name.to_snake_case());
  if let Some(stripped) = field_name.strip_prefix(&owner_prefix) {
    format!(
      "{}{}",
      schema_type.class_name.to_upper_camel_case(),
      stripped.to_upper_camel_case()
    )
  } else {
    format!(
      "{}{}",
      schema_type.class_name.to_upper_camel_case(),
      field_name.to_upper_camel_case()
    )
  }
}

pub fn gen_schema(
  schema: &Schema,
  context: &CodegenContext<'_>,
  suppress_version_cfg_attrs: bool,
) -> Result<TokenStream> {
  let version_cfg = VersionCfgContext::new(suppress_version_cfg_attrs);
  let mut token_stream_list: Vec<TokenStream> = vec![];

  for schema_enum in &schema.enums {
    token_stream_list.push(
      gen_schema_enum(schema_enum, &schema.module_name, context, version_cfg)
        .map_err(|err| format!("enum {}: {err}", schema_enum.name))?,
    );
  }

  for schema_type in &schema.types {
    let struct_name_ident: Ident = parse_str(&schema_type.class_name.to_upper_camel_case())?;
    let type_attrs = version_cfg.attrs(&schema_type.version);
    let field_version_cfg = if type_attrs.is_empty() {
      version_cfg
    } else {
      VersionCfgContext::new(true)
    };
    let sdk_type_attrs = if schema_type.name.is_empty() {
      quote! {}
    } else {
      let qname = &schema_type.name;
      quote! {
        #[sdk(qname = #qname)]
      }
    };
    let summary_doc = format!(" {}", schema_type.summary);
    let version_doc = if schema_type.version.is_empty() {
      " Available in Office2007 and above.".to_string()
    } else {
      format!(" Available in {} and above.", schema_type.version)
    };
    let qualified_doc = if schema_type.name.ends_with('/') {
      " When the object is serialized out as xml, it's qualified name is .".to_string()
    } else {
      let qualified_str = &schema_type.name[schema_type.name.find('/').unwrap() + 1..];
      format!(" When the object is serialized out as xml, it's qualified name is {qualified_str}.")
    };

    if is_leaf_text_wrapper(schema_type) {
      let xml_content_type = gen_xml_content_type(schema_type, schema, context)?;

      if can_alias_leaf_text_wrapper(schema_type) {
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

      if has_xmlns_fields(schema_type) {
        fields.push(quote! {
          #[sdk(xmlns)]
          pub xmlns: Option<String>,
        });
        fields.push(quote! {
          #[sdk(xmlns)]
          pub xmlns_map: std::collections::HashMap<String, String>,
        });
      }
      if needs_xml_header(schema_type) {
        fields.push(quote! {
          #[sdk(xml_header)]
          pub xml_header: crate::common::XmlHeaderType,
        });
      }
      fields.extend(gen_mce_fields(schema_type));

      for attr in &schema_type.attributes {
        fields.push(
          gen_attr(
            attr,
            &schema_type.class_name,
            &schema_type.name,
            schema,
            context,
            field_version_cfg,
          )
          .map_err(|err| {
            format!(
              "type {} attr {}: {err}",
              schema_type.class_name, attr.q_name
            )
          })?,
        );
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
        #[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
        #sdk_type_attrs
        pub struct #struct_name_ident {
          #( #fields )*
        }
      });

      continue;
    }

    let mut fields: Vec<TokenStream> = vec![];
    let mut child_choice_enums: Vec<TokenStream> = vec![];
    let mut items: Vec<TokenStream> = vec![];

    if has_xmlns_fields(schema_type) {
      fields.push(quote! {
        #[sdk(xmlns)]
        pub xmlns: Option<String>,
      });
      fields.push(quote! {
        #[sdk(xmlns)]
        pub xmlns_map: std::collections::HashMap<String, String>,
      });
    }
    if needs_xml_header(schema_type) {
      fields.push(quote! {
        #[sdk(xml_header)]
        pub xml_header: crate::common::XmlHeaderType,
      });
    }
    fields.extend(gen_mce_fields(schema_type));

    if is_leaf_text_type(schema_type) {
      for attr in &schema_type.attributes {
        fields.push(
          gen_attr(
            attr,
            &schema_type.class_name,
            &schema_type.name,
            schema,
            context,
            field_version_cfg,
          )
          .map_err(|err| {
            format!(
              "type {} attr {}: {err}",
              schema_type.class_name, attr.q_name
            )
          })?,
        );
      }

      let simple_type_name = gen_xml_content_type(schema_type, schema, context)
        .map_err(|err| format!("type {} xml content: {err}", schema_type.class_name))?;
      fields.push(quote! {
        #[sdk(text)]
        pub xml_content: Option<#simple_type_name>,
      });
    } else if is_leaf_element_type(schema_type) {
      for attr in &schema_type.attributes {
        fields.push(gen_attr(
          attr,
          &schema_type.class_name,
          &schema_type.name,
          schema,
          context,
          field_version_cfg,
        )?);
      }
    } else if is_composite_type(schema_type) {
      for attr in &schema_type.attributes {
        fields.push(gen_attr(
          attr,
          &schema_type.class_name,
          &schema_type.name,
          schema,
          context,
          field_version_cfg,
        )?);
      }

      if !schema_type.text_value_type.is_empty() {
        let simple_type_name = gen_xml_content_type(schema_type, schema, context)
          .map_err(|err| format!("type {} xml content: {err}", schema_type.class_name))?;
        fields.push(quote! {
          #[sdk(text)]
          pub xml_content: Option<#simple_type_name>,
        });
      }

      if schema_type.composite_kind == SchemaTypeCompositeKind::OneAll
        && schema_type.particle.kind == "All"
      {
        fields.extend(gen_root_all_fields(
          schema_type,
          schema,
          context,
          field_version_cfg,
        )?);
      } else if schema_type.composite_kind == SchemaTypeCompositeKind::OneAll {
        fields.extend(gen_one_all_children_fields(
          schema_type,
          schema,
          context,
          field_version_cfg,
        )?);
      } else if matches!(
        schema_type.composite_kind,
        SchemaTypeCompositeKind::SdkSequence | SchemaTypeCompositeKind::OneSequence
      ) && schema_type
        .children
        .iter()
        .filter(|child| child.kind == SchemaTypeChildKind::Choice)
        .count()
        == 1
        && has_uncovered_direct_children(schema_type)
      {
        let (mixed_fields, mixed_enums, mixed_items) = gen_mixed_choice_children_fields(
          schema_type,
          schema,
          context,
          field_version_cfg,
          version_cfg,
        )?;
        fields.extend(mixed_fields);
        child_choice_enums.extend(mixed_enums);
        items.extend(mixed_items);
      } else if matches!(
        schema_type.composite_kind,
        SchemaTypeCompositeKind::SdkSequence | SchemaTypeCompositeKind::OneSequence
      ) && schema_type.children.len() == 1
        && schema_type.children[0].kind == SchemaTypeChildKind::Any
      {
        let child_choice_enum_ident: Ident = parse_str(&format!(
          "{}ChildChoice",
          schema_type.class_name.to_upper_camel_case()
        ))?;
        let field_attrs = module_version_cfg_attrs(
          effective_version("", schema_type.children[0].initial_version.as_str()),
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
            UnknownXml(std::boxed::Box<String>),
          }
        });
      } else if matches!(
        schema_type.composite_kind,
        SchemaTypeCompositeKind::SdkSequence | SchemaTypeCompositeKind::OneSequence
      ) && schema_type.children.len() == 1
        && schema_type.children[0].kind == SchemaTypeChildKind::Choice
      {
        let (mixed_fields, mixed_enums, mixed_items) = gen_mixed_choice_children_fields(
          schema_type,
          schema,
          context,
          field_version_cfg,
          version_cfg,
        )?;
        fields.extend(mixed_fields);
        child_choice_enums.extend(mixed_enums);
        items.extend(mixed_items);
      } else if matches!(
        schema_type.composite_kind,
        SchemaTypeCompositeKind::SdkSequence | SchemaTypeCompositeKind::OneSequence
      ) && schema_type.children.len() == 1
        && matches!(
          schema_type.children[0].kind,
          SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild
        )
      {
        let single_child_fields =
          gen_single_sequence_child_fields(schema_type, schema, context, field_version_cfg)?;
        fields.extend(single_child_fields);
      } else if matches!(
        schema_type.composite_kind,
        SchemaTypeCompositeKind::SdkSequence | SchemaTypeCompositeKind::OneSequence
      ) && schema_type.children.iter().all(|child| {
        matches!(
          child.kind,
          SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild
        )
      }) {
        fields.extend(gen_direct_children_fields(
          schema_type,
          schema,
          context,
          field_version_cfg,
          None,
        )?);
      } else if is_one_sequence_flatten(schema_type) {
        let (one_sequence_fields, one_sequence_enums) =
          gen_one_sequence_fields(schema_type, schema, context, field_version_cfg, version_cfg)?;
        fields.extend(one_sequence_fields);
        child_choice_enums.extend(one_sequence_enums);
      } else if is_one_sequence_structurable(schema_type) {
        let (one_sequence_fields, one_sequence_items) = gen_structured_one_sequence_fields(
          schema_type,
          schema,
          context,
          field_version_cfg,
          version_cfg,
        )?;
        fields.extend(one_sequence_fields);
        child_choice_enums.extend(one_sequence_items);
      } else {
        let (field_option, enum_option, child_items) =
          gen_children(schema_type, schema, context, field_version_cfg, version_cfg)?;

        if let Some(field) = field_option {
          fields.push(field);
        }

        if let Some(enum_option) = enum_option {
          child_choice_enums.push(quote! {
            #( #type_attrs )*
            #enum_option
          });
        }
        items.extend(child_items);
      }
    } else if is_derived_type(schema_type) {
      let base_class_type = context
        .derived_base_type(schema_type)
        .ok_or_else(|| format!("{:?}", schema_type.name))?;

      for attr in &schema_type.attributes {
        fields.push(gen_attr(
          attr,
          &schema_type.class_name,
          &schema_type.name,
          schema,
          context,
          field_version_cfg,
        )?);
      }

      for attr in &base_class_type.attributes {
        fields.push(gen_attr(
          attr,
          &schema_type.class_name,
          &schema_type.name,
          schema,
          context,
          field_version_cfg,
        )?);
      }

      if schema_type.composite_kind == SchemaTypeCompositeKind::OneAll
        && schema_type.particle.kind == "All"
      {
        fields.extend(gen_root_all_fields(
          schema_type,
          schema,
          context,
          field_version_cfg,
        )?);
      } else if schema_type.composite_kind == SchemaTypeCompositeKind::OneAll {
        fields.extend(gen_one_all_children_fields(
          schema_type,
          schema,
          context,
          field_version_cfg,
        )?);
      } else if is_one_sequence_flatten(schema_type) && is_one_sequence_flatten(base_class_type) {
        let (one_sequence_fields, one_sequence_enums) =
          gen_one_sequence_fields(schema_type, schema, context, field_version_cfg, version_cfg)?;
        fields.extend(one_sequence_fields);
        child_choice_enums.extend(one_sequence_enums);
      } else if is_one_sequence_structurable(schema_type)
        && is_one_sequence_structurable(base_class_type)
      {
        let (one_sequence_fields, one_sequence_items) = gen_structured_one_sequence_fields(
          schema_type,
          schema,
          context,
          field_version_cfg,
          version_cfg,
        )?;
        fields.extend(one_sequence_fields);
        child_choice_enums.extend(one_sequence_items);
      } else {
        let (field_option, enum_option, child_items) =
          gen_children(schema_type, schema, context, field_version_cfg, version_cfg)?;

        if let Some(field) = field_option {
          fields.push(field);
        }

        if let Some(enum_option) = enum_option {
          child_choice_enums.push(quote! {
            #( #type_attrs )*
            #enum_option
          });
        }
        items.extend(child_items);
      }

      if schema_type.children.is_empty() && is_leaf_text_type(base_class_type) {
        let simple_type_name = gen_xml_content_type(schema_type, schema, context)?;
        fields.push(quote! {
          #[sdk(text)]
          pub xml_content: Option<#simple_type_name>,
        });
      }
    } else {
      return Err(format!("{schema_type:?}").into());
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
      #[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
      #sdk_type_attrs
      pub struct #struct_name_ident {
        #( #fields )*
      }
      #( #items )*
      #child_choice_tokens
    });
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

fn gen_schema_enum(
  schema_enum: &SchemaEnum,
  _module_name: &str,
  _context: &CodegenContext<'_>,
  version_cfg: VersionCfgContext,
) -> Result<TokenStream> {
  let enum_name_ident: Ident = parse_str(&schema_enum.name.to_upper_camel_case())?;
  let enum_attrs = version_cfg.attrs(&schema_enum.version);
  let nested_version_cfg = version_cfg.child(&schema_enum.version);
  let sdk_enum_attrs = if schema_enum.r#type.is_empty() {
    quote! {}
  } else {
    let qname = &schema_enum.r#type;
    quote! {
      #[sdk(qname = #qname)]
    }
  };
  let baseline_facets: Vec<_> = schema_enum
    .facets
    .iter()
    .filter(|facet| !is_microsoft365_version(&facet.version))
    .collect();

  let default_facet = baseline_facets
    .first()
    .copied()
    .unwrap_or_else(|| schema_enum.facets.first().expect("schema enum facet"));

  let mut alias_map: HashMap<String, Vec<String>> = HashMap::new();
  for facet in &schema_enum.facets {
    for alias in &facet.aliases {
      alias_map
        .entry(facet.value.clone())
        .or_default()
        .push(alias.clone());
    }
  }

  let variants = gen_schema_enum_variants(
    &schema_enum.facets.iter().collect::<Vec<_>>(),
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

fn gen_schema_enum_variants(
  facets: &[&crate::sdk_data::sdk_data_model::SchemaEnumFacet],
  default_facet: &crate::sdk_data::sdk_data_model::SchemaEnumFacet,
  alias_map: &HashMap<String, Vec<String>>,
  version_cfg: VersionCfgContext,
) -> Result<Vec<Variant>> {
  let mut variants = Vec::with_capacity(facets.len());

  for facet in facets {
    let variant_ident: Ident = if facet.name.is_empty() {
      parse_str(&escape_upper_camel_case(facet.value.to_upper_camel_case()))?
    } else {
      parse_str(&escape_upper_camel_case(facet.name.to_upper_camel_case()))?
    };
    let variant_attrs = version_cfg.attrs(&facet.version);
    let rename_attrs = if facet.value.is_empty() {
      quote! {}
    } else {
      let value = &facet.value;
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

fn gen_attr(
  attr: &SchemaTypeAttribute,
  _owner_class_name: &str,
  _owner_type_name: &str,
  schema: &Schema,
  context: &CodegenContext<'_>,
  version_cfg: VersionCfgContext,
) -> Result<TokenStream> {
  let attr_qname = &attr.q_name;
  let attr_name_ident: Ident = if attr.property_name.is_empty() {
    parse_str(&escape_snake_case(attr.q_name.to_snake_case()))?
  } else {
    parse_str(&escape_snake_case(attr.property_name.to_snake_case()))?
  };

  let effective_attr_type = attr.r#type.as_str();

  let type_ident: Type = match classify_attr_type(effective_attr_type)
    .ok_or_else(|| effective_attr_type.to_string())?
  {
    AttrTypeKind::List => parse_str("String")?,
    AttrTypeKind::Enum { .. } => {
      let (enum_module_name, enum_name) = context.resolve_attr_enum_module(effective_attr_type)?;

      if enum_module_name != schema.module_name {
        parse_str(&format!(
          "crate::schemas::{}::{}",
          enum_module_name,
          enum_name.to_upper_camel_case()
        ))?
      } else {
        parse_str(&enum_name.to_upper_camel_case())?
      }
    }
    AttrTypeKind::Simple { simple_type, .. } => {
      parse_str(&format!("crate::simple_type::{simple_type}"))?
    }
  };

  let bit_attrs = if let Some(bit) = attr.bit {
    quote! {
      #[sdk(bit = #bit)]
    }
  } else {
    quote! {}
  };

  let required = attr.required;
  let attr_attrs = module_version_cfg_attrs(&attr.version, version_cfg);
  let sdk_attr_attrs = quote! {
    #[sdk(attr(qname = #attr_qname))]
  };
  let property_comments_doc = format!(" {}", attr.property_comments);
  let version_doc = if attr.version.is_empty() {
    " Available in Office2007 and above.".to_string()
  } else {
    format!(" Available in {} and above.", attr.version)
  };
  let qualified_doc = format!(
    " Represents the following attribute in the schema: {}",
    attr.q_name
  );

  Ok(if required {
    quote! {
      #( #attr_attrs )*
      #[doc = #property_comments_doc]
      #[doc = ""]
      #[doc = #version_doc]
      #[doc = ""]
      #[doc = #qualified_doc]
      #sdk_attr_attrs
      #bit_attrs
      pub #attr_name_ident: #type_ident,
    }
  } else {
    quote! {
      #( #attr_attrs )*
      #[doc = #property_comments_doc]
      #[doc = ""]
      #[doc = #version_doc]
      #[doc = ""]
      #[doc = #qualified_doc]
      #sdk_attr_attrs
      #bit_attrs
      pub #attr_name_ident: Option<#type_ident>,
    }
  })
}

fn gen_children(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
  field_cfg: VersionCfgContext,
  item_cfg: VersionCfgContext,
) -> Result<(Option<TokenStream>, Option<ItemEnum>, Vec<TokenStream>)> {
  let resolved_children = context.resolve_children(schema_type)?;
  if resolved_children.is_empty() {
    return Ok((None, None, Vec::new()));
  }

  let choice_version = common_choice_version(
    "",
    &resolved_children
      .iter()
      .map(|child| child.version)
      .collect::<Vec<_>>(),
  );
  let variant_cfg = if version_cfg_attrs(choice_version).is_empty() {
    item_cfg
  } else {
    VersionCfgContext::new(true)
  };
  let field_attrs = module_version_cfg_attrs(choice_version, field_cfg);
  let sdk_choice_attrs = quote! {
    #[sdk(choice)]
  };
  let choice_field_name = "children".to_string();
  let choice_field_ident: Ident = parse_str(&choice_field_name)?;
  let child_choice_enum_ident: Ident = parse_str(&format!(
    "{}ChildChoice",
    schema_type.class_name.to_upper_camel_case()
  ))?;
  let field_option = Some(quote! {
    #( #field_attrs )*
    #sdk_choice_attrs
    pub #choice_field_ident: Vec<#child_choice_enum_ident>,
  });

  let mut items: Vec<TokenStream> = vec![];
  let mut variants: Vec<TokenStream> = vec![];
  let mut default_variant: Option<(Ident, Type)> = None;

  for (variant_index, child) in resolved_children.iter().enumerate() {
    match child.kind {
      SchemaTypeChildKind::Sequence => {
        let struct_ident: Ident = parse_str(&one_sequence_choice_sequence_struct_name(
          schema_type,
          1,
          &choice_field_name,
          variant_index,
        ))?;
        let variant_ident: Ident = parse_str(&format!("Sequence{}", variant_index + 1))?;
        let mut sequence_leafs = Vec::new();
        collect_resolved_sequence_leafs(&child.children, &mut sequence_leafs);
        let sequence_property_comments = format!(
          " Sequence of {}",
          sequence_leafs
            .iter()
            .map(|field| field.name.split('/').nth(1).unwrap_or(field.name))
            .collect::<Vec<_>>()
            .join(", ")
        );
        let sequence_version = common_choice_version(
          "",
          &sequence_leafs
            .iter()
            .map(|field| field.version)
            .collect::<Vec<_>>(),
        );
        let sequence_attrs = module_version_cfg_attrs(sequence_version, item_cfg);
        let sequence_field_cfg = if sequence_attrs.is_empty() {
          item_cfg
        } else {
          VersionCfgContext::new(true)
        };
        let mut sequence_fields_data = Vec::new();
        for field in &sequence_leafs {
          let resolved_child = context
            .resolve_one_sequence_child(schema_type, field.name)
            .map_err(|err| {
              format!(
                "sequence field {:?} in {:?}: {err}",
                field.name, schema_type.name
              )
            })?;
          sequence_fields_data.push(ResolvedOneSequenceSequenceField {
            child: resolved_child,
            optional: true,
            repeated: field.repeated,
            initial_version: field.version,
          });
        }
        let sequence_variant = ResolvedOneSequenceSequenceVariant {
          variant_name: variant_ident.to_string(),
          struct_name: struct_ident.to_string(),
          property_comments: sequence_property_comments.clone(),
          fields: sequence_fields_data,
        };
        if default_variant.is_none()
          && (choice_version == sequence_version
            || (choice_version.is_empty() && !is_microsoft365_version(sequence_version)))
        {
          default_variant = Some((variant_ident.clone(), parse2(quote! { #struct_ident })?));
        }
        let sequence_fields =
          gen_sequence_variant_fields(&sequence_variant, schema, context, sequence_field_cfg)?;
        let sequence_child_qnames: Vec<&str> = sequence_variant
          .fields
          .iter()
          .map(|field| field.child.name)
          .collect();

        let child_attrs = module_version_cfg_attrs(sequence_version, variant_cfg);
        items.push(quote! {
          #( #sequence_attrs )*
          #[doc = #sequence_property_comments]
          #[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
          pub struct #struct_ident {
            #( #sequence_fields )*
          }
        });
        variants.push(quote! {
          #( #child_attrs )*
          #( #[sdk(child(qname = #sequence_child_qnames))] )*
          #variant_ident(#struct_ident),
        });
      }
      _ => {
        let child_variant_name_ident: Ident = parse_str(&child.variant_name.to_upper_camel_case())?;
        let child_qname = child.name;

        let (sdk_variant_attrs, child_variant_type, wrap_box): (TokenStream, Type, bool) =
          if child.is_any {
            (quote! { #[sdk(any)] }, parse_str("String")?, true)
          } else {
            let child_type = context
              .type_map
              .get(child.name)
              .ok_or_else(|| format!("{:?}", child.name))?;
            if can_inline_text_child(child_type) {
              (
                quote! { #[sdk(text_child(qname = #child_qname))] },
                leaf_text_wrapper_alias_type(child.name, schema, context)?,
                false,
              )
            } else {
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

              (
                quote! { #[sdk(child(qname = #child_qname))] },
                child_variant_type,
                true,
              )
            }
          };
        let child_attrs = module_version_cfg_attrs(child.version, variant_cfg);
        if wrap_box {
          variants.push(quote! {
            #( #child_attrs )*
            #sdk_variant_attrs
            #child_variant_name_ident(std::boxed::Box<#child_variant_type>),
          });
        } else {
          variants.push(quote! {
            #( #child_attrs )*
            #sdk_variant_attrs
            #child_variant_name_ident(#child_variant_type),
          });
        }
      }
    }
  }

  let enum_attrs = module_version_cfg_attrs(choice_version, item_cfg);
  let enum_tokens = quote! {
    #( #enum_attrs )*
    #[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
    pub enum #child_choice_enum_ident {
      #( #variants )*
    }
  };
  let enum_option = Some(parse2(enum_tokens.clone()).map_err(|err| {
    format!(
      "failed to generate choice enum {} for {}: {err}\n{}",
      child_choice_enum_ident, schema_type.class_name, enum_tokens
    )
  })?);

  Ok((field_option, enum_option, items))
}

fn gen_one_all_children_fields(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
  version_cfg: VersionCfgContext,
) -> Result<Vec<TokenStream>> {
  let mut fields = Vec::new();

  for child in &schema_type.children {
    if !matches!(
      child.kind,
      SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild
    ) {
      continue;
    }

    let child_type = context
      .type_map
      .get(child.name.as_str())
      .ok_or_else(|| format!("{:?}", child.name))?;

    let field_name = child_field_name(child, child_type);
    let field_name_ident: Ident = parse_str(&field_name)?;
    let property_comments = if child.property_comments.is_empty() {
      Cow::Borrowed(" _")
    } else {
      Cow::Borrowed(child.property_comments.as_str())
    };
    let version = if child.initial_version.is_empty() {
      child_type.version.as_str()
    } else {
      child.initial_version.as_str()
    };
    let field_attrs = module_version_cfg_attrs(version, version_cfg);
    let resolved_child = ResolvedOneSequenceChild {
      name: child.name.as_str(),
      field_name: Cow::Owned(field_name),
      property_comments,
      version,
      kind: child.kind,
    };
    let (sdk_field_attrs, child_variant_type, wrap_box) =
      child_field_shape(&resolved_child, schema, context)?;
    let property_comments = resolved_child.property_comments.as_ref();

    if child.repeated {
      fields.push(quote! {
        #( #field_attrs )*
        #[doc = #property_comments]
        #sdk_field_attrs
        pub #field_name_ident: Vec<#child_variant_type>,
      });
    } else {
      if wrap_box {
        fields.push(quote! {
          #( #field_attrs )*
          #[doc = #property_comments]
          #sdk_field_attrs
          pub #field_name_ident: Option<std::boxed::Box<#child_variant_type>>,
        });
      } else {
        fields.push(quote! {
          #( #field_attrs )*
          #[doc = #property_comments]
          #sdk_field_attrs
          pub #field_name_ident: Option<#child_variant_type>,
        });
      }
    }
  }

  Ok(fields)
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

fn gen_xml_content_type(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<Type> {
  if !schema_type.text_value_type.is_empty() {
    return Ok(parse_str(&format!(
      "crate::simple_type::{}",
      schema_type.text_value_type
    ))?);
  }

  let first_name = &schema_type.name[0..schema_type.name.find('/').unwrap()];

  if let Some(schema_enum) = context.enum_by_type(first_name) {
    let enum_module = context
      .enum_module_by_type(first_name)
      .ok_or_else(|| format!("{first_name:?}"))?;

    if enum_module != schema.module_name {
      Ok(parse_str(&format!(
        "crate::schemas::{}::{}",
        enum_module,
        schema_enum.name.to_upper_camel_case()
      ))?)
    } else {
      Ok(parse_str(&schema_enum.name.to_upper_camel_case())?)
    }
  } else {
    Ok(parse_str(&format!(
      "crate::simple_type::{}",
      simple_type_mapping(first_name)
    ))?)
  }
}

fn can_alias_leaf_text_wrapper(schema_type: &SchemaType) -> bool {
  is_leaf_text_wrapper(schema_type)
    && schema_type.attributes.is_empty()
    && !schema_type.has_xmlns_fields
    && !schema_type.has_mc_ignorable_field
    && !needs_xml_header(schema_type)
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

fn composite_choice_display_name<'a>(child: &'a ResolvedCompositeChild<'a>) -> &'a str {
  if child.name.is_empty() {
    child.variant_name.as_ref()
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

fn gen_mce_fields(schema_type: &SchemaType) -> Vec<TokenStream> {
  let mut fields = Vec::new();
  if schema_type.has_mc_ignorable_field {
    fields.push(quote! {
      #[sdk(mce)]
      pub mc_ignorable: Option<String>,
    });
  }

  fields
}

fn child_field_shape(
  child: &ResolvedOneSequenceChild<'_>,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<(TokenStream, Type, bool)> {
  let child_qname = child.name;

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
    return Ok((quote! { #[sdk(any)] }, parse_str("String")?, true));
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

fn gen_one_sequence_fields(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
  field_cfg: VersionCfgContext,
  item_cfg: VersionCfgContext,
) -> Result<(Vec<TokenStream>, Vec<TokenStream>)> {
  let mut fields: Vec<TokenStream> = vec![];
  let mut paragraph_prefix_fields: Vec<TokenStream> = vec![];
  let mut enums: Vec<TokenStream> = vec![];
  let mut field_name_set = std::collections::HashSet::new();
  let flat_particles = flatten_one_sequence_particles(schema_type);
  let choice_slot_count = flat_particles
    .iter()
    .filter(|particle| matches!(particle.kind, FlatParticleKind::Choice(_)))
    .count();

  for (slot_index, flat_particle) in flat_particles.into_iter().enumerate() {
    match flat_particle.kind {
      FlatParticleKind::Leaf(particle) => {
        let child = context.resolve_one_sequence_child(schema_type, particle.name.as_str())?;
        let (sdk_field_attrs, child_variant_type, wrap_box) =
          child_field_shape(&child, schema, context)?;
        let child_name_ident: Ident = parse_str(&child.field_name)?;

        if !field_name_set.insert(child_name_ident.to_string()) {
          continue;
        }

        let property_comments = child.property_comments.as_ref();
        let field_attrs = module_version_cfg_attrs(
          effective_version(child.version, flat_particle.initial_version),
          field_cfg,
        );
        if flat_particle.repeated {
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
        } else if flat_particle.optional {
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
      FlatParticleKind::Choice(choice_particle) => {
        let choice = context.resolve_one_sequence_choice(
          schema_type,
          choice_particle,
          choice_slot_count,
          slot_index,
        )?;
        let choice_field_ident: Ident = parse_str(&choice.field_name)?;

        if !field_name_set.insert(choice_field_ident.to_string()) {
          continue;
        }

        let choice_enum_ident: Ident = parse_str(&choice.enum_name)?;
        let property_comments = choice.property_comments.as_str();
        let choice_version = common_choice_version(
          flat_particle.initial_version,
          &choice
            .variants
            .iter()
            .map(|variant| variant.version)
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
          let (sdk_variant_attrs, variant_type, wrap_box) =
            choice_child_variant_shape(variant, schema, context)?;
          let variant_ident: Ident = parse_str(&variant.field_name.to_upper_camel_case())?;
          let variant_attrs = module_version_cfg_attrs(variant.version, variant_cfg);
          if default_variant.is_none()
            && (choice_version == variant.version
              || (choice_version.is_empty() && !is_microsoft365_version(variant.version)))
          {
            default_variant = Some((variant_ident.clone(), variant_type.clone()));
          }
          let variant_tokens = if wrap_box {
            quote! {
              #( #variant_attrs )*
              #sdk_variant_attrs
              #variant_ident(std::boxed::Box<#variant_type>),
            }
          } else {
            quote! {
              #( #variant_attrs )*
              #sdk_variant_attrs
              #variant_ident(#variant_type),
            }
          };
          variants.push(variant_tokens);
        }

        let default_variant_ident = default_variant
          .or_else(|| {
            choice.variants.first().and_then(|variant| {
              let (_, variant_type, _) =
                choice_child_variant_shape(variant, schema, context).ok()?;
              let variant_ident =
                parse_str::<Ident>(&variant.field_name.to_upper_camel_case()).ok()?;
              Some((variant_ident, variant_type))
            })
          })
          .map(|(variant_ident, _variant_type)| variant_ident)
          .ok_or_else(|| choice.enum_name.clone())?;
        let enum_item = quote! {
          #( #enum_attrs )*
          #[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
          pub enum #choice_enum_ident {
            #( #variants )*
          }
        };
        enums.push(enum_item);
        enums.push(gen_choice_default_impl(
          &choice_enum_ident,
          &default_variant_ident,
          enum_attrs.clone(),
        ));

        if flat_particle.repeated {
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

  Ok((fields, enums))
}

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

  for (slot_index, particle) in structured_particles.into_iter().enumerate() {
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
          slot_index,
        )?;
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
                default_variant = Some((variant_ident.clone(), variant_type.clone()));
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
                default_variant = Some((variant_ident.clone(), parse2(quote! { #struct_ident })?));
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

        let (default_variant_ident, _default_variant_type) = default_variant
          .or_else(|| {
            choice.variants.first().and_then(|variant| match variant {
              ResolvedOneSequenceChoiceVariant::Leaf(child) => Some((
                parse_str::<Ident>(&child.field_name.to_upper_camel_case()).ok()?,
                one_sequence_child_variant_type(child, schema, context).ok()?,
              )),
              ResolvedOneSequenceChoiceVariant::Sequence(sequence_variant) => Some((
                parse_str::<Ident>(&sequence_variant.variant_name).ok()?,
                parse_str::<Type>(&sequence_variant.struct_name).ok()?,
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
        items.push(gen_choice_default_impl(
          &choice_enum_ident,
          &default_variant_ident,
          enum_attrs.clone(),
        ));

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

fn gen_single_sequence_child_fields(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
  field_cfg: VersionCfgContext,
) -> Result<Vec<TokenStream>> {
  let mut fields = Vec::new();
  let Some(child) = schema_type.children.first() else {
    return Ok(fields);
  };

  let resolved_child = context.resolve_one_sequence_child(schema_type, child.name.as_str())?;
  let (sdk_field_attrs, child_variant_type, wrap_box) =
    child_field_shape(&resolved_child, schema, context)?;
  let child_name_ident: Ident = parse_str(&resolved_child.field_name)?;
  let property_comments = resolved_child.property_comments.as_ref();
  let field_attrs = module_version_cfg_attrs(
    effective_version(resolved_child.version, child.initial_version.as_str()),
    field_cfg,
  );
  if child.repeated {
    fields.push(quote! {
      #( #field_attrs )*
      #[doc = #property_comments]
      #sdk_field_attrs
      pub #child_name_ident: Vec<#child_variant_type>,
    });
  } else if child.optional {
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
  } else if wrap_box {
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

  Ok(fields)
}

fn gen_direct_children_fields(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
  field_cfg: VersionCfgContext,
  skip_names: Option<&std::collections::HashSet<String>>,
) -> Result<Vec<TokenStream>> {
  let mut fields = Vec::new();
  let mut field_name_set = std::collections::HashSet::new();

  for child in &schema_type.children {
    if !matches!(
      child.kind,
      SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild
    ) {
      continue;
    }

    if let Some(skip_names) = skip_names
      && skip_names.contains(child.name.as_str())
    {
      continue;
    }

    let child_type = context
      .type_map
      .get(child.name.as_str())
      .ok_or_else(|| format!("{:?}", child.name))?;
    let field_name = child_field_name(child, child_type);
    if !field_name_set.insert(field_name.clone()) {
      continue;
    }

    let field_name_ident: Ident = parse_str(&field_name)?;
    let property_comments = if child.property_comments.is_empty() {
      Cow::Borrowed(" _")
    } else {
      Cow::Borrowed(child.property_comments.as_str())
    };
    let version = if child.initial_version.is_empty() {
      child_type.version.as_str()
    } else {
      child.initial_version.as_str()
    };
    let field_attrs = module_version_cfg_attrs(version, field_cfg);
    let resolved_child = ResolvedOneSequenceChild {
      name: child.name.as_str(),
      field_name: Cow::Owned(field_name),
      property_comments,
      version,
      kind: child.kind,
    };
    let (sdk_field_attrs, child_variant_type, wrap_box) =
      child_field_shape(&resolved_child, schema, context)?;
    let property_comments = resolved_child.property_comments.as_ref();

    if child.repeated {
      fields.push(quote! {
        #( #field_attrs )*
        #[doc = #property_comments]
        #sdk_field_attrs
        pub #field_name_ident: Vec<#child_variant_type>,
      });
    } else if child.optional {
      if wrap_box {
        fields.push(quote! {
          #( #field_attrs )*
          #[doc = #property_comments]
          #sdk_field_attrs
          pub #field_name_ident: Option<std::boxed::Box<#child_variant_type>>,
        });
      } else {
        fields.push(quote! {
          #( #field_attrs )*
          #[doc = #property_comments]
          #sdk_field_attrs
          pub #field_name_ident: Option<#child_variant_type>,
        });
      }
    } else if wrap_box {
      fields.push(quote! {
        #( #field_attrs )*
        #[doc = #property_comments]
        #sdk_field_attrs
        pub #field_name_ident: std::boxed::Box<#child_variant_type>,
      });
    } else {
      fields.push(quote! {
        #( #field_attrs )*
        #[doc = #property_comments]
        #sdk_field_attrs
        pub #field_name_ident: #child_variant_type,
      });
    }
  }

  Ok(fields)
}

fn build_direct_child_field_token(
  child: &SchemaTypeChild,
  schema: &Schema,
  context: &CodegenContext<'_>,
  field_cfg: VersionCfgContext,
  field_name_set: &mut std::collections::HashSet<String>,
) -> Result<Option<TokenStream>> {
  if !matches!(
    child.kind,
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild
  ) {
    return Ok(None);
  }

  let child_type = context
    .type_map
    .get(child.name.as_str())
    .ok_or_else(|| format!("{:?}", child.name))?;
  let field_name = child_field_name(child, child_type);
  if !field_name_set.insert(field_name.clone()) {
    return Ok(None);
  }

  let field_name_ident: Ident = parse_str(&field_name)?;
  let property_comments = if child.property_comments.is_empty() {
    Cow::Borrowed(" _")
  } else {
    Cow::Borrowed(child.property_comments.as_str())
  };
  let version = if child.initial_version.is_empty() {
    child_type.version.as_str()
  } else {
    child.initial_version.as_str()
  };
  let field_attrs = module_version_cfg_attrs(version, field_cfg);
  let resolved_child = ResolvedOneSequenceChild {
    name: child.name.as_str(),
    field_name: Cow::Owned(field_name),
    property_comments,
    version,
    kind: child.kind,
  };
  let (sdk_field_attrs, child_variant_type, wrap_box) =
    child_field_shape(&resolved_child, schema, context)?;
  let property_comments = resolved_child.property_comments.as_ref();

  let field_tokens = if child.repeated {
    quote! {
      #( #field_attrs )*
      #[doc = #property_comments]
      #sdk_field_attrs
      pub #field_name_ident: Vec<#child_variant_type>,
    }
  } else if child.optional {
    if wrap_box {
      quote! {
        #( #field_attrs )*
        #[doc = #property_comments]
        #sdk_field_attrs
        pub #field_name_ident: Option<std::boxed::Box<#child_variant_type>>,
      }
    } else {
      quote! {
        #( #field_attrs )*
        #[doc = #property_comments]
        #sdk_field_attrs
        pub #field_name_ident: Option<#child_variant_type>,
      }
    }
  } else if wrap_box {
    quote! {
      #( #field_attrs )*
      #[doc = #property_comments]
      #sdk_field_attrs
      pub #field_name_ident: std::boxed::Box<#child_variant_type>,
    }
  } else {
    quote! {
      #( #field_attrs )*
      #[doc = #property_comments]
      #sdk_field_attrs
      pub #field_name_ident: #child_variant_type,
    }
  };

  Ok(Some(field_tokens))
}

fn gen_mixed_choice_children_fields(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
  field_cfg: VersionCfgContext,
  item_cfg: VersionCfgContext,
) -> Result<(Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>)> {
  let mut fields = Vec::new();
  let mut enums = Vec::new();
  let mut items = Vec::new();
  let mut field_name_set = std::collections::HashSet::new();
  let Some(choice_child) = schema_type
    .children
    .iter()
    .find(|child| child.kind == SchemaTypeChildKind::Choice)
  else {
    return Ok((fields, enums, items));
  };

  let mut choice_variants = Vec::new();
  let mut choice_leaf_names = std::collections::HashSet::new();
  context.collect_resolved_children(
    schema_type,
    &choice_child.children,
    &mut choice_variants,
    &mut choice_leaf_names,
    true,
  )?;
  if choice_variants.is_empty() {
    return Ok((fields, enums, items));
  }

  let choice_slot_count = 1;
  let choice_field_name = one_sequence_choice_field_name(schema_type, choice_slot_count, 0);
  let choice_enum_name =
    one_sequence_choice_enum_name(schema_type, choice_slot_count, &choice_field_name);
  let choice_field_ident: Ident = parse_str(&choice_field_name)?;
  let choice_enum_ident: Ident = parse_str(&choice_enum_name)?;
  let choice_version = common_choice_version(
    choice_child.initial_version.as_str(),
    &choice_variants
      .iter()
      .map(|child| child.version)
      .collect::<Vec<_>>(),
  );
  let field_attrs = module_version_cfg_attrs(choice_version, field_cfg);
  let enum_attrs = module_version_cfg_attrs(choice_version, item_cfg);
  let variant_cfg = if enum_attrs.is_empty() {
    item_cfg
  } else {
    VersionCfgContext::new(true)
  };
  let sdk_choice_attrs = quote! {
    #[sdk(choice)]
  };

  let mut variants = Vec::new();
  let mut default_variant: Option<Ident> = None;
  for (variant_index, child) in choice_variants.iter().enumerate() {
    match child.kind {
      SchemaTypeChildKind::Sequence => {
        let struct_ident: Ident = parse_str(&one_sequence_choice_sequence_struct_name(
          schema_type,
          choice_slot_count,
          &choice_field_name,
          variant_index,
        ))?;
        let variant_ident: Ident = parse_str(&format!("Sequence{}", variant_index + 1))?;
        let mut sequence_leafs = Vec::new();
        collect_resolved_sequence_leafs(&child.children, &mut sequence_leafs);
        let sequence_property_comments = format!(
          " Sequence of {}",
          sequence_leafs
            .iter()
            .map(|field| field.name.split('/').nth(1).unwrap_or(field.name))
            .collect::<Vec<_>>()
            .join(", ")
        );
        let sequence_version = common_choice_version(
          "",
          &sequence_leafs
            .iter()
            .map(|field| field.version)
            .collect::<Vec<_>>(),
        );
        let sequence_attrs = module_version_cfg_attrs(sequence_version, item_cfg);
        let sequence_field_cfg = if sequence_attrs.is_empty() {
          item_cfg
        } else {
          VersionCfgContext::new(true)
        };
        let mut sequence_fields_data = Vec::new();
        for field in &sequence_leafs {
          let resolved_child = context
            .resolve_one_sequence_child(schema_type, field.name)
            .map_err(|err| {
              format!(
                "sequence field {:?} in {:?}: {err}",
                field.name, schema_type.name
              )
            })?;
          sequence_fields_data.push(ResolvedOneSequenceSequenceField {
            child: resolved_child,
            optional: true,
            repeated: false,
            initial_version: field.version,
          });
        }
        let sequence_variant = ResolvedOneSequenceSequenceVariant {
          variant_name: variant_ident.to_string(),
          struct_name: struct_ident.to_string(),
          property_comments: sequence_property_comments.clone(),
          fields: sequence_fields_data,
        };
        if default_variant.is_none()
          && (choice_version == sequence_version
            || (choice_version.is_empty() && !is_microsoft365_version(sequence_version)))
        {
          default_variant = Some(variant_ident.clone());
        }
        let sequence_fields =
          gen_sequence_variant_fields(&sequence_variant, schema, context, sequence_field_cfg)?;
        let child_attrs = module_version_cfg_attrs(sequence_version, variant_cfg);
        let sequence_child_qnames: Vec<&str> = sequence_variant
          .fields
          .iter()
          .map(|field| field.child.name)
          .collect();
        items.push(quote! {
          #( #sequence_attrs )*
          #[doc = #sequence_property_comments]
          #[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
          pub struct #struct_ident {
            #( #sequence_fields )*
          }
        });
        variants.push(quote! {
          #( #child_attrs )*
          #( #[sdk(child(qname = #sequence_child_qnames))] )*
          #variant_ident(#struct_ident),
        });
      }
      _ => {
        let synthetic_child = ResolvedOneSequenceChild {
          name: child.name,
          field_name: Cow::Owned(escape_snake_case(
            child.variant_name.to_string().to_snake_case(),
          )),
          property_comments: Cow::Borrowed(" _"),
          version: child.version,
          kind: child.kind,
        };
        let variant_ident: Ident = parse_str(&synthetic_child.field_name.to_upper_camel_case())?;
        let variant_attrs = module_version_cfg_attrs(child.version, variant_cfg);
        let (sdk_variant_attrs, variant_type, wrap_box) =
          choice_child_variant_shape(&synthetic_child, schema, context)?;
        if default_variant.is_none()
          && (choice_version == child.version
            || (choice_version.is_empty() && !is_microsoft365_version(child.version)))
        {
          default_variant = Some(variant_ident.clone());
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
    }
  }

  let default_variant_ident = default_variant
    .or_else(|| {
      choice_variants.first().and_then(|child| match child.kind {
        SchemaTypeChildKind::Sequence => parse_str::<Ident>("Sequence1").ok(),
        _ => parse_str::<Ident>(
          &child
            .variant_name
            .to_string()
            .to_snake_case()
            .to_upper_camel_case(),
        )
        .ok(),
      })
    })
    .ok_or_else(|| choice_enum_name.clone())?;

  enums.push(quote! {
    #( #enum_attrs )*
    #[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
    pub enum #choice_enum_ident {
      #( #variants )*
    }
  });
  enums.push(gen_choice_default_impl(
    &choice_enum_ident,
    &default_variant_ident,
    enum_attrs.clone(),
  ));

  let choice_property_comments: Cow<'_, str> = {
    let names = choice_variants
      .iter()
      .map(|child| match child.kind {
        SchemaTypeChildKind::Sequence => {
          let mut sequence_leafs = Vec::new();
          collect_resolved_sequence_leafs(&child.children, &mut sequence_leafs);
          format!(
            "sequence of {}",
            sequence_leafs
              .iter()
              .map(|field| field.name.split('/').nth(1).unwrap_or(field.name))
              .collect::<Vec<_>>()
              .join(", ")
          )
        }
        _ => composite_choice_display_name(child).to_string(),
      })
      .collect::<Vec<_>>()
      .join(", ");
    Cow::Owned(format!("Choice of {names}"))
  };
  let choice_field_tokens = if choice_child.repeated {
    quote! {
      #( #field_attrs )*
      #[doc = #choice_property_comments]
      #sdk_choice_attrs
      pub #choice_field_ident: Vec<#choice_enum_ident>,
    }
  } else {
    quote! {
      #( #field_attrs )*
      #[doc = #choice_property_comments]
      #sdk_choice_attrs
      pub #choice_field_ident: Option<#choice_enum_ident>,
    }
  };

  let choice_index = schema_type
    .children
    .iter()
    .position(|child| child.kind == SchemaTypeChildKind::Choice)
    .ok_or_else(|| schema_type.class_name.clone())?;

  for child in &schema_type.children[..choice_index] {
    if let Some(field_tokens) =
      build_direct_child_field_token(child, schema, context, field_cfg, &mut field_name_set)?
    {
      fields.push(field_tokens);
    }
  }

  fields.push(choice_field_tokens);

  for child in &schema_type.children[choice_index + 1..] {
    if let Some(field_tokens) =
      build_direct_child_field_token(child, schema, context, field_cfg, &mut field_name_set)?
    {
      fields.push(field_tokens);
    }
  }
  Ok((fields, enums, items))
}

fn has_uncovered_direct_children(schema_type: &SchemaType) -> bool {
  count_uncovered_direct_children(schema_type) > 0
}

fn count_uncovered_direct_children(schema_type: &SchemaType) -> usize {
  let Some(choice_child) = schema_type
    .children
    .iter()
    .find(|child| child.kind == SchemaTypeChildKind::Choice)
  else {
    return 0;
  };

  let mut choice_leaf_names = std::collections::HashSet::new();
  collect_choice_child_leaf_names(choice_child, &mut choice_leaf_names);

  schema_type
    .children
    .iter()
    .filter(|child| {
      matches!(
        child.kind,
        SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild
      ) && !choice_leaf_names.contains(child.name.as_str())
    })
    .count()
}

fn collect_choice_child_leaf_names(
  child: &crate::sdk_data::sdk_data_model::SchemaTypeChild,
  out: &mut std::collections::HashSet<String>,
) {
  match child.kind {
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any => {
      if !child.name.is_empty() {
        out.insert(child.name.clone());
      }
    }
    SchemaTypeChildKind::Choice | SchemaTypeChildKind::Sequence => {
      for item in &child.children {
        collect_choice_child_leaf_names(item, out);
      }
    }
  }
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

fn gen_choice_default_impl(
  choice_enum_ident: &Ident,
  default_variant_ident: &Ident,
  enum_attrs: Vec<Attribute>,
) -> TokenStream {
  quote! {
    #( #enum_attrs )*
    impl Default for #choice_enum_ident {
      fn default() -> Self {
        Self::#default_variant_ident(Default::default())
      }
    }
  }
}

fn gen_root_all_fields(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
  version_cfg: VersionCfgContext,
) -> Result<Vec<TokenStream>> {
  let mut fields = Vec::new();

  for particle in &schema_type.particle.items {
    let resolved_child = context.resolve_one_sequence_child(schema_type, particle.name.as_str())?;
    let (sdk_field_attrs, child_variant_type, wrap_box) =
      child_field_shape(&resolved_child, schema, context)?;

    let child_name_ident: Ident = parse_str(&resolved_child.field_name)?;
    let property_comments = resolved_child.property_comments.as_ref();
    let field_attrs = module_version_cfg_attrs(
      effective_version(resolved_child.version, particle.initial_version.as_str()),
      version_cfg,
    );

    let repeated = particle
      .occurs
      .first()
      .is_some_and(|occur| occur.max.is_none() || occur.max.is_some_and(|max| max > 1));
    let optional = particle
      .occurs
      .first()
      .is_some_and(|occur| occur.min.is_none() || occur.min == Some(0));

    if repeated {
      fields.push(quote! {
        #( #field_attrs )*
        #[doc = #property_comments]
        #sdk_field_attrs
        pub #child_name_ident: Vec<#child_variant_type>,
      });
    } else if optional {
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
  use crate::sdk_data::sdk_data_model::{
    Schema, SchemaEnumFacet, SchemaType, SchemaTypeChild, SchemaTypeCompositeKind, SchemaTypeKind,
    SchemaTypeParticle, SchemaTypeParticleOccur,
  };

  #[test]
  fn gen_schema_enum_emits_sdk_enum_attributes() {
    let schema_enum = SchemaEnum {
      name: "SizeValues".to_string(),
      r#type: "mso:ST_Size".to_string(),
      facets: vec![
        SchemaEnumFacet {
          name: String::new(),
          value: "normal".to_string(),
          version: String::new(),
          ..Default::default()
        },
        SchemaEnumFacet {
          name: String::new(),
          value: "large".to_string(),
          version: String::new(),
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let tokens = gen_schema_enum(
      &schema_enum,
      "",
      &CodegenContext::new(&[]),
      VersionCfgContext::default(),
    )
    .expect("schema enum tokens")
    .to_string();

    assert!(tokens.contains("qname"));
    assert!(tokens.contains("mso:ST_Size"));
    assert!(tokens.contains("ooxmlsdk_derive"));
    assert!(tokens.contains("SdkEnum"));
    assert!(tokens.contains("rename"));
    assert!(tokens.contains("normal"));
    assert!(tokens.contains("large"));
  }

  #[test]
  fn gen_schema_enum_keeps_mixed_version_variants_in_one_enum() {
    let schema_enum = SchemaEnum {
      name: "TriggerEventValues".to_string(),
      r#type: "p:ST_TLTriggerEvent".to_string(),
      facets: vec![
        SchemaEnumFacet {
          name: String::new(),
          value: "none".to_string(),
          version: String::new(),
          ..Default::default()
        },
        SchemaEnumFacet {
          name: String::new(),
          value: "onBegin".to_string(),
          version: String::new(),
          ..Default::default()
        },
        SchemaEnumFacet {
          name: String::new(),
          value: "onMediaBookmark".to_string(),
          version: "Office2010".to_string(),
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let tokens = gen_schema_enum(
      &schema_enum,
      "",
      &CodegenContext::new(&[]),
      VersionCfgContext::default(),
    )
    .expect("schema enum tokens")
    .to_string();

    assert_eq!(tokens.matches("pub enum TriggerEventValues").count(), 1);
    assert!(tokens.contains("OnMediaBookmark"));
    assert!(tokens.contains("cfg"));
    assert!(tokens.contains("microsoft365"));
  }

  #[test]
  fn gen_schema_flattens_one_all_root_into_named_children() {
    let schema_type = SchemaType {
      name: "w:CT_OdsoFieldMapData/w:fieldMapData".to_string(),
      class_name: "FieldMapData".to_string(),
      kind: SchemaTypeKind::Composite,
      composite_kind: SchemaTypeCompositeKind::OneAll,
      particle: SchemaTypeParticle {
        kind: "All".to_string(),
        items: vec![
          SchemaTypeParticle {
            name: "w:CT_String/w:name".to_string(),
            occurs: vec![SchemaTypeParticleOccur {
              min: Some(0),
              max: Some(1),
              ..Default::default()
            }],
            ..Default::default()
          },
          SchemaTypeParticle {
            name: "w:CT_String/w:mappedName".to_string(),
            occurs: vec![SchemaTypeParticleOccur {
              min: Some(0),
              max: Some(1),
              ..Default::default()
            }],
            ..Default::default()
          },
        ],
        ..Default::default()
      },
      children: vec![
        SchemaTypeChild {
          name: "w:CT_String/w:name".to_string(),
          property_name: "Name".to_string(),
          property_comments: "Name".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Child,
          optional: false,
          repeated: false,
          initial_version: String::new(),
          children: Vec::new(),
        },
        SchemaTypeChild {
          name: "w:CT_String/w:mappedName".to_string(),
          property_name: "MappedName".to_string(),
          property_comments: "Mapped Name".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Child,
          optional: false,
          repeated: false,
          initial_version: String::new(),
          children: Vec::new(),
        },
      ],
      ..Default::default()
    };
    let schema = Schema {
      prefix: "w".to_string(),
      module_name: "schemas_openxmlformats_org_wordprocessingml_2006_main".to_string(),
      types: vec![
        schema_type.clone(),
        SchemaType {
          name: "w:CT_String/w:name".to_string(),
          class_name: "Name".to_string(),
          kind: SchemaTypeKind::Composite,
          ..Default::default()
        },
        SchemaType {
          name: "w:CT_String/w:mappedName".to_string(),
          class_name: "MappedName".to_string(),
          kind: SchemaTypeKind::Composite,
          ..Default::default()
        },
      ],
      ..Default::default()
    };

    let tokens = gen_schema(
      &schema,
      &CodegenContext::new(std::slice::from_ref(&schema)),
      false,
    )
    .expect("schema tokens")
    .to_string();
    let compact: String = tokens.chars().filter(|ch| !ch.is_whitespace()).collect();

    assert!(compact.contains("pubname:Option<std::boxed::Box<Name>>"));
    assert!(compact.contains("pubmapped_name:Option<std::boxed::Box<MappedName>>"));
    assert!(!compact.contains("FieldMapDataChildChoice"));
    assert!(!compact.contains("pubchildren:Vec<"));
  }
}
