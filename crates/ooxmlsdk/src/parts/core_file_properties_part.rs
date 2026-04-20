//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties";
pub const PATH_PREFIX: &str = "docProps";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct CoreFilePropertiesPart {
  #[sdk(part_rid)]
  pub r_id: String,
  #[sdk(part_inner_path)]
  pub inner_path: String,
  #[sdk(part_root)]
  pub root_element: crate::schemas::opc_core_properties::CoreProperties,
}
