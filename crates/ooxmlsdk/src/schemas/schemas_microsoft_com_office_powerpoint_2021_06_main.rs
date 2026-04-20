//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the TaskHistoryDetails Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p216:taskHistoryDetails.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskHistoryDetails/p216:taskHistoryDetails")]
pub struct TaskHistoryDetails {
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
  #[sdk(child(qname = "p216:CT_TaskHistory/p216:history"))]
  pub task_history: std::boxed::Box<TaskHistory>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p216:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CommentAnchor Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p216:comment.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_CommentAnchor/p216:comment")]
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
/// When the object is serialized out as xml, it's qualified name is p216:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ExtensionList/p216:extLst")]
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
/// When the object is serialized out as xml, it's qualified name is p216:atrbtn.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskAssignUnassignUser/p216:atrbtn")]
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
/// When the object is serialized out as xml, it's qualified name is p216:asgn.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskAssignUnassignUser/p216:asgn")]
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
/// Defines the UnAsgnTaskAssignUnassignUser Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p216:unAsgn.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskAssignUnassignUser/p216:unAsgn")]
pub struct UnAsgnTaskAssignUnassignUser {
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
#[sdk(qname = "p216:CT_TaskAssignUnassignUser/")]
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
/// When the object is serialized out as xml, it's qualified name is p216:anchr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskAnchor/p216:anchr")]
pub struct TaskAnchor {
  /// _
  #[sdk(child(qname = "p216:CT_CommentAnchor/p216:comment"))]
  pub comment_anchor: std::boxed::Box<CommentAnchor>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p216:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the AddEmpty Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p216:add.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p216:add")]
pub struct AddEmpty {}
/// Defines the UnasgnAllEmpty Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p216:unasgnAll.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p216:unasgnAll")]
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
/// When the object is serialized out as xml, it's qualified name is p216:title.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskTitleEventInfo/p216:title")]
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
/// When the object is serialized out as xml, it's qualified name is p216:date.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskScheduleEventInfo/p216:date")]
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
/// When the object is serialized out as xml, it's qualified name is p216:pcntCmplt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskProgressEventInfo/p216:pcntCmplt")]
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
/// Defines the TaskPriorityRecord Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p216:pri.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskPriorityRecord/p216:pri")]
pub struct TaskPriorityRecord {
  /// val
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "0",
    max = "10",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the TaskUndo Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p216:undo.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskUndo/p216:undo")]
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
/// When the object is serialized out as xml, it's qualified name is p216:unknown.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskUnknownRecord/p216:unknown")]
pub struct TaskUnknownRecord {}
/// Defines the TaskHistoryEvent Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p216:event.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskHistoryEvent/p216:event")]
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
  #[sdk(child(qname = "p216:CT_TaskAssignUnassignUser/p216:atrbtn"))]
  pub atrbtn_task_assign_unassign_user: std::boxed::Box<AtrbtnTaskAssignUnassignUser>,
  /// _
  #[sdk(child(qname = "p216:CT_TaskAnchor/p216:anchr"))]
  pub task_anchor: Option<std::boxed::Box<TaskAnchor>>,
  #[sdk(choice)]
  pub task_history_event_choice: Option<TaskHistoryEventChoice>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p216:extLst"))]
  pub p216_ext_lst: Option<ExtensionList>,
}
/// Defines the TaskHistory Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p216:history.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskHistory/p216:history")]
pub struct TaskHistory {
  /// _
  #[sdk(child(qname = "p216:CT_TaskHistoryEvent/p216:event"))]
  pub p216_event: Vec<TaskHistoryEvent>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TaskHistoryEventChoice {
  #[sdk(child(qname = "p216:CT_TaskAssignUnassignUser/p216:asgn"))]
  P216Asgn(std::boxed::Box<AsgnTaskAssignUnassignUser>),
  #[sdk(child(qname = "p216:CT_TaskAssignUnassignUser/p216:unAsgn"))]
  P216UnAsgn(std::boxed::Box<UnAsgnTaskAssignUnassignUser>),
  #[sdk(child(qname = "p:CT_Empty/p216:add"))]
  P216Add(std::boxed::Box<AddEmpty>),
  #[sdk(child(qname = "p216:CT_TaskTitleEventInfo/p216:title"))]
  P216Title(std::boxed::Box<TaskTitleEventInfo>),
  #[sdk(child(qname = "p216:CT_TaskScheduleEventInfo/p216:date"))]
  P216Date(std::boxed::Box<TaskScheduleEventInfo>),
  #[sdk(child(qname = "p216:CT_TaskProgressEventInfo/p216:pcntCmplt"))]
  P216PcntCmplt(std::boxed::Box<TaskProgressEventInfo>),
  #[sdk(child(qname = "p216:CT_TaskPriorityRecord/p216:pri"))]
  P216Pri(std::boxed::Box<TaskPriorityRecord>),
  #[sdk(child(qname = "p:CT_Empty/p216:unasgnAll"))]
  P216UnasgnAll(std::boxed::Box<UnasgnAllEmpty>),
  #[sdk(child(qname = "p216:CT_TaskUndo/p216:undo"))]
  P216Undo(std::boxed::Box<TaskUndo>),
  #[sdk(child(qname = "p216:CT_TaskUnknownRecord/p216:unknown"))]
  P216Unknown(std::boxed::Box<TaskUnknownRecord>),
}
