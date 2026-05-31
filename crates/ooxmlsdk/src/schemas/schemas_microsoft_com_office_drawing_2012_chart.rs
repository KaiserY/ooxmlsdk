//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PivotSource Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:pivotSource")]
pub struct PivotSource {
  /// Pivot Name
  #[sdk(text_child(simple_type = "StringValue", qname = "c:name"))]
  pub pivot_table_name: crate::schemas::c::PivotTableName,
  /// Format ID
  #[sdk(child(qname = "c:fmtId"))]
  pub format_id: std::boxed::Box<crate::schemas::c::FormatId>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:extLst"))]
  pub extension_list: Option<crate::schemas::c::ExtensionList>,
}
/// Defines the NumberingFormat Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:numFmt")]
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
#[sdk(qname = "c15:spPr")]
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
/// Defines the Layout Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:layout")]
pub struct Layout {
  /// Manual Layout
  #[sdk(child(qname = "c:manualLayout"))]
  pub manual_layout: Option<std::boxed::Box<crate::schemas::c::ManualLayout>>,
  /// Chart Extensibility
  #[sdk(child(qname = "c:extLst"))]
  pub extension_list: Option<crate::schemas::c::ExtensionList>,
}
/// Defines the FullReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:fullRef")]
pub struct FullReference {
  /// Defines the SequenceOfReferences Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "c15:sqref"))]
  pub sequence_of_references: SequenceOfReferences,
}
/// Defines the LevelReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:levelRef")]
pub struct LevelReference {
  /// Defines the SequenceOfReferences Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "c15:sqref"))]
  pub sequence_of_references: SequenceOfReferences,
}
/// Defines the FormulaReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:formulaRef")]
pub struct FormulaReference {
  /// Defines the SequenceOfReferences Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "c15:sqref"))]
  pub sequence_of_references: SequenceOfReferences,
}
/// Defines the FilteredSeriesTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:filteredSeriesTitle")]
pub struct FilteredSeriesTitle {
  /// Defines the ChartText Class.
  #[sdk(child(qname = "c15:tx"))]
  pub chart_text: std::boxed::Box<ChartText>,
}
/// Defines the FilteredCategoryTitle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:filteredCategoryTitle")]
pub struct FilteredCategoryTitle {
  /// Defines the AxisDataSourceType Class.
  #[sdk(child(qname = "c15:cat"))]
  pub axis_data_source_type: std::boxed::Box<AxisDataSourceType>,
}
/// Defines the FilteredAreaSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:filteredAreaSeries")]
pub struct FilteredAreaSeries {
  /// Defines the AreaChartSeries Class.
  #[sdk(child(qname = "c15:ser"))]
  pub area_chart_series: std::boxed::Box<AreaChartSeries>,
}
/// Defines the FilteredBarSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:filteredBarSeries")]
pub struct FilteredBarSeries {
  /// Defines the BarChartSeries Class.
  #[sdk(child(qname = "c15:ser"))]
  pub bar_chart_series: std::boxed::Box<BarChartSeries>,
}
/// Defines the FilteredBubbleSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:filteredBubbleSeries")]
pub struct FilteredBubbleSeries {
  /// Defines the BubbleChartSeries Class.
  #[sdk(child(qname = "c15:ser"))]
  pub bubble_chart_series: std::boxed::Box<BubbleChartSeries>,
}
/// Defines the FilteredLineSeriesExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:filteredLineSeries")]
pub struct FilteredLineSeriesExtension {
  /// Defines the LineChartSeries Class.
  #[sdk(child(qname = "c15:ser"))]
  pub line_chart_series: std::boxed::Box<LineChartSeries>,
}
/// Defines the FilteredPieSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:filteredPieSeries")]
pub struct FilteredPieSeries {
  /// Defines the PieChartSeries Class.
  #[sdk(child(qname = "c15:ser"))]
  pub pie_chart_series: std::boxed::Box<PieChartSeries>,
}
/// Defines the FilteredRadarSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:filteredRadarSeries")]
pub struct FilteredRadarSeries {
  /// Defines the RadarChartSeries Class.
  #[sdk(child(qname = "c15:ser"))]
  pub radar_chart_series: std::boxed::Box<RadarChartSeries>,
}
/// Defines the FilteredScatterSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:filteredScatterSeries")]
pub struct FilteredScatterSeries {
  /// Defines the ScatterChartSeries Class.
  #[sdk(child(qname = "c15:ser"))]
  pub scatter_chart_series: std::boxed::Box<ScatterChartSeries>,
}
/// Defines the FilteredSurfaceSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:filteredSurfaceSeries")]
pub struct FilteredSurfaceSeries {
  /// Defines the SurfaceChartSeries Class.
  #[sdk(child(qname = "c15:ser"))]
  pub surface_chart_series: std::boxed::Box<SurfaceChartSeries>,
}
/// Defines the DataLabelsRange Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:datalabelsRange")]
pub struct DataLabelsRange {
  /// Defines the Formula Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "c15:f"))]
  pub formula: Formula,
  /// Defines the DataLabelsRangeChache Class.
  #[sdk(child(qname = "c15:dlblRangeCache"))]
  pub data_labels_range_chache: Option<std::boxed::Box<DataLabelsRangeChache>>,
}
/// Defines the CategoryFilterExceptions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:categoryFilterExceptions")]
pub struct CategoryFilterExceptions {
  /// Defines the CategoryFilterException Class.
  #[sdk(child(qname = "c15:categoryFilterException"))]
  pub category_filter_exception: Vec<CategoryFilterException>,
}
/// Defines the DataLabelFieldTable Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:dlblFieldTable")]
pub struct DataLabelFieldTable {
  /// Defines the DataLabelFieldTableEntry Class.
  #[sdk(child(qname = "c15:dlblFTEntry"))]
  pub data_label_field_table_entry: Vec<DataLabelFieldTableEntry>,
}
/// Defines the ExceptionForSave Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:xForSave")]
pub struct ExceptionForSave {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ShowDataLabelsRange Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:showDataLabelsRange")]
pub struct ShowDataLabelsRange {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ShowLeaderLines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:showLeaderLines")]
pub struct ShowLeaderLines {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the AutoGeneneratedCategories Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:autoCat")]
pub struct AutoGeneneratedCategories {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the InvertIfNegativeBoolean Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:invertIfNegative")]
pub struct InvertIfNegativeBoolean {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the Bubble3D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:bubble3D")]
pub struct Bubble3D {
  /// Boolean Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ChartText Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:tx")]
pub struct ChartText {
  #[sdk(
        choice(
            child(variant = StringReference, qname = "c:strRef"),
            child(variant = RichText, qname = "c:rich"),
            child(variant = StringLiteral, qname = "c:strLit")
        )
    )]
  pub chart_text_choice: Option<ChartTextChoice>,
}
/// Defines the LeaderLines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:leaderLines")]
pub struct LeaderLines {
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<crate::schemas::c::ChartShapeProperties>>,
}
/// Defines the SequenceOfReferences Class.
pub type SequenceOfReferences = crate::simple_type::StringValue;
/// Defines the Formula Class.
pub type Formula = crate::simple_type::StringValue;
/// Defines the TextFieldGuid Class.
pub type TextFieldGuid = crate::simple_type::StringValue;
/// Defines the AxisDataSourceType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:cat")]
pub struct AxisDataSourceType {
  #[sdk(
        choice(
            child(variant = MultiLevelStringReference, qname = "c:multiLvlStrRef"),
            child(variant = NumberReference, qname = "c:numRef"),
            child(variant = NumberLiteral, qname = "c:numLit"),
            child(variant = StringReference, qname = "c:strRef"),
            child(variant = StringLiteral, qname = "c:strLit")
        )
    )]
  pub axis_data_source_type_choice: Option<AxisDataSourceTypeChoice>,
}
/// Defines the BarChartSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:ser")]
pub struct BarChartSeries {
  /// Index
  #[sdk(child(qname = "c:idx"))]
  pub index: std::boxed::Box<crate::schemas::c::Index>,
  /// Order
  #[sdk(child(qname = "c:order"))]
  pub order: std::boxed::Box<crate::schemas::c::Order>,
  /// Series Text
  #[sdk(child(qname = "c:tx"))]
  pub series_text: Option<std::boxed::Box<crate::schemas::c::SeriesText>>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<crate::schemas::c::ChartShapeProperties>>,
  /// Invert if Negative.
  #[sdk(child(qname = "c:invertIfNegative"))]
  pub invert_if_negative: Option<crate::schemas::c::InvertIfNegative>,
  /// Defines the PictureOptions Class.
  #[sdk(child(qname = "c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<crate::schemas::c::PictureOptions>>,
  /// Defines the DataPoint Class.
  #[sdk(child(qname = "c:dPt"))]
  pub data_point: Vec<crate::schemas::c::DataPoint>,
  /// Data Labels.
  #[sdk(child(qname = "c:dLbls"))]
  pub data_labels: Option<std::boxed::Box<crate::schemas::c::DataLabels>>,
  /// Defines the Trendline Class.
  #[sdk(child(qname = "c:trendline"))]
  pub trendline: Vec<crate::schemas::c::Trendline>,
  /// Defines the ErrorBars Class.
  #[sdk(child(qname = "c:errBars"))]
  pub error_bars: Option<std::boxed::Box<crate::schemas::c::ErrorBars>>,
  /// Defines the CategoryAxisData Class.
  #[sdk(child(qname = "c:cat"))]
  pub category_axis_data: Option<std::boxed::Box<crate::schemas::c::CategoryAxisData>>,
  /// Defines the Values Class.
  #[sdk(child(qname = "c:val"))]
  pub values: Option<std::boxed::Box<crate::schemas::c::Values>>,
  /// Defines the Shape Class.
  #[sdk(child(qname = "c:shape"))]
  pub shape: Option<crate::schemas::c::Shape>,
  /// Defines the BarSerExtensionList Class.
  #[sdk(child(qname = "c:extLst"))]
  pub bar_ser_extension_list: Option<crate::schemas::c::BarSerExtensionList>,
}
/// Defines the LineChartSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:ser")]
pub struct LineChartSeries {
  /// Index
  #[sdk(child(qname = "c:idx"))]
  pub index: std::boxed::Box<crate::schemas::c::Index>,
  /// Order
  #[sdk(child(qname = "c:order"))]
  pub order: std::boxed::Box<crate::schemas::c::Order>,
  /// Series Text
  #[sdk(child(qname = "c:tx"))]
  pub series_text: Option<std::boxed::Box<crate::schemas::c::SeriesText>>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<crate::schemas::c::ChartShapeProperties>>,
  /// Marker.
  #[sdk(child(qname = "c:marker"))]
  pub marker: Option<std::boxed::Box<crate::schemas::c::Marker>>,
  /// Defines the PictureOptions Class.
  #[sdk(child(qname = "c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<crate::schemas::c::PictureOptions>>,
  /// Defines the DataPoint Class.
  #[sdk(child(qname = "c:dPt"))]
  pub data_point: Vec<crate::schemas::c::DataPoint>,
  /// Data Labels.
  #[sdk(child(qname = "c:dLbls"))]
  pub data_labels: Option<std::boxed::Box<crate::schemas::c::DataLabels>>,
  /// Defines the Trendline Class.
  #[sdk(child(qname = "c:trendline"))]
  pub trendline: Vec<crate::schemas::c::Trendline>,
  /// Defines the ErrorBars Class.
  #[sdk(child(qname = "c:errBars"))]
  pub error_bars: Option<std::boxed::Box<crate::schemas::c::ErrorBars>>,
  /// Defines the CategoryAxisData Class.
  #[sdk(child(qname = "c:cat"))]
  pub category_axis_data: Option<std::boxed::Box<crate::schemas::c::CategoryAxisData>>,
  /// Defines the Values Class.
  #[sdk(child(qname = "c:val"))]
  pub values: Option<std::boxed::Box<crate::schemas::c::Values>>,
  /// Defines the Smooth Class.
  #[sdk(child(qname = "c:smooth"))]
  pub smooth: Option<crate::schemas::c::Smooth>,
  /// Defines the LineSerExtensionList Class.
  #[sdk(child(qname = "c:extLst"))]
  pub line_ser_extension_list: Option<crate::schemas::c::LineSerExtensionList>,
}
/// Defines the ScatterChartSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:ser")]
pub struct ScatterChartSeries {
  /// Index
  #[sdk(child(qname = "c:idx"))]
  pub index: std::boxed::Box<crate::schemas::c::Index>,
  /// Order
  #[sdk(child(qname = "c:order"))]
  pub order: std::boxed::Box<crate::schemas::c::Order>,
  /// Series Text
  #[sdk(child(qname = "c:tx"))]
  pub series_text: Option<std::boxed::Box<crate::schemas::c::SeriesText>>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<crate::schemas::c::ChartShapeProperties>>,
  /// Marker.
  #[sdk(child(qname = "c:marker"))]
  pub marker: Option<std::boxed::Box<crate::schemas::c::Marker>>,
  /// Defines the DataPoint Class.
  #[sdk(child(qname = "c:dPt"))]
  pub data_point: Vec<crate::schemas::c::DataPoint>,
  /// Data Labels.
  #[sdk(child(qname = "c:dLbls"))]
  pub data_labels: Option<std::boxed::Box<crate::schemas::c::DataLabels>>,
  /// Defines the Trendline Class.
  #[sdk(child(qname = "c:trendline"))]
  pub trendline: Vec<crate::schemas::c::Trendline>,
  /// Defines the ErrorBars Class.
  #[sdk(child(qname = "c:errBars"))]
  pub error_bars: Vec<crate::schemas::c::ErrorBars>,
  /// Defines the XValues Class.
  #[sdk(child(qname = "c:xVal"))]
  pub x_values: Option<std::boxed::Box<crate::schemas::c::XValues>>,
  /// Defines the YValues Class.
  #[sdk(child(qname = "c:yVal"))]
  pub y_values: Option<std::boxed::Box<crate::schemas::c::YValues>>,
  /// Defines the Smooth Class.
  #[sdk(child(qname = "c:smooth"))]
  pub smooth: Option<crate::schemas::c::Smooth>,
  /// Defines the ScatterSerExtensionList Class.
  #[sdk(child(qname = "c:extLst"))]
  pub scatter_ser_extension_list: Option<crate::schemas::c::ScatterSerExtensionList>,
}
/// Defines the AreaChartSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:ser")]
pub struct AreaChartSeries {
  /// Index
  #[sdk(child(qname = "c:idx"))]
  pub index: std::boxed::Box<crate::schemas::c::Index>,
  /// Order
  #[sdk(child(qname = "c:order"))]
  pub order: std::boxed::Box<crate::schemas::c::Order>,
  /// Series Text
  #[sdk(child(qname = "c:tx"))]
  pub series_text: Option<std::boxed::Box<crate::schemas::c::SeriesText>>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<crate::schemas::c::ChartShapeProperties>>,
  /// Defines the PictureOptions Class.
  #[sdk(child(qname = "c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<crate::schemas::c::PictureOptions>>,
  /// Defines the DataPoint Class.
  #[sdk(child(qname = "c:dPt"))]
  pub data_point: Vec<crate::schemas::c::DataPoint>,
  /// Data Labels.
  #[sdk(child(qname = "c:dLbls"))]
  pub data_labels: Option<std::boxed::Box<crate::schemas::c::DataLabels>>,
  /// Defines the Trendline Class.
  #[sdk(child(qname = "c:trendline"))]
  pub trendline: Vec<crate::schemas::c::Trendline>,
  /// Defines the ErrorBars Class.
  #[sdk(child(qname = "c:errBars"))]
  pub error_bars: Vec<crate::schemas::c::ErrorBars>,
  /// Defines the CategoryAxisData Class.
  #[sdk(child(qname = "c:cat"))]
  pub category_axis_data: Option<std::boxed::Box<crate::schemas::c::CategoryAxisData>>,
  /// Defines the Values Class.
  #[sdk(child(qname = "c:val"))]
  pub values: Option<std::boxed::Box<crate::schemas::c::Values>>,
  /// Defines the AreaSerExtensionList Class.
  #[sdk(child(qname = "c:extLst"))]
  pub area_ser_extension_list: Option<crate::schemas::c::AreaSerExtensionList>,
}
/// Defines the PieChartSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:ser")]
pub struct PieChartSeries {
  /// Index
  #[sdk(child(qname = "c:idx"))]
  pub index: std::boxed::Box<crate::schemas::c::Index>,
  /// Order
  #[sdk(child(qname = "c:order"))]
  pub order: std::boxed::Box<crate::schemas::c::Order>,
  /// Series Text
  #[sdk(child(qname = "c:tx"))]
  pub series_text: Option<std::boxed::Box<crate::schemas::c::SeriesText>>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<crate::schemas::c::ChartShapeProperties>>,
  /// Defines the PictureOptions Class.
  #[sdk(child(qname = "c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<crate::schemas::c::PictureOptions>>,
  /// Explosion.
  #[sdk(child(qname = "c:explosion"))]
  pub explosion: Option<crate::schemas::c::Explosion>,
  /// Defines the DataPoint Class.
  #[sdk(child(qname = "c:dPt"))]
  pub data_point: Vec<crate::schemas::c::DataPoint>,
  /// Data Labels.
  #[sdk(child(qname = "c:dLbls"))]
  pub data_labels: Option<std::boxed::Box<crate::schemas::c::DataLabels>>,
  /// Defines the CategoryAxisData Class.
  #[sdk(child(qname = "c:cat"))]
  pub category_axis_data: Option<std::boxed::Box<crate::schemas::c::CategoryAxisData>>,
  /// Defines the Values Class.
  #[sdk(child(qname = "c:val"))]
  pub values: Option<std::boxed::Box<crate::schemas::c::Values>>,
  /// Defines the PieSerExtensionList Class.
  #[sdk(child(qname = "c:extLst"))]
  pub pie_ser_extension_list: Option<crate::schemas::c::PieSerExtensionList>,
}
/// Defines the BubbleChartSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:ser")]
pub struct BubbleChartSeries {
  /// Index
  #[sdk(child(qname = "c:idx"))]
  pub index: std::boxed::Box<crate::schemas::c::Index>,
  /// Order
  #[sdk(child(qname = "c:order"))]
  pub order: std::boxed::Box<crate::schemas::c::Order>,
  /// Series Text
  #[sdk(child(qname = "c:tx"))]
  pub series_text: Option<std::boxed::Box<crate::schemas::c::SeriesText>>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<crate::schemas::c::ChartShapeProperties>>,
  /// Defines the PictureOptions Class.
  #[sdk(child(qname = "c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<crate::schemas::c::PictureOptions>>,
  /// Invert if Negative.
  #[sdk(child(qname = "c:invertIfNegative"))]
  pub invert_if_negative: Option<crate::schemas::c::InvertIfNegative>,
  /// Defines the DataPoint Class.
  #[sdk(child(qname = "c:dPt"))]
  pub data_point: Vec<crate::schemas::c::DataPoint>,
  /// Data Labels.
  #[sdk(child(qname = "c:dLbls"))]
  pub data_labels: Option<std::boxed::Box<crate::schemas::c::DataLabels>>,
  /// Defines the Trendline Class.
  #[sdk(child(qname = "c:trendline"))]
  pub trendline: Vec<crate::schemas::c::Trendline>,
  /// Defines the ErrorBars Class.
  #[sdk(child(qname = "c:errBars"))]
  pub error_bars: Vec<crate::schemas::c::ErrorBars>,
  /// Defines the XValues Class.
  #[sdk(child(qname = "c:xVal"))]
  pub x_values: Option<std::boxed::Box<crate::schemas::c::XValues>>,
  /// Defines the YValues Class.
  #[sdk(child(qname = "c:yVal"))]
  pub y_values: Option<std::boxed::Box<crate::schemas::c::YValues>>,
  /// Defines the BubbleSize Class.
  #[sdk(child(qname = "c:bubbleSize"))]
  pub bubble_size: Option<std::boxed::Box<crate::schemas::c::BubbleSize>>,
  /// 3D Bubble.
  #[sdk(child(qname = "c:bubble3D"))]
  pub bubble3_d: Option<crate::schemas::c::Bubble3D>,
  /// Defines the BubbleSerExtensionList Class.
  #[sdk(child(qname = "c:extLst"))]
  pub bubble_ser_extension_list: Option<crate::schemas::c::BubbleSerExtensionList>,
}
/// Defines the RadarChartSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:ser")]
pub struct RadarChartSeries {
  /// Index
  #[sdk(child(qname = "c:idx"))]
  pub index: std::boxed::Box<crate::schemas::c::Index>,
  /// Order
  #[sdk(child(qname = "c:order"))]
  pub order: std::boxed::Box<crate::schemas::c::Order>,
  /// Series Text
  #[sdk(child(qname = "c:tx"))]
  pub series_text: Option<std::boxed::Box<crate::schemas::c::SeriesText>>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<crate::schemas::c::ChartShapeProperties>>,
  /// Defines the PictureOptions Class.
  #[sdk(child(qname = "c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<crate::schemas::c::PictureOptions>>,
  /// Marker.
  #[sdk(child(qname = "c:marker"))]
  pub marker: Option<std::boxed::Box<crate::schemas::c::Marker>>,
  /// Defines the DataPoint Class.
  #[sdk(child(qname = "c:dPt"))]
  pub data_point: Vec<crate::schemas::c::DataPoint>,
  /// Data Labels.
  #[sdk(child(qname = "c:dLbls"))]
  pub data_labels: Option<std::boxed::Box<crate::schemas::c::DataLabels>>,
  /// Defines the CategoryAxisData Class.
  #[sdk(child(qname = "c:cat"))]
  pub category_axis_data: Option<std::boxed::Box<crate::schemas::c::CategoryAxisData>>,
  /// Defines the Values Class.
  #[sdk(child(qname = "c:val"))]
  pub values: Option<std::boxed::Box<crate::schemas::c::Values>>,
  /// Defines the RadarSerExtensionList Class.
  #[sdk(child(qname = "c:extLst"))]
  pub radar_ser_extension_list: Option<crate::schemas::c::RadarSerExtensionList>,
}
/// Defines the SurfaceChartSeries Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:ser")]
pub struct SurfaceChartSeries {
  /// Index
  #[sdk(child(qname = "c:idx"))]
  pub index: std::boxed::Box<crate::schemas::c::Index>,
  /// Order
  #[sdk(child(qname = "c:order"))]
  pub order: std::boxed::Box<crate::schemas::c::Order>,
  /// Series Text
  #[sdk(child(qname = "c:tx"))]
  pub series_text: Option<std::boxed::Box<crate::schemas::c::SeriesText>>,
  /// Defines the ChartShapeProperties Class.
  #[sdk(child(qname = "c:spPr"))]
  pub chart_shape_properties: Option<std::boxed::Box<crate::schemas::c::ChartShapeProperties>>,
  /// Defines the PictureOptions Class.
  #[sdk(child(qname = "c:pictureOptions"))]
  pub picture_options: Option<std::boxed::Box<crate::schemas::c::PictureOptions>>,
  /// Defines the CategoryAxisData Class.
  #[sdk(child(qname = "c:cat"))]
  pub category_axis_data: Option<std::boxed::Box<crate::schemas::c::CategoryAxisData>>,
  /// Defines the Values Class.
  #[sdk(child(qname = "c:val"))]
  pub values: Option<std::boxed::Box<crate::schemas::c::Values>>,
  /// 3D Bubble.
  #[sdk(child(qname = "c:bubble3D"))]
  pub bubble3_d: Option<crate::schemas::c::Bubble3D>,
  /// Defines the SurfaceSerExtensionList Class.
  #[sdk(child(qname = "c:extLst"))]
  pub surface_ser_extension_list: Option<crate::schemas::c::SurfaceSerExtensionList>,
}
/// Defines the DataLabelsRangeChache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:dlblRangeCache")]
pub struct DataLabelsRangeChache {
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
/// Defines the DataLabelFieldTableCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:dlblFieldTableCache")]
pub struct DataLabelFieldTableCache {
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
/// Defines the Explosion Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:explosion")]
pub struct Explosion {
  /// Integer Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the Marker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:marker")]
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
/// Defines the DataLabel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:dLbl")]
pub struct DataLabel {
  /// Index.
  #[sdk(child(qname = "c:idx"))]
  pub index: std::boxed::Box<crate::schemas::c::Index>,
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
  pub data_label_choice: Option<DataLabelChoice>,
  /// Defines the DLblExtensionList Class.
  #[sdk(child(qname = "c:extLst"))]
  pub d_lbl_extension_list: Option<crate::schemas::c::DLblExtensionList>,
}
/// Defines the CategoryFilterException Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:categoryFilterException")]
pub struct CategoryFilterException {
  /// Defines the SequenceOfReferences Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "c15:sqref"))]
  pub sequence_of_references: SequenceOfReferences,
  /// Defines the ShapeProperties Class.
  #[sdk(child(qname = "c15:spPr"))]
  pub shape_properties: Option<std::boxed::Box<ShapeProperties>>,
  /// Defines the Explosion Class.
  #[sdk(child(qname = "c15:explosion"))]
  pub explosion: Option<Explosion>,
  /// Defines the InvertIfNegativeBoolean Class.
  #[sdk(child(qname = "c15:invertIfNegative"))]
  pub invert_if_negative_boolean: Option<InvertIfNegativeBoolean>,
  /// Defines the Bubble3D Class.
  #[sdk(child(qname = "c15:bubble3D"))]
  pub bubble3_d: Option<Bubble3D>,
  /// Defines the Marker Class.
  #[sdk(child(qname = "c15:marker"))]
  pub marker: Option<std::boxed::Box<Marker>>,
  /// Defines the DataLabel Class.
  #[sdk(child(qname = "c15:dLbl"))]
  pub data_label: Option<std::boxed::Box<DataLabel>>,
}
/// Defines the DataLabelFieldTableEntry Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c15:dlblFTEntry")]
pub struct DataLabelFieldTableEntry {
  /// Defines the TextFieldGuid Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "c15:txfldGUID"))]
  pub text_field_guid: TextFieldGuid,
  /// Defines the Formula Class.
  #[sdk(text_child(simple_type = "StringValue", qname = "c15:f"))]
  pub formula: Formula,
  /// Defines the DataLabelFieldTableCache Class.
  #[sdk(child(qname = "c15:dlblFieldTableCache"))]
  pub data_label_field_table_cache: Option<std::boxed::Box<DataLabelFieldTableCache>>,
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
pub enum ChartTextChoice {
  /// Defines the StringReference Class.
  StringReference(std::boxed::Box<crate::schemas::c::StringReference>),
  /// Rich Text.
  RichText(std::boxed::Box<crate::schemas::c::RichText>),
  /// String Literal.
  StringLiteral(std::boxed::Box<crate::schemas::c::StringLiteral>),
}
#[derive(Clone, Debug, PartialEq)]
pub enum AxisDataSourceTypeChoice {
  /// Multi Level String Reference.
  MultiLevelStringReference(std::boxed::Box<crate::schemas::c::MultiLevelStringReference>),
  /// Number Reference.
  NumberReference(std::boxed::Box<crate::schemas::c::NumberReference>),
  /// Number Literal.
  NumberLiteral(std::boxed::Box<crate::schemas::c::NumberLiteral>),
  /// Defines the StringReference Class.
  StringReference(std::boxed::Box<crate::schemas::c::StringReference>),
  /// String Literal.
  StringLiteral(std::boxed::Box<crate::schemas::c::StringLiteral>),
}
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
pub struct DataLabelChoiceSequence {
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
  #[sdk(text_child(simple_type = "StringValue", qname = "c:separator"))]
  pub separator: Option<crate::schemas::c::Separator>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum DataLabelChoice {
  Delete(std::boxed::Box<crate::schemas::c::Delete>),
  /// Sequence of c:layout, c:tx, c:numFmt, c:spPr, c:txPr, c:dLblPos, c:showLegendKey, c:showVal, c:showCatName, c:showSerName, c:showPercent, c:showBubbleSize, c:separator
  Sequence(std::boxed::Box<DataLabelChoiceSequence>),
}
