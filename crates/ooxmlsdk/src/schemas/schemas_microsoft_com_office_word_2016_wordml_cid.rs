//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CommentsIds Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "w16cid:CT_CommentsIds/w16cid:commentsIds")]
pub struct CommentsIds {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Defines the CommentId Class.
  #[sdk(child(office2019, qname = "w16cid:CT_CommentId/w16cid:commentId"))]
  pub w16cid_comment_id: Vec<CommentId>,
}
/// Defines the CommentId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "w16cid:CT_CommentId/w16cid:commentId")]
pub struct CommentId {
  /// paraId
  #[sdk(attr(office2019, qname = "w16cid:paraId"))]
  #[sdk(string_length(source = 2u32, union = 0u64, min = 4u32, max = 4u32))]
  pub w16cid_para_id: crate::simple_type::HexBinaryValue,
  /// durableId
  #[sdk(attr(office2019, qname = "w16cid:durableId"))]
  #[sdk(string_length(source = 2u32, union = 0u64, min = 4u32, max = 4u32))]
  pub w16cid_durable_id: crate::simple_type::HexBinaryValue,
}
