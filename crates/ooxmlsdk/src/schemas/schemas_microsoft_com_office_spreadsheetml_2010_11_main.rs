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
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:pivotCaches.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotCaches/x15:pivotCaches")]
pub struct PivotCaches {
  /// _
  #[sdk(child(qname = "x:CT_PivotCache/x:pivotCache"))]
  pub x_pivot_cache:
    Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCache>,
}
/// Defines the TimelineCachePivotCaches Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:timelineCachePivotCaches.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotCaches/x15:timelineCachePivotCaches")]
pub struct TimelineCachePivotCaches {
  /// _
  #[sdk(child(qname = "x:CT_PivotCache/x:pivotCache"))]
  pub x_pivot_cache:
    Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCache>,
}
/// Defines the OpenXmlPivotCachesElement Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_PivotCaches/")]
pub struct OpenXmlPivotCachesElement {
  /// PivotCache.
  #[sdk(child(qname = "x:CT_PivotCache/x:pivotCache"))]
  pub pivot_cache:
    Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCache>,
}
/// Defines the PivotTableReferences Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:pivotTableReferences.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_PivotTableReferences/x15:pivotTableReferences")]
pub struct PivotTableReferences {
  /// _
  #[sdk(child(qname = "x15:CT_PivotTableReference/x15:pivotTableReference"))]
  pub x15_pivot_table_reference: Vec<PivotTableReference>,
}
/// Defines the QueryTable Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:queryTable.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_QueryTable/x15:queryTable")]
pub struct QueryTable {
  /// clipped
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :clipped
  #[sdk(attr(qname = ":clipped"))]
  pub clipped: Option<crate::simple_type::BooleanValue>,
  /// sourceDataName
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :sourceDataName
  #[sdk(attr(qname = ":sourceDataName"))]
  pub source_data_name: Option<crate::simple_type::StringValue>,
  /// drillThrough
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :drillThrough
  #[sdk(attr(qname = ":drillThrough"))]
  pub drill_through: Option<crate::simple_type::BooleanValue>,
}
/// Defines the WebExtensions Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:webExtensions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_WebExtensions/x15:webExtensions")]
pub struct WebExtensions {
  /// _
  #[sdk(child(qname = "x15:CT_WebExtension/x15:webExtension"))]
  pub x15_web_extension: Vec<WebExtension>,
}
/// Defines the TimelineCacheReferences Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:timelineCacheRefs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_TimelineCacheRefs/x15:timelineCacheRefs")]
pub struct TimelineCacheReferences {
  /// _
  #[sdk(child(qname = "x15:CT_TimelineCacheRef/x15:timelineCacheRef"))]
  pub x15_timeline_cache_ref: Vec<TimelineCacheReference>,
}
/// Defines the TimelineReferences Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:timelineRefs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_TimelineRefs/x15:timelineRefs")]
pub struct TimelineReferences {
  /// _
  #[sdk(child(qname = "x15:CT_TimelineRef/x15:timelineRef"))]
  pub x15_timeline_ref: Vec<TimelineReference>,
}
/// Defines the WorkbookProperties Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:workbookPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_WorkbookPr/x15:workbookPr")]
pub struct WorkbookProperties {
  /// chartTrackingRefBase
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :chartTrackingRefBase
  #[sdk(attr(qname = ":chartTrackingRefBase"))]
  pub chart_tracking_reference_base: Option<crate::simple_type::BooleanValue>,
}
/// Defines the TimelineStyles Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:timelineStyles.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_TimelineStyles/x15:timelineStyles")]
pub struct TimelineStyles {
  /// defaultTimelineStyle
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :defaultTimelineStyle
  #[sdk(attr(qname = ":defaultTimelineStyle"))]
  pub default_timeline_style: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x15:CT_TimelineStyle/x15:timelineStyle"))]
  pub x15_timeline_style: Vec<TimelineStyle>,
}
/// Defines the DifferentialFormats Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:dxfs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dxfs/x15:dxfs")]
pub struct DifferentialFormats {
  /// Format Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x:CT_Dxf/x:dxf"))]
  pub x_dxf:
    Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DifferentialFormat>,
}
/// Defines the Connection Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:connection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_Connection/x15:connection")]
pub struct Connection {
  /// id
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// model
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :model
  #[sdk(attr(qname = ":model"))]
  pub model: Option<crate::simple_type::BooleanValue>,
  /// excludeFromRefreshAll
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :excludeFromRefreshAll
  #[sdk(attr(qname = ":excludeFromRefreshAll"))]
  pub exclude_from_refresh_all: Option<crate::simple_type::BooleanValue>,
  /// autoDelete
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :autoDelete
  #[sdk(attr(qname = ":autoDelete"))]
  pub auto_delete: Option<crate::simple_type::BooleanValue>,
  /// usedByAddin
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :usedByAddin
  #[sdk(attr(qname = ":usedByAddin"))]
  pub used_by_addin: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x:CT_TextPr/x15:textPr"))]
  pub text_properties: Option<std::boxed::Box<TextProperties>>,
  /// _
  #[sdk(child(qname = "x15:CT_ModelTextPr/x15:modelTextPr"))]
  pub model_text_properties: Option<ModelTextProperties>,
  /// _
  #[sdk(child(qname = "x15:CT_RangePr/x15:rangePr"))]
  pub range_properties: Option<RangeProperties>,
  /// _
  #[sdk(child(qname = "x15:CT_OledbPr/x15:oledbPr"))]
  pub ole_db_prpoperties: Option<std::boxed::Box<OleDbPrpoperties>>,
  /// _
  #[sdk(child(qname = "x15:CT_DataFeedPr/x15:dataFeedPr"))]
  pub data_feed_properties: Option<std::boxed::Box<DataFeedProperties>>,
}
/// Defines the CalculatedMember Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:calculatedMember.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_CalculatedMember/x15:calculatedMember")]
pub struct CalculatedMember {
  /// measureGroup
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :measureGroup
  #[sdk(attr(qname = ":measureGroup"))]
  pub measure_group: Option<crate::simple_type::StringValue>,
  /// numberFormat
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :numberFormat
  #[sdk(attr(qname = ":numberFormat"))]
  pub number_format: Option<CalculatedMemberNumberFormat>,
  /// measure
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :measure
  #[sdk(attr(qname = ":measure"))]
  pub measure: Option<crate::simple_type::BooleanValue>,
}
/// Defines the PivotTableUISettings Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:pivotTableUISettings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_PivotTableUISettings/x15:pivotTableUISettings")]
pub struct PivotTableUiSettings {
  /// sourceDataName
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :sourceDataName
  #[sdk(attr(qname = ":sourceDataName"))]
  pub source_data_name: Option<crate::simple_type::StringValue>,
  /// relNeededHidden
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :relNeededHidden
  #[sdk(attr(qname = ":relNeededHidden"))]
  pub rel_needed_hidden: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "x15:CT_FieldListActiveTabTopLevelEntity/x15:activeTabTopLevelEntity"))]
  pub x15_active_tab_top_level_entity: Vec<FieldListActiveTabTopLevelEntity>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x15:extLst"))]
  pub x15_ext_lst: Option<ExtensionList>,
}
/// Defines the PivotFilter Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:pivotFilter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_PivotFilter/x15:pivotFilter")]
pub struct PivotFilter {
  /// useWholeDay
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :useWholeDay
  #[sdk(attr(qname = ":useWholeDay"))]
  pub use_whole_day: crate::simple_type::BooleanValue,
}
/// Defines the CachedUniqueNames Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:cachedUniqueNames.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_CachedUniqueNames/x15:cachedUniqueNames")]
pub struct CachedUniqueNames {
  /// _
  #[sdk(child(qname = "x15:CT_CachedUniqueName/x15:cachedUniqueName"))]
  pub x15_cached_unique_name: Vec<CachedUniqueName>,
}
/// Defines the CacheHierarchy Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:cacheHierarchy.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_CacheHierarchy/x15:cacheHierarchy")]
pub struct CacheHierarchy {
  /// aggregatedColumn
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :aggregatedColumn
  #[sdk(attr(qname = ":aggregatedColumn"))]
  pub aggregated_column: crate::simple_type::Int32Value,
}
/// Defines the TimelinePivotCacheDefinition Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:timelinePivotCacheDefinition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_TimelinePivotCacheDefinition/x15:timelinePivotCacheDefinition")]
pub struct TimelinePivotCacheDefinition {
  /// timelineData
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :timelineData
  #[sdk(attr(qname = ":timelineData"))]
  pub timeline_data: Option<crate::simple_type::BooleanValue>,
}
/// Defines the PivotCacheIdVersion Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:pivotCacheIdVersion.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_PivotCacheIdVersion/x15:pivotCacheIdVersion")]
pub struct PivotCacheIdVersion {
  /// cacheIdSupportedVersion
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :cacheIdSupportedVersion
  #[sdk(attr(qname = ":cacheIdSupportedVersion"))]
  pub cache_id_supported_version: crate::simple_type::ByteValue,
  /// cacheIdCreatedVersion
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :cacheIdCreatedVersion
  #[sdk(attr(qname = ":cacheIdCreatedVersion"))]
  pub cache_id_created_version: crate::simple_type::ByteValue,
}
/// Defines the DataModel Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:dataModel.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_DataModel/x15:dataModel")]
pub struct DataModel {
  /// minVersionLoad
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :minVersionLoad
  #[sdk(attr(qname = ":minVersionLoad"))]
  pub min_version_load: Option<crate::simple_type::ByteValue>,
  /// _
  #[sdk(child(qname = "x15:CT_ModelTables/x15:modelTables"))]
  pub model_tables: Option<ModelTables>,
  /// _
  #[sdk(child(qname = "x15:CT_ModelRelationships/x15:modelRelationships"))]
  pub model_relationships: Option<ModelRelationships>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotTableData Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:pivotTableData.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_PivotTableData/x15:pivotTableData")]
pub struct PivotTableData {
  /// rowCount
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :rowCount
  #[sdk(attr(qname = ":rowCount"))]
  pub row_count: Option<crate::simple_type::UInt32Value>,
  /// columnCount
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :columnCount
  #[sdk(attr(qname = ":columnCount"))]
  pub column_count: Option<crate::simple_type::UInt32Value>,
  /// cacheId
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :cacheId
  #[sdk(attr(qname = ":cacheId"))]
  pub cache_id: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "x15:CT_PivotRow/x15:pivotRow"))]
  pub x15_pivot_row: Vec<PivotRow>,
}
/// Defines the PivotCacheDecoupled Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:pivotCacheDecoupled.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_PivotCacheDecoupled/x15:pivotCacheDecoupled")]
pub struct PivotCacheDecoupled {
  /// decoupled
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :decoupled
  #[sdk(attr(qname = ":decoupled"))]
  pub decoupled: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DataField Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:dataField.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_DataField/x15:dataField")]
pub struct DataField {
  /// isCountDistinct
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :isCountDistinct
  #[sdk(attr(qname = ":isCountDistinct"))]
  pub is_count_distinct: Option<crate::simple_type::BooleanValue>,
}
/// Defines the MovingPeriodState Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:movingPeriodState.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_MovingPeriodState/x15:movingPeriodState")]
pub struct MovingPeriodState {
  /// referenceDateBegin
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :referenceDateBegin
  #[sdk(attr(qname = ":referenceDateBegin"))]
  pub reference_date_begin: crate::simple_type::DateTimeValue,
  /// referencePeriod
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :referencePeriod
  #[sdk(attr(qname = ":referencePeriod"))]
  pub reference_period: MovingPeriodStep,
  /// referenceMultiple
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :referenceMultiple
  #[sdk(attr(qname = ":referenceMultiple"))]
  pub reference_multiple: crate::simple_type::UInt32Value,
  /// movingPeriod
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :movingPeriod
  #[sdk(attr(qname = ":movingPeriod"))]
  pub moving_period: MovingPeriodStep,
  /// movingMultiple
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :movingMultiple
  #[sdk(attr(qname = ":movingMultiple"))]
  pub moving_multiple: crate::simple_type::UInt32Value,
}
/// Defines the SlicerCaches Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:slicerCaches.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SlicerCaches/x15:slicerCaches")]
pub struct SlicerCaches {
  /// _
  #[sdk(child(qname = "x14:CT_SlicerCache/x14:slicerCache"))]
  pub x14_slicer_cache:
    Vec<crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerCache>,
}
/// Defines the TableSlicerCache Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:tableSlicerCache.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_TableSlicerCache/x15:tableSlicerCache")]
pub struct TableSlicerCache {
    /// tableId
    ///
    /// Available in Office2013 and above.
    ///
    /// Represents the following attribute in the schema: :tableId
    #[sdk(attr(qname = ":tableId"))]
    pub table_id: crate::simple_type::UInt32Value,
    /// column
    ///
    /// Available in Office2013 and above.
    ///
    /// Represents the following attribute in the schema: :column
    #[sdk(attr(qname = ":column"))]
    pub column: crate::simple_type::UInt32Value,
    /// sortOrder
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :sortOrder
    #[sdk(attr(qname = ":sortOrder"))]
    pub sort_order: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::TabularSlicerCacheSortOrderValues,
    >,
    /// customListSort
    ///
    /// Available in Office2013 and above.
    ///
    /// Represents the following attribute in the schema: :customListSort
    #[sdk(attr(qname = ":customListSort"))]
    pub custom_list_sort: Option<crate::simple_type::BooleanValue>,
    /// crossFilter
    ///
    /// Available in Office2010 and above.
    ///
    /// Represents the following attribute in the schema: :crossFilter
    #[sdk(attr(qname = ":crossFilter"))]
    pub cross_filter: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerCacheCrossFilterValues,
    >,
    /// _
    #[sdk(child(qname = "x:CT_ExtensionList/x15:extLst"))]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the SlicerCacheHideItemsWithNoData Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:slicerCacheHideItemsWithNoData.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_SlicerCacheHideNoData/x15:slicerCacheHideItemsWithNoData")]
pub struct SlicerCacheHideItemsWithNoData {
  /// count
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "x15:CT_SlicerCacheOlapLevelName/x15:slicerCacheOlapLevelName"))]
  pub x15_slicer_cache_olap_level_name: Vec<SlicerCacheOlapLevelName>,
}
/// Defines the SlicerCachePivotTables Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:slicerCachePivotTables.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SlicerCachePivotTables/x15:slicerCachePivotTables")]
pub struct SlicerCachePivotTables {
  /// _
  #[sdk(child(qname = "x14:CT_SlicerCachePivotTable/x14:pivotTable"))]
  pub x14_pivot_table: Vec<
    crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerCachePivotTable,
  >,
}
/// Defines the Survey Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:survey.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_Survey/x15:survey")]
pub struct Survey {
  /// id
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// guid
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :guid
  #[sdk(attr(qname = ":guid"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub guid: crate::simple_type::StringValue,
  /// title
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :title
  #[sdk(attr(qname = ":title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// description
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :description
  #[sdk(attr(qname = ":description"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x15:CT_SurveyElementPr/x15:surveyPr"))]
  pub survey_pr_survey_element_pr: Option<std::boxed::Box<SurveyPrSurveyElementPr>>,
  /// _
  #[sdk(child(qname = "x15:CT_SurveyElementPr/x15:titlePr"))]
  pub title_pr_survey_element_pr: Option<std::boxed::Box<TitlePrSurveyElementPr>>,
  /// _
  #[sdk(child(qname = "x15:CT_SurveyElementPr/x15:descriptionPr"))]
  pub description_pr_survey_element_pr: Option<std::boxed::Box<DescriptionPrSurveyElementPr>>,
  /// _
  #[sdk(child(qname = "x15:CT_SurveyQuestions/x15:questions"))]
  pub survey_questions: std::boxed::Box<SurveyQuestions>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Timelines Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:timelines.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_Timelines/x15:timelines")]
pub struct Timelines {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "x15:CT_Timeline/x15:timeline"))]
  pub x15_timeline: Vec<Timeline>,
}
/// Defines the TimelineCacheDefinition Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:timelineCacheDefinition.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_TimelineCacheDefinition/x15:timelineCacheDefinition")]
pub struct TimelineCacheDefinition {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// name
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// sourceName
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :sourceName
  #[sdk(attr(qname = ":sourceName"))]
  pub source_name: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x15:CT_TimelineCachePivotTables/x15:pivotTables"))]
  pub timeline_cache_pivot_tables: Option<TimelineCachePivotTables>,
  /// _
  #[sdk(child(qname = "x15:CT_TimelineState/x15:state"))]
  pub timeline_state: std::boxed::Box<TimelineState>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotTableReference Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:pivotTableReference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_PivotTableReference/x15:pivotTableReference")]
pub struct PivotTableReference {
  /// id
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the WebExtension Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:webExtension.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_WebExtension/x15:webExtension")]
pub struct WebExtension {
  /// appRef
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :appRef
  #[sdk(attr(qname = ":appRef"))]
  pub application_reference: crate::simple_type::StringValue,
  /// _
  #[sdk(text_child(qname = "x:ST_Formula/xne:f"))]
  pub formula: crate::simple_type::StringValue,
}
/// Defines the TimelineCacheReference Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:timelineCacheRef.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_TimelineCacheRef/x15:timelineCacheRef")]
pub struct TimelineCacheReference {
  /// id
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the TimelineReference Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:timelineRef.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_TimelineRef/x15:timelineRef")]
pub struct TimelineReference {
  /// id
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the TimelineStyle Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:timelineStyle.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_TimelineStyle/x15:timelineStyle")]
pub struct TimelineStyle {
  /// name
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x15:CT_TimelineStyleElements/x15:timelineStyleElements"))]
  pub timeline_style_elements: Option<TimelineStyleElements>,
}
/// Defines the TimelineStyleElement Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:timelineStyleElement.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_TimelineStyleElement/x15:timelineStyleElement")]
pub struct TimelineStyleElement {
  /// type
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: TimelineStyleType,
  /// dxfId
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :dxfId
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
}
/// Defines the TimelineStyleElements Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:timelineStyleElements.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_TimelineStyleElements/x15:timelineStyleElements")]
pub struct TimelineStyleElements {
  /// _
  #[sdk(child(qname = "x15:CT_TimelineStyleElement/x15:timelineStyleElement"))]
  pub x15_timeline_style_element: Vec<TimelineStyleElement>,
}
/// Defines the DbTable Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:dbTable.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_DbTable/x15:dbTable")]
pub struct DbTable {
  /// name
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// Defines the DbTables Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:dbTables.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_DbTables/x15:dbTables")]
pub struct DbTables {
  /// _
  #[sdk(child(qname = "x15:CT_DbTable/x15:dbTable"))]
  pub x15_db_table: Vec<DbTable>,
}
/// Defines the DbCommand Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:dbCommand.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_DbCommand/x15:dbCommand")]
pub struct DbCommand {
  /// text
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :text
  #[sdk(attr(qname = ":text"))]
  pub text: crate::simple_type::StringValue,
}
/// Defines the TextProperties Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:textPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_TextPr/x15:textPr")]
pub struct TextProperties {
  /// prompt
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :prompt
  #[sdk(attr(qname = ":prompt"))]
  pub prompt: Option<crate::simple_type::BooleanValue>,
  /// fileType
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :fileType
  #[sdk(attr(qname = ":fileType"))]
  pub file_type:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::FileTypeValues>,
  /// codePage
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :codePage
  #[sdk(attr(qname = ":codePage"))]
  pub code_page: Option<crate::simple_type::UInt32Value>,
  /// characterSet
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :characterSet
  #[sdk(attr(qname = ":characterSet"))]
  pub text_character_set: Option<crate::simple_type::StringValue>,
  /// firstRow
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :firstRow
  #[sdk(attr(qname = ":firstRow"))]
  pub first_row: Option<crate::simple_type::UInt32Value>,
  /// sourceFile
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :sourceFile
  #[sdk(attr(qname = ":sourceFile"))]
  pub source_file: Option<crate::simple_type::StringValue>,
  /// delimited
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :delimited
  #[sdk(attr(qname = ":delimited"))]
  pub delimited: Option<crate::simple_type::BooleanValue>,
  /// decimal
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :decimal
  #[sdk(attr(qname = ":decimal"))]
  pub decimal: Option<crate::simple_type::StringValue>,
  /// thousands
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :thousands
  #[sdk(attr(qname = ":thousands"))]
  pub thousands: Option<crate::simple_type::StringValue>,
  /// tab
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :tab
  #[sdk(attr(qname = ":tab"))]
  pub tab_as_delimiter: Option<crate::simple_type::BooleanValue>,
  /// space
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :space
  #[sdk(attr(qname = ":space"))]
  pub space: Option<crate::simple_type::BooleanValue>,
  /// comma
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :comma
  #[sdk(attr(qname = ":comma"))]
  pub comma: Option<crate::simple_type::BooleanValue>,
  /// semicolon
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :semicolon
  #[sdk(attr(qname = ":semicolon"))]
  pub semicolon: Option<crate::simple_type::BooleanValue>,
  /// consecutive
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :consecutive
  #[sdk(attr(qname = ":consecutive"))]
  pub consecutive: Option<crate::simple_type::BooleanValue>,
  /// qualifier
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :qualifier
  #[sdk(attr(qname = ":qualifier"))]
  pub qualifier:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::QualifierValues>,
  /// delimiter
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :delimiter
  #[sdk(attr(qname = ":delimiter"))]
  pub delimiter: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_TextFields/x:textFields"))]
  pub text_fields:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::TextFields>,
}
/// Defines the ModelTextProperties Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:modelTextPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_ModelTextPr/x15:modelTextPr")]
pub struct ModelTextProperties {
  /// headers
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :headers
  #[sdk(attr(qname = ":headers"))]
  pub headers: Option<crate::simple_type::BooleanValue>,
}
/// Defines the RangeProperties Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:rangePr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_RangePr/x15:rangePr")]
pub struct RangeProperties {
  /// sourceName
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :sourceName
  #[sdk(attr(qname = ":sourceName"))]
  pub source_name: crate::simple_type::StringValue,
}
/// Defines the OleDbPrpoperties Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:oledbPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_OledbPr/x15:oledbPr")]
pub struct OleDbPrpoperties {
  /// connection
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :connection
  #[sdk(attr(qname = ":connection"))]
  pub connection: Option<crate::simple_type::StringValue>,
  #[sdk(choice(
    qname = "x15:CT_DbTables/x15:dbTables",
    qname = "x15:CT_DbCommand/x15:dbCommand"
  ))]
  pub xml_children: Option<OleDbPrpopertiesChoice>,
}
/// Defines the DataFeedProperties Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:dataFeedPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_DataFeedPr/x15:dataFeedPr")]
pub struct DataFeedProperties {
  /// connection
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :connection
  #[sdk(attr(qname = ":connection"))]
  pub connection: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x15:CT_DbTables/x15:dbTables"))]
  pub db_tables: std::boxed::Box<DbTables>,
}
/// Defines the FieldListActiveTabTopLevelEntity Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:activeTabTopLevelEntity.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_FieldListActiveTabTopLevelEntity/x15:activeTabTopLevelEntity")]
pub struct FieldListActiveTabTopLevelEntity {
  /// name
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// type
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<crate::simple_type::UInt32Value>,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExtensionList/x15:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
/// Defines the CachedUniqueName Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:cachedUniqueName.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_CachedUniqueName/x15:cachedUniqueName")]
pub struct CachedUniqueName {
  /// index
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :index
  #[sdk(attr(qname = ":index"))]
  pub index: crate::simple_type::UInt32Value,
  /// name
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// Defines the ModelTable Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:modelTable.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_ModelTable/x15:modelTable")]
pub struct ModelTable {
  /// id
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// name
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// connection
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :connection
  #[sdk(attr(qname = ":connection"))]
  pub connection: crate::simple_type::StringValue,
}
/// Defines the ModelRelationship Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:modelRelationship.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_ModelRelationship/x15:modelRelationship")]
pub struct ModelRelationship {
  /// fromTable
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :fromTable
  #[sdk(attr(qname = ":fromTable"))]
  pub from_table: crate::simple_type::StringValue,
  /// fromColumn
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :fromColumn
  #[sdk(attr(qname = ":fromColumn"))]
  pub from_column: crate::simple_type::StringValue,
  /// toTable
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :toTable
  #[sdk(attr(qname = ":toTable"))]
  pub to_table: crate::simple_type::StringValue,
  /// toColumn
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :toColumn
  #[sdk(attr(qname = ":toColumn"))]
  pub to_column: crate::simple_type::StringValue,
}
/// Defines the ModelTables Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:modelTables.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_ModelTables/x15:modelTables")]
pub struct ModelTables {
  /// _
  #[sdk(child(qname = "x15:CT_ModelTable/x15:modelTable"))]
  pub x15_model_table: Vec<ModelTable>,
}
/// Defines the ModelRelationships Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:modelRelationships.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_ModelRelationships/x15:modelRelationships")]
pub struct ModelRelationships {
  /// _
  #[sdk(child(qname = "x15:CT_ModelRelationship/x15:modelRelationship"))]
  pub x15_model_relationship: Vec<ModelRelationship>,
}
/// Defines the PivotValueCell Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:c.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_PivotValueCell/x15:c")]
pub struct PivotValueCell {
  /// i
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :i
  #[sdk(attr(qname = ":i"))]
  pub item: Option<crate::simple_type::UInt32Value>,
  /// t
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub text: Option<SxvCellType>,
  /// _
  #[sdk(text_child(qname = "x:ST_Xstring/x15:v"))]
  pub xstring: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "x15:CT_PivotValueCellExtra/x15:x"))]
  pub pivot_value_cell_extra: Option<PivotValueCellExtra>,
}
/// Defines the Xstring Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:v.
pub type Xstring = crate::simple_type::StringValue;
/// Defines the PivotValueCellExtra Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:x.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_PivotValueCellExtra/x15:x")]
pub struct PivotValueCellExtra {
  /// in
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :in
  #[sdk(attr(qname = ":in"))]
  pub format_index: Option<crate::simple_type::UInt32Value>,
  /// bc
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :bc
  #[sdk(attr(qname = ":bc"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub background_color: Option<crate::simple_type::HexBinaryValue>,
  /// fc
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :fc
  #[sdk(attr(qname = ":fc"))]
  #[sdk(string_length(source = 0u32, min = 4u32, max = 4u32))]
  pub foreground_color: Option<crate::simple_type::HexBinaryValue>,
  /// i
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :i
  #[sdk(attr(qname = ":i"))]
  pub italic: Option<crate::simple_type::BooleanValue>,
  /// un
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :un
  #[sdk(attr(qname = ":un"))]
  pub underline: Option<crate::simple_type::BooleanValue>,
  /// st
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :st
  #[sdk(attr(qname = ":st"))]
  pub strikethrough: Option<crate::simple_type::BooleanValue>,
  /// b
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :b
  #[sdk(attr(qname = ":b"))]
  pub bold: Option<crate::simple_type::BooleanValue>,
}
/// Defines the PivotTableServerFormats Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:pivotTableServerFormats.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_PivotTableServerFormats/x15:pivotTableServerFormats")]
pub struct PivotTableServerFormats {
  /// count
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "x:CT_ServerFormat/x15:serverFormat"))]
  pub x15_server_format: Vec<ServerFormat>,
}
/// Defines the ServerFormat Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:serverFormat.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ServerFormat/x15:serverFormat")]
pub struct ServerFormat {
  /// Culture
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :culture
  #[sdk(attr(qname = ":culture"))]
  pub culture: Option<crate::simple_type::StringValue>,
  /// Format
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :format
  #[sdk(attr(qname = ":format"))]
  pub format: Option<crate::simple_type::StringValue>,
}
/// Defines the SlicerCacheOlapLevelName Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:slicerCacheOlapLevelName.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_SlicerCacheOlapLevelName/x15:slicerCacheOlapLevelName")]
pub struct SlicerCacheOlapLevelName {
  /// uniqueName
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :uniqueName
  #[sdk(attr(qname = ":uniqueName"))]
  pub unique_name: crate::simple_type::StringValue,
  /// count
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
}
/// Defines the SurveyPrSurveyElementPr Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:surveyPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_SurveyElementPr/x15:surveyPr")]
pub struct SurveyPrSurveyElementPr {
  /// cssClass
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :cssClass
  #[sdk(attr(qname = ":cssClass"))]
  pub css_class: Option<crate::simple_type::StringValue>,
  /// bottom
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :bottom
  #[sdk(attr(qname = ":bottom"))]
  pub bottom: Option<crate::simple_type::Int32Value>,
  /// top
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :top
  #[sdk(attr(qname = ":top"))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// left
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :left
  #[sdk(attr(qname = ":left"))]
  pub left: Option<crate::simple_type::Int32Value>,
  /// right
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :right
  #[sdk(attr(qname = ":right"))]
  pub right: Option<crate::simple_type::Int32Value>,
  /// width
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :width
  #[sdk(attr(qname = ":width"))]
  pub width: Option<crate::simple_type::UInt32Value>,
  /// height
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :height
  #[sdk(attr(qname = ":height"))]
  pub height: Option<crate::simple_type::UInt32Value>,
  /// position
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :position
  #[sdk(attr(qname = ":position"))]
  pub position: Option<SurveyPosition>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TitlePrSurveyElementPr Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:titlePr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_SurveyElementPr/x15:titlePr")]
pub struct TitlePrSurveyElementPr {
  /// cssClass
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :cssClass
  #[sdk(attr(qname = ":cssClass"))]
  pub css_class: Option<crate::simple_type::StringValue>,
  /// bottom
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :bottom
  #[sdk(attr(qname = ":bottom"))]
  pub bottom: Option<crate::simple_type::Int32Value>,
  /// top
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :top
  #[sdk(attr(qname = ":top"))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// left
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :left
  #[sdk(attr(qname = ":left"))]
  pub left: Option<crate::simple_type::Int32Value>,
  /// right
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :right
  #[sdk(attr(qname = ":right"))]
  pub right: Option<crate::simple_type::Int32Value>,
  /// width
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :width
  #[sdk(attr(qname = ":width"))]
  pub width: Option<crate::simple_type::UInt32Value>,
  /// height
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :height
  #[sdk(attr(qname = ":height"))]
  pub height: Option<crate::simple_type::UInt32Value>,
  /// position
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :position
  #[sdk(attr(qname = ":position"))]
  pub position: Option<SurveyPosition>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the DescriptionPrSurveyElementPr Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:descriptionPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_SurveyElementPr/x15:descriptionPr")]
pub struct DescriptionPrSurveyElementPr {
  /// cssClass
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :cssClass
  #[sdk(attr(qname = ":cssClass"))]
  pub css_class: Option<crate::simple_type::StringValue>,
  /// bottom
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :bottom
  #[sdk(attr(qname = ":bottom"))]
  pub bottom: Option<crate::simple_type::Int32Value>,
  /// top
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :top
  #[sdk(attr(qname = ":top"))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// left
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :left
  #[sdk(attr(qname = ":left"))]
  pub left: Option<crate::simple_type::Int32Value>,
  /// right
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :right
  #[sdk(attr(qname = ":right"))]
  pub right: Option<crate::simple_type::Int32Value>,
  /// width
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :width
  #[sdk(attr(qname = ":width"))]
  pub width: Option<crate::simple_type::UInt32Value>,
  /// height
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :height
  #[sdk(attr(qname = ":height"))]
  pub height: Option<crate::simple_type::UInt32Value>,
  /// position
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :position
  #[sdk(attr(qname = ":position"))]
  pub position: Option<SurveyPosition>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the QuestionsPrSurveyElementPr Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:questionsPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_SurveyElementPr/x15:questionsPr")]
pub struct QuestionsPrSurveyElementPr {
  /// cssClass
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :cssClass
  #[sdk(attr(qname = ":cssClass"))]
  pub css_class: Option<crate::simple_type::StringValue>,
  /// bottom
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :bottom
  #[sdk(attr(qname = ":bottom"))]
  pub bottom: Option<crate::simple_type::Int32Value>,
  /// top
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :top
  #[sdk(attr(qname = ":top"))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// left
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :left
  #[sdk(attr(qname = ":left"))]
  pub left: Option<crate::simple_type::Int32Value>,
  /// right
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :right
  #[sdk(attr(qname = ":right"))]
  pub right: Option<crate::simple_type::Int32Value>,
  /// width
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :width
  #[sdk(attr(qname = ":width"))]
  pub width: Option<crate::simple_type::UInt32Value>,
  /// height
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :height
  #[sdk(attr(qname = ":height"))]
  pub height: Option<crate::simple_type::UInt32Value>,
  /// position
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :position
  #[sdk(attr(qname = ":position"))]
  pub position: Option<SurveyPosition>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the QuestionPrSurveyElementPr Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:questionPr.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_SurveyElementPr/x15:questionPr")]
pub struct QuestionPrSurveyElementPr {
  /// cssClass
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :cssClass
  #[sdk(attr(qname = ":cssClass"))]
  pub css_class: Option<crate::simple_type::StringValue>,
  /// bottom
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :bottom
  #[sdk(attr(qname = ":bottom"))]
  pub bottom: Option<crate::simple_type::Int32Value>,
  /// top
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :top
  #[sdk(attr(qname = ":top"))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// left
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :left
  #[sdk(attr(qname = ":left"))]
  pub left: Option<crate::simple_type::Int32Value>,
  /// right
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :right
  #[sdk(attr(qname = ":right"))]
  pub right: Option<crate::simple_type::Int32Value>,
  /// width
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :width
  #[sdk(attr(qname = ":width"))]
  pub width: Option<crate::simple_type::UInt32Value>,
  /// height
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :height
  #[sdk(attr(qname = ":height"))]
  pub height: Option<crate::simple_type::UInt32Value>,
  /// position
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :position
  #[sdk(attr(qname = ":position"))]
  pub position: Option<SurveyPosition>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the OpenXmlSurveyElementPrElement Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_SurveyElementPr/")]
pub struct OpenXmlSurveyElementPrElement {
  /// cssClass
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :cssClass
  #[sdk(attr(qname = ":cssClass"))]
  pub css_class: Option<crate::simple_type::StringValue>,
  /// bottom
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :bottom
  #[sdk(attr(qname = ":bottom"))]
  pub bottom: Option<crate::simple_type::Int32Value>,
  /// top
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :top
  #[sdk(attr(qname = ":top"))]
  pub top: Option<crate::simple_type::Int32Value>,
  /// left
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :left
  #[sdk(attr(qname = ":left"))]
  pub left: Option<crate::simple_type::Int32Value>,
  /// right
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :right
  #[sdk(attr(qname = ":right"))]
  pub right: Option<crate::simple_type::Int32Value>,
  /// width
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :width
  #[sdk(attr(qname = ":width"))]
  pub width: Option<crate::simple_type::UInt32Value>,
  /// height
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :height
  #[sdk(attr(qname = ":height"))]
  pub height: Option<crate::simple_type::UInt32Value>,
  /// position
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :position
  #[sdk(attr(qname = ":position"))]
  pub position: Option<SurveyPosition>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Vec<ExtensionList>,
}
/// Defines the SurveyQuestions Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:questions.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_SurveyQuestions/x15:questions")]
pub struct SurveyQuestions {
  /// _
  #[sdk(child(qname = "x15:CT_SurveyElementPr/x15:questionsPr"))]
  pub questions_pr_survey_element_pr: Option<std::boxed::Box<QuestionsPrSurveyElementPr>>,
  /// _
  #[sdk(child(qname = "x15:CT_SurveyQuestion/x15:question"))]
  pub x15_question: Vec<SurveyQuestion>,
}
/// Defines the SurveyQuestion Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:question.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_SurveyQuestion/x15:question")]
pub struct SurveyQuestion {
  /// binding
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :binding
  #[sdk(attr(qname = ":binding"))]
  pub binding: crate::simple_type::UInt32Value,
  /// text
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :text
  #[sdk(attr(qname = ":text"))]
  pub text: Option<crate::simple_type::StringValue>,
  /// type
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: Option<QuestionType>,
  /// format
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :format
  #[sdk(attr(qname = ":format"))]
  pub format: Option<QuestionFormat>,
  /// helpText
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :helpText
  #[sdk(attr(qname = ":helpText"))]
  pub help_text: Option<crate::simple_type::StringValue>,
  /// required
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :required
  #[sdk(attr(qname = ":required"))]
  pub required: Option<crate::simple_type::BooleanValue>,
  /// defaultValue
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :defaultValue
  #[sdk(attr(qname = ":defaultValue"))]
  pub default_value: Option<crate::simple_type::StringValue>,
  /// decimalPlaces
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :decimalPlaces
  #[sdk(attr(qname = ":decimalPlaces"))]
  pub decimal_places: Option<crate::simple_type::UInt32Value>,
  /// rowSource
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :rowSource
  #[sdk(attr(qname = ":rowSource"))]
  pub row_source: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x15:CT_SurveyElementPr/x15:questionPr"))]
  pub question_pr_survey_element_pr: Option<std::boxed::Box<QuestionPrSurveyElementPr>>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the Timeline Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:timeline.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_Timeline/x15:timeline")]
pub struct Timeline {
  /// name
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// cache
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :cache
  #[sdk(attr(qname = ":cache"))]
  pub cache: crate::simple_type::StringValue,
  /// caption
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :caption
  #[sdk(attr(qname = ":caption"))]
  pub caption: Option<crate::simple_type::StringValue>,
  /// showHeader
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :showHeader
  #[sdk(attr(qname = ":showHeader"))]
  pub show_header: Option<crate::simple_type::BooleanValue>,
  /// showSelectionLabel
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :showSelectionLabel
  #[sdk(attr(qname = ":showSelectionLabel"))]
  pub show_selection_label: Option<crate::simple_type::BooleanValue>,
  /// showTimeLevel
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :showTimeLevel
  #[sdk(attr(qname = ":showTimeLevel"))]
  pub show_time_level: Option<crate::simple_type::BooleanValue>,
  /// showHorizontalScrollbar
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :showHorizontalScrollbar
  #[sdk(attr(qname = ":showHorizontalScrollbar"))]
  pub show_horizontal_scrollbar: Option<crate::simple_type::BooleanValue>,
  /// level
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :level
  #[sdk(attr(qname = ":level"))]
  pub level: crate::simple_type::UInt32Value,
  /// selectionLevel
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :selectionLevel
  #[sdk(attr(qname = ":selectionLevel"))]
  pub selection_level: crate::simple_type::UInt32Value,
  /// scrollPosition
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :scrollPosition
  #[sdk(attr(qname = ":scrollPosition"))]
  pub scroll_position: Option<crate::simple_type::DateTimeValue>,
  /// style
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :style
  #[sdk(attr(qname = ":style"))]
  pub style: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TimelineCachePivotTable Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:pivotTable.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_TimelineCachePivotTable/x15:pivotTable")]
pub struct TimelineCachePivotTable {
  /// tabId
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :tabId
  #[sdk(attr(qname = ":tabId"))]
  pub tab_id: crate::simple_type::UInt32Value,
  /// name
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
}
/// Defines the SelectionTimelineRange Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:selection.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_TimelineRange/x15:selection")]
pub struct SelectionTimelineRange {
  /// startDate
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :startDate
  #[sdk(attr(qname = ":startDate"))]
  pub start_date: crate::simple_type::DateTimeValue,
  /// endDate
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :endDate
  #[sdk(attr(qname = ":endDate"))]
  pub end_date: crate::simple_type::DateTimeValue,
}
/// Defines the BoundsTimelineRange Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:bounds.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_TimelineRange/x15:bounds")]
pub struct BoundsTimelineRange {
  /// startDate
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :startDate
  #[sdk(attr(qname = ":startDate"))]
  pub start_date: crate::simple_type::DateTimeValue,
  /// endDate
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :endDate
  #[sdk(attr(qname = ":endDate"))]
  pub end_date: crate::simple_type::DateTimeValue,
}
/// Defines the TimelineRange Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_TimelineRange/")]
pub struct TimelineRange {
  /// startDate
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :startDate
  #[sdk(attr(qname = ":startDate"))]
  pub start_date: crate::simple_type::DateTimeValue,
  /// endDate
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :endDate
  #[sdk(attr(qname = ":endDate"))]
  pub end_date: crate::simple_type::DateTimeValue,
}
/// Defines the AutoFilter Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:autoFilter.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_AutoFilter/x15:autoFilter")]
pub struct AutoFilter {
  /// Cell or Range Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_FilterColumn/x:filterColumn"))]
  pub x_filter_column:
    Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::FilterColumn>,
  /// _
  #[sdk(child(qname = "x:CT_SortState/x:sortState"))]
  pub x_sort_state: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SortState>,
  >,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList>,
}
/// Defines the TimelineCachePivotTables Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:pivotTables.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_TimelineCachePivotTables/x15:pivotTables")]
pub struct TimelineCachePivotTables {
  /// _
  #[sdk(child(qname = "x15:CT_TimelineCachePivotTable/x15:pivotTable"))]
  pub x15_pivot_table: Vec<TimelineCachePivotTable>,
}
/// Defines the TimelineState Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:state.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_TimelineState/x15:state")]
pub struct TimelineState {
  /// singleRangeFilterState
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :singleRangeFilterState
  #[sdk(attr(qname = ":singleRangeFilterState"))]
  pub single_range_filter_state: Option<crate::simple_type::BooleanValue>,
  /// minimalRefreshVersion
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :minimalRefreshVersion
  #[sdk(attr(qname = ":minimalRefreshVersion"))]
  pub minimal_refresh_version: crate::simple_type::UInt32Value,
  /// lastRefreshVersion
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :lastRefreshVersion
  #[sdk(attr(qname = ":lastRefreshVersion"))]
  pub last_refresh_version: crate::simple_type::UInt32Value,
  /// pivotCacheId
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :pivotCacheId
  #[sdk(attr(qname = ":pivotCacheId"))]
  pub pivot_cache_id: crate::simple_type::UInt32Value,
  /// filterType
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :filterType
  #[sdk(attr(qname = ":filterType"))]
  pub filter_type:
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotFilterValues,
  /// filterId
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :filterId
  #[sdk(attr(qname = ":filterId"))]
  pub filter_id: Option<crate::simple_type::UInt32Value>,
  /// filterTabId
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :filterTabId
  #[sdk(attr(qname = ":filterTabId"))]
  pub filter_tab_id: Option<crate::simple_type::UInt32Value>,
  /// filterPivotName
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :filterPivotName
  #[sdk(attr(qname = ":filterPivotName"))]
  pub filter_pivot_name: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x15:CT_TimelineRange/x15:selection"))]
  pub selection_timeline_range: Option<SelectionTimelineRange>,
  /// _
  #[sdk(child(qname = "x15:CT_TimelineRange/x15:bounds"))]
  pub bounds_timeline_range: std::boxed::Box<BoundsTimelineRange>,
  /// _
  #[sdk(child(qname = "x15:CT_MovingPeriodState/x15:movingPeriodState"))]
  pub moving_period_state: Option<MovingPeriodState>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the PivotRow Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15:pivotRow.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15:CT_PivotRow/x15:pivotRow")]
pub struct PivotRow {
  /// r
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub reference: Option<crate::simple_type::UInt32Value>,
  /// count
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "x15:CT_PivotValueCell/x15:c"))]
  pub x15_c: Vec<PivotValueCell>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum OleDbPrpopertiesChoice {
  /// Defines the DbTables Class.
  #[sdk(child(qname = "x15:CT_DbTables/x15:dbTables"))]
  X15DbTables(std::boxed::Box<DbTables>),
  /// Defines the DbCommand Class.
  #[sdk(child(qname = "x15:CT_DbCommand/x15:dbCommand"))]
  X15DbCommand(std::boxed::Box<DbCommand>),
}
