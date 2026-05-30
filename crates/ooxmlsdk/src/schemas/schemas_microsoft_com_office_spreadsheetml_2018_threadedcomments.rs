//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PersonList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xltc:personList")]
pub struct PersonList {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  /// Defines the Person Class.
  #[sdk(child(qname = "xltc:person"))]
  pub person: Vec<Person>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "xltc:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ThreadedComments Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xltc:ThreadedComments")]
pub struct ThreadedComments {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  /// Defines the ThreadedComment Class.
  #[sdk(child(qname = "xltc:threadedComment"))]
  pub threaded_comment: Vec<ThreadedComment>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "xltc:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Person Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xltc:person")]
pub struct Person {
  /// displayName
  #[sdk(attr(qname = ":displayName"))]
  pub display_name: crate::simple_type::StringValue,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// userId
  #[sdk(attr(qname = ":userId"))]
  pub user_id: Option<crate::simple_type::StringValue>,
  /// providerId
  #[sdk(attr(qname = ":providerId"))]
  pub provider_id: Option<crate::simple_type::StringValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "xltc:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xltc:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "x:ext"))]
  pub extension: Vec<crate::schemas::x::Extension>,
}
/// Defines the ThreadedComment Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xltc:threadedComment")]
pub struct ThreadedComment {
  /// ref
  #[sdk(attr(qname = ":ref"))]
  pub r#ref: Option<crate::simple_type::StringValue>,
  /// dT
  #[sdk(attr(qname = ":dT"))]
  pub d_t: Option<crate::simple_type::DateTimeValue>,
  /// personId
  #[sdk(attr(qname = ":personId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub person_id: crate::simple_type::StringValue,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// parentId
  #[sdk(attr(qname = ":parentId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub parent_id: Option<crate::simple_type::StringValue>,
  /// done
  #[sdk(attr(qname = ":done"))]
  pub done: Option<crate::simple_type::BooleanValue>,
  /// Defines the ThreadedCommentText Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "xltc:text"))]
  pub threaded_comment_text: Option<ThreadedCommentText>,
  /// Defines the ThreadedCommentMentions Class.
  #[sdk(child(qname = "xltc:mentions"))]
  pub threaded_comment_mentions: Option<ThreadedCommentMentions>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "xltc:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ThreadedCommentText Class.
pub type ThreadedCommentText = crate::simple_type::StringValue;
/// Defines the ThreadedCommentMentions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xltc:mentions")]
pub struct ThreadedCommentMentions {
  /// Defines the Mention Class.
  #[sdk(child(qname = "xltc:mention"))]
  pub mention: Vec<Mention>,
}
/// Defines the Mention Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xltc:mention")]
pub struct Mention {
  /// mentionpersonId
  #[sdk(attr(qname = ":mentionpersonId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub mentionperson_id: crate::simple_type::StringValue,
  /// mentionId
  #[sdk(attr(qname = ":mentionId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub mention_id: crate::simple_type::StringValue,
  /// startIndex
  #[sdk(attr(qname = ":startIndex"))]
  pub start_index: crate::simple_type::UInt32Value,
  /// length
  #[sdk(attr(qname = ":length"))]
  pub length: crate::simple_type::UInt32Value,
}
