//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the TaskHistoryDetails Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  microsoft365,
  qname = "p216:CT_TaskHistoryDetails/p216:taskHistoryDetails"
)]
pub struct TaskHistoryDetails {
  /// id
  #[sdk(attr(microsoft365, qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the TaskHistory Class.
  #[sdk(child(microsoft365, qname = "p216:CT_TaskHistory/p216:history"))]
  pub task_history: std::boxed::Box<TaskHistory>,
  /// Defines the ExtensionList Class.
  #[sdk(child(microsoft365, qname = "p:CT_ExtensionList/p216:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CommentAnchor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p216:CT_CommentAnchor/p216:comment")]
pub struct CommentAnchor {
  /// id
  #[sdk(attr(microsoft365, qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p:CT_ExtensionList/p216:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub extension: Vec<crate::schemas::p::Extension>,
}
/// Defines the AtrbtnTaskAssignUnassignUser Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p216:CT_TaskAssignUnassignUser/p216:atrbtn")]
pub struct AtrbtnTaskAssignUnassignUser {
  /// authorId
  #[sdk(attr(office2021, qname = ":authorId"))]
  #[sdk(string_format(kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
}
/// Defines the AsgnTaskAssignUnassignUser Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p216:CT_TaskAssignUnassignUser/p216:asgn")]
pub struct AsgnTaskAssignUnassignUser {
  /// authorId
  #[sdk(attr(office2021, qname = ":authorId"))]
  #[sdk(string_format(kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
}
/// Defines the UnAsgnTaskAssignUnassignUser Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p216:CT_TaskAssignUnassignUser/p216:unAsgn")]
pub struct UnAsgnTaskAssignUnassignUser {
  /// authorId
  #[sdk(attr(office2021, qname = ":authorId"))]
  #[sdk(string_format(kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
}
/// Defines the TaskAnchor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p216:CT_TaskAnchor/p216:anchr")]
pub struct TaskAnchor {
  /// Defines the CommentAnchor Class.
  #[sdk(child(microsoft365, qname = "p216:CT_CommentAnchor/p216:comment"))]
  pub comment_anchor: std::boxed::Box<CommentAnchor>,
  /// Defines the ExtensionList Class.
  #[sdk(child(microsoft365, qname = "p:CT_ExtensionList/p216:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TaskTitleEventInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p216:CT_TaskTitleEventInfo/p216:title")]
pub struct TaskTitleEventInfo {
  /// val
  #[sdk(attr(microsoft365, qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the TaskScheduleEventInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p216:CT_TaskScheduleEventInfo/p216:date")]
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
#[sdk(microsoft365, qname = "p216:CT_TaskProgressEventInfo/p216:pcntCmplt")]
pub struct TaskProgressEventInfo {
  /// val
  #[sdk(attr(microsoft365, qname = ":val"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the TaskPriorityRecord Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p216:CT_TaskPriorityRecord/p216:pri")]
pub struct TaskPriorityRecord {
  /// val
  #[sdk(attr(microsoft365, qname = ":val"))]
  #[sdk(number_range(range = 0..= 10))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the TaskUndo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p216:CT_TaskUndo/p216:undo")]
pub struct TaskUndo {
  /// id
  #[sdk(attr(microsoft365, qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the TaskHistoryEvent Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p216:CT_TaskHistoryEvent/p216:event")]
pub struct TaskHistoryEvent {
  /// time
  #[sdk(attr(microsoft365, qname = ":time"))]
  pub time: crate::simple_type::DateTimeValue,
  /// id
  #[sdk(attr(microsoft365, qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the AtrbtnTaskAssignUnassignUser Class.
  #[sdk(child(microsoft365, qname = "p216:CT_TaskAssignUnassignUser/p216:atrbtn"))]
  pub atrbtn_task_assign_unassign_user: std::boxed::Box<AtrbtnTaskAssignUnassignUser>,
  /// Defines the TaskAnchor Class.
  #[sdk(child(microsoft365, qname = "p216:CT_TaskAnchor/p216:anchr"))]
  pub task_anchor: Option<std::boxed::Box<TaskAnchor>>,
  #[sdk(
        choice(
            child(variant = AsgnTaskAssignUnassignUser, qname = "p216:asgn"),
            child(variant = UnAsgnTaskAssignUnassignUser, qname = "p216:unAsgn"),
            empty_child(variant = AddEmpty, qname = "p216:add"),
            child(variant = TaskTitleEventInfo, qname = "p216:title"),
            child(variant = TaskScheduleEventInfo, qname = "p216:date"),
            child(variant = TaskProgressEventInfo, qname = "p216:pcntCmplt"),
            child(variant = TaskPriorityRecord, qname = "p216:pri"),
            empty_child(variant = UnasgnAllEmpty, qname = "p216:unasgnAll"),
            child(variant = TaskUndo, qname = "p216:undo"),
            empty_child(variant = TaskUnknownRecord, qname = "p216:unknown")
        )
    )]
  pub task_history_event_choice: Option<TaskHistoryEventChoice>,
  /// Defines the ExtensionList Class.
  #[sdk(child(microsoft365, qname = "p:CT_ExtensionList/p216:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TaskHistory Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "p216:CT_TaskHistory/p216:history")]
pub struct TaskHistory {
  /// Defines the TaskHistoryEvent Class.
  #[sdk(child(microsoft365, qname = "p216:CT_TaskHistoryEvent/p216:event"))]
  pub task_history_event: Vec<TaskHistoryEvent>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TaskHistoryEventChoice {
  /// Defines the AsgnTaskAssignUnassignUser Class.
  AsgnTaskAssignUnassignUser(std::boxed::Box<AsgnTaskAssignUnassignUser>),
  /// Defines the UnAsgnTaskAssignUnassignUser Class.
  UnAsgnTaskAssignUnassignUser(std::boxed::Box<UnAsgnTaskAssignUnassignUser>),
  /// Defines the AddEmpty Class.
  #[sdk(empty_child(microsoft365, qname = "p:CT_Empty/p216:add"))]
  AddEmpty,
  /// Defines the TaskTitleEventInfo Class.
  TaskTitleEventInfo(std::boxed::Box<TaskTitleEventInfo>),
  /// Defines the TaskScheduleEventInfo Class.
  TaskScheduleEventInfo(std::boxed::Box<TaskScheduleEventInfo>),
  /// Defines the TaskProgressEventInfo Class.
  TaskProgressEventInfo(std::boxed::Box<TaskProgressEventInfo>),
  /// Defines the TaskPriorityRecord Class.
  TaskPriorityRecord(std::boxed::Box<TaskPriorityRecord>),
  /// Defines the UnasgnAllEmpty Class.
  #[sdk(empty_child(microsoft365, qname = "p:CT_Empty/p216:unasgnAll"))]
  UnasgnAllEmpty,
  /// Defines the TaskUndo Class.
  TaskUndo(std::boxed::Box<TaskUndo>),
  /// Defines the TaskUnknownRecord Class.
  #[sdk(empty_child(microsoft365, qname = "p216:CT_TaskUnknownRecord/p216:unknown"))]
  TaskUnknownRecord,
}
