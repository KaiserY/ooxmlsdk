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

fn is_relationship_model_field(ident: &Ident) -> bool {
  ident == "fallback_parts"
    || ident == "relationship_order"
    || ident == "data_part_reference_relationships"
    || ident == "reference_relationships"
    || ident == "raw_relationships"
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
      kind: marker.kind,
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
        let part = crate::sdk::SdkPackage::add_new_part_with_target_mode::<#part_ty>(
          self,
          relationship_id.clone(),
          crate::common::NewPartTargetMode::Fixed,
        )?;
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
  let child_field_locals: Vec<_> = child_infos.iter().map(|child| {
    let field_ident = &child.field_ident;
    let part_ty = &child.part_ty;
    let relationship_type = &child.relationship_type;
    let make_part = quote! {
      |relationship: &crate::common::RelationshipInfo| {
        if crate::common::relationship_type_matches(
          relationship.relationship_type(),
          #relationship_type,
        ) {
          relationship.target_part_id().map(|part_id| {
            <#part_ty as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
              &storage,
              relationship.id(),
              part_id,
            )
          })
        } else {
          None
        }
      }
    };
    match child.kind {
      PartChildKind::Repeated => quote! {
        let #field_ident = storage.package_relationships().iter().filter_map(#make_part).collect();
      },
      PartChildKind::Required | PartChildKind::Optional => quote! {
        let #field_ident = storage
          .package_relationships()
          .iter()
          .find_map(#make_part)
          .map(Box::new);
      },
    }
  }).collect();
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
  let represented_relationship_id_stmts = child_infos.iter().map(|child| {
    let field_ident = &child.field_ident;
    match child.kind {
      PartChildKind::Repeated => quote! {
        for part in &#field_ident {
          if let Some(relationship_id) = crate::sdk::SdkPartHandle::relationship_id(part) {
            represented_relationship_ids.insert(relationship_id);
          }
        }
      },
      PartChildKind::Required | PartChildKind::Optional => quote! {
        if let Some(part) = #field_ident.as_deref() {
          if let Some(relationship_id) = crate::sdk::SdkPartHandle::relationship_id(part) {
            represented_relationship_ids.insert(relationship_id);
          }
        }
      },
    }
  });
  let relationship_model_field_locals = quote! {
    let mut represented_relationship_ids = std::collections::HashSet::<&str>::new();
    #( #represented_relationship_id_stmts )*
    let mut fallback_parts = Vec::new();
    let relationship_order: Vec<Box<str>> = storage
      .package_relationships()
      .iter()
      .map(|relationship| relationship.id().into())
      .collect();
    let mut data_part_reference_relationships = Vec::new();
    let mut reference_relationships = Vec::new();
    let mut raw_relationships = Vec::new();
    for relationship in storage.package_relationships().iter() {
      if relationship.is_reference_relationship() {
        if relationship.reference_kind().is_some_and(|kind| {
          matches!(
            kind,
            crate::common::ReferenceRelationshipKind::Audio
              | crate::common::ReferenceRelationshipKind::Media
              | crate::common::ReferenceRelationshipKind::Video
          )
        }) {
          data_part_reference_relationships.push(relationship.clone());
        } else {
          reference_relationships.push(relationship.clone());
        }
      } else if relationship.target_kind() == crate::common::RelationshipTargetKind::InternalPart {
        if !represented_relationship_ids.contains(relationship.id()) {
          if let Some(part) = crate::parts::PartRef::from_relationship_storage(&storage, relationship) {
            fallback_parts.push(part);
          }
        }
      } else {
        raw_relationships.push(relationship.clone());
      }
    }
  };
  let package_modeled_child_stmts = child_infos.iter().map(|child| {
    let field_ident = &child.field_ident;
    match child.kind {
      PartChildKind::Repeated => quote! {
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
  });
  let package_collect_relationship_child_stmts = child_infos.iter().map(|child| {
    let field_ident = &child.field_ident;
    match child.kind {
      PartChildKind::Repeated => quote! {
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
  let package_collect_child_stmts = child_infos.iter().map(|child| {
    let field_ident = &child.field_ident;
    match child.kind {
      PartChildKind::Repeated => quote! {
        for part in &self.#field_ident {
          crate::sdk::SdkPartHandle::collect_modeled_part_relationship_graphs(part, self, graphs)?;
        }
      },
      PartChildKind::Required | PartChildKind::Optional => quote! {
        if let Some(part) = self.#field_ident.as_deref() {
          crate::sdk::SdkPartHandle::collect_modeled_part_relationship_graphs(part, self, graphs)?;
        }
      },
    }
  });

  Ok(quote! {
    impl crate::sdk::SdkPackage for #ident {
      const CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] =
        Self::GENERATED_CHILD_DESCRIPTORS;

      #[inline]
      fn storage(&self) -> &crate::common::SdkPackageStorage {
        &self.#storage_ident
      }

      #[inline]
      fn storage_mut(&mut self) -> &mut crate::common::SdkPackageStorage {
        &mut self.#storage_ident
      }

      #[inline]
      fn main_part_id(&self) -> Option<crate::common::PartId> {
        self.#main_part_id_ident
      }

      fn modeled_relationships(
        &self,
      ) -> Result<crate::common::RelationshipSet, crate::common::SdkError> {
        let mut relationships = crate::common::RelationshipSet::default();
        #( #package_modeled_child_stmts )*
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
        relationships.reorder_by_ids(&self.relationship_order);
        Ok(relationships)
      }

      fn modeled_relationship_graph(
        &self,
      ) -> Result<crate::common::RelationshipGraph, crate::common::SdkError> {
        Ok(self.modeled_relationships()?.to_relationship_graph())
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

      fn collect_modeled_part_relationship_graphs(
        &self,
        graphs: &mut std::collections::HashMap<
          crate::common::PartId,
          crate::common::RelationshipGraph,
        >,
      ) -> Result<(), crate::common::SdkError> {
        #( #package_collect_child_stmts )*
        for part in &self.fallback_parts {
          part.collect_modeled_part_relationship_graphs(self, graphs)?;
        }
        Ok(())
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

      pub fn relationships(&self) -> &crate::common::RelationshipSet {
        self.#storage_ident.package_relationships()
      }

      pub fn relationships_mut(&mut self) -> &mut crate::common::RelationshipSet {
        self.#storage_ident.package_relationships_mut()
      }

      #[inline]
      pub fn relationship_graph(&self) -> crate::common::RelationshipGraph {
        crate::sdk::SdkPackage::relationship_graph(self)
      }

      #[inline]
      pub fn replace_relationships_from_graph(
        &mut self,
        graph: crate::common::RelationshipGraph,
      ) {
        crate::sdk::SdkPackage::replace_relationships_from_graph(self, graph)
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
      pub fn add_hyperlink_relationship(
        &mut self,
        relationship_id: impl Into<String>,
        target: impl Into<String>,
      ) -> Result<&crate::common::RelationshipInfo, crate::common::SdkError> {
        crate::sdk::SdkPackage::add_hyperlink_relationship(self, relationship_id, target)
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
      pub fn delete_reference_relationship(
        &mut self,
        relationship_id: &str,
      ) -> Result<crate::common::RelationshipInfo, crate::common::SdkError> {
        crate::sdk::SdkPackage::delete_reference_relationship(self, relationship_id)
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
      pub fn relationships_by_type(
        &self,
        relationship_type: &str,
      ) -> impl Iterator<Item = &crate::common::RelationshipInfo> {
        crate::sdk::SdkPackage::relationships_by_type(self, relationship_type)
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

      #main_part_method

      #package_relationship_methods

      pub fn modeled_relationship_graph(
        &self,
      ) -> Result<crate::common::RelationshipGraph, crate::common::SdkError> {
        crate::sdk::SdkPackage::modeled_relationship_graph(self)
      }

      pub fn modeled_relationships(
        &self,
      ) -> Result<crate::common::RelationshipSet, crate::common::SdkError> {
        crate::sdk::SdkPackage::modeled_relationships(self)
      }

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
        PartChildKind::Repeated => quote! {
          #( #attrs )*
          pub fn #relationship_method_ident(
            &self,
          ) -> impl Iterator<Item = &crate::common::RelationshipInfo> + '_ {
            let _ = self.#field_ident;
            self.relationships_by_type(#relationship_type)
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
            self.relationships_by_type(#relationship_type)
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
    pub fn get_part_by_id(&self, relationship_id: &str) -> Option<crate::parts::PartRef> {
      crate::sdk::SdkPackage::get_part_by_id(self, relationship_id)
    }

    #[inline]
    pub fn try_get_part_by_id(&self, relationship_id: &str) -> Option<crate::parts::PartRef> {
      crate::sdk::SdkPackage::try_get_part_by_id(self, relationship_id)
    }

    #[inline]
    pub fn get_parts_of_type<T: crate::sdk::SdkPartHandle + 'static>(
      &self,
    ) -> impl Iterator<Item = T> + '_ {
      crate::sdk::SdkPackage::get_parts_of_type(self)
    }

    #[inline]
    pub fn get_sub_part_of_type<T: crate::sdk::SdkPartHandle + 'static>(&self) -> Option<T> {
      crate::sdk::SdkPackage::get_sub_part_of_type(self)
    }

    #[inline]
    pub fn get_id_of_part<T: crate::sdk::SdkPartHandle>(&self, part: &T) -> Option<&str> {
      crate::sdk::SdkPackage::get_id_of_part(self, part)
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
    pub fn delete_parts_of_type<T: crate::sdk::SdkPartHandle + 'static>(
      &mut self,
    ) -> Result<(), crate::common::SdkError> {
      crate::sdk::SdkPackage::delete_parts_of_type::<T>(self)
    }

    #[inline]
    pub fn delete_parts_recursively_of_type<T: crate::sdk::SdkPartHandle + 'static>(
      &mut self,
    ) -> Result<(), crate::common::SdkError> {
      crate::sdk::SdkPackage::delete_parts_recursively_of_type::<T>(self)
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
