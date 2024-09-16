use heck::{ToSnakeCase, ToUpperCamelCase};
use path_clean::clean;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, parse_str, Arm, FieldValue, Ident, ItemFn, ItemImpl, ItemStruct, Stmt, Type};

use crate::models::{OpenXmlPart, OpenXmlPartChild};
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

  if part.base == "OpenXmlPackage" {
    let mut field_declaration_list: Vec<Stmt> = vec![];
    let mut field_match_list: Vec<Arm> = vec![];
    let mut self_field_stmt_list: Vec<Stmt> = vec![];
    let mut child_field_stmt_list: Vec<Stmt> = vec![];
    let mut self_field_value_list: Vec<FieldValue> = vec![];

    for child in &part.children {
      if child.is_data_part_reference {
        continue;
      }

      gen_child_part_fields(
        child,
        &child.api_name,
        &part.paths.general,
        context,
        &mut field_declaration_list,
        &mut field_match_list,
        &mut child_field_stmt_list,
      );

      let child_name_str = child.api_name.to_snake_case();

      let child_name_ident: Ident = parse_str(&child_name_str).unwrap();

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
      } else if child.min_occurs_is_non_zero {
        field_declaration_list.push(
          parse2(quote! {
            let mut #child_name_ident: Option<std::boxed::Box<#child_type>> = None;
          })
          .unwrap(),
        );

        self_field_stmt_list.push(
          parse2(quote! {
            let #child_name_ident = #child_name_ident
              .ok_or_else(|| crate::common::SdkError::CommonError(#child_name_str.to_string()))?;
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

      self_field_value_list.push(
        parse2(quote! {
          #child_name_ident
        })
        .unwrap(),
      )
    }

    field_match_list.push(
      parse2(quote! {
        _ => ()
      })
      .unwrap(),
    );

    let part_new_fn: ItemFn = parse2(quote! {
      pub fn new(path: &str) -> Result<Self, crate::common::SdkError> {
        let zip_file = std::fs::File::open(path)?;

        let reader = std::io::BufReader::new(zip_file);

        let mut archive = zip::ZipArchive::new(reader)?;

        #( #field_declaration_list )*

        for i in 0..archive.len() {
          let file = archive.by_index(i)?;

          let file_path = match file.enclosed_name() {
            Some(path) => path.to_string_lossy().to_string(),
            None => {
              continue;
            }
          };

          match file_path.as_str() {
            #( #field_match_list, )*
          }
        }

        #( #child_field_stmt_list )*

        #( #self_field_stmt_list )*

        Ok(Self {
          #( #self_field_value_list, )*
        })
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

fn gen_child_part_fields(
  part_child: &OpenXmlPartChild,
  part_prefix: &str,
  path_prefix: &str,
  context: &GenContext,
  field_declaration_list: &mut Vec<Stmt>,
  field_match_list: &mut Vec<Arm>,
  child_field_stmt_list: &mut Vec<Stmt>,
) {
  let mut child_field_value_list: Vec<FieldValue> = vec![];

  let child_part = context
    .part_name_part_map
    .get(part_child.name.as_str())
    .ok_or(format!("{:?}", part_child.name))
    .unwrap();

  let init_ident: Ident = parse_str(&format!("Init{}", part_prefix).to_snake_case()).unwrap();

  field_declaration_list.push(
    parse2(quote! {
      let mut #init_ident = false;
    })
    .unwrap(),
  );

  if let Some(root_element_type) = context.part_name_type_map.get(child_part.name.as_str()) {
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

    let root_element_ident: Ident = parse_str("root_element").unwrap();

    let prefix_root_element = format!("{}_root_element", part_prefix.to_snake_case());

    let prefix_root_element_ident: Ident = parse_str(&prefix_root_element).unwrap();

    field_declaration_list.push(
      parse2(quote! {
        let mut #prefix_root_element_ident: Option<std::boxed::Box<#field_type>> = None;
      })
      .unwrap(),
    );

    child_field_value_list.push(
      parse2(quote! {
        #root_element_ident: #prefix_root_element_ident
        .ok_or_else(|| crate::common::SdkError::CommonError(#prefix_root_element.to_string()))?
      })
      .unwrap(),
    );

    let root_element_path_str = format!(
      "{}{}/{}.xml",
      path_prefix, child_part.paths.general, child_part.target
    );

    let root_element_path = clean(root_element_path_str);

    let root_element_path = root_element_path.to_string_lossy().replace("\\", "/");

    field_match_list.push(
      parse2(quote! {
        #root_element_path => {
          #init_ident = true;
          #prefix_root_element_ident = Some(std::boxed::Box::new(#field_type::from_reader(std::io::BufReader::new(file))?));
        }
      })
      .unwrap(),
    );
  }

  if !part_child.max_occurs_great_than_one {
    if part_child.has_fixed_content {
      for child in &child_part.children {
        if child.is_data_part_reference {
          continue;
        }

        let child_name_ident: Ident = parse_str(&child.api_name.to_snake_case()).unwrap();

        if child.max_occurs_great_than_one {
          child_field_value_list.push(
            parse2(quote! {
              #child_name_ident: vec![]
            })
            .unwrap(),
          );
        } else {
          child_field_value_list.push(
            parse2(quote! {
              #child_name_ident: None
            })
            .unwrap(),
          );
        }
      }
    } else {
      for child in &child_part.children {
        if child.is_data_part_reference {
          continue;
        }

        gen_child_part_fields(
          child,
          &format!("{}{}", part_prefix, child.api_name),
          &format!("{}{}/", path_prefix, child_part.paths.general),
          context,
          field_declaration_list,
          field_match_list,
          child_field_stmt_list,
        );

        let child_name_ident: Ident = parse_str(&child.api_name.to_snake_case()).unwrap();

        let prefix_child_name_str = format!("{}{}", part_prefix, child.api_name).to_snake_case();

        let prefix_child_name_ident: Ident = parse_str(&prefix_child_name_str).unwrap();

        let child_type: Type = parse_str(&format!(
          "crate::parts::{}::{}",
          child.name.to_snake_case(),
          child.name.to_upper_camel_case(),
        ))
        .unwrap();

        child_field_value_list.push(
          parse2(quote! {
            #child_name_ident: #prefix_child_name_ident
          })
          .unwrap(),
        );

        if child.max_occurs_great_than_one {
          field_declaration_list.push(
            parse2(quote! {
              let mut #prefix_child_name_ident: Vec<#child_type> = vec![];
            })
            .unwrap(),
          );
        } else if child.min_occurs_is_non_zero {
          field_declaration_list.push(
            parse2(quote! {
              let mut #prefix_child_name_ident: Option<std::boxed::Box<#child_type>> = None;
            })
            .unwrap(),
          );

          child_field_stmt_list.push(
          parse2(quote! {
            let #prefix_child_name_ident = #prefix_child_name_ident
              .ok_or_else(|| crate::common::SdkError::CommonError(#prefix_child_name_str.to_string()))?;
          })
          .unwrap(),
        );
        } else {
          field_declaration_list.push(
            parse2(quote! {
              let mut #prefix_child_name_ident: Option<std::boxed::Box<#child_type>> = None;
            })
            .unwrap(),
          );
        }
      }
    }

    let prefix_ident: Ident = parse_str(&part_prefix.to_snake_case()).unwrap();

    let child_type: Type = parse_str(&format!(
      "crate::parts::{}::{}",
      part_child.name.to_snake_case(),
      part_child.name.to_upper_camel_case(),
    ))
    .unwrap();

    child_field_stmt_list.push(
      parse2(quote! {
        if #init_ident {
          #prefix_ident = Some(std::boxed::Box::new(#child_type {
            #( #child_field_value_list, )*
          }));
        }
      })
      .unwrap(),
    );
  }
}
