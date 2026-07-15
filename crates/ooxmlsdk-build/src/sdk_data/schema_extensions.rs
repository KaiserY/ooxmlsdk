use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use heck::{ToSnakeCase, ToUpperCamelCase};
use serde::{Deserialize, Serialize};

use crate::Result;
use crate::sdk_code::codegen_ir::{
  Cardinality, FieldWireDecl, MemberDecl, SchemaModuleDecl, TypeKind, TypeRefDecl, VariantDecl,
  VariantWireDecl,
};
use crate::sdk_data::sdk_data_model::Schema;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaExtensions {
  pub enums: Vec<SchemaEnumExtension>,
  pub add_types: Vec<SchemaTypeAddTypeExtension>,
  pub types: Vec<SchemaTypeExtension>,
  pub choice_enums: Vec<SchemaChoiceEnumExtension>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaEnumExtension {
  pub name: String,
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
  pub have_mc_ignorable: Option<bool>,
  pub have_mc_preserve_attributes: Option<bool>,
  pub have_mc_preserve_elements: Option<bool>,
  pub have_mc_process_content: Option<bool>,
  pub have_mc_must_understand: Option<bool>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub extra_xmlns: Vec<String>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub canonical_namespace_prefixes: Vec<String>,
  pub attributes: Vec<SchemaTypeAttributeExtension>,
  pub children: Vec<SchemaTypeChildExtension>,
  pub add_children: Vec<SchemaTypeAddChildExtension>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub alternate_content: Vec<SchemaTypeAlternateContentExtension>,
  pub alternate_content_choice: bool,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub add_choice: Vec<SchemaChoiceVariantExtension>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeAlternateContentExtension {
  #[serde(skip_serializing_if = "String::is_empty")]
  pub before_field: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub after_field: String,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub children: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeAddTypeExtension {
  pub name: String,
  pub class_name: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub summary: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub version: Option<String>,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub base_class: String,
  pub kind: crate::sdk_data::sdk_data_model::SchemaTypeKind,
  pub composite_kind: crate::sdk_data::sdk_data_model::SchemaTypeCompositeKind,
  pub has_xml_header: bool,
  pub api_kind: crate::sdk_data::sdk_data_model::SchemaTypeApiKind,
  pub have_xmlns_fields: bool,
  pub have_mc_ignorable: bool,
  pub have_mc_preserve_attributes: bool,
  pub have_mc_preserve_elements: bool,
  pub have_mc_process_content: bool,
  pub have_mc_must_understand: bool,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub text_value_type: String,
  pub attributes: Vec<SchemaTypeAttributeExtension>,
  pub children: Vec<SchemaTypeNewChildExtension>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeAttributeExtension {
  pub q_name: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub override_q_name: String,
  pub property_name: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub property_comments: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub version: String,
  pub optional: Option<bool>,
  pub match_local_name: Option<bool>,
  pub empty_as_none: Option<bool>,
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
pub struct SchemaTypeNewChildExtension {
  pub name: String,
  pub property_name: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub property_comments: String,
  pub kind: crate::sdk_data::sdk_data_model::SchemaTypeChildKind,
  pub optional: bool,
  pub repeated: bool,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub initial_version: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaTypeAddChildExtension {
  pub name: String,
  pub property_name: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub property_comments: String,
  pub optional: Option<bool>,
  pub repeated: Option<bool>,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub initial_version: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub before_name: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub after_name: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub before_property_name: String,
  #[serde(skip_serializing_if = "String::is_empty")]
  pub after_property_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct SchemaChoiceEnumExtension {
  pub rust_name: String,
  pub repeated: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub add_xml_any: Option<Vec<String>>,
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

      for facet_extension in &extension.add_facets {
        let existing_facet = schema_enum.facets.iter_mut().find(|facet| {
          facet.name == facet_extension.name
            && facet.value == facet_extension.value
            && facet.version == facet_extension.version
        });

        if let Some(facet) = existing_facet {
          for alias in &facet_extension.aliases {
            if !facet.aliases.contains(alias) {
              facet.aliases.push(alias.clone());
            }
          }
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

    for extension in &extensions.add_types {
      add_schema_type(module_name, schema, extension)?;
    }

    for extension in &extensions.types {
      let mut type_indices: Vec<usize> = schema
        .types
        .iter()
        .enumerate()
        .filter_map(|(index, schema_type)| {
          (schema_type.class_name == extension.class_name).then_some(index)
        })
        .collect();
      if type_indices.is_empty() {
        type_indices.extend(
          schema
            .types
            .iter()
            .enumerate()
            .filter_map(|(index, schema_type)| {
              (schema_type.base_class == extension.class_name).then_some(index)
            }),
        );
      }
      if type_indices.is_empty() {
        return Err(
          format!(
            "schema extension type {}.{} not found",
            module_name, extension.class_name
          )
          .into(),
        );
      }

      for type_index in type_indices {
        let schema_type = &mut schema.types[type_index];

        if let Some(have_xmlns_fields) = extension.have_xmlns_fields {
          schema_type.have_xmlns_fields = have_xmlns_fields;
        }
        if !extension.override_name.is_empty() {
          schema_type.name = extension.override_name.clone();
        }
        if !extension.override_base_class.is_empty() {
          schema_type.base_class = extension.override_base_class.clone();
          let base_class_name = explicit_schema_base_class_name(schema_type.base_class.as_str())
            .unwrap_or(schema_type.base_class.as_str());
          match base_class_name {
            "OpenXmlLeafElement" => {
              schema_type.kind = crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf;
              schema_type.api_kind = crate::sdk_data::sdk_data_model::SchemaTypeApiKind::Struct;
            }
            "OpenXmlCompositeElement" | "OpenXmlPartRootElement" => {
              schema_type.kind = crate::sdk_data::sdk_data_model::SchemaTypeKind::Composite;
              schema_type.api_kind = crate::sdk_data::sdk_data_model::SchemaTypeApiKind::Struct;
            }
            "OpenXmlLeafTextElement" => {
              schema_type.kind = crate::sdk_data::sdk_data_model::SchemaTypeKind::LeafText;
              schema_type.api_kind =
                crate::sdk_data::sdk_data_model::SchemaTypeApiKind::LeafTextWrapper;
            }
            _ if explicit_schema_base_class_name(schema_type.base_class.as_str()).is_some() => {
              schema_type.kind = crate::sdk_data::sdk_data_model::SchemaTypeKind::Derived;
              schema_type.api_kind = crate::sdk_data::sdk_data_model::SchemaTypeApiKind::Struct;
            }
            _ => {}
          }
        }
        if let Some(have_mc_ignorable) = extension.have_mc_ignorable {
          schema_type.have_mc_ignorable = have_mc_ignorable;
        }
        if let Some(have_mc_preserve_attributes) = extension.have_mc_preserve_attributes {
          schema_type.have_mc_preserve_attributes = have_mc_preserve_attributes;
        }
        if let Some(have_mc_preserve_elements) = extension.have_mc_preserve_elements {
          schema_type.have_mc_preserve_elements = have_mc_preserve_elements;
        }
        if let Some(have_mc_process_content) = extension.have_mc_process_content {
          schema_type.have_mc_process_content = have_mc_process_content;
        }
        if let Some(have_mc_must_understand) = extension.have_mc_must_understand {
          schema_type.have_mc_must_understand = have_mc_must_understand;
        }
        for prefix in &extension.extra_xmlns {
          if !schema_type.extra_xmlns.contains(prefix) {
            schema_type.extra_xmlns.push(prefix.clone());
          }
        }
        for prefix in &extension.canonical_namespace_prefixes {
          if !schema_type.canonical_namespace_prefixes.contains(prefix) {
            schema_type
              .canonical_namespace_prefixes
              .push(prefix.clone());
          }
        }

        for attr_extension in &extension.attributes {
          let Some(attr) = schema_type.attributes.iter_mut().find(|attr| {
            (!attr_extension.q_name.is_empty() && attr.q_name == attr_extension.q_name)
              || (!attr_extension.property_name.is_empty()
                && attr.property_name == attr_extension.property_name)
          }) else {
            if attr_extension.q_name.is_empty()
              || attr_extension.property_name.is_empty()
              || attr_extension.override_type.is_empty()
            {
              return Err(
                format!(
                  "schema extension attribute {}.{} not found",
                  module_name, extension.class_name
                )
                .into(),
              );
            }

            schema_type
              .attributes
              .push(crate::sdk_data::sdk_data_model::SchemaTypeAttribute {
                q_name: attr_extension.q_name.clone(),
                property_name: attr_extension.property_name.clone(),
                r#type: attr_extension.override_type.clone(),
                property_comments: attr_extension.property_comments.clone(),
                version: attr_extension.version.clone(),
                required: !attr_extension.optional.unwrap_or(false),
                match_local_name: attr_extension.match_local_name.unwrap_or(false),
                empty_as_none: attr_extension.empty_as_none.unwrap_or(false),
                ..Default::default()
              });
            continue;
          };

          if !attr_extension.override_type.is_empty() {
            attr.r#type = attr_extension.override_type.clone();
          }
          if !attr_extension.override_q_name.is_empty() {
            attr.q_name = attr_extension.override_q_name.clone();
          }
          if !attr_extension.version.is_empty() {
            attr.version = attr_extension.version.clone();
          }
          if let Some(match_local_name) = attr_extension.match_local_name {
            attr.match_local_name = match_local_name;
          }
          if let Some(empty_as_none) = attr_extension.empty_as_none {
            attr.empty_as_none = empty_as_none;
          }
          if let Some(optional) = attr_extension.optional {
            attr.required = !optional;
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

        for child_extension in &extension.add_children {
          add_schema_type_child(
            module_name,
            &extension.class_name,
            schema_type,
            child_extension,
          )?;
        }
      }
    }
  }

  Ok(())
}

fn add_schema_type(
  module_name: &str,
  schema: &mut Schema,
  extension: &SchemaTypeAddTypeExtension,
) -> Result<()> {
  if extension.name.is_empty() || extension.class_name.is_empty() {
    return Err(format!("schema extension add type {module_name} has incomplete type").into());
  }
  if schema
    .types
    .iter()
    .any(|schema_type| schema_type.name == extension.name)
  {
    return Err(
      format!(
        "schema extension add type {}.{} already has element {}",
        module_name, extension.class_name, extension.name
      )
      .into(),
    );
  }
  if schema
    .types
    .iter()
    .any(|schema_type| schema_type.class_name == extension.class_name)
  {
    return Err(
      format!(
        "schema extension add type {}.{} already exists",
        module_name, extension.class_name
      )
      .into(),
    );
  }

  let mut attributes = Vec::with_capacity(extension.attributes.len());
  for attr in &extension.attributes {
    if attr.q_name.is_empty() || attr.property_name.is_empty() || attr.override_type.is_empty() {
      return Err(
        format!(
          "schema extension add type {}.{} has incomplete attribute",
          module_name, extension.class_name
        )
        .into(),
      );
    }
    attributes.push(crate::sdk_data::sdk_data_model::SchemaTypeAttribute {
      q_name: attr.q_name.clone(),
      property_name: attr.property_name.clone(),
      r#type: attr.override_type.clone(),
      property_comments: attr.property_comments.clone(),
      version: attr.version.clone(),
      required: !attr.optional.unwrap_or(false),
      ..Default::default()
    });
  }

  let children = extension
    .children
    .iter()
    .map(|child| crate::sdk_data::sdk_data_model::SchemaTypeChild {
      name: child.name.clone(),
      property_name: child.property_name.clone(),
      property_comments: child.property_comments.clone(),
      kind: child.kind,
      optional: child.optional,
      repeated: child.repeated,
      initial_version: child.initial_version.clone(),
      ..Default::default()
    })
    .collect();

  schema
    .types
    .push(crate::sdk_data::sdk_data_model::SchemaType {
      name: extension.name.clone(),
      class_name: extension.class_name.clone(),
      summary: extension.summary.clone(),
      version: extension.version.clone(),
      base_class: extension.base_class.clone(),
      kind: extension.kind,
      composite_kind: extension.composite_kind,
      has_xml_header: extension.has_xml_header,
      have_xmlns_fields: extension.have_xmlns_fields,
      have_mc_ignorable: extension.have_mc_ignorable,
      have_mc_preserve_attributes: extension.have_mc_preserve_attributes,
      have_mc_preserve_elements: extension.have_mc_preserve_elements,
      have_mc_process_content: extension.have_mc_process_content,
      have_mc_must_understand: extension.have_mc_must_understand,
      text_value_type: extension.text_value_type.clone(),
      api_kind: extension.api_kind.clone(),
      attributes,
      children,
      ..Default::default()
    });

  Ok(())
}

fn explicit_schema_base_class_name(base_class: &str) -> Option<&str> {
  let (module_path, rust_name) = base_class.rsplit_once("::")?;
  module_path
    .starts_with("crate::schemas::")
    .then_some(rust_name)
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

  for type_extension in &extensions.types {
    if type_extension.extra_xmlns.is_empty()
      && type_extension.canonical_namespace_prefixes.is_empty()
      && type_extension.alternate_content.is_empty()
      && !type_extension.alternate_content_choice
      && type_extension.add_choice.is_empty()
    {
      continue;
    }
    if type_extension.alternate_content_choice {
      promote_repeated_child_to_alternate_content_choice(
        module_name,
        ir,
        &type_extension.class_name,
      )?;
    }
    if !type_extension.add_choice.is_empty() {
      add_static_choice(module_name, ir, type_extension)?;
    }
    let Some(type_decl) = ir
      .types
      .iter_mut()
      .find(|type_decl| type_decl.rust_name == type_extension.class_name)
    else {
      return Err(
        format!(
          "schema extension type {}.{} not found",
          module_name, type_extension.class_name
        )
        .into(),
      );
    };

    for prefix in &type_extension.extra_xmlns {
      if !type_decl.support.extra_xmlns.contains(prefix) {
        type_decl.support.extra_xmlns.push(prefix.clone());
      }
    }
    for prefix in &type_extension.canonical_namespace_prefixes {
      if !type_decl
        .support
        .canonical_namespace_prefixes
        .contains(prefix)
      {
        type_decl
          .support
          .canonical_namespace_prefixes
          .push(prefix.clone());
      }
    }

    if !type_extension.alternate_content.is_empty() {
      add_alternate_content_fields(module_name, type_decl, type_extension)?;
    }
  }

  for choice_extension in &extensions.choice_enums {
    let Some(type_index) = ir
      .types
      .iter()
      .position(|type_decl| type_decl.rust_name == choice_extension.rust_name)
    else {
      return Err(
        format!(
          "schema extension choice enum {}.{} not found",
          module_name, choice_extension.rust_name
        )
        .into(),
      );
    };

    if ir.types[type_index].kind != TypeKind::ChoiceEnum {
      return Err(
        format!(
          "schema extension choice enum {}.{} targets {:?}",
          module_name, choice_extension.rust_name, ir.types[type_index].kind
        )
        .into(),
      );
    }

    if let Some(repeated) = choice_extension.repeated {
      apply_choice_enum_repeated(
        module_name,
        ir,
        choice_extension.rust_name.as_str(),
        repeated,
      )?;
    }

    let type_decl = &mut ir.types[type_index];

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

      let insert_index = type_decl
        .members
        .iter()
        .position(|member| matches!(member, MemberDecl::Variant(variant) if matches!(variant.wire, VariantWireDecl::Any { .. })))
        .unwrap_or(type_decl.members.len());
      type_decl.members.insert(
        insert_index,
        MemberDecl::Variant(VariantDecl {
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
        }),
      );
    }

    if let Some(qnames) = &choice_extension.add_xml_any {
      if qnames.is_empty() {
        return Err(
          format!(
            "schema extension choice enum {}.{} AddXmlAny requires at least one QName",
            module_name, choice_extension.rust_name
          )
          .into(),
        );
      }
      add_xml_any_choice_variant(type_decl, qnames.clone());
    }
  }

  Ok(())
}

fn add_xml_any_choice_variant(
  type_decl: &mut crate::sdk_code::codegen_ir::TypeDecl,
  qnames: Vec<String>,
) {
  if let Some(existing) = type_decl.members.iter_mut().find_map(|member| {
    let MemberDecl::Variant(variant) = member else {
      return None;
    };
    (variant.rust_name == "XmlAny").then_some(variant)
  }) {
    existing.wire = VariantWireDecl::Any { qnames };
    return;
  }

  type_decl.members.push(MemberDecl::Variant(VariantDecl {
    rust_name: "XmlAny".to_string(),
    docs: "Unknown XML child.".to_string(),
    version: String::new(),
    wire: VariantWireDecl::Any { qnames },
    payload: TypeRefDecl {
      rust_type: "std::boxed::Box<[u8]>".to_string(),
      module_path: None,
    },
  }));
}

fn add_static_choice(
  module_name: &str,
  ir: &mut SchemaModuleDecl,
  extension: &SchemaTypeExtension,
) -> Result<()> {
  let Some(type_index) = ir
    .types
    .iter()
    .position(|type_decl| type_decl.rust_name == extension.class_name)
  else {
    return Err(
      format!(
        "schema extension static choice {module_name}.{} not found",
        extension.class_name
      )
      .into(),
    );
  };
  if extension.add_choice.iter().any(|variant| {
    variant.rust_name.is_empty()
      || variant.q_name.is_empty()
      || variant.payload_rust_type.is_empty()
  }) {
    return Err(
      format!(
        "schema extension static choice {module_name}.{} has an incomplete variant",
        extension.class_name
      )
      .into(),
    );
  }
  let choice_name = format!("{}Choice", extension.class_name);
  let field_name = format!("{}_choice", extension.class_name.to_snake_case());
  ir.types[type_index]
    .members
    .push(MemberDecl::Field(crate::sdk_code::codegen_ir::FieldDecl {
      rust_name: field_name,
      docs: "Ordered typed XML children.".to_string(),
      version: String::new(),
      wire: FieldWireDecl::Choice,
      cardinality: Cardinality::Many,
      type_ref: TypeRefDecl {
        rust_type: choice_name.clone(),
        module_path: None,
      },
      validators: Vec::new(),
    }));
  ir.types[type_index].content_model =
    Some(crate::sdk_code::codegen_ir::ContentModelDecl::ChoiceOnly);
  ir.types[type_index].element_kind = Some(crate::sdk_code::codegen_ir::ElementKind::Composite);
  ir.types[type_index].base_rust_name = Some("OpenXmlCompositeElement".to_string());
  let version = ir.types[type_index].version.clone();
  ir.types.push(crate::sdk_code::codegen_ir::TypeDecl {
    rust_name: choice_name,
    xml_qname: None,
    docs: "Ordered typed XML children.".to_string(),
    version,
    is_abstract: false,
    kind: TypeKind::ChoiceEnum,
    element_kind: None,
    content_model: None,
    base_rust_name: None,
    base_module_path: None,
    xml_content: None,
    support: crate::sdk_code::codegen_ir::SystemSupportDecl::default(),
    estimated_size: 0,
    members: extension
      .add_choice
      .iter()
      .map(|variant| {
        MemberDecl::Variant(VariantDecl {
          rust_name: variant.rust_name.clone(),
          docs: variant.docs.clone(),
          version: variant.version.clone(),
          wire: VariantWireDecl::Child {
            qnames: vec![variant.q_name.clone()],
          },
          payload: TypeRefDecl {
            rust_type: variant.payload_rust_type.clone(),
            module_path: (!variant.payload_module_path.is_empty())
              .then(|| variant.payload_module_path.clone()),
          },
        })
      })
      .collect(),
  });
  Ok(())
}

fn promote_repeated_child_to_alternate_content_choice(
  module_name: &str,
  ir: &mut SchemaModuleDecl,
  class_name: &str,
) -> Result<()> {
  let Some(type_index) = ir
    .types
    .iter()
    .position(|type_decl| type_decl.rust_name == class_name)
  else {
    return Err(format!("schema extension type {module_name}.{class_name} not found").into());
  };
  let type_decl = &ir.types[type_index];
  let child_field_indexes = type_decl
    .members
    .iter()
    .enumerate()
    .filter_map(|(index, member)| match member {
      MemberDecl::Field(field)
        if matches!(
          field.wire,
          FieldWireDecl::Child { .. }
            | FieldWireDecl::TextChild { .. }
            | FieldWireDecl::Any
            | FieldWireDecl::Choice
        ) =>
      {
        Some(index)
      }
      _ => None,
    })
    .collect::<Vec<_>>();
  let [field_index] = child_field_indexes.as_slice() else {
    return Err(format!(
      "schema extension alternate content choice {module_name}.{class_name} requires one child field"
    )
    .into());
  };
  let MemberDecl::Field(field) = &type_decl.members[*field_index] else {
    unreachable!();
  };
  if field.cardinality != Cardinality::Many {
    return Err(format!(
      "schema extension alternate content choice {module_name}.{class_name} requires a repeated child"
    )
    .into());
  }
  let FieldWireDecl::Child { qname } = &field.wire else {
    return Err(format!(
      "schema extension alternate content choice {module_name}.{class_name} requires a typed child"
    )
    .into());
  };
  let field = field.clone();
  let qname = qname.clone();
  let choice_name = format!("{class_name}Choice");
  let known_variant_name = field
    .rust_name
    .trim_end_matches('s')
    .to_string()
    .to_upper_camel_case();
  let version = ir.types[type_index].version.clone();

  ir.types[type_index].members[*field_index] =
    MemberDecl::Field(crate::sdk_code::codegen_ir::FieldDecl {
      rust_name: "xml_children".to_string(),
      docs: field.docs.clone(),
      version: field.version.clone(),
      wire: FieldWireDecl::Choice,
      cardinality: Cardinality::Many,
      type_ref: TypeRefDecl {
        rust_type: choice_name.clone(),
        module_path: None,
      },
      validators: field.validators.clone(),
    });
  ir.types[type_index].content_model =
    Some(crate::sdk_code::codegen_ir::ContentModelDecl::ChoiceOnly);
  ir.types.push(crate::sdk_code::codegen_ir::TypeDecl {
    rust_name: choice_name,
    xml_qname: None,
    docs: field.docs.clone(),
    version,
    is_abstract: false,
    kind: TypeKind::ChoiceEnum,
    element_kind: None,
    content_model: None,
    base_rust_name: None,
    base_module_path: None,
    xml_content: None,
    support: crate::sdk_code::codegen_ir::SystemSupportDecl::default(),
    estimated_size: 0,
    members: vec![
      MemberDecl::Variant(VariantDecl {
        rust_name: known_variant_name,
        docs: field.docs,
        version: field.version,
        wire: VariantWireDecl::Child {
          qnames: vec![qname],
        },
        payload: field.type_ref,
      }),
      MemberDecl::Variant(VariantDecl {
        rust_name: "AlternateContent".to_string(),
        docs: "Markup Compatibility alternate content.".to_string(),
        version: String::new(),
        wire: VariantWireDecl::Child {
          qnames: vec!["mc:AlternateContent".to_string()],
        },
        payload: TypeRefDecl {
          rust_type: "AlternateContent".to_string(),
          module_path: Some(
            "crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006".to_string(),
          ),
        },
      }),
    ],
  });
  Ok(())
}

fn add_alternate_content_fields(
  module_name: &str,
  type_decl: &mut crate::sdk_code::codegen_ir::TypeDecl,
  extension: &SchemaTypeExtension,
) -> Result<()> {
  let field_count = extension.alternate_content.len();
  for (slot, alternate_content) in extension.alternate_content.iter().enumerate() {
    let location_count = [
      alternate_content.before_field.as_str(),
      alternate_content.after_field.as_str(),
    ]
    .into_iter()
    .filter(|field| !field.is_empty())
    .count();
    if location_count != 1 {
      return Err(
        format!(
          "schema extension alternate content {module_name}.{} must specify one field location",
          extension.class_name
        )
        .into(),
      );
    }
    if alternate_content.children.is_empty() {
      return Err(
        format!(
          "schema extension alternate content {module_name}.{} must declare children",
          extension.class_name
        )
        .into(),
      );
    }
    for child_name in &alternate_content.children {
      let child_is_supported = type_decl.members.iter().any(|member| {
        matches!(
          member,
          MemberDecl::Field(field)
            if field.rust_name == *child_name
              && matches!(field.wire, FieldWireDecl::Child { .. } | FieldWireDecl::TextChild { .. })
        )
      });
      if !child_is_supported {
        return Err(
          format!(
            "schema extension alternate content {module_name}.{} child {child_name} is not a direct child field",
            extension.class_name
          )
          .into(),
        );
      }
    }

    let location = if !alternate_content.before_field.is_empty() {
      alternate_content.before_field.as_str()
    } else {
      alternate_content.after_field.as_str()
    };
    let Some(location_index) = type_decl
      .members
      .iter()
      .position(|member| matches!(member, MemberDecl::Field(field) if field.rust_name == location))
    else {
      return Err(
        format!(
          "schema extension alternate content {module_name}.{} location {location} not found",
          extension.class_name
        )
        .into(),
      );
    };
    let insert_index = if alternate_content.before_field.is_empty() {
      location_index + 1
    } else {
      location_index
    };
    let rust_name = if field_count == 1 {
      "alternate_content".to_string()
    } else {
      format!("alternate_content_{slot}")
    };
    type_decl
      .support
      .alternate_content_children
      .insert(rust_name.clone(), alternate_content.children.clone());
    type_decl.members.insert(
      insert_index,
      MemberDecl::Field(crate::sdk_code::codegen_ir::FieldDecl {
        rust_name,
        docs: "Markup Compatibility alternate content at this schema position.".to_string(),
        version: String::new(),
        wire: FieldWireDecl::Child {
          qname: "mc:AlternateContent".to_string(),
        },
        cardinality: Cardinality::Many,
        type_ref: TypeRefDecl {
          rust_type: "AlternateContent".to_string(),
          module_path: Some(
            "crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006".to_string(),
          ),
        },
        validators: Vec::new(),
      }),
    );
  }
  Ok(())
}

fn apply_choice_enum_repeated(
  module_name: &str,
  ir: &mut SchemaModuleDecl,
  choice_rust_name: &str,
  repeated: bool,
) -> Result<()> {
  let mut changed = false;
  let cardinality = if repeated {
    Cardinality::Many
  } else {
    Cardinality::Optional
  };

  for type_decl in &mut ir.types {
    if type_decl.kind == TypeKind::ChoiceEnum {
      continue;
    }

    for member in &mut type_decl.members {
      let MemberDecl::Field(field) = member else {
        continue;
      };
      if field.wire == FieldWireDecl::Choice && field.type_ref.rust_type == choice_rust_name {
        field.cardinality = cardinality;
        changed = true;
      }
    }
  }

  if changed {
    Ok(())
  } else {
    Err(
      format!("schema extension choice enum {module_name}.{choice_rust_name} has no choice field")
        .into(),
    )
  }
}

fn variant_wire_qnames(wire: &VariantWireDecl) -> Option<&[String]> {
  match wire {
    VariantWireDecl::Child { qnames }
    | VariantWireDecl::Sequence { qnames }
    | VariantWireDecl::TextChild { qnames } => Some(qnames),
    VariantWireDecl::Any { .. } | VariantWireDecl::Text => None,
  }
}

fn add_schema_type_child(
  module_name: &str,
  class_name: &str,
  schema_type: &mut crate::sdk_data::sdk_data_model::SchemaType,
  extension: &SchemaTypeAddChildExtension,
) -> Result<()> {
  if extension.name.is_empty() || extension.property_name.is_empty() {
    return Err(
      format!("schema extension child {module_name}.{class_name} add child is incomplete").into(),
    );
  }
  if schema_type
    .children
    .iter()
    .any(|child| child.name == extension.name)
  {
    return Err(
      format!(
        "schema extension child {module_name}.{class_name} already has child {}",
        extension.name
      )
      .into(),
    );
  }
  if schema_type
    .children
    .iter()
    .any(|child| child.property_name == extension.property_name)
  {
    return Err(
      format!(
        "schema extension child {module_name}.{class_name} already has property {}",
        extension.property_name
      )
      .into(),
    );
  }

  let location_count = [
    extension.before_name.as_str(),
    extension.after_name.as_str(),
    extension.before_property_name.as_str(),
    extension.after_property_name.as_str(),
  ]
  .iter()
  .filter(|value| !value.is_empty())
  .count();
  if location_count != 1 {
    return Err(
      format!(
        "schema extension child {module_name}.{class_name} add child must specify one location"
      )
      .into(),
    );
  }

  let index = find_add_child_location_index(&schema_type.children, extension).ok_or_else(|| {
    format!("schema extension child {module_name}.{class_name} add child location not found")
  })?;

  schema_type.children.insert(
    index,
    crate::sdk_data::sdk_data_model::SchemaTypeChild {
      name: extension.name.clone(),
      property_name: extension.property_name.clone(),
      property_comments: extension.property_comments.clone(),
      kind: crate::sdk_data::sdk_data_model::SchemaTypeChildKind::Child,
      optional: extension.optional.unwrap_or(false),
      repeated: extension.repeated.unwrap_or(false),
      initial_version: extension.initial_version.clone(),
      ..Default::default()
    },
  );

  Ok(())
}

fn find_add_child_location_index(
  children: &[crate::sdk_data::sdk_data_model::SchemaTypeChild],
  extension: &SchemaTypeAddChildExtension,
) -> Option<usize> {
  for (index, child) in children.iter().enumerate() {
    if !extension.before_name.is_empty() && child.name == extension.before_name {
      return Some(index);
    }
    if !extension.after_name.is_empty() && child.name == extension.after_name {
      return Some(index + 1);
    }
    if !extension.before_property_name.is_empty()
      && child.property_name == extension.before_property_name
    {
      return Some(index);
    }
    if !extension.after_property_name.is_empty()
      && child.property_name == extension.after_property_name
    {
      return Some(index + 1);
    }
  }

  None
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
  use crate::sdk_code::codegen_ir::{
    Cardinality, FieldDecl, FieldWireDecl, SchemaModuleDecl, TypeDecl,
  };
  use crate::sdk_data::sdk_data_model::{
    SchemaEnum, SchemaEnumFacet, SchemaType, SchemaTypeAttribute, SchemaTypeChild,
  };

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
          have_mc_ignorable: Some(true),
          have_mc_preserve_attributes: Some(true),
          have_mc_preserve_elements: Some(true),
          have_mc_process_content: Some(true),
          have_mc_must_understand: Some(true),
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
    assert!(schemas[0].types[0].have_mc_ignorable);
    assert!(schemas[0].types[0].have_mc_preserve_attributes);
    assert!(schemas[0].types[0].have_mc_preserve_elements);
    assert!(schemas[0].types[0].have_mc_process_content);
    assert!(schemas[0].types[0].have_mc_must_understand);
  }

  #[test]
  fn applies_add_type_extension() {
    let mut schemas = vec![Schema {
      module_name: "test_schema".to_string(),
      ..Default::default()
    }];
    let extensions = vec![(
      "test_schema".to_string(),
      SchemaExtensions {
        add_types: vec![SchemaTypeAddTypeExtension {
          name: "t:CT_Added/t:added".to_string(),
          class_name: "Added".to_string(),
          summary: "Added type.".to_string(),
          base_class: "OpenXmlLeafElement".to_string(),
          kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf,
          api_kind: crate::sdk_data::sdk_data_model::SchemaTypeApiKind::Struct,
          attributes: vec![SchemaTypeAttributeExtension {
            q_name: "t:value".to_string(),
            property_name: "Value".to_string(),
            override_type: "StringValue".to_string(),
            optional: Some(true),
            ..Default::default()
          }],
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &extensions).unwrap();

    let added = &schemas[0].types[0];
    assert_eq!(added.name, "t:CT_Added/t:added");
    assert_eq!(added.class_name, "Added");
    assert_eq!(added.attributes[0].q_name, "t:value");
    assert_eq!(added.attributes[0].property_name, "Value");
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
  fn applies_child_add_extensions_after_property_name_chain() {
    let mut schemas = vec![Schema {
      module_name: "test_schema".to_string(),
      types: vec![SchemaType {
        class_name: "Parent".to_string(),
        children: vec![SchemaTypeChild {
          name: "t:CT_Existing/t:existing".to_string(),
          property_name: "Existing".to_string(),
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
          add_children: vec![
            SchemaTypeAddChildExtension {
              name: "t:CT_First/t:first".to_string(),
              property_name: "First".to_string(),
              property_comments: "Defines the First Class.".to_string(),
              optional: Some(true),
              initial_version: "Office2010".to_string(),
              after_property_name: "Existing".to_string(),
              ..Default::default()
            },
            SchemaTypeAddChildExtension {
              name: "t:CT_Second/t:second".to_string(),
              property_name: "Second".to_string(),
              property_comments: "Defines the Second Class.".to_string(),
              optional: Some(true),
              initial_version: "Office2010".to_string(),
              after_property_name: "First".to_string(),
              ..Default::default()
            },
          ],
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &extensions).unwrap();

    let first = &schemas[0].types[0].children[1];
    assert_eq!(first.name, "t:CT_First/t:first");
    assert_eq!(first.property_name, "First");
    assert_eq!(first.property_comments, "Defines the First Class.");
    assert!(first.optional);
    assert_eq!(first.initial_version, "Office2010");

    let second = &schemas[0].types[0].children[2];
    assert_eq!(second.name, "t:CT_Second/t:second");
    assert_eq!(second.property_name, "Second");
    assert_eq!(second.property_comments, "Defines the Second Class.");
    assert!(second.optional);
    assert_eq!(second.initial_version, "Office2010");
  }

  #[test]
  fn rejects_child_add_extension_without_location() {
    let mut schemas = vec![Schema {
      module_name: "test_schema".to_string(),
      types: vec![SchemaType {
        class_name: "Parent".to_string(),
        children: vec![SchemaTypeChild {
          name: "t:CT_Existing/t:existing".to_string(),
          property_name: "Existing".to_string(),
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
          add_children: vec![SchemaTypeAddChildExtension {
            name: "t:CT_New/t:new".to_string(),
            property_name: "New".to_string(),
            ..Default::default()
          }],
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    let err = apply_schema_extensions(&mut schemas, &extensions).unwrap_err();

    assert!(
      err
        .to_string()
        .contains("add child must specify one location")
    );
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
            override_type: "IntegerValue".to_string(),
            version: "Office2010".to_string(),
            ..Default::default()
          }],
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &extensions).unwrap();

    assert_eq!(schemas[0].types[0].attributes[0].r#type, "IntegerValue");
    assert_eq!(schemas[0].types[0].attributes[0].version, "Office2010");
  }

  #[test]
  fn applies_attribute_override_q_name_extension() {
    let mut schemas = vec![Schema {
      module_name: "test_schema".to_string(),
      types: vec![SchemaType {
        class_name: "BooleanFalse".to_string(),
        attributes: vec![SchemaTypeAttribute {
          q_name: "c16r3:val".to_string(),
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
          class_name: "BooleanFalse".to_string(),
          attributes: vec![SchemaTypeAttributeExtension {
            q_name: "c16r3:val".to_string(),
            override_q_name: ":val".to_string(),
            ..Default::default()
          }],
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &extensions).unwrap();

    assert_eq!(schemas[0].types[0].attributes[0].q_name, ":val");
  }

  #[test]
  fn applies_type_extension_to_types_with_matching_base_class() {
    let mut schemas = vec![Schema {
      module_name: "test_schema".to_string(),
      types: vec![
        SchemaType {
          class_name: "FirstDerived".to_string(),
          base_class: "SharedBase".to_string(),
          ..Default::default()
        },
        SchemaType {
          class_name: "SecondDerived".to_string(),
          base_class: "SharedBase".to_string(),
          ..Default::default()
        },
        SchemaType {
          class_name: "Unrelated".to_string(),
          base_class: "OtherBase".to_string(),
          ..Default::default()
        },
      ],
      ..Default::default()
    }];
    let extensions = vec![(
      "test_schema".to_string(),
      SchemaExtensions {
        types: vec![SchemaTypeExtension {
          class_name: "SharedBase".to_string(),
          attributes: vec![SchemaTypeAttributeExtension {
            q_name: "t:shared".to_string(),
            property_name: "Shared".to_string(),
            override_type: "StringValue".to_string(),
            optional: Some(true),
            ..Default::default()
          }],
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &extensions).unwrap();

    for schema_type in &schemas[0].types[..2] {
      assert_eq!(schema_type.attributes.len(), 1);
      assert_eq!(schema_type.attributes[0].q_name, "t:shared");
      assert_eq!(schema_type.attributes[0].property_name, "Shared");
      assert_eq!(schema_type.attributes[0].r#type, "StringValue");
      assert!(!schema_type.attributes[0].required);
    }
    assert!(schemas[0].types[2].attributes.is_empty());
  }

  #[test]
  fn applies_attribute_optional_extension_by_property_name() {
    let mut schemas = vec![Schema {
      module_name: "test_schema".to_string(),
      types: vec![SchemaType {
        class_name: "ConditionalFormatStyle".to_string(),
        attributes: vec![SchemaTypeAttribute {
          q_name: "w:val".to_string(),
          property_name: "Val".to_string(),
          required: true,
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
          class_name: "ConditionalFormatStyle".to_string(),
          attributes: vec![SchemaTypeAttributeExtension {
            property_name: "Val".to_string(),
            optional: Some(true),
            ..Default::default()
          }],
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &extensions).unwrap();

    assert!(!schemas[0].types[0].attributes[0].required);
  }

  #[test]
  fn applies_attribute_add_extension() {
    let mut schemas = vec![Schema {
      module_name: "test_schema".to_string(),
      types: vec![SchemaType {
        class_name: "AxisId".to_string(),
        base_class: "UnsignedIntegerType".to_string(),
        kind: crate::sdk_data::sdk_data_model::SchemaTypeKind::Derived,
        ..Default::default()
      }],
      ..Default::default()
    }];
    let extensions = vec![(
      "test_schema".to_string(),
      SchemaExtensions {
        types: vec![SchemaTypeExtension {
          class_name: "AxisId".to_string(),
          override_base_class: "OpenXmlLeafElement".to_string(),
          attributes: vec![SchemaTypeAttributeExtension {
            q_name: ":val".to_string(),
            override_q_name: String::new(),
            property_name: "Val".to_string(),
            property_comments: "Integer Value".to_string(),
            version: "Office2010".to_string(),
            optional: Some(false),
            match_local_name: None,
            empty_as_none: None,
            override_type: "Int32Value".to_string(),
          }],
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &extensions).unwrap();

    let attr = &schemas[0].types[0].attributes[0];
    assert_eq!(schemas[0].types[0].base_class, "OpenXmlLeafElement");
    assert_eq!(
      schemas[0].types[0].kind,
      crate::sdk_data::sdk_data_model::SchemaTypeKind::Leaf
    );
    assert_eq!(attr.q_name, ":val");
    assert_eq!(attr.property_name, "Val");
    assert_eq!(attr.property_comments, "Integer Value");
    assert_eq!(attr.version, "Office2010");
    assert_eq!(attr.r#type, "Int32Value");
    assert!(attr.required);
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
  fn applies_enum_alias_extension_to_existing_facet() {
    let mut schemas = vec![Schema {
      module_name: "test_schema".to_string(),
      enums: vec![SchemaEnum {
        name: "HeaderFooterValues".to_string(),
        facets: vec![SchemaEnumFacet {
          name: "Default".to_string(),
          value: "default".to_string(),
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
          name: "HeaderFooterValues".to_string(),
          add_facets: vec![SchemaEnumFacetExtension {
            name: "Default".to_string(),
            value: "default".to_string(),
            aliases: vec!["odd".to_string()],
            ..Default::default()
          }],
        }],
        ..Default::default()
      },
    )];

    apply_schema_extensions(&mut schemas, &extensions).unwrap();

    assert_eq!(schemas[0].enums[0].facets.len(), 1);
    assert_eq!(schemas[0].enums[0].facets[0].aliases, ["odd"]);
  }

  #[test]
  fn applies_choice_enum_add_variant_extension() {
    let mut ir = SchemaModuleDecl {
      module_name: "test_schema".to_string(),
      types: vec![TypeDecl {
        rust_name: "ControlPropertiesChoice".to_string(),
        kind: TypeKind::ChoiceEnum,
        estimated_size: 0,
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
          repeated: None,
          add_xml_any: None,
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
        estimated_size: 0,
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
          add_xml_any: Some(vec!["wp:wsp".to_string()]),
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
    assert_eq!(
      variant.wire,
      VariantWireDecl::Any {
        qnames: vec!["wp:wsp".to_string()]
      }
    );
    assert_eq!(variant.payload.rust_type, "std::boxed::Box<[u8]>");
    assert!(variant.payload.module_path.is_none());
  }

  #[test]
  fn narrows_existing_choice_enum_xml_any_extension_to_qnames() {
    let mut ir = SchemaModuleDecl {
      module_name: "test_schema".to_string(),
      types: vec![TypeDecl {
        rust_name: "GraphicDataChoice".to_string(),
        kind: TypeKind::ChoiceEnum,
        estimated_size: 0,
        members: vec![MemberDecl::Variant(VariantDecl {
          rust_name: "XmlAny".to_string(),
          wire: VariantWireDecl::Any { qnames: Vec::new() },
          payload: TypeRefDecl {
            rust_type: "std::boxed::Box<[u8]>".to_string(),
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
          rust_name: "GraphicDataChoice".to_string(),
          add_xml_any: Some(vec!["wp:wsp".to_string()]),
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_codegen_ir_schema_extensions("test_schema", &mut ir, &extensions).unwrap();

    assert_eq!(ir.types[0].members.len(), 1);
    let MemberDecl::Variant(variant) = &ir.types[0].members[0] else {
      panic!("expected variant");
    };
    assert_eq!(
      variant.wire,
      VariantWireDecl::Any {
        qnames: vec!["wp:wsp".to_string()]
      }
    );
  }

  #[test]
  fn applies_choice_enum_repeated_extension_to_choice_fields() {
    let mut ir = SchemaModuleDecl {
      module_name: "test_schema".to_string(),
      types: vec![
        TypeDecl {
          rust_name: "DataLabel".to_string(),
          kind: TypeKind::ElementStruct,
          estimated_size: 0,
          members: vec![MemberDecl::Field(FieldDecl {
            rust_name: "data_label_choice".to_string(),
            wire: FieldWireDecl::Choice,
            cardinality: Cardinality::Optional,
            type_ref: TypeRefDecl {
              rust_type: "DataLabelChoice".to_string(),
              module_path: None,
            },
            ..Default::default()
          })],
          ..Default::default()
        },
        TypeDecl {
          rust_name: "DataLabelChoice".to_string(),
          kind: TypeKind::ChoiceEnum,
          estimated_size: 0,
          members: vec![MemberDecl::Variant(VariantDecl {
            rust_name: "Delete".to_string(),
            wire: VariantWireDecl::Child {
              qnames: vec!["c:CT_Boolean/c:delete".to_string()],
            },
            payload: TypeRefDecl {
              rust_type: "Delete".to_string(),
              module_path: None,
            },
            ..Default::default()
          })],
          ..Default::default()
        },
      ],
      ..Default::default()
    };
    let extensions = vec![(
      "test_schema".to_string(),
      SchemaExtensions {
        choice_enums: vec![SchemaChoiceEnumExtension {
          rust_name: "DataLabelChoice".to_string(),
          repeated: Some(true),
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_codegen_ir_schema_extensions("test_schema", &mut ir, &extensions).unwrap();

    let MemberDecl::Field(field) = &ir.types[0].members[0] else {
      panic!("expected field");
    };
    assert_eq!(field.cardinality, Cardinality::Many);
  }

  #[test]
  fn adds_positioned_alternate_content_fields_without_single_slot_suffix() {
    let mut ir = SchemaModuleDecl {
      module_name: "test_schema".to_string(),
      types: vec![TypeDecl {
        rust_name: "Parent".to_string(),
        kind: TypeKind::ElementStruct,
        estimated_size: 0,
        members: vec![MemberDecl::Field(FieldDecl {
          rust_name: "known_child".to_string(),
          wire: FieldWireDecl::Child {
            qname: "t:known".to_string(),
          },
          cardinality: Cardinality::Optional,
          type_ref: TypeRefDecl {
            rust_type: "KnownChild".to_string(),
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
        types: vec![SchemaTypeExtension {
          class_name: "Parent".to_string(),
          alternate_content: vec![SchemaTypeAlternateContentExtension {
            after_field: "known_child".to_string(),
            children: vec!["known_child".to_string()],
            ..Default::default()
          }],
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_codegen_ir_schema_extensions("test_schema", &mut ir, &extensions).unwrap();

    let MemberDecl::Field(field) = &ir.types[0].members[1] else {
      panic!("expected alternate content field");
    };
    assert_eq!(field.rust_name, "alternate_content");
    assert_eq!(field.cardinality, Cardinality::Many);
    assert_eq!(
      ir.types[0]
        .support
        .alternate_content_children
        .get("alternate_content"),
      Some(&vec!["known_child".to_string()])
    );
    assert_eq!(
      field.wire,
      FieldWireDecl::Child {
        qname: "mc:AlternateContent".to_string()
      }
    );
  }

  #[test]
  fn numbers_multiple_positioned_alternate_content_fields_from_zero() {
    let child = |rust_name: &str| {
      MemberDecl::Field(FieldDecl {
        rust_name: rust_name.to_string(),
        wire: FieldWireDecl::Child {
          qname: format!("t:{rust_name}"),
        },
        cardinality: Cardinality::Optional,
        type_ref: TypeRefDecl {
          rust_type: "KnownChild".to_string(),
          module_path: None,
        },
        ..Default::default()
      })
    };
    let mut ir = SchemaModuleDecl {
      module_name: "test_schema".to_string(),
      types: vec![TypeDecl {
        rust_name: "Parent".to_string(),
        kind: TypeKind::ElementStruct,
        estimated_size: 0,
        members: vec![child("first"), child("second")],
        ..Default::default()
      }],
      ..Default::default()
    };
    let extensions = vec![(
      "test_schema".to_string(),
      SchemaExtensions {
        types: vec![SchemaTypeExtension {
          class_name: "Parent".to_string(),
          alternate_content: vec![
            SchemaTypeAlternateContentExtension {
              after_field: "first".to_string(),
              children: vec!["first".to_string()],
              ..Default::default()
            },
            SchemaTypeAlternateContentExtension {
              after_field: "second".to_string(),
              children: vec!["second".to_string()],
              ..Default::default()
            },
          ],
          ..Default::default()
        }],
        ..Default::default()
      },
    )];

    apply_codegen_ir_schema_extensions("test_schema", &mut ir, &extensions).unwrap();

    let names = ir.types[0]
      .members
      .iter()
      .filter_map(|member| match member {
        MemberDecl::Field(field) => Some(field.rust_name.as_str()),
        MemberDecl::Variant(_) => None,
      })
      .collect::<Vec<_>>();
    assert_eq!(
      names,
      [
        "first",
        "alternate_content_0",
        "second",
        "alternate_content_1"
      ]
    );
  }

  #[test]
  fn rejects_positioned_alternate_content_without_known_children() {
    let make_ir = || SchemaModuleDecl {
      module_name: "test_schema".to_string(),
      types: vec![TypeDecl {
        rust_name: "Parent".to_string(),
        kind: TypeKind::ElementStruct,
        members: vec![MemberDecl::Field(FieldDecl {
          rust_name: "known_child".to_string(),
          wire: FieldWireDecl::Child {
            qname: "t:known".to_string(),
          },
          cardinality: Cardinality::Optional,
          type_ref: TypeRefDecl {
            rust_type: "KnownChild".to_string(),
            module_path: None,
          },
          ..Default::default()
        })],
        ..Default::default()
      }],
      ..Default::default()
    };
    for children in [Vec::new(), vec!["missing_child".to_string()]] {
      let mut ir = make_ir();
      let extensions = vec![(
        "test_schema".to_string(),
        SchemaExtensions {
          types: vec![SchemaTypeExtension {
            class_name: "Parent".to_string(),
            alternate_content: vec![SchemaTypeAlternateContentExtension {
              after_field: "known_child".to_string(),
              children,
              ..Default::default()
            }],
            ..Default::default()
          }],
          ..Default::default()
        },
      )];
      let error = apply_codegen_ir_schema_extensions("test_schema", &mut ir, &extensions)
        .expect_err("invalid static MCE children");
      assert!(error.to_string().contains("alternate content"));
    }
  }
}
