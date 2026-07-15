use std::collections::{HashMap, HashSet};

use heck::ToUpperCamelCase;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaModuleDecl {
  pub module_name: String,
  pub target_namespace: String,
  pub prefix: String,
  pub typed_namespace: String,
  pub enums: Vec<EnumDecl>,
  pub types: Vec<TypeDecl>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct EnumDecl {
  pub rust_name: String,
  pub docs: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub version: Option<String>,
  pub value_type: EnumValueType,
  pub variants: Vec<EnumVariantDecl>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum EnumValueType {
  #[default]
  StringLike,
  NumericLike,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct EnumVariantDecl {
  pub rust_name: String,
  pub xml_value: String,
  pub aliases: Vec<String>,
  pub version: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct TypeDecl {
  pub rust_name: String,
  #[serde(rename = "XmlQName", skip_serializing_if = "Option::is_none")]
  pub xml_qname: Option<String>,
  pub docs: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub version: Option<String>,
  pub is_abstract: bool,
  pub kind: TypeKind,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub element_kind: Option<ElementKind>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_model: Option<ContentModelDecl>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub base_rust_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub base_module_path: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub xml_content: Option<TypeRefDecl>,
  #[serde(skip_serializing_if = "is_zero_usize")]
  pub estimated_size: usize,
  pub support: SystemSupportDecl,
  pub members: Vec<MemberDecl>,
}

fn is_zero_usize(value: &usize) -> bool {
  *value == 0
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum TypeKind {
  #[default]
  ElementStruct,
  ChoiceEnum,
  HelperStruct,
  LeafTextAlias,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ElementKind {
  #[default]
  LeafText,
  Leaf,
  Composite,
  Derived,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ContentModelDecl {
  #[default]
  None,
  OneAllDirectChildren,
  DirectChildrenOnly,
  ChoiceOnly,
  OneChoiceSingle,
  MixedChoiceChildren,
  SequenceAnyOnly,
  SequenceSingleChoice,
  SequenceDirectChildren,
  OneSequenceFlatten,
  OneSequenceStructured,
  GenericChildrenFallback,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct SystemSupportDecl {
  pub have_xmlns_fields: bool,
  pub has_xml_header: bool,
  pub have_mc_ignorable: bool,
  pub have_mc_preserve_attributes: bool,
  pub have_mc_preserve_elements: bool,
  pub have_mc_process_content: bool,
  pub have_mc_must_understand: bool,
  pub have_xml_other_children: bool,
  pub compact_xml_other_children: bool,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub extra_xmlns: Vec<String>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub canonical_namespace_prefixes: Vec<String>,
  #[serde(skip_serializing_if = "HashMap::is_empty")]
  pub alternate_content_children: HashMap<String, Vec<String>>,
}

impl SystemSupportDecl {
  pub fn has_mce_attributes(&self) -> bool {
    self.have_mc_ignorable
      || self.have_mc_preserve_attributes
      || self.have_mc_preserve_elements
      || self.have_mc_process_content
      || self.have_mc_must_understand
  }

  pub fn has_extra_support_fields(&self) -> bool {
    self.have_xmlns_fields
      || self.has_mce_attributes()
      || self.have_xml_other_children
      || !self.extra_xmlns.is_empty()
      || !self.canonical_namespace_prefixes.is_empty()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum MemberDecl {
  Field(FieldDecl),
  Variant(VariantDecl),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct FieldDecl {
  pub rust_name: String,
  pub docs: String,
  pub version: String,
  pub wire: FieldWireDecl,
  pub cardinality: Cardinality,
  pub type_ref: TypeRefDecl,
  pub validators: Vec<ValidatorDecl>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase", rename_all_fields = "PascalCase")]
pub enum FieldWireDecl {
  Attribute {
    #[serde(rename = "QName")]
    qname: String,
    bit: Option<u32>,
    #[serde(default)]
    list: bool,
    #[serde(default)]
    match_local_name: bool,
    #[serde(default)]
    empty_as_none: bool,
  },
  Child {
    #[serde(rename = "QName")]
    qname: String,
  },
  TextChild {
    #[serde(rename = "QName")]
    qname: String,
  },
  #[default]
  Choice,
  Any,
  Text,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct VariantDecl {
  pub rust_name: String,
  pub docs: String,
  pub version: String,
  pub wire: VariantWireDecl,
  pub payload: TypeRefDecl,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase", rename_all_fields = "PascalCase")]
pub enum VariantWireDecl {
  Child {
    #[serde(rename = "QNames")]
    qnames: Vec<String>,
  },
  Sequence {
    #[serde(rename = "QNames")]
    qnames: Vec<String>,
  },
  TextChild {
    #[serde(rename = "QNames")]
    qnames: Vec<String>,
  },
  Any {
    #[serde(rename = "QNames")]
    qnames: Vec<String>,
  },
  #[default]
  Text,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum Cardinality {
  #[default]
  One,
  Optional,
  Many,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct TypeRefDecl {
  pub rust_type: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub module_path: Option<String>,
}

const POINTER_SIZE: usize = 8;
const BOXED_SLICE_SIZE: usize = 16;
const VEC_SIZE: usize = 24;
const STRING_SIZE: usize = 24;
const OPTION_DISCRIMINANT_SIZE: usize = 8;
const ESTIMATED_TYPE_SIZE_CAP: usize = 512;

pub fn annotate_estimated_type_sizes(module: &mut SchemaModuleDecl) {
  let type_indexes: HashMap<String, usize> = module
    .types
    .iter()
    .enumerate()
    .map(|(index, ty)| (ty.rust_name.to_upper_camel_case(), index))
    .collect();
  let mut memo = HashMap::<String, usize>::new();

  for index in 0..module.types.len() {
    let mut visiting = HashSet::new();
    let size = estimate_type_decl_size(module, &type_indexes, index, &mut memo, &mut visiting);
    module.types[index].estimated_size = size;
  }
}

fn estimate_type_decl_size(
  module: &SchemaModuleDecl,
  type_indexes: &HashMap<String, usize>,
  index: usize,
  memo: &mut HashMap<String, usize>,
  visiting: &mut HashSet<String>,
) -> usize {
  let ty = &module.types[index];
  let key = ty.rust_name.to_upper_camel_case();
  if let Some(size) = memo.get(&key) {
    return *size;
  }
  if !visiting.insert(key.clone()) {
    return POINTER_SIZE;
  }

  let mut size = estimate_support_size(&ty.support);
  if let Some(base_rust_name) = &ty.base_rust_name
    && ty.base_module_path.is_none()
  {
    size = size.saturating_add(estimate_schema_type_ref_size_by_name(
      module,
      type_indexes,
      base_rust_name,
      memo,
      visiting,
    ));
  }
  if let Some(xml_content) = &ty.xml_content {
    size = size.saturating_add(estimate_type_ref_size(
      module,
      type_indexes,
      xml_content,
      memo,
      visiting,
    ));
  }

  match ty.kind {
    TypeKind::ChoiceEnum => {
      let max_variant_size = ty
        .members
        .iter()
        .filter_map(|member| match member {
          MemberDecl::Variant(variant) => Some(estimate_type_ref_size(
            module,
            type_indexes,
            &variant.payload,
            memo,
            visiting,
          )),
          MemberDecl::Field(_) => None,
        })
        .max()
        .unwrap_or(0);
      size = size.saturating_add(max_variant_size + OPTION_DISCRIMINANT_SIZE);
    }
    TypeKind::ElementStruct | TypeKind::HelperStruct | TypeKind::LeafTextAlias => {
      for member in &ty.members {
        let MemberDecl::Field(field) = member else {
          continue;
        };
        size = size.saturating_add(estimate_field_size(
          module,
          type_indexes,
          field,
          memo,
          visiting,
        ));
      }
    }
  }

  visiting.remove(&key);
  size = size.min(ESTIMATED_TYPE_SIZE_CAP);
  memo.insert(key, size);
  size
}

fn estimate_support_size(support: &SystemSupportDecl) -> usize {
  let mut size = 0usize;
  if support.have_xmlns_fields {
    size += VEC_SIZE;
  }
  size += BOXED_SLICE_SIZE
    * [
      support.have_mc_ignorable,
      support.have_mc_preserve_attributes,
      support.have_mc_preserve_elements,
      support.have_mc_process_content,
      support.have_mc_must_understand,
    ]
    .into_iter()
    .filter(|enabled| *enabled)
    .count();
  if support.have_xml_other_children {
    size += VEC_SIZE;
  }
  if !support.extra_xmlns.is_empty() {
    size += VEC_SIZE;
  }
  if !support.canonical_namespace_prefixes.is_empty() {
    size += VEC_SIZE;
  }
  size
}

fn estimate_field_size(
  module: &SchemaModuleDecl,
  type_indexes: &HashMap<String, usize>,
  field: &FieldDecl,
  memo: &mut HashMap<String, usize>,
  visiting: &mut HashSet<String>,
) -> usize {
  match field.cardinality {
    Cardinality::Many => VEC_SIZE,
    Cardinality::Optional => {
      estimate_type_ref_size(module, type_indexes, &field.type_ref, memo, visiting)
        .saturating_add(OPTION_DISCRIMINANT_SIZE)
    }
    Cardinality::One => {
      estimate_type_ref_size(module, type_indexes, &field.type_ref, memo, visiting)
    }
  }
}

fn estimate_type_ref_size(
  module: &SchemaModuleDecl,
  type_indexes: &HashMap<String, usize>,
  type_ref: &TypeRefDecl,
  memo: &mut HashMap<String, usize>,
  visiting: &mut HashSet<String>,
) -> usize {
  if type_ref.module_path.is_some() {
    return estimate_external_type_size(type_ref);
  }
  estimate_schema_type_ref_size_by_name(module, type_indexes, &type_ref.rust_type, memo, visiting)
}

fn estimate_schema_type_ref_size_by_name(
  module: &SchemaModuleDecl,
  type_indexes: &HashMap<String, usize>,
  rust_type: &str,
  memo: &mut HashMap<String, usize>,
  visiting: &mut HashSet<String>,
) -> usize {
  let rust_name = rust_type.to_upper_camel_case();
  if let Some(index) = type_indexes.get(&rust_name) {
    return estimate_type_decl_size(module, type_indexes, *index, memo, visiting);
  }
  estimate_named_rust_type_size(&rust_name)
}

fn estimate_external_type_size(type_ref: &TypeRefDecl) -> usize {
  match type_ref.module_path.as_deref() {
    Some("crate::simple_type") => estimate_named_rust_type_size(&type_ref.rust_type),
    Some(_) => POINTER_SIZE,
    None => estimate_named_rust_type_size(&type_ref.rust_type),
  }
}

fn estimate_named_rust_type_size(rust_type: &str) -> usize {
  match rust_type {
    "bool" | "BooleanValue" | "OnOffValue" => 1,
    "i8" | "u8" | "ByteValue" | "SByteValue" => 1,
    "i16" | "u16" | "Int16Value" | "UInt16Value" => 2,
    "i32" | "u32" | "Int32Value" | "UInt32Value" | "IntegerValue" | "EnumValue" => 4,
    "i64" | "u64" | "Int64Value" | "UInt64Value" | "DoubleValue" | "DateTimeValue" => 8,
    "f32" | "SingleValue" => 4,
    "f64" => 8,
    "String" | "StringValue" | "HexBinaryValue" | "Base64BinaryValue" => STRING_SIZE,
    _ if rust_type.starts_with("Vec<") => VEC_SIZE,
    _ if rust_type.starts_with("Option<") => POINTER_SIZE + OPTION_DISCRIMINANT_SIZE,
    _ if rust_type.starts_with("Box<") => POINTER_SIZE,
    _ => POINTER_SIZE,
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct ValidatorDecl {
  pub version: String,
  pub source_id: u32,
  pub union_id: Option<u64>,
  pub kind: ValidatorKind,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum StringFormatKind {
  #[default]
  Token,
  NcName,
  QName,
  Uri,
  Id,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum NumberSignKind {
  #[default]
  NonNegative,
  Positive,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase", rename_all_fields = "PascalCase")]
pub enum ValidatorKind {
  Required,
  StringLength {
    min: Option<u32>,
    max: Option<u32>,
    exact: Option<u32>,
    type_name: Option<String>,
  },
  Pattern {
    regex: String,
  },
  StringFormat {
    kind: StringFormatKind,
  },
  NumberRange {
    min: Option<String>,
    max: Option<String>,
    min_inclusive: bool,
    max_inclusive: bool,
  },
  NumberType {
    type_name: String,
  },
  NumberSign {
    kind: NumberSignKind,
  },
  EnumRef {
    type_name: String,
    union_id: u64,
    is_list: bool,
  },
  StringSet {
    values: Vec<String>,
  },
  Unsupported {
    name: String,
  },
  #[default]
  Placeholder,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serializes_codegen_ir_with_pascal_case_keys() {
    let module = SchemaModuleDecl {
      module_name: "schemas_example".to_string(),
      target_namespace: "urn:example".to_string(),
      prefix: "ex".to_string(),
      typed_namespace: "DocumentFormat.OpenXml.Example".to_string(),
      enums: vec![EnumDecl {
        rust_name: "ExampleEnum".to_string(),
        docs: String::new(),
        version: Some("Office2007".to_string()),
        value_type: EnumValueType::StringLike,
        variants: vec![EnumVariantDecl {
          rust_name: "Foo".to_string(),
          xml_value: "foo".to_string(),
          aliases: vec!["bar".to_string()],
          version: "Office2007".to_string(),
        }],
      }],
      types: vec![TypeDecl {
        rust_name: "ExampleType".to_string(),
        xml_qname: Some("ex:CT_Example/ex:example".to_string()),
        docs: "Example docs".to_string(),
        version: Some("Office2007".to_string()),
        is_abstract: false,
        kind: TypeKind::ElementStruct,
        element_kind: Some(ElementKind::Composite),
        content_model: Some(ContentModelDecl::GenericChildrenFallback),
        base_rust_name: None,
        base_module_path: None,
        xml_content: None,
        support: SystemSupportDecl {
          have_xmlns_fields: true,
          has_xml_header: true,
          have_mc_ignorable: false,
          have_mc_preserve_attributes: false,
          have_mc_preserve_elements: false,
          have_mc_process_content: false,
          have_mc_must_understand: false,
          have_xml_other_children: true,
          compact_xml_other_children: false,
          extra_xmlns: Vec::new(),
          canonical_namespace_prefixes: Vec::new(),
          alternate_content_children: HashMap::new(),
        },
        estimated_size: 0,
        members: vec![
          MemberDecl::Field(FieldDecl {
            rust_name: "id".to_string(),
            docs: String::new(),
            version: "Office2007".to_string(),
            wire: FieldWireDecl::Attribute {
              qname: "ex:id".to_string(),
              bit: Some(1),
              list: false,
              match_local_name: false,
              empty_as_none: false,
            },
            cardinality: Cardinality::One,
            type_ref: TypeRefDecl {
              rust_type: "StringValue".to_string(),
              module_path: Some("crate::simple_type".to_string()),
            },
            validators: vec![ValidatorDecl {
              version: "Office2007".to_string(),
              source_id: 0,
              union_id: None,
              kind: ValidatorKind::Pattern {
                regex: "[A-Z]+".to_string(),
              },
            }],
          }),
          MemberDecl::Variant(VariantDecl {
            rust_name: "UnknownXml".to_string(),
            docs: String::new(),
            version: "Office2007".to_string(),
            wire: VariantWireDecl::Any { qnames: Vec::new() },
            payload: TypeRefDecl {
              rust_type: "std::boxed::Box<[u8]>".to_string(),
              module_path: None,
            },
          }),
        ],
      }],
    };

    let json = serde_json::to_value(&module).unwrap();
    assert!(json.get("ModuleName").is_some());
    assert!(json.get("TargetNamespace").is_some());
    assert!(json.get("Types").is_some());

    let ty = &json["Types"][0];
    assert!(ty.get("RustName").is_some());
    assert!(ty.get("XmlQName").is_some());
    assert!(ty.get("Support").is_some());

    let attr_field = &ty["Members"][0]["Field"];
    assert_eq!(attr_field["Wire"]["Attribute"]["QName"], "ex:id");
    assert_eq!(attr_field["TypeRef"]["RustType"], "StringValue");
    assert_eq!(
      attr_field["Validators"][0]["Kind"]["Pattern"]["Regex"],
      "[A-Z]+"
    );
  }

  #[test]
  fn round_trips_codegen_ir_json() {
    let module = SchemaModuleDecl {
      module_name: "schemas_example".to_string(),
      target_namespace: "urn:example".to_string(),
      prefix: "ex".to_string(),
      typed_namespace: "DocumentFormat.OpenXml.Example".to_string(),
      enums: vec![],
      types: vec![TypeDecl {
        rust_name: "ExampleChoice".to_string(),
        xml_qname: None,
        docs: String::new(),
        version: None,
        is_abstract: false,
        kind: TypeKind::ChoiceEnum,
        element_kind: None,
        content_model: None,
        base_rust_name: None,
        base_module_path: None,
        xml_content: None,
        support: SystemSupportDecl::default(),
        estimated_size: 0,
        members: vec![MemberDecl::Variant(VariantDecl {
          rust_name: "Text".to_string(),
          docs: String::new(),
          version: String::new(),
          wire: VariantWireDecl::TextChild {
            qnames: vec!["w:t".to_string(), "m:t".to_string()],
          },
          payload: TypeRefDecl {
            rust_type: "StringValue".to_string(),
            module_path: Some("crate::simple_type".to_string()),
          },
        })],
      }],
    };

    let json = serde_json::to_string_pretty(&module).unwrap();
    let decoded: SchemaModuleDecl = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, module);
  }
}
