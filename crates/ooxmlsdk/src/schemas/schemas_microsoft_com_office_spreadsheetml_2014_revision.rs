//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RevisionContext {
  #[sdk(rename = "normal")]
  #[default]
  Normal,
  #[sdk(rename = "undo")]
  Undo,
  #[sdk(rename = "redo")]
  Redo,
  #[sdk(rename = "copy")]
  Copy,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RwColAction {
  #[sdk(rename = "insr")]
  #[default]
  Insr,
  #[sdk(rename = "delr")]
  Delr,
  #[sdk(rename = "insc")]
  Insc,
  #[sdk(rename = "delc")]
  Delc,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FeatureType {
  #[sdk(rename = "dataValidation")]
  #[default]
  DataValidation,
  #[sdk(rename = "hyperlink")]
  Hyperlink,
  #[sdk(rename = "rowColVisualOps")]
  RowColVisualOps,
  #[sdk(rename = "freezePanes")]
  FreezePanes,
  #[sdk(rename = "sparklines")]
  Sparklines,
  #[sdk(rename = "hideUnhideSheet")]
  HideUnhideSheet,
  #[sdk(rename = "showGridlinesHeadings")]
  ShowGridlinesHeadings,
  #[sdk(rename = "comment")]
  Comment,
  #[sdk(rename = "outlines")]
  Outlines,
  #[sdk(rename = "drawingElement")]
  DrawingElement,
  #[sdk(rename = "autoFilter")]
  AutoFilter,
  #[sdk(rename = "pivotTable")]
  PivotTable,
  #[sdk(rename = "future")]
  Future,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ExtFeatureType {
  #[sdk(rename = "reserved")]
  #[default]
  Reserved,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SubFeatureType {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "future")]
  Future,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ExtSubFeatureType {
  #[sdk(rename = "reserved")]
  #[default]
  Reserved,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RowColVisualOp {
  #[sdk(rename = "hide")]
  #[default]
  Hide,
  #[sdk(rename = "unhide")]
  Unhide,
  #[sdk(rename = "resize")]
  Resize,
  #[sdk(rename = "autosize")]
  Autosize,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SheetOp {
  #[sdk(rename = "insert")]
  #[default]
  Insert,
  #[sdk(rename = "delete")]
  Delete,
  #[sdk(rename = "reorder")]
  Reorder,
  #[sdk(rename = "rename")]
  Rename,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FillType {
  #[sdk(rename = "fill")]
  #[default]
  Fill,
  #[sdk(rename = "array")]
  Array,
  #[sdk(rename = "future")]
  Future,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FillTypeExt {
  #[sdk(rename = "test")]
  #[default]
  Test,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AdjustType {
  #[sdk(rename = "fmla")]
  #[default]
  Fmla,
  #[sdk(rename = "format")]
  Format,
  #[sdk(rename = "condFmt")]
  CondFmt,
  #[sdk(rename = "sparkline")]
  Sparkline,
  #[sdk(rename = "anchor")]
  Anchor,
  #[sdk(rename = "fmlaNoSticky")]
  FmlaNoSticky,
  #[sdk(rename = "noAdj")]
  NoAdj,
  #[sdk(rename = "fragile")]
  Fragile,
  #[sdk(rename = "future")]
  Future,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AdjustTypeExt {
  #[sdk(rename = "test")]
  #[default]
  Test,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum OartAnchorType {
  #[sdk(rename = "twoCell")]
  #[default]
  TwoCell,
  #[sdk(rename = "oneCell")]
  OneCell,
  #[sdk(rename = "absolute")]
  Absolute,
}
/// Defines the RevExHeaders Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:revHdrs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevExHeaders/xr:revHdrs")]
pub struct RevExHeaders {
  /// minRev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :minRev
  #[sdk(attr(qname = ":minRev"))]
  pub min_rev: crate::simple_type::UInt64Value,
  /// maxRev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :maxRev
  #[sdk(attr(qname = ":maxRev"))]
  pub max_rev: crate::simple_type::UInt64Value,
  /// docId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :docId
  #[sdk(attr(qname = ":docId"))]
  pub doc_id: crate::simple_type::StringValue,
  /// endpointId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :endpointId
  #[sdk(attr(qname = ":endpointId"))]
  pub endpoint_id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "xr:CT_RevExHeader/xr:hdr"))]
  pub xr_hdr: Vec<RevExHeader>,
}
/// Defines the RevExStream Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:revStream.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevExStream/xr:revStream")]
pub struct RevExStream {
  #[sdk(choice(
    qname = "xr:CT_RevExFuture/xr:xrrftr",
    qname = "xr:CT_RevExUnsupported/xr:xrrUspt",
    qname = "xr:CT_RevExTrimmed/xr:xrrTrim",
    qname = "xr:CT_RevExRowColumn/xr:xrrrc",
    qname = "xr:CT_RevExMove/xr:xrrm",
    qname = "xr:CT_RevExChangeCell/xr:xrrc",
    qname = "xr:CT_RevExFormatting/xr:xrrf",
    qname = "xr:CT_RevExDefinedName/xr:xrrDefName",
    qname = "xr:CT_RevExDelObj/xr:xrrdo",
    qname = "xr:CT_RevExChgObj/xr:xrrco",
    qname = "xr:CT_RevExSheetOp/xr:xrrSheet",
    qname = "xr:CT_RevisionList/xr:xrrList",
    qname = "xr:CT_RevListAutoExpandRw/xr:xrrListExpR",
    qname = "xr:CT_RevGroup/xr:xrrg"
  ))]
  pub xml_children: Vec<RevExStreamChoice>,
}
/// Defines the DifferentialFormatType Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:dxf.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dxf/xr:dxf")]
pub struct DifferentialFormatType {
  /// Font Properties
  #[sdk(child(qname = "x:CT_Font/x:font"))]
  pub font: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Font>,
  >,
  /// Number Format
  #[sdk(child(qname = "x:CT_NumFmt/x:numFmt"))]
  pub numbering_format:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::NumberingFormat>,
  /// Fill
  #[sdk(child(qname = "x:CT_Fill/x:fill"))]
  pub fill: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Fill>,
  >,
  /// Alignment
  #[sdk(child(qname = "x:CT_CellAlignment/x:alignment"))]
  pub alignment:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Alignment>,
  /// Border Properties
  #[sdk(child(qname = "x:CT_Border/x:border"))]
  pub border: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Border>,
  >,
  /// Protection Properties
  #[sdk(child(qname = "x:CT_CellProtection/x:protection"))]
  pub protection:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Protection>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList>,
}
/// Defines the RevisionPtr Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:revisionPtr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevisionPtr/xr:revisionPtr")]
pub struct RevisionPtr {
  /// revIDLastSave
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :revIDLastSave
  #[sdk(attr(qname = ":revIDLastSave"))]
  pub rev_id_last_save: crate::simple_type::UInt64Value,
  /// documentId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :documentId
  #[sdk(attr(qname = ":documentId"))]
  pub document_id: crate::simple_type::StringValue,
  /// Last coauthoring version.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xr6:coauthVersionLast
  #[sdk(attr(qname = "xr6:coauthVersionLast"))]
  pub xr6_coauth_version_last: Option<crate::simple_type::UInt64Value>,
  /// Maximum coauthoring version.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xr6:coauthVersionMax
  #[sdk(attr(qname = "xr6:coauthVersionMax"))]
  pub xr6_coauth_version_max: Option<crate::simple_type::UInt64Value>,
  /// Last save UID.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xr10:uidLastSave
  #[sdk(attr(qname = "xr10:uidLastSave"))]
  pub xr10_uid_last_save: Option<crate::simple_type::StringValue>,
}
/// Defines the StateBasedObject Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:objectState.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_StateBasedObject/xr:objectState")]
pub struct StateBasedObject {
  #[sdk(choice(
    qname = "x:CT_DataValidation/xr:dataValidation",
    qname = "x:CT_Hyperlink/xr:hyperlink",
    qname = "x14:CT_SparklineGroup/xr:sparklineGroup",
    qname = "x:CT_Comments/xr:comments",
    qname = "x:CT_AutoFilter/xr:autoFilter",
    qname = "x:CT_pivotTableDefinition/xr:pivotTableDefinition"
  ))]
  pub xml_children: Option<StateBasedObjectChoice>,
}
/// Defines the RevExHeader Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:hdr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevExHeader/xr:hdr")]
pub struct RevExHeader {
  /// id
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
  /// minRev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :minRev
  #[sdk(attr(qname = ":minRev"))]
  pub min_rev: Option<crate::simple_type::UInt64Value>,
  /// maxRev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :maxRev
  #[sdk(attr(qname = ":maxRev"))]
  pub max_rev: Option<crate::simple_type::UInt64Value>,
  /// time
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::DateTimeValue,
}
/// Defines the RevExFuture Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:xrrftr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevExFuture/xr:xrrftr")]
pub struct RevExFuture {
  /// rev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// sti
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sti
  #[sdk(attr(qname = ":sti"))]
  pub sti: Option<crate::simple_type::BooleanValue>,
  /// Defines the RevExTest Class.
  #[sdk(empty_child(qname = "xr:CT_RevExTest/xr:xrrtest"))]
  pub rev_ex_test: Option<()>,
}
/// Defines the RevExUnsupported Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:xrrUspt.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevExUnsupported/xr:xrrUspt")]
pub struct RevExUnsupported {
  /// rev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
}
/// Defines the RevExTrimmed Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:xrrTrim.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevExTrimmed/xr:xrrTrim")]
pub struct RevExTrimmed {
  /// rev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
}
/// Defines the RevExRowColumn Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:xrrrc.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevExRowColumn/xr:xrrrc")]
pub struct RevExRowColumn {
  /// rev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// eol
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :eol
  #[sdk(attr(qname = ":eol"))]
  pub eol: Option<crate::simple_type::BooleanValue>,
  /// ref
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub r#ref: crate::simple_type::StringValue,
  /// action
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :action
  #[sdk(attr(qname = ":action"))]
  pub action: RwColAction,
}
/// Defines the RevExMove Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:xrrm.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevExMove/xr:xrrm")]
pub struct RevExMove {
  /// rev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// src
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :src
  #[sdk(attr(qname = ":src"))]
  pub src: crate::simple_type::StringValue,
  /// dst
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :dst
  #[sdk(attr(qname = ":dst"))]
  pub dst: crate::simple_type::StringValue,
  /// srcSh
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :srcSh
  #[sdk(attr(qname = ":srcSh"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub src_sh: Option<crate::simple_type::StringValue>,
}
/// Defines the RevExChangeCell Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:xrrc.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevExChangeCell/xr:xrrc")]
pub struct RevExChangeCell {
  /// listUid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :listUid
  #[sdk(attr(qname = ":listUid"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub list_uid: Option<crate::simple_type::StringValue>,
  /// rev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// r
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub r: crate::simple_type::StringValue,
  /// t
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub t: Option<FillType>,
  /// x
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  pub x: Option<FillTypeExt>,
  /// w
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :w
  #[sdk(attr(qname = ":w"))]
  pub w: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "xr:CT_RevCell/xr:c"))]
  pub xr_c: Vec<RevCell>,
  /// _
  #[sdk(child(qname = "xr:CT_ChangeCellSubEdit/xr:ccse"))]
  pub xr_ccse: Vec<ChangeCellSubEdit>,
}
/// Defines the RevExFormatting Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:xrrf.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevExFormatting/xr:xrrf")]
pub struct RevExFormatting {
  /// rev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// numFmtId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :numFmtId
  #[sdk(attr(qname = ":numFmtId"))]
  pub num_fmt_id: Option<crate::simple_type::UInt32Value>,
  /// xfDxf
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :xfDxf
  #[sdk(attr(qname = ":xfDxf"))]
  pub xf_dxf: Option<crate::simple_type::BooleanValue>,
  /// style
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::BooleanValue>,
  /// sqref
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sqref
  #[sdk(attr(qname = ":sqref"))]
  pub sqref: crate::simple_type::ListValue<crate::simple_type::StringValue>,
  /// start
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :start
  #[sdk(attr(qname = ":start"))]
  pub start: Option<crate::simple_type::UInt32Value>,
  /// length
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :length
  #[sdk(attr(qname = ":length"))]
  pub length: Option<crate::simple_type::UInt32Value>,
  /// styleUid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :styleUid
  #[sdk(attr(qname = ":styleUid"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub style_uid: Option<crate::simple_type::StringValue>,
  /// fBlankCell
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :fBlankCell
  #[sdk(attr(qname = ":fBlankCell"))]
  pub f_blank_cell: Option<crate::simple_type::BooleanValue>,
  /// applyNumberFormat
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :applyNumberFormat
  #[sdk(attr(qname = ":applyNumberFormat"))]
  pub apply_number_format: Option<crate::simple_type::BooleanValue>,
  /// applyFont
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :applyFont
  #[sdk(attr(qname = ":applyFont"))]
  pub apply_font: Option<crate::simple_type::BooleanValue>,
  /// applyFill
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :applyFill
  #[sdk(attr(qname = ":applyFill"))]
  pub apply_fill: Option<crate::simple_type::BooleanValue>,
  /// applyBorder
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :applyBorder
  #[sdk(attr(qname = ":applyBorder"))]
  pub apply_border: Option<crate::simple_type::BooleanValue>,
  /// applyAlignment
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :applyAlignment
  #[sdk(attr(qname = ":applyAlignment"))]
  pub apply_alignment: Option<crate::simple_type::BooleanValue>,
  /// applyProtection
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :applyProtection
  #[sdk(attr(qname = ":applyProtection"))]
  pub apply_protection: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_Dxf/xr:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xr:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the RevExDefinedName Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:xrrDefName.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevExDefinedName/xr:xrrDefName")]
pub struct RevExDefinedName {
  /// rev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// customView
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :customView
  #[sdk(attr(qname = ":customView"))]
  pub custom_view: Option<crate::simple_type::BooleanValue>,
  /// name
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// function
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :function
  #[sdk(attr(qname = ":function"))]
  pub function: Option<crate::simple_type::BooleanValue>,
  /// functionGroupId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :functionGroupId
  #[sdk(attr(qname = ":functionGroupId"))]
  pub function_group_id: Option<crate::simple_type::ByteValue>,
  /// shortcutKey
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :shortcutKey
  #[sdk(attr(qname = ":shortcutKey"))]
  pub shortcut_key: Option<crate::simple_type::ByteValue>,
  /// hidden
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// customMenu
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :customMenu
  #[sdk(attr(qname = ":customMenu"))]
  pub custom_menu: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// help
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :help
  #[sdk(attr(qname = ":help"))]
  pub help: Option<crate::simple_type::StringValue>,
  /// statusBar
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :statusBar
  #[sdk(attr(qname = ":statusBar"))]
  pub status_bar: Option<crate::simple_type::StringValue>,
  /// comment
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :comment
  #[sdk(attr(qname = ":comment"))]
  pub comment: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "x:ST_Formula/xr:formula"))]
  pub formula_formula: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xr:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the RevExDelObj Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:xrrdo.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevExDelObj/xr:xrrdo")]
