//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Tasks Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:Tasks")]
pub struct Tasks {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  /// Defines the Task Class.
  #[sdk(child(office2021, qname = "t:Task"))]
  pub task: Vec<Task>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2021, qname = "t:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Task Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:Task")]
pub struct Task {
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the TaskAnchor Class.
  #[sdk(child(office2021, qname = "t:Anchor"))]
  pub task_anchor: Option<std::boxed::Box<TaskAnchor>>,
  /// Defines the TaskHistory Class.
  #[sdk(child(office2021, qname = "t:History"))]
  pub task_history: Option<TaskHistory>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2021, qname = "t:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:extLst")]
pub struct ExtensionList {
  /// Defines the Extension Class.
  #[sdk(child(office2021, qname = "oel:ext"))]
  pub extension: Vec<crate::schemas::oel::Extension>,
}
/// Defines the TaskAnchor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:Anchor")]
pub struct TaskAnchor {
  /// Defines the CommentAnchor Class.
  #[sdk(child(office2021, qname = "t:Comment"))]
  pub comment_anchor: Option<CommentAnchor>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2021, qname = "t:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TaskHistory Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:History")]
pub struct TaskHistory {
  /// Defines the TaskHistoryEvent Class.
  #[sdk(child(office2021, qname = "t:Event"))]
  pub task_history_event: Vec<TaskHistoryEvent>,
}
/// Defines the TaskHistoryEvent Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:Event")]
pub struct TaskHistoryEvent {
  /// time
  #[sdk(attr(office2021, qname = ":time"))]
  pub time: crate::simple_type::DateTimeValue,
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the AttributionTaskUser Class.
  #[sdk(child(office2021, qname = "t:Attribution"))]
  pub attribution_task_user: std::boxed::Box<AttributionTaskUser>,
  /// Defines the TaskAnchor Class.
  #[sdk(child(office2021, qname = "t:Anchor"))]
  pub task_anchor: Option<std::boxed::Box<TaskAnchor>>,
  #[sdk(
        choice(
            child(variant = AssignTaskUser, qname = "t:Assign"),
            child(variant = UnassignTaskUser, qname = "t:Unassign"),
            empty_child(variant = TaskCreateEventInfo, qname = "t:Create"),
            child(variant = TaskTitleEventInfo, qname = "t:SetTitle"),
            child(variant = TaskScheduleEventInfo, qname = "t:Schedule"),
            child(variant = TaskProgressEventInfo, qname = "t:Progress"),
            child(variant = TaskPriorityEventInfo, qname = "t:Priority"),
            empty_child(variant = TaskDeleteEventInfo, qname = "t:Delete"),
            empty_child(variant = TaskUndeleteEventInfo, qname = "t:Undelete"),
            empty_child(variant = TaskUnassignAll, qname = "t:UnassignAll"),
            child(variant = TaskUndo, qname = "t:Undo")
        )
    )]
  pub task_history_event_choice: Option<TaskHistoryEventChoice>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2021, qname = "t:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the AttributionTaskUser Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:Attribution")]
pub struct AttributionTaskUser {
  /// userId
  #[sdk(attr(office2021, qname = ":userId"))]
  pub user_id: crate::simple_type::StringValue,
  /// userName
  #[sdk(attr(office2021, qname = ":userName"))]
  pub user_name: crate::simple_type::StringValue,
  /// userProvider
  #[sdk(attr(office2021, qname = ":userProvider"))]
  pub user_provider: crate::simple_type::StringValue,
}
/// Defines the AssignTaskUser Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:Assign")]
pub struct AssignTaskUser {
  /// userId
  #[sdk(attr(office2021, qname = ":userId"))]
  pub user_id: crate::simple_type::StringValue,
  /// userName
  #[sdk(attr(office2021, qname = ":userName"))]
  pub user_name: crate::simple_type::StringValue,
  /// userProvider
  #[sdk(attr(office2021, qname = ":userProvider"))]
  pub user_provider: crate::simple_type::StringValue,
}
/// Defines the UnassignTaskUser Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:Unassign")]
pub struct UnassignTaskUser {
  /// userId
  #[sdk(attr(office2021, qname = ":userId"))]
  pub user_id: crate::simple_type::StringValue,
  /// userName
  #[sdk(attr(office2021, qname = ":userName"))]
  pub user_name: crate::simple_type::StringValue,
  /// userProvider
  #[sdk(attr(office2021, qname = ":userProvider"))]
  pub user_provider: crate::simple_type::StringValue,
}
/// Defines the TaskTitleEventInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:SetTitle")]
pub struct TaskTitleEventInfo {
  /// title
  #[sdk(attr(office2021, qname = ":title"))]
  pub title: crate::simple_type::StringValue,
}
/// Defines the TaskScheduleEventInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:Schedule")]
pub struct TaskScheduleEventInfo {
  /// startDate
  #[sdk(attr(office2021, qname = ":startDate"))]
  pub start_date: Option<crate::simple_type::DateTimeValue>,
  /// dueDate
  #[sdk(attr(office2021, qname = ":dueDate"))]
  pub due_date: Option<crate::simple_type::DateTimeValue>,
}
/// Defines the TaskProgressEventInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:Progress")]
pub struct TaskProgressEventInfo {
  /// percentComplete
  #[sdk(attr(office2021, qname = ":percentComplete"))]
  #[sdk(number_range(range = 0..= 100))]
  pub percent_complete: crate::simple_type::Int32Value,
}
/// Defines the TaskPriorityEventInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:Priority")]
pub struct TaskPriorityEventInfo {
  /// value
  #[sdk(attr(office2021, qname = ":value"))]
  #[sdk(number_range(range = 0..= 10))]
  pub value: crate::simple_type::Int32Value,
}
/// Defines the TaskUndo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:Undo")]
pub struct TaskUndo {
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the CommentAnchor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:Comment")]
pub struct CommentAnchor {
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  pub id: crate::simple_type::StringValue,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TaskHistoryEventChoice {
  /// Defines the AssignTaskUser Class.
  AssignTaskUser(std::boxed::Box<AssignTaskUser>),
  /// Defines the UnassignTaskUser Class.
  UnassignTaskUser(std::boxed::Box<UnassignTaskUser>),
  /// Defines the TaskCreateEventInfo Class.
  #[sdk(empty_child(office2021, qname = "t:Create"))]
  TaskCreateEventInfo,
  /// Defines the TaskTitleEventInfo Class.
  TaskTitleEventInfo(std::boxed::Box<TaskTitleEventInfo>),
  /// Defines the TaskScheduleEventInfo Class.
  TaskScheduleEventInfo(std::boxed::Box<TaskScheduleEventInfo>),
  /// Defines the TaskProgressEventInfo Class.
  TaskProgressEventInfo(std::boxed::Box<TaskProgressEventInfo>),
  /// Defines the TaskPriorityEventInfo Class.
  TaskPriorityEventInfo(std::boxed::Box<TaskPriorityEventInfo>),
  /// Defines the TaskDeleteEventInfo Class.
  #[sdk(empty_child(office2021, qname = "t:Delete"))]
  TaskDeleteEventInfo,
  /// Defines the TaskUndeleteEventInfo Class.
  #[sdk(empty_child(office2021, qname = "t:Undelete"))]
  TaskUndeleteEventInfo,
  /// Defines the TaskUnassignAll Class.
  #[sdk(empty_child(office2021, qname = "t:UnassignAll"))]
  TaskUnassignAll,
  /// Defines the TaskUndo Class.
  TaskUndo(std::boxed::Box<TaskUndo>),
}
