use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct CompatibilityConfig {
  pub rules: Vec<CompatibilityRule>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct CompatibilityRule {
  pub schema: String,
  #[serde(rename = "Type")]
  pub type_name: String,
  pub field: String,
  pub action: CompatibilityAction,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct CompatibilityBitmaskAttribute {
  #[serde(rename = "QName")]
  pub q_name: String,
  pub bit: u32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum CompatibilityAction {
  #[default]
  None,
  FallbackToRawXml,
  TextChoice,
  ExtraChild,
  AlternateContentChoice,
  MapAttributeValue {
    #[serde(rename = "Mappings")]
    mappings: Vec<CompatibilityValueMapping>,
  },
  StrictBitmaskAttributes {
    #[serde(rename = "Radix")]
    radix: u32,
    #[serde(rename = "Width")]
    width: usize,
    #[serde(rename = "Attributes")]
    attributes: Vec<CompatibilityBitmaskAttribute>,
  },
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct CompatibilityValueMapping {
  #[serde(rename = "From")]
  pub from: String,
  #[serde(rename = "To")]
  pub to: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct TypedSchema {
  pub name: String,
  pub class_name: String,
  pub part_class_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Namespace {
  pub prefix: String,
  pub uri: String,
  pub version: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Part {
  pub name: String,
  pub module_name: String,
  pub base: String,
  pub content_type: String,
  pub relationship_type: String,
  pub target: String,
  pub root: String,
  pub root_element: String,
  pub extension: String,
  pub version: String,
  pub features: Vec<String>,
  pub content_kind: PartContentKind,
  pub paths: PartPaths,
  pub root_type: String,
  pub root_class_name: String,
  pub schema_module: String,
  pub children: Vec<PartChild>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum PartContentKind {
  #[default]
  None,
  Xml,
  Text,
  Binary,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PartPaths {
  pub general: String,
  pub word: String,
  pub excel: String,
  pub power_point: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PartChild {
  pub min_occurs_is_non_zero: bool,
  pub max_occurs_great_than_one: bool,
  pub api_name: String,
  pub name: String,
  pub version: String,
  pub features: Vec<String>,
  pub has_fixed_content: bool,
  pub is_data_part_reference: bool,
  pub is_special_embedded_part: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Schema {
  pub target_namespace: String,
  pub prefix: String,
  pub typed_namespace: String,
  pub module_name: String,
  pub types: Vec<SchemaType>,
  pub enums: Vec<SchemaEnum>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaType {
  pub name: String,
  pub class_name: String,
  pub summary: String,
  pub version: String,
  pub part: String,
  pub base_class: String,
  pub kind: SchemaTypeKind,
  pub composite_kind: SchemaTypeCompositeKind,
  pub xml_header: SchemaTypeXmlHeader,
  pub is_abstract: bool,
  pub has_xmlns_fields: bool,
  pub has_mc_ignorable_field: bool,
  pub text_value_type: String,
  pub api_kind: SchemaTypeApiKind,
  pub attributes: Vec<SchemaTypeAttribute>,
  pub children: Vec<SchemaTypeChild>,
  #[serde(skip)]
  pub particle: SchemaTypeParticle,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum SchemaTypeXmlHeader {
  #[default]
  None,
  Plain,
  Standalone,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum SchemaTypeKind {
  #[default]
  Struct,
  LeafText,
  Leaf,
  Composite,
  Derived,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum SchemaTypeCompositeKind {
  #[default]
  None,
  OneSequence,
  OneChoice,
  OneAll,
  SdkSequence,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum SchemaTypeApiKind {
  #[default]
  Struct,
  LeafTextWrapper,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeAttribute {
  pub q_name: String,
  pub property_name: String,
  pub r#type: String,
  pub property_comments: String,
  pub version: String,
  pub required: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeChild {
  pub name: String,
  pub property_name: String,
  pub property_comments: String,
  pub kind: SchemaTypeChildKind,
  pub optional: bool,
  pub repeated: bool,
  pub initial_version: String,
  #[serde(default)]
  pub children: Vec<SchemaTypeChild>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum SchemaTypeChildKind {
  #[default]
  Child,
  TextChild,
  Choice,
  Sequence,
  Any,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeParticle {
  pub kind: String,
  pub name: String,
  pub occurs: Vec<SchemaTypeParticleOccur>,
  pub items: Vec<SchemaTypeParticle>,
  pub initial_version: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeParticleOccur {
  pub max: Option<u64>,
  pub min: Option<u64>,
  pub version: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaEnum {
  pub name: String,
  pub r#type: String,
  pub facets: Vec<SchemaEnumFacet>,
  pub version: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaEnumFacet {
  pub name: String,
  pub value: String,
  pub version: String,
}
