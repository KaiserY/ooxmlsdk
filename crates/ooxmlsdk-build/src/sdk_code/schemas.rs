use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Ident, ItemEnum, Type, Variant, parse_str, parse2};

use crate::generator::simple_type::simple_type_mapping;
use crate::sdk_code::helpers::{
  AttrTypeKind, classify_attr_type, is_composite_type, is_one_sequence_flatten,
  supports_xmlns_fields,
};
use crate::sdk_data::sdk_data_model::{
  Schema, SchemaEnum, SchemaType, SchemaTypeAttribute, SchemaTypeChild,
};
use crate::utils::{escape_snake_case, escape_upper_camel_case};

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, BoxError>;

#[derive(Debug)]
pub struct CodegenContext<'a> {
  enum_type_map: HashMap<&'a str, &'a SchemaEnum>,
  enum_type_module_map: HashMap<&'a str, &'a str>,
  type_map: HashMap<&'a str, &'a SchemaType>,
  type_prefix_map: HashMap<&'a str, &'a str>,
  enum_module_by_typed_namespace_and_name: HashMap<(&'a str, &'a str), &'a str>,
}

impl<'a> CodegenContext<'a> {
  pub fn new(schemas: &'a [Schema]) -> Self {
    let mut enum_type_map = HashMap::new();
    let mut enum_type_module_map = HashMap::new();
    let mut type_map = HashMap::new();
    let mut type_prefix_map = HashMap::new();
    let mut enum_module_by_typed_namespace_and_name = HashMap::new();

    for schema in schemas {
      for schema_type in &schema.types {
        type_map.insert(schema_type.name.as_str(), schema_type);
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

  pub fn type_prefix(&self, name: &str) -> Option<&'a str> {
    self.type_prefix_map.get(name).copied()
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
}

pub fn gen_schema(schema: &Schema, context: &CodegenContext<'_>) -> Result<TokenStream> {
  let mut token_stream_list: Vec<TokenStream> = vec![];

  for schema_enum in &schema.enums {
    let enum_name_ident: Ident = parse_str(&schema_enum.name.to_upper_camel_case())?;
    let mut variants: Vec<Variant> = vec![];

    for (index, facet) in schema_enum.facets.iter().enumerate() {
      let variant_ident: Ident = if facet.name.is_empty() {
        parse_str(&escape_upper_camel_case(facet.value.to_upper_camel_case()))?
      } else {
        parse_str(&escape_upper_camel_case(facet.name.to_upper_camel_case()))?
      };

      if index == 0 {
        variants.push(parse2(quote! {
          #[default]
          #variant_ident
        })?);
      } else {
        variants.push(parse2(quote! {
          #variant_ident
        })?);
      }
    }

    token_stream_list.push(quote! {
      #[derive(Clone, Debug, Default)]
      pub enum #enum_name_ident {
        #( #variants, )*
      }
    });
  }

  for schema_type in &schema.types {
    let mut fields: Vec<TokenStream> = vec![];
    let mut child_choice_enum_option: Option<ItemEnum> = None;

    if schema_type.base_class == "OpenXmlLeafTextElement" {
      for attr in &schema_type.attributes {
        fields.push(gen_attr(attr, schema, context)?);
      }

      let simple_type_name = gen_xml_content_type(schema_type, schema, context)?;
      fields.push(quote! {
        pub xml_content: Option<#simple_type_name>,
      });
    } else if schema_type.base_class == "OpenXmlLeafElement" {
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
    } else if schema_type.is_derived {
      let base_class_type_name = &schema_type.name[0..schema_type.name.find('/').unwrap() + 1];
      let base_class_type = context
        .type_map
        .get(base_class_type_name)
        .ok_or_else(|| format!("{base_class_type_name:?}"))?;

      for attr in &schema_type.attributes {
        fields.push(gen_attr(attr, schema, context)?);
      }

      for attr in &base_class_type.attributes {
        fields.push(gen_attr(attr, schema, context)?);
      }

      if is_one_sequence_flatten(schema_type) && base_class_type.composite_type == "OneSequence" {
        fields.extend(gen_one_sequence_fields(schema_type, schema, context)?);
      } else {
        let (field_option, enum_option) = gen_children(schema_type, schema, context)?;

        if let Some(field) = field_option {
          fields.push(field);
        }

        child_choice_enum_option = enum_option;
      }

      if schema_type.children.is_empty() && base_class_type.base_class == "OpenXmlLeafTextElement" {
        let simple_type_name = gen_xml_content_type(base_class_type, schema, context)?;
        fields.push(quote! {
          pub xml_content: Option<#simple_type_name>,
        });
      }
    } else {
      return Err(format!("{schema_type:?}").into());
    }

    let struct_name_ident: Ident = parse_str(&schema_type.class_name.to_upper_camel_case())?;
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

    token_stream_list.push(quote! {
      #[doc = #summary_doc]
      #[doc = ""]
      #[doc = #version_doc]
      #[doc = ""]
      #[doc = #qualified_doc]
      #[derive(Clone, Debug, Default)]
      pub struct #struct_name_ident {
        #( #fields )*
      }

      #child_choice_enum_option
    });
  }

  Ok(quote! {
    #( #token_stream_list )*
  })
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
      AttrTypeKind::Enum {
        typed_namespace,
        enum_name,
      } => {
        let enum_module_name = context
          .enum_module_by_typed_namespace_and_name(typed_namespace, enum_name)
          .ok_or_else(|| format!("{typed_namespace:?}:{enum_name:?}"))?;

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
        parse_str(&format!("crate::schemas::simple_type::{simple_type}"))?
      }
    };

