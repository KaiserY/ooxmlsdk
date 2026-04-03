use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
  Arm, Expr, FieldValue, Ident, ItemFn, ItemImpl, ItemMod, ItemStruct, Stmt, Type, parse_str,
  parse2,
};

use crate::Result;
use crate::sdk_code::versioning::{features_cfg_attrs, versioned_tokens};
use crate::sdk_data::sdk_data_model::{Part, PartChild, PartContentKind};

struct RelationshipBranch {
  version: String,
  relationship_type: Expr,
  body: TokenStream,
}

pub fn gen_part_module(part: &Part) -> Result<TokenStream> {
  let relationship_type_str = part.relationship_type.as_str();
  let relationship_type_stmt: Stmt = parse2(quote! {
    pub const RELATIONSHIP_TYPE: &str = #relationship_type_str;
  })?;
  let parent_path_ident = if part.children.is_empty() {
    parse_str::<Ident>("_parent_path")?
  } else {
    parse_str::<Ident>("parent_path")?
  };
  let path_ident = if part.content_kind == PartContentKind::None && part.children.is_empty() {
    parse_str::<Ident>("_path")?
  } else {
    parse_str::<Ident>("path")?
  };
  let r_id_ident = if part.base == "OpenXmlPackage" {
    parse_str::<Ident>("_r_id")?
  } else {
    parse_str::<Ident>("r_id")?
  };
  let file_path_set_ident = if part.children.is_empty() {
    parse_str::<Ident>("_file_path_set")?
  } else {
    parse_str::<Ident>("file_path_set")?
  };
  let archive_ident = if part.content_kind == PartContentKind::None && part.children.is_empty() {
    parse_str::<Ident>("_archive")?
  } else {
    parse_str::<Ident>("archive")?
  };

  let struct_name_ident: Ident = parse_str(&part.name.to_upper_camel_case())?;
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

  match part.content_kind {
    PartContentKind::Xml => {
      let root_type = part_root_type_tokens(part)?;
      fields.push(quote! {
        pub root_element: #root_type,
      });
    }
    PartContentKind::Text => {
      fields.push(quote! {
        pub part_content: String,
      });
    }
    PartContentKind::Binary => {
      fields.push(quote! {
        pub part_content: Vec<u8>,
      });
    }
    PartContentKind::None => {}
  }

  for child in &part.children {
    if child.is_data_part_reference {
      continue;
    }

    fields.push(child_field_tokens(child)?);
  }

  let part_struct: ItemStruct = parse2(quote! {
    #[derive(Clone, Debug, Default)]
    pub struct #struct_name_ident {
      #( #fields )*
    }
  })?;

  let mut field_declaration_list: Vec<Stmt> = vec![];
  let mut field_unwrap_list: Vec<Stmt> = vec![];
  let mut self_field_value_list: Vec<FieldValue> = vec![];
  let mut children_stmt: Option<Stmt> = None;
  let mut children_arm_list: Vec<RelationshipBranch> = vec![];

  let path_str = if part.paths.general.is_empty() {
    "".to_string()
  } else {
    format!("{}/", part.paths.general)
  };
  let part_rels_path_ident: Ident = parse_str(&format!("{}_rels_path", part.name.to_snake_case()))?;

  if part.base == "OpenXmlPackage" {
    field_declaration_list.push(parse2(quote! {
      let content_types = crate::schemas::opc_content_types::Types::from_reader(
        std::io::BufReader::new(archive.by_name("[Content_Types].xml")?),
      )?;
    })?);
    self_field_value_list.push(parse2(quote! { content_types })?);
  } else {
    self_field_value_list.push(parse2(quote! {
      r_id: r_id.to_string()
    })?);
  }

  if !part.children.is_empty() {
    field_declaration_list.push(parse2(quote! {
      let mut rels_path = String::new();
    })?);
    field_declaration_list.push(parse2(quote! {
      let child_parent_path = format!("{}{}", parent_path, #path_str);
    })?);
    field_declaration_list.push(parse2(quote! {
      let part_target_str = if path.ends_with(".xml") {
        &#path_ident[#path_ident
          .rfind('/')
          .ok_or_else(|| crate::common::SdkError::CommonError(#path_ident.to_string()))?
          + 1..#path_ident.len()]
      } else {
        ""
      };
    })?);
    field_declaration_list.push(parse2(quote! {
      let #part_rels_path_ident = crate::common::resolve_zip_file_path(
        &format!("{child_parent_path}_rels/{part_target_str}.rels"),
      );
    })?);
    field_declaration_list.push(parse2(quote! {
      let relationships = if let Some(file_path) = file_path_set.get(&#part_rels_path_ident) {
        rels_path = file_path.to_string();
        Some(crate::schemas::opc_relationships::Relationships::from_reader(
          std::io::BufReader::new(archive.by_name(file_path)?),
        )?)
      } else {
        None
      };
    })?);
    self_field_value_list.push(parse2(quote! { rels_path })?);
    self_field_value_list.push(parse2(quote! { relationships })?);
  }

  self_field_value_list.push(parse2(quote! {
    inner_path: #path_ident.to_string()
  })?);

  match part.content_kind {
    PartContentKind::Text => {
      field_declaration_list.push(parse2(quote! {
        use std::io::Read;
      })?);
      field_declaration_list.push(parse2(quote! {
        let mut part_content = String::new();
      })?);
      field_declaration_list.push(parse2(quote! {
        {
          let mut file = std::io::BufReader::new(#archive_ident.by_name(#path_ident)?);
          file.read_to_string(&mut part_content)?;
        }
      })?);
      self_field_value_list.push(parse2(quote! { part_content })?);
    }
    PartContentKind::Binary => {
      field_declaration_list.push(parse2(quote! {
        use std::io::Read;
      })?);
      field_declaration_list.push(parse2(quote! {
        let mut part_content;
      })?);
      field_declaration_list.push(parse2(quote! {
        {
          let mut zip_entry = #archive_ident.by_name(#path_ident)?;
          part_content = Vec::with_capacity(zip_entry.size() as usize);
          zip_entry.read_to_end(&mut part_content)?;
        }
      })?);
      self_field_value_list.push(parse2(quote! { part_content })?);
    }
    PartContentKind::Xml => {
      let root_type = part_root_type_tokens(part)?;
      field_declaration_list.push(parse2(quote! {
        let root_element = Some(#root_type::from_reader(std::io::BufReader::new(
          #archive_ident.by_name(#path_ident)?,
        ))?);
      })?);
      field_unwrap_list.push(parse2(quote! {
        let root_element = root_element
          .ok_or_else(|| crate::common::SdkError::CommonError("root_element".to_string()))?;
      })?);
      self_field_value_list.push(parse2(quote! { root_element })?);
    }
    PartContentKind::None => {}
  }

  for child in &part.children {
    if child.is_data_part_reference {
      continue;
    }

    let child_type = child_type_tokens(child)?;
    let child_api_name_ident: Ident = parse_str(&child.api_name.to_snake_case())?;
    let child_name_ident: Ident = parse_str(&child.name.to_snake_case())?;
    let relationship_type_expr: Expr = parse_str(&format!(
      "crate::parts::{}::RELATIONSHIP_TYPE",
      child.name.to_snake_case(),
    ))?;

    if child.max_occurs_great_than_one {
      let repeated_child_load = if child.min_occurs_is_non_zero {
        quote! {
          let #child_name_ident = #child_type::new_from_archive(
            &child_parent_path,
            &target_path,
            &relationship.id,
            file_path_set,
            #archive_ident,
          )?;
          #child_api_name_ident.push(#child_name_ident);
        }
      } else {
        quote! {
          if file_path_set.contains(&target_path) {
            let #child_name_ident = #child_type::new_from_archive(
              &child_parent_path,
              &target_path,
              &relationship.id,
              file_path_set,
              #archive_ident,
            )?;
            #child_api_name_ident.push(#child_name_ident);
          }
        }
      };
      field_declaration_list.push(
        parse2(versioned_tokens(
          &child.version,
          quote! {
            let mut #child_api_name_ident: Vec<#child_type> = vec![];
          },
        ))
        .map_err(|err| {
          format!(
            "{} repeated child declaration {}: {err}",
            part.name, child.name
          )
        })?,
      );
      children_arm_list.push(RelationshipBranch {
        version: child.version.clone(),
        relationship_type: relationship_type_expr.clone(),
        body: versioned_tokens(
          &child.version,
          quote! {
            let target_path = crate::common::resolve_zip_file_path(
              &crate::common::resolve_relationship_target_path(
                &child_parent_path,
                &relationship.target,
              ),
            );
            #repeated_child_load
          },
        ),
      });
    } else {
      let optional_child_load = if child.min_occurs_is_non_zero {
        quote! {
          #child_api_name_ident = Some(std::boxed::Box::new(#child_type::new_from_archive(
            &child_parent_path,
            &target_path,
            &relationship.id,
            file_path_set,
            #archive_ident,
          )?));
        }
      } else {
        quote! {
          if file_path_set.contains(&target_path) {
            #child_api_name_ident = Some(std::boxed::Box::new(#child_type::new_from_archive(
              &child_parent_path,
              &target_path,
              &relationship.id,
              file_path_set,
              #archive_ident,
            )?));
          }
        }
      };
      field_declaration_list.push(
        parse2(versioned_tokens(
          &child.version,
          quote! {
            let mut #child_api_name_ident: Option<std::boxed::Box<#child_type>> = None;
          },
        ))
        .map_err(|err| {
          format!(
            "{} optional child declaration {}: {err}",
            part.name, child.name
          )
        })?,
      );
      children_arm_list.push(RelationshipBranch {
        version: child.version.clone(),
        relationship_type: relationship_type_expr.clone(),
        body: versioned_tokens(
          &child.version,
          quote! {
            let target_path = crate::common::resolve_zip_file_path(
              &crate::common::resolve_relationship_target_path(
                &child_parent_path,
                &relationship.target,
              ),
            );
            #optional_child_load
          },
        ),
      });

      if child.min_occurs_is_non_zero {
        let child_api_name_str = child.api_name.to_snake_case();
        field_unwrap_list.push(parse2(versioned_tokens(&child.version, quote! {
          let #child_api_name_ident = #child_api_name_ident
            .ok_or_else(|| crate::common::SdkError::CommonError(#child_api_name_str.to_string()))?;
        })).map_err(|err| format!("{} required child unwrap {}: {err}", part.name, child.name))?);
      }
    }

    self_field_value_list.push(
      parse2(versioned_tokens(
        &child.version,
        quote! {
          #child_api_name_ident
        },
      ))
      .map_err(|err| format!("{} self field value {}: {err}", part.name, child.name))?,
    );
  }

  if !part.children.is_empty() {
    let children_match_arms: Vec<Arm> = children_arm_list
      .iter()
      .map(|branch| {
        let relationship_type = &branch.relationship_type;
        let body = &branch.body;
        let arm_version = branch.version.as_str();

        parse2(versioned_tokens(
          arm_version,
          quote! {
            relationship_type if relationship_type == #relationship_type => {
              #body
            }
          },
        ))
      })
      .collect::<std::result::Result<Vec<_>, _>>()?;

    children_stmt = Some(
      parse2(quote! {
          if let Some(relationships) = &relationships {
            for relationship in &relationships.relationship {
              match crate::common::normalize_relationship_type(relationship.r#type.as_str()) {
              #( #children_match_arms )*
              _ => {}
            }
          }
        }
      })
      .map_err(|err| format!("{} children stmt: {err}", part.name))?,
    );
  }

  let part_new_from_archive_fn: ItemFn = parse2(quote! {
    pub(crate) fn new_from_archive<R: std::io::Read + std::io::Seek>(
      #parent_path_ident: &str,
      #path_ident: &str,
      #r_id_ident: &str,
      #file_path_set_ident: &std::collections::HashSet<String>,
      #archive_ident: &mut zip::ZipArchive<R>,
    ) -> Result<Self, crate::common::SdkError> {
      #( #field_declaration_list )*
      #children_stmt
      #( #field_unwrap_list )*
      Ok(Self {
        #( #self_field_value_list, )*
      })
    }
  })?;

  let mut writer_stmt_list: Vec<Stmt> = vec![];
  let mut children_writer_stmt_list: Vec<Stmt> = vec![];
  let part_name_dir_path_ident: Ident =
    parse_str(&format!("{}_dir_path", part.name.to_snake_case()))?;

  writer_stmt_list.push(parse2(quote! {
    let options = zip::write::SimpleFileOptions::default()
      .compression_method(zip::CompressionMethod::Deflated)
      .unix_permissions(0o755);
  })?);
  writer_stmt_list.push(parse2(quote! {
    let directory_path = crate::common::resolve_zip_file_path(parent_path);
  })?);
  writer_stmt_list.push(parse2(quote! {
    if !directory_path.is_empty() && !entry_set.contains(&directory_path) {
      zip.add_directory(&directory_path, options)?;
      entry_set.insert(directory_path);
    }
  })?);
  writer_stmt_list.push(parse2(quote! {
    let #part_name_dir_path_ident = self
      .inner_path
      .rsplit_once('/')
      .map(|(dir_path, _)| crate::common::resolve_zip_file_path(&format!("{dir_path}/")))
      .unwrap_or_default();
  })?);
  writer_stmt_list.push(parse2(quote! {
    if !#part_name_dir_path_ident.is_empty() && !entry_set.contains(&#part_name_dir_path_ident) {
      zip.add_directory(&#part_name_dir_path_ident, options)?;
      entry_set.insert(#part_name_dir_path_ident);
    }
  })?);

  match part.content_kind {
    PartContentKind::Text => {
      writer_stmt_list.push(parse2(quote! {
        use std::io::Write;
      })?);
      writer_stmt_list.push(parse2(quote! {
        if !entry_set.contains(&self.inner_path) {
          zip.start_file(&self.inner_path, options)?;
          zip.write_all(self.part_content.as_bytes())?;
          entry_set.insert(self.inner_path.to_string());
        }
      })?);
    }
    PartContentKind::Binary => {
      writer_stmt_list.push(parse2(quote! {
        use std::io::Write;
      })?);
      writer_stmt_list.push(parse2(quote! {
        if !entry_set.contains(&self.inner_path) {
          zip.start_file(&self.inner_path, options)?;
          zip.write_all(&self.part_content)?;
          entry_set.insert(self.inner_path.to_string());
        }
      })?);
    }
    PartContentKind::Xml => {
      writer_stmt_list.push(parse2(quote! {
        use std::io::Write;
      })?);
      writer_stmt_list.push(parse2(quote! {
        if !entry_set.contains(&self.inner_path) {
          zip.start_file(&self.inner_path, options)?;
          zip.write_all(self.root_element.to_xml()?.as_bytes())?;
          entry_set.insert(self.inner_path.to_string());
        }
      })?);
    }
    PartContentKind::None => {
      if !part.children.is_empty() {
        writer_stmt_list.push(parse2(quote! {
          use std::io::Write;
        })?);
      }
    }
  }

  if !part.children.is_empty() {
    let path_str = path_str.as_str();
    writer_stmt_list.push(parse2(quote! {
      let child_parent_path = format!("{}{}", parent_path, #path_str);
    })?);
    writer_stmt_list.push(parse2(quote! {
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
    })?);
  }

  for child in &part.children {
    if child.is_data_part_reference {
      continue;
    }

    let child_api_name_ident: Ident = parse_str(&child.api_name.to_snake_case())?;
    let child_name_ident: Ident = parse_str(&child.name.to_snake_case())?;

    if child.max_occurs_great_than_one {
      children_writer_stmt_list.push(
        parse2(versioned_tokens(
          &child.version,
          quote! {
            for #child_name_ident in &self.#child_api_name_ident {
              #child_name_ident.save_zip(&child_parent_path, zip, entry_set)?;
            }
          },
        ))
        .map_err(|err| format!("{} repeated child writer {}: {err}", part.name, child.name))?,
      );
    } else if child.min_occurs_is_non_zero {
      children_writer_stmt_list.push(
        parse2(versioned_tokens(
          &child.version,
          quote! {
            self.#child_api_name_ident.save_zip(&child_parent_path, zip, entry_set)?;
          },
        ))
        .map_err(|err| format!("{} required child writer {}: {err}", part.name, child.name))?,
      );
    } else {
      children_writer_stmt_list.push(
        parse2(versioned_tokens(
          &child.version,
          quote! {
            if let Some(#child_api_name_ident) = &self.#child_api_name_ident {
              #child_api_name_ident.save_zip(&child_parent_path, zip, entry_set)?;
            }
          },
        ))
        .map_err(|err| format!("{} optional child writer {}: {err}", part.name, child.name))?,
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
  })?;

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
            None => continue,
          };
          file_path_set.insert(file_path);
        }
        Self::new_from_archive("", "", "", &file_path_set, &mut archive)
      }
    })?;

    let part_new_from_file_fn: ItemFn = parse2(quote! {
      pub fn new_from_file<P: AsRef<std::path::Path>>(
        path: P,
      ) -> Result<Self, crate::common::SdkError> {
        Self::new(std::io::BufReader::new(std::fs::File::open(path)?))
      }
    })?;

    let part_save_fn: ItemFn = parse2(quote! {
      pub fn save<W: std::io::Write + std::io::Seek>(
        &self,
        writer: W,
      ) -> Result<(), crate::common::SdkError> {
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
    })?;

    let part_save_to_file_fn: ItemFn = parse2(quote! {
      pub fn save_to_file<P: AsRef<std::path::Path>>(
        &self,
        path: P,
      ) -> Result<(), crate::common::SdkError> {
        self.save(std::fs::File::create(path)?)
      }
    })?;

    let part_impl: ItemImpl = parse2(quote! {
      impl #struct_name_ident {
        #part_new_fn
        #part_new_from_file_fn
        #part_new_from_archive_fn
        #part_save_fn
        #part_save_to_file_fn
        #part_save_zip_fn
      }
    })?;

    Ok(quote! {
      #relationship_type_stmt
      #part_struct
      #part_impl
    })
  } else {
    let part_impl: ItemImpl = parse2(quote! {
      impl #struct_name_ident {
        #part_new_from_archive_fn
        #part_save_zip_fn
      }
    })?;

    Ok(quote! {
      #relationship_type_stmt
      #part_struct
      #part_impl
    })
  }
}

