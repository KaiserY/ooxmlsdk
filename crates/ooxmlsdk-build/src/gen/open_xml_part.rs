use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, parse_str, FieldValue, Ident, ItemFn, ItemImpl, ItemStruct, Stmt, Type};

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
  } else if !part.extension.is_empty()
    || part.name == "CustomDataPart"
    || part.name == "CustomXmlPart"
    || part.name == "InternationalMacroSheetPart"
  {
    fields.push(quote! {
      pub path: String,
    });
  } else if part.name == "CoreFilePropertiesPart"
    || part.name == "XmlSignaturePart"
    || part.name == "MailMergeRecipientDataPart"
  {
    fields.push(quote! {
      pub content: String,
    });
  } else if part.base == "StylesPart" {
    fields.push(quote! {
      pub root_element: std::boxed::Box<crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles>,
    });
  } else if part.base == "CustomUIPart" {
    fields.push(quote! {
      pub root_element: std::boxed::Box<crate::schemas::schemas_microsoft_com_office_2006_01_customui::CustomUi>,
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
  } else if !part.extension.is_empty()
    || part.name == "CustomDataPart"
    || part.name == "CustomXmlPart"
    || part.name == "InternationalMacroSheetPart"
  {
    self_field_value_list.push(
      parse2(quote! {
        path: path.to_string()
      })
      .unwrap(),
    );
  } else if part.name == "CoreFilePropertiesPart"
    || part.name == "XmlSignaturePart"
    || part.name == "MailMergeRecipientDataPart"
  {
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
  } else if part.base == "StylesPart" {
    field_init_list.push(parse2(quote! {
      let root_element = Some(std::boxed::Box::new(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles::from_reader(std::io::BufReader::new(archive.by_name(path)?))?,
      ));
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
  } else if part.base == "CustomUIPart" {
    field_init_list.push(parse2(quote! {
      let root_element = Some(std::boxed::Box::new(
        crate::schemas::schemas_microsoft_com_office_2006_01_customui::CustomUi::from_reader(std::io::BufReader::new(archive.by_name(path)?))?,
      ));
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

    if context
      .part_name_type_map
      .contains_key(child_part.name.as_str())
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
    } else if !child_part.extension.is_empty()
      || child_part.name == "CustomDataPart"
      || child_part.name == "CustomXmlPart"
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
    } else if child_part.name == "CoreFilePropertiesPart"
      || child_part.name == "MailMergeRecipientDataPart"
    {
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
    } else if child_part.base == "StylesPart" || child_part.base == "CustomUIPart" {
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
