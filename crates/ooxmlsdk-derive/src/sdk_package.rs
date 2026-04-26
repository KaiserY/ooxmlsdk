use super::*;

struct PackageChildInfo {
  attrs: Vec<Attribute>,
  field_ident: Ident,
  part_ty: Type,
  kind: PartChildKind,
  relationship_type: String,
  main_accessor_ident: Option<Ident>,
}

struct PartChildMarkerInfo {
  part_ty: Type,
  kind: PartChildKind,
}

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

fn is_relationship_model_field(ident: &Ident) -> bool {
  ident == "fallback_parts"
    || ident == "relationship_order"
    || ident == "data_part_reference_relationships"
    || ident == "reference_relationships"
    || ident == "raw_relationships"
}

fn package_child_init_tokens(
  child: &PackageChildInfo,
  field_index: usize,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
  let field_ident = &child.field_ident;
  let part_ty = &child.part_ty;
  let field_index = field_index as u16;
  match child.kind {
    PartChildKind::Repeated | PartChildKind::RequiredRepeated => (
      quote! {
        let mut #field_ident: Vec<#part_ty> = Vec::new();
      },
      quote! {
        if let Some(child_part_id) = relationship.target_part_id() {
          let item_index = #field_ident.len();
          #field_ident.push(
            <#part_ty as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
              &storage,
              relationship.id(),
              child_part_id,
            ),
          );
          relationship_order.push(crate::sdk::RelationshipModelEntry::Child {
            field_index: #field_index,
            item_index,
          });
          represented_relationship = true;
        }
      },
    ),
    PartChildKind::Required | PartChildKind::Optional => (
      quote! {
        let mut #field_ident: Option<Box<#part_ty>> = None;
      },
      quote! {
        if #field_ident.is_none() {
          if let Some(child_part_id) = relationship.target_part_id() {
            #field_ident = Some(Box::new(
              <#part_ty as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                &storage,
                relationship.id(),
                child_part_id,
              ),
            ));
            relationship_order.push(crate::sdk::RelationshipModelEntry::Child {
              field_index: #field_index,
              item_index: 0,
            });
            represented_relationship = true;
          }
        }
      },
    ),
  }
}

fn package_relationship_dispatch_tokens(
  child_infos: &[PackageChildInfo],
) -> proc_macro2::TokenStream {
  use std::collections::BTreeMap;

  let mut exact_groups: BTreeMap<&str, Vec<proc_macro2::TokenStream>> = BTreeMap::new();
  let mut alias_branches = Vec::new();
  for (field_index, child) in child_infos.iter().enumerate() {
    let relationship_type = child.relationship_type.as_str();
    let (_, load_tokens) = package_child_init_tokens(child, field_index);
    exact_groups
      .entry(relationship_type)
      .or_default()
      .push(load_tokens.clone());
    if explicit_relationship_type_may_have_alias(relationship_type) {
      alias_branches.push((
        quote! {
          crate::common::relationship_type_matches_alias(
            relationship_type,
            #relationship_type,
          )
        },
        load_tokens,
      ));
    }
  }

  let exact_arms = exact_groups.iter().map(|(relationship_type, loads)| {
    quote! {
      #relationship_type => {
        #( #loads )*
      }
    }
  });
  let alias_fallback = if alias_branches.is_empty() {
    quote! {}
  } else {
    build_conditional_chain(&alias_branches, quote! {})
  };

  quote! {
    match relationship_type {
      #( #exact_arms )*
      _ => {
        #alias_fallback
      }
    }
  }
}

fn package_child_descriptors_tokens(child_infos: &[PackageChildInfo]) -> proc_macro2::TokenStream {
  if child_infos.is_empty() {
    return quote! {};
  }

  let descriptors = child_infos.iter().map(|child| {
    let field_name = child.field_ident.to_string();
    let relationship_type = child.relationship_type.as_str();
    let child_part_type = part_child_type_name(&child.part_ty);
    let cardinality = part_child_cardinality_tokens(child.kind);
    quote! {
      crate::sdk::PartChildDescriptor::new(
        #field_name,
        #relationship_type,
        #child_part_type,
        #cardinality,
      )
    }
  });

  quote! {
    const CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] = &[
      #( #descriptors, )*
    ];
  }
}

