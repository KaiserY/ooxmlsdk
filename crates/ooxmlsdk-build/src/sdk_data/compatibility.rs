use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::Result;
use crate::sdk_data::sdk_data_model::{
  CompatibilityAction, CompatibilityConfig, CompatibilityRule, Schema, SchemaType,
  SchemaTypeApiKind, SchemaTypeChild, SchemaTypeChildKind, SchemaTypeCompositeKind, SchemaTypeKind,
  SchemaTypeParticle, SchemaTypeParticleOccur,
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
      CompatibilityAction::FallbackToRawXml => {}
      CompatibilityAction::TextChoice => {}
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
      CompatibilityAction::CollectionSequenceRoot => {}
      CompatibilityAction::ExtraChild => {}
      CompatibilityAction::AlternateContentChoice => {}
      CompatibilityAction::AddAttribute {
        q_name,
        r#type,
        property_comments,
        ..
      } => {
        if q_name.is_empty() {
          return Err(
            format!(
              "compatibility rule {}.{}.{} AddAttribute QName cannot be empty",
              rule.schema, rule.type_name, rule.field
            )
            .into(),
          );
        }

        if r#type.is_empty() {
          return Err(
            format!(
              "compatibility rule {}.{}.{} AddAttribute type cannot be empty",
              rule.schema, rule.type_name, rule.field
            )
            .into(),
          );
        }

        if property_comments.is_empty() {
          return Err(
            format!(
              "compatibility rule {}.{}.{} AddAttribute property comments cannot be empty",
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

fn find_child_type_index_by_qname(
  sdk_data_schemas: &[Schema],
  field: &str,
) -> Option<(usize, usize)> {
  let field_suffix = field.rsplit(':').next().unwrap_or(field);
  let field_tail = format!("/{field}");
  let field_suffix_tail = format!("/{field_suffix}");

  sdk_data_schemas
    .iter()
    .enumerate()
    .find_map(|(schema_index, schema)| {
      schema
        .types
        .iter()
        .enumerate()
        .find_map(|(type_index, item)| {
          if item.name == field
            || item.name.ends_with(&field_tail)
            || item.name.ends_with(&field_suffix_tail)
            || item.class_name == field_suffix
          {
            Some((schema_index, type_index))
          } else {
            None
          }
        })
    })
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

pub fn preserve_namespace_decls_rule_for_schema<'a>(
  compatibility_rules: &'a [CompatibilityRule],
  schema: &str,
) -> Option<&'a CompatibilityRule> {
  compatibility_rules.iter().find(|rule| {
    rule.schema == schema
      && rule.type_name == "*"
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

pub fn fallback_to_raw_xml_rule_for_field<'a>(
  compatibility_rules: &'a [CompatibilityRule],
  schema: &str,
  type_name: &str,
  field: &str,
) -> Option<&'a CompatibilityRule> {
  compatibility_rules.iter().find(|rule| {
    rule.schema == schema
      && rule.type_name == type_name
      && rule.field == field
      && matches!(rule.action, CompatibilityAction::FallbackToRawXml)
  })
}

pub fn text_choice_rule_for_field<'a>(
  compatibility_rules: &'a [CompatibilityRule],
  schema: &str,
  type_name: &str,
  field: &str,
) -> Option<&'a CompatibilityRule> {
  compatibility_rules.iter().find(|rule| {
    rule.schema == schema
      && rule.type_name == type_name
      && rule.field == field
      && matches!(rule.action, CompatibilityAction::TextChoice)
  })
}

pub fn alternate_content_choice_rule_for_field<'a>(
  compatibility_rules: &'a [CompatibilityRule],
  schema: &str,
  type_name: &str,
  field: &str,
) -> Option<&'a CompatibilityRule> {
  compatibility_rules.iter().find(|rule| {
    rule.schema == schema
      && rule.type_name == type_name
      && rule.field == field
      && matches!(rule.action, CompatibilityAction::AlternateContentChoice)
  })
}

pub fn alternate_content_choice_rule_for_child<'a>(
  compatibility_rules: &'a [CompatibilityRule],
  schema: &str,
  type_name: &str,
  child_qname: &str,
) -> Option<&'a CompatibilityRule> {
  compatibility_rules.iter().find(|rule| {
    rule.schema == schema
      && rule.type_name == type_name
      && rule.field == child_qname
      && matches!(rule.action, CompatibilityAction::AlternateContentChoice)
  })
}

