//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the NamedSheetViews Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xnsv:namedSheetViews.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xnsv:CT_NamedSheetViews/xnsv:namedSheetViews")]
pub struct NamedSheetViews {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// _
  #[sdk(child(qname = "xnsv:CT_NamedSheetView/xnsv:namedSheetView"))]
  pub xnsv_named_sheet_view: Vec<NamedSheetView>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xnsv:extLst"))]
  pub xnsv_ext_lst: Option<ExtensionList>,
}
/// Defines the NamedSheetView Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xnsv:namedSheetView.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xnsv:CT_NamedSheetView/xnsv:namedSheetView")]
pub struct NamedSheetView {
  /// name
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// id
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "xnsv:CT_NsvFilter/xnsv:nsvFilter"))]
  pub xnsv_nsv_filter: Vec<NsvFilter>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xnsv:extLst"))]
  pub xnsv_ext_lst: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xnsv:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExtensionList/xnsv:extLst")]
pub struct ExtensionList {
  ///Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
/// Defines the NsvFilter Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xnsv:nsvFilter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xnsv:CT_NsvFilter/xnsv:nsvFilter")]
pub struct NsvFilter {
  /// filterId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :filterId
  #[sdk(attr(qname = ":filterId"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub filter_id: crate::simple_type::StringValue,
  /// ref
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub r#ref: Option<crate::simple_type::StringValue>,
  /// tableId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :tableId
  #[sdk(attr(qname = ":tableId"))]
  pub table_id: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "xnsv:CT_ColumnFilter/xnsv:columnFilter"))]
  pub xnsv_column_filter: Vec<ColumnFilter>,
  /// _
  #[sdk(child(qname = "xnsv:CT_SortRules/xnsv:sortRules"))]
  pub xnsv_sort_rules: Option<std::boxed::Box<SortRules>>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xnsv:extLst"))]
  pub xnsv_ext_lst: Option<ExtensionList>,
}
/// Defines the ColumnFilter Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xnsv:columnFilter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xnsv:CT_ColumnFilter/xnsv:columnFilter")]
pub struct ColumnFilter {
  /// colId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :colId
  #[sdk(attr(qname = ":colId"))]
  pub col_id: crate::simple_type::UInt32Value,
  /// id
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_Dxf/xnsv:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
  /// _
  #[sdk(child(qname = "x:CT_FilterColumn/xnsv:filter"))]
  pub xnsv_filter: Vec<FilterColumn>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xnsv:extLst"))]
  pub xnsv_ext_lst: Option<ExtensionList>,
}
/// Defines the SortRules Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xnsv:sortRules.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xnsv:CT_SortRules/xnsv:sortRules")]
pub struct SortRules {
  /// sortMethod
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :sortMethod
  #[sdk(attr(qname = ":sortMethod"))]
  pub sort_method:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SortMethodValues>,
  /// caseSensitive
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :caseSensitive
  #[sdk(attr(qname = ":caseSensitive"))]
  pub case_sensitive: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "xnsv:CT_SortRule/xnsv:sortRule"))]
  pub xnsv_sort_rule: Vec<SortRule>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xnsv:extLst"))]
  pub xnsv_ext_lst: Option<ExtensionList>,
}
/// Defines the DifferentialFormatType Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xnsv:dxf.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Dxf/xnsv:dxf")]
pub struct DifferentialFormatType {
  ///Font Properties
  #[sdk(child(qname = "x:CT_Font/x:font"))]
  pub font: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Font>,
  >,
  ///Number Format
  #[sdk(child(qname = "x:CT_NumFmt/x:numFmt"))]
  pub numbering_format:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::NumberingFormat>,
  ///Fill
  #[sdk(child(qname = "x:CT_Fill/x:fill"))]
  pub fill: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Fill>,
  >,
  ///Alignment
  #[sdk(child(qname = "x:CT_CellAlignment/x:alignment"))]
  pub alignment:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Alignment>,
  ///Border Properties
  #[sdk(child(qname = "x:CT_Border/x:border"))]
  pub border: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Border>,
  >,
  ///Protection Properties
  #[sdk(child(qname = "x:CT_CellProtection/x:protection"))]
  pub protection:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Protection>,
  ///Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList>,
}
/// Defines the FilterColumn Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xnsv:filter.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_FilterColumn/xnsv:filter")]
pub struct FilterColumn {
  /// Filter Column Data
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :colId
  #[sdk(attr(qname = ":colId"))]
  pub column_id: crate::simple_type::UInt32Value,
  /// Hidden AutoFilter Button
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :hiddenButton
  #[sdk(attr(qname = ":hiddenButton"))]
  pub hidden_button: Option<crate::simple_type::BooleanValue>,
  /// Show Filter Button
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :showButton
  #[sdk(attr(qname = ":showButton"))]
  pub show_button: Option<crate::simple_type::BooleanValue>,
  #[sdk(choice(
    qname = "x:CT_Filters/x:filters",
    qname = "x:CT_Top10/x:top10",
    qname = "x14:CT_CustomFilters/x14:customFilters",
    qname = "x:CT_CustomFilters/x:customFilters",
    qname = "x:CT_DynamicFilter/x:dynamicFilter",
    qname = "x:CT_ColorFilter/x:colorFilter",
    qname = "x14:CT_IconFilter/x14:iconFilter",
    qname = "x:CT_IconFilter/x:iconFilter",
    qname = "x:CT_ExtensionList/x:extLst"
  ))]
  pub xml_children: Option<FilterColumnChoice>,
}
/// Defines the SortRule Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xnsv:sortRule.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xnsv:CT_SortRule/xnsv:sortRule")]
pub struct SortRule {
  /// colId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :colId
  #[sdk(attr(qname = ":colId"))]
  pub col_id: crate::simple_type::UInt32Value,
  /// id
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "x:CT_Dxf/xnsv:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
  #[sdk(choice(
    qname = "x14:CT_SortCondition/xnsv:sortCondition",
    qname = "xlrd2:CT_RichSortCondition/xnsv:richSortCondition"
  ))]
  pub sort_rule_choice: Option<SortRuleChoice>,
}
/// Defines the SortCondition Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xnsv:sortCondition.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x14:CT_SortCondition/xnsv:sortCondition")]
pub struct SortCondition {
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
/// Defines the RichSortCondition Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xnsv:richSortCondition.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd2:CT_RichSortCondition/xnsv:richSortCondition")]
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
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum FilterColumnChoice {
  #[sdk(child(qname = "x:CT_Filters/x:filters"))]
  XFilters(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Filters>,
  ),
  #[sdk(child(qname = "x:CT_Top10/x:top10"))]
  XTop10(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Top10>,
  ),
  #[sdk(child(qname = "x14:CT_CustomFilters/x14:customFilters"))]
  X14CustomFilters(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::CustomFilters,
    >,
  ),
  #[sdk(child(qname = "x:CT_CustomFilters/x:customFilters"))]
  XCustomFilters(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CustomFilters,
    >,
  ),
  #[sdk(child(qname = "x:CT_DynamicFilter/x:dynamicFilter"))]
  XDynamicFilter(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DynamicFilter,
    >,
  ),
  #[sdk(child(qname = "x:CT_ColorFilter/x:colorFilter"))]
  XColorFilter(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ColorFilter,
    >,
  ),
  #[sdk(child(qname = "x14:CT_IconFilter/x14:iconFilter"))]
  X14IconFilter(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::IconFilter,
    >,
  ),
  #[sdk(child(qname = "x:CT_IconFilter/x:iconFilter"))]
  XIconFilter(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::IconFilter>,
  ),
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  XExtLst(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList,
    >,
  ),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum SortRuleChoice {
  #[sdk(child(qname = "x14:CT_SortCondition/xnsv:sortCondition"))]
  XnsvSortCondition(std::boxed::Box<SortCondition>),
  #[sdk(child(qname = "xlrd2:CT_RichSortCondition/xnsv:richSortCondition"))]
  XnsvRichSortCondition(std::boxed::Box<RichSortCondition>),
}
