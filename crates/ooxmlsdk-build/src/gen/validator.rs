use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, parse_str, ItemImpl, Stmt, Type};

use crate::models::OpenXmlSchema;
use crate::GenContext;

pub fn gen_validator(schema: &OpenXmlSchema, context: &GenContext) -> TokenStream {
  let mut token_stream_list: Vec<ItemImpl> = vec![];

  let schema_namespace = context
    .uri_namespace_map
    .get(schema.target_namespace.as_str())
    .ok_or(format!("{:?}", schema.target_namespace))
    .unwrap();

  let scheme_mod = context
    .prefix_schema_mod_map
    .get(schema_namespace.prefix.as_str())
    .ok_or(format!("{:?}", schema_namespace.prefix))
    .unwrap();

  let mut children_validator_stmt_list: Vec<Stmt> = vec![];

  for t in &schema.types {
    if t.is_abstract {
      continue;
    }

    let struct_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      scheme_mod,
      t.class_name.to_upper_camel_case()
    ))
    .unwrap();

    if t.base_class == "OpenXmlLeafTextElement" {
    } else if t.base_class == "OpenXmlLeafElement" {
    } else if t.base_class == "OpenXmlCompositeElement"
      || t.base_class == "CustomXmlElement"
      || t.base_class == "OpenXmlPartRootElement"
      || t.base_class == "SdtElement"
    {
      if t.is_one_sequence_flatten() {
      } else {
        children_validator_stmt_list.push(
          parse2(quote! {
            for child in &self.children {
              if !child::validate() {
                return false;
              }
            }
          })
          .unwrap(),
        );
      }
    } else if t.is_derived {
      let base_class_type = context
        .type_name_type_map
        .get(&t.name[0..t.name.find('/').unwrap() + 1])
        .ok_or(format!("{:?}", t.base_class))
        .unwrap();

      if t.is_one_sequence_flatten() && base_class_type.composite_type == "OneSequence" {
      } else {
        children_validator_stmt_list.push(
          parse2(quote! {
            for child in &self.children {
              if !child::validate() {
                return false;
              }
            }
          })
          .unwrap(),
        );
      }

      if t.children.is_empty() && base_class_type.base_class == "OpenXmlLeafTextElement" {
        children_validator_stmt_list.push(
          parse2(quote! {
            if let Some(child) = &self.child {
              if !child::validate() {
                return false;
              }
            }
          })
          .unwrap(),
        );
      }
    } else {
      panic!("{:?}", t);
    }

    token_stream_list.push(
      parse2(quote! {
        impl #struct_type {
          pub fn validate(&self) -> Result<bool, crate::common::SdkError> {
            #( #children_validator_stmt_list )*

            Ok(true)
          }
        }
      })
      .unwrap(),
    )
  }

  quote! {
    #( #token_stream_list )*
  }
}
