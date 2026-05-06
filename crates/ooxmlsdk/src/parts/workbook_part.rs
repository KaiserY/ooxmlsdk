//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
pub const PATH_PREFIX: &str = "xl";
pub const CONTENT_TYPE: &str = "";
pub const TARGET_NAME: &str = "workbook";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WorkbookPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl WorkbookPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook,
    WorkbookPart,
    as_workbook_part,
    as_workbook_part_mut
  );
  crate::sdk_part_child_methods! {
      repeated custom_xml_parts => crate ::parts::custom_xml_part::CustomXmlPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml";
      optional calculation_chain_part => crate
      ::parts::calculation_chain_part::CalculationChainPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain";
      optional cell_metadata_part => crate
      ::parts::cell_metadata_part::CellMetadataPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sheetMetadata";
      optional connections_part => crate ::parts::connections_part::ConnectionsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/connections";
      optional custom_xml_mappings_part => crate
      ::parts::custom_xml_mappings_part::CustomXmlMappingsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps";
      optional shared_string_table_part => crate
      ::parts::shared_string_table_part::SharedStringTablePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings";
      optional workbook_revision_header_part => crate
      ::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders";
      optional workbook_user_data_part => crate
      ::parts::workbook_user_data_part::WorkbookUserDataPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/usernames";
      optional workbook_styles_part => crate
      ::parts::workbook_styles_part::WorkbookStylesPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
      optional theme_part => crate ::parts::theme_part::ThemePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme";
      optional thumbnail_part => crate ::parts::thumbnail_part::ThumbnailPart,
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail";
      optional volatile_dependencies_part => crate
      ::parts::volatile_dependencies_part::VolatileDependenciesPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/volatileDependencies";
      repeated chartsheet_parts => crate ::parts::chartsheet_part::ChartsheetPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet";
      repeated dialogsheet_parts => crate ::parts::dialogsheet_part::DialogsheetPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet";
      repeated external_workbook_parts => crate
      ::parts::external_workbook_part::ExternalWorkbookPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink";
      repeated pivot_table_cache_definition_parts => crate
      ::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition";
      repeated worksheet_parts => crate ::parts::worksheet_part::WorksheetPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet";
      optional excel_attached_toolbars_part => crate
      ::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart,
      "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars";
      optional vba_project_part => crate ::parts::vba_project_part::VbaProjectPart,
      "http://schemas.microsoft.com/office/2006/relationships/vbaProject"; repeated
      macro_sheet_parts => crate ::parts::macro_sheet_part::MacroSheetPart,
      "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet"; repeated
      international_macro_sheet_parts => crate
      ::parts::international_macro_sheet_part::InternationalMacroSheetPart,
      "http://schemas.microsoft.com/office/2006/relationships/xlIntlMacrosheet";
      repeated custom_data_properties_parts => crate
      ::parts::custom_data_properties_part::CustomDataPropertiesPart,
      "http://schemas.microsoft.com/office/2007/relationships/customDataProps";
      repeated slicer_cache_parts => crate ::parts::slicer_cache_part::SlicerCachePart,
      "http://schemas.microsoft.com/office/2007/relationships/slicerCache"; repeated
      time_line_cache_parts => crate ::parts::time_line_cache_part::TimeLineCachePart,
      "http://schemas.microsoft.com/office/2011/relationships/timelineCache"; repeated
      workbook_person_parts => crate ::parts::workbook_person_part::WorkbookPersonPart,
      "http://schemas.microsoft.com/office/2017/10/relationships/person"; repeated
      rd_rich_value_parts => crate ::parts::rd_rich_value_part::RdRichValuePart,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue"; repeated
      ct_rd_rich_value_structure_parts => crate
      ::parts::rd_rich_value_structure_part::RdRichValueStructurePart,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure";
      repeated rd_array_parts => crate ::parts::rd_array_part::RdArrayPart,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdArray"; repeated
      rich_styles_parts => crate ::parts::rich_styles_part::RichStylesPart,
      "http://schemas.microsoft.com/office/2017/06/relationships/richStyles"; repeated
      rd_supporting_property_bag_parts => crate
      ::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag";
      repeated rd_supporting_property_bag_structure_parts => crate
      ::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure";
      repeated rd_rich_value_types_parts => crate
      ::parts::rd_rich_value_types_part::RdRichValueTypesPart,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes";
      optional rd_rich_value_web_image_part => crate
      ::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart,
      "http://schemas.microsoft.com/office/2020/07/relationships/rdRichValueWebImage";
      optional feature_property_bags_part => crate
      ::parts::feature_property_bags_part::FeaturePropertyBagsPart,
      "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag";
  }
}
