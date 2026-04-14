use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::Result;
use crate::sdk_data::schemas::normalize_schema_type_children;
use crate::sdk_data::sdk_data_model::{Schema, SchemaEnum, SchemaEnumFacet, SchemaTypeAttribute};
use crate::sdk_data::sdk_data_model::{SchemaTypeChild, SchemaTypeChildKind};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaExtensions {
  pub types: Vec<SchemaTypeExtension>,
  #[serde(default)]
  pub enums: Vec<SchemaEnum>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeExtension {
  pub class_name: String,
  pub has_xmlns_fields: Option<bool>,
  pub has_mc_ignorable_field: Option<bool>,
  pub text_value_type: Option<String>,
  #[serde(default)]
  pub attributes: Vec<SchemaTypeAttribute>,
  #[serde(default)]
  pub children: Vec<SchemaTypeChildExtension>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeChildExtension {
  pub kind: SchemaTypeChildKind,
  pub name: String,
  pub property_name: String,
  #[serde(default)]
  pub optional: bool,
  pub insert_before: Option<String>,
  #[serde(default)]
  pub children: Vec<SchemaTypeChildExtension>,
}

pub fn read_schema_extensions(dir: &Path) -> Result<Vec<(String, SchemaExtensions)>> {
  let mut schema_extensions = vec![];

  if !dir.exists() {
    return Ok(schema_extensions);
  }

  for entry in std::fs::read_dir(dir)? {
    let entry = entry?;
    let path = entry.path();

    if !path.is_file() || path.extension() != Some(std::ffi::OsStr::new("json")) {
      continue;
    }

    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let extensions: SchemaExtensions = serde_json::from_reader(reader)?;
    let Some(module_name) = path.file_stem().and_then(|stem| stem.to_str()) else {
      continue;
    };

    schema_extensions.push((module_name.to_string(), extensions));
  }

  schema_extensions.sort_by(|left, right| left.0.cmp(&right.0));

  Ok(schema_extensions)
}

pub fn apply_schema_extensions(
  schemas: &mut [Schema],
  schema_extensions: &[(String, SchemaExtensions)],
) -> Result<()> {
  for (module_name, extensions) in schema_extensions {
    let Some(schema) = schemas
      .iter_mut()
      .find(|schema| &schema.module_name == module_name)
    else {
      return Err(format!("schema extension module {module_name} not found").into());
    };

    for extension in &extensions.types {
      let Some(schema_type) = schema
        .types
        .iter_mut()
        .find(|schema_type| schema_type.class_name == extension.class_name)
      else {
        return Err(
          format!(
            "schema extension type {}.{} not found",
            module_name, extension.class_name
          )
          .into(),
        );
      };

      if let Some(has_xmlns_fields) = extension.has_xmlns_fields {
        schema_type.has_xmlns_fields = has_xmlns_fields;
      }

      if let Some(has_mc_ignorable_field) = extension.has_mc_ignorable_field {
        schema_type.has_mc_ignorable_field = has_mc_ignorable_field;
      }

      if let Some(text_value_type) = &extension.text_value_type {
        schema_type.text_value_type = text_value_type.clone();
      }

      merge_schema_type_attributes(&mut schema_type.attributes, &extension.attributes);
      merge_schema_type_children(&mut schema_type.children, &extension.children);
      normalize_schema_type_children(&mut schema_type.children);
    }

    for extension in &extensions.enums {
      let Some(schema_enum) = schema
        .enums
        .iter_mut()
        .find(|schema_enum| schema_enum.name == extension.name)
      else {
        return Err(
          format!(
            "schema extension enum {}.{} not found",
            module_name, extension.name
          )
          .into(),
        );
      };

      merge_schema_enum_facets(&mut schema_enum.facets, &extension.facets);
    }
  }

  Ok(())
}

fn merge_schema_type_attributes(
  target: &mut Vec<SchemaTypeAttribute>,
  extensions: &[SchemaTypeAttribute],
) {
  for attribute in extensions {
    if let Some(existing) = target.iter_mut().find(|existing| {
      existing.q_name == attribute.q_name
        || (!attribute.property_name.is_empty()
          && existing.property_name == attribute.property_name)
    }) {
      merge_schema_type_attribute(existing, attribute);
    } else {
      target.push(attribute.clone());
    }
  }
}

fn merge_schema_type_attribute(target: &mut SchemaTypeAttribute, extension: &SchemaTypeAttribute) {
  if !extension.q_name.is_empty() {
    target.q_name = extension.q_name.clone();
  }
  if !extension.property_name.is_empty() {
    target.property_name = extension.property_name.clone();
  }
  if !extension.r#type.is_empty() {
    target.r#type = extension.r#type.clone();
  }
  if !extension.property_comments.is_empty() {
    target.property_comments = extension.property_comments.clone();
  }
  if !extension.version.is_empty() {
    target.version = extension.version.clone();
  }
  if extension.required {
    target.required = true;
  }
  if extension.bit.is_some() {
    target.bit = extension.bit;
  }
}

fn merge_schema_enum_facets(target: &mut Vec<SchemaEnumFacet>, extensions: &[SchemaEnumFacet]) {
  for facet in extensions {
    if let Some(existing) = target.iter_mut().find(|existing| {
      (!facet.name.is_empty() && existing.name == facet.name)
        || (!facet.value.is_empty() && existing.value == facet.value)
    }) {
      merge_schema_enum_facet(existing, facet);
    } else {
      target.push(facet.clone());
    }
  }
}

fn merge_schema_enum_facet(target: &mut SchemaEnumFacet, extension: &SchemaEnumFacet) {
  if !extension.name.is_empty() {
    target.name = extension.name.clone();
  }
  if !extension.value.is_empty() {
    target.value = extension.value.clone();
  }
  if !extension.version.is_empty() {
    target.version = extension.version.clone();
  }
  for alias in &extension.aliases {
    if !target.aliases.contains(alias) {
      target.aliases.push(alias.clone());
    }
  }
}

fn merge_schema_type_children(
  target: &mut Vec<SchemaTypeChild>,
  extensions: &[SchemaTypeChildExtension],
) {
  for extension in extensions {
    if let Some(target_child) = find_merge_target(target, extension) {
      merge_schema_type_child(target_child, extension);
    } else if let Some(insert_before) = &extension.insert_before {
      if let Some(index) = find_insert_before_index(target, insert_before) {
        target.insert(index, runtime_schema_type_child(extension));
      } else {
        target.push(runtime_schema_type_child(extension));
      }
    } else {
      target.push(runtime_schema_type_child(extension));
    }
  }
}

fn merge_schema_type_child(target: &mut SchemaTypeChild, extension: &SchemaTypeChildExtension) {
  if !extension.name.is_empty() {
    target.name = extension.name.clone();
  }
  if !extension.property_name.is_empty() {
    target.property_name = extension.property_name.clone();
  }
  target.kind = extension.kind;
  target.optional = extension.optional;

  merge_schema_type_children(&mut target.children, &extension.children);
}

fn find_merge_target<'a>(
  target: &'a mut [SchemaTypeChild],
  extension: &SchemaTypeChildExtension,
) -> Option<&'a mut SchemaTypeChild> {
  if !extension.name.is_empty()
    && let Some(index) = target.iter().position(|child| child.name == extension.name)
  {
    return target.get_mut(index);
  }

  if !extension.property_name.is_empty()
    && let Some(index) = target
      .iter()
      .position(|child| child.property_name == extension.property_name)
  {
    return target.get_mut(index);
  }

  if matches!(extension.kind, SchemaTypeChildKind::Choice)
    && target
      .iter()
      .filter(|child| child.kind == SchemaTypeChildKind::Choice)
      .count()
      == 1
  {
    return target
      .iter_mut()
      .find(|child| child.kind == SchemaTypeChildKind::Choice);
  }

  None
}

