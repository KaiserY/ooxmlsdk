use crate::sdk_data::{
  context::Context,
  sdk_data_model::{
    Namespace, Schema, SchemaEnum, SchemaEnumFacet, SchemaType, SchemaTypeApiKind,
    SchemaTypeAttribute, SchemaTypeChild, SchemaTypeChildKind, SchemaTypeCompositeKind,
    SchemaTypeKind, SchemaTypeXmlHeader,
  },
  xsd::{ParsedParticleKind, ParsedXsd},
};

use crate::sdk_code::versioning::effective_version;
use heck::ToUpperCamelCase;
use std::collections::{HashMap, HashSet};

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
    .map(|schema| {
      let prefix = gen_context
        .namespace_uri_prefix_map
        .get(&schema.target_namespace)
        .cloned()
        .unwrap_or_default();
      let on_off_qname_overrides = xsd_on_off_only_to_on_off_qname_overrides(
        prefix.as_str(),
        schema.target_namespace.as_str(),
        &gen_context.xsd_schemas,
      );
      let measure_attr_overrides =
        xsd_measure_attr_type_overrides(schema.target_namespace.as_str(), &gen_context.xsd_schemas);
      let twips_measure_element_qname_overrides = xsd_twips_measure_element_qname_overrides(
        prefix.as_str(),
        schema.target_namespace.as_str(),
        &gen_context.xsd_schemas,
        &schema.types,
      );
      let table_width_element_qname_overrides = xsd_table_width_element_qname_overrides(
        prefix.as_str(),
        schema.target_namespace.as_str(),
        &gen_context.xsd_schemas,
        &schema.types,
      );

      Schema {
        target_namespace: schema.target_namespace.clone(),
        prefix,
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
            let composite_kind = resolve_composite_kind(
              ty,
              schema.target_namespace.as_str(),
              &gen_context.xsd_schemas,
            );
            let kind = resolve_kind(ty, &type_map);
            let raw_child_map: HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaTypeChild> =
              ty.children
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
            let have_xml_other_attrs = have_xml_other_attrs_for_mixed_version_content(
              ty,
              kind,
              schema.module_name.as_str(),
              &type_map,
              &children,
            );
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
              )
              || has_mce_context_xmlns_fields(ty, kind, have_xml_other_attrs, &type_map, &children);
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
            let have_direct_xml_other_children =
              have_direct_xml_other_children_for_targeted_mce_content(
                ty,
                kind,
                schema.module_name.as_str(),
              );
            assign_particle_ids(&mut children);

            let xml_header = if !ty.part.is_empty() || ty.base_class == "OpenXmlPartRootElement" {
              SchemaTypeXmlHeader::Standalone
            } else {
              SchemaTypeXmlHeader::None
            };

            let mut schema_type = SchemaType {
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
              have_direct_xml_other_children,
              parent_choice_has_any_in: Vec::new(),
              text_value_type: text_value_type_from_type_validators(ty),
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
                    .map(|validator| {
                      crate::sdk_data::sdk_data_model::SchemaTypeAttributeValidator {
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
                      }
                    })
                    .collect(),
                  ..Default::default()
                })
                .collect(),
              children,
            };
            apply_xsd_twips_measure_element_qname_overrides(
              &mut schema_type,
              &twips_measure_element_qname_overrides,
            );
            apply_xsd_table_width_element_qname_overrides(
              &mut schema_type,
              &table_width_element_qname_overrides,
            );
            apply_xsd_twips_measure_attr_type_overrides(&mut schema_type, &measure_attr_overrides);
            apply_on_off_qname_overrides(&mut schema_type, &on_off_qname_overrides);
            schema_type
          })
          .collect(),
        enums: schema
          .enums
          .iter()
          .map(|schema_enum| SchemaEnum {
            name: schema_enum.name.clone(),
            r#type: schema_enum.r#type.clone(),
            version: schema_enum.version.clone(),
            other_variant: None,
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
      }
    })
    .collect();

  schemas
}