pub(crate) fn expand_sdk_package(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let Data::Struct(data_struct) = &input.data else {
    return Err(syn::Error::new_spanned(
      input,
      "SdkPackage can only be derived for structs",
    ));
  };

  let Fields::Named(fields) = &data_struct.fields else {
    return Err(syn::Error::new_spanned(
      input,
      "SdkPackage can only be derived for named-field structs",
    ));
  };

  let storage_field = fields
    .named
    .iter()
    .find(|field| field.ident.as_ref().is_some_and(|ident| ident == "storage"))
    .ok_or_else(|| syn::Error::new_spanned(input, "SdkPackage requires a `storage` field"))?;
  let storage_ident = storage_field.ident.as_ref().unwrap();

  let main_part_id_field = fields
    .named
    .iter()
    .find(|field| {
      field
        .ident
        .as_ref()
        .is_some_and(|ident| ident == "main_part_id")
    })
    .ok_or_else(|| syn::Error::new_spanned(input, "SdkPackage requires a `main_part_id` field"))?;
  let main_part_id_ident = main_part_id_field.ident.as_ref().unwrap();
  let root_elements_ident = fields.named.iter().find_map(|field| {
    let ident = field.ident.as_ref()?;
    (ident == "root_elements").then_some(ident)
  });
  let root_elements_local = root_elements_ident.map(|_| {
    quote! {
      let root_elements = crate::parts::initialize_root_elements(&storage, open_mode)?;
    }
  });
  let root_elements_init = root_elements_ident.map(|ident| {
    quote! {
      #ident: root_elements,
    }
  });
  let mut child_infos = Vec::new();
  for field in &fields.named {
    let Some(field_ident) = field.ident.as_ref() else {
      continue;
    };
    if field_ident == "storage"
      || field_ident == "main_part_id"
      || field_ident == "root_elements"
      || is_relationship_model_field(field_ident)
    {
      continue;
    }

    let Some(marker) = part_child_field_info(&field.ty) else {
      return Err(syn::Error::new_spanned(
        field,
        "SdkPackage fields require storage, main_part_id, root_elements, or PartChild markers",
      ));
    };
    let Some(relationship_type) = parse_part_child_relationship_type_attr(&field.attrs)? else {
      return Err(syn::Error::new_spanned(
        field,
        "PartChild marker field requires #[sdk(part_child(relationship_type = ...))]",
      ));
    };
    child_infos.push(PackageChildInfo {
      attrs: passthrough_attrs(&field.attrs),
      field_ident: field_ident.clone(),
      part_ty: marker.part_ty,
      kind: parse_part_child_kind_attr(&field.attrs)?.unwrap_or(marker.kind),
      relationship_type,
      main_accessor_ident: parse_package_main_accessor(&field.attrs)?,
    });
  }
  let main_child = child_infos
    .iter()
    .find(|child| child.main_accessor_ident.is_some());
  let main_part_relationship_type = main_child.map(|child| {
    let part_ty = &child.part_ty;
    quote! {
      pub const MAIN_PART_RELATIONSHIP_TYPE: &'static str =
        <#part_ty as crate::sdk::SdkPartHandle>::RELATIONSHIP_TYPE;
    }
  });
  let main_part_method = main_child.map(|child| {
    let attrs = &child.attrs;
    let field_ident = &child.field_ident;
    let part_ty = &child.part_ty;
    let accessor_ident = child.main_accessor_ident.as_ref().unwrap();
    let add_accessor_ident: Ident =
      parse_str(&format!("add_{accessor_ident}")).expect("main part add accessor identifier");
    quote! {
      #( #attrs )*
      pub fn #accessor_ident(&self) -> Result<#part_ty, crate::common::SdkError> {
        self.#field_ident.as_deref().cloned().ok_or_else(|| {
            crate::common::SdkError::CommonError(
              concat!("missing main part for ", stringify!(#ident)).to_string(),
            )
          })
      }

      #( #attrs )*
      pub fn #add_accessor_ident(&mut self) -> Result<#part_ty, crate::common::SdkError> {
        let _ = self.#field_ident;
        if self.#main_part_id_ident.is_some() {
          return Err(crate::common::SdkError::CommonError(
            concat!("main part already exists for ", stringify!(#ident)).to_string(),
          ));
        }

        let relationship_id = self.relationships().next_relationship_id();
        let part = if let Some(content_type) = crate::sdk::default_main_part_content_type::<#part_ty>() {
          crate::sdk::SdkPackage::add_new_part_with_content_type_and_extension::<#part_ty>(
            self,
            relationship_id.clone(),
            content_type,
            <#part_ty as crate::sdk::SdkPartHandle>::EXTENSION,
            crate::common::NewPartTargetMode::Fixed,
          )?
        } else {
          crate::sdk::SdkPackage::add_new_part_with_target_mode::<#part_ty>(
            self,
            relationship_id.clone(),
            crate::common::NewPartTargetMode::Fixed,
          )?
        };
        self.#main_part_id_ident = Some(part.part_id());
        self.#field_ident = Some(Box::new(
          <#part_ty as crate::sdk::SdkPartHandle>::from_relationship_id(
            relationship_id,
            part.part_id(),
          ),
        ));
        Ok(part)
      }
    }
  });
  let main_relationship_expr = if main_child.is_some() {
    quote! {
      storage
        .package_relationships()
        .first_target_part_by_relationship_type(Self::MAIN_PART_RELATIONSHIP_TYPE)
    }
  } else {
    quote! { None }
  };
  let package_relationship_methods = package_relationship_method_tokens(&child_infos);
  let root_cache_impl = root_elements_ident.map(|root_elements_ident| {
    quote! {
      impl crate::parts::PartRootCache for #ident {
        #[inline]
        fn root_element(
          &self,
          part_id: crate::common::PartId,
        ) -> Option<&crate::parts::PartRootElement> {
          self
            .#root_elements_ident
            .get(part_id.index())
            .and_then(Option::as_ref)
        }

        #[inline]
        fn root_element_slot_mut(
          &mut self,
          part_id: crate::common::PartId,
        ) -> Option<&mut Option<crate::parts::PartRootElement>> {
          self.#root_elements_ident.get_mut(part_id.index())
        }

        #[inline]
        fn push_root_element_slot(&mut self) {
          self.#root_elements_ident.push(None);
        }
      }
    }
  });
  let child_field_locals: Vec<_> = child_infos
    .iter()
    .enumerate()
    .map(|(field_index, child)| package_child_init_tokens(child, field_index).0)
    .collect();
  let child_field_inits = child_infos.iter().map(|child| {
    let field_ident = &child.field_ident;
    quote! {
      #field_ident,
    }
  });
  let child_field_tuple_values: Vec<_> = child_infos
    .iter()
    .map(|child| {
      let field_ident = &child.field_ident;
      quote! { #field_ident }
    })
    .collect();
  let child_field_assigns: Vec<_> = child_infos
    .iter()
    .map(|child| {
      let field_ident = &child.field_ident;
      quote! {
        self.#field_ident = #field_ident;
      }
    })
    .collect();
  let relationship_dispatch = package_relationship_dispatch_tokens(&child_infos);
  let relationship_model_field_locals = quote! {
    let mut fallback_parts = Vec::new();
    let mut relationship_order: Vec<crate::sdk::RelationshipModelEntry> = Vec::new();
    let mut data_part_reference_relationships = Vec::new();
    let mut reference_relationships = Vec::new();
    let mut raw_relationships = Vec::new();
    for relationship in storage.package_relationships().iter() {
      let mut represented_relationship = false;
      let relationship_type = relationship.relationship_type();
      #relationship_dispatch
      if relationship.is_reference_relationship() {
        if relationship.reference_kind().is_some_and(|kind| {
          matches!(
            kind,
            crate::common::ReferenceRelationshipKind::Audio
              | crate::common::ReferenceRelationshipKind::Media
              | crate::common::ReferenceRelationshipKind::Video
          )
        }) {
          let item_index = data_part_reference_relationships.len();
          data_part_reference_relationships.push(relationship.clone());
          relationship_order.push(crate::sdk::RelationshipModelEntry::DataPartReference(item_index));
        } else {
          let item_index = reference_relationships.len();
          reference_relationships.push(relationship.clone());
          relationship_order.push(crate::sdk::RelationshipModelEntry::Reference(item_index));
        }
      } else if relationship.target_kind() == crate::common::RelationshipTargetKind::InternalPart {
        if !represented_relationship {
          if let Some(part) = crate::parts::PartRef::from_relationship_storage(&storage, relationship) {
            let item_index = fallback_parts.len();
            fallback_parts.push(part);
            relationship_order.push(crate::sdk::RelationshipModelEntry::Fallback(item_index));
          }
        }
      } else {
        let item_index = raw_relationships.len();
        raw_relationships.push(relationship.clone());
        relationship_order.push(crate::sdk::RelationshipModelEntry::Raw(item_index));
      }
    }
  };
  let package_modeled_child_stmts: Vec<_> = child_infos.iter().map(|child| {
    let field_ident = &child.field_ident;
    match child.kind {
      PartChildKind::Repeated | PartChildKind::RequiredRepeated => quote! {
        for part in &self.#field_ident {
          crate::sdk::add_part_handle_to_relationship_set(&mut relationships, &self.#storage_ident, None, part)?;
        }
      },
      PartChildKind::Required | PartChildKind::Optional => quote! {
        if let Some(part) = self.#field_ident.as_deref() {
          crate::sdk::add_part_handle_to_relationship_set(&mut relationships, &self.#storage_ident, None, part)?;
        }
      },
    }
  }).collect();
  let package_ordered_child_arms = child_infos.iter().enumerate().map(|(field_index, child)| {
    let field_index = field_index as u16;
    let field_ident = &child.field_ident;
    match child.kind {
      PartChildKind::Repeated | PartChildKind::RequiredRepeated => quote! {
        #field_index => {
          if let Some(part) = self.#field_ident.get(*item_index) {
            crate::sdk::add_part_handle_to_relationship_set(
              &mut relationships,
              &self.#storage_ident,
              None,
              part,
            )?;
          }
        }
      },
      PartChildKind::Required | PartChildKind::Optional => quote! {
        #field_index => {
          if *item_index == 0 {
            if let Some(part) = self.#field_ident.as_deref() {
              crate::sdk::add_part_handle_to_relationship_set(
                &mut relationships,
                &self.#storage_ident,
                None,
                part,
              )?;
            }
          }
        }
      },
    }
  });
  let package_unordered_child_stmts = package_modeled_child_stmts.iter();
  let package_collect_relationship_child_stmts = child_infos.iter().map(|child| {
    let field_ident = &child.field_ident;
    match child.kind {
      PartChildKind::Repeated | PartChildKind::RequiredRepeated => quote! {
        for part in &self.#field_ident {
          crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, self, relationships)?;
        }
      },
      PartChildKind::Required | PartChildKind::Optional => quote! {
        if let Some(part) = self.#field_ident.as_deref() {
          crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, self, relationships)?;
        }
      },
    }
  });
  let child_descriptors_assoc = package_child_descriptors_tokens(&child_infos);
  Ok(quote! {
    impl crate::sdk::SdkPackageInternal for #ident {
      #child_descriptors_assoc

      #[inline]
      fn storage(&self) -> &crate::common::SdkPackageStorage {
        &self.#storage_ident
      }

      #[inline]
      fn storage_mut(&mut self) -> &mut crate::common::SdkPackageStorage {
        &mut self.#storage_ident
      }

      fn modeled_relationships(
        &self,
      ) -> Result<crate::common::RelationshipSet, crate::common::SdkError> {
        let mut relationships = crate::common::RelationshipSet::default();
        if self.relationship_order.is_empty() {
          #( #package_unordered_child_stmts )*
          for part in &self.fallback_parts {
            crate::sdk::add_part_ref_to_relationship_set(&mut relationships, &self.#storage_ident, None, part)?;
          }
          for relationship in self
            .data_part_reference_relationships
            .iter()
            .chain(self.reference_relationships.iter())
            .chain(self.raw_relationships.iter())
          {
            relationships.add_relationship_info(relationship.clone())?;
          }
          return Ok(relationships);
        }

        for entry in &self.relationship_order {
          match entry {
            crate::sdk::RelationshipModelEntry::Child { field_index, item_index } => {
              match field_index {
                #( #package_ordered_child_arms, )*
                _ => {}
              }
            }
            crate::sdk::RelationshipModelEntry::Fallback(item_index) => {
              if let Some(part) = self.fallback_parts.get(*item_index) {
                crate::sdk::add_part_ref_to_relationship_set(
                  &mut relationships,
                  &self.#storage_ident,
                  None,
                  part,
                )?;
              }
            }
            crate::sdk::RelationshipModelEntry::DataPartReference(item_index) => {
              if let Some(relationship) = self.data_part_reference_relationships.get(*item_index) {
                relationships.add_relationship_info(relationship.clone())?;
              }
            }
            crate::sdk::RelationshipModelEntry::Reference(item_index) => {
              if let Some(relationship) = self.reference_relationships.get(*item_index) {
                relationships.add_relationship_info(relationship.clone())?;
              }
            }
            crate::sdk::RelationshipModelEntry::Raw(item_index) => {
              if let Some(relationship) = self.raw_relationships.get(*item_index) {
                relationships.add_relationship_info(relationship.clone())?;
              }
            }
          }
        }
        Ok(relationships)
      }

      fn refresh_relationship_model_from_storage(&mut self) {
        let (
          #( #child_field_tuple_values, )*
          fallback_parts,
          relationship_order,
          data_part_reference_relationships,
          reference_relationships,
          raw_relationships,
        ) = {
          let storage = &self.#storage_ident;
          #( #child_field_locals )*
          #relationship_model_field_locals
          (
            #( #child_field_tuple_values, )*
            fallback_parts,
            relationship_order,
            data_part_reference_relationships,
            reference_relationships,
            raw_relationships,
          )
        };

        #( #child_field_assigns )*
        self.fallback_parts = fallback_parts;
        self.relationship_order = relationship_order;
        self.data_part_reference_relationships = data_part_reference_relationships;
        self.reference_relationships = reference_relationships;
        self.raw_relationships = raw_relationships;
      }

      fn collect_modeled_part_relationships(
        &self,
        relationships: &mut std::collections::HashMap<
          crate::common::PartId,
          crate::common::RelationshipSet,
        >,
      ) -> Result<(), crate::common::SdkError> {
        #( #package_collect_relationship_child_stmts )*
        for part in &self.fallback_parts {
          part.collect_modeled_part_relationships(self, relationships)?;
        }
        Ok(())
      }

    }

    impl crate::sdk::SdkPackage for #ident {}

    impl #ident {
      #main_part_relationship_type

      pub fn new<R: std::io::Read + std::io::Seek>(
        reader: R,
      ) -> Result<Self, crate::common::SdkError> {
        Self::new_inner(reader, crate::common::PackageOpenMode::Eager)
      }

      pub fn new_lazy<R: std::io::Read + std::io::Seek>(
        reader: R,
      ) -> Result<Self, crate::common::SdkError> {
        Self::new_inner(reader, crate::common::PackageOpenMode::Lazy)
      }

      pub fn new_from_file<P: AsRef<std::path::Path>>(
        path: P,
      ) -> Result<Self, crate::common::SdkError> {
        Self::new(std::io::BufReader::new(std::fs::File::open(path)?))
      }

      pub fn new_from_file_lazy<P: AsRef<std::path::Path>>(
        path: P,
      ) -> Result<Self, crate::common::SdkError> {
        Self::new_lazy(std::io::BufReader::new(std::fs::File::open(path)?))
      }

      pub(crate) fn relationships(&self) -> crate::common::RelationshipView {
        self
          .try_relationships()
          .expect("package relationships could not be modeled")
      }

      pub(crate) fn try_relationships(
        &self,
      ) -> Result<crate::common::RelationshipView, crate::common::SdkError> {
        crate::sdk::SdkPackageInternal::modeled_relationships(self).map(Into::into)
      }

      #[inline]
      pub fn add_external_relationship(
        &mut self,
        relationship_id: impl Into<String>,
        relationship_type: impl Into<String>,
        target: impl Into<String>,
      ) -> Result<&crate::common::RelationshipInfo, crate::common::SdkError> {
        crate::sdk::SdkPackage::add_external_relationship(
          self,
          relationship_id,
          relationship_type,
          target,
        )
      }

      #[inline]
      pub fn add_external_relationship_auto_id(
        &mut self,
        relationship_type: impl Into<String>,
        target: impl Into<String>,
      ) -> Result<&crate::common::RelationshipInfo, crate::common::SdkError> {
        crate::sdk::SdkPackage::add_external_relationship_auto_id(
          self,
          relationship_type,
          target,
        )
      }

      #[inline]
      pub fn add_hyperlink_relationship(
        &mut self,
        relationship_id: impl Into<String>,
        target: impl Into<String>,
      ) -> Result<&crate::common::RelationshipInfo, crate::common::SdkError> {
        crate::sdk::SdkPackage::add_hyperlink_relationship(self, relationship_id, target)
      }

      #[inline]
      pub fn add_hyperlink_relationship_with_mode(
        &mut self,
        relationship_id: impl Into<String>,
        target: impl Into<String>,
        target_mode: crate::schemas::opc_relationships::TargetMode,
      ) -> Result<&crate::common::RelationshipInfo, crate::common::SdkError> {
        crate::sdk::SdkPackage::add_hyperlink_relationship_with_mode(
          self,
          relationship_id,
          target,
          target_mode,
        )
      }

      #[inline]
      pub fn add_hyperlink_relationship_auto_id(
        &mut self,
        target: impl Into<String>,
        target_mode: crate::schemas::opc_relationships::TargetMode,
      ) -> Result<&crate::common::RelationshipInfo, crate::common::SdkError> {
        crate::sdk::SdkPackage::add_hyperlink_relationship_auto_id(
          self,
          target,
          target_mode,
        )
      }

      #[inline]
      pub fn remove_relationship(
        &mut self,
        relationship_id: &str,
      ) -> Option<crate::common::RelationshipInfo> {
        crate::sdk::SdkPackage::remove_relationship(self, relationship_id)
      }

      #[inline]
      pub fn get_reference_relationship(
        &self,
        relationship_id: &str,
      ) -> Option<&crate::common::RelationshipInfo> {
        crate::sdk::SdkPackage::get_reference_relationship(self, relationship_id)
      }

      #[inline]
      pub fn get_external_relationship(
        &self,
        relationship_id: &str,
      ) -> Option<&crate::common::RelationshipInfo> {
        crate::sdk::SdkPackage::get_external_relationship(self, relationship_id)
      }

      #[inline]
      pub fn get_hyperlink_relationship(
        &self,
        relationship_id: &str,
      ) -> Option<&crate::common::RelationshipInfo> {
        crate::sdk::SdkPackage::get_hyperlink_relationship(self, relationship_id)
      }

      #[inline]
      pub fn delete_reference_relationship(
        &mut self,
        relationship_id: &str,
      ) -> Result<crate::common::RelationshipInfo, crate::common::SdkError> {
        crate::sdk::SdkPackage::delete_reference_relationship(self, relationship_id)
      }

      #[inline]
      pub fn delete_external_relationship(
        &mut self,
        relationship_id: &str,
      ) -> Result<crate::common::RelationshipInfo, crate::common::SdkError> {
        crate::sdk::SdkPackage::delete_external_relationship(self, relationship_id)
      }

      #[inline]
      pub fn change_relationship_id(
        &mut self,
        relationship_id: &str,
        new_relationship_id: impl Into<String>,
      ) -> Result<(), crate::common::SdkError> {
        crate::sdk::SdkPackage::change_relationship_id(
          self,
          relationship_id,
          new_relationship_id,
        )
      }

      #[inline]
      pub fn external_relationships(&self) -> impl Iterator<Item = &crate::common::RelationshipInfo> {
        crate::sdk::SdkPackage::external_relationships(self)
      }

      #[inline]
      pub fn hyperlink_relationships(&self) -> impl Iterator<Item = &crate::common::RelationshipInfo> {
        crate::sdk::SdkPackage::hyperlink_relationships(self)
      }

      #[inline]
      pub fn data_part_reference_relationships(
        &self,
      ) -> impl Iterator<Item = &crate::common::RelationshipInfo> {
        crate::sdk::SdkPackage::data_part_reference_relationships(self)
      }

      #[inline]
      pub fn media_data_parts(&self) -> impl Iterator<Item = crate::common::MediaDataPart> + '_ {
        crate::sdk::SdkPackage::media_data_parts(self)
      }

      #[inline]
      pub fn delete_unused_media_data_parts(&mut self) -> usize {
        crate::sdk::SdkPackage::delete_unused_media_data_parts(self)
      }

      #[inline]
      pub fn add_new_part<T: crate::sdk::SdkPartHandle>(
        &mut self,
        relationship_id: impl Into<String>,
      ) -> Result<T, crate::common::SdkError>
      where
        Self: crate::parts::PartRootCache,
      {
        crate::sdk::SdkPackage::add_new_part(self, relationship_id)
      }

      #[inline]
      pub fn add_new_part_auto_id<T: crate::sdk::SdkPartHandle>(
        &mut self,
      ) -> Result<T, crate::common::SdkError>
      where
        Self: crate::parts::PartRootCache,
      {
        crate::sdk::SdkPackage::add_new_part_auto_id(self)
      }

      #[inline]
      pub fn add_new_part_with_content_type<T: crate::sdk::SdkPartHandle>(
        &mut self,
        relationship_id: impl Into<String>,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<T, crate::common::SdkError>
      where
        Self: crate::parts::PartRootCache,
      {
        crate::sdk::SdkPackage::add_new_part_with_content_type(
          self,
          relationship_id,
          content_type,
        )
      }

      #[inline]
      pub fn add_new_part_with_content_type_auto_id<T: crate::sdk::SdkPartHandle>(
        &mut self,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<T, crate::common::SdkError>
      where
        Self: crate::parts::PartRootCache,
      {
        crate::sdk::SdkPackage::add_new_part_with_content_type_auto_id(self, content_type)
      }

      #[inline]
      pub fn add_new_part_with_content_type_and_extension<T: crate::sdk::SdkPartHandle>(
        &mut self,
        relationship_id: impl Into<String>,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        extension: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<T, crate::common::SdkError>
      where
        Self: crate::parts::PartRootCache,
      {
        crate::sdk::SdkPackage::add_new_part_with_content_type_and_extension(
          self,
          relationship_id,
          content_type,
          extension,
          crate::common::NewPartTargetMode::Indexed,
        )
      }

      #[inline]
      pub fn add_new_part_with_content_type_and_extension_auto_id<T: crate::sdk::SdkPartHandle>(
        &mut self,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        extension: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<T, crate::common::SdkError>
      where
        Self: crate::parts::PartRootCache,
      {
        let relationship_id = crate::sdk::SdkPackageInternal::relationships(self).next_relationship_id();
        crate::sdk::SdkPackage::add_new_part_with_content_type_and_extension(
          self,
          relationship_id,
          content_type,
          extension,
          crate::common::NewPartTargetMode::Indexed,
        )
      }

      #[inline]
      pub fn add_core_file_properties_part(
        &mut self,
      ) -> Result<crate::parts::core_file_properties_part::CoreFilePropertiesPart, crate::common::SdkError>
      where
        Self: crate::parts::PartRootCache,
      {
        crate::sdk::SdkPackage::add_core_file_properties_part(self)
      }

      #[inline]
      pub fn add_extended_file_properties_part(
        &mut self,
      ) -> Result<
        crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart,
        crate::common::SdkError,
      >
      where
        Self: crate::parts::PartRootCache,
      {
        crate::sdk::SdkPackage::add_extended_file_properties_part(self)
      }

      #[inline]
      pub fn add_custom_file_properties_part(
        &mut self,
      ) -> Result<
        crate::parts::custom_file_properties_part::CustomFilePropertiesPart,
        crate::common::SdkError,
      >
      where
        Self: crate::parts::PartRootCache,
      {
        crate::sdk::SdkPackage::add_custom_file_properties_part(self)
      }

      #[inline]
      pub fn add_digital_signature_origin_part(
        &mut self,
      ) -> Result<
        crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart,
        crate::common::SdkError,
      >
      where
        Self: crate::parts::PartRootCache,
      {
        crate::sdk::SdkPackage::add_digital_signature_origin_part(self)
      }

      #[inline]
      pub fn add_thumbnail_part(
        &mut self,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<crate::parts::thumbnail_part::ThumbnailPart, crate::common::SdkError>
      where
        Self: crate::parts::PartRootCache,
      {
        crate::sdk::SdkPackage::add_thumbnail_part(self, content_type)
      }

      #[inline]
      pub fn add_thumbnail_part_with_id(
        &mut self,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::thumbnail_part::ThumbnailPart, crate::common::SdkError>
      where
        Self: crate::parts::PartRootCache,
      {
        crate::sdk::SdkPackage::add_thumbnail_part_with_id(
          self,
          content_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_thumbnail_part_by_type(
        &mut self,
        part_type: crate::sdk::ThumbnailPartType,
      ) -> Result<crate::parts::thumbnail_part::ThumbnailPart, crate::common::SdkError>
      where
        Self: crate::parts::PartRootCache,
      {
        crate::sdk::SdkPackage::add_thumbnail_part_by_type(self, part_type)
      }

      #[inline]
      pub fn add_thumbnail_part_by_type_with_id(
        &mut self,
        part_type: crate::sdk::ThumbnailPartType,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::thumbnail_part::ThumbnailPart, crate::common::SdkError>
      where
        Self: crate::parts::PartRootCache,
      {
        crate::sdk::SdkPackage::add_thumbnail_part_by_type_with_id(
          self,
          part_type,
          relationship_id,
        )
      }

      #[inline]
      pub fn add_extended_part(
        &mut self,
        relationship_type: impl Into<String>,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        target_extension: impl Into<std::borrow::Cow<'static, str>>,
      ) -> Result<crate::parts::extended_part::ExtendedPart, crate::common::SdkError>
      where
        Self: crate::parts::PartRootCache,
      {
        crate::sdk::SdkPackage::add_extended_part(
          self,
          relationship_type,
          content_type,
          target_extension,
        )
      }

      #[inline]
      pub fn add_extended_part_with_id(
        &mut self,
        relationship_type: impl Into<String>,
        content_type: impl Into<std::borrow::Cow<'static, str>>,
        target_extension: impl Into<std::borrow::Cow<'static, str>>,
        relationship_id: impl Into<String>,
      ) -> Result<crate::parts::extended_part::ExtendedPart, crate::common::SdkError>
      where
        Self: crate::parts::PartRootCache,
      {
        crate::sdk::SdkPackage::add_extended_part_with_id(
          self,
          relationship_type,
          content_type,
          target_extension,
          relationship_id,
        )
      }

      pub fn save<W: std::io::Write + std::io::Seek>(
        &self,
        writer: W,
      ) -> Result<(), crate::common::SdkError> {
        crate::parts::save_package(self, writer)
      }

      pub fn copy_to<W: std::io::Write + std::io::Seek>(
        &self,
        writer: W,
      ) -> Result<(), crate::common::SdkError> {
        crate::parts::save_package(self, writer)
      }

      pub fn to_package_bytes(&self) -> Result<Vec<u8>, crate::common::SdkError> {
        let mut buffer = std::io::Cursor::new(Vec::new());
        self.copy_to(&mut buffer)?;
        Ok(buffer.into_inner())
      }

      pub fn to_owned_package(&self) -> Result<Self, crate::common::SdkError> {
        Self::new(std::io::Cursor::new(self.to_package_bytes()?))
      }

      pub fn save_as_file<P: AsRef<std::path::Path>>(
        &self,
        path: P,
      ) -> Result<(), crate::common::SdkError> {
        self.copy_to(std::io::BufWriter::new(std::fs::File::create(path)?))
      }

      #main_part_method

      #package_relationship_methods

      fn new_inner<R: std::io::Read + std::io::Seek>(
        reader: R,
        open_mode: crate::common::PackageOpenMode,
      ) -> Result<Self, crate::common::SdkError> {
        let storage = crate::common::SdkPackageStorage::open(reader, open_mode)?;
        let main_part_id = #main_relationship_expr;
        #root_elements_local
        #( #child_field_locals )*
        #relationship_model_field_locals

        Ok(Self {
          #main_part_id_ident: main_part_id,
          #root_elements_init
        #( #child_field_inits )*
        fallback_parts,
        relationship_order,
        data_part_reference_relationships,
          reference_relationships,
          raw_relationships,
          #storage_ident: storage,
        })
      }
    }

    #root_cache_impl
  })
}

fn package_relationship_method_tokens(
  child_infos: &[PackageChildInfo],
) -> proc_macro2::TokenStream {
  let accessors = child_infos
    .iter()
    .filter(|child| child.main_accessor_ident.is_none())
    .map(|child| {
      let attrs = &child.attrs;
      let field_ident = &child.field_ident;
      let method_ident = &child.field_ident;
      let relationship_method_ident = relationship_accessor_ident(method_ident);
      let part_ty = &child.part_ty;
      let relationship_type = &child.relationship_type;
      match child.kind {
        PartChildKind::Repeated | PartChildKind::RequiredRepeated => quote! {
          #( #attrs )*
          pub fn #relationship_method_ident(
            &self,
          ) -> impl Iterator<Item = &crate::common::RelationshipInfo> + '_ {
            let _ = self.#field_ident;
            crate::sdk::SdkPackage::relationships_by_type(self, #relationship_type)
          }

          #( #attrs )*
          pub fn #method_ident(&self) -> impl Iterator<Item = #part_ty> + '_ {
            self.#field_ident.iter().cloned()
          }
        },
        PartChildKind::Required | PartChildKind::Optional => quote! {
          #( #attrs )*
          pub fn #relationship_method_ident(
            &self,
          ) -> impl Iterator<Item = &crate::common::RelationshipInfo> + '_ {
            let _ = self.#field_ident;
            crate::sdk::SdkPackage::relationships_by_type(self, #relationship_type)
          }

          #( #attrs )*
          pub fn #method_ident(&self) -> Option<#part_ty> {
            self.#field_ident.as_deref().cloned()
          }
        },
      }
    });

  quote! {
    #[inline]
    pub fn parts(&self) -> impl Iterator<Item = crate::parts::IdPartPair<'_>> + '_ {
      crate::sdk::SdkPackage::parts(self)
    }

    #[inline]
    pub fn get_all_parts(&self) -> impl Iterator<Item = crate::parts::PartRef> + '_ {
      crate::sdk::SdkPackage::get_all_parts(self)
    }

    #[inline]
    pub fn get_part_by_id(&self, relationship_id: &str) -> Option<crate::parts::PartRef> {
      crate::sdk::SdkPackage::get_part_by_id(self, relationship_id)
    }

    #[inline]
    pub fn get_part_by_id_required(
      &self,
      relationship_id: &str,
    ) -> Result<crate::parts::PartRef, crate::common::SdkError> {
      crate::sdk::SdkPackage::get_part_by_id_required(self, relationship_id)
    }

    #[inline]
    pub fn try_get_part_by_id(&self, relationship_id: &str) -> Option<crate::parts::PartRef> {
      crate::sdk::SdkPackage::try_get_part_by_id(self, relationship_id)
    }

    #[inline]
    pub fn get_parts_of_type<T: crate::parts::PartRefDowncast>(
      &self,
    ) -> impl Iterator<Item = T> + '_ {
      crate::sdk::SdkPackage::get_parts_of_type(self)
    }

    #[inline]
    pub fn get_id_of_part<T: crate::sdk::SdkPartHandle>(&self, part: &T) -> Option<&str> {
      crate::sdk::SdkPackage::get_id_of_part(self, part)
    }

    #[inline]
    pub fn get_id_of_part_required<T: crate::sdk::SdkPartHandle>(
      &self,
      part: &T,
    ) -> Result<&str, crate::common::SdkError> {
      crate::sdk::SdkPackage::get_id_of_part_required(self, part)
    }

    #[inline]
    pub fn change_id_of_part<T: crate::sdk::SdkPartHandle>(
      &mut self,
      part: &T,
      new_relationship_id: impl Into<String>,
    ) -> Result<String, crate::common::SdkError> {
      crate::sdk::SdkPackage::change_id_of_part(self, part, new_relationship_id)
    }

    #[inline]
    pub fn delete_part_by_id(
      &mut self,
      relationship_id: &str,
    ) -> Result<bool, crate::common::SdkError> {
      crate::sdk::SdkPackage::delete_part_by_id(self, relationship_id)
    }

    #[inline]
    pub fn delete_part<T: crate::sdk::SdkPartHandle>(
      &mut self,
      part: T,
    ) -> Result<bool, crate::common::SdkError> {
      crate::sdk::SdkPackage::delete_part(self, part)
    }

    #[inline]
    pub fn delete_parts<T, I>(&mut self, parts: I) -> Result<(), crate::common::SdkError>
    where
      T: crate::sdk::SdkPartHandle,
      I: IntoIterator<Item = T>,
    {
      crate::sdk::SdkPackage::delete_parts(self, parts)
    }

    #[inline]
    pub fn add_part<T: crate::sdk::SdkPartHandle>(
      &mut self,
      part: T,
    ) -> Result<T, crate::common::SdkError> {
      crate::sdk::SdkPackage::add_part(self, part)
    }

    #[inline]
    pub fn add_part_with_id<T: crate::sdk::SdkPartHandle>(
      &mut self,
      part: T,
      relationship_id: impl Into<String>,
    ) -> Result<T, crate::common::SdkError> {
      crate::sdk::SdkPackage::add_part_with_id(self, part, relationship_id)
    }

    #[inline]
    pub fn add_part_from_package<
      P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      T: crate::sdk::SdkPartHandle,
    >(
      &mut self,
      source_package: &P,
      part: &T,
    ) -> Result<T, crate::common::SdkError>
    where
      Self: crate::parts::PartRootCache,
    {
      crate::sdk::SdkPackage::add_part_from_package(self, source_package, part)
    }

    #[inline]
    pub fn add_part_from_package_with_id<
      P: crate::sdk::SdkPackage + crate::parts::PartRootCache,
      T: crate::sdk::SdkPartHandle,
    >(
      &mut self,
      source_package: &P,
      part: &T,
      relationship_id: impl Into<String>,
    ) -> Result<T, crate::common::SdkError>
    where
      Self: crate::parts::PartRootCache,
    {
      crate::sdk::SdkPackage::add_part_from_package_with_id(
        self,
        source_package,
        part,
        relationship_id,
      )
    }

    #[inline]
    pub fn create_relationship_to_part<T: crate::sdk::SdkPartHandle>(
      &mut self,
      part: T,
    ) -> Result<String, crate::common::SdkError> {
      crate::sdk::SdkPackage::create_relationship_to_part(self, part)
    }

    #[inline]
    pub fn create_relationship_to_part_with_id<T: crate::sdk::SdkPartHandle>(
      &mut self,
      part: T,
      relationship_id: impl Into<String>,
    ) -> Result<String, crate::common::SdkError> {
      crate::sdk::SdkPackage::create_relationship_to_part_with_id(
        self,
        part,
        relationship_id,
      )
    }

    #[inline]
    pub fn create_media_data_part(
      &mut self,
      content_type: impl Into<std::borrow::Cow<'static, str>>,
      extension: impl AsRef<str>,
    ) -> Result<crate::common::MediaDataPart, crate::common::SdkError> {
      crate::sdk::SdkPackage::create_media_data_part(self, content_type, extension)
    }

    #[inline]
    pub fn create_media_data_part_with_content_type(
      &mut self,
      content_type: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Result<crate::common::MediaDataPart, crate::common::SdkError> {
      crate::sdk::SdkPackage::create_media_data_part_with_content_type(self, content_type)
    }

    #[inline]
    pub fn create_media_data_part_by_type(
      &mut self,
      part_type: crate::sdk::MediaDataPartType,
    ) -> Result<crate::common::MediaDataPart, crate::common::SdkError> {
      crate::sdk::SdkPackage::create_media_data_part_by_type(self, part_type)
    }

    #( #accessors )*
  }
}

fn relationship_accessor_ident(field_ident: &Ident) -> Ident {
  Ident::new(
    &format!("{}_relationships", field_ident),
    field_ident.span(),
  )
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

fn part_child_field_info(ty: &Type) -> Option<PartChildMarkerInfo> {
  if let Some(part_ty) = marker_inner_type(ty, "Vec") {
    return Some(PartChildMarkerInfo {
      part_ty,
      kind: PartChildKind::Repeated,
    });
  }
  if let Some(part_ty) = marker_inner_type(ty, "Option") {
    let part_ty = marker_inner_type(&part_ty, "Box").unwrap_or(part_ty);
    return Some(PartChildMarkerInfo {
      part_ty,
      kind: PartChildKind::Optional,
    });
  }
  part_child_marker_info(ty)
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

fn parse_package_main_accessor(attrs: &[Attribute]) -> syn::Result<Option<Ident>> {
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in metas {
      if let Meta::List(meta) = meta
        && meta.path.is_ident("package_main")
      {
        let mut accessor = None;
        meta.parse_nested_meta(|nested| {
          if nested.path.is_ident("accessor") {
            let value: LitStr = nested.value()?.parse()?;
            accessor = Some(value.value());
            Ok(())
          } else {
            Err(nested.error("unsupported sdk package_main attribute"))
          }
        })?;
        let Some(accessor) = accessor else {
          return Err(syn::Error::new_spanned(
            meta,
            "sdk package_main requires accessor",
          ));
        };
        return parse_str(&accessor).map(Some);
      }
    }
  }

  Ok(None)
}

fn passthrough_attrs(attrs: &[Attribute]) -> Vec<Attribute> {
  attrs
    .iter()
    .filter(|attr| !attr.path().is_ident("sdk"))
    .cloned()
    .collect()
}
