use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::Path;

use heck::ToUpperCamelCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{GenericArgument, PathArguments, Type, TypePath, parse_str};

use super::codegen_ir::{FieldWireDecl, MemberDecl, TypeKind, TypeRefDecl, VariantWireDecl};
use super::{LoadedSchema, write_generated_module};
use crate::Result;
use crate::utils::escape_upper_camel_case;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ValueKind {
  String,
  Base64Binary,
  DateTime,
  Decimal,
  HexBinary,
  Enum,
  U8,
  I8,
  U16,
  I16,
  U32,
  I32,
  U64,
  I64,
  F32,
  F64,
  UniversalMeasure,
  TwipsMeasure,
  SignedTwipsMeasure,
  HpsMeasure,
  SignedHpsMeasure,
  Coordinate,
  Coordinate32,
  DecimalNumberOrPercent,
  DrawingmlPercentage,
  TextPoint,
  TextBulletSize,
  MeasurementOrPercent,
}

impl ValueKind {
  fn ident(self) -> Ident {
    Ident::new(
      match self {
        Self::String => "String",
        Self::Base64Binary => "Base64Binary",
        Self::DateTime => "DateTime",
        Self::Decimal => "Decimal",
        Self::HexBinary => "HexBinary",
        Self::Enum => "Enum",
        Self::U8 => "U8",
        Self::I8 => "I8",
        Self::U16 => "U16",
        Self::I16 => "I16",
        Self::U32 => "U32",
        Self::I32 => "I32",
        Self::U64 => "U64",
        Self::I64 => "I64",
        Self::F32 => "F32",
        Self::F64 => "F64",
        Self::UniversalMeasure => "UniversalMeasure",
        Self::TwipsMeasure => "TwipsMeasure",
        Self::SignedTwipsMeasure => "SignedTwipsMeasure",
        Self::HpsMeasure => "HpsMeasure",
        Self::SignedHpsMeasure => "SignedHpsMeasure",
        Self::Coordinate => "Coordinate",
        Self::Coordinate32 => "Coordinate32",
        Self::DecimalNumberOrPercent => "DecimalNumberOrPercent",
        Self::DrawingmlPercentage => "DrawingmlPercentage",
        Self::TextPoint => "TextPoint",
        Self::TextBulletSize => "TextBulletSize",
        Self::MeasurementOrPercent => "MeasurementOrPercent",
      },
      Span::call_site(),
    )
  }
}

struct MappingBuilder<'a> {
  schemas: HashMap<&'a str, &'a LoadedSchema>,
  enums: HashSet<(String, String)>,
  aliases: HashMap<(String, String), &'a TypeRefDecl>,
  type_kinds: BTreeMap<String, BTreeSet<ValueKind>>,
  type_qname_kinds: BTreeMap<(String, String), BTreeSet<ValueKind>>,
  qname_kinds: BTreeMap<String, BTreeSet<ValueKind>>,
}

impl<'a> MappingBuilder<'a> {
  fn new(loaded_schemas: &'a [LoadedSchema]) -> Self {
    let schemas = loaded_schemas
      .iter()
      .map(|schema| (schema.ir.module_name.as_str(), schema))
      .collect();
    let mut enums = HashSet::new();
    let mut aliases = HashMap::new();
    for schema in loaded_schemas {
      let module = schema.ir.module_name.as_str();
      for value in &schema.ir.enums {
        enums.insert((module.to_string(), generated_type_name(&value.rust_name)));
      }
      for ty in &schema.ir.types {
        if ty.kind == TypeKind::LeafTextAlias
          && let Some(xml_content) = &ty.xml_content
        {
          aliases.insert(
            (module.to_string(), generated_type_name(&ty.rust_name)),
            xml_content,
          );
        }
      }
    }
    Self {
      schemas,
      enums,
      aliases,
      type_kinds: BTreeMap::new(),
      type_qname_kinds: BTreeMap::new(),
      qname_kinds: BTreeMap::new(),
    }
  }

