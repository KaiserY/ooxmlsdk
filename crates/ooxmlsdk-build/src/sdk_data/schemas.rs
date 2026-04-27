use crate::sdk_data::{
  context::Context,
  sdk_data_model::{
    Namespace, Schema, SchemaEnum, SchemaEnumFacet, SchemaType, SchemaTypeApiKind,
    SchemaTypeAttribute, SchemaTypeChild, SchemaTypeChildKind, SchemaTypeCompositeKind,
    SchemaTypeKind, SchemaTypeXmlHeader,
  },
  xsd::{parse_named_groups, parse_xsd, resolve_named_group_leaf_sets},
};

use heck::{ToSnakeCase, ToUpperCamelCase};
use std::collections::{BTreeSet, HashMap};

const MCE_ALTERNATE_CONTENT_NAME: &str = "mc:CT_AlternateContent/mc:AlternateContent";

#[derive(Clone, Debug)]
struct StableGroupSignature {
  property_name: String,
  leaf_qnames: BTreeSet<String>,
  match_kind: StableGroupMatchKind,
}

#[derive(Clone, Debug)]
enum StableGroupMatchKind {
  Exact,
  Compatible {
    allowed_extra_leafs: BTreeSet<String>,
  },
}

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
  let has_mce_alternate_content =
    type_map.contains_key("mc:CT_AlternateContent/mc:AlternateContent");
  let stable_group_signatures = load_stable_group_signatures(gen_context);

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
            let preserve_nested_choice_wrappers = matches!(
              composite_kind,
              SchemaTypeCompositeKind::OneSequence | SchemaTypeCompositeKind::SdkSequence
            );
            collect_choice_children(
              &ty.particle,
              &raw_child_map,
              &type_map,
              &mut children,
              false,
              false,
              preserve_nested_choice_wrappers,
            );
          }
          assign_stable_group_names(&mut children, &stable_group_signatures);
          mark_sequence_collection_children_repeated(ty, &mut children);
          mark_mixed_sequence_direct_children_optional(&mut children);
          if has_mce_alternate_content {
            insert_mce_alternate_content_for_mixed_version_choices(ty, &mut children);
          }
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

pub(crate) fn assign_mce_alternate_content_property_names(schemas: &mut [Schema]) {
  for schema in schemas {
    for schema_type in &mut schema.types {
      assign_mce_alternate_content_property_names_in_children(&mut schema_type.children);
    }
  }
}

fn assign_mce_alternate_content_property_names_in_children(children: &mut [SchemaTypeChild]) {
  let mut count = 0;
  for child in children.iter_mut() {
    if is_mce_alternate_content_child(child) {
      count += 1;
      child.property_name = if count == 1 {
        "mc_alternate_content".to_string()
      } else {
        format!("mc_alternate_content{count}")
      };
    }
  }

  for child in children {
    match child.kind {
      SchemaTypeChildKind::Sequence => {
        assign_mce_alternate_content_property_names_in_children(&mut child.children);
      }
      SchemaTypeChildKind::Choice => {
        assign_mce_alternate_content_property_names_in_choice(&mut child.children);
      }
      SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any => {}
    }
  }
}

fn assign_mce_alternate_content_property_names_in_choice(children: &mut [SchemaTypeChild]) {
  for child in children {
    if is_mce_alternate_content_child(child) {
      child.property_name = "mc_alternate_content".to_string();
    }

    match child.kind {
      SchemaTypeChildKind::Sequence => {
        assign_mce_alternate_content_property_names_in_children(&mut child.children);
      }
      SchemaTypeChildKind::Choice => {
        assign_mce_alternate_content_property_names_in_choice(&mut child.children);
      }
      SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any => {}
    }
  }
}

