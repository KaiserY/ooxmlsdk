use crate::sdk_data::{
  context::Context,
  sdk_data_model::{
    Namespace, Schema, SchemaEnum, SchemaEnumFacet, SchemaType, SchemaTypeApiKind,
    SchemaTypeAttribute, SchemaTypeChild, SchemaTypeChildKind, SchemaTypeCompositeKind,
    SchemaTypeKind, SchemaTypeXmlHeader,
  },
};

use crate::sdk_code::versioning::effective_version;
use heck::ToUpperCamelCase;
use std::collections::HashMap;

pub fn gen_namespaces(gen_context: &Context) -> Vec<Namespace> {
  let mut namespaces: Vec<Namespace> = gen_context
    .namespaces
    .iter()
    .filter(|namespace| !namespace.uri.is_empty())
    .map(|namespace| Namespace {
      prefix: namespace.prefix.clone(),
      uri: namespace.uri.clone(),
      version: namespace.version.clone(),
    })
    .collect();

  namespaces.sort_by(|left, right| {
    left
      .prefix
      .cmp(&right.prefix)
      .then(left.uri.cmp(&right.uri))
  });

  namespaces
}

pub fn gen_schemas(gen_context: &Context) -> Vec<Schema> {
  let type_map: HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType> = gen_context
    .schemas
    .iter()
    .flat_map(|schema| schema.types.iter())
    .map(|ty| (ty.name.as_str(), ty))
    .collect();

  let schemas: Vec<Schema> = gen_context
    .schemas
    .iter()
    .map(|schema| Schema {
      target_namespace: schema.target_namespace.clone(),
      prefix: gen_context
        .namespace_uri_prefix_map
        .get(&schema.target_namespace)
        .cloned()
        .unwrap_or_default(),
      typed_namespace: gen_context
        .namespace_uri_prefix_map
        .get(&schema.target_namespace)
        .and_then(|prefix| gen_context.prefix_typed_namespace_map.get(prefix))
        .cloned()
        .unwrap_or_default(),
      module_name: schema.module_name.clone(),
      types: schema
        .types
        .iter()
        .map(|ty| {
          let composite_kind = resolve_composite_kind(ty);
          let kind = resolve_kind(ty, &type_map);
          let have_xmlns_fields = ty.has_xmlns_fields
            || !ty.part.is_empty()
            || ty.base_class == "OpenXmlPartRootElement"
            || has_extension_xmlns_fields(ty, kind)
            || has_drawing_payload_xmlns_fields(ty, kind, &type_map)
            || has_spreadsheet_repeated_part_root_content_xmlns_fields(
              ty,
              kind,
              schema.module_name.as_str(),
              &type_map,
            );
          let raw_child_map: HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaTypeChild> = ty
            .children
            .iter()
            .map(|child| (child.name.as_str(), child))
            .collect();
          let mut children = Vec::new();
          if composite_kind == SchemaTypeCompositeKind::OneChoice {
            children.extend(gen_one_choice_children(ty, &raw_child_map, &type_map));
          } else if ty.particle.kind == "All" {
            children.extend(ty.children.iter().map(|child| SchemaTypeChild {
              particle_id: String::new(),
              name: child.name.clone(),
              property_name: child.property_name.clone(),
              property_comments: child.property_comments.clone(),
              kind: resolve_child_kind(child.name.as_str(), &type_map),
              optional: false,
              repeated: false,
              initial_version: String::new(),
              children: Vec::new(),
            }));
          } else if ty.particle.kind.is_empty() {
            if !ty.additional_elements.is_empty() {
              let variants: Vec<SchemaTypeChild> = ty
                .children
                .iter()
                .map(|child| SchemaTypeChild {
                  particle_id: String::new(),
                  name: child.name.clone(),
                  property_name: child.property_name.clone(),
                  property_comments: child.property_comments.clone(),
                  kind: resolve_child_kind(child.name.as_str(), &type_map),
                  optional: false,
                  repeated: false,
                  initial_version: String::new(),
                  children: Vec::new(),
                })
                .collect();

              if !variants.is_empty() {
                children.push(SchemaTypeChild {
                  particle_id: String::new(),
                  name: String::new(),
                  property_name: "children".to_string(),
                  property_comments: String::new(),
                  kind: SchemaTypeChildKind::Choice,
                  optional: false,
                  repeated: true,
                  initial_version: String::new(),
                  children: variants,
                });
              }
            } else {
              children.extend(ty.children.iter().map(|child| SchemaTypeChild {
                particle_id: String::new(),
                name: child.name.clone(),
                property_name: child.property_name.clone(),
                property_comments: child.property_comments.clone(),
                kind: resolve_child_kind(child.name.as_str(), &type_map),
                optional: false,
                repeated: false,
                initial_version: String::new(),
                children: Vec::new(),
              }));
            }
          } else {
            collect_choice_children(
              &ty.particle,
              &raw_child_map,
              &type_map,
              &mut children,
              false,
              false,
              false,
            );
          }
          mark_sequence_collection_children_repeated(ty, &mut children);
          mark_mixed_sequence_direct_children_optional(&mut children);
          let have_xml_other_attrs = have_xml_other_attrs_for_mixed_version_content(
            ty,
            kind,
            schema.module_name.as_str(),
            &type_map,
            &children,
          );
          let have_xml_other_children =
            have_xml_other_children_for_mixed_version_content(ty, &type_map, &children)
              || have_xml_other_children_for_spreadsheet_repeated_part_root_content_child(
                ty,
                kind,
                schema.module_name.as_str(),
                &type_map,
              )
              || have_xml_other_children_for_text_list_style_extension_siblings(ty, kind)
              || have_xml_other_children_for_common_repeated_content(
                ty,
                kind,
                have_xmlns_fields,
                have_xml_other_attrs,
                &children,
              );
          assign_particle_ids(&mut children);

          let xml_header = if !ty.part.is_empty() || ty.base_class == "OpenXmlPartRootElement" {
            SchemaTypeXmlHeader::Standalone
          } else {
            SchemaTypeXmlHeader::None
          };

          SchemaType {
            name: ty.name.clone(),
            class_name: ty.class_name.clone(),
            summary: ty.summary.clone(),
            version: ty.version.clone(),
            part: ty.part.clone(),
            base_class: ty.base_class.clone(),
            kind,
            composite_kind,
            xml_header,
            is_abstract: ty.is_abstract,
            have_xmlns_fields,
            have_xml_other_attrs,
            have_xml_other_children,
            have_direct_xml_other_children: false,
            parent_have_xml_other_children: false,
            text_value_type: String::new(),
            api_kind: resolve_api_kind(ty, &type_map),
            attributes: ty
              .attributes
              .iter()
              .map(|attr| SchemaTypeAttribute {
                q_name: attr.q_name.clone(),
                property_name: attr.property_name.clone(),
                r#type: attr.r#type.clone(),
                property_comments: attr.property_comments.clone(),
                version: attr.version.clone(),
                required: attr
                  .validators
                  .iter()
                  .any(|validator| validator.name == "RequiredValidator"),
                validators: attr
                  .validators
                  .iter()
                  .map(
                    |validator| crate::sdk_data::sdk_data_model::SchemaTypeAttributeValidator {
                      name: validator.name.clone(),
                      is_list: validator.is_list,
                      r#type: validator.r#type.clone(),
                      union_id: validator.union_id,
                      is_initial_version: validator.is_initial_version,
                      arguments: validator
                        .arguments
                        .iter()
                        .map(|argument| {
                          crate::sdk_data::sdk_data_model::SchemaTypeAttributeValidatorArgument {
                            name: argument.name.clone(),
                            r#type: argument.r#type.clone(),
                            value: argument.value.clone(),
                          }
                        })
                        .collect(),
                    },
                  )
                  .collect(),
                ..Default::default()
              })
              .collect(),
            children,
          }
        })
        .collect(),
      enums: schema
        .enums
        .iter()
        .map(|schema_enum| SchemaEnum {
          name: schema_enum.name.clone(),
          r#type: schema_enum.r#type.clone(),
          version: schema_enum.version.clone(),
          facets: schema_enum
            .facets
            .iter()
            .map(|facet| SchemaEnumFacet {
              name: facet.name.clone(),
              value: facet.value.clone(),
              version: facet.version.clone(),
              ..Default::default()
            })
            .collect(),
        })
        .collect(),
    })
    .collect();

  schemas
}

