//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties";
pub const PATH_PREFIX: &str = "docProps";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.extended-properties+xml";
pub const TARGET_NAME: &str = "app";
pub const EXTENSION: &str = "";
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct ExtendedFilePropertiesPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl ExtendedFilePropertiesPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_office_document_2006_extended_properties::Properties,
    ExtendedFilePropertiesPart,
    as_extended_file_properties_part,
    as_extended_file_properties_part_mut
  );
}
