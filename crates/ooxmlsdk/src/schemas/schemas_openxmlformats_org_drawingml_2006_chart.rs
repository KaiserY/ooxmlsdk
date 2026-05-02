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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumFmt/c:numFmt")]
pub struct NumberingFormat {
  /// Number Format Code
  #[sdk(attr(qname = ":formatCode"))]
  pub format_code: crate::simple_type::StringValue,
  /// Linked to Source
  #[sdk(attr(qname = ":sourceLinked"))]
  pub source_linked: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ChartShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ChartShapeProperties/c:spPr")]
pub struct ChartShapeProperties {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
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
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub a_ln: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline>,
  >,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub chart_shape_properties_choice3: Option<ChartShapePropertiesChoice3>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  /// Apply 3D shape properties.
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  pub a_sp3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType>,
  >,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the TextProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBody/c:txPr")]
pub struct TextProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Body Properties
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  pub body_properties:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BodyProperties>,
  /// Text List Styles
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  pub list_style: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ListStyle>,
  >,
  /// Text Paragraphs.
  #[sdk(child(qname = "a:CT_TextParagraph/a:p"))]
  pub a_p: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Paragraph>,
}
/// Rich Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBody/c:rich")]
pub struct RichText {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Body Properties
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  pub body_properties:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BodyProperties>,
  /// Text List Styles
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  pub list_style: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ListStyle>,
  >,
  /// Text Paragraphs.
  #[sdk(child(qname = "a:CT_TextParagraph/a:p"))]
  pub a_p: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Paragraph>,
}
/// Data Label Position.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DLblPos/c:dLblPos")]
pub struct DataLabelPosition {
  /// Data Label Position Value
  #[sdk(attr(qname = ":val"))]
  pub val: DataLabelPositionValues,
}
/// Show Legend Key.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showLegendKey")]
pub struct ShowLegendKey {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showVal")]
pub struct ShowValue {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Category Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showCatName")]
pub struct ShowCategoryName {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Series Name.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showSerName")]
pub struct ShowSeriesName {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Percent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showPercent")]
pub struct ShowPercent {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Bubble Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showBubbleSize")]
pub struct ShowBubbleSize {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Leader Lines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showLeaderLines")]
pub struct ShowLeaderLines {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the VaryColors Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:varyColors")]
pub struct VaryColors {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Wireframe.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:wireframe")]
pub struct Wireframe {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Delete.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:delete")]
pub struct Delete {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Overlay.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:overlay")]
pub struct Overlay {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Right Angle Axes.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:rAngAx")]
pub struct RightAngleAxes {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Horizontal Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showHorzBorder")]
pub struct ShowHorizontalBorder {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Vertical Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showVertBorder")]
pub struct ShowVerticalBorder {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Outline Border.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showOutline")]
pub struct ShowOutlineBorder {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Show Legend Keys.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showKeys")]
pub struct ShowKeys {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Invert if Negative.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:invertIfNegative")]
pub struct InvertIfNegative {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// 3D Bubble.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:bubble3D")]
pub struct Bubble3D {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Display R Squared Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:dispRSqr")]
pub struct DisplayRSquaredValue {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Display Equation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:dispEq")]
pub struct DisplayEquation {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// No End Cap.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:noEndCap")]
pub struct NoEndCap {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Apply To Front.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:applyToFront")]
pub struct ApplyToFront {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Apply To Sides.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:applyToSides")]
pub struct ApplyToSides {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Apply to End.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:applyToEnd")]
pub struct ApplyToEnd {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Chart Object.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:chartObject")]
pub struct ChartObject {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Data Cannot Be Changed.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:data")]
pub struct Data {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Formatting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:formatting")]
pub struct Formatting {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Selection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:selection")]
pub struct Selection {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// User Interface.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:userInterface")]
pub struct UserInterface {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Update Automatically.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:autoUpdate")]
pub struct AutoUpdate {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ShowMarker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:marker")]
pub struct ShowMarker {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Smooth Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:smooth")]
pub struct Smooth {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ShowNegativeBubbles Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showNegBubbles")]
pub struct ShowNegativeBubbles {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the AutoLabeled Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:auto")]
pub struct AutoLabeled {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the NoMultiLevelLabels Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:noMultiLvlLbl")]
pub struct NoMultiLevelLabels {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// True if the chart automatic title has been deleted..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:autoTitleDeleted")]
pub struct AutoTitleDeleted {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// True if only visible cells are plotted..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:plotVisOnly")]
pub struct PlotVisibleOnly {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// True if we should render datalabels over the maximum scale.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:showDLblsOverMax")]
pub struct ShowDataLabelsOverMaximum {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Date1904 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:date1904")]
pub struct Date1904 {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the RoundedCorners Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/c:roundedCorners")]
pub struct RoundedCorners {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Separator.
pub type Separator = crate::simple_type::StringValue;
/// Trendline Name.
pub type TrendlineName = crate::simple_type::StringValue;
/// Defines the Formula Class.
pub type Formula = crate::simple_type::StringValue;
/// Layout.
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Tx/c:tx")]
pub struct ChartText {
  #[sdk(choice(
    qname = "c:CT_StrRef/c:strRef",
    qname = "a:CT_TextBody/c:rich",
    qname = "c:CT_StrData/c:strLit"
  ))]
  pub chart_text_choice: Option<ChartTextChoice>,
}
/// Leader Lines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartLines/c:leaderLines")]
pub struct LeaderLines {
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Drop Lines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartLines/c:dropLines")]
pub struct DropLines {
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Major Gridlines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartLines/c:majorGridlines")]
pub struct MajorGridlines {
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Minor Gridlines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartLines/c:minorGridlines")]
pub struct MinorGridlines {
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Defines the SeriesLines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartLines/c:serLines")]
pub struct SeriesLines {
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Defines the HighLowLines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartLines/c:hiLowLines")]
pub struct HighLowLines {
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Index.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/c:idx")]
pub struct Index {
  /// Integer Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Order.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/c:order")]
pub struct Order {
  /// Integer Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Axis ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/c:axId")]
pub struct AxisId {
  /// Integer Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Crossing Axis ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/c:crossAx")]
pub struct CrossingAxis {
  /// Integer Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Point Count.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/c:ptCount")]
pub struct PointCount {
  /// Integer Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Second Pie Point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/c:secondPiePt")]
pub struct SecondPiePoint {
  /// Integer Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Explosion.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/c:explosion")]
pub struct Explosion {
  /// Integer Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Format ID.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/c:fmtId")]
pub struct FormatId {
  /// Integer Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Series Text.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SerTx/c:tx")]
pub struct SeriesText {
  #[sdk(choice(qname = "c:CT_StrRef/c:strRef", qname = "c:ST_Xstring/c:v"))]
  pub series_text_choice: Option<SeriesTextChoice>,
}
/// Grouping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Grouping/c:grouping")]
pub struct Grouping {
  /// Grouping Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<GroupingValues>,
}
/// Defines the LineChartSeries Class.
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
  /// Marker.
  #[sdk(child(qname = "c:CT_Marker/c:marker"))]
  pub marker: Option<std::boxed::Box<Marker>>,
  /// Defines the PictureOptions Class.
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// Defines the DataPoint Class.
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<DataPoint>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Defines the Trendline Class.
  #[sdk(child(qname = "c:CT_Trendline/c:trendline"))]
  pub c_trendline: Vec<Trendline>,
  /// Defines the ErrorBars Class.
  #[sdk(child(qname = "c:CT_ErrBars/c:errBars"))]
  pub c_err_bars: Option<std::boxed::Box<ErrorBars>>,
  /// Defines the CategoryAxisData Class.
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub c_cat: Option<std::boxed::Box<CategoryAxisData>>,
  /// Defines the Values Class.
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub c_val: Option<std::boxed::Box<Values>>,
  /// Defines the Smooth Class.
  #[sdk(child(qname = "c:CT_Boolean/c:smooth"))]
  pub c_smooth: Option<Smooth>,
  /// Defines the LineSerExtensionList Class.
  #[sdk(child(qname = "c:CT_LineSerExtensionList/c:extLst"))]
  pub c_ext_lst: Option<LineSerExtensionList>,
}
/// Data Labels.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DLbls/c:dLbls")]
pub struct DataLabels {
  /// Data Label.
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
  /// Defines the DLblsExtensionList Class.
  #[sdk(child(qname = "c:CT_DLblsExtensionList/c:extLst"))]
  pub c_ext_lst: Option<DLblsExtensionList>,
}
/// Bar Direction.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BarDir/c:barDir")]
pub struct BarDirection {
  /// Bar Direction Value
  #[sdk(attr(qname = ":val"))]
  pub val: BarDirectionValues,
}
/// Bar Grouping.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BarGrouping/c:grouping")]
pub struct BarGrouping {
  /// Bar Grouping Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<BarGroupingValues>,
}
/// Bar Chart Series.
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
  /// Invert if Negative.
  #[sdk(child(qname = "c:CT_Boolean/c:invertIfNegative"))]
  pub invert_if_negative: Option<InvertIfNegative>,
  /// Defines the PictureOptions Class.
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// Defines the DataPoint Class.
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<DataPoint>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Defines the Trendline Class.
  #[sdk(child(qname = "c:CT_Trendline/c:trendline"))]
  pub c_trendline: Vec<Trendline>,
  /// Defines the ErrorBars Class.
  #[sdk(child(qname = "c:CT_ErrBars/c:errBars"))]
  pub c_err_bars: Option<std::boxed::Box<ErrorBars>>,
  /// Defines the CategoryAxisData Class.
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub c_cat: Option<std::boxed::Box<CategoryAxisData>>,
  /// Defines the Values Class.
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub c_val: Option<std::boxed::Box<Values>>,
  /// Defines the Shape Class.
  #[sdk(child(qname = "c:CT_Shape/c:shape"))]
  pub c_shape: Option<Shape>,
  /// Defines the BarSerExtensionList Class.
  #[sdk(child(qname = "c:CT_BarSerExtensionList/c:extLst"))]
  pub c_ext_lst: Option<BarSerExtensionList>,
}
/// Area Chart Series.
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
  /// Defines the PictureOptions Class.
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// Defines the DataPoint Class.
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<DataPoint>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Defines the Trendline Class.
  #[sdk(child(qname = "c:CT_Trendline/c:trendline"))]
  pub c_trendline: Vec<Trendline>,
  /// Defines the ErrorBars Class.
  #[sdk(child(qname = "c:CT_ErrBars/c:errBars"))]
  pub c_err_bars: Vec<ErrorBars>,
  /// Defines the CategoryAxisData Class.
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub c_cat: Option<std::boxed::Box<CategoryAxisData>>,
  /// Defines the Values Class.
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub c_val: Option<std::boxed::Box<Values>>,
  /// Defines the AreaSerExtensionList Class.
  #[sdk(child(qname = "c:CT_AreaSerExtensionList/c:extLst"))]
  pub c_ext_lst: Option<AreaSerExtensionList>,
}
/// Pie Chart Series.
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
  /// Defines the PictureOptions Class.
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// Explosion.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:explosion"))]
  pub explosion: Option<Explosion>,
  /// Defines the DataPoint Class.
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<DataPoint>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Defines the CategoryAxisData Class.
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub c_cat: Option<std::boxed::Box<CategoryAxisData>>,
  /// Defines the Values Class.
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub c_val: Option<std::boxed::Box<Values>>,
  /// Defines the PieSerExtensionList Class.
  #[sdk(child(qname = "c:CT_PieSerExtensionList/c:extLst"))]
  pub c_ext_lst: Option<PieSerExtensionList>,
}
/// Surface Chart Series.
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
  /// Defines the PictureOptions Class.
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// Defines the CategoryAxisData Class.
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub category_axis_data: Option<std::boxed::Box<CategoryAxisData>>,
  /// Defines the Values Class.
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub values: Option<std::boxed::Box<Values>>,
  /// 3D Bubble.
  #[sdk(child(qname = "c:CT_Boolean/c:bubble3D"))]
  pub bubble3_d: Option<Bubble3D>,
  /// Defines the SurfaceSerExtensionList Class.
  #[sdk(child(qname = "c:CT_SurfaceSerExtensionList/c:extLst"))]
  pub surface_ser_extension_list: Option<SurfaceSerExtensionList>,
}
/// Band Formats.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BandFmts/c:bandFmts")]
pub struct BandFormats {
  /// Band Format.
  #[sdk(child(qname = "c:CT_BandFmt/c:bandFmt"))]
  pub c_band_fmt: Vec<BandFormat>,
}
/// Scaling.
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AxPos/c:axPos")]
pub struct AxisPosition {
  /// Axis Position Value
  #[sdk(attr(qname = ":val"))]
  pub val: AxisPositionValues,
}
/// Title.
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
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// Defines the TextProperties Class.
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Major Tick Mark.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TickMark/c:majorTickMark")]
pub struct MajorTickMark {
  /// Tick Mark Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<TickMarkValues>,
}
/// Minor Tick Mark.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TickMark/c:minorTickMark")]
pub struct MinorTickMark {
  /// Tick Mark Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<TickMarkValues>,
}
/// Tick Label Position.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TickLblPos/c:tickLblPos")]
pub struct TickLabelPosition {
  /// Tick Label Position Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<TickLabelPositionValues>,
}
/// Crosses.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Crosses/c:crosses")]
pub struct Crosses {
  /// Crosses Value
  #[sdk(attr(qname = ":val"))]
  pub val: CrossesValues,
}
/// Crossing Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:crossesAt")]
pub struct CrossesAt {
  /// Floating Point Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Left.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:x")]
pub struct Left {
  /// Floating Point Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Top.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:y")]
pub struct Top {
  /// Floating Point Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Width.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:w")]
pub struct Width {
  /// Floating Point Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Height.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:h")]
pub struct Height {
  /// Floating Point Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Forward.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:forward")]
pub struct Forward {
  /// Floating Point Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Backward.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:backward")]
pub struct Backward {
  /// Floating Point Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Intercept.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:intercept")]
pub struct Intercept {
  /// Floating Point Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Error Bar Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:val")]
pub struct ErrorBarValue {
  /// Floating Point Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Split Position.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:splitPos")]
pub struct SplitPosition {
  /// Floating Point Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Custom Display Unit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:custUnit")]
pub struct CustomDisplayUnit {
  /// Floating Point Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Maximum.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:max")]
pub struct MaxAxisValue {
  /// Floating Point Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Minimum.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Double/c:min")]
pub struct MinAxisValue {
  /// Floating Point Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Chart Space.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartSpace/c:chartSpace")]
pub struct ChartSpace {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Version number of the file, as determined by the features used by this chart
  #[sdk(attr(qname = ":version"))]
  pub version: Option<crate::simple_type::StringValue>,
  /// A space-delimited list of additional features used in this chart but not captured in the version
  #[sdk(attr(qname = ":featureList"))]
  pub feature_list: Option<crate::simple_type::StringValue>,
  /// A reference (in the form of a relationship ID) to a fallback image to be used if the chart cannot be loaded
  #[sdk(attr(qname = ":fallbackImg"))]
  pub fallback_img: Option<crate::simple_type::StringValue>,
  /// Defines the Date1904 Class.
  #[sdk(child(qname = "c:CT_Boolean/c:date1904"))]
  pub date1904: Option<Date1904>,
  /// Defines the EditingLanguage Class.
  #[sdk(child(qname = "c:CT_TextLanguageID/c:lang"))]
  pub editing_language: Option<EditingLanguage>,
  /// Defines the RoundedCorners Class.
  #[sdk(child(qname = "c:CT_Boolean/c:roundedCorners"))]
  pub rounded_corners: Option<RoundedCorners>,
  #[sdk(choice(qname = "c14:CT_Style/c14:style", qname = "c:CT_Style/c:style"))]
  pub chart_space_choice: Option<ChartSpaceChoice>,
  /// Defines the ColorMapOverride Class.
  #[sdk(child(qname = "a:CT_ColorMapping/c:clrMapOvr"))]
  pub c_clr_map_ovr: Option<std::boxed::Box<ColorMapOverride>>,
  /// Defines the PivotSource Class.
  #[sdk(child(qname = "c:CT_PivotSource/c:pivotSource"))]
  pub c_pivot_source: Option<std::boxed::Box<PivotSource>>,
  /// Defines the Protection Class.
  #[sdk(child(qname = "c:CT_Protection/c:protection"))]
  pub c_protection: Option<std::boxed::Box<Protection>>,
  /// Defines the Chart Class.
  #[sdk(child(qname = "c:CT_Chart/c:chart"))]
  pub c_chart: Option<std::boxed::Box<Chart>>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "a:CT_ShapeProperties/c:spPr"))]
  pub c_sp_pr: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextProperties Class.
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub c_tx_pr: Option<std::boxed::Box<TextProperties>>,
  /// Defines the ExternalData Class.
  #[sdk(child(qname = "c:CT_ExternalData/c:externalData"))]
  pub c_external_data: Option<std::boxed::Box<ExternalData>>,
  /// Defines the PrintSettings Class.
  #[sdk(child(qname = "c:CT_PrintSettings/c:printSettings"))]
  pub c_print_settings: Option<std::boxed::Box<PrintSettings>>,
  /// Defines the UserShapesReference Class.
  #[sdk(child(qname = "c:CT_RelId/c:userShapes"))]
  pub c_user_shapes: Option<UserShapesReference>,
  /// Defines the ChartSpaceExtensionList Class.
  #[sdk(child(qname = "c:CT_ChartSpaceExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ChartSpaceExtensionList>,
}
/// User Shapes.
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RelId/c:chart")]
pub struct ChartReference {
  /// Relationship Reference
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Legacy Drawing for Headers and Footers.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RelId/c:legacyDrawingHF")]
pub struct LegacyDrawingHeaderFooter {
  /// Relationship Reference
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Defines the UserShapesReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RelId/c:userShapes")]
pub struct UserShapesReference {
  /// Relationship Reference
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
}
/// Extension.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Extension/c:ext")]
pub struct Extension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Uniform Resource Identifier
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Numeric Value.
pub type NumericValue = crate::simple_type::StringValue;
/// Format Code.
pub type FormatCode = crate::simple_type::StringValue;
/// Odd Header.
pub type OddHeader = crate::simple_type::StringValue;
/// Odd Footer.
pub type OddFooter = crate::simple_type::StringValue;
/// Even Header.
pub type EvenHeader = crate::simple_type::StringValue;
/// Even Footer.
pub type EvenFooter = crate::simple_type::StringValue;
/// First Header.
pub type FirstHeader = crate::simple_type::StringValue;
/// First Footer.
pub type FirstFooter = crate::simple_type::StringValue;
/// Pivot Name.
pub type PivotTableName = crate::simple_type::StringValue;
/// Numeric Point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumVal/c:pt")]
pub struct NumericPoint {
  /// Index
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  /// Number Format
  #[sdk(attr(qname = ":formatCode"))]
  pub format_code: Option<crate::simple_type::StringValue>,
  /// Numeric Value
  #[sdk(text_child(qname = "c:ST_Xstring/c:v"))]
  pub numeric_value: crate::simple_type::StringValue,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ExtensionList/c:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "c:CT_Extension/c:ext"))]
  pub c_ext: Vec<Extension>,
}
/// Number Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumRef/c:numRef")]
pub struct NumberReference {
  /// Defines the Formula Class.
  #[sdk(text_child(qname = "xsd:string/c:f"))]
  pub formula: crate::simple_type::StringValue,
  /// Defines the NumberingCache Class.
  #[sdk(child(qname = "c:CT_NumData/c:numCache"))]
  pub numbering_cache: Option<std::boxed::Box<NumberingCache>>,
  /// Defines the NumRefExtensionList Class.
  #[sdk(child(qname = "c:CT_NumRefExtensionList/c:extLst"))]
  pub num_ref_extension_list: Option<NumRefExtensionList>,
}
/// Number Literal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumData/c:numLit")]
pub struct NumberLiteral {
  /// Format Code
  #[sdk(text_child(qname = "c:ST_Xstring/c:formatCode"))]
  pub format_code: Option<crate::simple_type::StringValue>,
  /// Point Count
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  pub point_count: Option<PointCount>,
  /// Numeric Point.
  #[sdk(child(qname = "c:CT_NumVal/c:pt"))]
  pub c_pt: Vec<NumericPoint>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// Defines the NumberingCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumData/c:numCache")]
pub struct NumberingCache {
  /// Format Code
  #[sdk(text_child(qname = "c:ST_Xstring/c:formatCode"))]
  pub format_code: Option<crate::simple_type::StringValue>,
  /// Point Count
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  pub point_count: Option<PointCount>,
  /// Numeric Point.
  #[sdk(child(qname = "c:CT_NumVal/c:pt"))]
  pub c_pt: Vec<NumericPoint>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// Level.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Lvl/c:lvl")]
pub struct Level {
  /// String Point.
  #[sdk(child(qname = "c:CT_StrVal/c:pt"))]
  pub c_pt: Vec<StringPoint>,
}
/// Multi Level String Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_MultiLvlStrRef/c:multiLvlStrRef")]
pub struct MultiLevelStringReference {
  /// Defines the Formula Class.
  #[sdk(text_child(qname = "xsd:string/c:f"))]
  pub formula: crate::simple_type::StringValue,
  /// Defines the MultiLevelStringCache Class.
  #[sdk(child(qname = "c:CT_MultiLvlStrData/c:multiLvlStrCache"))]
  pub multi_level_string_cache: Option<std::boxed::Box<MultiLevelStringCache>>,
  /// Defines the MultiLvlStrRefExtensionList Class.
  #[sdk(child(qname = "c:CT_MultiLvlStrRefExtensionList/c:extLst"))]
  pub multi_lvl_str_ref_extension_list: Option<MultiLvlStrRefExtensionList>,
}
/// Defines the StringReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StrRef/c:strRef")]
pub struct StringReference {
  /// Defines the Formula Class.
  #[sdk(text_child(qname = "xsd:string/c:f"))]
  pub formula: crate::simple_type::StringValue,
  /// Defines the StringCache Class.
  #[sdk(child(qname = "c:CT_StrData/c:strCache"))]
  pub string_cache: Option<std::boxed::Box<StringCache>>,
  /// Defines the StrRefExtensionList Class.
  #[sdk(child(qname = "c:CT_StrRefExtensionList/c:extLst"))]
  pub str_ref_extension_list: Option<StrRefExtensionList>,
}
/// String Literal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StrData/c:strLit")]
pub struct StringLiteral {
  /// Point Count.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  pub point_count: Option<PointCount>,
  /// String Point.
  #[sdk(child(qname = "c:CT_StrVal/c:pt"))]
  pub c_pt: Vec<StringPoint>,
  /// Defines the StrDataExtensionList Class.
  #[sdk(child(qname = "c:CT_StrDataExtensionList/c:extLst"))]
  pub c_ext_lst: Option<StrDataExtensionList>,
}
/// Defines the StringCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StrData/c:strCache")]
pub struct StringCache {
  /// Point Count.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  pub point_count: Option<PointCount>,
  /// String Point.
  #[sdk(child(qname = "c:CT_StrVal/c:pt"))]
  pub c_pt: Vec<StringPoint>,
  /// Defines the StrDataExtensionList Class.
  #[sdk(child(qname = "c:CT_StrDataExtensionList/c:extLst"))]
  pub c_ext_lst: Option<StrDataExtensionList>,
}
/// Layout Target.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LayoutTarget/c:layoutTarget")]
pub struct LayoutTarget {
  /// Layout Target Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<LayoutTargetValues>,
}
/// Left Mode.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LayoutMode/c:xMode")]
pub struct LeftMode {
  /// Layout Mode Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<LayoutModeValues>,
}
/// Top Mode.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LayoutMode/c:yMode")]
pub struct TopMode {
  /// Layout Mode Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<LayoutModeValues>,
}
/// Width Mode.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LayoutMode/c:wMode")]
pub struct WidthMode {
  /// Layout Mode Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<LayoutModeValues>,
}
/// Height Mode.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LayoutMode/c:hMode")]
pub struct HeightMode {
  /// Layout Mode Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<LayoutModeValues>,
}
/// Manual Layout.
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RotX/c:rotX")]
pub struct RotateX {
  /// X Rotation Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = -90..= 90))]
  pub val: Option<crate::simple_type::SByteValue>,
}
/// Height Percent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_HPercent/c:hPercent")]
pub struct HeightPercent {
  /// Height Percent Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 5..= 500))]
  pub val: Option<crate::simple_type::UInt16Value>,
}
/// Y Rotation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RotY/c:rotY")]
pub struct RotateY {
  /// Y Rotation Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..= 360))]
  pub val: Option<crate::simple_type::UInt16Value>,
}
/// Depth Percent.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DepthPercent/c:depthPercent")]
pub struct DepthPercent {
  /// Depth Percent Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 20..= 2000))]
  pub val: Option<crate::simple_type::UInt16Value>,
}
/// Perspective.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Perspective/c:perspective")]
pub struct Perspective {
  /// Perspective Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..= 240))]
  pub val: Option<crate::simple_type::ByteValue>,
}
/// Symbol.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_MarkerStyle/c:symbol")]
pub struct Symbol {
  /// Marker Style Value
  #[sdk(attr(qname = ":val"))]
  pub val: MarkerStyleValues,
}
/// Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_MarkerSize/c:size")]
pub struct Size {
  /// Marker Size Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 2..= 72))]
  pub val: Option<crate::simple_type::ByteValue>,
}
/// Marker.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Marker/c:marker")]
pub struct Marker {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Symbol
  #[sdk(child(qname = "c:CT_MarkerStyle/c:symbol"))]
  pub symbol: Option<Symbol>,
  /// Size
  #[sdk(child(qname = "c:CT_MarkerSize/c:size"))]
  pub size: Option<Size>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PictureOptions Class.
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TrendlineType/c:trendlineType")]
pub struct TrendlineType {
  /// Trendline Type Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<TrendlineValues>,
}
/// Polynomial Trendline Order.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Order/c:order")]
pub struct PolynomialOrder {
  /// Order Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 2..= 6))]
  pub val: crate::simple_type::ByteValue,
}
/// Period.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Period/c:period")]
pub struct Period {
  /// Period Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 2..))]
  pub val: crate::simple_type::UInt32Value,
}
/// Trendline Label.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TrendlineLbl/c:trendlineLbl")]
pub struct TrendlineLabel {
  /// Layout
  #[sdk(child(qname = "c:CT_Layout/c:layout"))]
  pub layout: Option<std::boxed::Box<Layout>>,
  /// Defines the ChartText Class.
  #[sdk(child(qname = "c:CT_Tx/c:tx"))]
  pub chart_text: Option<std::boxed::Box<ChartText>>,
  /// Number Format
  #[sdk(child(qname = "c:CT_NumFmt/c:numFmt"))]
  pub numbering_format: Option<NumberingFormat>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// Defines the TextProperties Class.
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Error Bar Direction.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ErrDir/c:errDir")]
pub struct ErrorDirection {
  /// Error Bar Direction Value
  #[sdk(attr(qname = ":val"))]
  pub val: ErrorBarDirectionValues,
}
/// Error Bar Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ErrBarType/c:errBarType")]
pub struct ErrorBarType {
  /// Error Bar Type Value
  #[sdk(attr(qname = ":val"))]
  pub val: ErrorBarValues,
}
/// Error Bar Value Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ErrValType/c:errValType")]
pub struct ErrorBarValueType {
  /// Error Bar Type Value
  #[sdk(attr(qname = ":val"))]
  pub val: ErrorValues,
}
/// Plus.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumDataSource/c:plus")]
pub struct Plus {
  #[sdk(choice(qname = "c:CT_NumRef/c:numRef", qname = "c:CT_NumData/c:numLit"))]
  pub plus_choice: Option<PlusChoice>,
}
/// Minus.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumDataSource/c:minus")]
pub struct Minus {
  #[sdk(choice(qname = "c:CT_NumRef/c:numRef", qname = "c:CT_NumData/c:numLit"))]
  pub minus_choice: Option<MinusChoice>,
}
/// Defines the Values Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumDataSource/c:val")]
pub struct Values {
  #[sdk(choice(qname = "c:CT_NumRef/c:numRef", qname = "c:CT_NumData/c:numLit"))]
  pub values_choice: Option<ValuesChoice>,
}
/// Defines the YValues Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumDataSource/c:yVal")]
pub struct YValues {
  #[sdk(choice(qname = "c:CT_NumRef/c:numRef", qname = "c:CT_NumData/c:numLit"))]
  pub y_values_choice: Option<YValuesChoice>,
}
/// Defines the BubbleSize Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumDataSource/c:bubbleSize")]
pub struct BubbleSize {
  #[sdk(choice(qname = "c:CT_NumRef/c:numRef", qname = "c:CT_NumData/c:numLit"))]
  pub bubble_size_choice: Option<BubbleSizeChoice>,
}
/// Gap Width.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_GapAmount/c:gapWidth")]
pub struct GapWidth {
  /// Gap Size Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..= 500))]
  pub val: Option<crate::simple_type::UInt16Value>,
}
/// Defines the GapDepth Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_GapAmount/c:gapDepth")]
pub struct GapDepth {
  /// Gap Size Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..= 500))]
  pub val: Option<crate::simple_type::UInt16Value>,
}
/// Up Bars.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UpDownBar/c:upBars")]
pub struct UpBars {
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Down Bars.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UpDownBar/c:downBars")]
pub struct DownBars {
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Pie of Pie or Bar of Pie Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_OfPieType/c:ofPieType")]
pub struct OfPieType {
  /// Pie of Pie or Bar of Pie Type Value
  #[sdk(attr(qname = ":val"))]
  pub val: OfPieValues,
}
/// Split Type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SplitType/c:splitType")]
pub struct SplitType {
  /// Split Type Value
  #[sdk(attr(qname = ":val"))]
  pub val: SplitValues,
}
/// Custom Split.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_CustSplit/c:custSplit")]
pub struct CustomSplit {
  /// Second Pie Point.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:secondPiePt"))]
  pub c_second_pie_pt: Vec<SecondPiePoint>,
}
/// Second Pie Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SecondPieSize/c:secondPieSize")]
pub struct SecondPieSize {
  /// Second Pie Size Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 5..= 200))]
  pub val: Option<crate::simple_type::UInt16Value>,
}
/// Band Format.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BandFmt/c:bandFmt")]
pub struct BandFormat {
  /// Index.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: std::boxed::Box<Index>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
}
/// Picture Format.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PictureFormat/c:pictureFormat")]
pub struct PictureFormat {
  /// Picture Format Value
  #[sdk(attr(qname = ":val"))]
  pub val: PictureFormatValues,
}
/// Picture Stack Unit.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PictureStackUnit/c:pictureStackUnit")]
pub struct PictureStackUnit {
  /// Picture Stack Unit
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..))]
  pub val: crate::simple_type::DoubleValue,
}
/// Built in Display Unit Value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BuiltInUnit/c:builtInUnit")]
pub struct BuiltInUnit {
  /// Built In Unit Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<BuiltInUnitValues>,
}
/// Display Units Label.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DispUnitsLbl/c:dispUnitsLbl")]
pub struct DisplayUnitsLabel {
  /// Layout
  #[sdk(child(qname = "c:CT_Layout/c:layout"))]
  pub layout: Option<std::boxed::Box<Layout>>,
  /// Defines the ChartText Class.
  #[sdk(child(qname = "c:CT_Tx/c:tx"))]
  pub chart_text: Option<std::boxed::Box<ChartText>>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// Defines the TextProperties Class.
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
}
/// Logarithmic Base.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LogBase/c:logBase")]
pub struct LogBase {
  /// Logarithmic Base Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 2..= 1000))]
  pub val: crate::simple_type::DoubleValue,
}
/// Axis Orientation.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Orientation/c:orientation")]
pub struct Orientation {
  /// Orientation Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<OrientationValues>,
}
/// Pivot Format.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PivotFmt/c:pivotFmt")]
pub struct PivotFormat {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: std::boxed::Box<Index>,
  /// Defines the ShapeProperties Class.
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LegendPos/c:legendPos")]
pub struct LegendPosition {
  /// Legend Position Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<LegendPositionValues>,
}
/// Legend Entry.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LegendEntry/c:legendEntry")]
pub struct LegendEntry {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: Option<Index>,
  #[sdk(choice(qname = "c:CT_Boolean/c:delete", qname = "a:CT_TextBody/c:txPr"))]
  pub legend_entry_choice: Option<LegendEntryChoice>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// Header and Footer.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_HeaderFooter/c:headerFooter")]
pub struct HeaderFooter {
  /// Align With Margins
  #[sdk(attr(qname = ":alignWithMargins"))]
  pub align_with_margins: Option<crate::simple_type::BooleanValue>,
  /// Different Odd Even
  #[sdk(attr(qname = ":differentOddEven"))]
  pub different_odd_even: Option<crate::simple_type::BooleanValue>,
  /// Different First
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PageMargins/c:pageMargins")]
pub struct PageMargins {
  /// Left
  #[sdk(attr(qname = ":l"))]
  pub left: crate::simple_type::DoubleValue,
  /// Right
  #[sdk(attr(qname = ":r"))]
  pub right: crate::simple_type::DoubleValue,
  /// Top
  #[sdk(attr(qname = ":t"))]
  pub top: crate::simple_type::DoubleValue,
  /// Bottom
  #[sdk(attr(qname = ":b"))]
  pub bottom: crate::simple_type::DoubleValue,
  /// Header
  #[sdk(attr(qname = ":header"))]
  pub header: crate::simple_type::DoubleValue,
  /// Footer
  #[sdk(attr(qname = ":footer"))]
  pub footer: crate::simple_type::DoubleValue,
}
/// Page Setup.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PageSetup/c:pageSetup")]
pub struct PageSetup {
  /// Page Size
  #[sdk(attr(qname = ":paperSize"))]
  pub paper_size: Option<crate::simple_type::UInt32Value>,
  /// First Page Number
  #[sdk(attr(qname = ":firstPageNumber"))]
  pub first_page_number: Option<crate::simple_type::Int32Value>,
  /// Orientation
  #[sdk(attr(qname = ":orientation"))]
  pub orientation: Option<PageSetupOrientationValues>,
  /// Black and White
  #[sdk(attr(qname = ":blackAndWhite"))]
  pub black_and_white: Option<crate::simple_type::BooleanValue>,
  /// Draft
  #[sdk(attr(qname = ":draft"))]
  pub draft: Option<crate::simple_type::BooleanValue>,
  /// Use First Page Number
  #[sdk(attr(qname = ":useFirstPageNumber"))]
  pub use_first_page_number: Option<crate::simple_type::BooleanValue>,
  /// Horizontal DPI
  #[sdk(attr(qname = ":horizontalDpi"))]
  pub horizontal_dpi: Option<crate::simple_type::Int32Value>,
  /// Vertical DPI
  #[sdk(attr(qname = ":verticalDpi"))]
  pub vertical_dpi: Option<crate::simple_type::Int32Value>,
  /// Copies
  #[sdk(attr(qname = ":copies"))]
  pub copies: Option<crate::simple_type::UInt32Value>,
}
/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeProperties/c:spPr")]
pub struct ShapeProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
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
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub a_ln: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline>,
  >,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  /// Apply 3D shape properties.
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  pub a_sp3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType>,
  >,
  /// Defines the ShapePropertiesExtensionList Class.
  #[sdk(child(qname = "a:CT_ShapePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapePropertiesExtensionList,
  >,
}
/// Data Label.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DLbl/c:dLbl")]
pub struct DataLabel {
  /// Index.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: Option<Index>,
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
  /// Defines the DLblExtensionList Class.
  #[sdk(child(qname = "c:CT_DLblExtensionList/c:extLst"))]
  pub c_ext_lst: Option<DLblExtensionList>,
}
/// Area Charts.
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
  pub c_ser: Vec<AreaChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Drop Lines.
  #[sdk(child(qname = "c:CT_ChartLines/c:dropLines"))]
  pub c_drop_lines: Option<std::boxed::Box<DropLines>>,
  /// Axis ID.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// Defines the AreaChartExtensionList Class.
  #[sdk(child(qname = "c:CT_AreaChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<AreaChartExtensionList>,
}
/// 3D Area Charts.
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
  pub c_ser: Vec<AreaChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Drop Lines.
  #[sdk(child(qname = "c:CT_ChartLines/c:dropLines"))]
  pub c_drop_lines: Option<std::boxed::Box<DropLines>>,
  /// Defines the GapDepth Class.
  #[sdk(child(qname = "c:CT_GapAmount/c:gapDepth"))]
  pub c_gap_depth: Option<GapDepth>,
  /// Axis ID.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// Defines the Area3DChartExtensionList Class.
  #[sdk(child(qname = "c:CT_Area3DChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<Area3DChartExtensionList>,
}
/// Line Charts.
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
  pub c_ser: Vec<LineChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Drop Lines.
  #[sdk(child(qname = "c:CT_ChartLines/c:dropLines"))]
  pub c_drop_lines: Option<std::boxed::Box<DropLines>>,
  /// Defines the HighLowLines Class.
  #[sdk(child(qname = "c:CT_ChartLines/c:hiLowLines"))]
  pub c_hi_low_lines: Option<std::boxed::Box<HighLowLines>>,
  /// Defines the UpDownBars Class.
  #[sdk(child(qname = "c:CT_UpDownBars/c:upDownBars"))]
  pub c_up_down_bars: Option<std::boxed::Box<UpDownBars>>,
  /// Defines the ShowMarker Class.
  #[sdk(child(qname = "c:CT_Boolean/c:marker"))]
  pub c_marker: Option<ShowMarker>,
  /// Defines the Smooth Class.
  #[sdk(child(qname = "c:CT_Boolean/c:smooth"))]
  pub c_smooth: Option<Smooth>,
  /// Axis ID.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// Defines the LineChartExtensionList Class.
  #[sdk(child(qname = "c:CT_LineChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<LineChartExtensionList>,
}
/// 3D Line Charts.
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
  pub c_ser: Vec<LineChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Drop Lines.
  #[sdk(child(qname = "c:CT_ChartLines/c:dropLines"))]
  pub c_drop_lines: Option<std::boxed::Box<DropLines>>,
  /// Defines the GapDepth Class.
  #[sdk(child(qname = "c:CT_GapAmount/c:gapDepth"))]
  pub c_gap_depth: Option<GapDepth>,
  /// Axis ID.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// Defines the Line3DChartExtensionList Class.
  #[sdk(child(qname = "c:CT_Line3DChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<Line3DChartExtensionList>,
}
/// Stock Charts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StockChart/c:stockChart")]
pub struct StockChart {
  /// Defines the LineChartSeries Class.
  #[sdk(child(qname = "c:CT_LineSer/c:ser"))]
  pub c_ser: Vec<LineChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Drop Lines.
  #[sdk(child(qname = "c:CT_ChartLines/c:dropLines"))]
  pub c_drop_lines: Option<std::boxed::Box<DropLines>>,
  /// Defines the HighLowLines Class.
  #[sdk(child(qname = "c:CT_ChartLines/c:hiLowLines"))]
  pub c_hi_low_lines: Option<std::boxed::Box<HighLowLines>>,
  /// Defines the UpDownBars Class.
  #[sdk(child(qname = "c:CT_UpDownBars/c:upDownBars"))]
  pub c_up_down_bars: Option<std::boxed::Box<UpDownBars>>,
  /// Axis ID.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// Defines the StockChartExtensionList Class.
  #[sdk(child(qname = "c:CT_StockChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<StockChartExtensionList>,
}
/// Radar Charts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RadarChart/c:radarChart")]
pub struct RadarChart {
  /// Defines the RadarStyle Class.
  #[sdk(child(qname = "c:CT_RadarStyle/c:radarStyle"))]
  pub radar_style: std::boxed::Box<RadarStyle>,
  /// Defines the VaryColors Class.
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// Defines the RadarChartSeries Class.
  #[sdk(child(qname = "c:CT_RadarSer/c:ser"))]
  pub c_ser: Vec<RadarChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Axis ID.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// Defines the RadarChartExtensionList Class.
  #[sdk(child(qname = "c:CT_RadarChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<RadarChartExtensionList>,
}
/// Scatter Charts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ScatterChart/c:scatterChart")]
pub struct ScatterChart {
  /// Defines the ScatterStyle Class.
  #[sdk(child(qname = "c:CT_ScatterStyle/c:scatterStyle"))]
  pub scatter_style: std::boxed::Box<ScatterStyle>,
  /// Defines the VaryColors Class.
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// Defines the ScatterChartSeries Class.
  #[sdk(child(qname = "c:CT_ScatterSer/c:ser"))]
  pub c_ser: Vec<ScatterChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Axis ID.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// Defines the ScatterChartExtensionList Class.
  #[sdk(child(qname = "c:CT_ScatterChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ScatterChartExtensionList>,
}
/// Pie Charts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PieChart/c:pieChart")]
pub struct PieChart {
  /// Defines the VaryColors Class.
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// Pie Chart Series.
  #[sdk(child(qname = "c:CT_PieSer/c:ser"))]
  pub c_ser: Vec<PieChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// First Slice Angle.
  #[sdk(child(qname = "c:CT_FirstSliceAng/c:firstSliceAng"))]
  pub c_first_slice_ang: Option<FirstSliceAngle>,
  /// Defines the PieChartExtensionList Class.
  #[sdk(child(qname = "c:CT_PieChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<PieChartExtensionList>,
}
/// 3D Pie Charts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Pie3DChart/c:pie3DChart")]
pub struct Pie3DChart {
  /// Defines the VaryColors Class.
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// Pie Chart Series.
  #[sdk(child(qname = "c:CT_PieSer/c:ser"))]
  pub c_ser: Vec<PieChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Defines the Pie3DChartExtensionList Class.
  #[sdk(child(qname = "c:CT_Pie3DChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<Pie3DChartExtensionList>,
}
/// Doughnut Charts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DoughnutChart/c:doughnutChart")]
pub struct DoughnutChart {
  /// Defines the VaryColors Class.
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// Pie Chart Series.
  #[sdk(child(qname = "c:CT_PieSer/c:ser"))]
  pub c_ser: Vec<PieChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// First Slice Angle.
  #[sdk(child(qname = "c:CT_FirstSliceAng/c:firstSliceAng"))]
  pub c_first_slice_ang: Option<FirstSliceAngle>,
  /// Hole Size.
  #[sdk(child(qname = "c:CT_HoleSize/c:holeSize"))]
  pub c_hole_size: std::boxed::Box<HoleSize>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// Bar Charts.
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
  pub c_ser: Vec<BarChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Gap Width.
  #[sdk(child(qname = "c:CT_GapAmount/c:gapWidth"))]
  pub c_gap_width: Option<GapWidth>,
  /// Defines the Overlap Class.
  #[sdk(child(qname = "c:CT_Overlap/c:overlap"))]
  pub c_overlap: Option<Overlap>,
  /// Defines the SeriesLines Class.
  #[sdk(child(qname = "c:CT_ChartLines/c:serLines"))]
  pub c_ser_lines: Vec<SeriesLines>,
  /// Axis ID.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// Defines the BarChartExtensionList Class.
  #[sdk(child(qname = "c:CT_BarChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<BarChartExtensionList>,
}
/// 3D Bar Charts.
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
  pub c_ser: Vec<BarChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Gap Width.
  #[sdk(child(qname = "c:CT_GapAmount/c:gapWidth"))]
  pub c_gap_width: Option<GapWidth>,
  /// Defines the GapDepth Class.
  #[sdk(child(qname = "c:CT_GapAmount/c:gapDepth"))]
  pub c_gap_depth: Option<GapDepth>,
  /// Defines the Shape Class.
  #[sdk(child(qname = "c:CT_Shape/c:shape"))]
  pub c_shape: Option<Shape>,
  /// Axis ID.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// Defines the Bar3DChartExtensionList Class.
  #[sdk(child(qname = "c:CT_Bar3DChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<Bar3DChartExtensionList>,
}
/// Pie of Pie or Bar of Pie Charts.
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
  pub c_ser: Vec<PieChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Gap Width.
  #[sdk(child(qname = "c:CT_GapAmount/c:gapWidth"))]
  pub c_gap_width: Option<GapWidth>,
  /// Split Type.
  #[sdk(child(qname = "c:CT_SplitType/c:splitType"))]
  pub c_split_type: Option<SplitType>,
  /// Split Position.
  #[sdk(child(qname = "c:CT_Double/c:splitPos"))]
  pub c_split_pos: Option<SplitPosition>,
  /// Custom Split.
  #[sdk(child(qname = "c:CT_CustSplit/c:custSplit"))]
  pub c_cust_split: Option<CustomSplit>,
  /// Second Pie Size.
  #[sdk(child(qname = "c:CT_SecondPieSize/c:secondPieSize"))]
  pub c_second_pie_size: Option<SecondPieSize>,
  /// Defines the SeriesLines Class.
  #[sdk(child(qname = "c:CT_ChartLines/c:serLines"))]
  pub c_ser_lines: Vec<SeriesLines>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// Surface Charts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SurfaceChart/c:surfaceChart")]
pub struct SurfaceChart {
  /// Wireframe
  #[sdk(child(qname = "c:CT_Boolean/c:wireframe"))]
  pub wireframe: Option<Wireframe>,
  /// Surface Chart Series.
  #[sdk(child(qname = "c:CT_SurfaceSer/c:ser"))]
  pub c_ser: Vec<SurfaceChartSeries>,
  /// Band Formats.
  #[sdk(child(qname = "c:CT_BandFmts/c:bandFmts"))]
  pub c_band_fmts: Option<BandFormats>,
  /// Axis ID.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// Defines the SurfaceChartExtensionList Class.
  #[sdk(child(qname = "c:CT_SurfaceChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<SurfaceChartExtensionList>,
}
/// 3D Surface Charts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Surface3DChart/c:surface3DChart")]
pub struct Surface3DChart {
  /// Wireframe.
  #[sdk(child(qname = "c:CT_Boolean/c:wireframe"))]
  pub wireframe: Option<Wireframe>,
  /// Defines the VaryColors Class.
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// Surface Chart Series.
  #[sdk(child(qname = "c:CT_SurfaceSer/c:ser"))]
  pub c_ser: Vec<SurfaceChartSeries>,
  /// Band Formats.
  #[sdk(child(qname = "c:CT_BandFmts/c:bandFmts"))]
  pub c_band_fmts: Option<BandFormats>,
  /// Axis ID.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// Defines the Surface3DChartExtensionList Class.
  #[sdk(child(qname = "c:CT_Surface3DChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<Surface3DChartExtensionList>,
}
/// Bubble Charts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BubbleChart/c:bubbleChart")]
pub struct BubbleChart {
  /// Defines the VaryColors Class.
  #[sdk(child(qname = "c:CT_Boolean/c:varyColors"))]
  pub vary_colors: Option<VaryColors>,
  /// Defines the BubbleChartSeries Class.
  #[sdk(child(qname = "c:CT_BubbleSer/c:ser"))]
  pub c_ser: Vec<BubbleChartSeries>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// 3D Bubble.
  #[sdk(child(qname = "c:CT_Boolean/c:bubble3D"))]
  pub c_bubble3_d: Option<Bubble3D>,
  /// Defines the BubbleScale Class.
  #[sdk(child(qname = "c:CT_BubbleScale/c:bubbleScale"))]
  pub c_bubble_scale: Option<BubbleScale>,
  /// Defines the ShowNegativeBubbles Class.
  #[sdk(child(qname = "c:CT_Boolean/c:showNegBubbles"))]
  pub c_show_neg_bubbles: Option<ShowNegativeBubbles>,
  /// Defines the SizeRepresents Class.
  #[sdk(child(qname = "c:CT_SizeRepresents/c:sizeRepresents"))]
  pub c_size_represents: Option<SizeRepresents>,
  /// Axis ID.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub c_ax_id: Vec<AxisId>,
  /// Defines the BubbleChartExtensionList Class.
  #[sdk(child(qname = "c:CT_BubbleChartExtensionList/c:extLst"))]
  pub c_ext_lst: Option<BubbleChartExtensionList>,
}
/// Value Axis.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ValAx/c:valAx")]
pub struct ValueAxis {
  /// Axis ID
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub axis_id: Option<AxisId>,
  /// Scaling
  #[sdk(child(qname = "c:CT_Scaling/c:scaling"))]
  pub scaling: Option<std::boxed::Box<Scaling>>,
  /// Delete
  #[sdk(child(qname = "c:CT_Boolean/c:delete"))]
  pub delete: Option<Delete>,
  /// Axis Position
  #[sdk(child(qname = "c:CT_AxPos/c:axPos"))]
  pub axis_position: Option<AxisPosition>,
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
  pub crossing_axis: Option<CrossingAxis>,
  #[sdk(choice(qname = "c:CT_Crosses/c:crosses", qname = "c:CT_Double/c:crossesAt"))]
  pub value_axis_choice: Option<ValueAxisChoice>,
  /// Defines the CrossBetween Class.
  #[sdk(child(qname = "c:CT_CrossBetween/c:crossBetween"))]
  pub c_cross_between: Option<CrossBetween>,
  /// Defines the MajorUnit Class.
  #[sdk(child(qname = "c:CT_AxisUnit/c:majorUnit"))]
  pub c_major_unit: Option<MajorUnit>,
  /// Defines the MinorUnit Class.
  #[sdk(child(qname = "c:CT_AxisUnit/c:minorUnit"))]
  pub c_minor_unit: Option<MinorUnit>,
  /// Defines the DisplayUnits Class.
  #[sdk(child(qname = "c:CT_DispUnits/c:dispUnits"))]
  pub c_disp_units: Option<std::boxed::Box<DisplayUnits>>,
  /// Defines the ValAxExtensionList Class.
  #[sdk(child(qname = "c:CT_ValAxExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ValAxExtensionList>,
}
/// Category Axis Data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_CatAx/c:catAx")]
pub struct CategoryAxis {
  /// Axis ID
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub axis_id: Option<AxisId>,
  /// Scaling
  #[sdk(child(qname = "c:CT_Scaling/c:scaling"))]
  pub scaling: Option<std::boxed::Box<Scaling>>,
  /// Delete
  #[sdk(child(qname = "c:CT_Boolean/c:delete"))]
  pub delete: Option<Delete>,
  /// Axis Position
  #[sdk(child(qname = "c:CT_AxPos/c:axPos"))]
  pub axis_position: Option<AxisPosition>,
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
  pub crossing_axis: Option<CrossingAxis>,
  #[sdk(choice(qname = "c:CT_Crosses/c:crosses", qname = "c:CT_Double/c:crossesAt"))]
  pub category_axis_choice: Option<CategoryAxisChoice>,
  /// Defines the AutoLabeled Class.
  #[sdk(child(qname = "c:CT_Boolean/c:auto"))]
  pub c_auto: Option<AutoLabeled>,
  /// Defines the LabelAlignment Class.
  #[sdk(child(qname = "c:CT_LblAlgn/c:lblAlgn"))]
  pub c_lbl_algn: Option<LabelAlignment>,
  /// Defines the LabelOffset Class.
  #[sdk(child(qname = "c:CT_LblOffset/c:lblOffset"))]
  pub c_lbl_offset: Option<LabelOffset>,
  /// Defines the TickLabelSkip Class.
  #[sdk(child(qname = "c:CT_Skip/c:tickLblSkip"))]
  pub c_tick_lbl_skip: Option<TickLabelSkip>,
  /// Defines the TickMarkSkip Class.
  #[sdk(child(qname = "c:CT_Skip/c:tickMarkSkip"))]
  pub c_tick_mark_skip: Option<TickMarkSkip>,
  /// Defines the NoMultiLevelLabels Class.
  #[sdk(child(qname = "c:CT_Boolean/c:noMultiLvlLbl"))]
  pub c_no_multi_lvl_lbl: Option<NoMultiLevelLabels>,
  /// Defines the CatAxExtensionList Class.
  #[sdk(child(qname = "c:CT_CatAxExtensionList/c:extLst"))]
  pub c_ext_lst: Option<CatAxExtensionList>,
}
/// Date Axis.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DateAx/c:dateAx")]
pub struct DateAxis {
  /// Axis ID
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub axis_id: Option<AxisId>,
  /// Scaling
  #[sdk(child(qname = "c:CT_Scaling/c:scaling"))]
  pub scaling: Option<std::boxed::Box<Scaling>>,
  /// Delete
  #[sdk(child(qname = "c:CT_Boolean/c:delete"))]
  pub delete: Option<Delete>,
  /// Axis Position
  #[sdk(child(qname = "c:CT_AxPos/c:axPos"))]
  pub axis_position: Option<AxisPosition>,
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
  pub crossing_axis: Option<CrossingAxis>,
  #[sdk(choice(qname = "c:CT_Crosses/c:crosses", qname = "c:CT_Double/c:crossesAt"))]
  pub date_axis_choice: Option<DateAxisChoice>,
  /// Defines the AutoLabeled Class.
  #[sdk(child(qname = "c:CT_Boolean/c:auto"))]
  pub c_auto: Option<AutoLabeled>,
  /// Defines the LabelOffset Class.
  #[sdk(child(qname = "c:CT_LblOffset/c:lblOffset"))]
  pub c_lbl_offset: Option<LabelOffset>,
  /// Defines the BaseTimeUnit Class.
  #[sdk(child(qname = "c:CT_TimeUnit/c:baseTimeUnit"))]
  pub c_base_time_unit: Option<BaseTimeUnit>,
  /// Defines the MajorUnit Class.
  #[sdk(child(qname = "c:CT_AxisUnit/c:majorUnit"))]
  pub c_major_unit: Option<MajorUnit>,
  /// Defines the MajorTimeUnit Class.
  #[sdk(child(qname = "c:CT_TimeUnit/c:majorTimeUnit"))]
  pub c_major_time_unit: Option<MajorTimeUnit>,
  /// Defines the MinorUnit Class.
  #[sdk(child(qname = "c:CT_AxisUnit/c:minorUnit"))]
  pub c_minor_unit: Option<MinorUnit>,
  /// Defines the MinorTimeUnit Class.
  #[sdk(child(qname = "c:CT_TimeUnit/c:minorTimeUnit"))]
  pub c_minor_time_unit: Option<MinorTimeUnit>,
  /// Defines the DateAxExtensionList Class.
  #[sdk(child(qname = "c:CT_DateAxExtensionList/c:extLst"))]
  pub c_ext_lst: Option<DateAxExtensionList>,
}
/// Series Axis.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SerAx/c:serAx")]
pub struct SeriesAxis {
  /// Axis ID
  #[sdk(child(qname = "c:CT_UnsignedInt/c:axId"))]
  pub axis_id: Option<AxisId>,
  /// Scaling
  #[sdk(child(qname = "c:CT_Scaling/c:scaling"))]
  pub scaling: Option<std::boxed::Box<Scaling>>,
  /// Delete
  #[sdk(child(qname = "c:CT_Boolean/c:delete"))]
  pub delete: Option<Delete>,
  /// Axis Position
  #[sdk(child(qname = "c:CT_AxPos/c:axPos"))]
  pub axis_position: Option<AxisPosition>,
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
  pub crossing_axis: Option<CrossingAxis>,
  #[sdk(choice(qname = "c:CT_Crosses/c:crosses", qname = "c:CT_Double/c:crossesAt"))]
  pub series_axis_choice: Option<SeriesAxisChoice>,
  /// Defines the TickLabelSkip Class.
  #[sdk(child(qname = "c:CT_Skip/c:tickLblSkip"))]
  pub c_tick_lbl_skip: Option<TickLabelSkip>,
  /// Defines the TickMarkSkip Class.
  #[sdk(child(qname = "c:CT_Skip/c:tickMarkSkip"))]
  pub c_tick_mark_skip: Option<TickMarkSkip>,
  /// Defines the SerAxExtensionList Class.
  #[sdk(child(qname = "c:CT_SerAxExtensionList/c:extLst"))]
  pub c_ext_lst: Option<SerAxExtensionList>,
}
/// Data Table.
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
  /// Defines the ChartShapeProperties Class.
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_FirstSliceAng/c:firstSliceAng")]
pub struct FirstSliceAngle {
  /// First Slice Angle Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..= 360))]
  pub val: Option<crate::simple_type::UInt16Value>,
}
/// Hole Size.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_HoleSize/c:holeSize")]
pub struct HoleSize {
  /// Hole Size Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 1..= 90))]
  pub val: crate::simple_type::ByteValue,
}
/// String Point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StrVal/c:pt")]
pub struct StringPoint {
  /// Index
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  /// Text Value
  #[sdk(text_child(qname = "c:ST_Xstring/c:v"))]
  pub numeric_value: crate::simple_type::StringValue,
}
/// Thickness.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_WallThickness/c:thickness")]
pub struct Thickness {
  /// val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..= 100))]
  pub val: Option<crate::simple_type::ByteValue>,
}
/// Defines the StockChartExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StockChartExtension/c:ext")]
pub struct StockChartExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c15:CT_FilteredLineSer/c15:filteredLineSeries", any))]
  pub stock_chart_extension_choice: Option<StockChartExtensionChoice>,
}
/// Defines the PieChartExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PieChartExtension/c:ext")]
pub struct PieChartExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c15:CT_FilteredPieSer/c15:filteredPieSeries", any))]
  pub pie_chart_extension_choice: Option<PieChartExtensionChoice>,
}
/// Defines the Pie3DChartExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Pie3DChartExtension/c:ext")]
pub struct Pie3DChartExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c15:CT_FilteredPieSer/c15:filteredPieSeries", any))]
  pub pie3_d_chart_extension_choice: Option<Pie3DChartExtensionChoice>,
}
/// Defines the NumRefExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumRefExtension/c:ext")]
pub struct NumRefExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "c15:CT_FullRef/c15:fullRef",
    qname = "c15:CT_LevelRef/c15:levelRef",
    qname = "c15:CT_FormulaRef/c15:formulaRef",
    any
  ))]
  pub num_ref_extension_choice: Option<NumRefExtensionChoice>,
}
/// Defines the StrDataExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StrDataExtension/c:ext")]
pub struct StrDataExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c:CT_Boolean/c15:autoCat", any))]
  pub str_data_extension_choice: Option<StrDataExtensionChoice>,
}
/// Defines the StrRefExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StrRefExtension/c:ext")]
pub struct StrRefExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "c15:CT_FullRef/c15:fullRef",
    qname = "c15:CT_LevelRef/c15:levelRef",
    qname = "c15:CT_FormulaRef/c15:formulaRef",
    any
  ))]
  pub str_ref_extension_choice: Option<StrRefExtensionChoice>,
}
/// Defines the MultiLvlStrRefExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_MultiLvlStrRefExtension/c:ext")]
pub struct MultiLvlStrRefExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "c15:CT_FullRef/c15:fullRef",
    qname = "c15:CT_LevelRef/c15:levelRef",
    qname = "c15:CT_FormulaRef/c15:formulaRef",
    any
  ))]
  pub multi_lvl_str_ref_extension_choice: Option<MultiLvlStrRefExtensionChoice>,
}
/// Defines the DLblsExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DLblsExtension/c:ext")]
pub struct DLblsExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "c:CT_Tx/c15:tx",
    qname = "c15:CT_DataLabelFieldTable/c15:dlblFieldTable",
    qname = "c:CT_Boolean/c15:showDataLabelsRange",
    qname = "a:CT_ShapeProperties/c15:spPr",
    qname = "c:CT_Layout/c15:layout",
    qname = "c:CT_Boolean/c15:showLeaderLines",
    qname = "c:CT_ChartLines/c15:leaderLines",
    any
  ))]
  pub d_lbls_extension_choice: Option<DLblsExtensionChoice>,
}
/// Defines the LineChartExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LineChartExtension/c:ext")]
pub struct LineChartExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c15:CT_FilteredLineSer/c15:filteredLineSeries", any))]
  pub line_chart_extension_choice: Option<LineChartExtensionChoice>,
}
/// Defines the Line3DChartExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Line3DChartExtension/c:ext")]
pub struct Line3DChartExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c15:CT_FilteredLineSer/c15:filteredLineSeries", any))]
  pub line3_d_chart_extension_choice: Option<Line3DChartExtensionChoice>,
}
/// Defines the ScatterChartExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ScatterChartExtension/c:ext")]
pub struct ScatterChartExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c15:CT_FilteredScatterSer/c15:filteredScatterSeries", any))]
  pub scatter_chart_extension_choice: Option<ScatterChartExtensionChoice>,
}
/// Defines the RadarChartExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RadarChartExtension/c:ext")]
pub struct RadarChartExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c15:CT_FilteredRadarSer/c15:filteredRadarSeries", any))]
  pub radar_chart_extension_choice: Option<RadarChartExtensionChoice>,
}
/// Defines the BarChartExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BarChartExtension/c:ext")]
pub struct BarChartExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c15:CT_FilteredBarSer/c15:filteredBarSeries", any))]
  pub bar_chart_extension_choice: Option<BarChartExtensionChoice>,
}
/// Defines the Bar3DChartExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Bar3DChartExtension/c:ext")]
pub struct Bar3DChartExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c15:CT_FilteredBarSer/c15:filteredBarSeries", any))]
  pub bar3_d_chart_extension_choice: Option<Bar3DChartExtensionChoice>,
}
/// Defines the AreaChartExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AreaChartExtension/c:ext")]
pub struct AreaChartExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c15:CT_FilteredAreaSer/c15:filteredAreaSeries", any))]
  pub area_chart_extension_choice: Option<AreaChartExtensionChoice>,
}
/// Defines the Area3DChartExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Area3DChartExtension/c:ext")]
pub struct Area3DChartExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c15:CT_FilteredAreaSer/c15:filteredAreaSeries", any))]
  pub area3_d_chart_extension_choice: Option<Area3DChartExtensionChoice>,
}
/// Defines the BubbleChartExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BubbleChartExtension/c:ext")]
pub struct BubbleChartExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c15:CT_FilteredBubbleSer/c15:filteredBubbleSeries", any))]
  pub bubble_chart_extension_choice: Option<BubbleChartExtensionChoice>,
}
/// Defines the SurfaceChartExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SurfaceChartExtension/c:ext")]
pub struct SurfaceChartExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c15:CT_FilteredSurfaceSer/c15:filteredSurfaceSeries", any))]
  pub surface_chart_extension_choice: Option<SurfaceChartExtensionChoice>,
}
/// Defines the Surface3DChartExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Surface3DChartExtension/c:ext")]
pub struct Surface3DChartExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c15:CT_FilteredSurfaceSer/c15:filteredSurfaceSeries", any))]
  pub surface3_d_chart_extension_choice: Option<Surface3DChartExtensionChoice>,
}
/// Defines the CatAxExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_CatAxExtension/c:ext")]
pub struct CatAxExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c:CT_NumFmt/c15:numFmt", any))]
  pub cat_ax_extension_choice: Option<CatAxExtensionChoice>,
}
/// Defines the DateAxExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DateAxExtension/c:ext")]
pub struct DateAxExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c:CT_NumFmt/c15:numFmt", any))]
  pub date_ax_extension_choice: Option<DateAxExtensionChoice>,
}
/// Defines the SerAxExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SerAxExtension/c:ext")]
pub struct SerAxExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c:CT_NumFmt/c15:numFmt", any))]
  pub ser_ax_extension_choice: Option<SerAxExtensionChoice>,
}
/// Defines the ValAxExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ValAxExtension/c:ext")]
pub struct ValAxExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(qname = "c:CT_NumFmt/c15:numFmt", any))]
  pub val_ax_extension_choice: Option<ValAxExtensionChoice>,
}
/// Defines the UpDownBars Class.
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StockChartExtensionList/c:extLst")]
pub struct StockChartExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the StockChartExtension Class.
  #[sdk(child(qname = "c:CT_StockChartExtension/c:ext"))]
  pub c_ext: Vec<StockChartExtension>,
}
/// Defines the PieChartExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PieChartExtensionList/c:extLst")]
pub struct PieChartExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the PieChartExtension Class.
  #[sdk(child(qname = "c:CT_PieChartExtension/c:ext"))]
  pub c_ext: Vec<PieChartExtension>,
}
/// Defines the Pie3DChartExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Pie3DChartExtensionList/c:extLst")]
pub struct Pie3DChartExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the Pie3DChartExtension Class.
  #[sdk(child(qname = "c:CT_Pie3DChartExtension/c:ext"))]
  pub c_ext: Vec<Pie3DChartExtension>,
}
/// Defines the NumRefExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_NumRefExtensionList/c:extLst")]
pub struct NumRefExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the NumRefExtension Class.
  #[sdk(child(qname = "c:CT_NumRefExtension/c:ext"))]
  pub c_ext: Vec<NumRefExtension>,
}
/// Defines the StrDataExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StrDataExtensionList/c:extLst")]
pub struct StrDataExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the StrDataExtension Class.
  #[sdk(child(qname = "c:CT_StrDataExtension/c:ext"))]
  pub c_ext: Vec<StrDataExtension>,
}
/// Defines the StrRefExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_StrRefExtensionList/c:extLst")]
pub struct StrRefExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the StrRefExtension Class.
  #[sdk(child(qname = "c:CT_StrRefExtension/c:ext"))]
  pub c_ext: Vec<StrRefExtension>,
}
/// Defines the MultiLevelStringCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_MultiLvlStrData/c:multiLvlStrCache")]
pub struct MultiLevelStringCache {
  /// Point Count.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  pub point_count: Option<PointCount>,
  /// Level.
  #[sdk(child(qname = "c:CT_Lvl/c:lvl"))]
  pub c_lvl: Vec<Level>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// Defines the MultiLvlStrRefExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_MultiLvlStrRefExtensionList/c:extLst")]
pub struct MultiLvlStrRefExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the MultiLvlStrRefExtension Class.
  #[sdk(child(qname = "c:CT_MultiLvlStrRefExtension/c:ext"))]
  pub c_ext: Vec<MultiLvlStrRefExtension>,
}
/// Defines the DLblsExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DLblsExtensionList/c:extLst")]
pub struct DLblsExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the DLblsExtension Class.
  #[sdk(child(qname = "c:CT_DLblsExtension/c:ext"))]
  pub c_ext: Vec<DLblsExtension>,
}
/// Defines the LineChartExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LineChartExtensionList/c:extLst")]
pub struct LineChartExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the LineChartExtension Class.
  #[sdk(child(qname = "c:CT_LineChartExtension/c:ext"))]
  pub c_ext: Vec<LineChartExtension>,
}
/// Defines the Line3DChartExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Line3DChartExtensionList/c:extLst")]
pub struct Line3DChartExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the Line3DChartExtension Class.
  #[sdk(child(qname = "c:CT_Line3DChartExtension/c:ext"))]
  pub c_ext: Vec<Line3DChartExtension>,
}
/// Defines the ScatterStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ScatterStyle/c:scatterStyle")]
pub struct ScatterStyle {
  /// Scatter Style Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<ScatterStyleValues>,
}
/// Defines the ScatterChartSeries Class.
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
  /// Marker.
  #[sdk(child(qname = "c:CT_Marker/c:marker"))]
  pub marker: Option<std::boxed::Box<Marker>>,
  /// Defines the DataPoint Class.
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<DataPoint>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Defines the Trendline Class.
  #[sdk(child(qname = "c:CT_Trendline/c:trendline"))]
  pub c_trendline: Vec<Trendline>,
  /// Defines the ErrorBars Class.
  #[sdk(child(qname = "c:CT_ErrBars/c:errBars"))]
  pub c_err_bars: Vec<ErrorBars>,
  /// Defines the XValues Class.
  #[sdk(child(qname = "c:CT_AxDataSource/c:xVal"))]
  pub c_x_val: Option<std::boxed::Box<XValues>>,
  /// Defines the YValues Class.
  #[sdk(child(qname = "c:CT_NumDataSource/c:yVal"))]
  pub c_y_val: Option<std::boxed::Box<YValues>>,
  /// Defines the Smooth Class.
  #[sdk(child(qname = "c:CT_Boolean/c:smooth"))]
  pub c_smooth: Option<Smooth>,
  /// Defines the ScatterSerExtensionList Class.
  #[sdk(child(qname = "c:CT_ScatterSerExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ScatterSerExtensionList>,
}
/// Defines the ScatterChartExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ScatterChartExtensionList/c:extLst")]
pub struct ScatterChartExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the ScatterChartExtension Class.
  #[sdk(child(qname = "c:CT_ScatterChartExtension/c:ext"))]
  pub c_ext: Vec<ScatterChartExtension>,
}
/// Defines the RadarStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RadarStyle/c:radarStyle")]
pub struct RadarStyle {
  /// Radar Style Value
  #[sdk(attr(qname = ":val"))]
  pub val: RadarStyleValues,
}
/// Defines the RadarChartSeries Class.
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
  /// Defines the PictureOptions Class.
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// Marker.
  #[sdk(child(qname = "c:CT_Marker/c:marker"))]
  pub marker: Option<std::boxed::Box<Marker>>,
  /// Defines the DataPoint Class.
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<DataPoint>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Defines the CategoryAxisData Class.
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub c_cat: Option<std::boxed::Box<CategoryAxisData>>,
  /// Defines the Values Class.
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub c_val: Option<std::boxed::Box<Values>>,
  /// Defines the RadarSerExtensionList Class.
  #[sdk(child(qname = "c:CT_RadarSerExtensionList/c:extLst"))]
  pub c_ext_lst: Option<RadarSerExtensionList>,
}
/// Defines the RadarChartExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RadarChartExtensionList/c:extLst")]
pub struct RadarChartExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the RadarChartExtension Class.
  #[sdk(child(qname = "c:CT_RadarChartExtension/c:ext"))]
  pub c_ext: Vec<RadarChartExtension>,
}
/// Defines the Overlap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Overlap/c:overlap")]
pub struct Overlap {
  /// Overlap Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = -100..= 100))]
  pub val: Option<crate::simple_type::SByteValue>,
}
/// Defines the BarChartExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BarChartExtensionList/c:extLst")]
pub struct BarChartExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the BarChartExtension Class.
  #[sdk(child(qname = "c:CT_BarChartExtension/c:ext"))]
  pub c_ext: Vec<BarChartExtension>,
}
/// Defines the Shape Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Shape/c:shape")]
pub struct Shape {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Shape Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<ShapeValues>,
}
/// Defines the Bar3DChartExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Bar3DChartExtensionList/c:extLst")]
pub struct Bar3DChartExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the Bar3DChartExtension Class.
  #[sdk(child(qname = "c:CT_Bar3DChartExtension/c:ext"))]
  pub c_ext: Vec<Bar3DChartExtension>,
}
/// Defines the AreaChartExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AreaChartExtensionList/c:extLst")]
pub struct AreaChartExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the AreaChartExtension Class.
  #[sdk(child(qname = "c:CT_AreaChartExtension/c:ext"))]
  pub c_ext: Vec<AreaChartExtension>,
}
/// Defines the Area3DChartExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Area3DChartExtensionList/c:extLst")]
pub struct Area3DChartExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the Area3DChartExtension Class.
  #[sdk(child(qname = "c:CT_Area3DChartExtension/c:ext"))]
  pub c_ext: Vec<Area3DChartExtension>,
}
/// Defines the BubbleChartSeries Class.
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
  /// Defines the PictureOptions Class.
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// Invert if Negative.
  #[sdk(child(qname = "c:CT_Boolean/c:invertIfNegative"))]
  pub invert_if_negative: Option<InvertIfNegative>,
  /// Defines the DataPoint Class.
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<DataPoint>,
  /// Data Labels.
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<std::boxed::Box<DataLabels>>,
  /// Defines the Trendline Class.
  #[sdk(child(qname = "c:CT_Trendline/c:trendline"))]
  pub c_trendline: Vec<Trendline>,
  /// Defines the ErrorBars Class.
  #[sdk(child(qname = "c:CT_ErrBars/c:errBars"))]
  pub c_err_bars: Vec<ErrorBars>,
  /// Defines the XValues Class.
  #[sdk(child(qname = "c:CT_AxDataSource/c:xVal"))]
  pub c_x_val: Option<std::boxed::Box<XValues>>,
  /// Defines the YValues Class.
  #[sdk(child(qname = "c:CT_NumDataSource/c:yVal"))]
  pub c_y_val: Option<std::boxed::Box<YValues>>,
  /// Defines the BubbleSize Class.
  #[sdk(child(qname = "c:CT_NumDataSource/c:bubbleSize"))]
  pub c_bubble_size: Option<std::boxed::Box<BubbleSize>>,
  /// 3D Bubble.
  #[sdk(child(qname = "c:CT_Boolean/c:bubble3D"))]
  pub c_bubble3_d: Option<Bubble3D>,
  /// Defines the BubbleSerExtensionList Class.
  #[sdk(child(qname = "c:CT_BubbleSerExtensionList/c:extLst"))]
  pub c_ext_lst: Option<BubbleSerExtensionList>,
}
/// Defines the BubbleScale Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BubbleScale/c:bubbleScale")]
pub struct BubbleScale {
  /// Bubble Scale Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..= 300))]
  pub val: Option<crate::simple_type::UInt32Value>,
}
/// Defines the SizeRepresents Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SizeRepresents/c:sizeRepresents")]
pub struct SizeRepresents {
  /// Size Represents Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<SizeRepresentsValues>,
}
/// Defines the BubbleChartExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BubbleChartExtensionList/c:extLst")]
pub struct BubbleChartExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the BubbleChartExtension Class.
  #[sdk(child(qname = "c:CT_BubbleChartExtension/c:ext"))]
  pub c_ext: Vec<BubbleChartExtension>,
}
/// Defines the SurfaceChartExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SurfaceChartExtensionList/c:extLst")]
pub struct SurfaceChartExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the SurfaceChartExtension Class.
  #[sdk(child(qname = "c:CT_SurfaceChartExtension/c:ext"))]
  pub c_ext: Vec<SurfaceChartExtension>,
}
/// Defines the Surface3DChartExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Surface3DChartExtensionList/c:extLst")]
pub struct Surface3DChartExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the Surface3DChartExtension Class.
  #[sdk(child(qname = "c:CT_Surface3DChartExtension/c:ext"))]
  pub c_ext: Vec<Surface3DChartExtension>,
}
/// Defines the LabelAlignment Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LblAlgn/c:lblAlgn")]
pub struct LabelAlignment {
  /// Label Alignment Value
  #[sdk(attr(qname = ":val"))]
  pub val: LabelAlignmentValues,
}
/// Defines the LabelOffset Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LblOffset/c:lblOffset")]
pub struct LabelOffset {
  /// Label Offset Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 0..= 1000))]
  pub val: Option<crate::simple_type::UInt16Value>,
}
/// Defines the TickLabelSkip Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Skip/c:tickLblSkip")]
pub struct TickLabelSkip {
  /// Tick Skip Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 1..))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the TickMarkSkip Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Skip/c:tickMarkSkip")]
pub struct TickMarkSkip {
  /// Tick Skip Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 1..))]
  pub val: crate::simple_type::Int32Value,
}
/// Defines the CatAxExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_CatAxExtensionList/c:extLst")]
pub struct CatAxExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the CatAxExtension Class.
  #[sdk(child(qname = "c:CT_CatAxExtension/c:ext"))]
  pub c_ext: Vec<CatAxExtension>,
}
/// Defines the BaseTimeUnit Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TimeUnit/c:baseTimeUnit")]
pub struct BaseTimeUnit {
  /// Time Unit Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<TimeUnitValues>,
}
/// Defines the MajorTimeUnit Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TimeUnit/c:majorTimeUnit")]
pub struct MajorTimeUnit {
  /// Time Unit Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<TimeUnitValues>,
}
/// Defines the MinorTimeUnit Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TimeUnit/c:minorTimeUnit")]
pub struct MinorTimeUnit {
  /// Time Unit Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<TimeUnitValues>,
}
/// Defines the MajorUnit Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AxisUnit/c:majorUnit")]
pub struct MajorUnit {
  /// Major Unit Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(min = 0, min_inclusive = false, max_inclusive = false))]
  pub val: crate::simple_type::DoubleValue,
}
/// Defines the MinorUnit Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AxisUnit/c:minorUnit")]
pub struct MinorUnit {
  /// Major Unit Value
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(min = 0, min_inclusive = false, max_inclusive = false))]
  pub val: crate::simple_type::DoubleValue,
}
/// Defines the DateAxExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DateAxExtensionList/c:extLst")]
pub struct DateAxExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the DateAxExtension Class.
  #[sdk(child(qname = "c:CT_DateAxExtension/c:ext"))]
  pub c_ext: Vec<DateAxExtension>,
}
/// Defines the SerAxExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SerAxExtensionList/c:extLst")]
pub struct SerAxExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the SerAxExtension Class.
  #[sdk(child(qname = "c:CT_SerAxExtension/c:ext"))]
  pub c_ext: Vec<SerAxExtension>,
}
/// Defines the CrossBetween Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_CrossBetween/c:crossBetween")]
pub struct CrossBetween {
  /// Cross Between Value
  #[sdk(attr(qname = ":val"))]
  pub val: CrossBetweenValues,
}
/// Defines the DisplayUnits Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DispUnits/c:dispUnits")]
pub struct DisplayUnits {
  #[sdk(choice(
    qname = "c:CT_Double/c:custUnit",
    qname = "c:CT_BuiltInUnit/c:builtInUnit"
  ))]
  pub display_units_choice: Option<DisplayUnitsChoice>,
  /// Display Units Label.
  #[sdk(child(qname = "c:CT_DispUnitsLbl/c:dispUnitsLbl"))]
  pub c_disp_units_lbl: Option<std::boxed::Box<DisplayUnitsLabel>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// Defines the ValAxExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ValAxExtensionList/c:extLst")]
