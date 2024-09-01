use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{parse_str, Ident, Type};

use crate::gen::simple_type::simple_type_mapping;
use crate::models::OpenXmlSchema;
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
      if facet.value == "none" {
        continue;
      }

      let variant_ident: Ident = if facet.name.is_empty() {
        parse_str(&escape_upper_camel_case(facet.value.to_upper_camel_case())).unwrap()
      } else {
        parse_str(&escape_upper_camel_case(facet.name.to_upper_camel_case())).unwrap()
      };

      variants.push(quote! {
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
    if t.children.is_empty()
      && t.attributes.is_empty()
      && context
        .type_base_class_type_map
        .contains_key(t.base_class.as_str())
    {
      let base_class_type = context
        .type_base_class_type_map
        .get(t.base_class.as_str())
        .ok_or(format!("{:?}", t.base_class))
        .unwrap();

      let base_class_namespace = context
        .type_name_namespace_map
        .get(base_class_type.name.as_str())
        .ok_or(format!("{:?}", t.base_class))
        .unwrap();

      let base_class_type_type: Type = if base_class_namespace.prefix != schema_namespace.prefix {
        let scheme_mod = context
          .prefix_schema_mod_map
          .get(base_class_namespace.prefix.as_str())
          .ok_or(format!("{:?}", base_class_namespace.prefix))
          .unwrap();

        parse_str(&format!(
          "crate::schemas::{}::{}",
          scheme_mod,
          base_class_type.class_name.to_upper_camel_case()
        ))
        .unwrap()
      } else {
        parse_str(&base_class_type.class_name.to_upper_camel_case()).unwrap()
      };

      let type_name_ident: Ident = parse_str(&t.class_name.to_upper_camel_case()).unwrap();

      let summary_doc = format!(" {}", t.summary);

      token_stream_list.push(quote! {
        #[doc = #summary_doc]
        pub type #type_name_ident = #base_class_type_type;
      });

      continue;
    }

    let mut fields: Vec<TokenStream> = vec![];

    let mut child_choice_enum_option: Option<TokenStream> = None;

    for attr in &t.attributes {
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
        fields.push(quote! {
          #[doc = #property_comments_doc]
          pub #attr_name_ident: #type_ident,
        })
      } else {
        fields.push(quote! {
          #[doc = #property_comments_doc]
          #[serde(skip_serializing_if = "Option::is_none")]
          pub #attr_name_ident: Option<#type_ident>,
        })
      }
    }

    if t.base_class == "OpenXmlLeafTextElement" {
      let name_list: Vec<&str> = t.name.split('/').collect();

      let first_name = name_list.first().ok_or(format!("{:?}", t.name)).unwrap();

      let simple_type_name: Type = parse_str(simple_type_mapping(first_name)).unwrap();

      fields.push(quote! {
        pub child: #simple_type_name,
      });
    } else if !t.children.is_empty() {
      let child_choice_enum_ident: Ident = parse_str(&format!(
        "{}ChildChoice",
        t.class_name.to_upper_camel_case()
      ))
      .unwrap();

      fields.push(quote! {
        #[serde(default)]
        pub children: Vec<#child_choice_enum_ident>,
      });

      let mut variants: Vec<TokenStream> = vec![];

      let mut child_variant_name_set: HashSet<String> = HashSet::new();

      for child in &t.children {
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
          #child_variant_name_ident(std::boxed::Box<#child_variant_type>),
        });
      }

      child_choice_enum_option = Some(quote! {
        #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
        pub enum #child_choice_enum_ident {
          #( #variants )*
        }
      });
    }

    let struct_name_ident: Ident = parse_str(&t.class_name.to_upper_camel_case()).unwrap();

    let summary_doc = format!(" {}", t.summary);

    if let Some(child_choice_enum) = child_choice_enum_option {
      token_stream_list.push(quote! {
        #[doc = #summary_doc]
        #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
        pub struct #struct_name_ident {
          #( #fields )*
        }

        #child_choice_enum
      });
    } else {
      token_stream_list.push(quote! {
        #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
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
