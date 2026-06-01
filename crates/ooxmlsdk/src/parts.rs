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
  #[sdk(relationship_type = RelationshipAFChunk)]
    AlternativeFormatImportPart(
        crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
    ),
    #[sdk(
        relationship_type = RelationshipCalcChain,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CalculationChain,
            accessor = as_calculation_chain_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml"
        )
    )]
    CalculationChainPart(crate::parts::calculation_chain_part::CalculationChainPart),
    #[sdk(
        relationship_type = RelationshipSheetMetadata,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Metadata,
            accessor = as_cell_metadata_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml"
        )
    )]
    CellMetadataPart(crate::parts::cell_metadata_part::CellMetadataPart),
    #[sdk(
        relationship_type = RelationshipChartColorStyle,
        root(
            element = crate::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ColorStyle,
            accessor = as_chart_color_style_part,
            content_type = "application/vnd.ms-office.chartcolorstyle+xml"
        )
    )]
    ChartColorStylePart(crate::parts::chart_color_style_part::ChartColorStylePart),
    #[sdk(
        relationship_type = RelationshipChartUserShapes,
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::UserShapes,
            accessor = as_chart_drawing_part,
            content_type = "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml"
        )
    )]
    ChartDrawingPart(crate::parts::chart_drawing_part::ChartDrawingPart),
    #[sdk(
        relationship_type = RelationshipChart,
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartSpace,
            accessor = as_chart_part,
            content_type = "application/vnd.openxmlformats-officedocument.drawingml.chart+xml"
        )
    )]
    ChartPart(crate::parts::chart_part::ChartPart),
    #[sdk(
        relationship_type = RelationshipChartStyle,
        root(
            element = crate::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ChartStyle,
            accessor = as_chart_style_part,
            content_type = "application/vnd.ms-office.chartstyle+xml"
        )
    )]
    ChartStylePart(crate::parts::chart_style_part::ChartStylePart),
    #[sdk(
        relationship_type = RelationshipChartsheet,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Chartsheet,
            accessor = as_chartsheet_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml"
        )
    )]
    ChartsheetPart(crate::parts::chartsheet_part::ChartsheetPart),
    #[sdk(
        relationship_type = RelationshipCommentAuthors,
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentAuthorList,
            accessor = as_comment_authors_part,
            content_type = "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml"
        )
    )]
    CommentAuthorsPart(crate::parts::comment_authors_part::CommentAuthorsPart),
    #[sdk(
        relationship_type = RelationshipConnections,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Connections,
            accessor = as_connections_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml"
        )
    )]
    ConnectionsPart(crate::parts::connections_part::ConnectionsPart),
    #[sdk(
        relationship_type = RelationshipCtrlProp,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::FormControlProperties,
            accessor = as_control_properties_part,
            content_type = "application/vnd.ms-excel.controlproperties+xml"
        )
    )]
    ControlPropertiesPart(crate::parts::control_properties_part::ControlPropertiesPart),
    #[sdk(
        relationship_type = RelationshipCoreProperties,
        root(
            element = crate::schemas::opc_core_properties::CoreProperties,
            accessor = as_core_file_properties_part,
            content_type = "application/vnd.openxmlformats-package.core-properties+xml"
        )
    )]
    CoreFilePropertiesPart(
        crate::parts::core_file_properties_part::CoreFilePropertiesPart,
    ),
    #[sdk(relationship_type = RelationshipCustomData)]
    CustomDataPart(crate::parts::custom_data_part::CustomDataPart),
    #[sdk(
        relationship_type = RelationshipCustomDataProps,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DatastoreItem,
            accessor = as_custom_data_properties_part,
            content_type = "application/vnd.ms-excel.customDataProperties+xml"
        )
    )]
    CustomDataPropertiesPart(
        crate::parts::custom_data_properties_part::CustomDataPropertiesPart,
    ),
    #[sdk(
        relationship_type = RelationshipCustomProperties,
        root(
            element = crate::schemas::schemas_openxmlformats_org_office_document_2006_custom_properties::Properties,
            accessor = as_custom_file_properties_part,
            content_type = "application/vnd.openxmlformats-officedocument.custom-properties+xml"
        )
    )]
    CustomFilePropertiesPart(
        crate::parts::custom_file_properties_part::CustomFilePropertiesPart,
    ),
    #[sdk(relationship_type = RelationshipCustomProperty)]
    CustomPropertyPart(crate::parts::custom_property_part::CustomPropertyPart),
    #[sdk(
        relationship_type = RelationshipXmlMaps,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::MapInfo,
            accessor = as_custom_xml_mappings_part,
            content_type = "application/xml"
        )
    )]
    CustomXmlMappingsPart(crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart),
    #[sdk(relationship_type = RelationshipCustomXml)]
    CustomXmlPart(crate::parts::custom_xml_part::CustomXmlPart),
    #[sdk(
        relationship_type = RelationshipCustomXmlProps,
        root(
            element = crate::schemas::schemas_openxmlformats_org_office_document_2006_custom_xml::DataStoreItem,
            accessor = as_custom_xml_properties_part,
            content_type = "application/vnd.openxmlformats-officedocument.customXmlProperties+xml"
        )
    )]
    CustomXmlPropertiesPart(
        crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart,
    ),
    #[sdk(
        relationship_type = RelationshipKeyMapCustomizations,
        root(
            element = crate::schemas::schemas_microsoft_com_office_word_2006_wordml::TemplateCommandGroup,
            accessor = as_customization_part,
            content_type = "application/vnd.ms-word.keyMapCustomizations+xml"
        )
    )]
    CustomizationPart(crate::parts::customization_part::CustomizationPart),
    #[sdk(
        relationship_type = RelationshipDiagramColors,
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::ColorsDefinition,
            accessor = as_diagram_colors_part,
            content_type = "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml"
        )
    )]
    DiagramColorsPart(crate::parts::diagram_colors_part::DiagramColorsPart),
    #[sdk(
        relationship_type = RelationshipDiagramData,
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::DataModelRoot,
            accessor = as_diagram_data_part,
            content_type = "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml"
        )
    )]
    DiagramDataPart(crate::parts::diagram_data_part::DiagramDataPart),
    #[sdk(
        relationship_type = RelationshipDiagramLayout,
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::LayoutDefinition,
            accessor = as_diagram_layout_definition_part,
            content_type = "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml"
        )
    )]
    DiagramLayoutDefinitionPart(
        crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
    ),
    #[sdk(
        relationship_type = RelationshipDiagramDrawing,
        root(
            element = crate::schemas::schemas_microsoft_com_office_drawing_2008_diagram::Drawing,
            accessor = as_diagram_persist_layout_part,
            content_type = "application/vnd.ms-office.drawingml.diagramDrawing+xml"
        )
    )]
    DiagramPersistLayoutPart(
        crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart,
    ),
    #[sdk(
        relationship_type = RelationshipDiagramQuickStyle,
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::StyleDefinition,
            accessor = as_diagram_style_part,
            content_type = "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml"
        )
    )]
    DiagramStylePart(crate::parts::diagram_style_part::DiagramStylePart),
    #[sdk(
        relationship_type = RelationshipDialogsheet,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DialogSheet,
            accessor = as_dialogsheet_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml"
        )
    )]
    DialogsheetPart(crate::parts::dialogsheet_part::DialogsheetPart),
    #[sdk(relationship_type = RelationshipOrigin)]
    DigitalSignatureOriginPart(
        crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart,
    ),
    #[sdk(
        relationship_type = RelationshipSettings,
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Settings,
            accessor = as_document_settings_part,
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"
        )
    )]
    DocumentSettingsPart(crate::parts::document_settings_part::DocumentSettingsPart),
    #[sdk(
        relationship_type = RelationshipDocumenttasks,
        root(
            element = crate::schemas::schemas_microsoft_com_office_tasks_2019_documenttasks::Tasks,
            accessor = as_document_tasks_part,
            content_type = "application/vnd.ms-office.documenttasks+xml"
        )
    )]
    DocumentTasksPart(crate::parts::document_tasks_part::DocumentTasksPart),
    #[sdk(
        relationship_type = RelationshipDrawing,
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::WorksheetDrawing,
            accessor = as_drawings_part,
            content_type = "application/vnd.openxmlformats-officedocument.drawing+xml"
        )
    )]
    DrawingsPart(crate::parts::drawings_part::DrawingsPart),
    #[sdk(relationship_type = RelationshipActiveXControlBinary)]
    EmbeddedControlPersistenceBinaryDataPart(
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    ),
    #[sdk(relationship_type = RelationshipControl)]
    EmbeddedControlPersistencePart(
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    ),
    #[sdk(relationship_type = RelationshipOleObject)]
    EmbeddedObjectPart(crate::parts::embedded_object_part::EmbeddedObjectPart),
    #[sdk(relationship_type = RelationshipPackage)]
    EmbeddedPackagePart(crate::parts::embedded_package_part::EmbeddedPackagePart),
    #[sdk(
        relationship_type = RelationshipEndnotes,
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Endnotes,
            accessor = as_endnotes_part,
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml"
        )
    )]
    EndnotesPart(crate::parts::endnotes_part::EndnotesPart),
    #[sdk(relationship_type = RelationshipAttachedToolbars)]
    ExcelAttachedToolbarsPart(
        crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart,
    ),
    #[sdk(
        relationship_type = RelationshipChartEx,
        root(
            element = crate::schemas::schemas_microsoft_com_office_drawing_2014_chartex::ChartSpace,
            accessor = as_extended_chart_part,
            content_type = "application/vnd.ms-office.chartex+xml"
        )
    )]
    ExtendedChartPart(crate::parts::extended_chart_part::ExtendedChartPart),
    #[sdk(
        relationship_type = RelationshipExtendedProperties,
        root(
            element = crate::schemas::schemas_openxmlformats_org_office_document_2006_extended_properties::Properties,
            accessor = as_extended_file_properties_part,
            content_type = "application/vnd.openxmlformats-officedocument.extended-properties+xml"
        )
    )]
    ExtendedFilePropertiesPart(
        crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart,
    ),
    #[sdk(
        relationship_type = RelationshipExternalLink,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExternalLink,
            accessor = as_external_workbook_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml"
        )
    )]
    ExternalWorkbookPart(crate::parts::external_workbook_part::ExternalWorkbookPart),
    #[sdk(
        relationship_type = RelationshipFeaturePropertyBag,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2022_featurepropertybag::FeaturePropertyBags,
            accessor = as_feature_property_bags_part,
            content_type = "application/vnd.ms-excel.featurepropertybag+xml"
        )
    )]
    FeaturePropertyBagsPart(
        crate::parts::feature_property_bags_part::FeaturePropertyBagsPart,
    ),
    #[sdk(relationship_type = RelationshipFont)]
    FontPart(crate::parts::font_part::FontPart),
    #[sdk(
        relationship_type = RelationshipFontTable,
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Fonts,
            accessor = as_font_table_part,
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"
        )
    )]
    FontTablePart(crate::parts::font_table_part::FontTablePart),
    #[sdk(
        relationship_type = RelationshipFooter,
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Footer,
            accessor = as_footer_part,
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"
        )
    )]
    FooterPart(crate::parts::footer_part::FooterPart),
    #[sdk(
        relationship_type = RelationshipFootnotes,
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Footnotes,
            accessor = as_footnotes_part,
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml"
        )
    )]
    FootnotesPart(crate::parts::footnotes_part::FootnotesPart),
    #[sdk(
        relationship_type = RelationshipGlossaryDocument,
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::GlossaryDocument,
            accessor = as_glossary_document_part,
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml"
        )
    )]
    GlossaryDocumentPart(crate::parts::glossary_document_part::GlossaryDocumentPart),
    #[sdk(
        relationship_type = RelationshipHandoutMaster,
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::HandoutMaster,
            accessor = as_handout_master_part,
            content_type = "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml"
        )
    )]
    HandoutMasterPart(crate::parts::handout_master_part::HandoutMasterPart),
    #[sdk(
        relationship_type = RelationshipHeader,
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Header,
            accessor = as_header_part,
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml"
        )
    )]
    HeaderPart(crate::parts::header_part::HeaderPart),
    #[sdk(relationship_type = RelationshipImage)]
    ImagePart(crate::parts::image_part::ImagePart),
    #[sdk(relationship_type = RelationshipXlIntlMacrosheet)]
    InternationalMacroSheetPart(
        crate::parts::international_macro_sheet_part::InternationalMacroSheetPart,
    ),
    #[sdk(
        relationship_type = RelationshipClassificationlabels,
        root(
            element = crate::schemas::schemas_microsoft_com_office_2020_mip_label_metadata::ClassificationLabelList,
            accessor = as_label_info_part,
            content_type = "application/vnd.ms-office.classificationlabels+xml"
        )
    )]
    LabelInfoPart(crate::parts::label_info_part::LabelInfoPart),
    #[sdk(relationship_type = RelationshipLegacyDocTextInfo)]
    LegacyDiagramTextInfoPart(
        crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart,
    ),
    #[sdk(relationship_type = RelationshipLegacyDiagramText)]
    LegacyDiagramTextPart(crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart),
    #[sdk(
        relationship_type = RelationshipXlMacrosheet,
        root(
            element = crate::schemas::schemas_microsoft_com_office_excel_2006_main::Macrosheet,
            accessor = as_macro_sheet_part,
            content_type = "application/vnd.ms-excel.macrosheet+xml"
        )
    )]
    MacroSheetPart(crate::parts::macro_sheet_part::MacroSheetPart),
    #[sdk(relationship_type = RelationshipRecipientData)]
    MailMergeRecipientDataPart(
        crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
    ),
    #[sdk(
        relationship_type = RelationshipOfficeDocument,
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document,
            accessor = as_main_document_part,
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"
        )
    )]
    MainDocumentPart(crate::parts::main_document_part::MainDocumentPart),
    #[sdk(relationship_type = RelationshipModel3d)]
    Model3DReferenceRelationshipPart(
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    ),
    #[sdk(
        relationship_type = RelationshipNamedSheetView,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2019_namedsheetviews::NamedSheetViews,
            accessor = as_named_sheet_views_part,
            content_type = "application/vnd.ms-excel.namedsheetviews+xml"
        )
    )]
    NamedSheetViewsPart(crate::parts::named_sheet_views_part::NamedSheetViewsPart),
    #[sdk(
        relationship_type = RelationshipNotesMaster,
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesMaster,
            accessor = as_notes_master_part,
            content_type = "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml"
        )
    )]
    NotesMasterPart(crate::parts::notes_master_part::NotesMasterPart),
    #[sdk(
        relationship_type = RelationshipNotesSlide,
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesSlide,
            accessor = as_notes_slide_part,
            content_type = "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"
        )
    )]
    NotesSlidePart(crate::parts::notes_slide_part::NotesSlidePart),
    #[sdk(
        relationship_type = RelationshipNumbering,
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Numbering,
            accessor = as_numbering_definitions_part,
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"
        )
    )]
    NumberingDefinitionsPart(
        crate::parts::numbering_definitions_part::NumberingDefinitionsPart,
    ),
    #[sdk(
        relationship_type = RelationshipPivotCacheDefinition,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheDefinition,
            accessor = as_pivot_table_cache_definition_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml"
        )
    )]
    PivotTableCacheDefinitionPart(
        crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart,
    ),
    #[sdk(
        relationship_type = RelationshipPivotCacheRecords,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheRecords,
            accessor = as_pivot_table_cache_records_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml"
        )
    )]
    PivotTableCacheRecordsPart(
        crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart,
    ),
    #[sdk(
        relationship_type = RelationshipPivotTable,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableDefinition,
            accessor = as_pivot_table_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml"
        )
    )]
    PivotTablePart(crate::parts::pivot_table_part::PivotTablePart),
    #[sdk(
        relationship_type = RelationshipAuthors,
        root(
            element = crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::AuthorList,
            accessor = as_power_point_authors_part,
            content_type = "application/vnd.ms-powerpoint.authors+xml"
        )
    )]
    PowerPointAuthorsPart(crate::parts::power_point_authors_part::PowerPointAuthorsPart),
    #[sdk(
        relationship_type = RelationshipComments,
        root(
            element = crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::CommentList,
            accessor = as_power_point_comment_part,
            content_type = "application/vnd.ms-powerpoint.comments+xml"
        )
    )]
    PowerPointCommentPart(crate::parts::power_point_comment_part::PowerPointCommentPart),
    #[sdk(
        relationship_type = RelationshipOfficeDocument,
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation,
            accessor = as_presentation_part,
            content_type = "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"
        )
    )]
    PresentationPart(crate::parts::presentation_part::PresentationPart),
    #[sdk(
        relationship_type = RelationshipPresProps,
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::PresentationProperties,
            accessor = as_presentation_properties_part,
            content_type = "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml"
        )
    )]
    PresentationPropertiesPart(
        crate::parts::presentation_properties_part::PresentationPropertiesPart,
    ),
    #[sdk(
        relationship_type = RelationshipQueryTable,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::QueryTable,
            accessor = as_query_table_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml"
        )
    )]
    QueryTablePart(crate::parts::query_table_part::QueryTablePart),
    #[sdk(relationship_type = RelationshipUserCustomization)]
    QuickAccessToolbarCustomizationsPart(
        crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart,
    ),
    #[sdk(
        relationship_type = RelationshipRdArray,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::ArrayData,
            accessor = as_rd_array_part,
            content_type = "application/vnd.ms-excel.rdarray+xml"
        )
    )]
    RdArrayPart(crate::parts::rd_array_part::RdArrayPart),
    #[sdk(
        relationship_type = RelationshipRdRichValue,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueData,
            accessor = as_rd_rich_value_part,
            content_type = "application/vnd.ms-excel.rdrichvalue+xml"
        )
    )]
    RdRichValuePart(crate::parts::rd_rich_value_part::RdRichValuePart),
    #[sdk(
        relationship_type = RelationshipRdRichValueStructure,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueStructures,
            accessor = as_rd_rich_value_structure_part,
            content_type = "application/vnd.ms-excel.rdrichvaluestructure+xml"
        )
    )]
    RdRichValueStructurePart(
        crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart,
    ),
    #[sdk(
        relationship_type = RelationshipRdRichValueTypes,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::RichValueTypesInfo,
            accessor = as_rd_rich_value_types_part,
            content_type = "application/vnd.ms-excel.rdrichvaluetypes+xml"
        )
    )]
    RdRichValueTypesPart(crate::parts::rd_rich_value_types_part::RdRichValueTypesPart),
    #[sdk(
        relationship_type = RelationshipRdRichValueWebImage,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2020_richdatawebimage::WebImagesSupportingRichData,
            accessor = as_rd_rich_value_web_image_part,
            content_type = "application/vnd.ms-excel.rdrichvaluewebimage+xml"
        )
    )]
    RdRichValueWebImagePart(
        crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart,
    ),
    #[sdk(
        relationship_type = RelationshipRdSupportingPropertyBag,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBags,
            accessor = as_rd_supporting_property_bag_part,
            content_type = "application/vnd.ms-excel.rdsupportingpropertybag+xml"
        )
    )]
    RdSupportingPropertyBagPart(
        crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart,
    ),
    #[sdk(
        relationship_type = RelationshipRdSupportingPropertyBagStructure,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBagStructures,
            accessor = as_rd_supporting_property_bag_structure_part,
            content_type = "application/vnd.ms-excel.rdsupportingpropertybagstructure+xml"
        )
    )]
    RdSupportingPropertyBagStructurePart(
        crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart,
    ),
    #[sdk(
        relationship_type = RelationshipExtensibility,
        root(
            element = crate::schemas::schemas_microsoft_com_office_2009_07_customui::CustomUi,
            accessor = as_ribbon_and_backstage_customizations_part,
            content_type = "application/xml"
        )
    )]
    RibbonAndBackstageCustomizationsPart(
        crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart,
    ),
    #[sdk(relationship_type = RelationshipExtensibility2)]
    RibbonExtensibilityPart(
        crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart,
    ),
    #[sdk(
        relationship_type = RelationshipRichStyles,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::RichStylesheet,
            accessor = as_rich_styles_part,
            content_type = "application/vnd.ms-excel.richstyles+xml"
        )
    )]
    RichStylesPart(crate::parts::rich_styles_part::RichStylesPart),
    #[sdk(
        relationship_type = RelationshipSharedStrings,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SharedStringTable,
            accessor = as_shared_string_table_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"
        )
    )]
    SharedStringTablePart(crate::parts::shared_string_table_part::SharedStringTablePart),
    #[sdk(
        relationship_type = RelationshipTableSingleCells,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SingleXmlCells,
            accessor = as_single_cell_table_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml"
        )
    )]
    SingleCellTablePart(crate::parts::single_cell_table_part::SingleCellTablePart),
    #[sdk(
        relationship_type = RelationshipSlicerCache,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerCacheDefinition,
            accessor = as_slicer_cache_part,
            content_type = "application/vnd.ms-excel.slicerCache+xml"
        )
    )]
    SlicerCachePart(crate::parts::slicer_cache_part::SlicerCachePart),
    #[sdk(
        relationship_type = RelationshipSlicer,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Slicers,
            accessor = as_slicers_part,
            content_type = "application/vnd.ms-excel.slicer+xml"
        )
    )]
    SlicersPart(crate::parts::slicers_part::SlicersPart),
    #[sdk(
        relationship_type = RelationshipComments2,
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentList,
            accessor = as_slide_comments_part,
            content_type = "application/vnd.openxmlformats-officedocument.presentationml.comments+xml"
        )
    )]
    SlideCommentsPart(crate::parts::slide_comments_part::SlideCommentsPart),
    #[sdk(
        relationship_type = RelationshipSlideLayout,
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideLayout,
            accessor = as_slide_layout_part,
            content_type = "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"
        )
    )]
    SlideLayoutPart(crate::parts::slide_layout_part::SlideLayoutPart),
    #[sdk(
        relationship_type = RelationshipSlideMaster,
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideMaster,
            accessor = as_slide_master_part,
            content_type = "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"
        )
    )]
    SlideMasterPart(crate::parts::slide_master_part::SlideMasterPart),
    #[sdk(
        relationship_type = RelationshipSlide,
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Slide,
            accessor = as_slide_part,
            content_type = "application/vnd.openxmlformats-officedocument.presentationml.slide+xml"
        )
    )]
    SlidePart(crate::parts::slide_part::SlidePart),
    #[sdk(
        relationship_type = RelationshipSlideUpdateInfo,
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideSyncProperties,
            accessor = as_slide_sync_data_part,
            content_type = "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml"
        )
    )]
    SlideSyncDataPart(crate::parts::slide_sync_data_part::SlideSyncDataPart),
    #[sdk(relationship_type = RelationshipPrinterSettings)]
    SpreadsheetPrinterSettingsPart(
        crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
    ),
    #[sdk(
        relationship_type = RelationshipStyles,
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles,
            accessor = as_style_definitions_part,
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"
        )
    )]
    StyleDefinitionsPart(crate::parts::style_definitions_part::StyleDefinitionsPart),
    #[sdk(
        relationship_type = RelationshipStylesWithEffects,
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles,
            accessor = as_styles_with_effects_part,
            content_type = "application/vnd.ms-word.stylesWithEffects+xml"
        )
    )]
    StylesWithEffectsPart(crate::parts::styles_with_effects_part::StylesWithEffectsPart),
    #[sdk(
        relationship_type = RelationshipTable,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Table,
            accessor = as_table_definition_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml"
        )
    )]
    TableDefinitionPart(crate::parts::table_definition_part::TableDefinitionPart),
    #[sdk(
        relationship_type = RelationshipTableStyles,
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TableStyleList,
            accessor = as_table_styles_part,
            content_type = "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml"
        )
    )]
    TableStylesPart(crate::parts::table_styles_part::TableStylesPart),
    #[sdk(
        relationship_type = RelationshipThemeOverride,
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ThemeOverride,
            accessor = as_theme_override_part,
            content_type = "application/vnd.openxmlformats-officedocument.themeOverride+xml"
        )
    )]
    ThemeOverridePart(crate::parts::theme_override_part::ThemeOverridePart),
    #[sdk(
        relationship_type = RelationshipTheme,
        root(
            element = crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Theme,
            accessor = as_theme_part,
            content_type = "application/vnd.openxmlformats-officedocument.theme+xml"
        )
    )]
    ThemePart(crate::parts::theme_part::ThemePart),
    #[sdk(relationship_type = RelationshipThumbnail)]
    ThumbnailPart(crate::parts::thumbnail_part::ThumbnailPart),
    #[sdk(
        relationship_type = RelationshipTimelineCache,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineCacheDefinition,
            accessor = as_time_line_cache_part,
            content_type = "application/vnd.ms-excel.timelineCache+xml"
        )
    )]
    TimeLineCachePart(crate::parts::time_line_cache_part::TimeLineCachePart),
    #[sdk(
        relationship_type = RelationshipTimeline,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::Timelines,
            accessor = as_time_line_part,
            content_type = "application/vnd.ms-excel.timeline+xml"
        )
    )]
    TimeLinePart(crate::parts::time_line_part::TimeLinePart),
    #[sdk(
        relationship_type = RelationshipTags,
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TagList,
            accessor = as_user_defined_tags_part,
            content_type = "application/vnd.openxmlformats-officedocument.presentationml.tags+xml"
        )
    )]
    UserDefinedTagsPart(crate::parts::user_defined_tags_part::UserDefinedTagsPart),
    #[sdk(
        relationship_type = RelationshipWordVbaData,
        root(
            element = crate::schemas::schemas_microsoft_com_office_word_2006_wordml::VbaSuppData,
            accessor = as_vba_data_part,
            content_type = "application/vnd.ms-word.vbaData+xml"
        )
    )]
    VbaDataPart(crate::parts::vba_data_part::VbaDataPart),
    #[sdk(relationship_type = RelationshipVbaProject)]
    VbaProjectPart(crate::parts::vba_project_part::VbaProjectPart),
    #[sdk(
        relationship_type = RelationshipViewProps,
        root(
            element = crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::ViewProperties,
            accessor = as_view_properties_part,
            content_type = "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml"
        )
    )]
    ViewPropertiesPart(crate::parts::view_properties_part::ViewPropertiesPart),
    #[sdk(relationship_type = RelationshipVmlDrawing)]
    VmlDrawingPart(crate::parts::vml_drawing_part::VmlDrawingPart),
    #[sdk(
        relationship_type = RelationshipVolatileDependencies,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::VolatileTypes,
            accessor = as_volatile_dependencies_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml"
        )
    )]
    VolatileDependenciesPart(
        crate::parts::volatile_dependencies_part::VolatileDependenciesPart,
    ),
    #[sdk(
        relationship_type = RelationshipWebextensiontaskpanes,
        root(
            element = crate::schemas::schemas_microsoft_com_office_webextensions_taskpanes_2010_11::Taskpanes,
            accessor = as_web_ex_taskpanes_part,
            content_type = "application/vnd.ms-office.webextensiontaskpanes+xml"
        )
    )]
    WebExTaskpanesPart(crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart),
    #[sdk(
        relationship_type = RelationshipWebextension,
        root(
            element = crate::schemas::schemas_microsoft_com_office_webextensions_webextension_2010_11::WebExtension,
            accessor = as_web_extension_part,
            content_type = "application/vnd.ms-office.webextension+xml"
        )
    )]
    WebExtensionPart(crate::parts::web_extension_part::WebExtensionPart),
    #[sdk(
        relationship_type = RelationshipWebSettings,
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::WebSettings,
            accessor = as_web_settings_part,
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml"
        )
    )]
    WebSettingsPart(crate::parts::web_settings_part::WebSettingsPart),
    #[sdk(relationship_type = RelationshipAttachedToolbars)]
    WordAttachedToolbarsPart(
        crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart,
    ),
    #[sdk(
        relationship_type = RelationshipCommentsExtensible,
        root(
            element = crate::schemas::schemas_microsoft_com_office_word_2018_wordml_cex::CommentsExtensible,
            accessor = as_word_comments_extensible_part,
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtensible+xml"
        )
    )]
    WordCommentsExtensiblePart(
        crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart,
    ),
    #[sdk(
        relationship_type = RelationshipCommentsExtended,
        root(
            element = crate::schemas::schemas_microsoft_com_office_word_2012_wordml::CommentsEx,
            accessor = as_wordprocessing_comments_ex_part,
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtended+xml"
        )
    )]
    WordprocessingCommentsExPart(
        crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart,
    ),
    #[sdk(
        relationship_type = RelationshipCommentsIds,
        root(
            element = crate::schemas::schemas_microsoft_com_office_word_2016_wordml_cid::CommentsIds,
            accessor = as_wordprocessing_comments_ids_part,
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsIds+xml"
        )
    )]
    WordprocessingCommentsIdsPart(
        crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart,
    ),
    #[sdk(
        relationship_type = RelationshipComments2,
        root(
            element = crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Comments,
            accessor = as_wordprocessing_comments_part,
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"
        )
    )]
    WordprocessingCommentsPart(
        crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart,
    ),
    #[sdk(
        relationship_type = RelationshipPeople,
        root(
            element = crate::schemas::schemas_microsoft_com_office_word_2012_wordml::People,
            accessor = as_wordprocessing_people_part,
            content_type = "application/vnd.openxmlformats-officedocument.wordprocessingml.people+xml"
        )
    )]
    WordprocessingPeoplePart(
        crate::parts::wordprocessing_people_part::WordprocessingPeoplePart,
    ),
    #[sdk(relationship_type = RelationshipPrinterSettings)]
    WordprocessingPrinterSettingsPart(
        crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart,
    ),
    #[sdk(
        relationship_type = RelationshipOfficeDocument,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook,
            accessor = as_workbook_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"
        )
    )]
    WorkbookPart(crate::parts::workbook_part::WorkbookPart),
    #[sdk(
        relationship_type = RelationshipPerson,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::PersonList,
            accessor = as_workbook_person_part,
            content_type = "application/vnd.ms-excel.person+xml"
        )
    )]
    WorkbookPersonPart(crate::parts::workbook_person_part::WorkbookPersonPart),
    #[sdk(
        relationship_type = RelationshipRevisionHeaders,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Headers,
            accessor = as_workbook_revision_header_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml"
        )
    )]
    WorkbookRevisionHeaderPart(
        crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart,
    ),
    #[sdk(
        relationship_type = RelationshipRevisionLog,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Revisions,
            accessor = as_workbook_revision_log_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml"
        )
    )]
    WorkbookRevisionLogPart(
        crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart,
    ),
    #[sdk(
        relationship_type = RelationshipStyles,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Stylesheet,
            accessor = as_workbook_styles_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"
        )
    )]
    WorkbookStylesPart(crate::parts::workbook_styles_part::WorkbookStylesPart),
    #[sdk(
        relationship_type = RelationshipUsernames,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Users,
            accessor = as_workbook_user_data_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml"
        )
    )]
    WorkbookUserDataPart(crate::parts::workbook_user_data_part::WorkbookUserDataPart),
    #[sdk(
        relationship_type = RelationshipComments2,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Comments,
            accessor = as_worksheet_comments_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml"
        )
    )]
    WorksheetCommentsPart(crate::parts::worksheet_comments_part::WorksheetCommentsPart),
    #[sdk(
        relationship_type = RelationshipWorksheet,
        root(
            element = crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Worksheet,
            accessor = as_worksheet_part,
            content_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"
        )
    )]
    WorksheetPart(crate::parts::worksheet_part::WorksheetPart),
    #[sdk(
        relationship_type = RelationshipWsSortMap,
        root(
            element = crate::schemas::schemas_microsoft_com_office_excel_2006_main::WorksheetSortMap,
            accessor = as_worksheet_sort_map_part,
            content_type = "application/vnd.ms-excel.wsSortMap+xml"
        )
    )]
    WorksheetSortMapPart(crate::parts::worksheet_sort_map_part::WorksheetSortMapPart),
    #[sdk(
        relationship_type = RelationshipThreadedComment,
        root(
            element = crate::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::ThreadedComments,
            accessor = as_worksheet_threaded_comments_part,
            content_type = "application/vnd.ms-excel.threadedcomments+xml"
        )
    )]
    WorksheetThreadedCommentsPart(
        crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart,
    ),
    #[sdk(relationship_type = RelationshipSignature)]
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
    && relationship.relationship_known_type()
      == Some(crate::namespaces::XmlKnownRelationshipNamespace::RelationshipCalcChain)
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
  zip.write_all(&storage.content_types().to_bytes()?)?;
  let package_relationships = storage.package_relationships();
  if !package_relationships.is_empty() {
    if entry_set.insert("_rels".to_string()) {
      zip.add_directory("_rels", options)?;
    }
    zip.start_file("_rels/.rels", options)?;
    zip.write_all(&package_relationships.to_relationships().to_bytes()?)?;
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
      zip.write_all(&relationships.to_relationships().to_bytes()?)?;
    }
    zip.start_file(part.path(), options)?;
    if let Some(root_element) = crate::sdk::SdkPackage::root_element(package, part_id) {
      zip.write_all(&root_element.to_bytes()?)?;
    } else {
      zip.write_all(part.data().bytes())?;
    }
  }
  zip.finish()?;
  Ok(())
}
