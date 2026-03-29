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
  pub composite_type: String,
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
