//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CommentsExtensible Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "w16cex:commentsExtensible")]
pub struct CommentsExtensible {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Defines the CommentExtensible Class.
  #[sdk(child(qname = "w16cex:commentExtensible"))]
  pub comment_extensible: Vec<CommentExtensible>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "w16cex:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CommentExtensible Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w16cex:commentExtensible")]
pub struct CommentExtensible {
  /// durableId
  #[sdk(attr(qname = "w16cex:durableId"))]
  #[sdk(string_length(source = 2u32, union = 0u64, min = 4u32, max = 4u32))]
  pub w16cex_durable_id: crate::simple_type::HexBinaryValue,
  /// dateUtc
  #[sdk(attr(qname = "w16cex:dateUtc"))]
  pub w16cex_date_utc: Option<crate::simple_type::DateTimeValue>,
  /// intelligentPlaceholder
  #[sdk(attr(qname = "w16cex:intelligentPlaceholder"))]
  pub w16cex_intelligent_placeholder: Option<crate::simple_type::OnOffValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "w16cex:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w16cex:extLst")]
pub struct ExtensionList {
  /// Defines the Extension Class.
  #[sdk(child(qname = "w16cur:ext"))]
  pub extension: Vec<crate::schemas::w16cur::Extension>,
}
