use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct OpenXmlPart {
  pub root: String,
  pub name: String,
  pub base: String,
  pub content_type: String,
  pub relationship_type: String,
  pub target: String,
  pub root_element: String,
  pub extension: String,
  pub paths: OpenXmlPartPaths,
  pub version: String,
  pub children: Vec<OpenXmlPartChild>,
  #[serde(skip)]
  pub module_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct OpenXmlPartPaths {
  pub general: String,
  pub word: String,
  pub excel: String,
  pub power_point: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct OpenXmlPartChild {
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
pub struct OpenXmlSchema {
  pub target_namespace: String,
  pub types: Vec<OpenXmlSchemaType>,
  pub enums: Vec<OpenXmlSchemaEnum>,
  #[serde(skip)]
  pub module_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct OpenXmlSchemaType {
  pub name: String,
  pub class_name: String,
  pub summary: String,
  pub version: String,
  pub part: String,
  pub composite_type: String,
  pub base_class: String,
  pub is_leaf_text: bool,
  pub is_leaf_element: bool,
  pub is_derived: bool,
  pub is_abstract: bool,
  pub attributes: Vec<OpenXmlSchemaTypeAttribute>,
  pub children: Vec<OpenXmlSchemaTypeChild>,
  pub particle: OpenXmlSchemaTypeParticle,
  #[serde(skip)]
  pub module_name: String,
}

impl OpenXmlSchemaType {
  pub fn is_one_sequence_flatten(&self) -> bool {
    if self.composite_type == "OneSequence" || self.particle.kind == "Sequence" {
      for p in &self.particle.items {
        if !p.kind.is_empty() || !p.items.is_empty() {
          return false;
        }
      }

      true
    } else {
      false
    }
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct OpenXmlSchemaTypeAttribute {
  pub q_name: String,
  pub property_name: String,
  pub r#type: String,
  pub property_comments: String,
  pub version: String,
  pub validators: Vec<OpenXmlSchemaTypeAttributeValidator>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct OpenXmlSchemaTypeAttributeValidator {
  pub name: String,
  pub is_list: bool,
  pub r#type: String,
  pub union_id: u64,
  pub is_initial_version: bool,
  pub arguments: Vec<OpenXmlSchemaTypeAttributeValidatorArgument>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct OpenXmlSchemaTypeAttributeValidatorArgument {
  pub name: String,
  pub r#type: String,
  pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct OpenXmlSchemaTypeChild {
  pub name: String,
  pub property_name: String,
  pub property_comments: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct OpenXmlSchemaTypeParticle {
  pub kind: String,
  pub name: String,
  pub occurs: Vec<OpenXmlSchemaTypeParticleOccur>,
  pub items: Vec<OpenXmlSchemaTypeParticle>,
  pub initial_version: String,
  pub require_filter: bool,
  pub namespace: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct OpenXmlSchemaTypeParticleOccur {
  pub max: u64,
  pub min: u64,
  pub include_version: bool,
  pub version: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct OpenXmlSchemaEnum {
  pub name: String,
  pub r#type: String,
  pub facets: Vec<OpenXmlSchemaEnumFacet>,
  pub version: String,
  #[serde(skip)]
  pub module_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct OpenXmlSchemaEnumFacet {
  pub name: String,
  pub value: String,
  pub version: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct OpenXmlNamespace {
  pub prefix: String,
  pub uri: String,
  pub version: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct TypedNamespace {
  pub prefix: String,
  pub namespace: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct TypedSchema {
  pub name: String,
  pub class_name: String,
  pub part_class_name: String,
}
