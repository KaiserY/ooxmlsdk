use super::*;

#[derive(Clone)]
enum NamedSequenceVariantFieldKind {
  Child { qname: String },
  TextChild { qname: String },
}

#[derive(Clone)]
struct NamedSequenceVariantField {
  ident: Ident,
  ty: Type,
  kind: NamedSequenceVariantFieldKind,
  optional: bool,
  repeated: bool,
  boxed: bool,
}

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

fn named_sequence_helper_ident(enum_ident: &Ident, variant_ident: &Ident) -> syn::Result<Ident> {
  parse_str(&format!("__{}{}SequenceHelper", enum_ident, variant_ident))
}

fn parse_named_sequence_variant_fields(
  _variant: &syn::Variant,
  fields: &syn::FieldsNamed,
) -> syn::Result<Vec<NamedSequenceVariantField>> {
  if fields.named.is_empty() || fields.named.len() > 2 {
    return Err(syn::Error::new_spanned(
      fields,
      "SdkChoice named #[sdk(sequence)] variants currently support 1 or 2 #[sdk(child(...))] or #[sdk(text_child(...))] fields",
    ));
  }

  let mut parsed = Vec::with_capacity(fields.named.len());
  for field in &fields.named {
    let field_ident = field
      .ident
      .clone()
      .ok_or_else(|| syn::Error::new_spanned(field, "named field ident"))?;
    let parsed_attrs = parse_sdk_type_field_attrs(&field.attrs)?;
    let Some(field_kind) = parsed_attrs.kind else {
      return Err(syn::Error::new_spanned(
        field,
        "named #[sdk(sequence)] variants require #[sdk(child(...))] fields",
      ));
    };
    let kind = match field_kind {
      SdkTypeFieldKind::Child { qname } => NamedSequenceVariantFieldKind::Child { qname },
      SdkTypeFieldKind::TextChild { qname } => NamedSequenceVariantFieldKind::TextChild { qname },
      _ => {
        return Err(syn::Error::new_spanned(
          field,
          "named #[sdk(sequence)] variants currently support only #[sdk(child(...))] or #[sdk(text_child(...))] fields",
        ));
      }
    };

    parsed.push(NamedSequenceVariantField {
      ident: field_ident,
      ty: field.ty.clone(),
      kind,
      optional: is_option_type(&field.ty),
      repeated: contains_vec_type(&field.ty),
      boxed: box_inner_type(&unwrap_option_vec_type(&field.ty)).is_some(),
    });
  }

  Ok(parsed)
}

