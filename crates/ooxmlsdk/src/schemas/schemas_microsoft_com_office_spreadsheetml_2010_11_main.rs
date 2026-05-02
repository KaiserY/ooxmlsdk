//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TimelineStyleType {
  #[sdk(rename = "selectionLabel")]
  #[default]
  SelectionLabel,
  #[sdk(rename = "timeLevel")]
  TimeLevel,
  #[sdk(rename = "periodLabel1")]
  PeriodLabel1,
  #[sdk(rename = "periodLabel2")]
  PeriodLabel2,
  #[sdk(rename = "selectedTimeBlock")]
  SelectedTimeBlock,
  #[sdk(rename = "unselectedTimeBlock")]
  UnselectedTimeBlock,
  #[sdk(rename = "selectedTimeBlockSpace")]
  SelectedTimeBlockSpace,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum CalculatedMemberNumberFormat {
  #[sdk(rename = "default")]
  #[default]
  Default,
  #[sdk(rename = "number")]
  Number,
  #[sdk(rename = "percent")]
  Percent,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SxvCellType {
  #[sdk(rename = "b")]
  #[default]
  Boolean,
  #[sdk(rename = "n")]
  Number,
  #[sdk(rename = "e")]
  Error,
  #[sdk(rename = "str")]
  String,
  #[sdk(rename = "d")]
  Date,
  #[sdk(rename = "bl")]
  Blank,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum MovingPeriodStep {
  #[sdk(rename = "year")]
  #[default]
  Year,
  #[sdk(rename = "quarter")]
  Quarter,
  #[sdk(rename = "month")]
  Month,
  #[sdk(rename = "week")]
  Week,
  #[sdk(rename = "day")]
  Day,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum QuestionType {
  #[sdk(rename = "checkBox")]
  #[default]
  CheckBox,
  #[sdk(rename = "choice")]
  Choice,
  #[sdk(rename = "date")]
  Date,
  #[sdk(rename = "time")]
  Time,
  #[sdk(rename = "multipleLinesOfText")]
  MultipleLinesOfText,
  #[sdk(rename = "number")]
  Number,
  #[sdk(rename = "singleLineOfText")]
  SingleLineOfText,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum QuestionFormat {
  #[sdk(rename = "generalDate")]
  #[default]
  GeneralDate,
  #[sdk(rename = "longDate")]
  LongDate,
  #[sdk(rename = "shortDate")]
  ShortDate,
  #[sdk(rename = "longTime")]
  LongTime,
  #[sdk(rename = "shortTime")]
  ShortTime,
  #[sdk(rename = "generalNumber")]
  GeneralNumber,
  #[sdk(rename = "standard")]
  Standard,
  #[sdk(rename = "fixed")]
  Fixed,
  #[sdk(rename = "percent")]
  Percent,
  #[sdk(rename = "currency")]
  Currency,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SurveyPosition {
  #[sdk(rename = "absolute")]
  #[default]
  Absolute,
  #[sdk(rename = "fixed")]
  Fixed,
  #[sdk(rename = "relative")]
  Relative,
  #[sdk(rename = "static")]
  Static,
  #[sdk(rename = "inherit")]
  Inherit,
}
/// Defines the PivotCaches Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x:CT_PivotCaches/x15:pivotCaches")]
pub struct PivotCaches {
  /// PivotCache.
  #[sdk(child(qname = "x:CT_PivotCache/x:pivotCache"))]
  pub x_pivot_cache: Vec<crate::schemas::x::PivotCache>,
}
/// Defines the TimelineCachePivotCaches Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x:CT_PivotCaches/x15:timelineCachePivotCaches")]
pub struct TimelineCachePivotCaches {
  /// PivotCache.
  #[sdk(child(qname = "x:CT_PivotCache/x:pivotCache"))]
  pub x_pivot_cache: Vec<crate::schemas::x::PivotCache>,
}
/// Defines the PivotTableReferences Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "x15:CT_PivotTableReferences/x15:pivotTableReferences"
)]
pub struct PivotTableReferences {
  /// Defines the PivotTableReference Class.
  #[sdk(child(
    office2013,
    qname = "x15:CT_PivotTableReference/x15:pivotTableReference"
  ))]
  pub x15_pivot_table_reference: Vec<PivotTableReference>,
}
/// Defines the QueryTable Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_QueryTable/x15:queryTable")]
pub struct QueryTable {
  /// clipped
  #[sdk(attr(office2013, qname = ":clipped"))]
  pub clipped: Option<crate::simple_type::BooleanValue>,
  /// sourceDataName
  #[sdk(attr(office2013, qname = ":sourceDataName"))]
  pub source_data_name: Option<crate::simple_type::StringValue>,
  /// drillThrough
  #[sdk(attr(office2013, qname = ":drillThrough"))]
  pub drill_through: Option<crate::simple_type::BooleanValue>,
}
/// Defines the WebExtensions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_WebExtensions/x15:webExtensions")]
pub struct WebExtensions {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the WebExtension Class.
  #[sdk(child(office2013, qname = "x15:CT_WebExtension/x15:webExtension"))]
  pub x15_web_extension: Vec<WebExtension>,
}
/// Defines the TimelineCacheReferences Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_TimelineCacheRefs/x15:timelineCacheRefs")]
pub struct TimelineCacheReferences {
  /// Defines the TimelineCacheReference Class.
  #[sdk(child(office2013, qname = "x15:CT_TimelineCacheRef/x15:timelineCacheRef"))]
  pub x15_timeline_cache_ref: Vec<TimelineCacheReference>,
}
/// Defines the TimelineReferences Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_TimelineRefs/x15:timelineRefs")]
pub struct TimelineReferences {
  /// Defines the TimelineReference Class.
  #[sdk(child(office2013, qname = "x15:CT_TimelineRef/x15:timelineRef"))]
  pub x15_timeline_ref: Vec<TimelineReference>,
}
/// Defines the WorkbookProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_WorkbookPr/x15:workbookPr")]
pub struct WorkbookProperties {
  /// chartTrackingRefBase
  #[sdk(attr(office2013, qname = ":chartTrackingRefBase"))]
  pub chart_tracking_reference_base: Option<crate::simple_type::BooleanValue>,
}
/// Defines the TimelineStyles Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_TimelineStyles/x15:timelineStyles")]
pub struct TimelineStyles {
  /// defaultTimelineStyle
  #[sdk(attr(office2013, qname = ":defaultTimelineStyle"))]
  pub default_timeline_style: crate::simple_type::StringValue,
  /// Defines the TimelineStyle Class.
  #[sdk(child(office2013, qname = "x15:CT_TimelineStyle/x15:timelineStyle"))]
  pub x15_timeline_style: Vec<TimelineStyle>,
}
/// Defines the DifferentialFormats Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x:CT_Dxfs/x15:dxfs")]
pub struct DifferentialFormats {
  /// Format Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Formatting.
  #[sdk(child(qname = "x:CT_Dxf/x:dxf"))]
  pub x_dxf: Vec<crate::schemas::x::DifferentialFormat>,
}
/// Defines the Connection Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_Connection/x15:connection")]
pub struct Connection {
  /// id
  #[sdk(attr(office2013, qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// model
  #[sdk(attr(office2013, qname = ":model"))]
  pub model: Option<crate::simple_type::BooleanValue>,
  /// excludeFromRefreshAll
  #[sdk(attr(office2013, qname = ":excludeFromRefreshAll"))]
  pub exclude_from_refresh_all: Option<crate::simple_type::BooleanValue>,
  /// autoDelete
  #[sdk(attr(office2013, qname = ":autoDelete"))]
  pub auto_delete: Option<crate::simple_type::BooleanValue>,
  /// usedByAddin
  #[sdk(attr(office2013, qname = ":usedByAddin"))]
  pub used_by_addin: Option<crate::simple_type::BooleanValue>,
  /// Defines the TextProperties Class.
  #[sdk(child(office2013, qname = "x:CT_TextPr/x15:textPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
  /// Defines the ModelTextProperties Class.
  #[sdk(child(office2013, qname = "x15:CT_ModelTextPr/x15:modelTextPr"))]
  pub model_text_properties: Option<ModelTextProperties>,
  /// Defines the RangeProperties Class.
  #[sdk(child(office2013, qname = "x15:CT_RangePr/x15:rangePr"))]
  pub range_properties: Option<RangeProperties>,
  /// Defines the OleDbPrpoperties Class.
  #[sdk(child(office2013, qname = "x15:CT_OledbPr/x15:oledbPr"))]
  pub ole_db_prpoperties: Option<std::boxed::Box<OleDbPrpoperties>>,
  /// Defines the DataFeedProperties Class.
  #[sdk(child(office2013, qname = "x15:CT_DataFeedPr/x15:dataFeedPr"))]
  pub data_feed_properties: Option<std::boxed::Box<DataFeedProperties>>,
}
/// Defines the CalculatedMember Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_CalculatedMember/x15:calculatedMember")]
pub struct CalculatedMember {
  /// measureGroup
  #[sdk(attr(office2013, qname = ":measureGroup"))]
  pub measure_group: Option<crate::simple_type::StringValue>,
  /// numberFormat
  #[sdk(attr(office2013, qname = ":numberFormat"))]
  pub number_format: Option<CalculatedMemberNumberFormat>,
  /// measure
  #[sdk(attr(office2013, qname = ":measure"))]
  pub measure: Option<crate::simple_type::BooleanValue>,
}
/// Defines the PivotTableUISettings Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "x15:CT_PivotTableUISettings/x15:pivotTableUISettings"
)]
pub struct PivotTableUiSettings {
  /// sourceDataName
  #[sdk(attr(office2013, qname = ":sourceDataName"))]
  pub source_data_name: Option<crate::simple_type::StringValue>,
  /// relNeededHidden
  #[sdk(attr(office2013, qname = ":relNeededHidden"))]
  pub rel_needed_hidden: Option<crate::simple_type::BooleanValue>,
  /// Defines the FieldListActiveTabTopLevelEntity Class.
  #[sdk(child(
    office2013,
    qname = "x15:CT_FieldListActiveTabTopLevelEntity/x15:activeTabTopLevelEntity"
  ))]
  pub x15_active_tab_top_level_entity: Vec<FieldListActiveTabTopLevelEntity>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2013, qname = "x:CT_ExtensionList/x15:extLst"))]
  pub x15_ext_lst: Option<ExtensionList>,
}
/// Defines the PivotFilter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_PivotFilter/x15:pivotFilter")]
pub struct PivotFilter {
  /// useWholeDay
  #[sdk(attr(office2013, qname = ":useWholeDay"))]
  pub use_whole_day: crate::simple_type::BooleanValue,
}
/// Defines the CachedUniqueNames Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_CachedUniqueNames/x15:cachedUniqueNames")]
pub struct CachedUniqueNames {
  /// Defines the CachedUniqueName Class.
  #[sdk(child(office2013, qname = "x15:CT_CachedUniqueName/x15:cachedUniqueName"))]
  pub x15_cached_unique_name: Vec<CachedUniqueName>,
}
/// Defines the CacheHierarchy Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_CacheHierarchy/x15:cacheHierarchy")]
pub struct CacheHierarchy {
  /// aggregatedColumn
  #[sdk(attr(office2013, qname = ":aggregatedColumn"))]
  pub aggregated_column: crate::simple_type::Int32Value,
}
/// Defines the TimelinePivotCacheDefinition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "x15:CT_TimelinePivotCacheDefinition/x15:timelinePivotCacheDefinition"
)]
pub struct TimelinePivotCacheDefinition {
  /// timelineData
  #[sdk(attr(office2013, qname = ":timelineData"))]
  pub timeline_data: Option<crate::simple_type::BooleanValue>,
}
/// Defines the PivotCacheIdVersion Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "x15:CT_PivotCacheIdVersion/x15:pivotCacheIdVersion"
)]
pub struct PivotCacheIdVersion {
  /// cacheIdSupportedVersion
  #[sdk(attr(office2013, qname = ":cacheIdSupportedVersion"))]
  pub cache_id_supported_version: crate::simple_type::ByteValue,
  /// cacheIdCreatedVersion
  #[sdk(attr(office2013, qname = ":cacheIdCreatedVersion"))]
  pub cache_id_created_version: crate::simple_type::ByteValue,
}
/// Defines the DataModel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_DataModel/x15:dataModel")]
pub struct DataModel {
  /// minVersionLoad
  #[sdk(attr(office2013, qname = ":minVersionLoad"))]
  pub min_version_load: Option<crate::simple_type::ByteValue>,
  /// Defines the ModelTables Class.
  #[sdk(child(office2013, qname = "x15:CT_ModelTables/x15:modelTables"))]
  pub model_tables: Option<ModelTables>,
  /// Defines the ModelRelationships Class.
  #[sdk(child(office2013, qname = "x15:CT_ModelRelationships/x15:modelRelationships"))]
  pub model_relationships: Option<ModelRelationships>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2013, qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotTableData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_PivotTableData/x15:pivotTableData")]
pub struct PivotTableData {
  /// rowCount
  #[sdk(attr(office2013, qname = ":rowCount"))]
  pub row_count: Option<crate::simple_type::UInt32Value>,
  /// columnCount
  #[sdk(attr(office2013, qname = ":columnCount"))]
  pub column_count: Option<crate::simple_type::UInt32Value>,
  /// cacheId
  #[sdk(attr(office2013, qname = ":cacheId"))]
  pub cache_id: crate::simple_type::UInt32Value,
  /// Defines the PivotRow Class.
  #[sdk(child(office2013, qname = "x15:CT_PivotRow/x15:pivotRow"))]
  pub x15_pivot_row: Vec<PivotRow>,
}
/// Defines the PivotCacheDecoupled Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "x15:CT_PivotCacheDecoupled/x15:pivotCacheDecoupled"
)]
pub struct PivotCacheDecoupled {
  /// decoupled
  #[sdk(attr(office2013, qname = ":decoupled"))]
  pub decoupled: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DataField Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_DataField/x15:dataField")]
pub struct DataField {
  /// isCountDistinct
  #[sdk(attr(office2013, qname = ":isCountDistinct"))]
  pub is_count_distinct: Option<crate::simple_type::BooleanValue>,
}
/// Defines the MovingPeriodState Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_MovingPeriodState/x15:movingPeriodState")]
pub struct MovingPeriodState {
  /// referenceDateBegin
  #[sdk(attr(office2013, qname = ":referenceDateBegin"))]
  pub reference_date_begin: crate::simple_type::DateTimeValue,
  /// referencePeriod
  #[sdk(attr(office2013, qname = ":referencePeriod"))]
  pub reference_period: MovingPeriodStep,
  /// referenceMultiple
  #[sdk(attr(office2013, qname = ":referenceMultiple"))]
  pub reference_multiple: crate::simple_type::UInt32Value,
  /// movingPeriod
  #[sdk(attr(office2013, qname = ":movingPeriod"))]
  pub moving_period: MovingPeriodStep,
  /// movingMultiple
  #[sdk(attr(office2013, qname = ":movingMultiple"))]
  pub moving_multiple: crate::simple_type::UInt32Value,
}
/// Defines the SlicerCaches Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x14:CT_SlicerCaches/x15:slicerCaches")]
pub struct SlicerCaches {
  /// Defines the SlicerCache Class.
  #[sdk(child(office2010, qname = "x14:CT_SlicerCache/x14:slicerCache"))]
  pub x14_slicer_cache: Vec<crate::schemas::x14::SlicerCache>,
}
/// Defines the TableSlicerCache Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_TableSlicerCache/x15:tableSlicerCache")]
pub struct TableSlicerCache {
  /// tableId
  #[sdk(attr(office2013, qname = ":tableId"))]
  pub table_id: crate::simple_type::UInt32Value,
  /// column
  #[sdk(attr(office2013, qname = ":column"))]
  pub column: crate::simple_type::UInt32Value,
  /// sortOrder
  #[sdk(attr(office2010, qname = ":sortOrder"))]
  pub sort_order: Option<crate::schemas::x14::TabularSlicerCacheSortOrderValues>,
  /// customListSort
  #[sdk(attr(office2013, qname = ":customListSort"))]
  pub custom_list_sort: Option<crate::simple_type::BooleanValue>,
  /// crossFilter
  #[sdk(attr(office2010, qname = ":crossFilter"))]
  pub cross_filter: Option<crate::schemas::x14::SlicerCacheCrossFilterValues>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2013, qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the SlicerCacheHideItemsWithNoData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "x15:CT_SlicerCacheHideNoData/x15:slicerCacheHideItemsWithNoData"
)]
pub struct SlicerCacheHideItemsWithNoData {
  /// count
  #[sdk(attr(office2013, qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Defines the SlicerCacheOlapLevelName Class.
  #[sdk(child(
    office2013,
    qname = "x15:CT_SlicerCacheOlapLevelName/x15:slicerCacheOlapLevelName"
  ))]
  pub x15_slicer_cache_olap_level_name: Vec<SlicerCacheOlapLevelName>,
}
/// Defines the SlicerCachePivotTables Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "x14:CT_SlicerCachePivotTables/x15:slicerCachePivotTables"
)]
pub struct SlicerCachePivotTables {
  /// Defines the SlicerCachePivotTable Class.
  #[sdk(child(office2010, qname = "x14:CT_SlicerCachePivotTable/x14:pivotTable"))]
  pub x14_pivot_table: Vec<crate::schemas::x14::SlicerCachePivotTable>,
}
/// Defines the Survey Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_Survey/x15:survey")]
pub struct Survey {
  /// id
  #[sdk(attr(office2013, qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// guid
  #[sdk(attr(office2013, qname = ":guid"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// title
  #[sdk(attr(office2013, qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// description
  #[sdk(attr(office2013, qname = ":description"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// Defines the SurveyPrSurveyElementPr Class.
  #[sdk(child(office2013, qname = "x15:CT_SurveyElementPr/x15:surveyPr"))]
  pub survey_pr_survey_element_pr: Option<std::boxed::Box<SurveyPrSurveyElementPr>>,
  /// Defines the TitlePrSurveyElementPr Class.
  #[sdk(child(office2013, qname = "x15:CT_SurveyElementPr/x15:titlePr"))]
  pub title_pr_survey_element_pr: Option<std::boxed::Box<TitlePrSurveyElementPr>>,
  /// Defines the DescriptionPrSurveyElementPr Class.
  #[sdk(child(office2013, qname = "x15:CT_SurveyElementPr/x15:descriptionPr"))]
  pub description_pr_survey_element_pr: Option<std::boxed::Box<DescriptionPrSurveyElementPr>>,
  /// Defines the SurveyQuestions Class.
  #[sdk(child(office2013, qname = "x15:CT_SurveyQuestions/x15:questions"))]
  pub survey_questions: std::boxed::Box<SurveyQuestions>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2013, qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Timelines Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_Timelines/x15:timelines")]
pub struct Timelines {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Defines the Timeline Class.
  #[sdk(child(office2013, qname = "x15:CT_Timeline/x15:timeline"))]
  pub x15_timeline: Vec<Timeline>,
}
/// Defines the TimelineCacheDefinition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "x15:CT_TimelineCacheDefinition/x15:timelineCacheDefinition"
)]
pub struct TimelineCacheDefinition {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// name
  #[sdk(attr(office2013, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// sourceName
  #[sdk(attr(office2013, qname = ":sourceName"))]
  pub source_name: crate::simple_type::StringValue,
  /// Defines the TimelineCachePivotTables Class.
  #[sdk(child(office2013, qname = "x15:CT_TimelineCachePivotTables/x15:pivotTables"))]
  pub timeline_cache_pivot_tables: Option<TimelineCachePivotTables>,
  /// Defines the TimelineState Class.
  #[sdk(child(office2013, qname = "x15:CT_TimelineState/x15:state"))]
  pub timeline_state: std::boxed::Box<TimelineState>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2013, qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotTableReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "x15:CT_PivotTableReference/x15:pivotTableReference"
)]
pub struct PivotTableReference {
  /// id
  #[sdk(attr(office2013, qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the WebExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_WebExtension/x15:webExtension")]
pub struct WebExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// appRef
  #[sdk(attr(office2013, qname = ":appRef"))]
  pub application_reference: crate::simple_type::StringValue,
  /// Defines the Formula Class.
  #[sdk(text_child(office2010, qname = "x:ST_Formula/xne:f"))]
  pub formula: crate::simple_type::StringValue,
}
/// Defines the TimelineCacheReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_TimelineCacheRef/x15:timelineCacheRef")]
pub struct TimelineCacheReference {
  /// id
  #[sdk(attr(office2013, qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the TimelineReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_TimelineRef/x15:timelineRef")]
pub struct TimelineReference {
  /// id
  #[sdk(attr(office2013, qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the TimelineStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_TimelineStyle/x15:timelineStyle")]
pub struct TimelineStyle {
  /// name
  #[sdk(attr(office2013, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Defines the TimelineStyleElements Class.
  #[sdk(child(
    office2013,
    qname = "x15:CT_TimelineStyleElements/x15:timelineStyleElements"
  ))]
  pub timeline_style_elements: Option<TimelineStyleElements>,
}
/// Defines the TimelineStyleElement Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "x15:CT_TimelineStyleElement/x15:timelineStyleElement"
)]
pub struct TimelineStyleElement {
  /// type
  #[sdk(attr(office2013, qname = ":type"))]
  pub r#type: TimelineStyleType,
  /// dxfId
  #[sdk(attr(office2013, qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
}
/// Defines the TimelineStyleElements Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "x15:CT_TimelineStyleElements/x15:timelineStyleElements"
)]
pub struct TimelineStyleElements {
  /// Defines the TimelineStyleElement Class.
  #[sdk(child(
    office2013,
    qname = "x15:CT_TimelineStyleElement/x15:timelineStyleElement"
  ))]
  pub x15_timeline_style_element: Vec<TimelineStyleElement>,
}
/// Defines the DbTable Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_DbTable/x15:dbTable")]
pub struct DbTable {
  /// name
  #[sdk(attr(office2013, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// Defines the DbTables Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_DbTables/x15:dbTables")]
pub struct DbTables {
  /// Defines the DbTable Class.
  #[sdk(child(office2013, qname = "x15:CT_DbTable/x15:dbTable"))]
  pub x15_db_table: Vec<DbTable>,
}
/// Defines the DbCommand Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_DbCommand/x15:dbCommand")]
pub struct DbCommand {
  /// text
  #[sdk(attr(office2013, qname = ":text"))]
  pub text: crate::simple_type::StringValue,
}
/// Defines the TextProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x:CT_TextPr/x15:textPr")]
pub struct TextProperties {
  /// prompt
  #[sdk(attr(qname = ":prompt"))]
  pub prompt: Option<crate::simple_type::BooleanValue>,
  /// fileType
  #[sdk(attr(qname = ":fileType"))]
  pub file_type: Option<crate::schemas::x::FileTypeValues>,
  /// codePage
  #[sdk(attr(qname = ":codePage"))]
  pub code_page: Option<crate::simple_type::UInt32Value>,
  /// characterSet
  #[sdk(attr(qname = ":characterSet"))]
  pub text_character_set: Option<crate::simple_type::StringValue>,
  /// firstRow
  #[sdk(attr(qname = ":firstRow"))]
  pub first_row: Option<crate::simple_type::UInt32Value>,
  /// sourceFile
  #[sdk(attr(qname = ":sourceFile"))]
  pub source_file: Option<crate::simple_type::StringValue>,
  /// delimited
  #[sdk(attr(qname = ":delimited"))]
  pub delimited: Option<crate::simple_type::BooleanValue>,
  /// decimal
  #[sdk(attr(qname = ":decimal"))]
  pub decimal: Option<crate::simple_type::StringValue>,
  /// thousands
  #[sdk(attr(qname = ":thousands"))]
  pub thousands: Option<crate::simple_type::StringValue>,
  /// tab
  #[sdk(attr(qname = ":tab"))]
  pub tab_as_delimiter: Option<crate::simple_type::BooleanValue>,
  /// space
  #[sdk(attr(qname = ":space"))]
  pub space: Option<crate::simple_type::BooleanValue>,
  /// comma
  #[sdk(attr(qname = ":comma"))]
  pub comma: Option<crate::simple_type::BooleanValue>,
  /// semicolon
  #[sdk(attr(qname = ":semicolon"))]
  pub semicolon: Option<crate::simple_type::BooleanValue>,
  /// consecutive
  #[sdk(attr(qname = ":consecutive"))]
  pub consecutive: Option<crate::simple_type::BooleanValue>,
  /// qualifier
  #[sdk(attr(qname = ":qualifier"))]
  pub qualifier: Option<crate::schemas::x::QualifierValues>,
  /// delimiter
  #[sdk(attr(qname = ":delimiter"))]
  pub delimiter: Option<crate::simple_type::StringValue>,
  /// Defines the TextFields Class.
  #[sdk(child(qname = "x:CT_TextFields/x:textFields"))]
  pub text_fields: Option<crate::schemas::x::TextFields>,
}
/// Defines the ModelTextProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_ModelTextPr/x15:modelTextPr")]
pub struct ModelTextProperties {
  /// headers
  #[sdk(attr(office2013, qname = ":headers"))]
  pub headers: Option<crate::simple_type::BooleanValue>,
}
/// Defines the RangeProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_RangePr/x15:rangePr")]
pub struct RangeProperties {
  /// sourceName
  #[sdk(attr(office2013, qname = ":sourceName"))]
  pub source_name: crate::simple_type::StringValue,
}
/// Defines the OleDbPrpoperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_OledbPr/x15:oledbPr")]
pub struct OleDbPrpoperties {
  /// connection
  #[sdk(attr(office2013, qname = ":connection"))]
  pub connection: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "x15:CT_DbTables/x15:dbTables",
    qname = "x15:CT_DbCommand/x15:dbCommand"
  ))]
  pub ole_db_prpoperties_choice: Option<OleDbPrpopertiesChoice>,
}
/// Defines the DataFeedProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_DataFeedPr/x15:dataFeedPr")]
pub struct DataFeedProperties {
  /// connection
  #[sdk(attr(office2013, qname = ":connection"))]
  pub connection: crate::simple_type::StringValue,
  /// Defines the DbTables Class.
  #[sdk(child(office2013, qname = "x15:CT_DbTables/x15:dbTables"))]
  pub db_tables: std::boxed::Box<DbTables>,
}
/// Defines the FieldListActiveTabTopLevelEntity Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "x15:CT_FieldListActiveTabTopLevelEntity/x15:activeTabTopLevelEntity"
)]
pub struct FieldListActiveTabTopLevelEntity {
  /// name
  #[sdk(attr(office2013, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// type
  #[sdk(attr(office2013, qname = ":type"))]
  pub r#type: Option<crate::simple_type::UInt32Value>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x:CT_ExtensionList/x15:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub x_ext: Vec<crate::schemas::x::Extension>,
}
/// Defines the CachedUniqueName Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_CachedUniqueName/x15:cachedUniqueName")]
pub struct CachedUniqueName {
  /// index
  #[sdk(attr(office2013, qname = ":index"))]
  pub index: crate::simple_type::UInt32Value,
  /// name
  #[sdk(attr(office2013, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// Defines the ModelTable Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_ModelTable/x15:modelTable")]
pub struct ModelTable {
  /// id
  #[sdk(attr(office2013, qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// name
  #[sdk(attr(office2013, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// connection
  #[sdk(attr(office2013, qname = ":connection"))]
  pub connection: crate::simple_type::StringValue,
}
/// Defines the ModelRelationship Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_ModelRelationship/x15:modelRelationship")]
pub struct ModelRelationship {
  /// fromTable
  #[sdk(attr(office2013, qname = ":fromTable"))]
  pub from_table: crate::simple_type::StringValue,
  /// fromColumn
  #[sdk(attr(office2013, qname = ":fromColumn"))]
  pub from_column: crate::simple_type::StringValue,
  /// toTable
  #[sdk(attr(office2013, qname = ":toTable"))]
  pub to_table: crate::simple_type::StringValue,
  /// toColumn
  #[sdk(attr(office2013, qname = ":toColumn"))]
  pub to_column: crate::simple_type::StringValue,
}
/// Defines the ModelTables Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_ModelTables/x15:modelTables")]
pub struct ModelTables {
  /// Defines the ModelTable Class.
  #[sdk(child(office2013, qname = "x15:CT_ModelTable/x15:modelTable"))]
  pub x15_model_table: Vec<ModelTable>,
}
/// Defines the ModelRelationships Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_ModelRelationships/x15:modelRelationships")]
pub struct ModelRelationships {
  /// Defines the ModelRelationship Class.
  #[sdk(child(office2013, qname = "x15:CT_ModelRelationship/x15:modelRelationship"))]
  pub x15_model_relationship: Vec<ModelRelationship>,
}
/// Defines the PivotValueCell Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_PivotValueCell/x15:c")]
pub struct PivotValueCell {
  /// i
  #[sdk(attr(office2013, qname = ":i"))]
  pub item: Option<crate::simple_type::UInt32Value>,
  /// t
  #[sdk(attr(office2013, qname = ":t"))]
  pub text: Option<SxvCellType>,
  /// Defines the Xstring Class.
  #[sdk(text_child(office2013, qname = "x:ST_Xstring/x15:v"))]
  pub xstring: crate::simple_type::StringValue,
  /// Defines the PivotValueCellExtra Class.
  #[sdk(child(office2013, qname = "x15:CT_PivotValueCellExtra/x15:x"))]
  pub pivot_value_cell_extra: Option<PivotValueCellExtra>,
}
/// Defines the Xstring Class.
pub type Xstring = crate::simple_type::StringValue;
/// Defines the PivotValueCellExtra Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_PivotValueCellExtra/x15:x")]
pub struct PivotValueCellExtra {
  /// in
  #[sdk(attr(office2013, qname = ":in"))]
  pub format_index: Option<crate::simple_type::UInt32Value>,
  /// bc
  #[sdk(attr(office2013, qname = ":bc"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub background_color: Option<crate::simple_type::HexBinaryValue>,
  /// fc
  #[sdk(attr(office2013, qname = ":fc"))]
  #[sdk(string_length(min = 4u32, max = 4u32))]
  pub foreground_color: Option<crate::simple_type::HexBinaryValue>,
  /// i
  #[sdk(attr(office2013, qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// un
  #[sdk(attr(office2013, qname = ":un"))]
  pub underline: Option<crate::simple_type::BooleanValue>,
  /// st
  #[sdk(attr(office2013, qname = ":st"))]
  pub strikethrough: Option<crate::simple_type::BooleanValue>,
  /// b
  #[sdk(attr(office2013, qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
}
/// Defines the PivotTableServerFormats Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "x15:CT_PivotTableServerFormats/x15:pivotTableServerFormats"
)]
pub struct PivotTableServerFormats {
  /// count
  #[sdk(attr(office2013, qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// Defines the ServerFormat Class.
  #[sdk(child(office2013, qname = "x:CT_ServerFormat/x15:serverFormat"))]
  pub x15_server_format: Vec<ServerFormat>,
}
/// Defines the ServerFormat Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x:CT_ServerFormat/x15:serverFormat")]
pub struct ServerFormat {
  /// Culture
  #[sdk(attr(qname = ":culture"))]
  pub culture: Option<crate::simple_type::StringValue>,
  /// Format
  #[sdk(attr(qname = ":format"))]
  pub format: Option<crate::simple_type::StringValue>,
}
/// Defines the SlicerCacheOlapLevelName Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "x15:CT_SlicerCacheOlapLevelName/x15:slicerCacheOlapLevelName"
)]
pub struct SlicerCacheOlapLevelName {
  /// uniqueName
  #[sdk(attr(office2013, qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// count
  #[sdk(attr(office2013, qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
}
/// Defines the SurveyPrSurveyElementPr Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_SurveyElementPr/x15:surveyPr")]
pub struct SurveyPrSurveyElementPr {
  /// cssClass
  #[sdk(attr(office2013, qname = ":cssClass"))]
  pub css_class: Option<crate::simple_type::StringValue>,
  /// bottom
  #[sdk(attr(office2013, qname = ":bottom"))]
  pub bottom: Option<crate::simple_type::Int32Value>,
  /// top
  #[sdk(attr(office2013, qname = ":top"))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// left
  #[sdk(attr(office2013, qname = ":left"))]
  pub left: Option<crate::simple_type::Int32Value>,
  /// right
  #[sdk(attr(office2013, qname = ":right"))]
  pub right: Option<crate::simple_type::Int32Value>,
  /// width
  #[sdk(attr(office2013, qname = ":width"))]
  pub width: Option<crate::simple_type::UInt32Value>,
  /// height
  #[sdk(attr(office2013, qname = ":height"))]
  pub height: Option<crate::simple_type::UInt32Value>,
  /// position
  #[sdk(attr(office2013, qname = ":position"))]
  pub position: Option<SurveyPosition>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2013, qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TitlePrSurveyElementPr Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_SurveyElementPr/x15:titlePr")]
pub struct TitlePrSurveyElementPr {
  /// cssClass
  #[sdk(attr(office2013, qname = ":cssClass"))]
  pub css_class: Option<crate::simple_type::StringValue>,
  /// bottom
  #[sdk(attr(office2013, qname = ":bottom"))]
  pub bottom: Option<crate::simple_type::Int32Value>,
  /// top
  #[sdk(attr(office2013, qname = ":top"))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// left
  #[sdk(attr(office2013, qname = ":left"))]
  pub left: Option<crate::simple_type::Int32Value>,
  /// right
  #[sdk(attr(office2013, qname = ":right"))]
  pub right: Option<crate::simple_type::Int32Value>,
  /// width
  #[sdk(attr(office2013, qname = ":width"))]
  pub width: Option<crate::simple_type::UInt32Value>,
  /// height
  #[sdk(attr(office2013, qname = ":height"))]
  pub height: Option<crate::simple_type::UInt32Value>,
  /// position
  #[sdk(attr(office2013, qname = ":position"))]
  pub position: Option<SurveyPosition>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2013, qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the DescriptionPrSurveyElementPr Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_SurveyElementPr/x15:descriptionPr")]
pub struct DescriptionPrSurveyElementPr {
  /// cssClass
  #[sdk(attr(office2013, qname = ":cssClass"))]
  pub css_class: Option<crate::simple_type::StringValue>,
  /// bottom
  #[sdk(attr(office2013, qname = ":bottom"))]
  pub bottom: Option<crate::simple_type::Int32Value>,
  /// top
  #[sdk(attr(office2013, qname = ":top"))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// left
  #[sdk(attr(office2013, qname = ":left"))]
  pub left: Option<crate::simple_type::Int32Value>,
  /// right
  #[sdk(attr(office2013, qname = ":right"))]
  pub right: Option<crate::simple_type::Int32Value>,
  /// width
  #[sdk(attr(office2013, qname = ":width"))]
  pub width: Option<crate::simple_type::UInt32Value>,
  /// height
  #[sdk(attr(office2013, qname = ":height"))]
  pub height: Option<crate::simple_type::UInt32Value>,
  /// position
  #[sdk(attr(office2013, qname = ":position"))]
  pub position: Option<SurveyPosition>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2013, qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the QuestionsPrSurveyElementPr Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_SurveyElementPr/x15:questionsPr")]
pub struct QuestionsPrSurveyElementPr {
  /// cssClass
  #[sdk(attr(office2013, qname = ":cssClass"))]
  pub css_class: Option<crate::simple_type::StringValue>,
  /// bottom
  #[sdk(attr(office2013, qname = ":bottom"))]
  pub bottom: Option<crate::simple_type::Int32Value>,
  /// top
  #[sdk(attr(office2013, qname = ":top"))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// left
  #[sdk(attr(office2013, qname = ":left"))]
  pub left: Option<crate::simple_type::Int32Value>,
  /// right
  #[sdk(attr(office2013, qname = ":right"))]
  pub right: Option<crate::simple_type::Int32Value>,
  /// width
  #[sdk(attr(office2013, qname = ":width"))]
  pub width: Option<crate::simple_type::UInt32Value>,
  /// height
  #[sdk(attr(office2013, qname = ":height"))]
  pub height: Option<crate::simple_type::UInt32Value>,
  /// position
  #[sdk(attr(office2013, qname = ":position"))]
  pub position: Option<SurveyPosition>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2013, qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the QuestionPrSurveyElementPr Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_SurveyElementPr/x15:questionPr")]
pub struct QuestionPrSurveyElementPr {
  /// cssClass
  #[sdk(attr(office2013, qname = ":cssClass"))]
  pub css_class: Option<crate::simple_type::StringValue>,
  /// bottom
  #[sdk(attr(office2013, qname = ":bottom"))]
  pub bottom: Option<crate::simple_type::Int32Value>,
  /// top
  #[sdk(attr(office2013, qname = ":top"))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// left
  #[sdk(attr(office2013, qname = ":left"))]
  pub left: Option<crate::simple_type::Int32Value>,
  /// right
  #[sdk(attr(office2013, qname = ":right"))]
  pub right: Option<crate::simple_type::Int32Value>,
  /// width
  #[sdk(attr(office2013, qname = ":width"))]
  pub width: Option<crate::simple_type::UInt32Value>,
  /// height
  #[sdk(attr(office2013, qname = ":height"))]
  pub height: Option<crate::simple_type::UInt32Value>,
  /// position
  #[sdk(attr(office2013, qname = ":position"))]
  pub position: Option<SurveyPosition>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2013, qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the SurveyQuestions Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_SurveyQuestions/x15:questions")]
pub struct SurveyQuestions {
  /// Defines the QuestionsPrSurveyElementPr Class.
  #[sdk(child(office2013, qname = "x15:CT_SurveyElementPr/x15:questionsPr"))]
  pub questions_pr_survey_element_pr: Option<std::boxed::Box<QuestionsPrSurveyElementPr>>,
  /// Defines the SurveyQuestion Class.
  #[sdk(child(office2013, qname = "x15:CT_SurveyQuestion/x15:question"))]
  pub x15_question: Vec<SurveyQuestion>,
}
/// Defines the SurveyQuestion Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_SurveyQuestion/x15:question")]
pub struct SurveyQuestion {
  /// binding
  #[sdk(attr(office2013, qname = ":binding"))]
  pub binding: crate::simple_type::UInt32Value,
  /// text
  #[sdk(attr(office2013, qname = ":text"))]
  pub text: Option<crate::simple_type::StringValue>,
  /// type
  #[sdk(attr(office2013, qname = ":type"))]
  pub r#type: Option<QuestionType>,
  /// format
  #[sdk(attr(office2013, qname = ":format"))]
  pub format: Option<QuestionFormat>,
  /// helpText
  #[sdk(attr(office2013, qname = ":helpText"))]
  pub help_text: Option<crate::simple_type::StringValue>,
  /// required
  #[sdk(attr(office2013, qname = ":required"))]
  pub required: Option<crate::simple_type::BooleanValue>,
  /// defaultValue
  #[sdk(attr(office2013, qname = ":defaultValue"))]
  pub default_value: Option<crate::simple_type::StringValue>,
  /// decimalPlaces
  #[sdk(attr(office2013, qname = ":decimalPlaces"))]
  pub decimal_places: Option<crate::simple_type::UInt32Value>,
  /// rowSource
  #[sdk(attr(office2013, qname = ":rowSource"))]
  pub row_source: Option<crate::simple_type::StringValue>,
  /// Defines the QuestionPrSurveyElementPr Class.
  #[sdk(child(office2013, qname = "x15:CT_SurveyElementPr/x15:questionPr"))]
  pub question_pr_survey_element_pr: Option<std::boxed::Box<QuestionPrSurveyElementPr>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2013, qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Timeline Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_Timeline/x15:timeline")]
pub struct Timeline {
  /// name
  #[sdk(attr(office2013, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// cache
  #[sdk(attr(office2013, qname = ":cache"))]
  pub cache: crate::simple_type::StringValue,
  /// caption
  #[sdk(attr(office2013, qname = ":caption"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// showHeader
  #[sdk(attr(office2013, qname = ":showHeader"))]
  pub show_header: Option<crate::simple_type::BooleanValue>,
  /// showSelectionLabel
  #[sdk(attr(office2013, qname = ":showSelectionLabel"))]
  pub show_selection_label: Option<crate::simple_type::BooleanValue>,
  /// showTimeLevel
  #[sdk(attr(office2013, qname = ":showTimeLevel"))]
  pub show_time_level: Option<crate::simple_type::BooleanValue>,
  /// showHorizontalScrollbar
  #[sdk(attr(office2013, qname = ":showHorizontalScrollbar"))]
  pub show_horizontal_scrollbar: Option<crate::simple_type::BooleanValue>,
  /// level
  #[sdk(attr(office2013, qname = ":level"))]
  pub level: crate::simple_type::UInt32Value,
  /// selectionLevel
  #[sdk(attr(office2013, qname = ":selectionLevel"))]
  pub selection_level: crate::simple_type::UInt32Value,
  /// scrollPosition
  #[sdk(attr(office2013, qname = ":scrollPosition"))]
  pub scroll_position: Option<crate::simple_type::DateTimeValue>,
  /// style
  #[sdk(attr(office2013, qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2013, qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TimelineCachePivotTable Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_TimelineCachePivotTable/x15:pivotTable")]
pub struct TimelineCachePivotTable {
  /// tabId
  #[sdk(attr(office2013, qname = ":tabId"))]
  pub tab_id: crate::simple_type::UInt32Value,
  /// name
  #[sdk(attr(office2013, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// Defines the SelectionTimelineRange Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_TimelineRange/x15:selection")]
pub struct SelectionTimelineRange {
  /// startDate
  #[sdk(attr(office2013, qname = ":startDate"))]
  pub start_date: crate::simple_type::DateTimeValue,
  /// endDate
  #[sdk(attr(office2013, qname = ":endDate"))]
  pub end_date: crate::simple_type::DateTimeValue,
}
/// Defines the BoundsTimelineRange Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_TimelineRange/x15:bounds")]
pub struct BoundsTimelineRange {
  /// startDate
  #[sdk(attr(office2013, qname = ":startDate"))]
  pub start_date: crate::simple_type::DateTimeValue,
  /// endDate
  #[sdk(attr(office2013, qname = ":endDate"))]
  pub end_date: crate::simple_type::DateTimeValue,
}
/// Defines the AutoFilter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x:CT_AutoFilter/x15:autoFilter")]
pub struct AutoFilter {
  /// Cell or Range Reference
  #[sdk(attr(qname = ":ref"))]
  pub reference: Option<crate::simple_type::StringValue>,
  /// AutoFilter Column.
  #[sdk(child(qname = "x:CT_FilterColumn/x:filterColumn"))]
  pub x_filter_column: Vec<crate::schemas::x::FilterColumn>,
  /// Sort State for Auto Filter.
  #[sdk(child(qname = "x:CT_SortState/x:sortState"))]
  pub x_sort_state: Option<std::boxed::Box<crate::schemas::x::SortState>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst: Option<crate::schemas::x::ExtensionList>,
}
/// Defines the TimelineCachePivotTables Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_TimelineCachePivotTables/x15:pivotTables")]
pub struct TimelineCachePivotTables {
  /// Defines the TimelineCachePivotTable Class.
  #[sdk(child(office2013, qname = "x15:CT_TimelineCachePivotTable/x15:pivotTable"))]
  pub x15_pivot_table: Vec<TimelineCachePivotTable>,
}
/// Defines the TimelineState Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_TimelineState/x15:state")]
pub struct TimelineState {
  /// singleRangeFilterState
  #[sdk(attr(office2013, qname = ":singleRangeFilterState"))]
  pub single_range_filter_state: Option<crate::simple_type::BooleanValue>,
  /// minimalRefreshVersion
  #[sdk(attr(office2013, qname = ":minimalRefreshVersion"))]
  pub minimal_refresh_version: crate::simple_type::UInt32Value,
  /// lastRefreshVersion
  #[sdk(attr(office2013, qname = ":lastRefreshVersion"))]
  pub last_refresh_version: crate::simple_type::UInt32Value,
  /// pivotCacheId
  #[sdk(attr(office2013, qname = ":pivotCacheId"))]
  pub pivot_cache_id: crate::simple_type::UInt32Value,
  /// filterType
  #[sdk(attr(office2013, qname = ":filterType"))]
  pub filter_type: crate::schemas::x::PivotFilterValues,
  /// filterId
  #[sdk(attr(office2013, qname = ":filterId"))]
  pub filter_id: Option<crate::simple_type::UInt32Value>,
  /// filterTabId
  #[sdk(attr(office2013, qname = ":filterTabId"))]
  pub filter_tab_id: Option<crate::simple_type::UInt32Value>,
  /// filterPivotName
  #[sdk(attr(office2013, qname = ":filterPivotName"))]
  pub filter_pivot_name: Option<crate::simple_type::StringValue>,
  /// Defines the SelectionTimelineRange Class.
  #[sdk(child(office2013, qname = "x15:CT_TimelineRange/x15:selection"))]
  pub selection_timeline_range: Option<SelectionTimelineRange>,
  /// Defines the BoundsTimelineRange Class.
  #[sdk(child(office2013, qname = "x15:CT_TimelineRange/x15:bounds"))]
  pub bounds_timeline_range: std::boxed::Box<BoundsTimelineRange>,
  /// Defines the MovingPeriodState Class.
  #[sdk(child(office2013, qname = "x15:CT_MovingPeriodState/x15:movingPeriodState"))]
  pub moving_period_state: Option<MovingPeriodState>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2013, qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotRow Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "x15:CT_PivotRow/x15:pivotRow")]
pub struct PivotRow {
  /// r
  #[sdk(attr(office2013, qname = ":r"))]
  pub reference: Option<crate::simple_type::UInt32Value>,
  /// count
  #[sdk(attr(office2013, qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// Defines the PivotValueCell Class.
  #[sdk(child(office2013, qname = "x15:CT_PivotValueCell/x15:c"))]
  pub x15_c: Vec<PivotValueCell>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OleDbPrpopertiesChoice {
  /// Defines the DbTables Class.
  #[sdk(child(office2013, qname = "x15:CT_DbTables/x15:dbTables"))]
  X15DbTables(std::boxed::Box<DbTables>),
  /// Defines the DbCommand Class.
  #[sdk(child(office2013, qname = "x15:CT_DbCommand/x15:dbCommand"))]
  X15DbCommand(std::boxed::Box<DbCommand>),
}
