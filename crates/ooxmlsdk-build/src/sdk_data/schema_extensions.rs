use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::Result;
use crate::sdk_code::codegen_ir::{
  MemberDecl, SchemaModuleDecl, TypeKind, TypeRefDecl, VariantDecl, VariantWireDecl,
};
use crate::sdk_data::sdk_data_model::Schema;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaExtensions {
  pub enums: Vec<SchemaEnumExtension>,
  pub types: Vec<SchemaTypeExtension>,
  pub choice_enums: Vec<SchemaChoiceEnumExtension>,
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
  #[serde(skip_serializing_if = "String::is_empty")]
  pub override_name: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub override_base_class: String,
  pub have_xmlns_fields: Option<bool>,
  pub have_xml_other_attrs: Option<bool>,
  pub have_xml_other_children: Option<bool>,
  pub have_direct_xml_other_children: Option<bool>,
  pub attributes: Vec<SchemaTypeAttributeExtension>,
  pub children: Vec<SchemaTypeChildExtension>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeAttributeExtension {
  pub q_name: String,
  pub property_name: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub override_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeChildExtension {
  pub name: String,
  pub property_name: String,
  pub optional: Option<bool>,
  pub repeated: Option<bool>,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub override_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaChoiceEnumExtension {
  pub rust_name: String,
  #[serde(skip_serializing_if = "is_false")]
  pub add_xml_any: bool,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub add_variants: Vec<SchemaChoiceVariantExtension>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaChoiceVariantExtension {
  pub rust_name: String,
  #[serde(rename = "QName")]
  pub q_name: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub docs: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub version: String,
  pub payload_rust_type: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub payload_module_path: String,
}

fn is_false(value: &bool) -> bool {
  !*value
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
      if !extension.override_name.is_empty() {
        schema_type.name = extension.override_name.clone();
      }
      if !extension.override_base_class.is_empty() {
        schema_type.base_class = extension.override_base_class.clone();
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

      for attr_extension in &extension.attributes {
        let Some(attr) = schema_type.attributes.iter_mut().find(|attr| {
          (!attr_extension.q_name.is_empty() && attr.q_name == attr_extension.q_name)
            || (!attr_extension.property_name.is_empty()
              && attr.property_name == attr_extension.property_name)
        }) else {
          return Err(
            format!(
              "schema extension attribute {}.{} not found",
              module_name, extension.class_name
            )
            .into(),
          );
        };

        if !attr_extension.override_type.is_empty() {
          attr.r#type = attr_extension.override_type.clone();
        }
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
        if let Some(repeated) = child_extension.repeated {
          child.repeated = repeated;
        }
        if !child_extension.override_name.is_empty() {
          child.name = child_extension.override_name.clone();
        }
      }
    }
  }

  Ok(())
}

pub fn apply_codegen_ir_schema_extensions(
  module_name: &str,
  ir: &mut SchemaModuleDecl,
  schema_extensions: &[(String, SchemaExtensions)],
) -> Result<()> {
  let Some((_, extensions)) = schema_extensions
    .iter()
    .find(|(extension_module_name, _)| extension_module_name == module_name)
  else {
    return Ok(());
  };

  for choice_extension in &extensions.choice_enums {
    let Some(type_decl) = ir
      .types
      .iter_mut()
      .find(|type_decl| type_decl.rust_name == choice_extension.rust_name)
    else {
      return Err(
        format!(
          "schema extension choice enum {}.{} not found",
          module_name, choice_extension.rust_name
        )
        .into(),
      );
    };

    if type_decl.kind != TypeKind::ChoiceEnum {
      return Err(
        format!(
          "schema extension choice enum {}.{} targets {:?}",
          module_name, choice_extension.rust_name, type_decl.kind
        )
        .into(),
      );
    }

    if choice_extension.add_xml_any {
      add_xml_any_choice_variant(type_decl);
    }

    for variant_extension in &choice_extension.add_variants {
      if variant_extension.rust_name.is_empty()
        || variant_extension.q_name.is_empty()
        || variant_extension.payload_rust_type.is_empty()
      {
        return Err(
          format!(
            "schema extension choice enum {}.{} has incomplete add variant",
            module_name, choice_extension.rust_name
          )
          .into(),
        );
      }

      if type_decl.members.iter().any(|member| {
        matches!(
          member,
          MemberDecl::Variant(variant) if variant.rust_name == variant_extension.rust_name
        )
      }) {
        return Err(
          format!(
            "schema extension choice enum {}.{} already has variant {}",
            module_name, choice_extension.rust_name, variant_extension.rust_name
          )
          .into(),
        );
      }

      if type_decl.members.iter().any(|member| {
        matches!(
          member,
          MemberDecl::Variant(variant)
            if variant_wire_qnames(&variant.wire)
              .is_some_and(|qnames| qnames.iter().any(|qname| qname == &variant_extension.q_name))
        )
      }) {
        return Err(
          format!(
            "schema extension choice enum {}.{} already has child {}",
            module_name, choice_extension.rust_name, variant_extension.q_name
          )
          .into(),
        );
      }

      type_decl.members.push(MemberDecl::Variant(VariantDecl {
        rust_name: variant_extension.rust_name.clone(),
        docs: variant_extension.docs.clone(),
        version: variant_extension.version.clone(),
        wire: VariantWireDecl::Child {
          qnames: vec![variant_extension.q_name.clone()],
        },
        payload: TypeRefDecl {
          rust_type: variant_extension.payload_rust_type.clone(),
          module_path: (!variant_extension.payload_module_path.is_empty())
            .then(|| variant_extension.payload_module_path.clone()),
        },
      }));
    }
  }

  Ok(())
}

fn add_xml_any_choice_variant(type_decl: &mut crate::sdk_code::codegen_ir::TypeDecl) {
  if type_decl.members.iter().any(|member| {
    matches!(
      member,
      MemberDecl::Variant(variant) if variant.rust_name == "XmlAny"
    )
  }) {
    return;
  }

  type_decl.members.push(MemberDecl::Variant(VariantDecl {
    rust_name: "XmlAny".to_string(),
    docs: "Unknown XML child.".to_string(),
    version: String::new(),
    wire: VariantWireDecl::Any,
    payload: TypeRefDecl {
      rust_type: "std::boxed::Box<[u8]>".to_string(),
      module_path: None,
    },
  }));
}

fn variant_wire_qnames(wire: &VariantWireDecl) -> Option<&[String]> {
  match wire {
    VariantWireDecl::Child { qnames }
    | VariantWireDecl::Sequence { qnames }
    | VariantWireDecl::TextChild { qnames } => Some(qnames),
    VariantWireDecl::Any | VariantWireDecl::Text => None,
  }
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
  use crate::sdk_code::codegen_ir::{SchemaModuleDecl, TypeDecl};
  use crate::sdk_data::sdk_data_model::{
    SchemaEnum, SchemaEnumFacet, SchemaType, SchemaTypeAttribute, SchemaTypeChild,
  };

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
  }

  #[test]
  fn applies_child_repeated_extension_by_name() {
    let mut schemas = vec![Schema {
      module_name: "test_schema".to_string(),
      types: vec![SchemaType {
        class_name: "Parent".to_string(),
        children: vec![SchemaTypeChild {
          name: "t:CT_Choice/t:choice".to_string(),
          repeated: false,
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
          children: vec![SchemaTypeChildExtension {
            name: "t:CT_Choice/t:choice".to_string(),
            repeated: Some(true),
            ..Default::default()
          }],
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &extensions).unwrap();

    assert!(schemas[0].types[0].children[0].repeated);
  }

  #[test]
  fn applies_child_override_name_extension_by_property_name() {
    let mut schemas = vec![Schema {
      module_name: "test_schema".to_string(),
      types: vec![SchemaType {
        class_name: "Parent".to_string(),
        children: vec![SchemaTypeChild {
          name: "t:CT_Old/t:child".to_string(),
          property_name: "Child".to_string(),
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
          children: vec![SchemaTypeChildExtension {
            property_name: "Child".to_string(),
            override_name: "t:CT_New/t:child".to_string(),
            ..Default::default()
          }],
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &extensions).unwrap();

    assert_eq!(schemas[0].types[0].children[0].name, "t:CT_New/t:child");
    assert_eq!(schemas[0].types[0].children[0].property_name, "Child");
  }

  #[test]
  fn applies_type_override_name_and_base_class_extension() {
    let mut schemas = vec![Schema {
      module_name: "test_schema".to_string(),
      types: vec![SchemaType {
        name: "t:CT_Old/t:item".to_string(),
        class_name: "Item".to_string(),
        base_class: "OldBase".to_string(),
        ..Default::default()
      }],
      ..Default::default()
    }];
    let extensions = vec![(
      "test_schema".to_string(),
      SchemaExtensions {
        types: vec![SchemaTypeExtension {
          class_name: "Item".to_string(),
          override_name: "t:CT_New/t:item".to_string(),
          override_base_class: "NewBase".to_string(),
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &extensions).unwrap();

    assert_eq!(schemas[0].types[0].name, "t:CT_New/t:item");
    assert_eq!(schemas[0].types[0].base_class, "NewBase");
  }

  #[test]
  fn applies_attribute_override_type_extension_by_property_name() {
    let mut schemas = vec![Schema {
      module_name: "test_schema".to_string(),
      types: vec![SchemaType {
        class_name: "DocGrid".to_string(),
        attributes: vec![SchemaTypeAttribute {
          q_name: "w:charSpace".to_string(),
          property_name: "CharacterSpace".to_string(),
          r#type: "Int32Value".to_string(),
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
          class_name: "DocGrid".to_string(),
          attributes: vec![SchemaTypeAttributeExtension {
            property_name: "CharacterSpace".to_string(),
            override_type: "Int32ZeroOnOverflowValue".to_string(),
            ..Default::default()
          }],
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &extensions).unwrap();

    assert_eq!(
      schemas[0].types[0].attributes[0].r#type,
      "Int32ZeroOnOverflowValue"
    );
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

  #[test]
  fn applies_choice_enum_add_variant_extension() {
    let mut ir = SchemaModuleDecl {
      module_name: "test_schema".to_string(),
      types: vec![TypeDecl {
        rust_name: "ControlPropertiesChoice".to_string(),
        kind: TypeKind::ChoiceEnum,
        members: vec![MemberDecl::Variant(VariantDecl {
          rust_name: "RunProperties".to_string(),
          wire: VariantWireDecl::Child {
            qnames: vec!["w:CT_RPr/w:rPr".to_string()],
          },
          payload: TypeRefDecl {
            rust_type: "RunProperties".to_string(),
            module_path: Some("crate::schemas::w".to_string()),
          },
          ..Default::default()
        })],
        ..Default::default()
      }],
      ..Default::default()
    };
    let extensions = vec![(
      "test_schema".to_string(),
      SchemaExtensions {
        choice_enums: vec![SchemaChoiceEnumExtension {
          rust_name: "ControlPropertiesChoice".to_string(),
          add_xml_any: false,
          add_variants: vec![SchemaChoiceVariantExtension {
            rust_name: "DrawingRunProperties".to_string(),
            q_name: "a:CT_TextCharacterProperties/a:rPr".to_string(),
            payload_rust_type: "RunProperties".to_string(),
            payload_module_path: "crate::schemas::a".to_string(),
            ..Default::default()
          }],
        }],
        ..Default::default()
      },
    )];

    apply_codegen_ir_schema_extensions("test_schema", &mut ir, &extensions).unwrap();

    let choice = &ir.types[0];
    assert_eq!(choice.members.len(), 2);
    let MemberDecl::Variant(variant) = &choice.members[1] else {
      panic!("expected variant");
    };
    assert_eq!(variant.rust_name, "DrawingRunProperties");
    assert_eq!(
      variant.wire,
      VariantWireDecl::Child {
        qnames: vec!["a:CT_TextCharacterProperties/a:rPr".to_string()]
      }
    );
    assert_eq!(variant.payload.rust_type, "RunProperties");
    assert_eq!(
      variant.payload.module_path.as_deref(),
      Some("crate::schemas::a")
    );
  }

  #[test]
  fn applies_choice_enum_add_xml_any_extension() {
    let mut ir = SchemaModuleDecl {
      module_name: "test_schema".to_string(),
      types: vec![TypeDecl {
        rust_name: "RunChoice".to_string(),
        kind: TypeKind::ChoiceEnum,
        members: vec![MemberDecl::Variant(VariantDecl {
          rust_name: "Text".to_string(),
          wire: VariantWireDecl::Child {
            qnames: vec!["w:CT_Text/w:t".to_string()],
          },
          payload: TypeRefDecl {
            rust_type: "Text".to_string(),
            module_path: None,
          },
          ..Default::default()
        })],
        ..Default::default()
      }],
      ..Default::default()
    };
    let extensions = vec![(
      "test_schema".to_string(),
      SchemaExtensions {
        choice_enums: vec![SchemaChoiceEnumExtension {
          rust_name: "RunChoice".to_string(),
          add_xml_any: true,
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_codegen_ir_schema_extensions("test_schema", &mut ir, &extensions).unwrap();

    let choice = &ir.types[0];
    assert_eq!(choice.members.len(), 2);
    let MemberDecl::Variant(variant) = &choice.members[1] else {
      panic!("expected variant");
    };
    assert_eq!(variant.rust_name, "XmlAny");
    assert_eq!(variant.wire, VariantWireDecl::Any);
    assert_eq!(variant.payload.rust_type, "std::boxed::Box<[u8]>");
    assert!(variant.payload.module_path.is_none());
  }
}
