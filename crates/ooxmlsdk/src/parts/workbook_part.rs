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
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[
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
  crate::sdk::PartChildDescriptor::new(
    "custom_data_properties_parts",
    "http://schemas.microsoft.com/office/2007/relationships/customDataProps",
    "crate::parts::custom_data_properties_part::CustomDataPropertiesPart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "slicer_cache_parts",
    "http://schemas.microsoft.com/office/2007/relationships/slicerCache",
    "crate::parts::slicer_cache_part::SlicerCachePart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "time_line_cache_parts",
    "http://schemas.microsoft.com/office/2011/relationships/timelineCache",
    "crate::parts::time_line_cache_part::TimeLineCachePart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "workbook_person_parts",
    "http://schemas.microsoft.com/office/2017/10/relationships/person",
    "crate::parts::workbook_person_part::WorkbookPersonPart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "rd_rich_value_parts",
    "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue",
    "crate::parts::rd_rich_value_part::RdRichValuePart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "ct_rd_rich_value_structure_parts",
    "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure",
    "crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "rd_array_parts",
    "http://schemas.microsoft.com/office/2017/06/relationships/rdArray",
    "crate::parts::rd_array_part::RdArrayPart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "rich_styles_parts",
    "http://schemas.microsoft.com/office/2017/06/relationships/richStyles",
    "crate::parts::rich_styles_part::RichStylesPart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "rd_supporting_property_bag_parts",
    "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag",
    "crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "rd_supporting_property_bag_structure_parts",
    "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure",
    "crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "rd_rich_value_types_parts",
    "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes",
    "crate::parts::rd_rich_value_types_part::RdRichValueTypesPart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "rd_rich_value_web_image_part",
    "http://schemas.microsoft.com/office/2020/07/relationships/rdRichValueWebImage",
    "crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart",
    crate::sdk::PartChildCardinality::Optional,
  ),
  crate::sdk::PartChildDescriptor::new(
    "feature_property_bags_part",
    "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag",
    "crate::parts::feature_property_bags_part::FeaturePropertyBagsPart",
    crate::sdk::PartChildCardinality::Optional,
  ),
];
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
  pub fn custom_xml_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
    )
  }
  pub fn custom_xml_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::custom_xml_part::CustomXmlPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::custom_xml_part::CustomXmlPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
    )
  }
  pub fn calculation_chain_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain",
    )
  }
  pub fn calculation_chain_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::calculation_chain_part::CalculationChainPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::calculation_chain_part::CalculationChainPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain",
    )
  }
  pub fn cell_metadata_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sheetMetadata",
    )
  }
  pub fn cell_metadata_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::cell_metadata_part::CellMetadataPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::cell_metadata_part::CellMetadataPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sheetMetadata",
    )
  }
  pub fn connections_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/connections",
    )
  }
  pub fn connections_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::connections_part::ConnectionsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::connections_part::ConnectionsPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/connections",
    )
  }
  pub fn custom_xml_mappings_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps",
    )
  }
  pub fn custom_xml_mappings_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps",
    )
  }
  pub fn shared_string_table_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings",
    )
  }
  pub fn shared_string_table_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::shared_string_table_part::SharedStringTablePart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::shared_string_table_part::SharedStringTablePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings",
    )
  }
  pub fn workbook_revision_header_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders",
    )
  }
  pub fn workbook_revision_header_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders",
    )
  }
  pub fn workbook_user_data_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/usernames",
    )
  }
  pub fn workbook_user_data_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::workbook_user_data_part::WorkbookUserDataPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::workbook_user_data_part::WorkbookUserDataPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/usernames",
    )
  }
  pub fn workbook_styles_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
    )
  }
  pub fn workbook_styles_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::workbook_styles_part::WorkbookStylesPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::workbook_styles_part::WorkbookStylesPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
    )
  }
  pub fn theme_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
    )
  }
  pub fn theme_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::theme_part::ThemePart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::theme_part::ThemePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
    )
  }
  pub fn thumbnail_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail",
    )
  }
  pub fn thumbnail_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::thumbnail_part::ThumbnailPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::thumbnail_part::ThumbnailPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail",
    )
  }
  pub fn volatile_dependencies_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/volatileDependencies",
    )
  }
  pub fn volatile_dependencies_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::volatile_dependencies_part::VolatileDependenciesPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::volatile_dependencies_part::VolatileDependenciesPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/volatileDependencies",
    )
  }
  pub fn chartsheet_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet",
    )
  }
  pub fn chartsheet_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::chartsheet_part::ChartsheetPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::chartsheet_part::ChartsheetPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet",
    )
  }
  pub fn dialogsheet_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet",
    )
  }
  pub fn dialogsheet_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::dialogsheet_part::DialogsheetPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::dialogsheet_part::DialogsheetPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet",
    )
  }
  pub fn external_workbook_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink",
    )
  }
  pub fn external_workbook_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::external_workbook_part::ExternalWorkbookPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::external_workbook_part::ExternalWorkbookPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink",
    )
  }
  pub fn pivot_table_cache_definition_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition",
    )
  }
  pub fn pivot_table_cache_definition_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<
    Item = crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart,
  > + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition",
    )
  }
  pub fn worksheet_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet",
    )
  }
  pub fn worksheet_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::worksheet_part::WorksheetPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::worksheet_part::WorksheetPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet",
    )
  }
  pub fn excel_attached_toolbars_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars",
    )
  }
  pub fn excel_attached_toolbars_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars",
    )
  }
  pub fn vba_project_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2006/relationships/vbaProject",
    )
  }
  pub fn vba_project_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::vba_project_part::VbaProjectPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::vba_project_part::VbaProjectPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2006/relationships/vbaProject",
    )
  }
  pub fn macro_sheet_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet",
    )
  }
  pub fn macro_sheet_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::macro_sheet_part::MacroSheetPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::macro_sheet_part::MacroSheetPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet",
    )
  }
  pub fn international_macro_sheet_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2006/relationships/xlIntlMacrosheet",
    )
  }
  pub fn international_macro_sheet_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::international_macro_sheet_part::InternationalMacroSheetPart> + 'a
  {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::international_macro_sheet_part::InternationalMacroSheetPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2006/relationships/xlIntlMacrosheet",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn custom_data_properties_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2007/relationships/customDataProps",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn custom_data_properties_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::custom_data_properties_part::CustomDataPropertiesPart> + 'a
  {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::custom_data_properties_part::CustomDataPropertiesPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2007/relationships/customDataProps",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn slicer_cache_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2007/relationships/slicerCache",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn slicer_cache_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::slicer_cache_part::SlicerCachePart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::slicer_cache_part::SlicerCachePart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2007/relationships/slicerCache",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn time_line_cache_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2011/relationships/timelineCache",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn time_line_cache_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::time_line_cache_part::TimeLineCachePart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::time_line_cache_part::TimeLineCachePart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2011/relationships/timelineCache",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn workbook_person_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2017/10/relationships/person",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn workbook_person_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::workbook_person_part::WorkbookPersonPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::workbook_person_part::WorkbookPersonPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2017/10/relationships/person",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn rd_rich_value_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn rd_rich_value_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::rd_rich_value_part::RdRichValuePart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::rd_rich_value_part::RdRichValuePart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn ct_rd_rich_value_structure_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn ct_rd_rich_value_structure_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart> + 'a
  {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn rd_array_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdArray",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn rd_array_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::rd_array_part::RdArrayPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::rd_array_part::RdArrayPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdArray",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn rich_styles_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2017/06/relationships/richStyles",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn rich_styles_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::rich_styles_part::RichStylesPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::rich_styles_part::RichStylesPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2017/06/relationships/richStyles",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn rd_supporting_property_bag_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn rd_supporting_property_bag_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart>
  + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn rd_supporting_property_bag_structure_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure",
    )
  }
  #[cfg(feature = "microsoft365")]
    pub fn rd_supporting_property_bag_structure_parts<'a, P: crate::sdk::SdkPackage>(
        &'a self,
        package: &'a P,
    ) -> impl Iterator<
        Item = crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart,
  > + 'a{
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn rd_rich_value_types_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn rd_rich_value_types_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::rd_rich_value_types_part::RdRichValueTypesPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::rd_rich_value_types_part::RdRichValueTypesPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn rd_rich_value_web_image_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2020/07/relationships/rdRichValueWebImage",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn rd_rich_value_web_image_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2020/07/relationships/rdRichValueWebImage",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn feature_property_bags_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn feature_property_bags_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::feature_property_bags_part::FeaturePropertyBagsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::feature_property_bags_part::FeaturePropertyBagsPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag",
    )
  }
}
