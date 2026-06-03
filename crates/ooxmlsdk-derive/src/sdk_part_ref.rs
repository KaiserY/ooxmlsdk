use super::*;

struct PartRefVariant {
  attrs: Vec<Attribute>,
  ident: Ident,
  ty: Type,
  relationship_type: Option<String>,
  descriptor: Option<PartRefDescriptor>,
  root: Option<PartRefRoot>,
  extended: bool,
}

struct PartRefDescriptor {
  path_prefix: String,
  content_type: String,
  target_name: String,
  extension: String,
}

struct PartRefRoot {
  element_ty: Type,
  accessor: Ident,
  content_type: String,
}

pub(crate) fn expand_sdk_part_ref(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let Data::Enum(data_enum) = &input.data else {
    return Err(syn::Error::new_spanned(
      input,
      "SdkPartRef can only be derived for enums",
    ));
  };

  let variants = parse_part_ref_variants(data_enum)?;
  let extended_variant = variants
    .iter()
    .find(|variant| variant.extended)
    .ok_or_else(|| syn::Error::new_spanned(input, "SdkPartRef requires ExtendedPart variant"))?;
  let extended_ident = &extended_variant.ident;
  let extended_ty = &extended_variant.ty;

  let part_id_arms = variants.iter().map(|variant| {
    let attrs = cfg_attrs(&variant.attrs);
    let variant_ident = &variant.ident;
    quote! {
      #( #attrs )*
      Self::#variant_ident(part) => <_ as crate::sdk::SdkPart>::part_id(part),
    }
  });

  let descriptor_impls = variants
    .iter()
    .filter(|variant| !variant.extended)
    .map(|variant| {
      let attrs = cfg_attrs(&variant.attrs);
      let variant_ty = &variant.ty;
      let relationship_type = variant
        .relationship_type
        .as_ref()
        .expect("non-extended PartRef variants require relationship type");
      let descriptor = variant
        .descriptor
        .as_ref()
        .expect("non-extended PartRef variants require descriptor");
      let path_prefix = descriptor.path_prefix.as_str();
      let content_type = descriptor.content_type.as_str();
      let target_name = descriptor.target_name.as_str();
      let extension = descriptor.extension.as_str();
      quote! {
        #( #attrs )*
        impl crate::sdk::SdkPartDescriptor for #variant_ty {
          const RELATIONSHIP_TYPE: &'static str = #relationship_type;
          const PATH_PREFIX: &'static str = #path_prefix;
          const CONTENT_TYPE: &'static str = #content_type;
          const TARGET_NAME: &'static str = #target_name;
          const EXTENSION: &'static str = #extension;
        }
      }
    });

  let relationship_arms = variants
    .iter()
    .filter(|variant| !variant.extended)
    .map(|variant| {
      let attrs = cfg_attrs(&variant.attrs);
      let variant_ident = &variant.ident;
      let variant_ty = &variant.ty;
      let relationship_type = variant
        .relationship_type
        .as_ref()
        .expect("non-extended PartRef variants require relationship type");
      let relationship_type_bytes =
        LitByteStr::new(relationship_type.as_bytes(), Span::call_site());
      let construct_part_ref = quote! {
        let part = if let Some(relationship_id) = relationship_id {
          <#variant_ty as crate::sdk::SdkPart>::from_relationship_id(relationship_id, part_id)
        } else {
          <#variant_ty as crate::sdk::SdkPart>::from_part_id(part_id)
        };
        return Some(Self::#variant_ident(part));
      };
      let descriptor = variant
        .descriptor
        .as_ref()
        .expect("non-extended PartRef variants require descriptor");
      let content_type = LitByteStr::new(descriptor.content_type.as_bytes(), Span::call_site());
      let path_prefix = descriptor.path_prefix.as_str();
      let target_name = descriptor.target_name.as_str();
      let content_type_match = quote! { part.content_type().as_bytes() == #content_type };
      let part_ref_guard = if descriptor.content_type.is_empty() {
        quote! { true }
      } else {
        quote! { #content_type_match }
      };
      let office_document_guard = if descriptor.content_type.is_empty() {
        quote! {
          crate::common::package_main_part_path_matches(
            part.path(),
            #path_prefix,
            #target_name,
          )
        }
      } else {
        quote! {
          #content_type_match
            || crate::common::package_main_part_path_matches(
              part.path(),
              #path_prefix,
              #target_name,
            )
        }
      };

      if relationship_type
        == "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"
      {
        quote! {
          #( #attrs )*
          if crate::common::relationship_type_matches_bytes(
            relationship_type,
            #relationship_type_bytes,
          ) && (#office_document_guard) {
            #construct_part_ref
          }
        }
      } else {
        quote! {
          #( #attrs )*
          if crate::common::relationship_type_matches_bytes(
            relationship_type,
            #relationship_type_bytes,
          ) && (#part_ref_guard) {
            #construct_part_ref
          }
        }
      }
    });

  let root_variants = variants
    .iter()
    .filter_map(|variant| variant.root.as_ref().map(|root| (variant, root)));
  let root_bridge_impls = root_variants.clone().map(|(variant, root)| {
    let attrs = cfg_attrs(&variant.attrs);
    let variant_ty = &variant.ty;
    let variant_ident = &variant.ident;
    let root_ty = &root.element_ty;
    quote! {
      #( #attrs )*
      impl crate::sdk::SdkPartRoot for #variant_ty {
        type RootElement = #root_ty;

        #[inline]
        fn wrap_root_element(root_element: Self::RootElement) -> crate::parts::PartRootElement {
          crate::parts::PartRootElement::#variant_ident(Box::new(root_element))
        }

        #[inline]
        fn root_element_ref(
          root_element: &crate::parts::PartRootElement,
        ) -> Option<&Self::RootElement> {
          match root_element {
            crate::parts::PartRootElement::#variant_ident(root) => Some(root.as_ref()),
            _ => None,
          }
        }

        #[inline]
        fn root_element_mut(
          root_element: &mut crate::parts::PartRootElement,
        ) -> Option<&mut Self::RootElement> {
          match root_element {
            crate::parts::PartRootElement::#variant_ident(root) => Some(root.as_mut()),
            _ => None,
          }
        }
      }
    }
  });
  let root_enum_variants = root_variants.clone().map(|(variant, root)| {
    let attrs = cfg_attrs(&variant.attrs);
    let variant_ident = &variant.ident;
    let root_ty = &root.element_ty;
    quote! {
      #( #attrs )*
      #variant_ident(Box<#root_ty>),
    }
  });
  let root_part_type_name_arms = root_variants.clone().map(|(variant, _)| {
    let attrs = cfg_attrs(&variant.attrs);
    let variant_ident = &variant.ident;
    quote! {
      #( #attrs )*
      Self::#variant_ident(_) => stringify!(#variant_ident),
    }
  });
  let root_accessor_methods = root_variants.clone().map(|(variant, root)| {
    let attrs = cfg_attrs(&variant.attrs);
    let variant_ident = &variant.ident;
    let root_ty = &root.element_ty;
    let accessor = &root.accessor;
    let accessor_mut: Ident =
      parse_str(&format!("{accessor}_mut")).expect("root accessor mut identifier");
    quote! {
      #( #attrs )*
      pub fn #accessor(&self) -> Option<&#root_ty> {
        match self {
          Self::#variant_ident(root) => Some(root.as_ref()),
          _ => None,
        }
      }

      #( #attrs )*
      pub fn #accessor_mut(&mut self) -> Option<&mut #root_ty> {
        match self {
          Self::#variant_ident(root) => Some(root.as_mut()),
          _ => None,
        }
      }
    }
  });
  let root_to_bytes_arms = root_variants.clone().map(|(variant, _)| {
    let attrs = cfg_attrs(&variant.attrs);
    let variant_ident = &variant.ident;
    quote! {
      #( #attrs )*
      Self::#variant_ident(root) => Ok(root.to_bytes()?),
    }
  });
  let root_clear_arms = root_variants.clone().map(|(variant, _)| {
    let attrs = cfg_attrs(&variant.attrs);
    let variant_ident = &variant.ident;
    if variant_ident == "MainDocumentPart" {
      quote! {
        #( #attrs )*
        Self::#variant_ident(root) => {
          crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::__ooxmlsdk_clear_recursive_table_dom_from_document(root.as_mut());
        }
      }
    } else {
      quote! {
        #( #attrs )*
        Self::#variant_ident(_) => {}
      }
    }
  });
  let root_from_part_id_branches = root_variants.clone().map(|(variant, root)| {
    let attrs = cfg_attrs(&variant.attrs);
    let variant_ident = &variant.ident;
    let root_ty = &root.element_ty;
    let content_type = LitByteStr::new(root.content_type.as_bytes(), Span::call_site());
    quote! {
      #( #attrs )*
      if crate::sdk::part_root_content_type_matches_bytes(
        #content_type,
        part.content_type().as_bytes(),
      ) {
        let bytes = part.data().bytes();
        let root = if let Some(bytes) = crate::common::decode_utf16_xml_bytes(bytes)? {
          <#root_ty>::from_reader(std::io::Cursor::new(bytes))?
        } else {
          <#root_ty>::from_bytes(bytes)?
        };
        #[cfg(feature = "mce")]
        let mut root = root;
        #[cfg(feature = "mce")]
        crate::sdk::SdkMce::process_mce(
          &mut root,
          &open_settings.markup_compatibility_process_settings,
        )?;
        return Ok(Some(Self::#variant_ident(Box::new(root))));
      }
    }
  });
  let root_validate_arms = root_variants.map(|(variant, _)| {
    let attrs = cfg_attrs(&variant.attrs);
    let variant_ident = &variant.ident;
    quote! {
      #( #attrs )*
      Self::#variant_ident(root) => crate::validator::SdkValidator::validate_into(root.as_ref(), context),
    }
  });

  Ok(quote! {
    #( #descriptor_impls )*
    #( #root_bridge_impls )*

    #[derive(Clone, Debug)]
    pub enum PartRootElement {
      #( #root_enum_variants )*
    }

    impl PartRootElement {
      #[inline]
      pub fn part_type_name(&self) -> &'static str {
        match self {
          #( #root_part_type_name_arms )*
        }
      }

      #( #root_accessor_methods )*

      pub fn to_bytes(&self) -> Result<Vec<u8>, crate::common::SdkError> {
        match self {
          #( #root_to_bytes_arms )*
        }
      }

      #[doc(hidden)]
      pub(crate) fn clear_recursive_dom(&mut self) {
        match self {
          #( #root_clear_arms )*
        }
      }

      pub(crate) fn from_part_id(
        storage: &crate::common::SdkPackageStorage,
        part_id: crate::common::PartId,
        open_settings: &crate::sdk::OpenSettings,
      ) -> Result<Option<Self>, crate::common::SdkError> {
        let Some(part) = storage.part(part_id) else {
          return Ok(None);
        };
        if part.relationship_type_bytes().is_some_and(|relationship_type| {
          crate::common::relationship_type_matches_bytes(
            relationship_type,
            crate::common::REL_AF_CHUNK,
          )
        })
        {
          return Ok(None);
        }
        #[cfg(not(feature = "mce"))]
        let _ = open_settings;
        #( #root_from_part_id_branches )*
        Ok(None)
      }
    }

    #[cfg(feature = "validators")]
    impl crate::validator::SdkValidator for PartRootElement {
      fn validate_into(&self, context: &mut crate::validator::ValidationContext) {
        match self {
          #( #root_validate_arms )*
        }
      }
    }

    impl #ident {
      #[inline]
      pub fn part_id(&self) -> crate::common::PartId {
        match self {
          #( #part_id_arms )*
        }
      }

      #[inline]
      pub(crate) fn from_part_id<P: crate::sdk::SdkPackage>(
        package: &P,
        part_id: crate::common::PartId,
      ) -> Option<Self> {
        Self::from_storage(crate::sdk::SdkPackage::storage(package), part_id, None)
      }

      #[inline]
      pub(crate) fn from_relationship_storage(
        storage: &crate::common::SdkPackageStorage,
        relationship: &crate::common::RelationshipInfo,
      ) -> Option<Self> {
        Self::from_storage(storage, relationship.target_part_id()?, Some(relationship.id()))
      }

      fn from_storage(
        storage: &crate::common::SdkPackageStorage,
        part_id: crate::common::PartId,
        relationship_id: Option<&str>,
      ) -> Option<Self> {
        let part = storage.part(part_id)?;
        let Some(relationship_type) = part.relationship_type_bytes() else {
          return Some(Self::extended_part(storage, part_id, relationship_id));
        };

        #( #relationship_arms )*

        Some(Self::extended_part(storage, part_id, relationship_id))
      }

      #[inline]
      fn extended_part(
        storage: &crate::common::SdkPackageStorage,
        part_id: crate::common::PartId,
        relationship_id: Option<&str>,
      ) -> Self {
        let part = if let Some(relationship_id) = relationship_id {
          <#extended_ty>::from_relationship_id_with_relationships(
            storage,
            relationship_id,
            part_id,
          )
        } else {
          <#extended_ty>::from_part_id_with_relationships(
            storage,
            part_id,
          )
        };
        Self::#extended_ident(part)
      }
    }
  })
}

