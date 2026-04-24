use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
  Attribute, Data, DataEnum, DeriveInput, Fields, Ident, LitByteStr, LitStr, Meta, Token, Type,
  TypePath, bracketed,
  parse::{Parse, ParseStream},
  parse_macro_input, parse_str,
  punctuated::Punctuated,
};

mod sdk_choice;
mod sdk_enum;
mod sdk_package;
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

#[proc_macro_derive(SdkPackage, attributes(sdk))]
pub fn sdk_package(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  match sdk_package::expand_sdk_package(&input) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.to_compile_error().into(),
  }
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum PartFieldMarker {
  Rid,
  ContentTypes,
  Relationships,
  RelsPath,
  InnerPath,
  Root,
  ExtendedParts,
}

#[derive(Clone)]
struct PartChildAttr {
  relationship_type: String,
  kind: PartChildKind,
}

#[derive(Clone)]
enum PartRelationshipTypeSource {
  Explicit(String),
  TypeConst,
}

#[derive(Clone)]
struct PartChildInfo {
  field_ident: Ident,
  ty: Type,
  kind: PartChildKind,
  relationship_type: PartRelationshipTypeSource,
}

#[derive(Clone)]
struct PartDataRefAttr {
  relationship_type: String,
  kind: PartChildKind,
}

#[derive(Clone)]
struct PartDataRefInfo {
  field_ident: Ident,
  ty: Type,
  kind: PartChildKind,
  relationship_type: PartRelationshipTypeSource,
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
  accepts_text: Option<bool>,
  accepts_any: Option<bool>,
  specific_qnames: Vec<String>,
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

#[derive(Clone)]
struct ParsedSdkTypeFieldAttrs {
  kind: Option<SdkTypeFieldKind>,
  choice_accepts_text: Option<bool>,
  choice_accepts_any: Option<bool>,
  choice_qnames: Vec<String>,
  validators: Vec<SdkFieldValidator>,
}

struct StringSetValues {
  values: Vec<String>,
}

impl Parse for StringSetValues {
  fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
    input.parse::<Token![&]>()?;
    let content;
    bracketed!(content in input);
    let literals = Punctuated::<LitStr, Token![,]>::parse_terminated(&content)?;

    Ok(Self {
      values: literals
        .into_iter()
        .map(|literal| literal.value())
        .collect(),
    })
  }
}

enum SdkChoiceVariantKind {
  Child { qnames: Vec<String> },
  Choice,
  Sequence,
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
  part_field_marker(field).ok().flatten().is_some()
    || part_content_kind(field).ok().flatten().is_some()
}

fn part_root_type_from_fields(fields: &syn::FieldsNamed) -> Option<Type> {
  fields
    .named
    .iter()
    .find(|field| matches!(part_field_marker(field), Ok(Some(PartFieldMarker::Root))))
    .map(|field| field.ty.clone())
}

fn part_content_kind_from_fields(fields: &syn::FieldsNamed) -> Option<DerivedPartContentKind> {
  fields
    .named
    .iter()
    .find_map(|field| part_content_kind(field).ok().flatten())
}

fn parse_part_child_field(field: &syn::Field) -> syn::Result<Option<PartChildInfo>> {
  let Some(field_ident) = field.ident.clone() else {
    return Ok(None);
  };

  if let Some(explicit) = parse_part_child_attr(&field.attrs)? {
    let inner_ty = match explicit.kind {
      PartChildKind::Repeated => unwrap_vec_inner(&field.ty),
      PartChildKind::Required => unwrap_box_inner(&field.ty),
      PartChildKind::Optional => unwrap_optional_box_inner(&field.ty),
    }
    .ok_or_else(|| {
      syn::Error::new_spanned(
        &field.ty,
        "part_child field type does not match declared kind",
      )
    })?;

    return Ok(Some(PartChildInfo {
      field_ident,
      ty: inner_ty,
      kind: explicit.kind,
      relationship_type: PartRelationshipTypeSource::Explicit(explicit.relationship_type),
    }));
  }

  if let Some((kind, inner_ty)) = infer_part_field_kind_and_inner_type(field)
    && is_part_terminal_type(&inner_ty)
    && !is_data_ref_terminal_type(&inner_ty)
  {
    return Ok(Some(PartChildInfo {
      field_ident,
      ty: inner_ty,
      kind,
      relationship_type: PartRelationshipTypeSource::TypeConst,
    }));
  }

  Ok(None)
}

