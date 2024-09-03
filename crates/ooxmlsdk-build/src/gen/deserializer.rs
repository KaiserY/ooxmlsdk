use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_str, Ident, ItemFn, Type};

use crate::models::OpenXmlSchema;
use crate::utils::escape_upper_camel_case;
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

    token_stream_list.push(quote! {
      impl #struct_type {
        #from_str_fn

        #from_reader_fn

        pub fn deserialize_self<'de, R: super::deserializers::XmlReader<'de>>(
          xml_reader: &mut R,
        ) -> Result<Self, super::deserializers::DeError> {
          if let quick_xml::events::Event::Empty(e) = xml_reader.next()? {
            println!("{:?}", e);
          } else {
            Err(super::deserializers::DeError::UnknownError)?;
          }

          Err(super::deserializers::DeError::UnknownError)
        }
      }
    })
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
