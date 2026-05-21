use super::*;

#[derive(Clone, Copy)]
enum DeserializeMode {
  Borrowed,
  Io,
}

impl DeserializeMode {
  fn deserialize_inner_ident(self) -> Ident {
    match self {
      Self::Borrowed => Ident::new("deserialize_borrowed_inner", Span::call_site()),
      Self::Io => Ident::new("deserialize_io_inner", Span::call_site()),
    }
  }

  fn tag_event_ty(self) -> proc_macro2::TokenStream {
    match self {
      Self::Borrowed => quote! { crate::common::SliceTagEvent },
      Self::Io => quote! { crate::common::IoTagEvent },
    }
  }

  fn skip_foreign_element_children_tokens(self) -> proc_macro2::TokenStream {
    match self {
      Self::Borrowed => quote! {
        crate::common::skip_foreign_element_children_borrowed(
          xml_reader,
          next_empty,
        )?;
      },
      Self::Io => quote! {
        crate::common::skip_foreign_element_children_io(
          xml_reader,
          next_empty,
        )?;
      },
    }
  }

  fn read_outer_xml_fn(self) -> proc_macro2::TokenStream {
    match self {
      Self::Borrowed => quote! { crate::common::read_outer_xml_borrowed },
      Self::Io => quote! { crate::common::read_outer_xml_io },
    }
  }
}

fn write_typed_child_tokens(
  child_ty: &syn::Type,
  value: proc_macro2::TokenStream,
  qname: &str,
) -> proc_macro2::TokenStream {
  let QNameInfo {
    tag_prefix,
    local_name,
  } = parse_qname_info(qname);
  let tag_prefix_lit = LitByteStr::new(tag_prefix.as_bytes(), Span::call_site());
  let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
  quote! {
    <#child_ty as crate::sdk::SdkType>::write_inner(
      #value,
      writer,
      xmlns_prefix,
      crate::sdk::ElementName::new(#tag_prefix_lit, #local_name_lit),
    )?;
  }
}

fn write_typed_child_element_name_tokens(
  child_ty: &syn::Type,
  value: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  quote! {
    <#child_ty as crate::sdk::SdkType>::write_inner(
      #value,
      writer,
      xmlns_prefix,
      <#child_ty as crate::sdk::SdkType>::ELEMENT_NAME,
    )?;
  }
}

fn deserialize_choice_inner_ident(mode: DeserializeMode) -> Ident {
  mode.deserialize_inner_ident()
}

