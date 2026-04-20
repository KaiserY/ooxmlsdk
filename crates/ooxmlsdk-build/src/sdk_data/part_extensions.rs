use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::Result;
use crate::sdk_data::sdk_data_model::{Part, PartChild, PartContentKind, PartPaths};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PartExtensions {
  pub parts: Vec<PartExtension>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PartExtension {
  pub name: String,
  pub clone_from_name: String,
  pub base: String,
  pub content_type: String,
  pub relationship_type: String,
  pub target: String,
  pub root: String,
  pub root_element: String,
  pub extension: String,
  pub version: String,
  #[serde(default)]
  pub features: Vec<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_kind: Option<PartContentKind>,
  pub paths: PartPaths,
  pub root_type: String,
  pub root_class_name: String,
  pub schema_module: String,
  #[serde(default)]
  pub children: Vec<PartChildExtension>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct PartChildExtension {
  pub min_occurs_is_non_zero: bool,
  pub max_occurs_great_than_one: bool,
  pub api_name: String,
  pub name: String,
  pub relationship_type: String,
  pub version: String,
  #[serde(default)]
  pub features: Vec<String>,
  pub has_fixed_content: bool,
  pub is_data_part_reference: bool,
  pub is_special_embedded_part: bool,
  pub insert_before: Option<String>,
}

pub fn read_part_extensions(dir: &Path) -> Result<Vec<(String, PartExtensions)>> {
  let mut part_extensions = vec![];

  if !dir.exists() {
    return Ok(part_extensions);
  }

  for entry in std::fs::read_dir(dir)? {
    let entry = entry?;
    let path = entry.path();

    if !path.is_file() || path.extension() != Some(std::ffi::OsStr::new("json")) {
      continue;
    }

    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let extensions: PartExtensions = serde_json::from_reader(reader)?;
    let Some(module_name) = path.file_stem().and_then(|stem| stem.to_str()) else {
      continue;
    };

    part_extensions.push((module_name.to_string(), extensions));
  }

  part_extensions.sort_by(|left, right| left.0.cmp(&right.0));

  Ok(part_extensions)
}

pub fn apply_part_extensions(
  parts: &mut Vec<Part>,
  part_extensions: &[(String, PartExtensions)],
) -> Result<()> {
  let mut pending: Vec<_> = part_extensions
    .iter()
    .flat_map(|(module_name, extensions)| {
      extensions
        .parts
        .iter()
        .map(move |extension| (module_name.as_str(), extension))
    })
    .collect();

  while !pending.is_empty() {
    let mut progressed = false;
    let mut next_pending = Vec::new();

    for (module_name, extension) in pending {
      let Some(part_index) = ensure_part(parts, module_name, extension)? else {
        next_pending.push((module_name, extension));
        continue;
      };
      let part = &mut parts[part_index];

      merge_part(part, extension);
      merge_part_children(part.name.as_str(), &mut part.children, &extension.children)?;
      progressed = true;
    }

    if !progressed {
      let unresolved = next_pending
        .iter()
        .map(|(module_name, extension)| format!("{}.{}", module_name, extension.clone_from_name))
        .collect::<Vec<_>>()
        .join(", ");
      return Err(format!("unresolved part extension clone sources: {unresolved}").into());
    }

    pending = next_pending;
  }

  fill_part_child_metadata(parts);
  validate_parts(parts)?;
  parts.sort_by(|left, right| left.module_name.cmp(&right.module_name));

  Ok(())
}

fn ensure_part(
  parts: &mut Vec<Part>,
  module_name: &str,
  extension: &PartExtension,
) -> Result<Option<usize>> {
  if let Some(index) = parts
    .iter()
    .position(|part| part.module_name == module_name)
  {
    return Ok(Some(index));
  }

  if !extension.name.is_empty()
    && let Some(index) = parts.iter().position(|part| part.name == extension.name)
  {
    return Ok(Some(index));
  }

  let mut part = if !extension.clone_from_name.is_empty() {
    let Some(source) = parts
      .iter()
      .find(|part| part.name == extension.clone_from_name)
      .cloned()
    else {
      return Ok(None);
    };
    source
  } else {
    Part::default()
  };

  part.module_name = module_name.to_string();

  if !extension.name.is_empty() {
    part.name = extension.name.clone();
  }

  if part.name.is_empty() {
    return Err(format!("part extension module {module_name} is missing Name").into());
  }

  parts.push(part);
  Ok(Some(parts.len() - 1))
}

fn merge_part(target: &mut Part, extension: &PartExtension) {
  if !extension.name.is_empty() {
    target.name = extension.name.clone();
  }
  if !extension.base.is_empty() {
    target.base = extension.base.clone();
  }
  if !extension.content_type.is_empty() {
    target.content_type = extension.content_type.clone();
  }
  if !extension.relationship_type.is_empty() {
    target.relationship_type = extension.relationship_type.clone();
  }
  if !extension.target.is_empty() {
    target.target = extension.target.clone();
  }
  if !extension.root.is_empty() {
    target.root = extension.root.clone();
  }
  if !extension.root_element.is_empty() {
    target.root_element = extension.root_element.clone();
  }
  if !extension.extension.is_empty() {
    target.extension = extension.extension.clone();
  }
  if !extension.version.is_empty() {
    target.version = extension.version.clone();
  }
  if !extension.features.is_empty() {
    target.features = extension.features.clone();
  }
  if let Some(content_kind) = extension.content_kind.clone() {
    target.content_kind = content_kind;
  }
  merge_part_paths(&mut target.paths, &extension.paths);
  if !extension.root_type.is_empty() {
    target.root_type = extension.root_type.clone();
  }
  if !extension.root_class_name.is_empty() {
    target.root_class_name = extension.root_class_name.clone();
  }
  if !extension.schema_module.is_empty() {
    target.schema_module = extension.schema_module.clone();
  }
}

fn merge_part_paths(target: &mut PartPaths, extension: &PartPaths) {
  if !extension.general.is_empty() {
    target.general = extension.general.clone();
  }
  if !extension.word.is_empty() {
    target.word = extension.word.clone();
  }
  if !extension.excel.is_empty() {
    target.excel = extension.excel.clone();
  }
  if !extension.power_point.is_empty() {
    target.power_point = extension.power_point.clone();
  }
}

fn merge_part_children(
  part_name: &str,
  target: &mut Vec<PartChild>,
  extensions: &[PartChildExtension],
) -> Result<()> {
  for extension in extensions {
    if let Some(existing) = target.iter_mut().find(|child| {
      (!extension.api_name.is_empty() && child.api_name == extension.api_name)
        || (!extension.relationship_type.is_empty()
          && !child.relationship_type.is_empty()
          && child.relationship_type == extension.relationship_type)
        || (!extension.name.is_empty()
          && child.name == extension.name
          && extension.relationship_type.is_empty())
    }) {
      merge_part_child(existing, extension);
      continue;
    }

    let child = runtime_part_child(extension);
    if let Some(insert_before) = &extension.insert_before {
      let Some(index) = find_insert_before_child_index(target, insert_before) else {
        return Err(
          format!(
            "part extension {} insert_before target {} not found",
            part_name, insert_before
          )
          .into(),
        );
      };
      target.insert(index, child);
    } else {
      target.push(child);
    }
  }

  Ok(())
}

fn merge_part_child(target: &mut PartChild, extension: &PartChildExtension) {
  if extension.min_occurs_is_non_zero {
    target.min_occurs_is_non_zero = true;
  }
  if extension.max_occurs_great_than_one {
    target.max_occurs_great_than_one = true;
  }
  if !extension.api_name.is_empty() {
    target.api_name = extension.api_name.clone();
  }
  if !extension.name.is_empty() {
    if target.is_data_part_reference && target.name != extension.name {
      target.is_data_part_reference = false;
    }
    target.name = extension.name.clone();
  }
  if !extension.relationship_type.is_empty() {
    target.relationship_type = extension.relationship_type.clone();
  }
  if !extension.version.is_empty() {
    target.version = extension.version.clone();
  }
  if !extension.features.is_empty() {
    target.features = extension.features.clone();
  }
  if extension.has_fixed_content {
    target.has_fixed_content = true;
  }
  if extension.is_data_part_reference {
    target.is_data_part_reference = true;
  }
  if extension.is_special_embedded_part {
    target.is_special_embedded_part = true;
  }
}

fn runtime_part_child(extension: &PartChildExtension) -> PartChild {
  PartChild {
    min_occurs_is_non_zero: extension.min_occurs_is_non_zero,
    max_occurs_great_than_one: extension.max_occurs_great_than_one,
    api_name: extension.api_name.clone(),
    name: extension.name.clone(),
    relationship_type: extension.relationship_type.clone(),
    version: extension.version.clone(),
    features: extension.features.clone(),
    has_fixed_content: extension.has_fixed_content,
    is_data_part_reference: extension.is_data_part_reference,
    is_special_embedded_part: extension.is_special_embedded_part,
  }
}

fn find_insert_before_child_index(children: &[PartChild], insert_before: &str) -> Option<usize> {
  children.iter().position(|child| {
    child.api_name == insert_before
      || child.name == insert_before
      || child.relationship_type == insert_before
  })
}

fn fill_part_child_metadata(parts: &mut [Part]) {
  let part_metadata: HashMap<String, (String, String, Vec<String>)> = parts
    .iter()
    .map(|part| {
      (
        part.name.clone(),
        (
          part.relationship_type.clone(),
          part.version.clone(),
          part.features.clone(),
        ),
      )
    })
    .collect();

  for part in parts {
    for child in &mut part.children {
      if let Some((relationship_type, version, features)) = part_metadata.get(&child.name) {
        if child.relationship_type.is_empty() {
          child.relationship_type = relationship_type.clone();
        }
        if child.version.is_empty() {
          child.version = version.clone();
        }
        if child.features.is_empty() {
          child.features = features.clone();
        }
      }
    }
  }
}

fn validate_parts(parts: &[Part]) -> Result<()> {
  let mut part_names: BTreeSet<&str> = BTreeSet::new();

  for part in parts {
    if !part_names.insert(part.name.as_str()) {
      return Err(format!("duplicate part definition {}", part.name).into());
    }

    if part.base == "OpenXmlPackage" && !part.relationship_type.is_empty() {
      return Err(
        format!(
          "package part {} must not define relationship type",
          part.name
        )
        .into(),
      );
    }

    if part.content_kind == PartContentKind::Xml
      && part.root.is_empty()
      && part.root_type.is_empty()
      && part.name != "CoreFilePropertiesPart"
    {
      return Err(
        format!(
          "xml part {} must define root or resolved root type",
          part.name
        )
        .into(),
      );
    }
  }

  let available_part_names: HashSet<&str> = parts.iter().map(|part| part.name.as_str()).collect();

  for part in parts {
    for child in &part.children {
      if child.is_data_part_reference {
        continue;
      }

      if !available_part_names.contains(child.name.as_str()) {
        return Err(
          format!(
            "part {} references missing child part {}",
            part.name, child.name
          )
          .into(),
        );
      }
    }
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn apply_part_extensions_can_add_new_text_part_and_parent_child() {
    let mut parts = vec![Part {
      name: "MainDocumentPart".to_string(),
      module_name: "main_document_part".to_string(),
      base: "OpenXmlPart".to_string(),
      content_kind: PartContentKind::Xml,
      root: "w:CT_Document".to_string(),
      root_type: "w:CT_Document/w:document".to_string(),
      root_class_name: "Document".to_string(),
      schema_module: "schemas_openxmlformats_org_wordprocessingml_2006_main".to_string(),
      features: vec!["parts".to_string()],
      paths: PartPaths {
        general: "word".to_string(),
        ..Default::default()
      },
      ..Default::default()
    }];

    let part_extensions = vec![
      (
        "main_document_part".to_string(),
        PartExtensions {
          parts: vec![PartExtension {
            name: "MainDocumentPart".to_string(),
            children: vec![PartChildExtension {
              max_occurs_great_than_one: true,
              api_name: "WordprocessingTextBoxParts".to_string(),
              name: "WordprocessingTextBoxPart".to_string(),
              ..Default::default()
            }],
            ..Default::default()
          }],
        },
      ),
      (
        "wordprocessing_text_box_part".to_string(),
        PartExtensions {
          parts: vec![PartExtension {
            name: "WordprocessingTextBoxPart".to_string(),
            base: "OpenXmlPart".to_string(),
            content_type: "application/vnd.openxmlformats-officedocument.wordprocessingml.txbx+xml"
              .to_string(),
            relationship_type: "http://schemas.microsoft.com/office/2006/relationships/txbx"
              .to_string(),
            target: "txbx".to_string(),
            extension: ".xml".to_string(),
            features: vec!["parts".to_string()],
            content_kind: Some(PartContentKind::Text),
            paths: PartPaths {
              general: "word".to_string(),
              ..Default::default()
            },
            ..Default::default()
          }],
        },
      ),
    ];

    apply_part_extensions(&mut parts, &part_extensions).unwrap();

    let main_document_part = parts
      .iter()
      .find(|part| part.name == "MainDocumentPart")
      .unwrap();
    let text_box_child = main_document_part
      .children
      .iter()
      .find(|child| child.name == "WordprocessingTextBoxPart")
      .unwrap();
    let text_box_part = parts
      .iter()
      .find(|part| part.name == "WordprocessingTextBoxPart")
      .unwrap();

    assert_eq!(text_box_part.module_name, "wordprocessing_text_box_part");
    assert_eq!(text_box_part.content_kind, PartContentKind::Text);
    assert_eq!(text_box_part.paths.general, "word");
    assert!(text_box_child.max_occurs_great_than_one);
    assert_eq!(
      text_box_child.relationship_type,
      "http://schemas.microsoft.com/office/2006/relationships/txbx"
    );
    assert_eq!(text_box_child.features, vec!["parts".to_string()]);
  }

  #[test]
  fn apply_part_extensions_keeps_same_named_children_with_distinct_relationship_types() {
    let mut parts = vec![
      Part {
        name: "SlidePart".to_string(),
        module_name: "slide_part".to_string(),
        base: "OpenXmlPart".to_string(),
        content_kind: PartContentKind::Xml,
        root: "p:CT_Slide".to_string(),
        root_type: "p:CT_Slide/p:sld".to_string(),
        root_class_name: "Slide".to_string(),
        schema_module: "schemas_openxmlformats_org_presentationml_2006_main".to_string(),
        features: vec!["parts".to_string()],
        paths: PartPaths {
          general: "slides".to_string(),
          ..Default::default()
        },
        ..Default::default()
      },
      Part {
        name: "MediaDataPart".to_string(),
        module_name: "media_data_part".to_string(),
        base: "OpenXmlPart".to_string(),
        content_kind: PartContentKind::Binary,
        features: vec!["parts".to_string()],
        paths: PartPaths {
          general: "../media".to_string(),
          ..Default::default()
        },
        ..Default::default()
      },
    ];
    parts[0].children.push(PartChild {
      max_occurs_great_than_one: true,
      api_name: "AudioReferenceRelationships".to_string(),
      name: "AudioReferenceRelationship".to_string(),
      relationship_type:
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio".to_string(),
      is_data_part_reference: true,
      ..Default::default()
    });

    let part_extensions = vec![(
      "slide_part".to_string(),
      PartExtensions {
        parts: vec![PartExtension {
          name: "SlidePart".to_string(),
          children: vec![
            PartChildExtension {
              max_occurs_great_than_one: true,
              api_name: "AudioReferenceRelationships".to_string(),
              name: "MediaDataPart".to_string(),
              relationship_type:
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio"
                  .to_string(),
              ..Default::default()
            },
            PartChildExtension {
              max_occurs_great_than_one: true,
              api_name: "MediaReferenceRelationships".to_string(),
              name: "MediaDataPart".to_string(),
              relationship_type: "http://schemas.microsoft.com/office/2007/relationships/media"
                .to_string(),
              ..Default::default()
            },
          ],
          ..Default::default()
        }],
      },
    )];

    apply_part_extensions(&mut parts, &part_extensions).unwrap();

    let slide_part = parts.iter().find(|part| part.name == "SlidePart").unwrap();
    assert_eq!(slide_part.children.len(), 2);
    assert_eq!(
      slide_part.children[0].api_name,
      "AudioReferenceRelationships"
    );
    assert!(!slide_part.children[0].is_data_part_reference);
    assert_eq!(
      slide_part.children[0].relationship_type,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio"
    );
    assert_eq!(
      slide_part.children[1].api_name,
      "MediaReferenceRelationships"
    );
    assert_eq!(
      slide_part.children[1].relationship_type,
      "http://schemas.microsoft.com/office/2007/relationships/media"
    );
  }

  #[test]
  fn apply_part_extensions_resolves_clone_from_extension_defined_part_out_of_order() {
    let mut parts = vec![];
    let part_extensions = vec![
      (
        "derived_media_part".to_string(),
        PartExtensions {
          parts: vec![PartExtension {
            name: "DerivedMediaPart".to_string(),
            clone_from_name: "MediaDataPart".to_string(),
            relationship_type: "http://example.com/relationships/derivedMedia".to_string(),
            ..Default::default()
          }],
        },
      ),
      (
        "media_data_part".to_string(),
        PartExtensions {
          parts: vec![PartExtension {
            name: "MediaDataPart".to_string(),
            base: "OpenXmlPart".to_string(),
            target: "mediadata".to_string(),
            extension: ".bin".to_string(),
            features: vec!["parts".to_string()],
            content_kind: Some(PartContentKind::Binary),
            paths: PartPaths {
              general: "../media".to_string(),
              ..Default::default()
            },
            ..Default::default()
          }],
        },
      ),
    ];

    apply_part_extensions(&mut parts, &part_extensions).unwrap();

    let derived_media_part = parts
      .iter()
      .find(|part| part.name == "DerivedMediaPart")
      .unwrap();
    assert_eq!(derived_media_part.module_name, "derived_media_part");
    assert_eq!(derived_media_part.content_kind, PartContentKind::Binary);
    assert_eq!(derived_media_part.paths.general, "../media");
    assert_eq!(
      derived_media_part.relationship_type,
      "http://example.com/relationships/derivedMedia"
    );
  }

  #[test]
  fn apply_part_extensions_errors_when_insert_before_target_is_missing() {
    let mut parts = vec![Part {
      name: "SlidePart".to_string(),
      module_name: "slide_part".to_string(),
      base: "OpenXmlPart".to_string(),
      children: vec![PartChild {
        api_name: "ImageParts".to_string(),
        name: "ImagePart".to_string(),
        relationship_type:
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image".to_string(),
        ..Default::default()
      }],
      ..Default::default()
    }];

    let err = apply_part_extensions(
      &mut parts,
      &[(
        "slide_part".to_string(),
        PartExtensions {
          parts: vec![PartExtension {
            name: "SlidePart".to_string(),
            children: vec![PartChildExtension {
              api_name: "AudioReferenceRelationships".to_string(),
              name: "MediaDataPart".to_string(),
              relationship_type:
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio"
                  .to_string(),
              insert_before: Some("MissingChild".to_string()),
              ..Default::default()
            }],
            ..Default::default()
          }],
        },
      )],
    )
    .unwrap_err();

    assert!(
      err
        .to_string()
        .contains("insert_before target MissingChild not found")
    );
  }
}