pub struct ValAxExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the ValAxExtension Class.
  #[sdk(child(qname = "c:CT_ValAxExtension/c:ext"))]
  pub c_ext: Vec<ValAxExtension>,
}
/// Defines the DLblExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DLblExtensionList/c:extLst")]
pub struct DLblExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the DLblExtension Class.
  #[sdk(child(qname = "c:CT_DLblExtension/c:ext"))]
  pub c_ext: Vec<DLblExtension>,
}
/// Defines the DLblExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DLblExtension/c:ext")]
pub struct DLblExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "c15:CT_DataLabelFieldTable/c15:dlblFieldTable",
    qname = "c:CT_Boolean/c15:xForSave",
    qname = "c:CT_Boolean/c15:showDataLabelsRange",
    qname = "a:CT_ShapeProperties/c15:spPr",
    qname = "c:CT_Layout/c15:layout",
    qname = "c16:CT_ChartUniqueID/c16:uniqueId",
    any
  ))]
  pub d_lbl_extension_choice: Option<DLblExtensionChoice>,
}
/// Defines the DataPoint Class.
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
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// Defines the PictureOptions Class.
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Trendline Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Trendline/c:trendline")]
pub struct Trendline {
  /// Trendline Name
  #[sdk(text_child(qname = "xsd:string/c:name"))]
  pub trendline_name: Option<crate::simple_type::StringValue>,
  /// Defines the ChartShapeProperties Class.
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
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<ChartShapeProperties>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CategoryAxisData Class.
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
  pub category_axis_data_choice: Option<CategoryAxisDataChoice>,
}
/// Defines the XValues Class.
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
  pub x_values_choice: Option<XValuesChoice>,
}
/// Defines the LineSerExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LineSerExtensionList/c:extLst")]
pub struct LineSerExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the LineSerExtension Class.
  #[sdk(child(qname = "c:CT_LineSerExtension/c:ext"))]
  pub c_ext: Vec<LineSerExtension>,
}
/// Defines the LineSerExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_LineSerExtension/c:ext")]
pub struct LineSerExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle",
    qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle",
    qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange",
    qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions",
    qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions",
    qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap",
    qname = "c16:CT_ChartUniqueID/c16:uniqueId",
    any
  ))]
  pub line_ser_extension_choice: Option<LineSerExtensionChoice>,
}
/// Defines the ScatterSerExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ScatterSerExtensionList/c:extLst")]
pub struct ScatterSerExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the ScatterSerExtension Class.
  #[sdk(child(qname = "c:CT_ScatterSerExtension/c:ext"))]
  pub c_ext: Vec<ScatterSerExtension>,
}
/// Defines the ScatterSerExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ScatterSerExtension/c:ext")]
pub struct ScatterSerExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle",
    qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle",
    qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange",
    qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions",
    qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions",
    qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap",
    qname = "c16:CT_ChartUniqueID/c16:uniqueId",
    any
  ))]
  pub scatter_ser_extension_choice: Option<ScatterSerExtensionChoice>,
}
/// Defines the RadarSerExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RadarSerExtensionList/c:extLst")]
pub struct RadarSerExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the RadarSerExtension Class.
  #[sdk(child(qname = "c:CT_RadarSerExtension/c:ext"))]
  pub c_ext: Vec<RadarSerExtension>,
}
/// Defines the RadarSerExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_RadarSerExtension/c:ext")]
pub struct RadarSerExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle",
    qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle",
    qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange",
    qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions",
    qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions",
    qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap",
    qname = "c16:CT_ChartUniqueID/c16:uniqueId",
    any
  ))]
  pub radar_ser_extension_choice: Option<RadarSerExtensionChoice>,
}
/// Defines the BarSerExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BarSerExtensionList/c:extLst")]
pub struct BarSerExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the BarSerExtension Class.
  #[sdk(child(qname = "c:CT_BarSerExtension/c:ext"))]
  pub c_ext: Vec<BarSerExtension>,
}
/// Defines the BarSerExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BarSerExtension/c:ext")]
pub struct BarSerExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "c14:CT_InvertSolidFillFmt/c14:invertSolidFillFmt",
    qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle",
    qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle",
    qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange",
    qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions",
    qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions",
    qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap",
    qname = "c16:CT_ChartUniqueID/c16:uniqueId",
    any
  ))]
  pub bar_ser_extension_choice: Option<BarSerExtensionChoice>,
}
/// Defines the AreaSerExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AreaSerExtensionList/c:extLst")]
pub struct AreaSerExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the AreaSerExtension Class.
  #[sdk(child(qname = "c:CT_AreaSerExtension/c:ext"))]
  pub c_ext: Vec<AreaSerExtension>,
}
/// Defines the AreaSerExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_AreaSerExtension/c:ext")]
pub struct AreaSerExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle",
    qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle",
    qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange",
    qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions",
    qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions",
    qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap",
    qname = "c16:CT_ChartUniqueID/c16:uniqueId",
    any
  ))]
  pub area_ser_extension_choice: Option<AreaSerExtensionChoice>,
}
/// Defines the PieSerExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PieSerExtensionList/c:extLst")]
pub struct PieSerExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the PieSerExtension Class.
  #[sdk(child(qname = "c:CT_PieSerExtension/c:ext"))]
  pub c_ext: Vec<PieSerExtension>,
}
/// Defines the PieSerExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PieSerExtension/c:ext")]
pub struct PieSerExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle",
    qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle",
    qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange",
    qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions",
    qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions",
    qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap",
    qname = "c16:CT_ChartUniqueID/c16:uniqueId",
    any
  ))]
  pub pie_ser_extension_choice: Option<PieSerExtensionChoice>,
}
/// Defines the BubbleSerExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BubbleSerExtensionList/c:extLst")]
pub struct BubbleSerExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the BubbleSerExtension Class.
  #[sdk(child(qname = "c:CT_BubbleSerExtension/c:ext"))]
  pub c_ext: Vec<BubbleSerExtension>,
}
/// Defines the BubbleSerExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_BubbleSerExtension/c:ext")]
pub struct BubbleSerExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "c14:CT_InvertSolidFillFmt/c14:invertSolidFillFmt",
    qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle",
    qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange",
    qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions",
    qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions",
    qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap",
    qname = "c16:CT_ChartUniqueID/c16:uniqueId",
    any
  ))]
  pub bubble_ser_extension_choice: Option<BubbleSerExtensionChoice>,
}
/// Defines the SurfaceSerExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SurfaceSerExtensionList/c:extLst")]
pub struct SurfaceSerExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the SurfaceSerExtension Class.
  #[sdk(child(qname = "c:CT_SurfaceSerExtension/c:ext"))]
  pub c_ext: Vec<SurfaceSerExtension>,
}
/// Defines the SurfaceSerExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_SurfaceSerExtension/c:ext")]
pub struct SurfaceSerExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle",
    qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle",
    qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions",
    qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions",
    qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap",
    qname = "c16:CT_ChartUniqueID/c16:uniqueId",
    any
  ))]
  pub surface_ser_extension_choice: Option<SurfaceSerExtensionChoice>,
}
/// Defines the DataDisplayOptions16 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "c16r3:CT_DataDisplayOptions16/c:ext")]
pub struct DataDisplayOptions16 {
  /// Defines the BooleanFalse Class.
  #[sdk(child(office2019, qname = "c16r3:CT_BooleanFalse/c16r3:dispNaAsBlank"))]
  pub boolean_false:
    Option<crate::schemas::schemas_microsoft_com_office_drawing_2017_03_chart::BooleanFalse>,
}
/// pivot chart format persistence data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_PivotFmts/c:pivotFmts")]
pub struct PivotFormats {
  /// Pivot Format.
  #[sdk(child(qname = "c:CT_PivotFmt/c:pivotFmt"))]
  pub c_pivot_fmt: Vec<PivotFormat>,
}
/// 3D view settings.
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Surface/c:floor")]
pub struct Floor {
  /// Thickness
  #[sdk(child(qname = "c:CT_WallThickness/c:thickness"))]
  pub thickness: Option<Thickness>,
  /// Defines the ShapeProperties Class.
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Surface/c:sideWall")]
pub struct SideWall {
  /// Thickness
  #[sdk(child(qname = "c:CT_WallThickness/c:thickness"))]
  pub thickness: Option<Thickness>,
  /// Defines the ShapeProperties Class.
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Surface/c:backWall")]
pub struct BackWall {
  /// Thickness
  #[sdk(child(qname = "c:CT_WallThickness/c:thickness"))]
  pub thickness: Option<Thickness>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "a:CT_ShapeProperties/c:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Picture Options
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<PictureOptions>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Plot data and formatting.
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
  /// Data Table.
  #[sdk(child(qname = "c:CT_DTable/c:dTable"))]
  pub c_d_table: Option<std::boxed::Box<DataTable>>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "a:CT_ShapeProperties/c:spPr"))]
  pub c_sp_pr: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// Legend data and formatting.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Legend/c:legend")]
