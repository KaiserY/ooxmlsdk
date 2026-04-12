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
    || schema_type
      .children
      .iter()
      .any(|child| child.kind == SchemaTypeChildKind::XmlHeader)
}

pub fn has_xmlns_fields(schema_type: &SchemaType) -> bool {
  schema_type.has_xmlns_fields
    || schema_type
      .children
      .iter()
      .any(|child| child.kind == SchemaTypeChildKind::Xmlns)
}

pub fn has_mce_fields(schema_type: &SchemaType) -> bool {
  schema_type.has_mc_ignorable_field
    || schema_type
      .children
      .iter()
      .any(|child| child.kind == SchemaTypeChildKind::Mce)
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
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild => child.children.is_empty(),
    SchemaTypeChildKind::Any => false,
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
    SchemaTypeChildKind::Xmlns | SchemaTypeChildKind::XmlHeader | SchemaTypeChildKind::Mce => false,
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
  pub variants: Vec<StructuredChoiceVariant<'a>>,
}

#[derive(Clone, Debug)]
pub enum StructuredChoiceVariant<'a> {
  Leaf(&'a SchemaTypeChild),
  Sequence(Vec<StructuredParticle<'a>>),
}

pub fn flatten_one_sequence_particles(schema_type: &SchemaType) -> Vec<FlatParticle<'_>> {
  let mut flat_particles = Vec::new();
  let parent_repeated = is_collection_sequence_root(schema_type);

  for child in &schema_type.children {
    flatten_one_sequence_child(child, false, parent_repeated, "", &mut flat_particles);
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
  let parent_repeated = is_collection_sequence_root(schema_type);

  for child in &schema_type.children {
    structure_one_sequence_child(child, false, parent_repeated, "", &mut particles);
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
    SchemaTypeChildKind::Xmlns | SchemaTypeChildKind::XmlHeader | SchemaTypeChildKind::Mce => {}
  }
}

fn can_structure_one_sequence_child(child: &SchemaTypeChild) -> bool {
  match child.kind {
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild => child.children.is_empty(),
    SchemaTypeChildKind::Any => false,
    SchemaTypeChildKind::Choice => {
      let mut leaf_name_set = std::collections::HashSet::new();

      !child.children.is_empty()
        && child
          .children
          .iter()
          .all(can_structure_one_sequence_choice_variant)
        && child
          .children
          .iter()
          .all(|item| collect_structured_choice_leaf_names(item, &mut leaf_name_set))
    }
    SchemaTypeChildKind::Sequence => child.children.iter().all(can_structure_one_sequence_child),
    SchemaTypeChildKind::Xmlns | SchemaTypeChildKind::XmlHeader | SchemaTypeChildKind::Mce => false,
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
    SchemaTypeChildKind::Xmlns | SchemaTypeChildKind::XmlHeader | SchemaTypeChildKind::Mce => false,
  }
}

fn can_structure_sequence_variant_child(child: &SchemaTypeChild) -> bool {
  if child.repeated {
    return false;
  }

  match child.kind {
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild => child.children.is_empty(),
    SchemaTypeChildKind::Any => false,
    SchemaTypeChildKind::Sequence => child
      .children
      .iter()
      .all(can_structure_sequence_variant_child),
    SchemaTypeChildKind::Choice
    | SchemaTypeChildKind::Xmlns
    | SchemaTypeChildKind::XmlHeader
    | SchemaTypeChildKind::Mce => false,
  }
}

pub fn is_any_only_composite_type(schema_type: &SchemaType) -> bool {
  schema_type.composite_kind == SchemaTypeCompositeKind::SdkSequence
    && schema_type.children.len() == 1
    && schema_type.children[0].kind == SchemaTypeChildKind::Any
}

pub fn has_any_child(schema_type: &SchemaType) -> bool {
  schema_type
    .children
    .iter()
    .any(|child| child.kind == SchemaTypeChildKind::Any)
}

fn collect_structured_choice_leaf_names<'a>(
  child: &'a SchemaTypeChild,
  out: &mut std::collections::HashSet<&'a str>,
) -> bool {
  match child.kind {
    SchemaTypeChildKind::Child | SchemaTypeChildKind::TextChild | SchemaTypeChildKind::Any => {
      !child.name.is_empty() && out.insert(child.name.as_str())
    }
    SchemaTypeChildKind::Choice => {
      !child.children.is_empty()
        && child
          .children
          .iter()
          .all(|item| collect_structured_choice_leaf_names(item, out))
    }
    SchemaTypeChildKind::Sequence => {
      if can_structure_sequence_variant_child(child) {
        true
      } else if child.children.len() == 1 {
        collect_structured_choice_leaf_names(&child.children[0], out)
      } else {
        false
      }
    }
    SchemaTypeChildKind::Xmlns | SchemaTypeChildKind::XmlHeader | SchemaTypeChildKind::Mce => false,
  }
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

      if !variants.is_empty() {
        out.push(StructuredParticle {
          kind: StructuredParticleKind::Choice(StructuredChoice { variants }),
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
    SchemaTypeChildKind::Xmlns | SchemaTypeChildKind::XmlHeader | SchemaTypeChildKind::Mce => {}
  }
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
    SchemaTypeChildKind::Xmlns | SchemaTypeChildKind::XmlHeader | SchemaTypeChildKind::Mce => {
      Vec::new()
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
    SchemaTypeChildKind::Sequence => {
      for grand_child in &child.children {
        structure_sequence_variant_child(grand_child, optional, repeated, initial_version, out);
      }
    }
    SchemaTypeChildKind::Choice
    | SchemaTypeChildKind::Xmlns
    | SchemaTypeChildKind::XmlHeader
    | SchemaTypeChildKind::Mce => {}
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
    SchemaType, SchemaTypeChild, SchemaTypeChildKind, SchemaTypeCompositeKind,
  };

  fn child(name: &str, optional: bool, repeated: bool, version: &'static str) -> SchemaTypeChild {
    SchemaTypeChild {
      name: name.to_string(),
      property_name: String::new(),
      property_comments: String::new(),
      kind: SchemaTypeChildKind::Child,
      optional,
      repeated,
      initial_version: version.to_string(),
      children: vec![],
    }
  }

  fn choice(
    items: Vec<SchemaTypeChild>,
    optional: bool,
    repeated: bool,
    version: &'static str,
  ) -> SchemaTypeChild {
    SchemaTypeChild {
      name: String::new(),
      property_name: String::new(),
      property_comments: String::new(),
      kind: SchemaTypeChildKind::Choice,
      optional,
      repeated,
      initial_version: version.to_string(),
      children: items,
    }
  }

  fn sequence(
    items: Vec<SchemaTypeChild>,
    optional: bool,
    repeated: bool,
    version: &'static str,
  ) -> SchemaTypeChild {
    SchemaTypeChild {
      name: String::new(),
      property_name: String::new(),
      property_comments: String::new(),
      kind: SchemaTypeChildKind::Sequence,
      optional,
      repeated,
      initial_version: version.to_string(),
      children: items,
    }
  }

  fn one_sequence_schema(items: Vec<SchemaTypeChild>) -> SchemaType {
    SchemaType {
      composite_kind: SchemaTypeCompositeKind::OneSequence,
      children: items,
      ..SchemaType::default()
    }
  }

  fn composite_schema(
    composite_kind: SchemaTypeCompositeKind,
    items: Vec<SchemaTypeChild>,
  ) -> SchemaType {
    SchemaType {
      composite_kind,
      children: items,
      ..SchemaType::default()
    }
  }

  #[test]
  fn flattens_group_sequence_wrappers() {
    let schema_type = one_sequence_schema(vec![sequence(
      vec![
        child("a:CT_Test/a:first", false, false, ""),
        child("a:CT_Test/a:second", true, false, "Office2021"),
      ],
      false,
      false,
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
    let schema_type = one_sequence_schema(vec![sequence(
      vec![child("a:CT_Test/a:item", false, false, "")],
      true,
      true,
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
    let schema_type = one_sequence_schema(vec![choice(
      vec![
        child("a:CT_Test/a:first", false, false, ""),
        child("a:CT_Test/a:second", false, false, ""),
      ],
      true,
      true,
      "",
    )]);

    assert!(is_one_sequence_flatten(&schema_type));

    let flat = flatten_one_sequence_particles(&schema_type);
    assert_eq!(flat.len(), 1);
    let FlatParticleKind::Choice(choice) = flat[0].kind else {
      panic!("expected choice");
    };
    assert_eq!(choice.children.len(), 2);
  }

  #[test]
  fn does_not_flatten_nested_choice_particles() {
    let schema_type = one_sequence_schema(vec![choice(
      vec![sequence(
        vec![child("a:CT_Test/a:first", false, false, "")],
        false,
        false,
        "",
      )],
      true,
      false,
      "",
    )]);

    assert!(!is_one_sequence_flatten(&schema_type));
  }

  #[test]
  fn structures_choice_with_sequence_variant() {
    let schema_type = one_sequence_schema(vec![choice(
      vec![
        child("c:CT_Test/c:delete", false, false, ""),
        sequence(
          vec![
            child("c:CT_Test/c:layout", true, false, ""),
            child("c:CT_Test/c:tx", true, false, ""),
          ],
          false,
          false,
          "",
        ),
      ],
      true,
      false,
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
    let schema_type = one_sequence_schema(vec![choice(
      vec![sequence(
        vec![choice(
          vec![child("a:CT_Test/a:first", false, false, "")],
          true,
          false,
          "",
        )],
        false,
        false,
        "",
      )],
      true,
      false,
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
    let schema_type = one_sequence_schema(vec![choice(
      vec![sequence(
        vec![
          choice(
            vec![child("a:CT_Test/a:first", false, false, "")],
            true,
            false,
            "",
          ),
          child("a:CT_Test/a:second", false, false, ""),
        ],
        false,
        false,
        "",
      )],
      true,
      false,
      "",
    )]);

    assert!(!is_one_sequence_structurable(&schema_type));
  }

  #[test]
  fn one_choice_composite_kind_is_source_metadata_only() {
    let schema_type = composite_schema(
      SchemaTypeCompositeKind::OneChoice,
      vec![choice(
        vec![child("a:CT_Test/a:first", false, false, "")],
        true,
        false,
        "",
      )],
    );

    assert!(!is_one_sequence_flatten(&schema_type));
    assert!(!is_one_sequence_structurable(&schema_type));
  }

  #[test]
  fn one_all_composite_kind_is_source_metadata_only() {
    let schema_type = composite_schema(
      SchemaTypeCompositeKind::OneAll,
      vec![
        child("a:CT_Test/a:first", true, false, ""),
        child("a:CT_Test/a:second", false, false, ""),
      ],
    );

    assert!(!is_one_sequence_flatten(&schema_type));
    assert!(!is_one_sequence_structurable(&schema_type));
  }
}
