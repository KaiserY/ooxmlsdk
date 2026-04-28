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
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:conditionalFormattings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_ConditionalFormattings/x14:conditionalFormattings")]
pub struct ConditionalFormattings {
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
  #[sdk(child(qname = "x14:CT_ConditionalFormatting/x14:conditionalFormatting"))]
  pub x14_conditional_formatting: Vec<ConditionalFormatting>,
}
/// Defines the DataValidations Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:dataValidations.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_DataValidations/x14:dataValidations")]
pub struct DataValidations {
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
  /// disablePrompts
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :disablePrompts
  #[sdk(attr(qname = ":disablePrompts"))]
  pub disable_prompts: Option<crate::simple_type::BooleanValue>,
  /// xWindow
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :xWindow
  #[sdk(attr(qname = ":xWindow"))]
  pub x_window: Option<crate::simple_type::UInt32Value>,
  /// yWindow
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :yWindow
  #[sdk(attr(qname = ":yWindow"))]
  pub y_window: Option<crate::simple_type::UInt32Value>,
  /// count
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x14:CT_DataValidation/x14:dataValidation"))]
  pub x14_data_validation: Vec<DataValidation>,
}
/// Defines the SparklineGroups Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:sparklineGroups.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SparklineGroups/x14:sparklineGroups")]
pub struct SparklineGroups {
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
  #[sdk(child(qname = "x14:CT_SparklineGroup/x14:sparklineGroup"))]
  pub x14_sparkline_group: Vec<SparklineGroup>,
}
/// Defines the SlicerList Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:slicerList.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SlicerRefs/x14:slicerList")]
pub struct SlicerList {
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
  #[sdk(child(qname = "x14:CT_SlicerRef/x14:slicer"))]
  pub x14_slicer: Vec<SlicerRef>,
}
/// Defines the ProtectedRanges Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:protectedRanges.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_ProtectedRanges/x14:protectedRanges")]
pub struct ProtectedRanges {
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
  #[sdk(child(qname = "x14:CT_ProtectedRange/x14:protectedRange"))]
  pub x14_protected_range: Vec<ProtectedRange>,
}
/// Defines the IgnoredErrors Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:ignoredErrors.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_IgnoredErrors/x14:ignoredErrors")]
pub struct IgnoredErrors {
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
  #[sdk(child(qname = "x14:CT_IgnoredError/x14:ignoredError"))]
  pub x14_ignored_error: Vec<IgnoredError>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x14:extLst"))]
  pub x14_ext_lst: Option<ExtensionList>,
}
/// Defines the DefinedNames Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:definedNames.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_DefinedNames/x14:definedNames")]
pub struct DefinedNames {
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
  #[sdk(child(qname = "x14:CT_DefinedName/x14:definedName"))]
  pub x14_defined_name: Vec<DefinedName>,
}
/// Defines the PivotCaches Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:pivotCaches.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotCaches/x14:pivotCaches")]
pub struct PivotCaches {
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
  #[sdk(child(qname = "x:CT_PivotCache/x:pivotCache"))]
  pub x_pivot_cache:
    Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCache>,
}
/// Defines the SlicerCaches Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:slicerCaches.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SlicerCaches/x14:slicerCaches")]
pub struct SlicerCaches {
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
  #[sdk(child(qname = "x14:CT_SlicerCache/x14:slicerCache"))]
  pub x14_slicer_cache: Vec<SlicerCache>,
}
/// Defines the WorkbookProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:workbookPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_WorkbookPr/x14:workbookPr")]
pub struct WorkbookProperties {
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
  /// defaultImageDpi
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :defaultImageDpi
  #[sdk(attr(qname = ":defaultImageDpi"))]
  pub default_image_dpi: Option<crate::simple_type::UInt32Value>,
  /// discardImageEditData
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :discardImageEditData
  #[sdk(attr(qname = ":discardImageEditData"))]
  pub discard_image_edit_data: Option<crate::simple_type::BooleanValue>,
  /// accuracyVersion
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :accuracyVersion
  #[sdk(attr(qname = ":accuracyVersion"))]
  pub accuracy_version: Option<crate::simple_type::UInt32Value>,
}
/// Defines the CalculatedMember Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:calculatedMember.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_CalculatedMember/x14:calculatedMember")]
pub struct CalculatedMember {
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
  /// displayFolder
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :displayFolder
  #[sdk(attr(qname = ":displayFolder"))]
  pub display_folder: Option<crate::simple_type::StringValue>,
  /// flattenHierarchies
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :flattenHierarchies
  #[sdk(attr(qname = ":flattenHierarchies"))]
  pub flatten_hierarchies: Option<crate::simple_type::BooleanValue>,
  /// dynamicSet
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :dynamicSet
  #[sdk(attr(qname = ":dynamicSet"))]
  pub dynamic_set: Option<crate::simple_type::BooleanValue>,
  /// hierarchizeDistinct
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :hierarchizeDistinct
  #[sdk(attr(qname = ":hierarchizeDistinct"))]
  pub hierarchize_distinct: Option<crate::simple_type::BooleanValue>,
  /// mdxLong
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :mdxLong
  #[sdk(attr(qname = ":mdxLong"))]
  pub mdx_long: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x14:CT_TupleSet/x14:tupleSet"))]
  pub tuple_set: Option<std::boxed::Box<TupleSet>>,
}
/// Defines the CacheHierarchy Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:cacheHierarchy.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_CacheHierarchy/x14:cacheHierarchy")]
pub struct CacheHierarchy {
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
  /// flattenHierarchies
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :flattenHierarchies
  #[sdk(attr(qname = ":flattenHierarchies"))]
  pub flatten_hierarchies: Option<crate::simple_type::BooleanValue>,
  /// measuresSet
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :measuresSet
  #[sdk(attr(qname = ":measuresSet"))]
  pub measures_set: Option<crate::simple_type::BooleanValue>,
  /// hierarchizeDistinct
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :hierarchizeDistinct
  #[sdk(attr(qname = ":hierarchizeDistinct"))]
  pub hierarchize_distinct: Option<crate::simple_type::BooleanValue>,
  /// ignore
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :ignore
  #[sdk(attr(qname = ":ignore"))]
  pub ignore: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x14:CT_SetLevels/x14:setLevels"))]
  pub set_levels: Option<SetLevels>,
}
/// Defines the DataField Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:dataField.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_DataField/x14:dataField")]
pub struct DataField {
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
  /// pivotShowAs
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :pivotShowAs
  #[sdk(attr(qname = ":pivotShowAs"))]
  pub pivot_show_as: Option<PivotShowAsValues>,
  /// sourceField
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :sourceField
  #[sdk(attr(qname = ":sourceField"))]
  pub source_field: Option<crate::simple_type::UInt32Value>,
  /// uniqueName
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :uniqueName
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: Option<crate::simple_type::StringValue>,
}
/// Defines the PivotField Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:pivotField.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_PivotField/x14:pivotField")]
pub struct PivotField {
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
  /// fillDownLabels
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :fillDownLabels
  #[sdk(attr(qname = ":fillDownLabels"))]
  pub fill_down_labels: Option<crate::simple_type::BooleanValue>,
  /// ignore
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :ignore
  #[sdk(attr(qname = ":ignore"))]
  pub ignore: Option<crate::simple_type::BooleanValue>,
}
/// Defines the PivotTableDefinition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:pivotTableDefinition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_PivotTableDefinition/x14:pivotTableDefinition")]
pub struct PivotTableDefinition {
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
  /// fillDownLabelsDefault
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :fillDownLabelsDefault
  #[sdk(attr(qname = ":fillDownLabelsDefault"))]
  pub fill_down_labels_default: Option<crate::simple_type::BooleanValue>,
  /// visualTotalsForSets
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :visualTotalsForSets
  #[sdk(attr(qname = ":visualTotalsForSets"))]
  pub visual_totals_for_sets: Option<crate::simple_type::BooleanValue>,
  /// calculatedMembersInFilters
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :calculatedMembersInFilters
  #[sdk(attr(qname = ":calculatedMembersInFilters"))]
  pub calculated_members_in_filters: Option<crate::simple_type::BooleanValue>,
  /// altText
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :altText
  #[sdk(attr(qname = ":altText"))]
  pub alt_text: Option<crate::simple_type::StringValue>,
  /// altTextSummary
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :altTextSummary
  #[sdk(attr(qname = ":altTextSummary"))]
  pub alt_text_summary: Option<crate::simple_type::StringValue>,
  /// enableEdit
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :enableEdit
  #[sdk(attr(qname = ":enableEdit"))]
  pub enable_edit: Option<crate::simple_type::BooleanValue>,
  /// autoApply
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :autoApply
  #[sdk(attr(qname = ":autoApply"))]
  pub auto_apply: Option<crate::simple_type::BooleanValue>,
  /// allocationMethod
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :allocationMethod
  #[sdk(attr(qname = ":allocationMethod"))]
  pub allocation_method: Option<AllocationMethodValues>,
  /// weightExpression
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :weightExpression
  #[sdk(attr(qname = ":weightExpression"))]
  pub weight_expression: Option<crate::simple_type::StringValue>,
  /// hideValuesRow
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :hideValuesRow
  #[sdk(attr(qname = ":hideValuesRow"))]
  pub hide_values_row: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x14:CT_PivotEdits/x14:pivotEdits"))]
  pub pivot_edits: Option<PivotEdits>,
  /// _
  #[sdk(child(qname = "x14:CT_PivotChanges/x14:pivotChanges"))]
  pub pivot_changes: Option<PivotChanges>,
  /// _
  #[sdk(child(qname = "x14:CT_ConditionalFormats/x14:conditionalFormats"))]
  pub conditional_formats: Option<ConditionalFormats>,
}
/// Defines the PivotCacheDefinition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:pivotCacheDefinition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_PivotCacheDefinition/x14:pivotCacheDefinition")]
pub struct PivotCacheDefinition {
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
  /// slicerData
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :slicerData
  #[sdk(attr(qname = ":slicerData"))]
  pub slicer_data: Option<crate::simple_type::BooleanValue>,
  /// pivotCacheId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :pivotCacheId
  #[sdk(attr(qname = ":pivotCacheId"))]
  pub pivot_cache_id: Option<crate::simple_type::UInt32Value>,
  /// supportSubqueryNonVisual
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supportSubqueryNonVisual
  #[sdk(attr(qname = ":supportSubqueryNonVisual"))]
  pub support_subquery_non_visual: Option<crate::simple_type::BooleanValue>,
  /// supportSubqueryCalcMem
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supportSubqueryCalcMem
  #[sdk(attr(qname = ":supportSubqueryCalcMem"))]
  pub support_subquery_calc_mem: Option<crate::simple_type::BooleanValue>,
  /// supportAddCalcMems
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :supportAddCalcMems
  #[sdk(attr(qname = ":supportAddCalcMems"))]
  pub support_add_calc_mems: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Connection Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:connection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_Connection/x14:connection")]
pub struct Connection {
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
  /// culture
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :culture
  #[sdk(attr(qname = ":culture"))]
  pub culture: Option<crate::simple_type::StringValue>,
  /// embeddedDataId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :embeddedDataId
  #[sdk(attr(qname = ":embeddedDataId"))]
  pub embedded_data_id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_CalculatedMembers/x14:calculatedMembers"))]
  pub calculated_members: Option<CalculatedMembers>,
}
/// Defines the Table Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:table.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_Table/x14:table")]
pub struct Table {
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
  /// altText
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :altText
  #[sdk(attr(qname = ":altText"))]
  pub alt_text: Option<crate::simple_type::StringValue>,
  /// altTextSummary
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :altTextSummary
  #[sdk(attr(qname = ":altTextSummary"))]
  pub alt_text_summary: Option<crate::simple_type::StringValue>,
}
/// Defines the SlicerStyles Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:slicerStyles.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SlicerStyles/x14:slicerStyles")]
pub struct SlicerStyles {
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
  /// defaultSlicerStyle
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :defaultSlicerStyle
  #[sdk(attr(qname = ":defaultSlicerStyle"))]
  pub default_slicer_style: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x14:CT_SlicerStyle/x14:slicerStyle"))]
  pub x14_slicer_style: Vec<SlicerStyle>,
}
/// Defines the DifferentialFormats Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:dxfs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dxfs/x14:dxfs")]
pub struct DifferentialFormats {
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
  /// Format Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Dxf/x:dxf"))]
  pub x_dxf:
    Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DifferentialFormat>,
}
/// Defines the OleItem Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:oleItem.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_OleItem/x14:oleItem")]
pub struct OleItem {
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
  /// name
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// icon
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :icon
  #[sdk(attr(qname = ":icon"))]
  pub icon: Option<crate::simple_type::BooleanValue>,
  /// advise
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :advise
  #[sdk(attr(qname = ":advise"))]
  pub advise: Option<crate::simple_type::BooleanValue>,
  /// preferPic
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :preferPic
  #[sdk(attr(qname = ":preferPic"))]
  pub prefer_picture: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_DdeValues/x14:values"))]
  pub dde_values: Option<DdeValues>,
}
/// Defines the PivotHierarchy Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:pivotHierarchy.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_PivotHierarchy/x14:pivotHierarchy")]
pub struct PivotHierarchy {
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
  /// ignore
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :ignore
  #[sdk(attr(qname = ":ignore"))]
  pub ignore: Option<crate::simple_type::BooleanValue>,
}
/// Defines the CacheField Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:cacheField.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_CacheField/x14:cacheField")]
pub struct CacheField {
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
  /// ignore
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :ignore
  #[sdk(attr(qname = ":ignore"))]
  pub ignore: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Id Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:id.
pub type Id = crate::simple_type::StringValue;
/// Defines the IconFilter Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:iconFilter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_IconFilter/x14:iconFilter")]
pub struct IconFilter {
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
  /// iconSet
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :iconSet
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set: IconSetTypeValues,
  /// iconId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :iconId
  #[sdk(attr(qname = ":iconId"))]
  pub icon_id: crate::simple_type::UInt32Value,
}
/// Defines the Filter Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:filter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_Filter/x14:filter")]
pub struct Filter {
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
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Defines the CustomFilters Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:customFilters.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_CustomFilters/x14:customFilters")]
pub struct CustomFilters {
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
  /// and
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :and
  #[sdk(attr(qname = ":and"))]
  pub and: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x14:CT_CustomFilter/x14:customFilter"))]
  pub x14_custom_filter: Vec<CustomFilter>,
}
/// Defines the SortCondition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:sortCondition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SortCondition/x14:sortCondition")]
pub struct SortCondition {
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
  /// descending
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :descending
  #[sdk(attr(qname = ":descending"))]
  pub descending: Option<crate::simple_type::BooleanValue>,
  /// sortBy
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :sortBy
  #[sdk(attr(qname = ":sortBy"))]
  pub sort_by:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SortByValues>,
  /// ref
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// customList
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :customList
  #[sdk(attr(qname = ":customList"))]
  pub custom_list: Option<crate::simple_type::StringValue>,
  /// dxfId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :dxfId
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
  /// iconSet
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :iconSet
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set: Option<IconSetTypeValues>,
  /// iconId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :iconId
  #[sdk(attr(qname = ":iconId"))]
  pub icon_id: Option<crate::simple_type::UInt32Value>,
}
/// Defines the SourceConnection Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:sourceConnection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SourceConnection/x14:sourceConnection")]
pub struct SourceConnection {
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
  /// name
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// Defines the DatastoreItem Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:datastoreItem.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_DatastoreItem/x14:datastoreItem")]
pub struct DatastoreItem {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
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
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the FormControlProperties Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:formControlPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_FormControlPr/x14:formControlPr")]
pub struct FormControlProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
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
  /// objectType
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :objectType
  #[sdk(attr(qname = ":objectType"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub object_type: Option<ObjectTypeValues>,
  /// checked
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :checked
  #[sdk(attr(qname = ":checked"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub checked: Option<CheckedValues>,
  /// colored
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :colored
  #[sdk(attr(qname = ":colored"))]
  pub colored: Option<crate::simple_type::BooleanValue>,
  /// dropLines
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :dropLines
  #[sdk(attr(qname = ":dropLines"))]
  pub drop_lines: Option<crate::simple_type::UInt32Value>,
  /// dropStyle
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :dropStyle
  #[sdk(attr(qname = ":dropStyle"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub drop_style: Option<DropStyleValues>,
  /// dx
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :dx
  #[sdk(attr(qname = ":dx"))]
  pub scroll_bar_width: Option<crate::simple_type::UInt32Value>,
  /// firstButton
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :firstButton
  #[sdk(attr(qname = ":firstButton"))]
  pub first_button: Option<crate::simple_type::BooleanValue>,
  /// fmlaGroup
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :fmlaGroup
  #[sdk(attr(qname = ":fmlaGroup"))]
  pub fmla_group: Option<crate::simple_type::StringValue>,
  /// fmlaLink
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :fmlaLink
  #[sdk(attr(qname = ":fmlaLink"))]
  pub fmla_link: Option<crate::simple_type::StringValue>,
  /// fmlaRange
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :fmlaRange
  #[sdk(attr(qname = ":fmlaRange"))]
  pub fmla_range: Option<crate::simple_type::StringValue>,
  /// fmlaTxbx
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :fmlaTxbx
  #[sdk(attr(qname = ":fmlaTxbx"))]
  pub fmla_textbox: Option<crate::simple_type::StringValue>,
  /// horiz
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :horiz
  #[sdk(attr(qname = ":horiz"))]
  pub horizontal: Option<crate::simple_type::BooleanValue>,
  /// inc
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :inc
  #[sdk(attr(qname = ":inc"))]
  pub incremental: Option<crate::simple_type::UInt32Value>,
  /// justLastX
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :justLastX
  #[sdk(attr(qname = ":justLastX"))]
  pub just_last_x: Option<crate::simple_type::BooleanValue>,
  /// lockText
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :lockText
  #[sdk(attr(qname = ":lockText"))]
  pub lock_text: Option<crate::simple_type::BooleanValue>,
  /// max
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :max
  #[sdk(attr(qname = ":max"))]
  pub max: Option<crate::simple_type::UInt32Value>,
  /// min
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :min
  #[sdk(attr(qname = ":min"))]
  pub min: Option<crate::simple_type::UInt32Value>,
  /// multiSel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :multiSel
  #[sdk(attr(qname = ":multiSel"))]
  pub multiple_selection: Option<crate::simple_type::StringValue>,
  /// noThreeD
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :noThreeD
  #[sdk(attr(qname = ":noThreeD"))]
  pub no_three_d: Option<crate::simple_type::BooleanValue>,
  /// noThreeD2
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :noThreeD2
  #[sdk(attr(qname = ":noThreeD2"))]
  pub no_three_d2: Option<crate::simple_type::BooleanValue>,
  /// page
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :page
  #[sdk(attr(qname = ":page"))]
  pub page: Option<crate::simple_type::UInt32Value>,
  /// sel
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :sel
  #[sdk(attr(qname = ":sel"))]
  pub selected: Option<crate::simple_type::UInt32Value>,
  /// seltype
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :seltype
  #[sdk(attr(qname = ":seltype"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub selection_type: Option<SelectionTypeValues>,
  /// textHAlign
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :textHAlign
  #[sdk(attr(qname = ":textHAlign"))]
  pub text_horizontal_align: Option<TextHorizontalAlignmentValues>,
  /// textVAlign
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :textVAlign
  #[sdk(attr(qname = ":textVAlign"))]
  pub text_vertical_align: Option<TextVerticalAlignmentValues>,
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::UInt32Value>,
  /// widthMin
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :widthMin
  #[sdk(attr(qname = ":widthMin"))]
  pub minimum_width: Option<crate::simple_type::UInt32Value>,
  /// editVal
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :editVal
  #[sdk(attr(qname = ":editVal"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub edit_val: Option<EditValidationValues>,
  /// multiLine
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :multiLine
  #[sdk(attr(qname = ":multiLine"))]
  pub multiple_lines: Option<crate::simple_type::BooleanValue>,
  /// verticalBar
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :verticalBar
  #[sdk(attr(qname = ":verticalBar"))]
  pub vertical_bar: Option<crate::simple_type::BooleanValue>,
  /// passwordEdit
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :passwordEdit
  #[sdk(attr(qname = ":passwordEdit"))]
  pub password_edit: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x14:CT_ListItems/x14:itemLst"))]
  pub list_items: Option<std::boxed::Box<ListItems>>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Slicers Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:slicers.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_Slicers/x14:slicers")]
pub struct Slicers {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
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
  #[sdk(child(qname = "x14:CT_Slicer/x14:slicer"))]
  pub x14_slicer: Vec<Slicer>,
}
/// Defines the SlicerCacheDefinition Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:slicerCacheDefinition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SlicerCacheDefinition/x14:slicerCacheDefinition")]
pub struct SlicerCacheDefinition {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
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
  /// name
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// sourceName
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :sourceName
  #[sdk(attr(qname = ":sourceName"))]
  pub source_name: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x14:CT_SlicerCachePivotTables/x14:pivotTables"))]
  pub slicer_cache_pivot_tables: Option<SlicerCachePivotTables>,
  /// _
  #[sdk(child(qname = "x14:CT_SlicerCacheData/x14:data"))]
  pub slicer_cache_data: Option<std::boxed::Box<SlicerCacheData>>,
  /// _
  #[sdk(child(qname = "x:CT_SlicerCacheDefinitionExtensionList/x14:extLst"))]
  pub slicer_cache_definition_extension_list: Option<SlicerCacheDefinitionExtensionList>,
}
/// Defines the ConditionalFormatting Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:conditionalFormatting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_ConditionalFormatting/x14:conditionalFormatting")]
pub struct ConditionalFormatting {
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
  /// pivot
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :pivot
  #[sdk(attr(qname = ":pivot"))]
  pub pivot: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x14:CT_CfRule/x14:cfRule"))]
  pub x14_cf_rule: Vec<ConditionalFormattingRule>,
  /// _
  #[sdk(text_child(qname = "xne:ST_Sqref/xne:sqref"))]
  pub xne_sqref: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x14:extLst"))]
  pub x14_ext_lst: Option<ExtensionList>,
}
/// Defines the ConditionalFormattingRule Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:cfRule.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_CfRule/x14:cfRule")]
pub struct ConditionalFormattingRule {
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
    /// type
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :type
    #[sdk(attr(qname = ":type"))]
    pub r#type: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ConditionalFormatValues,
    >,
    /// priority
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :priority
    #[sdk(attr(qname = ":priority"))]
    pub priority: Option<crate::simple_type::Int32Value>,
    /// stopIfTrue
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :stopIfTrue
    #[sdk(attr(qname = ":stopIfTrue"))]
    pub stop_if_true: Option<crate::simple_type::BooleanValue>,
    /// aboveAverage
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :aboveAverage
    #[sdk(attr(qname = ":aboveAverage"))]
    pub above_average: Option<crate::simple_type::BooleanValue>,
    /// percent
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :percent
    #[sdk(attr(qname = ":percent"))]
    pub percent: Option<crate::simple_type::BooleanValue>,
    /// bottom
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :bottom
    #[sdk(attr(qname = ":bottom"))]
    pub bottom: Option<crate::simple_type::BooleanValue>,
    /// operator
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :operator
    #[sdk(attr(qname = ":operator"))]
    pub operator: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ConditionalFormattingOperatorValues,
    >,
    /// text
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :text
    #[sdk(attr(qname = ":text"))]
    pub text: Option<crate::simple_type::StringValue>,
    /// timePeriod
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :timePeriod
    #[sdk(attr(qname = ":timePeriod"))]
    pub time_period: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::TimePeriodValues,
    >,
    /// rank
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :rank
    #[sdk(attr(qname = ":rank"))]
    pub rank: Option<crate::simple_type::UInt32Value>,
    /// stdDev
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :stdDev
    #[sdk(attr(qname = ":stdDev"))]
    pub standard_deviation: Option<crate::simple_type::Int32Value>,
    /// equalAverage
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :equalAverage
    #[sdk(attr(qname = ":equalAverage"))]
    pub equal_average: Option<crate::simple_type::BooleanValue>,
    /// activePresent
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :activePresent
    #[sdk(attr(qname = ":activePresent"))]
    pub active_present: Option<crate::simple_type::BooleanValue>,
    /// id
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :id
    #[sdk(attr(qname = ":id"))]
    #[sdk(
        pattern(
            source = 0u32,
            regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
        )
    )]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub id: Option<crate::simple_type::StringValue>,
    /// _
    #[sdk(text_child(qname = "x:ST_Formula/xne:f"))]
    pub xne_f: Vec<crate::simple_type::StringValue>,
    /// _
    #[sdk(child(qname = "x14:CT_ColorScale/x14:colorScale"))]
    pub x14_color_scale: Option<ColorScale>,
    /// _
    #[sdk(child(qname = "x14:CT_DataBar/x14:dataBar"))]
    pub x14_data_bar: Option<std::boxed::Box<DataBar>>,
    /// _
    #[sdk(child(qname = "x14:CT_IconSet/x14:iconSet"))]
    pub x14_icon_set: Option<IconSet>,
    /// _
    #[sdk(child(qname = "x:CT_Dxf/x14:dxf"))]
    pub x14_dxf: Option<std::boxed::Box<DifferentialType>>,
    /// _
    #[sdk(child(qname = "x:CT_ExtensionList/x14:extLst"))]
    pub x14_ext_lst: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExtensionList/x14:extLst")]
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
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
/// Defines the DataValidation Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:dataValidation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_DataValidation/x14:dataValidation")]
pub struct DataValidation {
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
    /// type
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :type
    #[sdk(attr(qname = ":type"))]
    pub r#type: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationValues,
    >,
    /// errorStyle
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :errorStyle
    #[sdk(attr(qname = ":errorStyle"))]
    pub error_style: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationErrorStyleValues,
    >,
    /// imeMode
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :imeMode
    #[sdk(attr(qname = ":imeMode"))]
    pub ime_mode: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationImeModeValues,
    >,
    /// operator
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :operator
    #[sdk(attr(qname = ":operator"))]
    pub operator: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationOperatorValues,
    >,
    /// allowBlank
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :allowBlank
    #[sdk(attr(qname = ":allowBlank"))]
    pub allow_blank: Option<crate::simple_type::BooleanValue>,
    /// showDropDown
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :showDropDown
    #[sdk(attr(qname = ":showDropDown"))]
    pub show_drop_down: Option<crate::simple_type::BooleanValue>,
    /// showInputMessage
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :showInputMessage
    #[sdk(attr(qname = ":showInputMessage"))]
    pub show_input_message: Option<crate::simple_type::BooleanValue>,
    /// showErrorMessage
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :showErrorMessage
    #[sdk(attr(qname = ":showErrorMessage"))]
    pub show_error_message: Option<crate::simple_type::BooleanValue>,
    /// errorTitle
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :errorTitle
    #[sdk(attr(qname = ":errorTitle"))]
    pub error_title: Option<crate::simple_type::StringValue>,
    /// error
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :error
    #[sdk(attr(qname = ":error"))]
    pub error: Option<crate::simple_type::StringValue>,
    /// promptTitle
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :promptTitle
    #[sdk(attr(qname = ":promptTitle"))]
    pub prompt_title: Option<crate::simple_type::StringValue>,
    /// prompt
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :prompt
    #[sdk(attr(qname = ":prompt"))]
    pub prompt: Option<crate::simple_type::StringValue>,
    /// _
    #[sdk(child(qname = "x14:CT_DataValidationFormula/x14:formula1"))]
    pub data_validation_forumla1: Option<DataValidationForumla1>,
    /// _
    #[sdk(child(qname = "x14:CT_DataValidationFormula/x14:formula2"))]
    pub data_validation_forumla2: Option<DataValidationForumla2>,
    /// _
    #[sdk(text_child(qname = "xne:ST_Sqref/xne:sqref"))]
    pub reference_sequence: crate::simple_type::StringValue,
}
/// Defines the DataValidationForumla1 Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:formula1.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_DataValidationFormula/x14:formula1")]
pub struct DataValidationForumla1 {
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
  #[sdk(text_child(qname = "x:ST_Formula/xne:f"))]
  pub formula: crate::simple_type::StringValue,
}
/// Defines the DataValidationForumla2 Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:formula2.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_DataValidationFormula/x14:formula2")]
pub struct DataValidationForumla2 {
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
  #[sdk(text_child(qname = "x:ST_Formula/xne:f"))]
  pub formula: crate::simple_type::StringValue,
}
/// Defines the DataValidationFormulaType Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_DataValidationFormula/")]
pub struct DataValidationFormulaType {
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
  #[sdk(text_child(qname = "x:ST_Formula/xne:f"))]
  pub formula: Vec<crate::simple_type::StringValue>,
}
/// Defines the SparklineGroup Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:sparklineGroup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SparklineGroup/x14:sparklineGroup")]
pub struct SparklineGroup {
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
  pub r#type: Option<SparklineTypeValues>,
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
  pub display_empty_cells_as: Option<DisplayBlanksAsValues>,
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
  pub min_axis_type: Option<SparklineAxisMinMaxValues>,
  /// maxAxisType
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :maxAxisType
  #[sdk(attr(qname = ":maxAxisType"))]
  pub max_axis_type: Option<SparklineAxisMinMaxValues>,
  /// rightToLeft
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :rightToLeft
  #[sdk(attr(qname = ":rightToLeft"))]
  pub right_to_left: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x14:colorSeries"))]
  pub series_color: Option<SeriesColor>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x14:colorNegative"))]
  pub negative_color: Option<NegativeColor>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x14:colorAxis"))]
  pub axis_color: Option<AxisColor>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x14:colorMarkers"))]
  pub markers_color: Option<MarkersColor>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x14:colorFirst"))]
  pub first_marker_color: Option<FirstMarkerColor>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x14:colorLast"))]
  pub last_marker_color: Option<LastMarkerColor>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x14:colorHigh"))]
  pub high_marker_color: Option<HighMarkerColor>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x14:colorLow"))]
  pub low_marker_color: Option<LowMarkerColor>,
  /// _
  #[sdk(text_child(qname = "x:ST_Formula/xne:f"))]
  pub formula: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x14:CT_Sparklines/x14:sparklines"))]
  pub sparklines: std::boxed::Box<Sparklines>,
}
/// Defines the SeriesColor Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:colorSeries.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x14:colorSeries")]
pub struct SeriesColor {
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
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the NegativeColor Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:colorNegative.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x14:colorNegative")]
pub struct NegativeColor {
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
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the AxisColor Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:colorAxis.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x14:colorAxis")]
pub struct AxisColor {
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
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the MarkersColor Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:colorMarkers.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x14:colorMarkers")]
pub struct MarkersColor {
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
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the FirstMarkerColor Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:colorFirst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x14:colorFirst")]
pub struct FirstMarkerColor {
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
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the LastMarkerColor Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:colorLast.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x14:colorLast")]
pub struct LastMarkerColor {
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
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the HighMarkerColor Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:colorHigh.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x14:colorHigh")]
pub struct HighMarkerColor {
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
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the LowMarkerColor Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:colorLow.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x14:colorLow")]
pub struct LowMarkerColor {
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
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the Color Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:color.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x14:color")]
pub struct Color {
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
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the FillColor Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:fillColor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x14:fillColor")]
pub struct FillColor {
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
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the BorderColor Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:borderColor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x14:borderColor")]
pub struct BorderColor {
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
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the NegativeFillColor Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:negativeFillColor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x14:negativeFillColor")]
pub struct NegativeFillColor {
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
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the NegativeBorderColor Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:negativeBorderColor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x14:negativeBorderColor")]
pub struct NegativeBorderColor {
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
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the BarAxisColor Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:axisColor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/x14:axisColor")]
pub struct BarAxisColor {
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
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the ColorType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Color/")]
pub struct ColorType {
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
  /// Automatic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :auto
  #[sdk(attr(qname = ":auto"))]
  pub auto: Option<crate::simple_type::BooleanValue>,
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :indexed
  #[sdk(attr(qname = ":indexed"))]
  pub indexed: Option<crate::simple_type::UInt32Value>,
  /// Alpha Red Green Blue Color Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rgb
  #[sdk(attr(qname = ":rgb"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub rgb: Option<crate::simple_type::HexBinaryValue>,
  /// Theme Color
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :theme
  #[sdk(attr(qname = ":theme"))]
  pub theme: Option<crate::simple_type::UInt32Value>,
  /// Tint
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tint
  #[sdk(attr(qname = ":tint"))]
  pub tint: Option<crate::simple_type::DoubleValue>,
}
/// Defines the Sparklines Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:sparklines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_Sparklines/x14:sparklines")]
pub struct Sparklines {
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
  #[sdk(child(qname = "x14:CT_Sparkline/x14:sparkline"))]
  pub x14_sparkline: Vec<Sparkline>,
}
/// Defines the Sparkline Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:sparkline.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_Sparkline/x14:sparkline")]
pub struct Sparkline {
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
  #[sdk(text_child(qname = "x:ST_Formula/xne:f"))]
  pub formula: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xne:ST_Sqref/xne:sqref"))]
  pub reference_sequence: crate::simple_type::StringValue,
}
/// Defines the SlicerRef Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:slicer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SlicerRef/x14:slicer")]
pub struct SlicerRef {
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
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the SlicerCache Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:slicerCache.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SlicerCache/x14:slicerCache")]
pub struct SlicerCache {
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
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the DefinedName Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:definedName.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_DefinedName/x14:definedName")]
pub struct DefinedName {
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
  /// name
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x14:CT_DefinedNameArgumentDescriptions/x14:argumentDescriptions"))]
  pub argument_descriptions: Option<ArgumentDescriptions>,
}
/// Defines the ArgumentDescriptions Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:argumentDescriptions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_DefinedNameArgumentDescriptions/x14:argumentDescriptions")]
pub struct ArgumentDescriptions {
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
  /// count
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x14:CT_DefinedNameArgumentDescription/x14:argumentDescription"))]
  pub x14_argument_description: Vec<ArgumentDescription>,
}
/// Defines the ArgumentDescription Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:argumentDescription.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_DefinedNameArgumentDescription/x14:argumentDescription")]
pub struct ArgumentDescription {
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
  /// index
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :index
  #[sdk(attr(qname = ":index"))]
  pub index: crate::simple_type::UInt32Value,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the TupleSet Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:tupleSet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_TupleSet/x14:tupleSet")]
pub struct TupleSet {
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
  /// rowCount
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :rowCount
  #[sdk(attr(qname = ":rowCount"))]
  pub row_count: Option<crate::simple_type::UInt32Value>,
  /// columnCount
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :columnCount
  #[sdk(attr(qname = ":columnCount"))]
  pub column_count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x14:CT_TupleSetHeaders/x14:headers"))]
  pub tuple_set_headers: std::boxed::Box<TupleSetHeaders>,
  /// _
  #[sdk(child(qname = "x14:CT_TupleSetRows/x14:rows"))]
  pub tuple_set_rows: std::boxed::Box<TupleSetRows>,
}
/// Defines the TupleSetHeaders Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:headers.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_TupleSetHeaders/x14:headers")]
pub struct TupleSetHeaders {
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
  #[sdk(child(qname = "x14:CT_TupleSetHeader/x14:header"))]
  pub x14_header: Vec<TupleSetHeader>,
}
/// Defines the TupleSetRows Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:rows.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_TupleSetRows/x14:rows")]
pub struct TupleSetRows {
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
  #[sdk(child(qname = "x14:CT_TupleSetRow/x14:row"))]
  pub x14_row: Vec<TupleSetRow>,
}
/// Defines the TupleSetHeader Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:header.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_TupleSetHeader/x14:header")]
pub struct TupleSetHeader {
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
  /// uniqueName
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :uniqueName
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: Option<crate::simple_type::StringValue>,
  /// hierarchyName
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :hierarchyName
  #[sdk(attr(qname = ":hierarchyName"))]
  pub hierarchy_name: Option<crate::simple_type::StringValue>,
}
/// Defines the TupleSetRow Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:row.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_TupleSetRow/x14:row")]
pub struct TupleSetRow {
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
  #[sdk(child(qname = "x14:CT_TupleSetRowItem/x14:rowItem"))]
  pub x14_row_item: Vec<TupleSetRowItem>,
}
/// Defines the TupleSetRowItem Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:rowItem.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_TupleSetRowItem/x14:rowItem")]
pub struct TupleSetRowItem {
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
  /// u
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :u
  #[sdk(attr(qname = ":u"))]
  pub unique_name: Option<crate::simple_type::StringValue>,
  /// d
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :d
  #[sdk(attr(qname = ":d"))]
  pub display_name: Option<crate::simple_type::StringValue>,
}
/// Defines the SetLevel Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:setLevel.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SetLevel/x14:setLevel")]
pub struct SetLevel {
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
  /// hierarchy
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :hierarchy
  #[sdk(attr(qname = ":hierarchy"))]
  pub hierarchy: crate::simple_type::Int32Value,
}
/// Defines the SetLevels Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:setLevels.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SetLevels/x14:setLevels")]
pub struct SetLevels {
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
  /// count
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x14:CT_SetLevel/x14:setLevel"))]
  pub x14_set_level: Vec<SetLevel>,
}
/// Defines the ColorScale Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:colorScale.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_ColorScale/x14:colorScale")]
pub struct ColorScale {
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
  #[sdk(child(qname = "x14:CT_Cfvo/x14:cfvo"))]
  pub x14_cfvo: Vec<ConditionalFormattingValueObject>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x14:color"))]
  pub x14_color: Vec<Color>,
}
/// Defines the DataBar Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:dataBar.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_DataBar/x14:dataBar")]
pub struct DataBar {
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
  /// minLength
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :minLength
  #[sdk(attr(qname = ":minLength"))]
  pub min_length: Option<crate::simple_type::UInt32Value>,
  /// maxLength
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :maxLength
  #[sdk(attr(qname = ":maxLength"))]
  pub max_length: Option<crate::simple_type::UInt32Value>,
  /// showValue
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showValue
  #[sdk(attr(qname = ":showValue"))]
  pub show_value: Option<crate::simple_type::BooleanValue>,
  /// border
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :border
  #[sdk(attr(qname = ":border"))]
  pub border: Option<crate::simple_type::BooleanValue>,
  /// gradient
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :gradient
  #[sdk(attr(qname = ":gradient"))]
  pub gradient: Option<crate::simple_type::BooleanValue>,
  /// direction
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :direction
  #[sdk(attr(qname = ":direction"))]
  pub direction: Option<DataBarDirectionValues>,
  /// negativeBarColorSameAsPositive
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :negativeBarColorSameAsPositive
  #[sdk(attr(qname = ":negativeBarColorSameAsPositive"))]
  pub negative_bar_color_same_as_positive: Option<crate::simple_type::BooleanValue>,
  /// negativeBarBorderColorSameAsPositive
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :negativeBarBorderColorSameAsPositive
  #[sdk(attr(qname = ":negativeBarBorderColorSameAsPositive"))]
  pub negative_bar_border_color_same_as_positive: Option<crate::simple_type::BooleanValue>,
  /// axisPosition
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :axisPosition
  #[sdk(attr(qname = ":axisPosition"))]
  pub axis_position: Option<DataBarAxisPositionValues>,
  /// _
  #[sdk(child(qname = "x14:CT_Cfvo/x14:cfvo"))]
  pub x14_cfvo: Vec<ConditionalFormattingValueObject>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x14:fillColor"))]
  pub x14_fill_color: Option<FillColor>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x14:borderColor"))]
  pub x14_border_color: Option<BorderColor>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x14:negativeFillColor"))]
  pub x14_negative_fill_color: Option<NegativeFillColor>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x14:negativeBorderColor"))]
  pub x14_negative_border_color: Option<NegativeBorderColor>,
  /// _
  #[sdk(child(qname = "x:CT_Color/x14:axisColor"))]
  pub x14_axis_color: Option<BarAxisColor>,
}
/// Defines the IconSet Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:iconSet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_IconSet/x14:iconSet")]
pub struct IconSet {
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
  /// iconSet
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :iconSet
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set_types: Option<IconSetTypeValues>,
  /// showValue
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showValue
  #[sdk(attr(qname = ":showValue"))]
  pub show_value: Option<crate::simple_type::BooleanValue>,
  /// percent
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :percent
  #[sdk(attr(qname = ":percent"))]
  pub percent: Option<crate::simple_type::BooleanValue>,
  /// reverse
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :reverse
  #[sdk(attr(qname = ":reverse"))]
  pub reverse: Option<crate::simple_type::BooleanValue>,
  /// custom
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :custom
  #[sdk(attr(qname = ":custom"))]
  pub custom: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x14:CT_Cfvo/x14:cfvo"))]
  pub x14_cfvo: Vec<ConditionalFormattingValueObject>,
  /// _
  #[sdk(child(qname = "x14:CT_CfIcon/x14:cfIcon"))]
  pub x14_cf_icon: Vec<ConditionalFormattingIcon>,
}
/// Defines the DifferentialType Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:dxf.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dxf/x14:dxf")]
pub struct DifferentialType {
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
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:cfvo.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_Cfvo/x14:cfvo")]
pub struct ConditionalFormattingValueObject {
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
  /// type
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: ConditionalFormattingValueObjectTypeValues,
  /// gte
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :gte
  #[sdk(attr(qname = ":gte"))]
  pub greater_than_or_equal: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(text_child(qname = "x:ST_Formula/xne:f"))]
  pub formula: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ConditionalFormattingIcon Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:cfIcon.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_CfIcon/x14:cfIcon")]
pub struct ConditionalFormattingIcon {
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
  /// iconSet
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :iconSet
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set: IconSetTypeValues,
  /// iconId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :iconId
  #[sdk(attr(qname = ":iconId"))]
  pub icon_id: crate::simple_type::UInt32Value,
}
/// Defines the PivotEdits Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:pivotEdits.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_PivotEdits/x14:pivotEdits")]
pub struct PivotEdits {
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
  #[sdk(child(qname = "x14:CT_PivotEdit/x14:pivotEdit"))]
  pub x14_pivot_edit: Vec<PivotEdit>,
}
/// Defines the PivotChanges Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:pivotChanges.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_PivotChanges/x14:pivotChanges")]
pub struct PivotChanges {
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
  #[sdk(child(qname = "x14:CT_PivotChange/x14:pivotChange"))]
  pub x14_pivot_change: Vec<PivotChange>,
}
/// Defines the ConditionalFormats Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:conditionalFormats.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_ConditionalFormats/x14:conditionalFormats")]
pub struct ConditionalFormats {
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
  /// count
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x14:CT_ConditionalFormat/x14:conditionalFormat"))]
  pub x14_conditional_format: Vec<ConditionalFormat>,
}
/// Defines the CalculatedMembers Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:calculatedMembers.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_CalculatedMembers/x14:calculatedMembers")]
pub struct CalculatedMembers {
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
  /// Calculated Members Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_CalculatedMember/x:calculatedMember"))]
  pub x_calculated_member:
    Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CalculatedMember>,
}
/// Defines the PivotEdit Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:pivotEdit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_PivotEdit/x14:pivotEdit")]
pub struct PivotEdit {
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
  #[sdk(child(qname = "x14:CT_PivotUserEdit/x14:userEdit"))]
  pub pivot_user_edit: std::boxed::Box<PivotUserEdit>,
  /// _
  #[sdk(child(qname = "x14:CT_TupleItems/x14:tupleItems"))]
  pub tuple_items: std::boxed::Box<TupleItems>,
  /// _
  #[sdk(child(qname = "x:CT_PivotArea/x14:pivotArea"))]
  pub pivot_area: std::boxed::Box<PivotArea>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotUserEdit Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:userEdit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_PivotUserEdit/x14:userEdit")]
pub struct PivotUserEdit {
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
  #[sdk(choice(
    qname = "x:ST_Formula/xne:f",
    qname = "x14:CT_PivotEditValue/x14:editValue"
  ))]
  pub xml_children: Option<PivotUserEditChoice>,
}
/// Defines the TupleItems Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:tupleItems.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_TupleItems/x14:tupleItems")]
pub struct TupleItems {
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
  #[sdk(text_child(qname = "x:ST_Xstring/x14:tupleItem"))]
  pub x14_tuple_item: Vec<crate::simple_type::StringValue>,
}
/// Defines the PivotArea Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:pivotArea.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotArea/x14:pivotArea")]
pub struct PivotArea {
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
  /// Field Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :field
  #[sdk(attr(qname = ":field"))]
  pub field: Option<crate::simple_type::Int32Value>,
  /// Rule Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotAreaValues>,
  /// Data Only
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dataOnly
  #[sdk(attr(qname = ":dataOnly"))]
  pub data_only: Option<crate::simple_type::BooleanValue>,
  /// Labels Only
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :labelOnly
  #[sdk(attr(qname = ":labelOnly"))]
  pub label_only: Option<crate::simple_type::BooleanValue>,
  /// Include Row Grand Total
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :grandRow
  #[sdk(attr(qname = ":grandRow"))]
  pub grand_row: Option<crate::simple_type::BooleanValue>,
  /// Include Column Grand Total
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :grandCol
  #[sdk(attr(qname = ":grandCol"))]
  pub grand_column: Option<crate::simple_type::BooleanValue>,
  /// Cache Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cacheIndex
  #[sdk(attr(qname = ":cacheIndex"))]
  pub cache_index: Option<crate::simple_type::BooleanValue>,
  /// Outline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :outline
  #[sdk(attr(qname = ":outline"))]
  pub outline: Option<crate::simple_type::BooleanValue>,
  /// Offset Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :offset
  #[sdk(attr(qname = ":offset"))]
  pub offset: Option<crate::simple_type::StringValue>,
  /// Collapsed Levels Are Subtotals
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :collapsedLevelsAreSubtotals
  #[sdk(attr(qname = ":collapsedLevelsAreSubtotals"))]
  pub collapsed_levels_are_subtotals: Option<crate::simple_type::BooleanValue>,
  /// Axis
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :axis
  #[sdk(attr(qname = ":axis"))]
  pub axis: Option<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableAxisValues,
  >,
  /// Field Position
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fieldPosition
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
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:pivotChange.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_PivotChange/x14:pivotChange")]
pub struct PivotChange {
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
  /// allocationMethod
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :allocationMethod
  #[sdk(attr(qname = ":allocationMethod"))]
  pub allocation_method: Option<AllocationMethodValues>,
  /// weightExpression
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :weightExpression
  #[sdk(attr(qname = ":weightExpression"))]
  pub weight_expression: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x14:CT_PivotEditValue/x14:editValue"))]
  pub pivot_edit_value: std::boxed::Box<PivotEditValue>,
  /// _
  #[sdk(child(qname = "x14:CT_TupleItems/x14:tupleItems"))]
  pub tuple_items: std::boxed::Box<TupleItems>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotEditValue Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:editValue.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_PivotEditValue/x14:editValue")]
pub struct PivotEditValue {
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
  /// valueType
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :valueType
  #[sdk(attr(qname = ":valueType"))]
  pub value_type: PivotEditValueTypeValues,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Xstring Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:tupleItem.
pub type Xstring = crate::simple_type::StringValue;
/// Defines the SlicerStyleElements Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:slicerStyleElements.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SlicerStyleElements/x14:slicerStyleElements")]
pub struct SlicerStyleElements {
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
  #[sdk(child(qname = "x14:CT_SlicerStyleElement/x14:slicerStyleElement"))]
  pub x14_slicer_style_element: Vec<SlicerStyleElement>,
}
/// Defines the DdeValues Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:values.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_DdeValues/x14:values")]
pub struct DdeValues {
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
  /// Rows
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rows
  #[sdk(attr(qname = ":rows"))]
  pub rows: Option<crate::simple_type::UInt32Value>,
  /// Columns
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cols
  #[sdk(attr(qname = ":cols"))]
  pub columns: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_DdeValue/x:value"))]
  pub x_value: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Value>,
}
/// Defines the ConditionalFormat Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:conditionalFormat.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_ConditionalFormat/x14:conditionalFormat")]
pub struct ConditionalFormat {
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
  /// scope
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :scope
  #[sdk(attr(qname = ":scope"))]
  pub scope:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ScopeValues>,
  /// type
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::RuleValues>,
  /// priority
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :priority
  #[sdk(attr(qname = ":priority"))]
  pub priority: Option<crate::simple_type::UInt32Value>,
  /// id
  ///
  /// Available in Office2010 and above.
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
  #[sdk(child(qname = "x:CT_PivotAreas/x14:pivotAreas"))]
  pub pivot_areas: Option<PivotAreas>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotAreas Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:pivotAreas.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotAreas/x14:pivotAreas")]
pub struct PivotAreas {
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
  /// Pivot Area Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_PivotArea/x:pivotArea"))]
  pub x_pivot_area:
    Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotArea>,
}
/// Defines the SlicerStyle Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:slicerStyle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SlicerStyle/x14:slicerStyle")]
pub struct SlicerStyle {
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
  /// name
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x14:CT_SlicerStyleElements/x14:slicerStyleElements"))]
  pub slicer_style_elements: Option<SlicerStyleElements>,
}
/// Defines the SlicerStyleElement Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:slicerStyleElement.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SlicerStyleElement/x14:slicerStyleElement")]
pub struct SlicerStyleElement {
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
  /// type
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: SlicerStyleTypeValues,
  /// dxfId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :dxfId
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
}
/// Defines the IgnoredError Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:ignoredError.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_IgnoredError/x14:ignoredError")]
pub struct IgnoredError {
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
  /// evalError
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :evalError
  #[sdk(attr(qname = ":evalError"))]
  pub eval_error: Option<crate::simple_type::BooleanValue>,
  /// twoDigitTextYear
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :twoDigitTextYear
  #[sdk(attr(qname = ":twoDigitTextYear"))]
  pub two_digit_text_year: Option<crate::simple_type::BooleanValue>,
  /// numberStoredAsText
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :numberStoredAsText
  #[sdk(attr(qname = ":numberStoredAsText"))]
  pub number_stored_as_text: Option<crate::simple_type::BooleanValue>,
  /// formula
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :formula
  #[sdk(attr(qname = ":formula"))]
  pub formula: Option<crate::simple_type::BooleanValue>,
  /// formulaRange
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :formulaRange
  #[sdk(attr(qname = ":formulaRange"))]
  pub formula_range: Option<crate::simple_type::BooleanValue>,
  /// unlockedFormula
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :unlockedFormula
  #[sdk(attr(qname = ":unlockedFormula"))]
  pub unlocked_formula: Option<crate::simple_type::BooleanValue>,
  /// emptyCellReference
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :emptyCellReference
  #[sdk(attr(qname = ":emptyCellReference"))]
  pub empty_cell_reference: Option<crate::simple_type::BooleanValue>,
  /// listDataValidation
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :listDataValidation
  #[sdk(attr(qname = ":listDataValidation"))]
  pub list_data_validation: Option<crate::simple_type::BooleanValue>,
  /// calculatedColumn
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :calculatedColumn
  #[sdk(attr(qname = ":calculatedColumn"))]
  pub calculated_column: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(text_child(qname = "xne:ST_Sqref/xne:sqref"))]
  pub reference_sequence: crate::simple_type::StringValue,
}
/// Defines the ProtectedRange Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:protectedRange.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_ProtectedRange/x14:protectedRange")]
pub struct ProtectedRange {
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
  /// password
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :password
  #[sdk(attr(qname = ":password"))]
  #[sdk(string_length(source = 0u32, min = 2u32, max = 2u32))]
  pub password: Option<crate::simple_type::HexBinaryValue>,
  /// algorithmName
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :algorithmName
  #[sdk(attr(qname = ":algorithmName"))]
  pub algorithm_name: Option<crate::simple_type::StringValue>,
  /// hashValue
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :hashValue
  #[sdk(attr(qname = ":hashValue"))]
  pub hash_value: Option<crate::simple_type::Base64BinaryValue>,
  /// saltValue
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :saltValue
  #[sdk(attr(qname = ":saltValue"))]
  pub salt_value: Option<crate::simple_type::Base64BinaryValue>,
  /// spinCount
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :spinCount
  #[sdk(attr(qname = ":spinCount"))]
  pub spin_count: Option<crate::simple_type::UInt32Value>,
  /// name
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// securityDescriptor
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :securityDescriptor
  #[sdk(attr(qname = ":securityDescriptor"))]
  pub security_descriptor: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xne:ST_Sqref/xne:sqref"))]
  pub reference_sequence: crate::simple_type::StringValue,
}
/// Defines the CustomFilter Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:customFilter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_CustomFilter/x14:customFilter")]
pub struct CustomFilter {
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
  /// operator
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :operator
  #[sdk(attr(qname = ":operator"))]
  pub operator: Option<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::FilterOperatorValues,
  >,
  /// val
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Defines the ListItem Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:item.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_ListItem/x14:item")]
pub struct ListItem {
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
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the ListItems Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:itemLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_ListItems/x14:itemLst")]
pub struct ListItems {
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
  #[sdk(child(qname = "x14:CT_ListItem/x14:item"))]
  pub x14_item: Vec<ListItem>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x14:extLst"))]
  pub x14_ext_lst: Option<ExtensionList>,
}
/// Defines the Slicer Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:slicer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_Slicer/x14:slicer")]
pub struct Slicer {
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
  /// name
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// cache
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :cache
  #[sdk(attr(qname = ":cache"))]
  pub cache: crate::simple_type::StringValue,
  /// caption
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :caption
  #[sdk(attr(qname = ":caption"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// startItem
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :startItem
  #[sdk(attr(qname = ":startItem"))]
  pub start_item: Option<crate::simple_type::UInt32Value>,
  /// columnCount
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :columnCount
  #[sdk(attr(qname = ":columnCount"))]
  pub column_count: Option<crate::simple_type::UInt32Value>,
  /// showCaption
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showCaption
  #[sdk(attr(qname = ":showCaption"))]
  pub show_caption: Option<crate::simple_type::BooleanValue>,
  /// level
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :level
  #[sdk(attr(qname = ":level"))]
  pub level: Option<crate::simple_type::UInt32Value>,
  /// style
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// lockedPosition
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :lockedPosition
  #[sdk(attr(qname = ":lockedPosition"))]
  pub locked_position: Option<crate::simple_type::BooleanValue>,
  /// rowHeight
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :rowHeight
  #[sdk(attr(qname = ":rowHeight"))]
  pub row_height: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the OlapSlicerCache Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:olap.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_OlapSlicerCache/x14:olap")]
pub struct OlapSlicerCache {
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
  /// pivotCacheId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :pivotCacheId
  #[sdk(attr(qname = ":pivotCacheId"))]
  pub pivot_cache_id: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "x14:CT_OlapSlicerCacheLevelsData/x14:levels"))]
  pub olap_slicer_cache_levels_data: std::boxed::Box<OlapSlicerCacheLevelsData>,
  /// _
  #[sdk(child(qname = "x14:CT_OlapSlicerCacheSelections/x14:selections"))]
  pub olap_slicer_cache_selections: std::boxed::Box<OlapSlicerCacheSelections>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TabularSlicerCache Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:tabular.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_TabularSlicerCache/x14:tabular")]
pub struct TabularSlicerCache {
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
  /// pivotCacheId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :pivotCacheId
  #[sdk(attr(qname = ":pivotCacheId"))]
  pub pivot_cache_id: crate::simple_type::UInt32Value,
  /// sortOrder
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :sortOrder
  #[sdk(attr(qname = ":sortOrder"))]
  pub sort_order: Option<TabularSlicerCacheSortOrderValues>,
  /// customListSort
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :customListSort
  #[sdk(attr(qname = ":customListSort"))]
  pub custom_list_sort: Option<crate::simple_type::BooleanValue>,
  /// showMissing
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :showMissing
  #[sdk(attr(qname = ":showMissing"))]
  pub show_missing: Option<crate::simple_type::BooleanValue>,
  /// crossFilter
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :crossFilter
  #[sdk(attr(qname = ":crossFilter"))]
  pub cross_filter: Option<SlicerCacheCrossFilterValues>,
  /// _
  #[sdk(child(qname = "x14:CT_TabularSlicerCacheItems/x14:items"))]
  pub tabular_slicer_cache_items: std::boxed::Box<TabularSlicerCacheItems>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the SlicerCachePivotTable Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:pivotTable.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SlicerCachePivotTable/x14:pivotTable")]
pub struct SlicerCachePivotTable {
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
  /// tabId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :tabId
  #[sdk(attr(qname = ":tabId"))]
  pub tab_id: crate::simple_type::UInt32Value,
  /// name
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// Defines the OlapSlicerCacheItemParent Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:p.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_OlapSlicerCacheItemParent/x14:p")]
pub struct OlapSlicerCacheItemParent {
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
  /// n
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub name: crate::simple_type::StringValue,
}
/// Defines the OlapSlicerCacheItem Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:i.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_OlapSlicerCacheItem/x14:i")]
pub struct OlapSlicerCacheItem {
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
  /// n
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub name: crate::simple_type::StringValue,
  /// c
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :c
  #[sdk(attr(qname = ":c"))]
  pub display_name: Option<crate::simple_type::StringValue>,
  /// nd
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :nd
  #[sdk(attr(qname = ":nd"))]
  pub non_display: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x14:CT_OlapSlicerCacheItemParent/x14:p"))]
  pub x14_p: Vec<OlapSlicerCacheItemParent>,
}
/// Defines the OlapSlicerCacheRange Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:range.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_OlapSlicerCacheRange/x14:range")]
pub struct OlapSlicerCacheRange {
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
  /// startItem
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :startItem
  #[sdk(attr(qname = ":startItem"))]
  pub start_item: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "x14:CT_OlapSlicerCacheItem/x14:i"))]
  pub x14_i: Vec<OlapSlicerCacheItem>,
}
/// Defines the OlapSlicerCacheRanges Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:ranges.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_OlapSlicerCacheRanges/x14:ranges")]
pub struct OlapSlicerCacheRanges {
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
  #[sdk(child(qname = "x14:CT_OlapSlicerCacheRange/x14:range"))]
  pub x14_range: Vec<OlapSlicerCacheRange>,
}
/// Defines the OlapSlicerCacheLevelData Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:level.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_OlapSlicerCacheLevelData/x14:level")]
pub struct OlapSlicerCacheLevelData {
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
  /// uniqueName
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :uniqueName
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// sourceCaption
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :sourceCaption
  #[sdk(attr(qname = ":sourceCaption"))]
  pub source_caption: Option<crate::simple_type::StringValue>,
  /// count
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// sortOrder
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :sortOrder
  #[sdk(attr(qname = ":sortOrder"))]
  pub sort_order: Option<OlapSlicerCacheSortOrderValues>,
  /// crossFilter
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :crossFilter
  #[sdk(attr(qname = ":crossFilter"))]
  pub cross_filter: Option<SlicerCacheCrossFilterValues>,
  /// _
  #[sdk(child(qname = "x14:CT_OlapSlicerCacheRanges/x14:ranges"))]
  pub olap_slicer_cache_ranges: Option<OlapSlicerCacheRanges>,
}
/// Defines the OlapSlicerCacheLevelsData Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:levels.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_OlapSlicerCacheLevelsData/x14:levels")]
pub struct OlapSlicerCacheLevelsData {
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
  /// count
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x14:CT_OlapSlicerCacheLevelData/x14:level"))]
  pub x14_level: Vec<OlapSlicerCacheLevelData>,
}
/// Defines the OlapSlicerCacheSelections Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:selections.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_OlapSlicerCacheSelections/x14:selections")]
pub struct OlapSlicerCacheSelections {
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
  /// count
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x14:CT_OlapSlicerCacheSelection/x14:selection"))]
  pub x14_selection: Vec<OlapSlicerCacheSelection>,
}
/// Defines the OlapSlicerCacheSelection Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:selection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_OlapSlicerCacheSelection/x14:selection")]
pub struct OlapSlicerCacheSelection {
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
  /// n
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub name: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x14:CT_OlapSlicerCacheItemParent/x14:p"))]
  pub x14_p: Vec<OlapSlicerCacheItemParent>,
}
/// Defines the TabularSlicerCacheItems Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:items.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_TabularSlicerCacheItems/x14:items")]
pub struct TabularSlicerCacheItems {
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
  /// count
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x14:CT_TabularSlicerCacheItem/x14:i"))]
  pub x14_i: Vec<TabularSlicerCacheItem>,
}
/// Defines the TabularSlicerCacheItem Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:i.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_TabularSlicerCacheItem/x14:i")]
pub struct TabularSlicerCacheItem {
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
  /// x
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :x
  #[sdk(attr(qname = ":x"))]
  pub atom: crate::simple_type::UInt32Value,
  /// s
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :s
  #[sdk(attr(qname = ":s"))]
  pub is_selected: Option<crate::simple_type::BooleanValue>,
  /// nd
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :nd
  #[sdk(attr(qname = ":nd"))]
  pub non_display: Option<crate::simple_type::BooleanValue>,
}
/// Defines the SlicerCachePivotTables Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:pivotTables.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SlicerCachePivotTables/x14:pivotTables")]
pub struct SlicerCachePivotTables {
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
  #[sdk(child(qname = "x14:CT_SlicerCachePivotTable/x14:pivotTable"))]
  pub x14_pivot_table: Vec<SlicerCachePivotTable>,
}
/// Defines the SlicerCacheData Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SlicerCacheData/x14:data")]
pub struct SlicerCacheData {
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
  #[sdk(choice(
    qname = "x14:CT_OlapSlicerCache/x14:olap",
    qname = "x14:CT_TabularSlicerCache/x14:tabular"
  ))]
  pub xml_children: Option<SlicerCacheDataChoice>,
}
/// Defines the SlicerCacheDefinitionExtensionList Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is x14:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_SlicerCacheDefinitionExtensionList/x14:extLst")]
pub struct SlicerCacheDefinitionExtensionList {
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
    #[sdk(child(qname = "x:CT_SlicerCacheDefinitionExtension/x:ext"))]
    pub x_ext: Vec<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SlicerCacheDefinitionExtension,
    >,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PivotUserEditChoice {
  /// Defines the Formula Class.
  #[sdk(text_child(qname = "x:ST_Formula/xne:f"))]
  XneF(crate::simple_type::StringValue),
  /// Defines the PivotEditValue Class.
  #[sdk(child(qname = "x14:CT_PivotEditValue/x14:editValue"))]
  X14EditValue(std::boxed::Box<PivotEditValue>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SlicerCacheDataChoice {
  /// Defines the OlapSlicerCache Class.
  #[sdk(child(qname = "x14:CT_OlapSlicerCache/x14:olap"))]
  X14Olap(std::boxed::Box<OlapSlicerCache>),
  /// Defines the TabularSlicerCache Class.
  #[sdk(child(qname = "x14:CT_TabularSlicerCache/x14:tabular"))]
  X14Tabular(std::boxed::Box<TabularSlicerCache>),
}
