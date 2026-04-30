//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the NamedSheetViews Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "xnsv:CT_NamedSheetViews/xnsv:namedSheetViews")]
pub struct NamedSheetViews {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// _
  #[sdk(child(office2021, qname = "xnsv:CT_NamedSheetView/xnsv:namedSheetView"))]
  pub xnsv_named_sheet_view: Vec<NamedSheetView>,
  /// _
  #[sdk(child(office2021, qname = "x:CT_ExtensionList/xnsv:extLst"))]
  pub xnsv_ext_lst: Option<ExtensionList>,
}
/// Defines the NamedSheetView Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "xnsv:CT_NamedSheetView/xnsv:namedSheetView")]
pub struct NamedSheetView {
  /// name
  #[sdk(attr(office2021, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// _
  #[sdk(child(office2021, qname = "xnsv:CT_NsvFilter/xnsv:nsvFilter"))]
  pub xnsv_nsv_filter: Vec<NsvFilter>,
  /// _
  #[sdk(child(office2021, qname = "x:CT_ExtensionList/xnsv:extLst"))]
  pub xnsv_ext_lst: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "x:CT_ExtensionList/xnsv:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
/// Defines the NsvFilter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "xnsv:CT_NsvFilter/xnsv:nsvFilter")]
pub struct NsvFilter {
  /// filterId
  #[sdk(attr(office2021, qname = ":filterId"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub filter_id: crate::simple_type::StringValue,
  /// ref
  #[sdk(attr(office2021, qname = ":ref"))]
  pub r#ref: Option<crate::simple_type::StringValue>,
  /// tableId
  #[sdk(attr(office2021, qname = ":tableId"))]
  pub table_id: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(office2021, qname = "xnsv:CT_ColumnFilter/xnsv:columnFilter"))]
  pub xnsv_column_filter: Vec<ColumnFilter>,
  /// _
  #[sdk(child(office2021, qname = "xnsv:CT_SortRules/xnsv:sortRules"))]
  pub xnsv_sort_rules: Option<std::boxed::Box<SortRules>>,
  /// _
  #[sdk(child(office2021, qname = "x:CT_ExtensionList/xnsv:extLst"))]
  pub xnsv_ext_lst: Option<ExtensionList>,
}
/// Defines the ColumnFilter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "xnsv:CT_ColumnFilter/xnsv:columnFilter")]
pub struct ColumnFilter {
  /// colId
  #[sdk(attr(office2021, qname = ":colId"))]
  pub col_id: crate::simple_type::UInt32Value,
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(office2021, qname = "x:CT_Dxf/xnsv:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
  /// _
  #[sdk(child(office2021, qname = "x:CT_FilterColumn/xnsv:filter"))]
  pub xnsv_filter: Vec<FilterColumn>,
  /// _
  #[sdk(child(office2021, qname = "x:CT_ExtensionList/xnsv:extLst"))]
  pub xnsv_ext_lst: Option<ExtensionList>,
}
/// Defines the SortRules Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "xnsv:CT_SortRules/xnsv:sortRules")]
pub struct SortRules {
  /// sortMethod
  #[sdk(attr(office2021, qname = ":sortMethod"))]
  pub sort_method:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SortMethodValues>,
  /// caseSensitive
  #[sdk(attr(office2021, qname = ":caseSensitive"))]
  pub case_sensitive: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(office2021, qname = "xnsv:CT_SortRule/xnsv:sortRule"))]
  pub xnsv_sort_rule: Vec<SortRule>,
  /// _
  #[sdk(child(office2021, qname = "x:CT_ExtensionList/xnsv:extLst"))]
  pub xnsv_ext_lst: Option<ExtensionList>,
}
/// Defines the DifferentialFormatType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "x:CT_Dxf/xnsv:dxf")]
pub struct DifferentialFormatType {
  /// Font Properties
  #[sdk(child(qname = "x:CT_Font/x:font"))]
  pub font: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Font>,
  >,
  /// Number Format
  #[sdk(child(qname = "x:CT_NumFmt/x:numFmt"))]
  pub numbering_format:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::NumberingFormat>,
  /// Fill
  #[sdk(child(qname = "x:CT_Fill/x:fill"))]
  pub fill: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Fill>,
  >,
  /// Alignment
  #[sdk(child(qname = "x:CT_CellAlignment/x:alignment"))]
  pub alignment:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Alignment>,
  /// Border Properties
  #[sdk(child(qname = "x:CT_Border/x:border"))]
  pub border: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Border>,
  >,
  /// Protection Properties
  #[sdk(child(qname = "x:CT_CellProtection/x:protection"))]
  pub protection:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Protection>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub extension_list:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList>,
}
/// Defines the FilterColumn Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "x:CT_FilterColumn/xnsv:filter")]
pub struct FilterColumn {
  /// Filter Column Data
  #[sdk(attr(qname = ":colId"))]
  pub column_id: crate::simple_type::UInt32Value,
  /// Hidden AutoFilter Button
  #[sdk(attr(qname = ":hiddenButton"))]
  pub hidden_button: Option<crate::simple_type::BooleanValue>,
  /// Show Filter Button
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "xnsv:CT_SortRule/xnsv:sortRule")]
pub struct SortRule {
  /// colId
  #[sdk(attr(office2021, qname = ":colId"))]
  pub col_id: crate::simple_type::UInt32Value,
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(office2021, qname = "x:CT_Dxf/xnsv:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
  #[sdk(choice(
    microsoft365,
    qname = "x14:CT_SortCondition/xnsv:sortCondition",
    qname = "xlrd2:CT_RichSortCondition/xnsv:richSortCondition"
  ))]
  pub sort_rule_choice: Option<SortRuleChoice>,
}
/// Defines the SortCondition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "x14:CT_SortCondition/xnsv:sortCondition")]
pub struct SortCondition {
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
/// Defines the RichSortCondition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2021,
  qname = "xlrd2:CT_RichSortCondition/xnsv:richSortCondition"
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
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum FilterColumnChoice {
  /// Filter Criteria.
  #[sdk(child(qname = "x:CT_Filters/x:filters"))]
  XFilters(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Filters>,
  ),
  /// Top 10.
  #[sdk(child(qname = "x:CT_Top10/x:top10"))]
  XTop10(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Top10>,
  ),
  /// Defines the CustomFilters Class.
  #[sdk(child(office2010, qname = "x14:CT_CustomFilters/x14:customFilters"))]
  X14CustomFilters(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::CustomFilters,
    >,
  ),
  /// Custom Filters.
  #[sdk(child(qname = "x:CT_CustomFilters/x:customFilters"))]
  XCustomFilters(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CustomFilters,
    >,
  ),
  /// Dynamic Filter.
  #[sdk(child(qname = "x:CT_DynamicFilter/x:dynamicFilter"))]
  XDynamicFilter(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DynamicFilter,
    >,
  ),
  /// Color Filter Criteria.
  #[sdk(child(qname = "x:CT_ColorFilter/x:colorFilter"))]
  XColorFilter(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ColorFilter,
    >,
  ),
  /// Defines the IconFilter Class.
  #[sdk(child(office2010, qname = "x14:CT_IconFilter/x14:iconFilter"))]
  X14IconFilter(
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::IconFilter,
    >,
  ),
  /// Icon Filter.
  #[sdk(child(qname = "x:CT_IconFilter/x:iconFilter"))]
  XIconFilter(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::IconFilter>,
  ),
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  XExtLst(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList,
    >,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SortRuleChoice {
  #[sdk(child(office2021, qname = "x14:CT_SortCondition/xnsv:sortCondition"))]
  XnsvSortCondition(std::boxed::Box<SortCondition>),
  #[sdk(child(
    office2021,
    qname = "xlrd2:CT_RichSortCondition/xnsv:richSortCondition"
  ))]
  XnsvRichSortCondition(std::boxed::Box<RichSortCondition>),
}
