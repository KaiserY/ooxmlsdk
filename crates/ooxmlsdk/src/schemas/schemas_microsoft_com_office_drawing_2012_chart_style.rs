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
#[sdk(qname = "cs:colorStyle")]
pub struct ColorStyle {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  /// meth
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
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::UInt32Value>,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub color_style_choice: Vec<ColorStyleChoice>,
  /// Defines the ColorStyleVariation Class.
  #[sdk(child(qname = "cs:variation"))]
  pub color_style_variation: Vec<ColorStyleVariation>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ChartStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:chartStyle")]
pub struct ChartStyle {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::UInt32Value>,
  /// Defines the AxisTitle Class.
  #[sdk(child(qname = "cs:axisTitle"))]
  pub axis_title: std::boxed::Box<AxisTitle>,
  /// Defines the CategoryAxis Class.
  #[sdk(child(qname = "cs:categoryAxis"))]
  pub category_axis: std::boxed::Box<CategoryAxis>,
  /// Defines the ChartArea Class.
  #[sdk(child(qname = "cs:chartArea"))]
  pub chart_area: std::boxed::Box<ChartArea>,
  /// Defines the DataLabel Class.
  #[sdk(child(qname = "cs:dataLabel"))]
  pub data_label: std::boxed::Box<DataLabel>,
  /// Defines the DataLabelCallout Class.
  #[sdk(child(qname = "cs:dataLabelCallout"))]
  pub data_label_callout: Option<std::boxed::Box<DataLabelCallout>>,
  /// Defines the DataPoint Class.
  #[sdk(child(qname = "cs:dataPoint"))]
  pub data_point: std::boxed::Box<DataPoint>,
  /// Defines the DataPoint3D Class.
  #[sdk(child(qname = "cs:dataPoint3D"))]
  pub data_point3_d: std::boxed::Box<DataPoint3D>,
  /// Defines the DataPointLine Class.
  #[sdk(child(qname = "cs:dataPointLine"))]
  pub data_point_line: std::boxed::Box<DataPointLine>,
  /// Defines the DataPointMarker Class.
  #[sdk(child(qname = "cs:dataPointMarker"))]
  pub data_point_marker: std::boxed::Box<DataPointMarker>,
  /// Defines the MarkerLayoutProperties Class.
  #[sdk(child(qname = "cs:dataPointMarkerLayout"))]
  pub marker_layout_properties: Option<MarkerLayoutProperties>,
  /// Defines the DataPointWireframe Class.
  #[sdk(child(qname = "cs:dataPointWireframe"))]
  pub data_point_wireframe: std::boxed::Box<DataPointWireframe>,
  /// Defines the DataTableStyle Class.
  #[sdk(child(qname = "cs:dataTable"))]
  pub data_table_style: std::boxed::Box<DataTableStyle>,
  /// Defines the DownBar Class.
  #[sdk(child(qname = "cs:downBar"))]
  pub down_bar: std::boxed::Box<DownBar>,
  /// Defines the DropLine Class.
  #[sdk(child(qname = "cs:dropLine"))]
  pub drop_line: std::boxed::Box<DropLine>,
  /// Defines the ErrorBar Class.
  #[sdk(child(qname = "cs:errorBar"))]
  pub error_bar: std::boxed::Box<ErrorBar>,
  /// Defines the Floor Class.
  #[sdk(child(qname = "cs:floor"))]
  pub floor: std::boxed::Box<Floor>,
  /// Defines the GridlineMajor Class.
  #[sdk(child(qname = "cs:gridlineMajor"))]
  pub gridline_major: std::boxed::Box<GridlineMajor>,
  /// Defines the GridlineMinor Class.
  #[sdk(child(qname = "cs:gridlineMinor"))]
  pub gridline_minor: std::boxed::Box<GridlineMinor>,
  /// Defines the HiLoLine Class.
  #[sdk(child(qname = "cs:hiLoLine"))]
  pub hi_lo_line: std::boxed::Box<HiLoLine>,
  /// Defines the LeaderLine Class.
  #[sdk(child(qname = "cs:leaderLine"))]
  pub leader_line: std::boxed::Box<LeaderLine>,
  /// Defines the LegendStyle Class.
  #[sdk(child(qname = "cs:legend"))]
  pub legend_style: std::boxed::Box<LegendStyle>,
  /// Defines the PlotArea Class.
  #[sdk(child(qname = "cs:plotArea"))]
  pub plot_area: std::boxed::Box<PlotArea>,
  /// Defines the PlotArea3D Class.
  #[sdk(child(qname = "cs:plotArea3D"))]
  pub plot_area3_d: std::boxed::Box<PlotArea3D>,
  /// Defines the SeriesAxis Class.
  #[sdk(child(qname = "cs:seriesAxis"))]
  pub series_axis: std::boxed::Box<SeriesAxis>,
  /// Defines the SeriesLine Class.
  #[sdk(child(qname = "cs:seriesLine"))]
  pub series_line: std::boxed::Box<SeriesLine>,
  /// Defines the TitleStyle Class.
  #[sdk(child(qname = "cs:title"))]
  pub title_style: std::boxed::Box<TitleStyle>,
  /// Defines the TrendlineStyle Class.
  #[sdk(child(qname = "cs:trendline"))]
  pub trendline_style: std::boxed::Box<TrendlineStyle>,
  /// Defines the TrendlineLabel Class.
  #[sdk(child(qname = "cs:trendlineLabel"))]
  pub trendline_label: std::boxed::Box<TrendlineLabel>,
  /// Defines the UpBar Class.
  #[sdk(child(qname = "cs:upBar"))]
  pub up_bar: std::boxed::Box<UpBar>,
  /// Defines the ValueAxis Class.
  #[sdk(child(qname = "cs:valueAxis"))]
  pub value_axis: std::boxed::Box<ValueAxis>,
  /// Defines the Wall Class.
  #[sdk(child(qname = "cs:wall"))]
  pub wall: std::boxed::Box<Wall>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ColorStyleVariation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:variation")]
pub struct ColorStyleVariation {
  #[sdk(
        choice(
            child(variant = Tint, qname = "a:tint"),
            child(variant = Shade, qname = "a:shade"),
            empty_child(variant = Complement, qname = "a:comp"),
            empty_child(variant = Inverse, qname = "a:inv"),
            empty_child(variant = Gray, qname = "a:gray"),
            child(variant = Alpha, qname = "a:alpha"),
            child(variant = AlphaOffset, qname = "a:alphaOff"),
            child(variant = AlphaModulation, qname = "a:alphaMod"),
            child(variant = Hue, qname = "a:hue"),
            child(variant = HueOffset, qname = "a:hueOff"),
            child(variant = HueModulation, qname = "a:hueMod"),
            child(variant = Saturation, qname = "a:sat"),
            child(variant = SaturationOffset, qname = "a:satOff"),
            child(variant = SaturationModulation, qname = "a:satMod"),
            child(variant = Luminance, qname = "a:lum"),
            child(variant = LuminanceOffset, qname = "a:lumOff"),
            child(variant = LuminanceModulation, qname = "a:lumMod"),
            child(variant = Red, qname = "a:red"),
            child(variant = RedOffset, qname = "a:redOff"),
            child(variant = RedModulation, qname = "a:redMod"),
            child(variant = Green, qname = "a:green"),
            child(variant = GreenOffset, qname = "a:greenOff"),
            child(variant = GreenModulation, qname = "a:greenMod"),
            child(variant = Blue, qname = "a:blue"),
            child(variant = BlueOffset, qname = "a:blueOff"),
            child(variant = BlueModulation, qname = "a:blueMod"),
            empty_child(variant = Gamma, qname = "a:gamma"),
            empty_child(variant = InverseGamma, qname = "a:invGamma")
        )
    )]
  pub color_style_variation_choice: Vec<ColorStyleVariationChoice>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:extLst")]
pub struct OfficeArtExtensionList {
  /// Extension.
  #[sdk(child(qname = "a:ext"))]
  pub extension: Vec<crate::schemas::a::Extension>,
}
/// Defines the StyleColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:styleClr")]
pub struct StyleColor {
  /// val
  #[sdk(attr(qname = ":val"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:unsignedInt"))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["auto"]))]
  #[sdk(string_format(source = 2u32, union = 0u64, kind = "token"))]
  pub val: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            child(variant = Tint, qname = "a:tint"),
            child(variant = Shade, qname = "a:shade"),
            empty_child(variant = Complement, qname = "a:comp"),
            empty_child(variant = Inverse, qname = "a:inv"),
            empty_child(variant = Gray, qname = "a:gray"),
            child(variant = Alpha, qname = "a:alpha"),
            child(variant = AlphaOffset, qname = "a:alphaOff"),
            child(variant = AlphaModulation, qname = "a:alphaMod"),
            child(variant = Hue, qname = "a:hue"),
            child(variant = HueOffset, qname = "a:hueOff"),
            child(variant = HueModulation, qname = "a:hueMod"),
            child(variant = Saturation, qname = "a:sat"),
            child(variant = SaturationOffset, qname = "a:satOff"),
            child(variant = SaturationModulation, qname = "a:satMod"),
            child(variant = Luminance, qname = "a:lum"),
            child(variant = LuminanceOffset, qname = "a:lumOff"),
            child(variant = LuminanceModulation, qname = "a:lumMod"),
            child(variant = Red, qname = "a:red"),
            child(variant = RedOffset, qname = "a:redOff"),
            child(variant = RedModulation, qname = "a:redMod"),
            child(variant = Green, qname = "a:green"),
            child(variant = GreenOffset, qname = "a:greenOff"),
            child(variant = GreenModulation, qname = "a:greenMod"),
            child(variant = Blue, qname = "a:blue"),
            child(variant = BlueOffset, qname = "a:blueOff"),
            child(variant = BlueModulation, qname = "a:blueMod"),
            empty_child(variant = Gamma, qname = "a:gamma"),
            empty_child(variant = InverseGamma, qname = "a:invGamma")
        )
    )]
  pub style_color_choice: Vec<StyleColorChoice>,
}
/// Defines the LineReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:lnRef")]
pub struct LineReference {
  /// idx
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub line_reference_choice: Option<LineReferenceChoice>,
  /// Defines the StyleColor Class.
  #[sdk(child(qname = "cs:styleClr"))]
  pub style_color: Option<StyleColor>,
}
/// Defines the FillReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:fillRef")]
pub struct FillReference {
  /// idx
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub fill_reference_choice: Option<FillReferenceChoice>,
  /// Defines the StyleColor Class.
  #[sdk(child(qname = "cs:styleClr"))]
  pub style_color: Option<StyleColor>,
}
/// Defines the EffectReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:effectRef")]
pub struct EffectReference {
  /// idx
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub effect_reference_choice: Option<EffectReferenceChoice>,
  /// Defines the StyleColor Class.
  #[sdk(child(qname = "cs:styleClr"))]
  pub style_color: Option<StyleColor>,
}
/// Defines the LineWidthScale Class.
pub type LineWidthScale = crate::simple_type::DoubleValue;
/// Defines the FontReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:fontRef")]
pub struct FontReference {
  /// idx
  #[sdk(attr(qname = ":idx"))]
  #[sdk(string_format(kind = "token"))]
  pub index: crate::schemas::a::FontCollectionIndexValues,
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  #[sdk(
        choice(
            child(variant = RgbColorModelPercentage, qname = "a:scrgbClr"),
            child(variant = RgbColorModelHex, qname = "a:srgbClr"),
            child(variant = HslColor, qname = "a:hslClr"),
            child(variant = SystemColor, qname = "a:sysClr"),
            child(variant = SchemeColor, qname = "a:schemeClr"),
            child(variant = PresetColor, qname = "a:prstClr")
        )
    )]
  pub font_reference_choice: Option<FontReferenceChoice>,
  /// Defines the StyleColor Class.
  #[sdk(child(qname = "cs:styleClr"))]
  pub style_color: Option<StyleColor>,
}
/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:spPr")]
pub struct ShapeProperties {
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// 2D Transform for Individual Objects
  #[sdk(child(qname = "a:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<crate::schemas::a::Transform2D>>,
  #[sdk(
        choice(
            child(variant = CustomGeometry, qname = "a:custGeom"),
            child(variant = PresetGeometry, qname = "a:prstGeom")
        )
    )]
  pub shape_properties_choice1: Option<ShapePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub shape_properties_choice2: Option<ShapePropertiesChoice2>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:ln"))]
  pub outline: Option<std::boxed::Box<crate::schemas::a::Outline>>,
  #[sdk(
        choice(
            child(variant = EffectList, qname = "a:effectLst"),
            child(variant = EffectDag, qname = "a:effectDag")
        )
    )]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:scene3d"))]
  pub scene3_d_type: Option<std::boxed::Box<crate::schemas::a::Scene3DType>>,
  /// Apply 3D shape properties.
  #[sdk(child(qname = "a:sp3d"))]
  pub shape3_d_type: Option<std::boxed::Box<crate::schemas::a::Shape3DType>>,
  /// Defines the ShapePropertiesExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub shape_properties_extension_list: Option<crate::schemas::a::ShapePropertiesExtensionList>,
}
/// Defines the TextCharacterPropertiesType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:defRPr")]
pub struct TextCharacterPropertiesType {
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
  #[sdk(number_range(range = 100..= 400000))]
  pub font_size: Option<crate::simple_type::Int32Value>,
  /// b
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
  /// i
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// u
  #[sdk(attr(qname = ":u"))]
  #[sdk(string_format(kind = "token"))]
  pub underline: Option<crate::schemas::a::TextUnderlineValues>,
  /// strike
  #[sdk(attr(qname = ":strike"))]
  #[sdk(string_format(kind = "token"))]
  pub strike: Option<crate::schemas::a::TextStrikeValues>,
  /// kern
  #[sdk(attr(qname = ":kern"))]
  #[sdk(number_range(range = 0..= 400000))]
  pub kerning: Option<crate::simple_type::Int32Value>,
  /// cap
  #[sdk(attr(qname = ":cap"))]
  #[sdk(string_format(kind = "token"))]
  pub capital: Option<crate::schemas::a::TextCapsValues>,
  /// spc
  #[sdk(attr(qname = ":spc"))]
  #[sdk(number_range(range = -400000..= 400000))]
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
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:ln"))]
  pub outline: Option<std::boxed::Box<crate::schemas::a::Outline>>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, qname = "a:solidFill"),
            child(variant = GradientFill, qname = "a:gradFill"),
            child(variant = BlipFill, qname = "a:blipFill"),
            child(variant = PatternFill, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub text_character_properties_type_choice1: Option<TextCharacterPropertiesTypeChoice>,
  #[sdk(
        choice(
            child(variant = EffectList, qname = "a:effectLst"),
            child(variant = EffectDag, qname = "a:effectDag")
        )
    )]
  pub text_character_properties_type_choice2: Option<TextCharacterPropertiesTypeChoice2>,
  /// Defines the Highlight Class.
  #[sdk(child(qname = "a:highlight"))]
  pub highlight: Option<std::boxed::Box<crate::schemas::a::Highlight>>,
  #[sdk(
        choice(
            empty_child(variant = UnderlineFollowsText, qname = "a:uLnTx"),
            child(variant = Underline, qname = "a:uLn")
        )
    )]
  pub text_character_properties_type_choice3: Option<TextCharacterPropertiesTypeChoice3>,
  #[sdk(
        choice(
            empty_child(variant = UnderlineFillText, qname = "a:uFillTx"),
            child(variant = UnderlineFill, qname = "a:uFill")
        )
    )]
  pub text_character_properties_type_choice4: Option<TextCharacterPropertiesTypeChoice4>,
  /// Latin Font.
  #[sdk(child(qname = "a:latin"))]
  pub latin_font: Option<crate::schemas::a::LatinFont>,
  /// East Asian Font.
  #[sdk(child(qname = "a:ea"))]
  pub east_asian_font: Option<crate::schemas::a::EastAsianFont>,
  /// Complex Script Font.
  #[sdk(child(qname = "a:cs"))]
  pub complex_script_font: Option<crate::schemas::a::ComplexScriptFont>,
  /// Defines the SymbolFont Class.
  #[sdk(child(qname = "a:sym"))]
  pub symbol_font: Option<crate::schemas::a::SymbolFont>,
  /// Defines the HyperlinkOnClick Class.
  #[sdk(child(qname = "a:hlinkClick"))]
  pub hyperlink_on_click: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnClick>>,
  /// Defines the HyperlinkOnMouseOver Class.
  #[sdk(child(qname = "a:hlinkMouseOver"))]
  pub hyperlink_on_mouse_over: Option<std::boxed::Box<crate::schemas::a::HyperlinkOnMouseOver>>,
  /// Defines the RightToLeft Class.
  #[sdk(child(qname = "a:rtl"))]
  pub right_to_left: Option<crate::schemas::a::RightToLeft>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Defines the TextBodyProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:bodyPr")]