  fn collect(mut self) -> Result<Self> {
    for (name, kind) in builtin_value_kinds() {
      self.insert_type(name, *kind);
    }

    let enum_names = self
      .enums
      .iter()
      .map(|(_, name)| name.clone())
      .collect::<Vec<_>>();
    for name in enum_names {
      self.insert_type(&name, ValueKind::Enum);
    }

    let alias_keys = self.aliases.keys().cloned().collect::<Vec<_>>();
    for (module, name) in alias_keys {
      let mut visiting = HashSet::new();
      let kind = self
        .resolve_named(&module, &name, &mut visiting)?
        .ok_or_else(|| format!("unresolved simple-type alias {module}::{name}"))?;
      self.insert_type(&name, kind);
    }

    let modules = self.schemas.keys().copied().collect::<Vec<_>>();
    for module in modules {
      let schema = self.schemas[module];
      for ty in &schema.ir.types {
        if let (Some(qname), Some(xml_content)) = (&ty.xml_qname, &ty.xml_content) {
          self.collect_usage(module, xml_content, qname, false)?;
        }
        for member in &ty.members {
          match member {
            MemberDecl::Field(field) => match &field.wire {
              FieldWireDecl::Attribute { qname, .. } | FieldWireDecl::TextChild { qname } => {
                self.collect_usage(module, &field.type_ref, qname, false)?;
              }
              _ => {}
            },
            MemberDecl::Variant(variant) => {
              if let VariantWireDecl::TextChild { qnames } = &variant.wire {
                for qname in qnames {
                  self.collect_usage(module, &variant.payload, qname, true)?;
                }
              }
            }
          }
        }
      }
    }
    for ((type_name, qname), kinds) in &self.type_qname_kinds {
      if kinds.len() > 1 {
        return Err(
          format!("ambiguous simple-type mapping for ({type_name}, {qname}): {kinds:?}").into(),
        );
      }
    }
    for (qname, kinds) in &self.qname_kinds {
      if kinds.len() > 1 {
        return Err(
          format!("ambiguous choice text simple-type mapping for {qname}: {kinds:?}").into(),
        );
      }
    }
    Ok(self)
  }

  fn collect_usage(
    &mut self,
    module: &str,
    type_ref: &TypeRefDecl,
    qname: &str,
    qname_fallback: bool,
  ) -> Result<()> {
    let mut visiting = HashSet::new();
    let kind = self
      .resolve_type_ref(module, type_ref, &mut visiting)?
      .ok_or_else(|| {
        format!(
          "unresolved XML value type {} in {module} for {qname}",
          type_ref.rust_type
        )
      })?;
    let Some(type_name) = innermost_type_path(&type_ref.rust_type)?.last().cloned() else {
      return Ok(());
    };
    self.insert_type(&type_name, kind);
    let qname = qname.split('/').nth(1).unwrap_or(qname);
    self
      .type_qname_kinds
      .entry((type_name, qname.to_string()))
      .or_default()
      .insert(kind);
    if qname_fallback {
      self
        .qname_kinds
        .entry(qname.to_string())
        .or_default()
        .insert(kind);
    }
    Ok(())
  }

  fn insert_type(&mut self, name: &str, kind: ValueKind) {
    self
      .type_kinds
      .entry(name.to_string())
      .or_default()
      .insert(kind);
  }

  fn resolve_type_ref(
    &self,
    current_module: &str,
    type_ref: &TypeRefDecl,
    visiting: &mut HashSet<(String, String)>,
  ) -> Result<Option<ValueKind>> {
    let path = innermost_type_path(&type_ref.rust_type)?;
    let Some(name) = path.last() else {
      return Ok(None);
    };
    if let Some(kind) = builtin_value_kind(name) {
      return Ok(Some(kind));
    }

    let module_path = type_ref
      .module_path
      .as_deref()
      .map(str::to_string)
      .or_else(|| path_module_path(&path));
    let module = schema_module_from_path(module_path.as_deref()).unwrap_or(current_module);
    self.resolve_named(module, name, visiting)
  }

  fn resolve_named(
    &self,
    module: &str,
    name: &str,
    visiting: &mut HashSet<(String, String)>,
  ) -> Result<Option<ValueKind>> {
    if let Some(kind) = builtin_value_kind(name) {
      return Ok(Some(kind));
    }
    let key = (module.to_string(), generated_type_name(name));
    if self.enums.contains(&key) {
      return Ok(Some(ValueKind::Enum));
    }
    let Some(type_ref) = self.aliases.get(&key) else {
      return Ok(None);
    };
    let key = (module.to_string(), name.to_string());
    if !visiting.insert(key.clone()) {
      return Err(format!("cyclic simple-type alias {module}::{name}").into());
    }
    let kind = self.resolve_type_ref(module, type_ref, visiting)?;
    visiting.remove(&key);
    Ok(kind)
  }
}

fn generated_type_name(name: &str) -> String {
  escape_upper_camel_case(name.to_upper_camel_case())
}

fn innermost_type_path(value: &str) -> Result<Vec<String>> {
  fn visit(ty: &Type) -> Option<Vec<String>> {
    let Type::Path(TypePath { path, .. }) = ty else {
      return None;
    };
    let last = path.segments.last()?;
    if matches!(
      last.ident.to_string().as_str(),
      "Vec" | "Option" | "Box" | "ListValue"
    ) && let PathArguments::AngleBracketed(arguments) = &last.arguments
      && let Some(GenericArgument::Type(inner)) = arguments.args.first()
    {
      return visit(inner);
    }
    Some(
      path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect(),
    )
  }

  let ty: Type = parse_str(value)?;
  Ok(visit(&ty).unwrap_or_default())
}

