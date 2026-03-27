use crate::sdk_data::sdk_data_model::{Schema, SchemaType};

const DRAWINGML_MAIN_NAMESPACE: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
const DRAWINGML_PICTURE_NAMESPACE: &str =
  "http://schemas.openxmlformats.org/drawingml/2006/picture";

pub fn is_composite_type(schema_type: &SchemaType) -> bool {
  schema_type.base_class == "OpenXmlCompositeElement"
    || schema_type.base_class == "CustomXmlElement"
    || schema_type.base_class == "OpenXmlPartRootElement"
    || schema_type.base_class == "SdtElement"
}

pub fn is_drawingml_namespace(schema: &Schema) -> bool {
  schema.target_namespace == DRAWINGML_MAIN_NAMESPACE
    || schema.target_namespace == DRAWINGML_PICTURE_NAMESPACE
}

pub fn needs_xml_header(schema_type: &SchemaType) -> bool {
  !schema_type.part.is_empty() || schema_type.base_class == "OpenXmlPartRootElement"
}

pub fn supports_xmlns_fields(schema_type: &SchemaType, schema: &Schema) -> bool {
  needs_xml_header(schema_type) || (is_composite_type(schema_type) && is_drawingml_namespace(schema))
}

pub fn is_one_sequence_flatten(schema_type: &SchemaType) -> bool {
  if schema_type.composite_type == "OneSequence" || schema_type.particle.kind == "Sequence" {
    for particle in &schema_type.particle.items {
      if !particle.kind.is_empty() || !particle.items.is_empty() {
        return false;
      }
    }

    true
  } else {
    false
  }
}
