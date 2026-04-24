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
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WorkbookPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_workbook_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml"
  ))]
  pub(crate) custom_xml_parts:
    crate::sdk::RepeatedPart<crate::parts::custom_xml_part::CustomXmlPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain"
  ))]
  pub(crate) calculation_chain_part:
    crate::sdk::OptionalPart<crate::parts::calculation_chain_part::CalculationChainPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sheetMetadata"
  ))]
  pub(crate) cell_metadata_part:
    crate::sdk::OptionalPart<crate::parts::cell_metadata_part::CellMetadataPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/connections"
  ))]
  pub(crate) connections_part:
    crate::sdk::OptionalPart<crate::parts::connections_part::ConnectionsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps"
  ))]
  pub(crate) custom_xml_mappings_part:
    crate::sdk::OptionalPart<crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings"
  ))]
  pub(crate) shared_string_table_part:
    crate::sdk::OptionalPart<crate::parts::shared_string_table_part::SharedStringTablePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders"
  ))]
  pub(crate) workbook_revision_header_part: crate::sdk::OptionalPart<
    crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/usernames"
  ))]
  pub(crate) workbook_user_data_part:
    crate::sdk::OptionalPart<crate::parts::workbook_user_data_part::WorkbookUserDataPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles"
  ))]
  pub(crate) workbook_styles_part:
    crate::sdk::OptionalPart<crate::parts::workbook_styles_part::WorkbookStylesPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme"
  ))]
  pub(crate) theme_part: crate::sdk::OptionalPart<crate::parts::theme_part::ThemePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail"
  ))]
  pub(crate) thumbnail_part: crate::sdk::OptionalPart<crate::parts::thumbnail_part::ThumbnailPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/volatileDependencies"
  ))]
  pub(crate) volatile_dependencies_part:
    crate::sdk::OptionalPart<crate::parts::volatile_dependencies_part::VolatileDependenciesPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet"
  ))]
  pub(crate) chartsheet_parts:
    crate::sdk::RepeatedPart<crate::parts::chartsheet_part::ChartsheetPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet"
  ))]
  pub(crate) dialogsheet_parts:
    crate::sdk::RepeatedPart<crate::parts::dialogsheet_part::DialogsheetPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink"
  ))]
  pub(crate) external_workbook_parts:
    crate::sdk::RepeatedPart<crate::parts::external_workbook_part::ExternalWorkbookPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition"
  ))]
  pub(crate) pivot_table_cache_definition_parts: crate::sdk::RepeatedPart<
    crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet"
  ))]
  pub(crate) worksheet_parts: crate::sdk::RepeatedPart<crate::parts::worksheet_part::WorksheetPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars"
  ))]
  pub(crate) excel_attached_toolbars_part:
    crate::sdk::OptionalPart<crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/vbaProject"
  ))]
  pub(crate) vba_project_part:
    crate::sdk::OptionalPart<crate::parts::vba_project_part::VbaProjectPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet"
  ))]
  pub(crate) macro_sheet_parts:
    crate::sdk::RepeatedPart<crate::parts::macro_sheet_part::MacroSheetPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/xlIntlMacrosheet"
  ))]
  pub(crate) international_macro_sheet_parts: crate::sdk::RepeatedPart<
    crate::parts::international_macro_sheet_part::InternationalMacroSheetPart,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/customDataProps"
  ))]
  pub(crate) custom_data_properties_parts:
    crate::sdk::RepeatedPart<crate::parts::custom_data_properties_part::CustomDataPropertiesPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/slicerCache"
  ))]
  pub(crate) slicer_cache_parts:
    crate::sdk::RepeatedPart<crate::parts::slicer_cache_part::SlicerCachePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/timelineCache"
  ))]
  pub(crate) time_line_cache_parts:
    crate::sdk::RepeatedPart<crate::parts::time_line_cache_part::TimeLineCachePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/10/relationships/person"
  ))]
  pub(crate) workbook_person_parts:
    crate::sdk::RepeatedPart<crate::parts::workbook_person_part::WorkbookPersonPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue"
  ))]
  pub(crate) rd_rich_value_parts:
    crate::sdk::RepeatedPart<crate::parts::rd_rich_value_part::RdRichValuePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure"
  ))]
  pub(crate) ct_rd_rich_value_structure_parts:
    crate::sdk::RepeatedPart<crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdArray"
  ))]
  pub(crate) rd_array_parts: crate::sdk::RepeatedPart<crate::parts::rd_array_part::RdArrayPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/richStyles"
  ))]
  pub(crate) rich_styles_parts:
    crate::sdk::RepeatedPart<crate::parts::rich_styles_part::RichStylesPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag"
  ))]
  pub(crate) rd_supporting_property_bag_parts: crate::sdk::RepeatedPart<
    crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure"
  ))]
  pub(crate) rd_supporting_property_bag_structure_parts: crate::sdk::RepeatedPart<
    crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes"
  ))]
  pub(crate) rd_rich_value_types_parts:
    crate::sdk::RepeatedPart<crate::parts::rd_rich_value_types_part::RdRichValueTypesPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2020/07/relationships/rdRichValueWebImage"
  ))]
  pub(crate) rd_rich_value_web_image_part:
    crate::sdk::OptionalPart<crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag"
  ))]
  pub(crate) feature_property_bags_part:
    crate::sdk::OptionalPart<crate::parts::feature_property_bags_part::FeaturePropertyBagsPart>,
}
