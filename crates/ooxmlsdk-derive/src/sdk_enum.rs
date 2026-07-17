use super::*;

pub(crate) fn expand_sdk_enum(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let Data::Enum(DataEnum { variants, .. }) = &input.data else {
    return Err(syn::Error::new_spanned(
      input,
      "SdkEnum can only be derived for enums",
    ));
  };

  let mut as_xml_bytes_arms = Vec::with_capacity(variants.len());
  let mut from_bytes_arms = Vec::with_capacity(variants.len());

  for variant in variants {
    let variant_ident = &variant.ident;
    let cfg_attrs = cfg_attrs(&variant.attrs);
    let (rename, aliases) = parse_sdk_enum_variant_attrs(&variant.attrs, variant_ident)?;

    if !matches!(variant.fields, Fields::Unit) {
      return Err(syn::Error::new_spanned(
        &variant.ident,
        "SdkEnum only supports fieldless variants",
      ));
    }

    let xml_value = rename.unwrap_or_else(|| variant_ident.to_string());
    let xml_bytes_lit = LitByteStr::new(xml_value.as_bytes(), Span::call_site());

    as_xml_bytes_arms.push(quote! {
      #(#cfg_attrs)*
      Self::#variant_ident => #xml_bytes_lit,
    });
    from_bytes_arms.push(quote! {
      #(#cfg_attrs)*
      #xml_bytes_lit => Some(Self::#variant_ident),
    });

    for alias in aliases {
      let alias_bytes_lit = LitByteStr::new(alias.as_bytes(), Span::call_site());
      from_bytes_arms.push(quote! {
        #(#cfg_attrs)*
        #alias_bytes_lit => Some(Self::#variant_ident),
      });
    }
  }

  let ty_name = ident.to_string();

  Ok(quote! {
    impl crate::sdk::SdkEnum for #ident {
      const TYPE_NAME: &'static str = #ty_name;

      fn as_xml_bytes(&self) -> &[u8] {
        match self {
          #( #as_xml_bytes_arms )*
        }
      }

      #[inline]
      fn try_from_xml_bytes(b: &[u8]) -> Option<Self> {
        match b {
          #( #from_bytes_arms )*
          _ => None,
        }
      }
    }

    impl ::std::str::FromStr for #ident {
      type Err = crate::common::SdkError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as crate::sdk::SdkEnum>::from_xml_bytes(s.as_bytes())
      }
    }

    impl ::std::fmt::Display for #ident {
      fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let value = match std::str::from_utf8(<Self as crate::sdk::SdkEnum>::as_xml_bytes(self)) {
          Ok(value) => value,
          Err(_) => return Err(::std::fmt::Error),
        };
        f.write_str(value)
      }
    }
  })
}
