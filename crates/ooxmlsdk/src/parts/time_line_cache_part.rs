//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2011/relationships/timelineCache";
pub const PATH_PREFIX: &str = "timelineCaches";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct TimeLineCachePart {
    #[sdk(part_rid)]
    pub r_id: String,
    #[sdk(part_inner_path)]
    pub inner_path: String,
    #[sdk(part_root)]
    pub root_element: crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineCacheDefinition,
}