fn parse_part_data_ref_field(field: &syn::Field) -> syn::Result<Option<PartDataRefInfo>> {
  let Some(field_ident) = field.ident.clone() else {
    return Ok(None);
  };

  if let Some(explicit) = parse_part_data_ref_attr(&field.attrs)? {
    let inner_ty = match explicit.kind {
      PartChildKind::Repeated => unwrap_vec_inner(&field.ty),
      PartChildKind::Required => unwrap_box_inner(&field.ty),
      PartChildKind::Optional => unwrap_optional_box_inner(&field.ty),
    }
    .ok_or_else(|| {
      syn::Error::new_spanned(
        &field.ty,
        "part_data_ref field type does not match declared kind",
      )
    })?;

    return Ok(Some(PartDataRefInfo {
      field_ident,
      ty: inner_ty,
      kind: explicit.kind,
      relationship_type: PartRelationshipTypeSource::Explicit(explicit.relationship_type),
    }));
  }

  if let Some((kind, inner_ty)) = infer_part_field_kind_and_inner_type(field)
    && is_data_ref_terminal_type(&inner_ty)
  {
    return Ok(Some(PartDataRefInfo {
      field_ident,
      ty: inner_ty,
      kind,
      relationship_type: PartRelationshipTypeSource::TypeConst,
    }));
  }

  Ok(None)
}

fn parse_part_field_marker(attrs: &[Attribute]) -> syn::Result<Option<PartFieldMarker>> {
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in metas {
      match meta {
        Meta::Path(path) if path.is_ident("part_rid") => return Ok(Some(PartFieldMarker::Rid)),
        Meta::Path(path) if path.is_ident("part_content_types") => {
          return Ok(Some(PartFieldMarker::ContentTypes));
        }
        Meta::Path(path) if path.is_ident("part_relationships") => {
          return Ok(Some(PartFieldMarker::Relationships));
        }
        Meta::Path(path) if path.is_ident("part_rels_path") => {
          return Ok(Some(PartFieldMarker::RelsPath));
        }
        Meta::Path(path) if path.is_ident("part_inner_path") => {
          return Ok(Some(PartFieldMarker::InnerPath));
        }
        Meta::Path(path) if path.is_ident("part_root") => return Ok(Some(PartFieldMarker::Root)),
        Meta::Path(path) if path.is_ident("part_extended_parts") => {
          return Ok(Some(PartFieldMarker::ExtendedParts));
        }
        _ => {}
      }
    }
  }
  Ok(None)
}

fn part_field_marker(field: &syn::Field) -> syn::Result<Option<PartFieldMarker>> {
  if let Some(marker) = parse_part_field_marker(&field.attrs)? {
    return Ok(Some(marker));
  }

  Ok(infer_part_field_marker(field))
}

fn infer_part_field_marker(field: &syn::Field) -> Option<PartFieldMarker> {
  let ident = field.ident.as_ref()?.to_string();

  match ident.as_str() {
    "r_id" if is_string_type(&field.ty) => Some(PartFieldMarker::Rid),
    "content_types" if is_terminal_type(&field.ty, "Types") => Some(PartFieldMarker::ContentTypes),
    "relationships" if is_option_of_terminal_type(&field.ty, "Relationships") => {
      Some(PartFieldMarker::Relationships)
    }
    "rels_path" if is_string_type(&field.ty) => Some(PartFieldMarker::RelsPath),
    "inner_path" if is_string_type(&field.ty) => Some(PartFieldMarker::InnerPath),
    "root_element" if !is_wrapped_collection_type(&field.ty) => Some(PartFieldMarker::Root),
    "extended_parts" if is_vec_of_terminal_type(&field.ty, "ExtendedPart") => {
      Some(PartFieldMarker::ExtendedParts)
    }
    _ => None,
  }
}

