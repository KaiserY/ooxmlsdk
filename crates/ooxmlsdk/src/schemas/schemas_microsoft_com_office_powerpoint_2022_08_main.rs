//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the TaskDetails Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p228:CT_TaskDetails/p228:taskDetails")]
pub struct TaskDetails {
  /// deleted
  #[sdk(attr(microsoft365, qname = ":deleted"))]
  pub deleted: Option<crate::simple_type::BooleanValue>,
  /// inactive
  #[sdk(attr(microsoft365, qname = ":inactive"))]
  pub inactive: Option<crate::simple_type::BooleanValue>,
  /// Defines the TaskHistory Class.
  #[sdk(child(microsoft365, qname = "p228:CT_TaskHistory/p228:history"))]
  pub task_history: std::boxed::Box<TaskHistory>,
  /// Defines the ExtensionList Class.
  #[sdk(child(microsoft365, qname = "p:CT_ExtensionList/p228:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CommentAnchor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p228:CT_CommentAnchor/p228:comment")]
pub struct CommentAnchor {
  /// id
  #[sdk(attr(microsoft365, qname = ":id"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p:CT_ExtensionList/p228:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub p_ext: Vec<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension>,
}
/// Defines the AtrbtnTaskAssignUnassignUser Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p228:CT_TaskAssignUnassignUser/p228:atrbtn")]
pub struct AtrbtnTaskAssignUnassignUser {
  /// authorId
  #[sdk(attr(office2021, qname = ":authorId"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
}
/// Defines the AsgnTaskAssignUnassignUser Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p228:CT_TaskAssignUnassignUser/p228:asgn")]
pub struct AsgnTaskAssignUnassignUser {
  /// authorId
  #[sdk(attr(office2021, qname = ":authorId"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
}
/// Defines the TaskAnchor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p228:CT_TaskAnchor/p228:anchr")]
pub struct TaskAnchor {
  /// Defines the CommentAnchor Class.
  #[sdk(child(microsoft365, qname = "p228:CT_CommentAnchor/p228:comment"))]
  pub comment_anchor: std::boxed::Box<CommentAnchor>,
  /// Defines the ExtensionList Class.
  #[sdk(child(microsoft365, qname = "p:CT_ExtensionList/p228:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TaskTitleEventInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p228:CT_TaskTitleEventInfo/p228:title")]
pub struct TaskTitleEventInfo {
  /// val
  #[sdk(attr(microsoft365, qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the TaskScheduleEventInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p228:CT_TaskScheduleEventInfo/p228:date")]
pub struct TaskScheduleEventInfo {
  /// stDt
  #[sdk(attr(microsoft365, qname = ":stDt"))]
  pub st_dt: Option<crate::simple_type::DateTimeValue>,
  /// endDt
  #[sdk(attr(microsoft365, qname = ":endDt"))]
  pub end_dt: Option<crate::simple_type::DateTimeValue>,
}
/// Defines the TaskProgressEventInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p228:CT_TaskProgressEventInfo/p228:pcntCmplt")]
pub struct TaskProgressEventInfo {
  /// val
  #[sdk(attr(microsoft365, qname = ":val"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p228:CT_TaskUndo/p228:undo")]
pub struct TaskUndo {
  /// id
  #[sdk(attr(microsoft365, qname = ":id"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the TaskHistoryEvent Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p228:CT_TaskHistoryEvent/p228:event")]
pub struct TaskHistoryEvent {
  /// time
  #[sdk(attr(microsoft365, qname = ":time"))]
  pub time: crate::simple_type::DateTimeValue,
  /// id
  #[sdk(attr(microsoft365, qname = ":id"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the AtrbtnTaskAssignUnassignUser Class.
  #[sdk(child(microsoft365, qname = "p228:CT_TaskAssignUnassignUser/p228:atrbtn"))]
  pub atrbtn_task_assign_unassign_user: Option<AtrbtnTaskAssignUnassignUser>,
  /// Defines the TaskAnchor Class.
  #[sdk(child(microsoft365, qname = "p228:CT_TaskAnchor/p228:anchr"))]
  pub task_anchor: Option<std::boxed::Box<TaskAnchor>>,
  #[sdk(choice(
    microsoft365,
    qname = "p228:CT_TaskAssignUnassignUser/p228:asgn",
    qname = "p:CT_Empty/p228:add",
    qname = "p228:CT_TaskTitleEventInfo/p228:title",
    qname = "p228:CT_TaskScheduleEventInfo/p228:date",
    qname = "p228:CT_TaskProgressEventInfo/p228:pcntCmplt",
    qname = "p:CT_Empty/p228:unasgnAll",
    qname = "p228:CT_TaskUndo/p228:undo",
    qname = "p228:CT_TaskUnknownRecord/p228:unknown"
  ))]
  pub task_history_event_choice: Option<TaskHistoryEventChoice>,
  /// Defines the ExtensionList Class.
  #[sdk(child(microsoft365, qname = "p:CT_ExtensionList/p228:extLst"))]
  pub p228_ext_lst: Option<ExtensionList>,
}
/// Defines the TaskHistory Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p228:CT_TaskHistory/p228:history")]
pub struct TaskHistory {
  /// Defines the TaskHistoryEvent Class.
  #[sdk(child(microsoft365, qname = "p228:CT_TaskHistoryEvent/p228:event"))]
  pub p228_event: Vec<TaskHistoryEvent>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TaskHistoryEventChoice {
  #[sdk(child(microsoft365, qname = "p228:CT_TaskAssignUnassignUser/p228:asgn"))]
  P228Asgn(std::boxed::Box<AsgnTaskAssignUnassignUser>),
  /// Defines the AddEmpty Class.
  #[sdk(empty_child(microsoft365, qname = "p:CT_Empty/p228:add"))]
  P228Add,
  #[sdk(child(microsoft365, qname = "p228:CT_TaskTitleEventInfo/p228:title"))]
  P228Title(std::boxed::Box<TaskTitleEventInfo>),
  #[sdk(child(microsoft365, qname = "p228:CT_TaskScheduleEventInfo/p228:date"))]
  P228Date(std::boxed::Box<TaskScheduleEventInfo>),
  #[sdk(child(microsoft365, qname = "p228:CT_TaskProgressEventInfo/p228:pcntCmplt"))]
  P228PcntCmplt(std::boxed::Box<TaskProgressEventInfo>),
  /// Defines the UnasgnAllEmpty Class.
  #[sdk(empty_child(microsoft365, qname = "p:CT_Empty/p228:unasgnAll"))]
  P228UnasgnAll,
  #[sdk(child(microsoft365, qname = "p228:CT_TaskUndo/p228:undo"))]
  P228Undo(std::boxed::Box<TaskUndo>),
  /// Defines the TaskUnknownRecord Class.
  #[sdk(empty_child(microsoft365, qname = "p228:CT_TaskUnknownRecord/p228:unknown"))]
  P228Unknown,
}
