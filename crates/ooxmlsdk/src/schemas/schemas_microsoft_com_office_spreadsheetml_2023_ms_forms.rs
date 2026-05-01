//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Question Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xlmsforms:CT_Question/xlmsforms:question")]
pub struct Question {
  /// id
  #[sdk(attr(microsoft365, qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(microsoft365, qname = "x:CT_ExtensionList/xlmsforms:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the MsForm Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xlmsforms:CT_MsForm/xlmsforms:msForm")]
pub struct MsForm {
  /// id
  #[sdk(attr(microsoft365, qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// isFormConnected
  #[sdk(attr(microsoft365, qname = ":isFormConnected"))]
  pub is_form_connected: Option<crate::simple_type::BooleanValue>,
  /// maxResponseId
  #[sdk(attr(microsoft365, qname = ":maxResponseId"))]
  pub max_response_id: Option<crate::simple_type::Int32Value>,
  /// latestEventMarker
  #[sdk(attr(microsoft365, qname = ":latestEventMarker"))]
  pub latest_event_marker: Option<crate::simple_type::StringValue>,
  /// Defines the SyncedQuestionId Class.
  #[sdk(text_child(microsoft365, qname = "x:ST_Xstring/xlmsforms:syncedQuestionId"))]
  pub xlmsforms_synced_question_id: Vec<crate::simple_type::StringValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(microsoft365, qname = "x:CT_ExtensionList/xlmsforms:extLst"))]
  pub xlmsforms_ext_lst: Option<ExtensionList>,
}
/// Defines the SyncedQuestionId Class.
pub type SyncedQuestionId = crate::simple_type::StringValue;
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "x:CT_ExtensionList/xlmsforms:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub x_ext: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
