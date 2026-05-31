use super::*;

#[derive(Clone, Copy)]
enum DeserializeMode {
  Borrowed,
  Io,
}

impl DeserializeMode {
  fn deserialize_inner_ident(self) -> Ident {
    match self {
      Self::Borrowed => Ident::new("read_borrowed_inner", Span::call_site()),
      Self::Io => Ident::new("read_io_inner", Span::call_site()),
    }
  }

  fn tag_event_ty(self) -> proc_macro2::TokenStream {
    match self {
      Self::Borrowed => quote! { crate::common::SliceTagEvent },
      Self::Io => quote! { crate::common::IoTagEvent },
    }
  }

  fn read_raw_empty_xml_fn(self) -> proc_macro2::TokenStream {
    match self {
      Self::Borrowed => quote! { crate::common::read_raw_empty_xml_borrowed },
      Self::Io => quote! { crate::common::read_raw_empty_xml_io },
    }
  }

  fn read_raw_element_xml_fn(self) -> proc_macro2::TokenStream {
    match self {
      Self::Borrowed => quote! { crate::common::read_raw_element_xml_borrowed },
      Self::Io => quote! { crate::common::read_raw_element_xml_io },
    }
  }

  fn read_raw_empty_xml_bytes_fn(self) -> proc_macro2::TokenStream {
    match self {
      Self::Borrowed => quote! { crate::common::read_raw_empty_xml_borrowed_bytes },
      Self::Io => quote! { crate::common::read_raw_empty_xml_io_bytes },
    }
  }

  fn read_raw_element_xml_bytes_fn(self) -> proc_macro2::TokenStream {
    match self {
      Self::Borrowed => quote! { crate::common::read_raw_element_xml_borrowed_bytes },
      Self::Io => quote! { crate::common::read_raw_element_xml_io_bytes },
    }
  }
}

fn deserialize_type_inner_ident(mode: DeserializeMode) -> Ident {
  mode.deserialize_inner_ident()
}

fn is_canonical_xmlns_prefix_namespace(prefix: &str) -> bool {
  matches!(prefix, "x14" | "xne")
}

fn dispatch_field_qname_local_name(qname: &str) -> Option<String> {
  let QNameInfo { local_name, .. } = parse_qname_info(qname);
  if local_name.is_empty() {
    return None;
  }
  Some(local_name)
}

fn has_unique_local_name_dispatch<I>(qnames: I) -> bool
where
  I: IntoIterator,
  I::Item: AsRef<str>,
{
  let mut seen = std::collections::HashSet::new();
  for qname in qnames {
    let Some(local_name) = dispatch_field_qname_local_name(qname.as_ref()) else {
      return false;
    };
    if !seen.insert(local_name) {
      return false;
    }
  }
  true
}

fn should_use_local_name_child_dispatch(
  _schema_qname: &str,
  child_qnames: impl IntoIterator<Item = String>,
  has_choice_dispatch: bool,
  has_any_dispatch: bool,
  has_xml_other_children_dispatch: bool,
) -> bool {
  if has_choice_dispatch || has_any_dispatch || has_xml_other_children_dispatch {
    return false;
  }

  let child_qnames = child_qnames.into_iter().collect::<Vec<_>>();
  !child_qnames.is_empty() && has_unique_local_name_dispatch(child_qnames)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn local_name_child_dispatch_allows_any_prefix_when_unambiguous() {
    assert!(should_use_local_name_child_dispatch(
      "w:CT_DocVars/w:docVars",
      ["w:CT_DocVar/w:docVar".to_string()],
      false,
      false,
      false,
    ));
  }

  #[test]
  fn local_name_child_dispatch_rejects_same_parent_conflicts() {
    assert!(!should_use_local_name_child_dispatch(
      "w:CT_SomeParent/w:parent",
      ["w:CT_R/w:r".to_string(), "m:CT_R/m:r".to_string()],
      false,
      false,
      false,
    ));
  }

  #[test]
  fn local_name_child_dispatch_rejects_xml_other_children() {
    assert!(!should_use_local_name_child_dispatch(
      "w:CT_RPrList/w:rPr",
      ["w:CT_OnOff/w:shadow".to_string()],
      false,
      false,
      true,
    ));
  }
}

fn qname_match_targets(qnames: &[String]) -> Vec<proc_macro2::TokenStream> {
  let mut seen = std::collections::HashSet::new();
  let mut targets = Vec::new();

  for qname in qnames {
    let QNameInfo {
      tag_prefix,
      local_name,
    } = parse_qname_info(qname);
    if !tag_prefix.is_empty() {
      let prefixed = format!("{tag_prefix}:{local_name}");
      if seen.insert(prefixed.clone()) {
        let tag_qname_lit = LitByteStr::new(prefixed.as_bytes(), Span::call_site());
        targets.push(quote! { #tag_qname_lit });
      }
    }
    if seen.insert(local_name.to_string()) {
      let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
      targets.push(quote! { #local_name_lit });
    }
  }

  targets
}

fn qname_match_target_keys(qnames: &[String]) -> Vec<String> {
  let mut seen = std::collections::HashSet::new();
  let mut targets = Vec::new();

  for qname in qnames {
    let QNameInfo {
      tag_prefix,
      local_name,
    } = parse_qname_info(qname);
    if !tag_prefix.is_empty() {
      let prefixed = format!("{tag_prefix}:{local_name}");
      if seen.insert(prefixed.clone()) {
        targets.push(prefixed);
      }
    }
    if seen.insert(local_name.clone()) {
      targets.push(local_name);
    }
  }

  targets
}

fn build_choice_parse_attempt_tokens(
  _field_ident: &Ident,
  _choice_ty: &Type,
  repeated: bool,
  _mode: DeserializeMode,
  _as_result: bool,
  _xml_child_slot_assign: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  let message = if repeated {
    "repeated choice read path must be flattened by SdkType"
  } else {
    "choice read path must be flattened by SdkType"
  };
  quote! {
    compile_error!(#message);
  }
}

#[derive(Clone)]
struct GroupedChoiceAttempt {
  condition: Option<proc_macro2::TokenStream>,
  tokens: proc_macro2::TokenStream,
}

fn build_grouped_choice_match_tokens(
  grouped: &std::collections::BTreeMap<String, Vec<GroupedChoiceAttempt>>,
  as_result: bool,
) -> Vec<proc_macro2::TokenStream> {
  let miss_exit = if as_result {
    quote! { return Ok(true); }
  } else {
    quote! { continue; }
  };

  grouped
    .iter()
    .map(|(target, attempts)| {
      let target_lit = LitByteStr::new(target.as_bytes(), Span::call_site());
      if attempts.len() == 1 {
        let tokens = &attempts[0].tokens;
        quote! {
          #target_lit => {
            #tokens
            #miss_exit
          }
        }
      } else {
        let mut ordered_attempts = attempts.clone();
        ordered_attempts.sort_by_key(|attempt| {
          std::cmp::Reverse(
            attempt
              .condition
              .as_ref()
              .map(|tokens| tokens.to_string().len())
              .unwrap_or(0usize),
          )
        });
        let mut branch_tokens = quote! {};
        for attempt in ordered_attempts.into_iter().rev() {
          let tokens = attempt.tokens;
          branch_tokens = if let Some(condition) = attempt.condition {
            quote! {
              if #condition {
                #tokens
              } else {
                #branch_tokens
              }
            }
          } else {
            quote! {
              #tokens
            }
          };
        }
        quote! {
          #target_lit => {
            #branch_tokens
            #miss_exit
          }
        }
      }
    })
    .collect()
}

fn write_typed_child_tokens(
  child_ty: &syn::Type,
  value: proc_macro2::TokenStream,
  qname: &str,
) -> proc_macro2::TokenStream {
  let start_tag_open = write_start_tag_open_tokens(qname);
  quote! {
    #start_tag_open
    <#child_ty as crate::sdk::SdkType>::write_inner(#value, writer)?;
  }
}

fn write_text_value_content_tokens(
  value_expr: proc_macro2::TokenStream,
  value_ty: &syn::Type,
  simple_type: Option<&str>,
  qname: &str,
) -> proc_macro2::TokenStream {
  if let Some(kind) = simple_union_effective_type_kind(value_ty, simple_type) {
    write_simple_union_value_tokens(kind, value_expr)
  } else if effective_type_name(value_ty, simple_type)
    .as_deref()
    .is_some_and(is_xml_schema_float_type_name)
  {
    write_xml_schema_float_effective_tokens(value_expr, value_ty, simple_type, qname)
  } else if is_string_like_effective_type(value_ty, simple_type) {
    quote! {
      crate::common::write_escaped_content_str(writer, #value_expr.as_ref())?;
    }
  } else {
    quote! {
      crate::common::write_escaped_content_text(writer, #value_expr)?;
    }
  }
}

fn choice_sequence_child_write_tokens(
  child: &SdkTypeChoiceSequenceChild,
  value_expr: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
  let Some(field_ty) = child.ty.as_ref() else {
    return Err(syn::Error::new(
      Span::call_site(),
      "sdk choice sequence child write path requires ty",
    ));
  };
  let repeated = contains_vec_type(field_ty);
  let optional = is_option_type(field_ty);
  let payload_ty = unwrap_option_vec_type(field_ty);
  let child_ty = box_inner_type(&payload_ty).unwrap_or_else(|| payload_ty.clone());
  let qname = &child.qname;
  let tokens = match child.kind {
    SdkTypeChoiceSequenceChildKind::Child => {
      let child_write = write_typed_child_tokens(&child_ty, quote! { child }, qname);
      let value_write = write_typed_child_tokens(&child_ty, value_expr.clone(), qname);
      if repeated {
        quote! {
          for child in #value_expr {
            #child_write
          }
        }
      } else if optional {
        quote! {
          if let Some(child) = #value_expr {
            #child_write
          }
        }
      } else {
        value_write
      }
    }
    SdkTypeChoiceSequenceChildKind::EmptyChild => {
      let empty_tag = write_empty_tag_tokens(qname);
      let write_empty = quote! {
        #empty_tag
      };
      if repeated {
        quote! {
          for _ in #value_expr {
            #write_empty
          }
        }
      } else if optional {
        quote! {
          if #value_expr.is_some() {
            #write_empty
          }
        }
      } else {
        write_empty
      }
    }
    SdkTypeChoiceSequenceChildKind::TextChild => {
      let start_tag = write_start_tag_tokens(qname);
      let end_tag = write_end_tag_tokens(qname);
      let write_value = |expr: proc_macro2::TokenStream| {
        let content =
          write_text_value_content_tokens(expr, &child_ty, child.simple_type.as_deref(), qname);
        quote! {
          #start_tag
          #content
          #end_tag
        }
      };
      if repeated {
        let write_child = write_value(quote! { child });
        quote! {
          for child in #value_expr {
            #write_child
          }
        }
      } else if optional {
        let write_child = write_value(quote! { child });
        quote! {
          if let Some(child) = #value_expr {
            #write_child
          }
        }
      } else {
        write_value(value_expr)
      }
    }
    SdkTypeChoiceSequenceChildKind::AnyChild => {
      build_any_child_write_tokens_for_value(value_expr, qname, repeated, optional)
    }
  };
  Ok(tokens)
}

fn build_choice_write_tokens(
  choice_ty: &Type,
  items: &[SdkTypeChoiceItem],
  field_ident: &Ident,
  repeated: bool,
  optional: bool,
) -> syn::Result<proc_macro2::TokenStream> {
  let mut arms = Vec::new();
  for item in items {
    match item {
      SdkTypeChoiceItem::Child { variant, ty, qname } => {
        let _ = ty;
        let value_expr = quote! { value.as_ref() };
        let start_tag_open = write_start_tag_open_tokens(qname);
        arms.push(quote! {
          #choice_ty::#variant(value) => {
            #start_tag_open
            crate::sdk::SdkType::write_inner(#value_expr, writer)?;
          }
        });
      }
      SdkTypeChoiceItem::EmptyChild { variant, qname } => {
        let empty_tag = write_empty_tag_tokens(qname);
        arms.push(quote! {
          #choice_ty::#variant => {
            #empty_tag
          }
        });
      }
      SdkTypeChoiceItem::TextChild {
        variant,
        ty,
        simple_type,
        qname,
      } => {
        let start_tag = write_start_tag_tokens(qname);
        let end_tag = write_end_tag_tokens(qname);
        let content = if let Some(payload_ty) = ty.clone().or_else(|| {
          simple_type
            .as_deref()
            .and_then(|simple_type| syn::parse_str::<Type>(simple_type).ok())
        }) {
          write_text_value_content_tokens(
            quote! { value },
            &payload_ty,
            simple_type.as_deref(),
            qname,
          )
        } else {
          quote! {
            crate::common::write_escaped_content_text(writer, value)?;
          }
        };
        arms.push(quote! {
          #choice_ty::#variant(value) => {
            #start_tag
            #content
            #end_tag
          }
        });
      }
      SdkTypeChoiceItem::AnyChild { variant, qname } => {
        let start_tag = write_start_tag_tokens(qname);
        let end_tag = write_end_tag_tokens(qname);
        arms.push(quote! {
          #choice_ty::#variant(value) => {
            #start_tag
            for value in value {
              writer.write_all(value.as_bytes())?;
            }
            #end_tag
          }
        });
      }
      SdkTypeChoiceItem::Sequence { variant, children } => {
        if children.iter().all(|child| child.field.is_some()) {
          let field_idents = children
            .iter()
            .map(|child| child.field.as_ref().expect("sequence field"))
            .collect::<Vec<_>>();
          let write_tokens = children
            .iter()
            .map(|child| {
              let field = child.field.as_ref().expect("sequence field");
              choice_sequence_child_write_tokens(child, quote! { #field })
            })
            .collect::<syn::Result<Vec<_>>>()?;
          arms.push(quote! {
            #choice_ty::#variant { #( #field_idents ),* } => {
              #( #write_tokens )*
            }
          });
        } else {
          arms.push(quote! {
            #choice_ty::#variant(value) => {
              crate::sdk::SdkType::write_inner(value.as_ref(), writer)?;
            }
          });
        }
      }
      SdkTypeChoiceItem::Any { variant } => {
        arms.push(quote! {
          #choice_ty::#variant(value) => {
            writer.write_all(value.as_ref())?;
          }
        });
      }
      SdkTypeChoiceItem::Text { variant } => {
        arms.push(quote! {
          #choice_ty::#variant(value) => {
            crate::common::write_escaped_content_text(writer, value)?;
          }
        });
      }
    }
  }

  let write_one = quote! {
    match choice {
      #( #arms )*
    }
  };
  if repeated {
    Ok(quote! {
      for choice in &self.#field_ident {
        #write_one
      }
    })
  } else if optional {
    Ok(quote! {
      if let Some(choice) = &self.#field_ident {
        #write_one
      }
    })
  } else {
    Ok(quote! {
      let choice = &self.#field_ident;
      #write_one
    })
  }
}

fn build_flat_choice_dispatch_match_tokens(
  _field_ident: &Ident,
  _choice_ty: &Type,
  repeated: bool,
  qnames: &[String],
  xml_child_slot_assign: proc_macro2::TokenStream,
) -> (
  proc_macro2::TokenStream,
  proc_macro2::TokenStream,
  proc_macro2::TokenStream,
  proc_macro2::TokenStream,
) {
  let targets = qname_match_targets(qnames);

  if repeated {
    (
      quote! {
        #( #targets )|* => {
          compile_error!("repeated choice read path must be flattened by SdkType");
          #xml_child_slot_assign
          continue;
        }
      },
      quote! {
        #( #targets )|* => {
          compile_error!("repeated choice read path must be flattened by SdkType");
          #xml_child_slot_assign
          continue;
        }
      },
      quote! {
        #( #targets )|* => {
          compile_error!("repeated choice read path must be flattened by SdkType");
          #xml_child_slot_assign
          return Ok(true);
        }
      },
      quote! {
        #( #targets )|* => {
          compile_error!("repeated choice read path must be flattened by SdkType");
          #xml_child_slot_assign
          return Ok(true);
        }
      },
    )
  } else {
    (
      quote! {
        #( #targets )|* => {
          compile_error!("choice read path must be flattened by SdkType");
          #xml_child_slot_assign
          continue;
        }
      },
      quote! {
        #( #targets )|* => {
          compile_error!("choice read path must be flattened by SdkType");
          #xml_child_slot_assign
          continue;
        }
      },
      quote! {
        #( #targets )|* => {
          compile_error!("choice read path must be flattened by SdkType");
          #xml_child_slot_assign
          return Ok(true);
        }
      },
      quote! {
        #( #targets )|* => {
          compile_error!("choice read path must be flattened by SdkType");
          #xml_child_slot_assign
          return Ok(true);
        }
      },
    )
  }
}

pub(crate) fn expand_sdk_type(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
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
    Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
      if let Some(schema_qname) = parse_sdk_qname(&input.attrs)? {
        return expand_tuple_wrapper(input, &schema_qname, fields);
      }
      Err(syn::Error::new_spanned(
        fields,
        "tuple wrappers require an #[sdk(qname = \"...\")] attribute",
      ))
    }
    _ => Ok(quote! {
      impl #impl_generics crate::sdk::SdkType for #ident #type_generics #where_clause {}
      #[cfg(feature = "mce")]
      impl #impl_generics crate::sdk::SdkMce for #ident #type_generics #where_clause {}
      #[cfg(feature = "validators")]
      impl #impl_generics crate::validator::SdkValidator for #ident #type_generics #where_clause {}
    }),
  }
}

fn expand_tuple_wrapper(
  input: &DeriveInput,
  schema_qname: &str,
  fields: &syn::FieldsUnnamed,
) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
  let inner_ty = &fields
    .unnamed
    .first()
    .ok_or_else(|| syn::Error::new_spanned(fields, "tuple wrapper missing inner field"))?
    .ty;
  let QNameInfo {
    tag_prefix,
    local_name,
  } = parse_qname_info(schema_qname);
  if local_name.is_empty() {
    return Err(syn::Error::new_spanned(
      input,
      "tuple wrappers require a concrete element qname",
    ));
  }

  let tag_qname = if tag_prefix.is_empty() {
    local_name.clone()
  } else {
    format!("{tag_prefix}:{local_name}")
  };
  let tag_qname_lit = LitByteStr::new(tag_qname.as_bytes(), Span::call_site());
  let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
  let start_tag_open = write_start_tag_open_tokens(schema_qname);
  let end_tag = write_end_tag_tokens(schema_qname);

  Ok(quote! {
    impl #impl_generics crate::sdk::SdkType for #ident #type_generics #where_clause {
      fn read_borrowed_inner<'de>(
        xml_reader: &mut crate::common::SliceReader<'de>,
        start: quick_xml::events::BytesStart<'de>,
        empty: bool,
      ) -> Result<Self, crate::common::SdkError> {
        <#inner_ty as crate::sdk::SdkType>::read_borrowed_inner(xml_reader, start, empty).map(Self)
      }

      fn read_io_inner<R: std::io::BufRead>(
        xml_reader: &mut crate::common::IoReader<R>,
        start: quick_xml::events::BytesStart<'static>,
        empty: bool,
      ) -> Result<Self, crate::common::SdkError> {
        <#inner_ty as crate::sdk::SdkType>::read_io_inner(xml_reader, start, empty).map(Self)
      }

      fn write_inner<W: std::io::Write>(
        &self,
        writer: &mut W,
      ) -> Result<(), std::io::Error> {
        <#inner_ty as crate::sdk::SdkType>::write_inner(&self.0, writer)?;
        #end_tag
        Ok(())
      }
    }

    #[cfg(feature = "validators")]
    impl #impl_generics crate::validator::SdkValidator for #ident #type_generics #where_clause {
      fn validate_into(&self, context: &mut crate::validator::ValidationContext) {
        <#inner_ty as crate::validator::SdkValidator>::validate_into(&self.0, context);
      }
    }

    #[cfg(feature = "mce")]
    impl #impl_generics crate::sdk::SdkMce for #ident #type_generics #where_clause {
      fn process_mce_with_context(
        &mut self,
        settings: &crate::sdk::MarkupCompatibilityProcessSettings,
        context: &mut crate::sdk::MceContext,
      ) -> Result<(), crate::common::SdkError> {
        <#inner_ty as crate::sdk::SdkMce>::process_mce_with_context(&mut self.0, settings, context)
      }
    }

    impl #impl_generics std::ops::Deref for #ident #type_generics #where_clause {
      type Target = #inner_ty;

      #[inline]
      fn deref(&self) -> &Self::Target {
        &self.0
      }
    }

    impl #impl_generics std::ops::DerefMut for #ident #type_generics #where_clause {
      #[inline]
      fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
      }
    }

    impl #impl_generics From<#inner_ty> for #ident #type_generics #where_clause {
      #[inline]
      fn from(value: #inner_ty) -> Self {
        Self(value)
      }
    }

    impl #impl_generics std::str::FromStr for #ident #type_generics #where_clause {
      type Err = crate::common::SdkError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut xml_reader = crate::common::from_str_inner(s)?;
        let (start, empty) = crate::common::read_root_start_borrowed_no_header(
          &mut xml_reader,
          stringify!(#ident),
          #tag_qname_lit,
          #local_name_lit,
        )?;
        <Self as crate::sdk::SdkType>::read_borrowed_inner(&mut xml_reader, start, empty)
      }
    }

    impl #impl_generics #ident #type_generics #where_clause {
      pub fn from_bytes(bytes: &[u8]) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_bytes_inner(bytes)?;
        let (start, empty) = crate::common::read_root_start_borrowed_no_header(
          &mut xml_reader,
          stringify!(#ident),
          #tag_qname_lit,
          #local_name_lit,
        )?;
        <Self as crate::sdk::SdkType>::read_borrowed_inner(&mut xml_reader, start, empty)
      }

      pub fn from_reader<R: std::io::BufRead>(
        reader: R,
      ) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_reader_inner(reader)?;
        let (start, empty) = crate::common::read_root_start_io_no_header(
          &mut xml_reader,
          stringify!(#ident),
          #tag_qname_lit,
          #local_name_lit,
        )?;
        <Self as crate::sdk::SdkType>::read_io_inner(&mut xml_reader, start, empty)
      }

      pub fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
        #start_tag_open
        <Self as crate::sdk::SdkType>::write_inner(self, writer)?;
        Ok(())
      }

      pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut writer = Vec::with_capacity(32);
        self.write_to(&mut writer)?;
        Ok(writer)
      }

      pub fn to_xml(&self) -> Result<String, crate::common::SdkError> {
        String::from_utf8(self.to_bytes()?).map_err(|err| {
          crate::common::SdkError::CommonError(format!("invalid utf-8 xml: {err}"))
        })
      }
    }

    impl #impl_generics ::std::fmt::Display for #ident #type_generics #where_clause {
      fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(&self.to_xml().map_err(|_| ::std::fmt::Error)?)
      }
    }
  })
}

fn mce_process_child_field_tokens(field: &SdkChildField) -> proc_macro2::TokenStream {
  let ident = &field.ident;
  if field.repeated {
    quote! {
      for child in &mut self.#ident {
        crate::sdk::SdkMce::process_mce_with_context(child, settings, context)?;
      }
    }
  } else if field.optional {
    quote! {
      if let Some(child) = &mut self.#ident {
        crate::sdk::SdkMce::process_mce_with_context(child, settings, context)?;
      }
    }
  } else {
    quote! {
      crate::sdk::SdkMce::process_mce_with_context(&mut self.#ident, settings, context)?;
    }
  }
}

