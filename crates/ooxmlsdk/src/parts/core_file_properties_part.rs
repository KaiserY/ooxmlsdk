//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties";
pub const PATH_PREFIX: &str = "docProps";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct CoreFilePropertiesPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_core_file_properties_part"))]
  pub(crate) root_element:
    crate::sdk::PartRoot<crate::schemas::opc_core_properties::CoreProperties>,
}
