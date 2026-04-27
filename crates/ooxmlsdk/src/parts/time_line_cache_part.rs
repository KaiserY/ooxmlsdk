//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2011/relationships/timelineCache";
pub const PATH_PREFIX: &str = "timelineCaches";
pub const CONTENT_TYPE: &str = "application/vnd.ms-excel.timelineCache+xml";
pub const TARGET_NAME: &str = "timelineCache";
pub const EXTENSION: &str = "";
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct TimeLineCachePart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl TimeLineCachePart {
  crate::sdk_part_root_methods!(
        crate
        ::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineCacheDefinition,
        TimeLineCachePart, as_time_line_cache_part, as_time_line_cache_part_mut
    );
}
