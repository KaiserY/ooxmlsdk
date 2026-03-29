use crate::sdk_data::sdk_data_model::{
  Schema, SchemaType, SchemaTypeApiKind, SchemaTypeCompositeKind, SchemaTypeKind,
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
  schema_type.composite_kind == SchemaTypeCompositeKind::OneSequence
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
