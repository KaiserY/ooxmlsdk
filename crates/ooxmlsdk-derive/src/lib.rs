use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
  Attribute, Data, DataEnum, DeriveInput, Fields, Ident, LitByteStr, LitStr, Meta, Token, Type,
  TypePath, parse_macro_input, parse_str,
};

mod sdk_choice;
mod sdk_enum;
mod sdk_part;
mod sdk_type;

#[proc_macro_derive(SdkEnum, attributes(sdk))]
pub fn sdk_enum(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  match sdk_enum::expand_sdk_enum(&input) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.to_compile_error().into(),
  }
}

#[proc_macro_derive(SdkType, attributes(sdk))]
pub fn sdk_type(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  match sdk_type::expand_sdk_type(&input) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.to_compile_error().into(),
  }
}

#[proc_macro_derive(SdkChoice, attributes(sdk))]
pub fn sdk_choice(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  match sdk_choice::expand_sdk_choice(&input) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.to_compile_error().into(),
  }
}

#[proc_macro_derive(SdkPart, attributes(sdk))]
pub fn sdk_part(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  match sdk_part::expand_sdk_part(&input) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.to_compile_error().into(),
  }
}

#[derive(Clone)]
enum PartChildKind {
  Repeated,
  Required,
  Optional,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DerivedPartContentKind {
  Text,
  Binary,
}

#[derive(Clone)]
struct PartChildInfo {
  field_ident: Ident,
  ty: Type,
  kind: PartChildKind,
}

impl PartChildInfo {
  fn module_ident(&self) -> syn::Result<Ident> {
    let type_ident = type_last_ident(&self.ty).ok_or_else(|| {
      syn::Error::new_spanned(&self.ty, "SdkPart child field must use a path type")
    })?;
    parse_str(&to_snake_case_local(&type_ident.to_string()))
      .map_err(|err| syn::Error::new_spanned(&self.ty, err))
  }

