use crate::sdk_code::versioning::effective_version;
use crate::sdk_data::compatibility::{
  preserve_namespace_decls_rule_for_schema, preserve_namespace_decls_rule_for_type,
};
use crate::sdk_data::sdk_data_model::{
  CompatibilityRule, Schema, SchemaType, SchemaTypeApiKind, SchemaTypeCompositeKind,
  SchemaTypeKind, SchemaTypeParticle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SimpleValueKind {
  StringLike,
  BoolLike,
  NumericLike,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttrTypeKind<'a> {
  List,
  Enum {
    typed_namespace: &'a str,
    enum_name: &'a str,
  },
  Simple {
    simple_type: &'a str,
    value_kind: SimpleValueKind,
  },
}

pub fn is_composite_type(schema_type: &SchemaType) -> bool {
  schema_type.kind == SchemaTypeKind::Composite
}

pub fn is_leaf_text_wrapper(schema_type: &SchemaType) -> bool {
  schema_type.api_kind == SchemaTypeApiKind::LeafTextWrapper
}

pub fn is_leaf_text_type(schema_type: &SchemaType) -> bool {
  schema_type.kind == SchemaTypeKind::LeafText
}

pub fn is_leaf_element_type(schema_type: &SchemaType) -> bool {
  schema_type.kind == SchemaTypeKind::Leaf
}

pub fn is_derived_type(schema_type: &SchemaType) -> bool {
  schema_type.kind == SchemaTypeKind::Derived
}

pub fn needs_xml_header(schema_type: &SchemaType) -> bool {
  schema_type.xml_header_standalone.is_some()
    || !schema_type.part.is_empty()
    || schema_type.base_class == "OpenXmlPartRootElement"
}

pub fn supports_compat_xmlns_fields(
  schema_type: &SchemaType,
  schema: &Schema,
  compatibility_rules: &[CompatibilityRule],
) -> bool {
  schema_type.has_xmlns_fields
    || needs_xml_header(schema_type)
    || preserve_namespace_decls_rule_for_schema(compatibility_rules, &schema.module_name).is_some()
    || preserve_namespace_decls_rule_for_type(
      compatibility_rules,
      &schema.module_name,
      &schema_type.class_name,
    )
    .is_some()
    || preserve_namespace_decls_rule_for_type(
      compatibility_rules,
      &schema.module_name,
      &schema_type.name,
    )
    .is_some()
}

pub fn is_collection_sequence_root(schema_type: &SchemaType) -> bool {
  schema_type.collection_sequence_root
}

fn supports_extended_sequence_strategy(schema_type: &SchemaType) -> bool {
  matches!(
    schema_type.composite_kind,
    SchemaTypeCompositeKind::OneSequence | SchemaTypeCompositeKind::SdkSequence
  )
}

pub fn is_one_sequence_flatten(schema_type: &SchemaType) -> bool {
  if supports_extended_sequence_strategy(schema_type) {
    schema_type
      .particle
      .items
      .iter()
      .all(can_flatten_one_sequence_particle)
  } else {
    false
  }
}

fn can_flatten_one_sequence_particle(particle: &SchemaTypeParticle) -> bool {
  match particle.kind.as_str() {
    "" => particle.items.is_empty() && !particle.name.is_empty(),
    "Choice" => {
      !particle.items.is_empty()
        && particle
          .items
          .iter()
          .all(|item| item.kind.is_empty() && item.items.is_empty() && !item.name.is_empty())
    }
    "Group" | "Sequence" => particle.items.iter().all(can_flatten_one_sequence_particle),
    _ => false,
  }
}

#[derive(Clone, Debug)]
pub enum FlatParticleKind<'a> {
  Leaf(&'a SchemaTypeParticle),
  Choice(&'a SchemaTypeParticle),
}

#[derive(Clone, Debug)]
pub struct FlatParticle<'a> {
  pub kind: FlatParticleKind<'a>,
  pub optional: bool,
  pub repeated: bool,
  pub initial_version: &'a str,
}

#[derive(Clone, Debug)]
pub enum StructuredParticleKind<'a> {
  Leaf(&'a SchemaTypeParticle),
  Choice(StructuredChoice<'a>),
}

#[derive(Clone, Debug)]
pub struct StructuredParticle<'a> {
  pub kind: StructuredParticleKind<'a>,
  pub optional: bool,
  pub repeated: bool,
  pub initial_version: &'a str,
}

#[derive(Clone, Debug)]
pub struct StructuredChoice<'a> {
  pub variants: Vec<StructuredChoiceVariant<'a>>,
}

