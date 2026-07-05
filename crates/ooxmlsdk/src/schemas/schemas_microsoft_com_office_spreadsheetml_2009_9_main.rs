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
#[sdk(no_prefix, qname = "x14:conditionalFormattings")]
pub struct ConditionalFormattings {
  /// Defines the ConditionalFormatting Class.
  #[sdk(child(qname = "x14:conditionalFormatting"))]
  pub conditional_formatting: Vec<ConditionalFormatting>,
}
/// Defines the DataValidations Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  no_prefix,
  canonical_namespace_prefix("xm:xne"),
  qname = "x14:dataValidations"
)]
pub struct DataValidations {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// disablePrompts
  #[sdk(attr(qname = ":disablePrompts"))]
  pub disable_prompts: Option<crate::simple_type::BooleanValue>,
  /// xWindow
  #[sdk(attr(qname = ":xWindow"))]
  pub x_window: Option<crate::simple_type::UInt32Value>,
  /// yWindow
  #[sdk(attr(qname = ":yWindow"))]
  pub y_window: Option<crate::simple_type::UInt32Value>,
  /// count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Defines the DataValidation Class.
  #[sdk(child(qname = "x14:dataValidation"))]
  pub data_validation: Vec<DataValidation>,
}
/// Defines the SparklineGroups Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  no_prefix,
  canonical_namespace_prefix("xm:xne"),
  qname = "x14:sparklineGroups"
)]
pub struct SparklineGroups {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the SparklineGroup Class.
  #[sdk(child(qname = "x14:sparklineGroup"))]
  pub sparkline_group: Vec<SparklineGroup>,
}
/// Defines the SlicerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:slicerList")]
pub struct SlicerList {
  /// Defines the SlicerRef Class.
  #[sdk(child(qname = "x14:slicer"))]
  pub slicer_ref: Vec<SlicerRef>,
}
/// Defines the ProtectedRanges Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:protectedRanges")]
pub struct ProtectedRanges {
  /// Defines the ProtectedRange Class.
  #[sdk(child(qname = "x14:protectedRange"))]
  pub protected_range: Vec<ProtectedRange>,
}
/// Defines the IgnoredErrors Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:ignoredErrors")]
pub struct IgnoredErrors {
  /// Defines the IgnoredError Class.
  #[sdk(child(qname = "x14:ignoredError"))]
  pub ignored_error: Vec<IgnoredError>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the DefinedNames Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:definedNames")]
pub struct DefinedNames {
  /// Defines the DefinedName Class.
  #[sdk(child(qname = "x14:definedName"))]
  pub defined_name: Vec<DefinedName>,
}
/// Defines the PivotCaches Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:pivotCaches")]
pub struct PivotCaches {
  /// PivotCache.
  #[sdk(child(qname = "x:pivotCache"))]
  pub pivot_cache: Vec<crate::schemas::x::PivotCache>,
}
/// Defines the SlicerCaches Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:slicerCaches")]
pub struct SlicerCaches {
  /// Defines the SlicerCache Class.
  #[sdk(child(qname = "x14:slicerCache"))]
  pub slicer_cache: Vec<SlicerCache>,
}
/// Defines the WorkbookProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:workbookPr")]
pub struct WorkbookProperties {
  /// defaultImageDpi
  #[sdk(attr(qname = ":defaultImageDpi"))]
  pub default_image_dpi: Option<crate::simple_type::UInt32Value>,
  /// discardImageEditData
  #[sdk(attr(qname = ":discardImageEditData"))]
  pub discard_image_edit_data: Option<crate::simple_type::BooleanValue>,
  /// accuracyVersion
  #[sdk(attr(qname = ":accuracyVersion"))]
  pub accuracy_version: Option<crate::simple_type::UInt32Value>,
}
/// Defines the CalculatedMember Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:calculatedMember")]
pub struct CalculatedMember {
  /// displayFolder
  #[sdk(attr(qname = ":displayFolder"))]
  pub display_folder: Option<crate::simple_type::StringValue>,
  /// flattenHierarchies
  #[sdk(attr(qname = ":flattenHierarchies"))]
  pub flatten_hierarchies: Option<crate::simple_type::BooleanValue>,
  /// dynamicSet
  #[sdk(attr(qname = ":dynamicSet"))]
  pub dynamic_set: Option<crate::simple_type::BooleanValue>,
  /// hierarchizeDistinct
  #[sdk(attr(qname = ":hierarchizeDistinct"))]
  pub hierarchize_distinct: Option<crate::simple_type::BooleanValue>,
  /// mdxLong
  #[sdk(attr(qname = ":mdxLong"))]
  pub mdx_long: Option<crate::simple_type::StringValue>,
  /// Defines the TupleSet Class.
  #[sdk(child(qname = "x14:tupleSet"))]
  pub tuple_set: Option<std::boxed::Box<TupleSet>>,
}
/// Defines the CacheHierarchy Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:cacheHierarchy")]
pub struct CacheHierarchy {
  /// flattenHierarchies
  #[sdk(attr(qname = ":flattenHierarchies"))]
  pub flatten_hierarchies: Option<crate::simple_type::BooleanValue>,
  /// measuresSet
  #[sdk(attr(qname = ":measuresSet"))]
  pub measures_set: Option<crate::simple_type::BooleanValue>,
  /// hierarchizeDistinct
  #[sdk(attr(qname = ":hierarchizeDistinct"))]
  pub hierarchize_distinct: Option<crate::simple_type::BooleanValue>,
  /// ignore
  #[sdk(attr(qname = ":ignore"))]
  pub ignore: Option<crate::simple_type::BooleanValue>,
  /// Defines the SetLevels Class.
  #[sdk(child(qname = "x14:setLevels"))]
  pub set_levels: Option<SetLevels>,
}
/// Defines the DataField Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:dataField")]
pub struct DataField {
  /// pivotShowAs
  #[sdk(attr(qname = ":pivotShowAs"))]
  pub pivot_show_as: Option<PivotShowAsValues>,
  /// sourceField
  #[sdk(attr(qname = ":sourceField"))]
  pub source_field: Option<crate::simple_type::UInt32Value>,
  /// uniqueName
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: Option<crate::simple_type::StringValue>,
}
/// Defines the PivotField Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:pivotField")]
pub struct PivotField {
  /// fillDownLabels
  #[sdk(attr(qname = ":fillDownLabels"))]
  pub fill_down_labels: Option<crate::simple_type::BooleanValue>,
  /// ignore
  #[sdk(attr(qname = ":ignore"))]
  pub ignore: Option<crate::simple_type::BooleanValue>,
}
/// Defines the PivotTableDefinition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:pivotTableDefinition")]
pub struct PivotTableDefinition {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// fillDownLabelsDefault
  #[sdk(attr(qname = ":fillDownLabelsDefault"))]
  pub fill_down_labels_default: Option<crate::simple_type::BooleanValue>,
  /// visualTotalsForSets
  #[sdk(attr(qname = ":visualTotalsForSets"))]
  pub visual_totals_for_sets: Option<crate::simple_type::BooleanValue>,
  /// calculatedMembersInFilters
  #[sdk(attr(qname = ":calculatedMembersInFilters"))]
  pub calculated_members_in_filters: Option<crate::simple_type::BooleanValue>,
  /// altText
  #[sdk(attr(qname = ":altText"))]
  pub alt_text: Option<crate::simple_type::StringValue>,
  /// altTextSummary
  #[sdk(attr(qname = ":altTextSummary"))]
  pub alt_text_summary: Option<crate::simple_type::StringValue>,
  /// enableEdit
  #[sdk(attr(qname = ":enableEdit"))]
  pub enable_edit: Option<crate::simple_type::BooleanValue>,
  /// autoApply
  #[sdk(attr(qname = ":autoApply"))]
  pub auto_apply: Option<crate::simple_type::BooleanValue>,
  /// allocationMethod
  #[sdk(attr(qname = ":allocationMethod"))]
  pub allocation_method: Option<AllocationMethodValues>,
  /// weightExpression
  #[sdk(attr(qname = ":weightExpression"))]
  pub weight_expression: Option<crate::simple_type::StringValue>,
  /// hideValuesRow
  #[sdk(attr(qname = ":hideValuesRow"))]
  pub hide_values_row: Option<crate::simple_type::BooleanValue>,
  /// Defines the PivotEdits Class.
  #[sdk(child(qname = "x14:pivotEdits"))]
  pub pivot_edits: Option<PivotEdits>,
  /// Defines the PivotChanges Class.
  #[sdk(child(qname = "x14:pivotChanges"))]
  pub pivot_changes: Option<PivotChanges>,
  /// Defines the ConditionalFormats Class.
  #[sdk(child(qname = "x14:conditionalFormats"))]
  pub conditional_formats: Option<ConditionalFormats>,
}
/// Defines the PivotCacheDefinition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:pivotCacheDefinition")]
pub struct PivotCacheDefinition {
  /// slicerData
  #[sdk(attr(qname = ":slicerData"))]
  pub slicer_data: Option<crate::simple_type::BooleanValue>,
  /// pivotCacheId
  #[sdk(attr(qname = ":pivotCacheId"))]
  pub pivot_cache_id: Option<crate::simple_type::UInt32Value>,
  /// supportSubqueryNonVisual
  #[sdk(attr(qname = ":supportSubqueryNonVisual"))]
  pub support_subquery_non_visual: Option<crate::simple_type::BooleanValue>,
  /// supportSubqueryCalcMem
  #[sdk(attr(qname = ":supportSubqueryCalcMem"))]
  pub support_subquery_calc_mem: Option<crate::simple_type::BooleanValue>,
  /// supportAddCalcMems
  #[sdk(attr(qname = ":supportAddCalcMems"))]
  pub support_add_calc_mems: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Connection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:connection")]
pub struct Connection {
  /// culture
  #[sdk(attr(qname = ":culture"))]
  pub culture: Option<crate::simple_type::StringValue>,
  /// embeddedDataId
  #[sdk(attr(qname = ":embeddedDataId"))]
  pub embedded_data_id: Option<crate::simple_type::StringValue>,
  /// Defines the CalculatedMembers Class.
  #[sdk(child(qname = "x14:calculatedMembers"))]
  pub calculated_members: Option<CalculatedMembers>,
}
/// Defines the Table Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:table")]
pub struct Table {
  /// altText
  #[sdk(attr(qname = ":altText"))]
  pub alt_text: Option<crate::simple_type::StringValue>,
  /// altTextSummary
  #[sdk(attr(qname = ":altTextSummary"))]
  pub alt_text_summary: Option<crate::simple_type::StringValue>,
}
/// Defines the SlicerStyles Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:slicerStyles")]
pub struct SlicerStyles {
  /// defaultSlicerStyle
  #[sdk(attr(qname = ":defaultSlicerStyle"))]
  pub default_slicer_style: crate::simple_type::StringValue,
  /// Defines the SlicerStyle Class.
  #[sdk(child(qname = "x14:slicerStyle"))]
  pub slicer_style: Vec<SlicerStyle>,
}
/// Defines the DifferentialFormats Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:dxfs")]
pub struct DifferentialFormats {
  /// Format Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Formatting.
  #[sdk(child(qname = "x:dxf"))]
  pub differential_format: Vec<crate::schemas::x::DifferentialFormat>,
}
/// Defines the OleItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:oleItem")]
pub struct OleItem {
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// icon
  #[sdk(attr(qname = ":icon"))]
  pub icon: Option<crate::simple_type::BooleanValue>,
  /// advise
  #[sdk(attr(qname = ":advise"))]
  pub advise: Option<crate::simple_type::BooleanValue>,
  /// preferPic
  #[sdk(attr(qname = ":preferPic"))]
  pub prefer_picture: Option<crate::simple_type::BooleanValue>,
  /// Defines the DdeValues Class.
  #[sdk(child(qname = "x14:values"))]
  pub dde_values: Option<DdeValues>,
}
/// Defines the PivotHierarchy Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:pivotHierarchy")]
pub struct PivotHierarchy {
  /// ignore
  #[sdk(attr(qname = ":ignore"))]
  pub ignore: Option<crate::simple_type::BooleanValue>,
}
/// Defines the CacheField Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:cacheField")]
pub struct CacheField {
  /// ignore
  #[sdk(attr(qname = ":ignore"))]
  pub ignore: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Id Class.
pub type Id = crate::simple_type::StringValue;
/// Defines the IconFilter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:iconFilter")]
pub struct IconFilter {
  /// iconSet
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set: IconSetTypeValues,
  /// iconId
  #[sdk(attr(qname = ":iconId"))]
  pub icon_id: crate::simple_type::UInt32Value,
}
/// Defines the Filter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:filter")]
pub struct Filter {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Defines the CustomFilters Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:customFilters")]
pub struct CustomFilters {
  /// and
  #[sdk(attr(qname = ":and"))]
  pub and: Option<crate::simple_type::BooleanValue>,
  /// Defines the CustomFilter Class.
  #[sdk(child(qname = "x14:customFilter"))]
  pub custom_filter: Vec<CustomFilter>,
}
/// Defines the SortCondition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:sortCondition")]
pub struct SortCondition {
  /// descending
  #[sdk(attr(qname = ":descending"))]
  pub descending: Option<crate::simple_type::BooleanValue>,
  /// sortBy
  #[sdk(attr(qname = ":sortBy"))]
  pub sort_by: Option<crate::schemas::x::SortByValues>,
  /// ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// customList
  #[sdk(attr(qname = ":customList"))]
  pub custom_list: Option<crate::simple_type::StringValue>,
  /// dxfId
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
  /// iconSet
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set: Option<IconSetTypeValues>,
  /// iconId
  #[sdk(attr(qname = ":iconId"))]
  pub icon_id: Option<crate::simple_type::UInt32Value>,
}
/// Defines the SourceConnection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:sourceConnection")]
pub struct SourceConnection {
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// Defines the DatastoreItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, xml_header, qname = "x14:datastoreItem")]
pub struct DatastoreItem {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the FormControlProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, xml_header, qname = "x14:formControlPr")]
pub struct FormControlProperties {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// objectType
  #[sdk(attr(qname = ":objectType"))]
  #[sdk(string_format(kind = "token"))]
  pub object_type: Option<ObjectTypeValues>,
  /// checked
  #[sdk(attr(qname = ":checked"))]
  #[sdk(string_format(kind = "token"))]
  pub checked: Option<CheckedValues>,
  /// colored
  #[sdk(attr(qname = ":colored"))]
  pub colored: Option<crate::simple_type::BooleanValue>,
  /// dropLines
  #[sdk(attr(qname = ":dropLines"))]
  pub drop_lines: Option<crate::simple_type::UInt32Value>,
  /// dropStyle
  #[sdk(attr(qname = ":dropStyle"))]
  #[sdk(string_format(kind = "token"))]
  pub drop_style: Option<DropStyleValues>,
  /// dx
  #[sdk(attr(qname = ":dx"))]
  pub scroll_bar_width: Option<crate::simple_type::UInt32Value>,
  /// firstButton
  #[sdk(attr(qname = ":firstButton"))]
  pub first_button: Option<crate::simple_type::BooleanValue>,
  /// fmlaGroup
  #[sdk(attr(qname = ":fmlaGroup"))]
  pub fmla_group: Option<crate::simple_type::StringValue>,
  /// fmlaLink
  #[sdk(attr(qname = ":fmlaLink"))]
  pub fmla_link: Option<crate::simple_type::StringValue>,
  /// fmlaRange
  #[sdk(attr(qname = ":fmlaRange"))]
  pub fmla_range: Option<crate::simple_type::StringValue>,
  /// fmlaTxbx
  #[sdk(attr(qname = ":fmlaTxbx"))]
  pub fmla_textbox: Option<crate::simple_type::StringValue>,
  /// horiz
  #[sdk(attr(qname = ":horiz"))]
  pub horizontal: Option<crate::simple_type::BooleanValue>,
  /// inc
  #[sdk(attr(qname = ":inc"))]
  pub incremental: Option<crate::simple_type::UInt32Value>,
  /// justLastX
  #[sdk(attr(qname = ":justLastX"))]
  pub just_last_x: Option<crate::simple_type::BooleanValue>,
  /// lockText
  #[sdk(attr(qname = ":lockText"))]
  pub lock_text: Option<crate::simple_type::BooleanValue>,
  /// max
  #[sdk(attr(qname = ":max"))]
  pub max: Option<crate::simple_type::UInt32Value>,
  /// min
  #[sdk(attr(qname = ":min"))]
  pub min: Option<crate::simple_type::UInt32Value>,
  /// multiSel
  #[sdk(attr(qname = ":multiSel"))]
  pub multiple_selection: Option<crate::simple_type::StringValue>,
  /// noThreeD
  #[sdk(attr(qname = ":noThreeD"))]
  pub no_three_d: Option<crate::simple_type::BooleanValue>,
  /// noThreeD2
  #[sdk(attr(qname = ":noThreeD2"))]
  pub no_three_d2: Option<crate::simple_type::BooleanValue>,
  /// page
  #[sdk(attr(qname = ":page"))]
  pub page: Option<crate::simple_type::UInt32Value>,
  /// sel
  #[sdk(attr(qname = ":sel"))]
  pub selected: Option<crate::simple_type::UInt32Value>,
  /// seltype
  #[sdk(attr(qname = ":seltype"))]
  #[sdk(string_format(kind = "token"))]
  pub selection_type: Option<SelectionTypeValues>,
  /// textHAlign
  #[sdk(attr(qname = ":textHAlign"))]
  pub text_horizontal_align: Option<TextHorizontalAlignmentValues>,
  /// textVAlign
  #[sdk(attr(qname = ":textVAlign"))]
  pub text_vertical_align: Option<TextVerticalAlignmentValues>,
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::UInt32Value>,
  /// widthMin
  #[sdk(attr(qname = ":widthMin"))]
  pub minimum_width: Option<crate::simple_type::UInt32Value>,
  /// editVal
  #[sdk(attr(qname = ":editVal"))]
  #[sdk(string_format(kind = "token"))]
  pub edit_val: Option<EditValidationValues>,
  /// multiLine
  #[sdk(attr(qname = ":multiLine"))]
  pub multiple_lines: Option<crate::simple_type::BooleanValue>,
  /// verticalBar
  #[sdk(attr(qname = ":verticalBar"))]
  pub vertical_bar: Option<crate::simple_type::BooleanValue>,
  /// passwordEdit
  #[sdk(attr(qname = ":passwordEdit"))]
  pub password_edit: Option<crate::simple_type::BooleanValue>,
  /// Defines the ListItems Class.
  #[sdk(child(qname = "x14:itemLst"))]
  pub list_items: Option<std::boxed::Box<ListItems>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Slicers Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, xml_header, qname = "x14:slicers")]
pub struct Slicers {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the Slicer Class.
  #[sdk(child(qname = "x14:slicer"))]
  pub slicer: Vec<Slicer>,
}
/// Defines the SlicerCacheDefinition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, xml_header, qname = "x14:slicerCacheDefinition")]
pub struct SlicerCacheDefinition {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// sourceName
  #[sdk(attr(qname = ":sourceName"))]
  pub source_name: crate::simple_type::StringValue,
  /// Defines the SlicerCachePivotTables Class.
  #[sdk(child(qname = "x14:pivotTables"))]
  pub slicer_cache_pivot_tables: Option<SlicerCachePivotTables>,
  /// Defines the SlicerCacheData Class.
  #[sdk(child(qname = "x14:data"))]
  pub slicer_cache_data: Option<std::boxed::Box<SlicerCacheData>>,
  /// Defines the SlicerCacheDefinitionExtensionList Class.
  #[sdk(child(qname = "x14:extLst"))]
  pub slicer_cache_definition_extension_list: Option<SlicerCacheDefinitionExtensionList>,
}
/// Defines the ConditionalFormatting Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  no_prefix,
  canonical_namespace_prefix("xm:xne"),
  qname = "x14:conditionalFormatting"
)]
pub struct ConditionalFormatting {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// pivot
  #[sdk(attr(qname = ":pivot"))]
  pub pivot: Option<crate::simple_type::BooleanValue>,
  /// Defines the ConditionalFormattingRule Class.
  #[sdk(child(qname = "x14:cfRule"))]
  pub conditional_formatting_rule: Vec<ConditionalFormattingRule>,
  /// Defines the ReferenceSequence Class.
  #[sdk(text_child(list, qname = "xne:sqref"))]
  pub reference_sequence: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ConditionalFormattingRule Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:cfRule")]
pub struct ConditionalFormattingRule {
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<crate::schemas::x::ConditionalFormatValues>,
  /// priority
  #[sdk(attr(qname = ":priority"))]
  pub priority: Option<crate::simple_type::Int32Value>,
  /// stopIfTrue
  #[sdk(attr(qname = ":stopIfTrue"))]
  pub stop_if_true: Option<crate::simple_type::BooleanValue>,
  /// aboveAverage
  #[sdk(attr(qname = ":aboveAverage"))]
  pub above_average: Option<crate::simple_type::BooleanValue>,
  /// percent
  #[sdk(attr(qname = ":percent"))]
  pub percent: Option<crate::simple_type::BooleanValue>,
  /// bottom
  #[sdk(attr(qname = ":bottom"))]
  pub bottom: Option<crate::simple_type::BooleanValue>,
  /// operator
  #[sdk(attr(qname = ":operator"))]
  pub operator: Option<crate::schemas::x::ConditionalFormattingOperatorValues>,
  /// text
  #[sdk(attr(qname = ":text"))]
  pub text: Option<crate::simple_type::StringValue>,
  /// timePeriod
  #[sdk(attr(qname = ":timePeriod"))]
  pub time_period: Option<crate::schemas::x::TimePeriodValues>,
  /// rank
  #[sdk(attr(qname = ":rank"))]
  pub rank: Option<crate::simple_type::UInt32Value>,
  /// stdDev
  #[sdk(attr(qname = ":stdDev"))]
  pub standard_deviation: Option<crate::simple_type::Int32Value>,
  /// equalAverage
  #[sdk(attr(qname = ":equalAverage"))]
  pub equal_average: Option<crate::simple_type::BooleanValue>,
  /// activePresent
  #[sdk(attr(qname = ":activePresent"))]
  pub active_present: Option<crate::simple_type::BooleanValue>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Defines the Formula Class.
  #[sdk(text_child(qname = "xne:f"))]
  pub formula: Vec<crate::schemas::xne::Formula>,
  /// Defines the ColorScale Class.
  #[sdk(child(qname = "x14:colorScale"))]
  pub color_scale: Option<ColorScale>,
  /// Defines the DataBar Class.
  #[sdk(child(qname = "x14:dataBar"))]
  pub data_bar: Option<std::boxed::Box<DataBar>>,
  /// Defines the IconSet Class.
  #[sdk(child(qname = "x14:iconSet"))]
  pub icon_set: Option<IconSet>,
  /// Defines the DifferentialType Class.
  #[sdk(child(qname = "x14:dxf"))]
  pub differential_type: Option<std::boxed::Box<DifferentialType>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "x:ext"))]
  pub extension: Vec<crate::schemas::x::Extension>,
}
/// Defines the DataValidation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:dataValidation")]
pub struct DataValidation {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
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
  /// Defines the DataValidationForumla1 Class.
  #[sdk(child(qname = "x14:formula1"))]
  pub data_validation_forumla1: Option<DataValidationForumla1>,
  /// Defines the DataValidationForumla2 Class.
  #[sdk(child(qname = "x14:formula2"))]
  pub data_validation_forumla2: Option<DataValidationForumla2>,
  /// Defines the ReferenceSequence Class.
  #[sdk(text_child(list, qname = "xne:sqref"))]
  pub reference_sequence: Vec<crate::simple_type::StringValue>,
}
/// Defines the DataValidationForumla1 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:formula1")]
pub struct DataValidationForumla1 {
  /// Defines the Formula Class.
  #[sdk(text_child(qname = "xne:f"))]
  pub formula: crate::schemas::xne::Formula,
}
/// Defines the DataValidationForumla2 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:formula2")]
pub struct DataValidationForumla2 {
  /// Defines the Formula Class.
  #[sdk(text_child(qname = "xne:f"))]
  pub formula: crate::schemas::xne::Formula,
}
/// Defines the SparklineGroup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:sparklineGroup")]
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
  pub r#type: Option<SparklineTypeValues>,
  /// dateAxis
  #[sdk(attr(qname = ":dateAxis"))]
  pub date_axis: Option<crate::simple_type::BooleanValue>,
  /// displayEmptyCellsAs
  #[sdk(attr(qname = ":displayEmptyCellsAs"))]
  pub display_empty_cells_as: Option<DisplayBlanksAsValues>,
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
  pub min_axis_type: Option<SparklineAxisMinMaxValues>,
  /// maxAxisType
  #[sdk(attr(qname = ":maxAxisType"))]
  pub max_axis_type: Option<SparklineAxisMinMaxValues>,
  /// rightToLeft
  #[sdk(attr(qname = ":rightToLeft"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// Sparkline Group Unique ID
  #[sdk(attr(qname = "xr2:uid"))]
  pub uid: Option<crate::simple_type::StringValue>,
  /// Defines the SeriesColor Class.
  #[sdk(child(qname = "x14:colorSeries"))]
  pub series_color: Option<SeriesColor>,
  /// Defines the NegativeColor Class.
  #[sdk(child(qname = "x14:colorNegative"))]
  pub negative_color: Option<NegativeColor>,
  /// Defines the AxisColor Class.
  #[sdk(child(qname = "x14:colorAxis"))]
  pub axis_color: Option<AxisColor>,
  /// Defines the MarkersColor Class.
  #[sdk(child(qname = "x14:colorMarkers"))]
  pub markers_color: Option<MarkersColor>,
  /// Defines the FirstMarkerColor Class.
  #[sdk(child(qname = "x14:colorFirst"))]
  pub first_marker_color: Option<FirstMarkerColor>,
  /// Defines the LastMarkerColor Class.
  #[sdk(child(qname = "x14:colorLast"))]
  pub last_marker_color: Option<LastMarkerColor>,
  /// Defines the HighMarkerColor Class.
  #[sdk(child(qname = "x14:colorHigh"))]
  pub high_marker_color: Option<HighMarkerColor>,
  /// Defines the LowMarkerColor Class.
  #[sdk(child(qname = "x14:colorLow"))]
  pub low_marker_color: Option<LowMarkerColor>,
  /// Defines the Formula Class.
  #[sdk(text_child(qname = "xne:f"))]
  pub formula: Option<crate::schemas::xne::Formula>,
  /// Defines the Sparklines Class.
  #[sdk(child(qname = "x14:sparklines"))]
  pub sparklines: Sparklines,
}
/// Defines the SeriesColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:colorSeries")]
pub struct SeriesColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
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
#[sdk(no_prefix, qname = "x14:colorNegative")]
pub struct NegativeColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
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
#[sdk(no_prefix, qname = "x14:colorAxis")]
pub struct AxisColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
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
#[sdk(no_prefix, qname = "x14:colorMarkers")]
pub struct MarkersColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
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
#[sdk(no_prefix, qname = "x14:colorFirst")]
pub struct FirstMarkerColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
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
#[sdk(no_prefix, qname = "x14:colorLast")]
pub struct LastMarkerColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
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
#[sdk(no_prefix, qname = "x14:colorHigh")]
pub struct HighMarkerColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
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
#[sdk(no_prefix, qname = "x14:colorLow")]
pub struct LowMarkerColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
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
#[sdk(no_prefix, qname = "x14:color")]
pub struct Color {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
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
#[sdk(no_prefix, qname = "x14:fillColor")]
pub struct FillColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
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
#[sdk(no_prefix, qname = "x14:borderColor")]
pub struct BorderColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
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
#[sdk(no_prefix, qname = "x14:negativeFillColor")]
pub struct NegativeFillColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
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
#[sdk(no_prefix, qname = "x14:negativeBorderColor")]
pub struct NegativeBorderColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
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
#[sdk(no_prefix, qname = "x14:axisColor")]
pub struct BarAxisColor {
  /// Automatic
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
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
#[sdk(no_prefix, qname = "x14:sparklines")]
pub struct Sparklines {
  /// Defines the Sparkline Class.
  #[sdk(child(qname = "x14:sparkline"))]
  pub sparkline: Vec<Sparkline>,
}
/// Defines the Sparkline Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:sparkline")]
pub struct Sparkline {
  /// Defines the Formula Class.
  #[sdk(text_child(qname = "xne:f"))]
  pub formula: Option<crate::schemas::xne::Formula>,
  /// Defines the ReferenceSequence Class.
  #[sdk(text_child(list, qname = "xne:sqref"))]
  pub reference_sequence: Vec<crate::simple_type::StringValue>,
}
/// Defines the SlicerRef Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:slicer")]
pub struct SlicerRef {
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the SlicerCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:slicerCache")]
pub struct SlicerCache {
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the DefinedName Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:definedName")]
pub struct DefinedName {
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Defines the ArgumentDescriptions Class.
  #[sdk(child(qname = "x14:argumentDescriptions"))]
  pub argument_descriptions: Option<ArgumentDescriptions>,
}
/// Defines the ArgumentDescriptions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:argumentDescriptions")]
pub struct ArgumentDescriptions {
  /// count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Defines the ArgumentDescription Class.
  #[sdk(child(qname = "x14:argumentDescription"))]
  pub argument_description: Vec<ArgumentDescription>,
}
/// Defines the ArgumentDescription Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:argumentDescription")]
pub struct ArgumentDescription {
  /// index
  #[sdk(attr(qname = ":index"))]
  pub index: crate::simple_type::UInt32Value,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the TupleSet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:tupleSet")]
pub struct TupleSet {
  /// rowCount
  #[sdk(attr(qname = ":rowCount"))]
  pub row_count: Option<crate::simple_type::UInt32Value>,
  /// columnCount
  #[sdk(attr(qname = ":columnCount"))]
  pub column_count: Option<crate::simple_type::UInt32Value>,
  /// Defines the TupleSetHeaders Class.
  #[sdk(child(qname = "x14:headers"))]
  pub tuple_set_headers: TupleSetHeaders,
  /// Defines the TupleSetRows Class.
  #[sdk(child(qname = "x14:rows"))]
  pub tuple_set_rows: TupleSetRows,
}
/// Defines the TupleSetHeaders Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:headers")]
pub struct TupleSetHeaders {
  /// Defines the TupleSetHeader Class.
  #[sdk(child(qname = "x14:header"))]
  pub tuple_set_header: Vec<TupleSetHeader>,
}
/// Defines the TupleSetRows Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:rows")]
pub struct TupleSetRows {
  /// Defines the TupleSetRow Class.
  #[sdk(child(qname = "x14:row"))]
  pub tuple_set_row: Vec<TupleSetRow>,
}
/// Defines the TupleSetHeader Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:header")]
pub struct TupleSetHeader {
  /// uniqueName
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: Option<crate::simple_type::StringValue>,
  /// hierarchyName
  #[sdk(attr(qname = ":hierarchyName"))]
  pub hierarchy_name: Option<crate::simple_type::StringValue>,
}
/// Defines the TupleSetRow Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:row")]
pub struct TupleSetRow {
  /// Defines the TupleSetRowItem Class.
  #[sdk(child(qname = "x14:rowItem"))]
  pub tuple_set_row_item: Vec<TupleSetRowItem>,
}
/// Defines the TupleSetRowItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:rowItem")]
pub struct TupleSetRowItem {
  /// u
  #[sdk(attr(qname = ":u"))]
  pub unique_name: Option<crate::simple_type::StringValue>,
  /// d
  #[sdk(attr(qname = ":d"))]
  pub display_name: Option<crate::simple_type::StringValue>,
}
/// Defines the SetLevel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:setLevel")]
pub struct SetLevel {
  /// hierarchy
  #[sdk(attr(qname = ":hierarchy"))]
  pub hierarchy: crate::simple_type::Int32Value,
}
/// Defines the SetLevels Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:setLevels")]
pub struct SetLevels {
  /// count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Defines the SetLevel Class.
  #[sdk(child(qname = "x14:setLevel"))]
  pub set_level: Vec<SetLevel>,
}
/// Defines the ColorScale Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:colorScale")]
pub struct ColorScale {
  /// Defines the ConditionalFormattingValueObject Class.
  #[sdk(child(qname = "x14:cfvo"))]
  pub conditional_formatting_value_object: Vec<ConditionalFormattingValueObject>,
  /// Defines the Color Class.
  #[sdk(child(qname = "x14:color"))]
  pub color: Vec<Color>,
}
/// Defines the DataBar Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:dataBar")]
pub struct DataBar {
  /// minLength
  #[sdk(attr(qname = ":minLength"))]
  pub min_length: Option<crate::simple_type::UInt32Value>,
  /// maxLength
  #[sdk(attr(qname = ":maxLength"))]
  pub max_length: Option<crate::simple_type::UInt32Value>,
  /// showValue
  #[sdk(attr(qname = ":showValue"))]
  pub show_value: Option<crate::simple_type::BooleanValue>,
  /// border
  #[sdk(attr(qname = ":border"))]
  pub border: Option<crate::simple_type::BooleanValue>,
  /// gradient
  #[sdk(attr(qname = ":gradient"))]
  pub gradient: Option<crate::simple_type::BooleanValue>,
  /// direction
  #[sdk(attr(qname = ":direction"))]
  pub direction: Option<DataBarDirectionValues>,
  /// negativeBarColorSameAsPositive
  #[sdk(attr(qname = ":negativeBarColorSameAsPositive"))]
  pub negative_bar_color_same_as_positive: Option<crate::simple_type::BooleanValue>,
  /// negativeBarBorderColorSameAsPositive
  #[sdk(attr(qname = ":negativeBarBorderColorSameAsPositive"))]
  pub negative_bar_border_color_same_as_positive: Option<crate::simple_type::BooleanValue>,
  /// axisPosition
  #[sdk(attr(qname = ":axisPosition"))]
  pub axis_position: Option<DataBarAxisPositionValues>,
  /// Defines the ConditionalFormattingValueObject Class.
  #[sdk(child(qname = "x14:cfvo"))]
  pub conditional_formatting_value_object: Vec<ConditionalFormattingValueObject>,
  /// Defines the FillColor Class.
  #[sdk(child(qname = "x14:fillColor"))]
  pub fill_color: Option<FillColor>,
  /// Defines the BorderColor Class.
  #[sdk(child(qname = "x14:borderColor"))]
  pub border_color: Option<BorderColor>,
  /// Defines the NegativeFillColor Class.
  #[sdk(child(qname = "x14:negativeFillColor"))]
  pub negative_fill_color: Option<NegativeFillColor>,
  /// Defines the NegativeBorderColor Class.
  #[sdk(child(qname = "x14:negativeBorderColor"))]
  pub negative_border_color: Option<NegativeBorderColor>,
  /// Defines the BarAxisColor Class.
  #[sdk(child(qname = "x14:axisColor"))]
  pub bar_axis_color: Option<BarAxisColor>,
}
/// Defines the IconSet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:iconSet")]
pub struct IconSet {
  /// iconSet
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set_types: Option<IconSetTypeValues>,
  /// showValue
  #[sdk(attr(qname = ":showValue"))]
  pub show_value: Option<crate::simple_type::BooleanValue>,
  /// percent
  #[sdk(attr(qname = ":percent"))]
  pub percent: Option<crate::simple_type::BooleanValue>,
  /// reverse
  #[sdk(attr(qname = ":reverse"))]
  pub reverse: Option<crate::simple_type::BooleanValue>,
  /// custom
  #[sdk(attr(qname = ":custom"))]
  pub custom: Option<crate::simple_type::BooleanValue>,
  /// Defines the ConditionalFormattingValueObject Class.
  #[sdk(child(qname = "x14:cfvo"))]
  pub conditional_formatting_value_object: Vec<ConditionalFormattingValueObject>,
  /// Defines the ConditionalFormattingIcon Class.
  #[sdk(child(qname = "x14:cfIcon"))]
  pub conditional_formatting_icon: Vec<ConditionalFormattingIcon>,
}
/// Defines the DifferentialType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, extra_xmlns("x"), qname = "x14:dxf")]
pub struct DifferentialType {
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
/// Defines the ConditionalFormattingValueObject Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:cfvo")]
pub struct ConditionalFormattingValueObject {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: ConditionalFormattingValueObjectTypeValues,
  /// gte
  #[sdk(attr(qname = ":gte"))]
  pub greater_than_or_equal: Option<crate::simple_type::BooleanValue>,
  /// Defines the Formula Class.
  #[sdk(text_child(qname = "xne:f"))]
  pub formula: Option<crate::schemas::xne::Formula>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ConditionalFormattingIcon Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:cfIcon")]
pub struct ConditionalFormattingIcon {
  /// iconSet
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set: IconSetTypeValues,
  /// iconId
  #[sdk(attr(qname = ":iconId"))]
  pub icon_id: crate::simple_type::UInt32Value,
}
/// Defines the PivotEdits Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:pivotEdits")]
pub struct PivotEdits {
  /// Defines the PivotEdit Class.
  #[sdk(child(qname = "x14:pivotEdit"))]
  pub pivot_edit: Vec<PivotEdit>,
}
/// Defines the PivotChanges Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:pivotChanges")]
pub struct PivotChanges {
  /// Defines the PivotChange Class.
  #[sdk(child(qname = "x14:pivotChange"))]
  pub pivot_change: Vec<PivotChange>,
}
/// Defines the ConditionalFormats Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:conditionalFormats")]
pub struct ConditionalFormats {
  /// count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Defines the ConditionalFormat Class.
  #[sdk(child(qname = "x14:conditionalFormat"))]
  pub conditional_format: Vec<ConditionalFormat>,
}
/// Defines the CalculatedMembers Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:calculatedMembers")]
pub struct CalculatedMembers {
  /// Calculated Members Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Calculated Member.
  #[sdk(child(qname = "x:calculatedMember"))]
  pub calculated_member: Vec<crate::schemas::x::CalculatedMember>,
}
/// Defines the PivotEdit Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:pivotEdit")]
pub struct PivotEdit {
  /// Defines the PivotUserEdit Class.
  #[sdk(child(qname = "x14:userEdit"))]
  pub pivot_user_edit: std::boxed::Box<PivotUserEdit>,
  /// Defines the TupleItems Class.
  #[sdk(child(qname = "x14:tupleItems"))]
  pub tuple_items: TupleItems,
  /// Defines the PivotArea Class.
  #[sdk(child(qname = "x14:pivotArea"))]
  pub pivot_area: std::boxed::Box<PivotArea>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotUserEdit Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:userEdit")]
pub struct PivotUserEdit {
  #[sdk(
        choice(
            text_child(variant = Formula, qname = "xne:f"),
            child(variant = PivotEditValue, qname = "x14:editValue")
        )
    )]
  pub pivot_user_edit_choice: Option<PivotUserEditChoice>,
}
/// Defines the TupleItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:tupleItems")]
pub struct TupleItems {
  /// Defines the Xstring Class.
  #[sdk(text_child(qname = "x14:tupleItem"))]
  pub xstring: Vec<Xstring>,
}
/// Defines the PivotArea Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:pivotArea")]
pub struct PivotArea {
  /// Field Index
  #[sdk(attr(qname = ":field"))]
  pub field: Option<crate::simple_type::Int32Value>,
  /// Rule Type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<crate::schemas::x::PivotAreaValues>,
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
  pub axis: Option<crate::schemas::x::PivotTableAxisValues>,
  /// Field Position
  #[sdk(attr(qname = ":fieldPosition"))]
  pub field_position: Option<crate::simple_type::UInt32Value>,
  /// References
  #[sdk(child(qname = "x:references"))]
  pub pivot_area_references: Option<crate::schemas::x::PivotAreaReferences>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:extLst"))]
  pub extension_list: Option<crate::schemas::x::ExtensionList>,
}
/// Defines the PivotChange Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:pivotChange")]
pub struct PivotChange {
  /// allocationMethod
  #[sdk(attr(qname = ":allocationMethod"))]
  pub allocation_method: Option<AllocationMethodValues>,
  /// weightExpression
  #[sdk(attr(qname = ":weightExpression"))]
  pub weight_expression: Option<crate::simple_type::StringValue>,
  /// Defines the PivotEditValue Class.
  #[sdk(child(qname = "x14:editValue"))]
  pub pivot_edit_value: PivotEditValue,
  /// Defines the TupleItems Class.
  #[sdk(child(qname = "x14:tupleItems"))]
  pub tuple_items: TupleItems,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotEditValue Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:editValue")]
pub struct PivotEditValue {
  /// valueType
  #[sdk(attr(qname = ":valueType"))]
  pub value_type: PivotEditValueTypeValues,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Xstring Class.
pub type Xstring = crate::simple_type::StringValue;
/// Defines the SlicerStyleElements Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:slicerStyleElements")]
pub struct SlicerStyleElements {
  /// Defines the SlicerStyleElement Class.
  #[sdk(child(qname = "x14:slicerStyleElement"))]
  pub slicer_style_element: Vec<SlicerStyleElement>,
}
/// Defines the DdeValues Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:values")]
pub struct DdeValues {
  /// Rows
  #[sdk(attr(qname = ":rows"))]
  pub rows: Option<crate::simple_type::UInt32Value>,
  /// Columns
  #[sdk(attr(qname = ":cols"))]
  pub columns: Option<crate::simple_type::UInt32Value>,
  /// Value.
  #[sdk(child(qname = "x:value"))]
  pub value: Vec<crate::schemas::x::Value>,
}
/// Defines the ConditionalFormat Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:conditionalFormat")]
pub struct ConditionalFormat {
  /// scope
  #[sdk(attr(qname = ":scope"))]
  pub scope: Option<crate::schemas::x::ScopeValues>,
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<crate::schemas::x::RuleValues>,
  /// priority
  #[sdk(attr(qname = ":priority"))]
  pub priority: Option<crate::simple_type::UInt32Value>,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the PivotAreas Class.
  #[sdk(child(qname = "x14:pivotAreas"))]
  pub pivot_areas: Option<PivotAreas>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotAreas Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:pivotAreas")]
pub struct PivotAreas {
  /// Pivot Area Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Calculated Item Location.
  #[sdk(child(qname = "x:pivotArea"))]
  pub pivot_area: Vec<crate::schemas::x::PivotArea>,
}
/// Defines the SlicerStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:slicerStyle")]
pub struct SlicerStyle {
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Defines the SlicerStyleElements Class.
  #[sdk(child(qname = "x14:slicerStyleElements"))]
  pub slicer_style_elements: Option<SlicerStyleElements>,
}
/// Defines the SlicerStyleElement Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:slicerStyleElement")]
pub struct SlicerStyleElement {
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: SlicerStyleTypeValues,
  /// dxfId
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
}
/// Defines the IgnoredError Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:ignoredError")]
pub struct IgnoredError {
  /// evalError
  #[sdk(attr(qname = ":evalError"))]
  pub eval_error: Option<crate::simple_type::BooleanValue>,
  /// twoDigitTextYear
  #[sdk(attr(qname = ":twoDigitTextYear"))]
  pub two_digit_text_year: Option<crate::simple_type::BooleanValue>,
  /// numberStoredAsText
  #[sdk(attr(qname = ":numberStoredAsText"))]
  pub number_stored_as_text: Option<crate::simple_type::BooleanValue>,
  /// formula
  #[sdk(attr(qname = ":formula"))]
  pub formula: Option<crate::simple_type::BooleanValue>,
  /// formulaRange
  #[sdk(attr(qname = ":formulaRange"))]
  pub formula_range: Option<crate::simple_type::BooleanValue>,
  /// unlockedFormula
  #[sdk(attr(qname = ":unlockedFormula"))]
  pub unlocked_formula: Option<crate::simple_type::BooleanValue>,
  /// emptyCellReference
  #[sdk(attr(qname = ":emptyCellReference"))]
  pub empty_cell_reference: Option<crate::simple_type::BooleanValue>,
  /// listDataValidation
  #[sdk(attr(qname = ":listDataValidation"))]
  pub list_data_validation: Option<crate::simple_type::BooleanValue>,
  /// calculatedColumn
  #[sdk(attr(qname = ":calculatedColumn"))]
  pub calculated_column: Option<crate::simple_type::BooleanValue>,
  /// Defines the ReferenceSequence Class.
  #[sdk(text_child(list, qname = "xne:sqref"))]
  pub reference_sequence: Vec<crate::simple_type::StringValue>,
}
/// Defines the ProtectedRange Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:protectedRange")]
pub struct ProtectedRange {
  /// password
  #[sdk(attr(qname = ":password"))]
  #[sdk(string_length(min = 2u32, max = 2u32))]
  pub password: Option<crate::simple_type::HexBinaryValue>,
  /// algorithmName
  #[sdk(attr(qname = ":algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  /// hashValue
  #[sdk(attr(qname = ":hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// saltValue
  #[sdk(attr(qname = ":saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// spinCount
  #[sdk(attr(qname = ":spinCount"))]
  pub spin_count: Option<crate::simple_type::UInt32Value>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// securityDescriptor
  #[sdk(attr(qname = ":securityDescriptor"))]
  pub security_descriptor: Option<crate::simple_type::StringValue>,
  /// Defines the ReferenceSequence Class.
  #[sdk(text_child(list, qname = "xne:sqref"))]
  pub reference_sequence: Vec<crate::simple_type::StringValue>,
}
/// Defines the CustomFilter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:customFilter")]
pub struct CustomFilter {
  /// operator
  #[sdk(attr(qname = ":operator"))]
  pub operator: Option<crate::schemas::x::FilterOperatorValues>,
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Defines the ListItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:item")]
pub struct ListItem {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the ListItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:itemLst")]
pub struct ListItems {
  /// Defines the ListItem Class.
  #[sdk(child(qname = "x14:item"))]
  pub list_item: Vec<ListItem>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Slicer Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:slicer")]
pub struct Slicer {
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// cache
  #[sdk(attr(qname = ":cache"))]
  pub cache: crate::simple_type::StringValue,
  /// caption
  #[sdk(attr(qname = ":caption"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// startItem
  #[sdk(attr(qname = ":startItem"))]
  pub start_item: Option<crate::simple_type::UInt32Value>,
  /// columnCount
  #[sdk(attr(qname = ":columnCount"))]
  pub column_count: Option<crate::simple_type::UInt32Value>,
  /// showCaption
  #[sdk(attr(qname = ":showCaption"))]
  pub show_caption: Option<crate::simple_type::BooleanValue>,
  /// level
  #[sdk(attr(qname = ":level"))]
  pub level: Option<crate::simple_type::UInt32Value>,
  /// style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// lockedPosition
  #[sdk(attr(qname = ":lockedPosition"))]
  pub locked_position: Option<crate::simple_type::BooleanValue>,
  /// rowHeight
  #[sdk(attr(qname = ":rowHeight"))]
  pub row_height: crate::simple_type::UInt32Value,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the OlapSlicerCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:olap")]
pub struct OlapSlicerCache {
  /// pivotCacheId
  #[sdk(attr(qname = ":pivotCacheId"))]
  pub pivot_cache_id: crate::simple_type::UInt32Value,
  /// Defines the OlapSlicerCacheLevelsData Class.
  #[sdk(child(qname = "x14:levels"))]
  pub olap_slicer_cache_levels_data: OlapSlicerCacheLevelsData,
  /// Defines the OlapSlicerCacheSelections Class.
  #[sdk(child(qname = "x14:selections"))]
  pub olap_slicer_cache_selections: OlapSlicerCacheSelections,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TabularSlicerCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:tabular")]
pub struct TabularSlicerCache {
  /// pivotCacheId
  #[sdk(attr(qname = ":pivotCacheId"))]
  pub pivot_cache_id: crate::simple_type::UInt32Value,
  /// sortOrder
  #[sdk(attr(qname = ":sortOrder"))]
  pub sort_order: Option<TabularSlicerCacheSortOrderValues>,
  /// customListSort
  #[sdk(attr(qname = ":customListSort"))]
  pub custom_list_sort: Option<crate::simple_type::BooleanValue>,
  /// showMissing
  #[sdk(attr(qname = ":showMissing"))]
  pub show_missing: Option<crate::simple_type::BooleanValue>,
  /// crossFilter
  #[sdk(attr(qname = ":crossFilter"))]
  pub cross_filter: Option<SlicerCacheCrossFilterValues>,
  /// Defines the TabularSlicerCacheItems Class.
  #[sdk(child(qname = "x14:items"))]
  pub tabular_slicer_cache_items: TabularSlicerCacheItems,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the SlicerCachePivotTable Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:pivotTable")]
pub struct SlicerCachePivotTable {
  /// tabId
  #[sdk(attr(qname = ":tabId"))]
  pub tab_id: crate::simple_type::UInt32Value,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// Defines the OlapSlicerCacheItemParent Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:p")]
pub struct OlapSlicerCacheItemParent {
  /// n
  #[sdk(attr(qname = ":n"))]
  pub name: crate::simple_type::StringValue,
}
/// Defines the OlapSlicerCacheItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:i")]
pub struct OlapSlicerCacheItem {
  /// n
  #[sdk(attr(qname = ":n"))]
  pub name: crate::simple_type::StringValue,
  /// c
  #[sdk(attr(qname = ":c"))]
  pub display_name: Option<crate::simple_type::StringValue>,
  /// nd
  #[sdk(attr(qname = ":nd"))]
  pub non_display: Option<crate::simple_type::BooleanValue>,
  /// Defines the OlapSlicerCacheItemParent Class.
  #[sdk(child(qname = "x14:p"))]
  pub olap_slicer_cache_item_parent: Vec<OlapSlicerCacheItemParent>,
}
/// Defines the OlapSlicerCacheRange Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:range")]
pub struct OlapSlicerCacheRange {
  /// startItem
  #[sdk(attr(qname = ":startItem"))]
  pub start_item: crate::simple_type::UInt32Value,
  /// Defines the OlapSlicerCacheItem Class.
  #[sdk(child(qname = "x14:i"))]
  pub olap_slicer_cache_item: Vec<OlapSlicerCacheItem>,
}
/// Defines the OlapSlicerCacheRanges Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:ranges")]
pub struct OlapSlicerCacheRanges {
  /// Defines the OlapSlicerCacheRange Class.
  #[sdk(child(qname = "x14:range"))]
  pub olap_slicer_cache_range: Vec<OlapSlicerCacheRange>,
}
/// Defines the OlapSlicerCacheLevelData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:level")]
pub struct OlapSlicerCacheLevelData {
  /// uniqueName
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// sourceCaption
  #[sdk(attr(qname = ":sourceCaption"))]
  pub source_caption: Option<crate::simple_type::StringValue>,
  /// count
  #[sdk(attr(qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// sortOrder
  #[sdk(attr(qname = ":sortOrder"))]
  pub sort_order: Option<OlapSlicerCacheSortOrderValues>,
  /// crossFilter
  #[sdk(attr(qname = ":crossFilter"))]
  pub cross_filter: Option<SlicerCacheCrossFilterValues>,
  /// Defines the OlapSlicerCacheRanges Class.
  #[sdk(child(qname = "x14:ranges"))]
  pub olap_slicer_cache_ranges: Option<OlapSlicerCacheRanges>,
}
/// Defines the OlapSlicerCacheLevelsData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:levels")]
pub struct OlapSlicerCacheLevelsData {
  /// count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Defines the OlapSlicerCacheLevelData Class.
  #[sdk(child(qname = "x14:level"))]
  pub olap_slicer_cache_level_data: Vec<OlapSlicerCacheLevelData>,
}
/// Defines the OlapSlicerCacheSelections Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:selections")]
pub struct OlapSlicerCacheSelections {
  /// count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Defines the OlapSlicerCacheSelection Class.
  #[sdk(child(qname = "x14:selection"))]
  pub olap_slicer_cache_selection: Vec<OlapSlicerCacheSelection>,
}
/// Defines the OlapSlicerCacheSelection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:selection")]
pub struct OlapSlicerCacheSelection {
  /// n
  #[sdk(attr(qname = ":n"))]
  pub name: crate::simple_type::StringValue,
  /// Defines the OlapSlicerCacheItemParent Class.
  #[sdk(child(qname = "x14:p"))]
  pub olap_slicer_cache_item_parent: Vec<OlapSlicerCacheItemParent>,
}
/// Defines the TabularSlicerCacheItems Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:items")]
pub struct TabularSlicerCacheItems {
  /// count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Defines the TabularSlicerCacheItem Class.
  #[sdk(child(qname = "x14:i"))]
  pub tabular_slicer_cache_item: Vec<TabularSlicerCacheItem>,
}
/// Defines the TabularSlicerCacheItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:i")]
pub struct TabularSlicerCacheItem {
  /// x
  #[sdk(attr(qname = ":x"))]
  pub atom: crate::simple_type::UInt32Value,
  /// s
  #[sdk(attr(qname = ":s"))]
  pub is_selected: Option<crate::simple_type::BooleanValue>,
  /// nd
  #[sdk(attr(qname = ":nd"))]
  pub non_display: Option<crate::simple_type::BooleanValue>,
}
/// Defines the SlicerCachePivotTables Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:pivotTables")]
pub struct SlicerCachePivotTables {
  /// Defines the SlicerCachePivotTable Class.
  #[sdk(child(qname = "x14:pivotTable"))]
  pub slicer_cache_pivot_table: Vec<SlicerCachePivotTable>,
}
/// Defines the SlicerCacheData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:data")]
pub struct SlicerCacheData {
  #[sdk(
        choice(
            child(variant = OlapSlicerCache, qname = "x14:olap"),
            child(variant = TabularSlicerCache, qname = "x14:tabular")
        )
    )]
  pub slicer_cache_data_choice: Option<SlicerCacheDataChoice>,
}
/// Defines the SlicerCacheDefinitionExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "x14:extLst")]
pub struct SlicerCacheDefinitionExtensionList {
  /// Defines the SlicerCacheDefinitionExtension Class.
  #[sdk(child(qname = "x:ext"))]
  pub slicer_cache_definition_extension: Vec<crate::schemas::x::SlicerCacheDefinitionExtension>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum PivotUserEditChoice {
  /// Defines the Formula Class.
  Formula(crate::schemas::xne::Formula),
  /// Defines the PivotEditValue Class.
  PivotEditValue(PivotEditValue),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SlicerCacheDataChoice {
  /// Defines the OlapSlicerCache Class.
  OlapSlicerCache(std::boxed::Box<OlapSlicerCache>),
  /// Defines the TabularSlicerCache Class.
  TabularSlicerCache(std::boxed::Box<TabularSlicerCache>),
}
