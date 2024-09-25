use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, parse_str, FieldValue, Ident, ItemFn, ItemImpl, ItemStruct, Stmt, Type};

use crate::models::OpenXmlPart;
use crate::GenContext;

pub fn gen_open_xml_part(part: &OpenXmlPart, context: &GenContext) -> TokenStream {
  let struct_name_ident: Ident = parse_str(&part.name.to_upper_camel_case()).unwrap();

  let mut fields: Vec<TokenStream> = vec![];

  if part.base == "OpenXmlPackage" {
    fields.push(quote! {
      pub content_types: String,
    });
  }

  if part.name == "CustomXmlPart" {
    fields.push(quote! {
      pub prefix: String,
    });

    fields.push(quote! {
      pub content: String,
    });
  } else if !part.extension.is_empty()
    || part.name == "CustomDataPart"
    || part.name == "InternationalMacroSheetPart"
  {
    fields.push(quote! {
      pub inner_path: String,
    });

    fields.push(quote! {
      pub path: String,
    });
  } else if context.target_type_map.contains_key(&part.root_element)
    || context.target_type_map.contains_key(&part.target)
  {
    let target_type = if let Some(target_type) = context.target_type_map.get(&part.root_element) {
      target_type
    } else if let Some(target_type) = context.target_type_map.get(&part.target) {
      target_type
    } else {
      panic!("{:?}", part);
    };

    let target_type_namespace = context
      .type_name_namespace_map
      .get(target_type.name.as_str())
      .ok_or(format!("{:?}", target_type.name))
      .unwrap();

    let scheme_mod = context
      .prefix_schema_mod_map
      .get(target_type_namespace.prefix.as_str())
      .ok_or(format!("{:?}", target_type_namespace.prefix))
      .unwrap();

    let field_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      scheme_mod,
      target_type.class_name.to_upper_camel_case()
    ))
    .unwrap();

    fields.push(quote! {
      pub root_element: std::boxed::Box<#field_type>,
    });
  } else if part.name == "CoreFilePropertiesPart" || part.name == "XmlSignaturePart" {
    fields.push(quote! {
      pub content: String,
    });
  } else if let Some(root_element_type) = context.part_name_type_map.get(part.name.as_str()) {
    if part.name == "CustomXmlPropertiesPart" || part.name == "ThemePart" {
      fields.push(quote! {
        pub prefix: String,
      });
    }

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
      });
    } else if child.min_occurs_is_non_zero {
      fields.push(quote! {
        pub #child_name_ident: std::boxed::Box<#child_type>,
      });
    } else {
      fields.push(quote! {
        pub #child_name_ident: Option<std::boxed::Box<#child_type>>,
      });
    }
  }

  let part_struct: ItemStruct = parse2(quote! {
    #[derive(Clone)]
    pub struct #struct_name_ident {
      #( #fields )*
    }
  })
  .unwrap();

  let mut field_declaration_list: Vec<Stmt> = vec![];
  let mut field_unwrap_list: Vec<Stmt> = vec![];
  let mut self_field_value_list: Vec<FieldValue> = vec![];
  let mut field_init_list: Vec<Stmt> = vec![];

  if part.name == "CustomXmlPart" {
    field_declaration_list.push(
      parse2(quote! {
        use std::io::Read;
      })
      .unwrap(),
    );

    field_declaration_list.push(
      parse2(quote! {
        let mut content = String::new();
      })
      .unwrap(),
    );

    field_init_list.push(
      parse2(quote! {
        {
          let mut file = std::io::BufReader::new(archive.by_name(path)?);

          file.read_to_string(&mut content)?;
        };
      })
      .unwrap(),
    );

    field_init_list.push(
      parse2(quote! {
        let prefix = path.replace("customXml/item", "").replace(".xml", "");
      })
      .unwrap(),
    );

    self_field_value_list.push(
      parse2(quote! {
        prefix
      })
      .unwrap(),
    );

    self_field_value_list.push(
      parse2(quote! {
        content
      })
      .unwrap(),
    );
  } else if !part.extension.is_empty()
    || part.name == "CustomDataPart"
    || part.name == "InternationalMacroSheetPart"
  {
    self_field_value_list.push(
      parse2(quote! {
        inner_path: path.to_string()
      })
      .unwrap(),
    );

    self_field_value_list.push(
      parse2(quote! {
        path: path.to_string()
      })
      .unwrap(),
    );
  } else if context.target_type_map.contains_key(&part.root_element)
    || context.target_type_map.contains_key(&part.target)
  {
    let target_type = if let Some(target_type) = context.target_type_map.get(&part.root_element) {
      target_type
    } else if let Some(target_type) = context.target_type_map.get(&part.target) {
      target_type
    } else {
      panic!("{:?}", part);
    };

    let target_type_namespace = context
      .type_name_namespace_map
      .get(target_type.name.as_str())
      .ok_or(format!("{:?}", target_type.name))
      .unwrap();

    let scheme_mod = context
      .prefix_schema_mod_map
      .get(target_type_namespace.prefix.as_str())
      .ok_or(format!("{:?}", target_type_namespace.prefix))
      .unwrap();

    let field_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      scheme_mod,
      target_type.class_name.to_upper_camel_case()
    ))
    .unwrap();

    field_init_list.push(parse2(quote! {
      let root_element = Some(std::boxed::Box::new(#field_type::from_reader(std::io::BufReader::new(archive.by_name(path)?))?));
    }).unwrap());

    field_unwrap_list.push(
      parse2(quote! {
        let root_element = root_element
          .ok_or_else(|| crate::common::SdkError::CommonError("root_element".to_string()))?;
      })
      .unwrap(),
    );

    self_field_value_list.push(
      parse2(quote! {
        root_element
      })
      .unwrap(),
    );
  } else if part.name == "CoreFilePropertiesPart" || part.name == "XmlSignaturePart" {
    field_declaration_list.push(
      parse2(quote! {
        use std::io::Read;
      })
      .unwrap(),
    );

    field_declaration_list.push(
      parse2(quote! {
        let mut content = String::new();
      })
      .unwrap(),
    );

    field_init_list.push(
      parse2(quote! {
        let mut file = std::io::BufReader::new(archive.by_name(path)?);
      })
      .unwrap(),
    );

    field_init_list.push(
      parse2(quote! {
        file.read_to_string(&mut content)?;
      })
      .unwrap(),
    );

    self_field_value_list.push(
      parse2(quote! {
        content
      })
      .unwrap(),
    );
  } else if let Some(root_element_type) = context.part_name_type_map.get(part.name.as_str()) {
    if part.name == "CustomXmlPropertiesPart" || part.name == "ThemePart" {
      let part_path_str = if part.paths.general.is_empty() {
        &part.target
      } else {
        &format!("{}/{}", part.paths.general, part.target)
      };

      field_init_list.push(
        parse2(quote! {
          let prefix = path.replace(
            &crate::common::resolve_zip_file_path(&format!("{}{}", parent_path, #part_path_str)),
            "",
          ).replace(".xml", "");
        })
        .unwrap(),
      );

      self_field_value_list.push(
        parse2(quote! {
          prefix
        })
        .unwrap(),
      );
    }

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

    field_init_list.push(parse2(quote! {
      let root_element = Some(std::boxed::Box::new(#field_type::from_reader(std::io::BufReader::new(archive.by_name(path)?))?));
    }).unwrap());

    field_unwrap_list.push(
      parse2(quote! {
        let root_element = root_element
          .ok_or_else(|| crate::common::SdkError::CommonError("root_element".to_string()))?;
      })
      .unwrap(),
    );

    self_field_value_list.push(
      parse2(quote! {
        root_element
      })
      .unwrap(),
    );
  }

  let path_str = if part.paths.general.is_empty() {
    ""
  } else {
    &format!("{}/", part.paths.general)
  };

  if !part.children.is_empty() {
    field_init_list.push(
      parse2(quote! {
        let child_parent_path = format!("{}{}", parent_path, #path_str);
      })
      .unwrap(),
    );
  }

  for child in &part.children {
    if child.is_data_part_reference {
      continue;
    }

    let child_part = context
      .part_name_part_map
      .get(child.name.as_str())
      .ok_or(format!("{:?}", child.name))
      .unwrap();

    let child_api_name_str = child.api_name.to_snake_case();

    let child_api_name_ident: Ident = parse_str(&child_api_name_str).unwrap();

    let child_name_str = child.name.to_snake_case();

    let child_name_ident: Ident = parse_str(&child_name_str).unwrap();

    let child_xml = format!("{}/{}.xml", child_part.paths.general, child_part.target);

    let child_path = format!("{}/{}", child_part.paths.general, child_part.target);

    let child_api_name_file_path_ident: Ident =
      parse_str(&format!("{}_file_path", child_api_name_str)).unwrap();

    let child_type: Type = parse_str(&format!(
      "crate::parts::{}::{}",
      child.name.to_snake_case(),
      child.name.to_upper_camel_case(),
    ))
    .unwrap();

    if child_part.name == "CustomXmlPart" {
      field_init_list.push(
        parse2(quote! {
          let #child_api_name_file_path_ident = crate::common::resolve_zip_file_path(
            &format!("{}{}", child_parent_path, #child_path),
          );
        })
        .unwrap(),
      );

      field_init_list.push(
        parse2(quote! {
          let item_props_path = crate::common::resolve_zip_file_path(
            &format!("{}../customXml/itemProps", child_parent_path),
          );
        })
        .unwrap(),
      );

      field_declaration_list.push(
        parse2(quote! {
          let mut #child_api_name_ident: Vec<#child_type> = vec![];
        })
        .unwrap(),
      );

      field_init_list.push(
        parse2(quote! {
          for file_path in file_path_set.iter() {
            if file_path.starts_with(&#child_api_name_file_path_ident) && !file_path.starts_with(&item_props_path) {
              let #child_name_ident = #child_type::new_from_archive(
                &child_parent_path,
                file_path,
                file_path_set,
                archive,
              )?;

              #child_api_name_ident.push(#child_name_ident);
            }
          }
        })
        .unwrap(),
      );
    } else if child_part.name == "CustomXmlPropertiesPart" {
      field_init_list.push(
        parse2(quote! {
          let #child_api_name_file_path_ident = crate::common::resolve_zip_file_path(
            &format!("{}{}{}.xml", child_parent_path, #child_path, prefix),
          );
        })
        .unwrap(),
      );

      field_init_list.push(
        parse2(quote! {
          let #child_api_name_ident = if let Some(file_path) = file_path_set.get(&#child_api_name_file_path_ident) {
            Some(std::boxed::Box::new(#child_type::new_from_archive(
              &child_parent_path,
              file_path,
              file_path_set,
              archive,
            )?))
          } else {
            None
          };
        })
        .unwrap(),
      );
    } else if child_part.name == "ThemePart" {
      field_declaration_list.push(
        parse2(quote! {
          let mut #child_api_name_ident: Option<std::boxed::Box<#child_type>> = None;
        })
        .unwrap(),
      );

      field_init_list.push(
        parse2(quote! {
          let #child_api_name_file_path_ident = crate::common::resolve_zip_file_path(
            &format!("{}{}", child_parent_path, #child_path),
          );
        })
        .unwrap(),
      );

      field_init_list.push(
        parse2(quote! {
          for file_path in file_path_set.iter() {
            if file_path.starts_with(&#child_api_name_file_path_ident) {
              #child_api_name_ident = Some(std::boxed::Box::new(#child_type::new_from_archive(
                &child_parent_path,
                file_path,
                file_path_set,
                archive,
              )?));

              break;
            }
          }
        })
        .unwrap(),
      );
    } else if !child_part.extension.is_empty()
      || child_part.name == "CustomDataPart"
      || child_part.name == "InternationalMacroSheetPart"
    {
      field_init_list.push(
        parse2(quote! {
          let #child_api_name_file_path_ident = crate::common::resolve_zip_file_path(
            &format!("{}{}", child_parent_path, #child_path),
          );
        })
        .unwrap(),
      );

      if child.max_occurs_great_than_one {
        field_declaration_list.push(
          parse2(quote! {
            let mut #child_api_name_ident: Vec<#child_type> = vec![];
          })
          .unwrap(),
        );

        field_init_list.push(
          parse2(quote! {
            for file_path in file_path_set.iter() {
              if file_path.starts_with(&#child_api_name_file_path_ident) {
                let #child_name_ident = #child_type::new_from_archive(
                  &child_parent_path,
                  file_path,
                  file_path_set,
                  archive,
                )?;

                #child_api_name_ident.push(#child_name_ident);
              }
            }
          })
          .unwrap(),
        );
      } else if child.min_occurs_is_non_zero {
        field_init_list.push(
          parse2(quote! {
            let #child_api_name_ident = if let Some(file_path) = file_path_set.get(&#child_api_name_file_path_ident) {
              Some(std::boxed::Box::new(#child_type::new_from_archive(
                &child_parent_path,
                file_path,
                file_path_set,
                archive,
              )?))
            } else {
              None
            };
          })
          .unwrap(),
        );

        field_unwrap_list.push(
          parse2(quote! {
            let #child_api_name_ident = #child_api_name_ident
              .ok_or_else(|| crate::common::SdkError::CommonError(#child_api_name_str.to_string()))?;
          })
          .unwrap(),
        );
      } else {
        field_init_list.push(
          parse2(quote! {
            let #child_api_name_ident = if let Some(file_path) = file_path_set.get(&#child_api_name_file_path_ident) {
              Some(std::boxed::Box::new(#child_type::new_from_archive(
                &child_parent_path,
                file_path,
                file_path_set,
                archive,
              )?))
            } else {
              None
            };
          })
          .unwrap(),
        );
      }
    } else if child_part.name == "CoreFilePropertiesPart" {
      field_init_list.push(
        parse2(quote! {
          let #child_api_name_file_path_ident = crate::common::resolve_zip_file_path(
            &format!("{}{}", child_parent_path, #child_xml),
          );
        })
        .unwrap(),
      );

      field_init_list.push(
        parse2(quote! {
          let #child_api_name_ident = if let Some(file_path) = file_path_set.get(&#child_api_name_file_path_ident) {
            Some(std::boxed::Box::new(#child_type::new_from_archive(
              &child_parent_path,
              file_path,
              file_path_set,
              archive,
            )?))
          } else {
            None
          };
        })
        .unwrap(),
      );
    } else if child_part.name == "XmlSignaturePart" {
      field_init_list.push(
        parse2(quote! {
          let #child_api_name_file_path_ident = crate::common::resolve_zip_file_path(
            &format!("{}{}", child_parent_path, #child_path),
          );
        })
        .unwrap(),
      );

      field_declaration_list.push(
        parse2(quote! {
          let mut #child_api_name_ident: Vec<#child_type> = vec![];
        })
        .unwrap(),
      );

      field_init_list.push(
        parse2(quote! {
          for file_path in file_path_set.iter() {
            if file_path.starts_with(&#child_api_name_file_path_ident) {
              let #child_name_ident = #child_type::new_from_archive(
                &child_parent_path,
                file_path,
                file_path_set,
                archive,
              )?;

              #child_api_name_ident.push(#child_name_ident);
            }
          }
        })
        .unwrap(),
      );
    } else if context
      .part_name_type_map
      .contains_key(child_part.name.as_str())
      || context
        .target_type_map
        .contains_key(&child_part.root_element)
      || context.target_type_map.contains_key(&child_part.target)
    {
      field_init_list.push(
        parse2(quote! {
          let #child_api_name_file_path_ident = crate::common::resolve_zip_file_path(
            &format!("{}{}", child_parent_path, #child_xml),
          );
        })
        .unwrap(),
      );

      if child.max_occurs_great_than_one {
        field_declaration_list.push(
          parse2(quote! {
            let mut #child_api_name_ident: Vec<#child_type> = vec![];
          })
          .unwrap(),
        );

        field_init_list.push(
          parse2(quote! {
            for file_path in file_path_set.iter() {
              if file_path.starts_with(&#child_api_name_file_path_ident) {
                let #child_name_ident = #child_type::new_from_archive(
                  &child_parent_path,
                  file_path,
                  file_path_set,
                  archive,
                )?;

                #child_api_name_ident.push(#child_name_ident);
              }
            }
          })
          .unwrap(),
        );
      } else if child.min_occurs_is_non_zero {
        field_init_list.push(
        parse2(quote! {
          let #child_api_name_ident = if let Some(file_path) = file_path_set.get(&#child_api_name_file_path_ident) {
            Some(std::boxed::Box::new(#child_type::new_from_archive(
              &child_parent_path,
              file_path,
              file_path_set,
              archive,
            )?))
          } else {
            None
          };
        })
        .unwrap(),
      );

        field_unwrap_list.push(
        parse2(quote! {
          let #child_api_name_ident = #child_api_name_ident
            .ok_or_else(|| crate::common::SdkError::CommonError(#child_api_name_str.to_string()))?;
        })
        .unwrap(),
      );
      } else {
        field_init_list.push(
        parse2(quote! {
          let #child_api_name_ident = if let Some(file_path) = file_path_set.get(&#child_api_name_file_path_ident) {
            Some(std::boxed::Box::new(#child_type::new_from_archive(
              &child_parent_path,
              file_path,
              file_path_set,
              archive,
            )?))
          } else {
            None
          };
        })
        .unwrap(),
      );
      }
    } else if child.max_occurs_great_than_one {
      field_declaration_list.push(
        parse2(quote! {
          let mut #child_api_name_ident: Vec<#child_type> = vec![];
        })
        .unwrap(),
      );
    } else {
      field_declaration_list.push(
        parse2(quote! {
          let mut #child_api_name_ident: Option<std::boxed::Box<#child_type>> = None;
        })
        .unwrap(),
      );
    }

    self_field_value_list.push(
      parse2(quote! {
        #child_api_name_ident
      })
      .unwrap(),
    );
  }

  if part.base == "OpenXmlPackage" {
    field_declaration_list.push(
      parse2(quote! {
        let content_types = {
          use std::io::Read;

          let mut content = String::new();

          let mut file = std::io::BufReader::new(archive.by_name("[Content_Types].xml")?);

          file.read_to_string(&mut content)?;

          content
        };
      })
      .unwrap(),
    );

    self_field_value_list.push(
      parse2(quote! {
        content_types
      })
      .unwrap(),
    );
  }

  let part_new_from_archive_fn: ItemFn = parse2(quote! {
    #[allow(unused_variables)]
    pub(crate) fn new_from_archive(
      parent_path: &str,
      path: &str,
      file_path_set: &std::collections::HashSet<String>,
      archive: &mut zip::ZipArchive<std::io::BufReader<std::fs::File>>,
    ) -> Result<Self, crate::common::SdkError> {
      #( #field_declaration_list )*

      #( #field_init_list )*

      #( #field_unwrap_list )*

      Ok(Self {
        #( #self_field_value_list, )*
      })
    }
  })
  .unwrap();

  let mut writer_stmt_list: Vec<Stmt> = vec![];

  let mut children_writer_stmt_list: Vec<Stmt> = vec![];

  let part_xml = format!("{}/{}.xml", part.paths.general, part.target);

  let part_path = format!("{}/{}", part.paths.general, part.target);

  let part_name_file_path_ident: Ident =
    parse_str(&format!("{}_file_path", part.name).to_snake_case()).unwrap();

  if part.name == "CustomXmlPart" {
    writer_stmt_list.push(
      parse2(quote! {
        use std::io::Write;
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        let #part_name_file_path_ident = crate::common::resolve_zip_file_path(
          &format!("{}{}{}.xml", parent_path, #part_path, self.prefix),
        );
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        let options = zip::write::SimpleFileOptions::default()
          .compression_method(zip::CompressionMethod::Stored)
          .unix_permissions(0o755);
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        zip.start_file(&#part_name_file_path_ident, options)?;
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        zip.write_all(self.content.as_bytes())?;
      })
      .unwrap(),
    );
  } else if part.name == "CustomXmlPropertiesPart" || part.name == "ThemePart" {
    writer_stmt_list.push(
      parse2(quote! {
        use std::io::Write;
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        let #part_name_file_path_ident = crate::common::resolve_zip_file_path(
          &format!("{}{}{}.xml", parent_path, #part_path, self.prefix),
        );
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        let options = zip::write::SimpleFileOptions::default()
          .compression_method(zip::CompressionMethod::Stored)
          .unix_permissions(0o755);
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        zip.start_file(&#part_name_file_path_ident, options)?;
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        zip.write_all(self.root_element.to_string()?.as_bytes())?;
      })
      .unwrap(),
    );
  } else if !part.extension.is_empty()
    || part.name == "CustomDataPart"
    || part.name == "InternationalMacroSheetPart"
  {
    writer_stmt_list.push(
      parse2(quote! {
        use std::io::{Read, Write};
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        let mut buffer = Vec::new();
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        let mut file = std::fs::File::open(&self.path)?;
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        file.read_to_end(&mut buffer)?;
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        let options = zip::write::SimpleFileOptions::default()
          .compression_method(zip::CompressionMethod::Stored)
          .unix_permissions(0o755);
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        zip.start_file(&self.inner_path, options)?;
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        zip.write_all(&buffer)?;
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        buffer.clear();
      })
      .unwrap(),
    );
  } else if part.name == "CoreFilePropertiesPart" || part.name == "XmlSignaturePart" {
    writer_stmt_list.push(
      parse2(quote! {
        use std::io::Write;
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        let #part_name_file_path_ident = crate::common::resolve_zip_file_path(
          &format!("{}{}", parent_path, #part_xml),
        );
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        let options = zip::write::SimpleFileOptions::default()
          .compression_method(zip::CompressionMethod::Stored)
          .unix_permissions(0o755);
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        zip.start_file(&#part_name_file_path_ident, options)?;
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        zip.write_all(self.content.as_bytes())?;
      })
      .unwrap(),
    );
  } else if context.part_name_type_map.contains_key(part.name.as_str())
    || context.target_type_map.contains_key(&part.root_element)
    || context.target_type_map.contains_key(&part.target)
  {
    writer_stmt_list.push(
      parse2(quote! {
        use std::io::Write;
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        let #part_name_file_path_ident = crate::common::resolve_zip_file_path(
          &format!("{}{}", parent_path, #part_xml),
        );
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        let options = zip::write::SimpleFileOptions::default()
          .compression_method(zip::CompressionMethod::Stored)
          .unix_permissions(0o755);
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        zip.start_file(&#part_name_file_path_ident, options)?;
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        zip.write_all(self.root_element.to_string()?.as_bytes())?;
      })
      .unwrap(),
    );
  }

  if !part.children.is_empty() {
    writer_stmt_list.push(
      parse2(quote! {
        let child_parent_path = format!("{}{}", parent_path, #path_str);
      })
      .unwrap(),
    );
  }

  for child in &part.children {
    if child.is_data_part_reference {
      continue;
    }

    let child_api_name_ident: Ident = parse_str(&child.api_name.to_snake_case()).unwrap();

    let child_name_ident: Ident = parse_str(&child.name.to_snake_case()).unwrap();

    if child.max_occurs_great_than_one {
      children_writer_stmt_list.push(
        parse2(quote! {
          for #child_name_ident in &self.#child_api_name_ident {
            #child_name_ident.save_zip(&child_parent_path, zip)?;
          }
        })
        .unwrap(),
      );
    } else if child.min_occurs_is_non_zero {
      children_writer_stmt_list.push(
        parse2(quote! {
          self.#child_api_name_ident.save_zip(&child_parent_path, zip)?;
        })
        .unwrap(),
      );
    } else {
      children_writer_stmt_list.push(
        parse2(quote! {
          if let Some(#child_api_name_ident) = &self.#child_api_name_ident {
            #child_api_name_ident.save_zip(&child_parent_path, zip)?;
          }
        })
        .unwrap(),
      );
    }
  }

  let part_save_zip_fn: ItemFn = parse2(quote! {
    #[allow(unused_variables)]
    pub(crate) fn save_zip(&self, parent_path: &str, zip: &mut zip::ZipWriter<std::fs::File>) -> Result<(), crate::common::SdkError> {
      #( #writer_stmt_list )*

      #( #children_writer_stmt_list )*

      Ok(())
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

        Self::new_from_archive("", "", &file_path_set, &mut archive)
      }
    })
    .unwrap();

    let part_save_fn: ItemFn = parse2(quote! {
      pub fn save(&self, path: &str) -> Result<(), crate::common::SdkError> {
        use std::io::Write;

        let path = std::path::Path::new(path);

        let file = std::fs::File::create(path)?;

        let mut zip = zip::ZipWriter::new(file);

        let options = zip::write::SimpleFileOptions::default()
          .compression_method(zip::CompressionMethod::Stored)
          .unix_permissions(0o755);

        zip.start_file("[Content_Types].xml", options)?;

        zip.write_all(self.content_types.as_bytes())?;

        self.save_zip("", &mut zip)?;

        zip.finish()?;

        Ok(())
      }
    })
    .unwrap();

    let part_impl: ItemImpl = parse2(quote! {
      impl #struct_name_ident {
        #part_new_fn

        #part_new_from_archive_fn

        #part_save_fn

        #part_save_zip_fn
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

        #part_save_zip_fn
      }
    })
    .unwrap();

    quote! {
      #part_struct

      #part_impl
    }
  }
}
