use super::*;

fn build_conditional_chain(
  branches: &[(proc_macro2::TokenStream, proc_macro2::TokenStream)],
  fallback: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  if branches.is_empty() {
    return fallback;
  }

  let mut chain = proc_macro2::TokenStream::new();

  for (index, (condition, body)) in branches.iter().enumerate() {
    if index == 0 {
      chain.extend(quote! {
        if #condition {
          #body
        }
      });
    } else {
      chain.extend(quote! {
        else if #condition {
          #body
        }
      });
    }
  }

  chain.extend(quote! {
    else {
      #fallback
    }
  });

  chain
}

fn explicit_relationship_type_may_have_alias(value: &str) -> bool {
  value.starts_with("http://schemas.openxmlformats.org/officeDocument/2006/relationships/")
    || value == "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail"
    || value == "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects"
}

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

  if is_part_handle_struct(fields) {
    return expand_part_handle(input, fields);
  }

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
  let mut alias_match_branches = Vec::new();
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
    let relationship_type_pattern = match &child.relationship_type {
      PartRelationshipTypeSource::Explicit(value) => quote! { #value },
      PartRelationshipTypeSource::TypeConst => {
        quote! { <#child_type as crate::sdk::SdkPart>::RELATIONSHIP_TYPE }
      }
    };
    let alias_match_condition = match &child.relationship_type {
      PartRelationshipTypeSource::Explicit(value)
        if explicit_relationship_type_may_have_alias(value) =>
      {
        Some(quote! {
          crate::common::relationship_type_matches_alias(
            relationship_type,
            #relationship_type_pattern,
          )
        })
      }
      PartRelationshipTypeSource::TypeConst => Some(quote! {
        crate::common::relationship_type_matches_alias(
          relationship_type,
          #relationship_type_pattern,
        )
      }),
      _ => None,
    };
    let child_field_ident = &child.field_ident;
    let child_item_ident = child.field_ident.clone();

    match child.kind {
      PartChildKind::Repeated => {
        field_declarations.push(quote! {
          let mut #child_field_ident: Vec<#child_type> = vec![];
        });
        let load_child_tokens = quote! {
          if let Some(loaded_child) = crate::common::load_typed_child_part::<_, #child_type>(
            &child_parent_path,
            relationship,
            #archive_ident,
            visited,
          )? {
            #child_field_ident.push(loaded_child);
          }
        };
        child_match_arms.push(quote! {
          #relationship_type_pattern => {
            #load_child_tokens
          }
        });
        if let Some(alias_match_condition) = alias_match_condition {
          alias_match_branches.push((alias_match_condition, load_child_tokens.clone()));
        }
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
        let load_child_tokens = quote! {
          if let Some(loaded_child) = crate::common::load_typed_child_part::<_, #child_type>(
            &child_parent_path,
            relationship,
            #archive_ident,
            visited,
          )? {
            #child_field_ident = Some(std::boxed::Box::new(loaded_child));
          }
        };
        child_match_arms.push(quote! {
          #relationship_type_pattern => {
            #load_child_tokens
          }
        });
        if let Some(alias_match_condition) = alias_match_condition {
          alias_match_branches.push((alias_match_condition, load_child_tokens.clone()));
        }
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
        let load_child_tokens = quote! {
          if let Some(loaded_child) = crate::common::load_typed_child_part::<_, #child_type>(
            &child_parent_path,
            relationship,
            #archive_ident,
            visited,
          )? {
            #child_field_ident = Some(std::boxed::Box::new(loaded_child));
          }
        };
        child_match_arms.push(quote! {
          #relationship_type_pattern => {
            #load_child_tokens
          }
        });
        if let Some(alias_match_condition) = alias_match_condition {
          alias_match_branches.push((alias_match_condition, load_child_tokens.clone()));
        }
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
    let relationship_type_pattern = match &data_ref.relationship_type {
      PartRelationshipTypeSource::Explicit(value) => quote! { #value },
      PartRelationshipTypeSource::TypeConst => {
        quote! { <#data_ref_type as crate::sdk::SdkDataPartReference>::RELATIONSHIP_TYPE }
      }
    };
    let alias_match_condition = match &data_ref.relationship_type {
      PartRelationshipTypeSource::Explicit(value)
        if explicit_relationship_type_may_have_alias(value) =>
      {
        Some(quote! {
          crate::common::relationship_type_matches_alias(
            relationship_type,
            #relationship_type_pattern,
          )
        })
      }
      PartRelationshipTypeSource::TypeConst => Some(quote! {
        crate::common::relationship_type_matches_alias(
          relationship_type,
          #relationship_type_pattern,
        )
      }),
      _ => None,
    };
    let data_ref_field_ident = &data_ref.field_ident;
    let data_ref_item_ident = data_ref.field_ident.clone();

    match data_ref.kind {
      PartChildKind::Repeated => {
        field_declarations.push(quote! {
          let mut #data_ref_field_ident: Vec<#data_ref_type> = vec![];
        });
        let load_data_ref_tokens = quote! {
          if let Some(loaded_data_ref) = crate::common::load_data_part_reference::<_, #data_ref_type>(
            &child_parent_path,
            relationship,
            #archive_ident,
          )? {
            #data_ref_field_ident.push(loaded_data_ref);
          }
        };
        child_match_arms.push(quote! {
          #relationship_type_pattern => {
            #load_data_ref_tokens
          }
        });
        if let Some(alias_match_condition) = alias_match_condition {
          alias_match_branches.push((alias_match_condition, load_data_ref_tokens.clone()));
        }
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
        let load_data_ref_tokens = quote! {
          if let Some(loaded_data_ref) = crate::common::load_data_part_reference::<_, #data_ref_type>(
            &child_parent_path,
            relationship,
            #archive_ident,
          )? {
            #data_ref_field_ident = Some(std::boxed::Box::new(loaded_data_ref));
          }
        };
        child_match_arms.push(quote! {
          #relationship_type_pattern => {
            #load_data_ref_tokens
          }
        });
        if let Some(alias_match_condition) = alias_match_condition {
          alias_match_branches.push((alias_match_condition, load_data_ref_tokens.clone()));
        }
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
        let load_data_ref_tokens = quote! {
          if let Some(loaded_data_ref) = crate::common::load_data_part_reference::<_, #data_ref_type>(
            &child_parent_path,
            relationship,
            #archive_ident,
          )? {
            #data_ref_field_ident = Some(std::boxed::Box::new(loaded_data_ref));
          }
        };
        child_match_arms.push(quote! {
          #relationship_type_pattern => {
            #load_data_ref_tokens
          }
        });
        if let Some(alias_match_condition) = alias_match_condition {
          alias_match_branches.push((alias_match_condition, load_data_ref_tokens.clone()));
        }
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

  let extended_fallback_tokens = if has_extended_parts {
    quote! {
      if let Some(loaded_extended_part) = crate::common::load_extended_part(
        &child_parent_path,
        relationship,
        #archive_ident,
        visited,
      )? {
        extended_parts.push(loaded_extended_part);
      }
    }
  } else {
    quote! {}
  };

  if needs_relationships {
    let alias_fallback_tokens = if alias_match_branches.is_empty() {
      extended_fallback_tokens
    } else {
      let alias_dispatch_chain =
        build_conditional_chain(&alias_match_branches, extended_fallback_tokens);
      quote! {
        let relationship_type = relationship.r#type.as_str();
        #alias_dispatch_chain
      }
    };
    field_declarations.push(quote! {
      if let Some(relationships) = &relationships {
        for relationship in &relationships.relationship {
          match relationship.r#type.as_str() {
            #( #child_match_arms )*
            _ => {
              #alias_fallback_tokens
            }
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
        self::CONTENT_TYPE,
        self::TARGET_NAME,
        self::EXTENSION,
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

struct PartHandleChildInfo {
  field_ident: Ident,
  part_ty: Type,
  variant_ident: Ident,
  kind: PartChildKind,
  relationship_type: String,
}

struct PartChildMarkerInfo {
  part_ty: Type,
  kind: PartChildKind,
}

struct PartHandleRootInfo {
  root_ty: Type,
  accessor_ident: Ident,
}

fn is_part_handle_struct(fields: &syn::FieldsNamed) -> bool {
  fields
    .named
    .iter()
    .any(|field| field.ident.as_ref().is_some_and(|ident| ident == "id"))
    && fields.named.iter().all(|field| {
      field.ident.as_ref().is_some_and(|ident| ident == "id")
        || part_child_marker_info(&field.ty).is_some()
        || marker_inner_type(&field.ty, "PartRoot").is_some()
    })
}

fn expand_part_handle(
  input: &DeriveInput,
  fields: &syn::FieldsNamed,
) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let field_inits = fields.named.iter().map(|field| {
    let field_ident = field.ident.as_ref().unwrap();
    if field_ident == "id" {
      quote! { #field_ident: part_id }
    } else {
      quote! { #field_ident: Default::default() }
    }
  });

  let mut child_infos = Vec::new();
  let mut root_info = None;
  for field in &fields.named {
    if field.ident.as_ref().is_some_and(|ident| ident == "id") {
      continue;
    }

    if let Some(marker) = part_child_marker_info(&field.ty) {
      let Some(relationship_type) = parse_part_child_relationship_type_attr(&field.attrs)? else {
        return Err(syn::Error::new_spanned(
          field,
          "PartChild marker field requires #[sdk(part_child(relationship_type = ...))]",
        ));
      };
      let field_ident = field.ident.clone().unwrap();
      child_infos.push(PartHandleChildInfo {
        variant_ident: part_ref_variant_ident(&marker.part_ty)?,
        field_ident,
        part_ty: marker.part_ty,
        kind: marker.kind,
        relationship_type,
      });
      continue;
    }

    if let Some(root_ty) = marker_inner_type(&field.ty, "PartRoot") {
      root_info = Some(PartHandleRootInfo {
        root_ty,
        accessor_ident: parse_part_root_accessor(&field.attrs, ident)?,
      });
      continue;
    }
  }

  let root_method = part_handle_root_method_tokens(ident, root_info.as_ref());
  let child_methods = part_handle_child_methods_tokens(ident, &child_infos);

  Ok(quote! {
    impl crate::sdk::SdkPartHandle for #ident {
      const DESCRIPTOR: crate::sdk::PartDescriptor = crate::sdk::PartDescriptor::new(
        self::RELATIONSHIP_TYPE,
        self::PATH_PREFIX,
        self::CONTENT_TYPE,
        self::TARGET_NAME,
        self::EXTENSION,
      );

      #[inline]
      fn from_part_id(part_id: crate::common::PartId) -> Self {
        Self {
          #( #field_inits, )*
        }
      }

      #[inline]
      fn part_id(self) -> crate::common::PartId {
        self.id
      }
    }

    impl #ident {
      #[inline]
      pub const fn part_id(self) -> crate::common::PartId {
        self.id
      }

      #[inline]
      pub fn relationships<P: crate::sdk::SdkPackage>(
        self,
        package: &P,
      ) -> Option<&crate::common::RelationshipSet> {
        <Self as crate::sdk::SdkPartHandle>::relationships(self, package)
      }

      #[inline]
      pub fn path<P: crate::sdk::SdkPackage>(self, package: &P) -> Option<&str> {
        <Self as crate::sdk::SdkPartHandle>::path(self, package)
      }

      #[inline]
      pub fn content_type<P: crate::sdk::SdkPackage>(self, package: &P) -> Option<&str> {
        <Self as crate::sdk::SdkPartHandle>::content_type(self, package)
      }

      #[inline]
      pub fn data_kind<P: crate::sdk::SdkPackage>(
        self,
        package: &P,
      ) -> Option<crate::common::StoredPartDataKind> {
        <Self as crate::sdk::SdkPartHandle>::data_kind(self, package)
      }

      #[inline]
      pub fn data<P: crate::sdk::SdkPackage>(self, package: &P) -> Option<&[u8]> {
        <Self as crate::sdk::SdkPartHandle>::data(self, package)
      }

      #[inline]
      pub fn set_data<P: crate::sdk::SdkPackage>(
        self,
        package: &mut P,
        data: impl Into<Vec<u8>>,
      ) -> Result<(), crate::common::SdkError> {
        <Self as crate::sdk::SdkPartHandle>::set_data(self, package, data)
      }

      #[inline]
      pub fn feed_data<P: crate::sdk::SdkPackage, R: std::io::Read>(
        self,
        package: &mut P,
        reader: &mut R,
      ) -> Result<(), crate::common::SdkError> {
        <Self as crate::sdk::SdkPartHandle>::feed_data(self, package, reader)
      }

      #[inline]
      pub fn external_relationships<P: crate::sdk::SdkPackage>(
        self,
        package: &P,
      ) -> impl Iterator<Item = &crate::common::RelationshipInfo> {
        <Self as crate::sdk::SdkPartHandle>::external_relationships(self, package)
      }

      #[inline]
      pub fn hyperlink_relationships<P: crate::sdk::SdkPackage>(
        self,
        package: &P,
      ) -> impl Iterator<Item = &crate::common::RelationshipInfo> {
        <Self as crate::sdk::SdkPartHandle>::hyperlink_relationships(self, package)
      }

      #[inline]
      pub fn data_part_reference_relationships<P: crate::sdk::SdkPackage>(
        self,
        package: &P,
      ) -> impl Iterator<Item = &crate::common::RelationshipInfo> {
        <Self as crate::sdk::SdkPartHandle>::data_part_reference_relationships(self, package)
      }

      #[inline]
      pub fn relationships_by_type<'a, P: crate::sdk::SdkPackage>(
        self,
        package: &'a P,
        relationship_type: &'a str,
      ) -> impl Iterator<Item = &'a crate::common::RelationshipInfo> {
        <Self as crate::sdk::SdkPartHandle>::relationships_by_type(
          self,
          package,
          relationship_type,
        )
      }

      #[inline]
      pub fn add_external_relationship<P: crate::sdk::SdkPackage>(
        self,
        package: &mut P,
        relationship_id: impl Into<String>,
        relationship_type: impl Into<String>,
        target: impl Into<String>,
      ) -> Result<&crate::common::RelationshipInfo, crate::common::SdkError> {
        <Self as crate::sdk::SdkPartHandle>::add_external_relationship(
          self,
          package,
          relationship_id,
          relationship_type,
          target,
        )
      }

      #[inline]
      pub fn add_hyperlink_relationship<P: crate::sdk::SdkPackage>(
        self,
        package: &mut P,
        relationship_id: impl Into<String>,
        target: impl Into<String>,
      ) -> Result<&crate::common::RelationshipInfo, crate::common::SdkError> {
        <Self as crate::sdk::SdkPartHandle>::add_hyperlink_relationship(
          self,
          package,
          relationship_id,
          target,
        )
      }

      #[inline]
      pub fn add_new_part<P, T>(
        self,
        package: &mut P,
        relationship_id: impl Into<String>,
      ) -> Result<T, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
        T: crate::sdk::SdkPartHandle,
      {
        <Self as crate::sdk::SdkPartHandle>::add_new_part(
          self,
          package,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_new_part_with_content_type<P, T>(
        self,
        package: &mut P,
        relationship_id: impl Into<String>,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<T, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
        T: crate::sdk::SdkPartHandle,
      {
        <Self as crate::sdk::SdkPartHandle>::add_new_part_with_content_type(
          self,
          package,
          relationship_id,
          content_type,
        )
      }

      #[inline]
      pub fn add_new_part_auto_id<P, T>(
        self,
        package: &mut P,
      ) -> Result<T, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
        T: crate::sdk::SdkPartHandle,
      {
        <Self as crate::sdk::SdkPartHandle>::add_new_part_auto_id(self, package)
      }

      #[inline]
      pub fn add_new_part_with_content_type_auto_id<P, T>(
        self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<T, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
        T: crate::sdk::SdkPartHandle,
      {
        <Self as crate::sdk::SdkPartHandle>::add_new_part_with_content_type_auto_id(
          self,
          package,
          content_type,
        )
      }

      #[inline]
      pub fn add_extended_part<P>(
        self,
        package: &mut P,
        relationship_type: impl Into<String>,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        target_extension: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<crate::parts::extended_part::ExtendedPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_extended_part(
          self,
          package,
          relationship_type,
          content_type,
          target_extension,
        )
      }

      #[inline]
      pub fn add_extended_part_with_id<P>(
        self,
        package: &mut P,
        relationship_type: impl Into<String>,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        target_extension: impl Into<std::borrow::Cow<'static, str>>,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::extended_part::ExtendedPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_extended_part_with_id(
          self,
          package,
          relationship_type,
          content_type,
          target_extension,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_image_part<P>(
        self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<crate::parts::image_part::ImagePart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_image_part(
          self,
          package,
          content_type,
        )
      }

      #[inline]
      pub fn add_image_part_with_id<P>(
        self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::image_part::ImagePart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_image_part_with_id(
          self,
          package,
          content_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_alternative_format_import_part<P>(
        self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<
        crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_alternative_format_import_part(
          self,
          package,
          content_type,
        )
      }

      #[inline]
      pub fn add_alternative_format_import_part_with_id<P>(
        self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        relationship_id: impl Into<String>,
      ) -> Result<
        crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_alternative_format_import_part_with_id(
          self,
          package,
          content_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_alternative_format_import_part_by_type<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::AlternativeFormatImportPartType,
      ) -> Result<
        crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_alternative_format_import_part_by_type(
          self,
          package,
          part_type,
        )
      }

      #[inline]
      pub fn add_alternative_format_import_part_by_type_with_id<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::AlternativeFormatImportPartType,
        relationship_id: impl Into<String>,
      ) -> Result<
        crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_alternative_format_import_part_by_type_with_id(
          self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_custom_xml_part<P>(
        self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<crate::parts::custom_xml_part::CustomXmlPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_custom_xml_part(
          self,
          package,
          content_type,
        )
      }

      #[inline]
      pub fn add_custom_xml_part_with_id<P>(
        self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::custom_xml_part::CustomXmlPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_custom_xml_part_with_id(
          self,
          package,
          content_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_custom_xml_part_by_type<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::CustomXmlPartType,
      ) -> Result<crate::parts::custom_xml_part::CustomXmlPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_custom_xml_part_by_type(
          self,
          package,
          part_type,
        )
      }

      #[inline]
      pub fn add_custom_xml_part_by_type_with_id<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::CustomXmlPartType,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::custom_xml_part::CustomXmlPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_custom_xml_part_by_type_with_id(
          self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_custom_property_part_by_type<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::CustomPropertyPartType,
      ) -> Result<crate::parts::custom_property_part::CustomPropertyPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_custom_property_part_by_type(
          self, package, part_type,
        )
      }

      #[inline]
      pub fn add_custom_property_part_by_type_with_id<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::CustomPropertyPartType,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::custom_property_part::CustomPropertyPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_custom_property_part_by_type_with_id(
          self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_embedded_object_part_by_type<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::EmbeddedObjectPartType,
      ) -> Result<crate::parts::embedded_object_part::EmbeddedObjectPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_embedded_object_part_by_type(
          self, package, part_type,
        )
      }

      #[inline]
      pub fn add_embedded_object_part_by_type_with_id<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::EmbeddedObjectPartType,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::embedded_object_part::EmbeddedObjectPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_embedded_object_part_by_type_with_id(
          self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_embedded_package_part<P>(
        self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<crate::parts::embedded_package_part::EmbeddedPackagePart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_embedded_package_part(
          self,
          package,
          content_type,
        )
      }

      #[inline]
      pub fn add_embedded_package_part_with_id<P>(
        self,
        package: &mut P,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::embedded_package_part::EmbeddedPackagePart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_embedded_package_part_with_id(
          self,
          package,
          content_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_embedded_package_part_by_type<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::EmbeddedPackagePartType,
      ) -> Result<crate::parts::embedded_package_part::EmbeddedPackagePart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_embedded_package_part_by_type(
          self, package, part_type,
        )
      }

      #[inline]
      pub fn add_embedded_package_part_by_type_with_id<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::EmbeddedPackagePartType,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::embedded_package_part::EmbeddedPackagePart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_embedded_package_part_by_type_with_id(
          self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_font_part_by_type<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::FontPartType,
      ) -> Result<crate::parts::font_part::FontPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_font_part_by_type(self, package, part_type)
      }

      #[inline]
      pub fn add_font_part_by_type_with_id<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::FontPartType,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::font_part::FontPart, crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_font_part_by_type_with_id(
          self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_mail_merge_recipient_data_part_by_type<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::MailMergeRecipientDataPartType,
      ) -> Result<
        crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_mail_merge_recipient_data_part_by_type(
          self, package, part_type,
        )
      }

      #[inline]
      pub fn add_mail_merge_recipient_data_part_by_type_with_id<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::MailMergeRecipientDataPartType,
        relationship_id: impl Into<String>,
      ) -> Result<
        crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_mail_merge_recipient_data_part_by_type_with_id(
          self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_embedded_control_persistence_binary_data_part_by_type<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::EmbeddedControlPersistenceBinaryDataPartType,
      ) -> Result<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_embedded_control_persistence_binary_data_part_by_type(
          self, package, part_type,
        )
      }

      #[inline]
      pub fn add_embedded_control_persistence_binary_data_part_by_type_with_id<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::EmbeddedControlPersistenceBinaryDataPartType,
        relationship_id: impl Into<String>,
      ) -> Result<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_embedded_control_persistence_binary_data_part_by_type_with_id(
          self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_embedded_control_persistence_part_by_type<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::EmbeddedControlPersistencePartType,
      ) -> Result<
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_embedded_control_persistence_part_by_type(
          self, package, part_type,
        )
      }

      #[inline]
      pub fn add_embedded_control_persistence_part_by_type_with_id<P>(
        self,
        package: &mut P,
        part_type: crate::sdk::EmbeddedControlPersistencePartType,
        relationship_id: impl Into<String>,
      ) -> Result<
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
        crate::common::SdkError,
      >
      where
        P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      {
        <Self as crate::sdk::SdkPartHandle>::add_embedded_control_persistence_part_by_type_with_id(
          self,
          package,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn remove_relationship<P: crate::sdk::SdkPackage>(
        self,
        package: &mut P,
        relationship_id: &str,
      ) -> Option<crate::common::RelationshipInfo> {
        <Self as crate::sdk::SdkPartHandle>::remove_relationship(
          self,
          package,
          relationship_id,
        )
      }

      #[inline]
      pub fn change_relationship_id<P: crate::sdk::SdkPackage>(
        self,
        package: &mut P,
        relationship_id: &str,
        new_relationship_id: impl Into<String>,
      ) -> Result<(), crate::common::SdkError> {
        <Self as crate::sdk::SdkPartHandle>::change_relationship_id(
          self,
          package,
          relationship_id,
          new_relationship_id,
        )
      }
    }

    #root_method
    #child_methods
  })
}

fn part_handle_root_method_tokens(
  part_ident: &Ident,
  root_info: Option<&PartHandleRootInfo>,
) -> proc_macro2::TokenStream {
  let Some(root_info) = root_info else {
    return quote! {};
  };
  let root_ty = &root_info.root_ty;
  let root_accessor_ident = &root_info.accessor_ident;
  let root_accessor_mut_ident: Ident =
    parse_str(&format!("{root_accessor_ident}_mut")).expect("root accessor mut identifier");

  quote! {
    impl #part_ident {
      pub fn root_element<P: crate::parts::PartRootCache>(
        self,
        package: &mut P,
      ) -> Result<&#root_ty, crate::common::SdkError> {
        if package
          .root_element(self.id)
          .and_then(crate::parts::PartRootElement::#root_accessor_ident)
          .is_none()
        {
          let root_element = {
            let part = package.storage().part(self.id).ok_or_else(|| {
              crate::common::SdkError::CommonError(format!(
                "part id {:?} is not present in package storage",
                self.id,
              ))
            })?;
            #root_ty::from_bytes(part.data().bytes())?
          };

          *package.root_element_slot_mut(self.id).ok_or_else(|| {
            crate::common::SdkError::CommonError(format!(
              "part id {:?} is not present in package root cache",
              self.id,
            ))
          })? = Some(crate::parts::PartRootElement::#part_ident(Box::new(root_element)));
        }

        package
          .root_element(self.id)
          .and_then(crate::parts::PartRootElement::#root_accessor_ident)
          .ok_or_else(|| {
            crate::common::SdkError::CommonError(
              concat!("cached root element has unexpected type for ", stringify!(#part_ident))
                .to_string(),
            )
          })
      }

      pub fn root_element_mut<P: crate::parts::PartRootCache>(
        self,
        package: &mut P,
      ) -> Result<&mut #root_ty, crate::common::SdkError> {
        if package
          .root_element(self.id)
          .and_then(crate::parts::PartRootElement::#root_accessor_ident)
          .is_none()
        {
          let root_element = {
            let part = package.storage().part(self.id).ok_or_else(|| {
              crate::common::SdkError::CommonError(format!(
                "part id {:?} is not present in package storage",
                self.id,
              ))
            })?;
            #root_ty::from_bytes(part.data().bytes())?
          };

          *package.root_element_slot_mut(self.id).ok_or_else(|| {
            crate::common::SdkError::CommonError(format!(
              "part id {:?} is not present in package root cache",
              self.id,
            ))
          })? = Some(crate::parts::PartRootElement::#part_ident(Box::new(root_element)));
        }

        package
          .root_element_slot_mut(self.id)
          .and_then(Option::as_mut)
          .and_then(crate::parts::PartRootElement::#root_accessor_mut_ident)
          .ok_or_else(|| {
            crate::common::SdkError::CommonError(
              concat!("cached root element has unexpected type for ", stringify!(#part_ident))
                .to_string(),
            )
          })
      }

      pub fn set_root_element<P: crate::parts::PartRootCache>(
        self,
        package: &mut P,
        root_element: #root_ty,
      ) -> Result<(), crate::common::SdkError> {
        *package.root_element_slot_mut(self.id).ok_or_else(|| {
          crate::common::SdkError::CommonError(format!(
            "part id {:?} is not present in package root cache",
            self.id,
          ))
        })? = Some(crate::parts::PartRootElement::#part_ident(Box::new(root_element)));

        Ok(())
      }
    }
  }
}

fn part_handle_child_methods_tokens(
  part_ident: &Ident,
  child_infos: &[PartHandleChildInfo],
) -> proc_macro2::TokenStream {
  let child_branches = child_infos.iter().map(|child| {
    let part_ty = &child.part_ty;
    let variant_ident = &child.variant_ident;
    let relationship_type = &child.relationship_type;
    quote! {
      if crate::common::relationship_type_matches(relationship_type, #relationship_type) {
        return Some(crate::parts::PartRef::#variant_ident(
          <#part_ty as crate::sdk::SdkPartHandle>::from_part_id(part_id),
        ));
      }
    }
  });

  let accessors = child_infos.iter().map(|child| {
    let method_ident = &child.field_ident;
    let relationship_method_ident = relationship_accessor_ident(method_ident);
    let part_ty = &child.part_ty;
    let relationship_type = &child.relationship_type;
    let map_relationship = quote! {
      |relationship: &crate::common::RelationshipInfo| {
        if crate::common::relationship_type_matches(
          relationship.relationship_type(),
          #relationship_type,
        ) {
          relationship
            .target_part_id()
            .map(<#part_ty as crate::sdk::SdkPartHandle>::from_part_id)
        } else {
          None
        }
      }
    };

    match child.kind {
      PartChildKind::Repeated => quote! {
        pub fn #relationship_method_ident<'a, P: crate::sdk::SdkPackage>(
          self,
          package: &'a P,
        ) -> impl Iterator<Item = &'a crate::common::RelationshipInfo> + 'a {
          let _ = self.#method_ident;
          self.relationships_by_type(package, #relationship_type)
        }

        pub fn #method_ident<'a, P: crate::sdk::SdkPackage>(
          self,
          package: &'a P,
        ) -> impl Iterator<Item = #part_ty> + 'a {
          let _ = self.#method_ident;
          self
            .relationships(package)
            .into_iter()
            .flat_map(|relationships| relationships.iter())
            .filter_map(#map_relationship)
        }
      },
      PartChildKind::Required | PartChildKind::Optional => quote! {
        pub fn #relationship_method_ident<'a, P: crate::sdk::SdkPackage>(
          self,
          package: &'a P,
        ) -> impl Iterator<Item = &'a crate::common::RelationshipInfo> + 'a {
          let _ = self.#method_ident;
          self.relationships_by_type(package, #relationship_type)
        }

        pub fn #method_ident<P: crate::sdk::SdkPackage>(
          self,
          package: &P,
        ) -> Option<#part_ty> {
          let _ = self.#method_ident;
          self
            .relationships(package)
            .into_iter()
            .flat_map(|relationships| relationships.iter())
            .find_map(#map_relationship)
        }
      },
    }
  });

  let part_ref_from_relationship = if child_infos.is_empty() {
    quote! {
      fn part_ref_from_relationship(
        relationship: &crate::common::RelationshipInfo,
      ) -> Option<crate::parts::PartRef> {
        let part_id = relationship.target_part_id()?;
        Some(crate::parts::PartRef::ExtendedPart(
          <crate::parts::extended_part::ExtendedPart as crate::sdk::SdkPartHandle>::from_part_id(part_id),
        ))
      }
    }
  } else {
    quote! {
      fn part_ref_from_relationship(
        relationship: &crate::common::RelationshipInfo,
      ) -> Option<crate::parts::PartRef> {
        let part_id = relationship.target_part_id()?;
        let relationship_type = relationship.relationship_type();
        #( #child_branches )*
        Some(crate::parts::PartRef::ExtendedPart(
          <crate::parts::extended_part::ExtendedPart as crate::sdk::SdkPartHandle>::from_part_id(part_id),
        ))
      }
    }
  };

  quote! {
    impl #part_ident {
      pub fn parts<'a, P: crate::sdk::SdkPackage>(
        self,
        package: &'a P,
      ) -> impl Iterator<Item = crate::parts::IdPartPair<'a>> + 'a {
        self
          .relationships(package)
          .into_iter()
          .flat_map(|relationships| relationships.iter())
          .filter_map(|relationship| {
            let part = Self::part_ref_from_relationship(relationship)?;
            Some(crate::parts::IdPartPair::new(relationship.id(), part))
          })
      }

      #[inline]
      pub fn get_part_by_id<P: crate::sdk::SdkPackage>(
        self,
        package: &P,
        relationship_id: &str,
      ) -> Option<crate::parts::PartRef> {
        let relationship = self.relationships(package)?.get(relationship_id)?;
        Self::part_ref_from_relationship(relationship)
      }

      #[inline]
      pub fn try_get_part_by_id<P: crate::sdk::SdkPackage>(
        self,
        package: &P,
        relationship_id: &str,
      ) -> Option<crate::parts::PartRef> {
        self.get_part_by_id(package, relationship_id)
      }

      pub fn get_parts_of_type<P: crate::sdk::SdkPackage, T: crate::sdk::SdkPartHandle + 'static>(
        self,
        package: &P,
      ) -> impl Iterator<Item = T> + '_ {
        self.parts(package).filter_map(|entry| entry.part.downcast::<T>())
      }

      pub fn get_sub_part_of_type<
        P: crate::sdk::SdkPackage,
        T: crate::sdk::SdkPartHandle + 'static,
      >(
        self,
        package: &P,
      ) -> Option<T> {
        self.get_parts_of_type::<P, T>(package).next()
      }

      pub fn get_id_of_part<P: crate::sdk::SdkPackage, T: crate::sdk::SdkPartHandle>(
        self,
        package: &P,
        part: T,
      ) -> Option<&str> {
        let target_part_id = part.part_id();
        self.relationships(package)?.iter().find_map(|relationship| {
          (relationship.target_part_id() == Some(target_part_id)).then_some(relationship.id())
        })
      }

      #[inline]
      pub fn delete_part_by_id<P: crate::sdk::SdkPackage>(
        self,
        package: &mut P,
        relationship_id: &str,
      ) -> Result<bool, crate::common::SdkError> {
        <Self as crate::sdk::SdkPartHandle>::delete_part_by_id(
          self,
          package,
          relationship_id,
        )
      }

      #[inline]
      pub fn delete_part<P: crate::sdk::SdkPackage, T: crate::sdk::SdkPartHandle>(
        self,
        package: &mut P,
        part: T,
      ) -> Result<bool, crate::common::SdkError> {
        <Self as crate::sdk::SdkPartHandle>::delete_part(self, package, part)
      }

      #[inline]
      pub fn delete_parts<P, T, I>(
        self,
        package: &mut P,
        parts: I,
      ) -> Result<(), crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
        T: crate::sdk::SdkPartHandle,
        I: IntoIterator<Item = T>,
      {
        <Self as crate::sdk::SdkPartHandle>::delete_parts(self, package, parts)
      }

      #[inline]
      pub fn delete_parts_of_type<P, T>(
        self,
        package: &mut P,
      ) -> Result<(), crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
        T: crate::sdk::SdkPartHandle + 'static,
      {
        <Self as crate::sdk::SdkPartHandle>::delete_parts_of_type::<P, T>(
          self,
          package,
        )
      }

      #[inline]
      pub fn delete_parts_recursively_of_type<P, T>(
        self,
        package: &mut P,
      ) -> Result<(), crate::common::SdkError>
      where
        P: crate::sdk::SdkPackage,
        T: crate::sdk::SdkPartHandle + 'static,
      {
        <Self as crate::sdk::SdkPartHandle>::delete_parts_recursively_of_type::<P, T>(
          self,
          package,
        )
      }

      #part_ref_from_relationship

      #( #accessors )*
    }
  }
}

fn relationship_accessor_ident(field_ident: &Ident) -> Ident {
  Ident::new(
    &format!("{}_relationships", field_ident),
    field_ident.span(),
  )
}

fn marker_inner_type(ty: &Type, marker: &str) -> Option<Type> {
  let Type::Path(type_path) = ty else {
    return None;
  };
  let segment = type_path.path.segments.last()?;
  if segment.ident != marker {
    return None;
  }
  let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
    return None;
  };
  let syn::GenericArgument::Type(inner) = args.args.first()? else {
    return None;
  };
  Some(inner.clone())
}

fn part_child_marker_info(ty: &Type) -> Option<PartChildMarkerInfo> {
  let Type::Path(type_path) = ty else {
    return None;
  };
  let segment = type_path.path.segments.last()?;
  let marker_name = segment.ident.to_string();
  let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
    return None;
  };
  let syn::GenericArgument::Type(part_ty) = args.args.first()? else {
    return None;
  };

  let kind = match marker_name.as_str() {
    "OptionalPart" => PartChildKind::Optional,
    "RequiredPart" => PartChildKind::Required,
    "RepeatedPart" => PartChildKind::Repeated,
    "PartChild" => {
      let syn::GenericArgument::Type(kind_ty) = args.args.iter().nth(1)? else {
        return None;
      };
      part_child_kind_from_type(kind_ty)?
    }
    _ => return None,
  };

  Some(PartChildMarkerInfo {
    part_ty: part_ty.clone(),
    kind,
  })
}

fn part_child_kind_from_type(ty: &Type) -> Option<PartChildKind> {
  let Type::Path(type_path) = ty else {
    return None;
  };
  let segment = type_path.path.segments.last()?;
  match segment.ident.to_string().as_str() {
    "OptionalPartKind" => Some(PartChildKind::Optional),
    "RequiredPartKind" => Some(PartChildKind::Required),
    "RepeatedPartKind" => Some(PartChildKind::Repeated),
    _ => None,
  }
}

fn part_ref_variant_ident(part_ty: &Type) -> syn::Result<Ident> {
  let Type::Path(TypePath { path, .. }) = part_ty else {
    return Err(syn::Error::new_spanned(
      part_ty,
      "PartChild marker inner type must be a path",
    ));
  };
  path
    .segments
    .last()
    .map(|segment| segment.ident.clone())
    .ok_or_else(|| syn::Error::new_spanned(part_ty, "PartChild marker inner type is empty"))
}

fn parse_part_root_accessor(attrs: &[Attribute], ident: &Ident) -> syn::Result<Ident> {
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in metas {
      if let Meta::List(meta) = meta
        && meta.path.is_ident("part_root")
      {
        let mut accessor = None;
        meta.parse_nested_meta(|nested| {
          if nested.path.is_ident("accessor") {
            let value: LitStr = nested.value()?.parse()?;
            accessor = Some(value.value());
            Ok(())
          } else {
            Err(nested.error("unsupported sdk part_root attribute"))
          }
        })?;
        let Some(accessor) = accessor else {
          return Err(syn::Error::new_spanned(
            meta,
            "sdk part_root requires accessor",
          ));
        };
        return parse_str(&accessor);
      }
    }
  }

  parse_str(&format!("as_{}", simple_snake_case(&ident.to_string())))
}

fn simple_snake_case(value: &str) -> String {
  let mut out = String::new();
  for (index, ch) in value.chars().enumerate() {
    if ch.is_uppercase() {
      if index > 0 {
        out.push('_');
      }
      out.extend(ch.to_lowercase());
    } else {
      out.push(ch);
    }
  }
  out
}
