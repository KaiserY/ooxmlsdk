//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SupportingPropertyBagValueType {
  #[sdk(rename = "d")]
  #[default]
  D,
  #[sdk(rename = "i")]
  I,
  #[sdk(rename = "b")]
  B,
  #[sdk(rename = "s")]
  S,
  #[sdk(rename = "spb")]
  Spb,
  #[sdk(rename = "spba")]
  Spba,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum SupportingPropertyBagArrayValueType {
  #[sdk(rename = "d")]
  #[default]
  D,
  #[sdk(rename = "i")]
  I,
  #[sdk(rename = "b")]
  B,
  #[sdk(rename = "s")]
  S,
  #[sdk(rename = "spb")]
  Spb,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum ArrayValueType {
  #[sdk(rename = "d")]
  #[default]
  D,
  #[sdk(rename = "i")]
  I,
  #[sdk(rename = "b")]
  B,
  #[sdk(rename = "e")]
  E,
  #[sdk(rename = "s")]
  S,
  #[sdk(rename = "r")]
  R,
  #[sdk(rename = "a")]
  A,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RichFormatPropertyType {
  #[sdk(rename = "b")]
  #[default]
  B,
  #[sdk(rename = "n")]
  N,
  #[sdk(rename = "i")]
  I,
  #[sdk(rename = "s")]
  S,
}
/// Defines the RichFilterColumn Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_RichFilterColumn/xlrd2:filterColumn")]
pub struct RichFilterColumn {
  #[sdk(choice(
    qname = "xlrd2:CT_RichFilters/xlrd2:filters",
    qname = "xlrd2:CT_RichTop10/xlrd2:top10",
    qname = "xlrd2:CT_CustomRichFilters/xlrd2:customFilters",
    qname = "xlrd2:CT_DynamicRichFilter/xlrd2:dynamicFilter",
    qname = "x:CT_ExtensionList/xlrd2:extLst"
  ))]
  pub rich_filter_column_choice: Option<RichFilterColumnChoice>,
}
/// Defines the RichSortCondition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2019,
  qname = "xlrd2:CT_RichSortCondition/xlrd2:richSortCondition"
)]
pub struct RichSortCondition {
  /// richSortKey
  #[sdk(attr(office2019, qname = ":richSortKey"))]
  pub rich_sort_key: Option<crate::simple_type::StringValue>,
  /// descending
  #[sdk(attr(office2010, qname = ":descending"))]
  pub descending: Option<crate::simple_type::BooleanValue>,
  /// sortBy
  #[sdk(attr(office2010, qname = ":sortBy"))]
  pub sort_by:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SortByValues>,
  /// ref
  #[sdk(attr(office2010, qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// customList
  #[sdk(attr(office2010, qname = ":customList"))]
  pub custom_list: Option<crate::simple_type::StringValue>,
  /// dxfId
  #[sdk(attr(office2010, qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
  /// iconSet
  #[sdk(attr(office2010, qname = ":iconSet"))]
  pub icon_set: Option<
    crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::IconSetTypeValues,
  >,
  /// iconId
  #[sdk(attr(office2010, qname = ":iconId"))]
  pub icon_id: Option<crate::simple_type::UInt32Value>,
}
/// Defines the SupportingPropertyBags Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2019,
  qname = "xlrd2:CT_SupportingPropertyBags/xlrd2:supportingPropertyBags"
)]
pub struct SupportingPropertyBags {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Defines the SupportingPropertyBagArrayData Class.
  #[sdk(child(
    office2019,
    qname = "xlrd2:CT_SupportingPropertyBagArrayData/xlrd2:spbArrays"
  ))]
  pub supporting_property_bag_array_data: Option<std::boxed::Box<SupportingPropertyBagArrayData>>,
  /// Defines the SupportingPropertyBagData Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_SupportingPropertyBagData/xlrd2:spbData"))]
  pub supporting_property_bag_data: std::boxed::Box<SupportingPropertyBagData>,
}
/// Defines the SupportingPropertyBagStructures Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2019,
  qname = "xlrd2:CT_SupportingPropertyBagStructures/xlrd2:spbStructures"
)]
pub struct SupportingPropertyBagStructures {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// count
  #[sdk(attr(office2019, qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// Defines the SupportingPropertyBagStructure Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_SupportingPropertyBagStructure/xlrd2:s"))]
  pub xlrd2_s: Vec<SupportingPropertyBagStructure>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub xlrd2_ext_lst: Option<ExtensionList>,
}
/// Defines the ArrayData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_ArrayData/xlrd2:arrayData")]
pub struct ArrayData {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// count
  #[sdk(attr(office2019, qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// Defines the Array Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_Array/xlrd2:a"))]
  pub xlrd2_a: Vec<Array>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub xlrd2_ext_lst: Option<ExtensionList>,
}
/// Defines the RichStylesheet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_RichStylesheet/xlrd2:richStyleSheet")]
pub struct RichStylesheet {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Defines the Dxfs Class.
  #[sdk(child(office2019, qname = "x:CT_Dxfs/xlrd2:dxfs"))]
  pub dxfs: Option<Dxfs>,
  /// Defines the RichFormatProperties Class.
  #[sdk(child(
    office2019,
    qname = "xlrd2:CT_RichFormatProperties/xlrd2:richProperties"
  ))]
  pub rich_format_properties: Option<RichFormatProperties>,
  /// Defines the RichStyles Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_RichStyles/xlrd2:richStyles"))]
  pub rich_styles: Option<RichStyles>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the RichValueTypesInfo Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_RichValueTypesInfo/xlrd2:rvTypesInfo")]
