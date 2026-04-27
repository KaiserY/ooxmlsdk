//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties";
pub const PATH_PREFIX: &str = "docProps";
pub const CONTENT_TYPE: &str = "application/vnd.openxmlformats-package.core-properties+xml";
pub const TARGET_NAME: &str = "core";
pub const EXTENSION: &str = "";
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct CoreFilePropertiesPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl CoreFilePropertiesPart {
  crate::sdk_part_root_methods!(
    crate::schemas::opc_core_properties::CoreProperties,
    CoreFilePropertiesPart,
    as_core_file_properties_part,
    as_core_file_properties_part_mut
  );
}
