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
#[sdk(qname = "xr:revHdrs")]
pub struct RevExHeaders {
  /// minRev
  #[sdk(attr(qname = ":minRev"))]
  pub min_rev: crate::simple_type::UInt64Value,
  /// maxRev
  #[sdk(attr(qname = ":maxRev"))]
  pub max_rev: crate::simple_type::UInt64Value,
  /// docId
  #[sdk(attr(qname = ":docId"))]
  pub doc_id: crate::simple_type::StringValue,
  /// endpointId
  #[sdk(attr(qname = ":endpointId"))]
  pub endpoint_id: crate::simple_type::StringValue,
  /// Defines the RevExHeader Class.
  #[sdk(child(qname = "xr:hdr"))]
  pub rev_ex_header: Vec<RevExHeader>,
}
/// Defines the RevExStream Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:revStream")]
pub struct RevExStream {
  #[sdk(
        choice(
            child(variant = RevExFuture, qname = "xr:xrrftr"),
            child(variant = RevExUnsupported, qname = "xr:xrrUspt"),
            child(variant = RevExTrimmed, qname = "xr:xrrTrim"),
            child(variant = RevExRowColumn, qname = "xr:xrrrc"),
            child(variant = RevExMove, qname = "xr:xrrm"),
            child(variant = RevExChangeCell, qname = "xr:xrrc"),
            child(variant = RevExFormatting, qname = "xr:xrrf"),
            child(variant = RevExDefinedName, qname = "xr:xrrDefName"),
            child(variant = RevExDelObj, qname = "xr:xrrdo"),
            child(variant = RevExChgObj, qname = "xr:xrrco"),
            child(variant = RevExSheetOp, qname = "xr:xrrSheet"),
            child(variant = RevisionList, qname = "xr:xrrList"),
            child(variant = RevListAutoExpandRw, qname = "xr:xrrListExpR"),
            child(variant = RevGroup, qname = "xr:xrrg")
        )
    )]
  pub rev_ex_stream_choice: Vec<RevExStreamChoice>,
}
/// Defines the DifferentialFormatType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:dxf")]
pub struct DifferentialFormatType {
  /// Font Properties
  #[sdk(child(qname = "x:font"))]
  pub font: Option<crate::schemas::x::Font>,
  /// Number Format
  #[sdk(child(qname = "x:numFmt"))]
  pub numbering_format: Option<crate::schemas::x::NumberingFormat>,
  /// Fill
  #[sdk(child(qname = "x:fill"))]
  pub fill: Option<std::boxed::Box<crate::schemas::x::Fill>>,
  /// Alignment
  #[sdk(child(qname = "x:alignment"))]
  pub alignment: Option<crate::schemas::x::Alignment>,
  /// Border Properties
  #[sdk(child(qname = "x:border"))]
  pub border: Option<std::boxed::Box<crate::schemas::x::Border>>,
  /// Protection Properties
  #[sdk(child(qname = "x:protection"))]
  pub protection: Option<crate::schemas::x::Protection>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:extLst"))]
  pub extension_list: Option<crate::schemas::x::ExtensionList>,
}
/// Defines the RevisionPtr Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:revisionPtr")]
pub struct RevisionPtr {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// revIDLastSave
  #[sdk(attr(qname = ":revIDLastSave"))]
  pub rev_id_last_save: crate::simple_type::UInt64Value,
  /// documentId
  #[sdk(attr(qname = ":documentId"))]
  pub document_id: crate::simple_type::StringValue,
}
/// Defines the StateBasedObject Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:objectState")]
pub struct StateBasedObject {
  #[sdk(
        choice(
            child(variant = DataValidation, qname = "xr:dataValidation"),
            child(variant = Hyperlink, qname = "xr:hyperlink"),
            child(variant = SparklineGroup, qname = "xr:sparklineGroup"),
            child(variant = Comments, qname = "xr:comments"),
            child(variant = AutoFilter, qname = "xr:autoFilter"),
            child(variant = PivotTableDefinition, qname = "xr:pivotTableDefinition")
        )
    )]
  pub state_based_object_choice: Option<StateBasedObjectChoice>,
}
/// Defines the RevExHeader Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:hdr")]
pub struct RevExHeader {
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
  /// minRev
  #[sdk(attr(qname = ":minRev"))]
  pub min_rev: Option<crate::simple_type::UInt64Value>,
  /// maxRev
  #[sdk(attr(qname = ":maxRev"))]
  pub max_rev: Option<crate::simple_type::UInt64Value>,
  /// time
  #[sdk(attr(qname = ":time"))]
  pub time: crate::simple_type::DateTimeValue,
}
/// Defines the RevExFuture Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:xrrftr")]
pub struct RevExFuture {
  /// rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// sti
  #[sdk(attr(qname = ":sti"))]
  pub sti: Option<crate::simple_type::BooleanValue>,
  /// Defines the RevExTest Class.
  #[sdk(empty_child(qname = "xr:xrrtest"))]
  pub rev_ex_test: Vec<()>,
}
/// Defines the RevExUnsupported Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:xrrUspt")]
pub struct RevExUnsupported {
  /// rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
}
/// Defines the RevExTrimmed Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:xrrTrim")]
pub struct RevExTrimmed {
  /// rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
}
/// Defines the RevExRowColumn Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:xrrrc")]
pub struct RevExRowColumn {
  /// rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// eol
  #[sdk(attr(qname = ":eol"))]
  pub eol: Option<crate::simple_type::BooleanValue>,
  /// ref
  #[sdk(attr(qname = ":ref"))]
  pub r#ref: crate::simple_type::StringValue,
  /// action
  #[sdk(attr(qname = ":action"))]
  pub action: RwColAction,
}
/// Defines the RevExMove Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:xrrm")]
pub struct RevExMove {
  /// rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// src
  #[sdk(attr(qname = ":src"))]
  pub src: crate::simple_type::StringValue,
  /// dst
  #[sdk(attr(qname = ":dst"))]
  pub dst: crate::simple_type::StringValue,
  /// srcSh
  #[sdk(attr(qname = ":srcSh"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub src_sh: Option<crate::simple_type::StringValue>,
}
/// Defines the RevExChangeCell Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:xrrc")]
pub struct RevExChangeCell {
  /// listUid
  #[sdk(attr(qname = ":listUid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub list_uid: Option<crate::simple_type::StringValue>,
  /// rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// r
  #[sdk(attr(qname = ":r"))]
  pub r: crate::simple_type::StringValue,
  /// t
  #[sdk(attr(qname = ":t"))]
  pub t: Option<FillType>,
  /// x
  #[sdk(attr(qname = ":x"))]
  pub x: Option<FillTypeExt>,
  /// w
  #[sdk(attr(qname = ":w"))]
  pub w: Option<crate::simple_type::UInt32Value>,
  /// Defines the RevCell Class.
  #[sdk(child(qname = "xr:c"))]
  pub rev_cell: Vec<RevCell>,
  /// Defines the ChangeCellSubEdit Class.
  #[sdk(child(qname = "xr:ccse"))]
  pub change_cell_sub_edit: Vec<ChangeCellSubEdit>,
}
/// Defines the RevExFormatting Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:xrrf")]
pub struct RevExFormatting {
  /// rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// numFmtId
  #[sdk(attr(qname = ":numFmtId"))]
  pub num_fmt_id: Option<crate::simple_type::UInt32Value>,
  /// xfDxf
  #[sdk(attr(qname = ":xfDxf"))]
  pub xf_dxf: Option<crate::simple_type::BooleanValue>,
  /// style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::BooleanValue>,
  /// sqref
  #[sdk(attr(list, qname = ":sqref"))]
  pub sqref: Vec<crate::simple_type::StringValue>,
  /// start
  #[sdk(attr(qname = ":start"))]
  pub start: Option<crate::simple_type::UInt32Value>,
  /// length
  #[sdk(attr(qname = ":length"))]
  pub length: Option<crate::simple_type::UInt32Value>,
  /// styleUid
  #[sdk(attr(qname = ":styleUid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub style_uid: Option<crate::simple_type::StringValue>,
  /// fBlankCell
  #[sdk(attr(qname = ":fBlankCell"))]
  pub f_blank_cell: Option<crate::simple_type::BooleanValue>,
  /// applyNumberFormat
  #[sdk(attr(qname = ":applyNumberFormat"))]
  pub apply_number_format: Option<crate::simple_type::BooleanValue>,
  /// applyFont
  #[sdk(attr(qname = ":applyFont"))]
  pub apply_font: Option<crate::simple_type::BooleanValue>,
  /// applyFill
  #[sdk(attr(qname = ":applyFill"))]
  pub apply_fill: Option<crate::simple_type::BooleanValue>,
  /// applyBorder
  #[sdk(attr(qname = ":applyBorder"))]
  pub apply_border: Option<crate::simple_type::BooleanValue>,
  /// applyAlignment
  #[sdk(attr(qname = ":applyAlignment"))]
  pub apply_alignment: Option<crate::simple_type::BooleanValue>,
  /// applyProtection
  #[sdk(attr(qname = ":applyProtection"))]
  pub apply_protection: Option<crate::simple_type::BooleanValue>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(qname = "xr:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "xr:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the RevExDefinedName Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:xrrDefName")]
pub struct RevExDefinedName {
  /// rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// customView
  #[sdk(attr(qname = ":customView"))]
  pub custom_view: Option<crate::simple_type::BooleanValue>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// function
  #[sdk(attr(qname = ":function"))]
  pub function: Option<crate::simple_type::BooleanValue>,
  /// functionGroupId
  #[sdk(attr(qname = ":functionGroupId"))]
  pub function_group_id: Option<crate::simple_type::ByteValue>,
  /// shortcutKey
  #[sdk(attr(qname = ":shortcutKey"))]
  pub shortcut_key: Option<crate::simple_type::ByteValue>,
  /// hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// customMenu
  #[sdk(attr(qname = ":customMenu"))]
  pub custom_menu: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(qname = ":description"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// help
  #[sdk(attr(qname = ":help"))]
  pub help: Option<crate::simple_type::StringValue>,
  /// statusBar
  #[sdk(attr(qname = ":statusBar"))]
  pub status_bar: Option<crate::simple_type::StringValue>,
  /// comment
  #[sdk(attr(qname = ":comment"))]
  pub comment: Option<crate::simple_type::StringValue>,
  /// Defines the FormulaFormula Class.
  #[sdk(text_child(qname = "xr:formula"))]
  pub formula_formula: Option<FormulaFormula>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "xr:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the RevExDelObj Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:xrrdo")]
pub struct RevExDelObj {
  /// rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// Defines the StateBasedHeader Class.
  #[sdk(child(qname = "xr:hdr"))]
  pub state_based_header: std::boxed::Box<StateBasedHeader>,
}
/// Defines the RevExChgObj Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:xrrco")]
pub struct RevExChgObj {
  /// rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// Defines the StateBasedHeader Class.
  #[sdk(child(qname = "xr:hdr"))]
  pub state_based_header: std::boxed::Box<StateBasedHeader>,
  #[sdk(
        choice(
            child(variant = RevisionStateLink, qname = "xr:link"),
            child(variant = RevisionState, qname = "xr:body")
        )
    )]
  pub rev_ex_chg_obj_choice: Option<RevExChgObjChoice>,
}
/// Defines the RevExSheetOp Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:xrrSheet")]
pub struct RevExSheetOp {
  /// rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// op
  #[sdk(attr(qname = ":op"))]
  pub op: SheetOp,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// idOrig
  #[sdk(attr(qname = ":idOrig"))]
  pub id_orig: Option<crate::simple_type::UInt32Value>,
  /// idNew
  #[sdk(attr(qname = ":idNew"))]
  pub id_new: Option<crate::simple_type::UInt32Value>,
}
/// Defines the RevisionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:xrrList")]
pub struct RevisionList {
  /// rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// Data
  #[sdk(attr(qname = ":Data"))]
  pub data: Option<crate::simple_type::BooleanValue>,
  /// Formatting
  #[sdk(attr(qname = ":Formatting"))]
  pub formatting: Option<crate::simple_type::BooleanValue>,
  /// RangeBased
  #[sdk(attr(qname = ":RangeBased"))]
  pub range_based: Option<crate::simple_type::BooleanValue>,
  /// Fake
  #[sdk(attr(qname = ":Fake"))]
  pub fake: Option<crate::simple_type::BooleanValue>,
  /// ref
  #[sdk(attr(qname = ":ref"))]
  pub r#ref: crate::simple_type::StringValue,
  /// Headers
  #[sdk(attr(qname = ":Headers"))]
  pub headers: Option<crate::simple_type::BooleanValue>,
  /// InsDelHeaders
  #[sdk(attr(qname = ":InsDelHeaders"))]
  pub ins_del_headers: Option<crate::simple_type::BooleanValue>,
  /// rId
  #[sdk(attr(qname = ":rId"))]
  pub r_id: crate::simple_type::UInt32Value,
}
/// Defines the RevListAutoExpandRw Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:xrrListExpR")]
pub struct RevListAutoExpandRw {
  /// rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  /// refAdded
  #[sdk(attr(qname = ":refAdded"))]
  pub ref_added: crate::simple_type::StringValue,
  /// listGuid
  #[sdk(attr(qname = ":listGuid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub list_guid: crate::simple_type::StringValue,
}
/// Defines the RevGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:xrrg")]
pub struct RevGroup {
  /// rev
  #[sdk(attr(qname = ":rev"))]
  pub rev: crate::simple_type::UInt64Value,
  /// uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// sh
  #[sdk(attr(qname = ":sh"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub sh: crate::simple_type::StringValue,
  /// uidp
  #[sdk(attr(qname = ":uidp"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uidp: Option<crate::simple_type::StringValue>,
  /// ctx
  #[sdk(attr(qname = ":ctx"))]
  pub ctx: Option<RevisionContext>,
  #[sdk(
        choice(
            child(variant = RevExFuture, qname = "xr:xrrftr"),
            child(variant = RevExUnsupported, qname = "xr:xrrUspt"),
            child(variant = RevExTrimmed, qname = "xr:xrrTrim"),
            child(variant = RevExRowColumn, qname = "xr:xrrrc"),
            child(variant = RevExMove, qname = "xr:xrrm"),
            child(variant = RevExChangeCell, qname = "xr:xrrc"),
            child(variant = RevExFormatting, qname = "xr:xrrf"),
            child(variant = RevExDefinedName, qname = "xr:xrrDefName"),
            child(variant = RevExDelObj, qname = "xr:xrrdo"),
            child(variant = RevExChgObj, qname = "xr:xrrco"),
            child(variant = RevExSheetOp, qname = "xr:xrrSheet"),
            child(variant = RevisionList, qname = "xr:xrrList"),
            child(variant = RevListAutoExpandRw, qname = "xr:xrrListExpR")
        )
    )]
  pub rev_group_choice: Vec<RevGroupChoice>,
}
/// Defines the RevCell Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:c")]
pub struct RevCell {
  /// t
  #[sdk(attr(qname = ":t"))]
  pub t: Option<crate::schemas::x::CellValues>,
  /// nop
  #[sdk(attr(qname = ":nop"))]
  pub nop: Option<crate::simple_type::BooleanValue>,
  /// tick
  #[sdk(attr(qname = ":tick"))]
  pub tick: Option<crate::simple_type::BooleanValue>,
  /// rep
  #[sdk(attr(qname = ":rep"))]
  pub rep: Option<crate::simple_type::UInt32Value>,
  /// Defines the FFormula Class.
  #[sdk(text_child(qname = "xr:f"))]
  pub f_formula: Option<FFormula>,
  /// Defines the Xstring Class.
  #[sdk(text_child(qname = "xr:v"))]
  pub xstring: Option<Xstring>,
  /// Defines the RstType Class.
  #[sdk(child(qname = "xr:is"))]
  pub rst_type: Option<std::boxed::Box<RstType>>,
}
/// Defines the ChangeCellSubEdit Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:ccse")]
pub struct ChangeCellSubEdit {
  /// r
  #[sdk(attr(qname = ":r"))]
  pub r: crate::simple_type::StringValue,
  /// t
  #[sdk(attr(qname = ":t"))]
  pub t: Option<FillType>,
  /// x
  #[sdk(attr(qname = ":x"))]
  pub x: Option<FillTypeExt>,
  /// w
  #[sdk(attr(qname = ":w"))]
  pub w: Option<crate::simple_type::UInt32Value>,
  /// Defines the RevCell Class.
  #[sdk(child(qname = "xr:c"))]
  pub rev_cell: Vec<RevCell>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "x:ext"))]
  pub extension: Vec<crate::schemas::x::Extension>,
}
/// Defines the FormulaFormula Class.
pub type FormulaFormula = crate::simple_type::StringValue;
/// Defines the FFormula Class.
pub type FFormula = crate::simple_type::StringValue;
/// Defines the StateBasedHeader Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:hdr")]
pub struct StateBasedHeader {
  /// uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid: crate::simple_type::StringValue,
  /// eft
  #[sdk(attr(qname = ":eft"))]
  pub eft: FeatureType,
  /// eftx
  #[sdk(attr(qname = ":eftx"))]
  pub eftx: Option<ExtFeatureType>,
  /// seft
  #[sdk(attr(qname = ":seft"))]
  pub seft: Option<SubFeatureType>,
  /// seftx
  #[sdk(attr(qname = ":seftx"))]
  pub seftx: Option<ExtSubFeatureType>,
  /// Defines the RefMap Class.
  #[sdk(child(qname = "xr:refmap"))]
  pub ref_map: Option<RefMap>,
}
/// Defines the RevisionStateLink Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:link")]
pub struct RevisionStateLink {
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the RevisionState Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:body")]
pub struct RevisionState {
  #[sdk(
        choice(
            child(variant = RowColVisualOps, qname = "xr:rowColVisualOps"),
            child(variant = HideUnhideSheet, qname = "xr:hideUnhideSheet"),
            child(variant = ShowGridlinesHeadings, qname = "xr:showGridlinesHeadings"),
            child(variant = FreezePanes, qname = "xr:freezePanes"),
            child(variant = Outlines, qname = "xr:outlines")
        )
    )]
  pub revision_state_choice: Option<RevisionStateChoice>,
}
/// Defines the RefMap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:refmap")]
pub struct RefMap {
  #[sdk(
        choice(
            child(variant = RefCell, qname = "xr:ref"),
            child(variant = SheetXluid, qname = "xr:sheetUid"),
            child(variant = RefOartAnchor, qname = "xr:oartAnchor"),
            empty_child(variant = RefFuture, qname = "xr:future"),
            child(variant = RefTest, qname = "xr:test")
        )
    )]
  pub ref_map_choice: Vec<RefMapChoice>,
}
/// Defines the RowColVisualOps Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:rowColVisualOps")]
pub struct RowColVisualOps {
  /// action
  #[sdk(attr(qname = ":action"))]
  pub action: RowColVisualOp,
  /// isRow
  #[sdk(attr(qname = ":isRow"))]
  pub is_row: crate::simple_type::BooleanValue,
  /// size
  #[sdk(attr(qname = ":size"))]
  pub size: Option<crate::simple_type::UInt32Value>,
  /// userSized
  #[sdk(attr(qname = ":userSized"))]
  pub user_sized: Option<crate::simple_type::BooleanValue>,
}
/// Defines the HideUnhideSheet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:hideUnhideSheet")]
pub struct HideUnhideSheet {
  /// hide
  #[sdk(attr(qname = ":hide"))]
  pub hide: crate::simple_type::BooleanValue,
}
/// Defines the ShowGridlinesHeadings Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:showGridlinesHeadings")]
pub struct ShowGridlinesHeadings {
  /// showGridLines
  #[sdk(attr(qname = ":showGridLines"))]
  pub show_grid_lines: crate::simple_type::BooleanValue,
  /// showRowCol
  #[sdk(attr(qname = ":showRowCol"))]
  pub show_row_col: crate::simple_type::BooleanValue,
}
/// Defines the FreezePanes Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:freezePanes")]
pub struct FreezePanes {
  /// sheetViewUid
  #[sdk(attr(qname = ":sheetViewUid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub sheet_view_uid: Option<crate::simple_type::StringValue>,
}
/// Defines the Outlines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:outlines")]
pub struct Outlines {
  /// isRow
  #[sdk(attr(qname = ":isRow"))]
  pub is_row: crate::simple_type::BooleanValue,
  /// Defines the Outline Class.
  #[sdk(child(qname = "xr:outline"))]
  pub outline: Vec<Outline>,
}
/// Defines the Outline Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:outline")]
pub struct Outline {
  /// isCollapsed
  #[sdk(attr(qname = ":isCollapsed"))]
  pub is_collapsed: crate::simple_type::BooleanValue,
  /// level
  #[sdk(attr(qname = ":level"))]
  pub level: crate::simple_type::ByteValue,
}
/// Defines the Xstring Class.
pub type Xstring = crate::simple_type::StringValue;
/// Defines the RstType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:is")]
pub struct RstType {
  /// Text
  #[sdk(child(qname = "x:t"))]
  pub text: Option<crate::schemas::x::Text>,
  /// Rich Text Run.
  #[sdk(child(qname = "x:r"))]
  pub run: Vec<crate::schemas::x::Run>,
  /// Phonetic Run.
  #[sdk(child(qname = "x:rPh"))]
  pub phonetic_run: Vec<crate::schemas::x::PhoneticRun>,
  /// Phonetic Properties.
  #[sdk(child(qname = "x:phoneticPr"))]
  pub phonetic_properties: Option<crate::schemas::x::PhoneticProperties>,
}
/// Defines the RefCell Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:ref")]
pub struct RefCell {
  /// n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// ajt
  #[sdk(attr(qname = ":ajt"))]
  pub ajt: AdjustType,
  /// ajtx
  #[sdk(attr(qname = ":ajtx"))]
  pub ajtx: Option<AdjustTypeExt>,
  /// homeRef
  #[sdk(attr(qname = ":homeRef"))]
  pub home_ref: Option<crate::simple_type::BooleanValue>,
  /// r
  #[sdk(attr(list, qname = ":r"))]
  pub r: Vec<crate::simple_type::StringValue>,
  /// uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid: Option<crate::simple_type::StringValue>,
  /// uidLast
  #[sdk(attr(qname = ":uidLast"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid_last: Option<crate::simple_type::StringValue>,
}
/// Defines the SheetXluid Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:sheetUid")]
pub struct SheetXluid {
  /// n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// ajt
  #[sdk(attr(qname = ":ajt"))]
  pub ajt: AdjustType,
  /// ajtx
  #[sdk(attr(qname = ":ajtx"))]
  pub ajtx: Option<AdjustTypeExt>,
  /// homeRef
  #[sdk(attr(qname = ":homeRef"))]
  pub home_ref: Option<crate::simple_type::BooleanValue>,
  /// uid
  #[sdk(attr(qname = ":uid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub uid: crate::simple_type::StringValue,
}
/// Defines the RefOartAnchor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:oartAnchor")]
pub struct RefOartAnchor {
  /// n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// ajt
  #[sdk(attr(qname = ":ajt"))]
  pub ajt: AdjustType,
  /// ajtx
  #[sdk(attr(qname = ":ajtx"))]
  pub ajtx: Option<AdjustTypeExt>,
  /// homeRef
  #[sdk(attr(qname = ":homeRef"))]
  pub home_ref: Option<crate::simple_type::BooleanValue>,
  /// r
  #[sdk(attr(qname = ":r"))]
  pub r: Option<crate::simple_type::StringValue>,
  /// fromRowOff
  #[sdk(attr(qname = ":fromRowOff"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub from_row_off: Option<crate::simple_type::Int64Value>,
  /// fromColOff
  #[sdk(attr(qname = ":fromColOff"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub from_col_off: Option<crate::simple_type::Int64Value>,
  /// toRowOff
  #[sdk(attr(qname = ":toRowOff"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub to_row_off: Option<crate::simple_type::Int64Value>,
  /// toColOff
  #[sdk(attr(qname = ":toColOff"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub to_col_off: Option<crate::simple_type::Int64Value>,
  /// cx
  #[sdk(attr(qname = ":cx"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub cx: Option<crate::simple_type::Int64Value>,
  /// cy
  #[sdk(attr(qname = ":cy"))]
  #[sdk(number_range(range = 0..= 2147483647))]
  pub cy: Option<crate::simple_type::Int64Value>,
  /// x
  #[sdk(attr(qname = ":x"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub x: Option<crate::simple_type::Int64Value>,
  /// y
  #[sdk(attr(qname = ":y"))]
  #[sdk(number_range(range = -27273042329600..= 27273042316900))]
  pub y: Option<crate::simple_type::Int64Value>,
  /// oat
  #[sdk(attr(qname = ":oat"))]
  pub oat: OartAnchorType,
}
/// Defines the RefTest Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:test")]
pub struct RefTest {
  /// n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// ajt
  #[sdk(attr(qname = ":ajt"))]
  pub ajt: AdjustType,
  /// ajtx
  #[sdk(attr(qname = ":ajtx"))]
  pub ajtx: Option<AdjustTypeExt>,
  /// homeRef
  #[sdk(attr(qname = ":homeRef"))]
  pub home_ref: Option<crate::simple_type::BooleanValue>,
}
/// Represents an external link to another workbook..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:dataValidation")]
pub struct DataValidation {
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<crate::schemas::x::DataValidationValues>,
  /// errorStyle
  #[sdk(attr(qname = ":errorStyle"))]
  pub error_style: Option<crate::schemas::x::DataValidationErrorStyleValues>,
  /// imeMode
  #[sdk(attr(qname = ":imeMode"))]
  pub ime_mode: Option<crate::schemas::x::DataValidationImeModeValues>,
  /// operator
  #[sdk(attr(qname = ":operator"))]
  pub operator: Option<crate::schemas::x::DataValidationOperatorValues>,
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
  #[sdk(attr(list, qname = ":sqref"))]
  pub sequence_of_references: Vec<crate::simple_type::StringValue>,
  /// Defines the List Class.
  #[sdk(text_child(qname = "x12ac:list"))]
  pub list: Option<crate::schemas::x12ac::List>,
  /// Defines the Formula1 Class.
  #[sdk(child(qname = "x:formula1"))]
  pub formula1: Option<crate::schemas::x::Formula1>,
  /// Defines the Formula2 Class.
  #[sdk(child(qname = "x:formula2"))]
  pub formula2: Option<crate::schemas::x::Formula2>,
}
/// Represents a hyperlink within a cell..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:hyperlink")]
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
#[sdk(qname = "xr:sparklineGroup")]
pub struct SparklineGroup {
  /// manualMax
  #[sdk(attr(qname = ":manualMax"))]
  pub manual_max: Option<crate::simple_type::DoubleValue>,
  /// manualMin
  #[sdk(attr(qname = ":manualMin"))]
  pub manual_min: Option<crate::simple_type::DoubleValue>,
  /// lineWeight
  #[sdk(attr(qname = ":lineWeight"))]
  pub line_weight: Option<crate::simple_type::DoubleValue>,
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<crate::schemas::x14::SparklineTypeValues>,
  /// dateAxis
  #[sdk(attr(qname = ":dateAxis"))]
  pub date_axis: Option<crate::simple_type::BooleanValue>,
  /// displayEmptyCellsAs
  #[sdk(attr(qname = ":displayEmptyCellsAs"))]
  pub display_empty_cells_as: Option<crate::schemas::x14::DisplayBlanksAsValues>,
  /// markers
  #[sdk(attr(qname = ":markers"))]
  pub markers: Option<crate::simple_type::BooleanValue>,
  /// high
  #[sdk(attr(qname = ":high"))]
  pub high: Option<crate::simple_type::BooleanValue>,
  /// low
  #[sdk(attr(qname = ":low"))]
  pub low: Option<crate::simple_type::BooleanValue>,
  /// first
  #[sdk(attr(qname = ":first"))]
  pub first: Option<crate::simple_type::BooleanValue>,
  /// last
  #[sdk(attr(qname = ":last"))]
  pub last: Option<crate::simple_type::BooleanValue>,
  /// negative
  #[sdk(attr(qname = ":negative"))]
  pub negative: Option<crate::simple_type::BooleanValue>,
  /// displayXAxis
  #[sdk(attr(qname = ":displayXAxis"))]
  pub display_x_axis: Option<crate::simple_type::BooleanValue>,
  /// displayHidden
  #[sdk(attr(qname = ":displayHidden"))]
  pub display_hidden: Option<crate::simple_type::BooleanValue>,
  /// minAxisType
  #[sdk(attr(qname = ":minAxisType"))]
  pub min_axis_type: Option<crate::schemas::x14::SparklineAxisMinMaxValues>,
  /// maxAxisType
  #[sdk(attr(qname = ":maxAxisType"))]
  pub max_axis_type: Option<crate::schemas::x14::SparklineAxisMinMaxValues>,
  /// rightToLeft
  #[sdk(attr(qname = ":rightToLeft"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// Defines the SeriesColor Class.
  #[sdk(child(qname = "x14:colorSeries"))]
  pub series_color: Option<crate::schemas::x14::SeriesColor>,
  /// Defines the NegativeColor Class.
  #[sdk(child(qname = "x14:colorNegative"))]
  pub negative_color: Option<crate::schemas::x14::NegativeColor>,
  /// Defines the AxisColor Class.
  #[sdk(child(qname = "x14:colorAxis"))]
  pub axis_color: Option<crate::schemas::x14::AxisColor>,
  /// Defines the MarkersColor Class.
  #[sdk(child(qname = "x14:colorMarkers"))]
  pub markers_color: Option<crate::schemas::x14::MarkersColor>,
  /// Defines the FirstMarkerColor Class.
  #[sdk(child(qname = "x14:colorFirst"))]
  pub first_marker_color: Option<crate::schemas::x14::FirstMarkerColor>,
  /// Defines the LastMarkerColor Class.
  #[sdk(child(qname = "x14:colorLast"))]
  pub last_marker_color: Option<crate::schemas::x14::LastMarkerColor>,
  /// Defines the HighMarkerColor Class.
  #[sdk(child(qname = "x14:colorHigh"))]
  pub high_marker_color: Option<crate::schemas::x14::HighMarkerColor>,
  /// Defines the LowMarkerColor Class.
  #[sdk(child(qname = "x14:colorLow"))]
  pub low_marker_color: Option<crate::schemas::x14::LowMarkerColor>,
  /// Defines the Formula Class.
  #[sdk(text_child(qname = "xne:f"))]
  pub formula: Option<crate::schemas::xne::Formula>,
  /// Defines the Sparklines Class.
  #[sdk(child(qname = "x14:sparklines"))]
  pub sparklines: crate::schemas::x14::Sparklines,
}
/// Represents one comment within a cell..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:comments")]
pub struct Comments {
  /// Authors
  #[sdk(child(qname = "x:authors"))]
  pub authors: crate::schemas::x::Authors,
  /// List of Comments
  #[sdk(child(qname = "x:commentList"))]
  pub comment_list: crate::schemas::x::CommentList,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:extLst"))]
  pub extension_list: Option<crate::schemas::x::ExtensionList>,
}
/// Represents an autofilter..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:autoFilter")]
pub struct AutoFilter {
  /// Cell or Range Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: Option<crate::simple_type::StringValue>,
  /// AutoFilter Column.
  #[sdk(child(qname = "x:filterColumn"))]
  pub filter_column: Vec<crate::schemas::x::FilterColumn>,
  /// Sort State for Auto Filter.
  #[sdk(child(qname = "x:sortState"))]
  pub sort_state: Option<std::boxed::Box<crate::schemas::x::SortState>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:extLst"))]
  pub extension_list: Option<crate::schemas::x::ExtensionList>,
}
/// Represents a PivotTable View..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xr:pivotTableDefinition")]
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
  /// Defines the Location Class.
  #[sdk(child(qname = "x:location"))]
  pub location: std::boxed::Box<crate::schemas::x::Location>,
  /// Defines the PivotFields Class.
  #[sdk(child(qname = "x:pivotFields"))]
  pub pivot_fields: Option<crate::schemas::x::PivotFields>,
  /// Defines the RowFields Class.
  #[sdk(child(qname = "x:rowFields"))]
  pub row_fields: Option<crate::schemas::x::RowFields>,
  /// Defines the RowItems Class.
  #[sdk(child(qname = "x:rowItems"))]
  pub row_items: Option<crate::schemas::x::RowItems>,
  /// Defines the ColumnFields Class.
  #[sdk(child(qname = "x:colFields"))]
  pub column_fields: Option<crate::schemas::x::ColumnFields>,
  /// Defines the ColumnItems Class.
  #[sdk(child(qname = "x:colItems"))]
  pub column_items: Option<crate::schemas::x::ColumnItems>,
  /// Defines the PageFields Class.
  #[sdk(child(qname = "x:pageFields"))]
  pub page_fields: Option<crate::schemas::x::PageFields>,
  /// Defines the DataFields Class.
  #[sdk(child(qname = "x:dataFields"))]
  pub data_fields: Option<crate::schemas::x::DataFields>,
  /// Defines the Formats Class.
  #[sdk(child(qname = "x:formats"))]
  pub formats: Option<crate::schemas::x::Formats>,
  /// Defines the ConditionalFormats Class.
  #[sdk(child(qname = "x:conditionalFormats"))]
  pub conditional_formats: Option<crate::schemas::x::ConditionalFormats>,
  /// Defines the ChartFormats Class.
  #[sdk(child(qname = "x:chartFormats"))]
  pub chart_formats: Option<crate::schemas::x::ChartFormats>,
  /// Defines the PivotHierarchies Class.
  #[sdk(child(qname = "x:pivotHierarchies"))]
  pub pivot_hierarchies: Option<crate::schemas::x::PivotHierarchies>,
  /// Defines the PivotTableStyle Class.
  #[sdk(child(qname = "x:pivotTableStyleInfo"))]
  pub pivot_table_style: Option<crate::schemas::x::PivotTableStyle>,
  /// Defines the PivotFilters Class.
  #[sdk(child(qname = "x:filters"))]
  pub pivot_filters: Option<crate::schemas::x::PivotFilters>,
  /// Defines the RowHierarchiesUsage Class.
  #[sdk(child(qname = "x:rowHierarchiesUsage"))]
  pub row_hierarchies_usage: Option<crate::schemas::x::RowHierarchiesUsage>,
  /// Defines the ColumnHierarchiesUsage Class.
  #[sdk(child(qname = "x:colHierarchiesUsage"))]
  pub column_hierarchies_usage: Option<crate::schemas::x::ColumnHierarchiesUsage>,
  /// Defines the PivotTableDefinitionExtensionList Class.
  #[sdk(child(qname = "x:extLst"))]
  pub pivot_table_definition_extension_list:
    Option<crate::schemas::x::PivotTableDefinitionExtensionList>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum RevExStreamChoice {
  /// Defines the RevExFuture Class.
  RevExFuture(std::boxed::Box<RevExFuture>),
  /// Defines the RevExUnsupported Class.
  RevExUnsupported(std::boxed::Box<RevExUnsupported>),
  /// Defines the RevExTrimmed Class.
  RevExTrimmed(std::boxed::Box<RevExTrimmed>),
  /// Defines the RevExRowColumn Class.
  RevExRowColumn(std::boxed::Box<RevExRowColumn>),
  /// Defines the RevExMove Class.
  RevExMove(std::boxed::Box<RevExMove>),
  /// Defines the RevExChangeCell Class.
  RevExChangeCell(std::boxed::Box<RevExChangeCell>),
  /// Defines the RevExFormatting Class.
  RevExFormatting(std::boxed::Box<RevExFormatting>),
  /// Defines the RevExDefinedName Class.
  RevExDefinedName(std::boxed::Box<RevExDefinedName>),
  /// Defines the RevExDelObj Class.
  RevExDelObj(std::boxed::Box<RevExDelObj>),
  /// Defines the RevExChgObj Class.
  RevExChgObj(std::boxed::Box<RevExChgObj>),
  /// Defines the RevExSheetOp Class.
  RevExSheetOp(std::boxed::Box<RevExSheetOp>),
  /// Defines the RevisionList Class.
  RevisionList(std::boxed::Box<RevisionList>),
  /// Defines the RevListAutoExpandRw Class.
  RevListAutoExpandRw(std::boxed::Box<RevListAutoExpandRw>),
  /// Defines the RevGroup Class.
  RevGroup(std::boxed::Box<RevGroup>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum StateBasedObjectChoice {
  /// Represents an external link to another workbook..
  DataValidation(std::boxed::Box<DataValidation>),
  /// Represents a hyperlink within a cell..
  Hyperlink(std::boxed::Box<Hyperlink>),
  /// Represents a sparkline group of 1 or more sparklines..
  SparklineGroup(std::boxed::Box<SparklineGroup>),
  /// Represents one comment within a cell..
  Comments(std::boxed::Box<Comments>),
  /// Represents an autofilter..
  AutoFilter(std::boxed::Box<AutoFilter>),
  /// Represents a PivotTable View..
  PivotTableDefinition(std::boxed::Box<PivotTableDefinition>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum RevExChgObjChoice {
  /// Defines the RevisionStateLink Class.
  RevisionStateLink(RevisionStateLink),
  /// Defines the RevisionState Class.
  RevisionState(std::boxed::Box<RevisionState>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum RevGroupChoice {
  /// Defines the RevExFuture Class.
  RevExFuture(std::boxed::Box<RevExFuture>),
  /// Defines the RevExUnsupported Class.
  RevExUnsupported(std::boxed::Box<RevExUnsupported>),
  /// Defines the RevExTrimmed Class.
  RevExTrimmed(std::boxed::Box<RevExTrimmed>),
  /// Defines the RevExRowColumn Class.
  RevExRowColumn(std::boxed::Box<RevExRowColumn>),
  /// Defines the RevExMove Class.
  RevExMove(std::boxed::Box<RevExMove>),
  /// Defines the RevExChangeCell Class.
  RevExChangeCell(std::boxed::Box<RevExChangeCell>),
  /// Defines the RevExFormatting Class.
  RevExFormatting(std::boxed::Box<RevExFormatting>),
  /// Defines the RevExDefinedName Class.
  RevExDefinedName(std::boxed::Box<RevExDefinedName>),
  /// Defines the RevExDelObj Class.
  RevExDelObj(std::boxed::Box<RevExDelObj>),
  /// Defines the RevExChgObj Class.
  RevExChgObj(std::boxed::Box<RevExChgObj>),
  /// Defines the RevExSheetOp Class.
  RevExSheetOp(std::boxed::Box<RevExSheetOp>),
  /// Defines the RevisionList Class.
  RevisionList(std::boxed::Box<RevisionList>),
  /// Defines the RevListAutoExpandRw Class.
  RevListAutoExpandRw(std::boxed::Box<RevListAutoExpandRw>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum RevisionStateChoice {
  /// Defines the RowColVisualOps Class.
  RowColVisualOps(RowColVisualOps),
  /// Defines the HideUnhideSheet Class.
  HideUnhideSheet(HideUnhideSheet),
  /// Defines the ShowGridlinesHeadings Class.
  ShowGridlinesHeadings(ShowGridlinesHeadings),
  /// Defines the FreezePanes Class.
  FreezePanes(FreezePanes),
  /// Defines the Outlines Class.
  Outlines(Outlines),
}
#[derive(Clone, Debug, PartialEq)]
pub enum RefMapChoice {
  /// Defines the RefCell Class.
  RefCell(std::boxed::Box<RefCell>),
  /// Defines the SheetXluid Class.
  SheetXluid(SheetXluid),
  /// Defines the RefOartAnchor Class.
  RefOartAnchor(std::boxed::Box<RefOartAnchor>),
  /// Defines the RefFuture Class.
  RefFuture,
  /// Defines the RefTest Class.
  RefTest(RefTest),
}