pub struct TextBodyProperties {
  /// Rotation
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Paragraph Spacing
  #[sdk(attr(qname = ":spcFirstLastPara"))]
  pub use_paragraph_spacing: Option<crate::simple_type::BooleanValue>,
  /// Text Vertical Overflow
  #[sdk(attr(qname = ":vertOverflow"))]
  #[sdk(string_format(kind = "token"))]
  pub vertical_overflow: Option<crate::schemas::a::TextVerticalOverflowValues>,
  /// Text Horizontal Overflow
  #[sdk(attr(qname = ":horzOverflow"))]
  #[sdk(string_format(kind = "token"))]
  pub horizontal_overflow: Option<crate::schemas::a::TextHorizontalOverflowValues>,
  /// Vertical Text
  #[sdk(attr(qname = ":vert"))]
  #[sdk(string_format(kind = "token"))]
  pub vertical: Option<crate::schemas::a::TextVerticalValues>,
  /// Text Wrapping Type
  #[sdk(attr(qname = ":wrap"))]
  #[sdk(string_format(kind = "token"))]
  pub wrap: Option<crate::schemas::a::TextWrappingValues>,
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
  #[sdk(number_range(range = 1..= 16))]
  pub column_count: Option<crate::simple_type::Int32Value>,
  /// Space Between Columns
  #[sdk(attr(qname = ":spcCol"))]
  #[sdk(number_range(range = 0..))]
  pub column_spacing: Option<crate::simple_type::Int32Value>,
  /// Columns Right-To-Left
  #[sdk(attr(qname = ":rtlCol"))]
  pub right_to_left_columns: Option<crate::simple_type::BooleanValue>,
  /// From WordArt
  #[sdk(attr(qname = ":fromWordArt"))]
  pub from_word_art: Option<crate::simple_type::BooleanValue>,
  /// Anchor
  #[sdk(attr(qname = ":anchor"))]
  #[sdk(string_format(kind = "token"))]
  pub anchor: Option<crate::schemas::a::TextAnchoringTypeValues>,
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
  #[sdk(child(qname = "a:prstTxWarp"))]
  pub preset_text_warp: Option<std::boxed::Box<crate::schemas::a::PresetTextWarp>>,
  #[sdk(
        choice(
            empty_child(variant = NoAutoFit, qname = "a:noAutofit"),
            child(variant = NormalAutoFit, qname = "a:normAutofit"),
            empty_child(variant = ShapeAutoFit, qname = "a:spAutoFit")
        )
    )]
  pub text_body_properties_choice1: Option<TextBodyPropertiesChoice>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:scene3d"))]
  pub scene3_d_type: Option<std::boxed::Box<crate::schemas::a::Scene3DType>>,
  #[sdk(
        choice(
            child(variant = Shape3DType, qname = "a:sp3d"),
            child(variant = FlatText, qname = "a:flatTx")
        )
    )]
  pub text_body_properties_choice2: Option<TextBodyPropertiesChoice2>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Defines the CategoryAxisProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:categoryAxis")]
