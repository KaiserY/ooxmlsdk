use super::*;

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
        Self::deserialize_inner(&mut xml_reader, None)
      }
    }

    impl #ident {
      pub fn from_str(s: &str) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_str_inner(s)?;
        Self::deserialize_inner(&mut xml_reader, None)
      }

      pub fn from_reader<R: std::io::BufRead>(
        reader: R,
      ) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_reader_inner(reader)?;
        Self::deserialize_inner(&mut xml_reader, None)
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
  repeated: bool,
  optional: bool,
) -> proc_macro2::TokenStream {
  let QNameInfo {
    tag_prefix,
    local_name,
  } = parse_qname_info(qname);
  let write_value_tokens = |value_expr: proc_macro2::TokenStream| {
    quote! {
      crate::common::write_start_tag_open(writer, xmlns_prefix, #tag_prefix, #local_name)?;
      writer.write_char('>')?;
      crate::common::write_escaped_text(writer, #value_expr)?;
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

  let mut child_decl_tokens = Vec::new();
  let mut child_dispatch_tokens = Vec::new();
  let mut child_write_tokens = Vec::new();
  let mut child_init_tokens = Vec::new();
  let mut child_validate_tokens = Vec::new();

  let xml_reader_ident = Ident::new("xml_reader", Span::call_site());

  for field in &fields.named {
    let field_ident = field
      .ident
      .as_ref()
      .ok_or_else(|| syn::Error::new_spanned(field, "SdkType requires named fields"))?;
    match parse_sdk_type_field_kind(&field.attrs)? {
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
          child_dispatch_tokens.push(quote! {
            if matches!(e.name().as_ref(), #match_target) {
              match #child_ty::deserialize_inner(#xml_reader_ident, Some((e, next_empty))) {
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
          child_dispatch_tokens.push(quote! {
            if matches!(e.name().as_ref(), #match_target) {
              match #child_ty::deserialize_inner(#xml_reader_ident, Some((e, next_empty))) {
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
          contains_vec_type(&field.ty),
          is_option_type(&field.ty),
        ));
        let match_target = text_child_match_target(&qname);
        let parse_arm = build_text_child_parse_arm(
          ident,
          field_ident,
          &qname,
          &field.ty,
          contains_vec_type(&field.ty),
          false,
        );
        child_dispatch_tokens.push(quote! {
          if matches!(e.name().as_ref(), #match_target) {
            match e.name().as_ref() {
              #parse_arm
              _ => false,
            };
            continue;
          }
        });
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
      pub(crate) fn deserialize_inner<'de, R: crate::common::XmlReader<'de>>(
        xml_reader: &mut R,
        xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
      ) -> Result<Self, crate::common::SdkError> {
        let mut pending_event = xml_event;

        #( #child_decl_tokens )*

        loop {
          if let Some((e, next_empty)) = pending_event.take() {
            #( #child_dispatch_tokens )*
            xml_reader.unread(if next_empty {
              quick_xml::events::Event::Empty(e)
            } else {
              quick_xml::events::Event::Start(e)
            })?;
            break;
          }

          match xml_reader.next()? {
            quick_xml::events::Event::Start(e) => {
              pending_event = Some((e, false));
            }
            quick_xml::events::Event::Empty(e) => {
              pending_event = Some((e, true));
            }
            quick_xml::events::Event::End(e) => {
              xml_reader.unread(quick_xml::events::Event::End(e))?;
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

      pub(crate) fn write_xml<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        xmlns_prefix: &str,
      ) -> Result<(), std::fmt::Error> {
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

  let mut child_fields = Vec::new();
  let mut text_child_fields = Vec::new();
  let mut choice_fields = Vec::new();
  for field in &fields.named {
    let field_ident = field
      .ident
      .as_ref()
      .ok_or_else(|| syn::Error::new_spanned(field, "SdkType requires named fields"))?;
    match parse_sdk_type_field_kind(&field.attrs)? {
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
    let build_arm = |reader_ident: &Ident, as_result: bool| {
      let target = if tag_prefix.is_empty() {
        quote! { #local_name_lit }
      } else {
        let tag_qname_lit = LitByteStr::new(
          format!("{tag_prefix}:{local_name}").as_bytes(),
          Span::call_site(),
        );
        quote! { #tag_qname_lit | #local_name_lit }
      };
      if field.repeated {
        if as_result {
          quote! {
              #target => {
                match #child_ty::deserialize_inner(#reader_ident, Some((e.clone(), next_empty))) {
                  Ok(parsed_child) => {
                    #field_ident.push(#parsed_child_expr);
                    Ok(true)
                  },
                Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
                Err(err) => Err(err),
              }
            },
          }
        } else {
          quote! {
              #target => {
                match #child_ty::deserialize_inner(#reader_ident, Some((e.clone(), next_empty))) {
                  Ok(parsed_child) => {
                    #field_ident.push(#parsed_child_expr);
                    true
                  },
                Err(crate::common::SdkError::MissingField { .. }) => true,
                Err(err) => return Err(err),
              }
            },
          }
        }
      } else {
        if as_result {
          quote! {
            #target => {
              match #child_ty::deserialize_inner(#reader_ident, Some((e.clone(), next_empty))) {
                Ok(parsed_child) => {
                  #field_ident = Some(#parsed_child_expr);
                  Ok(true)
                },
                Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
                Err(err) => Err(err),
              }
            },
          }
        } else {
          quote! {
            #target => {
              match #child_ty::deserialize_inner(#reader_ident, Some((e.clone(), next_empty))) {
                Ok(parsed_child) => {
                  #field_ident = Some(#parsed_child_expr);
                  true
                },
                Err(crate::common::SdkError::MissingField { .. }) => true,
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
      child_parse_tokens.push(build_arm(&xml_reader_ident, false));
      child_visit_parse_tokens.push(build_arm(&visitor_reader_ident, true));
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
      child_parse_tokens.push(build_arm(&xml_reader_ident, false));
      child_visit_parse_tokens.push(build_arm(&visitor_reader_ident, true));
    }
  }

  let mut choice_decl_tokens = Vec::new();
  let mut choice_parse_tokens = Vec::new();
  let mut choice_visit_parse_tokens = Vec::new();
  let mut choice_write_tokens = Vec::new();
  let mut choice_init_tokens = Vec::new();
  let mut choice_validate_tokens = Vec::new();
  for field in &choice_fields {
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
    let build_visit_arm = |reader_ident: &Ident| {
      if field.repeated {
        quote! {
          if !matched {
            match #choice_ty::deserialize_inner(#reader_ident, Some((e.clone(), next_empty))) {
              Ok(parsed_choice) => {
                #field_ident.push(parsed_choice);
                matched = true;
              }
              Err(crate::common::SdkError::MissingField { .. }) => {
                matched = true;
              }
              Err(crate::common::SdkError::UnexpectedTag { expected, .. })
                if expected == "choice" => {}
              Err(err) => return Err(err),
            }
          }
        }
      } else {
        quote! {
          if !matched {
            match #choice_ty::deserialize_inner(#reader_ident, Some((e.clone(), next_empty))) {
              Ok(parsed_choice) => {
                #field_ident = Some(parsed_choice);
                matched = true;
              }
              Err(crate::common::SdkError::MissingField { .. }) => {
                matched = true;
              }
              Err(crate::common::SdkError::UnexpectedTag { expected, .. })
                if expected == "choice" => {}
              Err(err) => return Err(err),
            }
          }
        }
      }
    };
    let build_parse_arm = |reader_ident: &Ident| {
      if field.repeated {
        quote! {
          if !matched {
            match #choice_ty::deserialize_inner(#reader_ident, Some((e.clone(), next_empty))) {
              Ok(parsed_choice) => {
                #field_ident.push(parsed_choice);
                matched = true;
              }
              Err(crate::common::SdkError::MissingField { .. }) => {
                matched = true;
              }
              Err(crate::common::SdkError::UnexpectedTag { expected, .. })
                if expected == "choice" => {}
              Err(err) => return Err(err),
            }
          }
        }
      } else {
        quote! {
          if !matched {
            match #choice_ty::deserialize_inner(#reader_ident, Some((e.clone(), next_empty))) {
              Ok(parsed_choice) => {
                #field_ident = Some(parsed_choice);
                matched = true;
              }
              Err(crate::common::SdkError::MissingField { .. }) => {
                matched = true;
              }
              Err(crate::common::SdkError::UnexpectedTag { expected, .. })
                if expected == "choice" => {}
              Err(err) => return Err(err),
            }
          }
        }
      }
    };
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
      choice_parse_tokens.push(build_parse_arm(&xml_reader_ident));
      choice_visit_parse_tokens.push(build_visit_arm(&visitor_reader_ident));
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
      choice_parse_tokens.push(build_parse_arm(&xml_reader_ident));
      choice_visit_parse_tokens.push(build_visit_arm(&visitor_reader_ident));
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
  let visit_foreign_child_tokens = if !has_child_dispatch && choice_fields.is_empty() {
    quote! {
      let mut visit_foreign_child = |
        _xml_reader: &mut R,
        _e: quick_xml::events::BytesStart<'de>,
        _next_empty: bool,
      | -> Result<bool, crate::common::SdkError> {
        Ok(false)
      };
    }
  } else if choice_fields.is_empty() {
    quote! {
      let mut visit_foreign_child = |
        xml_reader: &mut R,
        e: quick_xml::events::BytesStart<'de>,
        next_empty: bool,
      | -> Result<bool, crate::common::SdkError> {
        match e.name().as_ref() {
          #( #child_visit_parse_tokens )*
          _ => Ok(false),
        }
      };
    }
  } else {
    quote! {
      let mut visit_foreign_child = |
        xml_reader: &mut R,
        e: quick_xml::events::BytesStart<'de>,
        next_empty: bool,
      | -> Result<bool, crate::common::SdkError> {
        let matched: bool = match e.name().as_ref() {
          #( #child_visit_parse_tokens )*
          _ => Ok::<bool, crate::common::SdkError>(false),
        }?;
        if matched {
          return Ok(true);
        }
        let mut matched = false;
        #( #choice_visit_parse_tokens )*
        Ok(matched)
      };
    }
  };
  let main_dispatch_tokens = if choice_fields.is_empty() {
    quote! {
      let matched = match e.name().as_ref() {
        #( #child_parse_tokens )*
        _ => false,
      };
      if matched {
        continue;
      }
    }
  } else {
    quote! {
      let matched = match e.name().as_ref() {
        #( #child_parse_tokens )*
        _ => false,
      };
      if matched {
        continue;
      }
      let mut matched = false;
      #( #choice_parse_tokens )*
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
        Self::deserialize_inner(&mut xml_reader, None)
      }
    }

    impl #ident {
      pub fn from_str(s: &str) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_str_inner(s)?;
        Self::deserialize_inner(&mut xml_reader, None)
      }

      pub fn from_reader<R: std::io::BufRead>(
        reader: R,
      ) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_reader_inner(reader)?;
        Self::deserialize_inner(&mut xml_reader, None)
      }

      pub(crate) fn deserialize_inner<'de, R: crate::common::XmlReader<'de>>(
        xml_reader: &mut R,
        xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
      ) -> Result<Self, crate::common::SdkError> {
        let mut pending_event = xml_event;

        #( #child_decl_tokens )*
        #( #choice_decl_tokens )*

        loop {
          let (e, next_empty) = if let Some((e, next_empty)) = pending_event.take() {
            (e, next_empty)
          } else {
            match xml_reader.next()? {
              quick_xml::events::Event::Start(e) => (e, false),
              quick_xml::events::Event::Empty(e) => (e, true),
              quick_xml::events::Event::End(e) => {
                break;
              }
              quick_xml::events::Event::Eof => {
                return Err(crate::common::unexpected_eof(stringify!(#ident)));
              }
              _ => continue,
            }
          };

          {
            #visit_foreign_child_tokens
            #main_dispatch_tokens
            match e.name().as_ref() {
              b"mc:AlternateContent" | b"AlternateContent" => {
                crate::common::process_markup_compatibility_children(
                  xml_reader,
                  next_empty,
                  &mut visit_foreign_child,
                )?;
                continue;
              }
              _ => {
                if crate::common::is_foreign_prefixed_child(e.name().as_ref(), "") {
                  crate::common::process_foreign_element_children(
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
              e.name().as_ref(),
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

  let mut attr_fields = Vec::new();
  let mut child_fields = Vec::new();
  let mut text_child_fields = Vec::new();
  let mut choice_fields = Vec::new();
  let mut any_fields = Vec::new();
  let mut text_field = None;
  let mut xmlns_fields = Vec::new();
  let mut xml_header_field = None;
  let mut mc_ignorable_field = None;

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

    match parse_sdk_type_field_kind(&field.attrs)? {
      Some(SdkTypeFieldKind::Attr { name }) => attr_fields.push(SdkAttrField {
        ident: field_ident.clone(),
        name,
        ty: field.ty.clone(),
        optional: is_option_type(&field.ty),
        validators: parse_sdk_type_field_validators(&field.attrs)?,
      }),
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
      Some(SdkTypeFieldKind::Any) => any_fields.push(SdkAnyField {
        ident: field_ident.clone(),
        optional: is_option_type(&field.ty),
        repeated: contains_vec_type(&field.ty),
      }),
      Some(SdkTypeFieldKind::Text) => {
        text_field = Some(SdkTextField {
          ident: field_ident.clone(),
          ty: field.ty.clone(),
          optional: is_option_type(&field.ty),
        });
      }
      None => {
        return Err(syn::Error::new_spanned(
          field,
          "missing #[sdk(...)] field attribute",
        ));
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
      quote! { crate::common::parse_bool_attr(&attr, xml_reader.decoder(), stringify!(#ident), #name_lit)? }
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
    if field.optional {
      attr_write_tokens.push(quote! {
        if let Some(value) = &self.#field_ident {
          crate::common::write_attr_value(writer, #name_lit, value)?;
        }
      });
    } else {
      attr_write_tokens.push(quote! {
        crate::common::write_attr_value(writer, #name_lit, &self.#field_ident)?;
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
    let build_arm = |reader_ident: &Ident, as_result: bool| {
      let target = if tag_prefix.is_empty() {
        quote! { #local_name_lit }
      } else {
        let tag_qname_lit = LitByteStr::new(
          format!("{tag_prefix}:{local_name}").as_bytes(),
          Span::call_site(),
        );
        quote! { #tag_qname_lit | #local_name_lit }
      };
      if field.repeated {
        if as_result {
          quote! {
            #target => {
              match #child_ty::deserialize_inner(#reader_ident, Some((e.clone(), next_empty))) {
                Ok(parsed_child) => {
                  #field_ident.push(#parsed_child_expr);
                  Ok(true)
                },
                Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
                Err(err) => Err(err),
              }
            },
          }
        } else {
          quote! {
            #target => {
              match #child_ty::deserialize_inner(#reader_ident, Some((e.clone(), next_empty))) {
                Ok(parsed_child) => {
                  #field_ident.push(#parsed_child_expr);
                  true
                },
                Err(crate::common::SdkError::MissingField { .. }) => true,
                Err(err) => return Err(err),
              }
            },
          }
        }
      } else {
        if as_result {
          quote! {
            #target => {
              match #child_ty::deserialize_inner(#reader_ident, Some((e.clone(), next_empty))) {
                Ok(parsed_child) => {
                  #field_ident = Some(#parsed_child_expr);
                  Ok(true)
                },
                Err(crate::common::SdkError::MissingField { .. }) => Ok(true),
                Err(err) => Err(err),
              }
            },
          }
        } else {
          quote! {
            #target => {
              match #child_ty::deserialize_inner(#reader_ident, Some((e.clone(), next_empty))) {
                Ok(parsed_child) => {
                  #field_ident = Some(#parsed_child_expr);
                  true
                },
                Err(crate::common::SdkError::MissingField { .. }) => true,
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
      child_parse_tokens.push(build_arm(&xml_reader_ident, false));
      child_visit_parse_tokens.push(build_arm(&visitor_reader_ident, true));
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
      child_parse_tokens.push(build_arm(&xml_reader_ident, false));
      child_visit_parse_tokens.push(build_arm(&visitor_reader_ident, true));
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
  let mut choice_parse_tokens = Vec::new();
  let mut choice_visit_parse_tokens = Vec::new();
  let mut choice_write_tokens = Vec::new();
  let mut choice_init_tokens = Vec::new();
  let mut choice_text_parse_tokens = Vec::new();
  let mut choice_validate_tokens = Vec::new();
  for field in &choice_fields {
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
    let build_visit_block = |reader_ident: &Ident| {
      if field.repeated {
        quote! {
          if !matched {
            match #choice_ty::deserialize_inner(#reader_ident, Some((e.clone(), next_empty))) {
              Ok(parsed_choice) => {
                #field_ident.push(parsed_choice);
                matched = true;
              }
              Err(crate::common::SdkError::MissingField { .. }) => {
                matched = true;
              }
              Err(crate::common::SdkError::UnexpectedTag { expected, .. })
                if expected == "choice" => {}
              Err(err) => return Err(err),
            }
          }
        }
      } else {
        quote! {
          if !matched {
            match #choice_ty::deserialize_inner(#reader_ident, Some((e.clone(), next_empty))) {
              Ok(parsed_choice) => {
                #field_ident = Some(parsed_choice);
                matched = true;
              }
              Err(crate::common::SdkError::MissingField { .. }) => {
                matched = true;
              }
              Err(crate::common::SdkError::UnexpectedTag { expected, .. })
                if expected == "choice" => {}
              Err(err) => return Err(err),
            }
          }
        }
      }
    };
    let build_parse_block = |reader_ident: &Ident| {
      if field.repeated {
        quote! {
          if !matched {
            match #choice_ty::deserialize_inner(#reader_ident, Some((e.clone(), next_empty))) {
              Ok(parsed_choice) => {
                #field_ident.push(parsed_choice);
                matched = true;
              }
              Err(crate::common::SdkError::MissingField { .. }) => {
                matched = true;
              }
              Err(crate::common::SdkError::UnexpectedTag { expected, .. })
                if expected == "choice" => {}
              Err(err) => return Err(err),
            }
          }
        }
      } else {
        quote! {
          if !matched {
            match #choice_ty::deserialize_inner(#reader_ident, Some((e.clone(), next_empty))) {
              Ok(parsed_choice) => {
                #field_ident = Some(parsed_choice);
                matched = true;
              }
              Err(crate::common::SdkError::MissingField { .. }) => {
                matched = true;
              }
              Err(crate::common::SdkError::UnexpectedTag { expected, .. })
                if expected == "choice" => {}
              Err(err) => return Err(err),
            }
          }
        }
      }
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
      choice_parse_tokens.push(build_parse_block(&xml_reader_ident));
      choice_visit_parse_tokens.push(build_visit_block(&visitor_reader_ident));
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
      choice_parse_tokens.push(build_parse_block(&xml_reader_ident));
      choice_visit_parse_tokens.push(build_visit_block(&visitor_reader_ident));
      choice_text_parse_tokens.push(build_text_block(quote! { &text_value }));
    }
  }

  let mut any_decl_tokens = Vec::new();
  let mut any_init_tokens = Vec::new();
  let mut any_parse_tokens = Vec::new();
  let mut any_visit_parse_tokens = Vec::new();
  for field in &any_fields {
    let field_ident = &field.ident;

    if field.repeated {
      any_decl_tokens.push(quote! { let mut #field_ident = Vec::new(); });
      any_init_tokens.push(quote! { #field_ident });
      any_parse_tokens.push(quote! {
        if !matched {
          let xml = crate::common::read_outer_xml(xml_reader, e.clone(), next_empty)?;
          #field_ident.push(xml);
          matched = true;
        }
      });
      any_visit_parse_tokens.push(quote! {
        if !matched {
          let xml = crate::common::read_outer_xml(xml_reader, e.clone(), next_empty)?;
          #field_ident.push(xml);
          matched = true;
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
      any_parse_tokens.push(quote! {
        if !matched {
          let xml = crate::common::read_outer_xml(xml_reader, e.clone(), next_empty)?;
          #field_ident = Some(xml);
          matched = true;
        }
      });
      any_visit_parse_tokens.push(quote! {
        if !matched {
          let xml = crate::common::read_outer_xml(xml_reader, e.clone(), next_empty)?;
          #field_ident = Some(xml);
          matched = true;
        }
      });
    }
  }

  let has_child_dispatch = !child_fields.is_empty() || !text_child_fields.is_empty();
  let visit_foreign_child_tokens = if !has_child_dispatch && choice_fields.is_empty() {
    quote! {
      let mut visit_foreign_child = |
        _xml_reader: &mut R,
        _e: quick_xml::events::BytesStart<'de>,
        _next_empty: bool,
      | -> Result<bool, crate::common::SdkError> {
        Ok(false)
      };
    }
  } else if choice_fields.is_empty() {
    quote! {
      let mut visit_foreign_child = |
        xml_reader: &mut R,
        e: quick_xml::events::BytesStart<'de>,
        next_empty: bool,
      | -> Result<bool, crate::common::SdkError> {
        match e.name().as_ref() {
          #( #child_visit_parse_tokens )*
          _ => Ok(false),
        }
      };
    }
  } else {
    quote! {
      let mut visit_foreign_child = |
        xml_reader: &mut R,
        e: quick_xml::events::BytesStart<'de>,
        next_empty: bool,
      | -> Result<bool, crate::common::SdkError> {
        let matched: bool = match e.name().as_ref() {
          #( #child_visit_parse_tokens )*
          _ => Ok::<bool, crate::common::SdkError>(false),
        }?;
        if matched {
          return Ok(true);
        }
        let mut matched = false;
        #( #choice_visit_parse_tokens )*
        Ok(matched)
      };
    }
  };
  let child_choice_dispatch_tokens = if choice_fields.is_empty() {
    quote! {
      let matched = match e.name().as_ref() {
        #( #child_parse_tokens )*
        _ => false,
      };
      if matched {
        continue;
      }
    }
  } else {
    quote! {
      let mut matched = match e.name().as_ref() {
        #( #child_parse_tokens )*
        _ => false,
      };
      if !matched {
        #( #choice_parse_tokens )*
      }
      #( #any_parse_tokens )*
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
  for field in &fields.named {
    let field_ident = field
      .ident
      .as_ref()
      .ok_or_else(|| syn::Error::new_spanned(field, "SdkType requires named fields"))?;
    match parse_sdk_type_field_kind(&field.attrs)? {
      Some(SdkTypeFieldKind::Child { .. }) => {
        let repeated = contains_vec_type(&field.ty);
        let optional = is_option_type(&field.ty);
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
      Some(SdkTypeFieldKind::TextChild { qname }) => {
        ordered_write_tokens.push(build_text_child_write_tokens(
          field_ident,
          &qname,
          contains_vec_type(&field.ty),
          is_option_type(&field.ty),
        ));
      }
      Some(SdkTypeFieldKind::Choice) => {
        let repeated = contains_vec_type(&field.ty);
        let optional = is_option_type(&field.ty);
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
      Some(SdkTypeFieldKind::Any) => {
        let repeated = contains_vec_type(&field.ty);
        let optional = is_option_type(&field.ty);
        if repeated {
          ordered_write_tokens.push(quote! {
            for value in &self.#field_ident {
              writer.write_str(value.as_ref())?;
            }
          });
        } else if optional {
          ordered_write_tokens.push(quote! {
            if let Some(value) = &self.#field_ident {
              writer.write_str(value.as_ref())?;
            }
          });
        } else {
          ordered_write_tokens.push(quote! {
            writer.write_str(self.#field_ident.as_ref())?;
          });
        }
      }
      Some(SdkTypeFieldKind::Text) => {
        if is_option_type(&field.ty) {
          ordered_write_tokens.push(quote! {
            if let Some(value) = &self.#field_ident {
              crate::common::write_escaped_text(writer, value)?;
            }
          });
        } else {
          ordered_write_tokens.push(quote! {
            crate::common::write_escaped_text(writer, &self.#field_ident)?;
          });
        }
      }
      Some(SdkTypeFieldKind::Attr { .. }) | None => {}
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
      if text_field.optional {
        quote! {
          #field_ident: match #field_ident {
            Some(value) => Some(crate::common::parse_value::<#inner_ty>(
              &value,
              stringify!(#ident),
              #field_name_lit,
            )?),
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
            crate::common::parse_value::<#inner_ty>(&value, stringify!(#ident), #field_name_lit)?
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
        crate::common::write_attr_value(writer, "mc:Ignorable", mc_ignorable)?;
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
          writer.write_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\r\n")?;
        }
        crate::common::XmlHeaderType::Plain => {
          writer.write_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\r\n")?;
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
        Self::deserialize_inner(&mut xml_reader, None)
      }
    }

    impl #ident {
      pub fn from_str(s: &str) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_str_inner(s)?;
        Self::deserialize_inner(&mut xml_reader, None)
      }

      pub fn from_reader<R: std::io::BufRead>(
        reader: R,
      ) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_reader_inner(reader)?;
        Self::deserialize_inner(&mut xml_reader, None)
      }

      pub(crate) fn deserialize_inner<'de, R: crate::common::XmlReader<'de>>(
        xml_reader: &mut R,
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
            let mut next_event = None;
            let mut next_empty = false;
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
                next_event = Some(e);
              }
              quick_xml::events::Event::Empty(e) => {
                next_empty = true;
                next_event = Some(e);
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

            if let Some(e) = next_event {
              #child_choice_dispatch_tokens
              #visit_foreign_child_tokens
              match e.name().as_ref() {
                b"mc:AlternateContent" | b"AlternateContent" => {
                  crate::common::process_markup_compatibility_children(
                    xml_reader,
                    next_empty,
                    &mut visit_foreign_child,
                  )?;
                }
                _ => {
                  if crate::common::is_foreign_prefixed_child(
                    e.name().as_ref(),
                    #tag_prefix,
                  ) {
                    crate::common::process_foreign_element_children(
                      xml_reader,
                      next_empty,
                      &mut visit_foreign_child,
                    )?;
                  } else {
                    Err(crate::common::unexpected_tag(
                      stringify!(#ident),
                      "known child",
                      e.name().as_ref(),
                    ))?;
                  }
                }
              }
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

      pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
        let mut writer = String::with_capacity(32);
        self.write_xml(&mut writer, #to_xml_prefix_tokens)?;
        Ok(writer)
      }

      pub(crate) fn write_xml<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        xmlns_prefix: &str,
      ) -> Result<(), std::fmt::Error> {
        #xml_header_tokens
        crate::common::write_start_tag_open(writer, xmlns_prefix, #tag_prefix, #local_name)?;
        #special_namespace_write_tokens
        #mc_ignorable_write_tokens
        #( #attr_write_tokens )*
        if #has_body {
          writer.write_char('>')?;
          #( #ordered_write_tokens )*
          crate::common::write_end_tag(writer, xmlns_prefix, #tag_prefix, #local_name)?;
        } else {
          writer.write_str("/>")?;
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