fn parse_part_ref_variants(data_enum: &DataEnum) -> syn::Result<Vec<PartRefVariant>> {
  data_enum
    .variants
    .iter()
    .map(|variant| {
      let Fields::Unnamed(fields) = &variant.fields else {
        return Err(syn::Error::new_spanned(
          variant,
          "SdkPartRef variants must have one unnamed field",
        ));
      };
      if fields.unnamed.len() != 1 {
        return Err(syn::Error::new_spanned(
          variant,
          "SdkPartRef variants must have one unnamed field",
        ));
      }
      let extended = variant.ident == "ExtendedPart";
      let part_ref_attr = parse_part_ref_attr(&variant.attrs, &variant.ident)?;
      if !extended && part_ref_attr.relationship_type.is_none() {
        return Err(syn::Error::new_spanned(
          variant,
          "SdkPartRef variants require #[sdk(relationship_type = ...)]",
        ));
      }
      if !extended && part_ref_attr.descriptor.is_none() {
        return Err(syn::Error::new_spanned(
          variant,
          "SdkPartRef variants require descriptor attributes",
        ));
      }
      let root = match part_ref_attr.root {
        Some(mut root) => {
          if root.content_type.is_empty()
            && let Some(descriptor) = &part_ref_attr.descriptor
          {
            root.content_type = descriptor.content_type.clone();
          }
          Some(root)
        }
        None => None,
      };
      Ok(PartRefVariant {
        attrs: variant
          .attrs
          .iter()
          .filter(|attr| !attr.path().is_ident("sdk"))
          .cloned()
          .collect(),
        ident: variant.ident.clone(),
        ty: fields.unnamed[0].ty.clone(),
        relationship_type: part_ref_attr.relationship_type,
        descriptor: part_ref_attr.descriptor,
        root,
        extended,
      })
    })
    .collect()
}

