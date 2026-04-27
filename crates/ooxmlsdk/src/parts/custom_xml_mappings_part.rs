//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str = "application/xml";
pub const TARGET_NAME: &str = "xmlMaps";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct CustomXmlMappingsPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl CustomXmlMappingsPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::MapInfo,
    CustomXmlMappingsPart,
    as_custom_xml_mappings_part,
    as_custom_xml_mappings_part_mut
  );
}
