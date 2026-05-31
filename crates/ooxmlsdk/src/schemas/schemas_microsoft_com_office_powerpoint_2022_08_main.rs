//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the TaskDetails Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:taskDetails")]
pub struct TaskDetails {
  /// deleted
  #[sdk(attr(qname = ":deleted"))]
  pub deleted: Option<crate::simple_type::BooleanValue>,
  /// inactive
  #[sdk(attr(qname = ":inactive"))]
  pub inactive: Option<crate::simple_type::BooleanValue>,
  /// Defines the TaskHistory Class.
  #[sdk(child(qname = "p228:history"))]
  pub task_history: std::boxed::Box<TaskHistory>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p228:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CommentAnchor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:comment")]
pub struct CommentAnchor {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "p:ext"))]
  pub extension: Vec<crate::schemas::p::Extension>,
}
/// Defines the AtrbtnTaskAssignUnassignUser Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:atrbtn")]
pub struct AtrbtnTaskAssignUnassignUser {
  /// authorId
  #[sdk(attr(qname = ":authorId"))]
  #[sdk(string_format(kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
}
/// Defines the AsgnTaskAssignUnassignUser Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:asgn")]
pub struct AsgnTaskAssignUnassignUser {
  /// authorId
  #[sdk(attr(qname = ":authorId"))]
  #[sdk(string_format(kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
}
/// Defines the TaskAnchor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:anchr")]
pub struct TaskAnchor {
  /// Defines the CommentAnchor Class.
  #[sdk(child(qname = "p228:comment"))]
  pub comment_anchor: std::boxed::Box<CommentAnchor>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p228:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TaskTitleEventInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:title")]
pub struct TaskTitleEventInfo {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the TaskScheduleEventInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:date")]
pub struct TaskScheduleEventInfo {
  /// stDt
  #[sdk(attr(qname = ":stDt"))]
  pub st_dt: Option<crate::simple_type::DateTimeValue>,
  /// endDt
  #[sdk(attr(qname = ":endDt"))]
  pub end_dt: Option<crate::simple_type::DateTimeValue>,
}
/// Defines the TaskProgressEventInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:pcntCmplt")]
pub struct TaskProgressEventInfo {
  /// val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the TaskUndo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:undo")]
pub struct TaskUndo {
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the TaskHistoryEvent Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:event")]
pub struct TaskHistoryEvent {
  /// time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::DateTimeValue,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the AtrbtnTaskAssignUnassignUser Class.
  #[sdk(child(qname = "p228:atrbtn"))]
  pub atrbtn_task_assign_unassign_user: std::boxed::Box<AtrbtnTaskAssignUnassignUser>,
  /// Defines the TaskAnchor Class.
  #[sdk(child(qname = "p228:anchr"))]
  pub task_anchor: Option<std::boxed::Box<TaskAnchor>>,
  #[sdk(
        choice(
            child(variant = AsgnTaskAssignUnassignUser, qname = "p228:asgn"),
            empty_child(variant = AddEmpty, qname = "p228:add"),
            child(variant = TaskTitleEventInfo, qname = "p228:title"),
            child(variant = TaskScheduleEventInfo, qname = "p228:date"),
            child(variant = TaskProgressEventInfo, qname = "p228:pcntCmplt"),
            empty_child(variant = UnasgnAllEmpty, qname = "p228:unasgnAll"),
            child(variant = TaskUndo, qname = "p228:undo"),
            empty_child(variant = TaskUnknownRecord, qname = "p228:unknown")
        )
    )]
  pub task_history_event_choice: Option<TaskHistoryEventChoice>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "p228:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TaskHistory Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p228:history")]
pub struct TaskHistory {
  /// Defines the TaskHistoryEvent Class.
  #[sdk(child(qname = "p228:event"))]
  pub task_history_event: Vec<TaskHistoryEvent>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum TaskHistoryEventChoice {
  /// Defines the AsgnTaskAssignUnassignUser Class.
  AsgnTaskAssignUnassignUser(std::boxed::Box<AsgnTaskAssignUnassignUser>),
  /// Defines the AddEmpty Class.
  AddEmpty,
  /// Defines the TaskTitleEventInfo Class.
  TaskTitleEventInfo(std::boxed::Box<TaskTitleEventInfo>),
  /// Defines the TaskScheduleEventInfo Class.
  TaskScheduleEventInfo(std::boxed::Box<TaskScheduleEventInfo>),
  /// Defines the TaskProgressEventInfo Class.
  TaskProgressEventInfo(std::boxed::Box<TaskProgressEventInfo>),
  /// Defines the UnasgnAllEmpty Class.
  UnasgnAllEmpty,
  /// Defines the TaskUndo Class.
  TaskUndo(std::boxed::Box<TaskUndo>),
  /// Defines the TaskUnknownRecord Class.
  TaskUnknownRecord,
}
