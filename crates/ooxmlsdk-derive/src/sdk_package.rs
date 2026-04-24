use super::*;

struct PackageChildInfo {
  attrs: Vec<Attribute>,
  field_ident: Ident,
  part_ty: Type,
  variant_ident: Ident,
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
      variant_ident: part_ref_variant_ident(&marker.part_ty)?,
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
  })
}

fn package_relationship_method_tokens(
  child_infos: &[PackageChildInfo],
) -> proc_macro2::TokenStream {
  let part_ref_branches = child_infos.iter().map(|child| {
    let attrs = &child.attrs;
    let part_ty = &child.part_ty;
    let variant_ident = &child.variant_ident;
    let relationship_type = &child.relationship_type;
    quote! {
      #( #attrs )*
      if crate::common::relationship_type_matches(relationship_type, #relationship_type) {
        return Some(crate::parts::PartRef::#variant_ident(
          <#part_ty as crate::sdk::SdkPartHandle>::from_part_id(part_id),
        ));
      }
    }
  });

  let accessors = child_infos
    .iter()
    .filter(|child| child.main_accessor_ident.is_none())
    .map(|child| {
      let attrs = &child.attrs;
      let field_ident = &child.field_ident;
      let method_ident = &child.field_ident;
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
          pub fn #method_ident(&self) -> impl Iterator<Item = #part_ty> + '_ {
            let _ = self.#field_ident;
            self.relationships().iter().filter_map(#map_relationship)
          }
        },
        PartChildKind::Required | PartChildKind::Optional => quote! {
          #( #attrs )*
          pub fn #method_ident(&self) -> Option<#part_ty> {
            let _ = self.#field_ident;
            self.relationships().iter().find_map(#map_relationship)
          }
        },
      }
    });

  quote! {
    pub fn parts(&self) -> impl Iterator<Item = crate::parts::IdPartPair<'_>> + '_ {
      self
        .relationships()
        .iter()
        .filter_map(|relationship| {
          let part = Self::part_ref_from_relationship(relationship)?;
          Some(crate::parts::IdPartPair::new(relationship.id(), part))
        })
    }

    pub fn get_part_by_id(&self, relationship_id: &str) -> Option<crate::parts::PartRef> {
      let relationship = self.relationships().get(relationship_id)?;
      Self::part_ref_from_relationship(relationship)
    }

    fn part_ref_from_relationship(
      relationship: &crate::common::RelationshipInfo,
    ) -> Option<crate::parts::PartRef> {
      let part_id = relationship.target_part_id()?;
      let relationship_type = relationship.relationship_type();
      #( #part_ref_branches )*
      None
    }

    #( #accessors )*
  }
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
