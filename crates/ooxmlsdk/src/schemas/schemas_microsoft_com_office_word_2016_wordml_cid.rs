//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CommentsIds Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "w16cid:commentsIds")]
pub struct CommentsIds {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub mc_ignorable: Option<std::boxed::Box<[u8]>>,
  /// Defines the CommentId Class.
  #[sdk(child(qname = "w16cid:commentId"))]
  pub comment_id: Vec<CommentId>,
}
/// Defines the CommentId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w16cid:commentId")]
pub struct CommentId {
  /// paraId
  #[sdk(attr(qname = "w16cid:paraId"))]
  #[sdk(string_length(source = 2u32, union = 0u64, min = 4u32, max = 4u32))]
  pub w16cid_para_id: crate::simple_type::HexBinaryValue,
  /// durableId
  #[sdk(attr(qname = "w16cid:durableId"))]
  #[sdk(string_length(source = 2u32, union = 0u64, min = 4u32, max = 4u32))]
  pub w16cid_durable_id: crate::simple_type::HexBinaryValue,
}
