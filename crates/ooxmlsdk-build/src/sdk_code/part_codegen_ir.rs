use serde::{Deserialize, Serialize};

use crate::sdk_data::sdk_data_model::{Part, PartChild, PartContentKind};
use heck::ToUpperCamelCase;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct PartModuleDecl {
  pub module_name: String,
  pub struct_name: String,
  pub content_type: String,
  pub relationship_type: String,
  pub path_prefix: String,
  pub is_package: bool,
  pub main_part_module_name: Option<String>,
  pub main_part_struct_name: Option<String>,
  pub main_part_accessor_name: Option<String>,
  pub root_element_type: Option<String>,
  pub root_element_accessor_name: Option<String>,
  pub version: String,
  pub features: Vec<String>,
  pub fields: Vec<PartFieldDecl>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, rename_all = "PascalCase")]
pub struct PartFieldDecl {
  pub rust_name: String,
  pub rust_type: String,
  pub version: String,
  pub features: Vec<String>,
  pub kind: PartFieldKind,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum PartFieldKind {
  #[default]
  Rid,
  ContentTypes,
  Relationships,
  RelsPath,
  InnerPath,
  RootElement,
  TextContent,
  BinaryContent,
  ExtendedParts,
  ChildPart {
    relationship_type: String,
    cardinality: PartChildCardinality,
  },
  DataPartReference {
    relationship_type: String,
    cardinality: PartChildCardinality,
    reference_kind: PartDataReferenceKind,
  },
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum PartChildCardinality {
  #[default]
  Optional,
  Required,
  Repeated,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum PartDataReferenceKind {
  Audio,
  #[default]
  Media,
  Video,
}

pub fn build_part_codegen_ir(part: &Part, all_parts: &[Part]) -> PartModuleDecl {
  let mut fields = Vec::new();

  if part.base == "OpenXmlPackage" {
    fields.push(PartFieldDecl {
      rust_name: "content_types".to_string(),
      rust_type: "crate::schemas::opc_content_types::Types".to_string(),
      kind: PartFieldKind::ContentTypes,
      ..Default::default()
    });
  } else {
    fields.push(PartFieldDecl {
      rust_name: "r_id".to_string(),
      rust_type: "String".to_string(),
      kind: PartFieldKind::Rid,
      ..Default::default()
    });
  }

  if part.base != "OpenXmlPackage" || !part.children.is_empty() {
    fields.push(PartFieldDecl {
      rust_name: "relationships".to_string(),
      rust_type: "Option<crate::schemas::opc_relationships::Relationships>".to_string(),
      kind: PartFieldKind::Relationships,
      ..Default::default()
    });
    fields.push(PartFieldDecl {
      rust_name: "rels_path".to_string(),
      rust_type: "String".to_string(),
      kind: PartFieldKind::RelsPath,
      ..Default::default()
    });
    fields.push(PartFieldDecl {
      rust_name: "extended_parts".to_string(),
      rust_type: "Vec<crate::common::extended_part::ExtendedPart>".to_string(),
      kind: PartFieldKind::ExtendedParts,
      ..Default::default()
    });
  }

  fields.push(PartFieldDecl {
    rust_name: "inner_path".to_string(),
    rust_type: "String".to_string(),
    kind: PartFieldKind::InnerPath,
    ..Default::default()
  });

  match part.content_kind {
    PartContentKind::Xml => {
      fields.push(PartFieldDecl {
        rust_name: "root_element".to_string(),
        rust_type: part_root_rust_type(part),
        kind: PartFieldKind::RootElement,
        ..Default::default()
      });
    }
    PartContentKind::Text => {
      fields.push(PartFieldDecl {
        rust_name: "part_content".to_string(),
        rust_type: "String".to_string(),
        kind: PartFieldKind::TextContent,
        ..Default::default()
      });
    }
    PartContentKind::Binary => {
      fields.push(PartFieldDecl {
        rust_name: "part_content".to_string(),
        rust_type: "Vec<u8>".to_string(),
        kind: PartFieldKind::BinaryContent,
        ..Default::default()
      });
    }
    PartContentKind::None => {}
  }

  for child in &part.children {
    fields.push(build_part_child_field_decl(child, all_parts));
  }

  PartModuleDecl {
    module_name: part.module_name.clone(),
    struct_name: part.name.clone(),
    content_type: part.content_type.clone(),
    relationship_type: part.relationship_type.clone(),
    path_prefix: part.paths.general.clone(),
    is_package: part.base == "OpenXmlPackage",
    main_part_module_name: package_main_child(part).map(|child| child.name.to_snake_case()),
    main_part_struct_name: package_main_child(part).map(|child| child.name.clone()),
    main_part_accessor_name: package_main_child(part).map(|child| child.api_name.to_snake_case()),
    root_element_type: part_root_element_type(part),
    root_element_accessor_name: part_root_element_type(part)
      .map(|_| format!("as_{}", part.name.to_snake_case())),
    version: part.version.clone(),
    features: part.features.clone(),
    fields,
  }
}

fn package_main_child(part: &Part) -> Option<&PartChild> {
  if part.base != "OpenXmlPackage" {
    return None;
  }

  part
    .children
    .iter()
    .find(|child| child.min_occurs_is_non_zero)
    .or_else(|| part.children.first())
}

fn build_part_child_field_decl(child: &PartChild, all_parts: &[Part]) -> PartFieldDecl {
  let relationship_type = child_relationship_type(child, all_parts);
  if child.is_data_part_reference {
    let (rust_type, reference_kind) = data_reference_rust_type_and_kind(child.name.as_str());
    return PartFieldDecl {
      rust_name: child.api_name.to_snake_case(),
      rust_type: rust_type.to_string(),
      version: child.version.clone(),
      features: child.features.clone(),
      kind: PartFieldKind::DataPartReference {
        relationship_type,
        cardinality: child_cardinality(child),
        reference_kind,
      },
    };
  }

  PartFieldDecl {
    rust_name: child.api_name.to_snake_case(),
    rust_type: format!(
      "crate::parts::{}::{}",
      child.name.to_snake_case(),
      child.name
    ),
    version: child.version.clone(),
    features: child.features.clone(),
    kind: PartFieldKind::ChildPart {
      relationship_type,
      cardinality: child_cardinality(child),
    },
  }
}

fn child_relationship_type(child: &PartChild, all_parts: &[Part]) -> String {
  if !child.relationship_type.is_empty() {
    return child.relationship_type.clone();
  }

  all_parts
    .iter()
    .find(|part| part.name == child.name)
    .map(|part| part.relationship_type.clone())
    .or_else(|| data_reference_relationship_type(child.name.as_str()).map(str::to_string))
    .unwrap_or_default()
}

fn child_cardinality(child: &PartChild) -> PartChildCardinality {
  if child.max_occurs_great_than_one {
    PartChildCardinality::Repeated
  } else if child.min_occurs_is_non_zero {
    PartChildCardinality::Required
  } else {
    PartChildCardinality::Optional
  }
}

fn data_reference_rust_type_and_kind(name: &str) -> (&'static str, PartDataReferenceKind) {
  match name {
    "AudioReferenceRelationship" => (
      "crate::common::data_part::AudioReferenceRelationship",
      PartDataReferenceKind::Audio,
    ),
    "MediaReferenceRelationship" => (
      "crate::common::data_part::MediaReferenceRelationship",
      PartDataReferenceKind::Media,
    ),
    "VideoReferenceRelationship" => (
      "crate::common::data_part::VideoReferenceRelationship",
      PartDataReferenceKind::Video,
    ),
    other => panic!("unsupported data part reference relationship {other}"),
  }
}

fn data_reference_relationship_type(name: &str) -> Option<&'static str> {
  match name {
    "AudioReferenceRelationship" => {
      Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio")
    }
    "MediaReferenceRelationship" => {
      Some("http://schemas.microsoft.com/office/2007/relationships/media")
    }
    "VideoReferenceRelationship" => {
      Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/video")
    }
    _ => None,
  }
}

fn part_root_rust_type(part: &Part) -> String {
  if part.name == "CoreFilePropertiesPart" {
    "crate::schemas::opc_core_properties::CoreProperties".to_string()
  } else {
    format!(
      "crate::schemas::{}::{}",
      part.schema_module,
      part.root_class_name.to_upper_camel_case()
    )
  }
}

fn part_root_element_type(part: &Part) -> Option<String> {
  matches!(part.content_kind, PartContentKind::Xml).then(|| part_root_rust_type(part))
}

trait ToSnakeCaseLocal {
  fn to_snake_case(&self) -> String;
}

impl ToSnakeCaseLocal for str {
  fn to_snake_case(&self) -> String {
    heck::ToSnakeCase::to_snake_case(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::sdk_data::sdk_data_model::{PartChild, PartPaths};

  #[test]
  fn builds_part_codegen_ir_with_explicit_child_relationship_type() {
    let child_part = Part {
      name: "ThemePart".to_string(),
      module_name: "theme_part".to_string(),
      relationship_type:
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme".to_string(),
      ..Default::default()
    };
    let part = Part {
      name: "MainDocumentPart".to_string(),
      module_name: "main_document_part".to_string(),
      relationship_type:
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"
          .to_string(),
      paths: PartPaths {
        general: "word".to_string(),
        ..Default::default()
      },
      content_kind: PartContentKind::Xml,
      root_class_name: "Document".to_string(),
      schema_module: "schemas_openxmlformats_org_wordprocessingml_2006_main".to_string(),
      children: vec![PartChild {
        api_name: "ThemePart".to_string(),
        name: "ThemePart".to_string(),
        relationship_type: child_part.relationship_type.clone(),
        ..Default::default()
      }],
      ..Default::default()
    };

    let ir = build_part_codegen_ir(&part, &[part.clone(), child_part]);

    assert_eq!(ir.struct_name, "MainDocumentPart");
    assert_eq!(ir.relationship_type, part.relationship_type);
    assert_eq!(ir.path_prefix, "word");
    assert!(matches!(ir.fields[0].kind, PartFieldKind::Rid));
    assert!(matches!(ir.fields[1].kind, PartFieldKind::Relationships));
    assert!(matches!(ir.fields[2].kind, PartFieldKind::RelsPath));
    assert!(matches!(ir.fields[3].kind, PartFieldKind::ExtendedParts));
    assert!(matches!(ir.fields[4].kind, PartFieldKind::InnerPath));
    assert!(matches!(ir.fields[5].kind, PartFieldKind::RootElement));
    assert!(matches!(
      &ir.fields[6].kind,
      PartFieldKind::ChildPart {
        relationship_type,
        cardinality: PartChildCardinality::Optional,
      } if relationship_type == "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme"
    ));
  }
}
