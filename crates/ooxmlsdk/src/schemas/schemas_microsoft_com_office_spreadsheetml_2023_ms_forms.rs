//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Question Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlmsforms:question.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlmsforms:CT_Question/xlmsforms:question")]
pub struct Question {
  /// id
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xlmsforms:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the MsForm Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlmsforms:msForm.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlmsforms:CT_MsForm/xlmsforms:msForm")]
pub struct MsForm {
  /// id
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// isFormConnected
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :isFormConnected
  #[sdk(attr(qname = ":isFormConnected"))]
  pub is_form_connected: Option<crate::simple_type::BooleanValue>,
  /// maxResponseId
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :maxResponseId
  #[sdk(attr(qname = ":maxResponseId"))]
  pub max_response_id: Option<crate::simple_type::Int32Value>,
  /// latestEventMarker
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :latestEventMarker
  #[sdk(attr(qname = ":latestEventMarker"))]
  pub latest_event_marker: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "x:ST_Xstring/xlmsforms:syncedQuestionId"))]
  pub xlmsforms_synced_question_id: Vec<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xlmsforms:extLst"))]
  pub xlmsforms_ext_lst: Option<ExtensionList>,
}
/// Defines the SyncedQuestionId Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlmsforms:syncedQuestionId.
pub type SyncedQuestionId = crate::simple_type::StringValue;
/// Defines the ExtensionList Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlmsforms:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExtensionList/xlmsforms:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
