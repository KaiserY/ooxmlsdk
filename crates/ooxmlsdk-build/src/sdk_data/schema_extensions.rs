use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::Result;
use crate::sdk_data::sdk_data_model::Schema;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaExtensions {
  pub types: Vec<SchemaTypeExtension>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeExtension {
  pub class_name: String,
  pub have_xmlns_fields: Option<bool>,
  pub have_xml_other_attrs: Option<bool>,
  pub have_xml_other_children: Option<bool>,
  pub have_direct_xml_other_children: Option<bool>,
  pub parent_have_xml_other_children: Option<bool>,
  pub children: Vec<SchemaTypeChildExtension>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeChildExtension {
  pub name: String,
  pub property_name: String,
  pub optional: Option<bool>,
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

      if let Some(have_xmlns_fields) = extension.have_xmlns_fields {
        schema_type.have_xmlns_fields = have_xmlns_fields;
      }
      if let Some(have_xml_other_attrs) = extension.have_xml_other_attrs {
        schema_type.have_xml_other_attrs = have_xml_other_attrs;
      }
      if let Some(have_xml_other_children) = extension.have_xml_other_children {
        schema_type.have_xml_other_children = have_xml_other_children;
      }
      if let Some(have_direct_xml_other_children) = extension.have_direct_xml_other_children {
        schema_type.have_direct_xml_other_children = have_direct_xml_other_children;
      }
      if let Some(parent_have_xml_other_children) = extension.parent_have_xml_other_children {
        schema_type.parent_have_xml_other_children = parent_have_xml_other_children;
      }

      for child_extension in &extension.children {
        let Some(child) = find_child_mut(&mut schema_type.children, child_extension) else {
          return Err(
            format!(
              "schema extension child {}.{} not found",
              module_name, extension.class_name
            )
            .into(),
          );
        };

        if let Some(optional) = child_extension.optional {
          child.optional = optional;
        }
      }
    }
  }

  Ok(())
}

fn find_child_mut<'a>(
  children: &'a mut [crate::sdk_data::sdk_data_model::SchemaTypeChild],
  extension: &SchemaTypeChildExtension,
) -> Option<&'a mut crate::sdk_data::sdk_data_model::SchemaTypeChild> {
  children.iter_mut().find_map(|child| {
    if (!extension.name.is_empty() && child.name == extension.name)
      || (!extension.property_name.is_empty() && child.property_name == extension.property_name)
    {
      Some(child)
    } else {
      find_child_mut(&mut child.children, extension)
    }
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::sdk_data::sdk_data_model::{SchemaType, SchemaTypeChild};

  #[test]
  fn applies_child_optional_extension_by_property_name() {
    let mut schemas = vec![Schema {
      module_name: "test_schema".to_string(),
      types: vec![SchemaType {
        class_name: "Parent".to_string(),
        children: vec![SchemaTypeChild {
          name: "t:CT_Child/t:child".to_string(),
          property_name: "Child".to_string(),
          optional: false,
          ..Default::default()
        }],
        ..Default::default()
      }],
      ..Default::default()
    }];
    let extensions = vec![(
      "test_schema".to_string(),
      SchemaExtensions {
        types: vec![SchemaTypeExtension {
          class_name: "Parent".to_string(),
          have_direct_xml_other_children: Some(true),
          parent_have_xml_other_children: Some(true),
          children: vec![SchemaTypeChildExtension {
            property_name: "Child".to_string(),
            optional: Some(true),
            ..Default::default()
          }],
          ..Default::default()
        }],
      },
    )];

    apply_schema_extensions(&mut schemas, &extensions).unwrap();

    assert!(schemas[0].types[0].children[0].optional);
    assert!(schemas[0].types[0].have_direct_xml_other_children);
    assert!(schemas[0].types[0].parent_have_xml_other_children);
  }
}
