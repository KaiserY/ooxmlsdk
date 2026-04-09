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
    Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
      let Some(schema_qname) = parse_sdk_qname(&input.attrs)? else {
        return Ok(quote! {
          impl crate::sdk::SdkType for #ident {}
        });
      };
      expand_text_wrapper(input, &schema_qname, fields)
    }
    Fields::Named(fields) => {
      if let Some(schema_qname) = parse_sdk_qname(&input.attrs)? {
        return expand_named_struct(input, &schema_qname, fields);
      }
      expand_helper_struct(input, fields)
    }
    _ => Ok(quote! {
      impl crate::sdk::SdkType for #ident {}
    }),
  }
}

fn sdk_type_impl_tokens(
  ident: &Ident,
  impl_items: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  quote! {
    impl crate::sdk::SdkType for #ident {}

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

fn expand_sequence_helper_struct(
  input: &DeriveInput,
  fields: &syn::FieldsNamed,
) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;

  let mut child_decl_tokens = Vec::new();
  let mut child_parse_tokens = Vec::new();
  let mut child_write_tokens = Vec::new();
  let mut child_init_tokens = Vec::new();

  let xml_reader_ident = Ident::new("xml_reader", Span::call_site());

  for field in &fields.named {
    let field_ident = field
      .ident
      .as_ref()
      .ok_or_else(|| syn::Error::new_spanned(field, "SdkType requires named fields"))?;
    let Some(SdkTypeFieldKind::Child { qname }) = parse_sdk_type_field_kind(&field.attrs)? else {
      return Err(syn::Error::new_spanned(
        field,
        "sequence helper structs require only #[sdk(child(...))] fields",
      ));
    };

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
      child_parse_tokens.push(quote! {
        #match_target => {
          match #child_ty::deserialize_inner(#xml_reader_ident, Some((e.clone(), next_empty))) {
            Ok(parsed_child) => {
              #field_ident.push(#parsed_child_expr);
            }
            Err(crate::common::SdkError::MissingField { .. }) => {}
            Err(err) => return Err(err),
          }
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
      } else {
        child_write_tokens.push(quote! {
          self.#field_ident.write_xml(writer, xmlns_prefix)?;
        });
      }
      child_parse_tokens.push(quote! {
        #match_target => {
          match #child_ty::deserialize_inner(#xml_reader_ident, Some((e.clone(), next_empty))) {
            Ok(parsed_child) => {
              #field_ident = Some(#parsed_child_expr);
            }
            Err(crate::common::SdkError::MissingField { .. }) => {}
            Err(err) => return Err(err),
          }
        }
      });
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
            match e.name().as_ref() {
              #( #child_parse_tokens )*
              _ => {
                xml_reader.unread(if next_empty {
                  quick_xml::events::Event::Empty(e)
                } else {
                  quick_xml::events::Event::Start(e)
                })?;
                break;
              }
            }
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
  ))
}

