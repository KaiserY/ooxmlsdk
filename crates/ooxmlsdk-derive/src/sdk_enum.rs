use super::*;

pub(crate) fn expand_sdk_enum(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let Data::Enum(DataEnum { variants, .. }) = &input.data else {
    return Err(syn::Error::new_spanned(
      input,
      "SdkEnum can only be derived for enums",
    ));
  };

  let mut as_xml_arms = Vec::with_capacity(variants.len());
  let mut from_bytes_arms = Vec::with_capacity(variants.len());
  let mut other_arm = None;

  for variant in variants {
    let variant_ident = &variant.ident;
    let cfg_attrs = cfg_attrs(&variant.attrs);
    let (rename, aliases, is_other) = parse_sdk_enum_variant_attrs(&variant.attrs, variant_ident)?;

    if is_other {
      if other_arm.is_some() {
        return Err(syn::Error::new_spanned(
          &variant.ident,
          "SdkEnum supports at most one #[sdk(other)] variant",
        ));
      }

      let other_field_ty = match &variant.fields {
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => &fields.unnamed[0].ty,
        _ => {
          return Err(syn::Error::new_spanned(
            &variant.ident,
            "#[sdk(other)] variant must be a single-field tuple variant",
          ));
        }
      };

      let is_box_str = matches!(
        other_field_ty,
        Type::Path(TypePath { path, .. })
          if path.segments.len() == 1
            && path.segments[0].ident == "Box"
            && matches!(
              &path.segments[0].arguments,
              syn::PathArguments::AngleBracketed(args)
                if args.args.len() == 1
                  && matches!(
                    &args.args[0],
                    syn::GenericArgument::Type(Type::Path(TypePath { path, .. }))
                      if path.segments.len() == 1 && path.segments[0].ident == "str"
                  )
            )
      );
      if !is_box_str {
        return Err(syn::Error::new_spanned(
          other_field_ty,
          "#[sdk(other)] variant must use Box<str>",
        ));
      }

      as_xml_arms.push(quote! {
        #(#cfg_attrs)*
        Self::#variant_ident(value) => value.as_ref(),
      });
      other_arm = Some(quote! {
        #(#cfg_attrs)*
        other => Ok(Self::#variant_ident(String::from_utf8_lossy(other).into_owned().into_boxed_str())),
      });
      continue;
    }

    if !matches!(variant.fields, Fields::Unit) {
      return Err(syn::Error::new_spanned(
        &variant.ident,
        "SdkEnum only supports fieldless variants unless marked #[sdk(other)]",
      ));
    }

    let xml_value = rename.unwrap_or_else(|| variant_ident.to_string());
    let xml_value_lit = LitStr::new(&xml_value, Span::call_site());
    let xml_bytes_lit = LitByteStr::new(xml_value.as_bytes(), Span::call_site());

    as_xml_arms.push(quote! {
      #(#cfg_attrs)*
      Self::#variant_ident => #xml_value_lit,
    });
    from_bytes_arms.push(quote! {
      #(#cfg_attrs)*
      #xml_bytes_lit => Ok(Self::#variant_ident),
    });

    for alias in aliases {
      let alias_bytes_lit = LitByteStr::new(alias.as_bytes(), Span::call_site());
      from_bytes_arms.push(quote! {
        #(#cfg_attrs)*
        #alias_bytes_lit => Ok(Self::#variant_ident),
      });
    }
  }

  let ty_name = ident.to_string();
  let fallback_arm = if let Some(other_arm) = other_arm {
    other_arm
  } else {
    quote! {
      other => Err(crate::common::invalid_enum_value(
        #ty_name,
        String::from_utf8_lossy(other).into_owned(),
      )),
    }
  };

  Ok(quote! {
    impl crate::sdk::SdkEnum for #ident {
      fn as_xml_str(&self) -> &str {
        match self {
          #( #as_xml_arms )*
        }
      }

      fn from_xml_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
        match b {
          #( #from_bytes_arms )*
          #fallback_arm
        }
      }
    }

    impl #ident {
      pub fn as_xml_str(&self) -> &str {
        <Self as crate::sdk::SdkEnum>::as_xml_str(self)
      }

      pub fn to_xml(&self) -> String {
        self.as_xml_str().to_string()
      }

      pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
        <Self as crate::sdk::SdkEnum>::from_xml_bytes(b)
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
        f.write_str(self.as_xml_str())
      }
    }
  })
}
