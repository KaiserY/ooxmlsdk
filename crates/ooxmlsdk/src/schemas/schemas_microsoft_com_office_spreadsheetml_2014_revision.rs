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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevExHeaders/xr:revHdrs")]
pub struct RevExHeaders {
  /// minRev
  #[sdk(attr(office2016, qname = ":minRev"))]
  pub min_rev: crate::simple_type::UInt64Value,
  /// maxRev
  #[sdk(attr(office2016, qname = ":maxRev"))]
  pub max_rev: crate::simple_type::UInt64Value,
  /// docId
  #[sdk(attr(office2016, qname = ":docId"))]
  pub doc_id: crate::simple_type::StringValue,
  /// endpointId
  #[sdk(attr(office2016, qname = ":endpointId"))]
  pub endpoint_id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(office2016, qname = "xr:CT_RevExHeader/xr:hdr"))]
  pub xr_hdr: Vec<RevExHeader>,
}
/// Defines the RevExStream Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevExStream/xr:revStream")]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "x:CT_Dxf/xr:dxf")]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevisionPtr/xr:revisionPtr")]
pub struct RevisionPtr {
  /// revIDLastSave
  #[sdk(attr(office2016, qname = ":revIDLastSave"))]
  pub rev_id_last_save: crate::simple_type::UInt64Value,
  /// documentId
  #[sdk(attr(office2016, qname = ":documentId"))]
  pub document_id: crate::simple_type::StringValue,
}
/// Defines the StateBasedObject Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_StateBasedObject/xr:objectState")]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevExHeader/xr:hdr")]
pub struct RevExHeader {
  /// id
  #[sdk(attr(office2016, qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
  /// minRev
  #[sdk(attr(office2016, qname = ":minRev"))]
  pub min_rev: Option<crate::simple_type::UInt64Value>,
  /// maxRev
  #[sdk(attr(office2016, qname = ":maxRev"))]
  pub max_rev: Option<crate::simple_type::UInt64Value>,
  /// time
  #[sdk(attr(office2016, qname = ":time"))]
  pub time: crate::simple_type::DateTimeValue,
}
/// Defines the RevExFuture Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevExFuture/xr:xrrftr")]
pub struct RevExFuture {
  /// rev
  #[sdk(attr(office2016, qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(office2016, qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(office2016, qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(office2016, qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(office2016, qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// sti
  #[sdk(attr(office2016, qname = ":sti"))]
  pub sti: Option<crate::simple_type::BooleanValue>,
  /// Defines the RevExTest Class.
  #[sdk(empty_child(office2016, qname = "xr:CT_RevExTest/xr:xrrtest"))]
  pub rev_ex_test: Option<()>,
}
/// Defines the RevExUnsupported Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevExUnsupported/xr:xrrUspt")]
pub struct RevExUnsupported {
  /// rev
  #[sdk(attr(office2016, qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(office2016, qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(office2016, qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(office2016, qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(office2016, qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
}
/// Defines the RevExTrimmed Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevExTrimmed/xr:xrrTrim")]
pub struct RevExTrimmed {
  /// rev
  #[sdk(attr(office2016, qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(office2016, qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(office2016, qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(office2016, qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(office2016, qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
}
/// Defines the RevExRowColumn Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevExRowColumn/xr:xrrrc")]
pub struct RevExRowColumn {
  /// rev
  #[sdk(attr(office2016, qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(office2016, qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(office2016, qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(office2016, qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(office2016, qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// eol
  #[sdk(attr(office2016, qname = ":eol"))]
  pub eol: Option<crate::simple_type::BooleanValue>,
  /// ref
  #[sdk(attr(office2016, qname = ":ref"))]
  pub r#ref: crate::simple_type::StringValue,
  /// action
  #[sdk(attr(office2016, qname = ":action"))]
  pub action: RwColAction,
}
/// Defines the RevExMove Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevExMove/xr:xrrm")]
pub struct RevExMove {
  /// rev
  #[sdk(attr(office2016, qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(office2016, qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(office2016, qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(office2016, qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(office2016, qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// src
  #[sdk(attr(office2016, qname = ":src"))]
  pub src: crate::simple_type::StringValue,
  /// dst
  #[sdk(attr(office2016, qname = ":dst"))]
  pub dst: crate::simple_type::StringValue,
  /// srcSh
  #[sdk(attr(office2016, qname = ":srcSh"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub src_sh: Option<crate::simple_type::StringValue>,
}
/// Defines the RevExChangeCell Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevExChangeCell/xr:xrrc")]
pub struct RevExChangeCell {
  /// listUid
  #[sdk(attr(office2016, qname = ":listUid"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub list_uid: Option<crate::simple_type::StringValue>,
  /// rev
  #[sdk(attr(office2016, qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(office2016, qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(office2016, qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(office2016, qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(office2016, qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// r
  #[sdk(attr(office2016, qname = ":r"))]
  pub r: crate::simple_type::StringValue,
  /// t
  #[sdk(attr(office2016, qname = ":t"))]
  pub t: Option<FillType>,
  /// x
  #[sdk(attr(office2016, qname = ":x"))]
  pub x: Option<FillTypeExt>,
  /// w
  #[sdk(attr(office2016, qname = ":w"))]
  pub w: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(office2016, qname = "xr:CT_RevCell/xr:c"))]
  pub xr_c: Vec<RevCell>,
  /// _
  #[sdk(child(office2016, qname = "xr:CT_ChangeCellSubEdit/xr:ccse"))]
  pub xr_ccse: Vec<ChangeCellSubEdit>,
}
/// Defines the RevExFormatting Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevExFormatting/xr:xrrf")]
pub struct RevExFormatting {
  /// rev
  #[sdk(attr(office2016, qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(office2016, qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(office2016, qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(office2016, qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(office2016, qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// numFmtId
  #[sdk(attr(office2016, qname = ":numFmtId"))]
  pub num_fmt_id: Option<crate::simple_type::UInt32Value>,
  /// xfDxf
  #[sdk(attr(office2016, qname = ":xfDxf"))]
  pub xf_dxf: Option<crate::simple_type::BooleanValue>,
  /// style
  #[sdk(attr(office2016, qname = ":style"))]
  pub style: Option<crate::simple_type::BooleanValue>,
  /// sqref
  #[sdk(attr(office2016, qname = ":sqref"))]
  pub sqref: crate::simple_type::ListValue<crate::simple_type::StringValue>,
  /// start
  #[sdk(attr(office2016, qname = ":start"))]
  pub start: Option<crate::simple_type::UInt32Value>,
  /// length
  #[sdk(attr(office2016, qname = ":length"))]
  pub length: Option<crate::simple_type::UInt32Value>,
  /// styleUid
  #[sdk(attr(office2016, qname = ":styleUid"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub style_uid: Option<crate::simple_type::StringValue>,
  /// fBlankCell
  #[sdk(attr(office2016, qname = ":fBlankCell"))]
  pub f_blank_cell: Option<crate::simple_type::BooleanValue>,
  /// applyNumberFormat
  #[sdk(attr(office2016, qname = ":applyNumberFormat"))]
  pub apply_number_format: Option<crate::simple_type::BooleanValue>,
  /// applyFont
  #[sdk(attr(office2016, qname = ":applyFont"))]
  pub apply_font: Option<crate::simple_type::BooleanValue>,
  /// applyFill
  #[sdk(attr(office2016, qname = ":applyFill"))]
  pub apply_fill: Option<crate::simple_type::BooleanValue>,
  /// applyBorder
  #[sdk(attr(office2016, qname = ":applyBorder"))]
  pub apply_border: Option<crate::simple_type::BooleanValue>,
  /// applyAlignment
  #[sdk(attr(office2016, qname = ":applyAlignment"))]
  pub apply_alignment: Option<crate::simple_type::BooleanValue>,
  /// applyProtection
  #[sdk(attr(office2016, qname = ":applyProtection"))]
  pub apply_protection: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(office2016, qname = "x:CT_Dxf/xr:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
  /// _
  #[sdk(child(office2016, qname = "x:CT_ExtensionList/xr:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the RevExDefinedName Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevExDefinedName/xr:xrrDefName")]
pub struct RevExDefinedName {
  /// rev
  #[sdk(attr(office2016, qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(office2016, qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(office2016, qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(office2016, qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(office2016, qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// customView
  #[sdk(attr(office2016, qname = ":customView"))]
  pub custom_view: Option<crate::simple_type::BooleanValue>,
  /// name
  #[sdk(attr(office2016, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// function
  #[sdk(attr(office2016, qname = ":function"))]
  pub function: Option<crate::simple_type::BooleanValue>,
  /// functionGroupId
  #[sdk(attr(office2016, qname = ":functionGroupId"))]
  pub function_group_id: Option<crate::simple_type::ByteValue>,
  /// shortcutKey
  #[sdk(attr(office2016, qname = ":shortcutKey"))]
  pub shortcut_key: Option<crate::simple_type::ByteValue>,
  /// hidden
  #[sdk(attr(office2016, qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// customMenu
  #[sdk(attr(office2016, qname = ":customMenu"))]
  pub custom_menu: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(office2016, qname = ":description"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// help
  #[sdk(attr(office2016, qname = ":help"))]
  pub help: Option<crate::simple_type::StringValue>,
  /// statusBar
  #[sdk(attr(office2016, qname = ":statusBar"))]
  pub status_bar: Option<crate::simple_type::StringValue>,
  /// comment
  #[sdk(attr(office2016, qname = ":comment"))]
  pub comment: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(office2016, qname = "x:ST_Formula/xr:formula"))]
  pub formula_formula: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(office2016, qname = "x:CT_ExtensionList/xr:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the RevExDelObj Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevExDelObj/xr:xrrdo")]
pub struct RevExDelObj {
  /// rev
  #[sdk(attr(office2016, qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(office2016, qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(office2016, qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(office2016, qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(office2016, qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// _
  #[sdk(child(office2016, qname = "xr:CT_StateBasedHeader/xr:hdr"))]
  pub state_based_header: std::boxed::Box<StateBasedHeader>,
}
/// Defines the RevExChgObj Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevExChgObj/xr:xrrco")]
pub struct RevExChgObj {
  /// rev
  #[sdk(attr(office2016, qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(office2016, qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(office2016, qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(office2016, qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(office2016, qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// _
  #[sdk(child(office2016, qname = "xr:CT_StateBasedHeader/xr:hdr"))]
  pub state_based_header: std::boxed::Box<StateBasedHeader>,
  #[sdk(choice(
    microsoft365,
    qname = "xr:CT_RevisionStateLink/xr:link",
    qname = "xr:CT_RevisionState/xr:body"
  ))]
  pub rev_ex_chg_obj_choice: Option<RevExChgObjChoice>,
}
/// Defines the RevExSheetOp Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevExSheetOp/xr:xrrSheet")]
pub struct RevExSheetOp {
  /// rev
  #[sdk(attr(office2016, qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(office2016, qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(office2016, qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(office2016, qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(office2016, qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// op
  #[sdk(attr(office2016, qname = ":op"))]
  pub op: SheetOp,
  /// name
  #[sdk(attr(office2016, qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// idOrig
  #[sdk(attr(office2016, qname = ":idOrig"))]
  pub id_orig: Option<crate::simple_type::UInt32Value>,
  /// idNew
  #[sdk(attr(office2016, qname = ":idNew"))]
  pub id_new: Option<crate::simple_type::UInt32Value>,
}
/// Defines the RevisionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevisionList/xr:xrrList")]
pub struct RevisionList {
  /// rev
  #[sdk(attr(office2016, qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(office2016, qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(office2016, qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(office2016, qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(office2016, qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// Data
  #[sdk(attr(office2016, qname = ":Data"))]
  pub data: Option<crate::simple_type::BooleanValue>,
  /// Formatting
  #[sdk(attr(office2016, qname = ":Formatting"))]
  pub formatting: Option<crate::simple_type::BooleanValue>,
  /// RangeBased
  #[sdk(attr(office2016, qname = ":RangeBased"))]
  pub range_based: Option<crate::simple_type::BooleanValue>,
  /// Fake
  #[sdk(attr(office2016, qname = ":Fake"))]
  pub fake: Option<crate::simple_type::BooleanValue>,
  /// ref
  #[sdk(attr(office2016, qname = ":ref"))]
  pub r#ref: crate::simple_type::StringValue,
  /// Headers
  #[sdk(attr(office2016, qname = ":Headers"))]
  pub headers: Option<crate::simple_type::BooleanValue>,
  /// InsDelHeaders
  #[sdk(attr(office2016, qname = ":InsDelHeaders"))]
  pub ins_del_headers: Option<crate::simple_type::BooleanValue>,
  /// rId
  #[sdk(attr(office2016, qname = ":rId"))]
  pub r_id: crate::simple_type::UInt32Value,
}
/// Defines the RevListAutoExpandRw Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevListAutoExpandRw/xr:xrrListExpR")]
pub struct RevListAutoExpandRw {
  /// rev
  #[sdk(attr(office2016, qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(office2016, qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(office2016, qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(office2016, qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(office2016, qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// refAdded
  #[sdk(attr(office2016, qname = ":refAdded"))]
  pub ref_added: crate::simple_type::StringValue,
  /// listGuid
  #[sdk(attr(office2016, qname = ":listGuid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub list_guid: crate::simple_type::StringValue,
}
/// Defines the RevGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevGroup/xr:xrrg")]
pub struct RevGroup {
  /// rev
  #[sdk(attr(office2016, qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(office2016, qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(office2016, qname = ":sh"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(office2016, qname = ":uidp"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(office2016, qname = ":ctx"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevCell/xr:c")]
pub struct RevCell {
  /// t
  #[sdk(attr(office2016, qname = ":t"))]
  pub t: Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CellValues>,
  /// nop
  #[sdk(attr(office2016, qname = ":nop"))]
  pub nop: Option<crate::simple_type::BooleanValue>,
  /// tick
  #[sdk(attr(office2016, qname = ":tick"))]
  pub tick: Option<crate::simple_type::BooleanValue>,
  /// rep
  #[sdk(attr(office2016, qname = ":rep"))]
  pub rep: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(text_child(office2016, qname = "x:ST_Formula/xr:f"))]
  pub f_formula: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(office2016, qname = "x:ST_Xstring/xr:v"))]
  pub xstring: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(office2016, qname = "x:CT_Rst/xr:is"))]
  pub rst_type: Option<std::boxed::Box<RstType>>,
}
/// Defines the ChangeCellSubEdit Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_ChangeCellSubEdit/xr:ccse")]
pub struct ChangeCellSubEdit {
  /// r
  #[sdk(attr(office2016, qname = ":r"))]
  pub r: crate::simple_type::StringValue,
  /// t
  #[sdk(attr(office2016, qname = ":t"))]
  pub t: Option<FillType>,
  /// x
  #[sdk(attr(office2016, qname = ":x"))]
  pub x: Option<FillTypeExt>,
  /// w
  #[sdk(attr(office2016, qname = ":w"))]
  pub w: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(office2016, qname = "xr:CT_RevCell/xr:c"))]
  pub xr_c: Vec<RevCell>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "x:CT_ExtensionList/xr:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
