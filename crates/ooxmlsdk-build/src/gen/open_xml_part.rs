use heck::{ToSnakeCase, ToUpperCamelCase};
use path_clean::clean;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{
  parse2, parse_str, Arm, Block, Expr, FieldValue, Ident, ItemFn, ItemImpl, ItemStruct, Stmt, Type,
};

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

  if part.base == "OpenXmlPackage" {
    let mut field_declaration_list: Vec<Stmt> = vec![];
    let mut field_match_list: Vec<Arm> = vec![];
    let mut self_field_stmt_list: Vec<Stmt> = vec![];
    let mut child_field_stmt_list: Vec<Stmt> = vec![];
    let mut self_field_value_list: Vec<FieldValue> = vec![];
    let mut match_default_expr_list: Vec<Expr> = vec![];
    let mut match_default_block_list: Vec<Block> = vec![];

    for child in &part.children {
      if child.is_data_part_reference {
        continue;
      }

      gen_child_part_fields(
        child,
        &part.name,
        &child.api_name,
        &part.paths.general,
        context,
        &mut field_declaration_list,
        &mut field_match_list,
        &mut child_field_stmt_list,
        &mut match_default_expr_list,
        &mut match_default_block_list,
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

    let part_new_fn: ItemFn = parse2(quote! {
      pub fn new(path: &str) -> Result<Self, crate::common::SdkError> {
        use std::io::Read;

        let zip_file = std::fs::File::open(path)?;

        let reader = std::io::BufReader::new(zip_file);

        let mut archive = zip::ZipArchive::new(reader)?;

        #( #field_declaration_list )*

        for i in 0..archive.len() {
          let mut file = archive.by_index(i)?;

          let file_path = match file.enclosed_name() {
            Some(path) => path.to_string_lossy().to_string(),
            None => {
              continue;
            }
          };

          match file_path.as_str() {
            #( #field_match_list, )*
            _ => {
              if file_path == "[Content_Types].xml" {

              }
              #(else if #match_default_expr_list #match_default_block_list )*
            }
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

    let mut directory_set: HashSet<String> = HashSet::new();

    let mut writer_stmt_list: Vec<Stmt> = vec![];

    for child in &part.children {
      if child.is_data_part_reference {
        continue;
      }

      gen_child_writer_stmt_list(
        child,
        &part.name,
        "",
        "",
        true,
        context,
        &mut directory_set,
        &mut writer_stmt_list,
      );
    }

    let add_directory_stmt_list: Vec<Stmt> = directory_set
      .iter()
      .map(|d| {
        parse2(quote! {
          zip.add_directory(#d, zip::write::SimpleFileOptions::default())?;
        })
        .unwrap()
      })
      .collect();

    let part_save_fn: ItemFn = parse2(quote! {
      pub fn save(&self, path: &str) -> Result<(), crate::common::SdkError> {
        use std::io::prelude::*;

        let path = std::path::Path::new(path);

        let file = std::fs::File::create(path)?;

        let mut zip = zip::ZipWriter::new(file);

        zip.add_directory("test/", zip::write::SimpleFileOptions::default())?;

        #( #add_directory_stmt_list )*

        let options = zip::write::SimpleFileOptions::default()
          .compression_method(zip::CompressionMethod::Stored)
          .unix_permissions(0o755);

        zip.start_file("test/aa.txt", options)?;
        zip.write_all(b"Hello, World!\n")?;

        #( #writer_stmt_list )*

        zip.finish()?;

        Ok(())
      }
    })
    .unwrap();

    let part_impl: ItemImpl = parse2(quote! {
      impl #struct_name_ident {
        #part_new_fn

        #part_save_fn
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
fn gen_child_part_fields(
  part_child: &OpenXmlPartChild,
  package: &str,
  part_prefix: &str,
  path_prefix: &str,
  context: &GenContext,
  field_declaration_list: &mut Vec<Stmt>,
  field_match_list: &mut Vec<Arm>,
  child_field_stmt_list: &mut Vec<Stmt>,
  match_default_expr_list: &mut Vec<Expr>,
  match_default_block_list: &mut Vec<Block>,
) {
  let mut child_field_value_list: Vec<FieldValue> = vec![];

  let child_part = context
    .part_name_part_map
    .get(part_child.name.as_str())
    .ok_or(format!("{:?}", part_child.name))
    .unwrap();

  let prefix_ident: Ident = parse_str(&part_prefix.to_snake_case()).unwrap();

  let child_type: Type = parse_str(&format!(
    "crate::parts::{}::{}",
    part_child.name.to_snake_case(),
    part_child.name.to_upper_camel_case(),
  ))
  .unwrap();

  let init_ident: Ident = parse_str(&format!("Init{}", part_prefix).to_snake_case()).unwrap();

  if part_child.max_occurs_great_than_one {
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

      let root_element_path_str = format!(
        "{}{}/{}.xml",
        path_prefix, child_part.paths.general, child_part.target
      );

      let root_element_path = clean(root_element_path_str);

      let root_element_path = root_element_path.to_string_lossy().replace("\\", "/");

      let mut child_init_list: Vec<FieldValue> = vec![];

      for child in &child_part.children {
        if child.is_data_part_reference {
          continue;
        }

        let child_name_ident: Ident = parse_str(&child.api_name.to_snake_case()).unwrap();

        if child.max_occurs_great_than_one {
          child_init_list.push(
            parse2(quote! {
              #child_name_ident: vec![]
            })
            .unwrap(),
          );
        } else {
          child_init_list.push(
            parse2(quote! {
              #child_name_ident: None
            })
            .unwrap(),
          );
        }
      }

      field_match_list.push(
        parse2(quote! {
          #root_element_path => {
            #prefix_ident.push(#child_type {
              #root_element_ident: std::boxed::Box::new(#field_type::from_reader(std::io::BufReader::new(file))?),
              #( #child_init_list, )*
            });
          }
        })
        .unwrap(),
      );
    } else if !child_part.extension.is_empty() {
      if part_child.max_occurs_great_than_one {
        let target_path_str = format!(
          "{}{}/{}",
          path_prefix, child_part.paths.general, child_part.target
        );

        let target_path = clean(target_path_str);

        let target_path = target_path.to_string_lossy().replace("\\", "/");

        match_default_expr_list.push(
          parse2(quote! {
            file_path.starts_with(#target_path)
          })
          .unwrap(),
        );

        let mut child_init_list: Vec<FieldValue> = vec![];

        for child in &child_part.children {
          if child.is_data_part_reference {
            continue;
          }

          let child_name_ident: Ident = parse_str(&child.api_name.to_snake_case()).unwrap();

          if child.max_occurs_great_than_one {
            child_init_list.push(
              parse2(quote! {
                #child_name_ident: vec![]
              })
              .unwrap(),
            );
          } else {
            child_init_list.push(
              parse2(quote! {
                #child_name_ident: None
              })
              .unwrap(),
            );
          }
        }

        match_default_block_list.push(
          parse2(quote! {{
            #prefix_ident.push(#child_type {
              path: file_path.to_string(),
              #( #child_init_list, )*
            });
          }})
          .unwrap(),
        );
      } else {
        let target_path_ident: Ident = parse_str("path").unwrap();

        let prefix_target_path = format!("{}_path", part_prefix.to_snake_case());

        let prefix_target_path_ident: Ident = parse_str(&prefix_target_path).unwrap();

        field_declaration_list.push(
          parse2(quote! {
            let mut #prefix_target_path_ident: Option<String> = None;
          })
          .unwrap(),
        );

        child_field_value_list.push(
          parse2(quote! {
            #target_path_ident: #prefix_target_path_ident
            .ok_or_else(|| crate::common::SdkError::CommonError(#prefix_target_path.to_string()))?
          })
          .unwrap(),
        );

        let target_path_str = format!(
          "{}{}/{}",
          path_prefix, child_part.paths.general, child_part.target
        );

        let target_path = clean(target_path_str);

        let target_path = target_path.to_string_lossy().replace("\\", "/");

        match_default_expr_list.push(
          parse2(quote! {
            file_path.starts_with(#target_path)
          })
          .unwrap(),
        );

        match_default_block_list.push(
          parse2(quote! {{
            #init_ident = true;

            #prefix_target_path_ident = Some(file_path.to_string());
          }})
          .unwrap(),
        );
      }
    } else if part_child.api_name == "CustomXmlParts" {
      let child_part_child = context
        .part_name_part_map
        .get(child_part.children[0].name.as_str())
        .ok_or(format!("{:?}", child_part.children[0].name))
        .unwrap();

      let child_part_child_name_ident: Ident =
        parse_str(&child_part.children[0].api_name.to_snake_case()).unwrap();

      let child_part_child_type: Type = parse_str(&format!(
        "crate::parts::{}::{}",
        child_part_child.name.to_snake_case(),
        child_part_child.name.to_upper_camel_case(),
      ))
      .unwrap();

      if let Some(root_element_type) = context
        .part_name_type_map
        .get(child_part_child.name.as_str())
      {
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

        let target_path_str = format!(
          "{}{}/{}/{}",
          path_prefix, child_part.paths.general, child_part.target, child_part_child.target
        );

        let target_path = clean(target_path_str);

        let target_path = target_path.to_string_lossy().replace("\\", "/");

        match_default_expr_list.push(
          parse2(quote! {
            file_path.starts_with(#target_path)
          })
          .unwrap(),
        );

        match_default_block_list.push(
          parse2(quote! {{
            #prefix_ident.push(#child_type {
              #child_part_child_name_ident: Some(std::boxed::Box::new(#child_part_child_type {
                #root_element_ident: std::boxed::Box::new(#field_type::from_reader(std::io::BufReader::new(file))?),
              })),
            });
          }})
          .unwrap(),
        );
      }
    }
  } else if part_child.name == "CoreFilePropertiesPart" {
    let content_path_str = format!(
      "{}{}/{}.xml",
      path_prefix, child_part.paths.general, child_part.target
    );

    let content_path = clean(content_path_str);

    let content_path = content_path.to_string_lossy().replace("\\", "/");

    field_match_list.push(
      parse2(quote! {
        #content_path => {
          let mut buffer = String::new();

          file.read_to_string(&mut buffer)?;

          #prefix_ident = Some(std::boxed::Box::new(#child_type {
            content: buffer,
          }));
        }
      })
      .unwrap(),
    );
  } else if child_part.base == "StylesPart" {
    field_declaration_list.push(
      parse2(quote! {
        let mut #init_ident = false;
      })
      .unwrap(),
    );

    let field_type: Type =
      parse_str("crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles")
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
  } else {
    field_declaration_list.push(
      parse2(quote! {
        #[allow(unused_mut)]
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

      let root_element_path_str =
        if !child_part.paths.word.is_empty() && package == "WordprocessingDocument" {
          format!(
            "{}{}/{}.xml",
            path_prefix, child_part.paths.word, child_part.target
          )
        } else if !child_part.paths.excel.is_empty() && package == "SpreadsheetDocument" {
          format!(
            "{}{}/{}.xml",
            path_prefix, child_part.paths.excel, child_part.target
          )
        } else if !child_part.paths.power_point.is_empty() && package == "PresentationDocument" {
          format!(
            "{}{}/{}.xml",
            path_prefix, child_part.paths.power_point, child_part.target
          )
        } else {
          format!(
            "{}{}/{}.xml",
            path_prefix, child_part.paths.general, child_part.target
          )
        };

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
    } else if !child_part.extension.is_empty() {
      if part_child.max_occurs_great_than_one {
        let target_path_str = format!(
          "{}{}/{}",
          path_prefix, child_part.paths.general, child_part.target
        );

        let target_path = clean(target_path_str);

        let target_path = target_path.to_string_lossy().replace("\\", "/");

        match_default_expr_list.push(
          parse2(quote! {
            file_path.starts_with(#target_path)
          })
          .unwrap(),
        );

        let mut child_init_list: Vec<FieldValue> = vec![];

        for child in &child_part.children {
          if child.is_data_part_reference {
            continue;
          }

          let child_name_ident: Ident = parse_str(&child.api_name.to_snake_case()).unwrap();

          if child.max_occurs_great_than_one {
            child_init_list.push(
              parse2(quote! {
                #child_name_ident: vec![]
              })
              .unwrap(),
            );
          } else {
            child_init_list.push(
              parse2(quote! {
                #child_name_ident: None
              })
              .unwrap(),
            );
          }
        }

        match_default_block_list.push(
          parse2(quote! {{
            #prefix_ident.push(#child_type {
              path: file_path.to_string(),
              #( #child_init_list, )*
            });
          }})
          .unwrap(),
        );
      } else {
        let target_path_ident: Ident = parse_str("path").unwrap();

        let prefix_target_path = format!("{}_path", part_prefix.to_snake_case());

        let prefix_target_path_ident: Ident = parse_str(&prefix_target_path).unwrap();

        field_declaration_list.push(
          parse2(quote! {
            let mut #prefix_target_path_ident: Option<String> = None;
          })
          .unwrap(),
        );

        child_field_value_list.push(
          parse2(quote! {
            #target_path_ident: #prefix_target_path_ident
            .ok_or_else(|| crate::common::SdkError::CommonError(#prefix_target_path.to_string()))?
          })
          .unwrap(),
        );

        let target_path_str = format!(
          "{}{}/{}",
          path_prefix, child_part.paths.general, child_part.target
        );

        let target_path = clean(target_path_str);

        let target_path = target_path.to_string_lossy().replace("\\", "/");

        match_default_expr_list.push(
          parse2(quote! {
            file_path.starts_with(#target_path)
          })
          .unwrap(),
        );

        match_default_block_list.push(
          parse2(quote! {{
            #init_ident = true;

            #prefix_target_path_ident = Some(file_path.to_string());
          }})
          .unwrap(),
        );
      }
    }

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
          package,
          &format!("{}{}", part_prefix, child.api_name),
          &format!("{}{}/", path_prefix, child_part.paths.general),
          context,
          field_declaration_list,
          field_match_list,
          child_field_stmt_list,
          match_default_expr_list,
          match_default_block_list,
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
              #[allow(unused_mut)]
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

#[allow(clippy::too_many_arguments)]
fn gen_child_writer_stmt_list(
  part_child: &OpenXmlPartChild,
  package: &str,
  _part_prefix: &str,
  path_prefix: &str,
  with_xmlns: bool,
  context: &GenContext,
  directory_set: &mut HashSet<String>,
  writer_stmt_list: &mut Vec<Stmt>,
) {
  let child_part = context
    .part_name_part_map
    .get(part_child.name.as_str())
    .ok_or(format!("{:?}", part_child.name))
    .unwrap();

  if context
    .part_name_type_map
    .contains_key(child_part.name.as_str())
  {
    let child_ident: Ident = parse_str(&part_child.api_name.to_snake_case()).unwrap();

    let root_element_path_str =
      if !child_part.paths.word.is_empty() && package == "WordprocessingDocument" {
        format!("{}{}/", path_prefix, child_part.paths.word)
      } else if !child_part.paths.excel.is_empty() && package == "SpreadsheetDocument" {
        format!("{}{}/", path_prefix, child_part.paths.excel)
      } else if !child_part.paths.power_point.is_empty() && package == "PresentationDocument" {
        format!("{}{}/", path_prefix, child_part.paths.power_point)
      } else {
        format!("{}{}/", path_prefix, child_part.paths.general)
      };

    let root_element_file_str = format!("{}{}.xml", root_element_path_str, child_part.target);

    let root_element_path = clean(root_element_path_str)
      .to_string_lossy()
      .replace("\\", "/");

    let root_element_file_path = clean(root_element_file_str)
      .to_string_lossy()
      .replace("\\", "/");

    directory_set.insert(root_element_path);

    if part_child.max_occurs_great_than_one {
      writer_stmt_list.push(
        parse2(quote! {
          for child in &self.#child_ident {
            zip.start_file(#root_element_file_path, options)?;

            zip.write_all(child.root_element.to_string(#with_xmlns)?.as_bytes())?;
          }
        })
        .unwrap(),
      );
    } else if part_child.min_occurs_is_non_zero {
      writer_stmt_list.push(
        parse2(quote! {
          zip.start_file(#root_element_file_path, options)?;
        })
        .unwrap(),
      );

      writer_stmt_list.push(
        parse2(quote! {
          zip.write_all(&self.#child_ident.root_element.to_string(#with_xmlns)?.as_bytes())?;
        })
        .unwrap(),
      );
    } else {
      writer_stmt_list.push(
        parse2(quote! {
          if let Some(#child_ident) = &self.#child_ident {
            zip.start_file(#root_element_file_path, options)?;

            zip.write_all(#child_ident.root_element.to_string(#with_xmlns)?.as_bytes())?;
          }
        })
        .unwrap(),
      );
    }
  }
}
