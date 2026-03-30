use std::collections::{BTreeSet, HashSet};

use crate::sdk_data::{
  context::Context,
  sdk_data_model::{Part, PartChild, PartContentKind, PartPaths},
};

pub fn gen_parts(gen_context: &Context) -> Vec<Part> {
  let type_map: std::collections::HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType> =
    gen_context
      .schemas
      .iter()
      .flat_map(|schema| schema.types.iter())
      .map(|schema_type| (schema_type.name.as_str(), schema_type))
      .collect();

  let mut parts: Vec<Part> = gen_context
    .parts
    .iter()
    .map(|part| {
      let root_type = gen_context
        .part_name_type_name_map
        .get(&part.name)
        .cloned()
        .unwrap_or_default();
      let root_class_name = type_map
        .get(root_type.as_str())
        .map(|schema_type| schema_type.class_name.clone())
        .unwrap_or_default();
      let schema_module = gen_context
        .type_name_module_name_map
        .get(root_type.as_str())
        .cloned()
        .unwrap_or_default();

      Part {
        name: part.name.clone(),
        module_name: part.module_name.clone(),
        base: part.base.clone(),
        content_type: part.content_type.clone(),
        relationship_type: part.relationship_type.clone(),
        target: part.target.clone(),
        root: part.root.clone(),
        root_element: part.root_element.clone(),
        extension: part.extension.clone(),
        version: part.version.clone(),
        features: gen_part_features(&part.version),
        content_kind: resolve_content_kind(part, !root_type.is_empty()),
        paths: PartPaths {
          general: part.paths.general.clone(),
          word: part.paths.word.clone(),
          excel: part.paths.excel.clone(),
          power_point: part.paths.power_point.clone(),
        },
        root_type,
        root_class_name,
        schema_module,
        children: part
          .children
          .iter()
          .map(|child| PartChild {
            min_occurs_is_non_zero: child.min_occurs_is_non_zero,
            max_occurs_great_than_one: child.max_occurs_great_than_one,
            api_name: child.api_name.clone(),
            name: child.name.clone(),
            has_fixed_content: child.has_fixed_content,
            is_data_part_reference: child.is_data_part_reference,
            is_special_embedded_part: child.is_special_embedded_part,
          })
          .collect(),
      }
    })
    .collect();

  parts.sort_by(|left, right| left.module_name.cmp(&right.module_name));
  validate_parts(&parts);
  parts
}

fn resolve_content_kind(
  part: &crate::sdk_data::open_xml::OpenXmlPart,
  has_root_type: bool,
) -> PartContentKind {
  if part.base == "OpenXmlPackage" {
    return PartContentKind::None;
  }

  if matches!(part.name.as_str(), "CustomXmlPart" | "XmlSignaturePart") {
    return PartContentKind::Text;
  }

  if !part.extension.is_empty()
    || matches!(
      part.name.as_str(),
      "CustomDataPart" | "InternationalMacroSheetPart"
    )
  {
    return PartContentKind::Binary;
  }

  if has_root_type || part.name == "CoreFilePropertiesPart" {
    return PartContentKind::Xml;
  }

  if !part.content_type.is_empty() || !part.target.is_empty() || !part.root_element.is_empty() {
    return PartContentKind::None;
  }

  PartContentKind::Binary
}

fn gen_part_features(version: &str) -> Vec<String> {
  let mut features = vec!["parts".to_string()];

  if version.is_empty() || version == "Office2007" {
    features.push("office2007".to_string());
  } else {
    features.push("microsoft365".to_string());
  }

  features
}

fn validate_parts(parts: &[Part]) {
  let mut part_names: BTreeSet<&str> = BTreeSet::new();

  for part in parts {
    assert!(
      part_names.insert(part.name.as_str()),
      "duplicate part definition {}",
      part.name
    );

    if part.base == "OpenXmlPackage" {
      assert!(
        part.relationship_type.is_empty(),
        "package part {} must not define relationship type",
        part.name
      );
    }

    match part.content_kind {
      PartContentKind::None | PartContentKind::Text | PartContentKind::Binary => {}
      PartContentKind::Xml => {
        assert!(
          !part.root.is_empty()
            || !part.root_type.is_empty()
            || part.name == "CoreFilePropertiesPart",
          "xml part {} must define root or resolved root type",
          part.name
        );
      }
    }
  }

  let available_part_names: HashSet<&str> = parts.iter().map(|part| part.name.as_str()).collect();

  for part in parts {
    for child in &part.children {
      if child.is_data_part_reference {
        continue;
      }

      assert!(
        available_part_names.contains(child.name.as_str()),
        "part {} references missing child part {}",
        part.name,
        child.name
      );
    }
  }
}