#[derive(Default)]
struct PartRefAttr {
  relationship_type: Option<String>,
  descriptor: Option<PartRefDescriptor>,
  root: Option<PartRefRoot>,
}

fn parse_part_ref_attr(attrs: &[Attribute], variant_ident: &Ident) -> syn::Result<PartRefAttr> {
  let mut parsed = PartRefAttr::default();
  let mut path_prefix = ".".to_string();
  let mut content_type = String::new();
  let mut target_name = None;
  let mut extension = String::new();
  let mut has_descriptor = false;
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in metas {
      match meta {
        Meta::NameValue(name_value) if name_value.path.is_ident("relationship_type") => {
          parsed.relationship_type = Some(parse_lit_str_value(name_value.value)?);
        }
        Meta::NameValue(name_value) if name_value.path.is_ident("path_prefix") => {
          path_prefix = parse_lit_str_value(name_value.value)?;
          has_descriptor = true;
        }
        Meta::NameValue(name_value) if name_value.path.is_ident("content_type") => {
          content_type = parse_lit_str_value(name_value.value)?;
          has_descriptor = true;
        }
        Meta::NameValue(name_value) if name_value.path.is_ident("target_name") => {
          target_name = Some(parse_lit_str_value(name_value.value)?);
          has_descriptor = true;
        }
        Meta::NameValue(name_value) if name_value.path.is_ident("extension") => {
          extension = parse_lit_str_value(name_value.value)?;
          has_descriptor = true;
        }
        Meta::List(meta) if meta.path.is_ident("root") => {
          parsed.root = Some(parse_part_ref_root(meta, variant_ident)?);
        }
        _ => {}
      }
    }
  }
  if has_descriptor {
    parsed.descriptor = Some(PartRefDescriptor {
      path_prefix,
      content_type,
      target_name: target_name
        .ok_or_else(|| syn::Error::new(Span::call_site(), "sdk part ref requires target_name"))?,
      extension,
    });
  }
  Ok(parsed)
}

