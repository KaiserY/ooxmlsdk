//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
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
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:filterColumn.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichFilterColumn/xlrd2:filterColumn")]
pub struct RichFilterColumn {
  #[sdk(choice)]
  pub xml_children: Option<RichFilterColumnChoice>,
}
/// Defines the RichSortCondition Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:richSortCondition.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichSortCondition/xlrd2:richSortCondition")]
pub struct RichSortCondition {
  /// richSortKey
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :richSortKey
  #[sdk(attr(qname = ":richSortKey"))]
  pub rich_sort_key: Option<crate::simple_type::StringValue>,
  /// descending
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :descending
  #[sdk(attr(qname = ":descending"))]
  pub descending: Option<crate::simple_type::BooleanValue>,
  /// sortBy
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :sortBy
  #[sdk(attr(qname = ":sortBy"))]
  pub sort_by:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SortByValues>,
  /// ref
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// customList
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :customList
  #[sdk(attr(qname = ":customList"))]
  pub custom_list: Option<crate::simple_type::StringValue>,
  /// dxfId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :dxfId
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
  /// iconSet
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :iconSet
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set: Option<
    crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::IconSetTypeValues,
  >,
  /// iconId
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :iconId
  #[sdk(attr(qname = ":iconId"))]
  pub icon_id: Option<crate::simple_type::UInt32Value>,
}
/// Defines the SupportingPropertyBags Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:supportingPropertyBags.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_SupportingPropertyBags/xlrd2:supportingPropertyBags")]
pub struct SupportingPropertyBags {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "xlrd2:CT_SupportingPropertyBagArrayData/xlrd2:spbArrays"))]
  pub supporting_property_bag_array_data: Option<std::boxed::Box<SupportingPropertyBagArrayData>>,
  /// _
  #[sdk(child(qname = "xlrd2:CT_SupportingPropertyBagData/xlrd2:spbData"))]
  pub supporting_property_bag_data: std::boxed::Box<SupportingPropertyBagData>,
}
/// Defines the SupportingPropertyBagStructures Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:spbStructures.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_SupportingPropertyBagStructures/xlrd2:spbStructures")]
pub struct SupportingPropertyBagStructures {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// count
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "xlrd2:CT_SupportingPropertyBagStructure/xlrd2:s"))]
  pub xlrd2_s: Vec<SupportingPropertyBagStructure>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub xlrd2_ext_lst: Option<ExtensionList>,
}
/// Defines the ArrayData Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:arrayData.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_ArrayData/xlrd2:arrayData")]
pub struct ArrayData {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// count
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "xlrd2:CT_Array/xlrd2:a"))]
  pub xlrd2_a: Vec<Array>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub xlrd2_ext_lst: Option<ExtensionList>,
}
/// Defines the RichStylesheet Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:richStyleSheet.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichStylesheet/xlrd2:richStyleSheet")]
pub struct RichStylesheet {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "x:CT_Dxfs/xlrd2:dxfs"))]
  pub dxfs: Option<Dxfs>,
  /// _
  #[sdk(child(qname = "xlrd2:CT_RichFormatProperties/xlrd2:richProperties"))]
  pub rich_format_properties: Option<RichFormatProperties>,
  /// _
  #[sdk(child(qname = "xlrd2:CT_RichStyles/xlrd2:richStyles"))]
  pub rich_styles: Option<RichStyles>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the RichValueTypesInfo Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:rvTypesInfo.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichValueTypesInfo/xlrd2:rvTypesInfo")]
pub struct RichValueTypesInfo {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "xlrd2:CT_RichValueGlobalType/xlrd2:global"))]
  pub rich_value_global_type: Option<std::boxed::Box<RichValueGlobalType>>,
  /// _
  #[sdk(child(qname = "xlrd2:CT_RichValueTypes/xlrd2:types"))]
  pub rich_value_types: Option<RichValueTypes>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the RichFilters Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:filters.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichFilters/xlrd2:filters")]
