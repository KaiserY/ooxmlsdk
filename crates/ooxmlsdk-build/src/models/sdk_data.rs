use serde::{Deserialize, Serialize};

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
  pub has_fixed_content: bool,
  pub is_data_part_reference: bool,
  pub is_special_embedded_part: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PackageSchema {
  pub module_name: String,
  pub root: String,
  pub xmlns_uri: String,
  pub xml_header: PackageXmlHeader,
  pub types: Vec<PackageType>,
  pub enums: Vec<PackageEnum>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum PackageXmlHeader {
  #[default]
  None,
  Plain,
  Standalone,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PackageType {
  pub name: String,
  pub tag: String,
  pub prefix: String,
  pub kind: PackageTypeKind,
  pub has_xmlns_fields: bool,
  pub attributes: Vec<PackageAttribute>,
  pub text_children: Vec<PackageTextChild>,
  pub child_fields: Vec<PackageChildField>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum PackageTypeKind {
  #[default]
  Composite,
  Leaf,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PackageAttribute {
  pub field: String,
  pub q_name: String,
  pub r#type: String,
  pub required: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PackageTextChild {
  pub field: String,
  pub q_name: String,
  pub fixed_attributes: Vec<PackageFixedAttribute>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PackageFixedAttribute {
  pub q_name: String,
  pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PackageChildField {
  pub field: String,
  pub kind: PackageChildFieldKind,
  pub item_type: String,
  pub enum_name: String,
  pub variants: Vec<PackageChildVariant>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum PackageChildFieldKind {
  #[default]
  Vec,
  ChoiceVec,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PackageChildVariant {
  pub name: String,
  pub r#type: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PackageEnum {
  pub name: String,
  pub variants: Vec<PackageEnumVariant>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PackageEnumVariant {
  pub name: String,
  pub value: String,
  pub is_default: bool,
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
  pub is_abstract: bool,
  pub api_kind: SchemaTypeApiKind,
  pub attributes: Vec<SchemaTypeAttribute>,
  pub children: Vec<SchemaTypeChild>,
  pub particle: SchemaTypeParticle,
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
  pub validators: Vec<SchemaTypeAttributeValidator>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeAttributeValidator {
  pub name: String,
  pub is_list: bool,
  pub r#type: String,
  pub union_id: u64,
  pub is_initial_version: bool,
  pub arguments: Vec<SchemaTypeAttributeValidatorArgument>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeAttributeValidatorArgument {
  pub name: String,
  pub r#type: String,
  pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeChild {
  pub name: String,
  pub property_name: String,
  pub property_comments: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeParticle {
  pub kind: String,
  pub name: String,
  pub occurs: Vec<SchemaTypeParticleOccur>,
  pub items: Vec<SchemaTypeParticle>,
  pub initial_version: String,
  pub require_filter: bool,
  pub namespace: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeParticleOccur {
  pub max: u64,
  pub min: u64,
  pub include_version: bool,
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
