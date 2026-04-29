use crate::sdk_code::versioning::effective_version;
use crate::sdk_data::sdk_data_model::{
  SchemaType, SchemaTypeApiKind, SchemaTypeChild, SchemaTypeChildKind, SchemaTypeCompositeKind,
  SchemaTypeKind, SchemaTypeXmlHeader,
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
  schema_type.xml_header != SchemaTypeXmlHeader::None
}

pub fn has_xmlns_fields(schema_type: &SchemaType) -> bool {
  schema_type.has_xmlns_fields
}

pub fn has_mce_fields(schema_type: &SchemaType) -> bool {
  schema_type.has_mc_ignorable_field
    || schema_type.has_mc_must_understand_field
    || schema_type.has_mc_process_content_field
    || schema_type.has_mc_preserve_attributes_field
    || schema_type.has_mc_preserve_elements_field
}

fn supports_extended_sequence_strategy(schema_type: &SchemaType) -> bool {
  matches!(
    schema_type.composite_kind,
    SchemaTypeCompositeKind::OneSequence | SchemaTypeCompositeKind::SdkSequence
  ) || has_unambiguous_extended_sequence_topology(schema_type)
}

fn has_unambiguous_extended_sequence_topology(schema_type: &SchemaType) -> bool {
  if schema_type.children.is_empty() {
    return false;
  }

  schema_type.children.len() == 1
    && schema_type.children[0].kind == SchemaTypeChildKind::Sequence
    && !contains_choice_descendant(&schema_type.children[0])
}

fn contains_choice_descendant(child: &SchemaTypeChild) -> bool {
  child
    .children
    .iter()
    .any(|nested| nested.kind == SchemaTypeChildKind::Choice || contains_choice_descendant(nested))
}

pub fn is_one_sequence_flatten(schema_type: &SchemaType) -> bool {
  if supports_extended_sequence_strategy(schema_type) {
    if schema_type.children.is_empty() {
      return false;
    }
    schema_type
      .children
      .iter()
      .all(can_flatten_one_sequence_child)
  } else {
    false
  }
}

fn can_flatten_one_sequence_child(child: &SchemaTypeChild) -> bool {
  match child.kind {
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any => {
      child.children.is_empty()
    }
    SchemaTypeChildKind::Choice => {
      !child.children.is_empty()
        && child.children.iter().all(|item| {
          matches!(
            item.kind,
            SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any
          ) && item.children.is_empty()
        })
    }
    SchemaTypeChildKind::Sequence => child.children.iter().all(can_flatten_one_sequence_child),
  }
}

#[derive(Clone, Debug)]
pub enum FlatParticleKind<'a> {
  Leaf(&'a SchemaTypeChild),
  Choice(&'a SchemaTypeChild),
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
  Leaf(&'a SchemaTypeChild),
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
  pub property_name: &'a str,
  pub variants: Vec<StructuredChoiceVariant<'a>>,
}

#[derive(Clone, Debug)]
pub enum StructuredChoiceVariant<'a> {
  Leaf(&'a SchemaTypeChild),
  Sequence(Vec<StructuredParticle<'a>>),
}

pub fn flatten_one_sequence_particles(schema_type: &SchemaType) -> Vec<FlatParticle<'_>> {
  let mut flat_particles = Vec::new();

  for child in &schema_type.children {
    flatten_one_sequence_child(child, false, false, "", &mut flat_particles);
  }

  flat_particles
}

pub fn is_one_sequence_structurable(schema_type: &SchemaType) -> bool {
  if supports_extended_sequence_strategy(schema_type) {
    if schema_type.children.is_empty() {
      return false;
    }
    schema_type
      .children
      .iter()
      .all(can_structure_one_sequence_child)
  } else {
    false
  }
}

pub fn structure_one_sequence_particles(schema_type: &SchemaType) -> Vec<StructuredParticle<'_>> {
  let mut particles = Vec::new();

  for child in &schema_type.children {
    structure_one_sequence_child(child, false, false, "", &mut particles);
  }

  particles
}

fn flatten_one_sequence_child<'a>(
  child: &'a SchemaTypeChild,
  parent_optional: bool,
  parent_repeated: bool,
  parent_initial_version: &'a str,
  out: &mut Vec<FlatParticle<'a>>,
) {
  let optional = parent_optional || child.optional;
  let repeated = parent_repeated || child.repeated;
  let initial_version = effective_version(parent_initial_version, child.initial_version.as_str());

  match child.kind {
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any => {
      if !child.name.is_empty() || matches!(child.kind, SchemaTypeChildKind::Any) {
        out.push(FlatParticle {
          kind: FlatParticleKind::Leaf(child),
          optional,
          repeated,
          initial_version,
        });
      }
    }
    SchemaTypeChildKind::Choice => {
      if !child.children.is_empty() {
        out.push(FlatParticle {
          kind: FlatParticleKind::Choice(child),
          optional,
          repeated,
          initial_version,
        });
      }
    }
    SchemaTypeChildKind::Sequence => {
      for grand_child in &child.children {
        flatten_one_sequence_child(grand_child, optional, repeated, initial_version, out);
      }
    }
  }
}

