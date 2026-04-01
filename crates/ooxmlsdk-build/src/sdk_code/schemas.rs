use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;
use std::collections::HashMap;
use syn::{Ident, ItemEnum, Type, Variant, parse_str, parse2};

use crate::Result;
use crate::sdk_code::helpers::{
  AttrTypeKind, FlatParticleKind, StructuredChoice, StructuredChoiceVariant,
  StructuredParticleKind, classify_attr_type, flatten_one_sequence_particles, is_composite_type,
  is_derived_type, is_leaf_element_type, is_leaf_text_type, is_leaf_text_wrapper,
  is_one_sequence_flatten, is_one_sequence_structurable, structure_one_sequence_particles,
  supports_xmlns_fields,
};
use crate::sdk_code::versioning::{
  effective_version, is_microsoft365_version, not_microsoft365_cfg_attrs, version_cfg_attrs,
};
use crate::sdk_data::sdk_data_model::{
  Schema, SchemaEnum, SchemaType, SchemaTypeAttribute, SchemaTypeParticle,
};
use crate::simple_type::simple_type_mapping;
use crate::utils::{escape_snake_case, escape_upper_camel_case};

#[derive(Debug)]
pub struct CodegenContext<'a> {
  enum_type_map: HashMap<&'a str, &'a SchemaEnum>,
  enum_type_module_map: HashMap<&'a str, &'a str>,
  type_map: HashMap<&'a str, &'a SchemaType>,
  type_module_map: HashMap<&'a str, &'a str>,
  type_prefix_map: HashMap<&'a str, &'a str>,
  enum_module_by_typed_namespace_and_name: HashMap<(&'a str, &'a str), &'a str>,
}

#[derive(Debug)]
pub struct ResolvedOneSequenceChild<'a> {
  pub name: &'a str,
  pub field_name: Cow<'a, str>,
  pub property_comments: Cow<'a, str>,
  pub version: &'a str,
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
}