fn expand_helper_struct(
  input: &DeriveInput,
  fields: &syn::FieldsNamed,
) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;

  let mut child_fields = Vec::new();
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
      Some(SdkTypeFieldKind::Attr { .. }) | None => {
        return Err(syn::Error::new_spanned(
          field,
          "helper structs require #[sdk(child(...))] or #[sdk(choice)] fields",
        ));
      }
    }
  }

  let mut child_decl_tokens = Vec::new();
  let mut child_parse_tokens = Vec::new();
  let mut child_visit_parse_tokens = Vec::new();
  let mut child_write_tokens = Vec::new();
  let mut child_init_tokens = Vec::new();
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
      } else {
        child_write_tokens.push(quote! {
          self.#field_ident.write_xml(writer, xmlns_prefix)?;
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
  for field in &choice_fields {
    let field_ident = &field.ident;
    let choice_ty = unwrap_option_vec_type(&field.ty);
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
      } else {
        choice_write_tokens.push(quote! {
          self.#field_ident.write_xml(writer, xmlns_prefix)?;
        });
      }
      choice_parse_tokens.push(build_parse_arm(&xml_reader_ident));
      choice_visit_parse_tokens.push(build_visit_arm(&visitor_reader_ident));
    }
  }

  if child_fields.len() == fields.named.len() {
    return expand_sequence_helper_struct(input, fields);
  }

  let visit_foreign_child_tokens = if child_fields.is_empty() && choice_fields.is_empty() {
    quote! {
      let mut visit_foreign_child = |
        xml_reader: &mut R,
        _e: quick_xml::events::BytesStart<'de>,
        _next_empty: bool,
      | -> Result<bool, crate::common::SdkError> {
        Ok(false)
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

  Ok(quote! {
    impl crate::sdk::SdkType for #ident {}

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

fn expand_text_wrapper(
  input: &DeriveInput,
  schema_qname: &str,
  fields: &syn::FieldsUnnamed,
) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let QNameInfo {
    tag_prefix,
    local_name,
  } = parse_qname_info(schema_qname);
  if local_name.is_empty() {
    return Ok(quote! {
      impl crate::sdk::SdkType for #ident {}
    });
  }
  let tag_qname = if tag_prefix.is_empty() {
    local_name.clone()
  } else {
    format!("{tag_prefix}:{local_name}")
  };
  let tag_qname_lit = LitByteStr::new(tag_qname.as_bytes(), Span::call_site());
  let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
  let field = fields.unnamed.first().expect("one field exists");
  let field_ident = syn::Index::from(0);
  let inner_ty = unwrap_wrapped_type(&field.ty);
  let optional = is_option_type(&field.ty);
  let body_tokens = if optional {
    quote! {
      if let Some(xml_content) = &self.#field_ident {
        crate::common::write_escaped_text(writer, xml_content)?;
      }
    }
  } else {
    quote! {
      crate::common::write_escaped_text(writer, &self.#field_ident)?;
    }
  };
  let deserialize_tokens = if optional {
    quote! {
      let mut text = None;
      if !empty_tag {
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
                "value",
              )?;
            }
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
      Ok(Self(match text {
        Some(value) => Some(crate::common::parse_value::<#inner_ty>(
          &value,
          stringify!(#ident),
          "value",
        )?),
        None => None,
      }))
    }
  } else {
    quote! {
      let mut text = None;
      if !empty_tag {
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
                "value",
              )?;
            }
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
      let text = text.ok_or_else(|| crate::common::missing_field(stringify!(#ident), "value"))?;
      Ok(Self(crate::common::parse_value::<#inner_ty>(
        &text,
        stringify!(#ident),
        "value",
      )?))
    }
  };

  Ok(quote! {
    impl crate::sdk::SdkType for #ident {}

    impl From<#inner_ty> for #ident {
      fn from(value: #inner_ty) -> Self {
        Self(Some(value))
      }
    }

    impl From<Option<#inner_ty>> for #ident {
      fn from(value: Option<#inner_ty>) -> Self {
        Self(value)
      }
    }

    impl std::ops::Deref for #ident {
      type Target = Option<#inner_ty>;

      fn deref(&self) -> &Self::Target {
        &self.0
      }
    }

    impl std::ops::DerefMut for #ident {
      fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
          #tag_qname_lit | #local_name_lit => {}
          found => {
            return Err(crate::common::unexpected_tag(
              stringify!(#ident),
              stringify!(#ident),
              found,
            ));
          }
        }
        #deserialize_tokens

      }

      pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
        let mut writer = String::with_capacity(32);
        self.write_xml(&mut writer, "")?;
        Ok(writer)
      }

      pub(crate) fn write_xml<W: std::fmt::Write>(
        &self,
        writer: &mut W,
        xmlns_prefix: &str,
      ) -> Result<(), std::fmt::Error> {
        crate::common::write_start_tag_open(writer, xmlns_prefix, #tag_prefix, #local_name)?;
        writer.write_char('>')?;
        #body_tokens
        crate::common::write_end_tag(writer, xmlns_prefix, #tag_prefix, #local_name)?;
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

fn expand_named_struct(
  input: &DeriveInput,
  schema_qname: &str,
  fields: &syn::FieldsNamed,
) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let has_xml_header = has_struct_xml_header_attr(&input.attrs);
  let QNameInfo {
    tag_prefix,
    local_name,
  } = parse_qname_info(schema_qname);
  if local_name.is_empty() {
    return Ok(quote! {
      impl crate::sdk::SdkType for #ident {}
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
  let mut choice_fields = Vec::new();
  let mut text_field = None;
  let mut xmlns_fields = Vec::new();
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
      }),
      Some(SdkTypeFieldKind::Child { qname }) => child_fields.push(SdkChildField {
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

  let has_xmlns_fields = !xmlns_fields.is_empty();
  let has_mc_ignorable_field = mc_ignorable_field.is_some();

  let mut attr_decl_tokens = Vec::new();
  let mut attr_parse_tokens = Vec::new();
  let mut attr_write_tokens = Vec::new();
  let mut attr_init_tokens = Vec::new();
  let mut attr_finish_tokens = Vec::new();
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
  let namespace_attr_parse_tokens = match (has_xmlns_fields, has_mc_ignorable_field) {
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
  };

  let mut child_decl_tokens = Vec::new();
  let mut child_parse_tokens = Vec::new();
  let mut child_visit_parse_tokens = Vec::new();
  let mut child_write_tokens = Vec::new();
  let mut child_init_tokens = Vec::new();
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
      } else {
        child_write_tokens.push(quote! {
          self.#field_ident.write_xml(writer, xmlns_prefix)?;
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
  for field in &choice_fields {
    let field_ident = &field.ident;
    let choice_ty = unwrap_option_vec_type(&field.ty);
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
    if field.repeated {
      choice_decl_tokens.push(quote! { let mut #field_ident = Vec::new(); });
      choice_init_tokens.push(quote! { #field_ident });
      choice_write_tokens.push(quote! {
        for choice in &self.#field_ident {
          choice.write_xml(writer, xmlns_prefix)?;
        }
      });
      choice_parse_tokens.push(build_parse_block(&xml_reader_ident));
      choice_visit_parse_tokens.push(build_visit_block(&visitor_reader_ident));
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
      } else {
        choice_write_tokens.push(quote! {
          self.#field_ident.write_xml(writer, xmlns_prefix)?;
        });
      }
      choice_parse_tokens.push(build_parse_block(&xml_reader_ident));
      choice_visit_parse_tokens.push(build_visit_block(&visitor_reader_ident));
    }
  }

  let visit_foreign_child_tokens = if child_fields.is_empty() && choice_fields.is_empty() {
    quote! {
      let mut visit_foreign_child = |
        _xml_reader: &mut R,
        _e: quick_xml::events::BytesStart<'de>,
        _next_empty: bool,
      | -> Result<bool, crate::common::SdkError> {
        Ok(false)
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
    quote! {}
  };
  let text_write_tokens = if let Some(text_field) = &text_field {
    let field_ident = &text_field.ident;
    if text_field.optional {
      quote! {
        if let Some(value) = &self.#field_ident {
          crate::common::write_escaped_text(writer, value)?;
        }
      }
    } else {
      quote! {
        crate::common::write_escaped_text(writer, &self.#field_ident)?;
      }
    }
  } else {
    quote! {}
  };
  let mut body_write_tokens = Vec::new();
  let mut child_write_idx = 0usize;
  let mut choice_write_idx = 0usize;
  for field in &fields.named {
    match parse_sdk_type_field_kind(&field.attrs)? {
      Some(SdkTypeFieldKind::Child { .. }) => {
        body_write_tokens.push(child_write_tokens[child_write_idx].clone());
        child_write_idx += 1;
      }
      Some(SdkTypeFieldKind::Choice) => {
        body_write_tokens.push(choice_write_tokens[choice_write_idx].clone());
        choice_write_idx += 1;
      }
      Some(SdkTypeFieldKind::Text) => {
        body_write_tokens.push(text_write_tokens.clone());
      }
      Some(SdkTypeFieldKind::Attr { .. }) | None => {}
    }
  }
  let text_finish_tokens = if let Some(text_field) = &text_field {
    let field_ident = &text_field.ident;
    let inner_ty = unwrap_wrapped_type(&text_field.ty);
    let field_name_lit = LitStr::new(&field_ident.to_string(), Span::call_site());
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
  let xml_header_tokens = if has_xml_header {
    quote! {
      writer.write_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\r\n")?;
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

  let has_body = !child_fields.is_empty() || !choice_fields.is_empty() || text_field.is_some();

  Ok(quote! {
    impl crate::sdk::SdkType for #ident {}

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
        #text_decl_tokens

        if !empty_tag {
          loop {
            let mut next_event = None;
            let mut next_empty = false;
            match xml_reader.next()? {
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
              let mut matched = match e.name().as_ref() {
                #( #child_parse_tokens )*
                _ => false,
              };
              if !matched {
                #( #choice_parse_tokens )*
              }
              if matched {
                continue;
              }
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
          #text_finish_tokens
          #( #attr_finish_tokens, )*
          #special_namespace_init_tokens
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
          #( #body_write_tokens )*
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
