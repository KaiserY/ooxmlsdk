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
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct TimeLineCachePart {
    pub(crate) relationship_id: Option<String>,
    pub(crate) id: crate::common::PartId,
    #[sdk(part_root(accessor = "as_time_line_cache_part"))]
    pub(crate) root_element: crate::sdk::PartRoot<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineCacheDefinition,
    >,
    pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
    pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
    pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
    pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
    pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
