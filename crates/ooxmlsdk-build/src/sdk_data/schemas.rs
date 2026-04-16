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
          let composite_kind = resolve_composite_kind(ty);
          let has_xmlns_fields =
            ty.has_xmlns_fields || !ty.part.is_empty() || ty.base_class == "OpenXmlPartRootElement";
          let raw_child_map: HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaTypeChild> = ty
            .children
            .iter()
            .map(|child| (child.name.as_str(), child))
            .collect();
          let mut children = Vec::new();
          if ty.name == "w:CT_P/w:p" {
            children.extend(gen_hardcoded_paragraph_children(
              ty,
              &raw_child_map,
              &type_map,
            ));
          } else if composite_kind == SchemaTypeCompositeKind::OneChoice {
            children.extend(gen_one_choice_children(ty, &raw_child_map, &type_map));
          } else if ty.particle.kind == "All" {
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
            composite_kind,
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
              ..Default::default()
            })
            .collect(),
        })
        .collect(),
    })
    .collect();

  schemas
}

fn gen_hardcoded_paragraph_children(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  raw_child_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaTypeChild>,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
) -> Vec<SchemaTypeChild> {
  let paragraph_properties = hardcoded_schema_child(
    schema_type,
    raw_child_map,
    type_map,
    "w:CT_PPr/w:pPr",
    true,
    false,
    "",
  );

  let eg_range_markup_elements = SchemaTypeChild {
    name: String::new(),
    property_name: "eg_range_markup_elements".to_string(),
    property_comments: String::new(),
    kind: SchemaTypeChildKind::Choice,
    optional: false,
    repeated: false,
    initial_version: String::new(),
    children: vec![
      SchemaTypeChild {
        name: String::new(),
        property_name: "choice1".to_string(),
        property_comments: String::new(),
        kind: SchemaTypeChildKind::Choice,
        optional: false,
        repeated: false,
        initial_version: String::new(),
        children: [
          "w:CT_Bookmark/w:bookmarkStart",
          "w:CT_MarkupRange/w:bookmarkEnd",
          "w:CT_MarkupRange/w:commentRangeStart",
          "w:CT_MarkupRange/w:commentRangeEnd",
        ]
        .into_iter()
        .map(|name| {
          hardcoded_schema_child(schema_type, raw_child_map, type_map, name, false, false, "")
        })
        .collect(),
      },
      SchemaTypeChild {
        name: String::new(),
        property_name: "choice2".to_string(),
        property_comments: String::new(),
        kind: SchemaTypeChildKind::Choice,
        optional: false,
        repeated: false,
        initial_version: String::new(),
        children: [
          "w:CT_MoveBookmark/w:moveFromRangeStart",
          "w:CT_MarkupRange/w:moveFromRangeEnd",
          "w:CT_MoveBookmark/w:moveToRangeStart",
          "w:CT_MarkupRange/w:moveToRangeEnd",
          "w:CT_TrackChange/w:customXmlInsRangeStart",
          "w:CT_Markup/w:customXmlInsRangeEnd",
          "w:CT_TrackChange/w:customXmlDelRangeStart",
          "w:CT_Markup/w:customXmlDelRangeEnd",
          "w:CT_TrackChange/w:customXmlMoveFromRangeStart",
          "w:CT_Markup/w:customXmlMoveFromRangeEnd",
          "w:CT_TrackChange/w:customXmlMoveToRangeStart",
          "w:CT_Markup/w:customXmlMoveToRangeEnd",
        ]
        .into_iter()
        .map(|name| {
          hardcoded_schema_child(schema_type, raw_child_map, type_map, name, false, false, "")
        })
        .collect(),
      },
    ],
  };

  let run_level_range_choice = SchemaTypeChild {
    name: String::new(),
    property_name: "choice1".to_string(),
    property_comments: String::new(),
    kind: SchemaTypeChildKind::Choice,
    optional: false,
    repeated: false,
    initial_version: String::new(),
    children: vec![
      eg_range_markup_elements,
      hardcoded_schema_child(
        schema_type,
        raw_child_map,
        type_map,
        "w:CT_TrackChange/w14:customXmlConflictInsRangeStart",
        false,
        false,
        "Office2010",
      ),
      hardcoded_schema_child(
        schema_type,
        raw_child_map,
        type_map,
        "w:CT_Markup/w14:customXmlConflictInsRangeEnd",
        false,
        false,
        "Office2010",
      ),
      hardcoded_schema_child(
        schema_type,
        raw_child_map,
        type_map,
        "w:CT_TrackChange/w14:customXmlConflictDelRangeStart",
        false,
        false,
        "Office2010",
      ),
      hardcoded_schema_child(
        schema_type,
        raw_child_map,
        type_map,
        "w:CT_Markup/w14:customXmlConflictDelRangeEnd",
        false,
        false,
        "Office2010",
      ),
    ],
  };

  let eg_omath_math_elements = SchemaTypeChild {
    name: String::new(),
    property_name: "eg_omath_math_elements".to_string(),
    property_comments: String::new(),
    kind: SchemaTypeChildKind::Choice,
    optional: false,
    repeated: false,
    initial_version: String::new(),
    children: vec![
      "m:CT_Acc/m:acc",
      "m:CT_Bar/m:bar",
      "m:CT_Box/m:box",
      "m:CT_BorderBox/m:borderBox",
      "m:CT_D/m:d",
      "m:CT_EqArr/m:eqArr",
      "m:CT_F/m:f",
      "m:CT_Func/m:func",
      "m:CT_GroupChr/m:groupChr",
      "m:CT_LimLow/m:limLow",
      "m:CT_LimUpp/m:limUpp",
      "m:CT_M/m:m",
      "m:CT_Nary/m:nary",
      "m:CT_Phant/m:phant",
      "m:CT_Rad/m:rad",
      "m:CT_SPre/m:sPre",
      "m:CT_SSub/m:sSub",
      "m:CT_SSubSup/m:sSubSup",
      "m:CT_SSup/m:sSup",
    ]
    .into_iter()
    .map(|name| {
      hardcoded_schema_child(schema_type, raw_child_map, type_map, name, false, false, "")
    })
    .chain(std::iter::once(SchemaTypeChild {
      name: String::new(),
      property_name: "choice1".to_string(),
      property_comments: String::new(),
      kind: SchemaTypeChildKind::Choice,
      optional: false,
      repeated: false,
      initial_version: String::new(),
      children: vec![hardcoded_schema_child(
        schema_type,
        raw_child_map,
        type_map,
        "m:CT_R/m:r",
        false,
        false,
        "",
      )],
    }))
    .collect(),
  };

  let eg_math_content = SchemaTypeChild {
    name: String::new(),
    property_name: "eg_math_content".to_string(),
    property_comments: String::new(),
    kind: SchemaTypeChildKind::Choice,
    optional: false,
    repeated: false,
    initial_version: String::new(),
    children: vec![
      hardcoded_schema_child(
        schema_type,
        raw_child_map,
        type_map,
        "m:CT_OMathPara/m:oMathPara",
        false,
        false,
        "",
      ),
      hardcoded_schema_child(
        schema_type,
        raw_child_map,
        type_map,
        "m:CT_OMath/m:oMath",
        false,
        false,
        "",
      ),
    ],
  };

  let math_content_choice = SchemaTypeChild {
    name: String::new(),
    property_name: "choice2".to_string(),
    property_comments: String::new(),
    kind: SchemaTypeChildKind::Choice,
    optional: false,
    repeated: false,
    initial_version: String::new(),
    children: vec![eg_math_content, eg_omath_math_elements],
  };

  let eg_run_level_elts = SchemaTypeChild {
    name: String::new(),
    property_name: "eg_run_level_elts".to_string(),
    property_comments: String::new(),
    kind: SchemaTypeChildKind::Choice,
    optional: false,
    repeated: false,
    initial_version: String::new(),
    children: vec![
      SchemaTypeChild {
        name: String::new(),
        property_name: "choice1".to_string(),
        property_comments: String::new(),
        kind: SchemaTypeChildKind::Choice,
        optional: false,
        repeated: false,
        initial_version: String::new(),
        children: vec![
          hardcoded_schema_child(
            schema_type,
            raw_child_map,
            type_map,
            "w:CT_ProofErr/w:proofErr",
            false,
            false,
            "",
          ),
          hardcoded_schema_child(
            schema_type,
            raw_child_map,
            type_map,
            "w:CT_PermStart/w:permStart",
            false,
            false,
            "",
          ),
          hardcoded_schema_child(
            schema_type,
            raw_child_map,
            type_map,
            "w:CT_Perm/w:permEnd",
            false,
            false,
            "",
          ),
          run_level_range_choice,
          hardcoded_schema_child(
            schema_type,
            raw_child_map,
            type_map,
            "w:CT_RunTrackChange/w:ins",
            false,
            false,
            "",
          ),
          hardcoded_schema_child(
            schema_type,
            raw_child_map,
            type_map,
            "w:CT_RunTrackChange/w:del",
            false,
            false,
            "",
          ),
          hardcoded_schema_child(
            schema_type,
            raw_child_map,
            type_map,
            "w:CT_RunTrackChange/w:moveFrom",
            false,
            false,
            "",
          ),
          hardcoded_schema_child(
            schema_type,
            raw_child_map,
            type_map,
            "w:CT_RunTrackChange/w:moveTo",
            false,
            false,
            "",
          ),
          hardcoded_schema_child(
            schema_type,
            raw_child_map,
            type_map,
            "w:CT_ContentPart/w:contentPart",
            false,
            false,
            "Office2010",
          ),
          SchemaTypeChild {
            name: String::new(),
            property_name: "sequence1".to_string(),
            property_comments: String::new(),
            kind: SchemaTypeChildKind::Sequence,
            optional: false,
            repeated: false,
            initial_version: "Office2010".to_string(),
            children: vec![
              hardcoded_schema_child(
                schema_type,
                raw_child_map,
                type_map,
                "w:CT_RunTrackChange/w14:conflictIns",
                false,
                false,
                "Office2010",
              ),
              hardcoded_schema_child(
                schema_type,
                raw_child_map,
                type_map,
                "w:CT_RunTrackChange/w14:conflictDel",
                false,
                false,
                "Office2010",
              ),
            ],
          },
        ],
      },
      math_content_choice,
    ],
  };

  let complex_run_content_choice = SchemaTypeChild {
    name: String::new(),
    property_name: "choice2".to_string(),
    property_comments: String::new(),
    kind: SchemaTypeChildKind::Choice,
    optional: false,
    repeated: false,
    initial_version: String::new(),
    children: vec![
      SchemaTypeChild {
        name: String::new(),
        property_name: "choice1".to_string(),
        property_comments: String::new(),
        kind: SchemaTypeChildKind::Choice,
        optional: false,
        repeated: false,
        initial_version: String::new(),
        children: vec![
          hardcoded_schema_child(
            schema_type,
            raw_child_map,
            type_map,
            "w:CT_SdtRun/w:sdt",
            false,
            false,
            "",
          ),
          eg_run_level_elts,
        ],
      },
      hardcoded_schema_child(
        schema_type,
        raw_child_map,
        type_map,
        "w:CT_R/w:r",
        false,
        false,
        "",
      ),
      hardcoded_schema_child(
        schema_type,
        raw_child_map,
        type_map,
        "w:CT_BdoContentRun/w:bdo",
        false,
        false,
        "Office2010",
      ),
      hardcoded_schema_child(
        schema_type,
        raw_child_map,
        type_map,
        "w:CT_DirContentRun/w:dir",
        false,
        false,
        "Office2010",
      ),
    ],
  };

  let eg_p_content_children = vec![
    SchemaTypeChild {
      name: String::new(),
      property_name: "eg_p_content_base".to_string(),
      property_comments: String::new(),
      kind: SchemaTypeChildKind::Choice,
      optional: false,
      repeated: false,
      initial_version: String::new(),
      children: vec![
        hardcoded_schema_child(
          schema_type,
          raw_child_map,
          type_map,
          "w:CT_CustomXmlRun/w:customXml",
          false,
          false,
          "",
        ),
        hardcoded_schema_child(
          schema_type,
          raw_child_map,
          type_map,
          "w:CT_SimpleField/w:fldSimple",
          false,
          false,
          "",
        ),
        hardcoded_schema_child(
          schema_type,
          raw_child_map,
          type_map,
          "w:CT_Hyperlink/w:hyperlink",
          false,
          false,
          "",
        ),
      ],
    },
    complex_run_content_choice,
    hardcoded_schema_child(
      schema_type,
      raw_child_map,
      type_map,
      "w:CT_Rel/w:subDoc",
      false,
      false,
      "",
    ),
  ];

  vec![
    paragraph_properties,
    SchemaTypeChild {
      name: String::new(),
      property_name: "eg_p_content".to_string(),
      property_comments: String::new(),
      kind: SchemaTypeChildKind::Choice,
      optional: false,
      repeated: true,
      initial_version: String::new(),
      children: eg_p_content_children,
    },
  ]
}