fn can_structure_one_sequence_child(child: &SchemaTypeChild) -> bool {
  match child.kind {
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any => {
      child.children.is_empty()
    }
    SchemaTypeChildKind::Choice => {
      !child.children.is_empty()
        && child
          .children
          .iter()
          .all(can_structure_one_sequence_choice_variant)
    }
    SchemaTypeChildKind::Sequence => child.children.iter().all(can_structure_one_sequence_child),
  }
}

fn can_structure_one_sequence_choice_variant(child: &SchemaTypeChild) -> bool {
  match child.kind {
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild => child.children.is_empty(),
    SchemaTypeChildKind::Any => false,
    SchemaTypeChildKind::Choice => {
      !child.children.is_empty()
        && child
          .children
          .iter()
          .all(can_structure_one_sequence_choice_variant)
    }
    SchemaTypeChildKind::Sequence => {
      can_structure_sequence_variant_child(child)
        || (child.children.len() == 1
          && can_structure_one_sequence_choice_variant(&child.children[0]))
    }
  }
}

fn can_structure_sequence_variant_child(child: &SchemaTypeChild) -> bool {
  match child.kind {
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild => child.children.is_empty(),
    SchemaTypeChildKind::Any => false,
    SchemaTypeChildKind::Sequence => child
      .children
      .iter()
      .all(can_structure_sequence_variant_child),
    SchemaTypeChildKind::Choice => false,
  }
}

pub fn is_any_only_composite_type(schema_type: &SchemaType) -> bool {
  is_any_only_children(&schema_type.children)
}

fn is_any_only_children(children: &[SchemaTypeChild]) -> bool {
  if children.len() != 1 {
    return false;
  }

  match children[0].kind {
    SchemaTypeChildKind::Any => true,
    SchemaTypeChildKind::Sequence => is_any_only_children(&children[0].children),
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Choice => {
      false
    }
  }
}

pub fn has_any_child(schema_type: &SchemaType) -> bool {
  schema_type
    .children
    .iter()
    .any(|child| child.kind == SchemaTypeChildKind::Any)
}

fn structure_one_sequence_child<'a>(
  child: &'a SchemaTypeChild,
  parent_optional: bool,
  parent_repeated: bool,
  parent_initial_version: &'a str,
  out: &mut Vec<StructuredParticle<'a>>,
) {
  let optional = parent_optional || child.optional;
  let repeated = parent_repeated || child.repeated;
  let initial_version = effective_version(parent_initial_version, child.initial_version.as_str());

  match child.kind {
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any => {
      if !child.name.is_empty() || matches!(child.kind, SchemaTypeChildKind::Any) {
        out.push(StructuredParticle {
          kind: StructuredParticleKind::Leaf(child),
          optional,
          repeated,
          initial_version,
        });
      }
    }
    SchemaTypeChildKind::Choice => {
      let mut variants = Vec::new();

      for item in &child.children {
        variants.extend(structure_one_sequence_choice_variant(item, initial_version));
      }
      dedupe_structured_choice_variants(&mut variants);

      if !variants.is_empty() {
        out.push(StructuredParticle {
          kind: StructuredParticleKind::Choice(StructuredChoice {
            property_name: child.property_name.as_str(),
            variants,
          }),
          optional,
          repeated,
          initial_version,
        });
      }
    }
    SchemaTypeChildKind::Sequence => {
      for grand_child in &child.children {
        structure_one_sequence_child(grand_child, optional, repeated, initial_version, out);
      }
    }
  }
}

