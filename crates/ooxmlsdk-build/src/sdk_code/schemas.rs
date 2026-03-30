use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;
use std::collections::HashMap;
use syn::{Ident, ItemEnum, Type, Variant, parse_str, parse2};

use crate::sdk_code::helpers::{
  AttrTypeKind, classify_attr_type, flatten_one_sequence_particles, is_composite_type,
  is_derived_type, is_leaf_element_type, is_leaf_text_type, is_leaf_text_wrapper,
  is_one_sequence_flatten, supports_xmlns_fields,
};
use crate::sdk_code::versioning::{
  effective_version, is_microsoft365_version, not_microsoft365_cfg_attrs, version_cfg_attrs,
};
use crate::sdk_data::sdk_data_model::{Schema, SchemaEnum, SchemaType, SchemaTypeAttribute};
use crate::simple_type::simple_type_mapping;
use crate::utils::{escape_snake_case, escape_upper_camel_case};

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, BoxError>;

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
          .map(|item| effective_version(item.version.as_str(), particle.initial_version.as_str()))
          .unwrap_or(particle.initial_version.as_str()),
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
    let mut child_choice_enum_option: Option<ItemEnum> = None;

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
        fields.extend(gen_one_sequence_fields(schema_type, schema, context)?);
      } else {
        let (field_option, enum_option) = gen_children(schema_type, schema, context)?;

        if let Some(field) = field_option {
          fields.push(field);
        }

        child_choice_enum_option = enum_option;
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
        fields.extend(gen_one_sequence_fields(schema_type, schema, context)?);
      } else {
        let (field_option, enum_option) = gen_children(schema_type, schema, context)?;

        if let Some(field) = field_option {
          fields.push(field);
        }

        child_choice_enum_option = enum_option;
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

    let child_choice_tokens = if let Some(child_choice_enum) = child_choice_enum_option {
      quote! {
        #( #type_attrs )*
        #child_choice_enum
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
) -> Result<Vec<TokenStream>> {
  let mut fields: Vec<TokenStream> = vec![];
  let mut field_name_set = std::collections::HashSet::new();

  for flat_particle in flatten_one_sequence_particles(schema_type) {
    let particle = flat_particle.particle;
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
      particle.initial_version.as_str(),
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

  Ok(fields)
}
