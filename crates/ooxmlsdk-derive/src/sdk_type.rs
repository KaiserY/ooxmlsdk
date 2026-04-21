use super::*;

#[derive(Clone, Copy)]
enum DeserializeMode {
  Borrowed,
  Io,
}

fn deserialize_type_inner_ident(mode: DeserializeMode) -> Ident {
  match mode {
    DeserializeMode::Borrowed => Ident::new("deserialize_borrowed_inner", Span::call_site()),
    DeserializeMode::Io => Ident::new("deserialize_io_inner", Span::call_site()),
  }
}

pub(crate) fn expand_sdk_type(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let Data::Struct(data_struct) = &input.data else {
    return Err(syn::Error::new_spanned(
      input,
      "SdkType can only be derived for structs",
    ));
  };

  match &data_struct.fields {
    Fields::Named(fields) => {
      if let Some(schema_qname) = parse_sdk_qname(&input.attrs)? {
        return expand_named_struct(input, &schema_qname, fields);
      }
      expand_helper_struct(input, fields)
    }
    Fields::Unnamed(fields) if fields.unnamed.len() == 1 => Err(syn::Error::new_spanned(
      fields,
      "tuple leaf wrappers are no longer supported; use a named struct with an #[sdk(text)] xml_content field or a type alias",
    )),
    _ => Ok(quote! {
      impl crate::sdk::SdkType for #ident {}
      #[cfg(feature = "validators")]
      impl crate::validator::SdkValidator for #ident {}
    }),
  }
}

fn sdk_type_impl_tokens(
  ident: &Ident,
  impl_items: proc_macro2::TokenStream,
  validate_items: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  let deserialize_borrowed_inner_ident = deserialize_type_inner_ident(DeserializeMode::Borrowed);
  let deserialize_io_inner_ident = deserialize_type_inner_ident(DeserializeMode::Io);
  quote! {
    impl crate::sdk::SdkType for #ident {}
    #[cfg(feature = "validators")]
    impl crate::validator::SdkValidator for #ident {
      fn validate(&self) -> Result<(), crate::common::SdkError> {
        #validate_items
        Ok(())
      }
    }

    impl std::str::FromStr for #ident {
      type Err = crate::common::SdkError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut xml_reader = crate::common::from_str_inner(s)?;
        Self::#deserialize_borrowed_inner_ident(&mut xml_reader, None)
      }
    }

    impl #ident {
      pub fn from_bytes(bytes: &[u8]) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_bytes_inner(bytes)?;
        Self::#deserialize_borrowed_inner_ident(&mut xml_reader, None)
      }

      pub fn from_str(s: &str) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_str_inner(s)?;
        Self::#deserialize_borrowed_inner_ident(&mut xml_reader, None)
      }

      pub fn from_reader<R: std::io::BufRead>(
        reader: R,
      ) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_reader_inner(reader)?;
        Self::#deserialize_io_inner_ident(&mut xml_reader, None)
      }

      #impl_items
    }
  }
}

fn validator_source_union(validator: &SdkFieldValidator) -> (u32, Option<u64>) {
  match validator {
    SdkFieldValidator::Pattern {
      source_id,
      union_id,
      ..
    }
    | SdkFieldValidator::StringLength {
      source_id,
      union_id,
      ..
    }
    | SdkFieldValidator::StringFormat {
      source_id,
      union_id,
      ..
    }
    | SdkFieldValidator::StringSet {
      source_id,
      union_id,
      ..
    }
    | SdkFieldValidator::NumberRange {
      source_id,
      union_id,
      ..
    }
    | SdkFieldValidator::NumberType {
      source_id,
      union_id,
      ..
    }
    | SdkFieldValidator::NumberSign {
      source_id,
      union_id,
      ..
    } => (*source_id, *union_id),
  }
}