pub struct Legend {
  /// Legend Position
  #[sdk(child(qname = "c:CT_LegendPos/c:legendPos"))]
  pub legend_position: Option<LegendPosition>,
  /// Legend Entry.
  #[sdk(child(qname = "c:CT_LegendEntry/c:legendEntry"))]
  pub c_legend_entry: Vec<LegendEntry>,
  /// Layout.
  #[sdk(child(qname = "c:CT_Layout/c:layout"))]
  pub c_layout: Option<std::boxed::Box<Layout>>,
  /// Overlay.
  #[sdk(child(qname = "c:CT_Boolean/c:overlay"))]
  pub c_overlay: Option<Overlay>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub c_sp_pr: Option<std::boxed::Box<ChartShapeProperties>>,
  /// Defines the TextProperties Class.
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub c_tx_pr: Option<std::boxed::Box<TextProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<ExtensionList>,
}
/// The way that blank cells are plotted on a chart..
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_DispBlanksAs/c:dispBlanksAs")]
pub struct DisplayBlanksAs {
  /// Display Blanks As Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<DisplayBlanksAsValues>,
}
/// Extensibility container.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartExtensionList/c:extLst")]
pub struct ChartExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Defines the DataDisplayOptions16 Class.
  #[sdk(child(office2019, qname = "c16r3:CT_DataDisplayOptions16/c:ext"))]
  pub c_ext: Vec<DataDisplayOptions16>,
}
/// Defines the EditingLanguage Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_TextLanguageID/c:lang")]
pub struct EditingLanguage {
  /// Language Code
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the Style Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Style/c:style")]
pub struct Style {
  /// Style Type
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_range(range = 1..= 48))]
  pub val: Option<crate::simple_type::ByteValue>,
}
/// Defines the ColorMapOverride Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ColorMapping/c:clrMapOvr")]
pub struct ColorMapOverride {
  /// Background 1
  #[sdk(attr(qname = ":bg1"))]
  #[sdk(string_format(kind = "token"))]
  pub background1:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Text 1
  #[sdk(attr(qname = ":tx1"))]
  #[sdk(string_format(kind = "token"))]
  pub text1: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Background 2
  #[sdk(attr(qname = ":bg2"))]
  #[sdk(string_format(kind = "token"))]
  pub background2:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Text 2
  #[sdk(attr(qname = ":tx2"))]
  #[sdk(string_format(kind = "token"))]
  pub text2: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 1
  #[sdk(attr(qname = ":accent1"))]
  #[sdk(string_format(kind = "token"))]
  pub accent1:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 2
  #[sdk(attr(qname = ":accent2"))]
  #[sdk(string_format(kind = "token"))]
  pub accent2:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 3
  #[sdk(attr(qname = ":accent3"))]
  #[sdk(string_format(kind = "token"))]
  pub accent3:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 4
  #[sdk(attr(qname = ":accent4"))]
  #[sdk(string_format(kind = "token"))]
  pub accent4:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 5
  #[sdk(attr(qname = ":accent5"))]
  #[sdk(string_format(kind = "token"))]
  pub accent5:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Accent 6
  #[sdk(attr(qname = ":accent6"))]
  #[sdk(string_format(kind = "token"))]
  pub accent6:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Hyperlink
  #[sdk(attr(qname = ":hlink"))]
  #[sdk(string_format(kind = "token"))]
  pub hyperlink:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Followed Hyperlink
  #[sdk(attr(qname = ":folHlink"))]
  #[sdk(string_format(kind = "token"))]
  pub followed_hyperlink:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the PivotSource Class.
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ExternalData/c:externalData")]
pub struct ExternalData {
  /// Relationship Reference
  #[sdk(attr(qname = "r:id"))]
  pub id: crate::simple_type::StringValue,
  /// Update Automatically
  #[sdk(child(qname = "c:CT_Boolean/c:autoUpdate"))]
  pub auto_update: Option<AutoUpdate>,
}
/// Defines the PrintSettings Class.
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartSpaceExtensionList/c:extLst")]
pub struct ChartSpaceExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the ChartSpaceExtension Class.
  #[sdk(child(qname = "c:CT_ChartSpaceExtension/c:ext"))]
  pub c_ext: Vec<ChartSpaceExtension>,
}
/// Defines the ChartSpaceExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_ChartSpaceExtension/c:ext")]
pub struct ChartSpaceExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// URI
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(choice(
    qname = "c14:CT_PivotOptions/c14:pivotOptions",
    qname = "c14:CT_SketchOptions/c14:sketchOptions",
    qname = "c:CT_PivotSource/c15:pivotSource",
    any
  ))]
  pub chart_space_extension_choice: Option<ChartSpaceExtensionChoice>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ChartShapePropertiesChoice {
  /// Custom geometry.
  #[sdk(child(qname = "a:CT_CustomGeometry2D/a:custGeom"))]
  ACustGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomGeometry>,
  ),
  /// Preset geometry.
  #[sdk(child(qname = "a:CT_PresetGeometry2D/a:prstGeom"))]
  APrstGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetGeometry>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ChartShapePropertiesChoice2 {
  /// Defines the NoFill Class.
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill>),
  /// Defines the SolidFill Class.
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill>,
  ),
  /// Defines the GradientFill Class.
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill>,
  ),
  /// Defines the BlipFill Class.
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill>,
  ),
  /// Pattern Fill.
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ChartShapePropertiesChoice3 {
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList>,
  ),
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag>,
  ),
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
  #[sdk(child(qname = "c:CT_Boolean/c:delete"))]
  CDelete(std::boxed::Box<Delete>),
  /// Sequence of c:numFmt, c:spPr, c:txPr, c:dLblPos, c:showLegendKey, c:showVal, c:showCatName, c:showSerName, c:showPercent, c:showBubbleSize, c:separator, c:showLeaderLines, c:leaderLines
  #[sdk(sequence)]
  Sequence(std::boxed::Box<DataLabelsChoiceSequence>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ChartSpaceChoice {
  #[sdk(child(office2010, qname = "c14:CT_Style/c14:style"))]
  C14Style(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::Style>,
  ),
  #[sdk(child(qname = "c:CT_Style/c:style"))]
  CStyle(std::boxed::Box<Style>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum UserShapesChoice {
  /// Relative Anchor Shape Size.
  #[sdk(child(qname = "cdr:CT_RelSizeAnchor/cdr:relSizeAnchor"))]
  CdrRelSizeAnchor(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart_drawing::RelativeAnchorSize,
    >,
  ),
  /// Absolute Anchor Shape Size.
  #[sdk(child(qname = "cdr:CT_AbsSizeAnchor/cdr:absSizeAnchor"))]
  CdrAbsSizeAnchor(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart_drawing::AbsoluteAnchorSize,
    >,
  ),
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
pub enum LegendEntryChoice {
  #[sdk(child(qname = "c:CT_Boolean/c:delete"))]
  CDelete(std::boxed::Box<Delete>),
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  Sequence(std::boxed::Box<TextProperties>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice {
  /// Custom geometry.
  #[sdk(child(qname = "a:CT_CustomGeometry2D/a:custGeom"))]
  ACustGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomGeometry>,
  ),
  /// Preset geometry.
  #[sdk(child(qname = "a:CT_PresetGeometry2D/a:prstGeom"))]
  APrstGeom(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetGeometry>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice2 {
  /// Defines the NoFill Class.
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill>),
  /// Defines the SolidFill Class.
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill>,
  ),
  /// Defines the GradientFill Class.
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill>,
  ),
  /// Defines the BlipFill Class.
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill>,
  ),
  /// Pattern Fill.
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
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList>,
  ),
  /// Effect Container.
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
  /// Defines the FilteredLineSeriesExtension Class.
  #[sdk(child(office2013, qname = "c15:CT_FilteredLineSer/c15:filteredLineSeries"))]
  C15FilteredLineSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredLineSeriesExtension,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PieChartExtensionChoice {
  /// Defines the FilteredPieSeries Class.
  #[sdk(child(office2013, qname = "c15:CT_FilteredPieSer/c15:filteredPieSeries"))]
  C15FilteredPieSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredPieSeries,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Pie3DChartExtensionChoice {
  /// Defines the FilteredPieSeries Class.
  #[sdk(child(office2013, qname = "c15:CT_FilteredPieSer/c15:filteredPieSeries"))]
  C15FilteredPieSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredPieSeries,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumRefExtensionChoice {
  /// Defines the FullReference Class.
  #[sdk(child(office2013, qname = "c15:CT_FullRef/c15:fullRef"))]
  C15FullRef(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FullReference>,
  ),
  /// Defines the LevelReference Class.
  #[sdk(child(office2013, qname = "c15:CT_LevelRef/c15:levelRef"))]
  C15LevelRef(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::LevelReference,
    >,
  ),
  /// Defines the FormulaReference Class.
  #[sdk(child(office2013, qname = "c15:CT_FormulaRef/c15:formulaRef"))]
  C15FormulaRef(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FormulaReference,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum StrDataExtensionChoice {
  /// Defines the AutoGeneneratedCategories Class.
  #[sdk(child(office2013, qname = "c:CT_Boolean/c15:autoCat"))]
  C15AutoCat(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::AutoGeneneratedCategories,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum StrRefExtensionChoice {
  /// Defines the FullReference Class.
  #[sdk(child(office2013, qname = "c15:CT_FullRef/c15:fullRef"))]
  C15FullRef(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FullReference>,
  ),
  /// Defines the LevelReference Class.
  #[sdk(child(office2013, qname = "c15:CT_LevelRef/c15:levelRef"))]
  C15LevelRef(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::LevelReference,
    >,
  ),
  /// Defines the FormulaReference Class.
  #[sdk(child(office2013, qname = "c15:CT_FormulaRef/c15:formulaRef"))]
  C15FormulaRef(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FormulaReference,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MultiLvlStrRefExtensionChoice {
  /// Defines the FullReference Class.
  #[sdk(child(office2013, qname = "c15:CT_FullRef/c15:fullRef"))]
  C15FullRef(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FullReference>,
  ),
  /// Defines the LevelReference Class.
  #[sdk(child(office2013, qname = "c15:CT_LevelRef/c15:levelRef"))]
  C15LevelRef(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::LevelReference,
    >,
  ),
  /// Defines the FormulaReference Class.
  #[sdk(child(office2013, qname = "c15:CT_FormulaRef/c15:formulaRef"))]
  C15FormulaRef(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FormulaReference,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DLblsExtensionChoice {
  /// Defines the ChartText Class.
  #[sdk(child(office2013, qname = "c:CT_Tx/c15:tx"))]
  C15Tx(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ChartText>,
  ),
  /// Defines the DataLabelFieldTable Class.
  #[sdk(child(office2013, qname = "c15:CT_DataLabelFieldTable/c15:dlblFieldTable"))]
  C15DlblFieldTable(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelFieldTable,
    >,
  ),
  /// Defines the ShowDataLabelsRange Class.
  #[sdk(child(office2013, qname = "c:CT_Boolean/c15:showDataLabelsRange"))]
  C15ShowDataLabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShowDataLabelsRange,
    >,
  ),
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/c15:spPr"))]
  C15SpPr(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShapeProperties,
    >,
  ),
  /// Defines the Layout Class.
  #[sdk(child(office2013, qname = "c:CT_Layout/c15:layout"))]
  C15Layout(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::Layout>,
  ),
  /// Defines the ShowLeaderLines Class.
  #[sdk(child(office2013, qname = "c:CT_Boolean/c15:showLeaderLines"))]
  C15ShowLeaderLines(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShowLeaderLines,
    >,
  ),
  /// Defines the LeaderLines Class.
  #[sdk(child(office2013, qname = "c:CT_ChartLines/c15:leaderLines"))]
  C15LeaderLines(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::LeaderLines>,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LineChartExtensionChoice {
  /// Defines the FilteredLineSeriesExtension Class.
  #[sdk(child(office2013, qname = "c15:CT_FilteredLineSer/c15:filteredLineSeries"))]
  C15FilteredLineSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredLineSeriesExtension,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Line3DChartExtensionChoice {
  /// Defines the FilteredLineSeriesExtension Class.
  #[sdk(child(office2013, qname = "c15:CT_FilteredLineSer/c15:filteredLineSeries"))]
  C15FilteredLineSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredLineSeriesExtension,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ScatterChartExtensionChoice {
  /// Defines the FilteredScatterSeries Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredScatterSer/c15:filteredScatterSeries"
  ))]
  C15FilteredScatterSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredScatterSeries,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RadarChartExtensionChoice {
  /// Defines the FilteredRadarSeries Class.
  #[sdk(child(office2013, qname = "c15:CT_FilteredRadarSer/c15:filteredRadarSeries"))]
  C15FilteredRadarSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredRadarSeries,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BarChartExtensionChoice {
  /// Defines the FilteredBarSeries Class.
  #[sdk(child(office2013, qname = "c15:CT_FilteredBarSer/c15:filteredBarSeries"))]
  C15FilteredBarSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredBarSeries,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Bar3DChartExtensionChoice {
  /// Defines the FilteredBarSeries Class.
  #[sdk(child(office2013, qname = "c15:CT_FilteredBarSer/c15:filteredBarSeries"))]
  C15FilteredBarSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredBarSeries,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AreaChartExtensionChoice {
  /// Defines the FilteredAreaSeries Class.
  #[sdk(child(office2013, qname = "c15:CT_FilteredAreaSer/c15:filteredAreaSeries"))]
  C15FilteredAreaSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredAreaSeries,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Area3DChartExtensionChoice {
  /// Defines the FilteredAreaSeries Class.
  #[sdk(child(office2013, qname = "c15:CT_FilteredAreaSer/c15:filteredAreaSeries"))]
  C15FilteredAreaSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredAreaSeries,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BubbleChartExtensionChoice {
  /// Defines the FilteredBubbleSeries Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredBubbleSer/c15:filteredBubbleSeries"
  ))]
  C15FilteredBubbleSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredBubbleSeries,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SurfaceChartExtensionChoice {
  /// Defines the FilteredSurfaceSeries Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredSurfaceSer/c15:filteredSurfaceSeries"
  ))]
  C15FilteredSurfaceSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSurfaceSeries,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum Surface3DChartExtensionChoice {
  /// Defines the FilteredSurfaceSeries Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredSurfaceSer/c15:filteredSurfaceSeries"
  ))]
  C15FilteredSurfaceSeries(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSurfaceSeries,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CatAxExtensionChoice {
  /// Defines the NumberingFormat Class.
  #[sdk(child(office2013, qname = "c:CT_NumFmt/c15:numFmt"))]
  C15NumFmt(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::NumberingFormat,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DateAxExtensionChoice {
  /// Defines the NumberingFormat Class.
  #[sdk(child(office2013, qname = "c:CT_NumFmt/c15:numFmt"))]
  C15NumFmt(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::NumberingFormat,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SerAxExtensionChoice {
  /// Defines the NumberingFormat Class.
  #[sdk(child(office2013, qname = "c:CT_NumFmt/c15:numFmt"))]
  C15NumFmt(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::NumberingFormat,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ValAxExtensionChoice {
  /// Defines the NumberingFormat Class.
  #[sdk(child(office2013, qname = "c:CT_NumFmt/c15:numFmt"))]
  C15NumFmt(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::NumberingFormat,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
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
  /// Defines the DataLabelFieldTable Class.
  #[sdk(child(office2013, qname = "c15:CT_DataLabelFieldTable/c15:dlblFieldTable"))]
  C15DlblFieldTable(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelFieldTable,
    >,
  ),
  /// Defines the ExceptionForSave Class.
  #[sdk(child(office2013, qname = "c:CT_Boolean/c15:xForSave"))]
  C15XForSave(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ExceptionForSave,
    >,
  ),
  /// Defines the ShowDataLabelsRange Class.
  #[sdk(child(office2013, qname = "c:CT_Boolean/c15:showDataLabelsRange"))]
  C15ShowDataLabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShowDataLabelsRange,
    >,
  ),
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/c15:spPr"))]
  C15SpPr(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShapeProperties,
    >,
  ),
  /// Defines the Layout Class.
  #[sdk(child(office2013, qname = "c:CT_Layout/c15:layout"))]
  C15Layout(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::Layout>,
  ),
  /// Defines the UniqueIdChartUniqueID Class.
  #[sdk(child(office2016, qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
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
pub enum LineSerExtensionChoice {
  /// Defines the FilteredSeriesTitle Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle"
  ))]
  C15FilteredSeriesTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    >,
  ),
  /// Defines the FilteredCategoryTitle Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle"
  ))]
  C15FilteredCategoryTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    >,
  ),
  /// Defines the DataLabelsRange Class.
  #[sdk(child(office2013, qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange"))]
  C15DatalabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    >,
  ),
  /// Defines the CategoryFilterExceptions Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions"
  ))]
  C15CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    >,
  ),
  /// Defines the CategoryFilterExceptions Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"
  ))]
  C16CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    >,
  ),
  /// Defines the ChartDataPointUniqueIDMap Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap"
  ))]
  C16Datapointuniqueidmap(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    >,
  ),
  /// Defines the UniqueIdChartUniqueID Class.
  #[sdk(child(office2016, qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ScatterSerExtensionChoice {
  /// Defines the FilteredSeriesTitle Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle"
  ))]
  C15FilteredSeriesTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    >,
  ),
  /// Defines the FilteredCategoryTitle Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle"
  ))]
  C15FilteredCategoryTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    >,
  ),
  /// Defines the DataLabelsRange Class.
  #[sdk(child(office2013, qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange"))]
  C15DatalabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    >,
  ),
  /// Defines the CategoryFilterExceptions Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions"
  ))]
  C15CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    >,
  ),
  /// Defines the CategoryFilterExceptions Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"
  ))]
  C16CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    >,
  ),
  /// Defines the ChartDataPointUniqueIDMap Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap"
  ))]
  C16Datapointuniqueidmap(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    >,
  ),
  /// Defines the UniqueIdChartUniqueID Class.
  #[sdk(child(office2016, qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RadarSerExtensionChoice {
  /// Defines the FilteredSeriesTitle Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle"
  ))]
  C15FilteredSeriesTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    >,
  ),
  /// Defines the FilteredCategoryTitle Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle"
  ))]
  C15FilteredCategoryTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    >,
  ),
  /// Defines the DataLabelsRange Class.
  #[sdk(child(office2013, qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange"))]
  C15DatalabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    >,
  ),
  /// Defines the CategoryFilterExceptions Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions"
  ))]
  C15CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    >,
  ),
  /// Defines the CategoryFilterExceptions Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"
  ))]
  C16CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    >,
  ),
  /// Defines the ChartDataPointUniqueIDMap Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap"
  ))]
  C16Datapointuniqueidmap(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    >,
  ),
  /// Defines the UniqueIdChartUniqueID Class.
  #[sdk(child(office2016, qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BarSerExtensionChoice {
  /// Defines the InvertSolidFillFormat Class.
  #[sdk(child(office2010, qname = "c14:CT_InvertSolidFillFmt/c14:invertSolidFillFmt"))]
  C14InvertSolidFillFmt(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::InvertSolidFillFormat,
    >,
  ),
  /// Defines the FilteredSeriesTitle Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle"
  ))]
  C15FilteredSeriesTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    >,
  ),
  /// Defines the FilteredCategoryTitle Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle"
  ))]
  C15FilteredCategoryTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    >,
  ),
  /// Defines the DataLabelsRange Class.
  #[sdk(child(office2013, qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange"))]
  C15DatalabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    >,
  ),
  /// Defines the CategoryFilterExceptions Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions"
  ))]
  C15CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    >,
  ),
  /// Defines the CategoryFilterExceptions Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"
  ))]
  C16CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    >,
  ),
  /// Defines the ChartDataPointUniqueIDMap Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap"
  ))]
  C16Datapointuniqueidmap(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    >,
  ),
  /// Defines the UniqueIdChartUniqueID Class.
  #[sdk(child(office2016, qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AreaSerExtensionChoice {
  /// Defines the FilteredSeriesTitle Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle"
  ))]
  C15FilteredSeriesTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    >,
  ),
  /// Defines the FilteredCategoryTitle Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle"
  ))]
  C15FilteredCategoryTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    >,
  ),
  /// Defines the DataLabelsRange Class.
  #[sdk(child(office2013, qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange"))]
  C15DatalabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    >,
  ),
  /// Defines the CategoryFilterExceptions Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions"
  ))]
  C15CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    >,
  ),
  /// Defines the CategoryFilterExceptions Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"
  ))]
  C16CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    >,
  ),
  /// Defines the ChartDataPointUniqueIDMap Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap"
  ))]
  C16Datapointuniqueidmap(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    >,
  ),
  /// Defines the UniqueIdChartUniqueID Class.
  #[sdk(child(office2016, qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PieSerExtensionChoice {
  /// Defines the FilteredSeriesTitle Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle"
  ))]
  C15FilteredSeriesTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    >,
  ),
  /// Defines the FilteredCategoryTitle Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle"
  ))]
  C15FilteredCategoryTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    >,
  ),
  /// Defines the DataLabelsRange Class.
  #[sdk(child(office2013, qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange"))]
  C15DatalabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    >,
  ),
  /// Defines the CategoryFilterExceptions Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions"
  ))]
  C15CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    >,
  ),
  /// Defines the CategoryFilterExceptions Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"
  ))]
  C16CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    >,
  ),
  /// Defines the ChartDataPointUniqueIDMap Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap"
  ))]
  C16Datapointuniqueidmap(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    >,
  ),
  /// Defines the UniqueIdChartUniqueID Class.
  #[sdk(child(office2016, qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BubbleSerExtensionChoice {
  /// Defines the InvertSolidFillFormat Class.
  #[sdk(child(office2010, qname = "c14:CT_InvertSolidFillFmt/c14:invertSolidFillFmt"))]
  C14InvertSolidFillFmt(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::InvertSolidFillFormat,
    >,
  ),
  /// Defines the FilteredCategoryTitle Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle"
  ))]
  C15FilteredCategoryTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    >,
  ),
  /// Defines the DataLabelsRange Class.
  #[sdk(child(office2013, qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange"))]
  C15DatalabelsRange(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    >,
  ),
  /// Defines the CategoryFilterExceptions Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions"
  ))]
  C15CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    >,
  ),
  /// Defines the CategoryFilterExceptions Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"
  ))]
  C16CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    >,
  ),
  /// Defines the ChartDataPointUniqueIDMap Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap"
  ))]
  C16Datapointuniqueidmap(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    >,
  ),
  /// Defines the UniqueIdChartUniqueID Class.
  #[sdk(child(office2016, qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SurfaceSerExtensionChoice {
  /// Defines the FilteredSeriesTitle Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle"
  ))]
  C15FilteredSeriesTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    >,
  ),
  /// Defines the FilteredCategoryTitle Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle"
  ))]
  C15FilteredCategoryTitle(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    >,
  ),
  /// Defines the CategoryFilterExceptions Class.
  #[sdk(child(
    office2013,
    qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions"
  ))]
  C15CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    >,
  ),
  /// Defines the CategoryFilterExceptions Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"
  ))]
  C16CategoryFilterExceptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    >,
  ),
  /// Defines the ChartDataPointUniqueIDMap Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap"
  ))]
  C16Datapointuniqueidmap(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    >,
  ),
  /// Defines the UniqueIdChartUniqueID Class.
  #[sdk(child(office2016, qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  C16UniqueId(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    >,
  ),
  #[sdk(any)]
  XmlOther(String),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PlotAreaChoice {
  /// Area Charts.
  #[sdk(child(qname = "c:CT_AreaChart/c:areaChart"))]
  CAreaChart(std::boxed::Box<AreaChart>),
  /// 3D Area Charts.
  #[sdk(child(qname = "c:CT_Area3DChart/c:area3DChart"))]
  CArea3DChart(std::boxed::Box<Area3DChart>),
  /// Line Charts.
  #[sdk(child(qname = "c:CT_LineChart/c:lineChart"))]
  CLineChart(std::boxed::Box<LineChart>),
  /// 3D Line Charts.
  #[sdk(child(qname = "c:CT_Line3DChart/c:line3DChart"))]
  CLine3DChart(std::boxed::Box<Line3DChart>),
  /// Stock Charts.
  #[sdk(child(qname = "c:CT_StockChart/c:stockChart"))]
  CStockChart(std::boxed::Box<StockChart>),
  /// Radar Charts.
  #[sdk(child(qname = "c:CT_RadarChart/c:radarChart"))]
  CRadarChart(std::boxed::Box<RadarChart>),
  /// Scatter Charts.
  #[sdk(child(qname = "c:CT_ScatterChart/c:scatterChart"))]
  CScatterChart(std::boxed::Box<ScatterChart>),
  /// Pie Charts.
  #[sdk(child(qname = "c:CT_PieChart/c:pieChart"))]
  CPieChart(std::boxed::Box<PieChart>),
  /// 3D Pie Charts.
  #[sdk(child(qname = "c:CT_Pie3DChart/c:pie3DChart"))]
  CPie3DChart(std::boxed::Box<Pie3DChart>),
  /// Doughnut Charts.
  #[sdk(child(qname = "c:CT_DoughnutChart/c:doughnutChart"))]
  CDoughnutChart(std::boxed::Box<DoughnutChart>),
  /// Bar Charts.
  #[sdk(child(qname = "c:CT_BarChart/c:barChart"))]
  CBarChart(std::boxed::Box<BarChart>),
  /// 3D Bar Charts.
  #[sdk(child(qname = "c:CT_Bar3DChart/c:bar3DChart"))]
  CBar3DChart(std::boxed::Box<Bar3DChart>),
  /// Pie of Pie or Bar of Pie Charts.
  #[sdk(child(qname = "c:CT_OfPieChart/c:ofPieChart"))]
  COfPieChart(std::boxed::Box<OfPieChart>),
  /// Surface Charts.
  #[sdk(child(qname = "c:CT_SurfaceChart/c:surfaceChart"))]
  CSurfaceChart(std::boxed::Box<SurfaceChart>),
  /// 3D Surface Charts.
  #[sdk(child(qname = "c:CT_Surface3DChart/c:surface3DChart"))]
  CSurface3DChart(std::boxed::Box<Surface3DChart>),
  /// Bubble Charts.
  #[sdk(child(qname = "c:CT_BubbleChart/c:bubbleChart"))]
  CBubbleChart(std::boxed::Box<BubbleChart>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum PlotAreaChoice2 {
  /// Value Axis.
  #[sdk(child(qname = "c:CT_ValAx/c:valAx"))]
  CValAx(std::boxed::Box<ValueAxis>),
  /// Category Axis Data.
  #[sdk(child(qname = "c:CT_CatAx/c:catAx"))]
  CCatAx(std::boxed::Box<CategoryAxis>),
  /// Date Axis.
  #[sdk(child(qname = "c:CT_DateAx/c:dateAx"))]
  CDateAx(std::boxed::Box<DateAxis>),
  /// Series Axis.
  #[sdk(child(qname = "c:CT_SerAx/c:serAx"))]
  CSerAx(std::boxed::Box<SeriesAxis>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ChartSpaceExtensionChoice {
  /// Defines the PivotOptions Class.
  #[sdk(child(office2010, qname = "c14:CT_PivotOptions/c14:pivotOptions"))]
  C14PivotOptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::PivotOptions,
    >,
  ),
  /// Defines the SketchOptions Class.
  #[sdk(child(office2010, qname = "c14:CT_SketchOptions/c14:sketchOptions"))]
  C14SketchOptions(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::SketchOptions,
    >,
  ),
  /// Defines the PivotSource Class.
  #[sdk(child(office2013, qname = "c:CT_PivotSource/c15:pivotSource"))]
  C15PivotSource(
    std::boxed::Box<crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::PivotSource>,
  ),
  #[sdk(any)]
  XmlOther(String),
}
