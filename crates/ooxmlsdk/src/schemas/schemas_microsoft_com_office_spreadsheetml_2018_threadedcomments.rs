//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PersonList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xltc:CT_PersonList/xltc:personList")]
pub struct PersonList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Defines the Person Class.
  #[sdk(child(office2019, qname = "xltc:CT_Person/xltc:person"))]
  pub xltc_person: Vec<Person>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xltc:extLst"))]
  pub xltc_ext_lst: Option<ExtensionList>,
}
/// Defines the ThreadedComments Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xltc:CT_ThreadedComments/xltc:ThreadedComments")]
pub struct ThreadedComments {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Defines the ThreadedComment Class.
  #[sdk(child(office2019, qname = "xltc:CT_ThreadedComment/xltc:threadedComment"))]
  pub xltc_threaded_comment: Vec<ThreadedComment>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xltc:extLst"))]
  pub xltc_ext_lst: Option<ExtensionList>,
}
/// Defines the Person Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xltc:CT_Person/xltc:person")]
pub struct Person {
  /// displayName
  #[sdk(attr(office2019, qname = ":displayName"))]
  pub display_name: crate::simple_type::StringValue,
  /// id
  #[sdk(attr(office2019, qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// userId
  #[sdk(attr(office2019, qname = ":userId"))]
  pub user_id: Option<crate::simple_type::StringValue>,
  /// providerId
  #[sdk(attr(office2019, qname = ":providerId"))]
  pub provider_id: Option<crate::simple_type::StringValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xltc:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "x:CT_ExtensionList/xltc:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub x_ext: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
/// Defines the ThreadedComment Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xltc:CT_ThreadedComment/xltc:threadedComment")]
pub struct ThreadedComment {
  /// ref
  #[sdk(attr(office2019, qname = ":ref"))]
  pub r#ref: Option<crate::simple_type::StringValue>,
  /// dT
  #[sdk(attr(office2019, qname = ":dT"))]
  pub d_t: Option<crate::simple_type::DateTimeValue>,
  /// personId
  #[sdk(attr(office2019, qname = ":personId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub person_id: crate::simple_type::StringValue,
  /// id
  #[sdk(attr(office2019, qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// parentId
  #[sdk(attr(office2019, qname = ":parentId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub parent_id: Option<crate::simple_type::StringValue>,
  /// done
  #[sdk(attr(office2019, qname = ":done"))]
  pub done: Option<crate::simple_type::BooleanValue>,
  /// Defines the ThreadedCommentText Class.
  #[sdk(text_child(office2019, qname = "x:ST_Xstring/xltc:text"))]
  pub threaded_comment_text: Option<crate::simple_type::StringValue>,
  /// Defines the ThreadedCommentMentions Class.
  #[sdk(child(office2019, qname = "xltc:CT_ThreadedCommentMentions/xltc:mentions"))]
  pub threaded_comment_mentions: Option<ThreadedCommentMentions>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xltc:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ThreadedCommentText Class.
pub type ThreadedCommentText = crate::simple_type::StringValue;
/// Defines the ThreadedCommentMentions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xltc:CT_ThreadedCommentMentions/xltc:mentions")]
pub struct ThreadedCommentMentions {
  /// Defines the Mention Class.
  #[sdk(child(office2019, qname = "xltc:CT_Mention/xltc:mention"))]
  pub xltc_mention: Vec<Mention>,
}
/// Defines the Mention Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xltc:CT_Mention/xltc:mention")]
pub struct Mention {
  /// mentionpersonId
  #[sdk(attr(office2019, qname = ":mentionpersonId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub mentionperson_id: crate::simple_type::StringValue,
  /// mentionId
  #[sdk(attr(office2019, qname = ":mentionId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub mention_id: crate::simple_type::StringValue,
  /// startIndex
  #[sdk(attr(office2019, qname = ":startIndex"))]
  pub start_index: crate::simple_type::UInt32Value,
  /// length
  #[sdk(attr(office2019, qname = ":length"))]
  pub length: crate::simple_type::UInt32Value,
}
