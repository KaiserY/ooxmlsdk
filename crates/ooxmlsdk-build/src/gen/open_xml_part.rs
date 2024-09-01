use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use std::error::Error;
use syn::{parse_str, Ident, Type};

use crate::models::OpenXmlPart;
use crate::GenContext;

pub fn gen_open_xml_part(
  part: &OpenXmlPart,
  _context: &GenContext,
) -> Result<TokenStream, Box<dyn Error>> {
  let mut token_stream_list: Vec<TokenStream> = vec![];

  let struct_name_ident: Ident = parse_str(&part.name.to_upper_camel_case())?;

  let mut fields: Vec<TokenStream> = vec![];

  for child in &part.children {
    if child.is_data_part_reference {
      continue;
    }

    let child_name_ident: Ident = parse_str(&child.api_name.to_snake_case())?;

    let child_type: Type = parse_str(&format!(
      "crate::parts::{}::{}",
      child.name.to_snake_case(),
      child.name.to_upper_camel_case(),
    ))?;

    if child.min_occurs_is_non_zero {
      fields.push(quote! {
        pub #child_name_ident: std::boxed::Box<#child_type>,
      })
    } else if child.max_occurs_great_than_one {
      fields.push(quote! {
        pub #child_name_ident: Vec<#child_type>,
      })
    } else {
      fields.push(quote! {
        pub #child_name_ident: Option<std::boxed::Box<#child_type>>,
      })
    }
  }

  token_stream_list.push(quote! {
    #[derive(Clone, Debug)]
    pub struct #struct_name_ident {
      #( #fields )*
    }
  });

  Ok(quote! {
    #( #token_stream_list )*
  })
}
