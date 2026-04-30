//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DisplayBlanksAsValues {
  #[sdk(rename = "span")]
  #[default]
  Span,
  #[sdk(rename = "gap")]
  Gap,
  #[sdk(rename = "zero")]
  Zero,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SparklineAxisMinMaxValues {
  #[sdk(rename = "individual")]
  #[default]
  Individual,
  #[sdk(rename = "group")]
  Group,
  #[sdk(rename = "custom")]
  Custom,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SparklineTypeValues {
  #[sdk(rename = "line")]
  #[default]
  Line,
  #[sdk(rename = "column")]
  Column,
  #[sdk(rename = "stacked")]
  Stacked,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PivotShowAsValues {
  #[sdk(rename = "percentOfParent")]
  #[default]
  PercentOfParent,
  #[sdk(rename = "percentOfParentRow")]
  PercentOfParentRow,
  #[sdk(rename = "percentOfParentCol")]
  PercentOfParentColumn,
  #[sdk(rename = "percentOfRunningTotal")]
  PercentOfRunningTotal,
  #[sdk(rename = "rankAscending")]
  RankAscending,
  #[sdk(rename = "rankDescending")]
  RankDescending,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DataBarDirectionValues {
  #[sdk(rename = "context")]
  #[default]
  Context,
  #[sdk(rename = "leftToRight")]
  LeftToRight,
  #[sdk(rename = "rightToLeft")]
  RightToLeft,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DataBarAxisPositionValues {
  #[sdk(rename = "automatic")]
  #[default]
  Automatic,
  #[sdk(rename = "middle")]
  Middle,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ConditionalFormattingValueObjectTypeValues {
  #[sdk(rename = "num")]
  #[default]
  Numeric,
  #[sdk(rename = "percent")]
  Percent,
  #[sdk(rename = "max")]
  Max,
  #[sdk(rename = "min")]
  Min,
  #[sdk(rename = "formula")]
  Formula,
  #[sdk(rename = "percentile")]
  Percentile,
  #[sdk(rename = "autoMin")]
  AutoMin,
  #[sdk(rename = "autoMax")]
  AutoMax,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum IconSetTypeValues {
  #[sdk(rename = "3Arrows")]
  #[default]
  ThreeArrows,
  #[sdk(rename = "3ArrowsGray")]
  ThreeArrowsGray,
  #[sdk(rename = "3Flags")]
  ThreeFlags,
  #[sdk(rename = "3TrafficLights1")]
  ThreeTrafficLights1,
  #[sdk(rename = "3TrafficLights2")]
  ThreeTrafficLights2,
  #[sdk(rename = "3Signs")]
  ThreeSigns,
  #[sdk(rename = "3Symbols")]
  ThreeSymbols,
  #[sdk(rename = "3Symbols2")]
  ThreeSymbols2,
  #[sdk(rename = "4Arrows")]
  FourArrows,
  #[sdk(rename = "4ArrowsGray")]
  FourArrowsGray,
  #[sdk(rename = "4RedToBlack")]
  FourRedToBlack,
  #[sdk(rename = "4Rating")]
  FourRating,
  #[sdk(rename = "4TrafficLights")]
  FourTrafficLights,
  #[sdk(rename = "5Arrows")]
  FiveArrows,
  #[sdk(rename = "5ArrowsGray")]
  FiveArrowsGray,
  #[sdk(rename = "5Rating")]
  FiveRating,
  #[sdk(rename = "5Quarters")]
  FiveQuarters,
  #[sdk(rename = "3Stars")]
  ThreeStars,
  #[sdk(rename = "3Triangles")]
  ThreeTriangles,
  #[sdk(rename = "5Boxes")]
  FiveBoxes,
  #[sdk(rename = "NoIcons")]
  NoIcons,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PivotEditValueTypeValues {
  #[sdk(rename = "number")]
  #[default]
  Number,
  #[sdk(rename = "dateTime")]
  DateTime,
  #[sdk(rename = "string")]
  String,
  #[sdk(rename = "boolean")]
  Boolean,
  #[sdk(rename = "error")]
  Error,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AllocationMethodValues {
  #[sdk(rename = "equalAllocation")]
  #[default]
  EqualAllocation,
  #[sdk(rename = "equalIncrement")]
  EqualIncrement,
  #[sdk(rename = "weightedAllocation")]
  WeightedAllocation,
  #[sdk(rename = "weightedIncrement")]
  WeightedIncrement,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SlicerStyleTypeValues {
  #[sdk(rename = "unselectedItemWithData")]
  #[default]
  UnselectedItemWithData,
  #[sdk(rename = "selectedItemWithData")]
  SelectedItemWithData,
  #[sdk(rename = "unselectedItemWithNoData")]
  UnselectedItemWithNoData,
  #[sdk(rename = "selectedItemWithNoData")]
  SelectedItemWithNoData,
  #[sdk(rename = "hoveredUnselectedItemWithData")]
  HoveredUnselectedItemWithData,
  #[sdk(rename = "hoveredSelectedItemWithData")]
  HoveredSelectedItemWithData,
  #[sdk(rename = "hoveredUnselectedItemWithNoData")]
  HoveredUnselectedItemWithNoData,
  #[sdk(rename = "hoveredSelectedItemWithNoData")]
  HoveredSelectedItemWithNoData,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CheckedValues {
  #[sdk(rename = "Unchecked")]
  #[default]
  Unchecked,
  #[sdk(rename = "Checked")]
  Checked,
  #[sdk(rename = "Mixed")]
  Mixed,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DropStyleValues {
  #[sdk(rename = "combo")]
  #[default]
  Combo,
  #[sdk(rename = "comboedit")]
  ComboEdit,
  #[sdk(rename = "simple")]
  Simple,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SelectionTypeValues {
  #[sdk(rename = "single")]
  #[default]
  Single,
  #[sdk(rename = "multi")]
  Multiple,
  #[sdk(rename = "extended")]
  Extended,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextHorizontalAlignmentValues {
  #[sdk(rename = "left")]
  #[default]
  Left,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "right")]
  Right,
  #[sdk(rename = "justify")]
  Justify,
  #[sdk(rename = "distributed")]
  Distributed,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TextVerticalAlignmentValues {
  #[sdk(rename = "top")]
  #[default]
  Top,
  #[sdk(rename = "center")]
  Center,
  #[sdk(rename = "bottom")]
  Bottom,
  #[sdk(rename = "justify")]
  Justify,
  #[sdk(rename = "distributed")]
  Distributed,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum EditValidationValues {
  #[sdk(rename = "text")]
  #[default]
  Text,
  #[sdk(rename = "integer")]
  Integer,
  #[sdk(rename = "number")]
  Number,
  #[sdk(rename = "reference")]
  Reference,
  #[sdk(rename = "formula")]
  Formula,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum OlapSlicerCacheSortOrderValues {
  #[sdk(rename = "natural")]
  #[default]
  Natural,
  #[sdk(rename = "ascending")]
  Ascending,
  #[sdk(rename = "descending")]
  Descending,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TabularSlicerCacheSortOrderValues {
  #[sdk(rename = "ascending")]
  #[default]
  Ascending,
  #[sdk(rename = "descending")]
  Descending,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SlicerCacheCrossFilterValues {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "showItemsWithDataAtTop")]
  ShowItemsWithDataAtTop,
  #[sdk(rename = "showItemsWithNoData")]
  ShowItemsWithNoData,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ObjectTypeValues {
  #[sdk(rename = "Button")]
  #[default]
  Button,
  #[sdk(rename = "CheckBox")]
  CheckBox,
  #[sdk(rename = "Drop")]
  Drop,
  #[sdk(rename = "GBox")]
  GroupBox,
  #[sdk(rename = "Label")]
  Label,
  #[sdk(rename = "List")]
  List,
  #[sdk(rename = "Radio")]
  Radio,
  #[sdk(rename = "Scroll")]
  Scroll,
  #[sdk(rename = "Spin")]
  Spin,
  #[sdk(rename = "EditBox")]
  EditBox,
  #[sdk(rename = "Dialog")]
  Dialog,
}
/// Defines the ConditionalFormattings Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "x14:CT_ConditionalFormattings/x14:conditionalFormattings"
)]
pub struct ConditionalFormattings {
  /// _
  #[sdk(child(
    office2010,
    qname = "x14:CT_ConditionalFormatting/x14:conditionalFormatting"
  ))]
  pub x14_conditional_formatting: Vec<ConditionalFormatting>,
}
/// Defines the DataValidations Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_DataValidations/x14:dataValidations")]
pub struct DataValidations {
  /// disablePrompts
  #[sdk(attr(office2010, qname = ":disablePrompts"))]
  pub disable_prompts: Option<crate::simple_type::BooleanValue>,
  /// xWindow
  #[sdk(attr(office2010, qname = ":xWindow"))]
  pub x_window: Option<crate::simple_type::UInt32Value>,
  /// yWindow
  #[sdk(attr(office2010, qname = ":yWindow"))]
  pub y_window: Option<crate::simple_type::UInt32Value>,
  /// count
  #[sdk(attr(office2010, qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_DataValidation/x14:dataValidation"))]
  pub x14_data_validation: Vec<DataValidation>,
}
/// Defines the SparklineGroups Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_SparklineGroups/x14:sparklineGroups")]
pub struct SparklineGroups {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_SparklineGroup/x14:sparklineGroup"))]
  pub x14_sparkline_group: Vec<SparklineGroup>,
}
/// Defines the SlicerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_SlicerRefs/x14:slicerList")]
pub struct SlicerList {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_SlicerRef/x14:slicer"))]
  pub x14_slicer: Vec<SlicerRef>,
}
/// Defines the ProtectedRanges Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_ProtectedRanges/x14:protectedRanges")]
pub struct ProtectedRanges {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_ProtectedRange/x14:protectedRange"))]
  pub x14_protected_range: Vec<ProtectedRange>,
}
/// Defines the IgnoredErrors Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_IgnoredErrors/x14:ignoredErrors")]
pub struct IgnoredErrors {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_IgnoredError/x14:ignoredError"))]
  pub x14_ignored_error: Vec<IgnoredError>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_ExtensionList/x14:extLst"))]
  pub x14_ext_lst: Option<ExtensionList>,
}
/// Defines the DefinedNames Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_DefinedNames/x14:definedNames")]
pub struct DefinedNames {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_DefinedName/x14:definedName"))]
  pub x14_defined_name: Vec<DefinedName>,
}
/// Defines the PivotCaches Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_PivotCaches/x14:pivotCaches")]
pub struct PivotCaches {
  /// _
  #[sdk(child(qname = "x:CT_PivotCache/x:pivotCache"))]
  pub x_pivot_cache:
    Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCache>,
}
/// Defines the SlicerCaches Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_SlicerCaches/x14:slicerCaches")]
pub struct SlicerCaches {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_SlicerCache/x14:slicerCache"))]
  pub x14_slicer_cache: Vec<SlicerCache>,
}
/// Defines the WorkbookProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_WorkbookPr/x14:workbookPr")]
pub struct WorkbookProperties {
  /// defaultImageDpi
  #[sdk(attr(office2010, qname = ":defaultImageDpi"))]
  pub default_image_dpi: Option<crate::simple_type::UInt32Value>,
  /// discardImageEditData
  #[sdk(attr(office2010, qname = ":discardImageEditData"))]
  pub discard_image_edit_data: Option<crate::simple_type::BooleanValue>,
  /// accuracyVersion
  #[sdk(attr(office2010, qname = ":accuracyVersion"))]
  pub accuracy_version: Option<crate::simple_type::UInt32Value>,
}
/// Defines the CalculatedMember Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_CalculatedMember/x14:calculatedMember")]
pub struct CalculatedMember {
  /// displayFolder
  #[sdk(attr(office2010, qname = ":displayFolder"))]
  pub display_folder: Option<crate::simple_type::StringValue>,
  /// flattenHierarchies
  #[sdk(attr(office2010, qname = ":flattenHierarchies"))]
  pub flatten_hierarchies: Option<crate::simple_type::BooleanValue>,
  /// dynamicSet
  #[sdk(attr(office2010, qname = ":dynamicSet"))]
  pub dynamic_set: Option<crate::simple_type::BooleanValue>,
  /// hierarchizeDistinct
  #[sdk(attr(office2010, qname = ":hierarchizeDistinct"))]
  pub hierarchize_distinct: Option<crate::simple_type::BooleanValue>,
  /// mdxLong
  #[sdk(attr(office2010, qname = ":mdxLong"))]
  pub mdx_long: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_TupleSet/x14:tupleSet"))]
  pub tuple_set: Option<std::boxed::Box<TupleSet>>,
}
/// Defines the CacheHierarchy Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_CacheHierarchy/x14:cacheHierarchy")]
pub struct CacheHierarchy {
  /// flattenHierarchies
  #[sdk(attr(office2010, qname = ":flattenHierarchies"))]
  pub flatten_hierarchies: Option<crate::simple_type::BooleanValue>,
  /// measuresSet
  #[sdk(attr(office2010, qname = ":measuresSet"))]
  pub measures_set: Option<crate::simple_type::BooleanValue>,
  /// hierarchizeDistinct
  #[sdk(attr(office2010, qname = ":hierarchizeDistinct"))]
  pub hierarchize_distinct: Option<crate::simple_type::BooleanValue>,
  /// ignore
  #[sdk(attr(office2010, qname = ":ignore"))]
  pub ignore: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_SetLevels/x14:setLevels"))]
  pub set_levels: Option<SetLevels>,
}
/// Defines the DataField Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_DataField/x14:dataField")]
pub struct DataField {
  /// pivotShowAs
  #[sdk(attr(office2010, qname = ":pivotShowAs"))]
  pub pivot_show_as: Option<PivotShowAsValues>,
  /// sourceField
  #[sdk(attr(office2010, qname = ":sourceField"))]
  pub source_field: Option<crate::simple_type::UInt32Value>,
  /// uniqueName
  #[sdk(attr(office2010, qname = ":uniqueName"))]
  pub unique_name: Option<crate::simple_type::StringValue>,
}
/// Defines the PivotField Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_PivotField/x14:pivotField")]
pub struct PivotField {
  /// fillDownLabels
  #[sdk(attr(office2010, qname = ":fillDownLabels"))]
  pub fill_down_labels: Option<crate::simple_type::BooleanValue>,
  /// ignore
  #[sdk(attr(office2010, qname = ":ignore"))]
  pub ignore: Option<crate::simple_type::BooleanValue>,
}
/// Defines the PivotTableDefinition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "x14:CT_PivotTableDefinition/x14:pivotTableDefinition"
)]
pub struct PivotTableDefinition {
  /// fillDownLabelsDefault
  #[sdk(attr(office2010, qname = ":fillDownLabelsDefault"))]
  pub fill_down_labels_default: Option<crate::simple_type::BooleanValue>,
  /// visualTotalsForSets
  #[sdk(attr(office2010, qname = ":visualTotalsForSets"))]
  pub visual_totals_for_sets: Option<crate::simple_type::BooleanValue>,
  /// calculatedMembersInFilters
  #[sdk(attr(office2010, qname = ":calculatedMembersInFilters"))]
  pub calculated_members_in_filters: Option<crate::simple_type::BooleanValue>,
  /// altText
  #[sdk(attr(office2010, qname = ":altText"))]
  pub alt_text: Option<crate::simple_type::StringValue>,
  /// altTextSummary
  #[sdk(attr(office2010, qname = ":altTextSummary"))]
  pub alt_text_summary: Option<crate::simple_type::StringValue>,
  /// enableEdit
  #[sdk(attr(office2010, qname = ":enableEdit"))]
  pub enable_edit: Option<crate::simple_type::BooleanValue>,
  /// autoApply
  #[sdk(attr(office2010, qname = ":autoApply"))]
  pub auto_apply: Option<crate::simple_type::BooleanValue>,
  /// allocationMethod
  #[sdk(attr(office2010, qname = ":allocationMethod"))]
  pub allocation_method: Option<AllocationMethodValues>,
  /// weightExpression
  #[sdk(attr(office2010, qname = ":weightExpression"))]
  pub weight_expression: Option<crate::simple_type::StringValue>,
  /// hideValuesRow
  #[sdk(attr(office2010, qname = ":hideValuesRow"))]
  pub hide_values_row: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_PivotEdits/x14:pivotEdits"))]
  pub pivot_edits: Option<PivotEdits>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_PivotChanges/x14:pivotChanges"))]
  pub pivot_changes: Option<PivotChanges>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_ConditionalFormats/x14:conditionalFormats"))]
  pub conditional_formats: Option<ConditionalFormats>,
}
/// Defines the PivotCacheDefinition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "x14:CT_PivotCacheDefinition/x14:pivotCacheDefinition"
)]
pub struct PivotCacheDefinition {
  /// slicerData
  #[sdk(attr(office2010, qname = ":slicerData"))]
  pub slicer_data: Option<crate::simple_type::BooleanValue>,
  /// pivotCacheId
  #[sdk(attr(office2010, qname = ":pivotCacheId"))]
  pub pivot_cache_id: Option<crate::simple_type::UInt32Value>,
  /// supportSubqueryNonVisual
  #[sdk(attr(office2010, qname = ":supportSubqueryNonVisual"))]
  pub support_subquery_non_visual: Option<crate::simple_type::BooleanValue>,
  /// supportSubqueryCalcMem
  #[sdk(attr(office2010, qname = ":supportSubqueryCalcMem"))]
  pub support_subquery_calc_mem: Option<crate::simple_type::BooleanValue>,
  /// supportAddCalcMems
  #[sdk(attr(office2010, qname = ":supportAddCalcMems"))]
  pub support_add_calc_mems: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Connection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_Connection/x14:connection")]
pub struct Connection {
  /// culture
  #[sdk(attr(office2010, qname = ":culture"))]
  pub culture: Option<crate::simple_type::StringValue>,
  /// embeddedDataId
  #[sdk(attr(office2010, qname = ":embeddedDataId"))]
  pub embedded_data_id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_CalculatedMembers/x14:calculatedMembers"))]
  pub calculated_members: Option<CalculatedMembers>,
}
/// Defines the Table Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_Table/x14:table")]
pub struct Table {
  /// altText
  #[sdk(attr(office2010, qname = ":altText"))]
  pub alt_text: Option<crate::simple_type::StringValue>,
  /// altTextSummary
  #[sdk(attr(office2010, qname = ":altTextSummary"))]
  pub alt_text_summary: Option<crate::simple_type::StringValue>,
}
/// Defines the SlicerStyles Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_SlicerStyles/x14:slicerStyles")]
pub struct SlicerStyles {
  /// defaultSlicerStyle
  #[sdk(attr(office2010, qname = ":defaultSlicerStyle"))]
  pub default_slicer_style: crate::simple_type::StringValue,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_SlicerStyle/x14:slicerStyle"))]
  pub x14_slicer_style: Vec<SlicerStyle>,
}
/// Defines the DifferentialFormats Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_Dxfs/x14:dxfs")]
pub struct DifferentialFormats {
  /// Format Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Dxf/x:dxf"))]
  pub x_dxf:
    Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DifferentialFormat>,
}
/// Defines the OleItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_OleItem/x14:oleItem")]
pub struct OleItem {
  /// name
  #[sdk(attr(office2010, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// icon
  #[sdk(attr(office2010, qname = ":icon"))]
  pub icon: Option<crate::simple_type::BooleanValue>,
  /// advise
  #[sdk(attr(office2010, qname = ":advise"))]
  pub advise: Option<crate::simple_type::BooleanValue>,
  /// preferPic
  #[sdk(attr(office2010, qname = ":preferPic"))]
  pub prefer_picture: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_DdeValues/x14:values"))]
  pub dde_values: Option<DdeValues>,
}
/// Defines the PivotHierarchy Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_PivotHierarchy/x14:pivotHierarchy")]
pub struct PivotHierarchy {
  /// ignore
  #[sdk(attr(office2010, qname = ":ignore"))]
  pub ignore: Option<crate::simple_type::BooleanValue>,
}
/// Defines the CacheField Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_CacheField/x14:cacheField")]
pub struct CacheField {
  /// ignore
  #[sdk(attr(office2010, qname = ":ignore"))]
  pub ignore: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Id Class.
pub type Id = crate::simple_type::StringValue;
/// Defines the IconFilter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_IconFilter/x14:iconFilter")]
pub struct IconFilter {
  /// iconSet
  #[sdk(attr(office2010, qname = ":iconSet"))]
  pub icon_set: IconSetTypeValues,
  /// iconId
  #[sdk(attr(office2010, qname = ":iconId"))]
  pub icon_id: crate::simple_type::UInt32Value,
}
/// Defines the Filter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_Filter/x14:filter")]
pub struct Filter {
  /// val
  #[sdk(attr(office2010, qname = ":val"))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Defines the CustomFilters Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_CustomFilters/x14:customFilters")]
pub struct CustomFilters {
  /// and
  #[sdk(attr(office2010, qname = ":and"))]
  pub and: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_CustomFilter/x14:customFilter"))]
  pub x14_custom_filter: Vec<CustomFilter>,
}
/// Defines the SortCondition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_SortCondition/x14:sortCondition")]
pub struct SortCondition {
  /// descending
  #[sdk(attr(office2010, qname = ":descending"))]
  pub descending: Option<crate::simple_type::BooleanValue>,
  /// sortBy
  #[sdk(attr(office2010, qname = ":sortBy"))]
  pub sort_by:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SortByValues>,
  /// ref
  #[sdk(attr(office2010, qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// customList
  #[sdk(attr(office2010, qname = ":customList"))]
  pub custom_list: Option<crate::simple_type::StringValue>,
  /// dxfId
  #[sdk(attr(office2010, qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
  /// iconSet
  #[sdk(attr(office2010, qname = ":iconSet"))]
  pub icon_set: Option<IconSetTypeValues>,
  /// iconId
  #[sdk(attr(office2010, qname = ":iconId"))]
  pub icon_id: Option<crate::simple_type::UInt32Value>,
}
/// Defines the SourceConnection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_SourceConnection/x14:sourceConnection")]
pub struct SourceConnection {
  /// name
  #[sdk(attr(office2010, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// Defines the DatastoreItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_DatastoreItem/x14:datastoreItem")]
pub struct DatastoreItem {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(office2010, qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the FormControlProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_FormControlPr/x14:formControlPr")]
pub struct FormControlProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// objectType
  #[sdk(attr(office2010, qname = ":objectType"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub object_type: Option<ObjectTypeValues>,
  /// checked
  #[sdk(attr(office2010, qname = ":checked"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub checked: Option<CheckedValues>,
  /// colored
  #[sdk(attr(office2010, qname = ":colored"))]
  pub colored: Option<crate::simple_type::BooleanValue>,
  /// dropLines
  #[sdk(attr(office2010, qname = ":dropLines"))]
  pub drop_lines: Option<crate::simple_type::UInt32Value>,
  /// dropStyle
  #[sdk(attr(office2010, qname = ":dropStyle"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub drop_style: Option<DropStyleValues>,
  /// dx
  #[sdk(attr(office2010, qname = ":dx"))]
  pub scroll_bar_width: Option<crate::simple_type::UInt32Value>,
  /// firstButton
  #[sdk(attr(office2010, qname = ":firstButton"))]
  pub first_button: Option<crate::simple_type::BooleanValue>,
  /// fmlaGroup
  #[sdk(attr(office2010, qname = ":fmlaGroup"))]
  pub fmla_group: Option<crate::simple_type::StringValue>,
  /// fmlaLink
  #[sdk(attr(office2010, qname = ":fmlaLink"))]
  pub fmla_link: Option<crate::simple_type::StringValue>,
  /// fmlaRange
  #[sdk(attr(office2010, qname = ":fmlaRange"))]
  pub fmla_range: Option<crate::simple_type::StringValue>,
  /// fmlaTxbx
  #[sdk(attr(office2010, qname = ":fmlaTxbx"))]
  pub fmla_textbox: Option<crate::simple_type::StringValue>,
  /// horiz
  #[sdk(attr(office2010, qname = ":horiz"))]
  pub horizontal: Option<crate::simple_type::BooleanValue>,
  /// inc
  #[sdk(attr(office2010, qname = ":inc"))]
  pub incremental: Option<crate::simple_type::UInt32Value>,
  /// justLastX
  #[sdk(attr(office2010, qname = ":justLastX"))]
  pub just_last_x: Option<crate::simple_type::BooleanValue>,
  /// lockText
  #[sdk(attr(office2010, qname = ":lockText"))]
  pub lock_text: Option<crate::simple_type::BooleanValue>,
  /// max
  #[sdk(attr(office2010, qname = ":max"))]
  pub max: Option<crate::simple_type::UInt32Value>,
  /// min
  #[sdk(attr(office2010, qname = ":min"))]
  pub min: Option<crate::simple_type::UInt32Value>,
  /// multiSel
  #[sdk(attr(office2010, qname = ":multiSel"))]
  pub multiple_selection: Option<crate::simple_type::StringValue>,
  /// noThreeD
  #[sdk(attr(office2010, qname = ":noThreeD"))]
  pub no_three_d: Option<crate::simple_type::BooleanValue>,
  /// noThreeD2
  #[sdk(attr(office2010, qname = ":noThreeD2"))]
  pub no_three_d2: Option<crate::simple_type::BooleanValue>,
  /// page
  #[sdk(attr(office2010, qname = ":page"))]
  pub page: Option<crate::simple_type::UInt32Value>,
  /// sel
  #[sdk(attr(office2010, qname = ":sel"))]
  pub selected: Option<crate::simple_type::UInt32Value>,
  /// seltype
  #[sdk(attr(office2010, qname = ":seltype"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub selection_type: Option<SelectionTypeValues>,
  /// textHAlign
  #[sdk(attr(office2010, qname = ":textHAlign"))]
  pub text_horizontal_align: Option<TextHorizontalAlignmentValues>,
  /// textVAlign
  #[sdk(attr(office2010, qname = ":textVAlign"))]
  pub text_vertical_align: Option<TextVerticalAlignmentValues>,
  /// val
  #[sdk(attr(office2010, qname = ":val"))]
  pub val: Option<crate::simple_type::UInt32Value>,
  /// widthMin
  #[sdk(attr(office2010, qname = ":widthMin"))]
  pub minimum_width: Option<crate::simple_type::UInt32Value>,
  /// editVal
  #[sdk(attr(office2010, qname = ":editVal"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub edit_val: Option<EditValidationValues>,
  /// multiLine
  #[sdk(attr(office2010, qname = ":multiLine"))]
  pub multiple_lines: Option<crate::simple_type::BooleanValue>,
  /// verticalBar
  #[sdk(attr(office2010, qname = ":verticalBar"))]
  pub vertical_bar: Option<crate::simple_type::BooleanValue>,
  /// passwordEdit
  #[sdk(attr(office2010, qname = ":passwordEdit"))]
  pub password_edit: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_ListItems/x14:itemLst"))]
  pub list_items: Option<std::boxed::Box<ListItems>>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Slicers Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_Slicers/x14:slicers")]
pub struct Slicers {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_Slicer/x14:slicer"))]
  pub x14_slicer: Vec<Slicer>,
}
/// Defines the SlicerCacheDefinition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "x14:CT_SlicerCacheDefinition/x14:slicerCacheDefinition"
)]
pub struct SlicerCacheDefinition {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// name
  #[sdk(attr(office2010, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// sourceName
  #[sdk(attr(office2010, qname = ":sourceName"))]
  pub source_name: crate::simple_type::StringValue,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_SlicerCachePivotTables/x14:pivotTables"))]
  pub slicer_cache_pivot_tables: Option<SlicerCachePivotTables>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_SlicerCacheData/x14:data"))]
  pub slicer_cache_data: Option<std::boxed::Box<SlicerCacheData>>,
  /// _
  #[sdk(child(
    office2010,
    qname = "x:CT_SlicerCacheDefinitionExtensionList/x14:extLst"
  ))]
  pub slicer_cache_definition_extension_list: Option<SlicerCacheDefinitionExtensionList>,
}
/// Defines the ConditionalFormatting Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "x14:CT_ConditionalFormatting/x14:conditionalFormatting"
)]
pub struct ConditionalFormatting {
  /// pivot
  #[sdk(attr(office2010, qname = ":pivot"))]
  pub pivot: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_CfRule/x14:cfRule"))]
  pub x14_cf_rule: Vec<ConditionalFormattingRule>,
  /// _
  #[sdk(text_child(office2010, qname = "xne:ST_Sqref/xne:sqref"))]
  pub xne_sqref: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_ExtensionList/x14:extLst"))]
  pub x14_ext_lst: Option<ExtensionList>,
}
/// Defines the ConditionalFormattingRule Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_CfRule/x14:cfRule")]
pub struct ConditionalFormattingRule {
    /// type
    #[sdk(attr(office2010, qname = ":type"))]
    pub r#type: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ConditionalFormatValues,
    >,
    /// priority
    #[sdk(attr(office2010, qname = ":priority"))]
    pub priority: Option<crate::simple_type::Int32Value>,
    /// stopIfTrue
    #[sdk(attr(office2010, qname = ":stopIfTrue"))]
    pub stop_if_true: Option<crate::simple_type::BooleanValue>,
    /// aboveAverage
    #[sdk(attr(office2010, qname = ":aboveAverage"))]
    pub above_average: Option<crate::simple_type::BooleanValue>,
    /// percent
    #[sdk(attr(office2010, qname = ":percent"))]
    pub percent: Option<crate::simple_type::BooleanValue>,
    /// bottom
    #[sdk(attr(office2010, qname = ":bottom"))]
    pub bottom: Option<crate::simple_type::BooleanValue>,
    /// operator
    #[sdk(attr(office2010, qname = ":operator"))]
    pub operator: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ConditionalFormattingOperatorValues,
    >,
    /// text
    #[sdk(attr(office2010, qname = ":text"))]
    pub text: Option<crate::simple_type::StringValue>,
    /// timePeriod
    #[sdk(attr(office2010, qname = ":timePeriod"))]
    pub time_period: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::TimePeriodValues,
    >,
    /// rank
    #[sdk(attr(office2010, qname = ":rank"))]
    pub rank: Option<crate::simple_type::UInt32Value>,
    /// stdDev
    #[sdk(attr(office2010, qname = ":stdDev"))]
    pub standard_deviation: Option<crate::simple_type::Int32Value>,
    /// equalAverage
    #[sdk(attr(office2010, qname = ":equalAverage"))]
    pub equal_average: Option<crate::simple_type::BooleanValue>,
    /// activePresent
    #[sdk(attr(office2010, qname = ":activePresent"))]
    pub active_present: Option<crate::simple_type::BooleanValue>,
    /// id
    #[sdk(attr(office2010, qname = ":id"))]
    #[sdk(
        pattern(
            source = 0u32,
            regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
        )
    )]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub id: Option<crate::simple_type::StringValue>,
    /// _
    #[sdk(text_child(office2010, qname = "x:ST_Formula/xne:f"))]
    pub xne_f: Vec<crate::simple_type::StringValue>,
    /// _
    #[sdk(child(office2010, qname = "x14:CT_ColorScale/x14:colorScale"))]
    pub x14_color_scale: Option<ColorScale>,
    /// _
    #[sdk(child(office2010, qname = "x14:CT_DataBar/x14:dataBar"))]
    pub x14_data_bar: Option<std::boxed::Box<DataBar>>,
    /// _
    #[sdk(child(office2010, qname = "x14:CT_IconSet/x14:iconSet"))]
    pub x14_icon_set: Option<IconSet>,
    /// _
    #[sdk(child(office2010, qname = "x:CT_Dxf/x14:dxf"))]
    pub x14_dxf: Option<std::boxed::Box<DifferentialType>>,
    /// _
    #[sdk(child(office2010, qname = "x:CT_ExtensionList/x14:extLst"))]
    pub x14_ext_lst: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_ExtensionList/x14:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
/// Defines the DataValidation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_DataValidation/x14:dataValidation")]
pub struct DataValidation {
    /// type
    #[sdk(attr(office2010, qname = ":type"))]
    pub r#type: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationValues,
    >,
    /// errorStyle
    #[sdk(attr(office2010, qname = ":errorStyle"))]
    pub error_style: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationErrorStyleValues,
    >,
    /// imeMode
    #[sdk(attr(office2010, qname = ":imeMode"))]
    pub ime_mode: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationImeModeValues,
    >,
    /// operator
    #[sdk(attr(office2010, qname = ":operator"))]
    pub operator: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationOperatorValues,
    >,
    /// allowBlank
    #[sdk(attr(office2010, qname = ":allowBlank"))]
    pub allow_blank: Option<crate::simple_type::BooleanValue>,
    /// showDropDown
    #[sdk(attr(office2010, qname = ":showDropDown"))]
    pub show_drop_down: Option<crate::simple_type::BooleanValue>,
    /// showInputMessage
    #[sdk(attr(office2010, qname = ":showInputMessage"))]
    pub show_input_message: Option<crate::simple_type::BooleanValue>,
    /// showErrorMessage
    #[sdk(attr(office2010, qname = ":showErrorMessage"))]
    pub show_error_message: Option<crate::simple_type::BooleanValue>,
    /// errorTitle
    #[sdk(attr(office2010, qname = ":errorTitle"))]
    pub error_title: Option<crate::simple_type::StringValue>,
    /// error
    #[sdk(attr(office2010, qname = ":error"))]
    pub error: Option<crate::simple_type::StringValue>,
    /// promptTitle
    #[sdk(attr(office2010, qname = ":promptTitle"))]
    pub prompt_title: Option<crate::simple_type::StringValue>,
    /// prompt
    #[sdk(attr(office2010, qname = ":prompt"))]
    pub prompt: Option<crate::simple_type::StringValue>,
    /// _
    #[sdk(child(office2010, qname = "x14:CT_DataValidationFormula/x14:formula1"))]
    pub data_validation_forumla1: Option<DataValidationForumla1>,
    /// _
    #[sdk(child(office2010, qname = "x14:CT_DataValidationFormula/x14:formula2"))]
    pub data_validation_forumla2: Option<DataValidationForumla2>,
    /// _
    #[sdk(text_child(office2010, qname = "xne:ST_Sqref/xne:sqref"))]
    pub reference_sequence: crate::simple_type::StringValue,
}
/// Defines the DataValidationForumla1 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_DataValidationFormula/x14:formula1")]
pub struct DataValidationForumla1 {
  /// _
  #[sdk(text_child(office2010, qname = "x:ST_Formula/xne:f"))]
  pub formula: crate::simple_type::StringValue,
}
/// Defines the DataValidationForumla2 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_DataValidationFormula/x14:formula2")]
pub struct DataValidationForumla2 {
  /// _
  #[sdk(text_child(office2010, qname = "x:ST_Formula/xne:f"))]
  pub formula: crate::simple_type::StringValue,
}
/// Defines the DataValidationFormulaType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_DataValidationFormula/")]
pub struct DataValidationFormulaType {
  /// _
  #[sdk(text_child(office2010, qname = "x:ST_Formula/xne:f"))]
  pub formula: Vec<crate::simple_type::StringValue>,
}
/// Defines the SparklineGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_SparklineGroup/x14:sparklineGroup")]
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
  pub r#type: Option<SparklineTypeValues>,
  /// dateAxis
  #[sdk(attr(office2010, qname = ":dateAxis"))]
  pub date_axis: Option<crate::simple_type::BooleanValue>,
  /// displayEmptyCellsAs
  #[sdk(attr(office2010, qname = ":displayEmptyCellsAs"))]
  pub display_empty_cells_as: Option<DisplayBlanksAsValues>,
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
  pub min_axis_type: Option<SparklineAxisMinMaxValues>,
  /// maxAxisType
  #[sdk(attr(office2010, qname = ":maxAxisType"))]
  pub max_axis_type: Option<SparklineAxisMinMaxValues>,
  /// rightToLeft
  #[sdk(attr(office2010, qname = ":rightToLeft"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_Color/x14:colorSeries"))]
  pub series_color: Option<SeriesColor>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_Color/x14:colorNegative"))]
  pub negative_color: Option<NegativeColor>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_Color/x14:colorAxis"))]
  pub axis_color: Option<AxisColor>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_Color/x14:colorMarkers"))]
  pub markers_color: Option<MarkersColor>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_Color/x14:colorFirst"))]
  pub first_marker_color: Option<FirstMarkerColor>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_Color/x14:colorLast"))]
  pub last_marker_color: Option<LastMarkerColor>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_Color/x14:colorHigh"))]
  pub high_marker_color: Option<HighMarkerColor>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_Color/x14:colorLow"))]
  pub low_marker_color: Option<LowMarkerColor>,
  /// _
  #[sdk(text_child(office2010, qname = "x:ST_Formula/xne:f"))]
  pub formula: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_Sparklines/x14:sparklines"))]
  pub sparklines: std::boxed::Box<Sparklines>,
}
/// Defines the SeriesColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_Color/x14:colorSeries")]
pub struct SeriesColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the NegativeColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_Color/x14:colorNegative")]
pub struct NegativeColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the AxisColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_Color/x14:colorAxis")]
pub struct AxisColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the MarkersColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_Color/x14:colorMarkers")]
pub struct MarkersColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the FirstMarkerColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_Color/x14:colorFirst")]
pub struct FirstMarkerColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the LastMarkerColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_Color/x14:colorLast")]
pub struct LastMarkerColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the HighMarkerColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_Color/x14:colorHigh")]
pub struct HighMarkerColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the LowMarkerColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_Color/x14:colorLow")]
pub struct LowMarkerColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the Color Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_Color/x14:color")]
pub struct Color {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the FillColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_Color/x14:fillColor")]
pub struct FillColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the BorderColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_Color/x14:borderColor")]
pub struct BorderColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the NegativeFillColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_Color/x14:negativeFillColor")]
pub struct NegativeFillColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the NegativeBorderColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_Color/x14:negativeBorderColor")]
pub struct NegativeBorderColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the BarAxisColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_Color/x14:axisColor")]
pub struct BarAxisColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the ColorType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/")]
pub struct ColorType {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the Sparklines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_Sparklines/x14:sparklines")]
pub struct Sparklines {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_Sparkline/x14:sparkline"))]
  pub x14_sparkline: Vec<Sparkline>,
}
/// Defines the Sparkline Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_Sparkline/x14:sparkline")]
pub struct Sparkline {
  /// _
  #[sdk(text_child(office2010, qname = "x:ST_Formula/xne:f"))]
  pub formula: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(office2010, qname = "xne:ST_Sqref/xne:sqref"))]
  pub reference_sequence: crate::simple_type::StringValue,
}
/// Defines the SlicerRef Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_SlicerRef/x14:slicer")]
pub struct SlicerRef {
  /// id
  #[sdk(attr(office2010, qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the SlicerCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_SlicerCache/x14:slicerCache")]
pub struct SlicerCache {
  /// id
  #[sdk(attr(office2010, qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the DefinedName Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_DefinedName/x14:definedName")]
pub struct DefinedName {
  /// name
  #[sdk(attr(office2010, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// _
  #[sdk(child(
    office2010,
    qname = "x14:CT_DefinedNameArgumentDescriptions/x14:argumentDescriptions"
  ))]
  pub argument_descriptions: Option<ArgumentDescriptions>,
}
/// Defines the ArgumentDescriptions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "x14:CT_DefinedNameArgumentDescriptions/x14:argumentDescriptions"
)]
pub struct ArgumentDescriptions {
  /// count
  #[sdk(attr(office2010, qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(
    office2010,
    qname = "x14:CT_DefinedNameArgumentDescription/x14:argumentDescription"
  ))]
  pub x14_argument_description: Vec<ArgumentDescription>,
}
/// Defines the ArgumentDescription Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "x14:CT_DefinedNameArgumentDescription/x14:argumentDescription"
)]
pub struct ArgumentDescription {
  /// index
  #[sdk(attr(office2010, qname = ":index"))]
  pub index: crate::simple_type::UInt32Value,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the TupleSet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_TupleSet/x14:tupleSet")]
pub struct TupleSet {
  /// rowCount
  #[sdk(attr(office2010, qname = ":rowCount"))]
  pub row_count: Option<crate::simple_type::UInt32Value>,
  /// columnCount
  #[sdk(attr(office2010, qname = ":columnCount"))]
  pub column_count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_TupleSetHeaders/x14:headers"))]
  pub tuple_set_headers: std::boxed::Box<TupleSetHeaders>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_TupleSetRows/x14:rows"))]
  pub tuple_set_rows: std::boxed::Box<TupleSetRows>,
}
/// Defines the TupleSetHeaders Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_TupleSetHeaders/x14:headers")]
pub struct TupleSetHeaders {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_TupleSetHeader/x14:header"))]
  pub x14_header: Vec<TupleSetHeader>,
}
/// Defines the TupleSetRows Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_TupleSetRows/x14:rows")]
pub struct TupleSetRows {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_TupleSetRow/x14:row"))]
  pub x14_row: Vec<TupleSetRow>,
}
/// Defines the TupleSetHeader Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_TupleSetHeader/x14:header")]
pub struct TupleSetHeader {
  /// uniqueName
  #[sdk(attr(office2010, qname = ":uniqueName"))]
  pub unique_name: Option<crate::simple_type::StringValue>,
  /// hierarchyName
  #[sdk(attr(office2010, qname = ":hierarchyName"))]
  pub hierarchy_name: Option<crate::simple_type::StringValue>,
}
/// Defines the TupleSetRow Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_TupleSetRow/x14:row")]
pub struct TupleSetRow {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_TupleSetRowItem/x14:rowItem"))]
  pub x14_row_item: Vec<TupleSetRowItem>,
}
/// Defines the TupleSetRowItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_TupleSetRowItem/x14:rowItem")]
pub struct TupleSetRowItem {
  /// u
  #[sdk(attr(office2010, qname = ":u"))]
  pub unique_name: Option<crate::simple_type::StringValue>,
  /// d
  #[sdk(attr(office2010, qname = ":d"))]
  pub display_name: Option<crate::simple_type::StringValue>,
}
/// Defines the SetLevel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_SetLevel/x14:setLevel")]
pub struct SetLevel {
  /// hierarchy
  #[sdk(attr(office2010, qname = ":hierarchy"))]
  pub hierarchy: crate::simple_type::Int32Value,
}
/// Defines the SetLevels Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_SetLevels/x14:setLevels")]
pub struct SetLevels {
  /// count
  #[sdk(attr(office2010, qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_SetLevel/x14:setLevel"))]
  pub x14_set_level: Vec<SetLevel>,
}
/// Defines the ColorScale Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_ColorScale/x14:colorScale")]
pub struct ColorScale {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_Cfvo/x14:cfvo"))]
  pub x14_cfvo: Vec<ConditionalFormattingValueObject>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_Color/x14:color"))]
  pub x14_color: Vec<Color>,
}
/// Defines the DataBar Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_DataBar/x14:dataBar")]
pub struct DataBar {
  /// minLength
  #[sdk(attr(office2010, qname = ":minLength"))]
  pub min_length: Option<crate::simple_type::UInt32Value>,
  /// maxLength
  #[sdk(attr(office2010, qname = ":maxLength"))]
  pub max_length: Option<crate::simple_type::UInt32Value>,
  /// showValue
  #[sdk(attr(office2010, qname = ":showValue"))]
  pub show_value: Option<crate::simple_type::BooleanValue>,
  /// border
  #[sdk(attr(office2010, qname = ":border"))]
  pub border: Option<crate::simple_type::BooleanValue>,
  /// gradient
  #[sdk(attr(office2010, qname = ":gradient"))]
  pub gradient: Option<crate::simple_type::BooleanValue>,
  /// direction
  #[sdk(attr(office2010, qname = ":direction"))]
  pub direction: Option<DataBarDirectionValues>,
  /// negativeBarColorSameAsPositive
  #[sdk(attr(office2010, qname = ":negativeBarColorSameAsPositive"))]
  pub negative_bar_color_same_as_positive: Option<crate::simple_type::BooleanValue>,
  /// negativeBarBorderColorSameAsPositive
  #[sdk(attr(office2010, qname = ":negativeBarBorderColorSameAsPositive"))]
  pub negative_bar_border_color_same_as_positive: Option<crate::simple_type::BooleanValue>,
  /// axisPosition
  #[sdk(attr(office2010, qname = ":axisPosition"))]
  pub axis_position: Option<DataBarAxisPositionValues>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_Cfvo/x14:cfvo"))]
  pub x14_cfvo: Vec<ConditionalFormattingValueObject>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_Color/x14:fillColor"))]
  pub x14_fill_color: Option<FillColor>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_Color/x14:borderColor"))]
  pub x14_border_color: Option<BorderColor>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_Color/x14:negativeFillColor"))]
  pub x14_negative_fill_color: Option<NegativeFillColor>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_Color/x14:negativeBorderColor"))]
  pub x14_negative_border_color: Option<NegativeBorderColor>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_Color/x14:axisColor"))]
  pub x14_axis_color: Option<BarAxisColor>,
}
/// Defines the IconSet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_IconSet/x14:iconSet")]
pub struct IconSet {
  /// iconSet
  #[sdk(attr(office2010, qname = ":iconSet"))]
  pub icon_set_types: Option<IconSetTypeValues>,
  /// showValue
  #[sdk(attr(office2010, qname = ":showValue"))]
  pub show_value: Option<crate::simple_type::BooleanValue>,
  /// percent
  #[sdk(attr(office2010, qname = ":percent"))]
  pub percent: Option<crate::simple_type::BooleanValue>,
  /// reverse
  #[sdk(attr(office2010, qname = ":reverse"))]
  pub reverse: Option<crate::simple_type::BooleanValue>,
  /// custom
  #[sdk(attr(office2010, qname = ":custom"))]
  pub custom: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_Cfvo/x14:cfvo"))]
  pub x14_cfvo: Vec<ConditionalFormattingValueObject>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_CfIcon/x14:cfIcon"))]
  pub x14_cf_icon: Vec<ConditionalFormattingIcon>,
}
/// Defines the DifferentialType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_Dxf/x14:dxf")]
pub struct DifferentialType {
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
/// Defines the ConditionalFormattingValueObject Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_Cfvo/x14:cfvo")]
pub struct ConditionalFormattingValueObject {
  /// type
  #[sdk(attr(office2010, qname = ":type"))]
  pub r#type: ConditionalFormattingValueObjectTypeValues,
  /// gte
  #[sdk(attr(office2010, qname = ":gte"))]
  pub greater_than_or_equal: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(text_child(office2010, qname = "x:ST_Formula/xne:f"))]
  pub formula: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ConditionalFormattingIcon Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_CfIcon/x14:cfIcon")]
pub struct ConditionalFormattingIcon {
  /// iconSet
  #[sdk(attr(office2010, qname = ":iconSet"))]
  pub icon_set: IconSetTypeValues,
  /// iconId
  #[sdk(attr(office2010, qname = ":iconId"))]
  pub icon_id: crate::simple_type::UInt32Value,
}
/// Defines the PivotEdits Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_PivotEdits/x14:pivotEdits")]
pub struct PivotEdits {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_PivotEdit/x14:pivotEdit"))]
  pub x14_pivot_edit: Vec<PivotEdit>,
}
/// Defines the PivotChanges Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_PivotChanges/x14:pivotChanges")]
pub struct PivotChanges {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_PivotChange/x14:pivotChange"))]
  pub x14_pivot_change: Vec<PivotChange>,
}
/// Defines the ConditionalFormats Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_ConditionalFormats/x14:conditionalFormats")]
pub struct ConditionalFormats {
  /// count
  #[sdk(attr(office2010, qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_ConditionalFormat/x14:conditionalFormat"))]
  pub x14_conditional_format: Vec<ConditionalFormat>,
}
/// Defines the CalculatedMembers Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_CalculatedMembers/x14:calculatedMembers")]
pub struct CalculatedMembers {
  /// Calculated Members Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_CalculatedMember/x:calculatedMember"))]
  pub x_calculated_member:
    Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CalculatedMember>,
}
/// Defines the PivotEdit Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_PivotEdit/x14:pivotEdit")]
pub struct PivotEdit {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_PivotUserEdit/x14:userEdit"))]
  pub pivot_user_edit: std::boxed::Box<PivotUserEdit>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_TupleItems/x14:tupleItems"))]
  pub tuple_items: std::boxed::Box<TupleItems>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_PivotArea/x14:pivotArea"))]
  pub pivot_area: std::boxed::Box<PivotArea>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotUserEdit Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_PivotUserEdit/x14:userEdit")]
pub struct PivotUserEdit {
  #[sdk(choice(
    qname = "x:ST_Formula/xne:f",
    qname = "x14:CT_PivotEditValue/x14:editValue"
  ))]
  pub xml_children: Option<PivotUserEditChoice>,
}
/// Defines the TupleItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_TupleItems/x14:tupleItems")]
pub struct TupleItems {
  /// _
  #[sdk(text_child(office2010, qname = "x:ST_Xstring/x14:tupleItem"))]
  pub x14_tuple_item: Vec<crate::simple_type::StringValue>,
}
/// Defines the PivotArea Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_PivotArea/x14:pivotArea")]
pub struct PivotArea {
  /// Field Index
  #[sdk(attr(qname = ":field"))]
  pub field: Option<crate::simple_type::Int32Value>,
  /// Rule Type
  #[sdk(attr(qname = ":type"))]
  pub r#type:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotAreaValues>,
  /// Data Only
  #[sdk(attr(qname = ":dataOnly"))]
  pub data_only: Option<crate::simple_type::BooleanValue>,
  /// Labels Only
  #[sdk(attr(qname = ":labelOnly"))]
  pub label_only: Option<crate::simple_type::BooleanValue>,
  /// Include Row Grand Total
  #[sdk(attr(qname = ":grandRow"))]
  pub grand_row: Option<crate::simple_type::BooleanValue>,
  /// Include Column Grand Total
  #[sdk(attr(qname = ":grandCol"))]
  pub grand_column: Option<crate::simple_type::BooleanValue>,
  /// Cache Index
  #[sdk(attr(qname = ":cacheIndex"))]
  pub cache_index: Option<crate::simple_type::BooleanValue>,
  /// Outline
  #[sdk(attr(qname = ":outline"))]
  pub outline: Option<crate::simple_type::BooleanValue>,
  /// Offset Reference
  #[sdk(attr(qname = ":offset"))]
  pub offset: Option<crate::simple_type::StringValue>,
  /// Collapsed Levels Are Subtotals
  #[sdk(attr(qname = ":collapsedLevelsAreSubtotals"))]
  pub collapsed_levels_are_subtotals: Option<crate::simple_type::BooleanValue>,
  /// Axis
  #[sdk(attr(qname = ":axis"))]
  pub axis: Option<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableAxisValues,
  >,
  /// Field Position
  #[sdk(attr(qname = ":fieldPosition"))]
  pub field_position: Option<crate::simple_type::UInt32Value>,
  /// References
  #[sdk(child(qname = "x:CT_PivotAreaReferences/x:references"))]
  pub pivot_area_references:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotAreaReferences>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList>,
}
/// Defines the PivotChange Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_PivotChange/x14:pivotChange")]
pub struct PivotChange {
  /// allocationMethod
  #[sdk(attr(office2010, qname = ":allocationMethod"))]
  pub allocation_method: Option<AllocationMethodValues>,
  /// weightExpression
  #[sdk(attr(office2010, qname = ":weightExpression"))]
  pub weight_expression: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_PivotEditValue/x14:editValue"))]
  pub pivot_edit_value: std::boxed::Box<PivotEditValue>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_TupleItems/x14:tupleItems"))]
  pub tuple_items: std::boxed::Box<TupleItems>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotEditValue Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_PivotEditValue/x14:editValue")]
pub struct PivotEditValue {
  /// valueType
  #[sdk(attr(office2010, qname = ":valueType"))]
  pub value_type: PivotEditValueTypeValues,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Xstring Class.
pub type Xstring = crate::simple_type::StringValue;
/// Defines the SlicerStyleElements Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "x14:CT_SlicerStyleElements/x14:slicerStyleElements"
)]
pub struct SlicerStyleElements {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_SlicerStyleElement/x14:slicerStyleElement"))]
  pub x14_slicer_style_element: Vec<SlicerStyleElement>,
}
/// Defines the DdeValues Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_DdeValues/x14:values")]
pub struct DdeValues {
  /// Rows
  #[sdk(attr(qname = ":rows"))]
  pub rows: Option<crate::simple_type::UInt32Value>,
  /// Columns
  #[sdk(attr(qname = ":cols"))]
  pub columns: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_DdeValue/x:value"))]
  pub x_value: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Value>,
}
/// Defines the ConditionalFormat Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_ConditionalFormat/x14:conditionalFormat")]
pub struct ConditionalFormat {
  /// scope
  #[sdk(attr(office2010, qname = ":scope"))]
  pub scope:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ScopeValues>,
  /// type
  #[sdk(attr(office2010, qname = ":type"))]
  pub r#type:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::RuleValues>,
  /// priority
  #[sdk(attr(office2010, qname = ":priority"))]
  pub priority: Option<crate::simple_type::UInt32Value>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(office2010, qname = "x:CT_PivotAreas/x14:pivotAreas"))]
  pub pivot_areas: Option<PivotAreas>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotAreas Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x:CT_PivotAreas/x14:pivotAreas")]
