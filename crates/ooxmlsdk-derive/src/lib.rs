use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
  Attribute, Data, DataEnum, DeriveInput, Expr, ExprArray, ExprLit, ExprRange, ExprUnary, Fields,
  Ident, Lit, LitByteStr, LitStr, Meta, RangeLimits, Token, Type, TypePath, UnOp, bracketed,
  parse::{Parse, ParseStream},
  parse_macro_input, parse_str,
  punctuated::Punctuated,
};

mod namespaces;
mod sdk_enum;
mod sdk_package;
mod sdk_part;
mod sdk_part_ref;
mod sdk_type;
mod sdk_xml_namespace;
mod simple_type_mapping;

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

#[proc_macro_derive(SdkPart, attributes(sdk))]
pub fn sdk_part(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  match sdk_part::expand_sdk_part(&input) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.to_compile_error().into(),
  }
}

#[proc_macro_derive(SdkPartRef, attributes(sdk))]
pub fn sdk_part_ref(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  match sdk_part_ref::expand_sdk_part_ref(&input) {
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

#[proc_macro_derive(SdkXmlNamespace, attributes(sdk))]
pub fn sdk_xml_namespace(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  match sdk_xml_namespace::expand_sdk_xml_namespace(&input) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.to_compile_error().into(),
  }
}

#[derive(Clone, Copy)]
enum PartChildKind {
  Repeated,
  RequiredRepeated,
  Required,
  Optional,
}

#[derive(Clone)]
enum PartRelationshipTypeSource {
  Explicit(String),
  TypeConst,
}

fn relationship_match_condition_tokens(
  relationship_type: &PartRelationshipTypeSource,
  relationship_expr: proc_macro2::TokenStream,
  type_relationship_const: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  match relationship_type {
    PartRelationshipTypeSource::Explicit(value) => {
      quote! {
        crate::common::relationship_type_matches_bytes(
          #relationship_expr.relationship_type_bytes(),
          #value.as_bytes(),
        )
      }
    }
    PartRelationshipTypeSource::TypeConst => {
      quote! {
        crate::common::relationship_type_matches_bytes(
          #relationship_expr.relationship_type_bytes(),
          #type_relationship_const.as_bytes(),
        )
      }
    }
  }
}

#[derive(Clone)]
struct SdkAttrField {
  ident: Ident,
  name: String,
  simple_type: Option<String>,
  ty: Type,
  optional: bool,
  list: bool,
  match_local_name: bool,
  empty_as_none: bool,
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
struct SdkEmptyChildField {
  ident: Ident,
  qname: String,
  optional: bool,
  repeated: bool,
}

#[derive(Clone)]
struct SdkTextChildField {
  ident: Ident,
  qname: String,
  simple_type: Option<String>,
  ty: Type,
  optional: bool,
  repeated: bool,
  list: bool,
}

#[derive(Clone)]
struct SdkAnyChildField {
  ident: Ident,
  qname: String,
  optional: bool,
  repeated: bool,
}

#[derive(Clone)]
struct SdkTypeChoiceField {
  ident: Ident,
  ty: Type,
  optional: bool,
  repeated: bool,
  accepts_text: Option<bool>,
  accepts_any: Option<bool>,
  specific_qnames: Vec<String>,
  items: Vec<SdkTypeChoiceItem>,
}

#[derive(Clone)]
struct SdkAnyField {
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
  list: bool,
}

#[derive(Clone)]
enum SdkTypeFieldKind {
  Attr {
    name: String,
    simple_type: Option<String>,
    list: bool,
    match_local_name: bool,
    empty_as_none: bool,
  },
  Child {
    qname: String,
  },
  EmptyChild {
    qname: String,
  },
  TextChild {
    qname: String,
    simple_type: Option<String>,
    list: bool,
  },
  AnyChild {
    qname: String,
  },
  Choice,
  Any,
  Text {
    list: bool,
  },
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

fn parse_number_range_bound(value: Expr) -> syn::Result<String> {
  match value {
    Expr::Lit(ExprLit {
      lit: Lit::Str(value),
      ..
    }) => Ok(value.value()),
    Expr::Lit(ExprLit {
      lit: Lit::Int(value),
      ..
    }) => Ok(value.base10_digits().to_string()),
    Expr::Lit(ExprLit {
      lit: Lit::Float(value),
      ..
    }) => Ok(value.base10_digits().to_string()),
    Expr::Unary(ExprUnary {
      op: UnOp::Neg(_),
      expr,
      ..
    }) => match *expr {
      Expr::Lit(ExprLit {
        lit: Lit::Int(value),
        ..
      }) => Ok(format!("-{}", value.base10_digits())),
      Expr::Lit(ExprLit {
        lit: Lit::Float(value),
        ..
      }) => Ok(format!("-{}", value.base10_digits())),
      other => Err(syn::Error::new_spanned(
        other,
        "sdk number_range bound expects a string or numeric literal",
      )),
    },
    other => Err(syn::Error::new_spanned(
      other,
      "sdk number_range bound expects a string or numeric literal",
    )),
  }
}

fn parse_number_range(value: Expr) -> syn::Result<(Option<String>, Option<String>, bool, bool)> {
  match value {
    Expr::Range(ExprRange {
      start, limits, end, ..
    }) => {
      let min = start
        .map(|start| parse_number_range_bound(*start))
        .transpose()?;
      let max = end.map(|end| parse_number_range_bound(*end)).transpose()?;
      let max_inclusive = matches!(limits, RangeLimits::Closed(_));
      Ok((min, max, true, max_inclusive))
    }
    other => Err(syn::Error::new_spanned(
      other,
      "sdk number_range range expects a Rust range expression",
    )),
  }
}

#[derive(Clone)]
struct ParsedSdkTypeFieldAttrs {
  kind: Option<SdkTypeFieldKind>,
  choice_accepts_text: Option<bool>,
  choice_accepts_any: Option<bool>,
  choice_qnames: Vec<String>,
  choice_items: Vec<SdkTypeChoiceItem>,
  validators: Vec<SdkFieldValidator>,
}

#[derive(Clone)]
enum SdkTypeChoiceItem {
  Child {
    variant: Ident,
    ty: Option<Type>,
    boxed: bool,
    qname: String,
  },
  EmptyChild {
    variant: Ident,
    qname: String,
  },
  TextChild {
    variant: Ident,
    ty: Option<Type>,
    simple_type: Option<String>,
    is_enum: bool,
    qname: String,
  },
  AnyChild {
    variant: Ident,
    qname: String,
  },
  Sequence {
    variant: Ident,
    children: Vec<SdkTypeChoiceSequenceChild>,
  },
  Any {
    variant: Ident,
    qnames: Vec<String>,
  },
  Text {
    variant: Ident,
  },
}

#[derive(Clone)]
struct SdkTypeChoiceSequenceChild {
  kind: SdkTypeChoiceSequenceChildKind,
  option_field: Option<Ident>,
  ty: Option<Type>,
  simple_type: Option<String>,
  qname: String,
}

#[derive(Clone)]
enum SdkTypeChoiceSequenceChildKind {
  Child,
  EmptyChild,
  TextChild,
  AnyChild,
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

#[derive(Clone)]
struct QNameInfo {
  tag_prefix: String,
  local_name: String,
}

fn build_conditional_chain(
  branches: &[(proc_macro2::TokenStream, proc_macro2::TokenStream)],
  fallback: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  if branches.is_empty() {
    return fallback;
  }

  let mut chain = proc_macro2::TokenStream::new();

  for (index, (condition, body)) in branches.iter().enumerate() {
    if index == 0 {
      chain.extend(quote! {
        if #condition {
          #body
        }
      });
    } else {
      chain.extend(quote! {
        else if #condition {
          #body
        }
      });
    }
  }

  chain.extend(quote! {
    else {
      #fallback
    }
  });

  chain
}

fn is_relationship_model_field(ident: &Ident) -> bool {
  ident == "fallback_parts" || ident == "relationship_order" || ident == "modeled_relationships"
}

fn parse_part_child_relationship_type_attr(
  attrs: &[Attribute],
) -> syn::Result<Option<PartRelationshipTypeSource>> {
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
            let value = nested.value()?;
            let value: LitStr = value.parse()?;
            relationship_type = Some(PartRelationshipTypeSource::Explicit(value.value()));
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

fn parse_part_child_kind_attr(attrs: &[Attribute]) -> syn::Result<Option<PartChildKind>> {
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
        let mut kind = None;
        meta.parse_nested_meta(|nested| {
          if nested.path.is_ident("relationship_type") {
            let _value: LitStr = nested.value()?.parse()?;
            Ok(())
          } else if nested.path.is_ident("kind") {
            let value: LitStr = nested.value()?.parse()?;
            kind = Some(match value.value().as_str() {
              "optional" => PartChildKind::Optional,
              "required" => PartChildKind::Required,
              "repeated" => PartChildKind::Repeated,
              "required_repeated" => PartChildKind::RequiredRepeated,
              _ => return Err(nested.error("unsupported sdk part_child kind")),
            });
            Ok(())
          } else {
            Err(nested.error("unsupported sdk part_child attribute"))
          }
        })?;
        return Ok(kind);
      }
    }
  }

  Ok(None)
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum PrefixWriteMode {
  Normal,
  NoPrefixDual,
  NoPrefixOnly,
}

impl PrefixWriteMode {
  fn writes_no_prefix(self) -> bool {
    matches!(self, Self::NoPrefixDual | Self::NoPrefixOnly)
  }

  fn writes_no_prefix_only(self) -> bool {
    self == Self::NoPrefixOnly
  }
}

fn parse_sdk_prefix_write_mode(attrs: &[Attribute]) -> syn::Result<PrefixWriteMode> {
  let mut mode = PrefixWriteMode::Normal;
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in metas {
      if let Meta::Path(path) = meta {
        if path.is_ident("no_prefix") || path.is_ident("default_ns") {
          if mode == PrefixWriteMode::NoPrefixOnly {
            return Err(syn::Error::new_spanned(
              path,
              "no_prefix_only cannot be combined with no_prefix/default_ns",
            ));
          }
          mode = PrefixWriteMode::NoPrefixDual;
        } else if path.is_ident("no_prefix_only") {
          if mode == PrefixWriteMode::NoPrefixDual {
            return Err(syn::Error::new_spanned(
              path,
              "no_prefix_only cannot be combined with no_prefix/default_ns",
            ));
          }
          mode = PrefixWriteMode::NoPrefixOnly;
        }
      }
    }
  }
  Ok(mode)
}