#[derive(Clone)]
enum NamedSequenceVariantFieldKind {
  Child {
    qname: String,
  },
  EmptyChild {
    qname: String,
  },
  TextChild {
    qname: String,
    simple_type: Option<String>,
  },
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

fn choice_qname_patterns(
  qnames: &[String],
  local_name_fallback_qnames: &std::collections::HashSet<String>,
) -> Vec<proc_macro2::TokenStream> {
  let mut patterns = Vec::with_capacity(qnames.len());
  for qname in qnames {
    let QNameInfo {
      tag_prefix,
      local_name,
    } = parse_qname_info(qname);
    let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
    let allows_local_name = local_name_fallback_qnames.contains(qname);
    if tag_prefix.is_empty() {
      patterns.push(quote! { #local_name_lit });
    } else if !allows_local_name {
      let tag_qname_lit = LitByteStr::new(
        format!("{tag_prefix}:{local_name}").as_bytes(),
        Span::call_site(),
      );
      patterns.push(quote! { #tag_qname_lit });
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

fn empty_child_skip_tokens(
  ident: &Ident,
  mode: DeserializeMode,
  qnames: &[String],
  local_name_fallback_qnames: &std::collections::HashSet<String>,
) -> proc_macro2::TokenStream {
  let qname_patterns = choice_qname_patterns(qnames, local_name_fallback_qnames);
  let first_qname = qnames.first().map(String::as_str).unwrap_or("");
  let QNameInfo { tag_prefix, .. } = parse_qname_info(first_qname);
  let skip_foreign_children = mode.skip_foreign_element_children_tokens();
  let next_tag_event = mode.tag_event_ty();

  quote! {
    if !empty_tag {
      loop {
        match xml_reader.next_tag_event()? {
          #next_tag_event::Start(e, next_empty) => {
            let event_name = e.name();
            let event_name = event_name.as_ref();
            if crate::common::is_foreign_prefixed_child(event_name, #tag_prefix) {
              #skip_foreign_children
            } else {
              return Err(crate::common::unexpected_tag(
                stringify!(#ident),
                "empty child",
                event_name,
              ));
            }
          }
          #next_tag_event::End(e) => {
            match e.name().as_ref() {
              #( #qname_patterns )|* => break,
              _ => {}
            }
          }
          #next_tag_event::Eof => {
            return Err(crate::common::unexpected_eof(stringify!(#ident)));
          }
          #next_tag_event::Decl(_) | #next_tag_event::Other => {}
        }
      }
    }
  }
}

fn choice_child_local_name_fallback_qnames(
  variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> syn::Result<std::collections::HashSet<String>> {
  let mut local_name_qnames = std::collections::HashMap::<String, Vec<String>>::new();
  for variant in variants {
    match parse_sdk_choice_variant_kind(&variant.attrs)? {
      Some(SdkChoiceVariantKind::Child { qnames })
      | Some(SdkChoiceVariantKind::EmptyChild { qnames })
      | Some(SdkChoiceVariantKind::AnyChild { qnames })
      | Some(SdkChoiceVariantKind::TextChild { qnames, .. }) => {
        for qname in qnames {
          let QNameInfo { local_name, .. } = parse_qname_info(&qname);
          local_name_qnames.entry(local_name).or_default().push(qname);
        }
      }
      Some(
        SdkChoiceVariantKind::TypedChild
        | SdkChoiceVariantKind::Choice
        | SdkChoiceVariantKind::Sequence
        | SdkChoiceVariantKind::Any
        | SdkChoiceVariantKind::Text,
      )
      | None => {}
    }
  }

  Ok(
    local_name_qnames
      .into_values()
      .filter_map(|qnames| {
        if qnames.len() == 1 {
          return qnames.into_iter().next();
        }

        let base_qnames = qnames
          .into_iter()
          .filter(|qname| {
            let QNameInfo { tag_prefix, .. } = parse_qname_info(qname);
            !tag_prefix.as_bytes().last().is_some_and(u8::is_ascii_digit)
          })
          .collect::<Vec<_>>();

        if base_qnames.len() == 1 {
          base_qnames.into_iter().next()
        } else {
          None
        }
      })
      .collect(),
  )
}

fn any_child_dispatch_tokens(
  mode: DeserializeMode,
  cfg_attrs: &[Attribute],
  constructor: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  let read_outer_xml = mode.read_outer_xml_fn();

  quote! {
    #(#cfg_attrs)*
    {
      let xml = #read_outer_xml(xml_reader, e, empty_tag)?;
      return Ok(#constructor);
    }
  }
}

fn mce_process_field_tokens(
  ident: &Ident,
  optional: bool,
  repeated: bool,
) -> proc_macro2::TokenStream {
  if repeated {
    quote! {
      for child in #ident {
        crate::sdk::SdkMce::process_mce_with_context(child, settings, context)?;
      }
    }
  } else if optional {
    quote! {
      if let Some(child) = #ident {
        crate::sdk::SdkMce::process_mce_with_context(child, settings, context)?;
      }
    }
  } else {
    quote! {
      crate::sdk::SdkMce::process_mce_with_context(#ident, settings, context)?;
    }
  }
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
      SdkTypeFieldKind::Child { qname, .. } => NamedSequenceVariantFieldKind::Child { qname },
      SdkTypeFieldKind::EmptyChild { qname } => NamedSequenceVariantFieldKind::EmptyChild { qname },
      SdkTypeFieldKind::TextChild {
        qname, simple_type, ..
      } => NamedSequenceVariantFieldKind::TextChild { qname, simple_type },
      _ => {
        return Err(syn::Error::new_spanned(
          field,
          "named #[sdk(sequence)] variants currently support only #[sdk(child(...))], #[sdk(empty_child(...))] or #[sdk(text_child(...))] fields",
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
    NamedSequenceVariantFieldKind::Child { qname } => {
      if field.repeated {
        let child_write = write_typed_child_tokens(&inner_ty, quote! { child }, qname);
        quote! {
          for child in #field_ident {
            #child_write
          }
        }
      } else if field.optional {
        let child_write = write_typed_child_tokens(&inner_ty, quote! { child }, qname);
        quote! {
          if let Some(child) = #field_ident {
            #child_write
          }
        }
      } else {
        let child_write = write_typed_child_tokens(&inner_ty, quote! { #field_ident }, qname);
        quote! {
          #child_write
        }
      }
    }
    NamedSequenceVariantFieldKind::EmptyChild { qname } => {
      let QNameInfo {
        tag_prefix,
        local_name,
      } = parse_qname_info(qname);
      let write_tokens = quote! {
        crate::common::write_start_tag_open(writer, xmlns_prefix, #tag_prefix, #local_name)?;
        writer.write_all(b" />")?;
      };
      if field.repeated {
        quote! {
          for _ in #field_ident {
            #write_tokens
          }
        }
      } else if field.optional {
        quote! {
          if #field_ident.is_some() {
            #write_tokens
          }
        }
      } else {
        write_tokens
      }
    }
    NamedSequenceVariantFieldKind::TextChild { qname, simple_type } => {
      let QNameInfo {
        tag_prefix,
        local_name,
      } = parse_qname_info(qname);
      let write_value_tokens = |value_expr: proc_macro2::TokenStream| {
        let value_write_tokens =
          if let Some(kind) = simple_union_effective_type_kind(&inner_ty, simple_type.as_deref()) {
            write_simple_union_value_tokens(kind, value_expr.clone())
          } else if effective_type_name(&inner_ty, simple_type.as_deref())
            .as_deref()
            .is_some_and(is_xml_schema_float_type_name)
          {
            write_xml_schema_float_effective_tokens(
              value_expr.clone(),
              &inner_ty,
              simple_type.as_deref(),
              qname,
            )
          } else if is_string_like_effective_type(&inner_ty, simple_type.as_deref()) {
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
  if matches!(
    field.kind,
    NamedSequenceVariantFieldKind::TextChild { .. }
      | NamedSequenceVariantFieldKind::EmptyChild { .. }
  ) {
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
        crate::validator::SdkValidator::validate_into(#validate_expr, context);
      }
    }
  } else if field.optional {
    quote! {
      if let Some(child) = #field_ident {
        crate::validator::SdkValidator::validate_into(#validate_expr, context);
      }
    }
  } else {
    quote! {
      crate::validator::SdkValidator::validate_into(#validate_self_expr, context);
    }
  }
}

pub(crate) fn expand_sdk_choice(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
  let Data::Enum(DataEnum { variants, .. }) = &input.data else {
    return Err(syn::Error::new_spanned(
      input,
      "SdkChoice can only be derived for enums",
    ));
  };
  let local_name_fallback_qnames = choice_child_local_name_fallback_qnames(variants)?;

  let mut write_arms = Vec::with_capacity(variants.len());
  let mut direct_matcher_arms = Vec::with_capacity(variants.len());
  let mut helper_matcher_checks = Vec::new();
  let mut direct_child_dispatch_arms_borrowed = Vec::with_capacity(variants.len());
  let mut direct_child_dispatch_arms_io = Vec::with_capacity(variants.len());
  let mut helper_child_dispatch_tokens_borrowed = Vec::new();
  let mut helper_child_dispatch_tokens_io = Vec::new();
  let mut any_dispatch_tokens_borrowed = Vec::new();
  let mut any_dispatch_tokens_io = Vec::new();
  let mut text_from_string_tokens = Vec::new();
  let mut validate_arms = Vec::new();
  let mut mce_choice_arms = Vec::new();
  let mut mce_any_arms = Vec::new();
  let mut has_any_variant = false;
  let mut helper_items = Vec::new();
  let deserialize_borrowed_inner_ident = deserialize_choice_inner_ident(DeserializeMode::Borrowed);
  let deserialize_io_inner_ident = deserialize_choice_inner_ident(DeserializeMode::Io);

  for variant in variants {
    let variant_ident = &variant.ident;
    let cfg_attrs = cfg_attrs(&variant.attrs);
    let parsed_kind = parse_sdk_choice_variant_kind(&variant.attrs)?;
    let kind = match parsed_kind {
      Some(kind) => kind,
      None
        if matches!(
          &variant.fields,
          Fields::Unnamed(fields) if fields.unnamed.len() == 1
        ) =>
      {
        SdkChoiceVariantKind::TypedChild
      }
      None => {
        return Err(syn::Error::new_spanned(
          variant,
          "missing #[sdk(child(...))], #[sdk(empty_child(...))], #[sdk(text_child(...))], #[sdk(any_child(...))], #[sdk(choice)], #[sdk(sequence)] or #[sdk(any)] on choice variant",
        ));
      }
    };
    match (&variant.fields, kind) {
      (Fields::Unnamed(fields), SdkChoiceVariantKind::TypedChild) if fields.unnamed.len() == 1 => {
        let payload_ty = choice_variant_payload_type(variant)?;
        let inner_payload_ty = box_inner_type(&payload_ty).unwrap_or_else(|| payload_ty.clone());
        let inner_ty = choice_variant_inner_type(&payload_ty);
        let constructor = if is_box_type(&payload_ty) {
          quote! { Self::#variant_ident(std::boxed::Box::new(parsed_child)) }
        } else {
          quote! { Self::#variant_ident(parsed_child) }
        };
        helper_matcher_checks.push(quote! {
          #(#cfg_attrs)*
          if <#inner_ty as crate::sdk::SdkType>::matches_type_start_qname(name) {
            return true;
          }
        });
        helper_child_dispatch_tokens_borrowed.push(quote! {
          #(#cfg_attrs)*
          if <#inner_ty as crate::sdk::SdkType>::matches_type_start_qname(event_name) {
            let parsed_child = <#inner_ty as crate::sdk::SdkType>::read_borrowed(xml_reader, e, empty_tag)?;
            return Ok(#constructor);
          }
        });
        helper_child_dispatch_tokens_io.push(quote! {
          #(#cfg_attrs)*
          if <#inner_ty as crate::sdk::SdkType>::matches_type_start_qname(event_name) {
            let parsed_child = <#inner_ty as crate::sdk::SdkType>::read_io(xml_reader, e, empty_tag)?;
            return Ok(#constructor);
          }
        });
        let value_expr = if is_box_type(&payload_ty) {
          quote! { value.as_ref() }
        } else {
          quote! { value }
        };
        let write_child = write_typed_child_element_name_tokens(&inner_payload_ty, value_expr);
        write_arms.push(quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => {
            #write_child
            Ok(())
          },
        });
        let validate_arm = if is_box_type(&payload_ty) {
          quote! {
            #(#cfg_attrs)*
            Self::#variant_ident(value) => crate::validator::SdkValidator::validate_into(value.as_ref(), context),
          }
        } else {
          quote! {
            #(#cfg_attrs)*
            Self::#variant_ident(value) => crate::validator::SdkValidator::validate_into(value, context),
          }
        };
        validate_arms.push(validate_arm);
        mce_choice_arms.push(quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => {
            crate::sdk::SdkMce::process_mce_with_context(value, settings, context)
          },
        });
      }
      (Fields::Unnamed(fields), SdkChoiceVariantKind::Child { qnames })
        if fields.unnamed.len() == 1 =>
      {
        let payload_ty = choice_variant_payload_type(variant)?;
        let inner_payload_ty = box_inner_type(&payload_ty).unwrap_or_else(|| payload_ty.clone());
        if is_vec_string_type(&inner_payload_ty) {
          let qname_patterns = choice_qname_patterns(&qnames, &local_name_fallback_qnames);
          let QNameInfo {
            tag_prefix,
            local_name,
          } = parse_qname_info(qnames.first().map(String::as_str).unwrap_or_default());
          let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
          let tag_qname_lit = LitByteStr::new(
            format!("{tag_prefix}:{local_name}").as_bytes(),
            Span::call_site(),
          );
          let constructor = if is_box_type(&payload_ty) {
            quote! { Self::#variant_ident(std::boxed::Box::new(parsed_child)) }
          } else {
            quote! { Self::#variant_ident(parsed_child) }
          };
          direct_matcher_arms.push(quote! {
            #(#cfg_attrs)*
            #( #qname_patterns )|* => true,
          });
          direct_child_dispatch_arms_borrowed.push(quote! {
            #(#cfg_attrs)*
            #( #qname_patterns )|* => {
              let mut parsed_child = Vec::new();
              if !empty_tag {
                loop {
                  match xml_reader.next()? {
                    quick_xml::events::Event::Start(e) => {
                      let xml = crate::common::read_outer_xml_borrowed(xml_reader, e, false)?;
                      parsed_child.push(xml);
                    }
                    quick_xml::events::Event::Empty(e) => {
                      let xml = crate::common::read_outer_xml_borrowed(xml_reader, e, true)?;
                      parsed_child.push(xml);
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
              }
              return Ok(#constructor);
            }
          });
          direct_child_dispatch_arms_io.push(quote! {
            #(#cfg_attrs)*
            #( #qname_patterns )|* => {
              let mut parsed_child = Vec::new();
              if !empty_tag {
                loop {
                  match xml_reader.next()? {
                    quick_xml::events::Event::Start(e) => {
                      let xml = crate::common::read_outer_xml_io(xml_reader, e, false)?;
                      parsed_child.push(xml);
                    }
                    quick_xml::events::Event::Empty(e) => {
                      let xml = crate::common::read_outer_xml_io(xml_reader, e, true)?;
                      parsed_child.push(xml);
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
              }
              return Ok(#constructor);
            }
          });
          let value_expr = if is_box_type(&payload_ty) {
            quote! { value.as_ref() }
          } else {
            quote! { value }
          };
          write_arms.push(quote! {
            #(#cfg_attrs)*
            Self::#variant_ident(value) => {
              crate::common::write_start_tag_open(writer, xmlns_prefix, #tag_prefix, #local_name)?;
              writer.write_all(b">")?;
              for value in #value_expr {
                writer.write_all(value.as_bytes())?;
              }
              crate::common::write_end_tag(writer, xmlns_prefix, #tag_prefix, #local_name)
            }
          });
          validate_arms.push(quote! {
            #(#cfg_attrs)*
            Self::#variant_ident(_) => {},
          });
          continue;
        }
        let inner_ty = choice_variant_inner_type(&payload_ty);
        let constructor = if is_box_type(&payload_ty) {
          quote! { Self::#variant_ident(std::boxed::Box::new(parsed_child)) }
        } else {
          quote! { Self::#variant_ident(parsed_child) }
        };
        let qname_patterns = choice_qname_patterns(&qnames, &local_name_fallback_qnames);
        direct_matcher_arms.push(quote! {
          #(#cfg_attrs)*
          #( #qname_patterns )|* => true,
        });
        direct_child_dispatch_arms_borrowed.push(quote! {
          #(#cfg_attrs)*
          #( #qname_patterns )|* => {
            let parsed_child = <#inner_ty as crate::sdk::SdkType>::read_borrowed(xml_reader, e, empty_tag)?;
            return Ok(#constructor);
          }
        });
        direct_child_dispatch_arms_io.push(quote! {
          #(#cfg_attrs)*
          #( #qname_patterns )|* => {
            let parsed_child = <#inner_ty as crate::sdk::SdkType>::read_io(xml_reader, e, empty_tag)?;
            return Ok(#constructor);
          }
        });

        let write_child = write_typed_child_tokens(&inner_payload_ty, quote! { value }, &qnames[0]);
        let write_arm = quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => {
            #write_child
            Ok(())
          },
        };
        write_arms.push(write_arm);
        let validate_arm = if is_box_type(&payload_ty) {
          quote! {
            #(#cfg_attrs)*
            Self::#variant_ident(value) => crate::validator::SdkValidator::validate_into(value.as_ref(), context),
          }
        } else {
          quote! {
            #(#cfg_attrs)*
            Self::#variant_ident(value) => crate::validator::SdkValidator::validate_into(value, context),
          }
        };
        validate_arms.push(validate_arm);
        mce_choice_arms.push(quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => {
            crate::sdk::SdkMce::process_mce_with_context(value, settings, context)
          },
        });
      }
      (Fields::Unnamed(fields), SdkChoiceVariantKind::Choice) if fields.unnamed.len() == 1 => {
        let payload_ty = choice_variant_payload_type(variant)?;
        let inner_ty = choice_variant_inner_type(&payload_ty);
        let constructor = if is_box_type(&payload_ty) {
          quote! { Self::#variant_ident(std::boxed::Box::new(parsed_child)) }
        } else {
          quote! { Self::#variant_ident(parsed_child) }
        };
        helper_matcher_checks.push(quote! {
          #(#cfg_attrs)*
          if <#inner_ty as crate::sdk::SdkChoice>::matches_start_qname(name) {
            return true;
          }
        });
        helper_child_dispatch_tokens_borrowed.push(quote! {
          #(#cfg_attrs)*
          if <#inner_ty as crate::sdk::SdkChoice>::matches_start_qname(event_name) {
            let parsed_child = <#inner_ty>::#deserialize_borrowed_inner_ident(xml_reader, Some((e, empty_tag)))?;
            return Ok(#constructor);
          }
        });
        helper_child_dispatch_tokens_io.push(quote! {
          #(#cfg_attrs)*
          if <#inner_ty as crate::sdk::SdkChoice>::matches_start_qname(event_name) {
            let parsed_child = <#inner_ty>::#deserialize_io_inner_ident(xml_reader, Some((e, empty_tag)))?;
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
            Self::#variant_ident(value) => crate::validator::SdkValidator::validate_into(value.as_ref(), context),
          }
        } else {
          quote! {
            #(#cfg_attrs)*
            Self::#variant_ident(value) => crate::validator::SdkValidator::validate_into(value, context),
          }
        };
        validate_arms.push(validate_arm);
        let mce_value = if is_box_type(&payload_ty) {
          quote! { value.as_mut() }
        } else {
          quote! { value }
        };
        mce_choice_arms.push(quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => {
            crate::sdk::SdkMceChoice::process_mce_choice_with_context(
              #mce_value,
              settings,
              context,
            )
          },
        });
      }
      (Fields::Unit, SdkChoiceVariantKind::EmptyChild { qnames }) => {
        let qname_patterns = choice_qname_patterns(&qnames, &local_name_fallback_qnames);
        let write_qname = qnames.first().ok_or_else(|| {
          syn::Error::new_spanned(variant, "empty_child requires at least one qname")
        })?;
        let QNameInfo {
          tag_prefix,
          local_name,
        } = parse_qname_info(write_qname);
        let skip_tokens_borrowed = empty_child_skip_tokens(
          ident,
          DeserializeMode::Borrowed,
          &qnames,
          &local_name_fallback_qnames,
        );
        let skip_tokens_io = empty_child_skip_tokens(
          ident,
          DeserializeMode::Io,
          &qnames,
          &local_name_fallback_qnames,
        );
        direct_matcher_arms.push(quote! {
          #(#cfg_attrs)*
          #( #qname_patterns )|* => true,
        });
        direct_child_dispatch_arms_borrowed.push(quote! {
          #(#cfg_attrs)*
          #( #qname_patterns )|* => {
            #skip_tokens_borrowed
            return Ok(Self::#variant_ident);
          }
        });
        direct_child_dispatch_arms_io.push(quote! {
          #(#cfg_attrs)*
          #( #qname_patterns )|* => {
            #skip_tokens_io
            return Ok(Self::#variant_ident);
          }
        });
        write_arms.push(quote! {
          #(#cfg_attrs)*
          Self::#variant_ident => {
            crate::common::write_start_tag_open(writer, xmlns_prefix, #tag_prefix, #local_name)?;
            writer.write_all(b" />")?;
            Ok(())
          },
        });
        validate_arms.push(quote! {
          #(#cfg_attrs)*
          Self::#variant_ident => {},
        });
        mce_choice_arms.push(quote! {
          #(#cfg_attrs)*
          Self::#variant_ident => Ok(()),
        });
      }
      (Fields::Unnamed(fields), SdkChoiceVariantKind::Sequence) if fields.unnamed.len() == 1 => {
        let payload_ty = choice_variant_payload_type(variant)?;
        let inner_ty = choice_variant_inner_type(&payload_ty);
        let constructor = if is_box_type(&payload_ty) {
          quote! { Self::#variant_ident(std::boxed::Box::new(parsed_child)) }
        } else {
          quote! { Self::#variant_ident(parsed_child) }
        };
        helper_matcher_checks.push(quote! {
          #(#cfg_attrs)*
          if #inner_ty::matches_specific_start_qname(name) {
            return true;
          }
        });
        helper_child_dispatch_tokens_borrowed.push(quote! {
          #(#cfg_attrs)*
          if #inner_ty::matches_specific_start_qname(event_name) {
            let parsed_child = <#inner_ty as crate::sdk::SdkType>::read_borrowed(xml_reader, e, empty_tag)?;
            return Ok(#constructor);
          }
        });
        helper_child_dispatch_tokens_io.push(quote! {
          #(#cfg_attrs)*
          if #inner_ty::matches_specific_start_qname(event_name) {
            let parsed_child = <#inner_ty as crate::sdk::SdkType>::read_io(xml_reader, e, empty_tag)?;
            return Ok(#constructor);
          }
        });
        let write_arm = quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => <#inner_ty as crate::sdk::SdkType>::write_inner(
            value,
            writer,
            xmlns_prefix,
            crate::sdk::ElementName::new(b"", b""),
          ),
        };
        write_arms.push(write_arm);
        let validate_arm = if is_box_type(&payload_ty) {
          quote! {
            #(#cfg_attrs)*
            Self::#variant_ident(value) => crate::validator::SdkValidator::validate_into(value.as_ref(), context),
          }
        } else {
          quote! {
            #(#cfg_attrs)*
            Self::#variant_ident(value) => crate::validator::SdkValidator::validate_into(value, context),
          }
        };
        validate_arms.push(validate_arm);
        mce_choice_arms.push(quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => {
            crate::sdk::SdkMce::process_mce_with_context(value, settings, context)
          },
        });
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
            if matches!(
              field.kind,
              NamedSequenceVariantFieldKind::TextChild { .. }
                | NamedSequenceVariantFieldKind::EmptyChild { .. }
            ) {
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
          | NamedSequenceVariantFieldKind::EmptyChild { qname }
          | NamedSequenceVariantFieldKind::TextChild { qname, .. } => qname,
        };
        let start_qname_patterns = choice_qname_patterns(
          std::slice::from_ref(start_qname),
          &local_name_fallback_qnames,
        );
        direct_matcher_arms.push(quote! {
          #(#cfg_attrs)*
          #( #start_qname_patterns )|* => true,
        });
        direct_child_dispatch_arms_borrowed.push(quote! {
          #(#cfg_attrs)*
          #( #start_qname_patterns )|* => {
            let parsed_child = <#helper_ident as crate::sdk::SdkType>::read_borrowed(xml_reader, e, empty_tag)?;
            let #helper_ident { #( #field_idents ),* } = parsed_child;
            return Ok(Self::#variant_ident { #( #field_idents ),* });
          }
        });
        direct_child_dispatch_arms_io.push(quote! {
          #(#cfg_attrs)*
          #( #start_qname_patterns )|* => {
            let parsed_child = <#helper_ident as crate::sdk::SdkType>::read_io(xml_reader, e, empty_tag)?;
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
          },
        });
        let mce_named_field_tokens = named_fields
          .iter()
          .filter(|field| matches!(field.kind, NamedSequenceVariantFieldKind::Child { .. }))
          .map(|field| {
            let ident = &field.ident;
            mce_process_field_tokens(ident, field.optional, field.repeated)
          })
          .collect::<Vec<_>>();
        let mce_pattern_fields = named_fields
          .iter()
          .map(|field| {
            let ident = &field.ident;
            if matches!(field.kind, NamedSequenceVariantFieldKind::Child { .. }) {
              quote! { #ident }
            } else {
              quote! { #ident: _ }
            }
          })
          .collect::<Vec<_>>();
        mce_choice_arms.push(quote! {
          #(#cfg_attrs)*
          Self::#variant_ident { #( #mce_pattern_fields ),* } => {
            #( #mce_named_field_tokens )*
            Ok(())
          },
        });
      }
      (
        Fields::Unnamed(fields),
        SdkChoiceVariantKind::TextChild {
          qnames,
          simple_type,
        },
      ) if fields.unnamed.len() == 1 => {
        let payload_ty = choice_variant_payload_type(variant)?;
        let qname_patterns = choice_qname_patterns(&qnames, &local_name_fallback_qnames);
        let QNameInfo {
          tag_prefix,
          local_name,
        } = parse_qname_info(qnames.first().map(String::as_str).unwrap_or_default());
        let write_qname = qnames.first().map(String::as_str).unwrap_or_default();
        let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
        let tag_qname_lit = LitByteStr::new(
          format!("{tag_prefix}:{local_name}").as_bytes(),
          Span::call_site(),
        );
        let simple_union_kind =
          simple_union_effective_type_kind(&payload_ty, simple_type.as_deref());
        let parse_from_text_tokens = if let Some(kind) = simple_union_kind {
          let parse_tokens = parse_simple_union_value_tokens(kind, quote! { value });
          quote! {{
            let value = text.unwrap_or_default();
            #parse_tokens
          }}
        } else if is_string_like_effective_type(&payload_ty, simple_type.as_deref()) {
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
        let empty_value_tokens = if let Some(kind) = simple_union_kind {
          parse_simple_union_value_tokens(kind, quote! { String::new() })
        } else if is_string_like_effective_type(&payload_ty, simple_type.as_deref()) {
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
        direct_matcher_arms.push(quote! {
          #(#cfg_attrs)*
          #( #qname_patterns )|* => true,
        });
        direct_child_dispatch_arms_borrowed.push(quote! {
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
        direct_child_dispatch_arms_io.push(quote! {
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
        let value_write_tokens = if let Some(kind) =
          simple_union_effective_type_kind(&payload_ty, simple_type.as_deref())
        {
          write_simple_union_value_tokens(kind, quote! { value })
        } else if effective_type_name(&payload_ty, simple_type.as_deref())
          .as_deref()
          .is_some_and(is_xml_schema_float_type_name)
        {
          write_xml_schema_float_effective_tokens(
            quote! { value },
            &payload_ty,
            simple_type.as_deref(),
            write_qname,
          )
        } else if is_string_like_effective_type(&payload_ty, simple_type.as_deref()) {
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
          Self::#variant_ident(_) => {},
        });
      }
      (Fields::Unnamed(fields), SdkChoiceVariantKind::AnyChild { qnames })
        if fields.unnamed.len() == 1 =>
      {
        let qname_patterns = choice_qname_patterns(&qnames, &local_name_fallback_qnames);
        let QNameInfo {
          tag_prefix,
          local_name,
        } = parse_qname_info(qnames.first().map(String::as_str).unwrap_or_default());
        let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
        let tag_qname_lit = LitByteStr::new(
          format!("{tag_prefix}:{local_name}").as_bytes(),
          Span::call_site(),
        );
        direct_matcher_arms.push(quote! {
          #(#cfg_attrs)*
          #( #qname_patterns )|* => true,
        });
        direct_child_dispatch_arms_borrowed.push(quote! {
          #(#cfg_attrs)*
          #( #qname_patterns )|* => {
            let mut parsed_child = Vec::new();
            if !empty_tag {
              loop {
                match xml_reader.next()? {
                  quick_xml::events::Event::Start(e) => {
                    let xml = crate::common::read_outer_xml_borrowed(xml_reader, e, false)?;
                    parsed_child.push(xml);
                  }
                  quick_xml::events::Event::Empty(e) => {
                    let xml = crate::common::read_outer_xml_borrowed(xml_reader, e, true)?;
                    parsed_child.push(xml);
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
            }
            return Ok(Self::#variant_ident(parsed_child));
          }
        });
        direct_child_dispatch_arms_io.push(quote! {
          #(#cfg_attrs)*
          #( #qname_patterns )|* => {
            let mut parsed_child = Vec::new();
            if !empty_tag {
              loop {
                match xml_reader.next()? {
                  quick_xml::events::Event::Start(e) => {
                    let xml = crate::common::read_outer_xml_io(xml_reader, e, false)?;
                    parsed_child.push(xml);
                  }
                  quick_xml::events::Event::Empty(e) => {
                    let xml = crate::common::read_outer_xml_io(xml_reader, e, true)?;
                    parsed_child.push(xml);
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
            }
            return Ok(Self::#variant_ident(parsed_child));
          }
        });
        write_arms.push(quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(value) => {
            crate::common::write_start_tag_open(writer, xmlns_prefix, #tag_prefix, #local_name)?;
            writer.write_all(b">")?;
            for value in value {
              writer.write_all(value.as_bytes())?;
            }
            crate::common::write_end_tag(writer, xmlns_prefix, #tag_prefix, #local_name)
          }
        });
        validate_arms.push(quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(_) => {},
        });
      }
      (Fields::Unnamed(fields), SdkChoiceVariantKind::Any) if fields.unnamed.len() == 1 => {
        let payload_ty = choice_variant_payload_type(variant)?;
        has_any_variant = true;
        let any_xml_expr = if is_box_str_type(&payload_ty) {
          quote! { xml.as_ref() }
        } else if is_box_type(&payload_ty) {
          quote! { xml.as_ref().as_str() }
        } else {
          quote! { xml.as_str() }
        };
        let constructor = if is_box_str_type(&payload_ty) {
          quote! { Self::#variant_ident(xml.into_boxed_str()) }
        } else if is_box_type(&payload_ty) {
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
          Self::#variant_ident(_) => {},
        });
        mce_any_arms.push(quote! {
          #(#cfg_attrs)*
          Self::#variant_ident(xml) => {
            if let Some(children) = crate::common::mce_choice_replacement_children(
              #any_xml_expr,
              settings,
              context,
            )? {
              for child_xml in children {
                let mut child_reader = crate::common::from_bytes_inner(child_xml.as_bytes())?;
                let mut child = <Self as crate::sdk::SdkChoice>::deserialize_borrowed_inner(
                  &mut child_reader,
                  None,
                )?;
                <Self as crate::sdk::SdkMceChoice>::process_mce_choice_with_context(
                  &mut child,
                  settings,
                  context,
                )?;
                processed.push(child);
              }
            } else {
              <Self as crate::sdk::SdkMceChoice>::process_mce_choice_with_context(
                &mut item,
                settings,
                context,
              )?;
              processed.push(item);
            }
          }
        });
        any_dispatch_tokens_borrowed.push(any_child_dispatch_tokens(
          DeserializeMode::Borrowed,
          &cfg_attrs,
          &constructor,
        ));
        any_dispatch_tokens_io.push(any_child_dispatch_tokens(
          DeserializeMode::Io,
          &cfg_attrs,
          &constructor,
        ));
      }
      (Fields::Unnamed(fields), SdkChoiceVariantKind::Text) if fields.unnamed.len() == 1 => {
        let payload_ty = choice_variant_payload_type(variant)?;
        let value_write_tokens = if is_xml_schema_float_type(&payload_ty) {
          write_xml_schema_float_tokens(quote! { value }, &payload_ty)
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
          Self::#variant_ident(_) => {},
        });
        text_from_string_tokens.push(quote! {
          #(#cfg_attrs)*
          parsed = Some(Self::#variant_ident(value.to_owned().into()));
        });
      }
      _ => {
        return Err(syn::Error::new_spanned(
          variant,
          "SdkChoice supports single-field tuple variants, unit #[sdk(empty_child(...))] variants, plus named #[sdk(sequence)] variants with 1 or 2 #[sdk(child(...))] or #[sdk(text_child(...))] fields",
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
  let has_text_variant = !text_from_string_tokens.is_empty();
  let choice_trait_matches_start_tokens = if has_any_variant {
    quote! {
      #[inline]
      fn matches_start_qname(_name: &[u8]) -> bool {
        true
      }
    }
  } else {
    quote! {}
  };
  let choice_trait_accepts_any_tokens = if has_any_variant {
    quote! {
      #[inline]
      fn accepts_any_child() -> bool {
        true
      }
    }
  } else {
    quote! {}
  };
  let choice_trait_accepts_text_tokens = if has_text_variant {
    quote! {
      #[inline]
      fn accepts_text() -> bool {
        true
      }
    }
  } else {
    quote! {}
  };
  let choice_trait_from_text_tokens = if text_from_string_tokens.is_empty() {
    quote! {}
  } else {
    quote! {
      fn from_text_value(value: &str) -> Option<Self> {
        let mut parsed = None;
        #( #text_from_string_tokens )*
        parsed
      }
    }
  };

  let mce_choice_dispatch_tokens_borrowed = quote! {};
  let mce_choice_dispatch_tokens_io = quote! {};

  let read_tokens_borrowed = if any_dispatch_tokens_borrowed.is_empty() {
    quote! {
      let event_name = e.name();
      let event_name = event_name.as_ref();
      match event_name {
        #( #direct_child_dispatch_arms_borrowed )*
        _ => {
          #mce_choice_dispatch_tokens_borrowed
          #( #helper_child_dispatch_tokens_borrowed )*
          Err(crate::common::unexpected_tag(
            stringify!(#ident),
            "choice",
            event_name,
          ))
        }
      }
    }
  } else {
    quote! {
      let event_name = e.name();
      let event_name = event_name.as_ref();
      match event_name {
        #( #direct_child_dispatch_arms_borrowed )*
        _ => {
          #mce_choice_dispatch_tokens_borrowed
          #( #helper_child_dispatch_tokens_borrowed )*
          #( #any_dispatch_tokens_borrowed )*
        }
      }
    }
  };

  let read_tokens_io = if any_dispatch_tokens_io.is_empty() {
    quote! {
      let event_name = e.name();
      let event_name = event_name.as_ref();
      match event_name {
        #( #direct_child_dispatch_arms_io )*
        _ => {
          #mce_choice_dispatch_tokens_io
          #( #helper_child_dispatch_tokens_io )*
          Err(crate::common::unexpected_tag(
            stringify!(#ident),
            "choice",
            event_name,
          ))
        }
      }
    }
  } else {
    quote! {
      let event_name = e.name();
      let event_name = event_name.as_ref();
      match event_name {
        #( #direct_child_dispatch_arms_io )*
        _ => {
          #mce_choice_dispatch_tokens_io
          #( #helper_child_dispatch_tokens_io )*
          #( #any_dispatch_tokens_io )*
        }
      }
    }
  };

  Ok(quote! {
    #( #helper_items )*

    impl #impl_generics crate::sdk::SdkChoice for #ident #type_generics #where_clause {
      fn #deserialize_borrowed_inner_ident<'de>(
        xml_reader: &mut crate::common::SliceReader<'de>,
        xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
      ) -> Result<Self, crate::common::SdkError> {
        let (e, empty_tag) = if let Some((e, empty_tag)) = xml_event {
          (e, empty_tag)
        } else {
          loop {
            match xml_reader.next_tag_event()? {
              crate::common::SliceTagEvent::Start(e, empty_tag) => break (e, empty_tag),
              crate::common::SliceTagEvent::Eof => {
                return Err(crate::common::unexpected_eof(stringify!(#ident)));
              }
              crate::common::SliceTagEvent::Decl(_)
              | crate::common::SliceTagEvent::End(_)
              | crate::common::SliceTagEvent::Other => continue,
            }
          }
        };
        #read_tokens_borrowed
      }

      fn #deserialize_io_inner_ident<R: std::io::BufRead>(
        xml_reader: &mut crate::common::IoReader<R>,
        xml_event: Option<(quick_xml::events::BytesStart<'static>, bool)>,
      ) -> Result<Self, crate::common::SdkError> {
        let (e, empty_tag) = if let Some((e, empty_tag)) = xml_event {
          (e, empty_tag)
        } else {
          loop {
            match xml_reader.next_tag_event()? {
              crate::common::IoTagEvent::Start(e, empty_tag) => break (e, empty_tag),
              crate::common::IoTagEvent::Eof => {
                return Err(crate::common::unexpected_eof(stringify!(#ident)));
              }
              crate::common::IoTagEvent::Decl(_)
              | crate::common::IoTagEvent::End(_)
              | crate::common::IoTagEvent::Other => continue,
            }
          }
        };
        #read_tokens_io
      }

      fn write_xml<W: std::io::Write>(
        &self,
        writer: &mut W,
        xmlns_prefix: &str,
      ) -> Result<(), std::io::Error> {
        match self {
          #( #write_arms )*
        }
      }

      #[inline]
      fn matches_specific_start_qname(name: &[u8]) -> bool {
        #matches_specific_start_qname_tokens
      }

      #choice_trait_matches_start_tokens
      #choice_trait_accepts_any_tokens
      #choice_trait_accepts_text_tokens
      #choice_trait_from_text_tokens
    }
    #[cfg(feature = "validators")]
    impl #impl_generics crate::validator::SdkValidator for #ident #type_generics #where_clause {
      fn validate_into(&self, context: &mut crate::validator::ValidationContext) {
        match self {
          #( #validate_arms )*
        }
      }
    }

    #[cfg(feature = "mce")]
    impl #impl_generics crate::sdk::SdkMceChoice for #ident #type_generics #where_clause {
      fn process_mce_choice_with_context(
        &mut self,
        settings: &crate::sdk::MarkupCompatibilityProcessSettings,
        context: &mut crate::sdk::MceContext,
      ) -> Result<(), crate::common::SdkError> {
        match self {
          #( #mce_choice_arms )*
          _ => Ok(()),
        }
      }

      fn process_mce_choices_with_context(
        values: &mut Vec<Self>,
        settings: &crate::sdk::MarkupCompatibilityProcessSettings,
        context: &mut crate::sdk::MceContext,
      ) -> Result<(), crate::common::SdkError> {
        if matches!(
          settings.process_mode,
          crate::sdk::MarkupCompatibilityProcessMode::NoProcess
        ) {
          return Ok(());
        }
        let mut processed = Vec::with_capacity(values.len());
        for mut item in std::mem::take(values) {
          match &mut item {
            #( #mce_any_arms )*
            _ => {
              <Self as crate::sdk::SdkMceChoice>::process_mce_choice_with_context(
                &mut item,
                settings,
                context,
              )?;
              processed.push(item);
            }
          }
        }
        *values = processed;
        Ok(())
      }
    }

    impl #impl_generics #ident #type_generics #where_clause {
      pub(crate) fn #deserialize_borrowed_inner_ident<'de>(
        xml_reader: &mut crate::common::SliceReader<'de>,
        xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
      ) -> Result<Self, crate::common::SdkError> {
        <Self as crate::sdk::SdkChoice>::#deserialize_borrowed_inner_ident(xml_reader, xml_event)
      }

      pub(crate) fn #deserialize_io_inner_ident<R: std::io::BufRead>(
        xml_reader: &mut crate::common::IoReader<R>,
        xml_event: Option<(quick_xml::events::BytesStart<'static>, bool)>,
      ) -> Result<Self, crate::common::SdkError> {
        <Self as crate::sdk::SdkChoice>::#deserialize_io_inner_ident(xml_reader, xml_event)
      }

      pub(crate) fn write_xml<W: std::io::Write>(
        &self,
        writer: &mut W,
        xmlns_prefix: &str,
      ) -> Result<(), std::io::Error> {
        <Self as crate::sdk::SdkChoice>::write_xml(self, writer, xmlns_prefix)
      }
    }
  })
}