pub(crate) fn assign_schema_particle_ids(schemas: &mut [Schema]) {
  for schema in schemas {
    for schema_type in &mut schema.types {
      assign_particle_ids(&mut schema_type.children);
    }
  }
}

fn gen_one_choice_children(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  raw_child_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaTypeChild>,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> Vec<SchemaTypeChild> {
  let mut variants = Vec::new();

  if schema_type.particle.kind.is_empty() {
    variants.extend(schema_type.children.iter().map(|child| SchemaTypeChild {
      particle_id: String::new(),
      name: child.name.clone(),
      property_name: child.property_name.clone(),
      property_comments: child.property_comments.clone(),
      kind: resolve_child_kind(child.name.as_str(), type_map),
      optional: false,
      repeated: false,
      initial_version: String::new(),
      children: Vec::new(),
    }));
  } else {
    collect_one_choice_variants(
      &schema_type.particle,
      raw_child_map,
      type_map,
      &mut variants,
    );
  }

  if variants.is_empty() {
    return Vec::new();
  }

  let (optional, repeated, initial_version) = particle_cardinality(&schema_type.particle);

  vec![SchemaTypeChild {
    particle_id: String::new(),
    name: String::new(),
    property_name: "children".to_string(),
    property_comments: String::new(),
    kind: SchemaTypeChildKind::Choice,
    optional,
    repeated,
    initial_version,
    children: variants,
  }]
}

fn collect_one_choice_variants(
  particle: &crate::sdk_data::open_xml::OpenXmlSchemaTypeParticle,
  raw_child_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaTypeChild>,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
  out: &mut Vec<SchemaTypeChild>,
) {
  match particle.kind.as_str() {
    "Choice" => {
      for item in &particle.items {
        collect_choice_variant_children(item, raw_child_map, type_map, out, false, false, false);
      }
    }
    "Group" | "Sequence" => {
      for item in &particle.items {
        collect_one_choice_variants(item, raw_child_map, type_map, out);
      }
    }
    "Any" | "" => {
      collect_choice_variant_children(particle, raw_child_map, type_map, out, false, false, false);
    }
    _ => {}
  }
}

fn collect_choice_children(
  particle: &crate::sdk_data::open_xml::OpenXmlSchemaTypeParticle,
  raw_child_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaTypeChild>,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
  out: &mut Vec<SchemaTypeChild>,
  inherited_optional: bool,
  inherited_repeated: bool,
  preserve_nested_choice_wrappers: bool,
) {
  let (optional, repeated, initial_version) = particle_cardinality(particle);
  let optional = inherited_optional || optional;
  let repeated = inherited_repeated || repeated;

  match particle.kind.as_str() {
    "Any" => {
      out.push(SchemaTypeChild {
        particle_id: String::new(),
        name: String::new(),
        property_name: "UnknownXml".to_string(),
        property_comments: String::new(),
        kind: SchemaTypeChildKind::Any,
        optional,
        repeated,
        initial_version,
        children: Vec::new(),
      });
    }
    "" => {
      if let Some(child) = schema_child_from_particle(particle, raw_child_map, type_map) {
        out.push(SchemaTypeChild {
          particle_id: String::new(),
          optional: optional || child.optional,
          repeated: repeated || child.repeated,
          ..child
        });
      }
    }
    "Choice" => {
      let mut variants = Vec::new();
      for item in &particle.items {
        collect_choice_variant_children(
          item,
          raw_child_map,
          type_map,
          &mut variants,
          false,
          false,
          preserve_nested_choice_wrappers,
        );
      }
      if !variants.is_empty() {
        out.push(SchemaTypeChild {
          particle_id: String::new(),
          name: String::new(),
          property_name: "children".to_string(),
          property_comments: String::new(),
          kind: SchemaTypeChildKind::Choice,
          optional,
          repeated,
          initial_version,
          children: variants,
        });
      }
    }
    "Group" | "Sequence" => {
      if preserve_nested_choice_wrappers {
        let mut sequence_children = Vec::new();
        for item in &particle.items {
          collect_choice_variant_children(
            item,
            raw_child_map,
            type_map,
            &mut sequence_children,
            false,
            false,
            preserve_nested_choice_wrappers,
          );
        }
        if !sequence_children.is_empty() {
          out.push(SchemaTypeChild {
            particle_id: String::new(),
            name: String::new(),
            property_name: String::new(),
            property_comments: String::new(),
            kind: SchemaTypeChildKind::Sequence,
            optional,
            repeated,
            initial_version,
            children: sequence_children,
          });
        }
      } else {
        for item in &particle.items {
          collect_choice_children(
            item,
            raw_child_map,
            type_map,
            out,
            optional,
            repeated,
            preserve_nested_choice_wrappers,
          );
        }
      }
    }
    _ => {}
  }
}

fn collect_choice_variant_children(
  particle: &crate::sdk_data::open_xml::OpenXmlSchemaTypeParticle,
  raw_child_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaTypeChild>,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
  out: &mut Vec<SchemaTypeChild>,
  inherited_optional: bool,
  inherited_repeated: bool,
  preserve_nested_choice_wrappers: bool,
) {
  let (optional, repeated, initial_version) = particle_cardinality(particle);
  let optional = inherited_optional || optional;
  let repeated = inherited_repeated || repeated;

  match particle.kind.as_str() {
    "Any" => {
      out.push(SchemaTypeChild {
        particle_id: String::new(),
        name: String::new(),
        property_name: "UnknownXml".to_string(),
        property_comments: String::new(),
        kind: SchemaTypeChildKind::Any,
        optional,
        repeated,
        initial_version,
        children: Vec::new(),
      });
    }
    "" => {
      if let Some(child) = schema_child_from_particle(particle, raw_child_map, type_map) {
        out.push(SchemaTypeChild {
          particle_id: String::new(),
          optional: optional || child.optional,
          repeated: repeated || child.repeated,
          ..child
        });
      }
    }
    "Choice" => {
      let mut variants = Vec::new();
      for item in &particle.items {
        collect_choice_variant_children(
          item,
          raw_child_map,
          type_map,
          &mut variants,
          false,
          false,
          preserve_nested_choice_wrappers,
        );
      }
      normalize_choice_children(&mut variants, !preserve_nested_choice_wrappers);
      if !variants.is_empty() {
        out.push(SchemaTypeChild {
          particle_id: String::new(),
          name: String::new(),
          property_name: String::new(),
          property_comments: String::new(),
          kind: SchemaTypeChildKind::Choice,
          optional,
          repeated,
          initial_version,
          children: variants,
        });
      }
    }
    "Group" | "Sequence" => {
      let mut sequence_children = Vec::new();
      for item in &particle.items {
        collect_choice_variant_children(
          item,
          raw_child_map,
          type_map,
          &mut sequence_children,
          false,
          false,
          preserve_nested_choice_wrappers,
        );
      }
      if !sequence_children.is_empty() {
        if sequence_children.len() == 1 && sequence_children[0].kind == SchemaTypeChildKind::Choice
        {
          let mut child = sequence_children.remove(0);
          child.optional |= optional;
          child.repeated |= repeated;
          child.initial_version =
            effective_version(initial_version.as_str(), child.initial_version.as_str()).to_string();
          out.push(child);
          return;
        }
        out.push(SchemaTypeChild {
          particle_id: String::new(),
          name: String::new(),
          property_name: String::new(),
          property_comments: String::new(),
          kind: SchemaTypeChildKind::Sequence,
          optional,
          repeated,
          initial_version,
          children: sequence_children,
        });
      }
    }
    _ => {}
  }
}

fn normalize_choice_children(
  children: &mut Vec<SchemaTypeChild>,
  flatten_anonymous_choice_wrappers: bool,
) {
  for child in children.iter_mut() {
    normalize_choice_wrappers(child, flatten_anonymous_choice_wrappers);
  }

  if flatten_anonymous_choice_wrappers {
    flatten_anonymous_choice_children(children);
  }
}

fn normalize_choice_wrappers(child: &mut SchemaTypeChild, flatten_anonymous_choice_wrappers: bool) {
  match child.kind {
    SchemaTypeChildKind::Choice => {
      normalize_choice_children(&mut child.children, flatten_anonymous_choice_wrappers)
    }
    SchemaTypeChildKind::Sequence => {
      for nested in child.children.iter_mut() {
        normalize_choice_wrappers(nested, flatten_anonymous_choice_wrappers);
      }
      collapse_single_anonymous_sequence_child(child);
    }
    _ => {}
  }
}

fn flatten_anonymous_choice_children(children: &mut Vec<SchemaTypeChild>) {
  let mut flattened = Vec::with_capacity(children.len());

  for mut child in children.drain(..) {
    if is_anonymous_wrapper(&child, SchemaTypeChildKind::Choice) {
      let wrapper_optional = child.optional;
      let wrapper_repeated = child.repeated;
      let wrapper_initial_version = child.initial_version.clone();

      for mut nested in child.children.drain(..) {
        nested.optional |= wrapper_optional;
        nested.repeated |= wrapper_repeated;
        nested.initial_version = effective_version(
          wrapper_initial_version.as_str(),
          nested.initial_version.as_str(),
        )
        .to_string();
        flattened.push(nested);
      }
    } else {
      flattened.push(child);
    }
  }

  *children = flattened;
}

fn collapse_single_anonymous_sequence_child(child: &mut SchemaTypeChild) {
  if child.kind != SchemaTypeChildKind::Sequence || child.children.len() != 1 {
    return;
  }

  if !is_anonymous_wrapper(&child.children[0], SchemaTypeChildKind::Sequence) {
    return;
  }

  let mut nested = child.children.remove(0);
  child.optional |= nested.optional;
  child.repeated |= nested.repeated;
  child.initial_version = effective_version(
    child.initial_version.as_str(),
    nested.initial_version.as_str(),
  )
  .to_string();
  child.children = std::mem::take(&mut nested.children);
}

fn is_anonymous_wrapper(child: &SchemaTypeChild, kind: SchemaTypeChildKind) -> bool {
  child.kind == kind
    && child.name.is_empty()
    && child.property_name.is_empty()
    && child.property_comments.is_empty()
}

fn schema_child_from_particle(
  particle: &crate::sdk_data::open_xml::OpenXmlSchemaTypeParticle,
  raw_child_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaTypeChild>,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> Option<SchemaTypeChild> {
  if particle.name.is_empty() {
    if particle.kind == "Any" {
      return Some(SchemaTypeChild {
        particle_id: String::new(),
        name: String::new(),
        property_name: "UnknownXml".to_string(),
        property_comments: String::new(),
        kind: SchemaTypeChildKind::Any,
        optional: false,
        repeated: false,
        initial_version: String::new(),
        children: Vec::new(),
      });
    }

    return None;
  }

  let kind = resolve_child_kind(particle.name.as_str(), type_map);

  let (property_name, mut property_comments) = raw_child_map
    .get(particle.name.as_str())
    .map(|child| (child.property_name.clone(), child.property_comments.clone()))
    .unwrap_or_else(|| {
      let fallback_name = resolve_schema_type(particle.name.as_str(), type_map)
        .map(|child_type| child_type.class_name.clone())
        .unwrap_or_else(|| {
          particle
            .name
            .split('/')
            .nth(1)
            .unwrap_or(particle.name.as_str())
            .to_string()
        });
      (fallback_name, String::new())
    });
  if property_comments.is_empty()
    && let Some(summary) = resolve_schema_type(particle.name.as_str(), type_map)
      .map(|child_type| child_type.summary.as_str())
      .filter(|summary| !summary.is_empty())
  {
    property_comments = summary.to_string();
  }
  let is_collection_wrapper = kind == SchemaTypeChildKind::Child
    && property_name.is_empty()
    && particle.kind == "Sequence"
    && particle.items.len() == 1;

  Some(SchemaTypeChild {
    particle_id: String::new(),
    name: particle.name.clone(),
    property_name,
    property_comments,
    kind,
    optional: particle
      .occurs
      .first()
      .is_some_and(|occur| occur.min.is_none() || occur.min == Some(0)),
    repeated: particle
      .occurs
      .first()
      .is_some_and(|occur| occur.max.is_none() || occur.max.is_some_and(|max| max > 1))
      || is_collection_wrapper,
    initial_version: particle.initial_version.clone(),
    children: Vec::new(),
  })
}

fn particle_cardinality(
  particle: &crate::sdk_data::open_xml::OpenXmlSchemaTypeParticle,
) -> (bool, bool, String) {
  let occurs = particle.occurs.first();
  (
    occurs.is_some_and(|occur| occur.min.is_none() || occur.min == Some(0)),
    occurs.is_some_and(|occur| occur.max.is_none() || occur.max.is_some_and(|max| max > 1)),
    particle.initial_version.clone(),
  )
}

fn mark_sequence_collection_children_repeated(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  children: &mut [SchemaTypeChild],
) {
  if schema_type.composite_type != "SdkSequence"
    || schema_type.children.len() != 1
    || children.len() != 1
    || !children[0].property_name.is_empty()
    || !matches!(
      children[0].kind,
      SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild
    )
  {
    return;
  }

  children[0].repeated = true;
}

fn mark_mixed_sequence_direct_children_optional(children: &mut [SchemaTypeChild]) {
  let Some(choice_child) = children
    .iter()
    .find(|child| child.kind == SchemaTypeChildKind::Choice)
  else {
    return;
  };

  let mut choice_leaf_names = std::collections::HashSet::new();
  collect_choice_child_leaf_names(choice_child, &mut choice_leaf_names);

  if choice_leaf_names.is_empty() {
    return;
  }

  for child in children.iter_mut() {
    if matches!(
      child.kind,
      SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild
    ) && !choice_leaf_names.contains(child.name.as_str())
    {
      child.optional = true;
    }
  }
}

fn have_xml_other_children_for_mixed_version_content(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
  children: &[SchemaTypeChild],
) -> bool {
  is_office2007_or_default(schema_type.version.as_deref())
    && children_need_xml_other_children_for_mixed_version_content(children, type_map)
}

fn have_xml_other_children_for_common_repeated_content(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  kind: SchemaTypeKind,
  have_xmlns_fields: bool,
  have_xml_other_attrs: bool,
  children: &[SchemaTypeChild],
) -> bool {
  kind == SchemaTypeKind::Composite
    && is_office2007_or_default(schema_type.version.as_deref())
    && is_common_ooxml_content_module(schema_type.module_name.as_str())
    && (have_xmlns_fields || have_xml_other_attrs)
    && !is_extension_schema_type(schema_type)
    && children_have_repeated_element_child(children)
}

fn have_xml_other_children_for_spreadsheet_repeated_part_root_content_child(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  kind: SchemaTypeKind,
  module_name: &str,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> bool {
  matches!(kind, SchemaTypeKind::Composite | SchemaTypeKind::Derived)
    && module_name.contains("spreadsheetml_2006_main")
    && is_office2007_or_default(schema_type.version.as_deref())
    && !is_extension_schema_type(schema_type)
    && is_repeated_child_of_part_root(schema_type.name.as_str(), module_name, type_map)
    && particle_has_any_repeated_child(&schema_type.particle)
}

fn have_xml_other_children_for_text_list_style_extension_siblings(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  kind: SchemaTypeKind,
) -> bool {
  matches!(kind, SchemaTypeKind::Composite | SchemaTypeKind::Derived)
    && is_office2007_or_default(schema_type.version.as_deref())
    && schema_type.base_class == "TextListStyleType"
    && schema_type
      .children
      .iter()
      .any(|child| is_extension_list_name(child.name.as_str()))
}

fn have_xml_other_attrs_for_mixed_version_content(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  kind: SchemaTypeKind,
  module_name: &str,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
  children: &[SchemaTypeChild],
) -> bool {
  is_part_root_schema_type(schema_type)
    || is_mce_schema_type(schema_type)
    || (is_office2007_or_default(schema_type.version.as_deref())
      && (schema_type_has_later_version_attributes(schema_type)
        || children_need_xml_other_children_for_mixed_version_content(children, type_map)
        || have_xml_other_attrs_for_spreadsheet_extensible_composite(
          schema_type,
          kind,
          module_name,
        )
        || have_xml_other_attrs_for_spreadsheet_relationship_leaf(schema_type, kind, module_name)
        || have_xml_other_attrs_for_spreadsheet_repeated_part_root_content_child(
          schema_type,
          kind,
          module_name,
          type_map,
          children,
        )
        || have_xml_other_attrs_for_word_repeated_part_root_identity_child(
          schema_type,
          kind,
          module_name,
          type_map,
        )
        || have_xml_other_attrs_for_derived_text_content(schema_type, kind, type_map)
        || particle_has_mixed_version_non_element_choice(&schema_type.particle, type_map, "")))
}

fn have_xml_other_attrs_for_derived_text_content(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  kind: SchemaTypeKind,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> bool {
  if kind != SchemaTypeKind::Derived || !schema_type.is_leaf_text {
    return false;
  }

  resolve_derived_base_type(schema_type, type_map).is_some_and(|base_type| {
    base_type.is_leaf_text
      && !base_type.attributes.is_empty()
      && base_type.base_class == "OpenXmlLeafTextElement"
  })
}

fn have_xml_other_attrs_for_spreadsheet_extensible_composite(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  kind: SchemaTypeKind,
  module_name: &str,
) -> bool {
  kind == SchemaTypeKind::Composite
    && module_name.contains("spreadsheetml_2006_main")
    && !schema_type.attributes.is_empty()
    && !is_extension_schema_type(schema_type)
    && schema_type
      .children
      .iter()
      .any(|child| is_extension_list_name(child.name.as_str()))
}

fn have_xml_other_attrs_for_spreadsheet_relationship_leaf(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  kind: SchemaTypeKind,
  module_name: &str,
) -> bool {
  kind == SchemaTypeKind::Leaf
    && module_name.contains("spreadsheetml_2006_main")
    && schema_type
      .attributes
      .iter()
      .any(|attr| attr.q_name.starts_with("r:"))
}

fn have_xml_other_attrs_for_spreadsheet_repeated_part_root_content_child(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  kind: SchemaTypeKind,
  module_name: &str,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
  children: &[SchemaTypeChild],
) -> bool {
  matches!(kind, SchemaTypeKind::Composite | SchemaTypeKind::Derived)
    && module_name.contains("spreadsheetml_2006_main")
    && !is_extension_schema_type(schema_type)
    && is_repeated_child_of_part_root(schema_type.name.as_str(), module_name, type_map)
    && (children_have_repeated_element_child(children)
      || particle_has_any_repeated_child(&schema_type.particle))
}

fn have_xml_other_attrs_for_word_repeated_part_root_identity_child(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  kind: SchemaTypeKind,
  module_name: &str,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> bool {
  kind == SchemaTypeKind::Composite
    && module_name.contains("wordprocessingml_2006_main")
    && !is_extension_schema_type(schema_type)
    && schema_type_has_required_identity_attribute(schema_type)
    && is_repeated_child_of_part_root(schema_type.name.as_str(), module_name, type_map)
}

fn schema_type_has_required_identity_attribute(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
) -> bool {
  schema_type.attributes.iter().any(|attr| {
    let local_name = attr
      .q_name
      .rsplit(':')
      .next()
      .unwrap_or(attr.q_name.as_str());
    let is_identity_attr = local_name.ends_with("Id") || local_name.ends_with("ID");
    is_identity_attr
      && attr
        .validators
        .iter()
        .any(|validator| validator.name == "RequiredValidator")
  })
}

fn is_repeated_child_of_part_root(
  child_name: &str,
  module_name: &str,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> bool {
  type_map.values().any(|parent| {
    parent.module_name == module_name
      && is_part_root_schema_type(parent)
      && particle_has_repeated_child(&parent.particle, child_name)
  })
}

fn particle_has_repeated_child(
  particle: &crate::sdk_data::open_xml::OpenXmlSchemaTypeParticle,
  child_name: &str,
) -> bool {
  let (_, repeated, _) = particle_cardinality(particle);
  particle.name == child_name && repeated
    || particle
      .items
      .iter()
      .any(|item| particle_has_repeated_child(item, child_name))
}

fn particle_has_any_repeated_child(
  particle: &crate::sdk_data::open_xml::OpenXmlSchemaTypeParticle,
) -> bool {
  let (_, repeated, _) = particle_cardinality(particle);
  !particle.name.is_empty() && repeated
    || particle.items.iter().any(particle_has_any_repeated_child)
}

fn is_part_root_schema_type(schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType) -> bool {
  !schema_type.part.is_empty() || schema_type.base_class == "OpenXmlPartRootElement"
}

fn is_mce_schema_type(schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType) -> bool {
  schema_type.name.starts_with("mc:")
}

fn is_common_ooxml_content_module(module_name: &str) -> bool {
  module_name.contains("wordprocessingml_2006_main")
    || module_name.contains("spreadsheetml_2006_main")
    || module_name.contains("presentationml_2006_main")
    || module_name.contains("drawingml_2006")
}

fn is_extension_schema_type(schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType) -> bool {
  schema_type.class_name.contains("Extension")
    || schema_type.name.ends_with("/ext")
    || schema_type.name.ends_with("/extLst")
}

fn schema_type_has_later_version_attributes(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
) -> bool {
  schema_type
    .attributes
    .iter()
    .any(|attribute| !is_office2007_or_default(Some(attribute.version.as_str())))
}

fn children_need_xml_other_children_for_mixed_version_content(
  children: &[SchemaTypeChild],
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> bool {
  children.iter().enumerate().any(|(index, child)| {
    child_position_needs_xml_other_children_for_mixed_version_content(children, index, type_map)
      || children_need_xml_other_children_for_mixed_version_content(&child.children, type_map)
  })
}

fn children_have_repeated_element_child(children: &[SchemaTypeChild]) -> bool {
  children.iter().any(|child| {
    matches!(
      child.kind,
      SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild
    ) && child.repeated
      || children_have_repeated_element_child(&child.children)
  })
}

fn is_extension_list_name(name: &str) -> bool {
  name
    .rsplit('/')
    .next()
    .and_then(|name| name.rsplit(':').next())
    == Some("extLst")
}

fn child_position_needs_xml_other_children_for_mixed_version_content(
  children: &[SchemaTypeChild],
  index: usize,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> bool {
  let child = &children[index];
  is_mixed_version_direct_element_choice(child)
    || child_has_later_initial_version(child)
    || child_type_has_mixed_version_content(child, type_map)
    || (child_type_has_later_version_attributes(child, type_map)
      && !next_child_has_later_initial_version(children, index))
}

fn child_has_later_initial_version(child: &SchemaTypeChild) -> bool {
  !is_office2007_or_default(Some(child.initial_version.as_str()))
}

fn next_child_has_later_initial_version(children: &[SchemaTypeChild], index: usize) -> bool {
  children
    .get(index + 1)
    .is_some_and(child_has_later_initial_version)
}

fn child_type_has_later_version_attributes(
  child: &SchemaTypeChild,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> bool {
  if !matches!(
    child.kind,
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild
  ) {
    return false;
  }

  let Some(schema_type) = type_map.get(child.name.as_str()) else {
    return false;
  };

  is_office2007_or_default(schema_type.version.as_deref())
    && schema_type_has_later_version_attributes(schema_type)
}

fn child_type_has_mixed_version_content(
  child: &SchemaTypeChild,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> bool {
  if !matches!(
    child.kind,
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild
  ) {
    return false;
  }

  let Some(schema_type) = type_map.get(child.name.as_str()) else {
    return false;
  };

  is_office2007_or_default(schema_type.version.as_deref())
    && particle_has_mixed_version_non_element_choice(&schema_type.particle, type_map, "")
}

fn particle_has_mixed_version_non_element_choice(
  particle: &crate::sdk_data::open_xml::OpenXmlSchemaTypeParticle,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
  inherited_initial_version: &str,
) -> bool {
  if particle.kind == "Choice" {
    let mut versions = ChoiceVersionCoverage::default();
    collect_particle_choice_version_coverage(particle, inherited_initial_version, &mut versions);
    if versions.has_default
      && versions.has_later
      && !particle_choice_leafs_are_all_element_children(particle, type_map)
    {
      return true;
    }
  }

  let initial_version =
    effective_version(inherited_initial_version, particle.initial_version.as_str());
  particle
    .items
    .iter()
    .any(|item| particle_has_mixed_version_non_element_choice(item, type_map, initial_version))
}

fn is_mixed_version_direct_element_choice(child: &SchemaTypeChild) -> bool {
  if child.kind != SchemaTypeChildKind::Choice || !choice_leafs_are_all_element_children(child) {
    return false;
  }

  let mut versions = ChoiceVersionCoverage::default();
  collect_choice_version_coverage(child, "", &mut versions);
  versions.has_default && versions.has_later
}

fn choice_leafs_are_all_element_children(child: &SchemaTypeChild) -> bool {
  let mut leaf_kinds = ChoiceLeafKinds::default();
  collect_choice_leaf_kinds(child, &mut leaf_kinds);
  leaf_kinds.has_child && !leaf_kinds.has_non_child
}

fn particle_choice_leafs_are_all_element_children(
  particle: &crate::sdk_data::open_xml::OpenXmlSchemaTypeParticle,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> bool {
  let mut leaf_kinds = ChoiceLeafKinds::default();
  collect_particle_choice_leaf_kinds(particle, type_map, &mut leaf_kinds);
  leaf_kinds.has_child && !leaf_kinds.has_non_child
}

#[derive(Default)]
struct ChoiceLeafKinds {
  has_child: bool,
  has_non_child: bool,
}

fn collect_choice_leaf_kinds(child: &SchemaTypeChild, out: &mut ChoiceLeafKinds) {
  match child.kind {
    SchemaTypeChildKind::Child => out.has_child = true,
    SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any => out.has_non_child = true,
    SchemaTypeChildKind::Choice | SchemaTypeChildKind::Sequence => {
      for nested in &child.children {
        collect_choice_leaf_kinds(nested, out);
      }
    }
  }
}

fn collect_particle_choice_leaf_kinds(
  particle: &crate::sdk_data::open_xml::OpenXmlSchemaTypeParticle,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
  out: &mut ChoiceLeafKinds,
) {
  if particle.items.is_empty() {
    match resolve_child_kind(particle.name.as_str(), type_map) {
      SchemaTypeChildKind::Child => out.has_child = true,
      SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any => out.has_non_child = true,
      SchemaTypeChildKind::Choice | SchemaTypeChildKind::Sequence => {}
    }
    return;
  }

  for item in &particle.items {
    collect_particle_choice_leaf_kinds(item, type_map, out);
  }
}

#[derive(Default)]
struct ChoiceVersionCoverage {
  has_default: bool,
  has_later: bool,
}

fn collect_choice_version_coverage(
  child: &SchemaTypeChild,
  inherited_initial_version: &str,
  out: &mut ChoiceVersionCoverage,
) {
  let initial_version =
    effective_version(inherited_initial_version, child.initial_version.as_str());

  match child.kind {
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any => {
      if is_office2007_or_default(Some(initial_version)) {
        out.has_default = true;
      } else {
        out.has_later = true;
      }
    }
    SchemaTypeChildKind::Choice | SchemaTypeChildKind::Sequence => {
      for nested in &child.children {
        collect_choice_version_coverage(nested, initial_version, out);
      }
    }
  }
}

fn collect_particle_choice_version_coverage(
  particle: &crate::sdk_data::open_xml::OpenXmlSchemaTypeParticle,
  inherited_initial_version: &str,
  out: &mut ChoiceVersionCoverage,
) {
  let initial_version =
    effective_version(inherited_initial_version, particle.initial_version.as_str());

  if particle.items.is_empty() {
    if is_office2007_or_default(Some(initial_version)) {
      out.has_default = true;
    } else {
      out.has_later = true;
    }
    return;
  }

  for item in &particle.items {
    collect_particle_choice_version_coverage(item, initial_version, out);
  }
}

fn is_office2007_or_default(version: Option<&str>) -> bool {
  matches!(version, None | Some("") | Some("Office2007"))
}

fn assign_particle_ids(children: &mut [SchemaTypeChild]) {
  assign_particle_ids_with_prefix(children, "");
}

fn assign_particle_ids_with_prefix(children: &mut [SchemaTypeChild], prefix: &str) {
  let mut child_index = 0;
  let mut text_child_index = 0;
  let mut any_index = 0;
  let mut choice_index = 0;
  let mut sequence_index = 0;

  for child in children {
    let segment = match child.kind {
      SchemaTypeChildKind::Child => {
        child_index += 1;
        format!("child_{child_index}")
      }
      SchemaTypeChildKind::TextChild => {
        text_child_index += 1;
        format!("text_child_{text_child_index}")
      }
      SchemaTypeChildKind::Any => {
        any_index += 1;
        format!("any_{any_index}")
      }
      SchemaTypeChildKind::Choice => {
        choice_index += 1;
        format!("choice_{choice_index}")
      }
      SchemaTypeChildKind::Sequence => {
        sequence_index += 1;
        format!("sequence_{sequence_index}")
      }
    };

    child.particle_id = if prefix.is_empty() {
      segment
    } else {
      format!("{prefix}/{segment}")
    };

    assign_particle_ids_with_prefix(&mut child.children, &child.particle_id);
  }
}

fn collect_choice_child_leaf_names(
  child: &SchemaTypeChild,
  out: &mut std::collections::HashSet<String>,
) {
  match child.kind {
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any => {
      if !child.name.is_empty() {
        out.insert(child.name.clone());
      }
    }
    SchemaTypeChildKind::Choice | SchemaTypeChildKind::Sequence => {
      for item in &child.children {
        collect_choice_child_leaf_names(item, out);
      }
    }
  }
}

fn resolve_child_kind(
  child_name: &str,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> SchemaTypeChildKind {
  let Some(schema_type) = resolve_schema_type(child_name, type_map) else {
    return SchemaTypeChildKind::Child;
  };

  if schema_type.base_class == "OpenXmlLeafTextElement"
    && schema_type.attributes.is_empty()
    && !schema_type.has_xmlns_fields
  {
    SchemaTypeChildKind::TextChild
  } else {
    SchemaTypeChildKind::Child
  }
}

fn resolve_schema_type<'a>(
  child_name: &str,
  type_map: &HashMap<&'a str, &'a crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> Option<&'a crate::sdk_data::open_xml::OpenXmlSchemaType> {
  let mut candidates = Vec::new();
  candidates.push(child_name.to_string());

  if let Some((left, right)) = child_name.split_once('/') {
    candidates.push(left.to_string());
    candidates.push(right.to_string());

    if let Some((_, left_local)) = left.split_once(':') {
      candidates.push(left_local.to_string());
    }

    if let Some((_, right_local)) = right.split_once(':') {
      candidates.push(right_local.to_string());
      candidates.push(right_local.to_upper_camel_case());
    }
  }

  if let Some((_, local)) = child_name.split_once(':') {
    candidates.push(local.to_string());
    candidates.push(local.to_upper_camel_case());
  }

  for candidate in candidates {
    if let Some(schema_type) = type_map.get(candidate.as_str()) {
      return Some(*schema_type);
    }
  }

  let local_name = child_name.split('/').nth(1).unwrap_or(child_name);
  let local_name_without_prefix = local_name.split(':').nth(1).unwrap_or(local_name);
  let class_name = local_name_without_prefix.to_upper_camel_case();

  type_map.values().copied().find(|schema_type| {
    schema_type.name == child_name
      || schema_type.name.ends_with(&format!("/{local_name}"))
      || schema_type
        .name
        .ends_with(&format!("/{local_name_without_prefix}"))
      || schema_type.class_name == class_name
  })
}

fn resolve_api_kind(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> SchemaTypeApiKind {
  if !schema_type.attributes.is_empty() || !schema_type.children.is_empty() {
    return SchemaTypeApiKind::Struct;
  }

  if schema_type.base_class == "OpenXmlLeafTextElement" {
    return SchemaTypeApiKind::LeafTextWrapper;
  }

  let Some(base_class_type) = resolve_derived_base_type(schema_type, type_map) else {
    return SchemaTypeApiKind::Struct;
  };

  if base_class_type.base_class == "OpenXmlLeafTextElement"
    && base_class_type.attributes.is_empty()
    && base_class_type.children.is_empty()
  {
    SchemaTypeApiKind::LeafTextWrapper
  } else {
    SchemaTypeApiKind::Struct
  }
}

fn resolve_kind(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> SchemaTypeKind {
  match schema_type.base_class.as_str() {
    "OpenXmlLeafTextElement" => SchemaTypeKind::LeafText,
    "OpenXmlLeafElement" => SchemaTypeKind::Leaf,
    "OpenXmlCompositeElement" | "CustomXmlElement" | "OpenXmlPartRootElement" | "SdtElement" => {
      SchemaTypeKind::Composite
    }
    _ if resolve_derived_base_type(schema_type, type_map).is_some() => SchemaTypeKind::Derived,
    _ => SchemaTypeKind::Struct,
  }
}

fn has_drawing_payload_xmlns_fields(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  kind: SchemaTypeKind,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> bool {
  if !can_have_xmlns_fields(kind) {
    return false;
  }

  has_core_drawing_payload_xmlns_fields(schema_type)
    || has_drawing_hyperlink_xmlns_fields(schema_type, kind, type_map)
    || has_drawing_text_payload_xmlns_fields(schema_type)
    || has_drawing_extension_payload_xmlns_fields(schema_type)
}

fn has_core_drawing_payload_xmlns_fields(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
) -> bool {
  let Some(type_name) = schema_type_name(schema_type.name.as_str()) else {
    return false;
  };

  if matches!(
    drawing_schema_type_name(schema_type.name.as_str()),
    Some(
      "CT_GraphicalObject"
        | "CT_GraphicalObjectData"
        | "CT_Blip"
        | "CT_TextBodyProperties"
        | "CT_TextBody"
        | "CT_ShapeProperties"
        | "CT_NonVisualDrawingProps"
        | "CT_NonVisualDrawingShapeProps"
        | "CT_GvmlShape"
        | "CT_GvmlShapeNonVisual"
        | "CT_Transform2D"
        | "CT_LineProperties"
        | "CT_NoFillProperties"
        | "CT_EffectList"
        | "CT_PresetGeometry2D"
        | "CT_ShapeLocking"
        | "CT_GraphicalObjectFrameLocking"
        | "CT_SpreadSheetNonVisualDrawingProps"
    )
  ) {
    return true;
  }

  is_drawing_payload_module(schema_type.module_name.as_str())
    && matches!(
      type_name,
      "CT_Drawing" | "CT_Shape" | "CT_ShapeNonVisual" | "CT_RelSizeAnchor" | "CT_Marker"
    )
}

fn has_drawing_hyperlink_xmlns_fields(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  kind: SchemaTypeKind,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> bool {
  kind == SchemaTypeKind::Derived
    && resolve_derived_base_type(schema_type, type_map).is_some_and(|base_type| {
      base_type.name == "a:CT_Hyperlink/"
        && base_type.base_class == "OpenXmlCompositeElement"
        && base_type
          .attributes
          .iter()
          .any(|attribute| attribute.q_name == "r:id")
    })
}

fn has_drawing_text_payload_xmlns_fields(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
) -> bool {
  if !is_drawing_payload_module(schema_type.module_name.as_str()) {
    return false;
  }

  matches!(
    schema_type.class_name.as_str(),
    "Paragraph"
      | "ParagraphProperties"
      | "Run"
      | "RunProperties"
      | "DefaultRunProperties"
      | "EndParagraphRunProperties"
      | "TextCharacterPropertiesType"
      | "TextParagraphPropertiesType"
      | "ComplexScriptFont"
      | "EastAsianFont"
      | "LatinFont"
      | "ListStyle"
      | "RgbColorModelHex"
      | "ShapeAutoFit"
  )
}

fn has_drawing_extension_payload_xmlns_fields(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
) -> bool {
  if !is_drawing_payload_module(schema_type.module_name.as_str()) {
    return false;
  }

  matches!(
    schema_type.class_name.as_str(),
    "ImageProperties"
      | "UseLocalDpi"
      | "Media"
      | "DefaultImageDpi"
      | "DiscardImageEditData"
      | "CreationId"
      | "ModificationId"
      | "ChartTrackingReferenceBased"
      | "ThemeFamily"
  )
}

fn drawing_schema_type_name(name: &str) -> Option<&str> {
  let (prefix, rest) = name.split_once(':')?;
  if prefix != "a" {
    return None;
  }

  rest.split_once('/').map(|(type_name, _)| type_name)
}

fn schema_type_name(name: &str) -> Option<&str> {
  name
    .split_once(':')
    .and_then(|(_, rest)| rest.split_once('/'))
    .map(|(type_name, _)| type_name)
}

fn has_extension_xmlns_fields(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  kind: SchemaTypeKind,
) -> bool {
  if !can_have_xmlns_fields(kind) {
    return false;
  }

  schema_type.class_name == "Extension"
    || schema_type.class_name.contains("Extension")
    || schema_type.name.ends_with("/ext")
    || schema_type.name.ends_with("/extLst")
}

fn has_spreadsheet_repeated_part_root_content_xmlns_fields(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  kind: SchemaTypeKind,
  module_name: &str,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> bool {
  can_have_xmlns_fields(kind)
    && matches!(kind, SchemaTypeKind::Composite | SchemaTypeKind::Derived)
    && module_name.contains("spreadsheetml_2006_main")
    && is_office2007_or_default(schema_type.version.as_deref())
    && !is_extension_schema_type(schema_type)
    && is_repeated_child_of_part_root(schema_type.name.as_str(), module_name, type_map)
    && particle_has_any_repeated_child(&schema_type.particle)
}

fn can_have_xmlns_fields(kind: SchemaTypeKind) -> bool {
  !matches!(kind, SchemaTypeKind::LeafText)
}

fn is_drawing_payload_module(module_name: &str) -> bool {
  module_name.contains("drawingml")
    || module_name.contains("office_drawing")
    || module_name.contains("powerpoint")
    || module_name.contains("thememl")
}

fn resolve_composite_kind(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
) -> SchemaTypeCompositeKind {
  if schema_type.composite_type == "OneSequence" {
    SchemaTypeCompositeKind::OneSequence
  } else if schema_type.composite_type == "OneChoice" {
    SchemaTypeCompositeKind::OneChoice
  } else if schema_type.composite_type == "OneAll" {
    SchemaTypeCompositeKind::OneAll
  } else if schema_type.particle.kind == "Sequence" {
    SchemaTypeCompositeKind::SdkSequence
  } else {
    SchemaTypeCompositeKind::None
  }
}

fn resolve_derived_base_type<'a>(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  type_map: &HashMap<&'a str, &'a crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> Option<&'a crate::sdk_data::open_xml::OpenXmlSchemaType> {
  let base_class_type_name = schema_type
    .name
    .find('/')
    .map(|index| &schema_type.name[..index + 1])?;

  if base_class_type_name == schema_type.name {
    return None;
  }

  type_map.get(base_class_type_name).copied()
}
