//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:spPr")]
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
            child(variant = CustomGeometry, boxed, qname = "a:custGeom"),
            child(variant = PresetGeometry, boxed, qname = "a:prstGeom")
        )
    )]
  pub shape_properties_choice1: Option<ShapePropertiesChoice>,
  #[sdk(
        choice(
            child(variant = NoFill, qname = "a:noFill"),
            child(variant = SolidFill, boxed, qname = "a:solidFill"),
            child(variant = GradientFill, boxed, qname = "a:gradFill"),
            child(variant = BlipFill, boxed, qname = "a:blipFill"),
            child(variant = PatternFill, boxed, qname = "a:pattFill"),
            empty_child(variant = GroupFill, qname = "a:grpFill")
        )
    )]
  pub shape_properties_choice2: Option<ShapePropertiesChoice2>,
  /// Defines the Outline Class.
  #[sdk(child(qname = "a:ln"))]
  pub outline: Option<std::boxed::Box<crate::schemas::a::Outline>>,
  #[sdk(
        choice(
            child(variant = EffectList, boxed, qname = "a:effectLst"),
            child(variant = EffectDag, boxed, qname = "a:effectDag")
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
/// Defines the UnsignedIntegerType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:explosion")]
pub struct UnsignedIntegerType {
  /// Integer Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the InvertIfNegativeBoolean Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:invertIfNegative")]
pub struct InvertIfNegativeBoolean {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Bubble3DBoolean Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:bubble3D")]
pub struct Bubble3DBoolean {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Marker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:marker")]
pub struct Marker {
  /// Symbol
  #[sdk(child(qname = "c:symbol"))]
  pub symbol: Option<crate::schemas::c::Symbol>,
  /// Size
  #[sdk(child(qname = "c:size"))]
  pub size: Option<crate::schemas::c::Size>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<crate::schemas::c::ChartShapeProperties>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:extLst"))]
  pub extension_list: Option<crate::schemas::c::ExtensionList>,
}
/// Defines the DLbl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:dLbl")]
pub struct DLbl {
  /// Index.
  #[sdk(child(qname = "c:idx"))]
  pub index: crate::schemas::c::Index,
  #[sdk(
        choice(
            child(variant = Delete, qname = "c:delete"),
            sequence(
                variant = Sequence,
                child(qname = "c:layout"),
                child(qname = "c:tx"),
                child(qname = "c:numFmt"),
                child(qname = "c:spPr"),
                child(qname = "c:txPr"),
                child(qname = "c:dLblPos"),
                child(qname = "c:showLegendKey"),
                child(qname = "c:showVal"),
                child(qname = "c:showCatName"),
                child(qname = "c:showSerName"),
                child(qname = "c:showPercent"),
                child(qname = "c:showBubbleSize"),
                text_child(qname = "c:separator")
            )
        )
    )]
  pub d_lbl_choice: Option<DLblChoice>,
  /// Defines the DLblExtensionList Class.
  #[sdk(child(qname = "c:extLst"))]
  pub d_lbl_extension_list: Option<crate::schemas::c::DLblExtensionList>,
}
/// Defines the CategoryFilterExceptions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:categoryFilterExceptions")]
pub struct CategoryFilterExceptions {
  /// Defines the CategoryFilterException Class.
  #[sdk(child(qname = "c16:categoryFilterException"))]
  pub category_filter_exception: Vec<CategoryFilterException>,
}
/// Defines the PivotOptions16 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:pivotOptions16")]
pub struct PivotOptions16 {
  /// Defines the BooleanFalse Class.
  #[sdk(child(qname = "c16:showExpandCollapseFieldButtons"))]
  pub boolean_false: Option<BooleanFalse>,
}
/// Defines the ChartDataPointUniqueIDMap Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:datapointuniqueidmap")]
pub struct ChartDataPointUniqueIdMap {
  /// Defines the ChartDataPointUniqueIDMapEntry Class.
  #[sdk(child(qname = "c16:ptentry"))]
  pub chart_data_point_unique_id_map_entry: Vec<ChartDataPointUniqueIdMapEntry>,
}
/// Defines the UniqueIdChartUniqueID Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:uniqueId")]
pub struct UniqueIdChartUniqueId {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the UniqueID Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:uniqueID")]
pub struct UniqueId {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
}
/// Defines the CategoryFilterException Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:categoryFilterException")]
pub struct CategoryFilterException {
  /// Defines the UniqueIdChartUniqueID Class.
  #[sdk(child(qname = "c16:uniqueId"))]
  pub unique_id_chart_unique_id: UniqueIdChartUniqueId,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "c16:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the UnsignedIntegerType Class.
  #[sdk(child(qname = "c16:explosion"))]
  pub unsigned_integer_type: Option<UnsignedIntegerType>,
  /// Defines the InvertIfNegativeBoolean Class.
  #[sdk(child(qname = "c16:invertIfNegative"))]
  pub invert_if_negative_boolean: Option<InvertIfNegativeBoolean>,
  /// Defines the Bubble3DBoolean Class.
  #[sdk(child(qname = "c16:bubble3D"))]
  pub bubble3_d_boolean: Option<Bubble3DBoolean>,
  /// Defines the Marker Class.
  #[sdk(child(qname = "c16:marker"))]
  pub marker: Option<std::boxed::Box<Marker>>,
  /// Defines the DLbl Class.
  #[sdk(child(qname = "c16:dLbl"))]
  pub d_lbl: Option<std::boxed::Box<DLbl>>,
}
/// Defines the NumberDataType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:numCache")]
pub struct NumberDataType {
  /// Format Code
  #[sdk(text_child(qname = "c:formatCode"))]
  pub format_code: Option<crate::schemas::c::FormatCode>,
  /// Point Count
  #[sdk(child(qname = "c:ptCount"))]
  pub point_count: Option<crate::schemas::c::PointCount>,
  /// Numeric Point.
  #[sdk(child(qname = "c:pt"))]
  pub numeric_point: Vec<crate::schemas::c::NumericPoint>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "c:extLst"))]
  pub extension_list: Option<crate::schemas::c::ExtensionList>,
}
/// Defines the NumFilteredLiteralCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:filteredLitCache")]
pub struct NumFilteredLiteralCache {
  /// Defines the NumberDataType Class.
  #[sdk(child(qname = "c16:numCache"))]
  pub number_data_type: std::boxed::Box<NumberDataType>,
}
/// Defines the StringDataType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:strCache")]
pub struct StringDataType {
  /// Point Count.
  #[sdk(child(qname = "c:ptCount"))]
  pub point_count: Option<crate::schemas::c::PointCount>,
  /// String Point.
  #[sdk(child(qname = "c:pt"))]
  pub string_point: Vec<crate::schemas::c::StringPoint>,
  /// Defines the StrDataExtensionList Class.
  #[sdk(child(qname = "c:extLst"))]
  pub str_data_extension_list: Option<crate::schemas::c::StrDataExtensionList>,
}
/// Defines the StrFilteredLiteralCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:filteredLitCache")]
pub struct StrFilteredLiteralCache {
  /// Defines the StringDataType Class.
  #[sdk(child(qname = "c16:strCache"))]
  pub string_data_type: std::boxed::Box<StringDataType>,
}
/// Defines the MultiLvlStrData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:multiLvlStrCache")]
pub struct MultiLvlStrData {
  /// Point Count.
  #[sdk(child(qname = "c:ptCount"))]
  pub point_count: Option<crate::schemas::c::PointCount>,
  /// Level.
  #[sdk(child(qname = "c:lvl"))]
  pub level: Vec<crate::schemas::c::Level>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "c:extLst"))]
  pub extension_list: Option<crate::schemas::c::ExtensionList>,
}
/// Defines the MultiLvlStrFilteredLiteralCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:filteredLitCache")]
pub struct MultiLvlStrFilteredLiteralCache {
  /// Defines the MultiLvlStrData Class.
  #[sdk(child(qname = "c16:multiLvlStrCache"))]
  pub multi_lvl_str_data: std::boxed::Box<MultiLvlStrData>,
}
/// Defines the LiteralDataChart Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:literalDataChart")]
pub struct LiteralDataChart {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
/// Defines the BooleanFalse Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:showExpandCollapseFieldButtons")]
pub struct BooleanFalse {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the XsdunsignedInt Class.
pub type XsdunsignedInt = crate::simple_type::UInt32Value;
/// Defines the ChartDataPointUniqueIDMapEntry Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16:ptentry")]
pub struct ChartDataPointUniqueIdMapEntry {
  /// Defines the XsdunsignedInt Class.
  #[sdk(text_child(qname = "c16:ptidx"))]
  pub xsdunsigned_int: XsdunsignedInt,
  /// Defines the UniqueID Class.
  #[sdk(child(qname = "c16:uniqueID"))]
  pub unique_id: UniqueId,
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
  NoFill(crate::schemas::a::NoFill),
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct DLblChoiceSequence {
  /// Layout.
  #[sdk(child(qname = "c:layout"))]
  pub layout: Option<std::boxed::Box<crate::schemas::c::Layout>>,
  /// Defines the ChartText Class.
  #[sdk(child(qname = "c:tx"))]
  pub chart_text: Option<std::boxed::Box<crate::schemas::c::ChartText>>,
  /// Number Format.
  #[sdk(child(qname = "c:numFmt"))]
  pub numbering_format: Option<crate::schemas::c::NumberingFormat>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<crate::schemas::c::ChartShapeProperties>>,
  /// Defines the TextProperties Class.
  #[sdk(child(qname = "c:txPr"))]
  pub text_properties: Option<std::boxed::Box<crate::schemas::c::TextProperties>>,
  /// Data Label Position.
  #[sdk(child(qname = "c:dLblPos"))]
  pub data_label_position: Option<crate::schemas::c::DataLabelPosition>,
  /// Show Legend Key.
  #[sdk(child(qname = "c:showLegendKey"))]
  pub show_legend_key: Option<crate::schemas::c::ShowLegendKey>,
  /// Show Value.
  #[sdk(child(qname = "c:showVal"))]
  pub show_value: Option<crate::schemas::c::ShowValue>,
  /// Show Category Name.
  #[sdk(child(qname = "c:showCatName"))]
  pub show_category_name: Option<crate::schemas::c::ShowCategoryName>,
  /// Show Series Name.
  #[sdk(child(qname = "c:showSerName"))]
  pub show_series_name: Option<crate::schemas::c::ShowSeriesName>,
  /// Show Percent.
  #[sdk(child(qname = "c:showPercent"))]
  pub show_percent: Option<crate::schemas::c::ShowPercent>,
  /// Show Bubble Size.
  #[sdk(child(qname = "c:showBubbleSize"))]
  pub show_bubble_size: Option<crate::schemas::c::ShowBubbleSize>,
  /// Separator.
  #[sdk(text_child(qname = "c:separator"))]
  pub separator: Option<crate::schemas::c::Separator>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum DLblChoice {
  Delete(crate::schemas::c::Delete),
  /// Sequence of c:layout, c:tx, c:numFmt, c:spPr, c:txPr, c:dLblPos, c:showLegendKey, c:showVal, c:showCatName, c:showSerName, c:showPercent, c:showBubbleSize, c:separator
  Sequence(std::boxed::Box<DLblChoiceSequence>),
}
