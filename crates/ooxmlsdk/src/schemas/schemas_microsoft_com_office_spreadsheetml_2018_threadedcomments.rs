//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PersonList Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xltc:personList.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xltc:CT_PersonList/xltc:personList")]
pub struct PersonList {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "xltc:CT_Person/xltc:person"))]
  pub xltc_person: Vec<Person>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xltc:extLst"))]
  pub xltc_ext_lst: Option<ExtensionList>,
}
/// Defines the ThreadedComments Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xltc:ThreadedComments.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xltc:CT_ThreadedComments/xltc:ThreadedComments")]
pub struct ThreadedComments {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "xltc:CT_ThreadedComment/xltc:threadedComment"))]
  pub xltc_threaded_comment: Vec<ThreadedComment>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xltc:extLst"))]
  pub xltc_ext_lst: Option<ExtensionList>,
}
/// Defines the Person Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xltc:person.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xltc:CT_Person/xltc:person")]
pub struct Person {
  /// displayName
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :displayName
  #[sdk(attr(qname = ":displayName"))]
  pub display_name: crate::simple_type::StringValue,
  /// id
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// userId
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :userId
  #[sdk(attr(qname = ":userId"))]
  pub user_id: Option<crate::simple_type::StringValue>,
  /// providerId
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :providerId
  #[sdk(attr(qname = ":providerId"))]
  pub provider_id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xltc:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xltc:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExtensionList/xltc:extLst")]
pub struct ExtensionList {
  ///Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
/// Defines the ThreadedComment Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xltc:threadedComment.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xltc:CT_ThreadedComment/xltc:threadedComment")]
pub struct ThreadedComment {
  /// ref
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub r#ref: Option<crate::simple_type::StringValue>,
  /// dT
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :dT
  #[sdk(attr(qname = ":dT"))]
  pub d_t: Option<crate::simple_type::DateTimeValue>,
  /// personId
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :personId
  #[sdk(attr(qname = ":personId"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub person_id: crate::simple_type::StringValue,
  /// id
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// parentId
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :parentId
  #[sdk(attr(qname = ":parentId"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub parent_id: Option<crate::simple_type::StringValue>,
  /// done
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :done
  #[sdk(attr(qname = ":done"))]
  pub done: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(text_child(qname = "x:ST_Xstring/xltc:text"))]
  pub threaded_comment_text: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "xltc:CT_ThreadedCommentMentions/xltc:mentions"))]
  pub threaded_comment_mentions: Option<ThreadedCommentMentions>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xltc:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ThreadedCommentText Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xltc:text.
pub type ThreadedCommentText = crate::simple_type::StringValue;
/// Defines the ThreadedCommentMentions Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xltc:mentions.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xltc:CT_ThreadedCommentMentions/xltc:mentions")]
pub struct ThreadedCommentMentions {
  /// _
  #[sdk(child(qname = "xltc:CT_Mention/xltc:mention"))]
  pub xltc_mention: Vec<Mention>,
}
/// Defines the Mention Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xltc:mention.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xltc:CT_Mention/xltc:mention")]
pub struct Mention {
  /// mentionpersonId
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :mentionpersonId
  #[sdk(attr(qname = ":mentionpersonId"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub mentionperson_id: crate::simple_type::StringValue,
  /// mentionId
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :mentionId
  #[sdk(attr(qname = ":mentionId"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub mention_id: crate::simple_type::StringValue,
  /// startIndex
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :startIndex
  #[sdk(attr(qname = ":startIndex"))]
  pub start_index: crate::simple_type::UInt32Value,
  /// length
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :length
  #[sdk(attr(qname = ":length"))]
  pub length: crate::simple_type::UInt32Value,
}
