//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Macrosheet Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xne:macrosheet.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_Macrosheet/xne:macrosheet")]
pub struct Macrosheet {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
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
  #[cfg(not(feature = "mce"))]
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  /// Sheet Format Properties
  #[sdk(mce_child(qname = "x:CT_SheetFormatPr/x:sheetFormatPr"))]
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
  #[cfg(not(feature = "mce"))]
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content2:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  /// _
  #[sdk(mce_child(qname = "x:CT_DataConsolidate/x:dataConsolidate"))]
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
  #[cfg(not(feature = "mce"))]
  /// _
  #[sdk(child(qname = "mc:CT_AlternateContent/mc:AlternateContent"))]
  pub mc_alternate_content3:
    Option<crate::schemas::schemas_openxmlformats_org_markup_compatibility_2006::AlternateContent>,
  #[cfg(feature = "microsoft365")]
  /// _
  #[sdk(mce_child(qname = "x:CT_DrawingHF/x:drawingHF"))]
  pub x_drawing_hf:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DrawingHeaderFooter>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/x:extLst"))]
  pub x_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList>,
}
/// Worksheet Sort Map.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xne:worksheetSortMap.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xne:CT_WorksheetSortMap/xne:worksheetSortMap")]
pub struct WorksheetSortMap {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// Row Sort Map
  #[sdk(child(qname = "xne:CT_RowSortMap/xne:rowSortMap"))]
  pub row_sort_map: Option<RowSortMap>,
  /// Column Sort Map
  #[sdk(child(qname = "xne:CT_ColSortMap/xne:colSortMap"))]
  pub column_sort_map: Option<ColumnSortMap>,
}
#[cfg(feature = "microsoft365")]
/// Defines the ReferenceSequence Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is xne:sqref.
pub type ReferenceSequence = crate::simple_type::StringValue;
#[cfg(feature = "microsoft365")]
/// Defines the Formula Class.
///
/// Available in Office2010 and above.
///
/// When the object is serialized out as xml, it's qualified name is xne:f.
pub type Formula = crate::simple_type::StringValue;
/// Row Sort Map.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xne:rowSortMap.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xne:CT_RowSortMap/xne:rowSortMap")]
pub struct RowSortMap {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub r#ref: crate::simple_type::StringValue,
  /// Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xne:colSortMap.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xne:CT_ColSortMap/xne:colSortMap")]
pub struct ColumnSortMap {
  /// Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub r#ref: crate::simple_type::StringValue,
  /// Count
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :count
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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xne:row.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xne:CT_SortMapItem/xne:row")]
pub struct RowSortMapItem {
  /// New Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :newVal
  #[sdk(attr(qname = ":newVal"))]
  pub new_val: crate::simple_type::UInt32Value,
  /// Old Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oldVal
  #[sdk(attr(qname = ":oldVal"))]
  pub old_val: crate::simple_type::UInt32Value,
}
/// Column.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is xne:col.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xne:CT_SortMapItem/xne:col")]
pub struct ColumnSortMapItem {
  /// New Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :newVal
  #[sdk(attr(qname = ":newVal"))]
  pub new_val: crate::simple_type::UInt32Value,
  /// Old Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oldVal
  #[sdk(attr(qname = ":oldVal"))]
  pub old_val: crate::simple_type::UInt32Value,
}
/// Defines the SortMapItemType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xne:CT_SortMapItem/")]
pub struct SortMapItemType {
  /// New Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :newVal
  #[sdk(attr(qname = ":newVal"))]
  pub new_val: crate::simple_type::UInt32Value,
  /// Old Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :oldVal
  #[sdk(attr(qname = ":oldVal"))]
  pub old_val: crate::simple_type::UInt32Value,
}
