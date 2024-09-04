use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{parse_str, Ident, Type};

use crate::gen::simple_type::simple_type_mapping;
use crate::models::{
  OpenXmlNamespace, OpenXmlSchema, OpenXmlSchemaTypeAttribute, OpenXmlSchemaTypeChild,
};
use crate::utils::{escape_snake_case, escape_upper_camel_case};
use crate::GenContext;

pub fn gen_open_xml_schema(schema: &OpenXmlSchema, context: &GenContext) -> TokenStream {
  let mut token_stream_list: Vec<TokenStream> = vec![];

  let schema_namespace = context
    .uri_namespace_map
    .get(schema.target_namespace.as_str())
    .ok_or(format!("{:?}", schema.target_namespace))
    .unwrap();

  for e in &schema.enums {
    let e_enum_name_ident: Ident = parse_str(&e.name.to_upper_camel_case()).unwrap();

    let mut variants: Vec<TokenStream> = vec![];

    for facet in &e.facets {
      let variant_rename = &facet.value;

      let variant_ident: Ident = if facet.name.is_empty() {
        parse_str(&escape_upper_camel_case(facet.value.to_upper_camel_case())).unwrap()
      } else {
        parse_str(&escape_upper_camel_case(facet.name.to_upper_camel_case())).unwrap()
      };

      variants.push(quote! {
        #[serde(rename = #variant_rename)]
        #variant_ident,
      })
    }

    token_stream_list.push(quote! {
      #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
      pub enum #e_enum_name_ident {
        #( #variants )*
      }
    })
  }

  for t in &schema.types {
    let mut fields: Vec<TokenStream> = vec![];

    let mut child_choice_enum_option: Option<TokenStream> = None;

    let name_list: Vec<&str> = t.name.split('/').collect();

    let rename_ser_str = name_list.last().ok_or(format!("{:?}", t.name)).unwrap();

    let rename_list: Vec<&str> = rename_ser_str.split(':').collect();

    let rename_de_str = rename_list.last().ok_or(format!("{:?}", t.name)).unwrap();

    if t.base_class == "OpenXmlLeafTextElement" {
      for attr in &t.attributes {
        fields.push(gen_attr(attr, schema_namespace, context));
      }

      let name_list: Vec<&str> = t.name.split('/').collect();

      let first_name = name_list.first().ok_or(format!("{:?}", t.name)).unwrap();

      let simple_type_name: Type = if let Some(e) = context.enum_type_enum_map.get(first_name) {
        let e_namespace = context
          .enum_type_namespace_map
          .get(e.r#type.as_str())
          .ok_or(format!("{:?}", e.r#type))
          .unwrap();

        if e_namespace.prefix != schema_namespace.prefix {
          let scheme_mod = context
            .prefix_schema_mod_map
            .get(e_namespace.prefix.as_str())
            .ok_or(format!("{:?}", e_namespace.prefix))
            .unwrap();

          parse_str(&format!(
            "crate::schemas::{}::{}",
            scheme_mod,
            e.name.to_upper_camel_case()
          ))
          .unwrap()
        } else {
          parse_str(&e.name.to_upper_camel_case()).unwrap()
        }
      } else {
        parse_str(simple_type_mapping(first_name)).unwrap()
      };

      fields.push(quote! {
        #[serde(rename = "$value")]
        pub child: Option<#simple_type_name>,
      });
    } else if t.base_class == "OpenXmlLeafElement" {
      for attr in &t.attributes {
        fields.push(gen_attr(attr, schema_namespace, context));
      }
    } else if t.base_class == "OpenXmlCompositeElement"
      || t.base_class == "CustomXmlElement"
      || t.base_class == "OpenXmlPartRootElement"
      || t.base_class == "SdtElement"
    {
      for attr in &t.attributes {
        fields.push(gen_attr(attr, schema_namespace, context));
      }

      let (field_option, enum_option) =
        gen_children(&t.class_name, &t.children, schema_namespace, context);

      if let Some(field) = field_option {
        fields.push(field);
      }

      child_choice_enum_option = enum_option;
    } else if t.is_derived {
      let base_class_type = context
        .type_base_class_type_map
        .get(t.base_class.as_str())
        .ok_or(format!("{:?}", t.base_class))
        .unwrap();

      for attr in &t.attributes {
        fields.push(gen_attr(attr, schema_namespace, context));
      }

      for attr in &base_class_type.attributes {
        fields.push(gen_attr(attr, schema_namespace, context));
      }

      let mut children = t.children.clone();

      children.extend(base_class_type.children.clone());

      let (field_option, enum_option) =
        gen_children(&t.class_name, &children, schema_namespace, context);

      if let Some(field) = field_option {
        fields.push(field);
      }

      child_choice_enum_option = enum_option;
    } else {
      panic!("{:?}", t);
    }

    let struct_name_ident: Ident = parse_str(&t.class_name.to_upper_camel_case()).unwrap();

    let summary_doc = format!(" {}", t.summary);

    if let Some(child_choice_enum) = child_choice_enum_option {
      token_stream_list.push(quote! {
        #[doc = #summary_doc]
        #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
        #[serde(rename(serialize = #rename_ser_str, deserialize = #rename_de_str))]
        pub struct #struct_name_ident {
          #( #fields )*
        }

        #child_choice_enum
      });
    } else {
      token_stream_list.push(quote! {
        #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
        #[serde(rename(serialize = #rename_ser_str, deserialize = #rename_de_str))]
        pub struct #struct_name_ident {
          #( #fields )*
        }
      });
    }
  }

  quote! {
    #( #token_stream_list )*
  }
}