pub struct RichValueTypesInfo {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Defines the RichValueGlobalType Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_RichValueGlobalType/xlrd2:global"))]
  pub rich_value_global_type: Option<std::boxed::Box<RichValueGlobalType>>,
  /// Defines the RichValueTypes Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_RichValueTypes/xlrd2:types"))]
  pub rich_value_types: Option<RichValueTypes>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the RichFilters Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_RichFilters/xlrd2:filters")]
pub struct RichFilters {
  /// Defines the RichFilter Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_RichFilter/xlrd2:filter"))]
  pub xlrd2_filter: Vec<RichFilter>,
  /// Defines the RichDateGroupItem Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_RichDateGroupItem/xlrd2:dateGroupItem"))]
  pub xlrd2_date_group_item: Vec<RichDateGroupItem>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub xlrd2_ext_lst: Option<ExtensionList>,
}
/// Defines the RichTop10 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_RichTop10/xlrd2:top10")]
pub struct RichTop10 {
  /// key
  #[sdk(attr(office2019, qname = ":key"))]
  pub key: Option<crate::simple_type::StringValue>,
  /// Top
  #[sdk(attr(qname = ":top"))]
  pub top: Option<crate::simple_type::BooleanValue>,
  /// Filter by Percent
  #[sdk(attr(qname = ":percent"))]
  pub percent: Option<crate::simple_type::BooleanValue>,
  /// Top or Bottom Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
  /// Filter Value
  #[sdk(attr(qname = ":filterVal"))]
  pub filter_value: Option<crate::simple_type::DoubleValue>,
}
/// Defines the CustomRichFilters Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_CustomRichFilters/xlrd2:customFilters")]
pub struct CustomRichFilters {
  /// and
  #[sdk(attr(office2019, qname = ":and"))]
  pub and: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "xlrd2:CT_CustomRichFilter/xlrd2:customFilter",
    qname = "x:CT_ExtensionList/xlrd2:extLst"
  ))]
  pub custom_rich_filters_choice: Vec<CustomRichFiltersChoice>,
}
/// Defines the DynamicRichFilter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_DynamicRichFilter/xlrd2:dynamicFilter")]
pub struct DynamicRichFilter {
  /// key
  #[sdk(attr(office2019, qname = ":key"))]
  pub key: Option<crate::simple_type::StringValue>,
  /// Dynamic filter type
  #[sdk(attr(qname = ":type"))]
  pub r#type:
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DynamicFilterValues,
  /// Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::DoubleValue>,
  /// Max Value
  #[sdk(attr(qname = ":maxVal"))]
  pub max_val: Option<crate::simple_type::DoubleValue>,
  /// valIso
  #[sdk(attr(office2010, qname = ":valIso"))]
  pub val_iso: Option<crate::simple_type::DateTimeValue>,
  /// maxValIso
  #[sdk(attr(office2010, qname = ":maxValIso"))]
  pub max_val_iso: Option<crate::simple_type::DateTimeValue>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "x:CT_ExtensionList/xlrd2:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub x_ext: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
/// Defines the RichFilter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_RichFilter/xlrd2:filter")]
pub struct RichFilter {
  /// key
  #[sdk(attr(office2019, qname = ":key"))]
  pub key: Option<crate::simple_type::StringValue>,
  /// val
  #[sdk(attr(office2019, qname = ":val"))]
  pub val: Option<crate::simple_type::StringValue>,
  /// blank
  #[sdk(attr(office2019, qname = ":blank"))]
  pub blank: Option<crate::simple_type::BooleanValue>,
  /// nodata
  #[sdk(attr(office2019, qname = ":nodata"))]
  pub nodata: Option<crate::simple_type::BooleanValue>,
}
/// Defines the RichDateGroupItem Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_RichDateGroupItem/xlrd2:dateGroupItem")]
pub struct RichDateGroupItem {
  /// key
  #[sdk(attr(office2019, qname = ":key"))]
  pub key: Option<crate::simple_type::StringValue>,
  /// Year
  #[sdk(attr(qname = ":year"))]
  pub year: crate::simple_type::UInt16Value,
  /// Month
  #[sdk(attr(qname = ":month"))]
  pub month: Option<crate::simple_type::UInt16Value>,
  /// Day
  #[sdk(attr(qname = ":day"))]
  pub day: Option<crate::simple_type::UInt16Value>,
  /// Hour
  #[sdk(attr(qname = ":hour"))]
  pub hour: Option<crate::simple_type::UInt16Value>,
  /// Minute
  #[sdk(attr(qname = ":minute"))]
  pub minute: Option<crate::simple_type::UInt16Value>,
  /// Second
  #[sdk(attr(qname = ":second"))]
  pub second: Option<crate::simple_type::UInt16Value>,
  /// Date Time Grouping
  #[sdk(attr(qname = ":dateTimeGrouping"))]
  pub date_time_grouping:
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DateTimeGroupingValues,
}
/// Defines the CustomRichFilter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_CustomRichFilter/xlrd2:customFilter")]
pub struct CustomRichFilter {
  /// key
  #[sdk(attr(office2019, qname = ":key"))]
  pub key: Option<crate::simple_type::StringValue>,
  /// Filter Comparison Operator
  #[sdk(attr(qname = ":operator"))]
  pub operator: Option<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::FilterOperatorValues,
  >,
  /// Top or Bottom Value
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Defines the SupportingPropertyBagArrayData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2019,
  qname = "xlrd2:CT_SupportingPropertyBagArrayData/xlrd2:spbArrays"
)]
pub struct SupportingPropertyBagArrayData {
  /// count
  #[sdk(attr(office2019, qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// Defines the SupportingPropertyBagArray Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_SupportingPropertyBagArray/xlrd2:a"))]
  pub xlrd2_a: Vec<SupportingPropertyBagArray>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub xlrd2_ext_lst: Option<ExtensionList>,
}
/// Defines the SupportingPropertyBagData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_SupportingPropertyBagData/xlrd2:spbData")]
pub struct SupportingPropertyBagData {
  /// count
  #[sdk(attr(office2019, qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// Defines the SupportingPropertyBag Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_SupportingPropertyBag/xlrd2:spb"))]
  pub xlrd2_spb: Vec<SupportingPropertyBag>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub xlrd2_ext_lst: Option<ExtensionList>,
}
/// Defines the SupportingPropertyBag Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_SupportingPropertyBag/xlrd2:spb")]
pub struct SupportingPropertyBag {
  /// s
  #[sdk(attr(office2019, qname = ":s"))]
  pub s: crate::simple_type::UInt32Value,
  /// Defines the SupportingPropertyBagValue Class.
  #[sdk(text_child(office2019, qname = "xlrd2:CT_SupportingPropertyBagValue/xlrd2:v"))]
  pub xlrd2_v: Vec<crate::simple_type::StringValue>,
}
/// Defines the SupportingPropertyBagValue Class.
pub type SupportingPropertyBagValue = crate::simple_type::StringValue;
/// Defines the SupportingPropertyBagStructure Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_SupportingPropertyBagStructure/xlrd2:s")]
pub struct SupportingPropertyBagStructure {
  /// Defines the SupportingPropertyBagKey Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_SupportingPropertyBagKey/xlrd2:k"))]
  pub xlrd2_k: Vec<SupportingPropertyBagKey>,
}
/// Defines the SupportingPropertyBagKey Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_SupportingPropertyBagKey/xlrd2:k")]
pub struct SupportingPropertyBagKey {
  /// n
  #[sdk(attr(office2019, qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// t
  #[sdk(attr(office2019, qname = ":t"))]
  pub t: Option<SupportingPropertyBagValueType>,
}
/// Defines the SupportingPropertyBagArray Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_SupportingPropertyBagArray/xlrd2:a")]
pub struct SupportingPropertyBagArray {
  /// count
  #[sdk(attr(office2019, qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// Defines the SupportingPropertyBagArrayValue Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_SupportingPropertyBagArrayValue/xlrd2:v"))]
  pub xlrd2_v: Vec<SupportingPropertyBagArrayValue>,
}
/// Defines the SupportingPropertyBagArrayValue Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_SupportingPropertyBagArrayValue/xlrd2:v")]
pub struct SupportingPropertyBagArrayValue {
  /// t
  #[sdk(attr(office2019, qname = ":t"))]
  pub t: Option<SupportingPropertyBagArrayValueType>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Array Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_Array/xlrd2:a")]
pub struct Array {
  /// r
  #[sdk(attr(office2019, qname = ":r"))]
  pub r: crate::simple_type::UInt32Value,
  /// c
  #[sdk(attr(office2019, qname = ":c"))]
  pub c: Option<crate::simple_type::UInt32Value>,
  /// Defines the ArrayValue Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_ArrayValue/xlrd2:v"))]
  pub xlrd2_v: Vec<ArrayValue>,
}
/// Defines the ArrayValue Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_ArrayValue/xlrd2:v")]
pub struct ArrayValue {
  /// t
  #[sdk(attr(office2019, qname = ":t"))]
  pub t: Option<ArrayValueType>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Dxfs Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "x:CT_Dxfs/xlrd2:dxfs")]