pub fn gen_parts_mod(parts: &[Part]) -> Result<TokenStream> {
  let mut mod_list: Vec<ItemMod> = vec![];

  for part in parts {
    let mod_ident: Ident = parse_str(&part.module_name)?;
    let part_attrs = part_module_attrs(part);
    mod_list.push(parse2(quote! {
      #( #part_attrs )*
      pub mod #mod_ident;
    })?);
  }

  Ok(quote! {
    #( #mod_list )*
  })
}

fn part_module_attrs(part: &Part) -> Vec<syn::Attribute> {
  let filtered_features: Vec<String> = part
    .features
    .iter()
    .filter(|feature| feature.as_str() != "parts")
    .cloned()
    .collect();

  features_cfg_attrs(&filtered_features)
}

fn part_root_type_tokens(part: &Part) -> Result<Type> {
  if part.name == "CoreFilePropertiesPart" {
    return parse_str("crate::schemas::opc_core_properties::CoreProperties").map_err(Into::into);
  }

  parse_str(&format!(
    "crate::schemas::{}::{}",
    part.schema_module,
    part.root_class_name.to_upper_camel_case()
  ))
  .map_err(Into::into)
}

fn child_type_tokens(child: &PartChild) -> Result<Type> {
  parse_str(&format!(
    "crate::parts::{}::{}",
    child.name.to_snake_case(),
    child.name.to_upper_camel_case(),
  ))
  .map_err(Into::into)
}

fn child_field_tokens(child: &PartChild) -> Result<TokenStream> {
  let field_ident: Ident = parse_str(&child.api_name.to_snake_case())?;
  let child_type = child_type_tokens(child)?;

  if child.max_occurs_great_than_one {
    Ok(versioned_tokens(
      &child.version,
      quote! {
        pub #field_ident: Vec<#child_type>,
      },
    ))
  } else if child.min_occurs_is_non_zero {
    Ok(versioned_tokens(
      &child.version,
      quote! {
        pub #field_ident: std::boxed::Box<#child_type>,
      },
    ))
  } else {
    Ok(versioned_tokens(
      &child.version,
      quote! {
        pub #field_ident: Option<std::boxed::Box<#child_type>>,
      },
    ))
  }
}
