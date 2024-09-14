use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, parse_str, Arm, FieldValue, Ident, ItemFn, ItemImpl, ItemStruct, Stmt, Type};

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

  if part.base == "OpenXmlPackage" {
    let mut field_declaration_list: Vec<Stmt> = vec![];
    let mut field_match_list: Vec<Arm> = vec![];
    let mut field_unwrap_list: Vec<Stmt> = vec![];
    let mut child_assign_list: Vec<Stmt> = vec![];
    let mut self_field_value_list: Vec<FieldValue> = vec![];

    gen_part_fields(
      part,
      "",
      context,
      &mut field_declaration_list,
      &mut field_match_list,
      &mut field_unwrap_list,
      &mut child_assign_list,
      &mut self_field_value_list,
    );

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
          let file = archive.by_index(i).unwrap();

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

        #( #child_assign_list )*

        #( #field_unwrap_list )*

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

#[allow(clippy::too_many_arguments)]
fn gen_part_fields(
  part: &OpenXmlPart,
  prefix: &str,
  context: &GenContext,
  field_declaration_list: &mut Vec<Stmt>,
  field_match_list: &mut Vec<Arm>,
  field_unwrap_list: &mut Vec<Stmt>,
  child_assign_list: &mut Vec<Stmt>,
  self_field_value_list: &mut Vec<FieldValue>,
) {
  let part_mod = context
    .part_name_part_mod_map
    .get(part.name.as_str())
    .ok_or(format!("{:?}", part.name))
    .unwrap();

  let part_type: Type = parse_str(&format!(
    "crate::parts::{}::{}",
    part_mod,
    part.name.to_upper_camel_case()
  ))
  .unwrap();

  let mut child_field_value_list: Vec<FieldValue> = vec![];

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

    let root_element_ident: Ident = parse_str("root_element").unwrap();

    let prefix_element_ident: Ident =
      parse_str(&format!("{}_root_element", prefix.to_snake_case())).unwrap();

    if prefix.is_empty() {
      field_declaration_list.push(
        parse2(quote! {
          let #root_element_ident = None;
        })
        .unwrap(),
      );

      self_field_value_list.push(
        parse2(quote! {
          #root_element_ident
        })
        .unwrap(),
      )
    } else {
      field_declaration_list.push(
        parse2(quote! {
          let #prefix_element_ident = None;
        })
        .unwrap(),
      );

      child_field_value_list.push(
        parse2(quote! {
          #root_element_ident: #prefix_element_ident
        })
        .unwrap(),
      );
    }
  }

  for child in &part.children {
    if child.is_data_part_reference {
      continue;
    }

    if !child.has_fixed_content {
      if let Some(child_part) = context.part_name_part_map.get(child.name.as_str()) {
        gen_part_fields(
          child_part,
          &format!("{}{}", prefix, child_part.name),
          context,
          field_declaration_list,
          field_match_list,
          field_unwrap_list,
          child_assign_list,
          self_field_value_list,
        )
      }
    }

    let child_name_ident: Ident = parse_str(&child.api_name.to_snake_case()).unwrap();

    let prefix_child_name_ident: Ident =
      parse_str(&format!("{}{}", prefix, child.api_name).to_snake_case()).unwrap();

    field_declaration_list.push(
      parse2(quote! {
        let #prefix_child_name_ident = None;
      })
      .unwrap(),
    );

    if child.min_occurs_is_non_zero {
      field_unwrap_list.push(
        parse2(quote! {
          let #prefix_child_name_ident = #prefix_child_name_ident
            .ok_or_else(|| crate::common::SdkError::CommonError(#prefix_child_name_ident.to_string()))?;
        })
        .unwrap(),
      );
    }

    if prefix.is_empty() {
      self_field_value_list.push(
        parse2(quote! {
          #child_name_ident
        })
        .unwrap(),
      )
    } else {
      child_field_value_list.push(
        parse2(quote! {
          #child_name_ident: #prefix_child_name_ident
        })
        .unwrap(),
      );
    }
  }

  if !prefix.is_empty() {
    let child_ident: Ident = parse_str(&prefix.to_snake_case()).unwrap();

    child_assign_list.push(
      parse2(quote! {
        #child_ident = Some(#part_type {
          #( #child_field_value_list, )*
        });
      })
      .unwrap(),
    )
  }
}