fn dedupe_structured_choice_variants(variants: &mut Vec<StructuredChoiceVariant<'_>>) {
  let mut seen = std::collections::HashSet::new();

  variants.retain(|variant| seen.insert(structured_choice_variant_key(variant)));
}

fn structured_choice_variant_key(variant: &StructuredChoiceVariant<'_>) -> String {
  match variant {
    StructuredChoiceVariant::Leaf(child) => format!("leaf:{}", child.name),
    StructuredChoiceVariant::Sequence(particles) => format!(
      "sequence:{}",
      particles
        .iter()
        .map(structured_sequence_particle_key)
        .collect::<Vec<_>>()
        .join("|")
    ),
  }
}

fn structured_sequence_particle_key(particle: &StructuredParticle<'_>) -> String {
  let kind = match &particle.kind {
    StructuredParticleKind::Leaf(child) => child.name.as_str(),
    StructuredParticleKind::Choice(_) => "choice",
  };

  format!(
    "{}:{}:{}:{}",
    kind, particle.optional, particle.repeated, particle.initial_version
  )
}

fn structure_one_sequence_choice_variant<'a>(
  child: &'a SchemaTypeChild,
  parent_initial_version: &'a str,
) -> Vec<StructuredChoiceVariant<'a>> {
  let initial_version = effective_version(parent_initial_version, child.initial_version.as_str());

  match child.kind {
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any => {
      (!child.name.is_empty() || matches!(child.kind, SchemaTypeChildKind::Any))
        .then_some(StructuredChoiceVariant::Leaf(child))
        .into_iter()
        .collect()
    }
    SchemaTypeChildKind::Choice => child
      .children
      .iter()
      .flat_map(|child| structure_one_sequence_choice_variant(child, initial_version))
      .collect(),
    SchemaTypeChildKind::Sequence => {
      if can_structure_sequence_variant_child(child) {
        let mut particles = Vec::new();

        for grand_child in &child.children {
          structure_sequence_variant_child(
            grand_child,
            false,
            false,
            initial_version,
            &mut particles,
          );
        }

        (!particles.is_empty())
          .then_some(StructuredChoiceVariant::Sequence(particles))
          .into_iter()
          .collect()
      } else if child.children.len() == 1 {
        structure_one_sequence_choice_variant(&child.children[0], initial_version)
      } else {
        Vec::new()
      }
    }
  }
}

fn structure_sequence_variant_child<'a>(
  child: &'a SchemaTypeChild,
  parent_optional: bool,
  parent_repeated: bool,
  parent_initial_version: &'a str,
  out: &mut Vec<StructuredParticle<'a>>,
) {
  let optional = parent_optional || child.optional;
  let repeated = parent_repeated;
  let initial_version = effective_version(parent_initial_version, child.initial_version.as_str());

  match child.kind {
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any => {
      if !child.name.is_empty() || matches!(child.kind, SchemaTypeChildKind::Any) {
        out.push(StructuredParticle {
          kind: StructuredParticleKind::Leaf(child),
          optional,
          repeated,
          initial_version,
        });
      }
    }
    SchemaTypeChildKind::Sequence => {
      for grand_child in &child.children {
        structure_sequence_variant_child(grand_child, optional, repeated, initial_version, out);
      }
    }
    SchemaTypeChildKind::Choice => {}
  }
}

pub fn classify_simple_type(simple_type: &str) -> Option<SimpleValueKind> {
  match simple_type {
    "Base64BinaryValue" | "DateTimeValue" | "DecimalValue" | "HexBinaryValue" | "StringValue" => {
      Some(SimpleValueKind::StringLike)
    }
    "BooleanValue" | "OnOffValue" | "TrueFalseBlankValue" | "TrueFalseValue" => {
      Some(SimpleValueKind::BoolLike)
    }
    "ByteValue" | "SByteValue" | "Int16Value" | "Int32Value" | "Int64Value" | "IntegerValue"
    | "UInt16Value" | "UInt32Value" | "UInt64Value" | "DoubleValue" | "SingleValue" => {
      Some(SimpleValueKind::NumericLike)
    }
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
