//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the TaskHistoryDetails Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2021,
  qname = "p1912:CT_TaskHistoryDetails/p1912:taskHistoryDetails"
)]
pub struct TaskHistoryDetails {
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the TaskHistory Class.
  #[sdk(child(office2021, qname = "p1912:CT_TaskHistory/p1912:history"))]
  pub task_history: std::boxed::Box<TaskHistory>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2021, qname = "p:CT_ExtensionList/p1912:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CommentAnchor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p1912:CT_CommentAnchor/p1912:comment")]
pub struct CommentAnchor {
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p:CT_ExtensionList/p1912:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub p_ext: Vec<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension>,
}
/// Defines the AtrbtnTaskAssignUnassignUser Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p1912:CT_TaskAssignUnassignUser/p1912:atrbtn")]
pub struct AtrbtnTaskAssignUnassignUser {
  /// authorId
  #[sdk(attr(office2021, qname = ":authorId"))]
  #[sdk(string_format(kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
}
/// Defines the AsgnTaskAssignUnassignUser Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p1912:CT_TaskAssignUnassignUser/p1912:asgn")]
pub struct AsgnTaskAssignUnassignUser {
  /// authorId
  #[sdk(attr(office2021, qname = ":authorId"))]
  #[sdk(string_format(kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
}
/// Defines the UnAsgnTaskAssignUnassignUser Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p1912:CT_TaskAssignUnassignUser/p1912:unAsgn")]
pub struct UnAsgnTaskAssignUnassignUser {
  /// authorId
  #[sdk(attr(office2021, qname = ":authorId"))]
  #[sdk(string_format(kind = "token"))]
  pub author_id: crate::simple_type::StringValue,
}
/// Defines the TaskAnchor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p1912:CT_TaskAnchor/p1912:anchr")]
pub struct TaskAnchor {
  /// Defines the CommentAnchor Class.
  #[sdk(child(office2021, qname = "p1912:CT_CommentAnchor/p1912:comment"))]
  pub comment_anchor: std::boxed::Box<CommentAnchor>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2021, qname = "p:CT_ExtensionList/p1912:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TaskTitleEventInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p1912:CT_TaskTitleEventInfo/p1912:title")]
pub struct TaskTitleEventInfo {
  /// val
  #[sdk(attr(office2021, qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the TaskScheduleEventInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p1912:CT_TaskScheduleEventInfo/p1912:date")]
pub struct TaskScheduleEventInfo {
  /// stDt
  #[sdk(attr(office2021, qname = ":stDt"))]
  pub st_dt: Option<crate::simple_type::DateTimeValue>,
  /// endDt
  #[sdk(attr(office2021, qname = ":endDt"))]
  pub end_dt: Option<crate::simple_type::DateTimeValue>,
}
/// Defines the TaskProgressEventInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p1912:CT_TaskProgressEventInfo/p1912:pcntCmplt")]
pub struct TaskProgressEventInfo {
  /// val
  #[sdk(attr(office2021, qname = ":val"))]
  #[sdk(number_range(range = 0..= 100000))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the TaskPriorityRecord Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p1912:CT_TaskPriorityRecord/p1912:pri")]
pub struct TaskPriorityRecord {
  /// val
  #[sdk(attr(office2021, qname = ":val"))]
  #[sdk(number_range(range = 0..= 10))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the TaskUndo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p1912:CT_TaskUndo/p1912:undo")]
pub struct TaskUndo {
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the TaskHistoryEvent Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p1912:CT_TaskHistoryEvent/p1912:event")]
pub struct TaskHistoryEvent {
  /// time
  #[sdk(attr(office2021, qname = ":time"))]
  pub time: crate::simple_type::DateTimeValue,
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the AtrbtnTaskAssignUnassignUser Class.
  #[sdk(child(office2021, qname = "p1912:CT_TaskAssignUnassignUser/p1912:atrbtn"))]
  pub atrbtn_task_assign_unassign_user: Option<AtrbtnTaskAssignUnassignUser>,
  /// Defines the TaskAnchor Class.
  #[sdk(child(office2021, qname = "p1912:CT_TaskAnchor/p1912:anchr"))]
  pub task_anchor: Option<std::boxed::Box<TaskAnchor>>,
  #[sdk(choice(
    qname = "p1912:CT_TaskAssignUnassignUser/p1912:asgn",
    qname = "p1912:CT_TaskAssignUnassignUser/p1912:unAsgn",
    qname = "p:CT_Empty/p1912:add",
    qname = "p1912:CT_TaskTitleEventInfo/p1912:title",
    qname = "p1912:CT_TaskScheduleEventInfo/p1912:date",
    qname = "p1912:CT_TaskProgressEventInfo/p1912:pcntCmplt",
    qname = "p1912:CT_TaskPriorityRecord/p1912:pri",
    qname = "p:CT_Empty/p1912:unasgnAll",
    qname = "p1912:CT_TaskUndo/p1912:undo",
    qname = "p1912:CT_TaskUnknownRecord/p1912:unknown"
  ))]
  pub task_history_event_choice: Option<TaskHistoryEventChoice>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2021, qname = "p:CT_ExtensionList/p1912:extLst"))]
  pub p1912_ext_lst: Option<ExtensionList>,
}
/// Defines the TaskHistory Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "p1912:CT_TaskHistory/p1912:history")]
pub struct TaskHistory {
  /// Defines the TaskHistoryEvent Class.
  #[sdk(child(office2021, qname = "p1912:CT_TaskHistoryEvent/p1912:event"))]
  pub p1912_event: Vec<TaskHistoryEvent>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TaskHistoryEventChoice {
  #[sdk(child(office2021, qname = "p1912:CT_TaskAssignUnassignUser/p1912:asgn"))]
  P1912Asgn(std::boxed::Box<AsgnTaskAssignUnassignUser>),
  #[sdk(child(office2021, qname = "p1912:CT_TaskAssignUnassignUser/p1912:unAsgn"))]
  P1912UnAsgn(std::boxed::Box<UnAsgnTaskAssignUnassignUser>),
  /// Defines the AddEmpty Class.
  #[sdk(empty_child(office2021, qname = "p:CT_Empty/p1912:add"))]
  P1912Add,
  #[sdk(child(office2021, qname = "p1912:CT_TaskTitleEventInfo/p1912:title"))]
  P1912Title(std::boxed::Box<TaskTitleEventInfo>),
  #[sdk(child(office2021, qname = "p1912:CT_TaskScheduleEventInfo/p1912:date"))]
  P1912Date(std::boxed::Box<TaskScheduleEventInfo>),
  #[sdk(child(office2021, qname = "p1912:CT_TaskProgressEventInfo/p1912:pcntCmplt"))]
  P1912PcntCmplt(std::boxed::Box<TaskProgressEventInfo>),
  #[sdk(child(office2021, qname = "p1912:CT_TaskPriorityRecord/p1912:pri"))]
  P1912Pri(std::boxed::Box<TaskPriorityRecord>),
  /// Defines the UnasgnAllEmpty Class.
  #[sdk(empty_child(office2021, qname = "p:CT_Empty/p1912:unasgnAll"))]
  P1912UnasgnAll,
  #[sdk(child(office2021, qname = "p1912:CT_TaskUndo/p1912:undo"))]
  P1912Undo(std::boxed::Box<TaskUndo>),
  /// Defines the TaskUnknownRecord Class.
  #[sdk(empty_child(office2021, qname = "p1912:CT_TaskUnknownRecord/p1912:unknown"))]
  P1912Unknown,
}
