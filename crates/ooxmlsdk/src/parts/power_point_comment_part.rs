//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2018/10/relationships/comments";
pub const PATH_PREFIX: &str = "../comments";
pub const CONTENT_TYPE: &str = "application/vnd.ms-powerpoint.comments+xml";
pub const TARGET_NAME: &str = "modernComment";
pub const EXTENSION: &str = "";
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct PowerPointCommentPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl PowerPointCommentPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::CommentList,
    PowerPointCommentPart,
    as_power_point_comment_part,
    as_power_point_comment_part_mut
  );
}