fn mce_choice_impl_tokens(field: &SdkTypeChoiceField) -> proc_macro2::TokenStream {
  let ident = &field.ident;
  let choice_ty = unwrap_option_vec_type(&field.ty);
  let mut choice_arms = Vec::new();
  let mut any_variants = Vec::new();

  for item in &field.items {
    match item {
      SdkTypeChoiceItem::Child { variant, .. } => {
        choice_arms.push(quote! {
          #choice_ty::#variant(value) => {
            crate::sdk::SdkMce::process_mce_with_context(value, settings, context)?;
          }
        });
      }
      SdkTypeChoiceItem::Sequence { variant, children } => {
        if children.iter().all(|child| child.field.is_some()) {
          let pattern_fields = children
            .iter()
            .map(|child| {
              let field_ident = child.field.as_ref().expect("sequence field");
              if matches!(child.kind, SdkTypeChoiceSequenceChildKind::Child) {
                quote! { #field_ident }
              } else {
                quote! { #field_ident: _ }
              }
            })
            .collect::<Vec<_>>();
          let process_tokens = children
            .iter()
            .filter(|child| matches!(child.kind, SdkTypeChoiceSequenceChildKind::Child))
            .map(|child| {
              let field_ident = child.field.as_ref().expect("sequence field");
              quote! {
                crate::sdk::SdkMce::process_mce_with_context(#field_ident, settings, context)?;
              }
            })
            .collect::<Vec<_>>();
          choice_arms.push(quote! {
            #choice_ty::#variant { #( #pattern_fields ),* } => {
              #( #process_tokens )*
            }
          });
        } else {
          choice_arms.push(quote! {
            #choice_ty::#variant(value) => {
              crate::sdk::SdkMce::process_mce_with_context(value, settings, context)?;
            }
          });
        }
      }
      SdkTypeChoiceItem::EmptyChild { .. }
      | SdkTypeChoiceItem::TextChild { .. }
      | SdkTypeChoiceItem::AnyChild { .. }
      | SdkTypeChoiceItem::Text { .. } => {}
      SdkTypeChoiceItem::Any { variant } => {
        any_variants.push(variant);
      }
    }
  }

  let process_choice_for = |choice_expr: proc_macro2::TokenStream| {
    quote! {
      match #choice_expr {
        #( #choice_arms )*
        _ => {}
      }
    }
  };
  let process_self_choice = process_choice_for(quote! { self });

  let mut parse_replacement_arms = Vec::new();
  for item in &field.items {
    match item {
      SdkTypeChoiceItem::Child { variant, qname, .. } => {
        let targets = qname_match_targets(std::slice::from_ref(qname));
        parse_replacement_arms.push(quote! {
          #( #targets )|* => {
            Some(#choice_ty::#variant(std::boxed::Box::new(
              <_ as crate::sdk::SdkType>::read_borrowed_inner(&mut child_reader, e, next_empty)?
            )))
          }
        });
      }
      SdkTypeChoiceItem::EmptyChild { variant, qname } => {
        let targets = qname_match_targets(std::slice::from_ref(qname));
        let child_reader_ident = Ident::new("child_reader", Span::call_site());
        let skip_tokens = build_empty_child_skip_tokens(
          ident,
          qname,
          DeserializeMode::Borrowed,
          &child_reader_ident,
        );
        parse_replacement_arms.push(quote! {
          #( #targets )|* => {
            #skip_tokens
            Some(#choice_ty::#variant)
          }
        });
      }
      SdkTypeChoiceItem::TextChild { variant, qname, .. } => {
        let targets = qname_match_targets(std::slice::from_ref(qname));
        parse_replacement_arms.push(quote! {
          #( #targets )|* => {
            let parsed_child = if next_empty {
              crate::common::parse_value::<_>("", stringify!(#ident), stringify!(#variant))?
            } else {
              let value = child_reader.read_text(e.name())?;
              crate::common::parse_value::<_>(
                value.as_ref(),
                stringify!(#ident),
                stringify!(#variant),
              )?
            };
            Some(#choice_ty::#variant(parsed_child))
          }
        });
      }
      SdkTypeChoiceItem::AnyChild { variant, qname } => {
        let targets = qname_match_targets(std::slice::from_ref(qname));
        let QNameInfo {
          tag_prefix,
          local_name,
        } = parse_qname_info(qname);
        let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
        let tag_qname_lit = LitByteStr::new(
          format!("{tag_prefix}:{local_name}").as_bytes(),
          Span::call_site(),
        );
        parse_replacement_arms.push(quote! {
          #( #targets )|* => {
            let mut parsed_child = Vec::new();
            if !next_empty {
              loop {
                match child_reader.next()? {
                  quick_xml::events::Event::Start(e) => {
                    let xml = crate::common::read_raw_element_xml_borrowed(&mut child_reader, e)?;
                    parsed_child.push(xml);
                  }
                  quick_xml::events::Event::Empty(e) => {
                    let xml = crate::common::read_raw_empty_xml_borrowed(e)?;
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
            Some(#choice_ty::#variant(parsed_child.into()))
          }
        });
      }
      SdkTypeChoiceItem::Sequence { .. }
      | SdkTypeChoiceItem::Any { .. }
      | SdkTypeChoiceItem::Text { .. } => {}
    }
  }

  let mut mce_any_arms = Vec::new();
  let mut mce_replacement_read_methods = Vec::new();
  for variant in any_variants {
    let method_ident = Ident::new(
      &format!(
        "__ooxmlsdk_read_mce_replacement_child_{}",
        variant.to_string().to_ascii_lowercase()
      ),
      Span::call_site(),
    );
    mce_replacement_read_methods.push(quote! {
      fn #method_ident(
        child_xml: std::boxed::Box<[u8]>,
      ) -> Result<Self, crate::common::SdkError> {
        let parsed = {
          let mut child_reader = crate::common::from_bytes_inner(child_xml.as_ref())?;
          loop {
            match child_reader.next()? {
              quick_xml::events::Event::Start(e) => {
                let next_empty = false;
                let event_name = e.name().into_inner();
                break match event_name {
                  #( #parse_replacement_arms )*
                  _ => None,
                };
              }
              quick_xml::events::Event::Empty(e) => {
                let next_empty = true;
                let event_name = e.name().into_inner();
                break match event_name {
                  #( #parse_replacement_arms )*
                  _ => None,
                };
              }
              quick_xml::events::Event::Eof => {
                return Err(crate::common::unexpected_eof(stringify!(#ident)));
              }
              _ => {}
            }
          }
        };
        Ok(parsed.unwrap_or_else(|| Self::#variant(child_xml)))
      }
    });
    mce_any_arms.push(quote! {
      #choice_ty::#variant(xml) => {
        if let Some(children) = crate::common::mce_choice_replacement_child_bytes(
          xml.as_ref(),
          settings,
          context,
        )? {
          for child_xml in children {
            let mut child = Self::#method_ident(child_xml)?;
            Self::process_mce_choice_with_context(&mut child, settings, context)?;
            processed.push(child);
          }
        } else {
          Self::process_mce_choice_with_context(&mut item, settings, context)?;
          processed.push(item);
        }
      }
    });
  }

  let process_repeated_choices = if mce_any_arms.is_empty() {
    quote! {
      for choice in values {
        Self::process_mce_choice_with_context(choice, settings, context)?;
      }
    }
  } else {
    quote! {
      let mut processed = Vec::with_capacity(values.len());
      for mut item in std::mem::take(values) {
        match &mut item {
          #( #mce_any_arms )*
          _ => {
            Self::process_mce_choice_with_context(&mut item, settings, context)?;
            processed.push(item);
          }
        }
      }
      *values = processed;
    }
  };

  quote! {
    #[cfg(feature = "mce")]
    impl #choice_ty {
      #( #mce_replacement_read_methods )*

      fn process_mce_choice_with_context(
        &mut self,
        settings: &crate::sdk::MarkupCompatibilityProcessSettings,
        context: &mut crate::sdk::MceContext,
      ) -> Result<(), crate::common::SdkError> {
        #process_self_choice
        Ok(())
      }

      fn process_mce_choices_with_context(
        values: &mut Vec<Self>,
        settings: &crate::sdk::MarkupCompatibilityProcessSettings,
        context: &mut crate::sdk::MceContext,
      ) -> Result<(), crate::common::SdkError> {
        #process_repeated_choices
        Ok(())
      }
    }
  }
}

fn mce_process_choice_field_tokens(field: &SdkTypeChoiceField) -> proc_macro2::TokenStream {
  let ident = &field.ident;
  let choice_ty = unwrap_option_vec_type(&field.ty);
  if field.repeated {
    quote! {
      #choice_ty::process_mce_choices_with_context(&mut self.#ident, settings, context)?;
    }
  } else if field.optional {
    quote! {
      if let Some(choice) = &mut self.#ident {
        #choice_ty::process_mce_choice_with_context(choice, settings, context)?;
      }
    }
  } else {
    quote! {
      #choice_ty::process_mce_choice_with_context(&mut self.#ident, settings, context)?;
    }
  }
}

fn mce_xml_other_children_process_tokens(
  has_xml_other_children_field: bool,
  compact_xml_other_children: bool,
) -> proc_macro2::TokenStream {
  if !has_xml_other_children_field {
    return quote! {};
  }

  if compact_xml_other_children {
    return quote! {
      let mut xml_other_children = Vec::with_capacity(self.xml_other_children.len());
      for xml in std::mem::take(&mut self.xml_other_children) {
        if let Some(children) = crate::common::mce_choice_replacement_child_bytes(
          xml.as_ref(),
          settings,
          context,
        )? {
          xml_other_children.extend(children);
        } else {
          xml_other_children.push(xml);
        }
      }
      self.xml_other_children = xml_other_children;
    };
  }

  quote! {
    let mut xml_other_children = Vec::with_capacity(self.xml_other_children.len());
    for (slot, xml) in std::mem::take(&mut self.xml_other_children) {
      if let Some(children) = crate::common::mce_choice_replacement_child_bytes(
        xml.as_ref(),
        settings,
        context,
      )? {
        xml_other_children.extend(children.into_iter().map(|child| (slot, child)));
      } else {
        xml_other_children.push((slot, xml));
      }
    }
    self.xml_other_children = xml_other_children;
  }
}

fn mce_context_scope_tokens(
  xmlns_fields: &[Ident],
  xml_other_attrs_field: Option<&Ident>,
) -> (
  proc_macro2::TokenStream,
  proc_macro2::TokenStream,
  proc_macro2::TokenStream,
) {
  let xmlns_expr = if let Some(ident) = xmlns_fields.first() {
    quote! { self.#ident.as_slice() }
  } else {
    quote! { &[] as &[crate::common::XmlNamespace] }
  };
  let attrs_expr = if let Some(ident) = xml_other_attrs_field {
    quote! { self.#ident.as_slice() }
  } else {
    quote! { &[] as &[crate::common::XmlOtherAttr] }
  };

  if xmlns_fields.is_empty() && xml_other_attrs_field.is_none() {
    (quote! {}, quote! {}, quote! {})
  } else {
    let process_attrs_tokens = if let Some(ident) = xml_other_attrs_field {
      quote! {
        self
          .#ident
          .retain(|attr| !context.should_remove_ignorable_attribute(attr.name()));
      }
    } else {
      quote! {}
    };
    (
      quote! {
        let __mce_checkpoint = context.push(#xmlns_expr, #attrs_expr, settings)?;
      },
      process_attrs_tokens,
      quote! {
        context.pop(__mce_checkpoint);
      },
    )
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
      {
        static VALIDATOR_REGEX: std::sync::LazyLock<regex::Regex> =
          std::sync::LazyLock::new(|| {
            regex::Regex::new(concat!(r"\A(?:", #regex, r")\z"))
              .expect("generated validator regex must compile")
          });
        crate::validator::validate_pattern_regex(
          stringify!(#ident),
          stringify!(#field_ident),
          value,
          &VALIDATOR_REGEX,
        )?;
      }
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
      let format_tokens = if is_hex_binary_type(parse_ty)
        || matches!(
          validator,
          SdkFieldValidator::StringLength {
            type_name: Some(type_name),
            ..
          } if type_name == "w:ST_HexColorRGB"
        ) {
        quote! {
          crate::validator::validate_binary_format(
            stringify!(#ident),
            stringify!(#field_ident),
            value,
            crate::validator::BinaryFormatKind::Hex,
          )?;
        }
      } else {
        quote! {}
      };
      quote! {
        #format_tokens
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
      let range_tokens = number_range_runtime_bounds(min, max, *min_inclusive, *max_inclusive);
      quote! {
        crate::validator::validate_number_range(
          stringify!(#ident),
          stringify!(#field_ident),
          value,
          #range_tokens,
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

fn number_range_runtime_bounds(
  min: &Option<String>,
  max: &Option<String>,
  min_inclusive: bool,
  max_inclusive: bool,
) -> proc_macro2::TokenStream {
  let min_tokens = min.as_ref().map(|min| number_range_f64_literal(min));
  let max_tokens = max.as_ref().map(|max| number_range_f64_literal(max));
  let start = match (min_tokens, min_inclusive) {
    (Some(min), true) => quote! { std::ops::Bound::Included(#min) },
    (Some(min), false) => quote! { std::ops::Bound::Excluded(#min) },
    (None, _) => quote! { std::ops::Bound::Unbounded },
  };
  let end = match (max_tokens, max_inclusive) {
    (Some(max), true) => quote! { std::ops::Bound::Included(#max) },
    (Some(max), false) => quote! { std::ops::Bound::Excluded(#max) },
    (None, _) => quote! { std::ops::Bound::Unbounded },
  };
  quote! { (#start, #end) }
}

fn number_range_f64_literal(value: &str) -> proc_macro2::TokenStream {
  format!("{value}f64")
    .parse()
    .unwrap_or_else(|_| quote! { #value.parse::<f64>().expect("numeric validator bound") })
}

fn text_child_match_target(qname: &str) -> proc_macro2::TokenStream {
  text_child_match_target_with_local_name_dispatch(qname, false)
}

fn text_child_match_target_with_local_name_dispatch(
  qname: &str,
  use_local_name_dispatch: bool,
) -> proc_macro2::TokenStream {
  let QNameInfo {
    tag_prefix,
    local_name,
  } = parse_qname_info(qname);
  let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
  if use_local_name_dispatch || tag_prefix.is_empty() {
    quote! { #local_name_lit }
  } else {
    let tag_qname_lit = LitByteStr::new(
      format!("{tag_prefix}:{local_name}").as_bytes(),
      Span::call_site(),
    );
    quote! { #tag_qname_lit | #local_name_lit }
  }
}

struct TextChildParseArmOptions {
  repeated: bool,
  as_result: bool,
  use_local_name_dispatch: bool,
  list: bool,
}

fn parse_simple_union_attr_tokens(kind: SimpleUnionTypeKind) -> proc_macro2::TokenStream {
  match kind {
    SimpleUnionTypeKind::TwipsMeasure => {
      quote! { crate::common::parse_twips_measure_attr(&attr, decoder)? }
    }
    SimpleUnionTypeKind::SignedTwipsMeasure => {
      quote! { crate::common::parse_signed_twips_measure_attr(&attr, decoder)? }
    }
    SimpleUnionTypeKind::DecimalNumberOrPercent => {
      quote! { crate::common::parse_decimal_number_or_percent_attr(&attr, decoder)? }
    }
    SimpleUnionTypeKind::MeasurementOrPercent => {
      quote! { crate::common::parse_measurement_or_percent_attr(&attr, decoder)? }
    }
  }
}

fn write_simple_union_attr_tokens(
  kind: SimpleUnionTypeKind,
  attr_name: &LitStr,
  value_expr: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  match kind {
    SimpleUnionTypeKind::TwipsMeasure => {
      quote! { crate::common::write_twips_measure_attr(writer, #attr_name, #value_expr)?; }
    }
    SimpleUnionTypeKind::SignedTwipsMeasure => {
      quote! { crate::common::write_signed_twips_measure_attr(writer, #attr_name, #value_expr)?; }
    }
    SimpleUnionTypeKind::DecimalNumberOrPercent => {
      quote! { crate::common::write_decimal_number_or_percent_attr(writer, #attr_name, #value_expr)?; }
    }
    SimpleUnionTypeKind::MeasurementOrPercent => {
      quote! { crate::common::write_measurement_or_percent_attr(writer, #attr_name, #value_expr)?; }
    }
  }
}

fn build_text_child_parse_arm(
  owner_ident: &Ident,
  field_ident: &Ident,
  qname: &str,
  simple_type: Option<&str>,
  field_ty: &Type,
  xml_child_slot_assign: proc_macro2::TokenStream,
  options: TextChildParseArmOptions,
) -> proc_macro2::TokenStream {
  let target =
    text_child_match_target_with_local_name_dispatch(qname, options.use_local_name_dispatch);
  let value_ty = if options.list {
    vec_inner_type(&unwrap_option_type(field_ty))
      .unwrap_or_else(|| unwrap_option_vec_type(field_ty))
  } else {
    unwrap_option_vec_type(field_ty)
  };
  let simple_union_kind = simple_union_effective_type_kind(&value_ty, simple_type);
  let parse_from_text_tokens = if options.list {
    quote! {{
      crate::common::parse_list_value::<#value_ty>(
        text.as_ref(),
        stringify!(#owner_ident),
        stringify!(#field_ident),
      )?
    }}
  } else if let Some(kind) = simple_union_kind {
    let parse_tokens = parse_simple_union_value_tokens(kind, quote! { value });
    quote! {{
      let value = text.into_owned();
      #parse_tokens
    }}
  } else if is_string_like_effective_type(&value_ty, simple_type) {
    quote! { text.into_owned() }
  } else {
    quote! {{
      crate::common::parse_value::<#value_ty>(
        text.as_ref(),
        stringify!(#owner_ident),
        stringify!(#field_ident),
      )?
    }}
  };
  let empty_value_tokens = if options.list {
    quote! {
      crate::common::parse_list_value::<#value_ty>(
        "",
        stringify!(#owner_ident),
        stringify!(#field_ident),
      )?
    }
  } else if let Some(kind) = simple_union_kind {
    parse_simple_union_value_tokens(kind, quote! { String::new() })
  } else if is_string_like_effective_type(&value_ty, simple_type) {
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
  let assign_tokens = if options.repeated {
    quote! { #field_ident.push(parsed_child); }
  } else {
    quote! { #field_ident = Some(parsed_child); }
  };
  let finish_tokens = if options.as_result {
    quote! { Ok(true) }
  } else {
    quote! { true }
  };

  quote! {
    #target => {
      let parsed_child = if next_empty {
        #empty_value_tokens
      } else {
        let text = xml_reader.read_text(e.name())?;
        #parse_from_text_tokens
      };
      #assign_tokens
      #xml_child_slot_assign
      #finish_tokens
    },
  }
}

fn build_text_child_write_tokens(
  field_ident: &Ident,
  qname: &str,
  simple_type: Option<&str>,
  field_ty: &Type,
  repeated: bool,
  optional: bool,
  list: bool,
) -> proc_macro2::TokenStream {
  let inner_ty = if list {
    vec_inner_type(&unwrap_option_type(field_ty)).unwrap_or_else(|| unwrap_wrapped_type(field_ty))
  } else {
    unwrap_wrapped_type(field_ty)
  };
  let write_value_tokens = |value_expr: proc_macro2::TokenStream| {
    let start_tag = write_start_tag_tokens(qname);
    let end_tag = write_end_tag_tokens(qname);
    let value_write_tokens = if list {
      quote! {
        crate::common::write_list_text_content_value(writer, #value_expr.as_slice())?;
      }
    } else if let Some(kind) = simple_union_effective_type_kind(&inner_ty, simple_type) {
      write_simple_union_value_tokens(kind, value_expr.clone())
    } else if effective_type_name(&inner_ty, simple_type)
      .as_deref()
      .is_some_and(is_xml_schema_float_type_name)
    {
      write_xml_schema_float_effective_tokens(value_expr.clone(), &inner_ty, simple_type, qname)
    } else if is_string_like_effective_type(&inner_ty, simple_type) {
      quote! {
        crate::common::write_escaped_content_str(writer, #value_expr.as_ref())?;
      }
    } else {
      quote! {
        crate::common::write_escaped_content_text(writer, #value_expr)?;
      }
    };
    quote! {
      #start_tag
      #value_write_tokens
      #end_tag
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

fn build_empty_child_write_tokens(
  field_ident: &Ident,
  qname: &str,
  repeated: bool,
  optional: bool,
) -> proc_macro2::TokenStream {
  let empty_tag = write_empty_tag_tokens(qname);
  let write_tokens = quote! {
    #empty_tag
  };

  if repeated {
    quote! {
      for _ in &self.#field_ident {
        #write_tokens
      }
    }
  } else if optional {
    quote! {
      if self.#field_ident.is_some() {
        #write_tokens
      }
    }
  } else {
    quote! {
      #write_tokens
    }
  }
}

fn build_empty_child_skip_tokens(
  owner_ident: &Ident,
  qname: &str,
  mode: DeserializeMode,
  reader_ident: &Ident,
) -> proc_macro2::TokenStream {
  let QNameInfo { local_name, .. } = parse_qname_info(qname);
  let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
  let tag_qname_lit = LitByteStr::new(qname.as_bytes(), Span::call_site());
  let next_tag_event = mode.tag_event_ty();

  quote! {
    if !next_empty {
      loop {
        match #reader_ident.next_tag_event()? {
          #next_tag_event::Start(e, _) => {
            let event_name = e.name().into_inner();
            return Err(crate::common::unexpected_tag(
              stringify!(#owner_ident),
              "empty child",
              event_name,
            ));
          }
          #next_tag_event::End(e) => {
            if e.name().as_ref() == #tag_qname_lit || e.name().as_ref() == #local_name_lit {
              break;
            }
          }
          #next_tag_event::Eof => {
            return Err(crate::common::unexpected_eof(stringify!(#owner_ident)));
          }
          #next_tag_event::Decl(_) | #next_tag_event::Other => {}
        }
      }
    }
  }
}

fn build_any_child_parse_arm(
  owner_ident: &Ident,
  field_ident: &Ident,
  qname: &str,
  repeated: bool,
  as_result: bool,
  mode: DeserializeMode,
  xml_child_slot_assign: proc_macro2::TokenStream,
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
  let read_raw_empty_xml = mode.read_raw_empty_xml_fn();
  let read_raw_element_xml = mode.read_raw_element_xml_fn();
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
      let mut parsed_child = Vec::new();
      if !next_empty {
        loop {
          match xml_reader.next()? {
            quick_xml::events::Event::Start(e) => {
              let xml = #read_raw_element_xml(xml_reader, e)?;
              parsed_child.push(xml);
            }
            quick_xml::events::Event::Empty(e) => {
              let xml = #read_raw_empty_xml(e)?;
              parsed_child.push(xml);
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
      }
      #assign_tokens
      #xml_child_slot_assign
      #finish_tokens
    },
  }
}

fn build_any_child_write_tokens(
  field_ident: &Ident,
  qname: &str,
  repeated: bool,
  optional: bool,
) -> proc_macro2::TokenStream {
  build_any_child_write_tokens_for_value(quote! { &self.#field_ident }, qname, repeated, optional)
}

fn build_any_child_write_tokens_for_value(
  value_expr: proc_macro2::TokenStream,
  qname: &str,
  repeated: bool,
  optional: bool,
) -> proc_macro2::TokenStream {
  let start_tag = write_start_tag_tokens(qname);
  let end_tag = write_end_tag_tokens(qname);
  let write_value_tokens = quote! {
    #start_tag
    for value in value {
      writer.write_all(value.as_bytes())?;
    }
    #end_tag
  };

  if repeated {
    quote! {
      for value in #value_expr {
        #write_value_tokens
      }
    }
  } else if optional {
    quote! {
      if let Some(value) = #value_expr {
        #write_value_tokens
      }
    }
  } else {
    quote! {
      let value = #value_expr;
      #write_value_tokens
    }
  }
}

fn build_any_child_parse_tokens(
  field_ident: &Ident,
  field_ty: &Type,
  repeated: bool,
  mode: DeserializeMode,
  as_result: bool,
  xml_child_slot_assign: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  let field_item_ty = unwrap_option_vec_type(field_ty);
  let use_bytes = is_box_u8_slice_type(&field_item_ty);
  let read_raw_empty_xml = if use_bytes {
    mode.read_raw_empty_xml_bytes_fn()
  } else {
    mode.read_raw_empty_xml_fn()
  };
  let read_raw_element_xml = if use_bytes {
    mode.read_raw_element_xml_bytes_fn()
  } else {
    mode.read_raw_element_xml_fn()
  };
  let value_expr = if is_box_str_type(&field_item_ty) {
    quote! { xml.into_boxed_str() }
  } else {
    quote! { xml }
  };
  let assign = if repeated {
    quote! { #field_ident.push(#value_expr); }
  } else {
    quote! { #field_ident = Some(#value_expr); }
  };
  let tail = if as_result {
    quote! { return Ok(true); }
  } else {
    quote! { continue; }
  };

  quote! {
    if !matched {
      let xml = if next_empty {
        #read_raw_empty_xml(e)?
      } else {
        #read_raw_element_xml(xml_reader, e)?
      };
      #assign
      #xml_child_slot_assign
      #tail
    }
  }
}

fn build_pure_any_child_parse_tokens(
  field_ident: &Ident,
  field_ty: &Type,
  repeated: bool,
  mode: DeserializeMode,
) -> proc_macro2::TokenStream {
  let field_item_ty = unwrap_option_vec_type(field_ty);
  let use_bytes = is_box_u8_slice_type(&field_item_ty);
  let read_raw_empty_xml = if use_bytes {
    mode.read_raw_empty_xml_bytes_fn()
  } else {
    mode.read_raw_empty_xml_fn()
  };
  let read_raw_element_xml = if use_bytes {
    mode.read_raw_element_xml_bytes_fn()
  } else {
    mode.read_raw_element_xml_fn()
  };
  let value_expr = if is_box_str_type(&field_item_ty) {
    quote! { xml.into_boxed_str() }
  } else {
    quote! { xml }
  };
  let assign = if repeated {
    quote! { #field_ident.push(#value_expr); }
  } else {
    quote! { #field_ident = Some(#value_expr); }
  };

  quote! {
    let xml = if next_empty {
      #read_raw_empty_xml(e)?
    } else {
      #read_raw_element_xml(xml_reader, e)?
    };
    #assign
    continue;
  }
}

fn build_unmatched_child_tokens(
  owner_ident: &Ident,
  mode: DeserializeMode,
  has_xml_other_children_field: bool,
  compact_xml_other_children: bool,
) -> proc_macro2::TokenStream {
  if has_xml_other_children_field {
    let read_raw_empty_xml = mode.read_raw_empty_xml_bytes_fn();
    let read_raw_element_xml = mode.read_raw_element_xml_bytes_fn();
    if compact_xml_other_children {
      return quote! {
        let xml = if next_empty {
          #read_raw_empty_xml(e)?
        } else {
          #read_raw_element_xml(xml_reader, e)?
        };
        xml_other_children.push(xml);
        continue;
      };
    }
    return quote! {
      let xml = if next_empty {
        #read_raw_empty_xml(e)?
      } else {
        #read_raw_element_xml(xml_reader, e)?
      };
      xml_other_children.push((__xml_child_slot, xml));
      continue;
    };
  }

  quote! {
    Err(crate::common::unexpected_tag(
      stringify!(#owner_ident),
      "known child",
      event_name,
    ))?;
  }
}

fn expand_helper_struct(
  input: &DeriveInput,
  fields: &syn::FieldsNamed,
) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &input.ident;
  let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
  let read_borrowed_inner_ident = deserialize_type_inner_ident(DeserializeMode::Borrowed);
  let read_io_inner_ident = deserialize_type_inner_ident(DeserializeMode::Io);

  let mut child_fields = Vec::new();
  let mut empty_child_fields = Vec::new();
  let mut text_child_fields = Vec::new();
  let mut any_child_fields = Vec::new();
  let mut choice_fields = Vec::new();
  for field in &fields.named {
    let field_ident = field
      .ident
      .as_ref()
      .ok_or_else(|| syn::Error::new_spanned(field, "SdkType requires named fields"))?;
    let parsed_attrs = parse_sdk_type_field_attrs(&field.attrs)?;
    match parsed_attrs.kind {
      Some(SdkTypeFieldKind::Child { qname, .. }) => child_fields.push(SdkChildField {
        ident: field_ident.clone(),
        qname,
        ty: field.ty.clone(),
        optional: is_option_type(&field.ty),
        repeated: contains_vec_type(&field.ty),
      }),
      Some(SdkTypeFieldKind::EmptyChild { qname }) => empty_child_fields.push(SdkEmptyChildField {
        ident: field_ident.clone(),
        qname,
        optional: is_option_type(&field.ty),
        repeated: contains_vec_type(&field.ty),
      }),
      Some(SdkTypeFieldKind::TextChild {
        qname,
        simple_type,
        list,
      }) => text_child_fields.push(SdkTextChildField {
        ident: field_ident.clone(),
        qname,
        simple_type,
        ty: field.ty.clone(),
        optional: is_option_type(&field.ty),
        repeated: !list && contains_vec_type(&field.ty),
        list,
      }),
      Some(SdkTypeFieldKind::AnyChild { qname }) => any_child_fields.push(SdkAnyChildField {
        ident: field_ident.clone(),
        qname,
        optional: is_option_type(&field.ty),
        repeated: contains_vec_type(&field.ty),
      }),
      Some(SdkTypeFieldKind::Choice) => choice_fields.push(SdkTypeChoiceField {
        ident: field_ident.clone(),
        ty: field.ty.clone(),
        optional: is_option_type(&field.ty),
        repeated: contains_vec_type(&field.ty),
        accepts_text: parsed_attrs.choice_accepts_text,
        accepts_any: parsed_attrs.choice_accepts_any,
        specific_qnames: parsed_attrs.choice_qnames,
        items: parsed_attrs.choice_items,
      }),
      Some(SdkTypeFieldKind::Text { .. }) => {
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
          "helper structs require #[sdk(child(...))], #[sdk(empty_child(...))], #[sdk(text_child(...))], #[sdk(any_child(...))] or #[sdk(choice)] fields",
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
  let mut direct_child_match_tokens_borrowed: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_match_tokens_io: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_visit_match_tokens_borrowed: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_visit_match_tokens_io: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut child_parse_tokens_borrowed = Vec::new();
  let mut child_parse_tokens_io = Vec::new();
  let mut child_visit_parse_tokens_borrowed = Vec::new();
  let mut child_visit_parse_tokens_io = Vec::new();
  let mut child_write_tokens = Vec::new();
  let mut child_init_tokens = Vec::new();
  let mut child_validate_tokens = Vec::new();
  let xml_reader_ident = Ident::new("xml_reader", Span::call_site());
  let visitor_reader_ident = Ident::new("xml_reader", Span::call_site());
  for field in &child_fields {
    let field_ident = &field.ident;
    let xml_child_slot_assign = quote! { __ooxmlsdk_seen_child = true; };
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
      let deserialize_call = if deserialize_ident == &read_borrowed_inner_ident {
        quote! { <#child_ty as crate::sdk::SdkType>::read_borrowed_inner }
      } else {
        quote! { <#child_ty as crate::sdk::SdkType>::read_io_inner }
      };
      if field.repeated {
        if as_result {
          quote! {
              #case_index => {
                let parsed_child = #deserialize_call(#reader_ident, e, next_empty)?;
                #field_ident.push(#parsed_child_expr);
                __ooxmlsdk_seen_child = true;
                return Ok(true);
              },
          }
        } else {
          quote! {
              #case_index => {
                let parsed_child = #deserialize_call(#reader_ident, e, next_empty)?;
                #field_ident.push(#parsed_child_expr);
                __ooxmlsdk_seen_child = true;
                continue;
              },
          }
        }
      } else {
        if as_result {
          quote! {
            #case_index => {
              let parsed_child = #deserialize_call(#reader_ident, e, next_empty)?;
              #field_ident = Some(#parsed_child_expr);
              __ooxmlsdk_seen_child = true;
              return Ok(true);
            },
          }
        } else {
          quote! {
            #case_index => {
              let parsed_child = #deserialize_call(#reader_ident, e, next_empty)?;
              #field_ident = Some(#parsed_child_expr);
              __ooxmlsdk_seen_child = true;
              continue;
            },
          }
        }
      }
    };
    let build_match = |reader_ident: &Ident, as_result: bool, deserialize_ident: &Ident| {
      let deserialize_call = if deserialize_ident == &read_borrowed_inner_ident {
        quote! { <#child_ty as crate::sdk::SdkType>::read_borrowed_inner }
      } else {
        quote! { <#child_ty as crate::sdk::SdkType>::read_io_inner }
      };
      if field.repeated {
        if as_result {
          quote! {
            #target => {
              let parsed_child = #deserialize_call(#reader_ident, e, next_empty)?;
              #field_ident.push(#parsed_child_expr);
              __ooxmlsdk_seen_child = true;
              #xml_child_slot_assign
              return Ok(true);
            },
          }
        } else {
          quote! {
            #target => {
              let parsed_child = #deserialize_call(#reader_ident, e, next_empty)?;
              #field_ident.push(#parsed_child_expr);
              __ooxmlsdk_seen_child = true;
              #xml_child_slot_assign
              continue;
            },
          }
        }
      } else if as_result {
        quote! {
          #target => {
            let parsed_child = #deserialize_call(#reader_ident, e, next_empty)?;
            #field_ident = Some(#parsed_child_expr);
            __ooxmlsdk_seen_child = true;
            return Ok(true);
          },
        }
      } else {
        quote! {
          #target => {
            let parsed_child = #deserialize_call(#reader_ident, e, next_empty)?;
            #field_ident = Some(#parsed_child_expr);
            __ooxmlsdk_seen_child = true;
            continue;
          },
        }
      }
    };
    direct_child_case_arms.push(quote! { #target => #case_index, });

    if field.repeated {
      child_decl_tokens.push(quote! { let mut #field_ident = Vec::new(); });
      child_init_tokens.push(quote! { #field_ident });
      let child_write_call = write_typed_child_tokens(&child_ty, quote! { child }, &field.qname);
      child_write_tokens.push(quote! {
        for child in &self.#field_ident {
          #child_write_call
        }
      });
      child_validate_tokens.push(quote! {
        for child in &self.#field_ident {
          crate::validator::SdkValidator::validate_into(#validate_child_tokens, context);
        }
      });
      direct_child_dispatch_tokens_borrowed.push(build_dispatch(
        &xml_reader_ident,
        false,
        &read_borrowed_inner_ident,
      ));
      direct_child_dispatch_tokens_io.push(build_dispatch(
        &xml_reader_ident,
        false,
        &read_io_inner_ident,
      ));
      direct_child_visit_dispatch_tokens_borrowed.push(build_dispatch(
        &visitor_reader_ident,
        true,
        &read_borrowed_inner_ident,
      ));
      direct_child_visit_dispatch_tokens_io.push(build_dispatch(
        &visitor_reader_ident,
        true,
        &read_io_inner_ident,
      ));
      direct_child_match_tokens_borrowed.push(build_match(
        &xml_reader_ident,
        false,
        &read_borrowed_inner_ident,
      ));
      direct_child_match_tokens_io.push(build_match(
        &xml_reader_ident,
        false,
        &read_io_inner_ident,
      ));
      direct_child_visit_match_tokens_borrowed.push(build_match(
        &visitor_reader_ident,
        true,
        &read_borrowed_inner_ident,
      ));
      direct_child_visit_match_tokens_io.push(build_match(
        &visitor_reader_ident,
        true,
        &read_io_inner_ident,
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
        let child_write_call = write_typed_child_tokens(&child_ty, quote! { child }, &field.qname);
        child_write_tokens.push(quote! {
          if let Some(child) = &self.#field_ident {
            #child_write_call
          }
        });
        child_validate_tokens.push(quote! {
          if let Some(child) = &self.#field_ident {
            crate::validator::SdkValidator::validate_into(#validate_child_tokens, context);
          }
        });
      } else {
        let self_write_call =
          write_typed_child_tokens(&child_ty, quote! { &self.#field_ident }, &field.qname);
        child_write_tokens.push(quote! {
          #self_write_call
        });
        child_validate_tokens.push(quote! {
          crate::validator::SdkValidator::validate_into(#validate_self_tokens, context);
        });
      }
      direct_child_dispatch_tokens_borrowed.push(build_dispatch(
        &xml_reader_ident,
        false,
        &read_borrowed_inner_ident,
      ));
      direct_child_dispatch_tokens_io.push(build_dispatch(
        &xml_reader_ident,
        false,
        &read_io_inner_ident,
      ));
      direct_child_visit_dispatch_tokens_borrowed.push(build_dispatch(
        &visitor_reader_ident,
        true,
        &read_borrowed_inner_ident,
      ));
      direct_child_visit_dispatch_tokens_io.push(build_dispatch(
        &visitor_reader_ident,
        true,
        &read_io_inner_ident,
      ));
      direct_child_match_tokens_borrowed.push(build_match(
        &xml_reader_ident,
        false,
        &read_borrowed_inner_ident,
      ));
      direct_child_match_tokens_io.push(build_match(
        &xml_reader_ident,
        false,
        &read_io_inner_ident,
      ));
      direct_child_visit_match_tokens_borrowed.push(build_match(
        &visitor_reader_ident,
        true,
        &read_borrowed_inner_ident,
      ));
      direct_child_visit_match_tokens_io.push(build_match(
        &visitor_reader_ident,
        true,
        &read_io_inner_ident,
      ));
    }
  }

  let mut specific_choice_qname_counts = std::collections::HashMap::<String, usize>::new();
  for field in &choice_fields {
    if field.accepts_any.unwrap_or(false) {
      continue;
    }
    let mut seen = std::collections::HashSet::new();
    for qname in &field.specific_qnames {
      if seen.insert(qname) {
        *specific_choice_qname_counts
          .entry(qname.clone())
          .or_insert(0usize) += 1usize;
      }
    }
  }

  let mut choice_decl_tokens = Vec::new();
  let choice_match_init_tokens = Vec::<proc_macro2::TokenStream>::new();
  let choice_match_decl_tokens = Vec::<proc_macro2::TokenStream>::new();
  let mut choice_unique_parse_tokens_borrowed = Vec::new();
  let mut choice_unique_parse_tokens_io = Vec::new();
  let mut choice_unique_visit_parse_tokens_borrowed = Vec::new();
  let mut choice_unique_visit_parse_tokens_io = Vec::new();
  let mut choice_parse_tokens = Vec::new();
  let mut choice_visit_parse_tokens = Vec::new();
  let mut choice_write_tokens = Vec::new();
  let mut choice_init_tokens = Vec::new();
  let mut choice_validate_tokens = Vec::new();
  let mut flat_choice_match_tokens_borrowed = Vec::new();
  let mut flat_choice_match_tokens_io = Vec::new();
  let mut flat_choice_visit_match_tokens_borrowed = Vec::new();
  let mut flat_choice_visit_match_tokens_io = Vec::new();
  let mut grouped_choice_match_tokens_borrowed =
    std::collections::BTreeMap::<String, Vec<GroupedChoiceAttempt>>::new();
  let mut grouped_choice_match_tokens_io =
    std::collections::BTreeMap::<String, Vec<GroupedChoiceAttempt>>::new();
  let mut grouped_choice_visit_match_tokens_borrowed =
    std::collections::BTreeMap::<String, Vec<GroupedChoiceAttempt>>::new();
  let mut grouped_choice_visit_match_tokens_io =
    std::collections::BTreeMap::<String, Vec<GroupedChoiceAttempt>>::new();
  for field in &choice_fields {
    let field_ident = &field.ident;
    let xml_child_slot_assign = quote! { __ooxmlsdk_seen_child = true; };
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
    let field_accepts_any = field.accepts_any.unwrap_or(false);
    let dispatch_qnames: Vec<_> = field
      .specific_qnames
      .iter()
      .filter(|qname| !qname.is_empty())
      .cloned()
      .collect();
    let flatten_specific_choice = !field_accepts_any
      && !dispatch_qnames.is_empty()
      && dispatch_qnames.iter().all(|qname| {
        specific_choice_qname_counts
          .get(qname)
          .copied()
          .unwrap_or_default()
          == 1usize
      });
    if flatten_specific_choice {
      let (borrowed_parse_tokens, io_parse_tokens, borrowed_visit_tokens, io_visit_tokens) =
        build_flat_choice_dispatch_match_tokens(
          field_ident,
          &choice_ty,
          field.repeated,
          &dispatch_qnames,
          xml_child_slot_assign.clone(),
        );
      flat_choice_match_tokens_borrowed.push(borrowed_parse_tokens);
      flat_choice_match_tokens_io.push(io_parse_tokens);
      flat_choice_visit_match_tokens_borrowed.push(borrowed_visit_tokens);
      flat_choice_visit_match_tokens_io.push(io_visit_tokens);
    }
    if flatten_specific_choice {
    } else if !field_accepts_any && !dispatch_qnames.is_empty() {
      let borrowed_attempt = build_choice_parse_attempt_tokens(
        field_ident,
        &choice_ty,
        field.repeated,
        DeserializeMode::Borrowed,
        false,
        xml_child_slot_assign.clone(),
      );
      let io_attempt = build_choice_parse_attempt_tokens(
        field_ident,
        &choice_ty,
        field.repeated,
        DeserializeMode::Io,
        false,
        xml_child_slot_assign.clone(),
      );
      let borrowed_visit_attempt = build_choice_parse_attempt_tokens(
        field_ident,
        &choice_ty,
        field.repeated,
        DeserializeMode::Borrowed,
        true,
        xml_child_slot_assign.clone(),
      );
      let io_visit_attempt = build_choice_parse_attempt_tokens(
        field_ident,
        &choice_ty,
        field.repeated,
        DeserializeMode::Io,
        true,
        xml_child_slot_assign.clone(),
      );
      for target in qname_match_target_keys(&dispatch_qnames) {
        grouped_choice_match_tokens_borrowed
          .entry(target.clone())
          .or_default()
          .push(GroupedChoiceAttempt {
            condition: None,
            tokens: borrowed_attempt.clone(),
          });
        grouped_choice_match_tokens_io
          .entry(target.clone())
          .or_default()
          .push(GroupedChoiceAttempt {
            condition: None,
            tokens: io_attempt.clone(),
          });
        grouped_choice_visit_match_tokens_borrowed
          .entry(target.clone())
          .or_default()
          .push(GroupedChoiceAttempt {
            condition: None,
            tokens: borrowed_visit_attempt.clone(),
          });
        grouped_choice_visit_match_tokens_io
          .entry(target)
          .or_default()
          .push(GroupedChoiceAttempt {
            condition: None,
            tokens: io_visit_attempt.clone(),
          });
      }
    } else if field.repeated {
      choice_unique_parse_tokens_borrowed.push(quote! {
          compile_error!("repeated choice read path must be flattened by SdkType");
      });
      choice_unique_parse_tokens_io.push(quote! {
          compile_error!("repeated choice read path must be flattened by SdkType");
      });
      choice_unique_visit_parse_tokens_borrowed.push(quote! {
          compile_error!("repeated choice read path must be flattened by SdkType");
      });
      choice_unique_visit_parse_tokens_io.push(quote! {
          compile_error!("repeated choice read path must be flattened by SdkType");
      });
    } else {
      choice_unique_parse_tokens_borrowed.push(quote! {
          compile_error!("choice read path must be flattened by SdkType");
      });
      choice_unique_parse_tokens_io.push(quote! {
          compile_error!("choice read path must be flattened by SdkType");
      });
      choice_unique_visit_parse_tokens_borrowed.push(quote! {
          compile_error!("choice read path must be flattened by SdkType");
      });
      choice_unique_visit_parse_tokens_io.push(quote! {
          compile_error!("choice read path must be flattened by SdkType");
      });
    }
    if field.repeated {
      choice_decl_tokens.push(quote! { let mut #field_ident = Vec::new(); });
      choice_init_tokens.push(quote! { #field_ident });
      choice_write_tokens.push(build_choice_write_tokens(
        &choice_ty,
        &field.items,
        field_ident,
        true,
        false,
      )?);
      choice_validate_tokens.push(quote! {
        for choice in &self.#field_ident {
          crate::validator::SdkValidator::validate_into(#validate_choice_tokens, context);
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
        choice_write_tokens.push(build_choice_write_tokens(
          &choice_ty,
          &field.items,
          field_ident,
          false,
          true,
        )?);
        choice_validate_tokens.push(quote! {
          if let Some(choice) = &self.#field_ident {
            crate::validator::SdkValidator::validate_into(#validate_choice_tokens, context);
          }
        });
      } else {
        choice_write_tokens.push(build_choice_write_tokens(
          &choice_ty,
          &field.items,
          field_ident,
          false,
          false,
        )?);
        choice_validate_tokens.push(quote! {
          crate::validator::SdkValidator::validate_into(#validate_self_tokens, context);
        });
      }
      choice_parse_tokens.push(quote! {});
      choice_visit_parse_tokens.push(quote! {});
    }
  }
  flat_choice_match_tokens_borrowed.extend(build_grouped_choice_match_tokens(
    &grouped_choice_match_tokens_borrowed,
    false,
  ));
  flat_choice_match_tokens_io.extend(build_grouped_choice_match_tokens(
    &grouped_choice_match_tokens_io,
    false,
  ));
  flat_choice_visit_match_tokens_borrowed.extend(build_grouped_choice_match_tokens(
    &grouped_choice_visit_match_tokens_borrowed,
    true,
  ));
  flat_choice_visit_match_tokens_io.extend(build_grouped_choice_match_tokens(
    &grouped_choice_visit_match_tokens_io,
    true,
  ));

  let choice_match_count_decl_tokens = quote! {};
  let choice_match_conflict_tokens = quote! {};

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
      field.simple_type.as_deref(),
      &field.ty,
      field.repeated,
      field.optional,
      field.list,
    ));
    let parse_arm = build_text_child_parse_arm(
      ident,
      field_ident,
      &field.qname,
      field.simple_type.as_deref(),
      &field.ty,
      quote! { __ooxmlsdk_seen_child = true; },
      TextChildParseArmOptions {
        repeated: field.repeated,
        as_result: false,
        use_local_name_dispatch: false,
        list: field.list,
      },
    );
    child_parse_tokens_borrowed.push(parse_arm.clone());
    child_parse_tokens_io.push(parse_arm);
    let visit_parse_arm = build_text_child_parse_arm(
      ident,
      field_ident,
      &field.qname,
      field.simple_type.as_deref(),
      &field.ty,
      quote! { __ooxmlsdk_seen_child = true; },
      TextChildParseArmOptions {
        repeated: field.repeated,
        as_result: true,
        use_local_name_dispatch: false,
        list: field.list,
      },
    );
    child_visit_parse_tokens_borrowed.push(visit_parse_arm.clone());
    child_visit_parse_tokens_io.push(visit_parse_arm);
  }

  for field in &any_child_fields {
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
    child_write_tokens.push(build_any_child_write_tokens(
      field_ident,
      &field.qname,
      field.repeated,
      field.optional,
    ));
    child_parse_tokens_borrowed.push(build_any_child_parse_arm(
      ident,
      field_ident,
      &field.qname,
      field.repeated,
      false,
      DeserializeMode::Borrowed,
      quote! { __ooxmlsdk_seen_child = true; },
    ));
    child_parse_tokens_io.push(build_any_child_parse_arm(
      ident,
      field_ident,
      &field.qname,
      field.repeated,
      false,
      DeserializeMode::Io,
      quote! { __ooxmlsdk_seen_child = true; },
    ));
    child_visit_parse_tokens_borrowed.push(build_any_child_parse_arm(
      ident,
      field_ident,
      &field.qname,
      field.repeated,
      true,
      DeserializeMode::Borrowed,
      quote! { __ooxmlsdk_seen_child = true; },
    ));
    child_visit_parse_tokens_io.push(build_any_child_parse_arm(
      ident,
      field_ident,
      &field.qname,
      field.repeated,
      true,
      DeserializeMode::Io,
      quote! { __ooxmlsdk_seen_child = true; },
    ));
  }

  let has_choice_dispatch = !choice_fields.is_empty();
  let main_dispatch_tokens_borrowed = if !has_choice_dispatch {
    quote! {
      let matched = match event_name {
        #( #direct_child_match_tokens_borrowed )*
        #( #child_parse_tokens_borrowed )*
        _ => false,
      };
      if matched {
        continue;
      }
    }
  } else {
    quote! {
      #( #choice_match_init_tokens )*
      let matched = match event_name {
        #( #direct_child_match_tokens_borrowed )*
        #( #flat_choice_match_tokens_borrowed )*
        #( #child_parse_tokens_borrowed )*
        _ => {
          {
            #choice_match_count_decl_tokens
            #( #choice_match_decl_tokens )*
            #( #choice_unique_parse_tokens_borrowed )*
            #choice_match_conflict_tokens
          }
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
      let matched = match event_name {
        #( #direct_child_match_tokens_io )*
        #( #child_parse_tokens_io )*
        _ => false,
      };
      if matched {
        continue;
      }
    }
  } else {
    quote! {
      #( #choice_match_init_tokens )*
      let matched = match event_name {
        #( #direct_child_match_tokens_io )*
        #( #flat_choice_match_tokens_io )*
        #( #child_parse_tokens_io )*
        _ => {
          {
            #choice_match_count_decl_tokens
            #( #choice_match_decl_tokens )*
            #( #choice_unique_parse_tokens_io )*
            #choice_match_conflict_tokens
          }
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
  let unmatched_child_tokens_borrowed = quote! {
    if __ooxmlsdk_seen_child {
      let _ = event_name;
      xml_reader.unread(if next_empty {
        quick_xml::events::Event::Empty(e)
      } else {
        quick_xml::events::Event::Start(e)
      })?;
      break;
    }
    return Err(crate::common::unexpected_tag(
      stringify!(#ident),
      "known child",
      event_name,
    ));
  };
  let unmatched_child_tokens_io = quote! {
    if __ooxmlsdk_seen_child {
      let _ = event_name;
      xml_reader.unread(if next_empty {
        quick_xml::events::Event::Empty(e)
      } else {
        quick_xml::events::Event::Start(e)
      })?;
      break;
    }
    return Err(crate::common::unexpected_tag(
      stringify!(#ident),
      "known child",
      event_name,
    ));
  };
  let mce_child_process_tokens = child_fields
    .iter()
    .map(mce_process_child_field_tokens)
    .collect::<Vec<_>>();
  let mce_choice_process_tokens = choice_fields
    .iter()
    .map(mce_process_choice_field_tokens)
    .collect::<Vec<_>>();
  let mut mce_choice_impl_keys = std::collections::HashSet::new();
  let mce_choice_impl_tokens = choice_fields
    .iter()
    .filter_map(|field| {
      let choice_ty = unwrap_option_vec_type(&field.ty);
      mce_choice_impl_keys
        .insert(quote! { #choice_ty }.to_string())
        .then(|| mce_choice_impl_tokens(field))
    })
    .collect::<Vec<_>>();

  Ok(quote! {
    #( #mce_choice_impl_tokens )*

    impl #impl_generics crate::sdk::SdkType for #ident #type_generics #where_clause {
      fn read_borrowed_inner<'de>(
        xml_reader: &mut crate::common::SliceReader<'de>,
        start: quick_xml::events::BytesStart<'de>,
        empty: bool,
      ) -> Result<Self, crate::common::SdkError> {
        let mut pending_event = Some((start, empty));

        #( #child_decl_tokens )*
        #( #choice_decl_tokens )*
        let mut __ooxmlsdk_seen_child = false;

        loop {
          if let Some((e, next_empty)) = pending_event.take() {
            let event_name = e.name().into_inner();
            #main_dispatch_tokens_borrowed
            #unmatched_child_tokens_borrowed
          }

          match xml_reader.next()? {
            quick_xml::events::Event::Start(e) => {
              let e = e.into_owned();
              let next_empty = false;
              let event_name = e.name().into_inner();
              #main_dispatch_tokens_borrowed
              #unmatched_child_tokens_borrowed
            }
            quick_xml::events::Event::Empty(e) => {
              let e = e.into_owned();
              let next_empty = true;
              let event_name = e.name().into_inner();
              #main_dispatch_tokens_borrowed
              #unmatched_child_tokens_borrowed
            }
            quick_xml::events::Event::End(e) => {
              if __ooxmlsdk_seen_child {
                xml_reader.unread(quick_xml::events::Event::End(e))?;
              }
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

      fn read_io_inner<R: std::io::BufRead>(
        xml_reader: &mut crate::common::IoReader<R>,
        start: quick_xml::events::BytesStart<'static>,
        empty: bool,
      ) -> Result<Self, crate::common::SdkError> {
        let mut pending_event = Some((start, empty));

        #( #child_decl_tokens )*
        #( #choice_decl_tokens )*
        let mut __ooxmlsdk_seen_child = false;

        loop {
          if let Some((e, next_empty)) = pending_event.take() {
            let event_name = e.name().into_inner();
            #main_dispatch_tokens_io
            #unmatched_child_tokens_io
          }

          let next_event = match xml_reader.next_tag_event()? {
            crate::common::IoTagEvent::Start(e, next_empty) => Some((e, next_empty)),
            crate::common::IoTagEvent::End(e) => {
              if __ooxmlsdk_seen_child {
                xml_reader.unread(quick_xml::events::Event::End(e))?;
              }
              break;
            }
            crate::common::IoTagEvent::Eof => {
              return Err(crate::common::unexpected_eof(stringify!(#ident)));
            }
            crate::common::IoTagEvent::Decl(_) | crate::common::IoTagEvent::Other => None,
          };

          if let Some((e, next_empty)) = next_event {
            let event_name = e.name().into_inner();
            #main_dispatch_tokens_io
            #unmatched_child_tokens_io
          }
        }

        Ok(Self {
          #( #child_init_tokens, )*
          #( #choice_init_tokens, )*
        })
      }

      fn write_inner<W: std::io::Write>(
        &self,
        writer: &mut W,
      ) -> Result<(), std::io::Error> {
        #( #child_write_tokens )*
        #( #choice_write_tokens )*
        Ok(())
      }
    }
    #[cfg(feature = "validators")]
    impl #impl_generics crate::validator::SdkValidator for #ident #type_generics #where_clause {
      fn validate_into(&self, context: &mut crate::validator::ValidationContext) {
        #( #child_validate_tokens )*
        #( #choice_validate_tokens )*
      }
    }

    #[cfg(feature = "mce")]
    impl #impl_generics crate::sdk::SdkMce for #ident #type_generics #where_clause {
      fn process_mce_with_context(
        &mut self,
        settings: &crate::sdk::MarkupCompatibilityProcessSettings,
        context: &mut crate::sdk::MceContext,
      ) -> Result<(), crate::common::SdkError> {
        if matches!(
          settings.process_mode,
          crate::sdk::MarkupCompatibilityProcessMode::NoProcess
        ) {
          return Ok(());
        }
        #( #mce_child_process_tokens )*
        #( #mce_choice_process_tokens )*
        Ok(())
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
  let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
  let QNameInfo {
    tag_prefix,
    local_name,
  } = parse_qname_info(schema_qname);
  let tag_qname = if tag_prefix.is_empty() {
    local_name.clone()
  } else {
    format!("{tag_prefix}:{local_name}")
  };
  let tag_qname_lit = LitByteStr::new(tag_qname.as_bytes(), Span::call_site());
  let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
  let end_name_matches = if local_name.is_empty() {
    quote! { e.name() == __end_qname }
  } else if tag_prefix.is_empty() {
    quote! { e.name().as_ref() == #local_name_lit }
  } else {
    quote! { e.name().as_ref() == #tag_qname_lit || e.name().as_ref() == #local_name_lit }
  };
  let end_qname_decl = if local_name.is_empty() {
    quote! { let __end_qname = e.name(); }
  } else {
    quote! {}
  };
  let raw_tag_prefix_lit = LitByteStr::new(tag_prefix.as_bytes(), Span::call_site());
  let start_tag_open = write_start_tag_open_tokens(schema_qname);
  let end_tag = write_end_tag_tokens(schema_qname);
  let default_ns = parse_sdk_default_ns(&input.attrs)?;
  let fixed_namespace_uri = namespaces::uri_by_prefix(&tag_prefix);
  let fixed_namespace_uri_lit =
    fixed_namespace_uri.map(|uri| LitByteStr::new(uri.as_bytes(), Span::call_site()));
  let fixed_namespace_write_lit = fixed_namespace_uri.map(|uri| {
    let attr = if default_ns {
      format!(" xmlns=\"{uri}\"")
    } else {
      format!(" xmlns:{tag_prefix}=\"{uri}\"")
    };
    LitByteStr::new(attr.as_bytes(), Span::call_site())
  });
  let read_borrowed_inner_ident = deserialize_type_inner_ident(DeserializeMode::Borrowed);
  let read_io_inner_ident = deserialize_type_inner_ident(DeserializeMode::Io);

  let mut attr_fields = Vec::new();
  let mut child_fields = Vec::new();
  let mut empty_child_fields = Vec::new();
  let mut text_child_fields = Vec::new();
  let mut any_child_fields = Vec::new();
  let mut choice_fields = Vec::new();
  let mut any_fields = Vec::new();
  let mut text_field = None;
  let mut xmlns_fields = Vec::new();
  let mut xml_header_field = None;
  let mut xml_other_attrs_field = None;
  let mut xml_other_children_field = None;
  let mut compact_xml_other_children = false;
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
    if field_ident == "xml_other_attrs" {
      xml_other_attrs_field = Some(field_ident.clone());
      continue;
    }
    if field_ident == "xml_other_children" {
      compact_xml_other_children = vec_inner_type(&field.ty)
        .as_ref()
        .is_some_and(is_box_u8_slice_type);
      xml_other_children_field = Some(field_ident.clone());
      continue;
    }

    let parsed_attrs = parse_sdk_type_field_attrs(&field.attrs)?;
    let Some(field_kind) = parsed_attrs.kind else {
      return Err(syn::Error::new_spanned(
        field,
        "missing #[sdk(...)] field attribute",
      ));
    };
    ordered_field_specs.push((
      field_ident.clone(),
      field.ty.clone(),
      field_kind.clone(),
      parsed_attrs.choice_items.clone(),
    ));

    match field_kind {
      SdkTypeFieldKind::Attr {
        name,
        simple_type,
        list,
      } => attr_fields.push(SdkAttrField {
        ident: field_ident.clone(),
        name,
        simple_type,
        ty: field.ty.clone(),
        optional: is_option_type(&field.ty),
        list,
        validators: parsed_attrs.validators,
      }),
      SdkTypeFieldKind::Child { qname } => child_fields.push(SdkChildField {
        ident: field_ident.clone(),
        qname,
        ty: field.ty.clone(),
        optional: is_option_type(&field.ty),
        repeated: contains_vec_type(&field.ty),
      }),
      SdkTypeFieldKind::EmptyChild { qname } => empty_child_fields.push(SdkEmptyChildField {
        ident: field_ident.clone(),
        qname,
        optional: is_option_type(&field.ty),
        repeated: contains_vec_type(&field.ty),
      }),
      SdkTypeFieldKind::TextChild {
        qname,
        simple_type,
        list,
      } => text_child_fields.push(SdkTextChildField {
        ident: field_ident.clone(),
        qname,
        simple_type,
        ty: field.ty.clone(),
        optional: is_option_type(&field.ty),
        repeated: !list && contains_vec_type(&field.ty),
        list,
      }),
      SdkTypeFieldKind::AnyChild { qname } => any_child_fields.push(SdkAnyChildField {
        ident: field_ident.clone(),
        qname,
        optional: is_option_type(&field.ty),
        repeated: contains_vec_type(&field.ty),
      }),
      SdkTypeFieldKind::Choice => choice_fields.push(SdkTypeChoiceField {
        ident: field_ident.clone(),
        ty: field.ty.clone(),
        optional: is_option_type(&field.ty),
        repeated: contains_vec_type(&field.ty),
        accepts_text: parsed_attrs.choice_accepts_text,
        accepts_any: parsed_attrs.choice_accepts_any,
        specific_qnames: parsed_attrs.choice_qnames,
        items: parsed_attrs.choice_items,
      }),
      SdkTypeFieldKind::Any => any_fields.push(SdkAnyField {
        ident: field_ident.clone(),
        ty: field.ty.clone(),
        optional: is_option_type(&field.ty),
        repeated: contains_vec_type(&field.ty),
      }),
      SdkTypeFieldKind::Text { list } => {
        text_field = Some(SdkTextField {
          ident: field_ident.clone(),
          ty: field.ty.clone(),
          optional: is_option_type(&field.ty),
          list,
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
  let has_xml_other_attrs_field = xml_other_attrs_field.is_some();
  let has_xml_other_children_field = xml_other_children_field.is_some();
  let use_canonical_xmlns_prefix = has_xmlns_fields && {
    let QNameInfo { tag_prefix, .. } = parse_qname_info(schema_qname);
    is_canonical_xmlns_prefix_namespace(&tag_prefix)
  };
  let use_local_name_child_dispatch = should_use_local_name_child_dispatch(
    schema_qname,
    child_fields
      .iter()
      .map(|field| field.qname.clone())
      .chain(empty_child_fields.iter().map(|field| field.qname.clone()))
      .chain(text_child_fields.iter().map(|field| field.qname.clone()))
      .chain(any_child_fields.iter().map(|field| field.qname.clone())),
    !choice_fields.is_empty(),
    !any_fields.is_empty(),
    has_xml_other_children_field,
  );

  let mut xml_child_slot_by_field = std::collections::HashMap::<String, usize>::new();
  let mut xml_child_slot_count = 0usize;
  for (field_ident, _, field_kind, _) in &ordered_field_specs {
    if matches!(
      field_kind,
      SdkTypeFieldKind::Child { .. }
        | SdkTypeFieldKind::EmptyChild { .. }
        | SdkTypeFieldKind::TextChild { .. }
        | SdkTypeFieldKind::AnyChild { .. }
        | SdkTypeFieldKind::Choice
        | SdkTypeFieldKind::Any
    ) {
      xml_child_slot_count += 1usize;
      xml_child_slot_by_field.insert(field_ident.to_string(), xml_child_slot_count);
    }
  }

  let xml_child_slot_assign_tokens = |xml_child_slot: usize| {
    if !has_xml_other_children_field || compact_xml_other_children {
      quote! {}
    } else {
      quote! {
        __xml_child_slot = #xml_child_slot;
      }
    }
  };

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
    let value_ty = if field.list {
      vec_inner_type(&unwrap_option_type(&field.ty)).ok_or_else(|| {
        syn::Error::new_spanned(&field.ty, "#[sdk(attr(..., list))] requires Vec<T>")
      })?
    } else {
      unwrap_wrapped_type(&field.ty)
    };
    let simple_type = field.simple_type.as_deref();
    let simple_union_kind = simple_union_effective_type_kind(&value_ty, simple_type);
    let integer_kind = integer_effective_type_kind(&value_ty, simple_type);
    let parser = if field.list {
      quote! {
        crate::common::parse_list_attr::<#value_ty>(
          &attr,
          decoder,
          stringify!(#ident),
          #name_lit,
        )?
      }
    } else if let Some(kind) = simple_union_kind {
      parse_simple_union_attr_tokens(kind)
    } else if let Some(kind) = integer_kind {
      parse_integer_attr_tokens_by_kind(
        kind,
        quote! { &attr },
        quote! { decoder },
        quote! { stringify!(#ident) },
        quote! { #name_lit },
      )
    } else if is_sdk_enum_effective_type(&value_ty, simple_type) {
      quote! { crate::common::parse_enum_attr::<#value_ty>(&attr, decoder, stringify!(#ident))? }
    } else if is_string_like_effective_type(&value_ty, simple_type) {
      quote! { crate::common::decode_attr_value(&attr, decoder)? }
    } else {
      quote! { crate::common::parse_attr_value::<#value_ty>(&attr, decoder, stringify!(#ident), #name_lit)? }
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
    let attr_write_value_tokens = if field.list {
      quote! {
        crate::common::write_list_attr_value(writer, #name_lit, value.as_slice())?;
      }
    } else if let Some(kind) = simple_union_kind {
      write_simple_union_attr_tokens(kind, &name_lit, quote! { value })
    } else if effective_type_name(&value_ty, simple_type)
      .as_deref()
      .is_some_and(is_xml_schema_float_type_name)
    {
      let write_value_tokens =
        write_xml_schema_float_effective_tokens(quote! { value }, &value_ty, simple_type, "");
      quote! {
        writer.write_all(b" ")?;
        writer.write_all(#name_lit.as_bytes())?;
        writer.write_all(b"=\"")?;
        #write_value_tokens
        writer.write_all(b"\"")?;
      }
    } else if is_string_like_effective_type(&value_ty, simple_type) {
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

    let mut direct_validator_tokens = if simple_union_kind.is_some() {
      Vec::new()
    } else if is_hex_binary_effective_type(&value_ty, simple_type) {
      vec![quote! {
        crate::validator::validate_binary_format(
          stringify!(#ident),
          stringify!(#field_ident),
          value,
          crate::validator::BinaryFormatKind::Hex,
        )?;
      }]
    } else if is_base64_binary_effective_type(&value_ty, simple_type) {
      vec![quote! {
        crate::validator::validate_binary_format(
          stringify!(#ident),
          stringify!(#field_ident),
          value,
          crate::validator::BinaryFormatKind::Base64,
        )?;
      }]
    } else if is_decimal_value_effective_type(&value_ty, simple_type) {
      vec![quote! {
        crate::validator::validate_decimal_format(
          stringify!(#ident),
          stringify!(#field_ident),
          value,
        )?;
      }]
    } else if is_datetime_value_effective_type(&value_ty, simple_type) {
      vec![quote! {
        crate::validator::validate_datetime_format(
          stringify!(#ident),
          stringify!(#field_ident),
          value,
        )?;
      }]
    } else {
      Vec::new()
    };
    let mut union_validator_tokens: std::collections::BTreeMap<u32, Vec<proc_macro2::TokenStream>> =
      std::collections::BTreeMap::new();
    if simple_union_kind.is_none() {
      for validator in &field.validators {
        let token = validator_token(ident, field_ident, &value_ty, validator);
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
      if field.list && field.optional {
        attr_validate_tokens.push(quote! {
          if let Some(values) = &self.#field_ident {
            context.check(|| -> Result<(), crate::common::SdkError> {
              for value in values {
                #( #validator_tokens )*
              }
              Ok(())
            });
          }
        });
      } else if field.list {
        attr_validate_tokens.push(quote! {
          {
            let values = &self.#field_ident;
            context.check(|| -> Result<(), crate::common::SdkError> {
              for value in values {
                #( #validator_tokens )*
              }
              Ok(())
            });
          }
        });
      } else if field.optional {
        attr_validate_tokens.push(quote! {
          if let Some(value) = &self.#field_ident {
            context.check(|| -> Result<(), crate::common::SdkError> {
              #( #validator_tokens )*
              Ok(())
            });
          }
        });
      } else {
        attr_validate_tokens.push(quote! {
          {
            let value = &self.#field_ident;
            context.check(|| -> Result<(), crate::common::SdkError> {
              #( #validator_tokens )*
              Ok(())
            });
          }
        });
      }
    }
  }

  let xmlns_parse_tokens = if has_xmlns_fields {
    let default_xmlns_parse_tokens = if let Some(uri) = &fixed_namespace_uri_lit {
      quote! {
        if attr.value.as_ref() != #uri.as_slice() {
          xmlns.push(crate::common::XmlNamespace::new("", attr.value.as_ref()));
        }
      }
    } else {
      quote! {
        xmlns.push(crate::common::XmlNamespace::new("", attr.value.as_ref()));
      }
    };
    let prefixed_xmlns_parse_tokens = if let Some(uri_lit) = &fixed_namespace_uri_lit {
      quote! {
        if &key[6..] != #raw_tag_prefix_lit.as_slice()
          || attr.value.as_ref() != #uri_lit.as_slice()
        {
          xmlns.push(crate::common::XmlNamespace::new(&key[6..], attr.value.as_ref()));
        }
      }
    } else {
      quote! {
        xmlns.push(crate::common::XmlNamespace::new(&key[6..], attr.value.as_ref()));
      }
    };
    quote! {
      b"xmlns" => {
        #default_xmlns_parse_tokens
      }
      key if key.starts_with(b"xmlns:") => {
        #prefixed_xmlns_parse_tokens
      }
    }
  } else {
    quote! {}
  };
  let xml_other_attrs_parse_tokens = if has_xml_other_attrs_field && has_xmlns_fields {
    quote! {
      key => {
        xml_other_attrs.push(crate::common::XmlOtherAttr::new_raw(key, attr.value.as_ref()));
      }
    }
  } else if has_xml_other_attrs_field {
    quote! {
      b"xmlns" => {}
      key if key.starts_with(b"xmlns:") => {}
      key => {
        xml_other_attrs.push(crate::common::XmlOtherAttr::new_raw(key, attr.value.as_ref()));
      }
    }
  } else {
    quote! { _ => {} }
  };
  let namespace_attr_parse_tokens =
    if attr_fields.is_empty() && !has_xmlns_fields && !has_xml_other_attrs_field {
      quote! {}
    } else {
      let decoder_decl_tokens = if attr_fields.is_empty() {
        quote! {}
      } else {
        quote! { let decoder = xml_reader.decoder(); }
      };
      match (has_xmlns_fields, has_xml_other_attrs_field) {
        (true, true) => quote! {
          let mut xmlns = Vec::<crate::common::XmlNamespace>::new();
          let mut xml_other_attrs = Vec::<crate::common::XmlOtherAttr>::new();
          #decoder_decl_tokens
          for attr in e.attributes().with_checks(false) {
            let attr = attr?;
              match attr.key.as_ref() {
                #xmlns_parse_tokens
                #( #attr_parse_tokens )*
                #xml_other_attrs_parse_tokens
            }
          }
        },
        (true, false) => quote! {
          let mut xmlns = Vec::<crate::common::XmlNamespace>::new();
          #decoder_decl_tokens
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
          let mut xml_other_attrs = Vec::<crate::common::XmlOtherAttr>::new();
          #decoder_decl_tokens
          for attr in e.attributes().with_checks(false) {
            let attr = attr?;
            match attr.key.as_ref() {
              #( #attr_parse_tokens )*
              #xml_other_attrs_parse_tokens
            }
          }
        },
        (false, false) => quote! {
          #decoder_decl_tokens
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
    quote! {}
  };
  let mut direct_child_case_arms: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_dispatch_tokens_borrowed: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_dispatch_tokens_io: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_visit_dispatch_tokens_borrowed: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_visit_dispatch_tokens_io: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_match_tokens_borrowed: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_match_tokens_io: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_visit_match_tokens_borrowed: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut direct_child_visit_match_tokens_io: Vec<proc_macro2::TokenStream> = Vec::new();
  let mut child_decl_tokens = Vec::new();
  let mut child_parse_tokens_borrowed = Vec::new();
  let mut child_parse_tokens_io = Vec::new();
  let mut child_visit_parse_tokens_borrowed = Vec::new();
  let mut child_visit_parse_tokens_io = Vec::new();
  let mut child_write_tokens = Vec::new();
  let mut child_init_tokens = Vec::new();
  let mut child_validate_tokens = Vec::new();
  let xml_reader_ident = Ident::new("xml_reader", Span::call_site());
  let visitor_reader_ident = Ident::new("xml_reader", Span::call_site());
  for field in &child_fields {
    let field_ident = &field.ident;
    let xml_child_slot = xml_child_slot_by_field
      .get(&field_ident.to_string())
      .copied()
      .unwrap_or_default();
    let xml_child_slot_assign = xml_child_slot_assign_tokens(xml_child_slot);
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
    let target = if use_local_name_child_dispatch || tag_prefix.is_empty() {
      quote! { #local_name_lit }
    } else {
      let tag_qname_lit = LitByteStr::new(
        format!("{tag_prefix}:{local_name}").as_bytes(),
        Span::call_site(),
      );
      quote! { #tag_qname_lit | #local_name_lit }
    };
    let child_write_call = write_typed_child_tokens(&child_ty, quote! { child }, &field.qname);
    let self_write_call =
      write_typed_child_tokens(&child_ty, quote! { &self.#field_ident }, &field.qname);
    let build_dispatch = |reader_ident: &Ident, as_result: bool, deserialize_ident: &Ident| {
      let deserialize_call = if deserialize_ident == &read_borrowed_inner_ident {
        quote! { <#child_ty as crate::sdk::SdkType>::read_borrowed_inner }
      } else {
        quote! { <#child_ty as crate::sdk::SdkType>::read_io_inner }
      };
      if field.repeated {
        if as_result {
          quote! {
            #case_index => {
              let parsed_child = #deserialize_call(#reader_ident, e, next_empty)?;
              #field_ident.push(#parsed_child_expr);
              return Ok(true);
            },
          }
        } else {
          quote! {
            #case_index => {
              let parsed_child = #deserialize_call(#reader_ident, e, next_empty)?;
              #field_ident.push(#parsed_child_expr);
              continue;
            },
          }
        }
      } else {
        if as_result {
          quote! {
            #case_index => {
              let parsed_child = #deserialize_call(#reader_ident, e, next_empty)?;
              #field_ident = Some(#parsed_child_expr);
              #xml_child_slot_assign
              return Ok(true);
            },
          }
        } else {
          quote! {
            #case_index => {
              let parsed_child = #deserialize_call(#reader_ident, e, next_empty)?;
              #field_ident = Some(#parsed_child_expr);
              #xml_child_slot_assign
              continue;
            },
          }
        }
      }
    };
    let build_match = |reader_ident: &Ident, as_result: bool, deserialize_ident: &Ident| {
      let deserialize_call = if deserialize_ident == &read_borrowed_inner_ident {
        quote! { <#child_ty as crate::sdk::SdkType>::read_borrowed_inner }
      } else {
        quote! { <#child_ty as crate::sdk::SdkType>::read_io_inner }
      };
      if field.repeated {
        if as_result {
          quote! {
            #target => {
              let parsed_child = #deserialize_call(#reader_ident, e, next_empty)?;
              #field_ident.push(#parsed_child_expr);
              #xml_child_slot_assign
              return Ok(true);
            },
          }
        } else {
          quote! {
            #target => {
              let parsed_child = #deserialize_call(#reader_ident, e, next_empty)?;
              #field_ident.push(#parsed_child_expr);
              #xml_child_slot_assign
              continue;
            },
          }
        }
      } else if as_result {
        quote! {
          #target => {
            let parsed_child = #deserialize_call(#reader_ident, e, next_empty)?;
            #field_ident = Some(#parsed_child_expr);
            #xml_child_slot_assign
            return Ok(true);
          },
        }
      } else {
        quote! {
          #target => {
            let parsed_child = #deserialize_call(#reader_ident, e, next_empty)?;
            #field_ident = Some(#parsed_child_expr);
            #xml_child_slot_assign
            continue;
          },
        }
      }
    };

    if field.repeated {
      child_decl_tokens.push(quote! { let mut #field_ident = Vec::new(); });
      child_init_tokens.push(quote! { #field_ident });
      child_write_tokens.push(quote! {
        for child in &self.#field_ident {
          #child_write_call
        }
      });
      child_validate_tokens.push(quote! {
        for child in &self.#field_ident {
          crate::validator::SdkValidator::validate_into(#validate_child_tokens, context);
        }
      });
      direct_child_case_arms.push(quote! { #target => #case_index, });
      direct_child_dispatch_tokens_borrowed.push(build_dispatch(
        &xml_reader_ident,
        false,
        &read_borrowed_inner_ident,
      ));
      direct_child_dispatch_tokens_io.push(build_dispatch(
        &xml_reader_ident,
        false,
        &read_io_inner_ident,
      ));
      direct_child_visit_dispatch_tokens_borrowed.push(build_dispatch(
        &visitor_reader_ident,
        true,
        &read_borrowed_inner_ident,
      ));
      direct_child_visit_dispatch_tokens_io.push(build_dispatch(
        &visitor_reader_ident,
        true,
        &read_io_inner_ident,
      ));
      direct_child_match_tokens_borrowed.push(build_match(
        &xml_reader_ident,
        false,
        &read_borrowed_inner_ident,
      ));
      direct_child_match_tokens_io.push(build_match(
        &xml_reader_ident,
        false,
        &read_io_inner_ident,
      ));
      direct_child_visit_match_tokens_borrowed.push(build_match(
        &visitor_reader_ident,
        true,
        &read_borrowed_inner_ident,
      ));
      direct_child_visit_match_tokens_io.push(build_match(
        &visitor_reader_ident,
        true,
        &read_io_inner_ident,
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
            #child_write_call
          }
        });
        child_validate_tokens.push(quote! {
          if let Some(child) = &self.#field_ident {
            crate::validator::SdkValidator::validate_into(#validate_child_tokens, context);
          }
        });
      } else {
        child_write_tokens.push(quote! {
          #self_write_call
        });
        child_validate_tokens.push(quote! {
          crate::validator::SdkValidator::validate_into(#validate_self_tokens, context);
        });
      }
      direct_child_case_arms.push(quote! { #target => #case_index, });
      direct_child_dispatch_tokens_borrowed.push(build_dispatch(
        &xml_reader_ident,
        false,
        &read_borrowed_inner_ident,
      ));
      direct_child_dispatch_tokens_io.push(build_dispatch(
        &xml_reader_ident,
        false,
        &read_io_inner_ident,
      ));
      direct_child_visit_dispatch_tokens_borrowed.push(build_dispatch(
        &visitor_reader_ident,
        true,
        &read_borrowed_inner_ident,
      ));
      direct_child_visit_dispatch_tokens_io.push(build_dispatch(
        &visitor_reader_ident,
        true,
        &read_io_inner_ident,
      ));
      direct_child_match_tokens_borrowed.push(build_match(
        &xml_reader_ident,
        false,
        &read_borrowed_inner_ident,
      ));
      direct_child_match_tokens_io.push(build_match(
        &xml_reader_ident,
        false,
        &read_io_inner_ident,
      ));
      direct_child_visit_match_tokens_borrowed.push(build_match(
        &visitor_reader_ident,
        true,
        &read_borrowed_inner_ident,
      ));
      direct_child_visit_match_tokens_io.push(build_match(
        &visitor_reader_ident,
        true,
        &read_io_inner_ident,
      ));
    }
  }

  for field in &empty_child_fields {
    let field_ident = &field.ident;
    let xml_child_slot = xml_child_slot_by_field
      .get(&field_ident.to_string())
      .copied()
      .unwrap_or_default();
    let xml_child_slot_assign = xml_child_slot_assign_tokens(xml_child_slot);
    let QNameInfo {
      tag_prefix,
      local_name,
    } = parse_qname_info(&field.qname);
    let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
    let target = if use_local_name_child_dispatch || tag_prefix.is_empty() {
      quote! { #local_name_lit }
    } else {
      let tag_qname_lit = LitByteStr::new(
        format!("{tag_prefix}:{local_name}").as_bytes(),
        Span::call_site(),
      );
      quote! { #tag_qname_lit | #local_name_lit }
    };
    let case_index = direct_child_dispatch_tokens_borrowed.len() + 1;
    let build_dispatch = |reader_ident: &Ident, as_result: bool, mode: DeserializeMode| {
      let skip_tokens = build_empty_child_skip_tokens(ident, &field.qname, mode, reader_ident);
      let assign_tokens = if field.repeated {
        quote! { #field_ident.push(()); }
      } else {
        quote! { #field_ident = Some(()); }
      };
      if as_result {
        quote! {
          #case_index => {
            #skip_tokens
            #assign_tokens
            #xml_child_slot_assign
            return Ok(true);
          },
        }
      } else {
        quote! {
          #case_index => {
            #skip_tokens
            #assign_tokens
            #xml_child_slot_assign
            continue;
          },
        }
      }
    };
    let build_match = |reader_ident: &Ident, as_result: bool, mode: DeserializeMode| {
      let skip_tokens = build_empty_child_skip_tokens(ident, &field.qname, mode, reader_ident);
      let assign_tokens = if field.repeated {
        quote! { #field_ident.push(()); }
      } else {
        quote! { #field_ident = Some(()); }
      };
      if as_result {
        quote! {
          #target => {
            #skip_tokens
            #assign_tokens
            #xml_child_slot_assign
            return Ok(true);
          },
        }
      } else {
        quote! {
          #target => {
            #skip_tokens
            #assign_tokens
            #xml_child_slot_assign
            continue;
          },
        }
      }
    };
    direct_child_case_arms.push(quote! { #target => #case_index, });

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
    child_write_tokens.push(build_empty_child_write_tokens(
      field_ident,
      &field.qname,
      field.repeated,
      field.optional,
    ));
    direct_child_dispatch_tokens_borrowed.push(build_dispatch(
      &xml_reader_ident,
      false,
      DeserializeMode::Borrowed,
    ));
    direct_child_dispatch_tokens_io.push(build_dispatch(
      &xml_reader_ident,
      false,
      DeserializeMode::Io,
    ));
    direct_child_visit_dispatch_tokens_borrowed.push(build_dispatch(
      &visitor_reader_ident,
      true,
      DeserializeMode::Borrowed,
    ));
    direct_child_visit_dispatch_tokens_io.push(build_dispatch(
      &visitor_reader_ident,
      true,
      DeserializeMode::Io,
    ));
    direct_child_match_tokens_borrowed.push(build_match(
      &xml_reader_ident,
      false,
      DeserializeMode::Borrowed,
    ));
    direct_child_match_tokens_io.push(build_match(&xml_reader_ident, false, DeserializeMode::Io));
    direct_child_visit_match_tokens_borrowed.push(build_match(
      &visitor_reader_ident,
      true,
      DeserializeMode::Borrowed,
    ));
    direct_child_visit_match_tokens_io.push(build_match(
      &visitor_reader_ident,
      true,
      DeserializeMode::Io,
    ));
  }

  for field in &text_child_fields {
    let field_ident = &field.ident;
    let xml_child_slot = xml_child_slot_by_field
      .get(&field_ident.to_string())
      .copied()
      .unwrap_or_default();
    let xml_child_slot_assign = xml_child_slot_assign_tokens(xml_child_slot);
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
      field.simple_type.as_deref(),
      &field.ty,
      field.repeated,
      field.optional,
      field.list,
    ));
    let parse_arm = build_text_child_parse_arm(
      ident,
      field_ident,
      &field.qname,
      field.simple_type.as_deref(),
      &field.ty,
      xml_child_slot_assign.clone(),
      TextChildParseArmOptions {
        repeated: field.repeated,
        as_result: false,
        use_local_name_dispatch: use_local_name_child_dispatch,
        list: field.list,
      },
    );
    child_parse_tokens_borrowed.push(parse_arm.clone());
    child_parse_tokens_io.push(parse_arm);
    let visit_parse_arm = build_text_child_parse_arm(
      ident,
      field_ident,
      &field.qname,
      field.simple_type.as_deref(),
      &field.ty,
      xml_child_slot_assign,
      TextChildParseArmOptions {
        repeated: field.repeated,
        as_result: true,
        use_local_name_dispatch: use_local_name_child_dispatch,
        list: field.list,
      },
    );
    child_visit_parse_tokens_borrowed.push(visit_parse_arm.clone());
    child_visit_parse_tokens_io.push(visit_parse_arm);
  }

  for field in &any_child_fields {
    let field_ident = &field.ident;
    let xml_child_slot = xml_child_slot_by_field
      .get(&field_ident.to_string())
      .copied()
      .unwrap_or_default();
    let xml_child_slot_assign = xml_child_slot_assign_tokens(xml_child_slot);
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
    child_write_tokens.push(build_any_child_write_tokens(
      field_ident,
      &field.qname,
      field.repeated,
      field.optional,
    ));
    child_parse_tokens_borrowed.push(build_any_child_parse_arm(
      ident,
      field_ident,
      &field.qname,
      field.repeated,
      false,
      DeserializeMode::Borrowed,
      xml_child_slot_assign.clone(),
    ));
    child_parse_tokens_io.push(build_any_child_parse_arm(
      ident,
      field_ident,
      &field.qname,
      field.repeated,
      false,
      DeserializeMode::Io,
      xml_child_slot_assign.clone(),
    ));
    child_visit_parse_tokens_borrowed.push(build_any_child_parse_arm(
      ident,
      field_ident,
      &field.qname,
      field.repeated,
      true,
      DeserializeMode::Borrowed,
      xml_child_slot_assign.clone(),
    ));
    child_visit_parse_tokens_io.push(build_any_child_parse_arm(
      ident,
      field_ident,
      &field.qname,
      field.repeated,
      true,
      DeserializeMode::Io,
      xml_child_slot_assign,
    ));
  }

  let mut choice_decl_tokens = Vec::new();
  let choice_match_init_tokens = Vec::<proc_macro2::TokenStream>::new();
  let choice_match_decl_tokens = Vec::<proc_macro2::TokenStream>::new();
  let mut choice_unique_parse_tokens_borrowed = Vec::new();
  let mut choice_unique_parse_tokens_io = Vec::new();
  let mut choice_unique_visit_parse_tokens_borrowed = Vec::new();
  let mut choice_unique_visit_parse_tokens_io = Vec::new();
  let mut choice_parse_tokens = Vec::new();
  let mut choice_visit_parse_tokens = Vec::new();
  let mut choice_write_tokens = Vec::new();
  let mut choice_init_tokens = Vec::new();
  let mut choice_text_parse_tokens = Vec::new();
  let mut choice_validate_tokens = Vec::new();
  let mut flat_choice_match_tokens_borrowed = Vec::new();
  let mut flat_choice_match_tokens_io = Vec::new();
  let mut flat_choice_visit_match_tokens_borrowed = Vec::new();
  let mut flat_choice_visit_match_tokens_io = Vec::new();
  let mut flat_choice_has_wildcard = false;
  let mut grouped_choice_match_tokens_borrowed =
    std::collections::BTreeMap::<String, Vec<GroupedChoiceAttempt>>::new();
  let mut grouped_choice_match_tokens_io =
    std::collections::BTreeMap::<String, Vec<GroupedChoiceAttempt>>::new();
  let mut grouped_choice_visit_match_tokens_borrowed =
    std::collections::BTreeMap::<String, Vec<GroupedChoiceAttempt>>::new();
  let mut grouped_choice_visit_match_tokens_io =
    std::collections::BTreeMap::<String, Vec<GroupedChoiceAttempt>>::new();
  let mut specific_choice_qname_counts = std::collections::HashMap::<String, usize>::new();
  for field in &choice_fields {
    if field.accepts_any.unwrap_or(false) {
      continue;
    }
    let mut seen = std::collections::HashSet::new();
    for qname in &field.specific_qnames {
      if seen.insert(qname) {
        *specific_choice_qname_counts
          .entry(qname.clone())
          .or_insert(0usize) += 1usize;
      }
    }
  }
  let required_child_by_ident = child_fields
    .iter()
    .filter(|field| !field.optional)
    .map(|field| (field.ident.to_string(), field.ident.clone()))
    .collect::<std::collections::HashMap<_, _>>();
  let mut required_before_choice = std::collections::HashMap::<String, Vec<Ident>>::new();
  let mut prior_required_children = Vec::<Ident>::new();
  for (field_ident, _, field_kind, _) in &ordered_field_specs {
    match field_kind {
      SdkTypeFieldKind::Choice => {
        required_before_choice.insert(field_ident.to_string(), prior_required_children.clone());
      }
      SdkTypeFieldKind::Child { .. } => {
        if let Some(required_ident) = required_child_by_ident.get(&field_ident.to_string()) {
          prior_required_children.push(required_ident.clone());
        }
      }
      _ => {}
    }
  }
  for field in &choice_fields {
    let field_ident = &field.ident;
    let xml_child_slot = xml_child_slot_by_field
      .get(&field_ident.to_string())
      .copied()
      .unwrap_or_default();
    let xml_child_slot_assign = xml_child_slot_assign_tokens(xml_child_slot);
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
      if field.accepts_text == Some(false) {
        return quote! {};
      }
      let text_variant = field.items.iter().find_map(|item| match item {
        SdkTypeChoiceItem::Text { variant } => Some(variant),
        _ => None,
      });
      let Some(text_variant) = text_variant else {
        return quote! {};
      };
      if field.repeated {
        quote! {
          #field_ident.push(#choice_ty::#text_variant(#string_expr.to_string()));
          #xml_child_slot_assign
          handled_text = true;
        }
      } else {
        quote! {
          if !handled_text {
            #field_ident = Some(#choice_ty::#text_variant(#string_expr.to_string()));
            #xml_child_slot_assign
            handled_text = true;
          }
        }
      }
    };
    let field_accepts_any = field.accepts_any.unwrap_or(false);
    flat_choice_has_wildcard |= field_accepts_any;
    let required_before = required_before_choice
      .get(&field_ident.to_string())
      .cloned()
      .unwrap_or_default();
    let choice_order_condition = if required_before.is_empty() {
      None
    } else {
      Some(quote! { #( #required_before.is_some() )&&* })
    };
    let specific_qnames: Vec<_> = field
      .specific_qnames
      .iter()
      .filter(|qname| !qname.is_empty())
      .cloned()
      .collect();
    let flatten_choice_items = !field.items.is_empty()
      && (field_accepts_any
        || specific_qnames.iter().all(|qname| {
          specific_choice_qname_counts
            .get(qname)
            .copied()
            .unwrap_or_default()
            == 1usize
        }));
    if flatten_choice_items {
      let mut flat_choice_local_name_qnames =
        std::collections::HashMap::<String, Vec<String>>::new();
      for qname in &specific_qnames {
        let QNameInfo { local_name, .. } = parse_qname_info(qname);
        let qnames = flat_choice_local_name_qnames.entry(local_name).or_default();
        if !qnames.iter().any(|existing| existing == qname) {
          qnames.push(qname.clone());
        }
      }
      let flat_choice_local_name_fallback_qnames = flat_choice_local_name_qnames
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
        .collect::<std::collections::HashSet<_>>();
      let flat_choice_match_targets = |qnames: &[String]| {
        let mut seen = std::collections::HashSet::new();
        let mut targets = Vec::new();

        for qname in qnames {
          let QNameInfo {
            tag_prefix,
            local_name,
          } = parse_qname_info(qname);
          if !tag_prefix.is_empty() {
            let prefixed = format!("{tag_prefix}:{local_name}");
            if seen.insert(prefixed.clone()) {
              let tag_qname_lit = LitByteStr::new(prefixed.as_bytes(), Span::call_site());
              targets.push(quote! { #tag_qname_lit });
            }
          }
          if flat_choice_local_name_fallback_qnames.contains(qname)
            && seen.insert(local_name.to_string())
          {
            let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
            targets.push(quote! { #local_name_lit });
          }
        }

        targets
      };
      for item in &field.items {
        match item {
          SdkTypeChoiceItem::Child { variant, qname, .. } => {
            let targets = flat_choice_match_targets(std::slice::from_ref(qname));
            flat_choice_match_tokens_borrowed.push(quote! {
              #( #targets )|* => {
                let parsed_choice = #choice_ty::#variant(std::boxed::Box::new(
                  <_ as crate::sdk::SdkType>::read_borrowed_inner(xml_reader, e, next_empty)?
                ));
                #field_ident.push(parsed_choice);
                #xml_child_slot_assign
                continue;
              }
            });
            if !field.repeated {
              flat_choice_match_tokens_borrowed.pop();
              flat_choice_match_tokens_borrowed.push(quote! {
                #( #targets )|* => {
                  let parsed_choice = #choice_ty::#variant(std::boxed::Box::new(
                    <_ as crate::sdk::SdkType>::read_borrowed_inner(xml_reader, e, next_empty)?
                  ));
                  #field_ident = Some(parsed_choice);
                  #xml_child_slot_assign
                  continue;
                }
              });
            }
            flat_choice_match_tokens_io.push(quote! {
              #( #targets )|* => {
                let parsed_choice = #choice_ty::#variant(std::boxed::Box::new(
                  <_ as crate::sdk::SdkType>::read_io_inner(xml_reader, e, next_empty)?
                ));
                #field_ident.push(parsed_choice);
                #xml_child_slot_assign
                continue;
              }
            });
            if !field.repeated {
              flat_choice_match_tokens_io.pop();
              flat_choice_match_tokens_io.push(quote! {
                #( #targets )|* => {
                  let parsed_choice = #choice_ty::#variant(std::boxed::Box::new(
                    <_ as crate::sdk::SdkType>::read_io_inner(xml_reader, e, next_empty)?
                  ));
                  #field_ident = Some(parsed_choice);
                  #xml_child_slot_assign
                  continue;
                }
              });
            }
            flat_choice_visit_match_tokens_borrowed.push(quote! {
              #( #targets )|* => {
                let parsed_choice = #choice_ty::#variant(std::boxed::Box::new(
                  <_ as crate::sdk::SdkType>::read_borrowed_inner(xml_reader, e, next_empty)?
                ));
                #field_ident.push(parsed_choice);
                #xml_child_slot_assign
                return Ok(true);
              }
            });
            if !field.repeated {
              flat_choice_visit_match_tokens_borrowed.pop();
              flat_choice_visit_match_tokens_borrowed.push(quote! {
                #( #targets )|* => {
                  let parsed_choice = #choice_ty::#variant(std::boxed::Box::new(
                    <_ as crate::sdk::SdkType>::read_borrowed_inner(xml_reader, e, next_empty)?
                  ));
                  #field_ident = Some(parsed_choice);
                  #xml_child_slot_assign
                  return Ok(true);
                }
              });
            }
            flat_choice_visit_match_tokens_io.push(quote! {
              #( #targets )|* => {
                let parsed_choice = #choice_ty::#variant(std::boxed::Box::new(
                  <_ as crate::sdk::SdkType>::read_io_inner(xml_reader, e, next_empty)?
                ));
                #field_ident.push(parsed_choice);
                #xml_child_slot_assign
                return Ok(true);
              }
            });
            if !field.repeated {
              flat_choice_visit_match_tokens_io.pop();
              flat_choice_visit_match_tokens_io.push(quote! {
                #( #targets )|* => {
                  let parsed_choice = #choice_ty::#variant(std::boxed::Box::new(
                    <_ as crate::sdk::SdkType>::read_io_inner(xml_reader, e, next_empty)?
                  ));
                  #field_ident = Some(parsed_choice);
                  #xml_child_slot_assign
                  return Ok(true);
                }
              });
            }
          }
          SdkTypeChoiceItem::EmptyChild { variant, qname } => {
            let targets = flat_choice_match_targets(std::slice::from_ref(qname));
            let skip_tokens_borrowed = build_empty_child_skip_tokens(
              ident,
              qname,
              DeserializeMode::Borrowed,
              &xml_reader_ident,
            );
            let skip_tokens_io =
              build_empty_child_skip_tokens(ident, qname, DeserializeMode::Io, &xml_reader_ident);
            let assign_tokens = if field.repeated {
              quote! { #field_ident.push(#choice_ty::#variant); }
            } else {
              quote! { #field_ident = Some(#choice_ty::#variant); }
            };
            flat_choice_match_tokens_borrowed.push(quote! {
              #( #targets )|* => {
                #skip_tokens_borrowed
                #assign_tokens
                #xml_child_slot_assign
                continue;
              }
            });
            flat_choice_match_tokens_io.push(quote! {
              #( #targets )|* => {
                #skip_tokens_io
                #assign_tokens
                #xml_child_slot_assign
                continue;
              }
            });
            flat_choice_visit_match_tokens_borrowed.push(quote! {
              #( #targets )|* => {
                #skip_tokens_borrowed
                #assign_tokens
                #xml_child_slot_assign
                return Ok(true);
              }
            });
            flat_choice_visit_match_tokens_io.push(quote! {
              #( #targets )|* => {
                #skip_tokens_io
                #assign_tokens
                #xml_child_slot_assign
                return Ok(true);
              }
            });
          }
          SdkTypeChoiceItem::TextChild { variant, qname, .. } => {
            let targets = flat_choice_match_targets(std::slice::from_ref(qname));
            let parsed_tokens = quote! {
              let parsed_child = if next_empty {
                crate::common::parse_value::<_>("", stringify!(#ident), stringify!(#variant))?
              } else {
                let value = xml_reader.read_text(e.name())?;
                crate::common::parse_value::<_>(value.as_ref(), stringify!(#ident), stringify!(#variant))?
              };
            };
            let assign_tokens = if field.repeated {
              quote! { #field_ident.push(#choice_ty::#variant(parsed_child)); }
            } else {
              quote! { #field_ident = Some(#choice_ty::#variant(parsed_child)); }
            };
            flat_choice_match_tokens_borrowed.push(quote! {
              #( #targets )|* => {
                #parsed_tokens
                #assign_tokens
                #xml_child_slot_assign
                continue;
              }
            });
            flat_choice_match_tokens_io.push(quote! {
              #( #targets )|* => {
                #parsed_tokens
                #assign_tokens
                #xml_child_slot_assign
                continue;
              }
            });
            flat_choice_visit_match_tokens_borrowed.push(quote! {
              #( #targets )|* => {
                #parsed_tokens
                #assign_tokens
                #xml_child_slot_assign
                return Ok(true);
              }
            });
            flat_choice_visit_match_tokens_io.push(quote! {
              #( #targets )|* => {
                #parsed_tokens
                #assign_tokens
                #xml_child_slot_assign
                return Ok(true);
              }
            });
          }
          SdkTypeChoiceItem::AnyChild { variant, qname } => {
            let targets = flat_choice_match_targets(std::slice::from_ref(qname));
            let QNameInfo {
              tag_prefix,
              local_name,
            } = parse_qname_info(qname);
            let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
            let tag_qname_lit = LitByteStr::new(
              format!("{tag_prefix}:{local_name}").as_bytes(),
              Span::call_site(),
            );
            let parsed_borrowed_tokens = quote! {
              let mut parsed_child = Vec::new();
              if !next_empty {
                loop {
                  match xml_reader.next()? {
                    quick_xml::events::Event::Start(e) => {
                      let xml = crate::common::read_raw_element_xml_borrowed(xml_reader, e)?;
                      parsed_child.push(xml);
                    }
                    quick_xml::events::Event::Empty(e) => {
                      let xml = crate::common::read_raw_empty_xml_borrowed(e)?;
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
            };
            let parsed_io_tokens = quote! {
              let mut parsed_child = Vec::new();
              if !next_empty {
                loop {
                  match xml_reader.next()? {
                    quick_xml::events::Event::Start(e) => {
                      let xml = crate::common::read_raw_element_xml_io(xml_reader, e)?;
                      parsed_child.push(xml);
                    }
                    quick_xml::events::Event::Empty(e) => {
                      let xml = crate::common::read_raw_empty_xml_io(e)?;
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
            };
            let assign_tokens = if field.repeated {
              quote! { #field_ident.push(#choice_ty::#variant(parsed_child.into())); }
            } else {
              quote! { #field_ident = Some(#choice_ty::#variant(parsed_child.into())); }
            };
            flat_choice_match_tokens_borrowed.push(quote! {
              #( #targets )|* => {
                #parsed_borrowed_tokens
                #assign_tokens
                #xml_child_slot_assign
                continue;
              }
            });
            flat_choice_match_tokens_io.push(quote! {
              #( #targets )|* => {
                #parsed_io_tokens
                #assign_tokens
                #xml_child_slot_assign
                continue;
              }
            });
            flat_choice_visit_match_tokens_borrowed.push(quote! {
              #( #targets )|* => {
                #parsed_borrowed_tokens
                #assign_tokens
                #xml_child_slot_assign
                return Ok(true);
              }
            });
            flat_choice_visit_match_tokens_io.push(quote! {
              #( #targets )|* => {
                #parsed_io_tokens
                #assign_tokens
                #xml_child_slot_assign
                return Ok(true);
              }
            });
          }
          SdkTypeChoiceItem::Sequence { variant, children } => {
            let occupied_qnames = field
              .items
              .iter()
              .filter_map(|item| match item {
                SdkTypeChoiceItem::Child { qname, .. }
                | SdkTypeChoiceItem::EmptyChild { qname, .. }
                | SdkTypeChoiceItem::TextChild { qname, .. }
                | SdkTypeChoiceItem::AnyChild { qname, .. } => Some(qname),
                SdkTypeChoiceItem::Sequence { .. }
                | SdkTypeChoiceItem::Any { .. }
                | SdkTypeChoiceItem::Text { .. } => None,
              })
              .collect::<Vec<_>>();
            let sequence_qnames = children
              .iter()
              .filter(|child| !occupied_qnames.contains(&&child.qname))
              .map(|child| child.qname.clone())
              .collect::<Vec<_>>();
            if sequence_qnames.is_empty() {
              continue;
            }
            let targets = flat_choice_match_targets(&sequence_qnames);
            let assign_tokens = if field.repeated {
              quote! { #field_ident.push(#choice_ty::#variant(std::boxed::Box::new(parsed_child))); }
            } else {
              quote! { #field_ident = Some(#choice_ty::#variant(std::boxed::Box::new(parsed_child))); }
            };
            let named_sequence = children
              .iter()
              .all(|child| child.field.is_some() && child.ty.is_some());
            if named_sequence {
              let field_decls = children
                .iter()
                .map(|child| {
                  let field_ident = child.field.as_ref().expect("sequence field");
                  let field_ty = child.ty.as_ref().expect("sequence field ty");
                  if is_option_type(field_ty) {
                    quote! { let mut #field_ident: #field_ty = None; }
                  } else {
                    quote! { let mut #field_ident: Option<#field_ty> = None; }
                  }
                })
                .collect::<Vec<_>>();
              let final_fields = children
                .iter()
                .map(|child| {
                  let field_ident = child.field.as_ref().expect("sequence field");
                  let field_ty = child.ty.as_ref().expect("sequence field ty");
                  if is_option_type(field_ty) {
                    quote! {}
                  } else {
                    quote! {
                      let #field_ident = #field_ident.ok_or_else(|| {
                        crate::common::missing_field(stringify!(#ident), stringify!(#field_ident))
                      })?;
                    }
                  }
                })
                .collect::<Vec<_>>();
              let variant_fields = children
                .iter()
                .map(|child| child.field.as_ref().expect("sequence field"))
                .collect::<Vec<_>>();
              let build_sequence_child_arms = |mode: DeserializeMode| {
                let read_inner_ident = mode.deserialize_inner_ident();
                children
                  .iter()
                  .map(|child| {
                    let field_ident = child.field.as_ref().expect("sequence field");
                    let field_ty = child.ty.as_ref().expect("sequence field ty");
                    let value_ty = if is_option_type(field_ty) {
                      unwrap_option_type(field_ty)
                    } else {
                      field_ty.clone()
                    };
                    let targets = flat_choice_match_targets(std::slice::from_ref(&child.qname));
                    match &child.kind {
                      SdkTypeChoiceSequenceChildKind::Child => {
                        let parsed_tokens = if let Some(inner_ty) = box_inner_type(&value_ty) {
                          quote! {
                            let parsed_value = std::boxed::Box::new(
                              <#inner_ty as crate::sdk::SdkType>::#read_inner_ident(xml_reader, e, next_empty)?
                            );
                          }
                        } else {
                          quote! {
                            let parsed_value = <#value_ty as crate::sdk::SdkType>::#read_inner_ident(
                              xml_reader,
                              e,
                              next_empty,
                            )?;
                          }
                        };
                        quote! {
                          #( #targets )|* => {
                            #parsed_tokens
                            #field_ident = Some(parsed_value);
                            continue;
                          }
                        }
                      }
                      SdkTypeChoiceSequenceChildKind::TextChild => {
                        quote! {
                          #( #targets )|* => {
                            let parsed_value = if next_empty {
                              crate::common::parse_value::<#value_ty>("", stringify!(#ident), stringify!(#field_ident))?
                            } else {
                              let value = xml_reader.read_text(e.name())?;
                              crate::common::parse_value::<#value_ty>(value.as_ref(), stringify!(#ident), stringify!(#field_ident))?
                            };
                            #field_ident = Some(parsed_value);
                            continue;
                          }
                        }
                      }
                      SdkTypeChoiceSequenceChildKind::EmptyChild
                      | SdkTypeChoiceSequenceChildKind::AnyChild => quote! {},
                    }
                  })
                  .collect::<Vec<_>>()
              };
              let borrowed_child_arms = build_sequence_child_arms(DeserializeMode::Borrowed);
              let io_child_arms = build_sequence_child_arms(DeserializeMode::Io);
              let assign_named_tokens = if field.repeated {
                quote! { #field_ident.push(parsed_choice); }
              } else {
                quote! { #field_ident = Some(parsed_choice); }
              };
              let borrowed_sequence_tokens = quote! {
                #( #field_decls )*
                let mut pending_event = Some((e, next_empty));
                loop {
                  if let Some((e, next_empty)) = pending_event.take() {
                    let event_name = e.name().into_inner();
                    match event_name {
                      #( #borrowed_child_arms )*
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
                  match xml_reader.next_tag_event()? {
                    crate::common::SliceTagEvent::Start(e, next_empty) => {
                      pending_event = Some((e, next_empty));
                    }
                    crate::common::SliceTagEvent::End(e) => {
                      xml_reader.unread(quick_xml::events::Event::End(e))?;
                      break;
                    }
                    crate::common::SliceTagEvent::Eof => {
                      return Err(crate::common::unexpected_eof(stringify!(#ident)));
                    }
                    crate::common::SliceTagEvent::Decl(_)
                    | crate::common::SliceTagEvent::Other => {}
                  }
                }
                #( #final_fields )*
                let parsed_choice = #choice_ty::#variant { #( #variant_fields ),* };
              };
              let io_sequence_tokens = quote! {
                #( #field_decls )*
                let mut pending_event = Some((e, next_empty));
                loop {
                  if let Some((e, next_empty)) = pending_event.take() {
                    let event_name = e.name().into_inner();
                    match event_name {
                      #( #io_child_arms )*
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
                    crate::common::IoTagEvent::Decl(_)
                    | crate::common::IoTagEvent::Other => {}
                  }
                }
                #( #final_fields )*
                let parsed_choice = #choice_ty::#variant { #( #variant_fields ),* };
              };
              flat_choice_match_tokens_borrowed.push(quote! {
                #( #targets )|* => {
                  #borrowed_sequence_tokens
                  #assign_named_tokens
                  #xml_child_slot_assign
                  continue;
                }
              });
              flat_choice_match_tokens_io.push(quote! {
                #( #targets )|* => {
                  #io_sequence_tokens
                  #assign_named_tokens
                  #xml_child_slot_assign
                  continue;
                }
              });
              flat_choice_visit_match_tokens_borrowed.push(quote! {
                #( #targets )|* => {
                  #borrowed_sequence_tokens
                  #assign_named_tokens
                  #xml_child_slot_assign
                  return Ok(true);
                }
              });
              flat_choice_visit_match_tokens_io.push(quote! {
                #( #targets )|* => {
                  #io_sequence_tokens
                  #assign_named_tokens
                  #xml_child_slot_assign
                  return Ok(true);
                }
              });
              continue;
            }
            flat_choice_match_tokens_borrowed.push(quote! {
              #( #targets )|* => {
                let parsed_child = <_ as crate::sdk::SdkType>::read_borrowed_inner(xml_reader, e, next_empty)?;
                #assign_tokens
                #xml_child_slot_assign
                continue;
              }
            });
            flat_choice_match_tokens_io.push(quote! {
              #( #targets )|* => {
                let parsed_child = <_ as crate::sdk::SdkType>::read_io_inner(xml_reader, e, next_empty)?;
                #assign_tokens
                #xml_child_slot_assign
                continue;
              }
            });
            flat_choice_visit_match_tokens_borrowed.push(quote! {
              #( #targets )|* => {
                let parsed_child = <_ as crate::sdk::SdkType>::read_borrowed_inner(xml_reader, e, next_empty)?;
                #assign_tokens
                #xml_child_slot_assign
                return Ok(true);
              }
            });
            flat_choice_visit_match_tokens_io.push(quote! {
              #( #targets )|* => {
                let parsed_child = <_ as crate::sdk::SdkType>::read_io_inner(xml_reader, e, next_empty)?;
                #assign_tokens
                #xml_child_slot_assign
                return Ok(true);
              }
            });
          }
          SdkTypeChoiceItem::Any { variant } => {
            flat_choice_has_wildcard = true;
            let assign_tokens = if field.repeated {
              quote! { #field_ident.push(#choice_ty::#variant(xml)); }
            } else {
              quote! { #field_ident = Some(#choice_ty::#variant(xml)); }
            };
            flat_choice_match_tokens_borrowed.push(quote! {
              _ => {
                let xml = if next_empty {
                  crate::common::read_raw_empty_xml_borrowed_bytes(e)?
                } else {
                  crate::common::read_raw_element_xml_borrowed_bytes(xml_reader, e)?
                };
                #assign_tokens
                #xml_child_slot_assign
                continue;
              }
            });
            flat_choice_match_tokens_io.push(quote! {
              _ => {
                let xml = if next_empty {
                  crate::common::read_raw_empty_xml_io_bytes(e)?
                } else {
                  crate::common::read_raw_element_xml_io_bytes(xml_reader, e)?
                };
                #assign_tokens
                #xml_child_slot_assign
                continue;
              }
            });
            flat_choice_visit_match_tokens_borrowed.push(quote! {
              _ => {
                let xml = if next_empty {
                  crate::common::read_raw_empty_xml_borrowed_bytes(e)?
                } else {
                  crate::common::read_raw_element_xml_borrowed_bytes(xml_reader, e)?
                };
                #assign_tokens
                #xml_child_slot_assign
                return Ok(true);
              }
            });
            flat_choice_visit_match_tokens_io.push(quote! {
              _ => {
                let xml = if next_empty {
                  crate::common::read_raw_empty_xml_io_bytes(e)?
                } else {
                  crate::common::read_raw_element_xml_io_bytes(xml_reader, e)?
                };
                #assign_tokens
                #xml_child_slot_assign
                return Ok(true);
              }
            });
          }
          SdkTypeChoiceItem::Text { .. } => {}
        }
      }
    }
    let mut grouped_choice_items = false;
    if !flatten_choice_items && !field.items.is_empty() && !field_accepts_any {
      let mut grouped_choice_local_name_qnames =
        std::collections::HashMap::<String, Vec<String>>::new();
      for qname in &specific_qnames {
        let QNameInfo { local_name, .. } = parse_qname_info(qname);
        let qnames = grouped_choice_local_name_qnames
          .entry(local_name)
          .or_default();
        if !qnames.iter().any(|existing| existing == qname) {
          qnames.push(qname.clone());
        }
      }
      let grouped_choice_local_name_fallback_qnames = grouped_choice_local_name_qnames
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
        .collect::<std::collections::HashSet<_>>();
      let grouped_choice_match_target_keys = |qnames: &[String]| {
        let mut seen = std::collections::HashSet::new();
        let mut targets = Vec::new();

        for qname in qnames {
          let QNameInfo {
            tag_prefix,
            local_name,
          } = parse_qname_info(qname);
          if !tag_prefix.is_empty() {
            let prefixed = format!("{tag_prefix}:{local_name}");
            if seen.insert(prefixed.clone()) {
              targets.push(prefixed);
            }
          }
          if grouped_choice_local_name_fallback_qnames.contains(qname)
            && seen.insert(local_name.to_string())
          {
            targets.push(local_name);
          }
        }

        targets
      };
      let mut push_grouped_choice_attempts =
        |targets: Vec<String>,
         borrowed_tokens: proc_macro2::TokenStream,
         io_tokens: proc_macro2::TokenStream,
         borrowed_visit_tokens: proc_macro2::TokenStream,
         io_visit_tokens: proc_macro2::TokenStream| {
          if targets.is_empty() {
            return;
          }
          grouped_choice_items = true;
          for target in targets {
            grouped_choice_match_tokens_borrowed
              .entry(target.clone())
              .or_default()
              .push(GroupedChoiceAttempt {
                condition: choice_order_condition.clone(),
                tokens: borrowed_tokens.clone(),
              });
            grouped_choice_match_tokens_io
              .entry(target.clone())
              .or_default()
              .push(GroupedChoiceAttempt {
                condition: choice_order_condition.clone(),
                tokens: io_tokens.clone(),
              });
            grouped_choice_visit_match_tokens_borrowed
              .entry(target.clone())
              .or_default()
              .push(GroupedChoiceAttempt {
                condition: choice_order_condition.clone(),
                tokens: borrowed_visit_tokens.clone(),
              });
            grouped_choice_visit_match_tokens_io
              .entry(target)
              .or_default()
              .push(GroupedChoiceAttempt {
                condition: choice_order_condition.clone(),
                tokens: io_visit_tokens.clone(),
              });
          }
        };

      for item in &field.items {
        match item {
          SdkTypeChoiceItem::Child { variant, qname, .. } => {
            let targets = grouped_choice_match_target_keys(std::slice::from_ref(qname));
            let borrowed_parse = quote! {
              let parsed_choice = #choice_ty::#variant(std::boxed::Box::new(
                <_ as crate::sdk::SdkType>::read_borrowed_inner(xml_reader, e, next_empty)?
              ));
              #field_ident.push(parsed_choice);
              #xml_child_slot_assign
            };
            let borrowed_parse = if field.repeated {
              borrowed_parse
            } else {
              quote! {
                let parsed_choice = #choice_ty::#variant(std::boxed::Box::new(
                  <_ as crate::sdk::SdkType>::read_borrowed_inner(xml_reader, e, next_empty)?
                ));
                #field_ident = Some(parsed_choice);
                #xml_child_slot_assign
              }
            };
            let io_parse = quote! {
              let parsed_choice = #choice_ty::#variant(std::boxed::Box::new(
                <_ as crate::sdk::SdkType>::read_io_inner(xml_reader, e, next_empty)?
              ));
              #field_ident.push(parsed_choice);
              #xml_child_slot_assign
            };
            let io_parse = if field.repeated {
              io_parse
            } else {
              quote! {
                let parsed_choice = #choice_ty::#variant(std::boxed::Box::new(
                  <_ as crate::sdk::SdkType>::read_io_inner(xml_reader, e, next_empty)?
                ));
                #field_ident = Some(parsed_choice);
                #xml_child_slot_assign
              }
            };
            push_grouped_choice_attempts(
              targets,
              borrowed_parse.clone(),
              io_parse.clone(),
              borrowed_parse,
              io_parse,
            );
          }
          SdkTypeChoiceItem::EmptyChild { variant, qname } => {
            let targets = grouped_choice_match_target_keys(std::slice::from_ref(qname));
            let skip_tokens_borrowed = build_empty_child_skip_tokens(
              ident,
              qname,
              DeserializeMode::Borrowed,
              &xml_reader_ident,
            );
            let skip_tokens_io =
              build_empty_child_skip_tokens(ident, qname, DeserializeMode::Io, &xml_reader_ident);
            let assign_tokens = if field.repeated {
              quote! { #field_ident.push(#choice_ty::#variant); }
            } else {
              quote! { #field_ident = Some(#choice_ty::#variant); }
            };
            let borrowed_parse = quote! {
              #skip_tokens_borrowed
              #assign_tokens
              #xml_child_slot_assign
            };
            let io_parse = quote! {
              #skip_tokens_io
              #assign_tokens
              #xml_child_slot_assign
            };
            push_grouped_choice_attempts(
              targets,
              borrowed_parse.clone(),
              io_parse.clone(),
              borrowed_parse,
              io_parse,
            );
          }
          SdkTypeChoiceItem::TextChild { variant, qname, .. } => {
            let targets = grouped_choice_match_target_keys(std::slice::from_ref(qname));
            let parsed_tokens = quote! {
              let parsed_child = if next_empty {
                crate::common::parse_value::<_>("", stringify!(#ident), stringify!(#variant))?
              } else {
                let value = xml_reader.read_text(e.name())?;
                crate::common::parse_value::<_>(value.as_ref(), stringify!(#ident), stringify!(#variant))?
              };
            };
            let assign_tokens = if field.repeated {
              quote! { #field_ident.push(#choice_ty::#variant(parsed_child)); }
            } else {
              quote! { #field_ident = Some(#choice_ty::#variant(parsed_child)); }
            };
            let parse_tokens = quote! {
              #parsed_tokens
              #assign_tokens
              #xml_child_slot_assign
            };
            push_grouped_choice_attempts(
              targets,
              parse_tokens.clone(),
              parse_tokens.clone(),
              parse_tokens.clone(),
              parse_tokens,
            );
          }
          SdkTypeChoiceItem::AnyChild { variant, qname } => {
            let targets = grouped_choice_match_target_keys(std::slice::from_ref(qname));
            let QNameInfo {
              tag_prefix,
              local_name,
            } = parse_qname_info(qname);
            let local_name_lit = LitByteStr::new(local_name.as_bytes(), Span::call_site());
            let tag_qname_lit = LitByteStr::new(
              format!("{tag_prefix}:{local_name}").as_bytes(),
              Span::call_site(),
            );
            let parsed_borrowed_tokens = quote! {
              let mut parsed_child = Vec::new();
              if !next_empty {
                loop {
                  match xml_reader.next()? {
                    quick_xml::events::Event::Start(e) => {
                      let xml = crate::common::read_raw_element_xml_borrowed(xml_reader, e)?;
                      parsed_child.push(xml);
                    }
                    quick_xml::events::Event::Empty(e) => {
                      let xml = crate::common::read_raw_empty_xml_borrowed(e)?;
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
            };
            let parsed_io_tokens = quote! {
              let mut parsed_child = Vec::new();
              if !next_empty {
                loop {
                  match xml_reader.next()? {
                    quick_xml::events::Event::Start(e) => {
                      let xml = crate::common::read_raw_element_xml_io(xml_reader, e)?;
                      parsed_child.push(xml);
                    }
                    quick_xml::events::Event::Empty(e) => {
                      let xml = crate::common::read_raw_empty_xml_io(e)?;
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
            };
            let assign_tokens = if field.repeated {
              quote! { #field_ident.push(#choice_ty::#variant(parsed_child.into())); }
            } else {
              quote! { #field_ident = Some(#choice_ty::#variant(parsed_child.into())); }
            };
            let borrowed_parse = quote! {
              #parsed_borrowed_tokens
              #assign_tokens
              #xml_child_slot_assign
            };
            let io_parse = quote! {
              #parsed_io_tokens
              #assign_tokens
              #xml_child_slot_assign
            };
            push_grouped_choice_attempts(
              targets,
              borrowed_parse.clone(),
              io_parse.clone(),
              borrowed_parse,
              io_parse,
            );
          }
          SdkTypeChoiceItem::Sequence { variant, children } => {
            let occupied_qnames = field
              .items
              .iter()
              .filter_map(|item| match item {
                SdkTypeChoiceItem::Child { qname, .. }
                | SdkTypeChoiceItem::EmptyChild { qname, .. }
                | SdkTypeChoiceItem::TextChild { qname, .. }
                | SdkTypeChoiceItem::AnyChild { qname, .. } => Some(qname),
                SdkTypeChoiceItem::Sequence { .. }
                | SdkTypeChoiceItem::Any { .. }
                | SdkTypeChoiceItem::Text { .. } => None,
              })
              .collect::<Vec<_>>();
            let sequence_qnames = children
              .iter()
              .filter(|child| !occupied_qnames.contains(&&child.qname))
              .map(|child| child.qname.clone())
              .collect::<Vec<_>>();
            if sequence_qnames.is_empty() {
              continue;
            }
            let targets = grouped_choice_match_target_keys(&sequence_qnames);
            let assign_tokens = if field.repeated {
              quote! { #field_ident.push(#choice_ty::#variant(std::boxed::Box::new(parsed_child))); }
            } else {
              quote! { #field_ident = Some(#choice_ty::#variant(std::boxed::Box::new(parsed_child))); }
            };
            let named_sequence = children
              .iter()
              .all(|child| child.field.is_some() && child.ty.is_some());
            if named_sequence {
              let field_decls = children
                .iter()
                .map(|child| {
                  let field_ident = child.field.as_ref().expect("sequence field");
                  let field_ty = child.ty.as_ref().expect("sequence field ty");
                  if is_option_type(field_ty) {
                    quote! { let mut #field_ident: #field_ty = None; }
                  } else {
                    quote! { let mut #field_ident: Option<#field_ty> = None; }
                  }
                })
                .collect::<Vec<_>>();
              let final_fields = children
                .iter()
                .map(|child| {
                  let field_ident = child.field.as_ref().expect("sequence field");
                  let field_ty = child.ty.as_ref().expect("sequence field ty");
                  if is_option_type(field_ty) {
                    quote! {}
                  } else {
                    quote! {
                      let #field_ident = #field_ident.ok_or_else(|| {
                        crate::common::missing_field(stringify!(#ident), stringify!(#field_ident))
                      })?;
                    }
                  }
                })
                .collect::<Vec<_>>();
              let variant_fields = children
                .iter()
                .map(|child| child.field.as_ref().expect("sequence field"))
                .collect::<Vec<_>>();
              let build_sequence_child_arms = |mode: DeserializeMode| {
                let read_inner_ident = mode.deserialize_inner_ident();
                children
                  .iter()
                  .map(|child| {
                    let field_ident = child.field.as_ref().expect("sequence field");
                    let field_ty = child.ty.as_ref().expect("sequence field ty");
                    let value_ty = if is_option_type(field_ty) {
                      unwrap_option_type(field_ty)
                    } else {
                      field_ty.clone()
                    };
                    let targets = grouped_choice_match_target_keys(std::slice::from_ref(&child.qname));
                    match &child.kind {
                      SdkTypeChoiceSequenceChildKind::Child => {
                        let parsed_tokens = if let Some(inner_ty) = box_inner_type(&value_ty) {
                          quote! {
                            let parsed_value = std::boxed::Box::new(
                              <#inner_ty as crate::sdk::SdkType>::#read_inner_ident(xml_reader, e, next_empty)?
                            );
                          }
                        } else {
                          quote! {
                            let parsed_value = <#value_ty as crate::sdk::SdkType>::#read_inner_ident(
                              xml_reader,
                              e,
                              next_empty,
                            )?;
                          }
                        };
                        quote! {
                          #( #targets )|* => {
                            #parsed_tokens
                            #field_ident = Some(parsed_value);
                            continue;
                          }
                        }
                      }
                      SdkTypeChoiceSequenceChildKind::TextChild => {
                        quote! {
                          #( #targets )|* => {
                            let parsed_value = if next_empty {
                              crate::common::parse_value::<#value_ty>("", stringify!(#ident), stringify!(#field_ident))?
                            } else {
                              let value = xml_reader.read_text(e.name())?;
                              crate::common::parse_value::<#value_ty>(value.as_ref(), stringify!(#ident), stringify!(#field_ident))?
                            };
                            #field_ident = Some(parsed_value);
                            continue;
                          }
                        }
                      }
                      SdkTypeChoiceSequenceChildKind::EmptyChild
                      | SdkTypeChoiceSequenceChildKind::AnyChild => quote! {},
                    }
                  })
                  .collect::<Vec<_>>()
              };
              let borrowed_child_arms = build_sequence_child_arms(DeserializeMode::Borrowed);
              let io_child_arms = build_sequence_child_arms(DeserializeMode::Io);
              let assign_named_tokens = if field.repeated {
                quote! { #field_ident.push(parsed_choice); }
              } else {
                quote! { #field_ident = Some(parsed_choice); }
              };
              let borrowed_sequence_tokens = quote! {
                #( #field_decls )*
                let mut pending_event = Some((e, next_empty));
                loop {
                  if let Some((e, next_empty)) = pending_event.take() {
                    let event_name = e.name().into_inner();
                    match event_name {
                      #( #borrowed_child_arms )*
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
                  match xml_reader.next_tag_event()? {
                    crate::common::SliceTagEvent::Start(e, next_empty) => {
                      pending_event = Some((e, next_empty));
                    }
                    crate::common::SliceTagEvent::End(e) => {
                      xml_reader.unread(quick_xml::events::Event::End(e))?;
                      break;
                    }
                    crate::common::SliceTagEvent::Eof => {
                      return Err(crate::common::unexpected_eof(stringify!(#ident)));
                    }
                    crate::common::SliceTagEvent::Decl(_)
                    | crate::common::SliceTagEvent::Other => {}
                  }
                }
                #( #final_fields )*
                let parsed_choice = #choice_ty::#variant { #( #variant_fields ),* };
              };
              let io_sequence_tokens = quote! {
                #( #field_decls )*
                let mut pending_event = Some((e, next_empty));
                loop {
                  if let Some((e, next_empty)) = pending_event.take() {
                    let event_name = e.name().into_inner();
                    match event_name {
                      #( #io_child_arms )*
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
                    crate::common::IoTagEvent::Decl(_)
                    | crate::common::IoTagEvent::Other => {}
                  }
                }
                #( #final_fields )*
                let parsed_choice = #choice_ty::#variant { #( #variant_fields ),* };
              };
              let borrowed_parse = quote! {
                #borrowed_sequence_tokens
                #assign_named_tokens
                #xml_child_slot_assign
              };
              let io_parse = quote! {
                #io_sequence_tokens
                #assign_named_tokens
                #xml_child_slot_assign
              };
              push_grouped_choice_attempts(
                targets,
                borrowed_parse.clone(),
                io_parse.clone(),
                borrowed_parse,
                io_parse,
              );
              continue;
            }
            let borrowed_parse = quote! {
              let parsed_child = <_ as crate::sdk::SdkType>::read_borrowed_inner(xml_reader, e, next_empty)?;
              #assign_tokens
              #xml_child_slot_assign
            };
            let io_parse = quote! {
              let parsed_child = <_ as crate::sdk::SdkType>::read_io_inner(xml_reader, e, next_empty)?;
              #assign_tokens
              #xml_child_slot_assign
            };
            push_grouped_choice_attempts(
              targets,
              borrowed_parse.clone(),
              io_parse.clone(),
              borrowed_parse,
              io_parse,
            );
          }
          SdkTypeChoiceItem::Any { .. } | SdkTypeChoiceItem::Text { .. } => {}
        }
      }
    }
    let flatten_specific_choice = !flatten_choice_items
      && !grouped_choice_items
      && !field_accepts_any
      && !specific_qnames.is_empty()
      && specific_qnames.iter().all(|qname| {
        specific_choice_qname_counts
          .get(qname)
          .copied()
          .unwrap_or_default()
          == 1usize
      });
    if flatten_specific_choice {
      let (borrowed_parse_tokens, io_parse_tokens, borrowed_visit_tokens, io_visit_tokens) =
        build_flat_choice_dispatch_match_tokens(
          field_ident,
          &choice_ty,
          field.repeated,
          &specific_qnames,
          xml_child_slot_assign.clone(),
        );
      flat_choice_match_tokens_borrowed.push(borrowed_parse_tokens);
      flat_choice_match_tokens_io.push(io_parse_tokens);
      flat_choice_visit_match_tokens_borrowed.push(borrowed_visit_tokens);
      flat_choice_visit_match_tokens_io.push(io_visit_tokens);
    }
    if flatten_choice_items || grouped_choice_items || flatten_specific_choice {
    } else if !field_accepts_any && !specific_qnames.is_empty() {
      let borrowed_attempt = build_choice_parse_attempt_tokens(
        field_ident,
        &choice_ty,
        field.repeated,
        DeserializeMode::Borrowed,
        false,
        xml_child_slot_assign.clone(),
      );
      let io_attempt = build_choice_parse_attempt_tokens(
        field_ident,
        &choice_ty,
        field.repeated,
        DeserializeMode::Io,
        false,
        xml_child_slot_assign.clone(),
      );
      let borrowed_visit_attempt = build_choice_parse_attempt_tokens(
        field_ident,
        &choice_ty,
        field.repeated,
        DeserializeMode::Borrowed,
        true,
        xml_child_slot_assign.clone(),
      );
      let io_visit_attempt = build_choice_parse_attempt_tokens(
        field_ident,
        &choice_ty,
        field.repeated,
        DeserializeMode::Io,
        true,
        xml_child_slot_assign.clone(),
      );
      for target in qname_match_target_keys(&specific_qnames) {
        grouped_choice_match_tokens_borrowed
          .entry(target.clone())
          .or_default()
          .push(GroupedChoiceAttempt {
            condition: choice_order_condition.clone(),
            tokens: borrowed_attempt.clone(),
          });
        grouped_choice_match_tokens_io
          .entry(target.clone())
          .or_default()
          .push(GroupedChoiceAttempt {
            condition: choice_order_condition.clone(),
            tokens: io_attempt.clone(),
          });
        grouped_choice_visit_match_tokens_borrowed
          .entry(target.clone())
          .or_default()
          .push(GroupedChoiceAttempt {
            condition: choice_order_condition.clone(),
            tokens: borrowed_visit_attempt.clone(),
          });
        grouped_choice_visit_match_tokens_io
          .entry(target)
          .or_default()
          .push(GroupedChoiceAttempt {
            condition: choice_order_condition.clone(),
            tokens: io_visit_attempt.clone(),
          });
      }
    } else if field.repeated {
      choice_unique_parse_tokens_borrowed.push(quote! {
          compile_error!("repeated choice read path must be flattened by SdkType");
      });
      choice_unique_parse_tokens_io.push(quote! {
          compile_error!("repeated choice read path must be flattened by SdkType");
      });
      choice_unique_visit_parse_tokens_borrowed.push(quote! {
          compile_error!("repeated choice read path must be flattened by SdkType");
      });
      choice_unique_visit_parse_tokens_io.push(quote! {
          compile_error!("repeated choice read path must be flattened by SdkType");
      });
    } else {
      choice_unique_parse_tokens_borrowed.push(quote! {
          compile_error!("choice read path must be flattened by SdkType");
      });
      choice_unique_parse_tokens_io.push(quote! {
          compile_error!("choice read path must be flattened by SdkType");
      });
      choice_unique_visit_parse_tokens_borrowed.push(quote! {
          compile_error!("choice read path must be flattened by SdkType");
      });
      choice_unique_visit_parse_tokens_io.push(quote! {
          compile_error!("choice read path must be flattened by SdkType");
      });
    }
    if field.repeated {
      choice_decl_tokens.push(quote! { let mut #field_ident = Vec::new(); });
      choice_init_tokens.push(quote! { #field_ident });
      choice_write_tokens.push(build_choice_write_tokens(
        &choice_ty,
        &field.items,
        field_ident,
        true,
        false,
      )?);
      choice_validate_tokens.push(quote! {
        for choice in &self.#field_ident {
          crate::validator::SdkValidator::validate_into(#validate_choice_tokens, context);
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
        choice_write_tokens.push(build_choice_write_tokens(
          &choice_ty,
          &field.items,
          field_ident,
          false,
          true,
        )?);
        choice_validate_tokens.push(quote! {
          if let Some(choice) = &self.#field_ident {
            crate::validator::SdkValidator::validate_into(#validate_choice_tokens, context);
          }
        });
      } else {
        choice_write_tokens.push(build_choice_write_tokens(
          &choice_ty,
          &field.items,
          field_ident,
          false,
          false,
        )?);
        choice_validate_tokens.push(quote! {
          crate::validator::SdkValidator::validate_into(#validate_self_tokens, context);
        });
      }
      choice_parse_tokens.push(quote! {});
      choice_visit_parse_tokens.push(quote! {});
      choice_text_parse_tokens.push(build_text_block(quote! { &text_value }));
    }
  }
  flat_choice_match_tokens_borrowed.extend(build_grouped_choice_match_tokens(
    &grouped_choice_match_tokens_borrowed,
    false,
  ));
  flat_choice_match_tokens_io.extend(build_grouped_choice_match_tokens(
    &grouped_choice_match_tokens_io,
    false,
  ));
  flat_choice_visit_match_tokens_borrowed.extend(build_grouped_choice_match_tokens(
    &grouped_choice_visit_match_tokens_borrowed,
    true,
  ));
  flat_choice_visit_match_tokens_io.extend(build_grouped_choice_match_tokens(
    &grouped_choice_visit_match_tokens_io,
    true,
  ));

  let mut any_decl_tokens = Vec::new();
  let mut any_init_tokens = Vec::new();
  let mut any_parse_tokens_borrowed = Vec::new();
  let mut any_parse_tokens_io = Vec::new();
  let mut any_visit_parse_tokens_borrowed = Vec::new();
  let mut any_visit_parse_tokens_io = Vec::new();
  for field in &any_fields {
    let field_ident = &field.ident;
    let xml_child_slot = xml_child_slot_by_field
      .get(&field_ident.to_string())
      .copied()
      .unwrap_or_default();
    let xml_child_slot_assign = xml_child_slot_assign_tokens(xml_child_slot);

    if field.repeated {
      any_decl_tokens.push(quote! { let mut #field_ident = Vec::new(); });
      any_init_tokens.push(quote! { #field_ident });
      any_parse_tokens_borrowed.push(build_any_child_parse_tokens(
        field_ident,
        &field.ty,
        true,
        DeserializeMode::Borrowed,
        false,
        xml_child_slot_assign.clone(),
      ));
      any_parse_tokens_io.push(build_any_child_parse_tokens(
        field_ident,
        &field.ty,
        true,
        DeserializeMode::Io,
        false,
        xml_child_slot_assign.clone(),
      ));
      any_visit_parse_tokens_borrowed.push(build_any_child_parse_tokens(
        field_ident,
        &field.ty,
        true,
        DeserializeMode::Borrowed,
        true,
        xml_child_slot_assign.clone(),
      ));
      any_visit_parse_tokens_io.push(build_any_child_parse_tokens(
        field_ident,
        &field.ty,
        true,
        DeserializeMode::Io,
        true,
        xml_child_slot_assign.clone(),
      ));
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
      any_parse_tokens_borrowed.push(build_any_child_parse_tokens(
        field_ident,
        &field.ty,
        false,
        DeserializeMode::Borrowed,
        false,
        xml_child_slot_assign.clone(),
      ));
      any_parse_tokens_io.push(build_any_child_parse_tokens(
        field_ident,
        &field.ty,
        false,
        DeserializeMode::Io,
        false,
        xml_child_slot_assign.clone(),
      ));
      any_visit_parse_tokens_borrowed.push(build_any_child_parse_tokens(
        field_ident,
        &field.ty,
        false,
        DeserializeMode::Borrowed,
        true,
        xml_child_slot_assign.clone(),
      ));
      any_visit_parse_tokens_io.push(build_any_child_parse_tokens(
        field_ident,
        &field.ty,
        false,
        DeserializeMode::Io,
        true,
        xml_child_slot_assign.clone(),
      ));
    }
  }
  let pure_any_parse_tokens_borrowed = if let Some(field) = any_fields.first() {
    let field_ident = &field.ident;
    build_pure_any_child_parse_tokens(
      field_ident,
      &field.ty,
      field.repeated,
      DeserializeMode::Borrowed,
    )
  } else {
    quote! {}
  };
  let pure_any_parse_tokens_io = if let Some(field) = any_fields.first() {
    let field_ident = &field.ident;
    build_pure_any_child_parse_tokens(field_ident, &field.ty, field.repeated, DeserializeMode::Io)
  } else {
    quote! {}
  };

  let choice_match_count_decl_tokens = quote! {};
  let choice_match_conflict_tokens = quote! {};

  let has_child_dispatch = !child_fields.is_empty()
    || !empty_child_fields.is_empty()
    || !text_child_fields.is_empty()
    || !any_child_fields.is_empty();
  let has_text_child_dispatch = !text_child_fields.is_empty() || !any_child_fields.is_empty();
  let has_choice_dispatch = !choice_fields.is_empty();
  let has_any_choice_dispatch = choice_fields.iter().any(|field| {
    field.accepts_any.unwrap_or(false)
      || field
        .items
        .iter()
        .any(|item| matches!(item, SdkTypeChoiceItem::Any { .. }))
  });
  let has_non_any_choice_dispatch = choice_fields
    .iter()
    .any(|field| !field.accepts_any.unwrap_or(false));
  let has_any_dispatch = !any_fields.is_empty();
  let pure_any_dispatch =
    !has_child_dispatch && !has_choice_dispatch && has_any_dispatch && !has_text_child_dispatch;
  let pure_any_choice_dispatch = !has_child_dispatch
    && has_choice_dispatch
    && !has_any_dispatch
    && !has_text_child_dispatch
    && has_any_choice_dispatch
    && !has_non_any_choice_dispatch;
  let unmatched_child_tokens_borrowed = if pure_any_dispatch || has_any_choice_dispatch {
    quote! {}
  } else {
    build_unmatched_child_tokens(
      ident,
      DeserializeMode::Borrowed,
      has_xml_other_children_field,
      compact_xml_other_children,
    )
  };
  let unmatched_child_tokens_io = if pure_any_dispatch || has_any_choice_dispatch {
    quote! {}
  } else {
    build_unmatched_child_tokens(
      ident,
      DeserializeMode::Io,
      has_xml_other_children_field,
      compact_xml_other_children,
    )
  };
  let flat_choice_dispatch_tokens_borrowed = if flat_choice_match_tokens_borrowed.is_empty() {
    quote! {}
  } else {
    let event_name_tokens = if pure_any_choice_dispatch {
      quote! { let event_name = e.name().into_inner(); }
    } else {
      quote! {}
    };
    let fallback_tokens = if flat_choice_has_wildcard {
      quote! {}
    } else {
      quote! { _ => {} }
    };
    quote! {
      #event_name_tokens
      match event_name {
        #( #flat_choice_match_tokens_borrowed )*
        #fallback_tokens
      }
    }
  };
  let flat_choice_dispatch_tokens_io = if flat_choice_match_tokens_io.is_empty() {
    quote! {}
  } else {
    let event_name_tokens = if pure_any_choice_dispatch {
      quote! { let event_name = e.name().into_inner(); }
    } else {
      quote! {}
    };
    let fallback_tokens = if flat_choice_has_wildcard {
      quote! {}
    } else {
      quote! { _ => {} }
    };
    quote! {
      #event_name_tokens
      match event_name {
        #( #flat_choice_match_tokens_io )*
        #fallback_tokens
      }
    }
  };
  let choice_fallback_tokens_borrowed = if flat_choice_has_wildcard {
    quote! {}
  } else {
    quote! {
      _ => {
        #( #choice_match_init_tokens )*
        {
          #choice_match_count_decl_tokens
          #( #choice_match_decl_tokens )*
          #( #choice_unique_parse_tokens_borrowed )*
          #choice_match_conflict_tokens
        }
      }
    }
  };
  let choice_fallback_tokens_io = if flat_choice_has_wildcard {
    quote! {}
  } else {
    quote! {
      _ => {
        #( #choice_match_init_tokens )*
        {
          #choice_match_count_decl_tokens
          #( #choice_match_decl_tokens )*
          #( #choice_unique_parse_tokens_io )*
          #choice_match_conflict_tokens
        }
      }
    }
  };
  let child_choice_dispatch_tokens_borrowed =
    if !has_child_dispatch && !has_choice_dispatch && !has_any_dispatch {
      quote! {}
    } else if pure_any_dispatch {
      pure_any_parse_tokens_borrowed
    } else if !has_child_dispatch && !has_any_dispatch {
      if flat_choice_has_wildcard {
        flat_choice_dispatch_tokens_borrowed.clone()
      } else {
        quote! {
          #flat_choice_dispatch_tokens_borrowed
          #( #choice_match_init_tokens )*
          {
            #choice_match_count_decl_tokens
            #( #choice_match_decl_tokens )*
            #( #choice_unique_parse_tokens_borrowed )*
            #choice_match_conflict_tokens
          }
        }
      }
    } else if !has_child_dispatch {
      quote! {
        #flat_choice_dispatch_tokens_borrowed
        #( #choice_match_init_tokens )*
        {
          #choice_match_count_decl_tokens
          #( #choice_match_decl_tokens )*
          #( #choice_unique_parse_tokens_borrowed )*
          #choice_match_conflict_tokens
        }
        let mut matched = false;
        #( #choice_parse_tokens )*
        #( #any_parse_tokens_borrowed )*
        if matched {
          continue;
        }
      }
    } else if !has_choice_dispatch && !has_any_dispatch && !has_text_child_dispatch {
      quote! {
        match event_name {
          #( #direct_child_match_tokens_borrowed )*
          _ => {}
        }
      }
    } else if !has_choice_dispatch && !has_any_dispatch {
      quote! {
        let matched = match event_name {
          #( #direct_child_match_tokens_borrowed )*
          #( #child_parse_tokens_borrowed )*
          _ => false,
        };
        if matched {
          continue;
        }
      }
    } else if !has_any_dispatch && !has_text_child_dispatch {
      quote! {
        match event_name {
          #( #direct_child_match_tokens_borrowed )*
          #( #flat_choice_match_tokens_borrowed )*
          #choice_fallback_tokens_borrowed
        }
      }
    } else if !has_any_dispatch {
      quote! {
        let matched = match event_name {
          #( #direct_child_match_tokens_borrowed )*
          #( #flat_choice_match_tokens_borrowed )*
          #( #child_parse_tokens_borrowed )*
          _ => {
            #( #choice_match_init_tokens )*
            {
              #choice_match_count_decl_tokens
              #( #choice_match_decl_tokens )*
              #( #choice_unique_parse_tokens_borrowed )*
              #choice_match_conflict_tokens
            }
            false
          },
        };
        if matched {
          continue;
        }
      }
    } else {
      quote! {
        let matched = match event_name {
          #( #direct_child_match_tokens_borrowed )*
          #( #flat_choice_match_tokens_borrowed )*
          #( #child_parse_tokens_borrowed )*
          _ => {
            #( #choice_match_init_tokens )*
            {
              #choice_match_count_decl_tokens
              #( #choice_match_decl_tokens )*
              #( #choice_unique_parse_tokens_borrowed )*
              #choice_match_conflict_tokens
            }
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
  let child_choice_dispatch_tokens_io =
    if !has_child_dispatch && !has_choice_dispatch && !has_any_dispatch {
      quote! {}
    } else if pure_any_dispatch {
      pure_any_parse_tokens_io
    } else if !has_child_dispatch && !has_any_dispatch {
      if flat_choice_has_wildcard {
        flat_choice_dispatch_tokens_io.clone()
      } else {
        quote! {
          #flat_choice_dispatch_tokens_io
          #( #choice_match_init_tokens )*
          {
            #choice_match_count_decl_tokens
            #( #choice_match_decl_tokens )*
            #( #choice_unique_parse_tokens_io )*
            #choice_match_conflict_tokens
          }
        }
      }
    } else if !has_child_dispatch {
      quote! {
        #flat_choice_dispatch_tokens_io
        #( #choice_match_init_tokens )*
        {
          #choice_match_count_decl_tokens
          #( #choice_match_decl_tokens )*
          #( #choice_unique_parse_tokens_io )*
          #choice_match_conflict_tokens
        }
        let mut matched = false;
        #( #choice_parse_tokens )*
        #( #any_parse_tokens_io )*
        if matched {
          continue;
        }
      }
    } else if !has_choice_dispatch && !has_any_dispatch && !has_text_child_dispatch {
      quote! {
        match event_name {
          #( #direct_child_match_tokens_io )*
          _ => {}
        }
      }
    } else if !has_choice_dispatch && !has_any_dispatch {
      quote! {
        let matched = match event_name {
          #( #direct_child_match_tokens_io )*
          #( #child_parse_tokens_io )*
          _ => false,
        };
        if matched {
          continue;
        }
      }
    } else if !has_any_dispatch && !has_text_child_dispatch {
      quote! {
        match event_name {
          #( #direct_child_match_tokens_io )*
          #( #flat_choice_match_tokens_io )*
          #choice_fallback_tokens_io
        }
      }
    } else if !has_any_dispatch {
      quote! {
        let matched = match event_name {
          #( #direct_child_match_tokens_io )*
          #( #flat_choice_match_tokens_io )*
          #( #child_parse_tokens_io )*
          _ => {
            #( #choice_match_init_tokens )*
            {
              #choice_match_count_decl_tokens
              #( #choice_match_decl_tokens )*
              #( #choice_unique_parse_tokens_io )*
              #choice_match_conflict_tokens
            }
            false
          },
        };
        if matched {
          continue;
        }
      }
    } else {
      quote! {
        let matched = match event_name {
          #( #direct_child_match_tokens_io )*
          #( #flat_choice_match_tokens_io )*
          #( #child_parse_tokens_io )*
          _ => {
            #( #choice_match_init_tokens )*
            {
              #choice_match_count_decl_tokens
              #( #choice_match_decl_tokens )*
              #( #choice_unique_parse_tokens_io )*
              #choice_match_conflict_tokens
            }
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
  let xml_other_children_write_setup_tokens = quote! {};
  let xml_other_children_write_trailing_tokens = if has_xml_other_children_field {
    if compact_xml_other_children {
      quote! {
        for xml in &self.xml_other_children {
          writer.write_all(xml.as_ref())?;
        }
      }
    } else {
      quote! {
        for (slot, xml) in &self.xml_other_children {
          if *slot == #xml_child_slot_count {
            writer.write_all(xml.as_ref())?;
          }
        }
      }
    }
  } else {
    quote! {}
  };
  for (field_ident, field_ty, field_kind, choice_items) in &ordered_field_specs {
    if has_xml_other_children_field
      && !compact_xml_other_children
      && matches!(
        field_kind,
        SdkTypeFieldKind::Child { .. }
          | SdkTypeFieldKind::EmptyChild { .. }
          | SdkTypeFieldKind::TextChild { .. }
          | SdkTypeFieldKind::AnyChild { .. }
          | SdkTypeFieldKind::Choice
          | SdkTypeFieldKind::Any
      )
    {
      let xml_other_slot = xml_child_slot_by_field
        .get(&field_ident.to_string())
        .copied()
        .unwrap_or_default()
        .saturating_sub(1usize);
      ordered_write_tokens.push(quote! {
        for (slot, xml) in &self.xml_other_children {
          if *slot == #xml_other_slot {
            writer.write_all(xml.as_ref())?;
          }
        }
      });
    }
    match field_kind {
      SdkTypeFieldKind::Child { qname, .. } => {
        let repeated = contains_vec_type(field_ty);
        let optional = is_option_type(field_ty);
        let payload_ty = unwrap_option_vec_type(field_ty);
        let child_ty = box_inner_type(&payload_ty).unwrap_or_else(|| payload_ty.clone());
        let child_write_call = write_typed_child_tokens(&child_ty, quote! { child }, qname);
        let self_write_call =
          write_typed_child_tokens(&child_ty, quote! { &self.#field_ident }, qname);
        if repeated {
          ordered_write_tokens.push(quote! {
            for child in &self.#field_ident {
              #child_write_call
            }
          });
        } else if optional {
          ordered_write_tokens.push(quote! {
            if let Some(child) = &self.#field_ident {
              #child_write_call
            }
          });
        } else {
          ordered_write_tokens.push(quote! {
            #self_write_call
          });
        }
      }
      SdkTypeFieldKind::TextChild {
        qname,
        simple_type,
        list,
      } => {
        let repeated = !list && contains_vec_type(field_ty);
        ordered_write_tokens.push(build_text_child_write_tokens(
          field_ident,
          qname,
          simple_type.as_deref(),
          field_ty,
          repeated,
          is_option_type(field_ty),
          *list,
        ));
      }
      SdkTypeFieldKind::AnyChild { qname } => {
        ordered_write_tokens.push(build_any_child_write_tokens(
          field_ident,
          qname,
          contains_vec_type(field_ty),
          is_option_type(field_ty),
        ));
      }
      SdkTypeFieldKind::EmptyChild { qname } => {
        ordered_write_tokens.push(build_empty_child_write_tokens(
          field_ident,
          qname,
          contains_vec_type(field_ty),
          is_option_type(field_ty),
        ));
      }
      SdkTypeFieldKind::Choice => {
        let choice_ty = unwrap_wrapped_type(field_ty);
        let repeated = contains_vec_type(field_ty);
        let optional = is_option_type(field_ty);
        ordered_write_tokens.push(build_choice_write_tokens(
          &choice_ty,
          choice_items,
          field_ident,
          repeated,
          optional,
        )?);
      }
      SdkTypeFieldKind::Any => {
        let repeated = contains_vec_type(field_ty);
        let optional = is_option_type(field_ty);
        let value_bytes_expr = if is_box_u8_slice_type(&unwrap_option_vec_type(field_ty)) {
          quote! { value.as_ref() }
        } else {
          quote! { value.as_bytes() }
        };
        let field_bytes_expr = if is_box_u8_slice_type(&unwrap_option_vec_type(field_ty)) {
          quote! { self.#field_ident.as_ref() }
        } else {
          quote! { self.#field_ident.as_bytes() }
        };
        if repeated {
          ordered_write_tokens.push(quote! {
            for value in &self.#field_ident {
              writer.write_all(#value_bytes_expr)?;
            }
          });
        } else if optional {
          ordered_write_tokens.push(quote! {
            if let Some(value) = &self.#field_ident {
              writer.write_all(#value_bytes_expr)?;
            }
          });
        } else {
          ordered_write_tokens.push(quote! {
            writer.write_all(#field_bytes_expr)?;
          });
        }
      }
      SdkTypeFieldKind::Text { list } => {
        let inner_ty = if *list {
          vec_inner_type(&unwrap_option_type(field_ty))
            .unwrap_or_else(|| unwrap_wrapped_type(field_ty))
        } else {
          unwrap_wrapped_type(field_ty)
        };
        let optional_text_write_tokens = if *list {
          quote! {
            crate::common::write_list_text_content_value(writer, value.as_slice())?;
          }
        } else if let Some(kind) = simple_union_type_kind(&inner_ty) {
          write_simple_union_value_tokens(kind, quote! { value })
        } else if is_xml_schema_float_type(&inner_ty) {
          write_xml_schema_float_tokens(quote! { value }, &inner_ty)
        } else if is_string_like_type(&inner_ty) {
          quote! {
            crate::common::write_escaped_content_str(writer, value.as_ref())?;
          }
        } else {
          quote! {
            crate::common::write_escaped_content_text(writer, value)?;
          }
        };
        let required_text_write_tokens = if *list {
          quote! {
            crate::common::write_list_text_content_value(writer, self.#field_ident.as_slice())?;
          }
        } else if let Some(kind) = simple_union_type_kind(&inner_ty) {
          write_simple_union_value_tokens(kind, quote! { &self.#field_ident })
        } else if is_xml_schema_float_type(&inner_ty) {
          write_xml_schema_float_tokens(quote! { &self.#field_ident }, &inner_ty)
        } else if is_string_like_type(&inner_ty) {
          quote! {
            crate::common::write_escaped_content_str(writer, self.#field_ident.as_ref())?;
          }
        } else {
          quote! {
            crate::common::write_escaped_content_text(writer, &self.#field_ident)?;
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
  if has_xml_other_children_field {
    ordered_write_tokens.push(quote! {
      #xml_other_children_write_trailing_tokens
    });
  }
  let text_finish_tokens = if let Some(text_field) = &text_field {
    let field_ident = &text_field.ident;
    let inner_ty = if text_field.list {
      vec_inner_type(&unwrap_option_type(&text_field.ty))
        .unwrap_or_else(|| unwrap_wrapped_type(&text_field.ty))
    } else {
      unwrap_wrapped_type(&text_field.ty)
    };
    let field_name_lit = LitStr::new(&field_ident.to_string(), Span::call_site());
    if text_field.list {
      let parse_value_tokens = quote! {
        crate::common::parse_list_value::<#inner_ty>(&value, stringify!(#ident), #field_name_lit)?
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
    } else if let Some(kind) = simple_union_type_kind(&inner_ty) {
      let parse_value_tokens = parse_simple_union_value_tokens(kind, quote! { value });
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
    } else if is_string_like_type(&inner_ty) {
      quote! {
        #field_ident,
      }
    } else {
      let parse_value_tokens = quote! {
        crate::common::parse_value::<#inner_ty>(&value, stringify!(#ident), #field_name_lit)?
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
  let choice_accepts_text_tokens = if choice_fields.is_empty() {
    quote! { false }
  } else {
    let all_known = choice_fields
      .iter()
      .all(|field| field.accepts_text.is_some());
    if all_known {
      let any_accepts_text = choice_fields
        .iter()
        .any(|field| field.accepts_text.expect("checked above"));
      quote! { #any_accepts_text }
    } else {
      let choice_accepts_text_checks: Vec<_> = choice_fields
        .iter()
        .map(|field| {
          if let Some(accepts_text) = field.accepts_text {
            quote! { #accepts_text }
          } else {
            let accepts_text = field
              .items
              .iter()
              .any(|item| matches!(item, SdkTypeChoiceItem::Text { .. }));
            quote! { #accepts_text }
          }
        })
        .collect();
      quote! {
        false #( || #choice_accepts_text_checks )*
      }
    }
  };
  let choice_accepts_text_known = if choice_fields.is_empty() {
    Some(false)
  } else if choice_fields
    .iter()
    .all(|field| field.accepts_text.is_some())
  {
    Some(
      choice_fields
        .iter()
        .any(|field| field.accepts_text.expect("checked above")),
    )
  } else {
    None
  };
  let borrowed_decl_event_tokens = if has_xml_header_field {
    quote! {
      quick_xml::events::Event::Decl(e) => {
        xml_header_state = if matches!(
          e.standalone(),
          Some(Ok(value)) if value.as_ref().eq_ignore_ascii_case(b"yes")
        ) {
          crate::common::XmlHeaderType::Standalone
        } else {
          crate::common::XmlHeaderType::Plain
        };
      },
    }
  } else {
    quote! {
      quick_xml::events::Event::Decl(_) => {},
    }
  };
  let tag_decl_tokens_borrowed = if has_xml_header_field {
    quote! {
      crate::common::SliceTagEvent::Decl(standalone) => {
        xml_header_state = if standalone {
          crate::common::XmlHeaderType::Standalone
        } else {
          crate::common::XmlHeaderType::Plain
        };
      },
    }
  } else {
    quote! {
      crate::common::SliceTagEvent::Decl(_) => {},
    }
  };
  let tag_decl_tokens_io = if has_xml_header_field {
    quote! {
      crate::common::IoTagEvent::Decl(standalone) => {
        xml_header_state = if standalone {
          crate::common::XmlHeaderType::Standalone
        } else {
          crate::common::XmlHeaderType::Plain
        };
      },
    }
  } else {
    quote! {
      crate::common::IoTagEvent::Decl(_) => {},
    }
  };
  let raw_xml_other_children_only = has_xml_other_children_field
    && !has_child_dispatch
    && !has_choice_dispatch
    && !has_any_dispatch
    && !has_text_child_dispatch;
  let needs_child_event_name = !raw_xml_other_children_only
    && (has_child_dispatch
      || has_any_dispatch
      || has_text_child_dispatch
      || !pure_any_choice_dispatch);
  let child_event_name_tokens = if !needs_child_event_name {
    quote! {}
  } else if use_local_name_child_dispatch {
    quote! {
      let event_name = e.local_name().into_inner();
    }
  } else {
    quote! {
      let event_name = e.name().into_inner();
    }
  };
  let borrowed_children_text_loop_tokens = quote! {
    if !empty_tag {
      loop {
        match xml_reader.next()? {
          #borrowed_decl_event_tokens
          quick_xml::events::Event::Start(e) => {
            let next_empty = false;
            #child_event_name_tokens
            #child_choice_dispatch_tokens_borrowed
            #unmatched_child_tokens_borrowed
          }
          quick_xml::events::Event::Empty(e) => {
            let next_empty = true;
            #child_event_name_tokens
            #child_choice_dispatch_tokens_borrowed
            #unmatched_child_tokens_borrowed
          }
          #text_read_tokens
          quick_xml::events::Event::End(e) => {
            if #end_name_matches {
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
  };
  let borrowed_children_tag_loop_tokens = quote! {
    if !empty_tag {
      loop {
        match xml_reader.next_tag_event()? {
          #tag_decl_tokens_borrowed
          crate::common::SliceTagEvent::Start(e, next_empty) => {
            #child_event_name_tokens
            #child_choice_dispatch_tokens_borrowed
            #unmatched_child_tokens_borrowed
          }
          crate::common::SliceTagEvent::End(e) => {
            if #end_name_matches {
              break;
            }
          }
          crate::common::SliceTagEvent::Eof => {
            return Err(crate::common::unexpected_eof(stringify!(#ident)));
          }
          crate::common::SliceTagEvent::Other => {}
        }
      }
    }
  };
  let borrowed_children_loop_tokens = if text_field.is_some() {
    borrowed_children_text_loop_tokens.clone()
  } else if choice_accepts_text_known == Some(false) {
    borrowed_children_tag_loop_tokens.clone()
  } else if choice_accepts_text_known == Some(true) {
    borrowed_children_text_loop_tokens.clone()
  } else {
    quote! {
      if #choice_accepts_text_tokens {
        #borrowed_children_text_loop_tokens
      } else {
        #borrowed_children_tag_loop_tokens
      }
    }
  };
  let io_children_text_loop_tokens = quote! {
    if !empty_tag {
      loop {
        match xml_reader.next()? {
          #borrowed_decl_event_tokens
          quick_xml::events::Event::Start(e) => {
            let e = e.into_owned();
            let next_empty = false;
            #child_event_name_tokens
            #child_choice_dispatch_tokens_io
            #unmatched_child_tokens_io
          }
          quick_xml::events::Event::Empty(e) => {
            let e = e.into_owned();
            let next_empty = true;
            #child_event_name_tokens
            #child_choice_dispatch_tokens_io
            #unmatched_child_tokens_io
          }
          #text_read_tokens
          quick_xml::events::Event::End(e) => {
            if #end_name_matches {
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
  };
  let io_children_tag_loop_tokens = quote! {
    if !empty_tag {
      loop {
        match xml_reader.next_tag_event()? {
          #tag_decl_tokens_io
          crate::common::IoTagEvent::Start(e, next_empty) => {
            #child_event_name_tokens
            #child_choice_dispatch_tokens_io
            #unmatched_child_tokens_io
          }
          crate::common::IoTagEvent::End(e) => {
            if #end_name_matches {
              break;
            }
          }
          crate::common::IoTagEvent::Eof => {
            return Err(crate::common::unexpected_eof(stringify!(#ident)));
          }
          crate::common::IoTagEvent::Other => {}
        }
      }
    }
  };
  let io_children_loop_tokens = if text_field.is_some() {
    io_children_text_loop_tokens.clone()
  } else if choice_accepts_text_known == Some(false) {
    io_children_tag_loop_tokens.clone()
  } else if choice_accepts_text_known == Some(true) {
    io_children_text_loop_tokens.clone()
  } else {
    quote! {
      if #choice_accepts_text_tokens {
        #io_children_text_loop_tokens
      } else {
        #io_children_tag_loop_tokens
      }
    }
  };

  let fixed_namespace_write_tokens = if has_xmlns_fields {
    if let Some(attr) = &fixed_namespace_write_lit {
      quote! {
        writer.write_all(#attr)?;
      }
    } else {
      quote! {}
    }
  } else {
    quote! {}
  };
  let special_namespace_write_tokens = if has_xmlns_fields {
    let fixed_namespace_skip_tokens = if let Some(uri_lit) = &fixed_namespace_uri_lit {
      if default_ns {
        quote! {
          if declaration.uri_bytes() == #uri_lit.as_slice()
            && (declaration.is_default()
              || declaration.prefix_bytes() == #raw_tag_prefix_lit.as_slice())
          {
            continue;
          }
        }
      } else {
        quote! {
          if declaration.uri_bytes() == #uri_lit.as_slice()
            && declaration.prefix_bytes() == #raw_tag_prefix_lit.as_slice()
          {
            continue;
          }
        }
      }
    } else {
      quote! {}
    };
    let prefix_tokens = if use_canonical_xmlns_prefix {
      quote! {
        let prefix = declaration
          .uri
          .canonical_prefix_bytes(declaration.prefix_bytes());
        let prefix = if declaration.is_default() {
          None
        } else {
          Some(prefix)
        };
      }
    } else {
      quote! {
        let prefix = if declaration.is_default() {
          None
        } else {
          Some(declaration.prefix_bytes())
        };
      }
    };
    quote! {
      #fixed_namespace_write_tokens
      for declaration in &self.xmlns {
        #fixed_namespace_skip_tokens
        #prefix_tokens
        crate::common::write_xmlns_attr(writer, prefix, declaration.uri_bytes())?;
      }
    }
  } else {
    quote! {}
  };
  let xml_other_attrs_write_tokens = if has_xml_other_attrs_field {
    quote! {
      for attr in &self.xml_other_attrs {
        attr.write(writer)?;
      }
    }
  } else {
    quote! {}
  };
  let special_namespace_init_tokens = if has_xmlns_fields {
    quote! {
      xmlns,
    }
  } else {
    quote! {}
  };
  let xml_other_attrs_init_tokens = if has_xml_other_attrs_field {
    quote! {
      xml_other_attrs,
    }
  } else {
    quote! {}
  };
  let xml_other_children_decl_tokens = if has_xml_other_children_field {
    if compact_xml_other_children {
      quote! {
        let mut xml_other_children = Vec::<std::boxed::Box<[u8]>>::new();
      }
    } else {
      quote! {
        let mut xml_other_children = Vec::<(usize, std::boxed::Box<[u8]>)>::new();
        let mut __xml_child_slot = 0usize;
      }
    }
  } else {
    quote! {}
  };
  let xml_other_children_init_tokens = if has_xml_other_children_field {
    quote! {
      xml_other_children,
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
  let xml_header_assign_tokens = if has_xml_header_field {
    quote! {
      value.xml_header = xml_header_state;
    }
  } else {
    quote! {}
  };
  let root_read_borrowed_tokens = if has_xml_header_field {
    quote! {
      let (start, empty, xml_header_state) = crate::common::read_root_start_borrowed(
        &mut xml_reader,
        stringify!(#ident),
        #tag_qname_lit,
        #local_name_lit,
      )?;
      let mut value = <Self as crate::sdk::SdkType>::read_borrowed_inner(&mut xml_reader, start, empty)?;
      #xml_header_assign_tokens
      Ok(value)
    }
  } else {
    quote! {
      let (start, empty) = crate::common::read_root_start_borrowed_no_header(
        &mut xml_reader,
        stringify!(#ident),
        #tag_qname_lit,
        #local_name_lit,
      )?;
      <Self as crate::sdk::SdkType>::read_borrowed_inner(&mut xml_reader, start, empty)
    }
  };
  let root_read_io_tokens = if has_xml_header_field {
    quote! {
      let (start, empty, xml_header_state) = crate::common::read_root_start_io(
        &mut xml_reader,
        stringify!(#ident),
        #tag_qname_lit,
        #local_name_lit,
      )?;
      let mut value = <Self as crate::sdk::SdkType>::read_io_inner(&mut xml_reader, start, empty)?;
      #xml_header_assign_tokens
      Ok(value)
    }
  } else {
    quote! {
      let (start, empty) = crate::common::read_root_start_io_no_header(
        &mut xml_reader,
        stringify!(#ident),
        #tag_qname_lit,
        #local_name_lit,
      )?;
      <Self as crate::sdk::SdkType>::read_io_inner(&mut xml_reader, start, empty)
    }
  };
  let has_body = !child_fields.is_empty()
    || !empty_child_fields.is_empty()
    || !text_child_fields.is_empty()
    || !choice_fields.is_empty()
    || !any_fields.is_empty()
    || has_xml_other_children_field
    || text_field.is_some();
  let close_body_tokens = if local_name.is_empty() {
    quote! {}
  } else {
    quote! {
      #end_tag
    }
  };
  let body_write_tokens = if has_body {
    quote! {
      writer.write_all(b">")?;
      #xml_other_children_write_setup_tokens
      #( #ordered_write_tokens )*
      #close_body_tokens
      Ok(())
    }
  } else if local_name.is_empty() {
    quote! {
      Ok(())
    }
  } else {
    quote! {
      writer.write_all(b" />")?;
      Ok(())
    }
  };
  let mce_child_process_tokens = child_fields
    .iter()
    .map(mce_process_child_field_tokens)
    .collect::<Vec<_>>();
  let mce_choice_process_tokens = choice_fields
    .iter()
    .map(mce_process_choice_field_tokens)
    .collect::<Vec<_>>();
  let mut mce_choice_impl_keys = std::collections::HashSet::new();
  let mce_choice_impl_tokens = choice_fields
    .iter()
    .filter_map(|field| {
      let choice_ty = unwrap_option_vec_type(&field.ty);
      mce_choice_impl_keys
        .insert(quote! { #choice_ty }.to_string())
        .then(|| mce_choice_impl_tokens(field))
    })
    .collect::<Vec<_>>();
  let (mce_context_push_tokens, mce_xml_other_attrs_process_tokens, mce_context_pop_tokens) =
    mce_context_scope_tokens(&xmlns_fields, xml_other_attrs_field.as_ref());
  let mce_xml_other_children_process_tokens =
    mce_xml_other_children_process_tokens(has_xml_other_children_field, compact_xml_other_children);
  let public_root_from_str_tokens = if local_name.is_empty() {
    quote! {}
  } else {
    quote! {
      impl #impl_generics std::str::FromStr for #ident #type_generics #where_clause {
        type Err = crate::common::SdkError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
          let mut xml_reader = crate::common::from_str_inner(s)?;
          #root_read_borrowed_tokens
        }
      }
    }
  };
  let public_root_methods_tokens = if local_name.is_empty() {
    quote! {}
  } else {
    quote! {
      pub fn from_bytes(bytes: &[u8]) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_bytes_inner(bytes)?;
        #root_read_borrowed_tokens
      }

      pub fn from_reader<R: std::io::BufRead>(
        reader: R,
      ) -> Result<Self, crate::common::SdkError> {
        let mut xml_reader = crate::common::from_reader_inner(reader)?;
        #root_read_io_tokens
      }

      pub fn write_to<W: std::io::Write>(
        &self,
        writer: &mut W,
      ) -> Result<(), std::io::Error> {
        #xml_header_tokens
        #start_tag_open
        <Self as crate::sdk::SdkType>::write_inner(self, writer)?;
        Ok(())
      }

      pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut writer = Vec::with_capacity(32);
        self.write_to(&mut writer)?;
        Ok(writer)
      }

      pub fn to_xml(&self) -> Result<String, crate::common::SdkError> {
        String::from_utf8(self.to_bytes()?).map_err(|err| {
          crate::common::SdkError::CommonError(format!("invalid utf-8 xml: {err}"))
        })
      }
    }
  };
  let public_root_display_tokens = if local_name.is_empty() {
    quote! {}
  } else {
    quote! {
      impl #impl_generics ::std::fmt::Display for #ident #type_generics #where_clause {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
          f.write_str(&self.to_xml().map_err(|_| ::std::fmt::Error)?)
        }
      }
    }
  };
  Ok(quote! {
    #( #mce_choice_impl_tokens )*

    impl #impl_generics crate::sdk::SdkType for #ident #type_generics #where_clause {
      fn read_borrowed_inner<'de>(
        xml_reader: &mut crate::common::SliceReader<'de>,
        e: quick_xml::events::BytesStart<'de>,
        empty_tag: bool,
      ) -> Result<Self, crate::common::SdkError> {
        #xml_header_decl_tokens
        #end_qname_decl
        #( #attr_decl_tokens )*
        #namespace_attr_parse_tokens
        #( #child_decl_tokens )*
        #( #choice_decl_tokens )*
        #( #any_decl_tokens )*
        #xml_other_children_decl_tokens
        #text_decl_tokens

        #borrowed_children_loop_tokens

        Ok(Self {
          #( #attr_init_tokens, )*
          #( #child_init_tokens, )*
          #( #choice_init_tokens, )*
          #( #any_init_tokens, )*
          #text_finish_tokens
          #( #attr_finish_tokens, )*
          #special_namespace_init_tokens
          #xml_header_init_tokens
          #xml_other_attrs_init_tokens
          #xml_other_children_init_tokens
        })
      }

      fn read_io_inner<R: std::io::BufRead>(
        xml_reader: &mut crate::common::IoReader<R>,
        e: quick_xml::events::BytesStart<'static>,
        empty_tag: bool,
      ) -> Result<Self, crate::common::SdkError> {
        #xml_header_decl_tokens
        #end_qname_decl
        #( #attr_decl_tokens )*
        #namespace_attr_parse_tokens
        #( #child_decl_tokens )*
        #( #choice_decl_tokens )*
        #( #any_decl_tokens )*
        #xml_other_children_decl_tokens
        #text_decl_tokens

        #io_children_loop_tokens

        Ok(Self {
          #( #attr_init_tokens, )*
          #( #child_init_tokens, )*
          #( #choice_init_tokens, )*
          #( #any_init_tokens, )*
          #text_finish_tokens
          #( #attr_finish_tokens, )*
          #special_namespace_init_tokens
          #xml_header_init_tokens
          #xml_other_attrs_init_tokens
          #xml_other_children_init_tokens
        })
      }

      fn write_inner<W: std::io::Write>(
        &self,
        writer: &mut W,
      ) -> Result<(), std::io::Error> {
        #special_namespace_write_tokens
        #( #attr_write_tokens )*
        #xml_other_attrs_write_tokens
        #body_write_tokens
      }
    }
    #[cfg(feature = "validators")]
    impl #impl_generics crate::validator::SdkValidator for #ident #type_generics #where_clause {
      fn validate_into(&self, context: &mut crate::validator::ValidationContext) {
        #( #attr_validate_tokens )*
        #( #child_validate_tokens )*
        #( #choice_validate_tokens )*
      }
    }

    #[cfg(feature = "mce")]
    impl #impl_generics crate::sdk::SdkMce for #ident #type_generics #where_clause {
      fn process_mce_with_context(
        &mut self,
        settings: &crate::sdk::MarkupCompatibilityProcessSettings,
        context: &mut crate::sdk::MceContext,
      ) -> Result<(), crate::common::SdkError> {
        if matches!(
          settings.process_mode,
          crate::sdk::MarkupCompatibilityProcessMode::NoProcess
        ) {
          return Ok(());
        }
        #mce_context_push_tokens
        #mce_xml_other_attrs_process_tokens
        #mce_xml_other_children_process_tokens
        #( #mce_child_process_tokens )*
        #( #mce_choice_process_tokens )*
        #mce_context_pop_tokens
        Ok(())
      }
    }

    #public_root_from_str_tokens

    impl #impl_generics #ident #type_generics #where_clause {
      #public_root_methods_tokens
    }

    #public_root_display_tokens
  })
}