  fn item_ident(&self) -> syn::Result<Ident> {
    let type_ident = type_last_ident(&self.ty).ok_or_else(|| {
      syn::Error::new_spanned(&self.ty, "SdkPart child field must use a path type")
    })?;
    parse_str(&to_snake_case_local(&type_ident.to_string()))
      .map_err(|err| syn::Error::new_spanned(&self.ty, err))
  }
}

#[derive(Clone)]
struct SdkAttrField {
  ident: Ident,
  name: String,
  ty: Type,
  optional: bool,
  validators: Vec<SdkFieldValidator>,
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
struct SdkTextChildField {
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
struct SdkAnyField {
  ident: Ident,
  optional: bool,
  repeated: bool,
}

#[derive(Clone)]
struct SdkTextField {
  ident: Ident,
  ty: Type,
  optional: bool,
}

#[derive(Clone)]
enum SdkTypeFieldKind {
  Attr { name: String },
  Child { qname: String },
  TextChild { qname: String },
  Choice,
  Any,
  Text,
}

#[derive(Clone)]
enum SdkFieldValidator {
  Pattern {
    regex: String,
    source_id: u32,
    union_id: Option<u64>,
  },
  StringLength {
    min: Option<u32>,
    max: Option<u32>,
    type_name: Option<String>,
    source_id: u32,
    union_id: Option<u64>,
  },
  StringFormat {
    kind: SdkStringFormatKind,
    source_id: u32,
    union_id: Option<u64>,
  },
  StringSet {
    values: Vec<String>,
    source_id: u32,
    union_id: Option<u64>,
  },
  NumberRange {
    min: Option<String>,
    max: Option<String>,
    min_inclusive: bool,
    max_inclusive: bool,
    source_id: u32,
    union_id: Option<u64>,
  },
  NumberType {
    type_name: String,
    source_id: u32,
    union_id: Option<u64>,
  },
  NumberSign {
    kind: SdkNumberSignKind,
    source_id: u32,
    union_id: Option<u64>,
  },
}

#[derive(Clone)]
enum SdkStringFormatKind {
  Token,
  NcName,
  QName,
  Uri,
  Id,
}

#[derive(Clone)]
enum SdkNumberSignKind {
  NonNegative,
  Positive,
}

enum SdkChoiceVariantKind {
  Child { qnames: Vec<String> },
  TextChild { qnames: Vec<String> },
  Any,
  Text,
}

#[derive(Clone)]
struct QNameInfo {
  tag_prefix: String,
  local_name: String,
}

fn is_system_part_field(field: &syn::Field) -> bool {
  field
    .ident
    .as_ref()
    .map(|ident| {
      matches!(
        ident.to_string().as_str(),
        "content_types"
          | "relationships"
          | "rels_path"
          | "inner_path"
          | "root_element"
          | "part_content"
      )
    })
    .unwrap_or(false)
}

fn is_field_named(field: &syn::Field, expected: &str) -> bool {
  field
    .ident
    .as_ref()
    .map(|ident| ident == expected)
    .unwrap_or(false)
}

fn part_root_type_from_fields(fields: &syn::FieldsNamed) -> Option<Type> {
  fields
    .named
    .iter()
    .find(|field| is_field_named(field, "root_element"))
    .map(|field| field.ty.clone())
}

fn part_content_kind_from_fields(fields: &syn::FieldsNamed) -> Option<DerivedPartContentKind> {
  let field = fields
    .named
    .iter()
    .find(|field| is_field_named(field, "part_content"))?;
  let kind_name = type_last_ident(&field.ty)?.to_string();
  match kind_name.as_str() {
    "String" => Some(DerivedPartContentKind::Text),
    "Vec" => Some(DerivedPartContentKind::Binary),
    _ => None,
  }
}

fn parse_part_child_field(field: &syn::Field) -> syn::Result<Option<PartChildInfo>> {
  let Some(field_ident) = field.ident.clone() else {
    return Ok(None);
  };

  if let Some(inner_ty) = unwrap_vec_inner(&field.ty) {
    return Ok(Some(PartChildInfo {
      field_ident,
      ty: inner_ty,
      kind: PartChildKind::Repeated,
    }));
  }

  if let Some(inner_ty) = unwrap_box_inner(&field.ty) {
    return Ok(Some(PartChildInfo {
      field_ident,
      ty: inner_ty,
      kind: PartChildKind::Required,
    }));
  }

  if let Some(inner_ty) = unwrap_optional_box_inner(&field.ty) {
    return Ok(Some(PartChildInfo {
      field_ident,
      ty: inner_ty,
      kind: PartChildKind::Optional,
    }));
  }

  Ok(None)
}

fn unwrap_vec_inner(ty: &Type) -> Option<Type> {
  let Type::Path(TypePath { path, .. }) = ty else {
    return None;
  };
  let seg = path.segments.last()?;
  if seg.ident != "Vec" {
    return None;
  }
  let syn::PathArguments::AngleBracketed(args) = &seg.arguments else {
    return None;
  };
  args.args.iter().find_map(|arg| match arg {
    syn::GenericArgument::Type(inner) => Some(inner.clone()),
    _ => None,
  })
}

fn unwrap_box_inner(ty: &Type) -> Option<Type> {
  let Type::Path(TypePath { path, .. }) = ty else {
    return None;
  };
  let seg = path.segments.last()?;
  if seg.ident != "Box" {
    return None;
  }
  let syn::PathArguments::AngleBracketed(args) = &seg.arguments else {
    return None;
  };
  args.args.iter().find_map(|arg| match arg {
    syn::GenericArgument::Type(inner) => Some(inner.clone()),
    _ => None,
  })
}

fn unwrap_optional_box_inner(ty: &Type) -> Option<Type> {
  let Type::Path(TypePath { path, .. }) = ty else {
    return None;
  };
  let seg = path.segments.last()?;
  if seg.ident != "Option" {
    return None;
  }
  let syn::PathArguments::AngleBracketed(args) = &seg.arguments else {
    return None;
  };
  let inner = args.args.iter().find_map(|arg| match arg {
    syn::GenericArgument::Type(inner) => Some(inner.clone()),
    _ => None,
  })?;
  unwrap_box_inner(&inner)
}

fn type_last_ident(ty: &Type) -> Option<&Ident> {
  let Type::Path(TypePath { path, .. }) = ty else {
    return None;
  };
  path.segments.last().map(|segment| &segment.ident)
}

fn to_snake_case_local(value: &str) -> String {
  let mut out = String::with_capacity(value.len());
  let mut chars = value.chars().peekable();
  let mut prev_is_lower_or_digit = false;
  let mut prev_is_upper = false;
  while let Some(ch) = chars.next() {
    let next_is_lower = chars
      .peek()
      .map(|next| next.is_ascii_lowercase())
      .unwrap_or(false);
    if ch.is_ascii_uppercase() {
      if prev_is_lower_or_digit || (prev_is_upper && next_is_lower) {
        out.push('_');
      }
      out.push(ch.to_ascii_lowercase());
      prev_is_lower_or_digit = false;
      prev_is_upper = true;
    } else {
      prev_is_lower_or_digit = ch.is_ascii_lowercase() || ch.is_ascii_digit();
      prev_is_upper = false;
      out.push(ch);
    }
  }
  out
}

#[cfg(test)]
fn snapshot_file_stem(input: &DeriveInput) -> String {
  let raw_name = parse_sdk_qname(&input.attrs)
    .ok()
    .flatten()
    .unwrap_or_else(|| input.ident.to_string());
  sanitize_snapshot_component(&raw_name)
}

#[cfg(test)]
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
  let mut attr_name = None;
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in metas {
      match meta {
        Meta::List(meta) if meta.path.is_ident("attr") => {
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("qname") {
              let value: LitStr = nested.value()?.parse()?;
              attr_name = Some(normalize_attr_qname(&value.value()));
              Ok(())
            } else {
              Err(nested.error("unsupported sdk attr attribute"))
            }
          })?;
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
        Meta::List(meta) if meta.path.is_ident("text_child") => {
          let mut qname = None;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("qname") {
              let value: LitStr = nested.value()?.parse()?;
              qname = Some(value.value());
              Ok(())
            } else {
              Err(nested.error("unsupported sdk text_child attribute"))
            }
          })?;
          return Ok(Some(SdkTypeFieldKind::TextChild {
            qname: qname.unwrap_or_default(),
          }));
        }
        Meta::Path(path) if path.is_ident("text") => return Ok(Some(SdkTypeFieldKind::Text)),
        Meta::Path(path) if path.is_ident("choice") => return Ok(Some(SdkTypeFieldKind::Choice)),
        Meta::Path(path) if path.is_ident("any") => return Ok(Some(SdkTypeFieldKind::Any)),
        Meta::NameValue(meta) if meta.path.is_ident("bit") => {}
        Meta::Path(path)
          if path.is_ident("xmlns") || path.is_ident("mce") || path.is_ident("xml_header") =>
        {
          continue;
        }
        Meta::List(meta)
          if meta.path.is_ident("pattern")
            || meta.path.is_ident("string_length")
            || meta.path.is_ident("string_format")
            || meta.path.is_ident("string_set")
            || meta.path.is_ident("number_type")
            || meta.path.is_ident("number_range")
            || meta.path.is_ident("number_sign") =>
        {
          continue;
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
  if attr_name.is_some() {
    return Ok(Some(SdkTypeFieldKind::Attr {
      name: attr_name.unwrap_or_default(),
    }));
  }
  Ok(None)
}

fn parse_sdk_type_field_validators(attrs: &[Attribute]) -> syn::Result<Vec<SdkFieldValidator>> {
  let mut validators = Vec::new();

  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }

    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in metas {
      match meta {
        Meta::List(meta) if meta.path.is_ident("pattern") => {
          let mut regex = None;
          let mut source_id = 0;
          let mut union_id = None;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("regex") {
              let value: LitStr = nested.value()?.parse()?;
              regex = Some(value.value());
              Ok(())
            } else if nested.path.is_ident("source") {
              let value: syn::LitInt = nested.value()?.parse()?;
              source_id = value.base10_parse::<u32>()?;
              Ok(())
            } else if nested.path.is_ident("union") {
              let value: syn::LitInt = nested.value()?.parse()?;
              union_id = Some(value.base10_parse::<u64>()?);
              Ok(())
            } else {
              Err(nested.error("unsupported sdk pattern attribute"))
            }
          })?;
          if let Some(regex) = regex {
            validators.push(SdkFieldValidator::Pattern {
              regex,
              source_id,
              union_id,
            });
          }
        }
        Meta::List(meta) if meta.path.is_ident("string_length") => {
          let mut min = None;
          let mut max = None;
          let mut type_name = None;
          let mut source_id = 0;
          let mut union_id = None;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("min") {
              let value: syn::LitInt = nested.value()?.parse()?;
              min = Some(value.base10_parse::<u32>()?);
              Ok(())
            } else if nested.path.is_ident("max") {
              let value: syn::LitInt = nested.value()?.parse()?;
              max = Some(value.base10_parse::<u32>()?);
              Ok(())
            } else if nested.path.is_ident("type_name") {
              let value: LitStr = nested.value()?.parse()?;
              type_name = Some(value.value());
              Ok(())
            } else if nested.path.is_ident("source") {
              let value: syn::LitInt = nested.value()?.parse()?;
              source_id = value.base10_parse::<u32>()?;
              Ok(())
            } else if nested.path.is_ident("union") {
              let value: syn::LitInt = nested.value()?.parse()?;
              union_id = Some(value.base10_parse::<u64>()?);
              Ok(())
            } else {
              Err(nested.error("unsupported sdk string_length attribute"))
            }
          })?;
          validators.push(SdkFieldValidator::StringLength {
            min,
            max,
            type_name,
            source_id,
            union_id,
          });
        }
        Meta::List(meta) if meta.path.is_ident("number_type") => {
          let mut type_name = None;
          let mut source_id = 0;
          let mut union_id = None;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("type_name") {
              let value: LitStr = nested.value()?.parse()?;
              type_name = Some(value.value());
              Ok(())
            } else if nested.path.is_ident("source") {
              let value: syn::LitInt = nested.value()?.parse()?;
              source_id = value.base10_parse::<u32>()?;
              Ok(())
            } else if nested.path.is_ident("union") {
              let value: syn::LitInt = nested.value()?.parse()?;
              union_id = Some(value.base10_parse::<u64>()?);
              Ok(())
            } else {
              Err(nested.error("unsupported sdk number_type attribute"))
            }
          })?;
          if let Some(type_name) = type_name {
            validators.push(SdkFieldValidator::NumberType {
              type_name,
              source_id,
              union_id,
            });
          }
        }
        Meta::List(meta) if meta.path.is_ident("number_range") => {
          let mut min = None;
          let mut max = None;
          let mut min_inclusive = true;
          let mut max_inclusive = true;
          let mut source_id = 0;
          let mut union_id = None;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("min") {
              let value: LitStr = nested.value()?.parse()?;
              min = Some(value.value());
              Ok(())
            } else if nested.path.is_ident("max") {
              let value: LitStr = nested.value()?.parse()?;
              max = Some(value.value());
              Ok(())
            } else if nested.path.is_ident("min_inclusive") {
              let value: syn::LitBool = nested.value()?.parse()?;
              min_inclusive = value.value;
              Ok(())
            } else if nested.path.is_ident("max_inclusive") {
              let value: syn::LitBool = nested.value()?.parse()?;
              max_inclusive = value.value;
              Ok(())
            } else if nested.path.is_ident("source") {
              let value: syn::LitInt = nested.value()?.parse()?;
              source_id = value.base10_parse::<u32>()?;
              Ok(())
            } else if nested.path.is_ident("union") {
              let value: syn::LitInt = nested.value()?.parse()?;
              union_id = Some(value.base10_parse::<u64>()?);
              Ok(())
            } else {
              Err(nested.error("unsupported sdk number_range attribute"))
            }
          })?;
          validators.push(SdkFieldValidator::NumberRange {
            min,
            max,
            min_inclusive,
            max_inclusive,
            source_id,
            union_id,
          });
        }
        Meta::List(meta) if meta.path.is_ident("string_format") => {
          let mut kind = None;
          let mut source_id = 0;
          let mut union_id = None;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("kind") {
              let value: LitStr = nested.value()?.parse()?;
              kind = Some(match value.value().as_str() {
                "token" => SdkStringFormatKind::Token,
                "ncname" => SdkStringFormatKind::NcName,
                "qname" => SdkStringFormatKind::QName,
                "uri" => SdkStringFormatKind::Uri,
                "id" => SdkStringFormatKind::Id,
                _ => return Err(nested.error("unsupported sdk string_format kind")),
              });
              Ok(())
            } else if nested.path.is_ident("source") {
              let value: syn::LitInt = nested.value()?.parse()?;
              source_id = value.base10_parse::<u32>()?;
              Ok(())
            } else if nested.path.is_ident("union") {
              let value: syn::LitInt = nested.value()?.parse()?;
              union_id = Some(value.base10_parse::<u64>()?);
              Ok(())
            } else {
              Err(nested.error("unsupported sdk string_format attribute"))
            }
          })?;
          if let Some(kind) = kind {
            validators.push(SdkFieldValidator::StringFormat {
              kind,
              source_id,
              union_id,
            });
          }
        }
        Meta::List(meta) if meta.path.is_ident("string_set") => {
          let mut values = Vec::new();
          let mut source_id = 0;
          let mut union_id = None;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("values") {
              let expr: syn::Expr = nested.value()?.parse()?;
              let syn::Expr::Reference(reference) = expr else {
                return Err(nested.error("sdk string_set values must be a slice reference"));
              };
              let syn::Expr::Array(array) = *reference.expr else {
                return Err(nested.error("sdk string_set values must be an array"));
              };
              for element in array.elems {
                let syn::Expr::Lit(expr_lit) = element else {
                  return Err(nested.error("sdk string_set values must be string literals"));
                };
                let syn::Lit::Str(value) = expr_lit.lit else {
                  return Err(nested.error("sdk string_set values must be string literals"));
                };
                values.push(value.value());
              }
              Ok(())
            } else if nested.path.is_ident("source") {
              let value: syn::LitInt = nested.value()?.parse()?;
              source_id = value.base10_parse::<u32>()?;
              Ok(())
            } else if nested.path.is_ident("union") {
              let value: syn::LitInt = nested.value()?.parse()?;
              union_id = Some(value.base10_parse::<u64>()?);
              Ok(())
            } else {
              Err(nested.error("unsupported sdk string_set attribute"))
            }
          })?;
          validators.push(SdkFieldValidator::StringSet {
            values,
            source_id,
            union_id,
          });
        }
        Meta::List(meta) if meta.path.is_ident("number_sign") => {
          let mut kind = None;
          let mut source_id = 0;
          let mut union_id = None;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("kind") {
              let value: LitStr = nested.value()?.parse()?;
              kind = Some(match value.value().as_str() {
                "non_negative" => SdkNumberSignKind::NonNegative,
                "positive" => SdkNumberSignKind::Positive,
                _ => return Err(nested.error("unsupported sdk number_sign kind")),
              });
              Ok(())
            } else if nested.path.is_ident("source") {
              let value: syn::LitInt = nested.value()?.parse()?;
              source_id = value.base10_parse::<u32>()?;
              Ok(())
            } else if nested.path.is_ident("union") {
              let value: syn::LitInt = nested.value()?.parse()?;
              union_id = Some(value.base10_parse::<u64>()?);
              Ok(())
            } else {
              Err(nested.error("unsupported sdk number_sign attribute"))
            }
          })?;
          if let Some(kind) = kind {
            validators.push(SdkFieldValidator::NumberSign {
              kind,
              source_id,
              union_id,
            });
          }
        }
        _ => {}
      }
    }
  }

  Ok(validators)
}

