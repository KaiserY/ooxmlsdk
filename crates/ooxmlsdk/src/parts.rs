//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub mod alternative_format_import_part;
pub mod calculation_chain_part;
pub mod cell_metadata_part;
#[cfg(feature = "microsoft365")]
pub mod chart_color_style_part;
pub mod chart_drawing_part;
pub mod chart_part;
#[cfg(feature = "microsoft365")]
pub mod chart_style_part;
pub mod chartsheet_part;
pub mod comment_authors_part;
pub mod connections_part;
#[cfg(feature = "microsoft365")]
pub mod control_properties_part;
pub mod core_file_properties_part;
#[cfg(feature = "microsoft365")]
pub mod custom_data_part;
#[cfg(feature = "microsoft365")]
pub mod custom_data_properties_part;
pub mod custom_file_properties_part;
pub mod custom_property_part;
pub mod custom_xml_mappings_part;
pub mod custom_xml_part;
pub mod custom_xml_properties_part;
pub mod customization_part;
pub mod diagram_colors_part;
pub mod diagram_data_part;
pub mod diagram_layout_definition_part;
#[cfg(feature = "microsoft365")]
pub mod diagram_persist_layout_part;
pub mod diagram_style_part;
pub mod dialogsheet_part;
pub mod digital_signature_origin_part;
pub mod document_settings_part;
#[cfg(feature = "microsoft365")]
pub mod document_tasks_part;
pub mod drawings_part;
pub mod embedded_control_persistence_binary_data_part;
pub mod embedded_control_persistence_part;
pub mod embedded_object_part;
pub mod embedded_package_part;
pub mod endnotes_part;
pub mod excel_attached_toolbars_part;
#[cfg(feature = "microsoft365")]
pub mod extended_chart_part;
pub mod extended_file_properties_part;
pub mod extended_part;
pub mod external_workbook_part;
#[cfg(feature = "microsoft365")]
pub mod feature_property_bags_part;
pub mod font_part;
pub mod font_table_part;
pub mod footer_part;
pub mod footnotes_part;
pub mod glossary_document_part;
pub mod handout_master_part;
pub mod header_part;
pub mod image_part;
pub mod international_macro_sheet_part;
#[cfg(feature = "microsoft365")]
pub mod label_info_part;
pub mod legacy_diagram_text_info_part;
pub mod legacy_diagram_text_part;
pub mod macro_sheet_part;
pub mod mail_merge_recipient_data_part;
pub mod main_document_part;
#[cfg(feature = "microsoft365")]
pub mod model3_d_reference_relationship_part;
#[cfg(feature = "microsoft365")]
pub mod named_sheet_views_part;
pub mod notes_master_part;
pub mod notes_slide_part;
pub mod numbering_definitions_part;
pub mod pivot_table_cache_definition_part;
pub mod pivot_table_cache_records_part;
pub mod pivot_table_part;
#[cfg(feature = "microsoft365")]
pub mod power_point_authors_part;
#[cfg(feature = "microsoft365")]
pub mod power_point_comment_part;
pub mod presentation_document;
pub mod presentation_part;
pub mod presentation_properties_part;
pub mod query_table_part;
pub mod quick_access_toolbar_customizations_part;
#[cfg(feature = "microsoft365")]
pub mod rd_array_part;
#[cfg(feature = "microsoft365")]
pub mod rd_rich_value_part;
#[cfg(feature = "microsoft365")]
pub mod rd_rich_value_structure_part;
#[cfg(feature = "microsoft365")]
pub mod rd_rich_value_types_part;
#[cfg(feature = "microsoft365")]
pub mod rd_rich_value_web_image_part;
#[cfg(feature = "microsoft365")]
pub mod rd_supporting_property_bag_part;
#[cfg(feature = "microsoft365")]
pub mod rd_supporting_property_bag_structure_part;
#[cfg(feature = "microsoft365")]
pub mod ribbon_and_backstage_customizations_part;
pub mod ribbon_extensibility_part;
#[cfg(feature = "microsoft365")]
pub mod rich_styles_part;
pub mod shared_string_table_part;
pub mod single_cell_table_part;
#[cfg(feature = "microsoft365")]
pub mod slicer_cache_part;
#[cfg(feature = "microsoft365")]
pub mod slicers_part;
pub mod slide_comments_part;
pub mod slide_layout_part;
pub mod slide_master_part;
pub mod slide_part;
pub mod slide_sync_data_part;
pub mod spreadsheet_document;
pub mod spreadsheet_printer_settings_part;
pub mod style_definitions_part;
#[cfg(feature = "microsoft365")]
pub mod styles_with_effects_part;
pub mod table_definition_part;
pub mod table_styles_part;
pub mod theme_override_part;
pub mod theme_part;
pub mod thumbnail_part;
#[cfg(feature = "microsoft365")]
pub mod time_line_cache_part;
#[cfg(feature = "microsoft365")]
pub mod time_line_part;
pub mod user_defined_tags_part;
pub mod vba_data_part;
pub mod vba_project_part;
pub mod view_properties_part;
pub mod vml_drawing_part;
pub mod volatile_dependencies_part;
#[cfg(feature = "microsoft365")]
pub mod web_ex_taskpanes_part;
#[cfg(feature = "microsoft365")]
pub mod web_extension_part;
pub mod web_settings_part;
pub mod word_attached_toolbars_part;
#[cfg(feature = "microsoft365")]
pub mod word_comments_extensible_part;
#[cfg(feature = "microsoft365")]
pub mod wordprocessing_comments_ex_part;
#[cfg(feature = "microsoft365")]
pub mod wordprocessing_comments_ids_part;
pub mod wordprocessing_comments_part;
pub mod wordprocessing_document;
#[cfg(feature = "microsoft365")]
pub mod wordprocessing_people_part;
pub mod wordprocessing_printer_settings_part;
pub mod workbook_part;
#[cfg(feature = "microsoft365")]
pub mod workbook_person_part;
pub mod workbook_revision_header_part;
pub mod workbook_revision_log_part;
pub mod workbook_styles_part;
pub mod workbook_user_data_part;
pub mod worksheet_comments_part;
pub mod worksheet_part;
pub mod worksheet_sort_map_part;
#[cfg(feature = "microsoft365")]
pub mod worksheet_threaded_comments_part;
pub mod xml_signature_part;
macro_rules! define_part_ref {
    ($($(#[$attrs:meta])* $variant:ident ($part_ty:ty),)*) => {
        #[derive(Clone, Debug, Eq, PartialEq)] pub enum PartRef { $($(#[$attrs])*
        $variant ($part_ty),)* ExtendedPart(crate ::parts::extended_part::ExtendedPart),
        } impl PartRef { pub fn part_id(& self) -> crate ::common::PartId { match self {
        $($(#[$attrs])* Self:: $variant (part) => part.part_id(),)*
        Self::ExtendedPart(part) => { < crate ::parts::extended_part::ExtendedPart as
        crate ::sdk::SdkPart > ::part_id(part) } } } }
    };
}
define_part_ref! {
    AlternativeFormatImportPart(crate
    ::parts::alternative_format_import_part::AlternativeFormatImportPart),
    CalculationChainPart(crate ::parts::calculation_chain_part::CalculationChainPart),
    CellMetadataPart(crate ::parts::cell_metadata_part::CellMetadataPart), #[cfg(feature
    = "microsoft365")] ChartColorStylePart(crate
    ::parts::chart_color_style_part::ChartColorStylePart), ChartDrawingPart(crate
    ::parts::chart_drawing_part::ChartDrawingPart), ChartPart(crate
    ::parts::chart_part::ChartPart), #[cfg(feature = "microsoft365")]
    ChartStylePart(crate ::parts::chart_style_part::ChartStylePart), ChartsheetPart(crate
    ::parts::chartsheet_part::ChartsheetPart), CommentAuthorsPart(crate
    ::parts::comment_authors_part::CommentAuthorsPart), ConnectionsPart(crate
    ::parts::connections_part::ConnectionsPart), #[cfg(feature = "microsoft365")]
    ControlPropertiesPart(crate ::parts::control_properties_part::ControlPropertiesPart),
    CoreFilePropertiesPart(crate
    ::parts::core_file_properties_part::CoreFilePropertiesPart), #[cfg(feature =
    "microsoft365")] CustomDataPart(crate ::parts::custom_data_part::CustomDataPart),
    #[cfg(feature = "microsoft365")] CustomDataPropertiesPart(crate
    ::parts::custom_data_properties_part::CustomDataPropertiesPart),
    CustomFilePropertiesPart(crate
    ::parts::custom_file_properties_part::CustomFilePropertiesPart),
    CustomPropertyPart(crate ::parts::custom_property_part::CustomPropertyPart),
    CustomXmlMappingsPart(crate
    ::parts::custom_xml_mappings_part::CustomXmlMappingsPart), CustomXmlPart(crate
    ::parts::custom_xml_part::CustomXmlPart), CustomXmlPropertiesPart(crate
    ::parts::custom_xml_properties_part::CustomXmlPropertiesPart),
    CustomizationPart(crate ::parts::customization_part::CustomizationPart),
    DiagramColorsPart(crate ::parts::diagram_colors_part::DiagramColorsPart),
    DiagramDataPart(crate ::parts::diagram_data_part::DiagramDataPart),
    DiagramLayoutDefinitionPart(crate
    ::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart), #[cfg(feature
    = "microsoft365")] DiagramPersistLayoutPart(crate
    ::parts::diagram_persist_layout_part::DiagramPersistLayoutPart),
    DiagramStylePart(crate ::parts::diagram_style_part::DiagramStylePart),
    DialogsheetPart(crate ::parts::dialogsheet_part::DialogsheetPart),
    DigitalSignatureOriginPart(crate
    ::parts::digital_signature_origin_part::DigitalSignatureOriginPart),
    DocumentSettingsPart(crate ::parts::document_settings_part::DocumentSettingsPart),
    #[cfg(feature = "microsoft365")] DocumentTasksPart(crate
    ::parts::document_tasks_part::DocumentTasksPart), DrawingsPart(crate
    ::parts::drawings_part::DrawingsPart), EmbeddedControlPersistenceBinaryDataPart(crate
    ::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart),
    EmbeddedControlPersistencePart(crate
    ::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart),
    EmbeddedObjectPart(crate ::parts::embedded_object_part::EmbeddedObjectPart),
    EmbeddedPackagePart(crate ::parts::embedded_package_part::EmbeddedPackagePart),
    EndnotesPart(crate ::parts::endnotes_part::EndnotesPart),
    ExcelAttachedToolbarsPart(crate
    ::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart), #[cfg(feature =
    "microsoft365")] ExtendedChartPart(crate
    ::parts::extended_chart_part::ExtendedChartPart), ExtendedFilePropertiesPart(crate
    ::parts::extended_file_properties_part::ExtendedFilePropertiesPart),
    ExternalWorkbookPart(crate ::parts::external_workbook_part::ExternalWorkbookPart),
    #[cfg(feature = "microsoft365")] FeaturePropertyBagsPart(crate
    ::parts::feature_property_bags_part::FeaturePropertyBagsPart), FontPart(crate
    ::parts::font_part::FontPart), FontTablePart(crate
    ::parts::font_table_part::FontTablePart), FooterPart(crate
    ::parts::footer_part::FooterPart), FootnotesPart(crate
    ::parts::footnotes_part::FootnotesPart), GlossaryDocumentPart(crate
    ::parts::glossary_document_part::GlossaryDocumentPart), HandoutMasterPart(crate
    ::parts::handout_master_part::HandoutMasterPart), HeaderPart(crate
    ::parts::header_part::HeaderPart), ImagePart(crate ::parts::image_part::ImagePart),
    InternationalMacroSheetPart(crate
    ::parts::international_macro_sheet_part::InternationalMacroSheetPart), #[cfg(feature
    = "microsoft365")] LabelInfoPart(crate ::parts::label_info_part::LabelInfoPart),
    LegacyDiagramTextInfoPart(crate
    ::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart),
    LegacyDiagramTextPart(crate
    ::parts::legacy_diagram_text_part::LegacyDiagramTextPart), MacroSheetPart(crate
    ::parts::macro_sheet_part::MacroSheetPart), MailMergeRecipientDataPart(crate
    ::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart),
    MainDocumentPart(crate ::parts::main_document_part::MainDocumentPart), #[cfg(feature
    = "microsoft365")] Model3DReferenceRelationshipPart(crate
    ::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart),
    #[cfg(feature = "microsoft365")] NamedSheetViewsPart(crate
    ::parts::named_sheet_views_part::NamedSheetViewsPart), NotesMasterPart(crate
    ::parts::notes_master_part::NotesMasterPart), NotesSlidePart(crate
    ::parts::notes_slide_part::NotesSlidePart), NumberingDefinitionsPart(crate
    ::parts::numbering_definitions_part::NumberingDefinitionsPart),
    PivotTableCacheDefinitionPart(crate
    ::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart),
    PivotTableCacheRecordsPart(crate
    ::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart),
    PivotTablePart(crate ::parts::pivot_table_part::PivotTablePart), #[cfg(feature =
    "microsoft365")] PowerPointAuthorsPart(crate
    ::parts::power_point_authors_part::PowerPointAuthorsPart), #[cfg(feature =
    "microsoft365")] PowerPointCommentPart(crate
    ::parts::power_point_comment_part::PowerPointCommentPart), PresentationPart(crate
    ::parts::presentation_part::PresentationPart), PresentationPropertiesPart(crate
    ::parts::presentation_properties_part::PresentationPropertiesPart),
    QueryTablePart(crate ::parts::query_table_part::QueryTablePart),
    QuickAccessToolbarCustomizationsPart(crate
    ::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart),
    #[cfg(feature = "microsoft365")] RdArrayPart(crate
    ::parts::rd_array_part::RdArrayPart), #[cfg(feature = "microsoft365")]
    RdRichValuePart(crate ::parts::rd_rich_value_part::RdRichValuePart), #[cfg(feature =
    "microsoft365")] RdRichValueStructurePart(crate
    ::parts::rd_rich_value_structure_part::RdRichValueStructurePart), #[cfg(feature =
    "microsoft365")] RdRichValueTypesPart(crate
    ::parts::rd_rich_value_types_part::RdRichValueTypesPart), #[cfg(feature =
    "microsoft365")] RdRichValueWebImagePart(crate
    ::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart), #[cfg(feature =
    "microsoft365")] RdSupportingPropertyBagPart(crate
    ::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart), #[cfg(feature
    = "microsoft365")] RdSupportingPropertyBagStructurePart(crate
    ::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart),
    #[cfg(feature = "microsoft365")] RibbonAndBackstageCustomizationsPart(crate
    ::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart),
    RibbonExtensibilityPart(crate
    ::parts::ribbon_extensibility_part::RibbonExtensibilityPart), #[cfg(feature =
    "microsoft365")] RichStylesPart(crate ::parts::rich_styles_part::RichStylesPart),
    SharedStringTablePart(crate
    ::parts::shared_string_table_part::SharedStringTablePart), SingleCellTablePart(crate
    ::parts::single_cell_table_part::SingleCellTablePart), #[cfg(feature =
    "microsoft365")] SlicerCachePart(crate ::parts::slicer_cache_part::SlicerCachePart),
    #[cfg(feature = "microsoft365")] SlicersPart(crate
    ::parts::slicers_part::SlicersPart), SlideCommentsPart(crate
    ::parts::slide_comments_part::SlideCommentsPart), SlideLayoutPart(crate
    ::parts::slide_layout_part::SlideLayoutPart), SlideMasterPart(crate
    ::parts::slide_master_part::SlideMasterPart), SlidePart(crate
    ::parts::slide_part::SlidePart), SlideSyncDataPart(crate
    ::parts::slide_sync_data_part::SlideSyncDataPart),
    SpreadsheetPrinterSettingsPart(crate
    ::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart),
    StyleDefinitionsPart(crate ::parts::style_definitions_part::StyleDefinitionsPart),
    #[cfg(feature = "microsoft365")] StylesWithEffectsPart(crate
    ::parts::styles_with_effects_part::StylesWithEffectsPart), TableDefinitionPart(crate
    ::parts::table_definition_part::TableDefinitionPart), TableStylesPart(crate
    ::parts::table_styles_part::TableStylesPart), ThemeOverridePart(crate
    ::parts::theme_override_part::ThemeOverridePart), ThemePart(crate
    ::parts::theme_part::ThemePart), ThumbnailPart(crate
    ::parts::thumbnail_part::ThumbnailPart), #[cfg(feature = "microsoft365")]
    TimeLineCachePart(crate ::parts::time_line_cache_part::TimeLineCachePart),
    #[cfg(feature = "microsoft365")] TimeLinePart(crate
    ::parts::time_line_part::TimeLinePart), UserDefinedTagsPart(crate
    ::parts::user_defined_tags_part::UserDefinedTagsPart), VbaDataPart(crate
    ::parts::vba_data_part::VbaDataPart), VbaProjectPart(crate
    ::parts::vba_project_part::VbaProjectPart), ViewPropertiesPart(crate
    ::parts::view_properties_part::ViewPropertiesPart), VmlDrawingPart(crate
    ::parts::vml_drawing_part::VmlDrawingPart), VolatileDependenciesPart(crate
    ::parts::volatile_dependencies_part::VolatileDependenciesPart), #[cfg(feature =
    "microsoft365")] WebExTaskpanesPart(crate
    ::parts::web_ex_taskpanes_part::WebExTaskpanesPart), #[cfg(feature = "microsoft365")]
    WebExtensionPart(crate ::parts::web_extension_part::WebExtensionPart),
    WebSettingsPart(crate ::parts::web_settings_part::WebSettingsPart),
    WordAttachedToolbarsPart(crate
    ::parts::word_attached_toolbars_part::WordAttachedToolbarsPart), #[cfg(feature =
    "microsoft365")] WordCommentsExtensiblePart(crate
    ::parts::word_comments_extensible_part::WordCommentsExtensiblePart), #[cfg(feature =
    "microsoft365")] WordprocessingCommentsExPart(crate
    ::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart),
    #[cfg(feature = "microsoft365")] WordprocessingCommentsIdsPart(crate
    ::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart),
    WordprocessingCommentsPart(crate
    ::parts::wordprocessing_comments_part::WordprocessingCommentsPart), #[cfg(feature =
    "microsoft365")] WordprocessingPeoplePart(crate
    ::parts::wordprocessing_people_part::WordprocessingPeoplePart),
    WordprocessingPrinterSettingsPart(crate
    ::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart),
    WorkbookPart(crate ::parts::workbook_part::WorkbookPart), #[cfg(feature =
    "microsoft365")] WorkbookPersonPart(crate
    ::parts::workbook_person_part::WorkbookPersonPart), WorkbookRevisionHeaderPart(crate
    ::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart),
    WorkbookRevisionLogPart(crate
    ::parts::workbook_revision_log_part::WorkbookRevisionLogPart),
    WorkbookStylesPart(crate ::parts::workbook_styles_part::WorkbookStylesPart),
    WorkbookUserDataPart(crate ::parts::workbook_user_data_part::WorkbookUserDataPart),
    WorksheetCommentsPart(crate ::parts::worksheet_comments_part::WorksheetCommentsPart),
    WorksheetPart(crate ::parts::worksheet_part::WorksheetPart),
    WorksheetSortMapPart(crate ::parts::worksheet_sort_map_part::WorksheetSortMapPart),
    #[cfg(feature = "microsoft365")] WorksheetThreadedCommentsPart(crate
    ::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart),
    XmlSignaturePart(crate ::parts::xml_signature_part::XmlSignaturePart),
}
impl PartRef {
  pub(crate) fn from_part_id<P: crate::sdk::SdkPackage>(
    package: &P,
    part_id: crate::common::PartId,
  ) -> Option<Self> {
    Self::from_storage(
      crate::sdk::SdkPackageInternal::storage(package),
      part_id,
      None,
    )
  }
  pub(crate) fn from_relationship_storage(
    storage: &crate::common::SdkPackageStorage,
    relationship: &crate::common::RelationshipInfo,
  ) -> Option<Self> {
    Self::from_storage(
      storage,
      relationship.target_part_id()?,
      Some(relationship.id()),
    )
  }
  fn from_storage(
    storage: &crate::common::SdkPackageStorage,
    part_id: crate::common::PartId,
    relationship_id: Option<&str>,
  ) -> Option<Self> {
    let part = storage.part(part_id)?;
    let Some(relationship_type) = part.relationship_type() else {
      let part = if let Some(relationship_id) = relationship_id {
        <crate::parts::extended_part::ExtendedPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                    storage,
                    relationship_id,
                    part_id,
                )
      } else {
        <crate::parts::extended_part::ExtendedPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                    storage,
                    part_id,
                )
      };
      return Some(PartRef::ExtendedPart(part));
    };
    match relationship_type {
      "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary" => {
        if true {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::EmbeddedControlPersistenceBinaryDataPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars" => {
        if part.content_type() == "application/vnd.ms-excel.attachedToolbars" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ExcelAttachedToolbarsPart(part));
        }
        if part.content_type() == "application/vnd.ms-word.attachedToolbars" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WordAttachedToolbarsPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/keyMapCustomizations" => {
        if part.content_type() == "application/vnd.ms-word.keyMapCustomizations+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::customization_part::CustomizationPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::customization_part::CustomizationPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CustomizationPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/legacyDiagramText" => {
        if part.content_type() == "application/vnd.ms-office.legacyDiagramText" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::LegacyDiagramTextPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/legacyDocTextInfo" => {
        if part.content_type() == "application/vnd.ms-office.legacyDocTextInfo" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::LegacyDiagramTextInfoPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/ui/extensibility" => {
        if part.content_type() == "application/xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RibbonExtensibilityPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/ui/userCustomization" => {
        if part.content_type() == "application/xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::QuickAccessToolbarCustomizationsPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/vbaProject" => {
        if part.content_type() == "application/vnd.ms-office.vbaProject" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::vba_project_part::VbaProjectPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::vba_project_part::VbaProjectPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::VbaProjectPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/wordVbaData" => {
        if part.content_type() == "application/vnd.ms-word.vbaData+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::vba_data_part::VbaDataPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::vba_data_part::VbaDataPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::VbaDataPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/wsSortMap" => {
        if part.content_type() == "application/vnd.ms-excel.wsSortMap+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::worksheet_sort_map_part::WorksheetSortMapPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::worksheet_sort_map_part::WorksheetSortMapPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorksheetSortMapPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/xlIntlMacrosheet" => {
        if part.content_type() == "application/vnd.ms-excel.intlmacrosheet+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::international_macro_sheet_part::InternationalMacroSheetPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::international_macro_sheet_part::InternationalMacroSheetPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::InternationalMacroSheetPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet" => {
        if part.content_type() == "application/vnd.ms-excel.macrosheet+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::macro_sheet_part::MacroSheetPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::macro_sheet_part::MacroSheetPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::MacroSheetPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/customData" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/binary" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::custom_data_part::CustomDataPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::custom_data_part::CustomDataPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CustomDataPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/customDataProps" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.customDataProperties+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::custom_data_properties_part::CustomDataPropertiesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::custom_data_properties_part::CustomDataPropertiesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CustomDataPropertiesPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.drawingml.diagramDrawing+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DiagramPersistLayoutPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/slicer" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.slicer+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::slicers_part::SlicersPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::slicers_part::SlicersPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SlicersPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/slicerCache" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.slicerCache+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::slicer_cache_part::SlicerCachePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::slicer_cache_part::SlicerCachePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SlicerCachePart(part));
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-word.stylesWithEffects+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::styles_with_effects_part::StylesWithEffectsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::styles_with_effects_part::StylesWithEffectsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::StylesWithEffectsPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RibbonAndBackstageCustomizationsPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.chartcolorstyle+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::chart_color_style_part::ChartColorStylePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::chart_color_style_part::ChartColorStylePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ChartColorStylePart(part));
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/chartStyle" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.chartstyle+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::chart_style_part::ChartStylePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::chart_style_part::ChartStylePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ChartStylePart(part));
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/commentsExtended" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtended+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WordprocessingCommentsExPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/people" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.people+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::wordprocessing_people_part::WordprocessingPeoplePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::wordprocessing_people_part::WordprocessingPeoplePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WordprocessingPeoplePart(part));
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/timeline" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.timeline+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::time_line_part::TimeLinePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::time_line_part::TimeLinePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::TimeLinePart(part));
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/timelineCache" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.timelineCache+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::time_line_cache_part::TimeLineCachePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::time_line_cache_part::TimeLineCachePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::TimeLineCachePart(part));
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/webextension" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.webextension+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::web_extension_part::WebExtensionPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::web_extension_part::WebExtensionPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WebExtensionPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.webextensiontaskpanes+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WebExTaskpanesPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2014/relationships/chartEx" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.chartex+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::extended_chart_part::ExtendedChartPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::extended_chart_part::ExtendedChartPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ExtendedChartPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2016/09/relationships/commentsIds" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsIds+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WordprocessingCommentsIdsPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/model3d" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "model/gltf-binary" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::Model3DReferenceRelationshipPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdArray" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdarray+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::rd_array_part::RdArrayPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::rd_array_part::RdArrayPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RdArrayPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdrichvalue+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::rd_rich_value_part::RdRichValuePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::rd_rich_value_part::RdRichValuePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RdRichValuePart(part));
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdrichvaluestructure+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RdRichValueStructurePart(part));
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdrichvaluetypes+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::rd_rich_value_types_part::RdRichValueTypesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::rd_rich_value_types_part::RdRichValueTypesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RdRichValueTypesPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdsupportingpropertybag+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RdSupportingPropertyBagPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure" =>
      {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdsupportingpropertybagstructure+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RdSupportingPropertyBagStructurePart(part));
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/richStyles" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.richstyles+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::rich_styles_part::RichStylesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::rich_styles_part::RichStylesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RichStylesPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2017/10/relationships/person" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.person+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::workbook_person_part::WorkbookPersonPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::workbook_person_part::WorkbookPersonPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorkbookPersonPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2017/10/relationships/threadedComment" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.threadedcomments+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorksheetThreadedCommentsPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2018/08/relationships/commentsExtensible" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtensible+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WordCommentsExtensiblePart(part));
        }
      }
      "http://schemas.microsoft.com/office/2018/10/relationships/authors" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-powerpoint.authors+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::power_point_authors_part::PowerPointAuthorsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::power_point_authors_part::PowerPointAuthorsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::PowerPointAuthorsPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2018/10/relationships/comments" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-powerpoint.comments+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::power_point_comment_part::PowerPointCommentPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::power_point_comment_part::PowerPointCommentPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::PowerPointCommentPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2019/04/relationships/namedSheetView" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.namedsheetviews+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::named_sheet_views_part::NamedSheetViewsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::named_sheet_views_part::NamedSheetViewsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::NamedSheetViewsPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2019/05/relationships/documenttasks" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.documenttasks+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::document_tasks_part::DocumentTasksPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::document_tasks_part::DocumentTasksPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DocumentTasksPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.classificationlabels+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::label_info_part::LabelInfoPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::label_info_part::LabelInfoPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::LabelInfoPart(part));
        }
      }
      "http://schemas.microsoft.com/office/2020/07/relationships/rdRichValueWebImage" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdrichvaluewebimage+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RdRichValueWebImagePart(part));
        }
      }
      "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.featurepropertybag+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::feature_property_bags_part::FeaturePropertyBagsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::feature_property_bags_part::FeaturePropertyBagsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::FeaturePropertyBagsPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk" => {
        if true {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::alternative_format_import_part::AlternativeFormatImportPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::alternative_format_import_part::AlternativeFormatImportPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::AlternativeFormatImportPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::calculation_chain_part::CalculationChainPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::calculation_chain_part::CalculationChainPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CalculationChainPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.chart+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::chart_part::ChartPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::chart_part::ChartPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ChartPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartUserShapes" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::chart_drawing_part::ChartDrawingPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::chart_drawing_part::ChartDrawingPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ChartDrawingPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::chartsheet_part::ChartsheetPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::chartsheet_part::ChartsheetPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ChartsheetPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/commentAuthors" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::comment_authors_part::CommentAuthorsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::comment_authors_part::CommentAuthorsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CommentAuthorsPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.comments+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::slide_comments_part::SlideCommentsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::slide_comments_part::SlideCommentsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SlideCommentsPart(part));
        }
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WordprocessingCommentsPart(part));
        }
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::worksheet_comments_part::WorksheetCommentsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::worksheet_comments_part::WorksheetCommentsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorksheetCommentsPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/connections" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::connections_part::ConnectionsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::connections_part::ConnectionsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ConnectionsPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control" => {
        if true {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::EmbeddedControlPersistencePart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/ctrlProp" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.controlproperties+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::control_properties_part::ControlPropertiesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::control_properties_part::ControlPropertiesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ControlPropertiesPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.custom-properties+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::custom_file_properties_part::CustomFilePropertiesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::custom_file_properties_part::CustomFilePropertiesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CustomFilePropertiesPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty" => {
        if true {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::custom_property_part::CustomPropertyPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::custom_property_part::CustomPropertyPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CustomPropertyPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml" => {
        if true {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::custom_xml_part::CustomXmlPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::custom_xml_part::CustomXmlPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CustomXmlPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.customXmlProperties+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CustomXmlPropertiesPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::diagram_colors_part::DiagramColorsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::diagram_colors_part::DiagramColorsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DiagramColorsPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::diagram_data_part::DiagramDataPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::diagram_data_part::DiagramDataPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DiagramDataPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DiagramLayoutDefinitionPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::diagram_style_part::DiagramStylePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::diagram_style_part::DiagramStylePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DiagramStylePart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::dialogsheet_part::DialogsheetPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::dialogsheet_part::DialogsheetPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DialogsheetPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing" => {
        if part.content_type() == "application/vnd.openxmlformats-officedocument.drawing+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::drawings_part::DrawingsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::drawings_part::DrawingsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DrawingsPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::endnotes_part::EndnotesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::endnotes_part::EndnotesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::EndnotesPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.extended-properties+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ExtendedFilePropertiesPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::external_workbook_part::ExternalWorkbookPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::external_workbook_part::ExternalWorkbookPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ExternalWorkbookPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font" => {
        if true {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::font_part::FontPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::font_part::FontPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::FontPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::font_table_part::FontTablePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::font_table_part::FontTablePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::FontTablePart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::footer_part::FooterPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::footer_part::FooterPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::FooterPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::footnotes_part::FootnotesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::footnotes_part::FootnotesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::FootnotesPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/glossaryDocument" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::glossary_document_part::GlossaryDocumentPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::glossary_document_part::GlossaryDocumentPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::GlossaryDocumentPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::handout_master_part::HandoutMasterPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::handout_master_part::HandoutMasterPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::HandoutMasterPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::header_part::HeaderPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::header_part::HeaderPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::HeaderPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" => {
        if true {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::image_part::ImagePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::image_part::ImagePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ImagePart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::notes_master_part::NotesMasterPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::notes_master_part::NotesMasterPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::NotesMasterPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::notes_slide_part::NotesSlidePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::notes_slide_part::NotesSlidePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::NotesSlidePart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::numbering_definitions_part::NumberingDefinitionsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::numbering_definitions_part::NumberingDefinitionsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::NumberingDefinitionsPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" => {
        if part.path() == "word/document.xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::main_document_part::MainDocumentPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::main_document_part::MainDocumentPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::MainDocumentPart(part));
        }
        if part.path() == "ppt/presentation.xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::presentation_part::PresentationPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::presentation_part::PresentationPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::PresentationPart(part));
        }
        if part.path() == "xl/workbook.xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::workbook_part::WorkbookPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::workbook_part::WorkbookPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorkbookPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject" => {
        if true {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::embedded_object_part::EmbeddedObjectPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::embedded_object_part::EmbeddedObjectPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::EmbeddedObjectPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package" => {
        if true {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::embedded_package_part::EmbeddedPackagePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::embedded_package_part::EmbeddedPackagePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::EmbeddedPackagePart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::PivotTableCacheDefinitionPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheRecords" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::PivotTableCacheRecordsPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::pivot_table_part::PivotTablePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::pivot_table_part::PivotTablePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::PivotTablePart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::presentation_properties_part::PresentationPropertiesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::presentation_properties_part::PresentationPropertiesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::PresentationPropertiesPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.printerSettings"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SpreadsheetPrinterSettingsPart(part));
        }
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.printerSettings"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WordprocessingPrinterSettingsPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/queryTable" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::query_table_part::QueryTablePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::query_table_part::QueryTablePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::QueryTablePart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/recipientData" => {
        if true {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::MailMergeRecipientDataPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorkbookRevisionHeaderPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionLog" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorkbookRevisionLogPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::document_settings_part::DocumentSettingsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::document_settings_part::DocumentSettingsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DocumentSettingsPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::shared_string_table_part::SharedStringTablePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::shared_string_table_part::SharedStringTablePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SharedStringTablePart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sheetMetadata" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::cell_metadata_part::CellMetadataPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::cell_metadata_part::CellMetadataPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CellMetadataPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.slide+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::slide_part::SlidePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::slide_part::SlidePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SlidePart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::slide_layout_part::SlideLayoutPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::slide_layout_part::SlideLayoutPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SlideLayoutPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::slide_master_part::SlideMasterPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::slide_master_part::SlideMasterPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SlideMasterPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideUpdateInfo" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::slide_sync_data_part::SlideSyncDataPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::slide_sync_data_part::SlideSyncDataPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SlideSyncDataPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::style_definitions_part::StyleDefinitionsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::style_definitions_part::StyleDefinitionsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::StyleDefinitionsPart(part));
        }
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::workbook_styles_part::WorkbookStylesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::workbook_styles_part::WorkbookStylesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorkbookStylesPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::table_definition_part::TableDefinitionPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::table_definition_part::TableDefinitionPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::TableDefinitionPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableSingleCells" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::single_cell_table_part::SingleCellTablePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::single_cell_table_part::SingleCellTablePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SingleCellTablePart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::table_styles_part::TableStylesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::table_styles_part::TableStylesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::TableStylesPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.tags+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::user_defined_tags_part::UserDefinedTagsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::user_defined_tags_part::UserDefinedTagsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::UserDefinedTagsPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" => {
        if part.content_type() == "application/vnd.openxmlformats-officedocument.theme+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::theme_part::ThemePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::theme_part::ThemePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ThemePart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride" => {
        if part.content_type() == "application/vnd.openxmlformats-officedocument.themeOverride+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::theme_override_part::ThemeOverridePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::theme_override_part::ThemeOverridePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ThemeOverridePart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/usernames" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::workbook_user_data_part::WorkbookUserDataPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::workbook_user_data_part::WorkbookUserDataPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorkbookUserDataPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::view_properties_part::ViewPropertiesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::view_properties_part::ViewPropertiesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ViewPropertiesPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing" => {
        if part.content_type() == "application/vnd.openxmlformats-officedocument.vmlDrawing" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::vml_drawing_part::VmlDrawingPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::vml_drawing_part::VmlDrawingPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::VmlDrawingPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/volatileDependencies" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::volatile_dependencies_part::VolatileDependenciesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::volatile_dependencies_part::VolatileDependenciesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::VolatileDependenciesPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::web_settings_part::WebSettingsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::web_settings_part::WebSettingsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WebSettingsPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::worksheet_part::WorksheetPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::worksheet_part::WorksheetPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorksheetPart(part));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps" => {
        if part.content_type() == "application/xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CustomXmlMappingsPart(part));
        }
      }
      "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin" => {
        if part.content_type() == "application/vnd.openxmlformats-package.digital-signature-origin"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DigitalSignatureOriginPart(part));
        }
      }
      "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/signature" => {
        if part.content_type()
          == "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml"
        {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::xml_signature_part::XmlSignaturePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::xml_signature_part::XmlSignaturePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::XmlSignaturePart(part));
        }
      }
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" => {
        if part.content_type() == "application/vnd.openxmlformats-package.core-properties+xml" {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::core_file_properties_part::CoreFilePropertiesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::core_file_properties_part::CoreFilePropertiesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CoreFilePropertiesPart(part));
        }
      }
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail" => {
        if true {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::thumbnail_part::ThumbnailPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::thumbnail_part::ThumbnailPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ThumbnailPart(part));
        }
      }
      _ => {
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk",
          "",
          ".",
          "afchunk",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::alternative_format_import_part::AlternativeFormatImportPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::alternative_format_import_part::AlternativeFormatImportPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::AlternativeFormatImportPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml",
          ".",
          "calcChain",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::calculation_chain_part::CalculationChainPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::calculation_chain_part::CalculationChainPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CalculationChainPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sheetMetadata",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml",
          ".",
          "metadata",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::cell_metadata_part::CellMetadataPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::cell_metadata_part::CellMetadataPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CellMetadataPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle",
          "application/vnd.ms-office.chartcolorstyle+xml",
          ".",
          "colors",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::chart_color_style_part::ChartColorStylePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::chart_color_style_part::ChartColorStylePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ChartColorStylePart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartUserShapes",
          "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml",
          "../drawings",
          "drawing",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::chart_drawing_part::ChartDrawingPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::chart_drawing_part::ChartDrawingPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ChartDrawingPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart",
          "application/vnd.openxmlformats-officedocument.drawingml.chart+xml",
          "charts",
          "chart",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::chart_part::ChartPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::chart_part::ChartPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ChartPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2011/relationships/chartStyle",
          "application/vnd.ms-office.chartstyle+xml",
          ".",
          "style",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::chart_style_part::ChartStylePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::chart_style_part::ChartStylePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ChartStylePart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml",
          "chartsheets",
          "sheet",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::chartsheet_part::ChartsheetPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::chartsheet_part::ChartsheetPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ChartsheetPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/commentAuthors",
          "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml",
          ".",
          "commentAuthors",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::comment_authors_part::CommentAuthorsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::comment_authors_part::CommentAuthorsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CommentAuthorsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/connections",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml",
          ".",
          "connections",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::connections_part::ConnectionsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::connections_part::ConnectionsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ConnectionsPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/ctrlProp",
          "application/vnd.ms-excel.controlproperties+xml",
          "../ctrlProps",
          "ctrlProp",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::control_properties_part::ControlPropertiesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::control_properties_part::ControlPropertiesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ControlPropertiesPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties",
          "application/vnd.openxmlformats-package.core-properties+xml",
          "docProps",
          "core",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::core_file_properties_part::CoreFilePropertiesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::core_file_properties_part::CoreFilePropertiesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CoreFilePropertiesPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2007/relationships/customData",
          "application/binary",
          "customData",
          "customData",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::custom_data_part::CustomDataPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::custom_data_part::CustomDataPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CustomDataPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2007/relationships/customDataProps",
          "application/vnd.ms-excel.customDataProperties+xml",
          "customData",
          "customDataProps",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::custom_data_properties_part::CustomDataPropertiesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::custom_data_properties_part::CustomDataPropertiesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CustomDataPropertiesPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties",
          "application/vnd.openxmlformats-officedocument.custom-properties+xml",
          "docProps",
          "custom",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::custom_file_properties_part::CustomFilePropertiesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::custom_file_properties_part::CustomFilePropertiesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CustomFilePropertiesPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty",
          "",
          ".",
          "CustomProperty",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::custom_property_part::CustomPropertyPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::custom_property_part::CustomPropertyPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CustomPropertyPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps",
          "application/xml",
          ".",
          "xmlMaps",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CustomXmlMappingsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
          "",
          "../customXml",
          "item",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::custom_xml_part::CustomXmlPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::custom_xml_part::CustomXmlPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CustomXmlPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps",
          "application/vnd.openxmlformats-officedocument.customXmlProperties+xml",
          ".",
          "itemProps",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CustomXmlPropertiesPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2006/relationships/keyMapCustomizations",
          "application/vnd.ms-word.keyMapCustomizations+xml",
          ".",
          "customizations",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::customization_part::CustomizationPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::customization_part::CustomizationPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::CustomizationPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors",
          "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml",
          "../graphics",
          "colors",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::diagram_colors_part::DiagramColorsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::diagram_colors_part::DiagramColorsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DiagramColorsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData",
          "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml",
          "../graphics",
          "data",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::diagram_data_part::DiagramDataPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::diagram_data_part::DiagramDataPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DiagramDataPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout",
          "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml",
          "../graphics",
          "layout",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DiagramLayoutDefinitionPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing",
          "application/vnd.ms-office.drawingml.diagramDrawing+xml",
          "../diagrams",
          "drawing",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DiagramPersistLayoutPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle",
          "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml",
          "../graphics",
          "quickStyle",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::diagram_style_part::DiagramStylePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::diagram_style_part::DiagramStylePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DiagramStylePart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml",
          "dialogsheets",
          "sheet",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::dialogsheet_part::DialogsheetPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::dialogsheet_part::DialogsheetPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DialogsheetPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin",
          "application/vnd.openxmlformats-package.digital-signature-origin",
          "_xmlsignatures",
          "origin",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DigitalSignatureOriginPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml",
          ".",
          "settings",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::document_settings_part::DocumentSettingsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::document_settings_part::DocumentSettingsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DocumentSettingsPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2019/05/relationships/documenttasks",
          "application/vnd.ms-office.documenttasks+xml",
          ".",
          "tasks",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::document_tasks_part::DocumentTasksPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::document_tasks_part::DocumentTasksPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DocumentTasksPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing",
          "application/vnd.openxmlformats-officedocument.drawing+xml",
          "../drawings",
          "drawing",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::drawings_part::DrawingsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::drawings_part::DrawingsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::DrawingsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary",
          "",
          ".",
          "ActiveXControl",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::EmbeddedControlPersistenceBinaryDataPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control",
          "",
          "embeddings",
          "control",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::EmbeddedControlPersistencePart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
          "",
          "embeddings",
          "embeddedObject",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::embedded_object_part::EmbeddedObjectPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::embedded_object_part::EmbeddedObjectPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::EmbeddedObjectPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
          "",
          "embeddings",
          "package",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::embedded_package_part::EmbeddedPackagePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::embedded_package_part::EmbeddedPackagePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::EmbeddedPackagePart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml",
          ".",
          "endnotes",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::endnotes_part::EndnotesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::endnotes_part::EndnotesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::EndnotesPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars",
          "application/vnd.ms-excel.attachedToolbars",
          ".",
          "attachedToolbars",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ExcelAttachedToolbarsPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2014/relationships/chartEx",
          "application/vnd.ms-office.chartex+xml",
          "extendedCharts",
          "chart",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::extended_chart_part::ExtendedChartPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::extended_chart_part::ExtendedChartPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ExtendedChartPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties",
          "application/vnd.openxmlformats-officedocument.extended-properties+xml",
          "docProps",
          "app",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ExtendedFilePropertiesPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml",
          "externalReferences",
          "externalReference",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::external_workbook_part::ExternalWorkbookPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::external_workbook_part::ExternalWorkbookPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ExternalWorkbookPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag",
          "application/vnd.ms-excel.featurepropertybag+xml",
          "featurePropertyBag",
          "featurePropertyBag",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::feature_property_bags_part::FeaturePropertyBagsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::feature_property_bags_part::FeaturePropertyBagsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::FeaturePropertyBagsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font",
          "",
          "fonts",
          "font",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::font_part::FontPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::font_part::FontPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::FontPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml",
          ".",
          "fontTable",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::font_table_part::FontTablePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::font_table_part::FontTablePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::FontTablePart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml",
          ".",
          "footer",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::footer_part::FooterPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::footer_part::FooterPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::FooterPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml",
          ".",
          "footnotes",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::footnotes_part::FootnotesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::footnotes_part::FootnotesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::FootnotesPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/glossaryDocument",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml",
          "glossary",
          "document",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::glossary_document_part::GlossaryDocumentPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::glossary_document_part::GlossaryDocumentPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::GlossaryDocumentPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster",
          "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml",
          "handoutMasters",
          "handoutMaster",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::handout_master_part::HandoutMasterPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::handout_master_part::HandoutMasterPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::HandoutMasterPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml",
          ".",
          "header",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::header_part::HeaderPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::header_part::HeaderPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::HeaderPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
          "",
          "../media",
          "image",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::image_part::ImagePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::image_part::ImagePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ImagePart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2006/relationships/xlIntlMacrosheet",
          "application/vnd.ms-excel.intlmacrosheet+xml",
          "macrosheets",
          "intlsheet",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::international_macro_sheet_part::InternationalMacroSheetPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::international_macro_sheet_part::InternationalMacroSheetPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::InternationalMacroSheetPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels",
          "application/vnd.ms-office.classificationlabels+xml",
          "docMetadata",
          "LabelInfo",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::label_info_part::LabelInfoPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::label_info_part::LabelInfoPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::LabelInfoPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2006/relationships/legacyDocTextInfo",
          "application/vnd.ms-office.legacyDocTextInfo",
          ".",
          "legacyDocTextInfo",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::LegacyDiagramTextInfoPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2006/relationships/legacyDiagramText",
          "application/vnd.ms-office.legacyDiagramText",
          ".",
          "legacyDiagramText",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::LegacyDiagramTextPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet",
          "application/vnd.ms-excel.macrosheet+xml",
          "macrosheets",
          "sheet",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::macro_sheet_part::MacroSheetPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::macro_sheet_part::MacroSheetPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::MacroSheetPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/recipientData",
          "",
          ".",
          "recipients",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::MailMergeRecipientDataPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
          "",
          "word",
          "document",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::main_document_part::MainDocumentPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::main_document_part::MainDocumentPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::MainDocumentPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/06/relationships/model3d",
          "model/gltf-binary",
          "../media",
          "model3d",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::Model3DReferenceRelationshipPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2019/04/relationships/namedSheetView",
          "application/vnd.ms-excel.namedsheetviews+xml",
          "../namedSheetViews",
          "namedSheetView",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::named_sheet_views_part::NamedSheetViewsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::named_sheet_views_part::NamedSheetViewsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::NamedSheetViewsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster",
          "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml",
          "notesMasters",
          "notesMaster",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::notes_master_part::NotesMasterPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::notes_master_part::NotesMasterPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::NotesMasterPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide",
          "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml",
          "../notesSlides",
          "notesSlide",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::notes_slide_part::NotesSlidePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::notes_slide_part::NotesSlidePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::NotesSlidePart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml",
          ".",
          "numbering",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::numbering_definitions_part::NumberingDefinitionsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::numbering_definitions_part::NumberingDefinitionsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::NumberingDefinitionsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml",
          "../pivotCache",
          "pivotCacheDefinition",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::PivotTableCacheDefinitionPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheRecords",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml",
          ".",
          "pivotCacheRecords",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::PivotTableCacheRecordsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml",
          "../pivotTables",
          "pivotTable",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::pivot_table_part::PivotTablePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::pivot_table_part::PivotTablePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::PivotTablePart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2018/10/relationships/authors",
          "application/vnd.ms-powerpoint.authors+xml",
          ".",
          "authors",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::power_point_authors_part::PowerPointAuthorsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::power_point_authors_part::PowerPointAuthorsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::PowerPointAuthorsPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2018/10/relationships/comments",
          "application/vnd.ms-powerpoint.comments+xml",
          "../comments",
          "modernComment",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::power_point_comment_part::PowerPointCommentPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::power_point_comment_part::PowerPointCommentPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::PowerPointCommentPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
          "",
          "ppt",
          "presentation",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::presentation_part::PresentationPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::presentation_part::PresentationPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::PresentationPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps",
          "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml",
          ".",
          "presProps",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::presentation_properties_part::PresentationPropertiesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::presentation_properties_part::PresentationPropertiesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::PresentationPropertiesPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/queryTable",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml",
          "../queryTables",
          "queryTable",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::query_table_part::QueryTablePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::query_table_part::QueryTablePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::QueryTablePart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2006/relationships/ui/userCustomization",
          "application/xml",
          "userCustomization",
          "customUI",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::QuickAccessToolbarCustomizationsPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/06/relationships/rdArray",
          "application/vnd.ms-excel.rdarray+xml",
          "richData",
          "rdarray",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::rd_array_part::RdArrayPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::rd_array_part::RdArrayPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RdArrayPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue",
          "application/vnd.ms-excel.rdrichvalue+xml",
          "richData",
          "rdrichvalue",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::rd_rich_value_part::RdRichValuePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::rd_rich_value_part::RdRichValuePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RdRichValuePart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure",
          "application/vnd.ms-excel.rdrichvaluestructure+xml",
          "richData",
          "rdrichvaluestructure",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RdRichValueStructurePart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes",
          "application/vnd.ms-excel.rdrichvaluetypes+xml",
          "richData",
          "rdRichValueTypes",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::rd_rich_value_types_part::RdRichValueTypesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::rd_rich_value_types_part::RdRichValueTypesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RdRichValueTypesPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2020/07/relationships/rdRichValueWebImage",
          "application/vnd.ms-excel.rdrichvaluewebimage+xml",
          "richData",
          "rdRichValueWebImage",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RdRichValueWebImagePart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag",
          "application/vnd.ms-excel.rdsupportingpropertybag+xml",
          "richData",
          "rdsupportingpropertybag",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RdSupportingPropertyBagPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure",
          "application/vnd.ms-excel.rdsupportingpropertybagstructure+xml",
          "richData",
          "rdsupportingpropertybagstructure",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RdSupportingPropertyBagStructurePart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility",
          "application/xml",
          "customUI",
          "customUI",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RibbonAndBackstageCustomizationsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2006/relationships/ui/extensibility",
          "application/xml",
          "customUI",
          "customUI",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RibbonExtensibilityPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/06/relationships/richStyles",
          "application/vnd.ms-excel.richstyles+xml",
          "richData",
          "richStyles",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::rich_styles_part::RichStylesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::rich_styles_part::RichStylesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::RichStylesPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml",
          ".",
          "sharedStrings",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::shared_string_table_part::SharedStringTablePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::shared_string_table_part::SharedStringTablePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SharedStringTablePart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableSingleCells",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml",
          "../tables",
          "tableSingleCells",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::single_cell_table_part::SingleCellTablePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::single_cell_table_part::SingleCellTablePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SingleCellTablePart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2007/relationships/slicerCache",
          "application/vnd.ms-excel.slicerCache+xml",
          "slicerCaches",
          "slicerCache",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::slicer_cache_part::SlicerCachePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::slicer_cache_part::SlicerCachePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SlicerCachePart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2007/relationships/slicer",
          "application/vnd.ms-excel.slicer+xml",
          "../slicers",
          "slicer",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::slicers_part::SlicersPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::slicers_part::SlicersPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SlicersPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
          "application/vnd.openxmlformats-officedocument.presentationml.comments+xml",
          "../comments",
          "comment",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::slide_comments_part::SlideCommentsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::slide_comments_part::SlideCommentsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SlideCommentsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout",
          "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml",
          "../slideLayouts",
          "slideLayout",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::slide_layout_part::SlideLayoutPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::slide_layout_part::SlideLayoutPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SlideLayoutPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster",
          "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml",
          "slideMasters",
          "slideMaster",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::slide_master_part::SlideMasterPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::slide_master_part::SlideMasterPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SlideMasterPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
          "application/vnd.openxmlformats-officedocument.presentationml.slide+xml",
          "slides",
          "slide",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::slide_part::SlidePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::slide_part::SlidePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SlidePart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideUpdateInfo",
          "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml",
          "slideUpdateInfo",
          "slideUpdateInfo",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::slide_sync_data_part::SlideSyncDataPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::slide_sync_data_part::SlideSyncDataPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SlideSyncDataPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.printerSettings",
          "../printerSettings",
          "printerSettings",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::SpreadsheetPrinterSettingsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml",
          ".",
          "styles",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::style_definitions_part::StyleDefinitionsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::style_definitions_part::StyleDefinitionsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::StyleDefinitionsPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects",
          "application/vnd.ms-word.stylesWithEffects+xml",
          ".",
          "stylesWithEffects",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::styles_with_effects_part::StylesWithEffectsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::styles_with_effects_part::StylesWithEffectsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::StylesWithEffectsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml",
          "../tables",
          "table",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::table_definition_part::TableDefinitionPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::table_definition_part::TableDefinitionPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::TableDefinitionPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles",
          "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml",
          ".",
          "tableStyles",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::table_styles_part::TableStylesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::table_styles_part::TableStylesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::TableStylesPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride",
          "application/vnd.openxmlformats-officedocument.themeOverride+xml",
          "theme",
          "themeoverride",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::theme_override_part::ThemeOverridePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::theme_override_part::ThemeOverridePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ThemeOverridePart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
          "application/vnd.openxmlformats-officedocument.theme+xml",
          "theme",
          "theme",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::theme_part::ThemePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::theme_part::ThemePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ThemePart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail",
          "",
          "docProps",
          "thumbnail",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::thumbnail_part::ThumbnailPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::thumbnail_part::ThumbnailPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ThumbnailPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2011/relationships/timelineCache",
          "application/vnd.ms-excel.timelineCache+xml",
          "timelineCaches",
          "timelineCache",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::time_line_cache_part::TimeLineCachePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::time_line_cache_part::TimeLineCachePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::TimeLineCachePart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2011/relationships/timeline",
          "application/vnd.ms-excel.timeline+xml",
          "../timelines",
          "timeline",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::time_line_part::TimeLinePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::time_line_part::TimeLinePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::TimeLinePart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags",
          "application/vnd.openxmlformats-officedocument.presentationml.tags+xml",
          "tags",
          "tag",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::user_defined_tags_part::UserDefinedTagsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::user_defined_tags_part::UserDefinedTagsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::UserDefinedTagsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2006/relationships/wordVbaData",
          "application/vnd.ms-word.vbaData+xml",
          ".",
          "vbaData",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::vba_data_part::VbaDataPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::vba_data_part::VbaDataPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::VbaDataPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2006/relationships/vbaProject",
          "application/vnd.ms-office.vbaProject",
          ".",
          "vbaProject",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::vba_project_part::VbaProjectPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::vba_project_part::VbaProjectPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::VbaProjectPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps",
          "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml",
          ".",
          "viewProps",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::view_properties_part::ViewPropertiesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::view_properties_part::ViewPropertiesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::ViewPropertiesPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing",
          "application/vnd.openxmlformats-officedocument.vmlDrawing",
          "../drawings",
          "vmldrawing",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::vml_drawing_part::VmlDrawingPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::vml_drawing_part::VmlDrawingPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::VmlDrawingPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/volatileDependencies",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml",
          ".",
          "volatileDependencies",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::volatile_dependencies_part::VolatileDependenciesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::volatile_dependencies_part::VolatileDependenciesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::VolatileDependenciesPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes",
          "application/vnd.ms-office.webextensiontaskpanes+xml",
          "../webextensions",
          "taskpanes",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WebExTaskpanesPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2011/relationships/webextension",
          "application/vnd.ms-office.webextension+xml",
          "../webextensions",
          "webextension",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::web_extension_part::WebExtensionPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::web_extension_part::WebExtensionPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WebExtensionPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml",
          ".",
          "webSettings",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::web_settings_part::WebSettingsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::web_settings_part::WebSettingsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WebSettingsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars",
          "application/vnd.ms-word.attachedToolbars",
          ".",
          "attachedToolbars",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WordAttachedToolbarsPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2018/08/relationships/commentsExtensible",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtensible+xml",
          ".",
          "commentsExtensible",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WordCommentsExtensiblePart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2011/relationships/commentsExtended",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtended+xml",
          ".",
          "commentsExtended",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WordprocessingCommentsExPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2016/09/relationships/commentsIds",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsIds+xml",
          ".",
          "commentsIds",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WordprocessingCommentsIdsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml",
          ".",
          "comments",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WordprocessingCommentsPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2011/relationships/people",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.people+xml",
          ".",
          "people",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::wordprocessing_people_part::WordprocessingPeoplePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::wordprocessing_people_part::WordprocessingPeoplePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WordprocessingPeoplePart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.printerSettings",
          "../printerSettings",
          "printerSettings",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WordprocessingPrinterSettingsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
          "",
          "xl",
          "workbook",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::workbook_part::WorkbookPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::workbook_part::WorkbookPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorkbookPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/10/relationships/person",
          "application/vnd.ms-excel.person+xml",
          "persons",
          "person",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::workbook_person_part::WorkbookPersonPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::workbook_person_part::WorkbookPersonPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorkbookPersonPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml",
          "revisions",
          "revisionHeaders",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorkbookRevisionHeaderPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionLog",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml",
          ".",
          "revisionLog",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorkbookRevisionLogPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml",
          ".",
          "styles",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::workbook_styles_part::WorkbookStylesPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::workbook_styles_part::WorkbookStylesPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorkbookStylesPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/usernames",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml",
          "revisions",
          "userNames",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::workbook_user_data_part::WorkbookUserDataPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::workbook_user_data_part::WorkbookUserDataPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorkbookUserDataPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml",
          "..",
          "comments",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::worksheet_comments_part::WorksheetCommentsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::worksheet_comments_part::WorksheetCommentsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorksheetCommentsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet",
          "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml",
          "worksheets",
          "sheet",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::worksheet_part::WorksheetPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::worksheet_part::WorksheetPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorksheetPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2006/relationships/wsSortMap",
          "application/vnd.ms-excel.wsSortMap+xml",
          ".",
          "wsSortMap",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::worksheet_sort_map_part::WorksheetSortMapPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::worksheet_sort_map_part::WorksheetSortMapPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorksheetSortMapPart(part));
        }
        #[cfg(feature = "microsoft365")]
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/10/relationships/threadedComment",
          "application/vnd.ms-excel.threadedcomments+xml",
          "../threadedcomments",
          "threadedcomment",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::WorksheetThreadedCommentsPart(part));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/signature",
          "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml",
          "_xmlsignatures",
          "sig",
        ) {
          let part = if let Some(relationship_id) = relationship_id {
            <crate::parts::xml_signature_part::XmlSignaturePart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                            storage,
                            relationship_id,
                            part_id,
                        )
          } else {
            <crate::parts::xml_signature_part::XmlSignaturePart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                            storage,
                            part_id,
                        )
          };
          return Some(PartRef::XmlSignaturePart(part));
        }
      }
    }
    let part = if let Some(relationship_id) = relationship_id {
      <crate::parts::extended_part::ExtendedPart as crate::sdk::SdkPartInternal>::from_relationship_id_with_relationships(
                storage,
                relationship_id,
                part_id,
            )
    } else {
      <crate::parts::extended_part::ExtendedPart as crate::sdk::SdkPartInternal>::from_part_id_with_relationships(
                storage,
                part_id,
            )
    };
    Some(PartRef::ExtendedPart(part))
  }
}
#[derive(Clone, Debug)]
pub struct IdPartPair<'a> {
  pub relationship_id: &'a str,
  pub part: PartRef,
}
impl<'a> IdPartPair<'a> {
  pub const fn new(relationship_id: &'a str, part: PartRef) -> Self {
    Self {
      relationship_id,
      part,
    }
  }
}
macro_rules! define_part_root_element {
    (
        $($(#[$attrs:meta])* $variant:ident ($root_ty:ty, $root_accessor:ident,
        $root_accessor_mut:ident, $content_type:literal),)*
    ) => {
        #[derive(Clone, Debug)] pub enum PartRootElement { $($(#[$attrs])* $variant (Box
        < $root_ty >),)* } impl PartRootElement { pub fn part_type_name(& self) -> &
        'static str { match self { $($(#[$attrs])* Self:: $variant (_) =>
        stringify!($variant),)* } } $($(#[$attrs])* pub fn $root_accessor (& self) ->
        Option < & $root_ty > { match self { Self:: $variant (root) => Some(root
        .as_ref()), _ => None, } } $(#[$attrs])* pub fn $root_accessor_mut (& mut self)
        -> Option < & mut $root_ty > { match self { Self:: $variant (root) => Some(root
        .as_mut()), _ => None, } })* pub fn to_xml_bytes(& self) -> Result < Vec < u8 >,
        crate ::common::SdkError > { match self { $($(#[$attrs])* Self:: $variant (root)
        => Ok(root.to_xml_bytes() ?),)* } } pub (crate) fn from_part_id(storage : & crate
        ::common::SdkPackageStorage, part_id : crate ::common::PartId,) -> Result <
        Option < Self >, crate ::common::SdkError > { let Some(part) = storage
        .part(part_id) else { return Ok(None); }; $($(#[$attrs])* if !
        matches!($content_type, "" | "application/xml" | "text/xml") && part
        .content_type() == $content_type { return < $root_ty > ::from_bytes(part.data()
        .bytes()).map(Box::new).map(Self:: $variant).map(Some); })* Ok(None) } }
    };
}
define_part_root_element! {
    CalculationChainPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CalculationChain,
    as_calculation_chain_part, as_calculation_chain_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml"),
    CellMetadataPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Metadata,
    as_cell_metadata_part, as_cell_metadata_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml"),
    #[cfg(feature = "microsoft365")] ChartColorStylePart(crate
    ::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ColorStyle,
    as_chart_color_style_part, as_chart_color_style_part_mut,
    "application/vnd.ms-office.chartcolorstyle+xml"), ChartDrawingPart(crate
    ::schemas::schemas_openxmlformats_org_drawingml_2006_chart::UserShapes,
    as_chart_drawing_part, as_chart_drawing_part_mut,
    "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml"),
    ChartPart(crate
    ::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartSpace,
    as_chart_part, as_chart_part_mut,
    "application/vnd.openxmlformats-officedocument.drawingml.chart+xml"), #[cfg(feature =
    "microsoft365")] ChartStylePart(crate
    ::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ChartStyle,
    as_chart_style_part, as_chart_style_part_mut,
    "application/vnd.ms-office.chartstyle+xml"), ChartsheetPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Chartsheet,
    as_chartsheet_part, as_chartsheet_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml"),
    CommentAuthorsPart(crate
    ::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentAuthorList,
    as_comment_authors_part, as_comment_authors_part_mut,
    "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml"),
    ConnectionsPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Connections,
    as_connections_part, as_connections_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml"),
    #[cfg(feature = "microsoft365")] ControlPropertiesPart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::FormControlProperties,
    as_control_properties_part, as_control_properties_part_mut,
    "application/vnd.ms-excel.controlproperties+xml"), CoreFilePropertiesPart(crate
    ::schemas::opc_core_properties::CoreProperties, as_core_file_properties_part,
    as_core_file_properties_part_mut,
    "application/vnd.openxmlformats-package.core-properties+xml"), #[cfg(feature =
    "microsoft365")] CustomDataPropertiesPart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DatastoreItem,
    as_custom_data_properties_part, as_custom_data_properties_part_mut,
    "application/vnd.ms-excel.customDataProperties+xml"), CustomFilePropertiesPart(crate
    ::schemas::schemas_openxmlformats_org_office_document_2006_custom_properties::Properties,
    as_custom_file_properties_part, as_custom_file_properties_part_mut,
    "application/vnd.openxmlformats-officedocument.custom-properties+xml"),
    CustomXmlMappingsPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::MapInfo,
    as_custom_xml_mappings_part, as_custom_xml_mappings_part_mut, "application/xml"),
    CustomXmlPropertiesPart(crate
    ::schemas::schemas_openxmlformats_org_office_document_2006_custom_xml::DataStoreItem,
    as_custom_xml_properties_part, as_custom_xml_properties_part_mut,
    "application/vnd.openxmlformats-officedocument.customXmlProperties+xml"),
    CustomizationPart(crate
    ::schemas::schemas_microsoft_com_office_word_2006_wordml::TemplateCommandGroup,
    as_customization_part, as_customization_part_mut,
    "application/vnd.ms-word.keyMapCustomizations+xml"), DiagramColorsPart(crate
    ::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::ColorsDefinition,
    as_diagram_colors_part, as_diagram_colors_part_mut,
    "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml"),
    DiagramDataPart(crate
    ::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::DataModelRoot,
    as_diagram_data_part, as_diagram_data_part_mut,
    "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml"),
    DiagramLayoutDefinitionPart(crate
    ::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::LayoutDefinition,
    as_diagram_layout_definition_part, as_diagram_layout_definition_part_mut,
    "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml"),
    #[cfg(feature = "microsoft365")] DiagramPersistLayoutPart(crate
    ::schemas::schemas_microsoft_com_office_drawing_2008_diagram::Drawing,
    as_diagram_persist_layout_part, as_diagram_persist_layout_part_mut,
    "application/vnd.ms-office.drawingml.diagramDrawing+xml"), DiagramStylePart(crate
    ::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::StyleDefinition,
    as_diagram_style_part, as_diagram_style_part_mut,
    "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml"),
    DialogsheetPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DialogSheet,
    as_dialogsheet_part, as_dialogsheet_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml"),
    DocumentSettingsPart(crate
    ::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Settings,
    as_document_settings_part, as_document_settings_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"),
    #[cfg(feature = "microsoft365")] DocumentTasksPart(crate
    ::schemas::schemas_microsoft_com_office_tasks_2019_documenttasks::Tasks,
    as_document_tasks_part, as_document_tasks_part_mut,
    "application/vnd.ms-office.documenttasks+xml"), DrawingsPart(crate
    ::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::WorksheetDrawing,
    as_drawings_part, as_drawings_part_mut,
    "application/vnd.openxmlformats-officedocument.drawing+xml"), EndnotesPart(crate
    ::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Endnotes,
    as_endnotes_part, as_endnotes_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml"),
    #[cfg(feature = "microsoft365")] ExtendedChartPart(crate
    ::schemas::schemas_microsoft_com_office_drawing_2014_chartex::ChartSpace,
    as_extended_chart_part, as_extended_chart_part_mut,
    "application/vnd.ms-office.chartex+xml"), ExtendedFilePropertiesPart(crate
    ::schemas::schemas_openxmlformats_org_office_document_2006_extended_properties::Properties,
    as_extended_file_properties_part, as_extended_file_properties_part_mut,
    "application/vnd.openxmlformats-officedocument.extended-properties+xml"),
    ExternalWorkbookPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExternalLink,
    as_external_workbook_part, as_external_workbook_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml"),
    #[cfg(feature = "microsoft365")] FeaturePropertyBagsPart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2022_featurepropertybag::FeaturePropertyBags,
    as_feature_property_bags_part, as_feature_property_bags_part_mut,
    "application/vnd.ms-excel.featurepropertybag+xml"), FontTablePart(crate
    ::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Fonts,
    as_font_table_part, as_font_table_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"),
    FooterPart(crate
    ::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Footer,
    as_footer_part, as_footer_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"),
    FootnotesPart(crate
    ::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Footnotes,
    as_footnotes_part, as_footnotes_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml"),
    GlossaryDocumentPart(crate
    ::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::GlossaryDocument,
    as_glossary_document_part, as_glossary_document_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml"),
    HandoutMasterPart(crate
    ::schemas::schemas_openxmlformats_org_presentationml_2006_main::HandoutMaster,
    as_handout_master_part, as_handout_master_part_mut,
    "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml"),
    HeaderPart(crate
    ::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Header,
    as_header_part, as_header_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml"),
    #[cfg(feature = "microsoft365")] LabelInfoPart(crate
    ::schemas::schemas_microsoft_com_office_2020_mip_label_metadata::ClassificationLabelList,
    as_label_info_part, as_label_info_part_mut,
    "application/vnd.ms-office.classificationlabels+xml"), MacroSheetPart(crate
    ::schemas::schemas_microsoft_com_office_excel_2006_main::Macrosheet,
    as_macro_sheet_part, as_macro_sheet_part_mut,
    "application/vnd.ms-excel.macrosheet+xml"), MainDocumentPart(crate
    ::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document,
    as_main_document_part, as_main_document_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"),
    #[cfg(feature = "microsoft365")] NamedSheetViewsPart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2019_namedsheetviews::NamedSheetViews,
    as_named_sheet_views_part, as_named_sheet_views_part_mut,
    "application/vnd.ms-excel.namedsheetviews+xml"), NotesMasterPart(crate
    ::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesMaster,
    as_notes_master_part, as_notes_master_part_mut,
    "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml"),
    NotesSlidePart(crate
    ::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesSlide,
    as_notes_slide_part, as_notes_slide_part_mut,
    "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"),
    NumberingDefinitionsPart(crate
    ::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Numbering,
    as_numbering_definitions_part, as_numbering_definitions_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"),
    PivotTableCacheDefinitionPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheDefinition,
    as_pivot_table_cache_definition_part, as_pivot_table_cache_definition_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml"),
    PivotTableCacheRecordsPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheRecords,
    as_pivot_table_cache_records_part, as_pivot_table_cache_records_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml"),
    PivotTablePart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableDefinition,
    as_pivot_table_part, as_pivot_table_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml"),
    #[cfg(feature = "microsoft365")] PowerPointAuthorsPart(crate
    ::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::AuthorList,
    as_power_point_authors_part, as_power_point_authors_part_mut,
    "application/vnd.ms-powerpoint.authors+xml"), #[cfg(feature = "microsoft365")]
    PowerPointCommentPart(crate
    ::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::CommentList,
    as_power_point_comment_part, as_power_point_comment_part_mut,
    "application/vnd.ms-powerpoint.comments+xml"), PresentationPart(crate
    ::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation,
    as_presentation_part, as_presentation_part_mut,
    "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"),
    PresentationPropertiesPart(crate
    ::schemas::schemas_openxmlformats_org_presentationml_2006_main::PresentationProperties,
    as_presentation_properties_part, as_presentation_properties_part_mut,
    "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml"),
    QueryTablePart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::QueryTable,
    as_query_table_part, as_query_table_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml"),
    #[cfg(feature = "microsoft365")] RdArrayPart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::ArrayData,
    as_rd_array_part, as_rd_array_part_mut, "application/vnd.ms-excel.rdarray+xml"),
    #[cfg(feature = "microsoft365")] RdRichValuePart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueData,
    as_rd_rich_value_part, as_rd_rich_value_part_mut,
    "application/vnd.ms-excel.rdrichvalue+xml"), #[cfg(feature = "microsoft365")]
    RdRichValueStructurePart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueStructures,
    as_rd_rich_value_structure_part, as_rd_rich_value_structure_part_mut,
    "application/vnd.ms-excel.rdrichvaluestructure+xml"), #[cfg(feature =
    "microsoft365")] RdRichValueTypesPart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::RichValueTypesInfo,
    as_rd_rich_value_types_part, as_rd_rich_value_types_part_mut,
    "application/vnd.ms-excel.rdrichvaluetypes+xml"), #[cfg(feature = "microsoft365")]
    RdRichValueWebImagePart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2020_richdatawebimage::WebImagesSupportingRichData,
    as_rd_rich_value_web_image_part, as_rd_rich_value_web_image_part_mut,
    "application/vnd.ms-excel.rdrichvaluewebimage+xml"), #[cfg(feature = "microsoft365")]
    RdSupportingPropertyBagPart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBags,
    as_rd_supporting_property_bag_part, as_rd_supporting_property_bag_part_mut,
    "application/vnd.ms-excel.rdsupportingpropertybag+xml"), #[cfg(feature =
    "microsoft365")] RdSupportingPropertyBagStructurePart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBagStructures,
    as_rd_supporting_property_bag_structure_part,
    as_rd_supporting_property_bag_structure_part_mut,
    "application/vnd.ms-excel.rdsupportingpropertybagstructure+xml"), #[cfg(feature =
    "microsoft365")] RibbonAndBackstageCustomizationsPart(crate
    ::schemas::schemas_microsoft_com_office_2009_07_customui::CustomUi,
    as_ribbon_and_backstage_customizations_part,
    as_ribbon_and_backstage_customizations_part_mut, "application/xml"), #[cfg(feature =
    "microsoft365")] RichStylesPart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::RichStylesheet,
    as_rich_styles_part, as_rich_styles_part_mut,
    "application/vnd.ms-excel.richstyles+xml"), SharedStringTablePart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SharedStringTable,
    as_shared_string_table_part, as_shared_string_table_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"),
    SingleCellTablePart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SingleXmlCells,
    as_single_cell_table_part, as_single_cell_table_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml"),
    #[cfg(feature = "microsoft365")] SlicerCachePart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerCacheDefinition,
    as_slicer_cache_part, as_slicer_cache_part_mut,
    "application/vnd.ms-excel.slicerCache+xml"), #[cfg(feature = "microsoft365")]
    SlicersPart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Slicers,
    as_slicers_part, as_slicers_part_mut, "application/vnd.ms-excel.slicer+xml"),
    SlideCommentsPart(crate
    ::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentList,
    as_slide_comments_part, as_slide_comments_part_mut,
    "application/vnd.openxmlformats-officedocument.presentationml.comments+xml"),
    SlideLayoutPart(crate
    ::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideLayout,
    as_slide_layout_part, as_slide_layout_part_mut,
    "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"),
    SlideMasterPart(crate
    ::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideMaster,
    as_slide_master_part, as_slide_master_part_mut,
    "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"),
    SlidePart(crate
    ::schemas::schemas_openxmlformats_org_presentationml_2006_main::Slide, as_slide_part,
    as_slide_part_mut,
    "application/vnd.openxmlformats-officedocument.presentationml.slide+xml"),
    SlideSyncDataPart(crate
    ::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideSyncProperties,
    as_slide_sync_data_part, as_slide_sync_data_part_mut,
    "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml"),
    StyleDefinitionsPart(crate
    ::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles,
    as_style_definitions_part, as_style_definitions_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"),
    #[cfg(feature = "microsoft365")] StylesWithEffectsPart(crate
    ::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles,
    as_styles_with_effects_part, as_styles_with_effects_part_mut,
    "application/vnd.ms-word.stylesWithEffects+xml"), TableDefinitionPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Table,
    as_table_definition_part, as_table_definition_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml"),
    TableStylesPart(crate
    ::schemas::schemas_openxmlformats_org_drawingml_2006_main::TableStyleList,
    as_table_styles_part, as_table_styles_part_mut,
    "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml"),
    ThemeOverridePart(crate
    ::schemas::schemas_openxmlformats_org_drawingml_2006_main::ThemeOverride,
    as_theme_override_part, as_theme_override_part_mut,
    "application/vnd.openxmlformats-officedocument.themeOverride+xml"), ThemePart(crate
    ::schemas::schemas_openxmlformats_org_drawingml_2006_main::Theme, as_theme_part,
    as_theme_part_mut, "application/vnd.openxmlformats-officedocument.theme+xml"),
    #[cfg(feature = "microsoft365")] TimeLineCachePart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineCacheDefinition,
    as_time_line_cache_part, as_time_line_cache_part_mut,
    "application/vnd.ms-excel.timelineCache+xml"), #[cfg(feature = "microsoft365")]
    TimeLinePart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::Timelines,
    as_time_line_part, as_time_line_part_mut, "application/vnd.ms-excel.timeline+xml"),
    UserDefinedTagsPart(crate
    ::schemas::schemas_openxmlformats_org_presentationml_2006_main::TagList,
    as_user_defined_tags_part, as_user_defined_tags_part_mut,
    "application/vnd.openxmlformats-officedocument.presentationml.tags+xml"),
    VbaDataPart(crate
    ::schemas::schemas_microsoft_com_office_word_2006_wordml::VbaSuppData,
    as_vba_data_part, as_vba_data_part_mut, "application/vnd.ms-word.vbaData+xml"),
    ViewPropertiesPart(crate
    ::schemas::schemas_openxmlformats_org_presentationml_2006_main::ViewProperties,
    as_view_properties_part, as_view_properties_part_mut,
    "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml"),
    VolatileDependenciesPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::VolatileTypes,
    as_volatile_dependencies_part, as_volatile_dependencies_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml"),
    #[cfg(feature = "microsoft365")] WebExTaskpanesPart(crate
    ::schemas::schemas_microsoft_com_office_webextensions_taskpanes_2010_11::Taskpanes,
    as_web_ex_taskpanes_part, as_web_ex_taskpanes_part_mut,
    "application/vnd.ms-office.webextensiontaskpanes+xml"), #[cfg(feature =
    "microsoft365")] WebExtensionPart(crate
    ::schemas::schemas_microsoft_com_office_webextensions_webextension_2010_11::WebExtension,
    as_web_extension_part, as_web_extension_part_mut,
    "application/vnd.ms-office.webextension+xml"), WebSettingsPart(crate
    ::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::WebSettings,
    as_web_settings_part, as_web_settings_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml"),
    #[cfg(feature = "microsoft365")] WordCommentsExtensiblePart(crate
    ::schemas::schemas_microsoft_com_office_word_2018_wordml_cex::CommentsExtensible,
    as_word_comments_extensible_part, as_word_comments_extensible_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtensible+xml"),
    #[cfg(feature = "microsoft365")] WordprocessingCommentsExPart(crate
    ::schemas::schemas_microsoft_com_office_word_2012_wordml::CommentsEx,
    as_wordprocessing_comments_ex_part, as_wordprocessing_comments_ex_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtended+xml"),
    #[cfg(feature = "microsoft365")] WordprocessingCommentsIdsPart(crate
    ::schemas::schemas_microsoft_com_office_word_2016_wordml_cid::CommentsIds,
    as_wordprocessing_comments_ids_part, as_wordprocessing_comments_ids_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsIds+xml"),
    WordprocessingCommentsPart(crate
    ::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Comments,
    as_wordprocessing_comments_part, as_wordprocessing_comments_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"),
    #[cfg(feature = "microsoft365")] WordprocessingPeoplePart(crate
    ::schemas::schemas_microsoft_com_office_word_2012_wordml::People,
    as_wordprocessing_people_part, as_wordprocessing_people_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.people+xml"),
    WorkbookPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook,
    as_workbook_part, as_workbook_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"),
    #[cfg(feature = "microsoft365")] WorkbookPersonPart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::PersonList,
    as_workbook_person_part, as_workbook_person_part_mut,
    "application/vnd.ms-excel.person+xml"), WorkbookRevisionHeaderPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Headers,
    as_workbook_revision_header_part, as_workbook_revision_header_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml"),
    WorkbookRevisionLogPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Revisions,
    as_workbook_revision_log_part, as_workbook_revision_log_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml"),
    WorkbookStylesPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Stylesheet,
    as_workbook_styles_part, as_workbook_styles_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"),
    WorkbookUserDataPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Users,
    as_workbook_user_data_part, as_workbook_user_data_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml"),
    WorksheetCommentsPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Comments,
    as_worksheet_comments_part, as_worksheet_comments_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml"),
    WorksheetPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Worksheet,
    as_worksheet_part, as_worksheet_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"),
    WorksheetSortMapPart(crate
    ::schemas::schemas_microsoft_com_office_excel_2006_main::WorksheetSortMap,
    as_worksheet_sort_map_part, as_worksheet_sort_map_part_mut,
    "application/vnd.ms-excel.wsSortMap+xml"), #[cfg(feature = "microsoft365")]
    WorksheetThreadedCommentsPart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::ThreadedComments,
    as_worksheet_threaded_comments_part, as_worksheet_threaded_comments_part_mut,
    "application/vnd.ms-excel.threadedcomments+xml"),
}
pub(crate) fn initialize_root_elements(
  storage: &crate::common::SdkPackageStorage,
  open_mode: crate::common::PackageOpenMode,
) -> Result<Vec<Option<crate::parts::PartRootElement>>, crate::common::SdkError> {
  let mut root_elements = vec![None; storage.parts().len()];
  if matches!(open_mode, crate::common::PackageOpenMode::Eager) {
    for (index, slot) in root_elements.iter_mut().enumerate() {
      let part_id = crate::common::PartId::from_index(index);
      if let Some(root_element) = crate::parts::PartRootElement::from_part_id(storage, part_id)? {
        *slot = Some(root_element);
      }
    }
  }
  Ok(root_elements)
}
pub(crate) fn load_all_part_roots<P>(package: &mut P) -> Result<(), crate::common::SdkError>
where
  P: crate::sdk::SdkPackage,
{
  validate_missing_internal_relationships(package)?;
  let part_count = crate::sdk::SdkPackageInternal::storage(package)
    .parts()
    .len();
  for index in 0..part_count {
    let part_id = crate::common::PartId::from_index(index);
    if crate::sdk::SdkPackageInternal::storage(package)
      .part(part_id)
      .is_none()
      || crate::sdk::SdkPackageInternal::is_root_element_loaded(package, part_id)
    {
      continue;
    }
    let root_element = crate::parts::PartRootElement::from_part_id(
      crate::sdk::SdkPackageInternal::storage(package),
      part_id,
    )?;
    if let Some(root_element) = root_element
      && let Some(slot) = crate::sdk::SdkPackageInternal::root_element_slot_mut(package, part_id)
    {
      *slot = Some(root_element);
    }
  }
  Ok(())
}
fn validate_missing_internal_relationships<P>(package: &P) -> Result<(), crate::common::SdkError>
where
  P: crate::sdk::SdkPackage,
{
  let storage = crate::sdk::SdkPackageInternal::storage(package);
  let open_settings = crate::sdk::SdkPackageInternal::open_settings(package);
  for relationship in storage.package_relationships().iter() {
    validate_missing_internal_relationship(open_settings, relationship)?;
  }
  for part in storage.parts().iter().filter(|part| !part.is_deleted()) {
    for relationship in part.relationships().iter() {
      validate_missing_internal_relationship(open_settings, relationship)?;
    }
  }
  Ok(())
}
fn validate_missing_internal_relationship(
  open_settings: &crate::sdk::OpenSettings,
  relationship: &crate::common::RelationshipInfo,
) -> Result<(), crate::common::SdkError> {
  if !matches!(
    relationship.target_kind(),
    crate::common::RelationshipTargetKind::Missing
  ) || should_ignore_missing_relationship(open_settings, relationship)
  {
    return Ok(());
  }
  Err(crate::common::SdkError::CommonError(format!(
    "relationship {} targets missing part {}",
    relationship.id(),
    relationship.target()
  )))
}
fn should_ignore_missing_relationship(
  open_settings: &crate::sdk::OpenSettings,
  relationship: &crate::common::RelationshipInfo,
) -> bool {
  open_settings.ignore_calculation_chain_part_relationship
    && crate::common::relationship_type_matches(
      relationship.relationship_type(),
      crate::parts::calculation_chain_part::RELATIONSHIP_TYPE,
    )
}
pub fn save_package<P, W>(package: &P, writer: W) -> Result<(), crate::common::SdkError>
where
  P: crate::sdk::SdkPackage,
  W: std::io::Write + std::io::Seek,
{
  use std::io::Write as _;
  let mut zip = zip::ZipWriter::new(writer);
  let options = zip::write::SimpleFileOptions::default()
    .compression_method(zip::CompressionMethod::Deflated)
    .unix_permissions(0o755);
  let mut entry_set = std::collections::HashSet::<String>::new();
  let storage = crate::sdk::SdkPackageInternal::storage(package);
  zip.start_file("[Content_Types].xml", options)?;
  zip.write_all(&storage.content_types().to_xml_bytes()?)?;
  let package_relationships = storage.package_relationships();
  if !package_relationships.is_empty() {
    if entry_set.insert("_rels".to_string()) {
      zip.add_directory("_rels", options)?;
    }
    zip.start_file("_rels/.rels", options)?;
    zip.write_all(&package_relationships.to_relationships().to_xml_bytes()?)?;
  }
  for (index, part) in storage.parts().iter().enumerate() {
    if part.is_deleted() {
      continue;
    }
    let part_id = crate::common::PartId::from_index(index);
    let parent_path = crate::common::parent_zip_path(part.path());
    let directory_path = parent_path.strip_suffix('/').unwrap_or(&parent_path);
    if !directory_path.is_empty() && entry_set.insert(directory_path.to_string()) {
      zip.add_directory(directory_path, options)?;
    }
    if let Some(relationships) = storage.relationships(part_id)
      && !relationships.is_empty()
    {
      let rels_dir_path = crate::common::part_relationships_directory_path(part.path());
      if !rels_dir_path.is_empty() && entry_set.insert(rels_dir_path.clone()) {
        zip.add_directory(&rels_dir_path, options)?;
      }
      let rels_path = crate::common::part_relationships_path(part.path());
      zip.start_file(&rels_path, options)?;
      zip.write_all(&relationships.to_relationships().to_xml_bytes()?)?;
    }
    zip.start_file(part.path(), options)?;
    if let Some(root_element) = crate::sdk::SdkPackageInternal::root_element(package, part_id) {
      zip.write_all(&root_element.to_xml_bytes()?)?;
    } else {
      zip.write_all(part.data().bytes())?;
    }
  }
  zip.finish()?;
  Ok(())
}