pub struct CategoryAxisProperties {
  /// visible
  #[sdk(attr(qname = ":visible"))]
  #[sdk(string_format(kind = "token"))]
  pub visible: Option<Boolean>,
  /// majorTick
  #[sdk(attr(qname = ":majorTick"))]
  #[sdk(string_format(kind = "token"))]
  pub major_tick: Option<TickMarkNinch>,
  /// minorTick
  #[sdk(attr(qname = ":minorTick"))]
  #[sdk(string_format(kind = "token"))]
  pub minor_tick_prop: Option<TickMarkNinch>,
  /// labelPosition
  #[sdk(attr(qname = ":labelPosition"))]
  #[sdk(string_format(kind = "token"))]
  pub label_position: Option<TickLabelPositionNinch>,
  /// majorGridlines
  #[sdk(attr(qname = ":majorGridlines"))]
  #[sdk(string_format(kind = "token"))]
  pub major_gridlines: Option<Boolean>,
  /// minorGridlines
  #[sdk(attr(qname = ":minorGridlines"))]
  #[sdk(string_format(kind = "token"))]
  pub minor_gridlines_prop: Option<Boolean>,
  /// title
  #[sdk(attr(qname = ":title"))]
  #[sdk(string_format(kind = "token"))]
  pub title_prop: Option<Boolean>,
}
/// Defines the SeriesAxisProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:seriesAxis")]
pub struct SeriesAxisProperties {
  /// visible
  #[sdk(attr(qname = ":visible"))]
  #[sdk(string_format(kind = "token"))]
  pub visible: Option<Boolean>,
  /// majorTick
  #[sdk(attr(qname = ":majorTick"))]
  #[sdk(string_format(kind = "token"))]
  pub major_tick: Option<TickMarkNinch>,
  /// minorTick
  #[sdk(attr(qname = ":minorTick"))]
  #[sdk(string_format(kind = "token"))]
  pub minor_tick_prop: Option<TickMarkNinch>,
  /// labelPosition
  #[sdk(attr(qname = ":labelPosition"))]
  #[sdk(string_format(kind = "token"))]
  pub label_position: Option<TickLabelPositionNinch>,
  /// majorGridlines
  #[sdk(attr(qname = ":majorGridlines"))]
  #[sdk(string_format(kind = "token"))]
  pub major_gridlines: Option<Boolean>,
  /// minorGridlines
  #[sdk(attr(qname = ":minorGridlines"))]
  #[sdk(string_format(kind = "token"))]
  pub minor_gridlines_prop: Option<Boolean>,
  /// title
  #[sdk(attr(qname = ":title"))]
  #[sdk(string_format(kind = "token"))]
  pub title_prop: Option<Boolean>,
}
/// Defines the ValueAxisProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:valueAxis")]
pub struct ValueAxisProperties {
  /// visible
  #[sdk(attr(qname = ":visible"))]
  #[sdk(string_format(kind = "token"))]
  pub visible: Option<Boolean>,
  /// majorTick
  #[sdk(attr(qname = ":majorTick"))]
  #[sdk(string_format(kind = "token"))]
  pub major_tick: Option<TickMarkNinch>,
  /// minorTick
  #[sdk(attr(qname = ":minorTick"))]
  #[sdk(string_format(kind = "token"))]
  pub minor_tick_prop: Option<TickMarkNinch>,
  /// labelPosition
  #[sdk(attr(qname = ":labelPosition"))]
  #[sdk(string_format(kind = "token"))]
  pub label_position: Option<TickLabelPositionNinch>,
  /// majorGridlines
  #[sdk(attr(qname = ":majorGridlines"))]
  #[sdk(string_format(kind = "token"))]
  pub major_gridlines: Option<Boolean>,
  /// minorGridlines
  #[sdk(attr(qname = ":minorGridlines"))]
  #[sdk(string_format(kind = "token"))]
  pub minor_gridlines_prop: Option<Boolean>,
  /// title
  #[sdk(attr(qname = ":title"))]
  #[sdk(string_format(kind = "token"))]
  pub title_prop: Option<Boolean>,
}
/// Defines the DataSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:dataSeries")]
pub struct DataSeries {
  /// overlap
  #[sdk(attr(qname = ":overlap"))]
  #[sdk(number_range(range = -100..= 100))]
  pub overlap: Option<crate::simple_type::SByteValue>,
  /// gapWidth
  #[sdk(attr(qname = ":gapWidth"))]
  #[sdk(number_range(range = 0..= 500))]
  pub gap_width: Option<crate::simple_type::UInt16Value>,
  /// gapDepth
  #[sdk(attr(qname = ":gapDepth"))]
  #[sdk(number_range(range = 0..= 500))]
  pub gap_depth: Option<crate::simple_type::UInt16Value>,
  /// doughnutHoleSize
  #[sdk(attr(qname = ":doughnutHoleSize"))]
  #[sdk(number_range(range = 10..= 90))]
  pub doughnut_hole_size: Option<crate::simple_type::ByteValue>,
  /// markerVisible
  #[sdk(attr(qname = ":markerVisible"))]
  #[sdk(string_format(kind = "token"))]
  pub marker_visible: Option<Boolean>,
  /// hiloLines
  #[sdk(attr(qname = ":hiloLines"))]
  #[sdk(string_format(kind = "token"))]
  pub hilo_lines: Option<Boolean>,
  /// dropLines
  #[sdk(attr(qname = ":dropLines"))]
  #[sdk(string_format(kind = "token"))]
  pub drop_lines: Option<Boolean>,
  /// seriesLines
  #[sdk(attr(qname = ":seriesLines"))]
  #[sdk(string_format(kind = "token"))]
  pub series_lines: Option<Boolean>,
}
/// Defines the DataLabels Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:dataLabels")]
pub struct DataLabels {
  /// position
  #[sdk(attr(qname = ":position"))]
  #[sdk(string_format(kind = "token"))]
  pub position: Option<DataLabelsPosition>,
  /// value
  #[sdk(attr(qname = ":value"))]
  #[sdk(string_format(kind = "token"))]
  pub value: Option<Boolean>,
  /// seriesName
  #[sdk(attr(qname = ":seriesName"))]
  #[sdk(string_format(kind = "token"))]
  pub series_name: Option<Boolean>,
  /// categoryName
  #[sdk(attr(qname = ":categoryName"))]
  #[sdk(string_format(kind = "token"))]
  pub category_name: Option<Boolean>,
  /// legendKey
  #[sdk(attr(qname = ":legendKey"))]
  #[sdk(string_format(kind = "token"))]
  pub legend_key: Option<Boolean>,
  /// percentage
  #[sdk(attr(qname = ":percentage"))]
  #[sdk(string_format(kind = "token"))]
  pub percentage: Option<Boolean>,
}
/// Defines the DataTable Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:dataTable")]
pub struct DataTable {
  /// legendKeys
  #[sdk(attr(qname = ":legendKeys"))]
  #[sdk(string_format(kind = "token"))]
  pub legend_keys: Option<Boolean>,
  /// horizontalBorder
  #[sdk(attr(qname = ":horizontalBorder"))]
  #[sdk(string_format(kind = "token"))]
  pub horizontal_border: Option<Boolean>,
  /// verticalBorder
  #[sdk(attr(qname = ":verticalBorder"))]
  #[sdk(string_format(kind = "token"))]
  pub vertical_border: Option<Boolean>,
  /// outlineBorder
  #[sdk(attr(qname = ":outlineBorder"))]
  #[sdk(string_format(kind = "token"))]
  pub outline_border: Option<Boolean>,
}
/// Defines the Legend Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:legend")]
pub struct Legend {
  /// visible
  #[sdk(attr(qname = ":visible"))]
  #[sdk(string_format(kind = "token"))]
  pub visible: Option<Boolean>,
  /// includeInLayout
  #[sdk(attr(qname = ":includeInLayout"))]
  #[sdk(string_format(kind = "token"))]
  pub include_in_layout: Option<Boolean>,
  /// position
  #[sdk(attr(qname = ":position"))]
  #[sdk(string_format(kind = "token"))]
  pub position: Option<LegendPosition>,
}
/// Defines the Title Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:title")]
pub struct Title {
  /// position
  #[sdk(attr(qname = ":position"))]
  #[sdk(string_format(kind = "token"))]
  pub position: Option<TitlePosition>,
}
/// Defines the Trendline Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:trendline")]
pub struct Trendline {
  /// add
  #[sdk(attr(qname = ":add"))]
  #[sdk(string_format(kind = "token"))]
  pub add: Option<Boolean>,
  /// equation
  #[sdk(attr(qname = ":equation"))]
  #[sdk(string_format(kind = "token"))]
  pub equation: Option<Boolean>,
  /// rsquared
  #[sdk(attr(qname = ":rsquared"))]
  #[sdk(string_format(kind = "token"))]
  pub r_squared: Option<Boolean>,
}
/// Defines the View3DProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:view3D")]
pub struct View3DProperties {
  /// rotX
  #[sdk(attr(qname = ":rotX"))]
  #[sdk(number_range(range = -90..= 90))]
  pub rot_x: Option<crate::simple_type::SByteValue>,
  /// rotY
  #[sdk(attr(qname = ":rotY"))]
  #[sdk(number_range(range = 0..= 360))]
  pub rot_y: Option<crate::simple_type::UInt16Value>,
  /// rAngAx
  #[sdk(attr(qname = ":rAngAx"))]
  #[sdk(string_format(kind = "token"))]
  pub right_angle_axes: Option<Boolean>,
  /// perspective
  #[sdk(attr(qname = ":perspective"))]
  #[sdk(number_range(range = 0..= 240))]
  pub perspective: Option<crate::simple_type::ByteValue>,
  /// heightPercent
  #[sdk(attr(qname = ":heightPercent"))]
  #[sdk(number_range(range = 5..= 500))]
  pub height_percent: Option<crate::simple_type::UInt16Value>,
  /// depthPercent
  #[sdk(attr(qname = ":depthPercent"))]
  #[sdk(number_range(range = 20..= 2000))]
  pub depth_percent: Option<crate::simple_type::UInt16Value>,
}
/// Defines the AxisTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:axisTitle")]
pub struct AxisTitle {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the CategoryAxis Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:categoryAxis")]
pub struct CategoryAxis {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ChartArea Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:chartArea")]
pub struct ChartArea {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataLabel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:dataLabel")]
pub struct DataLabel {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataLabelCallout Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:dataLabelCallout")]
pub struct DataLabelCallout {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPoint Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:dataPoint")]
pub struct DataPoint {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPoint3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:dataPoint3D")]
pub struct DataPoint3D {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPointLine Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:dataPointLine")]
pub struct DataPointLine {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPointMarker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:dataPointMarker")]
pub struct DataPointMarker {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataPointWireframe Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:dataPointWireframe")]
pub struct DataPointWireframe {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DataTableStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:dataTable")]
pub struct DataTableStyle {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DownBar Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:downBar")]
pub struct DownBar {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DropLine Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:dropLine")]
pub struct DropLine {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ErrorBar Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:errorBar")]
pub struct ErrorBar {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the Floor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:floor")]
pub struct Floor {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the GridlineMajor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:gridlineMajor")]
pub struct GridlineMajor {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the GridlineMinor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:gridlineMinor")]
pub struct GridlineMinor {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the HiLoLine Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:hiLoLine")]
pub struct HiLoLine {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the LeaderLine Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:leaderLine")]
pub struct LeaderLine {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the LegendStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:legend")]
pub struct LegendStyle {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PlotArea Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:plotArea")]
pub struct PlotArea {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the PlotArea3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:plotArea3D")]
pub struct PlotArea3D {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the SeriesAxis Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:seriesAxis")]
pub struct SeriesAxis {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the SeriesLine Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:seriesLine")]
pub struct SeriesLine {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the TitleStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:title")]
pub struct TitleStyle {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the TrendlineStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:trendline")]
pub struct TrendlineStyle {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the TrendlineLabel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:trendlineLabel")]
pub struct TrendlineLabel {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the UpBar Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:upBar")]
pub struct UpBar {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the ValueAxis Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:valueAxis")]
pub struct ValueAxis {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the Wall Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:wall")]
pub struct Wall {
  /// mods
  #[sdk(attr(list, qname = ":mods"))]
  pub modifiers: Option<Vec<crate::simple_type::StringValue>>,
  /// Defines the LineReference Class.
  #[sdk(child(qname = "cs:lnRef"))]
  pub line_reference: std::boxed::Box<LineReference>,
  /// Defines the LineWidthScale Class.
  #[sdk(text_child(simple_type = "DoubleValue", qname = "cs:lineWidthScale"))]
  pub line_width_scale: Option<LineWidthScale>,
  /// Defines the FillReference Class.
  #[sdk(child(qname = "cs:fillRef"))]
  pub fill_reference: std::boxed::Box<FillReference>,
  /// Defines the EffectReference Class.
  #[sdk(child(qname = "cs:effectRef"))]
  pub effect_reference: std::boxed::Box<EffectReference>,
  /// Defines the FontReference Class.
  #[sdk(child(qname = "cs:fontRef"))]
  pub font_reference: std::boxed::Box<FontReference>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cs:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TextCharacterPropertiesType Class.
  #[sdk(child(qname = "cs:defRPr"))]
  pub text_character_properties_type: Option<std::boxed::Box<TextCharacterPropertiesType>>,
  /// Defines the TextBodyProperties Class.
  #[sdk(child(qname = "cs:bodyPr"))]
  pub text_body_properties: Option<std::boxed::Box<TextBodyProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "cs:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the MarkerLayoutProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cs:dataPointMarkerLayout")]
pub struct MarkerLayoutProperties {
  /// symbol
  #[sdk(attr(qname = ":symbol"))]
  #[sdk(string_format(kind = "token"))]
  pub symbol: Option<MarkerStyle>,
  /// size
  #[sdk(attr(qname = ":size"))]
  #[sdk(number_range(range = 2..= 72))]
  pub size: Option<crate::simple_type::ByteValue>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ColorStyleChoice {
  RgbColorModelPercentage(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  HslColor(std::boxed::Box<crate::schemas::a::HslColor>),
  SystemColor(std::boxed::Box<crate::schemas::a::SystemColor>),
  SchemeColor(std::boxed::Box<crate::schemas::a::SchemeColor>),
  PresetColor(std::boxed::Box<crate::schemas::a::PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ColorStyleVariationChoice {
  /// Tint.
  Tint(std::boxed::Box<crate::schemas::a::Tint>),
  /// Shade.
  Shade(std::boxed::Box<crate::schemas::a::Shade>),
  /// Complement.
  #[sdk(empty_child(qname = "a:comp"))]
  Complement,
  /// Inverse.
  #[sdk(empty_child(qname = "a:inv"))]
  Inverse,
  /// Gray.
  #[sdk(empty_child(qname = "a:gray"))]
  Gray,
  /// Alpha.
  Alpha(std::boxed::Box<crate::schemas::a::Alpha>),
  /// Alpha Offset.
  AlphaOffset(std::boxed::Box<crate::schemas::a::AlphaOffset>),
  /// Alpha Modulation.
  AlphaModulation(std::boxed::Box<crate::schemas::a::AlphaModulation>),
  /// Hue.
  Hue(std::boxed::Box<crate::schemas::a::Hue>),
  /// Hue Offset.
  HueOffset(std::boxed::Box<crate::schemas::a::HueOffset>),
  /// Hue Modulate.
  HueModulation(std::boxed::Box<crate::schemas::a::HueModulation>),
  /// Saturation.
  Saturation(std::boxed::Box<crate::schemas::a::Saturation>),
  /// Saturation Offset.
  SaturationOffset(std::boxed::Box<crate::schemas::a::SaturationOffset>),
  /// Saturation Modulation.
  SaturationModulation(std::boxed::Box<crate::schemas::a::SaturationModulation>),
  /// Luminance.
  Luminance(std::boxed::Box<crate::schemas::a::Luminance>),
  /// Luminance Offset.
  LuminanceOffset(std::boxed::Box<crate::schemas::a::LuminanceOffset>),
  /// Luminance Modulation.
  LuminanceModulation(std::boxed::Box<crate::schemas::a::LuminanceModulation>),
  /// Red.
  Red(std::boxed::Box<crate::schemas::a::Red>),
  /// Red Offset.
  RedOffset(std::boxed::Box<crate::schemas::a::RedOffset>),
  /// Red Modulation.
  RedModulation(std::boxed::Box<crate::schemas::a::RedModulation>),
  /// Green.
  Green(std::boxed::Box<crate::schemas::a::Green>),
  /// Green Offset.
  GreenOffset(std::boxed::Box<crate::schemas::a::GreenOffset>),
  /// Green Modification.
  GreenModulation(std::boxed::Box<crate::schemas::a::GreenModulation>),
  /// Blue.
  Blue(std::boxed::Box<crate::schemas::a::Blue>),
  /// Blue Offset.
  BlueOffset(std::boxed::Box<crate::schemas::a::BlueOffset>),
  /// Blue Modification.
  BlueModulation(std::boxed::Box<crate::schemas::a::BlueModulation>),
  /// Gamma.
  #[sdk(empty_child(qname = "a:gamma"))]
  Gamma,
  /// Inverse Gamma.
  #[sdk(empty_child(qname = "a:invGamma"))]
  InverseGamma,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum StyleColorChoice {
  /// Tint.
  Tint(std::boxed::Box<crate::schemas::a::Tint>),
  /// Shade.
  Shade(std::boxed::Box<crate::schemas::a::Shade>),
  /// Complement.
  #[sdk(empty_child(qname = "a:comp"))]
  Complement,
  /// Inverse.
  #[sdk(empty_child(qname = "a:inv"))]
  Inverse,
  /// Gray.
  #[sdk(empty_child(qname = "a:gray"))]
  Gray,
  /// Alpha.
  Alpha(std::boxed::Box<crate::schemas::a::Alpha>),
  /// Alpha Offset.
  AlphaOffset(std::boxed::Box<crate::schemas::a::AlphaOffset>),
  /// Alpha Modulation.
  AlphaModulation(std::boxed::Box<crate::schemas::a::AlphaModulation>),
  /// Hue.
  Hue(std::boxed::Box<crate::schemas::a::Hue>),
  /// Hue Offset.
  HueOffset(std::boxed::Box<crate::schemas::a::HueOffset>),
  /// Hue Modulate.
  HueModulation(std::boxed::Box<crate::schemas::a::HueModulation>),
  /// Saturation.
  Saturation(std::boxed::Box<crate::schemas::a::Saturation>),
  /// Saturation Offset.
  SaturationOffset(std::boxed::Box<crate::schemas::a::SaturationOffset>),
  /// Saturation Modulation.
  SaturationModulation(std::boxed::Box<crate::schemas::a::SaturationModulation>),
  /// Luminance.
  Luminance(std::boxed::Box<crate::schemas::a::Luminance>),
  /// Luminance Offset.
  LuminanceOffset(std::boxed::Box<crate::schemas::a::LuminanceOffset>),
  /// Luminance Modulation.
  LuminanceModulation(std::boxed::Box<crate::schemas::a::LuminanceModulation>),
  /// Red.
  Red(std::boxed::Box<crate::schemas::a::Red>),
  /// Red Offset.
  RedOffset(std::boxed::Box<crate::schemas::a::RedOffset>),
  /// Red Modulation.
  RedModulation(std::boxed::Box<crate::schemas::a::RedModulation>),
  /// Green.
  Green(std::boxed::Box<crate::schemas::a::Green>),
  /// Green Offset.
  GreenOffset(std::boxed::Box<crate::schemas::a::GreenOffset>),
  /// Green Modification.
  GreenModulation(std::boxed::Box<crate::schemas::a::GreenModulation>),
  /// Blue.
  Blue(std::boxed::Box<crate::schemas::a::Blue>),
  /// Blue Offset.
  BlueOffset(std::boxed::Box<crate::schemas::a::BlueOffset>),
  /// Blue Modification.
  BlueModulation(std::boxed::Box<crate::schemas::a::BlueModulation>),
  /// Gamma.
  #[sdk(empty_child(qname = "a:gamma"))]
  Gamma,
  /// Inverse Gamma.
  #[sdk(empty_child(qname = "a:invGamma"))]
  InverseGamma,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LineReferenceChoice {
  RgbColorModelPercentage(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  HslColor(std::boxed::Box<crate::schemas::a::HslColor>),
  SystemColor(std::boxed::Box<crate::schemas::a::SystemColor>),
  SchemeColor(std::boxed::Box<crate::schemas::a::SchemeColor>),
  PresetColor(std::boxed::Box<crate::schemas::a::PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FillReferenceChoice {
  RgbColorModelPercentage(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  HslColor(std::boxed::Box<crate::schemas::a::HslColor>),
  SystemColor(std::boxed::Box<crate::schemas::a::SystemColor>),
  SchemeColor(std::boxed::Box<crate::schemas::a::SchemeColor>),
  PresetColor(std::boxed::Box<crate::schemas::a::PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum EffectReferenceChoice {
  RgbColorModelPercentage(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  HslColor(std::boxed::Box<crate::schemas::a::HslColor>),
  SystemColor(std::boxed::Box<crate::schemas::a::SystemColor>),
  SchemeColor(std::boxed::Box<crate::schemas::a::SchemeColor>),
  PresetColor(std::boxed::Box<crate::schemas::a::PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FontReferenceChoice {
  RgbColorModelPercentage(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  HslColor(std::boxed::Box<crate::schemas::a::HslColor>),
  SystemColor(std::boxed::Box<crate::schemas::a::SystemColor>),
  SchemeColor(std::boxed::Box<crate::schemas::a::SchemeColor>),
  PresetColor(std::boxed::Box<crate::schemas::a::PresetColor>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice {
  /// Custom geometry.
  CustomGeometry(std::boxed::Box<crate::schemas::a::CustomGeometry>),
  /// Preset geometry.
  PresetGeometry(std::boxed::Box<crate::schemas::a::PresetGeometry>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice2 {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<crate::schemas::a::NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<crate::schemas::a::SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<crate::schemas::a::GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<crate::schemas::a::BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<crate::schemas::a::PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:grpFill"))]
  GroupFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice3 {
  /// Effect Container.
  EffectList(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextCharacterPropertiesTypeChoice {
  /// Defines the NoFill Class.
  NoFill(std::boxed::Box<crate::schemas::a::NoFill>),
  /// Defines the SolidFill Class.
  SolidFill(std::boxed::Box<crate::schemas::a::SolidFill>),
  /// Defines the GradientFill Class.
  GradientFill(std::boxed::Box<crate::schemas::a::GradientFill>),
  /// Defines the BlipFill Class.
  BlipFill(std::boxed::Box<crate::schemas::a::BlipFill>),
  /// Pattern Fill.
  PatternFill(std::boxed::Box<crate::schemas::a::PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:grpFill"))]
  GroupFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextCharacterPropertiesTypeChoice2 {
  /// Effect Container.
  EffectList(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextCharacterPropertiesTypeChoice3 {
  /// Underline Follows Text.
  #[sdk(empty_child(qname = "a:uLnTx"))]
  UnderlineFollowsText,
  /// Underline Stroke.
  Underline(std::boxed::Box<crate::schemas::a::Underline>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextCharacterPropertiesTypeChoice4 {
  /// Underline Fill Properties Follow Text.
  #[sdk(empty_child(qname = "a:uFillTx"))]
  UnderlineFillText,
  /// Underline Fill.
  UnderlineFill(std::boxed::Box<crate::schemas::a::UnderlineFill>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextBodyPropertiesChoice {
  /// No AutoFit.
  #[sdk(empty_child(qname = "a:noAutofit"))]
  NoAutoFit,
  /// Normal AutoFit.
  NormalAutoFit(std::boxed::Box<crate::schemas::a::NormalAutoFit>),
  /// Shape AutoFit.
  #[sdk(empty_child(qname = "a:spAutoFit"))]
  ShapeAutoFit,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextBodyPropertiesChoice2 {
  /// Apply 3D shape properties.
  Shape3DType(std::boxed::Box<crate::schemas::a::Shape3DType>),
  /// No text in 3D scene.
  FlatText(std::boxed::Box<crate::schemas::a::FlatText>),
}