#[derive(Clone, Debug)]
pub enum StructuredChoiceVariant<'a> {
  Leaf(&'a SchemaTypeParticle),
  Sequence(Vec<StructuredParticle<'a>>),
}

pub fn flatten_one_sequence_particles(schema_type: &SchemaType) -> Vec<FlatParticle<'_>> {
  let mut flat_particles = Vec::new();
  let parent_repeated = is_collection_sequence_root(schema_type);

  for particle in &schema_type.particle.items {
    flatten_one_sequence_particle(particle, false, parent_repeated, "", &mut flat_particles);
  }

  flat_particles
}

pub fn is_one_sequence_structurable(schema_type: &SchemaType) -> bool {
  if supports_extended_sequence_strategy(schema_type) {
    schema_type
      .particle
      .items
      .iter()
      .all(can_structure_one_sequence_particle)
  } else {
    false
  }
}

pub fn structure_one_sequence_particles(schema_type: &SchemaType) -> Vec<StructuredParticle<'_>> {
  let mut particles = Vec::new();
  let parent_repeated = is_collection_sequence_root(schema_type);

  for particle in &schema_type.particle.items {
    structure_one_sequence_particle(particle, false, parent_repeated, "", &mut particles);
  }

  particles
}

fn flatten_one_sequence_particle<'a>(
  particle: &'a SchemaTypeParticle,
  parent_optional: bool,
  parent_repeated: bool,
  parent_initial_version: &'a str,
  out: &mut Vec<FlatParticle<'a>>,
) {
  let occurs = particle.occurs.first();
  let optional = parent_optional || occurs.is_some_and(|occur| occur.min == 0);
  let repeated = parent_repeated || occurs.is_some_and(|occur| occur.max != 1);
  let initial_version =
    effective_version(parent_initial_version, particle.initial_version.as_str());

  match particle.kind.as_str() {
    "" => {
      if !particle.name.is_empty() {
        out.push(FlatParticle {
          kind: FlatParticleKind::Leaf(particle),
          optional,
          repeated,
          initial_version,
        });
      }
    }
    "Choice" => {
      if !particle.items.is_empty() {
        out.push(FlatParticle {
          kind: FlatParticleKind::Choice(particle),
          optional,
          repeated,
          initial_version,
        });
      }
    }
    "Group" | "Sequence" => {
      for child in &particle.items {
        flatten_one_sequence_particle(child, optional, repeated, initial_version, out);
      }
    }
    _ => {}
  }
}

fn can_structure_one_sequence_particle(particle: &SchemaTypeParticle) -> bool {
  match particle.kind.as_str() {
    "" => particle.items.is_empty() && !particle.name.is_empty(),
    "Choice" => {
      let mut leaf_name_set = std::collections::HashSet::new();

      !particle.items.is_empty()
        && particle
          .items
          .iter()
          .all(can_structure_one_sequence_choice_variant)
        && particle
          .items
          .iter()
          .all(|item| collect_structured_choice_leaf_names(item, &mut leaf_name_set))
    }
    "Group" | "Sequence" => particle
      .items
      .iter()
      .all(can_structure_one_sequence_particle),
    _ => false,
  }
}

