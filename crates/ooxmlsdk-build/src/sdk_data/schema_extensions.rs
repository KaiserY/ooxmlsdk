use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::Result;
use crate::sdk_data::sdk_data_model::{Schema, SchemaTypeAttribute};
use crate::sdk_data::sdk_data_model::{SchemaTypeChild, SchemaTypeChildKind};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaExtensions {
  pub types: Vec<SchemaTypeExtension>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeExtension {
  pub class_name: String,
  pub has_xmlns_fields: Option<bool>,
  pub has_mc_ignorable_field: Option<bool>,
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

      merge_schema_type_attributes(&mut schema_type.attributes, &extension.attributes);
      merge_schema_type_children(&mut schema_type.children, &extension.children);
    }
  }

  Ok(())
}

fn merge_schema_type_attributes(
  target: &mut Vec<SchemaTypeAttribute>,
  extensions: &[SchemaTypeAttribute],
) {
  for attribute in extensions {
    if target.iter().any(|existing| {
      existing.q_name == attribute.q_name
        || (!attribute.property_name.is_empty()
          && existing.property_name == attribute.property_name)
    }) {
      continue;
    }

    target.push(attribute.clone());
  }
}

fn merge_schema_type_children(
  target: &mut Vec<SchemaTypeChild>,
  extensions: &[SchemaTypeChildExtension],
) {
  for extension in extensions {
    if let Some(target_child) = find_merge_target(target, extension) {
      merge_schema_type_child(target_child, extension);
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
