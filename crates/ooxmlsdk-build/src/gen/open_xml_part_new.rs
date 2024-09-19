use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, parse_str, Ident, ItemFn, ItemImpl, ItemStruct, Stmt, Type};

use crate::models::OpenXmlPart;
use crate::GenContext;

pub fn gen_open_xml_part(part: &OpenXmlPart, context: &GenContext) -> TokenStream {
  let struct_name_ident: Ident = parse_str(&part.name.to_upper_camel_case()).unwrap();

  let mut fields: Vec<TokenStream> = vec![];

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
  } else if !part.extension.is_empty() {
    fields.push(quote! {
      pub path: String,
    });
  } else if part.name == "CoreFilePropertiesPart" {
    fields.push(quote! {
      pub content: String,
    });
  } else if part.base == "StylesPart" {
    fields.push(quote! {
      pub root_element: std::boxed::Box<crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles>,
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

    if child.max_occurs_great_than_one {
      fields.push(quote! {
        pub #child_name_ident: Vec<#child_type>,
      })
    } else if child.min_occurs_is_non_zero {
      fields.push(quote! {
        pub #child_name_ident: std::boxed::Box<#child_type>,
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

  let mut field_declaration_list: Vec<Stmt> = vec![];
  let mut field_init_list: Vec<Ident> = vec![];

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

    field_declaration_list.push(
      parse2(quote! {
        let mut root_element: Option<std::boxed::Box<#field_type>> = None;
      })
      .unwrap(),
    );

    field_init_list.push(parse_str("root_element").unwrap());
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

    if child.max_occurs_great_than_one {
      field_declaration_list.push(
        parse2(quote! {
          let mut #child_name_ident: Vec<#child_type> = vec![];
        })
        .unwrap(),
      );
    } else {
      field_declaration_list.push(
        parse2(quote! {
          let mut #child_name_ident: Option<std::boxed::Box<#child_type>> = None;
        })
        .unwrap(),
      );
    }

    field_init_list.push(child_name_ident);
  }

  let part_new_from_archive_fn: ItemFn = parse2(quote! {
    pub fn new_from_archive(
      dir: &str,
      file_path_list: &Vec<String>,
      archive: &mut zip::ZipArchive<std::io::BufReader<std::fs::File>>,
    ) -> Result<Self, crate::common::SdkError> {
      #( #field_declaration_list )*

      Ok(Self {
        #( #field_init_list, )*
      })
    }
  })
  .unwrap();

  if part.base == "OpenXmlPackage" {
    let part_new_fn: ItemFn = parse2(quote! {
      pub fn new(path: &str) -> Result<Self, crate::common::SdkError> {
        let zip_file = std::fs::File::open(path)?;

        let reader = std::io::BufReader::new(zip_file);

        let mut archive = zip::ZipArchive::new(reader)?;

        let mut file_path_list: Vec<String> = vec![];

        for i in 0..archive.len() {
          let file = archive.by_index(i)?;

          let file_path = match file.enclosed_name() {
            Some(path) => path.to_string_lossy().to_string(),
            None => {
              continue;
            }
          };

          file_path_list.push(file_path);
        }

        Self::new_from_archive("", &file_path_list, &mut archive)
      }
    })
    .unwrap();

    let part_impl: ItemImpl = parse2(quote! {
      impl #struct_name_ident {
        #part_new_fn

        #part_new_from_archive_fn
      }
    })
    .unwrap();

    quote! {
      #part_struct

      #part_impl
    }
  } else {
    let part_impl: ItemImpl = parse2(quote! {
      impl #struct_name_ident {
        #part_new_from_archive_fn
      }
    })
    .unwrap();

    quote! {
      #part_struct

      #part_impl
    }
  }
}
