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
  #[sdk(part_rid)]
  pub r_id: String,
  #[sdk(part_relationships)]
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  #[sdk(part_rels_path)]
  pub rels_path: String,
  #[sdk(part_inner_path)]
  pub inner_path: String,
  #[sdk(part_root)]
  pub root_element: crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
    kind = "repeated"
  ))]
  pub custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain",
    kind = "optional"
  ))]
  pub calculation_chain_part:
    Option<std::boxed::Box<crate::parts::calculation_chain_part::CalculationChainPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sheetMetadata",
    kind = "optional"
  ))]
  pub cell_metadata_part:
    Option<std::boxed::Box<crate::parts::cell_metadata_part::CellMetadataPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/connections",
    kind = "optional"
  ))]
  pub connections_part: Option<std::boxed::Box<crate::parts::connections_part::ConnectionsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps",
    kind = "optional"
  ))]
  pub custom_xml_mappings_part:
    Option<std::boxed::Box<crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings",
    kind = "optional"
  ))]
  pub shared_string_table_part:
    Option<std::boxed::Box<crate::parts::shared_string_table_part::SharedStringTablePart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders",
    kind = "optional"
  ))]
  pub workbook_revision_header_part: Option<
    std::boxed::Box<crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart>,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/usernames",
    kind = "optional"
  ))]
  pub workbook_user_data_part:
    Option<std::boxed::Box<crate::parts::workbook_user_data_part::WorkbookUserDataPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
    kind = "optional"
  ))]
  pub workbook_styles_part:
    Option<std::boxed::Box<crate::parts::workbook_styles_part::WorkbookStylesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
    kind = "optional"
  ))]
  pub theme_part: Option<std::boxed::Box<crate::parts::theme_part::ThemePart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail",
    kind = "optional"
  ))]
  pub thumbnail_part: Option<std::boxed::Box<crate::parts::thumbnail_part::ThumbnailPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/volatileDependencies",
    kind = "optional"
  ))]
  pub volatile_dependencies_part:
    Option<std::boxed::Box<crate::parts::volatile_dependencies_part::VolatileDependenciesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet",
    kind = "repeated"
  ))]
  pub chartsheet_parts: Vec<crate::parts::chartsheet_part::ChartsheetPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet",
    kind = "repeated"
  ))]
  pub dialogsheet_parts: Vec<crate::parts::dialogsheet_part::DialogsheetPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink",
    kind = "repeated"
  ))]
  pub external_workbook_parts: Vec<crate::parts::external_workbook_part::ExternalWorkbookPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition",
    kind = "repeated"
  ))]
  pub pivot_table_cache_definition_parts:
    Vec<crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet",
    kind = "repeated"
  ))]
  pub worksheet_parts: Vec<crate::parts::worksheet_part::WorksheetPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars",
    kind = "optional"
  ))]
  pub excel_attached_toolbars_part:
    Option<std::boxed::Box<crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/vbaProject",
    kind = "optional"
  ))]
  pub vba_project_part: Option<std::boxed::Box<crate::parts::vba_project_part::VbaProjectPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet",
    kind = "repeated"
  ))]
  pub macro_sheet_parts: Vec<crate::parts::macro_sheet_part::MacroSheetPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/xlIntlMacrosheet",
    kind = "repeated"
  ))]
  pub international_macro_sheet_parts:
    Vec<crate::parts::international_macro_sheet_part::InternationalMacroSheetPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/customDataProps",
    kind = "repeated"
  ))]
  pub custom_data_properties_parts:
    Vec<crate::parts::custom_data_properties_part::CustomDataPropertiesPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/slicerCache",
    kind = "repeated"
  ))]
  pub slicer_cache_parts: Vec<crate::parts::slicer_cache_part::SlicerCachePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/timelineCache",
    kind = "repeated"
  ))]
  pub time_line_cache_parts: Vec<crate::parts::time_line_cache_part::TimeLineCachePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/10/relationships/person",
    kind = "repeated"
  ))]
  pub workbook_person_parts: Vec<crate::parts::workbook_person_part::WorkbookPersonPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue",
    kind = "repeated"
  ))]
  pub rd_rich_value_parts: Vec<crate::parts::rd_rich_value_part::RdRichValuePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure",
    kind = "repeated"
  ))]
  pub ct_rd_rich_value_structure_parts:
    Vec<crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdArray",
    kind = "repeated"
  ))]
  pub rd_array_parts: Vec<crate::parts::rd_array_part::RdArrayPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/richStyles",
    kind = "repeated"
  ))]
  pub rich_styles_parts: Vec<crate::parts::rich_styles_part::RichStylesPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag",
    kind = "repeated"
  ))]
  pub rd_supporting_property_bag_parts:
    Vec<crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure",
    kind = "repeated"
  ))]
  pub rd_supporting_property_bag_structure_parts: Vec<
    crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes",
    kind = "repeated"
  ))]
  pub rd_rich_value_types_parts: Vec<crate::parts::rd_rich_value_types_part::RdRichValueTypesPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2020/07/relationships/rdRichValueWebImage",
    kind = "optional"
  ))]
  pub rd_rich_value_web_image_part:
    Option<std::boxed::Box<crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag",
    kind = "optional"
  ))]
  pub feature_property_bags_part:
    Option<std::boxed::Box<crate::parts::feature_property_bags_part::FeaturePropertyBagsPart>>,
}
