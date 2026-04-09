use super::*;

pub(crate) fn expand_sdk_choice(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let Data::Enum(DataEnum { variants, .. }) = &input.data else {
    return Err(syn::Error::new_spanned(
      input,
      "SdkChoice can only be derived for enums",
    ));
  };

  let mut write_arms = Vec::with_capacity(variants.len());
  let mut read_arms = Vec::with_capacity(variants.len());

  for variant in variants {
    if variant.fields.len() != 1 {
      return Err(syn::Error::new_spanned(
        &variant.ident,
        "SdkChoice only supports single-field tuple variants",
      ));
    }

    let variant_ident = &variant.ident;
    let cfg_attrs = cfg_attrs(&variant.attrs);
    let kind = parse_sdk_choice_variant_kind(&variant.attrs)?.ok_or_else(|| {
      syn::Error::new_spanned(
        variant,
        "missing #[sdk(child(...))] or #[sdk(any)] on choice variant",
      )
    })?;
    let payload_ty = choice_variant_payload_type(variant)?;
    let inner_ty = choice_variant_inner_type(&payload_ty);

    match kind {
      SdkChoiceVariantKind::Child { qnames } => {
        let constructor = if is_box_type(&payload_ty) {
          quote! { Self::#variant_ident(std::boxed::Box::new(parsed_child)) }
        } else {
          quote! { Self::#variant_ident(parsed_child) }
        };
        let mut qname_patterns = Vec::with_capacity(qnames.len());
        for qname in qnames {
          let QNameInfo {
            tag_prefix,
            local_name,
          } = parse_qname_info(&qname);
          let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
          if tag_prefix.is_empty() {
            qname_patterns.push(quote! { #local_name_lit });
          } else {
            let tag_qname_lit = LitByteStr::new(
              format!("{tag_prefix}:{local_name}").as_bytes(),
              Span::call_site(),
            );
            qname_patterns.push(quote! { #tag_qname_lit | #local_name_lit });
          }
        }
        let read_arm = quote! {
          #( #qname_patterns )|* => {
            let parsed_child = #inner_ty::deserialize_inner(xml_reader, Some((e.clone(), empty_tag)))?;
            Ok(#constructor)
          }
        };
        read_arms.push(quote! { #(#cfg_attrs)* #read_arm });

        let write_arm = quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => value.write_xml(writer, xmlns_prefix),
        };
        write_arms.push(write_arm);
      }
      SdkChoiceVariantKind::Any => {
        let write_arm = quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => writer.write_str(value.as_ref()),
        };
        write_arms.push(write_arm);
        read_arms.push(quote! {
          #(#cfg_attrs)*
          _ => {
            let xml = crate::common::read_outer_xml(xml_reader, e, empty_tag)?;
            Ok(Self::#variant_ident(std::boxed::Box::new(xml)))
          }
        });
      }
    }
  }

  Ok(quote! {
    impl crate::sdk::SdkChoice for #ident {}

    impl #ident {
      pub(crate) fn deserialize_inner<'de, R: crate::common::XmlReader<'de>>(
        xml_reader: &mut R,
        xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
      ) -> Result<Self, crate::common::SdkError> {
        let (e, empty_tag) = if let Some((e, empty_tag)) = xml_event {
          (e, empty_tag)
        } else {
          loop {
            match xml_reader.next()? {
              quick_xml::events::Event::Start(e) => break (e, false),
              quick_xml::events::Event::Empty(e) => break (e, true),
              quick_xml::events::Event::Eof => {
                return Err(crate::common::unexpected_eof(stringify!(#ident)));
              }
              _ => continue,
            }
          }
        };

        match e.name().as_ref() {
          #( #read_arms )*
          _ => Err(crate::common::unexpected_tag(
            stringify!(#ident),
            "choice",
            e.name().as_ref(),
          )),
        }
      }

      pub(crate) fn write_xml<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        xmlns_prefix: &str,
      ) -> Result<(), std::fmt::Error> {
        match self {
          #( #write_arms )*
        }
      }
    }
  })
}
