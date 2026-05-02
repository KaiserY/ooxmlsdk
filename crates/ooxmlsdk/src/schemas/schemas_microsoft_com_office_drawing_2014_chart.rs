//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "a:CT_ShapeProperties/c16:spPr")]
pub struct ShapeProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Black and White Mode
  #[sdk(attr(qname = ":bwMode"))]
  #[sdk(string_format(kind = "token"))]
  pub black_white_mode: Option<crate::schemas::a::BlackWhiteModeValues>,
  /// 2D Transform for Individual Objects
  #[sdk(child(qname = "a:CT_Transform2D/a:xfrm"))]
  pub transform2_d: Option<std::boxed::Box<crate::schemas::a::Transform2D>>,
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
  pub a_ln: Option<std::boxed::Box<crate::schemas::a::Outline>>,
  #[sdk(choice(
    qname = "a:CT_EffectList/a:effectLst",
    qname = "a:CT_EffectContainer/a:effectDag"
  ))]
  pub shape_properties_choice3: Option<ShapePropertiesChoice3>,
  /// 3D Scene Properties.
  #[sdk(child(qname = "a:CT_Scene3D/a:scene3d"))]
  pub a_scene3d: Option<std::boxed::Box<crate::schemas::a::Scene3DType>>,
  /// Apply 3D shape properties.
  #[sdk(child(qname = "a:CT_Shape3D/a:sp3d"))]
  pub a_sp3d: Option<std::boxed::Box<crate::schemas::a::Shape3DType>>,
  /// Defines the ShapePropertiesExtensionList Class.
  #[sdk(child(qname = "a:CT_ShapePropertiesExtensionList/a:extLst"))]
  pub a_ext_lst: Option<crate::schemas::a::ShapePropertiesExtensionList>,
}
/// Defines the UnsignedIntegerType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c:CT_UnsignedInt/c16:explosion")]
pub struct UnsignedIntegerType {
  /// Integer Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the InvertIfNegativeBoolean Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c:CT_Boolean/c16:invertIfNegative")]
pub struct InvertIfNegativeBoolean {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Bubble3DBoolean Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c:CT_Boolean/c16:bubble3D")]
pub struct Bubble3DBoolean {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Marker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c:CT_Marker/c16:marker")]
pub struct Marker {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Symbol
  #[sdk(child(qname = "c:CT_MarkerStyle/c:symbol"))]
  pub symbol: Option<crate::schemas::c::Symbol>,
  /// Size
  #[sdk(child(qname = "c:CT_MarkerSize/c:size"))]
  pub size: Option<crate::schemas::c::Size>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<crate::schemas::c::ChartShapeProperties>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list: Option<crate::schemas::c::ExtensionList>,
}
/// Defines the DLbl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c:CT_DLbl/c16:dLbl")]
pub struct DLbl {
  /// Index.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index: Option<crate::schemas::c::Index>,
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
  pub d_lbl_choice: Option<DLblChoice>,
  /// Defines the DLblExtensionList Class.
  #[sdk(child(qname = "c:CT_DLblExtensionList/c:extLst"))]
  pub c_ext_lst: Option<crate::schemas::c::DLblExtensionList>,
}
/// Defines the CategoryFilterExceptions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"
)]
pub struct CategoryFilterExceptions {
  /// Defines the CategoryFilterException Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_CategoryFilterException/c16:categoryFilterException"
  ))]
  pub c16_category_filter_exception: Vec<CategoryFilterException>,
}
/// Defines the PivotOptions16 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c16:CT_PivotOptions16/c16:pivotOptions16")]
pub struct PivotOptions16 {
  /// Defines the BooleanFalse Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_BooleanFalse/c16:showExpandCollapseFieldButtons"
  ))]
  pub boolean_false: Option<BooleanFalse>,
}
/// Defines the ChartDataPointUniqueIDMap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "c16:CT_ChartDataPointUniqueIDMap/c16:datapointuniqueidmap"
)]
pub struct ChartDataPointUniqueIdMap {
  /// Defines the ChartDataPointUniqueIDMapEntry Class.
  #[sdk(child(
    office2016,
    qname = "c16:CT_ChartDataPointUniqueIDMapEntry/c16:ptentry"
  ))]
  pub c16_ptentry: Vec<ChartDataPointUniqueIdMapEntry>,
}
/// Defines the UniqueIdChartUniqueID Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c16:CT_ChartUniqueID/c16:uniqueId")]
pub struct UniqueIdChartUniqueId {
  /// val
  #[sdk(attr(office2016, qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the UniqueID Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c16:CT_ChartUniqueID/c16:uniqueID")]
pub struct UniqueId {
  /// val
  #[sdk(attr(office2016, qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the CategoryFilterException Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "c16:CT_CategoryFilterException/c16:categoryFilterException"
)]
pub struct CategoryFilterException {
  /// Defines the UniqueIdChartUniqueID Class.
  #[sdk(child(office2016, qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  pub unique_id_chart_unique_id: std::boxed::Box<UniqueIdChartUniqueId>,
  /// Defines the ShapeProperties Class.
  #[sdk(child(office2016, qname = "a:CT_ShapeProperties/c16:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the UnsignedIntegerType Class.
  #[sdk(child(office2016, qname = "c:CT_UnsignedInt/c16:explosion"))]
  pub unsigned_integer_type: Option<UnsignedIntegerType>,
  /// Defines the InvertIfNegativeBoolean Class.
  #[sdk(child(office2016, qname = "c:CT_Boolean/c16:invertIfNegative"))]
  pub invert_if_negative_boolean: Option<InvertIfNegativeBoolean>,
  /// Defines the Bubble3DBoolean Class.
  #[sdk(child(office2016, qname = "c:CT_Boolean/c16:bubble3D"))]
  pub bubble3_d_boolean: Option<Bubble3DBoolean>,
  /// Defines the Marker Class.
  #[sdk(child(office2016, qname = "c:CT_Marker/c16:marker"))]
  pub marker: Option<std::boxed::Box<Marker>>,
  /// Defines the DLbl Class.
  #[sdk(child(office2016, qname = "c:CT_DLbl/c16:dLbl"))]
  pub d_lbl: Option<std::boxed::Box<DLbl>>,
}
/// Defines the NumberDataType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c:CT_NumData/c16:numCache")]
pub struct NumberDataType {
  /// Format Code
  #[sdk(text_child(qname = "c:ST_Xstring/c:formatCode"))]
  pub format_code: Option<crate::simple_type::StringValue>,
  /// Point Count
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  pub point_count: Option<crate::schemas::c::PointCount>,
  /// Numeric Point.
  #[sdk(child(qname = "c:CT_NumVal/c:pt"))]
  pub c_pt: Vec<crate::schemas::c::NumericPoint>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<crate::schemas::c::ExtensionList>,
}
/// Defines the NumFilteredLiteralCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "c16:CT_NumFilteredLiteralCache/c16:filteredLitCache"
)]
pub struct NumFilteredLiteralCache {
  /// Defines the NumberDataType Class.
  #[sdk(child(office2016, qname = "c:CT_NumData/c16:numCache"))]
  pub number_data_type: std::boxed::Box<NumberDataType>,
}
/// Defines the StringDataType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c:CT_StrData/c16:strCache")]
pub struct StringDataType {
  /// Point Count.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  pub point_count: Option<crate::schemas::c::PointCount>,
  /// String Point.
  #[sdk(child(qname = "c:CT_StrVal/c:pt"))]
  pub c_pt: Vec<crate::schemas::c::StringPoint>,
  /// Defines the StrDataExtensionList Class.
  #[sdk(child(qname = "c:CT_StrDataExtensionList/c:extLst"))]
  pub c_ext_lst: Option<crate::schemas::c::StrDataExtensionList>,
}
/// Defines the StrFilteredLiteralCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "c16:CT_StrFilteredLiteralCache/c16:filteredLitCache"
)]
pub struct StrFilteredLiteralCache {
  /// Defines the StringDataType Class.
  #[sdk(child(office2016, qname = "c:CT_StrData/c16:strCache"))]
  pub string_data_type: std::boxed::Box<StringDataType>,
}
/// Defines the MultiLvlStrData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c:CT_MultiLvlStrData/c16:multiLvlStrCache")]
pub struct MultiLvlStrData {
  /// Point Count.
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  pub point_count: Option<crate::schemas::c::PointCount>,
  /// Level.
  #[sdk(child(qname = "c:CT_Lvl/c:lvl"))]
  pub c_lvl: Vec<crate::schemas::c::Level>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst: Option<crate::schemas::c::ExtensionList>,
}
/// Defines the MultiLvlStrFilteredLiteralCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "c16:CT_MultiLvlStrFilteredLiteralCache/c16:filteredLitCache"
)]
pub struct MultiLvlStrFilteredLiteralCache {
  /// Defines the MultiLvlStrData Class.
  #[sdk(child(office2016, qname = "c:CT_MultiLvlStrData/c16:multiLvlStrCache"))]
  pub multi_lvl_str_data: std::boxed::Box<MultiLvlStrData>,
}
/// Defines the LiteralDataChart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c16:CT_LiteralDataChart/c16:literalDataChart")]
pub struct LiteralDataChart {
  /// val
  #[sdk(attr(office2016, qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
/// Defines the BooleanFalse Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "c16:CT_BooleanFalse/c16:showExpandCollapseFieldButtons"
)]
pub struct BooleanFalse {
  /// val
  #[sdk(attr(office2016, qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the XsdunsignedInt Class.
pub type XsdunsignedInt = crate::simple_type::UInt32Value;
/// Defines the ChartDataPointUniqueIDMapEntry Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "c16:CT_ChartDataPointUniqueIDMapEntry/c16:ptentry"
)]
pub struct ChartDataPointUniqueIdMapEntry {
  /// Defines the XsdunsignedInt Class.
  #[sdk(text_child(office2016, qname = "xsd:unsignedInt/c16:ptidx"))]
  pub xsdunsigned_int: crate::simple_type::UInt32Value,
  /// Defines the UniqueID Class.
  #[sdk(child(office2016, qname = "c16:CT_ChartUniqueID/c16:uniqueID"))]
  pub unique_id: std::boxed::Box<UniqueId>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice {
  /// Custom geometry.
  #[sdk(child(qname = "a:CT_CustomGeometry2D/a:custGeom"))]
  ACustGeom(std::boxed::Box<crate::schemas::a::CustomGeometry>),
  /// Preset geometry.
  #[sdk(child(qname = "a:CT_PresetGeometry2D/a:prstGeom"))]
  APrstGeom(std::boxed::Box<crate::schemas::a::PresetGeometry>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice2 {
  /// Defines the NoFill Class.
  #[sdk(child(qname = "a:CT_NoFillProperties/a:noFill"))]
  ANoFill(std::boxed::Box<crate::schemas::a::NoFill>),
  /// Defines the SolidFill Class.
  #[sdk(child(qname = "a:CT_SolidColorFillProperties/a:solidFill"))]
  ASolidFill(std::boxed::Box<crate::schemas::a::SolidFill>),
  /// Defines the GradientFill Class.
  #[sdk(child(qname = "a:CT_GradientFillProperties/a:gradFill"))]
  AGradFill(std::boxed::Box<crate::schemas::a::GradientFill>),
  /// Defines the BlipFill Class.
  #[sdk(child(qname = "a:CT_BlipFillProperties/a:blipFill"))]
  ABlipFill(std::boxed::Box<crate::schemas::a::BlipFill>),
  /// Pattern Fill.
  #[sdk(child(qname = "a:CT_PatternFillProperties/a:pattFill"))]
  APattFill(std::boxed::Box<crate::schemas::a::PatternFill>),
  /// Group Fill.
  #[sdk(empty_child(qname = "a:CT_GroupFillProperties/a:grpFill"))]
  AGrpFill,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShapePropertiesChoice3 {
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectList/a:effectLst"))]
  AEffectLst(std::boxed::Box<crate::schemas::a::EffectList>),
  /// Effect Container.
  #[sdk(child(qname = "a:CT_EffectContainer/a:effectDag"))]
  AEffectDag(std::boxed::Box<crate::schemas::a::EffectDag>),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct DLblChoiceSequence {
  /// Layout.
  #[sdk(child(qname = "c:CT_Layout/c:layout"))]
  pub layout: Option<std::boxed::Box<crate::schemas::c::Layout>>,
  /// Defines the ChartText Class.
  #[sdk(child(qname = "c:CT_Tx/c:tx"))]
  pub chart_text: Option<std::boxed::Box<crate::schemas::c::ChartText>>,
  /// Number Format.
  #[sdk(child(qname = "c:CT_NumFmt/c:numFmt"))]
  pub numbering_format: Option<crate::schemas::c::NumberingFormat>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<crate::schemas::c::ChartShapeProperties>>,
  /// Defines the TextProperties Class.
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub text_properties: Option<std::boxed::Box<crate::schemas::c::TextProperties>>,
  /// Data Label Position.
  #[sdk(child(qname = "c:CT_DLblPos/c:dLblPos"))]
  pub data_label_position: Option<crate::schemas::c::DataLabelPosition>,
  /// Show Legend Key.
  #[sdk(child(qname = "c:CT_Boolean/c:showLegendKey"))]
  pub show_legend_key: Option<crate::schemas::c::ShowLegendKey>,
  /// Show Value.
  #[sdk(child(qname = "c:CT_Boolean/c:showVal"))]
  pub show_value: Option<crate::schemas::c::ShowValue>,
  /// Show Category Name.
  #[sdk(child(qname = "c:CT_Boolean/c:showCatName"))]
  pub show_category_name: Option<crate::schemas::c::ShowCategoryName>,
  /// Show Series Name.
  #[sdk(child(qname = "c:CT_Boolean/c:showSerName"))]
  pub show_series_name: Option<crate::schemas::c::ShowSeriesName>,
  /// Show Percent.
  #[sdk(child(qname = "c:CT_Boolean/c:showPercent"))]
  pub show_percent: Option<crate::schemas::c::ShowPercent>,
  /// Show Bubble Size.
  #[sdk(child(qname = "c:CT_Boolean/c:showBubbleSize"))]
  pub show_bubble_size: Option<crate::schemas::c::ShowBubbleSize>,
  /// Separator.
  #[sdk(text_child(qname = "xsd:string/c:separator"))]
  pub separator: Option<crate::simple_type::StringValue>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DLblChoice {
  #[sdk(child(qname = "c:CT_Boolean/c:delete"))]
  CDelete(std::boxed::Box<crate::schemas::c::Delete>),
  /// Sequence of c:layout, c:tx, c:numFmt, c:spPr, c:txPr, c:dLblPos, c:showLegendKey, c:showVal, c:showCatName, c:showSerName, c:showPercent, c:showBubbleSize, c:separator
  #[sdk(sequence)]
  Sequence(std::boxed::Box<DLblChoiceSequence>),
}
