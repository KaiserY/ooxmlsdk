//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.customXmlProperties+xml";
pub const TARGET_NAME: &str = "itemProps";
pub const EXTENSION: &str = "";
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct CustomXmlPropertiesPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl CustomXmlPropertiesPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_office_document_2006_custom_xml::DataStoreItem,
    CustomXmlPropertiesPart,
    as_custom_xml_properties_part,
    as_custom_xml_properties_part_mut
  );
}
