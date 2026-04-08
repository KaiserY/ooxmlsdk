use crate::sdk_data::open_xml::{
  OpenXmlSchema, OpenXmlSchemaType, OpenXmlSchemaTypeAttribute,
  OpenXmlSchemaTypeAttributeValidator, OpenXmlSchemaTypeChild, OpenXmlSchemaTypeParticle,
};

const MC_TARGET_NAMESPACE: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";
const MC_PREFIX: &str = "mc";

pub fn gen_mc_schema() -> OpenXmlSchema {
  OpenXmlSchema {
    target_namespace: MC_TARGET_NAMESPACE.to_string(),
    types: vec![
      gen_alternate_content_type(),
      gen_choice_type(),
      gen_fallback_type(),
    ],
    enums: vec![],
    module_name: "schemas_openxmlformats_org_markup_compatibility_2006".to_string(),
  }
}

fn gen_alternate_content_type() -> OpenXmlSchemaType {
  OpenXmlSchemaType {
    name: format!("{MC_PREFIX}:CT_AlternateContent/{MC_PREFIX}:AlternateContent"),
    class_name: "AlternateContent".to_string(),
    summary: "Defines the AlternateContent Class.".to_string(),
    version: "Office2007".to_string(),
    part: String::new(),
    composite_type: String::new(),
    base_class: "OpenXmlCompositeElement".to_string(),
    is_leaf_text: false,
    is_leaf_element: false,
    is_derived: false,
    is_abstract: false,
    has_xmlns_fields: true,
    has_mc_ignorable_field: true,
    attributes: vec![
      optional_mc_attr("MustUnderstand"),
      optional_mc_attr("ProcessContent"),
    ],
    children: vec![
      OpenXmlSchemaTypeChild {
        name: format!("{MC_PREFIX}:CT_Choice/{MC_PREFIX}:Choice"),
        property_name: String::new(),
        property_comments: String::new(),
      },
      OpenXmlSchemaTypeChild {
        name: format!("{MC_PREFIX}:CT_Fallback/{MC_PREFIX}:Fallback"),
        property_name: String::new(),
        property_comments: String::new(),
      },
    ],
    particle: OpenXmlSchemaTypeParticle::default(),
    module_name: String::new(),
  }
}

fn gen_choice_type() -> OpenXmlSchemaType {
  OpenXmlSchemaType {
    name: format!("{MC_PREFIX}:CT_Choice/{MC_PREFIX}:Choice"),
    class_name: "Choice".to_string(),
    summary: "Defines the Choice Class.".to_string(),
    version: "Office2007".to_string(),
    part: String::new(),
    composite_type: String::new(),
    base_class: "OpenXmlCompositeElement".to_string(),
    is_leaf_text: false,
    is_leaf_element: false,
    is_derived: false,
    is_abstract: false,
    has_xmlns_fields: true,
    has_mc_ignorable_field: true,
    attributes: vec![
      required_attr("Requires"),
      optional_mc_attr("MustUnderstand"),
      optional_mc_attr("ProcessContent"),
    ],
    children: vec![],
    particle: any_particle(),
    module_name: String::new(),
  }
}

fn gen_fallback_type() -> OpenXmlSchemaType {
  OpenXmlSchemaType {
    name: format!("{MC_PREFIX}:CT_Fallback/{MC_PREFIX}:Fallback"),
    class_name: "Fallback".to_string(),
    summary: "Defines the Fallback Class.".to_string(),
    version: "Office2007".to_string(),
    part: String::new(),
    composite_type: String::new(),
    base_class: "OpenXmlCompositeElement".to_string(),
    is_leaf_text: false,
    is_leaf_element: false,
    is_derived: false,
    is_abstract: false,
    has_xmlns_fields: true,
    has_mc_ignorable_field: true,
    attributes: vec![
      optional_mc_attr("MustUnderstand"),
      optional_mc_attr("ProcessContent"),
    ],
    children: vec![],
    particle: any_particle(),
    module_name: String::new(),
  }
}

fn optional_mc_attr(name: &str) -> OpenXmlSchemaTypeAttribute {
  OpenXmlSchemaTypeAttribute {
    q_name: format!("{MC_PREFIX}:{name}"),
    property_name: String::new(),
    r#type: "StringValue".to_string(),
    property_comments: name.to_string(),
    version: "Office2007".to_string(),
    validators: vec![],
  }
}

fn required_attr(name: &str) -> OpenXmlSchemaTypeAttribute {
  OpenXmlSchemaTypeAttribute {
    q_name: name.to_string(),
    property_name: "requires".to_string(),
    r#type: "StringValue".to_string(),
    property_comments: name.to_string(),
    version: "Office2007".to_string(),
    validators: vec![OpenXmlSchemaTypeAttributeValidator {
      name: "RequiredValidator".to_string(),
      is_list: false,
      r#type: String::new(),
      union_id: 0,
      is_initial_version: false,
      arguments: vec![],
    }],
  }
}

fn any_particle() -> OpenXmlSchemaTypeParticle {
  OpenXmlSchemaTypeParticle {
    kind: "Sequence".to_string(),
    name: String::new(),
    occurs: vec![],
    items: vec![OpenXmlSchemaTypeParticle {
      kind: "Any".to_string(),
      name: String::new(),
      occurs: vec![],
      items: vec![],
      initial_version: "Office2007".to_string(),
      require_filter: false,
      namespace: String::new(),
    }],
    initial_version: "Office2007".to_string(),
    require_filter: false,
    namespace: String::new(),
  }
}