fn hardcoded_schema_child(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  raw_child_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaTypeChild>,
  type_map: &HashMap<&str, &crate::sdk_data::open_xml::OpenXmlSchemaType>,
  name: &str,
  optional: bool,
  repeated: bool,
  initial_version: &str,
) -> SchemaTypeChild {
  let raw_child = raw_child_map.get(name).unwrap_or_else(|| {
    panic!(
      "missing hardcoded child {name} for schema {}",
      schema_type.name
    )
  });

  SchemaTypeChild {
    name: raw_child.name.clone(),
    property_name: raw_child.property_name.clone(),
    property_comments: raw_child.property_comments.clone(),
    kind: resolve_child_kind(name, type_map),
    optional,
    repeated,
    initial_version: initial_version.to_string(),
    children: Vec::new(),
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
        collect_choice_variant_children(item, raw_child_map, type_map, out, false, false);
      }
    }
    "Group" | "Sequence" => {
      for item in &particle.items {
        collect_one_choice_variants(item, raw_child_map, type_map, out);
      }
    }
    "Any" | "" => {
      collect_choice_variant_children(particle, raw_child_map, type_map, out, false, false);
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
      normalize_choice_children(&mut variants);
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
          child.initial_version =
            effective_initial_version(initial_version.as_str(), child.initial_version.as_str())
              .to_string();
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

fn normalize_choice_children(children: &mut Vec<SchemaTypeChild>) {
  for child in children.iter_mut() {
    normalize_choice_wrappers(child);
  }

  flatten_anonymous_choice_children(children);
}

fn normalize_choice_wrappers(child: &mut SchemaTypeChild) {
  match child.kind {
    SchemaTypeChildKind::Choice => normalize_choice_children(&mut child.children),
    SchemaTypeChildKind::Sequence => {
      for nested in child.children.iter_mut() {
        normalize_choice_wrappers(nested);
      }
      collapse_single_anonymous_sequence_child(child);
    }
    _ => {}
  }
}

fn flatten_anonymous_choice_children(children: &mut Vec<SchemaTypeChild>) {
  let mut flattened = Vec::with_capacity(children.len());

  for child in children.drain(..) {
    if is_anonymous_wrapper(&child, SchemaTypeChildKind::Choice) {
      flattened.extend(child.children);
    } else {
      flattened.push(child);
    }
  }

  *children = flattened;
}

pub(crate) fn normalize_schema_type_children(children: &mut [SchemaTypeChild]) {
  for child in children.iter_mut() {
    normalize_choice_wrappers(child);
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
  child.initial_version = effective_initial_version(
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

fn effective_initial_version<'a>(left: &'a str, right: &'a str) -> &'a str {
  match (left, right) {
    ("", version) => version,
    (version, "") => version,
    (left, right) if left == right => left,
    (left, right) => {
      if is_microsoft365_version(left) {
        left
      } else if is_microsoft365_version(right) {
        right
      } else {
        left
      }
    }
  }
}

fn is_microsoft365_version(version: &str) -> bool {
  matches!(
    version,
    "Office2010" | "Office2013" | "Office2016" | "Office2019" | "Office2021" | "Microsoft365"
  )
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

#[cfg(test)]
mod tests {
  use super::*;

  fn child(name: &str) -> SchemaTypeChild {
    SchemaTypeChild {
      name: name.to_string(),
      property_name: String::new(),
      property_comments: String::new(),
      kind: SchemaTypeChildKind::Child,
      optional: true,
      repeated: true,
      initial_version: String::new(),
      children: Vec::new(),
    }
  }

  fn anonymous_wrapper(
    kind: SchemaTypeChildKind,
    children: Vec<SchemaTypeChild>,
  ) -> SchemaTypeChild {
    SchemaTypeChild {
      name: String::new(),
      property_name: String::new(),
      property_comments: String::new(),
      kind,
      optional: true,
      repeated: true,
      initial_version: String::new(),
      children,
    }
  }

  #[test]
  fn flattens_anonymous_choice_children() {
    let mut children = vec![
      anonymous_wrapper(
        SchemaTypeChildKind::Choice,
        vec![
          child("w:CT_CustomXmlRun/w:customXml"),
          child("w:CT_SimpleField/w:fldSimple"),
        ],
      ),
      child("w:CT_Rel/w:subDoc"),
    ];

    normalize_choice_children(&mut children);

    assert_eq!(children.len(), 3);
    assert_eq!(children[0].name, "w:CT_CustomXmlRun/w:customXml");
    assert_eq!(children[1].name, "w:CT_SimpleField/w:fldSimple");
    assert_eq!(children[2].name, "w:CT_Rel/w:subDoc");
  }

  #[test]
  fn preserves_named_choice_wrappers() {
    let mut children = vec![SchemaTypeChild {
      name: String::new(),
      property_name: "eg_p_content".to_string(),
      property_comments: String::new(),
      kind: SchemaTypeChildKind::Choice,
      optional: false,
      repeated: true,
      initial_version: String::new(),
      children: vec![child("w:CT_R/w:r")],
    }];

    normalize_choice_children(&mut children);

    assert_eq!(children.len(), 1);
    assert_eq!(children[0].property_name, "eg_p_content");
    assert_eq!(children[0].kind, SchemaTypeChildKind::Choice);
  }

  #[test]
  fn recursively_flattens_anonymous_choice_wrappers_inside_named_choices() {
    let mut children = vec![SchemaTypeChild {
      name: String::new(),
      property_name: "eg_p_content".to_string(),
      property_comments: String::new(),
      kind: SchemaTypeChildKind::Choice,
      optional: false,
      repeated: true,
      initial_version: String::new(),
      children: vec![anonymous_wrapper(
        SchemaTypeChildKind::Choice,
        vec![child("w:CT_R/w:r"), child("m:CT_OMath/m:oMath")],
      )],
    }];

    normalize_choice_children(&mut children);

    assert_eq!(children.len(), 1);
    assert_eq!(children[0].property_name, "eg_p_content");
    assert_eq!(children[0].children.len(), 2);
    assert_eq!(children[0].children[0].name, "w:CT_R/w:r");
    assert_eq!(children[0].children[1].name, "m:CT_OMath/m:oMath");
  }

  #[test]
  fn collapses_nested_anonymous_sequence_wrapper() {
    let mut child = anonymous_wrapper(
      SchemaTypeChildKind::Sequence,
      vec![anonymous_wrapper(
        SchemaTypeChildKind::Sequence,
        vec![
          child("w:CT_RunTrackChange/w14:conflictIns"),
          child("w:CT_RunTrackChange/w14:conflictDel"),
        ],
      )],
    );
    child.initial_version = "Office2010".to_string();

    normalize_choice_wrappers(&mut child);

    assert_eq!(child.kind, SchemaTypeChildKind::Sequence);
    assert_eq!(child.initial_version, "Office2010");
    assert_eq!(child.children.len(), 2);
    assert_eq!(
      child.children[0].name,
      "w:CT_RunTrackChange/w14:conflictIns"
    );
    assert_eq!(
      child.children[1].name,
      "w:CT_RunTrackChange/w14:conflictDel"
    );
  }
}