fn load_stable_group_signatures(gen_context: &Context) -> Vec<StableGroupSignature> {
  let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let workspace_root = manifest_dir
    .parent()
    .and_then(|path| path.parent())
    .expect("workspace root");
  let xsd_dir = workspace_root.join("schemas/OfficeOpenXML-XMLSchema-Transitional");
  let Ok(entries) = std::fs::read_dir(&xsd_dir) else {
    return Vec::new();
  };

  let mut group_particles = std::collections::BTreeMap::new();

  for entry in entries.flatten() {
    let path = entry.path();
    if !path.is_file() || path.extension() != Some(std::ffi::OsStr::new("xsd")) {
      continue;
    }

    let Ok(source) = std::fs::read_to_string(&path) else {
      continue;
    };
    let Ok(parsed_xsd) = parse_xsd(&source) else {
      continue;
    };
    let Some(default_prefix) = gen_context
      .namespace_uri_prefix_map
      .get(parsed_xsd.target_namespace.as_str())
    else {
      continue;
    };
    let Ok(parsed_groups) = parse_named_groups(&source, default_prefix.as_str()) else {
      continue;
    };

    group_particles.extend(parsed_groups.groups);
  }

  let group_leaf_sets = resolve_named_group_leaf_sets(&group_particles);
  let omath_math_elements = group_leaf_sets
    .get("m:EG_OMathMathElements")
    .cloned()
    .unwrap_or_default();

  group_leaf_sets
    .into_iter()
    .filter_map(|(group_ref, leaf_qnames)| {
      if !matches!(group_ref.split(':').next(), Some("w") | Some("m")) {
        return None;
      }

      let local_name = group_ref.rsplit(':').next().unwrap_or(group_ref.as_str());
      if !local_name.starts_with("EG_") || leaf_qnames.is_empty() {
        return None;
      }

      let match_kind = if group_ref == "w:EG_RunLevelElts" {
        StableGroupMatchKind::Compatible {
          allowed_extra_leafs: omath_math_elements
            .iter()
            .cloned()
            .chain(
              [
                "w:contentPart",
                "w14:customXmlConflictInsRangeStart",
                "w14:customXmlConflictInsRangeEnd",
                "w14:customXmlConflictDelRangeStart",
                "w14:customXmlConflictDelRangeEnd",
                "w14:conflictIns",
                "w14:conflictDel",
              ]
              .into_iter()
              .map(str::to_string),
            )
            .collect(),
        }
      } else {
        StableGroupMatchKind::Exact
      };

      Some(StableGroupSignature {
        property_name: local_name.to_snake_case(),
        leaf_qnames,
        match_kind,
      })
    })
    .collect()
}

fn assign_stable_group_names(
  children: &mut [SchemaTypeChild],
  signatures: &[StableGroupSignature],
) {
  for child in children {
    assign_stable_group_name(child, signatures);
  }
}

fn assign_stable_group_name(child: &mut SchemaTypeChild, signatures: &[StableGroupSignature]) {
  for nested in &mut child.children {
    assign_stable_group_name(nested, signatures);
  }

  if !child.property_name.is_empty()
    || !matches!(
      child.kind,
      SchemaTypeChildKind::Choice | SchemaTypeChildKind::Sequence
    )
  {
    return;
  }

  if let Some(property_name) = infer_stable_group_name(child, signatures) {
    child.property_name = property_name;
  }
}

fn infer_stable_group_name(
  child: &SchemaTypeChild,
  signatures: &[StableGroupSignature],
) -> Option<String> {
  let leaf_qnames = child_leaf_qnames(child);
  if leaf_qnames.is_empty() {
    return None;
  }

  let exact_matches = signatures
    .iter()
    .filter(|signature| {
      matches!(signature.match_kind, StableGroupMatchKind::Exact)
        && signature.leaf_qnames == leaf_qnames
    })
    .collect::<Vec<_>>();
  if let Some(property_name) = select_unique_group_name(exact_matches) {
    return Some(property_name);
  }

  let mut compatible_matches = signatures
    .iter()
    .filter(|signature| match &signature.match_kind {
      StableGroupMatchKind::Exact => false,
      StableGroupMatchKind::Compatible {
        allowed_extra_leafs,
      } => {
        signature.leaf_qnames.is_subset(&leaf_qnames)
          && leaf_qnames
            .difference(&signature.leaf_qnames)
            .all(|leaf| allowed_extra_leafs.contains(leaf))
      }
    })
    .collect::<Vec<_>>();

  compatible_matches.sort_by_key(|right| std::cmp::Reverse(right.leaf_qnames.len()));

  if compatible_matches.is_empty() {
    return None;
  }

  let best_len = compatible_matches[0].leaf_qnames.len();
  let best_matches = compatible_matches
    .into_iter()
    .take_while(|signature| signature.leaf_qnames.len() == best_len)
    .collect::<Vec<_>>();

  select_unique_group_name(best_matches)
}

