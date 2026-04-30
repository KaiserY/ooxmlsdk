//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PivotSource Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_PivotSource/c15:pivotSource")]
pub struct PivotSource {
  /// Pivot Name
  #[sdk(text_child(qname = "c:ST_Xstring/c:name"))]
  pub pivot_table_name: crate::simple_type::StringValue,
  /// Format ID
  #[sdk(child(qname = "c:CT_UnsignedInt/c:fmtId"))]
  pub format_id:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::FormatId>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ExtensionList>,
}
/// Defines the NumberingFormat Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_NumFmt/c15:numFmt")]
pub struct NumberingFormat {
  /// Number Format Code
  #[sdk(attr(qname = ":formatCode"))]
  pub format_code: crate::simple_type::StringValue,
  /// Linked to Source
  #[sdk(attr(qname = ":sourceLinked"))]
  pub source_linked: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ShapeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "a:CT_ShapeProperties/c15:spPr")]
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
/// Defines the Layout Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_Layout/c15:layout")]
pub struct Layout {
  /// Manual Layout
  #[sdk(child(qname = "c:CT_ManualLayout/c:manualLayout"))]
  pub manual_layout: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ManualLayout>,
  >,
  /// Chart Extensibility
  #[sdk(child(qname = "c:CT_ExtensionList/c:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ExtensionList>,
}
/// Defines the FullReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c15:CT_FullRef/c15:fullRef")]
pub struct FullReference {
  /// _
  #[sdk(text_child(office2013, qname = "xsd:string/c15:sqref"))]
  pub sequence_of_references: crate::simple_type::StringValue,
}
/// Defines the LevelReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c15:CT_LevelRef/c15:levelRef")]
pub struct LevelReference {
  /// _
  #[sdk(text_child(office2013, qname = "xsd:string/c15:sqref"))]
  pub sequence_of_references: crate::simple_type::StringValue,
}
/// Defines the FormulaReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c15:CT_FormulaRef/c15:formulaRef")]
pub struct FormulaReference {
  /// _
  #[sdk(text_child(office2013, qname = "xsd:string/c15:sqref"))]
  pub sequence_of_references: crate::simple_type::StringValue,
}
/// Defines the FilteredSeriesTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "c15:CT_FilteredSeriesTitle/c15:filteredSeriesTitle"
)]
pub struct FilteredSeriesTitle {
  /// _
  #[sdk(child(office2013, qname = "c:CT_Tx/c15:tx"))]
  pub chart_text: std::boxed::Box<ChartText>,
}
/// Defines the FilteredCategoryTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "c15:CT_FilteredCategoryTitle/c15:filteredCategoryTitle"
)]
pub struct FilteredCategoryTitle {
  /// _
  #[sdk(child(office2013, qname = "c:CT_AxDataSource/c15:cat"))]
  pub axis_data_source_type: std::boxed::Box<AxisDataSourceType>,
}
/// Defines the FilteredAreaSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c15:CT_FilteredAreaSer/c15:filteredAreaSeries")]
pub struct FilteredAreaSeries {
  /// _
  #[sdk(child(office2013, qname = "c:CT_AreaSer/c15:ser"))]
  pub area_chart_series: std::boxed::Box<AreaChartSeries>,
}
/// Defines the FilteredBarSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c15:CT_FilteredBarSer/c15:filteredBarSeries")]
pub struct FilteredBarSeries {
  /// _
  #[sdk(child(office2013, qname = "c:CT_BarSer/c15:ser"))]
  pub bar_chart_series: std::boxed::Box<BarChartSeries>,
}
/// Defines the FilteredBubbleSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "c15:CT_FilteredBubbleSer/c15:filteredBubbleSeries"
)]
pub struct FilteredBubbleSeries {
  /// _
  #[sdk(child(office2013, qname = "c:CT_BubbleSer/c15:ser"))]
  pub bubble_chart_series: std::boxed::Box<BubbleChartSeries>,
}
/// Defines the FilteredLineSeriesExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c15:CT_FilteredLineSer/c15:filteredLineSeries")]
pub struct FilteredLineSeriesExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// _
  #[sdk(child(office2013, qname = "c:CT_LineSer/c15:ser"))]
  pub line_chart_series: std::boxed::Box<LineChartSeries>,
}
/// Defines the FilteredPieSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c15:CT_FilteredPieSer/c15:filteredPieSeries")]
pub struct FilteredPieSeries {
  /// _
  #[sdk(child(office2013, qname = "c:CT_PieSer/c15:ser"))]
  pub pie_chart_series: std::boxed::Box<PieChartSeries>,
}
/// Defines the FilteredRadarSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c15:CT_FilteredRadarSer/c15:filteredRadarSeries")]
pub struct FilteredRadarSeries {
  /// _
  #[sdk(child(office2013, qname = "c:CT_RadarSer/c15:ser"))]
  pub radar_chart_series: std::boxed::Box<RadarChartSeries>,
}
/// Defines the FilteredScatterSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "c15:CT_FilteredScatterSer/c15:filteredScatterSeries"
)]
pub struct FilteredScatterSeries {
  /// _
  #[sdk(child(office2013, qname = "c:CT_ScatterSer/c15:ser"))]
  pub scatter_chart_series: std::boxed::Box<ScatterChartSeries>,
}
/// Defines the FilteredSurfaceSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "c15:CT_FilteredSurfaceSer/c15:filteredSurfaceSeries"
)]
pub struct FilteredSurfaceSeries {
  /// _
  #[sdk(child(office2013, qname = "c:CT_SurfaceSer/c15:ser"))]
  pub surface_chart_series: std::boxed::Box<SurfaceChartSeries>,
}
/// Defines the DataLabelsRange Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c15:CT_SeriesDataLabelsRange/c15:datalabelsRange")]
pub struct DataLabelsRange {
  /// _
  #[sdk(text_child(office2013, qname = "xsd:string/c15:f"))]
  pub formula: crate::simple_type::StringValue,
  /// _
  #[sdk(child(office2013, qname = "c:CT_StrData/c15:dlblRangeCache"))]
  pub data_labels_range_chache: Option<std::boxed::Box<DataLabelsRangeChache>>,
}
/// Defines the CategoryFilterExceptions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "c15:CT_CategoryFilterExceptions/c15:categoryFilterExceptions"
)]
pub struct CategoryFilterExceptions {
  /// _
  #[sdk(child(
    office2013,
    qname = "c15:CT_CategoryFilterException/c15:categoryFilterException"
  ))]
  pub c15_category_filter_exception: Vec<CategoryFilterException>,
}
/// Defines the DataLabelFieldTable Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c15:CT_DataLabelFieldTable/c15:dlblFieldTable")]
pub struct DataLabelFieldTable {
  /// _
  #[sdk(child(office2013, qname = "c15:CT_DataLabelFieldTableEntry/c15:dlblFTEntry"))]
  pub c15_dlbl_ft_entry: Vec<DataLabelFieldTableEntry>,
}
/// Defines the ExceptionForSave Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_Boolean/c15:xForSave")]
pub struct ExceptionForSave {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ShowDataLabelsRange Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_Boolean/c15:showDataLabelsRange")]
pub struct ShowDataLabelsRange {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ShowLeaderLines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_Boolean/c15:showLeaderLines")]
pub struct ShowLeaderLines {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the AutoGeneneratedCategories Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_Boolean/c15:autoCat")]
pub struct AutoGeneneratedCategories {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the InvertIfNegativeBoolean Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_Boolean/c15:invertIfNegative")]
pub struct InvertIfNegativeBoolean {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Bubble3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_Boolean/c15:bubble3D")]
pub struct Bubble3D {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ChartText Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_Tx/c15:tx")]
pub struct ChartText {
  #[sdk(choice(
    qname = "c:CT_StrRef/c:strRef",
    qname = "a:CT_TextBody/c:rich",
    qname = "c:CT_StrData/c:strLit"
  ))]
  pub xml_children: Option<ChartTextChoice>,
}
/// Defines the LeaderLines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_ChartLines/c15:leaderLines")]
pub struct LeaderLines {
  /// _
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    >,
  >,
}
/// Defines the SequenceOfReferences Class.
pub type SequenceOfReferences = crate::simple_type::StringValue;
/// Defines the Formula Class.
pub type Formula = crate::simple_type::StringValue;
/// Defines the TextFieldGuid Class.
pub type TextFieldGuid = crate::simple_type::StringValue;
/// Defines the AxisDataSourceType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_AxDataSource/c15:cat")]
pub struct AxisDataSourceType {
  #[sdk(choice(
    qname = "c:CT_MultiLvlStrRef/c:multiLvlStrRef",
    qname = "c:CT_NumRef/c:numRef",
    qname = "c:CT_NumData/c:numLit",
    qname = "c:CT_StrRef/c:strRef",
    qname = "c:CT_StrData/c:strLit"
  ))]
  pub xml_children: Option<AxisDataSourceTypeChoice>,
}
/// Defines the BarChartSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_BarSer/c15:ser")]
pub struct BarChartSeries {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index>,
  /// Order
  #[sdk(child(qname = "c:CT_UnsignedInt/c:order"))]
  pub order:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Order>,
  /// Series Text
  #[sdk(child(qname = "c:CT_SerTx/c:tx"))]
  pub series_text: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SeriesText>,
  >,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:invertIfNegative"))]
  pub invert_if_negative:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::InvertIfNegative>,
  /// _
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PictureOptions,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataPoint>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataLabels>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_Trendline/c:trendline"))]
  pub c_trendline: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Trendline>,
  /// _
  #[sdk(child(qname = "c:CT_ErrBars/c:errBars"))]
  pub c_err_bars: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ErrorBars>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub c_cat: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::CategoryAxisData,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub c_val: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Values>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_Shape/c:shape"))]
  pub c_shape: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Shape>,
  /// _
  #[sdk(child(qname = "c:CT_BarSerExtensionList/c:extLst"))]
  pub c_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::BarSerExtensionList>,
}
/// Defines the LineChartSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_LineSer/c15:ser")]
pub struct LineChartSeries {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index>,
  /// Order
  #[sdk(child(qname = "c:CT_UnsignedInt/c:order"))]
  pub order:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Order>,
  /// Series Text
  #[sdk(child(qname = "c:CT_SerTx/c:tx"))]
  pub series_text: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SeriesText>,
  >,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_Marker/c:marker"))]
  pub marker: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Marker>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PictureOptions,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataPoint>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataLabels>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_Trendline/c:trendline"))]
  pub c_trendline: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Trendline>,
  /// _
  #[sdk(child(qname = "c:CT_ErrBars/c:errBars"))]
  pub c_err_bars: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ErrorBars>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub c_cat: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::CategoryAxisData,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub c_val: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Values>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:smooth"))]
  pub c_smooth: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Smooth>,
  /// _
  #[sdk(child(qname = "c:CT_LineSerExtensionList/c:extLst"))]
  pub c_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::LineSerExtensionList>,
}
/// Defines the ScatterChartSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_ScatterSer/c15:ser")]
pub struct ScatterChartSeries {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index>,
  /// Order
  #[sdk(child(qname = "c:CT_UnsignedInt/c:order"))]
  pub order:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Order>,
  /// Series Text
  #[sdk(child(qname = "c:CT_SerTx/c:tx"))]
  pub series_text: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SeriesText>,
  >,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_Marker/c:marker"))]
  pub marker: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Marker>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataPoint>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataLabels>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_Trendline/c:trendline"))]
  pub c_trendline: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Trendline>,
  /// _
  #[sdk(child(qname = "c:CT_ErrBars/c:errBars"))]
  pub c_err_bars: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ErrorBars>,
  /// _
  #[sdk(child(qname = "c:CT_AxDataSource/c:xVal"))]
  pub c_x_val: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::XValues>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:yVal"))]
  pub c_y_val: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::YValues>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:smooth"))]
  pub c_smooth: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Smooth>,
  /// _
  #[sdk(child(qname = "c:CT_ScatterSerExtensionList/c:extLst"))]
  pub c_ext_lst: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ScatterSerExtensionList,
  >,
}
/// Defines the AreaChartSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_AreaSer/c15:ser")]
pub struct AreaChartSeries {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index>,
  /// Order
  #[sdk(child(qname = "c:CT_UnsignedInt/c:order"))]
  pub order:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Order>,
  /// Series Text
  #[sdk(child(qname = "c:CT_SerTx/c:tx"))]
  pub series_text: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SeriesText>,
  >,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PictureOptions,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataPoint>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataLabels>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_Trendline/c:trendline"))]
  pub c_trendline: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Trendline>,
  /// _
  #[sdk(child(qname = "c:CT_ErrBars/c:errBars"))]
  pub c_err_bars: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ErrorBars>,
  /// _
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub c_cat: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::CategoryAxisData,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub c_val: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Values>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_AreaSerExtensionList/c:extLst"))]
  pub c_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::AreaSerExtensionList>,
}
/// Defines the PieChartSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_PieSer/c15:ser")]
pub struct PieChartSeries {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index>,
  /// Order
  #[sdk(child(qname = "c:CT_UnsignedInt/c:order"))]
  pub order:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Order>,
  /// Series Text
  #[sdk(child(qname = "c:CT_SerTx/c:tx"))]
  pub series_text: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SeriesText>,
  >,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PictureOptions,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_UnsignedInt/c:explosion"))]
  pub explosion: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Explosion>,
  /// _
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataPoint>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataLabels>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub c_cat: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::CategoryAxisData,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub c_val: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Values>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_PieSerExtensionList/c:extLst"))]
  pub c_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PieSerExtensionList>,
}
/// Defines the BubbleChartSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_BubbleSer/c15:ser")]
pub struct BubbleChartSeries {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index>,
  /// Order
  #[sdk(child(qname = "c:CT_UnsignedInt/c:order"))]
  pub order:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Order>,
  /// Series Text
  #[sdk(child(qname = "c:CT_SerTx/c:tx"))]
  pub series_text: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SeriesText>,
  >,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PictureOptions,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:invertIfNegative"))]
  pub invert_if_negative:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::InvertIfNegative>,
  /// _
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataPoint>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataLabels>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_Trendline/c:trendline"))]
  pub c_trendline: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Trendline>,
  /// _
  #[sdk(child(qname = "c:CT_ErrBars/c:errBars"))]
  pub c_err_bars: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ErrorBars>,
  /// _
  #[sdk(child(qname = "c:CT_AxDataSource/c:xVal"))]
  pub c_x_val: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::XValues>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:yVal"))]
  pub c_y_val: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::YValues>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:bubbleSize"))]
  pub c_bubble_size: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::BubbleSize>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:bubble3D"))]
  pub c_bubble3_d:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Bubble3D>,
  /// _
  #[sdk(child(qname = "c:CT_BubbleSerExtensionList/c:extLst"))]
  pub c_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::BubbleSerExtensionList>,
}
/// Defines the RadarChartSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_RadarSer/c15:ser")]
pub struct RadarChartSeries {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index>,
  /// Order
  #[sdk(child(qname = "c:CT_UnsignedInt/c:order"))]
  pub order:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Order>,
  /// Series Text
  #[sdk(child(qname = "c:CT_SerTx/c:tx"))]
  pub series_text: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SeriesText>,
  >,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PictureOptions,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_Marker/c:marker"))]
  pub marker: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Marker>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_DPt/c:dPt"))]
  pub c_d_pt: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataPoint>,
  /// _
  #[sdk(child(qname = "c:CT_DLbls/c:dLbls"))]
  pub c_d_lbls: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DataLabels>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub c_cat: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::CategoryAxisData,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub c_val: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Values>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_RadarSerExtensionList/c:extLst"))]
  pub c_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::RadarSerExtensionList>,
}
/// Defines the SurfaceChartSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_SurfaceSer/c15:ser")]
pub struct SurfaceChartSeries {
  /// Index
  #[sdk(child(qname = "c:CT_UnsignedInt/c:idx"))]
  pub index:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Index>,
  /// Order
  #[sdk(child(qname = "c:CT_UnsignedInt/c:order"))]
  pub order:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Order>,
  /// Series Text
  #[sdk(child(qname = "c:CT_SerTx/c:tx"))]
  pub series_text: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SeriesText>,
  >,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "a:CT_ChartShapeProperties/c:spPr"))]
  pub chart_shape_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartShapeProperties,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_PictureOptions/c:pictureOptions"))]
  pub picture_options: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::PictureOptions,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_AxDataSource/c:cat"))]
  pub category_axis_data: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::CategoryAxisData,
    >,
  >,
  /// _
  #[sdk(child(qname = "c:CT_NumDataSource/c:val"))]
  pub values: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Values>,
  >,
  /// _
  #[sdk(child(qname = "c:CT_Boolean/c:bubble3D"))]
  pub bubble3_d: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Bubble3D>,
  /// _
  #[sdk(child(qname = "c:CT_SurfaceSerExtensionList/c:extLst"))]
  pub surface_ser_extension_list: Option<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::SurfaceSerExtensionList,
  >,
}
/// Defines the DataLabelsRangeChache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_StrData/c15:dlblRangeCache")]
pub struct DataLabelsRangeChache {
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
/// Defines the DataLabelFieldTableCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_StrData/c15:dlblFieldTableCache")]
pub struct DataLabelFieldTableCache {
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
/// Defines the Explosion Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_UnsignedInt/c15:explosion")]
pub struct Explosion {
  /// Integer Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the Marker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_Marker/c15:marker")]
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
/// Defines the DataLabel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c:CT_DLbl/c15:dLbl")]
pub struct DataLabel {
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
  pub data_label_choice: Option<DataLabelChoice>,
  /// _
  #[sdk(child(qname = "c:CT_DLblExtensionList/c:extLst"))]
  pub c_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::DLblExtensionList>,
}
/// Defines the CategoryFilterException Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "c15:CT_CategoryFilterException/c15:categoryFilterException"
)]
pub struct CategoryFilterException {
  /// _
  #[sdk(text_child(office2013, qname = "xsd:string/c15:sqref"))]
  pub sequence_of_references: crate::simple_type::StringValue,
  /// _
  #[sdk(child(office2013, qname = "a:CT_ShapeProperties/c15:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// _
  #[sdk(child(office2013, qname = "c:CT_UnsignedInt/c15:explosion"))]
  pub explosion: Option<Explosion>,
  /// _
  #[sdk(child(office2013, qname = "c:CT_Boolean/c15:invertIfNegative"))]
  pub invert_if_negative_boolean: Option<InvertIfNegativeBoolean>,
  /// _
  #[sdk(child(office2013, qname = "c:CT_Boolean/c15:bubble3D"))]
  pub bubble3_d: Option<Bubble3D>,
  /// _
  #[sdk(child(office2013, qname = "c:CT_Marker/c15:marker"))]
  pub marker: Option<std::boxed::Box<Marker>>,
  /// _
  #[sdk(child(office2013, qname = "c:CT_DLbl/c15:dLbl"))]
  pub data_label: Option<std::boxed::Box<DataLabel>>,
}
/// Defines the DataLabelFieldTableEntry Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "c15:CT_DataLabelFieldTableEntry/c15:dlblFTEntry")]
pub struct DataLabelFieldTableEntry {
  /// _
  #[sdk(text_child(office2013, qname = "xsd:string/c15:txfldGUID"))]
  pub text_field_guid: crate::simple_type::StringValue,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:string/c15:f"))]
  pub formula: crate::simple_type::StringValue,
  /// _
  #[sdk(child(office2013, qname = "c:CT_StrData/c15:dlblFieldTableCache"))]
  pub data_label_field_table_cache: Option<std::boxed::Box<DataLabelFieldTableCache>>,
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
pub enum ChartTextChoice {
  /// Defines the StringReference Class.
  #[sdk(child(qname = "c:CT_StrRef/c:strRef"))]
  CStrRef(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::StringReference,
    >,
  ),
  /// Rich Text.
  #[sdk(child(qname = "a:CT_TextBody/c:rich"))]
  CRich(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::RichText>),
  /// String Literal.
  #[sdk(child(qname = "c:CT_StrData/c:strLit"))]
  CStrLit(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::StringLiteral>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum AxisDataSourceTypeChoice {
  /// Multi Level String Reference.
  #[sdk(child(qname = "c:CT_MultiLvlStrRef/c:multiLvlStrRef"))]
  CMultiLvlStrRef(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::MultiLevelStringReference,
    >,
  ),
  /// Number Reference.
  #[sdk(child(qname = "c:CT_NumRef/c:numRef"))]
  CNumRef(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::NumberReference,
    >,
  ),
  /// Number Literal.
  #[sdk(child(qname = "c:CT_NumData/c:numLit"))]
  CNumLit(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::NumberLiteral>,
  ),
  /// Defines the StringReference Class.
  #[sdk(child(qname = "c:CT_StrRef/c:strRef"))]
  CStrRef(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::StringReference,
    >,
  ),
  /// String Literal.
  #[sdk(child(qname = "c:CT_StrData/c:strLit"))]
  CStrLit(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::StringLiteral>,
  ),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct DataLabelChoiceSequence {
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
pub enum DataLabelChoice {
  /// Delete.
  #[sdk(child(qname = "c:CT_Boolean/c:delete"))]
  CDelete(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::Delete>),
  /// Sequence of c:layout, c:tx, c:numFmt, c:spPr, c:txPr, c:dLblPos, c:showLegendKey, c:showVal, c:showCatName, c:showSerName, c:showPercent, c:showBubbleSize, c:separator
  #[sdk(sequence)]
  Sequence(std::boxed::Box<DataLabelChoiceSequence>),
}
