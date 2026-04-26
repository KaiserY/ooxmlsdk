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
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
    kind = "repeated"
  ))]
  pub(crate) custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain",
    kind = "optional"
  ))]
  pub(crate) calculation_chain_part:
    Option<Box<crate::parts::calculation_chain_part::CalculationChainPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sheetMetadata",
    kind = "optional"
  ))]
  pub(crate) cell_metadata_part: Option<Box<crate::parts::cell_metadata_part::CellMetadataPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/connections",
    kind = "optional"
  ))]
  pub(crate) connections_part: Option<Box<crate::parts::connections_part::ConnectionsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps",
    kind = "optional"
  ))]
  pub(crate) custom_xml_mappings_part:
    Option<Box<crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings",
    kind = "optional"
  ))]
  pub(crate) shared_string_table_part:
    Option<Box<crate::parts::shared_string_table_part::SharedStringTablePart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders",
    kind = "optional"
  ))]
  pub(crate) workbook_revision_header_part:
    Option<Box<crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/usernames",
    kind = "optional"
  ))]
  pub(crate) workbook_user_data_part:
    Option<Box<crate::parts::workbook_user_data_part::WorkbookUserDataPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
    kind = "optional"
  ))]
  pub(crate) workbook_styles_part:
    Option<Box<crate::parts::workbook_styles_part::WorkbookStylesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
    kind = "optional"
  ))]
  pub(crate) theme_part: Option<Box<crate::parts::theme_part::ThemePart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail",
    kind = "optional"
  ))]
  pub(crate) thumbnail_part: Option<Box<crate::parts::thumbnail_part::ThumbnailPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/volatileDependencies",
    kind = "optional"
  ))]
  pub(crate) volatile_dependencies_part:
    Option<Box<crate::parts::volatile_dependencies_part::VolatileDependenciesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet",
    kind = "repeated"
  ))]
  pub(crate) chartsheet_parts: Vec<crate::parts::chartsheet_part::ChartsheetPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet",
    kind = "repeated"
  ))]
  pub(crate) dialogsheet_parts: Vec<crate::parts::dialogsheet_part::DialogsheetPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink",
    kind = "repeated"
  ))]
  pub(crate) external_workbook_parts:
    Vec<crate::parts::external_workbook_part::ExternalWorkbookPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition",
    kind = "repeated"
  ))]
  pub(crate) pivot_table_cache_definition_parts:
    Vec<crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet",
    kind = "repeated"
  ))]
  pub(crate) worksheet_parts: Vec<crate::parts::worksheet_part::WorksheetPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars",
    kind = "optional"
  ))]
  pub(crate) excel_attached_toolbars_part:
    Option<Box<crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/vbaProject",
    kind = "optional"
  ))]
  pub(crate) vba_project_part: Option<Box<crate::parts::vba_project_part::VbaProjectPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet",
    kind = "repeated"
  ))]
  pub(crate) macro_sheet_parts: Vec<crate::parts::macro_sheet_part::MacroSheetPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/xlIntlMacrosheet",
    kind = "repeated"
  ))]
  pub(crate) international_macro_sheet_parts:
    Vec<crate::parts::international_macro_sheet_part::InternationalMacroSheetPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/customDataProps",
    kind = "repeated"
  ))]
  pub(crate) custom_data_properties_parts:
    Vec<crate::parts::custom_data_properties_part::CustomDataPropertiesPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/slicerCache",
    kind = "repeated"
  ))]
  pub(crate) slicer_cache_parts: Vec<crate::parts::slicer_cache_part::SlicerCachePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/timelineCache",
    kind = "repeated"
  ))]
  pub(crate) time_line_cache_parts: Vec<crate::parts::time_line_cache_part::TimeLineCachePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/10/relationships/person",
    kind = "repeated"
  ))]
  pub(crate) workbook_person_parts: Vec<crate::parts::workbook_person_part::WorkbookPersonPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue",
    kind = "repeated"
  ))]
  pub(crate) rd_rich_value_parts: Vec<crate::parts::rd_rich_value_part::RdRichValuePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure",
    kind = "repeated"
  ))]
  pub(crate) ct_rd_rich_value_structure_parts:
    Vec<crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdArray",
    kind = "repeated"
  ))]
  pub(crate) rd_array_parts: Vec<crate::parts::rd_array_part::RdArrayPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/richStyles",
    kind = "repeated"
  ))]
  pub(crate) rich_styles_parts: Vec<crate::parts::rich_styles_part::RichStylesPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag",
    kind = "repeated"
  ))]
  pub(crate) rd_supporting_property_bag_parts:
    Vec<crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure",
    kind = "repeated"
  ))]
  pub(crate) rd_supporting_property_bag_structure_parts: Vec<
    crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes",
    kind = "repeated"
  ))]
  pub(crate) rd_rich_value_types_parts:
    Vec<crate::parts::rd_rich_value_types_part::RdRichValueTypesPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2020/07/relationships/rdRichValueWebImage",
    kind = "optional"
  ))]
  pub(crate) rd_rich_value_web_image_part:
    Option<Box<crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag",
    kind = "optional"
  ))]
  pub(crate) feature_property_bags_part:
    Option<Box<crate::parts::feature_property_bags_part::FeaturePropertyBagsPart>>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
