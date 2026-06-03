//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Macrosheet Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xne:macrosheet")]
pub struct Macrosheet {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<crate::common::XmlOtherAttr>,
  /// Sheet Properties
  #[sdk(child(qname = "x:sheetPr"))]
  pub sheet_properties: Option<std::boxed::Box<crate::schemas::x::SheetProperties>>,
  /// Macro Sheet Dimensions
  #[sdk(child(qname = "x:dimension"))]
  pub sheet_dimension: Option<crate::schemas::x::SheetDimension>,
  /// Macro Sheet Views
  #[sdk(child(qname = "x:sheetViews"))]
  pub sheet_views: Option<std::boxed::Box<crate::schemas::x::SheetViews>>,
  /// Sheet Format Properties
  #[sdk(child(qname = "x:sheetFormatPr"))]
  pub sheet_format_properties: Option<crate::schemas::x::SheetFormatProperties>,
  /// Column Information.
  #[sdk(child(qname = "x:cols"))]
  pub columns: Vec<crate::schemas::x::Columns>,
  /// Sheet Data.
  #[sdk(child(qname = "x:sheetData"))]
  pub sheet_data: std::boxed::Box<crate::schemas::x::SheetData>,
  /// Sheet Protection.
  #[sdk(child(qname = "x:sheetProtection"))]
  pub sheet_protection: Option<crate::schemas::x::SheetProtection>,
  /// AutoFilter Settings.
  #[sdk(child(qname = "x:autoFilter"))]
  pub auto_filter: Option<std::boxed::Box<crate::schemas::x::AutoFilter>>,
  /// Sort State for Auto Filter.
  #[sdk(child(qname = "x:sortState"))]
  pub sort_state: Option<std::boxed::Box<crate::schemas::x::SortState>>,
  /// Data Consolidation.
  #[sdk(child(qname = "x:dataConsolidate"))]
  pub data_consolidate: Option<std::boxed::Box<crate::schemas::x::DataConsolidate>>,
  /// Custom Sheet Views.
  #[sdk(child(qname = "x:customSheetViews"))]
  pub custom_sheet_views: Option<crate::schemas::x::CustomSheetViews>,
  /// Phonetic Properties.
  #[sdk(child(qname = "x:phoneticPr"))]
  pub phonetic_properties: Option<crate::schemas::x::PhoneticProperties>,
  /// Conditional Formatting.
  #[sdk(child(qname = "x:conditionalFormatting"))]
  pub conditional_formatting: Vec<crate::schemas::x::ConditionalFormatting>,
  /// Print Options.
  #[sdk(child(qname = "x:printOptions"))]
  pub print_options: Option<crate::schemas::x::PrintOptions>,
  /// Page Margins.
  #[sdk(child(qname = "x:pageMargins"))]
  pub page_margins: Option<crate::schemas::x::PageMargins>,
  /// Page Setup Settings.
  #[sdk(child(qname = "x:pageSetup"))]
  pub page_setup: Option<crate::schemas::x::PageSetup>,
  /// Header Footer Settings.
  #[sdk(child(qname = "x:headerFooter"))]
  pub header_footer: Option<std::boxed::Box<crate::schemas::x::HeaderFooter>>,
  /// Horizontal Page Breaks.
  #[sdk(child(qname = "x:rowBreaks"))]
  pub row_breaks: Option<crate::schemas::x::RowBreaks>,
  /// Vertical Page Breaks.
  #[sdk(child(qname = "x:colBreaks"))]
  pub column_breaks: Option<crate::schemas::x::ColumnBreaks>,
  /// Custom Properties.
  #[sdk(child(qname = "x:customProperties"))]
  pub custom_properties: Option<crate::schemas::x::CustomProperties>,
  /// Drawing.
  #[sdk(child(qname = "x:drawing"))]
  pub drawing: Option<crate::schemas::x::Drawing>,
  /// Defines the LegacyDrawing Class.
  #[sdk(child(qname = "x:legacyDrawing"))]
  pub legacy_drawing: Option<crate::schemas::x::LegacyDrawing>,
  /// Legacy Drawing Reference in  Header Footer.
  #[sdk(child(qname = "x:legacyDrawingHF"))]
  pub legacy_drawing_header_footer: Option<crate::schemas::x::LegacyDrawingHeaderFooter>,
  /// Defines the Picture Class.
  #[sdk(child(qname = "x:picture"))]
  pub picture: Option<crate::schemas::x::Picture>,
  /// Defines the OleObjects Class.
  #[sdk(child(qname = "x:oleObjects"))]
  pub ole_objects: Option<crate::schemas::x::OleObjects>,
  /// Defines the DrawingHeaderFooter Class.
  #[sdk(child(qname = "x:drawingHF"))]
  pub drawing_header_footer: Option<crate::schemas::x::DrawingHeaderFooter>,
  /// Defines the ExtensionList Class.
  #[sdk(child(qname = "x:extLst"))]
  pub extension_list: Option<crate::schemas::x::ExtensionList>,
}
/// Worksheet Sort Map.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xne:worksheetSortMap")]
pub struct WorksheetSortMap {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  /// Row Sort Map
  #[sdk(child(qname = "xne:rowSortMap"))]
  pub row_sort_map: Option<RowSortMap>,
  /// Column Sort Map
  #[sdk(child(qname = "xne:colSortMap"))]
  pub column_sort_map: Option<ColumnSortMap>,
}
/// Defines the ReferenceSequence Class.
pub type ReferenceSequence = Vec<crate::simple_type::StringValue>;
/// Defines the Formula Class.
pub type Formula = crate::simple_type::StringValue;
/// Row Sort Map.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xne:rowSortMap")]
pub struct RowSortMap {
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub r#ref: crate::simple_type::StringValue,
  /// Count
  #[sdk(attr(qname = ":count"))]
  #[sdk(number_range(max = 536870910, min_inclusive = false))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Row.
  #[sdk(child(qname = "xne:row"))]
  pub row_sort_map_item: Vec<RowSortMapItem>,
}
/// Column Sort Map.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xne:colSortMap")]
pub struct ColumnSortMap {
  /// Reference
  #[sdk(attr(qname = ":ref"))]
  pub r#ref: crate::simple_type::StringValue,
  /// Count
  #[sdk(attr(qname = ":count"))]
  #[sdk(number_range(max = 536870910, min_inclusive = false))]
  pub count: Option<crate::simple_type::UInt32Value>,
  /// Column.
  #[sdk(child(qname = "xne:col"))]
  pub column_sort_map_item: Vec<ColumnSortMapItem>,
}
/// Row.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xne:row")]
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
#[sdk(qname = "xne:col")]
pub struct ColumnSortMapItem {
  /// New Value
  #[sdk(attr(qname = ":newVal"))]
  pub new_val: crate::simple_type::UInt32Value,
  /// Old Value
  #[sdk(attr(qname = ":oldVal"))]
  pub old_val: crate::simple_type::UInt32Value,
}
