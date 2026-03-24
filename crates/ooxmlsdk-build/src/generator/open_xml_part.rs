use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Arm, FieldValue, Ident, ItemFn, ItemImpl, ItemStruct, Stmt, Type, parse_str, parse2};

use crate::GenContext;
use crate::models::OpenXmlPart;
use crate::utils::get_or_panic;

pub fn gen_open_xml_parts(part: &OpenXmlPart, gen_context: &GenContext) -> TokenStream {
  let relationship_type_str = &part.relationship_type;

  let relationship_type_stmt: Stmt = parse2(quote! {
    pub const RELATIONSHIP_TYPE: &str = #relationship_type_str;
  })
  .unwrap();

  let struct_name_ident: Ident = parse_str(&part.name.to_upper_camel_case()).unwrap();

  let mut fields: Vec<TokenStream> = vec![];

  if part.base == "OpenXmlPackage" {
    fields.push(quote! {
      pub content_types: crate::schemas::opc_content_types::Types,
    });
  } else {
    fields.push(quote! {
      pub r_id: String,
    });
  }

  if !part.children.is_empty() {
    fields.push(quote! {
      pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
    });

    fields.push(quote! {
      pub rels_path: String,
    });
  }

  fields.push(quote! {
    pub inner_path: String,
  });

  if part.name == "CustomXmlPart" || part.name == "XmlSignaturePart" {
    fields.push(quote! {
      pub part_content: String,
    });
  } else if !part.extension.is_empty()
    || part.name == "CustomDataPart"
    || part.name == "InternationalMacroSheetPart"
  {
    fields.push(quote! {
      pub part_content: Vec<u8>,
    });
  } else if part.name == "CoreFilePropertiesPart" {
    fields.push(quote! {
      pub root_element: crate::schemas::opc_core_properties::CoreProperties,
    });
  } else if let Some(root_element_type_name) =
    gen_context.part_name_type_name_map.get(part.name.as_str())
  {
    let root_element_type = get_or_panic!(gen_context.type_name_type_map, root_element_type_name);

    let field_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      root_element_type.module_name,
      root_element_type.class_name.to_upper_camel_case()
    ))
    .unwrap();

    fields.push(quote! {
      pub root_element: #field_type,
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
    #[derive(Clone, Debug, Default)]
    pub struct #struct_name_ident {
      #( #fields )*
    }
  })
  .unwrap();

  let mut field_declaration_list: Vec<Stmt> = vec![];
  let mut field_unwrap_list: Vec<Stmt> = vec![];
  let mut self_field_value_list: Vec<FieldValue> = vec![];
  let mut children_stmt: Option<Stmt> = None;
  let mut children_arm_list: Vec<Arm> = vec![];

  let path_str = if part.paths.general.is_empty() {
    ""
  } else {
    &format!("{}/", part.paths.general)
  };

  let part_rels_path_ident: Ident =
    parse_str(&format!("{}_rels_path", part.name.to_snake_case())).unwrap();

  if part.base == "OpenXmlPackage" {
    field_declaration_list.push(
      parse2(quote! {
        let content_types = crate::schemas::opc_content_types::Types::from_reader(
          std::io::BufReader::new(archive.by_name("[Content_Types].xml")?,
        ))?;
      })
      .unwrap(),
    );

    self_field_value_list.push(
      parse2(quote! {
        content_types
      })
      .unwrap(),
    );
  } else {
    self_field_value_list.push(
      parse2(quote! {
        r_id: r_id.to_string()
      })
      .unwrap(),
    );
  }

  if !part.children.is_empty() {
    field_declaration_list.push(
      parse2(quote! {
        let mut rels_path = "".to_string();
      })
      .unwrap(),
    );

    field_declaration_list.push(
      parse2(quote! {
        let child_parent_path = format!("{}{}", parent_path, #path_str);
      })
      .unwrap(),
    );

    field_declaration_list.push(
      parse2(quote! {
        let part_target_str = if path.ends_with(".xml") {
          &path[path
            .rfind('/')
            .ok_or_else(|| crate::common::SdkError::CommonError(path.to_string()))?
            + 1..path.len()]
        } else {
          ""
        };
      })
      .unwrap(),
    );

    field_declaration_list.push(
      parse2(quote! {
        let #part_rels_path_ident = crate::common::resolve_zip_file_path(
          &format!("{child_parent_path}_rels/{part_target_str}.rels"),
        );
      })
      .unwrap(),
    );

    field_declaration_list.push(
      parse2(quote! {
        let relationships = if let Some(file_path) = file_path_set.get(&#part_rels_path_ident) {
          rels_path = file_path.to_string();

          Some(crate::schemas::opc_relationships::Relationships::from_reader(
              std::io::BufReader::new(archive.by_name(file_path)?,
            ))?,
          )
        } else {
          None
        };
      })
      .unwrap(),
    );

    self_field_value_list.push(
      parse2(quote! {
        rels_path
      })
      .unwrap(),
    );

    self_field_value_list.push(
      parse2(quote! {
        relationships
      })
      .unwrap(),
    );
  }

  self_field_value_list.push(
    parse2(quote! {
      inner_path: path.to_string()
    })
    .unwrap(),
  );

  if part.name == "CustomXmlPart" || part.name == "XmlSignaturePart" {
    field_declaration_list.push(
      parse2(quote! {
        use std::io::Read;
      })
      .unwrap(),
    );

    field_declaration_list.push(
      parse2(quote! {
        let mut part_content = String::new();
      })
      .unwrap(),
    );

    field_declaration_list.push(
      parse2(quote! {
        {
          let mut file = std::io::BufReader::new(archive.by_name(path)?);

          file.read_to_string(&mut part_content)?;
        }
      })
      .unwrap(),
    );

    self_field_value_list.push(
      parse2(quote! {
        part_content
      })
      .unwrap(),
    );
  } else if !part.extension.is_empty()
    || part.name == "CustomDataPart"
    || part.name == "InternationalMacroSheetPart"
  {
    field_declaration_list.push(
      parse2(quote! {
        use std::io::Read;
      })
      .unwrap(),
    );

    field_declaration_list.push(
      parse2(quote! {
        let mut part_content;
      })
      .unwrap(),
    );

    field_declaration_list.push(
      parse2(quote! {
        {
          let mut zip_entry = archive.by_name(path)?;

          part_content = Vec::with_capacity(zip_entry.size() as usize);

          zip_entry.read_to_end(&mut part_content)?;
        }
      })
      .unwrap(),
    );

    self_field_value_list.push(
      parse2(quote! {
        part_content
      })
      .unwrap(),
    );
  } else if part.name == "CoreFilePropertiesPart" {
    field_declaration_list.push(parse2(quote! {
      let root_element = Some(
        crate::schemas::opc_core_properties::CoreProperties::from_reader(std::io::BufReader::new(archive.by_name(path)?))?,
      );
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
  } else if let Some(root_element_type_name) =
    gen_context.part_name_type_name_map.get(part.name.as_str())
  {
    let root_element_type = get_or_panic!(gen_context.type_name_type_map, root_element_type_name);

    let field_type: Type = parse_str(&format!(
      "crate::schemas::{}::{}",
      root_element_type.module_name,
      root_element_type.class_name.to_upper_camel_case()
    ))
    .unwrap();

    field_declaration_list.push(parse2(quote! {
      let root_element = Some(#field_type::from_reader(std::io::BufReader::new(archive.by_name(path)?))?);
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

  for child in &part.children {
    if child.is_data_part_reference {
      continue;
    }

    let child_type: Type = parse_str(&format!(
      "crate::parts::{}::{}",
      child.name.to_snake_case(),
      child.name.to_upper_camel_case(),
    ))
    .unwrap();

    let child_api_name_str = child.api_name.to_snake_case();

    let child_api_name_ident: Ident = parse_str(&child_api_name_str).unwrap();

    let child_name_str = child.name.to_snake_case();

    let child_name_ident: Ident = parse_str(&child_name_str).unwrap();

    let relationship_type_ty: Type = parse_str(&format!(
      "crate::parts::{}::RELATIONSHIP_TYPE",
      child.name.to_snake_case(),
    ))
    .unwrap();

    if child.max_occurs_great_than_one {
      field_declaration_list.push(
        parse2(quote! {
          let mut #child_api_name_ident: Vec<#child_type> = vec![];
        })
        .unwrap(),
      );

      children_arm_list.push(
        parse2(quote! {
          #relationship_type_ty => {
            let target_path = crate::common::resolve_zip_file_path(
              &format!("{}{}", child_parent_path, relationship.target),
            );

            let #child_name_ident = #child_type::new_from_archive(
              &child_parent_path,
              &target_path,
              &relationship.id,
              file_path_set,
              archive,
            )?;

            #child_api_name_ident.push(#child_name_ident);
          }
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

      children_arm_list.push(
        parse2(quote! {
          #relationship_type_ty => {
            let target_path = crate::common::resolve_zip_file_path(
              &format!("{}{}", child_parent_path, relationship.target),
            );

            #child_api_name_ident = Some(std::boxed::Box::new(#child_type::new_from_archive(
              &child_parent_path,
              &target_path,
              &relationship.id,
              file_path_set,
              archive,
            )?));
          }
        })
        .unwrap(),
      );

      if child.min_occurs_is_non_zero {
        field_unwrap_list.push(
          parse2(quote! {
            let #child_api_name_ident = #child_api_name_ident
                .ok_or_else(|| crate::common::SdkError::CommonError(#child_api_name_str.to_string()))?;
          })
          .unwrap(),
        );
      }
    }

    self_field_value_list.push(
      parse2(quote! {
        #child_api_name_ident
      })
      .unwrap(),
    );
  }

  if !part.children.is_empty() {
    children_arm_list.push(
      parse2(quote! {
        _ => ()
      })
      .unwrap(),
    );

    children_stmt = Some(
      parse2(quote! {
        if let Some(relationships) = &relationships {
          for relationship in &relationships.relationship {
            #[allow(clippy::single_match)]
            match relationship.r#type.as_str() {
              #( #children_arm_list, )*
            }
          }
        }
      })
      .unwrap(),
    );
  }

  let part_new_from_archive_fn: ItemFn = parse2(quote! {
    #[allow(unused_variables)]
    pub(crate) fn new_from_archive<R: std::io::Read + std::io::Seek>(
      parent_path: &str,
      path: &str,
      r_id: &str,
      file_path_set: &std::collections::HashSet<String>,
      archive: &mut zip::ZipArchive<R>,
    ) -> Result<Self, crate::common::SdkError> {
      #( #field_declaration_list )*

      #children_stmt

      #( #field_unwrap_list )*

      Ok(Self {
        #( #self_field_value_list, )*
      })
    }
  })
  .unwrap();

  let mut writer_stmt_list: Vec<Stmt> = vec![];

  let mut children_writer_stmt_list: Vec<Stmt> = vec![];

  let part_paths_general = &part.paths.general;

  let part_name_dir_path_ident: Ident =
    parse_str(&format!("{}_dir_path", part.name).to_snake_case()).unwrap();

  writer_stmt_list.push(
    parse2(quote! {
      let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);
    })
    .unwrap(),
  );

  writer_stmt_list.push(
    parse2(quote! {
      let directory_path = crate::common::resolve_zip_file_path(parent_path);
    })
    .unwrap(),
  );

  writer_stmt_list.push(
    parse2(quote! {
      if !directory_path.is_empty() && !entry_set.contains(&directory_path) {
        zip.add_directory(&directory_path, options)?;

        entry_set.insert(directory_path);
      }
    })
    .unwrap(),
  );

  writer_stmt_list.push(
    parse2(quote! {
      let #part_name_dir_path_ident = crate::common::resolve_zip_file_path(
        &format!("{}{}/", parent_path, #part_paths_general),
      );
    })
    .unwrap(),
  );

  writer_stmt_list.push(
    parse2(quote! {
      if !#part_name_dir_path_ident.is_empty() && !entry_set.contains(&#part_name_dir_path_ident) {
        zip.add_directory(&#part_name_dir_path_ident, options)?;

        entry_set.insert(#part_name_dir_path_ident);
      }
    })
    .unwrap(),
  );

  if part.name == "CustomXmlPart" || part.name == "XmlSignaturePart" {
    writer_stmt_list.push(
      parse2(quote! {
        use std::io::Write;
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        if !entry_set.contains(&self.inner_path) {
          zip.start_file(&self.inner_path, options)?;

          zip.write_all(self.part_content.as_bytes())?;

          entry_set.insert(self.inner_path.to_string());
        }
      })
      .unwrap(),
    );
  } else if !part.extension.is_empty()
    || part.name == "CustomDataPart"
    || part.name == "InternationalMacroSheetPart"
  {
    writer_stmt_list.push(
      parse2(quote! {
        use std::io::Write;
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        if !entry_set.contains(&self.inner_path) {
          zip.start_file(&self.inner_path, options)?;

          zip.write_all(&self.part_content)?;

          entry_set.insert(self.inner_path.to_string());
        }
      })
      .unwrap(),
    );
  } else if part.name == "CoreFilePropertiesPart"
    || gen_context
      .part_name_type_name_map
      .contains_key(part.name.as_str())
  {
    writer_stmt_list.push(
      parse2(quote! {
        use std::io::Write;
      })
      .unwrap(),
    );

    writer_stmt_list.push(
      parse2(quote! {
        if !entry_set.contains(&self.inner_path) {
          zip.start_file(&self.inner_path, options)?;

          zip.write_all(self.root_element.to_xml()?.as_bytes())?;

          entry_set.insert(self.inner_path.to_string());
        }
      })
      .unwrap(),
    );
  } else if !part.children.is_empty() {
    writer_stmt_list.push(
      parse2(quote! {
        use std::io::Write;
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

    writer_stmt_list.push(
      parse2(quote! {
        if let Some(relationships) = &self.relationships {
          let rels_dir_path = crate::common::resolve_zip_file_path(
            &format!("{child_parent_path}_rels"),
          );

          if !rels_dir_path.is_empty() && !entry_set.contains(&rels_dir_path) {
            zip.add_directory(&rels_dir_path, options)?;

            entry_set.insert(rels_dir_path);
          }

          if !entry_set.contains(&self.rels_path) {
            zip.start_file(&self.rels_path, options)?;

            zip.write_all(relationships.to_xml()?.as_bytes())?;

            entry_set.insert(self.rels_path.to_string());
          }
        }
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
            #child_name_ident.save_zip(&child_parent_path, zip, entry_set)?;
          }
        })
        .unwrap(),
      );
    } else if child.min_occurs_is_non_zero {
      children_writer_stmt_list.push(
        parse2(quote! {
          self.#child_api_name_ident.save_zip(&child_parent_path, zip, entry_set)?;
        })
        .unwrap(),
      );
    } else {
      children_writer_stmt_list.push(
        parse2(quote! {
          if let Some(#child_api_name_ident) = &self.#child_api_name_ident {
            #child_api_name_ident.save_zip(&child_parent_path, zip, entry_set)?;
          }
        })
        .unwrap(),
      );
    }
  }

  let part_save_zip_fn: ItemFn = parse2(quote! {
    pub(crate) fn save_zip<W: std::io::Write + std::io::Seek>(
      &self,
      parent_path: &str,
      zip: &mut zip::ZipWriter<W>,
      entry_set: &mut std::collections::HashSet<String>,
    ) -> Result<(), crate::common::SdkError> {
      #( #writer_stmt_list )*

      #( #children_writer_stmt_list )*

      Ok(())
    }
  })
  .unwrap();

  if part.base == "OpenXmlPackage" {
    let part_new_fn: ItemFn = parse2(quote! {
      pub fn new<R: std::io::Read + std::io::Seek>(
        reader: R,
      ) -> Result<Self, crate::common::SdkError> {
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

        Self::new_from_archive("", "", "", &file_path_set, &mut archive)
      }
    })
    .unwrap();

    let part_new_from_file_fn: ItemFn = parse2(quote! {
      pub fn new_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, crate::common::SdkError> {
        Self::new(std::io::BufReader::new(std::fs::File::open(path)?))
      }
    })
    .unwrap();

    let part_save_fn: ItemFn = parse2(quote! {
      pub fn save<W: std::io::Write + std::io::Seek>(&self, writer: W) -> Result<(), crate::common::SdkError> {
        use std::io::Write;

        let mut entry_set: std::collections::HashSet<String> = std::collections::HashSet::new();

        let mut zip = zip::ZipWriter::new(writer);

        let options = zip::write::SimpleFileOptions::default()
          .compression_method(zip::CompressionMethod::Deflated)
          .unix_permissions(0o755);

        zip.start_file("[Content_Types].xml", options)?;

        zip.write_all(self.content_types.to_xml()?.as_bytes())?;

        self.save_zip("", &mut zip, &mut entry_set)?;

        zip.finish()?;

        Ok(())
      }
    })
    .unwrap();

    let part_save_to_file_fn: ItemFn = parse2(quote! {
      pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), crate::common::SdkError> {
        self.save(std::fs::File::create(path)?)
      }
    })
    .unwrap();

    let part_impl: ItemImpl = parse2(quote! {
      impl #struct_name_ident {
        #part_new_fn

        #part_new_from_file_fn

        #part_new_from_archive_fn

        #part_save_fn

        #part_save_to_file_fn

        #part_save_zip_fn
      }
    })
    .unwrap();

    quote! {
      #relationship_type_stmt

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
      #relationship_type_stmt

      #part_struct

      #part_impl
    }
  }
}
