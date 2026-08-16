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

  let mut prefixes = Vec::with_capacity(variants.len());
  let mut from_prefix_bytes_arms = Vec::with_capacity(variants.len());
  let mut uris = Vec::with_capacity(variants.len());
  let mut from_uri_bytes_arms = Vec::with_capacity(variants.len());
  let mut compatible_uri_arms = Vec::new();
  let mut prefix_uri_arms =
    std::collections::BTreeMap::<String, Vec<proc_macro2::TokenStream>>::new();
  let mut seen_prefixes = std::collections::HashSet::new();
  let mut seen_uris = std::collections::HashSet::new();

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
    let prefix_bytes_lit = LitByteStr::new(prefix.as_bytes(), Span::call_site());
    let uri_lit = LitStr::new(&uri, Span::call_site());
    let uri_bytes_lit = LitByteStr::new(uri.as_bytes(), Span::call_site());
    let alias_bytes_lits = parse_sdk_xml_namespace_alias_attrs(&variant.attrs)?
      .into_iter()
      .map(|alias| LitByteStr::new(alias.as_bytes(), Span::call_site()))
      .collect::<Vec<_>>();

    prefixes.push(quote! {
      #(#cfg_attrs)*
      #prefix_lit,
    });
    if seen_prefixes.insert(prefix.clone()) {
      from_prefix_bytes_arms.push(quote! {
        #(#cfg_attrs)*
        #prefix_bytes_lit => Some(Self::#variant_ident),
      });
    }
    uris.push(quote! {
      #(#cfg_attrs)*
      #uri_lit,
    });
    if seen_uris.insert(uri.clone()) {
      from_uri_bytes_arms.push(quote! {
        #(#cfg_attrs)*
        #uri_bytes_lit => Some(Self::#variant_ident),
      });
    }
    prefix_uri_arms.entry(prefix).or_default().push(quote! {
      #(#cfg_attrs)*
      #uri_bytes_lit #( | #alias_bytes_lits )* => Some(Self::#variant_ident),
    });
    for alias_lit in alias_bytes_lits {
      compatible_uri_arms.push(quote! {
        #(#cfg_attrs)*
        #alias_lit => Some(Self::#variant_ident),
      });
    }
  }

  let prefix_uri_arms = prefix_uri_arms.into_iter().map(|(prefix, uri_arms)| {
    let prefix_lit = LitByteStr::new(prefix.as_bytes(), Span::call_site());
    quote! {
      #prefix_lit => match uri {
        #( #uri_arms )*
        _ => Self::from_compatible_uri_bytes(uri),
      },
    }
  });
  let table_name = ident.to_string().to_ascii_uppercase();
  let prefixes_ident = Ident::new(
    &format!("__OOXMLSDK_{table_name}_PREFIXES"),
    Span::call_site(),
  );
  let uris_ident = Ident::new(&format!("__OOXMLSDK_{table_name}_URIS"), Span::call_site());

  Ok(quote! {
    static #prefixes_ident: &[&str] = &[
      #( #prefixes )*
    ];
    static #uris_ident: &[&str] = &[
      #( #uris )*
    ];

    impl #ident {
      pub const fn prefix_bytes(self) -> &'static [u8] {
        #prefixes_ident[self as usize].as_bytes()
      }

      pub const fn prefix(self) -> &'static str {
        #prefixes_ident[self as usize]
      }

      pub const fn uri_bytes(self) -> &'static [u8] {
        #uris_ident[self as usize].as_bytes()
      }

      pub const fn uri(self) -> &'static str {
        #uris_ident[self as usize]
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

      #[doc(hidden)]
      #[cold]
      #[inline(never)]
      pub(crate) fn from_compatible_uri_bytes(uri: &[u8]) -> Option<Self> {
        match uri {
          #( #from_uri_bytes_arms )*
          #( #compatible_uri_arms )*
          _ => None,
        }
      }

      #[doc(hidden)]
      #[inline]
      pub(crate) fn from_prefix_uri_bytes(prefix: &[u8], uri: &[u8]) -> Option<Self> {
        if let Some(namespace) = Self::from_prefix_bytes(prefix)
          && namespace.uri_bytes() == uri
        {
          return Some(namespace);
        }
        Self::from_noncanonical_prefix_uri_bytes(prefix, uri)
      }

      #[cold]
      #[inline(never)]
      fn from_noncanonical_prefix_uri_bytes(prefix: &[u8], uri: &[u8]) -> Option<Self> {
        match prefix {
          #( #prefix_uri_arms )*
          _ => Self::from_compatible_uri_bytes(uri),
        }
      }

      #[doc(hidden)]
      pub(crate) fn from_prefix_bytes(prefix: &[u8]) -> Option<Self> {
        match prefix {
          #( #from_prefix_bytes_arms )*
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

fn parse_sdk_xml_namespace_alias_attrs(attrs: &[Attribute]) -> syn::Result<Vec<String>> {
  attrs
    .iter()
    .filter(|attr| attr.path().is_ident("sdk_alias"))
    .map(|attr| {
      let alias = attr.parse_args::<LitStr>()?.value();
      if alias.is_empty() {
        Err(syn::Error::new_spanned(
          attr,
          "SdkXmlNamespace aliases must be non-empty",
        ))
      } else {
        Ok(alias)
      }
    })
    .collect()
}
