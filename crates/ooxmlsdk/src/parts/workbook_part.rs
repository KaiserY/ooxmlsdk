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
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml"
  ))]
  pub(crate) custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain"
  ))]
  pub(crate) calculation_chain_part:
    Option<Box<crate::parts::calculation_chain_part::CalculationChainPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sheetMetadata"
  ))]
  pub(crate) cell_metadata_part: Option<Box<crate::parts::cell_metadata_part::CellMetadataPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/connections"
  ))]
  pub(crate) connections_part: Option<Box<crate::parts::connections_part::ConnectionsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps"
  ))]
  pub(crate) custom_xml_mappings_part:
    Option<Box<crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings"
  ))]
  pub(crate) shared_string_table_part:
    Option<Box<crate::parts::shared_string_table_part::SharedStringTablePart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders"
  ))]
  pub(crate) workbook_revision_header_part:
    Option<Box<crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/usernames"
  ))]
  pub(crate) workbook_user_data_part:
    Option<Box<crate::parts::workbook_user_data_part::WorkbookUserDataPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles"
  ))]
  pub(crate) workbook_styles_part:
    Option<Box<crate::parts::workbook_styles_part::WorkbookStylesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme"
  ))]
  pub(crate) theme_part: Option<Box<crate::parts::theme_part::ThemePart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail"
  ))]
  pub(crate) thumbnail_part: Option<Box<crate::parts::thumbnail_part::ThumbnailPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/volatileDependencies"
  ))]
  pub(crate) volatile_dependencies_part:
    Option<Box<crate::parts::volatile_dependencies_part::VolatileDependenciesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet"
  ))]
  pub(crate) chartsheet_parts: Vec<crate::parts::chartsheet_part::ChartsheetPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet"
  ))]
  pub(crate) dialogsheet_parts: Vec<crate::parts::dialogsheet_part::DialogsheetPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink"
  ))]
  pub(crate) external_workbook_parts:
    Vec<crate::parts::external_workbook_part::ExternalWorkbookPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition"
  ))]
  pub(crate) pivot_table_cache_definition_parts:
    Vec<crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet"
  ))]
  pub(crate) worksheet_parts: Vec<crate::parts::worksheet_part::WorksheetPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars"
  ))]
  pub(crate) excel_attached_toolbars_part:
    Option<Box<crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/vbaProject"
  ))]
  pub(crate) vba_project_part: Option<Box<crate::parts::vba_project_part::VbaProjectPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet"
  ))]
  pub(crate) macro_sheet_parts: Vec<crate::parts::macro_sheet_part::MacroSheetPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/xlIntlMacrosheet"
  ))]
  pub(crate) international_macro_sheet_parts:
    Vec<crate::parts::international_macro_sheet_part::InternationalMacroSheetPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/customDataProps"
  ))]
  pub(crate) custom_data_properties_parts:
    Vec<crate::parts::custom_data_properties_part::CustomDataPropertiesPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/slicerCache"
  ))]
  pub(crate) slicer_cache_parts: Vec<crate::parts::slicer_cache_part::SlicerCachePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/timelineCache"
  ))]
  pub(crate) time_line_cache_parts: Vec<crate::parts::time_line_cache_part::TimeLineCachePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/10/relationships/person"
  ))]
  pub(crate) workbook_person_parts: Vec<crate::parts::workbook_person_part::WorkbookPersonPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue"
  ))]
  pub(crate) rd_rich_value_parts: Vec<crate::parts::rd_rich_value_part::RdRichValuePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure"
  ))]
  pub(crate) ct_rd_rich_value_structure_parts:
    Vec<crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdArray"
  ))]
  pub(crate) rd_array_parts: Vec<crate::parts::rd_array_part::RdArrayPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/richStyles"
  ))]
  pub(crate) rich_styles_parts: Vec<crate::parts::rich_styles_part::RichStylesPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag"
  ))]
  pub(crate) rd_supporting_property_bag_parts:
    Vec<crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure"
  ))]
  pub(crate) rd_supporting_property_bag_structure_parts: Vec<
    crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes"
  ))]
  pub(crate) rd_rich_value_types_parts:
    Vec<crate::parts::rd_rich_value_types_part::RdRichValueTypesPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2020/07/relationships/rdRichValueWebImage"
  ))]
  pub(crate) rd_rich_value_web_image_part:
    Option<Box<crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag"
  ))]
  pub(crate) feature_property_bags_part:
    Option<Box<crate::parts::feature_property_bags_part::FeaturePropertyBagsPart>>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<Box<str>>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl WorkbookPart {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] = &[
    crate::sdk::PartChildDescriptor::new(
      "custom_xml_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
      "crate::parts::custom_xml_part::CustomXmlPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "calculation_chain_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain",
      "crate::parts::calculation_chain_part::CalculationChainPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "cell_metadata_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sheetMetadata",
      "crate::parts::cell_metadata_part::CellMetadataPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "connections_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/connections",
      "crate::parts::connections_part::ConnectionsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "custom_xml_mappings_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps",
      "crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "shared_string_table_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings",
      "crate::parts::shared_string_table_part::SharedStringTablePart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "workbook_revision_header_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders",
      "crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "workbook_user_data_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/usernames",
      "crate::parts::workbook_user_data_part::WorkbookUserDataPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "workbook_styles_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
      "crate::parts::workbook_styles_part::WorkbookStylesPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "theme_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
      "crate::parts::theme_part::ThemePart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "thumbnail_part",
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail",
      "crate::parts::thumbnail_part::ThumbnailPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "volatile_dependencies_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/volatileDependencies",
      "crate::parts::volatile_dependencies_part::VolatileDependenciesPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "chartsheet_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet",
      "crate::parts::chartsheet_part::ChartsheetPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "dialogsheet_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet",
      "crate::parts::dialogsheet_part::DialogsheetPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "external_workbook_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink",
      "crate::parts::external_workbook_part::ExternalWorkbookPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "pivot_table_cache_definition_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition",
      "crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "worksheet_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet",
      "crate::parts::worksheet_part::WorksheetPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "excel_attached_toolbars_part",
      "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars",
      "crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "vba_project_part",
      "http://schemas.microsoft.com/office/2006/relationships/vbaProject",
      "crate::parts::vba_project_part::VbaProjectPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "macro_sheet_parts",
      "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet",
      "crate::parts::macro_sheet_part::MacroSheetPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "international_macro_sheet_parts",
      "http://schemas.microsoft.com/office/2006/relationships/xlIntlMacrosheet",
      "crate::parts::international_macro_sheet_part::InternationalMacroSheetPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "custom_data_properties_parts",
      "http://schemas.microsoft.com/office/2007/relationships/customDataProps",
      "crate::parts::custom_data_properties_part::CustomDataPropertiesPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "slicer_cache_parts",
      "http://schemas.microsoft.com/office/2007/relationships/slicerCache",
      "crate::parts::slicer_cache_part::SlicerCachePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "time_line_cache_parts",
      "http://schemas.microsoft.com/office/2011/relationships/timelineCache",
      "crate::parts::time_line_cache_part::TimeLineCachePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "workbook_person_parts",
      "http://schemas.microsoft.com/office/2017/10/relationships/person",
      "crate::parts::workbook_person_part::WorkbookPersonPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "rd_rich_value_parts",
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue",
      "crate::parts::rd_rich_value_part::RdRichValuePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "ct_rd_rich_value_structure_parts",
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure",
      "crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "rd_array_parts",
      "http://schemas.microsoft.com/office/2017/06/relationships/rdArray",
      "crate::parts::rd_array_part::RdArrayPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "rich_styles_parts",
      "http://schemas.microsoft.com/office/2017/06/relationships/richStyles",
      "crate::parts::rich_styles_part::RichStylesPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "rd_supporting_property_bag_parts",
      "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag",
      "crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "rd_supporting_property_bag_structure_parts",
      "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure",
      "crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "rd_rich_value_types_parts",
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes",
      "crate::parts::rd_rich_value_types_part::RdRichValueTypesPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "rd_rich_value_web_image_part",
      "http://schemas.microsoft.com/office/2020/07/relationships/rdRichValueWebImage",
      "crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "feature_property_bags_part",
      "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag",
      "crate::parts::feature_property_bags_part::FeaturePropertyBagsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
  ];
}
