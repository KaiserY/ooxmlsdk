use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::Result;
use crate::sdk_data::sdk_data_model::Schema;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaExtensions {
  pub enums: Vec<SchemaEnumExtension>,
  pub types: Vec<SchemaTypeExtension>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaEnumExtension {
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub has_other_variant: Option<bool>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub add_facets: Vec<SchemaEnumFacetExtension>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaEnumFacetExtension {
  pub name: String,
  pub value: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub version: String,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub aliases: Vec<String>,
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

      if extension.has_other_variant.unwrap_or(false) {
        schema_enum.other_variant = Some(crate::sdk_data::sdk_data_model::SchemaEnumOtherVariant {
          name: "OtherVariant".to_string(),
          r#type: "Box<str>".to_string(),
        });
      }

      for facet_extension in &extension.add_facets {
        let already_exists = schema_enum.facets.iter().any(|facet| {
          facet.name == facet_extension.name
            && facet.value == facet_extension.value
            && facet.version == facet_extension.version
        });

        if already_exists {
          continue;
        }

        schema_enum
          .facets
          .push(crate::sdk_data::sdk_data_model::SchemaEnumFacet {
            name: facet_extension.name.clone(),
            value: facet_extension.value.clone(),
            version: facet_extension.version.clone(),
            aliases: facet_extension.aliases.clone(),
          });
      }
    }

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
  use crate::sdk_data::sdk_data_model::{SchemaEnum, SchemaEnumFacet, SchemaType, SchemaTypeChild};

  #[test]
  fn applies_enum_other_variant_extension() {
    let mut schemas = vec![Schema {
      module_name: "test_schema".to_string(),
      enums: vec![SchemaEnum {
        name: "StrictCharacterSet".to_string(),
        facets: vec![SchemaEnumFacet {
          name: "Known".to_string(),
          value: "known".to_string(),
          ..Default::default()
        }],
        ..Default::default()
      }],
      ..Default::default()
    }];
    let extensions = vec![(
      "test_schema".to_string(),
      SchemaExtensions {
        enums: vec![SchemaEnumExtension {
          name: "StrictCharacterSet".to_string(),
          has_other_variant: Some(true),
          add_facets: vec![],
        }],
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &extensions).unwrap();

    let other = schemas[0].enums[0].other_variant.as_ref().unwrap();
    assert_eq!(other.name, "OtherVariant");
    assert_eq!(other.r#type, "Box<str>");
  }

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
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &extensions).unwrap();

    assert!(schemas[0].types[0].children[0].optional);
    assert!(schemas[0].types[0].have_direct_xml_other_children);
    assert!(schemas[0].types[0].parent_have_xml_other_children);
  }

  #[test]
  fn applies_enum_add_facets_extension() {
    let mut schemas = vec![Schema {
      module_name: "test_schema".to_string(),
      enums: vec![SchemaEnum {
        name: "LevelJustificationValues".to_string(),
        facets: vec![SchemaEnumFacet {
          name: "Left".to_string(),
          value: "left".to_string(),
          ..Default::default()
        }],
        ..Default::default()
      }],
      ..Default::default()
    }];
    let extensions = vec![(
      "test_schema".to_string(),
      SchemaExtensions {
        enums: vec![SchemaEnumExtension {
          name: "LevelJustificationValues".to_string(),
          add_facets: vec![
            SchemaEnumFacetExtension {
              name: "Start".to_string(),
              value: "start".to_string(),
              version: "Office2010".to_string(),
              aliases: vec![],
            },
            SchemaEnumFacetExtension {
              name: "End".to_string(),
              value: "end".to_string(),
              version: "Office2010".to_string(),
              aliases: vec![],
            },
          ],
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &extensions).unwrap();

    assert_eq!(schemas[0].enums[0].facets.len(), 3);
    assert_eq!(schemas[0].enums[0].facets[1].name, "Start");
    assert_eq!(schemas[0].enums[0].facets[1].value, "start");
    assert_eq!(schemas[0].enums[0].facets[1].version, "Office2010");
    assert_eq!(schemas[0].enums[0].facets[2].name, "End");
    assert_eq!(schemas[0].enums[0].facets[2].value, "end");
    assert_eq!(schemas[0].enums[0].facets[2].version, "Office2010");
  }
}
