//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CommentV2MonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "pc2:CT_CommentV2MonikerList/pc2:cmMkLst")]
pub struct CommentV2MonikerList {
  /// Defines the SlideMonikerList Class.
  #[sdk(child(office2016, qname = "pc:CT_SlideMonikerList/pc:sldMkLst"))]
  pub slide_moniker_list: std::boxed::Box<
    crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::SlideMonikerList,
  >,
  /// Defines the CommentV2Moniker Class.
  #[sdk(child(microsoft365, qname = "pc2:CT_CommentV2Moniker/pc2:cmMK"))]
  pub comment_v2_moniker: std::boxed::Box<CommentV2Moniker>,
}
/// Defines the CommentReplyV2MonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  microsoft365,
  qname = "pc2:CT_CommentReplyV2MonikerList/pc2:cmRplyMkLst"
)]
pub struct CommentReplyV2MonikerList {
  /// Defines the CommentV2MonikerList Class.
  #[sdk(child(microsoft365, qname = "pc2:CT_CommentV2MonikerList/pc2:cmMkLst"))]
  pub comment_v2_moniker_list: std::boxed::Box<CommentV2MonikerList>,
  /// Defines the CommentReplyV2Moniker Class.
  #[sdk(child(microsoft365, qname = "pc2:CT_CommentReplyV2Moniker/pc2:cmRplyMk"))]
  pub comment_reply_v2_moniker: std::boxed::Box<CommentReplyV2Moniker>,
}
/// Defines the CommentV2Moniker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "pc2:CT_CommentV2Moniker/pc2:cmMK")]
pub struct CommentV2Moniker {
  /// id
  #[sdk(attr(microsoft365, qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the CommentReplyV2Moniker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "pc2:CT_CommentReplyV2Moniker/pc2:cmRplyMk")]
pub struct CommentReplyV2Moniker {
  /// id
  #[sdk(attr(microsoft365, qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
}