fn path_module_path(path: &[String]) -> Option<String> {
  (path.len() > 1).then(|| path[..path.len() - 1].join("::"))
}

fn schema_module_from_path(path: Option<&str>) -> Option<&str> {
  path?.strip_prefix("crate::schemas::")
}

fn builtin_value_kinds() -> &'static [(&'static str, ValueKind)] {
  &[
    ("str", ValueKind::String),
    ("String", ValueKind::String),
    ("StringValue", ValueKind::String),
    ("Base64BinaryValue", ValueKind::Base64Binary),
    ("DateTimeValue", ValueKind::DateTime),
    ("DecimalValue", ValueKind::Decimal),
    ("HexBinaryValue", ValueKind::HexBinary),
    ("BooleanValue", ValueKind::Enum),
    ("OnOffValue", ValueKind::Enum),
    ("TrueFalseBlankValue", ValueKind::Enum),
    ("TrueFalseValue", ValueKind::Enum),
    ("ByteValue", ValueKind::U8),
    ("u8", ValueKind::U8),
    ("SByteValue", ValueKind::I8),
    ("i8", ValueKind::I8),
    ("UInt16Value", ValueKind::U16),
    ("u16", ValueKind::U16),
    ("Int16Value", ValueKind::I16),
    ("i16", ValueKind::I16),
    ("UInt32Value", ValueKind::U32),
    ("u32", ValueKind::U32),
    ("Int32Value", ValueKind::I32),
    ("i32", ValueKind::I32),
    ("UInt64Value", ValueKind::U64),
    ("u64", ValueKind::U64),
    ("Int64Value", ValueKind::I64),
    ("IntegerValue", ValueKind::I64),
    ("i64", ValueKind::I64),
    ("SingleValue", ValueKind::F32),
    ("f32", ValueKind::F32),
    ("DoubleValue", ValueKind::F64),
    ("f64", ValueKind::F64),
    ("EmuValue", ValueKind::I64),
    ("TwipsValue", ValueKind::I64),
    ("UnsignedDecimalValue", ValueKind::U64),
    ("DrawingmlPercentValue", ValueKind::I32),
    ("WordPercentValue", ValueKind::I64),
    ("DrawingmlAngleValue", ValueKind::I32),
    ("VmlFixedValue", ValueKind::I32),
    ("TextFontSizeValue", ValueKind::I32),
    ("TextSpacingPointValue", ValueKind::I32),
    ("UniversalMeasureValue", ValueKind::UniversalMeasure),
    ("PositiveUniversalMeasureValue", ValueKind::UniversalMeasure),
    ("TwipsMeasureValue", ValueKind::TwipsMeasure),
    ("SignedTwipsMeasureValue", ValueKind::SignedTwipsMeasure),
    ("HpsMeasureValue", ValueKind::HpsMeasure),
    ("SignedHpsMeasureValue", ValueKind::SignedHpsMeasure),
    ("CoordinateValue", ValueKind::Coordinate),
    ("PositiveCoordinateValue", ValueKind::Coordinate),
    ("Coordinate32Value", ValueKind::Coordinate32),
    ("PositiveCoordinate32Value", ValueKind::Coordinate32),
    (
      "DecimalNumberOrPercentValue",
      ValueKind::DecimalNumberOrPercent,
    ),
    ("DrawingmlPercentageValue", ValueKind::DrawingmlPercentage),
    (
      "PositiveDrawingmlPercentageValue",
      ValueKind::DrawingmlPercentage,
    ),
    ("FixedPercentageValue", ValueKind::DrawingmlPercentage),
    (
      "PositiveFixedPercentageValue",
      ValueKind::DrawingmlPercentage,
    ),
    (
      "TextFontScalePercentOrPercentStringValue",
      ValueKind::DrawingmlPercentage,
    ),
    (
      "TextSpacingPercentOrPercentStringValue",
      ValueKind::DrawingmlPercentage,
    ),
    ("TextPointValue", ValueKind::TextPoint),
    ("TextBulletSizeValue", ValueKind::TextBulletSize),
    ("TextBulletSizePercentValue", ValueKind::I32),
    ("TextBulletSizeDecimalValue", ValueKind::I32),
    ("MeasurementOrPercentValue", ValueKind::MeasurementOrPercent),
  ]
}

fn builtin_value_kind(name: &str) -> Option<ValueKind> {
  builtin_value_kinds()
    .iter()
    .find_map(|(candidate, kind)| (*candidate == name).then_some(*kind))
}

fn unique_entries<K: Clone + Ord>(
  entries: &BTreeMap<K, BTreeSet<ValueKind>>,
) -> impl Iterator<Item = (K, ValueKind)> + '_ {
  entries
    .iter()
    .filter(|(_, kinds)| kinds.len() == 1)
    .map(|(key, kinds)| (key.clone(), *kinds.first().expect("one value kind")))
}