fn parse_sdk_xml_header(attrs: &[Attribute]) -> syn::Result<bool> {
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in metas {
      if let Meta::Path(path) = meta
        && path.is_ident("xml_header")
      {
        return Ok(true);
      }
    }
  }
  Ok(false)
}

enum ExtraXmlnsArg {
  Ident(Ident),
  Lit(LitStr),
}

impl Parse for ExtraXmlnsArg {
  fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
    if input.peek(LitStr) {
      return input.parse().map(Self::Lit);
    }
    input.parse().map(Self::Ident)
  }
}

impl ExtraXmlnsArg {
  fn value(self) -> String {
    match self {
      Self::Ident(ident) => ident.to_string(),
      Self::Lit(lit) => lit.value(),
    }
  }
}

fn parse_sdk_extra_xmlns(attrs: &[Attribute]) -> syn::Result<Vec<String>> {
  let mut prefixes = Vec::new();
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in metas {
      let Meta::List(meta) = meta else {
        continue;
      };
      if !meta.path.is_ident("extra_xmlns") {
        continue;
      }
      let args = meta.parse_args_with(Punctuated::<ExtraXmlnsArg, Token![,]>::parse_terminated)?;
      for arg in args {
        let prefix = arg.value();
        if !prefixes.contains(&prefix) {
          prefixes.push(prefix);
        }
      }
    }
  }
  Ok(prefixes)
}

