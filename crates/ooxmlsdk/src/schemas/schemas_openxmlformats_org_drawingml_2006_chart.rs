//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LayoutTargetValues {
  #[sdk(rename = "inner")]
  #[default]
  Inner,
  #[sdk(rename = "outer")]
  Outer,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LayoutModeValues {
  #[sdk(rename = "edge")]
  #[default]
  Edge,
  #[sdk(rename = "factor")]
  Factor,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SizeRepresentsValues {
  #[sdk(rename = "area")]
  #[default]
  Area,
  #[sdk(rename = "w")]
  Width,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LabelAlignmentValues {
  #[sdk(rename = "ctr")]
  #[default]
  Center,
  #[sdk(rename = "l")]
  Left,
  #[sdk(rename = "r")]
  Right,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DataLabelPositionValues {
  #[sdk(rename = "bestFit")]
  #[default]
  BestFit,
  #[sdk(rename = "b")]
  Bottom,
  #[sdk(rename = "ctr")]
  Center,
  #[sdk(rename = "inBase")]
  InsideBase,
  #[sdk(rename = "inEnd")]
  InsideEnd,
  #[sdk(rename = "l")]
  Left,
  #[sdk(rename = "outEnd")]
  OutsideEnd,
  #[sdk(rename = "r")]
  Right,
  #[sdk(rename = "t")]
  Top,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TrendlineValues {
  #[sdk(rename = "exp")]
  #[default]
  Exponential,
  #[sdk(rename = "linear")]
  Linear,
  #[sdk(rename = "log")]
  Logarithmic,
  #[sdk(rename = "movingAvg")]
  MovingAverage,
  #[sdk(rename = "poly")]
  Polynomial,
  #[sdk(rename = "power")]
  Power,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ErrorBarDirectionValues {
  #[sdk(rename = "x")]
  #[default]
  X,
  #[sdk(rename = "y")]
  Y,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ErrorBarValues {
  #[sdk(rename = "both")]
  #[default]
  Both,
  #[sdk(rename = "minus")]
  Minus,
  #[sdk(rename = "plus")]
  Plus,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ErrorValues {
  #[sdk(rename = "cust")]
  #[default]
  Custom,
  #[sdk(rename = "fixedVal")]
  FixedValue,
  #[sdk(rename = "percentage")]
  Percentage,
  #[sdk(rename = "stdDev")]
  StandardDeviation,
  #[sdk(rename = "stdErr")]
  StandardError,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum GroupingValues {
  #[sdk(rename = "percentStacked")]
  #[default]
  PercentStacked,
  #[sdk(rename = "standard")]
  Standard,
  #[sdk(rename = "stacked")]
  Stacked,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RadarStyleValues {
  #[sdk(rename = "standard")]
  #[default]
  Standard,
  #[sdk(rename = "marker")]
  Marker,
  #[sdk(rename = "filled")]
  Filled,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BarGroupingValues {
  #[sdk(rename = "percentStacked")]
  #[default]
  PercentStacked,
  #[sdk(rename = "clustered")]
  Clustered,
  #[sdk(rename = "standard")]
  Standard,
  #[sdk(rename = "stacked")]
  Stacked,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BarDirectionValues {
  #[sdk(rename = "bar")]
  #[default]
  Bar,
  #[sdk(rename = "col")]
  Column,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ShapeValues {
  #[sdk(rename = "cone")]
  #[default]
  Cone,
  #[sdk(rename = "coneToMax")]
  ConeToMax,
  #[sdk(rename = "box")]
  Box,
  #[sdk(rename = "cylinder")]
  Cylinder,
  #[sdk(rename = "pyramid")]
  Pyramid,
  #[sdk(rename = "pyramidToMax")]
  PyramidToMaximum,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum OfPieValues {
  #[sdk(rename = "pie")]
  #[default]
  Pie,
  #[sdk(rename = "bar")]
  Bar,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AxisPositionValues {
  #[sdk(rename = "b")]
  #[default]
  Bottom,
  #[sdk(rename = "l")]
  Left,
  #[sdk(rename = "r")]
  Right,
  #[sdk(rename = "t")]
  Top,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CrossesValues {
  #[sdk(rename = "autoZero")]
  #[default]
  AutoZero,
  #[sdk(rename = "max")]
  Maximum,
  #[sdk(rename = "min")]
  Minimum,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CrossBetweenValues {
  #[sdk(rename = "between")]
  #[default]
  Between,
  #[sdk(rename = "midCat")]
  MidpointCategory,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TickMarkValues {
  #[sdk(rename = "cross")]
  #[default]
  Cross,
  #[sdk(rename = "in")]
  Inside,
  #[sdk(rename = "none")]
  None,
  #[sdk(rename = "out")]
  Outside,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TickLabelPositionValues {
  #[sdk(rename = "high")]
  #[default]
  High,
  #[sdk(rename = "low")]
  Low,
  #[sdk(rename = "nextTo")]
  NextTo,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TimeUnitValues {
  #[sdk(rename = "days")]
  #[default]
  Days,
  #[sdk(rename = "months")]
  Months,
  #[sdk(rename = "years")]
  Years,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum BuiltInUnitValues {
  #[sdk(rename = "hundreds")]
  #[default]
  Hundreds,
  #[sdk(rename = "thousands")]
  Thousands,
  #[sdk(rename = "tenThousands")]
  TenThousands,
  #[sdk(rename = "hundredThousands")]
  HundredThousands,
  #[sdk(rename = "millions")]
  Millions,
  #[sdk(rename = "tenMillions")]
  TenMillions,
  #[sdk(rename = "hundredMillions")]
  HundredMillions,
  #[sdk(rename = "billions")]
  Billions,
  #[sdk(rename = "trillions")]
  Trillions,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PictureFormatValues {
  #[sdk(rename = "stretch")]
  #[default]
  Stretch,
  #[sdk(rename = "stack")]
  Stack,
  #[sdk(rename = "stackScale")]
  StackScale,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum OrientationValues {
  #[sdk(rename = "maxMin")]
  #[default]
  MaxMin,
  #[sdk(rename = "minMax")]
  MinMax,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LegendPositionValues {
  #[sdk(rename = "b")]
  #[default]
  Bottom,
  #[sdk(rename = "tr")]
  TopRight,
  #[sdk(rename = "l")]
  Left,
  #[sdk(rename = "r")]
  Right,
  #[sdk(rename = "t")]
  Top,
}
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
pub enum PageSetupOrientationValues {
  #[sdk(rename = "default")]
  #[default]
  Default,
  #[sdk(rename = "portrait")]
  Portrait,
  #[sdk(rename = "landscape")]
  Landscape,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ScatterStyleValues {
  #[sdk(rename = "line")]
  #[default]
  Line,
  #[sdk(rename = "lineMarker")]
  LineMarker,
  #[sdk(rename = "marker")]
  Marker,
  #[sdk(rename = "smooth")]
  Smooth,
  #[sdk(rename = "smoothMarker")]
  SmoothMarker,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum MarkerStyleValues {
  #[sdk(rename = "auto")]
  #[default]
  Auto,
  #[sdk(rename = "circle")]
  Circle,
  #[sdk(rename = "dash")]
  Dash,
  #[sdk(rename = "diamond")]
  Diamond,
  #[sdk(rename = "dot")]
  Dot,
  #[sdk(rename = "none")]
  None,
  #[sdk(rename = "picture")]
  Picture,
  #[sdk(rename = "plus")]
  Plus,
  #[sdk(rename = "square")]
  Square,
  #[sdk(rename = "star")]
  Star,
  #[sdk(rename = "triangle")]
  Triangle,
  #[sdk(rename = "x")]
  X,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SplitValues {
  #[sdk(rename = "cust")]
  #[default]
  Custom,
  #[sdk(rename = "percent")]
  Percent,
  #[sdk(rename = "pos")]
  Position,
  #[sdk(rename = "val")]
  Value,
}
/// Number Format.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:numFmt.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumFmt/c:numFmt")]
pub struct NumberingFormat {
  /// Number Format Code
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :formatCode
  #[sdk(attr(qname = ":formatCode"))]
  pub format_code: crate::simple_type::StringValue,
  /// Linked to Source
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sourceLinked
  #[sdk(attr(qname = ":sourceLinked"))]
  pub source_linked: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ChartShapeProperties Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:spPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ChartShapeProperties/c:spPr")]
pub struct ChartShapeProperties {
  /// Black and White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bwMode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  /// 2D Transform for Individual Objects
  #[sdk(child(qname = "a:CT_Transform2D/a:xfrm"))]
  pub transform2_d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Transform2D>,
  >,
  #[sdk(choice(
    qname = "a:CT_CustomGeometry2D/a:custGeom",
    qname = "a:CT_PresetGeometry2D/a:prstGeom"
  ))]
  pub chart_shape_properties_choice1: Option<ChartShapePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill"
  ))]
  pub chart_shape_properties_choice2: Option<ChartShapePropertiesChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub a_ln: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline>,
  >,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub chart_shape_properties_choice3: Option<ChartShapePropertiesChoice3>,
  /// _
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  pub a_sp3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the TextProperties Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:txPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBody/c:txPr")]
pub struct TextProperties {
  /// Body Properties
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  pub body_properties:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BodyProperties>,
  /// Text List Styles
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  pub list_style: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ListStyle>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_TextParagraph/a:p"))]
  pub a_p: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Paragraph>,
}
/// Rich Text.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:rich.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBody/c:rich")]
pub struct RichText {
  /// Body Properties
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  pub body_properties:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BodyProperties>,
  /// Text List Styles
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  pub list_style: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ListStyle>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_TextParagraph/a:p"))]
  pub a_p: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Paragraph>,
}
/// Defines the TextBodyType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBody/")]
pub struct TextBodyType {
  #[sdk(choice(
    qname = "a:CT_TextBodyProperties/a:bodyPr",
    qname = "a:CT_TextListStyle/a:lstStyle",
    qname = "a:CT_TextParagraph/a:p"
  ))]
  pub xml_children: Vec<TextBodyTypeChoice>,
}
/// Data Label Position.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:dLblPos.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DLblPos/c:dLblPos")]
pub struct DataLabelPosition {
  /// Data Label Position Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: DataLabelPositionValues,
}
/// Show Legend Key.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:showLegendKey.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showLegendKey")]
pub struct ShowLegendKey {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:showVal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showVal")]
pub struct ShowValue {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Category Name.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:showCatName.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showCatName")]
pub struct ShowCategoryName {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Series Name.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:showSerName.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showSerName")]
pub struct ShowSeriesName {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Percent.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:showPercent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showPercent")]
pub struct ShowPercent {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Bubble Size.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:showBubbleSize.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showBubbleSize")]
pub struct ShowBubbleSize {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Leader Lines.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:showLeaderLines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showLeaderLines")]
pub struct ShowLeaderLines {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the VaryColors Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:varyColors.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:varyColors")]
pub struct VaryColors {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Wireframe.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:wireframe.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:wireframe")]
pub struct Wireframe {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Delete.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:delete.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:delete")]
pub struct Delete {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Overlay.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:overlay.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:overlay")]
pub struct Overlay {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Right Angle Axes.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:rAngAx.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:rAngAx")]
pub struct RightAngleAxes {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Horizontal Border.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:showHorzBorder.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showHorzBorder")]
pub struct ShowHorizontalBorder {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Vertical Border.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:showVertBorder.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showVertBorder")]
pub struct ShowVerticalBorder {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Outline Border.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:showOutline.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showOutline")]
pub struct ShowOutlineBorder {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Legend Keys.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:showKeys.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showKeys")]
pub struct ShowKeys {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Invert if Negative.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:invertIfNegative.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:invertIfNegative")]
pub struct InvertIfNegative {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// 3D Bubble.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:bubble3D.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:bubble3D")]
pub struct Bubble3D {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Display R Squared Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:dispRSqr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:dispRSqr")]
pub struct DisplayRSquaredValue {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Display Equation.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:dispEq.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:dispEq")]
pub struct DisplayEquation {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// No End Cap.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:noEndCap.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:noEndCap")]
pub struct NoEndCap {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Apply To Front.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:applyToFront.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:applyToFront")]
pub struct ApplyToFront {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Apply To Sides.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:applyToSides.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:applyToSides")]
pub struct ApplyToSides {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Apply to End.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:applyToEnd.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:applyToEnd")]
pub struct ApplyToEnd {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Chart Object.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:chartObject.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:chartObject")]
pub struct ChartObject {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Data Cannot Be Changed.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:data")]
pub struct Data {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Formatting.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:formatting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:formatting")]
pub struct Formatting {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Selection.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:selection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:selection")]
pub struct Selection {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// User Interface.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:userInterface.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:userInterface")]
pub struct UserInterface {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Update Automatically.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:autoUpdate.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:autoUpdate")]
pub struct AutoUpdate {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ShowMarker Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:marker.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:marker")]
pub struct ShowMarker {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Smooth Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:smooth.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:smooth")]
pub struct Smooth {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ShowNegativeBubbles Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:showNegBubbles.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showNegBubbles")]
pub struct ShowNegativeBubbles {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the AutoLabeled Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:auto.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:auto")]
pub struct AutoLabeled {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the NoMultiLevelLabels Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:noMultiLvlLbl.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:noMultiLvlLbl")]
pub struct NoMultiLevelLabels {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// True if the chart automatic title has been deleted..
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:autoTitleDeleted.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:autoTitleDeleted")]
pub struct AutoTitleDeleted {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// True if only visible cells are plotted..
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:plotVisOnly.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:plotVisOnly")]
pub struct PlotVisibleOnly {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// True if we should render datalabels over the maximum scale.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:showDLblsOverMax.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showDLblsOverMax")]
pub struct ShowDataLabelsOverMaximum {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Date1904 Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:date1904.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:date1904")]
pub struct Date1904 {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the RoundedCorners Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:roundedCorners.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:roundedCorners")]
pub struct RoundedCorners {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the BooleanType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/")]
pub struct BooleanType {
  /// Boolean Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Separator.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:separator.
pub type Separator = crate::simple_type::StringValue;
/// Trendline Name.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:name.
pub type TrendlineName = crate::simple_type::StringValue;
/// Defines the Formula Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:f.
pub type Formula = crate::simple_type::StringValue;
/// Layout.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:layout.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Layout/c:layout")]
pub struct Layout {
  /// Manual Layout
  #[sdk(child(qname = "c:CT_ManualLayout/c:manualLayout"))]
  pub manual_layout: Option<std::boxed::Box<ManualLayout>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ChartText Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:tx.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Tx/c:tx")]
pub struct ChartText {
  #[sdk(choice(
    qname = "c:CT_StrRef/c:strRef",
    qname = "a:CT_TextBody/c:rich",
    qname = "c:CT_StrData/c:strLit"
  ))]
  pub xml_children: Option<ChartTextChoice>,
}
/// Leader Lines.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:leaderLines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartLines/c:leaderLines")]
pub struct LeaderLines {
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Drop Lines.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:dropLines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartLines/c:dropLines")]
pub struct DropLines {
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Major Gridlines.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:majorGridlines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartLines/c:majorGridlines")]
pub struct MajorGridlines {
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Minor Gridlines.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:minorGridlines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartLines/c:minorGridlines")]
pub struct MinorGridlines {
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Defines the SeriesLines Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:serLines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartLines/c:serLines")]
pub struct SeriesLines {
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Defines the HighLowLines Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:hiLowLines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartLines/c:hiLowLines")]
pub struct HighLowLines {
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Defines the ChartLinesType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartLines/")]
pub struct ChartLinesType {
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Vec<ChartShapeProperties>,
}
/// Index.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:idx.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/c:idx")]
pub struct Index {
  /// Integer Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Order.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:order.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/c:order")]
pub struct Order {
  /// Integer Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Axis ID.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:axId.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/c:axId")]
pub struct AxisId {
  /// Integer Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Crossing Axis ID.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:crossAx.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/c:crossAx")]
pub struct CrossingAxis {
  /// Integer Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Point Count.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ptCount.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/c:ptCount")]
pub struct PointCount {
  /// Integer Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Second Pie Point.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:secondPiePt.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/c:secondPiePt")]
pub struct SecondPiePoint {
  /// Integer Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Explosion.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:explosion.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/c:explosion")]
pub struct Explosion {
  /// Integer Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Format ID.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:fmtId.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/c:fmtId")]
pub struct FormatId {
  /// Integer Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the UnsignedIntegerType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/")]
pub struct UnsignedIntegerType {
  /// Integer Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Series Text.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:tx.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SerTx/c:tx")]
pub struct SeriesText {
  #[sdk(choice(qname = "c:CT_StrRef/c:strRef", qname = "c:ST_Xstring/c:v"))]
  pub xml_children: Option<SeriesTextChoice>,
}
/// Grouping.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:grouping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Grouping/c:grouping")]
pub struct Grouping {
  /// Grouping Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<GroupingValues>,
}
/// Defines the LineChartSeries Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ser.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LineSer/c:ser")]
pub struct LineChartSeries {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: std::boxed::Box<Index>,
  /// Order
  #[sdk(child(qname = "c:CT_UnsignedInt/c:order"))]
  pub order: std::boxed::Box<Order>,
  /// Series Text
  #[sdk(child(qname = "c:CT_SerTx/c:tx"))]
  pub series_text: Option<std::boxed::Box<SeriesText>>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// _
  #[sdk(child(qname = "c:CT_Marker/c:marker"))]
  pub marker: Option<std::boxed::Box<Marker>>,
  /// _
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// _
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<DataPoint>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "c:CT_Trendline/c:trendline"))]
  pub c_trendline: Vec<Trendline>,
  /// _
  #[sdk(child(qname = "c:CT_ErrBars/c:errBars"))]
  pub c_err_bars: Option<std::boxed::Box<ErrorBars>>,
  /// _
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub c_cat: Option<std::boxed::Box<CategoryAxisData>>,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub c_val: Option<std::boxed::Box<Values>>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:smooth"))]
  pub c_smooth: Option<Smooth>,
  /// _
  #[sdk(child(qname = "c:CT_LineSerExtensionList/c:extLst"))]
  pub c_ext_lst: Option<LineSerExtensionList>,
}
/// Data Labels.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:dLbls.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DLbls/c:dLbls")]
pub struct DataLabels {
  /// _
  #[sdk(child(qname = "c:CT_DLbl/c:dLbl"))]
  pub c_d_lbl: Vec<DataLabel>,
  #[sdk(choice(
    qname = "c:CT_Boolean/c:delete",
    qname = "c:CT_NumFmt/c:numFmt",
    qname = "a:CT_ChartShapeProperties/c:spPr",
    qname = "a:CT_TextBody/c:txPr",
    qname = "c:CT_DLblPos/c:dLblPos",
    qname = "c:CT_Boolean/c:showLegendKey",
    qname = "c:CT_Boolean/c:showVal",
    qname = "c:CT_Boolean/c:showCatName",
    qname = "c:CT_Boolean/c:showSerName",
    qname = "c:CT_Boolean/c:showPercent",
    qname = "c:CT_Boolean/c:showBubbleSize",
    qname = "xsd:string/c:separator",
    qname = "c:CT_Boolean/c:showLeaderLines",
    qname = "c:CT_ChartLines/c:leaderLines"
  ))]
  pub data_labels_choice: Option<DataLabelsChoice>,
  /// _
  #[sdk(child(qname = "c:CT_DLblsExtensionList/c:extLst"))]
  pub c_ext_lst: Option<DLblsExtensionList>,
}
/// Bar Direction.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:barDir.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BarDir/c:barDir")]
pub struct BarDirection {
  /// Bar Direction Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: BarDirectionValues,
}
/// Bar Grouping.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:grouping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BarGrouping/c:grouping")]
pub struct BarGrouping {
  /// Bar Grouping Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<BarGroupingValues>,
}
/// Bar Chart Series.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ser.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BarSer/c:ser")]
pub struct BarChartSeries {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: std::boxed::Box<Index>,
  /// Order
  #[sdk(child(qname = "c:CT_UnsignedInt/c:order"))]
  pub order: std::boxed::Box<Order>,
  /// Series Text
  #[sdk(child(qname = "c:CT_SerTx/c:tx"))]
  pub series_text: Option<std::boxed::Box<SeriesText>>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:invertIfNegative"))]
  pub invert_if_negative: Option<InvertIfNegative>,
  /// _
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// _
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<DataPoint>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "c:CT_Trendline/c:trendline"))]
  pub c_trendline: Vec<Trendline>,
  /// _
  #[sdk(child(qname = "c:CT_ErrBars/c:errBars"))]
  pub c_err_bars: Option<std::boxed::Box<ErrorBars>>,
  /// _
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub c_cat: Option<std::boxed::Box<CategoryAxisData>>,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub c_val: Option<std::boxed::Box<Values>>,
  /// _
  #[sdk(child(qname = "c:CT_Shape/c:shape"))]
  pub c_shape: Option<Shape>,
  /// _
  #[sdk(child(qname = "c:CT_BarSerExtensionList/c:extLst"))]
  pub c_ext_lst: Option<BarSerExtensionList>,
}
/// Area Chart Series.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ser.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AreaSer/c:ser")]
pub struct AreaChartSeries {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: std::boxed::Box<Index>,
  /// Order
  #[sdk(child(qname = "c:CT_UnsignedInt/c:order"))]
  pub order: std::boxed::Box<Order>,
  /// Series Text
  #[sdk(child(qname = "c:CT_SerTx/c:tx"))]
  pub series_text: Option<std::boxed::Box<SeriesText>>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// _
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// _
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<DataPoint>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "c:CT_Trendline/c:trendline"))]
  pub c_trendline: Vec<Trendline>,
  /// _
  #[sdk(child(qname = "c:CT_ErrBars/c:errBars"))]
  pub c_err_bars: Vec<ErrorBars>,
  /// _
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub c_cat: Option<std::boxed::Box<CategoryAxisData>>,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub c_val: Option<std::boxed::Box<Values>>,
  /// _
  #[sdk(child(qname = "c:CT_AreaSerExtensionList/c:extLst"))]
  pub c_ext_lst: Option<AreaSerExtensionList>,
}
/// Pie Chart Series.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ser.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PieSer/c:ser")]
pub struct PieChartSeries {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: std::boxed::Box<Index>,
  /// Order
  #[sdk(child(qname = "c:CT_UnsignedInt/c:order"))]
  pub order: std::boxed::Box<Order>,
  /// Series Text
  #[sdk(child(qname = "c:CT_SerTx/c:tx"))]
  pub series_text: Option<std::boxed::Box<SeriesText>>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// _
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:explosion"))]
  pub explosion: Option<Explosion>,
  /// _
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<DataPoint>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub c_cat: Option<std::boxed::Box<CategoryAxisData>>,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub c_val: Option<std::boxed::Box<Values>>,
  /// _
  #[sdk(child(qname = "c:CT_PieSerExtensionList/c:extLst"))]
  pub c_ext_lst: Option<PieSerExtensionList>,
}
/// Surface Chart Series.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ser.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SurfaceSer/c:ser")]
pub struct SurfaceChartSeries {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: std::boxed::Box<Index>,
  /// Order
  #[sdk(child(qname = "c:CT_UnsignedInt/c:order"))]
  pub order: std::boxed::Box<Order>,
  /// Series Text
  #[sdk(child(qname = "c:CT_SerTx/c:tx"))]
  pub series_text: Option<std::boxed::Box<SeriesText>>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// _
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// _
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub category_axis_data: Option<std::boxed::Box<CategoryAxisData>>,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub values: Option<std::boxed::Box<Values>>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:bubble3D"))]
  pub bubble3_d: Option<Bubble3D>,
  /// _
  #[sdk(child(qname = "c:CT_SurfaceSerExtensionList/c:extLst"))]
  pub surface_ser_extension_list: Option<SurfaceSerExtensionList>,
}
/// Band Formats.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:bandFmts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BandFmts/c:bandFmts")]
pub struct BandFormats {
  /// _
  #[sdk(child(qname = "c:CT_BandFmt/c:bandFmt"))]
  pub c_band_fmt: Vec<BandFormat>,
}
/// Scaling.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:scaling.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Scaling/c:scaling")]
pub struct Scaling {
  /// Logarithmic Base
  #[sdk(child(qname = "c:CT_LogBase/c:logBase"))]
  pub log_base: Option<LogBase>,
  /// Axis Orientation
  #[sdk(child(qname = "c:CT_Orientation/c:orientation"))]
  pub orientation: Option<Orientation>,
  /// Maximum
  #[sdk(child(qname = "c:CT_Double/c:max"))]
  pub max_axis_value: Option<MaxAxisValue>,
  /// Minimum
  #[sdk(child(qname = "c:CT_Double/c:min"))]
  pub min_axis_value: Option<MinAxisValue>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Axis Position.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:axPos.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AxPos/c:axPos")]
pub struct AxisPosition {
  /// Axis Position Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: AxisPositionValues,
}
/// Title.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:title.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Title/c:title")]
pub struct Title {
  /// Chart Text
  #[sdk(child(qname = "c:CT_Tx/c:tx"))]
  pub chart_text: Option<std::boxed::Box<ChartText>>,
  /// Layout
  #[sdk(child(qname = "c:CT_Layout/c:layout"))]
  pub layout: Option<std::boxed::Box<Layout>>,
  /// Overlay
  #[sdk(child(qname = "c:CT_Boolean/c:overlay"))]
  pub overlay: Option<Overlay>,
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Major Tick Mark.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:majorTickMark.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TickMark/c:majorTickMark")]
pub struct MajorTickMark {
  /// Tick Mark Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<TickMarkValues>,
}
/// Minor Tick Mark.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:minorTickMark.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TickMark/c:minorTickMark")]
pub struct MinorTickMark {
  /// Tick Mark Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<TickMarkValues>,
}
/// Defines the TickMarkType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TickMark/")]
pub struct TickMarkType {
  /// Tick Mark Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<TickMarkValues>,
}
/// Tick Label Position.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:tickLblPos.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TickLblPos/c:tickLblPos")]
pub struct TickLabelPosition {
  /// Tick Label Position Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<TickLabelPositionValues>,
}
/// Crosses.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:crosses.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Crosses/c:crosses")]
pub struct Crosses {
  /// Crosses Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: CrossesValues,
}
/// Crossing Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:crossesAt.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:crossesAt")]
pub struct CrossesAt {
  /// Floating Point Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Left.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:x.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:x")]
pub struct Left {
  /// Floating Point Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Top.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:y.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:y")]
pub struct Top {
  /// Floating Point Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Width.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:w.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:w")]
pub struct Width {
  /// Floating Point Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Height.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:h.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:h")]
pub struct Height {
  /// Floating Point Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Forward.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:forward.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:forward")]
pub struct Forward {
  /// Floating Point Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Backward.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:backward.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:backward")]
pub struct Backward {
  /// Floating Point Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Intercept.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:intercept.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:intercept")]
pub struct Intercept {
  /// Floating Point Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Error Bar Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:val.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:val")]
pub struct ErrorBarValue {
  /// Floating Point Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Split Position.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:splitPos.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:splitPos")]
pub struct SplitPosition {
  /// Floating Point Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Custom Display Unit.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:custUnit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:custUnit")]
pub struct CustomDisplayUnit {
  /// Floating Point Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Maximum.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:max.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:max")]
pub struct MaxAxisValue {
  /// Floating Point Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Minimum.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:min.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:min")]
pub struct MinAxisValue {
  /// Floating Point Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Defines the DoubleType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/")]
pub struct DoubleType {
  /// Floating Point Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Chart Space.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:chartSpace.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartSpace/c:chartSpace")]
pub struct ChartSpace {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Version number of the file, as determined by the features used by this chart
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :version
  #[sdk(attr(qname = ":version"))]
  pub version: Option<crate::simple_type::StringValue>,
  /// A space-delimited list of additional features used in this chart but not captured in the version
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :featureList
  #[sdk(attr(qname = ":featureList"))]
  pub feature_list: Option<crate::simple_type::StringValue>,
  /// A reference (in the form of a relationship ID) to a fallback image to be used if the chart cannot be loaded
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fallbackImg
  #[sdk(attr(qname = ":fallbackImg"))]
  pub fallback_img: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:date1904"))]
  pub date1904: Option<Date1904>,
  /// _
  #[sdk(child(qname = "c:CT_TextLanguageID/c:lang"))]
  pub editing_language: Option<EditingLanguage>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:roundedCorners"))]
  pub rounded_corners: Option<RoundedCorners>,
  #[cfg(not(feature = "mce"))]
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[sdk(choice(qname = "c:CT_Style/c:style"))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c14:CT_Style/c14:style"))
  )]
  pub chart_space_choice: Option<ChartSpaceChoice>,
  /// _
  #[sdk(child(qname = "a:CT_ColorMapping/c:clrMapOvr"))]
  pub c_clr_map_ovr: Option<std::boxed::Box<ColorMapOverride>>,
  /// _
  #[sdk(child(qname = "c:CT_PivotSource/c:pivotSource"))]
  pub c_pivot_source: Option<std::boxed::Box<PivotSource>>,
  /// _
  #[sdk(child(qname = "c:CT_Protection/c:protection"))]
  pub c_protection: Option<std::boxed::Box<Protection>>,
  /// _
  #[sdk(child(qname = "c:CT_Chart/c:chart"))]
  pub c_chart: std::boxed::Box<Chart>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/c:spPr"))]
  pub c_sp_pr: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub c_tx_pr: Option<std::boxed::Box<TextProperties>>,
  /// _
  #[sdk(child(qname = "c:CT_ExternalData/c:externalData"))]
  pub c_external_data: Option<std::boxed::Box<ExternalData>>,
  /// _
  #[sdk(child(qname = "c:CT_PrintSettings/c:printSettings"))]
  pub c_print_settings: Option<std::boxed::Box<PrintSettings>>,
  /// _
  #[sdk(child(qname = "c:CT_RelId/c:userShapes"))]
  pub c_user_shapes: Option<UserShapesReference>,
  /// _
  #[sdk(child(qname = "c:CT_ChartSpaceExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ChartSpaceExtensionList>,
}
/// User Shapes.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:userShapes.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cdr:CT_Drawing/c:userShapes")]
pub struct UserShapes {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  #[sdk(choice(
    qname = "cdr:CT_RelSizeAnchor/cdr:relSizeAnchor",
    qname = "cdr:CT_AbsSizeAnchor/cdr:absSizeAnchor"
  ))]
  pub user_shapes_choice: Vec<UserShapesChoice>,
}
/// Reference to Chart Part.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:chart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RelId/c:chart")]
pub struct ChartReference {
  /// Relationship Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Legacy Drawing for Headers and Footers.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:legacyDrawingHF.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RelId/c:legacyDrawingHF")]
pub struct LegacyDrawingHeaderFooter {
  /// Relationship Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the UserShapesReference Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:userShapes.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RelId/c:userShapes")]
pub struct UserShapesReference {
  /// Relationship Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the RelationshipIdType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RelId/")]
pub struct RelationshipIdType {
  /// Relationship Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Extension.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Extension/c:ext")]
pub struct Extension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Uniform Resource Identifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uri: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Numeric Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:v.
pub type NumericValue = crate::simple_type::StringValue;
/// Format Code.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:formatCode.
pub type FormatCode = crate::simple_type::StringValue;
/// Odd Header.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:oddHeader.
pub type OddHeader = crate::simple_type::StringValue;
/// Odd Footer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:oddFooter.
pub type OddFooter = crate::simple_type::StringValue;
/// Even Header.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:evenHeader.
pub type EvenHeader = crate::simple_type::StringValue;
/// Even Footer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:evenFooter.
pub type EvenFooter = crate::simple_type::StringValue;
/// First Header.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:firstHeader.
pub type FirstHeader = crate::simple_type::StringValue;
/// First Footer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:firstFooter.
pub type FirstFooter = crate::simple_type::StringValue;
/// Pivot Name.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:name.
pub type PivotTableName = crate::simple_type::StringValue;
/// Numeric Point.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:pt.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumVal/c:pt")]
pub struct NumericPoint {
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  /// Number Format
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :formatCode
  #[sdk(attr(qname = ":formatCode"))]
  pub format_code: Option<crate::simple_type::StringValue>,
  /// Numeric Value
  #[sdk(text_child(qname = "c:ST_Xstring/c:v"))]
  pub numeric_value: crate::simple_type::StringValue,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ExtensionList/c:extLst")]
pub struct ExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_Extension/c:ext"))]
  pub c_ext: Vec<Extension>,
}
/// Number Reference.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:numRef.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumRef/c:numRef")]
pub struct NumberReference {
  /// _
  #[sdk(text_child(qname = "xsd:string/c:f"))]
  pub formula: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "c:CT_NumData/c:numCache"))]
  pub numbering_cache: Option<std::boxed::Box<NumberingCache>>,
  /// _
  #[sdk(child(qname = "c:CT_NumRefExtensionList/c:extLst"))]
  pub num_ref_extension_list: Option<NumRefExtensionList>,
}
/// Number Literal.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:numLit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumData/c:numLit")]
pub struct NumberLiteral {
  /// Format Code
  #[sdk(text_child(qname = "c:ST_Xstring/c:formatCode"))]
  pub format_code: Option<crate::simple_type::StringValue>,
  /// Point Count
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  pub point_count: Option<PointCount>,
  /// _
  #[sdk(child(qname = "c:CT_NumVal/c:pt"))]
  pub c_pt: Vec<NumericPoint>,
  /// _
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// Defines the NumberingCache Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:numCache.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumData/c:numCache")]
pub struct NumberingCache {
  /// Format Code
  #[sdk(text_child(qname = "c:ST_Xstring/c:formatCode"))]
  pub format_code: Option<crate::simple_type::StringValue>,
  /// Point Count
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  pub point_count: Option<PointCount>,
  /// _
  #[sdk(child(qname = "c:CT_NumVal/c:pt"))]
  pub c_pt: Vec<NumericPoint>,
  /// _
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// Defines the NumberDataType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumData/")]
pub struct NumberDataType {
  #[sdk(choice(
    qname = "c:ST_Xstring/c:formatCode",
    qname = "c:CT_UnsignedInt/c:ptCount",
    qname = "c:CT_NumVal/c:pt",
    qname = "c:CT_ExtensionList/c:extLst"
  ))]
  pub xml_children: Vec<NumberDataTypeChoice>,
}
/// Level.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:lvl.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Lvl/c:lvl")]
pub struct Level {
  /// _
  #[sdk(child(qname = "c:CT_StrVal/c:pt"))]
  pub c_pt: Vec<StringPoint>,
}
/// Multi Level String Reference.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:multiLvlStrRef.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_MultiLvlStrRef/c:multiLvlStrRef")]
pub struct MultiLevelStringReference {
  /// _
  #[sdk(text_child(qname = "xsd:string/c:f"))]
  pub formula: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "c:CT_MultiLvlStrData/c:multiLvlStrCache"))]
  pub multi_level_string_cache: Option<std::boxed::Box<MultiLevelStringCache>>,
  /// _
  #[sdk(child(qname = "c:CT_MultiLvlStrRefExtensionList/c:extLst"))]
  pub multi_lvl_str_ref_extension_list: Option<MultiLvlStrRefExtensionList>,
}
/// Defines the StringReference Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:strRef.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StrRef/c:strRef")]
pub struct StringReference {
  /// _
  #[sdk(text_child(qname = "xsd:string/c:f"))]
  pub formula: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "c:CT_StrData/c:strCache"))]
  pub string_cache: Option<std::boxed::Box<StringCache>>,
  /// _
  #[sdk(child(qname = "c:CT_StrRefExtensionList/c:extLst"))]
  pub str_ref_extension_list: Option<StrRefExtensionList>,
}
/// String Literal.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:strLit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StrData/c:strLit")]
pub struct StringLiteral {
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  pub point_count: Option<PointCount>,
  /// _
  #[sdk(child(qname = "c:CT_StrVal/c:pt"))]
  pub c_pt: Vec<StringPoint>,
  /// _
  #[sdk(child(qname = "c:CT_StrDataExtensionList/c:extLst"))]
  pub c_ext_lst: Option<StrDataExtensionList>,
}
/// Defines the StringCache Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:strCache.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StrData/c:strCache")]
pub struct StringCache {
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  pub point_count: Option<PointCount>,
  /// _
  #[sdk(child(qname = "c:CT_StrVal/c:pt"))]
  pub c_pt: Vec<StringPoint>,
  /// _
  #[sdk(child(qname = "c:CT_StrDataExtensionList/c:extLst"))]
  pub c_ext_lst: Option<StrDataExtensionList>,
}
/// Defines the StringDataType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StrData/")]
pub struct StringDataType {
  #[sdk(choice(
    qname = "c:CT_UnsignedInt/c:ptCount",
    qname = "c:CT_StrVal/c:pt",
    qname = "c:CT_StrDataExtensionList/c:extLst"
  ))]
  pub xml_children: Vec<StringDataTypeChoice>,
}
/// Layout Target.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:layoutTarget.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LayoutTarget/c:layoutTarget")]
pub struct LayoutTarget {
  /// Layout Target Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<LayoutTargetValues>,
}
/// Left Mode.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:xMode.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LayoutMode/c:xMode")]
pub struct LeftMode {
  /// Layout Mode Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<LayoutModeValues>,
}
/// Top Mode.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:yMode.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LayoutMode/c:yMode")]
pub struct TopMode {
  /// Layout Mode Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<LayoutModeValues>,
}
/// Width Mode.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:wMode.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LayoutMode/c:wMode")]
pub struct WidthMode {
  /// Layout Mode Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<LayoutModeValues>,
}
/// Height Mode.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:hMode.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LayoutMode/c:hMode")]
pub struct HeightMode {
  /// Layout Mode Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<LayoutModeValues>,
}
/// Defines the LayoutModeType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LayoutMode/")]
pub struct LayoutModeType {
  /// Layout Mode Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<LayoutModeValues>,
}
/// Manual Layout.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:manualLayout.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ManualLayout/c:manualLayout")]
pub struct ManualLayout {
  /// Layout Target
  #[sdk(child(qname = "c:CT_LayoutTarget/c:layoutTarget"))]
  pub layout_target: Option<LayoutTarget>,
  /// Left Mode
  #[sdk(child(qname = "c:CT_LayoutMode/c:xMode"))]
  pub left_mode: Option<LeftMode>,
  /// Top Mode
  #[sdk(child(qname = "c:CT_LayoutMode/c:yMode"))]
  pub top_mode: Option<TopMode>,
  /// Width Mode
  #[sdk(child(qname = "c:CT_LayoutMode/c:wMode"))]
  pub width_mode: Option<WidthMode>,
  /// Height Mode
  #[sdk(child(qname = "c:CT_LayoutMode/c:hMode"))]
  pub height_mode: Option<HeightMode>,
  /// Left
  #[sdk(child(qname = "c:CT_Double/c:x"))]
  pub left: Option<Left>,
  /// Top
  #[sdk(child(qname = "c:CT_Double/c:y"))]
  pub top: Option<Top>,
  /// Width
  #[sdk(child(qname = "c:CT_Double/c:w"))]
  pub width: Option<Width>,
  /// Height
  #[sdk(child(qname = "c:CT_Double/c:h"))]
  pub height: Option<Height>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// X Rotation.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:rotX.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RotX/c:rotX")]
pub struct RotateX {
  /// X Rotation Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-90",
    max = "90",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: Option<crate::simple_type::SByteValue>,
}
/// Height Percent.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:hPercent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_HPercent/c:hPercent")]
pub struct HeightPercent {
  /// Height Percent Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 0u32,
    min = "5",
    max = "500",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: Option<crate::simple_type::UInt16Value>,
}
/// Y Rotation.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:rotY.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RotY/c:rotY")]
pub struct RotateY {
  /// Y Rotation Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "360",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: Option<crate::simple_type::UInt16Value>,
}
/// Depth Percent.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:depthPercent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DepthPercent/c:depthPercent")]
pub struct DepthPercent {
  /// Depth Percent Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 0u32,
    min = "20",
    max = "2000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: Option<crate::simple_type::UInt16Value>,
}
/// Perspective.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:perspective.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Perspective/c:perspective")]
pub struct Perspective {
  /// Perspective Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "240",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: Option<crate::simple_type::ByteValue>,
}
/// Symbol.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:symbol.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_MarkerStyle/c:symbol")]
pub struct Symbol {
  /// Marker Style Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: MarkerStyleValues,
}
/// Size.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_MarkerSize/c:size")]
pub struct Size {
  /// Marker Size Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 0u32,
    min = "2",
    max = "72",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: Option<crate::simple_type::ByteValue>,
}
/// Marker.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:marker.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Marker/c:marker")]
pub struct Marker {
  /// Symbol
  #[sdk(child(qname = "c:CT_MarkerStyle/c:symbol"))]
  pub symbol: Option<Symbol>,
  /// Size
  #[sdk(child(qname = "c:CT_MarkerSize/c:size"))]
  pub size: Option<Size>,
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PictureOptions Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:pictureOptions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PictureOptions/c:pictureOptions")]
pub struct PictureOptions {
  /// Apply To Front
  #[sdk(child(qname = "c:CT_Boolean/c:applyToFront"))]
  pub apply_to_front: Option<ApplyToFront>,
  /// Apply To Sides
  #[sdk(child(qname = "c:CT_Boolean/c:applyToSides"))]
  pub apply_to_sides: Option<ApplyToSides>,
  /// Apply to End
  #[sdk(child(qname = "c:CT_Boolean/c:applyToEnd"))]
  pub apply_to_end: Option<ApplyToEnd>,
  /// Picture Format
  #[sdk(child(qname = "c:CT_PictureFormat/c:pictureFormat"))]
  pub picture_format: Option<PictureFormat>,
  /// Picture Stack Unit
  #[sdk(child(qname = "c:CT_PictureStackUnit/c:pictureStackUnit"))]
  pub picture_stack_unit: Option<PictureStackUnit>,
}
/// Trendline Type.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:trendlineType.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TrendlineType/c:trendlineType")]
pub struct TrendlineType {
  /// Trendline Type Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<TrendlineValues>,
}
/// Polynomial Trendline Order.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:order.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Order/c:order")]
pub struct PolynomialOrder {
  /// Order Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "2",
    max = "6",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::ByteValue,
}
/// Period.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:period.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Period/c:period")]
pub struct Period {
  /// Period Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(source = 1u32, min = "2", min_inclusive = true, max_inclusive = false))]
  pub val: crate::simple_type::UInt32Value,
}
/// Trendline Label.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:trendlineLbl.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TrendlineLbl/c:trendlineLbl")]
pub struct TrendlineLabel {
  /// Layout
  #[sdk(child(qname = "c:CT_Layout/c:layout"))]
  pub layout: Option<std::boxed::Box<Layout>>,
  /// _
  #[sdk(child(qname = "c:CT_Tx/c:tx"))]
  pub chart_text: Option<std::boxed::Box<ChartText>>,
  /// Number Format
  #[sdk(child(qname = "c:CT_NumFmt/c:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Error Bar Direction.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:errDir.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ErrDir/c:errDir")]
pub struct ErrorDirection {
  /// Error Bar Direction Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: ErrorBarDirectionValues,
}
/// Error Bar Type.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:errBarType.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ErrBarType/c:errBarType")]
pub struct ErrorBarType {
  /// Error Bar Type Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: ErrorBarValues,
}
/// Error Bar Value Type.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:errValType.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ErrValType/c:errValType")]
pub struct ErrorBarValueType {
  /// Error Bar Type Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: ErrorValues,
}
/// Plus.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:plus.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumDataSource/c:plus")]
pub struct Plus {
  #[sdk(choice(qname = "c:CT_NumRef/c:numRef", qname = "c:CT_NumData/c:numLit"))]
  pub xml_children: Option<PlusChoice>,
}
/// Minus.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:minus.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumDataSource/c:minus")]
pub struct Minus {
  #[sdk(choice(qname = "c:CT_NumRef/c:numRef", qname = "c:CT_NumData/c:numLit"))]
  pub xml_children: Option<MinusChoice>,
}
/// Defines the Values Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:val.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumDataSource/c:val")]
pub struct Values {
  #[sdk(choice(qname = "c:CT_NumRef/c:numRef", qname = "c:CT_NumData/c:numLit"))]
  pub xml_children: Option<ValuesChoice>,
}
/// Defines the YValues Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:yVal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumDataSource/c:yVal")]
pub struct YValues {
  #[sdk(choice(qname = "c:CT_NumRef/c:numRef", qname = "c:CT_NumData/c:numLit"))]
  pub xml_children: Option<YValuesChoice>,
}
/// Defines the BubbleSize Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:bubbleSize.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumDataSource/c:bubbleSize")]
pub struct BubbleSize {
  #[sdk(choice(qname = "c:CT_NumRef/c:numRef", qname = "c:CT_NumData/c:numLit"))]
  pub xml_children: Option<BubbleSizeChoice>,
}
/// Defines the NumberDataSourceType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumDataSource/")]
pub struct NumberDataSourceType {
  #[sdk(choice(qname = "c:CT_NumRef/c:numRef", qname = "c:CT_NumData/c:numLit"))]
  pub xml_children: Option<NumberDataSourceTypeChoice>,
}
/// Gap Width.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:gapWidth.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_GapAmount/c:gapWidth")]
pub struct GapWidth {
  /// Gap Size Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "500",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: Option<crate::simple_type::UInt16Value>,
}
/// Defines the GapDepth Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:gapDepth.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_GapAmount/c:gapDepth")]
pub struct GapDepth {
  /// Gap Size Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "500",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: Option<crate::simple_type::UInt16Value>,
}
/// Defines the GapAmountType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_GapAmount/")]
pub struct GapAmountType {
  /// Gap Size Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "500",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: Option<crate::simple_type::UInt16Value>,
}
/// Up Bars.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:upBars.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UpDownBar/c:upBars")]
pub struct UpBars {
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Down Bars.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:downBars.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UpDownBar/c:downBars")]
pub struct DownBars {
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Defines the UpDownBarType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UpDownBar/")]
pub struct UpDownBarType {
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Vec<ChartShapeProperties>,
}
/// Pie of Pie or Bar of Pie Type.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ofPieType.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_OfPieType/c:ofPieType")]
pub struct OfPieType {
  /// Pie of Pie or Bar of Pie Type Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: OfPieValues,
}
/// Split Type.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:splitType.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SplitType/c:splitType")]
pub struct SplitType {
  /// Split Type Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: SplitValues,
}
/// Custom Split.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:custSplit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_CustSplit/c:custSplit")]
pub struct CustomSplit {
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:secondPiePt"))]
  pub c_second_pie_pt: Vec<SecondPiePoint>,
}
/// Second Pie Size.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:secondPieSize.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SecondPieSize/c:secondPieSize")]
pub struct SecondPieSize {
  /// Second Pie Size Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 0u32,
    min = "5",
    max = "200",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: Option<crate::simple_type::UInt16Value>,
}
/// Band Format.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:bandFmt.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BandFmt/c:bandFmt")]
pub struct BandFormat {
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: std::boxed::Box<Index>,
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Picture Format.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:pictureFormat.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PictureFormat/c:pictureFormat")]
pub struct PictureFormat {
  /// Picture Format Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: PictureFormatValues,
}
/// Picture Stack Unit.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:pictureStackUnit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PictureStackUnit/c:pictureStackUnit")]
pub struct PictureStackUnit {
  /// Picture Stack Unit
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(source = 1u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub val: crate::simple_type::DoubleValue,
}
/// Built in Display Unit Value.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:builtInUnit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BuiltInUnit/c:builtInUnit")]
pub struct BuiltInUnit {
  /// Built In Unit Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<BuiltInUnitValues>,
}
/// Display Units Label.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:dispUnitsLbl.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DispUnitsLbl/c:dispUnitsLbl")]
pub struct DisplayUnitsLabel {
  /// Layout
  #[sdk(child(qname = "c:CT_Layout/c:layout"))]
  pub layout: Option<std::boxed::Box<Layout>>,
  /// _
  #[sdk(child(qname = "c:CT_Tx/c:tx"))]
  pub chart_text: Option<std::boxed::Box<ChartText>>,
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
}
/// Logarithmic Base.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:logBase.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LogBase/c:logBase")]
pub struct LogBase {
  /// Logarithmic Base Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "2",
    max = "1000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::DoubleValue,
}
/// Axis Orientation.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:orientation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Orientation/c:orientation")]
pub struct Orientation {
  /// Orientation Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<OrientationValues>,
}
/// Pivot Format.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:pivotFmt.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PivotFmt/c:pivotFmt")]
pub struct PivotFormat {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: std::boxed::Box<Index>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/c:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Marker
  #[sdk(child(qname = "c:CT_Marker/c:marker"))]
  pub marker: Option<std::boxed::Box<Marker>>,
  /// Data Label
  #[sdk(child(qname = "c:CT_DLbl/c:dLbl"))]
  pub data_label: Option<std::boxed::Box<DataLabel>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Legend Position.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:legendPos.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LegendPos/c:legendPos")]
pub struct LegendPosition {
  /// Legend Position Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<LegendPositionValues>,
}
/// Legend Entry.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:legendEntry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LegendEntry/c:legendEntry")]
pub struct LegendEntry {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: std::boxed::Box<Index>,
  #[sdk(choice(qname = "c:CT_Boolean/c:delete", qname = "a:CT_TextBody/c:txPr"))]
  pub legend_entry_choice: Option<LegendEntryChoice>,
  /// _
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// Header and Footer.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:headerFooter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_HeaderFooter/c:headerFooter")]
pub struct HeaderFooter {
  /// Align With Margins
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :alignWithMargins
  #[sdk(attr(qname = ":alignWithMargins"))]
  pub align_with_margins: Option<crate::simple_type::BooleanValue>,
  /// Different Odd Even
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :differentOddEven
  #[sdk(attr(qname = ":differentOddEven"))]
  pub different_odd_even: Option<crate::simple_type::BooleanValue>,
  /// Different First
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :differentFirst
  #[sdk(attr(qname = ":differentFirst"))]
  pub different_first: Option<crate::simple_type::BooleanValue>,
  /// Odd Header
  #[sdk(text_child(qname = "c:ST_Xstring/c:oddHeader"))]
  pub odd_header: Option<crate::simple_type::StringValue>,
  /// Odd Footer
  #[sdk(text_child(qname = "c:ST_Xstring/c:oddFooter"))]
  pub odd_footer: Option<crate::simple_type::StringValue>,
  /// Even Header
  #[sdk(text_child(qname = "c:ST_Xstring/c:evenHeader"))]
  pub even_header: Option<crate::simple_type::StringValue>,
  /// Even Footer
  #[sdk(text_child(qname = "c:ST_Xstring/c:evenFooter"))]
  pub even_footer: Option<crate::simple_type::StringValue>,
  /// First Header
  #[sdk(text_child(qname = "c:ST_Xstring/c:firstHeader"))]
  pub first_header: Option<crate::simple_type::StringValue>,
  /// First Footer
  #[sdk(text_child(qname = "c:ST_Xstring/c:firstFooter"))]
  pub first_footer: Option<crate::simple_type::StringValue>,
}
/// Page Margins.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:pageMargins.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PageMargins/c:pageMargins")]
pub struct PageMargins {
  /// Left
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :l
  #[sdk(attr(qname = ":l"))]
  pub left: crate::simple_type::DoubleValue,
  /// Right
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub right: crate::simple_type::DoubleValue,
  /// Top
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub top: crate::simple_type::DoubleValue,
  /// Bottom
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :b
  #[sdk(attr(qname = ":b"))]
  pub bottom: crate::simple_type::DoubleValue,
  /// Header
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :header
  #[sdk(attr(qname = ":header"))]
  pub header: crate::simple_type::DoubleValue,
  /// Footer
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :footer
  #[sdk(attr(qname = ":footer"))]
  pub footer: crate::simple_type::DoubleValue,
}
/// Page Setup.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:pageSetup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PageSetup/c:pageSetup")]
pub struct PageSetup {
  /// Page Size
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :paperSize
  #[sdk(attr(qname = ":paperSize"))]
  pub paper_size: Option<crate::simple_type::UInt32Value>,
  /// First Page Number
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :firstPageNumber
  #[sdk(attr(qname = ":firstPageNumber"))]
  pub first_page_number: Option<crate::simple_type::Int32Value>,
  /// Orientation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :orientation
  #[sdk(attr(qname = ":orientation"))]
  pub orientation: Option<PageSetupOrientationValues>,
  /// Black and White
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :blackAndWhite
  #[sdk(attr(qname = ":blackAndWhite"))]
  pub black_and_white: Option<crate::simple_type::BooleanValue>,
  /// Draft
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :draft
  #[sdk(attr(qname = ":draft"))]
  pub draft: Option<crate::simple_type::BooleanValue>,
  /// Use First Page Number
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :useFirstPageNumber
  #[sdk(attr(qname = ":useFirstPageNumber"))]
  pub use_first_page_number: Option<crate::simple_type::BooleanValue>,
  /// Horizontal DPI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :horizontalDpi
  #[sdk(attr(qname = ":horizontalDpi"))]
  pub horizontal_dpi: Option<crate::simple_type::Int32Value>,
  /// Vertical DPI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :verticalDpi
  #[sdk(attr(qname = ":verticalDpi"))]
  pub vertical_dpi: Option<crate::simple_type::Int32Value>,
  /// Copies
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :copies
  #[sdk(attr(qname = ":copies"))]
  pub copies: Option<crate::simple_type::UInt32Value>,
}
/// Defines the ShapeProperties Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:spPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeProperties/c:spPr")]
pub struct ShapeProperties {
  /// Black and White Mode
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bwMode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub black_white_mode:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues>,
  /// 2D Transform for Individual Objects
  #[sdk(child(qname = "a:CT_Transform2D/a:xfrm"))]
  pub transform2_d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Transform2D>,
  >,
  #[sdk(choice(
    qname = "a:CT_CustomGeometry2D/a:custGeom",
    qname = "a:CT_PresetGeometry2D/a:prstGeom"
  ))]
  pub shape_properties_choice1: Option<ShapePropertiesChoice>,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub shape_properties_choice2: Option<ShapePropertiesChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub a_ln: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline>,
  >,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// _
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  pub a_sp3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_ShapePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapePropertiesExtensionList,
  >,
}
/// Data Label.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:dLbl.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DLbl/c:dLbl")]
pub struct DataLabel {
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: std::boxed::Box<Index>,
  #[sdk(choice(
    qname = "c:CT_Boolean/c:delete",
    qname = "c:CT_Layout/c:layout",
    qname = "c:CT_Tx/c:tx",
    qname = "c:CT_NumFmt/c:numFmt",
    qname = "a:CT_ChartShapeProperties/c:spPr",
    qname = "a:CT_TextBody/c:txPr",
    qname = "c:CT_DLblPos/c:dLblPos",
    qname = "c:CT_Boolean/c:showLegendKey",
    qname = "c:CT_Boolean/c:showVal",
    qname = "c:CT_Boolean/c:showCatName",
    qname = "c:CT_Boolean/c:showSerName",
    qname = "c:CT_Boolean/c:showPercent",
    qname = "c:CT_Boolean/c:showBubbleSize",
    qname = "xsd:string/c:separator"
  ))]
  pub data_label_choice: Option<DataLabelChoice>,
  /// _
  #[sdk(child(qname = "c:CT_DLblExtensionList/c:extLst"))]
  pub c_ext_lst: Option<DLblExtensionList>,
}
/// Area Charts.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:areaChart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AreaChart/c:areaChart")]
pub struct AreaChart {
  /// Grouping
  #[sdk(child(qname = "c:CT_Grouping/c:grouping"))]
  pub grouping: Option<Grouping>,
  /// Defines the VaryColors Class.
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// Area Chart Series.
  #[sdk(child(qname = "c:CT_AreaSer/c:ser"))]
  pub area_chart_series: Vec<AreaChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub data_labels: Option<std::boxed::Box<DataLabels>>,
  /// Drop Lines.
  #[sdk(child(qname = "c:CT_ChartLines/c:dropLines"))]
  pub drop_lines: Option<std::boxed::Box<DropLines>>,
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// _
  #[sdk(child(qname = "c:CT_AreaChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<AreaChartExtensionList>,
}
/// 3D Area Charts.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:area3DChart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Area3DChart/c:area3DChart")]
pub struct Area3DChart {
  /// Grouping
  #[sdk(child(qname = "c:CT_Grouping/c:grouping"))]
  pub grouping: Option<Grouping>,
  /// Defines the VaryColors Class.
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// Area Chart Series.
  #[sdk(child(qname = "c:CT_AreaSer/c:ser"))]
  pub area_chart_series: Vec<AreaChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub data_labels: Option<std::boxed::Box<DataLabels>>,
  /// Drop Lines.
  #[sdk(child(qname = "c:CT_ChartLines/c:dropLines"))]
  pub drop_lines: Option<std::boxed::Box<DropLines>>,
  /// _
  #[sdk(child(qname = "c:CT_GapAmount/c:gapDepth"))]
  pub c_gap_depth: Option<GapDepth>,
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// _
  #[sdk(child(qname = "c:CT_Area3DChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<Area3DChartExtensionList>,
}
/// Line Charts.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:lineChart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LineChart/c:lineChart")]
pub struct LineChart {
  /// Grouping
  #[sdk(child(qname = "c:CT_Grouping/c:grouping"))]
  pub grouping: std::boxed::Box<Grouping>,
  /// Defines the VaryColors Class.
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// Defines the LineChartSeries Class.
  #[sdk(child(qname = "c:CT_LineSer/c:ser"))]
  pub line_chart_series: Vec<LineChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub data_labels: Option<std::boxed::Box<DataLabels>>,
  /// Drop Lines.
  #[sdk(child(qname = "c:CT_ChartLines/c:dropLines"))]
  pub drop_lines: Option<std::boxed::Box<DropLines>>,
  /// _
  #[sdk(child(qname = "c:CT_ChartLines/c:hiLowLines"))]
  pub c_hi_low_lines: Option<std::boxed::Box<HighLowLines>>,
  /// _
  #[sdk(child(qname = "c:CT_UpDownBars/c:upDownBars"))]
  pub c_up_down_bars: Option<std::boxed::Box<UpDownBars>>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:marker"))]
  pub c_marker: Option<ShowMarker>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:smooth"))]
  pub c_smooth: Option<Smooth>,
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// _
  #[sdk(child(qname = "c:CT_LineChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<LineChartExtensionList>,
}
/// 3D Line Charts.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:line3DChart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Line3DChart/c:line3DChart")]
pub struct Line3DChart {
  /// Grouping
  #[sdk(child(qname = "c:CT_Grouping/c:grouping"))]
  pub grouping: std::boxed::Box<Grouping>,
  /// Defines the VaryColors Class.
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// Defines the LineChartSeries Class.
  #[sdk(child(qname = "c:CT_LineSer/c:ser"))]
  pub line_chart_series: Vec<LineChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub data_labels: Option<std::boxed::Box<DataLabels>>,
  /// Drop Lines.
  #[sdk(child(qname = "c:CT_ChartLines/c:dropLines"))]
  pub drop_lines: Option<std::boxed::Box<DropLines>>,
  /// _
  #[sdk(child(qname = "c:CT_GapAmount/c:gapDepth"))]
  pub c_gap_depth: Option<GapDepth>,
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// _
  #[sdk(child(qname = "c:CT_Line3DChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<Line3DChartExtensionList>,
}
/// Stock Charts.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:stockChart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StockChart/c:stockChart")]
pub struct StockChart {
  /// _
  #[sdk(child(qname = "c:CT_LineSer/c:ser"))]
  pub c_ser: Vec<LineChartSeries>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "c:CT_ChartLines/c:dropLines"))]
  pub c_drop_lines: Option<std::boxed::Box<DropLines>>,
  /// _
  #[sdk(child(qname = "c:CT_ChartLines/c:hiLowLines"))]
  pub c_hi_low_lines: Option<std::boxed::Box<HighLowLines>>,
  /// _
  #[sdk(child(qname = "c:CT_UpDownBars/c:upDownBars"))]
  pub c_up_down_bars: Option<std::boxed::Box<UpDownBars>>,
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// _
  #[sdk(child(qname = "c:CT_StockChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<StockChartExtensionList>,
}
/// Radar Charts.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:radarChart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RadarChart/c:radarChart")]
pub struct RadarChart {
  /// _
  #[sdk(child(qname = "c:CT_RadarStyle/c:radarStyle"))]
  pub radar_style: std::boxed::Box<RadarStyle>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// _
  #[sdk(child(qname = "c:CT_RadarSer/c:ser"))]
  pub c_ser: Vec<RadarChartSeries>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// _
  #[sdk(child(qname = "c:CT_RadarChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<RadarChartExtensionList>,
}
/// Scatter Charts.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:scatterChart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ScatterChart/c:scatterChart")]
pub struct ScatterChart {
  /// _
  #[sdk(child(qname = "c:CT_ScatterStyle/c:scatterStyle"))]
  pub scatter_style: std::boxed::Box<ScatterStyle>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// _
  #[sdk(child(qname = "c:CT_ScatterSer/c:ser"))]
  pub c_ser: Vec<ScatterChartSeries>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// _
  #[sdk(child(qname = "c:CT_ScatterChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ScatterChartExtensionList>,
}
/// Pie Charts.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:pieChart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PieChart/c:pieChart")]
pub struct PieChart {
  /// Defines the VaryColors Class.
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// Pie Chart Series.
  #[sdk(child(qname = "c:CT_PieSer/c:ser"))]
  pub pie_chart_series: Vec<PieChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub data_labels: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "c:CT_FirstSliceAng/c:firstSliceAng"))]
  pub c_first_slice_ang: Option<FirstSliceAngle>,
  /// _
  #[sdk(child(qname = "c:CT_PieChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<PieChartExtensionList>,
}
/// 3D Pie Charts.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:pie3DChart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Pie3DChart/c:pie3DChart")]
pub struct Pie3DChart {
  /// Defines the VaryColors Class.
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// Pie Chart Series.
  #[sdk(child(qname = "c:CT_PieSer/c:ser"))]
  pub pie_chart_series: Vec<PieChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub data_labels: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "c:CT_Pie3DChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<Pie3DChartExtensionList>,
}
/// Doughnut Charts.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:doughnutChart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DoughnutChart/c:doughnutChart")]
pub struct DoughnutChart {
  /// Defines the VaryColors Class.
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// Pie Chart Series.
  #[sdk(child(qname = "c:CT_PieSer/c:ser"))]
  pub pie_chart_series: Vec<PieChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub data_labels: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "c:CT_FirstSliceAng/c:firstSliceAng"))]
  pub c_first_slice_ang: Option<FirstSliceAngle>,
  /// _
  #[sdk(child(qname = "c:CT_HoleSize/c:holeSize"))]
  pub c_hole_size: std::boxed::Box<HoleSize>,
  /// _
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// Bar Charts.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:barChart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BarChart/c:barChart")]
pub struct BarChart {
  /// Bar Direction
  #[sdk(child(qname = "c:CT_BarDir/c:barDir"))]
  pub bar_direction: std::boxed::Box<BarDirection>,
  /// Bar Grouping
  #[sdk(child(qname = "c:CT_BarGrouping/c:grouping"))]
  pub bar_grouping: Option<BarGrouping>,
  /// Defines the VaryColors Class.
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// Bar Chart Series.
  #[sdk(child(qname = "c:CT_BarSer/c:ser"))]
  pub bar_chart_series: Vec<BarChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub data_labels: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "c:CT_GapAmount/c:gapWidth"))]
  pub c_gap_width: Option<GapWidth>,
  /// _
  #[sdk(child(qname = "c:CT_Overlap/c:overlap"))]
  pub c_overlap: Option<Overlap>,
  /// _
  #[sdk(child(qname = "c:CT_ChartLines/c:serLines"))]
  pub c_ser_lines: Vec<SeriesLines>,
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// _
  #[sdk(child(qname = "c:CT_BarChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<BarChartExtensionList>,
}
/// 3D Bar Charts.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:bar3DChart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Bar3DChart/c:bar3DChart")]
pub struct Bar3DChart {
  /// Bar Direction
  #[sdk(child(qname = "c:CT_BarDir/c:barDir"))]
  pub bar_direction: std::boxed::Box<BarDirection>,
  /// Bar Grouping
  #[sdk(child(qname = "c:CT_BarGrouping/c:grouping"))]
  pub bar_grouping: Option<BarGrouping>,
  /// Defines the VaryColors Class.
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// Bar Chart Series.
  #[sdk(child(qname = "c:CT_BarSer/c:ser"))]
  pub bar_chart_series: Vec<BarChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub data_labels: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "c:CT_GapAmount/c:gapWidth"))]
  pub c_gap_width: Option<GapWidth>,
  /// _
  #[sdk(child(qname = "c:CT_GapAmount/c:gapDepth"))]
  pub c_gap_depth: Option<GapDepth>,
  /// _
  #[sdk(child(qname = "c:CT_Shape/c:shape"))]
  pub c_shape: Option<Shape>,
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// _
  #[sdk(child(qname = "c:CT_Bar3DChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<Bar3DChartExtensionList>,
}
/// Pie of Pie or Bar of Pie Charts.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ofPieChart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_OfPieChart/c:ofPieChart")]
pub struct OfPieChart {
  /// Pie of Pie or Bar of Pie Type
  #[sdk(child(qname = "c:CT_OfPieType/c:ofPieType"))]
  pub of_pie_type: std::boxed::Box<OfPieType>,
  /// Defines the VaryColors Class.
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// Pie Chart Series.
  #[sdk(child(qname = "c:CT_PieSer/c:ser"))]
  pub pie_chart_series: Vec<PieChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub data_labels: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "c:CT_GapAmount/c:gapWidth"))]
  pub c_gap_width: Option<GapWidth>,
  /// _
  #[sdk(child(qname = "c:CT_SplitType/c:splitType"))]
  pub c_split_type: Option<SplitType>,
  /// _
  #[sdk(child(qname = "c:CT_Double/c:splitPos"))]
  pub c_split_pos: Option<SplitPosition>,
  /// _
  #[sdk(child(qname = "c:CT_CustSplit/c:custSplit"))]
  pub c_cust_split: Option<CustomSplit>,
  /// _
  #[sdk(child(qname = "c:CT_SecondPieSize/c:secondPieSize"))]
  pub c_second_pie_size: Option<SecondPieSize>,
  /// _
  #[sdk(child(qname = "c:CT_ChartLines/c:serLines"))]
  pub c_ser_lines: Vec<SeriesLines>,
  /// _
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// Surface Charts.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:surfaceChart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SurfaceChart/c:surfaceChart")]
pub struct SurfaceChart {
  /// Wireframe
  #[sdk(child(qname = "c:CT_Boolean/c:wireframe"))]
  pub wireframe: Option<Wireframe>,
  /// Surface Chart Series.
  #[sdk(child(qname = "c:CT_SurfaceSer/c:ser"))]
  pub surface_chart_series: Vec<SurfaceChartSeries>,
  /// Band Formats.
  #[sdk(child(qname = "c:CT_BandFmts/c:bandFmts"))]
  pub band_formats: Option<BandFormats>,
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// _
  #[sdk(child(qname = "c:CT_SurfaceChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<SurfaceChartExtensionList>,
}
/// 3D Surface Charts.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:surface3DChart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Surface3DChart/c:surface3DChart")]
pub struct Surface3DChart {
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:wireframe"))]
  pub wireframe: Option<Wireframe>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// _
  #[sdk(child(qname = "c:CT_SurfaceSer/c:ser"))]
  pub c_ser: Vec<SurfaceChartSeries>,
  /// _
  #[sdk(child(qname = "c:CT_BandFmts/c:bandFmts"))]
  pub c_band_fmts: Option<BandFormats>,
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// _
  #[sdk(child(qname = "c:CT_Surface3DChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<Surface3DChartExtensionList>,
}
/// Bubble Charts.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:bubbleChart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BubbleChart/c:bubbleChart")]
pub struct BubbleChart {
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// _
  #[sdk(child(qname = "c:CT_BubbleSer/c:ser"))]
  pub c_ser: Vec<BubbleChartSeries>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:bubble3D"))]
  pub c_bubble3_d: Option<Bubble3D>,
  /// _
  #[sdk(child(qname = "c:CT_BubbleScale/c:bubbleScale"))]
  pub c_bubble_scale: Option<BubbleScale>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:showNegBubbles"))]
  pub c_show_neg_bubbles: Option<ShowNegativeBubbles>,
  /// _
  #[sdk(child(qname = "c:CT_SizeRepresents/c:sizeRepresents"))]
  pub c_size_represents: Option<SizeRepresents>,
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// _
  #[sdk(child(qname = "c:CT_BubbleChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<BubbleChartExtensionList>,
}
/// Value Axis.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:valAx.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ValAx/c:valAx")]
pub struct ValueAxis {
  /// Axis ID
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub axis_id: std::boxed::Box<AxisId>,
  /// Scaling
  #[sdk(child(qname = "c:CT_Scaling/c:scaling"))]
  pub scaling: std::boxed::Box<Scaling>,
  /// Delete
  #[sdk(child(qname = "c:CT_Boolean/c:delete"))]
  pub delete: Option<Delete>,
  /// Axis Position
  #[sdk(child(qname = "c:CT_AxPos/c:axPos"))]
  pub axis_position: std::boxed::Box<AxisPosition>,
  /// Major Gridlines
  #[sdk(child(qname = "c:CT_ChartLines/c:majorGridlines"))]
  pub major_gridlines: Option<std::boxed::Box<MajorGridlines>>,
  /// Minor Gridlines
  #[sdk(child(qname = "c:CT_ChartLines/c:minorGridlines"))]
  pub minor_gridlines: Option<std::boxed::Box<MinorGridlines>>,
  /// Title
  #[sdk(child(qname = "c:CT_Title/c:title"))]
  pub title: Option<std::boxed::Box<Title>>,
  /// Number Format
  #[sdk(child(qname = "c:CT_NumFmt/c:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Major Tick Mark
  #[sdk(child(qname = "c:CT_TickMark/c:majorTickMark"))]
  pub major_tick_mark: Option<MajorTickMark>,
  /// Minor Tick Mark
  #[sdk(child(qname = "c:CT_TickMark/c:minorTickMark"))]
  pub minor_tick_mark: Option<MinorTickMark>,
  /// Tick Label Position
  #[sdk(child(qname = "c:CT_TickLblPos/c:tickLblPos"))]
  pub tick_label_position: Option<TickLabelPosition>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// Defines the TextProperties Class.
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
  /// Crossing Axis ID
  #[sdk(child(qname = "c:CT_UnsignedInt/c:crossAx"))]
  pub crossing_axis: std::boxed::Box<CrossingAxis>,
  #[sdk(choice(qname = "c:CT_Crosses/c:crosses", qname = "c:CT_Double/c:crossesAt"))]
  pub value_axis_choice: Option<ValueAxisChoice>,
  /// _
  #[sdk(child(qname = "c:CT_CrossBetween/c:crossBetween"))]
  pub c_cross_between: Option<CrossBetween>,
  /// _
  #[sdk(child(qname = "c:CT_AxisUnit/c:majorUnit"))]
  pub c_major_unit: Option<MajorUnit>,
  /// _
  #[sdk(child(qname = "c:CT_AxisUnit/c:minorUnit"))]
  pub c_minor_unit: Option<MinorUnit>,
  /// _
  #[sdk(child(qname = "c:CT_DispUnits/c:dispUnits"))]
  pub c_disp_units: Option<std::boxed::Box<DisplayUnits>>,
  /// _
  #[sdk(child(qname = "c:CT_ValAxExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ValAxExtensionList>,
}
/// Category Axis Data.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:catAx.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_CatAx/c:catAx")]
pub struct CategoryAxis {
  /// Axis ID
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub axis_id: std::boxed::Box<AxisId>,
  /// Scaling
  #[sdk(child(qname = "c:CT_Scaling/c:scaling"))]
  pub scaling: std::boxed::Box<Scaling>,
  /// Delete
  #[sdk(child(qname = "c:CT_Boolean/c:delete"))]
  pub delete: Option<Delete>,
  /// Axis Position
  #[sdk(child(qname = "c:CT_AxPos/c:axPos"))]
  pub axis_position: std::boxed::Box<AxisPosition>,
  /// Major Gridlines
  #[sdk(child(qname = "c:CT_ChartLines/c:majorGridlines"))]
  pub major_gridlines: Option<std::boxed::Box<MajorGridlines>>,
  /// Minor Gridlines
  #[sdk(child(qname = "c:CT_ChartLines/c:minorGridlines"))]
  pub minor_gridlines: Option<std::boxed::Box<MinorGridlines>>,
  /// Title
  #[sdk(child(qname = "c:CT_Title/c:title"))]
  pub title: Option<std::boxed::Box<Title>>,
  /// Number Format
  #[sdk(child(qname = "c:CT_NumFmt/c:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Major Tick Mark
  #[sdk(child(qname = "c:CT_TickMark/c:majorTickMark"))]
  pub major_tick_mark: Option<MajorTickMark>,
  /// Minor Tick Mark
  #[sdk(child(qname = "c:CT_TickMark/c:minorTickMark"))]
  pub minor_tick_mark: Option<MinorTickMark>,
  /// Tick Label Position
  #[sdk(child(qname = "c:CT_TickLblPos/c:tickLblPos"))]
  pub tick_label_position: Option<TickLabelPosition>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// Defines the TextProperties Class.
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
  /// Crossing Axis ID
  #[sdk(child(qname = "c:CT_UnsignedInt/c:crossAx"))]
  pub crossing_axis: std::boxed::Box<CrossingAxis>,
  #[sdk(choice(qname = "c:CT_Crosses/c:crosses", qname = "c:CT_Double/c:crossesAt"))]
  pub category_axis_choice: Option<CategoryAxisChoice>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:auto"))]
  pub c_auto: Option<AutoLabeled>,
  /// _
  #[sdk(child(qname = "c:CT_LblAlgn/c:lblAlgn"))]
  pub c_lbl_algn: Option<LabelAlignment>,
  /// _
  #[sdk(child(qname = "c:CT_LblOffset/c:lblOffset"))]
  pub c_lbl_offset: Option<LabelOffset>,
  /// _
  #[sdk(child(qname = "c:CT_Skip/c:tickLblSkip"))]
  pub c_tick_lbl_skip: Option<TickLabelSkip>,
  /// _
  #[sdk(child(qname = "c:CT_Skip/c:tickMarkSkip"))]
  pub c_tick_mark_skip: Option<TickMarkSkip>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:noMultiLvlLbl"))]
  pub c_no_multi_lvl_lbl: Option<NoMultiLevelLabels>,
  /// _
  #[sdk(child(qname = "c:CT_CatAxExtensionList/c:extLst"))]
  pub c_ext_lst: Option<CatAxExtensionList>,
}
/// Date Axis.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:dateAx.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DateAx/c:dateAx")]
pub struct DateAxis {
  /// Axis ID
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub axis_id: std::boxed::Box<AxisId>,
  /// Scaling
  #[sdk(child(qname = "c:CT_Scaling/c:scaling"))]
  pub scaling: std::boxed::Box<Scaling>,
  /// Delete
  #[sdk(child(qname = "c:CT_Boolean/c:delete"))]
  pub delete: Option<Delete>,
  /// Axis Position
  #[sdk(child(qname = "c:CT_AxPos/c:axPos"))]
  pub axis_position: std::boxed::Box<AxisPosition>,
  /// Major Gridlines
  #[sdk(child(qname = "c:CT_ChartLines/c:majorGridlines"))]
  pub major_gridlines: Option<std::boxed::Box<MajorGridlines>>,
  /// Minor Gridlines
  #[sdk(child(qname = "c:CT_ChartLines/c:minorGridlines"))]
  pub minor_gridlines: Option<std::boxed::Box<MinorGridlines>>,
  /// Title
  #[sdk(child(qname = "c:CT_Title/c:title"))]
  pub title: Option<std::boxed::Box<Title>>,
  /// Number Format
  #[sdk(child(qname = "c:CT_NumFmt/c:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Major Tick Mark
  #[sdk(child(qname = "c:CT_TickMark/c:majorTickMark"))]
  pub major_tick_mark: Option<MajorTickMark>,
  /// Minor Tick Mark
  #[sdk(child(qname = "c:CT_TickMark/c:minorTickMark"))]
  pub minor_tick_mark: Option<MinorTickMark>,
  /// Tick Label Position
  #[sdk(child(qname = "c:CT_TickLblPos/c:tickLblPos"))]
  pub tick_label_position: Option<TickLabelPosition>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// Defines the TextProperties Class.
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
  /// Crossing Axis ID
  #[sdk(child(qname = "c:CT_UnsignedInt/c:crossAx"))]
  pub crossing_axis: std::boxed::Box<CrossingAxis>,
  #[sdk(choice(qname = "c:CT_Crosses/c:crosses", qname = "c:CT_Double/c:crossesAt"))]
  pub date_axis_choice: Option<DateAxisChoice>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:auto"))]
  pub c_auto: Option<AutoLabeled>,
  /// _
  #[sdk(child(qname = "c:CT_LblOffset/c:lblOffset"))]
  pub c_lbl_offset: Option<LabelOffset>,
  /// _
  #[sdk(child(qname = "c:CT_TimeUnit/c:baseTimeUnit"))]
  pub c_base_time_unit: Option<BaseTimeUnit>,
  /// _
  #[sdk(child(qname = "c:CT_AxisUnit/c:majorUnit"))]
  pub c_major_unit: Option<MajorUnit>,
  /// _
  #[sdk(child(qname = "c:CT_TimeUnit/c:majorTimeUnit"))]
  pub c_major_time_unit: Option<MajorTimeUnit>,
  /// _
  #[sdk(child(qname = "c:CT_AxisUnit/c:minorUnit"))]
  pub c_minor_unit: Option<MinorUnit>,
  /// _
  #[sdk(child(qname = "c:CT_TimeUnit/c:minorTimeUnit"))]
  pub c_minor_time_unit: Option<MinorTimeUnit>,
  /// _
  #[sdk(child(qname = "c:CT_DateAxExtensionList/c:extLst"))]
  pub c_ext_lst: Option<DateAxExtensionList>,
}
/// Series Axis.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:serAx.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SerAx/c:serAx")]
pub struct SeriesAxis {
  /// Axis ID
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub axis_id: std::boxed::Box<AxisId>,
  /// Scaling
  #[sdk(child(qname = "c:CT_Scaling/c:scaling"))]
  pub scaling: std::boxed::Box<Scaling>,
  /// Delete
  #[sdk(child(qname = "c:CT_Boolean/c:delete"))]
  pub delete: Option<Delete>,
  /// Axis Position
  #[sdk(child(qname = "c:CT_AxPos/c:axPos"))]
  pub axis_position: std::boxed::Box<AxisPosition>,
  /// Major Gridlines
  #[sdk(child(qname = "c:CT_ChartLines/c:majorGridlines"))]
  pub major_gridlines: Option<std::boxed::Box<MajorGridlines>>,
  /// Minor Gridlines
  #[sdk(child(qname = "c:CT_ChartLines/c:minorGridlines"))]
  pub minor_gridlines: Option<std::boxed::Box<MinorGridlines>>,
  /// Title
  #[sdk(child(qname = "c:CT_Title/c:title"))]
  pub title: Option<std::boxed::Box<Title>>,
  /// Number Format
  #[sdk(child(qname = "c:CT_NumFmt/c:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Major Tick Mark
  #[sdk(child(qname = "c:CT_TickMark/c:majorTickMark"))]
  pub major_tick_mark: Option<MajorTickMark>,
  /// Minor Tick Mark
  #[sdk(child(qname = "c:CT_TickMark/c:minorTickMark"))]
  pub minor_tick_mark: Option<MinorTickMark>,
  /// Tick Label Position
  #[sdk(child(qname = "c:CT_TickLblPos/c:tickLblPos"))]
  pub tick_label_position: Option<TickLabelPosition>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// Defines the TextProperties Class.
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
  /// Crossing Axis ID
  #[sdk(child(qname = "c:CT_UnsignedInt/c:crossAx"))]
  pub crossing_axis: std::boxed::Box<CrossingAxis>,
  #[sdk(choice(qname = "c:CT_Crosses/c:crosses", qname = "c:CT_Double/c:crossesAt"))]
  pub series_axis_choice: Option<SeriesAxisChoice>,
  /// _
  #[sdk(child(qname = "c:CT_Skip/c:tickLblSkip"))]
  pub c_tick_lbl_skip: Option<TickLabelSkip>,
  /// _
  #[sdk(child(qname = "c:CT_Skip/c:tickMarkSkip"))]
  pub c_tick_mark_skip: Option<TickMarkSkip>,
  /// _
  #[sdk(child(qname = "c:CT_SerAxExtensionList/c:extLst"))]
  pub c_ext_lst: Option<SerAxExtensionList>,
}
/// Data Table.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:dTable.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DTable/c:dTable")]
pub struct DataTable {
  /// Show Horizontal Border
  #[sdk(child(qname = "c:CT_Boolean/c:showHorzBorder"))]
  pub show_horizontal_border: Option<ShowHorizontalBorder>,
  /// Show Vertical Border
  #[sdk(child(qname = "c:CT_Boolean/c:showVertBorder"))]
  pub show_vertical_border: Option<ShowVerticalBorder>,
  /// Show Outline Border
  #[sdk(child(qname = "c:CT_Boolean/c:showOutline"))]
  pub show_outline_border: Option<ShowOutlineBorder>,
  /// Show Legend Keys
  #[sdk(child(qname = "c:CT_Boolean/c:showKeys"))]
  pub show_keys: Option<ShowKeys>,
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// Text Properties
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// First Slice Angle.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:firstSliceAng.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_FirstSliceAng/c:firstSliceAng")]
pub struct FirstSliceAngle {
  /// First Slice Angle Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "360",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: Option<crate::simple_type::UInt16Value>,
}
/// Hole Size.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:holeSize.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_HoleSize/c:holeSize")]
pub struct HoleSize {
  /// Hole Size Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 1u32,
    min = "1",
    max = "90",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: crate::simple_type::ByteValue,
}
/// String Point.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:pt.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StrVal/c:pt")]
pub struct StringPoint {
  /// Index
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  /// Text Value
  #[sdk(text_child(qname = "c:ST_Xstring/c:v"))]
  pub numeric_value: crate::simple_type::StringValue,
}
/// Thickness.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:thickness.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_WallThickness/c:thickness")]
pub struct Thickness {
  /// val
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "100",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: Option<crate::simple_type::ByteValue>,
}
/// Defines the StockChartExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StockChartExtension/c:ext")]
pub struct StockChartExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c15:CT_FilteredLineSer/c15:filteredLineSeries"))
  )]
  pub xml_children: Option<StockChartExtensionChoice>,
}
/// Defines the PieChartExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PieChartExtension/c:ext")]
pub struct PieChartExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c15:CT_FilteredPieSer/c15:filteredPieSeries"))
  )]
  pub xml_children: Option<PieChartExtensionChoice>,
}
/// Defines the Pie3DChartExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Pie3DChartExtension/c:ext")]
pub struct Pie3DChartExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c15:CT_FilteredPieSer/c15:filteredPieSeries"))
  )]
  pub xml_children: Option<Pie3DChartExtensionChoice>,
}
/// Defines the NumRefExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumRefExtension/c:ext")]
pub struct NumRefExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "c15:CT_FullRef/c15:fullRef",
      qname = "c15:CT_LevelRef/c15:levelRef",
      qname = "c15:CT_FormulaRef/c15:formulaRef"
    ))
  )]
  pub xml_children: Option<NumRefExtensionChoice>,
}
/// Defines the StrDataExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StrDataExtension/c:ext")]
pub struct StrDataExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c:CT_Boolean/c15:autoCat"))
  )]
  pub xml_children: Option<StrDataExtensionChoice>,
}
/// Defines the StrRefExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StrRefExtension/c:ext")]
pub struct StrRefExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "c15:CT_FullRef/c15:fullRef",
      qname = "c15:CT_LevelRef/c15:levelRef",
      qname = "c15:CT_FormulaRef/c15:formulaRef"
    ))
  )]
  pub xml_children: Option<StrRefExtensionChoice>,
}
/// Defines the MultiLvlStrRefExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_MultiLvlStrRefExtension/c:ext")]
pub struct MultiLvlStrRefExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "c15:CT_FullRef/c15:fullRef",
      qname = "c15:CT_LevelRef/c15:levelRef",
      qname = "c15:CT_FormulaRef/c15:formulaRef"
    ))
  )]
  pub xml_children: Option<MultiLvlStrRefExtensionChoice>,
}
/// Defines the DLblsExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DLblsExtension/c:ext")]
pub struct DLblsExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "c:CT_Tx/c15:tx",
      qname = "c15:CT_DataLabelFieldTable/c15:dlblFieldTable",
      qname = "c:CT_Boolean/c15:showDataLabelsRange",
      qname = "a:CT_ShapeProperties/c15:spPr",
      qname = "c:CT_Layout/c15:layout",
      qname = "c:CT_Boolean/c15:showLeaderLines",
      qname = "c:CT_ChartLines/c15:leaderLines"
    ))
  )]
  pub xml_children: Option<DLblsExtensionChoice>,
}
/// Defines the LineChartExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LineChartExtension/c:ext")]
pub struct LineChartExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c15:CT_FilteredLineSer/c15:filteredLineSeries"))
  )]
  pub xml_children: Option<LineChartExtensionChoice>,
}
/// Defines the Line3DChartExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Line3DChartExtension/c:ext")]
pub struct Line3DChartExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c15:CT_FilteredLineSer/c15:filteredLineSeries"))
  )]
  pub xml_children: Option<Line3DChartExtensionChoice>,
}
/// Defines the ScatterChartExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ScatterChartExtension/c:ext")]
pub struct ScatterChartExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c15:CT_FilteredScatterSer/c15:filteredScatterSeries"))
  )]
  pub xml_children: Option<ScatterChartExtensionChoice>,
}
/// Defines the RadarChartExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RadarChartExtension/c:ext")]
pub struct RadarChartExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c15:CT_FilteredRadarSer/c15:filteredRadarSeries"))
  )]
  pub xml_children: Option<RadarChartExtensionChoice>,
}
/// Defines the BarChartExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BarChartExtension/c:ext")]
pub struct BarChartExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c15:CT_FilteredBarSer/c15:filteredBarSeries"))
  )]
  pub xml_children: Option<BarChartExtensionChoice>,
}
/// Defines the Bar3DChartExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Bar3DChartExtension/c:ext")]
pub struct Bar3DChartExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c15:CT_FilteredBarSer/c15:filteredBarSeries"))
  )]
  pub xml_children: Option<Bar3DChartExtensionChoice>,
}
/// Defines the AreaChartExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AreaChartExtension/c:ext")]
pub struct AreaChartExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c15:CT_FilteredAreaSer/c15:filteredAreaSeries"))
  )]
  pub xml_children: Option<AreaChartExtensionChoice>,
}
/// Defines the Area3DChartExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Area3DChartExtension/c:ext")]
pub struct Area3DChartExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c15:CT_FilteredAreaSer/c15:filteredAreaSeries"))
  )]
  pub xml_children: Option<Area3DChartExtensionChoice>,
}
/// Defines the BubbleChartExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BubbleChartExtension/c:ext")]
pub struct BubbleChartExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c15:CT_FilteredBubbleSer/c15:filteredBubbleSeries"))
  )]
  pub xml_children: Option<BubbleChartExtensionChoice>,
}
/// Defines the SurfaceChartExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SurfaceChartExtension/c:ext")]
pub struct SurfaceChartExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c15:CT_FilteredSurfaceSer/c15:filteredSurfaceSeries"))
  )]
  pub xml_children: Option<SurfaceChartExtensionChoice>,
}
/// Defines the Surface3DChartExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Surface3DChartExtension/c:ext")]
pub struct Surface3DChartExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c15:CT_FilteredSurfaceSer/c15:filteredSurfaceSeries"))
  )]
  pub xml_children: Option<Surface3DChartExtensionChoice>,
}
/// Defines the CatAxExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_CatAxExtension/c:ext")]
pub struct CatAxExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c:CT_NumFmt/c15:numFmt"))
  )]
  pub xml_children: Option<CatAxExtensionChoice>,
}
/// Defines the DateAxExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DateAxExtension/c:ext")]
pub struct DateAxExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c:CT_NumFmt/c15:numFmt"))
  )]
  pub xml_children: Option<DateAxExtensionChoice>,
}
/// Defines the SerAxExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SerAxExtension/c:ext")]
pub struct SerAxExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c:CT_NumFmt/c15:numFmt"))
  )]
  pub xml_children: Option<SerAxExtensionChoice>,
}
/// Defines the ValAxExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ValAxExtension/c:ext")]
pub struct ValAxExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(qname = "c:CT_NumFmt/c15:numFmt"))
  )]
  pub xml_children: Option<ValAxExtensionChoice>,
}
/// Defines the UpDownBars Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:upDownBars.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UpDownBars/c:upDownBars")]
pub struct UpDownBars {
  /// Gap Width
  #[sdk(child(qname = "c:CT_GapAmount/c:gapWidth"))]
  pub gap_width: Option<GapWidth>,
  /// Up Bars
  #[sdk(child(qname = "c:CT_UpDownBar/c:upBars"))]
  pub up_bars: Option<std::boxed::Box<UpBars>>,
  /// Down Bars
  #[sdk(child(qname = "c:CT_UpDownBar/c:downBars"))]
  pub down_bars: Option<std::boxed::Box<DownBars>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the StockChartExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StockChartExtensionList/c:extLst")]
pub struct StockChartExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_StockChartExtension/c:ext"))]
  pub c_ext: Vec<StockChartExtension>,
}
/// Defines the PieChartExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PieChartExtensionList/c:extLst")]
pub struct PieChartExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_PieChartExtension/c:ext"))]
  pub c_ext: Vec<PieChartExtension>,
}
/// Defines the Pie3DChartExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Pie3DChartExtensionList/c:extLst")]
pub struct Pie3DChartExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_Pie3DChartExtension/c:ext"))]
  pub c_ext: Vec<Pie3DChartExtension>,
}
/// Defines the NumRefExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumRefExtensionList/c:extLst")]
pub struct NumRefExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_NumRefExtension/c:ext"))]
  pub c_ext: Vec<NumRefExtension>,
}
/// Defines the StrDataExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StrDataExtensionList/c:extLst")]
pub struct StrDataExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_StrDataExtension/c:ext"))]
  pub c_ext: Vec<StrDataExtension>,
}
/// Defines the StrRefExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StrRefExtensionList/c:extLst")]
pub struct StrRefExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_StrRefExtension/c:ext"))]
  pub c_ext: Vec<StrRefExtension>,
}
/// Defines the MultiLevelStringCache Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:multiLvlStrCache.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_MultiLvlStrData/c:multiLvlStrCache")]
pub struct MultiLevelStringCache {
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  pub point_count: Option<PointCount>,
  /// _
  #[sdk(child(qname = "c:CT_Lvl/c:lvl"))]
  pub c_lvl: Vec<Level>,
  /// _
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// Defines the MultiLvlStrRefExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_MultiLvlStrRefExtensionList/c:extLst")]
pub struct MultiLvlStrRefExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_MultiLvlStrRefExtension/c:ext"))]
  pub c_ext: Vec<MultiLvlStrRefExtension>,
}
/// Defines the DLblsExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DLblsExtensionList/c:extLst")]
pub struct DLblsExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_DLblsExtension/c:ext"))]
  pub c_ext: Vec<DLblsExtension>,
}
/// Defines the LineChartExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LineChartExtensionList/c:extLst")]
pub struct LineChartExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_LineChartExtension/c:ext"))]
  pub c_ext: Vec<LineChartExtension>,
}
/// Defines the Line3DChartExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Line3DChartExtensionList/c:extLst")]
pub struct Line3DChartExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_Line3DChartExtension/c:ext"))]
  pub c_ext: Vec<Line3DChartExtension>,
}
/// Defines the ScatterStyle Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:scatterStyle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ScatterStyle/c:scatterStyle")]
pub struct ScatterStyle {
  /// Scatter Style Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<ScatterStyleValues>,
}
/// Defines the ScatterChartSeries Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ser.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ScatterSer/c:ser")]
pub struct ScatterChartSeries {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: std::boxed::Box<Index>,
  /// Order
  #[sdk(child(qname = "c:CT_UnsignedInt/c:order"))]
  pub order: std::boxed::Box<Order>,
  /// Series Text
  #[sdk(child(qname = "c:CT_SerTx/c:tx"))]
  pub series_text: Option<std::boxed::Box<SeriesText>>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// _
  #[sdk(child(qname = "c:CT_Marker/c:marker"))]
  pub marker: Option<std::boxed::Box<Marker>>,
  /// _
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<DataPoint>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "c:CT_Trendline/c:trendline"))]
  pub c_trendline: Vec<Trendline>,
  /// _
  #[sdk(child(qname = "c:CT_ErrBars/c:errBars"))]
  pub c_err_bars: Vec<ErrorBars>,
  /// _
  #[sdk(child(qname = "c:CT_AxDataSource/c:xVal"))]
  pub c_x_val: Option<std::boxed::Box<XValues>>,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:yVal"))]
  pub c_y_val: Option<std::boxed::Box<YValues>>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:smooth"))]
  pub c_smooth: Option<Smooth>,
  /// _
  #[sdk(child(qname = "c:CT_ScatterSerExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ScatterSerExtensionList>,
}
/// Defines the ScatterChartExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ScatterChartExtensionList/c:extLst")]
pub struct ScatterChartExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_ScatterChartExtension/c:ext"))]
  pub c_ext: Vec<ScatterChartExtension>,
}
/// Defines the RadarStyle Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:radarStyle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RadarStyle/c:radarStyle")]
pub struct RadarStyle {
  /// Radar Style Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: RadarStyleValues,
}
/// Defines the RadarChartSeries Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ser.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RadarSer/c:ser")]
pub struct RadarChartSeries {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: std::boxed::Box<Index>,
  /// Order
  #[sdk(child(qname = "c:CT_UnsignedInt/c:order"))]
  pub order: std::boxed::Box<Order>,
  /// Series Text
  #[sdk(child(qname = "c:CT_SerTx/c:tx"))]
  pub series_text: Option<std::boxed::Box<SeriesText>>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// _
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// _
  #[sdk(child(qname = "c:CT_Marker/c:marker"))]
  pub marker: Option<std::boxed::Box<Marker>>,
  /// _
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<DataPoint>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub c_cat: Option<std::boxed::Box<CategoryAxisData>>,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub c_val: Option<std::boxed::Box<Values>>,
  /// _
  #[sdk(child(qname = "c:CT_RadarSerExtensionList/c:extLst"))]
  pub c_ext_lst: Option<RadarSerExtensionList>,
}
/// Defines the RadarChartExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RadarChartExtensionList/c:extLst")]
pub struct RadarChartExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_RadarChartExtension/c:ext"))]
  pub c_ext: Vec<RadarChartExtension>,
}
/// Defines the Overlap Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:overlap.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Overlap/c:overlap")]
pub struct Overlap {
  /// Overlap Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-100",
    max = "100",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: Option<crate::simple_type::SByteValue>,
}
/// Defines the BarChartExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BarChartExtensionList/c:extLst")]
pub struct BarChartExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_BarChartExtension/c:ext"))]
  pub c_ext: Vec<BarChartExtension>,
}
/// Defines the Shape Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:shape.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Shape/c:shape")]
pub struct Shape {
  /// Shape Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<ShapeValues>,
}
/// Defines the Bar3DChartExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Bar3DChartExtensionList/c:extLst")]
pub struct Bar3DChartExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_Bar3DChartExtension/c:ext"))]
  pub c_ext: Vec<Bar3DChartExtension>,
}
/// Defines the AreaChartExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AreaChartExtensionList/c:extLst")]
pub struct AreaChartExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_AreaChartExtension/c:ext"))]
  pub c_ext: Vec<AreaChartExtension>,
}
/// Defines the Area3DChartExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Area3DChartExtensionList/c:extLst")]
pub struct Area3DChartExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_Area3DChartExtension/c:ext"))]
  pub c_ext: Vec<Area3DChartExtension>,
}
/// Defines the BubbleChartSeries Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ser.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BubbleSer/c:ser")]
pub struct BubbleChartSeries {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: std::boxed::Box<Index>,
  /// Order
  #[sdk(child(qname = "c:CT_UnsignedInt/c:order"))]
  pub order: std::boxed::Box<Order>,
  /// Series Text
  #[sdk(child(qname = "c:CT_SerTx/c:tx"))]
  pub series_text: Option<std::boxed::Box<SeriesText>>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// _
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:invertIfNegative"))]
  pub invert_if_negative: Option<InvertIfNegative>,
  /// _
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<DataPoint>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "c:CT_Trendline/c:trendline"))]
  pub c_trendline: Vec<Trendline>,
  /// _
  #[sdk(child(qname = "c:CT_ErrBars/c:errBars"))]
  pub c_err_bars: Vec<ErrorBars>,
  /// _
  #[sdk(child(qname = "c:CT_AxDataSource/c:xVal"))]
  pub c_x_val: Option<std::boxed::Box<XValues>>,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:yVal"))]
  pub c_y_val: Option<std::boxed::Box<YValues>>,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:bubbleSize"))]
  pub c_bubble_size: Option<std::boxed::Box<BubbleSize>>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:bubble3D"))]
  pub c_bubble3_d: Option<Bubble3D>,
  /// _
  #[sdk(child(qname = "c:CT_BubbleSerExtensionList/c:extLst"))]
  pub c_ext_lst: Option<BubbleSerExtensionList>,
}
/// Defines the BubbleScale Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:bubbleScale.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BubbleScale/c:bubbleScale")]
pub struct BubbleScale {
  /// Bubble Scale Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "300",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: Option<crate::simple_type::UInt32Value>,
}
/// Defines the SizeRepresents Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:sizeRepresents.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SizeRepresents/c:sizeRepresents")]
pub struct SizeRepresents {
  /// Size Represents Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<SizeRepresentsValues>,
}
/// Defines the BubbleChartExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BubbleChartExtensionList/c:extLst")]
pub struct BubbleChartExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_BubbleChartExtension/c:ext"))]
  pub c_ext: Vec<BubbleChartExtension>,
}
/// Defines the SurfaceChartExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SurfaceChartExtensionList/c:extLst")]
pub struct SurfaceChartExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_SurfaceChartExtension/c:ext"))]
  pub c_ext: Vec<SurfaceChartExtension>,
}
/// Defines the Surface3DChartExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Surface3DChartExtensionList/c:extLst")]
pub struct Surface3DChartExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_Surface3DChartExtension/c:ext"))]
  pub c_ext: Vec<Surface3DChartExtension>,
}
/// Defines the LabelAlignment Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:lblAlgn.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LblAlgn/c:lblAlgn")]
pub struct LabelAlignment {
  /// Label Alignment Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: LabelAlignmentValues,
}
/// Defines the LabelOffset Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:lblOffset.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LblOffset/c:lblOffset")]
pub struct LabelOffset {
  /// Label Offset Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "1000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: Option<crate::simple_type::UInt16Value>,
}
/// Defines the TickLabelSkip Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:tickLblSkip.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Skip/c:tickLblSkip")]
pub struct TickLabelSkip {
  /// Tick Skip Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(source = 1u32, min = "1", min_inclusive = true, max_inclusive = false))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the TickMarkSkip Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:tickMarkSkip.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Skip/c:tickMarkSkip")]
pub struct TickMarkSkip {
  /// Tick Skip Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(source = 1u32, min = "1", min_inclusive = true, max_inclusive = false))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the SkipType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Skip/")]
pub struct SkipType {
  /// Tick Skip Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(source = 1u32, min = "1", min_inclusive = true, max_inclusive = false))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the CatAxExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_CatAxExtensionList/c:extLst")]
pub struct CatAxExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_CatAxExtension/c:ext"))]
  pub c_ext: Vec<CatAxExtension>,
}
/// Defines the BaseTimeUnit Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:baseTimeUnit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TimeUnit/c:baseTimeUnit")]
pub struct BaseTimeUnit {
  /// Time Unit Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<TimeUnitValues>,
}
/// Defines the MajorTimeUnit Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:majorTimeUnit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TimeUnit/c:majorTimeUnit")]
pub struct MajorTimeUnit {
  /// Time Unit Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<TimeUnitValues>,
}
/// Defines the MinorTimeUnit Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:minorTimeUnit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TimeUnit/c:minorTimeUnit")]
pub struct MinorTimeUnit {
  /// Time Unit Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<TimeUnitValues>,
}
/// Defines the TimeUnitType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TimeUnit/")]
pub struct TimeUnitType {
  /// Time Unit Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<TimeUnitValues>,
}
/// Defines the MajorUnit Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:majorUnit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AxisUnit/c:majorUnit")]
pub struct MajorUnit {
  /// Major Unit Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(source = 1u32, min = "0", min_inclusive = false, max_inclusive = false))]
  pub val: crate::simple_type::DoubleValue,
}
/// Defines the MinorUnit Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:minorUnit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AxisUnit/c:minorUnit")]
pub struct MinorUnit {
  /// Major Unit Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(source = 1u32, min = "0", min_inclusive = false, max_inclusive = false))]
  pub val: crate::simple_type::DoubleValue,
}
/// Defines the AxisUnitType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AxisUnit/")]
pub struct AxisUnitType {
  /// Major Unit Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(source = 1u32, min = "0", min_inclusive = false, max_inclusive = false))]
  pub val: crate::simple_type::DoubleValue,
}
/// Defines the DateAxExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DateAxExtensionList/c:extLst")]
pub struct DateAxExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_DateAxExtension/c:ext"))]
  pub c_ext: Vec<DateAxExtension>,
}
/// Defines the SerAxExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SerAxExtensionList/c:extLst")]
pub struct SerAxExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_SerAxExtension/c:ext"))]
  pub c_ext: Vec<SerAxExtension>,
}
/// Defines the CrossBetween Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:crossBetween.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_CrossBetween/c:crossBetween")]
pub struct CrossBetween {
  /// Cross Between Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: CrossBetweenValues,
}
/// Defines the DisplayUnits Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:dispUnits.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DispUnits/c:dispUnits")]
pub struct DisplayUnits {
  #[sdk(choice(
    qname = "c:CT_Double/c:custUnit",
    qname = "c:CT_BuiltInUnit/c:builtInUnit"
  ))]
  pub display_units_choice: Option<DisplayUnitsChoice>,
  /// _
  #[sdk(child(qname = "c:CT_DispUnitsLbl/c:dispUnitsLbl"))]
  pub c_disp_units_lbl: Option<std::boxed::Box<DisplayUnitsLabel>>,
  /// _
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// Defines the ValAxExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ValAxExtensionList/c:extLst")]
pub struct ValAxExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_ValAxExtension/c:ext"))]
  pub c_ext: Vec<ValAxExtension>,
}
/// Defines the DLblExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DLblExtensionList/c:extLst")]
pub struct DLblExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_DLblExtension/c:ext"))]
  pub c_ext: Vec<DLblExtension>,
}
/// Defines the DLblExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DLblExtension/c:ext")]
pub struct DLblExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "c15:CT_DataLabelFieldTable/c15:dlblFieldTable",
      qname = "c:CT_Boolean/c15:xForSave",
      qname = "c:CT_Boolean/c15:showDataLabelsRange",
      qname = "a:CT_ShapeProperties/c15:spPr",
      qname = "c:CT_Layout/c15:layout",
      qname = "c16:CT_ChartUniqueID/c16:uniqueId"
    ))
  )]
  pub xml_children: Option<DLblExtensionChoice>,
}
/// Defines the DataPoint Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:dPt.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DPt/c:dPt")]
pub struct DataPoint {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: std::boxed::Box<Index>,
  /// Invert if Negative
  #[sdk(child(qname = "c:CT_Boolean/c:invertIfNegative"))]
  pub invert_if_negative: Option<InvertIfNegative>,
  /// Marker
  #[sdk(child(qname = "c:CT_Marker/c:marker"))]
  pub marker: Option<std::boxed::Box<Marker>>,
  /// 3D Bubble
  #[sdk(child(qname = "c:CT_Boolean/c:bubble3D"))]
  pub bubble3_d: Option<Bubble3D>,
  /// Explosion
  #[sdk(child(qname = "c:CT_UnsignedInt/c:explosion"))]
  pub explosion: Option<Explosion>,
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// _
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Trendline Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:trendline.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Trendline/c:trendline")]
pub struct Trendline {
  /// Trendline Name
  #[sdk(text_child(qname = "xsd:string/c:name"))]
  pub trendline_name: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// Trendline Type
  #[sdk(child(qname = "c:CT_TrendlineType/c:trendlineType"))]
  pub trendline_type: std::boxed::Box<TrendlineType>,
  /// Polynomial Trendline Order
  #[sdk(child(qname = "c:CT_Order/c:order"))]
  pub polynomial_order: Option<PolynomialOrder>,
  /// Period
  #[sdk(child(qname = "c:CT_Period/c:period"))]
  pub period: Option<Period>,
  /// Forward
  #[sdk(child(qname = "c:CT_Double/c:forward"))]
  pub forward: Option<Forward>,
  /// Backward
  #[sdk(child(qname = "c:CT_Double/c:backward"))]
  pub backward: Option<Backward>,
  /// Intercept
  #[sdk(child(qname = "c:CT_Double/c:intercept"))]
  pub intercept: Option<Intercept>,
  /// Display R Squared Value
  #[sdk(child(qname = "c:CT_Boolean/c:dispRSqr"))]
  pub display_r_squared_value: Option<DisplayRSquaredValue>,
  /// Display Equation
  #[sdk(child(qname = "c:CT_Boolean/c:dispEq"))]
  pub display_equation: Option<DisplayEquation>,
  /// Trendline Label
  #[sdk(child(qname = "c:CT_TrendlineLbl/c:trendlineLbl"))]
  pub trendline_label: Option<std::boxed::Box<TrendlineLabel>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ErrorBars Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:errBars.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ErrBars/c:errBars")]
pub struct ErrorBars {
  /// Error Bar Direction
  #[sdk(child(qname = "c:CT_ErrDir/c:errDir"))]
  pub error_direction: Option<ErrorDirection>,
  /// Error Bar Type
  #[sdk(child(qname = "c:CT_ErrBarType/c:errBarType"))]
  pub error_bar_type: std::boxed::Box<ErrorBarType>,
  /// Error Bar Value Type
  #[sdk(child(qname = "c:CT_ErrValType/c:errValType"))]
  pub error_bar_value_type: std::boxed::Box<ErrorBarValueType>,
  /// No End Cap
  #[sdk(child(qname = "c:CT_Boolean/c:noEndCap"))]
  pub no_end_cap: Option<NoEndCap>,
  /// Plus
  #[sdk(child(qname = "c:CT_NumDataSource/c:plus"))]
  pub plus: Option<std::boxed::Box<Plus>>,
  /// Minus
  #[sdk(child(qname = "c:CT_NumDataSource/c:minus"))]
  pub minus: Option<std::boxed::Box<Minus>>,
  /// Error Bar Value
  #[sdk(child(qname = "c:CT_Double/c:val"))]
  pub error_bar_value: Option<ErrorBarValue>,
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CategoryAxisData Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:cat.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AxDataSource/c:cat")]
pub struct CategoryAxisData {
  #[sdk(choice(
    qname = "c:CT_MultiLvlStrRef/c:multiLvlStrRef",
    qname = "c:CT_NumRef/c:numRef",
    qname = "c:CT_NumData/c:numLit",
    qname = "c:CT_StrRef/c:strRef",
    qname = "c:CT_StrData/c:strLit"
  ))]
  pub xml_children: Option<CategoryAxisDataChoice>,
}
/// Defines the XValues Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:xVal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AxDataSource/c:xVal")]
pub struct XValues {
  #[sdk(choice(
    qname = "c:CT_MultiLvlStrRef/c:multiLvlStrRef",
    qname = "c:CT_NumRef/c:numRef",
    qname = "c:CT_NumData/c:numLit",
    qname = "c:CT_StrRef/c:strRef",
    qname = "c:CT_StrData/c:strLit"
  ))]
  pub xml_children: Option<XValuesChoice>,
}
/// Defines the AxisDataSourceType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AxDataSource/")]
pub struct AxisDataSourceType {
  #[sdk(choice(
    qname = "c:CT_MultiLvlStrRef/c:multiLvlStrRef",
    qname = "c:CT_NumRef/c:numRef",
    qname = "c:CT_NumData/c:numLit",
    qname = "c:CT_StrRef/c:strRef",
    qname = "c:CT_StrData/c:strLit"
  ))]
  pub xml_children: Option<AxisDataSourceTypeChoice>,
}
/// Defines the LineSerExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LineSerExtensionList/c:extLst")]
pub struct LineSerExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_LineSerExtension/c:ext"))]
  pub c_ext: Vec<LineSerExtension>,
}
/// Defines the LineSerExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LineSerExtension/c:ext")]
pub struct LineSerExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle",
      qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle",
      qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange",
      qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions",
      qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions",
      qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap",
      qname = "c16:CT_ChartUniqueID/c16:uniqueId"
    ))
  )]
  pub xml_children: Option<LineSerExtensionChoice>,
}
/// Defines the ScatterSerExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ScatterSerExtensionList/c:extLst")]
pub struct ScatterSerExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_ScatterSerExtension/c:ext"))]
  pub c_ext: Vec<ScatterSerExtension>,
}
/// Defines the ScatterSerExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ScatterSerExtension/c:ext")]
pub struct ScatterSerExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle",
      qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle",
      qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange",
      qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions",
      qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions",
      qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap",
      qname = "c16:CT_ChartUniqueID/c16:uniqueId"
    ))
  )]
  pub xml_children: Option<ScatterSerExtensionChoice>,
}
/// Defines the RadarSerExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RadarSerExtensionList/c:extLst")]
pub struct RadarSerExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_RadarSerExtension/c:ext"))]
  pub c_ext: Vec<RadarSerExtension>,
}
/// Defines the RadarSerExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RadarSerExtension/c:ext")]
pub struct RadarSerExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle",
      qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle",
      qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange",
      qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions",
      qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions",
      qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap",
      qname = "c16:CT_ChartUniqueID/c16:uniqueId"
    ))
  )]
  pub xml_children: Option<RadarSerExtensionChoice>,
}
/// Defines the BarSerExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BarSerExtensionList/c:extLst")]
pub struct BarSerExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_BarSerExtension/c:ext"))]
  pub c_ext: Vec<BarSerExtension>,
}
/// Defines the BarSerExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BarSerExtension/c:ext")]
pub struct BarSerExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "c14:CT_InvertSolidFillFmt/c14:invertSolidFillFmt",
      qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle",
      qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle",
      qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange",
      qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions",
      qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions",
      qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap",
      qname = "c16:CT_ChartUniqueID/c16:uniqueId"
    ))
  )]
  pub xml_children: Option<BarSerExtensionChoice>,
}
/// Defines the AreaSerExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AreaSerExtensionList/c:extLst")]
pub struct AreaSerExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_AreaSerExtension/c:ext"))]
  pub c_ext: Vec<AreaSerExtension>,
}
/// Defines the AreaSerExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AreaSerExtension/c:ext")]
pub struct AreaSerExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle",
      qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle",
      qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange",
      qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions",
      qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions",
      qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap",
      qname = "c16:CT_ChartUniqueID/c16:uniqueId"
    ))
  )]
  pub xml_children: Option<AreaSerExtensionChoice>,
}
/// Defines the PieSerExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PieSerExtensionList/c:extLst")]
pub struct PieSerExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_PieSerExtension/c:ext"))]
  pub c_ext: Vec<PieSerExtension>,
}
/// Defines the PieSerExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PieSerExtension/c:ext")]
pub struct PieSerExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle",
      qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle",
      qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange",
      qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions",
      qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions",
      qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap",
      qname = "c16:CT_ChartUniqueID/c16:uniqueId"
    ))
  )]
  pub xml_children: Option<PieSerExtensionChoice>,
}
/// Defines the BubbleSerExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BubbleSerExtensionList/c:extLst")]
pub struct BubbleSerExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_BubbleSerExtension/c:ext"))]
  pub c_ext: Vec<BubbleSerExtension>,
}
/// Defines the BubbleSerExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BubbleSerExtension/c:ext")]
pub struct BubbleSerExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "c14:CT_InvertSolidFillFmt/c14:invertSolidFillFmt",
      qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle",
      qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange",
      qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions",
      qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions",
      qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap",
      qname = "c16:CT_ChartUniqueID/c16:uniqueId"
    ))
  )]
  pub xml_children: Option<BubbleSerExtensionChoice>,
}
/// Defines the SurfaceSerExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SurfaceSerExtensionList/c:extLst")]
pub struct SurfaceSerExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_SurfaceSerExtension/c:ext"))]
  pub c_ext: Vec<SurfaceSerExtension>,
}
/// Defines the SurfaceSerExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SurfaceSerExtension/c:ext")]
pub struct SurfaceSerExtension {
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle",
      qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle",
      qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions",
      qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions",
      qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap",
      qname = "c16:CT_ChartUniqueID/c16:uniqueId"
    ))
  )]
  pub xml_children: Option<SurfaceSerExtensionChoice>,
}
#[cfg(feature = "microsoft365")]
/// Defines the DataDisplayOptions16 Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16r3:CT_DataDisplayOptions16/c:ext")]
pub struct DataDisplayOptions16 {
  /// _
  #[sdk(child(qname = "c16r3:CT_BooleanFalse/c16r3:dispNaAsBlank"))]
  pub boolean_false:
    Option<crate::schemas::schemas_microsoft_com_office_drawing_2017_03_chart::BooleanFalse>,
}
/// pivot chart format persistence data.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:pivotFmts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PivotFmts/c:pivotFmts")]
pub struct PivotFormats {
  /// _
  #[sdk(child(qname = "c:CT_PivotFmt/c:pivotFmt"))]
  pub c_pivot_fmt: Vec<PivotFormat>,
}
/// 3D view settings.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:view3D.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_View3D/c:view3D")]
pub struct View3D {
  /// X Rotation
  #[sdk(child(qname = "c:CT_RotX/c:rotX"))]
  pub rotate_x: Option<RotateX>,
  /// Height Percent
  #[sdk(child(qname = "c:CT_HPercent/c:hPercent"))]
  pub height_percent: Option<HeightPercent>,
  /// Y Rotation
  #[sdk(child(qname = "c:CT_RotY/c:rotY"))]
  pub rotate_y: Option<RotateY>,
  /// Depth Percent
  #[sdk(child(qname = "c:CT_DepthPercent/c:depthPercent"))]
  pub depth_percent: Option<DepthPercent>,
  /// Right Angle Axes
  #[sdk(child(qname = "c:CT_Boolean/c:rAngAx"))]
  pub right_angle_axes: Option<RightAngleAxes>,
  /// Perspective
  #[sdk(child(qname = "c:CT_Perspective/c:perspective"))]
  pub perspective: Option<Perspective>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// 3D floor formatting.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:floor.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Surface/c:floor")]
pub struct Floor {
  /// Thickness
  #[sdk(child(qname = "c:CT_WallThickness/c:thickness"))]
  pub thickness: Option<Thickness>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/c:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Picture Options
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// 3D side wall formatting.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:sideWall.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Surface/c:sideWall")]
pub struct SideWall {
  /// Thickness
  #[sdk(child(qname = "c:CT_WallThickness/c:thickness"))]
  pub thickness: Option<Thickness>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/c:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Picture Options
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// 3D back wall formatting.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:backWall.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Surface/c:backWall")]
pub struct BackWall {
  /// Thickness
  #[sdk(child(qname = "c:CT_WallThickness/c:thickness"))]
  pub thickness: Option<Thickness>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/c:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Picture Options
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the SurfaceType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Surface/")]
pub struct SurfaceType {
  #[sdk(choice(
    qname = "c:CT_WallThickness/c:thickness",
    qname = "a:CT_ShapeProperties/c:spPr",
    qname = "c:CT_PictureOptions/c:pictureOptions",
    qname = "c:CT_ExtensionList/c:extLst"
  ))]
  pub xml_children: Vec<SurfaceTypeChoice>,
}
/// Plot data and formatting.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:plotArea.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PlotArea/c:plotArea")]
pub struct PlotArea {
  /// Layout
  #[sdk(child(qname = "c:CT_Layout/c:layout"))]
  pub layout: Option<std::boxed::Box<Layout>>,
  #[sdk(choice(
    qname = "c:CT_AreaChart/c:areaChart",
    qname = "c:CT_Area3DChart/c:area3DChart",
    qname = "c:CT_LineChart/c:lineChart",
    qname = "c:CT_Line3DChart/c:line3DChart",
    qname = "c:CT_StockChart/c:stockChart",
    qname = "c:CT_RadarChart/c:radarChart",
    qname = "c:CT_ScatterChart/c:scatterChart",
    qname = "c:CT_PieChart/c:pieChart",
    qname = "c:CT_Pie3DChart/c:pie3DChart",
    qname = "c:CT_DoughnutChart/c:doughnutChart",
    qname = "c:CT_BarChart/c:barChart",
    qname = "c:CT_Bar3DChart/c:bar3DChart",
    qname = "c:CT_OfPieChart/c:ofPieChart",
    qname = "c:CT_SurfaceChart/c:surfaceChart",
    qname = "c:CT_Surface3DChart/c:surface3DChart",
    qname = "c:CT_BubbleChart/c:bubbleChart"
  ))]
  pub plot_area_choice1: Vec<PlotAreaChoice>,
  #[sdk(choice(
    qname = "c:CT_ValAx/c:valAx",
    qname = "c:CT_CatAx/c:catAx",
    qname = "c:CT_DateAx/c:dateAx",
    qname = "c:CT_SerAx/c:serAx"
  ))]
  pub plot_area_choice2: Vec<PlotAreaChoice2>,
  /// _
  #[sdk(child(qname = "c:CT_DTable/c:dTable"))]
  pub c_d_table: Option<std::boxed::Box<DataTable>>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/c:spPr"))]
  pub c_sp_pr: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// Legend data and formatting.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:legend.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Legend/c:legend")]