fn kind_tokens(kind: ValueKind) -> TokenStream {
  let ident = kind.ident();
  quote! { ValueKind::#ident }
}

pub(super) fn write_simple_type_mapping(
  loaded_schemas: &[LoadedSchema],
  out_dir: &Path,
) -> Result<()> {
  let mapping = MappingBuilder::new(loaded_schemas).collect()?;

  let type_arms = unique_entries(&mapping.type_kinds).map(|(name, kind)| {
    let kind = kind_tokens(kind);
    quote! { #name => Some(#kind), }
  });
  let ambiguous_types = mapping
    .type_kinds
    .iter()
    .filter_map(|(name, kinds)| (kinds.len() > 1).then_some(name.as_str()))
    .collect::<HashSet<_>>();
  let type_qname_arms = unique_entries(&mapping.type_qname_kinds)
    .filter(|((name, _), _)| ambiguous_types.contains(name.as_str()))
    .map(|((name, qname), kind)| {
      let kind = kind_tokens(kind);
      quote! { (#name, #qname) => Some(#kind), }
    });
  let qname_arms = unique_entries(&mapping.qname_kinds).map(|(qname, kind)| {
    let kind = kind_tokens(kind);
    quote! { #qname => Some(#kind), }
  });

  let tokens = quote! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(crate) enum ValueKind {
      String,
      Base64Binary,
      DateTime,
      Decimal,
      HexBinary,
      Enum,
      U8,
      I8,
      U16,
      I16,
      U32,
      I32,
      U64,
      I64,
      F32,
      F64,
      UniversalMeasure,
      TwipsMeasure,
      SignedTwipsMeasure,
      HpsMeasure,
      SignedHpsMeasure,
      Coordinate,
      Coordinate32,
      DecimalNumberOrPercent,
      DrawingmlPercentage,
      TextPoint,
      TextBulletSize,
      MeasurementOrPercent,
    }

    impl ValueKind {
      pub(crate) const fn canonical_type_name(self) -> &'static str {
        match self {
          Self::String => "StringValue",
          Self::Base64Binary => "Base64BinaryValue",
          Self::DateTime => "DateTimeValue",
          Self::Decimal => "DecimalValue",
          Self::HexBinary => "HexBinaryValue",
          Self::Enum => "__SdkEnum",
          Self::U8 => "ByteValue",
          Self::I8 => "SByteValue",
          Self::U16 => "UInt16Value",
          Self::I16 => "Int16Value",
          Self::U32 => "UInt32Value",
          Self::I32 => "Int32Value",
          Self::U64 => "UInt64Value",
          Self::I64 => "Int64Value",
          Self::F32 => "SingleValue",
          Self::F64 => "DoubleValue",
          Self::UniversalMeasure => "UniversalMeasureValue",
          Self::TwipsMeasure => "TwipsMeasureValue",
          Self::SignedTwipsMeasure => "SignedTwipsMeasureValue",
          Self::HpsMeasure => "HpsMeasureValue",
          Self::SignedHpsMeasure => "SignedHpsMeasureValue",
          Self::Coordinate => "CoordinateValue",
          Self::Coordinate32 => "Coordinate32Value",
          Self::DecimalNumberOrPercent => "DecimalNumberOrPercentValue",
          Self::DrawingmlPercentage => "DrawingmlPercentageValue",
          Self::TextPoint => "TextPointValue",
          Self::TextBulletSize => "TextBulletSizeValue",
          Self::MeasurementOrPercent => "MeasurementOrPercentValue",
        }
      }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(crate) struct ValueShape {
      pub(crate) kind: ValueKind,
      pub(crate) list: bool,
    }

    fn kind_by_type_name(name: &str) -> Option<ValueKind> {
      match name {
        #( #type_arms )*
        _ => None,
      }
    }

    fn kind_by_type_and_qname(name: &str, qname: &str) -> Option<ValueKind> {
      match (name, qname) {
        #( #type_qname_arms )*
        _ => None,
      }
    }

    fn kind_by_qname(qname: &str) -> Option<ValueKind> {
      match qname {
        #( #qname_arms )*
        _ => None,
      }
    }

    pub(crate) fn resolve(
      type_name: Option<&str>,
      qname: &str,
      list: bool,
    ) -> Option<ValueShape> {
      let kind = type_name
        .and_then(|name| {
          kind_by_type_and_qname(name, qname).or_else(|| kind_by_type_name(name))
        })
        .or_else(|| kind_by_qname(qname))?;
      Some(ValueShape {
        kind,
        list,
      })
    }
  };

  write_generated_module(&out_dir.join("simple_type_mapping.rs"), tokens)
}