fn can_structure_one_sequence_choice_variant(particle: &SchemaTypeParticle) -> bool {
  match particle.kind.as_str() {
    "" => particle.items.is_empty() && !particle.name.is_empty(),
    "Choice" => {
      !particle.items.is_empty()
        && particle
          .items
          .iter()
          .all(can_structure_one_sequence_choice_variant)
    }
    "Group" | "Sequence" => {
      can_structure_sequence_variant_particle(particle)
        || (particle.items.len() == 1
          && can_structure_one_sequence_choice_variant(&particle.items[0]))
    }
    _ => false,
  }
}

fn can_structure_sequence_variant_particle(particle: &SchemaTypeParticle) -> bool {
  let occurs = particle.occurs.first();
  if occurs.is_some_and(|occur| occur.max != 1) {
    return false;
  }

  match particle.kind.as_str() {
    "" => particle.items.is_empty() && !particle.name.is_empty(),
    "Group" | "Sequence" => particle
      .items
      .iter()
      .all(can_structure_sequence_variant_particle),
    _ => false,
  }
}

fn collect_structured_choice_leaf_names<'a>(
  particle: &'a SchemaTypeParticle,
  out: &mut std::collections::HashSet<&'a str>,
) -> bool {
  match particle.kind.as_str() {
    "" => !particle.name.is_empty() && out.insert(particle.name.as_str()),
    "Choice" => {
      !particle.items.is_empty()
        && particle
          .items
          .iter()
          .all(|item| collect_structured_choice_leaf_names(item, out))
    }
    "Group" | "Sequence" => {
      if can_structure_sequence_variant_particle(particle) {
        true
      } else if particle.items.len() == 1 {
        collect_structured_choice_leaf_names(&particle.items[0], out)
      } else {
        false
      }
    }
    _ => false,
  }
}

fn structure_one_sequence_particle<'a>(
  particle: &'a SchemaTypeParticle,
  parent_optional: bool,
  parent_repeated: bool,
  parent_initial_version: &'a str,
  out: &mut Vec<StructuredParticle<'a>>,
) {
  let occurs = particle.occurs.first();
  let optional = parent_optional || occurs.is_some_and(|occur| occur.min == 0);
  let repeated = parent_repeated || occurs.is_some_and(|occur| occur.max != 1);
  let initial_version =
    effective_version(parent_initial_version, particle.initial_version.as_str());

  match particle.kind.as_str() {
    "" => {
      if !particle.name.is_empty() {
        out.push(StructuredParticle {
          kind: StructuredParticleKind::Leaf(particle),
          optional,
          repeated,
          initial_version,
        });
      }
    }
    "Choice" => {
      let mut variants = Vec::new();

      for item in &particle.items {
        variants.extend(structure_one_sequence_choice_variant(item, initial_version));
      }

      if !variants.is_empty() {
        out.push(StructuredParticle {
          kind: StructuredParticleKind::Choice(StructuredChoice { variants }),
          optional,
          repeated,
          initial_version,
        });
      }
    }
    "Group" | "Sequence" => {
      for child in &particle.items {
        structure_one_sequence_particle(child, optional, repeated, initial_version, out);
      }
    }
    _ => {}
  }
}

fn structure_one_sequence_choice_variant<'a>(
  particle: &'a SchemaTypeParticle,
  parent_initial_version: &'a str,
) -> Vec<StructuredChoiceVariant<'a>> {
  let initial_version =
    effective_version(parent_initial_version, particle.initial_version.as_str());

  match particle.kind.as_str() {
    "" => (!particle.name.is_empty())
      .then_some(StructuredChoiceVariant::Leaf(particle))
      .into_iter()
      .collect(),
    "Choice" => particle
      .items
      .iter()
      .flat_map(|child| structure_one_sequence_choice_variant(child, initial_version))
      .collect(),
    "Group" | "Sequence" => {
      if can_structure_sequence_variant_particle(particle) {
        let mut particles = Vec::new();

        for child in &particle.items {
          structure_sequence_variant_particle(child, false, false, initial_version, &mut particles);
        }

        (!particles.is_empty())
          .then_some(StructuredChoiceVariant::Sequence(particles))
          .into_iter()
          .collect()
      } else if particle.items.len() == 1 {
        structure_one_sequence_choice_variant(&particle.items[0], initial_version)
      } else {
        Vec::new()
      }
    }
    _ => Vec::new(),
  }
}

