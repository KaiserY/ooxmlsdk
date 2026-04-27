//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2018/08/relationships/commentsExtensible";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtensible+xml";
pub const TARGET_NAME: &str = "commentsExtensible";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WordCommentsExtensiblePart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl WordCommentsExtensiblePart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_word_2018_wordml_cex::CommentsExtensible,
    WordCommentsExtensiblePart,
    as_word_comments_extensible_part,
    as_word_comments_extensible_part_mut
  );
}
