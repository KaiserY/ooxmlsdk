use super::*;

pub(crate) fn expand_sdk_part(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
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

  if !is_part_handle_spec(&input.attrs)? {
    return Err(syn::Error::new_spanned(
      input,
      "SdkPart supports generated part handle specs only",
    ));
  }

  expand_part_handle_spec(input, fields)
}

fn is_part_handle_spec(attrs: &[Attribute]) -> syn::Result<bool> {
  let mut is_spec = false;
  for attr in attrs.iter().filter(|attr| attr.path().is_ident("sdk")) {
    attr.parse_nested_meta(|meta| {
      if meta.path.is_ident("part_handle_spec") {
        is_spec = true;
        Ok(())
      } else {
        Err(meta.error("unsupported SdkPart attribute"))
      }
    })?;
  }
  Ok(is_spec)
}

struct PartHandleChildInfo {
  attrs: Vec<Attribute>,
  field_ident: Ident,
  part_ty: Type,
  kind: PartChildKind,
  relationship_type: PartRelationshipTypeSource,
}

struct PartChildMarkerInfo {
  part_ty: Type,
  kind: PartChildKind,
}

struct PartHandleRootInfo {
  root_ty: Type,
}

fn expand_part_handle_spec(
  input: &DeriveInput,
  fields: &syn::FieldsNamed,
) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let mut child_infos = Vec::new();
  let mut root_info = None;
  for field in &fields.named {
    if let Some(marker) = part_child_field_info(&field.ty) {
      let relationship_type = parse_part_child_relationship_type_attr(&field.attrs)?
        .unwrap_or(PartRelationshipTypeSource::TypeConst);
      let field_ident = field.ident.clone().unwrap();
      child_infos.push(PartHandleChildInfo {
        attrs: cfg_attrs(&field.attrs),
        field_ident,
        part_ty: marker.part_ty,
        kind: parse_part_child_kind_attr(&field.attrs)?.unwrap_or(marker.kind),
        relationship_type,
      });
      continue;
    }

    if let Some(root_ty) = marker_inner_type(&field.ty, "PartRoot") {
      root_info = Some(PartHandleRootInfo { root_ty });
      continue;
    }

    return Err(syn::Error::new_spanned(
      field,
      "part handle specs may contain only PartRoot and PartChild marker fields",
    ));
  }

  let handle_ty = quote! { crate::sdk::PartHandle<#ident> };
  let marker_field_reads = fields.named.iter().map(|field| {
    let field_ident = field.ident.as_ref().expect("named fields");
    quote! { let _ = &spec.#field_ident; }
  });
  let root_method = part_handle_root_method_tokens(&handle_ty, ident, root_info.as_ref());
  let child_methods = part_handle_child_methods_tokens(&handle_ty, &child_infos);
  let constraint_impl = part_constraint_impl_tokens(&child_infos);
  Ok(quote! {
    const _: fn(&#ident) = |spec| {
      #( #marker_field_reads )*
    };

    impl crate::sdk::SdkPart for #handle_ty {
      #constraint_impl
    }

    #root_method
    #child_methods
  })
}

fn part_constraint_impl_tokens(child_infos: &[PartHandleChildInfo]) -> proc_macro2::TokenStream {
  let entries = child_infos.iter().map(|child| {
    let attrs = &child.attrs;
    let entry = part_constraint_entry_tokens(child);
    quote! {
      #( #attrs )*
      #entry
    }
  });
  let lookup = if child_infos.iter().all(|child| child.attrs.is_empty()) {
    let arms = child_infos.iter().enumerate().map(|(index, child)| {
      let variant_ident = part_kind_variant_ident(&child.part_ty);
      quote! {
        crate::parts::PartKind::#variant_ident => Some(Self::CHILD_PART_CONSTRAINTS[#index]),
      }
    });
    quote! {
      match kind {
        #( #arms )*
        _ => None,
      }
    }
  } else {
    quote! {
      Self::CHILD_PART_CONSTRAINTS
        .iter()
        .copied()
        .find(|constraint| constraint.child_kind == kind)
    }
  };

  quote! {
    const CHILD_PART_CONSTRAINTS: &'static [crate::sdk::PartConstraint] = &[
      #( #entries, )*
    ];

    #[inline]
    fn child_part_constraint(
      kind: crate::parts::PartKind,
    ) -> Option<crate::sdk::PartConstraint> {
      #lookup
    }
  }
}

