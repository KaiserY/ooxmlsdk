use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::Result;
use crate::sdk_data::sdk_data_model::{
  CompatibilityAction, CompatibilityConfig, CompatibilityRule, Schema, SchemaTypeApiKind,
  SchemaTypeKind,
};

pub fn read_compatibility(path: &Path) -> Result<CompatibilityConfig> {
  if !path.exists() {
    return Ok(CompatibilityConfig::default());
  }

  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let compatibility: CompatibilityConfig = serde_json::from_reader(reader)?;

  validate_compatibility(&compatibility)?;

  Ok(compatibility)
}

fn validate_compatibility(compatibility: &CompatibilityConfig) -> Result<()> {
  for rule in &compatibility.rules {
    if rule.schema.is_empty() {
      return Err("compatibility rule schema cannot be empty".into());
    }

    if rule.type_name.is_empty() {
      return Err("compatibility rule type cannot be empty".into());
    }

    if rule.field.is_empty() {
      return Err("compatibility rule field cannot be empty".into());
    }

    match &rule.action {
      CompatibilityAction::TreatAsString => {}
      CompatibilityAction::PreserveNamespaceDecls => {
        if rule.field != "xmlns_map" {
          return Err(
            format!(
              "compatibility rule {}.{}.{} PreserveNamespaceDecls field must be xmlns_map",
              rule.schema, rule.type_name, rule.field
            )
            .into(),
          );
        }
      }
      CompatibilityAction::MapAttributeValue { mappings } => {
        if mappings.is_empty() {
          return Err(
            format!(
              "compatibility rule {}.{}.{} MapAttributeValue requires mappings",
              rule.schema, rule.type_name, rule.field
            )
            .into(),
          );
        }

        if mappings
          .iter()
          .any(|mapping| mapping.from.is_empty() || mapping.to.is_empty())
        {
          return Err(
            format!(
              "compatibility rule {}.{}.{} MapAttributeValue entries cannot be empty",
              rule.schema, rule.type_name, rule.field
            )
            .into(),
          );
        }
      }
      CompatibilityAction::StrictBitmaskAttributes {
        radix,
        width,
        attributes,
      } => {
        if *radix != 2 && *radix != 16 {
          return Err(
            format!(
              "compatibility rule {}.{}.{} StrictBitmaskAttributes radix must be 2 or 16",
              rule.schema, rule.type_name, rule.field
            )
            .into(),
          );
        }

        if *width == 0 {
          return Err(
            format!(
              "compatibility rule {}.{}.{} StrictBitmaskAttributes width must be greater than 0",
              rule.schema, rule.type_name, rule.field
            )
            .into(),
          );
        }

        if attributes.is_empty() {
          return Err(
            format!(
              "compatibility rule {}.{}.{} StrictBitmaskAttributes requires attributes",
              rule.schema, rule.type_name, rule.field
            )
            .into(),
          );
        }

        if attributes
          .iter()
          .any(|attribute| attribute.q_name.is_empty())
        {
          return Err(
            format!(
              "compatibility rule {}.{}.{} StrictBitmaskAttributes attribute QName cannot be empty",
              rule.schema, rule.type_name, rule.field
            )
            .into(),
          );
        }
      }
      CompatibilityAction::None => {
        return Err(
          format!(
            "compatibility rule {}.{}.{} action cannot be None",
            rule.schema, rule.type_name, rule.field
          )
          .into(),
        );
      }
    }
  }

  Ok(())
}

pub fn apply_compatibility(
  sdk_data_schemas: &mut [Schema],
  compatibility: &CompatibilityConfig,
) -> Result<()> {
  for rule in &compatibility.rules {
    apply_rule(sdk_data_schemas, rule)?;
  }

  Ok(())
}

fn find_schema_type_index(schema: &Schema, type_name: &str) -> Option<usize> {
  schema
    .types
    .iter()
    .position(|item| item.class_name == type_name || item.name == type_name)
}

fn derived_base_type_index(schema: &Schema, schema_type_index: usize) -> Option<usize> {
  let schema_type = &schema.types[schema_type_index];
  let base_type_name = schema_type
    .name
    .find('/')
    .map(|index| &schema_type.name[..index + 1])?;

  if base_type_name == schema_type.name {
    return None;
  }

  find_schema_type_index(schema, base_type_name)
}

fn find_attribute_index_in_type(
  schema: &Schema,
  schema_type_index: usize,
  field: &str,
) -> Option<usize> {
  schema.types[schema_type_index]
    .attributes
    .iter()
    .position(|attr| attr.property_name == field || attr.q_name == field)
}