pub fn gen_attr(
  attr: &OpenXmlSchemaTypeAttribute,
  schema_namespace: &OpenXmlNamespace,
  context: &GenContext,
) -> TokenStream {
  let q_name_list: Vec<&str> = attr.q_name.split(':').collect();

  let q_name_local = q_name_list
    .last()
    .ok_or(format!("{:?}", attr.r#type))
    .unwrap();

  let attr_rename_ser_str = if attr.q_name.starts_with(':') {
    format!("@{}", &attr.q_name[1..attr.q_name.len()])
  } else {
    format!("@{}", attr.q_name)
  };

  let attr_rename_de_str = format!("@{}", q_name_local);

  let attr_name_ident: Ident = if attr.property_name.is_empty() {
    parse_str(&escape_snake_case(attr.q_name.to_snake_case())).unwrap()
  } else {
    parse_str(&escape_snake_case(attr.property_name.to_snake_case())).unwrap()
  };

  let type_ident: Type = if attr.r#type.starts_with("ListValue<") {
    parse_str("String").unwrap()
  } else if attr.r#type.starts_with("EnumValue<") {
    let type_list: Vec<&str> = attr.r#type.split('.').collect();

    let enum_name = type_list
      .last()
      .ok_or(format!("{:?}", attr.r#type))
      .unwrap();
    let enum_name = &enum_name[0..enum_name.len() - 1];

    let e = context
      .enum_name_enum_map
      .get(enum_name)
      .ok_or(format!("{:?}", enum_name))
      .unwrap();

    let e_namespace = context
      .enum_type_namespace_map
      .get(e.r#type.as_str())
      .ok_or(format!("{:?}", e.r#type))
      .unwrap();

    if e_namespace.prefix != schema_namespace.prefix {
      let scheme_mod = context
        .prefix_schema_mod_map
        .get(e_namespace.prefix.as_str())
        .ok_or(format!("{:?}", e_namespace.prefix))
        .unwrap();

      parse_str(&format!(
        "crate::schemas::{}::{}",
        scheme_mod,
        e.name.to_upper_camel_case()
      ))
      .unwrap()
    } else {
      parse_str(&e.name.to_upper_camel_case()).unwrap()
    }
  } else {
    parse_str(&format!("crate::schemas::simple_type::{}", &attr.r#type)).unwrap()
  };

  let mut required = false;

  for validator in &attr.validators {
    if validator.name == "RequiredValidator" {
      required = true;
    }
  }

  let property_comments_doc = format!(" {}", attr.property_comments);

  if required {
    quote! {
      #[doc = #property_comments_doc]
      #[serde(rename(serialize = #attr_rename_ser_str, deserialize = #attr_rename_de_str))]
      pub #attr_name_ident: #type_ident,
    }
  } else {
    quote! {
      #[doc = #property_comments_doc]
      #[serde(skip_serializing_if = "Option::is_none")]
      #[serde(rename(serialize = #attr_rename_ser_str, deserialize = #attr_rename_de_str))]
      pub #attr_name_ident: Option<#type_ident>,
    }
  }
}

pub fn gen_children(
  class_name: &str,
  children: &Vec<OpenXmlSchemaTypeChild>,
  schema_namespace: &OpenXmlNamespace,
  context: &GenContext,
) -> (Option<TokenStream>, Option<TokenStream>) {
  if children.is_empty() {
    return (None, None);
  }

  let child_choice_enum_ident: Ident =
    parse_str(&format!("{}ChildChoice", class_name.to_upper_camel_case())).unwrap();

  let field_option = Some(quote! {
    #[serde(default, rename = "$value")]
    pub children: Vec<#child_choice_enum_ident>,
  });

  let mut variants: Vec<TokenStream> = vec![];

  let mut child_variant_name_set: HashSet<String> = HashSet::new();

  for child in children {
    let child_type = context
      .type_name_type_map
      .get(child.name.as_str())
      .ok_or(format!("{:?}", child.name))
      .unwrap();

    let child_namespace = context
      .type_name_namespace_map
      .get(child.name.as_str())
      .ok_or(format!("{:?}", child.name))
      .unwrap();

    let child_name_list: Vec<&str> = child.name.split('/').collect();

    let child_rename_ser_str = child_name_list
      .last()
      .ok_or(format!("{:?}", class_name))
      .unwrap();

    let child_rename_list: Vec<&str> = child_rename_ser_str.split(':').collect();

    let child_rename_de_str = child_rename_list
      .last()
      .ok_or(format!("{:?}", class_name))
      .unwrap();

    let mut child_variant_name: String = if child.property_name.is_empty() {
      child_type.class_name.to_upper_camel_case()
    } else {
      child.property_name.to_upper_camel_case()
    };

    if child_variant_name_set.contains(child_variant_name.as_str()) {
      child_variant_name =
        format!("{}:{}", child_namespace.prefix, child_variant_name).to_upper_camel_case();
    } else {
      child_variant_name_set.insert(child_variant_name.clone());
    }

    let child_variant_name_ident: Ident = parse_str(&child_variant_name).unwrap();

    let child_variant_type: Type = if child_namespace.prefix != schema_namespace.prefix {
      let scheme_mod = context
        .prefix_schema_mod_map
        .get(child_namespace.prefix.as_str())
        .ok_or(format!("{:?}", child_namespace.prefix))
        .unwrap();

      parse_str(&format!(
        "crate::schemas::{}::{}",
        scheme_mod,
        child_type.class_name.to_upper_camel_case()
      ))
      .unwrap()
    } else {
      parse_str(&child_type.class_name.to_upper_camel_case()).unwrap()
    };

    variants.push(quote! {
      #[serde(rename(serialize = #child_rename_ser_str, deserialize = #child_rename_de_str))]
      #child_variant_name_ident(std::boxed::Box<#child_variant_type>),
    });
  }

  let enum_option = Some(quote! {
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub enum #child_choice_enum_ident {
      #( #variants )*
    }
  });

  (field_option, enum_option)
}
