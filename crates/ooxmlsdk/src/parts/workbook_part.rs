//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WorkbookPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook,
  >,
  pub(crate) custom_xml_parts:
    crate::sdk::RepeatedPart<crate::parts::custom_xml_part::CustomXmlPart>,
  pub(crate) calculation_chain_part:
    crate::sdk::OptionalPart<crate::parts::calculation_chain_part::CalculationChainPart>,
  pub(crate) cell_metadata_part:
    crate::sdk::OptionalPart<crate::parts::cell_metadata_part::CellMetadataPart>,
  pub(crate) connections_part:
    crate::sdk::OptionalPart<crate::parts::connections_part::ConnectionsPart>,
  pub(crate) custom_xml_mappings_part:
    crate::sdk::OptionalPart<crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart>,
  pub(crate) shared_string_table_part:
    crate::sdk::OptionalPart<crate::parts::shared_string_table_part::SharedStringTablePart>,
  pub(crate) workbook_revision_header_part: crate::sdk::OptionalPart<
    crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart,
  >,
  pub(crate) workbook_user_data_part:
    crate::sdk::OptionalPart<crate::parts::workbook_user_data_part::WorkbookUserDataPart>,
  pub(crate) workbook_styles_part:
    crate::sdk::OptionalPart<crate::parts::workbook_styles_part::WorkbookStylesPart>,
  pub(crate) theme_part: crate::sdk::OptionalPart<crate::parts::theme_part::ThemePart>,
  pub(crate) thumbnail_part: crate::sdk::OptionalPart<crate::parts::thumbnail_part::ThumbnailPart>,
  pub(crate) volatile_dependencies_part:
    crate::sdk::OptionalPart<crate::parts::volatile_dependencies_part::VolatileDependenciesPart>,
  pub(crate) chartsheet_parts:
    crate::sdk::RepeatedPart<crate::parts::chartsheet_part::ChartsheetPart>,
  pub(crate) dialogsheet_parts:
    crate::sdk::RepeatedPart<crate::parts::dialogsheet_part::DialogsheetPart>,
  pub(crate) external_workbook_parts:
    crate::sdk::RepeatedPart<crate::parts::external_workbook_part::ExternalWorkbookPart>,
  pub(crate) pivot_table_cache_definition_parts: crate::sdk::RepeatedPart<
    crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart,
  >,
  pub(crate) worksheet_parts: crate::sdk::RepeatedPart<crate::parts::worksheet_part::WorksheetPart>,
  pub(crate) excel_attached_toolbars_part:
    crate::sdk::OptionalPart<crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart>,
  pub(crate) vba_project_part:
    crate::sdk::OptionalPart<crate::parts::vba_project_part::VbaProjectPart>,
  pub(crate) macro_sheet_parts:
    crate::sdk::RepeatedPart<crate::parts::macro_sheet_part::MacroSheetPart>,
  pub(crate) international_macro_sheet_parts: crate::sdk::RepeatedPart<
    crate::parts::international_macro_sheet_part::InternationalMacroSheetPart,
  >,
  pub(crate) custom_data_properties_parts:
    crate::sdk::RepeatedPart<crate::parts::custom_data_properties_part::CustomDataPropertiesPart>,
  pub(crate) slicer_cache_parts:
    crate::sdk::RepeatedPart<crate::parts::slicer_cache_part::SlicerCachePart>,
  pub(crate) time_line_cache_parts:
    crate::sdk::RepeatedPart<crate::parts::time_line_cache_part::TimeLineCachePart>,
  pub(crate) workbook_person_parts:
    crate::sdk::RepeatedPart<crate::parts::workbook_person_part::WorkbookPersonPart>,
  pub(crate) rd_rich_value_parts:
    crate::sdk::RepeatedPart<crate::parts::rd_rich_value_part::RdRichValuePart>,
  pub(crate) ct_rd_rich_value_structure_parts:
    crate::sdk::RepeatedPart<crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart>,
  pub(crate) rd_array_parts: crate::sdk::RepeatedPart<crate::parts::rd_array_part::RdArrayPart>,
  pub(crate) rich_styles_parts:
    crate::sdk::RepeatedPart<crate::parts::rich_styles_part::RichStylesPart>,
  pub(crate) rd_supporting_property_bag_parts: crate::sdk::RepeatedPart<
    crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart,
  >,
  pub(crate) rd_supporting_property_bag_structure_parts: crate::sdk::RepeatedPart<
    crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart,
  >,
  pub(crate) rd_rich_value_types_parts:
    crate::sdk::RepeatedPart<crate::parts::rd_rich_value_types_part::RdRichValueTypesPart>,
  pub(crate) rd_rich_value_web_image_part:
    crate::sdk::OptionalPart<crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart>,
  pub(crate) feature_property_bags_part:
    crate::sdk::OptionalPart<crate::parts::feature_property_bags_part::FeaturePropertyBagsPart>,
}