pub struct Dxfs {
  /// Format Count
  #[sdk(attr(qname = ":count"))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Formatting.
  #[sdk(child(qname = "x:CT_Dxf/x:dxf"))]
  pub x_dxf:
    Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DifferentialFormat>,
}
/// Defines the RichFormatProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2019,
  qname = "xlrd2:CT_RichFormatProperties/xlrd2:richProperties"
)]
pub struct RichFormatProperties {
  /// Defines the RichFormatProperty Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_RichFormatProperty/xlrd2:rPr"))]
  pub xlrd2_r_pr: Vec<RichFormatProperty>,
}
/// Defines the RichStyles Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_RichStyles/xlrd2:richStyles")]
pub struct RichStyles {
  /// Defines the RichStyle Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_RichStyle/xlrd2:rSty"))]
  pub xlrd2_r_sty: Vec<RichStyle>,
}
/// Defines the RichFormatProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_RichFormatProperty/xlrd2:rPr")]
pub struct RichFormatProperty {
  /// n
  #[sdk(attr(office2019, qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// t
  #[sdk(attr(office2019, qname = ":t"))]
  pub t: RichFormatPropertyType,
}
/// Defines the RichStyle Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_RichStyle/xlrd2:rSty")]
pub struct RichStyle {
  /// dxfid
  #[sdk(attr(office2019, qname = ":dxfid"))]
  pub dxfid: Option<crate::simple_type::UInt32Value>,
  /// Defines the RichStylePropertyValue Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_RichStylePropertyValue/xlrd2:rpv"))]
  pub xlrd2_rpv: Vec<RichStylePropertyValue>,
}
/// Defines the RichStylePropertyValue Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_RichStylePropertyValue/xlrd2:rpv")]
pub struct RichStylePropertyValue {
  /// i
  #[sdk(attr(office2019, qname = ":i"))]
  pub i: crate::simple_type::UInt32Value,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the RichValueGlobalType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_RichValueGlobalType/xlrd2:global")]
pub struct RichValueGlobalType {
  /// Defines the RichValueTypeKeyFlags Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_RichValueTypeKeyFlags/xlrd2:keyFlags"))]
  pub rich_value_type_key_flags: Option<RichValueTypeKeyFlags>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the RichValueTypes Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_RichValueTypes/xlrd2:types")]
pub struct RichValueTypes {
  /// Defines the RichValueType Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_RichValueType/xlrd2:type"))]
  pub xlrd2_type: Vec<RichValueType>,
}
/// Defines the RichValueType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_RichValueType/xlrd2:type")]
pub struct RichValueType {
  /// name
  #[sdk(attr(office2019, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Defines the RichValueTypeKeyFlags Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_RichValueTypeKeyFlags/xlrd2:keyFlags"))]
  pub rich_value_type_key_flags: Option<RichValueTypeKeyFlags>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the RichValueTypeKeyFlags Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_RichValueTypeKeyFlags/xlrd2:keyFlags")]
pub struct RichValueTypeKeyFlags {
  /// Defines the RichValueTypeReservedKey Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_RichValueTypeReservedKey/xlrd2:key"))]
  pub xlrd2_key: Vec<RichValueTypeReservedKey>,
}
/// Defines the RichValueTypeReservedKey Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_RichValueTypeReservedKey/xlrd2:key")]
pub struct RichValueTypeReservedKey {
  /// name
  #[sdk(attr(office2019, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Defines the RichValueTypeReservedKeyFlag Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_RichValueTypeReservedKeyFlag/xlrd2:flag"))]
  pub xlrd2_flag: Vec<RichValueTypeReservedKeyFlag>,
}
/// Defines the RichValueTypeReservedKeyFlag Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd2:CT_RichValueTypeReservedKeyFlag/xlrd2:flag")]
pub struct RichValueTypeReservedKeyFlag {
  /// name
  #[sdk(attr(office2019, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// value
  #[sdk(attr(office2019, qname = ":value"))]
  pub value: crate::simple_type::BooleanValue,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum RichFilterColumnChoice {
  /// Defines the RichFilters Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_RichFilters/xlrd2:filters"))]
  Xlrd2Filters(std::boxed::Box<RichFilters>),
  /// Defines the RichTop10 Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_RichTop10/xlrd2:top10"))]
  Xlrd2Top10(std::boxed::Box<RichTop10>),
  /// Defines the CustomRichFilters Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_CustomRichFilters/xlrd2:customFilters"))]
  Xlrd2CustomFilters(std::boxed::Box<CustomRichFilters>),
  /// Defines the DynamicRichFilter Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_DynamicRichFilter/xlrd2:dynamicFilter"))]
  Xlrd2DynamicFilter(std::boxed::Box<DynamicRichFilter>),
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  Xlrd2ExtLst(std::boxed::Box<ExtensionList>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum CustomRichFiltersChoice {
  /// Defines the CustomRichFilter Class.
  #[sdk(child(office2019, qname = "xlrd2:CT_CustomRichFilter/xlrd2:customFilter"))]
  Xlrd2CustomFilter(std::boxed::Box<CustomRichFilter>),
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  Xlrd2ExtLst(std::boxed::Box<ExtensionList>),
}
