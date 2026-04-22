//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml";
pub const PATH_PREFIX: &str = "../customXml";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct CustomXmlPart {
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub part_content: String,
  pub custom_xml_properties_part:
    Option<std::boxed::Box<crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart>>,
}
