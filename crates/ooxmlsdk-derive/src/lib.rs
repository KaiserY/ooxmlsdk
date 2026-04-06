use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::{
  io::Write,
  sync::{Mutex, OnceLock},
};
use syn::{
  Attribute, Data, DataEnum, DeriveInput, Fields, Ident, LitByteStr, LitStr, Meta, Token, Type,
  TypePath, parse_macro_input,
};

static SNAPSHOT_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
static SNAPSHOT_INIT: OnceLock<()> = OnceLock::new();
const SNAPSHOT_DIR: &str = concat!(
  env!("CARGO_MANIFEST_DIR"),
  "/../../tmp/ooxmlsdk_macro_expanded"
);

#[proc_macro_derive(SdkEnum, attributes(sdk))]
pub fn sdk_enum(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  match expand_sdk_enum(&input) {
    Ok(tokens) => {
      dump_macro_expansion("SdkEnum", &input, &tokens);
      tokens.into()
    }
    Err(err) => err.to_compile_error().into(),
  }
}

#[proc_macro_derive(SdkType, attributes(sdk))]
pub fn sdk_type(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  match expand_sdk_type(&input) {
    Ok(tokens) => {
      dump_macro_expansion("SdkType", &input, &tokens);
      tokens.into()
    }
    Err(err) => err.to_compile_error().into(),
  }
}

#[proc_macro_derive(SdkChoice, attributes(sdk))]
pub fn sdk_choice(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  match expand_sdk_choice(&input) {
    Ok(tokens) => {
      dump_macro_expansion("SdkChoice", &input, &tokens);
      tokens.into()
    }
    Err(err) => err.to_compile_error().into(),
  }
}

fn dump_macro_expansion(kind: &str, input: &DeriveInput, tokens: &proc_macro2::TokenStream) {
  let lock = SNAPSHOT_LOCK.get_or_init(|| Mutex::new(()));
  let _guard = lock.lock().expect("snapshot lock");
  SNAPSHOT_INIT.get_or_init(|| {
    let _ = std::fs::remove_dir_all(SNAPSHOT_DIR);
    let _ = std::fs::create_dir_all(SNAPSHOT_DIR);
  });
  let kind_dir = format!("{SNAPSHOT_DIR}/{kind}");
  if std::fs::create_dir_all(&kind_dir).is_err() {
    return;
  }
  let snapshot_name = snapshot_file_stem(input);
  let file_path = format!("{kind_dir}/{snapshot_name}.rs");
  let mut file = match std::fs::File::create(&file_path) {
    Ok(file) => file,
    Err(_) => return,
  };
  let _ = writeln!(
    file,
    "// ===== {}: {} =====\n{}\n",
    kind, input.ident, tokens
  );
}

fn expand_sdk_enum(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let Data::Enum(DataEnum { variants, .. }) = &input.data else {
    return Err(syn::Error::new_spanned(
      input,
      "SdkEnum can only be derived for enums",
    ));
  };

  let mut as_xml_arms = Vec::with_capacity(variants.len());
  let mut from_str_arms = Vec::with_capacity(variants.len());
  let mut from_bytes_arms = Vec::with_capacity(variants.len());

  for variant in variants {
    if !matches!(variant.fields, Fields::Unit) {
      return Err(syn::Error::new_spanned(
        &variant.ident,
        "SdkEnum only supports fieldless variants",
      ));
    }

    let variant_ident = &variant.ident;
    let cfg_attrs = cfg_attrs(&variant.attrs);
    let (rename, aliases) = parse_sdk_enum_variant_attrs(&variant.attrs, variant_ident)?;
    let xml_value = rename.unwrap_or_else(|| variant_ident.to_string());
    let xml_value_lit = LitStr::new(&xml_value, Span::call_site());
    let xml_bytes_lit = LitByteStr::new(xml_value.as_bytes(), Span::call_site());

    as_xml_arms.push(quote! {
      #(#cfg_attrs)*
      Self::#variant_ident => #xml_value_lit,
    });
    from_str_arms.push(quote! {
      #(#cfg_attrs)*
      #xml_value_lit => Ok(Self::#variant_ident),
    });
    from_bytes_arms.push(quote! {
      #(#cfg_attrs)*
      #xml_bytes_lit => Ok(Self::#variant_ident),
    });

    for alias in aliases {
      let alias_lit = LitStr::new(&alias, Span::call_site());
      let alias_bytes_lit = LitByteStr::new(alias.as_bytes(), Span::call_site());
      from_str_arms.push(quote! {
        #(#cfg_attrs)*
        #alias_lit => Ok(Self::#variant_ident),
      });
      from_bytes_arms.push(quote! {
        #(#cfg_attrs)*
        #alias_bytes_lit => Ok(Self::#variant_ident),
      });
    }
  }

  let ty_name = ident.to_string();

  Ok(quote! {
    impl crate::sdk::SdkEnum for #ident {
      fn as_xml_str(&self) -> &'static str {
        match self {
          #( #as_xml_arms )*
        }
      }
    }

    impl #ident {
      pub fn as_xml_str(&self) -> &'static str {
        <Self as crate::sdk::SdkEnum>::as_xml_str(self)
      }

      pub fn to_xml(&self) -> String {
        self.as_xml_str().to_string()
      }

      pub fn from_bytes(b: &[u8]) -> Result<Self, crate::common::SdkError> {
        match b {
          #( #from_bytes_arms )*
          other => Err(crate::common::invalid_enum_value(
            #ty_name,
            String::from_utf8_lossy(other).into_owned(),
          )),
        }
      }
    }

    impl ::std::str::FromStr for #ident {
      type Err = crate::common::SdkError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
          #( #from_str_arms )*
          _ => Err(crate::common::invalid_enum_value(#ty_name, s)),
        }
      }
    }

    impl ::std::fmt::Display for #ident {
      fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_xml_str())
      }
    }
  })
}