fn named_sequence_write_tokens(field: &NamedSequenceVariantField) -> proc_macro2::TokenStream {
  let field_ident = &field.ident;
  let inner_ty = unwrap_wrapped_type(&field.ty);

  match &field.kind {
    NamedSequenceVariantFieldKind::Child { .. } => {
      if field.repeated {
        quote! {
          for child in #field_ident {
            child.write_xml(writer, xmlns_prefix)?;
          }
        }
      } else if field.optional {
        quote! {
          if let Some(child) = #field_ident {
            child.write_xml(writer, xmlns_prefix)?;
          }
        }
      } else {
        quote! {
          #field_ident.write_xml(writer, xmlns_prefix)?;
        }
      }
    }
    NamedSequenceVariantFieldKind::TextChild { qname } => {
      let QNameInfo {
        tag_prefix,
        local_name,
      } = parse_qname_info(qname);
      let write_value_tokens = |value_expr: proc_macro2::TokenStream| {
        let value_write_tokens = if is_xml_schema_float_type(&inner_ty) {
          write_xml_schema_float_tokens(value_expr.clone(), &inner_ty)
        } else if is_bool_type(&inner_ty) {
          write_bool_tokens(value_expr.clone(), &inner_ty)
        } else if is_string_like_type(&inner_ty) {
          quote! {
            crate::common::write_escaped_str(writer, #value_expr.as_ref())?;
          }
        } else {
          quote! {
            crate::common::write_escaped_text(writer, #value_expr)?;
          }
        };
        quote! {
          crate::common::write_start_tag_open(writer, xmlns_prefix, #tag_prefix, #local_name)?;
          writer.write_all(b">")?;
          #value_write_tokens
          crate::common::write_end_tag(writer, xmlns_prefix, #tag_prefix, #local_name)?;
        }
      };

      if field.repeated {
        let write_child_tokens = write_value_tokens(quote! { child });
        quote! {
          for child in #field_ident {
            #write_child_tokens
          }
        }
      } else if field.optional {
        let write_child_tokens = write_value_tokens(quote! { value });
        quote! {
          if let Some(value) = #field_ident {
            #write_child_tokens
          }
        }
      } else {
        write_value_tokens(quote! { #field_ident })
      }
    }
  }
}

fn named_sequence_validate_tokens(field: &NamedSequenceVariantField) -> proc_macro2::TokenStream {
  let field_ident = &field.ident;
  if matches!(field.kind, NamedSequenceVariantFieldKind::TextChild { .. }) {
    return quote! {};
  }
  let validate_expr = if field.boxed {
    quote! { child.as_ref() }
  } else {
    quote! { child }
  };
  let validate_self_expr = if field.boxed {
    quote! { #field_ident.as_ref() }
  } else {
    quote! { #field_ident }
  };

  if field.repeated {
    quote! {
      for child in #field_ident {
        crate::validator::SdkValidator::validate(#validate_expr)?;
      }
    }
  } else if field.optional {
    quote! {
      if let Some(child) = #field_ident {
        crate::validator::SdkValidator::validate(#validate_expr)?;
      }
    }
  } else {
    quote! {
      crate::validator::SdkValidator::validate(#validate_self_expr)?;
    }
  }
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
  let mut direct_matcher_arms = Vec::with_capacity(variants.len());
  let mut helper_matcher_checks = Vec::new();
  let mut direct_child_dispatch_arms = Vec::with_capacity(variants.len());
  let mut helper_child_dispatch_tokens = Vec::new();
  let mut any_dispatch_tokens = Vec::new();
  let mut text_from_string_tokens = Vec::new();
  let mut validate_arms = Vec::new();
  let mut has_any_variant = false;
  let mut helper_items = Vec::new();

  for variant in variants {
    let variant_ident = &variant.ident;
    let cfg_attrs = cfg_attrs(&variant.attrs);
    let kind = parse_sdk_choice_variant_kind(&variant.attrs)?.ok_or_else(|| {
      syn::Error::new_spanned(
        variant,
        "missing #[sdk(child(...))], #[sdk(text_child(...))], #[sdk(choice)], #[sdk(sequence)] or #[sdk(any)] on choice variant",
      )
    })?;
    match (&variant.fields, kind) {
      (Fields::Unnamed(fields), SdkChoiceVariantKind::Child { qnames })
        if fields.unnamed.len() == 1 =>
      {
        let payload_ty = choice_variant_payload_type(variant)?;
        let inner_ty = choice_variant_inner_type(&payload_ty);
        let constructor = if is_box_type(&payload_ty) {
          quote! { Self::#variant_ident(std::boxed::Box::new(parsed_child)) }
        } else {
          quote! { Self::#variant_ident(parsed_child) }
        };
        let qname_patterns = choice_qname_patterns(&qnames);
        direct_matcher_arms.push(quote! {
          #(#cfg_attrs)*
          #( #qname_patterns )|* => true,
        });
        direct_child_dispatch_arms.push(quote! {
          #(#cfg_attrs)*
          #( #qname_patterns )|* => {
            let parsed_child = #inner_ty::deserialize_inner(xml_reader, Some((e, empty_tag)))?;
            return Ok(#constructor);
          }
        });

        let write_arm = quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => value.write_xml(writer, xmlns_prefix),
        };
        write_arms.push(write_arm);
        let validate_arm = if is_box_type(&payload_ty) {
          quote! {
            #(#cfg_attrs)*
            Self::#variant_ident(value) => crate::validator::SdkValidator::validate(value.as_ref()),
          }
        } else {
          quote! {
            #(#cfg_attrs)*
            Self::#variant_ident(value) => crate::validator::SdkValidator::validate(value),
          }
        };
        validate_arms.push(validate_arm);
      }
      (Fields::Unnamed(fields), SdkChoiceVariantKind::Choice | SdkChoiceVariantKind::Sequence)
        if fields.unnamed.len() == 1 =>
      {
        let payload_ty = choice_variant_payload_type(variant)?;
        let inner_ty = choice_variant_inner_type(&payload_ty);
        let constructor = if is_box_type(&payload_ty) {
          quote! { Self::#variant_ident(std::boxed::Box::new(parsed_child)) }
        } else {
          quote! { Self::#variant_ident(parsed_child) }
        };
        helper_matcher_checks.push(quote! {
          #(#cfg_attrs)*
          if #inner_ty::matches_start_qname(name) {
            return true;
          }
        });
        helper_child_dispatch_tokens.push(quote! {
          #(#cfg_attrs)*
          if #inner_ty::matches_start_qname(e.name().as_ref()) {
            let parsed_child = #inner_ty::deserialize_inner(xml_reader, Some((e, empty_tag)))?;
            return Ok(#constructor);
          }
        });
        let write_arm = quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => value.write_xml(writer, xmlns_prefix),
        };
        write_arms.push(write_arm);
        let validate_arm = if is_box_type(&payload_ty) {
          quote! {
            #(#cfg_attrs)*
            Self::#variant_ident(value) => crate::validator::SdkValidator::validate(value.as_ref()),
          }
        } else {
          quote! {
            #(#cfg_attrs)*
            Self::#variant_ident(value) => crate::validator::SdkValidator::validate(value),
          }
        };
        validate_arms.push(validate_arm);
      }
      (Fields::Named(fields), SdkChoiceVariantKind::Sequence) => {
        let helper_ident = named_sequence_helper_ident(ident, variant_ident)?;
        let named_fields = parse_named_sequence_variant_fields(variant, fields)?;
        let field_idents: Vec<_> = named_fields
          .iter()
          .map(|field| field.ident.clone())
          .collect();
        let write_pattern_fields = named_fields
          .iter()
          .map(|field| {
            let ident = &field.ident;
            quote! { #ident }
          })
          .collect::<Vec<_>>();
        let validate_pattern_fields = named_fields
          .iter()
          .map(|field| {
            let ident = &field.ident;
            if matches!(field.kind, NamedSequenceVariantFieldKind::TextChild { .. }) {
              quote! { #ident: _ }
            } else {
              quote! { #ident }
            }
          })
          .collect::<Vec<_>>();
        let helper_fields = fields.named.iter().collect::<Vec<_>>();
        let write_tokens = named_fields
          .iter()
          .map(named_sequence_write_tokens)
          .collect::<Vec<_>>();
        let validate_tokens = named_fields
          .iter()
          .map(named_sequence_validate_tokens)
          .collect::<Vec<_>>();

        helper_items.push(quote! {
          #( #cfg_attrs )*
          #[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
          struct #helper_ident {
            #( #helper_fields, )*
          }
        });
        let start_qname = match &named_fields[0].kind {
          NamedSequenceVariantFieldKind::Child { qname }
          | NamedSequenceVariantFieldKind::TextChild { qname } => qname,
        };
        let start_qname_patterns = choice_qname_patterns(std::slice::from_ref(start_qname));
        direct_matcher_arms.push(quote! {
          #(#cfg_attrs)*
          #( #start_qname_patterns )|* => true,
        });
        direct_child_dispatch_arms.push(quote! {
          #(#cfg_attrs)*
          #( #start_qname_patterns )|* => {
            let parsed_child = #helper_ident::deserialize_inner(xml_reader, Some((e, empty_tag)))?;
            let #helper_ident { #( #field_idents ),* } = parsed_child;
            return Ok(Self::#variant_ident { #( #field_idents ),* });
          }
        });
        write_arms.push(quote! {
          #(#cfg_attrs)*
          Self::#variant_ident { #( #write_pattern_fields ),* } => {
            #( #write_tokens )*
            Ok(())
          },
        });
        validate_arms.push(quote! {
          #(#cfg_attrs)*
          Self::#variant_ident { #( #validate_pattern_fields ),* } => {
            #( #validate_tokens )*
            Ok(())
          },
        });
      }
      (Fields::Unnamed(fields), SdkChoiceVariantKind::TextChild { qnames })
        if fields.unnamed.len() == 1 =>
      {
        let payload_ty = choice_variant_payload_type(variant)?;
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
        } else if is_bool_type(&payload_ty) {
          let parse_value_tokens = parse_bool_tokens(
            quote! { &value },
            &payload_ty,
            quote! { stringify!(#ident) },
            quote! { stringify!(#variant_ident) },
          );
          quote! {{
            let value = text.unwrap_or_default();
            #parse_value_tokens
          }}
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
        } else if is_bool_type(&payload_ty) {
          parse_bool_tokens(
            quote! { "" },
            &payload_ty,
            quote! { stringify!(#ident) },
            quote! { stringify!(#variant_ident) },
          )
        } else {
          quote! {
            crate::common::parse_value::<#payload_ty>(
              "",
              stringify!(#ident),
              stringify!(#variant_ident),
            )?
          }
        };
        direct_matcher_arms.push(quote! {
          #(#cfg_attrs)*
          #( #qname_patterns )|* => true,
        });
        direct_child_dispatch_arms.push(quote! {
          #(#cfg_attrs)*
          #( #qname_patterns )|* => {
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
        });
        let value_write_tokens = if is_xml_schema_float_type(&payload_ty) {
          write_xml_schema_float_tokens(quote! { value }, &payload_ty)
        } else if is_bool_type(&payload_ty) {
          write_bool_tokens(quote! { value }, &payload_ty)
        } else if is_string_like_type(&payload_ty) {
          quote! {
            crate::common::write_escaped_str(writer, value.as_ref())?;
          }
        } else {
          quote! {
            crate::common::write_escaped_text(writer, value)?;
          }
        };
        let write_arm = quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => {
            crate::common::write_start_tag_open(writer, xmlns_prefix, #tag_prefix, #local_name)?;
            writer.write_all(b">")?;
            #value_write_tokens
            crate::common::write_end_tag(writer, xmlns_prefix, #tag_prefix, #local_name)
          }
        };
        write_arms.push(write_arm);
        validate_arms.push(quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(_) => Ok(()),
        });
      }
      (Fields::Unnamed(fields), SdkChoiceVariantKind::Any) if fields.unnamed.len() == 1 => {
        let payload_ty = choice_variant_payload_type(variant)?;
        has_any_variant = true;
        let constructor = if is_box_type(&payload_ty) {
          quote! { Self::#variant_ident(std::boxed::Box::new(xml)) }
        } else {
          quote! { Self::#variant_ident(xml) }
        };
        let write_arm = quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => writer.write_all(value.as_bytes()),
        };
        write_arms.push(write_arm);
        validate_arms.push(quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(_) => Ok(()),
        });
        any_dispatch_tokens.push(quote! {
          #(#cfg_attrs)*
          {
            let xml = crate::common::read_outer_xml(xml_reader, e, empty_tag)?;
            return Ok(#constructor);
          }
        });
      }
      (Fields::Unnamed(fields), SdkChoiceVariantKind::Text) if fields.unnamed.len() == 1 => {
        let payload_ty = choice_variant_payload_type(variant)?;
        let value_write_tokens = if is_xml_schema_float_type(&payload_ty) {
          write_xml_schema_float_tokens(quote! { value }, &payload_ty)
        } else if is_bool_type(&payload_ty) {
          write_bool_tokens(quote! { value }, &payload_ty)
        } else if is_string_like_type(&payload_ty) {
          quote! {
            crate::common::write_escaped_str(writer, value.as_ref())?;
          }
        } else {
          quote! {
            crate::common::write_escaped_text(writer, value)?;
          }
        };
        let write_arm = quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => {
            #value_write_tokens
            Ok(())
          },
        };
        write_arms.push(write_arm);
        validate_arms.push(quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(_) => Ok(()),
        });
        text_from_string_tokens.push(quote! {
          #(#cfg_attrs)*
          parsed = Some(Self::#variant_ident(value.to_owned().into()));
        });
      }
      _ => {
        return Err(syn::Error::new_spanned(
          variant,
          "SdkChoice supports single-field tuple variants, plus named #[sdk(sequence)] variants with 1 or 2 #[sdk(child(...))] or #[sdk(text_child(...))] fields",
        ));
      }
    }
  }

  let matches_specific_start_qname_tokens = quote! {
    match name {
      #( #direct_matcher_arms )*
      _ => {
        #( #helper_matcher_checks )*
        false
      }
    }
  };
  let matches_start_qname_tokens = if has_any_variant {
    quote! {
      if Self::matches_specific_start_qname(name) {
        true
      } else {
        true
      }
    }
  } else {
    quote! {
      Self::matches_specific_start_qname(name)
    }
  };
  let from_text_value_tokens = if text_from_string_tokens.is_empty() {
    quote! { None }
  } else {
    quote! {
      let mut parsed = None;
      #( #text_from_string_tokens )*
      parsed
    }
  };

  let read_tokens = if any_dispatch_tokens.is_empty() {
    quote! {
      match e.name().as_ref() {
        #( #direct_child_dispatch_arms )*
        _ => {
          #( #helper_child_dispatch_tokens )*
          let found = e.name();
          Err(crate::common::unexpected_tag(
            stringify!(#ident),
            "choice",
            found.as_ref(),
          ))
        }
      }
    }
  } else {
    quote! {
      match e.name().as_ref() {
        #( #direct_child_dispatch_arms )*
        _ => {
          #( #helper_child_dispatch_tokens )*
          #( #any_dispatch_tokens )*
        }
      }
    }
  };

  Ok(quote! {
    #( #helper_items )*

    impl crate::sdk::SdkChoice for #ident {}
    #[cfg(feature = "validators")]
    impl crate::validator::SdkValidator for #ident {
      fn validate(&self) -> Result<(), crate::common::SdkError> {
        match self {
          #( #validate_arms )*
        }
      }
    }

    impl #ident {
      #[inline]
      pub(crate) fn matches_specific_start_qname(name: &[u8]) -> bool {
        #matches_specific_start_qname_tokens
      }

      #[inline]
      pub(crate) fn matches_start_qname(name: &[u8]) -> bool {
        #matches_start_qname_tokens
      }

      #[inline]
      pub(crate) const fn accepts_any_child() -> bool {
        #has_any_variant
      }

      pub(crate) fn from_text_value(value: &str) -> Option<Self> {
        #from_text_value_tokens
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

      pub(crate) fn write_xml<W: std::io::Write>(
        &self,
        writer: &mut W,
        xmlns_prefix: &str,
      ) -> Result<(), std::io::Error> {
        match self {
          #( #write_arms )*
        }
      }
    }
  })
}