fn text_value_type_from_type_validators(
  ty: &crate::sdk_data::open_xml::OpenXmlSchemaType,
) -> String {
  ty.validators
    .iter()
    .find(|validator| validator.is_list && !validator.r#type.is_empty())
    .map(|validator| format!("ListValue<{}>", validator.r#type))
    .unwrap_or_default()
}

fn xsd_on_off_only_to_on_off_qname_overrides(
  prefix: &str,
  target_namespace: &str,
  xsd_schemas: &HashMap<String, ParsedXsd>,
) -> HashMap<String, String> {
  if prefix.is_empty() {
    return HashMap::new();
  }

  let Some(xsd) = xsd_schemas.get(target_namespace) else {
    return HashMap::new();
  };

  let mut xsd_element_types: HashMap<String, HashSet<String>> = HashMap::new();
  for complex_type in xsd.complex_types.values().chain(xsd.root_elements.values()) {
    for child in &complex_type.children {
      let local_name = xsd_child_local_name(child.q_name.as_str());
      let type_name = xsd_type_local_name(child.r#type.as_str());
      if !local_name.is_empty() && !type_name.is_empty() {
        xsd_element_types
          .entry(local_name.to_string())
          .or_default()
          .insert(type_name.to_string());
      }
    }
  }

  xsd_element_types
    .into_iter()
    .filter_map(|(local_name, types)| {
      if local_name == "hidden" || types.len() != 1 || !types.contains("CT_OnOff") {
        return None;
      }
      Some((
        format!("{prefix}:CT_OnOffOnly/{prefix}:{local_name}"),
        format!("{prefix}:CT_OnOff/{prefix}:{local_name}"),
      ))
    })
    .collect()
}

fn xsd_child_local_name(q_name: &str) -> &str {
  q_name.rsplit(':').next().unwrap_or(q_name)
}

fn xsd_type_local_name(type_name: &str) -> &str {
  type_name.rsplit(':').next().unwrap_or(type_name)
}

fn xsd_measure_attr_type_overrides(
  target_namespace: &str,
  xsd_schemas: &HashMap<String, ParsedXsd>,
) -> HashMap<(String, String), String> {
  let Some(xsd) = xsd_schemas.get(target_namespace) else {
    return HashMap::new();
  };

  xsd
    .complex_types
    .iter()
    .flat_map(|(complex_type_name, complex_type)| {
      complex_type.attributes.iter().filter_map(|attr| {
        xsd_measure_value_type(attr.xsd_type.as_str()).map(|value_type| {
          (
            complex_type_name.clone(),
            xsd_child_local_name(attr.q_name.as_str()).to_string(),
            value_type.to_string(),
          )
        })
      })
    })
    .map(|(complex_type_name, attr_name, value_type)| ((complex_type_name, attr_name), value_type))
    .collect()
}

fn xsd_twips_measure_element_qname_overrides(
  prefix: &str,
  target_namespace: &str,
  xsd_schemas: &HashMap<String, ParsedXsd>,
  schema_types: &[crate::sdk_data::open_xml::OpenXmlSchemaType],
) -> HashMap<String, (String, String)> {
  if prefix.is_empty() {
    return HashMap::new();
  }

  let Some(xsd) = xsd_schemas.get(target_namespace) else {
    return HashMap::new();
  };

  let mut xsd_element_types: HashMap<String, HashSet<String>> = HashMap::new();
  for complex_type in xsd.complex_types.values().chain(xsd.root_elements.values()) {
    for child in &complex_type.children {
      let type_name = xsd_type_local_name(child.r#type.as_str());
      if is_xsd_twips_measure_complex_type(type_name) {
        xsd_element_types
          .entry(xsd_child_local_name(child.q_name.as_str()).to_string())
          .or_default()
          .insert(type_name.to_string());
      }
    }
  }

  let mut data_element_counts: HashMap<String, usize> = HashMap::new();
  for schema_type in schema_types {
    if let Some(local_name) = schema_type_element_local_name(schema_type.name.as_str())
      && !local_name.is_empty()
    {
      *data_element_counts
        .entry(local_name.to_string())
        .or_default() += 1;
    }
  }

  schema_types
    .iter()
    .filter_map(|schema_type| {
      let local_name = schema_type_element_local_name(schema_type.name.as_str())?;
      if data_element_counts.get(local_name).copied() != Some(1) {
        return None;
      }

      let xsd_types = xsd_element_types.get(local_name)?;
      if xsd_types.len() != 1 {
        return None;
      }

      let xsd_type = xsd_types.iter().next()?;
      if schema_type_complex_type_name(schema_type.name.as_str()) == Some(xsd_type.as_str()) {
        return None;
      }

      let base_class = match xsd_type.as_str() {
        "CT_TwipsMeasure" => "TwipsMeasureType",
        "CT_SignedTwipsMeasure" => "SignedTwipsMeasureType",
        _ => return None,
      };

      Some((
        schema_type.name.clone(),
        (
          format!("{prefix}:{xsd_type}/{prefix}:{local_name}"),
          base_class.to_string(),
        ),
      ))
    })
    .collect()
}

fn xsd_table_width_element_qname_overrides(
  prefix: &str,
  target_namespace: &str,
  xsd_schemas: &HashMap<String, ParsedXsd>,
  schema_types: &[crate::sdk_data::open_xml::OpenXmlSchemaType],
) -> HashMap<String, (String, String)> {
  if prefix.is_empty() {
    return HashMap::new();
  }

  let Some(xsd) = xsd_schemas.get(target_namespace) else {
    return HashMap::new();
  };

  let xsd_table_width_children: HashSet<(String, String)> = xsd
    .complex_types
    .iter()
    .flat_map(|(complex_type_name, complex_type)| {
      complex_type.children.iter().filter_map(|child| {
        if xsd_type_local_name(child.r#type.as_str()) == "CT_TblWidth" {
          Some((
            complex_type_name.clone(),
            xsd_child_local_name(child.q_name.as_str()).to_string(),
          ))
        } else {
          None
        }
      })
    })
    .collect();

  let mut qnames_to_override = HashSet::new();
  for schema_type in schema_types {
    let Some(parent_complex_type_name) = schema_type_complex_type_name(schema_type.name.as_str())
    else {
      continue;
    };

    for child in &schema_type.children {
      let Some(child_complex_type_name) = schema_type_complex_type_name(child.name.as_str()) else {
        continue;
      };
      let Some(child_local_name) = schema_type_element_local_name(child.name.as_str()) else {
        continue;
      };

      if xsd_table_width_children.contains(&(
        parent_complex_type_name.to_string(),
        child_local_name.to_string(),
      )) && child_complex_type_name != "CT_TblWidth"
      {
        qnames_to_override.insert(child.name.clone());
      }
    }
  }

  qnames_to_override
    .into_iter()
    .filter_map(|qname| {
      let local_name = schema_type_element_local_name(qname.as_str())?.to_string();
      Some((
        qname,
        (
          format!("{prefix}:CT_TblWidth/{prefix}:{}", local_name),
          "TableWidthType".to_string(),
        ),
      ))
    })
    .collect()
}

fn xsd_measure_value_type(type_name: &str) -> Option<&'static str> {
  match xsd_type_local_name(type_name) {
    "ST_TwipsMeasure" => Some("TwipsMeasureValue"),
    "ST_SignedTwipsMeasure" => Some("SignedTwipsMeasureValue"),
    "ST_Coordinate" => Some("CoordinateValue"),
    "ST_Coordinate32" => Some("Coordinate32Value"),
    "ST_PositiveCoordinate" => Some("PositiveCoordinateValue"),
    "ST_PositiveCoordinate32" => Some("PositiveCoordinate32Value"),
    "ST_Percentage" => Some("DrawingmlPercentageValue"),
    "ST_PositivePercentage" => Some("PositiveDrawingmlPercentageValue"),
    "ST_FixedPercentage" => Some("FixedPercentageValue"),
    "ST_PositiveFixedPercentage" => Some("PositiveFixedPercentageValue"),
    "ST_TextBulletSize" => Some("TextBulletSizeValue"),
    "ST_TextBulletSizeDecimal" => Some("TextBulletSizeValue"),
    "ST_TextBulletSizePercent" => Some("TextBulletSizeValue"),
    "ST_TextFontScalePercentOrPercentString" => Some("TextFontScalePercentOrPercentStringValue"),
    "ST_TextFontSize" => Some("TextFontSizeValue"),
    "ST_TextPoint" => Some("TextPointValue"),
    "ST_TextSpacingPercentOrPercentString" => Some("TextSpacingPercentOrPercentStringValue"),
    "ST_TextSpacingPoint" => Some("TextSpacingPointValue"),
    "ST_DecimalNumberOrPercent" => Some("DecimalNumberOrPercentValue"),
    "ST_MeasurementOrPercent" => Some("MeasurementOrPercentValue"),
    "ST_UniversalMeasure" => Some("UniversalMeasureValue"),
    "ST_PositiveUniversalMeasure" => Some("PositiveUniversalMeasureValue"),
    _ => None,
  }
}

fn is_xsd_twips_measure_complex_type(type_name: &str) -> bool {
  matches!(type_name, "CT_TwipsMeasure" | "CT_SignedTwipsMeasure")
}

fn apply_xsd_twips_measure_attr_type_overrides(
  schema_type: &mut SchemaType,
  attr_overrides: &HashMap<(String, String), String>,
) {
  let Some(complex_type_name) = schema_type_complex_type_name(schema_type.name.as_str()) else {
    return;
  };

  for attr in &mut schema_type.attributes {
    if let Some(value_type) = attr_overrides.get(&(
      complex_type_name.to_string(),
      xsd_child_local_name(attr.q_name.as_str()).to_string(),
    )) && is_measure_override_candidate_type(attr.r#type.as_str())
    {
      attr.r#type = value_type.clone();
    }
  }
}

fn apply_xsd_twips_measure_element_qname_overrides(
  schema_type: &mut SchemaType,
  qname_overrides: &HashMap<String, (String, String)>,
) {
  if let Some((override_name, override_base_class)) = qname_overrides.get(schema_type.name.as_str())
  {
    schema_type.name = override_name.clone();
    schema_type.base_class = override_base_class.clone();
    schema_type.kind = SchemaTypeKind::Derived;
    schema_type.attributes.clear();
  }

  apply_xsd_twips_measure_element_qname_overrides_to_children(
    &mut schema_type.children,
    qname_overrides,
  );
}

fn apply_xsd_table_width_element_qname_overrides(
  schema_type: &mut SchemaType,
  qname_overrides: &HashMap<String, (String, String)>,
) {
  if let Some((override_name, override_base_class)) = qname_overrides.get(schema_type.name.as_str())
  {
    schema_type.name = override_name.clone();
    schema_type.base_class = override_base_class.clone();
    schema_type.kind = SchemaTypeKind::Derived;
    schema_type.attributes.clear();
  }

  apply_xsd_table_width_element_qname_overrides_to_children(
    &mut schema_type.children,
    qname_overrides,
  );
}

fn apply_xsd_table_width_element_qname_overrides_to_children(
  children: &mut [SchemaTypeChild],
  qname_overrides: &HashMap<String, (String, String)>,
) {
  for child in children {
    if let Some((override_name, _)) = qname_overrides.get(child.name.as_str()) {
      child.name = override_name.clone();
    }
    apply_xsd_table_width_element_qname_overrides_to_children(&mut child.children, qname_overrides);
  }
}

fn apply_xsd_twips_measure_element_qname_overrides_to_children(
  children: &mut [SchemaTypeChild],
  qname_overrides: &HashMap<String, (String, String)>,
) {
  for child in children {
    if let Some((override_name, _)) = qname_overrides.get(child.name.as_str()) {
      child.name = override_name.clone();
    }
    apply_xsd_twips_measure_element_qname_overrides_to_children(
      &mut child.children,
      qname_overrides,
    );
  }
}

fn schema_type_complex_type_name(name: &str) -> Option<&str> {
  let before_element_name = name.split_once('/')?.0;
  Some(xsd_type_local_name(before_element_name))
}

fn schema_type_element_local_name(name: &str) -> Option<&str> {
  let (_, element_name) = name.split_once('/')?;
  if element_name.is_empty() {
    None
  } else {
    Some(xsd_child_local_name(element_name))
  }
}

fn is_integer_value_type(type_name: &str) -> bool {
  matches!(
    type_name,
    "SByteValue"
      | "ByteValue"
      | "Int16Value"
      | "UInt16Value"
      | "Int32Value"
      | "UInt32Value"
      | "Int64Value"
      | "UInt64Value"
      | "IntegerValue"
  )
}

fn is_measure_override_candidate_type(type_name: &str) -> bool {
  is_integer_value_type(type_name) || type_name == "StringValue"
}

fn apply_on_off_qname_overrides(
  schema_type: &mut SchemaType,
  qname_overrides: &HashMap<String, String>,
) {
  if let Some(override_name) = qname_overrides.get(schema_type.name.as_str()) {
    schema_type.name = override_name.clone();
    if schema_type.base_class == "OnOffOnlyType" {
      schema_type.base_class = "OnOffType".to_string();
    }
  }

  apply_on_off_qname_overrides_to_children(&mut schema_type.children, qname_overrides);
}

fn apply_on_off_qname_overrides_to_children(
  children: &mut [SchemaTypeChild],
  qname_overrides: &HashMap<String, String>,
) {
  for child in children {
    if let Some(override_name) = qname_overrides.get(child.name.as_str()) {
      child.name = override_name.clone();
    }
    apply_on_off_qname_overrides_to_children(&mut child.children, qname_overrides);
  }
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
  let mut used_leaf_names = HashSet::new();
  let mut used_variant_names = HashSet::new();

  for child in children.drain(..) {
    let mut candidate_leafs = Vec::new();
    if collect_flattenable_choice_leafs(&child, &mut candidate_leafs)
      && choice_leaf_names_are_unique(&candidate_leafs, &used_leaf_names)
      && choice_variant_names_are_unique(&candidate_leafs, &used_variant_names)
    {
      for leaf in candidate_leafs {
        track_choice_leaf(&leaf, &mut used_leaf_names, &mut used_variant_names);
        flattened.push(leaf);
      }
    } else {
      track_choice_leaf(&child, &mut used_leaf_names, &mut used_variant_names);
      flattened.push(child);
    }
  }

  *children = flattened;
}

fn collect_flattenable_choice_leafs(
  child: &SchemaTypeChild,
  leafs: &mut Vec<SchemaTypeChild>,
) -> bool {
  if !is_flattenable_choice_wrapper(child) {
    return false;
  }

  let original_len = leafs.len();
  for nested in &child.children {
    if is_required_element_child(nested) {
      leafs.push(nested.clone());
    } else if !collect_flattenable_choice_leafs(nested, leafs) {
      leafs.truncate(original_len);
      return false;
    }
  }

  !child.children.is_empty()
}

fn is_flattenable_choice_wrapper(child: &SchemaTypeChild) -> bool {
  child.kind == SchemaTypeChildKind::Choice
    && child.name.is_empty()
    && child.property_name.is_empty()
    && !child.optional
    && !child.repeated
}

fn is_required_element_child(child: &SchemaTypeChild) -> bool {
  child.kind == SchemaTypeChildKind::Child
    && !child.name.is_empty()
    && !child.optional
    && !child.repeated
    && child.children.is_empty()
}

fn choice_leaf_names_are_unique(
  children: &[SchemaTypeChild],
  used_leaf_names: &HashSet<String>,
) -> bool {
  let mut names = used_leaf_names.clone();
  children
    .iter()
    .all(|child| names.insert(child.name.clone()))
}

fn choice_variant_names_are_unique(
  children: &[SchemaTypeChild],
  used_variant_names: &HashSet<String>,
) -> bool {
  let mut names = used_variant_names.clone();
  children
    .iter()
    .all(|child| names.insert(choice_child_variant_name(child)))
}

fn track_choice_leaf(
  child: &SchemaTypeChild,
  used_leaf_names: &mut HashSet<String>,
  used_variant_names: &mut HashSet<String>,
) {
  if is_required_element_child(child) {
    used_leaf_names.insert(child.name.clone());
    used_variant_names.insert(choice_child_variant_name(child));
  }
}

fn choice_child_variant_name(child: &SchemaTypeChild) -> String {
  if child.name.is_empty() {
    child.property_name.to_upper_camel_case()
  } else {
    let element_qname = child.name.split('/').nth(1).unwrap_or(child.name.as_str());
    let mut parts = element_qname.split(':');
    match (parts.next(), parts.next()) {
      (Some(prefix), Some(local)) => {
        format!(
          "{}{}",
          prefix.to_upper_camel_case(),
          local.to_upper_camel_case()
        )
      }
      (Some(local), None) => local.to_upper_camel_case(),
      _ => element_qname.to_upper_camel_case(),
    }
  }
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

fn have_direct_xml_other_children_for_targeted_mce_content(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  kind: SchemaTypeKind,
  module_name: &str,
) -> bool {
  if !matches!(kind, SchemaTypeKind::Composite | SchemaTypeKind::Derived)
    || is_extension_schema_type(schema_type)
  {
    return false;
  }

  let name = schema_type.name.as_str();
  let is_common_office2007_content = is_office2007_or_default(schema_type.version.as_deref())
    && is_common_ooxml_content_module(module_name);

  (is_common_office2007_content
    && module_name.contains("wordprocessingml_2006_main")
    && matches!(
      name,
      "w:CT_Settings/w:settings"
        | "w:CT_RPr/w:rPr"
        | "w:CT_RPrBaseStyleable/w:rPr"
        | "w:CT_PPr/w:pPr"
    ))
    || (is_common_office2007_content
      && module_name.contains("presentationml_2006_main")
      && matches!(
        name,
        "a:CT_TextListStyle/p:titleStyle"
          | "a:CT_TextListStyle/p:bodyStyle"
          | "a:CT_TextListStyle/p:otherStyle"
      ))
    || (is_common_office2007_content
      && module_name.contains("spreadsheetml_2006_main")
      && name == "x:CT_Rst/x:si")
    || (is_common_office2007_content
      && module_name.contains("drawingml_2006_main")
      && name == "a:CT_RegularTextRun/a:r")
    || (module_name.contains("microsoft_com_office_drawing_2014_chartex")
      && name == "cx:CT_ChartSpace/cx:chartSpace")
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
        || have_xml_other_attrs_for_derived_text_base(schema_type, kind, type_map)
        || have_xml_other_attrs_for_derived_text_content(schema_type, kind, type_map)
        || particle_has_mixed_version_non_element_choice(&schema_type.particle, type_map, "")))
}

fn has_mce_context_xmlns_fields(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  kind: SchemaTypeKind,
  have_xml_other_attrs: bool,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
  children: &[SchemaTypeChild],
) -> bool {
  have_xml_other_attrs
    && can_have_xmlns_fields(kind)
    && matches!(kind, SchemaTypeKind::Composite | SchemaTypeKind::Derived)
    && !is_extension_schema_type(schema_type)
    && children.iter().any(child_can_carry_mce_context)
    && (children_need_xml_other_children_for_mixed_version_content(children, type_map)
      || particle_has_any_repeated_child(&schema_type.particle)
      || particle_has_mixed_version_non_element_choice(&schema_type.particle, type_map, ""))
}

fn child_can_carry_mce_context(child: &SchemaTypeChild) -> bool {
  matches!(
    child.kind,
    SchemaTypeChildKind::Child
      | SchemaTypeChildKind::Choice
      | SchemaTypeChildKind::Sequence
      | SchemaTypeChildKind::Any
  ) || child.children.iter().any(child_can_carry_mce_context)
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

fn have_xml_other_attrs_for_derived_text_base(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  kind: SchemaTypeKind,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> bool {
  kind == SchemaTypeKind::LeafText
    && schema_type.base_class == "OpenXmlLeafTextElement"
    && !schema_type.attributes.is_empty()
    && type_map.values().any(|derived_type| {
      derived_type.is_leaf_text
        && derived_type.base_class == schema_type.class_name
        && resolve_kind(derived_type, type_map) == SchemaTypeKind::Derived
        && derived_type.attributes.is_empty()
        && derived_type.children.is_empty()
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
  target_namespace: &str,
  xsd_schemas: &HashMap<String, ParsedXsd>,
) -> SchemaTypeCompositeKind {
  if matches_xsd_repeatable_choice_rule(schema_type, target_namespace, xsd_schemas) {
    SchemaTypeCompositeKind::XsdRepeatableChoice
  } else if schema_type.composite_type == "OneSequence" {
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

fn matches_xsd_repeatable_choice_rule(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  target_namespace: &str,
  xsd_schemas: &HashMap<String, ParsedXsd>,
) -> bool {
  if !is_xsd_repeatable_choice_candidate(schema_type) {
    return false;
  }

  let Some(type_name) = schema_type_name(schema_type.name.as_str()) else {
    return false;
  };

  let Some(xsd) = xsd_schemas.get(target_namespace) else {
    return false;
  };

  let Some(complex_type) = xsd.complex_types.get(type_name) else {
    return false;
  };

  let Some(particle) = complex_type.top_level_particle else {
    return false;
  };

  particle.kind == ParsedParticleKind::Choice && particle.max_occurs == u64::MAX
}

fn is_xsd_repeatable_choice_candidate(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
) -> bool {
  schema_type.composite_type == "OneSequence"
    || (schema_type.composite_type.is_empty() && schema_type.particle.kind == "Sequence")
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

#[cfg(test)]
mod tests {
  use super::{
    gen_schemas, is_measure_override_candidate_type, matches_xsd_repeatable_choice_rule,
    resolve_composite_kind, schema_type_complex_type_name, schema_type_name, xsd_child_local_name,
    xsd_measure_attr_type_overrides, xsd_on_off_only_to_on_off_qname_overrides,
    xsd_table_width_element_qname_overrides, xsd_twips_measure_element_qname_overrides,
  };
  use crate::sdk_data::{
    context::Context,
    open_xml::{OpenXmlSchemaType, OpenXmlSchemaTypeParticle},
    sdk_data_model::{SchemaTypeChildKind, SchemaTypeCompositeKind, SchemaTypeKind},
    xsd::{parse_xsd, repeatable_choice_element_names},
  };
  use std::collections::{BTreeMap, HashMap, HashSet};
  use std::path::PathBuf;

  #[test]
  fn upgrades_one_sequence_to_xsd_repeatable_choice() {
    let mut xsd_schemas = HashMap::new();
    xsd_schemas.insert(
      "urn:test".to_string(),
      parse_xsd(
        r#"
        <xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema" targetNamespace="urn:test">
          <xsd:complexType name="CT_Font">
            <xsd:choice maxOccurs="unbounded">
              <xsd:element name="name" type="xsd:string"/>
              <xsd:element name="sz" type="xsd:string"/>
            </xsd:choice>
          </xsd:complexType>
        </xsd:schema>
        "#,
      )
      .expect("parse xsd"),
    );

    let schema_type = OpenXmlSchemaType {
      name: "x:CT_Font/x:font".to_string(),
      composite_type: "OneSequence".to_string(),
      particle: OpenXmlSchemaTypeParticle {
        kind: "Sequence".to_string(),
        ..OpenXmlSchemaTypeParticle::default()
      },
      ..OpenXmlSchemaType::default()
    };

    assert!(matches_xsd_repeatable_choice_rule(
      &schema_type,
      "urn:test",
      &xsd_schemas,
    ));
    assert_eq!(
      resolve_composite_kind(&schema_type, "urn:test", &xsd_schemas),
      SchemaTypeCompositeKind::XsdRepeatableChoice,
    );
  }

  #[test]
  fn keeps_other_one_sequence_types_unchanged() {
    let mut xsd_schemas = HashMap::new();
    xsd_schemas.insert(
      "urn:test".to_string(),
      parse_xsd(
        r#"
        <xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema" targetNamespace="urn:test">
          <xsd:complexType name="CT_Example">
            <xsd:sequence>
              <xsd:element name="name" type="xsd:string"/>
            </xsd:sequence>
          </xsd:complexType>
        </xsd:schema>
        "#,
      )
      .expect("parse xsd"),
    );

    let schema_type = OpenXmlSchemaType {
      name: "x:CT_Example/x:example".to_string(),
      composite_type: "OneSequence".to_string(),
      ..OpenXmlSchemaType::default()
    };

    assert_eq!(
      resolve_composite_kind(&schema_type, "urn:test", &xsd_schemas),
      SchemaTypeCompositeKind::OneSequence,
    );
  }

  #[test]
  fn upgrades_sdk_sequence_to_xsd_repeatable_choice() {
    let mut xsd_schemas = HashMap::new();
    xsd_schemas.insert(
      "urn:test".to_string(),
      parse_xsd(
        r#"
        <xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema" targetNamespace="urn:test">
          <xsd:complexType name="CT_RunProperties">
            <xsd:choice maxOccurs="unbounded">
              <xsd:element name="font" type="xsd:string"/>
              <xsd:element name="size" type="xsd:string"/>
            </xsd:choice>
          </xsd:complexType>
        </xsd:schema>
        "#,
      )
      .expect("parse xsd"),
    );

    let schema_type = OpenXmlSchemaType {
      name: "x:CT_RunProperties/x:rPr".to_string(),
      particle: OpenXmlSchemaTypeParticle {
        kind: "Sequence".to_string(),
        ..OpenXmlSchemaTypeParticle::default()
      },
      ..OpenXmlSchemaType::default()
    };

    assert!(matches_xsd_repeatable_choice_rule(
      &schema_type,
      "urn:test",
      &xsd_schemas,
    ));
    assert_eq!(
      resolve_composite_kind(&schema_type, "urn:test", &xsd_schemas),
      SchemaTypeCompositeKind::XsdRepeatableChoice,
    );
  }

  #[test]
  fn actual_repo_rule_matches_repeatable_choice_types() {
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
    let context = Context::new(&data_dir).expect("context");
    let schemas = gen_schemas(&context);

    let matched: Vec<(String, String)> = schemas
      .iter()
      .flat_map(|schema| {
        schema
          .types
          .iter()
          .filter(|schema_type| {
            schema_type.composite_kind == SchemaTypeCompositeKind::XsdRepeatableChoice
          })
          .map(|schema_type| (schema.module_name.clone(), schema_type.name.clone()))
      })
      .collect();

    assert_eq!(
      matched,
      vec![
        (
          "schemas_openxmlformats_org_spreadsheetml_2006_main".to_string(),
          "x:CT_RPrElt/x:rPr".to_string(),
        ),
        (
          "schemas_openxmlformats_org_spreadsheetml_2006_main".to_string(),
          "x:CT_Font/x:font".to_string(),
        ),
      ],
    );

    let font = schemas
      .iter()
      .flat_map(|schema| schema.types.iter())
      .find(|schema_type| schema_type.name == "x:CT_Font/x:font")
      .expect("font");

    assert_eq!(font.kind, SchemaTypeKind::Composite);
  }

  #[test]
  #[ignore]
  fn scan_xsd_repeated_child_backfill_candidates() {
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
    let context = Context::new(&data_dir).expect("context");
    let schemas = gen_schemas(&context);
    let mut candidates: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for schema in &schemas {
      let Some(xsd) = context.xsd_schemas.get(schema.target_namespace.as_str()) else {
        continue;
      };

      for schema_type in &schema.types {
        if schema_type.composite_kind == SchemaTypeCompositeKind::XsdRepeatableChoice {
          continue;
        }

        let Some(type_name) = schema_type_name(schema_type.name.as_str()) else {
          continue;
        };
        let repeated_names = repeatable_choice_element_names(xsd, type_name);
        if repeated_names.is_empty() {
          continue;
        }

        for child in &schema_type.children {
          if !matches!(
            child.kind,
            SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild
          ) || child.repeated
          {
            continue;
          }

          let child_local_name = xsd_child_local_name(child.name.rsplit('/').next().unwrap_or(""));
          if repeated_names.contains(child_local_name) {
            candidates
              .entry(format!(
                "{} {} {}",
                schema.module_name, schema_type.name, schema_type.class_name
              ))
              .or_default()
              .push(format!("{} {}", child.name, child.property_name));
          }
        }
      }
    }

    let child_count = candidates.values().map(Vec::len).sum::<usize>();
    println!(
      "xsd repeated child backfill candidate types: {}, children: {}",
      candidates.len(),
      child_count
    );
    for (schema_type, mut children) in candidates {
      children.sort();
      children.dedup();
      println!("{} ({})", schema_type, children.len());
      for child in &children {
        println!("  {child}");
      }
    }
  }

  #[test]
  fn mixed_sequence_choice_preserves_direct_child_occurs() {
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
    let context = Context::new(&data_dir).expect("context");
    let schemas = gen_schemas(&context);
    let schema_type = schemas
      .iter()
      .find(|schema| schema.module_name == "schemas_microsoft_com_office_powerpoint_2019_12_main")
      .and_then(|schema| {
        schema
          .types
          .iter()
          .find(|schema_type| schema_type.class_name == "TaskHistoryEvent")
      })
      .expect("TaskHistoryEvent");

    let atrbtn = schema_type
      .children
      .iter()
      .find(|child| child.name == "p1912:CT_TaskAssignUnassignUser/p1912:atrbtn")
      .expect("atrbtn child");
    assert_eq!(atrbtn.kind, SchemaTypeChildKind::Child);
    assert!(!atrbtn.optional);
    assert!(!atrbtn.repeated);

    let choice = schema_type
      .children
      .iter()
      .find(|child| child.kind == SchemaTypeChildKind::Choice)
      .expect("choice child");
    assert!(choice.optional);
    assert!(!choice.repeated);
  }

  #[test]
  fn actual_repo_upgrades_on_off_only_elements_from_xsd_except_style_hidden() {
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
    let context = Context::new(&data_dir).expect("context");
    let schemas = gen_schemas(&context);

    let word_schema = schemas
      .iter()
      .find(|schema| schema.module_name == "schemas_openxmlformats_org_wordprocessingml_2006_main")
      .expect("word schema");

    let upgraded: HashSet<String> = word_schema
      .types
      .iter()
      .filter(|schema_type| schema_type.name.starts_with("w:CT_OnOff/"))
      .filter(|schema_type| schema_type.base_class == "OnOffType")
      .map(|schema_type| schema_type.class_name.clone())
      .collect();

    for expected in [
      "AutoRedefine",
      "BiDiVisual",
      "CantSplit",
      "FlatBorders",
      "HideMark",
      "LinkedToFile",
      "Locked",
      "NoBorder",
      "NoResizeAllowed",
      "NoWrap",
      "Personal",
      "PersonalCompose",
      "PersonalReply",
      "PrimaryStyle",
      "SemiHidden",
      "TableCellFitText",
      "TableHeader",
      "UnhideWhenUsed",
    ] {
      assert!(upgraded.contains(expected), "missing {expected}");
    }

    let style_hidden = word_schema
      .types
      .iter()
      .find(|schema_type| schema_type.class_name == "StyleHidden")
      .expect("StyleHidden");
    assert_eq!(style_hidden.name, "w:CT_OnOffOnly/w:hidden");
    assert_eq!(style_hidden.base_class, "OnOffOnlyType");
  }

  #[test]
  fn actual_repo_upgrades_measure_attrs_from_xsd_to_simple_union_types() {
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
    let context = Context::new(&data_dir).expect("context");

    let word_raw_schema = context
      .schemas
      .iter()
      .find(|schema| schema.module_name == "schemas_openxmlformats_org_wordprocessingml_2006_main")
      .expect("raw word schema");
    let attr_overrides = xsd_measure_attr_type_overrides(
      word_raw_schema.target_namespace.as_str(),
      &context.xsd_schemas,
    );

    let raw_measure_candidates: Vec<(&str, &str, &str, String)> = word_raw_schema
      .types
      .iter()
      .flat_map(|schema_type| {
        schema_type.attributes.iter().filter_map(|attr| {
          let complex_type_name = schema_type_complex_type_name(schema_type.name.as_str())?;
          let expected_type = attr_overrides.get(&(
            complex_type_name.to_string(),
            xsd_child_local_name(attr.q_name.as_str()).to_string(),
          ))?;
          if is_measure_override_candidate_type(attr.r#type.as_str()) {
            Some((
              schema_type.class_name.as_str(),
              attr.property_name.as_str(),
              attr.r#type.as_str(),
              expected_type.clone(),
            ))
          } else {
            None
          }
        })
      })
      .collect();

    assert!(raw_measure_candidates.len() >= 19);

    let schemas = gen_schemas(&context);
    let word_schema = schemas
      .iter()
      .find(|schema| schema.module_name == "schemas_openxmlformats_org_wordprocessingml_2006_main")
      .expect("word schema");

    for (class_name, property_name, raw_type, expected_type) in raw_measure_candidates {
      let schema_type = word_schema
        .types
        .iter()
        .find(|schema_type| schema_type.class_name == class_name)
        .unwrap_or_else(|| panic!("missing generated type {class_name}"));
      let attr = schema_type
        .attributes
        .iter()
        .find(|attr| attr.property_name == property_name)
        .unwrap_or_else(|| panic!("missing generated attr {class_name}.{property_name}"));
      assert_eq!(
        attr.r#type, expected_type,
        "{class_name}.{property_name} raw type was {raw_type}"
      );
    }

    let page_size = word_schema
      .types
      .iter()
      .find(|schema_type| schema_type.class_name == "PageSize")
      .expect("PageSize");
    assert_eq!(
      page_size
        .attributes
        .iter()
        .find(|attr| attr.property_name == "Code")
        .expect("PageSize.Code")
        .r#type,
      "UInt16Value"
    );

    let columns = word_schema
      .types
      .iter()
      .find(|schema_type| schema_type.class_name == "Columns")
      .expect("Columns");
    assert_eq!(
      columns
        .attributes
        .iter()
        .find(|attr| attr.property_name == "Space")
        .expect("Columns.Space")
        .r#type,
      "TwipsMeasureValue"
    );

    let drawing_schema = schemas
      .iter()
      .find(|schema| schema.module_name == "schemas_openxmlformats_org_drawingml_2006_main")
      .expect("drawing schema");
    for (class_name, property_name, expected_type) in [
      ("GradientStop", "Position", "PositiveFixedPercentageValue"),
      ("RelativeRectangleType", "Left", "DrawingmlPercentageValue"),
      (
        "NormalAutoFit",
        "FontScale",
        "TextFontScalePercentOrPercentStringValue",
      ),
      (
        "NormalAutoFit",
        "LineSpaceReduction",
        "TextSpacingPercentOrPercentStringValue",
      ),
      (
        "SpacingPercent",
        "Val",
        "TextSpacingPercentOrPercentStringValue",
      ),
    ] {
      let schema_type = drawing_schema
        .types
        .iter()
        .find(|schema_type| schema_type.class_name == class_name)
        .unwrap_or_else(|| panic!("missing drawing type {class_name}"));
      let attr = schema_type
        .attributes
        .iter()
        .find(|attr| attr.property_name == property_name)
        .unwrap_or_else(|| panic!("missing drawing attr {class_name}.{property_name}"));
      assert_eq!(attr.r#type, expected_type, "{class_name}.{property_name}");
    }
  }

  #[test]
  fn actual_repo_upgrades_uniquely_identified_twips_measure_elements_from_xsd() {
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
    let context = Context::new(&data_dir).expect("context");

    let word_raw_schema = context
      .schemas
      .iter()
      .find(|schema| schema.module_name == "schemas_openxmlformats_org_wordprocessingml_2006_main")
      .expect("raw word schema");
    let prefix = context
      .namespace_uri_prefix_map
      .get(&word_raw_schema.target_namespace)
      .expect("word prefix");
    let qname_overrides = xsd_twips_measure_element_qname_overrides(
      prefix,
      word_raw_schema.target_namespace.as_str(),
      &context.xsd_schemas,
      &word_raw_schema.types,
    );

    assert_eq!(qname_overrides.len(), 1);
    assert_eq!(
      qname_overrides
        .get("w:CT_NonNegativeShort/w:defaultTabStop")
        .map(|(qname, base_class)| (qname.as_str(), base_class.as_str())),
      Some(("w:CT_TwipsMeasure/w:defaultTabStop", "TwipsMeasureType"))
    );
    assert!(!qname_overrides.contains_key("w:CT_TextScale/w:w"));

    let schemas = gen_schemas(&context);
    let word_schema = schemas
      .iter()
      .find(|schema| schema.module_name == "schemas_openxmlformats_org_wordprocessingml_2006_main")
      .expect("word schema");

    let default_tab_stop = word_schema
      .types
      .iter()
      .find(|schema_type| schema_type.class_name == "DefaultTabStop")
      .expect("DefaultTabStop");
    assert_eq!(default_tab_stop.name, "w:CT_TwipsMeasure/w:defaultTabStop");
    assert_eq!(default_tab_stop.base_class, "TwipsMeasureType");

    let character_scale = word_schema
      .types
      .iter()
      .find(|schema_type| schema_type.class_name == "CharacterScale")
      .expect("CharacterScale");
    assert_eq!(character_scale.name, "w:CT_TextScale/w:w");
  }

  #[test]
  fn actual_repo_upgrades_table_width_elements_from_xsd_parent_child_rules() {
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
    let context = Context::new(&data_dir).expect("context");

    let word_raw_schema = context
      .schemas
      .iter()
      .find(|schema| schema.module_name == "schemas_openxmlformats_org_wordprocessingml_2006_main")
      .expect("raw word schema");
    let prefix = context
      .namespace_uri_prefix_map
      .get(&word_raw_schema.target_namespace)
      .expect("word prefix");
    let qname_overrides = xsd_table_width_element_qname_overrides(
      prefix,
      word_raw_schema.target_namespace.as_str(),
      &context.xsd_schemas,
      &word_raw_schema.types,
    );

    assert_eq!(qname_overrides.len(), 3);
    for (from, to, base_class) in [
      (
        "w:CT_TblWidthDxaNil/w:left",
        "w:CT_TblWidth/w:left",
        "TableWidthType",
      ),
      (
        "w:CT_TblWidthDxaNil/w:right",
        "w:CT_TblWidth/w:right",
        "TableWidthType",
      ),
      (
        "w:CT_TblWidthShort/w:tblInd",
        "w:CT_TblWidth/w:tblInd",
        "TableWidthType",
      ),
    ] {
      assert_eq!(
        qname_overrides
          .get(from)
          .map(|(qname, base_class)| (qname.as_str(), base_class.as_str())),
        Some((to, base_class))
      );
    }
    assert!(!qname_overrides.contains_key("w:CT_Border/w:left"));
    assert!(!qname_overrides.contains_key("w:CT_Border/w:right"));
    assert!(!qname_overrides.contains_key("w:CT_Border/w:top"));
    assert!(!qname_overrides.contains_key("w:CT_Border/w:bottom"));

    let schemas = gen_schemas(&context);
    let word_schema = schemas
      .iter()
      .find(|schema| schema.module_name == "schemas_openxmlformats_org_wordprocessingml_2006_main")
      .expect("word schema");

    for (class_name, expected_qname) in [
      ("TableCellLeftMargin", "w:CT_TblWidth/w:left"),
      ("TableCellRightMargin", "w:CT_TblWidth/w:right"),
      ("TableIndentation", "w:CT_TblWidth/w:tblInd"),
    ] {
      let schema_type = word_schema
        .types
        .iter()
        .find(|schema_type| schema_type.class_name == class_name)
        .unwrap_or_else(|| panic!("missing {class_name}"));
      assert_eq!(schema_type.name, expected_qname);
      assert_eq!(schema_type.base_class, "TableWidthType");
      assert_eq!(schema_type.kind, SchemaTypeKind::Derived);
      assert!(schema_type.attributes.is_empty());
    }
  }

  #[test]
  fn on_off_xsd_upgrade_map_excludes_ambiguous_style_hidden() {
    let mut xsd_schemas = HashMap::new();
    xsd_schemas.insert(
      "urn:test".to_string(),
      parse_xsd(
        r#"
        <xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema" targetNamespace="urn:test">
          <xsd:complexType name="CT_Style">
            <xsd:sequence>
              <xsd:element name="hidden" type="CT_OnOff" minOccurs="0"/>
              <xsd:element name="qFormat" type="CT_OnOff" minOccurs="0"/>
            </xsd:sequence>
          </xsd:complexType>
        </xsd:schema>
        "#,
      )
      .expect("parse xsd"),
    );

    let overrides = xsd_on_off_only_to_on_off_qname_overrides("w", "urn:test", &xsd_schemas);
    assert_eq!(
      overrides
        .get("w:CT_OnOffOnly/w:qFormat")
        .map(String::as_str),
      Some("w:CT_OnOff/w:qFormat")
    );
    assert!(!overrides.contains_key("w:CT_OnOffOnly/w:hidden"));
  }

  #[test]
  fn actual_repo_does_not_auto_generate_direct_xml_other_children_for_parent_choice_cases() {
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
    let context = Context::new(&data_dir).expect("context");
    let schemas = gen_schemas(&context);

    let direct_other_children: Vec<String> = schemas
      .iter()
      .flat_map(|schema| schema.types.iter())
      .filter(|schema_type| schema_type.have_direct_xml_other_children)
      .map(|schema_type| schema_type.name.clone())
      .collect();

    assert!(!direct_other_children.iter().any(|name| {
      matches!(
        name.as_str(),
        "w:CT_FontsList/w:fonts"
          | "w:CT_Body/w:body"
          | "w:CT_P/w:p"
          | "w:CT_R/w:r"
          | "xdr:CT_Drawing/xdr:wsDr"
          | "x:CT_Controls/x:controls"
          | "p:CT_ControlList/p:controls"
      )
    }));
  }

  #[test]
  fn actual_repo_keeps_targeted_direct_xml_other_children_cases() {
    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
    let context = Context::new(&data_dir).expect("context");
    let schemas = gen_schemas(&context);

    let direct_other_children: HashSet<String> = schemas
      .iter()
      .flat_map(|schema| schema.types.iter())
      .filter(|schema_type| schema_type.have_direct_xml_other_children)
      .map(|schema_type| schema_type.name.clone())
      .collect();

    for expected in [
      "w:CT_Settings/w:settings",
      "w:CT_RPr/w:rPr",
      "w:CT_RPrBaseStyleable/w:rPr",
      "w:CT_PPr/w:pPr",
      "x:CT_Rst/x:si",
      "a:CT_RegularTextRun/a:r",
      "cx:CT_ChartSpace/cx:chartSpace",
    ] {
      assert!(
        direct_other_children.contains(expected),
        "missing {expected}"
      );
    }
  }
}
