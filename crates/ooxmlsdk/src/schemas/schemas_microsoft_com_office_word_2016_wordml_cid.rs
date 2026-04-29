//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CommentsIds Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is w16cid:commentsIds.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w16cid:CT_CommentsIds/w16cid:commentsIds")]
pub struct CommentsIds {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// _
  #[sdk(child(qname = "w16cid:CT_CommentId/w16cid:commentId"))]
  pub w16cid_comment_id: Vec<CommentId>,
}
/// Defines the CommentId Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is w16cid:commentId.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w16cid:CT_CommentId/w16cid:commentId")]
pub struct CommentId {
  /// paraId
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: w16cid:paraId
  #[sdk(attr(qname = "w16cid:paraId"))]
  #[sdk(string_length(source = 2u32, union = 0u64, min = 4u32, max = 4u32))]
  pub w16cid_para_id: crate::simple_type::HexBinaryValue,
  /// durableId
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: w16cid:durableId
  #[sdk(attr(qname = "w16cid:durableId"))]
  #[sdk(string_length(source = 2u32, union = 0u64, min = 4u32, max = 4u32))]
  pub w16cid_durable_id: crate::simple_type::HexBinaryValue,
}