fn snapshot_file_stem(input: &DeriveInput) -> String {
  let raw_name = parse_sdk_qname(&input.attrs)
    .ok()
    .flatten()
    .unwrap_or_else(|| input.ident.to_string());
  sanitize_snapshot_component(&raw_name)
}

fn sanitize_snapshot_component(value: &str) -> String {
  let mut out = String::with_capacity(value.len());
  for ch in value.chars() {
    if ch.is_ascii_alphanumeric() {
      out.push(ch);
    } else {
      out.push('_');
    }
  }
  if out.is_empty() {
    "unknown".to_string()
  } else {
    out
  }
}

fn expand_sdk_type(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let Data::Struct(data_struct) = &input.data else {
    return Err(syn::Error::new_spanned(
      input,
      "SdkType can only be derived for structs",
    ));
  };

  match &data_struct.fields {
    Fields::Unnamed(fields) if fields.unnamed.len() == 1 && has_struct_text_attr(&input.attrs) => {
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

  Ok(quote! {
    impl crate::sdk::SdkType for #ident {}

    impl #ident {
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
    }
  })
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
      child_visit_parse_tokens.push(build_arm(&xml_reader_ident, true));
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
      child_visit_parse_tokens.push(build_arm(&xml_reader_ident, true));
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
          _ => false,
        };
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
  let mut special_namespace_fields = Vec::new();

  for field in &fields.named {
    let field_ident = field
      .ident
      .as_ref()
      .ok_or_else(|| syn::Error::new_spanned(field, "SdkType requires named fields"))?;
    if is_namespace_field(field_ident) {
      special_namespace_fields.push(field_ident.clone());
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

  let has_namespace_fields = !special_namespace_fields.is_empty();

  let mut attr_decl_tokens = Vec::new();
  let mut attr_parse_tokens = Vec::new();
  let mut attr_write_tokens = Vec::new();
  let mut attr_init_tokens = Vec::new();
  let mut attr_finish_tokens = Vec::new();
  for field in &attr_fields {
    let field_ident = &field.ident;
    let name_lit = LitStr::new(&field.name, Span::call_site());
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
        if attr.key.as_ref() == #name_lit.as_bytes() {
          #field_ident = Some(#parser);
        }
      });
      attr_init_tokens.push(quote! { #field_ident });
    } else {
      attr_decl_tokens.push(quote! { let mut #field_ident = None; });
      attr_parse_tokens.push(quote! {
        if attr.key.as_ref() == #name_lit.as_bytes() {
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

  let special_namespace_parse_tokens = if has_namespace_fields {
    quote! {
      let mut xmlns = None;
      let mut xmlns_map = std::collections::HashMap::<String, String>::new();
      let mut mc_ignorable = None;
      for attr in e.attributes().with_checks(false) {
        let attr = attr?;
        match attr.key.as_ref() {
          b"xmlns" => {
            xmlns = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?);
          }
          b"mc:Ignorable" => {
            mc_ignorable = Some(crate::common::decode_attr_value(&attr, xml_reader.decoder())?);
          }
          key => {
            if key.starts_with(b"xmlns:") {
              let prefix = String::from_utf8_lossy(&key[6..]).into_owned();
              let uri = crate::common::decode_attr_value(&attr, xml_reader.decoder())?;
              if let Some(canonical_prefix) = crate::namespaces::prefix_by_uri(uri.as_str()) {
                xmlns_map.entry(canonical_prefix.to_string()).or_insert(uri);
              } else {
                xmlns_map.insert(prefix, uri);
              }
            }
          }
        }
      }
    }
  } else {
    quote! {}
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
      child_visit_parse_tokens.push(build_arm(&xml_reader_ident, true));
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
      child_visit_parse_tokens.push(build_arm(&xml_reader_ident, true));
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
    let parsed_choice_expr = if box_inner_type(&choice_ty).is_some() {
      quote! { std::boxed::Box::new(parsed_choice) }
    } else {
      quote! { parsed_choice }
    };
    let build_visit_block = |reader_ident: &Ident| {
      if field.repeated {
        quote! {
          if !matched {
            match #choice_ty::deserialize_inner(#reader_ident, Some((e.clone(), next_empty))) {
              Ok(parsed_choice) => {
                #field_ident.push(#parsed_choice_expr);
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
                #field_ident = Some(#parsed_choice_expr);
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
                #field_ident.push(#parsed_choice_expr);
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
                #field_ident = Some(#parsed_choice_expr);
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

  let special_namespace_write_tokens = if has_namespace_fields {
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
      if let Some(mc_ignorable) = &self.mc_ignorable {
        crate::common::write_attr_value(writer, "mc:Ignorable", mc_ignorable)?;
      }
    }
  } else {
    quote! {}
  };
  let special_namespace_init_tokens = if has_namespace_fields {
    quote! {
      xmlns,
      xmlns_map,
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
  let to_xml_prefix_tokens = if has_namespace_fields {
    quote! {
      if let Some(xmlns) = &self.xmlns {
        if crate::namespaces::prefix_by_uri(xmlns.as_str()) == Some(#tag_prefix) {
          #tag_prefix
        } else {
          ""
        }
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
        #special_namespace_parse_tokens
        #( #child_decl_tokens )*
        #( #choice_decl_tokens )*
        #text_decl_tokens

        for attr in e.attributes().with_checks(false) {
          let attr = attr?;
          #( #attr_parse_tokens )*
        }

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
        #( #attr_write_tokens )*
        if #has_body {
          writer.write_char('>')?;
          #text_write_tokens
          #( #child_write_tokens )*
          #( #choice_write_tokens )*
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

fn expand_sdk_choice(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
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

#[derive(Clone)]
struct SdkAttrField {
  ident: Ident,
  name: String,
  ty: Type,
  optional: bool,
}

#[derive(Clone)]
struct SdkChildField {
  ident: Ident,
  qname: String,
  ty: Type,
  optional: bool,
  repeated: bool,
}

#[derive(Clone)]
struct SdkChoiceField {
  ident: Ident,
  ty: Type,
  optional: bool,
  repeated: bool,
}

#[derive(Clone)]
struct SdkTextField {
  ident: Ident,
  ty: Type,
  optional: bool,
}

enum SdkTypeFieldKind {
  Attr { name: String },
  Child { qname: String },
  Text,
  Choice,
}

enum SdkChoiceVariantKind {
  Child { qnames: Vec<String> },
  Any,
}

#[derive(Clone)]
struct QNameInfo {
  tag_prefix: String,
  local_name: String,
}

fn parse_sdk_qname(attrs: &[Attribute]) -> syn::Result<Option<String>> {
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in metas {
      if let Meta::NameValue(meta) = meta
        && meta.path.is_ident("qname")
      {
        return Ok(Some(parse_string_expr(meta.value)?));
      }
    }
  }
  Ok(None)
}

fn parse_sdk_type_field_kind(attrs: &[Attribute]) -> syn::Result<Option<SdkTypeFieldKind>> {
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    if let Some(meta) = metas.into_iter().next() {
      match meta {
        Meta::List(meta) if meta.path.is_ident("attr") => {
          let mut name = None;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("qname") {
              let value: LitStr = nested.value()?.parse()?;
              name = Some(normalize_attr_qname(&value.value()));
              Ok(())
            } else {
              Err(nested.error("unsupported sdk attr attribute"))
            }
          })?;
          return Ok(Some(SdkTypeFieldKind::Attr {
            name: name.unwrap_or_default(),
          }));
        }
        Meta::List(meta) if meta.path.is_ident("child") => {
          let mut qname = None;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("qname") {
              let value: LitStr = nested.value()?.parse()?;
              qname = Some(value.value());
              Ok(())
            } else {
              Err(nested.error("unsupported sdk child attribute"))
            }
          })?;
          return Ok(Some(SdkTypeFieldKind::Child {
            qname: qname.unwrap_or_default(),
          }));
        }
        Meta::Path(path) if path.is_ident("text") => return Ok(Some(SdkTypeFieldKind::Text)),
        Meta::Path(path) if path.is_ident("choice") => return Ok(Some(SdkTypeFieldKind::Choice)),
        Meta::Path(path) if path.is_ident("any") => {
          return Ok(Some(SdkTypeFieldKind::Child {
            qname: String::new(),
          }));
        }
        other => {
          return Err(syn::Error::new_spanned(
            other,
            "unsupported sdk field attribute",
          ));
        }
      }
    }
  }
  Ok(None)
}

fn parse_sdk_choice_variant_kind(attrs: &[Attribute]) -> syn::Result<Option<SdkChoiceVariantKind>> {
  let mut child_qnames = Vec::new();
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    if let Some(meta) = metas.into_iter().next() {
      match meta {
        Meta::List(meta) if meta.path.is_ident("child") => {
          let mut qname = None;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("qname") {
              let value: LitStr = nested.value()?.parse()?;
              qname = Some(value.value());
              Ok(())
            } else {
              Err(nested.error("unsupported sdk choice child attribute"))
            }
          })?;
          child_qnames.push(qname.unwrap_or_default());
        }
        Meta::Path(path) if path.is_ident("any") => return Ok(Some(SdkChoiceVariantKind::Any)),
        other => {
          let _ = other;
        }
      }
    }
  }
  if !child_qnames.is_empty() {
    return Ok(Some(SdkChoiceVariantKind::Child {
      qnames: child_qnames,
    }));
  }
  Ok(None)
}

fn normalize_attr_qname(qname: &str) -> String {
  qname.strip_prefix(':').unwrap_or(qname).to_string()
}

fn parse_sdk_enum_variant_attrs(
  attrs: &[Attribute],
  variant_ident: &Ident,
) -> syn::Result<(Option<String>, Vec<String>)> {
  let mut rename = None;
  let mut aliases = Vec::new();
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in metas {
      match meta {
        Meta::NameValue(meta) if meta.path.is_ident("rename") => {
          rename = Some(parse_string_expr(meta.value)?);
        }
        Meta::NameValue(meta) if meta.path.is_ident("alias") => {
          aliases.extend(parse_alias_expr(meta.value)?);
        }
        Meta::Path(path) if path.is_ident("default") => {}
        other => {
          return Err(syn::Error::new_spanned(
            other,
            "unsupported sdk enum attribute",
          ));
        }
      }
    }
  }

  if rename.is_none() && aliases.is_empty() {
    rename = Some(variant_ident.to_string());
  }
  Ok((rename, aliases))
}

fn parse_alias_expr(expr: syn::Expr) -> syn::Result<Vec<String>> {
  Ok(vec![parse_string_expr(expr)?])
}

fn parse_string_expr(expr: syn::Expr) -> syn::Result<String> {
  match expr {
    syn::Expr::Lit(expr) => match expr.lit {
      syn::Lit::Str(value) => Ok(value.value()),
      other => Err(syn::Error::new_spanned(other, "expected string literal")),
    },
    other => Err(syn::Error::new_spanned(other, "expected string literal")),
  }
}

fn cfg_attrs(attrs: &[Attribute]) -> Vec<Attribute> {
  attrs
    .iter()
    .filter(|attr| attr.path().is_ident("cfg"))
    .cloned()
    .collect()
}

fn parse_qname_info(qname: &str) -> QNameInfo {
  let element_name = qname.rsplit('/').next().unwrap_or(qname);
  let mut split = element_name.splitn(2, ':');
  let first = split.next().unwrap_or_default();
  let second = split.next();
  match second {
    Some(local_name) => QNameInfo {
      tag_prefix: first.to_string(),
      local_name: local_name.to_string(),
    },
    None => QNameInfo {
      tag_prefix: String::new(),
      local_name: first.to_string(),
    },
  }
}

fn has_struct_text_attr(attrs: &[Attribute]) -> bool {
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    if let Ok(metas) =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)
    {
      for meta in metas {
        if matches!(meta, Meta::Path(path) if path.is_ident("text")) {
          return true;
        }
      }
    }
  }
  false
}

fn has_struct_xml_header_attr(attrs: &[Attribute]) -> bool {
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    if let Ok(metas) =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)
    {
      for meta in metas {
        if matches!(meta, Meta::Path(path) if path.is_ident("xml_header")) {
          return true;
        }
      }
    }
  }
  false
}

fn is_namespace_field(ident: &Ident) -> bool {
  matches!(
    ident.to_string().as_str(),
    "xmlns" | "xmlns_map" | "mc_ignorable"
  )
}

fn is_option_type(ty: &Type) -> bool {
  matches!(ty, Type::Path(TypePath { path, .. }) if path.segments.last().is_some_and(|segment| segment.ident == "Option"))
}

fn is_vec_type(ty: &Type) -> bool {
  matches!(ty, Type::Path(TypePath { path, .. }) if path.segments.last().is_some_and(|segment| segment.ident == "Vec"))
}

fn is_box_type(ty: &Type) -> bool {
  matches!(ty, Type::Path(TypePath { path, .. }) if path.segments.last().is_some_and(|segment| segment.ident == "Box"))
}

fn contains_vec_type(ty: &Type) -> bool {
  if is_vec_type(ty) {
    return true;
  }
  if let Type::Path(TypePath { path, .. }) = ty
    && let Some(segment) = path.segments.last()
    && matches!(segment.ident.to_string().as_str(), "Option" | "Box")
    && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
    && let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first()
  {
    return contains_vec_type(inner_ty);
  }
  false
}

fn box_inner_type(ty: &Type) -> Option<Type> {
  if let Type::Path(TypePath { path, .. }) = ty
    && let Some(segment) = path.segments.last()
    && segment.ident == "Box"
    && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
    && let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first()
  {
    return Some(inner_ty.clone());
  }
  None
}

fn unwrap_wrapped_type(ty: &Type) -> Type {
  if let Type::Path(TypePath { path, .. }) = ty
    && let Some(segment) = path.segments.last()
    && matches!(segment.ident.to_string().as_str(), "Option" | "Vec")
    && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
    && let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first()
  {
    return unwrap_wrapped_type(inner_ty);
  }
  if let Some(inner_ty) = box_inner_type(ty) {
    return unwrap_wrapped_type(&inner_ty);
  }
  ty.clone()
}

fn unwrap_option_vec_type(ty: &Type) -> Type {
  if let Type::Path(TypePath { path, .. }) = ty
    && let Some(segment) = path.segments.last()
    && matches!(segment.ident.to_string().as_str(), "Option" | "Vec")
    && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
    && let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first()
  {
    return unwrap_option_vec_type(inner_ty);
  }
  ty.clone()
}

fn is_bool_type(ty: &Type) -> bool {
  matches!(ty, Type::Path(TypePath { path, .. }) if path.segments.last().is_some_and(|segment| {
    matches!(
      segment.ident.to_string().as_str(),
      "bool" | "BooleanValue" | "OnOffValue" | "TrueFalseBlankValue" | "TrueFalseValue"
    )
  }))
}

fn is_string_like_type(ty: &Type) -> bool {
  matches!(ty, Type::Path(TypePath { path, .. }) if path.segments.last().is_some_and(|segment| {
    matches!(
      segment.ident.to_string().as_str(),
      "String"
        | "StringValue"
        | "DateTimeValue"
        | "DecimalValue"
        | "HexBinaryValue"
        | "Base64BinaryValue"
        | "IntegerValue"
        | "SByteValue"
    )
  }))
}

fn choice_variant_payload_type(variant: &syn::Variant) -> syn::Result<Type> {
  match &variant.fields {
    Fields::Unnamed(fields) if fields.unnamed.len() == 1 => Ok(fields.unnamed[0].ty.clone()),
    _ => Err(syn::Error::new_spanned(
      variant,
      "SdkChoice only supports single-field tuple variants",
    )),
  }
}

fn choice_variant_inner_type(ty: &Type) -> proc_macro2::TokenStream {
  if let Some(inner_ty) = box_inner_type(ty) {
    quote! { #inner_ty }
  } else {
    quote! { #ty }
  }
}
