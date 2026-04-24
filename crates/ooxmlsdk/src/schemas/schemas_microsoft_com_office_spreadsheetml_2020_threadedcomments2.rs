//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the XsdunsignedInt Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xltc2:checksum.
pub type XsdunsignedInt = crate::simple_type::UInt32Value;
/// Defines the CommentHyperlink Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xltc2:hyperlink.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xltc2:CT_CommentHyperlink/xltc2:hyperlink")]
pub struct CommentHyperlink {
  /// startIndex
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :startIndex
  #[sdk(attr(qname = ":startIndex"))]
  pub start_index: crate::simple_type::UInt32Value,
  /// length
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :length
  #[sdk(attr(qname = ":length"))]
  pub length: crate::simple_type::UInt32Value,
  /// url
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :url
  #[sdk(attr(qname = ":url"))]
  pub url: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xltc2:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xltc2:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExtensionList/xltc2:extLst")]
pub struct ExtensionList {
  ///Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