fn parse_sdk_canonical_namespace_prefixes(attrs: &[Attribute]) -> syn::Result<Vec<String>> {
  let mut prefixes = Vec::new();
  for attr in attrs {
    if !attr.path().is_ident("sdk") {
      continue;
    }
    let metas =
      attr.parse_args_with(syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in metas {
      let Meta::List(meta) = meta else {
        continue;
      };
      if !meta.path.is_ident("canonical_namespace_prefix") {
        continue;
      }
      let args = meta.parse_args_with(Punctuated::<ExtraXmlnsArg, Token![,]>::parse_terminated)?;
      for arg in args {
        let prefix = arg.value();
        if !prefixes.contains(&prefix) {
          prefixes.push(prefix);
        }
      }
    }
  }
  Ok(prefixes)
}

fn is_sdk_version_marker_path(path: &syn::Path) -> bool {
  path.is_ident("office2010")
    || path.is_ident("office2013")
    || path.is_ident("office2016")
    || path.is_ident("office2019")
    || path.is_ident("office2021")
    || path.is_ident("microsoft365")
}

fn parse_sdk_type_field_attrs(attrs: &[Attribute]) -> syn::Result<ParsedSdkTypeFieldAttrs> {
  let mut attr_name = None;
  let mut attr_list = false;
  let mut attr_match_local_name = false;
  let mut attr_empty_as_none = false;
  let mut attr_simple_type = None;
  let mut kind = None;
  let mut choice_accepts_text = None;
  let mut choice_accepts_any = None;
  let mut choice_qnames = Vec::new();
  let mut choice_items = Vec::new();
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
            } else if nested.path.is_ident("simple_type") {
              let value: LitStr = nested.value()?.parse()?;
              attr_simple_type = Some(value.value());
              Ok(())
            } else if nested.path.is_ident("list") {
              attr_list = true;
              Ok(())
            } else if nested.path.is_ident("match_local_name") {
              attr_match_local_name = true;
              Ok(())
            } else if nested.path.is_ident("empty_as_none") {
              attr_empty_as_none = true;
              Ok(())
            } else if is_sdk_version_marker_path(&nested.path) {
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
            } else if nested.path.is_ident("no_prefix") || is_sdk_version_marker_path(&nested.path)
            {
              Ok(())
            } else {
              Err(nested.error("unsupported sdk child attribute"))
            }
          })?;
          let qname =
            qname.ok_or_else(|| syn::Error::new_spanned(&meta.path, "sdk child requires qname"))?;
          kind = Some(SdkTypeFieldKind::Child { qname });
        }
        Meta::Path(path) if path.is_ident("child") => {
          return Err(syn::Error::new_spanned(path, "sdk child requires qname"));
        }
        Meta::List(meta) if meta.path.is_ident("empty_child") => {
          let mut qname = None;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("qname") {
              let value: LitStr = nested.value()?.parse()?;
              qname = Some(value.value());
              Ok(())
            } else if nested.path.is_ident("no_prefix") || is_sdk_version_marker_path(&nested.path)
            {
              Ok(())
            } else {
              Err(nested.error("unsupported sdk empty_child attribute"))
            }
          })?;
          kind = Some(SdkTypeFieldKind::EmptyChild {
            qname: qname.unwrap_or_default(),
          });
        }
        Meta::List(meta) if meta.path.is_ident("text_child") => {
          let mut qname = None;
          let mut simple_type = None;
          let mut list = false;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("qname") {
              let value: LitStr = nested.value()?.parse()?;
              qname = Some(value.value());
              Ok(())
            } else if nested.path.is_ident("simple_type") {
              let value: LitStr = nested.value()?.parse()?;
              simple_type = Some(value.value());
              Ok(())
            } else if nested.path.is_ident("list") {
              list = true;
              Ok(())
            } else if nested.path.is_ident("no_prefix") || is_sdk_version_marker_path(&nested.path)
            {
              Ok(())
            } else {
              Err(nested.error("unsupported sdk text_child attribute"))
            }
          })?;
          kind = Some(SdkTypeFieldKind::TextChild {
            qname: qname.unwrap_or_default(),
            simple_type,
            list,
          });
        }
        Meta::List(meta) if meta.path.is_ident("any_child") => {
          let mut qname = None;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("qname") {
              let value: LitStr = nested.value()?.parse()?;
              qname = Some(value.value());
              Ok(())
            } else if nested.path.is_ident("no_prefix") || is_sdk_version_marker_path(&nested.path)
            {
              Ok(())
            } else {
              Err(nested.error("unsupported sdk any_child attribute"))
            }
          })?;
          kind = Some(SdkTypeFieldKind::AnyChild {
            qname: qname.unwrap_or_default(),
          });
        }
        Meta::Path(path) if path.is_ident("text") => {
          kind = Some(SdkTypeFieldKind::Text { list: false });
        }
        Meta::List(meta) if meta.path.is_ident("text") => {
          let mut list = false;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("list") {
              list = true;
              Ok(())
            } else {
              Err(nested.error("unsupported sdk text attribute"))
            }
          })?;
          kind = Some(SdkTypeFieldKind::Text { list });
        }
        Meta::List(meta) if meta.path.is_ident("choice") => {
          let mut accepts_text = false;
          let mut accepts_any = false;
          meta.parse_nested_meta(|nested| {
            if nested.path.is_ident("text") {
              accepts_text = true;
              if nested.input.is_empty() {
                choice_items.push(SdkTypeChoiceItem::Text {
                  variant: Ident::new("Text", Span::call_site()),
                });
              } else {
                let mut variant = None;
                nested.parse_nested_meta(|text| {
                  if text.path.is_ident("variant") {
                    variant = Some(text.value()?.parse()?);
                    Ok(())
                  } else {
                    Err(text.error("unsupported sdk choice text attribute"))
                  }
                })?;
                let variant = variant.ok_or_else(|| {
                  syn::Error::new_spanned(&nested.path, "sdk choice text requires variant")
                })?;
                choice_items.push(SdkTypeChoiceItem::Text { variant });
              }
              Ok(())
            } else if nested.path.is_ident("any") {
              if nested.input.is_empty() {
                accepts_any = true;
                choice_items.push(SdkTypeChoiceItem::Any {
                  variant: Ident::new("XmlAny", Span::call_site()),
                  qnames: Vec::new(),
                });
              } else {
                let mut variant = None;
                let mut qnames = Vec::new();
                nested.parse_nested_meta(|any| {
                  if any.path.is_ident("variant") {
                    variant = Some(any.value()?.parse()?);
                    Ok(())
                  } else if any.path.is_ident("qnames") {
                    let values: ExprArray = any.value()?.parse()?;
                    for value in values.elems {
                      let Expr::Lit(ExprLit {
                        lit: Lit::Str(value),
                        ..
                      }) = value
                      else {
                        return Err(any.error("sdk choice any qnames must be string literals"));
                      };
                      qnames.push(value.value());
                    }
                    Ok(())
                  } else {
                    Err(any.error("unsupported sdk choice any attribute"))
                  }
                })?;
                let variant = variant.unwrap_or_else(|| Ident::new("XmlAny", Span::call_site()));
                accepts_any = true;
                choice_qnames.extend(qnames.iter().cloned());
                choice_items.push(SdkTypeChoiceItem::Any { variant, qnames });
              }
              Ok(())
            } else if nested.path.is_ident("qname") {
              let value: LitStr = nested.value()?.parse()?;
              choice_qnames.push(value.value());
              Ok(())
            } else if nested.path.is_ident("child")
              || nested.path.is_ident("empty_child")
              || nested.path.is_ident("text_child")
              || nested.path.is_ident("any_child")
            {
              let mut variant = None;
              let mut ty = None;
              let mut boxed = false;
              let mut simple_type = None;
              let mut is_enum = false;
              let mut qname = None;
              nested.parse_nested_meta(|choice_child| {
                if choice_child.path.is_ident("qname") {
                  let value: LitStr = choice_child.value()?.parse()?;
                  qname = Some(value.value());
                  Ok(())
                } else if choice_child.path.is_ident("no_prefix") {
                  Ok(())
                } else if choice_child.path.is_ident("variant") {
                  variant = Some(choice_child.value()?.parse()?);
                  Ok(())
                } else if choice_child.path.is_ident("ty") {
                  let value: LitStr = choice_child.value()?.parse()?;
                  ty = Some(parse_str(&value.value())?);
                  Ok(())
                } else if choice_child.path.is_ident("boxed") {
                  boxed = true;
                  Ok(())
                } else if choice_child.path.is_ident("simple_type") {
                  let value: LitStr = choice_child.value()?.parse()?;
                  simple_type = Some(value.value());
                  Ok(())
                } else if choice_child.path.is_ident("enum") {
                  is_enum = true;
                  Ok(())
                } else if is_sdk_version_marker_path(&choice_child.path) {
                  Ok(())
                } else {
                  Err(choice_child.error("unsupported sdk choice child attribute"))
                }
              })?;
              let qname = qname.unwrap_or_default();
              choice_qnames.push(qname.clone());
              let variant = variant.ok_or_else(|| {
                syn::Error::new_spanned(&nested.path, "sdk choice child requires variant")
              })?;
              if nested.path.is_ident("child") {
                choice_items.push(SdkTypeChoiceItem::Child {
                  variant,
                  ty,
                  boxed,
                  qname,
                });
              } else if nested.path.is_ident("empty_child") {
                choice_items.push(SdkTypeChoiceItem::EmptyChild { variant, qname });
              } else if nested.path.is_ident("text_child") {
                choice_items.push(SdkTypeChoiceItem::TextChild {
                  variant,
                  ty,
                  simple_type,
                  is_enum,
                  qname,
                });
              } else {
                choice_items.push(SdkTypeChoiceItem::AnyChild { variant, qname });
              }
              Ok(())
            } else if nested.path.is_ident("sequence") {
              let mut variant = None;
              let mut children = Vec::new();
              nested.parse_nested_meta(|sequence| {
                if sequence.path.is_ident("variant") {
                  variant = Some(sequence.value()?.parse()?);
                  Ok(())
                } else if sequence.path.is_ident("ty") {
                  let value: LitStr = sequence.value()?.parse()?;
                  let _: Type = parse_str(&value.value())?;
                  Ok(())
                } else if sequence.path.is_ident("child")
                  || sequence.path.is_ident("empty_child")
                  || sequence.path.is_ident("text_child")
                  || sequence.path.is_ident("any_child")
                {
                  let kind = if sequence.path.is_ident("child") {
                    SdkTypeChoiceSequenceChildKind::Child
                  } else if sequence.path.is_ident("empty_child") {
                    SdkTypeChoiceSequenceChildKind::EmptyChild
                  } else if sequence.path.is_ident("text_child") {
                    SdkTypeChoiceSequenceChildKind::TextChild
                  } else {
                    SdkTypeChoiceSequenceChildKind::AnyChild
                  };
                  let mut option_field = None;
                  let mut ty = None;
                  let mut simple_type = None;
                  let mut qname = None;
                  sequence.parse_nested_meta(|sequence_child| {
                    if sequence_child.path.is_ident("qname") {
                      let value: LitStr = sequence_child.value()?.parse()?;
                      qname = Some(value.value());
                      Ok(())
                    } else if sequence_child.path.is_ident("no_prefix") {
                      Ok(())
                    } else if sequence_child.path.is_ident("option_field") {
                      option_field = Some(sequence_child.value()?.parse()?);
                      Ok(())
                    } else if sequence_child.path.is_ident("ty") {
                      let value: LitStr = sequence_child.value()?.parse()?;
                      ty = Some(parse_str(&value.value())?);
                      Ok(())
                    } else if sequence_child.path.is_ident("simple_type") {
                      let value: LitStr = sequence_child.value()?.parse()?;
                      simple_type = Some(value.value());
                      Ok(())
                    } else if sequence_child.path.is_ident("enum")
                      || is_sdk_version_marker_path(&sequence_child.path)
                    {
                      Ok(())
                    } else {
                      Err(sequence_child.error("unsupported sdk choice sequence child attribute"))
                    }
                  })?;
                  let qname = qname.unwrap_or_default();
                  choice_qnames.push(qname.clone());
                  children.push(SdkTypeChoiceSequenceChild {
                    kind,
                    option_field,
                    ty,
                    simple_type,
                    qname,
                  });
                  Ok(())
                } else if is_sdk_version_marker_path(&sequence.path) {
                  Ok(())
                } else {
                  Err(sequence.error("unsupported sdk choice sequence attribute"))
                }
              })?;
              let variant = variant.ok_or_else(|| {
                syn::Error::new_spanned(&nested.path, "sdk choice sequence requires variant")
              })?;
              choice_items.push(SdkTypeChoiceItem::Sequence { variant, children });
              Ok(())
            } else if is_sdk_version_marker_path(&nested.path) {
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
        Meta::Path(path) if is_sdk_version_marker_path(&path) => {}
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
            if nested.path.is_ident("range") {
              let (range_min, range_max, range_min_inclusive, range_max_inclusive) =
                parse_number_range(nested.value()?.parse()?)?;
              min = range_min;
              max = range_max;
              min_inclusive = range_min_inclusive;
              max_inclusive = range_max_inclusive;
              Ok(())
            } else if nested.path.is_ident("min") {
              min = Some(parse_number_range_bound(nested.value()?.parse()?)?);
              Ok(())
            } else if nested.path.is_ident("max") {
              max = Some(parse_number_range_bound(nested.value()?.parse()?)?);
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
      simple_type: attr_simple_type,
      list: attr_list,
      match_local_name: attr_match_local_name,
      empty_as_none: attr_empty_as_none,
    });
  }

  Ok(ParsedSdkTypeFieldAttrs {
    kind,
    choice_accepts_text,
    choice_accepts_any,
    choice_qnames,
    choice_items,
    validators,
  })
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

fn qname_write_prefix(prefix: &str, no_prefix: bool) -> &str {
  if no_prefix { "" } else { prefix }
}

fn write_tag_literal_tokens(
  qname: &str,
  open: &str,
  close: &str,
  no_prefix: bool,
) -> proc_macro2::TokenStream {
  let QNameInfo {
    tag_prefix,
    local_name,
  } = parse_qname_info(qname);
  let write_prefix = qname_write_prefix(&tag_prefix, no_prefix);
  let tag = if write_prefix.is_empty() {
    format!("{open}{local_name}{close}")
  } else {
    format!("{open}{write_prefix}:{local_name}{close}")
  };
  let tag_lit = LitByteStr::new(tag.as_bytes(), Span::call_site());
  quote! { writer.write_all(#tag_lit)?; }
}

fn write_start_tag_open_tokens(qname: &str, no_prefix: bool) -> proc_macro2::TokenStream {
  write_tag_literal_tokens(qname, "<", "", no_prefix)
}

fn write_start_tag_tokens(qname: &str, no_prefix: bool) -> proc_macro2::TokenStream {
  write_tag_literal_tokens(qname, "<", ">", no_prefix)
}

fn write_empty_tag_tokens(qname: &str, no_prefix: bool) -> proc_macro2::TokenStream {
  write_tag_literal_tokens(qname, "<", " />", no_prefix)
}

fn write_end_tag_tokens(qname: &str, no_prefix: bool) -> proc_macro2::TokenStream {
  write_tag_literal_tokens(qname, "</", ">", no_prefix)
}

fn is_xmlns_field(ident: &Ident) -> bool {
  ident == "xmlns"
}

fn is_option_type(ty: &Type) -> bool {
  matches!(ty, Type::Path(TypePath { path, .. }) if path.segments.last().is_some_and(|segment| segment.ident == "Option"))
}

fn is_vec_type(ty: &Type) -> bool {
  matches!(ty, Type::Path(TypePath { path, .. }) if path.segments.last().is_some_and(|segment| segment.ident == "Vec"))
}

fn is_box_u8_slice_type(ty: &Type) -> bool {
  box_inner_type(ty).as_ref().is_some_and(
    |inner_ty| matches!(inner_ty, Type::Slice(slice) if is_terminal_type(&slice.elem, "u8")),
  )
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

fn unwrap_option_type(ty: &Type) -> Type {
  if let Type::Path(TypePath { path, .. }) = ty
    && let Some(segment) = path.segments.last()
    && segment.ident == "Option"
    && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
    && let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first()
  {
    return inner_ty.clone();
  }
  ty.clone()
}

fn vec_inner_type(ty: &Type) -> Option<Type> {
  if let Type::Path(TypePath { path, .. }) = ty
    && let Some(segment) = path.segments.last()
    && segment.ident == "Vec"
    && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
    && let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first()
  {
    return Some(inner_ty.clone());
  }
  None
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

fn is_string_like_type(ty: &Type) -> bool {
  effective_type_name(ty, None)
    .as_deref()
    .is_some_and(is_string_like_type_name)
}

fn type_terminal_name(ty: &Type) -> Option<String> {
  match ty {
    Type::Path(TypePath { path, .. }) => path
      .segments
      .last()
      .map(|segment| segment.ident.to_string()),
    _ => None,
  }
}

fn effective_type_name<'a>(ty: &'a Type, simple_type: Option<&'a str>) -> Option<String> {
  effective_type_name_with_qname(ty, simple_type, "")
}

fn effective_type_name_with_qname(
  ty: &Type,
  simple_type: Option<&str>,
  qname: &str,
) -> Option<String> {
  let type_name = type_terminal_name(ty);
  simple_type_mapping::resolve(type_name.as_deref(), simple_type, qname, false)
    .map(|shape| shape.kind.canonical_type_name().to_string())
    .or_else(|| simple_type.map(str::to_string))
    .or(type_name)
}

fn mapped_simple_type_name(
  ty: Option<&Type>,
  simple_type: Option<&str>,
  qname: &str,
  list: bool,
) -> Option<&'static str> {
  let type_name = ty.and_then(type_terminal_name);
  simple_type_mapping::resolve(type_name.as_deref(), simple_type, qname, list)
    .map(|shape| shape.kind.canonical_type_name())
}

fn is_string_like_type_name(name: &str) -> bool {
  matches!(
    name,
    "str"
      | "String"
      | "StringValue"
      | "DateTimeValue"
      | "DecimalValue"
      | "HexBinaryValue"
      | "Base64BinaryValue"
  )
}

fn is_string_like_effective_type(ty: &Type, simple_type: Option<&str>) -> bool {
  effective_type_name(ty, simple_type)
    .as_deref()
    .is_some_and(is_string_like_type_name)
}

fn is_sdk_enum_type_name(name: &str) -> bool {
  matches!(
    name,
    "__SdkEnum" | "BooleanValue" | "OnOffValue" | "TrueFalseBlankValue" | "TrueFalseValue"
  ) || name.ends_with("Values")
}

fn is_sdk_enum_effective_type(ty: &Type, simple_type: Option<&str>) -> bool {
  effective_type_name(ty, simple_type)
    .as_deref()
    .is_some_and(is_sdk_enum_type_name)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SimpleUnionTypeKind {
  TwipsMeasure,
  SignedTwipsMeasure,
  DecimalNumberOrPercent,
  MeasurementOrPercent,
}

fn simple_union_type_kind(ty: &Type) -> Option<SimpleUnionTypeKind> {
  effective_type_name(ty, None).and_then(|ident| simple_union_type_kind_name(ident.as_str()))
}

fn simple_union_type_kind_name(name: &str) -> Option<SimpleUnionTypeKind> {
  Some(match name {
    "TwipsMeasureValue" => SimpleUnionTypeKind::TwipsMeasure,
    "SignedTwipsMeasureValue" => SimpleUnionTypeKind::SignedTwipsMeasure,
    "DecimalNumberOrPercentValue" => SimpleUnionTypeKind::DecimalNumberOrPercent,
    "MeasurementOrPercentValue" => SimpleUnionTypeKind::MeasurementOrPercent,
    _ => return None,
  })
}

fn simple_union_effective_type_kind(
  ty: &Type,
  simple_type: Option<&str>,
) -> Option<SimpleUnionTypeKind> {
  effective_type_name(ty, simple_type).and_then(|name| simple_union_type_kind_name(name.as_str()))
}

fn parse_simple_union_value_tokens(
  kind: SimpleUnionTypeKind,
  value_expr: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  match kind {
    SimpleUnionTypeKind::TwipsMeasure => {
      quote! { crate::common::parse_twips_measure_value(#value_expr)? }
    }
    SimpleUnionTypeKind::SignedTwipsMeasure => {
      quote! { crate::common::parse_signed_twips_measure_value(#value_expr)? }
    }
    SimpleUnionTypeKind::DecimalNumberOrPercent => {
      quote! { crate::common::parse_decimal_number_or_percent_value(#value_expr)? }
    }
    SimpleUnionTypeKind::MeasurementOrPercent => {
      quote! { crate::common::parse_measurement_or_percent_value(#value_expr)? }
    }
  }
}

fn write_simple_union_value_tokens(
  kind: SimpleUnionTypeKind,
  value_expr: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  match kind {
    SimpleUnionTypeKind::TwipsMeasure => {
      quote! { crate::common::write_twips_measure_value(writer, #value_expr)?; }
    }
    SimpleUnionTypeKind::SignedTwipsMeasure => {
      quote! { crate::common::write_signed_twips_measure_value(writer, #value_expr)?; }
    }
    SimpleUnionTypeKind::DecimalNumberOrPercent => {
      quote! { crate::common::write_decimal_number_or_percent_value(writer, #value_expr)?; }
    }
    SimpleUnionTypeKind::MeasurementOrPercent => {
      quote! { crate::common::write_measurement_or_percent_value(writer, #value_expr)?; }
    }
  }
}

fn is_from_bytes_attr_type_name(name: &str) -> bool {
  matches!(
    name,
    "UniversalMeasureValue"
      | "HpsMeasureValue"
      | "SignedHpsMeasureValue"
      | "DrawingmlPercentageValue"
      | "PositiveDrawingmlPercentageValue"
      | "FixedPercentageValue"
      | "PositiveFixedPercentageValue"
      | "TextFontScalePercentOrPercentStringValue"
      | "TextSpacingPercentOrPercentStringValue"
      | "CoordinateValue"
      | "PositiveCoordinateValue"
      | "Coordinate32Value"
      | "PositiveCoordinate32Value"
      | "TextPointValue"
      | "TextBulletSizeValue"
  )
}

fn write_from_bytes_value_tokens(
  value_ty: &Type,
  simple_type: Option<&str>,
  value_expr: proc_macro2::TokenStream,
) -> Option<proc_macro2::TokenStream> {
  let write_fn = match effective_type_name(value_ty, simple_type).as_deref() {
    Some("UniversalMeasureValue") => quote! { crate::common::write_universal_measure_value },
    Some("HpsMeasureValue") => quote! { crate::common::write_hps_measure_value },
    Some("SignedHpsMeasureValue") => quote! { crate::common::write_signed_hps_measure_value },
    Some("CoordinateValue") => quote! { crate::common::write_coordinate_value },
    Some("Coordinate32Value") => quote! { crate::common::write_coordinate32_value },
    Some("DrawingmlPercentageValue") => {
      quote! { crate::common::write_drawingml_percentage_value }
    }
    Some("TextPointValue") => quote! { crate::common::write_text_point_value },
    Some("TextBulletSizeValue") => quote! { crate::common::write_text_bullet_size_value },
    _ => return None,
  };
  Some(quote! { #write_fn(writer, #value_expr)?; })
}

fn is_from_bytes_attr_effective_type(ty: &Type, simple_type: Option<&str>) -> bool {
  effective_type_name(ty, simple_type)
    .as_deref()
    .is_some_and(is_from_bytes_attr_type_name)
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

fn integer_type_kind_name(name: &str) -> Option<IntegerTypeKind> {
  Some(match name {
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

fn integer_effective_type_kind(ty: &Type, simple_type: Option<&str>) -> Option<IntegerTypeKind> {
  effective_type_name(ty, simple_type).and_then(|name| integer_type_kind_name(name.as_str()))
}

fn is_xml_schema_float_type(ty: &Type) -> bool {
  effective_type_name(ty, None)
    .as_deref()
    .is_some_and(is_xml_schema_float_type_name)
}

fn is_xml_schema_float_type_name(name: &str) -> bool {
  matches!(name, "DoubleValue" | "SingleValue" | "f64" | "f32")
}

fn write_xml_schema_float_tokens(
  value_expr: proc_macro2::TokenStream,
  float_ty: &Type,
) -> proc_macro2::TokenStream {
  write_xml_schema_float_tokens_with_qname(value_expr, float_ty, "")
}

fn write_xml_schema_float_tokens_with_qname(
  value_expr: proc_macro2::TokenStream,
  float_ty: &Type,
  qname: &str,
) -> proc_macro2::TokenStream {
  write_xml_schema_float_effective_tokens(value_expr, float_ty, None, qname)
}

fn write_xml_schema_float_effective_tokens(
  value_expr: proc_macro2::TokenStream,
  float_ty: &Type,
  simple_type: Option<&str>,
  qname: &str,
) -> proc_macro2::TokenStream {
  let single_precision = matches!(
    qname.split_once('/').map(|(type_name, _)| type_name),
    Some("xsd:float")
  ) || effective_type_name(float_ty, simple_type)
    .is_some_and(|name| matches!(name.as_str(), "SingleValue" | "f32"));
  let (positive_infinity, negative_infinity) = if single_precision {
    (quote! { f32::INFINITY }, quote! { f32::NEG_INFINITY })
  } else {
    (quote! { f64::INFINITY }, quote! { f64::NEG_INFINITY })
  };
  let write_finite = if single_precision {
    quote! { crate::common::write_f32_value(writer, __ooxmlsdk_float_value)?; }
  } else {
    quote! { crate::common::write_f64_value(writer, __ooxmlsdk_float_value)?; }
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
        #write_finite
      }
    }
  }
}

fn write_integer_value_tokens_by_kind(
  kind: IntegerTypeKind,
  value_expr: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  match kind {
    IntegerTypeKind::U8 => quote! {
      crate::common::write_u8_value(writer, *#value_expr)?;
    },
    IntegerTypeKind::I8 => quote! {
      crate::common::write_i8_value(writer, *#value_expr)?;
    },
    IntegerTypeKind::U16 => quote! {
      crate::common::write_u16_value(writer, *#value_expr)?;
    },
    IntegerTypeKind::I16 => quote! {
      crate::common::write_i16_value(writer, *#value_expr)?;
    },
    IntegerTypeKind::U32 => quote! {
      crate::common::write_u32_value(writer, *#value_expr)?;
    },
    IntegerTypeKind::I32 => quote! {
      crate::common::write_i32_value(writer, *#value_expr)?;
    },
    IntegerTypeKind::U64 => quote! {
      crate::common::write_u64_value(writer, *#value_expr)?;
    },
    IntegerTypeKind::I64 => quote! {
      crate::common::write_i64_value(writer, *#value_expr)?;
    },
  }
}

fn parse_integer_attr_tokens_by_kind(
  kind: IntegerTypeKind,
  attr_expr: proc_macro2::TokenStream,
  decoder_expr: proc_macro2::TokenStream,
  owner_expr: proc_macro2::TokenStream,
  field_expr: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  match kind {
    IntegerTypeKind::U8 => quote! {
      crate::common::parse_u8_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    IntegerTypeKind::I8 => quote! {
      crate::common::parse_i8_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    IntegerTypeKind::U16 => quote! {
      crate::common::parse_u16_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    IntegerTypeKind::I16 => quote! {
      crate::common::parse_i16_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    IntegerTypeKind::U32 => quote! {
      crate::common::parse_u32_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    IntegerTypeKind::I32 => quote! {
      crate::common::parse_i32_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    IntegerTypeKind::U64 => quote! {
      crate::common::parse_u64_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
    IntegerTypeKind::I64 => quote! {
      crate::common::parse_i64_attr(#attr_expr, #decoder_expr, #owner_expr, #field_expr)?
    },
  }
}

fn parse_integer_bytes_fn_by_kind(kind: IntegerTypeKind) -> proc_macro2::TokenStream {
  match kind {
    IntegerTypeKind::U8 => quote! { crate::common::parse_u8_bytes },
    IntegerTypeKind::I8 => quote! { crate::common::parse_i8_bytes },
    IntegerTypeKind::U16 => quote! { crate::common::parse_u16_bytes },
    IntegerTypeKind::I16 => quote! { crate::common::parse_i16_bytes },
    IntegerTypeKind::U32 => quote! { crate::common::parse_u32_bytes },
    IntegerTypeKind::I32 => quote! { crate::common::parse_i32_bytes },
    IntegerTypeKind::U64 => quote! { crate::common::parse_u64_bytes },
    IntegerTypeKind::I64 => quote! { crate::common::parse_i64_bytes },
  }
}

fn is_hex_binary_type(ty: &Type) -> bool {
  type_terminal_name(ty).as_deref() == Some("HexBinaryValue")
}

fn is_hex_binary_effective_type(ty: &Type, simple_type: Option<&str>) -> bool {
  effective_type_name(ty, simple_type).as_deref() == Some("HexBinaryValue")
}

fn is_base64_binary_effective_type(ty: &Type, simple_type: Option<&str>) -> bool {
  effective_type_name(ty, simple_type).as_deref() == Some("Base64BinaryValue")
}

fn is_decimal_value_effective_type(ty: &Type, simple_type: Option<&str>) -> bool {
  effective_type_name(ty, simple_type).as_deref() == Some("DecimalValue")
}

fn is_datetime_value_effective_type(ty: &Type, simple_type: Option<&str>) -> bool {
  effective_type_name(ty, simple_type).as_deref() == Some("DateTimeValue")
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
      "SdkPart" => sdk_part::expand_sdk_part(&input).expect("SdkPart expansion"),
      "SdkPartRef" => sdk_part_ref::expand_sdk_part_ref(&input).expect("SdkPartRef expansion"),
      "SdkPackage" => sdk_package::expand_sdk_package(&input).expect("SdkPackage expansion"),
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
      format!("pub struct {target}<"),
      format!("pub struct {target}("),
      format!("struct {target} {{"),
      format!("struct {target}<"),
      format!("struct {target}("),
      format!("pub enum {target} {{"),
      format!("pub enum {target}<"),
      format!("enum {target} {{"),
      format!("enum {target}<"),
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
            .split(['{', '(', '<'])
            .next()
            .map(|s| s.split_whitespace().next().unwrap_or_default().to_string());
        } else if let Some(rest) = trimmed.strip_prefix("struct ") {
          item_name = rest
            .split(['{', '(', '<'])
            .next()
            .map(|s| s.split_whitespace().next().unwrap_or_default().to_string());
        } else if let Some(rest) = trimmed.strip_prefix("pub enum ") {
          item_name = rest
            .split(['{', '<'])
            .next()
            .map(|s| s.split_whitespace().next().unwrap_or_default().to_string());
        } else if let Some(rest) = trimmed.strip_prefix("enum ") {
          item_name = rest
            .split(['{', '<'])
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