fn parse_part_content_attr(attrs: &[Attribute]) -> syn::Result<Option<DerivedPartContentKind>> {
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in metas {
      if let Meta::List(meta) = meta
        && meta.path.is_ident("part_content")
      {
        let mut kind = None;
        meta.parse_nested_meta(|nested| {
          if nested.path.is_ident("kind") {
            let value: LitStr = nested.value()?.parse()?;
            kind = Some(match value.value().as_str() {
              "text" => DerivedPartContentKind::Text,
              "binary" => DerivedPartContentKind::Binary,
              _ => return Err(nested.error("unsupported sdk part_content kind")),
            });
            Ok(())
          } else {
            Err(nested.error("unsupported sdk part_content attribute"))
          }
        })?;
        return Ok(kind);
      }
    }
  }
  Ok(None)
}

fn part_content_kind(field: &syn::Field) -> syn::Result<Option<DerivedPartContentKind>> {
  if let Some(kind) = parse_part_content_attr(&field.attrs)? {
    return Ok(Some(kind));
  }

  let Some(field_ident) = &field.ident else {
    return Ok(None);
  };
  if field_ident != "part_content" {
    return Ok(None);
  }

  if is_string_type(&field.ty) {
    return Ok(Some(DerivedPartContentKind::Text));
  }
  if is_vec_of_u8_type(&field.ty) {
    return Ok(Some(DerivedPartContentKind::Binary));
  }

  Ok(None)
}

fn parse_part_child_attr(attrs: &[Attribute]) -> syn::Result<Option<PartChildAttr>> {
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in metas {
      if let Meta::List(meta) = meta
        && meta.path.is_ident("part_child")
      {
        let mut relationship_type = None;
        let mut kind = None;
        meta.parse_nested_meta(|nested| {
          if nested.path.is_ident("relationship_type") {
            let value: LitStr = nested.value()?.parse()?;
            relationship_type = Some(value.value());
            Ok(())
          } else if nested.path.is_ident("kind") {
            let value: LitStr = nested.value()?.parse()?;
            kind = Some(match value.value().as_str() {
              "optional" => PartChildKind::Optional,
              "required" => PartChildKind::Required,
              "repeated" => PartChildKind::Repeated,
              _ => return Err(nested.error("unsupported sdk part_child kind")),
            });
            Ok(())
          } else {
            Err(nested.error("unsupported sdk part_child attribute"))
          }
        })?;
        let Some(relationship_type) = relationship_type else {
          return Err(syn::Error::new_spanned(
            meta,
            "sdk part_child requires relationship_type",
          ));
        };
        let Some(kind) = kind else {
          return Err(syn::Error::new_spanned(
            meta,
            "sdk part_child requires kind",
          ));
        };
        return Ok(Some(PartChildAttr {
          relationship_type,
          kind,
        }));
      }
    }
  }
  Ok(None)
}

fn parse_part_child_relationship_type_attr(attrs: &[Attribute]) -> syn::Result<Option<String>> {
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in metas {
      if let Meta::List(meta) = meta
        && meta.path.is_ident("part_child")
      {
        let mut relationship_type = None;
        meta.parse_nested_meta(|nested| {
          if nested.path.is_ident("relationship_type") {
            let value: LitStr = nested.value()?.parse()?;
            relationship_type = Some(value.value());
            Ok(())
          } else if nested.path.is_ident("kind") {
            let _value: LitStr = nested.value()?.parse()?;
            Ok(())
          } else {
            Err(nested.error("unsupported sdk part_child attribute"))
          }
        })?;
        return Ok(relationship_type);
      }
    }
  }
  Ok(None)
}

