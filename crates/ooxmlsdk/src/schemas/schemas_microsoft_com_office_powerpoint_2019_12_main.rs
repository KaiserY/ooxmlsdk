//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the TaskHistoryDetails Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1912:taskHistoryDetails.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p1912:CT_TaskHistoryDetails/p1912:taskHistoryDetails")]
pub struct TaskHistoryDetails {
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
  #[sdk(child(qname = "p1912:CT_TaskHistory/p1912:history"))]
  pub task_history: std::boxed::Box<TaskHistory>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p1912:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CommentAnchor Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1912:comment.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p1912:CT_CommentAnchor/p1912:comment")]
pub struct CommentAnchor {
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
/// Defines the ExtensionList Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1912:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ExtensionList/p1912:extLst")]
pub struct ExtensionList {
  ///Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub extension:
    Vec<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension>,
}
/// Defines the AtrbtnTaskAssignUnassignUser Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1912:atrbtn.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p1912:CT_TaskAssignUnassignUser/p1912:atrbtn")]
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
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1912:asgn.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p1912:CT_TaskAssignUnassignUser/p1912:asgn")]
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
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1912:unAsgn.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p1912:CT_TaskAssignUnassignUser/p1912:unAsgn")]
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
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p1912:CT_TaskAssignUnassignUser/")]
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
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1912:anchr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p1912:CT_TaskAnchor/p1912:anchr")]
pub struct TaskAnchor {
  /// _
  #[sdk(child(qname = "p1912:CT_CommentAnchor/p1912:comment"))]
  pub comment_anchor: std::boxed::Box<CommentAnchor>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p1912:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the AddEmpty Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1912:add.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p1912:add")]
pub struct AddEmpty {}
/// Defines the UnasgnAllEmpty Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1912:unasgnAll.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_Empty/p1912:unasgnAll")]
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
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1912:title.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p1912:CT_TaskTitleEventInfo/p1912:title")]
pub struct TaskTitleEventInfo {
  /// val
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the TaskScheduleEventInfo Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1912:date.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p1912:CT_TaskScheduleEventInfo/p1912:date")]
pub struct TaskScheduleEventInfo {
  /// stDt
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :stDt
  #[sdk(attr(qname = ":stDt"))]
  pub st_dt: Option<crate::simple_type::DateTimeValue>,
  /// endDt
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :endDt
  #[sdk(attr(qname = ":endDt"))]
  pub end_dt: Option<crate::simple_type::DateTimeValue>,
}
/// Defines the TaskProgressEventInfo Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1912:pcntCmplt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p1912:CT_TaskProgressEventInfo/p1912:pcntCmplt")]
pub struct TaskProgressEventInfo {
  /// val
  ///
  /// Available in Office2021 and above.
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
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1912:pri.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p1912:CT_TaskPriorityRecord/p1912:pri")]
pub struct TaskPriorityRecord {
  /// val
  ///
  /// Available in Office2021 and above.
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
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1912:undo.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p1912:CT_TaskUndo/p1912:undo")]
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
/// Defines the TaskUnknownRecord Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1912:unknown.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p1912:CT_TaskUnknownRecord/p1912:unknown")]
pub struct TaskUnknownRecord {}
/// Defines the TaskHistoryEvent Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1912:event.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p1912:CT_TaskHistoryEvent/p1912:event")]
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
  #[sdk(child(qname = "p1912:CT_TaskAssignUnassignUser/p1912:atrbtn"))]
  pub atrbtn_task_assign_unassign_user: std::boxed::Box<AtrbtnTaskAssignUnassignUser>,
  /// _
  #[sdk(child(qname = "p1912:CT_TaskAnchor/p1912:anchr"))]
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
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p1912:extLst"))]
  pub p1912_ext_lst: Option<ExtensionList>,
}
/// Defines the TaskHistory Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1912:history.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p1912:CT_TaskHistory/p1912:history")]
pub struct TaskHistory {
  /// _
  #[sdk(child(qname = "p1912:CT_TaskHistoryEvent/p1912:event"))]
  pub p1912_event: Vec<TaskHistoryEvent>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TaskHistoryEventChoice {
  #[sdk(child(qname = "p1912:CT_TaskAssignUnassignUser/p1912:asgn"))]
  P1912Asgn(std::boxed::Box<AsgnTaskAssignUnassignUser>),
  #[sdk(child(qname = "p1912:CT_TaskAssignUnassignUser/p1912:unAsgn"))]
  P1912UnAsgn(std::boxed::Box<UnAsgnTaskAssignUnassignUser>),
  #[sdk(child(qname = "p:CT_Empty/p1912:add"))]
  P1912Add(std::boxed::Box<AddEmpty>),
  #[sdk(child(qname = "p1912:CT_TaskTitleEventInfo/p1912:title"))]
  P1912Title(std::boxed::Box<TaskTitleEventInfo>),
  #[sdk(child(qname = "p1912:CT_TaskScheduleEventInfo/p1912:date"))]
  P1912Date(std::boxed::Box<TaskScheduleEventInfo>),
  #[sdk(child(qname = "p1912:CT_TaskProgressEventInfo/p1912:pcntCmplt"))]
  P1912PcntCmplt(std::boxed::Box<TaskProgressEventInfo>),
  #[sdk(child(qname = "p1912:CT_TaskPriorityRecord/p1912:pri"))]
  P1912Pri(std::boxed::Box<TaskPriorityRecord>),
  #[sdk(child(qname = "p:CT_Empty/p1912:unasgnAll"))]
  P1912UnasgnAll(std::boxed::Box<UnasgnAllEmpty>),
  #[sdk(child(qname = "p1912:CT_TaskUndo/p1912:undo"))]
  P1912Undo(std::boxed::Box<TaskUndo>),
  #[sdk(child(qname = "p1912:CT_TaskUnknownRecord/p1912:unknown"))]
  P1912Unknown(std::boxed::Box<TaskUnknownRecord>),
}
