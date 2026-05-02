//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CommentsExtensible Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2021,
  qname = "w16cex:CT_CommentsExtensible/w16cex:commentsExtensible"
)]
pub struct CommentsExtensible {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Defines the CommentExtensible Class.
  #[sdk(child(
    office2021,
    qname = "w16cex:CT_CommentExtensible/w16cex:commentExtensible"
  ))]
  pub w16cex_comment_extensible: Vec<CommentExtensible>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2021, qname = "w16cur:CT_ExtensionList/w16cex:extLst"))]
  pub w16cex_ext_lst: Option<ExtensionList>,
}
/// Defines the CommentExtensible Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2021,
  qname = "w16cex:CT_CommentExtensible/w16cex:commentExtensible"
)]
pub struct CommentExtensible {
  /// durableId
  #[sdk(attr(office2021, qname = "w16cex:durableId"))]
  #[sdk(string_length(source = 2u32, union = 0u64, min = 4u32, max = 4u32))]
  pub w16cex_durable_id: crate::simple_type::HexBinaryValue,
  /// dateUtc
  #[sdk(attr(office2021, qname = "w16cex:dateUtc"))]
  pub w16cex_date_utc: Option<crate::simple_type::DateTimeValue>,
  /// intelligentPlaceholder
  #[sdk(attr(office2021, qname = "w16cex:intelligentPlaceholder"))]
  pub w16cex_intelligent_placeholder: Option<crate::simple_type::OnOffValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2021, qname = "w16cur:CT_ExtensionList/w16cex:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "w16cur:CT_ExtensionList/w16cex:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the Extension Class.
  #[sdk(child(office2021, qname = "w16cur:CT_Extension/w16cur:ext"))]
  pub w16cur_ext: Vec<crate::schemas::w16cur::Extension>,
}
