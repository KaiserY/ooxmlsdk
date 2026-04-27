//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2007/relationships/slicerCache";
pub const PATH_PREFIX: &str = "slicerCaches";
pub const CONTENT_TYPE: &str = "application/vnd.ms-excel.slicerCache+xml";
pub const TARGET_NAME: &str = "slicerCache";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct SlicerCachePart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl SlicerCachePart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerCacheDefinition,
    SlicerCachePart,
    as_slicer_cache_part,
    as_slicer_cache_part_mut
  );
}