fn parse_sdk_choice_variant_kind(attrs: &[Attribute]) -> syn::Result<Option<SdkChoiceVariantKind>> {
  let mut child_qnames = Vec::new();
  let mut text_child_qnames = Vec::new();
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
        Meta::List(meta) if meta.path.is_ident("text_child") => {
          let mut qname = None;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("qname") {
              let value: LitStr = nested.value()?.parse()?;
              qname = Some(value.value());
              Ok(())
            } else {
              Err(nested.error("unsupported sdk choice text_child attribute"))
            }
          })?;
          text_child_qnames.push(qname.unwrap_or_default());
        }
        Meta::Path(path) if path.is_ident("any") => return Ok(Some(SdkChoiceVariantKind::Any)),
        Meta::Path(path) if path.is_ident("text") => return Ok(Some(SdkChoiceVariantKind::Text)),
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
  if !text_child_qnames.is_empty() {
    return Ok(Some(SdkChoiceVariantKind::TextChild {
      qnames: text_child_qnames,
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

fn is_xmlns_field(ident: &Ident) -> bool {
  matches!(ident.to_string().as_str(), "xmlns" | "xmlns_map")
}

fn is_mc_ignorable_field(ident: &Ident) -> bool {
  ident == "mc_ignorable"
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

fn is_hex_binary_type(ty: &Type) -> bool {
  matches!(ty, Type::Path(TypePath { path, .. }) if path.segments.last().is_some_and(|segment| segment.ident == "HexBinaryValue"))
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

#[cfg(test)]
mod tests {
  use super::*;
  use std::{
    fs,
    io::Write,
    sync::{Mutex, OnceLock},
  };

  static SNAPSHOT_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
  static SNAPSHOT_INIT: OnceLock<()> = OnceLock::new();
  const SNAPSHOT_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../target/ooxmlsdk_macro_expanded"
  );

  fn dump_macro_expansion(kind: &str, input: &DeriveInput, tokens: &proc_macro2::TokenStream) {
    let lock = SNAPSHOT_LOCK.get_or_init(|| Mutex::new(()));
    let _guard = lock.lock().expect("snapshot lock");
    SNAPSHOT_INIT.get_or_init(|| {
      let _ = fs::remove_dir_all(SNAPSHOT_DIR);
      let _ = fs::create_dir_all(SNAPSHOT_DIR);
    });
    let kind_dir = format!("{SNAPSHOT_DIR}/{kind}");
    if fs::create_dir_all(&kind_dir).is_err() {
      return;
    }
    let snapshot_name = snapshot_file_stem(input);
    let file_path = format!("{kind_dir}/{snapshot_name}.rs");
    let mut file = match fs::File::create(&file_path) {
      Ok(file) => file,
      Err(_) => return,
    };
    let formatted = syn::parse2::<syn::File>(tokens.clone())
      .map(|file| prettyplease::unparse(&file))
      .unwrap_or_else(|_| tokens.to_string());
    let _ = writeln!(
      file,
      "// ===== {}: {} =====\n{}\n",
      kind, input.ident, formatted
    );
  }

  #[test]
  #[ignore]
  fn dump_context_node_expansion() {
    dump_one_expansion(
      &std::env::var("OOXMLSDK_DUMP_KIND").unwrap_or_else(|_| "SdkPart".to_string()),
      &std::env::var("OOXMLSDK_DUMP_FILE")
        .unwrap_or_else(|_| "parts/main_document_part.rs".to_string()),
      &std::env::var("OOXMLSDK_DUMP_TARGET").unwrap_or_else(|_| "MainDocumentPart".to_string()),
    );
  }

  fn dump_one_expansion(kind: &str, file: &str, target: &str) {
    let source = fs::read_to_string(format!(
      "{}/../../crates/ooxmlsdk/src/{}",
      env!("CARGO_MANIFEST_DIR"),
      file
    ))
    .expect("read runtime source file");
    let item_src = extract_derive_item(&source, kind, target)
      .unwrap_or_else(|| panic!("no {kind} derive named {target} found in runtime source"));
    let input = syn::parse_str::<DeriveInput>(&item_src).expect("parse part derive");
    let tokens = match kind {
      "SdkEnum" => sdk_enum::expand_sdk_enum(&input).expect("SdkEnum expansion"),
      "SdkType" => sdk_type::expand_sdk_type(&input).expect("SdkType expansion"),
      "SdkChoice" => sdk_choice::expand_sdk_choice(&input).expect("SdkChoice expansion"),
      "SdkPart" => sdk_part::expand_sdk_part(&input).expect("SdkPart expansion"),
      other => panic!("unexpected kind: {other}"),
    };
    dump_macro_expansion(kind, &input, &tokens);
  }

  fn extract_derive_item(source: &str, kind: &str, target: &str) -> Option<String> {
    let target_patterns = [
      format!("pub struct {target} {{"),
      format!("pub struct {target}("),
      format!("struct {target} {{"),
      format!("struct {target}("),
      format!("pub enum {target} {{"),
      format!("enum {target} {{"),
    ];
    let target_pos = target_patterns
      .iter()
      .find_map(|pattern| source.find(pattern))?;
    let derive_pos = source[..target_pos].rfind("#[derive(")?;
    if !source[derive_pos..target_pos].contains(kind) {
      return None;
    }
    let start = derive_pos;

    let mut item = String::new();
    let mut started = false;
    let mut brace_depth = 0isize;
    let mut item_name = None::<String>;
    for line in source[start..].lines() {
      item.push_str(line);
      item.push('\n');
      if item_name.is_none() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("pub struct ") {
          item_name = rest
            .split(['{', '('])
            .next()
            .map(|s| s.split_whitespace().next().unwrap_or_default().to_string());
        } else if let Some(rest) = trimmed.strip_prefix("struct ") {
          item_name = rest
            .split(['{', '('])
            .next()
            .map(|s| s.split_whitespace().next().unwrap_or_default().to_string());
        } else if let Some(rest) = trimmed.strip_prefix("pub enum ") {
          item_name = rest
            .split('{')
            .next()
            .map(|s| s.split_whitespace().next().unwrap_or_default().to_string());
        } else if let Some(rest) = trimmed.strip_prefix("enum ") {
          item_name = rest
            .split('{')
            .next()
            .map(|s| s.split_whitespace().next().unwrap_or_default().to_string());
        }
      }
      if !started {
        if line.contains(" struct ")
          || line.trim_start().starts_with("struct ")
          || line.contains(" enum ")
          || line.trim_start().starts_with("enum ")
        {
          started = true;
          brace_depth += line.chars().filter(|ch| *ch == '{').count() as isize;
          brace_depth -= line.chars().filter(|ch| *ch == '}').count() as isize;
          if brace_depth <= 0 && line.contains(';') {
            break;
          }
          continue;
        }
        continue;
      }

      brace_depth += line.chars().filter(|ch| *ch == '{').count() as isize;
      brace_depth -= line.chars().filter(|ch| *ch == '}').count() as isize;
      if brace_depth <= 0 {
        break;
      }
    }

    if item_name.as_deref() == Some(target) {
      Some(item)
    } else {
      None
    }
  }
}
