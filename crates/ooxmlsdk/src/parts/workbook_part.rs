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
  #[sdk(part_root(accessor = "as_workbook_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook,
  >,
  #[sdk(part_child(relationship_type = RelationshipCustomXml))]
  pub(crate) custom_xml_parts:
    crate::sdk::RepeatedPart<crate::parts::custom_xml_part::CustomXmlPart>,
  #[sdk(part_child(relationship_type = RelationshipCalcChain))]
  pub(crate) calculation_chain_part:
    crate::sdk::OptionalPart<crate::parts::calculation_chain_part::CalculationChainPart>,
  #[sdk(part_child(relationship_type = RelationshipSheetMetadata))]
  pub(crate) cell_metadata_part:
    crate::sdk::OptionalPart<crate::parts::cell_metadata_part::CellMetadataPart>,
  #[sdk(part_child(relationship_type = RelationshipConnections))]
  pub(crate) connections_part:
    crate::sdk::OptionalPart<crate::parts::connections_part::ConnectionsPart>,
  #[sdk(part_child(relationship_type = RelationshipXmlMaps))]
  pub(crate) custom_xml_mappings_part:
    crate::sdk::OptionalPart<crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart>,
  #[sdk(part_child(relationship_type = RelationshipSharedStrings))]
  pub(crate) shared_string_table_part:
    crate::sdk::OptionalPart<crate::parts::shared_string_table_part::SharedStringTablePart>,
  #[sdk(part_child(relationship_type = RelationshipRevisionHeaders))]
  pub(crate) workbook_revision_header_part: crate::sdk::OptionalPart<
    crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart,
  >,
  #[sdk(part_child(relationship_type = RelationshipUsernames))]
  pub(crate) workbook_user_data_part:
    crate::sdk::OptionalPart<crate::parts::workbook_user_data_part::WorkbookUserDataPart>,
  #[sdk(part_child(relationship_type = RelationshipStyles))]
  pub(crate) workbook_styles_part:
    crate::sdk::OptionalPart<crate::parts::workbook_styles_part::WorkbookStylesPart>,
  #[sdk(part_child(relationship_type = RelationshipTheme))]
  pub(crate) theme_part: crate::sdk::OptionalPart<crate::parts::theme_part::ThemePart>,
  #[sdk(part_child(relationship_type = RelationshipThumbnail))]
  pub(crate) thumbnail_part: crate::sdk::OptionalPart<crate::parts::thumbnail_part::ThumbnailPart>,
  #[sdk(part_child(relationship_type = RelationshipVolatileDependencies))]
  pub(crate) volatile_dependencies_part:
    crate::sdk::OptionalPart<crate::parts::volatile_dependencies_part::VolatileDependenciesPart>,
  #[sdk(part_child(relationship_type = RelationshipChartsheet))]
  pub(crate) chartsheet_parts:
    crate::sdk::RepeatedPart<crate::parts::chartsheet_part::ChartsheetPart>,
  #[sdk(part_child(relationship_type = RelationshipDialogsheet))]
  pub(crate) dialogsheet_parts:
    crate::sdk::RepeatedPart<crate::parts::dialogsheet_part::DialogsheetPart>,
  #[sdk(part_child(relationship_type = RelationshipExternalLink))]
  pub(crate) external_workbook_parts:
    crate::sdk::RepeatedPart<crate::parts::external_workbook_part::ExternalWorkbookPart>,
  #[sdk(part_child(relationship_type = RelationshipPivotCacheDefinition))]
  pub(crate) pivot_table_cache_definition_parts: crate::sdk::RepeatedPart<
    crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart,
  >,
  #[sdk(part_child(relationship_type = RelationshipWorksheet))]
  pub(crate) worksheet_parts: crate::sdk::RepeatedPart<crate::parts::worksheet_part::WorksheetPart>,
  #[sdk(part_child(relationship_type = RelationshipAttachedToolbars))]
  pub(crate) excel_attached_toolbars_part:
    crate::sdk::OptionalPart<crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart>,
  #[sdk(part_child(relationship_type = RelationshipVbaProject))]
  pub(crate) vba_project_part:
    crate::sdk::OptionalPart<crate::parts::vba_project_part::VbaProjectPart>,
  #[sdk(part_child(relationship_type = RelationshipXlMacrosheet))]
  pub(crate) macro_sheet_parts:
    crate::sdk::RepeatedPart<crate::parts::macro_sheet_part::MacroSheetPart>,
  #[sdk(part_child(relationship_type = RelationshipXlIntlMacrosheet))]
  pub(crate) international_macro_sheet_parts: crate::sdk::RepeatedPart<
    crate::parts::international_macro_sheet_part::InternationalMacroSheetPart,
  >,
  #[sdk(part_child(relationship_type = RelationshipCustomDataProps))]
  pub(crate) custom_data_properties_parts:
    crate::sdk::RepeatedPart<crate::parts::custom_data_properties_part::CustomDataPropertiesPart>,
  #[sdk(part_child(relationship_type = RelationshipSlicerCache))]
  pub(crate) slicer_cache_parts:
    crate::sdk::RepeatedPart<crate::parts::slicer_cache_part::SlicerCachePart>,
  #[sdk(part_child(relationship_type = RelationshipTimelineCache))]
  pub(crate) time_line_cache_parts:
    crate::sdk::RepeatedPart<crate::parts::time_line_cache_part::TimeLineCachePart>,
  #[sdk(part_child(relationship_type = RelationshipPerson))]
  pub(crate) workbook_person_parts:
    crate::sdk::RepeatedPart<crate::parts::workbook_person_part::WorkbookPersonPart>,
  #[sdk(part_child(relationship_type = RelationshipRdRichValue))]
  pub(crate) rd_rich_value_parts:
    crate::sdk::RepeatedPart<crate::parts::rd_rich_value_part::RdRichValuePart>,
  #[sdk(part_child(relationship_type = RelationshipRdRichValueStructure))]
  pub(crate) ct_rd_rich_value_structure_parts:
    crate::sdk::RepeatedPart<crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart>,
  #[sdk(part_child(relationship_type = RelationshipRdArray))]
  pub(crate) rd_array_parts: crate::sdk::RepeatedPart<crate::parts::rd_array_part::RdArrayPart>,
  #[sdk(part_child(relationship_type = RelationshipRichStyles))]
  pub(crate) rich_styles_parts:
    crate::sdk::RepeatedPart<crate::parts::rich_styles_part::RichStylesPart>,
  #[sdk(part_child(relationship_type = RelationshipRdSupportingPropertyBag))]
  pub(crate) rd_supporting_property_bag_parts: crate::sdk::RepeatedPart<
    crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart,
  >,
  #[sdk(part_child(relationship_type = RelationshipRdSupportingPropertyBagStructure))]
  pub(crate) rd_supporting_property_bag_structure_parts: crate::sdk::RepeatedPart<
    crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart,
  >,
  #[sdk(part_child(relationship_type = RelationshipRdRichValueTypes))]
  pub(crate) rd_rich_value_types_parts:
    crate::sdk::RepeatedPart<crate::parts::rd_rich_value_types_part::RdRichValueTypesPart>,
  #[sdk(part_child(relationship_type = RelationshipRdRichValueWebImage))]
  pub(crate) rd_rich_value_web_image_part:
    crate::sdk::OptionalPart<crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart>,
  #[sdk(part_child(relationship_type = RelationshipFeaturePropertyBag))]
  pub(crate) feature_property_bags_part:
    crate::sdk::OptionalPart<crate::parts::feature_property_bags_part::FeaturePropertyBagsPart>,
}