fn find_insert_before_index(target: &[SchemaTypeChild], insert_before: &str) -> Option<usize> {
  target.iter().position(|child| {
    child.name == insert_before
      || child.name.ends_with(&format!("/{insert_before}"))
      || child.property_name == insert_before
      || (child.kind == SchemaTypeChildKind::Choice
        && choice_child_contains_qname(child, insert_before))
  })
}

fn choice_child_contains_qname(child: &SchemaTypeChild, child_qname: &str) -> bool {
  child.children.iter().any(|nested| {
    nested.name == child_qname
      || nested.name.ends_with(&format!("/{child_qname}"))
      || nested.property_name == child_qname
      || choice_child_contains_qname(nested, child_qname)
  })
}

fn runtime_schema_type_child(extension: &SchemaTypeChildExtension) -> SchemaTypeChild {
  SchemaTypeChild {
    name: extension.name.clone(),
    property_name: extension.property_name.clone(),
    property_comments: String::new(),
    kind: extension.kind,
    optional: extension.optional,
    repeated: false,
    initial_version: String::new(),
    children: extension
      .children
      .iter()
      .map(runtime_schema_type_child)
      .collect(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::sdk_data::sdk_data_model::{
    SchemaType, SchemaTypeApiKind, SchemaTypeChild, SchemaTypeChildKind, SchemaTypeCompositeKind,
    SchemaTypeKind, SchemaTypeParticle, SchemaTypeXmlHeader,
  };

  fn anonymous_choice(children: Vec<SchemaTypeChild>) -> SchemaTypeChild {
    SchemaTypeChild {
      name: String::new(),
      property_name: String::new(),
      property_comments: String::new(),
      kind: SchemaTypeChildKind::Choice,
      optional: false,
      repeated: true,
      initial_version: String::new(),
      children,
    }
  }

  fn leaf(name: &str) -> SchemaTypeChild {
    SchemaTypeChild {
      name: name.to_string(),
      property_name: String::new(),
      property_comments: String::new(),
      kind: SchemaTypeChildKind::Child,
      optional: true,
      repeated: true,
      initial_version: String::new(),
      children: Vec::new(),
    }
  }

  #[test]
  fn normalizes_named_choice_after_extension_merge() {
    let mut schemas = vec![Schema {
      module_name: "test".to_string(),
      types: vec![SchemaType {
        class_name: "Paragraph".to_string(),
        children: vec![anonymous_choice(vec![anonymous_choice(vec![
          leaf("w:CT_R/w:r"),
          leaf("m:CT_OMath/m:oMath"),
        ])])],
        ..SchemaType {
          name: String::new(),
          class_name: String::new(),
          summary: String::new(),
          version: None,
          part: String::new(),
          base_class: String::new(),
          kind: SchemaTypeKind::Composite,
          composite_kind: SchemaTypeCompositeKind::OneSequence,
          xml_header: SchemaTypeXmlHeader::None,
          is_abstract: false,
          has_xmlns_fields: false,
          has_mc_ignorable_field: false,
          text_value_type: String::new(),
          api_kind: SchemaTypeApiKind::Struct,
          attributes: Vec::new(),
          children: Vec::new(),
          particle: SchemaTypeParticle::default(),
        }
      }],
      ..Schema::default()
    }];
    let schema_extensions = vec![(
      "test".to_string(),
      SchemaExtensions {
        types: vec![SchemaTypeExtension {
          class_name: "Paragraph".to_string(),
          children: vec![SchemaTypeChildExtension {
            kind: SchemaTypeChildKind::Choice,
            name: String::new(),
            property_name: "eg_p_content".to_string(),
            optional: false,
            insert_before: None,
            children: Vec::new(),
          }],
          ..SchemaTypeExtension::default()
        }],
        enums: Vec::new(),
      },
    )];

    apply_schema_extensions(&mut schemas, &schema_extensions).expect("apply schema extensions");

    let paragraph = &schemas[0].types[0];
    assert_eq!(paragraph.children.len(), 1);
    assert_eq!(paragraph.children[0].property_name, "eg_p_content");
    assert_eq!(paragraph.children[0].children.len(), 2);
    assert_eq!(paragraph.children[0].children[0].name, "w:CT_R/w:r");
    assert_eq!(paragraph.children[0].children[1].name, "m:CT_OMath/m:oMath");
  }
}
