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
    if field_ident == "storage" || field_ident == "main_part_id" || field_ident == "root_elements" {
      continue;
    }

    let Some(marker) = part_child_marker_info(&field.ty) else {
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
        let _ = self.#field_ident;
        self
          .#main_part_id_ident
          .map(<#part_ty as crate::sdk::SdkPartHandle>::from_part_id)
          .ok_or_else(|| {
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
          relationship_id,
          crate::common::NewPartTargetMode::Fixed,
        )?;
        self.#main_part_id_ident = Some(part.part_id());
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
  let child_field_inits = child_infos.iter().map(|child| {
    let field_ident = &child.field_ident;
    quote! {
      #field_ident: Default::default(),
    }
  });

  Ok(quote! {
    impl crate::sdk::SdkPackage for #ident {
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

      pub fn save<W: std::io::Write + std::io::Seek>(
        &self,
        writer: W,
      ) -> Result<(), crate::common::SdkError> {
        crate::parts::save_package(self, writer)
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

        Ok(Self {
          #storage_ident: storage,
          #main_part_id_ident: main_part_id,
          #root_elements_init
          #( #child_field_inits )*
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
          #( #attrs )*
          pub fn #relationship_method_ident(
            &self,
          ) -> impl Iterator<Item = &crate::common::RelationshipInfo> + '_ {
            let _ = self.#field_ident;
            self.relationships_by_type(#relationship_type)
          }

          #( #attrs )*
          pub fn #method_ident(&self) -> impl Iterator<Item = #part_ty> + '_ {
            let _ = self.#field_ident;
            self.relationships().iter().filter_map(#map_relationship)
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
            let _ = self.#field_ident;
            self.relationships().iter().find_map(#map_relationship)
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
    pub fn get_id_of_part<T: crate::sdk::SdkPartHandle>(&self, part: T) -> Option<&str> {
      crate::sdk::SdkPackage::get_id_of_part(self, part)
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
