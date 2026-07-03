//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Question Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlmsforms:question")]
pub struct Question {
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "xlmsforms:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the MsForm Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlmsforms:msForm")]
pub struct MsForm {
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// isFormConnected
  #[sdk(attr(qname = ":isFormConnected"))]
  pub is_form_connected: Option<crate::simple_type::BooleanValue>,
  /// maxResponseId
  #[sdk(attr(qname = ":maxResponseId"))]
  pub max_response_id: Option<crate::simple_type::Int32Value>,
  /// latestEventMarker
  #[sdk(attr(qname = ":latestEventMarker"))]
  pub latest_event_marker: Option<crate::simple_type::StringValue>,
  /// Defines the SyncedQuestionId Class.
  #[sdk(text_child(qname = "xlmsforms:syncedQuestionId"))]
  pub synced_question_id: Vec<SyncedQuestionId>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "xlmsforms:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the SyncedQuestionId Class.
pub type SyncedQuestionId = crate::simple_type::StringValue;
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlmsforms:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "x:ext"))]
  pub extension: Vec<crate::schemas::x::Extension>,
}
