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
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct CoreFilePropertiesPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_core_file_properties_part"))]
  pub(crate) root_element:
    crate::sdk::PartRoot<crate::schemas::opc_core_properties::CoreProperties>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
  pub(crate) modeled_relationships: Vec<crate::common::RelationshipInfo>,
}
