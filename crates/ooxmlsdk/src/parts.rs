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
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPartRef)]
pub enum PartRef {
  #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk",
        target_name = "afchunk",
        extension = ".dat"
    )]
    AlternativeFormatImportPart(
        crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain",
        target_name = "calcChain",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CalculationChain
        )
    )]
    CalculationChainPart(crate::parts::calculation_chain_part::CalculationChainPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sheetMetadata",
        target_name = "metadata",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Metadata
        )
    )]
    CellMetadataPart(crate::parts::cell_metadata_part::CellMetadataPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle",
        target_name = "colors",
        content_type = "application/vnd.ms-office.chartcolorstyle+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ColorStyle
        )
    )]
    ChartColorStylePart(crate::parts::chart_color_style_part::ChartColorStylePart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartUserShapes",
        target_name = "drawing",
        path_prefix = "../drawings",
        content_type = "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::UserShapes
        )
    )]
    ChartDrawingPart(crate::parts::chart_drawing_part::ChartDrawingPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart",
        target_name = "chart",
        path_prefix = "charts",
        content_type = "application/vnd.openxmlformats-officedocument.drawingml.chart+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartSpace
        )
    )]
    ChartPart(crate::parts::chart_part::ChartPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2011/relationships/chartStyle",
        target_name = "style",
        content_type = "application/vnd.ms-office.chartstyle+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ChartStyle
        )
    )]
    ChartStylePart(crate::parts::chart_style_part::ChartStylePart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet",
        target_name = "sheet",
        path_prefix = "chartsheets",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Chartsheet
        )
    )]
    ChartsheetPart(crate::parts::chartsheet_part::ChartsheetPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/commentAuthors",
        target_name = "commentAuthors",
        content_type = "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentAuthorList
        )
    )]
    CommentAuthorsPart(crate::parts::comment_authors_part::CommentAuthorsPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/connections",
        target_name = "connections",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Connections
        )
    )]
    ConnectionsPart(crate::parts::connections_part::ConnectionsPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/ctrlProp",
        target_name = "ctrlProp",
        path_prefix = "../ctrlProps",
        content_type = "application/vnd.ms-excel.controlproperties+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::FormControlProperties
        )
    )]
    ControlPropertiesPart(crate::parts::control_properties_part::ControlPropertiesPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties",
        target_name = "core",
        path_prefix = "docProps",
        content_type = "application/vnd.openxmlformats-package.core-properties+xml",
        root(element = crate::schemas::opc_core_properties::CoreProperties)
    )]
    CoreFilePropertiesPart(
        crate::parts::core_file_properties_part::CoreFilePropertiesPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2007/relationships/customData",
        target_name = "customData",
        path_prefix = "customData",
        content_type = "application/binary"
    )]
    CustomDataPart(crate::parts::custom_data_part::CustomDataPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2007/relationships/customDataProps",
        target_name = "customDataProps",
        path_prefix = "customData",
        content_type = "application/vnd.ms-excel.customDataProperties+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DatastoreItem
        )
    )]
    CustomDataPropertiesPart(
        crate::parts::custom_data_properties_part::CustomDataPropertiesPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties",
        target_name = "custom",
        path_prefix = "docProps",
        content_type = "application/vnd.openxmlformats-officedocument.custom-properties+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_office_document_2006_custom_properties::Properties
        )
    )]
    CustomFilePropertiesPart(
        crate::parts::custom_file_properties_part::CustomFilePropertiesPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty",
        target_name = "CustomProperty",
        extension = ".bin"
    )]
    CustomPropertyPart(crate::parts::custom_property_part::CustomPropertyPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps",
        target_name = "xmlMaps",
        content_type = "application/xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::MapInfo
        )
    )]
    CustomXmlMappingsPart(crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
        target_name = "item",
        path_prefix = "../customXml"
    )]
    CustomXmlPart(crate::parts::custom_xml_part::CustomXmlPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps",
        target_name = "itemProps",
        content_type = "application/vnd.openxmlformats-officedocument.customXmlProperties+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_office_document_2006_custom_xml::DataStoreItem
        )
    )]
    CustomXmlPropertiesPart(
        crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2006/relationships/keyMapCustomizations",
        target_name = "customizations",
        content_type = "application/vnd.ms-word.keyMapCustomizations+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_word_2006_wordml::TemplateCommandGroup
        )
    )]
    CustomizationPart(crate::parts::customization_part::CustomizationPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors",
        target_name = "colors",
        path_prefix = "../graphics",
        content_type = "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::ColorsDefinition
        )
    )]
    DiagramColorsPart(crate::parts::diagram_colors_part::DiagramColorsPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData",
        target_name = "data",
        path_prefix = "../graphics",
        content_type = "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::DataModelRoot
        )
    )]
    DiagramDataPart(crate::parts::diagram_data_part::DiagramDataPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout",
        target_name = "layout",
        path_prefix = "../graphics",
        content_type = "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::LayoutDefinition
        )
    )]
    DiagramLayoutDefinitionPart(
        crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing",
        target_name = "drawing",
        path_prefix = "../diagrams",
        content_type = "application/vnd.ms-office.drawingml.diagramDrawing+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_drawing_2008_diagram::Drawing
        )
    )]
    DiagramPersistLayoutPart(
        crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle",
        target_name = "quickStyle",
        path_prefix = "../graphics",
        content_type = "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::StyleDefinition
        )
    )]
    DiagramStylePart(crate::parts::diagram_style_part::DiagramStylePart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet",
        target_name = "sheet",
        path_prefix = "dialogsheets",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DialogSheet
        )
    )]
    DialogsheetPart(crate::parts::dialogsheet_part::DialogsheetPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin",
        target_name = "origin",
        path_prefix = "_xmlsignatures",
        content_type = "application/vnd.openxmlformats-package.digital-signature-origin",
        extension = ".sigs"
    )]
    DigitalSignatureOriginPart(
        crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings",
        target_name = "settings",
        content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Settings
        )
    )]
    DocumentSettingsPart(crate::parts::document_settings_part::DocumentSettingsPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2019/05/relationships/documenttasks",
        target_name = "tasks",
        content_type = "application/vnd.ms-office.documenttasks+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_tasks_2019_documenttasks::Tasks
        )
    )]
    DocumentTasksPart(crate::parts::document_tasks_part::DocumentTasksPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing",
        target_name = "drawing",
        path_prefix = "../drawings",
        content_type = "application/vnd.openxmlformats-officedocument.drawing+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::WorksheetDrawing
        )
    )]
    DrawingsPart(crate::parts::drawings_part::DrawingsPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary",
        target_name = "ActiveXControl",
        extension = ".bin"
    )]
    EmbeddedControlPersistenceBinaryDataPart(
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control",
        target_name = "control",
        path_prefix = "embeddings",
        extension = ".bin"
    )]
    EmbeddedControlPersistencePart(
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
        target_name = "embeddedObject",
        path_prefix = "embeddings",
        extension = ".bin"
    )]
    EmbeddedObjectPart(crate::parts::embedded_object_part::EmbeddedObjectPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
        target_name = "package",
        path_prefix = "embeddings",
        extension = ".bin"
    )]
    EmbeddedPackagePart(crate::parts::embedded_package_part::EmbeddedPackagePart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes",
        target_name = "endnotes",
        content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Endnotes
        )
    )]
    EndnotesPart(crate::parts::endnotes_part::EndnotesPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars",
        target_name = "attachedToolbars",
        content_type = "application/vnd.ms-excel.attachedToolbars",
        extension = ".bin"
    )]
    ExcelAttachedToolbarsPart(
        crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2014/relationships/chartEx",
        target_name = "chart",
        path_prefix = "extendedCharts",
        content_type = "application/vnd.ms-office.chartex+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_drawing_2014_chartex::ChartSpace
        )
    )]
    ExtendedChartPart(crate::parts::extended_chart_part::ExtendedChartPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties",
        target_name = "app",
        path_prefix = "docProps",
        content_type = "application/vnd.openxmlformats-officedocument.extended-properties+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_office_document_2006_extended_properties::Properties
        )
    )]
    ExtendedFilePropertiesPart(
        crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink",
        target_name = "externalReference",
        path_prefix = "externalReferences",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExternalLink
        )
    )]
    ExternalWorkbookPart(crate::parts::external_workbook_part::ExternalWorkbookPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag",
        target_name = "featurePropertyBag",
        path_prefix = "featurePropertyBag",
        content_type = "application/vnd.ms-excel.featurepropertybag+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2022_featurepropertybag::FeaturePropertyBags
        )
    )]
    FeaturePropertyBagsPart(
        crate::parts::feature_property_bags_part::FeaturePropertyBagsPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font",
        target_name = "font",
        path_prefix = "fonts",
        extension = ".dat"
    )]
    FontPart(crate::parts::font_part::FontPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable",
        target_name = "fontTable",
        content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Fonts
        )
    )]
    FontTablePart(crate::parts::font_table_part::FontTablePart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer",
        target_name = "footer",
        content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Footer
        )
    )]
    FooterPart(crate::parts::footer_part::FooterPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes",
        target_name = "footnotes",
        content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Footnotes
        )
    )]
    FootnotesPart(crate::parts::footnotes_part::FootnotesPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/glossaryDocument",
        target_name = "document",
        path_prefix = "glossary",
        content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::GlossaryDocument
        )
    )]
    GlossaryDocumentPart(crate::parts::glossary_document_part::GlossaryDocumentPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster",
        target_name = "handoutMaster",
        path_prefix = "handoutMasters",
        content_type = "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::HandoutMaster
        )
    )]
    HandoutMasterPart(crate::parts::handout_master_part::HandoutMasterPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header",
        target_name = "header",
        content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Header
        )
    )]
    HeaderPart(crate::parts::header_part::HeaderPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
        target_name = "image",
        path_prefix = "../media",
        extension = ".bin"
    )]
    ImagePart(crate::parts::image_part::ImagePart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2006/relationships/xlIntlMacrosheet",
        target_name = "intlsheet",
        path_prefix = "macrosheets",
        content_type = "application/vnd.ms-excel.intlmacrosheet+xml"
    )]
    InternationalMacroSheetPart(
        crate::parts::international_macro_sheet_part::InternationalMacroSheetPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels",
        target_name = "LabelInfo",
        path_prefix = "docMetadata",
        content_type = "application/vnd.ms-office.classificationlabels+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_2020_mip_label_metadata::ClassificationLabelList
        )
    )]
    LabelInfoPart(crate::parts::label_info_part::LabelInfoPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2006/relationships/legacyDocTextInfo",
        target_name = "legacyDocTextInfo",
        content_type = "application/vnd.ms-office.legacyDocTextInfo",
        extension = ".bin"
    )]
    LegacyDiagramTextInfoPart(
        crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2006/relationships/legacyDiagramText",
        target_name = "legacyDiagramText",
        content_type = "application/vnd.ms-office.legacyDiagramText",
        extension = ".bin"
    )]
    LegacyDiagramTextPart(crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet",
        target_name = "sheet",
        path_prefix = "macrosheets",
        content_type = "application/vnd.ms-excel.macrosheet+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_excel_2006_main::Macrosheet
        )
    )]
    MacroSheetPart(crate::parts::macro_sheet_part::MacroSheetPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/recipientData",
        target_name = "recipients"
    )]
    MailMergeRecipientDataPart(
        crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
        target_name = "document",
        path_prefix = "word",
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document,
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"
        )
    )]
    MainDocumentPart(crate::parts::main_document_part::MainDocumentPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/model3d",
        target_name = "model3d",
        path_prefix = "../media",
        content_type = "model/gltf-binary",
        extension = ".glb"
    )]
    Model3DReferenceRelationshipPart(
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2019/04/relationships/namedSheetView",
        target_name = "namedSheetView",
        path_prefix = "../namedSheetViews",
        content_type = "application/vnd.ms-excel.namedsheetviews+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2019_namedsheetviews::NamedSheetViews
        )
    )]
    NamedSheetViewsPart(crate::parts::named_sheet_views_part::NamedSheetViewsPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster",
        target_name = "notesMaster",
        path_prefix = "notesMasters",
        content_type = "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesMaster
        )
    )]
    NotesMasterPart(crate::parts::notes_master_part::NotesMasterPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide",
        target_name = "notesSlide",
        path_prefix = "../notesSlides",
        content_type = "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesSlide
        )
    )]
    NotesSlidePart(crate::parts::notes_slide_part::NotesSlidePart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering",
        target_name = "numbering",
        content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Numbering
        )
    )]
    NumberingDefinitionsPart(
        crate::parts::numbering_definitions_part::NumberingDefinitionsPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition",
        target_name = "pivotCacheDefinition",
        path_prefix = "../pivotCache",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheDefinition
        )
    )]
    PivotTableCacheDefinitionPart(
        crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheRecords",
        target_name = "pivotCacheRecords",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheRecords
        )
    )]
    PivotTableCacheRecordsPart(
        crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable",
        target_name = "pivotTable",
        path_prefix = "../pivotTables",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableDefinition
        )
    )]
    PivotTablePart(crate::parts::pivot_table_part::PivotTablePart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2018/10/relationships/authors",
        target_name = "authors",
        content_type = "application/vnd.ms-powerpoint.authors+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::AuthorList
        )
    )]
    PowerPointAuthorsPart(crate::parts::power_point_authors_part::PowerPointAuthorsPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2018/10/relationships/comments",
        target_name = "modernComment",
        path_prefix = "../comments",
        content_type = "application/vnd.ms-powerpoint.comments+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::CommentList
        )
    )]
    PowerPointCommentPart(crate::parts::power_point_comment_part::PowerPointCommentPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
        target_name = "presentation",
        path_prefix = "ppt",
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation,
            content_type = "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"
        )
    )]
    PresentationPart(crate::parts::presentation_part::PresentationPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps",
        target_name = "presProps",
        content_type = "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::PresentationProperties
        )
    )]
    PresentationPropertiesPart(
        crate::parts::presentation_properties_part::PresentationPropertiesPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/queryTable",
        target_name = "queryTable",
        path_prefix = "../queryTables",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::QueryTable
        )
    )]
    QueryTablePart(crate::parts::query_table_part::QueryTablePart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2006/relationships/ui/userCustomization",
        target_name = "customUI",
        path_prefix = "userCustomization",
        content_type = "application/xml"
    )]
    QuickAccessToolbarCustomizationsPart(
        crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdArray",
        target_name = "rdarray",
        path_prefix = "richData",
        content_type = "application/vnd.ms-excel.rdarray+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::ArrayData
        )
    )]
    RdArrayPart(crate::parts::rd_array_part::RdArrayPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue",
        target_name = "rdrichvalue",
        path_prefix = "richData",
        content_type = "application/vnd.ms-excel.rdrichvalue+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueData
        )
    )]
    RdRichValuePart(crate::parts::rd_rich_value_part::RdRichValuePart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure",
        target_name = "rdrichvaluestructure",
        path_prefix = "richData",
        content_type = "application/vnd.ms-excel.rdrichvaluestructure+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueStructures
        )
    )]
    RdRichValueStructurePart(
        crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes",
        target_name = "rdRichValueTypes",
        path_prefix = "richData",
        content_type = "application/vnd.ms-excel.rdrichvaluetypes+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::RichValueTypesInfo
        )
    )]
    RdRichValueTypesPart(crate::parts::rd_rich_value_types_part::RdRichValueTypesPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2020/07/relationships/rdRichValueWebImage",
        target_name = "rdRichValueWebImage",
        path_prefix = "richData",
        content_type = "application/vnd.ms-excel.rdrichvaluewebimage+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2020_richdatawebimage::WebImagesSupportingRichData
        )
    )]
    RdRichValueWebImagePart(
        crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag",
        target_name = "rdsupportingpropertybag",
        path_prefix = "richData",
        content_type = "application/vnd.ms-excel.rdsupportingpropertybag+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBags
        )
    )]
    RdSupportingPropertyBagPart(
        crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure",
        target_name = "rdsupportingpropertybagstructure",
        path_prefix = "richData",
        content_type = "application/vnd.ms-excel.rdsupportingpropertybagstructure+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBagStructures
        )
    )]
    RdSupportingPropertyBagStructurePart(
        crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility",
        target_name = "customUI",
        path_prefix = "customUI",
        content_type = "application/xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_2009_07_customui::CustomUi
        )
    )]
    RibbonAndBackstageCustomizationsPart(
        crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2006/relationships/ui/extensibility",
        target_name = "customUI",
        path_prefix = "customUI",
        content_type = "application/xml"
    )]
    RibbonExtensibilityPart(
        crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/richStyles",
        target_name = "richStyles",
        path_prefix = "richData",
        content_type = "application/vnd.ms-excel.richstyles+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::RichStylesheet
        )
    )]
    RichStylesPart(crate::parts::rich_styles_part::RichStylesPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings",
        target_name = "sharedStrings",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SharedStringTable
        )
    )]
    SharedStringTablePart(crate::parts::shared_string_table_part::SharedStringTablePart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableSingleCells",
        target_name = "tableSingleCells",
        path_prefix = "../tables",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SingleXmlCells
        )
    )]
    SingleCellTablePart(crate::parts::single_cell_table_part::SingleCellTablePart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2007/relationships/slicerCache",
        target_name = "slicerCache",
        path_prefix = "slicerCaches",
        content_type = "application/vnd.ms-excel.slicerCache+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerCacheDefinition
        )
    )]
    SlicerCachePart(crate::parts::slicer_cache_part::SlicerCachePart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2007/relationships/slicer",
        target_name = "slicer",
        path_prefix = "../slicers",
        content_type = "application/vnd.ms-excel.slicer+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Slicers
        )
    )]
    SlicersPart(crate::parts::slicers_part::SlicersPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
        target_name = "comment",
        path_prefix = "../comments",
        content_type = "application/vnd.openxmlformats-officedocument.presentationml.comments+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentList
        )
    )]
    SlideCommentsPart(crate::parts::slide_comments_part::SlideCommentsPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout",
        target_name = "slideLayout",
        path_prefix = "../slideLayouts",
        content_type = "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideLayout
        )
    )]
    SlideLayoutPart(crate::parts::slide_layout_part::SlideLayoutPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster",
        target_name = "slideMaster",
        path_prefix = "slideMasters",
        content_type = "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideMaster
        )
    )]
    SlideMasterPart(crate::parts::slide_master_part::SlideMasterPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
        target_name = "slide",
        path_prefix = "slides",
        content_type = "application/vnd.openxmlformats-officedocument.presentationml.slide+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Slide
        )
    )]
    SlidePart(crate::parts::slide_part::SlidePart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideUpdateInfo",
        target_name = "slideUpdateInfo",
        path_prefix = "slideUpdateInfo",
        content_type = "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideSyncProperties
        )
    )]
    SlideSyncDataPart(crate::parts::slide_sync_data_part::SlideSyncDataPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings",
        target_name = "printerSettings",
        path_prefix = "../printerSettings",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.printerSettings",
        extension = ".bin"
    )]
    SpreadsheetPrinterSettingsPart(
        crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
        target_name = "styles",
        content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles
        )
    )]
    StyleDefinitionsPart(crate::parts::style_definitions_part::StyleDefinitionsPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects",
        target_name = "stylesWithEffects",
        content_type = "application/vnd.ms-word.stylesWithEffects+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles
        )
    )]
    StylesWithEffectsPart(crate::parts::styles_with_effects_part::StylesWithEffectsPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table",
        target_name = "table",
        path_prefix = "../tables",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Table
        )
    )]
    TableDefinitionPart(crate::parts::table_definition_part::TableDefinitionPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles",
        target_name = "tableStyles",
        content_type = "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TableStyleList
        )
    )]
    TableStylesPart(crate::parts::table_styles_part::TableStylesPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride",
        target_name = "themeoverride",
        path_prefix = "theme",
        content_type = "application/vnd.openxmlformats-officedocument.themeOverride+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ThemeOverride
        )
    )]
    ThemeOverridePart(crate::parts::theme_override_part::ThemeOverridePart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
        target_name = "theme",
        path_prefix = "theme",
        content_type = "application/vnd.openxmlformats-officedocument.theme+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Theme
        )
    )]
    ThemePart(crate::parts::theme_part::ThemePart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail",
        target_name = "thumbnail",
        path_prefix = "docProps",
        extension = ".bin"
    )]
    ThumbnailPart(crate::parts::thumbnail_part::ThumbnailPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2011/relationships/timelineCache",
        target_name = "timelineCache",
        path_prefix = "timelineCaches",
        content_type = "application/vnd.ms-excel.timelineCache+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineCacheDefinition
        )
    )]
    TimeLineCachePart(crate::parts::time_line_cache_part::TimeLineCachePart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2011/relationships/timeline",
        target_name = "timeline",
        path_prefix = "../timelines",
        content_type = "application/vnd.ms-excel.timeline+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::Timelines
        )
    )]
    TimeLinePart(crate::parts::time_line_part::TimeLinePart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags",
        target_name = "tag",
        path_prefix = "tags",
        content_type = "application/vnd.openxmlformats-officedocument.presentationml.tags+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TagList
        )
    )]
    UserDefinedTagsPart(crate::parts::user_defined_tags_part::UserDefinedTagsPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2006/relationships/wordVbaData",
        target_name = "vbaData",
        content_type = "application/vnd.ms-word.vbaData+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_word_2006_wordml::VbaSuppData
        )
    )]
    VbaDataPart(crate::parts::vba_data_part::VbaDataPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2006/relationships/vbaProject",
        target_name = "vbaProject",
        content_type = "application/vnd.ms-office.vbaProject",
        extension = ".bin"
    )]
    VbaProjectPart(crate::parts::vba_project_part::VbaProjectPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps",
        target_name = "viewProps",
        content_type = "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::ViewProperties
        )
    )]
    ViewPropertiesPart(crate::parts::view_properties_part::ViewPropertiesPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing",
        target_name = "vmldrawing",
        path_prefix = "../drawings",
        content_type = "application/vnd.openxmlformats-officedocument.vmlDrawing",
        extension = ".vml"
    )]
    VmlDrawingPart(crate::parts::vml_drawing_part::VmlDrawingPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/volatileDependencies",
        target_name = "volatileDependencies",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::VolatileTypes
        )
    )]
    VolatileDependenciesPart(
        crate::parts::volatile_dependencies_part::VolatileDependenciesPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes",
        target_name = "taskpanes",
        path_prefix = "../webextensions",
        content_type = "application/vnd.ms-office.webextensiontaskpanes+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_webextensions_taskpanes_2010_11::Taskpanes
        )
    )]
    WebExTaskpanesPart(crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2011/relationships/webextension",
        target_name = "webextension",
        path_prefix = "../webextensions",
        content_type = "application/vnd.ms-office.webextension+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_webextensions_webextension_2010_11::WebExtension
        )
    )]
    WebExtensionPart(crate::parts::web_extension_part::WebExtensionPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings",
        target_name = "webSettings",
        content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::WebSettings
        )
    )]
    WebSettingsPart(crate::parts::web_settings_part::WebSettingsPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars",
        target_name = "attachedToolbars",
        content_type = "application/vnd.ms-word.attachedToolbars",
        extension = ".bin"
    )]
    WordAttachedToolbarsPart(
        crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2018/08/relationships/commentsExtensible",
        target_name = "commentsExtensible",
        content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtensible+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_word_2018_wordml_cex::CommentsExtensible
        )
    )]
    WordCommentsExtensiblePart(
        crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2011/relationships/commentsExtended",
        target_name = "commentsExtended",
        content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtended+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_word_2012_wordml::CommentsEx
        )
    )]
    WordprocessingCommentsExPart(
        crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2016/09/relationships/commentsIds",
        target_name = "commentsIds",
        content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsIds+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_word_2016_wordml_cid::CommentsIds
        )
    )]
    WordprocessingCommentsIdsPart(
        crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
        target_name = "comments",
        content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Comments
        )
    )]
    WordprocessingCommentsPart(
        crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2011/relationships/people",
        target_name = "people",
        content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.people+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_word_2012_wordml::People
        )
    )]
    WordprocessingPeoplePart(
        crate::parts::wordprocessing_people_part::WordprocessingPeoplePart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings",
        target_name = "printerSettings",
        path_prefix = "../printerSettings",
        content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.printerSettings",
        extension = ".bin"
    )]
    WordprocessingPrinterSettingsPart(
        crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
        target_name = "workbook",
        path_prefix = "xl",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"
        )
    )]
    WorkbookPart(crate::parts::workbook_part::WorkbookPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2017/10/relationships/person",
        target_name = "person",
        path_prefix = "persons",
        content_type = "application/vnd.ms-excel.person+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::PersonList
        )
    )]
    WorkbookPersonPart(crate::parts::workbook_person_part::WorkbookPersonPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders",
        target_name = "revisionHeaders",
        path_prefix = "revisions",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Headers
        )
    )]
    WorkbookRevisionHeaderPart(
        crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionLog",
        target_name = "revisionLog",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Revisions
        )
    )]
    WorkbookRevisionLogPart(
        crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
        target_name = "styles",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Stylesheet
        )
    )]
    WorkbookStylesPart(crate::parts::workbook_styles_part::WorkbookStylesPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/usernames",
        target_name = "userNames",
        path_prefix = "revisions",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Users
        )
    )]
    WorkbookUserDataPart(crate::parts::workbook_user_data_part::WorkbookUserDataPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
        target_name = "comments",
        path_prefix = "..",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Comments
        )
    )]
    WorksheetCommentsPart(crate::parts::worksheet_comments_part::WorksheetCommentsPart),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet",
        target_name = "sheet",
        path_prefix = "worksheets",
        content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml",
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Worksheet
        )
    )]
    WorksheetPart(crate::parts::worksheet_part::WorksheetPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2006/relationships/wsSortMap",
        target_name = "wsSortMap",
        content_type = "application/vnd.ms-excel.wsSortMap+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_excel_2006_main::WorksheetSortMap
        )
    )]
    WorksheetSortMapPart(crate::parts::worksheet_sort_map_part::WorksheetSortMapPart),
    #[sdk(
        relationship_type = "http://schemas.microsoft.com/office/2017/10/relationships/threadedComment",
        target_name = "threadedcomment",
        path_prefix = "../threadedcomments",
        content_type = "application/vnd.ms-excel.threadedcomments+xml",
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::ThreadedComments
        )
    )]
    WorksheetThreadedCommentsPart(
        crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart,
    ),
    #[sdk(
        relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/signature",
        target_name = "sig",
        path_prefix = "_xmlsignatures",
        content_type = "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml"
    )]
    XmlSignaturePart(crate::parts::xml_signature_part::XmlSignaturePart),
    ExtendedPart(crate::parts::extended_part::ExtendedPart),
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
  let part_count = crate::sdk::SdkPackage::storage(package).parts().len();
  for index in 0..part_count {
    let part_id = crate::common::PartId::from_index(index);
    if crate::sdk::SdkPackage::storage(package)
      .part(part_id)
      .is_none()
      || crate::sdk::SdkPackage::is_root_element_loaded(package, part_id)
    {
      continue;
    }
    let root_element = crate::parts::PartRootElement::from_part_id(
      crate::sdk::SdkPackage::storage(package),
      part_id,
      crate::sdk::SdkPackage::open_settings(package),
    )?;
    if let Some(root_element) = root_element
      && let Some(slot) = crate::sdk::SdkPackage::root_element_slot_mut(package, part_id)
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
  let storage = crate::sdk::SdkPackage::storage(package);
  let open_settings = crate::sdk::SdkPackage::open_settings(package);
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
    && relationship.relationship_type_bytes()
      == b"http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain"
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
  let storage = crate::sdk::SdkPackage::storage(package);
  zip.start_file("[Content_Types].xml", options)?;
  crate::sdk::SdkType::write_to(storage.content_types(), &mut zip)?;
  let package_relationships = storage.package_relationships();
  if !package_relationships.is_empty() {
    if entry_set.insert("_rels".to_string()) {
      zip.add_directory("_rels", options)?;
    }
    zip.start_file("_rels/.rels", options)?;
    package_relationships.write_xml(&mut zip)?;
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
      relationships.write_xml(&mut zip)?;
    }
    zip.start_file(part.path(), options)?;
    if let Some(root_element) = crate::sdk::SdkPackage::root_element(package, part_id) {
      root_element.write_to(&mut zip)?;
    } else {
      zip.write_all(part.data().bytes())?;
    }
  }
  zip.finish()?;
  Ok(())
}
