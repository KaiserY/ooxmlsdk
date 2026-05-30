//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CommentV2MonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc2:cmMkLst")]
pub struct CommentV2MonikerList {
  /// Defines the SlideMonikerList Class.
  #[sdk(child(qname = "pc:sldMkLst"))]
  pub slide_moniker_list: std::boxed::Box<crate::schemas::pc::SlideMonikerList>,
  /// Defines the CommentV2Moniker Class.
  #[sdk(child(qname = "pc2:cmMK"))]
  pub comment_v2_moniker: std::boxed::Box<CommentV2Moniker>,
}
/// Defines the CommentReplyV2MonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc2:cmRplyMkLst")]
pub struct CommentReplyV2MonikerList {
  /// Defines the CommentV2MonikerList Class.
  #[sdk(child(qname = "pc2:cmMkLst"))]
  pub comment_v2_moniker_list: std::boxed::Box<CommentV2MonikerList>,
  /// Defines the CommentReplyV2Moniker Class.
  #[sdk(child(qname = "pc2:cmRplyMk"))]
  pub comment_reply_v2_moniker: std::boxed::Box<CommentReplyV2Moniker>,
}
/// Defines the CommentV2Moniker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc2:cmMK")]
pub struct CommentV2Moniker {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the CommentReplyV2Moniker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc2:cmRplyMk")]
pub struct CommentReplyV2Moniker {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
}
