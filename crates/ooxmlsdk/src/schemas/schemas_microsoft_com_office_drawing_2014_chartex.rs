//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum FormulaDirection {
  #[sdk(rename = "col")]
  #[default]
  Col,
  #[sdk(rename = "row")]
  Row,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum StringDimensionType {
  #[sdk(rename = "cat")]
  #[default]
  Cat,
  #[sdk(rename = "colorStr")]
  ColorStr,
  #[sdk(rename = "entityId")]
  EntityId,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum PosAlign {
  #[sdk(rename = "min")]
  #[default]
  Min,
  #[sdk(rename = "ctr")]
  Ctr,
  #[sdk(rename = "max")]
  Max,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum ParentLabelLayoutVal {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "banner")]
  Banner,
  #[sdk(rename = "overlapping")]
  Overlapping,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum RegionLabelLayoutEnum {
  #[sdk(rename = "none")]
  #[default]
  None,
  #[sdk(rename = "bestFitOnly")]
  BestFitOnly,
  #[sdk(rename = "showAll")]
  ShowAll,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum IntervalClosedSide {
  #[sdk(rename = "l")]
  #[default]
  L,
  #[sdk(rename = "r")]
  R,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum QuartileMethod {
  #[sdk(rename = "inclusive")]
  #[default]
  Inclusive,
  #[sdk(rename = "exclusive")]
  Exclusive,
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:chartSpace.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_ChartSpace/cx:chartSpace")]
pub struct ChartSpace {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// version
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :version
  #[sdk(attr(qname = ":version"))]
  pub version: Option<crate::simple_type::StringValue>,
  /// featureList
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :featureList
  #[sdk(attr(qname = ":featureList"))]
  pub feature_list: Option<crate::simple_type::StringValue>,
  /// fallbackImg
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :fallbackImg
  #[sdk(attr(qname = ":fallbackImg"))]
  pub fallback_img: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "cx:CT_ChartData/cx:chartData"))]
  pub chart_data: Option<std::boxed::Box<ChartData>>,
  /// _
  #[sdk(child(qname = "cx:CT_ChartData/cx:chartDataIntentionallyChanged"))]
  pub chart_data_intentionally_changed: Option<std::boxed::Box<ChartDataIntentionallyChanged>>,
  /// _
  #[sdk(child(qname = "cx:CT_Chart/cx:chart"))]
  pub chart: std::boxed::Box<Chart>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBody/cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// _
  #[sdk(child(qname = "a:CT_ColorMapping/cx:clrMapOvr"))]
  pub color_mapping_type: Option<std::boxed::Box<ColorMappingType>>,
  /// _
  #[sdk(child(qname = "cx:CT_FormatOverrides/cx:fmtOvrs"))]
  pub format_overrides: Option<FormatOverrides>,
  /// _
  #[sdk(child(qname = "cx:CT_PrintSettings/cx:printSettings"))]
  pub print_settings: Option<std::boxed::Box<PrintSettings>>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the RelId Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:chart.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_RelId/cx:chart")]
pub struct RelId {
  /// id
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the Openxmlsdk_49BECFFA_3B03_4D13_8272_D6CCB22579E3XsdunsignedInt Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:openxmlsdk_49BECFFA_3B03_4D13_8272_D6CCB22579E3.
pub type Openxmlsdk49becffa3b034d138272D6ccb22579e3XsdunsignedInt = crate::simple_type::UInt32Value;
/// Defines the BinCountXsdunsignedInt Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:binCount.
pub type BinCountXsdunsignedInt = crate::simple_type::UInt32Value;
/// Defines the Extension2 Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:ext.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Extension/cx:ext")]
pub struct Extension2 {
  /// uri
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub uri: Option<crate::simple_type::StringValue>,
  #[sdk(choice)]
  pub xml_children: Vec<Extension2Choice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum Extension2Choice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the MinColorSolidColorFillProperties Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:minColor.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_SolidColorFillProperties/cx:minColor")]
pub struct MinColorSolidColorFillProperties {
  #[sdk(choice)]
  pub xml_children: Option<MinColorSolidColorFillPropertiesChoice>,
}
/// Defines the MidColorSolidColorFillProperties Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:midColor.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_SolidColorFillProperties/cx:midColor")]
pub struct MidColorSolidColorFillProperties {
  #[sdk(choice)]
  pub xml_children: Option<MidColorSolidColorFillPropertiesChoice>,
}
/// Defines the MaxColorSolidColorFillProperties Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:maxColor.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_SolidColorFillProperties/cx:maxColor")]
pub struct MaxColorSolidColorFillProperties {
  #[sdk(choice)]
  pub xml_children: Option<MaxColorSolidColorFillPropertiesChoice>,
}
/// Defines the OpenXmlSolidColorFillPropertiesElement Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_SolidColorFillProperties/")]
pub struct OpenXmlSolidColorFillPropertiesElement {
  #[sdk(choice)]
  pub xml_children: Option<OpenXmlSolidColorFillPropertiesElementChoice>,
}
/// Defines the ChartStringValue Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:pt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_StringValue/cx:pt")]
pub struct ChartStringValue {
  /// idx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub index: crate::simple_type::UInt32Value,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Formula Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:f.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Formula/cx:f")]
pub struct Formula {
  /// dir
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  pub dir: Option<FormulaDirection>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the NfFormula Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:nf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Formula/cx:nf")]
pub struct NfFormula {
  /// dir
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  pub dir: Option<FormulaDirection>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the OpenXmlFormulaElement Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Formula/")]
pub struct OpenXmlFormulaElement {
  /// dir
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :dir
  #[sdk(attr(qname = ":dir"))]
  pub dir: Option<FormulaDirection>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the StringLevel Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:lvl.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_StringLevel/cx:lvl")]
pub struct StringLevel {
  /// ptCount
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ptCount
  #[sdk(attr(qname = ":ptCount"))]
  pub pt_count: crate::simple_type::UInt32Value,
  /// name
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "cx:CT_StringValue/cx:pt"))]
  pub cx_pt: Vec<ChartStringValue>,
}
/// Defines the NumericValue Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:pt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_NumericValue/cx:pt")]
pub struct NumericValue {
  /// idx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub idx: crate::simple_type::UInt32Value,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::DoubleValue>,
}
/// Defines the NumericLevel Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:lvl.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_NumericLevel/cx:lvl")]
pub struct NumericLevel {
  /// ptCount
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ptCount
  #[sdk(attr(qname = ":ptCount"))]
  pub pt_count: crate::simple_type::UInt32Value,
  /// formatCode
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :formatCode
  #[sdk(attr(qname = ":formatCode"))]
  pub format_code: Option<crate::simple_type::StringValue>,
  /// name
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "cx:CT_NumericValue/cx:pt"))]
  pub cx_pt: Vec<NumericValue>,
}
/// Defines the NumericDimension Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:numDim.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_NumericDimension/cx:numDim")]
pub struct NumericDimension {
  /// type
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: NumericDimensionType,
  #[sdk(choice)]
  pub xml_children: Option<NumericDimensionChoice>,
}
/// Defines the StringDimension Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:strDim.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_StringDimension/cx:strDim")]
pub struct StringDimension {
  /// type
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: StringDimensionType,
  #[sdk(choice)]
  pub xml_children: Option<StringDimensionChoice>,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_ExtensionList/cx:extLst")]
pub struct ExtensionList {
  /// _
  #[sdk(child(qname = "cx:CT_Extension/cx:ext"))]
  pub cx_ext: Vec<Extension2>,
}
/// Defines the ExternalData Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:externalData.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_ExternalData/cx:externalData")]
pub struct ExternalData {
  /// RelId of the relationship for the external data
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
  /// True if the external link should automatically update
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: cx:autoUpdate
  #[sdk(attr(qname = "cx:autoUpdate"))]
  pub cx_auto_update: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Data Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:data.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Data/cx:data")]
pub struct Data {
  /// id
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  #[sdk(choice)]
  pub data_choice: Vec<DataChoice>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the VXsdstring Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:v.
pub type VXsdstring = crate::simple_type::StringValue;
/// Defines the CopyrightXsdstring Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:copyright.
pub type CopyrightXsdstring = crate::simple_type::StringValue;
/// Defines the SeparatorXsdstring Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:separator.
pub type SeparatorXsdstring = crate::simple_type::StringValue;
/// Defines the OddHeaderXsdstring Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:oddHeader.
pub type OddHeaderXsdstring = crate::simple_type::StringValue;
/// Defines the OddFooterXsdstring Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:oddFooter.
pub type OddFooterXsdstring = crate::simple_type::StringValue;
/// Defines the EvenHeaderXsdstring Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:evenHeader.
pub type EvenHeaderXsdstring = crate::simple_type::StringValue;
/// Defines the EvenFooterXsdstring Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:evenFooter.
pub type EvenFooterXsdstring = crate::simple_type::StringValue;
/// Defines the FirstHeaderXsdstring Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:firstHeader.
pub type FirstHeaderXsdstring = crate::simple_type::StringValue;
/// Defines the FirstFooterXsdstring Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:firstFooter.
pub type FirstFooterXsdstring = crate::simple_type::StringValue;
/// Defines the TextData Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:txData.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_TextData/cx:txData")]
pub struct TextData {
  #[sdk(choice)]
  pub xml_children: Option<TextDataChoice>,
}
/// Defines the RichTextBody Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:rich.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBody/cx:rich")]
pub struct RichTextBody {
  ///Body Properties
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  pub body_properties:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BodyProperties>,
  ///Text List Styles
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  pub list_style: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ListStyle>,
  >,
  /// _
  #[sdk(child(qname = "a:CT_TextParagraph/a:p"))]
  pub a_p: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Paragraph>,
}
/// Defines the TxPrTextBody Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:txPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBody/cx:txPr")]
pub struct TxPrTextBody {
  ///Body Properties
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  pub body_properties:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BodyProperties>,
  ///Text List Styles
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_TextBody/")]
pub struct TextBodyType {
  #[sdk(choice)]
  pub xml_children: Vec<TextBodyTypeChoice>,
}
/// Defines the Text Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:tx.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Text/cx:tx")]
pub struct Text {
  #[sdk(choice)]
  pub xml_children: Option<TextChoice>,
}
/// Defines the ShapeProperties Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:spPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ShapeProperties/cx:spPr")]
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
  #[sdk(choice)]
  pub shape_properties_choice1: Option<ShapePropertiesChoice>,
  #[sdk(choice)]
  pub shape_properties_choice2: Option<ShapePropertiesChoice2>,
  /// _
  #[sdk(child(qname = "a:CT_LineProperties/a:ln"))]
  pub a_ln: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline>,
  >,
  #[sdk(choice)]
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
/// Defines the Offset Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:offset.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Offset/cx:offset")]
pub struct Offset {
  /// top
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :top
  #[sdk(attr(qname = ":top"))]
  pub top: crate::simple_type::DoubleValue,
  /// left
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :left
  #[sdk(attr(qname = ":left"))]
  pub left: crate::simple_type::DoubleValue,
}
/// Defines the AxisUnitsLabel Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:unitsLabel.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_AxisUnitsLabel/cx:unitsLabel")]
pub struct AxisUnitsLabel {
  /// _
  #[sdk(child(qname = "cx:CT_Text/cx:tx"))]
  pub text: Option<std::boxed::Box<Text>>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBody/cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the CategoryAxisScaling Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:catScaling.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_CategoryAxisScaling/cx:catScaling")]
pub struct CategoryAxisScaling {
  /// gapWidth
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :gapWidth
  #[sdk(attr(qname = ":gapWidth"))]
  #[sdk(number_range(
    source = 0u32,
    union = 0u64,
    min = "0",
    min_inclusive = true,
    max_inclusive = false
  ))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub gap_width: Option<crate::simple_type::StringValue>,
}
/// Defines the ValueAxisScaling Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:valScaling.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_ValueAxisScaling/cx:valScaling")]
pub struct ValueAxisScaling {
  /// max
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :max
  #[sdk(attr(qname = ":max"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:double"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub max: Option<crate::simple_type::StringValue>,
  /// min
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :min
  #[sdk(attr(qname = ":min"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:double"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub min: Option<crate::simple_type::StringValue>,
  /// majorUnit
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :majorUnit
  #[sdk(attr(qname = ":majorUnit"))]
  #[sdk(number_range(
    source = 0u32,
    union = 0u64,
    min = "0",
    min_inclusive = false,
    max_inclusive = false
  ))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub major_unit: Option<crate::simple_type::StringValue>,
  /// minorUnit
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :minorUnit
  #[sdk(attr(qname = ":minorUnit"))]
  #[sdk(number_range(
    source = 0u32,
    union = 0u64,
    min = "0",
    min_inclusive = false,
    max_inclusive = false
  ))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub minor_unit: Option<crate::simple_type::StringValue>,
}
/// Defines the AxisTitle Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:title.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_AxisTitle/cx:title")]
pub struct AxisTitle {
  /// _
  #[sdk(child(qname = "cx:CT_Text/cx:tx"))]
  pub text: Option<std::boxed::Box<Text>>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBody/cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// _
  #[sdk(child(qname = "cx:CT_Offset/cx:offset"))]
  pub offset: Option<Offset>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the AxisUnits Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:units.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_AxisUnits/cx:units")]
pub struct AxisUnits {
  /// unit
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :unit
  #[sdk(attr(qname = ":unit"))]
  pub unit: Option<AxisUnit>,
  /// _
  #[sdk(child(qname = "cx:CT_AxisUnitsLabel/cx:unitsLabel"))]
  pub axis_units_label: Option<std::boxed::Box<AxisUnitsLabel>>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the MajorGridlinesGridlines Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:majorGridlines.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Gridlines/cx:majorGridlines")]
pub struct MajorGridlinesGridlines {
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the MinorGridlinesGridlines Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:minorGridlines.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Gridlines/cx:minorGridlines")]
pub struct MinorGridlinesGridlines {
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the OpenXmlGridlinesElement Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Gridlines/")]
pub struct OpenXmlGridlinesElement {
  #[sdk(choice)]
  pub xml_children: Vec<OpenXmlGridlinesElementChoice>,
}
/// Defines the MajorTickMarksTickMarks Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:majorTickMarks.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_TickMarks/cx:majorTickMarks")]
pub struct MajorTickMarksTickMarks {
  /// type
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<TickMarksType>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the MinorTickMarksTickMarks Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:minorTickMarks.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_TickMarks/cx:minorTickMarks")]
pub struct MinorTickMarksTickMarks {
  /// type
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<TickMarksType>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the OpenXmlTickMarksElement Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_TickMarks/")]
pub struct OpenXmlTickMarksElement {
  /// type
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<TickMarksType>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Vec<ExtensionList>,
}
/// Defines the TickLabels Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:tickLabels.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_TickLabels/cx:tickLabels")]
pub struct TickLabels {
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the NumberFormat Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:numFmt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_NumberFormat/cx:numFmt")]
pub struct NumberFormat {
  /// formatCode
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :formatCode
  #[sdk(attr(qname = ":formatCode"))]
  pub format_code: crate::simple_type::StringValue,
  /// sourceLinked
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sourceLinked
  #[sdk(attr(qname = ":sourceLinked"))]
  pub source_linked: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Xsddouble Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:binSize.
pub type Xsddouble = crate::simple_type::DoubleValue;
/// Defines the Address Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:address.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Address/cx:address")]
pub struct Address {
  /// address1
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :address1
  #[sdk(attr(qname = ":address1"))]
  pub address1: Option<crate::simple_type::StringValue>,
  /// countryRegion
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :countryRegion
  #[sdk(attr(qname = ":countryRegion"))]
  pub country_region: Option<crate::simple_type::StringValue>,
  /// adminDistrict1
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :adminDistrict1
  #[sdk(attr(qname = ":adminDistrict1"))]
  pub admin_district1: Option<crate::simple_type::StringValue>,
  /// adminDistrict2
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :adminDistrict2
  #[sdk(attr(qname = ":adminDistrict2"))]
  pub admin_district2: Option<crate::simple_type::StringValue>,
  /// postalCode
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :postalCode
  #[sdk(attr(qname = ":postalCode"))]
  pub postal_code: Option<crate::simple_type::StringValue>,
  /// locality
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :locality
  #[sdk(attr(qname = ":locality"))]
  pub locality: Option<crate::simple_type::StringValue>,
  /// isoCountryCode
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :isoCountryCode
  #[sdk(attr(qname = ":isoCountryCode"))]
  pub iso_country_code: Option<crate::simple_type::StringValue>,
}
/// Defines the GeoLocation Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoLocation.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoLocation/cx:geoLocation")]
pub struct GeoLocation {
  /// latitude
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :latitude
  #[sdk(attr(qname = ":latitude"))]
  pub latitude: Option<crate::simple_type::DoubleValue>,
  /// longitude
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :longitude
  #[sdk(attr(qname = ":longitude"))]
  pub longitude: Option<crate::simple_type::DoubleValue>,
  /// entityName
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityName
  #[sdk(attr(qname = ":entityName"))]
  pub entity_name: crate::simple_type::StringValue,
  /// entityType
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityType
  #[sdk(attr(qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
  /// _
  #[sdk(child(qname = "cx:CT_Address/cx:address"))]
  pub address: Option<Address>,
}
/// Defines the GeoLocationQuery Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoLocationQuery.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoLocationQuery/cx:geoLocationQuery")]
pub struct GeoLocationQuery {
  /// countryRegion
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :countryRegion
  #[sdk(attr(qname = ":countryRegion"))]
  pub country_region: Option<crate::simple_type::StringValue>,
  /// adminDistrict1
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :adminDistrict1
  #[sdk(attr(qname = ":adminDistrict1"))]
  pub admin_district1: Option<crate::simple_type::StringValue>,
  /// adminDistrict2
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :adminDistrict2
  #[sdk(attr(qname = ":adminDistrict2"))]
  pub admin_district2: Option<crate::simple_type::StringValue>,
  /// postalCode
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :postalCode
  #[sdk(attr(qname = ":postalCode"))]
  pub postal_code: Option<crate::simple_type::StringValue>,
  /// entityType
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityType
  #[sdk(attr(qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
}
/// Defines the GeoLocations Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoLocations.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoLocations/cx:geoLocations")]
pub struct GeoLocations {
  /// _
  #[sdk(child(qname = "cx:CT_GeoLocation/cx:geoLocation"))]
  pub geo_location: Option<std::boxed::Box<GeoLocation>>,
}
/// Defines the GeoLocationQueryResult Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoLocationQueryResult.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoLocationQueryResult/cx:geoLocationQueryResult")]
pub struct GeoLocationQueryResult {
  /// _
  #[sdk(child(qname = "cx:CT_GeoLocationQuery/cx:geoLocationQuery"))]
  pub geo_location_query: Option<GeoLocationQuery>,
  /// _
  #[sdk(child(qname = "cx:CT_GeoLocations/cx:geoLocations"))]
  pub geo_locations: Option<std::boxed::Box<GeoLocations>>,
}
/// Defines the GeoPolygon Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoPolygon.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoPolygon/cx:geoPolygon")]
pub struct GeoPolygon {
  /// polygonId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :polygonId
  #[sdk(attr(qname = ":polygonId"))]
  pub polygon_id: crate::simple_type::StringValue,
  /// numPoints
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :numPoints
  #[sdk(attr(qname = ":numPoints"))]
  pub num_points: crate::simple_type::IntegerValue,
  /// pcaRings
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :pcaRings
  #[sdk(attr(qname = ":pcaRings"))]
  pub pca_rings: crate::simple_type::StringValue,
}
/// Defines the GeoPolygons Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoPolygons.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoPolygons/cx:geoPolygons")]
pub struct GeoPolygons {
  /// _
  #[sdk(child(qname = "cx:CT_GeoPolygon/cx:geoPolygon"))]
  pub cx_geo_polygon: Vec<GeoPolygon>,
}
/// Defines the Copyrights Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:copyrights.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Copyrights/cx:copyrights")]
pub struct Copyrights {
  /// _
  #[sdk(text_child(qname = "xsd:string/cx:copyright"))]
  pub cx_copyright: Vec<crate::simple_type::StringValue>,
}
/// Defines the GeoDataEntityQuery Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoDataEntityQuery.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoDataEntityQuery/cx:geoDataEntityQuery")]
pub struct GeoDataEntityQuery {
  /// entityType
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityType
  #[sdk(attr(qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
  /// entityId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityId
  #[sdk(attr(qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
}
/// Defines the GeoData Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoData.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoData/cx:geoData")]
pub struct GeoData {
  /// entityName
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityName
  #[sdk(attr(qname = ":entityName"))]
  pub entity_name: crate::simple_type::StringValue,
  /// entityId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityId
  #[sdk(attr(qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
  /// east
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :east
  #[sdk(attr(qname = ":east"))]
  pub east: crate::simple_type::DoubleValue,
  /// west
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :west
  #[sdk(attr(qname = ":west"))]
  pub west: crate::simple_type::DoubleValue,
  /// north
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :north
  #[sdk(attr(qname = ":north"))]
  pub north: crate::simple_type::DoubleValue,
  /// south
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :south
  #[sdk(attr(qname = ":south"))]
  pub south: crate::simple_type::DoubleValue,
  /// _
  #[sdk(child(qname = "cx:CT_GeoPolygons/cx:geoPolygons"))]
  pub geo_polygons: Option<GeoPolygons>,
  /// _
  #[sdk(child(qname = "cx:CT_Copyrights/cx:copyrights"))]
  pub copyrights: Option<Copyrights>,
}
/// Defines the GeoDataEntityQueryResult Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoDataEntityQueryResult.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoDataEntityQueryResult/cx:geoDataEntityQueryResult")]
pub struct GeoDataEntityQueryResult {
  /// _
  #[sdk(child(qname = "cx:CT_GeoDataEntityQuery/cx:geoDataEntityQuery"))]
  pub geo_data_entity_query: Option<GeoDataEntityQuery>,
  /// _
  #[sdk(child(qname = "cx:CT_GeoData/cx:geoData"))]
  pub geo_data: Option<std::boxed::Box<GeoData>>,
}
/// Defines the GeoDataPointQuery Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoDataPointQuery.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoDataPointQuery/cx:geoDataPointQuery")]
pub struct GeoDataPointQuery {
  /// entityType
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityType
  #[sdk(attr(qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
  /// latitude
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :latitude
  #[sdk(attr(qname = ":latitude"))]
  pub latitude: crate::simple_type::DoubleValue,
  /// longitude
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :longitude
  #[sdk(attr(qname = ":longitude"))]
  pub longitude: crate::simple_type::DoubleValue,
}
/// Defines the GeoDataPointToEntityQuery Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoDataPointToEntityQuery.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoDataPointToEntityQuery/cx:geoDataPointToEntityQuery")]
pub struct GeoDataPointToEntityQuery {
  /// entityType
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityType
  #[sdk(attr(qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
  /// entityId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityId
  #[sdk(attr(qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
}
/// Defines the GeoDataPointToEntityQueryResult Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoDataPointToEntityQueryResult.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoDataPointToEntityQueryResult/cx:geoDataPointToEntityQueryResult")]
pub struct GeoDataPointToEntityQueryResult {
  /// _
  #[sdk(child(qname = "cx:CT_GeoDataPointQuery/cx:geoDataPointQuery"))]
  pub geo_data_point_query: Option<GeoDataPointQuery>,
  /// _
  #[sdk(child(qname = "cx:CT_GeoDataPointToEntityQuery/cx:geoDataPointToEntityQuery"))]
  pub geo_data_point_to_entity_query: Option<GeoDataPointToEntityQuery>,
}
/// Defines the EntityType Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:entityType.
pub type EntityType = EntityTypeEnum;
/// Defines the GeoChildTypes Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoChildTypes.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoChildTypes/cx:geoChildTypes")]
pub struct GeoChildTypes {
  /// _
  #[sdk(text_child(qname = "cx:ST_EntityType/cx:entityType"))]
  pub cx_entity_type: Vec<EntityTypeEnum>,
}
/// Defines the GeoHierarchyEntity Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoHierarchyEntity.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoHierarchyEntity/cx:geoHierarchyEntity")]
pub struct GeoHierarchyEntity {
  /// entityName
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityName
  #[sdk(attr(qname = ":entityName"))]
  pub entity_name: crate::simple_type::StringValue,
  /// entityId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityId
  #[sdk(attr(qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
  /// entityType
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityType
  #[sdk(attr(qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
}
/// Defines the GeoChildEntitiesQuery Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoChildEntitiesQuery.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoChildEntitiesQuery/cx:geoChildEntitiesQuery")]
pub struct GeoChildEntitiesQuery {
  /// entityId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityId
  #[sdk(attr(qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "cx:CT_GeoChildTypes/cx:geoChildTypes"))]
  pub geo_child_types: Option<GeoChildTypes>,
}
/// Defines the GeoChildEntities Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoChildEntities.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoChildEntities/cx:geoChildEntities")]
pub struct GeoChildEntities {
  /// _
  #[sdk(child(qname = "cx:CT_GeoHierarchyEntity/cx:geoHierarchyEntity"))]
  pub cx_geo_hierarchy_entity: Vec<GeoHierarchyEntity>,
}
/// Defines the GeoChildEntitiesQueryResult Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoChildEntitiesQueryResult.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoChildEntitiesQueryResult/cx:geoChildEntitiesQueryResult")]
pub struct GeoChildEntitiesQueryResult {
  /// _
  #[sdk(child(qname = "cx:CT_GeoChildEntitiesQuery/cx:geoChildEntitiesQuery"))]
  pub geo_child_entities_query: Option<std::boxed::Box<GeoChildEntitiesQuery>>,
  /// _
  #[sdk(child(qname = "cx:CT_GeoChildEntities/cx:geoChildEntities"))]
  pub geo_child_entities: Option<GeoChildEntities>,
}
/// Defines the GeoParentEntitiesQuery Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoParentEntitiesQuery.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoParentEntitiesQuery/cx:geoParentEntitiesQuery")]
pub struct GeoParentEntitiesQuery {
  /// entityId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityId
  #[sdk(attr(qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
}
/// Defines the GeoEntity Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoEntity.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoEntity/cx:geoEntity")]
pub struct GeoEntity {
  /// entityName
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityName
  #[sdk(attr(qname = ":entityName"))]
  pub entity_name: crate::simple_type::StringValue,
  /// entityType
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityType
  #[sdk(attr(qname = ":entityType"))]
  pub entity_type: EntityTypeEnum,
}
/// Defines the GeoParentEntity Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoParentEntity.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoParentEntity/cx:geoParentEntity")]
pub struct GeoParentEntity {
  /// entityId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :entityId
  #[sdk(attr(qname = ":entityId"))]
  pub entity_id: crate::simple_type::StringValue,
}
/// Defines the GeoParentEntitiesQueryResult Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoParentEntitiesQueryResult.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoParentEntitiesQueryResult/cx:geoParentEntitiesQueryResult")]
pub struct GeoParentEntitiesQueryResult {
  /// _
  #[sdk(child(qname = "cx:CT_GeoParentEntitiesQuery/cx:geoParentEntitiesQuery"))]
  pub geo_parent_entities_query: std::boxed::Box<GeoParentEntitiesQuery>,
  /// _
  #[sdk(child(qname = "cx:CT_GeoEntity/cx:geoEntity"))]
  pub geo_entity: Option<GeoEntity>,
  /// _
  #[sdk(child(qname = "cx:CT_GeoParentEntity/cx:geoParentEntity"))]
  pub geo_parent_entity: Option<GeoParentEntity>,
}
/// Defines the GeoLocationQueryResults Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoLocationQueryResults.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoLocationQueryResults/cx:geoLocationQueryResults")]
pub struct GeoLocationQueryResults {
  /// _
  #[sdk(child(qname = "cx:CT_GeoLocationQueryResult/cx:geoLocationQueryResult"))]
  pub cx_geo_location_query_result: Vec<GeoLocationQueryResult>,
}
/// Defines the GeoDataEntityQueryResults Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoDataEntityQueryResults.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoDataEntityQueryResults/cx:geoDataEntityQueryResults")]
pub struct GeoDataEntityQueryResults {
  /// _
  #[sdk(child(qname = "cx:CT_GeoDataEntityQueryResult/cx:geoDataEntityQueryResult"))]
  pub cx_geo_data_entity_query_result: Vec<GeoDataEntityQueryResult>,
}
/// Defines the GeoDataPointToEntityQueryResults Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoDataPointToEntityQueryResults.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoDataPointToEntityQueryResults/cx:geoDataPointToEntityQueryResults")]
pub struct GeoDataPointToEntityQueryResults {
  /// _
  #[sdk(child(
    qname = "cx:CT_GeoDataPointToEntityQueryResult/cx:geoDataPointToEntityQueryResult"
  ))]
  pub cx_geo_data_point_to_entity_query_result: Vec<GeoDataPointToEntityQueryResult>,
}
/// Defines the GeoChildEntitiesQueryResults Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoChildEntitiesQueryResults.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoChildEntitiesQueryResults/cx:geoChildEntitiesQueryResults")]
pub struct GeoChildEntitiesQueryResults {
  /// _
  #[sdk(child(qname = "cx:CT_GeoChildEntitiesQueryResult/cx:geoChildEntitiesQueryResult"))]
  pub cx_geo_child_entities_query_result: Vec<GeoChildEntitiesQueryResult>,
}
/// Defines the GeoParentEntitiesQueryResults Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoParentEntitiesQueryResults.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoParentEntitiesQueryResults/cx:geoParentEntitiesQueryResults")]
pub struct GeoParentEntitiesQueryResults {
  /// _
  #[sdk(child(qname = "cx:CT_GeoParentEntitiesQueryResult/cx:geoParentEntitiesQueryResult"))]
  pub cx_geo_parent_entities_query_result: Vec<GeoParentEntitiesQueryResult>,
}
/// Defines the Xsdbase64Binary Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:binary.
pub type Xsdbase64Binary = crate::simple_type::Base64BinaryValue;
/// Defines the Clear Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:clear.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Clear/cx:clear")]
pub struct Clear {
  /// _
  #[sdk(child(qname = "cx:CT_GeoLocationQueryResults/cx:geoLocationQueryResults"))]
  pub geo_location_query_results: Option<GeoLocationQueryResults>,
  /// _
  #[sdk(child(qname = "cx:CT_GeoDataEntityQueryResults/cx:geoDataEntityQueryResults"))]
  pub geo_data_entity_query_results: Option<GeoDataEntityQueryResults>,
  /// _
  #[sdk(child(
    qname = "cx:CT_GeoDataPointToEntityQueryResults/cx:geoDataPointToEntityQueryResults"
  ))]
  pub geo_data_point_to_entity_query_results: Option<GeoDataPointToEntityQueryResults>,
  /// _
  #[sdk(child(qname = "cx:CT_GeoChildEntitiesQueryResults/cx:geoChildEntitiesQueryResults"))]
  pub geo_child_entities_query_results: Option<GeoChildEntitiesQueryResults>,
  /// _
  #[sdk(child(qname = "cx:CT_GeoParentEntitiesQueryResults/cx:geoParentEntitiesQueryResults"))]
  pub geo_parent_entities_query_results: Option<GeoParentEntitiesQueryResults>,
}
/// Defines the GeoCache Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geoCache.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_GeoCache/cx:geoCache")]
pub struct GeoCache {
  /// provider
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :provider
  #[sdk(attr(qname = ":provider"))]
  pub provider: crate::simple_type::StringValue,
  #[sdk(choice)]
  pub xml_children: Vec<GeoCacheChoice>,
}
/// Defines the ParentLabelLayout Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:parentLabelLayout.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_ParentLabelLayout/cx:parentLabelLayout")]
pub struct ParentLabelLayout {
  /// val
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub parent_label_layout_val: ParentLabelLayoutVal,
}
/// Defines the RegionLabelLayout Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:regionLabelLayout.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_RegionLabelLayout/cx:regionLabelLayout")]
pub struct RegionLabelLayout {
  /// val
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: RegionLabelLayoutEnum,
}
/// Defines the SeriesElementVisibilities Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:visibility.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_SeriesElementVisibilities/cx:visibility")]
pub struct SeriesElementVisibilities {
  /// connectorLines
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :connectorLines
  #[sdk(attr(qname = ":connectorLines"))]
  pub connector_lines: Option<crate::simple_type::BooleanValue>,
  /// meanLine
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :meanLine
  #[sdk(attr(qname = ":meanLine"))]
  pub mean_line: Option<crate::simple_type::BooleanValue>,
  /// meanMarker
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :meanMarker
  #[sdk(attr(qname = ":meanMarker"))]
  pub mean_marker: Option<crate::simple_type::BooleanValue>,
  /// nonoutliers
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :nonoutliers
  #[sdk(attr(qname = ":nonoutliers"))]
  pub nonoutliers: Option<crate::simple_type::BooleanValue>,
  /// outliers
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :outliers
  #[sdk(attr(qname = ":outliers"))]
  pub outliers: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Aggregation Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:aggregation.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Aggregation/cx:aggregation")]
pub struct Aggregation {}
/// Defines the Binning Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:binning.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Binning/cx:binning")]
pub struct Binning {
  /// intervalClosed
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :intervalClosed
  #[sdk(attr(qname = ":intervalClosed"))]
  pub interval_closed: Option<IntervalClosedSide>,
  /// underflow
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :underflow
  #[sdk(attr(qname = ":underflow"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:double"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub underflow: Option<crate::simple_type::StringValue>,
  /// overflow
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :overflow
  #[sdk(attr(qname = ":overflow"))]
  #[sdk(number_type(source = 0u32, union = 0u64, type_name = "xsd:double"))]
  #[sdk(string_format(source = 1u32, union = 0u64, kind = "token"))]
  pub overflow: Option<crate::simple_type::StringValue>,
  #[sdk(choice)]
  pub xml_children: Option<BinningChoice>,
}
/// Defines the Geography Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:geography.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Geography/cx:geography")]
pub struct Geography {
  /// projectionType
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :projectionType
  #[sdk(attr(qname = ":projectionType"))]
  pub projection_type: Option<GeoProjectionType>,
  /// viewedRegionType
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :viewedRegionType
  #[sdk(attr(qname = ":viewedRegionType"))]
  pub viewed_region_type: Option<GeoMappingLevel>,
  /// cultureLanguage
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :cultureLanguage
  #[sdk(attr(qname = ":cultureLanguage"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub culture_language: crate::simple_type::StringValue,
  /// cultureRegion
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :cultureRegion
  #[sdk(attr(qname = ":cultureRegion"))]
  pub culture_region: crate::simple_type::StringValue,
  /// attribution
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :attribution
  #[sdk(attr(qname = ":attribution"))]
  pub attribution: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "cx:CT_GeoCache/cx:geoCache"))]
  pub geo_cache: Option<GeoCache>,
}
/// Defines the Statistics Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:statistics.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Statistics/cx:statistics")]
pub struct Statistics {
  /// quartileMethod
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :quartileMethod
  #[sdk(attr(qname = ":quartileMethod"))]
  pub quartile_method: Option<QuartileMethod>,
}
/// Defines the Subtotals Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:subtotals.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Subtotals/cx:subtotals")]
pub struct Subtotals {
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/cx:idx"))]
  pub cx_idx: Vec<UnsignedIntegerType>,
}
/// Defines the ExtremeValueColorPosition Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:extremeValue.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_ExtremeValueColorPosition/cx:extremeValue")]
pub struct ExtremeValueColorPosition {}
/// Defines the NumberColorPosition Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:number.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_NumberColorPosition/cx:number")]
pub struct NumberColorPosition {
  /// val
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Defines the PercentageColorPosition Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:percent.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_PercentageColorPosition/cx:percent")]
pub struct PercentageColorPosition {
  /// val
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
}
/// Defines the MinValueColorEndPosition Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:min.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_ValueColorEndPosition/cx:min")]
pub struct MinValueColorEndPosition {
  #[sdk(choice)]
  pub xml_children: Option<MinValueColorEndPositionChoice>,
}
/// Defines the MaxValueColorEndPosition Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:max.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_ValueColorEndPosition/cx:max")]
pub struct MaxValueColorEndPosition {
  #[sdk(choice)]
  pub xml_children: Option<MaxValueColorEndPositionChoice>,
}
/// Defines the OpenXmlValueColorEndPositionElement Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_ValueColorEndPosition/")]
pub struct OpenXmlValueColorEndPositionElement {
  #[sdk(choice)]
  pub xml_children: Option<OpenXmlValueColorEndPositionElementChoice>,
}
/// Defines the ValueColorMiddlePosition Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:mid.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_ValueColorMiddlePosition/cx:mid")]
pub struct ValueColorMiddlePosition {
  #[sdk(choice)]
  pub xml_children: Option<ValueColorMiddlePositionChoice>,
}
/// Defines the DataLabelVisibilities Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:visibility.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_DataLabelVisibilities/cx:visibility")]
pub struct DataLabelVisibilities {
  /// seriesName
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :seriesName
  #[sdk(attr(qname = ":seriesName"))]
  pub series_name: Option<crate::simple_type::BooleanValue>,
  /// categoryName
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :categoryName
  #[sdk(attr(qname = ":categoryName"))]
  pub category_name: Option<crate::simple_type::BooleanValue>,
  /// value
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :value
  #[sdk(attr(qname = ":value"))]
  pub value: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DataLabel Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:dataLabel.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_DataLabel/cx:dataLabel")]
pub struct DataLabel {
  /// idx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub idx: crate::simple_type::UInt32Value,
  /// pos
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :pos
  #[sdk(attr(qname = ":pos"))]
  pub pos: Option<DataLabelPos>,
  /// _
  #[sdk(child(qname = "cx:CT_NumberFormat/cx:numFmt"))]
  pub number_format: Option<NumberFormat>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBody/cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// _
  #[sdk(child(qname = "cx:CT_DataLabelVisibilities/cx:visibility"))]
  pub data_label_visibilities: Option<DataLabelVisibilities>,
  /// _
  #[sdk(text_child(qname = "xsd:string/cx:separator"))]
  pub separator_xsdstring: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the DataLabelHidden Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:dataLabelHidden.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_DataLabelHidden/cx:dataLabelHidden")]
pub struct DataLabelHidden {
  /// idx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub idx: crate::simple_type::UInt32Value,
}
/// Defines the ValueColors Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:valueColors.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_ValueColors/cx:valueColors")]
pub struct ValueColors {
  /// _
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/cx:minColor"))]
  pub min_color_solid_color_fill_properties:
    Option<std::boxed::Box<MinColorSolidColorFillProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/cx:midColor"))]
  pub mid_color_solid_color_fill_properties:
    Option<std::boxed::Box<MidColorSolidColorFillProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/cx:maxColor"))]
  pub max_color_solid_color_fill_properties:
    Option<std::boxed::Box<MaxColorSolidColorFillProperties>>,
}
/// Defines the ValueColorPositions Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:valueColorPositions.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_ValueColorPositions/cx:valueColorPositions")]
pub struct ValueColorPositions {
  /// count
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  #[sdk(number_range(
    source = 0u32,
    min = "2",
    max = "3",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub count: Option<crate::simple_type::Int32Value>,
  /// _
  #[sdk(child(qname = "cx:CT_ValueColorEndPosition/cx:min"))]
  pub min_value_color_end_position: Option<std::boxed::Box<MinValueColorEndPosition>>,
  /// _
  #[sdk(child(qname = "cx:CT_ValueColorMiddlePosition/cx:mid"))]
  pub value_color_middle_position: Option<std::boxed::Box<ValueColorMiddlePosition>>,
  /// _
  #[sdk(child(qname = "cx:CT_ValueColorEndPosition/cx:max"))]
  pub max_value_color_end_position: Option<std::boxed::Box<MaxValueColorEndPosition>>,
}
/// Defines the DataPoint Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:dataPt.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_DataPoint/cx:dataPt")]
pub struct DataPoint {
  /// idx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub idx: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the DataLabels Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:dataLabels.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_DataLabels/cx:dataLabels")]
pub struct DataLabels {
  /// pos
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :pos
  #[sdk(attr(qname = ":pos"))]
  pub pos: Option<DataLabelPos>,
  /// _
  #[sdk(child(qname = "cx:CT_NumberFormat/cx:numFmt"))]
  pub number_format: Option<NumberFormat>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBody/cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// _
  #[sdk(child(qname = "cx:CT_DataLabelVisibilities/cx:visibility"))]
  pub data_label_visibilities: Option<DataLabelVisibilities>,
  /// _
  #[sdk(text_child(qname = "xsd:string/cx:separator"))]
  pub separator_xsdstring: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "cx:CT_DataLabel/cx:dataLabel"))]
  pub cx_data_label: Vec<DataLabel>,
  /// _
  #[sdk(child(qname = "cx:CT_DataLabelHidden/cx:dataLabelHidden"))]
  pub cx_data_label_hidden: Vec<DataLabelHidden>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the DataId Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:dataId.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_DataId/cx:dataId")]
pub struct DataId {
  /// val
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the SeriesLayoutProperties Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:layoutPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_SeriesLayoutProperties/cx:layoutPr")]
pub struct SeriesLayoutProperties {
  /// _
  #[sdk(child(qname = "cx:CT_ParentLabelLayout/cx:parentLabelLayout"))]
  pub parent_label_layout: Option<ParentLabelLayout>,
  /// _
  #[sdk(child(qname = "cx:CT_RegionLabelLayout/cx:regionLabelLayout"))]
  pub region_label_layout: Option<RegionLabelLayout>,
  /// _
  #[sdk(child(qname = "cx:CT_SeriesElementVisibilities/cx:visibility"))]
  pub series_element_visibilities: Option<SeriesElementVisibilities>,
  #[sdk(choice)]
  pub series_layout_properties_choice: Option<SeriesLayoutPropertiesChoice>,
  /// _
  #[sdk(child(qname = "cx:CT_Geography/cx:geography"))]
  pub cx_geography: Option<std::boxed::Box<Geography>>,
  /// _
  #[sdk(child(qname = "cx:CT_Statistics/cx:statistics"))]
  pub cx_statistics: Option<Statistics>,
  /// _
  #[sdk(child(qname = "cx:CT_Subtotals/cx:subtotals"))]
  pub cx_subtotals: Option<Subtotals>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the AxisId Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:axisId.
pub type AxisId = crate::simple_type::UInt32Value;
/// Defines the PlotSurface Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:plotSurface.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_PlotSurface/cx:plotSurface")]
pub struct PlotSurface {
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Series Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:series.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Series/cx:series")]
pub struct Series {
  /// layoutId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :layoutId
  #[sdk(attr(qname = ":layoutId"))]
  pub layout_id: SeriesLayout,
  /// hidden
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  /// ownerIdx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :ownerIdx
  #[sdk(attr(qname = ":ownerIdx"))]
  pub owner_idx: Option<crate::simple_type::UInt32Value>,
  /// uniqueId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :uniqueId
  #[sdk(attr(qname = ":uniqueId"))]
  pub unique_id: Option<crate::simple_type::StringValue>,
  /// formatIdx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :formatIdx
  #[sdk(attr(qname = ":formatIdx"))]
  pub format_idx: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "cx:CT_Text/cx:tx"))]
  pub text: Option<std::boxed::Box<Text>>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "cx:CT_ValueColors/cx:valueColors"))]
  pub value_colors: Option<std::boxed::Box<ValueColors>>,
  /// _
  #[sdk(child(qname = "cx:CT_ValueColorPositions/cx:valueColorPositions"))]
  pub value_color_positions: Option<std::boxed::Box<ValueColorPositions>>,
  /// _
  #[sdk(child(qname = "cx:CT_DataPoint/cx:dataPt"))]
  pub cx_data_pt: Vec<DataPoint>,
  /// _
  #[sdk(child(qname = "cx:CT_DataLabels/cx:dataLabels"))]
  pub cx_data_labels: Option<std::boxed::Box<DataLabels>>,
  /// _
  #[sdk(child(qname = "cx:CT_DataId/cx:dataId"))]
  pub cx_data_id: Option<DataId>,
  /// _
  #[sdk(child(qname = "cx:CT_SeriesLayoutProperties/cx:layoutPr"))]
  pub cx_layout_pr: Option<std::boxed::Box<SeriesLayoutProperties>>,
  /// _
  #[sdk(text_child(qname = "cx:ST_AxisId/cx:axisId"))]
  pub cx_axis_id: Vec<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the PlotAreaRegion Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:plotAreaRegion.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_PlotAreaRegion/cx:plotAreaRegion")]
pub struct PlotAreaRegion {
  /// _
  #[sdk(child(qname = "cx:CT_PlotSurface/cx:plotSurface"))]
  pub plot_surface: Option<std::boxed::Box<PlotSurface>>,
  /// _
  #[sdk(child(qname = "cx:CT_Series/cx:series"))]
  pub cx_series: Vec<Series>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the Axis Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:axis.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Axis/cx:axis")]
pub struct Axis {
  /// id
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// hidden
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :hidden
  #[sdk(attr(qname = ":hidden"))]
  pub hidden: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice)]
  pub axis_choice: Option<AxisChoice>,
  /// _
  #[sdk(child(qname = "cx:CT_AxisTitle/cx:title"))]
  pub cx_title: Option<std::boxed::Box<AxisTitle>>,
  /// _
  #[sdk(child(qname = "cx:CT_AxisUnits/cx:units"))]
  pub cx_units: Option<std::boxed::Box<AxisUnits>>,
  /// _
  #[sdk(child(qname = "cx:CT_Gridlines/cx:majorGridlines"))]
  pub cx_major_gridlines: Option<std::boxed::Box<MajorGridlinesGridlines>>,
  /// _
  #[sdk(child(qname = "cx:CT_Gridlines/cx:minorGridlines"))]
  pub cx_minor_gridlines: Option<std::boxed::Box<MinorGridlinesGridlines>>,
  /// _
  #[sdk(child(qname = "cx:CT_TickMarks/cx:majorTickMarks"))]
  pub cx_major_tick_marks: Option<std::boxed::Box<MajorTickMarksTickMarks>>,
  /// _
  #[sdk(child(qname = "cx:CT_TickMarks/cx:minorTickMarks"))]
  pub cx_minor_tick_marks: Option<std::boxed::Box<MinorTickMarksTickMarks>>,
  /// _
  #[sdk(child(qname = "cx:CT_TickLabels/cx:tickLabels"))]
  pub cx_tick_labels: Option<std::boxed::Box<TickLabels>>,
  /// _
  #[sdk(child(qname = "cx:CT_NumberFormat/cx:numFmt"))]
  pub cx_num_fmt: Option<NumberFormat>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub cx_sp_pr: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBody/cx:txPr"))]
  pub cx_tx_pr: Option<std::boxed::Box<TxPrTextBody>>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the ChartTitle Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:title.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_ChartTitle/cx:title")]
pub struct ChartTitle {
  /// pos
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :pos
  #[sdk(attr(qname = ":pos"))]
  pub pos: Option<SidePos>,
  /// align
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :align
  #[sdk(attr(qname = ":align"))]
  pub align: Option<PosAlign>,
  /// overlay
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :overlay
  #[sdk(attr(qname = ":overlay"))]
  pub overlay: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "cx:CT_Text/cx:tx"))]
  pub text: Option<std::boxed::Box<Text>>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBody/cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// _
  #[sdk(child(qname = "cx:CT_Offset/cx:offset"))]
  pub offset: Option<Offset>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PlotArea Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:plotArea.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_PlotArea/cx:plotArea")]
