use std::fs;
use std::path::Path;

use crate::Result;
use crate::sdk_data::open_xml::{
  OpenXmlSchema, OpenXmlSchemaType, OpenXmlSchemaTypeChild, OpenXmlSchemaTypeParticle,
};
use crate::sdk_data::xsd::{ParsedXsd, parse_xsd};

const MC_TARGET_NAMESPACE: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";
const MC_PREFIX: &str = "mc";

pub fn gen_mc_schema_from_xsd(schemas_dir: &Path) -> Result<Option<OpenXmlSchema>> {
  if !schemas_dir.exists() {
    return Ok(None);
  }

  let source = fs::read_to_string(schemas_dir.join("mc.xsd"))?;
  let xsd = parse_xsd(&source).map_err(
    |err| -> Box<dyn std::error::Error + Send + Sync + 'static> { format!("mc.xsd: {err}").into() },
  )?;
  Ok(Some(gen_mc_schema(&xsd)))
}

pub(crate) fn gen_mc_schema(xsd: &ParsedXsd) -> OpenXmlSchema {
  let alternate_content = xsd
    .root_elements
    .get("AlternateContent")
    .or_else(|| {
      xsd.root_elements.iter().find_map(|(name, complex_type)| {
        matches_local_name(name, "AlternateContent").then_some(complex_type)
      })
    })
    .expect("missing AlternateContent");
  let choice = alternate_content
    .children
    .iter()
    .find(|child| matches_local_name(&child.q_name, "Choice"))
    .expect("missing Choice");
  let fallback = alternate_content
    .children
    .iter()
    .find(|child| matches_local_name(&child.q_name, "Fallback"))
    .expect("missing Fallback");

  let choice_type = choice
    .complex_type
    .as_ref()
    .expect("missing Choice complexType");
  let fallback_type = fallback
    .complex_type
    .as_ref()
    .expect("missing Fallback complexType");

  OpenXmlSchema {
    target_namespace: MC_TARGET_NAMESPACE.to_string(),
    types: vec![
      gen_alternate_content_type(&alternate_content.attributes),
      gen_choice_type(&choice_type.attributes),
      gen_fallback_type(&fallback_type.attributes),
    ],
    enums: vec![],
    module_name: "schemas_openxmlformats_org_markup_compatibility_2006".to_string(),
  }
}

fn gen_alternate_content_type(
  _attributes: &[crate::sdk_data::xsd::ParsedAttribute],
) -> OpenXmlSchemaType {
  OpenXmlSchemaType {
    name: format!("{MC_PREFIX}:CT_AlternateContent/{MC_PREFIX}:AlternateContent"),
    class_name: "AlternateContent".to_string(),
    summary: "Defines the AlternateContent Class.".to_string(),
    version: Some("Office2007".to_string()),
    part: String::new(),
    composite_type: String::new(),
    base_class: "OpenXmlCompositeElement".to_string(),
    is_leaf_text: false,
    is_leaf_element: false,
    is_derived: false,
    is_abstract: false,
    has_xmlns_fields: true,
    additional_elements: vec![],
    attributes: vec![],
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

fn gen_choice_type(_attributes: &[crate::sdk_data::xsd::ParsedAttribute]) -> OpenXmlSchemaType {
  OpenXmlSchemaType {
    name: format!("{MC_PREFIX}:CT_Choice/{MC_PREFIX}:Choice"),
    class_name: "Choice".to_string(),
    summary: "Defines the Choice Class.".to_string(),
    version: Some("Office2007".to_string()),
    part: String::new(),
    composite_type: String::new(),
    base_class: "OpenXmlCompositeElement".to_string(),
    is_leaf_text: false,
    is_leaf_element: false,
    is_derived: false,
    is_abstract: false,
    has_xmlns_fields: true,
    additional_elements: vec![],
    attributes: vec![],
    children: vec![],
    particle: any_particle(),
    module_name: String::new(),
  }
}

fn gen_fallback_type(_attributes: &[crate::sdk_data::xsd::ParsedAttribute]) -> OpenXmlSchemaType {
  OpenXmlSchemaType {
    name: format!("{MC_PREFIX}:CT_Fallback/{MC_PREFIX}:Fallback"),
    class_name: "Fallback".to_string(),
    summary: "Defines the Fallback Class.".to_string(),
    version: Some("Office2007".to_string()),
    part: String::new(),
    composite_type: String::new(),
    base_class: "OpenXmlCompositeElement".to_string(),
    is_leaf_text: false,
    is_leaf_element: false,
    is_derived: false,
    is_abstract: false,
    has_xmlns_fields: true,
    additional_elements: vec![],
    attributes: vec![],
    children: vec![],
    particle: any_particle(),
    module_name: String::new(),
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
    initial_version: String::new(),
    require_filter: false,
    namespace: String::new(),
  }
}

fn matches_local_name(value: &str, expected: &str) -> bool {
  value
    .rsplit_once(':')
    .map(|(_, local)| local)
    .unwrap_or(value)
    == expected
}
