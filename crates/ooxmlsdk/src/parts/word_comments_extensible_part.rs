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
  #[sdk(part_root(accessor = "as_word_comments_extensible_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_microsoft_com_office_word_2018_wordml_cex::CommentsExtensible,
  >,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
