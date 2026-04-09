use crate::sdk_data::{
  context::Context,
  sdk_data_model::{
    Namespace, Schema, SchemaEnum, SchemaEnumFacet, SchemaType, SchemaTypeApiKind,
    SchemaTypeAttribute, SchemaTypeAttributeValidator, SchemaTypeAttributeValidatorArgument,
    SchemaTypeChild, SchemaTypeCompositeKind, SchemaTypeFixedAttribute, SchemaTypeKind,
    SchemaTypeParticle, SchemaTypeParticleOccur, SchemaTypeXmlHeader,
  },
};

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
        .map(|ty| SchemaType {
          name: ty.name.clone(),
          class_name: ty.class_name.clone(),
          summary: ty.summary.clone(),
          version: ty.version.clone(),
          part: ty.part.clone(),
          base_class: ty.base_class.clone(),
          kind: resolve_kind(ty, &type_map),
          composite_kind: resolve_composite_kind(ty),
          xml_header: SchemaTypeXmlHeader::None,
          is_abstract: ty.is_abstract,
          has_xmlns_fields: ty.has_xmlns_fields,
          has_mc_ignorable_field: ty.has_mc_ignorable_field
            || !ty.part.is_empty()
            || ty.base_class == "OpenXmlPartRootElement",
          mixed_content: false,
          text_value_type: String::new(),
          fixed_attributes: Vec::<SchemaTypeFixedAttribute>::new(),
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
              validators: attr
                .validators
                .iter()
                .map(|validator| SchemaTypeAttributeValidator {
                  name: validator.name.clone(),
                  is_list: validator.is_list,
                  r#type: validator.r#type.clone(),
                  union_id: validator.union_id,
                  is_initial_version: validator.is_initial_version,
                  arguments: validator
                    .arguments
                    .iter()
                    .map(|argument| SchemaTypeAttributeValidatorArgument {
                      name: argument.name.clone(),
                      r#type: argument.r#type.clone(),
                      value: argument.value.clone(),
                    })
                    .collect(),
                })
                .collect(),
            })
            .collect(),
          children: ty
            .children
            .iter()
            .map(|child| SchemaTypeChild {
              name: child.name.clone(),
              property_name: child.property_name.clone(),
              property_comments: child.property_comments.clone(),
            })
            .collect(),
          particle: gen_particle(&ty.particle),
          collection_sequence_root: false,
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
    require_filter: particle.require_filter,
    namespace: particle.namespace.clone(),
    occurs: particle
      .occurs
      .iter()
      .map(|occur| SchemaTypeParticleOccur {
        max: occur.max,
        min: occur.min,
        include_version: occur.include_version,
        version: occur.version.clone(),
      })
      .collect(),
    items: particle.items.iter().map(gen_particle).collect(),
  }
}
