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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskHistoryDetails/p216:taskHistoryDetails")]
pub struct TaskHistoryDetails {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_CommentAnchor/p216:comment")]
pub struct CommentAnchor {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ExtensionList/p216:extLst")]
pub struct ExtensionList {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub extension:
    Vec<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension>,
}
/// Defines the AtrbtnTaskAssignUnassignUser Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p216:atrbtn.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskAssignUnassignUser/p216:atrbtn")]
pub struct AtrbtnTaskAssignUnassignUser {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskAssignUnassignUser/p216:asgn")]
pub struct AsgnTaskAssignUnassignUser {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskAssignUnassignUser/p216:unAsgn")]
pub struct UnAsgnTaskAssignUnassignUser {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskAssignUnassignUser/")]
pub struct OpenXmlTaskAssignUnassignUserElement {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskAnchor/p216:anchr")]
pub struct TaskAnchor {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "p216:CT_CommentAnchor/p216:comment"))]
  pub comment_anchor: std::boxed::Box<CommentAnchor>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p216:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TaskTitleEventInfo Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p216:title.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskTitleEventInfo/p216:title")]
pub struct TaskTitleEventInfo {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskScheduleEventInfo/p216:date")]
pub struct TaskScheduleEventInfo {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskProgressEventInfo/p216:pcntCmplt")]
pub struct TaskProgressEventInfo {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskPriorityRecord/p216:pri")]
pub struct TaskPriorityRecord {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskUndo/p216:undo")]
pub struct TaskUndo {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
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
/// Defines the TaskHistoryEvent Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is p216:event.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskHistoryEvent/p216:event")]
pub struct TaskHistoryEvent {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
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
  #[sdk(choice(
    qname = "p216:CT_TaskAssignUnassignUser/p216:asgn",
    qname = "p216:CT_TaskAssignUnassignUser/p216:unAsgn",
    qname = "p:CT_Empty/p216:add",
    qname = "p216:CT_TaskTitleEventInfo/p216:title",
    qname = "p216:CT_TaskScheduleEventInfo/p216:date",
    qname = "p216:CT_TaskProgressEventInfo/p216:pcntCmplt",
    qname = "p216:CT_TaskPriorityRecord/p216:pri",
    qname = "p:CT_Empty/p216:unasgnAll",
    qname = "p216:CT_TaskUndo/p216:undo",
    qname = "p216:CT_TaskUnknownRecord/p216:unknown"
  ))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p216:CT_TaskHistory/p216:history")]
pub struct TaskHistory {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "p216:CT_TaskHistoryEvent/p216:event"))]
  pub p216_event: Vec<TaskHistoryEvent>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TaskHistoryEventChoice {
  #[sdk(child(qname = "p216:CT_TaskAssignUnassignUser/p216:asgn"))]
  P216Asgn(std::boxed::Box<AsgnTaskAssignUnassignUser>),
  #[sdk(child(qname = "p216:CT_TaskAssignUnassignUser/p216:unAsgn"))]
  P216UnAsgn(std::boxed::Box<UnAsgnTaskAssignUnassignUser>),
  /// Defines the AddEmpty Class.
  #[sdk(empty_child(qname = "p:CT_Empty/p216:add"))]
  P216Add,
  #[sdk(child(qname = "p216:CT_TaskTitleEventInfo/p216:title"))]
  P216Title(std::boxed::Box<TaskTitleEventInfo>),
  #[sdk(child(qname = "p216:CT_TaskScheduleEventInfo/p216:date"))]
  P216Date(std::boxed::Box<TaskScheduleEventInfo>),
  #[sdk(child(qname = "p216:CT_TaskProgressEventInfo/p216:pcntCmplt"))]
  P216PcntCmplt(std::boxed::Box<TaskProgressEventInfo>),
  #[sdk(child(qname = "p216:CT_TaskPriorityRecord/p216:pri"))]
  P216Pri(std::boxed::Box<TaskPriorityRecord>),
  /// Defines the UnasgnAllEmpty Class.
  #[sdk(empty_child(qname = "p:CT_Empty/p216:unasgnAll"))]
  P216UnasgnAll,
  #[sdk(child(qname = "p216:CT_TaskUndo/p216:undo"))]
  P216Undo(std::boxed::Box<TaskUndo>),
  /// Defines the TaskUnknownRecord Class.
  #[sdk(empty_child(qname = "p216:CT_TaskUnknownRecord/p216:unknown"))]
  P216Unknown,
}
