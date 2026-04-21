use super::*;

pub(crate) fn expand_sdk_part(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let Data::Struct(data_struct) = &input.data else {
    return Err(syn::Error::new_spanned(
      input,
      "SdkPart can only be derived for structs",
    ));
  };

  let Fields::Named(fields) = &data_struct.fields else {
    return Err(syn::Error::new_spanned(
      input,
      "SdkPart can only be derived for named-field structs",
    ));
  };

  let has_content_types = fields.named.iter().any(|field| {
    matches!(
      parse_part_field_marker(&field.attrs),
      Ok(Some(PartFieldMarker::ContentTypes))
    )
  });
  let has_relationships = fields.named.iter().any(|field| {
    matches!(
      parse_part_field_marker(&field.attrs),
      Ok(Some(PartFieldMarker::Relationships))
    )
  });

  let mut child_infos = Vec::new();
  for field in &fields.named {
    if is_system_part_field(field) {
      continue;
    }

    let Some(child_info) = parse_part_child_field(field)? else {
      return Err(syn::Error::new_spanned(
        field,
        "SdkPart fields require explicit #[sdk(part_child(...))] or a part system marker",
      ));
    };
    child_infos.push(child_info);
  }
  let needs_relationships = has_relationships || !child_infos.is_empty();

  let mut field_declarations = Vec::new();
  let mut field_unwraps = Vec::new();
  let mut self_field_values = Vec::new();
  let mut child_match_arms = Vec::new();
  let mut child_save_stmts = Vec::new();
  let mut child_validate_stmts = Vec::new();
  let root_type = part_root_type_from_fields(fields);
  let has_root_element = root_type.is_some();
  let content_kind = part_content_kind_from_fields(fields);
  let parent_path_ident = if needs_relationships {
    parse_str::<Ident>("parent_path")?
  } else {
    parse_str::<Ident>("_parent_path")?
  };
  let path_ident = if needs_relationships || root_type.is_some() || content_kind.is_some() {
    parse_str::<Ident>("path")?
  } else {
    parse_str::<Ident>("_path")?
  };
  let r_id_ident = if has_content_types {
    parse_str::<Ident>("_r_id")?
  } else {
    parse_str::<Ident>("r_id")?
  };
  let file_path_set_ident = if needs_relationships {
    parse_str::<Ident>("file_path_set")?
  } else {
    parse_str::<Ident>("_file_path_set")?
  };
  let archive_ident =
    if has_content_types || needs_relationships || has_root_element || content_kind.is_some() {
      parse_str::<Ident>("archive")?
    } else {
      parse_str::<Ident>("_archive")?
    };

  if has_content_types {
    field_declarations.push(quote! {
      use std::io::Read;
    });
    field_declarations.push(quote! {
      let content_types = {
        let mut zip_entry = #archive_ident.by_name("[Content_Types].xml")?;
        let mut bytes = Vec::with_capacity(zip_entry.size() as usize);
        zip_entry.read_to_end(&mut bytes)?;
        crate::schemas::opc_content_types::Types::from_bytes(&bytes)?
      };
    });
    self_field_values.push(quote! { content_types });
  } else {
    self_field_values.push(quote! { #r_id_ident: #r_id_ident.to_string() });
  }

  if needs_relationships {
    field_declarations.push(quote! {
      let mut rels_path = String::new();
    });
    field_declarations.push(quote! {
      let child_parent_path = crate::common::parent_zip_path(#path_ident);
    });
    field_declarations.push(quote! {
      let part_target_str = #path_ident
        .rsplit('/')
        .next()
        .unwrap_or_default();
    });
    field_declarations.push(quote! {
      let rels_candidate_path = crate::common::resolve_zip_file_path(
        &format!("{child_parent_path}_rels/{part_target_str}.rels"),
      );
    });
    field_declarations.push(quote! {
      let relationships = if let Some(file_path) = #file_path_set_ident.get(&rels_candidate_path) {
        rels_path = file_path.to_string();
        Some({
          use std::io::Read;

          let mut zip_entry = #archive_ident.by_name(file_path)?;
          let mut bytes = Vec::with_capacity(zip_entry.size() as usize);
          zip_entry.read_to_end(&mut bytes)?;
          crate::schemas::opc_relationships::Relationships::from_bytes(&bytes)?
        })
      } else {
        None
      };
    });
    self_field_values.push(quote! { rels_path });
    self_field_values.push(quote! { relationships });
  }

  self_field_values.push(quote! { inner_path: #path_ident.to_string() });

  for child in &child_infos {
    let child_type = &child.ty;
    let relationship_type_values = relationship_type_match_values(&child.relationship_type);
    let child_field_ident = &child.field_ident;
    let child_item_ident = child.field_ident.clone();
    let child_load_ident: Ident = parse_str(&format!("loaded_{}", child_item_ident))?;

    match child.kind {
      PartChildKind::Repeated => {
        field_declarations.push(quote! {
          let mut #child_field_ident: Vec<#child_type> = vec![];
        });
        child_match_arms.push(quote! {
          #( #relationship_type_values )|* => {
            let target_path = crate::common::resolve_zip_file_path(
              &crate::common::resolve_relationship_target_path(
                &child_parent_path,
                &relationship.target,
              ),
            );
            if #file_path_set_ident.contains(&target_path) {
              let inserted = visited.insert(target_path.clone());
              if inserted {
                let #child_load_ident = #child_type::new_from_archive(
                  &child_parent_path,
                  &target_path,
                  &relationship.id,
                  #file_path_set_ident,
                  #archive_ident,
                  visited,
                );
                visited.remove(&target_path);
                let #child_load_ident = #child_load_ident?;
                #child_field_ident.push(#child_load_ident);
              }
            }
          }
        });
        child_save_stmts.push(quote! {
          for #child_item_ident in &self.#child_field_ident {
            #child_item_ident.save_zip(&child_parent_path, zip, entry_set, visited)?;
          }
        });
        child_validate_stmts.push(quote! {
          for #child_item_ident in &self.#child_field_ident {
            crate::validator::SdkValidator::validate(#child_item_ident)?;
          }
        });
        self_field_values.push(quote! { #child_field_ident });
      }
      PartChildKind::Required => {
        field_declarations.push(quote! {
          let mut #child_field_ident: Option<std::boxed::Box<#child_type>> = None;
        });
        child_match_arms.push(quote! {
          #( #relationship_type_values )|* => {
            let target_path = crate::common::resolve_zip_file_path(
              &crate::common::resolve_relationship_target_path(
                &child_parent_path,
                &relationship.target,
              ),
            );
            if #file_path_set_ident.contains(&target_path) {
              let inserted = visited.insert(target_path.clone());
              if inserted {
                let #child_load_ident = #child_type::new_from_archive(
                  &child_parent_path,
                  &target_path,
                  &relationship.id,
                  #file_path_set_ident,
                  #archive_ident,
                  visited,
                );
                visited.remove(&target_path);
                let #child_load_ident = #child_load_ident?;
                #child_field_ident = Some(std::boxed::Box::new(#child_load_ident));
              }
            }
          }
        });
        field_unwraps.push(quote! {
          let #child_field_ident = #child_field_ident
            .ok_or_else(|| crate::common::missing_field(
              stringify!(#ident),
              stringify!(#child_field_ident),
            ))?;
        });
        child_save_stmts.push(quote! {
          self.#child_field_ident.save_zip(&child_parent_path, zip, entry_set, visited)?;
        });
        child_validate_stmts.push(quote! {
          crate::validator::SdkValidator::validate(self.#child_field_ident.as_ref())?;
        });
        self_field_values.push(quote! { #child_field_ident });
      }
      PartChildKind::Optional => {
        field_declarations.push(quote! {
          let mut #child_field_ident: Option<std::boxed::Box<#child_type>> = None;
        });
        child_match_arms.push(quote! {
          #( #relationship_type_values )|* => {
            let target_path = crate::common::resolve_zip_file_path(
              &crate::common::resolve_relationship_target_path(
                &child_parent_path,
                &relationship.target,
              ),
            );
            if #file_path_set_ident.contains(&target_path) {
              let inserted = visited.insert(target_path.clone());
              if inserted {
                let #child_load_ident = #child_type::new_from_archive(
                  &child_parent_path,
                  &target_path,
                  &relationship.id,
                  #file_path_set_ident,
                  #archive_ident,
                  visited,
                );
                visited.remove(&target_path);
                let #child_load_ident = #child_load_ident?;
                #child_field_ident = Some(std::boxed::Box::new(#child_load_ident));
              }
            }
          }
        });
        child_save_stmts.push(quote! {
          if let Some(#child_item_ident) = &self.#child_field_ident {
            #child_item_ident.save_zip(&child_parent_path, zip, entry_set, visited)?;
          }
        });
        child_validate_stmts.push(quote! {
          if let Some(#child_item_ident) = &self.#child_field_ident {
            crate::validator::SdkValidator::validate(#child_item_ident.as_ref())?;
          }
        });
        self_field_values.push(quote! { #child_field_ident });
      }
    }
  }

  if needs_relationships {
    field_declarations.push(quote! {
      if let Some(relationships) = &relationships {
        for relationship in &relationships.relationship {
          match relationship.r#type.as_str() {
            #( #child_match_arms )*
            _ => {}
          }
        }
      }
    });
  }

  if let Some(root_type) = root_type {
    field_declarations.push(quote! {
      let root_element = Some({
        use std::io::Read;

        let mut zip_entry = #archive_ident.by_name(#path_ident)?;
        let mut bytes = Vec::with_capacity(zip_entry.size() as usize);
        zip_entry.read_to_end(&mut bytes)?;
        #root_type::from_bytes(&bytes)?
      });
    });
    field_unwraps.push(quote! {
      let root_element = root_element
        .ok_or_else(|| crate::common::missing_field(
          stringify!(#ident),
          "root_element",
        ))?;
    });
    self_field_values.push(quote! { root_element });
  } else if let Some(kind) = content_kind {
    match kind {
      DerivedPartContentKind::Text => {
        field_declarations.push(quote! {
          use std::io::Read;
        });
        field_declarations.push(quote! {
          let mut part_content = String::new();
        });
        field_declarations.push(quote! {
          {
            let mut file = std::io::BufReader::new(#archive_ident.by_name(#path_ident)?);
            file.read_to_string(&mut part_content)?;
          }
        });
        self_field_values.push(quote! { part_content });
      }
      DerivedPartContentKind::Binary => {
        field_declarations.push(quote! {
          use std::io::Read;
        });
        field_declarations.push(quote! {
          let mut part_content;
        });
        field_declarations.push(quote! {
          {
            let mut zip_entry = #archive_ident.by_name(#path_ident)?;
            part_content = Vec::with_capacity(zip_entry.size() as usize);
            zip_entry.read_to_end(&mut part_content)?;
          }
        });
        self_field_values.push(quote! { part_content });
      }
    }
  }

  let new_from_archive_impl = quote! {
    pub(crate) fn new_from_archive<R: std::io::Read + std::io::Seek>(
      #parent_path_ident: &str,
      #path_ident: &str,
      #r_id_ident: &str,
      #file_path_set_ident: &std::collections::HashSet<String>,
      #archive_ident: &mut zip::ZipArchive<R>,
      visited: &mut std::collections::HashSet<String>,
    ) -> Result<Self, crate::common::SdkError> {
      #( #field_declarations )*
      #( #field_unwraps )*
      Ok(Self {
        #(
          #self_field_values,
        )*
      })
    }
  };

  let mut content_write_stmts: Vec<proc_macro2::TokenStream> = Vec::new();
  if has_root_element {
    content_write_stmts.push(quote! {
      if entry_set.insert(self.inner_path.clone()) {
        zip.start_file(&self.inner_path, options)?;
        let xml = self.root_element.to_xml_bytes()?;
        zip.write_all(&xml)?;
      }
    });
  } else if let Some(kind) = content_kind {
    match kind {
      DerivedPartContentKind::Text => {
        content_write_stmts.push(quote! {
          if entry_set.insert(self.inner_path.clone()) {
            zip.start_file(&self.inner_path, options)?;
            zip.write_all(self.part_content.as_bytes())?;
          }
        });
      }
      DerivedPartContentKind::Binary => {
        content_write_stmts.push(quote! {
          if entry_set.insert(self.inner_path.clone()) {
            zip.start_file(&self.inner_path, options)?;
            zip.write_all(&self.part_content)?;
          }
        });
      }
    }
  }

  let relationships_write_tokens = if needs_relationships {
    quote! {
      if let Some(relationships) = &self.relationships {
        let rels_parent_path = crate::common::parent_zip_path(&self.inner_path);
        let rels_dir_path = crate::common::resolve_zip_file_path(
          &format!("{rels_parent_path}_rels"),
        );
        if !rels_dir_path.is_empty() && entry_set.insert(rels_dir_path.clone()) {
          zip.add_directory(&rels_dir_path, options)?;
        }
        if entry_set.insert(self.rels_path.clone()) {
          zip.start_file(&self.rels_path, options)?;
          let xml = relationships.to_xml_bytes()?;
          zip.write_all(&xml)?;
        }
      }
    }
  } else {
    quote! {}
  };

  let save_zip_impl = quote! {
    pub(crate) fn save_zip<W: std::io::Write + std::io::Seek>(
      &self,
      parent_path: &str,
      zip: &mut zip::ZipWriter<W>,
      entry_set: &mut std::collections::HashSet<String>,
      visited: &mut std::collections::HashSet<String>,
    ) -> Result<(), crate::common::SdkError> {
      use std::io::Write;
      if !visited.insert(self.inner_path.clone()) {
        return Ok(());
      }
      let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);
      let directory_path = crate::common::resolve_zip_file_path(parent_path);
      if !directory_path.is_empty() && entry_set.insert(directory_path.clone()) {
        zip.add_directory(&directory_path, options)?;
      }
      let child_parent_path = crate::common::parent_zip_path(&self.inner_path);
      let dir_path = self
        .inner_path
        .rsplit_once('/')
        .map(|(dir_path, _)| crate::common::resolve_zip_file_path(&format!("{dir_path}/")))
        .unwrap_or_default();
      if !dir_path.is_empty() && entry_set.insert(dir_path.clone()) {
        zip.add_directory(&dir_path, options)?;
      }
      #relationships_write_tokens
      #( #content_write_stmts )*
      #( #child_save_stmts )*
      Ok(())
    }
  };

  let mut impl_items = vec![new_from_archive_impl, save_zip_impl];

  if has_content_types {
    impl_items.insert(
      0,
      quote! {
        pub fn new<R: std::io::Read + std::io::Seek>(
          reader: R,
        ) -> Result<Self, crate::common::SdkError> {
          let mut archive = zip::ZipArchive::new(reader)?;
          let mut file_path_set: std::collections::HashSet<String> =
            std::collections::HashSet::with_capacity(archive.len());
          let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();
          for i in 0..archive.len() {
            let file = archive.by_index(i)?;
            let file_path = match file.enclosed_name() {
              Some(path) => path.to_string_lossy().to_string(),
              None => continue,
            };
            file_path_set.insert(file_path);
          }
          Self::new_from_archive("", "", "", &file_path_set, &mut archive, &mut visited)
        }
      },
    );
    impl_items.insert(
      1,
      quote! {
        pub fn new_from_file<P: AsRef<std::path::Path>>(
          path: P,
        ) -> Result<Self, crate::common::SdkError> {
          Self::new(std::io::BufReader::new(std::fs::File::open(path)?))
        }
      },
    );
    impl_items.insert(
      2,
      quote! {
        pub fn save<W: std::io::Write + std::io::Seek>(
          &self,
          writer: W,
        ) -> Result<(), crate::common::SdkError> {
          use std::io::Write;
          let mut entry_set: std::collections::HashSet<String> = std::collections::HashSet::new();
          let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();
          let mut zip = zip::ZipWriter::new(writer);
          let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);
          zip.start_file("[Content_Types].xml", options)?;
          let xml = self.content_types.to_xml_bytes()?;
          zip.write_all(&xml)?;
          self.save_zip("", &mut zip, &mut entry_set, &mut visited)?;
          zip.finish()?;
          Ok(())
        }
      },
    );
    impl_items.insert(
      3,
      quote! {
        pub fn save_to_file<P: AsRef<std::path::Path>>(
          &self,
          path: P,
        ) -> Result<(), crate::common::SdkError> {
          self.save(std::fs::File::create(path)?)
        }
      },
    );
  }

  let part_validate_stmts = if has_root_element {
    let mut part_validate_stmts = child_validate_stmts;
    part_validate_stmts.insert(
      0,
      quote! {
        crate::validator::SdkValidator::validate(&self.root_element)?;
      },
    );
    part_validate_stmts
  } else {
    child_validate_stmts
  };

  Ok(quote! {
    impl crate::sdk::SdkPart for #ident {}
    #[cfg(feature = "validators")]
    impl crate::validator::SdkValidator for #ident {
      fn validate(&self) -> Result<(), crate::common::SdkError> {
        #( #part_validate_stmts )*
        Ok(())
      }
    }
    impl #ident {
      #[cfg(feature = "validators")]
      pub fn validate(&self) -> Result<(), crate::common::SdkError> {
        crate::validator::SdkValidator::validate(self)
      }

      #[cfg(feature = "validators")]
      pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
      }

      #( #impl_items )*
    }
  })
}
