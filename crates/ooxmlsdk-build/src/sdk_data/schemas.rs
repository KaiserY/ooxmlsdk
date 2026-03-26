use crate::sdk_data::{
  context::Context,
  sdk_data_model::{
    Schema, SchemaEnum, SchemaEnumFacet, SchemaType, SchemaTypeAttribute,
    SchemaTypeAttributeValidator, SchemaTypeAttributeValidatorArgument, SchemaTypeChild,
    SchemaTypeParticle, SchemaTypeParticleOccur,
  },
};

pub fn gen_schemas(gen_context: &Context) -> Vec<Schema> {
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
          composite_type: ty.composite_type.clone(),
          base_class: ty.base_class.clone(),
          is_leaf_text: ty.is_leaf_text,
          is_leaf_element: ty.is_leaf_element,
          is_derived: ty.is_derived,
          is_abstract: ty.is_abstract,
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
              schema_module: gen_context
                .type_name_module_name_map
                .get(&child.name)
                .cloned(),
            })
            .collect(),
          particle: gen_particle(&ty.particle),
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
