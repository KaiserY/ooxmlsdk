//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2017/10/relationships/threadedComment";
pub const PATH_PREFIX: &str = "../threadedcomments";
pub const CONTENT_TYPE: &str = "application/vnd.ms-excel.threadedcomments+xml";
pub const TARGET_NAME: &str = "threadedcomment";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WorksheetThreadedCommentsPart {
    pub(crate) relationship_id: Option<String>,
    pub(crate) id: crate::common::PartId,
    #[sdk(part_root(accessor = "as_worksheet_threaded_comments_part"))]
    pub(crate) root_element: crate::sdk::PartRoot<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::ThreadedComments,
    >,
    pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
    pub(crate) relationship_order: Vec<Box<str>>,
    pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
    pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
    pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl WorksheetThreadedCommentsPart {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] = &[];
}
