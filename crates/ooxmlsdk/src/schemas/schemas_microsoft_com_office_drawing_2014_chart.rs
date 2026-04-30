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
/// Defines the BooleanType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c:CT_Boolean/")]
pub struct BooleanType {
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
  pub symbol: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Symbol>,
  /// Size
  #[sdk(child(qname = "c:CT_MarkerSize/c:size"))]
  pub size: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Size>,
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    >,
  >,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ExtensionList>,
}
/// Defines the DLbl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c:CT_DLbl/c16:dLbl")]
pub struct DLbl {
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index>,
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
  /// _
  #[sdk(child(qname = "c:CT_DLblExtensionList/c:extLst"))]
  pub c_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DLblExtensionList>,
}
/// Defines the CategoryFilterExceptions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "c16:CT_CategoryFilterExceptions/c16:categoryFilterExceptions"
)]
pub struct CategoryFilterExceptions {
  /// _
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
  /// _
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
  /// _
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
/// Defines the UniqueIDChart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c16:CT_ChartUniqueID/")]
pub struct UniqueIdChart {
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
  /// _
  #[sdk(child(office2016, qname = "c16:CT_ChartUniqueID/c16:uniqueId"))]
  pub unique_id_chart_unique_id: std::boxed::Box<UniqueIdChartUniqueId>,
  /// _
  #[sdk(child(office2016, qname = "a:CT_ShapeProperties/c16:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2016, qname = "c:CT_UnsignedInt/c16:explosion"))]
  pub unsigned_integer_type: Option<UnsignedIntegerType>,
  /// _
  #[sdk(child(office2016, qname = "c:CT_Boolean/c16:invertIfNegative"))]
  pub invert_if_negative_boolean: Option<InvertIfNegativeBoolean>,
  /// _
  #[sdk(child(office2016, qname = "c:CT_Boolean/c16:bubble3D"))]
  pub bubble3_d_boolean: Option<Bubble3DBoolean>,
  /// _
  #[sdk(child(office2016, qname = "c:CT_Marker/c16:marker"))]
  pub marker: Option<std::boxed::Box<Marker>>,
  /// _
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
  pub point_count:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PointCount>,
  /// _
  #[sdk(child(qname = "c:CT_NumVal/c:pt"))]
  pub c_pt: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::NumericPoint>,
  /// _
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ExtensionList>,
}
/// Defines the NumFilteredLiteralCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "c16:CT_NumFilteredLiteralCache/c16:filteredLitCache"
)]
pub struct NumFilteredLiteralCache {
  /// _
  #[sdk(child(office2016, qname = "c:CT_NumData/c16:numCache"))]
  pub number_data_type: std::boxed::Box<NumberDataType>,
}
/// Defines the StringDataType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c:CT_StrData/c16:strCache")]
pub struct StringDataType {
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  pub point_count:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PointCount>,
  /// _
  #[sdk(child(qname = "c:CT_StrVal/c:pt"))]
  pub c_pt: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::StringPoint>,
  /// _
  #[sdk(child(qname = "c:CT_StrDataExtensionList/c:extLst"))]
  pub c_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::StrDataExtensionList>,
}
/// Defines the StrFilteredLiteralCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "c16:CT_StrFilteredLiteralCache/c16:filteredLitCache"
)]
pub struct StrFilteredLiteralCache {
  /// _
  #[sdk(child(office2016, qname = "c:CT_StrData/c16:strCache"))]
  pub string_data_type: std::boxed::Box<StringDataType>,
}
/// Defines the MultiLvlStrData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "c:CT_MultiLvlStrData/c16:multiLvlStrCache")]
pub struct MultiLvlStrData {
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:ptCount"))]
  pub point_count:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PointCount>,
  /// _
  #[sdk(child(qname = "c:CT_Lvl/c:lvl"))]
  pub c_lvl: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Level>,
  /// _
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub c_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ExtensionList>,
}
/// Defines the MultiLvlStrFilteredLiteralCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "c16:CT_MultiLvlStrFilteredLiteralCache/c16:filteredLitCache"
)]
pub struct MultiLvlStrFilteredLiteralCache {
  /// _
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
  /// _
  #[sdk(text_child(office2016, qname = "xsd:unsignedInt/c16:ptidx"))]
  pub xsdunsigned_int: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(office2016, qname = "c16:CT_ChartUniqueID/c16:uniqueID"))]
  pub unique_id: std::boxed::Box<UniqueId>,
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
pub struct DLblChoiceSequence {
  /// Layout.
  #[sdk(child(qname = "c:CT_Layout/c:layout"))]
  pub layout: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Layout>,
  >,
  /// Defines the ChartText Class.
  #[sdk(child(qname = "c:CT_Tx/c:tx"))]
  pub chart_text: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartText>,
  >,
  /// Number Format.
  #[sdk(child(qname = "c:CT_NumFmt/c:numFmt"))]
  pub numbering_format:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::NumberingFormat>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    >,
  >,
  /// Defines the TextProperties Class.
  #[sdk(child(qname = "a:CT_TextBody/c:txPr"))]
  pub text_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::TextProperties,
    >,
  >,
  /// Data Label Position.
  #[sdk(child(qname = "c:CT_DLblPos/c:dLblPos"))]
  pub data_label_position:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataLabelPosition>,
  /// Show Legend Key.
  #[sdk(child(qname = "c:CT_Boolean/c:showLegendKey"))]
  pub show_legend_key:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ShowLegendKey>,
  /// Show Value.
  #[sdk(child(qname = "c:CT_Boolean/c:showVal"))]
  pub show_value:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ShowValue>,
  /// Show Category Name.
  #[sdk(child(qname = "c:CT_Boolean/c:showCatName"))]
  pub show_category_name:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ShowCategoryName>,
  /// Show Series Name.
  #[sdk(child(qname = "c:CT_Boolean/c:showSerName"))]
  pub show_series_name:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ShowSeriesName>,
  /// Show Percent.
  #[sdk(child(qname = "c:CT_Boolean/c:showPercent"))]
  pub show_percent:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ShowPercent>,
  /// Show Bubble Size.
  #[sdk(child(qname = "c:CT_Boolean/c:showBubbleSize"))]
  pub show_bubble_size:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ShowBubbleSize>,
  /// Separator.
  #[sdk(text_child(qname = "xsd:string/c:separator"))]
  pub separator: Option<crate::simple_type::StringValue>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum DLblChoice {
  /// Delete.
  #[sdk(child(qname = "c:CT_Boolean/c:delete"))]
  CDelete(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Delete>),
  /// Sequence of c:layout, c:tx, c:numFmt, c:spPr, c:txPr, c:dLblPos, c:showLegendKey, c:showVal, c:showCatName, c:showSerName, c:showPercent, c:showBubbleSize, c:separator
  #[sdk(sequence)]
  Sequence(std::boxed::Box<DLblChoiceSequence>),
}
