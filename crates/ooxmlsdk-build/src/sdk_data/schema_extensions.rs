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
  pub clone_from_class_name: String,
  pub name: String,
  pub has_xmlns_fields: Option<bool>,
  pub has_mc_ignorable_field: Option<bool>,
  pub text_value_type: Option<String>,
  #[serde(default)]
  pub mc_alternate_content_insert_before: Vec<String>,
  #[serde(default)]
  pub attributes: Vec<SchemaTypeAttribute>,
  #[serde(default)]
  pub children: Vec<SchemaTypeChildExtension>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeChildExtension {
  pub kind: SchemaTypeChildKind,
  pub match_particle_id: String,
  pub name: String,
  pub property_name: String,
  #[serde(default)]
  pub optional: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub repeated: Option<bool>,
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
      let schema_type_index = if let Some(index) = schema
        .types
        .iter()
        .position(|schema_type| schema_type.class_name == extension.class_name)
      {
        index
      } else if !extension.clone_from_class_name.is_empty() {
        let Some(source_index) = schema
          .types
          .iter()
          .position(|schema_type| schema_type.class_name == extension.clone_from_class_name)
        else {
          return Err(
            format!(
              "schema extension source type {}.{} not found",
              module_name, extension.clone_from_class_name
            )
            .into(),
          );
        };

        let mut cloned = schema.types[source_index].clone();
        cloned.class_name = extension.class_name.clone();
        if !extension.name.is_empty() {
          cloned.name = extension.name.clone();
        }

        schema.types.insert(source_index + 1, cloned);
        source_index + 1
      } else {
        return Err(
          format!(
            "schema extension type {}.{} not found",
            module_name, extension.class_name
          )
          .into(),
        );
      };

      let schema_type = &mut schema.types[schema_type_index];

      if !extension.name.is_empty() {
        schema_type.name = extension.name.clone();
      }

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
      let mut expanded_children =
        expand_mc_alternate_content_insert_before(&extension.mc_alternate_content_insert_before);
      expanded_children.extend(extension.children.iter().cloned());
      merge_schema_type_children(&mut schema_type.children, &expanded_children);
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

fn expand_mc_alternate_content_insert_before(
  insert_before: &[String],
) -> Vec<SchemaTypeChildExtension> {
  insert_before
    .iter()
    .map(|insert_before| SchemaTypeChildExtension {
      kind: SchemaTypeChildKind::Child,
      match_particle_id: String::new(),
      name: "mc:CT_AlternateContent/mc:AlternateContent".to_string(),
      property_name: "mc_alternate_content".to_string(),
      optional: true,
      repeated: None,
      insert_before: Some(insert_before.clone()),
      children: Vec::new(),
    })
    .collect()
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
        || (attribute.q_name.contains(':')
          && attribute.property_name.is_empty()
          && attribute_local_name(existing.q_name.as_str())
            == attribute_local_name(attribute.q_name.as_str()))
    }) {
      merge_schema_type_attribute(existing, attribute);
    } else {
      target.push(attribute.clone());
    }
  }
}