pub struct Legend {
  /// Legend Position
  #[sdk(child(qname = "c:CT_LegendPos/c:legendPos"))]
  pub legend_position: Option<LegendPosition>,
  /// _
  #[sdk(child(qname = "c:CT_LegendEntry/c:legendEntry"))]
  pub c_legend_entry: Vec<LegendEntry>,
  /// _
  #[sdk(child(qname = "c:CT_Layout/c:layout"))]
  pub c_layout: Option<std::boxed::Box<Layout>>,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:overlay"))]
  pub c_overlay: Option<Overlay>,
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub c_sp_pr: Option<std::boxed::Box<ChartShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub c_tx_pr: Option<std::boxed::Box<TextProperties>>,
  /// _
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// The way that blank cells are plotted on a chart..
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:dispBlanksAs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DispBlanksAs/c:dispBlanksAs")]
pub struct DisplayBlanksAs {
  /// Display Blanks As Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<DisplayBlanksAsValues>,
}
/// Extensibility container.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartExtensionList/c:extLst")]
pub struct ChartExtensionList {
  #[cfg(not(feature = "mce"))]
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Vec<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[cfg(feature = "microsoft365")]
  /// _
  #[sdk(mce_child(qname = "c16r3:CT_DataDisplayOptions16/c:ext"))]
  pub c_ext: Vec<DataDisplayOptions16>,
}
/// Defines the EditingLanguage Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:lang.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TextLanguageID/c:lang")]
pub struct EditingLanguage {
  /// Language Code
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the Style Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:style.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Style/c:style")]
pub struct Style {
  /// Style Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "48",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub val: Option<crate::simple_type::ByteValue>,
}
/// Defines the ColorMapOverride Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:clrMapOvr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ColorMapping/c:clrMapOvr")]
pub struct ColorMapOverride {
  /// Background 1
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bg1
  #[sdk(attr(qname = ":bg1"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub background1:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Text 1
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tx1
  #[sdk(attr(qname = ":tx1"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub text1: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Background 2
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bg2
  #[sdk(attr(qname = ":bg2"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub background2:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Text 2
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tx2
  #[sdk(attr(qname = ":tx2"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub text2: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 1
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :accent1
  #[sdk(attr(qname = ":accent1"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent1:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 2
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :accent2
  #[sdk(attr(qname = ":accent2"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent2:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 3
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :accent3
  #[sdk(attr(qname = ":accent3"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent3:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 4
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :accent4
  #[sdk(attr(qname = ":accent4"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent4:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 5
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :accent5
  #[sdk(attr(qname = ":accent5"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent5:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 6
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :accent6
  #[sdk(attr(qname = ":accent6"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub accent6:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Hyperlink
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hlink
  #[sdk(attr(qname = ":hlink"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub hyperlink:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Followed Hyperlink
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :folHlink
  #[sdk(attr(qname = ":folHlink"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub followed_hyperlink:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the PivotSource Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:pivotSource.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PivotSource/c:pivotSource")]
pub struct PivotSource {
  /// Pivot Name
  #[sdk(text_child(qname = "c:ST_Xstring/c:name"))]
  pub pivot_table_name: crate::simple_type::StringValue,
  /// Format ID
  #[sdk(child(qname = "c:CT_UnsignedInt/c:fmtId"))]
  pub format_id: std::boxed::Box<FormatId>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Protection Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:protection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Protection/c:protection")]
pub struct Protection {
  /// Chart Object
  #[sdk(child(qname = "c:CT_Boolean/c:chartObject"))]
  pub chart_object: Option<ChartObject>,
  /// Data Cannot Be Changed
  #[sdk(child(qname = "c:CT_Boolean/c:data"))]
  pub data: Option<Data>,
  /// Formatting
  #[sdk(child(qname = "c:CT_Boolean/c:formatting"))]
  pub formatting: Option<Formatting>,
  /// Selection
  #[sdk(child(qname = "c:CT_Boolean/c:selection"))]
  pub selection: Option<Selection>,
  /// User Interface
  #[sdk(child(qname = "c:CT_Boolean/c:userInterface"))]
  pub user_interface: Option<UserInterface>,
}
/// Defines the Chart Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:chart.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Chart/c:chart")]
pub struct Chart {
  /// Title data and formatting
  #[sdk(child(qname = "c:CT_Title/c:title"))]
  pub title: Option<std::boxed::Box<Title>>,
  /// True if the chart automatic title has been deleted.
  #[sdk(child(qname = "c:CT_Boolean/c:autoTitleDeleted"))]
  pub auto_title_deleted: Option<AutoTitleDeleted>,
  /// pivot chart format persistence data
  #[sdk(child(qname = "c:CT_PivotFmts/c:pivotFmts"))]
  pub pivot_formats: Option<PivotFormats>,
  /// 3D view settings
  #[sdk(child(qname = "c:CT_View3D/c:view3D"))]
  pub view3_d: Option<std::boxed::Box<View3D>>,
  /// 3D floor formatting
  #[sdk(child(qname = "c:CT_Surface/c:floor"))]
  pub floor: Option<std::boxed::Box<Floor>>,
  /// 3D side wall formatting
  #[sdk(child(qname = "c:CT_Surface/c:sideWall"))]
  pub side_wall: Option<std::boxed::Box<SideWall>>,
  /// 3D back wall formatting
  #[sdk(child(qname = "c:CT_Surface/c:backWall"))]
  pub back_wall: Option<std::boxed::Box<BackWall>>,
  /// Plot data and formatting
  #[sdk(child(qname = "c:CT_PlotArea/c:plotArea"))]
  pub plot_area: std::boxed::Box<PlotArea>,
  /// Legend data and formatting
  #[sdk(child(qname = "c:CT_Legend/c:legend"))]
  pub legend: Option<std::boxed::Box<Legend>>,
  /// True if only visible cells are plotted.
  #[sdk(child(qname = "c:CT_Boolean/c:plotVisOnly"))]
  pub plot_visible_only: Option<PlotVisibleOnly>,
  /// The way that blank cells are plotted on a chart.
  #[sdk(child(qname = "c:CT_DispBlanksAs/c:dispBlanksAs"))]
  pub display_blanks_as: Option<DisplayBlanksAs>,
  /// True if we should render datalabels over the maximum scale
  #[sdk(child(qname = "c:CT_Boolean/c:showDLblsOverMax"))]
  pub show_data_labels_over_maximum: Option<ShowDataLabelsOverMaximum>,
  /// Extensibility container
  #[sdk(child(qname = "c:CT_ChartExtensionList/c:extLst"))]
  pub chart_extension_list: Option<ChartExtensionList>,
}
/// Defines the ExternalData Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:externalData.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ExternalData/c:externalData")]
pub struct ExternalData {
  /// Relationship Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// Update Automatically
  #[sdk(child(qname = "c:CT_Boolean/c:autoUpdate"))]
  pub auto_update: Option<AutoUpdate>,
}
/// Defines the PrintSettings Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:printSettings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PrintSettings/c:printSettings")]
pub struct PrintSettings {
  /// Header and Footer
  #[sdk(child(qname = "c:CT_HeaderFooter/c:headerFooter"))]
  pub header_footer: Option<HeaderFooter>,
  /// Page Margins
  #[sdk(child(qname = "c:CT_PageMargins/c:pageMargins"))]
  pub page_margins: Option<PageMargins>,
  /// Page Setup
  #[sdk(child(qname = "c:CT_PageSetup/c:pageSetup"))]
  pub page_setup: Option<PageSetup>,
  /// Legacy Drawing for Headers and Footers
  #[sdk(child(qname = "c:CT_RelId/c:legacyDrawingHF"))]
  pub legacy_drawing_header_footer: Option<LegacyDrawingHeaderFooter>,
}
/// Defines the ChartSpaceExtensionList Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartSpaceExtensionList/c:extLst")]
pub struct ChartSpaceExtensionList {
  /// _
  #[sdk(child(qname = "c:CT_ChartSpaceExtension/c:ext"))]
  pub c_ext: Vec<ChartSpaceExtension>,
}
/// Defines the ChartSpaceExtension Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is c:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartSpaceExtension/c:ext")]
pub struct ChartSpaceExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// URI
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(any))]
  #[cfg_attr(
    feature = "microsoft365",
    sdk(choice(
      qname = "c14:CT_PivotOptions/c14:pivotOptions",
      qname = "c14:CT_SketchOptions/c14:sketchOptions",
      qname = "c:CT_PivotSource/c15:pivotSource"
    ))
  )]
  pub xml_children: Option<ChartSpaceExtensionChoice>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ChartShapePropertiesChoice {
  #[sdk(child(qname = "a:CT_CustomGeometry2D/a:custGeom"))]
  ACustGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomGeometry>,
  ),
  #[sdk(child(qname = "a:CT_PresetGeometry2D/a:prstGeom"))]
  APrstGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetGeometry>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ChartShapePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill>,
  ),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill>,
  ),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill>,
  ),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ChartShapePropertiesChoice3 {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList>,
  ),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextBodyTypeChoice {
  /// Body Properties
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  ABodyPr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BodyProperties>,
  ),
  /// Text List Styles
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  ALstStyle(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ListStyle>,
  ),
  #[sdk(child(qname = "a:CT_TextParagraph/a:p"))]
  AP(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Paragraph>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ChartTextChoice {
  /// Defines the StringReference Class.
  #[sdk(child(qname = "c:CT_StrRef/c:strRef"))]
  CStrRef(std::boxed::Box<StringReference>),
  /// Rich Text.
  #[sdk(child(qname = "a:CT_TextBody/c:rich"))]
  CRich(std::boxed::Box<RichText>),
  /// String Literal.
  #[sdk(child(qname = "c:CT_StrData/c:strLit"))]
  CStrLit(std::boxed::Box<StringLiteral>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SeriesTextChoice {
  /// Defines the StringReference Class.
  #[sdk(child(qname = "c:CT_StrRef/c:strRef"))]
  CStrRef(std::boxed::Box<StringReference>),
  /// Numeric Value.
  #[sdk(text_child(qname = "c:ST_Xstring/c:v"))]
  CV(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct DataLabelsChoiceSequence {
  /// Number Format.
  #[sdk(child(qname = "c:CT_NumFmt/c:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// Defines the TextProperties Class.
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
  /// Data Label Position.
  #[sdk(child(qname = "c:CT_DLblPos/c:dLblPos"))]
  pub data_label_position: Option<DataLabelPosition>,
  /// Show Legend Key.
  #[sdk(child(qname = "c:CT_Boolean/c:showLegendKey"))]
  pub show_legend_key: Option<ShowLegendKey>,
  /// Show Value.
  #[sdk(child(qname = "c:CT_Boolean/c:showVal"))]
  pub show_value: Option<ShowValue>,
  /// Show Category Name.
  #[sdk(child(qname = "c:CT_Boolean/c:showCatName"))]
  pub show_category_name: Option<ShowCategoryName>,
  /// Show Series Name.
  #[sdk(child(qname = "c:CT_Boolean/c:showSerName"))]
  pub show_series_name: Option<ShowSeriesName>,
  /// Show Percent.
  #[sdk(child(qname = "c:CT_Boolean/c:showPercent"))]
  pub show_percent: Option<ShowPercent>,
  /// Show Bubble Size.
  #[sdk(child(qname = "c:CT_Boolean/c:showBubbleSize"))]
  pub show_bubble_size: Option<ShowBubbleSize>,
  /// Separator.
  #[sdk(text_child(qname = "xsd:string/c:separator"))]
  pub separator: Option<crate::simple_type::StringValue>,
  /// Show Leader Lines.
  #[sdk(child(qname = "c:CT_Boolean/c:showLeaderLines"))]
  pub show_leader_lines: Option<ShowLeaderLines>,
  /// Leader Lines.
  #[sdk(child(qname = "c:CT_ChartLines/c:leaderLines"))]
  pub leader_lines: Option<std::boxed::Box<LeaderLines>>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DataLabelsChoice {
  /// Delete.
  #[sdk(child(qname = "c:CT_Boolean/c:delete"))]
  CDelete(std::boxed::Box<Delete>),
  /// Sequence of c:numFmt, c:spPr, c:txPr, c:dLblPos, c:showLegendKey, c:showVal, c:showCatName, c:showSerName, c:showPercent, c:showBubbleSize, c:separator, c:showLeaderLines, c:leaderLines
  #[sdk(sequence)]
  Sequence(std::boxed::Box<DataLabelsChoiceSequence>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ChartSpaceChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c14:CT_Style/c14:style"))]
  C14Style(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::Style>,
  ),
  #[sdk(child(qname = "c:CT_Style/c:style"))]
  CStyle(std::boxed::Box<Style>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum UserShapesChoice {
  #[sdk(child(qname = "cdr:CT_RelSizeAnchor/cdr:relSizeAnchor"))]
  CdrRelSizeAnchor(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart_drawing::RelativeAnchorSize,
    >,
  ),
  #[sdk(child(qname = "cdr:CT_AbsSizeAnchor/cdr:absSizeAnchor"))]
  CdrAbsSizeAnchor(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart_drawing::AbsoluteAnchorSize,
    >,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumberDataTypeChoice {
  /// Format Code
  #[sdk(text_child(qname = "c:ST_Xstring/c:formatCode"))]
  CFormatCode(crate::simple_type::StringValue),
  /// Point Count
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  CPtCount(std::boxed::Box<PointCount>),
  #[sdk(child(qname = "c:CT_NumVal/c:pt"))]
  CPt(std::boxed::Box<NumericPoint>),
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  CExtLst(std::boxed::Box<ExtensionList>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum StringDataTypeChoice {
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  CPtCount(std::boxed::Box<PointCount>),
  #[sdk(child(qname = "c:CT_StrVal/c:pt"))]
  CPt(std::boxed::Box<StringPoint>),
  #[sdk(child(qname = "c:CT_StrDataExtensionList/c:extLst"))]
  CExtLst(std::boxed::Box<StrDataExtensionList>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PlusChoice {
  /// Number Reference.
  #[sdk(child(qname = "c:CT_NumRef/c:numRef"))]
  CNumRef(std::boxed::Box<NumberReference>),
  /// Number Literal.
  #[sdk(child(qname = "c:CT_NumData/c:numLit"))]
  CNumLit(std::boxed::Box<NumberLiteral>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MinusChoice {
  /// Number Reference.
  #[sdk(child(qname = "c:CT_NumRef/c:numRef"))]
  CNumRef(std::boxed::Box<NumberReference>),
  /// Number Literal.
  #[sdk(child(qname = "c:CT_NumData/c:numLit"))]
  CNumLit(std::boxed::Box<NumberLiteral>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ValuesChoice {
  /// Number Reference.
  #[sdk(child(qname = "c:CT_NumRef/c:numRef"))]
  CNumRef(std::boxed::Box<NumberReference>),
  /// Number Literal.
  #[sdk(child(qname = "c:CT_NumData/c:numLit"))]
  CNumLit(std::boxed::Box<NumberLiteral>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum YValuesChoice {
  /// Number Reference.
  #[sdk(child(qname = "c:CT_NumRef/c:numRef"))]
  CNumRef(std::boxed::Box<NumberReference>),
  /// Number Literal.
  #[sdk(child(qname = "c:CT_NumData/c:numLit"))]
  CNumLit(std::boxed::Box<NumberLiteral>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BubbleSizeChoice {
  /// Number Reference.
  #[sdk(child(qname = "c:CT_NumRef/c:numRef"))]
  CNumRef(std::boxed::Box<NumberReference>),
  /// Number Literal.
  #[sdk(child(qname = "c:CT_NumData/c:numLit"))]
  CNumLit(std::boxed::Box<NumberLiteral>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumberDataSourceTypeChoice {
  /// Number Reference.
  #[sdk(child(qname = "c:CT_NumRef/c:numRef"))]
  CNumRef(std::boxed::Box<NumberReference>),
  /// Number Literal.
  #[sdk(child(qname = "c:CT_NumData/c:numLit"))]
  CNumLit(std::boxed::Box<NumberLiteral>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LegendEntryChoice {
  /// Delete.
  #[sdk(child(qname = "c:CT_Boolean/c:delete"))]
  CDelete(std::boxed::Box<Delete>),
  /// Defines the TextProperties Class.
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  CTxPr(std::boxed::Box<TextProperties>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice {
  #[sdk(child(qname = "a:CT_CustomGeometry2D/a:custGeom"))]
  ACustGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomGeometry>,
  ),
  #[sdk(child(qname = "a:CT_PresetGeometry2D/a:prstGeom"))]
  APrstGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetGeometry>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice2 {
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill>),
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill>,
  ),
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill>,
  ),
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill>,
  ),
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill>,
  ),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice3 {
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList>,
  ),
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag>,
  ),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct DataLabelChoiceSequence {
  /// Layout.
  #[sdk(child(qname = "c:CT_Layout/c:layout"))]
  pub layout: Option<std::boxed::Box<Layout>>,
  /// Defines the ChartText Class.
  #[sdk(child(qname = "c:CT_Tx/c:tx"))]
  pub chart_text: Option<std::boxed::Box<ChartText>>,
  /// Number Format.
  #[sdk(child(qname = "c:CT_NumFmt/c:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// Defines the TextProperties Class.
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
  /// Data Label Position.
  #[sdk(child(qname = "c:CT_DLblPos/c:dLblPos"))]
  pub data_label_position: Option<DataLabelPosition>,
  /// Show Legend Key.
  #[sdk(child(qname = "c:CT_Boolean/c:showLegendKey"))]
  pub show_legend_key: Option<ShowLegendKey>,
  /// Show Value.
  #[sdk(child(qname = "c:CT_Boolean/c:showVal"))]
  pub show_value: Option<ShowValue>,
  /// Show Category Name.
  #[sdk(child(qname = "c:CT_Boolean/c:showCatName"))]
  pub show_category_name: Option<ShowCategoryName>,
  /// Show Series Name.
  #[sdk(child(qname = "c:CT_Boolean/c:showSerName"))]
  pub show_series_name: Option<ShowSeriesName>,
  /// Show Percent.
  #[sdk(child(qname = "c:CT_Boolean/c:showPercent"))]
  pub show_percent: Option<ShowPercent>,
  /// Show Bubble Size.
  #[sdk(child(qname = "c:CT_Boolean/c:showBubbleSize"))]
  pub show_bubble_size: Option<ShowBubbleSize>,
  /// Separator.
  #[sdk(text_child(qname = "xsd:string/c:separator"))]
  pub separator: Option<crate::simple_type::StringValue>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DataLabelChoice {
  /// Delete.
  #[sdk(child(qname = "c:CT_Boolean/c:delete"))]
  CDelete(std::boxed::Box<Delete>),
  /// Sequence of c:layout, c:tx, c:numFmt, c:spPr, c:txPr, c:dLblPos, c:showLegendKey, c:showVal, c:showCatName, c:showSerName, c:showPercent, c:showBubbleSize, c:separator
  #[sdk(sequence)]
  Sequence(std::boxed::Box<DataLabelChoiceSequence>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ValueAxisChoice {
  #[sdk(child(qname = "c:CT_Crosses/c:crosses"))]
  CCrosses(std::boxed::Box<Crosses>),
  #[sdk(child(qname = "c:CT_Double/c:crossesAt"))]
  CCrossesAt(std::boxed::Box<CrossesAt>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CategoryAxisChoice {
  #[sdk(child(qname = "c:CT_Crosses/c:crosses"))]
  CCrosses(std::boxed::Box<Crosses>),
  #[sdk(child(qname = "c:CT_Double/c:crossesAt"))]
  CCrossesAt(std::boxed::Box<CrossesAt>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DateAxisChoice {
  #[sdk(child(qname = "c:CT_Crosses/c:crosses"))]
  CCrosses(std::boxed::Box<Crosses>),
  #[sdk(child(qname = "c:CT_Double/c:crossesAt"))]
  CCrossesAt(std::boxed::Box<CrossesAt>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SeriesAxisChoice {
  #[sdk(child(qname = "c:CT_Crosses/c:crosses"))]
  CCrosses(std::boxed::Box<Crosses>),
  #[sdk(child(qname = "c:CT_Double/c:crossesAt"))]
  CCrossesAt(std::boxed::Box<CrossesAt>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum StockChartExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredLineSer/c15:filteredLineSeries"))]
  C15FilteredLineSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredLineSeriesExtension,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PieChartExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredPieSer/c15:filteredPieSeries"))]
  C15FilteredPieSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredPieSeries,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Pie3DChartExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredPieSer/c15:filteredPieSeries"))]
  C15FilteredPieSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredPieSeries,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumRefExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FullRef/c15:fullRef"))]
  C15FullRef(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FullReference>,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_LevelRef/c15:levelRef"))]
  C15LevelRef(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::LevelReference,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FormulaRef/c15:formulaRef"))]
  C15FormulaRef(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FormulaReference,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum StrDataExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c:CT_Boolean/c15:autoCat"))]
  C15AutoCat(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::AutoGeneneratedCategories,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum StrRefExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FullRef/c15:fullRef"))]
  C15FullRef(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FullReference>,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_LevelRef/c15:levelRef"))]
  C15LevelRef(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::LevelReference,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FormulaRef/c15:formulaRef"))]
  C15FormulaRef(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FormulaReference,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MultiLvlStrRefExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FullRef/c15:fullRef"))]
  C15FullRef(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FullReference>,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_LevelRef/c15:levelRef"))]
  C15LevelRef(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::LevelReference,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FormulaRef/c15:formulaRef"))]
  C15FormulaRef(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FormulaReference,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DLblsExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c:CT_Tx/c15:tx"))]
  C15Tx(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ChartText>,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_DataLabelFieldTable/c15:dlblFieldTable"))]
  C15DlblFieldTable(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelFieldTable,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c:CT_Boolean/c15:showDataLabelsRange"))]
  C15ShowDataLabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShowDataLabelsRange,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "a:CT_ShapeProperties/c15:spPr"))]
  C15SpPr(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShapeProperties,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c:CT_Layout/c15:layout"))]
  C15Layout(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::Layout>,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c:CT_Boolean/c15:showLeaderLines"))]
  C15ShowLeaderLines(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShowLeaderLines,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c:CT_ChartLines/c15:leaderLines"))]
  C15LeaderLines(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::LeaderLines>,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LineChartExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredLineSer/c15:filteredLineSeries"))]
  C15FilteredLineSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredLineSeriesExtension,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Line3DChartExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredLineSer/c15:filteredLineSeries"))]
  C15FilteredLineSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredLineSeriesExtension,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ScatterChartExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredScatterSer/c15:filteredScatterSeries"))]
  C15FilteredScatterSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredScatterSeries,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RadarChartExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredRadarSer/c15:filteredRadarSeries"))]
  C15FilteredRadarSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredRadarSeries,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BarChartExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredBarSer/c15:filteredBarSeries"))]
  C15FilteredBarSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredBarSeries,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Bar3DChartExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredBarSer/c15:filteredBarSeries"))]
  C15FilteredBarSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredBarSeries,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AreaChartExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredAreaSer/c15:filteredAreaSeries"))]
  C15FilteredAreaSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredAreaSeries,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Area3DChartExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredAreaSer/c15:filteredAreaSeries"))]
  C15FilteredAreaSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredAreaSeries,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BubbleChartExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredBubbleSer/c15:filteredBubbleSeries"))]
  C15FilteredBubbleSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredBubbleSeries,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SurfaceChartExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredSurfaceSer/c15:filteredSurfaceSeries"))]
  C15FilteredSurfaceSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSurfaceSeries,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Surface3DChartExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredSurfaceSer/c15:filteredSurfaceSeries"))]
  C15FilteredSurfaceSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSurfaceSeries,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CatAxExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c:CT_NumFmt/c15:numFmt"))]
  C15NumFmt(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::NumberingFormat,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DateAxExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c:CT_NumFmt/c15:numFmt"))]
  C15NumFmt(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::NumberingFormat,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SerAxExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c:CT_NumFmt/c15:numFmt"))]
  C15NumFmt(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::NumberingFormat,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ValAxExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c:CT_NumFmt/c15:numFmt"))]
  C15NumFmt(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::NumberingFormat,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DisplayUnitsChoice {
  #[sdk(child(qname = "c:CT_Double/c:custUnit"))]
  CCustUnit(std::boxed::Box<CustomDisplayUnit>),
  #[sdk(child(qname = "c:CT_BuiltInUnit/c:builtInUnit"))]
  CBuiltInUnit(std::boxed::Box<BuiltInUnit>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DLblExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_DataLabelFieldTable/c15:dlblFieldTable"))]
  C15DlblFieldTable(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelFieldTable,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c:CT_Boolean/c15:xForSave"))]
  C15XForSave(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ExceptionForSave,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c:CT_Boolean/c15:showDataLabelsRange"))]
  C15ShowDataLabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShowDataLabelsRange,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "a:CT_ShapeProperties/c15:spPr"))]
  C15SpPr(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShapeProperties,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c:CT_Layout/c15:layout"))]
  C15Layout(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::Layout>,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CategoryAxisDataChoice {
  /// Multi Level String Reference.
  #[sdk(child(qname = "c:CT_MultiLvlStrRef/c:multiLvlStrRef"))]
  CMultiLvlStrRef(std::boxed::Box<MultiLevelStringReference>),
  /// Number Reference.
  #[sdk(child(qname = "c:CT_NumRef/c:numRef"))]
  CNumRef(std::boxed::Box<NumberReference>),
  /// Number Literal.
  #[sdk(child(qname = "c:CT_NumData/c:numLit"))]
  CNumLit(std::boxed::Box<NumberLiteral>),
  /// Defines the StringReference Class.
  #[sdk(child(qname = "c:CT_StrRef/c:strRef"))]
  CStrRef(std::boxed::Box<StringReference>),
  /// String Literal.
  #[sdk(child(qname = "c:CT_StrData/c:strLit"))]
  CStrLit(std::boxed::Box<StringLiteral>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum XValuesChoice {
  /// Multi Level String Reference.
  #[sdk(child(qname = "c:CT_MultiLvlStrRef/c:multiLvlStrRef"))]
  CMultiLvlStrRef(std::boxed::Box<MultiLevelStringReference>),
  /// Number Reference.
  #[sdk(child(qname = "c:CT_NumRef/c:numRef"))]
  CNumRef(std::boxed::Box<NumberReference>),
  /// Number Literal.
  #[sdk(child(qname = "c:CT_NumData/c:numLit"))]
  CNumLit(std::boxed::Box<NumberLiteral>),
  /// Defines the StringReference Class.
  #[sdk(child(qname = "c:CT_StrRef/c:strRef"))]
  CStrRef(std::boxed::Box<StringReference>),
  /// String Literal.
  #[sdk(child(qname = "c:CT_StrData/c:strLit"))]
  CStrLit(std::boxed::Box<StringLiteral>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AxisDataSourceTypeChoice {
  /// Multi Level String Reference.
  #[sdk(child(qname = "c:CT_MultiLvlStrRef/c:multiLvlStrRef"))]
  CMultiLvlStrRef(std::boxed::Box<MultiLevelStringReference>),
  /// Number Reference.
  #[sdk(child(qname = "c:CT_NumRef/c:numRef"))]
  CNumRef(std::boxed::Box<NumberReference>),
  /// Number Literal.
  #[sdk(child(qname = "c:CT_NumData/c:numLit"))]
  CNumLit(std::boxed::Box<NumberLiteral>),
  /// Defines the StringReference Class.
  #[sdk(child(qname = "c:CT_StrRef/c:strRef"))]
  CStrRef(std::boxed::Box<StringReference>),
  /// String Literal.
  #[sdk(child(qname = "c:CT_StrData/c:strLit"))]
  CStrLit(std::boxed::Box<StringLiteral>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LineSerExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle"))]
  C15FilteredSeriesTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle"))]
  C15FilteredCategoryTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange"))]
  C15DatalabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions"))]
  C15CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"))]
  C16CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap"))]
  C16Datapointuniqueidmap(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ScatterSerExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle"))]
  C15FilteredSeriesTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle"))]
  C15FilteredCategoryTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange"))]
  C15DatalabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions"))]
  C15CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"))]
  C16CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap"))]
  C16Datapointuniqueidmap(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RadarSerExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle"))]
  C15FilteredSeriesTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle"))]
  C15FilteredCategoryTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange"))]
  C15DatalabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions"))]
  C15CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"))]
  C16CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap"))]
  C16Datapointuniqueidmap(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BarSerExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c14:CT_InvertSolidFillFmt/c14:invertSolidFillFmt"))]
  C14InvertSolidFillFmt(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::InvertSolidFillFormat,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle"))]
  C15FilteredSeriesTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle"))]
  C15FilteredCategoryTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange"))]
  C15DatalabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions"))]
  C15CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"))]
  C16CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap"))]
  C16Datapointuniqueidmap(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AreaSerExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle"))]
  C15FilteredSeriesTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle"))]
  C15FilteredCategoryTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange"))]
  C15DatalabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions"))]
  C15CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"))]
  C16CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap"))]
  C16Datapointuniqueidmap(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PieSerExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle"))]
  C15FilteredSeriesTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle"))]
  C15FilteredCategoryTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange"))]
  C15DatalabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions"))]
  C15CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"))]
  C16CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap"))]
  C16Datapointuniqueidmap(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BubbleSerExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c14:CT_InvertSolidFillFmt/c14:invertSolidFillFmt"))]
  C14InvertSolidFillFmt(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::InvertSolidFillFormat,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle"))]
  C15FilteredCategoryTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange"))]
  C15DatalabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions"))]
  C15CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"))]
  C16CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap"))]
  C16Datapointuniqueidmap(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SurfaceSerExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle"))]
  C15FilteredSeriesTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle"))]
  C15FilteredCategoryTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions"))]
  C15CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"))]
  C16CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap"))]
  C16Datapointuniqueidmap(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SurfaceTypeChoice {
  /// Thickness
  #[sdk(child(qname = "c:CT_WallThickness/c:thickness"))]
  CThickness(std::boxed::Box<Thickness>),
  #[sdk(child(qname = "a:CT_ShapeProperties/c:spPr"))]
  CSpPr(std::boxed::Box<ShapeProperties>),
  /// Picture Options
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  CPictureOptions(std::boxed::Box<PictureOptions>),
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  CExtLst(std::boxed::Box<ExtensionList>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PlotAreaChoice {
  #[sdk(child(qname = "c:CT_AreaChart/c:areaChart"))]
  CAreaChart(std::boxed::Box<AreaChart>),
  #[sdk(child(qname = "c:CT_Area3DChart/c:area3DChart"))]
  CArea3DChart(std::boxed::Box<Area3DChart>),
  #[sdk(child(qname = "c:CT_LineChart/c:lineChart"))]
  CLineChart(std::boxed::Box<LineChart>),
  #[sdk(child(qname = "c:CT_Line3DChart/c:line3DChart"))]
  CLine3DChart(std::boxed::Box<Line3DChart>),
  #[sdk(child(qname = "c:CT_StockChart/c:stockChart"))]
  CStockChart(std::boxed::Box<StockChart>),
  #[sdk(child(qname = "c:CT_RadarChart/c:radarChart"))]
  CRadarChart(std::boxed::Box<RadarChart>),
  #[sdk(child(qname = "c:CT_ScatterChart/c:scatterChart"))]
  CScatterChart(std::boxed::Box<ScatterChart>),
  #[sdk(child(qname = "c:CT_PieChart/c:pieChart"))]
  CPieChart(std::boxed::Box<PieChart>),
  #[sdk(child(qname = "c:CT_Pie3DChart/c:pie3DChart"))]
  CPie3DChart(std::boxed::Box<Pie3DChart>),
  #[sdk(child(qname = "c:CT_DoughnutChart/c:doughnutChart"))]
  CDoughnutChart(std::boxed::Box<DoughnutChart>),
  #[sdk(child(qname = "c:CT_BarChart/c:barChart"))]
  CBarChart(std::boxed::Box<BarChart>),
  #[sdk(child(qname = "c:CT_Bar3DChart/c:bar3DChart"))]
  CBar3DChart(std::boxed::Box<Bar3DChart>),
  #[sdk(child(qname = "c:CT_OfPieChart/c:ofPieChart"))]
  COfPieChart(std::boxed::Box<OfPieChart>),
  #[sdk(child(qname = "c:CT_SurfaceChart/c:surfaceChart"))]
  CSurfaceChart(std::boxed::Box<SurfaceChart>),
  #[sdk(child(qname = "c:CT_Surface3DChart/c:surface3DChart"))]
  CSurface3DChart(std::boxed::Box<Surface3DChart>),
  #[sdk(child(qname = "c:CT_BubbleChart/c:bubbleChart"))]
  CBubbleChart(std::boxed::Box<BubbleChart>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PlotAreaChoice2 {
  #[sdk(child(qname = "c:CT_ValAx/c:valAx"))]
  CValAx(std::boxed::Box<ValueAxis>),
  #[sdk(child(qname = "c:CT_CatAx/c:catAx"))]
  CCatAx(std::boxed::Box<CategoryAxis>),
  #[sdk(child(qname = "c:CT_DateAx/c:dateAx"))]
  CDateAx(std::boxed::Box<DateAxis>),
  #[sdk(child(qname = "c:CT_SerAx/c:serAx"))]
  CSerAx(std::boxed::Box<SeriesAxis>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ChartSpaceExtensionChoice {
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c14:CT_PivotOptions/c14:pivotOptions"))]
  C14PivotOptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::PivotOptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c14:CT_SketchOptions/c14:sketchOptions"))]
  C14SketchOptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::SketchOptions,
    >,
  ),
  #[cfg(feature = "microsoft365")]
  #[sdk(child(qname = "c:CT_PivotSource/c15:pivotSource"))]
  C15PivotSource(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::PivotSource>,
  ),
  #[sdk(any)]
  UnknownXml(String),
}
