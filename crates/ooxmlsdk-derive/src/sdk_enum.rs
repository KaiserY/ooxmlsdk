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
  let mut write_xml_attr_value_other_arm = None;
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

      let is_box_bytes = matches!(
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
                    syn::GenericArgument::Type(Type::Slice(slice))
                      if matches!(
                        slice.elem.as_ref(),
                        Type::Path(TypePath { path, .. })
                          if path.segments.len() == 1 && path.segments[0].ident == "u8"
                      )
                  )
            )
      );
      if !is_box_bytes {
        return Err(syn::Error::new_spanned(
          other_field_ty,
          "#[sdk(other)] variant must use Box<[u8]>",
        ));
      }

      as_xml_bytes_arms.push(quote! {
        #(#cfg_attrs)*
        Self::#variant_ident(value) => value.as_ref(),
      });
      write_xml_attr_value_other_arm = Some(quote! {
        #(#cfg_attrs)*
        Self::#variant_ident(value) => crate::common::write_escaped_bytes(writer, value),
      });
      other_arm = Some(quote! {
        #(#cfg_attrs)*
        other => Ok(Self::#variant_ident(Box::<[u8]>::from(other))),
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
    let xml_bytes_lit = LitByteStr::new(xml_value.as_bytes(), Span::call_site());

    as_xml_bytes_arms.push(quote! {
      #(#cfg_attrs)*
      Self::#variant_ident => #xml_bytes_lit,
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
      fn as_xml_bytes(&self) -> &[u8] {
        match self {
          #( #as_xml_bytes_arms )*
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
      #[inline]
      pub(crate) fn write_xml_attr_value<W: ::std::io::Write>(
        &self,
        writer: &mut W,
      ) -> ::std::io::Result<()> {
        match self {
          #write_xml_attr_value_other_arm
          _ => writer.write_all(<Self as crate::sdk::SdkEnum>::as_xml_bytes(self)),
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
        f.write_str(
          std::str::from_utf8(<Self as crate::sdk::SdkEnum>::as_xml_bytes(self))
            .map_err(|_| ::std::fmt::Error)?,
        )
      }
    }
  })
}