  let required = attr
    .validators
    .iter()
    .any(|validator| validator.name == "RequiredValidator");
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
      #[doc = #property_comments_doc]
      #[doc = ""]
      #[doc = #version_doc]
      #[doc = ""]
      #[doc = #qualified_doc]
      pub #attr_name_ident: #type_ident,
    }
  } else {
    quote! {
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
  if schema_type.children.is_empty() {
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

  for child in &schema_type.children {
    let child_type = context
      .type_map
      .get(child.name.as_str())
      .ok_or_else(|| format!("{:?}", child.name))?;
    let child_prefix = context
      .type_prefix_map
      .get(child.name.as_str())
      .ok_or_else(|| format!("{:?}", child.name))?;
    let child_last_name = &child.name[child.name.find('/').unwrap() + 1..];
    let child_variant_name_ident: Ident = parse_str(&child_last_name.to_upper_camel_case())?;

    let child_variant_type: Type = if *child_prefix != schema.prefix {
      parse_str(&format!(
        "crate::schemas::{}::{}",
        &child
          .schema_module
          .as_deref()
          .ok_or_else(|| format!("{:?}", child.name))?,
        child_type.class_name.to_upper_camel_case()
      ))?
    } else {
      parse_str(&child_type.class_name.to_upper_camel_case())?
    };

    variants.push(quote! {
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
      "crate::schemas::simple_type::{}",
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
  let mut child_map: HashMap<&str, &SchemaTypeChild> = HashMap::new();

  for child in &schema_type.children {
    child_map.insert(&child.name, child);
  }

  for particle in &schema_type.particle.items {
    let child = child_map
      .get(particle.name.as_str())
      .ok_or_else(|| format!("{:?}", particle.name))?;
    let child_type = context
      .type_map
      .get(particle.name.as_str())
      .ok_or_else(|| format!("{:?}", particle.name))?;
    let child_prefix = context
      .type_prefix_map
      .get(child.name.as_str())
      .ok_or_else(|| format!("{:?}", child.name))?;

    let child_variant_type: Type = if *child_prefix != schema.prefix {
      parse_str(&format!(
        "crate::schemas::{}::{}",
        child
          .schema_module
          .as_deref()
          .ok_or_else(|| format!("{:?}", child.name))?,
        child_type.class_name.to_upper_camel_case()
      ))?
    } else {
      parse_str(&child_type.class_name.to_upper_camel_case())?
    };

    let child_name_ident: Ident = if child.property_name.is_empty() {
      let child_last_name = &child.name[child.name.find('/').unwrap() + 1..];
      parse_str(&child_last_name.to_snake_case())?
    } else {
      parse_str(&escape_snake_case(child.property_name.to_snake_case()))?
    };

    let property_comments = if child.property_comments.is_empty() {
      " _"
    } else {
      &child.property_comments
    };

    if particle.occurs.is_empty() {
      fields.push(quote! {
        #[doc = #property_comments]
        pub #child_name_ident: std::boxed::Box<#child_variant_type>,
      });
    } else if particle.occurs[0].min == 0 && particle.occurs[0].max == 1 {
      fields.push(quote! {
        #[doc = #property_comments]
        pub #child_name_ident: Option<std::boxed::Box<#child_variant_type>>,
      });
    } else {
      fields.push(quote! {
        #[doc = #property_comments]
        pub #child_name_ident: Vec<#child_variant_type>,
      });
    }
  }

  Ok(fields)
}
