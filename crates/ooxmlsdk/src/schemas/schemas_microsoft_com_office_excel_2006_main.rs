//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Macrosheet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Macrosheet/xne:macrosheet")]
pub struct Macrosheet {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  pub xml_other_children: Vec<(usize, String)>,
  /// Sheet Properties
  #[sdk(child(qname = "x:CT_SheetPr/x:sheetPr"))]
  pub sheet_properties: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SheetProperties,
    >,
  >,
  /// Macro Sheet Dimensions
  #[sdk(child(qname = "x:CT_SheetDimension/x:dimension"))]
  pub sheet_dimension:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SheetDimension>,
  /// Macro Sheet Views
  #[sdk(child(qname = "x:CT_SheetViews/x:sheetViews"))]
  pub sheet_views: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SheetViews>,
  >,
  /// Sheet Format Properties
  #[sdk(child(qname = "x:CT_SheetFormatPr/x:sheetFormatPr"))]
  pub sheet_format_properties: Option<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SheetFormatProperties,
  >,
  /// _
  #[sdk(child(qname = "x:CT_Cols/x:cols"))]
  pub x_cols: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Columns>,
  /// _
  #[sdk(child(qname = "x:CT_SheetData/x:sheetData"))]
  pub x_sheet_data:
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SheetData>,
  /// _
  #[sdk(child(qname = "x:CT_SheetProtection/x:sheetProtection"))]
  pub x_sheet_protection:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SheetProtection>,
  /// _
  #[sdk(child(qname = "x:CT_AutoFilter/x:autoFilter"))]
  pub x_auto_filter: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::AutoFilter>,
  >,
  /// _
  #[sdk(child(qname = "x:CT_SortState/x:sortState"))]
  pub x_sort_state: Option<
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SortState>,
  >,
  /// _
  #[sdk(child(qname = "x:CT_DataConsolidate/x:dataConsolidate"))]
  pub x_data_consolidate: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataConsolidate,
    >,
  >,
  /// _
  #[sdk(child(qname = "x:CT_CustomSheetViews/x:customSheetViews"))]
  pub x_custom_sheet_views:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CustomSheetViews>,
  /// _
  #[sdk(child(qname = "x:CT_PhoneticPr/x:phoneticPr"))]
  pub x_phonetic_pr:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PhoneticProperties>,
  /// _
  #[sdk(child(qname = "x:CT_ConditionalFormatting/x:conditionalFormatting"))]
  pub x_conditional_formatting:
    Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ConditionalFormatting>,
  /// _
  #[sdk(child(qname = "x:CT_PrintOptions/x:printOptions"))]
  pub x_print_options:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PrintOptions>,
  /// _
  #[sdk(child(qname = "x:CT_PageMargins/x:pageMargins"))]
  pub x_page_margins:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PageMargins>,
  /// _
  #[sdk(child(qname = "x:CT_PageSetup/x:pageSetup"))]
  pub x_page_setup:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PageSetup>,
  /// _
  #[sdk(child(qname = "x:CT_HeaderFooter/x:headerFooter"))]
  pub x_header_footer: Option<
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::HeaderFooter,
    >,
  >,
  /// _
  #[sdk(child(qname = "x:CT_PageBreak/x:rowBreaks"))]
  pub x_row_breaks:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::RowBreaks>,
  /// _
  #[sdk(child(qname = "x:CT_PageBreak/x:colBreaks"))]
  pub x_col_breaks:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ColumnBreaks>,
  /// _
  #[sdk(child(qname = "x:CT_CustomProperties/x:customProperties"))]
  pub x_custom_properties:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CustomProperties>,
  /// _
  #[sdk(child(qname = "x:CT_Drawing/x:drawing"))]
  pub x_drawing:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Drawing>,
  /// _
  #[sdk(child(qname = "x:CT_LegacyDrawing/x:legacyDrawing"))]
  pub x_legacy_drawing:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::LegacyDrawing>,
  /// _
  #[sdk(child(qname = "x:CT_LegacyDrawing/x:legacyDrawingHF"))]
  pub x_legacy_drawing_hf: Option<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::LegacyDrawingHeaderFooter,
  >,
  /// _
  #[sdk(child(qname = "x:CT_SheetBackgroundPicture/x:picture"))]
  pub x_picture:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Picture>,
  /// _
  #[sdk(child(qname = "x:CT_OleObjects/x:oleObjects"))]
  pub x_ole_objects:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::OleObjects>,
  /// _
  #[sdk(child(office2010, qname = "x:CT_DrawingHF/x:drawingHF"))]
  pub x_drawing_hf:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DrawingHeaderFooter>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList>,
}
/// Worksheet Sort Map.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xne:CT_WorksheetSortMap/xne:worksheetSortMap")]
pub struct WorksheetSortMap {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Row Sort Map
  #[sdk(child(qname = "xne:CT_RowSortMap/xne:rowSortMap"))]
  pub row_sort_map: Option<RowSortMap>,
  /// Column Sort Map
  #[sdk(child(qname = "xne:CT_ColSortMap/xne:colSortMap"))]
  pub column_sort_map: Option<ColumnSortMap>,
}
/// Defines the ReferenceSequence Class.
pub type ReferenceSequence = crate::simple_type::StringValue;
/// Defines the Formula Class.
pub type Formula = crate::simple_type::StringValue;
/// Row Sort Map.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xne:CT_RowSortMap/xne:rowSortMap")]
pub struct RowSortMap {
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub r#ref: crate::simple_type::StringValue,
  /// Count
  #[sdk(attr(qname = ":count"))]
  #[sdk(number_range(
    source = 0u32,
    max = "536870910",
    min_inclusive = false,
    max_inclusive = true
  ))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "xne:CT_SortMapItem/xne:row"))]
  pub xne_row: Vec<RowSortMapItem>,
}
/// Column Sort Map.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xne:CT_ColSortMap/xne:colSortMap")]
pub struct ColumnSortMap {
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub r#ref: crate::simple_type::StringValue,
  /// Count
  #[sdk(attr(qname = ":count"))]
  #[sdk(number_range(
    source = 0u32,
    max = "536870910",
    min_inclusive = false,
    max_inclusive = true
  ))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// _
  #[sdk(child(qname = "xne:CT_SortMapItem/xne:col"))]
  pub xne_col: Vec<ColumnSortMapItem>,
}
/// Row.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xne:CT_SortMapItem/xne:row")]
pub struct RowSortMapItem {
  /// New Value
  #[sdk(attr(qname = ":newVal"))]
  pub new_val: crate::simple_type::UInt32Value,
  /// Old Value
  #[sdk(attr(qname = ":oldVal"))]
  pub old_val: crate::simple_type::UInt32Value,
}
/// Column.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xne:CT_SortMapItem/xne:col")]
pub struct ColumnSortMapItem {
  /// New Value
  #[sdk(attr(qname = ":newVal"))]
  pub new_val: crate::simple_type::UInt32Value,
  /// Old Value
  #[sdk(attr(qname = ":oldVal"))]
  pub old_val: crate::simple_type::UInt32Value,
}
/// Defines the SortMapItemType Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xne:CT_SortMapItem/")]
pub struct SortMapItemType {
  /// New Value
  #[sdk(attr(qname = ":newVal"))]
  pub new_val: crate::simple_type::UInt32Value,
  /// Old Value
  #[sdk(attr(qname = ":oldVal"))]
  pub old_val: crate::simple_type::UInt32Value,
}
