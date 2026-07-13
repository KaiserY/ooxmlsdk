//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the NamedSheetViews Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix_only, xml_header, qname = "xnsv:namedSheetViews")]
pub struct NamedSheetViews {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub mc_ignorable: Option<std::boxed::Box<[u8]>>,
  /// Defines the NamedSheetView Class.
  #[sdk(child(qname = "xnsv:namedSheetView"))]
  pub named_sheet_view: Vec<NamedSheetView>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "xnsv:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the NamedSheetView Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix_only, qname = "xnsv:namedSheetView")]
pub struct NamedSheetView {
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: crate::simple_type::StringValue,
  /// Defines the NsvFilter Class.
  #[sdk(child(qname = "xnsv:nsvFilter"))]
  pub nsv_filter: Vec<NsvFilter>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "xnsv:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix_only, qname = "xnsv:extLst")]
pub struct ExtensionList {
  /// Extension.
  #[sdk(child(qname = "x:ext"))]
  pub extension: Vec<crate::schemas::x::Extension>,
}
/// Defines the NsvFilter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix_only, qname = "xnsv:nsvFilter")]
pub struct NsvFilter {
  /// filterId
  #[sdk(attr(qname = ":filterId"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub filter_id: crate::simple_type::StringValue,
  /// ref
  #[sdk(attr(qname = ":ref"))]
  pub r#ref: Option<crate::simple_type::StringValue>,
  /// tableId
  #[sdk(attr(qname = ":tableId"))]
  pub table_id: Option<crate::simple_type::UInt32Value>,
  /// Defines the ColumnFilter Class.
  #[sdk(child(qname = "xnsv:columnFilter"))]
  pub column_filter: Vec<ColumnFilter>,
  /// Defines the SortRules Class.
  #[sdk(child(qname = "xnsv:sortRules"))]
  pub sort_rules: Option<std::boxed::Box<SortRules>>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "xnsv:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the ColumnFilter Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix_only, qname = "xnsv:columnFilter")]
pub struct ColumnFilter {
  /// colId
  #[sdk(attr(qname = ":colId"))]
  pub col_id: crate::simple_type::UInt32Value,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(qname = "xnsv:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
  /// Defines the FilterColumn Class.
  #[sdk(child(qname = "xnsv:filter"))]
  pub filter_column: Vec<FilterColumn>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "xnsv:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the SortRules Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix_only, qname = "xnsv:sortRules")]
pub struct SortRules {
  /// sortMethod
  #[sdk(attr(qname = ":sortMethod"))]
  pub sort_method: Option<crate::schemas::x::SortMethodValues>,
  /// caseSensitive
  #[sdk(attr(qname = ":caseSensitive"))]
  pub case_sensitive: Option<crate::simple_type::BooleanValue>,
  /// Defines the SortRule Class.
  #[sdk(child(qname = "xnsv:sortRule"))]
  pub sort_rule: Vec<SortRule>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "xnsv:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the DifferentialFormatType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix_only, qname = "xnsv:dxf")]
pub struct DifferentialFormatType {
  /// Font Properties
  #[sdk(child(qname = "x:font"))]
  pub font: Option<crate::schemas::x::Font>,
  /// Number Format
  #[sdk(child(qname = "x:numFmt"))]
  pub numbering_format: Option<crate::schemas::x::NumberingFormat>,
  /// Fill
  #[sdk(child(qname = "x:fill"))]
  pub fill: Option<std::boxed::Box<crate::schemas::x::Fill>>,
  /// Alignment
  #[sdk(child(qname = "x:alignment"))]
  pub alignment: Option<crate::schemas::x::Alignment>,
  /// Border Properties
  #[sdk(child(qname = "x:border"))]
  pub border: Option<std::boxed::Box<crate::schemas::x::Border>>,
  /// Protection Properties
  #[sdk(child(qname = "x:protection"))]
  pub protection: Option<crate::schemas::x::Protection>,
  /// Future Feature Data Storage Area
  #[sdk(child(qname = "x:extLst"))]
  pub extension_list: Option<crate::schemas::x::ExtensionList>,
}
/// Defines the FilterColumn Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix_only, qname = "xnsv:filter")]
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
  #[sdk(
        choice(
            child(variant = Filters, qname = "x:filters"),
            child(variant = Top10, qname = "x:top10"),
            child(variant = X14CustomFilters, qname = "x14:customFilters"),
            child(variant = XCustomFilters, qname = "x:customFilters"),
            child(variant = DynamicFilter, qname = "x:dynamicFilter"),
            child(variant = ColorFilter, qname = "x:colorFilter"),
            child(variant = X14IconFilter, qname = "x14:iconFilter"),
            child(variant = XIconFilter, qname = "x:iconFilter"),
            child(variant = ExtensionList, qname = "x:extLst")
        )
    )]
  pub filter_column_choice: Option<FilterColumnChoice>,
}
/// Defines the SortRule Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix_only, qname = "xnsv:sortRule")]
pub struct SortRule {
  /// colId
  #[sdk(attr(qname = ":colId"))]
  pub col_id: crate::simple_type::UInt32Value,
  /// id
  #[sdk(attr(qname = ":id"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// Defines the DifferentialFormatType Class.
  #[sdk(child(qname = "xnsv:dxf"))]
  pub differential_format_type: Option<std::boxed::Box<DifferentialFormatType>>,
  #[sdk(
        choice(
            child(variant = SortCondition, boxed, qname = "xnsv:sortCondition"),
            child(variant = RichSortCondition, boxed, qname = "xnsv:richSortCondition")
        )
    )]
  pub sort_rule_choice: Option<SortRuleChoice>,
}
/// Defines the SortCondition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix_only, qname = "xnsv:sortCondition")]
pub struct SortCondition {
  /// descending
  #[sdk(attr(qname = ":descending"))]
  pub descending: Option<crate::simple_type::BooleanValue>,
  /// sortBy
  #[sdk(attr(qname = ":sortBy"))]
  pub sort_by: Option<crate::schemas::x::SortByValues>,
  /// ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// customList
  #[sdk(attr(qname = ":customList"))]
  pub custom_list: Option<crate::simple_type::StringValue>,
  /// dxfId
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
  /// iconSet
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set: Option<crate::schemas::x14::IconSetTypeValues>,
  /// iconId
  #[sdk(attr(qname = ":iconId"))]
  pub icon_id: Option<crate::simple_type::UInt32Value>,
}
/// Defines the RichSortCondition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix_only, qname = "xnsv:richSortCondition")]
pub struct RichSortCondition {
  /// richSortKey
  #[sdk(attr(qname = ":richSortKey"))]
  pub rich_sort_key: Option<crate::simple_type::StringValue>,
  /// descending
  #[sdk(attr(qname = ":descending"))]
  pub descending: Option<crate::simple_type::BooleanValue>,
  /// sortBy
  #[sdk(attr(qname = ":sortBy"))]
  pub sort_by: Option<crate::schemas::x::SortByValues>,
  /// ref
  #[sdk(attr(qname = ":ref"))]
  pub reference: crate::simple_type::StringValue,
  /// customList
  #[sdk(attr(qname = ":customList"))]
  pub custom_list: Option<crate::simple_type::StringValue>,
  /// dxfId
  #[sdk(attr(qname = ":dxfId"))]
  pub format_id: Option<crate::simple_type::UInt32Value>,
  /// iconSet
  #[sdk(attr(qname = ":iconSet"))]
  pub icon_set: Option<crate::schemas::x14::IconSetTypeValues>,
  /// iconId
  #[sdk(attr(qname = ":iconId"))]
  pub icon_id: Option<crate::simple_type::UInt32Value>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum FilterColumnChoice {
  /// Filter Criteria.
  Filters(crate::schemas::x::Filters),
  /// Top 10.
  Top10(crate::schemas::x::Top10),
  /// Defines the CustomFilters Class.
  X14CustomFilters(crate::schemas::x14::CustomFilters),
  /// Custom Filters.
  XCustomFilters(crate::schemas::x::CustomFilters),
  /// Dynamic Filter.
  DynamicFilter(crate::schemas::x::DynamicFilter),
  /// Color Filter Criteria.
  ColorFilter(crate::schemas::x::ColorFilter),
  /// Defines the IconFilter Class.
  X14IconFilter(crate::schemas::x14::IconFilter),
  /// Icon Filter.
  XIconFilter(crate::schemas::x::IconFilter),
  /// Defines the ExtensionList Class.
  ExtensionList(crate::schemas::x::ExtensionList),
}
#[derive(Clone, Debug, PartialEq)]
pub enum SortRuleChoice {
  /// Defines the SortCondition Class.
  SortCondition(std::boxed::Box<SortCondition>),
  /// Defines the RichSortCondition Class.
  RichSortCondition(std::boxed::Box<RichSortCondition>),
}
