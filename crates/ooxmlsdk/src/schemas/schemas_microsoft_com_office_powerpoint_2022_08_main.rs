//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the TaskDetails Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p228:taskDetails.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:CT_TaskDetails/p228:taskDetails")]
pub struct TaskDetails {
  /// deleted
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :deleted
  #[sdk(attr(qname = ":deleted"))]
  pub deleted: Option<crate::simple_type::BooleanValue>,
  /// inactive
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :inactive
  #[sdk(attr(qname = ":inactive"))]
  pub inactive: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "p228:CT_TaskHistory/p228:history"))]
  pub task_history: std::boxed::Box<TaskHistory>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p228:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CommentAnchor Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p228:comment.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:CT_CommentAnchor/p228:comment")]
pub struct CommentAnchor {
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
/// Defines the ExtensionList Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p228:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ExtensionList/p228:extLst")]
pub struct ExtensionList {
  ///Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub extension:
    Vec<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension>,
}
/// Defines the AtrbtnTaskAssignUnassignUser Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p228:atrbtn.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:CT_TaskAssignUnassignUser/p228:atrbtn")]
pub struct AtrbtnTaskAssignUnassignUser {
  /// authorId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :authorId
  #[sdk(attr(qname = ":authorId"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
}
/// Defines the AsgnTaskAssignUnassignUser Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p228:asgn.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:CT_TaskAssignUnassignUser/p228:asgn")]
pub struct AsgnTaskAssignUnassignUser {
  /// authorId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :authorId
  #[sdk(attr(qname = ":authorId"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
}
/// Defines the OpenXmlTaskAssignUnassignUserElement Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:CT_TaskAssignUnassignUser/")]
pub struct OpenXmlTaskAssignUnassignUserElement {
  /// authorId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :authorId
  #[sdk(attr(qname = ":authorId"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
}
/// Defines the TaskAnchor Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p228:anchr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:CT_TaskAnchor/p228:anchr")]
pub struct TaskAnchor {
  /// _
  #[sdk(child(qname = "p228:CT_CommentAnchor/p228:comment"))]
  pub comment_anchor: std::boxed::Box<CommentAnchor>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p228:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the AddEmpty Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p228:add.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p228:add")]
pub struct AddEmpty {}
/// Defines the UnasgnAllEmpty Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p228:unasgnAll.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p228:unasgnAll")]
pub struct UnasgnAllEmpty {}
/// Defines the EmptyType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/")]
pub struct EmptyType {}
/// Defines the TaskTitleEventInfo Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p228:title.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:CT_TaskTitleEventInfo/p228:title")]
pub struct TaskTitleEventInfo {
  /// val
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the TaskScheduleEventInfo Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p228:date.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:CT_TaskScheduleEventInfo/p228:date")]
pub struct TaskScheduleEventInfo {
  /// stDt
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :stDt
  #[sdk(attr(qname = ":stDt"))]
  pub st_dt: Option<crate::simple_type::DateTimeValue>,
  /// endDt
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :endDt
  #[sdk(attr(qname = ":endDt"))]
  pub end_dt: Option<crate::simple_type::DateTimeValue>,
}
/// Defines the TaskProgressEventInfo Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p228:pcntCmplt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:CT_TaskProgressEventInfo/p228:pcntCmplt")]
pub struct TaskProgressEventInfo {
  /// val
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the TaskUndo Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p228:undo.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:CT_TaskUndo/p228:undo")]
pub struct TaskUndo {
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
/// Defines the TaskUnknownRecord Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p228:unknown.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:CT_TaskUnknownRecord/p228:unknown")]
pub struct TaskUnknownRecord {}
/// Defines the TaskHistoryEvent Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p228:event.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:CT_TaskHistoryEvent/p228:event")]
pub struct TaskHistoryEvent {
  /// time
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::DateTimeValue,
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
  /// _
  #[sdk(child(qname = "p228:CT_TaskAssignUnassignUser/p228:atrbtn"))]
  pub atrbtn_task_assign_unassign_user: std::boxed::Box<AtrbtnTaskAssignUnassignUser>,
  /// _
  #[sdk(child(qname = "p228:CT_TaskAnchor/p228:anchr"))]
  pub task_anchor: Option<std::boxed::Box<TaskAnchor>>,
  #[sdk(choice)]
  pub task_history_event_choice: Option<TaskHistoryEventChoice>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p228:extLst"))]
  pub p228_ext_lst: Option<ExtensionList>,
}
/// Defines the TaskHistory Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p228:history.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:CT_TaskHistory/p228:history")]
pub struct TaskHistory {
  /// _
  #[sdk(child(qname = "p228:CT_TaskHistoryEvent/p228:event"))]
  pub p228_event: Vec<TaskHistoryEvent>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TaskHistoryEventChoice {
  #[sdk(child(qname = "p228:CT_TaskAssignUnassignUser/p228:asgn"))]
  P228Asgn(std::boxed::Box<AsgnTaskAssignUnassignUser>),
  #[sdk(child(qname = "p:CT_Empty/p228:add"))]
  P228Add(std::boxed::Box<AddEmpty>),
  #[sdk(child(qname = "p228:CT_TaskTitleEventInfo/p228:title"))]
  P228Title(std::boxed::Box<TaskTitleEventInfo>),
  #[sdk(child(qname = "p228:CT_TaskScheduleEventInfo/p228:date"))]
  P228Date(std::boxed::Box<TaskScheduleEventInfo>),
  #[sdk(child(qname = "p228:CT_TaskProgressEventInfo/p228:pcntCmplt"))]
  P228PcntCmplt(std::boxed::Box<TaskProgressEventInfo>),
  #[sdk(child(qname = "p:CT_Empty/p228:unasgnAll"))]
  P228UnasgnAll(std::boxed::Box<UnasgnAllEmpty>),
  #[sdk(child(qname = "p228:CT_TaskUndo/p228:undo"))]
  P228Undo(std::boxed::Box<TaskUndo>),
  #[sdk(child(qname = "p228:CT_TaskUnknownRecord/p228:unknown"))]
  P228Unknown(std::boxed::Box<TaskUnknownRecord>),
}
