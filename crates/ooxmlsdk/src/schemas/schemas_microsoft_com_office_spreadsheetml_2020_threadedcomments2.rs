//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the XsdunsignedInt Class.
pub type XsdunsignedInt = crate::simple_type::UInt32Value;
/// Defines the CommentHyperlink Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "xltc2:CT_CommentHyperlink/xltc2:hyperlink")]
pub struct CommentHyperlink {
  /// startIndex
  #[sdk(attr(office2021, qname = ":startIndex"))]
  pub start_index: crate::simple_type::UInt32Value,
  /// length
  #[sdk(attr(office2021, qname = ":length"))]
  pub length: crate::simple_type::UInt32Value,
  /// url
  #[sdk(attr(office2021, qname = ":url"))]
  pub url: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2021, qname = "x:CT_ExtensionList/xltc2:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "x:CT_ExtensionList/xltc2:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub x_ext: Vec<crate::schemas::x::Extension>,
}
