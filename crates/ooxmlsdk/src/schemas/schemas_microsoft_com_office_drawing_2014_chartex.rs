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
#[sdk(office2016, qname = "cx:CT_ChartSpace/cx:chartSpace")]
pub struct ChartSpace {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// version
  #[sdk(attr(office2016, qname = ":version"))]
  pub version: Option<crate::simple_type::StringValue>,
  /// featureList
  #[sdk(attr(office2016, qname = ":featureList"))]
  pub feature_list: Option<crate::simple_type::StringValue>,
  /// fallbackImg
  #[sdk(attr(office2016, qname = ":fallbackImg"))]
  pub fallback_img: Option<crate::simple_type::StringValue>,
  /// Defines the ChartData Class.
  #[sdk(child(office2016, qname = "cx:CT_ChartData/cx:chartData"))]
  pub chart_data: Option<std::boxed::Box<ChartData>>,
  /// Defines the Chart Class.
  #[sdk(child(office2016, qname = "cx:CT_Chart/cx:chart"))]
  pub chart: std::boxed::Box<Chart>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2016, qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TxPrTextBody Class.
  #[sdk(child(office2016, qname = "a:CT_TextBody/cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// Defines the ColorMappingType Class.
  #[sdk(child(office2016, qname = "a:CT_ColorMapping/cx:clrMapOvr"))]
  pub color_mapping_type: Option<std::boxed::Box<ColorMappingType>>,
  /// Defines the FormatOverrides Class.
  #[sdk(child(office2016, qname = "cx:CT_FormatOverrides/cx:fmtOvrs"))]
  pub format_overrides: Option<FormatOverrides>,
  /// Defines the PrintSettings Class.
  #[sdk(child(office2016, qname = "cx:CT_PrintSettings/cx:printSettings"))]
  pub print_settings: Option<std::boxed::Box<PrintSettings>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the RelId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_RelId/cx:chart")]
pub struct RelId {
  /// id
  #[sdk(attr(office2016, qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the Openxmlsdk_49BECFFA_3B03_4D13_8272_D6CCB22579E3XsdunsignedInt Class.
pub type Openxmlsdk49becffa3b034d138272D6ccb22579e3XsdunsignedInt = crate::simple_type::UInt32Value;
/// Defines the BinCountXsdunsignedInt Class.
pub type BinCountXsdunsignedInt = crate::simple_type::UInt32Value;
/// Defines the Extension2 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Extension/cx:ext")]
pub struct Extension2 {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// uri
  #[sdk(attr(office2016, qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the MinColorSolidColorFillProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_SolidColorFillProperties/cx:minColor")]
pub struct MinColorSolidColorFillProperties {
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub min_color_solid_color_fill_properties_choice: Option<MinColorSolidColorFillPropertiesChoice>,
}
/// Defines the MidColorSolidColorFillProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_SolidColorFillProperties/cx:midColor")]
pub struct MidColorSolidColorFillProperties {
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub mid_color_solid_color_fill_properties_choice: Option<MidColorSolidColorFillPropertiesChoice>,
}
/// Defines the MaxColorSolidColorFillProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_SolidColorFillProperties/cx:maxColor")]
pub struct MaxColorSolidColorFillProperties {
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub max_color_solid_color_fill_properties_choice: Option<MaxColorSolidColorFillPropertiesChoice>,
}
/// Defines the ChartStringValue Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_StringValue/cx:pt")]
pub struct ChartStringValue {
  /// idx
  #[sdk(attr(office2016, qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Formula Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Formula/cx:f")]
pub struct Formula {
  /// dir
  #[sdk(attr(office2016, qname = ":dir"))]
  pub dir: Option<FormulaDirection>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the NfFormula Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Formula/cx:nf")]
pub struct NfFormula {
  /// dir
  #[sdk(attr(office2016, qname = ":dir"))]
  pub dir: Option<FormulaDirection>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the StringLevel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_StringLevel/cx:lvl")]
pub struct StringLevel {
  /// ptCount
  #[sdk(attr(office2016, qname = ":ptCount"))]
  pub pt_count: crate::simple_type::UInt32Value,
  /// name
  #[sdk(attr(office2016, qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Defines the ChartStringValue Class.
  #[sdk(child(office2016, qname = "cx:CT_StringValue/cx:pt"))]
  pub cx_pt: Vec<ChartStringValue>,
}
/// Defines the NumericValue Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_NumericValue/cx:pt")]
pub struct NumericValue {
  /// idx
  #[sdk(attr(office2016, qname = ":idx"))]
  pub idx: crate::simple_type::UInt32Value,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::DoubleValue>,
}
/// Defines the NumericLevel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_NumericLevel/cx:lvl")]
pub struct NumericLevel {
  /// ptCount
  #[sdk(attr(office2016, qname = ":ptCount"))]
  pub pt_count: crate::simple_type::UInt32Value,
  /// formatCode
  #[sdk(attr(office2016, qname = ":formatCode"))]
  pub format_code: Option<crate::simple_type::StringValue>,
  /// name
  #[sdk(attr(office2016, qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// Defines the NumericValue Class.
  #[sdk(child(office2016, qname = "cx:CT_NumericValue/cx:pt"))]
  pub cx_pt: Vec<NumericValue>,
}
/// Defines the NumericDimension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_NumericDimension/cx:numDim")]
pub struct NumericDimension {
  /// type
  #[sdk(attr(office2016, qname = ":type"))]
  pub r#type: NumericDimensionType,
  #[sdk(choice(
    microsoft365,
    qname = "cx:CT_Formula/cx:f",
    qname = "cx:CT_Formula/cx:nf",
    qname = "cx:CT_NumericLevel/cx:lvl"
  ))]
  pub numeric_dimension_choice: Option<NumericDimensionChoice>,
}
/// Defines the StringDimension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_StringDimension/cx:strDim")]
pub struct StringDimension {
  /// type
  #[sdk(attr(office2016, qname = ":type"))]
  pub r#type: StringDimensionType,
  #[sdk(choice(
    microsoft365,
    qname = "cx:CT_Formula/cx:f",
    qname = "cx:CT_Formula/cx:nf",
    qname = "cx:CT_StringLevel/cx:lvl"
  ))]
  pub string_dimension_choice: Option<StringDimensionChoice>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_ExtensionList/cx:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the Extension2 Class.
  #[sdk(child(office2016, qname = "cx:CT_Extension/cx:ext"))]
  pub cx_ext: Vec<Extension2>,
}
/// Defines the ExternalData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_ExternalData/cx:externalData")]
pub struct ExternalData {
  /// RelId of the relationship for the external data
  #[sdk(attr(office2016, qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
  /// True if the external link should automatically update
  #[sdk(attr(office2016, qname = "cx:autoUpdate"))]
  pub cx_auto_update: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Data Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Data/cx:data")]
pub struct Data {
  /// id
  #[sdk(attr(office2016, qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  #[sdk(choice(
    microsoft365,
    qname = "cx:CT_NumericDimension/cx:numDim",
    qname = "cx:CT_StringDimension/cx:strDim"
  ))]
  pub data_choice: Vec<DataChoice>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub cx_ext_lst: Option<ExtensionList>,
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
#[sdk(office2016, qname = "cx:CT_TextData/cx:txData")]
pub struct TextData {
  #[sdk(choice(microsoft365, qname = "cx:CT_Formula/cx:f", qname = "xsd:string/cx:v"))]
  pub text_data_choice: Option<TextDataChoice>,
}
/// Defines the RichTextBody Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_TextBody/cx:rich")]
pub struct RichTextBody {
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
/// Defines the TxPrTextBody Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_TextBody/cx:txPr")]
pub struct TxPrTextBody {
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
/// Defines the Text Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Text/cx:tx")]
pub struct Text {
  #[sdk(choice(qname = "cx:CT_TextData/cx:txData", qname = "a:CT_TextBody/cx:rich"))]
  pub text_choice: Option<TextChoice>,
}
/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_ShapeProperties/cx:spPr")]
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
/// Defines the Offset Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Offset/cx:offset")]
pub struct Offset {
  /// top
  #[sdk(attr(office2016, qname = ":top"))]
  pub top: crate::simple_type::DoubleValue,
  /// left
  #[sdk(attr(office2016, qname = ":left"))]
  pub left: crate::simple_type::DoubleValue,
}
/// Defines the AxisUnitsLabel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_AxisUnitsLabel/cx:unitsLabel")]
pub struct AxisUnitsLabel {
  /// Defines the Text Class.
  #[sdk(child(office2016, qname = "cx:CT_Text/cx:tx"))]
  pub text: Option<std::boxed::Box<Text>>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2016, qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TxPrTextBody Class.
  #[sdk(child(office2016, qname = "a:CT_TextBody/cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CategoryAxisScaling Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_CategoryAxisScaling/cx:catScaling")]
pub struct CategoryAxisScaling {
  /// gapWidth
  #[sdk(attr(office2016, qname = ":gapWidth"))]
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
#[sdk(office2016, qname = "cx:CT_ValueAxisScaling/cx:valScaling")]
pub struct ValueAxisScaling {
  /// max
  #[sdk(attr(office2016, qname = ":max"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:double"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub max: Option<crate::simple_type::StringValue>,
  /// min
  #[sdk(attr(office2016, qname = ":min"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:double"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub min: Option<crate::simple_type::StringValue>,
  /// majorUnit
  #[sdk(attr(office2016, qname = ":majorUnit"))]
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
  #[sdk(attr(office2016, qname = ":minorUnit"))]
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
#[sdk(office2016, qname = "cx:CT_AxisTitle/cx:title")]
pub struct AxisTitle {
  /// Defines the Text Class.
  #[sdk(child(office2016, qname = "cx:CT_Text/cx:tx"))]
  pub text: Option<std::boxed::Box<Text>>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2016, qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TxPrTextBody Class.
  #[sdk(child(office2016, qname = "a:CT_TextBody/cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// Defines the Offset Class.
  #[sdk(child(office2016, qname = "cx:CT_Offset/cx:offset"))]
  pub offset: Option<Offset>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the AxisUnits Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_AxisUnits/cx:units")]
pub struct AxisUnits {
  /// unit
  #[sdk(attr(office2016, qname = ":unit"))]
  pub unit: Option<AxisUnit>,
  /// Defines the AxisUnitsLabel Class.
  #[sdk(child(office2016, qname = "cx:CT_AxisUnitsLabel/cx:unitsLabel"))]
  pub axis_units_label: Option<std::boxed::Box<AxisUnitsLabel>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the MajorGridlinesGridlines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Gridlines/cx:majorGridlines")]
pub struct MajorGridlinesGridlines {
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2016, qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the MinorGridlinesGridlines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Gridlines/cx:minorGridlines")]
pub struct MinorGridlinesGridlines {
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2016, qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the MajorTickMarksTickMarks Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_TickMarks/cx:majorTickMarks")]
pub struct MajorTickMarksTickMarks {
  /// type
  #[sdk(attr(office2016, qname = ":type"))]
  pub r#type: Option<TickMarksType>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the MinorTickMarksTickMarks Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_TickMarks/cx:minorTickMarks")]
pub struct MinorTickMarksTickMarks {
  /// type
  #[sdk(attr(office2016, qname = ":type"))]
  pub r#type: Option<TickMarksType>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TickLabels Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_TickLabels/cx:tickLabels")]
pub struct TickLabels {
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the NumberFormat Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_NumberFormat/cx:numFmt")]
pub struct NumberFormat {
  /// formatCode
  #[sdk(attr(office2016, qname = ":formatCode"))]
  pub format_code: crate::simple_type::StringValue,
  /// sourceLinked
  #[sdk(attr(office2016, qname = ":sourceLinked"))]
  pub source_linked: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Xsddouble Class.
pub type Xsddouble = crate::simple_type::DoubleValue;
/// Defines the Address Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Address/cx:address")]
pub struct Address {
  /// address1
  #[sdk(attr(office2016, qname = ":address1"))]
  pub address1: Option<crate::simple_type::StringValue>,
  /// countryRegion
  #[sdk(attr(office2016, qname = ":countryRegion"))]
  pub country_region: Option<crate::simple_type::StringValue>,
  /// adminDistrict1
  #[sdk(attr(office2016, qname = ":adminDistrict1"))]
  pub admin_district1: Option<crate::simple_type::StringValue>,
  /// adminDistrict2
  #[sdk(attr(office2016, qname = ":adminDistrict2"))]
  pub admin_district2: Option<crate::simple_type::StringValue>,
  /// postalCode
  #[sdk(attr(office2016, qname = ":postalCode"))]
  pub postal_code: Option<crate::simple_type::StringValue>,
  /// locality
  #[sdk(attr(office2016, qname = ":locality"))]
  pub locality: Option<crate::simple_type::StringValue>,
  /// isoCountryCode
  #[sdk(attr(office2016, qname = ":isoCountryCode"))]
  pub iso_country_code: Option<crate::simple_type::StringValue>,
}
/// Defines the GeoLocation Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_GeoLocation/cx:geoLocation")]
pub struct GeoLocation {
  /// latitude
  #[sdk(attr(office2016, qname = ":latitude"))]
  pub latitude: Option<crate::simple_type::DoubleValue>,
  /// longitude
  #[sdk(attr(office2016, qname = ":longitude"))]
  pub longitude: Option<crate::simple_type::DoubleValue>,
  /// entityName
  #[sdk(attr(office2016, qname = ":entityName"))]
  pub entity_name: crate::simple_type::StringValue,
  /// entityType
  #[sdk(attr(office2016, qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
  /// Defines the Address Class.
  #[sdk(child(office2016, qname = "cx:CT_Address/cx:address"))]
  pub address: Option<Address>,
}
/// Defines the GeoLocationQuery Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_GeoLocationQuery/cx:geoLocationQuery")]
pub struct GeoLocationQuery {
  /// countryRegion
  #[sdk(attr(office2016, qname = ":countryRegion"))]
  pub country_region: Option<crate::simple_type::StringValue>,
  /// adminDistrict1
  #[sdk(attr(office2016, qname = ":adminDistrict1"))]
  pub admin_district1: Option<crate::simple_type::StringValue>,
  /// adminDistrict2
  #[sdk(attr(office2016, qname = ":adminDistrict2"))]
  pub admin_district2: Option<crate::simple_type::StringValue>,
  /// postalCode
  #[sdk(attr(office2016, qname = ":postalCode"))]
  pub postal_code: Option<crate::simple_type::StringValue>,
  /// entityType
  #[sdk(attr(office2016, qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
}
/// Defines the GeoLocations Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_GeoLocations/cx:geoLocations")]
pub struct GeoLocations {
  /// Defines the GeoLocation Class.
  #[sdk(child(office2016, qname = "cx:CT_GeoLocation/cx:geoLocation"))]
  pub geo_location: Option<std::boxed::Box<GeoLocation>>,
}
/// Defines the GeoLocationQueryResult Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "cx:CT_GeoLocationQueryResult/cx:geoLocationQueryResult"
)]
pub struct GeoLocationQueryResult {
  /// Defines the GeoLocationQuery Class.
  #[sdk(child(office2016, qname = "cx:CT_GeoLocationQuery/cx:geoLocationQuery"))]
  pub geo_location_query: Option<GeoLocationQuery>,
  /// Defines the GeoLocations Class.
  #[sdk(child(office2016, qname = "cx:CT_GeoLocations/cx:geoLocations"))]
  pub geo_locations: Option<std::boxed::Box<GeoLocations>>,
}
/// Defines the GeoPolygon Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_GeoPolygon/cx:geoPolygon")]
pub struct GeoPolygon {
  /// polygonId
  #[sdk(attr(office2016, qname = ":polygonId"))]
  pub polygon_id: crate::simple_type::StringValue,
  /// numPoints
  #[sdk(attr(office2016, qname = ":numPoints"))]
  pub num_points: crate::simple_type::IntegerValue,
  /// pcaRings
  #[sdk(attr(office2016, qname = ":pcaRings"))]
  pub pca_rings: crate::simple_type::StringValue,
}
/// Defines the GeoPolygons Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_GeoPolygons/cx:geoPolygons")]
pub struct GeoPolygons {
  /// Defines the GeoPolygon Class.
  #[sdk(child(office2016, qname = "cx:CT_GeoPolygon/cx:geoPolygon"))]
  pub cx_geo_polygon: Vec<GeoPolygon>,
}
/// Defines the Copyrights Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Copyrights/cx:copyrights")]
pub struct Copyrights {
  /// Defines the CopyrightXsdstring Class.
  #[sdk(text_child(office2016, qname = "xsd:string/cx:copyright"))]
  pub cx_copyright: Vec<crate::simple_type::StringValue>,
}
/// Defines the GeoDataEntityQuery Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_GeoDataEntityQuery/cx:geoDataEntityQuery")]
pub struct GeoDataEntityQuery {
  /// entityType
  #[sdk(attr(office2016, qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
  /// entityId
  #[sdk(attr(office2016, qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
}
/// Defines the GeoData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_GeoData/cx:geoData")]
pub struct GeoData {
  /// entityName
  #[sdk(attr(office2016, qname = ":entityName"))]
  pub entity_name: crate::simple_type::StringValue,
  /// entityId
  #[sdk(attr(office2016, qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
  /// east
  #[sdk(attr(office2016, qname = ":east"))]
  pub east: crate::simple_type::DoubleValue,
  /// west
  #[sdk(attr(office2016, qname = ":west"))]
  pub west: crate::simple_type::DoubleValue,
  /// north
  #[sdk(attr(office2016, qname = ":north"))]
  pub north: crate::simple_type::DoubleValue,
  /// south
  #[sdk(attr(office2016, qname = ":south"))]
  pub south: crate::simple_type::DoubleValue,
  /// Defines the GeoPolygons Class.
  #[sdk(child(office2016, qname = "cx:CT_GeoPolygons/cx:geoPolygons"))]
  pub geo_polygons: Option<GeoPolygons>,
  /// Defines the Copyrights Class.
  #[sdk(child(office2016, qname = "cx:CT_Copyrights/cx:copyrights"))]
  pub copyrights: Option<Copyrights>,
}
/// Defines the GeoDataEntityQueryResult Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "cx:CT_GeoDataEntityQueryResult/cx:geoDataEntityQueryResult"
)]
pub struct GeoDataEntityQueryResult {
  /// Defines the GeoDataEntityQuery Class.
  #[sdk(child(office2016, qname = "cx:CT_GeoDataEntityQuery/cx:geoDataEntityQuery"))]
  pub geo_data_entity_query: Option<GeoDataEntityQuery>,
  /// Defines the GeoData Class.
  #[sdk(child(office2016, qname = "cx:CT_GeoData/cx:geoData"))]
  pub geo_data: Option<std::boxed::Box<GeoData>>,
}
/// Defines the GeoDataPointQuery Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_GeoDataPointQuery/cx:geoDataPointQuery")]
pub struct GeoDataPointQuery {
  /// entityType
  #[sdk(attr(office2016, qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
  /// latitude
  #[sdk(attr(office2016, qname = ":latitude"))]
  pub latitude: crate::simple_type::DoubleValue,
  /// longitude
  #[sdk(attr(office2016, qname = ":longitude"))]
  pub longitude: crate::simple_type::DoubleValue,
}
/// Defines the GeoDataPointToEntityQuery Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "cx:CT_GeoDataPointToEntityQuery/cx:geoDataPointToEntityQuery"
)]
pub struct GeoDataPointToEntityQuery {
  /// entityType
  #[sdk(attr(office2016, qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
  /// entityId
  #[sdk(attr(office2016, qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
}
/// Defines the GeoDataPointToEntityQueryResult Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "cx:CT_GeoDataPointToEntityQueryResult/cx:geoDataPointToEntityQueryResult"
)]
pub struct GeoDataPointToEntityQueryResult {
  /// Defines the GeoDataPointQuery Class.
  #[sdk(child(office2016, qname = "cx:CT_GeoDataPointQuery/cx:geoDataPointQuery"))]
  pub geo_data_point_query: Option<GeoDataPointQuery>,
  /// Defines the GeoDataPointToEntityQuery Class.
  #[sdk(child(
    office2016,
    qname = "cx:CT_GeoDataPointToEntityQuery/cx:geoDataPointToEntityQuery"
  ))]
  pub geo_data_point_to_entity_query: Option<GeoDataPointToEntityQuery>,
}
/// Defines the EntityType Class.
pub type EntityType = EntityTypeEnum;
/// Defines the GeoChildTypes Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_GeoChildTypes/cx:geoChildTypes")]
pub struct GeoChildTypes {
  /// Defines the EntityType Class.
  #[sdk(text_child(office2016, qname = "cx:ST_EntityType/cx:entityType"))]
  pub cx_entity_type: Vec<EntityTypeEnum>,
}
/// Defines the GeoHierarchyEntity Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_GeoHierarchyEntity/cx:geoHierarchyEntity")]
pub struct GeoHierarchyEntity {
  /// entityName
  #[sdk(attr(office2016, qname = ":entityName"))]
  pub entity_name: crate::simple_type::StringValue,
  /// entityId
  #[sdk(attr(office2016, qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
  /// entityType
  #[sdk(attr(office2016, qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
}
/// Defines the GeoChildEntitiesQuery Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "cx:CT_GeoChildEntitiesQuery/cx:geoChildEntitiesQuery"
)]
pub struct GeoChildEntitiesQuery {
  /// entityId
  #[sdk(attr(office2016, qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
  /// Defines the GeoChildTypes Class.
  #[sdk(child(office2016, qname = "cx:CT_GeoChildTypes/cx:geoChildTypes"))]
  pub geo_child_types: Option<GeoChildTypes>,
}
/// Defines the GeoChildEntities Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_GeoChildEntities/cx:geoChildEntities")]
pub struct GeoChildEntities {
  /// Defines the GeoHierarchyEntity Class.
  #[sdk(child(office2016, qname = "cx:CT_GeoHierarchyEntity/cx:geoHierarchyEntity"))]
  pub cx_geo_hierarchy_entity: Vec<GeoHierarchyEntity>,
}
/// Defines the GeoChildEntitiesQueryResult Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "cx:CT_GeoChildEntitiesQueryResult/cx:geoChildEntitiesQueryResult"
)]
pub struct GeoChildEntitiesQueryResult {
  /// Defines the GeoChildEntitiesQuery Class.
  #[sdk(child(
    office2016,
    qname = "cx:CT_GeoChildEntitiesQuery/cx:geoChildEntitiesQuery"
  ))]
  pub geo_child_entities_query: Option<std::boxed::Box<GeoChildEntitiesQuery>>,
  /// Defines the GeoChildEntities Class.
  #[sdk(child(office2016, qname = "cx:CT_GeoChildEntities/cx:geoChildEntities"))]
  pub geo_child_entities: Option<GeoChildEntities>,
}
/// Defines the GeoParentEntitiesQuery Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "cx:CT_GeoParentEntitiesQuery/cx:geoParentEntitiesQuery"
)]
pub struct GeoParentEntitiesQuery {
  /// entityId
  #[sdk(attr(office2016, qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
}
/// Defines the GeoEntity Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_GeoEntity/cx:geoEntity")]
pub struct GeoEntity {
  /// entityName
  #[sdk(attr(office2016, qname = ":entityName"))]
  pub entity_name: crate::simple_type::StringValue,
  /// entityType
  #[sdk(attr(office2016, qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
}
/// Defines the GeoParentEntity Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_GeoParentEntity/cx:geoParentEntity")]
pub struct GeoParentEntity {
  /// entityId
  #[sdk(attr(office2016, qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
}
/// Defines the GeoParentEntitiesQueryResult Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "cx:CT_GeoParentEntitiesQueryResult/cx:geoParentEntitiesQueryResult"
)]
pub struct GeoParentEntitiesQueryResult {
  /// Defines the GeoParentEntitiesQuery Class.
  #[sdk(child(
    office2016,
    qname = "cx:CT_GeoParentEntitiesQuery/cx:geoParentEntitiesQuery"
  ))]
  pub geo_parent_entities_query: std::boxed::Box<GeoParentEntitiesQuery>,
  /// Defines the GeoEntity Class.
  #[sdk(child(office2016, qname = "cx:CT_GeoEntity/cx:geoEntity"))]
  pub geo_entity: Option<GeoEntity>,
  /// Defines the GeoParentEntity Class.
  #[sdk(child(office2016, qname = "cx:CT_GeoParentEntity/cx:geoParentEntity"))]
  pub geo_parent_entity: Option<GeoParentEntity>,
}
/// Defines the GeoLocationQueryResults Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "cx:CT_GeoLocationQueryResults/cx:geoLocationQueryResults"
)]
pub struct GeoLocationQueryResults {
  /// Defines the GeoLocationQueryResult Class.
  #[sdk(child(
    office2016,
    qname = "cx:CT_GeoLocationQueryResult/cx:geoLocationQueryResult"
  ))]
  pub cx_geo_location_query_result: Vec<GeoLocationQueryResult>,
}
/// Defines the GeoDataEntityQueryResults Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "cx:CT_GeoDataEntityQueryResults/cx:geoDataEntityQueryResults"
)]
pub struct GeoDataEntityQueryResults {
  /// Defines the GeoDataEntityQueryResult Class.
  #[sdk(child(
    office2016,
    qname = "cx:CT_GeoDataEntityQueryResult/cx:geoDataEntityQueryResult"
  ))]
  pub cx_geo_data_entity_query_result: Vec<GeoDataEntityQueryResult>,
}
/// Defines the GeoDataPointToEntityQueryResults Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "cx:CT_GeoDataPointToEntityQueryResults/cx:geoDataPointToEntityQueryResults"
)]
pub struct GeoDataPointToEntityQueryResults {
  /// Defines the GeoDataPointToEntityQueryResult Class.
  #[sdk(child(
    office2016,
    qname = "cx:CT_GeoDataPointToEntityQueryResult/cx:geoDataPointToEntityQueryResult"
  ))]
  pub cx_geo_data_point_to_entity_query_result: Vec<GeoDataPointToEntityQueryResult>,
}
/// Defines the GeoChildEntitiesQueryResults Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "cx:CT_GeoChildEntitiesQueryResults/cx:geoChildEntitiesQueryResults"
)]
pub struct GeoChildEntitiesQueryResults {
  /// Defines the GeoChildEntitiesQueryResult Class.
  #[sdk(child(
    office2016,
    qname = "cx:CT_GeoChildEntitiesQueryResult/cx:geoChildEntitiesQueryResult"
  ))]
  pub cx_geo_child_entities_query_result: Vec<GeoChildEntitiesQueryResult>,
}
/// Defines the GeoParentEntitiesQueryResults Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "cx:CT_GeoParentEntitiesQueryResults/cx:geoParentEntitiesQueryResults"
)]
pub struct GeoParentEntitiesQueryResults {
  /// Defines the GeoParentEntitiesQueryResult Class.
  #[sdk(child(
    office2016,
    qname = "cx:CT_GeoParentEntitiesQueryResult/cx:geoParentEntitiesQueryResult"
  ))]
  pub cx_geo_parent_entities_query_result: Vec<GeoParentEntitiesQueryResult>,
}
/// Defines the Xsdbase64Binary Class.
pub type Xsdbase64Binary = crate::simple_type::Base64BinaryValue;
/// Defines the Clear Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Clear/cx:clear")]
pub struct Clear {
  /// Defines the GeoLocationQueryResults Class.
  #[sdk(child(
    office2016,
    qname = "cx:CT_GeoLocationQueryResults/cx:geoLocationQueryResults"
  ))]
  pub geo_location_query_results: Option<GeoLocationQueryResults>,
  /// Defines the GeoDataEntityQueryResults Class.
  #[sdk(child(
    office2016,
    qname = "cx:CT_GeoDataEntityQueryResults/cx:geoDataEntityQueryResults"
  ))]
  pub geo_data_entity_query_results: Option<GeoDataEntityQueryResults>,
  /// Defines the GeoDataPointToEntityQueryResults Class.
  #[sdk(child(
    office2016,
    qname = "cx:CT_GeoDataPointToEntityQueryResults/cx:geoDataPointToEntityQueryResults"
  ))]
  pub geo_data_point_to_entity_query_results: Option<GeoDataPointToEntityQueryResults>,
  /// Defines the GeoChildEntitiesQueryResults Class.
  #[sdk(child(
    office2016,
    qname = "cx:CT_GeoChildEntitiesQueryResults/cx:geoChildEntitiesQueryResults"
  ))]
  pub geo_child_entities_query_results: Option<GeoChildEntitiesQueryResults>,
  /// Defines the GeoParentEntitiesQueryResults Class.
  #[sdk(child(
    office2016,
    qname = "cx:CT_GeoParentEntitiesQueryResults/cx:geoParentEntitiesQueryResults"
  ))]
  pub geo_parent_entities_query_results: Option<GeoParentEntitiesQueryResults>,
}
/// Defines the GeoCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_GeoCache/cx:geoCache")]
pub struct GeoCache {
  /// provider
  #[sdk(attr(office2016, qname = ":provider"))]
  pub provider: crate::simple_type::StringValue,
  #[sdk(choice(qname = "xsd:base64Binary/cx:binary", qname = "cx:CT_Clear/cx:clear"))]
  pub geo_cache_choice: Vec<GeoCacheChoice>,
}
/// Defines the ParentLabelLayout Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_ParentLabelLayout/cx:parentLabelLayout")]
pub struct ParentLabelLayout {
  /// val
  #[sdk(attr(office2016, qname = ":val"))]
  pub parent_label_layout_val: ParentLabelLayoutVal,
}
/// Defines the RegionLabelLayout Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_RegionLabelLayout/cx:regionLabelLayout")]
pub struct RegionLabelLayout {
  /// val
  #[sdk(attr(office2016, qname = ":val"))]
  pub val: RegionLabelLayoutEnum,
}
/// Defines the SeriesElementVisibilities Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_SeriesElementVisibilities/cx:visibility")]
pub struct SeriesElementVisibilities {
  /// connectorLines
  #[sdk(attr(office2016, qname = ":connectorLines"))]
  pub connector_lines: Option<crate::simple_type::BooleanValue>,
  /// meanLine
  #[sdk(attr(office2016, qname = ":meanLine"))]
  pub mean_line: Option<crate::simple_type::BooleanValue>,
  /// meanMarker
  #[sdk(attr(office2016, qname = ":meanMarker"))]
  pub mean_marker: Option<crate::simple_type::BooleanValue>,
  /// nonoutliers
  #[sdk(attr(office2016, qname = ":nonoutliers"))]
  pub nonoutliers: Option<crate::simple_type::BooleanValue>,
  /// outliers
  #[sdk(attr(office2016, qname = ":outliers"))]
  pub outliers: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Binning Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Binning/cx:binning")]
pub struct Binning {
  /// intervalClosed
  #[sdk(attr(office2016, qname = ":intervalClosed"))]
  pub interval_closed: Option<IntervalClosedSide>,
  /// underflow
  #[sdk(attr(office2016, qname = ":underflow"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:double"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub underflow: Option<crate::simple_type::StringValue>,
  /// overflow
  #[sdk(attr(office2016, qname = ":overflow"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:double"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub overflow: Option<crate::simple_type::StringValue>,
  #[sdk(choice(qname = "xsd:double/cx:binSize", qname = "xsd:unsignedInt/cx:binCount"))]
  pub binning_choice: Option<BinningChoice>,
}
/// Defines the Geography Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Geography/cx:geography")]
pub struct Geography {
  /// projectionType
  #[sdk(attr(office2016, qname = ":projectionType"))]
  pub projection_type: Option<GeoProjectionType>,
  /// viewedRegionType
  #[sdk(attr(office2016, qname = ":viewedRegionType"))]
  pub viewed_region_type: Option<GeoMappingLevel>,
  /// cultureLanguage
  #[sdk(attr(office2016, qname = ":cultureLanguage"))]
  #[sdk(string_format(kind = "token"))]
  pub culture_language: crate::simple_type::StringValue,
  /// cultureRegion
  #[sdk(attr(office2016, qname = ":cultureRegion"))]
  pub culture_region: crate::simple_type::StringValue,
  /// attribution
  #[sdk(attr(office2016, qname = ":attribution"))]
  pub attribution: crate::simple_type::StringValue,
  /// Defines the GeoCache Class.
  #[sdk(child(office2016, qname = "cx:CT_GeoCache/cx:geoCache"))]
  pub geo_cache: Option<GeoCache>,
}
/// Defines the Statistics Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Statistics/cx:statistics")]
pub struct Statistics {
  /// quartileMethod
  #[sdk(attr(office2016, qname = ":quartileMethod"))]
  pub quartile_method: Option<QuartileMethod>,
}
/// Defines the Subtotals Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Subtotals/cx:subtotals")]
pub struct Subtotals {
  /// Index of subtotal data point.
  #[sdk(child(office2016, qname = "c:CT_UnsignedInt/cx:idx"))]
  pub cx_idx: Vec<UnsignedIntegerType>,
}
/// Defines the NumberColorPosition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_NumberColorPosition/cx:number")]
pub struct NumberColorPosition {
  /// val
  #[sdk(attr(office2016, qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Defines the PercentageColorPosition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_PercentageColorPosition/cx:percent")]
pub struct PercentageColorPosition {
  /// val
  #[sdk(attr(office2016, qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Defines the MinValueColorEndPosition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_ValueColorEndPosition/cx:min")]
pub struct MinValueColorEndPosition {
  #[sdk(choice(
    qname = "cx:CT_ExtremeValueColorPosition/cx:extremeValue",
    qname = "cx:CT_NumberColorPosition/cx:number",
    qname = "cx:CT_PercentageColorPosition/cx:percent"
  ))]
  pub min_value_color_end_position_choice: Option<MinValueColorEndPositionChoice>,
}
/// Defines the MaxValueColorEndPosition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_ValueColorEndPosition/cx:max")]
pub struct MaxValueColorEndPosition {
  #[sdk(choice(
    qname = "cx:CT_ExtremeValueColorPosition/cx:extremeValue",
    qname = "cx:CT_NumberColorPosition/cx:number",
    qname = "cx:CT_PercentageColorPosition/cx:percent"
  ))]
  pub max_value_color_end_position_choice: Option<MaxValueColorEndPositionChoice>,
}
/// Defines the ValueColorMiddlePosition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_ValueColorMiddlePosition/cx:mid")]
pub struct ValueColorMiddlePosition {
  #[sdk(choice(
    qname = "cx:CT_NumberColorPosition/cx:number",
    qname = "cx:CT_PercentageColorPosition/cx:percent"
  ))]
  pub value_color_middle_position_choice: Option<ValueColorMiddlePositionChoice>,
}
/// Defines the DataLabelVisibilities Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_DataLabelVisibilities/cx:visibility")]
pub struct DataLabelVisibilities {
  /// seriesName
  #[sdk(attr(office2016, qname = ":seriesName"))]
  pub series_name: Option<crate::simple_type::BooleanValue>,
  /// categoryName
  #[sdk(attr(office2016, qname = ":categoryName"))]
  pub category_name: Option<crate::simple_type::BooleanValue>,
  /// value
  #[sdk(attr(office2016, qname = ":value"))]
  pub value: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DataLabel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_DataLabel/cx:dataLabel")]
pub struct DataLabel {
  /// idx
  #[sdk(attr(office2016, qname = ":idx"))]
  pub idx: crate::simple_type::UInt32Value,
  /// pos
  #[sdk(attr(office2016, qname = ":pos"))]
  pub pos: Option<DataLabelPos>,
  /// Defines the NumberFormat Class.
  #[sdk(child(office2016, qname = "cx:CT_NumberFormat/cx:numFmt"))]
  pub number_format: Option<NumberFormat>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2016, qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TxPrTextBody Class.
  #[sdk(child(office2016, qname = "a:CT_TextBody/cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// Defines the DataLabelVisibilities Class.
  #[sdk(child(office2016, qname = "cx:CT_DataLabelVisibilities/cx:visibility"))]
  pub data_label_visibilities: Option<DataLabelVisibilities>,
  /// Defines the SeparatorXsdstring Class.
  #[sdk(text_child(office2016, qname = "xsd:string/cx:separator"))]
  pub separator_xsdstring: Option<crate::simple_type::StringValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the DataLabelHidden Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_DataLabelHidden/cx:dataLabelHidden")]
pub struct DataLabelHidden {
  /// idx
  #[sdk(attr(office2016, qname = ":idx"))]
  pub idx: crate::simple_type::UInt32Value,
}
/// Defines the ValueColors Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_ValueColors/cx:valueColors")]
pub struct ValueColors {
  /// Defines the MinColorSolidColorFillProperties Class.
  #[sdk(child(office2016, qname = "a:CT_SolidColorFillProperties/cx:minColor"))]
  pub min_color_solid_color_fill_properties:
    Option<std::boxed::Box<MinColorSolidColorFillProperties>>,
  /// Defines the MidColorSolidColorFillProperties Class.
  #[sdk(child(office2016, qname = "a:CT_SolidColorFillProperties/cx:midColor"))]
  pub mid_color_solid_color_fill_properties:
    Option<std::boxed::Box<MidColorSolidColorFillProperties>>,
  /// Defines the MaxColorSolidColorFillProperties Class.
  #[sdk(child(office2016, qname = "a:CT_SolidColorFillProperties/cx:maxColor"))]
  pub max_color_solid_color_fill_properties:
    Option<std::boxed::Box<MaxColorSolidColorFillProperties>>,
}
/// Defines the ValueColorPositions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_ValueColorPositions/cx:valueColorPositions")]
pub struct ValueColorPositions {
  /// count
  #[sdk(attr(office2016, qname = ":count"))]
  #[sdk(number_range(range = 2..= 3))]
  pub count: Option<crate::simple_type::Int32Value>,
  /// Defines the MinValueColorEndPosition Class.
  #[sdk(child(office2016, qname = "cx:CT_ValueColorEndPosition/cx:min"))]
  pub min_value_color_end_position: Option<std::boxed::Box<MinValueColorEndPosition>>,
  /// Defines the ValueColorMiddlePosition Class.
  #[sdk(child(office2016, qname = "cx:CT_ValueColorMiddlePosition/cx:mid"))]
  pub value_color_middle_position: Option<std::boxed::Box<ValueColorMiddlePosition>>,
  /// Defines the MaxValueColorEndPosition Class.
  #[sdk(child(office2016, qname = "cx:CT_ValueColorEndPosition/cx:max"))]
  pub max_value_color_end_position: Option<std::boxed::Box<MaxValueColorEndPosition>>,
}
/// Defines the DataPoint Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_DataPoint/cx:dataPt")]
pub struct DataPoint {
  /// idx
  #[sdk(attr(office2016, qname = ":idx"))]
  pub idx: crate::simple_type::UInt32Value,
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2016, qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the DataLabels Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_DataLabels/cx:dataLabels")]
pub struct DataLabels {
  /// pos
  #[sdk(attr(office2016, qname = ":pos"))]
  pub pos: Option<DataLabelPos>,
  /// Defines the NumberFormat Class.
  #[sdk(child(office2016, qname = "cx:CT_NumberFormat/cx:numFmt"))]
  pub number_format: Option<NumberFormat>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2016, qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TxPrTextBody Class.
  #[sdk(child(office2016, qname = "a:CT_TextBody/cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// Defines the DataLabelVisibilities Class.
  #[sdk(child(office2016, qname = "cx:CT_DataLabelVisibilities/cx:visibility"))]
  pub data_label_visibilities: Option<DataLabelVisibilities>,
  /// Defines the SeparatorXsdstring Class.
  #[sdk(text_child(office2016, qname = "xsd:string/cx:separator"))]
  pub separator_xsdstring: Option<crate::simple_type::StringValue>,
  /// Defines the DataLabel Class.
  #[sdk(child(office2016, qname = "cx:CT_DataLabel/cx:dataLabel"))]
  pub cx_data_label: Vec<DataLabel>,
  /// Defines the DataLabelHidden Class.
  #[sdk(child(office2016, qname = "cx:CT_DataLabelHidden/cx:dataLabelHidden"))]
  pub cx_data_label_hidden: Vec<DataLabelHidden>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the DataId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_DataId/cx:dataId")]
pub struct DataId {
  /// val
  #[sdk(attr(office2016, qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the SeriesLayoutProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_SeriesLayoutProperties/cx:layoutPr")]
pub struct SeriesLayoutProperties {
  /// Defines the ParentLabelLayout Class.
  #[sdk(child(office2016, qname = "cx:CT_ParentLabelLayout/cx:parentLabelLayout"))]
  pub parent_label_layout: Option<ParentLabelLayout>,
  /// Defines the RegionLabelLayout Class.
  #[sdk(child(office2016, qname = "cx:CT_RegionLabelLayout/cx:regionLabelLayout"))]
  pub region_label_layout: Option<RegionLabelLayout>,
  /// Defines the SeriesElementVisibilities Class.
  #[sdk(child(office2016, qname = "cx:CT_SeriesElementVisibilities/cx:visibility"))]
  pub series_element_visibilities: Option<SeriesElementVisibilities>,
  #[sdk(choice(
    microsoft365,
    qname = "cx:CT_Aggregation/cx:aggregation",
    qname = "cx:CT_Binning/cx:binning"
  ))]
  pub series_layout_properties_choice: Option<SeriesLayoutPropertiesChoice>,
  /// Defines the Geography Class.
  #[sdk(child(office2016, qname = "cx:CT_Geography/cx:geography"))]
  pub cx_geography: Option<std::boxed::Box<Geography>>,
  /// Defines the Statistics Class.
  #[sdk(child(office2016, qname = "cx:CT_Statistics/cx:statistics"))]
  pub cx_statistics: Option<Statistics>,
  /// Defines the Subtotals Class.
  #[sdk(child(office2016, qname = "cx:CT_Subtotals/cx:subtotals"))]
  pub cx_subtotals: Option<Subtotals>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the AxisId Class.
pub type AxisId = crate::simple_type::UInt32Value;
/// Defines the PlotSurface Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_PlotSurface/cx:plotSurface")]
pub struct PlotSurface {
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2016, qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Series Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Series/cx:series")]
pub struct Series {
  /// layoutId
  #[sdk(attr(office2016, qname = ":layoutId"))]
  pub layout_id: SeriesLayout,
  /// hidden
  #[sdk(attr(office2016, qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// ownerIdx
  #[sdk(attr(office2016, qname = ":ownerIdx"))]
  pub owner_idx: Option<crate::simple_type::UInt32Value>,
  /// uniqueId
  #[sdk(attr(office2016, qname = ":uniqueId"))]
  pub unique_id: Option<crate::simple_type::StringValue>,
  /// formatIdx
  #[sdk(attr(office2016, qname = ":formatIdx"))]
  pub format_idx: Option<crate::simple_type::UInt32Value>,
  /// Defines the Text Class.
  #[sdk(child(office2016, qname = "cx:CT_Text/cx:tx"))]
  pub text: Option<std::boxed::Box<Text>>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2016, qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ValueColors Class.
  #[sdk(child(office2016, qname = "cx:CT_ValueColors/cx:valueColors"))]
  pub value_colors: Option<std::boxed::Box<ValueColors>>,
  /// Defines the ValueColorPositions Class.
  #[sdk(child(office2016, qname = "cx:CT_ValueColorPositions/cx:valueColorPositions"))]
  pub value_color_positions: Option<std::boxed::Box<ValueColorPositions>>,
  /// Defines the DataPoint Class.
  #[sdk(child(office2016, qname = "cx:CT_DataPoint/cx:dataPt"))]
  pub cx_data_pt: Vec<DataPoint>,
  /// Defines the DataLabels Class.
  #[sdk(child(office2016, qname = "cx:CT_DataLabels/cx:dataLabels"))]
  pub cx_data_labels: Option<std::boxed::Box<DataLabels>>,
  /// Defines the DataId Class.
  #[sdk(child(office2016, qname = "cx:CT_DataId/cx:dataId"))]
  pub cx_data_id: Option<DataId>,
  /// Defines the SeriesLayoutProperties Class.
  #[sdk(child(office2016, qname = "cx:CT_SeriesLayoutProperties/cx:layoutPr"))]
  pub cx_layout_pr: Option<std::boxed::Box<SeriesLayoutProperties>>,
  /// Defines the AxisId Class.
  #[sdk(text_child(office2016, qname = "cx:ST_AxisId/cx:axisId"))]
  pub cx_axis_id: Vec<crate::simple_type::UInt32Value>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the PlotAreaRegion Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_PlotAreaRegion/cx:plotAreaRegion")]
pub struct PlotAreaRegion {
  /// Defines the PlotSurface Class.
  #[sdk(child(office2016, qname = "cx:CT_PlotSurface/cx:plotSurface"))]
  pub plot_surface: Option<std::boxed::Box<PlotSurface>>,
  /// Defines the Series Class.
  #[sdk(child(office2016, qname = "cx:CT_Series/cx:series"))]
  pub cx_series: Vec<Series>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the Axis Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Axis/cx:axis")]
pub struct Axis {
  /// id
  #[sdk(attr(office2016, qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// hidden
  #[sdk(attr(office2016, qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    microsoft365,
    qname = "cx:CT_CategoryAxisScaling/cx:catScaling",
    qname = "cx:CT_ValueAxisScaling/cx:valScaling"
  ))]
  pub axis_choice: Option<AxisChoice>,
  /// Defines the AxisTitle Class.
  #[sdk(child(office2016, qname = "cx:CT_AxisTitle/cx:title"))]
  pub cx_title: Option<std::boxed::Box<AxisTitle>>,
  /// Defines the AxisUnits Class.
  #[sdk(child(office2016, qname = "cx:CT_AxisUnits/cx:units"))]
  pub cx_units: Option<std::boxed::Box<AxisUnits>>,
  /// Defines the MajorGridlinesGridlines Class.
  #[sdk(child(office2016, qname = "cx:CT_Gridlines/cx:majorGridlines"))]
  pub cx_major_gridlines: Option<std::boxed::Box<MajorGridlinesGridlines>>,
  /// Defines the MinorGridlinesGridlines Class.
  #[sdk(child(office2016, qname = "cx:CT_Gridlines/cx:minorGridlines"))]
  pub cx_minor_gridlines: Option<std::boxed::Box<MinorGridlinesGridlines>>,
  /// Defines the MajorTickMarksTickMarks Class.
  #[sdk(child(office2016, qname = "cx:CT_TickMarks/cx:majorTickMarks"))]
  pub cx_major_tick_marks: Option<std::boxed::Box<MajorTickMarksTickMarks>>,
  /// Defines the MinorTickMarksTickMarks Class.
  #[sdk(child(office2016, qname = "cx:CT_TickMarks/cx:minorTickMarks"))]
  pub cx_minor_tick_marks: Option<std::boxed::Box<MinorTickMarksTickMarks>>,
  /// Defines the TickLabels Class.
  #[sdk(child(office2016, qname = "cx:CT_TickLabels/cx:tickLabels"))]
  pub cx_tick_labels: Option<std::boxed::Box<TickLabels>>,
  /// Defines the NumberFormat Class.
  #[sdk(child(office2016, qname = "cx:CT_NumberFormat/cx:numFmt"))]
  pub cx_num_fmt: Option<NumberFormat>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2016, qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub cx_sp_pr: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TxPrTextBody Class.
  #[sdk(child(office2016, qname = "a:CT_TextBody/cx:txPr"))]
  pub cx_tx_pr: Option<std::boxed::Box<TxPrTextBody>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the ChartTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_ChartTitle/cx:title")]
pub struct ChartTitle {
  /// pos
  #[sdk(attr(office2016, qname = ":pos"))]
  pub pos: Option<SidePos>,
  /// align
  #[sdk(attr(office2016, qname = ":align"))]
  pub align: Option<PosAlign>,
  /// overlay
  #[sdk(attr(office2016, qname = ":overlay"))]
  pub overlay: Option<crate::simple_type::BooleanValue>,
  /// Defines the Text Class.
  #[sdk(child(office2016, qname = "cx:CT_Text/cx:tx"))]
  pub text: Option<std::boxed::Box<Text>>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2016, qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TxPrTextBody Class.
  #[sdk(child(office2016, qname = "a:CT_TextBody/cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// Defines the Offset Class.
  #[sdk(child(office2016, qname = "cx:CT_Offset/cx:offset"))]
  pub offset: Option<Offset>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PlotArea Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_PlotArea/cx:plotArea")]
pub struct PlotArea {
  /// Defines the PlotAreaRegion Class.
  #[sdk(child(office2016, qname = "cx:CT_PlotAreaRegion/cx:plotAreaRegion"))]
  pub plot_area_region: std::boxed::Box<PlotAreaRegion>,
  /// Defines the Axis Class.
  #[sdk(child(office2016, qname = "cx:CT_Axis/cx:axis"))]
  pub cx_axis: Vec<Axis>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2016, qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub cx_sp_pr: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the Legend Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Legend/cx:legend")]
pub struct Legend {
  /// pos
  #[sdk(attr(office2016, qname = ":pos"))]
  pub pos: Option<SidePos>,
  /// align
  #[sdk(attr(office2016, qname = ":align"))]
  pub align: Option<PosAlign>,
  /// overlay
  #[sdk(attr(office2016, qname = ":overlay"))]
  pub overlay: Option<crate::simple_type::BooleanValue>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2016, qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the TxPrTextBody Class.
  #[sdk(child(office2016, qname = "a:CT_TextBody/cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// Defines the Offset Class.
  #[sdk(child(office2016, qname = "cx:CT_Offset/cx:offset"))]
  pub offset: Option<Offset>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the FormatOverride Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_FormatOverride/cx:fmtOvr")]
pub struct FormatOverride {
  /// idx
  #[sdk(attr(office2016, qname = ":idx"))]
  pub idx: crate::simple_type::UInt32Value,
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2016, qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the HeaderFooter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_HeaderFooter/cx:headerFooter")]
pub struct HeaderFooter {
  /// alignWithMargins
  #[sdk(attr(office2016, qname = ":alignWithMargins"))]
  pub align_with_margins: Option<crate::simple_type::BooleanValue>,
  /// differentOddEven
  #[sdk(attr(office2016, qname = ":differentOddEven"))]
  pub different_odd_even: Option<crate::simple_type::BooleanValue>,
  /// differentFirst
  #[sdk(attr(office2016, qname = ":differentFirst"))]
  pub different_first: Option<crate::simple_type::BooleanValue>,
  /// Defines the OddHeaderXsdstring Class.
  #[sdk(text_child(office2016, qname = "xsd:string/cx:oddHeader"))]
  pub odd_header_xsdstring: Option<crate::simple_type::StringValue>,
  /// Defines the OddFooterXsdstring Class.
  #[sdk(text_child(office2016, qname = "xsd:string/cx:oddFooter"))]
  pub odd_footer_xsdstring: Option<crate::simple_type::StringValue>,
  /// Defines the EvenHeaderXsdstring Class.
  #[sdk(text_child(office2016, qname = "xsd:string/cx:evenHeader"))]
  pub even_header_xsdstring: Option<crate::simple_type::StringValue>,
  /// Defines the EvenFooterXsdstring Class.
  #[sdk(text_child(office2016, qname = "xsd:string/cx:evenFooter"))]
  pub even_footer_xsdstring: Option<crate::simple_type::StringValue>,
  /// Defines the FirstHeaderXsdstring Class.
  #[sdk(text_child(office2016, qname = "xsd:string/cx:firstHeader"))]
  pub first_header_xsdstring: Option<crate::simple_type::StringValue>,
  /// Defines the FirstFooterXsdstring Class.
  #[sdk(text_child(office2016, qname = "xsd:string/cx:firstFooter"))]
  pub first_footer_xsdstring: Option<crate::simple_type::StringValue>,
}
/// Defines the PageMargins Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_PageMargins/cx:pageMargins")]
pub struct PageMargins {
  /// l
  #[sdk(attr(office2016, qname = ":l"))]
  pub l: crate::simple_type::DoubleValue,
  /// r
  #[sdk(attr(office2016, qname = ":r"))]
  pub r: crate::simple_type::DoubleValue,
  /// t
  #[sdk(attr(office2016, qname = ":t"))]
  pub t: crate::simple_type::DoubleValue,
  /// b
  #[sdk(attr(office2016, qname = ":b"))]
  pub b: crate::simple_type::DoubleValue,
  /// header
  #[sdk(attr(office2016, qname = ":header"))]
  pub header: crate::simple_type::DoubleValue,
  /// footer
  #[sdk(attr(office2016, qname = ":footer"))]
  pub footer: crate::simple_type::DoubleValue,
}
/// Defines the PageSetup Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_PageSetup/cx:pageSetup")]
pub struct PageSetup {
  /// paperSize
  #[sdk(attr(office2016, qname = ":paperSize"))]
  pub paper_size: Option<crate::simple_type::UInt32Value>,
  /// firstPageNumber
  #[sdk(attr(office2016, qname = ":firstPageNumber"))]
  pub first_page_number: Option<crate::simple_type::UInt32Value>,
  /// orientation
  #[sdk(attr(office2016, qname = ":orientation"))]
  pub orientation: Option<PageOrientation>,
  /// blackAndWhite
  #[sdk(attr(office2016, qname = ":blackAndWhite"))]
  pub black_and_white: Option<crate::simple_type::BooleanValue>,
  /// draft
  #[sdk(attr(office2016, qname = ":draft"))]
  pub draft: Option<crate::simple_type::BooleanValue>,
  /// useFirstPageNumber
  #[sdk(attr(office2016, qname = ":useFirstPageNumber"))]
  pub use_first_page_number: Option<crate::simple_type::BooleanValue>,
  /// horizontalDpi
  #[sdk(attr(office2016, qname = ":horizontalDpi"))]
  pub horizontal_dpi: Option<crate::simple_type::Int32Value>,
  /// verticalDpi
  #[sdk(attr(office2016, qname = ":verticalDpi"))]
  pub vertical_dpi: Option<crate::simple_type::Int32Value>,
  /// copies
  #[sdk(attr(office2016, qname = ":copies"))]
  pub copies: Option<crate::simple_type::UInt32Value>,
}
/// Defines the ChartData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_ChartData/cx:chartData")]
pub struct ChartData {
  /// Defines the ExternalData Class.
  #[sdk(child(office2016, qname = "cx:CT_ExternalData/cx:externalData"))]
  pub external_data: Option<ExternalData>,
  /// Defines the Data Class.
  #[sdk(child(office2016, qname = "cx:CT_Data/cx:data"))]
  pub cx_data: Vec<Data>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the Chart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_Chart/cx:chart")]
pub struct Chart {
  /// Defines the ChartTitle Class.
  #[sdk(child(office2016, qname = "cx:CT_ChartTitle/cx:title"))]
  pub chart_title: Option<std::boxed::Box<ChartTitle>>,
  /// Defines the PlotArea Class.
  #[sdk(child(office2016, qname = "cx:CT_PlotArea/cx:plotArea"))]
  pub plot_area: std::boxed::Box<PlotArea>,
  /// Defines the Legend Class.
  #[sdk(child(office2016, qname = "cx:CT_Legend/cx:legend"))]
  pub legend: Option<std::boxed::Box<Legend>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2016, qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ColorMappingType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_ColorMapping/cx:clrMapOvr")]
pub struct ColorMappingType {
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
/// Defines the FormatOverrides Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_FormatOverrides/cx:fmtOvrs")]
pub struct FormatOverrides {
  /// Defines the FormatOverride Class.
  #[sdk(child(office2016, qname = "cx:CT_FormatOverride/cx:fmtOvr"))]
  pub cx_fmt_ovr: Vec<FormatOverride>,
}
/// Defines the PrintSettings Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "cx:CT_PrintSettings/cx:printSettings")]
pub struct PrintSettings {
  /// Defines the HeaderFooter Class.
  #[sdk(child(office2016, qname = "cx:CT_HeaderFooter/cx:headerFooter"))]
  pub header_footer: Option<HeaderFooter>,
  /// Defines the PageMargins Class.
  #[sdk(child(office2016, qname = "cx:CT_PageMargins/cx:pageMargins"))]
  pub page_margins: Option<PageMargins>,
  /// Defines the PageSetup Class.
  #[sdk(child(office2016, qname = "cx:CT_PageSetup/cx:pageSetup"))]
  pub page_setup: Option<PageSetup>,
}
/// Index of subtotal data point.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c:CT_UnsignedInt/cx:idx")]
pub struct UnsignedIntegerType {
  /// Integer Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MinColorSolidColorFillPropertiesChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MidColorSolidColorFillPropertiesChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MaxColorSolidColorFillPropertiesChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct NumericDimensionChoiceSequence {
  /// Defines the Formula Class.
  #[sdk(child(office2016, qname = "cx:CT_Formula/cx:f"))]
  pub formula: std::boxed::Box<Formula>,
  /// Defines the NfFormula Class.
  #[sdk(child(office2016, qname = "cx:CT_Formula/cx:nf"))]
  pub nf_formula: Option<NfFormula>,
  /// Defines the NumericLevel Class.
  #[sdk(child(office2016, qname = "cx:CT_NumericLevel/cx:lvl"))]
  pub numeric_level: Vec<NumericLevel>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum NumericDimensionChoice {
  /// Sequence of cx:f, cx:nf, cx:lvl
  #[sdk(sequence)]
  Sequence(std::boxed::Box<NumericDimensionChoiceSequence>),
  #[sdk(child(office2016, qname = "cx:CT_NumericLevel/cx:lvl"))]
  CxLvl(std::boxed::Box<NumericLevel>),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct StringDimensionChoiceSequence {
  /// Defines the Formula Class.
  #[sdk(child(office2016, qname = "cx:CT_Formula/cx:f"))]
  pub formula: std::boxed::Box<Formula>,
  /// Defines the NfFormula Class.
  #[sdk(child(office2016, qname = "cx:CT_Formula/cx:nf"))]
  pub nf_formula: Option<NfFormula>,
  /// Defines the StringLevel Class.
  #[sdk(child(office2016, qname = "cx:CT_StringLevel/cx:lvl"))]
  pub string_level: Vec<StringLevel>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum StringDimensionChoice {
  /// Sequence of cx:f, cx:nf, cx:lvl
  #[sdk(sequence)]
  Sequence(std::boxed::Box<StringDimensionChoiceSequence>),
  #[sdk(child(office2016, qname = "cx:CT_StringLevel/cx:lvl"))]
  CxLvl(std::boxed::Box<StringLevel>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DataChoice {
  #[sdk(child(office2016, qname = "cx:CT_NumericDimension/cx:numDim"))]
  CxNumDim(std::boxed::Box<NumericDimension>),
  #[sdk(child(office2016, qname = "cx:CT_StringDimension/cx:strDim"))]
  CxStrDim(std::boxed::Box<StringDimension>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextDataChoice {
  /// Sequence of cx:f, cx:v
  #[sdk(sequence)]
  Sequence {
    /// Defines the Formula Class.
    #[sdk(child(office2016, qname = "cx:CT_Formula/cx:f"))]
    formula: std::boxed::Box<Formula>,
    /// Defines the VXsdstring Class.
    #[sdk(text_child(office2016, qname = "xsd:string/cx:v"))]
    v_xsdstring: Option<crate::simple_type::StringValue>,
  },
  #[sdk(text_child(office2016, qname = "xsd:string/cx:v"))]
  CxV(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TextChoice {
  /// Defines the TextData Class.
  #[sdk(child(office2016, qname = "cx:CT_TextData/cx:txData"))]
  CxTxData(std::boxed::Box<TextData>),
  /// Defines the RichTextBody Class.
  #[sdk(child(office2016, qname = "a:CT_TextBody/cx:rich"))]
  CxRich(std::boxed::Box<RichTextBody>),
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum GeoCacheChoice {
  /// Defines the Xsdbase64Binary Class.
  #[sdk(text_child(office2016, qname = "xsd:base64Binary/cx:binary"))]
  CxBinary(crate::simple_type::Base64BinaryValue),
  /// Defines the Clear Class.
  #[sdk(child(office2016, qname = "cx:CT_Clear/cx:clear"))]
  CxClear(std::boxed::Box<Clear>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum BinningChoice {
  /// Defines the Xsddouble Class.
  #[sdk(text_child(office2016, qname = "xsd:double/cx:binSize"))]
  CxBinSize(crate::simple_type::DoubleValue),
  /// Defines the BinCountXsdunsignedInt Class.
  #[sdk(text_child(office2016, qname = "xsd:unsignedInt/cx:binCount"))]
  CxBinCount(crate::simple_type::UInt32Value),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MinValueColorEndPositionChoice {
  /// Defines the ExtremeValueColorPosition Class.
  #[sdk(empty_child(office2016, qname = "cx:CT_ExtremeValueColorPosition/cx:extremeValue"))]
  CxExtremeValue,
  /// Defines the NumberColorPosition Class.
  #[sdk(child(office2016, qname = "cx:CT_NumberColorPosition/cx:number"))]
  CxNumber(std::boxed::Box<NumberColorPosition>),
  /// Defines the PercentageColorPosition Class.
  #[sdk(child(office2016, qname = "cx:CT_PercentageColorPosition/cx:percent"))]
  CxPercent(std::boxed::Box<PercentageColorPosition>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum MaxValueColorEndPositionChoice {
  /// Defines the ExtremeValueColorPosition Class.
  #[sdk(empty_child(office2016, qname = "cx:CT_ExtremeValueColorPosition/cx:extremeValue"))]
  CxExtremeValue,
  /// Defines the NumberColorPosition Class.
  #[sdk(child(office2016, qname = "cx:CT_NumberColorPosition/cx:number"))]
  CxNumber(std::boxed::Box<NumberColorPosition>),
  /// Defines the PercentageColorPosition Class.
  #[sdk(child(office2016, qname = "cx:CT_PercentageColorPosition/cx:percent"))]
  CxPercent(std::boxed::Box<PercentageColorPosition>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ValueColorMiddlePositionChoice {
  /// Defines the NumberColorPosition Class.
  #[sdk(child(office2016, qname = "cx:CT_NumberColorPosition/cx:number"))]
  CxNumber(std::boxed::Box<NumberColorPosition>),
  /// Defines the PercentageColorPosition Class.
  #[sdk(child(office2016, qname = "cx:CT_PercentageColorPosition/cx:percent"))]
  CxPercent(std::boxed::Box<PercentageColorPosition>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SeriesLayoutPropertiesChoice {
  /// Defines the Aggregation Class.
  #[sdk(empty_child(office2016, qname = "cx:CT_Aggregation/cx:aggregation"))]
  CxAggregation,
  #[sdk(child(office2016, qname = "cx:CT_Binning/cx:binning"))]
  CxBinning(std::boxed::Box<Binning>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AxisChoice {
  #[sdk(child(office2016, qname = "cx:CT_CategoryAxisScaling/cx:catScaling"))]
  CxCatScaling(std::boxed::Box<CategoryAxisScaling>),
  #[sdk(child(office2016, qname = "cx:CT_ValueAxisScaling/cx:valScaling"))]
  CxValScaling(std::boxed::Box<ValueAxisScaling>),
}
