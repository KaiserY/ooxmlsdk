//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Tasks Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:CT_Tasks/t:Tasks")]
pub struct Tasks {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Defines the Task Class.
  #[sdk(child(office2021, qname = "t:CT_Task/t:Task"))]
  pub t_task: Vec<Task>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2021, qname = "oel:CT_ExtensionList/t:extLst"))]
  pub t_ext_lst: Option<ExtensionList>,
}
/// Defines the Task Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:CT_Task/t:Task")]
pub struct Task {
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the TaskAnchor Class.
  #[sdk(child(office2021, qname = "t:CT_TaskAnchor/t:Anchor"))]
  pub task_anchor: Option<std::boxed::Box<TaskAnchor>>,
  /// Defines the TaskHistory Class.
  #[sdk(child(office2021, qname = "t:CT_TaskHistory/t:History"))]
  pub task_history: Option<TaskHistory>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2021, qname = "oel:CT_ExtensionList/t:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "oel:CT_ExtensionList/t:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the Extension Class.
  #[sdk(child(office2021, qname = "oel:CT_Extension/oel:ext"))]
  pub oel_ext: Vec<crate::schemas::schemas_microsoft_com_office_2019_extlst::Extension>,
}
/// Defines the TaskAnchor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:CT_TaskAnchor/t:Anchor")]
pub struct TaskAnchor {
  /// Defines the CommentAnchor Class.
  #[sdk(child(office2021, qname = "t:CT_CommentAnchor/t:Comment"))]
  pub comment_anchor: Option<CommentAnchor>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2021, qname = "oel:CT_ExtensionList/t:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TaskHistory Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:CT_TaskHistory/t:History")]
pub struct TaskHistory {
  /// Defines the TaskHistoryEvent Class.
  #[sdk(child(office2021, qname = "t:CT_TaskHistoryEvent/t:Event"))]
  pub t_event: Vec<TaskHistoryEvent>,
}
/// Defines the TaskHistoryEvent Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:CT_TaskHistoryEvent/t:Event")]
pub struct TaskHistoryEvent {
  /// time
  #[sdk(attr(office2021, qname = ":time"))]
  pub time: crate::simple_type::DateTimeValue,
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the AttributionTaskUser Class.
  #[sdk(child(office2021, qname = "t:CT_TaskUser/t:Attribution"))]
  pub attribution_task_user: Option<AttributionTaskUser>,
  /// Defines the TaskAnchor Class.
  #[sdk(child(office2021, qname = "t:CT_TaskAnchor/t:Anchor"))]
  pub task_anchor: Option<std::boxed::Box<TaskAnchor>>,
  #[sdk(choice(
    microsoft365,
    qname = "t:CT_TaskUser/t:Assign",
    qname = "t:CT_TaskUser/t:Unassign",
    qname = "t:CT_TaskCreateEventInfo/t:Create",
    qname = "t:CT_TaskTitleEventInfo/t:SetTitle",
    qname = "t:CT_TaskScheduleEventInfo/t:Schedule",
    qname = "t:CT_TaskProgressEventInfo/t:Progress",
    qname = "t:CT_TaskPriorityEventInfo/t:Priority",
    qname = "t:CT_TaskDeleteEventInfo/t:Delete",
    qname = "t:CT_TaskUndeleteEventInfo/t:Undelete",
    qname = "t:CT_TaskUnassignAll/t:UnassignAll",
    qname = "t:CT_TaskUndo/t:Undo"
  ))]
  pub task_history_event_choice: Option<TaskHistoryEventChoice>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2021, qname = "oel:CT_ExtensionList/t:extLst"))]
  pub t_ext_lst: Option<ExtensionList>,
}
/// Defines the AttributionTaskUser Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:CT_TaskUser/t:Attribution")]
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
#[sdk(office2021, qname = "t:CT_TaskUser/t:Assign")]
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
#[sdk(office2021, qname = "t:CT_TaskUser/t:Unassign")]
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
#[sdk(office2021, qname = "t:CT_TaskTitleEventInfo/t:SetTitle")]
pub struct TaskTitleEventInfo {
  /// title
  #[sdk(attr(office2021, qname = ":title"))]
  pub title: crate::simple_type::StringValue,
}
/// Defines the TaskScheduleEventInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:CT_TaskScheduleEventInfo/t:Schedule")]
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
#[sdk(office2021, qname = "t:CT_TaskProgressEventInfo/t:Progress")]
pub struct TaskProgressEventInfo {
  /// percentComplete
  #[sdk(attr(office2021, qname = ":percentComplete"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "100",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub percent_complete: crate::simple_type::Int32Value,
}
/// Defines the TaskPriorityEventInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:CT_TaskPriorityEventInfo/t:Priority")]
pub struct TaskPriorityEventInfo {
  /// value
  #[sdk(attr(office2021, qname = ":value"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "10",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub value: crate::simple_type::Int32Value,
}
/// Defines the TaskUndo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:CT_TaskUndo/t:Undo")]
pub struct TaskUndo {
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the CommentAnchor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "t:CT_CommentAnchor/t:Comment")]
pub struct CommentAnchor {
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  pub id: crate::simple_type::StringValue,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TaskHistoryEventChoice {
  #[sdk(child(office2021, qname = "t:CT_TaskUser/t:Assign"))]
  TAssign(std::boxed::Box<AssignTaskUser>),
  #[sdk(child(office2021, qname = "t:CT_TaskUser/t:Unassign"))]
  TUnassign(std::boxed::Box<UnassignTaskUser>),
  /// Defines the TaskCreateEventInfo Class.
  #[sdk(empty_child(office2021, qname = "t:CT_TaskCreateEventInfo/t:Create"))]
  TCreate,
  #[sdk(child(office2021, qname = "t:CT_TaskTitleEventInfo/t:SetTitle"))]
  TSetTitle(std::boxed::Box<TaskTitleEventInfo>),
  #[sdk(child(office2021, qname = "t:CT_TaskScheduleEventInfo/t:Schedule"))]
  TSchedule(std::boxed::Box<TaskScheduleEventInfo>),
  #[sdk(child(office2021, qname = "t:CT_TaskProgressEventInfo/t:Progress"))]
  TProgress(std::boxed::Box<TaskProgressEventInfo>),
  #[sdk(child(office2021, qname = "t:CT_TaskPriorityEventInfo/t:Priority"))]
  TPriority(std::boxed::Box<TaskPriorityEventInfo>),
  /// Defines the TaskDeleteEventInfo Class.
  #[sdk(empty_child(office2021, qname = "t:CT_TaskDeleteEventInfo/t:Delete"))]
  TDelete,
  /// Defines the TaskUndeleteEventInfo Class.
  #[sdk(empty_child(office2021, qname = "t:CT_TaskUndeleteEventInfo/t:Undelete"))]
  TUndelete,
  /// Defines the TaskUnassignAll Class.
  #[sdk(empty_child(office2021, qname = "t:CT_TaskUnassignAll/t:UnassignAll"))]
  TUnassignAll,
  #[sdk(child(office2021, qname = "t:CT_TaskUndo/t:Undo"))]
  TUndo(std::boxed::Box<TaskUndo>),
}