pub struct RevExDelObj {
  /// rev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// _
  #[sdk(child(qname = "xr:CT_StateBasedHeader/xr:hdr"))]
  pub state_based_header: std::boxed::Box<StateBasedHeader>,
}
/// Defines the RevExChgObj Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:xrrco.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevExChgObj/xr:xrrco")]
pub struct RevExChgObj {
  /// rev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// _
  #[sdk(child(qname = "xr:CT_StateBasedHeader/xr:hdr"))]
  pub state_based_header: std::boxed::Box<StateBasedHeader>,
  #[sdk(choice(
    qname = "xr:CT_RevisionStateLink/xr:link",
    qname = "xr:CT_RevisionState/xr:body"
  ))]
  pub rev_ex_chg_obj_choice: Option<RevExChgObjChoice>,
}
/// Defines the RevExSheetOp Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:xrrSheet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevExSheetOp/xr:xrrSheet")]
pub struct RevExSheetOp {
  /// rev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// op
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :op
  #[sdk(attr(qname = ":op"))]
  pub op: SheetOp,
  /// name
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// idOrig
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :idOrig
  #[sdk(attr(qname = ":idOrig"))]
  pub id_orig: Option<crate::simple_type::UInt32Value>,
  /// idNew
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :idNew
  #[sdk(attr(qname = ":idNew"))]
  pub id_new: Option<crate::simple_type::UInt32Value>,
}
/// Defines the RevisionList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:xrrList.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevisionList/xr:xrrList")]
pub struct RevisionList {
  /// rev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// Data
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :Data
  #[sdk(attr(qname = ":Data"))]
  pub data: Option<crate::simple_type::BooleanValue>,
  /// Formatting
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :Formatting
  #[sdk(attr(qname = ":Formatting"))]
  pub formatting: Option<crate::simple_type::BooleanValue>,
  /// RangeBased
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :RangeBased
  #[sdk(attr(qname = ":RangeBased"))]
  pub range_based: Option<crate::simple_type::BooleanValue>,
  /// Fake
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :Fake
  #[sdk(attr(qname = ":Fake"))]
  pub fake: Option<crate::simple_type::BooleanValue>,
  /// ref
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub r#ref: crate::simple_type::StringValue,
  /// Headers
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :Headers
  #[sdk(attr(qname = ":Headers"))]
  pub headers: Option<crate::simple_type::BooleanValue>,
  /// InsDelHeaders
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :InsDelHeaders
  #[sdk(attr(qname = ":InsDelHeaders"))]
  pub ins_del_headers: Option<crate::simple_type::BooleanValue>,
  /// rId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :rId
  #[sdk(attr(qname = ":rId"))]
  pub r_id: crate::simple_type::UInt32Value,
}
/// Defines the RevListAutoExpandRw Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:xrrListExpR.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevListAutoExpandRw/xr:xrrListExpR")]
pub struct RevListAutoExpandRw {
  /// rev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// refAdded
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :refAdded
  #[sdk(attr(qname = ":refAdded"))]
  pub ref_added: crate::simple_type::StringValue,
  /// listGuid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :listGuid
  #[sdk(attr(qname = ":listGuid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub list_guid: crate::simple_type::StringValue,
}
/// Defines the RevGroup Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:xrrg.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevGroup/xr:xrrg")]
pub struct RevGroup {
  /// rev
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  #[sdk(choice(
    qname = "xr:CT_RevExFuture/xr:xrrftr",
    qname = "xr:CT_RevExUnsupported/xr:xrrUspt",
    qname = "xr:CT_RevExTrimmed/xr:xrrTrim",
    qname = "xr:CT_RevExRowColumn/xr:xrrrc",
    qname = "xr:CT_RevExMove/xr:xrrm",
    qname = "xr:CT_RevExChangeCell/xr:xrrc",
    qname = "xr:CT_RevExFormatting/xr:xrrf",
    qname = "xr:CT_RevExDefinedName/xr:xrrDefName",
    qname = "xr:CT_RevExDelObj/xr:xrrdo",
    qname = "xr:CT_RevExChgObj/xr:xrrco",
    qname = "xr:CT_RevExSheetOp/xr:xrrSheet",
    qname = "xr:CT_RevisionList/xr:xrrList",
    qname = "xr:CT_RevListAutoExpandRw/xr:xrrListExpR"
  ))]
  pub xml_children: Vec<RevGroupChoice>,
}
/// Defines the RevCell Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:c.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevCell/xr:c")]
pub struct RevCell {
  /// t
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub t: Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CellValues>,
  /// nop
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :nop
  #[sdk(attr(qname = ":nop"))]
  pub nop: Option<crate::simple_type::BooleanValue>,
  /// tick
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :tick
  #[sdk(attr(qname = ":tick"))]
  pub tick: Option<crate::simple_type::BooleanValue>,
  /// rep
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :rep
  #[sdk(attr(qname = ":rep"))]
  pub rep: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(text_child(qname = "x:ST_Formula/xr:f"))]
  pub f_formula: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "x:ST_Xstring/xr:v"))]
  pub xstring: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_Rst/xr:is"))]
  pub rst_type: Option<std::boxed::Box<RstType>>,
}
/// Defines the ChangeCellSubEdit Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:ccse.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_ChangeCellSubEdit/xr:ccse")]
pub struct ChangeCellSubEdit {
  /// r
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub r: crate::simple_type::StringValue,
  /// t
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub t: Option<FillType>,
  /// x
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  pub x: Option<FillTypeExt>,
  /// w
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :w
  #[sdk(attr(qname = ":w"))]
  pub w: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "xr:CT_RevCell/xr:c"))]
  pub xr_c: Vec<RevCell>,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExtensionList/xr:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