fn parse_lit_str_value(value: Expr) -> syn::Result<String> {
  match value {
    Expr::Lit(ExprLit {
      lit: Lit::Str(value),
      ..
    }) => Ok(value.value()),
    value => Err(syn::Error::new_spanned(value, "expected string literal")),
  }
}

fn parse_part_ref_root(meta: syn::MetaList, variant_ident: &Ident) -> syn::Result<PartRefRoot> {
  let mut element_ty = None;
  let mut accessor = None;
  let mut content_type = String::new();

  meta.parse_nested_meta(|nested| {
    if nested.path.is_ident("element") {
      let value: Expr = nested.value()?.parse()?;
      let Expr::Path(path) = value else {
        return Err(nested.error("root element must be a type path"));
      };
      element_ty = Some(Type::Path(TypePath {
        qself: None,
        path: path.path,
      }));
      Ok(())
    } else if nested.path.is_ident("accessor") {
      let value: Expr = nested.value()?.parse()?;
      let Expr::Path(path) = value else {
        return Err(nested.error("root accessor must be an identifier"));
      };
      let Some(ident) = path.path.get_ident() else {
        return Err(nested.error("root accessor must be an identifier"));
      };
      accessor = Some(ident.clone());
      Ok(())
    } else if nested.path.is_ident("content_type") {
      let value: LitStr = nested.value()?.parse()?;
      content_type = value.value();
      Ok(())
    } else {
      Err(nested.error("unsupported sdk root attribute"))
    }
  })?;

  Ok(PartRefRoot {
    element_ty: element_ty
      .ok_or_else(|| syn::Error::new_spanned(&meta, "sdk root requires element"))?,
    accessor: accessor.unwrap_or_else(|| {
      Ident::new(
        &format!("as_{}", simple_snake_case(&variant_ident.to_string())),
        Span::call_site(),
      )
    }),
    content_type,
  })
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

fn cfg_attrs(attrs: &[Attribute]) -> Vec<Attribute> {
  attrs
    .iter()
    .filter(|attr| attr.path().is_ident("cfg") || attr.path().is_ident("cfg_attr"))
    .cloned()
    .collect()
}