fn find_attribute_index_in_type_or_base(
  schema: &Schema,
  schema_type_index: usize,
  field: &str,
) -> Option<(usize, usize)> {
  let mut current_index = Some(schema_type_index);

  while let Some(index) = current_index {
    if let Some(attr_index) = find_attribute_index_in_type(schema, index, field) {
      return Some((index, attr_index));
    }

    current_index = derived_base_type_index(schema, index);
  }

  None
}

pub fn strict_bitmask_rule_for_field<'a>(
  compatibility_rules: &'a [CompatibilityRule],
  schema: &str,
  type_name: &str,
  field: &str,
) -> Option<&'a CompatibilityRule> {
  compatibility_rules.iter().find(|rule| {
    rule.schema == schema
      && rule.type_name == type_name
      && rule.field == field
      && matches!(
        rule.action,
        CompatibilityAction::StrictBitmaskAttributes { .. }
      )
  })
}

pub fn map_attribute_value_rule_for_field<'a>(
  compatibility_rules: &'a [CompatibilityRule],
  schema: &str,
  type_name: &str,
  field: &str,
) -> Option<&'a CompatibilityRule> {
  compatibility_rules.iter().find(|rule| {
    rule.schema == schema
      && rule.type_name == type_name
      && rule.field == field
      && matches!(rule.action, CompatibilityAction::MapAttributeValue { .. })
  })
}

pub fn preserve_namespace_decls_rule_for_type<'a>(
  compatibility_rules: &'a [CompatibilityRule],
  schema: &str,
  type_name: &str,
) -> Option<&'a CompatibilityRule> {
  compatibility_rules.iter().find(|rule| {
    rule.schema == schema
      && rule.type_name == type_name
      && rule.field == "xmlns_map"
      && matches!(rule.action, CompatibilityAction::PreserveNamespaceDecls)
  })
}

pub fn treat_as_string_rule_for_field<'a>(
  compatibility_rules: &'a [CompatibilityRule],
  schema: &str,
  type_name: &str,
  field: &str,
) -> Option<&'a CompatibilityRule> {
  compatibility_rules.iter().find(|rule| {
    rule.schema == schema
      && rule.type_name == type_name
      && rule.field == field
      && matches!(rule.action, CompatibilityAction::TreatAsString)
  })
}

