//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CommentsExtensible Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is w16cex:commentsExtensible.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w16cex:CT_CommentsExtensible/w16cex:commentsExtensible")]
pub struct CommentsExtensible {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "w16cex:CT_CommentExtensible/w16cex:commentExtensible"))]
  pub w16cex_comment_extensible: Vec<CommentExtensible>,
  /// _
  #[sdk(child(qname = "w16cur:CT_ExtensionList/w16cex:extLst"))]
  pub w16cex_ext_lst: Option<ExtensionList>,
}
/// Defines the CommentExtensible Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is w16cex:commentExtensible.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w16cex:CT_CommentExtensible/w16cex:commentExtensible")]
pub struct CommentExtensible {
  /// durableId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: w16cex:durableId
  #[sdk(attr(qname = "w16cex:durableId"))]
  #[sdk(string_length(source = 2u32, union = 0u64, min = 4u32, max = 4u32))]
  pub w16cex_durable_id: crate::simple_type::HexBinaryValue,
  /// dateUtc
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: w16cex:dateUtc
  #[sdk(attr(qname = "w16cex:dateUtc"))]
  pub w16cex_date_utc: Option<crate::simple_type::DateTimeValue>,
  /// intelligentPlaceholder
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: w16cex:intelligentPlaceholder
  #[sdk(attr(qname = "w16cex:intelligentPlaceholder"))]
  pub w16cex_intelligent_placeholder: Option<crate::simple_type::OnOffValue>,
  /// _
  #[sdk(child(qname = "w16cur:CT_ExtensionList/w16cex:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is w16cex:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w16cur:CT_ExtensionList/w16cex:extLst")]
pub struct ExtensionList {
  /// _
  #[sdk(child(qname = "w16cur:CT_Extension/w16cur:ext"))]
  pub w16cur_ext: Vec<crate::schemas::schemas_microsoft_com_office_word_2018_wordml::Extension>,
}
