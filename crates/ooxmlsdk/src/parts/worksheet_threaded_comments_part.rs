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
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WorksheetThreadedCommentsPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl WorksheetThreadedCommentsPart {
  crate::sdk_part_root_methods!(
        crate
        ::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::ThreadedComments,
        WorksheetThreadedCommentsPart, as_worksheet_threaded_comments_part,
        as_worksheet_threaded_comments_part_mut
    );
}
