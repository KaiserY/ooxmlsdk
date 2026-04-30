//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ColorStyleMethodEnum {
  #[sdk(rename = "cycle")]
  #[default]
  Cycle,
  #[sdk(rename = "withinLinear")]
  WithinLinear,
  #[sdk(rename = "acrossLinear")]
  AcrossLinear,
  #[sdk(rename = "withinLinearReversed")]
  WithinLinearReversed,
  #[sdk(rename = "acrossLinearReversed")]
  AcrossLinearReversed,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StyleReferenceModifierEnum {
  #[sdk(rename = "ignoreCSTransforms")]
  #[default]
  IgnoreCsTransforms,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StyleColorEnum {
  #[sdk(rename = "auto")]
  #[default]
  Automatic,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StyleEntryModifierEnum {
  #[sdk(rename = "allowNoFillOverride")]
  #[default]
  AllowNoFillOverride,
  #[sdk(rename = "allowNoLineOverride")]
  AllowNoLineOverride,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum MarkerStyle {
  #[sdk(rename = "circle")]
  #[default]
  Circle,
  #[sdk(rename = "dash")]
  Dash,
  #[sdk(rename = "diamond")]
  Diamond,
  #[sdk(rename = "dot")]
  Dot,
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
pub enum Boolean {
  #[sdk(rename = "false")]
  #[default]
  False,
  #[sdk(rename = "true")]
  True,
  #[sdk(rename = "ninch")]
  Ninch,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TickMarkNinch {
  #[sdk(rename = "cross")]
  #[default]
  Cross,
  #[sdk(rename = "inside")]
  Inside,
  #[sdk(rename = "none")]
  None,
  #[sdk(rename = "outside")]
  Outside,
  #[sdk(rename = "ninch")]
  Ninch,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TickLabelPositionNinch {
  #[sdk(rename = "high")]
  #[default]
  High,
  #[sdk(rename = "low")]
  Low,
  #[sdk(rename = "nextToAxis")]
  NextToAxis,
  #[sdk(rename = "none")]
  None,
  #[sdk(rename = "ninch")]
  Ninch,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DataLabelsPosition {
  #[sdk(rename = "center")]
  #[default]
  Center,
  #[sdk(rename = "insideEnd")]
  InsideEnd,
  #[sdk(rename = "insideBase")]
  InsideBase,
  #[sdk(rename = "outsideEnd")]
  OutsideEnd,
  #[sdk(rename = "ninch")]
  Ninch,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum LegendPosition {
  #[sdk(rename = "right")]
  #[default]
  Right,
  #[sdk(rename = "top")]
  Top,
  #[sdk(rename = "left")]
  Left,
  #[sdk(rename = "bottom")]
  Bottom,
  #[sdk(rename = "ninch")]
  Ninch,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TitlePosition {
  #[sdk(rename = "above")]
  #[default]
  Above,
  #[sdk(rename = "overlay")]
  Overlay,
  #[sdk(rename = "off")]
  Off,
  #[sdk(rename = "ninch")]
  Ninch,
}
/// Defines the ColorStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_ColorStyle/cs:colorStyle")]
pub struct ColorStyle {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// meth
  #[sdk(attr(office2013, qname = ":meth"))]
  #[sdk(
        string_set(
            source = 1u32,
            union = 0u64,
            values = &["cycle",
            "withinLinear",
            "acrossLinear",
            "withinLinearReversed",
            "acrossLinearReversed"]
        )
    )]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub method: crate::simple_type::StringValue,
  /// id
  #[sdk(attr(office2013, qname = ":id"))]
  pub id: Option<crate::simple_type::UInt32Value>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub color_style_choice: Vec<ColorStyleChoice>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_ColorStyleVariation/cs:variation"))]
  pub cs_variation: Vec<ColorStyleVariation>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub cs_ext_lst: Option<OfficeArtExtensionList>,
}
/// Defines the ChartStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_ChartStyle/cs:chartStyle")]
pub struct ChartStyle {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// id
  #[sdk(attr(office2013, qname = ":id"))]
  pub id: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:axisTitle"))]
  pub axis_title: std::boxed::Box<AxisTitle>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:categoryAxis"))]
  pub category_axis: std::boxed::Box<CategoryAxis>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:chartArea"))]
  pub chart_area: std::boxed::Box<ChartArea>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:dataLabel"))]
  pub data_label: std::boxed::Box<DataLabel>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:dataLabelCallout"))]
  pub data_label_callout: Option<std::boxed::Box<DataLabelCallout>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:dataPoint"))]
  pub data_point: std::boxed::Box<DataPoint>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:dataPoint3D"))]
  pub data_point3_d: std::boxed::Box<DataPoint3D>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:dataPointLine"))]
  pub data_point_line: std::boxed::Box<DataPointLine>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:dataPointMarker"))]
  pub data_point_marker: std::boxed::Box<DataPointMarker>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_MarkerLayout/cs:dataPointMarkerLayout"))]
  pub marker_layout_properties: Option<MarkerLayoutProperties>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:dataPointWireframe"))]
  pub data_point_wireframe: std::boxed::Box<DataPointWireframe>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:dataTable"))]
  pub data_table_style: std::boxed::Box<DataTableStyle>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:downBar"))]
  pub down_bar: std::boxed::Box<DownBar>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:dropLine"))]
  pub drop_line: std::boxed::Box<DropLine>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:errorBar"))]
  pub error_bar: std::boxed::Box<ErrorBar>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:floor"))]
  pub floor: std::boxed::Box<Floor>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:gridlineMajor"))]
  pub gridline_major: std::boxed::Box<GridlineMajor>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:gridlineMinor"))]
  pub gridline_minor: std::boxed::Box<GridlineMinor>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:hiLoLine"))]
  pub hi_lo_line: std::boxed::Box<HiLoLine>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:leaderLine"))]
  pub leader_line: std::boxed::Box<LeaderLine>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:legend"))]
  pub legend_style: std::boxed::Box<LegendStyle>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:plotArea"))]
  pub plot_area: std::boxed::Box<PlotArea>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:plotArea3D"))]
  pub plot_area3_d: std::boxed::Box<PlotArea3D>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:seriesAxis"))]
  pub series_axis: std::boxed::Box<SeriesAxis>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:seriesLine"))]
  pub series_line: std::boxed::Box<SeriesLine>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:title"))]
  pub title_style: std::boxed::Box<TitleStyle>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:trendline"))]
  pub trendline_style: std::boxed::Box<TrendlineStyle>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:trendlineLabel"))]
  pub trendline_label: std::boxed::Box<TrendlineLabel>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:upBar"))]
  pub up_bar: std::boxed::Box<UpBar>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:valueAxis"))]
  pub value_axis: std::boxed::Box<ValueAxis>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleEntry/cs:wall"))]
  pub wall: std::boxed::Box<Wall>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ColorStyleVariation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_ColorStyleVariation/cs:variation")]
pub struct ColorStyleVariation {
  #[sdk(choice(
    qname = "a:CT_PositiveFixedPercentage/a:tint",
    qname = "a:CT_PositiveFixedPercentage/a:shade",
    qname = "a:CT_ComplementTransform/a:comp",
    qname = "a:CT_InverseTransform/a:inv",
    qname = "a:CT_GrayscaleTransform/a:gray",
    qname = "a:CT_PositiveFixedPercentage/a:alpha",
    qname = "a:CT_FixedPercentage/a:alphaOff",
    qname = "a:CT_PositivePercentage/a:alphaMod",
    qname = "a:CT_PositiveFixedAngle/a:hue",
    qname = "a:CT_Angle/a:hueOff",
    qname = "a:CT_PositivePercentage/a:hueMod",
    qname = "a:CT_Percentage/a:sat",
    qname = "a:CT_Percentage/a:satOff",
    qname = "a:CT_Percentage/a:satMod",
    qname = "a:CT_Percentage/a:lum",
    qname = "a:CT_Percentage/a:lumOff",
    qname = "a:CT_Percentage/a:lumMod",
    qname = "a:CT_Percentage/a:red",
    qname = "a:CT_Percentage/a:redOff",
    qname = "a:CT_Percentage/a:redMod",
    qname = "a:CT_Percentage/a:green",
    qname = "a:CT_Percentage/a:greenOff",
    qname = "a:CT_Percentage/a:greenMod",
    qname = "a:CT_Percentage/a:blue",
    qname = "a:CT_Percentage/a:blueOff",
    qname = "a:CT_Percentage/a:blueMod",
    qname = "a:CT_GammaTransform/a:gamma",
    qname = "a:CT_InverseGammaTransform/a:invGamma"
  ))]
  pub color_style_variation_choice: Vec<ColorStyleVariationChoice>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst")]
pub struct OfficeArtExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
/// Defines the StyleColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleColor/cs:styleClr")]
pub struct StyleColor {
  /// val
  #[sdk(attr(office2013, qname = ":val"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub val: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "a:CT_PositiveFixedPercentage/a:tint",
    qname = "a:CT_PositiveFixedPercentage/a:shade",
    qname = "a:CT_ComplementTransform/a:comp",
    qname = "a:CT_InverseTransform/a:inv",
    qname = "a:CT_GrayscaleTransform/a:gray",
    qname = "a:CT_PositiveFixedPercentage/a:alpha",
    qname = "a:CT_FixedPercentage/a:alphaOff",
    qname = "a:CT_PositivePercentage/a:alphaMod",
    qname = "a:CT_PositiveFixedAngle/a:hue",
    qname = "a:CT_Angle/a:hueOff",
    qname = "a:CT_PositivePercentage/a:hueMod",
    qname = "a:CT_Percentage/a:sat",
    qname = "a:CT_Percentage/a:satOff",
    qname = "a:CT_Percentage/a:satMod",
    qname = "a:CT_Percentage/a:lum",
    qname = "a:CT_Percentage/a:lumOff",
    qname = "a:CT_Percentage/a:lumMod",
    qname = "a:CT_Percentage/a:red",
    qname = "a:CT_Percentage/a:redOff",
    qname = "a:CT_Percentage/a:redMod",
    qname = "a:CT_Percentage/a:green",
    qname = "a:CT_Percentage/a:greenOff",
    qname = "a:CT_Percentage/a:greenMod",
    qname = "a:CT_Percentage/a:blue",
    qname = "a:CT_Percentage/a:blueOff",
    qname = "a:CT_Percentage/a:blueMod",
    qname = "a:CT_GammaTransform/a:gamma",
    qname = "a:CT_InverseGammaTransform/a:invGamma"
  ))]
  pub style_color_choice: Vec<StyleColorChoice>,
}
/// Defines the LineReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleReference/cs:lnRef")]
pub struct LineReference {
  /// idx
  #[sdk(attr(office2013, qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub line_reference_choice: Option<LineReferenceChoice>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleColor/cs:styleClr"))]
  pub cs_style_clr: Option<StyleColor>,
}
/// Defines the FillReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleReference/cs:fillRef")]
pub struct FillReference {
  /// idx
  #[sdk(attr(office2013, qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub fill_reference_choice: Option<FillReferenceChoice>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleColor/cs:styleClr"))]
  pub cs_style_clr: Option<StyleColor>,
}
/// Defines the EffectReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleReference/cs:effectRef")]
pub struct EffectReference {
  /// idx
  #[sdk(attr(office2013, qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub effect_reference_choice: Option<EffectReferenceChoice>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleColor/cs:styleClr"))]
  pub cs_style_clr: Option<StyleColor>,
}
/// Defines the LineWidthScale Class.
pub type LineWidthScale = crate::simple_type::DoubleValue;
/// Defines the FontReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_FontReference/cs:fontRef")]
pub struct FontReference {
  /// idx
  #[sdk(attr(office2013, qname = ":idx"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub index:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FontCollectionIndexValues,
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub font_reference_choice: Option<FontReferenceChoice>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleColor/cs:styleClr"))]
  pub cs_style_clr: Option<StyleColor>,
}
/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "a:CT_ShapeProperties/cs:spPr")]
pub struct ShapeProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Black and White Mode
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
/// Defines the TextCharacterPropertiesType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr")]
pub struct TextCharacterPropertiesType {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// kumimoji
  #[sdk(attr(qname = ":kumimoji"))]
  pub kumimoji: Option<crate::simple_type::BooleanValue>,
  /// lang
  #[sdk(attr(qname = ":lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// altLang
  #[sdk(attr(qname = ":altLang"))]
  pub alternative_language: Option<crate::simple_type::StringValue>,
  /// sz
  #[sdk(attr(qname = ":sz"))]
  #[sdk(number_range(
    source = 0u32,
    min = "100",
    max = "400000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub font_size: Option<crate::simple_type::Int32Value>,
  /// b
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// i
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// u
  #[sdk(attr(qname = ":u"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub underline:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextUnderlineValues>,
  /// strike
  #[sdk(attr(qname = ":strike"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub strike:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextStrikeValues>,
  /// kern
  #[sdk(attr(qname = ":kern"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "400000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub kerning: Option<crate::simple_type::Int32Value>,
  /// cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub capital:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextCapsValues>,
  /// spc
  #[sdk(attr(qname = ":spc"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-400000",
    max = "400000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub spacing: Option<crate::simple_type::Int32Value>,
  /// normalizeH
  #[sdk(attr(qname = ":normalizeH"))]
  pub normalize_height: Option<crate::simple_type::BooleanValue>,
  /// baseline
  #[sdk(attr(qname = ":baseline"))]
  pub baseline: Option<crate::simple_type::Int32Value>,
  /// noProof
  #[sdk(attr(qname = ":noProof"))]
  pub no_proof: Option<crate::simple_type::BooleanValue>,
  /// dirty
  #[sdk(attr(qname = ":dirty"))]
  pub dirty: Option<crate::simple_type::BooleanValue>,
  /// err
  #[sdk(attr(qname = ":err"))]
  pub spelling_error: Option<crate::simple_type::BooleanValue>,
  /// smtClean
  #[sdk(attr(qname = ":smtClean"))]
  pub smart_tag_clean: Option<crate::simple_type::BooleanValue>,
  /// smtId
  #[sdk(attr(qname = ":smtId"))]
  pub smart_tag_id: Option<crate::simple_type::UInt32Value>,
  /// bmk
  #[sdk(attr(qname = ":bmk"))]
  pub bookmark: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub outline: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline>,
  >,
  #[sdk(choice(
    qname = "a:CT_NoFillProperties/a:noFill",
    qname = "a:CT_SolidColorFillProperties/a:solidFill",
    qname = "a:CT_GradientFillProperties/a:gradFill",
    qname = "a:CT_BlipFillProperties/a:blipFill",
    qname = "a:CT_PatternFillProperties/a:pattFill",
    qname = "a:CT_GroupFillProperties/a:grpFill"
  ))]
  pub text_character_properties_type_choice1: Option<TextCharacterPropertiesTypeChoice>,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub text_character_properties_type_choice2: Option<TextCharacterPropertiesTypeChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_Color/a:highlight"))]
  pub a_highlight: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Highlight>,
  >,
  #[sdk(choice(
    qname = "a:CT_TextUnderlineLineFollowText/a:uLnTx",
    qname = "a:CT_LineProperties/a:uLn"
  ))]
  pub text_character_properties_type_choice3: Option<TextCharacterPropertiesTypeChoice3>,
  #[sdk(choice(
    qname = "a:CT_TextUnderlineFillFollowText/a:uFillTx",
    qname = "a:CT_TextUnderlineFillGroupWrapper/a:uFill"
  ))]
  pub text_character_properties_type_choice4: Option<TextCharacterPropertiesTypeChoice4>,
  /// _
  #[sdk(child(qname = "a:CT_TextFont/a:latin"))]
  pub a_latin: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LatinFont>,
  /// _
  #[sdk(child(qname = "a:CT_TextFont/a:ea"))]
  pub a_ea: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EastAsianFont>,
  /// _
  #[sdk(child(qname = "a:CT_TextFont/a:cs"))]
  pub a_cs:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ComplexScriptFont>,
  /// _
  #[sdk(child(qname = "a:CT_TextFont/a:sym"))]
  pub a_sym: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SymbolFont>,
  /// _
  #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkClick"))]
  pub a_hlink_click: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnClick,
    >,
  >,
  /// _
  #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkMouseOver"))]
  pub a_hlink_mouse_over: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnMouseOver,
    >,
  >,
  /// _
  #[sdk(child(qname = "a:CT_Bool/a:rtl"))]
  pub a_rtl: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RightToLeft>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the TextBodyProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr")]
