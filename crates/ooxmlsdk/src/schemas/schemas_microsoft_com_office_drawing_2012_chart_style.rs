//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum StyleReferenceModifierEnum {
  #[sdk(rename = "ignoreCSTransforms")]
  #[default]
  IgnoreCsTransforms,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum StyleColorEnum {
  #[sdk(rename = "auto")]
  #[default]
  Automatic,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum StyleEntryModifierEnum {
  #[sdk(rename = "allowNoFillOverride")]
  #[default]
  AllowNoFillOverride,
  #[sdk(rename = "allowNoLineOverride")]
  AllowNoLineOverride,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum Boolean {
  #[sdk(rename = "false")]
  #[default]
  False,
  #[sdk(rename = "true")]
  True,
  #[sdk(rename = "ninch")]
  Ninch,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:colorStyle.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_ColorStyle/cs:colorStyle")]
pub struct ColorStyle {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// meth
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :meth
  #[sdk(attr(qname = ":meth"))]
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
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
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
  #[sdk(child(qname = "cs:CT_ColorStyleVariation/cs:variation"))]
  pub cs_variation: Vec<ColorStyleVariation>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub cs_ext_lst: Option<OfficeArtExtensionList>,
}
/// Defines the ChartStyle Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:chartStyle.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_ChartStyle/cs:chartStyle")]
pub struct ChartStyle {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// id
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:axisTitle"))]
  pub axis_title: std::boxed::Box<AxisTitle>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:categoryAxis"))]
  pub category_axis: std::boxed::Box<CategoryAxis>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:chartArea"))]
  pub chart_area: std::boxed::Box<ChartArea>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:dataLabel"))]
  pub data_label: std::boxed::Box<DataLabel>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:dataLabelCallout"))]
  pub data_label_callout: Option<std::boxed::Box<DataLabelCallout>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:dataPoint"))]
  pub data_point: std::boxed::Box<DataPoint>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:dataPoint3D"))]
  pub data_point3_d: std::boxed::Box<DataPoint3D>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:dataPointLine"))]
  pub data_point_line: std::boxed::Box<DataPointLine>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:dataPointMarker"))]
  pub data_point_marker: std::boxed::Box<DataPointMarker>,
  /// _
  #[sdk(child(qname = "cs:CT_MarkerLayout/cs:dataPointMarkerLayout"))]
  pub marker_layout_properties: Option<MarkerLayoutProperties>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:dataPointWireframe"))]
  pub data_point_wireframe: std::boxed::Box<DataPointWireframe>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:dataTable"))]
  pub data_table_style: std::boxed::Box<DataTableStyle>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:downBar"))]
  pub down_bar: std::boxed::Box<DownBar>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:dropLine"))]
  pub drop_line: std::boxed::Box<DropLine>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:errorBar"))]
  pub error_bar: std::boxed::Box<ErrorBar>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:floor"))]
  pub floor: std::boxed::Box<Floor>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:gridlineMajor"))]
  pub gridline_major: std::boxed::Box<GridlineMajor>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:gridlineMinor"))]
  pub gridline_minor: std::boxed::Box<GridlineMinor>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:hiLoLine"))]
  pub hi_lo_line: std::boxed::Box<HiLoLine>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:leaderLine"))]
  pub leader_line: std::boxed::Box<LeaderLine>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:legend"))]
  pub legend_style: std::boxed::Box<LegendStyle>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:plotArea"))]
  pub plot_area: std::boxed::Box<PlotArea>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:plotArea3D"))]
  pub plot_area3_d: std::boxed::Box<PlotArea3D>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:seriesAxis"))]
  pub series_axis: std::boxed::Box<SeriesAxis>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:seriesLine"))]
  pub series_line: std::boxed::Box<SeriesLine>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:title"))]
  pub title_style: std::boxed::Box<TitleStyle>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:trendline"))]
  pub trendline_style: std::boxed::Box<TrendlineStyle>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:trendlineLabel"))]
  pub trendline_label: std::boxed::Box<TrendlineLabel>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:upBar"))]
  pub up_bar: std::boxed::Box<UpBar>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:valueAxis"))]
  pub value_axis: std::boxed::Box<ValueAxis>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleEntry/cs:wall"))]
  pub wall: std::boxed::Box<Wall>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ColorStyleVariation Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:variation.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_ColorStyleVariation/cs:variation")]
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
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeArtExtensionList/cs:extLst")]
pub struct OfficeArtExtensionList {
  ///Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
/// Defines the StyleColor Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:styleClr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleColor/cs:styleClr")]
pub struct StyleColor {
  /// val
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
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
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:lnRef.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleReference/cs:lnRef")]
pub struct LineReference {
  /// idx
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
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
  #[sdk(child(qname = "cs:CT_StyleColor/cs:styleClr"))]
  pub cs_style_clr: Option<StyleColor>,
}
/// Defines the FillReference Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:fillRef.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleReference/cs:fillRef")]
pub struct FillReference {
  /// idx
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
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
  #[sdk(child(qname = "cs:CT_StyleColor/cs:styleClr"))]
  pub cs_style_clr: Option<StyleColor>,
}
/// Defines the EffectReference Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:effectRef.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleReference/cs:effectRef")]
pub struct EffectReference {
  /// idx
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
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
  #[sdk(child(qname = "cs:CT_StyleColor/cs:styleClr"))]
  pub cs_style_clr: Option<StyleColor>,
}
/// Defines the StyleReference Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleReference/")]
pub struct StyleReference {
  /// idx
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr",
    qname = "cs:CT_StyleColor/cs:styleClr"
  ))]
  pub xml_children: Vec<StyleReferenceChoice>,
}
/// Defines the LineWidthScale Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:lineWidthScale.
pub type LineWidthScale = crate::simple_type::DoubleValue;
/// Defines the FontReference Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:fontRef.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_FontReference/cs:fontRef")]
pub struct FontReference {
  /// idx
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub index:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FontCollectionIndexValues,
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
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
  #[sdk(child(qname = "cs:CT_StyleColor/cs:styleClr"))]
  pub cs_style_clr: Option<StyleColor>,
}
/// Defines the ShapeProperties Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:spPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeProperties/cs:spPr")]
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
  ///2D Transform for Individual Objects
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
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:defRPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextCharacterProperties/cs:defRPr")]
pub struct TextCharacterPropertiesType {
  /// kumimoji
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :kumimoji
  #[sdk(attr(qname = ":kumimoji"))]
  pub kumimoji: Option<crate::simple_type::BooleanValue>,
  /// lang
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lang
  #[sdk(attr(qname = ":lang"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// altLang
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :altLang
  #[sdk(attr(qname = ":altLang"))]
  pub alternative_language: Option<crate::simple_type::StringValue>,
  /// sz
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sz
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :b
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// i
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :i
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// u
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :u
  #[sdk(attr(qname = ":u"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub underline:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextUnderlineValues>,
  /// strike
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :strike
  #[sdk(attr(qname = ":strike"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub strike:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextStrikeValues>,
  /// kern
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :kern
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub capital:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextCapsValues>,
  /// spc
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spc
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :normalizeH
  #[sdk(attr(qname = ":normalizeH"))]
  pub normalize_height: Option<crate::simple_type::BooleanValue>,
  /// baseline
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :baseline
  #[sdk(attr(qname = ":baseline"))]
  pub baseline: Option<crate::simple_type::Int32Value>,
  /// noProof
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :noProof
  #[sdk(attr(qname = ":noProof"))]
  pub no_proof: Option<crate::simple_type::BooleanValue>,
  /// dirty
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dirty
  #[sdk(attr(qname = ":dirty"))]
  pub dirty: Option<crate::simple_type::BooleanValue>,
  /// err
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :err
  #[sdk(attr(qname = ":err"))]
  pub spelling_error: Option<crate::simple_type::BooleanValue>,
  /// smtClean
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :smtClean
  #[sdk(attr(qname = ":smtClean"))]
  pub smart_tag_clean: Option<crate::simple_type::BooleanValue>,
  /// smtId
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :smtId
  #[sdk(attr(qname = ":smtId"))]
  pub smart_tag_id: Option<crate::simple_type::UInt32Value>,
  /// bmk
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bmk
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
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:bodyPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBodyProperties/cs:bodyPr")]
pub struct TextBodyProperties {
  /// Rotation
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rot
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Paragraph Spacing
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spcFirstLastPara
  #[sdk(attr(qname = ":spcFirstLastPara"))]
  pub use_paragraph_spacing: Option<crate::simple_type::BooleanValue>,
  /// Text Vertical Overflow
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vertOverflow
  #[sdk(attr(qname = ":vertOverflow"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub vertical_overflow: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextVerticalOverflowValues,
  >,
  /// Text Horizontal Overflow
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :horzOverflow
  #[sdk(attr(qname = ":horzOverflow"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub horizontal_overflow: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextHorizontalOverflowValues,
  >,
  /// Vertical Text
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vert
  #[sdk(attr(qname = ":vert"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub vertical:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextVerticalValues>,
  /// Text Wrapping Type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :wrap
  #[sdk(attr(qname = ":wrap"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub wrap:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextWrappingValues>,
  /// Left Inset
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :lIns
  #[sdk(attr(qname = ":lIns"))]
  pub left_inset: Option<crate::simple_type::Int32Value>,
  /// Top Inset
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tIns
  #[sdk(attr(qname = ":tIns"))]
  pub top_inset: Option<crate::simple_type::Int32Value>,
  /// Right Inset
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rIns
  #[sdk(attr(qname = ":rIns"))]
  pub right_inset: Option<crate::simple_type::Int32Value>,
  /// Bottom Inset
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :bIns
  #[sdk(attr(qname = ":bIns"))]
  pub bottom_inset: Option<crate::simple_type::Int32Value>,
  /// Number of Columns
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :numCol
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
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :spcCol
  #[sdk(attr(qname = ":spcCol"))]
  #[sdk(number_range(source = 0u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub column_spacing: Option<crate::simple_type::Int32Value>,
  /// Columns Right-To-Left
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :rtlCol
  #[sdk(attr(qname = ":rtlCol"))]
  pub right_to_left_columns: Option<crate::simple_type::BooleanValue>,
  /// From WordArt
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fromWordArt
  #[sdk(attr(qname = ":fromWordArt"))]
  pub from_word_art: Option<crate::simple_type::BooleanValue>,
  /// Anchor
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :anchor
  #[sdk(attr(qname = ":anchor"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub anchor:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextAnchoringTypeValues>,
  /// Anchor Center
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :anchorCtr
  #[sdk(attr(qname = ":anchorCtr"))]
  pub anchor_center: Option<crate::simple_type::BooleanValue>,
  /// Force Anti-Alias
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :forceAA
  #[sdk(attr(qname = ":forceAA"))]
  pub force_anti_alias: Option<crate::simple_type::BooleanValue>,
  /// Text Upright
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :upright
  #[sdk(attr(qname = ":upright"))]
  pub up_right: Option<crate::simple_type::BooleanValue>,
  /// Compatible Line Spacing
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :compatLnSpc
  #[sdk(attr(qname = ":compatLnSpc"))]
  pub compatible_line_spacing: Option<crate::simple_type::BooleanValue>,
  ///Preset Text Shape
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
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:categoryAxis.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_AxisProps/cs:categoryAxis")]
pub struct CategoryAxisProperties {
  /// visible
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub visible: Option<Boolean>,
  /// majorTick
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :majorTick
  #[sdk(attr(qname = ":majorTick"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub major_tick: Option<TickMarkNinch>,
  /// minorTick
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :minorTick
  #[sdk(attr(qname = ":minorTick"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub minor_tick_prop: Option<TickMarkNinch>,
  /// labelPosition
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :labelPosition
  #[sdk(attr(qname = ":labelPosition"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub label_position: Option<TickLabelPositionNinch>,
  /// majorGridlines
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :majorGridlines
  #[sdk(attr(qname = ":majorGridlines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub major_gridlines: Option<Boolean>,
  /// minorGridlines
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :minorGridlines
  #[sdk(attr(qname = ":minorGridlines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub minor_gridlines_prop: Option<Boolean>,
  /// title
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub title_prop: Option<Boolean>,
}
/// Defines the SeriesAxisProperties Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:seriesAxis.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_AxisProps/cs:seriesAxis")]
pub struct SeriesAxisProperties {
  /// visible
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub visible: Option<Boolean>,
  /// majorTick
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :majorTick
  #[sdk(attr(qname = ":majorTick"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub major_tick: Option<TickMarkNinch>,
  /// minorTick
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :minorTick
  #[sdk(attr(qname = ":minorTick"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub minor_tick_prop: Option<TickMarkNinch>,
  /// labelPosition
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :labelPosition
  #[sdk(attr(qname = ":labelPosition"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub label_position: Option<TickLabelPositionNinch>,
  /// majorGridlines
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :majorGridlines
  #[sdk(attr(qname = ":majorGridlines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub major_gridlines: Option<Boolean>,
  /// minorGridlines
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :minorGridlines
  #[sdk(attr(qname = ":minorGridlines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub minor_gridlines_prop: Option<Boolean>,
  /// title
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub title_prop: Option<Boolean>,
}
/// Defines the ValueAxisProperties Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:valueAxis.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_AxisProps/cs:valueAxis")]
pub struct ValueAxisProperties {
  /// visible
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub visible: Option<Boolean>,
  /// majorTick
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :majorTick
  #[sdk(attr(qname = ":majorTick"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub major_tick: Option<TickMarkNinch>,
  /// minorTick
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :minorTick
  #[sdk(attr(qname = ":minorTick"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub minor_tick_prop: Option<TickMarkNinch>,
  /// labelPosition
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :labelPosition
  #[sdk(attr(qname = ":labelPosition"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub label_position: Option<TickLabelPositionNinch>,
  /// majorGridlines
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :majorGridlines
  #[sdk(attr(qname = ":majorGridlines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub major_gridlines: Option<Boolean>,
  /// minorGridlines
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :minorGridlines
  #[sdk(attr(qname = ":minorGridlines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub minor_gridlines_prop: Option<Boolean>,
  /// title
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub title_prop: Option<Boolean>,
}
/// Defines the AxisProperties Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_AxisProps/")]
pub struct AxisProperties {
  /// visible
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub visible: Option<Boolean>,
  /// majorTick
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :majorTick
  #[sdk(attr(qname = ":majorTick"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub major_tick: Option<TickMarkNinch>,
  /// minorTick
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :minorTick
  #[sdk(attr(qname = ":minorTick"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub minor_tick_prop: Option<TickMarkNinch>,
  /// labelPosition
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :labelPosition
  #[sdk(attr(qname = ":labelPosition"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub label_position: Option<TickLabelPositionNinch>,
  /// majorGridlines
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :majorGridlines
  #[sdk(attr(qname = ":majorGridlines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub major_gridlines: Option<Boolean>,
  /// minorGridlines
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :minorGridlines
  #[sdk(attr(qname = ":minorGridlines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub minor_gridlines_prop: Option<Boolean>,
  /// title
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub title_prop: Option<Boolean>,
}
/// Defines the DataSeries Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:dataSeries.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_DataSeriesProps/cs:dataSeries")]
pub struct DataSeries {
  /// overlap
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :overlap
  #[sdk(attr(qname = ":overlap"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-100",
    max = "100",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub overlap: Option<crate::simple_type::SByteValue>,
  /// gapWidth
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :gapWidth
  #[sdk(attr(qname = ":gapWidth"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "500",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub gap_width: Option<crate::simple_type::UInt16Value>,
  /// gapDepth
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :gapDepth
  #[sdk(attr(qname = ":gapDepth"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "500",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub gap_depth: Option<crate::simple_type::UInt16Value>,
  /// doughnutHoleSize
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :doughnutHoleSize
  #[sdk(attr(qname = ":doughnutHoleSize"))]
  #[sdk(number_range(
    source = 0u32,
    min = "10",
    max = "90",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub doughnut_hole_size: Option<crate::simple_type::ByteValue>,
  /// markerVisible
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :markerVisible
  #[sdk(attr(qname = ":markerVisible"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub marker_visible: Option<Boolean>,
  /// hiloLines
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :hiloLines
  #[sdk(attr(qname = ":hiloLines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub hilo_lines: Option<Boolean>,
  /// dropLines
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :dropLines
  #[sdk(attr(qname = ":dropLines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub drop_lines: Option<Boolean>,
  /// seriesLines
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :seriesLines
  #[sdk(attr(qname = ":seriesLines"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub series_lines: Option<Boolean>,
}
/// Defines the DataLabels Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:dataLabels.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_DataLabelsProps/cs:dataLabels")]
pub struct DataLabels {
  /// position
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :position
  #[sdk(attr(qname = ":position"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub position: Option<DataLabelsPosition>,
  /// value
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :value
  #[sdk(attr(qname = ":value"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub value: Option<Boolean>,
  /// seriesName
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :seriesName
  #[sdk(attr(qname = ":seriesName"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub series_name: Option<Boolean>,
  /// categoryName
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :categoryName
  #[sdk(attr(qname = ":categoryName"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub category_name: Option<Boolean>,
  /// legendKey
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :legendKey
  #[sdk(attr(qname = ":legendKey"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub legend_key: Option<Boolean>,
  /// percentage
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :percentage
  #[sdk(attr(qname = ":percentage"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub percentage: Option<Boolean>,
}
/// Defines the DataTable Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:dataTable.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_DataTableProps/cs:dataTable")]
pub struct DataTable {
  /// legendKeys
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :legendKeys
  #[sdk(attr(qname = ":legendKeys"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub legend_keys: Option<Boolean>,
  /// horizontalBorder
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :horizontalBorder
  #[sdk(attr(qname = ":horizontalBorder"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub horizontal_border: Option<Boolean>,
  /// verticalBorder
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :verticalBorder
  #[sdk(attr(qname = ":verticalBorder"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub vertical_border: Option<Boolean>,
  /// outlineBorder
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :outlineBorder
  #[sdk(attr(qname = ":outlineBorder"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub outline_border: Option<Boolean>,
}
/// Defines the Legend Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:legend.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_LegendProps/cs:legend")]
pub struct Legend {
  /// visible
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :visible
  #[sdk(attr(qname = ":visible"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub visible: Option<Boolean>,
  /// includeInLayout
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :includeInLayout
  #[sdk(attr(qname = ":includeInLayout"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub include_in_layout: Option<Boolean>,
  /// position
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :position
  #[sdk(attr(qname = ":position"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub position: Option<LegendPosition>,
}
/// Defines the Title Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:title.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_TitleProps/cs:title")]
pub struct Title {
  /// position
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :position
  #[sdk(attr(qname = ":position"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub position: Option<TitlePosition>,
}
/// Defines the Trendline Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:trendline.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_TrendlineProps/cs:trendline")]
pub struct Trendline {
  /// add
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :add
  #[sdk(attr(qname = ":add"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub add: Option<Boolean>,
  /// equation
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :equation
  #[sdk(attr(qname = ":equation"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub equation: Option<Boolean>,
  /// rsquared
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :rsquared
  #[sdk(attr(qname = ":rsquared"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub r_squared: Option<Boolean>,
}
/// Defines the View3DProperties Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:view3D.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_View3DProps/cs:view3D")]
pub struct View3DProperties {
  /// rotX
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :rotX
  #[sdk(attr(qname = ":rotX"))]
  #[sdk(number_range(
    source = 0u32,
    min = "-90",
    max = "90",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub rot_x: Option<crate::simple_type::SByteValue>,
  /// rotY
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :rotY
  #[sdk(attr(qname = ":rotY"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "360",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub rot_y: Option<crate::simple_type::UInt16Value>,
  /// rAngAx
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :rAngAx
  #[sdk(attr(qname = ":rAngAx"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub right_angle_axes: Option<Boolean>,
  /// perspective
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :perspective
  #[sdk(attr(qname = ":perspective"))]
  #[sdk(number_range(
    source = 0u32,
    min = "0",
    max = "240",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub perspective: Option<crate::simple_type::ByteValue>,
  /// heightPercent
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :heightPercent
  #[sdk(attr(qname = ":heightPercent"))]
  #[sdk(number_range(
    source = 0u32,
    min = "5",
    max = "500",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub height_percent: Option<crate::simple_type::UInt16Value>,
  /// depthPercent
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :depthPercent
  #[sdk(attr(qname = ":depthPercent"))]
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
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:axisTitle.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:axisTitle")]
pub struct AxisTitle {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the CategoryAxis Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:categoryAxis.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:categoryAxis")]
pub struct CategoryAxis {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ChartArea Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:chartArea.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:chartArea")]
pub struct ChartArea {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataLabel Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:dataLabel.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:dataLabel")]
pub struct DataLabel {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataLabelCallout Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:dataLabelCallout.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:dataLabelCallout")]
pub struct DataLabelCallout {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPoint Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:dataPoint.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:dataPoint")]
pub struct DataPoint {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPoint3D Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:dataPoint3D.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:dataPoint3D")]
pub struct DataPoint3D {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPointLine Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:dataPointLine.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:dataPointLine")]
pub struct DataPointLine {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPointMarker Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:dataPointMarker.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:dataPointMarker")]
pub struct DataPointMarker {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPointWireframe Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:dataPointWireframe.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:dataPointWireframe")]
pub struct DataPointWireframe {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataTableStyle Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:dataTable.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:dataTable")]
pub struct DataTableStyle {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DownBar Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:downBar.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:downBar")]
pub struct DownBar {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DropLine Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:dropLine.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:dropLine")]
pub struct DropLine {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ErrorBar Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:errorBar.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:errorBar")]
pub struct ErrorBar {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the Floor Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:floor.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:floor")]
pub struct Floor {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the GridlineMajor Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:gridlineMajor.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:gridlineMajor")]
pub struct GridlineMajor {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the GridlineMinor Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:gridlineMinor.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:gridlineMinor")]
pub struct GridlineMinor {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the HiLoLine Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:hiLoLine.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:hiLoLine")]
pub struct HiLoLine {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the LeaderLine Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:leaderLine.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:leaderLine")]
pub struct LeaderLine {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the LegendStyle Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:legend.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:legend")]
pub struct LegendStyle {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PlotArea Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:plotArea.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:plotArea")]
pub struct PlotArea {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PlotArea3D Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:plotArea3D.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:plotArea3D")]
pub struct PlotArea3D {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the SeriesAxis Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:seriesAxis.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:seriesAxis")]
pub struct SeriesAxis {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the SeriesLine Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:seriesLine.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:seriesLine")]
pub struct SeriesLine {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the TitleStyle Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:title.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:title")]
pub struct TitleStyle {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the TrendlineStyle Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:trendline.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:trendline")]
pub struct TrendlineStyle {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the TrendlineLabel Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:trendlineLabel.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:trendlineLabel")]
pub struct TrendlineLabel {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the UpBar Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:upBar.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:upBar")]
pub struct UpBar {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ValueAxis Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:valueAxis.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:valueAxis")]
pub struct ValueAxis {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the Wall Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:wall.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/cs:wall")]
pub struct Wall {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// _
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  pub line_width_scale: Option<crate::simple_type::DoubleValue>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// _
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// _
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the StyleEntry Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_StyleEntry/")]
pub struct StyleEntry {
  /// mods
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :mods
  #[sdk(attr(qname = ":mods"))]
  pub modifiers: Option<crate::simple_type::ListValue<crate::simple_type::StringValue>>,
  #[sdk(choice(
    qname = "cs:CT_StyleReference/cs:lnRef",
    qname = "xsd:double/cs:lineWidthScale",
    qname = "cs:CT_StyleReference/cs:fillRef",
    qname = "cs:CT_StyleReference/cs:effectRef",
    qname = "cs:CT_FontReference/cs:fontRef",
    qname = "a:CT_ShapeProperties/cs:spPr",
    qname = "a:CT_TextCharacterProperties/cs:defRPr",
    qname = "a:CT_TextBodyProperties/cs:bodyPr",
    qname = "a:CT_OfficeArtExtensionList/cs:extLst"
  ))]
  pub xml_children: Vec<StyleEntryChoice>,
}
/// Defines the MarkerLayoutProperties Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is cs:dataPointMarkerLayout.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:CT_MarkerLayout/cs:dataPointMarkerLayout")]
pub struct MarkerLayoutProperties {
  /// symbol
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :symbol
  #[sdk(attr(qname = ":symbol"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub symbol: Option<MarkerStyle>,
  /// size
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :size
  #[sdk(attr(qname = ":size"))]
  #[sdk(number_range(
    source = 0u32,
    min = "2",
    max = "72",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub size: Option<crate::simple_type::ByteValue>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum ColorStyleVariationChoice {
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:tint"))]
  ATint(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Tint>),
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:shade"))]
  AShade(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shade>),
  #[sdk(child(qname = "a:CT_ComplementTransform/a:comp"))]
  AComp(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Complement>,
  ),
  #[sdk(child(qname = "a:CT_InverseTransform/a:inv"))]
  AInv(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Inverse>),
  #[sdk(child(qname = "a:CT_GrayscaleTransform/a:gray"))]
  AGray(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Gray>),
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
  #[sdk(child(qname = "a:CT_GammaTransform/a:gamma"))]
  AGamma(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Gamma>),
  #[sdk(child(qname = "a:CT_InverseGammaTransform/a:invGamma"))]
  AInvGamma(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::InverseGamma>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum StyleColorChoice {
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:tint"))]
  ATint(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Tint>),
  #[sdk(child(qname = "a:CT_PositiveFixedPercentage/a:shade"))]
  AShade(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shade>),
  #[sdk(child(qname = "a:CT_ComplementTransform/a:comp"))]
  AComp(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Complement>,
  ),
  #[sdk(child(qname = "a:CT_InverseTransform/a:inv"))]
  AInv(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Inverse>),
  #[sdk(child(qname = "a:CT_GrayscaleTransform/a:gray"))]
  AGray(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Gray>),
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
  #[sdk(child(qname = "a:CT_GammaTransform/a:gamma"))]
  AGamma(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Gamma>),
  #[sdk(child(qname = "a:CT_InverseGammaTransform/a:invGamma"))]
  AInvGamma(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::InverseGamma>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum StyleReferenceChoice {
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
  #[sdk(child(qname = "cs:CT_StyleColor/cs:styleClr"))]
  CsStyleClr(std::boxed::Box<StyleColor>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
  #[sdk(child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupFill>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
  #[sdk(child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupFill>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TextCharacterPropertiesTypeChoice3 {
  #[sdk(child(qname = "a:CT_TextUnderlineLineFollowText/a:uLnTx"))]
  AULnTx(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::UnderlineFollowsText,
    >,
  ),
  #[sdk(child(qname = "a:CT_LineProperties/a:uLn"))]
  AULn(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Underline>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TextCharacterPropertiesTypeChoice4 {
  #[sdk(child(qname = "a:CT_TextUnderlineFillFollowText/a:uFillTx"))]
  AUFillTx(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::UnderlineFillText,
    >,
  ),
  #[sdk(child(qname = "a:CT_TextUnderlineFillGroupWrapper/a:uFill"))]
  AUFill(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::UnderlineFill>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TextBodyPropertiesChoice {
  #[sdk(child(qname = "a:CT_TextNoAutofit/a:noAutofit"))]
  ANoAutofit(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoAutoFit>,
  ),
  #[sdk(child(qname = "a:CT_TextNormalAutofit/a:normAutofit"))]
  ANormAutofit(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NormalAutoFit>,
  ),
  #[sdk(child(qname = "a:CT_TextShapeAutofit/a:spAutoFit"))]
  ASpAutoFit(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapeAutoFit>,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
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
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum StyleEntryChoice {
  #[sdk(child(qname = "cs:CT_StyleReference/cs:lnRef"))]
  CsLnRef(std::boxed::Box<LineReference>),
  #[sdk(text_child(qname = "xsd:double/cs:lineWidthScale"))]
  CsLineWidthScale(crate::simple_type::DoubleValue),
  #[sdk(child(qname = "cs:CT_StyleReference/cs:fillRef"))]
  CsFillRef(std::boxed::Box<FillReference>),
  #[sdk(child(qname = "cs:CT_StyleReference/cs:effectRef"))]
  CsEffectRef(std::boxed::Box<EffectReference>),
  #[sdk(child(qname = "cs:CT_FontReference/cs:fontRef"))]
  CsFontRef(std::boxed::Box<FontReference>),
  #[sdk(child(qname = "a:CT_ShapeProperties/cs:spPr"))]
  CsSpPr(std::boxed::Box<ShapeProperties>),
  #[sdk(child(qname = "a:CT_TextCharacterProperties/cs:defRPr"))]
  CsDefRPr(std::boxed::Box<TextCharacterPropertiesType>),
  #[sdk(child(qname = "a:CT_TextBodyProperties/cs:bodyPr"))]
  CsBodyPr(std::boxed::Box<TextBodyProperties>),
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/cs:extLst"))]
  CsExtLst(std::boxed::Box<OfficeArtExtensionList>),
}
