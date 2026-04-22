//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
pub const PATH_PREFIX: &str = "xl";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct WorkbookPart {
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub root_element: crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook,
  pub custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart>,
  pub calculation_chain_part:
    Option<std::boxed::Box<crate::parts::calculation_chain_part::CalculationChainPart>>,
  pub cell_metadata_part:
    Option<std::boxed::Box<crate::parts::cell_metadata_part::CellMetadataPart>>,
  pub connections_part: Option<std::boxed::Box<crate::parts::connections_part::ConnectionsPart>>,
  pub custom_xml_mappings_part:
    Option<std::boxed::Box<crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart>>,
  pub shared_string_table_part:
    Option<std::boxed::Box<crate::parts::shared_string_table_part::SharedStringTablePart>>,
  pub workbook_revision_header_part: Option<
    std::boxed::Box<crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart>,
  >,
  pub workbook_user_data_part:
    Option<std::boxed::Box<crate::parts::workbook_user_data_part::WorkbookUserDataPart>>,
  pub workbook_styles_part:
    Option<std::boxed::Box<crate::parts::workbook_styles_part::WorkbookStylesPart>>,
  pub theme_part: Option<std::boxed::Box<crate::parts::theme_part::ThemePart>>,
  pub thumbnail_part: Option<std::boxed::Box<crate::parts::thumbnail_part::ThumbnailPart>>,
  pub volatile_dependencies_part:
    Option<std::boxed::Box<crate::parts::volatile_dependencies_part::VolatileDependenciesPart>>,
  pub chartsheet_parts: Vec<crate::parts::chartsheet_part::ChartsheetPart>,
  pub dialogsheet_parts: Vec<crate::parts::dialogsheet_part::DialogsheetPart>,
  pub external_workbook_parts: Vec<crate::parts::external_workbook_part::ExternalWorkbookPart>,
  pub pivot_table_cache_definition_parts:
    Vec<crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart>,
  pub worksheet_parts: Vec<crate::parts::worksheet_part::WorksheetPart>,
  pub excel_attached_toolbars_part:
    Option<std::boxed::Box<crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart>>,
  pub vba_project_part: Option<std::boxed::Box<crate::parts::vba_project_part::VbaProjectPart>>,
  pub macro_sheet_parts: Vec<crate::parts::macro_sheet_part::MacroSheetPart>,
  pub international_macro_sheet_parts:
    Vec<crate::parts::international_macro_sheet_part::InternationalMacroSheetPart>,
  #[cfg(feature = "microsoft365")]
  pub custom_data_properties_parts:
    Vec<crate::parts::custom_data_properties_part::CustomDataPropertiesPart>,
  #[cfg(feature = "microsoft365")]
  pub slicer_cache_parts: Vec<crate::parts::slicer_cache_part::SlicerCachePart>,
  #[cfg(feature = "microsoft365")]
  pub time_line_cache_parts: Vec<crate::parts::time_line_cache_part::TimeLineCachePart>,
  #[cfg(feature = "microsoft365")]
  pub workbook_person_parts: Vec<crate::parts::workbook_person_part::WorkbookPersonPart>,
  #[cfg(feature = "microsoft365")]
  pub rd_rich_value_parts: Vec<crate::parts::rd_rich_value_part::RdRichValuePart>,
  #[cfg(feature = "microsoft365")]
  pub ct_rd_rich_value_structure_parts:
    Vec<crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart>,
  #[cfg(feature = "microsoft365")]
  pub rd_array_parts: Vec<crate::parts::rd_array_part::RdArrayPart>,
  #[cfg(feature = "microsoft365")]
  pub rich_styles_parts: Vec<crate::parts::rich_styles_part::RichStylesPart>,
  #[cfg(feature = "microsoft365")]
  pub rd_supporting_property_bag_parts:
    Vec<crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart>,
  #[cfg(feature = "microsoft365")]
  pub rd_supporting_property_bag_structure_parts: Vec<
    crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart,
  >,
  #[cfg(feature = "microsoft365")]
  pub rd_rich_value_types_parts: Vec<crate::parts::rd_rich_value_types_part::RdRichValueTypesPart>,
  #[cfg(feature = "microsoft365")]
  pub rd_rich_value_web_image_part:
    Option<std::boxed::Box<crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart>>,
  #[cfg(feature = "microsoft365")]
  pub feature_property_bags_part:
    Option<std::boxed::Box<crate::parts::feature_property_bags_part::FeaturePropertyBagsPart>>,
}