fn structure_sequence_variant_particle<'a>(
  particle: &'a SchemaTypeParticle,
  parent_optional: bool,
  parent_repeated: bool,
  parent_initial_version: &'a str,
  out: &mut Vec<StructuredParticle<'a>>,
) {
  let occurs = particle.occurs.first();
  let optional = parent_optional || occurs.is_some_and(|occur| occur.min == 0);
  let repeated = parent_repeated || occurs.is_some_and(|occur| occur.max != 1);
  let initial_version =
    effective_version(parent_initial_version, particle.initial_version.as_str());

  match particle.kind.as_str() {
    "" => {
      if !particle.name.is_empty() {
        out.push(StructuredParticle {
          kind: StructuredParticleKind::Leaf(particle),
          optional,
          repeated,
          initial_version,
        });
      }
    }
    "Group" | "Sequence" => {
      for child in &particle.items {
        structure_sequence_variant_particle(child, optional, repeated, initial_version, out);
      }
    }
    _ => {}
  }
}

pub fn classify_simple_type(simple_type: &str) -> Option<SimpleValueKind> {
  match simple_type {
    "Base64BinaryValue" | "DateTimeValue" | "DecimalValue" | "HexBinaryValue" | "IntegerValue"
    | "SByteValue" | "StringValue" => Some(SimpleValueKind::StringLike),
    "BooleanValue" | "OnOffValue" | "TrueFalseBlankValue" | "TrueFalseValue" => {
      Some(SimpleValueKind::BoolLike)
    }
    "ByteValue" | "Int16Value" | "Int32Value" | "Int64Value" | "UInt16Value" | "UInt32Value"
    | "UInt64Value" | "DoubleValue" | "SingleValue" => Some(SimpleValueKind::NumericLike),
    _ => None,
  }
}

