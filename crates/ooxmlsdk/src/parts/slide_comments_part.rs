//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments";
pub const PATH_PREFIX: &str = "../comments";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.presentationml.comments+xml";
pub const TARGET_NAME: &str = "comment";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct SlideCommentsPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl SlideCommentsPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentList,
    SlideCommentsPart,
    as_slide_comments_part,
    as_slide_comments_part_mut
  );
}
