//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub mod alternative_format_import_part;
pub mod calculation_chain_part;
pub mod cell_metadata_part;
pub mod chart_color_style_part;
pub mod chart_drawing_part;
pub mod chart_part;
pub mod chart_style_part;
pub mod chartsheet_part;
pub mod comment_authors_part;
pub mod connections_part;
pub mod control_properties_part;
pub mod core_file_properties_part;
pub mod custom_data_part;
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
pub mod diagram_persist_layout_part;
pub mod diagram_style_part;
pub mod dialogsheet_part;
pub mod digital_signature_origin_part;
pub mod document_settings_part;
pub mod document_tasks_part;
pub mod drawings_part;
pub mod embedded_control_persistence_binary_data_part;
pub mod embedded_control_persistence_part;
pub mod embedded_object_part;
pub mod embedded_package_part;
pub mod endnotes_part;
pub mod excel_attached_toolbars_part;
pub mod extended_chart_part;
pub mod extended_file_properties_part;
pub mod extended_part;
pub mod external_workbook_part;
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
pub mod label_info_part;
pub mod legacy_diagram_text_info_part;
pub mod legacy_diagram_text_part;
pub mod macro_sheet_part;
pub mod mail_merge_recipient_data_part;
pub mod main_document_part;
pub mod model3_d_reference_relationship_part;
pub mod named_sheet_views_part;
pub mod notes_master_part;
pub mod notes_slide_part;
pub mod numbering_definitions_part;
pub mod pivot_table_cache_definition_part;
pub mod pivot_table_cache_records_part;
pub mod pivot_table_part;
pub mod power_point_authors_part;
pub mod power_point_comment_part;
pub mod presentation_document;
pub mod presentation_part;
pub mod presentation_properties_part;
pub mod query_table_part;
pub mod quick_access_toolbar_customizations_part;
pub mod rd_array_part;
pub mod rd_rich_value_part;
pub mod rd_rich_value_structure_part;
pub mod rd_rich_value_types_part;
pub mod rd_rich_value_web_image_part;
pub mod rd_supporting_property_bag_part;
pub mod rd_supporting_property_bag_structure_part;
pub mod ribbon_and_backstage_customizations_part;
pub mod ribbon_extensibility_part;
pub mod rich_styles_part;
pub mod shared_string_table_part;
pub mod single_cell_table_part;
pub mod slicer_cache_part;
pub mod slicers_part;
pub mod slide_comments_part;
pub mod slide_layout_part;
pub mod slide_master_part;
pub mod slide_part;
pub mod slide_sync_data_part;
pub mod spreadsheet_document;
pub mod spreadsheet_printer_settings_part;
pub mod style_definitions_part;
pub mod styles_with_effects_part;
pub mod table_definition_part;
pub mod table_styles_part;
pub mod theme_override_part;
pub mod theme_part;
pub mod thumbnail_part;
pub mod time_line_cache_part;
pub mod time_line_part;
pub mod user_defined_tags_part;
pub mod vba_data_part;
pub mod vba_project_part;
pub mod view_properties_part;
pub mod vml_drawing_part;
pub mod volatile_dependencies_part;
pub mod web_ex_taskpanes_part;
pub mod web_extension_part;
pub mod web_settings_part;
pub mod word_attached_toolbars_part;
pub mod word_comments_extensible_part;
pub mod wordprocessing_comments_ex_part;
pub mod wordprocessing_comments_ids_part;
pub mod wordprocessing_comments_part;
pub mod wordprocessing_document;
pub mod wordprocessing_people_part;
pub mod wordprocessing_printer_settings_part;
pub mod workbook_part;
pub mod workbook_person_part;
pub mod workbook_revision_header_part;
pub mod workbook_revision_log_part;
pub mod workbook_styles_part;
pub mod workbook_user_data_part;
pub mod worksheet_comments_part;
pub mod worksheet_part;
pub mod worksheet_sort_map_part;
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
    CellMetadataPart(crate ::parts::cell_metadata_part::CellMetadataPart),
    ChartColorStylePart(crate ::parts::chart_color_style_part::ChartColorStylePart),
    ChartDrawingPart(crate ::parts::chart_drawing_part::ChartDrawingPart),
    ChartPart(crate ::parts::chart_part::ChartPart), ChartStylePart(crate
    ::parts::chart_style_part::ChartStylePart), ChartsheetPart(crate
    ::parts::chartsheet_part::ChartsheetPart), CommentAuthorsPart(crate
    ::parts::comment_authors_part::CommentAuthorsPart), ConnectionsPart(crate
    ::parts::connections_part::ConnectionsPart), ControlPropertiesPart(crate
    ::parts::control_properties_part::ControlPropertiesPart),
    CoreFilePropertiesPart(crate
    ::parts::core_file_properties_part::CoreFilePropertiesPart), CustomDataPart(crate
    ::parts::custom_data_part::CustomDataPart), CustomDataPropertiesPart(crate
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
    ::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart),
    DiagramPersistLayoutPart(crate
    ::parts::diagram_persist_layout_part::DiagramPersistLayoutPart),
    DiagramStylePart(crate ::parts::diagram_style_part::DiagramStylePart),
    DialogsheetPart(crate ::parts::dialogsheet_part::DialogsheetPart),
    DigitalSignatureOriginPart(crate
    ::parts::digital_signature_origin_part::DigitalSignatureOriginPart),
    DocumentSettingsPart(crate ::parts::document_settings_part::DocumentSettingsPart),
    DocumentTasksPart(crate ::parts::document_tasks_part::DocumentTasksPart),
    DrawingsPart(crate ::parts::drawings_part::DrawingsPart),
    EmbeddedControlPersistenceBinaryDataPart(crate
    ::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart),
    EmbeddedControlPersistencePart(crate
    ::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart),
    EmbeddedObjectPart(crate ::parts::embedded_object_part::EmbeddedObjectPart),
    EmbeddedPackagePart(crate ::parts::embedded_package_part::EmbeddedPackagePart),
    EndnotesPart(crate ::parts::endnotes_part::EndnotesPart),
    ExcelAttachedToolbarsPart(crate
    ::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart),
    ExtendedChartPart(crate ::parts::extended_chart_part::ExtendedChartPart),
    ExtendedFilePropertiesPart(crate
    ::parts::extended_file_properties_part::ExtendedFilePropertiesPart),
    ExternalWorkbookPart(crate ::parts::external_workbook_part::ExternalWorkbookPart),
    FeaturePropertyBagsPart(crate
    ::parts::feature_property_bags_part::FeaturePropertyBagsPart), FontPart(crate
    ::parts::font_part::FontPart), FontTablePart(crate
    ::parts::font_table_part::FontTablePart), FooterPart(crate
    ::parts::footer_part::FooterPart), FootnotesPart(crate
    ::parts::footnotes_part::FootnotesPart), GlossaryDocumentPart(crate
    ::parts::glossary_document_part::GlossaryDocumentPart), HandoutMasterPart(crate
    ::parts::handout_master_part::HandoutMasterPart), HeaderPart(crate
    ::parts::header_part::HeaderPart), ImagePart(crate ::parts::image_part::ImagePart),
    InternationalMacroSheetPart(crate
    ::parts::international_macro_sheet_part::InternationalMacroSheetPart),
    LabelInfoPart(crate ::parts::label_info_part::LabelInfoPart),
    LegacyDiagramTextInfoPart(crate
    ::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart),
    LegacyDiagramTextPart(crate
    ::parts::legacy_diagram_text_part::LegacyDiagramTextPart), MacroSheetPart(crate
    ::parts::macro_sheet_part::MacroSheetPart), MailMergeRecipientDataPart(crate
    ::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart),
    MainDocumentPart(crate ::parts::main_document_part::MainDocumentPart),
    Model3DReferenceRelationshipPart(crate
    ::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart),
    NamedSheetViewsPart(crate ::parts::named_sheet_views_part::NamedSheetViewsPart),
    NotesMasterPart(crate ::parts::notes_master_part::NotesMasterPart),
    NotesSlidePart(crate ::parts::notes_slide_part::NotesSlidePart),
    NumberingDefinitionsPart(crate
    ::parts::numbering_definitions_part::NumberingDefinitionsPart),
    PivotTableCacheDefinitionPart(crate
    ::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart),
    PivotTableCacheRecordsPart(crate
    ::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart),
    PivotTablePart(crate ::parts::pivot_table_part::PivotTablePart),
    PowerPointAuthorsPart(crate
    ::parts::power_point_authors_part::PowerPointAuthorsPart),
    PowerPointCommentPart(crate
    ::parts::power_point_comment_part::PowerPointCommentPart), PresentationPart(crate
    ::parts::presentation_part::PresentationPart), PresentationPropertiesPart(crate
    ::parts::presentation_properties_part::PresentationPropertiesPart),
    QueryTablePart(crate ::parts::query_table_part::QueryTablePart),
    QuickAccessToolbarCustomizationsPart(crate
    ::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart),
    RdArrayPart(crate ::parts::rd_array_part::RdArrayPart), RdRichValuePart(crate
    ::parts::rd_rich_value_part::RdRichValuePart), RdRichValueStructurePart(crate
    ::parts::rd_rich_value_structure_part::RdRichValueStructurePart),
    RdRichValueTypesPart(crate ::parts::rd_rich_value_types_part::RdRichValueTypesPart),
    RdRichValueWebImagePart(crate
    ::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart),
    RdSupportingPropertyBagPart(crate
    ::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart),
    RdSupportingPropertyBagStructurePart(crate
    ::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart),
    RibbonAndBackstageCustomizationsPart(crate
    ::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart),
    RibbonExtensibilityPart(crate
    ::parts::ribbon_extensibility_part::RibbonExtensibilityPart), RichStylesPart(crate
    ::parts::rich_styles_part::RichStylesPart), SharedStringTablePart(crate
    ::parts::shared_string_table_part::SharedStringTablePart), SingleCellTablePart(crate
    ::parts::single_cell_table_part::SingleCellTablePart), SlicerCachePart(crate
    ::parts::slicer_cache_part::SlicerCachePart), SlicersPart(crate
    ::parts::slicers_part::SlicersPart), SlideCommentsPart(crate
    ::parts::slide_comments_part::SlideCommentsPart), SlideLayoutPart(crate
    ::parts::slide_layout_part::SlideLayoutPart), SlideMasterPart(crate
    ::parts::slide_master_part::SlideMasterPart), SlidePart(crate
    ::parts::slide_part::SlidePart), SlideSyncDataPart(crate
    ::parts::slide_sync_data_part::SlideSyncDataPart),
    SpreadsheetPrinterSettingsPart(crate
    ::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart),
    StyleDefinitionsPart(crate ::parts::style_definitions_part::StyleDefinitionsPart),
    StylesWithEffectsPart(crate
    ::parts::styles_with_effects_part::StylesWithEffectsPart), TableDefinitionPart(crate
    ::parts::table_definition_part::TableDefinitionPart), TableStylesPart(crate
    ::parts::table_styles_part::TableStylesPart), ThemeOverridePart(crate
    ::parts::theme_override_part::ThemeOverridePart), ThemePart(crate
    ::parts::theme_part::ThemePart), ThumbnailPart(crate
    ::parts::thumbnail_part::ThumbnailPart), TimeLineCachePart(crate
    ::parts::time_line_cache_part::TimeLineCachePart), TimeLinePart(crate
    ::parts::time_line_part::TimeLinePart), UserDefinedTagsPart(crate
    ::parts::user_defined_tags_part::UserDefinedTagsPart), VbaDataPart(crate
    ::parts::vba_data_part::VbaDataPart), VbaProjectPart(crate
    ::parts::vba_project_part::VbaProjectPart), ViewPropertiesPart(crate
    ::parts::view_properties_part::ViewPropertiesPart), VmlDrawingPart(crate
    ::parts::vml_drawing_part::VmlDrawingPart), VolatileDependenciesPart(crate
    ::parts::volatile_dependencies_part::VolatileDependenciesPart),
    WebExTaskpanesPart(crate ::parts::web_ex_taskpanes_part::WebExTaskpanesPart),
    WebExtensionPart(crate ::parts::web_extension_part::WebExtensionPart),
    WebSettingsPart(crate ::parts::web_settings_part::WebSettingsPart),
    WordAttachedToolbarsPart(crate
    ::parts::word_attached_toolbars_part::WordAttachedToolbarsPart),
    WordCommentsExtensiblePart(crate
    ::parts::word_comments_extensible_part::WordCommentsExtensiblePart),
    WordprocessingCommentsExPart(crate
    ::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart),
    WordprocessingCommentsIdsPart(crate
    ::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart),
    WordprocessingCommentsPart(crate
    ::parts::wordprocessing_comments_part::WordprocessingCommentsPart),
    WordprocessingPeoplePart(crate
    ::parts::wordprocessing_people_part::WordprocessingPeoplePart),
    WordprocessingPrinterSettingsPart(crate
    ::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart),
    WorkbookPart(crate ::parts::workbook_part::WorkbookPart), WorkbookPersonPart(crate
    ::parts::workbook_person_part::WorkbookPersonPart), WorkbookRevisionHeaderPart(crate
    ::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart),
    WorkbookRevisionLogPart(crate
    ::parts::workbook_revision_log_part::WorkbookRevisionLogPart),
    WorkbookStylesPart(crate ::parts::workbook_styles_part::WorkbookStylesPart),
    WorkbookUserDataPart(crate ::parts::workbook_user_data_part::WorkbookUserDataPart),
    WorksheetCommentsPart(crate ::parts::worksheet_comments_part::WorksheetCommentsPart),
    WorksheetPart(crate ::parts::worksheet_part::WorksheetPart),
    WorksheetSortMapPart(crate ::parts::worksheet_sort_map_part::WorksheetSortMapPart),
    WorksheetThreadedCommentsPart(crate
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
          return Some(
                        <crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars" => {
        if part.content_type() == "application/vnd.ms-excel.attachedToolbars" {
          return Some(
            <crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if part.content_type() == "application/vnd.ms-word.attachedToolbars" {
          return Some(
            <crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/keyMapCustomizations" => {
        if part.content_type() == "application/vnd.ms-word.keyMapCustomizations+xml" {
          return Some(
            <crate::parts::customization_part::CustomizationPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/legacyDiagramText" => {
        if part.content_type() == "application/vnd.ms-office.legacyDiagramText" {
          return Some(
            <crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/legacyDocTextInfo" => {
        if part.content_type() == "application/vnd.ms-office.legacyDocTextInfo" {
          return Some(
            <crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/ui/extensibility" => {
        if part.content_type() == "application/xml" {
          return Some(
            <crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/ui/userCustomization" => {
        if part.content_type() == "application/xml" {
          return Some(
                        <crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/vbaProject" => {
        if part.content_type() == "application/vnd.ms-office.vbaProject" {
          return Some(
            <crate::parts::vba_project_part::VbaProjectPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/wordVbaData" => {
        if part.content_type() == "application/vnd.ms-word.vbaData+xml" {
          return Some(<crate::parts::vba_data_part::VbaDataPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/wsSortMap" => {
        if part.content_type() == "application/vnd.ms-excel.wsSortMap+xml" {
          return Some(
            <crate::parts::worksheet_sort_map_part::WorksheetSortMapPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/xlIntlMacrosheet" => {
        if part.content_type() == "application/vnd.ms-excel.intlmacrosheet+xml" {
          return Some(
                        <crate::parts::international_macro_sheet_part::InternationalMacroSheetPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet" => {
        if part.content_type() == "application/vnd.ms-excel.macrosheet+xml" {
          return Some(
            <crate::parts::macro_sheet_part::MacroSheetPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/customData" => {
        if part.content_type() == "application/binary" {
          return Some(
            <crate::parts::custom_data_part::CustomDataPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/customDataProps" => {
        if part.content_type() == "application/vnd.ms-excel.customDataProperties+xml" {
          return Some(
            <crate::parts::custom_data_properties_part::CustomDataPropertiesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing" => {
        if part.content_type() == "application/vnd.ms-office.drawingml.diagramDrawing+xml" {
          return Some(
            <crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/slicer" => {
        if part.content_type() == "application/vnd.ms-excel.slicer+xml" {
          return Some(<crate::parts::slicers_part::SlicersPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/slicerCache" => {
        if part.content_type() == "application/vnd.ms-excel.slicerCache+xml" {
          return Some(
            <crate::parts::slicer_cache_part::SlicerCachePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects" => {
        if part.content_type() == "application/vnd.ms-word.stylesWithEffects+xml" {
          return Some(
            <crate::parts::styles_with_effects_part::StylesWithEffectsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility" => {
        if part.content_type() == "application/xml" {
          return Some(
                        <crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle" => {
        if part.content_type() == "application/vnd.ms-office.chartcolorstyle+xml" {
          return Some(
            <crate::parts::chart_color_style_part::ChartColorStylePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/chartStyle" => {
        if part.content_type() == "application/vnd.ms-office.chartstyle+xml" {
          return Some(
            <crate::parts::chart_style_part::ChartStylePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/commentsExtended" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtended+xml"
        {
          return Some(
                        <crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/people" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.people+xml"
        {
          return Some(
            <crate::parts::wordprocessing_people_part::WordprocessingPeoplePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/timeline" => {
        if part.content_type() == "application/vnd.ms-excel.timeline+xml" {
          return Some(<crate::parts::time_line_part::TimeLinePart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/timelineCache" => {
        if part.content_type() == "application/vnd.ms-excel.timelineCache+xml" {
          return Some(
            <crate::parts::time_line_cache_part::TimeLineCachePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/webextension" => {
        if part.content_type() == "application/vnd.ms-office.webextension+xml" {
          return Some(
            <crate::parts::web_extension_part::WebExtensionPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes" => {
        if part.content_type() == "application/vnd.ms-office.webextensiontaskpanes+xml" {
          return Some(
            <crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2014/relationships/chartEx" => {
        if part.content_type() == "application/vnd.ms-office.chartex+xml" {
          return Some(
            <crate::parts::extended_chart_part::ExtendedChartPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2016/09/relationships/commentsIds" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsIds+xml"
        {
          return Some(
                        <crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/model3d" => {
        if part.content_type() == "model/gltf-binary" {
          return Some(
                        <crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdArray" => {
        if part.content_type() == "application/vnd.ms-excel.rdarray+xml" {
          return Some(<crate::parts::rd_array_part::RdArrayPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue" => {
        if part.content_type() == "application/vnd.ms-excel.rdrichvalue+xml" {
          return Some(
            <crate::parts::rd_rich_value_part::RdRichValuePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure" => {
        if part.content_type() == "application/vnd.ms-excel.rdrichvaluestructure+xml" {
          return Some(
            <crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes" => {
        if part.content_type() == "application/vnd.ms-excel.rdrichvaluetypes+xml" {
          return Some(
            <crate::parts::rd_rich_value_types_part::RdRichValueTypesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag" => {
        if part.content_type() == "application/vnd.ms-excel.rdsupportingpropertybag+xml" {
          return Some(
                        <crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure" => {
        if part.content_type() == "application/vnd.ms-excel.rdsupportingpropertybagstructure+xml" {
          return Some(
                        <crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/richStyles" => {
        if part.content_type() == "application/vnd.ms-excel.richstyles+xml" {
          return Some(
            <crate::parts::rich_styles_part::RichStylesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2017/10/relationships/person" => {
        if part.content_type() == "application/vnd.ms-excel.person+xml" {
          return Some(
            <crate::parts::workbook_person_part::WorkbookPersonPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2017/10/relationships/threadedComment" => {
        if part.content_type() == "application/vnd.ms-excel.threadedcomments+xml" {
          return Some(
                        <crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2018/08/relationships/commentsExtensible" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtensible+xml"
        {
          return Some(
                        <crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2018/10/relationships/authors" => {
        if part.content_type() == "application/vnd.ms-powerpoint.authors+xml" {
          return Some(
            <crate::parts::power_point_authors_part::PowerPointAuthorsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2018/10/relationships/comments" => {
        if part.content_type() == "application/vnd.ms-powerpoint.comments+xml" {
          return Some(
            <crate::parts::power_point_comment_part::PowerPointCommentPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2019/04/relationships/namedSheetView" => {
        if part.content_type() == "application/vnd.ms-excel.namedsheetviews+xml" {
          return Some(
            <crate::parts::named_sheet_views_part::NamedSheetViewsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2019/05/relationships/documenttasks" => {
        if part.content_type() == "application/vnd.ms-office.documenttasks+xml" {
          return Some(
            <crate::parts::document_tasks_part::DocumentTasksPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels" => {
        if part.content_type() == "application/vnd.ms-office.classificationlabels+xml" {
          return Some(
            <crate::parts::label_info_part::LabelInfoPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2020/07/relationships/rdRichValueWebImage" => {
        if part.content_type() == "application/vnd.ms-excel.rdrichvaluewebimage+xml" {
          return Some(
            <crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag" => {
        if part.content_type() == "application/vnd.ms-excel.featurepropertybag+xml" {
          return Some(
            <crate::parts::feature_property_bags_part::FeaturePropertyBagsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk" => {
        if true {
          return Some(
                        <crate::parts::alternative_format_import_part::AlternativeFormatImportPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml"
        {
          return Some(
            <crate::parts::calculation_chain_part::CalculationChainPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.chart+xml"
        {
          return Some(<crate::parts::chart_part::ChartPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartUserShapes" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml"
        {
          return Some(
            <crate::parts::chart_drawing_part::ChartDrawingPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml"
        {
          return Some(
            <crate::parts::chartsheet_part::ChartsheetPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/commentAuthors" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml"
        {
          return Some(
            <crate::parts::comment_authors_part::CommentAuthorsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.comments+xml"
        {
          return Some(
            <crate::parts::slide_comments_part::SlideCommentsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"
        {
          return Some(
            <crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml"
        {
          return Some(
            <crate::parts::worksheet_comments_part::WorksheetCommentsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/connections" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml"
        {
          return Some(
            <crate::parts::connections_part::ConnectionsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control" => {
        if true {
          return Some(
                        <crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/ctrlProp" => {
        if part.content_type() == "application/vnd.ms-excel.controlproperties+xml" {
          return Some(
            <crate::parts::control_properties_part::ControlPropertiesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.custom-properties+xml"
        {
          return Some(
            <crate::parts::custom_file_properties_part::CustomFilePropertiesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty" => {
        if true {
          return Some(
            <crate::parts::custom_property_part::CustomPropertyPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml" => {
        if true {
          return Some(
            <crate::parts::custom_xml_part::CustomXmlPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.customXmlProperties+xml"
        {
          return Some(
            <crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml"
        {
          return Some(
            <crate::parts::diagram_colors_part::DiagramColorsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml"
        {
          return Some(
            <crate::parts::diagram_data_part::DiagramDataPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml"
        {
          return Some(
                        <crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml"
        {
          return Some(
            <crate::parts::diagram_style_part::DiagramStylePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml"
        {
          return Some(
            <crate::parts::dialogsheet_part::DialogsheetPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing" => {
        if part.content_type() == "application/vnd.openxmlformats-officedocument.drawing+xml" {
          return Some(<crate::parts::drawings_part::DrawingsPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml"
        {
          return Some(<crate::parts::endnotes_part::EndnotesPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.extended-properties+xml"
        {
          return Some(
                        <crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml"
        {
          return Some(
            <crate::parts::external_workbook_part::ExternalWorkbookPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font" => {
        if true {
          return Some(<crate::parts::font_part::FontPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"
        {
          return Some(
            <crate::parts::font_table_part::FontTablePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"
        {
          return Some(<crate::parts::footer_part::FooterPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml"
        {
          return Some(
            <crate::parts::footnotes_part::FootnotesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/glossaryDocument" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml"
        {
          return Some(
            <crate::parts::glossary_document_part::GlossaryDocumentPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml"
        {
          return Some(
            <crate::parts::handout_master_part::HandoutMasterPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml"
        {
          return Some(<crate::parts::header_part::HeaderPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" => {
        if true {
          return Some(<crate::parts::image_part::ImagePart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml"
        {
          return Some(
            <crate::parts::notes_master_part::NotesMasterPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"
        {
          return Some(
            <crate::parts::notes_slide_part::NotesSlidePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"
        {
          return Some(
            <crate::parts::numbering_definitions_part::NumberingDefinitionsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" => {
        if part.path() == "word/document.xml" {
          return Some(
            <crate::parts::main_document_part::MainDocumentPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if part.path() == "ppt/presentation.xml" {
          return Some(
            <crate::parts::presentation_part::PresentationPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if part.path() == "xl/workbook.xml" {
          return Some(<crate::parts::workbook_part::WorkbookPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject" => {
        if true {
          return Some(
            <crate::parts::embedded_object_part::EmbeddedObjectPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package" => {
        if true {
          return Some(
            <crate::parts::embedded_package_part::EmbeddedPackagePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml"
        {
          return Some(
                        <crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheRecords" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml"
        {
          return Some(
                        <crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml"
        {
          return Some(
            <crate::parts::pivot_table_part::PivotTablePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml"
        {
          return Some(
            <crate::parts::presentation_properties_part::PresentationPropertiesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.printerSettings"
        {
          return Some(
                        <crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.printerSettings"
        {
          return Some(
                        <crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/queryTable" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml"
        {
          return Some(
            <crate::parts::query_table_part::QueryTablePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/recipientData" => {
        if true {
          return Some(
                        <crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml"
        {
          return Some(
                        <crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionLog" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml"
        {
          return Some(
            <crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"
        {
          return Some(
            <crate::parts::document_settings_part::DocumentSettingsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"
        {
          return Some(
            <crate::parts::shared_string_table_part::SharedStringTablePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sheetMetadata" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml"
        {
          return Some(
            <crate::parts::cell_metadata_part::CellMetadataPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.slide+xml"
        {
          return Some(<crate::parts::slide_part::SlidePart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"
        {
          return Some(
            <crate::parts::slide_layout_part::SlideLayoutPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"
        {
          return Some(
            <crate::parts::slide_master_part::SlideMasterPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideUpdateInfo" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml"
        {
          return Some(
            <crate::parts::slide_sync_data_part::SlideSyncDataPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"
        {
          return Some(
            <crate::parts::style_definitions_part::StyleDefinitionsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"
        {
          return Some(
            <crate::parts::workbook_styles_part::WorkbookStylesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml"
        {
          return Some(
            <crate::parts::table_definition_part::TableDefinitionPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableSingleCells" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml"
        {
          return Some(
            <crate::parts::single_cell_table_part::SingleCellTablePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml"
        {
          return Some(
            <crate::parts::table_styles_part::TableStylesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.tags+xml"
        {
          return Some(
            <crate::parts::user_defined_tags_part::UserDefinedTagsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" => {
        if part.content_type() == "application/vnd.openxmlformats-officedocument.theme+xml" {
          return Some(<crate::parts::theme_part::ThemePart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride" => {
        if part.content_type() == "application/vnd.openxmlformats-officedocument.themeOverride+xml"
        {
          return Some(
            <crate::parts::theme_override_part::ThemeOverridePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/usernames" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml"
        {
          return Some(
            <crate::parts::workbook_user_data_part::WorkbookUserDataPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml"
        {
          return Some(
            <crate::parts::view_properties_part::ViewPropertiesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing" => {
        if part.content_type() == "application/vnd.openxmlformats-officedocument.vmlDrawing" {
          return Some(
            <crate::parts::vml_drawing_part::VmlDrawingPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/volatileDependencies" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml"
        {
          return Some(
            <crate::parts::volatile_dependencies_part::VolatileDependenciesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml"
        {
          return Some(
            <crate::parts::web_settings_part::WebSettingsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"
        {
          return Some(
            <crate::parts::worksheet_part::WorksheetPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps" => {
        if part.content_type() == "application/xml" {
          return Some(
            <crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin" => {
        if part.content_type() == "application/vnd.openxmlformats-package.digital-signature-origin"
        {
          return Some(
                        <crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/signature" => {
        if part.content_type()
          == "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml"
        {
          return Some(
            <crate::parts::xml_signature_part::XmlSignaturePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" => {
        if part.content_type() == "application/vnd.openxmlformats-package.core-properties+xml" {
          return Some(
            <crate::parts::core_file_properties_part::CoreFilePropertiesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
      }
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail" => {
        if true {
          return Some(
            <crate::parts::thumbnail_part::ThumbnailPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
                        <crate::parts::alternative_format_import_part::AlternativeFormatImportPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
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
          return Some(
            <crate::parts::calculation_chain_part::CalculationChainPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::cell_metadata_part::CellMetadataPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle",
          "application/vnd.ms-office.chartcolorstyle+xml",
          ".",
          "colors",
        ) {
          return Some(
            <crate::parts::chart_color_style_part::ChartColorStylePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::chart_drawing_part::ChartDrawingPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(<crate::parts::chart_part::ChartPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2011/relationships/chartStyle",
          "application/vnd.ms-office.chartstyle+xml",
          ".",
          "style",
        ) {
          return Some(
            <crate::parts::chart_style_part::ChartStylePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::chartsheet_part::ChartsheetPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::comment_authors_part::CommentAuthorsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::connections_part::ConnectionsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.openxmlformats.org/officeDocument/2006/relationships/ctrlProp",
          "application/vnd.ms-excel.controlproperties+xml",
          "../ctrlProps",
          "ctrlProp",
        ) {
          return Some(
            <crate::parts::control_properties_part::ControlPropertiesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::core_file_properties_part::CoreFilePropertiesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2007/relationships/customData",
          "application/binary",
          "customData",
          "customData",
        ) {
          return Some(
            <crate::parts::custom_data_part::CustomDataPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2007/relationships/customDataProps",
          "application/vnd.ms-excel.customDataProperties+xml",
          "customData",
          "customDataProps",
        ) {
          return Some(
            <crate::parts::custom_data_properties_part::CustomDataPropertiesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::custom_file_properties_part::CustomFilePropertiesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::custom_property_part::CustomPropertyPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::custom_xml_part::CustomXmlPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::customization_part::CustomizationPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::diagram_colors_part::DiagramColorsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::diagram_data_part::DiagramDataPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
                        <crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing",
          "application/vnd.ms-office.drawingml.diagramDrawing+xml",
          "../diagrams",
          "drawing",
        ) {
          return Some(
            <crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::diagram_style_part::DiagramStylePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::dialogsheet_part::DialogsheetPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
                        <crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
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
          return Some(
            <crate::parts::document_settings_part::DocumentSettingsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2019/05/relationships/documenttasks",
          "application/vnd.ms-office.documenttasks+xml",
          ".",
          "tasks",
        ) {
          return Some(
            <crate::parts::document_tasks_part::DocumentTasksPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(<crate::parts::drawings_part::DrawingsPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
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
          return Some(
                        <crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
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
          return Some(
                        <crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
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
          return Some(
            <crate::parts::embedded_object_part::EmbeddedObjectPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::embedded_package_part::EmbeddedPackagePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(<crate::parts::endnotes_part::EndnotesPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
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
          return Some(
            <crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2014/relationships/chartEx",
          "application/vnd.ms-office.chartex+xml",
          "extendedCharts",
          "chart",
        ) {
          return Some(
            <crate::parts::extended_chart_part::ExtendedChartPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
                        <crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
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
          return Some(
            <crate::parts::external_workbook_part::ExternalWorkbookPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag",
          "application/vnd.ms-excel.featurepropertybag+xml",
          "featurePropertyBag",
          "featurePropertyBag",
        ) {
          return Some(
            <crate::parts::feature_property_bags_part::FeaturePropertyBagsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(<crate::parts::font_part::FontPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
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
          return Some(
            <crate::parts::font_table_part::FontTablePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(<crate::parts::footer_part::FooterPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
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
          return Some(
            <crate::parts::footnotes_part::FootnotesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::glossary_document_part::GlossaryDocumentPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::handout_master_part::HandoutMasterPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(<crate::parts::header_part::HeaderPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
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
          return Some(<crate::parts::image_part::ImagePart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
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
          return Some(
                        <crate::parts::international_macro_sheet_part::InternationalMacroSheetPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels",
          "application/vnd.ms-office.classificationlabels+xml",
          "docMetadata",
          "LabelInfo",
        ) {
          return Some(
            <crate::parts::label_info_part::LabelInfoPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::macro_sheet_part::MacroSheetPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
                        <crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
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
          return Some(
            <crate::parts::main_document_part::MainDocumentPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/06/relationships/model3d",
          "model/gltf-binary",
          "../media",
          "model3d",
        ) {
          return Some(
                        <crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2019/04/relationships/namedSheetView",
          "application/vnd.ms-excel.namedsheetviews+xml",
          "../namedSheetViews",
          "namedSheetView",
        ) {
          return Some(
            <crate::parts::named_sheet_views_part::NamedSheetViewsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::notes_master_part::NotesMasterPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::notes_slide_part::NotesSlidePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::numbering_definitions_part::NumberingDefinitionsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
                        <crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
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
          return Some(
                        <crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
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
          return Some(
            <crate::parts::pivot_table_part::PivotTablePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2018/10/relationships/authors",
          "application/vnd.ms-powerpoint.authors+xml",
          ".",
          "authors",
        ) {
          return Some(
            <crate::parts::power_point_authors_part::PowerPointAuthorsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2018/10/relationships/comments",
          "application/vnd.ms-powerpoint.comments+xml",
          "../comments",
          "modernComment",
        ) {
          return Some(
            <crate::parts::power_point_comment_part::PowerPointCommentPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::presentation_part::PresentationPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::presentation_properties_part::PresentationPropertiesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::query_table_part::QueryTablePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
                        <crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/06/relationships/rdArray",
          "application/vnd.ms-excel.rdarray+xml",
          "richData",
          "rdarray",
        ) {
          return Some(<crate::parts::rd_array_part::RdArrayPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue",
          "application/vnd.ms-excel.rdrichvalue+xml",
          "richData",
          "rdrichvalue",
        ) {
          return Some(
            <crate::parts::rd_rich_value_part::RdRichValuePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure",
          "application/vnd.ms-excel.rdrichvaluestructure+xml",
          "richData",
          "rdrichvaluestructure",
        ) {
          return Some(
            <crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes",
          "application/vnd.ms-excel.rdrichvaluetypes+xml",
          "richData",
          "rdRichValueTypes",
        ) {
          return Some(
            <crate::parts::rd_rich_value_types_part::RdRichValueTypesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2020/07/relationships/rdRichValueWebImage",
          "application/vnd.ms-excel.rdrichvaluewebimage+xml",
          "richData",
          "rdRichValueWebImage",
        ) {
          return Some(
            <crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag",
          "application/vnd.ms-excel.rdsupportingpropertybag+xml",
          "richData",
          "rdsupportingpropertybag",
        ) {
          return Some(
                        <crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure",
          "application/vnd.ms-excel.rdsupportingpropertybagstructure+xml",
          "richData",
          "rdsupportingpropertybagstructure",
        ) {
          return Some(
                        <crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility",
          "application/xml",
          "customUI",
          "customUI",
        ) {
          return Some(
                        <crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
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
          return Some(
            <crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/06/relationships/richStyles",
          "application/vnd.ms-excel.richstyles+xml",
          "richData",
          "richStyles",
        ) {
          return Some(
            <crate::parts::rich_styles_part::RichStylesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::shared_string_table_part::SharedStringTablePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::single_cell_table_part::SingleCellTablePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2007/relationships/slicerCache",
          "application/vnd.ms-excel.slicerCache+xml",
          "slicerCaches",
          "slicerCache",
        ) {
          return Some(
            <crate::parts::slicer_cache_part::SlicerCachePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2007/relationships/slicer",
          "application/vnd.ms-excel.slicer+xml",
          "../slicers",
          "slicer",
        ) {
          return Some(<crate::parts::slicers_part::SlicersPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
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
          return Some(
            <crate::parts::slide_comments_part::SlideCommentsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::slide_layout_part::SlideLayoutPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::slide_master_part::SlideMasterPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(<crate::parts::slide_part::SlidePart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
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
          return Some(
            <crate::parts::slide_sync_data_part::SlideSyncDataPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
                        <crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
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
          return Some(
            <crate::parts::style_definitions_part::StyleDefinitionsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects",
          "application/vnd.ms-word.stylesWithEffects+xml",
          ".",
          "stylesWithEffects",
        ) {
          return Some(
            <crate::parts::styles_with_effects_part::StylesWithEffectsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::table_definition_part::TableDefinitionPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::table_styles_part::TableStylesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::theme_override_part::ThemeOverridePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(<crate::parts::theme_part::ThemePart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
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
          return Some(
            <crate::parts::thumbnail_part::ThumbnailPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2011/relationships/timelineCache",
          "application/vnd.ms-excel.timelineCache+xml",
          "timelineCaches",
          "timelineCache",
        ) {
          return Some(
            <crate::parts::time_line_cache_part::TimeLineCachePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2011/relationships/timeline",
          "application/vnd.ms-excel.timeline+xml",
          "../timelines",
          "timeline",
        ) {
          return Some(<crate::parts::time_line_part::TimeLinePart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
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
          return Some(
            <crate::parts::user_defined_tags_part::UserDefinedTagsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(<crate::parts::vba_data_part::VbaDataPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
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
          return Some(
            <crate::parts::vba_project_part::VbaProjectPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::view_properties_part::ViewPropertiesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::vml_drawing_part::VmlDrawingPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::volatile_dependencies_part::VolatileDependenciesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes",
          "application/vnd.ms-office.webextensiontaskpanes+xml",
          "../webextensions",
          "taskpanes",
        ) {
          return Some(
            <crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2011/relationships/webextension",
          "application/vnd.ms-office.webextension+xml",
          "../webextensions",
          "webextension",
        ) {
          return Some(
            <crate::parts::web_extension_part::WebExtensionPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::web_settings_part::WebSettingsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2018/08/relationships/commentsExtensible",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtensible+xml",
          ".",
          "commentsExtensible",
        ) {
          return Some(
                        <crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2011/relationships/commentsExtended",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtended+xml",
          ".",
          "commentsExtended",
        ) {
          return Some(
                        <crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2016/09/relationships/commentsIds",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsIds+xml",
          ".",
          "commentsIds",
        ) {
          return Some(
                        <crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
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
          return Some(
            <crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2011/relationships/people",
          "application/vnd.openxmlformats-officedocument.wordprocessingml.people+xml",
          ".",
          "people",
        ) {
          return Some(
            <crate::parts::wordprocessing_people_part::WordprocessingPeoplePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
                        <crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
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
          return Some(<crate::parts::workbook_part::WorkbookPart>::make_part_ref(
            storage,
            part_id,
            relationship_id,
          ));
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/10/relationships/person",
          "application/vnd.ms-excel.person+xml",
          "persons",
          "person",
        ) {
          return Some(
            <crate::parts::workbook_person_part::WorkbookPersonPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
                        <crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
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
          return Some(
            <crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::workbook_styles_part::WorkbookStylesPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::workbook_user_data_part::WorkbookUserDataPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::worksheet_comments_part::WorksheetCommentsPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::worksheet_part::WorksheetPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
          return Some(
            <crate::parts::worksheet_sort_map_part::WorksheetSortMapPart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
        }
        if crate::common::part_descriptor_matches(
          relationship_type,
          part.content_type(),
          part.path(),
          "http://schemas.microsoft.com/office/2017/10/relationships/threadedComment",
          "application/vnd.ms-excel.threadedcomments+xml",
          "../threadedcomments",
          "threadedcomment",
        ) {
          return Some(
                        <crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart>::make_part_ref(
                            storage,
                            part_id,
                            relationship_id,
                        ),
                    );
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
          return Some(
            <crate::parts::xml_signature_part::XmlSignaturePart>::make_part_ref(
              storage,
              part_id,
              relationship_id,
            ),
          );
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
        ::common::SdkPackageStorage, part_id : crate ::common::PartId, open_settings : &
        crate ::sdk::OpenSettings,) -> Result < Option < Self >, crate ::common::SdkError
        > { let Some(part) = storage.part(part_id) else { return Ok(None); };
        #[cfg(not(feature = "mce"))] let _ = open_settings; $($(#[$attrs])* if !
        matches!($content_type, "" | "application/xml" | "text/xml") && part
        .content_type() == $content_type { #[cfg(feature = "mce")] let mut root = <
        $root_ty > ::from_bytes(part.data().bytes()) ?; #[cfg(feature = "mce")] crate
        ::sdk::SdkMce::process_mce(& mut root, & open_settings
        .markup_compatibility_process_settings,) ?; #[cfg(not(feature = "mce"))] let root
        = < $root_ty > ::from_bytes(part.data().bytes()) ?; return Ok(Some(Self::
        $variant (Box::new(root)))); })* Ok(None) } }
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
    ChartColorStylePart(crate
    ::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ColorStyle,
    as_chart_color_style_part, as_chart_color_style_part_mut,
    "application/vnd.ms-office.chartcolorstyle+xml"), ChartDrawingPart(crate
    ::schemas::schemas_openxmlformats_org_drawingml_2006_chart::UserShapes,
    as_chart_drawing_part, as_chart_drawing_part_mut,
    "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml"),
    ChartPart(crate
    ::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartSpace,
    as_chart_part, as_chart_part_mut,
    "application/vnd.openxmlformats-officedocument.drawingml.chart+xml"),
    ChartStylePart(crate
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
    ControlPropertiesPart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::FormControlProperties,
    as_control_properties_part, as_control_properties_part_mut,
    "application/vnd.ms-excel.controlproperties+xml"), CoreFilePropertiesPart(crate
    ::schemas::opc_core_properties::CoreProperties, as_core_file_properties_part,
    as_core_file_properties_part_mut,
    "application/vnd.openxmlformats-package.core-properties+xml"),
    CustomDataPropertiesPart(crate
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
    DiagramPersistLayoutPart(crate
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
    DocumentTasksPart(crate
    ::schemas::schemas_microsoft_com_office_tasks_2019_documenttasks::Tasks,
    as_document_tasks_part, as_document_tasks_part_mut,
    "application/vnd.ms-office.documenttasks+xml"), DrawingsPart(crate
    ::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::WorksheetDrawing,
    as_drawings_part, as_drawings_part_mut,
    "application/vnd.openxmlformats-officedocument.drawing+xml"), EndnotesPart(crate
    ::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Endnotes,
    as_endnotes_part, as_endnotes_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml"),
    ExtendedChartPart(crate
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
    FeaturePropertyBagsPart(crate
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
    LabelInfoPart(crate
    ::schemas::schemas_microsoft_com_office_2020_mip_label_metadata::ClassificationLabelList,
    as_label_info_part, as_label_info_part_mut,
    "application/vnd.ms-office.classificationlabels+xml"), MacroSheetPart(crate
    ::schemas::schemas_microsoft_com_office_excel_2006_main::Macrosheet,
    as_macro_sheet_part, as_macro_sheet_part_mut,
    "application/vnd.ms-excel.macrosheet+xml"), MainDocumentPart(crate
    ::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document,
    as_main_document_part, as_main_document_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"),
    NamedSheetViewsPart(crate
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
    PowerPointAuthorsPart(crate
    ::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::AuthorList,
    as_power_point_authors_part, as_power_point_authors_part_mut,
    "application/vnd.ms-powerpoint.authors+xml"), PowerPointCommentPart(crate
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
    RdArrayPart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::ArrayData,
    as_rd_array_part, as_rd_array_part_mut, "application/vnd.ms-excel.rdarray+xml"),
    RdRichValuePart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueData,
    as_rd_rich_value_part, as_rd_rich_value_part_mut,
    "application/vnd.ms-excel.rdrichvalue+xml"), RdRichValueStructurePart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueStructures,
    as_rd_rich_value_structure_part, as_rd_rich_value_structure_part_mut,
    "application/vnd.ms-excel.rdrichvaluestructure+xml"), RdRichValueTypesPart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::RichValueTypesInfo,
    as_rd_rich_value_types_part, as_rd_rich_value_types_part_mut,
    "application/vnd.ms-excel.rdrichvaluetypes+xml"), RdRichValueWebImagePart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2020_richdatawebimage::WebImagesSupportingRichData,
    as_rd_rich_value_web_image_part, as_rd_rich_value_web_image_part_mut,
    "application/vnd.ms-excel.rdrichvaluewebimage+xml"),
    RdSupportingPropertyBagPart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBags,
    as_rd_supporting_property_bag_part, as_rd_supporting_property_bag_part_mut,
    "application/vnd.ms-excel.rdsupportingpropertybag+xml"),
    RdSupportingPropertyBagStructurePart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBagStructures,
    as_rd_supporting_property_bag_structure_part,
    as_rd_supporting_property_bag_structure_part_mut,
    "application/vnd.ms-excel.rdsupportingpropertybagstructure+xml"),
    RibbonAndBackstageCustomizationsPart(crate
    ::schemas::schemas_microsoft_com_office_2009_07_customui::CustomUi,
    as_ribbon_and_backstage_customizations_part,
    as_ribbon_and_backstage_customizations_part_mut, "application/xml"),
    RichStylesPart(crate
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
    SlicerCachePart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerCacheDefinition,
    as_slicer_cache_part, as_slicer_cache_part_mut,
    "application/vnd.ms-excel.slicerCache+xml"), SlicersPart(crate
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
    StylesWithEffectsPart(crate
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
    TimeLineCachePart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineCacheDefinition,
    as_time_line_cache_part, as_time_line_cache_part_mut,
    "application/vnd.ms-excel.timelineCache+xml"), TimeLinePart(crate
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
    WebExTaskpanesPart(crate
    ::schemas::schemas_microsoft_com_office_webextensions_taskpanes_2010_11::Taskpanes,
    as_web_ex_taskpanes_part, as_web_ex_taskpanes_part_mut,
    "application/vnd.ms-office.webextensiontaskpanes+xml"), WebExtensionPart(crate
    ::schemas::schemas_microsoft_com_office_webextensions_webextension_2010_11::WebExtension,
    as_web_extension_part, as_web_extension_part_mut,
    "application/vnd.ms-office.webextension+xml"), WebSettingsPart(crate
    ::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::WebSettings,
    as_web_settings_part, as_web_settings_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml"),
    WordCommentsExtensiblePart(crate
    ::schemas::schemas_microsoft_com_office_word_2018_wordml_cex::CommentsExtensible,
    as_word_comments_extensible_part, as_word_comments_extensible_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtensible+xml"),
    WordprocessingCommentsExPart(crate
    ::schemas::schemas_microsoft_com_office_word_2012_wordml::CommentsEx,
    as_wordprocessing_comments_ex_part, as_wordprocessing_comments_ex_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtended+xml"),
    WordprocessingCommentsIdsPart(crate
    ::schemas::schemas_microsoft_com_office_word_2016_wordml_cid::CommentsIds,
    as_wordprocessing_comments_ids_part, as_wordprocessing_comments_ids_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsIds+xml"),
    WordprocessingCommentsPart(crate
    ::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Comments,
    as_wordprocessing_comments_part, as_wordprocessing_comments_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"),
    WordprocessingPeoplePart(crate
    ::schemas::schemas_microsoft_com_office_word_2012_wordml::People,
    as_wordprocessing_people_part, as_wordprocessing_people_part_mut,
    "application/vnd.openxmlformats-officedocument.wordprocessingml.people+xml"),
    WorkbookPart(crate
    ::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook,
    as_workbook_part, as_workbook_part_mut,
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"),
    WorkbookPersonPart(crate
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
    "application/vnd.ms-excel.wsSortMap+xml"), WorksheetThreadedCommentsPart(crate
    ::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::ThreadedComments,
    as_worksheet_threaded_comments_part, as_worksheet_threaded_comments_part_mut,
    "application/vnd.ms-excel.threadedcomments+xml"),
}
pub(crate) fn initialize_root_elements(
  storage: &crate::common::SdkPackageStorage,
  open_settings: &crate::sdk::OpenSettings,
) -> Result<Vec<Option<crate::parts::PartRootElement>>, crate::common::SdkError> {
  let mut root_elements = vec![None; storage.parts().len()];
  if matches!(
    open_settings.root_element_open_mode(),
    crate::common::PackageOpenMode::Eager
  ) {
    for (index, slot) in root_elements.iter_mut().enumerate() {
      let part_id = crate::common::PartId::from_index(index);
      if let Some(root_element) =
        crate::parts::PartRootElement::from_part_id(storage, part_id, open_settings)?
      {
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
      crate::sdk::SdkPackageInternal::open_settings(package),
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
