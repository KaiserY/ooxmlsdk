use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;

use syn::{parse2, parse_str, Arm, Ident, ItemImpl, Type};

use crate::models::{OpenXmlSchema, OpenXmlSchemaTypeAttribute};
use crate::utils::{escape_snake_case, escape_upper_camel_case};
use crate::GenContext;

pub fn gen_serializer(schema: &OpenXmlSchema, context: &GenContext) -> TokenStream {
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

  for e in &schema.enums {
    let enum_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      scheme_mod,
      e.name.to_upper_camel_case()
    ))
    .unwrap();

    let mut variants: Vec<Arm> = vec![];

    for facet in &e.facets {
      let variant_rename = &facet.value;

      let variant_ident: Ident = if facet.name.is_empty() {
        parse_str(&escape_upper_camel_case(facet.value.to_upper_camel_case())).unwrap()
      } else {
        parse_str(&escape_upper_camel_case(facet.name.to_upper_camel_case())).unwrap()
      };

      variants.push(
        parse2(quote! {
           Self::#variant_ident => #variant_rename.to_string(),
        })
        .unwrap(),
      )
    }

    token_stream_list.push(
      parse2(quote! {
        impl #enum_type {
          #[allow(clippy::inherent_to_string)]
          pub fn to_string(&self) -> String {
            match self {
              #( #variants )*
            }
          }
        }
      })
      .unwrap(),
    )
  }

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

    let name_list: Vec<&str> = t.name.split('/').collect();

    let rename_ser_str = name_list.last().ok_or(format!("{:?}", t.name)).unwrap();

    let rename_list: Vec<&str> = rename_ser_str.split(':').collect();

    let rename_de_str = rename_list.last().ok_or(format!("{:?}", t.name)).unwrap();

    let children_writer = quote! {};

    let end_writer = quote! {};

    let mut variants: Vec<TokenStream> = vec![];

    for attr in &t.attributes {
      variants.push(gen_attr(attr));
    }

    if t.base_class == "OpenXmlLeafTextElement" {
    } else if t.base_class == "OpenXmlLeafElement" {
    } else if t.base_class == "OpenXmlCompositeElement"
      || t.base_class == "CustomXmlElement"
      || t.base_class == "OpenXmlPartRootElement"
      || t.base_class == "SdtElement"
    {
    } else if t.is_derived {
      let base_class_type = context
        .type_base_class_type_map
        .get(t.base_class.as_str())
        .ok_or(format!("{:?}", t.base_class))
        .unwrap();

      for attr in &base_class_type.attributes {
        variants.push(gen_attr(attr));
      }
    } else {
      panic!("{:?}", t);
    };

    let attr_writer = quote! {
      #( #variants )*
    };

    token_stream_list.push(
      parse2(quote! {
        impl #struct_type {
          #[allow(clippy::inherent_to_string)]
          pub fn to_string(&self, with_xmlns: bool) -> Result<String, super::serializer_common::SeError> {
            use std::fmt::Write;

            let mut writer = String::new();

            writer.write_char('<')?;

            if with_xmlns {
              writer.write_str(#rename_ser_str)?;
            } else {
              writer.write_str(#rename_de_str)?;
            }

            #attr_writer

            writer.write_char('>')?;

            #children_writer

            #end_writer

            Ok(writer)
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

pub fn gen_attr(attr: &OpenXmlSchemaTypeAttribute) -> TokenStream {
  let attr_rename_ser_str = if attr.q_name.starts_with(':') {
    &attr.q_name[1..attr.q_name.len()]
  } else {
    &attr.q_name
  };

  let attr_name_ident: Ident = if attr.property_name.is_empty() {
    parse_str(&escape_snake_case(attr.q_name.to_snake_case())).unwrap()
  } else {
    parse_str(&escape_snake_case(attr.property_name.to_snake_case())).unwrap()
  };

  let mut required = false;

  for validator in &attr.validators {
    if validator.name == "RequiredValidator" {
      required = true;
    }
  }

  if required {
    quote! {
      writer.write_char(' ')?;
      writer.write_str(#attr_rename_ser_str)?;
      writer.write_str("=\"")?;
      writer.write_str(&quick_xml::escape::escape(&self.#attr_name_ident.to_string()))?;
      writer.write_char('"')?;
    }
  } else {
    quote! {
      if let Some(#attr_name_ident) = &self.#attr_name_ident {
        writer.write_char(' ')?;
        writer.write_str(#attr_rename_ser_str)?;
        writer.write_str("=\"")?;
        writer.write_str(&quick_xml::escape::escape(&#attr_name_ident.to_string()))?;
        writer.write_char('"')?;
      }
    }
  }
}