pub struct RichFilters {
  /// _
  #[sdk(child(qname = "xlrd2:CT_RichFilter/xlrd2:filter"))]
  pub xlrd2_filter: Vec<RichFilter>,
  /// _
  #[sdk(child(qname = "xlrd2:CT_RichDateGroupItem/xlrd2:dateGroupItem"))]
  pub xlrd2_date_group_item: Vec<RichDateGroupItem>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub xlrd2_ext_lst: Option<ExtensionList>,
}
/// Defines the RichTop10 Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:top10.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichTop10/xlrd2:top10")]
pub struct RichTop10 {
  /// key
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :key
  #[sdk(attr(qname = ":key"))]
  pub key: Option<crate::simple_type::StringValue>,
  /// Top
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :top
  #[sdk(attr(qname = ":top"))]
  pub top: Option<crate::simple_type::BooleanValue>,
  /// Filter by Percent
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :percent
  #[sdk(attr(qname = ":percent"))]
  pub percent: Option<crate::simple_type::BooleanValue>,
  /// Top or Bottom Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::DoubleValue,
  /// Filter Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :filterVal
  #[sdk(attr(qname = ":filterVal"))]
  pub filter_value: Option<crate::simple_type::DoubleValue>,
}
/// Defines the CustomRichFilters Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:customFilters.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_CustomRichFilters/xlrd2:customFilters")]
pub struct CustomRichFilters {
  /// and
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :and
  #[sdk(attr(qname = ":and"))]
  pub and: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice)]
  pub custom_rich_filters_choice: Vec<CustomRichFiltersChoice>,
}
/// Defines the DynamicRichFilter Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:dynamicFilter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_DynamicRichFilter/xlrd2:dynamicFilter")]
pub struct DynamicRichFilter {
  /// key
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :key
  #[sdk(attr(qname = ":key"))]
  pub key: Option<crate::simple_type::StringValue>,
  /// Dynamic filter type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type:
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DynamicFilterValues,
  /// Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::DoubleValue>,
  /// Max Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :maxVal
  #[sdk(attr(qname = ":maxVal"))]
  pub max_val: Option<crate::simple_type::DoubleValue>,
  /// valIso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :valIso
  #[sdk(attr(qname = ":valIso"))]
  pub val_iso: Option<crate::simple_type::DateTimeValue>,
  /// maxValIso
  ///
  /// Available in Office2010 and above.
  ///
  /// Represents the following attribute in the schema: :maxValIso
  #[sdk(attr(qname = ":maxValIso"))]
  pub max_val_iso: Option<crate::simple_type::DateTimeValue>,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExtensionList/xlrd2:extLst")]
pub struct ExtensionList {
  ///Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
/// Defines the RichFilter Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:filter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichFilter/xlrd2:filter")]
pub struct RichFilter {
  /// key
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :key
  #[sdk(attr(qname = ":key"))]
  pub key: Option<crate::simple_type::StringValue>,
  /// val
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::StringValue>,
  /// blank
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :blank
  #[sdk(attr(qname = ":blank"))]
  pub blank: Option<crate::simple_type::BooleanValue>,
  /// nodata
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :nodata
  #[sdk(attr(qname = ":nodata"))]
  pub nodata: Option<crate::simple_type::BooleanValue>,
}
/// Defines the RichDateGroupItem Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:dateGroupItem.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichDateGroupItem/xlrd2:dateGroupItem")]
pub struct RichDateGroupItem {
  /// key
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :key
  #[sdk(attr(qname = ":key"))]
  pub key: Option<crate::simple_type::StringValue>,
  /// Year
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :year
  #[sdk(attr(qname = ":year"))]
  pub year: crate::simple_type::UInt16Value,
  /// Month
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :month
  #[sdk(attr(qname = ":month"))]
  pub month: Option<crate::simple_type::UInt16Value>,
  /// Day
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :day
  #[sdk(attr(qname = ":day"))]
  pub day: Option<crate::simple_type::UInt16Value>,
  /// Hour
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hour
  #[sdk(attr(qname = ":hour"))]
  pub hour: Option<crate::simple_type::UInt16Value>,
  /// Minute
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :minute
  #[sdk(attr(qname = ":minute"))]
  pub minute: Option<crate::simple_type::UInt16Value>,
  /// Second
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :second
  #[sdk(attr(qname = ":second"))]
  pub second: Option<crate::simple_type::UInt16Value>,
  /// Date Time Grouping
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :dateTimeGrouping
  #[sdk(attr(qname = ":dateTimeGrouping"))]
  pub date_time_grouping:
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DateTimeGroupingValues,
}
/// Defines the CustomRichFilter Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:customFilter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_CustomRichFilter/xlrd2:customFilter")]
pub struct CustomRichFilter {
  /// key
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :key
  #[sdk(attr(qname = ":key"))]
  pub key: Option<crate::simple_type::StringValue>,
  /// Filter Comparison Operator
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :operator
  #[sdk(attr(qname = ":operator"))]
  pub operator: Option<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::FilterOperatorValues,
  >,
  /// Top or Bottom Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::StringValue>,
}
/// Defines the SupportingPropertyBagArrayData Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:spbArrays.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_SupportingPropertyBagArrayData/xlrd2:spbArrays")]
pub struct SupportingPropertyBagArrayData {
  /// count
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "xlrd2:CT_SupportingPropertyBagArray/xlrd2:a"))]
  pub xlrd2_a: Vec<SupportingPropertyBagArray>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub xlrd2_ext_lst: Option<ExtensionList>,
}
/// Defines the SupportingPropertyBagData Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:spbData.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_SupportingPropertyBagData/xlrd2:spbData")]
pub struct SupportingPropertyBagData {
  /// count
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "xlrd2:CT_SupportingPropertyBag/xlrd2:spb"))]
  pub xlrd2_spb: Vec<SupportingPropertyBag>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub xlrd2_ext_lst: Option<ExtensionList>,
}
/// Defines the SupportingPropertyBag Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:spb.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_SupportingPropertyBag/xlrd2:spb")]
pub struct SupportingPropertyBag {
  /// s
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :s
  #[sdk(attr(qname = ":s"))]
  pub s: crate::simple_type::UInt32Value,
  /// _
  #[sdk(text_child(qname = "xlrd2:CT_SupportingPropertyBagValue/xlrd2:v"))]
  pub xlrd2_v: Vec<crate::simple_type::StringValue>,
}
/// Defines the SupportingPropertyBagValue Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:v.
pub type SupportingPropertyBagValue = crate::simple_type::StringValue;
/// Defines the SupportingPropertyBagStructure Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:s.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_SupportingPropertyBagStructure/xlrd2:s")]
pub struct SupportingPropertyBagStructure {
  /// _
  #[sdk(child(qname = "xlrd2:CT_SupportingPropertyBagKey/xlrd2:k"))]
  pub xlrd2_k: Vec<SupportingPropertyBagKey>,
}
/// Defines the SupportingPropertyBagKey Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:k.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_SupportingPropertyBagKey/xlrd2:k")]
pub struct SupportingPropertyBagKey {
  /// n
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// t
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub t: Option<SupportingPropertyBagValueType>,
}
/// Defines the SupportingPropertyBagArray Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:a.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_SupportingPropertyBagArray/xlrd2:a")]
pub struct SupportingPropertyBagArray {
  /// count
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "xlrd2:CT_SupportingPropertyBagArrayValue/xlrd2:v"))]
  pub xlrd2_v: Vec<SupportingPropertyBagArrayValue>,
}
/// Defines the SupportingPropertyBagArrayValue Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:v.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_SupportingPropertyBagArrayValue/xlrd2:v")]
pub struct SupportingPropertyBagArrayValue {
  /// t
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub t: Option<SupportingPropertyBagArrayValueType>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Array Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:a.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_Array/xlrd2:a")]
pub struct Array {
  /// r
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :r
  #[sdk(attr(qname = ":r"))]
  pub r: crate::simple_type::UInt32Value,
  /// c
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :c
  #[sdk(attr(qname = ":c"))]
  pub c: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "xlrd2:CT_ArrayValue/xlrd2:v"))]
  pub xlrd2_v: Vec<ArrayValue>,
}
/// Defines the ArrayValue Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:v.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_ArrayValue/xlrd2:v")]
pub struct ArrayValue {
  /// t
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub t: Option<ArrayValueType>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Dxfs Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:dxfs.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dxfs/xlrd2:dxfs")]
pub struct Dxfs {
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
/// Defines the RichFormatProperties Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:richProperties.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichFormatProperties/xlrd2:richProperties")]
pub struct RichFormatProperties {
  /// _
  #[sdk(child(qname = "xlrd2:CT_RichFormatProperty/xlrd2:rPr"))]
  pub xlrd2_r_pr: Vec<RichFormatProperty>,
}
/// Defines the RichStyles Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:richStyles.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichStyles/xlrd2:richStyles")]
pub struct RichStyles {
  /// _
  #[sdk(child(qname = "xlrd2:CT_RichStyle/xlrd2:rSty"))]
  pub xlrd2_r_sty: Vec<RichStyle>,
}
/// Defines the RichFormatProperty Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:rPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichFormatProperty/xlrd2:rPr")]
pub struct RichFormatProperty {
  /// n
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// t
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub t: RichFormatPropertyType,
}
/// Defines the RichStyle Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:rSty.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichStyle/xlrd2:rSty")]
pub struct RichStyle {
  /// dxfid
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :dxfid
  #[sdk(attr(qname = ":dxfid"))]
  pub dxfid: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "xlrd2:CT_RichStylePropertyValue/xlrd2:rpv"))]
  pub xlrd2_rpv: Vec<RichStylePropertyValue>,
}
/// Defines the RichStylePropertyValue Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:rpv.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichStylePropertyValue/xlrd2:rpv")]
pub struct RichStylePropertyValue {
  /// i
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :i
  #[sdk(attr(qname = ":i"))]
  pub i: crate::simple_type::UInt32Value,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the RichValueGlobalType Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:global.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichValueGlobalType/xlrd2:global")]
pub struct RichValueGlobalType {
  /// _
  #[sdk(child(qname = "xlrd2:CT_RichValueTypeKeyFlags/xlrd2:keyFlags"))]
  pub rich_value_type_key_flags: Option<RichValueTypeKeyFlags>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the RichValueTypes Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:types.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichValueTypes/xlrd2:types")]
pub struct RichValueTypes {
  /// _
  #[sdk(child(qname = "xlrd2:CT_RichValueType/xlrd2:type"))]
  pub xlrd2_type: Vec<RichValueType>,
}
/// Defines the RichValueType Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:type.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichValueType/xlrd2:type")]
pub struct RichValueType {
  /// name
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "xlrd2:CT_RichValueTypeKeyFlags/xlrd2:keyFlags"))]
  pub rich_value_type_key_flags: Option<RichValueTypeKeyFlags>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the RichValueTypeKeyFlags Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:keyFlags.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichValueTypeKeyFlags/xlrd2:keyFlags")]