fn apply_rule(sdk_data_schemas: &mut [Schema], rule: &CompatibilityRule) -> Result<()> {
  let schema = sdk_data_schemas
    .iter_mut()
    .find(|schema| schema.module_name == rule.schema)
    .ok_or_else(|| format!("compatibility schema {} not found", rule.schema))?;

  let schema_type_index = find_schema_type_index(schema, &rule.type_name).ok_or_else(|| {
    format!(
      "compatibility type {}.{} not found",
      rule.schema, rule.type_name
    )
  })?;

  let attribute_index =
    find_attribute_index_in_type_or_base(schema, schema_type_index, &rule.field);

  match &rule.action {
    CompatibilityAction::TreatAsString => {
      if let Some((attribute_type_index, attribute_index)) = attribute_index {
        if attribute_type_index == schema_type_index {
          let attribute = &mut schema.types[attribute_type_index].attributes[attribute_index];
          attribute.r#type = "StringValue".to_string();
        }
      } else if rule.field == "xml_content"
        && (schema.types[schema_type_index].api_kind == SchemaTypeApiKind::LeafTextWrapper
          || schema.types[schema_type_index].kind == SchemaTypeKind::LeafText)
      {
        // Leaf text wrappers derive their xml_content type during codegen.
      } else {
        return Err(
          format!(
            "compatibility field {}.{}.{} not found",
            rule.schema, rule.type_name, rule.field
          )
          .into(),
        );
      }
    }
    CompatibilityAction::MapAttributeValue { .. } => {
      if attribute_index.is_none() {
        return Err(
          format!(
            "compatibility field {}.{}.{} not found",
            rule.schema, rule.type_name, rule.field
          )
          .into(),
        );
      }
    }
    CompatibilityAction::PreserveNamespaceDecls => {}
    CompatibilityAction::StrictBitmaskAttributes { .. } => {}
    CompatibilityAction::None => {}
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::sdk_data::sdk_data_model::{SchemaType, SchemaTypeAttribute};

  #[test]
  fn treat_validator_type_as_string_rewrites_attribute_type() {
    let mut schemas = vec![Schema {
      module_name: "schemas_openxmlformats_org_wordprocessingml_2006_main".to_string(),
      types: vec![SchemaType {
        class_name: "PageSize".to_string(),
        attributes: vec![SchemaTypeAttribute {
          property_name: "Width".to_string(),
          q_name: "w:w".to_string(),
          r#type: "UInt32Value".to_string(),
          ..Default::default()
        }],
        ..Default::default()
      }],
      ..Default::default()
    }];

    let compatibility = CompatibilityConfig {
      rules: vec![CompatibilityRule {
        schema: "schemas_openxmlformats_org_wordprocessingml_2006_main".to_string(),
        type_name: "PageSize".to_string(),
        field: "Width".to_string(),
        action: CompatibilityAction::TreatAsString,
      }],
    };

    apply_compatibility(&mut schemas, &compatibility).unwrap();

    assert_eq!(schemas[0].types[0].attributes[0].r#type, "StringValue");
  }

  #[test]
  fn treat_as_string_on_derived_attribute_does_not_rewrite_base_type() {
    let mut schemas = vec![Schema {
      module_name: "schemas_openxmlformats_org_wordprocessingml_2006_main".to_string(),
      types: vec![
        SchemaType {
          name: "w:CT_NonNegativeShort/".to_string(),
          class_name: "NonNegativeShortType".to_string(),
          attributes: vec![SchemaTypeAttribute {
            property_name: "Val".to_string(),
            q_name: "w:val".to_string(),
            r#type: "Int16Value".to_string(),
            ..Default::default()
          }],
          ..Default::default()
        },
        SchemaType {
          name: "w:CT_NonNegativeShort/w:defaultTabStop".to_string(),
          class_name: "DefaultTabStop".to_string(),
          ..Default::default()
        },
      ],
      ..Default::default()
    }];

    let compatibility = CompatibilityConfig {
      rules: vec![CompatibilityRule {
        schema: "schemas_openxmlformats_org_wordprocessingml_2006_main".to_string(),
        type_name: "DefaultTabStop".to_string(),
        field: "Val".to_string(),
        action: CompatibilityAction::TreatAsString,
      }],
    };

    apply_compatibility(&mut schemas, &compatibility).unwrap();

    assert_eq!(schemas[0].types[0].attributes[0].r#type, "Int16Value");
  }

  #[test]
  fn treat_as_string_allows_leaf_text_xml_content_rule() {
    let mut schemas = vec![Schema {
      module_name: "schemas_microsoft_com_office_word_2010_wordprocessing_drawing".to_string(),
      types: vec![SchemaType {
        class_name: "PercentagePositionHeightOffset".to_string(),
        kind: SchemaTypeKind::LeafText,
        api_kind: SchemaTypeApiKind::LeafTextWrapper,
        ..Default::default()
      }],
      ..Default::default()
    }];

    let compatibility = CompatibilityConfig {
      rules: vec![CompatibilityRule {
        schema: "schemas_microsoft_com_office_word_2010_wordprocessing_drawing".to_string(),
        type_name: "PercentagePositionHeightOffset".to_string(),
        field: "xml_content".to_string(),
        action: CompatibilityAction::TreatAsString,
      }],
    };

    apply_compatibility(&mut schemas, &compatibility).unwrap();
  }

  #[test]
  fn preserve_namespace_decls_is_a_noop_for_schema_metadata() {
    let mut schemas = vec![Schema {
      module_name: "schemas_openxmlformats_org_spreadsheetml_2006_main".to_string(),
      types: vec![SchemaType {
        class_name: "WorkbookExtension".to_string(),
        attributes: vec![SchemaTypeAttribute {
          property_name: "Uri".to_string(),
          q_name: "uri".to_string(),
          r#type: "StringValue".to_string(),
          ..Default::default()
        }],
        ..Default::default()
      }],
      ..Default::default()
    }];

    let compatibility = CompatibilityConfig {
      rules: vec![CompatibilityRule {
        schema: "schemas_openxmlformats_org_spreadsheetml_2006_main".to_string(),
        type_name: "WorkbookExtension".to_string(),
        field: "xmlns_map".to_string(),
        action: CompatibilityAction::PreserveNamespaceDecls,
      }],
    };

    apply_compatibility(&mut schemas, &compatibility).unwrap();

    assert_eq!(schemas[0].types[0].attributes[0].r#type, "StringValue");
  }
}