/// Defines the FormulaFormula Class.
pub type FormulaFormula = crate::simple_type::StringValue;
/// Defines the FFormula Class.
pub type FFormula = crate::simple_type::StringValue;
/// Defines the StateBasedHeader Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_StateBasedHeader/xr:hdr")]
pub struct StateBasedHeader {
  /// uid
  #[sdk(attr(office2016, qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// eft
  #[sdk(attr(office2016, qname = ":eft"))]
  pub eft: FeatureType,
  /// eftx
  #[sdk(attr(office2016, qname = ":eftx"))]
  pub eftx: Option<ExtFeatureType>,
  /// seft
  #[sdk(attr(office2016, qname = ":seft"))]
  pub seft: Option<SubFeatureType>,
  /// seftx
  #[sdk(attr(office2016, qname = ":seftx"))]
  pub seftx: Option<ExtSubFeatureType>,
  /// _
  #[sdk(child(office2016, qname = "xr:CT_RefMap/xr:refmap"))]
  pub ref_map: Option<RefMap>,
}
/// Defines the RevisionStateLink Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevisionStateLink/xr:link")]
pub struct RevisionStateLink {
  /// id
  #[sdk(attr(office2016, qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the RevisionState Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RevisionState/xr:body")]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RefMap/xr:refmap")]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RowColVisualOps/xr:rowColVisualOps")]
pub struct RowColVisualOps {
  /// action
  #[sdk(attr(office2016, qname = ":action"))]
  pub action: RowColVisualOp,
  /// isRow
  #[sdk(attr(office2016, qname = ":isRow"))]
  pub is_row: crate::simple_type::BooleanValue,
  /// size
  #[sdk(attr(office2016, qname = ":size"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// userSized
  #[sdk(attr(office2016, qname = ":userSized"))]
  pub user_sized: Option<crate::simple_type::BooleanValue>,
}
/// Defines the HideUnhideSheet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_HideUnhideSheet/xr:hideUnhideSheet")]
pub struct HideUnhideSheet {
  /// hide
  #[sdk(attr(office2016, qname = ":hide"))]
  pub hide: crate::simple_type::BooleanValue,
}
/// Defines the ShowGridlinesHeadings Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "xr:CT_ShowGridlinesHeadings/xr:showGridlinesHeadings"
)]
pub struct ShowGridlinesHeadings {
  /// showGridLines
  #[sdk(attr(office2016, qname = ":showGridLines"))]
  pub show_grid_lines: crate::simple_type::BooleanValue,
  /// showRowCol
  #[sdk(attr(office2016, qname = ":showRowCol"))]
  pub show_row_col: crate::simple_type::BooleanValue,
}
/// Defines the FreezePanes Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_FreezePanes/xr:freezePanes")]
pub struct FreezePanes {
  /// sheetViewUid
  #[sdk(attr(office2016, qname = ":sheetViewUid"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub sheet_view_uid: Option<crate::simple_type::StringValue>,
}
/// Defines the Outlines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_Outlines/xr:outlines")]
pub struct Outlines {
  /// isRow
  #[sdk(attr(office2016, qname = ":isRow"))]
  pub is_row: crate::simple_type::BooleanValue,
  /// _
  #[sdk(child(office2016, qname = "xr:CT_Outline/xr:outline"))]
  pub xr_outline: Vec<Outline>,
}
/// Defines the Outline Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_Outline/xr:outline")]
pub struct Outline {
  /// isCollapsed
  #[sdk(attr(office2016, qname = ":isCollapsed"))]
  pub is_collapsed: crate::simple_type::BooleanValue,
  /// level
  #[sdk(attr(office2016, qname = ":level"))]
  pub level: crate::simple_type::ByteValue,
}
/// Defines the Xstring Class.
pub type Xstring = crate::simple_type::StringValue;
/// Defines the RstType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "x:CT_Rst/xr:is")]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RefCell/xr:ref")]
pub struct RefCell {
  /// n
  #[sdk(attr(office2016, qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// ajt
  #[sdk(attr(office2016, qname = ":ajt"))]
  pub ajt: AdjustType,
  /// ajtx
  #[sdk(attr(office2016, qname = ":ajtx"))]
  pub ajtx: Option<AdjustTypeExt>,
  /// homeRef
  #[sdk(attr(office2016, qname = ":homeRef"))]
  pub home_ref: Option<crate::simple_type::BooleanValue>,
  /// r
  #[sdk(attr(office2016, qname = ":r"))]
  pub r: crate::simple_type::ListValue<crate::simple_type::StringValue>,
  /// uid
  #[sdk(attr(office2016, qname = ":uid"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uid: Option<crate::simple_type::StringValue>,
  /// uidLast
  #[sdk(attr(office2016, qname = ":uidLast"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uid_last: Option<crate::simple_type::StringValue>,
}
/// Defines the SheetXluid Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_SheetXluid/xr:sheetUid")]
pub struct SheetXluid {
  /// n
  #[sdk(attr(office2016, qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// ajt
  #[sdk(attr(office2016, qname = ":ajt"))]
  pub ajt: AdjustType,
  /// ajtx
  #[sdk(attr(office2016, qname = ":ajtx"))]
  pub ajtx: Option<AdjustTypeExt>,
  /// homeRef
  #[sdk(attr(office2016, qname = ":homeRef"))]
  pub home_ref: Option<crate::simple_type::BooleanValue>,
  /// uid
  #[sdk(attr(office2016, qname = ":uid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uid: crate::simple_type::StringValue,
}
/// Defines the RefOartAnchor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RefOartAnchor/xr:oartAnchor")]
pub struct RefOartAnchor {
  /// n
  #[sdk(attr(office2016, qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// ajt
  #[sdk(attr(office2016, qname = ":ajt"))]
  pub ajt: AdjustType,
  /// ajtx
  #[sdk(attr(office2016, qname = ":ajtx"))]
  pub ajtx: Option<AdjustTypeExt>,
  /// homeRef
  #[sdk(attr(office2016, qname = ":homeRef"))]
  pub home_ref: Option<crate::simple_type::BooleanValue>,
  /// r
  #[sdk(attr(office2016, qname = ":r"))]
  pub r: Option<crate::simple_type::StringValue>,
  /// fromRowOff
  #[sdk(attr(office2016, qname = ":fromRowOff"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub from_row_off: Option<crate::simple_type::Int64Value>,
  /// fromColOff
  #[sdk(attr(office2016, qname = ":fromColOff"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub from_col_off: Option<crate::simple_type::Int64Value>,
  /// toRowOff
  #[sdk(attr(office2016, qname = ":toRowOff"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub to_row_off: Option<crate::simple_type::Int64Value>,
  /// toColOff
  #[sdk(attr(office2016, qname = ":toColOff"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub to_col_off: Option<crate::simple_type::Int64Value>,
  /// cx
  #[sdk(attr(office2016, qname = ":cx"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cx: Option<crate::simple_type::Int64Value>,
  /// cy
  #[sdk(attr(office2016, qname = ":cy"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "2147483647",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub cy: Option<crate::simple_type::Int64Value>,
  /// x
  #[sdk(attr(office2016, qname = ":x"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub x: Option<crate::simple_type::Int64Value>,
  /// y
  #[sdk(attr(office2016, qname = ":y"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub y: Option<crate::simple_type::Int64Value>,
  /// oat
  #[sdk(attr(office2016, qname = ":oat"))]
  pub oat: OartAnchorType,
}
/// Defines the RefTest Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "xr:CT_RefTest/xr:test")]
pub struct RefTest {
  /// n
  #[sdk(attr(office2016, qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// ajt
  #[sdk(attr(office2016, qname = ":ajt"))]
  pub ajt: AdjustType,
  /// ajtx
  #[sdk(attr(office2016, qname = ":ajtx"))]
  pub ajtx: Option<AdjustTypeExt>,
  /// homeRef
  #[sdk(attr(office2016, qname = ":homeRef"))]
  pub home_ref: Option<crate::simple_type::BooleanValue>,
}
/// Represents an external link to another workbook..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "x:CT_DataValidation/xr:dataValidation")]
pub struct DataValidation {
    /// type
    #[sdk(attr(qname = ":type"))]
    pub r#type: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationValues,
    >,
    /// errorStyle
    #[sdk(attr(qname = ":errorStyle"))]
    pub error_style: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationErrorStyleValues,
    >,
    /// imeMode
    #[sdk(attr(qname = ":imeMode"))]
    pub ime_mode: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationImeModeValues,
    >,
    /// operator
    #[sdk(attr(qname = ":operator"))]
    pub operator: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationOperatorValues,
    >,
    /// allowBlank
    #[sdk(attr(qname = ":allowBlank"))]
    pub allow_blank: Option<crate::simple_type::BooleanValue>,
    /// showDropDown
    #[sdk(attr(qname = ":showDropDown"))]
    pub show_drop_down: Option<crate::simple_type::BooleanValue>,
    /// showInputMessage
    #[sdk(attr(qname = ":showInputMessage"))]
    pub show_input_message: Option<crate::simple_type::BooleanValue>,
    /// showErrorMessage
    #[sdk(attr(qname = ":showErrorMessage"))]
    pub show_error_message: Option<crate::simple_type::BooleanValue>,
    /// errorTitle
    #[sdk(attr(qname = ":errorTitle"))]
    pub error_title: Option<crate::simple_type::StringValue>,
    /// error
    #[sdk(attr(qname = ":error"))]
    pub error: Option<crate::simple_type::StringValue>,
    /// promptTitle
    #[sdk(attr(qname = ":promptTitle"))]
    pub prompt_title: Option<crate::simple_type::StringValue>,
    /// prompt
    #[sdk(attr(qname = ":prompt"))]
    pub prompt: Option<crate::simple_type::StringValue>,
    /// sqref
    #[sdk(attr(qname = ":sqref"))]
    pub sequence_of_references: crate::simple_type::ListValue<
        crate::simple_type::StringValue,
    >,
    /// _
    #[sdk(text_child(office2013, qname = "x:ST_Xstring/x12ac:list"))]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "x:CT_Hyperlink/xr:hyperlink")]
pub struct Hyperlink {
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// Relationship Id
  #[sdk(attr(qname = "r:id"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Location
  #[sdk(attr(qname = ":location"))]
  pub location: Option<crate::simple_type::StringValue>,
  /// Tool Tip
  #[sdk(attr(qname = ":tooltip"))]
  pub tooltip: Option<crate::simple_type::StringValue>,
  /// Display String
  #[sdk(attr(qname = ":display"))]
  pub display: Option<crate::simple_type::StringValue>,
}
/// Represents a sparkline group of 1 or more sparklines..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "x14:CT_SparklineGroup/xr:sparklineGroup")]
pub struct SparklineGroup {
    /// manualMax
    #[sdk(attr(office2010, qname = ":manualMax"))]
    pub manual_max: Option<crate::simple_type::DoubleValue>,
    /// manualMin
    #[sdk(attr(office2010, qname = ":manualMin"))]
    pub manual_min: Option<crate::simple_type::DoubleValue>,
    /// lineWeight
    #[sdk(attr(office2010, qname = ":lineWeight"))]
    pub line_weight: Option<crate::simple_type::DoubleValue>,
    /// type
    #[sdk(attr(office2010, qname = ":type"))]
    pub r#type: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SparklineTypeValues,
    >,
    /// dateAxis
    #[sdk(attr(office2010, qname = ":dateAxis"))]
    pub date_axis: Option<crate::simple_type::BooleanValue>,
    /// displayEmptyCellsAs
    #[sdk(attr(office2010, qname = ":displayEmptyCellsAs"))]
    pub display_empty_cells_as: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DisplayBlanksAsValues,
    >,
    /// markers
    #[sdk(attr(office2010, qname = ":markers"))]
    pub markers: Option<crate::simple_type::BooleanValue>,
    /// high
    #[sdk(attr(office2010, qname = ":high"))]
    pub high: Option<crate::simple_type::BooleanValue>,
    /// low
    #[sdk(attr(office2010, qname = ":low"))]
    pub low: Option<crate::simple_type::BooleanValue>,
    /// first
    #[sdk(attr(office2010, qname = ":first"))]
    pub first: Option<crate::simple_type::BooleanValue>,
    /// last
    #[sdk(attr(office2010, qname = ":last"))]
    pub last: Option<crate::simple_type::BooleanValue>,
    /// negative
    #[sdk(attr(office2010, qname = ":negative"))]
    pub negative: Option<crate::simple_type::BooleanValue>,
    /// displayXAxis
    #[sdk(attr(office2010, qname = ":displayXAxis"))]
    pub display_x_axis: Option<crate::simple_type::BooleanValue>,
    /// displayHidden
    #[sdk(attr(office2010, qname = ":displayHidden"))]
    pub display_hidden: Option<crate::simple_type::BooleanValue>,
    /// minAxisType
    #[sdk(attr(office2010, qname = ":minAxisType"))]
    pub min_axis_type: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SparklineAxisMinMaxValues,
    >,
    /// maxAxisType
    #[sdk(attr(office2010, qname = ":maxAxisType"))]
    pub max_axis_type: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SparklineAxisMinMaxValues,
    >,
    /// rightToLeft
    #[sdk(attr(office2010, qname = ":rightToLeft"))]
    pub right_to_left: Option<crate::simple_type::BooleanValue>,
    /// _
    #[sdk(child(office2010, qname = "x:CT_Color/x14:colorSeries"))]
    pub series_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SeriesColor,
    >,
    /// _
    #[sdk(child(office2010, qname = "x:CT_Color/x14:colorNegative"))]
    pub negative_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::NegativeColor,
    >,
    /// _
    #[sdk(child(office2010, qname = "x:CT_Color/x14:colorAxis"))]
    pub axis_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::AxisColor,
    >,
    /// _
    #[sdk(child(office2010, qname = "x:CT_Color/x14:colorMarkers"))]
    pub markers_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::MarkersColor,
    >,
    /// _
    #[sdk(child(office2010, qname = "x:CT_Color/x14:colorFirst"))]
    pub first_marker_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::FirstMarkerColor,
    >,
    /// _
    #[sdk(child(office2010, qname = "x:CT_Color/x14:colorLast"))]
    pub last_marker_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::LastMarkerColor,
    >,
    /// _
    #[sdk(child(office2010, qname = "x:CT_Color/x14:colorHigh"))]
    pub high_marker_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::HighMarkerColor,
    >,
    /// _
    #[sdk(child(office2010, qname = "x:CT_Color/x14:colorLow"))]
    pub low_marker_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::LowMarkerColor,
    >,
    /// _
    #[sdk(text_child(office2010, qname = "x:ST_Formula/xne:f"))]
    pub formula: Option<crate::simple_type::StringValue>,
    /// _
    #[sdk(child(office2010, qname = "x14:CT_Sparklines/x14:sparklines"))]
    pub sparklines: std::boxed::Box<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Sparklines,
    >,
}
/// Represents one comment within a cell..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "x:CT_Comments/xr:comments")]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "x:CT_AutoFilter/xr:autoFilter")]
pub struct AutoFilter {
  /// Cell or Range Reference
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "x:CT_pivotTableDefinition/xr:pivotTableDefinition"
)]
pub struct PivotTableDefinition {
    /// name
    #[sdk(attr(qname = ":name"))]
    pub name: crate::simple_type::StringValue,
    /// cacheId
    #[sdk(attr(qname = ":cacheId"))]
    pub cache_id: crate::simple_type::UInt32Value,
    /// dataOnRows
    #[sdk(attr(qname = ":dataOnRows"))]
    pub data_on_rows: Option<crate::simple_type::BooleanValue>,
    /// dataPosition
    #[sdk(attr(qname = ":dataPosition"))]
    pub data_position: Option<crate::simple_type::UInt32Value>,
    /// Auto Format Id
    #[sdk(attr(qname = ":autoFormatId"))]
    pub auto_format_id: Option<crate::simple_type::UInt32Value>,
    /// Apply Number Formats
    #[sdk(attr(qname = ":applyNumberFormats"))]
    pub apply_number_formats: Option<crate::simple_type::BooleanValue>,
    /// Apply Border Formats
    #[sdk(attr(qname = ":applyBorderFormats"))]
    pub apply_border_formats: Option<crate::simple_type::BooleanValue>,
    /// Apply Font Formats
    #[sdk(attr(qname = ":applyFontFormats"))]
    pub apply_font_formats: Option<crate::simple_type::BooleanValue>,
    /// Apply Pattern Formats
    #[sdk(attr(qname = ":applyPatternFormats"))]
    pub apply_pattern_formats: Option<crate::simple_type::BooleanValue>,
    /// Apply Alignment Formats
    #[sdk(attr(qname = ":applyAlignmentFormats"))]
    pub apply_alignment_formats: Option<crate::simple_type::BooleanValue>,
    /// Apply Width / Height Formats
    #[sdk(attr(qname = ":applyWidthHeightFormats"))]
    pub apply_width_height_formats: Option<crate::simple_type::BooleanValue>,
    /// dataCaption
    #[sdk(attr(qname = ":dataCaption"))]
    pub data_caption: crate::simple_type::StringValue,
    /// grandTotalCaption
    #[sdk(attr(qname = ":grandTotalCaption"))]
    pub grand_total_caption: Option<crate::simple_type::StringValue>,
    /// errorCaption
    #[sdk(attr(qname = ":errorCaption"))]
    pub error_caption: Option<crate::simple_type::StringValue>,
    /// showError
    #[sdk(attr(qname = ":showError"))]
    pub show_error: Option<crate::simple_type::BooleanValue>,
    /// missingCaption
    #[sdk(attr(qname = ":missingCaption"))]
    pub missing_caption: Option<crate::simple_type::StringValue>,
    /// showMissing
    #[sdk(attr(qname = ":showMissing"))]
    pub show_missing: Option<crate::simple_type::BooleanValue>,
    /// pageStyle
    #[sdk(attr(qname = ":pageStyle"))]
    pub page_style: Option<crate::simple_type::StringValue>,
    /// pivotTableStyle
    #[sdk(attr(qname = ":pivotTableStyle"))]
    pub pivot_table_style_name: Option<crate::simple_type::StringValue>,
    /// vacatedStyle
    #[sdk(attr(qname = ":vacatedStyle"))]
    pub vacated_style: Option<crate::simple_type::StringValue>,
    /// tag
    #[sdk(attr(qname = ":tag"))]
    pub tag: Option<crate::simple_type::StringValue>,
    /// updatedVersion
    #[sdk(attr(qname = ":updatedVersion"))]
    pub updated_version: Option<crate::simple_type::ByteValue>,
    /// minRefreshableVersion
    #[sdk(attr(qname = ":minRefreshableVersion"))]
    pub min_refreshable_version: Option<crate::simple_type::ByteValue>,
    /// asteriskTotals
    #[sdk(attr(qname = ":asteriskTotals"))]
    pub asterisk_totals: Option<crate::simple_type::BooleanValue>,
    /// showItems
    #[sdk(attr(qname = ":showItems"))]
    pub show_items: Option<crate::simple_type::BooleanValue>,
    /// editData
    #[sdk(attr(qname = ":editData"))]
    pub edit_data: Option<crate::simple_type::BooleanValue>,
    /// disableFieldList
    #[sdk(attr(qname = ":disableFieldList"))]
    pub disable_field_list: Option<crate::simple_type::BooleanValue>,
    /// showCalcMbrs
    #[sdk(attr(qname = ":showCalcMbrs"))]
    pub show_calculated_members: Option<crate::simple_type::BooleanValue>,
    /// visualTotals
    #[sdk(attr(qname = ":visualTotals"))]
    pub visual_totals: Option<crate::simple_type::BooleanValue>,
    /// showMultipleLabel
    #[sdk(attr(qname = ":showMultipleLabel"))]
    pub show_multiple_label: Option<crate::simple_type::BooleanValue>,
    /// showDataDropDown
    #[sdk(attr(qname = ":showDataDropDown"))]
    pub show_data_drop_down: Option<crate::simple_type::BooleanValue>,
    /// showDrill
    #[sdk(attr(qname = ":showDrill"))]
    pub show_drill: Option<crate::simple_type::BooleanValue>,
    /// printDrill
    #[sdk(attr(qname = ":printDrill"))]
    pub print_drill: Option<crate::simple_type::BooleanValue>,
    /// showMemberPropertyTips
    #[sdk(attr(qname = ":showMemberPropertyTips"))]
    pub show_member_property_tips: Option<crate::simple_type::BooleanValue>,
    /// showDataTips
    #[sdk(attr(qname = ":showDataTips"))]
    pub show_data_tips: Option<crate::simple_type::BooleanValue>,
    /// enableWizard
    #[sdk(attr(qname = ":enableWizard"))]
    pub enable_wizard: Option<crate::simple_type::BooleanValue>,
    /// enableDrill
    #[sdk(attr(qname = ":enableDrill"))]
    pub enable_drill: Option<crate::simple_type::BooleanValue>,
    /// enableFieldProperties
    #[sdk(attr(qname = ":enableFieldProperties"))]
    pub enable_field_properties: Option<crate::simple_type::BooleanValue>,
    /// preserveFormatting
    #[sdk(attr(qname = ":preserveFormatting"))]
    pub preserve_formatting: Option<crate::simple_type::BooleanValue>,
    /// useAutoFormatting
    #[sdk(attr(qname = ":useAutoFormatting"))]
    pub use_auto_formatting: Option<crate::simple_type::BooleanValue>,
    /// pageWrap
    #[sdk(attr(qname = ":pageWrap"))]
    pub page_wrap: Option<crate::simple_type::UInt32Value>,
    /// pageOverThenDown
    #[sdk(attr(qname = ":pageOverThenDown"))]
    pub page_over_then_down: Option<crate::simple_type::BooleanValue>,
    /// subtotalHiddenItems
    #[sdk(attr(qname = ":subtotalHiddenItems"))]
    pub subtotal_hidden_items: Option<crate::simple_type::BooleanValue>,
    /// rowGrandTotals
    #[sdk(attr(qname = ":rowGrandTotals"))]
    pub row_grand_totals: Option<crate::simple_type::BooleanValue>,
    /// colGrandTotals
    #[sdk(attr(qname = ":colGrandTotals"))]
    pub column_grand_totals: Option<crate::simple_type::BooleanValue>,
    /// fieldPrintTitles
    #[sdk(attr(qname = ":fieldPrintTitles"))]
    pub field_print_titles: Option<crate::simple_type::BooleanValue>,
    /// itemPrintTitles
    #[sdk(attr(qname = ":itemPrintTitles"))]
    pub item_print_titles: Option<crate::simple_type::BooleanValue>,
    /// mergeItem
    #[sdk(attr(qname = ":mergeItem"))]
    pub merge_item: Option<crate::simple_type::BooleanValue>,
    /// showDropZones
    #[sdk(attr(qname = ":showDropZones"))]
    pub show_drop_zones: Option<crate::simple_type::BooleanValue>,
    /// createdVersion
    #[sdk(attr(qname = ":createdVersion"))]
    pub created_version: Option<crate::simple_type::ByteValue>,
    /// indent
    #[sdk(attr(qname = ":indent"))]
    pub indent: Option<crate::simple_type::UInt32Value>,
    /// showEmptyRow
    #[sdk(attr(qname = ":showEmptyRow"))]
    pub show_empty_row: Option<crate::simple_type::BooleanValue>,
    /// showEmptyCol
    #[sdk(attr(qname = ":showEmptyCol"))]
    pub show_empty_column: Option<crate::simple_type::BooleanValue>,
    /// showHeaders
    #[sdk(attr(qname = ":showHeaders"))]
    pub show_headers: Option<crate::simple_type::BooleanValue>,
    /// compact
    #[sdk(attr(qname = ":compact"))]
    pub compact: Option<crate::simple_type::BooleanValue>,
    /// outline
    #[sdk(attr(qname = ":outline"))]
    pub outline: Option<crate::simple_type::BooleanValue>,
    /// outlineData
    #[sdk(attr(qname = ":outlineData"))]
    pub outline_data: Option<crate::simple_type::BooleanValue>,
    /// compactData
    #[sdk(attr(qname = ":compactData"))]
    pub compact_data: Option<crate::simple_type::BooleanValue>,
    /// published
    #[sdk(attr(qname = ":published"))]
    pub published: Option<crate::simple_type::BooleanValue>,
    /// gridDropZones
    #[sdk(attr(qname = ":gridDropZones"))]
    pub grid_drop_zones: Option<crate::simple_type::BooleanValue>,
    /// immersive
    #[sdk(attr(qname = ":immersive"))]
    pub stop_immersive_ui: Option<crate::simple_type::BooleanValue>,
    /// multipleFieldFilters
    #[sdk(attr(qname = ":multipleFieldFilters"))]
    pub multiple_field_filters: Option<crate::simple_type::BooleanValue>,
    /// chartFormat
    #[sdk(attr(qname = ":chartFormat"))]
    pub chart_format: Option<crate::simple_type::UInt32Value>,
    /// rowHeaderCaption
    #[sdk(attr(qname = ":rowHeaderCaption"))]
    pub row_header_caption: Option<crate::simple_type::StringValue>,
    /// colHeaderCaption
    #[sdk(attr(qname = ":colHeaderCaption"))]
    pub column_header_caption: Option<crate::simple_type::StringValue>,
    /// fieldListSortAscending
    #[sdk(attr(qname = ":fieldListSortAscending"))]
    pub field_list_sort_ascending: Option<crate::simple_type::BooleanValue>,
    /// mdxSubqueries
    #[sdk(attr(qname = ":mdxSubqueries"))]
    pub mdx_subqueries: Option<crate::simple_type::BooleanValue>,
    /// customListSort
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
  #[sdk(child(office2016, qname = "xr:CT_RevExFuture/xr:xrrftr"))]
  XrXrrftr(std::boxed::Box<RevExFuture>),
  /// Defines the RevExUnsupported Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExUnsupported/xr:xrrUspt"))]
  XrXrrUspt(std::boxed::Box<RevExUnsupported>),
  /// Defines the RevExTrimmed Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExTrimmed/xr:xrrTrim"))]
  XrXrrTrim(std::boxed::Box<RevExTrimmed>),
  /// Defines the RevExRowColumn Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExRowColumn/xr:xrrrc"))]
  XrXrrrc(std::boxed::Box<RevExRowColumn>),
  /// Defines the RevExMove Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExMove/xr:xrrm"))]
  XrXrrm(std::boxed::Box<RevExMove>),
  /// Defines the RevExChangeCell Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExChangeCell/xr:xrrc"))]
  XrXrrc(std::boxed::Box<RevExChangeCell>),
  /// Defines the RevExFormatting Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExFormatting/xr:xrrf"))]
  XrXrrf(std::boxed::Box<RevExFormatting>),
  /// Defines the RevExDefinedName Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExDefinedName/xr:xrrDefName"))]
  XrXrrDefName(std::boxed::Box<RevExDefinedName>),
  /// Defines the RevExDelObj Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExDelObj/xr:xrrdo"))]
  XrXrrdo(std::boxed::Box<RevExDelObj>),
  /// Defines the RevExChgObj Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExChgObj/xr:xrrco"))]
  XrXrrco(std::boxed::Box<RevExChgObj>),
  /// Defines the RevExSheetOp Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExSheetOp/xr:xrrSheet"))]
  XrXrrSheet(std::boxed::Box<RevExSheetOp>),
  /// Defines the RevisionList Class.
  #[sdk(child(office2016, qname = "xr:CT_RevisionList/xr:xrrList"))]
  XrXrrList(std::boxed::Box<RevisionList>),
  /// Defines the RevListAutoExpandRw Class.
  #[sdk(child(office2016, qname = "xr:CT_RevListAutoExpandRw/xr:xrrListExpR"))]
  XrXrrListExpR(std::boxed::Box<RevListAutoExpandRw>),
  /// Defines the RevGroup Class.
  #[sdk(child(office2016, qname = "xr:CT_RevGroup/xr:xrrg"))]
  XrXrrg(std::boxed::Box<RevGroup>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum StateBasedObjectChoice {
  /// Represents an external link to another workbook..
  #[sdk(child(office2016, qname = "x:CT_DataValidation/xr:dataValidation"))]
  XrDataValidation(std::boxed::Box<DataValidation>),
  /// Represents a hyperlink within a cell..
  #[sdk(child(office2016, qname = "x:CT_Hyperlink/xr:hyperlink"))]
  XrHyperlink(std::boxed::Box<Hyperlink>),
  /// Represents a sparkline group of 1 or more sparklines..
  #[sdk(child(office2016, qname = "x14:CT_SparklineGroup/xr:sparklineGroup"))]
  XrSparklineGroup(std::boxed::Box<SparklineGroup>),
  /// Represents one comment within a cell..
  #[sdk(child(office2016, qname = "x:CT_Comments/xr:comments"))]
  XrComments(std::boxed::Box<Comments>),
  /// Represents an autofilter..
  #[sdk(child(office2016, qname = "x:CT_AutoFilter/xr:autoFilter"))]
  XrAutoFilter(std::boxed::Box<AutoFilter>),
  /// Represents a PivotTable View..
  #[sdk(child(
    office2016,
    qname = "x:CT_pivotTableDefinition/xr:pivotTableDefinition"
  ))]
  XrPivotTableDefinition(std::boxed::Box<PivotTableDefinition>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RevExChgObjChoice {
  #[sdk(child(office2016, qname = "xr:CT_RevisionStateLink/xr:link"))]
  XrLink(std::boxed::Box<RevisionStateLink>),
  #[sdk(child(office2016, qname = "xr:CT_RevisionState/xr:body"))]
  XrBody(std::boxed::Box<RevisionState>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RevGroupChoice {
  /// Defines the RevExFuture Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExFuture/xr:xrrftr"))]
  XrXrrftr(std::boxed::Box<RevExFuture>),
  /// Defines the RevExUnsupported Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExUnsupported/xr:xrrUspt"))]
  XrXrrUspt(std::boxed::Box<RevExUnsupported>),
  /// Defines the RevExTrimmed Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExTrimmed/xr:xrrTrim"))]
  XrXrrTrim(std::boxed::Box<RevExTrimmed>),
  /// Defines the RevExRowColumn Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExRowColumn/xr:xrrrc"))]
  XrXrrrc(std::boxed::Box<RevExRowColumn>),
  /// Defines the RevExMove Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExMove/xr:xrrm"))]
  XrXrrm(std::boxed::Box<RevExMove>),
  /// Defines the RevExChangeCell Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExChangeCell/xr:xrrc"))]
  XrXrrc(std::boxed::Box<RevExChangeCell>),
  /// Defines the RevExFormatting Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExFormatting/xr:xrrf"))]
  XrXrrf(std::boxed::Box<RevExFormatting>),
  /// Defines the RevExDefinedName Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExDefinedName/xr:xrrDefName"))]
  XrXrrDefName(std::boxed::Box<RevExDefinedName>),
  /// Defines the RevExDelObj Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExDelObj/xr:xrrdo"))]
  XrXrrdo(std::boxed::Box<RevExDelObj>),
  /// Defines the RevExChgObj Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExChgObj/xr:xrrco"))]
  XrXrrco(std::boxed::Box<RevExChgObj>),
  /// Defines the RevExSheetOp Class.
  #[sdk(child(office2016, qname = "xr:CT_RevExSheetOp/xr:xrrSheet"))]
  XrXrrSheet(std::boxed::Box<RevExSheetOp>),
  /// Defines the RevisionList Class.
  #[sdk(child(office2016, qname = "xr:CT_RevisionList/xr:xrrList"))]
  XrXrrList(std::boxed::Box<RevisionList>),
  /// Defines the RevListAutoExpandRw Class.
  #[sdk(child(office2016, qname = "xr:CT_RevListAutoExpandRw/xr:xrrListExpR"))]
  XrXrrListExpR(std::boxed::Box<RevListAutoExpandRw>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RevisionStateChoice {
  /// Defines the RowColVisualOps Class.
  #[sdk(child(office2016, qname = "xr:CT_RowColVisualOps/xr:rowColVisualOps"))]
  XrRowColVisualOps(std::boxed::Box<RowColVisualOps>),
  /// Defines the HideUnhideSheet Class.
  #[sdk(child(office2016, qname = "xr:CT_HideUnhideSheet/xr:hideUnhideSheet"))]
  XrHideUnhideSheet(std::boxed::Box<HideUnhideSheet>),
  /// Defines the ShowGridlinesHeadings Class.
  #[sdk(child(
    office2016,
    qname = "xr:CT_ShowGridlinesHeadings/xr:showGridlinesHeadings"
  ))]
  XrShowGridlinesHeadings(std::boxed::Box<ShowGridlinesHeadings>),
  /// Defines the FreezePanes Class.
  #[sdk(child(office2016, qname = "xr:CT_FreezePanes/xr:freezePanes"))]
  XrFreezePanes(std::boxed::Box<FreezePanes>),
  /// Defines the Outlines Class.
  #[sdk(child(office2016, qname = "xr:CT_Outlines/xr:outlines"))]
  XrOutlines(std::boxed::Box<Outlines>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RefMapChoice {
  /// Defines the RefCell Class.
  #[sdk(child(office2016, qname = "xr:CT_RefCell/xr:ref"))]
  XrRef(std::boxed::Box<RefCell>),
  /// Defines the SheetXluid Class.
  #[sdk(child(office2016, qname = "xr:CT_SheetXluid/xr:sheetUid"))]
  XrSheetUid(std::boxed::Box<SheetXluid>),
  /// Defines the RefOartAnchor Class.
  #[sdk(child(office2016, qname = "xr:CT_RefOartAnchor/xr:oartAnchor"))]
  XrOartAnchor(std::boxed::Box<RefOartAnchor>),
  /// Defines the RefFuture Class.
  #[sdk(empty_child(office2016, qname = "xr:CT_RefFuture/xr:future"))]
  XrFuture,
  /// Defines the RefTest Class.
  #[sdk(child(office2016, qname = "xr:CT_RefTest/xr:test"))]
  XrTest(std::boxed::Box<RefTest>),
}