pub struct RichValueTypeKeyFlags {
  /// _
  #[sdk(child(qname = "xlrd2:CT_RichValueTypeReservedKey/xlrd2:key"))]
  pub xlrd2_key: Vec<RichValueTypeReservedKey>,
}
/// Defines the RichValueTypeReservedKey Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:key.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichValueTypeReservedKey/xlrd2:key")]
pub struct RichValueTypeReservedKey {
  /// name
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "xlrd2:CT_RichValueTypeReservedKeyFlag/xlrd2:flag"))]
  pub xlrd2_flag: Vec<RichValueTypeReservedKeyFlag>,
}
/// Defines the RichValueTypeReservedKeyFlag Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd2:flag.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichValueTypeReservedKeyFlag/xlrd2:flag")]
pub struct RichValueTypeReservedKeyFlag {
  /// name
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// value
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :value
  #[sdk(attr(qname = ":value"))]
  pub value: crate::simple_type::BooleanValue,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum RichFilterColumnChoice {
  #[sdk(child(qname = "xlrd2:CT_RichFilters/xlrd2:filters"))]
  Xlrd2Filters(std::boxed::Box<RichFilters>),
  #[sdk(child(qname = "xlrd2:CT_RichTop10/xlrd2:top10"))]
  Xlrd2Top10(std::boxed::Box<RichTop10>),
  #[sdk(child(qname = "xlrd2:CT_CustomRichFilters/xlrd2:customFilters"))]
  Xlrd2CustomFilters(std::boxed::Box<CustomRichFilters>),
  #[sdk(child(qname = "xlrd2:CT_DynamicRichFilter/xlrd2:dynamicFilter"))]
  Xlrd2DynamicFilter(std::boxed::Box<DynamicRichFilter>),
  #[sdk(child(qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  Xlrd2ExtLst(std::boxed::Box<ExtensionList>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum CustomRichFiltersChoice {
  #[sdk(child(qname = "xlrd2:CT_CustomRichFilter/xlrd2:customFilter"))]
  Xlrd2CustomFilter(std::boxed::Box<CustomRichFilter>),
  #[sdk(child(qname = "x:CT_ExtensionList/xlrd2:extLst"))]
  Xlrd2ExtLst(std::boxed::Box<ExtensionList>),
}
