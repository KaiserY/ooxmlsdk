use super::*;

pub(crate) fn expand_sdk_xml_namespace(
  input: &DeriveInput,
) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let Data::Enum(DataEnum { variants, .. }) = &input.data else {
    return Err(syn::Error::new_spanned(
      input,
      "SdkXmlNamespace can only be derived for enums",
    ));
  };

  let mut prefix_arms = Vec::with_capacity(variants.len());
  let mut prefix_bytes_arms = Vec::with_capacity(variants.len());
  let mut uri_arms = Vec::with_capacity(variants.len());
  let mut uri_bytes_arms = Vec::with_capacity(variants.len());
  let mut from_uri_bytes_arms = Vec::with_capacity(variants.len());

  for variant in variants {
    if !matches!(variant.fields, Fields::Unit) {
      return Err(syn::Error::new_spanned(
        &variant.ident,
        "SdkXmlNamespace only supports fieldless variants",
      ));
    }

    let variant_ident = &variant.ident;
    let cfg_attrs = cfg_attrs(&variant.attrs);
    let (prefix, uri) = parse_sdk_xml_namespace_variant_attr(&variant.attrs)?;
    let prefix_lit = LitStr::new(&prefix, Span::call_site());
    let uri_lit = LitStr::new(&uri, Span::call_site());
    let uri_bytes_lit = LitByteStr::new(uri.as_bytes(), Span::call_site());

    prefix_arms.push(quote! {
      #(#cfg_attrs)*
      Self::#variant_ident => #prefix_lit,
    });
    prefix_bytes_arms.push(quote! {
      #(#cfg_attrs)*
      Self::#variant_ident => #prefix_lit.as_bytes(),
    });
    uri_arms.push(quote! {
      #(#cfg_attrs)*
      Self::#variant_ident => #uri_lit,
    });
    uri_bytes_arms.push(quote! {
      #(#cfg_attrs)*
      Self::#variant_ident => #uri_lit.as_bytes(),
    });
    from_uri_bytes_arms.push(quote! {
      #(#cfg_attrs)*
      #uri_bytes_lit => Some(Self::#variant_ident),
    });
  }

  Ok(quote! {
    impl #ident {
      pub const fn prefix_bytes(self) -> &'static [u8] {
        match self {
          #( #prefix_bytes_arms )*
        }
      }

      pub const fn prefix(self) -> &'static str {
        match self {
          #( #prefix_arms )*
        }
      }

      pub const fn uri_bytes(self) -> &'static [u8] {
        match self {
          #( #uri_bytes_arms )*
        }
      }

      pub const fn uri(self) -> &'static str {
        match self {
          #( #uri_arms )*
        }
      }

      pub fn from_uri(uri: &str) -> Option<Self> {
        Self::from_uri_bytes(uri.as_bytes())
      }

      pub fn from_uri_bytes(uri: &[u8]) -> Option<Self> {
        match uri {
          #( #from_uri_bytes_arms )*
          _ => None,
        }
      }
    }
  })
}

fn parse_sdk_xml_namespace_variant_attr(attrs: &[Attribute]) -> syn::Result<(String, String)> {
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }

    let values = attr.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)?;
    if values.len() != 2 {
      return Err(syn::Error::new_spanned(
        attr,
        "SdkXmlNamespace expects #[sdk(\"prefix\", \"uri\")]",
      ));
    }

    let mut values = values.into_iter();
    let prefix = values.next().expect("prefix").value();
    let uri = values.next().expect("uri").value();
    if prefix.is_empty() || uri.is_empty() {
      return Err(syn::Error::new_spanned(
        attr,
        "SdkXmlNamespace prefix and URI must be non-empty",
      ));
    }
    return Ok((prefix, uri));
  }

  Err(syn::Error::new(
    Span::call_site(),
    "SdkXmlNamespace variants require #[sdk(\"prefix\", \"uri\")]",
  ))
}
