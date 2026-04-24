//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CommentV2MonikerList Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc2:cmMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc2:CT_CommentV2MonikerList/pc2:cmMkLst")]
pub struct CommentV2MonikerList {
  /// _
  #[sdk(child(qname = "pc:CT_SlideMonikerList/pc:sldMkLst"))]
  pub slide_moniker_list: std::boxed::Box<
    crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::SlideMonikerList,
  >,
  /// _
  #[sdk(child(qname = "pc2:CT_CommentV2Moniker/pc2:cmMK"))]
  pub comment_v2_moniker: std::boxed::Box<CommentV2Moniker>,
}
/// Defines the CommentReplyV2MonikerList Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc2:cmRplyMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc2:CT_CommentReplyV2MonikerList/pc2:cmRplyMkLst")]
pub struct CommentReplyV2MonikerList {
  /// _
  #[sdk(child(qname = "pc2:CT_CommentV2MonikerList/pc2:cmMkLst"))]
  pub comment_v2_moniker_list: std::boxed::Box<CommentV2MonikerList>,
  /// _
  #[sdk(child(qname = "pc2:CT_CommentReplyV2Moniker/pc2:cmRplyMk"))]
  pub comment_reply_v2_moniker: std::boxed::Box<CommentReplyV2Moniker>,
}
/// Defines the CommentV2Moniker Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc2:cmMK.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc2:CT_CommentV2Moniker/pc2:cmMK")]
pub struct CommentV2Moniker {
  /// id
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the CommentReplyV2Moniker Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc2:cmRplyMk.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc2:CT_CommentReplyV2Moniker/pc2:cmRplyMk")]
pub struct CommentReplyV2Moniker {
  /// id
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
}