pub struct PlotArea {
  /// _
  #[sdk(child(qname = "cx:CT_PlotAreaRegion/cx:plotAreaRegion"))]
  pub plot_area_region: std::boxed::Box<PlotAreaRegion>,
  /// _
  #[sdk(child(qname = "cx:CT_Axis/cx:axis"))]
  pub cx_axis: Vec<Axis>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub cx_sp_pr: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the Legend Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:legend.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Legend/cx:legend")]
pub struct Legend {
  /// pos
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :pos
  #[sdk(attr(qname = ":pos"))]
  pub pos: Option<SidePos>,
  /// align
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :align
  #[sdk(attr(qname = ":align"))]
  pub align: Option<PosAlign>,
  /// overlay
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :overlay
  #[sdk(attr(qname = ":overlay"))]
  pub overlay: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_TextBody/cx:txPr"))]
  pub tx_pr_text_body: Option<std::boxed::Box<TxPrTextBody>>,
  /// _
  #[sdk(child(qname = "cx:CT_Offset/cx:offset"))]
  pub offset: Option<Offset>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the FormatOverride Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:fmtOvr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_FormatOverride/cx:fmtOvr")]
pub struct FormatOverride {
  /// idx
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub idx: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "a:CT_ShapeProperties/cx:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the HeaderFooter Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:headerFooter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_HeaderFooter/cx:headerFooter")]
pub struct HeaderFooter {
  /// alignWithMargins
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :alignWithMargins
  #[sdk(attr(qname = ":alignWithMargins"))]
  pub align_with_margins: Option<crate::simple_type::BooleanValue>,
  /// differentOddEven
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :differentOddEven
  #[sdk(attr(qname = ":differentOddEven"))]
  pub different_odd_even: Option<crate::simple_type::BooleanValue>,
  /// differentFirst
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :differentFirst
  #[sdk(attr(qname = ":differentFirst"))]
  pub different_first: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/cx:oddHeader"))]
  pub odd_header_xsdstring: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/cx:oddFooter"))]
  pub odd_footer_xsdstring: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/cx:evenHeader"))]
  pub even_header_xsdstring: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/cx:evenFooter"))]
  pub even_footer_xsdstring: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/cx:firstHeader"))]
  pub first_header_xsdstring: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(text_child(qname = "xsd:string/cx:firstFooter"))]
  pub first_footer_xsdstring: Option<crate::simple_type::StringValue>,
}
/// Defines the PageMargins Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:pageMargins.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_PageMargins/cx:pageMargins")]
pub struct PageMargins {
  /// l
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :l
  #[sdk(attr(qname = ":l"))]
  pub l: crate::simple_type::DoubleValue,
  /// r
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub r: crate::simple_type::DoubleValue,
  /// t
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub t: crate::simple_type::DoubleValue,
  /// b
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :b
  #[sdk(attr(qname = ":b"))]
  pub b: crate::simple_type::DoubleValue,
  /// header
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :header
  #[sdk(attr(qname = ":header"))]
  pub header: crate::simple_type::DoubleValue,
  /// footer
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :footer
  #[sdk(attr(qname = ":footer"))]
  pub footer: crate::simple_type::DoubleValue,
}
/// Defines the PageSetup Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:pageSetup.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_PageSetup/cx:pageSetup")]
pub struct PageSetup {
  /// paperSize
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :paperSize
  #[sdk(attr(qname = ":paperSize"))]
  pub paper_size: Option<crate::simple_type::UInt32Value>,
  /// firstPageNumber
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :firstPageNumber
  #[sdk(attr(qname = ":firstPageNumber"))]
  pub first_page_number: Option<crate::simple_type::UInt32Value>,
  /// orientation
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :orientation
  #[sdk(attr(qname = ":orientation"))]
  pub orientation: Option<PageOrientation>,
  /// blackAndWhite
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :blackAndWhite
  #[sdk(attr(qname = ":blackAndWhite"))]
  pub black_and_white: Option<crate::simple_type::BooleanValue>,
  /// draft
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :draft
  #[sdk(attr(qname = ":draft"))]
  pub draft: Option<crate::simple_type::BooleanValue>,
  /// useFirstPageNumber
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :useFirstPageNumber
  #[sdk(attr(qname = ":useFirstPageNumber"))]
  pub use_first_page_number: Option<crate::simple_type::BooleanValue>,
  /// horizontalDpi
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :horizontalDpi
  #[sdk(attr(qname = ":horizontalDpi"))]
  pub horizontal_dpi: Option<crate::simple_type::Int32Value>,
  /// verticalDpi
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :verticalDpi
  #[sdk(attr(qname = ":verticalDpi"))]
  pub vertical_dpi: Option<crate::simple_type::Int32Value>,
  /// copies
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :copies
  #[sdk(attr(qname = ":copies"))]
  pub copies: Option<crate::simple_type::UInt32Value>,
}
/// Defines the ChartData Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:chartData.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_ChartData/cx:chartData")]
pub struct ChartData {
  /// _
  #[sdk(child(qname = "cx:CT_ExternalData/cx:externalData"))]
  pub external_data: Option<ExternalData>,
  /// _
  #[sdk(child(qname = "cx:CT_Data/cx:data"))]
  pub cx_data: Vec<Data>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the ChartData Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:chartDataIntentionallyChanged.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_ChartData/cx:chartDataIntentionallyChanged")]
pub struct ChartDataIntentionallyChanged {
  /// _
  #[sdk(child(qname = "cx:CT_ExternalData/cx:externalData"))]
  pub external_data: Option<ExternalData>,
  /// _
  #[sdk(child(qname = "cx:CT_Data/cx:data"))]
  pub cx_data: Vec<Data>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub cx_ext_lst: Option<ExtensionList>,
}
/// Defines the Chart Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:chart.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_Chart/cx:chart")]
pub struct Chart {
  /// _
  #[sdk(child(qname = "cx:CT_ChartTitle/cx:title"))]
  pub chart_title: Option<std::boxed::Box<ChartTitle>>,
  /// _
  #[sdk(child(qname = "cx:CT_PlotArea/cx:plotArea"))]
  pub plot_area: std::boxed::Box<PlotArea>,
  /// _
  #[sdk(child(qname = "cx:CT_Legend/cx:legend"))]
  pub legend: Option<std::boxed::Box<Legend>>,
  /// _
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ColorMappingType Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:clrMapOvr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_ColorMapping/cx:clrMapOvr")]
pub struct ColorMappingType {
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
/// Defines the FormatOverrides Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:fmtOvrs.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_FormatOverrides/cx:fmtOvrs")]
pub struct FormatOverrides {
  /// _
  #[sdk(child(qname = "cx:CT_FormatOverride/cx:fmtOvr"))]
  pub cx_fmt_ovr: Vec<FormatOverride>,
}
/// Defines the PrintSettings Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:printSettings.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cx:CT_PrintSettings/cx:printSettings")]
pub struct PrintSettings {
  /// _
  #[sdk(child(qname = "cx:CT_HeaderFooter/cx:headerFooter"))]
  pub header_footer: Option<HeaderFooter>,
  /// _
  #[sdk(child(qname = "cx:CT_PageMargins/cx:pageMargins"))]
  pub page_margins: Option<PageMargins>,
  /// _
  #[sdk(child(qname = "cx:CT_PageSetup/cx:pageSetup"))]
  pub page_setup: Option<PageSetup>,
}
/// Index of subtotal data point.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is cx:idx.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_UnsignedInt/cx:idx")]
pub struct UnsignedIntegerType {
  /// Integer Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum MinColorSolidColorFillPropertiesChoice {
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
pub enum MidColorSolidColorFillPropertiesChoice {
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
pub enum MaxColorSolidColorFillPropertiesChoice {
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
pub enum OpenXmlSolidColorFillPropertiesElementChoice {
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
pub struct NumericDimensionChoiceSequence {
  ///Defines the Formula Class.
  #[sdk(child(qname = "cx:CT_Formula/cx:f"))]
  pub formula: std::boxed::Box<Formula>,
  ///Defines the NfFormula Class.
  #[sdk(child(qname = "cx:CT_Formula/cx:nf"))]
  pub nf_formula: Option<NfFormula>,
  ///Defines the NumericLevel Class.
  #[sdk(child(qname = "cx:CT_NumericLevel/cx:lvl"))]
  pub numeric_level: Vec<NumericLevel>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum NumericDimensionChoice {
  #[sdk(sequence)]
  Sequence(std::boxed::Box<NumericDimensionChoiceSequence>),
  #[sdk(child(qname = "cx:CT_NumericLevel/cx:lvl"))]
  CxLvl(std::boxed::Box<NumericLevel>),
}
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
pub struct StringDimensionChoiceSequence {
  ///Defines the Formula Class.
  #[sdk(child(qname = "cx:CT_Formula/cx:f"))]
  pub formula: std::boxed::Box<Formula>,
  ///Defines the NfFormula Class.
  #[sdk(child(qname = "cx:CT_Formula/cx:nf"))]
  pub nf_formula: Option<NfFormula>,
  ///Defines the StringLevel Class.
  #[sdk(child(qname = "cx:CT_StringLevel/cx:lvl"))]
  pub string_level: Vec<StringLevel>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum StringDimensionChoice {
  #[sdk(sequence)]
  Sequence(std::boxed::Box<StringDimensionChoiceSequence>),
  #[sdk(child(qname = "cx:CT_StringLevel/cx:lvl"))]
  CxLvl(std::boxed::Box<StringLevel>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum DataChoice {
  #[sdk(child(qname = "cx:CT_NumericDimension/cx:numDim"))]
  CxNumDim(std::boxed::Box<NumericDimension>),
  #[sdk(child(qname = "cx:CT_StringDimension/cx:strDim"))]
  CxStrDim(std::boxed::Box<StringDimension>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TextDataChoice {
  #[sdk(sequence)]
  Sequence {
    ///Defines the Formula Class.
    #[sdk(child(qname = "cx:CT_Formula/cx:f"))]
    formula: std::boxed::Box<Formula>,
    ///Defines the VXsdstring Class.
    #[sdk(text_child(qname = "xsd:string/cx:v"))]
    v_xsdstring: Option<crate::simple_type::StringValue>,
  },
  #[sdk(text_child(qname = "xsd:string/cx:v"))]
  CxV(crate::simple_type::StringValue),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TextBodyTypeChoice {
  #[sdk(child(qname = "a:CT_TextBodyProperties/a:bodyPr"))]
  ABodyPr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BodyProperties>,
  ),
  #[sdk(child(qname = "a:CT_TextListStyle/a:lstStyle"))]
  ALstStyle(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ListStyle>,
  ),
  #[sdk(child(qname = "a:CT_TextParagraph/a:p"))]
  AP(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Paragraph>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum TextChoice {
  #[sdk(child(qname = "cx:CT_TextData/cx:txData"))]
  CxTxData(std::boxed::Box<TextData>),
  #[sdk(child(qname = "a:CT_TextBody/cx:rich"))]
  CxRich(std::boxed::Box<RichTextBody>),
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
pub enum OpenXmlGridlinesElementChoice {
  #[sdk(child(qname = "a:CT_ShapeProperties/cx:spPr"))]
  CxSpPr(std::boxed::Box<ShapeProperties>),
  #[sdk(child(qname = "cx:CT_ExtensionList/cx:extLst"))]
  CxExtLst(std::boxed::Box<ExtensionList>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum GeoCacheChoice {
  #[sdk(text_child(qname = "xsd:base64Binary/cx:binary"))]
  CxBinary(crate::simple_type::Base64BinaryValue),
  #[sdk(child(qname = "cx:CT_Clear/cx:clear"))]
  CxClear(std::boxed::Box<Clear>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum BinningChoice {
  #[sdk(text_child(qname = "xsd:double/cx:binSize"))]
  CxBinSize(crate::simple_type::DoubleValue),
  #[sdk(text_child(qname = "xsd:unsignedInt/cx:binCount"))]
  CxBinCount(crate::simple_type::UInt32Value),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum MinValueColorEndPositionChoice {
  #[sdk(child(qname = "cx:CT_ExtremeValueColorPosition/cx:extremeValue"))]
  CxExtremeValue(std::boxed::Box<ExtremeValueColorPosition>),
  #[sdk(child(qname = "cx:CT_NumberColorPosition/cx:number"))]
  CxNumber(std::boxed::Box<NumberColorPosition>),
  #[sdk(child(qname = "cx:CT_PercentageColorPosition/cx:percent"))]
  CxPercent(std::boxed::Box<PercentageColorPosition>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum MaxValueColorEndPositionChoice {
  #[sdk(child(qname = "cx:CT_ExtremeValueColorPosition/cx:extremeValue"))]
  CxExtremeValue(std::boxed::Box<ExtremeValueColorPosition>),
  #[sdk(child(qname = "cx:CT_NumberColorPosition/cx:number"))]
  CxNumber(std::boxed::Box<NumberColorPosition>),
  #[sdk(child(qname = "cx:CT_PercentageColorPosition/cx:percent"))]
  CxPercent(std::boxed::Box<PercentageColorPosition>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum OpenXmlValueColorEndPositionElementChoice {
  #[sdk(child(qname = "cx:CT_ExtremeValueColorPosition/cx:extremeValue"))]
  CxExtremeValue(std::boxed::Box<ExtremeValueColorPosition>),
  #[sdk(child(qname = "cx:CT_NumberColorPosition/cx:number"))]
  CxNumber(std::boxed::Box<NumberColorPosition>),
  #[sdk(child(qname = "cx:CT_PercentageColorPosition/cx:percent"))]
  CxPercent(std::boxed::Box<PercentageColorPosition>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum ValueColorMiddlePositionChoice {
  #[sdk(child(qname = "cx:CT_NumberColorPosition/cx:number"))]
  CxNumber(std::boxed::Box<NumberColorPosition>),
  #[sdk(child(qname = "cx:CT_PercentageColorPosition/cx:percent"))]
  CxPercent(std::boxed::Box<PercentageColorPosition>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum SeriesLayoutPropertiesChoice {
  #[sdk(child(qname = "cx:CT_Aggregation/cx:aggregation"))]
  CxAggregation(std::boxed::Box<Aggregation>),
  #[sdk(child(qname = "cx:CT_Binning/cx:binning"))]
  CxBinning(std::boxed::Box<Binning>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum AxisChoice {
  #[sdk(child(qname = "cx:CT_CategoryAxisScaling/cx:catScaling"))]
  CxCatScaling(std::boxed::Box<CategoryAxisScaling>),
  #[sdk(child(qname = "cx:CT_ValueAxisScaling/cx:valScaling"))]
  CxValScaling(std::boxed::Box<ValueAxisScaling>),
}
