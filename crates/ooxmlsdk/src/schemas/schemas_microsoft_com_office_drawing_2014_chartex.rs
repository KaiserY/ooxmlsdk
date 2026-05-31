//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ExtensionDropMode {
  #[sdk(rename = "never")]
  #[default]
  Never,
  #[sdk(rename = "onModelChange")]
  OnModelChange,
  #[sdk(rename = "onDataChange")]
  OnDataChange,
  #[sdk(rename = "always")]
  Always,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum FormulaDirection {
  #[sdk(rename = "col")]
  #[default]
  Col,
  #[sdk(rename = "row")]
  Row,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum StringDimensionType {
  #[sdk(rename = "cat")]
  #[default]
  Cat,
  #[sdk(rename = "colorStr")]
  ColorStr,
  #[sdk(rename = "entityId")]
  EntityId,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum NumericDimensionType {
  #[sdk(rename = "val")]
  #[default]
  Val,
  #[sdk(rename = "x")]
  X,
  #[sdk(rename = "y")]
  Y,
  #[sdk(rename = "size")]
  Size,
  #[sdk(rename = "colorVal")]
  ColorVal,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SidePos {
  #[sdk(rename = "l")]
  #[default]
  L,
  #[sdk(rename = "t")]
  T,
  #[sdk(rename = "r")]
  R,
  #[sdk(rename = "b")]
  B,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PosAlign {
  #[sdk(rename = "min")]
  #[default]
  Min,
  #[sdk(rename = "ctr")]
  Ctr,
  #[sdk(rename = "max")]
  Max,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum AxisUnit {
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
  #[sdk(rename = "percentage")]
  Percentage,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TickMarksType {
  #[sdk(rename = "in")]
  #[default]
  In,
  #[sdk(rename = "out")]
  Out,
  #[sdk(rename = "cross")]
  Cross,
  #[sdk(rename = "none")]
  None,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SeriesLayout {
  #[sdk(rename = "boxWhisker")]
  #[default]
  BoxWhisker,
  #[sdk(rename = "clusteredColumn")]
  ClusteredColumn,
  #[sdk(rename = "funnel")]
  Funnel,
  #[sdk(rename = "paretoLine")]
  ParetoLine,
  #[sdk(rename = "regionMap")]
  RegionMap,
  #[sdk(rename = "sunburst")]
  Sunburst,
  #[sdk(rename = "treemap")]
  Treemap,
  #[sdk(rename = "waterfall")]
  Waterfall,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ParentLabelLayoutVal {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "banner")]
  Banner,
  #[sdk(rename = "overlapping")]
  Overlapping,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RegionLabelLayoutEnum {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "bestFitOnly")]
  BestFitOnly,
  #[sdk(rename = "showAll")]
  ShowAll,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum IntervalClosedSide {
  #[sdk(rename = "l")]
  #[default]
  L,
  #[sdk(rename = "r")]
  R,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum EntityTypeEnum {
  #[sdk(rename = "Address")]
  #[default]
  Address,
  #[sdk(rename = "AdminDistrict")]
  AdminDistrict,
  #[sdk(rename = "AdminDistrict2")]
  AdminDistrict2,
  #[sdk(rename = "AdminDistrict3")]
  AdminDistrict3,
  #[sdk(rename = "Continent")]
  Continent,
  #[sdk(rename = "CountryRegion")]
  CountryRegion,
  #[sdk(rename = "Locality")]
  Locality,
  #[sdk(rename = "Ocean")]
  Ocean,
  #[sdk(rename = "Planet")]
  Planet,
  #[sdk(rename = "PostalCode")]
  PostalCode,
  #[sdk(rename = "Region")]
  Region,
  #[sdk(rename = "Unsupported")]
  Unsupported,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum GeoProjectionType {
  #[sdk(rename = "mercator")]
  #[default]
  Mercator,
  #[sdk(rename = "miller")]
  Miller,
  #[sdk(rename = "robinson")]
  Robinson,
  #[sdk(rename = "albers")]
  Albers,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum GeoMappingLevel {
  #[sdk(rename = "dataOnly")]
  #[default]
  DataOnly,
  #[sdk(rename = "postalCode")]
  PostalCode,
  #[sdk(rename = "county")]
  County,
  #[sdk(rename = "state")]
  State,
  #[sdk(rename = "countryRegion")]
  CountryRegion,
  #[sdk(rename = "countryRegionList")]
  CountryRegionList,
  #[sdk(rename = "world")]
  World,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum QuartileMethod {
  #[sdk(rename = "inclusive")]
  #[default]
  Inclusive,
  #[sdk(rename = "exclusive")]
  Exclusive,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum DataLabelPos {
  #[sdk(rename = "bestFit")]
  #[default]
  BestFit,
  #[sdk(rename = "b")]
  B,
  #[sdk(rename = "ctr")]
  Ctr,
  #[sdk(rename = "inBase")]
  InBase,
  #[sdk(rename = "inEnd")]
  InEnd,
  #[sdk(rename = "l")]
  L,
  #[sdk(rename = "outEnd")]
  OutEnd,
  #[sdk(rename = "r")]
  R,
  #[sdk(rename = "t")]
  T,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum PageOrientation {
  #[sdk(rename = "default")]
  #[default]
  Default,
  #[sdk(rename = "portrait")]
  Portrait,
  #[sdk(rename = "landscape")]
  Landscape,
}
/// Defines the ChartSpace Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:chartSpace")]
pub struct ChartSpace {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// version
  #[sdk(attr(qname = ":version"))]
  pub version: Option<crate::simple_type::StringValue>,
  /// featureList
  #[sdk(attr(qname = ":featureList"))]
  pub feature_list: Option<crate::simple_type::StringValue>,
  /// fallbackImg
  #[sdk(attr(qname = ":fallbackImg"))]
  pub fallback_img: Option<crate::simple_type::StringValue>,
  /// Defines the ChartData Class.
  #[sdk(child(qname = "cx:chartData"))]
  pub chart_data: Option<std::boxed::Box<ChartData>>,
  /// Defines the Chart Class.
  #[sdk(child(qname = "cx:chart"))]
  pub chart: std::boxed::Box<Chart>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TxPrTextBody Class.
  #[sdk(child(qname = "cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// Defines the ColorMappingType Class.
  #[sdk(child(qname = "cx:clrMapOvr"))]
  pub color_mapping_type: Option<std::boxed::Box<ColorMappingType>>,
  /// Defines the FormatOverrides Class.
  #[sdk(child(qname = "cx:fmtOvrs"))]
  pub format_overrides: Option<FormatOverrides>,
  /// Defines the PrintSettings Class.
  #[sdk(child(qname = "cx:printSettings"))]
  pub print_settings: Option<std::boxed::Box<PrintSettings>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the RelId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:chart")]
pub struct RelId {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the Openxmlsdk_49BECFFA_3B03_4D13_8272_D6CCB22579E3XsdunsignedInt Class.
pub type Openxmlsdk49becffa3b034d138272D6ccb22579e3XsdunsignedInt = crate::simple_type::UInt32Value;
/// Defines the BinCountXsdunsignedInt Class.
pub type BinCountXsdunsignedInt = crate::simple_type::UInt32Value;
/// Defines the Extension2 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:ext")]
pub struct Extension2 {
  /// uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<std::boxed::Box<[u8]>>,
}
/// Defines the MinColorSolidColorFillProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:minColor")]
pub struct MinColorSolidColorFillProperties {
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
  pub min_color_solid_color_fill_properties_choice: Option<MinColorSolidColorFillPropertiesChoice>,
}
/// Defines the MidColorSolidColorFillProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:midColor")]
pub struct MidColorSolidColorFillProperties {
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
  pub mid_color_solid_color_fill_properties_choice: Option<MidColorSolidColorFillPropertiesChoice>,
}
/// Defines the MaxColorSolidColorFillProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:maxColor")]
pub struct MaxColorSolidColorFillProperties {
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
  pub max_color_solid_color_fill_properties_choice: Option<MaxColorSolidColorFillPropertiesChoice>,
}
/// Defines the ChartStringValue Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:pt")]
pub struct ChartStringValue {
  /// idx
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Formula Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:f")]
pub struct Formula {
  /// dir
  #[sdk(attr(qname = ":dir"))]
  pub dir: Option<FormulaDirection>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the NfFormula Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:nf")]
pub struct NfFormula {
  /// dir
  #[sdk(attr(qname = ":dir"))]
  pub dir: Option<FormulaDirection>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the StringLevel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:lvl")]
pub struct StringLevel {
  /// ptCount
  #[sdk(attr(qname = ":ptCount"))]
  pub pt_count: crate::simple_type::UInt32Value,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Defines the ChartStringValue Class.
  #[sdk(child(qname = "cx:pt"))]
  pub chart_string_value: Vec<ChartStringValue>,
}
/// Defines the NumericValue Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:pt")]
pub struct NumericValue {
  /// idx
  #[sdk(attr(qname = ":idx"))]
  pub idx: crate::simple_type::UInt32Value,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::DoubleValue>,
}
/// Defines the NumericLevel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:lvl")]
pub struct NumericLevel {
  /// ptCount
  #[sdk(attr(qname = ":ptCount"))]
  pub pt_count: crate::simple_type::UInt32Value,
  /// formatCode
  #[sdk(attr(qname = ":formatCode"))]
  pub format_code: Option<crate::simple_type::StringValue>,
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Defines the NumericValue Class.
  #[sdk(child(qname = "cx:pt"))]
  pub numeric_value: Vec<NumericValue>,
}
/// Defines the NumericDimension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:numDim")]
pub struct NumericDimension {
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: NumericDimensionType,
  #[sdk(
        choice(
            sequence(
                variant = Sequence,
                child(qname = "cx:f"),
                child(qname = "cx:nf"),
                child(qname = "cx:lvl")
            ),
            child(variant = NumericLevel, qname = "cx:lvl")
        )
    )]
  pub numeric_dimension_choice: Option<NumericDimensionChoice>,
}
/// Defines the StringDimension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:strDim")]
pub struct StringDimension {
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: StringDimensionType,
  #[sdk(
        choice(
            sequence(
                variant = Sequence,
                child(qname = "cx:f"),
                child(qname = "cx:nf"),
                child(qname = "cx:lvl")
            ),
            child(variant = StringLevel, qname = "cx:lvl")
        )
    )]
  pub string_dimension_choice: Option<StringDimensionChoice>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:extLst")]
pub struct ExtensionList {
  /// Defines the Extension2 Class.
  #[sdk(child(qname = "cx:ext"))]
  pub extension2: Vec<Extension2>,
}
/// Defines the ExternalData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:externalData")]
pub struct ExternalData {
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// RelId of the relationship for the external data
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
  /// True if the external link should automatically update
  #[sdk(attr(qname = "cx:autoUpdate"))]
  pub cx_auto_update: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Data Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:data")]
pub struct Data {
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  #[sdk(
        choice(
            child(variant = NumericDimension, qname = "cx:numDim"),
            child(variant = StringDimension, qname = "cx:strDim")
        )
    )]
  pub data_choice: Vec<DataChoice>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the VXsdstring Class.
pub type VXsdstring = crate::simple_type::StringValue;
/// Defines the CopyrightXsdstring Class.
pub type CopyrightXsdstring = crate::simple_type::StringValue;
/// Defines the SeparatorXsdstring Class.
pub type SeparatorXsdstring = crate::simple_type::StringValue;
/// Defines the OddHeaderXsdstring Class.
pub type OddHeaderXsdstring = crate::simple_type::StringValue;
/// Defines the OddFooterXsdstring Class.
pub type OddFooterXsdstring = crate::simple_type::StringValue;
/// Defines the EvenHeaderXsdstring Class.
pub type EvenHeaderXsdstring = crate::simple_type::StringValue;
/// Defines the EvenFooterXsdstring Class.
pub type EvenFooterXsdstring = crate::simple_type::StringValue;
/// Defines the FirstHeaderXsdstring Class.
pub type FirstHeaderXsdstring = crate::simple_type::StringValue;
/// Defines the FirstFooterXsdstring Class.
pub type FirstFooterXsdstring = crate::simple_type::StringValue;
/// Defines the TextData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:txData")]
pub struct TextData {
  #[sdk(
        choice(
            sequence(
                variant = Sequence,
                child(
                    field = formula,
                    ty = "std :: boxed :: Box < Formula >",
                    qname = "cx:f"
                ),
                text_child(
                    field = v_xsdstring,
                    ty = "Option < VXsdstring >",
                    qname = "cx:v"
                )
            ),
            text_child(variant = VXsdstring, qname = "cx:v")
        )
    )]
  pub text_data_choice: Option<TextDataChoice>,
}
/// Defines the RichTextBody Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:rich")]
pub struct RichTextBody {
  /// Body Properties
  #[sdk(child(qname = "a:bodyPr"))]
  pub body_properties: std::boxed::Box<crate::schemas::a::BodyProperties>,
  /// Text List Styles
  #[sdk(child(qname = "a:lstStyle"))]
  pub list_style: Option<std::boxed::Box<crate::schemas::a::ListStyle>>,
  /// Text Paragraphs.
  #[sdk(child(qname = "a:p"))]
  pub paragraph: Vec<crate::schemas::a::Paragraph>,
}
/// Defines the TxPrTextBody Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:txPr")]
pub struct TxPrTextBody {
  /// Body Properties
  #[sdk(child(qname = "a:bodyPr"))]
  pub body_properties: std::boxed::Box<crate::schemas::a::BodyProperties>,
  /// Text List Styles
  #[sdk(child(qname = "a:lstStyle"))]
  pub list_style: Option<std::boxed::Box<crate::schemas::a::ListStyle>>,
  /// Text Paragraphs.
  #[sdk(child(qname = "a:p"))]
  pub paragraph: Vec<crate::schemas::a::Paragraph>,
}
/// Defines the Text Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:tx")]
pub struct Text {
  #[sdk(
        choice(
            child(variant = TextData, qname = "cx:txData"),
            child(variant = RichTextBody, qname = "cx:rich")
        )
    )]
  pub text_choice: Option<TextChoice>,
}
/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:spPr")]
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
/// Defines the Offset Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:offset")]
pub struct Offset {
  /// top
  #[sdk(attr(qname = ":top"))]
  pub top: crate::simple_type::DoubleValue,
  /// left
  #[sdk(attr(qname = ":left"))]
  pub left: crate::simple_type::DoubleValue,
}
/// Defines the AxisUnitsLabel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:unitsLabel")]
pub struct AxisUnitsLabel {
  /// Defines the Text Class.
  #[sdk(child(qname = "cx:tx"))]
  pub text: Option<std::boxed::Box<Text>>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TxPrTextBody Class.
  #[sdk(child(qname = "cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CategoryAxisScaling Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:catScaling")]
pub struct CategoryAxisScaling {
  /// gapWidth
  #[sdk(attr(qname = ":gapWidth"))]
  #[sdk(number_range(
    source = 0u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false,
  ))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub gap_width: Option<crate::simple_type::StringValue>,
}
/// Defines the ValueAxisScaling Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:valScaling")]
pub struct ValueAxisScaling {
  /// max
  #[sdk(attr(qname = ":max"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:double"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub max: Option<crate::simple_type::StringValue>,
  /// min
  #[sdk(attr(qname = ":min"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:double"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub min: Option<crate::simple_type::StringValue>,
  /// majorUnit
  #[sdk(attr(qname = ":majorUnit"))]
  #[sdk(number_range(
    source = 0u32,
    union = 0u64,
    min = "0",
    min_inclusive = false,
    max_inclusive = false,
  ))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub major_unit: Option<crate::simple_type::StringValue>,
  /// minorUnit
  #[sdk(attr(qname = ":minorUnit"))]
  #[sdk(number_range(
    source = 0u32,
    union = 0u64,
    min = "0",
    min_inclusive = false,
    max_inclusive = false,
  ))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub minor_unit: Option<crate::simple_type::StringValue>,
}
/// Defines the AxisTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:title")]
pub struct AxisTitle {
  /// Defines the Text Class.
  #[sdk(child(qname = "cx:tx"))]
  pub text: Option<std::boxed::Box<Text>>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TxPrTextBody Class.
  #[sdk(child(qname = "cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// Defines the Offset Class.
  #[sdk(child(qname = "cx:offset"))]
  pub offset: Option<Offset>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the AxisUnits Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:units")]
pub struct AxisUnits {
  /// unit
  #[sdk(attr(qname = ":unit"))]
  pub unit: Option<AxisUnit>,
  /// Defines the AxisUnitsLabel Class.
  #[sdk(child(qname = "cx:unitsLabel"))]
  pub axis_units_label: Option<std::boxed::Box<AxisUnitsLabel>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the MajorGridlinesGridlines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:majorGridlines")]
pub struct MajorGridlinesGridlines {
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the MinorGridlinesGridlines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:minorGridlines")]
pub struct MinorGridlinesGridlines {
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the MajorTickMarksTickMarks Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:majorTickMarks")]
pub struct MajorTickMarksTickMarks {
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<TickMarksType>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the MinorTickMarksTickMarks Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:minorTickMarks")]
pub struct MinorTickMarksTickMarks {
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<TickMarksType>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TickLabels Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:tickLabels")]
pub struct TickLabels {
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the NumberFormat Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:numFmt")]
pub struct NumberFormat {
  /// formatCode
  #[sdk(attr(qname = ":formatCode"))]
  pub format_code: crate::simple_type::StringValue,
  /// sourceLinked
  #[sdk(attr(qname = ":sourceLinked"))]
  pub source_linked: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Xsddouble Class.
pub type Xsddouble = crate::simple_type::DoubleValue;
/// Defines the Address Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:address")]
pub struct Address {
  /// address1
  #[sdk(attr(qname = ":address1"))]
  pub address1: Option<crate::simple_type::StringValue>,
  /// countryRegion
  #[sdk(attr(qname = ":countryRegion"))]
  pub country_region: Option<crate::simple_type::StringValue>,
  /// adminDistrict1
  #[sdk(attr(qname = ":adminDistrict1"))]
  pub admin_district1: Option<crate::simple_type::StringValue>,
  /// adminDistrict2
  #[sdk(attr(qname = ":adminDistrict2"))]
  pub admin_district2: Option<crate::simple_type::StringValue>,
  /// postalCode
  #[sdk(attr(qname = ":postalCode"))]
  pub postal_code: Option<crate::simple_type::StringValue>,
  /// locality
  #[sdk(attr(qname = ":locality"))]
  pub locality: Option<crate::simple_type::StringValue>,
  /// isoCountryCode
  #[sdk(attr(qname = ":isoCountryCode"))]
  pub iso_country_code: Option<crate::simple_type::StringValue>,
}
/// Defines the GeoLocation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoLocation")]
pub struct GeoLocation {
  /// latitude
  #[sdk(attr(qname = ":latitude"))]
  pub latitude: Option<crate::simple_type::DoubleValue>,
  /// longitude
  #[sdk(attr(qname = ":longitude"))]
  pub longitude: Option<crate::simple_type::DoubleValue>,
  /// entityName
  #[sdk(attr(qname = ":entityName"))]
  pub entity_name: crate::simple_type::StringValue,
  /// entityType
  #[sdk(attr(qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
  /// Defines the Address Class.
  #[sdk(child(qname = "cx:address"))]
  pub address: Option<Address>,
}
/// Defines the GeoLocationQuery Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoLocationQuery")]
pub struct GeoLocationQuery {
  /// countryRegion
  #[sdk(attr(qname = ":countryRegion"))]
  pub country_region: Option<crate::simple_type::StringValue>,
  /// adminDistrict1
  #[sdk(attr(qname = ":adminDistrict1"))]
  pub admin_district1: Option<crate::simple_type::StringValue>,
  /// adminDistrict2
  #[sdk(attr(qname = ":adminDistrict2"))]
  pub admin_district2: Option<crate::simple_type::StringValue>,
  /// postalCode
  #[sdk(attr(qname = ":postalCode"))]
  pub postal_code: Option<crate::simple_type::StringValue>,
  /// entityType
  #[sdk(attr(qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
}
/// Defines the GeoLocations Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoLocations")]
pub struct GeoLocations {
  /// Defines the GeoLocation Class.
  #[sdk(child(qname = "cx:geoLocation"))]
  pub geo_location: Option<std::boxed::Box<GeoLocation>>,
}
/// Defines the GeoLocationQueryResult Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoLocationQueryResult")]
pub struct GeoLocationQueryResult {
  /// Defines the GeoLocationQuery Class.
  #[sdk(child(qname = "cx:geoLocationQuery"))]
  pub geo_location_query: Option<GeoLocationQuery>,
  /// Defines the GeoLocations Class.
  #[sdk(child(qname = "cx:geoLocations"))]
  pub geo_locations: Option<std::boxed::Box<GeoLocations>>,
}
/// Defines the GeoPolygon Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoPolygon")]
pub struct GeoPolygon {
  /// polygonId
  #[sdk(attr(qname = ":polygonId"))]
  pub polygon_id: crate::simple_type::StringValue,
  /// numPoints
  #[sdk(attr(qname = ":numPoints"))]
  pub num_points: crate::simple_type::IntegerValue,
  /// pcaRings
  #[sdk(attr(qname = ":pcaRings"))]
  pub pca_rings: crate::simple_type::StringValue,
}
/// Defines the GeoPolygons Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoPolygons")]
pub struct GeoPolygons {
  /// Defines the GeoPolygon Class.
  #[sdk(child(qname = "cx:geoPolygon"))]
  pub geo_polygon: Vec<GeoPolygon>,
}
/// Defines the Copyrights Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:copyrights")]
pub struct Copyrights {
  /// Defines the CopyrightXsdstring Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "cx:copyright"))]
  pub copyright_xsdstring: Vec<CopyrightXsdstring>,
}
/// Defines the GeoDataEntityQuery Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoDataEntityQuery")]
pub struct GeoDataEntityQuery {
  /// entityType
  #[sdk(attr(qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
  /// entityId
  #[sdk(attr(qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
}
/// Defines the GeoData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoData")]
pub struct GeoData {
  /// entityName
  #[sdk(attr(qname = ":entityName"))]
  pub entity_name: crate::simple_type::StringValue,
  /// entityId
  #[sdk(attr(qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
  /// east
  #[sdk(attr(qname = ":east"))]
  pub east: crate::simple_type::DoubleValue,
  /// west
  #[sdk(attr(qname = ":west"))]
  pub west: crate::simple_type::DoubleValue,
  /// north
  #[sdk(attr(qname = ":north"))]
  pub north: crate::simple_type::DoubleValue,
  /// south
  #[sdk(attr(qname = ":south"))]
  pub south: crate::simple_type::DoubleValue,
  /// Defines the GeoPolygons Class.
  #[sdk(child(qname = "cx:geoPolygons"))]
  pub geo_polygons: Option<GeoPolygons>,
  /// Defines the Copyrights Class.
  #[sdk(child(qname = "cx:copyrights"))]
  pub copyrights: Option<Copyrights>,
}
/// Defines the GeoDataEntityQueryResult Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoDataEntityQueryResult")]
pub struct GeoDataEntityQueryResult {
  /// Defines the GeoDataEntityQuery Class.
  #[sdk(child(qname = "cx:geoDataEntityQuery"))]
  pub geo_data_entity_query: Option<GeoDataEntityQuery>,
  /// Defines the GeoData Class.
  #[sdk(child(qname = "cx:geoData"))]
  pub geo_data: Option<std::boxed::Box<GeoData>>,
}
/// Defines the GeoDataPointQuery Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoDataPointQuery")]
pub struct GeoDataPointQuery {
  /// entityType
  #[sdk(attr(qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
  /// latitude
  #[sdk(attr(qname = ":latitude"))]
  pub latitude: crate::simple_type::DoubleValue,
  /// longitude
  #[sdk(attr(qname = ":longitude"))]
  pub longitude: crate::simple_type::DoubleValue,
}
/// Defines the GeoDataPointToEntityQuery Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoDataPointToEntityQuery")]
pub struct GeoDataPointToEntityQuery {
  /// entityType
  #[sdk(attr(qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
  /// entityId
  #[sdk(attr(qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
}
/// Defines the GeoDataPointToEntityQueryResult Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoDataPointToEntityQueryResult")]
pub struct GeoDataPointToEntityQueryResult {
  /// Defines the GeoDataPointQuery Class.
  #[sdk(child(qname = "cx:geoDataPointQuery"))]
  pub geo_data_point_query: Option<GeoDataPointQuery>,
  /// Defines the GeoDataPointToEntityQuery Class.
  #[sdk(child(qname = "cx:geoDataPointToEntityQuery"))]
  pub geo_data_point_to_entity_query: Option<GeoDataPointToEntityQuery>,
}
/// Defines the EntityType Class.
pub type EntityType = EntityTypeEnum;
/// Defines the GeoChildTypes Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoChildTypes")]
pub struct GeoChildTypes {
  /// Defines the EntityType Class.
  #[sdk(text_child(qname = "cx:entityType"))]
  pub entity_type: Vec<EntityType>,
}
/// Defines the GeoHierarchyEntity Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoHierarchyEntity")]
pub struct GeoHierarchyEntity {
  /// entityName
  #[sdk(attr(qname = ":entityName"))]
  pub entity_name: crate::simple_type::StringValue,
  /// entityId
  #[sdk(attr(qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
  /// entityType
  #[sdk(attr(qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
}
/// Defines the GeoChildEntitiesQuery Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoChildEntitiesQuery")]
pub struct GeoChildEntitiesQuery {
  /// entityId
  #[sdk(attr(qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
  /// Defines the GeoChildTypes Class.
  #[sdk(child(qname = "cx:geoChildTypes"))]
  pub geo_child_types: Option<GeoChildTypes>,
}
/// Defines the GeoChildEntities Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoChildEntities")]
pub struct GeoChildEntities {
  /// Defines the GeoHierarchyEntity Class.
  #[sdk(child(qname = "cx:geoHierarchyEntity"))]
  pub geo_hierarchy_entity: Vec<GeoHierarchyEntity>,
}
/// Defines the GeoChildEntitiesQueryResult Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoChildEntitiesQueryResult")]
pub struct GeoChildEntitiesQueryResult {
  /// Defines the GeoChildEntitiesQuery Class.
  #[sdk(child(qname = "cx:geoChildEntitiesQuery"))]
  pub geo_child_entities_query: Option<std::boxed::Box<GeoChildEntitiesQuery>>,
  /// Defines the GeoChildEntities Class.
  #[sdk(child(qname = "cx:geoChildEntities"))]
  pub geo_child_entities: Option<GeoChildEntities>,
}
/// Defines the GeoParentEntitiesQuery Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoParentEntitiesQuery")]
pub struct GeoParentEntitiesQuery {
  /// entityId
  #[sdk(attr(qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
}
/// Defines the GeoEntity Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoEntity")]
pub struct GeoEntity {
  /// entityName
  #[sdk(attr(qname = ":entityName"))]
  pub entity_name: crate::simple_type::StringValue,
  /// entityType
  #[sdk(attr(qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
}
/// Defines the GeoParentEntity Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoParentEntity")]
pub struct GeoParentEntity {
  /// entityId
  #[sdk(attr(qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
}
/// Defines the GeoParentEntitiesQueryResult Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoParentEntitiesQueryResult")]
pub struct GeoParentEntitiesQueryResult {
  /// Defines the GeoParentEntitiesQuery Class.
  #[sdk(child(qname = "cx:geoParentEntitiesQuery"))]
  pub geo_parent_entities_query: std::boxed::Box<GeoParentEntitiesQuery>,
  /// Defines the GeoEntity Class.
  #[sdk(child(qname = "cx:geoEntity"))]
  pub geo_entity: Option<GeoEntity>,
  /// Defines the GeoParentEntity Class.
  #[sdk(child(qname = "cx:geoParentEntity"))]
  pub geo_parent_entity: Option<GeoParentEntity>,
}
/// Defines the GeoLocationQueryResults Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoLocationQueryResults")]
pub struct GeoLocationQueryResults {
  /// Defines the GeoLocationQueryResult Class.
  #[sdk(child(qname = "cx:geoLocationQueryResult"))]
  pub geo_location_query_result: Vec<GeoLocationQueryResult>,
}
/// Defines the GeoDataEntityQueryResults Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoDataEntityQueryResults")]
pub struct GeoDataEntityQueryResults {
  /// Defines the GeoDataEntityQueryResult Class.
  #[sdk(child(qname = "cx:geoDataEntityQueryResult"))]
  pub geo_data_entity_query_result: Vec<GeoDataEntityQueryResult>,
}
/// Defines the GeoDataPointToEntityQueryResults Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoDataPointToEntityQueryResults")]
pub struct GeoDataPointToEntityQueryResults {
  /// Defines the GeoDataPointToEntityQueryResult Class.
  #[sdk(child(qname = "cx:geoDataPointToEntityQueryResult"))]
  pub geo_data_point_to_entity_query_result: Vec<GeoDataPointToEntityQueryResult>,
}
/// Defines the GeoChildEntitiesQueryResults Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoChildEntitiesQueryResults")]
pub struct GeoChildEntitiesQueryResults {
  /// Defines the GeoChildEntitiesQueryResult Class.
  #[sdk(child(qname = "cx:geoChildEntitiesQueryResult"))]
  pub geo_child_entities_query_result: Vec<GeoChildEntitiesQueryResult>,
}
/// Defines the GeoParentEntitiesQueryResults Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoParentEntitiesQueryResults")]
pub struct GeoParentEntitiesQueryResults {
  /// Defines the GeoParentEntitiesQueryResult Class.
  #[sdk(child(qname = "cx:geoParentEntitiesQueryResult"))]
  pub geo_parent_entities_query_result: Vec<GeoParentEntitiesQueryResult>,
}
/// Defines the Xsdbase64Binary Class.
pub type Xsdbase64Binary = crate::simple_type::Base64BinaryValue;
/// Defines the Clear Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:clear")]
pub struct Clear {
  /// Defines the GeoLocationQueryResults Class.
  #[sdk(child(qname = "cx:geoLocationQueryResults"))]
  pub geo_location_query_results: Option<GeoLocationQueryResults>,
  /// Defines the GeoDataEntityQueryResults Class.
  #[sdk(child(qname = "cx:geoDataEntityQueryResults"))]
  pub geo_data_entity_query_results: Option<GeoDataEntityQueryResults>,
  /// Defines the GeoDataPointToEntityQueryResults Class.
  #[sdk(child(qname = "cx:geoDataPointToEntityQueryResults"))]
  pub geo_data_point_to_entity_query_results: Option<GeoDataPointToEntityQueryResults>,
  /// Defines the GeoChildEntitiesQueryResults Class.
  #[sdk(child(qname = "cx:geoChildEntitiesQueryResults"))]
  pub geo_child_entities_query_results: Option<GeoChildEntitiesQueryResults>,
  /// Defines the GeoParentEntitiesQueryResults Class.
  #[sdk(child(qname = "cx:geoParentEntitiesQueryResults"))]
  pub geo_parent_entities_query_results: Option<GeoParentEntitiesQueryResults>,
}
/// Defines the GeoCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geoCache")]
pub struct GeoCache {
  /// provider
  #[sdk(attr(qname = ":provider"))]
  pub provider: crate::simple_type::StringValue,
  #[sdk(
        choice(
            text_child(variant = Xsdbase64Binary, qname = "cx:binary"),
            child(variant = Clear, qname = "cx:clear")
        )
    )]
  pub geo_cache_choice: Vec<GeoCacheChoice>,
}
/// Defines the ParentLabelLayout Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:parentLabelLayout")]
pub struct ParentLabelLayout {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub parent_label_layout_val: ParentLabelLayoutVal,
}
/// Defines the RegionLabelLayout Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:regionLabelLayout")]
pub struct RegionLabelLayout {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: RegionLabelLayoutEnum,
}
/// Defines the SeriesElementVisibilities Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:visibility")]
pub struct SeriesElementVisibilities {
  /// connectorLines
  #[sdk(attr(qname = ":connectorLines"))]
  pub connector_lines: Option<crate::simple_type::BooleanValue>,
  /// meanLine
  #[sdk(attr(qname = ":meanLine"))]
  pub mean_line: Option<crate::simple_type::BooleanValue>,
  /// meanMarker
  #[sdk(attr(qname = ":meanMarker"))]
  pub mean_marker: Option<crate::simple_type::BooleanValue>,
  /// nonoutliers
  #[sdk(attr(qname = ":nonoutliers"))]
  pub nonoutliers: Option<crate::simple_type::BooleanValue>,
  /// outliers
  #[sdk(attr(qname = ":outliers"))]
  pub outliers: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Binning Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:binning")]
pub struct Binning {
  /// intervalClosed
  #[sdk(attr(qname = ":intervalClosed"))]
  pub interval_closed: Option<IntervalClosedSide>,
  /// underflow
  #[sdk(attr(qname = ":underflow"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:double"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub underflow: Option<crate::simple_type::StringValue>,
  /// overflow
  #[sdk(attr(qname = ":overflow"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:double"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub overflow: Option<crate::simple_type::StringValue>,
  #[sdk(
        choice(
            text_child(
                variant = Xsddouble,
                simple_type = "DoubleValue",
                qname = "cx:binSize"
            ),
            text_child(variant = BinCountXsdunsignedInt, qname = "cx:binCount")
        )
    )]
  pub binning_choice: Option<BinningChoice>,
}
/// Defines the Geography Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:geography")]
pub struct Geography {
  /// projectionType
  #[sdk(attr(qname = ":projectionType"))]
  pub projection_type: Option<GeoProjectionType>,
  /// viewedRegionType
  #[sdk(attr(qname = ":viewedRegionType"))]
  pub viewed_region_type: Option<GeoMappingLevel>,
  /// cultureLanguage
  #[sdk(attr(qname = ":cultureLanguage"))]
  #[sdk(string_format(kind = "token"))]
  pub culture_language: crate::simple_type::StringValue,
  /// cultureRegion
  #[sdk(attr(qname = ":cultureRegion"))]
  pub culture_region: crate::simple_type::StringValue,
  /// attribution
  #[sdk(attr(qname = ":attribution"))]
  pub attribution: crate::simple_type::StringValue,
  /// Defines the GeoCache Class.
  #[sdk(child(qname = "cx:geoCache"))]
  pub geo_cache: Option<GeoCache>,
}
/// Defines the Statistics Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:statistics")]
pub struct Statistics {
  /// quartileMethod
  #[sdk(attr(qname = ":quartileMethod"))]
  pub quartile_method: Option<QuartileMethod>,
}
/// Defines the Subtotals Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:subtotals")]
pub struct Subtotals {
  /// Index of subtotal data point.
  #[sdk(child(qname = "cx:idx"))]
  pub unsigned_integer_type: Vec<UnsignedIntegerType>,
}
/// Defines the NumberColorPosition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:number")]
pub struct NumberColorPosition {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Defines the PercentageColorPosition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:percent")]
pub struct PercentageColorPosition {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Defines the MinValueColorEndPosition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:min")]
pub struct MinValueColorEndPosition {
  #[sdk(
        choice(
            empty_child(variant = ExtremeValueColorPosition, qname = "cx:extremeValue"),
            child(variant = NumberColorPosition, qname = "cx:number"),
            child(variant = PercentageColorPosition, qname = "cx:percent")
        )
    )]
  pub min_value_color_end_position_choice: Option<MinValueColorEndPositionChoice>,
}
/// Defines the MaxValueColorEndPosition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:max")]
pub struct MaxValueColorEndPosition {
  #[sdk(
        choice(
            empty_child(variant = ExtremeValueColorPosition, qname = "cx:extremeValue"),
            child(variant = NumberColorPosition, qname = "cx:number"),
            child(variant = PercentageColorPosition, qname = "cx:percent")
        )
    )]
  pub max_value_color_end_position_choice: Option<MaxValueColorEndPositionChoice>,
}
/// Defines the ValueColorMiddlePosition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:mid")]
pub struct ValueColorMiddlePosition {
  #[sdk(
        choice(
            child(variant = NumberColorPosition, qname = "cx:number"),
            child(variant = PercentageColorPosition, qname = "cx:percent")
        )
    )]
  pub value_color_middle_position_choice: Option<ValueColorMiddlePositionChoice>,
}
/// Defines the DataLabelVisibilities Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:visibility")]
pub struct DataLabelVisibilities {
  /// seriesName
  #[sdk(attr(qname = ":seriesName"))]
  pub series_name: Option<crate::simple_type::BooleanValue>,
  /// categoryName
  #[sdk(attr(qname = ":categoryName"))]
  pub category_name: Option<crate::simple_type::BooleanValue>,
  /// value
  #[sdk(attr(qname = ":value"))]
  pub value: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DataLabel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:dataLabel")]
pub struct DataLabel {
  /// idx
  #[sdk(attr(qname = ":idx"))]
  pub idx: crate::simple_type::UInt32Value,
  /// pos
  #[sdk(attr(qname = ":pos"))]
  pub pos: Option<DataLabelPos>,
  /// Defines the NumberFormat Class.
  #[sdk(child(qname = "cx:numFmt"))]
  pub number_format: Option<NumberFormat>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TxPrTextBody Class.
  #[sdk(child(qname = "cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// Defines the DataLabelVisibilities Class.
  #[sdk(child(qname = "cx:visibility"))]
  pub data_label_visibilities: Option<DataLabelVisibilities>,
  /// Defines the SeparatorXsdstring Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "cx:separator"))]
  pub separator_xsdstring: Option<SeparatorXsdstring>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the DataLabelHidden Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:dataLabelHidden")]
pub struct DataLabelHidden {
  /// idx
  #[sdk(attr(qname = ":idx"))]
  pub idx: crate::simple_type::UInt32Value,
}
/// Defines the ValueColors Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:valueColors")]
pub struct ValueColors {
  /// Defines the MinColorSolidColorFillProperties Class.
  #[sdk(child(qname = "cx:minColor"))]
  pub min_color_solid_color_fill_properties:
    Option<std::boxed::Box<MinColorSolidColorFillProperties>>,
  /// Defines the MidColorSolidColorFillProperties Class.
  #[sdk(child(qname = "cx:midColor"))]
  pub mid_color_solid_color_fill_properties:
    Option<std::boxed::Box<MidColorSolidColorFillProperties>>,
  /// Defines the MaxColorSolidColorFillProperties Class.
  #[sdk(child(qname = "cx:maxColor"))]
  pub max_color_solid_color_fill_properties:
    Option<std::boxed::Box<MaxColorSolidColorFillProperties>>,
}
/// Defines the ValueColorPositions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:valueColorPositions")]
pub struct ValueColorPositions {
  /// count
  #[sdk(attr(qname = ":count"))]
  #[sdk(number_range(range = 2..= 3))]
  pub count: Option<crate::simple_type::Int32Value>,
  /// Defines the MinValueColorEndPosition Class.
  #[sdk(child(qname = "cx:min"))]
  pub min_value_color_end_position: Option<std::boxed::Box<MinValueColorEndPosition>>,
  /// Defines the ValueColorMiddlePosition Class.
  #[sdk(child(qname = "cx:mid"))]
  pub value_color_middle_position: Option<std::boxed::Box<ValueColorMiddlePosition>>,
  /// Defines the MaxValueColorEndPosition Class.
  #[sdk(child(qname = "cx:max"))]
  pub max_value_color_end_position: Option<std::boxed::Box<MaxValueColorEndPosition>>,
}
/// Defines the DataPoint Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:dataPt")]
pub struct DataPoint {
  /// idx
  #[sdk(attr(qname = ":idx"))]
  pub idx: crate::simple_type::UInt32Value,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the DataLabels Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:dataLabels")]
pub struct DataLabels {
  /// pos
  #[sdk(attr(qname = ":pos"))]
  pub pos: Option<DataLabelPos>,
  /// Defines the NumberFormat Class.
  #[sdk(child(qname = "cx:numFmt"))]
  pub number_format: Option<NumberFormat>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TxPrTextBody Class.
  #[sdk(child(qname = "cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// Defines the DataLabelVisibilities Class.
  #[sdk(child(qname = "cx:visibility"))]
  pub data_label_visibilities: Option<DataLabelVisibilities>,
  /// Defines the SeparatorXsdstring Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "cx:separator"))]
  pub separator_xsdstring: Option<SeparatorXsdstring>,
  /// Defines the DataLabel Class.
  #[sdk(child(qname = "cx:dataLabel"))]
  pub data_label: Vec<DataLabel>,
  /// Defines the DataLabelHidden Class.
  #[sdk(child(qname = "cx:dataLabelHidden"))]
  pub data_label_hidden: Vec<DataLabelHidden>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the DataId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:dataId")]
pub struct DataId {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the SeriesLayoutProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:layoutPr")]
pub struct SeriesLayoutProperties {
  /// Defines the ParentLabelLayout Class.
  #[sdk(child(qname = "cx:parentLabelLayout"))]
  pub parent_label_layout: Option<ParentLabelLayout>,
  /// Defines the RegionLabelLayout Class.
  #[sdk(child(qname = "cx:regionLabelLayout"))]
  pub region_label_layout: Option<RegionLabelLayout>,
  /// Defines the SeriesElementVisibilities Class.
  #[sdk(child(qname = "cx:visibility"))]
  pub series_element_visibilities: Option<SeriesElementVisibilities>,
  #[sdk(
        choice(
            empty_child(variant = Aggregation, qname = "cx:aggregation"),
            child(variant = Binning, qname = "cx:binning")
        )
    )]
  pub series_layout_properties_choice: Option<SeriesLayoutPropertiesChoice>,
  /// Defines the Geography Class.
  #[sdk(child(qname = "cx:geography"))]
  pub geography: Option<std::boxed::Box<Geography>>,
  /// Defines the Statistics Class.
  #[sdk(child(qname = "cx:statistics"))]
  pub statistics: Option<Statistics>,
  /// Defines the Subtotals Class.
  #[sdk(child(qname = "cx:subtotals"))]
  pub subtotals: Option<Subtotals>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the AxisId Class.
pub type AxisId = crate::simple_type::UInt32Value;
/// Defines the PlotSurface Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:plotSurface")]
pub struct PlotSurface {
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Series Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:series")]
pub struct Series {
  /// layoutId
  #[sdk(attr(qname = ":layoutId"))]
  pub layout_id: SeriesLayout,
  /// hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// ownerIdx
  #[sdk(attr(qname = ":ownerIdx"))]
  pub owner_idx: Option<crate::simple_type::UInt32Value>,
  /// uniqueId
  #[sdk(attr(qname = ":uniqueId"))]
  pub unique_id: Option<crate::simple_type::StringValue>,
  /// formatIdx
  #[sdk(attr(qname = ":formatIdx"))]
  pub format_idx: Option<crate::simple_type::UInt32Value>,
  /// Defines the Text Class.
  #[sdk(child(qname = "cx:tx"))]
  pub text: Option<std::boxed::Box<Text>>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ValueColors Class.
  #[sdk(child(qname = "cx:valueColors"))]
  pub value_colors: Option<std::boxed::Box<ValueColors>>,
  /// Defines the ValueColorPositions Class.
  #[sdk(child(qname = "cx:valueColorPositions"))]
  pub value_color_positions: Option<std::boxed::Box<ValueColorPositions>>,
  /// Defines the DataPoint Class.
  #[sdk(child(qname = "cx:dataPt"))]
  pub data_point: Vec<DataPoint>,
  /// Defines the DataLabels Class.
  #[sdk(child(qname = "cx:dataLabels"))]
  pub data_labels: Option<std::boxed::Box<DataLabels>>,
  /// Defines the DataId Class.
  #[sdk(child(qname = "cx:dataId"))]
  pub data_id: Option<DataId>,
  /// Defines the SeriesLayoutProperties Class.
  #[sdk(child(qname = "cx:layoutPr"))]
  pub series_layout_properties: Option<std::boxed::Box<SeriesLayoutProperties>>,
  /// Defines the AxisId Class.
  #[sdk(text_child(simple_type = "UInt32Value", qname = "cx:axisId"))]
  pub axis_id: Vec<AxisId>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PlotAreaRegion Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:plotAreaRegion")]
pub struct PlotAreaRegion {
  /// Defines the PlotSurface Class.
  #[sdk(child(qname = "cx:plotSurface"))]
  pub plot_surface: Option<std::boxed::Box<PlotSurface>>,
  /// Defines the Series Class.
  #[sdk(child(qname = "cx:series"))]
  pub series: Vec<Series>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Axis Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:axis")]
pub struct Axis {
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  #[sdk(
        choice(
            child(variant = CategoryAxisScaling, qname = "cx:catScaling"),
            child(variant = ValueAxisScaling, qname = "cx:valScaling")
        )
    )]
  pub axis_choice: Option<AxisChoice>,
  /// Defines the AxisTitle Class.
  #[sdk(child(qname = "cx:title"))]
  pub axis_title: Option<std::boxed::Box<AxisTitle>>,
  /// Defines the AxisUnits Class.
  #[sdk(child(qname = "cx:units"))]
  pub axis_units: Option<std::boxed::Box<AxisUnits>>,
  /// Defines the MajorGridlinesGridlines Class.
  #[sdk(child(qname = "cx:majorGridlines"))]
  pub major_gridlines_gridlines: Option<std::boxed::Box<MajorGridlinesGridlines>>,
  /// Defines the MinorGridlinesGridlines Class.
  #[sdk(child(qname = "cx:minorGridlines"))]
  pub minor_gridlines_gridlines: Option<std::boxed::Box<MinorGridlinesGridlines>>,
  /// Defines the MajorTickMarksTickMarks Class.
  #[sdk(child(qname = "cx:majorTickMarks"))]
  pub major_tick_marks_tick_marks: Option<std::boxed::Box<MajorTickMarksTickMarks>>,
  /// Defines the MinorTickMarksTickMarks Class.
  #[sdk(child(qname = "cx:minorTickMarks"))]
  pub minor_tick_marks_tick_marks: Option<std::boxed::Box<MinorTickMarksTickMarks>>,
  /// Defines the TickLabels Class.
  #[sdk(child(qname = "cx:tickLabels"))]
  pub tick_labels: Option<std::boxed::Box<TickLabels>>,
  /// Defines the NumberFormat Class.
  #[sdk(child(qname = "cx:numFmt"))]
  pub number_format: Option<NumberFormat>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TxPrTextBody Class.
  #[sdk(child(qname = "cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ChartTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:title")]
pub struct ChartTitle {
  /// pos
  #[sdk(attr(qname = ":pos"))]
  pub pos: Option<SidePos>,
  /// align
  #[sdk(attr(qname = ":align"))]
  pub align: Option<PosAlign>,
  /// overlay
  #[sdk(attr(qname = ":overlay"))]
  pub overlay: Option<crate::simple_type::BooleanValue>,
  /// Defines the Text Class.
  #[sdk(child(qname = "cx:tx"))]
  pub text: Option<std::boxed::Box<Text>>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TxPrTextBody Class.
  #[sdk(child(qname = "cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// Defines the Offset Class.
  #[sdk(child(qname = "cx:offset"))]
  pub offset: Option<Offset>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PlotArea Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:plotArea")]
pub struct PlotArea {
  /// Defines the PlotAreaRegion Class.
  #[sdk(child(qname = "cx:plotAreaRegion"))]
  pub plot_area_region: std::boxed::Box<PlotAreaRegion>,
  /// Defines the Axis Class.
  #[sdk(child(qname = "cx:axis"))]
  pub axis: Vec<Axis>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Legend Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:legend")]
pub struct Legend {
  /// pos
  #[sdk(attr(qname = ":pos"))]
  pub pos: Option<SidePos>,
  /// align
  #[sdk(attr(qname = ":align"))]
  pub align: Option<PosAlign>,
  /// overlay
  #[sdk(attr(qname = ":overlay"))]
  pub overlay: Option<crate::simple_type::BooleanValue>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TxPrTextBody Class.
  #[sdk(child(qname = "cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// Defines the Offset Class.
  #[sdk(child(qname = "cx:offset"))]
  pub offset: Option<Offset>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the FormatOverride Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:fmtOvr")]
pub struct FormatOverride {
  /// idx
  #[sdk(attr(qname = ":idx"))]
  pub idx: crate::simple_type::UInt32Value,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the HeaderFooter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:headerFooter")]
pub struct HeaderFooter {
  /// alignWithMargins
  #[sdk(attr(qname = ":alignWithMargins"))]
  pub align_with_margins: Option<crate::simple_type::BooleanValue>,
  /// differentOddEven
  #[sdk(attr(qname = ":differentOddEven"))]
  pub different_odd_even: Option<crate::simple_type::BooleanValue>,
  /// differentFirst
  #[sdk(attr(qname = ":differentFirst"))]
  pub different_first: Option<crate::simple_type::BooleanValue>,
  /// Defines the OddHeaderXsdstring Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "cx:oddHeader"))]
  pub odd_header_xsdstring: Option<OddHeaderXsdstring>,
  /// Defines the OddFooterXsdstring Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "cx:oddFooter"))]
  pub odd_footer_xsdstring: Option<OddFooterXsdstring>,
  /// Defines the EvenHeaderXsdstring Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "cx:evenHeader"))]
  pub even_header_xsdstring: Option<EvenHeaderXsdstring>,
  /// Defines the EvenFooterXsdstring Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "cx:evenFooter"))]
  pub even_footer_xsdstring: Option<EvenFooterXsdstring>,
  /// Defines the FirstHeaderXsdstring Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "cx:firstHeader"))]
  pub first_header_xsdstring: Option<FirstHeaderXsdstring>,
  /// Defines the FirstFooterXsdstring Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "cx:firstFooter"))]
  pub first_footer_xsdstring: Option<FirstFooterXsdstring>,
}
/// Defines the PageMargins Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:pageMargins")]
pub struct PageMargins {
  /// l
  #[sdk(attr(qname = ":l"))]
  pub l: crate::simple_type::DoubleValue,
  /// r
  #[sdk(attr(qname = ":r"))]
  pub r: crate::simple_type::DoubleValue,
  /// t
  #[sdk(attr(qname = ":t"))]
  pub t: crate::simple_type::DoubleValue,
  /// b
  #[sdk(attr(qname = ":b"))]
  pub b: crate::simple_type::DoubleValue,
  /// header
  #[sdk(attr(qname = ":header"))]
  pub header: crate::simple_type::DoubleValue,
  /// footer
  #[sdk(attr(qname = ":footer"))]
  pub footer: crate::simple_type::DoubleValue,
}
/// Defines the PageSetup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:pageSetup")]
pub struct PageSetup {
  /// paperSize
  #[sdk(attr(qname = ":paperSize"))]
  pub paper_size: Option<crate::simple_type::UInt32Value>,
  /// firstPageNumber
  #[sdk(attr(qname = ":firstPageNumber"))]
  pub first_page_number: Option<crate::simple_type::UInt32Value>,
  /// orientation
  #[sdk(attr(qname = ":orientation"))]
  pub orientation: Option<PageOrientation>,
  /// blackAndWhite
  #[sdk(attr(qname = ":blackAndWhite"))]
  pub black_and_white: Option<crate::simple_type::BooleanValue>,
  /// draft
  #[sdk(attr(qname = ":draft"))]
  pub draft: Option<crate::simple_type::BooleanValue>,
  /// useFirstPageNumber
  #[sdk(attr(qname = ":useFirstPageNumber"))]
  pub use_first_page_number: Option<crate::simple_type::BooleanValue>,
  /// horizontalDpi
  #[sdk(attr(qname = ":horizontalDpi"))]
  pub horizontal_dpi: Option<crate::simple_type::Int32Value>,
  /// verticalDpi
  #[sdk(attr(qname = ":verticalDpi"))]
  pub vertical_dpi: Option<crate::simple_type::Int32Value>,
  /// copies
  #[sdk(attr(qname = ":copies"))]
  pub copies: Option<crate::simple_type::UInt32Value>,
}
/// Defines the ChartData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:chartData")]
pub struct ChartData {
  /// Defines the ExternalData Class.
  #[sdk(child(qname = "cx:externalData"))]
  pub external_data: Option<ExternalData>,
  /// Defines the Data Class.
  #[sdk(child(qname = "cx:data"))]
  pub data: Vec<Data>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Chart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:chart")]
pub struct Chart {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the ChartTitle Class.
  #[sdk(child(qname = "cx:title"))]
  pub chart_title: Option<std::boxed::Box<ChartTitle>>,
  /// Defines the PlotArea Class.
  #[sdk(child(qname = "cx:plotArea"))]
  pub plot_area: std::boxed::Box<PlotArea>,
  /// Defines the Legend Class.
  #[sdk(child(qname = "cx:legend"))]
  pub legend: Option<std::boxed::Box<Legend>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ColorMappingType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:clrMapOvr")]
pub struct ColorMappingType {
  /// Background 1
  #[sdk(attr(qname = ":bg1"))]
  #[sdk(string_format(kind = "token"))]
  pub background1: crate::schemas::a::ColorSchemeIndexValues,
  /// Text 1
  #[sdk(attr(qname = ":tx1"))]
  #[sdk(string_format(kind = "token"))]
  pub text1: crate::schemas::a::ColorSchemeIndexValues,
  /// Background 2
  #[sdk(attr(qname = ":bg2"))]
  #[sdk(string_format(kind = "token"))]
  pub background2: crate::schemas::a::ColorSchemeIndexValues,
  /// Text 2
  #[sdk(attr(qname = ":tx2"))]
  #[sdk(string_format(kind = "token"))]
  pub text2: crate::schemas::a::ColorSchemeIndexValues,
  /// Accent 1
  #[sdk(attr(qname = ":accent1"))]
  #[sdk(string_format(kind = "token"))]
  pub accent1: crate::schemas::a::ColorSchemeIndexValues,
  /// Accent 2
  #[sdk(attr(qname = ":accent2"))]
  #[sdk(string_format(kind = "token"))]
  pub accent2: crate::schemas::a::ColorSchemeIndexValues,
  /// Accent 3
  #[sdk(attr(qname = ":accent3"))]
  #[sdk(string_format(kind = "token"))]
  pub accent3: crate::schemas::a::ColorSchemeIndexValues,
  /// Accent 4
  #[sdk(attr(qname = ":accent4"))]
  #[sdk(string_format(kind = "token"))]
  pub accent4: crate::schemas::a::ColorSchemeIndexValues,
  /// Accent 5
  #[sdk(attr(qname = ":accent5"))]
  #[sdk(string_format(kind = "token"))]
  pub accent5: crate::schemas::a::ColorSchemeIndexValues,
  /// Accent 6
  #[sdk(attr(qname = ":accent6"))]
  #[sdk(string_format(kind = "token"))]
  pub accent6: crate::schemas::a::ColorSchemeIndexValues,
  /// Hyperlink
  #[sdk(attr(qname = ":hlink"))]
  #[sdk(string_format(kind = "token"))]
  pub hyperlink: crate::schemas::a::ColorSchemeIndexValues,
  /// Followed Hyperlink
  #[sdk(attr(qname = ":folHlink"))]
  #[sdk(string_format(kind = "token"))]
  pub followed_hyperlink: crate::schemas::a::ColorSchemeIndexValues,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "a:extLst"))]
  pub extension_list: Option<crate::schemas::a::ExtensionList>,
}
/// Defines the FormatOverrides Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:fmtOvrs")]
pub struct FormatOverrides {
  /// Defines the FormatOverride Class.
  #[sdk(child(qname = "cx:fmtOvr"))]
  pub format_override: Vec<FormatOverride>,
}
/// Defines the PrintSettings Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:printSettings")]
pub struct PrintSettings {
  /// Defines the HeaderFooter Class.
  #[sdk(child(qname = "cx:headerFooter"))]
  pub header_footer: Option<HeaderFooter>,
  /// Defines the PageMargins Class.
  #[sdk(child(qname = "cx:pageMargins"))]
  pub page_margins: Option<PageMargins>,
  /// Defines the PageSetup Class.
  #[sdk(child(qname = "cx:pageSetup"))]
  pub page_setup: Option<PageSetup>,
}
/// Index of subtotal data point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:idx")]
pub struct UnsignedIntegerType {
  /// Integer Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
#[derive(Clone, Debug, PartialEq)]
pub enum MinColorSolidColorFillPropertiesChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<crate::schemas::a::HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<crate::schemas::a::SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<crate::schemas::a::SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<crate::schemas::a::PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum MidColorSolidColorFillPropertiesChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<crate::schemas::a::HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<crate::schemas::a::SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<crate::schemas::a::SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<crate::schemas::a::PresetColor>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum MaxColorSolidColorFillPropertiesChoice {
  /// RGB Color Model - Percentage Variant.
  RgbColorModelPercentage(std::boxed::Box<crate::schemas::a::RgbColorModelPercentage>),
  /// RGB Color Model - Hex Variant.
  RgbColorModelHex(std::boxed::Box<crate::schemas::a::RgbColorModelHex>),
  /// Hue, Saturation, Luminance Color Model.
  HslColor(std::boxed::Box<crate::schemas::a::HslColor>),
  /// System Color.
  SystemColor(std::boxed::Box<crate::schemas::a::SystemColor>),
  /// Scheme Color.
  SchemeColor(std::boxed::Box<crate::schemas::a::SchemeColor>),
  /// Preset Color.
  PresetColor(std::boxed::Box<crate::schemas::a::PresetColor>),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct NumericDimensionChoiceSequence {
  /// Defines the Formula Class.
  #[sdk(child(qname = "cx:f"))]
  pub formula: std::boxed::Box<Formula>,
  /// Defines the NfFormula Class.
  #[sdk(child(qname = "cx:nf"))]
  pub nf_formula: Option<NfFormula>,
  /// Defines the NumericLevel Class.
  #[sdk(child(qname = "cx:lvl"))]
  pub numeric_level: Vec<NumericLevel>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum NumericDimensionChoice {
  /// Sequence of cx:f, cx:nf, cx:lvl
  Sequence(std::boxed::Box<NumericDimensionChoiceSequence>),
  /// Defines the NumericLevel Class.
  NumericLevel(std::boxed::Box<NumericLevel>),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct StringDimensionChoiceSequence {
  /// Defines the Formula Class.
  #[sdk(child(qname = "cx:f"))]
  pub formula: std::boxed::Box<Formula>,
  /// Defines the NfFormula Class.
  #[sdk(child(qname = "cx:nf"))]
  pub nf_formula: Option<NfFormula>,
  /// Defines the StringLevel Class.
  #[sdk(child(qname = "cx:lvl"))]
  pub string_level: Vec<StringLevel>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum StringDimensionChoice {
  /// Sequence of cx:f, cx:nf, cx:lvl
  Sequence(std::boxed::Box<StringDimensionChoiceSequence>),
  /// Defines the StringLevel Class.
  StringLevel(std::boxed::Box<StringLevel>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum DataChoice {
  /// Defines the NumericDimension Class.
  NumericDimension(std::boxed::Box<NumericDimension>),
  /// Defines the StringDimension Class.
  StringDimension(std::boxed::Box<StringDimension>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TextDataChoice {
  /// Sequence of cx:f, cx:v
  Sequence {
    /// Defines the Formula Class.
    formula: std::boxed::Box<Formula>,
    /// Defines the VXsdstring Class.
    v_xsdstring: Option<VXsdstring>,
  },
  /// Defines the VXsdstring Class.
  VXsdstring(VXsdstring),
}
#[derive(Clone, Debug, PartialEq)]
pub enum TextChoice {
  /// Defines the TextData Class.
  TextData(std::boxed::Box<TextData>),
  /// Defines the RichTextBody Class.
  RichTextBody(std::boxed::Box<RichTextBody>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice {
  /// Custom geometry.
  CustomGeometry(std::boxed::Box<crate::schemas::a::CustomGeometry>),
  /// Preset geometry.
  PresetGeometry(std::boxed::Box<crate::schemas::a::PresetGeometry>),
}
#[derive(Clone, Debug, PartialEq)]
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
  GroupFill,
}
#[derive(Clone, Debug, PartialEq)]
pub enum ShapePropertiesChoice3 {
  /// Effect Container.
  EffectList(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  EffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum GeoCacheChoice {
  /// Defines the Xsdbase64Binary Class.
  Xsdbase64Binary(Xsdbase64Binary),
  /// Defines the Clear Class.
  Clear(std::boxed::Box<Clear>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum BinningChoice {
  /// Defines the Xsddouble Class.
  Xsddouble(Xsddouble),
  /// Defines the BinCountXsdunsignedInt Class.
  BinCountXsdunsignedInt(BinCountXsdunsignedInt),
}
#[derive(Clone, Debug, PartialEq)]
pub enum MinValueColorEndPositionChoice {
  /// Defines the ExtremeValueColorPosition Class.
  ExtremeValueColorPosition,
  /// Defines the NumberColorPosition Class.
  NumberColorPosition(std::boxed::Box<NumberColorPosition>),
  /// Defines the PercentageColorPosition Class.
  PercentageColorPosition(std::boxed::Box<PercentageColorPosition>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum MaxValueColorEndPositionChoice {
  /// Defines the ExtremeValueColorPosition Class.
  ExtremeValueColorPosition,
  /// Defines the NumberColorPosition Class.
  NumberColorPosition(std::boxed::Box<NumberColorPosition>),
  /// Defines the PercentageColorPosition Class.
  PercentageColorPosition(std::boxed::Box<PercentageColorPosition>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum ValueColorMiddlePositionChoice {
  /// Defines the NumberColorPosition Class.
  NumberColorPosition(std::boxed::Box<NumberColorPosition>),
  /// Defines the PercentageColorPosition Class.
  PercentageColorPosition(std::boxed::Box<PercentageColorPosition>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SeriesLayoutPropertiesChoice {
  /// Defines the Aggregation Class.
  Aggregation,
  /// Defines the Binning Class.
  Binning(std::boxed::Box<Binning>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum AxisChoice {
  /// Defines the CategoryAxisScaling Class.
  CategoryAxisScaling(std::boxed::Box<CategoryAxisScaling>),
  /// Defines the ValueAxisScaling Class.
  ValueAxisScaling(std::boxed::Box<ValueAxisScaling>),
}