fn parse_part_data_ref_attr(attrs: &[Attribute]) -> syn::Result<Option<PartDataRefAttr>> {
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in metas {
      if let Meta::List(meta) = meta
        && meta.path.is_ident("part_data_ref")
      {
        let mut relationship_type = None;
        let mut kind = None;
        let mut has_reference_kind = false;
        meta.parse_nested_meta(|nested| {
          if nested.path.is_ident("relationship_type") {
            let value: LitStr = nested.value()?.parse()?;
            relationship_type = Some(value.value());
            Ok(())
          } else if nested.path.is_ident("kind") {
            let value: LitStr = nested.value()?.parse()?;
            kind = Some(match value.value().as_str() {
              "optional" => PartChildKind::Optional,
              "required" => PartChildKind::Required,
              "repeated" => PartChildKind::Repeated,
              _ => return Err(nested.error("unsupported sdk part_data_ref kind")),
            });
            Ok(())
          } else if nested.path.is_ident("reference_kind") {
            let value: LitStr = nested.value()?.parse()?;
            match value.value().as_str() {
              "audio" | "media" | "video" => {
                has_reference_kind = true;
              }
              _ => return Err(nested.error("unsupported sdk part_data_ref reference_kind")),
            }
            Ok(())
          } else {
            Err(nested.error("unsupported sdk part_data_ref attribute"))
          }
        })?;
        let Some(relationship_type) = relationship_type else {
          return Err(syn::Error::new_spanned(
            meta,
            "sdk part_data_ref requires relationship_type",
          ));
        };
        let Some(kind) = kind else {
          return Err(syn::Error::new_spanned(
            meta,
            "sdk part_data_ref requires kind",
          ));
        };
        if !has_reference_kind {
          return Err(syn::Error::new_spanned(
            meta,
            "sdk part_data_ref requires reference_kind",
          ));
        }
        return Ok(Some(PartDataRefAttr {
          relationship_type,
          kind,
        }));
      }
    }
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

fn infer_part_field_kind_and_inner_type(field: &syn::Field) -> Option<(PartChildKind, Type)> {
  if let Some(inner) = unwrap_vec_inner(&field.ty) {
    return Some((PartChildKind::Repeated, inner));
  }
  if let Some(inner) = unwrap_optional_box_inner(&field.ty) {
    return Some((PartChildKind::Optional, inner));
  }
  if let Some(inner) = unwrap_box_inner(&field.ty) {
    return Some((PartChildKind::Required, inner));
  }
  None
}

fn is_string_type(ty: &Type) -> bool {
  is_terminal_type(ty, "String")
}

fn is_terminal_type(ty: &Type, expected: &str) -> bool {
  let Type::Path(TypePath { path, .. }) = ty else {
    return false;
  };

  path
    .segments
    .last()
    .map(|segment| segment.ident == expected)
    .unwrap_or(false)
}

fn is_option_of_terminal_type(ty: &Type, expected: &str) -> bool {
  let Type::Path(TypePath { path, .. }) = ty else {
    return false;
  };
  let Some(seg) = path.segments.last() else {
    return false;
  };
  if seg.ident != "Option" {
    return false;
  }
  let syn::PathArguments::AngleBracketed(args) = &seg.arguments else {
    return false;
  };

  args.args.iter().any(|arg| match arg {
    syn::GenericArgument::Type(inner) => is_terminal_type(inner, expected),
    _ => false,
  })
}

fn is_vec_of_terminal_type(ty: &Type, expected: &str) -> bool {
  unwrap_vec_inner(ty)
    .map(|inner| is_terminal_type(&inner, expected))
    .unwrap_or(false)
}

fn is_vec_of_u8_type(ty: &Type) -> bool {
  unwrap_vec_inner(ty)
    .map(|inner| matches!(inner, Type::Path(TypePath { path, .. }) if path.is_ident("u8")))
    .unwrap_or(false)
}

fn terminal_type_ident(ty: &Type) -> Option<String> {
  let Type::Path(TypePath { path, .. }) = ty else {
    return None;
  };

  path
    .segments
    .last()
    .map(|segment| segment.ident.to_string())
}

fn is_part_terminal_type(ty: &Type) -> bool {
  terminal_type_ident(ty)
    .map(|ident| ident.ends_with("Part"))
    .unwrap_or(false)
}

fn is_data_ref_terminal_type(ty: &Type) -> bool {
  matches!(
    terminal_type_ident(ty).as_deref(),
    Some("AudioReferenceRelationship")
      | Some("MediaReferenceRelationship")
      | Some("VideoReferenceRelationship")
  )
}

fn is_wrapped_collection_type(ty: &Type) -> bool {
  unwrap_vec_inner(ty).is_some()
    || unwrap_box_inner(ty).is_some()
    || unwrap_optional_box_inner(ty).is_some()
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

fn parse_sdk_type_field_attrs(attrs: &[Attribute]) -> syn::Result<ParsedSdkTypeFieldAttrs> {
  let mut attr_name = None;
  let mut kind = None;
  let mut choice_accepts_text = None;
  let mut choice_accepts_any = None;
  let mut choice_qnames = Vec::new();
  let mut validators = Vec::new();

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
          kind = Some(SdkTypeFieldKind::Child {
            qname: qname.unwrap_or_default(),
          });
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
          kind = Some(SdkTypeFieldKind::TextChild {
            qname: qname.unwrap_or_default(),
          });
        }
        Meta::Path(path) if path.is_ident("text") => {
          kind = Some(SdkTypeFieldKind::Text);
        }
        Meta::List(meta) if meta.path.is_ident("choice") => {
          let mut accepts_text = false;
          let mut accepts_any = false;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("text") {
              accepts_text = true;
              Ok(())
            } else if nested.path.is_ident("any") {
              accepts_any = true;
              Ok(())
            } else if nested.path.is_ident("qname") {
              let value: LitStr = nested.value()?.parse()?;
              choice_qnames.push(value.value());
              Ok(())
            } else {
              Err(nested.error("unsupported sdk choice attribute"))
            }
          })?;
          kind = Some(SdkTypeFieldKind::Choice);
          choice_accepts_text = Some(choice_accepts_text.unwrap_or(false) || accepts_text);
          choice_accepts_any = Some(choice_accepts_any.unwrap_or(false) || accepts_any);
        }
        Meta::Path(path) if path.is_ident("choice") => {
          kind = Some(SdkTypeFieldKind::Choice);
          choice_accepts_text.get_or_insert(false);
          choice_accepts_any.get_or_insert(false);
        }
        Meta::NameValue(meta) if meta.path.is_ident("choice_accepts_text") => {
          let value = match &meta.value {
            syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
              syn::Lit::Bool(value) => value.value,
              _ => {
                return Err(syn::Error::new_spanned(
                  &expr_lit.lit,
                  "sdk choice_accepts_text expects a bool literal",
                ));
              }
            },
            _ => {
              return Err(syn::Error::new_spanned(
                &meta.value,
                "sdk choice_accepts_text expects a bool literal",
              ));
            }
          };
          choice_accepts_text = Some(value);
        }
        Meta::Path(path) if path.is_ident("any") => {
          kind = Some(SdkTypeFieldKind::Any);
        }
        Meta::NameValue(meta) if meta.path.is_ident("bit") => {}
        Meta::Path(path)
          if path.is_ident("xmlns") || path.is_ident("mce") || path.is_ident("xml_header") =>
        {
          continue;
        }
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
          let mut kind_value = None;
          let mut source_id = 0;
          let mut union_id = None;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("kind") {
              let value: LitStr = nested.value()?.parse()?;
              kind_value = Some(match value.value().as_str() {
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
          if let Some(kind) = kind_value {
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
              values = nested.value()?.parse::<StringSetValues>()?.values;
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
          let mut kind_value = None;
          let mut source_id = 0;
          let mut union_id = None;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("kind") {
              let value: LitStr = nested.value()?.parse()?;
              kind_value = Some(match value.value().as_str() {
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
          if let Some(kind) = kind_value {
            validators.push(SdkFieldValidator::NumberSign {
              kind,
              source_id,
              union_id,
            });
          }
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

  if kind.is_none() && attr_name.is_some() {
    kind = Some(SdkTypeFieldKind::Attr {
      name: attr_name.unwrap_or_default(),
    });
  }

  Ok(ParsedSdkTypeFieldAttrs {
    kind,
    choice_accepts_text,
    choice_accepts_any,
    choice_qnames,
    validators,
  })
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
        Meta::Path(path) if path.is_ident("choice") => {
          return Ok(Some(SdkChoiceVariantKind::Choice));
        }
        Meta::Path(path) if path.is_ident("sequence") => {
          return Ok(Some(SdkChoiceVariantKind::Sequence));
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
  ident == "xmlns"
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum BoolTypeKind {
  PlainBool,
  BooleanValue,
  OnOffValue,
  TrueFalseBlankValue,
  TrueFalseValue,
}

fn bool_type_kind(ty: &Type) -> Option<BoolTypeKind> {
  let Type::Path(TypePath { path, .. }) = ty else {
    return None;
  };
  let ident = path.segments.last()?.ident.to_string();
  Some(match ident.as_str() {
    "bool" => BoolTypeKind::PlainBool,
    "BooleanValue" => BoolTypeKind::BooleanValue,
    "OnOffValue" => BoolTypeKind::OnOffValue,
    "TrueFalseBlankValue" => BoolTypeKind::TrueFalseBlankValue,
    "TrueFalseValue" => BoolTypeKind::TrueFalseValue,
    _ => return None,
  })
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
    )
  }))
}

fn is_sdk_enum_type(ty: &Type) -> bool {
  matches!(ty, Type::Path(TypePath { path, .. }) if path.segments.last().is_some_and(|segment| {
    segment.ident.to_string().ends_with("Values")
  }))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum IntegerTypeKind {
  U8,
  I8,
  U16,
  I16,
  U32,
  I32,
  U64,
  I64,
}

fn integer_type_kind(ty: &Type) -> Option<IntegerTypeKind> {
  let Type::Path(TypePath { path, .. }) = ty else {
    return None;
  };
  let ident = path.segments.last()?.ident.to_string();
  Some(match ident.as_str() {
    "ByteValue" | "u8" => IntegerTypeKind::U8,
    "SByteValue" | "i8" => IntegerTypeKind::I8,
    "UInt16Value" | "u16" => IntegerTypeKind::U16,
    "Int16Value" | "i16" => IntegerTypeKind::I16,
    "UInt32Value" | "u32" => IntegerTypeKind::U32,
    "Int32Value" | "i32" => IntegerTypeKind::I32,
    "UInt64Value" | "u64" => IntegerTypeKind::U64,
    "Int64Value" | "IntegerValue" | "i64" => IntegerTypeKind::I64,
    _ => return None,
  })
}

fn is_xml_schema_float_type(ty: &Type) -> bool {
  matches!(ty, Type::Path(TypePath { path, .. }) if path.segments.last().is_some_and(|segment| {
    matches!(segment.ident.to_string().as_str(), "DoubleValue" | "SingleValue" | "f64" | "f32")
  }))
}

fn write_xml_schema_float_tokens(
  value_expr: proc_macro2::TokenStream,
  float_ty: &Type,
) -> proc_macro2::TokenStream {
  let (positive_infinity, negative_infinity) = if matches!(float_ty, Type::Path(TypePath { path, .. }) if path.segments.last().is_some_and(|segment| matches!(segment.ident.to_string().as_str(), "SingleValue" | "f32")))
  {
    (quote! { f32::INFINITY }, quote! { f32::NEG_INFINITY })
  } else {
    (quote! { f64::INFINITY }, quote! { f64::NEG_INFINITY })
  };

  quote! {
    {
      let __ooxmlsdk_float_value = *#value_expr;
      if __ooxmlsdk_float_value.is_nan() {
        writer.write_all(b"NaN")?;
      } else if __ooxmlsdk_float_value == #positive_infinity {
        writer.write_all(b"INF")?;
      } else if __ooxmlsdk_float_value == #negative_infinity {
        writer.write_all(b"-INF")?;
      } else {
        crate::common::write_escaped_text(writer, #value_expr)?;
      }
    }
  }
}

fn parse_bool_tokens(
  value_expr: proc_macro2::TokenStream,
  bool_ty: &Type,
  owner_expr: proc_macro2::TokenStream,
  field_expr: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  match bool_type_kind(bool_ty) {
    Some(BoolTypeKind::BooleanValue) => quote! {
      crate::common::parse_boolean_value_str(#value_expr, #owner_expr, #field_expr)?
    },
    Some(BoolTypeKind::OnOffValue) => quote! {
      crate::common::parse_on_off_str(#value_expr, #owner_expr, #field_expr)?
    },
    Some(BoolTypeKind::TrueFalseBlankValue) => quote! {
      crate::common::parse_true_false_blank_str(#value_expr, #owner_expr, #field_expr)?
    },
    Some(BoolTypeKind::TrueFalseValue) => quote! {
      crate::common::parse_true_false_str(#value_expr, #owner_expr, #field_expr)?
    },
    Some(BoolTypeKind::PlainBool) => quote! {
      crate::common::parse_bool_str(#value_expr, #owner_expr, #field_expr)?
    },
    None => unreachable!("parse_bool_tokens requires a bool-like type"),
  }
}

fn parse_bool_attr_tokens(
  attr_expr: proc_macro2::TokenStream,
  decoder_expr: proc_macro2::TokenStream,
  bool_ty: &Type,
  owner_expr: proc_macro2::TokenStream,
  field_expr: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  match bool_type_kind(bool_ty) {
    Some(BoolTypeKind::BooleanValue) => quote! {
      crate::common::parse_boolean_value_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    Some(BoolTypeKind::OnOffValue) => quote! {
      crate::common::parse_on_off_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    Some(BoolTypeKind::TrueFalseBlankValue) => quote! {
      crate::common::parse_true_false_blank_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    Some(BoolTypeKind::TrueFalseValue) => quote! {
      crate::common::parse_true_false_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    Some(BoolTypeKind::PlainBool) => quote! {
      crate::common::parse_bool_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    None => unreachable!("parse_bool_attr_tokens requires a bool-like type"),
  }
}

fn parse_integer_attr_tokens(
  attr_expr: proc_macro2::TokenStream,
  decoder_expr: proc_macro2::TokenStream,
  integer_ty: &Type,
  owner_expr: proc_macro2::TokenStream,
  field_expr: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  match integer_type_kind(integer_ty) {
    Some(IntegerTypeKind::U8) => quote! {
      crate::common::parse_u8_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    Some(IntegerTypeKind::I8) => quote! {
      crate::common::parse_i8_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    Some(IntegerTypeKind::U16) => quote! {
      crate::common::parse_u16_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    Some(IntegerTypeKind::I16) => quote! {
      crate::common::parse_i16_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    Some(IntegerTypeKind::U32) => quote! {
      crate::common::parse_u32_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    Some(IntegerTypeKind::I32) => quote! {
      crate::common::parse_i32_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    Some(IntegerTypeKind::U64) => quote! {
      crate::common::parse_u64_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    Some(IntegerTypeKind::I64) => quote! {
      crate::common::parse_i64_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    None => unreachable!("parse_integer_attr_tokens requires an integer-like type"),
  }
}

fn write_bool_tokens(
  value_expr: proc_macro2::TokenStream,
  bool_ty: &Type,
) -> proc_macro2::TokenStream {
  match bool_type_kind(bool_ty) {
    Some(BoolTypeKind::BooleanValue) => quote! {
      writer.write_all(if *#value_expr { b"1" } else { b"0" })?;
    },
    Some(_) => quote! {
      writer.write_all(if *#value_expr { b"true" } else { b"false" })?;
    },
    None => unreachable!("write_bool_tokens requires a bool-like type"),
  }
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
  const RUNTIME_SRC_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../ooxmlsdk/src");

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

  #[test]
  fn infers_part_system_markers_from_field_name_and_type() {
    let relationships: syn::Field = syn::parse_quote! { pub relationships: Option<crate::schemas::opc_relationships::Relationships> };
    let root_element: syn::Field = syn::parse_quote! { pub root_element: crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document };
    let extended_parts: syn::Field =
      syn::parse_quote! { pub extended_parts: Vec<crate::common::extended_part::ExtendedPart> };
    let binary_part_content: syn::Field = syn::parse_quote! { pub part_content: Vec<u8> };
    let text_part_content: syn::Field = syn::parse_quote! { pub part_content: String };

    assert!(matches!(
      part_field_marker(&relationships),
      Ok(Some(PartFieldMarker::Relationships))
    ));
    assert!(matches!(
      part_field_marker(&root_element),
      Ok(Some(PartFieldMarker::Root))
    ));
    assert!(matches!(
      part_field_marker(&extended_parts),
      Ok(Some(PartFieldMarker::ExtendedParts))
    ));
    assert!(matches!(
      part_content_kind(&binary_part_content),
      Ok(Some(DerivedPartContentKind::Binary))
    ));
    assert!(matches!(
      part_content_kind(&text_part_content),
      Ok(Some(DerivedPartContentKind::Text))
    ));
  }

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

    let file_path = format!("{kind_dir}/{}.rs", snapshot_file_stem(input));
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

  fn dump_one_expansion(kind: &str, file: &str, target: &str) {
    let source =
      fs::read_to_string(format!("{RUNTIME_SRC_DIR}/{file}")).expect("read runtime source file");
    let item_src = extract_derive_item(&source, kind, target)
      .unwrap_or_else(|| panic!("no {kind} derive named {target} found in runtime source"));
    let input = syn::parse_str::<DeriveInput>(&item_src).expect("parse derive input");
    let tokens = match kind {
      "SdkEnum" => sdk_enum::expand_sdk_enum(&input).expect("SdkEnum expansion"),
      "SdkType" => sdk_type::expand_sdk_type(&input).expect("SdkType expansion"),
      "SdkChoice" => sdk_choice::expand_sdk_choice(&input).expect("SdkChoice expansion"),
      "SdkPart" => sdk_part::expand_sdk_part(&input).expect("SdkPart expansion"),
      other => panic!("unexpected kind: {other}"),
    };
    dump_macro_expansion(kind, &input, &tokens);
  }

  fn snapshot_file_stem(input: &DeriveInput) -> String {
    let ident = input.ident.to_string();
    let mut stem = String::with_capacity(ident.len() * 2);
    for (idx, ch) in ident.chars().enumerate() {
      if ch.is_ascii_uppercase() {
        if idx > 0 {
          stem.push('_');
        }
        stem.push(ch.to_ascii_lowercase());
      } else if ch.is_ascii_alphanumeric() {
        stem.push(ch);
      } else {
        stem.push('_');
      }
    }
    stem
  }

  fn extract_derive_item(source: &str, kind: &str, target: &str) -> Option<String> {
    let syntax_markers = [
      format!("pub struct {target} {{"),
      format!("pub struct {target}("),
      format!("struct {target} {{"),
      format!("struct {target}("),
      format!("pub enum {target} {{"),
      format!("enum {target} {{"),
    ];
    let target_pos = syntax_markers
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
        }
        continue;
      }

      brace_depth += line.chars().filter(|ch| *ch == '{').count() as isize;
      brace_depth -= line.chars().filter(|ch| *ch == '}').count() as isize;
      if brace_depth <= 0 {
        break;
      }
    }

    (item_name.as_deref() == Some(target)).then_some(item)
  }
}