/// Defines the FormulaFormula Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:formula.
pub type FormulaFormula = crate::simple_type::StringValue;
/// Defines the FFormula Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:f.
pub type FFormula = crate::simple_type::StringValue;
/// Defines the StateBasedHeader Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:hdr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_StateBasedHeader/xr:hdr")]
pub struct StateBasedHeader {
  /// uid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// eft
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :eft
  #[sdk(attr(qname = ":eft"))]
  pub eft: FeatureType,
  /// eftx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :eftx
  #[sdk(attr(qname = ":eftx"))]
  pub eftx: Option<ExtFeatureType>,
  /// seft
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :seft
  #[sdk(attr(qname = ":seft"))]
  pub seft: Option<SubFeatureType>,
  /// seftx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :seftx
  #[sdk(attr(qname = ":seftx"))]
  pub seftx: Option<ExtSubFeatureType>,
  /// _
  #[sdk(child(qname = "xr:CT_RefMap/xr:refmap"))]
  pub ref_map: Option<RefMap>,
}
/// Defines the RevisionStateLink Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:link.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevisionStateLink/xr:link")]
pub struct RevisionStateLink {
  /// id
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the RevisionState Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:body.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RevisionState/xr:body")]
pub struct RevisionState {
  #[sdk(choice(
    qname = "xr:CT_RowColVisualOps/xr:rowColVisualOps",
    qname = "xr:CT_HideUnhideSheet/xr:hideUnhideSheet",
    qname = "xr:CT_ShowGridlinesHeadings/xr:showGridlinesHeadings",
    qname = "xr:CT_FreezePanes/xr:freezePanes",
    qname = "xr:CT_Outlines/xr:outlines"
  ))]
  pub xml_children: Option<RevisionStateChoice>,
}
/// Defines the RefMap Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:refmap.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RefMap/xr:refmap")]
pub struct RefMap {
  #[sdk(choice(
    qname = "xr:CT_RefCell/xr:ref",
    qname = "xr:CT_SheetXluid/xr:sheetUid",
    qname = "xr:CT_RefOartAnchor/xr:oartAnchor",
    qname = "xr:CT_RefFuture/xr:future",
    qname = "xr:CT_RefTest/xr:test"
  ))]
  pub xml_children: Vec<RefMapChoice>,
}
/// Defines the RowColVisualOps Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:rowColVisualOps.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RowColVisualOps/xr:rowColVisualOps")]
pub struct RowColVisualOps {
  /// action
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :action
  #[sdk(attr(qname = ":action"))]
  pub action: RowColVisualOp,
  /// isRow
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :isRow
  #[sdk(attr(qname = ":isRow"))]
  pub is_row: crate::simple_type::BooleanValue,
  /// size
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// userSized
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :userSized
  #[sdk(attr(qname = ":userSized"))]
  pub user_sized: Option<crate::simple_type::BooleanValue>,
}
/// Defines the HideUnhideSheet Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:hideUnhideSheet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_HideUnhideSheet/xr:hideUnhideSheet")]
pub struct HideUnhideSheet {
  /// hide
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :hide
  #[sdk(attr(qname = ":hide"))]
  pub hide: crate::simple_type::BooleanValue,
}
/// Defines the ShowGridlinesHeadings Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:showGridlinesHeadings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_ShowGridlinesHeadings/xr:showGridlinesHeadings")]
pub struct ShowGridlinesHeadings {
  /// showGridLines
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :showGridLines
  #[sdk(attr(qname = ":showGridLines"))]
  pub show_grid_lines: crate::simple_type::BooleanValue,
  /// showRowCol
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :showRowCol
  #[sdk(attr(qname = ":showRowCol"))]
  pub show_row_col: crate::simple_type::BooleanValue,
}
/// Defines the FreezePanes Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:freezePanes.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_FreezePanes/xr:freezePanes")]
pub struct FreezePanes {
  /// sheetViewUid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sheetViewUid
  #[sdk(attr(qname = ":sheetViewUid"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub sheet_view_uid: Option<crate::simple_type::StringValue>,
}
/// Defines the Outlines Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:outlines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_Outlines/xr:outlines")]
pub struct Outlines {
  /// isRow
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :isRow
  #[sdk(attr(qname = ":isRow"))]
  pub is_row: crate::simple_type::BooleanValue,
  /// _
  #[sdk(child(qname = "xr:CT_Outline/xr:outline"))]
  pub xr_outline: Vec<Outline>,
}
/// Defines the Outline Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:outline.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_Outline/xr:outline")]
pub struct Outline {
  /// isCollapsed
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :isCollapsed
  #[sdk(attr(qname = ":isCollapsed"))]
  pub is_collapsed: crate::simple_type::BooleanValue,
  /// level
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :level
  #[sdk(attr(qname = ":level"))]
  pub level: crate::simple_type::ByteValue,
}
/// Defines the Xstring Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:v.
pub type Xstring = crate::simple_type::StringValue;
/// Defines the RstType Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:is.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Rst/xr:is")]
pub struct RstType {
  /// Text
  #[sdk(child(qname = "x:CT_Xstring/x:t"))]
  pub text: Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Text>,
  /// _
  #[sdk(child(qname = "x:CT_RElt/x:r"))]
  pub x_r: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Run>,
  /// _
  #[sdk(child(qname = "x:CT_PhoneticRun/x:rPh"))]
  pub x_r_ph: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PhoneticRun>,
  /// _
  #[sdk(child(qname = "x:CT_PhoneticPr/x:phoneticPr"))]
  pub x_phonetic_pr:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PhoneticProperties>,
}
/// Defines the RefCell Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:ref.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RefCell/xr:ref")]
pub struct RefCell {
  /// n
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// ajt
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ajt
  #[sdk(attr(qname = ":ajt"))]
  pub ajt: AdjustType,
  /// ajtx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ajtx
  #[sdk(attr(qname = ":ajtx"))]
  pub ajtx: Option<AdjustTypeExt>,
  /// homeRef
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :homeRef
  #[sdk(attr(qname = ":homeRef"))]
  pub home_ref: Option<crate::simple_type::BooleanValue>,
  /// r
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub r: crate::simple_type::ListValue<crate::simple_type::StringValue>,
  /// uid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uid: Option<crate::simple_type::StringValue>,
  /// uidLast
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uidLast
  #[sdk(attr(qname = ":uidLast"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uid_last: Option<crate::simple_type::StringValue>,
}
/// Defines the SheetXluid Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:sheetUid.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_SheetXluid/xr:sheetUid")]
pub struct SheetXluid {
  /// n
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// ajt
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ajt
  #[sdk(attr(qname = ":ajt"))]
  pub ajt: AdjustType,
  /// ajtx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ajtx
  #[sdk(attr(qname = ":ajtx"))]
  pub ajtx: Option<AdjustTypeExt>,
  /// homeRef
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :homeRef
  #[sdk(attr(qname = ":homeRef"))]
  pub home_ref: Option<crate::simple_type::BooleanValue>,
  /// uid
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
}
/// Defines the RefOartAnchor Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:oartAnchor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RefOartAnchor/xr:oartAnchor")]
pub struct RefOartAnchor {
  /// n
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// ajt
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ajt
  #[sdk(attr(qname = ":ajt"))]
  pub ajt: AdjustType,
  /// ajtx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ajtx
  #[sdk(attr(qname = ":ajtx"))]
  pub ajtx: Option<AdjustTypeExt>,
  /// homeRef
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :homeRef
  #[sdk(attr(qname = ":homeRef"))]
  pub home_ref: Option<crate::simple_type::BooleanValue>,
  /// r
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub r: Option<crate::simple_type::StringValue>,
  /// fromRowOff
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :fromRowOff
  #[sdk(attr(qname = ":fromRowOff"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub from_row_off: Option<crate::simple_type::Int64Value>,
  /// fromColOff
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :fromColOff
  #[sdk(attr(qname = ":fromColOff"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub from_col_off: Option<crate::simple_type::Int64Value>,
  /// toRowOff
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :toRowOff
  #[sdk(attr(qname = ":toRowOff"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub to_row_off: Option<crate::simple_type::Int64Value>,
  /// toColOff
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :toColOff
  #[sdk(attr(qname = ":toColOff"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub to_col_off: Option<crate::simple_type::Int64Value>,
  /// cx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :cx
  #[sdk(attr(qname = ":cx"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cx: Option<crate::simple_type::Int64Value>,
  /// cy
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :cy
  #[sdk(attr(qname = ":cy"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cy: Option<crate::simple_type::Int64Value>,
  /// x
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub x: Option<crate::simple_type::Int64Value>,
  /// y
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :y
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub y: Option<crate::simple_type::Int64Value>,
  /// oat
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :oat
  #[sdk(attr(qname = ":oat"))]
  pub oat: OartAnchorType,
}
/// Defines the RefTest Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:test.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:CT_RefTest/xr:test")]
pub struct RefTest {
  /// n
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// ajt
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ajt
  #[sdk(attr(qname = ":ajt"))]
  pub ajt: AdjustType,
  /// ajtx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ajtx
  #[sdk(attr(qname = ":ajtx"))]
  pub ajtx: Option<AdjustTypeExt>,
  /// homeRef
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :homeRef
  #[sdk(attr(qname = ":homeRef"))]
  pub home_ref: Option<crate::simple_type::BooleanValue>,
}
/// Represents an external link to another workbook..
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:dataValidation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DataValidation/xr:dataValidation")]
pub struct DataValidation {
    /// type
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :type
    #[sdk(attr(qname = ":type"))]
    pub r#type: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationValues,
    >,
    /// errorStyle
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :errorStyle
    #[sdk(attr(qname = ":errorStyle"))]
    pub error_style: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationErrorStyleValues,
    >,
    /// imeMode
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :imeMode
    #[sdk(attr(qname = ":imeMode"))]
    pub ime_mode: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationImeModeValues,
    >,
    /// operator
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :operator
    #[sdk(attr(qname = ":operator"))]
    pub operator: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationOperatorValues,
    >,
    /// allowBlank
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :allowBlank
    #[sdk(attr(qname = ":allowBlank"))]
    pub allow_blank: Option<crate::simple_type::BooleanValue>,
    /// showDropDown
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :showDropDown
    #[sdk(attr(qname = ":showDropDown"))]
    pub show_drop_down: Option<crate::simple_type::BooleanValue>,
    /// showInputMessage
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :showInputMessage
    #[sdk(attr(qname = ":showInputMessage"))]
    pub show_input_message: Option<crate::simple_type::BooleanValue>,
    /// showErrorMessage
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :showErrorMessage
    #[sdk(attr(qname = ":showErrorMessage"))]
    pub show_error_message: Option<crate::simple_type::BooleanValue>,
    /// errorTitle
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :errorTitle
    #[sdk(attr(qname = ":errorTitle"))]
    pub error_title: Option<crate::simple_type::StringValue>,
    /// error
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :error
    #[sdk(attr(qname = ":error"))]
    pub error: Option<crate::simple_type::StringValue>,
    /// promptTitle
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :promptTitle
    #[sdk(attr(qname = ":promptTitle"))]
    pub prompt_title: Option<crate::simple_type::StringValue>,
    /// prompt
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :prompt
    #[sdk(attr(qname = ":prompt"))]
    pub prompt: Option<crate::simple_type::StringValue>,
    /// sqref
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :sqref
    #[sdk(attr(qname = ":sqref"))]
    pub sequence_of_references: crate::simple_type::ListValue<
        crate::simple_type::StringValue,
    >,
    /// _
    #[sdk(text_child(qname = "x:ST_Xstring/x12ac:list"))]
    pub list: Option<crate::simple_type::StringValue>,
    /// _
    #[sdk(child(qname = "x:CT_Xstring/x:formula1"))]
    pub formula1: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Formula1,
    >,
    /// _
    #[sdk(child(qname = "x:CT_Xstring/x:formula2"))]
    pub formula2: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Formula2,
    >,
}
/// Represents a hyperlink within a cell..
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:hyperlink.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Hyperlink/xr:hyperlink")]
pub struct Hyperlink {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// Relationship Id
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Location
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :location
  #[sdk(attr(qname = ":location"))]
  pub location: Option<crate::simple_type::StringValue>,
  /// Tool Tip
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tooltip
  #[sdk(attr(qname = ":tooltip"))]
  pub tooltip: Option<crate::simple_type::StringValue>,
  /// Display String
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :display
  #[sdk(attr(qname = ":display"))]
  pub display: Option<crate::simple_type::StringValue>,
}
/// Represents a sparkline group of 1 or more sparklines..
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:sparklineGroup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SparklineGroup/xr:sparklineGroup")]
pub struct SparklineGroup {
    /// manualMax
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :manualMax
    #[sdk(attr(qname = ":manualMax"))]
    pub manual_max: Option<crate::simple_type::DoubleValue>,
    /// manualMin
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :manualMin
    #[sdk(attr(qname = ":manualMin"))]
    pub manual_min: Option<crate::simple_type::DoubleValue>,
    /// lineWeight
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :lineWeight
    #[sdk(attr(qname = ":lineWeight"))]
    pub line_weight: Option<crate::simple_type::DoubleValue>,
    /// type
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :type
    #[sdk(attr(qname = ":type"))]
    pub r#type: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SparklineTypeValues,
    >,
    /// dateAxis
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :dateAxis
    #[sdk(attr(qname = ":dateAxis"))]
    pub date_axis: Option<crate::simple_type::BooleanValue>,
    /// displayEmptyCellsAs
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :displayEmptyCellsAs
    #[sdk(attr(qname = ":displayEmptyCellsAs"))]
    pub display_empty_cells_as: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DisplayBlanksAsValues,
    >,
    /// markers
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :markers
    #[sdk(attr(qname = ":markers"))]
    pub markers: Option<crate::simple_type::BooleanValue>,
    /// high
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :high
    #[sdk(attr(qname = ":high"))]
    pub high: Option<crate::simple_type::BooleanValue>,
    /// low
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :low
    #[sdk(attr(qname = ":low"))]
    pub low: Option<crate::simple_type::BooleanValue>,
    /// first
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :first
    #[sdk(attr(qname = ":first"))]
    pub first: Option<crate::simple_type::BooleanValue>,
    /// last
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :last
    #[sdk(attr(qname = ":last"))]
    pub last: Option<crate::simple_type::BooleanValue>,
    /// negative
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :negative
    #[sdk(attr(qname = ":negative"))]
    pub negative: Option<crate::simple_type::BooleanValue>,
    /// displayXAxis
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :displayXAxis
    #[sdk(attr(qname = ":displayXAxis"))]
    pub display_x_axis: Option<crate::simple_type::BooleanValue>,
    /// displayHidden
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :displayHidden
    #[sdk(attr(qname = ":displayHidden"))]
    pub display_hidden: Option<crate::simple_type::BooleanValue>,
    /// minAxisType
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :minAxisType
    #[sdk(attr(qname = ":minAxisType"))]
    pub min_axis_type: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SparklineAxisMinMaxValues,
    >,
    /// maxAxisType
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :maxAxisType
    #[sdk(attr(qname = ":maxAxisType"))]
    pub max_axis_type: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SparklineAxisMinMaxValues,
    >,
    /// rightToLeft
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :rightToLeft
    #[sdk(attr(qname = ":rightToLeft"))]
    pub right_to_left: Option<crate::simple_type::BooleanValue>,
    /// _
    #[sdk(child(qname = "x:CT_Color/x14:colorSeries"))]
    pub series_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SeriesColor,
    >,
    /// _
    #[sdk(child(qname = "x:CT_Color/x14:colorNegative"))]
    pub negative_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::NegativeColor,
    >,
    /// _
    #[sdk(child(qname = "x:CT_Color/x14:colorAxis"))]
    pub axis_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::AxisColor,
    >,
    /// _
    #[sdk(child(qname = "x:CT_Color/x14:colorMarkers"))]
    pub markers_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::MarkersColor,
    >,
    /// _
    #[sdk(child(qname = "x:CT_Color/x14:colorFirst"))]
    pub first_marker_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::FirstMarkerColor,
    >,
    /// _
    #[sdk(child(qname = "x:CT_Color/x14:colorLast"))]
    pub last_marker_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::LastMarkerColor,
    >,
    /// _
    #[sdk(child(qname = "x:CT_Color/x14:colorHigh"))]
    pub high_marker_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::HighMarkerColor,
    >,
    /// _
    #[sdk(child(qname = "x:CT_Color/x14:colorLow"))]
    pub low_marker_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::LowMarkerColor,
    >,
    /// _
    #[sdk(text_child(qname = "x:ST_Formula/xne:f"))]
    pub formula: Option<crate::simple_type::StringValue>,
    /// _
    #[sdk(child(qname = "x14:CT_Sparklines/x14:sparklines"))]
    pub sparklines: std::boxed::Box<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Sparklines,
    >,
}
/// Represents one comment within a cell..
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:comments.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Comments/xr:comments")]
pub struct Comments {
  /// Authors
  #[sdk(child(qname = "x:CT_Authors/x:authors"))]
  pub authors:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Authors>,
  /// List of Comments
  #[sdk(child(qname = "x:CT_CommentList/x:commentList"))]
  pub comment_list: std::boxed::Box<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CommentList,
  >,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList>,
}
/// Represents an autofilter..
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:autoFilter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_AutoFilter/xr:autoFilter")]
pub struct AutoFilter {
  /// Cell or Range Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_FilterColumn/x:filterColumn"))]
  pub x_filter_column:
    Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::FilterColumn>,
  /// _
  #[sdk(child(qname = "x:CT_SortState/x:sortState"))]
  pub x_sort_state: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SortState>,
  >,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList>,
}
/// Represents a PivotTable View..
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is xr:pivotTableDefinition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_pivotTableDefinition/xr:pivotTableDefinition")]
pub struct PivotTableDefinition {
    /// name
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :name
    #[sdk(attr(qname = ":name"))]
    pub name: crate::simple_type::StringValue,
    /// cacheId
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :cacheId
    #[sdk(attr(qname = ":cacheId"))]
    pub cache_id: crate::simple_type::UInt32Value,
    /// dataOnRows
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :dataOnRows
    #[sdk(attr(qname = ":dataOnRows"))]
    pub data_on_rows: Option<crate::simple_type::BooleanValue>,
    /// dataPosition
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :dataPosition
    #[sdk(attr(qname = ":dataPosition"))]
    pub data_position: Option<crate::simple_type::UInt32Value>,
    /// Auto Format Id
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :autoFormatId
    #[sdk(attr(qname = ":autoFormatId"))]
    pub auto_format_id: Option<crate::simple_type::UInt32Value>,
    /// Apply Number Formats
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :applyNumberFormats
    #[sdk(attr(qname = ":applyNumberFormats"))]
    pub apply_number_formats: Option<crate::simple_type::BooleanValue>,
    /// Apply Border Formats
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :applyBorderFormats
    #[sdk(attr(qname = ":applyBorderFormats"))]
    pub apply_border_formats: Option<crate::simple_type::BooleanValue>,
    /// Apply Font Formats
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :applyFontFormats
    #[sdk(attr(qname = ":applyFontFormats"))]
    pub apply_font_formats: Option<crate::simple_type::BooleanValue>,
    /// Apply Pattern Formats
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :applyPatternFormats
    #[sdk(attr(qname = ":applyPatternFormats"))]
    pub apply_pattern_formats: Option<crate::simple_type::BooleanValue>,
    /// Apply Alignment Formats
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :applyAlignmentFormats
    #[sdk(attr(qname = ":applyAlignmentFormats"))]
    pub apply_alignment_formats: Option<crate::simple_type::BooleanValue>,
    /// Apply Width / Height Formats
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :applyWidthHeightFormats
    #[sdk(attr(qname = ":applyWidthHeightFormats"))]
    pub apply_width_height_formats: Option<crate::simple_type::BooleanValue>,
    /// dataCaption
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :dataCaption
    #[sdk(attr(qname = ":dataCaption"))]
    pub data_caption: crate::simple_type::StringValue,
    /// grandTotalCaption
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :grandTotalCaption
    #[sdk(attr(qname = ":grandTotalCaption"))]
    pub grand_total_caption: Option<crate::simple_type::StringValue>,
    /// errorCaption
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :errorCaption
    #[sdk(attr(qname = ":errorCaption"))]
    pub error_caption: Option<crate::simple_type::StringValue>,
    /// showError
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :showError
    #[sdk(attr(qname = ":showError"))]
    pub show_error: Option<crate::simple_type::BooleanValue>,
    /// missingCaption
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :missingCaption
    #[sdk(attr(qname = ":missingCaption"))]
    pub missing_caption: Option<crate::simple_type::StringValue>,
    /// showMissing
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :showMissing
    #[sdk(attr(qname = ":showMissing"))]
    pub show_missing: Option<crate::simple_type::BooleanValue>,
    /// pageStyle
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :pageStyle
    #[sdk(attr(qname = ":pageStyle"))]
    pub page_style: Option<crate::simple_type::StringValue>,
    /// pivotTableStyle
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :pivotTableStyle
    #[sdk(attr(qname = ":pivotTableStyle"))]
    pub pivot_table_style_name: Option<crate::simple_type::StringValue>,
    /// vacatedStyle
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :vacatedStyle
    #[sdk(attr(qname = ":vacatedStyle"))]
    pub vacated_style: Option<crate::simple_type::StringValue>,
    /// tag
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :tag
    #[sdk(attr(qname = ":tag"))]
    pub tag: Option<crate::simple_type::StringValue>,
    /// updatedVersion
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :updatedVersion
    #[sdk(attr(qname = ":updatedVersion"))]
    pub updated_version: Option<crate::simple_type::ByteValue>,
    /// minRefreshableVersion
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :minRefreshableVersion
    #[sdk(attr(qname = ":minRefreshableVersion"))]
    pub min_refreshable_version: Option<crate::simple_type::ByteValue>,
    /// asteriskTotals
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :asteriskTotals
    #[sdk(attr(qname = ":asteriskTotals"))]
    pub asterisk_totals: Option<crate::simple_type::BooleanValue>,
    /// showItems
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :showItems
    #[sdk(attr(qname = ":showItems"))]
    pub show_items: Option<crate::simple_type::BooleanValue>,
    /// editData
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :editData
    #[sdk(attr(qname = ":editData"))]
    pub edit_data: Option<crate::simple_type::BooleanValue>,
    /// disableFieldList
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :disableFieldList
    #[sdk(attr(qname = ":disableFieldList"))]
    pub disable_field_list: Option<crate::simple_type::BooleanValue>,
    /// showCalcMbrs
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :showCalcMbrs
    #[sdk(attr(qname = ":showCalcMbrs"))]
    pub show_calculated_members: Option<crate::simple_type::BooleanValue>,
    /// visualTotals
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :visualTotals
    #[sdk(attr(qname = ":visualTotals"))]
    pub visual_totals: Option<crate::simple_type::BooleanValue>,
    /// showMultipleLabel
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :showMultipleLabel
    #[sdk(attr(qname = ":showMultipleLabel"))]
    pub show_multiple_label: Option<crate::simple_type::BooleanValue>,
    /// showDataDropDown
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :showDataDropDown
    #[sdk(attr(qname = ":showDataDropDown"))]
    pub show_data_drop_down: Option<crate::simple_type::BooleanValue>,
    /// showDrill
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :showDrill
    #[sdk(attr(qname = ":showDrill"))]
    pub show_drill: Option<crate::simple_type::BooleanValue>,
    /// printDrill
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :printDrill
    #[sdk(attr(qname = ":printDrill"))]
    pub print_drill: Option<crate::simple_type::BooleanValue>,
    /// showMemberPropertyTips
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :showMemberPropertyTips
    #[sdk(attr(qname = ":showMemberPropertyTips"))]
    pub show_member_property_tips: Option<crate::simple_type::BooleanValue>,
    /// showDataTips
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :showDataTips
    #[sdk(attr(qname = ":showDataTips"))]
    pub show_data_tips: Option<crate::simple_type::BooleanValue>,
    /// enableWizard
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :enableWizard
    #[sdk(attr(qname = ":enableWizard"))]
    pub enable_wizard: Option<crate::simple_type::BooleanValue>,
    /// enableDrill
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :enableDrill
    #[sdk(attr(qname = ":enableDrill"))]
    pub enable_drill: Option<crate::simple_type::BooleanValue>,
    /// enableFieldProperties
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :enableFieldProperties
    #[sdk(attr(qname = ":enableFieldProperties"))]
    pub enable_field_properties: Option<crate::simple_type::BooleanValue>,
    /// preserveFormatting
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :preserveFormatting
    #[sdk(attr(qname = ":preserveFormatting"))]
    pub preserve_formatting: Option<crate::simple_type::BooleanValue>,
    /// useAutoFormatting
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :useAutoFormatting
    #[sdk(attr(qname = ":useAutoFormatting"))]
    pub use_auto_formatting: Option<crate::simple_type::BooleanValue>,
    /// pageWrap
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :pageWrap
    #[sdk(attr(qname = ":pageWrap"))]
    pub page_wrap: Option<crate::simple_type::UInt32Value>,
    /// pageOverThenDown
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :pageOverThenDown
    #[sdk(attr(qname = ":pageOverThenDown"))]
    pub page_over_then_down: Option<crate::simple_type::BooleanValue>,
    /// subtotalHiddenItems
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :subtotalHiddenItems
    #[sdk(attr(qname = ":subtotalHiddenItems"))]
    pub subtotal_hidden_items: Option<crate::simple_type::BooleanValue>,
    /// rowGrandTotals
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :rowGrandTotals
    #[sdk(attr(qname = ":rowGrandTotals"))]
    pub row_grand_totals: Option<crate::simple_type::BooleanValue>,
    /// colGrandTotals
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :colGrandTotals
    #[sdk(attr(qname = ":colGrandTotals"))]
    pub column_grand_totals: Option<crate::simple_type::BooleanValue>,
    /// fieldPrintTitles
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :fieldPrintTitles
    #[sdk(attr(qname = ":fieldPrintTitles"))]
    pub field_print_titles: Option<crate::simple_type::BooleanValue>,
    /// itemPrintTitles
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :itemPrintTitles
    #[sdk(attr(qname = ":itemPrintTitles"))]
    pub item_print_titles: Option<crate::simple_type::BooleanValue>,
    /// mergeItem
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :mergeItem
    #[sdk(attr(qname = ":mergeItem"))]
    pub merge_item: Option<crate::simple_type::BooleanValue>,
    /// showDropZones
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :showDropZones
    #[sdk(attr(qname = ":showDropZones"))]
    pub show_drop_zones: Option<crate::simple_type::BooleanValue>,
    /// createdVersion
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :createdVersion
    #[sdk(attr(qname = ":createdVersion"))]
    pub created_version: Option<crate::simple_type::ByteValue>,
    /// indent
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :indent
    #[sdk(attr(qname = ":indent"))]
    pub indent: Option<crate::simple_type::UInt32Value>,
    /// showEmptyRow
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :showEmptyRow
    #[sdk(attr(qname = ":showEmptyRow"))]
    pub show_empty_row: Option<crate::simple_type::BooleanValue>,
    /// showEmptyCol
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :showEmptyCol
    #[sdk(attr(qname = ":showEmptyCol"))]
    pub show_empty_column: Option<crate::simple_type::BooleanValue>,
    /// showHeaders
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :showHeaders
    #[sdk(attr(qname = ":showHeaders"))]
    pub show_headers: Option<crate::simple_type::BooleanValue>,
    /// compact
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :compact
    #[sdk(attr(qname = ":compact"))]
    pub compact: Option<crate::simple_type::BooleanValue>,
    /// outline
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :outline
    #[sdk(attr(qname = ":outline"))]
    pub outline: Option<crate::simple_type::BooleanValue>,
    /// outlineData
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :outlineData
    #[sdk(attr(qname = ":outlineData"))]
    pub outline_data: Option<crate::simple_type::BooleanValue>,
    /// compactData
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :compactData
    #[sdk(attr(qname = ":compactData"))]
    pub compact_data: Option<crate::simple_type::BooleanValue>,
    /// published
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :published
    #[sdk(attr(qname = ":published"))]
    pub published: Option<crate::simple_type::BooleanValue>,
    /// gridDropZones
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :gridDropZones
    #[sdk(attr(qname = ":gridDropZones"))]
    pub grid_drop_zones: Option<crate::simple_type::BooleanValue>,
    /// immersive
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :immersive
    #[sdk(attr(qname = ":immersive"))]
    pub stop_immersive_ui: Option<crate::simple_type::BooleanValue>,
    /// multipleFieldFilters
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :multipleFieldFilters
    #[sdk(attr(qname = ":multipleFieldFilters"))]
    pub multiple_field_filters: Option<crate::simple_type::BooleanValue>,
    /// chartFormat
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :chartFormat
    #[sdk(attr(qname = ":chartFormat"))]
    pub chart_format: Option<crate::simple_type::UInt32Value>,
    /// rowHeaderCaption
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :rowHeaderCaption
    #[sdk(attr(qname = ":rowHeaderCaption"))]
    pub row_header_caption: Option<crate::simple_type::StringValue>,
    /// colHeaderCaption
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :colHeaderCaption
    #[sdk(attr(qname = ":colHeaderCaption"))]
    pub column_header_caption: Option<crate::simple_type::StringValue>,
    /// fieldListSortAscending
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :fieldListSortAscending
    #[sdk(attr(qname = ":fieldListSortAscending"))]
    pub field_list_sort_ascending: Option<crate::simple_type::BooleanValue>,
    /// mdxSubqueries
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :mdxSubqueries
    #[sdk(attr(qname = ":mdxSubqueries"))]
    pub mdx_subqueries: Option<crate::simple_type::BooleanValue>,
    /// customListSort
    ///
    /// Available in Office2007 and above.
    ///
    /// Represents the following attribute in the schema: :customListSort
    #[sdk(attr(qname = ":customListSort"))]
    pub custom_list_sort: Option<crate::simple_type::BooleanValue>,
    /// _
    #[sdk(child(qname = "x:CT_Location/x:location"))]
    pub location: std::boxed::Box<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Location,
    >,
    /// _
    #[sdk(child(qname = "x:CT_PivotFields/x:pivotFields"))]
    pub pivot_fields: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotFields,
    >,
    /// _
    #[sdk(child(qname = "x:CT_RowFields/x:rowFields"))]
    pub row_fields: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::RowFields,
    >,
    /// _
    #[sdk(child(qname = "x:CT_rowItems/x:rowItems"))]
    pub row_items: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::RowItems,
    >,
    /// _
    #[sdk(child(qname = "x:CT_ColFields/x:colFields"))]
    pub column_fields: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ColumnFields,
    >,
    /// _
    #[sdk(child(qname = "x:CT_colItems/x:colItems"))]
    pub column_items: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ColumnItems,
    >,
    /// _
    #[sdk(child(qname = "x:CT_PageFields/x:pageFields"))]
    pub page_fields: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PageFields,
    >,
    /// _
    #[sdk(child(qname = "x:CT_DataFields/x:dataFields"))]
    pub data_fields: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataFields,
    >,
    /// _
    #[sdk(child(qname = "x:CT_Formats/x:formats"))]
    pub formats: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Formats,
    >,
    /// _
    #[sdk(child(qname = "x:CT_ConditionalFormats/x:conditionalFormats"))]
    pub conditional_formats: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ConditionalFormats,
    >,
    /// _
    #[sdk(child(qname = "x:CT_ChartFormats/x:chartFormats"))]
    pub chart_formats: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ChartFormats,
    >,
    /// _
    #[sdk(child(qname = "x:CT_PivotHierarchies/x:pivotHierarchies"))]
    pub pivot_hierarchies: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotHierarchies,
    >,
    /// _
    #[sdk(child(qname = "x:CT_PivotTableStyle/x:pivotTableStyleInfo"))]
    pub pivot_table_style: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableStyle,
    >,
    /// _
    #[sdk(child(qname = "x:CT_PivotFilters/x:filters"))]
    pub pivot_filters: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotFilters,
    >,
    /// _
    #[sdk(child(qname = "x:CT_RowHierarchiesUsage/x:rowHierarchiesUsage"))]
    pub row_hierarchies_usage: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::RowHierarchiesUsage,
    >,
    /// _
    #[sdk(child(qname = "x:CT_ColHierarchiesUsage/x:colHierarchiesUsage"))]
    pub column_hierarchies_usage: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ColumnHierarchiesUsage,
    >,
    /// _
    #[sdk(child(qname = "x:CT_pivotTableDefinitionExtensionList/x:extLst"))]
    pub pivot_table_definition_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableDefinitionExtensionList,
    >,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RevExStreamChoice {
  /// Defines the RevExFuture Class.
  #[sdk(child(qname = "xr:CT_RevExFuture/xr:xrrftr"))]
  XrXrrftr(std::boxed::Box<RevExFuture>),
  /// Defines the RevExUnsupported Class.
  #[sdk(child(qname = "xr:CT_RevExUnsupported/xr:xrrUspt"))]
  XrXrrUspt(std::boxed::Box<RevExUnsupported>),
  /// Defines the RevExTrimmed Class.
  #[sdk(child(qname = "xr:CT_RevExTrimmed/xr:xrrTrim"))]
  XrXrrTrim(std::boxed::Box<RevExTrimmed>),
  /// Defines the RevExRowColumn Class.
  #[sdk(child(qname = "xr:CT_RevExRowColumn/xr:xrrrc"))]
  XrXrrrc(std::boxed::Box<RevExRowColumn>),
  /// Defines the RevExMove Class.
  #[sdk(child(qname = "xr:CT_RevExMove/xr:xrrm"))]
  XrXrrm(std::boxed::Box<RevExMove>),
  /// Defines the RevExChangeCell Class.
  #[sdk(child(qname = "xr:CT_RevExChangeCell/xr:xrrc"))]
  XrXrrc(std::boxed::Box<RevExChangeCell>),
  /// Defines the RevExFormatting Class.
  #[sdk(child(qname = "xr:CT_RevExFormatting/xr:xrrf"))]
  XrXrrf(std::boxed::Box<RevExFormatting>),
  /// Defines the RevExDefinedName Class.
  #[sdk(child(qname = "xr:CT_RevExDefinedName/xr:xrrDefName"))]
  XrXrrDefName(std::boxed::Box<RevExDefinedName>),
  /// Defines the RevExDelObj Class.
  #[sdk(child(qname = "xr:CT_RevExDelObj/xr:xrrdo"))]
  XrXrrdo(std::boxed::Box<RevExDelObj>),
  /// Defines the RevExChgObj Class.
  #[sdk(child(qname = "xr:CT_RevExChgObj/xr:xrrco"))]
  XrXrrco(std::boxed::Box<RevExChgObj>),
  /// Defines the RevExSheetOp Class.
  #[sdk(child(qname = "xr:CT_RevExSheetOp/xr:xrrSheet"))]
  XrXrrSheet(std::boxed::Box<RevExSheetOp>),
  /// Defines the RevisionList Class.
  #[sdk(child(qname = "xr:CT_RevisionList/xr:xrrList"))]
  XrXrrList(std::boxed::Box<RevisionList>),
  /// Defines the RevListAutoExpandRw Class.
  #[sdk(child(qname = "xr:CT_RevListAutoExpandRw/xr:xrrListExpR"))]
  XrXrrListExpR(std::boxed::Box<RevListAutoExpandRw>),
  /// Defines the RevGroup Class.
  #[sdk(child(qname = "xr:CT_RevGroup/xr:xrrg"))]
  XrXrrg(std::boxed::Box<RevGroup>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum StateBasedObjectChoice {
  /// Represents an external link to another workbook..
  #[sdk(child(qname = "x:CT_DataValidation/xr:dataValidation"))]
  XrDataValidation(std::boxed::Box<DataValidation>),
  /// Represents a hyperlink within a cell..
  #[sdk(child(qname = "x:CT_Hyperlink/xr:hyperlink"))]
  XrHyperlink(std::boxed::Box<Hyperlink>),
  /// Represents a sparkline group of 1 or more sparklines..
  #[sdk(child(qname = "x14:CT_SparklineGroup/xr:sparklineGroup"))]
  XrSparklineGroup(std::boxed::Box<SparklineGroup>),
  /// Represents one comment within a cell..
  #[sdk(child(qname = "x:CT_Comments/xr:comments"))]
  XrComments(std::boxed::Box<Comments>),
  /// Represents an autofilter..
  #[sdk(child(qname = "x:CT_AutoFilter/xr:autoFilter"))]
  XrAutoFilter(std::boxed::Box<AutoFilter>),
  /// Represents a PivotTable View..
  #[sdk(child(qname = "x:CT_pivotTableDefinition/xr:pivotTableDefinition"))]
  XrPivotTableDefinition(std::boxed::Box<PivotTableDefinition>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RevExChgObjChoice {
  #[sdk(child(qname = "xr:CT_RevisionStateLink/xr:link"))]
  XrLink(std::boxed::Box<RevisionStateLink>),
  #[sdk(child(qname = "xr:CT_RevisionState/xr:body"))]
  XrBody(std::boxed::Box<RevisionState>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RevGroupChoice {
  /// Defines the RevExFuture Class.
  #[sdk(child(qname = "xr:CT_RevExFuture/xr:xrrftr"))]
  XrXrrftr(std::boxed::Box<RevExFuture>),
  /// Defines the RevExUnsupported Class.
  #[sdk(child(qname = "xr:CT_RevExUnsupported/xr:xrrUspt"))]
  XrXrrUspt(std::boxed::Box<RevExUnsupported>),
  /// Defines the RevExTrimmed Class.
  #[sdk(child(qname = "xr:CT_RevExTrimmed/xr:xrrTrim"))]
  XrXrrTrim(std::boxed::Box<RevExTrimmed>),
  /// Defines the RevExRowColumn Class.
  #[sdk(child(qname = "xr:CT_RevExRowColumn/xr:xrrrc"))]
  XrXrrrc(std::boxed::Box<RevExRowColumn>),
  /// Defines the RevExMove Class.
  #[sdk(child(qname = "xr:CT_RevExMove/xr:xrrm"))]
  XrXrrm(std::boxed::Box<RevExMove>),
  /// Defines the RevExChangeCell Class.
  #[sdk(child(qname = "xr:CT_RevExChangeCell/xr:xrrc"))]
  XrXrrc(std::boxed::Box<RevExChangeCell>),
  /// Defines the RevExFormatting Class.
  #[sdk(child(qname = "xr:CT_RevExFormatting/xr:xrrf"))]
  XrXrrf(std::boxed::Box<RevExFormatting>),
  /// Defines the RevExDefinedName Class.
  #[sdk(child(qname = "xr:CT_RevExDefinedName/xr:xrrDefName"))]
  XrXrrDefName(std::boxed::Box<RevExDefinedName>),
  /// Defines the RevExDelObj Class.
  #[sdk(child(qname = "xr:CT_RevExDelObj/xr:xrrdo"))]
  XrXrrdo(std::boxed::Box<RevExDelObj>),
  /// Defines the RevExChgObj Class.
  #[sdk(child(qname = "xr:CT_RevExChgObj/xr:xrrco"))]
  XrXrrco(std::boxed::Box<RevExChgObj>),
  /// Defines the RevExSheetOp Class.
  #[sdk(child(qname = "xr:CT_RevExSheetOp/xr:xrrSheet"))]
  XrXrrSheet(std::boxed::Box<RevExSheetOp>),
  /// Defines the RevisionList Class.
  #[sdk(child(qname = "xr:CT_RevisionList/xr:xrrList"))]
  XrXrrList(std::boxed::Box<RevisionList>),
  /// Defines the RevListAutoExpandRw Class.
  #[sdk(child(qname = "xr:CT_RevListAutoExpandRw/xr:xrrListExpR"))]
  XrXrrListExpR(std::boxed::Box<RevListAutoExpandRw>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RevisionStateChoice {
  /// Defines the RowColVisualOps Class.
  #[sdk(child(qname = "xr:CT_RowColVisualOps/xr:rowColVisualOps"))]
  XrRowColVisualOps(std::boxed::Box<RowColVisualOps>),
  /// Defines the HideUnhideSheet Class.
  #[sdk(child(qname = "xr:CT_HideUnhideSheet/xr:hideUnhideSheet"))]
  XrHideUnhideSheet(std::boxed::Box<HideUnhideSheet>),
  /// Defines the ShowGridlinesHeadings Class.
  #[sdk(child(qname = "xr:CT_ShowGridlinesHeadings/xr:showGridlinesHeadings"))]
  XrShowGridlinesHeadings(std::boxed::Box<ShowGridlinesHeadings>),
  /// Defines the FreezePanes Class.
  #[sdk(child(qname = "xr:CT_FreezePanes/xr:freezePanes"))]
  XrFreezePanes(std::boxed::Box<FreezePanes>),
  /// Defines the Outlines Class.
  #[sdk(child(qname = "xr:CT_Outlines/xr:outlines"))]
  XrOutlines(std::boxed::Box<Outlines>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RefMapChoice {
  /// Defines the RefCell Class.
  #[sdk(child(qname = "xr:CT_RefCell/xr:ref"))]
  XrRef(std::boxed::Box<RefCell>),
  /// Defines the SheetXluid Class.
  #[sdk(child(qname = "xr:CT_SheetXluid/xr:sheetUid"))]
  XrSheetUid(std::boxed::Box<SheetXluid>),
  /// Defines the RefOartAnchor Class.
  #[sdk(child(qname = "xr:CT_RefOartAnchor/xr:oartAnchor"))]
  XrOartAnchor(std::boxed::Box<RefOartAnchor>),
  /// Defines the RefFuture Class.
  #[sdk(empty_child(qname = "xr:CT_RefFuture/xr:future"))]
  XrFuture,
  /// Defines the RefTest Class.
  #[sdk(child(qname = "xr:CT_RefTest/xr:test"))]
  XrTest(std::boxed::Box<RefTest>),
}
