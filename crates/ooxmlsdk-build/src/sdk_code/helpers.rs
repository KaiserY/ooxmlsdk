use crate::sdk_code::versioning::effective_version;
use crate::sdk_data::sdk_data_model::{
  Schema, SchemaType, SchemaTypeApiKind, SchemaTypeCompositeKind, SchemaTypeKind,
  SchemaTypeParticle,
};

const DRAWINGML_MAIN_NAMESPACE: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
const DRAWINGML_PICTURE_NAMESPACE: &str =
  "http://schemas.openxmlformats.org/drawingml/2006/picture";

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

pub fn is_drawingml_namespace(schema: &Schema) -> bool {
  schema.target_namespace == DRAWINGML_MAIN_NAMESPACE
    || schema.target_namespace == DRAWINGML_PICTURE_NAMESPACE
}

pub fn needs_xml_header(schema_type: &SchemaType) -> bool {
  !schema_type.part.is_empty() || schema_type.base_class == "OpenXmlPartRootElement"
}

pub fn supports_xmlns_fields(schema_type: &SchemaType, schema: &Schema) -> bool {
  needs_xml_header(schema_type)
    || (is_composite_type(schema_type) && is_drawingml_namespace(schema))
}

pub fn is_one_sequence_flatten(schema_type: &SchemaType) -> bool {
  if schema_type.composite_kind == SchemaTypeCompositeKind::OneSequence
    || schema_type.particle.kind == "Sequence"
  {
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

#[derive(Clone, Copy, Debug)]
pub enum FlatParticleKind<'a> {
  Leaf(&'a SchemaTypeParticle),
  Choice(&'a SchemaTypeParticle),
}

#[derive(Clone, Copy, Debug)]
pub struct FlatParticle<'a> {
  pub kind: FlatParticleKind<'a>,
  pub optional: bool,
  pub repeated: bool,
  pub initial_version: &'a str,
}

pub fn flatten_one_sequence_particles(schema_type: &SchemaType) -> Vec<FlatParticle<'_>> {
  let mut flat_particles = Vec::new();

  for particle in &schema_type.particle.items {
    flatten_one_sequence_particle(particle, false, false, "", &mut flat_particles);
  }

  flat_particles
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
}
