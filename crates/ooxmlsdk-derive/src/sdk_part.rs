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
      part_field_marker(field),
      Ok(Some(PartFieldMarker::ContentTypes))
    )
  });
  let has_relationships = fields.named.iter().any(|field| {
    matches!(
      part_field_marker(field),
      Ok(Some(PartFieldMarker::Relationships))
    )
  });
  let has_extended_parts = fields.named.iter().any(|field| {
    matches!(
      part_field_marker(field),
      Ok(Some(PartFieldMarker::ExtendedParts))
    )
  });

  let mut child_infos = Vec::new();
  let mut data_ref_infos = Vec::new();
  for field in &fields.named {
    if is_system_part_field(field) {
      continue;
    }

    if let Some(child_info) = parse_part_child_field(field)? {
      child_infos.push(child_info);
      continue;
    }

    if let Some(data_ref_info) = parse_part_data_ref_field(field)? {
      data_ref_infos.push(data_ref_info);
      continue;
    }

    return Err(syn::Error::new_spanned(
      field,
      "SdkPart fields require explicit #[sdk(part_child(...))], #[sdk(part_data_ref(...))], or a part system marker",
    ));
  }
  let needs_relationships = has_relationships
    || has_extended_parts
    || !child_infos.is_empty()
    || !data_ref_infos.is_empty();

  let mut field_declarations = Vec::new();
  let mut field_unwraps = Vec::new();
  let mut self_field_values = Vec::new();
  let mut child_match_arms = Vec::new();
  let mut child_save_stmts = Vec::new();
  let mut child_validate_stmts = Vec::new();
  let mut part_iter_chains = Vec::new();
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
  let part_index_ident = if has_root_element || content_kind.is_some() {
    parse_str::<Ident>("part_index")?
  } else {
    parse_str::<Ident>("_part_index")?
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
      let child_parent_path = crate::common::parent_zip_path(#path_ident);
    });
    field_declarations.push(quote! {
      let (rels_path, relationships) =
        crate::common::load_part_relationships(#path_ident, #archive_ident)?;
    });
    self_field_values.push(quote! { rels_path });
    self_field_values.push(quote! { relationships });
  }

  if has_extended_parts {
    field_declarations.push(quote! {
      let mut extended_parts: Vec<crate::common::ExtendedPart> = vec![];
    });
    self_field_values.push(quote! { extended_parts });
  }

  self_field_values.push(quote! { inner_path: #path_ident.to_string() });

  for child in &child_infos {
    let child_type = &child.ty;
    let Some(child_type_name) = terminal_type_ident(child_type) else {
      return Err(syn::Error::new_spanned(
        child_type,
        "failed to resolve child part type name",
      ));
    };
    let child_variant_ident: Ident = parse_str(&child_type_name)?;
    let relationship_type_expr = match &child.relationship_type {
      PartRelationshipTypeSource::Explicit(value) => quote! { #value },
      PartRelationshipTypeSource::TypeConst => {
        quote! { <#child_type as crate::sdk::SdkPart>::relationship_type() }
      }
    };
    let child_field_ident = &child.field_ident;
    let child_item_ident = child.field_ident.clone();

    match child.kind {
      PartChildKind::Repeated => {
        field_declarations.push(quote! {
          let mut #child_field_ident: Vec<#child_type> = vec![];
        });
        child_match_arms.push(quote! {
          relationship_type if crate::common::relationship_type_matches(
            relationship_type,
            #relationship_type_expr,
          ) => {
            if let Some(loaded_child) = crate::common::load_typed_child_part::<_, #child_type>(
              &child_parent_path,
              relationship,
              #archive_ident,
              visited,
            )? {
              #child_field_ident.push(loaded_child);
            }
          }
        });
        child_save_stmts.push(quote! {
          for #child_item_ident in &self.#child_field_ident {
            crate::common::save_typed_child_part(
              #child_item_ident,
              &child_parent_path,
              zip,
              entry_set,
              visited,
            )?;
          }
        });
        child_validate_stmts.push(quote! {
          for #child_item_ident in &self.#child_field_ident {
            crate::validator::SdkValidator::validate(#child_item_ident)?;
          }
        });
        part_iter_chains.push(quote! {
          self.#child_field_ident.iter().map(|part| {
            crate::parts::IdPartPair::new(
              part.r_id.as_str(),
              crate::parts::PartRef::#child_variant_ident(part),
            )
          })
        });
        self_field_values.push(quote! { #child_field_ident });
      }
      PartChildKind::Required => {
        field_declarations.push(quote! {
          let mut #child_field_ident: Option<std::boxed::Box<#child_type>> = None;
        });
        child_match_arms.push(quote! {
          relationship_type if crate::common::relationship_type_matches(
            relationship_type,
            #relationship_type_expr,
          ) => {
            if let Some(loaded_child) = crate::common::load_typed_child_part::<_, #child_type>(
              &child_parent_path,
              relationship,
              #archive_ident,
              visited,
            )? {
              #child_field_ident = Some(std::boxed::Box::new(loaded_child));
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
          crate::common::save_typed_child_part(
            self.#child_field_ident.as_ref(),
            &child_parent_path,
            zip,
            entry_set,
            visited,
          )?;
        });
        child_validate_stmts.push(quote! {
          crate::validator::SdkValidator::validate(self.#child_field_ident.as_ref())?;
        });
        part_iter_chains.push(quote! {
          std::iter::once(crate::parts::IdPartPair::new(
            self.#child_field_ident.r_id.as_str(),
            crate::parts::PartRef::#child_variant_ident(self.#child_field_ident.as_ref()),
          ))
        });
        self_field_values.push(quote! { #child_field_ident });
      }
      PartChildKind::Optional => {
        field_declarations.push(quote! {
          let mut #child_field_ident: Option<std::boxed::Box<#child_type>> = None;
        });
        child_match_arms.push(quote! {
          relationship_type if crate::common::relationship_type_matches(
            relationship_type,
            #relationship_type_expr,
          ) => {
            if let Some(loaded_child) = crate::common::load_typed_child_part::<_, #child_type>(
              &child_parent_path,
              relationship,
              #archive_ident,
              visited,
            )? {
              #child_field_ident = Some(std::boxed::Box::new(loaded_child));
            }
          }
        });
        child_save_stmts.push(quote! {
          if let Some(#child_item_ident) = &self.#child_field_ident {
            crate::common::save_typed_child_part(
              #child_item_ident.as_ref(),
              &child_parent_path,
              zip,
              entry_set,
              visited,
            )?;
          }
        });
        child_validate_stmts.push(quote! {
          if let Some(#child_item_ident) = &self.#child_field_ident {
            crate::validator::SdkValidator::validate(#child_item_ident.as_ref())?;
          }
        });
        part_iter_chains.push(quote! {
          self.#child_field_ident.iter().map(|part| {
            crate::parts::IdPartPair::new(
              part.r_id.as_str(),
              crate::parts::PartRef::#child_variant_ident(part.as_ref()),
            )
          })
        });
        self_field_values.push(quote! { #child_field_ident });
      }
    }
  }

  for data_ref in &data_ref_infos {
    let data_ref_type = &data_ref.ty;
    let relationship_type_expr = match &data_ref.relationship_type {
      PartRelationshipTypeSource::Explicit(value) => quote! { #value },
      PartRelationshipTypeSource::TypeConst => {
        quote! { <#data_ref_type as crate::sdk::SdkDataPartReference>::RELATIONSHIP_TYPE }
      }
    };
    let data_ref_field_ident = &data_ref.field_ident;
    let data_ref_item_ident = data_ref.field_ident.clone();

    match data_ref.kind {
      PartChildKind::Repeated => {
        field_declarations.push(quote! {
          let mut #data_ref_field_ident: Vec<#data_ref_type> = vec![];
        });
        child_match_arms.push(quote! {
          relationship_type if crate::common::relationship_type_matches(
            relationship_type,
            #relationship_type_expr,
          ) => {
            if let Some(loaded_data_ref) = crate::common::load_data_part_reference::<_, #data_ref_type>(
              &child_parent_path,
              relationship,
              #archive_ident,
            )? {
              #data_ref_field_ident.push(loaded_data_ref);
            }
          }
        });
        child_save_stmts.push(quote! {
          for #data_ref_item_ident in &self.#data_ref_field_ident {
            crate::common::save_data_part_reference(
              #data_ref_item_ident,
              &child_parent_path,
              zip,
              entry_set,
            )?;
          }
        });
        child_validate_stmts.push(quote! {
          for #data_ref_item_ident in &self.#data_ref_field_ident {
            crate::validator::SdkValidator::validate(#data_ref_item_ident)?;
          }
        });
        self_field_values.push(quote! { #data_ref_field_ident });
      }
      PartChildKind::Required => {
        field_declarations.push(quote! {
          let mut #data_ref_field_ident: Option<std::boxed::Box<#data_ref_type>> = None;
        });
        child_match_arms.push(quote! {
          relationship_type if crate::common::relationship_type_matches(
            relationship_type,
            #relationship_type_expr,
          ) => {
            if let Some(loaded_data_ref) = crate::common::load_data_part_reference::<_, #data_ref_type>(
              &child_parent_path,
              relationship,
              #archive_ident,
            )? {
              #data_ref_field_ident = Some(std::boxed::Box::new(loaded_data_ref));
            }
          }
        });
        field_unwraps.push(quote! {
          let #data_ref_field_ident = #data_ref_field_ident
            .ok_or_else(|| crate::common::missing_field(
              stringify!(#ident),
              stringify!(#data_ref_field_ident),
            ))?;
        });
        child_save_stmts.push(quote! {
          crate::common::save_data_part_reference(
            self.#data_ref_field_ident.as_ref(),
            &child_parent_path,
            zip,
            entry_set,
          )?;
        });
        child_validate_stmts.push(quote! {
          crate::validator::SdkValidator::validate(self.#data_ref_field_ident.as_ref())?;
        });
        self_field_values.push(quote! { #data_ref_field_ident });
      }
      PartChildKind::Optional => {
        field_declarations.push(quote! {
          let mut #data_ref_field_ident: Option<std::boxed::Box<#data_ref_type>> = None;
        });
        child_match_arms.push(quote! {
          relationship_type if crate::common::relationship_type_matches(
            relationship_type,
            #relationship_type_expr,
          ) => {
            if let Some(loaded_data_ref) = crate::common::load_data_part_reference::<_, #data_ref_type>(
              &child_parent_path,
              relationship,
              #archive_ident,
            )? {
              #data_ref_field_ident = Some(std::boxed::Box::new(loaded_data_ref));
            }
          }
        });
        child_save_stmts.push(quote! {
          if let Some(#data_ref_item_ident) = &self.#data_ref_field_ident {
            crate::common::save_data_part_reference(
              #data_ref_item_ident.as_ref(),
              &child_parent_path,
              zip,
              entry_set,
            )?;
          }
        });
        child_validate_stmts.push(quote! {
          if let Some(#data_ref_item_ident) = &self.#data_ref_field_ident {
            crate::validator::SdkValidator::validate(#data_ref_item_ident.as_ref())?;
          }
        });
        self_field_values.push(quote! { #data_ref_field_ident });
      }
    }
  }

  let extended_fallback_arm = if has_extended_parts {
    quote! {
      _ => {
        if let Some(loaded_extended_part) = crate::common::load_extended_part(
          &child_parent_path,
          relationship,
          #archive_ident,
          visited,
        )? {
          extended_parts.push(loaded_extended_part);
        }
      }
    }
  } else {
    quote! { _ => {} }
  };

  if needs_relationships {
    field_declarations.push(quote! {
      if let Some(relationships) = &relationships {
        for relationship in &relationships.relationship {
          match relationship.r#type.as_str() {
            #( #child_match_arms )*
            #extended_fallback_arm
          }
        }
      }
    });
  }

  if let Some(root_type) = root_type {
    field_declarations.push(quote! {
      let root_element = Some({
        use std::io::Read;

        let mut zip_entry = if let Some(part_index) = #part_index_ident {
          #archive_ident.by_index(part_index)?
        } else {
          #archive_ident.by_name(#path_ident)?
        };
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
            let zip_entry = if let Some(part_index) = #part_index_ident {
              #archive_ident.by_index(part_index)?
            } else {
              #archive_ident.by_name(#path_ident)?
            };
            let mut file = std::io::BufReader::new(zip_entry);
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
            let mut zip_entry = if let Some(part_index) = #part_index_ident {
              #archive_ident.by_index(part_index)?
            } else {
              #archive_ident.by_name(#path_ident)?
            };
            part_content = Vec::with_capacity(zip_entry.size() as usize);
            zip_entry.read_to_end(&mut part_content)?;
          }
        });
        self_field_values.push(quote! { part_content });
      }
    }
  }

  let trait_new_from_archive_impl = quote! {
    fn new_from_archive<R: std::io::Read + std::io::Seek>(
      #parent_path_ident: &str,
      #path_ident: &str,
      #r_id_ident: &str,
      #part_index_ident: Option<usize>,
      #archive_ident: &mut zip::ZipArchive<R>,
      visited: &mut std::collections::HashSet<usize>,
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
        crate::common::save_part_relationships(
          &self.inner_path,
          &self.rels_path,
          relationships,
          zip,
          entry_set,
        )?;
      }
    }
  } else {
    quote! {}
  };
  let extended_save_tokens = if has_extended_parts {
    quote! {
      for part in &self.extended_parts {
        part.save_zip(&child_parent_path, zip, entry_set, visited)?;
      }
    }
  } else {
    quote! {}
  };

  let trait_save_zip_impl = quote! {
    fn save_zip<W: std::io::Write + std::io::Seek>(
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
      #extended_save_tokens
      Ok(())
    }
  };

  let mut impl_items = Vec::new();

  if has_content_types {
    impl_items.insert(
      0,
      quote! {
        pub fn new<R: std::io::Read + std::io::Seek>(
          reader: R,
        ) -> Result<Self, crate::common::SdkError> {
          let mut archive = zip::ZipArchive::new(reader)?;
          let mut visited: std::collections::HashSet<usize> = std::collections::HashSet::new();
          <Self as crate::sdk::SdkPart>::new_from_archive(
            "",
            "",
            "",
            None,
            &mut archive,
            &mut visited,
          )
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
          <Self as crate::sdk::SdkPart>::save_zip(
            self,
            "",
            &mut zip,
            &mut entry_set,
            &mut visited,
          )?;
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

  let parts_methods = quote! {
    pub fn parts(&self) -> impl Iterator<Item = crate::parts::IdPartPair<'_>> + '_ {
      std::iter::empty::<crate::parts::IdPartPair<'_>>()
        #( .chain(#part_iter_chains) )*
    }

    pub fn get_parts_of_type<T: crate::sdk::SdkPart + 'static>(
      &self,
    ) -> impl Iterator<Item = &T> + '_ {
      self
        .parts()
        .filter_map(|entry| entry.part.downcast_ref::<T>())
    }

    pub fn get_sub_part_of_type<T: crate::sdk::SdkPart + 'static>(&self) -> Option<&T> {
      self.get_parts_of_type::<T>().next()
    }

    pub fn get_part_by_id(&self, relationship_id: &str) -> Option<crate::parts::PartRef<'_>> {
      self
        .parts()
        .find(|entry| entry.relationship_id == relationship_id)
        .map(|entry| entry.part)
    }

    pub fn get_part_by_id_as<T: crate::sdk::SdkPart + 'static>(
      &self,
      relationship_id: &str,
    ) -> Option<&T> {
      self
        .parts()
        .find(|entry| entry.relationship_id == relationship_id)
        .and_then(|entry| entry.part.downcast_ref::<T>())
    }

    pub fn get_id_of_part<T: crate::sdk::SdkPart + 'static>(&self, part: &T) -> Option<&str> {
      self.parts().find_map(|entry| {
        let candidate = entry.part.downcast_ref::<T>()?;
        std::ptr::eq(candidate, part).then_some(entry.relationship_id)
      })
    }
  };

  Ok(quote! {
    impl crate::sdk::SdkPart for #ident {
      const DESCRIPTOR: crate::sdk::PartDescriptor = crate::sdk::PartDescriptor::new(
        self::RELATIONSHIP_TYPE,
        self::PATH_PREFIX,
      );

      #trait_new_from_archive_impl

      #trait_save_zip_impl
    }
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

      #parts_methods

      #( #impl_items )*
    }
  })
}