pub struct PivotAreas {
  /// Pivot Area Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_PivotArea/x:pivotArea"))]
  pub x_pivot_area:
    Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotArea>,
}
/// Defines the SlicerStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_SlicerStyle/x14:slicerStyle")]
pub struct SlicerStyle {
  /// name
  #[sdk(attr(office2010, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// _
  #[sdk(child(
    office2010,
    qname = "x14:CT_SlicerStyleElements/x14:slicerStyleElements"
  ))]
  pub slicer_style_elements: Option<SlicerStyleElements>,
}
/// Defines the SlicerStyleElement Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_SlicerStyleElement/x14:slicerStyleElement")]
pub struct SlicerStyleElement {
  /// type
  #[sdk(attr(office2010, qname = ":type"))]
  pub r#type: SlicerStyleTypeValues,
  /// dxfId
  #[sdk(attr(office2010, qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
}
/// Defines the IgnoredError Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_IgnoredError/x14:ignoredError")]
pub struct IgnoredError {
  /// evalError
  #[sdk(attr(office2010, qname = ":evalError"))]
  pub eval_error: Option<crate::simple_type::BooleanValue>,
  /// twoDigitTextYear
  #[sdk(attr(office2010, qname = ":twoDigitTextYear"))]
  pub two_digit_text_year: Option<crate::simple_type::BooleanValue>,
  /// numberStoredAsText
  #[sdk(attr(office2010, qname = ":numberStoredAsText"))]
  pub number_stored_as_text: Option<crate::simple_type::BooleanValue>,
  /// formula
  #[sdk(attr(office2010, qname = ":formula"))]
  pub formula: Option<crate::simple_type::BooleanValue>,
  /// formulaRange
  #[sdk(attr(office2010, qname = ":formulaRange"))]
  pub formula_range: Option<crate::simple_type::BooleanValue>,
  /// unlockedFormula
  #[sdk(attr(office2010, qname = ":unlockedFormula"))]
  pub unlocked_formula: Option<crate::simple_type::BooleanValue>,
  /// emptyCellReference
  #[sdk(attr(office2010, qname = ":emptyCellReference"))]
  pub empty_cell_reference: Option<crate::simple_type::BooleanValue>,
  /// listDataValidation
  #[sdk(attr(office2010, qname = ":listDataValidation"))]
  pub list_data_validation: Option<crate::simple_type::BooleanValue>,
  /// calculatedColumn
  #[sdk(attr(office2010, qname = ":calculatedColumn"))]
  pub calculated_column: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(text_child(office2010, qname = "xne:ST_Sqref/xne:sqref"))]
  pub reference_sequence: crate::simple_type::StringValue,
}
/// Defines the ProtectedRange Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_ProtectedRange/x14:protectedRange")]
pub struct ProtectedRange {
  /// password
  #[sdk(attr(office2010, qname = ":password"))]
  #[sdk(string_length(source = 0u32, min = 2u32, max = 2u32))]
  pub password: Option<crate::simple_type::HexBinaryValue>,
  /// algorithmName
  #[sdk(attr(office2010, qname = ":algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  /// hashValue
  #[sdk(attr(office2010, qname = ":hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// saltValue
  #[sdk(attr(office2010, qname = ":saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// spinCount
  #[sdk(attr(office2010, qname = ":spinCount"))]
  pub spin_count: Option<crate::simple_type::UInt32Value>,
  /// name
  #[sdk(attr(office2010, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// securityDescriptor
  #[sdk(attr(office2010, qname = ":securityDescriptor"))]
  pub security_descriptor: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(office2010, qname = "xne:ST_Sqref/xne:sqref"))]
  pub reference_sequence: crate::simple_type::StringValue,
}
/// Defines the CustomFilter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_CustomFilter/x14:customFilter")]
pub struct CustomFilter {
  /// operator
  #[sdk(attr(office2010, qname = ":operator"))]
  pub operator: Option<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::FilterOperatorValues,
  >,
  /// val
  #[sdk(attr(office2010, qname = ":val"))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Defines the ListItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_ListItem/x14:item")]
pub struct ListItem {
  /// val
  #[sdk(attr(office2010, qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the ListItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_ListItems/x14:itemLst")]
pub struct ListItems {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_ListItem/x14:item"))]
  pub x14_item: Vec<ListItem>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_ExtensionList/x14:extLst"))]
  pub x14_ext_lst: Option<ExtensionList>,
}
/// Defines the Slicer Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_Slicer/x14:slicer")]
pub struct Slicer {
  /// name
  #[sdk(attr(office2010, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// cache
  #[sdk(attr(office2010, qname = ":cache"))]
  pub cache: crate::simple_type::StringValue,
  /// caption
  #[sdk(attr(office2010, qname = ":caption"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// startItem
  #[sdk(attr(office2010, qname = ":startItem"))]
  pub start_item: Option<crate::simple_type::UInt32Value>,
  /// columnCount
  #[sdk(attr(office2010, qname = ":columnCount"))]
  pub column_count: Option<crate::simple_type::UInt32Value>,
  /// showCaption
  #[sdk(attr(office2010, qname = ":showCaption"))]
  pub show_caption: Option<crate::simple_type::BooleanValue>,
  /// level
  #[sdk(attr(office2010, qname = ":level"))]
  pub level: Option<crate::simple_type::UInt32Value>,
  /// style
  #[sdk(attr(office2010, qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// lockedPosition
  #[sdk(attr(office2010, qname = ":lockedPosition"))]
  pub locked_position: Option<crate::simple_type::BooleanValue>,
  /// rowHeight
  #[sdk(attr(office2010, qname = ":rowHeight"))]
  pub row_height: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(office2010, qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the OlapSlicerCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_OlapSlicerCache/x14:olap")]
pub struct OlapSlicerCache {
  /// pivotCacheId
  #[sdk(attr(office2010, qname = ":pivotCacheId"))]
  pub pivot_cache_id: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_OlapSlicerCacheLevelsData/x14:levels"))]
  pub olap_slicer_cache_levels_data: std::boxed::Box<OlapSlicerCacheLevelsData>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_OlapSlicerCacheSelections/x14:selections"))]
  pub olap_slicer_cache_selections: std::boxed::Box<OlapSlicerCacheSelections>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TabularSlicerCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_TabularSlicerCache/x14:tabular")]
pub struct TabularSlicerCache {
  /// pivotCacheId
  #[sdk(attr(office2010, qname = ":pivotCacheId"))]
  pub pivot_cache_id: crate::simple_type::UInt32Value,
  /// sortOrder
  #[sdk(attr(office2010, qname = ":sortOrder"))]
  pub sort_order: Option<TabularSlicerCacheSortOrderValues>,
  /// customListSort
  #[sdk(attr(office2010, qname = ":customListSort"))]
  pub custom_list_sort: Option<crate::simple_type::BooleanValue>,
  /// showMissing
  #[sdk(attr(office2010, qname = ":showMissing"))]
  pub show_missing: Option<crate::simple_type::BooleanValue>,
  /// crossFilter
  #[sdk(attr(office2010, qname = ":crossFilter"))]
  pub cross_filter: Option<SlicerCacheCrossFilterValues>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_TabularSlicerCacheItems/x14:items"))]
  pub tabular_slicer_cache_items: std::boxed::Box<TabularSlicerCacheItems>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the SlicerCachePivotTable Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_SlicerCachePivotTable/x14:pivotTable")]
pub struct SlicerCachePivotTable {
  /// tabId
  #[sdk(attr(office2010, qname = ":tabId"))]
  pub tab_id: crate::simple_type::UInt32Value,
  /// name
  #[sdk(attr(office2010, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// Defines the OlapSlicerCacheItemParent Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_OlapSlicerCacheItemParent/x14:p")]
pub struct OlapSlicerCacheItemParent {
  /// n
  #[sdk(attr(office2010, qname = ":n"))]
  pub name: crate::simple_type::StringValue,
}
/// Defines the OlapSlicerCacheItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_OlapSlicerCacheItem/x14:i")]
pub struct OlapSlicerCacheItem {
  /// n
  #[sdk(attr(office2010, qname = ":n"))]
  pub name: crate::simple_type::StringValue,
  /// c
  #[sdk(attr(office2010, qname = ":c"))]
  pub display_name: Option<crate::simple_type::StringValue>,
  /// nd
  #[sdk(attr(office2010, qname = ":nd"))]
  pub non_display: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_OlapSlicerCacheItemParent/x14:p"))]
  pub x14_p: Vec<OlapSlicerCacheItemParent>,
}
/// Defines the OlapSlicerCacheRange Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_OlapSlicerCacheRange/x14:range")]
pub struct OlapSlicerCacheRange {
  /// startItem
  #[sdk(attr(office2010, qname = ":startItem"))]
  pub start_item: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_OlapSlicerCacheItem/x14:i"))]
  pub x14_i: Vec<OlapSlicerCacheItem>,
}
/// Defines the OlapSlicerCacheRanges Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_OlapSlicerCacheRanges/x14:ranges")]
pub struct OlapSlicerCacheRanges {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_OlapSlicerCacheRange/x14:range"))]
  pub x14_range: Vec<OlapSlicerCacheRange>,
}
/// Defines the OlapSlicerCacheLevelData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_OlapSlicerCacheLevelData/x14:level")]
pub struct OlapSlicerCacheLevelData {
  /// uniqueName
  #[sdk(attr(office2010, qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// sourceCaption
  #[sdk(attr(office2010, qname = ":sourceCaption"))]
  pub source_caption: Option<crate::simple_type::StringValue>,
  /// count
  #[sdk(attr(office2010, qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// sortOrder
  #[sdk(attr(office2010, qname = ":sortOrder"))]
  pub sort_order: Option<OlapSlicerCacheSortOrderValues>,
  /// crossFilter
  #[sdk(attr(office2010, qname = ":crossFilter"))]
  pub cross_filter: Option<SlicerCacheCrossFilterValues>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_OlapSlicerCacheRanges/x14:ranges"))]
  pub olap_slicer_cache_ranges: Option<OlapSlicerCacheRanges>,
}
/// Defines the OlapSlicerCacheLevelsData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_OlapSlicerCacheLevelsData/x14:levels")]
pub struct OlapSlicerCacheLevelsData {
  /// count
  #[sdk(attr(office2010, qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_OlapSlicerCacheLevelData/x14:level"))]
  pub x14_level: Vec<OlapSlicerCacheLevelData>,
}
/// Defines the OlapSlicerCacheSelections Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_OlapSlicerCacheSelections/x14:selections")]
pub struct OlapSlicerCacheSelections {
  /// count
  #[sdk(attr(office2010, qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_OlapSlicerCacheSelection/x14:selection"))]
  pub x14_selection: Vec<OlapSlicerCacheSelection>,
}
/// Defines the OlapSlicerCacheSelection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_OlapSlicerCacheSelection/x14:selection")]
pub struct OlapSlicerCacheSelection {
  /// n
  #[sdk(attr(office2010, qname = ":n"))]
  pub name: crate::simple_type::StringValue,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_OlapSlicerCacheItemParent/x14:p"))]
  pub x14_p: Vec<OlapSlicerCacheItemParent>,
}
/// Defines the TabularSlicerCacheItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_TabularSlicerCacheItems/x14:items")]
pub struct TabularSlicerCacheItems {
  /// count
  #[sdk(attr(office2010, qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(office2010, qname = "x14:CT_TabularSlicerCacheItem/x14:i"))]
  pub x14_i: Vec<TabularSlicerCacheItem>,
}
/// Defines the TabularSlicerCacheItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_TabularSlicerCacheItem/x14:i")]
pub struct TabularSlicerCacheItem {
  /// x
  #[sdk(attr(office2010, qname = ":x"))]
  pub atom: crate::simple_type::UInt32Value,
  /// s
  #[sdk(attr(office2010, qname = ":s"))]
  pub is_selected: Option<crate::simple_type::BooleanValue>,
  /// nd
  #[sdk(attr(office2010, qname = ":nd"))]
  pub non_display: Option<crate::simple_type::BooleanValue>,
}
/// Defines the SlicerCachePivotTables Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_SlicerCachePivotTables/x14:pivotTables")]
pub struct SlicerCachePivotTables {
  /// _
  #[sdk(child(office2010, qname = "x14:CT_SlicerCachePivotTable/x14:pivotTable"))]
  pub x14_pivot_table: Vec<SlicerCachePivotTable>,
}
/// Defines the SlicerCacheData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "x14:CT_SlicerCacheData/x14:data")]
pub struct SlicerCacheData {
  #[sdk(choice(
    qname = "x14:CT_OlapSlicerCache/x14:olap",
    qname = "x14:CT_TabularSlicerCache/x14:tabular"
  ))]
  pub xml_children: Option<SlicerCacheDataChoice>,
}
/// Defines the SlicerCacheDefinitionExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "x:CT_SlicerCacheDefinitionExtensionList/x14:extLst"
)]
pub struct SlicerCacheDefinitionExtensionList {
    pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
    /// _
    #[sdk(child(qname = "x:CT_SlicerCacheDefinitionExtension/x:ext"))]
    pub x_ext: Vec<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SlicerCacheDefinitionExtension,
    >,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PivotUserEditChoice {
  /// Defines the Formula Class.
  #[sdk(text_child(office2010, qname = "x:ST_Formula/xne:f"))]
  XneF(crate::simple_type::StringValue),
  /// Defines the PivotEditValue Class.
  #[sdk(child(office2010, qname = "x14:CT_PivotEditValue/x14:editValue"))]
  X14EditValue(std::boxed::Box<PivotEditValue>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SlicerCacheDataChoice {
  /// Defines the OlapSlicerCache Class.
  #[sdk(child(office2010, qname = "x14:CT_OlapSlicerCache/x14:olap"))]
  X14Olap(std::boxed::Box<OlapSlicerCache>),
  /// Defines the TabularSlicerCache Class.
  #[sdk(child(office2010, qname = "x14:CT_TabularSlicerCache/x14:tabular"))]
  X14Tabular(std::boxed::Box<TabularSlicerCache>),
}