fn apply_rule(sdk_data_schemas: &mut [Schema], rule: &CompatibilityRule) -> Result<()> {
  let schema_index = sdk_data_schemas
    .iter()
    .position(|schema| schema.module_name == rule.schema)
    .ok_or_else(|| format!("compatibility schema {} not found", rule.schema))?;

  if matches!(rule.action, CompatibilityAction::PreserveNamespaceDecls) && rule.type_name == "*" {
    let schema = &mut sdk_data_schemas[schema_index];
    for schema_type in &mut schema.types {
      schema_type.has_xmlns_fields = true;
    }
    return Ok(());
  }

  let schema_type_index = {
    let schema = &sdk_data_schemas[schema_index];
    find_schema_type_index(schema, &rule.type_name).ok_or_else(|| {
      format!(
        "compatibility type {}.{} not found",
        rule.schema, rule.type_name
      )
    })?
  };

  let attribute_index = {
    let schema = &sdk_data_schemas[schema_index];
    find_attribute_index_in_type_or_base(schema, schema_type_index, &rule.field)
  };

  match &rule.action {
    CompatibilityAction::TreatAsString => {
      if let Some((attribute_type_index, attribute_index)) = attribute_index {
        if attribute_type_index == schema_type_index {
          let schema = &mut sdk_data_schemas[schema_index];
          let attribute = &mut schema.types[attribute_type_index].attributes[attribute_index];
          attribute.r#type = "StringValue".to_string();
        }
      } else {
        let schema = &sdk_data_schemas[schema_index];
        if rule.field == "xml_content"
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
    }
    CompatibilityAction::FallbackToRawXml => {}
    CompatibilityAction::TextChoice => {
      let schema = &mut sdk_data_schemas[schema_index];
      if let Some(child) = schema.types[schema_type_index]
        .children
        .iter_mut()
        .find(|child| child.kind == SchemaTypeChildKind::Choice)
      {
        child.property_name = rule.field.clone();
      }
    }
    CompatibilityAction::CollectionSequenceRoot => {
      let schema = &mut sdk_data_schemas[schema_index];
      schema.types[schema_type_index].collection_sequence_root = true;
    }
    CompatibilityAction::AlternateContentChoice => {
      let schema = &mut sdk_data_schemas[schema_index];
      let schema_type = &mut schema.types[schema_type_index];
      ensure_alternate_content_child(schema_type, &rule.field);
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
    CompatibilityAction::AddAttribute {
      q_name,
      r#type,
      property_comments,
      required,
    } => {
      let schema = &mut sdk_data_schemas[schema_index];
      let schema_type = &mut schema.types[schema_type_index];

      if schema_type
        .attributes
        .iter()
        .any(|attr| attr.q_name == *q_name || attr.property_name == rule.field)
      {
        return Ok(());
      }

      schema_type
        .attributes
        .push(crate::sdk_data::sdk_data_model::SchemaTypeAttribute {
          q_name: q_name.clone(),
          property_name: rule.field.clone(),
          r#type: r#type.clone(),
          property_comments: property_comments.clone(),
          version: "Office2007".to_string(),
          validators: if *required {
            vec![
              crate::sdk_data::sdk_data_model::SchemaTypeAttributeValidator {
                name: "RequiredValidator".to_string(),
                is_list: false,
                r#type: String::new(),
                union_id: 0,
                is_initial_version: false,
                arguments: Vec::new(),
              },
            ]
          } else {
            Vec::new()
          },
        });
    }
    CompatibilityAction::PreserveNamespaceDecls => {
      let schema = &mut sdk_data_schemas[schema_index];
      schema.types[schema_type_index].has_xmlns_fields = true;
    }
    CompatibilityAction::ExtraChild => {
      let (child_schema_index, child_type_index) =
        find_child_type_index_by_qname(sdk_data_schemas, &rule.field).ok_or_else(|| {
          format!(
            "compatibility child {}.{}.{} not found",
            rule.schema, rule.type_name, rule.field
          )
        })?;
      let child_schema = &sdk_data_schemas[child_schema_index];
      let child_name = child_schema.types[child_type_index].name.clone();
      let child_property_comments = child_schema.types[child_type_index].summary.clone();
      let child_version = child_schema.types[child_type_index].version.clone();
      let child_name_for_particle = child_name.clone();
      let target_schema = &mut sdk_data_schemas[schema_index];

      if target_schema.types[schema_type_index]
        .children
        .iter()
        .any(|child| child.name == child_name)
      {
        return Ok(());
      }

      target_schema.types[schema_type_index].children.push(
        crate::sdk_data::sdk_data_model::SchemaTypeChild {
          name: child_name,
          property_name: String::new(),
          property_comments: child_property_comments,
          kind: SchemaTypeChildKind::Child,
          optional: true,
          repeated: false,
          initial_version: String::new(),
          children: Vec::new(),
        },
      );

      if target_schema.types[schema_type_index].particle.kind == "Sequence"
        || matches!(
          target_schema.types[schema_type_index].composite_kind,
          SchemaTypeCompositeKind::OneSequence | SchemaTypeCompositeKind::SdkSequence
        )
      {
        target_schema.types[schema_type_index]
          .particle
          .items
          .push(SchemaTypeParticle {
            kind: String::new(),
            name: child_name_for_particle,
            occurs: vec![SchemaTypeParticleOccur {
              max: 1,
              min: 0,
              include_version: false,
              version: String::new(),
            }],
            items: vec![],
            initial_version: child_version,
            require_filter: false,
            namespace: String::new(),
          });
      }
    }
    CompatibilityAction::StrictBitmaskAttributes { .. } => {}
    CompatibilityAction::None => {}
  }

  Ok(())
}

fn ensure_alternate_content_child(schema_type: &mut SchemaType, child_qname: &str) {
  if schema_type.children.iter().any(|child| {
    child.name == "mc:CT_AlternateContent/mc:AlternateContent"
      || child.property_name == "mc_alternate_content"
  }) {
    return;
  }

  let alternate_content_child = crate::sdk_data::sdk_data_model::SchemaTypeChild {
    name: "mc:CT_AlternateContent/mc:AlternateContent".to_string(),
    property_name: "mc_alternate_content".to_string(),
    property_comments: "Preserves the mc:AlternateContent wrapper for this child.".to_string(),
    kind: SchemaTypeChildKind::Child,
    optional: true,
    repeated: false,
    initial_version: String::new(),
    children: Vec::new(),
  };

  if let Some(index) = schema_type.children.iter().position(|child| {
    child.name == child_qname
      || child.name.ends_with(&format!("/{child_qname}"))
      || child.property_name == child_qname
  }) {
    schema_type.children.insert(index, alternate_content_child);
    return;
  }

  if let Some(index) = schema_type.children.iter().position(|child| {
    child.kind == SchemaTypeChildKind::Choice && choice_child_contains_qname(child, child_qname)
  }) {
    schema_type.children.insert(index, alternate_content_child);
  } else {
    schema_type.children.push(alternate_content_child);
  }
}

fn choice_child_contains_qname(child: &SchemaTypeChild, child_qname: &str) -> bool {
  child.children.iter().any(|nested| {
    nested.name == child_qname
      || nested.name.ends_with(&format!("/{child_qname}"))
      || nested.property_name == child_qname
      || choice_child_contains_qname(nested, child_qname)
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::sdk_data::sdk_data_model::{
    SchemaType, SchemaTypeAttribute, SchemaTypeChild, SchemaTypeChildKind,
  };

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

  #[test]
  fn preserve_namespace_decls_allows_schema_wildcard() {
    let mut schemas = vec![Schema {
      module_name: "schemas_openxmlformats_org_drawingml_2006_main".to_string(),
      types: vec![SchemaType {
        class_name: "Graphic".to_string(),
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
        schema: "schemas_openxmlformats_org_drawingml_2006_main".to_string(),
        type_name: "*".to_string(),
        field: "xmlns_map".to_string(),
        action: CompatibilityAction::PreserveNamespaceDecls,
      }],
    };

    apply_compatibility(&mut schemas, &compatibility).unwrap();

    assert_eq!(schemas[0].types[0].attributes[0].r#type, "StringValue");
  }

  #[test]
  fn extra_child_appends_schema_child() {
    let mut schemas = vec![Schema {
      module_name: "schemas_openxmlformats_org_wordprocessingml_2006_main".to_string(),
      types: vec![
        SchemaType {
          name: "w:CT_CompatSetting/w:compatSetting".to_string(),
          class_name: "CompatibilitySetting".to_string(),
          summary: "Defines the CompatibilitySetting Class.".to_string(),
          ..Default::default()
        },
        SchemaType {
          name: "w:CT_Settings/w:settings".to_string(),
          class_name: "Settings".to_string(),
          ..Default::default()
        },
      ],
      ..Default::default()
    }];

    let compatibility = CompatibilityConfig {
      rules: vec![CompatibilityRule {
        schema: "schemas_openxmlformats_org_wordprocessingml_2006_main".to_string(),
        type_name: "Settings".to_string(),
        field: "w:compatSetting".to_string(),
        action: CompatibilityAction::ExtraChild,
      }],
    };

    apply_compatibility(&mut schemas, &compatibility).unwrap();

    assert_eq!(schemas[0].types[1].children.len(), 1);
    assert_eq!(
      schemas[0].types[1].children[0].name,
      "w:CT_CompatSetting/w:compatSetting"
    );
    assert_eq!(schemas[0].types[1].children[0].property_name, "");
  }

  #[test]
  fn extra_child_can_refer_to_child_type_in_another_schema() {
    let mut schemas = vec![
      Schema {
        module_name: "schemas_openxmlformats_org_wordprocessingml_2006_main".to_string(),
        types: vec![SchemaType {
          name: "w:CT_RPrBaseStyleable/w:rPr".to_string(),
          class_name: "RunPropertiesBaseStyle".to_string(),
          particle: crate::sdk_data::sdk_data_model::SchemaTypeParticle {
            kind: "Sequence".to_string(),
            ..Default::default()
          },
          ..Default::default()
        }],
        ..Default::default()
      },
      Schema {
        module_name: "schemas_microsoft_com_office_word_2010_wordml".to_string(),
        types: vec![SchemaType {
          name: "w14:CT_Ligatures/w14:ligatures".to_string(),
          class_name: "Ligatures".to_string(),
          summary: "Ligatures".to_string(),
          version: "Office2010".to_string(),
          ..Default::default()
        }],
        ..Default::default()
      },
    ];

    let compatibility = CompatibilityConfig {
      rules: vec![CompatibilityRule {
        schema: "schemas_openxmlformats_org_wordprocessingml_2006_main".to_string(),
        type_name: "RunPropertiesBaseStyle".to_string(),
        field: "w14:ligatures".to_string(),
        action: CompatibilityAction::ExtraChild,
      }],
    };

    apply_compatibility(&mut schemas, &compatibility).unwrap();

    assert_eq!(schemas[0].types[0].children.len(), 1);
    assert_eq!(
      schemas[0].types[0].children[0].name,
      "w14:CT_Ligatures/w14:ligatures"
    );
    assert_eq!(schemas[0].types[0].particle.items.len(), 1);
    assert_eq!(
      schemas[0].types[0].particle.items[0].name,
      "w14:CT_Ligatures/w14:ligatures"
    );
  }

  #[test]
  fn collection_sequence_root_marks_schema_type() {
    let mut schemas = vec![Schema {
      module_name: "schemas_openxmlformats_org_wordprocessingml_2006_main".to_string(),
      types: vec![SchemaType {
        class_name: "Footnotes".to_string(),
        ..Default::default()
      }],
      ..Default::default()
    }];

    let compatibility = CompatibilityConfig {
      rules: vec![CompatibilityRule {
        schema: "schemas_openxmlformats_org_wordprocessingml_2006_main".to_string(),
        type_name: "Footnotes".to_string(),
        field: "collection_sequence_root".to_string(),
        action: CompatibilityAction::CollectionSequenceRoot,
      }],
    };

    apply_compatibility(&mut schemas, &compatibility).unwrap();

    assert!(schemas[0].types[0].collection_sequence_root);
  }

  #[test]
  fn text_choice_renames_choice_child() {
    let mut schemas = vec![Schema {
      module_name: "schemas_openxmlformats_org_wordprocessingml_2006_main".to_string(),
      types: vec![SchemaType {
        class_name: "Run".to_string(),
        children: vec![SchemaTypeChild {
          kind: SchemaTypeChildKind::Choice,
          property_name: "children".to_string(),
          ..Default::default()
        }],
        ..Default::default()
      }],
      ..Default::default()
    }];

    let compatibility = CompatibilityConfig {
      rules: vec![CompatibilityRule {
        schema: "schemas_openxmlformats_org_wordprocessingml_2006_main".to_string(),
        type_name: "Run".to_string(),
        field: "run_choice".to_string(),
        action: CompatibilityAction::TextChoice,
      }],
    };

    apply_compatibility(&mut schemas, &compatibility).unwrap();

    assert_eq!(schemas[0].types[0].children[0].property_name, "run_choice");
  }

  #[test]
  fn alternate_content_choice_adds_mc_alternate_content_child() {
    let mut schemas = vec![Schema {
      module_name: "schemas_openxmlformats_org_wordprocessingml_2006_main".to_string(),
      types: vec![SchemaType {
        class_name: "Run".to_string(),
        ..Default::default()
      }],
      ..Default::default()
    }];

    let compatibility = CompatibilityConfig {
      rules: vec![CompatibilityRule {
        schema: "schemas_openxmlformats_org_wordprocessingml_2006_main".to_string(),
        type_name: "Run".to_string(),
        field: "w:CT_Drawing/w:drawing".to_string(),
        action: CompatibilityAction::AlternateContentChoice,
      }],
    };

    apply_compatibility(&mut schemas, &compatibility).unwrap();

    assert!(schemas[0].types[0].children.iter().any(|child| {
      child.name == "mc:CT_AlternateContent/mc:AlternateContent"
        && child.property_name == "mc_alternate_content"
        && child.kind == SchemaTypeChildKind::Child
    }));
  }
}