fn attribute_local_name(qname: &str) -> &str {
  qname.rsplit(':').next().unwrap_or(qname)
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
  let target = top_level_extension_children(target);
  for extension in extensions {
    if let Some(target_child) = find_merge_target(target, extension) {
      merge_schema_type_child(
        target_child,
        extension,
        extension.match_particle_id.is_empty(),
      );
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

fn top_level_extension_children(target: &mut Vec<SchemaTypeChild>) -> &mut Vec<SchemaTypeChild> {
  if target.len() == 1 && target[0].kind == SchemaTypeChildKind::Sequence {
    &mut target[0].children
  } else {
    target
  }
}

fn merge_schema_type_child(
  target: &mut SchemaTypeChild,
  extension: &SchemaTypeChildExtension,
  allow_property_name_override: bool,
) {
  if !extension.name.is_empty() {
    target.name = extension.name.clone();
  }
  if allow_property_name_override && !extension.property_name.is_empty() {
    target.property_name = extension.property_name.clone();
  }
  target.kind = extension.kind;
  target.optional = extension.optional;
  if let Some(repeated) = extension.repeated {
    target.repeated = repeated;
  }

  merge_schema_type_children(&mut target.children, &extension.children);
}

fn find_merge_target<'a>(
  target: &'a mut [SchemaTypeChild],
  extension: &SchemaTypeChildExtension,
) -> Option<&'a mut SchemaTypeChild> {
  if !extension.match_particle_id.is_empty()
    && let Some(index) = target
      .iter()
      .position(|child| child.particle_id == extension.match_particle_id)
  {
    return target.get_mut(index);
  }

  find_merge_target_legacy(target, extension)
}

fn find_merge_target_legacy<'a>(
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
    child.particle_id == insert_before
      || child.name == insert_before
      || child.name.ends_with(&format!("/{insert_before}"))
      || child.property_name == insert_before
      || (insert_before == "unknown_xml" && child.kind == SchemaTypeChildKind::Any)
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
    particle_id: String::new(),
    name: extension.name.clone(),
    property_name: extension.property_name.clone(),
    property_comments: String::new(),
    kind: extension.kind,
    optional: extension.optional,
    repeated: extension.repeated.unwrap_or(false),
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
    Schema, SchemaType, SchemaTypeAttribute, SchemaTypeChild, SchemaTypeChildKind,
  };

  #[test]
  fn apply_schema_extensions_overrides_type_name_and_attribute_qname_by_local_name() {
    let mut schemas = vec![Schema {
      module_name: "test_module".to_string(),
      types: vec![SchemaType {
        class_name: "SymEx".to_string(),
        name: "w16se:CT_SymEx/w16se:symEx".to_string(),
        attributes: vec![
          SchemaTypeAttribute {
            q_name: "w16se:font".to_string(),
            property_name: "w16se_font".to_string(),
            r#type: "StringValue".to_string(),
            ..Default::default()
          },
          SchemaTypeAttribute {
            q_name: "w16se:char".to_string(),
            property_name: "w16se_char".to_string(),
            r#type: "HexBinaryValue".to_string(),
            ..Default::default()
          },
        ],
        ..Default::default()
      }],
      ..Default::default()
    }];
    let schema_extensions = vec![(
      "test_module".to_string(),
      SchemaExtensions {
        types: vec![SchemaTypeExtension {
          class_name: "SymEx".to_string(),
          name: "w16se:CT_SymEx/w16se:sym".to_string(),
          attributes: vec![
            SchemaTypeAttribute {
              q_name: "w:font".to_string(),
              ..Default::default()
            },
            SchemaTypeAttribute {
              q_name: "w:char".to_string(),
              ..Default::default()
            },
          ],
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &schema_extensions).unwrap();

    let sym_ex = &schemas[0].types[0];
    assert_eq!(sym_ex.name, "w16se:CT_SymEx/w16se:sym");
    assert_eq!(sym_ex.attributes.len(), 2);
    assert_eq!(sym_ex.attributes[0].q_name, "w:font");
    assert_eq!(sym_ex.attributes[0].property_name, "w16se_font");
    assert_eq!(sym_ex.attributes[0].r#type, "StringValue");
    assert_eq!(sym_ex.attributes[1].q_name, "w:char");
    assert_eq!(sym_ex.attributes[1].property_name, "w16se_char");
    assert_eq!(sym_ex.attributes[1].r#type, "HexBinaryValue");
  }

  #[test]
  fn apply_schema_extensions_can_append_child_inside_existing_choice() {
    let mut schemas = vec![Schema {
      module_name: "test_module".to_string(),
      types: vec![SchemaType {
        class_name: "Run".to_string(),
        children: vec![SchemaTypeChild {
          particle_id: "sequence_1/choice_1".to_string(),
          kind: SchemaTypeChildKind::Choice,
          children: vec![SchemaTypeChild {
            particle_id: "sequence_1/choice_1/text_child_1".to_string(),
            kind: SchemaTypeChildKind::TextChild,
            ..Default::default()
          }],
          ..Default::default()
        }],
        ..Default::default()
      }],
      ..Default::default()
    }];
    let schema_extensions = vec![(
      "test_module".to_string(),
      SchemaExtensions {
        types: vec![SchemaTypeExtension {
          class_name: "Run".to_string(),
          children: vec![SchemaTypeChildExtension {
            kind: SchemaTypeChildKind::Choice,
            match_particle_id: "sequence_1/choice_1".to_string(),
            children: vec![SchemaTypeChildExtension {
              kind: SchemaTypeChildKind::Child,
              name: "w16se:CT_SymEx/w16se:sym".to_string(),
              ..Default::default()
            }],
            ..Default::default()
          }],
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &schema_extensions).unwrap();

    let run = &schemas[0].types[0];
    assert_eq!(run.children.len(), 1);
    assert_eq!(run.children[0].kind, SchemaTypeChildKind::Choice);
    assert_eq!(run.children[0].children.len(), 2);
    assert!(
      run.children[0]
        .children
        .iter()
        .any(|child| child.name == "w16se:CT_SymEx/w16se:sym")
    );
  }

  #[test]
  fn apply_schema_extensions_can_clone_existing_type_and_insert_child() {
    let mut schemas = vec![Schema {
      module_name: "test_module".to_string(),
      types: vec![
        SchemaType {
          class_name: "OnOffType".to_string(),
          name: "w:CT_OnOff/".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          attributes: vec![SchemaTypeAttribute {
            q_name: "w:val".to_string(),
            property_name: "val".to_string(),
            r#type: "OnOffValue".to_string(),
            ..Default::default()
          }],
          ..Default::default()
        },
        SchemaType {
          class_name: "Settings".to_string(),
          children: vec![SchemaTypeChild {
            name: "w:CT_String/w:decimalSymbol".to_string(),
            property_name: "w_decimal_symbol".to_string(),
            kind: SchemaTypeChildKind::Child,
            optional: true,
            ..Default::default()
          }],
          ..Default::default()
        },
      ],
      ..Default::default()
    }];
    let schema_extensions = vec![(
      "test_module".to_string(),
      SchemaExtensions {
        types: vec![
          SchemaTypeExtension {
            class_name: "DoNotEmbedSmartTags".to_string(),
            clone_from_class_name: "OnOffType".to_string(),
            name: "w:CT_OnOff/w:doNotEmbedSmartTags".to_string(),
            ..Default::default()
          },
          SchemaTypeExtension {
            class_name: "Settings".to_string(),
            children: vec![SchemaTypeChildExtension {
              kind: SchemaTypeChildKind::Child,
              name: "w:CT_OnOff/w:doNotEmbedSmartTags".to_string(),
              property_name: "w_do_not_embed_smart_tags".to_string(),
              optional: true,
              insert_before: Some("w:CT_String/w:decimalSymbol".to_string()),
              ..Default::default()
            }],
            ..Default::default()
          },
        ],
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &schema_extensions).unwrap();

    assert_eq!(schemas[0].types[1].class_name, "DoNotEmbedSmartTags");
    assert_eq!(schemas[0].types[1].name, "w:CT_OnOff/w:doNotEmbedSmartTags");
    assert_eq!(schemas[0].types[1].attributes.len(), 1);

    let settings = schemas[0]
      .types
      .iter()
      .find(|schema_type| schema_type.class_name == "Settings")
      .unwrap();
    assert_eq!(settings.children.len(), 2);
    assert_eq!(
      settings.children[0].name,
      "w:CT_OnOff/w:doNotEmbedSmartTags"
    );
    assert_eq!(
      settings.children[0].property_name,
      "w_do_not_embed_smart_tags"
    );
    assert_eq!(settings.children[1].name, "w:CT_String/w:decimalSymbol");
  }
}