fn select_unique_group_name(matches: Vec<&StableGroupSignature>) -> Option<String> {
  let mut property_names = matches
    .into_iter()
    .map(|signature| signature.property_name.as_str())
    .collect::<Vec<_>>();

  property_names.sort_unstable();
  property_names.dedup();

  (property_names.len() == 1).then(|| property_names[0].to_string())
}

fn child_leaf_qnames(child: &SchemaTypeChild) -> BTreeSet<String> {
  let mut leaf_qnames = BTreeSet::new();
  collect_child_leaf_qnames(child, &mut leaf_qnames);
  leaf_qnames
}

fn collect_child_leaf_qnames(child: &SchemaTypeChild, out: &mut BTreeSet<String>) {
  match child.kind {
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild => {
      if !child.name.is_empty() {
        out.insert(
          child
            .name
            .rsplit('/')
            .next()
            .unwrap_or(child.name.as_str())
            .to_string(),
        );
      }
    }
    SchemaTypeChildKind::Choice | SchemaTypeChildKind::Sequence => {
      for nested in &child.children {
        collect_child_leaf_qnames(nested, out);
      }
    }
    SchemaTypeChildKind::Any => {}
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
            effective_initial_version(initial_version.as_str(), child.initial_version.as_str())
              .to_string();
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
        nested.initial_version = effective_initial_version(
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

pub(crate) fn normalize_schema_type_children(children: &mut [SchemaTypeChild]) {
  for child in children.iter_mut() {
    normalize_choice_wrappers(child, false);
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

fn insert_mce_alternate_content_for_mixed_version_choices(
  schema_type: &crate::sdk_data::open_xml::OpenXmlSchemaType,
  children: &mut Vec<SchemaTypeChild>,
) {
  if !is_office2007_or_default(schema_type.version.as_deref()) {
    return;
  }

  insert_mce_alternate_content_in_children(children);
}

fn insert_mce_alternate_content_in_children(children: &mut Vec<SchemaTypeChild>) {
  let children = if children.len() == 1 && children[0].kind == SchemaTypeChildKind::Sequence {
    &mut children[0].children
  } else {
    children
  };

  let mut index = 0;
  while index < children.len() {
    if is_mixed_version_choice(&children[index], "") {
      if children[index].repeated {
        insert_mce_alternate_content_choice_variant(&mut children[index]);
        index += 1;
      } else if index > 0 && is_mce_alternate_content_child(&children[index - 1]) {
        index += 1;
      } else {
        children.insert(index, mce_alternate_content_child(true));
        index += 2;
      }
    } else {
      index += 1;
    }
  }
}

fn insert_mce_alternate_content_choice_variant(choice: &mut SchemaTypeChild) {
  if choice.children.iter().any(is_mce_alternate_content_child) {
    return;
  }

  choice
    .children
    .insert(0, mce_alternate_content_child(false));
}

fn mce_alternate_content_child(optional: bool) -> SchemaTypeChild {
  SchemaTypeChild {
    particle_id: String::new(),
    name: MCE_ALTERNATE_CONTENT_NAME.to_string(),
    property_name: "mc_alternate_content".to_string(),
    property_comments: String::new(),
    kind: SchemaTypeChildKind::Child,
    optional,
    repeated: false,
    initial_version: String::new(),
    children: Vec::new(),
  }
}

fn is_mce_alternate_content_child(child: &SchemaTypeChild) -> bool {
  child.name == MCE_ALTERNATE_CONTENT_NAME
}

fn is_mixed_version_choice(child: &SchemaTypeChild, inherited_initial_version: &str) -> bool {
  if child.kind != SchemaTypeChildKind::Choice {
    return false;
  }

  let mut versions = ChoiceVersionCoverage::default();
  collect_choice_version_coverage(child, inherited_initial_version, &mut versions);
  versions.has_default && versions.has_later
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
    effective_initial_version(inherited_initial_version, child.initial_version.as_str());

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
