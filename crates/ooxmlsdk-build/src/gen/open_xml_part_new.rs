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
  let mut field_unwrap_list: Vec<Stmt> = vec![];
  let mut field_ident_list: Vec<Ident> = vec![];
  let mut field_init_list: Vec<Stmt> = vec![];

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

    field_init_list.push(parse2(quote! {
      if file_path_set.contains(path) {
        init_self = true;

        let file = archive.by_name(path)?;

        root_element = Some(std::boxed::Box::new(#field_type::from_reader(std::io::BufReader::new(file))?));
      }
    }).unwrap());

    field_unwrap_list.push(
      parse2(quote! {
        let root_element = root_element
          .ok_or_else(|| crate::common::SdkError::CommonError("root_element".to_string()))?;
      })
      .unwrap(),
    );

    field_ident_list.push(parse_str("root_element").unwrap());
  } else if !part.extension.is_empty() {
    field_declaration_list.push(
      parse2(quote! {
        let mut path: Option<String> = None;
      })
      .unwrap(),
    );

    field_unwrap_list.push(
      parse2(quote! {
        let path = path
          .ok_or_else(|| crate::common::SdkError::CommonError("path".to_string()))?;
      })
      .unwrap(),
    );

    field_ident_list.push(parse_str("path").unwrap());
  } else if part.name == "CoreFilePropertiesPart" {
    field_declaration_list.push(
      parse2(quote! {
        let mut content: Option<String> = None;
      })
      .unwrap(),
    );

    field_unwrap_list.push(
      parse2(quote! {
        let content = content
          .ok_or_else(|| crate::common::SdkError::CommonError("content".to_string()))?;
      })
      .unwrap(),
    );

    field_ident_list.push(parse_str("content").unwrap());
  } else if part.base == "StylesPart" {
    field_declaration_list.push(
      parse2(quote! {
        let mut root_element: Option<std::boxed::Box<crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles>> = None;
      })
      .unwrap(),
    );

    field_unwrap_list.push(
      parse2(quote! {
        let root_element = root_element
          .ok_or_else(|| crate::common::SdkError::CommonError("root_element".to_string()))?;
      })
      .unwrap(),
    );

    field_ident_list.push(parse_str("root_element").unwrap());
  }

  let path_str = &part.paths.general;

  for child in &part.children {
    if child.is_data_part_reference {
      continue;
    }

    let child_part = context
      .part_name_part_map
      .get(child.name.as_str())
      .ok_or(format!("{:?}", child.name))
      .unwrap();

    let child_name_str = child.api_name.to_snake_case();

    let child_name_ident: Ident = parse_str(&child_name_str).unwrap();

    let child_type: Type = parse_str(&format!(
      "crate::parts::{}::{}",
      child.name.to_snake_case(),
      child.name.to_upper_camel_case(),
    ))
    .unwrap();

    let child_path = format!("{}/{}.xml", child_part.paths.general, child_part.target);

    if child.max_occurs_great_than_one {
      field_declaration_list.push(
        parse2(quote! {
          let mut #child_name_ident: Vec<#child_type> = vec![];
        })
        .unwrap(),
      );
    } else if child.min_occurs_is_non_zero {
      field_init_list.push(
        parse2(quote! {
          let #child_name_ident = #child_type::new_from_archive(
            &child_parent_path,
            &format!("{}{}", child_parent_path, #child_path),
            file_path_set,
            archive,
          )?;
        })
        .unwrap(),
      );

      field_unwrap_list.push(
        parse2(quote! {
          let #child_name_ident = #child_name_ident
            .ok_or_else(|| crate::common::SdkError::CommonError(#child_name_str.to_string()))?;
        })
        .unwrap(),
      );
    } else {
      field_init_list.push(
        parse2(quote! {
          let #child_name_ident = #child_type::new_from_archive(
            &child_parent_path,
            &format!("{}{}", child_parent_path, #child_path),
            file_path_set,
            archive,
          )?;
        })
        .unwrap(),
      );
    }

    field_ident_list.push(child_name_ident);
  }

  let part_new_from_archive_fn: ItemFn = parse2(quote! {
    pub(crate) fn new_from_archive(
      parent_path: &str,
      path: &str,
      file_path_set: &std::collections::HashSet<String>,
      archive: &mut zip::ZipArchive<std::io::BufReader<std::fs::File>>,
    ) -> Result<Option<std::boxed::Box<Self>>, crate::common::SdkError> {
      println!("{}", parent_path);
      println!("{}", path);

      let mut init_self = false;

      let child_parent_path = format!("{}{}", parent_path, #path_str);

      #( #field_declaration_list )*

      #( #field_init_list )*

      #( #field_unwrap_list )*

      if init_self {
        Ok(Some(std::boxed::Box::new(Self {
          #( #field_ident_list, )*
        })))
      } else {
        Ok(None)
      }
    }
  })
  .unwrap();

  if part.base == "OpenXmlPackage" {
    let part_new_fn: ItemFn = parse2(quote! {
      pub fn new(path: &str) -> Result<Self, crate::common::SdkError> {
        let zip_file = std::fs::File::open(path)?;

        let reader = std::io::BufReader::new(zip_file);

        let mut archive = zip::ZipArchive::new(reader)?;

        let mut file_path_set: std::collections::HashSet<String> = std::collections::HashSet::new();

        for i in 0..archive.len() {
          let file = archive.by_index(i)?;

          let file_path = match file.enclosed_name() {
            Some(path) => path.to_string_lossy().to_string(),
            None => {
              continue;
            }
          };

          file_path_set.insert(file_path);
        }

        let package = Self::new_from_archive("", "", &file_path_set, &mut archive)?
          .ok_or_else(|| crate::common::SdkError::CommonError("package".to_string()))?;

        Ok(*package)
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