impl<'a> CodegenContext<'a> {
  pub fn new(schemas: &'a [Schema]) -> Self {
    let mut enum_type_map = HashMap::new();
    let mut enum_type_module_map = HashMap::new();
    let mut type_map = HashMap::new();
    let mut type_module_map = HashMap::new();
    let mut type_prefix_map = HashMap::new();
    let mut enum_module_by_typed_namespace_and_name = HashMap::new();

    for schema in schemas {
      for schema_type in &schema.types {
        type_map.insert(schema_type.name.as_str(), schema_type);
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
      let field_name = if child.property_name.is_empty() {
        let child_last_name = child
          .name
          .split('/')
          .nth(1)
          .ok_or_else(|| child.name.clone())?;
        Cow::Owned(child_last_name.to_snake_case())
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
        version: child_type.version.as_str(),
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
    })
  }

  pub fn resolve_one_sequence_choice(
    &self,
    schema_type: &'a SchemaType,
    choice_particle: &'a SchemaTypeParticle,
    choice_slot_count: usize,
    slot_index: usize,
  ) -> Result<ResolvedOneSequenceChoice<'a>> {
    let mut variants = Vec::new();

    for particle in &choice_particle.items {
      variants.push(self.resolve_one_sequence_child(schema_type, particle.name.as_str())?);
    }

    let field_name = one_sequence_choice_field_name(schema_type, choice_slot_count, slot_index);
    let enum_name = one_sequence_choice_enum_name(schema_type, choice_slot_count, &field_name);
    let property_comments = format!(
      "Choice of {}",
      variants
        .iter()
        .map(|child| child.name.split('/').nth(1).unwrap_or(child.name))
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

    for child in &schema_type.children {
      let child_last_name = child
        .name
        .split('/')
        .nth(1)
        .ok_or_else(|| child.name.clone())?;

      resolved.push(ResolvedCompositeChild {
        name: child.name.as_str(),
        variant_name: Cow::Borrowed(child_last_name),
        version: self
          .type_by_name(child.name.as_str())
          .map(|item| item.version.as_str())
          .unwrap_or_default(),
      });
    }

    for particle in &schema_type.particle.items {
      if particle.name.is_empty()
        || schema_type
          .children
          .iter()
          .any(|child| child.name == particle.name)
      {
        continue;
      }

      let particle_last_name = particle
        .name
        .split('/')
        .nth(1)
        .ok_or_else(|| particle.name.clone())?;

      resolved.push(ResolvedCompositeChild {
        name: particle.name.as_str(),
        variant_name: Cow::Borrowed(particle_last_name),
        version: self
          .type_by_name(particle.name.as_str())
          .map(|item| effective_version(item.version.as_str(), &particle.initial_version))
          .unwrap_or(&particle.initial_version),
      });
    }

    Ok(resolved)
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
  if choice_slot_count <= 1 {
    format!("{}Choice", schema_type.class_name.to_upper_camel_case())
  } else {
    format!(
      "{}{}Choice",
      schema_type.class_name.to_upper_camel_case(),
      field_name.to_upper_camel_case()
    )
  }
}

fn one_sequence_choice_sequence_struct_name(
  schema_type: &SchemaType,
  choice_slot_count: usize,
  field_name: &str,
  variant_index: usize,
) -> String {
  if choice_slot_count <= 1 {
    format!(
      "{}ChoiceSequence{}",
      schema_type.class_name.to_upper_camel_case(),
      variant_index + 1
    )
  } else {
    format!(
      "{}{}Sequence{}",
      schema_type.class_name.to_upper_camel_case(),
      field_name.to_upper_camel_case(),
      variant_index + 1
    )
  }
}

pub fn gen_schema(schema: &Schema, context: &CodegenContext<'_>) -> Result<TokenStream> {
  let mut token_stream_list: Vec<TokenStream> = vec![];

  for schema_enum in &schema.enums {
    token_stream_list.push(gen_schema_enum(schema_enum)?);
  }

  for schema_type in &schema.types {
    let struct_name_ident: Ident = parse_str(&schema_type.class_name.to_upper_camel_case())?;
    let type_attrs = version_cfg_attrs(&schema_type.version);
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

      token_stream_list.push(quote! {
        #( #type_attrs )*
        #[doc = #summary_doc]
        #[doc = ""]
        #[doc = #version_doc]
        #[doc = ""]
        #[doc = #qualified_doc]
        #[derive(Clone, Debug, Default)]
        pub struct #struct_name_ident(pub Option<#xml_content_type>);

        #( #type_attrs )*
        impl From<#xml_content_type> for #struct_name_ident {
          fn from(value: #xml_content_type) -> Self {
            Self(Some(value))
          }
        }

        #( #type_attrs )*
        impl From<Option<#xml_content_type>> for #struct_name_ident {
          fn from(value: Option<#xml_content_type>) -> Self {
            Self(value)
          }
        }

        #( #type_attrs )*
        impl std::ops::Deref for #struct_name_ident {
          type Target = Option<#xml_content_type>;

          fn deref(&self) -> &Self::Target {
            &self.0
          }
        }

        #( #type_attrs )*
        impl std::ops::DerefMut for #struct_name_ident {
          fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
          }
        }
      });

      continue;
    }

    let mut fields: Vec<TokenStream> = vec![];
    let mut child_choice_enums: Vec<TokenStream> = vec![];

    if is_leaf_text_type(schema_type) {
      for attr in &schema_type.attributes {
        fields.push(gen_attr(attr, schema, context)?);
      }

      let simple_type_name = gen_xml_content_type(schema_type, schema, context)?;
      fields.push(quote! {
        pub xml_content: Option<#simple_type_name>,
      });
    } else if is_leaf_element_type(schema_type) {
      for attr in &schema_type.attributes {
        fields.push(gen_attr(attr, schema, context)?);
      }
    } else if is_composite_type(schema_type) {
      if supports_xmlns_fields(schema_type, schema) {
        fields.push(quote! {
          pub xmlns: Option<String>,
        });
        fields.push(quote! {
          pub xmlns_map: std::collections::HashMap<String, String>,
        });
        fields.push(quote! {
          pub mc_ignorable: Option<String>,
        });
      }

      for attr in &schema_type.attributes {
        fields.push(gen_attr(attr, schema, context)?);
      }

      if is_one_sequence_flatten(schema_type) {
        let (one_sequence_fields, one_sequence_enums) =
          gen_one_sequence_fields(schema_type, schema, context)?;
        fields.extend(one_sequence_fields);
        child_choice_enums.extend(one_sequence_enums);
      } else if is_one_sequence_structurable(schema_type) {
        let (one_sequence_fields, one_sequence_items) =
          gen_structured_one_sequence_fields(schema_type, schema, context)?;
        fields.extend(one_sequence_fields);
        child_choice_enums.extend(one_sequence_items);
      } else {
        let (field_option, enum_option) = gen_children(schema_type, schema, context)?;

        if let Some(field) = field_option {
          fields.push(field);
        }

        if let Some(enum_option) = enum_option {
          child_choice_enums.push(quote! {
            #( #type_attrs )*
            #enum_option
          });
        }
      }
    } else if is_derived_type(schema_type) {
      let base_class_type = context
        .derived_base_type(schema_type)
        .ok_or_else(|| format!("{:?}", schema_type.name))?;

      for attr in &schema_type.attributes {
        fields.push(gen_attr(attr, schema, context)?);
      }

      for attr in &base_class_type.attributes {
        fields.push(gen_attr(attr, schema, context)?);
      }

      if is_one_sequence_flatten(schema_type) && is_one_sequence_flatten(base_class_type) {
        let (one_sequence_fields, one_sequence_enums) =
          gen_one_sequence_fields(schema_type, schema, context)?;
        fields.extend(one_sequence_fields);
        child_choice_enums.extend(one_sequence_enums);
      } else if is_one_sequence_structurable(schema_type)
        && is_one_sequence_structurable(base_class_type)
      {
        let (one_sequence_fields, one_sequence_items) =
          gen_structured_one_sequence_fields(schema_type, schema, context)?;
        fields.extend(one_sequence_fields);
        child_choice_enums.extend(one_sequence_items);
      } else {
        let (field_option, enum_option) = gen_children(schema_type, schema, context)?;

        if let Some(field) = field_option {
          fields.push(field);
        }

        if let Some(enum_option) = enum_option {
          child_choice_enums.push(quote! {
            #( #type_attrs )*
            #enum_option
          });
        }
      }

      if schema_type.children.is_empty() && is_leaf_text_type(base_class_type) {
        let simple_type_name = gen_xml_content_type(base_class_type, schema, context)?;
        fields.push(quote! {
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
      #[derive(Clone, Debug, Default)]
      pub struct #struct_name_ident {
        #( #fields )*
      }

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

fn gen_schema_enum(schema_enum: &SchemaEnum) -> Result<TokenStream> {
  let enum_name_ident: Ident = parse_str(&schema_enum.name.to_upper_camel_case())?;
  let enum_attrs = version_cfg_attrs(&schema_enum.version);
  let baseline_facets: Vec<_> = schema_enum
    .facets
    .iter()
    .filter(|facet| !is_microsoft365_version(&facet.version))
    .collect();
  let has_later_facets = schema_enum
    .facets
    .iter()
    .any(|facet| is_microsoft365_version(&facet.version));

  if baseline_facets.is_empty() && has_later_facets {
    let later_enum_attrs = if is_microsoft365_version(&schema_enum.version) {
      Vec::new()
    } else {
      version_cfg_attrs("Microsoft365")
    };
    let default_facet = schema_enum.facets.first().expect("schema enum facet");
    let variants = gen_schema_enum_variants(
      &schema_enum.facets.iter().collect::<Vec<_>>(),
      default_facet,
      false,
    )?;

    return Ok(quote! {
      #( #later_enum_attrs )*
      #( #enum_attrs )*
      #[derive(Clone, Debug, Default)]
      pub enum #enum_name_ident {
        #( #variants, )*
      }
    });
  }

  if !baseline_facets.is_empty()
    && has_later_facets
    && !is_microsoft365_version(&schema_enum.version)
  {
    let baseline_enum_attrs = not_microsoft365_cfg_attrs();
    let baseline_default = baseline_facets
      .first()
      .copied()
      .expect("baseline schema enum facet");
    let later_default = baseline_default;

    let baseline_variants = gen_schema_enum_variants(&baseline_facets, baseline_default, false)?;
    let later_variants = gen_schema_enum_variants(
      &schema_enum.facets.iter().collect::<Vec<_>>(),
      later_default,
      true,
    )?;

    return Ok(quote! {
      #( #baseline_enum_attrs )*
      #( #enum_attrs )*
      #[derive(Clone, Debug, Default)]
      pub enum #enum_name_ident {
        #( #baseline_variants, )*
      }

      #[cfg(feature = "microsoft365")]
      #( #enum_attrs )*
      #[derive(Clone, Debug, Default)]
      pub enum #enum_name_ident {
        #( #later_variants, )*
      }
    });
  }

  let default_facet = baseline_facets
    .first()
    .copied()
    .unwrap_or_else(|| schema_enum.facets.first().expect("schema enum facet"));
  let variants = gen_schema_enum_variants(
    &schema_enum.facets.iter().collect::<Vec<_>>(),
    default_facet,
    true,
  )?;

  Ok(quote! {
    #( #enum_attrs )*
    #[derive(Clone, Debug, Default)]
    pub enum #enum_name_ident {
      #( #variants, )*
    }
  })
}

