use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_str, Ident, ItemFn, LitByteStr, Type};

use crate::models::OpenXmlSchema;
use crate::utils::{escape_snake_case, escape_upper_camel_case};
use crate::GenContext;

pub fn gen_deserializer(schema: &OpenXmlSchema, context: &GenContext) -> TokenStream {
  let mut token_stream_list: Vec<TokenStream> = vec![];

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

    let mut variants: Vec<TokenStream> = vec![];

    for facet in &e.facets {
      let variant_rename = &facet.value;

      let variant_ident: Ident = if facet.name.is_empty() {
        parse_str(&escape_upper_camel_case(facet.value.to_upper_camel_case())).unwrap()
      } else {
        parse_str(&escape_upper_camel_case(facet.name.to_upper_camel_case())).unwrap()
      };

      variants.push(quote! {
        #variant_rename => Ok(Self::#variant_ident),
      })
    }

    token_stream_list.push(quote! {
      impl std::str::FromStr for #enum_type {
        type Err = super::deserializers::DeError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
          match s {
            #( #variants )*
            _ => Err(Self::Err::UnknownError),
          }
        }
      }
    })
  }

  let from_str_fn = gen_from_str_fn();

  let from_reader_fn = gen_from_reader_fn();

  for t in &schema.types {
    if t.children.is_empty()
      && t.attributes.is_empty()
      && context
        .type_base_class_type_map
        .contains_key(t.base_class.as_str())
    {
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

    let rename_de_literal: LitByteStr = parse_str(&format!("b\"{}\"", rename_de_str)).unwrap();

    let mut field_declaration_list: Vec<TokenStream> = vec![];
    let mut field_match_list: Vec<TokenStream> = vec![];
    let mut field_unwrap_list: Vec<TokenStream> = vec![];
    let mut field_init_list: Vec<TokenStream> = vec![];

    for attr in &t.attributes {
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

      field_declaration_list.push(quote! {
        let mut #attr_name_ident = None;
      });

      let attr_rename_ser_literal: LitByteStr =
        parse_str(&format!("b\"{}\"", attr_rename_ser_str)).unwrap();

      field_match_list.push(quote! {
        #attr_rename_ser_literal => {
          #attr_name_ident = Some(
            attr
              .decode_and_unescape_value(xml_reader.decoder())?
              .to_string(),
          )
        }
      });

      let mut required = false;

      for validator in &attr.validators {
        if validator.name == "RequiredValidator" {
          required = true;
        }
      }

      if !required {
        field_unwrap_list.push(quote! {
          let #attr_name_ident = #attr_name_ident.ok_or_else(|| super::deserializers::DeError::UnknownError)?;
        })
      }

      field_init_list.push(quote! {
        #attr_name_ident,
      })
    }

    let output = quote! {
      impl #struct_type {
        #from_str_fn

        #from_reader_fn

        pub fn deserialize_self<'de, R: super::deserializers::XmlReader<'de>>(
          xml_reader: &mut R,
        ) -> Result<Self, super::deserializers::DeError> {
          if let quick_xml::events::Event::Empty(e) = xml_reader.next()? {
            if e.name().local_name().as_ref() != #rename_de_literal {
              Err(super::deserializers::DeError::UnknownError)?;
            }

            #( #field_declaration_list )*

            for attr in e.attributes() {
              let attr = attr?;

              match attr.key.as_ref() {
                #( #field_match_list )*
                _ => (),
              }
            }

            #( #field_unwrap_list )*

            Ok(Self {
              #( #field_init_list )*
            })
          } else {
            Err(super::deserializers::DeError::UnknownError)?
          }
        }
      }
    };

    let syntax_tree = syn::parse2(output.clone()).unwrap();
    let _ = prettyplease::unparse(&syntax_tree);

    token_stream_list.push(output);
  }

  quote! {
    #( #token_stream_list )*
  }
}

pub fn gen_from_str_fn() -> ItemFn {
  let token_stream = quote! {
    pub fn from_str(s: &str) -> Result<Self, super::deserializers::DeError> {
      let mut xml_reader = super::deserializers::SliceReader::new(quick_xml::Reader::from_str(s));

      Self::deserialize_self(&mut xml_reader)
    }
  };

  parse_str(&token_stream.to_string()).unwrap()
}

pub fn gen_from_reader_fn() -> ItemFn {
  let token_stream = quote! {
    pub fn from_reader<R: std::io::BufRead>(
      reader: R,
    ) -> Result<Self, super::deserializers::DeError> {
      let mut xml_reader =
        super::deserializers::IoReader::new(quick_xml::Reader::from_reader(reader));

      Self::deserialize_self(&mut xml_reader)
    }
  };

  parse_str(&token_stream.to_string()).unwrap()
}