pub struct TextBodyProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Rotation
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Paragraph Spacing
  #[sdk(attr(qname = ":spcFirstLastPara"))]
  pub use_paragraph_spacing: Option<crate::simple_type::BooleanValue>,
  /// Text Vertical Overflow
  #[sdk(attr(qname = ":vertOverflow"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub vertical_overflow: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextVerticalOverflowValues,
  >,
  /// Text Horizontal Overflow
  #[sdk(attr(qname = ":horzOverflow"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub horizontal_overflow: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextHorizontalOverflowValues,
  >,
  /// Vertical Text
  #[sdk(attr(qname = ":vert"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub vertical:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextVerticalValues>,
  /// Text Wrapping Type
  #[sdk(attr(qname = ":wrap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub wrap:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextWrappingValues>,
  /// Left Inset
  #[sdk(attr(qname = ":lIns"))]
  pub left_inset: Option<crate::simple_type::Int32Value>,
  /// Top Inset
  #[sdk(attr(qname = ":tIns"))]
  pub top_inset: Option<crate::simple_type::Int32Value>,
  /// Right Inset
  #[sdk(attr(qname = ":rIns"))]
  pub right_inset: Option<crate::simple_type::Int32Value>,
  /// Bottom Inset
  #[sdk(attr(qname = ":bIns"))]
  pub bottom_inset: Option<crate::simple_type::Int32Value>,
  /// Number of Columns
  #[sdk(attr(qname = ":numCol"))]
  #[sdk(number_range(
    source = 0u32,
    min = "1",
    max = "16",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub column_count: Option<crate::simple_type::Int32Value>,
  /// Space Between Columns
  #[sdk(attr(qname = ":spcCol"))]
  #[sdk(number_range(source = 0u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub column_spacing: Option<crate::simple_type::Int32Value>,
  /// Columns Right-To-Left
  #[sdk(attr(qname = ":rtlCol"))]
  pub right_to_left_columns: Option<crate::simple_type::BooleanValue>,
  /// From WordArt
  #[sdk(attr(qname = ":fromWordArt"))]
  pub from_word_art: Option<crate::simple_type::BooleanValue>,
  /// Anchor
  #[sdk(attr(qname = ":anchor"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub anchor:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextAnchoringTypeValues>,
  /// Anchor Center
  #[sdk(attr(qname = ":anchorCtr"))]
  pub anchor_center: Option<crate::simple_type::BooleanValue>,
  /// Force Anti-Alias
  #[sdk(attr(qname = ":forceAA"))]
  pub force_anti_alias: Option<crate::simple_type::BooleanValue>,
  /// Text Upright
  #[sdk(attr(qname = ":upright"))]
  pub up_right: Option<crate::simple_type::BooleanValue>,
  /// Compatible Line Spacing
  #[sdk(attr(qname = ":compatLnSpc"))]
  pub compatible_line_spacing: Option<crate::simple_type::BooleanValue>,
  /// Preset Text Shape
  #[sdk(child(qname = "a:CT_PresetTextShape/a:prstTxWarp"))]
  pub preset_text_warp: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetTextWarp>,
  >,
  #[sdk(choice(
    qname = "a:CT_TextNoAutofit/a:noAutofit",
    qname = "a:CT_TextNormalAutofit/a:normAutofit",
    qname = "a:CT_TextShapeAutofit/a:spAutoFit"
  ))]
  pub text_body_properties_choice1: Option<TextBodyPropertiesChoice>,
  /// _
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType>,
  >,
  #[sdk(choice(qname = "a:CT_Shape3D/a:sp3d", qname = "a:CT_FlatText/a:flatTx"))]
  pub text_body_properties_choice2: Option<TextBodyPropertiesChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/a:extLst"))]
  pub a_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList>,
}
/// Defines the CategoryAxisProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_AxisProps/cs:categoryAxis")]
pub struct CategoryAxisProperties {
  /// visible
  #[sdk(attr(office2013, qname = ":visible"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub visible: Option<Boolean>,
  /// majorTick
  #[sdk(attr(office2013, qname = ":majorTick"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub major_tick: Option<TickMarkNinch>,
  /// minorTick
  #[sdk(attr(office2013, qname = ":minorTick"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub minor_tick_prop: Option<TickMarkNinch>,
  /// labelPosition
  #[sdk(attr(office2013, qname = ":labelPosition"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub label_position: Option<TickLabelPositionNinch>,
  /// majorGridlines
  #[sdk(attr(office2013, qname = ":majorGridlines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub major_gridlines: Option<Boolean>,
  /// minorGridlines
  #[sdk(attr(office2013, qname = ":minorGridlines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub minor_gridlines_prop: Option<Boolean>,
  /// title
  #[sdk(attr(office2013, qname = ":title"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub title_prop: Option<Boolean>,
}
/// Defines the SeriesAxisProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_AxisProps/cs:seriesAxis")]
pub struct SeriesAxisProperties {
  /// visible
  #[sdk(attr(office2013, qname = ":visible"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub visible: Option<Boolean>,
  /// majorTick
  #[sdk(attr(office2013, qname = ":majorTick"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub major_tick: Option<TickMarkNinch>,
  /// minorTick
  #[sdk(attr(office2013, qname = ":minorTick"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub minor_tick_prop: Option<TickMarkNinch>,
  /// labelPosition
  #[sdk(attr(office2013, qname = ":labelPosition"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub label_position: Option<TickLabelPositionNinch>,
  /// majorGridlines
  #[sdk(attr(office2013, qname = ":majorGridlines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub major_gridlines: Option<Boolean>,
  /// minorGridlines
  #[sdk(attr(office2013, qname = ":minorGridlines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub minor_gridlines_prop: Option<Boolean>,
  /// title
  #[sdk(attr(office2013, qname = ":title"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub title_prop: Option<Boolean>,
}
/// Defines the ValueAxisProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_AxisProps/cs:valueAxis")]
pub struct ValueAxisProperties {
  /// visible
  #[sdk(attr(office2013, qname = ":visible"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub visible: Option<Boolean>,
  /// majorTick
  #[sdk(attr(office2013, qname = ":majorTick"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub major_tick: Option<TickMarkNinch>,
  /// minorTick
  #[sdk(attr(office2013, qname = ":minorTick"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub minor_tick_prop: Option<TickMarkNinch>,
  /// labelPosition
  #[sdk(attr(office2013, qname = ":labelPosition"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub label_position: Option<TickLabelPositionNinch>,
  /// majorGridlines
  #[sdk(attr(office2013, qname = ":majorGridlines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub major_gridlines: Option<Boolean>,
  /// minorGridlines
  #[sdk(attr(office2013, qname = ":minorGridlines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub minor_gridlines_prop: Option<Boolean>,
  /// title
  #[sdk(attr(office2013, qname = ":title"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub title_prop: Option<Boolean>,
}
/// Defines the DataSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_DataSeriesProps/cs:dataSeries")]
pub struct DataSeries {
  /// overlap
  #[sdk(attr(office2013, qname = ":overlap"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-100",
    max = "100",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub overlap: Option<crate::simple_type::SByteValue>,
  /// gapWidth
  #[sdk(attr(office2013, qname = ":gapWidth"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "500",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub gap_width: Option<crate::simple_type::UInt16Value>,
  /// gapDepth
  #[sdk(attr(office2013, qname = ":gapDepth"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "500",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub gap_depth: Option<crate::simple_type::UInt16Value>,
  /// doughnutHoleSize
  #[sdk(attr(office2013, qname = ":doughnutHoleSize"))]
  #[sdk(number_range(
    source = 0u32,
    min = "10",
    max = "90",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub doughnut_hole_size: Option<crate::simple_type::ByteValue>,
  /// markerVisible
  #[sdk(attr(office2013, qname = ":markerVisible"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub marker_visible: Option<Boolean>,
  /// hiloLines
  #[sdk(attr(office2013, qname = ":hiloLines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub hilo_lines: Option<Boolean>,
  /// dropLines
  #[sdk(attr(office2013, qname = ":dropLines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub drop_lines: Option<Boolean>,
  /// seriesLines
  #[sdk(attr(office2013, qname = ":seriesLines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub series_lines: Option<Boolean>,
}
/// Defines the DataLabels Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_DataLabelsProps/cs:dataLabels")]
pub struct DataLabels {
  /// position
  #[sdk(attr(office2013, qname = ":position"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub position: Option<DataLabelsPosition>,
  /// value
  #[sdk(attr(office2013, qname = ":value"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub value: Option<Boolean>,
  /// seriesName
  #[sdk(attr(office2013, qname = ":seriesName"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub series_name: Option<Boolean>,
  /// categoryName
  #[sdk(attr(office2013, qname = ":categoryName"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub category_name: Option<Boolean>,
  /// legendKey
  #[sdk(attr(office2013, qname = ":legendKey"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub legend_key: Option<Boolean>,
  /// percentage
  #[sdk(attr(office2013, qname = ":percentage"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub percentage: Option<Boolean>,
}
/// Defines the DataTable Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_DataTableProps/cs:dataTable")]
pub struct DataTable {
  /// legendKeys
  #[sdk(attr(office2013, qname = ":legendKeys"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub legend_keys: Option<Boolean>,
  /// horizontalBorder
  #[sdk(attr(office2013, qname = ":horizontalBorder"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub horizontal_border: Option<Boolean>,
  /// verticalBorder
  #[sdk(attr(office2013, qname = ":verticalBorder"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub vertical_border: Option<Boolean>,
  /// outlineBorder
  #[sdk(attr(office2013, qname = ":outlineBorder"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub outline_border: Option<Boolean>,
}
/// Defines the Legend Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_LegendProps/cs:legend")]
pub struct Legend {
  /// visible
  #[sdk(attr(office2013, qname = ":visible"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub visible: Option<Boolean>,
  /// includeInLayout
  #[sdk(attr(office2013, qname = ":includeInLayout"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub include_in_layout: Option<Boolean>,
  /// position
  #[sdk(attr(office2013, qname = ":position"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub position: Option<LegendPosition>,
}
/// Defines the Title Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_TitleProps/cs:title")]
pub struct Title {
  /// position
  #[sdk(attr(office2013, qname = ":position"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub position: Option<TitlePosition>,
}
/// Defines the Trendline Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_TrendlineProps/cs:trendline")]
pub struct Trendline {
  /// add
  #[sdk(attr(office2013, qname = ":add"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub add: Option<Boolean>,
  /// equation
  #[sdk(attr(office2013, qname = ":equation"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub equation: Option<Boolean>,
  /// rsquared
  #[sdk(attr(office2013, qname = ":rsquared"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub r_squared: Option<Boolean>,
}
/// Defines the View3DProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_View3DProps/cs:view3D")]
pub struct View3DProperties {
  /// rotX
  #[sdk(attr(office2013, qname = ":rotX"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-90",
    max = "90",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub rot_x: Option<crate::simple_type::SByteValue>,
  /// rotY
  #[sdk(attr(office2013, qname = ":rotY"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "360",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub rot_y: Option<crate::simple_type::UInt16Value>,
  /// rAngAx
  #[sdk(attr(office2013, qname = ":rAngAx"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub right_angle_axes: Option<Boolean>,
  /// perspective
  #[sdk(attr(office2013, qname = ":perspective"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "240",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub perspective: Option<crate::simple_type::ByteValue>,
  /// heightPercent
  #[sdk(attr(office2013, qname = ":heightPercent"))]
  #[sdk(number_range(
    source = 0u32,
    min = "5",
    max = "500",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub height_percent: Option<crate::simple_type::UInt16Value>,
  /// depthPercent
  #[sdk(attr(office2013, qname = ":depthPercent"))]
  #[sdk(number_range(
    source = 0u32,
    min = "20",
    max = "2000",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub depth_percent: Option<crate::simple_type::UInt16Value>,
}
/// Defines the AxisTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:axisTitle")]
pub struct AxisTitle {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the CategoryAxis Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:categoryAxis")]
pub struct CategoryAxis {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ChartArea Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:chartArea")]
pub struct ChartArea {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataLabel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:dataLabel")]
pub struct DataLabel {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataLabelCallout Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:dataLabelCallout")]
pub struct DataLabelCallout {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPoint Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:dataPoint")]
pub struct DataPoint {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPoint3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:dataPoint3D")]
pub struct DataPoint3D {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPointLine Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:dataPointLine")]
pub struct DataPointLine {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPointMarker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:dataPointMarker")]
pub struct DataPointMarker {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPointWireframe Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:dataPointWireframe")]
pub struct DataPointWireframe {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataTableStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:dataTable")]
pub struct DataTableStyle {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DownBar Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:downBar")]
pub struct DownBar {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DropLine Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:dropLine")]
pub struct DropLine {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ErrorBar Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:errorBar")]
pub struct ErrorBar {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the Floor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:floor")]
pub struct Floor {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the GridlineMajor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:gridlineMajor")]
pub struct GridlineMajor {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the GridlineMinor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:gridlineMinor")]
pub struct GridlineMinor {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the HiLoLine Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:hiLoLine")]
pub struct HiLoLine {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the LeaderLine Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:leaderLine")]
pub struct LeaderLine {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the LegendStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:legend")]
pub struct LegendStyle {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PlotArea Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:plotArea")]
pub struct PlotArea {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PlotArea3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:plotArea3D")]
pub struct PlotArea3D {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the SeriesAxis Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:seriesAxis")]
pub struct SeriesAxis {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the SeriesLine Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:seriesLine")]
pub struct SeriesLine {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the TitleStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:title")]
pub struct TitleStyle {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the TrendlineStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:trendline")]
pub struct TrendlineStyle {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the TrendlineLabel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:trendlineLabel")]
pub struct TrendlineLabel {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the UpBar Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:upBar")]
pub struct UpBar {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ValueAxis Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:valueAxis")]
pub struct ValueAxis {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the Wall Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_StyleEntry/cs:wall")]
pub struct Wall {
  /// mods
  #[sdk(attr(office2013, qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(office2013, qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the MarkerLayoutProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "cs:CT_MarkerLayout/cs:dataPointMarkerLayout")]
pub struct MarkerLayoutProperties {
  /// symbol
  #[sdk(attr(office2013, qname = ":symbol"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub symbol: Option<MarkerStyle>,
  /// size
  #[sdk(attr(office2013, qname = ":size"))]
  #[sdk(number_range(
    source = 0u32,
    min = "2",
    max = "72",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub size: Option<crate::simple_type::ByteValue>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ColorStyleChoice {
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ColorStyleVariationChoice {
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:tint"))]
  ATint(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Tint>),
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:shade"))]
  AShade(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shade>),
  /// Complement.
  #[sdk(empty_child(qname = "a:CT_ComplementTransform/a:comp"))]
  AComp,
  /// Inverse.
  #[sdk(empty_child(qname = "a:CT_InverseTransform/a:inv"))]
  AInv,
  /// Gray.
  #[sdk(empty_child(qname = "a:CT_GrayscaleTransform/a:gray"))]
  AGray,
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:alpha"))]
  AAlpha(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Alpha>),
  #[sdk(child(qname = "a:CT_FixedPercentage/a:alphaOff"))]
  AAlphaOff(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaOffset>,
  ),
  #[sdk(child(qname = "a:CT_PositivePercentage/a:alphaMod"))]
  AAlphaMod(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaModulation,
    >,
  ),
  #[sdk(child(qname = "a:CT_PositiveFixedAngle/a:hue"))]
  AHue(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Hue>),
  #[sdk(child(qname = "a:CT_Angle/a:hueOff"))]
  AHueOff(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HueOffset>,
  ),
  #[sdk(child(qname = "a:CT_PositivePercentage/a:hueMod"))]
  AHueMod(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HueModulation>,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:sat"))]
  ASat(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Saturation>),
  #[sdk(child(qname = "a:CT_Percentage/a:satOff"))]
  ASatOff(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SaturationOffset,
    >,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:satMod"))]
  ASatMod(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SaturationModulation,
    >,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:lum"))]
  ALum(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Luminance>),
  #[sdk(child(qname = "a:CT_Percentage/a:lumOff"))]
  ALumOff(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LuminanceOffset,
    >,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:lumMod"))]
  ALumMod(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LuminanceModulation,
    >,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:red"))]
  ARed(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Red>),
  #[sdk(child(qname = "a:CT_Percentage/a:redOff"))]
  ARedOff(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RedOffset>,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:redMod"))]
  ARedMod(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RedModulation>,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:green"))]
  AGreen(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Green>),
  #[sdk(child(qname = "a:CT_Percentage/a:greenOff"))]
  AGreenOff(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GreenOffset>,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:greenMod"))]
  AGreenMod(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GreenModulation,
    >,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:blue"))]
  ABlue(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Blue>),
  #[sdk(child(qname = "a:CT_Percentage/a:blueOff"))]
  ABlueOff(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlueOffset>,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:blueMod"))]
  ABlueMod(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlueModulation>,
  ),
  /// Gamma.
  #[sdk(empty_child(qname = "a:CT_GammaTransform/a:gamma"))]
  AGamma,
  /// Inverse Gamma.
  #[sdk(empty_child(qname = "a:CT_InverseGammaTransform/a:invGamma"))]
  AInvGamma,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum StyleColorChoice {
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:tint"))]
  ATint(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Tint>),
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:shade"))]
  AShade(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shade>),
  /// Complement.
  #[sdk(empty_child(qname = "a:CT_ComplementTransform/a:comp"))]
  AComp,
  /// Inverse.
  #[sdk(empty_child(qname = "a:CT_InverseTransform/a:inv"))]
  AInv,
  /// Gray.
  #[sdk(empty_child(qname = "a:CT_GrayscaleTransform/a:gray"))]
  AGray,
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:alpha"))]
  AAlpha(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Alpha>),
  #[sdk(child(qname = "a:CT_FixedPercentage/a:alphaOff"))]
  AAlphaOff(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaOffset>,
  ),
  #[sdk(child(qname = "a:CT_PositivePercentage/a:alphaMod"))]
  AAlphaMod(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaModulation,
    >,
  ),
  #[sdk(child(qname = "a:CT_PositiveFixedAngle/a:hue"))]
  AHue(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Hue>),
  #[sdk(child(qname = "a:CT_Angle/a:hueOff"))]
  AHueOff(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HueOffset>,
  ),
  #[sdk(child(qname = "a:CT_PositivePercentage/a:hueMod"))]
  AHueMod(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HueModulation>,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:sat"))]
  ASat(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Saturation>),
  #[sdk(child(qname = "a:CT_Percentage/a:satOff"))]
  ASatOff(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SaturationOffset,
    >,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:satMod"))]
  ASatMod(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SaturationModulation,
    >,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:lum"))]
  ALum(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Luminance>),
  #[sdk(child(qname = "a:CT_Percentage/a:lumOff"))]
  ALumOff(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LuminanceOffset,
    >,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:lumMod"))]
  ALumMod(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LuminanceModulation,
    >,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:red"))]
  ARed(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Red>),
  #[sdk(child(qname = "a:CT_Percentage/a:redOff"))]
  ARedOff(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RedOffset>,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:redMod"))]
  ARedMod(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RedModulation>,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:green"))]
  AGreen(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Green>),
  #[sdk(child(qname = "a:CT_Percentage/a:greenOff"))]
  AGreenOff(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GreenOffset>,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:greenMod"))]
  AGreenMod(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GreenModulation,
    >,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:blue"))]
  ABlue(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Blue>),
  #[sdk(child(qname = "a:CT_Percentage/a:blueOff"))]
  ABlueOff(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlueOffset>,
  ),
  #[sdk(child(qname = "a:CT_Percentage/a:blueMod"))]
  ABlueMod(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlueModulation>,
  ),
  /// Gamma.
  #[sdk(empty_child(qname = "a:CT_GammaTransform/a:gamma"))]
  AGamma,
  /// Inverse Gamma.
  #[sdk(empty_child(qname = "a:CT_InverseGammaTransform/a:invGamma"))]
  AInvGamma,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LineReferenceChoice {
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FillReferenceChoice {
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EffectReferenceChoice {
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FontReferenceChoice {
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextCharacterPropertiesTypeChoice {
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
pub enum TextCharacterPropertiesTypeChoice2 {
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
pub enum TextCharacterPropertiesTypeChoice3 {
  /// Underline Follows Text.
  #[sdk(empty_child(qname = "a:CT_TextUnderlineLineFollowText/a:uLnTx"))]
  AULnTx,
  #[sdk(child(qname = "a:CT_LineProperties/a:uLn"))]
  AULn(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Underline>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextCharacterPropertiesTypeChoice4 {
  /// Underline Fill Properties Follow Text.
  #[sdk(empty_child(qname = "a:CT_TextUnderlineFillFollowText/a:uFillTx"))]
  AUFillTx,
  #[sdk(child(qname = "a:CT_TextUnderlineFillGroupWrapper/a:uFill"))]
  AUFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::UnderlineFill>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextBodyPropertiesChoice {
  /// No AutoFit.
  #[sdk(empty_child(qname = "a:CT_TextNoAutofit/a:noAutofit"))]
  ANoAutofit,
  #[sdk(child(qname = "a:CT_TextNormalAutofit/a:normAutofit"))]
  ANormAutofit(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NormalAutoFit>,
  ),
  #[sdk(child(qname = "a:CT_TextShapeAutofit/a:spAutoFit"))]
  ASpAutoFit(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapeAutoFit>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextBodyPropertiesChoice2 {
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  ASp3d(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType>,
  ),
  #[sdk(child(qname = "a:CT_FlatText/a:flatTx"))]
  AFlatTx(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FlatText>,
  ),
}
