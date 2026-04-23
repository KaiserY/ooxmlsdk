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
#[derive(Clone, Copy, Debug)]
pub enum PartRef<'a> {
  AlternativeFormatImportPart(
        &'a crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
    ),
    CalculationChainPart(&'a crate::parts::calculation_chain_part::CalculationChainPart),
    CellMetadataPart(&'a crate::parts::cell_metadata_part::CellMetadataPart),
    #[cfg(feature = "microsoft365")]
    ChartColorStylePart(&'a crate::parts::chart_color_style_part::ChartColorStylePart),
    ChartDrawingPart(&'a crate::parts::chart_drawing_part::ChartDrawingPart),
    ChartPart(&'a crate::parts::chart_part::ChartPart),
    #[cfg(feature = "microsoft365")]
    ChartStylePart(&'a crate::parts::chart_style_part::ChartStylePart),
    ChartsheetPart(&'a crate::parts::chartsheet_part::ChartsheetPart),
    CommentAuthorsPart(&'a crate::parts::comment_authors_part::CommentAuthorsPart),
    ConnectionsPart(&'a crate::parts::connections_part::ConnectionsPart),
    #[cfg(feature = "microsoft365")]
    ControlPropertiesPart(
        &'a crate::parts::control_properties_part::ControlPropertiesPart,
    ),
    CoreFilePropertiesPart(
        &'a crate::parts::core_file_properties_part::CoreFilePropertiesPart,
    ),
    #[cfg(feature = "microsoft365")]
    CustomDataPart(&'a crate::parts::custom_data_part::CustomDataPart),
    #[cfg(feature = "microsoft365")]
    CustomDataPropertiesPart(
        &'a crate::parts::custom_data_properties_part::CustomDataPropertiesPart,
    ),
    CustomFilePropertiesPart(
        &'a crate::parts::custom_file_properties_part::CustomFilePropertiesPart,
    ),
    CustomPropertyPart(&'a crate::parts::custom_property_part::CustomPropertyPart),
    CustomXmlMappingsPart(
        &'a crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart,
    ),
    CustomXmlPart(&'a crate::parts::custom_xml_part::CustomXmlPart),
    CustomXmlPropertiesPart(
        &'a crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart,
    ),
    CustomizationPart(&'a crate::parts::customization_part::CustomizationPart),
    DiagramColorsPart(&'a crate::parts::diagram_colors_part::DiagramColorsPart),
    DiagramDataPart(&'a crate::parts::diagram_data_part::DiagramDataPart),
    DiagramLayoutDefinitionPart(
        &'a crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
    ),
    #[cfg(feature = "microsoft365")]
    DiagramPersistLayoutPart(
        &'a crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart,
    ),
    DiagramStylePart(&'a crate::parts::diagram_style_part::DiagramStylePart),
    DialogsheetPart(&'a crate::parts::dialogsheet_part::DialogsheetPart),
    DigitalSignatureOriginPart(
        &'a crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart,
    ),
    DocumentSettingsPart(&'a crate::parts::document_settings_part::DocumentSettingsPart),
    #[cfg(feature = "microsoft365")]
    DocumentTasksPart(&'a crate::parts::document_tasks_part::DocumentTasksPart),
    DrawingsPart(&'a crate::parts::drawings_part::DrawingsPart),
    EmbeddedControlPersistenceBinaryDataPart(
        &'a crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    ),
    EmbeddedControlPersistencePart(
        &'a crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    ),
    EmbeddedObjectPart(&'a crate::parts::embedded_object_part::EmbeddedObjectPart),
    EmbeddedPackagePart(&'a crate::parts::embedded_package_part::EmbeddedPackagePart),
    EndnotesPart(&'a crate::parts::endnotes_part::EndnotesPart),
    ExcelAttachedToolbarsPart(
        &'a crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart,
    ),
    #[cfg(feature = "microsoft365")]
    ExtendedChartPart(&'a crate::parts::extended_chart_part::ExtendedChartPart),
    ExtendedFilePropertiesPart(
        &'a crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart,
    ),
    ExternalWorkbookPart(&'a crate::parts::external_workbook_part::ExternalWorkbookPart),
    #[cfg(feature = "microsoft365")]
    FeaturePropertyBagsPart(
        &'a crate::parts::feature_property_bags_part::FeaturePropertyBagsPart,
    ),
    FontPart(&'a crate::parts::font_part::FontPart),
    FontTablePart(&'a crate::parts::font_table_part::FontTablePart),
    FooterPart(&'a crate::parts::footer_part::FooterPart),
    FootnotesPart(&'a crate::parts::footnotes_part::FootnotesPart),
    GlossaryDocumentPart(&'a crate::parts::glossary_document_part::GlossaryDocumentPart),
    HandoutMasterPart(&'a crate::parts::handout_master_part::HandoutMasterPart),
    HeaderPart(&'a crate::parts::header_part::HeaderPart),
    ImagePart(&'a crate::parts::image_part::ImagePart),
    InternationalMacroSheetPart(
        &'a crate::parts::international_macro_sheet_part::InternationalMacroSheetPart,
    ),
    #[cfg(feature = "microsoft365")]
    LabelInfoPart(&'a crate::parts::label_info_part::LabelInfoPart),
    LegacyDiagramTextInfoPart(
        &'a crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart,
    ),
    LegacyDiagramTextPart(
        &'a crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart,
    ),
    MacroSheetPart(&'a crate::parts::macro_sheet_part::MacroSheetPart),
    MailMergeRecipientDataPart(
        &'a crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
    ),
    MainDocumentPart(&'a crate::parts::main_document_part::MainDocumentPart),
    #[cfg(feature = "microsoft365")]
    Model3DReferenceRelationshipPart(
        &'a crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    ),
    #[cfg(feature = "microsoft365")]
    NamedSheetViewsPart(&'a crate::parts::named_sheet_views_part::NamedSheetViewsPart),
    NotesMasterPart(&'a crate::parts::notes_master_part::NotesMasterPart),
    NotesSlidePart(&'a crate::parts::notes_slide_part::NotesSlidePart),
    NumberingDefinitionsPart(
        &'a crate::parts::numbering_definitions_part::NumberingDefinitionsPart,
    ),
    PivotTableCacheDefinitionPart(
        &'a crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart,
    ),
    PivotTableCacheRecordsPart(
        &'a crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart,
    ),
    PivotTablePart(&'a crate::parts::pivot_table_part::PivotTablePart),
    #[cfg(feature = "microsoft365")]
    PowerPointAuthorsPart(
        &'a crate::parts::power_point_authors_part::PowerPointAuthorsPart,
    ),
    #[cfg(feature = "microsoft365")]
    PowerPointCommentPart(
        &'a crate::parts::power_point_comment_part::PowerPointCommentPart,
    ),
    PresentationDocument(&'a crate::parts::presentation_document::PresentationDocument),
    PresentationPart(&'a crate::parts::presentation_part::PresentationPart),
    PresentationPropertiesPart(
        &'a crate::parts::presentation_properties_part::PresentationPropertiesPart,
    ),
    QueryTablePart(&'a crate::parts::query_table_part::QueryTablePart),
    QuickAccessToolbarCustomizationsPart(
        &'a crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart,
    ),
    #[cfg(feature = "microsoft365")]
    RdArrayPart(&'a crate::parts::rd_array_part::RdArrayPart),
    #[cfg(feature = "microsoft365")]
    RdRichValuePart(&'a crate::parts::rd_rich_value_part::RdRichValuePart),
    #[cfg(feature = "microsoft365")]
    RdRichValueStructurePart(
        &'a crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart,
    ),
    #[cfg(feature = "microsoft365")]
    RdRichValueTypesPart(
        &'a crate::parts::rd_rich_value_types_part::RdRichValueTypesPart,
    ),
    #[cfg(feature = "microsoft365")]
    RdRichValueWebImagePart(
        &'a crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart,
    ),
    #[cfg(feature = "microsoft365")]
    RdSupportingPropertyBagPart(
        &'a crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart,
    ),
    #[cfg(feature = "microsoft365")]
    RdSupportingPropertyBagStructurePart(
        &'a crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart,
    ),
    #[cfg(feature = "microsoft365")]
    RibbonAndBackstageCustomizationsPart(
        &'a crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart,
    ),
    RibbonExtensibilityPart(
        &'a crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart,
    ),
    #[cfg(feature = "microsoft365")]
    RichStylesPart(&'a crate::parts::rich_styles_part::RichStylesPart),
    SharedStringTablePart(
        &'a crate::parts::shared_string_table_part::SharedStringTablePart,
    ),
    SingleCellTablePart(&'a crate::parts::single_cell_table_part::SingleCellTablePart),
    #[cfg(feature = "microsoft365")]
    SlicerCachePart(&'a crate::parts::slicer_cache_part::SlicerCachePart),
    #[cfg(feature = "microsoft365")]
    SlicersPart(&'a crate::parts::slicers_part::SlicersPart),
    SlideCommentsPart(&'a crate::parts::slide_comments_part::SlideCommentsPart),
    SlideLayoutPart(&'a crate::parts::slide_layout_part::SlideLayoutPart),
    SlideMasterPart(&'a crate::parts::slide_master_part::SlideMasterPart),
    SlidePart(&'a crate::parts::slide_part::SlidePart),
    SlideSyncDataPart(&'a crate::parts::slide_sync_data_part::SlideSyncDataPart),
    SpreadsheetDocument(&'a crate::parts::spreadsheet_document::SpreadsheetDocument),
    SpreadsheetPrinterSettingsPart(
        &'a crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
    ),
    StyleDefinitionsPart(&'a crate::parts::style_definitions_part::StyleDefinitionsPart),
    #[cfg(feature = "microsoft365")]
    StylesWithEffectsPart(
        &'a crate::parts::styles_with_effects_part::StylesWithEffectsPart,
    ),
    TableDefinitionPart(&'a crate::parts::table_definition_part::TableDefinitionPart),
    TableStylesPart(&'a crate::parts::table_styles_part::TableStylesPart),
    ThemeOverridePart(&'a crate::parts::theme_override_part::ThemeOverridePart),
    ThemePart(&'a crate::parts::theme_part::ThemePart),
    ThumbnailPart(&'a crate::parts::thumbnail_part::ThumbnailPart),
    #[cfg(feature = "microsoft365")]
    TimeLineCachePart(&'a crate::parts::time_line_cache_part::TimeLineCachePart),
    #[cfg(feature = "microsoft365")]
    TimeLinePart(&'a crate::parts::time_line_part::TimeLinePart),
    UserDefinedTagsPart(&'a crate::parts::user_defined_tags_part::UserDefinedTagsPart),
    VbaDataPart(&'a crate::parts::vba_data_part::VbaDataPart),
    VbaProjectPart(&'a crate::parts::vba_project_part::VbaProjectPart),
    ViewPropertiesPart(&'a crate::parts::view_properties_part::ViewPropertiesPart),
    VmlDrawingPart(&'a crate::parts::vml_drawing_part::VmlDrawingPart),
    VolatileDependenciesPart(
        &'a crate::parts::volatile_dependencies_part::VolatileDependenciesPart,
    ),
    #[cfg(feature = "microsoft365")]
    WebExTaskpanesPart(&'a crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart),
    #[cfg(feature = "microsoft365")]
    WebExtensionPart(&'a crate::parts::web_extension_part::WebExtensionPart),
    WebSettingsPart(&'a crate::parts::web_settings_part::WebSettingsPart),
    WordAttachedToolbarsPart(
        &'a crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart,
    ),
    #[cfg(feature = "microsoft365")]
    WordCommentsExtensiblePart(
        &'a crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart,
    ),
    #[cfg(feature = "microsoft365")]
    WordprocessingCommentsExPart(
        &'a crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart,
    ),
    #[cfg(feature = "microsoft365")]
    WordprocessingCommentsIdsPart(
        &'a crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart,
    ),
    WordprocessingCommentsPart(
        &'a crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart,
    ),
    WordprocessingDocument(
        &'a crate::parts::wordprocessing_document::WordprocessingDocument,
    ),
    #[cfg(feature = "microsoft365")]
    WordprocessingPeoplePart(
        &'a crate::parts::wordprocessing_people_part::WordprocessingPeoplePart,
    ),
    WordprocessingPrinterSettingsPart(
        &'a crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart,
    ),
    WorkbookPart(&'a crate::parts::workbook_part::WorkbookPart),
    #[cfg(feature = "microsoft365")]
    WorkbookPersonPart(&'a crate::parts::workbook_person_part::WorkbookPersonPart),
    WorkbookRevisionHeaderPart(
        &'a crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart,
    ),
    WorkbookRevisionLogPart(
        &'a crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart,
    ),
    WorkbookStylesPart(&'a crate::parts::workbook_styles_part::WorkbookStylesPart),
    WorkbookUserDataPart(
        &'a crate::parts::workbook_user_data_part::WorkbookUserDataPart,
    ),
    WorksheetCommentsPart(
        &'a crate::parts::worksheet_comments_part::WorksheetCommentsPart,
    ),
    WorksheetPart(&'a crate::parts::worksheet_part::WorksheetPart),
    WorksheetSortMapPart(
        &'a crate::parts::worksheet_sort_map_part::WorksheetSortMapPart,
    ),
    #[cfg(feature = "microsoft365")]
    WorksheetThreadedCommentsPart(
        &'a crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart,
    ),
    XmlSignaturePart(&'a crate::parts::xml_signature_part::XmlSignaturePart),
}
impl<'a> PartRef<'a> {
  pub fn downcast_ref<T: crate::sdk::SdkPart + 'static>(&self) -> Option<&'a T> {
    match self {
      PartRef::AlternativeFormatImportPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::CalculationChainPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::CellMetadataPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::ChartColorStylePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::ChartDrawingPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::ChartPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::ChartStylePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::ChartsheetPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::CommentAuthorsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::ConnectionsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::ControlPropertiesPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::CoreFilePropertiesPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::CustomDataPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::CustomDataPropertiesPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::CustomFilePropertiesPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::CustomPropertyPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::CustomXmlMappingsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::CustomXmlPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::CustomXmlPropertiesPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::CustomizationPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::DiagramColorsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::DiagramDataPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::DiagramLayoutDefinitionPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::DiagramPersistLayoutPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::DiagramStylePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::DialogsheetPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::DigitalSignatureOriginPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::DocumentSettingsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::DocumentTasksPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::DrawingsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::EmbeddedControlPersistenceBinaryDataPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::EmbeddedControlPersistencePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::EmbeddedObjectPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::EmbeddedPackagePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::EndnotesPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::ExcelAttachedToolbarsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::ExtendedChartPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::ExtendedFilePropertiesPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::ExternalWorkbookPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::FeaturePropertyBagsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::FontPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::FontTablePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::FooterPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::FootnotesPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::GlossaryDocumentPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::HandoutMasterPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::HeaderPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::ImagePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::InternationalMacroSheetPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::LabelInfoPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::LegacyDiagramTextInfoPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::LegacyDiagramTextPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::MacroSheetPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::MailMergeRecipientDataPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::MainDocumentPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::Model3DReferenceRelationshipPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::NamedSheetViewsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::NotesMasterPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::NotesSlidePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::NumberingDefinitionsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::PivotTableCacheDefinitionPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::PivotTableCacheRecordsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::PivotTablePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::PowerPointAuthorsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::PowerPointCommentPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::PresentationDocument(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::PresentationPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::PresentationPropertiesPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::QueryTablePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::QuickAccessToolbarCustomizationsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RdArrayPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RdRichValuePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RdRichValueStructurePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RdRichValueTypesPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RdRichValueWebImagePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RdSupportingPropertyBagPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RdSupportingPropertyBagStructurePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RibbonAndBackstageCustomizationsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::RibbonExtensibilityPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RichStylesPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::SharedStringTablePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::SingleCellTablePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::SlicerCachePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::SlicersPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::SlideCommentsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::SlideLayoutPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::SlideMasterPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::SlidePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::SlideSyncDataPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::SpreadsheetDocument(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::SpreadsheetPrinterSettingsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::StyleDefinitionsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::StylesWithEffectsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::TableDefinitionPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::TableStylesPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::ThemeOverridePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::ThemePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::ThumbnailPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::TimeLineCachePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::TimeLinePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::UserDefinedTagsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::VbaDataPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::VbaProjectPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::ViewPropertiesPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::VmlDrawingPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::VolatileDependenciesPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::WebExTaskpanesPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::WebExtensionPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::WebSettingsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::WordAttachedToolbarsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::WordCommentsExtensiblePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::WordprocessingCommentsExPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::WordprocessingCommentsIdsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::WordprocessingCommentsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::WordprocessingDocument(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::WordprocessingPeoplePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::WordprocessingPrinterSettingsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::WorkbookPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::WorkbookPersonPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::WorkbookRevisionHeaderPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::WorkbookRevisionLogPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::WorkbookStylesPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::WorkbookUserDataPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::WorksheetCommentsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::WorksheetPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::WorksheetSortMapPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      #[cfg(feature = "microsoft365")]
      PartRef::WorksheetThreadedCommentsPart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
      PartRef::XmlSignaturePart(part) => {
        let any: &dyn std::any::Any = *part;
        any.downcast_ref::<T>()
      }
    }
  }
}
#[derive(Clone, Copy, Debug)]
pub struct IdPartPair<'a> {
  pub relationship_id: &'a str,
  pub part: PartRef<'a>,
}
impl<'a> IdPartPair<'a> {
  pub const fn new(relationship_id: &'a str, part: PartRef<'a>) -> Self {
    Self {
      relationship_id,
      part,
    }
  }
}
