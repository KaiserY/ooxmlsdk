use super::*;

fn choice_qname_patterns(qnames: &[String]) -> Vec<proc_macro2::TokenStream> {
  let mut patterns = Vec::with_capacity(qnames.len());
  for qname in qnames {
    let QNameInfo {
      tag_prefix,
      local_name,
    } = parse_qname_info(qname);
    let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
    if tag_prefix.is_empty() {
      patterns.push(quote! { #local_name_lit });
    } else {
      let tag_qname_lit = LitByteStr::new(
        format!("{tag_prefix}:{local_name}").as_bytes(),
        Span::call_site(),
      );
      patterns.push(quote! { #tag_qname_lit | #local_name_lit });
    }
  }
  patterns
}

pub(crate) fn expand_sdk_choice(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let Data::Enum(DataEnum { variants, .. }) = &input.data else {
    return Err(syn::Error::new_spanned(
      input,
      "SdkChoice can only be derived for enums",
    ));
  };

  let mut write_arms = Vec::with_capacity(variants.len());
  let mut child_dispatch_tokens = Vec::with_capacity(variants.len());
  let mut any_dispatch_tokens = Vec::new();
  let mut text_from_string_tokens = Vec::new();

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
        "missing #[sdk(child(...))], #[sdk(text_child(...))] or #[sdk(any)] on choice variant",
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
        let qname_patterns = choice_qname_patterns(&qnames);
        let read_arm = quote! {
          if matches!(e.name().as_ref(), #( #qname_patterns )|*) {
            let parsed_child = #inner_ty::deserialize_inner(xml_reader, Some((e, empty_tag)))?;
            return Ok(#constructor);
          }
        };
        child_dispatch_tokens.push(quote! { #(#cfg_attrs)* #read_arm });

        let write_arm = quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => value.write_xml(writer, xmlns_prefix),
        };
        write_arms.push(write_arm);
      }
      SdkChoiceVariantKind::TextChild { qnames } => {
        let qname_patterns = choice_qname_patterns(&qnames);
        let QNameInfo {
          tag_prefix,
          local_name,
        } = parse_qname_info(qnames.first().map(String::as_str).unwrap_or_default());
        let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
        let tag_qname_lit = LitByteStr::new(
          format!("{tag_prefix}:{local_name}").as_bytes(),
          Span::call_site(),
        );
        let parse_from_text_tokens = if is_string_like_type(&payload_ty) {
          quote! { text.unwrap_or_default() }
        } else {
          quote! {{
            let value = text.unwrap_or_default();
            crate::common::parse_value::<#payload_ty>(
              &value,
              stringify!(#ident),
              stringify!(#variant_ident),
            )?
          }}
        };
        let empty_value_tokens = if is_string_like_type(&payload_ty) {
          quote! { Default::default() }
        } else {
          quote! {
            crate::common::parse_value::<#payload_ty>(
              "",
              stringify!(#ident),
              stringify!(#variant_ident),
            )?
          }
        };
        let read_arm = quote! {
          if matches!(e.name().as_ref(), #( #qname_patterns )|*) {
            let parsed_child = if empty_tag {
              #empty_value_tokens
            } else {
              let mut text = None;
              loop {
                match xml_reader.next()? {
                  quick_xml::events::Event::Text(t) => {
                    crate::common::push_xml_text(&mut text, t)?;
                  }
                  quick_xml::events::Event::GeneralRef(t) => {
                    crate::common::push_xml_general_ref(
                      &mut text,
                      t,
                      stringify!(#ident),
                      stringify!(#variant_ident),
                    )?;
                  }
                  quick_xml::events::Event::End(end) => {
                    if end.name().as_ref() == #tag_qname_lit
                      || end.name().as_ref() == #local_name_lit
                    {
                      break;
                    }
                  }
                  quick_xml::events::Event::Eof => {
                    return Err(crate::common::unexpected_eof(stringify!(#ident)));
                  }
                  _ => {}
                }
              }
              #parse_from_text_tokens
            };
            return Ok(Self::#variant_ident(parsed_child));
          }
        };
        child_dispatch_tokens.push(quote! { #(#cfg_attrs)* #read_arm });
        let write_arm = quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => {
            crate::common::write_start_tag_open(writer, xmlns_prefix, #tag_prefix, #local_name)?;
            writer.write_char('>')?;
            crate::common::write_escaped_text(writer, value)?;
            crate::common::write_end_tag(writer, xmlns_prefix, #tag_prefix, #local_name)
          }
        };
        write_arms.push(write_arm);
      }
      SdkChoiceVariantKind::Any => {
        let constructor = if is_box_type(&payload_ty) {
          quote! { Self::#variant_ident(std::boxed::Box::new(xml)) }
        } else {
          quote! { Self::#variant_ident(xml) }
        };
        let write_arm = quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => writer.write_str(value.as_ref()),
        };
        write_arms.push(write_arm);
        any_dispatch_tokens.push(quote! {
          #(#cfg_attrs)*
          {
            let xml = crate::common::read_outer_xml(xml_reader, e, empty_tag)?;
            return Ok(#constructor);
          }
        });
      }
      SdkChoiceVariantKind::Text => {
        let write_arm = quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => crate::common::write_escaped_text(writer, value),
        };
        write_arms.push(write_arm);
        text_from_string_tokens.push(quote! {
          #(#cfg_attrs)*
          return Some(Self::#variant_ident(value.to_owned().into()));
        });
      }
    }
  }

  let read_tokens = if any_dispatch_tokens.is_empty() {
    quote! {
      #( #child_dispatch_tokens )*
      {
        let found = e.name();
        Err(crate::common::unexpected_tag(
          stringify!(#ident),
          "choice",
          found.as_ref(),
        ))
      }
    }
  } else {
    quote! {
      #( #child_dispatch_tokens )*
      #( #any_dispatch_tokens )*
    }
  };

  Ok(quote! {
    impl crate::sdk::SdkChoice for #ident {}
    #[cfg(feature = "validators")]
    impl crate::validator::SdkValidator for #ident {}

    impl #ident {
      #[allow(unreachable_code)]
      pub(crate) fn from_text_value(value: &str) -> Option<Self> {
        #( #text_from_string_tokens )*
        None
      }

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
        #read_tokens
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
