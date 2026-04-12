use crate::sdk_data::{
  context::Context,
  sdk_data_model::{
    Namespace, Schema, SchemaEnum, SchemaEnumFacet, SchemaType, SchemaTypeApiKind,
    SchemaTypeAttribute, SchemaTypeChild, SchemaTypeChildKind, SchemaTypeCompositeKind,
    SchemaTypeKind, SchemaTypeParticle, SchemaTypeParticleOccur, SchemaTypeXmlHeader,
  },
};

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
          let has_xmlns_fields =
            ty.has_xmlns_fields || !ty.part.is_empty() || ty.base_class == "OpenXmlPartRootElement";
          let raw_child_map: HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaTypeChild> = ty
            .children
            .iter()
            .map(|child| (child.name.as_str(), child))
            .collect();
          let mut children = Vec::new();
          if ty.particle.kind == "All" {
            children.extend(ty.children.iter().map(|child| SchemaTypeChild {
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
            );
          }
          mark_sequence_collection_children_repeated(ty, &mut children);
          mark_mixed_sequence_direct_children_optional(&mut children);

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
            kind: resolve_kind(ty, &type_map),
            composite_kind: resolve_composite_kind(ty),
            xml_header,
            is_abstract: ty.is_abstract,
            has_xmlns_fields,
            has_mc_ignorable_field: ty.has_mc_ignorable_field
              || !ty.part.is_empty()
              || ty.base_class == "OpenXmlPartRootElement",
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
              })
              .collect(),
            children,
            particle: gen_particle(&ty.particle),
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
            })
            .collect(),
        })
        .collect(),
    })
    .collect();

  schemas
}

fn collect_choice_children(
  particle: &crate::sdk_data::open_xml::OpenXmlSchemaTypeParticle,
  raw_child_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaTypeChild>,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
  out: &mut Vec<SchemaTypeChild>,
  inherited_optional: bool,
  inherited_repeated: bool,
) {
  let (optional, repeated, initial_version) = particle_cardinality(particle);
  let optional = inherited_optional || optional;
  let repeated = inherited_repeated || repeated;

  match particle.kind.as_str() {
    "Any" => {
      out.push(SchemaTypeChild {
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
          optional,
          repeated,
        );
      }
      if !variants.is_empty() {
        out.push(SchemaTypeChild {
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
      for item in &particle.items {
        collect_choice_children(item, raw_child_map, type_map, out, optional, repeated);
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
) {
  let (optional, repeated, initial_version) = particle_cardinality(particle);
  let optional = inherited_optional || optional;
  let repeated = inherited_repeated || repeated;

  match particle.kind.as_str() {
    "Any" => {
      out.push(SchemaTypeChild {
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
          optional,
          repeated,
        );
      }
      if !variants.is_empty() {
        out.push(SchemaTypeChild {
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
          optional,
          repeated,
        );
      }
      if !sequence_children.is_empty() {
        if sequence_children.len() == 1 && sequence_children[0].kind == SchemaTypeChildKind::Choice
        {
          let mut child = sequence_children.remove(0);
          child.optional |= optional;
          child.repeated |= repeated;
          child.initial_version = initial_version;
          out.push(child);
          return;
        }
        out.push(SchemaTypeChild {
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

fn schema_child_from_particle(
  particle: &crate::sdk_data::open_xml::OpenXmlSchemaTypeParticle,
  raw_child_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaTypeChild>,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> Option<SchemaTypeChild> {
  if particle.name.is_empty() {
    if particle.kind == "Any" {
      return Some(SchemaTypeChild {
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

  let (property_name, property_comments) = raw_child_map
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
  let is_collection_wrapper = kind == SchemaTypeChildKind::Child
    && property_name.is_empty()
    && particle.kind == "Sequence"
    && particle.items.len() == 1;

  Some(SchemaTypeChild {
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

fn gen_particle(
  particle: &crate::sdk_data::open_xml::OpenXmlSchemaTypeParticle,
) -> SchemaTypeParticle {
  SchemaTypeParticle {
    kind: particle.kind.clone(),
    name: particle.name.clone(),
    initial_version: particle.initial_version.clone(),
    occurs: particle
      .occurs
      .iter()
      .map(|occur| SchemaTypeParticleOccur {
        max: occur.max,
        min: occur.min,
        version: occur.version.clone(),
      })
      .collect(),
    items: particle.items.iter().map(gen_particle).collect(),
  }
}