fn validator_token(
  ident: &Ident,
  field_ident: &Ident,
  parse_ty: &Type,
  validator: &SdkFieldValidator,
) -> proc_macro2::TokenStream {
  match validator {
    SdkFieldValidator::Pattern { regex, .. } => quote! {
      crate::validator::validate_pattern(
        stringify!(#ident),
        stringify!(#field_ident),
        value,
        #regex,
      )?;
    },
    SdkFieldValidator::StringLength { min, max, .. } => {
      let min_tokens = match min {
        Some(min) => quote! { Some(#min) },
        None => quote! { None },
      };
      let max_tokens = match max {
        Some(max) => quote! { Some(#max) },
        None => quote! { None },
      };
      let length_kind_tokens = if is_hex_binary_type(parse_ty)
        || matches!(
          validator,
          SdkFieldValidator::StringLength {
            type_name: Some(type_name),
            ..
          } if type_name == "w:ST_HexColorRGB"
        ) {
        quote! { crate::validator::StringLengthKind::HexBinaryBytes }
      } else {
        quote! { crate::validator::StringLengthKind::Characters }
      };
      quote! {
        crate::validator::validate_string_length_with_kind(
          stringify!(#ident),
          stringify!(#field_ident),
          value.to_string(),
          #min_tokens,
          #max_tokens,
          #length_kind_tokens,
        )?;
      }
    }
    SdkFieldValidator::StringFormat { kind, .. } => {
      let kind_tokens = match kind {
        SdkStringFormatKind::Token => quote! { crate::validator::StringFormatKind::Token },
        SdkStringFormatKind::NcName => quote! { crate::validator::StringFormatKind::NcName },
        SdkStringFormatKind::QName => quote! { crate::validator::StringFormatKind::QName },
        SdkStringFormatKind::Uri => quote! { crate::validator::StringFormatKind::Uri },
        SdkStringFormatKind::Id => quote! { crate::validator::StringFormatKind::Id },
      };
      quote! {
        crate::validator::validate_string_format(
          stringify!(#ident),
          stringify!(#field_ident),
          value,
          #kind_tokens,
        )?;
      }
    }
    SdkFieldValidator::StringSet { values, .. } => quote! {
      crate::validator::validate_string_set(
        stringify!(#ident),
        stringify!(#field_ident),
        value,
        &[ #( #values ),* ],
      )?;
    },
    SdkFieldValidator::NumberRange {
      min,
      max,
      min_inclusive,
      max_inclusive,
      ..
    } => {
      let min_tokens = match min {
        Some(min) => quote! { Some(#min) },
        None => quote! { None },
      };
      let max_tokens = match max {
        Some(max) => quote! { Some(#max) },
        None => quote! { None },
      };
      quote! {
        crate::validator::validate_number_range(
          stringify!(#ident),
          stringify!(#field_ident),
          value,
          #min_tokens,
          #max_tokens,
          #min_inclusive,
          #max_inclusive,
        )?;
      }
    }
    SdkFieldValidator::NumberType { type_name, .. } => quote! {
      crate::validator::validate_number_type(
        stringify!(#ident),
        stringify!(#field_ident),
        value,
        #type_name,
      )?;
    },
    SdkFieldValidator::NumberSign { kind, .. } => {
      let kind_tokens = match kind {
        SdkNumberSignKind::NonNegative => {
          quote! { crate::validator::NumberSignKind::NonNegative }
        }
        SdkNumberSignKind::Positive => quote! { crate::validator::NumberSignKind::Positive },
      };
      quote! {
        crate::validator::validate_number_sign(
          stringify!(#ident),
          stringify!(#field_ident),
          value,
          #kind_tokens,
        )?;
      }
    }
  }
}

fn text_child_match_target(qname: &str) -> proc_macro2::TokenStream {
  let QNameInfo {
    tag_prefix,
    local_name,
  } = parse_qname_info(qname);
  let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
  if tag_prefix.is_empty() {
    quote! { #local_name_lit }
  } else {
    let tag_qname_lit = LitByteStr::new(
      format!("{tag_prefix}:{local_name}").as_bytes(),
      Span::call_site(),
    );
    quote! { #tag_qname_lit | #local_name_lit }
  }
}

fn build_text_child_parse_arm(
  owner_ident: &Ident,
  field_ident: &Ident,
  qname: &str,
  field_ty: &Type,
  repeated: bool,
  as_result: bool,
) -> proc_macro2::TokenStream {
  let QNameInfo {
    tag_prefix,
    local_name,
  } = parse_qname_info(qname);
  let target = text_child_match_target(qname);
  let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
  let tag_qname_lit = LitByteStr::new(
    format!("{tag_prefix}:{local_name}").as_bytes(),
    Span::call_site(),
  );
  let value_ty = unwrap_option_vec_type(field_ty);
  let parse_from_text_tokens = if is_string_like_type(&value_ty) {
    quote! { text.unwrap_or_default() }
  } else if is_bool_type(&value_ty) {
    let parse_value_tokens = parse_bool_tokens(
      quote! { &value },
      &value_ty,
      quote! { stringify!(#owner_ident) },
      quote! { stringify!(#field_ident) },
    );
    quote! {{
      let value = text.unwrap_or_default();
      #parse_value_tokens
    }}
  } else {
    quote! {{
      let value = text.unwrap_or_default();
      crate::common::parse_value::<#value_ty>(
        &value,
        stringify!(#owner_ident),
        stringify!(#field_ident),
      )?
    }}
  };
  let empty_value_tokens = if is_string_like_type(&value_ty) {
    quote! { Default::default() }
  } else if is_bool_type(&value_ty) {
    parse_bool_tokens(
      quote! { "" },
      &value_ty,
      quote! { stringify!(#owner_ident) },
      quote! { stringify!(#field_ident) },
    )
  } else {
    quote! {
      crate::common::parse_value::<#value_ty>(
        "",
        stringify!(#owner_ident),
        stringify!(#field_ident),
      )?
    }
  };
  let assign_tokens = if repeated {
    quote! { #field_ident.push(parsed_child); }
  } else {
    quote! { #field_ident = Some(parsed_child); }
  };
  let finish_tokens = if as_result {
    quote! { Ok(true) }
  } else {
    quote! { true }
  };

  quote! {
    #target => {
      let parsed_child = if next_empty {
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
                stringify!(#owner_ident),
                stringify!(#field_ident),
              )?;
            }
            quick_xml::events::Event::End(end) => {
              if end.name().as_ref() == #tag_qname_lit || end.name().as_ref() == #local_name_lit {
                break;
              }
            }
            quick_xml::events::Event::Eof => {
              return Err(crate::common::unexpected_eof(stringify!(#owner_ident)));
            }
            _ => {}
          }
        }
        #parse_from_text_tokens
      };
      #assign_tokens
      #finish_tokens
    },
  }
}

fn build_text_child_write_tokens(
  field_ident: &Ident,
  qname: &str,
  field_ty: &Type,
  repeated: bool,
  optional: bool,
) -> proc_macro2::TokenStream {
  let QNameInfo {
    tag_prefix,
    local_name,
  } = parse_qname_info(qname);
  let inner_ty = unwrap_wrapped_type(field_ty);
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

  if repeated {
    let write_child_tokens = write_value_tokens(quote! { child });
    quote! {
      for child in &self.#field_ident {
        #write_child_tokens
      }
    }
  } else if optional {
    let write_child_tokens = write_value_tokens(quote! { child });
    quote! {
      if let Some(child) = &self.#field_ident {
        #write_child_tokens
      }
    }
  } else {
    let write_child_tokens = write_value_tokens(quote! { &self.#field_ident });
    quote! {
      #write_child_tokens
    }
  }
}

fn expand_sequence_helper_struct(
  input: &DeriveInput,
  fields: &syn::FieldsNamed,
) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let deserialize_borrowed_inner_ident = deserialize_type_inner_ident(DeserializeMode::Borrowed);
  let deserialize_io_inner_ident = deserialize_type_inner_ident(DeserializeMode::Io);

  let mut child_decl_tokens = Vec::new();
  let mut child_dispatch_tokens_borrowed = Vec::new();
  let mut child_dispatch_tokens_io = Vec::new();
  let mut child_write_tokens = Vec::new();
  let mut child_init_tokens = Vec::new();
  let mut child_validate_tokens = Vec::new();
  let mut matcher_checks = Vec::new();

  let xml_reader_ident = Ident::new("xml_reader", Span::call_site());

  for field in &fields.named {
    let field_ident = field
      .ident
      .as_ref()
      .ok_or_else(|| syn::Error::new_spanned(field, "SdkType requires named fields"))?;
    let parsed_attrs = parse_sdk_type_field_attrs(&field.attrs)?;
    match parsed_attrs.kind {
      Some(SdkTypeFieldKind::Child { qname }) => {
        let QNameInfo {
          tag_prefix,
          local_name,
        } = parse_qname_info(&qname);
        let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
        let payload_ty = unwrap_option_vec_type(&field.ty);
        let child_ty = if let Some(inner_ty) = box_inner_type(&payload_ty) {
          inner_ty
        } else {
          payload_ty.clone()
        };
        let validate_child_tokens = if box_inner_type(&payload_ty).is_some() {
          quote! { child.as_ref() }
        } else {
          quote! { child }
        };
        let validate_self_tokens = if box_inner_type(&payload_ty).is_some() {
          quote! { self.#field_ident.as_ref() }
        } else {
          quote! { &self.#field_ident }
        };
        let parsed_child_expr = if box_inner_type(&payload_ty).is_some() {
          quote! { std::boxed::Box::new(parsed_child) }
        } else {
          quote! { parsed_child }
        };
        let match_target = if tag_prefix.is_empty() {
          quote! { #local_name_lit }
        } else {
          let tag_qname_lit = LitByteStr::new(
            format!("{tag_prefix}:{local_name}").as_bytes(),
            Span::call_site(),
          );
          quote! { #tag_qname_lit | #local_name_lit }
        };
        matcher_checks.push(quote! {
          if matches!(name, #match_target) {
            return true;
          }
        });

        if contains_vec_type(&field.ty) {
          child_decl_tokens.push(quote! { let mut #field_ident = Vec::new(); });
          child_init_tokens.push(quote! { #field_ident });
          child_write_tokens.push(quote! {
            for child in &self.#field_ident {
              child.write_xml(writer, xmlns_prefix)?;
            }
          });
          child_validate_tokens.push(quote! {
            for child in &self.#field_ident {
              crate::validator::SdkValidator::validate(#validate_child_tokens)?;
            }
          });
          child_dispatch_tokens_borrowed.push(quote! {
            if matches!(event_name, #match_target) {
              match #child_ty::#deserialize_borrowed_inner_ident(#xml_reader_ident, Some((e, next_empty))) {
                Ok(parsed_child) => {
                  #field_ident.push(#parsed_child_expr);
                }
                Err(crate::common::SdkError::MissingField { .. }) => {}
                Err(err) => return Err(err),
              }
              continue;
            }
          });
          child_dispatch_tokens_io.push(quote! {
            if matches!(event_name, #match_target) {
              match #child_ty::#deserialize_io_inner_ident(#xml_reader_ident, Some((e, next_empty))) {
                Ok(parsed_child) => {
                  #field_ident.push(#parsed_child_expr);
                }
                Err(crate::common::SdkError::MissingField { .. }) => {}
                Err(err) => return Err(err),
              }
              continue;
            }
          });
        } else {
          child_decl_tokens.push(quote! { let mut #field_ident = None; });
          if is_option_type(&field.ty) {
            child_init_tokens.push(quote! { #field_ident });
          } else {
            child_init_tokens.push(quote! {
              #field_ident: #field_ident.ok_or_else(|| crate::common::missing_field(
                stringify!(#ident),
                stringify!(#field_ident),
              ))?
            });
          }
          if is_option_type(&field.ty) {
            child_write_tokens.push(quote! {
              if let Some(child) = &self.#field_ident {
                child.write_xml(writer, xmlns_prefix)?;
              }
            });
            child_validate_tokens.push(quote! {
              if let Some(child) = &self.#field_ident {
                crate::validator::SdkValidator::validate(#validate_child_tokens)?;
              }
            });
          } else {
            child_write_tokens.push(quote! {
              self.#field_ident.write_xml(writer, xmlns_prefix)?;
            });
            child_validate_tokens.push(quote! {
              crate::validator::SdkValidator::validate(#validate_self_tokens)?;
            });
          }
          child_dispatch_tokens_borrowed.push(quote! {
            if matches!(event_name, #match_target) {
              match #child_ty::#deserialize_borrowed_inner_ident(#xml_reader_ident, Some((e, next_empty))) {
                Ok(parsed_child) => {
                  #field_ident = Some(#parsed_child_expr);
                }
                Err(crate::common::SdkError::MissingField { .. }) => {}
                Err(err) => return Err(err),
              }
              continue;
            }
          });
          child_dispatch_tokens_io.push(quote! {
            if matches!(event_name, #match_target) {
              match #child_ty::#deserialize_io_inner_ident(#xml_reader_ident, Some((e, next_empty))) {
                Ok(parsed_child) => {
                  #field_ident = Some(#parsed_child_expr);
                }
                Err(crate::common::SdkError::MissingField { .. }) => {}
                Err(err) => return Err(err),
              }
              continue;
            }
          });
        }
      }
      Some(SdkTypeFieldKind::TextChild { qname }) => {
        if contains_vec_type(&field.ty) {
          child_decl_tokens.push(quote! { let mut #field_ident = Vec::new(); });
          child_init_tokens.push(quote! { #field_ident });
        } else {
          child_decl_tokens.push(quote! { let mut #field_ident = None; });
          if is_option_type(&field.ty) {
            child_init_tokens.push(quote! { #field_ident });
          } else {
            child_init_tokens.push(quote! {
              #field_ident: #field_ident.ok_or_else(|| crate::common::missing_field(
                stringify!(#ident),
                stringify!(#field_ident),
              ))?
            });
          }
        }
        child_write_tokens.push(build_text_child_write_tokens(
          field_ident,
          &qname,
          &field.ty,
          contains_vec_type(&field.ty),
          is_option_type(&field.ty),
        ));
        let match_target = text_child_match_target(&qname);
        matcher_checks.push(quote! {
          if matches!(name, #match_target) {
            return true;
          }
        });
        let parse_arm = build_text_child_parse_arm(
          ident,
          field_ident,
          &qname,
          &field.ty,
          contains_vec_type(&field.ty),
          false,
        );
        let text_child_dispatch = quote! {
          if matches!(event_name, #match_target) {
            match event_name {
              #parse_arm
              _ => false,
            };
            continue;
          }
        };
        child_dispatch_tokens_borrowed.push(text_child_dispatch.clone());
        child_dispatch_tokens_io.push(text_child_dispatch);
      }
      _ => {
        return Err(syn::Error::new_spanned(
          field,
          "sequence helper structs require only #[sdk(child(...))] or #[sdk(text_child(...))] fields",
        ));
      }
    }
  }

  Ok(sdk_type_impl_tokens(
    ident,
    quote! {
      #[inline]
      pub(crate) fn matches_specific_start_qname(name: &[u8]) -> bool {
        #( #matcher_checks )*
        false
      }

      #[inline]
      pub(crate) fn matches_start_qname(name: &[u8]) -> bool {
        Self::matches_specific_start_qname(name)
      }

      #[inline]
      pub(crate) const fn accepts_any_child() -> bool {
        false
      }

      pub(crate) fn #deserialize_borrowed_inner_ident<'de>(
        xml_reader: &mut crate::common::SliceReader<'de>,
        xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
      ) -> Result<Self, crate::common::SdkError> {
        let mut pending_event = xml_event;

        #( #child_decl_tokens )*

        loop {
          if let Some((e, next_empty)) = pending_event.take() {
            let event_name = e.name();
            let event_name = event_name.as_ref();
            #( #child_dispatch_tokens_borrowed )*
            xml_reader.unread(if next_empty {
              quick_xml::events::Event::Empty(e)
            } else {
              quick_xml::events::Event::Start(e)
            })?;
            break;
          }

          match xml_reader.next()? {
            quick_xml::events::Event::Start(e) => {
              pending_event = Some((e.into_owned(), false));
            }
            quick_xml::events::Event::Empty(e) => {
              pending_event = Some((e.into_owned(), true));
            }
            quick_xml::events::Event::End(e) => {
              xml_reader.unread(quick_xml::events::Event::End(e.into_owned()))?;
              break;
            }
            quick_xml::events::Event::Eof => {
              return Err(crate::common::unexpected_eof(stringify!(#ident)));
            }
            _ => {}
          }
        }

        Ok(Self {
          #( #child_init_tokens, )*
        })
      }

      pub(crate) fn #deserialize_io_inner_ident<R: std::io::BufRead>(
        xml_reader: &mut crate::common::IoReader<R>,
        xml_event: Option<(quick_xml::events::BytesStart<'static>, bool)>,
      ) -> Result<Self, crate::common::SdkError> {
        let mut pending_event = xml_event;

        #( #child_decl_tokens )*

        loop {
          if let Some((e, next_empty)) = pending_event.take() {
            let event_name = e.name();
            let event_name = event_name.as_ref();
            #( #child_dispatch_tokens_io )*
            xml_reader.unread(if next_empty {
              quick_xml::events::Event::Empty(e)
            } else {
              quick_xml::events::Event::Start(e)
            })?;
            break;
          }

          match xml_reader.next_tag_event()? {
            crate::common::IoTagEvent::Start(e, next_empty) => {
              pending_event = Some((e, next_empty));
            }
            crate::common::IoTagEvent::End(e) => {
              xml_reader.unread(quick_xml::events::Event::End(e))?;
              break;
            }
            crate::common::IoTagEvent::Eof => {
              return Err(crate::common::unexpected_eof(stringify!(#ident)));
            }
            crate::common::IoTagEvent::Decl(_) | crate::common::IoTagEvent::Other => {}
          }
        }

        Ok(Self {
          #( #child_init_tokens, )*
        })
      }

      pub(crate) fn write_xml<W: std::io::Write>(
        &self,
        writer: &mut W,
        xmlns_prefix: &str,
      ) -> Result<(), std::io::Error> {
        #( #child_write_tokens )*
        Ok(())
      }
    },
    quote! {
      #( #child_validate_tokens )*
    },
  ))
}

fn expand_helper_struct(
  input: &DeriveInput,
  fields: &syn::FieldsNamed,
) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let deserialize_borrowed_inner_ident = deserialize_type_inner_ident(DeserializeMode::Borrowed);
  let deserialize_io_inner_ident = deserialize_type_inner_ident(DeserializeMode::Io);

  let mut child_fields = Vec::new();
  let mut text_child_fields = Vec::new();
  let mut choice_fields = Vec::new();
  for field in &fields.named {
    let field_ident = field
      .ident
      .as_ref()
      .ok_or_else(|| syn::Error::new_spanned(field, "SdkType requires named fields"))?;
    let parsed_attrs = parse_sdk_type_field_attrs(&field.attrs)?;
    match parsed_attrs.kind {
      Some(SdkTypeFieldKind::Child { qname }) => child_fields.push(SdkChildField {
        ident: field_ident.clone(),
        qname,
        ty: field.ty.clone(),
        optional: is_option_type(&field.ty),
        repeated: contains_vec_type(&field.ty),
      }),
      Some(SdkTypeFieldKind::TextChild { qname }) => text_child_fields.push(SdkTextChildField {
        ident: field_ident.clone(),
        qname,
        ty: field.ty.clone(),
        optional: is_option_type(&field.ty),
        repeated: contains_vec_type(&field.ty),
      }),
      Some(SdkTypeFieldKind::Choice) => choice_fields.push(SdkChoiceField {
        ident: field_ident.clone(),
        ty: field.ty.clone(),
        optional: is_option_type(&field.ty),
        repeated: contains_vec_type(&field.ty),
      }),
      Some(SdkTypeFieldKind::Text) => {
        return Err(syn::Error::new_spanned(
          field,
          "helper structs do not support #[sdk(text)]",
        ));
      }
      Some(SdkTypeFieldKind::Any) => {
        return Err(syn::Error::new_spanned(
          field,
          "helper structs do not support #[sdk(any)]",
        ));
      }
      Some(SdkTypeFieldKind::Attr { .. }) | None => {
        return Err(syn::Error::new_spanned(
          field,
          "helper structs require #[sdk(child(...))], #[sdk(text_child(...))] or #[sdk(choice)] fields",
        ));
      }
    }
  }

  let mut child_decl_tokens = Vec::new();
  let mut direct_child_case_arms: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_dispatch_tokens_borrowed: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_dispatch_tokens_io: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_visit_dispatch_tokens_borrowed: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_visit_dispatch_tokens_io: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut child_parse_tokens = Vec::new();
  let mut child_visit_parse_tokens = Vec::new();
  let mut child_write_tokens = Vec::new();
  let mut child_init_tokens = Vec::new();
  let mut child_validate_tokens = Vec::new();
  let xml_reader_ident = Ident::new("xml_reader", Span::call_site());
  let visitor_reader_ident = Ident::new("xml_reader", Span::call_site());
  for field in &child_fields {
    let field_ident = &field.ident;
    let QNameInfo {
      tag_prefix,
      local_name,
    } = parse_qname_info(&field.qname);
    let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
    let payload_ty = unwrap_option_vec_type(&field.ty);
    let child_ty = if let Some(inner_ty) = box_inner_type(&payload_ty) {
      inner_ty
    } else {
      payload_ty.clone()
    };
    let validate_child_tokens = if box_inner_type(&payload_ty).is_some() {
      quote! { child.as_ref() }
    } else {
      quote! { child }
    };
    let validate_self_tokens = if box_inner_type(&payload_ty).is_some() {
      quote! { self.#field_ident.as_ref() }
    } else {
      quote! { &self.#field_ident }
    };
    let parsed_child_expr = if box_inner_type(&payload_ty).is_some() {
      quote! { std::boxed::Box::new(parsed_child) }
    } else {
      quote! { parsed_child }
    };
    let case_index = direct_child_dispatch_tokens_borrowed.len() + 1;
    let target = if tag_prefix.is_empty() {
      quote! { #local_name_lit }
    } else {
      let tag_qname_lit = LitByteStr::new(
        format!("{tag_prefix}:{local_name}").as_bytes(),
        Span::call_site(),
      );
      quote! { #tag_qname_lit | #local_name_lit }
    };
    let build_dispatch = |reader_ident: &Ident, as_result: bool, deserialize_ident: &Ident| {
      if field.repeated {
        if as_result {
          quote! {
              #case_index => {
                return match #child_ty::#deserialize_ident(#reader_ident, Some((e, next_empty))) {
                  Ok(parsed_child) => {
                    #field_ident.push(#parsed_child_expr);
                    Ok(true)
                  },
                  Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
                  Err(err) => Err(err),
                };
              },
          }
        } else {
          quote! {
              #case_index => {
                match #child_ty::#deserialize_ident(#reader_ident, Some((e, next_empty))) {
                  Ok(parsed_child) => {
                    #field_ident.push(#parsed_child_expr);
                    continue;
                  },
                  Err(crate::common::SdkError::MissingField { .. }) => continue,
                  Err(err) => return Err(err),
                }
              },
          }
        }
      } else {
        if as_result {
          quote! {
            #case_index => {
              return match #child_ty::#deserialize_ident(#reader_ident, Some((e, next_empty))) {
                Ok(parsed_child) => {
                  #field_ident = Some(#parsed_child_expr);
                  Ok(true)
                },
                Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
                Err(err) => Err(err),
              };
            },
          }
        } else {
          quote! {
            #case_index => {
              match #child_ty::#deserialize_ident(#reader_ident, Some((e, next_empty))) {
                Ok(parsed_child) => {
                  #field_ident = Some(#parsed_child_expr);
                  continue;
                },
                Err(crate::common::SdkError::MissingField { .. }) => continue,
                Err(err) => return Err(err),
              }
            },
          }
        }
      }
    };
    direct_child_case_arms.push(quote! { #target => #case_index, });

    if field.repeated {
      child_decl_tokens.push(quote! { let mut #field_ident = Vec::new(); });
      child_init_tokens.push(quote! { #field_ident });
      child_write_tokens.push(quote! {
        for child in &self.#field_ident {
          child.write_xml(writer, xmlns_prefix)?;
        }
      });
      child_validate_tokens.push(quote! {
        for child in &self.#field_ident {
          crate::validator::SdkValidator::validate(#validate_child_tokens)?;
        }
      });
      direct_child_dispatch_tokens_borrowed.push(build_dispatch(
        &xml_reader_ident,
        false,
        &deserialize_borrowed_inner_ident,
      ));
      direct_child_dispatch_tokens_io.push(build_dispatch(
        &xml_reader_ident,
        false,
        &deserialize_io_inner_ident,
      ));
      direct_child_visit_dispatch_tokens_borrowed.push(build_dispatch(
        &visitor_reader_ident,
        true,
        &deserialize_borrowed_inner_ident,
      ));
      direct_child_visit_dispatch_tokens_io.push(build_dispatch(
        &visitor_reader_ident,
        true,
        &deserialize_io_inner_ident,
      ));
    } else {
      child_decl_tokens.push(quote! { let mut #field_ident = None; });
      if field.optional {
        child_init_tokens.push(quote! { #field_ident });
      } else {
        child_init_tokens.push(quote! {
          #field_ident: #field_ident.ok_or_else(|| crate::common::missing_field(
            stringify!(#ident),
            stringify!(#field_ident),
          ))?
        });
      }
      if field.optional {
        child_write_tokens.push(quote! {
          if let Some(child) = &self.#field_ident {
            child.write_xml(writer, xmlns_prefix)?;
          }
        });
        child_validate_tokens.push(quote! {
          if let Some(child) = &self.#field_ident {
            crate::validator::SdkValidator::validate(#validate_child_tokens)?;
          }
        });
      } else {
        child_write_tokens.push(quote! {
          self.#field_ident.write_xml(writer, xmlns_prefix)?;
        });
        child_validate_tokens.push(quote! {
          crate::validator::SdkValidator::validate(#validate_self_tokens)?;
        });
      }
      direct_child_dispatch_tokens_borrowed.push(build_dispatch(
        &xml_reader_ident,
        false,
        &deserialize_borrowed_inner_ident,
      ));
      direct_child_dispatch_tokens_io.push(build_dispatch(
        &xml_reader_ident,
        false,
        &deserialize_io_inner_ident,
      ));
      direct_child_visit_dispatch_tokens_borrowed.push(build_dispatch(
        &visitor_reader_ident,
        true,
        &deserialize_borrowed_inner_ident,
      ));
      direct_child_visit_dispatch_tokens_io.push(build_dispatch(
        &visitor_reader_ident,
        true,
        &deserialize_io_inner_ident,
      ));
    }
  }

  let mut choice_decl_tokens = Vec::new();
  let mut choice_match_init_tokens = Vec::new();
  let mut choice_match_decl_tokens = Vec::new();
  let mut choice_unique_parse_tokens_borrowed = Vec::new();
  let mut choice_unique_parse_tokens_io = Vec::new();
  let mut choice_unique_visit_parse_tokens_borrowed = Vec::new();
  let mut choice_unique_visit_parse_tokens_io = Vec::new();
  let mut choice_any_unique_parse_tokens_borrowed = Vec::new();
  let mut choice_any_unique_parse_tokens_io = Vec::new();
  let mut choice_any_unique_visit_parse_tokens_borrowed = Vec::new();
  let mut choice_any_unique_visit_parse_tokens_io = Vec::new();
  let mut choice_parse_tokens = Vec::new();
  let mut choice_visit_parse_tokens = Vec::new();
  let mut choice_write_tokens = Vec::new();
  let mut choice_init_tokens = Vec::new();
  let mut choice_validate_tokens = Vec::new();
  for field in &choice_fields {
    let match_ident = Ident::new(
      &format!("__choice_match_{}", choice_match_init_tokens.len()),
      Span::call_site(),
    );
    let any_ident = Ident::new(
      &format!("__choice_any_{}", choice_match_init_tokens.len()),
      Span::call_site(),
    );
    let field_ident = &field.ident;
    let choice_ty = unwrap_option_vec_type(&field.ty);
    let validate_choice_tokens = if box_inner_type(&choice_ty).is_some() {
      quote! { choice.as_ref() }
    } else {
      quote! { choice }
    };
    let validate_self_tokens = if box_inner_type(&choice_ty).is_some() {
      quote! { self.#field_ident.as_ref() }
    } else {
      quote! { &self.#field_ident }
    };
    choice_match_init_tokens.push(quote! {
      let mut #match_ident = false;
      let mut #any_ident = false;
    });
    choice_match_decl_tokens.push(quote! {
      #match_ident = #choice_ty::matches_specific_start_qname(event_name);
      if #match_ident {
        specific_choice_match_count += 1usize;
      } else if #choice_ty::accepts_any_child() {
        #any_ident = true;
        any_choice_match_count += 1usize;
      }
    });
    if field.repeated {
      choice_unique_parse_tokens_borrowed.push(quote! {
        if #match_ident && specific_choice_match_count == 1usize {
          match #choice_ty::#deserialize_borrowed_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident.push(parsed_choice);
              continue;
            }
            Err(crate::common::SdkError::MissingField { .. }) => continue,
            Err(err) => return Err(err),
          }
        }
      });
      choice_unique_parse_tokens_io.push(quote! {
        if #match_ident && specific_choice_match_count == 1usize {
          match #choice_ty::#deserialize_io_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident.push(parsed_choice);
              continue;
            }
            Err(crate::common::SdkError::MissingField { .. }) => continue,
            Err(err) => return Err(err),
          }
        }
      });
      choice_unique_visit_parse_tokens_borrowed.push(quote! {
        if #match_ident && specific_choice_match_count == 1usize {
          return match #choice_ty::#deserialize_borrowed_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident.push(parsed_choice);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          };
        }
      });
      choice_unique_visit_parse_tokens_io.push(quote! {
        if #match_ident && specific_choice_match_count == 1usize {
          return match #choice_ty::#deserialize_io_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident.push(parsed_choice);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          };
        }
      });
      choice_any_unique_parse_tokens_borrowed.push(quote! {
        if #any_ident && specific_choice_match_count == 0usize && any_choice_match_count == 1usize {
          match #choice_ty::#deserialize_borrowed_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident.push(parsed_choice);
              continue;
            }
            Err(crate::common::SdkError::MissingField { .. }) => continue,
            Err(err) => return Err(err),
          }
        }
      });
      choice_any_unique_parse_tokens_io.push(quote! {
        if #any_ident && specific_choice_match_count == 0usize && any_choice_match_count == 1usize {
          match #choice_ty::#deserialize_io_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident.push(parsed_choice);
              continue;
            }
            Err(crate::common::SdkError::MissingField { .. }) => continue,
            Err(err) => return Err(err),
          }
        }
      });
      choice_any_unique_visit_parse_tokens_borrowed.push(quote! {
        if #any_ident && specific_choice_match_count == 0usize && any_choice_match_count == 1usize {
          return match #choice_ty::#deserialize_borrowed_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident.push(parsed_choice);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          };
        }
      });
      choice_any_unique_visit_parse_tokens_io.push(quote! {
        if #any_ident && specific_choice_match_count == 0usize && any_choice_match_count == 1usize {
          return match #choice_ty::#deserialize_io_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident.push(parsed_choice);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          };
        }
      });
    } else {
      choice_unique_parse_tokens_borrowed.push(quote! {
        if #match_ident && specific_choice_match_count == 1usize {
          match #choice_ty::#deserialize_borrowed_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident = Some(parsed_choice);
              continue;
            }
            Err(crate::common::SdkError::MissingField { .. }) => continue,
            Err(err) => return Err(err),
          }
        }
      });
      choice_unique_parse_tokens_io.push(quote! {
        if #match_ident && specific_choice_match_count == 1usize {
          match #choice_ty::#deserialize_io_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident = Some(parsed_choice);
              continue;
            }
            Err(crate::common::SdkError::MissingField { .. }) => continue,
            Err(err) => return Err(err),
          }
        }
      });
      choice_unique_visit_parse_tokens_borrowed.push(quote! {
        if #match_ident && specific_choice_match_count == 1usize {
          return match #choice_ty::#deserialize_borrowed_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident = Some(parsed_choice);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          };
        }
      });
      choice_unique_visit_parse_tokens_io.push(quote! {
        if #match_ident && specific_choice_match_count == 1usize {
          return match #choice_ty::#deserialize_io_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident = Some(parsed_choice);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          };
        }
      });
      choice_any_unique_parse_tokens_borrowed.push(quote! {
        if #any_ident && specific_choice_match_count == 0usize && any_choice_match_count == 1usize {
          match #choice_ty::#deserialize_borrowed_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident = Some(parsed_choice);
              continue;
            }
            Err(crate::common::SdkError::MissingField { .. }) => continue,
            Err(err) => return Err(err),
          }
        }
      });
      choice_any_unique_parse_tokens_io.push(quote! {
        if #any_ident && specific_choice_match_count == 0usize && any_choice_match_count == 1usize {
          match #choice_ty::#deserialize_io_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident = Some(parsed_choice);
              continue;
            }
            Err(crate::common::SdkError::MissingField { .. }) => continue,
            Err(err) => return Err(err),
          }
        }
      });
      choice_any_unique_visit_parse_tokens_borrowed.push(quote! {
        if #any_ident && specific_choice_match_count == 0usize && any_choice_match_count == 1usize {
          return match #choice_ty::#deserialize_borrowed_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident = Some(parsed_choice);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          };
        }
      });
      choice_any_unique_visit_parse_tokens_io.push(quote! {
        if #any_ident && specific_choice_match_count == 0usize && any_choice_match_count == 1usize {
          return match #choice_ty::#deserialize_io_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident = Some(parsed_choice);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          };
        }
      });
    }
    if field.repeated {
      choice_decl_tokens.push(quote! { let mut #field_ident = Vec::new(); });
      choice_init_tokens.push(quote! { #field_ident });
      choice_write_tokens.push(quote! {
        for choice in &self.#field_ident {
          choice.write_xml(writer, xmlns_prefix)?;
        }
      });
      choice_validate_tokens.push(quote! {
        for choice in &self.#field_ident {
          crate::validator::SdkValidator::validate(#validate_choice_tokens)?;
        }
      });
      choice_parse_tokens.push(quote! {});
      choice_visit_parse_tokens.push(quote! {});
    } else {
      choice_decl_tokens.push(quote! { let mut #field_ident = None; });
      if field.optional {
        choice_init_tokens.push(quote! { #field_ident });
      } else {
        choice_init_tokens.push(quote! {
          #field_ident: #field_ident.ok_or_else(|| crate::common::missing_field(
            stringify!(#ident),
            stringify!(#field_ident),
          ))?
        });
      }
      if field.optional {
        choice_write_tokens.push(quote! {
          if let Some(choice) = &self.#field_ident {
            choice.write_xml(writer, xmlns_prefix)?;
          }
        });
        choice_validate_tokens.push(quote! {
          if let Some(choice) = &self.#field_ident {
            crate::validator::SdkValidator::validate(#validate_choice_tokens)?;
          }
        });
      } else {
        choice_write_tokens.push(quote! {
          self.#field_ident.write_xml(writer, xmlns_prefix)?;
        });
        choice_validate_tokens.push(quote! {
          crate::validator::SdkValidator::validate(#validate_self_tokens)?;
        });
      }
      choice_parse_tokens.push(quote! {});
      choice_visit_parse_tokens.push(quote! {});
    }
  }

  for field in &text_child_fields {
    let field_ident = &field.ident;
    if field.repeated {
      child_decl_tokens.push(quote! { let mut #field_ident = Vec::new(); });
      child_init_tokens.push(quote! { #field_ident });
    } else {
      child_decl_tokens.push(quote! { let mut #field_ident = None; });
      if field.optional {
        child_init_tokens.push(quote! { #field_ident });
      } else {
        child_init_tokens.push(quote! {
          #field_ident: #field_ident.ok_or_else(|| crate::common::missing_field(
            stringify!(#ident),
            stringify!(#field_ident),
          ))?
        });
      }
    }
    child_write_tokens.push(build_text_child_write_tokens(
      field_ident,
      &field.qname,
      &field.ty,
      field.repeated,
      field.optional,
    ));
    child_parse_tokens.push(build_text_child_parse_arm(
      ident,
      field_ident,
      &field.qname,
      &field.ty,
      field.repeated,
      false,
    ));
    child_visit_parse_tokens.push(build_text_child_parse_arm(
      ident,
      field_ident,
      &field.qname,
      &field.ty,
      field.repeated,
      true,
    ));
  }

  if child_fields.len() + text_child_fields.len() == fields.named.len() {
    return expand_sequence_helper_struct(input, fields);
  }

  let has_child_dispatch = !child_fields.is_empty() || !text_child_fields.is_empty();
  let has_choice_dispatch = !choice_fields.is_empty();
  let visit_foreign_child_tokens_borrowed = if !has_child_dispatch && !has_choice_dispatch {
    quote! {
      let mut visit_foreign_child = |
        _xml_reader: &mut crate::common::SliceReader<'de>,
        _e: quick_xml::events::BytesStart<'de>,
        _next_empty: bool,
      | -> Result<bool, crate::common::SdkError> {
        Ok(false)
      };
    }
  } else if !has_choice_dispatch {
    quote! {
      let mut visit_foreign_child = |
        xml_reader: &mut crate::common::SliceReader<'de>,
        e: quick_xml::events::BytesStart<'de>,
        next_empty: bool,
      | -> Result<bool, crate::common::SdkError> {
        let event_name = e.name();
        let event_name = event_name.as_ref();
        let direct_child_case = match event_name {
          #( #direct_child_case_arms )*
          _ => 0usize,
        };
        match direct_child_case {
          #( #direct_child_visit_dispatch_tokens_borrowed )*
          _ => {}
        }
        match event_name {
          #( #child_visit_parse_tokens )*
          _ => Ok(false),
        }
      };
    }
  } else {
    quote! {
      let mut visit_foreign_child = |
        xml_reader: &mut crate::common::SliceReader<'de>,
        e: quick_xml::events::BytesStart<'de>,
        next_empty: bool,
      | -> Result<bool, crate::common::SdkError> {
        let event_name = e.name();
        let event_name = event_name.as_ref();
        let direct_child_case = match event_name {
          #( #direct_child_case_arms )*
          _ => 0usize,
        };
        match direct_child_case {
          #( #direct_child_visit_dispatch_tokens_borrowed )*
          _ => {}
        }
        #( #choice_match_init_tokens )*
        {
          let mut specific_choice_match_count = 0usize;
          let mut any_choice_match_count = 0usize;
          #( #choice_match_decl_tokens )*
          #( #choice_unique_visit_parse_tokens_borrowed )*
          #( #choice_any_unique_visit_parse_tokens_borrowed )*
          if specific_choice_match_count > 1usize || any_choice_match_count > 1usize {
            return Err(crate::common::unexpected_tag(
              stringify!(#ident),
              "known child",
              event_name,
            ));
          }
        }
        let matched: bool = match event_name {
          #( #child_visit_parse_tokens )*
          _ => {
            let mut matched = false;
            #( #choice_visit_parse_tokens )*
            Ok::<bool, crate::common::SdkError>(matched)
          }
        }?;
        Ok(matched)
      };
    }
  };
  let visit_foreign_child_tokens_io = if !has_child_dispatch && !has_choice_dispatch {
    quote! {
      let mut visit_foreign_child = |
        _xml_reader: &mut crate::common::IoReader<R>,
        _e: quick_xml::events::BytesStart<'static>,
        _next_empty: bool,
      | -> Result<bool, crate::common::SdkError> {
        Ok(false)
      };
    }
  } else if !has_choice_dispatch {
    quote! {
      let mut visit_foreign_child = |
        xml_reader: &mut crate::common::IoReader<R>,
        e: quick_xml::events::BytesStart<'static>,
        next_empty: bool,
      | -> Result<bool, crate::common::SdkError> {
        let event_name = e.name();
        let event_name = event_name.as_ref();
        let direct_child_case = match event_name {
          #( #direct_child_case_arms )*
          _ => 0usize,
        };
        match direct_child_case {
          #( #direct_child_visit_dispatch_tokens_io )*
          _ => {}
        }
        match event_name {
          #( #child_visit_parse_tokens )*
          _ => Ok(false),
        }
      };
    }
  } else {
    quote! {
      let mut visit_foreign_child = |
        xml_reader: &mut crate::common::IoReader<R>,
        e: quick_xml::events::BytesStart<'static>,
        next_empty: bool,
      | -> Result<bool, crate::common::SdkError> {
        let event_name = e.name();
        let event_name = event_name.as_ref();
        let direct_child_case = match event_name {
          #( #direct_child_case_arms )*
          _ => 0usize,
        };
        match direct_child_case {
          #( #direct_child_visit_dispatch_tokens_io )*
          _ => {}
        }
        #( #choice_match_init_tokens )*
        {
          let mut specific_choice_match_count = 0usize;
          let mut any_choice_match_count = 0usize;
          #( #choice_match_decl_tokens )*
          #( #choice_unique_visit_parse_tokens_io )*
          #( #choice_any_unique_visit_parse_tokens_io )*
          if specific_choice_match_count > 1usize || any_choice_match_count > 1usize {
            return Err(crate::common::unexpected_tag(
              stringify!(#ident),
              "known child",
              event_name,
            ));
          }
        }
        let matched: bool = match event_name {
          #( #child_visit_parse_tokens )*
          _ => {
            let mut matched = false;
            #( #choice_visit_parse_tokens )*
            Ok::<bool, crate::common::SdkError>(matched)
          }
        }?;
        Ok(matched)
      };
    }
  };
  let main_dispatch_tokens_borrowed = if !has_choice_dispatch {
    quote! {
      let direct_child_case = match event_name {
        #( #direct_child_case_arms )*
        _ => 0usize,
      };
      match direct_child_case {
        #( #direct_child_dispatch_tokens_borrowed )*
        _ => {}
      }
      let matched = match event_name {
        #( #child_parse_tokens )*
        _ => false,
      };
      if matched {
        continue;
      }
    }
  } else {
    quote! {
      let direct_child_case = match event_name {
        #( #direct_child_case_arms )*
        _ => 0usize,
      };
      match direct_child_case {
        #( #direct_child_dispatch_tokens_borrowed )*
        _ => {}
      }
      #( #choice_match_init_tokens )*
      {
        let mut specific_choice_match_count = 0usize;
        let mut any_choice_match_count = 0usize;
        #( #choice_match_decl_tokens )*
        #( #choice_unique_parse_tokens_borrowed )*
        #( #choice_any_unique_parse_tokens_borrowed )*
        if specific_choice_match_count > 1usize || any_choice_match_count > 1usize {
          return Err(crate::common::unexpected_tag(
            stringify!(#ident),
            "known child",
            event_name,
          ));
        }
      }
      let matched = match event_name {
        #( #child_parse_tokens )*
        _ => {
          let mut matched = false;
          #( #choice_parse_tokens )*
          matched
        }
      };
      if matched {
        continue;
      }
    }
  };
  let main_dispatch_tokens_io = if !has_choice_dispatch {
    quote! {
      let direct_child_case = match event_name {
        #( #direct_child_case_arms )*
        _ => 0usize,
      };
      match direct_child_case {
        #( #direct_child_dispatch_tokens_io )*
        _ => {}
      }
      let matched = match event_name {
        #( #child_parse_tokens )*
        _ => false,
      };
      if matched {
        continue;
      }
    }
  } else {
    quote! {
      let direct_child_case = match event_name {
        #( #direct_child_case_arms )*
        _ => 0usize,
      };
      match direct_child_case {
        #( #direct_child_dispatch_tokens_io )*
        _ => {}
      }
      #( #choice_match_init_tokens )*
      {
        let mut specific_choice_match_count = 0usize;
        let mut any_choice_match_count = 0usize;
        #( #choice_match_decl_tokens )*
        #( #choice_unique_parse_tokens_io )*
        #( #choice_any_unique_parse_tokens_io )*
        if specific_choice_match_count > 1usize || any_choice_match_count > 1usize {
          return Err(crate::common::unexpected_tag(
            stringify!(#ident),
            "known child",
            event_name,
          ));
        }
      }
      let matched = match event_name {
        #( #child_parse_tokens )*
        _ => {
          let mut matched = false;
          #( #choice_parse_tokens )*
          matched
        }
      };
      if matched {
        continue;
      }
    }
  };

  Ok(quote! {
    impl crate::sdk::SdkType for #ident {}
    #[cfg(feature = "validators")]
    impl crate::validator::SdkValidator for #ident {
      fn validate(&self) -> Result<(), crate::common::SdkError> {
        #( #child_validate_tokens )*
        #( #choice_validate_tokens )*
        Ok(())
      }
    }

    impl std::str::FromStr for #ident {
      type Err = crate::common::SdkError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut xml_reader = crate::common::from_str_inner(s)?;
        Self::#deserialize_borrowed_inner_ident(&mut xml_reader, None)
      }
    }

    impl #ident {
      pub fn from_bytes(bytes: &[u8]) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_bytes_inner(bytes)?;
        Self::#deserialize_borrowed_inner_ident(&mut xml_reader, None)
      }

      pub fn from_str(s: &str) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_str_inner(s)?;
        Self::#deserialize_borrowed_inner_ident(&mut xml_reader, None)
      }

      pub fn from_reader<R: std::io::BufRead>(
        reader: R,
      ) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_reader_inner(reader)?;
        Self::#deserialize_io_inner_ident(&mut xml_reader, None)
      }

      pub(crate) fn #deserialize_borrowed_inner_ident<'de>(
        xml_reader: &mut crate::common::SliceReader<'de>,
        xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
      ) -> Result<Self, crate::common::SdkError> {
        let mut pending_event = xml_event;

        #( #child_decl_tokens )*
        #( #choice_decl_tokens )*

        loop {
          if let Some((e, next_empty)) = pending_event.take() {
            let event_name = e.name();
            let event_name = event_name.as_ref();
            #main_dispatch_tokens_borrowed
            match event_name {
              b"mc:AlternateContent" | b"AlternateContent" => {
                #visit_foreign_child_tokens_borrowed
                crate::common::process_markup_compatibility_children_borrowed(
                  xml_reader,
                  next_empty,
                  &mut visit_foreign_child,
                )?;
                continue;
              }
              _ => {
                if crate::common::is_foreign_prefixed_child(event_name, "") {
                  #visit_foreign_child_tokens_borrowed
                  crate::common::process_foreign_element_children_borrowed(
                  xml_reader,
                  next_empty,
                  &mut visit_foreign_child,
                  )?;
                  continue;
                }
              }
            }
            return Err(crate::common::unexpected_tag(
              stringify!(#ident),
              "known child",
              event_name,
            ));
          }

          match xml_reader.next()? {
            quick_xml::events::Event::Start(e) => {
              let e = e.into_owned();
              let next_empty = false;
              let event_name = e.name();
              let event_name = event_name.as_ref();
              #main_dispatch_tokens_borrowed
              match event_name {
                b"mc:AlternateContent" | b"AlternateContent" => {
                  #visit_foreign_child_tokens_borrowed
                  crate::common::process_markup_compatibility_children_borrowed(
                    xml_reader,
                    next_empty,
                    &mut visit_foreign_child,
                  )?;
                  continue;
                }
                _ => {
                  if crate::common::is_foreign_prefixed_child(event_name, "") {
                    #visit_foreign_child_tokens_borrowed
                    crate::common::process_foreign_element_children_borrowed(
                    xml_reader,
                    next_empty,
                    &mut visit_foreign_child,
                    )?;
                    continue;
                  }
                }
              }
              return Err(crate::common::unexpected_tag(
                stringify!(#ident),
                "known child",
                event_name,
              ));
            }
            quick_xml::events::Event::Empty(e) => {
              let e = e.into_owned();
              let next_empty = true;
              let event_name = e.name();
              let event_name = event_name.as_ref();
              #main_dispatch_tokens_borrowed
              match event_name {
                b"mc:AlternateContent" | b"AlternateContent" => {
                  #visit_foreign_child_tokens_borrowed
                  crate::common::process_markup_compatibility_children_borrowed(
                    xml_reader,
                    next_empty,
                    &mut visit_foreign_child,
                  )?;
                  continue;
                }
                _ => {
                  if crate::common::is_foreign_prefixed_child(event_name, "") {
                    #visit_foreign_child_tokens_borrowed
                    crate::common::process_foreign_element_children_borrowed(
                    xml_reader,
                    next_empty,
                    &mut visit_foreign_child,
                    )?;
                    continue;
                  }
                }
              }
              return Err(crate::common::unexpected_tag(
                stringify!(#ident),
                "known child",
                event_name,
              ));
            }
            quick_xml::events::Event::End(_) => {
              break;
            }
            quick_xml::events::Event::Eof => {
              return Err(crate::common::unexpected_eof(stringify!(#ident)));
            }
            _ => continue,
          }
        }

        Ok(Self {
          #( #child_init_tokens, )*
          #( #choice_init_tokens, )*
        })
      }

      pub(crate) fn #deserialize_io_inner_ident<R: std::io::BufRead>(
        xml_reader: &mut crate::common::IoReader<R>,
        xml_event: Option<(quick_xml::events::BytesStart<'static>, bool)>,
      ) -> Result<Self, crate::common::SdkError> {
        let mut pending_event = xml_event;

        #( #child_decl_tokens )*
        #( #choice_decl_tokens )*

        loop {
          if let Some((e, next_empty)) = pending_event.take() {
            let event_name = e.name();
            let event_name = event_name.as_ref();
            #main_dispatch_tokens_io
            match event_name {
              b"mc:AlternateContent" | b"AlternateContent" => {
                #visit_foreign_child_tokens_io
                crate::common::process_markup_compatibility_children_io(
                  xml_reader,
                  next_empty,
                  &mut visit_foreign_child,
                )?;
                continue;
              }
              _ => {
                if crate::common::is_foreign_prefixed_child(event_name, "") {
                  #visit_foreign_child_tokens_io
                  crate::common::process_foreign_element_children_io(
                    xml_reader,
                    next_empty,
                    &mut visit_foreign_child,
                  )?;
                  continue;
                }
              }
            }
            return Err(crate::common::unexpected_tag(
              stringify!(#ident),
              "known child",
              event_name,
            ));
          }

          let next_event = match xml_reader.next_tag_event()? {
            crate::common::IoTagEvent::Start(e, next_empty) => Some((e, next_empty)),
            crate::common::IoTagEvent::End(_) => break,
            crate::common::IoTagEvent::Eof => {
              return Err(crate::common::unexpected_eof(stringify!(#ident)));
            }
            crate::common::IoTagEvent::Decl(_) | crate::common::IoTagEvent::Other => None,
          };

          if let Some((e, next_empty)) = next_event {
            let event_name = e.name();
            let event_name = event_name.as_ref();
            #main_dispatch_tokens_io
            match event_name {
              b"mc:AlternateContent" | b"AlternateContent" => {
                #visit_foreign_child_tokens_io
                crate::common::process_markup_compatibility_children_io(
                  xml_reader,
                  next_empty,
                  &mut visit_foreign_child,
                )?;
                continue;
              }
              _ => {
                if crate::common::is_foreign_prefixed_child(event_name, "") {
                  #visit_foreign_child_tokens_io
                  crate::common::process_foreign_element_children_io(
                    xml_reader,
                    next_empty,
                    &mut visit_foreign_child,
                  )?;
                  continue;
                }
              }
            }
            return Err(crate::common::unexpected_tag(
              stringify!(#ident),
              "known child",
              event_name,
            ));
          }
        }

        Ok(Self {
          #( #child_init_tokens, )*
          #( #choice_init_tokens, )*
        })
      }
    }
  })
}

fn expand_named_struct(
  input: &DeriveInput,
  schema_qname: &str,
  fields: &syn::FieldsNamed,
) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let QNameInfo {
    tag_prefix,
    local_name,
  } = parse_qname_info(schema_qname);
  if local_name.is_empty() {
    return Ok(quote! {
      impl crate::sdk::SdkType for #ident {}
      #[cfg(feature = "validators")]
      impl crate::validator::SdkValidator for #ident {}
    });
  }
  let tag_qname = if tag_prefix.is_empty() {
    local_name.clone()
  } else {
    format!("{tag_prefix}:{local_name}")
  };
  let tag_qname_lit = LitByteStr::new(tag_qname.as_bytes(), Span::call_site());
  let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
  let deserialize_borrowed_inner_ident = deserialize_type_inner_ident(DeserializeMode::Borrowed);
  let deserialize_io_inner_ident = deserialize_type_inner_ident(DeserializeMode::Io);

  let mut attr_fields = Vec::new();
  let mut child_fields = Vec::new();
  let mut text_child_fields = Vec::new();
  let mut choice_fields = Vec::new();
  let mut any_fields = Vec::new();
  let mut text_field = None;
  let mut xmlns_fields = Vec::new();
  let mut xml_header_field = None;
  let mut mc_ignorable_field = None;
  let mut ordered_field_specs = Vec::new();

  for field in &fields.named {
    let field_ident = field
      .ident
      .as_ref()
      .ok_or_else(|| syn::Error::new_spanned(field, "SdkType requires named fields"))?;
    if is_xmlns_field(field_ident) {
      xmlns_fields.push(field_ident.clone());
      continue;
    }
    if field_ident == "xml_header" {
      xml_header_field = Some(field_ident.clone());
      continue;
    }
    if is_mc_ignorable_field(field_ident) {
      mc_ignorable_field = Some(field_ident.clone());
      continue;
    }

    let parsed_attrs = parse_sdk_type_field_attrs(&field.attrs)?;
    let Some(field_kind) = parsed_attrs.kind else {
      return Err(syn::Error::new_spanned(
        field,
        "missing #[sdk(...)] field attribute",
      ));
    };
    ordered_field_specs.push((field_ident.clone(), field.ty.clone(), field_kind.clone()));

    match field_kind {
      SdkTypeFieldKind::Attr { name } => attr_fields.push(SdkAttrField {
        ident: field_ident.clone(),
        name,
        ty: field.ty.clone(),
        optional: is_option_type(&field.ty),
        validators: parsed_attrs.validators,
      }),
      SdkTypeFieldKind::Child { qname } => child_fields.push(SdkChildField {
        ident: field_ident.clone(),
        qname,
        ty: field.ty.clone(),
        optional: is_option_type(&field.ty),
        repeated: contains_vec_type(&field.ty),
      }),
      SdkTypeFieldKind::TextChild { qname } => text_child_fields.push(SdkTextChildField {
        ident: field_ident.clone(),
        qname,
        ty: field.ty.clone(),
        optional: is_option_type(&field.ty),
        repeated: contains_vec_type(&field.ty),
      }),
      SdkTypeFieldKind::Choice => choice_fields.push(SdkChoiceField {
        ident: field_ident.clone(),
        ty: field.ty.clone(),
        optional: is_option_type(&field.ty),
        repeated: contains_vec_type(&field.ty),
      }),
      SdkTypeFieldKind::Any => any_fields.push(SdkAnyField {
        ident: field_ident.clone(),
        optional: is_option_type(&field.ty),
        repeated: contains_vec_type(&field.ty),
      }),
      SdkTypeFieldKind::Text => {
        text_field = Some(SdkTextField {
          ident: field_ident.clone(),
          ty: field.ty.clone(),
          optional: is_option_type(&field.ty),
        });
      }
    }
  }

  if any_fields.len() > 1 {
    return Err(syn::Error::new_spanned(
      input,
      "SdkType currently supports at most one #[sdk(any)] field",
    ));
  }

  let has_xmlns_fields = !xmlns_fields.is_empty();
  let has_xml_header_field = xml_header_field.is_some();
  let has_mc_ignorable_field = mc_ignorable_field.is_some();

  let mut attr_decl_tokens = Vec::new();
  let mut attr_parse_tokens = Vec::new();
  let mut attr_write_tokens = Vec::new();
  let mut attr_init_tokens = Vec::new();
  let mut attr_finish_tokens = Vec::new();
  let mut attr_validate_tokens = Vec::new();
  for field in &attr_fields {
    let field_ident = &field.ident;
    let name_lit = LitStr::new(&field.name, Span::call_site());
    let name_bytes_lit = LitByteStr::new(field.name.as_bytes(), Span::call_site());
    let parse_ty = unwrap_wrapped_type(&field.ty);
    let parser = if is_bool_type(&parse_ty) {
      parse_bool_attr_tokens(
        quote! { &attr },
        quote! { xml_reader.decoder() },
        &parse_ty,
        quote! { stringify!(#ident) },
        quote! { #name_lit },
      )
    } else if is_string_like_type(&parse_ty) {
      quote! { crate::common::decode_attr_value(&attr, xml_reader.decoder())? }
    } else {
      quote! { crate::common::parse_attr_value::<#parse_ty>(&attr, xml_reader.decoder(), stringify!(#ident), #name_lit)? }
    };
    if field.optional {
      attr_decl_tokens.push(quote! { let mut #field_ident = None; });
      attr_parse_tokens.push(quote! {
        #name_bytes_lit => {
          #field_ident = Some(#parser);
        }
      });
      attr_init_tokens.push(quote! { #field_ident });
    } else {
      attr_decl_tokens.push(quote! { let mut #field_ident = None; });
      attr_parse_tokens.push(quote! {
        #name_bytes_lit => {
          #field_ident = Some(#parser);
        }
      });
      attr_finish_tokens.push(quote! {
        #field_ident: #field_ident.ok_or_else(|| crate::common::missing_field(
          stringify!(#ident),
          stringify!(#field_ident),
        ))?
      });
    }
    let attr_write_value_tokens = if is_xml_schema_float_type(&parse_ty) {
      let write_value_tokens = write_xml_schema_float_tokens(quote! { value }, &parse_ty);
      quote! {
        writer.write_all(b" ")?;
        writer.write_all(#name_lit.as_bytes())?;
        writer.write_all(b"=\"")?;
        #write_value_tokens
        writer.write_all(b"\"")?;
      }
    } else if is_bool_type(&parse_ty) {
      let write_value_tokens = write_bool_tokens(quote! { value }, &parse_ty);
      quote! {
        writer.write_all(b" ")?;
        writer.write_all(#name_lit.as_bytes())?;
        writer.write_all(b"=\"")?;
        #write_value_tokens
        writer.write_all(b"\"")?;
      }
    } else if is_string_like_type(&parse_ty) {
      quote! {
        crate::common::write_attr_value_str(writer, #name_lit, value.as_ref())?;
      }
    } else {
      quote! {
        crate::common::write_attr_value(writer, #name_lit, value)?;
      }
    };
    if field.optional {
      attr_write_tokens.push(quote! {
        if let Some(value) = &self.#field_ident {
          #attr_write_value_tokens
        }
      });
    } else {
      attr_write_tokens.push(quote! {
        let value = &self.#field_ident;
        #attr_write_value_tokens
      });
    }

    let mut direct_validator_tokens = Vec::new();
    let mut union_validator_tokens: std::collections::BTreeMap<u32, Vec<proc_macro2::TokenStream>> =
      std::collections::BTreeMap::new();
    for validator in &field.validators {
      let token = validator_token(ident, field_ident, &parse_ty, validator);
      let (source_id, union_id) = validator_source_union(validator);
      if union_id.is_some() {
        union_validator_tokens
          .entry(source_id)
          .or_default()
          .push(token);
      } else {
        direct_validator_tokens.push(token);
      }
    }
    let union_validator_tokens: Vec<_> = if union_validator_tokens.is_empty() {
      Vec::new()
    } else {
      let branch_tokens: Vec<_> = union_validator_tokens
        .into_values()
        .map(|tokens| {
          quote! {
            (|| -> Result<(), crate::common::SdkError> {
              #( #tokens )*
              Ok::<(), crate::common::SdkError>(())
            })()
          }
        })
        .collect();
      vec![quote! {
        {
          let mut first_error: Option<crate::common::SdkError> = None;
          let mut matched = false;
          for branch_result in [#( #branch_tokens ),*] {
            match branch_result {
              Ok(()) => {
                matched = true;
                break;
              }
              Err(err) => {
                if first_error.is_none() {
                  first_error = Some(err);
                }
              }
            }
          }
          if !matched {
            return Err(first_error.unwrap_or_else(|| {
              crate::common::SdkError::CommonError(format!(
                "all union validator branches failed for {}.{}",
                stringify!(#ident),
                stringify!(#field_ident),
              ))
            }));
          }
        }
      }]
    };
    let mut validator_tokens = direct_validator_tokens;
    validator_tokens.extend(union_validator_tokens);
    if !validator_tokens.is_empty() {
      if field.optional {
        attr_validate_tokens.push(quote! {
          if let Some(value) = &self.#field_ident {
            #( #validator_tokens )*
          }
        });
      } else {
        attr_validate_tokens.push(quote! {
          {
            let value = &self.#field_ident;
            #( #validator_tokens )*
          }
        });
      }
    }
  }

  let xmlns_parse_tokens = if has_xmlns_fields {
    quote! {
      b"xmlns" => {
        xmlns = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?);
      }
      key if key.starts_with(b"xmlns:") => {
        let prefix = String::from_utf8_lossy(&key[6..]).into_owned();
        let uri = crate::common::decode_attr_value(&attr, xml_reader.decoder())?;
        if let Some(canonical_prefix) = crate::namespaces::prefix_by_uri(uri.as_str()) {
          xmlns_map.entry(canonical_prefix.to_string()).or_insert(uri);
        } else {
          xmlns_map.insert(prefix, uri);
        }
      }
    }
  } else {
    quote! {}
  };
  let mc_ignorable_parse_tokens = if has_mc_ignorable_field {
    quote! {
      b"mc:Ignorable" => {
        mc_ignorable = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?);
      }
    }
  } else {
    quote! {}
  };
  let namespace_attr_parse_tokens =
    if attr_fields.is_empty() && !has_xmlns_fields && !has_mc_ignorable_field {
      quote! {}
    } else {
      match (has_xmlns_fields, has_mc_ignorable_field) {
        (true, true) => quote! {
          let mut xmlns = None;
          let mut xmlns_map = std::collections::HashMap::<String, String>::new();
          let mut mc_ignorable = None;
          for attr in e.attributes().with_checks(false) {
            let attr = attr?;
            match attr.key.as_ref() {
              #xmlns_parse_tokens
              #mc_ignorable_parse_tokens
              #( #attr_parse_tokens )*
              _ => {}
            }
          }
        },
        (true, false) => quote! {
          let mut xmlns = None;
          let mut xmlns_map = std::collections::HashMap::<String, String>::new();
          for attr in e.attributes().with_checks(false) {
            let attr = attr?;
            match attr.key.as_ref() {
              #xmlns_parse_tokens
              #( #attr_parse_tokens )*
              _ => {}
            }
          }
        },
        (false, true) => quote! {
          let mut mc_ignorable = None;
          for attr in e.attributes().with_checks(false) {
            let attr = attr?;
            match attr.key.as_ref() {
              #mc_ignorable_parse_tokens
              #( #attr_parse_tokens )*
              _ => {}
            }
          }
        },
        (false, false) => quote! {
          for attr in e.attributes().with_checks(false) {
            let attr = attr?;
            match attr.key.as_ref() {
              #( #attr_parse_tokens )*
              _ => {}
            }
          }
        },
      }
    };
  let xml_header_decl_tokens = if has_xml_header_field {
    quote! {
      let mut xml_header_state = crate::common::XmlHeaderType::default();
    }
  } else {
    quote! {
      let mut xml_header_state = crate::common::XmlHeaderType::None;
    }
  };
  let mut direct_child_case_arms: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_dispatch_tokens_borrowed: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_dispatch_tokens_io: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_visit_dispatch_tokens_borrowed: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_visit_dispatch_tokens_io: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut child_decl_tokens = Vec::new();
  let mut child_parse_tokens = Vec::new();
  let mut child_visit_parse_tokens = Vec::new();
  let mut child_write_tokens = Vec::new();
  let mut child_init_tokens = Vec::new();
  let mut child_validate_tokens = Vec::new();
  let xml_reader_ident = Ident::new("xml_reader", Span::call_site());
  let visitor_reader_ident = Ident::new("xml_reader", Span::call_site());
  for field in &child_fields {
    let field_ident = &field.ident;
    let QNameInfo {
      tag_prefix,
      local_name,
    } = parse_qname_info(&field.qname);
    let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
    let payload_ty = unwrap_option_vec_type(&field.ty);
    let child_ty = if let Some(inner_ty) = box_inner_type(&payload_ty) {
      inner_ty
    } else {
      payload_ty.clone()
    };
    let validate_child_tokens = if box_inner_type(&payload_ty).is_some() {
      quote! { child.as_ref() }
    } else {
      quote! { child }
    };
    let validate_self_tokens = if box_inner_type(&payload_ty).is_some() {
      quote! { self.#field_ident.as_ref() }
    } else {
      quote! { &self.#field_ident }
    };
    let parsed_child_expr = if box_inner_type(&payload_ty).is_some() {
      quote! { std::boxed::Box::new(parsed_child) }
    } else {
      quote! { parsed_child }
    };
    let case_index = direct_child_dispatch_tokens_borrowed.len() + 1;
    let target = if tag_prefix.is_empty() {
      quote! { #local_name_lit }
    } else {
      let tag_qname_lit = LitByteStr::new(
        format!("{tag_prefix}:{local_name}").as_bytes(),
        Span::call_site(),
      );
      quote! { #tag_qname_lit | #local_name_lit }
    };
    let build_dispatch = |reader_ident: &Ident, as_result: bool, deserialize_ident: &Ident| {
      if field.repeated {
        if as_result {
          quote! {
            #case_index => {
              return match #child_ty::#deserialize_ident(#reader_ident, Some((e, next_empty))) {
                Ok(parsed_child) => {
                  #field_ident.push(#parsed_child_expr);
                  Ok(true)
                },
                Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
                Err(err) => Err(err),
              };
            },
          }
        } else {
          quote! {
            #case_index => {
              match #child_ty::#deserialize_ident(#reader_ident, Some((e, next_empty))) {
                Ok(parsed_child) => {
                  #field_ident.push(#parsed_child_expr);
                  continue;
                },
                Err(crate::common::SdkError::MissingField { .. }) => continue,
                Err(err) => return Err(err),
              }
            },
          }
        }
      } else {
        if as_result {
          quote! {
            #case_index => {
              return match #child_ty::#deserialize_ident(#reader_ident, Some((e, next_empty))) {
                Ok(parsed_child) => {
                  #field_ident = Some(#parsed_child_expr);
                  Ok(true)
                },
                Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
                Err(err) => Err(err),
              };
            },
          }
        } else {
          quote! {
            #case_index => {
              match #child_ty::#deserialize_ident(#reader_ident, Some((e, next_empty))) {
                Ok(parsed_child) => {
                  #field_ident = Some(#parsed_child_expr);
                  continue;
                },
                Err(crate::common::SdkError::MissingField { .. }) => continue,
                Err(err) => return Err(err),
              }
            },
          }
        }
      }
    };

    if field.repeated {
      child_decl_tokens.push(quote! { let mut #field_ident = Vec::new(); });
      child_init_tokens.push(quote! { #field_ident });
      child_write_tokens.push(quote! {
        for child in &self.#field_ident {
          child.write_xml(writer, xmlns_prefix)?;
        }
      });
      child_validate_tokens.push(quote! {
        for child in &self.#field_ident {
          crate::validator::SdkValidator::validate(#validate_child_tokens)?;
        }
      });
      direct_child_case_arms.push(quote! { #target => #case_index, });
      direct_child_dispatch_tokens_borrowed.push(build_dispatch(
        &xml_reader_ident,
        false,
        &deserialize_borrowed_inner_ident,
      ));
      direct_child_dispatch_tokens_io.push(build_dispatch(
        &xml_reader_ident,
        false,
        &deserialize_io_inner_ident,
      ));
      direct_child_visit_dispatch_tokens_borrowed.push(build_dispatch(
        &visitor_reader_ident,
        true,
        &deserialize_borrowed_inner_ident,
      ));
      direct_child_visit_dispatch_tokens_io.push(build_dispatch(
        &visitor_reader_ident,
        true,
        &deserialize_io_inner_ident,
      ));
    } else {
      child_decl_tokens.push(quote! { let mut #field_ident = None; });
      if field.optional {
        child_init_tokens.push(quote! { #field_ident });
      } else {
        child_init_tokens.push(quote! {
          #field_ident: #field_ident.ok_or_else(|| crate::common::missing_field(
            stringify!(#ident),
            stringify!(#field_ident),
          ))?
        });
      }
      if field.optional {
        child_write_tokens.push(quote! {
          if let Some(child) = &self.#field_ident {
            child.write_xml(writer, xmlns_prefix)?;
          }
        });
        child_validate_tokens.push(quote! {
          if let Some(child) = &self.#field_ident {
            crate::validator::SdkValidator::validate(#validate_child_tokens)?;
          }
        });
      } else {
        child_write_tokens.push(quote! {
          self.#field_ident.write_xml(writer, xmlns_prefix)?;
        });
        child_validate_tokens.push(quote! {
          crate::validator::SdkValidator::validate(#validate_self_tokens)?;
        });
      }
      direct_child_case_arms.push(quote! { #target => #case_index, });
      direct_child_dispatch_tokens_borrowed.push(build_dispatch(
        &xml_reader_ident,
        false,
        &deserialize_borrowed_inner_ident,
      ));
      direct_child_dispatch_tokens_io.push(build_dispatch(
        &xml_reader_ident,
        false,
        &deserialize_io_inner_ident,
      ));
      direct_child_visit_dispatch_tokens_borrowed.push(build_dispatch(
        &visitor_reader_ident,
        true,
        &deserialize_borrowed_inner_ident,
      ));
      direct_child_visit_dispatch_tokens_io.push(build_dispatch(
        &visitor_reader_ident,
        true,
        &deserialize_io_inner_ident,
      ));
    }
  }

  for field in &text_child_fields {
    let field_ident = &field.ident;
    if field.repeated {
      child_decl_tokens.push(quote! { let mut #field_ident = Vec::new(); });
      child_init_tokens.push(quote! { #field_ident });
    } else {
      child_decl_tokens.push(quote! { let mut #field_ident = None; });
      if field.optional {
        child_init_tokens.push(quote! { #field_ident });
      } else {
        child_init_tokens.push(quote! {
          #field_ident: #field_ident.ok_or_else(|| crate::common::missing_field(
            stringify!(#ident),
            stringify!(#field_ident),
          ))?
        });
      }
    }
    child_write_tokens.push(build_text_child_write_tokens(
      field_ident,
      &field.qname,
      &field.ty,
      field.repeated,
      field.optional,
    ));
    child_parse_tokens.push(build_text_child_parse_arm(
      ident,
      field_ident,
      &field.qname,
      &field.ty,
      field.repeated,
      false,
    ));
    child_visit_parse_tokens.push(build_text_child_parse_arm(
      ident,
      field_ident,
      &field.qname,
      &field.ty,
      field.repeated,
      true,
    ));
  }

  let mut choice_decl_tokens = Vec::new();
  let mut choice_match_init_tokens = Vec::new();
  let mut choice_match_decl_tokens = Vec::new();
  let mut choice_unique_parse_tokens_borrowed = Vec::new();
  let mut choice_unique_parse_tokens_io = Vec::new();
  let mut choice_unique_visit_parse_tokens_borrowed = Vec::new();
  let mut choice_unique_visit_parse_tokens_io = Vec::new();
  let mut choice_any_unique_parse_tokens_borrowed = Vec::new();
  let mut choice_any_unique_parse_tokens_io = Vec::new();
  let mut choice_any_unique_visit_parse_tokens_borrowed = Vec::new();
  let mut choice_any_unique_visit_parse_tokens_io = Vec::new();
  let mut choice_parse_tokens = Vec::new();
  let mut choice_visit_parse_tokens = Vec::new();
  let mut choice_write_tokens = Vec::new();
  let mut choice_init_tokens = Vec::new();
  let mut choice_text_parse_tokens = Vec::new();
  let mut choice_validate_tokens = Vec::new();
  for field in &choice_fields {
    let match_ident = Ident::new(
      &format!("__choice_match_{}", choice_match_init_tokens.len()),
      Span::call_site(),
    );
    let any_ident = Ident::new(
      &format!("__choice_any_{}", choice_match_init_tokens.len()),
      Span::call_site(),
    );
    let field_ident = &field.ident;
    let choice_ty = unwrap_option_vec_type(&field.ty);
    let validate_choice_tokens = if box_inner_type(&choice_ty).is_some() {
      quote! { choice.as_ref() }
    } else {
      quote! { choice }
    };
    let validate_self_tokens = if box_inner_type(&choice_ty).is_some() {
      quote! { self.#field_ident.as_ref() }
    } else {
      quote! { &self.#field_ident }
    };
    let build_text_block = |string_expr: proc_macro2::TokenStream| {
      if field.repeated {
        quote! {
          if let Some(parsed_choice) = #choice_ty::from_text_value(#string_expr) {
            #field_ident.push(parsed_choice);
            handled_text = true;
          }
        }
      } else {
        quote! {
          if !handled_text {
            if let Some(parsed_choice) = #choice_ty::from_text_value(#string_expr) {
              #field_ident = Some(parsed_choice);
              handled_text = true;
            }
          }
        }
      }
    };
    choice_match_init_tokens.push(quote! {
      let mut #match_ident = false;
      let mut #any_ident = false;
    });
    choice_match_decl_tokens.push(quote! {
      #match_ident = #choice_ty::matches_specific_start_qname(event_name);
      if #match_ident {
        specific_choice_match_count += 1usize;
      } else if #choice_ty::accepts_any_child() {
        #any_ident = true;
        any_choice_match_count += 1usize;
      }
    });
    if field.repeated {
      choice_unique_parse_tokens_borrowed.push(quote! {
        if #match_ident && specific_choice_match_count == 1usize {
          match #choice_ty::#deserialize_borrowed_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident.push(parsed_choice);
              continue;
            }
            Err(crate::common::SdkError::MissingField { .. }) => continue,
            Err(err) => return Err(err),
          }
        }
      });
      choice_unique_parse_tokens_io.push(quote! {
        if #match_ident && specific_choice_match_count == 1usize {
          match #choice_ty::#deserialize_io_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident.push(parsed_choice);
              continue;
            }
            Err(crate::common::SdkError::MissingField { .. }) => continue,
            Err(err) => return Err(err),
          }
        }
      });
      choice_unique_visit_parse_tokens_borrowed.push(quote! {
        if #match_ident && specific_choice_match_count == 1usize {
          return match #choice_ty::#deserialize_borrowed_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident.push(parsed_choice);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          };
        }
      });
      choice_unique_visit_parse_tokens_io.push(quote! {
        if #match_ident && specific_choice_match_count == 1usize {
          return match #choice_ty::#deserialize_io_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident.push(parsed_choice);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          };
        }
      });
      choice_any_unique_parse_tokens_borrowed.push(quote! {
        if #any_ident && specific_choice_match_count == 0usize && any_choice_match_count == 1usize {
          match #choice_ty::#deserialize_borrowed_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident.push(parsed_choice);
              continue;
            }
            Err(crate::common::SdkError::MissingField { .. }) => continue,
            Err(err) => return Err(err),
          }
        }
      });
      choice_any_unique_parse_tokens_io.push(quote! {
        if #any_ident && specific_choice_match_count == 0usize && any_choice_match_count == 1usize {
          match #choice_ty::#deserialize_io_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident.push(parsed_choice);
              continue;
            }
            Err(crate::common::SdkError::MissingField { .. }) => continue,
            Err(err) => return Err(err),
          }
        }
      });
      choice_any_unique_visit_parse_tokens_borrowed.push(quote! {
        if #any_ident && specific_choice_match_count == 0usize && any_choice_match_count == 1usize {
          return match #choice_ty::#deserialize_borrowed_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident.push(parsed_choice);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          };
        }
      });
      choice_any_unique_visit_parse_tokens_io.push(quote! {
        if #any_ident && specific_choice_match_count == 0usize && any_choice_match_count == 1usize {
          return match #choice_ty::#deserialize_io_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident.push(parsed_choice);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          };
        }
      });
    } else {
      choice_unique_parse_tokens_borrowed.push(quote! {
        if #match_ident && specific_choice_match_count == 1usize {
          match #choice_ty::#deserialize_borrowed_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident = Some(parsed_choice);
              continue;
            }
            Err(crate::common::SdkError::MissingField { .. }) => continue,
            Err(err) => return Err(err),
          }
        }
      });
      choice_unique_parse_tokens_io.push(quote! {
        if #match_ident && specific_choice_match_count == 1usize {
          match #choice_ty::#deserialize_io_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident = Some(parsed_choice);
              continue;
            }
            Err(crate::common::SdkError::MissingField { .. }) => continue,
            Err(err) => return Err(err),
          }
        }
      });
      choice_unique_visit_parse_tokens_borrowed.push(quote! {
        if #match_ident && specific_choice_match_count == 1usize {
          return match #choice_ty::#deserialize_borrowed_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident = Some(parsed_choice);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          };
        }
      });
      choice_unique_visit_parse_tokens_io.push(quote! {
        if #match_ident && specific_choice_match_count == 1usize {
          return match #choice_ty::#deserialize_io_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident = Some(parsed_choice);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          };
        }
      });
      choice_any_unique_parse_tokens_borrowed.push(quote! {
        if #any_ident && specific_choice_match_count == 0usize && any_choice_match_count == 1usize {
          match #choice_ty::#deserialize_borrowed_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident = Some(parsed_choice);
              continue;
            }
            Err(crate::common::SdkError::MissingField { .. }) => continue,
            Err(err) => return Err(err),
          }
        }
      });
      choice_any_unique_parse_tokens_io.push(quote! {
        if #any_ident && specific_choice_match_count == 0usize && any_choice_match_count == 1usize {
          match #choice_ty::#deserialize_io_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident = Some(parsed_choice);
              continue;
            }
            Err(crate::common::SdkError::MissingField { .. }) => continue,
            Err(err) => return Err(err),
          }
        }
      });
      choice_any_unique_visit_parse_tokens_borrowed.push(quote! {
        if #any_ident && specific_choice_match_count == 0usize && any_choice_match_count == 1usize {
          return match #choice_ty::#deserialize_borrowed_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident = Some(parsed_choice);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          };
        }
      });
      choice_any_unique_visit_parse_tokens_io.push(quote! {
        if #any_ident && specific_choice_match_count == 0usize && any_choice_match_count == 1usize {
          return match #choice_ty::#deserialize_io_inner_ident(xml_reader, Some((e, next_empty))) {
            Ok(parsed_choice) => {
              #field_ident = Some(parsed_choice);
              Ok(true)
            }
            Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
            Err(err) => Err(err),
          };
        }
      });
    }
    if field.repeated {
      choice_decl_tokens.push(quote! { let mut #field_ident = Vec::new(); });
      choice_init_tokens.push(quote! { #field_ident });
      choice_write_tokens.push(quote! {
        for choice in &self.#field_ident {
          choice.write_xml(writer, xmlns_prefix)?;
        }
      });
      choice_validate_tokens.push(quote! {
        for choice in &self.#field_ident {
          crate::validator::SdkValidator::validate(#validate_choice_tokens)?;
        }
      });
      choice_parse_tokens.push(quote! {});
      choice_visit_parse_tokens.push(quote! {});
      choice_text_parse_tokens.push(build_text_block(quote! { &text_value }));
    } else {
      choice_decl_tokens.push(quote! { let mut #field_ident = None; });
      if field.optional {
        choice_init_tokens.push(quote! { #field_ident });
      } else {
        choice_init_tokens.push(quote! {
          #field_ident: #field_ident.ok_or_else(|| crate::common::missing_field(
            stringify!(#ident),
            stringify!(#field_ident),
          ))?
        });
      }
      if field.optional {
        choice_write_tokens.push(quote! {
          if let Some(choice) = &self.#field_ident {
            choice.write_xml(writer, xmlns_prefix)?;
          }
        });
        choice_validate_tokens.push(quote! {
          if let Some(choice) = &self.#field_ident {
            crate::validator::SdkValidator::validate(#validate_choice_tokens)?;
          }
        });
      } else {
        choice_write_tokens.push(quote! {
          self.#field_ident.write_xml(writer, xmlns_prefix)?;
        });
        choice_validate_tokens.push(quote! {
          crate::validator::SdkValidator::validate(#validate_self_tokens)?;
        });
      }
      choice_parse_tokens.push(quote! {});
      choice_visit_parse_tokens.push(quote! {});
      choice_text_parse_tokens.push(build_text_block(quote! { &text_value }));
    }
  }

  let mut any_decl_tokens = Vec::new();
  let mut any_init_tokens = Vec::new();
  let mut any_parse_tokens_borrowed = Vec::new();
  let mut any_parse_tokens_io = Vec::new();
  let mut any_visit_parse_tokens_borrowed = Vec::new();
  let mut any_visit_parse_tokens_io = Vec::new();
  for field in &any_fields {
    let field_ident = &field.ident;

    if field.repeated {
      any_decl_tokens.push(quote! { let mut #field_ident = Vec::new(); });
      any_init_tokens.push(quote! { #field_ident });
      any_parse_tokens_borrowed.push(quote! {
        if !matched {
          let xml = crate::common::read_outer_xml_borrowed(xml_reader, e, next_empty)?;
          #field_ident.push(xml);
          continue;
        }
      });
      any_parse_tokens_io.push(quote! {
        if !matched {
          let xml = crate::common::read_outer_xml_io(xml_reader, e, next_empty)?;
          #field_ident.push(xml);
          continue;
        }
      });
      any_visit_parse_tokens_borrowed.push(quote! {
        if !matched {
          let xml = crate::common::read_outer_xml_borrowed(xml_reader, e, next_empty)?;
          #field_ident.push(xml);
          return Ok(true);
        }
      });
      any_visit_parse_tokens_io.push(quote! {
        if !matched {
          let xml = crate::common::read_outer_xml_io(xml_reader, e, next_empty)?;
          #field_ident.push(xml);
          return Ok(true);
        }
      });
    } else {
      any_decl_tokens.push(quote! { let mut #field_ident = None; });
      if field.optional {
        any_init_tokens.push(quote! { #field_ident });
      } else {
        any_init_tokens.push(quote! {
          #field_ident: #field_ident.ok_or_else(|| crate::common::missing_field(
            stringify!(#ident),
            stringify!(#field_ident),
          ))?
        });
      }
      any_parse_tokens_borrowed.push(quote! {
        if !matched {
          let xml = crate::common::read_outer_xml_borrowed(xml_reader, e, next_empty)?;
          #field_ident = Some(xml);
          continue;
        }
      });
      any_parse_tokens_io.push(quote! {
        if !matched {
          let xml = crate::common::read_outer_xml_io(xml_reader, e, next_empty)?;
          #field_ident = Some(xml);
          continue;
        }
      });
      any_visit_parse_tokens_borrowed.push(quote! {
        if !matched {
          let xml = crate::common::read_outer_xml_borrowed(xml_reader, e, next_empty)?;
          #field_ident = Some(xml);
          return Ok(true);
        }
      });
      any_visit_parse_tokens_io.push(quote! {
        if !matched {
          let xml = crate::common::read_outer_xml_io(xml_reader, e, next_empty)?;
          #field_ident = Some(xml);
          return Ok(true);
        }
      });
    }
  }

  let has_child_dispatch = !child_fields.is_empty() || !text_child_fields.is_empty();
  let has_choice_dispatch = !choice_fields.is_empty();
  let has_any_dispatch = !any_fields.is_empty();
  let visit_foreign_child_tokens_borrowed =
    if !has_child_dispatch && !has_choice_dispatch && !has_any_dispatch {
      quote! {
        let mut visit_foreign_child = |
          _xml_reader: &mut crate::common::SliceReader<'de>,
          _e: quick_xml::events::BytesStart<'de>,
          _next_empty: bool,
        | -> Result<bool, crate::common::SdkError> {
          Ok(false)
        };
      }
    } else if !has_choice_dispatch && !has_any_dispatch {
      quote! {
        let mut visit_foreign_child = |
          xml_reader: &mut crate::common::SliceReader<'de>,
          e: quick_xml::events::BytesStart<'de>,
          next_empty: bool,
        | -> Result<bool, crate::common::SdkError> {
          let event_name = e.name();
          let event_name = event_name.as_ref();
          let direct_child_case = match event_name {
            #( #direct_child_case_arms )*
            _ => 0usize,
          };
          match direct_child_case {
            #( #direct_child_visit_dispatch_tokens_borrowed )*
            _ => {}
          }
          match event_name {
            #( #child_visit_parse_tokens )*
            _ => Ok(false),
          }
        };
      }
    } else {
      quote! {
        let mut visit_foreign_child = |
          xml_reader: &mut crate::common::SliceReader<'de>,
          e: quick_xml::events::BytesStart<'de>,
          next_empty: bool,
        | -> Result<bool, crate::common::SdkError> {
          let event_name = e.name();
          let event_name = event_name.as_ref();
          let direct_child_case = match event_name {
            #( #direct_child_case_arms )*
            _ => 0usize,
          };
          match direct_child_case {
            #( #direct_child_visit_dispatch_tokens_borrowed )*
            _ => {}
          }
          #( #choice_match_init_tokens )*
          {
            let mut specific_choice_match_count = 0usize;
            let mut any_choice_match_count = 0usize;
            #( #choice_match_decl_tokens )*
            #( #choice_unique_visit_parse_tokens_borrowed )*
            #( #choice_any_unique_visit_parse_tokens_borrowed )*
            if specific_choice_match_count > 1usize || any_choice_match_count > 1usize {
              return Err(crate::common::unexpected_tag(
                stringify!(#ident),
                "known child",
                event_name,
              ));
            }
          }
          let matched: bool = match event_name {
            #( #child_visit_parse_tokens )*
            _ => {
              let mut matched = false;
              #( #choice_visit_parse_tokens )*
              #( #any_visit_parse_tokens_borrowed )*
              Ok::<bool, crate::common::SdkError>(matched)
            }
          }?;
          Ok(matched)
        };
      }
    };
  let visit_foreign_child_tokens_io =
    if !has_child_dispatch && !has_choice_dispatch && !has_any_dispatch {
      quote! {
        let mut visit_foreign_child = |
          _xml_reader: &mut crate::common::IoReader<R>,
          _e: quick_xml::events::BytesStart<'static>,
          _next_empty: bool,
        | -> Result<bool, crate::common::SdkError> {
          Ok(false)
        };
      }
    } else if !has_choice_dispatch && !has_any_dispatch {
      quote! {
        let mut visit_foreign_child = |
          xml_reader: &mut crate::common::IoReader<R>,
          e: quick_xml::events::BytesStart<'static>,
          next_empty: bool,
        | -> Result<bool, crate::common::SdkError> {
          let event_name = e.name();
          let event_name = event_name.as_ref();
          let direct_child_case = match event_name {
            #( #direct_child_case_arms )*
            _ => 0usize,
          };
          match direct_child_case {
            #( #direct_child_visit_dispatch_tokens_io )*
            _ => {}
          }
          match event_name {
            #( #child_visit_parse_tokens )*
            _ => Ok(false),
          }
        };
      }
    } else {
      quote! {
        let mut visit_foreign_child = |
          xml_reader: &mut crate::common::IoReader<R>,
          e: quick_xml::events::BytesStart<'static>,
          next_empty: bool,
        | -> Result<bool, crate::common::SdkError> {
          let event_name = e.name();
          let event_name = event_name.as_ref();
          let direct_child_case = match event_name {
            #( #direct_child_case_arms )*
            _ => 0usize,
          };
          match direct_child_case {
            #( #direct_child_visit_dispatch_tokens_io )*
            _ => {}
          }
          #( #choice_match_init_tokens )*
          {
            let mut specific_choice_match_count = 0usize;
            let mut any_choice_match_count = 0usize;
            #( #choice_match_decl_tokens )*
            #( #choice_unique_visit_parse_tokens_io )*
            #( #choice_any_unique_visit_parse_tokens_io )*
            if specific_choice_match_count > 1usize || any_choice_match_count > 1usize {
              return Err(crate::common::unexpected_tag(
                stringify!(#ident),
                "known child",
                event_name,
              ));
            }
          }
          let matched: bool = match event_name {
            #( #child_visit_parse_tokens )*
            _ => {
              let mut matched = false;
              #( #choice_visit_parse_tokens )*
              #( #any_visit_parse_tokens_io )*
              Ok::<bool, crate::common::SdkError>(matched)
            }
          }?;
          Ok(matched)
        };
      }
    };
  let child_choice_dispatch_tokens_borrowed = if !has_choice_dispatch && !has_any_dispatch {
    quote! {
      let direct_child_case = match event_name {
        #( #direct_child_case_arms )*
        _ => 0usize,
      };
      match direct_child_case {
        #( #direct_child_dispatch_tokens_borrowed )*
        _ => {}
      }
      let matched = match event_name {
        #( #child_parse_tokens )*
        _ => false,
      };
      if matched {
        continue;
      }
    }
  } else {
    quote! {
      let direct_child_case = match event_name {
        #( #direct_child_case_arms )*
        _ => 0usize,
      };
      match direct_child_case {
        #( #direct_child_dispatch_tokens_borrowed )*
        _ => {}
      }
      #( #choice_match_init_tokens )*
      {
        let mut specific_choice_match_count = 0usize;
        let mut any_choice_match_count = 0usize;
        #( #choice_match_decl_tokens )*
        #( #choice_unique_parse_tokens_borrowed )*
        #( #choice_any_unique_parse_tokens_borrowed )*
        if specific_choice_match_count > 1usize || any_choice_match_count > 1usize {
          return Err(crate::common::unexpected_tag(
            stringify!(#ident),
            "known child",
            event_name,
          ));
        }
      }
      let matched = match event_name {
        #( #child_parse_tokens )*
        _ => {
          let mut matched = false;
          #( #choice_parse_tokens )*
          #( #any_parse_tokens_borrowed )*
          matched
        }
      };
      if matched {
        continue;
      }
    }
  };
  let child_choice_dispatch_tokens_io = if !has_choice_dispatch && !has_any_dispatch {
    quote! {
      let direct_child_case = match event_name {
        #( #direct_child_case_arms )*
        _ => 0usize,
      };
      match direct_child_case {
        #( #direct_child_dispatch_tokens_io )*
        _ => {}
      }
      let matched = match event_name {
        #( #child_parse_tokens )*
        _ => false,
      };
      if matched {
        continue;
      }
    }
  } else {
    quote! {
      let direct_child_case = match event_name {
        #( #direct_child_case_arms )*
        _ => 0usize,
      };
      match direct_child_case {
        #( #direct_child_dispatch_tokens_io )*
        _ => {}
      }
      #( #choice_match_init_tokens )*
      {
        let mut specific_choice_match_count = 0usize;
        let mut any_choice_match_count = 0usize;
        #( #choice_match_decl_tokens )*
        #( #choice_unique_parse_tokens_io )*
        #( #choice_any_unique_parse_tokens_io )*
        if specific_choice_match_count > 1usize || any_choice_match_count > 1usize {
          return Err(crate::common::unexpected_tag(
            stringify!(#ident),
            "known child",
            event_name,
          ));
        }
      }
      let matched = match event_name {
        #( #child_parse_tokens )*
        _ => {
          let mut matched = false;
          #( #choice_parse_tokens )*
          #( #any_parse_tokens_io )*
          matched
        }
      };
      if matched {
        continue;
      }
    }
  };

  let text_decl_tokens = if let Some(text_field) = &text_field {
    let field_ident = &text_field.ident;
    let _ = text_field.optional;
    quote! { let mut #field_ident = None; }
  } else {
    quote! {}
  };
  let text_read_tokens = if let Some(text_field) = &text_field {
    let field_ident = &text_field.ident;
    quote! {
      quick_xml::events::Event::Text(t) => {
        crate::common::push_xml_text(&mut #field_ident, t)?;
      }
      quick_xml::events::Event::GeneralRef(t) => {
        crate::common::push_xml_general_ref(
          &mut #field_ident,
          t,
          stringify!(#ident),
          stringify!(#field_ident),
        )?;
      }
    }
  } else {
    if choice_fields.is_empty() {
      quote! {}
    } else {
      quote! {
        quick_xml::events::Event::Text(t) => {
          let mut text_value = None;
          crate::common::push_xml_text(&mut text_value, t)?;
          if let Some(text_value) = text_value {
            let mut handled_text = false;
            #( #choice_text_parse_tokens )*
            if !handled_text && !text_value.trim().is_empty() {
              return Err(crate::common::unexpected_tag(
                stringify!(#ident),
                "known child",
                b"#text",
              ));
            }
          }
        }
        quick_xml::events::Event::GeneralRef(t) => {
          let mut text_value = None;
          crate::common::push_xml_general_ref(
            &mut text_value,
            t,
            stringify!(#ident),
            "#text",
          )?;
          if let Some(text_value) = text_value {
            let mut handled_text = false;
            #( #choice_text_parse_tokens )*
            if !handled_text && !text_value.trim().is_empty() {
              return Err(crate::common::unexpected_tag(
                stringify!(#ident),
                "known child",
                b"#text",
              ));
            }
          }
        }
      }
    }
  };
  let mut ordered_write_tokens = Vec::new();
  for (field_ident, field_ty, field_kind) in &ordered_field_specs {
    match field_kind {
      SdkTypeFieldKind::Child { .. } => {
        let repeated = contains_vec_type(field_ty);
        let optional = is_option_type(field_ty);
        if repeated {
          ordered_write_tokens.push(quote! {
            for child in &self.#field_ident {
              child.write_xml(writer, xmlns_prefix)?;
            }
          });
        } else if optional {
          ordered_write_tokens.push(quote! {
            if let Some(child) = &self.#field_ident {
              child.write_xml(writer, xmlns_prefix)?;
            }
          });
        } else {
          ordered_write_tokens.push(quote! {
            self.#field_ident.write_xml(writer, xmlns_prefix)?;
          });
        }
      }
      SdkTypeFieldKind::TextChild { qname } => {
        ordered_write_tokens.push(build_text_child_write_tokens(
          field_ident,
          qname,
          field_ty,
          contains_vec_type(field_ty),
          is_option_type(field_ty),
        ));
      }
      SdkTypeFieldKind::Choice => {
        let repeated = contains_vec_type(field_ty);
        let optional = is_option_type(field_ty);
        if repeated {
          ordered_write_tokens.push(quote! {
            for choice in &self.#field_ident {
              choice.write_xml(writer, xmlns_prefix)?;
            }
          });
        } else if optional {
          ordered_write_tokens.push(quote! {
            if let Some(choice) = &self.#field_ident {
              choice.write_xml(writer, xmlns_prefix)?;
            }
          });
        } else {
          ordered_write_tokens.push(quote! {
            self.#field_ident.write_xml(writer, xmlns_prefix)?;
          });
        }
      }
      SdkTypeFieldKind::Any => {
        let repeated = contains_vec_type(field_ty);
        let optional = is_option_type(field_ty);
        if repeated {
          ordered_write_tokens.push(quote! {
            for value in &self.#field_ident {
              writer.write_all(value.as_bytes())?;
            }
          });
        } else if optional {
          ordered_write_tokens.push(quote! {
            if let Some(value) = &self.#field_ident {
              writer.write_all(value.as_bytes())?;
            }
          });
        } else {
          ordered_write_tokens.push(quote! {
            writer.write_all(self.#field_ident.as_bytes())?;
          });
        }
      }
      SdkTypeFieldKind::Text => {
        let inner_ty = unwrap_wrapped_type(field_ty);
        let optional_text_write_tokens = if is_xml_schema_float_type(&inner_ty) {
          write_xml_schema_float_tokens(quote! { value }, &inner_ty)
        } else if is_bool_type(&inner_ty) {
          write_bool_tokens(quote! { value }, &inner_ty)
        } else if is_string_like_type(&inner_ty) {
          quote! {
            crate::common::write_escaped_str(writer, value.as_ref())?;
          }
        } else {
          quote! {
            crate::common::write_escaped_text(writer, value)?;
          }
        };
        let required_text_write_tokens = if is_xml_schema_float_type(&inner_ty) {
          write_xml_schema_float_tokens(quote! { &self.#field_ident }, &inner_ty)
        } else if is_bool_type(&inner_ty) {
          write_bool_tokens(quote! { &self.#field_ident }, &inner_ty)
        } else if is_string_like_type(&inner_ty) {
          quote! {
            crate::common::write_escaped_str(writer, self.#field_ident.as_ref())?;
          }
        } else {
          quote! {
            crate::common::write_escaped_text(writer, &self.#field_ident)?;
          }
        };
        if is_option_type(field_ty) {
          ordered_write_tokens.push(quote! {
            if let Some(value) = &self.#field_ident {
              #optional_text_write_tokens
            }
          });
        } else {
          ordered_write_tokens.push(quote! {
            #required_text_write_tokens
          });
        }
      }
      SdkTypeFieldKind::Attr { .. } => {}
    }
  }
  let text_finish_tokens = if let Some(text_field) = &text_field {
    let field_ident = &text_field.ident;
    let inner_ty = unwrap_wrapped_type(&text_field.ty);
    let field_name_lit = LitStr::new(&field_ident.to_string(), Span::call_site());
    if is_string_like_type(&inner_ty) {
      quote! {
        #field_ident,
      }
    } else {
      let parse_value_tokens = if is_bool_type(&inner_ty) {
        parse_bool_tokens(
          quote! { &value },
          &inner_ty,
          quote! { stringify!(#ident) },
          quote! { #field_name_lit },
        )
      } else {
        quote! {
          crate::common::parse_value::<#inner_ty>(&value, stringify!(#ident), #field_name_lit)?
        }
      };
      if text_field.optional {
        quote! {
          #field_ident: match #field_ident {
            Some(value) => Some(#parse_value_tokens),
            None => None,
          },
        }
      } else {
        quote! {
          #field_ident: {
            let value = #field_ident.ok_or_else(|| crate::common::missing_field(
              stringify!(#ident),
              #field_name_lit,
            ))?;
            #parse_value_tokens
          },
        }
      }
    }
  } else {
    quote! {}
  };

  let special_namespace_write_tokens = if has_xmlns_fields {
    quote! {
      if let Some(xmlns) = &self.xmlns {
        crate::common::write_xmlns_attr(writer, None, xmlns)?;
      }
      {
        let mut xmlns_entries: Vec<_> = self.xmlns_map.iter().collect();
        xmlns_entries.sort_unstable_by(|(left_key, _), (right_key, _)| left_key.cmp(right_key));
        for (k, v) in xmlns_entries {
          crate::common::write_xmlns_attr(writer, Some(k), v)?;
        }
      }
    }
  } else {
    quote! {}
  };
  let mc_ignorable_write_tokens = if has_mc_ignorable_field {
    quote! {
      if let Some(mc_ignorable) = &self.mc_ignorable {
        crate::common::write_attr_value_str(writer, "mc:Ignorable", mc_ignorable.as_ref())?;
      }
    }
  } else {
    quote! {}
  };
  let special_namespace_init_tokens = if has_xmlns_fields {
    quote! {
      xmlns,
      xmlns_map,
    }
  } else {
    quote! {}
  };
  let mc_ignorable_init_tokens = if has_mc_ignorable_field {
    quote! {
      mc_ignorable,
    }
  } else {
    quote! {}
  };
  let xml_header_init_tokens = if has_xml_header_field {
    quote! {
      xml_header: xml_header_state,
    }
  } else {
    quote! {}
  };
  let xml_header_tokens = if has_xml_header_field {
    quote! {
      match self.xml_header {
        crate::common::XmlHeaderType::Standalone => {
          writer.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\r\n")?;
        }
        crate::common::XmlHeaderType::Plain => {
          writer.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\r\n")?;
        }
        crate::common::XmlHeaderType::None => {}
      }
    }
  } else {
    quote! {}
  };
  let to_xml_prefix_tokens = if has_xmlns_fields {
    quote! {
      if self.xmlns.is_some() {
        #tag_prefix
      } else {
        ""
      }
    }
  } else {
    quote! { "" }
  };

  let has_body = !child_fields.is_empty()
    || !text_child_fields.is_empty()
    || !choice_fields.is_empty()
    || text_field.is_some();
  Ok(quote! {
    impl crate::sdk::SdkType for #ident {}
    #[cfg(feature = "validators")]
    impl crate::validator::SdkValidator for #ident {
      fn validate(&self) -> Result<(), crate::common::SdkError> {
        #( #attr_validate_tokens )*
        #( #child_validate_tokens )*
        #( #choice_validate_tokens )*
        Ok(())
      }
    }

    impl std::str::FromStr for #ident {
      type Err = crate::common::SdkError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut xml_reader = crate::common::from_str_inner(s)?;
        Self::#deserialize_borrowed_inner_ident(&mut xml_reader, None)
      }
    }

    impl #ident {
      pub fn from_bytes(bytes: &[u8]) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_bytes_inner(bytes)?;
        Self::#deserialize_borrowed_inner_ident(&mut xml_reader, None)
      }

      pub fn from_str(s: &str) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_str_inner(s)?;
        Self::#deserialize_borrowed_inner_ident(&mut xml_reader, None)
      }

      pub fn from_reader<R: std::io::BufRead>(
        reader: R,
      ) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_reader_inner(reader)?;
        Self::#deserialize_io_inner_ident(&mut xml_reader, None)
      }

      pub(crate) fn #deserialize_borrowed_inner_ident<'de>(
        xml_reader: &mut crate::common::SliceReader<'de>,
        xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
      ) -> Result<Self, crate::common::SdkError> {
        #xml_header_decl_tokens
        let (e, empty_tag) = if let Some((e, empty_tag)) = xml_event {
          (e, empty_tag)
        } else {
          loop {
            match xml_reader.next()? {
              quick_xml::events::Event::Decl(e) => {
                if #has_xml_header_field {
                  xml_header_state = if matches!(
                    e.standalone(),
                    Some(Ok(value)) if value.as_ref().eq_ignore_ascii_case(b"yes")
                  ) {
                    crate::common::XmlHeaderType::Standalone
                  } else {
                    crate::common::XmlHeaderType::Plain
                  };
                }
              }
              quick_xml::events::Event::Start(e) => break (e.into_owned(), false),
              quick_xml::events::Event::Empty(e) => break (e.into_owned(), true),
              quick_xml::events::Event::Eof => {
                return Err(crate::common::unexpected_eof(stringify!(#ident)));
              }
              _ => continue,
            }
          }
        };
        match e.name().as_ref() {
          #tag_qname_lit | #local_name_lit => {}
          found => {
            return Err(crate::common::unexpected_tag(
              stringify!(#ident),
              stringify!(#ident),
              found,
            ));
          }
        }

        #( #attr_decl_tokens )*
        #namespace_attr_parse_tokens
        #( #child_decl_tokens )*
        #( #choice_decl_tokens )*
        #( #any_decl_tokens )*
        #text_decl_tokens

        if !empty_tag {
          loop {
            match xml_reader.next()? {
              quick_xml::events::Event::Decl(e) => {
                if #has_xml_header_field {
                  xml_header_state = if matches!(
                    e.standalone(),
                    Some(Ok(value)) if value.as_ref().eq_ignore_ascii_case(b"yes")
                  ) {
                    crate::common::XmlHeaderType::Standalone
                  } else {
                    crate::common::XmlHeaderType::Plain
                  };
                }
              }
              quick_xml::events::Event::Start(e) => {
                let e = e.into_owned();
                let next_empty = false;
                let event_name = e.name();
                let event_name = event_name.as_ref();
                #child_choice_dispatch_tokens_borrowed
                match event_name {
                  b"mc:AlternateContent" | b"AlternateContent" => {
                    #visit_foreign_child_tokens_borrowed
                    crate::common::process_markup_compatibility_children_borrowed(
                      xml_reader,
                      next_empty,
                      &mut visit_foreign_child,
                    )?;
                  }
                  _ => {
                    if crate::common::is_foreign_prefixed_child(
                      event_name,
                      #tag_prefix,
                    ) {
                      #visit_foreign_child_tokens_borrowed
                      crate::common::process_foreign_element_children_borrowed(
                        xml_reader,
                        next_empty,
                        &mut visit_foreign_child,
                      )?;
                    } else {
                      Err(crate::common::unexpected_tag(
                        stringify!(#ident),
                        "known child",
                        event_name,
                      ))?;
                    }
                  }
                }
              }
              quick_xml::events::Event::Empty(e) => {
                let e = e.into_owned();
                let next_empty = true;
                let event_name = e.name();
                let event_name = event_name.as_ref();
                #child_choice_dispatch_tokens_borrowed
                match event_name {
                  b"mc:AlternateContent" | b"AlternateContent" => {
                    #visit_foreign_child_tokens_borrowed
                    crate::common::process_markup_compatibility_children_borrowed(
                      xml_reader,
                      next_empty,
                      &mut visit_foreign_child,
                    )?;
                  }
                  _ => {
                    if crate::common::is_foreign_prefixed_child(
                      event_name,
                      #tag_prefix,
                    ) {
                      #visit_foreign_child_tokens_borrowed
                      crate::common::process_foreign_element_children_borrowed(
                        xml_reader,
                        next_empty,
                        &mut visit_foreign_child,
                      )?;
                    } else {
                      Err(crate::common::unexpected_tag(
                        stringify!(#ident),
                        "known child",
                        event_name,
                      ))?;
                    }
                  }
                }
              }
              #text_read_tokens
              quick_xml::events::Event::End(e) => {
                if e.name().as_ref() == #tag_qname_lit || e.name().as_ref() == #local_name_lit {
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

        Ok(Self {
          #( #attr_init_tokens, )*
          #( #child_init_tokens, )*
          #( #choice_init_tokens, )*
          #( #any_init_tokens, )*
          #text_finish_tokens
          #( #attr_finish_tokens, )*
          #special_namespace_init_tokens
          #xml_header_init_tokens
          #mc_ignorable_init_tokens
        })
      }

      pub(crate) fn #deserialize_io_inner_ident<R: std::io::BufRead>(
        xml_reader: &mut crate::common::IoReader<R>,
        xml_event: Option<(quick_xml::events::BytesStart<'static>, bool)>,
      ) -> Result<Self, crate::common::SdkError> {
        #xml_header_decl_tokens
        let (e, empty_tag) = if let Some((e, empty_tag)) = xml_event {
          (e, empty_tag)
        } else {
          loop {
            let next_event = match xml_reader.next_tag_event()? {
              crate::common::IoTagEvent::Decl(standalone) => {
                if #has_xml_header_field {
                  xml_header_state = if standalone {
                    crate::common::XmlHeaderType::Standalone
                  } else {
                    crate::common::XmlHeaderType::Plain
                  };
                }
                None
              }
              crate::common::IoTagEvent::Start(e, empty_tag) => Some((e, empty_tag)),
              crate::common::IoTagEvent::Eof => {
                return Err(crate::common::unexpected_eof(stringify!(#ident)));
              }
              crate::common::IoTagEvent::End(_) | crate::common::IoTagEvent::Other => None,
            };
            if let Some((e, empty_tag)) = next_event {
              break (e, empty_tag);
            }
          }
        };
        match e.name().as_ref() {
          #tag_qname_lit | #local_name_lit => {}
          found => {
            return Err(crate::common::unexpected_tag(
              stringify!(#ident),
              stringify!(#ident),
              found,
            ));
          }
        }

        #( #attr_decl_tokens )*
        #namespace_attr_parse_tokens
        #( #child_decl_tokens )*
        #( #choice_decl_tokens )*
        #( #any_decl_tokens )*
        #text_decl_tokens

        if !empty_tag {
          loop {
            match xml_reader.next()? {
              quick_xml::events::Event::Decl(e) => {
                if #has_xml_header_field {
                  xml_header_state = if matches!(
                    e.standalone(),
                    Some(Ok(value)) if value.as_ref().eq_ignore_ascii_case(b"yes")
                  ) {
                    crate::common::XmlHeaderType::Standalone
                  } else {
                    crate::common::XmlHeaderType::Plain
                  };
                }
              }
              quick_xml::events::Event::Start(e) => {
                let e = e.into_owned();
                let next_empty = false;
                let event_name = e.name();
                let event_name = event_name.as_ref();
                #child_choice_dispatch_tokens_io
                match event_name {
                  b"mc:AlternateContent" | b"AlternateContent" => {
                    #visit_foreign_child_tokens_io
                    crate::common::process_markup_compatibility_children_io(
                      xml_reader,
                      next_empty,
                      &mut visit_foreign_child,
                    )?;
                  }
                  _ => {
                    if crate::common::is_foreign_prefixed_child(
                      event_name,
                      #tag_prefix,
                    ) {
                      #visit_foreign_child_tokens_io
                      crate::common::process_foreign_element_children_io(
                        xml_reader,
                        next_empty,
                        &mut visit_foreign_child,
                      )?;
                    } else {
                      Err(crate::common::unexpected_tag(
                        stringify!(#ident),
                        "known child",
                        event_name,
                      ))?;
                    }
                  }
                }
              }
              quick_xml::events::Event::Empty(e) => {
                let e = e.into_owned();
                let next_empty = true;
                let event_name = e.name();
                let event_name = event_name.as_ref();
                #child_choice_dispatch_tokens_io
                match event_name {
                  b"mc:AlternateContent" | b"AlternateContent" => {
                    #visit_foreign_child_tokens_io
                    crate::common::process_markup_compatibility_children_io(
                      xml_reader,
                      next_empty,
                      &mut visit_foreign_child,
                    )?;
                  }
                  _ => {
                    if crate::common::is_foreign_prefixed_child(
                      event_name,
                      #tag_prefix,
                    ) {
                      #visit_foreign_child_tokens_io
                      crate::common::process_foreign_element_children_io(
                        xml_reader,
                        next_empty,
                        &mut visit_foreign_child,
                      )?;
                    } else {
                      Err(crate::common::unexpected_tag(
                        stringify!(#ident),
                        "known child",
                        event_name,
                      ))?;
                    }
                  }
                }
              }
              #text_read_tokens
              quick_xml::events::Event::End(e) => {
                if e.name().as_ref() == #tag_qname_lit || e.name().as_ref() == #local_name_lit {
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

        Ok(Self {
          #( #attr_init_tokens, )*
          #( #child_init_tokens, )*
          #( #choice_init_tokens, )*
          #( #any_init_tokens, )*
          #text_finish_tokens
          #( #attr_finish_tokens, )*
          #special_namespace_init_tokens
          #xml_header_init_tokens
          #mc_ignorable_init_tokens
        })
      }

      pub fn write_xml_to_writer<W: std::io::Write>(
        &self,
        writer: &mut W,
      ) -> Result<(), std::io::Error> {
        self.write_xml(writer, #to_xml_prefix_tokens)
      }

      pub fn to_xml_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut writer = Vec::with_capacity(32);
        self.write_xml_to_writer(&mut writer)?;
        Ok(writer)
      }

      pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
        let bytes = self.to_xml_bytes().map_err(|_| std::fmt::Error)?;
        // SAFETY: XML serialization only writes UTF-8 data via byte literals and UTF-8 strings.
        Ok(unsafe { String::from_utf8_unchecked(bytes) })
      }

      pub(crate) fn write_xml<W: std::io::Write>(
        &self,
        writer: &mut W,
        xmlns_prefix: &str,
      ) -> Result<(), std::io::Error> {
        #xml_header_tokens
        crate::common::write_start_tag_open(writer, xmlns_prefix, #tag_prefix, #local_name)?;
        #special_namespace_write_tokens
        #mc_ignorable_write_tokens
        #( #attr_write_tokens )*
        if #has_body {
          writer.write_all(b">")?;
          #( #ordered_write_tokens )*
          crate::common::write_end_tag(writer, xmlns_prefix, #tag_prefix, #local_name)?;
        } else {
          writer.write_all(b"/>")?;
        }
        Ok(())
      }
    }

    impl ::std::fmt::Display for #ident {
      fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{}", self.to_xml()?)
      }
    }
  })
}