fn gen_schema_enum_variants(
  facets: &[&crate::sdk_data::sdk_data_model::SchemaEnumFacet],
  default_facet: &crate::sdk_data::sdk_data_model::SchemaEnumFacet,
  emit_version_cfgs: bool,
) -> Result<Vec<Variant>> {
  let mut variants = Vec::with_capacity(facets.len());

  for facet in facets {
    let variant_ident: Ident = if facet.name.is_empty() {
      parse_str(&escape_upper_camel_case(facet.value.to_upper_camel_case()))?
    } else {
      parse_str(&escape_upper_camel_case(facet.name.to_upper_camel_case()))?
    };
    let variant_attrs = if emit_version_cfgs {
      version_cfg_attrs(&facet.version)
    } else {
      Vec::new()
    };
    let default_attr = if std::ptr::eq(*facet, default_facet) {
      quote! { #[default] }
    } else {
      quote! {}
    };

    variants.push(parse2(quote! {
      #( #variant_attrs )*
      #default_attr
      #variant_ident
    })?);
  }

  Ok(variants)
}

fn gen_attr(
  attr: &SchemaTypeAttribute,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<TokenStream> {
  let attr_name_ident: Ident = if attr.property_name.is_empty() {
    parse_str(&escape_snake_case(attr.q_name.to_snake_case()))?
  } else {
    parse_str(&escape_snake_case(attr.property_name.to_snake_case()))?
  };

  let type_ident: Type =
    match classify_attr_type(&attr.r#type).ok_or_else(|| attr.r#type.clone())? {
      AttrTypeKind::List => parse_str("String")?,
      AttrTypeKind::Enum { .. } => {
        let (enum_module_name, enum_name) = context.resolve_attr_enum_module(&attr.r#type)?;

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

  let required = attr
    .validators
    .iter()
    .any(|validator| validator.name == "RequiredValidator");
  let attr_attrs = version_cfg_attrs(&attr.version);
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
      pub #attr_name_ident: Option<#type_ident>,
    }
  })
}

fn gen_children(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<(Option<TokenStream>, Option<ItemEnum>)> {
  let resolved_children = context.resolve_children(schema_type)?;

  if resolved_children.is_empty() {
    return Ok((None, None));
  }

  let child_choice_enum_ident: Ident = parse_str(&format!(
    "{}ChildChoice",
    schema_type.class_name.to_upper_camel_case()
  ))?;

  let field_option = Some(quote! {
    pub children: Vec<#child_choice_enum_ident>,
  });

  let mut variants: Vec<TokenStream> = vec![];

  for child in &resolved_children {
    let child_type = context
      .type_map
      .get(child.name)
      .ok_or_else(|| format!("{:?}", child.name))?;
    let child_prefix = context
      .type_prefix_map
      .get(child.name)
      .ok_or_else(|| format!("{:?}", child.name))?;
    let child_variant_name_ident: Ident = parse_str(&child.variant_name.to_upper_camel_case())?;

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
    let child_attrs = version_cfg_attrs(child.version);

    variants.push(quote! {
      #( #child_attrs )*
      #child_variant_name_ident(std::boxed::Box<#child_variant_type>),
    });
  }

  let enum_option = Some(parse2(quote! {
    #[derive(Clone, Debug)]
    pub enum #child_choice_enum_ident {
      #( #variants )*
    }
  })?);

  Ok((field_option, enum_option))
}

fn gen_xml_content_type(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<Type> {
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

fn gen_one_sequence_fields(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<(Vec<TokenStream>, Vec<TokenStream>)> {
  let mut fields: Vec<TokenStream> = vec![];
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
        let child_type = context
          .type_map
          .get(child.name)
          .ok_or_else(|| format!("{:?}", particle.name))?;
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

        let child_name_ident: Ident = parse_str(&child.field_name)?;

        if !field_name_set.insert(child_name_ident.to_string()) {
          continue;
        }

        let property_comments = child.property_comments.as_ref();
        let field_attrs = version_cfg_attrs(effective_version(
          child.version,
          flat_particle.initial_version,
        ));

        if flat_particle.repeated {
          fields.push(quote! {
            #( #field_attrs )*
            #[doc = #property_comments]
            pub #child_name_ident: Vec<#child_variant_type>,
          });
        } else if flat_particle.optional {
          fields.push(quote! {
            #( #field_attrs )*
            #[doc = #property_comments]
            pub #child_name_ident: Option<std::boxed::Box<#child_variant_type>>,
          });
        } else {
          fields.push(quote! {
            #( #field_attrs )*
            #[doc = #property_comments]
            pub #child_name_ident: std::boxed::Box<#child_variant_type>,
          });
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
        let field_attrs = version_cfg_attrs(choice_version);

        let mut variants = Vec::new();
        let mut default_variant = None;

        for variant in &choice.variants {
          let variant_type = one_sequence_child_variant_type(variant, schema, context)?;
          let variant_ident: Ident = parse_str(&variant.field_name.to_upper_camel_case())?;
          let variant_attrs = version_cfg_attrs(variant.version);

          if default_variant.is_none()
            && (choice_version == variant.version
              || (choice_version.is_empty() && !is_microsoft365_version(variant.version)))
          {
            default_variant = Some((variant_ident.clone(), variant_type.clone()));
          }

          variants.push(quote! {
            #( #variant_attrs )*
            #variant_ident(std::boxed::Box<#variant_type>),
          });
        }

        let (default_variant_ident, default_variant_type) = default_variant
          .or_else(|| {
            choice.variants.first().and_then(|variant| {
              let variant_type = one_sequence_child_variant_type(variant, schema, context).ok()?;
              let variant_ident =
                parse_str::<Ident>(&variant.field_name.to_upper_camel_case()).ok()?;
              Some((variant_ident, variant_type))
            })
          })
          .ok_or_else(|| choice.enum_name.clone())?;
        let enum_item = quote! {
          #[derive(Clone, Debug)]
          pub enum #choice_enum_ident {
            #( #variants )*
          }
        };
        enums.push(quote! {
          #( #field_attrs )*
          #enum_item
        });
        enums.push(quote! {
          #( #field_attrs )*
          impl Default for #choice_enum_ident {
            fn default() -> Self {
              Self::#default_variant_ident(std::boxed::Box::<#default_variant_type>::default())
            }
          }
        });

        if flat_particle.repeated {
          fields.push(quote! {
            #( #field_attrs )*
            #[doc = #property_comments]
            pub #choice_field_ident: Vec<#choice_enum_ident>,
          });
        } else if flat_particle.optional {
          fields.push(quote! {
            #( #field_attrs )*
            #[doc = #property_comments]
            pub #choice_field_ident: Option<#choice_enum_ident>,
          });
        } else {
          fields.push(quote! {
            #( #field_attrs )*
            #[doc = #property_comments]
            pub #choice_field_ident: #choice_enum_ident,
          });
        }
      }
    }
  }

  Ok((fields, enums))
}

fn gen_structured_one_sequence_fields(
  schema_type: &SchemaType,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<(Vec<TokenStream>, Vec<TokenStream>)> {
  let mut fields: Vec<TokenStream> = vec![];
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
        let child_type = context
          .type_map
          .get(child.name)
          .ok_or_else(|| format!("{:?}", leaf.name))?;
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
        let child_name_ident: Ident = parse_str(&child.field_name)?;

        if !field_name_set.insert(child_name_ident.to_string()) {
          continue;
        }

        let property_comments = child.property_comments.as_ref();
        let field_attrs =
          version_cfg_attrs(effective_version(child.version, particle.initial_version));

        if particle.repeated {
          fields.push(quote! {
            #( #field_attrs )*
            #[doc = #property_comments]
            pub #child_name_ident: Vec<#child_variant_type>,
          });
        } else if particle.optional {
          fields.push(quote! {
            #( #field_attrs )*
            #[doc = #property_comments]
            pub #child_name_ident: Option<std::boxed::Box<#child_variant_type>>,
          });
        } else {
          fields.push(quote! {
            #( #field_attrs )*
            #[doc = #property_comments]
            pub #child_name_ident: std::boxed::Box<#child_variant_type>,
          });
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
        let field_attrs = version_cfg_attrs(choice_version);
        let mut variants = Vec::new();
        let mut default_variant = None;

        for variant in &choice.variants {
          match variant {
            ResolvedOneSequenceChoiceVariant::Leaf(child) => {
              let variant_type = one_sequence_child_variant_type(child, schema, context)?;
              let variant_ident: Ident = parse_str(&child.field_name.to_upper_camel_case())?;
              let variant_attrs = version_cfg_attrs(child.version);

              if default_variant.is_none()
                && (choice_version == child.version
                  || (choice_version.is_empty() && !is_microsoft365_version(child.version)))
              {
                default_variant = Some((variant_ident.clone(), variant_type.clone()));
              }

              variants.push(quote! {
                #( #variant_attrs )*
                #variant_ident(std::boxed::Box<#variant_type>),
              });
            }
            ResolvedOneSequenceChoiceVariant::Sequence(sequence_variant) => {
              let struct_ident: Ident = parse_str(&sequence_variant.struct_name)?;
              let variant_ident: Ident = parse_str(&sequence_variant.variant_name)?;
              let sequence_fields = gen_sequence_variant_fields(sequence_variant, schema, context)?;
              let sequence_property_comments = sequence_variant.property_comments.as_str();
              let sequence_version = common_choice_version(
                "",
                &sequence_variant
                  .fields
                  .iter()
                  .map(|field| effective_version(field.child.version, field.initial_version))
                  .collect::<Vec<_>>(),
              );
              let sequence_attrs = version_cfg_attrs(sequence_version);

              items.push(quote! {
                #( #sequence_attrs )*
                #[doc = #sequence_property_comments]
                #[derive(Clone, Debug, Default)]
                pub struct #struct_ident {
                  #( #sequence_fields )*
                }
              });

              if default_variant.is_none()
                && (choice_version == sequence_version
                  || (choice_version.is_empty() && !is_microsoft365_version(sequence_version)))
              {
                default_variant = Some((variant_ident.clone(), parse2(quote! { #struct_ident })?));
              }

              variants.push(quote! {
                #( #sequence_attrs )*
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
          #( #field_attrs )*
          #[derive(Clone, Debug)]
          pub enum #choice_enum_ident {
            #( #variants )*
          }
        });
        items.push(quote! {
          #( #field_attrs )*
          impl Default for #choice_enum_ident {
            fn default() -> Self {
              Self::#default_variant_ident(Default::default())
            }
          }
        });

        if particle.repeated {
          fields.push(quote! {
            #( #field_attrs )*
            #[doc = #property_comments]
            pub #choice_field_ident: Vec<#choice_enum_ident>,
          });
        } else if particle.optional {
          fields.push(quote! {
            #( #field_attrs )*
            #[doc = #property_comments]
            pub #choice_field_ident: Option<#choice_enum_ident>,
          });
        } else {
          fields.push(quote! {
            #( #field_attrs )*
            #[doc = #property_comments]
            pub #choice_field_ident: #choice_enum_ident,
          });
        }
      }
    }
  }

  Ok((fields, items))
}

fn gen_sequence_variant_fields(
  sequence_variant: &ResolvedOneSequenceSequenceVariant<'_>,
  schema: &Schema,
  context: &CodegenContext<'_>,
) -> Result<Vec<TokenStream>> {
  let mut fields = Vec::new();

  for field in &sequence_variant.fields {
    let child = &field.child;
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

    let child_name_ident: Ident = parse_str(&child.field_name)?;
    let property_comments = child.property_comments.as_ref();
    let field_attrs = version_cfg_attrs(effective_version(child.version, field.initial_version));

    if field.repeated {
      fields.push(quote! {
        #( #field_attrs )*
        #[doc = #property_comments]
        pub #child_name_ident: Vec<#child_variant_type>,
      });
    } else if field.optional {
      fields.push(quote! {
        #( #field_attrs )*
        #[doc = #property_comments]
        pub #child_name_ident: Option<std::boxed::Box<#child_variant_type>>,
      });
    } else {
      fields.push(quote! {
        #( #field_attrs )*
        #[doc = #property_comments]
        pub #child_name_ident: std::boxed::Box<#child_variant_type>,
      });
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