fn part_constraint_entry_tokens(child: &PartHandleChildInfo) -> proc_macro2::TokenStream {
  let part_ty = &child.part_ty;
  let relationship_type = part_relationship_type_tokens(&child.relationship_type, part_ty);
  let (min_occurs_is_non_zero, max_occurs_great_than_one) = match child.kind {
    PartChildKind::Optional => (false, false),
    PartChildKind::Required => (true, false),
    PartChildKind::Repeated => (false, true),
    PartChildKind::RequiredRepeated => (true, true),
  };
  quote! {
    crate::sdk::PartConstraint::new(
      <#part_ty as crate::sdk::SdkPartDescriptor>::KIND,
      #relationship_type,
      <#part_ty as crate::sdk::SdkPartDescriptor>::CONTENT_TYPE,
      #min_occurs_is_non_zero,
      #max_occurs_great_than_one,
    )
  }
}

fn part_relationship_type_tokens(
  relationship_type: &PartRelationshipTypeSource,
  part_ty: &Type,
) -> proc_macro2::TokenStream {
  match relationship_type {
    PartRelationshipTypeSource::Explicit(value) => quote! { #value },
    PartRelationshipTypeSource::TypeConst => {
      quote! { <#part_ty as crate::sdk::SdkPartDescriptor>::RELATIONSHIP_TYPE }
    }
  }
}

fn part_kind_variant_ident(part_ty: &Type) -> &Ident {
  let Type::Path(type_path) = part_ty else {
    panic!("generated child part type must be a path")
  };
  &type_path
    .path
    .segments
    .last()
    .expect("generated child part path must not be empty")
    .ident
}

fn part_handle_root_method_tokens(
  part_ty: &proc_macro2::TokenStream,
  part_ident: &Ident,
  root_info: Option<&PartHandleRootInfo>,
) -> proc_macro2::TokenStream {
  let Some(root_info) = root_info else {
    return quote! {};
  };
  let root_ty = &root_info.root_ty;
  let load_root_element = quote! {
    {
      let root_element = {
        let storage = crate::sdk::SdkPackage::storage(package);
        let bytes = storage.part_bytes_for_root(part_slot)?;
        if let Some(bytes) = crate::common::decode_utf16_xml_bytes(&bytes)? {
          <#root_ty as crate::sdk::SdkType>::from_bytes(&bytes)?
        } else {
          <#root_ty as crate::sdk::SdkType>::from_bytes(&bytes)?
        }
      };

      #[cfg(feature = "mce")]
      let root_element = {
        let mut root_element = root_element;
        root_element.process_mce(
          &crate::sdk::SdkPackage::open_settings(package)
            .markup_compatibility_process_settings,
        )?;
        root_element
      };

      root_element
    }
  };

  quote! {
    impl #part_ty {
      #[inline]
      pub fn is_root_element_loaded<P: crate::sdk::SdkPackage>(&self, package: &P) -> bool {
        self
          .resolve_optional(crate::sdk::SdkPackage::storage(package))
          .is_some_and(|part_slot| {
            crate::sdk::SdkPackage::is_root_element_loaded(package, part_slot)
          })
      }

      #[inline]
      pub fn unload_root_element<P: crate::sdk::SdkPackage>(
        &self,
        package: &mut P,
      ) -> Option<crate::parts::PartRootElement> {
        let part_slot = self
          .resolve_optional(crate::sdk::SdkPackage::storage(package))?;
        crate::sdk::SdkPackage::unload_root_element(package, part_slot)
      }

      pub fn root_element<'a, P: crate::sdk::SdkPackage>(
        &self,
        package: &'a P,
      ) -> Result<&'a #root_ty, crate::common::SdkError> {
        let part_slot = self.resolve(crate::sdk::SdkPackage::storage(package))?;
        if crate::sdk::SdkPackage::root_element(package, part_slot)
          .and_then(<Self as crate::sdk::SdkPartRoot>::root_element_ref)
          .is_none()
        {
          let root_element = #load_root_element;
          crate::sdk::SdkPackage::cache_root_element(
            package,
            part_slot,
            <Self as crate::sdk::SdkPartRoot>::wrap_root_element(root_element),
          )
          .ok_or(crate::common::SdkError::StalePart)?;
        }

        crate::sdk::SdkPackage::root_element(package, part_slot)
          .and_then(<Self as crate::sdk::SdkPartRoot>::root_element_ref)
          .ok_or_else(|| {
            crate::common::SdkError::CommonError(
              concat!("cached root element has unexpected type for ", stringify!(#part_ident))
                .to_string(),
            )
          })
      }

      pub fn root_element_mut<'a, P: crate::sdk::SdkPackage>(
        &self,
        package: &'a mut P,
      ) -> Result<&'a mut #root_ty, crate::common::SdkError> {
        self.root_element(package)?;
        let part_slot = self.resolve(crate::sdk::SdkPackage::storage(package))?;
        crate::sdk::SdkPackage::root_element_mut(package, part_slot)
          .and_then(<Self as crate::sdk::SdkPartRoot>::root_element_mut)
          .ok_or_else(|| {
            crate::common::SdkError::CommonError(
              concat!("cached root element has unexpected type for ", stringify!(#part_ident))
                .to_string(),
            )
          })
      }

      pub fn set_root_element<P: crate::sdk::SdkPackage>(
        &self,
        package: &mut P,
        root_element: #root_ty,
      ) -> Result<(), crate::common::SdkError> {
        let part_slot = self.resolve(crate::sdk::SdkPackage::storage(package))?;
        crate::sdk::SdkPackage::replace_root_element(
          package,
          part_slot,
          <Self as crate::sdk::SdkPartRoot>::wrap_root_element(root_element),
        )
        .then_some(())
        .ok_or(crate::common::SdkError::StalePart)
      }
    }
  }
}

fn part_handle_child_methods_tokens(
  part_ty: &proc_macro2::TokenStream,
  child_infos: &[PartHandleChildInfo],
) -> proc_macro2::TokenStream {
  let accessors = child_infos.iter().map(|child| {
    let method_ident = &child.field_ident;
    let part_ty = &child.part_ty;
    let relationship_matches = relationship_match_condition_tokens(
      &child.relationship_type,
      quote! { relationship },
      quote! { <#part_ty as crate::sdk::SdkPartDescriptor>::RELATIONSHIP_TYPE },
    );
    let map_relationship = quote! {
      move |relationship: &crate::common::RelationshipInfo| {
        if #relationship_matches {
          crate::sdk::relationship_target_as_part::<#part_ty>(storage, relationship)
        } else {
          None
        }
      }
    };

    match child.kind {
      PartChildKind::Repeated | PartChildKind::RequiredRepeated => quote! {
        pub fn #method_ident<'a, P: crate::sdk::SdkPackage>(
          &'a self,
          package: &'a P,
        ) -> impl Iterator<Item = #part_ty> + 'a {
          let storage = crate::sdk::SdkPackage::storage(package);
          self
            .resolve_optional(storage)
            .into_iter()
            .flat_map(move |part_slot| storage.relationships(part_slot))
            .flat_map(crate::common::RelationshipSet::iter)
            .filter_map(#map_relationship)
        }
      },
      PartChildKind::Required | PartChildKind::Optional => quote! {
        pub fn #method_ident<P: crate::sdk::SdkPackage>(
          &self,
          package: &P,
        ) -> Option<#part_ty> {
          let storage = crate::sdk::SdkPackage::storage(package);
          self
            .resolve_optional(storage)
            .into_iter()
            .flat_map(move |part_slot| storage.relationships(part_slot))
            .flat_map(crate::common::RelationshipSet::iter)
            .find_map(#map_relationship)
        }
      },
    }
  });

  quote! {
    impl #part_ty {
      #( #accessors )*
    }
  }
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
