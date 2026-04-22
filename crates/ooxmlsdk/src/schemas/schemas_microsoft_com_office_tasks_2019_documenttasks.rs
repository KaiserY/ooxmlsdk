//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Tasks Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:Tasks.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_Tasks/t:Tasks")]
pub struct Tasks {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "t:CT_Task/t:Task"))]
  pub t_task: Vec<Task>,
  /// _
  #[sdk(child(qname = "oel:CT_ExtensionList/t:extLst"))]
  pub t_ext_lst: Option<ExtensionList>,
}
/// Defines the Task Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:Task.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_Task/t:Task")]
pub struct Task {
  /// id
  ///
  /// Available in Office2021 and above.
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
  #[sdk(child(qname = "t:CT_TaskAnchor/t:Anchor"))]
  pub task_anchor: Option<std::boxed::Box<TaskAnchor>>,
  /// _
  #[sdk(child(qname = "t:CT_TaskHistory/t:History"))]
  pub task_history: Option<TaskHistory>,
  /// _
  #[sdk(child(qname = "oel:CT_ExtensionList/t:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "oel:CT_ExtensionList/t:extLst")]
pub struct ExtensionList {
  /// _
  #[sdk(child(qname = "oel:CT_Extension/oel:ext"))]
  pub oel_ext: Vec<crate::schemas::schemas_microsoft_com_office_2019_extlst::Extension>,
}
/// Defines the TaskAnchor Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:Anchor.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_TaskAnchor/t:Anchor")]
pub struct TaskAnchor {
  /// _
  #[sdk(child(qname = "t:CT_CommentAnchor/t:Comment"))]
  pub comment_anchor: Option<CommentAnchor>,
  /// _
  #[sdk(child(qname = "oel:CT_ExtensionList/t:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TaskHistory Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:History.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_TaskHistory/t:History")]
pub struct TaskHistory {
  /// _
  #[sdk(child(qname = "t:CT_TaskHistoryEvent/t:Event"))]
  pub t_event: Vec<TaskHistoryEvent>,
}
/// Defines the TaskHistoryEvent Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:Event.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_TaskHistoryEvent/t:Event")]
pub struct TaskHistoryEvent {
  /// time
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::DateTimeValue,
  /// id
  ///
  /// Available in Office2021 and above.
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
  #[sdk(child(qname = "t:CT_TaskUser/t:Attribution"))]
  pub attribution_task_user: std::boxed::Box<AttributionTaskUser>,
  /// _
  #[sdk(child(qname = "t:CT_TaskAnchor/t:Anchor"))]
  pub task_anchor: Option<std::boxed::Box<TaskAnchor>>,
  #[sdk(choice(
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
  /// _
  #[sdk(child(qname = "oel:CT_ExtensionList/t:extLst"))]
  pub t_ext_lst: Option<ExtensionList>,
}
/// Defines the AttributionTaskUser Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:Attribution.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_TaskUser/t:Attribution")]
pub struct AttributionTaskUser {
  /// userId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :userId
  #[sdk(attr(qname = ":userId"))]
  pub user_id: crate::simple_type::StringValue,
  /// userName
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :userName
  #[sdk(attr(qname = ":userName"))]
  pub user_name: crate::simple_type::StringValue,
  /// userProvider
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :userProvider
  #[sdk(attr(qname = ":userProvider"))]
  pub user_provider: crate::simple_type::StringValue,
}
/// Defines the AssignTaskUser Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:Assign.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_TaskUser/t:Assign")]
pub struct AssignTaskUser {
  /// userId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :userId
  #[sdk(attr(qname = ":userId"))]
  pub user_id: crate::simple_type::StringValue,
  /// userName
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :userName
  #[sdk(attr(qname = ":userName"))]
  pub user_name: crate::simple_type::StringValue,
  /// userProvider
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :userProvider
  #[sdk(attr(qname = ":userProvider"))]
  pub user_provider: crate::simple_type::StringValue,
}
/// Defines the UnassignTaskUser Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:Unassign.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_TaskUser/t:Unassign")]
pub struct UnassignTaskUser {
  /// userId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :userId
  #[sdk(attr(qname = ":userId"))]
  pub user_id: crate::simple_type::StringValue,
  /// userName
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :userName
  #[sdk(attr(qname = ":userName"))]
  pub user_name: crate::simple_type::StringValue,
  /// userProvider
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :userProvider
  #[sdk(attr(qname = ":userProvider"))]
  pub user_provider: crate::simple_type::StringValue,
}
/// Defines the OpenXmlTaskUserElement Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_TaskUser/")]
pub struct OpenXmlTaskUserElement {
  /// userId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :userId
  #[sdk(attr(qname = ":userId"))]
  pub user_id: crate::simple_type::StringValue,
  /// userName
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :userName
  #[sdk(attr(qname = ":userName"))]
  pub user_name: crate::simple_type::StringValue,
  /// userProvider
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :userProvider
  #[sdk(attr(qname = ":userProvider"))]
  pub user_provider: crate::simple_type::StringValue,
}
/// Defines the TaskCreateEventInfo Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:Create.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_TaskCreateEventInfo/t:Create")]
pub struct TaskCreateEventInfo {}
/// Defines the TaskTitleEventInfo Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:SetTitle.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_TaskTitleEventInfo/t:SetTitle")]
pub struct TaskTitleEventInfo {
  /// title
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  pub title: crate::simple_type::StringValue,
}
/// Defines the TaskScheduleEventInfo Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:Schedule.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_TaskScheduleEventInfo/t:Schedule")]
pub struct TaskScheduleEventInfo {
  /// startDate
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :startDate
  #[sdk(attr(qname = ":startDate"))]
  pub start_date: Option<crate::simple_type::DateTimeValue>,
  /// dueDate
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :dueDate
  #[sdk(attr(qname = ":dueDate"))]
  pub due_date: Option<crate::simple_type::DateTimeValue>,
}
/// Defines the TaskProgressEventInfo Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:Progress.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_TaskProgressEventInfo/t:Progress")]
pub struct TaskProgressEventInfo {
  /// percentComplete
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :percentComplete
  #[sdk(attr(qname = ":percentComplete"))]
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
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:Priority.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_TaskPriorityEventInfo/t:Priority")]
pub struct TaskPriorityEventInfo {
  /// value
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :value
  #[sdk(attr(qname = ":value"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "10",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub value: crate::simple_type::Int32Value,
}
/// Defines the TaskDeleteEventInfo Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:Delete.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_TaskDeleteEventInfo/t:Delete")]
pub struct TaskDeleteEventInfo {}
/// Defines the TaskUndeleteEventInfo Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:Undelete.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_TaskUndeleteEventInfo/t:Undelete")]
pub struct TaskUndeleteEventInfo {}
/// Defines the TaskUnassignAll Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:UnassignAll.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_TaskUnassignAll/t:UnassignAll")]
pub struct TaskUnassignAll {}
/// Defines the TaskUndo Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:Undo.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_TaskUndo/t:Undo")]
pub struct TaskUndo {
  /// id
  ///
  /// Available in Office2021 and above.
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
/// Defines the CommentAnchor Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is t:Comment.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "t:CT_CommentAnchor/t:Comment")]
pub struct CommentAnchor {
  /// id
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TaskHistoryEventChoice {
  #[sdk(child(qname = "t:CT_TaskUser/t:Assign"))]
  TAssign(std::boxed::Box<AssignTaskUser>),
  #[sdk(child(qname = "t:CT_TaskUser/t:Unassign"))]
  TUnassign(std::boxed::Box<UnassignTaskUser>),
  #[sdk(child(qname = "t:CT_TaskCreateEventInfo/t:Create"))]
  TCreate(std::boxed::Box<TaskCreateEventInfo>),
  #[sdk(child(qname = "t:CT_TaskTitleEventInfo/t:SetTitle"))]
  TSetTitle(std::boxed::Box<TaskTitleEventInfo>),
  #[sdk(child(qname = "t:CT_TaskScheduleEventInfo/t:Schedule"))]
  TSchedule(std::boxed::Box<TaskScheduleEventInfo>),
  #[sdk(child(qname = "t:CT_TaskProgressEventInfo/t:Progress"))]
  TProgress(std::boxed::Box<TaskProgressEventInfo>),
  #[sdk(child(qname = "t:CT_TaskPriorityEventInfo/t:Priority"))]
  TPriority(std::boxed::Box<TaskPriorityEventInfo>),
  #[sdk(child(qname = "t:CT_TaskDeleteEventInfo/t:Delete"))]
  TDelete(std::boxed::Box<TaskDeleteEventInfo>),
  #[sdk(child(qname = "t:CT_TaskUndeleteEventInfo/t:Undelete"))]
  TUndelete(std::boxed::Box<TaskUndeleteEventInfo>),
  #[sdk(child(qname = "t:CT_TaskUnassignAll/t:UnassignAll"))]
  TUnassignAll(std::boxed::Box<TaskUnassignAll>),
  #[sdk(child(qname = "t:CT_TaskUndo/t:Undo"))]
  TUndo(std::boxed::Box<TaskUndo>),
}