pub fn classify_attr_type(attr_type: &str) -> Option<AttrTypeKind<'_>> {
  if attr_type.starts_with("ListValue<") {
    return Some(AttrTypeKind::List);
  }

  if attr_type.starts_with("EnumValue<") {
    let typed_namespace = &attr_type[attr_type.find('<')? + 1..attr_type.rfind('.')?];
    let enum_name = &attr_type[attr_type.rfind('.')? + 1..attr_type.len() - 1];
    return Some(AttrTypeKind::Enum {
      typed_namespace,
      enum_name,
    });
  }

  Some(AttrTypeKind::Simple {
    simple_type: attr_type,
    value_kind: classify_simple_type(attr_type)?,
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::sdk_data::sdk_data_model::{
    SchemaType, SchemaTypeCompositeKind, SchemaTypeParticle, SchemaTypeParticleOccur,
  };

  fn leaf(name: &str, min: u64, max: u64, version: &'static str) -> SchemaTypeParticle {
    SchemaTypeParticle {
      kind: String::new(),
      name: name.to_string(),
      occurs: vec![SchemaTypeParticleOccur {
        max,
        min,
        include_version: false,
        version: String::new(),
      }],
      items: vec![],
      initial_version: version.to_string(),
      require_filter: false,
      namespace: String::new(),
    }
  }

  fn node(
    kind: &str,
    items: Vec<SchemaTypeParticle>,
    min: u64,
    max: u64,
    version: &'static str,
  ) -> SchemaTypeParticle {
    SchemaTypeParticle {
      kind: kind.to_string(),
      name: String::new(),
      occurs: vec![SchemaTypeParticleOccur {
        max,
        min,
        include_version: false,
        version: String::new(),
      }],
      items,
      initial_version: version.to_string(),
      require_filter: false,
      namespace: String::new(),
    }
  }

  fn one_sequence_schema(items: Vec<SchemaTypeParticle>) -> SchemaType {
    SchemaType {
      composite_kind: SchemaTypeCompositeKind::OneSequence,
      particle: SchemaTypeParticle {
        kind: "Sequence".to_string(),
        name: String::new(),
        occurs: vec![],
        items,
        initial_version: String::new(),
        require_filter: false,
        namespace: String::new(),
      },
      ..SchemaType::default()
    }
  }

  fn composite_schema(
    composite_kind: SchemaTypeCompositeKind,
    particle_kind: &str,
    items: Vec<SchemaTypeParticle>,
  ) -> SchemaType {
    SchemaType {
      composite_kind,
      particle: SchemaTypeParticle {
        kind: particle_kind.to_string(),
        name: String::new(),
        occurs: vec![],
        items,
        initial_version: String::new(),
        require_filter: false,
        namespace: String::new(),
      },
      ..SchemaType::default()
    }
  }

  #[test]
  fn flattens_group_sequence_wrappers() {
    let schema_type = one_sequence_schema(vec![node(
      "Group",
      vec![node(
        "Sequence",
        vec![
          leaf("a:CT_Test/a:first", 1, 1, ""),
          leaf("a:CT_Test/a:second", 0, 1, "Office2021"),
        ],
        1,
        1,
        "",
      )],
      1,
      1,
      "",
    )]);

    assert!(is_one_sequence_flatten(&schema_type));

    let flat = flatten_one_sequence_particles(&schema_type);
    assert_eq!(flat.len(), 2);
    let FlatParticleKind::Leaf(first) = flat[0].kind else {
      panic!("expected leaf");
    };
    assert_eq!(first.name, "a:CT_Test/a:first");
    assert!(!flat[0].optional);
    assert!(!flat[0].repeated);
    let FlatParticleKind::Leaf(second) = flat[1].kind else {
      panic!("expected leaf");
    };
    assert_eq!(second.name, "a:CT_Test/a:second");
    assert!(flat[1].optional);
    assert_eq!(flat[1].initial_version, "Office2021");
  }

  #[test]
  fn propagates_wrapper_occurs_and_version() {
    let schema_type = one_sequence_schema(vec![node(
      "Group",
      vec![leaf("a:CT_Test/a:item", 1, 1, "")],
      0,
      2,
      "Office2021",
    )]);

    let flat = flatten_one_sequence_particles(&schema_type);
    assert_eq!(flat.len(), 1);
    assert!(flat[0].optional);
    assert!(flat[0].repeated);
    assert_eq!(flat[0].initial_version, "Office2021");
  }

  #[test]
  fn flattens_leaf_choice_particles() {
    let schema_type = one_sequence_schema(vec![node(
      "Choice",
      vec![
        leaf("a:CT_Test/a:first", 1, 1, ""),
        leaf("a:CT_Test/a:second", 1, 1, ""),
      ],
      1,
      1,
      "",
    )]);

    assert!(is_one_sequence_flatten(&schema_type));

    let flat = flatten_one_sequence_particles(&schema_type);
    assert_eq!(flat.len(), 1);
    let FlatParticleKind::Choice(choice) = flat[0].kind else {
      panic!("expected choice");
    };
    assert_eq!(choice.items.len(), 2);
  }

  #[test]
  fn does_not_flatten_nested_choice_particles() {
    let schema_type = one_sequence_schema(vec![node(
      "Choice",
      vec![node(
        "Sequence",
        vec![leaf("a:CT_Test/a:first", 1, 1, "")],
        1,
        1,
        "",
      )],
      1,
      1,
      "",
    )]);

    assert!(!is_one_sequence_flatten(&schema_type));
  }

  #[test]
  fn structures_choice_with_sequence_variant() {
    let schema_type = one_sequence_schema(vec![node(
      "Choice",
      vec![
        leaf("c:CT_Test/c:delete", 1, 1, ""),
        node(
          "Group",
          vec![node(
            "Sequence",
            vec![
              leaf("c:CT_Test/c:layout", 0, 1, ""),
              leaf("c:CT_Test/c:tx", 0, 1, ""),
            ],
            1,
            1,
            "",
          )],
          1,
          1,
          "",
        ),
      ],
      1,
      1,
      "",
    )]);

    assert!(!is_one_sequence_flatten(&schema_type));
    assert!(is_one_sequence_structurable(&schema_type));

    let structured = structure_one_sequence_particles(&schema_type);
    assert_eq!(structured.len(), 1);
    let StructuredParticleKind::Choice(choice) = &structured[0].kind else {
      panic!("expected structured choice");
    };
    assert_eq!(choice.variants.len(), 2);
    assert!(matches!(
      choice.variants[0],
      StructuredChoiceVariant::Leaf(_)
    ));
    let StructuredChoiceVariant::Sequence(sequence_variant) = &choice.variants[1] else {
      panic!("expected structured sequence variant");
    };
    assert_eq!(sequence_variant.len(), 2);
    let StructuredParticleKind::Leaf(layout) = &sequence_variant[0].kind else {
      panic!("expected layout leaf");
    };
    assert_eq!(layout.name, "c:CT_Test/c:layout");
    assert!(sequence_variant[0].optional);
  }

  #[test]
  fn structures_choice_with_nested_choice_wrapper_variant() {
    let schema_type = one_sequence_schema(vec![node(
      "Choice",
      vec![node(
        "Group",
        vec![node(
          "Choice",
          vec![leaf("a:CT_Test/a:first", 1, 1, "")],
          1,
          1,
          "",
        )],
        1,
        1,
        "",
      )],
      1,
      1,
      "",
    )]);

    assert!(is_one_sequence_structurable(&schema_type));

    let structured = structure_one_sequence_particles(&schema_type);
    assert_eq!(structured.len(), 1);
    let StructuredParticleKind::Choice(choice) = &structured[0].kind else {
      panic!("expected structured choice");
    };
    assert_eq!(choice.variants.len(), 1);
    assert!(matches!(
      choice.variants[0],
      StructuredChoiceVariant::Leaf(_)
    ));
  }

  #[test]
  fn does_not_structure_choice_with_mixed_choice_and_leaf_sequence_variant() {
    let schema_type = one_sequence_schema(vec![node(
      "Choice",
      vec![node(
        "Group",
        vec![
          node(
            "Choice",
            vec![leaf("a:CT_Test/a:first", 1, 1, "")],
            1,
            1,
            "",
          ),
          leaf("a:CT_Test/a:second", 1, 1, ""),
        ],
        1,
        1,
        "",
      )],
      1,
      1,
      "",
    )]);

    assert!(!is_one_sequence_structurable(&schema_type));
  }

  #[test]
  fn one_choice_composite_kind_is_source_metadata_only() {
    let schema_type = composite_schema(
      SchemaTypeCompositeKind::OneChoice,
      "Choice",
      vec![leaf("a:CT_Test/a:first", 1, 1, "")],
    );

    assert!(!is_one_sequence_flatten(&schema_type));
    assert!(!is_one_sequence_structurable(&schema_type));
  }

  #[test]
  fn one_all_composite_kind_is_source_metadata_only() {
    let schema_type = composite_schema(
      SchemaTypeCompositeKind::OneAll,
      "All",
      vec![
        leaf("a:CT_Test/a:first", 0, 1, ""),
        leaf("a:CT_Test/a:second", 1, 1, ""),
      ],
    );

    assert!(!is_one_sequence_flatten(&schema_type));
    assert!(!is_one_sequence_structurable(&schema_type));
  }
}
