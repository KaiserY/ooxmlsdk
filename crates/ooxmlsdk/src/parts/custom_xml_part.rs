//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml";
pub const PATH_PREFIX: &str = "../customXml";
pub const CONTENT_TYPE: &str = "";
pub const TARGET_NAME: &str = "item";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct CustomXmlPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl CustomXmlPart {
  crate::sdk_part_child_methods! {
      optional custom_xml_properties_part => crate
      ::parts::custom_xml_properties_part::CustomXmlPropertiesPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps";
  }
}
