use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, parse_str, Arm, Ident, ItemFn, ItemImpl, ItemStruct, Stmt, Type};

use crate::models::OpenXmlPart;
use crate::GenContext;

pub fn gen_open_xml_part(part: &OpenXmlPart, context: &GenContext) -> TokenStream {
  let struct_name_ident: Ident = parse_str(&part.name.to_upper_camel_case()).unwrap();

  let mut fields: Vec<TokenStream> = vec![];

  if part.base != "OpenXmlPackage" {
    fields.push(quote! {
      pub inner_path: String,
    });

    fields.push(quote! {
      pub outer_path: String,
    });
  }

  if let Some(root_element_type) = context.part_name_type_map.get(part.name.as_str()) {
    let root_element_type_namespace = context
      .type_name_namespace_map
      .get(root_element_type.name.as_str())
      .ok_or(format!("{:?}", root_element_type.name))
      .unwrap();

    let scheme_mod = context
      .prefix_schema_mod_map
      .get(root_element_type_namespace.prefix.as_str())
      .ok_or(format!("{:?}", root_element_type_namespace.prefix))
      .unwrap();

    let field_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      scheme_mod,
      root_element_type.class_name.to_upper_camel_case()
    ))
    .unwrap();

    fields.push(quote! {
      pub root_element: std::boxed::Box<#field_type>,
    });
  }

  for child in &part.children {
    if child.is_data_part_reference {
      continue;
    }

    let child_name_ident: Ident = parse_str(&child.api_name.to_snake_case()).unwrap();

    let child_type: Type = parse_str(&format!(
      "crate::parts::{}::{}",
      child.name.to_snake_case(),
      child.name.to_upper_camel_case(),
    ))
    .unwrap();

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

  let part_struct: ItemStruct = parse2(quote! {
    #[derive(Clone, Debug)]
    pub struct #struct_name_ident {
      #( #fields )*
    }
  })
  .unwrap();

  if part.base == "_OpenXmlPackage" {
    let field_declaration_list: Vec<Stmt> = vec![];
    let field_match_list: Vec<Arm> = vec![];
    let field_unwrap_list: Vec<Stmt> = vec![];
    let field_init_list: Vec<Stmt> = vec![];
    let field_ident_list: Vec<Ident> = vec![];

    let part_new_fn: ItemFn = parse2(quote! {
      pub fn new(_path: &str) -> Result<Self, crate::common::SdkError> {
        let zip_file = std::fs::File::open(path)?;

        let reader = std::io::BufReader::new(zip_file);

        let mut archive = zip::ZipArchive::new(reader)?;

        #( #field_declaration_list )*

        for i in 0..archive.len() {
          let file = archive.by_index(i).unwrap();

          let file_path = match file.enclosed_name() {
            Some(path) => path.to_string_lossy().to_string(),
            None => {
              continue;
            }
          };

          match file_path.as_str() {
            #( #field_match_list, )*
            _ => (),
          }
        }

        #( #field_unwrap_list, )*

        #( #field_init_list, )*

        Self {
          #( #field_ident_list, )*
        }
      }
    })
    .unwrap();

    let part_impl: ItemImpl = parse2(quote! {
      impl #struct_name_ident {
        #part_new_fn
      }
    })
    .unwrap();

    quote! {
      #part_struct

      #part_impl
    }
  } else {
    quote! {
      #part_struct
    }
  }
}
