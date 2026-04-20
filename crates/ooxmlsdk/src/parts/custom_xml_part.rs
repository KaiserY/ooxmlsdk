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
  #[sdk(part_rid)]
  pub r_id: String,
  #[sdk(part_relationships)]
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  #[sdk(part_rels_path)]
  pub rels_path: String,
  #[sdk(part_inner_path)]
  pub inner_path: String,
  #[sdk(part_content(kind = "text"))]
  pub part_content: String,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps",
    kind = "optional"
  ))]
  pub custom_xml_properties_part:
    Option<std::boxed::Box<crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart>>,
}
